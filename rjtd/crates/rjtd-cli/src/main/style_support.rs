use std::collections::{BTreeMap, BTreeSet};

use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, StyleStreamRecordSummary, StyleStreamSubrecordSummary,
};

use super::support::{
    FNV1A64_OFFSET, bytes_to_hex, escaped_text, fnv1a64, fnv1a64_update, format_fnv1a64_digest,
    format_hex_preview,
};
use super::text_position_count_support::{
    classify_text_count_entry_family, format_be16_hex_fields, read_be16_fields,
    text_count_entry_tail_offset,
};

const STYLE_RECORD_PAYLOAD_PREVIEW_BYTES: usize = 16;

pub(crate) fn format_u32_hex_values(values: &[u32]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }

    values
        .iter()
        .map(|value| format!("0x{value:08x}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_u16_hex_values(values: &[u16]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }

    values
        .iter()
        .map(|value| format!("0x{value:04x}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_usize_values(values: &[usize]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }

    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_optional_text(value: Option<&str>) -> String {
    value
        .filter(|text| !text.is_empty())
        .map(escaped_text)
        .unwrap_or_else(|| "-".to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CliStyleCandidate {
    pub(crate) id: usize,
    pub(crate) record_index: usize,
    pub(crate) offset: usize,
    pub(crate) label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DocumentViewStyleGroup {
    pub(crate) id: u16,
    pub(crate) record_count: usize,
    pub(crate) codes: Vec<u16>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct StyleFieldSummary {
    pub(crate) nonzero_count: usize,
    pub(crate) distinct_values: BTreeSet<u16>,
    pub(crate) value_counts: BTreeMap<u16, usize>,
    pub(crate) text_style_id_hits: BTreeMap<usize, usize>,
    pub(crate) text_style_index_hits: BTreeMap<usize, usize>,
    pub(crate) page_style_id_hits: BTreeMap<usize, usize>,
    pub(crate) page_style_index_hits: BTreeMap<usize, usize>,
    pub(crate) view_style_group_hits: BTreeMap<u16, usize>,
}

pub(crate) fn collect_labeled_style_candidates(
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

pub(crate) fn collect_document_view_style_groups(
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

pub(crate) fn document_view_style_group_id(code: u16) -> Option<u16> {
    let high = code >> 8;
    let low = code & 0x00ff;
    if (0x31..=0x39).contains(&high) && (0x04..=0x07).contains(&low) {
        Some(high - 0x30)
    } else {
        None
    }
}

pub(crate) fn style_record_payload<'a>(
    stream_bytes: &'a [u8],
    record: &StyleStreamRecordSummary,
) -> Option<&'a [u8]> {
    let start = record.offset().checked_add(4)?;
    let end = start.checked_add(record.payload_len())?;
    stream_bytes.get(start..end)
}

pub(crate) fn format_style_record_payload_preview(
    stream_bytes: &[u8],
    record: &StyleStreamRecordSummary,
) -> String {
    let Some(payload) = style_record_payload(stream_bytes, record) else {
        return "invalid".to_string();
    };
    format_hex_preview(payload, STYLE_RECORD_PAYLOAD_PREVIEW_BYTES)
}

pub(crate) fn format_style_record_payload_be16(
    stream_bytes: &[u8],
    record: &StyleStreamRecordSummary,
) -> String {
    let Some(payload) = style_record_payload(stream_bytes, record) else {
        return "invalid".to_string();
    };
    format_be16_hex_fields(payload)
}

pub(crate) fn format_style_record_payload_digest(
    stream_bytes: &[u8],
    record: &StyleStreamRecordSummary,
) -> String {
    let Some(payload) = style_record_payload(stream_bytes, record) else {
        return "invalid".to_string();
    };
    format_fnv1a64_digest(fnv1a64(payload))
}

pub(crate) fn format_document_view_group_payload_digest(
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

pub(crate) fn summarize_text_position_style_fields(
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

pub(crate) fn format_indexed_u16_fields(fields: &[u16]) -> String {
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

pub(crate) fn format_u16_value_counts(counts: &BTreeMap<u16, usize>) -> String {
    if counts.is_empty() {
        return "-".to_string();
    }

    counts
        .iter()
        .map(|(value, count)| format!("0x{value:04x}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_string_counts(counts: &BTreeMap<String, usize>) -> String {
    if counts.is_empty() {
        return "-".to_string();
    }

    counts
        .iter()
        .map(|(value, count)| format!("{}:{}", escaped_text(value), count))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_min_max(min: Option<usize>, max: Option<usize>) -> String {
    match (min, max) {
        (Some(min), Some(max)) => format!("{min}..{max}"),
        _ => "-".to_string(),
    }
}

pub(crate) fn update_min_max(min: &mut Option<usize>, max: &mut Option<usize>, value: usize) {
    *min = Some(min.map_or(value, |min| min.min(value)));
    *max = Some(max.map_or(value, |max| max.max(value)));
}

pub(crate) fn count_tail_field(counts: &mut BTreeMap<u16, usize>, fields: &[u16], index: usize) {
    if let Some(value) = fields.get(index) {
        *counts.entry(*value).or_insert(0) += 1;
    }
}

pub(crate) fn has_style_hit(fields: &[u16], candidates: &[CliStyleCandidate]) -> bool {
    fields.iter().filter(|value| **value != 0).any(|value| {
        candidates.iter().any(|candidate| {
            candidate.id == *value as usize || candidate.record_index == *value as usize
        })
    })
}

pub(crate) fn has_view_style_group_hit(fields: &[u16], groups: &[DocumentViewStyleGroup]) -> bool {
    fields
        .iter()
        .filter(|value| **value != 0)
        .any(|value| groups.iter().any(|group| group.id == *value))
}

pub(crate) fn format_view_style_group_hits(
    fields: &[u16],
    groups: &[DocumentViewStyleGroup],
) -> String {
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

pub(crate) fn format_style_id_hits(fields: &[u16], candidates: &[CliStyleCandidate]) -> String {
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

pub(crate) fn format_candidate_id_hit_counts(
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

pub(crate) fn format_view_style_group_hit_counts(
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

pub(crate) fn format_candidate_index_hit_counts(
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

pub(crate) fn format_style_index_hits(fields: &[u16], candidates: &[CliStyleCandidate]) -> String {
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

pub(crate) fn page_layout_slot_parts(
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

pub(crate) fn active_page_layout_slot_pairs(
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

pub(crate) fn page_layout_slot_part05_is_active(
    parts: &BTreeMap<u8, &StyleStreamSubrecordSummary>,
) -> bool {
    parts
        .get(&0x05)
        .and_then(|subrecord| subrecord.payload().first().copied())
        .is_some_and(|byte| byte != 0)
}

pub(crate) fn format_page_layout_slot_pairs(pairs: &BTreeSet<(u8, u8)>) -> String {
    if pairs.is_empty() {
        return "-".to_string();
    }
    pairs
        .iter()
        .map(|(left, right)| format!("0x{left:02x}/0x{right:02x}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_page_layout_slot_part(
    parts: &BTreeMap<u8, &StyleStreamSubrecordSummary>,
    part: u8,
) -> String {
    parts
        .get(&part)
        .map(|subrecord| bytes_to_hex(subrecord.payload()))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_page_layout_slot_part_first(
    parts: &BTreeMap<u8, &StyleStreamSubrecordSummary>,
    part: u8,
) -> String {
    parts
        .get(&part)
        .and_then(|subrecord| subrecord.payload().first().copied())
        .map(|byte| format!("0x{byte:02x}"))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_page_layout_slot_part_nonzero(
    parts: &BTreeMap<u8, &StyleStreamSubrecordSummary>,
    part: u8,
) -> String {
    parts
        .get(&part)
        .and_then(|subrecord| subrecord.payload().first().copied())
        .map(|byte| (byte != 0).to_string())
        .unwrap_or_else(|| "-".to_string())
}
