use rjtd_core::container::read_cfb_stream;
use rjtd_core::document_text::{map_document_text, read_document_text_payload};
use rjtd_core::document_text_position::{
    DOCUMENT_TEXT_POSITION_TABLES_PATH, read_document_text_position_tables,
};

use crate::input::read_file;

use super::line_mark_support::*;
use super::page_mark_support::{
    page_mark_entries_summary, page_mark_summary, paper_mark_entries_summary, paper_mark_summary,
};
use super::support::*;
use super::text_boundary_support::format_mark_ids;
use super::text_position_count_support::*;

const MARK_VISIBLE_TEXT_PROBE_DELTA_UNITS: usize = 29;
const MARK_TABLE_MARKER: &[u8] = b"MarkV.01";
const MARK_TABLE_HEADER_BYTES: usize = 6;

pub(crate) fn run_text_positions(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "text-positions")?;
    let bytes = read_file(path)?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.entries().is_empty() {
        return Err("DocumentTextPositionTables missing MarkV.01 table".into());
    }
    for entry in table.entries() {
        write_stdout_line(&format!("{}\t{}", entry.id(), entry.offset()))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_mark_header(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-mark-header")?;
    let bytes = read_file(path)?;
    let stream = read_cfb_stream(&bytes, DOCUMENT_TEXT_POSITION_TABLES_PATH)
        .map_err(|error| error.to_string())?;
    let mark_offsets = find_subslice_offsets(&stream, MARK_TABLE_MARKER);
    if mark_offsets.is_empty() {
        return Err("DocumentTextPositionTables missing MarkV.01 marker".into());
    }

    for mark_offset in mark_offsets {
        let header_start = mark_offset + MARK_TABLE_MARKER.len();
        let header_end = header_start + MARK_TABLE_HEADER_BYTES;
        let header = stream.get(header_start..header_end).unwrap_or(&[]);
        write_stdout_line(&format!(
            "header\t{}\t{}\tbe16={}\tle16={}\tbe32@0={}\tbe32@2={}",
            mark_offset,
            bytes_to_hex(header),
            format_be16_fields(header),
            format_le16_fields(header),
            format_be32_candidate(header, 0),
            format_be32_candidate(header, 2)
        ))?;

        if header.len() != MARK_TABLE_HEADER_BYTES {
            continue;
        }

        let mut entry_offset = header_end;
        let mut entry_index = 0usize;
        while entry_offset + 2 <= stream.len() {
            let id = u16::from_be_bytes([stream[entry_offset], stream[entry_offset + 1]]);
            if id == 0xffff {
                break;
            }
            if entry_offset + 6 > stream.len() {
                break;
            }
            let raw = &stream[entry_offset..entry_offset + 6];
            write_stdout_line(&format!(
                "entry\t{}\t{}\t{}\t{}\t{}\t{}",
                mark_offset,
                entry_index,
                entry_offset,
                id,
                read_be32_candidate(raw, 2),
                bytes_to_hex(raw)
            ))?;
            entry_index += 1;
            entry_offset += 6;
        }
    }
    Ok(())
}

pub(crate) fn run_text_position_mark_summary(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-mark-summary")?;
    let bytes = read_file(path)?;
    let position_stream = read_cfb_stream(&bytes, DOCUMENT_TEXT_POSITION_TABLES_PATH)
        .map_err(|error| error.to_string())?;
    let mark_offset = find_subslice_offsets(&position_stream, MARK_TABLE_MARKER)
        .into_iter()
        .next()
        .ok_or_else(|| "DocumentTextPositionTables missing MarkV.01 marker".to_string())?;
    let header_start = mark_offset + MARK_TABLE_MARKER.len();
    let header = position_stream
        .get(header_start..header_start + MARK_TABLE_HEADER_BYTES)
        .unwrap_or(&[]);
    let mark_header_value = header
        .get(4..6)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string());

    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    let max_mark_offset = table
        .entries()
        .iter()
        .map(|entry| entry.offset())
        .max()
        .map(|offset| offset.to_string())
        .unwrap_or_else(|| "-".to_string());
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document_text_units = map
        .entries()
        .last()
        .map(|entry| entry.unit_end())
        .unwrap_or_default();

    write_stdout_line(&format!(
        "summary\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        mark_offset,
        bytes_to_hex(header),
        mark_header_value,
        table.entries().len(),
        max_mark_offset,
        payload.bytes().len(),
        document_text_units,
        line_mark_summary(&bytes),
        page_mark_summary(&bytes),
        paper_mark_summary(&bytes),
        stream_len_summary(&bytes, "/PageMark"),
        stream_len_summary(&bytes, "/PaperMark")
    ))?;
    Ok(())
}

