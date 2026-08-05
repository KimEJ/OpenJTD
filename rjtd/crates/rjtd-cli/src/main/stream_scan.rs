use std::collections::BTreeMap;

use rjtd_core::container::{EntryKind, inspect_cfb_entries, read_cfb_stream};

use crate::input::read_file;

use super::support::*;

pub(crate) fn run_stream_words(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "stream-words")?;
    let stream_path = required_path(args.next(), "stream-words")?;
    let stream_path = unescaped_path(&stream_path)?;
    let bytes = read_file(path)?;
    let stream = read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
    for (index, word) in be16_words(&stream).enumerate() {
        write_stdout_line(&format!("{}\t{}\t0x{:04x}", index, index * 2, word))?;
    }
    Ok(())
}

pub(crate) fn run_stream_word_frequencies(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "stream-word-frequencies")?;
    let stream_path = required_path(args.next(), "stream-word-frequencies")?;
    let stream_path = unescaped_path(&stream_path)?;
    let bytes = read_file(path)?;
    let stream = read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
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

pub(crate) fn run_stream_dwords(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "stream-dwords")?;
    let stream_path = required_path(args.next(), "stream-dwords")?;
    let stream_path = unescaped_path(&stream_path)?;
    let bytes = read_file(path)?;
    let stream = read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
    for (index, dword) in be32_dwords(&stream).enumerate() {
        write_stdout_line(&format!("{}\t{}\t0x{:08x}", index, index * 4, dword))?;
    }
    Ok(())
}

pub(crate) fn run_stream_dword_frequencies(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "stream-dword-frequencies")?;
    let stream_path = required_path(args.next(), "stream-dword-frequencies")?;
    let stream_path = unescaped_path(&stream_path)?;
    let bytes = read_file(path)?;
    let stream = read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
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

pub(crate) fn run_stream_text_probe(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "stream-text-probe")?;
    let stream_path = required_path(args.next(), "stream-text-probe")?;
    let stream_path = unescaped_path(&stream_path)?;
    let bytes = read_file(path)?;
    let stream = read_cfb_stream(&bytes, &stream_path).map_err(|error| error.to_string())?;
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

pub(crate) fn run_stream_find(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "stream-find")?;
    let needle_path = required_path(args.next(), "stream-find")?;
    let needle_path = unescaped_path(&needle_path)?;
    let bytes = read_file(path)?;
    let needle = read_cfb_stream(&bytes, &needle_path).map_err(|error| error.to_string())?;
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

pub(crate) fn run_stream_find_bytes(mut args: impl Iterator<Item = String>) -> Result<(), String> {
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
