use super::*;
use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct PageMarkRecordLineIndexMatch {
    pub(crate) normalized_scan_index: usize,
    pub(crate) header: PageMarkRecordHeader,
    pub(crate) offset_from_line_start: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct PageMarkRecordLineIndexRow {
    pub(crate) line_mark_record_index: usize,
    pub(crate) matches: Vec<PageMarkRecordLineIndexMatch>,
}

pub(crate) fn page_mark_record_line_index_rows(
    normalized_headers: &[PageMarkRecordHeader],
    line_mark_record_indexes: &[usize],
) -> Vec<PageMarkRecordLineIndexRow> {
    line_mark_record_indexes
        .iter()
        .copied()
        .map(|line_mark_record_index| {
            let matches = normalized_headers
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(normalized_scan_index, header)| {
                    let line_start = header.line_start as usize;
                    let line_end = header.line_end as usize;
                    (line_start <= line_mark_record_index && line_mark_record_index <= line_end)
                        .then(|| PageMarkRecordLineIndexMatch {
                            normalized_scan_index,
                            header,
                            offset_from_line_start: line_mark_record_index - line_start,
                        })
                })
                .collect();
            PageMarkRecordLineIndexRow {
                line_mark_record_index,
                matches,
            }
        })
        .collect()
}

/// Literal size comparison between the observed `/PageMark` raw line extent
/// (`max(lineEnd) + 1`) and the `/LineMark` declared record count.
///
/// The two carry different weight. A negative value means the raw line field
/// takes values above the last declared record ordinal, which refutes reading
/// the raw line field as a `/LineMark` record ordinal in that file. A positive
/// value only means the observed raw line ranges stop short of the declared
/// records, so containment cannot reach the top ordinals; that is incomplete
/// coverage, not a refutation. Across the 50 local files that expose both a
/// normalized `/PageMark` record and a `/LineMark` declared record count, the
/// value is never `0`: 35 are negative and 15 are exactly `1`.
pub(crate) fn line_mark_declared_record_count_minus_page_mark_line_extent(
    normalized_headers: &[PageMarkRecordHeader],
    line_mark_bytes: &[u8],
) -> Option<i64> {
    let extent = normalized_headers
        .iter()
        .map(|header| i64::from(header.line_end) + 1)
        .max()?;
    Some(line_mark_declared_record_count(line_mark_bytes)? as i64 - extent)
}

/// Literal decomposition of the observed `/PageMark` raw line extent against two
/// `/LineMark` be-delta header fields, reported only for streams whose normalized
/// record walk contains exactly one flags-high `0x0005` record.
///
/// The gate is what makes the numbers mean anything. Over the 54 readable local
/// `/PageMark` streams, 27 carry exactly one such record and all 27 satisfy every
/// equality below. The gate returns no decomposition for the other 27 streams;
/// individual boundary coincidences do occur there, so the arithmetic is not a
/// format-wide identity. Nothing here decodes what a raw line is, what the flags
/// select, or what unit `span_after_flags_high_five` measures.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PageMarkRawLineExtentLineMarkDecomposition {
    pub(crate) flags_high_five_scan_index: usize,
    pub(crate) first_record_line_end_plus_one: u32,
    pub(crate) line_mark_header_u16_at_12: u16,
    pub(crate) flags_high_five_line_end_plus_one: u32,
    pub(crate) line_mark_declared_record_count: usize,
    pub(crate) span_after_flags_high_five: u32,
    pub(crate) line_extent: u32,
}

impl PageMarkRawLineExtentLineMarkDecomposition {
    pub(crate) fn first_record_boundary_equals_line_mark_header_u16_at_12(self) -> bool {
        u64::from(self.first_record_line_end_plus_one) == u64::from(self.line_mark_header_u16_at_12)
    }

    pub(crate) fn flags_high_five_boundary_equals_declared_record_count_minus_two(self) -> bool {
        self.line_mark_declared_record_count
            .checked_sub(2)
            .and_then(|value| u64::try_from(value).ok())
            == Some(u64::from(self.flags_high_five_line_end_plus_one))
    }

