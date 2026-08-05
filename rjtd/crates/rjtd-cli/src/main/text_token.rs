use rjtd_core::document_text::{
    DocumentTextElement, map_document_text, read_document_text_payload,
};

use crate::input::read_file;

use super::support::*;
use super::text_boundary_support::build_control_delimited_ranges;
use super::text_position_count_support::*;

pub(crate) fn run_cat(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "cat")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    write_stdout(payload.text())?;
    Ok(())
}

pub(crate) fn run_text_tokens(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "text-tokens")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    for element in payload.parsed_text().elements() {
        match element {
            DocumentTextElement::TextRun(text) => {
                write_stdout_line(&format!("text\t{}", escaped_text(text)))?;
            }
            DocumentTextElement::InlineText(segment) => {
                write_stdout_line(&format!(
                    "inline\t0x{:04x}\t{}",
                    segment.selector(),
                    escaped_text(segment.text())
                ))?;
            }
            DocumentTextElement::SkippedInlineText(segment) => {
                let selector = segment
                    .selector()
                    .map(|selector| format!("0x{selector:04x}"))
                    .unwrap_or_else(|| "-".to_string());
                write_stdout_line(&format!(
                    "skipped-inline\t{}\t{}\t{}",
                    selector,
                    segment.raw_bytes().len(),
                    escaped_text(segment.text())
                ))?;
            }
            DocumentTextElement::ControlBoundary(control) => {
                write_stdout_line(&format!("control\t0x{:04x}", control.code()))?;
            }
        }
    }
    Ok(())
}

pub(crate) fn run_text_control_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-control-context")?;
    let filter = args
        .next()
        .map(|value| parse_u16_argument(&value))
        .transpose()?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let control_indexes = map
        .entries()
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.kind().as_str() == "control")
        .filter(|(_, entry)| filter.is_none_or(|code| entry.code() == Some(code)))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    for index in control_indexes {
        let entry = &map.entries()[index];
        let Some(code) = entry.code() else {
            continue;
        };
        write_stdout_line(&format!(
            "control-context\t{}\t0x{:04x}\tbyte={}-{}\tunit={}-{}\tprev={}\tnext={}\tprev-control={}\tnext-control={}",
            index,
            code,
            entry.byte_start(),
            entry.byte_end(),
            entry.unit_start(),
            entry.unit_end(),
            format_map_entry_at(map.entries(), index.checked_sub(1)),
            format_map_entry_at(map.entries(), index.checked_add(1)),
            format_nearest_control_entry(map.entries(), index, false),
            format_nearest_control_entry(map.entries(), index, true)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_control_clusters(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-control-clusters")?;
    let filter = args
        .next()
        .map(|value| parse_u16_argument(&value))
        .transpose()?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let entries = map.entries();
    let mut index = 0usize;

    while index < entries.len() {
        if entries[index].kind().as_str() != "control" {
            index += 1;
            continue;
        }

        let start = index;
        let mut end = index + 1;
        while end < entries.len() && entries[end].kind().as_str() == "control" {
            end += 1;
        }

        let cluster = &entries[start..end];
        if filter.is_none_or(|code| cluster.iter().any(|entry| entry.code() == Some(code))) {
            let first = &entries[start];
            let last = &entries[end - 1];
            write_stdout_line(&format!(
                "control-cluster\t{}-{}\tlen={}\tcodes={}\tbyte={}-{}\tunit={}-{}\tprev={}\tnext={}",
                start,
                end - 1,
                cluster.len(),
                format_control_code_sequence(cluster),
                first.byte_start(),
                last.byte_end(),
                first.unit_start(),
                last.unit_end(),
                format_map_entry_at(entries, start.checked_sub(1)),
                format_map_entry_at(entries, Some(end))
            ))?;
        }

        index = end;
    }
    Ok(())
}

pub(crate) fn run_text_control_ranges(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-control-ranges")?;
    let filter = args
        .next()
        .map(|value| parse_u16_argument(&value))
        .transpose()?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let entries = map.entries();
    let ranges = build_control_delimited_ranges(entries, filter);

    for range in ranges {
        let range_entries = &entries[range.entry_start..range.entry_end];
        write_stdout_line(&format!(
            "control-range\t{}\tdelimiter={}\tprev={}\tnext={}\tentries={}\tbyte={}-{}\tunit={}-{}\t{}",
            range.index,
            format_control_range_delimiter(filter),
            format_control_range_boundary(entries, range.previous_delimiter, "start"),
            format_control_range_boundary(entries, range.next_delimiter, "end"),
            format_entry_index_span(range.entry_start, range.entry_end),
            range.byte_start,
            range.byte_end,
            range.unit_start,
            range.unit_end,
            format_control_range_contents(range_entries)
        ))?;
    }
    Ok(())
}
