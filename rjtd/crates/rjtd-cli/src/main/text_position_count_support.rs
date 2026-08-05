use std::collections::{BTreeMap, BTreeSet};

use rjtd_core::container::read_cfb_stream;

use super::support::*;
use super::text_boundary_support::format_range_control_counts;

pub(crate) fn document_text_map_meta(
    entry: &rjtd_core::document_text::DocumentTextMapEntry,
) -> String {
    match (entry.selector(), entry.code()) {
        (Some(selector), _) => format!("0x{selector:04x}"),
        (_, Some(code)) => format!("0x{code:04x}"),
        (None, None) => "-".to_string(),
    }
}

pub(crate) fn format_byte_context(
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

pub(crate) fn format_unit_context(
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

pub(crate) fn format_optional_byte_context(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: Option<u16>,
) -> String {
    offset
        .map(|offset| format_byte_context(entries, offset as usize))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_unit_context(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: Option<u16>,
) -> String {
    offset
        .map(|offset| format_unit_context(entries, offset as usize))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_unit_context_with_delta(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: Option<u16>,
    delta: usize,
) -> String {
    offset
        .map(|offset| format_unit_context(entries, (offset as usize).saturating_add(delta)))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn unit_hit(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
) -> Option<&rjtd_core::document_text::DocumentTextMapEntry> {
    entries
        .iter()
        .find(|entry| entry.contains_unit_offset(offset))
}

pub(crate) fn unit_text_hit(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    offset: usize,
) -> Option<&rjtd_core::document_text::DocumentTextMapEntry> {
    unit_hit(entries, offset).filter(|entry| entry.kind().as_str() == "text")
}

pub(crate) fn format_between_context(
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

pub(crate) fn summarize_map_entry(
    entry: &rjtd_core::document_text::DocumentTextMapEntry,
) -> String {
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

pub(crate) fn format_map_entry_at(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    index: Option<usize>,
) -> String {
    index
        .and_then(|index| entries.get(index))
        .map(summarize_map_entry)
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_nearest_control_entry(
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

pub(crate) fn format_control_code_sequence(
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

pub(crate) fn format_control_range_delimiter(filter: Option<u16>) -> String {
    filter
        .map(|code| format!("0x{code:04x}"))
        .unwrap_or_else(|| "all".to_string())
}

pub(crate) fn format_control_range_boundary(
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

pub(crate) fn format_entry_index_span(start: usize, end: usize) -> String {
    if start >= end {
        "-".to_string()
    } else {
        format!("{start}-{}", end - 1)
    }
}

pub(crate) fn format_control_range_contents(
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

pub(crate) fn format_byte_range_preview(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    format_range_preview(entries, start, end, |entry| {
        (entry.byte_start(), entry.byte_end())
    })
}

pub(crate) fn format_unit_range_preview(
    entries: &[rjtd_core::document_text::DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    format_range_preview(entries, start, end, |entry| {
        (entry.unit_start(), entry.unit_end())
    })
}

pub(crate) fn format_range_preview(
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

pub(crate) fn format_be16_fields(bytes: &[u8]) -> String {
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

pub(crate) fn format_be16_hex_fields(bytes: &[u8]) -> String {
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

pub(crate) fn format_be16_signed_fields(bytes: &[u8]) -> String {
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

pub(crate) fn read_be16_fields(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
        .collect()
}

pub(crate) fn optional_tail_span(start: Option<u16>, end: Option<u16>) -> Option<i64> {
    Some(end? as i64 - start? as i64)
}

pub(crate) fn format_optional_u16_decimal(value: Option<u16>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_u16_hex(value: Option<u16>) -> String {
    value
        .map(|value| format!("0x{value:04x}"))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_i64(value: Option<i64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_u64(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_f32_3(value: Option<f32>) -> String {
    value
        .map(|value| format!("{value:.3}"))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_span_relation(chosen_span: u32, tail_span: Option<i64>) -> &'static str {
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

pub(crate) fn count_tail_delta_hit(
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

pub(crate) type TailDeltaGroupKey = (
    &'static str,
    Option<u16>,
    Option<u16>,
    Option<u16>,
    Option<u16>,
);
pub(crate) type TailDeltaRow = (Option<u16>, Option<u16>);
pub(crate) type TailDeltaGroups = BTreeMap<TailDeltaGroupKey, Vec<TailDeltaRow>>;

#[derive(Clone, Copy, Default)]
pub(crate) struct TailDeltaScore {
    pub(crate) unit_hits: usize,
    pub(crate) text_hits: usize,
    pub(crate) both_unit_rows: usize,
    pub(crate) both_text_rows: usize,
}

pub(crate) struct TailDeltaBest {
    pub(crate) unit_delta: usize,
    pub(crate) unit_score: TailDeltaScore,
    pub(crate) text_delta: usize,
    pub(crate) text_score: TailDeltaScore,
}

#[derive(Default)]
pub(crate) struct TailFieldRoleSummary {
    pub(crate) nonzero_count: usize,
    pub(crate) distinct_values: BTreeSet<u16>,
    pub(crate) value_counts: BTreeMap<u16, usize>,
    pub(crate) unit_delta_hits: BTreeMap<usize, usize>,
    pub(crate) text_delta_hits: BTreeMap<usize, usize>,
}

impl TailFieldRoleSummary {
    pub(crate) fn delta_hit_count(&self, delta: usize, text_only: bool) -> usize {
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

pub(crate) struct TailFieldPairRoleSummary {
    pub(crate) pair_count: usize,
    pub(crate) endpoints: usize,
    pub(crate) span_eq_count: usize,
    pub(crate) span_lt_count: usize,
    pub(crate) span_gt_count: usize,
    pub(crate) best: TailDeltaBest,
    pub(crate) delta_scores: BTreeMap<usize, TailDeltaScore>,
}

impl TailFieldPairRoleSummary {
    pub(crate) fn delta_score(&self, delta: usize) -> TailDeltaScore {
        self.delta_scores.get(&delta).copied().unwrap_or_default()
    }
}

pub(crate) fn summarize_tail_field_roles(
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

pub(crate) fn summarize_tail_field_pair_roles(
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

pub(crate) fn nonzero_u16(value: u16) -> Option<u16> {
    (value != 0).then_some(value)
}

pub(crate) fn best_tail_deltas(
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

pub(crate) fn score_tail_delta_group(
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

pub(crate) fn is_better_unit_delta(
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

pub(crate) fn is_better_text_delta(
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

pub(crate) fn format_best_unit_delta(delta: usize, score: TailDeltaScore) -> String {
    format!("{}:{}:{}", delta, score.unit_hits, score.both_unit_rows)
}

pub(crate) fn format_best_text_delta(delta: usize, score: TailDeltaScore) -> String {
    format!("{}:{}:{}", delta, score.text_hits, score.both_text_rows)
}

pub(crate) fn format_tail_delta_score(score: TailDeltaScore) -> String {
    format!(
        "{}:{}:{}:{}",
        score.unit_hits, score.text_hits, score.both_unit_rows, score.both_text_rows
    )
}

pub(crate) fn format_tail_extra_byte(bytes: &[u8]) -> String {
    let extra = bytes.chunks_exact(2).remainder();
    if extra.is_empty() {
        "-".to_string()
    } else {
        bytes_to_hex(extra)
    }
}

pub(crate) fn format_le16_fields(bytes: &[u8]) -> String {
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

pub(crate) fn format_be32_candidate(bytes: &[u8], offset: usize) -> String {
    if offset + 4 > bytes.len() {
        "-".to_string()
    } else {
        read_be32_candidate(bytes, offset).to_string()
    }
}

pub(crate) fn stream_len_summary(bytes: &[u8], path: &str) -> String {
    read_cfb_stream(bytes, path)
        .map(|stream| stream.len().to_string())
        .unwrap_or_else(|_| "missing".to_string())
}

pub(crate) fn text_count_entry_chosen_range(raw: &[u8], family: &str) -> (u32, u32) {
    if family == "be1-shifted" {
        (read_be32_candidate(raw, 1), read_be32_candidate(raw, 5))
    } else {
        (read_be32_candidate(raw, 0), read_be32_candidate(raw, 4))
    }
}

pub(crate) fn text_count_entry_tail_offset(family: &str) -> usize {
    if family == "be1-shifted" { 9 } else { 8 }
}

pub(crate) fn classify_text_count_entry_family(raw: &[u8]) -> &'static str {
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
