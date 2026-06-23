use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};
use std::path::Path;

use rjtd_core::Error;
use rjtd_core::container::{
    CfbSectorChain, EntryKind, StreamStorage, inspect_cfb_directory, inspect_cfb_entries,
    inspect_cfb_overview, inspect_cfb_stream_chain, inspect_cfb_stream_location, read_cfb_stream,
};
use rjtd_core::document_text::{
    COMPRESSED_DOCUMENT_PATH, DOCUMENT_TEXT_PATH, DocumentTextElement, EMBEDDED_DOCUMENT_TEXT_PATH,
    map_document_text, read_document_text_payload,
};
use rjtd_core::document_text_position::{
    DOCUMENT_TEXT_POSITION_TABLES_PATH, read_document_text_position_tables,
};
use rjtd_core::format::detect_format;
use rjtd_core::layout_mark::{PageMark, PaperMark, read_page_mark, read_paper_mark};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, StyleStreamRecordSummary,
    StyleStreamSubrecordSummary, TEXT_LAYOUT_STYLE_PATH, read_style_streams,
};
#[cfg(not(target_arch = "wasm32"))]
use rjtd_export::to_pdf_with_file_name;
use rjtd_export::{to_json, to_markdown, to_plain_text};
use rjtd_model::{
    DocumentCore, ObjectFdmIndexBbox, ObjectFdmIndexEntryCandidate, ObjectFrameRecordCandidate,
    ObjectFrameReferenceRowCandidate, ObjectImagePayloadSpan, ObjectImageSignatureHit,
    ObjectStreamCandidate as ModelObjectStreamCandidate, TableCandidate, TextBoundaryCandidate,
    TextCountRange, TextLayoutExactEvidence, page_mark_u16_geometry_profile, parse_document,
};

const BROKEN_PIPE_EXIT: &str = "__rjtd_broken_pipe__";
const MARK_VISIBLE_TEXT_PROBE_DELTA_UNITS: usize = 29;
const MARK_TABLE_MARKER: &[u8] = b"MarkV.01";
const MARK_TABLE_HEADER_BYTES: usize = 6;
const SO_RECORD_MARKER: &[u8] = b"SO\0\0";
const SO_RECORD_BYTES: usize = 36;
const SO_RECORD_DWORDS: usize = SO_RECORD_BYTES / 4;
const OBJECT_STREAM_PREFIX_PREVIEW_BYTES: usize = 16;
const OBJECT_STREAM_MAX_REPORTED_HITS: usize = 6;
const VISUAL_LIST_MAGIC_OFFSET: usize = 4;
const VISUAL_LIST_MAGIC: &[u8; 4] = b"BMDV";
const VISUAL_LIST_WIDTH_OFFSET: usize = 0x1c;
const VISUAL_LIST_HEIGHT_OFFSET: usize = 0x20;
const VISUAL_LIST_BIT_DEPTH_OFFSET: usize = 0x2c;
const VISUAL_LIST_RLE_LENGTH_OFFSET: usize = 0x4c;
const EMBEDDED_PRESS_SNAPSHOT_MAGIC: &[u8; 12] = b"JSSnapShot32";
const EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET: usize = 0x24;
const EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET: usize = 0x34;
const EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET: usize = 0x48;
const EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET: usize = 0x4c;
const JSEQ3_CONTENTS_MAGIC_UTF16LE: &[u8; 16] = b"M\0A\0T\0H\0.\0V\0A\0F\0";
const JSEQ3_SO_TRAILER_BYTES: usize = 64;
const JSEQ3_SO_FIELD_BYTES: usize = 4;
const JSEQ3_SO_FIELD_COUNT: usize = 9;
const JSEQ3_TEXT_MARKERS: &[&str] = &["Times New Roman", "JustUnitMark", "JustOubunMark"];
const OBJECT_REFERENCE_CONTEXT_BEFORE_BYTES: usize = 8;
const OBJECT_REFERENCE_CONTEXT_AFTER_BYTES: usize = 8;
const OBJECT_REFERENCE_FIELD_STRIDES: &[usize] = &[
    4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 68, 72, 80, 84,
];
const OBJECT_FRAME_REFERENCE_RECORD_CANDIDATES: &[ObjectFrameReferenceRecordCandidate] = &[
    ObjectFrameReferenceRecordCandidate {
        encoding: "u16-le",
        stride: 12,
        field_offset: 5,
    },
    ObjectFrameReferenceRecordCandidate {
        encoding: "u16-be",
        stride: 12,
        field_offset: 7,
    },
    ObjectFrameReferenceRecordCandidate {
        encoding: "u16-be",
        stride: 20,
        field_offset: 15,
    },
];
const STYLE_RECORD_PAYLOAD_PREVIEW_BYTES: usize = 16;
const LAYOUT_MAP_DELTA_MIN: isize = -4096;
const LAYOUT_MAP_DELTA_MAX: isize = 4096;
const FNV1A64_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV1A64_PRIME: u64 = 0x0000_0100_0000_01b3;

fn main() {
    let code = match run(std::env::args().skip(1)) {
        Ok(()) => 0,
        Err(message) if message == BROKEN_PIPE_EXIT => 0,
        Err(message) => {
            eprintln!("error: {message}");
            2
        }
    };

    std::process::exit(code);
}

