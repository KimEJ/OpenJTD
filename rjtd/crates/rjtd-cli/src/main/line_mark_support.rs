use rjtd_core::container::read_cfb_stream;

use super::object_fdm_support::read_be16_candidate;
use super::support::*;

pub(crate) fn format_word_context(words: &[u16], start: usize, end: usize) -> String {
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

pub(crate) fn is_line_mark_tag(word: u16) -> bool {
    matches!(word, 0x1000..=0x1002)
}

pub(crate) fn format_line_word_at(words: &[u16], index: usize) -> String {
    words
        .get(index)
        .map(|word| format!("0x{word:04x}"))
        .unwrap_or_else(|| "out-of-range".to_string())
}

pub(crate) fn format_line_word_index_context(words: Option<&[u16]>, index: usize) -> String {
    let Some(words) = words else {
        return "missing".to_string();
    };
    words
        .get(index)
        .map(|word| format!("hit:{index}:0x{word:04x}"))
        .unwrap_or_else(|| format!("out-of-range:{}", words.len()))
}

pub(crate) fn format_line_byte_offset_context(
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

pub(crate) fn format_index_context(limit: Option<usize>, value: usize) -> String {
    let Some(limit) = limit else {
        return "missing".to_string();
    };
    if value < limit {
        format!("hit:{value}")
    } else {
        format!("out-of-range:{limit}")
    }
}

pub(crate) fn format_line_word_context_around(words: &[u16], index: usize) -> String {
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

pub(crate) fn format_nearest_line_tag(words: &[u16], index: usize, before: bool) -> String {
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

pub(crate) fn line_mark_summary(bytes: &[u8]) -> String {
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

pub(crate) fn write_line_mark_intervals(stream: &[u8]) -> Result<(), String> {
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
