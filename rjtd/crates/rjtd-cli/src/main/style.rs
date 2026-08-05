use std::collections::BTreeMap;

use rjtd_core::document_text::read_document_text_payload;
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, StyleStreamRecordSummary,
    TEXT_LAYOUT_STYLE_PATH, read_style_streams,
};

use crate::input::read_file;

use super::style_support::{
    active_page_layout_slot_pairs, document_view_style_group_id,
    format_document_view_group_payload_digest, format_optional_text, format_page_layout_slot_pairs,
    format_page_layout_slot_part, format_page_layout_slot_part_first,
    format_page_layout_slot_part_nonzero, format_style_record_payload_be16,
    format_style_record_payload_digest, format_style_record_payload_preview, format_u16_hex_values,
    format_u32_hex_values, format_usize_values, page_layout_slot_parts,
};
use super::support::{be16_words, escaped_path, escaped_text, required_path, write_stdout_line};

pub(crate) fn run_style_records(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "style-records")?;
    let bytes = read_file(path)?;
    let streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
    write_stdout_line(&format!("style_streams\t{}", streams.len()))?;
    for stream in streams {
        let summary = stream.summary();
        write_stdout_line(&format!(
            "stream\t{}\tbytes={}\tfamily={}\trecordLayout={}\trecordCount={}\theaderU32Be={}\theaderU16Be={}",
            escaped_path(stream.name()),
            stream.bytes().len(),
            summary.family().as_str(),
            summary.record_layout().as_str(),
            summary.records().len(),
            format_u32_hex_values(summary.header_u32_be()),
            format_u16_hex_values(summary.header_u16_be())
        ))?;
        for (record_index, record) in summary.records().iter().enumerate() {
            write_stdout_line(&format!(
                "record\t{}\t{}\toffset={}\tcode=0x{:04x}\tpayloadLength={}\tlabel={}",
                escaped_path(stream.name()),
                record_index,
                record.offset(),
                record.code(),
                record.payload_len(),
                format_optional_text(record.label())
            ))?;
        }
    }
    Ok(())
}

pub(crate) fn run_page_layout_style_slots(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "page-layout-style-slots")?;
    let bytes = read_file(path)?;
    let streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
    let Some(stream) = streams
        .iter()
        .find(|stream| stream.name() == PAGE_LAYOUT_STYLE_PATH)
    else {
        write_stdout_line(&format!(
            "summary\tstatus=missing\tstream={}\trecords=0\tslots=0\tdecoded=false",
            PAGE_LAYOUT_STYLE_PATH
        ))?;
        return Ok(());
    };

    let summary = stream.summary();
    let slot_sets = summary
        .records()
        .iter()
        .map(|record| page_layout_slot_parts(record.subrecords()))
        .collect::<Vec<_>>();
    let slot_count = slot_sets.iter().map(BTreeMap::len).sum::<usize>();
    let paired_slot_pairs = active_page_layout_slot_pairs(&slot_sets);
    write_stdout_line(&format!(
        "summary\tstatus=ok\tstream={}\tstream-bytes={}\trecords={}\tslots={}\tpaired-slot-pairs={}\tfacing-pages-candidate={}\tdecoded=false",
        PAGE_LAYOUT_STYLE_PATH,
        stream.bytes().len(),
        summary.records().len(),
        slot_count,
        format_page_layout_slot_pairs(&paired_slot_pairs),
        !paired_slot_pairs.is_empty()
    ))?;
    for (record_index, record) in summary.records().iter().enumerate() {
        write_stdout_line(&format!(
            "record\t{}\toffset={}\tcode=0x{:04x}\tpayloadLength={}\tlabel={}\tsubrecords={}",
            record_index,
            record.offset(),
            record.code(),
            record.payload_len(),
            format_optional_text(record.label()),
            record.subrecords().len()
        ))?;
        for (slot, parts) in &slot_sets[record_index] {
            write_stdout_line(&format!(
                "slot\t{}\t0x{:02x}\tpart05First={}\tpart05NonZero={}\tpart04={}\tpart05={}\tpart06={}\tpart07={}",
                record_index,
                slot,
                format_page_layout_slot_part_first(parts, 0x05),
                format_page_layout_slot_part_nonzero(parts, 0x05),
                format_page_layout_slot_part(parts, 0x04),
                format_page_layout_slot_part(parts, 0x05),
                format_page_layout_slot_part(parts, 0x06),
                format_page_layout_slot_part(parts, 0x07)
            ))?;
        }
    }
    Ok(())
}