    pub(crate) fn line_extent_equals_declared_record_count_plus_span_minus_one(self) -> bool {
        u64::try_from(self.line_mark_declared_record_count)
            .ok()
            .and_then(|count| count.checked_add(u64::from(self.span_after_flags_high_five)))
            .and_then(|sum| sum.checked_sub(1))
            == Some(u64::from(self.line_extent))
    }

    pub(crate) fn all_equalities_hold(self) -> bool {
        self.first_record_boundary_equals_line_mark_header_u16_at_12()
            && self.flags_high_five_boundary_equals_declared_record_count_minus_two()
            && self.line_extent_equals_declared_record_count_plus_span_minus_one()
    }
}

pub(crate) fn page_mark_raw_line_extent_line_mark_decomposition(
    normalized_headers: &[PageMarkRecordHeader],
    line_mark_bytes: &[u8],
) -> Option<PageMarkRawLineExtentLineMarkDecomposition> {
    let mut flags_high_five = normalized_headers
        .iter()
        .enumerate()
        .filter(|(_, header)| (header.flags >> 16) as u16 == 0x0005);
    let (flags_high_five_scan_index, flags_high_five_header) = flags_high_five.next()?;
    if flags_high_five.next().is_some() {
        return None;
    }
    let next_header = normalized_headers.get(flags_high_five_scan_index + 1)?;
    let first_record_line_end_plus_one = normalized_headers.first()?.line_end.checked_add(1)?;
    let flags_high_five_line_end_plus_one = flags_high_five_header.line_end.checked_add(1)?;
    let span_after_flags_high_five = next_header
        .line_end
        .checked_sub(next_header.line_start)?
        .checked_add(1)?;
    let line_extent = normalized_headers.iter().try_fold(0, |extent, header| {
        Some(extent.max(header.line_end.checked_add(1)?))
    })?;
    Some(PageMarkRawLineExtentLineMarkDecomposition {
        flags_high_five_scan_index,
        first_record_line_end_plus_one,
        line_mark_header_u16_at_12: read_be16_at(
            line_mark_bytes,
            LINE_MARK_BE_DELTA_HEADER_U16_12_OFFSET,
        )?,
        flags_high_five_line_end_plus_one,
        line_mark_declared_record_count: line_mark_declared_record_count(line_mark_bytes)?,
        span_after_flags_high_five,
        line_extent,
    })
}

fn push_page_mark_raw_line_extent_decomposition_json(
    output: &mut String,
    decomposition: Option<PageMarkRawLineExtentLineMarkDecomposition>,
) {
    let Some(decomposition) = decomposition else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"source\":\"/PageMark normalized records+/LineMark be-delta header\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"selectionReady\":false");
    output.push_str(",\"gate\":\"exactly-one-normalized-record-with-flags-high-0x0005\"");
    output.push_str(",\"flagsHighFiveScanIndex\":");
    output.push_str(&decomposition.flags_high_five_scan_index.to_string());
    output.push_str(",\"firstRecordLineEndPlusOne\":");
    output.push_str(&decomposition.first_record_line_end_plus_one.to_string());
    output.push_str(",\"lineMarkHeaderU16At12\":");
    output.push_str(&decomposition.line_mark_header_u16_at_12.to_string());
    output.push_str(",\"firstRecordBoundaryEqualsLineMarkHeaderU16At12\":");
    output.push_str(json_bool(
        decomposition.first_record_boundary_equals_line_mark_header_u16_at_12(),
    ));
    output.push_str(",\"flagsHighFiveLineEndPlusOne\":");
    output.push_str(&decomposition.flags_high_five_line_end_plus_one.to_string());
    output.push_str(",\"lineMarkDeclaredRecordCount\":");
    output.push_str(&decomposition.line_mark_declared_record_count.to_string());
    output.push_str(",\"flagsHighFiveBoundaryEqualsDeclaredRecordCountMinusTwo\":");
    output.push_str(json_bool(
        decomposition.flags_high_five_boundary_equals_declared_record_count_minus_two(),
    ));
    output.push_str(",\"spanAfterFlagsHighFive\":");
    output.push_str(&decomposition.span_after_flags_high_five.to_string());
    output.push_str(",\"lineExtent\":");
    output.push_str(&decomposition.line_extent.to_string());
    output.push_str(",\"lineExtentEqualsDeclaredRecordCountPlusSpanMinusOne\":");
    output.push_str(json_bool(
        decomposition.line_extent_equals_declared_record_count_plus_span_minus_one(),
    ));
    output.push_str(",\"allEqualitiesHold\":");
    output.push_str(json_bool(decomposition.all_equalities_hold()));
    output.push_str(",\"blockedReasons\":[\"page-mark-raw-line-unit-role-undecoded\",\"span-after-flags-high-five-role-and-unit-undecoded\",\"line-mark-header-u16-at-12-role-undecoded\"]");
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-raw-line-extent-decomposition-diagnostic-only\"",
    );
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"page-mark-raw-line-extent-terms-carry-no-decoded-page-space-y\"}",
    );
}

