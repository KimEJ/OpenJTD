use std::collections::{BTreeMap, BTreeSet};

use rjtd_core::container::{EntryKind, inspect_cfb_entries, read_cfb_stream};

use super::object_stream_support::*;
use super::support::*;
use super::text_position_count_support::{format_be16_hex_fields, read_be16_fields};

pub(crate) const FDM_INDEX_HEADER_BYTES: usize = 20;
pub(crate) const FDM_INDEX_ENTRY_BYTES: usize = 22;
pub(crate) const FDM_INDEX_DECLARED_COUNT_OFFSET: usize = 18;
pub(crate) const FDM_INDEX_HEADER_V1: &str = "fdm-index-v1";

pub(crate) fn fdm_vector_path_for_index(index_path: &str) -> Option<String> {
    index_path
        .strip_suffix("/FDMIndex")
        .map(|prefix| format!("{prefix}/FDMVector"))
}

pub(crate) fn fdm_index_declared_count(index_stream: &[u8]) -> Option<usize> {
    read_be16_candidate(index_stream, FDM_INDEX_DECLARED_COUNT_OFFSET).map(usize::from)
}

pub(crate) fn fdm_index_trailing_bytes(index_stream: &[u8]) -> usize {
    index_stream.len().saturating_sub(FDM_INDEX_HEADER_BYTES) % FDM_INDEX_ENTRY_BYTES
}

pub(crate) fn fdm_index_header_family(index_stream: &[u8]) -> &'static str {
    if index_stream.starts_with(&[0x03, 0x0b, 0x00, 0x01]) {
        FDM_INDEX_HEADER_V1
    } else {
        "unknown-header"
    }
}

pub(crate) fn format_fdm_index_header_u16(index_stream: &[u8]) -> String {
    format_be16_hex_fields(&index_stream[..index_stream.len().min(FDM_INDEX_HEADER_BYTES)])
}

pub(crate) fn parse_fdm_index_entries(
    index_stream: &[u8],
    vector_len: usize,
) -> Vec<FdmIndexEntry> {
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
pub(crate) struct FdmIndexEntryStats {
    pub(crate) rows: usize,
    pub(crate) valid_offsets: usize,
    pub(crate) invalid_offsets: usize,
    pub(crate) image_rows: usize,
    pub(crate) image_hits: usize,
    pub(crate) first_invalid_row: Option<usize>,
    pub(crate) first_invalid_offset: Option<usize>,
}

pub(crate) fn fdm_index_entry_stats(
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

pub(crate) fn fdm_index_shape_family(
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

pub(crate) fn fdm_index_row_scope(
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

pub(crate) fn fdm_index_row_role(entry: &FdmIndexEntry) -> &'static str {
    if entry.valid_vector_offset {
        "vector-segment"
    } else if fdm_index_row_is_coordinate_like(&entry.row) {
        "coordinate-like-invalid"
    } else {
        "invalid-vector-offset"
    }
}

pub(crate) fn fdm_index_row_is_coordinate_like(row: &[u8]) -> bool {
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

pub(crate) fn fdm_vector_segment(
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

pub(crate) fn fdm_segment_signature_hits(
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

pub(crate) fn fdm_relative_signature_hits(
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

pub(crate) fn classify_object_frame_reference_record(
    record: &ObjectFrameReferenceRecord,
) -> &'static str {
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

pub(crate) fn object_frame_row_suffix(
    record: &ObjectFrameReferenceRecord,
    len: usize,
) -> Option<&[u8]> {
    record.row.get(record.row.len().checked_sub(len)?..)
}

pub(crate) fn object_frame_row_prefix(
    record: &ObjectFrameReferenceRecord,
    suffix_len: usize,
) -> Option<&[u8]> {
    record.row.get(..record.row.len().checked_sub(suffix_len)?)
}

pub(crate) fn find_object_frame_suffix_match<'a>(
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

pub(crate) fn readable_cfb_streams(data: &[u8]) -> Result<BTreeMap<String, Vec<u8>>, String> {
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

pub(crate) fn object_reference_pattern_len(encoding: &str) -> usize {
    match encoding {
        "u16-le" | "u16-be" => 2,
        "u32-le" | "u32-be" => 4,
        _ => 1,
    }
}

pub(crate) fn object_reference_context(
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

pub(crate) fn read_le16_candidate(bytes: &[u8], offset: usize) -> Option<u16> {
    let bytes = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

pub(crate) fn read_be16_candidate(bytes: &[u8], offset: usize) -> Option<u16> {
    let bytes = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_be_bytes([bytes[0], bytes[1]]))
}

pub(crate) fn read_le32_candidate(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn read_be32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn read_i32_be_at(bytes: &[u8], offset: usize) -> Option<i32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(crate) fn format_usize_set(values: &BTreeSet<usize>) -> String {
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

pub(crate) fn format_string_set(values: &BTreeSet<String>) -> String {
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

pub(crate) fn format_frame_reference_record_candidates() -> String {
    OBJECT_FRAME_REFERENCE_RECORD_CANDIDATES
        .iter()
        .map(ObjectFrameReferenceRecordCandidate::name)
        .collect::<Vec<_>>()
        .join(",")
}