pub(crate) fn run_style_candidates(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "style-candidates")?;
    let bytes = read_file(path)?;
    let streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
    let mut lines = Vec::new();
    for stream in streams {
        if stream.name() != TEXT_LAYOUT_STYLE_PATH {
            continue;
        }
        let summary = stream.summary();
        for (record_index, record) in summary.records().iter().enumerate() {
            let Some(label) = record
                .label()
                .map(str::trim)
                .filter(|label| !label.is_empty())
            else {
                continue;
            };
            let candidate_id = lines.len() + 1;
            lines.push(format!(
                "candidate\t{}\t{}\t{}\toffset={}\tcode=0x{:04x}\tpayloadLength={}\tname={}",
                candidate_id,
                escaped_path(stream.name()),
                record_index,
                record.offset(),
                record.code(),
                record.payload_len(),
                escaped_text(label)
            ));
        }
    }
    write_stdout_line(&format!("style_candidates\t{}", lines.len()))?;
    for line in lines {
        write_stdout_line(&line)?;
    }
    Ok(())
}

pub(crate) fn run_text_layout_style_records(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-layout-style-records")?;
    let bytes = read_file(path)?;
    let streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
    let Some(stream) = streams
        .iter()
        .find(|stream| stream.name() == TEXT_LAYOUT_STYLE_PATH)
    else {
        write_stdout_line(
            "summary\tstatus=missing\tstream=/TextLayoutStyle\tstream-bytes=0\trecords=0\tlabeled=0",
        )?;
        return Ok(());
    };
    let summary = stream.summary();
    let labeled_count = summary
        .records()
        .iter()
        .filter(|record| record.label().is_some_and(|label| !label.trim().is_empty()))
        .count();
    write_stdout_line(&format!(
        "summary\tstatus=ok\tstream={}\tstream-bytes={}\trecords={}\tlabeled={}",
        escaped_path(stream.name()),
        stream.bytes().len(),
        summary.records().len(),
        labeled_count
    ))?;

    let mut candidate_id = 0usize;
    for (record_index, record) in summary.records().iter().enumerate() {
        let label = record
            .label()
            .map(str::trim)
            .filter(|label| !label.is_empty());
        let candidate = if label.is_some() {
            candidate_id += 1;
            candidate_id.to_string()
        } else {
            "-".to_string()
        };
        write_stdout_line(&format!(
            "record\t{}\tcandidate={}\toffset={}\tcode=0x{:04x}\tpayloadLength={}\tpayloadDigest={}\tpayloadPrefix={}\tpayloadBe16={}\tlabel={}",
            record_index,
            candidate,
            record.offset(),
            record.code(),
            record.payload_len(),
            format_style_record_payload_digest(stream.bytes(), record),
            format_style_record_payload_preview(stream.bytes(), record),
            format_style_record_payload_be16(stream.bytes(), record),
            format_optional_text(label)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_document_view_style_groups(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "document-view-style-groups")?;
    let bytes = read_file(path)?;
    let streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
    let Some(stream) = streams
        .iter()
        .find(|stream| stream.name() == DOCUMENT_VIEW_STYLES_PATH)
    else {
        write_stdout_line(
            "summary\tstatus=missing\tstream-bytes=0\trecords=0\tgroups=0\tgroup-records=0",
        )?;
        return Ok(());
    };
    let summary = stream.summary();
    let mut groups: BTreeMap<u16, Vec<(usize, &StyleStreamRecordSummary)>> = BTreeMap::new();
    for (record_index, record) in summary.records().iter().enumerate() {
        if let Some(group_id) = document_view_style_group_id(record.code()) {
            groups
                .entry(group_id)
                .or_default()
                .push((record_index, record));
        }
    }
    let group_record_count = groups.values().map(Vec::len).sum::<usize>();

    write_stdout_line(&format!(
        "summary\tstatus=ok\tstream-bytes={}\trecords={}\tgroups={}\tgroup-records={}",
        stream.bytes().len(),
        summary.records().len(),
        groups.len(),
        group_record_count
    ))?;

    for (group_id, records) in groups {
        let codes = records
            .iter()
            .map(|(_, record)| record.code())
            .collect::<Vec<_>>();
        let payload_lengths = records
            .iter()
            .map(|(_, record)| record.payload_len())
            .collect::<Vec<_>>();
        write_stdout_line(&format!(
            "group\t{}\trecords={}\tcodes={}\tpayloadLengths={}\tpayloadDigest={}",
            group_id,
            records.len(),
            format_u16_hex_values(&codes),
            format_usize_values(&payload_lengths),
            format_document_view_group_payload_digest(stream.bytes(), &records)
        ))?;

        for (record_index, record) in records {
            write_stdout_line(&format!(
                "record\t{}\t{}\toffset={}\tcode=0x{:04x}\tpayloadLength={}\tpayloadDigest={}\tpayloadPrefix={}\tpayloadBe16={}",
                group_id,
                record_index,
                record.offset(),
                record.code(),
                record.payload_len(),
                format_style_record_payload_digest(stream.bytes(), record),
                format_style_record_payload_preview(stream.bytes(), record),
                format_style_record_payload_be16(stream.bytes(), record)
            ))?;
        }
    }
    Ok(())
}

pub(crate) fn run_paragraph_style_records(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "paragraph-style-records")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let words = be16_words(payload.bytes()).collect::<Vec<_>>();
    let mut count = 0usize;
    let mut i = 0usize;
    while i + 16 < words.len() {
        // Match: 0x001c 0x0010 len 0x0000 0x0026 0x0005 [w6] [w7] [w8] [w9] [w10]
        // followed by footer (len 0x0000 0x0010 0x001f) then text run
        // Only match len=17 (0x0011) records with w4=0x0026 w5=0x0005
        if words[i] != 0x001c || words[i + 1] != 0x0010 {
            i += 1;
            continue;
        }
        let len = words[i + 2] as usize;
        if len != 17 {
            i += 1;
            continue;
        }
        if words[i + 4] != 0x0026 || words[i + 5] != 0x0005 {
            i += 1;
            continue;
        }
        // Verify footer: w[len-4]=len, w[len-3]=0, w[len-2]=class, w[len-1]=0x001f
        let footer_start = i + len - 4;
        if footer_start + 3 >= words.len() {
            i += 1;
            continue;
        }
        if words[footer_start] as usize != len
            || words[footer_start + 1] != 0x0000
            || words[footer_start + 2] != 0x0010
            || words[footer_start + 3] != 0x001f
        {
            i += 1;
            continue;
        }
        let w6 = words[i + 6];
        let w7 = words[i + 7];
        let w8 = words[i + 8];
        let w10 = words[i + 10];
        // Collect following text run (words after 0x001f until next control)
        let text_start = i + len;
        let mut text = String::new();
        let mut j = text_start;
        while j < words.len().min(text_start + 64) {
            let ch = words[j];
            if ch < 0x0020 || (0xd800..=0xdfff).contains(&ch) || ch == 0xffff {
                break;
            }
            if let Some(c) = char::from_u32(ch as u32) {
                text.push(c);
            }
            j += 1;
        }
        write_stdout_line(&format!(
            "record\t{}\tword={}\tw6=0x{:04x}\tw7={}\tw8=0x{:04x}\tw10=0x{:04x}\ttext={}",
            count,
            i,
            w6,
            w7,
            w8,
            w10,
            text.chars().take(24).collect::<String>()
        ))?;
        count += 1;
        i += len;
        continue;
    }
    write_stdout_line(&format!("summary\trecords={}\tdecoded=false", count))?;
    Ok(())
}