fn run(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let mut args = args.into_iter();

    match args.next().as_deref() {
        None | Some("-h") | Some("--help") => print_help(),
        Some("streams") => {
            let path = required_path(args.next(), "streams")?;
            let bytes = read_file(&path)?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            for entry in entries {
                write_stdout_line(&format!(
                    "{}\t{}\t{}",
                    entry.kind().as_str(),
                    entry.size(),
                    escaped_path(entry.path())
                ))?;
            }
            Ok(())
        }
        Some("info") => {
            let path = required_path(args.next(), "info")?;
            let bytes = read_file(&path)?;
            let format = detect_format(&bytes);
            write_stdout_line(&format!("format\t{}", format.as_str()))?;

            if format.as_str() == "unknown" {
                return Ok(());
            }

            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            let stream_count = entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
                .count();
            let storage_count = entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Storage)
                .count();
            write_stdout_line(&format!("streams\t{stream_count}"))?;
            write_stdout_line(&format!("storages\t{storage_count}"))?;
            print_entry_size(&entries, DOCUMENT_TEXT_PATH, "document_text_bytes")?;
            print_entry_size(
                &entries,
                DOCUMENT_TEXT_POSITION_TABLES_PATH,
                "document_text_position_table_bytes",
            )?;
            print_entry_size(
                &entries,
                COMPRESSED_DOCUMENT_PATH,
                "compressed_document_bytes",
            )?;
            if !entries
                .iter()
                .any(|entry| entry.path() == DOCUMENT_TEXT_PATH)
                && read_document_text_payload(&bytes)
                    .is_ok_and(|payload| payload.source_name() == EMBEDDED_DOCUMENT_TEXT_PATH)
            {
                write_stdout_line("embedded_document_text\tpresent")?;
            }
            Ok(())
        }
        Some("dump-stream") => {
            let path = required_path(args.next(), "dump-stream")?;
            let stream_path = required_path(args.next(), "dump-stream")?;
            let stream_path = unescaped_path(&stream_path)?;
            let bytes = read_file(path)?;
            let stream =
                read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
            write_stdout_bytes(&stream)?;
            Ok(())
        }
        Some("style-records") => {
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
        Some("page-layout-style-slots") => {
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
        Some("style-candidates") => {
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
        Some("text-layout-style-records") => {
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
        Some("document-view-style-groups") => {
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
            let mut groups: BTreeMap<u16, Vec<(usize, &StyleStreamRecordSummary)>> =
                BTreeMap::new();
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
        Some("cfb-map") => {
            let path = required_path(args.next(), "cfb-map")?;
            let bytes = read_file(path)?;
            let overview = inspect_cfb_overview(&bytes).map_err(|error| error.to_string())?;
            write_stdout_line(&format!("sector_size\t{}", overview.sector_size()))?;
            write_stdout_line(&format!(
                "mini_stream_cutoff\t{}",
                overview.mini_stream_cutoff()
            ))?;
            write_stdout_line(&format!(
                "fat_sectors\t{}\t{}",
                overview.fat_sector_ids().len(),
                format_sector_ids(overview.fat_sector_ids())
            ))?;
            write_cfb_chain("directory_chain", overview.directory_chain())?;
            write_cfb_chain("mini_fat_chain", overview.mini_fat_chain())?;
            write_stdout_line(&format!(
                "root_mini_stream\t{}\t{}",
                overview.root_start_sector(),
                overview.root_size()
            ))?;
            write_cfb_chain("mini_stream_chain", overview.mini_stream_chain())?;
            Ok(())
        }
        Some("cfb-dir") => {
            let path = required_path(args.next(), "cfb-dir")?;
            let bytes = read_file(path)?;
            let entries = inspect_cfb_directory(&bytes).map_err(|error| error.to_string())?;
            for entry in entries {
                write_stdout_line(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    entry.id(),
                    entry.kind().as_str(),
                    entry.size(),
                    entry.start_sector(),
                    format_cfb_id(entry.left_id()),
                    format_cfb_id(entry.right_id()),
                    format_cfb_id(entry.child_id()),
                    escaped_path(entry.path().unwrap_or("-")),
                    escaped_text(entry.name()),
                    entry.name().encode_utf16().count()
                ))?;
            }
            Ok(())
        }
        Some("stream-meta") => {
            let path = required_path(args.next(), "stream-meta")?;
            let stream_path = required_path(args.next(), "stream-meta")?;
            let stream_path = unescaped_path(&stream_path)?;
            let bytes = read_file(path)?;
            let location = inspect_cfb_stream_location(&bytes, &stream_path)
                .map_err(|error| error.to_string())?;
            write_stdout_line(&format!("path\t{}", escaped_path(location.path())))?;
            write_stdout_line(&format!("size\t{}", location.size()))?;
            write_stdout_line(&format!("start_sector\t{}", location.start_sector()))?;
            write_stdout_line(&format!("storage\t{}", location.storage().as_str()))?;
            write_stdout_line(&format!(
                "mini_stream_cutoff\t{}",
                location.mini_stream_cutoff()
            ))?;
            write_stdout_line(&format!(
                "mini_stream_bytes\t{}",
                location.mini_stream_bytes()
            ))?;
            write_stdout_line(&format!(
                "mini_fat_entries\t{}",
                location.mini_fat_entries()
            ))?;
            Ok(())
        }
        Some("stream-chain") => {
            let path = required_path(args.next(), "stream-chain")?;
            let stream_path = required_path(args.next(), "stream-chain")?;
            let stream_path = unescaped_path(&stream_path)?;
            let bytes = read_file(path)?;
            let chain = inspect_cfb_stream_chain(&bytes, &stream_path)
                .map_err(|error| error.to_string())?;
            let location = chain.location();
            write_stdout_line(&format!("path\t{}", escaped_path(location.path())))?;
            write_stdout_line(&format!("storage\t{}", location.storage().as_str()))?;
            write_stdout_line(&format!("declared_size\t{}", location.size()))?;
            write_stdout_line(&format!("start_sector\t{}", location.start_sector()))?;
            write_stdout_line(&format!("sector_size\t{}", chain.sector_size()))?;
            write_stdout_line(&format!(
                "offset_basis\t{}",
                stream_chain_offset_basis(location.storage())
            ))?;
            write_stdout_line(&format!("chain_bytes\t{}", chain.capacity_bytes()))?;
            write_stdout_line(&format!("status\t{}", chain.status().as_str()))?;
            for (index, sector) in chain.sectors().iter().enumerate() {
                write_stdout_line(&format!(
                    "sector\t{}\t{}\t{}\t{}",
                    index,
                    sector.sector_id(),
                    sector.byte_offset(),
                    sector.byte_len()
                ))?;
            }
            Ok(())
        }
        Some("stream-words") => {
            let path = required_path(args.next(), "stream-words")?;
            let stream_path = required_path(args.next(), "stream-words")?;
            let stream_path = unescaped_path(&stream_path)?;
            let bytes = read_file(path)?;
            let stream =
                read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
            for (index, word) in be16_words(&stream).enumerate() {
                write_stdout_line(&format!("{}\t{}\t0x{:04x}", index, index * 2, word))?;
            }
            Ok(())
        }
        Some("stream-word-frequencies") => {
            let path = required_path(args.next(), "stream-word-frequencies")?;
            let stream_path = required_path(args.next(), "stream-word-frequencies")?;
            let stream_path = unescaped_path(&stream_path)?;
            let bytes = read_file(path)?;
            let stream =
                read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
            let mut counts = BTreeMap::new();
            for word in be16_words(&stream) {
                *counts.entry(word).or_insert(0usize) += 1;
            }
            let mut counts = counts.into_iter().collect::<Vec<_>>();
            counts.sort_by(|(left_word, left_count), (right_word, right_count)| {
                right_count
                    .cmp(left_count)
                    .then_with(|| left_word.cmp(right_word))
            });
            for (word, count) in counts {
                write_stdout_line(&format!("{}\t0x{:04x}", count, word))?;
            }
            Ok(())
        }
        Some("line-mark-tags") => {
            let path = required_path(args.next(), "line-mark-tags")?;
            let bytes = read_file(path)?;
            let stream = read_cfb_stream(&bytes, "/LineMark").map_err(|error| error.to_string())?;
            let words = be16_words(&stream).collect::<Vec<_>>();
            for (index, word) in words.iter().enumerate() {
                if is_line_mark_tag(*word) {
                    write_stdout_line(&format!(
                        "tag\t{}\t{}\t0x{:04x}\tprev={}\tnext={}",
                        index,
                        index * 2,
                        word,
                        format_word_context(&words, index.saturating_sub(4), index),
                        format_word_context(&words, index + 1, (index + 7).min(words.len()))
                    ))?;
                }
            }
            Ok(())
        }
        Some("line-mark-intervals") => {
            let path = required_path(args.next(), "line-mark-intervals")?;
            let bytes = read_file(path)?;
            let stream = read_cfb_stream(&bytes, "/LineMark").map_err(|error| error.to_string())?;
            write_line_mark_intervals(&stream)
        }
        Some("line-mark-text-context") => {
            let path = required_path(args.next(), "line-mark-text-context")?;
            let bytes = read_file(path)?;
            let stream = read_cfb_stream(&bytes, "/LineMark").map_err(|error| error.to_string())?;
            let line_words = be16_words(&stream).collect::<Vec<_>>();
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let text_words = be16_words(payload.bytes()).collect::<Vec<_>>();
            let map = map_document_text(payload.bytes());

            for (index, word) in line_words.iter().enumerate() {
                if !is_line_mark_tag(*word) {
                    continue;
                }
                let next_word = line_words.get(index + 1).copied();
                let first_text_unit =
                    next_word.and_then(|word| text_words.iter().position(|text| *text == word));
                let text_word_hits = next_word
                    .map(|word| text_words.iter().filter(|text| **text == word).count())
                    .unwrap_or_default();
                write_stdout_line(&format!(
                    "tag\t{}\t{}\t0x{:04x}\tline-byte={}\tline-unit={}\tnext0={}\tdoc-word-hits={}\tfirst-doc-unit={}\tfirst-doc-context={}\tprev={}\tnext={}",
                    index,
                    index * 2,
                    word,
                    format_byte_context(map.entries(), index * 2),
                    format_unit_context(map.entries(), index),
                    next_word
                        .map(|word| format!("0x{word:04x}"))
                        .unwrap_or_else(|| "-".to_string()),
                    text_word_hits,
                    first_text_unit
                        .map(|unit| unit.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                    first_text_unit
                        .map(|unit| format_unit_context(map.entries(), unit))
                        .unwrap_or_else(|| "-".to_string()),
                    format_word_context(&line_words, index.saturating_sub(4), index),
                    format_word_context(&line_words, index + 1, (index + 7).min(line_words.len()))
                ))?;
            }
            Ok(())
        }
        Some("stream-dwords") => {
            let path = required_path(args.next(), "stream-dwords")?;
            let stream_path = required_path(args.next(), "stream-dwords")?;
            let stream_path = unescaped_path(&stream_path)?;
            let bytes = read_file(path)?;
            let stream =
                read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
            for (index, dword) in be32_dwords(&stream).enumerate() {
                write_stdout_line(&format!("{}\t{}\t0x{:08x}", index, index * 4, dword))?;
            }
            Ok(())
        }
        Some("stream-dword-frequencies") => {
            let path = required_path(args.next(), "stream-dword-frequencies")?;
            let stream_path = required_path(args.next(), "stream-dword-frequencies")?;
            let stream_path = unescaped_path(&stream_path)?;
            let bytes = read_file(path)?;
            let stream =
                read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
            let mut counts = BTreeMap::new();
            for dword in be32_dwords(&stream) {
                *counts.entry(dword).or_insert(0usize) += 1;
            }
            let mut counts = counts.into_iter().collect::<Vec<_>>();
            counts.sort_by(|(left_dword, left_count), (right_dword, right_count)| {
                right_count
                    .cmp(left_count)
                    .then_with(|| left_dword.cmp(right_dword))
            });
            for (dword, count) in counts {
                write_stdout_line(&format!("{}\t0x{:08x}", count, dword))?;
            }
            Ok(())
        }
        Some("stream-text-probe") => {
            let path = required_path(args.next(), "stream-text-probe")?;
            let stream_path = required_path(args.next(), "stream-text-probe")?;
            let stream_path = unescaped_path(&stream_path)?;
            let bytes = read_file(path)?;
            let stream =
                read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
            for (offset, text) in ascii_text_runs(&stream, 4) {
                write_stdout_line(&format!("ascii\t{}\t{}", offset, escaped_text(&text)))?;
            }
            for (offset, text) in utf16_text_runs(&stream, Utf16Endian::Little, 4) {
                write_stdout_line(&format!("utf16le\t{}\t{}", offset, escaped_text(&text)))?;
            }
            for (offset, text) in utf16_text_runs(&stream, Utf16Endian::Big, 4) {
                write_stdout_line(&format!("utf16be\t{}\t{}", offset, escaped_text(&text)))?;
            }
            Ok(())
        }
        Some("stream-find") => {
            let path = required_path(args.next(), "stream-find")?;
            let needle_path = required_path(args.next(), "stream-find")?;
            let needle_path = unescaped_path(&needle_path)?;
            let bytes = read_file(path)?;
            let needle =
                read_cfb_stream(&bytes, &needle_path).map_err(|error| error.to_string())?;
            if needle.is_empty() {
                return Err(format!("stream `{needle_path}` is empty"));
            }
            write_stdout_line(&format!(
                "needle\t{}\t{}",
                escaped_path(&needle_path),
                needle.len()
            ))?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            for entry in entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
            {
                match read_cfb_stream(&bytes, entry.path()) {
                    Ok(haystack) => {
                        for offset in find_subslice_offsets(&haystack, &needle) {
                            write_stdout_line(&format!(
                                "match\t{}\t{}\t{}",
                                escaped_path(entry.path()),
                                offset,
                                needle.len()
                            ))?;
                        }
                    }
                    Err(error) => {
                        write_stdout_line(&format!(
                            "unreadable\t{}\t{}",
                            escaped_path(entry.path()),
                            error
                        ))?;
                    }
                }
            }
            Ok(())
        }
        Some("stream-find-bytes") => {
            let path = required_path(args.next(), "stream-find-bytes")?;
            let needle_hex = args
                .next()
                .ok_or_else(|| "missing hex bytes for `stream-find-bytes`".to_string())?;
            let needle = parse_hex_bytes(&needle_hex)?;
            if needle.is_empty() {
                return Err("hex needle is empty".into());
            }
            let bytes = read_file(path)?;
            write_stdout_line(&format!(
                "needle\t{}\t{}",
                bytes_to_hex(&needle),
                needle.len()
            ))?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            for entry in entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
            {
                match read_cfb_stream(&bytes, entry.path()) {
                    Ok(haystack) => {
                        for offset in find_subslice_offsets(&haystack, &needle) {
                            write_stdout_line(&format!(
                                "match\t{}\t{}\t{}",
                                escaped_path(entry.path()),
                                offset,
                                needle.len()
                            ))?;
                        }
                    }
                    Err(error) => {
                        write_stdout_line(&format!(
                            "unreadable\t{}\t{}",
                            escaped_path(entry.path()),
                            error
                        ))?;
                    }
                }
            }
            Ok(())
        }
        Some("so-records") => {
            let path = required_path(args.next(), "so-records")?;
            let bytes = read_file(path)?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            for entry in entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
            {
                match read_cfb_stream(&bytes, entry.path()) {
                    Ok(stream) => {
                        for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
                            write_stdout_line(&format!(
                                "record\t{}\t{}\t{}\t{}",
                                escaped_path(entry.path()),
                                offset,
                                format_le32_fields(&stream[offset..], SO_RECORD_DWORDS),
                                bytes_to_hex(stream_tail(&stream, offset, SO_RECORD_BYTES))
                            ))?;
                        }
                    }
                    Err(error) => {
                        write_stdout_line(&format!(
                            "unreadable\t{}\t{}",
                            escaped_path(entry.path()),
                            error
                        ))?;
                    }
                }
            }
            Ok(())
        }
        Some("object-stream-candidates") => {
            let path = required_path(args.next(), "object-stream-candidates")?;
            let bytes = read_file(path)?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            let mut stream_count = 0usize;
            let mut unreadable_count = 0usize;
            let mut candidates = Vec::new();
            let mut reason_counts = BTreeMap::<&'static str, usize>::new();

            for entry in entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
            {
                stream_count += 1;
                let stream = match read_cfb_stream(&bytes, entry.path()) {
                    Ok(stream) => stream,
                    Err(error) => {
                        unreadable_count += 1;
                        write_stdout_line(&format!(
                            "unreadable\t{}\t{}",
                            escaped_path(entry.path()),
                            error
                        ))?;
                        continue;
                    }
                };

                let Some(candidate) = classify_object_stream_candidate(entry.path(), &stream)
                else {
                    continue;
                };
                for reason in &candidate.reasons {
                    *reason_counts.entry(reason).or_default() += 1;
                }
                candidates.push(candidate);
            }

            let visual_list_raster_count = candidates
                .iter()
                .filter(|candidate| candidate.visual_list.is_some())
                .count();
            let embedded_press_snapshot_count = candidates
                .iter()
                .filter(|candidate| candidate.embedded_press_snapshot.is_some())
                .count();
            let jseq3_formula_count = candidates
                .iter()
                .filter(|candidate| candidate.jseq3_formula.is_some())
                .count();
            write_stdout_line(&format!(
                "summary\tstreams={}\tcandidates={}\tunreadable={}\tobject-path={}\timage-path={}\tshape-path={}\ttable-path={}\tvisual-list-path={}\tvisual-list-raster={}\tfigure-link={}\tembedded-press-snapshot={}\tjseq3-formula={}\tso-marker={}\timage-signature={}\tsvg-signature={}\tdecoded=false",
                stream_count,
                candidates.len(),
                unreadable_count,
                object_stream_reason_count(&reason_counts, "object-path"),
                object_stream_reason_count(&reason_counts, "image-path"),
                object_stream_reason_count(&reason_counts, "shape-path"),
                object_stream_reason_count(&reason_counts, "table-path"),
                object_stream_reason_count(&reason_counts, "visual-list-path"),
                visual_list_raster_count,
                object_stream_reason_count(&reason_counts, "figure-link"),
                embedded_press_snapshot_count,
                jseq3_formula_count,
                object_stream_reason_count(&reason_counts, "so-marker"),
                object_stream_reason_count(&reason_counts, "image-signature"),
                object_stream_reason_count(&reason_counts, "svg-signature"),
            ))?;

            for (index, candidate) in candidates.iter().enumerate() {
                write_stdout_line(&format!(
                    "object-stream-candidate\t{}\tstream={}\tsize={}\treasons={}\timage-signatures={}\tsvg-offsets={}\tso-offsets={}\tvisual-list={}\tembedded-press-snapshot={}\tjseq3-formula={}\tprefix={}\tdecoded=false",
                    index,
                    escaped_path(&candidate.path),
                    candidate.size,
                    candidate.reasons.join(","),
                    format_object_signature_hits(&candidate.image_signature_hits),
                    format_usize_hit_list(&candidate.svg_offsets),
                    format_usize_hit_list(&candidate.so_offsets),
                    format_visual_list_candidate(candidate.visual_list.as_ref()),
                    format_embedded_press_snapshot_candidate(
                        candidate.embedded_press_snapshot.as_ref()
                    ),
                    format_jseq3_formula_candidate(candidate.jseq3_formula.as_ref()),
                    candidate.prefix_hex,
                ))?;
            }
            Ok(())
        }
        Some("object-ownership-references") => {
            let path = required_path(args.next(), "object-ownership-references")?;
            let bytes = read_file(path)?;
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let streams = readable_cfb_streams(&bytes)?;
            let mut source_count = 0usize;
            let mut reference_count = 0usize;
            let mut reported_offset_count = 0usize;
            let mut missing_target_count = 0usize;

            for candidate in document.object_stream_candidates() {
                let references = candidate.ownership_reference_candidates();
                if references.is_empty() {
                    continue;
                }
                source_count += 1;
                reference_count += references.len();
                for reference in references {
                    reported_offset_count += reference.offsets().len();
                    let Some(target_stream) = streams.get(reference.target_path()) else {
                        missing_target_count += 1;
                        write_stdout_line(&format!(
                            "object-ownership-reference\tsource={}\ttarget={}\tencoding={}\toffset=-\ttotal={}\ttarget-missing=true\tdecoded=false",
                            escaped_path(candidate.path()),
                            escaped_path(reference.target_path()),
                            reference.encoding(),
                            reference.total_matches()
                        ))?;
                        continue;
                    };

                    let pattern_len = object_reference_pattern_len(reference.encoding());
                    for offset in reference.offsets() {
                        let context = object_reference_context(target_stream, *offset, pattern_len);
                        write_stdout_line(&format!(
                            "object-ownership-reference\tsource={}\ttarget={}\tencoding={}\toffset={}\ttotal={}\tmod2={}\tmod4={}\twindow-start={}\twindow-hex={}\tle16={}\tbe16={}\tle32={}\tbe32={}\tdecoded=false",
                            escaped_path(candidate.path()),
                            escaped_path(reference.target_path()),
                            reference.encoding(),
                            offset,
                            reference.total_matches(),
                            offset % 2,
                            offset % 4,
                            context.start,
                            context.hex,
                            format_optional_u16_decimal(read_le16_candidate(
                                target_stream,
                                *offset
                            )),
                            format_optional_u16_decimal(read_be16_candidate(
                                target_stream,
                                *offset
                            )),
                            format_optional_u32(read_le32_candidate(target_stream, *offset)),
                            format_optional_u32(read_be32_at(target_stream, *offset))
                        ))?;
                    }
                }
            }

            write_stdout_line(&format!(
                "summary\tsources={}\treferences={}\treported-offsets={}\ttarget-missing={}\tdecoded=false",
                source_count, reference_count, reported_offset_count, missing_target_count
            ))?;
            Ok(())
        }
        Some("object-ownership-reference-fields") => {
            let path = required_path(args.next(), "object-ownership-reference-fields")?;
            let bytes = read_file(path)?;
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let mut summaries =
                BTreeMap::<ObjectReferenceFieldKey, ObjectReferenceFieldSummary>::new();
            let mut source_count = 0usize;
            let mut reference_count = 0usize;
            let mut reported_offset_count = 0usize;

            for candidate in document.object_stream_candidates() {
                let Some(ownership) = candidate.ownership_candidate() else {
                    continue;
                };
                let embedding_index = ownership.embedding_index();
                let references = candidate.ownership_reference_candidates();
                if references.is_empty() {
                    continue;
                }
                source_count += 1;
                reference_count += references.len();

                for reference in references {
                    let pattern_len = object_reference_pattern_len(reference.encoding());
                    for offset in reference.offsets() {
                        reported_offset_count += 1;
                        for stride in OBJECT_REFERENCE_FIELD_STRIDES {
                            let field_offset = offset % stride;
                            let key = ObjectReferenceFieldKey::new(
                                reference.target_path(),
                                reference.encoding(),
                                *stride,
                                field_offset,
                            );
                            let summary = summaries.entry(key).or_default();
                            summary.matches += 1;
                            summary.row_indexes.insert(offset / stride);
                            summary.source_streams.insert(candidate.path().to_string());
                            if let Some(index) = embedding_index {
                                summary.embedding_indexes.insert(index);
                            }
                            if field_offset + pattern_len > *stride {
                                summary.cross_row_matches += 1;
                            }
                        }
                    }
                }
            }

            write_stdout_line(&format!(
                "summary\tsources={}\treferences={}\treported-offsets={}\tfield-groups={}\tstrides={}\tdecoded=false",
                source_count,
                reference_count,
                reported_offset_count,
                summaries.len(),
                format_usize_values(OBJECT_REFERENCE_FIELD_STRIDES)
            ))?;

            for (key, summary) in summaries {
                write_stdout_line(&format!(
                    "object-ownership-reference-field\ttarget={}\tencoding={}\tstride={}\tfield-offset={}\tmatches={}\tsource-count={}\tembedding-indexes={}\trow-indexes={}\tcross-row={}\tdecoded=false",
                    escaped_path(&key.target_path),
                    key.encoding,
                    key.stride,
                    key.field_offset,
                    summary.matches,
                    summary.source_streams.len(),
                    format_usize_set(&summary.embedding_indexes),
                    format_usize_set(&summary.row_indexes),
                    summary.cross_row_matches
                ))?;
            }
            Ok(())
        }
        Some("object-frame-reference-records") => {
            let path = required_path(args.next(), "object-frame-reference-records")?;
            let bytes = read_file(path)?;
            let collection = collect_object_frame_reference_records(&bytes)?;
            for record in &collection.records {
                write_stdout_line(&format!(
                    "object-frame-reference-record\tsource={}\tembedding={}\ttarget={}\tencoding={}\tstride={}\tfield-offset={}\toffset={}\trow-index={}\trow-start={}\tcandidate={}\trow-hex={}\trow-be16={}\trow-le16={}\trow-be32={}\trow-le32={}\tdecoded=false",
                    escaped_path(&record.source_path),
                    format_optional_usize(record.embedding_index),
                    escaped_path(&record.target_path),
                    record.encoding,
                    record.stride,
                    record.field_offset,
                    record.offset,
                    record.row_index,
                    record.row_start,
                    record.candidate,
                    bytes_to_hex(&record.row),
                    format_be16_hex_fields(&record.row),
                    format_le16_fields(&record.row),
                    format_be32_fields(&record.row),
                    format_le32_fields(&record.row, record.stride / 4)
                ))?;
            }

            write_stdout_line(&format!(
                "summary\tsources={}\tframe-references={}\trecords={}\tskipped={}\tcandidates={}\tdecoded=false",
                collection.source_count,
                collection.reference_count,
                collection.records.len(),
                collection.skipped_count,
                format_frame_reference_record_candidates()
            ))?;
            Ok(())
        }
        Some("object-frame-record-families") => {
            let path = required_path(args.next(), "object-frame-record-families")?;
            let bytes = read_file(path)?;
            let collection = collect_object_frame_reference_records(&bytes)?;
            let mut families = BTreeMap::<String, ObjectFrameRecordFamilySummary>::new();

            for record in &collection.records {
                let family = classify_object_frame_reference_record(record);
                let summary = families.entry(family.to_string()).or_default();
                summary.rows += 1;
                summary.candidates.insert(record.candidate.clone());
                if let Some(index) = record.embedding_index {
                    summary.embedding_indexes.insert(index);
                }
                summary.examples.insert(bytes_to_hex(&record.row));
            }

            for (family, summary) in &families {
                write_stdout_line(&format!(
                    "object-frame-record-family\tfamily={}\trows={}\tcandidates={}\tembeddings={}\texamples={}\tdecoded=false",
                    family,
                    summary.rows,
                    format_string_set(&summary.candidates),
                    format_usize_set(&summary.embedding_indexes),
                    format_string_set(&summary.examples)
                ))?;
            }

            write_stdout_line(&format!(
                "summary\tfamilies={}\trecords={}\tskipped={}\tcandidates={}\tdecoded=false",
                families.len(),
                collection.records.len(),
                collection.skipped_count,
                format_frame_reference_record_candidates()
            ))?;
            Ok(())
        }
        Some("object-frame-row-links") => {
            let path = required_path(args.next(), "object-frame-row-links")?;
            let bytes = read_file(path)?;
            let collection = collect_object_frame_reference_records(&bytes)?;
            let row12_records = collection
                .records
                .iter()
                .filter(|record| record.stride == 12)
                .collect::<Vec<_>>();
            let mut row20_count = 0usize;
            let mut linked_count = 0usize;
            let mut relation_counts = BTreeMap::<String, usize>::new();
            let mut pair_counts = BTreeMap::<String, usize>::new();

            for record in collection
                .records
                .iter()
                .filter(|record| record.stride == 20 && record.field_offset == 15)
            {
                row20_count += 1;
                let suffix = object_frame_row_suffix(record, 12).unwrap_or(&[]);
                let (relation, matched) =
                    find_object_frame_suffix_match(record, suffix, &row12_records);
                if matched.is_some() {
                    linked_count += 1;
                }
                *relation_counts.entry(relation.to_string()).or_insert(0) += 1;

                let row_family = classify_object_frame_reference_record(record);
                let suffix_family = matched
                    .map(classify_object_frame_reference_record)
                    .unwrap_or("unmatched-suffix");
                *pair_counts
                    .entry(format!("{row_family}->{suffix_family}"))
                    .or_insert(0) += 1;

                write_stdout_line(&format!(
                    "object-frame-row-link\tsource={}\tembedding={}\trow20-family={}\trow20-start={}\trow20-index={}\tprefix-hex={}\tsuffix-hex={}\trelation={}\tsuffix-family={}\tmatched-source={}\tmatched-row-start={}\tmatched-row-index={}\tdecoded=false",
                    escaped_path(&record.source_path),
                    format_optional_usize(record.embedding_index),
                    row_family,
                    record.row_start,
                    record.row_index,
                    bytes_to_hex(object_frame_row_prefix(record, 12).unwrap_or(&[])),
                    bytes_to_hex(suffix),
                    relation,
                    suffix_family,
                    format_optional_text(matched.map(|record| record.source_path.as_str())),
                    format_optional_usize(matched.map(|record| record.row_start)),
                    format_optional_usize(matched.map(|record| record.row_index))
                ))?;
            }

            write_stdout_line(&format!(
                "summary\trow20={}\tlinked={}\tunlinked={}\trelations={}\tfamily-pairs={}\tdecoded=false",
                row20_count,
                linked_count,
                row20_count.saturating_sub(linked_count),
                format_string_counts(&relation_counts),
                format_string_counts(&pair_counts)
            ))?;
            Ok(())
        }
        Some("object-image-frame-candidates") => {
            let path = required_path(args.next(), "object-image-frame-candidates")?;
            let bytes = read_file(path)?;
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let mut source_count = 0usize;
            let mut frame_linked_count = 0usize;
            let mut missing_frame_count = 0usize;
            let mut preferred_counts = BTreeMap::<String, usize>::new();
            let mut total_frame_rows = 0usize;
            let mut total_dimensioned_payloads = 0usize;
            let mut total_aspect_candidates = 0usize;

            for candidate in document
                .object_stream_candidates()
                .iter()
                .filter(|candidate| !candidate.image_payload_spans().is_empty())
            {
                source_count += 1;
                let summary = summarize_object_image_frame_candidate(candidate);
                total_frame_rows += summary.frame_rows;
                let dimensioned_payloads =
                    object_payload_dimension_count(candidate.image_payload_spans());
                let aspect_candidates = coordinate_payload_aspect_candidate_count(
                    &summary.coordinate_pairs,
                    candidate.image_payload_spans(),
                );
                total_dimensioned_payloads += dimensioned_payloads;
                total_aspect_candidates += aspect_candidates;
                if summary.frame_rows == 0 {
                    missing_frame_count += 1;
                } else {
                    frame_linked_count += 1;
                }
                *preferred_counts
                    .entry(summary.preferred.to_string())
                    .or_default() += 1;

                write_stdout_line(&format!(
                    "object-image-frame-candidate\tsource={}\tembedding={}\tpayloads={}\tpayload-kinds={}\tpayload-dimensions={}\tdimensioned-payloads={}\tframe-rows={}\trow-families={}\trow12-tail-coordinate={}\trow12-tail-zero={}\trow20-tail-window={}\trow20-linked={}\tle-row12={}\tpreferred={}\tcoordinate-pairs={}\tbest-coordinate-aspect-delta-permille={}\tdecoded=false",
                    escaped_path(candidate.path()),
                    format_optional_usize(summary.embedding_index),
                    candidate.image_payload_spans().len(),
                    format_string_set(&summary.payload_kinds),
                    format_object_payload_dimensions(candidate.image_payload_spans()),
                    dimensioned_payloads,
                    summary.frame_rows,
                    format_string_counts(&summary.family_counts),
                    summary.row12_tail_coordinate,
                    summary.row12_tail_zero,
                    summary.row20_tail_window,
                    summary.row20_linked,
                    summary.le_row12,
                    summary.preferred,
                    format_object_frame_coordinate_pairs(&summary.coordinate_pairs),
                    format_optional_u64(best_coordinate_payload_aspect_delta_permille(
                        &summary.coordinate_pairs,
                        candidate.image_payload_spans()
                    ))
                ))?;
            }

            write_stdout_line(&format!(
                "summary\tsources={}\tframe-linked={}\tmissing-frame={}\tframe-rows={}\tdimensioned-payloads={}\taspect-candidates={}\tpreferred={}\tdecoded=false",
                source_count,
                frame_linked_count,
                missing_frame_count,
                total_frame_rows,
                total_dimensioned_payloads,
                total_aspect_candidates,
                format_string_counts(&preferred_counts)
            ))?;
            Ok(())
        }
        Some("object-fdm-image-candidates") => {
            let path = required_path(args.next(), "object-fdm-image-candidates")?;
            let bytes = read_file(path)?;
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let mut source_count = 0usize;
            let mut candidate_count = 0usize;
            let mut image_hit_count = 0usize;
            let mut complete_payload_count = 0usize;
            let mut plausible_bbox_count = 0usize;
            let renderable_count = 0usize;

            for candidate in document.object_stream_candidates() {
                let image_entries = candidate
                    .fdm_index_entry_candidates()
                    .iter()
                    .filter(|entry| !entry.segment_image_signature_hits().is_empty())
                    .collect::<Vec<_>>();
                if image_entries.is_empty() {
                    continue;
                }

                source_count += 1;
                for entry in image_entries {
                    let bbox = entry.bbox();
                    let normalized = normalize_fdm_bbox(bbox);
                    let bbox_order = fdm_bbox_order(bbox);
                    let bbox_width = normalized.2.saturating_sub(normalized.0);
                    let bbox_height = normalized.3.saturating_sub(normalized.1);
                    let bbox_plausible = fdm_bbox_is_plausible(bbox);
                    let complete_payloads = fdm_entry_complete_payload_count(candidate, entry);
                    candidate_count += 1;
                    image_hit_count += entry.segment_image_signature_hits().len();
                    complete_payload_count += complete_payloads;
                    if bbox_plausible {
                        plausible_bbox_count += 1;
                    }

                    write_stdout_line(&format!(
                        "object-fdm-image-candidate\tsource={}\tindex={}\trow={}\tvector-offset={}\tnext-vector-offset={}\tvector-length={}\tkind=0x{:04x}\tbbox={},{},{},{}\tnormalized-bbox={},{},{},{}\tbbox-size={}x{}\tbbox-order={}\tbbox-plausible={}\timage-hits={}\tcomplete-payloads={}\timage-signatures={}\tsegment-image-signatures={}\trenderable=false\treason=page-placement-unproven\tdecoded=false",
                        escaped_path(candidate.path()),
                        escaped_path(entry.index_path()),
                        entry.row_index(),
                        entry.vector_offset(),
                        entry.next_vector_offset(),
                        entry.vector_len(),
                        entry.kind(),
                        bbox.left(),
                        bbox.top(),
                        bbox.right(),
                        bbox.bottom(),
                        normalized.0,
                        normalized.1,
                        normalized.2,
                        normalized.3,
                        bbox_width,
                        bbox_height,
                        bbox_order,
                        bbox_plausible,
                        entry.segment_image_signature_hits().len(),
                        complete_payloads,
                        format_model_object_signature_hits(entry.image_signature_hits()),
                        format_model_object_signature_hits(entry.segment_image_signature_hits())
                    ))?;
                }
            }

            write_stdout_line(&format!(
                "summary\tsources={}\tcandidates={}\timage-hits={}\tcomplete-payloads={}\tbbox-plausible={}\trenderable={}\tdecoded=false",
                source_count,
                candidate_count,
                image_hit_count,
                complete_payload_count,
                plausible_bbox_count,
                renderable_count
            ))?;
            Ok(())
        }
        Some("object-fdm-frame-links") => {
            let path = required_path(args.next(), "object-fdm-frame-links")?;
            let bytes = read_file(path)?;
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let mut source_count = 0usize;
            let mut candidate_count = 0usize;
            let mut frame_linked_count = 0usize;
            let mut missing_frame_count = 0usize;
            let mut complete_payload_count = 0usize;
            let renderable_count = 0usize;

            for candidate in document.object_stream_candidates() {
                let image_entries = candidate
                    .fdm_index_entry_candidates()
                    .iter()
                    .filter(|entry| !entry.segment_image_signature_hits().is_empty())
                    .collect::<Vec<_>>();
                if image_entries.is_empty() {
                    continue;
                }

                source_count += 1;
                for entry in image_entries {
                    let complete_payload_spans = fdm_entry_complete_payload_spans(candidate, entry);
                    let complete_payloads = complete_payload_spans.len();
                    let frame_record = fdm_frame_record_for_entry(
                        document.object_frame_records(),
                        entry.row_index(),
                    );
                    candidate_count += 1;
                    complete_payload_count += complete_payloads;
                    if frame_record.is_some() {
                        frame_linked_count += 1;
                    } else {
                        missing_frame_count += 1;
                    }

                    write_stdout_line(&format!(
                        "object-fdm-frame-link\tsource={}\tindex={}\trow={}\timage-hits={}\tcomplete-payloads={}\tframe-linked={}\tframe-source={}\tframe-row={}\tframe-start={}\tframe-object-id={}\tframe-kind={}\tframe-type={}\tframe-geometry={}\tframe-size={}\tpayload-dimensions={}\tdimensioned-payloads={}\tbest-aspect-delta-permille={}\tlink-basis=fdm-row-index-to-frame-object-id\trenderable=false\treason=page-placement-unproven\tdecoded=false",
                        escaped_path(candidate.path()),
                        escaped_path(entry.index_path()),
                        entry.row_index(),
                        entry.segment_image_signature_hits().len(),
                        complete_payloads,
                        frame_record.is_some(),
                        format_optional_text(
                            frame_record.map(ObjectFrameRecordCandidate::source_path)
                        ),
                        format_optional_usize(
                            frame_record.map(ObjectFrameRecordCandidate::row_index)
                        ),
                        format_optional_usize(
                            frame_record.map(ObjectFrameRecordCandidate::row_start)
                        ),
                        format_optional_u16_decimal(
                            frame_record.map(ObjectFrameRecordCandidate::object_id)
                        ),
                        format_optional_u16_hex(
                            frame_record.map(ObjectFrameRecordCandidate::record_kind)
                        ),
                        format_optional_u16_hex(
                            frame_record.map(ObjectFrameRecordCandidate::object_type)
                        ),
                        format_optional_frame_geometry(frame_record),
                        format_optional_frame_size(frame_record),
                        format_fdm_payload_dimensions(&complete_payload_spans),
                        fdm_payload_dimension_count(&complete_payload_spans),
                        format_optional_u64(best_frame_payload_aspect_delta_permille(
                            frame_record,
                            &complete_payload_spans
                        ))
                    ))?;
                }
            }

            write_stdout_line(&format!(
                "summary\tsources={}\tcandidates={}\tframe-linked={}\tmissing-frame={}\tcomplete-payloads={}\trenderable={}\tdecoded=false",
                source_count,
                candidate_count,
                frame_linked_count,
                missing_frame_count,
                complete_payload_count,
                renderable_count
            ))?;
            Ok(())
        }
        Some("object-fdm-index") => {
            let path = required_path(args.next(), "object-fdm-index")?;
            let bytes = read_file(path)?;
            let streams = readable_cfb_streams(&bytes)?;
            let mut index_count = 0usize;
            let mut parsed_entries = 0usize;
            let mut entries_with_images = 0usize;
            let mut image_hit_count = 0usize;
            let mut missing_vector_count = 0usize;

            for (index_path, index_stream) in streams
                .iter()
                .filter(|(path, _)| path.ends_with("/FDMIndex"))
            {
                index_count += 1;
                let Some(vector_path) = fdm_vector_path_for_index(index_path) else {
                    missing_vector_count += 1;
                    continue;
                };
                let Some(vector_stream) = streams.get(&vector_path) else {
                    missing_vector_count += 1;
                    write_stdout_line(&format!(
                        "object-fdm-index-summary\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes=0\tdeclared-count={}\tparsed-entries=0\ttrailing-bytes=0\tentries-with-image=0\timage-hits=0\tvector-missing=true\tdecoded=false",
                        escaped_path(index_path),
                        escaped_path(&vector_path),
                        index_stream.len(),
                        format_optional_usize(fdm_index_declared_count(index_stream))
                    ))?;
                    continue;
                };

                let entries = parse_fdm_index_entries(index_stream, vector_stream.len());
                let vector_hits = image_signature_hits(vector_stream);
                let mut index_entries_with_images = 0usize;
                let mut index_image_hits = 0usize;
                for entry in &entries {
                    let segment = fdm_vector_segment(entry.vector_offset, &entries, vector_stream);
                    let segment_hits =
                        fdm_segment_signature_hits(&vector_hits, segment.start, segment.end);
                    if !segment_hits.is_empty() {
                        index_entries_with_images += 1;
                        index_image_hits += segment_hits.len();
                    }
                }
                parsed_entries += entries.len();
                entries_with_images += index_entries_with_images;
                image_hit_count += index_image_hits;

                write_stdout_line(&format!(
                    "object-fdm-index-summary\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes={}\tdeclared-count={}\tparsed-entries={}\ttrailing-bytes={}\tentries-with-image={}\timage-hits={}\tvector-missing=false\tdecoded=false",
                    escaped_path(index_path),
                    escaped_path(&vector_path),
                    index_stream.len(),
                    vector_stream.len(),
                    format_optional_usize(fdm_index_declared_count(index_stream)),
                    entries.len(),
                    fdm_index_trailing_bytes(index_stream),
                    index_entries_with_images,
                    index_image_hits
                ))?;

                for entry in entries.iter() {
                    let segment = fdm_vector_segment(entry.vector_offset, &entries, vector_stream);
                    let segment_hits =
                        fdm_segment_signature_hits(&vector_hits, segment.start, segment.end);
                    let relative_hits = fdm_relative_signature_hits(&segment_hits, segment.start);
                    let vector_prefix = vector_stream
                        .get(segment.start..segment.end)
                        .unwrap_or_default();
                    write_stdout_line(&format!(
                        "object-fdm-index-entry\tindex={}\tvector={}\trow={}\tindex-offset={}\tvector-offset={}\tnext-vector-offset={}\tvector-length={}\tkind=0x{:04x}\tbbox={},{},{},{}\tvalid-vector-offset={}\tvector-prefix={}\timage-signatures={}\tsegment-image-signatures={}\tdecoded=false",
                        escaped_path(index_path),
                        escaped_path(&vector_path),
                        entry.row_index,
                        entry.index_offset,
                        entry.vector_offset,
                        segment.end,
                        segment.end.saturating_sub(segment.start),
                        entry.kind,
                        entry.left,
                        entry.top,
                        entry.right,
                        entry.bottom,
                        entry.valid_vector_offset,
                        format_hex_preview(vector_prefix, OBJECT_STREAM_PREFIX_PREVIEW_BYTES),
                        format_object_signature_hits(&segment_hits),
                        format_object_signature_hits(&relative_hits)
                    ))?;
                }
            }

            write_stdout_line(&format!(
                "summary\tindexes={}\tentries={}\tentries-with-image={}\timage-hits={}\tmissing-vectors={}\tdecoded=false",
                index_count,
                parsed_entries,
                entries_with_images,
                image_hit_count,
                missing_vector_count
            ))?;
            Ok(())
        }
        Some("object-fdm-index-shape") => {
            let path = required_path(args.next(), "object-fdm-index-shape")?;
            let bytes = read_file(path)?;
            let streams = readable_cfb_streams(&bytes)?;
            let mut index_count = 0usize;
            let mut header_v1_count = 0usize;
            let mut unknown_header_count = 0usize;
            let mut declared_plausible_count = 0usize;
            let mut stream_rows = 0usize;
            let mut stream_invalid_rows = 0usize;
            let mut declared_rows = 0usize;
            let mut declared_invalid_rows = 0usize;
            let mut declared_image_hits = 0usize;
            let mut shape_counts = BTreeMap::<String, usize>::new();

            for (index_path, index_stream) in streams
                .iter()
                .filter(|(path, _)| path.ends_with("/FDMIndex"))
            {
                index_count += 1;
                let Some(vector_path) = fdm_vector_path_for_index(index_path) else {
                    continue;
                };
                let header_family = fdm_index_header_family(index_stream);
                if header_family == FDM_INDEX_HEADER_V1 {
                    header_v1_count += 1;
                } else {
                    unknown_header_count += 1;
                }
                let declared_count = fdm_index_declared_count(index_stream);
                let Some(vector_stream) = streams.get(&vector_path) else {
                    *shape_counts
                        .entry("missing-vector".to_string())
                        .or_default() += 1;
                    write_stdout_line(&format!(
                        "object-fdm-index-shape\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes=0\theader-family={}\theader-u16be={}\tdeclared-count={}\tdeclared-plausible=false\trow22-stream-rows=0\trow22-trailing-bytes=0\tdeclared-row22=-\tpost-declared-bytes=-\tall-valid=0\tall-invalid=0\tall-image-rows=0\tall-image-hits=0\tdeclared-valid=0\tdeclared-invalid=0\tdeclared-image-rows=0\tdeclared-image-hits=0\tfirst-invalid-row=-\tfirst-invalid-offset=-\tshape=missing-vector\tdecoded=false",
                        escaped_path(index_path),
                        escaped_path(&vector_path),
                        index_stream.len(),
                        header_family,
                        format_fdm_index_header_u16(index_stream),
                        format_optional_usize(declared_count)
                    ))?;
                    continue;
                };

                let entries = parse_fdm_index_entries(index_stream, vector_stream.len());
                let vector_hits = image_signature_hits(vector_stream);
                let declared_plausible = header_family == FDM_INDEX_HEADER_V1
                    && declared_count.is_some_and(|count| count <= entries.len());
                let declared_entry_count = if declared_plausible {
                    declared_count.unwrap_or_default()
                } else {
                    0
                };
                let declared_entries = &entries[..declared_entry_count];
                let all_stats = fdm_index_entry_stats(&entries, &vector_hits, vector_stream);
                let declared_stats =
                    fdm_index_entry_stats(declared_entries, &vector_hits, vector_stream);
                let post_declared_bytes = declared_plausible.then(|| {
                    index_stream.len().saturating_sub(
                        FDM_INDEX_HEADER_BYTES + declared_entry_count * FDM_INDEX_ENTRY_BYTES,
                    )
                });
                let shape = fdm_index_shape_family(
                    header_family,
                    declared_plausible,
                    entries.len(),
                    fdm_index_trailing_bytes(index_stream),
                    declared_entry_count,
                    &all_stats,
                    &declared_stats,
                );

                if declared_plausible {
                    declared_plausible_count += 1;
                }
                stream_rows += all_stats.rows;
                stream_invalid_rows += all_stats.invalid_offsets;
                declared_rows += declared_stats.rows;
                declared_invalid_rows += declared_stats.invalid_offsets;
                declared_image_hits += declared_stats.image_hits;
                *shape_counts.entry(shape.to_string()).or_default() += 1;

                write_stdout_line(&format!(
                    "object-fdm-index-shape\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes={}\theader-family={}\theader-u16be={}\tdeclared-count={}\tdeclared-plausible={}\trow22-stream-rows={}\trow22-trailing-bytes={}\tdeclared-row22={}\tpost-declared-bytes={}\tall-valid={}\tall-invalid={}\tall-image-rows={}\tall-image-hits={}\tdeclared-valid={}\tdeclared-invalid={}\tdeclared-image-rows={}\tdeclared-image-hits={}\tfirst-invalid-row={}\tfirst-invalid-offset={}\tshape={}\tdecoded=false",
                    escaped_path(index_path),
                    escaped_path(&vector_path),
                    index_stream.len(),
                    vector_stream.len(),
                    header_family,
                    format_fdm_index_header_u16(index_stream),
                    format_optional_usize(declared_count),
                    declared_plausible,
                    entries.len(),
                    fdm_index_trailing_bytes(index_stream),
                    format_optional_usize(declared_plausible.then_some(declared_entry_count)),
                    format_optional_usize(post_declared_bytes),
                    all_stats.valid_offsets,
                    all_stats.invalid_offsets,
                    all_stats.image_rows,
                    all_stats.image_hits,
                    declared_stats.valid_offsets,
                    declared_stats.invalid_offsets,
                    declared_stats.image_rows,
                    declared_stats.image_hits,
                    format_optional_usize(all_stats.first_invalid_row),
                    format_optional_usize(all_stats.first_invalid_offset),
                    shape
                ))?;
            }

            write_stdout_line(&format!(
                "summary\tindexes={}\theader-v1={}\tunknown-header={}\tdeclared-plausible={}\tstream-rows={}\tstream-invalid={}\tdeclared-rows={}\tdeclared-invalid={}\tdeclared-image-hits={}\tshapes={}\tdecoded=false",
                index_count,
                header_v1_count,
                unknown_header_count,
                declared_plausible_count,
                stream_rows,
                stream_invalid_rows,
                declared_rows,
                declared_invalid_rows,
                declared_image_hits,
                format_string_counts(&shape_counts)
            ))?;
            Ok(())
        }
        Some("object-fdm-index-rows") => {
            let path = required_path(args.next(), "object-fdm-index-rows")?;
            let bytes = read_file(path)?;
            let streams = readable_cfb_streams(&bytes)?;
            let mut index_count = 0usize;
            let mut row_count = 0usize;
            let mut declared_rows = 0usize;
            let mut post_declared_rows = 0usize;
            let mut raw_rows = 0usize;
            let mut valid_rows = 0usize;
            let mut invalid_rows = 0usize;
            let mut image_hits = 0usize;
            let mut missing_vector_count = 0usize;
            let mut role_counts = BTreeMap::<String, usize>::new();

            for (index_path, index_stream) in streams
                .iter()
                .filter(|(path, _)| path.ends_with("/FDMIndex"))
            {
                index_count += 1;
                let Some(vector_path) = fdm_vector_path_for_index(index_path) else {
                    continue;
                };
                let header_family = fdm_index_header_family(index_stream);
                let declared_count = fdm_index_declared_count(index_stream);
                let Some(vector_stream) = streams.get(&vector_path) else {
                    missing_vector_count += 1;
                    write_stdout_line(&format!(
                        "object-fdm-index-rows-summary\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes=0\theader-family={}\tdeclared-count={}\trows=0\tdeclared-rows=0\tpost-declared-rows=0\traw-rows=0\tvalid-rows=0\tinvalid-rows=0\timage-hits=0\troles=-\tvector-missing=true\tdecoded=false",
                        escaped_path(index_path),
                        escaped_path(&vector_path),
                        index_stream.len(),
                        header_family,
                        format_optional_usize(declared_count)
                    ))?;
                    continue;
                };

                let entries = parse_fdm_index_entries(index_stream, vector_stream.len());
                let declared_plausible = header_family == FDM_INDEX_HEADER_V1
                    && declared_count.is_some_and(|count| count <= entries.len());
                let declared_entry_count = if declared_plausible {
                    declared_count.unwrap_or_default()
                } else {
                    0
                };
                let vector_hits = image_signature_hits(vector_stream);
                let mut index_rows = 0usize;
                let mut index_declared_rows = 0usize;
                let mut index_post_declared_rows = 0usize;
                let mut index_raw_rows = 0usize;
                let mut index_valid_rows = 0usize;
                let mut index_invalid_rows = 0usize;
                let mut index_image_hits = 0usize;
                let mut index_role_counts = BTreeMap::<String, usize>::new();

                for entry in &entries {
                    let scope = fdm_index_row_scope(
                        entry.row_index,
                        declared_plausible,
                        declared_entry_count,
                    );
                    let role = fdm_index_row_role(entry);
                    let segment = fdm_vector_segment(entry.vector_offset, &entries, vector_stream);
                    let segment_hits =
                        fdm_segment_signature_hits(&vector_hits, segment.start, segment.end);
                    let relative_hits = fdm_relative_signature_hits(&segment_hits, segment.start);

                    index_rows += 1;
                    match scope {
                        "declared" => index_declared_rows += 1,
                        "post-declared" => index_post_declared_rows += 1,
                        _ => index_raw_rows += 1,
                    }
                    if entry.valid_vector_offset {
                        index_valid_rows += 1;
                    } else {
                        index_invalid_rows += 1;
                    }
                    index_image_hits += segment_hits.len();
                    *index_role_counts.entry(role.to_string()).or_default() += 1;

                    write_stdout_line(&format!(
                        "object-fdm-index-row\tindex={}\tvector={}\trow={}\tscope={}\trole={}\tindex-offset={}\tvector-offset={}\tnext-vector-offset={}\tvector-length={}\tkind=0x{:04x}\tbbox={},{},{},{}\tvalid-vector-offset={}\tbe16={}\ti16={}\trow-bytes={}\timage-signatures={}\tsegment-image-signatures={}\tdecoded=false",
                        escaped_path(index_path),
                        escaped_path(&vector_path),
                        entry.row_index,
                        scope,
                        role,
                        entry.index_offset,
                        entry.vector_offset,
                        segment.end,
                        segment.end.saturating_sub(segment.start),
                        entry.kind,
                        entry.left,
                        entry.top,
                        entry.right,
                        entry.bottom,
                        entry.valid_vector_offset,
                        format_be16_hex_fields(&entry.row),
                        format_be16_signed_fields(&entry.row),
                        format_hex_preview(&entry.row, FDM_INDEX_ENTRY_BYTES),
                        format_object_signature_hits(&segment_hits),
                        format_object_signature_hits(&relative_hits)
                    ))?;
                }

                row_count += index_rows;
                declared_rows += index_declared_rows;
                post_declared_rows += index_post_declared_rows;
                raw_rows += index_raw_rows;
                valid_rows += index_valid_rows;
                invalid_rows += index_invalid_rows;
                image_hits += index_image_hits;
                for (role, count) in index_role_counts.iter() {
                    *role_counts.entry(role.clone()).or_default() += *count;
                }

                write_stdout_line(&format!(
                    "object-fdm-index-rows-summary\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes={}\theader-family={}\tdeclared-count={}\trows={}\tdeclared-rows={}\tpost-declared-rows={}\traw-rows={}\tvalid-rows={}\tinvalid-rows={}\timage-hits={}\troles={}\tvector-missing=false\tdecoded=false",
                    escaped_path(index_path),
                    escaped_path(&vector_path),
                    index_stream.len(),
                    vector_stream.len(),
                    header_family,
                    format_optional_usize(declared_count),
                    index_rows,
                    index_declared_rows,
                    index_post_declared_rows,
                    index_raw_rows,
                    index_valid_rows,
                    index_invalid_rows,
                    index_image_hits,
                    format_string_counts(&index_role_counts)
                ))?;
            }

            write_stdout_line(&format!(
                "summary\tindexes={}\trows={}\tdeclared-rows={}\tpost-declared-rows={}\traw-rows={}\tvalid-rows={}\tinvalid-rows={}\timage-hits={}\tmissing-vectors={}\troles={}\tdecoded=false",
                index_count,
                row_count,
                declared_rows,
                post_declared_rows,
                raw_rows,
                valid_rows,
                invalid_rows,
                image_hits,
                missing_vector_count,
                format_string_counts(&role_counts)
            ))?;
            Ok(())
        }
        Some("so-record-clusters") => {
            let path = required_path(args.next(), "so-record-clusters")?;
            let bytes = read_file(path)?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            let mut clusters = BTreeMap::<Vec<u8>, Vec<String>>::new();
            for entry in entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
            {
                let stream = match read_cfb_stream(&bytes, entry.path()) {
                    Ok(stream) => stream,
                    Err(error) => {
                        write_stdout_line(&format!(
                            "unreadable\t{}\t{}",
                            escaped_path(entry.path()),
                            error
                        ))?;
                        continue;
                    }
                };
                for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
                    let raw = stream_tail(&stream, offset, SO_RECORD_BYTES).to_vec();
                    clusters.entry(raw).or_default().push(format!(
                        "{}@{}",
                        escaped_path(entry.path()),
                        offset
                    ));
                }
            }

            for (raw, locations) in clusters {
                write_stdout_line(&format!(
                    "cluster\t{}\t{}\t{}\t{}",
                    locations.len(),
                    format_le32_fields(&raw, SO_RECORD_DWORDS),
                    bytes_to_hex(&raw),
                    locations.join(",")
                ))?;
            }
            Ok(())
        }
        Some("so-record-fields") => {
            let path = required_path(args.next(), "so-record-fields")?;
            let bytes = read_file(path)?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            for entry in entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
            {
                let stream = match read_cfb_stream(&bytes, entry.path()) {
                    Ok(stream) => stream,
                    Err(error) => {
                        write_stdout_line(&format!(
                            "unreadable\t{}\t{}",
                            escaped_path(entry.path()),
                            error
                        ))?;
                        continue;
                    }
                };
                for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
                    for (field_index, field) in
                        le32_dwords(stream_tail(&stream, offset, SO_RECORD_BYTES)).enumerate()
                    {
                        write_stdout_line(&format!(
                            "field\t{}\t{}\t{}\t0x{:08x}\t{}\t{}\t0x{:04x}\t{}\t0x{:04x}\t{}",
                            escaped_path(entry.path()),
                            offset,
                            field_index,
                            field,
                            field,
                            field as i32,
                            field as u16,
                            field as u16,
                            (field >> 16) as u16,
                            (field >> 16) as u16
                        ))?;
                    }
                }
            }
            Ok(())
        }
        Some("so-record-geometry") => {
            let path = required_path(args.next(), "so-record-geometry")?;
            let bytes = read_file(path)?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            for entry in entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
            {
                let stream = match read_cfb_stream(&bytes, entry.path()) {
                    Ok(stream) => stream,
                    Err(error) => {
                        write_stdout_line(&format!(
                            "unreadable\t{}\t{}",
                            escaped_path(entry.path()),
                            error
                        ))?;
                        continue;
                    }
                };
                for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
                    let raw = stream_tail(&stream, offset, SO_RECORD_BYTES);
                    let fields = le32_dwords(raw).collect::<Vec<_>>();
                    let (f1, f2, f3, f4, xyxy_width, xyxy_height, xywh_right, xywh_bottom) =
                        format_so_geometry_candidate(&fields);
                    write_stdout_line(&format!(
                        "candidate\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                        escaped_path(entry.path()),
                        offset,
                        classify_so_geometry_fields(&fields),
                        f1,
                        f2,
                        f3,
                        f4,
                        xyxy_width,
                        xyxy_height,
                        xywh_right,
                        xywh_bottom,
                        bytes_to_hex(raw)
                    ))?;
                }
            }
            Ok(())
        }
        Some("so-record-halves") => {
            let path = required_path(args.next(), "so-record-halves")?;
            let bytes = read_file(path)?;
            let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
            for entry in entries
                .iter()
                .filter(|entry| entry.kind() == EntryKind::Stream)
            {
                let stream = match read_cfb_stream(&bytes, entry.path()) {
                    Ok(stream) => stream,
                    Err(error) => {
                        write_stdout_line(&format!(
                            "unreadable\t{}\t{}",
                            escaped_path(entry.path()),
                            error
                        ))?;
                        continue;
                    }
                };
                for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
                    let raw = stream_tail(&stream, offset, SO_RECORD_BYTES);
                    let fields = le32_dwords(raw).collect::<Vec<_>>();
                    write_stdout_line(&format!(
                        "halves\t{}\t{}\t{}\tlo_u16={}\thi_u16={}\tlo_i16={}\thi_i16={}\t{}",
                        escaped_path(entry.path()),
                        offset,
                        classify_so_geometry_fields(&fields),
                        format_so_u16_halves(&fields, false),
                        format_so_u16_halves(&fields, true),
                        format_so_i16_halves(&fields, false),
                        format_so_i16_halves(&fields, true),
                        bytes_to_hex(raw)
                    ))?;
                }
            }
            Ok(())
        }
        Some("cat") => {
            let path = required_path(args.next(), "cat")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            write_stdout(payload.text())?;
            Ok(())
        }
        Some("text-tokens") => {
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
        Some("text-control-context") => {
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
        Some("text-control-clusters") => {
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
                if filter.is_none_or(|code| cluster.iter().any(|entry| entry.code() == Some(code)))
                {
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
        Some("text-control-ranges") => {
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
        Some("text-positions") => {
            let path = required_path(args.next(), "text-positions")?;
            let bytes = read_file(path)?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.entries().is_empty() {
                return Err("DocumentTextPositionTables missing MarkV.01 table".into());
            }
            for entry in table.entries() {
                write_stdout_line(&format!("{}\t{}", entry.id(), entry.offset()))?;
            }
            Ok(())
        }
        Some("text-position-mark-header") => {
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
        Some("text-position-mark-summary") => {
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

            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
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
        Some("text-position-counts") => {
            let path = required_path(args.next(), "text-position-counts")?;
            let bytes = read_file(path)?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            let Some(header) = table.text_count_header() else {
                return Err("DocumentTextPositionTables missing TCntV.01 count table".into());
            };
            write_stdout_line(&format!(
                "header\t{}\t{}\t{}\t{}\t{}",
                header.kind(),
                header.reserved(),
                header.declared_count(),
                header.entries_offset(),
                table.text_count_entries().len()
            ))?;
            for entry in table.text_count_entries() {
                write_stdout_line(&format!(
                    "entry\t{}\t{}\t{}\t{}",
                    entry.index(),
                    entry.start_offset(),
                    entry.end_offset(),
                    bytes_to_hex(entry.raw())
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-context") => {
            let path = required_path(args.next(), "text-position-count-context")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());

            for entry in table.text_count_entries() {
                let start = entry.start_offset() as usize;
                let end = entry.end_offset() as usize;
                write_stdout_line(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    entry.index(),
                    entry.start_offset(),
                    entry.end_offset(),
                    format_byte_context(map.entries(), start),
                    format_byte_context(map.entries(), end),
                    format_unit_context(map.entries(), start),
                    format_unit_context(map.entries(), end)
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-tail-context") => {
            let path = required_path(args.next(), "text-position-count-tail-context")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                let t1 = tail_fields.get(1).copied();
                let t2 = tail_fields.get(2).copied();
                write_stdout_line(&format!(
                    "tail-context\t{}\t{}\t{}\t{}\tt1={}\tt2={}\ttspan={}\tt1-byte={}\tt2-byte={}\tt1-unit={}\tt2-unit={}",
                    entry.index(),
                    family,
                    chosen_start,
                    chosen_end,
                    format_optional_u16_decimal(t1),
                    format_optional_u16_decimal(t2),
                    format_optional_i64(optional_tail_span(t1, t2)),
                    format_optional_byte_context(map.entries(), t1),
                    format_optional_byte_context(map.entries(), t2),
                    format_optional_unit_context(map.entries(), t1),
                    format_optional_unit_context(map.entries(), t2)
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-clusters") => {
            let path = required_path(args.next(), "text-position-count-clusters")?;
            let bytes = read_file(path)?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }

            let mut clusters = BTreeMap::new();
            for entry in table.text_count_entries() {
                clusters
                    .entry((entry.start_offset(), entry.end_offset()))
                    .or_insert_with(Vec::new)
                    .push(entry);
            }

            for ((start, end), entries) in clusters {
                let indexes = entries
                    .iter()
                    .map(|entry| entry.index().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                let tail_variants = entries
                    .iter()
                    .map(|entry| bytes_to_hex(&entry.raw()[8..]))
                    .collect::<BTreeSet<_>>();
                let tail_variant_count = tail_variants.len();
                let tail_variants = tail_variants.into_iter().collect::<Vec<_>>().join(",");
                write_stdout_line(&format!(
                    "{start}\t{end}\t{}\t{indexes}\t{tail_variant_count}\t{tail_variants}",
                    entries.len(),
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-candidates") => {
            let path = required_path(args.next(), "text-position-count-candidates")?;
            let bytes = read_file(path)?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                write_stdout_line(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    entry.index(),
                    read_be32_candidate(raw, 0),
                    read_be32_candidate(raw, 4),
                    read_be32_candidate(raw, 1),
                    read_be32_candidate(raw, 5),
                    bytes_to_hex(raw)
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-family") => {
            let path = required_path(args.next(), "text-position-count-family")?;
            let bytes = read_file(path)?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let be0_start = read_be32_candidate(raw, 0);
                let be0_end = read_be32_candidate(raw, 4);
                let be1_start = read_be32_candidate(raw, 1);
                let be1_end = read_be32_candidate(raw, 5);
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                write_stdout_line(&format!(
                    "family\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\tlead=0x{:02x}\ttail={}",
                    entry.index(),
                    family,
                    chosen_start,
                    chosen_end,
                    be0_start,
                    be0_end,
                    be1_start,
                    be1_end,
                    raw[0],
                    bytes_to_hex(&raw[tail_offset..])
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-fields") => {
            let path = required_path(args.next(), "text-position-count-fields")?;
            let bytes = read_file(path)?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail = &raw[tail_offset..];
                write_stdout_line(&format!(
                    "fields\t{}\t{}\t{}\t{}\t{}\tlead=0x{:02x}\ttail-offset={}\ttail-be16={}\ttail-extra={}\traw={}",
                    entry.index(),
                    family,
                    chosen_start,
                    chosen_end,
                    chosen_end.saturating_sub(chosen_start),
                    raw[0],
                    tail_offset,
                    format_be16_hex_fields(tail),
                    format_tail_extra_byte(tail),
                    bytes_to_hex(raw)
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-field-deltas") => {
            let path = required_path(args.next(), "text-position-count-field-deltas")?;
            let bytes = read_file(path)?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let chosen_span = chosen_end.saturating_sub(chosen_start);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                let t1 = tail_fields.get(1).copied();
                let t2 = tail_fields.get(2).copied();
                let tail_span = optional_tail_span(t1, t2);
                write_stdout_line(&format!(
                    "delta\t{}\t{}\t{}\t{}\t{}\ttail-offset={}\tt1={}\tt2={}\ttspan={}\tspan-relation={}\tstart-minus-t1={}\tend-minus-t2={}\tt0={}\tt3={}\tt4={}\tt7={}\traw={}",
                    entry.index(),
                    family,
                    chosen_start,
                    chosen_end,
                    chosen_span,
                    tail_offset,
                    format_optional_u16_decimal(t1),
                    format_optional_u16_decimal(t2),
                    format_optional_i64(tail_span),
                    format_span_relation(chosen_span, tail_span),
                    format_optional_i64(t1.map(|value| chosen_start as i64 - value as i64)),
                    format_optional_i64(t2.map(|value| chosen_end as i64 - value as i64)),
                    format_optional_u16_hex(tail_fields.first().copied()),
                    format_optional_u16_hex(tail_fields.get(3).copied()),
                    format_optional_u16_hex(tail_fields.get(4).copied()),
                    format_optional_u16_hex(tail_fields.get(7).copied()),
                    bytes_to_hex(raw)
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-tail-delta-scan") => {
            let path = required_path(args.next(), "text-position-count-tail-delta-scan")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());

            for delta in 0..=64usize {
                let mut endpoints = 0usize;
                let mut unit_hits = 0usize;
                let mut text_hits = 0usize;
                let mut both_unit_rows = 0usize;
                let mut both_text_rows = 0usize;

                for entry in table.text_count_entries() {
                    let raw = entry.raw();
                    let family = classify_text_count_entry_family(raw);
                    let tail_offset = text_count_entry_tail_offset(family);
                    let tail_fields = read_be16_fields(&raw[tail_offset..]);
                    let t1 = tail_fields.get(1).copied();
                    let t2 = tail_fields.get(2).copied();
                    let t1_unit_hit = count_tail_delta_hit(map.entries(), t1, delta, false);
                    let t2_unit_hit = count_tail_delta_hit(map.entries(), t2, delta, false);
                    let t1_text_hit = count_tail_delta_hit(map.entries(), t1, delta, true);
                    let t2_text_hit = count_tail_delta_hit(map.entries(), t2, delta, true);

                    endpoints += usize::from(t1.is_some()) + usize::from(t2.is_some());
                    unit_hits += usize::from(t1_unit_hit) + usize::from(t2_unit_hit);
                    text_hits += usize::from(t1_text_hit) + usize::from(t2_text_hit);
                    if t1_unit_hit && t2_unit_hit {
                        both_unit_rows += 1;
                    }
                    if t1_text_hit && t2_text_hit {
                        both_text_rows += 1;
                    }
                }

                write_stdout_line(&format!(
                    "delta\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    delta,
                    table.text_count_entries().len(),
                    endpoints,
                    unit_hits,
                    text_hits,
                    both_unit_rows,
                    both_text_rows
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-tail-delta-groups") => {
            let path = required_path(args.next(), "text-position-count-tail-delta-groups")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());
            let mut groups: TailDeltaGroups = BTreeMap::new();

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                let key = (
                    family,
                    tail_fields.first().copied(),
                    tail_fields.get(3).copied(),
                    tail_fields.get(4).copied(),
                    tail_fields.get(7).copied(),
                );
                groups
                    .entry(key)
                    .or_default()
                    .push((tail_fields.get(1).copied(), tail_fields.get(2).copied()));
            }

            for ((family, t0, t3, t4, t7), rows) in groups {
                let endpoints = rows
                    .iter()
                    .map(|(t1, t2)| usize::from(t1.is_some()) + usize::from(t2.is_some()))
                    .sum::<usize>();
                let best = best_tail_deltas(map.entries(), &rows);

                let delta0 = score_tail_delta_group(map.entries(), &rows, 0);
                let delta29 = score_tail_delta_group(map.entries(), &rows, 29);
                let delta30 = score_tail_delta_group(map.entries(), &rows, 30);
                write_stdout_line(&format!(
                    "group\t{}\tt0={}\tt3={}\tt4={}\tt7={}\trows={}\tendpoints={}\tbest-unit={}\tbest-text={}\td0={}\td29={}\td30={}",
                    family,
                    format_optional_u16_hex(t0),
                    format_optional_u16_hex(t3),
                    format_optional_u16_hex(t4),
                    format_optional_u16_hex(t7),
                    rows.len(),
                    endpoints,
                    format_best_unit_delta(best.unit_delta, best.unit_score),
                    format_best_text_delta(best.text_delta, best.text_score),
                    format_tail_delta_score(delta0),
                    format_tail_delta_score(delta29),
                    format_tail_delta_score(delta30)
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-tail-row-deltas") => {
            let path = required_path(args.next(), "text-position-count-tail-row-deltas")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());
            let document_units = map
                .entries()
                .last()
                .map(|entry| entry.unit_end())
                .unwrap_or_default();
            write_stdout_line(&format!(
                "summary\tentries={}\tdoc-bytes={}\tdoc-units={}",
                table.text_count_entries().len(),
                payload.bytes().len(),
                document_units
            ))?;

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                let t1 = tail_fields.get(1).copied();
                let t2 = tail_fields.get(2).copied();
                let rows = [(t1, t2)];
                let best = best_tail_deltas(map.entries(), &rows);

                let delta0 = score_tail_delta_group(map.entries(), &rows, 0);
                let delta29 = score_tail_delta_group(map.entries(), &rows, 29);
                let delta30 = score_tail_delta_group(map.entries(), &rows, 30);
                write_stdout_line(&format!(
                    "row\t{}\t{}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tspan={}\tt1={}\tt2={}\ttspan={}\tbest-unit={}\tbest-text={}\td0={}\td29={}\td30={}",
                    entry.index(),
                    family,
                    format_optional_u16_hex(tail_fields.first().copied()),
                    format_optional_u16_hex(tail_fields.get(3).copied()),
                    format_optional_u16_hex(tail_fields.get(4).copied()),
                    format_optional_u16_hex(tail_fields.get(7).copied()),
                    chosen_start,
                    chosen_end,
                    chosen_end.saturating_sub(chosen_start),
                    format_optional_u16_decimal(t1),
                    format_optional_u16_decimal(t2),
                    format_optional_i64(optional_tail_span(t1, t2)),
                    format_best_unit_delta(best.unit_delta, best.unit_score),
                    format_best_text_delta(best.text_delta, best.text_score),
                    format_tail_delta_score(delta0),
                    format_tail_delta_score(delta29),
                    format_tail_delta_score(delta30)
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-tail-row-context") => {
            let path = required_path(args.next(), "text-position-count-tail-row-context")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                let t1 = tail_fields.get(1).copied();
                let t2 = tail_fields.get(2).copied();
                let rows = [(t1, t2)];
                let best = best_tail_deltas(map.entries(), &rows);
                write_stdout_line(&format!(
                    "row-context\t{}\t{}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tt1={}\tt2={}\tbest-unit={}\tbest-text={}\tstart-byte={}\tend-byte={}\tstart-unit={}\tend-unit={}\tt1-unit-best={}\tt2-unit-best={}\tt1-text-best={}\tt2-text-best={}",
                    entry.index(),
                    family,
                    format_optional_u16_hex(tail_fields.first().copied()),
                    format_optional_u16_hex(tail_fields.get(3).copied()),
                    format_optional_u16_hex(tail_fields.get(4).copied()),
                    format_optional_u16_hex(tail_fields.get(7).copied()),
                    chosen_start,
                    chosen_end,
                    format_optional_u16_decimal(t1),
                    format_optional_u16_decimal(t2),
                    format_best_unit_delta(best.unit_delta, best.unit_score),
                    format_best_text_delta(best.text_delta, best.text_score),
                    format_byte_context(map.entries(), chosen_start as usize),
                    format_byte_context(map.entries(), chosen_end as usize),
                    format_unit_context(map.entries(), chosen_start as usize),
                    format_unit_context(map.entries(), chosen_end as usize),
                    format_optional_unit_context_with_delta(map.entries(), t1, best.unit_delta),
                    format_optional_unit_context_with_delta(map.entries(), t2, best.unit_delta),
                    format_optional_unit_context_with_delta(map.entries(), t1, best.text_delta),
                    format_optional_unit_context_with_delta(map.entries(), t2, best.text_delta)
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-tail-field-roles") => {
            let path = required_path(args.next(), "text-position-count-tail-field-roles")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document_units = map
                .entries()
                .last()
                .map(|entry| entry.unit_end())
                .unwrap_or_default();
            let (position_status, table) = match read_document_text_position_tables(&bytes) {
                Ok(table) => ("ok".to_string(), Some(table)),
                Err(Error::NotFound(_)) => ("missing".to_string(), None),
                Err(Error::InvalidData(message)) => {
                    (format!("invalid:{}", escaped_text(&message)), None)
                }
                Err(error) => return Err(error.to_string()),
            };
            let text_count_entries = table
                .as_ref()
                .map(|table| table.text_count_entries())
                .unwrap_or(&[]);
            let field_summaries =
                summarize_tail_field_roles(text_count_entries, map.entries(), &[0, 29, 30]);
            let pair_summaries =
                summarize_tail_field_pair_roles(text_count_entries, map.entries(), &[0, 29, 30]);

            write_stdout_line(&format!(
                "summary\tposition-status={}\tentries={}\tdoc-bytes={}\tdoc-units={}",
                position_status,
                text_count_entries.len(),
                payload.bytes().len(),
                document_units
            ))?;

            for (field_index, field) in field_summaries.iter().enumerate() {
                write_stdout_line(&format!(
                    "field\tf{}\tnonzero={}\tdistinct={}\tvalues={}\tunit-d0={}\ttext-d0={}\tunit-d29={}\ttext-d29={}\tunit-d30={}\ttext-d30={}",
                    field_index,
                    field.nonzero_count,
                    field.distinct_values.len(),
                    format_u16_value_counts(&field.value_counts),
                    field.delta_hit_count(0, false),
                    field.delta_hit_count(0, true),
                    field.delta_hit_count(29, false),
                    field.delta_hit_count(29, true),
                    field.delta_hit_count(30, false),
                    field.delta_hit_count(30, true)
                ))?;
            }

            for (field_index, pair) in pair_summaries.iter().enumerate() {
                write_stdout_line(&format!(
                    "pair\tf{}-f{}\tpairs={}\tendpoints={}\tspan-eq={}\tspan-lt={}\tspan-gt={}\tbest-unit={}\tbest-text={}\td0={}\td29={}\td30={}",
                    field_index,
                    field_index + 1,
                    pair.pair_count,
                    pair.endpoints,
                    pair.span_eq_count,
                    pair.span_lt_count,
                    pair.span_gt_count,
                    format_best_unit_delta(pair.best.unit_delta, pair.best.unit_score),
                    format_best_text_delta(pair.best.text_delta, pair.best.text_score),
                    format_tail_delta_score(pair.delta_score(0)),
                    format_tail_delta_score(pair.delta_score(29)),
                    format_tail_delta_score(pair.delta_score(30))
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-range-preview") => {
            let path = required_path(args.next(), "text-position-count-range-preview")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                write_stdout_line(&format!(
                    "range-preview\t{}\t{}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tspan={}\tbyte-range={}\tunit-range={}",
                    entry.index(),
                    family,
                    format_optional_u16_hex(tail_fields.first().copied()),
                    format_optional_u16_hex(tail_fields.get(3).copied()),
                    format_optional_u16_hex(tail_fields.get(4).copied()),
                    format_optional_u16_hex(tail_fields.get(7).copied()),
                    chosen_start,
                    chosen_end,
                    chosen_end.saturating_sub(chosen_start),
                    format_byte_range_preview(
                        map.entries(),
                        chosen_start as usize,
                        chosen_end as usize
                    ),
                    format_unit_range_preview(
                        map.entries(),
                        chosen_start as usize,
                        chosen_end as usize
                    )
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-range-boundaries") => {
            let path = required_path(args.next(), "text-position-count-range-boundaries")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                write_stdout_line(&format!(
                    "range-boundary\t{}\t{}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tspan={}\tbyte-boundary={}\tunit-boundary={}",
                    entry.index(),
                    family,
                    format_optional_u16_hex(tail_fields.first().copied()),
                    format_optional_u16_hex(tail_fields.get(3).copied()),
                    format_optional_u16_hex(tail_fields.get(4).copied()),
                    format_optional_u16_hex(tail_fields.get(7).copied()),
                    chosen_start,
                    chosen_end,
                    chosen_end.saturating_sub(chosen_start),
                    format_byte_range_boundaries(
                        map.entries(),
                        chosen_start as usize,
                        chosen_end as usize
                    ),
                    format_unit_range_boundaries(
                        map.entries(),
                        chosen_start as usize,
                        chosen_end as usize
                    )
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-control-ranges") => {
            let path = required_path(args.next(), "text-position-count-control-ranges")?;
            let filter = args
                .next()
                .map(|value| parse_u16_argument(&value))
                .transpose()?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }
            let map = map_document_text(payload.bytes());
            let ranges = build_control_delimited_ranges(map.entries(), filter);

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                write_stdout_line(&format!(
                    "count-control-range\t{}\t{}\tdelimiter={}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tspan={}\tbyte-ranges={}\tunit-ranges={}",
                    entry.index(),
                    family,
                    format_control_range_delimiter(filter),
                    format_optional_u16_hex(tail_fields.first().copied()),
                    format_optional_u16_hex(tail_fields.get(3).copied()),
                    format_optional_u16_hex(tail_fields.get(4).copied()),
                    format_optional_u16_hex(tail_fields.get(7).copied()),
                    chosen_start,
                    chosen_end,
                    chosen_end.saturating_sub(chosen_start),
                    format_control_range_hits(
                        map.entries(),
                        &ranges,
                        chosen_start as usize,
                        chosen_end as usize,
                        RangeBasis::Byte,
                    ),
                    format_control_range_hits(
                        map.entries(),
                        &ranges,
                        chosen_start as usize,
                        chosen_end as usize,
                        RangeBasis::Unit,
                    )
                ))?;
            }
            Ok(())
        }
        Some("text-boundary-candidates") => {
            let path = required_path(args.next(), "text-boundary-candidates")?;
            let bytes = read_file(path)?;
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;

            for candidate in document.text_boundary_candidates() {
                write_stdout_line(&format!(
                    "text-boundary-candidate\t{}\tkind={}\trange={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tinterval-kind={}\tfirst={}\tlast={}\tsource={}-{}\tdecoded=false",
                    candidate.index(),
                    candidate.kind(),
                    candidate.text_count_range_index(),
                    candidate.basis().as_str(),
                    candidate.delimiter_code(),
                    candidate.interval_count(),
                    format_boundary_candidate_interval_kind(candidate.interval_count()),
                    candidate.first_interval_index(),
                    candidate.last_interval_index(),
                    candidate.source_start(),
                    candidate.source_end()
                ))?;
            }
            Ok(())
        }
        Some("table-candidates") => {
            let path = required_path(args.next(), "table-candidates")?;
            let bytes = read_file(path)?;
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;

            for candidate in document.table_candidates() {
                write_stdout_line(&format!(
                    "table-candidate\t{}\tkind={}\trange={}\tboundary={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tfirst={}\tlast={}\tsource={}-{}\tsparse={}\tcells={}/{}/{}\tmax-columns={}\tinterval-details={}\tdecoded=false",
                    candidate.index(),
                    candidate.kind(),
                    format_model_table_source_index(candidate.text_count_range_index()),
                    format_model_table_source_index(candidate.text_boundary_candidate_index()),
                    candidate.basis().as_str(),
                    candidate.delimiter_code(),
                    candidate.interval_count(),
                    candidate.first_interval_index(),
                    candidate.last_interval_index(),
                    candidate.source_start(),
                    candidate.source_end(),
                    candidate.is_sparse_document_text_control_run_candidate(),
                    candidate.non_empty_cell_count_candidate(),
                    candidate.empty_cell_count_candidate(),
                    candidate.cell_count_candidate(),
                    candidate.max_column_segment_count(),
                    format_table_candidate_intervals(candidate)
                ))?;
            }
            Ok(())
        }
        Some("table-candidate-context") => {
            let path = required_path(args.next(), "table-candidate-context")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;

            for candidate in document.table_candidates() {
                let basis = range_basis_from_candidate(candidate.basis().as_str());
                write_stdout_line(&format!(
                    "table-candidate-context\t{}\trange={}\tboundary={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tsource={}-{}\tshape={}\tinterval-contexts={}\tdecoded=false",
                    candidate.index(),
                    format_model_table_source_index(candidate.text_count_range_index()),
                    format_model_table_source_index(candidate.text_boundary_candidate_index()),
                    candidate.basis().as_str(),
                    candidate.delimiter_code(),
                    candidate.interval_count(),
                    candidate.source_start(),
                    candidate.source_end(),
                    format_table_candidate_text_shape(candidate, map.entries(), basis),
                    format_table_candidate_interval_contexts(candidate, map.entries(), basis)
                ))?;
            }
            Ok(())
        }
        Some("table-cell-like-candidates") => {
            let path = required_path(args.next(), "table-cell-like-candidates")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;

            for candidate in document.table_candidates() {
                let basis = range_basis_from_candidate(candidate.basis().as_str());
                if !is_table_candidate_cell_like(candidate, map.entries(), basis) {
                    continue;
                }
                write_stdout_line(&format!(
                    "table-cell-like-candidate\t{}\trange={}\tboundary={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tsource={}-{}\tshape={}\ttexts={}\tcolumn-split-candidate-rows={}\tmax-column-segment-count={}\tcolumn-segment-pattern-consistent={}\tcolumn-segment-pattern-mismatch-rows={}\tcolumn-grid-candidate={}\tcolumn-grid-shape={}\tcolumn-grid-pattern={}\tinterval-column-segments={}\tdecoded=false",
                    candidate.index(),
                    format_model_table_source_index(candidate.text_count_range_index()),
                    format_model_table_source_index(candidate.text_boundary_candidate_index()),
                    candidate.basis().as_str(),
                    candidate.delimiter_code(),
                    candidate.interval_count(),
                    candidate.source_start(),
                    candidate.source_end(),
                    format_table_candidate_text_shape(candidate, map.entries(), basis),
                    format_table_candidate_interval_texts(candidate, map.entries(), basis),
                    candidate.column_split_candidate_row_count(),
                    candidate.max_column_segment_count(),
                    candidate.column_segment_pattern_consistent(),
                    candidate.column_segment_pattern_mismatch_rows(),
                    if candidate.column_segment_grid_candidate().is_some() {
                        "true"
                    } else {
                        "false"
                    },
                    format_table_candidate_column_grid_shape(candidate),
                    format_table_candidate_column_grid_pattern(candidate),
                    format_table_candidate_interval_column_segments(candidate)
                ))?;
            }
            Ok(())
        }
        Some("text-boundary-candidate-context") => {
            let path = required_path(args.next(), "text-boundary-candidate-context")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;

            for candidate in document.text_boundary_candidates() {
                let basis = range_basis_from_candidate(candidate.basis().as_str());
                write_stdout_line(&format!(
                    "text-boundary-candidate-context\t{}\trange={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tinterval-kind={}\tsource={}-{}\tline-breaks={}\ttext={}\tedges={}\tdecoded=false",
                    candidate.index(),
                    candidate.text_count_range_index(),
                    candidate.basis().as_str(),
                    candidate.delimiter_code(),
                    candidate.interval_count(),
                    format_boundary_candidate_interval_kind(candidate.interval_count()),
                    candidate.source_start(),
                    candidate.source_end(),
                    range_line_break_count(
                        map.entries(),
                        candidate.source_start(),
                        candidate.source_end(),
                        basis
                    ),
                    format_candidate_range_preview(
                        map.entries(),
                        candidate.source_start(),
                        candidate.source_end(),
                        basis
                    ),
                    format_candidate_range_boundaries(
                        map.entries(),
                        candidate.source_start(),
                        candidate.source_end(),
                        basis
                    )
                ))?;
            }
            Ok(())
        }
        Some("text-boundary-candidate-agreement") => {
            let path = required_path(args.next(), "text-boundary-candidate-agreement")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let mut pair_index = 0usize;

            for byte_candidate in document
                .text_boundary_candidates()
                .iter()
                .filter(|candidate| candidate.basis().as_str() == "byte")
            {
                let Some(unit_candidate) =
                    document
                        .text_boundary_candidates()
                        .iter()
                        .find(|candidate| {
                            candidate.basis().as_str() == "unit"
                                && candidate.text_count_range_index()
                                    == byte_candidate.text_count_range_index()
                                && candidate.delimiter_code() == byte_candidate.delimiter_code()
                        })
                else {
                    continue;
                };
                let byte_text = range_visible_text(
                    map.entries(),
                    byte_candidate.source_start(),
                    byte_candidate.source_end(),
                    RangeBasis::Byte,
                );
                let unit_text = range_visible_text(
                    map.entries(),
                    unit_candidate.source_start(),
                    unit_candidate.source_end(),
                    RangeBasis::Unit,
                );
                let byte_line_breaks = text_line_break_count(&byte_text);
                let unit_line_breaks = text_line_break_count(&unit_text);

                write_stdout_line(&format!(
                    "text-boundary-candidate-agreement\t{}\trange={}\tdelimiter=0x{:04x}\tbyte-index={}\tunit-index={}\tbyte-intervals={}\tunit-intervals={}\tbyte-interval-kind={}\tunit-interval-kind={}\tbyte-edge-good={}\tunit-edge-good={}\tbyte-line-breaks={}\tunit-line-breaks={}\ttext-match={}\tline-break-match={}\tbyte-text={}\tunit-text={}\tdecoded=false",
                    pair_index,
                    byte_candidate.text_count_range_index(),
                    byte_candidate.delimiter_code(),
                    byte_candidate.index(),
                    unit_candidate.index(),
                    byte_candidate.interval_count(),
                    unit_candidate.interval_count(),
                    format_boundary_candidate_interval_kind(byte_candidate.interval_count()),
                    format_boundary_candidate_interval_kind(unit_candidate.interval_count()),
                    is_boundary_candidate_edge_good(
                        map.entries(),
                        byte_candidate.source_start(),
                        byte_candidate.source_end(),
                        RangeBasis::Byte
                    ),
                    is_boundary_candidate_edge_good(
                        map.entries(),
                        unit_candidate.source_start(),
                        unit_candidate.source_end(),
                        RangeBasis::Unit
                    ),
                    byte_line_breaks,
                    unit_line_breaks,
                    byte_text == unit_text,
                    byte_line_breaks == unit_line_breaks,
                    escaped_text_preview(&byte_text, 80),
                    escaped_text_preview(&unit_text, 80)
                ))?;
                pair_index += 1;
            }
            Ok(())
        }
        Some("text-boundary-candidate-layout-context") => {
            let path = required_path(args.next(), "text-boundary-candidate-layout-context")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
            let line_words = line_stream
                .as_deref()
                .map(|stream| be16_words(stream).collect::<Vec<_>>());
            let page_mark = read_page_mark(&bytes).ok();
            let page_bytes = read_cfb_stream(&bytes, "/PageMark")
                .ok()
                .map(|stream| stream.len());
            let paper_mark = read_paper_mark(&bytes).ok();
            let paper_bytes = read_cfb_stream(&bytes, "/PaperMark")
                .ok()
                .map(|stream| stream.len());

            let candidates = document
                .text_boundary_candidates()
                .iter()
                .filter(|candidate| {
                    candidate.basis().as_str() == "unit"
                        && candidate.delimiter_code() == 0x001c
                        && candidate.interval_count() == 1
                })
                .collect::<Vec<_>>();
            let selected_count = candidates
                .iter()
                .filter(|candidate| {
                    is_strict_unit_paragraph_candidate(
                        map.entries(),
                        candidate.source_start(),
                        candidate.source_end(),
                    )
                })
                .count();

            write_stdout_line(&format!(
                "summary\tunit-001c-single-candidates={}\trule-selected={}\tline-bytes={}\tline-words={}\tpage-rows={}\tpage-bytes={}\tpaper-rows={}\tpaper-bytes={}",
                candidates.len(),
                selected_count,
                format_optional_usize(line_stream.as_ref().map(|stream| stream.len())),
                format_optional_usize(line_words.as_ref().map(Vec::len)),
                format_optional_usize(page_mark.as_ref().map(|mark| mark.entries().len())),
                format_optional_usize(page_bytes),
                format_optional_usize(paper_mark.as_ref().map(|mark| mark.entries().len())),
                format_optional_usize(paper_bytes),
            ))?;

            for candidate in candidates {
                let text = range_visible_text(
                    map.entries(),
                    candidate.source_start(),
                    candidate.source_end(),
                    RangeBasis::Unit,
                );
                let line_breaks = text_line_break_count(&text);
                let edge_good = is_boundary_candidate_edge_good(
                    map.entries(),
                    candidate.source_start(),
                    candidate.source_end(),
                    RangeBasis::Unit,
                );
                let non_empty = !text.is_empty();
                let selected = edge_good && non_empty && line_breaks <= 1;
                write_stdout_line(&format!(
                    "candidate\t{}\trange={}\tselected={}\tedge-good={}\tnon-empty={}\tline-breaks={}\tsource={}-{}\ttext={}\tline-word-start={}\tline-word-end={}\tline-byte-start={}\tline-byte-end={}\tpage-row-start={}\tpage-row-end={}\tpage-byte-start={}\tpage-byte-end={}\tpaper-row-start={}\tpaper-row-end={}\tpaper-byte-start={}\tpaper-byte-end={}\tdecoded=false",
                    candidate.index(),
                    candidate.text_count_range_index(),
                    selected,
                    edge_good,
                    non_empty,
                    line_breaks,
                    candidate.source_start(),
                    candidate.source_end(),
                    escaped_text_preview(&text, 80),
                    format_line_word_index_context(line_words.as_deref(), candidate.source_start()),
                    format_line_word_index_context(line_words.as_deref(), candidate.source_end()),
                    format_line_byte_offset_context(
                        line_words.as_deref(),
                        line_stream.as_ref().map(|stream| stream.len()),
                        candidate.source_start()
                    ),
                    format_line_byte_offset_context(
                        line_words.as_deref(),
                        line_stream.as_ref().map(|stream| stream.len()),
                        candidate.source_end()
                    ),
                    format_index_context(
                        page_mark.as_ref().map(|mark| mark.entries().len()),
                        candidate.source_start()
                    ),
                    format_index_context(
                        page_mark.as_ref().map(|mark| mark.entries().len()),
                        candidate.source_end()
                    ),
                    format_index_context(page_bytes, candidate.source_start()),
                    format_index_context(page_bytes, candidate.source_end()),
                    format_index_context(
                        paper_mark.as_ref().map(|mark| mark.entries().len()),
                        candidate.source_start()
                    ),
                    format_index_context(
                        paper_mark.as_ref().map(|mark| mark.entries().len()),
                        candidate.source_end()
                    ),
                    format_index_context(paper_bytes, candidate.source_start()),
                    format_index_context(paper_bytes, candidate.source_end()),
                ))?;
            }
            Ok(())
        }
        Some("text-boundary-layout-map") => {
            let path = required_path(args.next(), "text-boundary-layout-map")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
            let line_words = line_stream
                .as_deref()
                .map(|stream| be16_words(stream).collect::<Vec<_>>());
            let page_mark = read_page_mark(&bytes).ok();
            let paper_mark = read_paper_mark(&bytes).ok();

            let candidates = collect_unit_001c_single_layout_candidates(
                map.entries(),
                document.text_boundary_candidates(),
            );
            let selected_count = candidates
                .iter()
                .filter(|candidate| candidate.selected)
                .count();
            let target_sets = layout_map_target_sets(
                line_words.as_deref(),
                page_mark.as_ref(),
                paper_mark.as_ref(),
            );
            let target_set_count = target_sets.len();
            let base_count = layout_map_bases().len();

            write_stdout_line(&format!(
                "summary\tunit-001c-single-candidates={}\trule-selected={}\ttarget-sets={}\tbases={}\tdelta-range={}..{}",
                candidates.len(),
                selected_count,
                target_set_count,
                base_count,
                LAYOUT_MAP_DELTA_MIN,
                LAYOUT_MAP_DELTA_MAX
            ))?;

            write_layout_map_best_rows("all", &candidates, &target_sets)?;
            let selected = candidates
                .iter()
                .copied()
                .filter(|candidate| candidate.selected)
                .collect::<Vec<_>>();
            write_layout_map_best_rows("selected", &selected, &target_sets)?;
            Ok(())
        }
        Some("text-boundary-layout-map-rows") => {
            let path = required_path(args.next(), "text-boundary-layout-map-rows")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
            let line_words = line_stream
                .as_deref()
                .map(|stream| be16_words(stream).collect::<Vec<_>>());
            let page_mark = read_page_mark(&bytes).ok();
            let paper_mark = read_paper_mark(&bytes).ok();

            let candidates = collect_unit_001c_single_layout_candidates(
                map.entries(),
                document.text_boundary_candidates(),
            );
            let selected_count = candidates
                .iter()
                .filter(|candidate| candidate.selected)
                .count();
            let target_sets = layout_map_target_sets(
                line_words.as_deref(),
                page_mark.as_ref(),
                paper_mark.as_ref(),
            );
            let base_count = layout_map_bases().len();
            write_stdout_line(&format!(
                "summary\tunit-001c-single-candidates={}\trule-selected={}\ttarget-sets={}\tbases={}\tlocal-rows={}",
                candidates.len(),
                selected_count,
                target_sets.len(),
                base_count,
                candidates.len() * target_sets.len() * base_count
            ))?;

            for candidate in &candidates {
                let text = range_visible_text(
                    map.entries(),
                    candidate.source_start,
                    candidate.source_end,
                    RangeBasis::Unit,
                );
                let range = document
                    .text_count_ranges()
                    .get(candidate.text_count_range_index);
                for target_set in &target_sets {
                    for base in layout_map_bases() {
                        let single = [*candidate];
                        let (delta, score) = best_layout_map_delta(&single, target_set, *base);
                        write_stdout_line(&format!(
                            "local\tcandidate={}\trange={}\tselected={}\ttarget={}\tbase={}\tdelta={}\tdelta-at-boundary={}\texact={}\ttotal-distance={}\tmax-distance={}\tstart-nearest={}\tend-nearest={}\tsource={}-{}\ttext={}\ttcnt={}\tdecoded=false",
                            candidate.index,
                            candidate.text_count_range_index,
                            candidate.selected,
                            target_set.name,
                            base.name(),
                            delta,
                            delta == LAYOUT_MAP_DELTA_MIN || delta == LAYOUT_MAP_DELTA_MAX,
                            score.exact_hits,
                            format_optional_usize(score.total_distance),
                            format_optional_usize(score.max_distance),
                            format_layout_map_endpoint(
                                candidate.source_start,
                                target_set,
                                *base,
                                delta
                            ),
                            format_layout_map_endpoint(
                                candidate.source_end,
                                target_set,
                                *base,
                                delta
                            ),
                            candidate.source_start,
                            candidate.source_end,
                            escaped_text_preview(&text, 80),
                            format_text_count_range_summary(range),
                        ))?;
                    }
                }
            }
            Ok(())
        }
        Some("text-boundary-paragraph-like") => {
            let path = required_path(args.next(), "text-boundary-paragraph-like")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
            let line_words = line_stream
                .as_deref()
                .map(|stream| be16_words(stream).collect::<Vec<_>>());
            let page_mark = read_page_mark(&bytes).ok();
            let paper_mark = read_paper_mark(&bytes).ok();
            let target_sets = layout_map_target_sets(
                line_words.as_deref(),
                page_mark.as_ref(),
                paper_mark.as_ref(),
            );
            let candidates = collect_unit_001c_single_layout_candidates(
                map.entries(),
                document.text_boundary_candidates(),
            );

            let mut rows = Vec::new();
            for candidate in &candidates {
                let evidence = layout_paragraph_like_evidence(candidate, &target_sets);
                rows.push((
                    *candidate,
                    evidence.paragraph_like,
                    evidence.line_word_evidence,
                    evidence.page_field_evidence,
                ));
            }
            let strict_selected = rows
                .iter()
                .filter(|(candidate, _, _, _)| candidate.selected)
                .count();
            let paragraph_like_count = rows
                .iter()
                .filter(|(_, paragraph_like, _, _)| *paragraph_like)
                .count();
            let selected_non_paragraph_like = strict_selected.saturating_sub(paragraph_like_count);
            write_stdout_line(&format!(
                "summary\tunit-001c-single-candidates={}\tstrict-selected={}\tparagraph-like={}\tselected-non-paragraph-like={}\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false",
                candidates.len(),
                strict_selected,
                paragraph_like_count,
                selected_non_paragraph_like
            ))?;

            for (candidate, paragraph_like, line_word_evidence, page_field_evidence) in rows {
                let text = range_visible_text(
                    map.entries(),
                    candidate.source_start,
                    candidate.source_end,
                    RangeBasis::Unit,
                );
                let range = document
                    .text_count_ranges()
                    .get(candidate.text_count_range_index);
                write_stdout_line(&format!(
                    "candidate\t{}\trange={}\tstrict-selected={}\tparagraph-like={}\tline-word-evidence={}\tpage-field-evidence={}\tsource={}-{}\ttext={}\ttcnt={}\tdecoded=false",
                    candidate.index,
                    candidate.text_count_range_index,
                    candidate.selected,
                    paragraph_like,
                    format_layout_exact_evidence(line_word_evidence.as_ref()),
                    format_layout_exact_evidence(page_field_evidence.as_ref()),
                    candidate.source_start,
                    candidate.source_end,
                    escaped_text_preview(&text, 80),
                    format_text_count_range_summary(range),
                ))?;
            }
            Ok(())
        }
        Some("text-boundary-paragraph-like-style-context") => {
            let path = required_path(args.next(), "text-boundary-paragraph-like-style-context")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
            let line_words = line_stream
                .as_deref()
                .map(|stream| be16_words(stream).collect::<Vec<_>>());
            let page_mark = read_page_mark(&bytes).ok();
            let paper_mark = read_paper_mark(&bytes).ok();
            let target_sets = layout_map_target_sets(
                line_words.as_deref(),
                page_mark.as_ref(),
                paper_mark.as_ref(),
            );
            let style_streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
            let text_style_candidates =
                collect_labeled_style_candidates(&style_streams, TEXT_LAYOUT_STYLE_PATH);
            let page_style_candidates =
                collect_labeled_style_candidates(&style_streams, PAGE_LAYOUT_STYLE_PATH);
            let view_style_groups = collect_document_view_style_groups(&style_streams);
            let view_style_records = style_streams
                .iter()
                .find(|stream| stream.name() == DOCUMENT_VIEW_STYLES_PATH)
                .map(|stream| stream.summary().records().len())
                .unwrap_or_default();
            let candidates = collect_unit_001c_single_layout_candidates(
                map.entries(),
                document.text_boundary_candidates(),
            );
            let rows = candidates
                .iter()
                .map(|candidate| {
                    (
                        *candidate,
                        layout_paragraph_like_evidence(candidate, &target_sets),
                    )
                })
                .collect::<Vec<_>>();
            let strict_selected = rows
                .iter()
                .filter(|(candidate, _)| candidate.selected)
                .count();
            let paragraph_like_count = rows
                .iter()
                .filter(|(_, evidence)| evidence.paragraph_like)
                .count();
            let selected_non_paragraph_like = strict_selected.saturating_sub(paragraph_like_count);

            write_stdout_line(&format!(
                "summary\tunit-001c-single-candidates={}\tstrict-selected={}\tparagraph-like={}\tselected-non-paragraph-like={}\ttext-style-candidates={}\tpage-style-candidates={}\tview-style-records={}\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false",
                candidates.len(),
                strict_selected,
                paragraph_like_count,
                selected_non_paragraph_like,
                text_style_candidates.len(),
                page_style_candidates.len(),
                view_style_records
            ))?;

            for (candidate, evidence) in rows {
                let text = range_visible_text(
                    map.entries(),
                    candidate.source_start,
                    candidate.source_end,
                    RangeBasis::Unit,
                );
                let range = document
                    .text_count_ranges()
                    .get(candidate.text_count_range_index);
                let tail_fields = range.map(|range| range.tail_fields()).unwrap_or(&[]);
                let byte_range = range
                    .map(|range| {
                        format_byte_range_preview(
                            map.entries(),
                            range.start() as usize,
                            range.end() as usize,
                        )
                    })
                    .unwrap_or_else(|| "-".to_string());
                let unit_range = range
                    .map(|range| {
                        format_unit_range_preview(
                            map.entries(),
                            range.start() as usize,
                            range.end() as usize,
                        )
                    })
                    .unwrap_or_else(|| "-".to_string());
                write_stdout_line(&format!(
                    "candidate\t{}\trange={}\tstrict-selected={}\tparagraph-like={}\tline-word-evidence={}\tpage-field-evidence={}\ttail-fields={}\ttext-style-id-hits={}\ttext-style-index-hits={}\tpage-style-id-hits={}\tpage-style-index-hits={}\tview-style-group-hits={}\tbyte-range={}\tunit-range={}\tsource={}-{}\ttext={}\ttcnt={}\tdecoded=false",
                    candidate.index,
                    candidate.text_count_range_index,
                    candidate.selected,
                    evidence.paragraph_like,
                    format_layout_exact_evidence(evidence.line_word_evidence.as_ref()),
                    format_layout_exact_evidence(evidence.page_field_evidence.as_ref()),
                    format_indexed_u16_fields(tail_fields),
                    format_style_id_hits(tail_fields, &text_style_candidates),
                    format_style_index_hits(tail_fields, &text_style_candidates),
                    format_style_id_hits(tail_fields, &page_style_candidates),
                    format_style_index_hits(tail_fields, &page_style_candidates),
                    format_view_style_group_hits(tail_fields, &view_style_groups),
                    byte_range,
                    unit_range,
                    candidate.source_start,
                    candidate.source_end,
                    escaped_text_preview(&text, 80),
                    format_text_count_range_summary(range),
                ))?;
            }
            Ok(())
        }
        Some("text-boundary-paragraph-like-discriminators") => {
            let path = required_path(args.next(), "text-boundary-paragraph-like-discriminators")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
            let line_words = line_stream
                .as_deref()
                .map(|stream| be16_words(stream).collect::<Vec<_>>());
            let page_mark = read_page_mark(&bytes).ok();
            let paper_mark = read_paper_mark(&bytes).ok();
            let target_sets = layout_map_target_sets(
                line_words.as_deref(),
                page_mark.as_ref(),
                paper_mark.as_ref(),
            );
            let style_streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
            let text_style_candidates =
                collect_labeled_style_candidates(&style_streams, TEXT_LAYOUT_STYLE_PATH);
            let page_style_candidates =
                collect_labeled_style_candidates(&style_streams, PAGE_LAYOUT_STYLE_PATH);
            let view_style_groups = collect_document_view_style_groups(&style_streams);
            let candidates = collect_unit_001c_single_layout_candidates(
                map.entries(),
                document.text_boundary_candidates(),
            );

            let mut paragraph_like = ParagraphLikeBucketSummary::default();
            let mut strict_non_paragraph = ParagraphLikeBucketSummary::default();
            let mut non_strict = ParagraphLikeBucketSummary::default();
            for candidate in &candidates {
                let evidence = layout_paragraph_like_evidence(candidate, &target_sets);
                let range = document
                    .text_count_ranges()
                    .get(candidate.text_count_range_index);
                let bucket = if evidence.paragraph_like {
                    &mut paragraph_like
                } else if candidate.selected {
                    &mut strict_non_paragraph
                } else {
                    &mut non_strict
                };
                bucket.observe(
                    candidate,
                    &evidence,
                    range,
                    &text_style_candidates,
                    &page_style_candidates,
                    &view_style_groups,
                );
            }

            write_stdout_line(&format!(
                "summary\tunit-001c-single-candidates={}\tstrict-selected={}\tparagraph-like={}\tselected-non-paragraph-like={}\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false",
                candidates.len(),
                paragraph_like.strict_selected + strict_non_paragraph.strict_selected,
                paragraph_like.rows,
                strict_non_paragraph.rows
            ))?;
            write_stdout_line(&format!(
                "bucket\tparagraph-like\t{}",
                paragraph_like.format_fields()
            ))?;
            write_stdout_line(&format!(
                "bucket\tstrict-non-paragraph\t{}",
                strict_non_paragraph.format_fields()
            ))?;
            write_stdout_line(&format!(
                "bucket\tnon-strict\t{}",
                non_strict.format_fields()
            ))?;
            Ok(())
        }
        Some("text-paragraph-boundary-targets") => {
            let path = required_path(args.next(), "text-paragraph-boundary-targets")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;
            let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
            let line_words = line_stream
                .as_deref()
                .map(|stream| be16_words(stream).collect::<Vec<_>>());
            let page_mark = read_page_mark(&bytes).ok();

            write_stdout_line(&format!(
                "summary\ttext-paragraph-boundary-candidates={}\tline-words={}\tpage-rows={}\trule=strict-unit-001c-single+nonzero-tcnt-span+line-word-value-exact2+page-be32-field-exact2\tdecoded=false",
                document.text_paragraph_boundary_candidates().len(),
                format_optional_usize(line_words.as_ref().map(Vec::len)),
                format_optional_usize(page_mark.as_ref().map(|mark| mark.entries().len())),
            ))?;

            for candidate in document.text_paragraph_boundary_candidates() {
                let text = range_visible_text(
                    map.entries(),
                    candidate.source_start(),
                    candidate.source_end(),
                    RangeBasis::Unit,
                );
                let line_start =
                    layout_evidence_value(candidate.source_start(), candidate.line_word_evidence());
                let line_end =
                    layout_evidence_value(candidate.source_end(), candidate.line_word_evidence());
                let page_start = layout_evidence_value(
                    candidate.source_start(),
                    candidate.page_field_evidence(),
                );
                let page_end =
                    layout_evidence_value(candidate.source_end(), candidate.page_field_evidence());
                let range = document
                    .text_count_ranges()
                    .get(candidate.text_count_range_index());
                write_stdout_line(&format!(
                    "text-paragraph-boundary-target\t{}\tboundary={}\trange={}\tsource={}-{}\tspan={}\tline-word-evidence={}\tline-start={}\tline-end={}\tpage-field-evidence={}\tpage-start={}\tpage-end={}\ttext={}\ttcnt={}\tdecoded=false",
                    candidate.index(),
                    candidate.text_boundary_candidate_index(),
                    candidate.text_count_range_index(),
                    candidate.source_start(),
                    candidate.source_end(),
                    candidate.text_count_range_span(),
                    format_model_layout_exact_evidence(candidate.line_word_evidence()),
                    format_line_word_value_refs(line_words.as_deref(), line_start),
                    format_line_word_value_refs(line_words.as_deref(), line_end),
                    format_model_layout_exact_evidence(candidate.page_field_evidence()),
                    format_page_be32_field_value_refs(page_mark.as_ref(), page_start),
                    format_page_be32_field_value_refs(page_mark.as_ref(), page_end),
                    escaped_text_preview(&text, 80),
                    format_text_count_range_summary(range),
                ))?;
            }
            Ok(())
        }
        Some("text-position-count-layout-context") => {
            let path = required_path(args.next(), "text-position-count-layout-context")?;
            let bytes = read_file(path)?;
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
            if table.text_count_entries().is_empty() {
                return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
            }

            let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
            let line_words = line_stream
                .as_deref()
                .map(|stream| be16_words(stream).collect::<Vec<_>>());
            let page_mark = read_page_mark(&bytes).ok();
            let page_bytes = read_cfb_stream(&bytes, "/PageMark")
                .ok()
                .map(|stream| stream.len());
            let paper_mark = read_paper_mark(&bytes).ok();
            let paper_bytes = read_cfb_stream(&bytes, "/PaperMark")
                .ok()
                .map(|stream| stream.len());

            write_stdout_line(&format!(
                "summary\tentries={}\tline-bytes={}\tline-words={}\tpage-rows={}\tpage-bytes={}\tpaper-rows={}\tpaper-bytes={}",
                table.text_count_entries().len(),
                format_optional_usize(line_stream.as_ref().map(|stream| stream.len())),
                format_optional_usize(line_words.as_ref().map(Vec::len)),
                format_optional_usize(page_mark.as_ref().map(|mark| mark.entries().len())),
                format_optional_usize(page_bytes),
                format_optional_usize(paper_mark.as_ref().map(|mark| mark.entries().len())),
                format_optional_usize(paper_bytes),
            ))?;

            for entry in table.text_count_entries() {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (start, end) = text_count_entry_chosen_range(raw, family);
                write_stdout_line(&format!(
                    "entry\t{}\t{}\t{}\t{}\tline-word-start={}\tline-word-end={}\tline-byte-start={}\tline-byte-end={}\tpage-row-start={}\tpage-row-end={}\tpage-byte-start={}\tpage-byte-end={}\tpaper-row-start={}\tpaper-row-end={}\tpaper-byte-start={}\tpaper-byte-end={}",
                    entry.index(),
                    family,
                    start,
                    end,
                    format_line_word_index_context(line_words.as_deref(), start as usize),
                    format_line_word_index_context(line_words.as_deref(), end as usize),
                    format_line_byte_offset_context(
                        line_words.as_deref(),
                        line_stream.as_ref().map(|stream| stream.len()),
                        start as usize
                    ),
                    format_line_byte_offset_context(
                        line_words.as_deref(),
                        line_stream.as_ref().map(|stream| stream.len()),
                        end as usize
                    ),
                    format_index_context(
                        page_mark.as_ref().map(|mark| mark.entries().len()),
                        start as usize
                    ),
                    format_index_context(
                        page_mark.as_ref().map(|mark| mark.entries().len()),
                        end as usize
                    ),
                    format_index_context(page_bytes, start as usize),
                    format_index_context(page_bytes, end as usize),
                    format_index_context(
                        paper_mark.as_ref().map(|mark| mark.entries().len()),
                        start as usize
                    ),
                    format_index_context(
                        paper_mark.as_ref().map(|mark| mark.entries().len()),
                        end as usize
                    ),
                    format_index_context(paper_bytes, start as usize),
                    format_index_context(paper_bytes, end as usize),
                ))?;
            }
            Ok(())
        }
        Some("text-position-style-context") => {
            let path = required_path(args.next(), "text-position-style-context")?;
            let bytes = read_file(path)?;
            let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
            let map = map_document_text(payload.bytes());
            let (position_status, table) = match read_document_text_position_tables(&bytes) {
                Ok(table) => ("ok".to_string(), Some(table)),
                Err(Error::NotFound(_)) => ("missing".to_string(), None),
                Err(Error::InvalidData(message)) => {
                    (format!("invalid:{}", escaped_text(&message)), None)
                }
                Err(error) => return Err(error.to_string()),
            };
            let text_count_entries = table
                .as_ref()
                .map(|table| table.text_count_entries())
                .unwrap_or(&[]);
            let style_streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
            let text_style_candidates =
                collect_labeled_style_candidates(&style_streams, TEXT_LAYOUT_STYLE_PATH);
            let page_style_candidates =
                collect_labeled_style_candidates(&style_streams, PAGE_LAYOUT_STYLE_PATH);
            let view_style_groups = collect_document_view_style_groups(&style_streams);
            let view_style_records = style_streams
                .iter()
                .find(|stream| stream.name() == DOCUMENT_VIEW_STYLES_PATH)
                .map(|stream| stream.summary().records().len())
                .unwrap_or_default();

            write_stdout_line(&format!(
                "summary\tposition-status={}\tentries={}\ttext-style-candidates={}\tpage-style-candidates={}\tview-style-records={}",
                position_status,
                text_count_entries.len(),
                text_style_candidates.len(),
                page_style_candidates.len(),
                view_style_records
            ))?;

            for entry in text_count_entries {
                let raw = entry.raw();
                let family = classify_text_count_entry_family(raw);
                let (start, end) = text_count_entry_chosen_range(raw, family);
                let tail_offset = text_count_entry_tail_offset(family);
                let tail_fields = read_be16_fields(&raw[tail_offset..]);
                write_stdout_line(&format!(
                    "entry\t{}\t{}\tstart={}\tend={}\tspan={}\ttail-fields={}\ttext-style-id-hits={}\ttext-style-index-hits={}\tpage-style-id-hits={}\tpage-style-index-hits={}\tview-style-group-hits={}\tbyte-range={}",
                    entry.index(),
                    family,
                    start,
                    end,
                    end.saturating_sub(start),
                    format_indexed_u16_fields(&tail_fields),
                    format_style_id_hits(&tail_fields, &text_style_candidates),
                    format_style_index_hits(&tail_fields, &text_style_candidates),
                    format_style_id_hits(&tail_fields, &page_style_candidates),
                    format_style_index_hits(&tail_fields, &page_style_candidates),
                    format_view_style_group_hits(&tail_fields, &view_style_groups),
                    format_byte_range_preview(map.entries(), start as usize, end as usize)
                ))?;
            }
            Ok(())
        }
        Some("text-position-style-summary") => {
            let path = required_path(args.next(), "text-position-style-summary")?;
            let bytes = read_file(path)?;
            let (position_status, table) = match read_document_text_position_tables(&bytes) {
                Ok(table) => ("ok".to_string(), Some(table)),
                Err(Error::NotFound(_)) => ("missing".to_string(), None),
                Err(Error::InvalidData(message)) => {
                    (format!("invalid:{}", escaped_text(&message)), None)
                }
                Err(error) => return Err(error.to_string()),
            };
            let text_count_entries = table
                .as_ref()
                .map(|table| table.text_count_entries())
                .unwrap_or(&[]);
            let style_streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
            let text_style_candidates =
                collect_labeled_style_candidates(&style_streams, TEXT_LAYOUT_STYLE_PATH);
            let page_style_candidates =
                collect_labeled_style_candidates(&style_streams, PAGE_LAYOUT_STYLE_PATH);
            let view_style_groups = collect_document_view_style_groups(&style_streams);
            let view_style_records = style_streams
                .iter()
                .find(|stream| stream.name() == DOCUMENT_VIEW_STYLES_PATH)
                .map(|stream| stream.summary().records().len())
                .unwrap_or_default();
            let field_summaries = summarize_text_position_style_fields(
                text_count_entries,
                &text_style_candidates,
                &page_style_candidates,
                &view_style_groups,
            );

            write_stdout_line(&format!(
                "summary\tposition-status={}\tentries={}\ttext-style-candidates={}\tpage-style-candidates={}\tview-style-records={}",
                position_status,
                text_count_entries.len(),
                text_style_candidates.len(),
                page_style_candidates.len(),
                view_style_records
            ))?;

            for (field_index, field) in field_summaries.iter().enumerate() {
                write_stdout_line(&format!(
                    "field\tf{}\tnonzero={}\tdistinct={}\tvalues={}\ttext-style-id-hits={}\ttext-style-index-hits={}\tpage-style-id-hits={}\tpage-style-index-hits={}\tview-style-group-hits={}",
                    field_index,
                    field.nonzero_count,
                    field.distinct_values.len(),
                    format_u16_value_counts(&field.value_counts),
                    format_candidate_id_hit_counts(
                        &field.text_style_id_hits,
                        &text_style_candidates
                    ),
                    format_candidate_index_hit_counts(
                        &field.text_style_index_hits,
                        &text_style_candidates
                    ),
                    format_candidate_id_hit_counts(
                        &field.page_style_id_hits,
                        &page_style_candidates
                    ),
                    format_candidate_index_hit_counts(
                        &field.page_style_index_hits,
                        &page_style_candidates
                    ),
                    format_view_style_group_hit_counts(
                        &field.view_style_group_hits,
                        &view_style_groups
                    )
                ))?;
            }
            Ok(())
        }
        Some("paper-marks") => {
            let path = required_path(args.next(), "paper-marks")?;
            let bytes = read_file(path)?;
            let paper_mark = read_paper_mark(&bytes).map_err(|error| error.to_string())?;
            let header = paper_mark.header();
            write_stdout_line(&format!(
                "header\t{}\t{}\t{}\t{}",
                header.count_value(),
                header.stride_value(),
                header.last_index_value(),
                paper_mark.entries().len()
            ))?;
            for entry in paper_mark.entries() {
                write_stdout_line(&format!(
                    "entry\t{}\t0x{:08x}",
                    entry.index(),
                    entry.flags()
                ))?;
            }
            Ok(())
        }
        Some("paper-mark-shape") => {
            let path = required_path(args.next(), "paper-mark-shape")?;
            let bytes = read_file(path)?;
            let location = inspect_cfb_stream_location(&bytes, "/PaperMark")
                .map_err(|error| error.to_string())?;
            let stream =
                read_cfb_stream(&bytes, "/PaperMark").map_err(|error| error.to_string())?;
            write_stdout_line(&format!(
                "stream\t{}\t{}\t{}",
                stream.len(),
                location.size(),
                location.storage().as_str()
            ))?;
            write_stdout_line(&format!(
                "alignment\tu32\t{}",
                stream.len().is_multiple_of(4)
            ))?;

            if stream.len() < 12 {
                write_stdout_line("header\t-\t-\t-")?;
                return Ok(());
            }

            let header_count = read_be32_candidate(&stream, 0);
            let header_stride = read_be32_candidate(&stream, 4);
            let header_last = read_be32_candidate(&stream, 8);
            write_stdout_line(&format!(
                "header\t{}\t{}\t{}",
                header_count, header_stride, header_last
            ))?;

            let tail_bytes = stream.len() - 12;
            let classification =
                classify_paper_mark_shape(tail_bytes, header_count, header_stride, header_last);
            write_stdout_line(&format!(
                "classification\t{}\t{}\t{}\t{}",
                classification.name,
                format_optional_usize(classification.rows),
                format_optional_usize(classification.row_bytes),
                classification.trim_bytes
            ))?;
            write_fixed_row_candidate("fixed8", tail_bytes, 8)?;
            write_header_row_candidate(
                "count-plus-one",
                tail_bytes,
                header_count.saturating_add(1),
            )?;
            write_header_row_candidate("count", tail_bytes, header_count)?;
            Ok(())
        }
        Some("page-marks") => {
            let path = required_path(args.next(), "page-marks")?;
            let bytes = read_file(path)?;
            let page_mark = read_page_mark(&bytes).map_err(|error| error.to_string())?;
            let header = page_mark.header();
            write_stdout_line(&format!(
                "header\t{}\t{}\t{}\t{}",
                header.count_value(),
                header.stride_value(),
                header.last_index_value(),
                page_mark.entries().len()
            ))?;
            write_stdout_line(&format!(
                "family\t{}\t{}\t{}",
                page_mark.family().as_str(),
                page_mark
                    .entries()
                    .first()
                    .map(|entry| entry.raw().len().to_string())
                    .unwrap_or_else(|| "-".to_string()),
                page_mark.trailing_bytes().len()
            ))?;
            for (row, entry) in page_mark.entries().iter().enumerate() {
                let u16_fields = be16_words(entry.raw()).collect::<Vec<_>>();
                let u16_profile = page_mark_u16_geometry_profile(&u16_fields);
                write_stdout_line(&format!(
                    "entry\t{}\t{}\t{}\tflags={}\tlineStart={}\tlineEnd={}\tu16Class={}",
                    row,
                    format_optional_u32(entry.index()),
                    bytes_to_hex(entry.raw()),
                    format_optional_u32(entry.flags()),
                    format_optional_u32(entry.line_start()),
                    format_optional_u32(entry.line_end()),
                    u16_profile.class_name()
                ))?;
            }
            if !page_mark.trailing_bytes().is_empty() {
                write_stdout_line(&format!(
                    "trailing\t{}",
                    bytes_to_hex(page_mark.trailing_bytes())
                ))?;
            }
            Ok(())
        }
        Some("page-mark-u16-profile") => {
            let path = required_path(args.next(), "page-mark-u16-profile")?;
            let bytes = read_file(path)?;
            let page_mark = read_page_mark(&bytes).map_err(|error| error.to_string())?;
            write_page_mark_u16_profile(&page_mark)
        }
        Some("page-mark-shape") => {
            let path = required_path(args.next(), "page-mark-shape")?;
            let bytes = read_file(path)?;
            let location = inspect_cfb_stream_location(&bytes, "/PageMark")
                .map_err(|error| error.to_string())?;
            let stream = read_cfb_stream(&bytes, "/PageMark").map_err(|error| error.to_string())?;
            write_stdout_line(&format!(
                "stream\t{}\t{}\t{}",
                stream.len(),
                location.size(),
                location.storage().as_str()
            ))?;
            write_stdout_line(&format!(
                "alignment\tu32\t{}",
                stream.len().is_multiple_of(4)
            ))?;

            if stream.len() < 12 {
                write_stdout_line("header\t-\t-\t-")?;
                return Ok(());
            }

            let header_count = read_be32_candidate(&stream, 0);
            let header_stride = read_be32_candidate(&stream, 4);
            let header_last = read_be32_candidate(&stream, 8);
            write_stdout_line(&format!(
                "header\t{}\t{}\t{}",
                header_count, header_stride, header_last
            ))?;

            let tail_bytes = stream.len() - 12;
            let classification =
                classify_page_mark_shape(tail_bytes, header_count, header_stride, header_last);
            write_stdout_line(&format!(
                "classification\t{}\t{}\t{}\t{}",
                classification.name,
                format_optional_usize(classification.rows),
                format_optional_usize(classification.row_bytes),
                classification.trim_bytes
            ))?;
            write_fixed_row_candidate("fixed84", tail_bytes, 84)?;
            write_header_row_candidate(
                "count-plus-one",
                tail_bytes,
                header_count.saturating_add(1),
            )?;
            write_header_row_candidate("count", tail_bytes, header_count)?;
            if tail_bytes >= 2 {
                write_fixed_row_candidate("fixed84-trim2", tail_bytes - 2, 84)?;
                write_header_row_candidate(
                    "count-plus-one-trim2",
                    tail_bytes - 2,
                    header_count.saturating_add(1),
                )?;
                write_header_row_candidate("count-trim2", tail_bytes - 2, header_count)?;
            }
            Ok(())
        }
        Some("text-map") => {
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
        Some("text-position-context") => {
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
        Some("text-position-line-context") => {
            let path = required_path(args.next(), "text-position-line-context")?;
            let bytes = read_file(path)?;
            let line_stream =
                read_cfb_stream(&bytes, "/LineMark").map_err(|error| error.to_string())?;
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
            let table =
                read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;

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
        Some("text-position-delta-scan") => {
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
        Some("page-layer-tree") => {
            let path = required_path(args.next(), "page-layer-tree")?;
            let page_index = required_page_index(args.next(), "page-layer-tree")?;
            let bytes = read_file(&path)?;
            let mut core = DocumentCore::from_bytes(&bytes).map_err(|error| error.to_string())?;
            core.set_file_name(&path);
            write_stdout(
                &core
                    .get_page_layer_tree(page_index)
                    .map_err(|error| error.to_string())?,
            )
        }
        Some("page-info") => {
            let path = required_path(args.next(), "page-info")?;
            let page_index = required_page_index(args.next(), "page-info")?;
            let bytes = read_file(&path)?;
            let mut core = DocumentCore::from_bytes(&bytes).map_err(|error| error.to_string())?;
            core.set_file_name(&path);
            write_stdout(
                &core
                    .get_page_info(page_index)
                    .map_err(|error| error.to_string())?,
            )
        }
        Some("page-svg") => {
            let path = required_path(args.next(), "page-svg")?;
            let page_index = required_page_index(args.next(), "page-svg")?;
            let bytes = read_file(&path)?;
            let mut core = DocumentCore::from_bytes(&bytes).map_err(|error| error.to_string())?;
            core.set_file_name(&path);
            write_stdout(
                &core
                    .render_page_svg(page_index)
                    .map_err(|error| error.to_string())?,
            )
        }
        Some("export") => {
            let path = required_path(args.next(), "export")?;
            let options = export_options(args)?;
            let bytes = read_file(&path)?;
            let document = parse_document(&bytes).map_err(|error| error.to_string())?;

            match options.format.as_str() {
                "json" => write_stdout(&to_json(&document))?,
                "md" | "markdown" => write_stdout(&to_markdown(&document))?,
                "txt" | "text" => write_stdout(&to_plain_text(&document))?,
                "pdf" => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let Some(output_path) = options.output.as_deref() else {
                            return Err(
                                "PDF export requires `-o <output.pdf>` or `--output <output.pdf>`"
                                    .into(),
                            );
                        };
                        let pdf = to_pdf_with_file_name(&document, &path)?;
                        write_file(output_path, &pdf)?;
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        return Err("PDF export is only available on native targets".into());
                    }
                }
                "html" => {
                    return Err("HTML export is planned after the document model matures".into());
                }
                other => return Err(format!("unsupported export format: {other}")),
            }
            Ok(())
        }
        Some(command) => Err(format!("unknown command: {command}")),
    }
}

fn required_path(path: Option<String>, command: &str) -> Result<String, String> {
    path.ok_or_else(|| format!("missing path for `{command}`"))
}

fn required_page_index(page_index: Option<String>, command: &str) -> Result<u32, String> {
    let value = page_index.ok_or_else(|| format!("missing page index for `{command}`"))?;
    value
        .parse::<u32>()
        .map_err(|_| format!("invalid page index `{value}` for `{command}`"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExportOptions {
    format: String,
    output: Option<String>,
}

fn export_options(args: impl Iterator<Item = String>) -> Result<ExportOptions, String> {
    let mut format = None;
    let mut output = None;
    let mut args = args.peekable();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--format" => {
                let Some(value) = args.next() else {
                    return Err("missing value for `--format`".to_string());
                };
                format = Some(value);
            }
            "--output" | "-o" => {
                let Some(value) = args.next() else {
                    return Err(format!("missing value for `{arg}`"));
                };
                output = Some(value);
            }
            other => {
                return Err(format!(
                    "unexpected export argument `{other}`; usage: rjtd export <file> --format <json|md|text|html|pdf> [-o output.pdf]"
                ));
            }
        }
    }

    Ok(ExportOptions {
        format: format.ok_or_else(|| {
            "usage: rjtd export <file> --format <json|md|text|html|pdf> [-o output.pdf]".to_string()
        })?,
        output,
    })
}

fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>, String> {
    let path = path.as_ref();
    std::fs::read(path).map_err(|error| format!("cannot read `{}`: {error}", path.display()))
}

fn write_file(path: impl AsRef<Path>, bytes: &[u8]) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create `{}`: {error}", parent.display()))?;
    }
    std::fs::write(path, bytes)
        .map_err(|error| format!("cannot write `{}`: {error}", path.display()))
}

fn print_help() -> Result<(), String> {
    write_stdout(
        "\
rjtd

Rust-based Ichitaro (JTD) Document Engine

Usage:
  rjtd streams <file.jtd>
  rjtd info <file.jtd>
  rjtd dump-stream <file.jtd> <stream-path>
  rjtd style-records <file.jtd>
  rjtd page-layout-style-slots <file.jtd>
  rjtd style-candidates <file.jtd>
  rjtd text-layout-style-records <file.jtd>
  rjtd document-view-style-groups <file.jtd>
  rjtd cfb-map <file.jtd>
  rjtd cfb-dir <file.jtd>
  rjtd stream-meta <file.jtd> <stream-path>
  rjtd stream-chain <file.jtd> <stream-path>
  rjtd stream-words <file.jtd> <stream-path>
  rjtd stream-word-frequencies <file.jtd> <stream-path>
  rjtd line-mark-tags <file.jtd>
  rjtd line-mark-intervals <file.jtd>
  rjtd line-mark-text-context <file.jtd>
  rjtd stream-dwords <file.jtd> <stream-path>
  rjtd stream-dword-frequencies <file.jtd> <stream-path>
  rjtd stream-text-probe <file.jtd> <stream-path>
  rjtd stream-find <file.jtd> <stream-path>
  rjtd stream-find-bytes <file.jtd> <hex-bytes>
  rjtd so-records <file.jtd>
  rjtd object-stream-candidates <file.jtd>
  rjtd object-ownership-references <file.jtd>
  rjtd object-ownership-reference-fields <file.jtd>
  rjtd object-frame-reference-records <file.jtd>
  rjtd object-frame-record-families <file.jtd>
  rjtd object-frame-row-links <file.jtd>
  rjtd object-image-frame-candidates <file.jtd>
  rjtd object-fdm-image-candidates <file.jtd>
  rjtd object-fdm-frame-links <file.jtd>
  rjtd object-fdm-index <file.jtd>
  rjtd object-fdm-index-shape <file.jtd>
  rjtd object-fdm-index-rows <file.jtd>
  rjtd so-record-clusters <file.jtd>
  rjtd so-record-fields <file.jtd>
  rjtd so-record-geometry <file.jtd>
  rjtd so-record-halves <file.jtd>
  rjtd cat <file.jtd>
  rjtd text-tokens <file.jtd>
  rjtd text-control-context <file.jtd> [control-code]
  rjtd text-control-clusters <file.jtd> [control-code]
  rjtd text-control-ranges <file.jtd> [control-code]
  rjtd text-positions <file.jtd>
  rjtd text-position-mark-header <file.jtd>
  rjtd text-position-mark-summary <file.jtd>
  rjtd text-position-counts <file.jtd>
  rjtd text-position-count-context <file.jtd>
  rjtd text-position-count-tail-context <file.jtd>
  rjtd text-position-count-clusters <file.jtd>
  rjtd text-position-count-candidates <file.jtd>
  rjtd text-position-count-family <file.jtd>
  rjtd text-position-count-fields <file.jtd>
  rjtd text-position-count-field-deltas <file.jtd>
  rjtd text-position-count-tail-delta-scan <file.jtd>
  rjtd text-position-count-tail-delta-groups <file.jtd>
  rjtd text-position-count-tail-row-deltas <file.jtd>
  rjtd text-position-count-tail-row-context <file.jtd>
  rjtd text-position-count-tail-field-roles <file.jtd>
  rjtd text-position-count-range-preview <file.jtd>
  rjtd text-position-count-range-boundaries <file.jtd>
  rjtd text-position-count-control-ranges <file.jtd> [control-code]
  rjtd text-boundary-candidates <file.jtd>
  rjtd table-candidates <file.jtd>
  rjtd table-candidate-context <file.jtd>
  rjtd table-cell-like-candidates <file.jtd>
  rjtd text-boundary-candidate-context <file.jtd>
  rjtd text-boundary-candidate-agreement <file.jtd>
  rjtd text-boundary-candidate-layout-context <file.jtd>
  rjtd text-boundary-layout-map <file.jtd>
  rjtd text-boundary-layout-map-rows <file.jtd>
  rjtd text-boundary-paragraph-like <file.jtd>
  rjtd text-boundary-paragraph-like-style-context <file.jtd>
  rjtd text-boundary-paragraph-like-discriminators <file.jtd>
  rjtd text-paragraph-boundary-targets <file.jtd>
  rjtd text-position-count-layout-context <file.jtd>
  rjtd text-position-style-context <file.jtd>
  rjtd text-position-style-summary <file.jtd>
  rjtd paper-marks <file.jtd>
  rjtd paper-mark-shape <file.jtd>
  rjtd page-marks <file.jtd>
  rjtd page-mark-u16-profile <file.jtd>
  rjtd page-mark-shape <file.jtd>
  rjtd text-map <file.jtd>
  rjtd text-position-context <file.jtd>
  rjtd text-position-line-context <file.jtd>
  rjtd text-position-delta-scan <file.jtd>
  rjtd page-info <file.jtd> <zero-based-page-index>
  rjtd page-layer-tree <file.jtd> <zero-based-page-index>
  rjtd page-svg <file.jtd> <zero-based-page-index>
  rjtd export <file.jtd> --format <json|md|text|html|pdf> [-o output.pdf]
",
    )
}

fn print_entry_size(
    entries: &[rjtd_core::container::ContainerEntry],
    path: &str,
    label: &str,
) -> Result<(), String> {
    let value = entries
        .iter()
        .find(|entry| entry.path() == path)
        .map(|entry| entry.size().to_string())
        .unwrap_or_else(|| "-".to_string());
    write_stdout_line(&format!("{label}\t{value}"))
}

fn write_cfb_chain(label: &str, chain: &CfbSectorChain) -> Result<(), String> {
    write_stdout_line(&format!(
        "{}\t{}\t{}\t{}",
        label,
        chain.status().as_str(),
        chain.sectors().len(),
        format_sector_ids(chain.sectors())
    ))
}

fn format_sector_ids(sectors: &[u32]) -> String {
    if sectors.is_empty() {
        return "-".to_string();
    }

    sectors
        .iter()
        .map(|sector| sector.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn format_cfb_id(id: u32) -> String {
    if id == 0xffff_ffff {
        "-".to_string()
    } else {
        id.to_string()
    }
}

fn format_u32_hex_values(values: &[u32]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }

    values
        .iter()
        .map(|value| format!("0x{value:08x}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_u16_hex_values(values: &[u16]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }

    values
        .iter()
        .map(|value| format!("0x{value:04x}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_usize_values(values: &[usize]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }

    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn format_optional_text(value: Option<&str>) -> String {
    value
        .filter(|text| !text.is_empty())
        .map(escaped_text)
        .unwrap_or_else(|| "-".to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliStyleCandidate {
    id: usize,
    record_index: usize,
    offset: usize,
    label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DocumentViewStyleGroup {
    id: u16,
    record_count: usize,
    codes: Vec<u16>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct StyleFieldSummary {
    nonzero_count: usize,
    distinct_values: BTreeSet<u16>,
    value_counts: BTreeMap<u16, usize>,
    text_style_id_hits: BTreeMap<usize, usize>,
    text_style_index_hits: BTreeMap<usize, usize>,
    page_style_id_hits: BTreeMap<usize, usize>,
    page_style_index_hits: BTreeMap<usize, usize>,
    view_style_group_hits: BTreeMap<u16, usize>,
}

fn collect_labeled_style_candidates(
    streams: &[rjtd_core::style_stream::StyleStream],
    path: &str,
) -> Vec<CliStyleCandidate> {
    let mut candidates = Vec::new();

    for stream in streams {
        if stream.name() != path {
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

            candidates.push(CliStyleCandidate {
                id: candidates.len() + 1,
                record_index,
                offset: record.offset(),
                label: label.to_string(),
            });
        }
    }

    candidates
}

fn collect_document_view_style_groups(
    streams: &[rjtd_core::style_stream::StyleStream],
) -> Vec<DocumentViewStyleGroup> {
    let mut groups: BTreeMap<u16, Vec<u16>> = BTreeMap::new();

    for stream in streams {
        if stream.name() != DOCUMENT_VIEW_STYLES_PATH {
            continue;
        }

        for record in stream.summary().records() {
            if let Some(group_id) = document_view_style_group_id(record.code()) {
                groups.entry(group_id).or_default().push(record.code());
            }
        }
    }

    groups
        .into_iter()
        .map(|(id, mut codes)| {
            codes.sort_unstable();
            codes.dedup();
            DocumentViewStyleGroup {
                id,
                record_count: codes.len(),
                codes,
            }
        })
        .collect()
}

fn document_view_style_group_id(code: u16) -> Option<u16> {
    let high = code >> 8;
    let low = code & 0x00ff;
    if (0x31..=0x39).contains(&high) && (0x04..=0x07).contains(&low) {
        Some(high - 0x30)
    } else {
        None
    }
}

fn style_record_payload<'a>(
    stream_bytes: &'a [u8],
    record: &StyleStreamRecordSummary,
) -> Option<&'a [u8]> {
    let start = record.offset().checked_add(4)?;
    let end = start.checked_add(record.payload_len())?;
    stream_bytes.get(start..end)
}

fn format_style_record_payload_preview(
    stream_bytes: &[u8],
    record: &StyleStreamRecordSummary,
) -> String {
    let Some(payload) = style_record_payload(stream_bytes, record) else {
        return "invalid".to_string();
    };
    format_hex_preview(payload, STYLE_RECORD_PAYLOAD_PREVIEW_BYTES)
}

fn format_style_record_payload_be16(
    stream_bytes: &[u8],
    record: &StyleStreamRecordSummary,
) -> String {
    let Some(payload) = style_record_payload(stream_bytes, record) else {
        return "invalid".to_string();
    };
    format_be16_hex_fields(payload)
}

fn format_style_record_payload_digest(
    stream_bytes: &[u8],
    record: &StyleStreamRecordSummary,
) -> String {
    let Some(payload) = style_record_payload(stream_bytes, record) else {
        return "invalid".to_string();
    };
    format_fnv1a64_digest(fnv1a64(payload))
}

fn format_document_view_group_payload_digest(
    stream_bytes: &[u8],
    records: &[(usize, &StyleStreamRecordSummary)],
) -> String {
    let mut digest = FNV1A64_OFFSET;
    for (_, record) in records {
        let Some(payload) = style_record_payload(stream_bytes, record) else {
            return "invalid".to_string();
        };
        digest = fnv1a64_update(digest, payload);
    }
    format_fnv1a64_digest(digest)
}

fn summarize_text_position_style_fields(
    entries: &[rjtd_core::document_text_position::DocumentTextCountEntry],
    text_style_candidates: &[CliStyleCandidate],
    page_style_candidates: &[CliStyleCandidate],
    view_style_groups: &[DocumentViewStyleGroup],
) -> Vec<StyleFieldSummary> {
    let mut fields = Vec::new();

    for entry in entries {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);

        if fields.len() < tail_fields.len() {
            fields.resize_with(tail_fields.len(), StyleFieldSummary::default);
        }

        for (field_index, value) in tail_fields.into_iter().enumerate() {
            if value == 0 {
                continue;
            }

            let field = &mut fields[field_index];
            field.nonzero_count += 1;
            field.distinct_values.insert(value);
            *field.value_counts.entry(value).or_insert(0) += 1;
            if let Some(candidate) = text_style_candidates
                .iter()
                .find(|candidate| candidate.id == value as usize)
            {
                *field.text_style_id_hits.entry(candidate.id).or_insert(0) += 1;
            }
            if let Some(candidate) = text_style_candidates
                .iter()
                .find(|candidate| candidate.record_index == value as usize)
            {
                *field
                    .text_style_index_hits
                    .entry(candidate.record_index)
                    .or_insert(0) += 1;
            }
            if let Some(candidate) = page_style_candidates
                .iter()
                .find(|candidate| candidate.id == value as usize)
            {
                *field.page_style_id_hits.entry(candidate.id).or_insert(0) += 1;
            }
            if let Some(candidate) = page_style_candidates
                .iter()
                .find(|candidate| candidate.record_index == value as usize)
            {
                *field
                    .page_style_index_hits
                    .entry(candidate.record_index)
                    .or_insert(0) += 1;
            }
            if let Some(group) = view_style_groups.iter().find(|group| group.id == value) {
                *field.view_style_group_hits.entry(group.id).or_insert(0) += 1;
            }
        }
    }

    fields
}

fn format_indexed_u16_fields(fields: &[u16]) -> String {
    if fields.is_empty() {
        return "-".to_string();
    }

    fields
        .iter()
        .enumerate()
        .map(|(index, value)| format!("f{index}=0x{value:04x}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_u16_value_counts(counts: &BTreeMap<u16, usize>) -> String {
    if counts.is_empty() {
        return "-".to_string();
    }

    counts
        .iter()
        .map(|(value, count)| format!("0x{value:04x}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_string_counts(counts: &BTreeMap<String, usize>) -> String {
    if counts.is_empty() {
        return "-".to_string();
    }

    counts
        .iter()
        .map(|(value, count)| format!("{}:{}", escaped_text(value), count))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_min_max(min: Option<usize>, max: Option<usize>) -> String {
    match (min, max) {
        (Some(min), Some(max)) => format!("{min}..{max}"),
        _ => "-".to_string(),
    }
}

fn update_min_max(min: &mut Option<usize>, max: &mut Option<usize>, value: usize) {
    *min = Some(min.map_or(value, |min| min.min(value)));
    *max = Some(max.map_or(value, |max| max.max(value)));
}

fn count_tail_field(counts: &mut BTreeMap<u16, usize>, fields: &[u16], index: usize) {
    if let Some(value) = fields.get(index) {
        *counts.entry(*value).or_insert(0) += 1;
    }
}

fn has_style_hit(fields: &[u16], candidates: &[CliStyleCandidate]) -> bool {
    fields.iter().filter(|value| **value != 0).any(|value| {
        candidates.iter().any(|candidate| {
            candidate.id == *value as usize || candidate.record_index == *value as usize
        })
    })
}

fn has_view_style_group_hit(fields: &[u16], groups: &[DocumentViewStyleGroup]) -> bool {
    fields
        .iter()
        .filter(|value| **value != 0)
        .any(|value| groups.iter().any(|group| group.id == *value))
}

fn format_view_style_group_hits(fields: &[u16], groups: &[DocumentViewStyleGroup]) -> String {
    let hits = fields
        .iter()
        .enumerate()
        .filter(|(_, value)| **value != 0)
        .filter_map(|(field_index, value)| {
            let group = groups.iter().find(|group| group.id == *value)?;
            Some(format!(
                "f{}=0x{:04x}:group{}:records{}:codes{}",
                field_index,
                value,
                group.id,
                group.record_count,
                format_u16_hex_values(&group.codes)
            ))
        })
        .collect::<Vec<_>>();
    if hits.is_empty() {
        "-".to_string()
    } else {
        hits.join(",")
    }
}

fn format_style_id_hits(fields: &[u16], candidates: &[CliStyleCandidate]) -> String {
    let hits = fields
        .iter()
        .enumerate()
        .filter(|(_, value)| **value != 0)
        .filter_map(|(field_index, value)| {
            let candidate = candidates
                .iter()
                .find(|candidate| candidate.id == *value as usize)?;
            Some(format!(
                "f{}=0x{:04x}:id{}:offset{}:{}",
                field_index,
                value,
                candidate.id,
                candidate.offset,
                escaped_text(&candidate.label)
            ))
        })
        .collect::<Vec<_>>();
    if hits.is_empty() {
        "-".to_string()
    } else {
        hits.join(",")
    }
}

fn format_candidate_id_hit_counts(
    hits: &BTreeMap<usize, usize>,
    candidates: &[CliStyleCandidate],
) -> String {
    if hits.is_empty() {
        return "-".to_string();
    }

    hits.iter()
        .filter_map(|(candidate_id, count)| {
            let candidate = candidates
                .iter()
                .find(|candidate| candidate.id == *candidate_id)?;
            Some(format!(
                "id{}:{}:offset{}:{}",
                candidate.id,
                count,
                candidate.offset,
                escaped_text(&candidate.label)
            ))
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn format_view_style_group_hit_counts(
    hits: &BTreeMap<u16, usize>,
    groups: &[DocumentViewStyleGroup],
) -> String {
    if hits.is_empty() {
        return "-".to_string();
    }

    hits.iter()
        .filter_map(|(group_id, count)| {
            let group = groups.iter().find(|group| group.id == *group_id)?;
            Some(format!(
                "group{}:{}:records{}:codes{}",
                group.id,
                count,
                group.record_count,
                format_u16_hex_values(&group.codes)
            ))
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn format_candidate_index_hit_counts(
    hits: &BTreeMap<usize, usize>,
    candidates: &[CliStyleCandidate],
) -> String {
    if hits.is_empty() {
        return "-".to_string();
    }

    hits.iter()
        .filter_map(|(record_index, count)| {
            let candidate = candidates
                .iter()
                .find(|candidate| candidate.record_index == *record_index)?;
            Some(format!(
                "idx{}:{}:id{}:offset{}:{}",
                candidate.record_index,
                count,
                candidate.id,
                candidate.offset,
                escaped_text(&candidate.label)
            ))
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn format_style_index_hits(fields: &[u16], candidates: &[CliStyleCandidate]) -> String {
    let hits = fields
        .iter()
        .enumerate()
        .filter(|(_, value)| **value != 0)
        .filter_map(|(field_index, value)| {
            let candidate = candidates
                .iter()
                .find(|candidate| candidate.record_index == *value as usize)?;
            Some(format!(
                "f{}=0x{:04x}:idx{}:id{}:offset{}:{}",
                field_index,
                value,
                candidate.record_index,
                candidate.id,
                candidate.offset,
                escaped_text(&candidate.label)
            ))
        })
        .collect::<Vec<_>>();
    if hits.is_empty() {
        "-".to_string()
    } else {
        hits.join(",")
    }
}

fn stream_chain_offset_basis(storage: StreamStorage) -> &'static str {
    match storage {
        StreamStorage::Mini => "mini-stream",
        StreamStorage::Regular => "file",
    }
}

fn write_stdout(text: &str) -> Result<(), String> {
    write_stdout_bytes(text.as_bytes())
}

fn write_stdout_line(line: &str) -> Result<(), String> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(line.as_bytes()).map_err(stdout_error)?;
    stdout.write_all(b"\n").map_err(stdout_error)
}

fn write_stdout_bytes(bytes: &[u8]) -> Result<(), String> {
    io::stdout().write_all(bytes).map_err(stdout_error)
}

fn stdout_error(error: io::Error) -> String {
    if error.kind() == io::ErrorKind::BrokenPipe {
        BROKEN_PIPE_EXIT.to_string()
    } else {
        format!("cannot write to stdout: {error}")
    }
}

fn escaped_path(path: &str) -> String {
    let mut escaped = String::new();
    for character in path.chars() {
        if character.is_ascii_control() {
            escaped.push_str(&format!("\\x{:02X}", character as u32));
        } else {
            escaped.push(character);
        }
    }
    escaped
}

fn escaped_text(text: &str) -> String {
    let mut escaped = String::new();
    for character in text.chars() {
        match character {
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_ascii_control() => {
                escaped.push_str(&format!("\\u{:04X}", character as u32));
            }
            character => escaped.push(character),
        }
    }
    escaped
}

fn escaped_text_preview(text: &str, max_chars: usize) -> String {
    let mut preview = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        preview.push_str("...");
    }
    escaped_text(&preview)
}

struct ObjectStreamCandidate {
    path: String,
    size: usize,
    reasons: Vec<&'static str>,
    image_signature_hits: Vec<ObjectSignatureHit>,
    svg_offsets: Vec<usize>,
    so_offsets: Vec<usize>,
    visual_list: Option<CliVisualListCandidate>,
    embedded_press_snapshot: Option<CliEmbeddedPressSnapshotCandidate>,
    jseq3_formula: Option<CliJseq3FormulaCandidate>,
    prefix_hex: String,
}

struct CliVisualListCandidate {
    width: u32,
    height: u32,
    bit_depth: u32,
    rle_data_len: usize,
}

struct CliEmbeddedPressSnapshotCandidate {
    width: u32,
    height: u32,
    body_length_candidate: u32,
    object_count_candidate: u32,
}

struct CliJseq3FormulaCandidate {
    so_trailer_offset: Option<usize>,
    so_trailer_fields: Vec<u32>,
    text_markers: Vec<CliJseq3TextMarkerCandidate>,
}

struct CliJseq3TextMarkerCandidate {
    text: &'static str,
    offset: usize,
}

struct ObjectSignatureHit {
    kind: &'static str,
    offset: usize,
}

fn classify_object_stream_candidate(path: &str, stream: &[u8]) -> Option<ObjectStreamCandidate> {
    let mut reasons = Vec::new();
    push_object_path_reasons(path, &mut reasons);

    let image_signature_hits = image_signature_hits(stream);
    if !image_signature_hits.is_empty() {
        push_unique_reason(&mut reasons, "image-signature");
    }

    let svg_offsets = svg_signature_offsets(stream);
    if !svg_offsets.is_empty() {
        push_unique_reason(&mut reasons, "svg-signature");
    }

    let so_offsets = find_subslice_offsets(stream, SO_RECORD_MARKER);
    if !so_offsets.is_empty() {
        push_unique_reason(&mut reasons, "so-marker");
    }
    let visual_list = visual_list_candidate_from_stream(path, stream);
    let embedded_press_snapshot = embedded_press_snapshot_candidate_from_stream(stream);
    let jseq3_formula = jseq3_formula_candidate_from_stream(path, stream);
    if figure_link_candidate_from_stream(path, stream) {
        push_unique_reason(&mut reasons, "figure-link");
    }
    if embedded_press_snapshot.is_some() {
        push_unique_reason(&mut reasons, "embedded-press-snapshot");
    }
    if jseq3_formula.is_some() {
        push_unique_reason(&mut reasons, "jseq3-formula");
    }

    if reasons.is_empty() {
        return None;
    }

    Some(ObjectStreamCandidate {
        path: path.to_string(),
        size: stream.len(),
        reasons,
        image_signature_hits,
        svg_offsets,
        so_offsets,
        visual_list,
        embedded_press_snapshot,
        jseq3_formula,
        prefix_hex: format_hex_preview(stream, OBJECT_STREAM_PREFIX_PREVIEW_BYTES),
    })
}

fn figure_link_candidate_from_stream(path: &str, stream: &[u8]) -> bool {
    let lower = path.to_ascii_lowercase();
    if !lower.contains("/figuredata/") || !lower.ends_with("/link") {
        return false;
    }
    let header_bytes = 8usize;
    let row_bytes = 14usize;
    if stream.len() < header_bytes + row_bytes {
        return false;
    }
    let row_payload_len = stream.len().saturating_sub(header_bytes);
    if !row_payload_len.is_multiple_of(row_bytes) {
        return false;
    }
    let row_count = row_payload_len / row_bytes;
    if row_count == 0 || read_be16_candidate(stream, 6).map(usize::from) != Some(row_count) {
        return false;
    }
    (0..row_count).all(|row_index| {
        let row_start = header_bytes + row_index * row_bytes;
        read_be16_candidate(stream, row_start + 8) == Some(0x0016)
    })
}

fn embedded_press_snapshot_candidate_from_stream(
    stream: &[u8],
) -> Option<CliEmbeddedPressSnapshotCandidate> {
    if stream.get(..EMBEDDED_PRESS_SNAPSHOT_MAGIC.len())? != EMBEDDED_PRESS_SNAPSHOT_MAGIC {
        return None;
    }
    let body_length_candidate =
        read_le32_candidate(stream, EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET)?;
    let object_count_candidate =
        read_le32_candidate(stream, EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET)?;
    let width = read_le32_candidate(stream, EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET)?;
    let height = read_le32_candidate(stream, EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET)?;
    if body_length_candidate == 0 || width == 0 || height == 0 {
        return None;
    }
    Some(CliEmbeddedPressSnapshotCandidate {
        width,
        height,
        body_length_candidate,
        object_count_candidate,
    })
}

fn jseq3_formula_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<CliJseq3FormulaCandidate> {
    if !path.ends_with("/JSEQ3Contents") {
        return None;
    }
    if stream.get(..JSEQ3_CONTENTS_MAGIC_UTF16LE.len())? != JSEQ3_CONTENTS_MAGIC_UTF16LE {
        return None;
    }
    let so_trailer_offset = find_subslice_offsets(stream, SO_RECORD_MARKER)
        .into_iter()
        .find(|offset| {
            offset.saturating_add(JSEQ3_SO_FIELD_COUNT * JSEQ3_SO_FIELD_BYTES) <= stream.len()
                && offset.saturating_add(JSEQ3_SO_TRAILER_BYTES) >= stream.len()
        });
    let so_trailer_fields = so_trailer_offset
        .and_then(|offset| stream.get(offset..))
        .map(jseq3_so_trailer_fields)
        .unwrap_or_default();
    Some(CliJseq3FormulaCandidate {
        so_trailer_offset,
        so_trailer_fields,
        text_markers: jseq3_text_marker_candidates(stream),
    })
}

fn jseq3_so_trailer_fields(trailer: &[u8]) -> Vec<u32> {
    (0..JSEQ3_SO_FIELD_COUNT)
        .filter_map(|index| read_le32_candidate(trailer, index * JSEQ3_SO_FIELD_BYTES))
        .collect()
}

fn jseq3_text_marker_candidates(stream: &[u8]) -> Vec<CliJseq3TextMarkerCandidate> {
    let mut candidates = Vec::new();
    for marker in JSEQ3_TEXT_MARKERS {
        let encoded = utf16le_bytes(marker);
        for offset in find_subslice_offsets(stream, &encoded) {
            candidates.push(CliJseq3TextMarkerCandidate {
                text: marker,
                offset,
            });
        }
    }
    candidates.sort_by_key(|candidate| candidate.offset);
    candidates
}

fn utf16le_bytes(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

fn visual_list_candidate_from_stream(path: &str, stream: &[u8]) -> Option<CliVisualListCandidate> {
    if !path.to_ascii_lowercase().contains("visuallist") {
        return None;
    }
    if stream.get(VISUAL_LIST_MAGIC_OFFSET..VISUAL_LIST_MAGIC_OFFSET + VISUAL_LIST_MAGIC.len())?
        != VISUAL_LIST_MAGIC
    {
        return None;
    }
    Some(CliVisualListCandidate {
        width: read_be32_at(stream, VISUAL_LIST_WIDTH_OFFSET)?,
        height: read_be32_at(stream, VISUAL_LIST_HEIGHT_OFFSET)?,
        bit_depth: read_be32_at(stream, VISUAL_LIST_BIT_DEPTH_OFFSET)?,
        rle_data_len: read_be32_at(stream, VISUAL_LIST_RLE_LENGTH_OFFSET)? as usize,
    })
}

fn format_embedded_press_snapshot_candidate(
    candidate: Option<&CliEmbeddedPressSnapshotCandidate>,
) -> String {
    candidate
        .map(|candidate| {
            format!(
                "JSSnapShot32,{}x{},body={},objects={}",
                candidate.width,
                candidate.height,
                candidate.body_length_candidate,
                candidate.object_count_candidate
            )
        })
        .unwrap_or_else(|| "-".to_string())
}

fn format_jseq3_formula_candidate(candidate: Option<&CliJseq3FormulaCandidate>) -> String {
    let Some(candidate) = candidate else {
        return "-".to_string();
    };
    let fields = if candidate.so_trailer_fields.is_empty() {
        "-".to_string()
    } else {
        candidate
            .so_trailer_fields
            .iter()
            .map(|field| format!("0x{field:08x}"))
            .collect::<Vec<_>>()
            .join(",")
    };
    let markers = if candidate.text_markers.is_empty() {
        "-".to_string()
    } else {
        candidate
            .text_markers
            .iter()
            .map(|marker| format!("{}@{}", marker.text, marker.offset))
            .collect::<Vec<_>>()
            .join(",")
    };
    format!(
        "MATH.VAF,so={},fields={},markers={}",
        format_optional_usize(candidate.so_trailer_offset),
        fields,
        markers
    )
}

fn format_visual_list_candidate(candidate: Option<&CliVisualListCandidate>) -> String {
    candidate
        .map(|candidate| {
            format!(
                "{}x{}x{}bpp,rle={}",
                candidate.width, candidate.height, candidate.bit_depth, candidate.rle_data_len
            )
        })
        .unwrap_or_else(|| "-".to_string())
}

fn push_object_path_reasons(path: &str, reasons: &mut Vec<&'static str>) {
    let lower = path.to_ascii_lowercase();
    let segments = lower
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &[
                "embeditems",
                "embedding",
                "jsfart",
                "compobj",
                "ole",
                "object",
                "bin",
            ],
        )
    }) {
        push_unique_reason(reasons, "object-path");
    }

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &[
                "image", "picture", "graphic", "bitmap", "png", "jpg", "jpeg", "gif", "bmp", "tif",
                "tiff", "wmf", "emf",
            ],
        )
    }) {
        push_unique_reason(reasons, "image-path");
    }

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &["figure", "shape", "draw", "frame", "layoutbox", "svg"],
        )
    }) {
        push_unique_reason(reasons, "shape-path");
    }

    if segments.iter().any(|segment| {
        contains_any(segment, &["table", "cell", "tbl", "hyo"])
            && !contains_any(segment, &["positiontable", "style"])
    }) {
        push_unique_reason(reasons, "table-path");
    }

    if segments.contains(&"visuallist") {
        push_unique_reason(reasons, "visual-list-path");
    }
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn push_unique_reason(reasons: &mut Vec<&'static str>, reason: &'static str) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn image_signature_hits(stream: &[u8]) -> Vec<ObjectSignatureHit> {
    let mut hits = Vec::new();
    push_signature_hits(&mut hits, stream, "png", b"\x89PNG\r\n\x1a\n", true);
    push_signature_hits(&mut hits, stream, "jpeg", b"\xff\xd8\xff", true);
    push_signature_hits(&mut hits, stream, "gif87a", b"GIF87a", true);
    push_signature_hits(&mut hits, stream, "gif89a", b"GIF89a", true);
    push_signature_hits(&mut hits, stream, "tiff-le", b"II\x2a\0", true);
    push_signature_hits(&mut hits, stream, "tiff-be", b"MM\0\x2a", true);
    push_signature_hits(
        &mut hits,
        stream,
        "wmf-placeable",
        b"\xd7\xcd\xc6\x9a",
        true,
    );
    push_signature_hits(&mut hits, stream, "bmp", b"BM", false);

    hits.sort_by(|left, right| {
        left.offset
            .cmp(&right.offset)
            .then_with(|| left.kind.cmp(right.kind))
    });
    hits
}

fn push_signature_hits(
    hits: &mut Vec<ObjectSignatureHit>,
    stream: &[u8],
    kind: &'static str,
    signature: &[u8],
    scan_anywhere: bool,
) {
    let offsets = if scan_anywhere {
        find_subslice_offsets(stream, signature)
    } else if stream.starts_with(signature) {
        vec![0]
    } else {
        Vec::new()
    };

    for offset in offsets {
        hits.push(ObjectSignatureHit { kind, offset });
    }
}

fn svg_signature_offsets(stream: &[u8]) -> Vec<usize> {
    let ascii_lower = stream
        .iter()
        .map(|byte| byte.to_ascii_lowercase())
        .collect::<Vec<_>>();
    find_subslice_offsets(&ascii_lower, b"<svg")
}

fn object_stream_reason_count(
    reason_counts: &BTreeMap<&'static str, usize>,
    reason: &'static str,
) -> usize {
    reason_counts.get(reason).copied().unwrap_or_default()
}

fn format_object_signature_hits(hits: &[ObjectSignatureHit]) -> String {
    if hits.is_empty() {
        return "-".to_string();
    }

    let mut values = hits
        .iter()
        .take(OBJECT_STREAM_MAX_REPORTED_HITS)
        .map(|hit| format!("{}@{}", hit.kind, hit.offset))
        .collect::<Vec<_>>();
    if hits.len() > OBJECT_STREAM_MAX_REPORTED_HITS {
        values.push(format!("+{}", hits.len() - OBJECT_STREAM_MAX_REPORTED_HITS));
    }
    values.join(",")
}

fn format_usize_hit_list(offsets: &[usize]) -> String {
    if offsets.is_empty() {
        return "-".to_string();
    }

    let mut values = offsets
        .iter()
        .take(OBJECT_STREAM_MAX_REPORTED_HITS)
        .map(usize::to_string)
        .collect::<Vec<_>>();
    if offsets.len() > OBJECT_STREAM_MAX_REPORTED_HITS {
        values.push(format!(
            "+{}",
            offsets.len() - OBJECT_STREAM_MAX_REPORTED_HITS
        ));
    }
    values.join(",")
}

struct ObjectReferenceContext {
    start: usize,
    hex: String,
}

struct ObjectFrameReferenceRecordCandidate {
    encoding: &'static str,
    stride: usize,
    field_offset: usize,
}

impl ObjectFrameReferenceRecordCandidate {
    fn name(&self) -> String {
        format!("{}/{}/{}", self.encoding, self.stride, self.field_offset)
    }
}

#[derive(Debug, Clone)]
struct ObjectFrameReferenceRecord {
    source_path: String,
    embedding_index: Option<usize>,
    target_path: String,
    encoding: String,
    stride: usize,
    field_offset: usize,
    offset: usize,
    row_index: usize,
    row_start: usize,
    candidate: String,
    row: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
struct ObjectFrameReferenceRecordCollection {
    source_count: usize,
    reference_count: usize,
    skipped_count: usize,
    records: Vec<ObjectFrameReferenceRecord>,
}

#[derive(Debug, Clone, Default)]
struct ObjectFrameRecordFamilySummary {
    rows: usize,
    candidates: BTreeSet<String>,
    embedding_indexes: BTreeSet<usize>,
    examples: BTreeSet<String>,
}

#[derive(Debug, Clone, Default)]
struct ObjectImageFrameCandidateSummary {
    embedding_index: Option<usize>,
    payload_kinds: BTreeSet<String>,
    frame_rows: usize,
    family_counts: BTreeMap<String, usize>,
    row12_tail_coordinate: usize,
    row12_tail_zero: usize,
    row20_tail_window: usize,
    row20_linked: usize,
    le_row12: usize,
    coordinate_pairs: Vec<ObjectFrameCoordinatePair>,
    preferred: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ObjectFrameCoordinatePair {
    row_start: usize,
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FdmIndexEntry {
    row_index: usize,
    index_offset: usize,
    row: Vec<u8>,
    vector_offset: usize,
    kind: u16,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    valid_vector_offset: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FdmVectorSegment {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ObjectReferenceFieldKey {
    target_path: String,
    encoding: String,
    stride: usize,
    field_offset: usize,
}

impl ObjectReferenceFieldKey {
    fn new(target_path: &str, encoding: &str, stride: usize, field_offset: usize) -> Self {
        Self {
            target_path: target_path.to_string(),
            encoding: encoding.to_string(),
            stride,
            field_offset,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct ObjectReferenceFieldSummary {
    matches: usize,
    cross_row_matches: usize,
    source_streams: BTreeSet<String>,
    embedding_indexes: BTreeSet<usize>,
    row_indexes: BTreeSet<usize>,
}

fn collect_object_frame_reference_records(
    data: &[u8],
) -> Result<ObjectFrameReferenceRecordCollection, String> {
    let document = parse_document(data).map_err(|error| error.to_string())?;
    let streams = readable_cfb_streams(data)?;
    let mut collection = ObjectFrameReferenceRecordCollection::default();

    for candidate in document.object_stream_candidates() {
        let embedding_index = candidate
            .ownership_candidate()
            .and_then(|ownership| ownership.embedding_index());
        let mut source_reported = false;

        for reference in candidate
            .ownership_reference_candidates()
            .iter()
            .filter(|reference| reference.target_path().eq_ignore_ascii_case("/Frame"))
        {
            collection.reference_count += 1;
            let Some(target_stream) = streams.get(reference.target_path()) else {
                collection.skipped_count += reference.offsets().len();
                continue;
            };

            for offset in reference.offsets() {
                let offset = *offset;
                for projection in
                    OBJECT_FRAME_REFERENCE_RECORD_CANDIDATES
                        .iter()
                        .filter(|projection| {
                            projection.encoding == reference.encoding()
                                && offset % projection.stride == projection.field_offset
                        })
                {
                    let pattern_len = object_reference_pattern_len(reference.encoding());
                    if projection.field_offset + pattern_len > projection.stride {
                        collection.skipped_count += 1;
                        continue;
                    }
                    let row_start = offset - projection.field_offset;
                    let Some(row_end) = row_start.checked_add(projection.stride) else {
                        collection.skipped_count += 1;
                        continue;
                    };
                    let Some(row) = target_stream.get(row_start..row_end) else {
                        collection.skipped_count += 1;
                        continue;
                    };

                    if !source_reported {
                        collection.source_count += 1;
                        source_reported = true;
                    }
                    collection.records.push(ObjectFrameReferenceRecord {
                        source_path: candidate.path().to_string(),
                        embedding_index,
                        target_path: reference.target_path().to_string(),
                        encoding: reference.encoding().to_string(),
                        stride: projection.stride,
                        field_offset: projection.field_offset,
                        offset,
                        row_index: offset / projection.stride,
                        row_start,
                        candidate: projection.name(),
                        row: row.to_vec(),
                    });
                }
            }
        }
    }

    Ok(collection)
}

fn summarize_object_image_frame_candidate(
    candidate: &ModelObjectStreamCandidate,
) -> ObjectImageFrameCandidateSummary {
    let mut summary = ObjectImageFrameCandidateSummary {
        embedding_index: candidate
            .ownership_candidate()
            .and_then(|ownership| ownership.embedding_index()),
        ..ObjectImageFrameCandidateSummary::default()
    };

    for span in candidate.image_payload_spans() {
        summary.payload_kinds.insert(span.kind().to_string());
    }

    for row in candidate.frame_reference_row_candidates() {
        summary.frame_rows += 1;
        *summary
            .family_counts
            .entry(row.family().to_string())
            .or_default() += 1;

        if row.encoding() == "u16-be"
            && row.stride() == 12
            && row.field_offset() == 7
            && row.family() == "frame-index-tail-coordinate-row12"
        {
            summary.row12_tail_coordinate += 1;
            if let Some(pair) = object_frame_coordinate_pair(row) {
                summary.coordinate_pairs.push(pair);
            }
        } else if row.encoding() == "u16-be"
            && row.stride() == 12
            && row.field_offset() == 7
            && row.family() == "frame-index-tail-zero-row12"
        {
            summary.row12_tail_zero += 1;
        } else if row.encoding() == "u16-be"
            && row.stride() == 20
            && row.field_offset() == 15
            && row.family() == "frame-index-tail-window20"
        {
            summary.row20_tail_window += 1;
            if row.suffix_link().is_some() {
                summary.row20_linked += 1;
            }
        } else if row.encoding() == "u16-le" && row.stride() == 12 && row.field_offset() == 5 {
            summary.le_row12 += 1;
        }
    }

    summary.preferred = preferred_object_image_frame_candidate(&summary);
    summary
}

fn preferred_object_image_frame_candidate(
    summary: &ObjectImageFrameCandidateSummary,
) -> &'static str {
    if summary.row12_tail_coordinate > 0 {
        "row12-tail-coordinate"
    } else if summary.row12_tail_zero > 0 {
        "row12-tail-zero"
    } else if summary.row20_tail_window > 0 {
        "row20-tail-window"
    } else if summary.le_row12 > 0 {
        "u16-le-row12"
    } else {
        "none"
    }
}

fn fdm_entry_complete_payload_count(
    candidate: &ModelObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> usize {
    fdm_entry_complete_payload_spans(candidate, entry).len()
}

fn fdm_entry_complete_payload_spans<'a>(
    candidate: &'a ModelObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> Vec<&'a ObjectImagePayloadSpan> {
    candidate
        .image_payload_spans()
        .iter()
        .filter(|span| {
            span.complete()
                && span.signature_offset() >= entry.vector_offset()
                && span.signature_offset() < entry.next_vector_offset()
        })
        .collect()
}

fn fdm_frame_record_for_entry(
    records: &[ObjectFrameRecordCandidate],
    row_index: usize,
) -> Option<&ObjectFrameRecordCandidate> {
    let object_id = u16::try_from(row_index).ok()?;
    records
        .iter()
        .find(|record| record.object_id() == object_id)
}

fn normalize_fdm_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.right()),
        bbox.top().min(bbox.bottom()),
        bbox.left().max(bbox.right()),
        bbox.top().max(bbox.bottom()),
    )
}

fn fdm_bbox_order(bbox: ObjectFdmIndexBbox) -> &'static str {
    match (bbox.left() <= bbox.right(), bbox.top() <= bbox.bottom()) {
        (true, true) => "forward",
        (false, true) => "inverted-x",
        (true, false) => "inverted-y",
        (false, false) => "inverted-xy",
    }
}

fn fdm_bbox_is_plausible(bbox: ObjectFdmIndexBbox) -> bool {
    let normalized = normalize_fdm_bbox(bbox);
    let width = normalized.2.saturating_sub(normalized.0);
    let height = normalized.3.saturating_sub(normalized.1);
    width > 0 && height > 0 && width <= 200_000 && height <= 200_000
}

fn format_model_object_signature_hits(hits: &[ObjectImageSignatureHit]) -> String {
    let hits = hits
        .iter()
        .map(|hit| format!("{}@{}", hit.kind(), hit.offset()))
        .collect::<Vec<_>>();
    if hits.is_empty() {
        "-".to_string()
    } else {
        hits.join(",")
    }
}

fn format_optional_frame_geometry(record: Option<&ObjectFrameRecordCandidate>) -> String {
    record
        .map(|record| {
            format!(
                "{},{},{},{}",
                record.x(),
                record.y(),
                record.width(),
                record.height()
            )
        })
        .unwrap_or_else(|| "-".to_string())
}

fn format_optional_frame_size(record: Option<&ObjectFrameRecordCandidate>) -> String {
    record
        .map(|record| format!("{}x{}", record.width(), record.height()))
        .unwrap_or_else(|| "-".to_string())
}

fn format_fdm_payload_dimensions(spans: &[&ObjectImagePayloadSpan]) -> String {
    let dimensions = spans
        .iter()
        .filter_map(|span| {
            let dimensions = span.dimensions()?;
            Some(format!(
                "{}@{}:{}x{}",
                span.kind(),
                span.signature_offset(),
                dimensions.width(),
                dimensions.height()
            ))
        })
        .collect::<Vec<_>>();
    if dimensions.is_empty() {
        "-".to_string()
    } else {
        dimensions.join(",")
    }
}

fn fdm_payload_dimension_count(spans: &[&ObjectImagePayloadSpan]) -> usize {
    spans
        .iter()
        .filter(|span| span.dimensions().is_some())
        .count()
}

fn best_frame_payload_aspect_delta_permille(
    frame_record: Option<&ObjectFrameRecordCandidate>,
    spans: &[&ObjectImagePayloadSpan],
) -> Option<u64> {
    let frame_record = frame_record?;
    let frame_width = u128::from(frame_record.width());
    let frame_height = u128::from(frame_record.height());
    if frame_width == 0 || frame_height == 0 {
        return None;
    }

    spans
        .iter()
        .filter_map(|span| {
            let dimensions = span.dimensions()?;
            aspect_delta_permille(
                frame_width,
                frame_height,
                u128::from(dimensions.width()),
                u128::from(dimensions.height()),
            )
        })
        .min()
}

fn object_frame_coordinate_pair(
    row: &ObjectFrameReferenceRowCandidate,
) -> Option<ObjectFrameCoordinatePair> {
    let be16 = read_be16_fields(row.row());
    Some(ObjectFrameCoordinatePair {
        row_start: row.row_start(),
        x: *be16.get(2)?,
        y: *be16.get(4)?,
    })
}

fn format_object_frame_coordinate_pairs(pairs: &[ObjectFrameCoordinatePair]) -> String {
    if pairs.is_empty() {
        return "-".to_string();
    }

    pairs
        .iter()
        .take(OBJECT_STREAM_MAX_REPORTED_HITS)
        .map(|pair| format!("{}:{}x{}", pair.row_start, pair.x, pair.y))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_object_payload_dimensions(spans: &[ObjectImagePayloadSpan]) -> String {
    let dimensions = spans
        .iter()
        .filter_map(|span| {
            let dimensions = span.dimensions()?;
            Some(format!(
                "{}@{}:{}x{}",
                span.kind(),
                span.signature_offset(),
                dimensions.width(),
                dimensions.height()
            ))
        })
        .collect::<Vec<_>>();
    if dimensions.is_empty() {
        "-".to_string()
    } else {
        dimensions.join(",")
    }
}

fn object_payload_dimension_count(spans: &[ObjectImagePayloadSpan]) -> usize {
    spans
        .iter()
        .filter(|span| span.dimensions().is_some())
        .count()
}

fn coordinate_payload_aspect_candidate_count(
    pairs: &[ObjectFrameCoordinatePair],
    spans: &[ObjectImagePayloadSpan],
) -> usize {
    let dimensioned_payloads = object_payload_dimension_count(spans);
    let nonzero_pairs = pairs
        .iter()
        .filter(|pair| pair.x != 0 && pair.y != 0)
        .count();
    dimensioned_payloads.saturating_mul(nonzero_pairs)
}

fn best_coordinate_payload_aspect_delta_permille(
    pairs: &[ObjectFrameCoordinatePair],
    spans: &[ObjectImagePayloadSpan],
) -> Option<u64> {
    pairs
        .iter()
        .filter(|pair| pair.x != 0 && pair.y != 0)
        .flat_map(|pair| {
            spans.iter().filter_map(move |span| {
                let dimensions = span.dimensions()?;
                aspect_delta_permille(
                    u128::from(pair.x),
                    u128::from(pair.y),
                    u128::from(dimensions.width()),
                    u128::from(dimensions.height()),
                )
            })
        })
        .min()
}

fn aspect_delta_permille(
    frame_width: u128,
    frame_height: u128,
    image_width: u128,
    image_height: u128,
) -> Option<u64> {
    if frame_width == 0 || frame_height == 0 || image_width == 0 || image_height == 0 {
        return None;
    }
    let left = frame_width.saturating_mul(image_height);
    let right = image_width.saturating_mul(frame_height);
    let denominator = left.max(right);
    if denominator == 0 {
        return None;
    }
    let delta = left.abs_diff(right);
    Some(((delta.saturating_mul(1000)) / denominator) as u64)
}

const FDM_INDEX_HEADER_BYTES: usize = 20;
const FDM_INDEX_ENTRY_BYTES: usize = 22;
const FDM_INDEX_DECLARED_COUNT_OFFSET: usize = 18;
const FDM_INDEX_HEADER_V1: &str = "fdm-index-v1";

fn fdm_vector_path_for_index(index_path: &str) -> Option<String> {
    index_path
        .strip_suffix("/FDMIndex")
        .map(|prefix| format!("{prefix}/FDMVector"))
}

fn fdm_index_declared_count(index_stream: &[u8]) -> Option<usize> {
    read_be16_candidate(index_stream, FDM_INDEX_DECLARED_COUNT_OFFSET).map(usize::from)
}

fn fdm_index_trailing_bytes(index_stream: &[u8]) -> usize {
    index_stream.len().saturating_sub(FDM_INDEX_HEADER_BYTES) % FDM_INDEX_ENTRY_BYTES
}

fn fdm_index_header_family(index_stream: &[u8]) -> &'static str {
    if index_stream.starts_with(&[0x03, 0x0b, 0x00, 0x01]) {
        FDM_INDEX_HEADER_V1
    } else {
        "unknown-header"
    }
}

fn format_fdm_index_header_u16(index_stream: &[u8]) -> String {
    format_be16_hex_fields(&index_stream[..index_stream.len().min(FDM_INDEX_HEADER_BYTES)])
}

fn parse_fdm_index_entries(index_stream: &[u8], vector_len: usize) -> Vec<FdmIndexEntry> {
    if index_stream.len() < FDM_INDEX_HEADER_BYTES {
        return Vec::new();
    }

    let entry_bytes = index_stream.len() - FDM_INDEX_HEADER_BYTES;
    let entry_count = entry_bytes / FDM_INDEX_ENTRY_BYTES;
    let mut entries = Vec::with_capacity(entry_count);
    for row_index in 0..entry_count {
        let index_offset = FDM_INDEX_HEADER_BYTES + row_index * FDM_INDEX_ENTRY_BYTES;
        let Some(vector_offset) = read_be32_at(index_stream, index_offset) else {
            continue;
        };
        let Some(kind) = read_be16_candidate(index_stream, index_offset + 4) else {
            continue;
        };
        let Some(left) = read_i32_be_at(index_stream, index_offset + 6) else {
            continue;
        };
        let Some(top) = read_i32_be_at(index_stream, index_offset + 10) else {
            continue;
        };
        let Some(right) = read_i32_be_at(index_stream, index_offset + 14) else {
            continue;
        };
        let Some(bottom) = read_i32_be_at(index_stream, index_offset + 18) else {
            continue;
        };
        let vector_offset = vector_offset as usize;
        let row = index_stream[index_offset..index_offset + FDM_INDEX_ENTRY_BYTES].to_vec();
        entries.push(FdmIndexEntry {
            row_index,
            index_offset,
            row,
            vector_offset,
            kind,
            left,
            top,
            right,
            bottom,
            valid_vector_offset: vector_offset < vector_len,
        });
    }
    entries
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct FdmIndexEntryStats {
    rows: usize,
    valid_offsets: usize,
    invalid_offsets: usize,
    image_rows: usize,
    image_hits: usize,
    first_invalid_row: Option<usize>,
    first_invalid_offset: Option<usize>,
}

fn fdm_index_entry_stats(
    entries: &[FdmIndexEntry],
    vector_hits: &[ObjectSignatureHit],
    vector_stream: &[u8],
) -> FdmIndexEntryStats {
    let mut stats = FdmIndexEntryStats {
        rows: entries.len(),
        ..FdmIndexEntryStats::default()
    };

    for entry in entries {
        if entry.valid_vector_offset {
            stats.valid_offsets += 1;
        } else {
            stats.invalid_offsets += 1;
            if stats.first_invalid_row.is_none() {
                stats.first_invalid_row = Some(entry.row_index);
                stats.first_invalid_offset = Some(entry.vector_offset);
            }
        }

        let segment = fdm_vector_segment(entry.vector_offset, entries, vector_stream);
        let segment_hits = fdm_segment_signature_hits(vector_hits, segment.start, segment.end);
        if !segment_hits.is_empty() {
            stats.image_rows += 1;
            stats.image_hits += segment_hits.len();
        }
    }

    stats
}

fn fdm_index_shape_family(
    header_family: &str,
    declared_plausible: bool,
    stream_rows: usize,
    trailing_bytes: usize,
    declared_rows: usize,
    all_stats: &FdmIndexEntryStats,
    declared_stats: &FdmIndexEntryStats,
) -> &'static str {
    if header_family != FDM_INDEX_HEADER_V1 {
        return "unknown-header";
    }
    if !declared_plausible {
        return "invalid-declared-count";
    }
    if declared_rows == stream_rows && trailing_bytes == 0 && all_stats.invalid_offsets == 0 {
        return "row22-exact";
    }
    if declared_rows < stream_rows && declared_stats.invalid_offsets == 0 {
        return "row22-count-prefix";
    }
    if declared_stats.invalid_offsets > 0 {
        return "row22-mixed-declared";
    }
    "row22-trailing"
}

fn fdm_index_row_scope(
    row_index: usize,
    declared_plausible: bool,
    declared_entry_count: usize,
) -> &'static str {
    if !declared_plausible {
        "raw"
    } else if row_index < declared_entry_count {
        "declared"
    } else {
        "post-declared"
    }
}

fn fdm_index_row_role(entry: &FdmIndexEntry) -> &'static str {
    if entry.valid_vector_offset {
        "vector-segment"
    } else if fdm_index_row_is_coordinate_like(&entry.row) {
        "coordinate-like-invalid"
    } else {
        "invalid-vector-offset"
    }
}

fn fdm_index_row_is_coordinate_like(row: &[u8]) -> bool {
    let words = read_be16_fields(row);
    if words.len() < FDM_INDEX_ENTRY_BYTES / 2 {
        return false;
    }

    let negative_like_words = words.iter().filter(|word| **word >= 0x8000).count();
    let strongly_negative_words = words.iter().filter(|word| **word >= 0xc000).count();
    let small_positive_words = words
        .iter()
        .filter(|word| **word > 0 && **word <= 0x2000)
        .count();

    negative_like_words >= 3 && strongly_negative_words >= 2 && small_positive_words >= 1
}

fn fdm_vector_segment(
    vector_offset: usize,
    entries: &[FdmIndexEntry],
    vector_stream: &[u8],
) -> FdmVectorSegment {
    let start = vector_offset.min(vector_stream.len());
    let end = entries
        .iter()
        .filter_map(|entry| {
            (entry.vector_offset > vector_offset && entry.vector_offset <= vector_stream.len())
                .then_some(entry.vector_offset)
        })
        .min()
        .unwrap_or(vector_stream.len());
    FdmVectorSegment { start, end }
}

fn fdm_segment_signature_hits(
    vector_hits: &[ObjectSignatureHit],
    start: usize,
    end: usize,
) -> Vec<ObjectSignatureHit> {
    vector_hits
        .iter()
        .filter(|hit| hit.offset >= start && hit.offset < end)
        .map(|hit| ObjectSignatureHit {
            kind: hit.kind,
            offset: hit.offset,
        })
        .collect()
}

fn fdm_relative_signature_hits(
    segment_hits: &[ObjectSignatureHit],
    segment_start: usize,
) -> Vec<ObjectSignatureHit> {
    segment_hits
        .iter()
        .map(|hit| ObjectSignatureHit {
            kind: hit.kind,
            offset: hit.offset.saturating_sub(segment_start),
        })
        .collect()
}

fn classify_object_frame_reference_record(record: &ObjectFrameReferenceRecord) -> &'static str {
    let be16 = read_be16_fields(&record.row);

    match (record.encoding.as_str(), record.stride, record.field_offset) {
        ("u16-le", 12, 5)
            if be16.len() == 6
                && be16[1] == 0
                && be16[3] == 0
                && be16[4] <= 0x0010
                && be16[5] <= 0x0010 =>
        {
            "frame-index-flag-row12"
        }
        ("u16-le", 12, 5) => "frame-index-mixed-row12",
        ("u16-be", 12, 7)
            if be16.len() == 6
                && be16[0] == 0
                && be16[1] == 0
                && be16[2] == 0
                && be16[3] == 0
                && be16[5] == 0 =>
        {
            "frame-index-tail-zero-row12"
        }
        ("u16-be", 12, 7) if be16.len() == 6 && be16[1] == 0 && be16[3] == 0 && be16[5] == 0 => {
            "frame-index-tail-coordinate-row12"
        }
        ("u16-be", 12, 7) => "frame-index-tail-mixed-row12",
        ("u16-be", 20, 15) if be16.len() == 10 && be16[9] == 0 => "frame-index-tail-window20",
        ("u16-be", 20, 15) => "frame-index-mixed-window20",
        _ => "frame-index-unknown",
    }
}

fn object_frame_row_suffix(record: &ObjectFrameReferenceRecord, len: usize) -> Option<&[u8]> {
    record.row.get(record.row.len().checked_sub(len)?..)
}

fn object_frame_row_prefix(
    record: &ObjectFrameReferenceRecord,
    suffix_len: usize,
) -> Option<&[u8]> {
    record.row.get(..record.row.len().checked_sub(suffix_len)?)
}

fn find_object_frame_suffix_match<'a>(
    record: &ObjectFrameReferenceRecord,
    suffix: &[u8],
    row12_records: &[&'a ObjectFrameReferenceRecord],
) -> (&'static str, Option<&'a ObjectFrameReferenceRecord>) {
    if suffix.is_empty() {
        return ("none", None);
    }

    if let Some(matched) = row12_records
        .iter()
        .copied()
        .find(|candidate| candidate.source_path == record.source_path && candidate.row == suffix)
    {
        return ("same-source", Some(matched));
    }

    if let Some(matched) = row12_records.iter().copied().find(|candidate| {
        candidate.embedding_index == record.embedding_index && candidate.row == suffix
    }) {
        return ("same-embedding", Some(matched));
    }

    if let Some(matched) = row12_records
        .iter()
        .copied()
        .find(|candidate| candidate.row == suffix)
    {
        return ("global", Some(matched));
    }

    ("none", None)
}

fn readable_cfb_streams(data: &[u8]) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let entries = inspect_cfb_entries(data).map_err(|error| error.to_string())?;
    let mut streams = BTreeMap::new();
    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        if let Ok(stream) = read_cfb_stream(data, entry.path()) {
            streams.insert(entry.path().to_string(), stream);
        }
    }
    Ok(streams)
}

fn object_reference_pattern_len(encoding: &str) -> usize {
    match encoding {
        "u16-le" | "u16-be" => 2,
        "u32-le" | "u32-be" => 4,
        _ => 1,
    }
}

fn object_reference_context(
    stream: &[u8],
    offset: usize,
    pattern_len: usize,
) -> ObjectReferenceContext {
    let start = offset.saturating_sub(OBJECT_REFERENCE_CONTEXT_BEFORE_BYTES);
    let end = stream.len().min(
        offset
            .saturating_add(pattern_len)
            .saturating_add(OBJECT_REFERENCE_CONTEXT_AFTER_BYTES),
    );
    ObjectReferenceContext {
        start,
        hex: bytes_to_hex(stream.get(start..end).unwrap_or_default()),
    }
}

fn read_le16_candidate(bytes: &[u8], offset: usize) -> Option<u16> {
    let bytes = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_be16_candidate(bytes: &[u8], offset: usize) -> Option<u16> {
    let bytes = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn read_le32_candidate(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_be32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_i32_be_at(bytes: &[u8], offset: usize) -> Option<i32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn format_usize_set(values: &BTreeSet<usize>) -> String {
    if values.is_empty() {
        return "-".to_string();
    }
    values
        .iter()
        .take(OBJECT_STREAM_MAX_REPORTED_HITS)
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn format_string_set(values: &BTreeSet<String>) -> String {
    if values.is_empty() {
        return "-".to_string();
    }

    let mut formatted = values
        .iter()
        .take(OBJECT_STREAM_MAX_REPORTED_HITS)
        .cloned()
        .collect::<Vec<_>>();
    if values.len() > OBJECT_STREAM_MAX_REPORTED_HITS {
        formatted.push(format!(
            "+{}",
            values.len() - OBJECT_STREAM_MAX_REPORTED_HITS
        ));
    }
    formatted.join(",")
}

fn format_frame_reference_record_candidates() -> String {
    OBJECT_FRAME_REFERENCE_RECORD_CANDIDATES
        .iter()
        .map(ObjectFrameReferenceRecordCandidate::name)
        .collect::<Vec<_>>()
        .join(",")
}

fn document_text_map_meta(entry: &rjtd_core::document_text::DocumentTextMapEntry) -> String {
    match (entry.selector(), entry.code()) {
        (Some(selector), _) => format!("0x{selector:04x}"),
        (_, Some(code)) => format!("0x{code:04x}"),
        (None, None) => "-".to_string(),
    }
}

fn format_byte_context(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
) -> String {
    if let Some(entry) = entries
        .iter()
        .find(|entry| entry.contains_byte_offset(offset))
    {
        return format!("hit:{}", summarize_map_entry(entry));
    }

    format_between_context(
        entries
            .iter()
            .filter(|entry| entry.byte_end() <= offset)
            .max_by_key(|entry| entry.byte_end()),
        entries
            .iter()
            .filter(|entry| entry.byte_start() >= offset)
            .min_by_key(|entry| entry.byte_start()),
    )
}

fn format_unit_context(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
) -> String {
    if let Some(entry) = unit_hit(entries, offset) {
        return format!("hit:{}", summarize_map_entry(entry));
    }

    format_between_context(
        entries
            .iter()
            .filter(|entry| entry.unit_end() <= offset)
            .max_by_key(|entry| entry.unit_end()),
        entries
            .iter()
            .filter(|entry| entry.unit_start() >= offset)
            .min_by_key(|entry| entry.unit_start()),
    )
}

fn format_optional_byte_context(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: Option<u16>,
) -> String {
    offset
        .map(|offset| format_byte_context(entries, offset as usize))
        .unwrap_or_else(|| "-".to_string())
}

fn format_optional_unit_context(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: Option<u16>,
) -> String {
    offset
        .map(|offset| format_unit_context(entries, offset as usize))
        .unwrap_or_else(|| "-".to_string())
}

fn format_optional_unit_context_with_delta(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: Option<u16>,
    delta: usize,
) -> String {
    offset
        .map(|offset| format_unit_context(entries, (offset as usize).saturating_add(delta)))
        .unwrap_or_else(|| "-".to_string())
}

fn unit_hit(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
) -> Option<&rjtd_core::document_text::DocumentTextMapEntry> {
    entries
        .iter()
        .find(|entry| entry.contains_unit_offset(offset))
}

fn unit_text_hit(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
) -> Option<&rjtd_core::document_text::DocumentTextMapEntry> {
    unit_hit(entries, offset).filter(|entry| entry.kind().as_str() == "text")
}

fn format_between_context(
    previous: Option<&rjtd_core::document_text::DocumentTextMapEntry>,
    next: Option<&rjtd_core::document_text::DocumentTextMapEntry>,
) -> String {
    format!(
        "between:{}|{}",
        previous
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string()),
        next.map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string())
    )
}

fn summarize_map_entry(entry: &rjtd_core::document_text::DocumentTextMapEntry) -> String {
    format!(
        "{}({})@{}-{}/{}-{}:{}",
        entry.kind().as_str(),
        document_text_map_meta(entry),
        entry.byte_start(),
        entry.byte_end(),
        entry.unit_start(),
        entry.unit_end(),
        escaped_text_preview(entry.text(), 40)
    )
}

fn format_map_entry_at(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    index: Option<usize>,
) -> String {
    index
        .and_then(|index| entries.get(index))
        .map(summarize_map_entry)
        .unwrap_or_else(|| "-".to_string())
}

fn format_nearest_control_entry(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    index: usize,
    after: bool,
) -> String {
    let found = if after {
        entries
            .iter()
            .enumerate()
            .skip(index.saturating_add(1))
            .find(|(_, entry)| entry.kind().as_str() == "control")
    } else {
        entries
            .iter()
            .enumerate()
            .take(index)
            .rev()
            .find(|(_, entry)| entry.kind().as_str() == "control")
    };

    found
        .and_then(|(control_index, entry)| {
            Some(format!(
                "0x{:04x}@{},d={},byte={},unit={}",
                entry.code()?,
                control_index,
                control_index as isize - index as isize,
                entry.byte_start(),
                entry.unit_start()
            ))
        })
        .unwrap_or_else(|| "-".to_string())
}

fn format_control_code_sequence(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
) -> String {
    entries
        .iter()
        .map(|entry| {
            entry
                .code()
                .map(|code| format!("0x{code:04x}"))
                .unwrap_or_else(|| "-".to_string())
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn format_control_range_delimiter(filter: Option<u16>) -> String {
    filter
        .map(|code| format!("0x{code:04x}"))
        .unwrap_or_else(|| "all".to_string())
}

fn format_control_range_boundary(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    index: Option<usize>,
    edge_label: &str,
) -> String {
    let Some(index) = index else {
        return edge_label.to_string();
    };
    let Some(entry) = entries.get(index) else {
        return "-".to_string();
    };

    entry
        .code()
        .map(|code| {
            format!(
                "0x{code:04x}@{index},byte={},unit={}",
                entry.byte_start(),
                entry.unit_start()
            )
        })
        .unwrap_or_else(|| format!("{index}:{}", summarize_map_entry(entry)))
}

fn format_entry_index_span(start: usize, end: usize) -> String {
    if start >= end {
        "-".to_string()
    } else {
        format!("{start}-{}", end - 1)
    }
}

fn format_control_range_contents(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
) -> String {
    let mut text_count = 0usize;
    let mut inline_count = 0usize;
    let mut skipped_count = 0usize;
    let mut control_count = 0usize;
    let mut preview = String::new();

    for entry in entries {
        match entry.kind().as_str() {
            "text" => text_count += 1,
            "inline" => inline_count += 1,
            "skipped-inline" => skipped_count += 1,
            "control" => control_count += 1,
            _ => {}
        }

        if entry.kind().as_str() != "control" {
            preview.push_str(entry.text());
        }
    }

    let controls = format_range_control_counts(entries.iter());
    let preview = if preview.is_empty() {
        "-".to_string()
    } else {
        escaped_text_preview(&preview, 80)
    };

    format!(
        "entries={},text={text_count},inline={inline_count},skipped={skipped_count},control={control_count},controls={controls},preview={preview}",
        entries.len()
    )
}

fn format_byte_range_preview(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    format_range_preview(entries, start, end, |entry| {
        (entry.byte_start(), entry.byte_end())
    })
}

fn format_unit_range_preview(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    format_range_preview(entries, start, end, |entry| {
        (entry.unit_start(), entry.unit_end())
    })
}

fn format_range_preview(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    bounds: impl Fn(&rjtd_core::document_text::DocumentTextMapEntry) -> (usize, usize),
) -> String {
    let mut entry_count = 0usize;
    let mut text_count = 0usize;
    let mut inline_count = 0usize;
    let mut skipped_count = 0usize;
    let mut control_count = 0usize;
    let mut preview = String::new();

    if start < end {
        for entry in entries {
            let (entry_start, entry_end) = bounds(entry);
            if entry_start >= end || entry_end <= start {
                continue;
            }

            entry_count += 1;
            match entry.kind().as_str() {
                "text" => text_count += 1,
                "inline" => inline_count += 1,
                "skipped-inline" => skipped_count += 1,
                "control" => control_count += 1,
                _ => {}
            }

            if entry.kind().as_str() != "control" {
                preview.push_str(entry.text());
            }
        }
    }

    let preview = if preview.is_empty() {
        "-".to_string()
    } else {
        escaped_text_preview(&preview, 80)
    };
    format!(
        "entries={entry_count},text={text_count},inline={inline_count},skipped={skipped_count},control={control_count},preview={preview}"
    )
}

fn format_boundary_candidate_interval_kind(interval_count: usize) -> &'static str {
    if interval_count == 1 {
        "single"
    } else {
        "multi"
    }
}

fn format_table_candidate_intervals(candidate: &TableCandidate) -> String {
    if candidate.intervals().is_empty() {
        return "-".to_string();
    }

    candidate
        .intervals()
        .iter()
        .map(|interval| {
            format!(
                "{}:source-interval={},source={}-{},line-breaks={},text={}",
                interval.index(),
                interval.source_interval_index(),
                interval.source_start(),
                interval.source_end(),
                interval.line_break_count(),
                escaped_text(interval.text_preview())
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn format_table_candidate_text_shape(
    candidate: &TableCandidate,
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    basis: RangeBasis,
) -> String {
    let mut non_empty = 0usize;
    let mut empty = 0usize;
    let mut total_chars = 0usize;
    let mut min_chars: Option<usize> = None;
    let mut max_chars: Option<usize> = None;
    let mut total_line_breaks = 0usize;

    for interval in candidate.intervals() {
        let text = range_visible_text(
            entries,
            interval.source_start(),
            interval.source_end(),
            basis,
        );
        let chars = text.chars().count();
        let line_breaks = text_line_break_count(&text);
        if chars == 0 {
            empty += 1;
        } else {
            non_empty += 1;
        }
        total_chars += chars;
        total_line_breaks += line_breaks;
        min_chars = Some(min_chars.map_or(chars, |value| value.min(chars)));
        max_chars = Some(max_chars.map_or(chars, |value| value.max(chars)));
    }

    format!(
        "non-empty={non_empty},empty={empty},min-chars={},max-chars={},total-chars={total_chars},line-breaks={total_line_breaks},cell-like={}",
        min_chars
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string()),
        max_chars
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string()),
        if non_empty > 1 && empty == 0 && total_line_breaks == 0 {
            "true"
        } else {
            "false"
        }
    )
}

fn is_table_candidate_cell_like(
    candidate: &TableCandidate,
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    basis: RangeBasis,
) -> bool {
    let mut non_empty = 0usize;
    let mut empty = 0usize;
    let mut line_breaks = 0usize;

    for interval in candidate.intervals() {
        let text = range_visible_text(
            entries,
            interval.source_start(),
            interval.source_end(),
            basis,
        );
        if text.is_empty() {
            empty += 1;
        } else {
            non_empty += 1;
        }
        line_breaks += text_line_break_count(&text);
    }

    non_empty > 1 && empty == 0 && line_breaks == 0
}

fn format_table_candidate_interval_texts(
    candidate: &TableCandidate,
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    basis: RangeBasis,
) -> String {
    if candidate.intervals().is_empty() {
        return "-".to_string();
    }

    candidate
        .intervals()
        .iter()
        .map(|interval| {
            let text = range_visible_text(
                entries,
                interval.source_start(),
                interval.source_end(),
                basis,
            );
            format!(
                "{}:source-interval={},source={}-{},chars={},text={}",
                interval.index(),
                interval.source_interval_index(),
                interval.source_start(),
                interval.source_end(),
                text.chars().count(),
                escaped_text_preview(&text, 80)
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn format_table_candidate_interval_column_segments(candidate: &TableCandidate) -> String {
    let interval_segments = candidate
        .intervals()
        .iter()
        .filter(|interval| !interval.column_segments().is_empty())
        .map(|interval| {
            let segments = interval
                .column_segments()
                .iter()
                .map(|segment| {
                    format!(
                        "{}:{}:{}-{}:{}",
                        segment.index(),
                        segment.kind().as_str(),
                        segment.char_start(),
                        segment.char_end(),
                        escaped_text(segment.text())
                    )
                })
                .collect::<Vec<_>>()
                .join("|");
            format!("{}={segments}", interval.index())
        })
        .collect::<Vec<_>>();

    if interval_segments.is_empty() {
        "-".to_string()
    } else {
        interval_segments.join(";")
    }
}

fn format_table_candidate_column_grid_shape(candidate: &TableCandidate) -> String {
    candidate
        .column_segment_grid_candidate()
        .map(|grid| format!("{}x{}", grid.row_count(), grid.column_count()))
        .unwrap_or_else(|| "-".to_string())
}

fn format_table_candidate_column_grid_pattern(candidate: &TableCandidate) -> String {
    candidate
        .column_segment_grid_candidate()
        .map(|grid| {
            grid.pattern()
                .iter()
                .map(|kind| kind.as_str())
                .collect::<Vec<_>>()
                .join("|")
        })
        .unwrap_or_else(|| "-".to_string())
}

fn format_table_candidate_interval_contexts(
    candidate: &TableCandidate,
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    basis: RangeBasis,
) -> String {
    if candidate.intervals().is_empty() {
        return "-".to_string();
    }

    candidate
        .intervals()
        .iter()
        .map(|interval| {
            let text = range_visible_text(
                entries,
                interval.source_start(),
                interval.source_end(),
                basis,
            );
            format!(
                "{}:source-interval={},source={}-{},chars={},line-breaks={},text={},edges={}",
                interval.index(),
                interval.source_interval_index(),
                interval.source_start(),
                interval.source_end(),
                text.chars().count(),
                text_line_break_count(&text),
                escaped_text_preview(&text, 80),
                format_candidate_range_boundaries(
                    entries,
                    interval.source_start(),
                    interval.source_end(),
                    basis
                )
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn range_basis_from_candidate(basis: &str) -> RangeBasis {
    match basis {
        "byte" => RangeBasis::Byte,
        "unit" => RangeBasis::Unit,
        _ => unreachable!("unexpected text boundary candidate basis"),
    }
}

fn format_candidate_range_preview(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    match basis {
        RangeBasis::Byte => format_byte_range_preview(entries, start, end),
        RangeBasis::Unit => format_unit_range_preview(entries, start, end),
    }
}

fn format_candidate_range_boundaries(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    match basis {
        RangeBasis::Byte => format_byte_range_boundaries(entries, start, end),
        RangeBasis::Unit => format_unit_range_boundaries(entries, start, end),
    }
}

fn range_line_break_count(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> usize {
    text_line_break_count(&range_visible_text(entries, start, end, basis))
}

fn text_line_break_count(text: &str) -> usize {
    text.chars()
        .filter(|character| matches!(character, '\n' | '\r'))
        .count()
}

fn range_visible_text(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    entries
        .iter()
        .filter(|entry| range_overlaps_entry(entry, start, end, basis))
        .map(|entry| range_text_overlap(entry, start, end, basis))
        .collect()
}

fn is_boundary_candidate_edge_good(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> bool {
    range_starts_after_control_gap(entries, start, basis)
        && range_ends_on_aligned_text(entries, end, basis)
}

fn is_strict_unit_paragraph_candidate(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> bool {
    let text = range_visible_text(entries, start, end, RangeBasis::Unit);
    is_boundary_candidate_edge_good(entries, start, end, RangeBasis::Unit)
        && !text.is_empty()
        && text_line_break_count(&text) <= 1
}

fn collect_unit_001c_single_layout_candidates(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    candidates: &[TextBoundaryCandidate],
) -> Vec<LayoutBoundaryCandidate> {
    candidates
        .iter()
        .filter(|candidate| {
            candidate.basis().as_str() == "unit"
                && candidate.delimiter_code() == 0x001c
                && candidate.interval_count() == 1
        })
        .map(|candidate| {
            let selected = is_strict_unit_paragraph_candidate(
                entries,
                candidate.source_start(),
                candidate.source_end(),
            );
            LayoutBoundaryCandidate::new(
                candidate.index(),
                candidate.text_count_range_index(),
                candidate.source_start(),
                candidate.source_end(),
                selected,
            )
        })
        .collect()
}

#[derive(Clone, Copy)]
struct LayoutBoundaryCandidate {
    index: usize,
    text_count_range_index: usize,
    source_start: usize,
    source_end: usize,
    selected: bool,
}

impl LayoutBoundaryCandidate {
    fn new(
        index: usize,
        text_count_range_index: usize,
        source_start: usize,
        source_end: usize,
        selected: bool,
    ) -> Self {
        Self {
            index,
            text_count_range_index,
            source_start,
            source_end,
            selected,
        }
    }
}

#[derive(Clone, Copy)]
enum LayoutMapBase {
    Unit,
    UnitTimes2,
    UnitDiv2Floor,
    UnitDiv2Ceil,
}

impl LayoutMapBase {
    fn name(self) -> &'static str {
        match self {
            Self::Unit => "unit",
            Self::UnitTimes2 => "unit-times-2",
            Self::UnitDiv2Floor => "unit-div2-floor",
            Self::UnitDiv2Ceil => "unit-div2-ceil",
        }
    }

    fn apply(self, value: usize) -> i64 {
        match self {
            Self::Unit => value as i64,
            Self::UnitTimes2 => (value as i64) * 2,
            Self::UnitDiv2Floor => (value / 2) as i64,
            Self::UnitDiv2Ceil => value.div_ceil(2) as i64,
        }
    }
}

#[derive(Clone)]
struct LayoutMapTargetSet {
    name: &'static str,
    points: Vec<usize>,
}

impl LayoutMapTargetSet {
    fn new(name: &'static str, points: impl IntoIterator<Item = usize>) -> Self {
        Self {
            name,
            points: sorted_unique_usize(points),
        }
    }
}

#[derive(Clone, Copy)]
struct LayoutMapScore {
    candidates: usize,
    endpoints: usize,
    valid_endpoints: usize,
    exact_hits: usize,
    invalid_endpoints: usize,
    total_distance: Option<usize>,
    max_distance: Option<usize>,
}

#[derive(Clone)]
struct LayoutExactEvidence {
    target: &'static str,
    base: LayoutMapBase,
    delta: isize,
    start: String,
    end: String,
}

struct LayoutParagraphLikeEvidence {
    paragraph_like: bool,
    line_word_evidence: Option<LayoutExactEvidence>,
    page_field_evidence: Option<LayoutExactEvidence>,
}

#[derive(Default)]
struct ParagraphLikeBucketSummary {
    rows: usize,
    strict_selected: usize,
    line_word_exact2: usize,
    page_field_exact2: usize,
    dual_exact2: usize,
    text_style_hits: usize,
    page_style_hits: usize,
    view_style_group_hits: usize,
    missing_tcnt: usize,
    source_span_min: Option<usize>,
    source_span_max: Option<usize>,
    range_span_min: Option<usize>,
    range_span_max: Option<usize>,
    family_counts: BTreeMap<String, usize>,
    f0_counts: BTreeMap<u16, usize>,
    f4_counts: BTreeMap<u16, usize>,
    f7_counts: BTreeMap<u16, usize>,
    line_evidence_counts: BTreeMap<String, usize>,
    page_evidence_counts: BTreeMap<String, usize>,
}

impl ParagraphLikeBucketSummary {
    fn observe(
        &mut self,
        candidate: &LayoutBoundaryCandidate,
        evidence: &LayoutParagraphLikeEvidence,
        range: Option<&TextCountRange>,
        text_style_candidates: &[CliStyleCandidate],
        page_style_candidates: &[CliStyleCandidate],
        view_style_groups: &[DocumentViewStyleGroup],
    ) {
        self.rows += 1;
        if candidate.selected {
            self.strict_selected += 1;
        }
        if evidence.line_word_evidence.is_some() {
            self.line_word_exact2 += 1;
        }
        if evidence.page_field_evidence.is_some() {
            self.page_field_exact2 += 1;
        }
        if evidence.line_word_evidence.is_some() && evidence.page_field_evidence.is_some() {
            self.dual_exact2 += 1;
        }
        update_min_max(
            &mut self.source_span_min,
            &mut self.source_span_max,
            candidate.source_end.saturating_sub(candidate.source_start),
        );
        if let Some(evidence) = evidence.line_word_evidence.as_ref() {
            *self
                .line_evidence_counts
                .entry(format_layout_evidence_signature(evidence))
                .or_insert(0) += 1;
        }
        if let Some(evidence) = evidence.page_field_evidence.as_ref() {
            *self
                .page_evidence_counts
                .entry(format_layout_evidence_signature(evidence))
                .or_insert(0) += 1;
        }

        let Some(range) = range else {
            self.missing_tcnt += 1;
            return;
        };
        *self
            .family_counts
            .entry(range.family().to_string())
            .or_insert(0) += 1;
        update_min_max(
            &mut self.range_span_min,
            &mut self.range_span_max,
            range.span() as usize,
        );
        let tail_fields = range.tail_fields();
        count_tail_field(&mut self.f0_counts, tail_fields, 0);
        count_tail_field(&mut self.f4_counts, tail_fields, 4);
        count_tail_field(&mut self.f7_counts, tail_fields, 7);
        if has_style_hit(tail_fields, text_style_candidates) {
            self.text_style_hits += 1;
        }
        if has_style_hit(tail_fields, page_style_candidates) {
            self.page_style_hits += 1;
        }
        if has_view_style_group_hit(tail_fields, view_style_groups) {
            self.view_style_group_hits += 1;
        }
    }

    fn format_fields(&self) -> String {
        format!(
            "rows={}\tstrict-selected={}\tline-word-exact2={}\tpage-field-exact2={}\tdual-exact2={}\ttext-style-hit={}\tpage-style-hit={}\tview-style-group-hit={}\tmissing-tcnt={}\tsource-spans={}\trange-spans={}\tfamilies={}\tf0={}\tf4={}\tf7={}\tline-evidence={}\tpage-evidence={}\tdecoded=false",
            self.rows,
            self.strict_selected,
            self.line_word_exact2,
            self.page_field_exact2,
            self.dual_exact2,
            self.text_style_hits,
            self.page_style_hits,
            self.view_style_group_hits,
            self.missing_tcnt,
            format_min_max(self.source_span_min, self.source_span_max),
            format_min_max(self.range_span_min, self.range_span_max),
            format_string_counts(&self.family_counts),
            format_u16_value_counts(&self.f0_counts),
            format_u16_value_counts(&self.f4_counts),
            format_u16_value_counts(&self.f7_counts),
            format_string_counts(&self.line_evidence_counts),
            format_string_counts(&self.page_evidence_counts),
        )
    }
}

fn layout_map_bases() -> &'static [LayoutMapBase] {
    &[
        LayoutMapBase::Unit,
        LayoutMapBase::UnitTimes2,
        LayoutMapBase::UnitDiv2Floor,
        LayoutMapBase::UnitDiv2Ceil,
    ]
}

fn layout_map_target_sets(
    line_words: Option<&[u16]>,
    page_mark: Option<&PageMark>,
    paper_mark: Option<&PaperMark>,
) -> Vec<LayoutMapTargetSet> {
    vec![
        LayoutMapTargetSet::new(
            "line-tag-index",
            line_words
                .into_iter()
                .flat_map(|words| {
                    words
                        .iter()
                        .enumerate()
                        .filter(|(_, word)| is_line_mark_tag(**word))
                        .map(|(index, _)| index)
                })
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "line-tag-byte",
            line_words
                .into_iter()
                .flat_map(|words| {
                    words
                        .iter()
                        .enumerate()
                        .filter(|(_, word)| is_line_mark_tag(**word))
                        .map(|(index, _)| index * 2)
                })
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "line-word-value",
            line_words
                .into_iter()
                .flat_map(|words| words.iter().map(|word| *word as usize))
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "page-entry-index",
            page_mark
                .into_iter()
                .flat_map(|mark| mark.entries().iter().filter_map(|entry| entry.index()))
                .map(|value| value as usize)
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "page-entry-byte-boundary",
            page_mark
                .map(page_mark_entry_byte_boundaries)
                .unwrap_or_default(),
        ),
        LayoutMapTargetSet::new(
            "page-be32-field",
            page_mark
                .into_iter()
                .flat_map(|mark| {
                    mark.entries().iter().flat_map(|entry| {
                        entry.raw().chunks_exact(4).map(|chunk| {
                            u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as usize
                        })
                    })
                })
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "paper-entry-index",
            paper_mark
                .into_iter()
                .flat_map(|mark| mark.entries().iter().map(|entry| entry.index() as usize))
                .collect::<Vec<_>>(),
        ),
        LayoutMapTargetSet::new(
            "paper-entry-byte-boundary",
            paper_mark
                .map(paper_mark_entry_byte_boundaries)
                .unwrap_or_default(),
        ),
    ]
}

fn page_mark_entry_byte_boundaries(page_mark: &PageMark) -> Vec<usize> {
    let mut offset = 12usize;
    let mut points = vec![offset];
    for entry in page_mark.entries() {
        offset += entry.raw().len();
        points.push(offset);
    }
    points
}

fn paper_mark_entry_byte_boundaries(paper_mark: &PaperMark) -> Vec<usize> {
    let mut offset = 12usize;
    let mut points = vec![offset];
    for _ in paper_mark.entries() {
        offset += 8;
        points.push(offset);
    }
    points
}

fn sorted_unique_usize(points: impl IntoIterator<Item = usize>) -> Vec<usize> {
    let mut points = points.into_iter().collect::<Vec<_>>();
    points.sort_unstable();
    points.dedup();
    points
}

fn write_layout_map_best_rows(
    scope: &str,
    candidates: &[LayoutBoundaryCandidate],
    target_sets: &[LayoutMapTargetSet],
) -> Result<(), String> {
    for target_set in target_sets {
        for base in layout_map_bases() {
            let (delta, score) = best_layout_map_delta(candidates, target_set, *base);
            write_stdout_line(&format!(
                "best\tscope={}\ttarget={}\tbase={}\tdelta={}\tdelta-at-boundary={}\tpoints={}\tcandidates={}\tendpoints={}\tvalid={}\tinvalid={}\texact={}\ttotal-distance={}\tmax-distance={}\tdecoded=false",
                scope,
                target_set.name,
                base.name(),
                delta,
                delta == LAYOUT_MAP_DELTA_MIN || delta == LAYOUT_MAP_DELTA_MAX,
                target_set.points.len(),
                score.candidates,
                score.endpoints,
                score.valid_endpoints,
                score.invalid_endpoints,
                score.exact_hits,
                format_optional_usize(score.total_distance),
                format_optional_usize(score.max_distance),
            ))?;
        }
    }
    Ok(())
}

fn best_layout_exact2_evidence(
    candidate: &LayoutBoundaryCandidate,
    target_sets: &[LayoutMapTargetSet],
    target_name: &'static str,
) -> Option<LayoutExactEvidence> {
    let target_set = target_sets
        .iter()
        .find(|target_set| target_set.name == target_name)?;
    let mut best: Option<LayoutExactEvidence> = None;
    let single = [*candidate];
    for base in layout_map_bases() {
        let (delta, score) = best_layout_map_delta(&single, target_set, *base);
        let at_boundary = delta == LAYOUT_MAP_DELTA_MIN || delta == LAYOUT_MAP_DELTA_MAX;
        if at_boundary || score.exact_hits != 2 || score.total_distance != Some(0) {
            continue;
        }
        let evidence = LayoutExactEvidence {
            target: target_set.name,
            base: *base,
            delta,
            start: format_layout_map_endpoint(candidate.source_start, target_set, *base, delta),
            end: format_layout_map_endpoint(candidate.source_end, target_set, *base, delta),
        };
        let replace = best.as_ref().is_none_or(|best| {
            delta.unsigned_abs() < best.delta.unsigned_abs()
                || (delta.unsigned_abs() == best.delta.unsigned_abs()
                    && base.name() < best.base.name())
        });
        if replace {
            best = Some(evidence);
        }
    }
    best
}

fn layout_paragraph_like_evidence(
    candidate: &LayoutBoundaryCandidate,
    target_sets: &[LayoutMapTargetSet],
) -> LayoutParagraphLikeEvidence {
    let line_word_evidence = best_layout_exact2_evidence(candidate, target_sets, "line-word-value");
    let page_field_evidence =
        best_layout_exact2_evidence(candidate, target_sets, "page-be32-field");
    LayoutParagraphLikeEvidence {
        paragraph_like: candidate.selected
            && line_word_evidence.is_some()
            && page_field_evidence.is_some(),
        line_word_evidence,
        page_field_evidence,
    }
}

fn format_layout_evidence_signature(evidence: &LayoutExactEvidence) -> String {
    format!(
        "{}/{}/{}",
        evidence.target,
        evidence.base.name(),
        evidence.delta
    )
}

fn format_layout_exact_evidence(evidence: Option<&LayoutExactEvidence>) -> String {
    let Some(evidence) = evidence else {
        return "-".to_string();
    };
    format!(
        "{}:{}:{}:{}|{}",
        evidence.target,
        evidence.base.name(),
        evidence.delta,
        evidence.start,
        evidence.end
    )
}

fn format_model_layout_exact_evidence(evidence: &TextLayoutExactEvidence) -> String {
    format!(
        "{}:{}:{}",
        evidence.target(),
        evidence.base(),
        evidence.delta()
    )
}

fn layout_evidence_value(offset: usize, evidence: &TextLayoutExactEvidence) -> Option<usize> {
    let base = match evidence.base() {
        "unit" => offset as i64,
        "unit-times-2" => (offset as i64) * 2,
        "unit-div2-floor" => (offset / 2) as i64,
        "unit-div2-ceil" => offset.div_ceil(2) as i64,
        _ => return None,
    };
    let value = base + evidence.delta() as i64;
    usize::try_from(value).ok()
}

fn format_line_word_value_refs(line_words: Option<&[u16]>, value: Option<usize>) -> String {
    let Some(value) = value else {
        return "value=invalid,hits=0,refs=-".to_string();
    };
    let Some(line_words) = line_words else {
        return format!("value={value},hits=-,refs=missing");
    };
    let hits = line_words
        .iter()
        .enumerate()
        .filter(|(_, word)| **word as usize == value)
        .map(|(index, word)| format!("word{index}:0x{word:04x}"))
        .collect::<Vec<_>>();
    format_limited_hit_refs(value, &hits)
}

fn format_page_be32_field_value_refs(page_mark: Option<&PageMark>, value: Option<usize>) -> String {
    let Some(value) = value else {
        return "value=invalid,hits=0,refs=-".to_string();
    };
    let Some(page_mark) = page_mark else {
        return format!("value={value},hits=-,refs=missing");
    };
    let hits = page_mark
        .entries()
        .iter()
        .enumerate()
        .flat_map(|(row_index, entry)| {
            entry
                .raw()
                .chunks_exact(4)
                .enumerate()
                .filter_map(move |(field_index, chunk)| {
                    let field = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    (field as usize == value)
                        .then(|| format!("row{row_index}:f{field_index}:0x{field:08x}"))
                })
        })
        .collect::<Vec<_>>();
    format_limited_hit_refs(value, &hits)
}

fn format_limited_hit_refs(value: usize, hits: &[String]) -> String {
    if hits.is_empty() {
        return format!("value={value},hits=0,refs=-");
    }
    let mut refs = hits.iter().take(8).cloned().collect::<Vec<_>>();
    if hits.len() > refs.len() {
        refs.push(format!("+{}more", hits.len() - refs.len()));
    }
    format!("value={value},hits={},refs={}", hits.len(), refs.join(","))
}

fn format_layout_map_endpoint(
    offset: usize,
    target_set: &LayoutMapTargetSet,
    base: LayoutMapBase,
    delta: isize,
) -> String {
    let value = base.apply(offset) + delta as i64;
    if value < 0 {
        return format!("{}:{}->invalid", offset, value);
    }
    if target_set.points.is_empty() {
        return format!("{}:{}->missing", offset, value);
    }
    let value = value as usize;
    let (point, distance) = nearest_usize_point(&target_set.points, value);
    format!("{offset}:{value}->{point}:d={distance}")
}

fn format_text_count_range_summary(range: Option<&TextCountRange>) -> String {
    let Some(range) = range else {
        return "-".to_string();
    };
    format!(
        "index={},family={},start={},end={},span={},declared-start={},declared-end={},tail={}",
        range.index(),
        range.family(),
        range.start(),
        range.end(),
        range.span(),
        range.declared_start(),
        range.declared_end(),
        format_u16_values(range.tail_fields()),
    )
}

fn format_u16_values(values: &[u16]) -> String {
    if values.is_empty() {
        "-".to_string()
    } else {
        values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn best_layout_map_delta(
    candidates: &[LayoutBoundaryCandidate],
    target_set: &LayoutMapTargetSet,
    base: LayoutMapBase,
) -> (isize, LayoutMapScore) {
    let mut best_delta = 0isize;
    let mut best_score = score_layout_map_delta(candidates, target_set, base, best_delta);
    if candidates.is_empty() || target_set.points.is_empty() {
        return (best_delta, best_score);
    }

    for delta in LAYOUT_MAP_DELTA_MIN..=LAYOUT_MAP_DELTA_MAX {
        if delta == 0 {
            continue;
        }
        let score = score_layout_map_delta(candidates, target_set, base, delta);
        if is_better_layout_map_score(score, delta, best_score, best_delta) {
            best_delta = delta;
            best_score = score;
        }
    }

    (best_delta, best_score)
}

fn score_layout_map_delta(
    candidates: &[LayoutBoundaryCandidate],
    target_set: &LayoutMapTargetSet,
    base: LayoutMapBase,
    delta: isize,
) -> LayoutMapScore {
    let mut score = LayoutMapScore {
        candidates: candidates.len(),
        endpoints: candidates.len() * 2,
        valid_endpoints: 0,
        exact_hits: 0,
        invalid_endpoints: 0,
        total_distance: None,
        max_distance: None,
    };
    if target_set.points.is_empty() {
        score.invalid_endpoints = score.endpoints;
        return score;
    }

    let mut total_distance = 0usize;
    let mut max_distance = 0usize;
    for candidate in candidates {
        for offset in [candidate.source_start, candidate.source_end] {
            let value = base.apply(offset) + delta as i64;
            if value < 0 {
                score.invalid_endpoints += 1;
                continue;
            }
            score.valid_endpoints += 1;
            let distance = nearest_usize_distance(&target_set.points, value as usize);
            if distance == 0 {
                score.exact_hits += 1;
            }
            total_distance += distance;
            max_distance = max_distance.max(distance);
        }
    }
    if score.valid_endpoints > 0 {
        score.total_distance = Some(total_distance);
        score.max_distance = Some(max_distance);
    }
    score
}

fn nearest_usize_distance(points: &[usize], value: usize) -> usize {
    nearest_usize_point(points, value).1
}

fn nearest_usize_point(points: &[usize], value: usize) -> (usize, usize) {
    match points.binary_search(&value) {
        Ok(index) => (points[index], 0),
        Err(index) => {
            let mut best = (0usize, usize::MAX);
            if let Some(point) = points.get(index) {
                best = (*point, point.abs_diff(value));
            }
            if index > 0 {
                let point = points[index - 1];
                let distance = point.abs_diff(value);
                if distance < best.1 {
                    best = (point, distance);
                }
            }
            best
        }
    }
}

fn is_better_layout_map_score(
    candidate: LayoutMapScore,
    candidate_delta: isize,
    best: LayoutMapScore,
    best_delta: isize,
) -> bool {
    candidate.exact_hits > best.exact_hits
        || (candidate.exact_hits == best.exact_hits
            && (candidate.invalid_endpoints < best.invalid_endpoints
                || (candidate.invalid_endpoints == best.invalid_endpoints
                    && (is_better_optional_distance(
                        candidate.total_distance,
                        best.total_distance,
                    ) || (candidate.total_distance == best.total_distance
                        && (is_better_optional_distance(
                            candidate.max_distance,
                            best.max_distance,
                        ) || (candidate.max_distance == best.max_distance
                            && candidate_delta.unsigned_abs() < best_delta.unsigned_abs())))))))
}

fn is_better_optional_distance(candidate: Option<usize>, best: Option<usize>) -> bool {
    match (candidate, best) {
        (Some(candidate), Some(best)) => candidate < best,
        (Some(_), None) => true,
        _ => false,
    }
}

fn range_starts_after_control_gap(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> bool {
    let touches_entry = entries.iter().any(|entry| {
        let (entry_start, entry_end) = entry_range(entry, basis);
        entry_start == offset || (entry_start < offset && offset < entry_end)
    });
    !touches_entry
        && previous_range_entry(entries, offset, basis)
            .is_some_and(|entry| entry.kind().as_str() == "control")
}

fn range_ends_on_aligned_text(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> bool {
    entries.iter().any(|entry| {
        let (_, entry_end) = entry_range(entry, basis);
        entry_end == offset && entry.kind().as_str() == "text"
    })
}

fn range_text_overlap(
    entry: &rjtd_core::document_text::DocumentTextMapEntry,
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    if entry.kind().as_str() == "control" || start >= end {
        return String::new();
    }

    let (entry_start, entry_end) = entry_range(entry, basis);
    let overlap_start = entry_start.max(start);
    let overlap_end = entry_end.min(end);
    if overlap_start >= overlap_end {
        return String::new();
    }

    let (relative_start, relative_end) = match basis {
        RangeBasis::Byte => (
            overlap_start.saturating_sub(entry.byte_start()) / 2,
            overlap_end
                .saturating_sub(entry.byte_start())
                .saturating_add(1)
                / 2,
        ),
        RangeBasis::Unit => (
            overlap_start.saturating_sub(entry.unit_start()),
            overlap_end.saturating_sub(entry.unit_start()),
        ),
    };
    text_by_utf16_units(entry.text(), relative_start, relative_end)
}

fn text_by_utf16_units(text: &str, start: usize, end: usize) -> String {
    let mut output = String::new();
    let mut current = 0usize;
    for character in text.chars() {
        let next = current + character.len_utf16();
        if next > start && current < end {
            output.push(character);
        }
        current = next;
    }
    output
}

#[derive(Clone, Copy)]
enum RangeBasis {
    Byte,
    Unit,
}

struct ControlDelimitedRange {
    index: usize,
    previous_delimiter: Option<usize>,
    next_delimiter: Option<usize>,
    entry_start: usize,
    entry_end: usize,
    byte_start: usize,
    byte_end: usize,
    unit_start: usize,
    unit_end: usize,
}

fn build_control_delimited_ranges(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    filter: Option<u16>,
) -> Vec<ControlDelimitedRange> {
    let delimiters = entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.kind().as_str() == "control")
        .filter(|(_, entry)| filter.is_none_or(|code| entry.code() == Some(code)))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    (0..=delimiters.len())
        .map(|range_index| {
            let previous_delimiter = range_index
                .checked_sub(1)
                .and_then(|index| delimiters.get(index).copied());
            let next_delimiter = delimiters.get(range_index).copied();
            let entry_start = previous_delimiter.map_or(0, |index| index + 1);
            let entry_end = next_delimiter.unwrap_or(entries.len());
            let range_entries = entries.get(entry_start..entry_end).unwrap_or(&[]);
            let byte_start = previous_delimiter
                .and_then(|index| entries.get(index))
                .map(|entry| entry.byte_end())
                .or_else(|| range_entries.first().map(|entry| entry.byte_start()))
                .or_else(|| {
                    next_delimiter
                        .and_then(|index| entries.get(index))
                        .map(|entry| entry.byte_start())
                })
                .unwrap_or(0);
            let byte_end = next_delimiter
                .and_then(|index| entries.get(index))
                .map(|entry| entry.byte_start())
                .or_else(|| range_entries.last().map(|entry| entry.byte_end()))
                .unwrap_or(byte_start);
            let unit_start = previous_delimiter
                .and_then(|index| entries.get(index))
                .map(|entry| entry.unit_end())
                .or_else(|| range_entries.first().map(|entry| entry.unit_start()))
                .or_else(|| {
                    next_delimiter
                        .and_then(|index| entries.get(index))
                        .map(|entry| entry.unit_start())
                })
                .unwrap_or(0);
            let unit_end = next_delimiter
                .and_then(|index| entries.get(index))
                .map(|entry| entry.unit_start())
                .or_else(|| range_entries.last().map(|entry| entry.unit_end()))
                .unwrap_or(unit_start);

            ControlDelimitedRange {
                index: range_index,
                previous_delimiter,
                next_delimiter,
                entry_start,
                entry_end,
                byte_start,
                byte_end,
                unit_start,
                unit_end,
            }
        })
        .collect()
}

fn format_control_range_hits(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    ranges: &[ControlDelimitedRange],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    let hits = ranges
        .iter()
        .filter(|range| control_range_overlaps(range, start, end, basis))
        .collect::<Vec<_>>();
    let Some(first) = hits.first() else {
        return "count=0,first=-,last=-,byte=-,unit=-,entry-ranges=-,controls=-,preview=-"
            .to_string();
    };
    let first = *first;
    let last = hits.last().copied().unwrap_or(first);
    let controls = format_range_control_counts(
        hits.iter()
            .flat_map(|range| entries[range.entry_start..range.entry_end].iter()),
    );
    let mut preview = String::new();
    for range in &hits {
        for entry in &entries[range.entry_start..range.entry_end] {
            if entry.kind().as_str() != "control" {
                preview.push_str(entry.text());
            }
        }
    }
    let preview = if preview.is_empty() {
        "-".to_string()
    } else {
        escaped_text_preview(&preview, 80)
    };

    format!(
        "count={},first={},last={},byte={}-{},unit={}-{},entry-ranges={},controls={},preview={}",
        hits.len(),
        first.index,
        last.index,
        first.byte_start,
        last.byte_end,
        first.unit_start,
        last.unit_end,
        format_control_range_hit_entry_spans(&hits),
        controls,
        preview
    )
}

fn control_range_overlaps(
    range: &ControlDelimitedRange,
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> bool {
    let (range_start, range_end) = control_range_basis_span(range, basis);
    if start == end {
        return range_start <= start && start <= range_end;
    }

    start < range_end && end > range_start
}

fn control_range_basis_span(range: &ControlDelimitedRange, basis: RangeBasis) -> (usize, usize) {
    match basis {
        RangeBasis::Byte => (range.byte_start, range.byte_end),
        RangeBasis::Unit => (range.unit_start, range.unit_end),
    }
}

fn format_control_range_hit_entry_spans(hits: &[&ControlDelimitedRange]) -> String {
    let spans = hits
        .iter()
        .map(|range| format_entry_index_span(range.entry_start, range.entry_end))
        .filter(|span| span != "-")
        .collect::<Vec<_>>();

    if spans.is_empty() {
        "-".to_string()
    } else {
        spans.join("+")
    }
}

fn format_byte_range_boundaries(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    format_range_boundaries(entries, start, end, RangeBasis::Byte)
}

fn format_unit_range_boundaries(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    format_range_boundaries(entries, start, end, RangeBasis::Unit)
}

fn format_range_boundaries(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> String {
    let overlapping = entries
        .iter()
        .filter(|entry| range_overlaps_entry(entry, start, end, basis))
        .collect::<Vec<_>>();
    let full_count = overlapping
        .iter()
        .filter(|entry| {
            let (entry_start, entry_end) = entry_range(entry, basis);
            start <= entry_start && entry_end <= end
        })
        .count();
    let controls = format_range_control_counts(overlapping.iter().copied());
    let first = overlapping
        .first()
        .map(|entry| summarize_map_entry(entry))
        .unwrap_or_else(|| "-".to_string());
    let last = overlapping
        .last()
        .map(|entry| summarize_map_entry(entry))
        .unwrap_or_else(|| "-".to_string());
    let previous = previous_range_entry(entries, start, basis)
        .map(summarize_map_entry)
        .unwrap_or_else(|| "-".to_string());
    let next = next_range_entry(entries, end, basis)
        .map(summarize_map_entry)
        .unwrap_or_else(|| "-".to_string());

    format!(
        "inside={},full={},partial={},start-edge={},end-edge={},first={},last={},prev={},next={},controls={}",
        overlapping.len(),
        full_count,
        overlapping.len().saturating_sub(full_count),
        format_range_start_edge(entries, start, basis),
        format_range_end_edge(entries, end, basis),
        first,
        last,
        previous,
        next,
        controls
    )
}

fn entry_range(
    entry: &rjtd_core::document_text::DocumentTextMapEntry,
    basis: RangeBasis,
) -> (usize, usize) {
    match basis {
        RangeBasis::Byte => (entry.byte_start(), entry.byte_end()),
        RangeBasis::Unit => (entry.unit_start(), entry.unit_end()),
    }
}

fn range_overlaps_entry(
    entry: &rjtd_core::document_text::DocumentTextMapEntry,
    start: usize,
    end: usize,
    basis: RangeBasis,
) -> bool {
    if start >= end {
        return false;
    }
    let (entry_start, entry_end) = entry_range(entry, basis);
    entry_start < end && entry_end > start
}

fn previous_range_entry(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> Option<&rjtd_core::document_text::DocumentTextMapEntry> {
    entries
        .iter()
        .filter(|entry| entry_range(entry, basis).1 <= offset)
        .max_by_key(|entry| entry_range(entry, basis).1)
}

fn next_range_entry(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> Option<&rjtd_core::document_text::DocumentTextMapEntry> {
    entries
        .iter()
        .filter(|entry| entry_range(entry, basis).0 >= offset)
        .min_by_key(|entry| entry_range(entry, basis).0)
}

fn format_range_start_edge(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> String {
    if let Some(entry) = entries
        .iter()
        .find(|entry| entry_range(entry, basis).0 == offset)
    {
        return format!("aligned:{}", summarize_map_entry(entry));
    }

    if let Some(entry) = entries.iter().find(|entry| {
        let (entry_start, entry_end) = entry_range(entry, basis);
        entry_start < offset && offset < entry_end
    }) {
        return format!("inside:{}", summarize_map_entry(entry));
    }

    format!(
        "gap:{}|{}",
        previous_range_entry(entries, offset, basis)
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string()),
        next_range_entry(entries, offset, basis)
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string())
    )
}

fn format_range_end_edge(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
    basis: RangeBasis,
) -> String {
    if let Some(entry) = entries
        .iter()
        .find(|entry| entry_range(entry, basis).1 == offset)
    {
        return format!("aligned:{}", summarize_map_entry(entry));
    }

    if let Some(entry) = entries.iter().find(|entry| {
        let (entry_start, entry_end) = entry_range(entry, basis);
        entry_start < offset && offset < entry_end
    }) {
        return format!("inside:{}", summarize_map_entry(entry));
    }

    format!(
        "gap:{}|{}",
        previous_range_entry(entries, offset, basis)
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string()),
        next_range_entry(entries, offset, basis)
            .map(summarize_map_entry)
            .unwrap_or_else(|| "-".to_string())
    )
}

fn format_range_control_counts<'a>(
    entries: impl Iterator<Item = &'a rjtd_core::document_text::DocumentTextMapEntry>,
) -> String {
    let mut counts = BTreeMap::new();
    for entry in entries {
        if entry.kind().as_str() == "control"
            && let Some(code) = entry.code()
        {
            *counts.entry(code).or_insert(0usize) += 1;
        }
    }

    if counts.is_empty() {
        "-".to_string()
    } else {
        counts
            .into_iter()
            .map(|(code, count)| format!("0x{code:04x}:{count}"))
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn format_mark_ids(ids: impl Iterator<Item = u16>) -> String {
    let values = ids.map(|id| id.to_string()).collect::<Vec<_>>();
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(",")
    }
}

fn format_word_context(words: &[u16], start: usize, end: usize) -> String {
    let values = words
        .get(start..end)
        .unwrap_or(&[])
        .iter()
        .map(|word| format!("0x{word:04x}"))
        .collect::<Vec<_>>();
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(",")
    }
}

fn is_line_mark_tag(word: u16) -> bool {
    matches!(word, 0x1000..=0x1002)
}

fn format_line_word_at(words: &[u16], index: usize) -> String {
    words
        .get(index)
        .map(|word| format!("0x{word:04x}"))
        .unwrap_or_else(|| "out-of-range".to_string())
}

fn format_line_word_index_context(words: Option<&[u16]>, index: usize) -> String {
    let Some(words) = words else {
        return "missing".to_string();
    };
    words
        .get(index)
        .map(|word| format!("hit:{index}:0x{word:04x}"))
        .unwrap_or_else(|| format!("out-of-range:{}", words.len()))
}

fn format_line_byte_offset_context(
    words: Option<&[u16]>,
    byte_len: Option<usize>,
    offset: usize,
) -> String {
    let (Some(words), Some(byte_len)) = (words, byte_len) else {
        return "missing".to_string();
    };
    if offset >= byte_len {
        return format!("out-of-range:{byte_len}");
    }
    if !offset.is_multiple_of(2) {
        return format!("unaligned:{offset}");
    }
    format_line_word_index_context(Some(words), offset / 2)
}

fn format_index_context(limit: Option<usize>, value: usize) -> String {
    let Some(limit) = limit else {
        return "missing".to_string();
    };
    if value < limit {
        format!("hit:{value}")
    } else {
        format!("out-of-range:{limit}")
    }
}

fn format_line_word_context_around(words: &[u16], index: usize) -> String {
    let previous_end = index.min(words.len());
    let previous_start = previous_end.saturating_sub(4);
    let next_start = index.saturating_add(1).min(words.len());
    let next_end = index.saturating_add(7).min(words.len());
    format!(
        "prev={}|next={}",
        format_word_context(words, previous_start, previous_end),
        format_word_context(words, next_start, next_end)
    )
}

fn format_nearest_line_tag(words: &[u16], index: usize, before: bool) -> String {
    let found = if before {
        let end = index.min(words.len());
        words[..end]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, word)| is_line_mark_tag(**word))
    } else {
        let start = index.saturating_add(1).min(words.len());
        words[start..]
            .iter()
            .enumerate()
            .find(|(_, word)| is_line_mark_tag(**word))
            .map(|(offset, word)| (start + offset, word))
    };

    found
        .map(|(tag_index, word)| {
            let delta = tag_index as isize - index as isize;
            format!("0x{word:04x}@{tag_index},d={delta}")
        })
        .unwrap_or_else(|| "-".to_string())
}

fn format_be16_fields(bytes: &[u8]) -> String {
    let values = read_be16_fields(bytes)
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(",")
    }
}

fn format_be16_hex_fields(bytes: &[u8]) -> String {
    let values = read_be16_fields(bytes)
        .into_iter()
        .map(|value| format!("0x{value:04x}"))
        .collect::<Vec<_>>();
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(",")
    }
}

fn format_be16_signed_fields(bytes: &[u8]) -> String {
    let values = read_be16_fields(bytes)
        .into_iter()
        .map(|value| (value as i16).to_string())
        .collect::<Vec<_>>();
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(",")
    }
}

fn read_be16_fields(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
        .collect()
}

fn optional_tail_span(start: Option<u16>, end: Option<u16>) -> Option<i64> {
    Some(end? as i64 - start? as i64)
}

fn format_optional_u16_decimal(value: Option<u16>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn format_optional_u16_hex(value: Option<u16>) -> String {
    value
        .map(|value| format!("0x{value:04x}"))
        .unwrap_or_else(|| "-".to_string())
}

fn format_optional_i64(value: Option<i64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn format_optional_u64(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn format_span_relation(chosen_span: u32, tail_span: Option<i64>) -> &'static str {
    let Some(tail_span) = tail_span else {
        return "-";
    };
    let chosen_span = chosen_span as i64;
    match tail_span.cmp(&chosen_span) {
        std::cmp::Ordering::Equal => "eq",
        std::cmp::Ordering::Greater => "gt",
        std::cmp::Ordering::Less => "lt",
    }
}

fn count_tail_delta_hit(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: Option<u16>,
    delta: usize,
    text_only: bool,
) -> bool {
    let Some(offset) = offset else {
        return false;
    };
    let offset = (offset as usize).saturating_add(delta);
    if text_only {
        unit_text_hit(entries, offset).is_some()
    } else {
        unit_hit(entries, offset).is_some()
    }
}

type TailDeltaGroupKey = (
    &'static str,
    Option<u16>,
    Option<u16>,
    Option<u16>,
    Option<u16>,
);
type TailDeltaRow = (Option<u16>, Option<u16>);
type TailDeltaGroups = BTreeMap<TailDeltaGroupKey, Vec<TailDeltaRow>>;

#[derive(Clone, Copy, Default)]
struct TailDeltaScore {
    unit_hits: usize,
    text_hits: usize,
    both_unit_rows: usize,
    both_text_rows: usize,
}

struct TailDeltaBest {
    unit_delta: usize,
    unit_score: TailDeltaScore,
    text_delta: usize,
    text_score: TailDeltaScore,
}

#[derive(Default)]
struct TailFieldRoleSummary {
    nonzero_count: usize,
    distinct_values: BTreeSet<u16>,
    value_counts: BTreeMap<u16, usize>,
    unit_delta_hits: BTreeMap<usize, usize>,
    text_delta_hits: BTreeMap<usize, usize>,
}

impl TailFieldRoleSummary {
    fn delta_hit_count(&self, delta: usize, text_only: bool) -> usize {
        if text_only {
            self.text_delta_hits
                .get(&delta)
                .copied()
                .unwrap_or_default()
        } else {
            self.unit_delta_hits
                .get(&delta)
                .copied()
                .unwrap_or_default()
        }
    }
}

struct TailFieldPairRoleSummary {
    pair_count: usize,
    endpoints: usize,
    span_eq_count: usize,
    span_lt_count: usize,
    span_gt_count: usize,
    best: TailDeltaBest,
    delta_scores: BTreeMap<usize, TailDeltaScore>,
}

impl TailFieldPairRoleSummary {
    fn delta_score(&self, delta: usize) -> TailDeltaScore {
        self.delta_scores.get(&delta).copied().unwrap_or_default()
    }
}

fn summarize_tail_field_roles(
    entries: &[rjtd_core::document_text_position::DocumentTextCountEntry],
    map_entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    deltas: &[usize],
) -> Vec<TailFieldRoleSummary> {
    let mut fields = Vec::new();

    for entry in entries {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);

        if fields.len() < tail_fields.len() {
            fields.resize_with(tail_fields.len(), TailFieldRoleSummary::default);
        }

        for (field_index, value) in tail_fields.into_iter().enumerate() {
            if value == 0 {
                continue;
            }

            let field = &mut fields[field_index];
            field.nonzero_count += 1;
            field.distinct_values.insert(value);
            *field.value_counts.entry(value).or_insert(0) += 1;
            for delta in deltas {
                if count_tail_delta_hit(map_entries, Some(value), *delta, false) {
                    *field.unit_delta_hits.entry(*delta).or_insert(0) += 1;
                }
                if count_tail_delta_hit(map_entries, Some(value), *delta, true) {
                    *field.text_delta_hits.entry(*delta).or_insert(0) += 1;
                }
            }
        }
    }

    fields
}

fn summarize_tail_field_pair_roles(
    entries: &[rjtd_core::document_text_position::DocumentTextCountEntry],
    map_entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    deltas: &[usize],
) -> Vec<TailFieldPairRoleSummary> {
    let mut rows_by_pair: Vec<Vec<TailDeltaRow>> = Vec::new();
    let mut spans_by_pair: Vec<Vec<Option<i64>>> = Vec::new();
    let mut chosen_spans = Vec::new();

    for entry in entries {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let chosen_span = chosen_end.saturating_sub(chosen_start) as i64;
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);

        if tail_fields.len() < 2 {
            continue;
        }
        if rows_by_pair.len() < tail_fields.len() - 1 {
            rows_by_pair.resize_with(tail_fields.len() - 1, Vec::new);
            spans_by_pair.resize_with(tail_fields.len() - 1, Vec::new);
        }

        for pair_index in 0..tail_fields.len() - 1 {
            let left = nonzero_u16(tail_fields[pair_index]);
            let right = nonzero_u16(tail_fields[pair_index + 1]);
            rows_by_pair[pair_index].push((left, right));
            spans_by_pair[pair_index].push(optional_tail_span(left, right));
        }
        chosen_spans.push(chosen_span);
    }

    rows_by_pair
        .into_iter()
        .enumerate()
        .map(|(pair_index, rows)| {
            let pair_count = rows
                .iter()
                .filter(|(left, right)| left.is_some() && right.is_some())
                .count();
            let endpoints = rows
                .iter()
                .map(|(left, right)| usize::from(left.is_some()) + usize::from(right.is_some()))
                .sum::<usize>();
            let mut span_eq_count = 0usize;
            let mut span_lt_count = 0usize;
            let mut span_gt_count = 0usize;
            for (row_index, span) in spans_by_pair
                .get(pair_index)
                .into_iter()
                .flat_map(|spans| spans.iter())
                .enumerate()
            {
                let Some(span) = span else {
                    continue;
                };
                match span.cmp(&chosen_spans[row_index]) {
                    std::cmp::Ordering::Equal => span_eq_count += 1,
                    std::cmp::Ordering::Less => span_lt_count += 1,
                    std::cmp::Ordering::Greater => span_gt_count += 1,
                }
            }
            let best = best_tail_deltas(map_entries, &rows);
            let delta_scores = deltas
                .iter()
                .map(|delta| (*delta, score_tail_delta_group(map_entries, &rows, *delta)))
                .collect();

            TailFieldPairRoleSummary {
                pair_count,
                endpoints,
                span_eq_count,
                span_lt_count,
                span_gt_count,
                best,
                delta_scores,
            }
        })
        .collect()
}

fn nonzero_u16(value: u16) -> Option<u16> {
    (value != 0).then_some(value)
}

fn best_tail_deltas(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    rows: &[TailDeltaRow],
) -> TailDeltaBest {
    let mut best = TailDeltaBest {
        unit_delta: 0,
        unit_score: score_tail_delta_group(entries, rows, 0),
        text_delta: 0,
        text_score: score_tail_delta_group(entries, rows, 0),
    };

    for delta in 1..=64usize {
        let score = score_tail_delta_group(entries, rows, delta);
        if is_better_unit_delta(score, delta, best.unit_score, best.unit_delta) {
            best.unit_delta = delta;
            best.unit_score = score;
        }
        if is_better_text_delta(score, delta, best.text_score, best.text_delta) {
            best.text_delta = delta;
            best.text_score = score;
        }
    }

    best
}

fn score_tail_delta_group(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    rows: &[TailDeltaRow],
    delta: usize,
) -> TailDeltaScore {
    let mut score = TailDeltaScore::default();
    for (t1, t2) in rows {
        let t1_unit_hit = count_tail_delta_hit(entries, *t1, delta, false);
        let t2_unit_hit = count_tail_delta_hit(entries, *t2, delta, false);
        let t1_text_hit = count_tail_delta_hit(entries, *t1, delta, true);
        let t2_text_hit = count_tail_delta_hit(entries, *t2, delta, true);

        score.unit_hits += usize::from(t1_unit_hit) + usize::from(t2_unit_hit);
        score.text_hits += usize::from(t1_text_hit) + usize::from(t2_text_hit);
        if t1_unit_hit && t2_unit_hit {
            score.both_unit_rows += 1;
        }
        if t1_text_hit && t2_text_hit {
            score.both_text_rows += 1;
        }
    }
    score
}

fn is_better_unit_delta(
    candidate: TailDeltaScore,
    candidate_delta: usize,
    best: TailDeltaScore,
    best_delta: usize,
) -> bool {
    candidate.unit_hits > best.unit_hits
        || (candidate.unit_hits == best.unit_hits
            && (candidate.both_unit_rows > best.both_unit_rows
                || (candidate.both_unit_rows == best.both_unit_rows
                    && (candidate.text_hits > best.text_hits
                        || (candidate.text_hits == best.text_hits
                            && (candidate.both_text_rows > best.both_text_rows
                                || (candidate.both_text_rows == best.both_text_rows
                                    && candidate_delta < best_delta)))))))
}

fn is_better_text_delta(
    candidate: TailDeltaScore,
    candidate_delta: usize,
    best: TailDeltaScore,
    best_delta: usize,
) -> bool {
    candidate.text_hits > best.text_hits
        || (candidate.text_hits == best.text_hits
            && (candidate.both_text_rows > best.both_text_rows
                || (candidate.both_text_rows == best.both_text_rows
                    && (candidate.unit_hits > best.unit_hits
                        || (candidate.unit_hits == best.unit_hits
                            && (candidate.both_unit_rows > best.both_unit_rows
                                || (candidate.both_unit_rows == best.both_unit_rows
                                    && candidate_delta < best_delta)))))))
}

fn format_best_unit_delta(delta: usize, score: TailDeltaScore) -> String {
    format!("{}:{}:{}", delta, score.unit_hits, score.both_unit_rows)
}

fn format_best_text_delta(delta: usize, score: TailDeltaScore) -> String {
    format!("{}:{}:{}", delta, score.text_hits, score.both_text_rows)
}

fn format_tail_delta_score(score: TailDeltaScore) -> String {
    format!(
        "{}:{}:{}:{}",
        score.unit_hits, score.text_hits, score.both_unit_rows, score.both_text_rows
    )
}

fn format_tail_extra_byte(bytes: &[u8]) -> String {
    let extra = bytes.chunks_exact(2).remainder();
    if extra.is_empty() {
        "-".to_string()
    } else {
        bytes_to_hex(extra)
    }
}

fn format_le16_fields(bytes: &[u8]) -> String {
    let values = bytes
        .chunks_exact(2)
        .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]).to_string())
        .collect::<Vec<_>>();
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(",")
    }
}

fn format_be32_candidate(bytes: &[u8], offset: usize) -> String {
    if offset + 4 > bytes.len() {
        "-".to_string()
    } else {
        read_be32_candidate(bytes, offset).to_string()
    }
}

fn stream_len_summary(bytes: &[u8], path: &str) -> String {
    read_cfb_stream(bytes, path)
        .map(|stream| stream.len().to_string())
        .unwrap_or_else(|_| "missing".to_string())
}

fn line_mark_summary(bytes: &[u8]) -> String {
    let Ok(stream) = read_cfb_stream(bytes, "/LineMark") else {
        return "missing".to_string();
    };
    let words = be16_words(&stream).take(4).collect::<Vec<_>>();
    format!(
        "len={},words={}",
        stream.len(),
        words
            .iter()
            .map(|word| format!("0x{word:04x}"))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn write_line_mark_intervals(stream: &[u8]) -> Result<(), String> {
    const HEADER_BYTES: usize = 18;
    const COUNT_OFFSET: usize = 8;
    const BASE_UNIT: usize = 16;
    const RECORD_BYTES: usize = 4;

    let words = be16_words(stream).collect::<Vec<_>>();
    let declared_count = read_be16_candidate(stream, COUNT_OFFSET)
        .map(usize::from)
        .unwrap_or_default();
    let max_records = stream.len().saturating_sub(HEADER_BYTES) / RECORD_BYTES;
    let parsed_limit = declared_count.min(max_records);
    let mut rows = Vec::new();
    let mut parsed_records = 0usize;
    let mut unit_start = BASE_UNIT;
    for record_index in 0..parsed_limit {
        let byte_offset = HEADER_BYTES + record_index * RECORD_BYTES;
        let Some(delta_word) = read_be16_candidate(stream, byte_offset) else {
            break;
        };
        let Some(flag_word) = read_be16_candidate(stream, byte_offset + 2) else {
            break;
        };
        let delta = delta_word as i16;
        if delta <= 0 {
            rows.push(format!(
                "line-mark-interval-stop\trecord={record_index}\tbyte={byte_offset}\tdelta={delta}\tflag=0x{flag_word:04x}\treason=non-positive-delta"
            ));
            break;
        }
        let unit_end = unit_start.saturating_add(delta as usize);
        let word_index = byte_offset / 2;
        rows.push(format!(
            "line-mark-interval\trecord={record_index}\tbyte={byte_offset}\tword={word_index}\tdelta={delta}\tflag=0x{flag_word:04x}\tunit-start={unit_start}\tunit-end={unit_end}\tprev={}\tnext={}",
            format_word_context(&words, word_index.saturating_sub(4), word_index),
            format_word_context(&words, word_index + 2, (word_index + 8).min(words.len()))
        ));
        parsed_records += 1;
        unit_start = unit_end;
    }
    let profile = if parsed_records > 0 {
        "be16-delta-v1"
    } else {
        "unparsed"
    };
    write_stdout_line(&format!(
        "summary\tlen={}\twords={}\tprofile={}\tdeclared-count={}\tmax-records={}\tparsed-records={}\tbase-unit={}\theader={}",
        stream.len(),
        words.len(),
        profile,
        declared_count,
        max_records,
        parsed_records,
        BASE_UNIT,
        format_word_context(&words, 0, HEADER_BYTES / 2)
    ))?;
    for row in rows {
        write_stdout_line(&row)?;
    }
    Ok(())
}

fn page_mark_summary(bytes: &[u8]) -> String {
    let Ok(page_mark) = read_page_mark(bytes) else {
        return "missing".to_string();
    };
    let header = page_mark.header();
    format!(
        "count={},stride={},last={},entries={},family={}",
        header.count_value(),
        header.stride_value(),
        header.last_index_value(),
        page_mark.entries().len(),
        page_mark.family().as_str()
    )
}

fn page_mark_entries_summary(bytes: &[u8]) -> String {
    read_page_mark(bytes)
        .map(|page_mark| page_mark.entries().len().to_string())
        .unwrap_or_else(|_| "missing".to_string())
}

fn paper_mark_summary(bytes: &[u8]) -> String {
    let Ok(paper_mark) = read_paper_mark(bytes) else {
        return "missing".to_string();
    };
    let header = paper_mark.header();
    format!(
        "count={},stride={},last={},entries={}",
        header.count_value(),
        header.stride_value(),
        header.last_index_value(),
        paper_mark.entries().len()
    )
}

fn paper_mark_entries_summary(bytes: &[u8]) -> String {
    read_paper_mark(bytes)
        .map(|paper_mark| paper_mark.entries().len().to_string())
        .unwrap_or_else(|_| "missing".to_string())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

const PAGE_MARK_U16_PROFILE_WORD_INDEXES: [usize; 8] = [10, 13, 14, 17, 18, 19, 20, 21];
const PAGE_MARK_U16_PROFILE_CLASSES: [&str; 4] = [
    "zero-sentinel",
    "additive-row",
    "additive-boundary",
    "mixed-payload",
];

fn write_page_mark_u16_profile(page_mark: &PageMark) -> Result<(), String> {
    let mut class_counts = BTreeMap::<&'static str, usize>::new();
    let mut tuple_counts = BTreeMap::<(&'static str, [Option<u16>; 8]), usize>::new();

    for entry in page_mark.entries() {
        let fields = be16_words(entry.raw()).collect::<Vec<_>>();
        let profile = page_mark_u16_geometry_profile(&fields);
        let class_name = profile.class_name();
        *class_counts.entry(class_name).or_insert(0) += 1;
        let tuple = PAGE_MARK_U16_PROFILE_WORD_INDEXES.map(|index| fields.get(index).copied());
        *tuple_counts.entry((class_name, tuple)).or_insert(0) += 1;
    }

    write_stdout_line(&format!(
        "summary\tentries={}\tzero-sentinel={}\tadditive-row={}\tadditive-boundary={}\tmixed-payload={}\tdecoded=false",
        page_mark.entries().len(),
        class_counts.get("zero-sentinel").copied().unwrap_or(0),
        class_counts.get("additive-row").copied().unwrap_or(0),
        class_counts.get("additive-boundary").copied().unwrap_or(0),
        class_counts.get("mixed-payload").copied().unwrap_or(0)
    ))?;

    for class_name in PAGE_MARK_U16_PROFILE_CLASSES {
        write_stdout_line(&format!(
            "profile\t{}\t{}",
            class_name,
            class_counts.get(class_name).copied().unwrap_or(0)
        ))?;
    }

    for ((class_name, tuple), count) in tuple_counts {
        write_stdout_line(&format!(
            "tuple\t{}\t{}\t{}",
            class_name,
            count,
            format_page_mark_u16_profile_tuple(&tuple)
        ))?;
    }

    Ok(())
}

fn format_page_mark_u16_profile_tuple(tuple: &[Option<u16>; 8]) -> String {
    PAGE_MARK_U16_PROFILE_WORD_INDEXES
        .iter()
        .zip(tuple.iter())
        .map(|(word_index, value)| {
            format!(
                "w{}={}",
                word_index,
                value
                    .map(|value| format!("{value}/0x{value:04x}"))
                    .unwrap_or_else(|| "-".to_string())
            )
        })
        .collect::<Vec<_>>()
        .join("\t")
}

fn page_layout_slot_parts(
    subrecords: &[StyleStreamSubrecordSummary],
) -> BTreeMap<u8, BTreeMap<u8, &StyleStreamSubrecordSummary>> {
    let mut slots: BTreeMap<u8, BTreeMap<u8, &StyleStreamSubrecordSummary>> = BTreeMap::new();
    for subrecord in subrecords {
        let code = subrecord.code();
        let slot = (code >> 8) as u8;
        let part = (code & 0xff) as u8;
        if !(0x31..=0x39).contains(&slot) || !(0x04..=0x07).contains(&part) {
            continue;
        }
        slots.entry(slot).or_default().insert(part, subrecord);
    }
    slots
}

fn active_page_layout_slot_pairs(
    slot_sets: &[BTreeMap<u8, BTreeMap<u8, &StyleStreamSubrecordSummary>>],
) -> BTreeSet<(u8, u8)> {
    let mut pairs = BTreeSet::new();
    for slots in slot_sets {
        for (left, right) in [(0x32, 0x33), (0x34, 0x35), (0x36, 0x37), (0x38, 0x39)] {
            let left_active = slots
                .get(&left)
                .is_some_and(page_layout_slot_part05_is_active);
            let right_active = slots
                .get(&right)
                .is_some_and(page_layout_slot_part05_is_active);
            if left_active && right_active {
                pairs.insert((left, right));
            }
        }
    }
    pairs
}

fn page_layout_slot_part05_is_active(parts: &BTreeMap<u8, &StyleStreamSubrecordSummary>) -> bool {
    parts
        .get(&0x05)
        .and_then(|subrecord| subrecord.payload().first().copied())
        .is_some_and(|byte| byte != 0)
}

fn format_page_layout_slot_pairs(pairs: &BTreeSet<(u8, u8)>) -> String {
    if pairs.is_empty() {
        return "-".to_string();
    }
    pairs
        .iter()
        .map(|(left, right)| format!("0x{left:02x}/0x{right:02x}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_page_layout_slot_part(
    parts: &BTreeMap<u8, &StyleStreamSubrecordSummary>,
    part: u8,
) -> String {
    parts
        .get(&part)
        .map(|subrecord| bytes_to_hex(subrecord.payload()))
        .unwrap_or_else(|| "-".to_string())
}

fn format_page_layout_slot_part_first(
    parts: &BTreeMap<u8, &StyleStreamSubrecordSummary>,
    part: u8,
) -> String {
    parts
        .get(&part)
        .and_then(|subrecord| subrecord.payload().first().copied())
        .map(|byte| format!("0x{byte:02x}"))
        .unwrap_or_else(|| "-".to_string())
}

fn format_page_layout_slot_part_nonzero(
    parts: &BTreeMap<u8, &StyleStreamSubrecordSummary>,
    part: u8,
) -> String {
    parts
        .get(&part)
        .and_then(|subrecord| subrecord.payload().first().copied())
        .map(|byte| (byte != 0).to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn format_hex_preview(bytes: &[u8], max_bytes: usize) -> String {
    if bytes.is_empty() {
        return "-".to_string();
    }

    let preview_len = bytes.len().min(max_bytes);
    let mut preview = bytes_to_hex(&bytes[..preview_len]);
    if bytes.len() > max_bytes {
        preview.push_str("...");
    }
    preview
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    fnv1a64_update(FNV1A64_OFFSET, bytes)
}

fn fnv1a64_update(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= *byte as u64;
        digest = digest.wrapping_mul(FNV1A64_PRIME);
    }
    digest
}

fn format_fnv1a64_digest(digest: u64) -> String {
    format!("0x{digest:016x}")
}

fn ascii_text_runs(bytes: &[u8], min_chars: usize) -> Vec<(usize, String)> {
    let mut runs = Vec::new();
    let mut start = None;
    let mut text = String::new();

    for (offset, byte) in bytes.iter().copied().enumerate() {
        if byte.is_ascii_graphic() || byte == b' ' {
            start.get_or_insert(offset);
            text.push(byte as char);
            continue;
        }

        push_text_run(&mut runs, start.take(), &mut text, min_chars);
    }
    push_text_run(&mut runs, start, &mut text, min_chars);

    runs
}

#[derive(Clone, Copy)]
enum Utf16Endian {
    Little,
    Big,
}

fn utf16_text_runs(bytes: &[u8], endian: Utf16Endian, min_chars: usize) -> Vec<(usize, String)> {
    let mut runs = Vec::new();
    for alignment in 0..2 {
        let mut start = None;
        let mut text = String::new();
        let mut offset = alignment;
        while offset + 1 < bytes.len() {
            let unit = match endian {
                Utf16Endian::Little => u16::from_le_bytes([bytes[offset], bytes[offset + 1]]),
                Utf16Endian::Big => u16::from_be_bytes([bytes[offset], bytes[offset + 1]]),
            };
            if let Some(character) = char::from_u32(unit as u32)
                && is_probe_text_char(character)
            {
                start.get_or_insert(offset);
                text.push(character);
                offset += 2;
                continue;
            }

            push_text_run(&mut runs, start.take(), &mut text, min_chars);
            offset += 2;
        }
        push_text_run(&mut runs, start, &mut text, min_chars);
    }
    runs.sort_by_key(|(offset, _)| *offset);
    runs
}

fn push_text_run(
    runs: &mut Vec<(usize, String)>,
    start: Option<usize>,
    text: &mut String,
    min_chars: usize,
) {
    if let Some(start) = start
        && text.chars().count() >= min_chars
    {
        runs.push((start, std::mem::take(text)));
        return;
    }

    text.clear();
}

fn is_probe_text_char(character: char) -> bool {
    !character.is_control() && character != '\u{fffd}'
}

fn read_be32_candidate(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

fn text_count_entry_chosen_range(raw: &[u8], family: &str) -> (u32, u32) {
    if family == "be1-shifted" {
        (read_be32_candidate(raw, 1), read_be32_candidate(raw, 5))
    } else {
        (read_be32_candidate(raw, 0), read_be32_candidate(raw, 4))
    }
}

fn text_count_entry_tail_offset(family: &str) -> usize {
    if family == "be1-shifted" { 9 } else { 8 }
}

fn classify_text_count_entry_family(raw: &[u8]) -> &'static str {
    let be0_start = read_be32_candidate(raw, 0);
    let be0_end = read_be32_candidate(raw, 4);
    let be1_start = read_be32_candidate(raw, 1);
    let be1_end = read_be32_candidate(raw, 5);

    if be0_start < 256 && be1_start >= 256 && be1_end >= be1_start && be0_end > be1_end {
        "be1-shifted"
    } else {
        "be0"
    }
}

struct PageMarkShapeClassification {
    name: &'static str,
    rows: Option<usize>,
    row_bytes: Option<usize>,
    trim_bytes: usize,
}

impl PageMarkShapeClassification {
    fn new(
        name: &'static str,
        rows: Option<usize>,
        row_bytes: Option<usize>,
        trim_bytes: usize,
    ) -> Self {
        Self {
            name,
            rows,
            row_bytes,
            trim_bytes,
        }
    }
}

fn classify_page_mark_shape(
    tail_bytes: usize,
    header_count: u32,
    header_stride: u32,
    header_last: u32,
) -> PageMarkShapeClassification {
    if header_stride != 0x10 || header_count > 10_000 || header_last > 10_000 {
        return PageMarkShapeClassification::new("non-page-header", None, None, 0);
    }

    let count_plus_one = header_count.saturating_add(1) as usize;
    if count_plus_one > 0 && tail_bytes.is_multiple_of(84) {
        let rows = tail_bytes / 84;
        if rows == count_plus_one {
            return PageMarkShapeClassification::new(
                "fixed84-count-plus-one",
                Some(rows),
                Some(84),
                0,
            );
        }
        return PageMarkShapeClassification::new("fixed84", Some(rows), Some(84), 0);
    }

    if count_plus_one > 0 && tail_bytes.is_multiple_of(count_plus_one) {
        return PageMarkShapeClassification::new(
            "count-plus-one-variable",
            Some(count_plus_one),
            Some(tail_bytes / count_plus_one),
            0,
        );
    }

    if tail_bytes >= 2 {
        let trimmed = tail_bytes - 2;
        if count_plus_one > 0 && trimmed.is_multiple_of(count_plus_one) {
            return PageMarkShapeClassification::new(
                "count-plus-one-trim2",
                Some(count_plus_one),
                Some(trimmed / count_plus_one),
                2,
            );
        }
    }

    let count = header_count as usize;
    if count > 0 && tail_bytes.is_multiple_of(count) {
        return PageMarkShapeClassification::new(
            "count-variable",
            Some(count),
            Some(tail_bytes / count),
            0,
        );
    }

    if tail_bytes >= 84 {
        return PageMarkShapeClassification::new(
            "fixed84-tail",
            Some(tail_bytes / 84),
            Some(84),
            tail_bytes % 84,
        );
    }

    PageMarkShapeClassification::new("unclassified", None, None, 0)
}

fn classify_paper_mark_shape(
    tail_bytes: usize,
    header_count: u32,
    header_stride: u32,
    header_last: u32,
) -> PageMarkShapeClassification {
    if header_stride != 0x0c || header_count > 10_000 || header_last > 10_000 {
        return PageMarkShapeClassification::new("non-paper-header", None, None, 0);
    }

    if tail_bytes.is_multiple_of(8) {
        return PageMarkShapeClassification::new("fixed8", Some(tail_bytes / 8), Some(8), 0);
    }

    PageMarkShapeClassification::new("unclassified", None, None, 0)
}

fn format_optional_usize(value: Option<usize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn format_model_table_source_index(value: usize) -> String {
    if value >= usize::MAX - 1 {
        "-".to_string()
    } else {
        value.to_string()
    }
}

fn format_optional_u32(value: Option<u32>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn write_fixed_row_candidate(
    label: &str,
    tail_bytes: usize,
    row_bytes: usize,
) -> Result<(), String> {
    write_stdout_line(&format!(
        "candidate\t{}\t{}\t{}\t{}",
        label,
        tail_bytes / row_bytes,
        row_bytes,
        tail_bytes % row_bytes
    ))
}

fn write_header_row_candidate(
    label: &str,
    tail_bytes: usize,
    row_count: u32,
) -> Result<(), String> {
    if row_count == 0 {
        return write_stdout_line(&format!("candidate\t{label}\t-\t-\t-"));
    }
    let row_count = row_count as usize;
    write_stdout_line(&format!(
        "candidate\t{}\t{}\t{}\t{}",
        label,
        row_count,
        tail_bytes / row_count,
        tail_bytes % row_count
    ))
}

fn be16_words(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .chunks_exact(2)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn be32_dwords(bytes: &[u8]) -> impl Iterator<Item = u32> + '_ {
    bytes
        .chunks_exact(4)
        .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn le32_dwords(bytes: &[u8]) -> impl Iterator<Item = u32> + '_ {
    bytes
        .chunks_exact(4)
        .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn format_le32_fields(bytes: &[u8], max_fields: usize) -> String {
    let fields = le32_dwords(bytes)
        .take(max_fields)
        .map(|value| format!("0x{value:08x}"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        "-".to_string()
    } else {
        fields.join(",")
    }
}

fn format_be32_fields(bytes: &[u8]) -> String {
    let fields = be32_dwords(bytes)
        .map(|value| format!("0x{value:08x}"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        "-".to_string()
    } else {
        fields.join(",")
    }
}

fn classify_so_geometry_fields(fields: &[u32]) -> &'static str {
    if fields.len() < 5 {
        return "truncated";
    }

    let values = &fields[1..5];
    if is_jseq3_like_packed_so(fields) {
        return "packed-jseq3-like";
    }

    if is_ffff_preamble_so(fields) {
        return "packed-ffff-preamble";
    }

    if values.iter().any(|value| value >> 16 != 0) {
        return "packed";
    }

    if values == [7, 0x100, 0, 0x64] || values == [0x100, 0, 0x64, 0] {
        return "default-control";
    }

    if values.iter().any(|value| *value > 0x100) {
        return "geometry-like";
    }

    "unknown"
}

fn is_jseq3_like_packed_so(fields: &[u32]) -> bool {
    fields.len() >= 8
        && fields[4] == 0
        && fields[5] == 0
        && fields[6] == (fields[2] & 0xffff)
        && fields[7] != 0
        && fields[1] >> 16 != 0
        && fields[2] >> 16 != 0
        && fields[3] >> 16 != 0
}

fn is_ffff_preamble_so(fields: &[u32]) -> bool {
    fields.len() >= SO_RECORD_DWORDS
        && fields[1] >> 16 != 0
        && fields[2] >> 16 != 0
        && fields[3] == 0x0000ffff
        && fields[4..SO_RECORD_DWORDS].iter().all(|field| *field == 0)
}

fn format_so_geometry_candidate(
    fields: &[u32],
) -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
) {
    if fields.len() < 5 {
        let empty = "-".to_string();
        return (
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty,
        );
    }

    let f1 = fields[1];
    let f2 = fields[2];
    let f3 = fields[3];
    let f4 = fields[4];
    (
        f1.to_string(),
        f2.to_string(),
        f3.to_string(),
        f4.to_string(),
        (f3 as i64 - f1 as i64).to_string(),
        (f4 as i64 - f2 as i64).to_string(),
        (f1 as u64 + f3 as u64).to_string(),
        (f2 as u64 + f4 as u64).to_string(),
    )
}

fn format_so_u16_halves(fields: &[u32], high: bool) -> String {
    format_so_halves(fields, high, |value| value.to_string())
}

fn format_so_i16_halves(fields: &[u32], high: bool) -> String {
    format_so_halves(fields, high, |value| (value as i16).to_string())
}

fn format_so_halves(fields: &[u32], high: bool, formatter: impl Fn(u16) -> String) -> String {
    let halves = fields
        .iter()
        .skip(1)
        .map(|field| {
            if high {
                (field >> 16) as u16
            } else {
                *field as u16
            }
        })
        .map(formatter)
        .collect::<Vec<_>>();
    if halves.is_empty() {
        "-".to_string()
    } else {
        halves.join(",")
    }
}

fn stream_tail(stream: &[u8], offset: usize, byte_count: usize) -> &[u8] {
    let end = offset.saturating_add(byte_count).min(stream.len());
    &stream[offset..end]
}

fn find_subslice_offsets(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return Vec::new();
    }

    haystack
        .windows(needle.len())
        .enumerate()
        .filter_map(|(offset, candidate)| (candidate == needle).then_some(offset))
        .collect()
}

fn parse_hex_bytes(input: &str) -> Result<Vec<u8>, String> {
    let compact = input
        .chars()
        .filter(|character| !character.is_ascii_whitespace() && *character != '_')
        .collect::<String>()
        .replace("0x", "")
        .replace("0X", "");

    if compact.len() % 2 != 0 {
        return Err("hex bytes must contain an even number of digits".into());
    }

    let mut bytes = Vec::with_capacity(compact.len() / 2);
    let mut chars = compact.chars();
    while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
        bytes.push(hex_pair(high, low)?);
    }
    Ok(bytes)
}

fn parse_u16_argument(input: &str) -> Result<u16, String> {
    let compact = input.replace('_', "");
    if let Some(hex) = compact
        .strip_prefix("0x")
        .or_else(|| compact.strip_prefix("0X"))
    {
        u16::from_str_radix(hex, 16).map_err(|_| format!("invalid u16 value: {input}"))
    } else {
        compact
            .parse::<u16>()
            .map_err(|_| format!("invalid u16 value: {input}"))
    }
}

fn unescaped_path(path: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut chars = path.chars().peekable();

    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }

        match chars.next() {
            Some('x') => {
                let high = chars
                    .next()
                    .ok_or_else(|| "incomplete \\x escape in stream path".to_string())?;
                let low = chars
                    .next()
                    .ok_or_else(|| "incomplete \\x escape in stream path".to_string())?;
                let byte = hex_pair(high, low)?;
                output.push(byte as char);
            }
            Some(other) => {
                output.push('\\');
                output.push(other);
            }
            None => output.push('\\'),
        }
    }

    Ok(output)
}

fn hex_pair(high: char, low: char) -> Result<u8, String> {
    let high = high
        .to_digit(16)
        .ok_or_else(|| format!("invalid hex escape digit: {high}"))?;
    let low = low
        .to_digit(16)
        .ok_or_else(|| format!("invalid hex escape digit: {low}"))?;
    Ok(((high << 4) | low) as u8)
}
