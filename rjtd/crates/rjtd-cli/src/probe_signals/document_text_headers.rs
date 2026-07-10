use rjtd_core::document_text::read_document_text_payload;
use rjtd_model::{TableCandidate, TableCandidateColumnSegment, TextCountRangeOverlapBasis};

use super::TableLineHeaderRowSignal;

const DOCUMENT_TEXT_LINE_HEADER_BYTES: usize = 24;

pub(super) fn document_text_table_line_header_rows(
    bytes: &[u8],
    candidates: &[TableCandidate],
) -> Vec<TableLineHeaderRowSignal> {
    let Ok(payload) = read_document_text_payload(bytes) else {
        return Vec::new();
    };
    let document_text = payload.bytes();
    let mut rows = Vec::new();
    for candidate in candidates
        .iter()
        .filter(|candidate| candidate.kind() == "documentTextControlRunTableCandidate")
    {
        for interval in candidate.intervals() {
            let headers = line_headers_for_interval(
                document_text,
                candidate.basis(),
                interval.source_start(),
                interval.source_end(),
            );
            if headers.is_empty() {
                continue;
            }
            let matched_headers = interval
                .column_segments()
                .iter()
                .filter_map(|segment| {
                    cell_line_header(candidate.basis(), &headers, segment).copied()
                })
                .collect::<Vec<_>>();
            rows.push(TableLineHeaderRowSignal {
                candidate_index: candidate.index(),
                row_index: interval.index(),
                source_start: interval.source_start(),
                source_end: interval.source_end(),
                raw_offsets: headers.iter().map(|header| header.offset_units).collect(),
                raw_extents: headers.iter().map(|header| header.extent_units).collect(),
                cell_offsets: matched_headers
                    .iter()
                    .map(|header| header.offset_units)
                    .collect(),
                cell_extents: matched_headers
                    .iter()
                    .map(|header| header.extent_units)
                    .collect(),
                font_size_units: matched_headers
                    .iter()
                    .map(|header| header.font_size_units)
                    .collect(),
            });
        }
    }
    rows
}

fn line_headers_for_interval(
    bytes: &[u8],
    basis: TextCountRangeOverlapBasis,
    source_start: usize,
    source_end: usize,
) -> Vec<DocumentTextLineHeader> {
    let Some((mut offset, end)) = table_interval_byte_range(bytes, basis, source_start, source_end)
    else {
        return Vec::new();
    };
    if offset % 2 != 0 {
        offset += 1;
    }
    let mut headers = Vec::new();
    while offset + DOCUMENT_TEXT_LINE_HEADER_BYTES <= end {
        if let Some(header) = document_text_line_header_at(bytes, offset) {
            headers.push(header);
            offset = header.end;
        } else {
            offset += 2;
        }
    }
    headers
}

fn table_interval_byte_range(
    bytes: &[u8],
    basis: TextCountRangeOverlapBasis,
    source_start: usize,
    source_end: usize,
) -> Option<(usize, usize)> {
    if source_start >= source_end {
        return None;
    }
    let (byte_start, byte_end) = match basis {
        TextCountRangeOverlapBasis::Byte => (source_start, source_end),
        TextCountRangeOverlapBasis::Unit => {
            (source_start.checked_mul(2)?, source_end.checked_mul(2)?)
        }
    };
    if byte_start >= bytes.len() || byte_start >= byte_end {
        return None;
    }
    Some((byte_start, byte_end.min(bytes.len())))
}

fn cell_line_header<'a>(
    basis: TextCountRangeOverlapBasis,
    headers: &'a [DocumentTextLineHeader],
    segment: &TableCandidateColumnSegment,
) -> Option<&'a DocumentTextLineHeader> {
    let segment_start = table_source_offset_to_units(basis, segment.source_start()?);
    let segment_end = table_source_offset_to_units(basis, segment.source_end()?);

    headers
        .iter()
        .filter(|header| header.end / 2 <= segment_start)
        .min_by_key(|header| segment_start.saturating_sub(header.end / 2))
        .or_else(|| {
            headers
                .iter()
                .filter(|header| {
                    ranges_overlap_half_open(
                        header.start / 2,
                        header.end / 2,
                        segment_start,
                        segment_end,
                    )
                })
                .min_by_key(|header| {
                    segment_start
                        .abs_diff(header.start / 2)
                        .min(segment_end.abs_diff(header.end / 2))
                })
        })
        .or_else(|| {
            headers.iter().min_by_key(|header| {
                segment_start
                    .abs_diff(header.start / 2)
                    .min(segment_start.abs_diff(header.end / 2))
            })
        })
}

fn table_source_offset_to_units(basis: TextCountRangeOverlapBasis, offset: usize) -> usize {
    match basis {
        TextCountRangeOverlapBasis::Byte => offset / 2,
        TextCountRangeOverlapBasis::Unit => offset,
    }
}

fn ranges_overlap_half_open(
    start: usize,
    end: usize,
    other_start: usize,
    other_end: usize,
) -> bool {
    start < other_end && other_start < end
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DocumentTextLineHeader {
    offset_units: u16,
    extent_units: u16,
    font_size_units: u16,
    start: usize,
    end: usize,
}

fn document_text_line_header_at(bytes: &[u8], offset: usize) -> Option<DocumentTextLineHeader> {
    if offset + DOCUMENT_TEXT_LINE_HEADER_BYTES > bytes.len()
        || !bytes[offset..].starts_with(&[0x00, 0x1c, 0x00, 0x30])
    {
        return None;
    }
    let mut words = [0u16; 12];
    for (index, chunk) in bytes[offset..offset + DOCUMENT_TEXT_LINE_HEADER_BYTES]
        .chunks_exact(2)
        .enumerate()
    {
        words[index] = u16::from_be_bytes([chunk[0], chunk[1]]);
    }
    if words[2] == 0
        || words[6] != 0x00ff
        || words[7] != 0
        || words[9] != 0
        || words[10] != 0x0030
        || words[11] != 0x001f
    {
        return None;
    }
    Some(DocumentTextLineHeader {
        offset_units: words[4],
        extent_units: words[5],
        font_size_units: words[2],
        start: offset,
        end: offset + DOCUMENT_TEXT_LINE_HEADER_BYTES,
    })
}
