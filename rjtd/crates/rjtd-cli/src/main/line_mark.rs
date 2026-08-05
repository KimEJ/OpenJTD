use std::path::Path;

use rjtd_core::container::read_cfb_stream;
use rjtd_core::document_text::{map_document_text, read_document_text_payload};

use crate::input::read_file;
use crate::{probe_compare, probe_corpus};

use super::line_mark_support::*;
use super::support::*;
use super::text_position_count_support::{format_byte_context, format_unit_context};

pub(crate) fn run_line_mark_tags(mut args: impl Iterator<Item = String>) -> Result<(), String> {
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

pub(crate) fn run_line_mark_intervals(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "line-mark-intervals")?;
    let bytes = read_file(path)?;
    let stream = read_cfb_stream(&bytes, "/LineMark").map_err(|error| error.to_string())?;
    write_line_mark_intervals(&stream)
}

pub(crate) fn run_source_y_probe_audit(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "source-y-probe-audit")?;
    let lines = probe_corpus::source_y_probe_audit_lines(Path::new(&path))?;
    for line in lines {
        write_stdout_line(&line)?;
    }
    Ok(())
}

pub(crate) fn run_source_y_probe_compare(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let base_path = required_path(args.next(), "source-y-probe-compare")?;
    let candidate_path = required_path(args.next(), "source-y-probe-compare")?;
    let lines = probe_compare::source_y_probe_compare_lines(
        Path::new(&base_path),
        Path::new(&candidate_path),
    )?;
    for line in lines {
        write_stdout_line(&line)?;
    }
    Ok(())
}

pub(crate) fn run_line_mark_text_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
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