pub(crate) fn push_table_grid_page_mark_variable_record_normalization_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        output.push_str("null");
        return;
    };
    let legacy_headers = page_mark_record_headers(bytes);
    let normalized_headers = page_mark_normalized_record_headers(bytes);
    if normalized_headers.is_empty() {
        output.push_str("null");
        return;
    }

    let legacy_offsets = legacy_headers
        .iter()
        .map(|header| header.offset)
        .collect::<BTreeSet<_>>();
    let normalized_only = normalized_headers
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, header)| !legacy_offsets.contains(&header.offset))
        .collect::<Vec<_>>();
    let normalized_tail_byte_count = (0..normalized_headers.len())
        .filter_map(|scan_index| {
            page_mark_record_tail_range(&normalized_headers, scan_index, bytes.len())
        })
        .map(|range| range.len())
        .sum::<usize>();
    let normalized_partition_complete = normalized_headers
        .first()
        .is_some_and(|header| header.offset == 12)
        && PAGE_MARK_RECORD_HEADER_BYTES * normalized_headers.len() + normalized_tail_byte_count
            == bytes.len().saturating_sub(12);
    let distinct_indexes = normalized_headers
        .iter()
        .map(|header| header.index)
        .collect::<BTreeSet<_>>();
    let duplicate_index_count = normalized_headers
        .len()
        .saturating_sub(distinct_indexes.len());
    let overlapping_adjacent_line_domain_count = normalized_headers
        .windows(2)
        .filter(|pair| pair[1].line_start <= pair[0].line_end)
        .count();
    let flags_high_u16_values = normalized_headers
        .iter()
        .map(|header| (header.flags >> 16) as u16)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let line_mark_record_indexes =
        table_grid_previous_row_span_line_mark_record_indexes(document, candidate);
    // Raw extent of the observed line fields, reported as the two literal
    // aggregates it is rather than as a decoded domain: the maximum `lineEnd + 1`
    // and the minimum `lineStart`. Whether those bound a /LineMark ordinal space
    // is not settled here.
    let max_line_end_plus_one = normalized_headers
        .iter()
        .map(|header| header.line_end as usize + 1)
        .max();
    let min_line_start = normalized_headers
        .iter()
        .map(|header| header.line_start as usize)
        .min();
    let line_mark_bytes = raw_stream_bytes(document, LINE_MARK_PATH);
    let line_mark_declared_record_count = line_mark_bytes.and_then(line_mark_declared_record_count);
    let max_line_end_plus_one_equals_line_mark_record_count = max_line_end_plus_one
        .zip(line_mark_declared_record_count)
        .is_some_and(|(extent, count)| extent == count);
    // Only the negative direction refutes anything: raw line values above the last
    // declared record ordinal cannot be /LineMark record ordinals, so unanimous
    // containment below stays a selection rather than an identity proof.
    let declared_record_count_minus_line_extent = line_mark_bytes.and_then(|bytes| {
        line_mark_declared_record_count_minus_page_mark_line_extent(&normalized_headers, bytes)
    });
    let line_extent_exceeds_declared_record_count =
        declared_record_count_minus_line_extent.is_some_and(|delta| delta < 0);
    let mut relationship_blocked_reasons = vec![
        "line-mark-record-index-page-line-candidate-selected-by-containment",
        "page-mark-raw-line-range-role-unproven",
        "line-domain-to-page-space-y-transform-required",
    ];
    if line_extent_exceeds_declared_record_count {
        relationship_blocked_reasons
            .push("page-mark-raw-line-extent-exceeds-line-mark-declared-record-count");
    }
    let line_index_rows =
        page_mark_record_line_index_rows(&normalized_headers, &line_mark_record_indexes);
    let covered_row_count = line_index_rows
        .iter()
        .filter(|row| !row.matches.is_empty())
        .count();
    let uniquely_covered_row_count = line_index_rows
        .iter()
        .filter(|row| row.matches.len() == 1)
        .count();
    let uniquely_covered_scan_indexes = line_index_rows
        .iter()
        .filter_map(|row| (row.matches.len() == 1).then_some(row.matches[0].normalized_scan_index))
        .collect::<BTreeSet<_>>();
    let all_rows_uniquely_covered =
        !line_index_rows.is_empty() && uniquely_covered_row_count == line_index_rows.len();
    let all_rows_share_one_normalized_record =
        all_rows_uniquely_covered && uniquely_covered_scan_indexes.len() == 1;

    output
        .push_str("{\"source\":\"/PageMark observed variable raw record normalization+/LineMark\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"selectionReady\":false");
    output.push_str(",\"acceptedFlagsHighU16Values\":");
    push_u16_array_json(output, &PAGE_MARK_NORMALIZED_VIEW_FLAGS_HIGH_U16_VALUES);
    output.push_str(",\"observedFlagsHighU16Values\":");
    push_u16_array_json(output, &flags_high_u16_values);
    output.push_str(",\"flagsLowU16Semantics\":\"opaque-source-field\"");
    output.push_str(",\"legacyExactRecordHeaderCount\":");
    output.push_str(&legacy_headers.len().to_string());
    output.push_str(",\"normalizedRecordHeaderCount\":");
    output.push_str(&normalized_headers.len().to_string());
    output.push_str(",\"normalizedOnlyRecordHeaderCount\":");
    output.push_str(&normalized_only.len().to_string());
    output.push_str(",\"normalizedOnlyRecordHeaders\":[");
    for (index, (scan_index, header)) in normalized_only.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"normalizedScanIndex\":");
        output.push_str(&scan_index.to_string());
        output.push_str(",\"byteOffset\":");
        output.push_str(&header.offset.to_string());
        output.push_str(",\"index\":");
        output.push_str(&header.index.to_string());
        output.push_str(",\"flags\":");
        output.push_str(&header.flags.to_string());
        output.push_str(",\"flagsHex\":");
        output.push_str(&json_string(&format!("0x{:08x}", header.flags)));
        output.push_str(",\"lineStart\":");
        output.push_str(&header.line_start.to_string());
        output.push_str(",\"lineEnd\":");
        output.push_str(&header.line_end.to_string());
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"normalizedTailByteCount\":");
    output.push_str(&normalized_tail_byte_count.to_string());
    output.push_str(",\"normalizedPartitionComplete\":");
    output.push_str(json_bool(normalized_partition_complete));
    output.push_str(",\"duplicateIndexCount\":");
    output.push_str(&duplicate_index_count.to_string());
    output.push_str(",\"overlappingAdjacentLineDomainCount\":");
    output.push_str(&overlapping_adjacent_line_domain_count.to_string());
    output.push_str(",\"legacyConsumersUnchanged\":true");
    output.push_str(",\"lineMarkRecordIndexToPageLineRelationship\":{\"source\":\"/LineMark record ordinal contained by normalized /PageMark raw line range\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"selectionReady\":false");
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &line_mark_record_indexes);
    output.push_str(",\"rowCount\":");
    output.push_str(&line_index_rows.len().to_string());
    output.push_str(",\"coveredRowCount\":");
    output.push_str(&covered_row_count.to_string());
    output.push_str(",\"uniquelyCoveredRowCount\":");
    output.push_str(&uniquely_covered_row_count.to_string());
    output.push_str(",\"allRowsUniquelyCovered\":");
    output.push_str(json_bool(all_rows_uniquely_covered));
    output.push_str(",\"allRowsShareOneNormalizedRecord\":");
    output.push_str(json_bool(all_rows_share_one_normalized_record));
    output.push_str(",\"rows\":[");
    for (row_index, row) in line_index_rows.iter().enumerate() {
        if row_index > 0 {
            output.push(',');
        }
        output.push_str("{\"lineMarkRecordIndex\":");
        output.push_str(&row.line_mark_record_index.to_string());
        output.push_str(",\"containingRecordCount\":");
        output.push_str(&row.matches.len().to_string());
        output.push_str(",\"matches\":[");
        for (match_index, match_) in row.matches.iter().enumerate() {
            if match_index > 0 {
                output.push(',');
            }
            output.push_str("{\"normalizedScanIndex\":");
            output.push_str(&match_.normalized_scan_index.to_string());
            output.push_str(",\"recordIndex\":");
            output.push_str(&match_.header.index.to_string());
            output.push_str(",\"recordLineStart\":");
            output.push_str(&match_.header.line_start.to_string());
            output.push_str(",\"recordLineEnd\":");
            output.push_str(&match_.header.line_end.to_string());
            output.push_str(",\"offsetFromRecordLineStart\":");
            output.push_str(&match_.offset_from_line_start.to_string());
            output.push('}');
        }
        output.push_str("]}");
    }
    output.push(']');
    output.push_str(",\"maxLineEndPlusOne\":");
    push_option_usize_json(output, max_line_end_plus_one);
    output.push_str(",\"minLineStart\":");
    push_option_usize_json(output, min_line_start);
    output.push_str(",\"lineMarkDeclaredRecordCount\":");
    push_option_usize_json(output, line_mark_declared_record_count);
    output.push_str(",\"maxLineEndPlusOneEqualsLineMarkDeclaredRecordCount\":");
    output.push_str(json_bool(
        max_line_end_plus_one_equals_line_mark_record_count,
    ));
    output.push_str(",\"lineMarkDeclaredRecordCountMinusMaxLineEndPlusOne\":");
    match declared_record_count_minus_line_extent {
        Some(delta) => output.push_str(&delta.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"identityCandidateSelectedByContainment\":");
    output.push_str(json_bool(all_rows_uniquely_covered));
    output.push_str(",\"identityCandidateRefutedByRawLineExtent\":");
    output.push_str(json_bool(line_extent_exceeds_declared_record_count));
    output.push_str(",\"identityCandidateIndependentlyProven\":false");
    output.push_str(",\"pageSpaceYDecoded\":false");
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &relationship_blocked_reasons);
    output.push_str(",\"rawLineExtentDecomposition\":");
    push_page_mark_raw_line_extent_decomposition_json(
        output,
        line_mark_bytes.and_then(|bytes| {
            page_mark_raw_line_extent_line_mark_decomposition(&normalized_headers, bytes)
        }),
    );
    output.push_str(
        ",\"renderPromotionContribution\":\"normalized-record-line-domain-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"line-mark-record-index-to-page-line-mapping-not-independently-proven\"}");
    output.push_str(",\"blockedReasons\":[\"page-mark-variable-record-flags-semantics-undecoded\",\"page-mark-duplicate-and-overlapping-record-roles-unproven\",\"normalized-record-view-not-admitted-to-legacy-consumers\"]");
    output.push_str(",\"renderPromotionContribution\":\"page-mark-variable-record-normalization-diagnostic-only\"");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"page-mark-variable-record-semantics-undecoded\"}",
    );
}