pub(crate) fn run_text_map(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "text-map")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let position_entries = read_document_text_position_tables(&bytes)
        .map(|table| table.entries().to_vec())
        .unwrap_or_default();
    let map = map_document_text(payload.bytes());

    for entry in map.entries() {
        let byte_marks = format_mark_ids(
            position_entries
                .iter()
                .filter(|position| entry.contains_byte_offset(position.offset() as usize))
                .map(|position| position.id()),
        );
        let unit_marks = format_mark_ids(
            position_entries
                .iter()
                .filter(|position| entry.contains_unit_offset(position.offset() as usize))
                .map(|position| position.id()),
        );
        write_stdout_line(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            entry.byte_start(),
            entry.byte_end(),
            entry.unit_start(),
            entry.unit_end(),
            entry.kind().as_str(),
            document_text_map_meta(entry),
            byte_marks,
            unit_marks,
            escaped_text_preview(entry.text(), 80)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let position_table =
        read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());

    for position in position_table.entries() {
        let offset = position.offset() as usize;
        write_stdout_line(&format!(
            "{}\t{}\t{}\t{}\t{}",
            position.id(),
            position.offset(),
            format_byte_context(map.entries(), offset),
            format_unit_context(map.entries(), offset),
            format_unit_context(
                map.entries(),
                offset.saturating_add(MARK_VISIBLE_TEXT_PROBE_DELTA_UNITS),
            )
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_line_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-line-context")?;
    let bytes = read_file(path)?;
    let line_stream = read_cfb_stream(&bytes, "/LineMark").map_err(|error| error.to_string())?;
    let line_words = be16_words(&line_stream).collect::<Vec<_>>();
    let position_stream = read_cfb_stream(&bytes, DOCUMENT_TEXT_POSITION_TABLES_PATH)
        .map_err(|error| error.to_string())?;
    let mark_offset = find_subslice_offsets(&position_stream, MARK_TABLE_MARKER)
        .into_iter()
        .next()
        .ok_or_else(|| "DocumentTextPositionTables missing MarkV.01 marker".to_string())?;
    let header_start = mark_offset + MARK_TABLE_MARKER.len();
    let header = position_stream
        .get(header_start..header_start + MARK_TABLE_HEADER_BYTES)
        .unwrap_or(&[]);
    let header_line_index = header
        .get(4..6)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]) as usize);
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;

    write_stdout_line(&format!(
        "summary\tline-words={}\tline-tags={}\tmark-entries={}\tpage-entries={}\tpaper-entries={}",
        line_words.len(),
        line_words
            .iter()
            .filter(|word| is_line_mark_tag(**word))
            .count(),
        table.entries().len(),
        page_mark_entries_summary(&bytes),
        paper_mark_entries_summary(&bytes)
    ))?;
    if let Some(line_index) = header_line_index {
        write_stdout_line(&format!(
            "header\t{}\t{}\tline-index={}\tword={}\tprev-tag={}\tnext-tag={}\tcontext={}",
            mark_offset,
            bytes_to_hex(header),
            line_index,
            format_line_word_at(&line_words, line_index),
            format_nearest_line_tag(&line_words, line_index, true),
            format_nearest_line_tag(&line_words, line_index, false),
            format_line_word_context_around(&line_words, line_index)
        ))?;
    } else {
        write_stdout_line(&format!(
            "header\t{}\t{}\tline-index=-\tword=-\tprev-tag=-\tnext-tag=-\tcontext=-",
            mark_offset,
            bytes_to_hex(header)
        ))?;
    }

    for position in table.entries() {
        let line_index = position.offset() as usize;
        write_stdout_line(&format!(
            "entry\t{}\t{}\tline-index={}\tword={}\tprev-tag={}\tnext-tag={}\tcontext={}",
            position.id(),
            position.offset(),
            line_index,
            format_line_word_at(&line_words, line_index),
            format_nearest_line_tag(&line_words, line_index, true),
            format_nearest_line_tag(&line_words, line_index, false),
            format_line_word_context_around(&line_words, line_index)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_delta_scan(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-delta-scan")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let position_table =
        read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if position_table.entries().is_empty() {
        return Err("DocumentTextPositionTables missing MarkV.01 table".into());
    }
    let map = map_document_text(payload.bytes());

    for delta in 0..=64usize {
        let mut unit_hits = 0usize;
        let mut text_hits = 0usize;
        for position in position_table.entries() {
            let offset = (position.offset() as usize).saturating_add(delta);
            if unit_hit(map.entries(), offset).is_some() {
                unit_hits += 1;
            }
            if unit_text_hit(map.entries(), offset).is_some() {
                text_hits += 1;
            }
        }
        write_stdout_line(&format!(
            "delta\t{}\t{}\t{}\t{}",
            delta,
            position_table.entries().len(),
            unit_hits,
            text_hits
        ))?;
    }
    Ok(())
}
