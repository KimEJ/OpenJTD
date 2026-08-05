use super::*;
use crate::*;

pub(crate) fn table_grid_source_only_stride_row_coverage_summary(
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Option<TableGridSourceOnlyStrideRowCoverageSummary> {
    let source_layout = source_layout?;
    let stride = source_layout.line_mark_page_origin_stride.as_ref()?;
    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate)?;
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        return None;
    }
    let rows =
        table_grid_line_mark_row_gap_sequence_rows(candidate, &sibling, &line_mark_intervals);
    if rows.is_empty() {
        return None;
    }

    let mut matched_row_count = 0usize;
    let mut line_mark_record_indexes = Vec::new();
    let mut row_span_units = Vec::new();
    let mut line_mark_span_units = Vec::new();
    let mut row_span_residual_units = Vec::new();
    for row in &rows {
        let row_span = row
            .row_source_end_units
            .saturating_sub(row.row_source_start_units);
        row_span_units.push(row_span);
        if let Some(previous) = row.previous_line_mark {
            let line_mark_span = line_mark_interval_span_units(previous);
            line_mark_record_indexes.push(previous.record_index);
            line_mark_span_units.push(line_mark_span);
            row_span_residual_units.push(line_mark_span as i32 - row_span as i32);
            if line_mark_span == row_span {
                matched_row_count += 1;
            }
        }
    }

    let line_mark_record_stride = uniform_usize_stride(&line_mark_record_indexes);
    let all_rows_covered = !rows.is_empty()
        && matched_row_count == rows.len()
        && line_mark_record_indexes.len() == rows.len();
    let matches_stride_candidate_record_indexes =
        line_mark_record_indexes.as_slice() == stride.line_mark_record_indexes.as_slice();

    Some(TableGridSourceOnlyStrideRowCoverageSummary {
        candidate_row_count: rows.len(),
        matched_row_count,
        all_rows_covered,
        line_mark_record_selection: "previous-compact-row-span-record",
        line_mark_record_indexes,
        uniform_line_mark_record_stride: line_mark_record_stride.is_some(),
        line_mark_record_stride,
        matches_stride_candidate_record_indexes,
        row_span_units,
        line_mark_span_units,
        row_span_residual_units,
    })
}

pub(crate) fn table_grid_line_mark_row_boundary_alignment_summary(
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Option<TableGridLineMarkRowBoundaryAlignmentSummary> {
    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate)?;
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        return None;
    }
    let rows =
        table_grid_line_mark_row_gap_sequence_rows(candidate, &sibling, &line_mark_intervals);
    if rows.is_empty() {
        return None;
    }
    let stride_record_indexes = source_layout
        .and_then(|layout| layout.line_mark_page_origin_stride.as_ref())
        .map(|stride| stride.line_mark_record_indexes.as_slice());

    Some(TableGridLineMarkRowBoundaryAlignmentSummary {
        candidate_row_count: rows.len(),
        selected_spacing_record_alignment: table_grid_line_mark_row_boundary_alignment_family(
            &rows,
            "selected-spacing-records",
            "selected-record-overlaps-row-and-matches-post-row-gap-span",
            table_grid_selected_line_mark_for_boundary_alignment,
            stride_record_indexes,
        ),
        previous_row_span_record_alignment: table_grid_line_mark_row_boundary_alignment_family(
            &rows,
            "previous-row-span-records",
            "previous-record-span-equals-compact-row-span",
            table_grid_previous_line_mark_for_boundary_alignment,
            stride_record_indexes,
        ),
        next_record_alignment: table_grid_line_mark_row_boundary_alignment_family(
            &rows,
            "next-records",
            "next-record-neighboring-row-span-candidate",
            table_grid_next_line_mark_for_boundary_alignment,
            stride_record_indexes,
        ),
    })
}

pub(crate) fn table_grid_selected_line_mark_for_boundary_alignment(
    row: &TableGridLineMarkRowGapSequenceRow,
) -> Option<ShanaiLanLineMarkInterval> {
    Some(row.selected_line_mark)
}

pub(crate) fn table_grid_previous_line_mark_for_boundary_alignment(
    row: &TableGridLineMarkRowGapSequenceRow,
) -> Option<ShanaiLanLineMarkInterval> {
    row.previous_line_mark
}

pub(crate) fn table_grid_next_line_mark_for_boundary_alignment(
    row: &TableGridLineMarkRowGapSequenceRow,
) -> Option<ShanaiLanLineMarkInterval> {
    row.next_line_mark
}

pub(crate) fn table_grid_line_mark_row_boundary_alignment_family(
    rows: &[TableGridLineMarkRowGapSequenceRow],
    family: &'static str,
    span_interpretation: &'static str,
    select_line_mark: fn(&TableGridLineMarkRowGapSequenceRow) -> Option<ShanaiLanLineMarkInterval>,
    stride_record_indexes: Option<&[usize]>,
) -> Option<TableGridLineMarkRowBoundaryAlignmentFamily> {
    let mut alignment_rows = Vec::new();
    for row in rows {
        let Some(line_mark) = select_line_mark(row) else {
            continue;
        };
        let start_residual_units =
            signed_usize_delta_i32(line_mark.unit_start, row.row_source_start_units);
        let end_residual_units =
            signed_usize_delta_i32(line_mark.unit_end, row.row_source_end_units);
        let line_mark_span_units = line_mark_interval_span_units(line_mark);
        let row_span_units = row
            .row_source_end_units
            .saturating_sub(row.row_source_start_units);
        let span_residual_units = signed_usize_delta_i32(line_mark_span_units, row_span_units);
        alignment_rows.push(TableGridLineMarkRowBoundaryAlignmentRow {
            compact_row_index: row.compact_row_index,
            sparse_row_index: row.sparse_row_index,
            source_interval_index: row.source_interval_index,
            line_mark_record_index: line_mark.record_index,
            row_source_start_units: row.row_source_start_units,
            row_source_end_units: row.row_source_end_units,
            line_mark_start_units: line_mark.unit_start,
            line_mark_end_units: line_mark.unit_end,
            start_residual_units,
            end_residual_units,
            span_residual_units,
            exact_boundary_aligned: start_residual_units == 0 && end_residual_units == 0,
        });
    }
    if alignment_rows.is_empty() {
        return None;
    }

    let record_indexes = alignment_rows
        .iter()
        .map(|row| row.line_mark_record_index)
        .collect::<Vec<_>>();
    let line_mark_record_stride = uniform_usize_stride(&record_indexes);
    let row_source_start_units = alignment_rows
        .iter()
        .map(|row| row.row_source_start_units)
        .collect::<Vec<_>>();
    let row_source_end_units = alignment_rows
        .iter()
        .map(|row| row.row_source_end_units)
        .collect::<Vec<_>>();
    let line_mark_start_units = alignment_rows
        .iter()
        .map(|row| row.line_mark_start_units)
        .collect::<Vec<_>>();
    let line_mark_end_units = alignment_rows
        .iter()
        .map(|row| row.line_mark_end_units)
        .collect::<Vec<_>>();
    let start_residual_units = alignment_rows
        .iter()
        .map(|row| row.start_residual_units)
        .collect::<Vec<_>>();
    let end_residual_units = alignment_rows
        .iter()
        .map(|row| row.end_residual_units)
        .collect::<Vec<_>>();
    let span_residual_units = alignment_rows
        .iter()
        .map(|row| row.span_residual_units)
        .collect::<Vec<_>>();
    let exact_boundary_match_count = alignment_rows
        .iter()
        .filter(|row| row.exact_boundary_aligned)
        .count();
    let exact_boundary_aligned = exact_boundary_match_count == alignment_rows.len();
    let stable_start_residual_units = single_i32_value(&start_residual_units);
    let stable_end_residual_units = single_i32_value(&end_residual_units);
    let stable_span_residual_units = single_i32_value(&span_residual_units);
    let row_boundary_offset_candidate_units = stable_start_residual_units
        .zip(stable_end_residual_units)
        .and_then(|(start_residual, end_residual)| {
            (start_residual == end_residual).then_some(start_residual)
        });
    let offset_normalized_start_residual_units = row_boundary_offset_candidate_units
        .map(|offset| {
            start_residual_units
                .iter()
                .map(|residual| residual - offset)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let offset_normalized_end_residual_units = row_boundary_offset_candidate_units
        .map(|offset| {
            end_residual_units
                .iter()
                .map(|residual| residual - offset)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let offset_normalized_exact_boundary_match_count = offset_normalized_start_residual_units
        .iter()
        .zip(&offset_normalized_end_residual_units)
        .filter(|(start, end)| **start == 0 && **end == 0)
        .count();
    let offset_normalized_exact_boundary_aligned = !alignment_rows.is_empty()
        && offset_normalized_exact_boundary_match_count == alignment_rows.len();
    let span_only_match =
        !exact_boundary_aligned && span_residual_units.iter().all(|residual| *residual == 0);
    let matches_stride_candidate_record_indexes =
        stride_record_indexes.is_some_and(|indexes| record_indexes.as_slice() == indexes);

    Some(TableGridLineMarkRowBoundaryAlignmentFamily {
        family,
        span_interpretation,
        row_count: alignment_rows.len(),
        record_indexes,
        uniform_line_mark_record_stride: line_mark_record_stride.is_some(),
        line_mark_record_stride,
        matches_stride_candidate_record_indexes,
        row_source_start_units,
        row_source_end_units,
        line_mark_start_units,
        line_mark_end_units,
        start_residual_units,
        end_residual_units,
        span_residual_units,
        exact_boundary_match_count,
        exact_boundary_aligned,
        start_residual_stable: stable_start_residual_units.is_some(),
        end_residual_stable: stable_end_residual_units.is_some(),
        span_residual_stable: stable_span_residual_units.is_some(),
        stable_start_residual_units,
        stable_end_residual_units,
        stable_span_residual_units,
        row_boundary_offset_candidate_units,
        offset_normalized_start_residual_units,
        offset_normalized_end_residual_units,
        offset_normalized_exact_boundary_match_count,
        offset_normalized_exact_boundary_aligned,
        span_only_match,
        rows: alignment_rows,
    })
}

pub(crate) fn table_grid_page_mark_subrecord_line_range_record_coverage_summary(
    document: &Document,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> Option<TableGridPageMarkSubrecordLineRangeRecordCoverageSummary> {
    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let page_mark = document.page_marks().first()?;
    let record_headers = page_mark_record_headers(page_mark_bytes);
    if record_headers.is_empty() {
        return None;
    }
    let stride = source_layout?.line_mark_page_origin_stride.as_ref()?;
    let previous_record_indexes = stride.line_mark_record_indexes.clone();
    let selected_record_indexes = subrecord_span_readiness
        .map(|readiness| readiness.selected_record_indexes.clone())
        .unwrap_or_default();
    if selected_record_indexes.is_empty() && previous_record_indexes.is_empty() {
        return None;
    }

    let candidates = page_mark_raw_subrecord_line_span_candidates(
        page_mark_bytes,
        &record_headers,
        page_mark_subrecord_line_range_max_candidate(page_mark, &record_headers),
    );
    if candidates.is_empty() {
        return None;
    }

    let (
        selected_covered_record_indexes,
        selected_containing_candidate_byte_offsets,
        selected_nearest_matches,
    ) = table_grid_page_mark_subrecord_line_range_record_matches(
        &selected_record_indexes,
        &candidates,
    );
    let (
        previous_covered_record_indexes,
        previous_containing_candidate_byte_offsets,
        previous_nearest_matches,
    ) = table_grid_page_mark_subrecord_line_range_record_matches(
        &previous_record_indexes,
        &candidates,
    );

    Some(TableGridPageMarkSubrecordLineRangeRecordCoverageSummary {
        candidate_count: candidates.len(),
        selected_coverage_complete: !selected_record_indexes.is_empty()
            && selected_covered_record_indexes.len() == selected_record_indexes.len(),
        previous_coverage_complete: !previous_record_indexes.is_empty()
            && previous_covered_record_indexes.len() == previous_record_indexes.len(),
        selected_record_indexes,
        previous_record_indexes,
        selected_covered_record_indexes,
        previous_covered_record_indexes,
        selected_containing_candidate_byte_offsets,
        previous_containing_candidate_byte_offsets,
        selected_nearest_matches,
        previous_nearest_matches,
    })
}

pub(crate) fn table_grid_page_mark_subrecord_line_range_record_matches(
    record_indexes: &[usize],
    candidates: &[PageMarkRawSubrecordLineSpanCandidate],
) -> (
    Vec<usize>,
    Vec<usize>,
    Vec<TableGridPageMarkSubrecordLineRangeRecordMatch>,
) {
    let mut covered_record_indexes = Vec::new();
    let mut containing_candidate_byte_offsets = Vec::new();
    let mut nearest_matches = Vec::new();
    for record_index in record_indexes {
        let containing = candidates.iter().find(|candidate| {
            page_mark_subrecord_line_range_contains_record(candidate, *record_index)
        });
        if let Some(candidate) = containing {
            covered_record_indexes.push(*record_index);
            containing_candidate_byte_offsets.push(candidate.byte_offset);
        }
        if let Some(nearest) = candidates.iter().min_by(|left, right| {
            page_mark_subrecord_line_range_record_distance(left, *record_index)
                .cmp(&page_mark_subrecord_line_range_record_distance(
                    right,
                    *record_index,
                ))
                .then_with(|| left.raw_record_scan_index.cmp(&right.raw_record_scan_index))
                .then_with(|| left.byte_offset.cmp(&right.byte_offset))
        }) {
            nearest_matches.push(TableGridPageMarkSubrecordLineRangeRecordMatch {
                record_index: *record_index,
                distance_units: page_mark_subrecord_line_range_record_distance(
                    nearest,
                    *record_index,
                ),
                candidate: *nearest,
            });
        }
    }
    (
        covered_record_indexes,
        containing_candidate_byte_offsets,
        nearest_matches,
    )
}

pub(crate) fn table_grid_stride_page_mark_entry_line_bounds_coverage_summary(
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Option<TableGridStridePageMarkEntryLineBoundsCoverageSummary> {
    let source_layout = source_layout?;
    let stride = source_layout.line_mark_page_origin_stride.as_ref()?;
    let line_offsets_from_page_start = stride
        .line_mark_record_indexes
        .iter()
        .map(|record_index| record_index.saturating_sub(stride.page_line_start))
        .collect::<Vec<_>>();
    let row_count_matches_stride_candidate =
        source_layout.row_count == stride.line_mark_record_indexes.len();
    let all_line_mark_records_within_page_mark_entry = !stride.line_mark_record_indexes.is_empty()
        && stride.line_mark_record_indexes.iter().all(|record_index| {
            stride.page_line_start <= *record_index && *record_index <= stride.page_line_end
        });
    let coverage_ready =
        row_count_matches_stride_candidate && all_line_mark_records_within_page_mark_entry;

    Some(TableGridStridePageMarkEntryLineBoundsCoverageSummary {
        candidate_row_count: source_layout.row_count,
        line_mark_record_indexes: stride.line_mark_record_indexes.clone(),
        record_stride: stride.record_stride,
        page_mark_entry_index: stride.page_mark_entry_index,
        page_index_candidate: stride.page_index_candidate,
        page_line_start: stride.page_line_start,
        page_line_end: stride.page_line_end,
        line_offsets_from_page_start,
        row_count_matches_stride_candidate,
        all_line_mark_records_within_page_mark_entry,
        coverage_ready,
    })
}

pub(crate) fn push_table_grid_source_only_stride_row_coverage_summary_json(
    output: &mut String,
    summary: Option<&TableGridSourceOnlyStrideRowCoverageSummary>,
) {
    let Some(summary) = summary else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/LineMark source unit ranges+table row source unit ranges\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateSpace\":\"documentTextSourceUnits\"");
    output.push_str(",\"policy\":\"previous-line-mark-record-span-equals-table-row-source-span\"");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&summary.candidate_row_count.to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&summary.matched_row_count.to_string());
    output.push_str(",\"allRowsCovered\":");
    output.push_str(if summary.all_rows_covered {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordSelection\":");
    output.push_str(&json_string(summary.line_mark_record_selection));
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &summary.line_mark_record_indexes);
    output.push_str(",\"uniformLineMarkRecordStride\":");
    output.push_str(if summary.uniform_line_mark_record_stride {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordStride\":");
    push_optional_usize_json(output, summary.line_mark_record_stride);
    output.push_str(",\"matchesStrideCandidateRecordIndexes\":");
    output.push_str(if summary.matches_stride_candidate_record_indexes {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSpanUnits\":");
    push_usize_array_json(output, &summary.row_span_units);
    output.push_str(",\"lineMarkSpanUnits\":");
    push_usize_array_json(output, &summary.line_mark_span_units);
    output.push_str(",\"rowSpanResidualUnits\":");
    push_i32_array_json(output, &summary.row_span_residual_units);
    output.push_str(",\"pageYTransformDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"source-only-line-mark-row-span-coverage\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if summary.all_rows_covered {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "line-mark-row-spans-do-not-cover-table-row-source-spans",
        ));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_line_mark_row_boundary_alignment_summary_json(
    output: &mut String,
    summary: Option<&TableGridLineMarkRowBoundaryAlignmentSummary>,
) {
    let Some(summary) = summary else {
        output.push_str("null");
        return;
    };

    output.push_str(
        "{\"source\":\"/LineMark source unit boundaries+table row source unit boundaries\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateSpace\":\"documentTextSourceUnits\"");
    output.push_str(",\"policy\":\"line-mark-start-end-compared-to-table-row-source-start-end\"");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&summary.candidate_row_count.to_string());
    let previous = summary.previous_row_span_record_alignment.as_ref();
    let row_boundary_offset_candidate_units =
        previous.and_then(|family| family.row_boundary_offset_candidate_units);
    output.push_str(",\"rowBoundaryOffsetCandidateFamily\":");
    if row_boundary_offset_candidate_units.is_some() {
        output.push_str(&json_string("previous-row-span-records"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rowBoundaryOffsetCandidateUnits\":");
    push_optional_i32_json(output, row_boundary_offset_candidate_units);
    output.push_str(",\"rowBoundaryOffsetCandidateStable\":");
    output.push_str(if row_boundary_offset_candidate_units.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowBoundaryOffsetCandidateRequiresTransform\":");
    output.push_str(
        if previous.is_some_and(|family| {
            family.row_boundary_offset_candidate_units.is_some() && !family.exact_boundary_aligned
        }) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"previousRowSpanRecordAlignmentOffsetNormalizedExact\":");
    output.push_str(
        if previous.is_some_and(|family| family.offset_normalized_exact_boundary_aligned) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"selectedSpacingRecordAlignment\":");
    push_table_grid_line_mark_row_boundary_alignment_family_json(
        output,
        summary.selected_spacing_record_alignment.as_ref(),
    );
    output.push_str(",\"previousRowSpanRecordAlignment\":");
    push_table_grid_line_mark_row_boundary_alignment_family_json(
        output,
        summary.previous_row_span_record_alignment.as_ref(),
    );
    output.push_str(",\"nextRecordAlignment\":");
    push_table_grid_line_mark_row_boundary_alignment_family_json(
        output,
        summary.next_record_alignment.as_ref(),
    );
    output.push_str(",\"pageYTransformDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"source-only-line-mark-row-boundary-alignment\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if previous.is_some_and(|family| family.exact_boundary_aligned) {
        output.push_str("null");
    } else if previous.is_some_and(|family| family.row_boundary_offset_candidate_units.is_some()) {
        output.push_str(&json_string(
            "line-mark-row-boundaries-require-source-offset-transform",
        ));
    } else {
        output.push_str(&json_string(
            "line-mark-row-boundaries-do-not-exactly-align-with-table-rows",
        ));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_line_mark_row_boundary_alignment_family_json(
    output: &mut String,
    family: Option<&TableGridLineMarkRowBoundaryAlignmentFamily>,
) {
    let Some(family) = family else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"family\":");
    output.push_str(&json_string(family.family));
    output.push_str(",\"spanInterpretation\":");
    output.push_str(&json_string(family.span_interpretation));
    output.push_str(",\"rowCount\":");
    output.push_str(&family.row_count.to_string());
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &family.record_indexes);
    output.push_str(",\"uniformLineMarkRecordStride\":");
    output.push_str(if family.uniform_line_mark_record_stride {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordStride\":");
    push_optional_usize_json(output, family.line_mark_record_stride);
    output.push_str(",\"matchesStrideCandidateRecordIndexes\":");
    output.push_str(if family.matches_stride_candidate_record_indexes {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSourceStartUnits\":");
    push_usize_array_json(output, &family.row_source_start_units);
    output.push_str(",\"rowSourceEndUnits\":");
    push_usize_array_json(output, &family.row_source_end_units);
    output.push_str(",\"lineMarkStartUnits\":");
    push_usize_array_json(output, &family.line_mark_start_units);
    output.push_str(",\"lineMarkEndUnits\":");
    push_usize_array_json(output, &family.line_mark_end_units);
    output.push_str(",\"startResidualUnits\":");
    push_i32_array_json(output, &family.start_residual_units);
    output.push_str(",\"endResidualUnits\":");
    push_i32_array_json(output, &family.end_residual_units);
    output.push_str(",\"spanResidualUnits\":");
    push_i32_array_json(output, &family.span_residual_units);
    output.push_str(",\"exactBoundaryMatchCount\":");
    output.push_str(&family.exact_boundary_match_count.to_string());
    output.push_str(",\"exactBoundaryAligned\":");
    output.push_str(if family.exact_boundary_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"startResidualStable\":");
    output.push_str(if family.start_residual_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"endResidualStable\":");
    output.push_str(if family.end_residual_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"spanResidualStable\":");
    output.push_str(if family.span_residual_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"stableStartResidualUnits\":");
    push_optional_i32_json(output, family.stable_start_residual_units);
    output.push_str(",\"stableEndResidualUnits\":");
    push_optional_i32_json(output, family.stable_end_residual_units);
    output.push_str(",\"stableSpanResidualUnits\":");
    push_optional_i32_json(output, family.stable_span_residual_units);
    output.push_str(",\"rowBoundaryOffsetCandidateUnits\":");
    push_optional_i32_json(output, family.row_boundary_offset_candidate_units);
    output.push_str(",\"offsetNormalizationPolicy\":");
    output.push_str(&json_string(
        "line-mark-boundary-minus-row-source-boundary-minus-stable-offset",
    ));
    output.push_str(",\"offsetNormalizedStartResidualUnits\":");
    push_i32_array_json(output, &family.offset_normalized_start_residual_units);
    output.push_str(",\"offsetNormalizedEndResidualUnits\":");
    push_i32_array_json(output, &family.offset_normalized_end_residual_units);
    output.push_str(",\"offsetNormalizedExactBoundaryMatchCount\":");
    output.push_str(
        &family
            .offset_normalized_exact_boundary_match_count
            .to_string(),
    );
    output.push_str(",\"offsetNormalizedExactBoundaryAligned\":");
    output.push_str(if family.offset_normalized_exact_boundary_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"spanOnlyMatch\":");
    output.push_str(if family.span_only_match {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rows\":[");
    for (index, row) in family.rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_line_mark_row_boundary_alignment_row_json(output, row);
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_line_mark_row_boundary_alignment_row_json(
    output: &mut String,
    row: &TableGridLineMarkRowBoundaryAlignmentRow,
) {
    output.push_str("{\"compactRow\":");
    output.push_str(&row.compact_row_index.to_string());
    output.push_str(",\"sparseRow\":");
    output.push_str(&row.sparse_row_index.to_string());
    output.push_str(",\"sourceIntervalIndex\":");
    output.push_str(&row.source_interval_index.to_string());
    output.push_str(",\"lineMarkRecordIndex\":");
    output.push_str(&row.line_mark_record_index.to_string());
    output.push_str(",\"rowSourceUnitRange\":");
    output.push_str(&source_range_json(
        row.row_source_start_units,
        row.row_source_end_units,
    ));
    output.push_str(",\"lineMarkUnitRange\":");
    output.push_str(&source_range_json(
        row.line_mark_start_units,
        row.line_mark_end_units,
    ));
    output.push_str(",\"startResidualUnits\":");
    output.push_str(&row.start_residual_units.to_string());
    output.push_str(",\"endResidualUnits\":");
    output.push_str(&row.end_residual_units.to_string());
    output.push_str(",\"spanResidualUnits\":");
    output.push_str(&row.span_residual_units.to_string());
    output.push_str(",\"exactBoundaryAligned\":");
    output.push_str(if row.exact_boundary_aligned {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_subrecord_line_range_record_coverage_summary_json(
    output: &mut String,
    summary: Option<&TableGridPageMarkSubrecordLineRangeRecordCoverageSummary>,
) {
    let Some(summary) = summary else {
        output.push_str("null");
        return;
    };

    output.push_str(
        "{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark record indexes\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"policy\":\"subrecord-line-start-end-must-contain-line-mark-record-index\"");
    output.push_str(",\"candidateCount\":");
    output.push_str(&summary.candidate_count.to_string());
    output.push_str(",\"selectedSpacingRecordIndexes\":");
    push_usize_array_json(output, &summary.selected_record_indexes);
    output.push_str(",\"previousRowSpanRecordIndexes\":");
    push_usize_array_json(output, &summary.previous_record_indexes);
    output.push_str(",\"selectedCoveredRecordIndexes\":");
    push_usize_array_json(output, &summary.selected_covered_record_indexes);
    output.push_str(",\"previousCoveredRecordIndexes\":");
    push_usize_array_json(output, &summary.previous_covered_record_indexes);
    output.push_str(",\"selectedContainingCandidateByteOffsets\":");
    push_usize_array_json(output, &summary.selected_containing_candidate_byte_offsets);
    output.push_str(",\"previousContainingCandidateByteOffsets\":");
    push_usize_array_json(output, &summary.previous_containing_candidate_byte_offsets);
    output.push_str(",\"selectedCoverageComplete\":");
    output.push_str(if summary.selected_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousCoverageComplete\":");
    output.push_str(if summary.previous_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedNearestLineRangeMatches\":");
    push_table_grid_page_mark_subrecord_line_range_record_matches_json(
        output,
        &summary.selected_nearest_matches,
    );
    output.push_str(",\"previousNearestLineRangeMatches\":");
    push_table_grid_page_mark_subrecord_line_range_record_matches_json(
        output,
        &summary.previous_nearest_matches,
    );
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-subrecord-line-range-record-coverage\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if summary.selected_coverage_complete || summary.previous_coverage_complete {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "page-mark-subrecord-line-ranges-do-not-cover-line-mark-records",
        ));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_subrecord_line_range_record_matches_json(
    output: &mut String,
    matches: &[TableGridPageMarkSubrecordLineRangeRecordMatch],
) {
    output.push('[');
    for (index, match_) in matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"recordIndex\":");
        output.push_str(&match_.record_index.to_string());
        output.push_str(",\"distanceUnits\":");
        output.push_str(&match_.distance_units.to_string());
        output.push_str(",\"candidate\":");
        push_page_mark_raw_subrecord_line_span_candidate_json(output, &match_.candidate);
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_table_grid_stride_page_mark_entry_line_bounds_coverage_summary_json(
    output: &mut String,
    summary: Option<&TableGridStridePageMarkEntryLineBoundsCoverageSummary>,
) {
    let Some(summary) = summary else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/LineMark record indexes+/PageMark entry line bounds\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourceDomain\":\"line-mark-record-index\"");
    output.push_str(",\"pageMarkDomain\":\"page-mark-line-index\"");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&summary.candidate_row_count.to_string());
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &summary.line_mark_record_indexes);
    output.push_str(",\"recordStride\":");
    output.push_str(&summary.record_stride.to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&summary.page_mark_entry_index.to_string());
    output.push_str(",\"pageIndexCandidate\":");
    push_optional_usize_json(output, summary.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    output.push_str(&summary.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&summary.page_line_end.to_string());
    output.push_str(",\"lineOffsetsFromPageStart\":");
    push_usize_array_json(output, &summary.line_offsets_from_page_start);
    output.push_str(",\"rowCountMatchesStrideCandidate\":");
    output.push_str(if summary.row_count_matches_stride_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allLineMarkRecordsWithinPageMarkEntry\":");
    output.push_str(if summary.all_line_mark_records_within_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"coverageReady\":");
    output.push_str(if summary.coverage_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRangeCoverageEvaluated\":false");
    output.push_str(",\"sourceRangeCoverageSkippedReason\":\"document-text-unit-ranges-are-not-page-mark-line-indexes\"");
    output.push_str(",\"pageYTransformDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"source-only-stride-row-page-mark-entry-coverage\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if summary.coverage_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "line-mark-records-not-contained-in-page-mark-entry",
        ));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_line_mark_page_origin_stride_candidate_json(
    output: &mut String,
    candidate: Option<&TableGridLineMarkPageOriginStrideCandidate>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/LineMark+/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"diagnosticOnly\":true");
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &candidate.line_mark_record_indexes);
    output.push_str(",\"recordStride\":");
    output.push_str(&candidate.record_stride.to_string());
    output.push_str(",\"interleavedRecordCountBetweenRows\":");
    output.push_str(&candidate.record_stride.saturating_sub(1).to_string());
    output.push_str(",\"firstLineMarkRecordIndex\":");
    output.push_str(&candidate.first_line_mark_record_index.to_string());
    output.push_str(",\"lastLineMarkRecordIndex\":");
    output.push_str(&candidate.last_line_mark_record_index.to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&candidate.page_mark_entry_index.to_string());
    output.push_str(",\"pageIndexCandidate\":");
    push_optional_usize_json(output, candidate.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    output.push_str(&candidate.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&candidate.page_line_end.to_string());
    output.push_str(",\"lineOffsetFromPageStart\":");
    output.push_str(&candidate.line_offset_from_page_start.to_string());
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{:.3}", candidate.row_height));
    output.push_str(",\"rawRecordIndexRowTops\":[");
    for (index, top) in candidate.raw_record_index_row_tops.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{top:.3}"));
    }
    output.push(']');
    output.push_str(",\"strideCollapsedRowTops\":[");
    for (index, top) in candidate.stride_collapsed_row_tops.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{top:.3}"));
    }
    output.push(']');
    output.push_str(",\"pageMarkU16Fields\":");
    push_u16_array_json(output, &candidate.page_mark_u16_fields);
    output.push_str(",\"pageMarkU16FieldsHex\":");
    push_u16_hex_array_json(output, &candidate.page_mark_u16_fields);
    output.push_str(",\"pageMarkU16GeometryHypotheses\":");
    push_page_mark_u16_geometry_hypotheses_json(
        output,
        &candidate.page_mark_u16_fields,
        Some(PageMarkU16LayoutComparison {
            page_width_px: candidate.page_width_px,
            page_height_px: candidate.page_height_px,
            page_margin_px: candidate.page_margin_px,
            page_body_width_px: candidate.page_body_width_px,
        }),
    );
    output.push_str(",\"renderPromotionContribution\":\"stride-aware-page-y-diagnostic\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-record-stride-to-page-y-transform-unproven",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_layout_stream_probe_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let line_mark_bytes = raw_stream_bytes(document, LINE_MARK_PATH).map(<[u8]>::len);
    let line_mark_word_count = line_mark_bytes.map(|len| len / 2);
    let page_mark = document.page_marks().first();
    let page_mark_entry_count = page_mark.map(|mark| mark.entries().len()).unwrap_or(0);
    let page_mark_direct_hit_count =
        table_candidate_direct_page_mark_line_hit_count(page_mark, candidate);
    let paper_mark_bytes = raw_stream_bytes(document, PAPER_MARK_PATH).map(<[u8]>::len);
    let paper_mark = document.paper_marks().first();
    let paper_mark_entry_count = paper_mark.map(|mark| mark.entries().len()).unwrap_or(0);
    let frame_record_count = document.object_frame_records().len();

    output.push_str("{\"lineMarkPresent\":");
    output.push_str(if line_mark_bytes.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkByteLength\":");
    push_optional_usize_json(output, line_mark_bytes);
    output.push_str(",\"lineMarkWordCount\":");
    push_optional_usize_json(output, line_mark_word_count);
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(shanai_lan_line_mark_profile(document)));
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    output.push_str(",\"lineMarkIntervalCount\":");
    output.push_str(&line_mark_intervals.len().to_string());
    output.push_str(",\"lineMarkRowEvidence\":");
    push_table_grid_line_mark_row_evidence_json(output, candidate, &line_mark_intervals);
    output.push_str(",\"candidateStartWithinLineMarkWordIndex\":");
    output.push_str(
        if line_mark_word_count.is_some_and(|count| candidate.source_start() < count) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"candidateEndWithinLineMarkWordIndex\":");
    output.push_str(
        if line_mark_word_count.is_some_and(|count| candidate.source_end() <= count) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"pageMarkPresent\":");
    output.push_str(if page_mark.is_some() { "true" } else { "false" });
    output.push_str(",\"pageMarkEntryCount\":");
    output.push_str(&page_mark_entry_count.to_string());
    output.push_str(",\"pageMarkLineMarkRecordEvidence\":");
    push_table_grid_page_mark_line_mark_record_evidence_json(
        output,
        page_mark,
        candidate,
        &line_mark_intervals,
    );
    output.push_str(",\"paperMarkPageAssociationEvidence\":");
    push_table_grid_paper_mark_page_association_evidence_json(
        output,
        page_mark,
        paper_mark,
        candidate,
        &line_mark_intervals,
    );
    output.push_str(",\"candidateRangeDirectPageMarkLineHitCount\":");
    output.push_str(&page_mark_direct_hit_count.to_string());
    output.push_str(",\"paperMarkPresent\":");
    output.push_str(if paper_mark_bytes.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paperMarkByteLength\":");
    push_optional_usize_json(output, paper_mark_bytes);
    output.push_str(",\"paperMarkEntryCount\":");
    output.push_str(&paper_mark_entry_count.to_string());
    output.push_str(",\"paperMarkHeaderCount\":");
    match paper_mark {
        Some(mark) => output.push_str(&mark.header_count().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"paperMarkHeaderStride\":");
    match paper_mark {
        Some(mark) => output.push_str(&mark.header_stride().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"paperMarkHeaderLastIndex\":");
    match paper_mark {
        Some(mark) => output.push_str(&mark.header_last_index().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"pagePaperMarkEntryCountAligned\":");
    output.push_str(
        if page_mark
            .zip(paper_mark)
            .is_some_and(|(page, paper)| page.entries().len() == paper.entries().len())
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"objectFrameRecordCount\":");
    output.push_str(&frame_record_count.to_string());
    output.push_str(",\"objectFrameSourceUnitLinkCount\":0");
    output.push_str(",\"directPlacementEvidence\":false");
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_line_mark_record_evidence_json(
    output: &mut String,
    page_mark: Option<&DocumentPageMark>,
    candidate: &TableCandidate,
    intervals: &[ShanaiLanLineMarkInterval],
) {
    let Some(page_mark) = page_mark else {
        output.push_str("null");
        return;
    };
    if intervals.is_empty() {
        output.push_str("null");
        return;
    }

    let mut row_matches = Vec::new();
    for (row_index, row) in candidate.intervals().iter().enumerate() {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start());
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end());
        if let Some(interval) =
            best_line_mark_interval_for_unit_range(intervals, row_unit_start, row_unit_end)
        {
            let page_entry = page_mark.entries().iter().find(|entry| {
                let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                    return false;
                };
                let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                    return false;
                };
                line_start <= interval.record_index && interval.record_index <= line_end
            });
            row_matches.push((
                row_index,
                row_unit_start,
                row_unit_end,
                interval,
                page_entry,
            ));
        }
    }

    let matched_page_row_indexes = row_matches
        .iter()
        .filter_map(|(_, _, _, _, entry)| entry.map(DocumentPageMarkEntry::row_index))
        .collect::<Vec<_>>();
    let row_page_match_count = matched_page_row_indexes.len();
    let all_rows_page_matched = row_page_match_count == candidate.intervals().len();
    let first_page_row_index = matched_page_row_indexes.first().copied();
    let single_page_matched = first_page_row_index.is_some()
        && matched_page_row_indexes
            .iter()
            .all(|row_index| Some(*row_index) == first_page_row_index);
    let first_record_index = row_matches
        .iter()
        .map(|(_, _, _, interval, _)| interval.record_index)
        .min();
    let last_record_index = row_matches
        .iter()
        .map(|(_, _, _, interval, _)| interval.record_index)
        .max();

    output.push_str("{\"source\":\"/PageMark+/LineMark\"");
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"lineMarkMatchedRowCount\":");
    output.push_str(&row_matches.len().to_string());
    output.push_str(",\"rowPageMatchCount\":");
    output.push_str(&row_page_match_count.to_string());
    output.push_str(",\"allRowsPageMatched\":");
    output.push_str(if all_rows_page_matched {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singlePageMatched\":");
    output.push_str(if single_page_matched { "true" } else { "false" });
    output.push_str(",\"lineMarkRecordRange\":");
    match (first_record_index, last_record_index) {
        (Some(start), Some(end)) => {
            output.push_str(&source_range_json(start, end.saturating_add(1)));
        }
        _ => output.push_str("null"),
    }
    output.push_str(",\"matchedPageMarkEntryIndex\":");
    push_optional_usize_json(output, first_page_row_index.filter(|_| single_page_matched));
    let matched_record_indexes = row_matches
        .iter()
        .map(|(_, _, _, interval, _)| interval.record_index)
        .collect::<Vec<_>>();
    push_line_mark_record_stride_fields_json(output, &matched_record_indexes);
    output.push_str(",\"renderPromotionContribution\":\"page-association-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("page-mark-line-index-not-y-coordinate"));
    output.push_str(",\"rows\":[");
    for (index, (row_index, row_unit_start, row_unit_end, interval, page_entry)) in
        row_matches.iter().enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(*row_unit_start, *row_unit_end));
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&interval.record_index.to_string());
        output.push_str(",\"pageMarkEntryIndex\":");
        match page_entry {
            Some(entry) => output.push_str(&entry.row_index().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"pageIndexCandidate\":");
        match page_entry.and_then(|entry| entry.index()) {
            Some(index) => output.push_str(&index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"pageLineStart\":");
        match page_entry.and_then(|entry| entry.line_start()) {
            Some(start) => output.push_str(&start.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"pageLineEnd\":");
        match page_entry.and_then(|entry| entry.line_end()) {
            Some(end) => output.push_str(&end.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"withinPageLineRange\":");
        output.push_str(if page_entry.is_some() {
            "true"
        } else {
            "false"
        });
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_paper_mark_page_association_evidence_json(
    output: &mut String,
    page_mark: Option<&DocumentPageMark>,
    paper_mark: Option<&DocumentPaperMark>,
    candidate: &TableCandidate,
    intervals: &[ShanaiLanLineMarkInterval],
) {
    let (Some(page_mark), Some(paper_mark)) = (page_mark, paper_mark) else {
        output.push_str("null");
        return;
    };
    if intervals.is_empty() {
        output.push_str("null");
        return;
    }

    let mut row_matches = Vec::new();
    for (row_index, row) in candidate.intervals().iter().enumerate() {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start());
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end());
        if let Some(interval) =
            best_line_mark_interval_for_unit_range(intervals, row_unit_start, row_unit_end)
        {
            let page_entry = page_mark.entries().iter().find(|entry| {
                let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                    return false;
                };
                let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                    return false;
                };
                line_start <= interval.record_index && interval.record_index <= line_end
            });
            let paper_entry = page_entry.and_then(|page_entry| {
                page_entry
                    .index()
                    .and_then(|page_index| {
                        paper_mark
                            .entries()
                            .iter()
                            .find(|entry| entry.index() == page_index)
                    })
                    .or_else(|| paper_mark.entries().get(page_entry.row_index()))
            });
            row_matches.push((row_index, interval, page_entry, paper_entry));
        }
    }

    if row_matches.is_empty() {
        output.push_str("null");
        return;
    }

    let matched_page_row_indexes = row_matches
        .iter()
        .filter_map(|(_, _, entry, _)| entry.map(DocumentPageMarkEntry::row_index))
        .collect::<Vec<_>>();
    let matched_paper_row_indexes = row_matches
        .iter()
        .filter_map(|(_, _, _, entry)| entry.map(DocumentPaperMarkEntry::row_index))
        .collect::<Vec<_>>();
    let first_page_row_index = matched_page_row_indexes.first().copied();
    let first_paper_row_index = matched_paper_row_indexes.first().copied();
    let single_page_matched = first_page_row_index.is_some()
        && matched_page_row_indexes
            .iter()
            .all(|row_index| Some(*row_index) == first_page_row_index);
    let single_paper_mark_matched = first_paper_row_index.is_some()
        && matched_paper_row_indexes
            .iter()
            .all(|row_index| Some(*row_index) == first_paper_row_index);
    let matched_paper_entry =
        first_paper_row_index.and_then(|row_index| paper_mark.entries().get(row_index));

    output.push_str("{\"source\":\"/PageMark+/PaperMark+/LineMark\"");
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"pageMarkMatchedRowCount\":");
    output.push_str(&matched_page_row_indexes.len().to_string());
    output.push_str(",\"paperMarkMatchedRowCount\":");
    output.push_str(&matched_paper_row_indexes.len().to_string());
    output.push_str(",\"singlePageMatched\":");
    output.push_str(if single_page_matched { "true" } else { "false" });
    output.push_str(",\"singlePaperMarkMatched\":");
    output.push_str(if single_paper_mark_matched {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedPageMarkEntryIndex\":");
    push_optional_usize_json(output, first_page_row_index.filter(|_| single_page_matched));
    output.push_str(",\"matchedPaperMarkEntryIndex\":");
    push_optional_usize_json(
        output,
        first_paper_row_index.filter(|_| single_paper_mark_matched),
    );
    output.push_str(",\"matchedPaperMarkIndex\":");
    match matched_paper_entry.filter(|_| single_paper_mark_matched) {
        Some(entry) => output.push_str(&entry.index().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"matchedPaperMarkFlags\":");
    match matched_paper_entry.filter(|_| single_paper_mark_matched) {
        Some(entry) => output.push_str(&entry.flags().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"matchedPaperMarkFlagsHex\":");
    match matched_paper_entry.filter(|_| single_paper_mark_matched) {
        Some(entry) => output.push_str(&json_string(&format!("0x{:08x}", entry.flags()))),
        None => output.push_str("null"),
    }
    output.push_str(",\"pagePaperMarkEntryCountAligned\":");
    output.push_str(if page_mark.entries().len() == paper_mark.entries().len() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"paper-mark-page-row-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("paper-mark-flag-semantics-undecoded"));
    output.push_str(",\"rows\":[");
    for (index, (row_index, interval, page_entry, paper_entry)) in row_matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&interval.record_index.to_string());
        output.push_str(",\"pageMarkEntryIndex\":");
        match page_entry {
            Some(entry) => output.push_str(&entry.row_index().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"pageIndexCandidate\":");
        match page_entry.and_then(|entry| entry.index()) {
            Some(index) => output.push_str(&index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"paperMarkEntryIndex\":");
        match paper_entry {
            Some(entry) => output.push_str(&entry.row_index().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"paperMarkIndex\":");
        match paper_entry {
            Some(entry) => output.push_str(&entry.index().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"paperMarkFlags\":");
        match paper_entry {
            Some(entry) => output.push_str(&entry.flags().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"paperMarkFlagsHex\":");
        match paper_entry {
            Some(entry) => output.push_str(&json_string(&format!("0x{:08x}", entry.flags()))),
            None => output.push_str("null"),
        }
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_line_mark_row_evidence_json(
    output: &mut String,
    candidate: &TableCandidate,
    intervals: &[ShanaiLanLineMarkInterval],
) {
    if intervals.is_empty() {
        output.push_str("null");
        return;
    }

    let candidate_unit_start =
        table_source_offset_to_units(candidate.basis(), candidate.source_start());
    let candidate_unit_end =
        table_source_offset_to_units(candidate.basis(), candidate.source_end());
    let mut row_matches = Vec::new();
    for (row_index, row) in candidate.intervals().iter().enumerate() {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start());
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end());
        if let Some(interval) =
            best_line_mark_interval_for_unit_range(intervals, row_unit_start, row_unit_end)
        {
            let exact_boundary_match =
                interval.unit_start == row_unit_start && interval.unit_end == row_unit_end;
            row_matches.push((
                row_index,
                row_unit_start,
                row_unit_end,
                interval,
                exact_boundary_match,
            ));
        }
    }

    let exact_row_match_count = row_matches
        .iter()
        .filter(|(_, _, _, _, exact)| *exact)
        .count();
    let row_count_matches_candidate = row_matches.len() == candidate.intervals().len();
    let contiguous_record_indexes = !row_matches.is_empty()
        && row_matches
            .windows(2)
            .all(|pair| pair[1].3.record_index == pair[0].3.record_index + 1);

    output.push_str("{\"source\":");
    output.push_str(&json_string(LINE_MARK_PATH));
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"candidateUnitRange\":");
    output.push_str(&source_range_json(candidate_unit_start, candidate_unit_end));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&row_matches.len().to_string());
    output.push_str(",\"exactRowMatchCount\":");
    output.push_str(&exact_row_match_count.to_string());
    output.push_str(",\"rowCountMatchesCandidate\":");
    output.push_str(if row_count_matches_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"contiguousRecordIndexes\":");
    output.push_str(if contiguous_record_indexes {
        "true"
    } else {
        "false"
    });
    let matched_record_indexes = row_matches
        .iter()
        .map(|(_, _, _, interval, _)| interval.record_index)
        .collect::<Vec<_>>();
    push_line_mark_record_stride_fields_json(output, &matched_record_indexes);
    output
        .push_str(",\"renderPromotionContribution\":\"row-boundary-and-row-order-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-units-not-y-page-coordinate-transform",
    ));
    output.push_str(",\"rows\":[");
    for (index, (row_index, row_unit_start, row_unit_end, interval, exact)) in
        row_matches.iter().enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(*row_unit_start, *row_unit_end));
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&interval.record_index.to_string());
        output.push_str(",\"lineMarkUnitRange\":");
        output.push_str(&source_range_json(interval.unit_start, interval.unit_end));
        output.push_str(",\"deltaUnits\":");
        output.push_str(&(interval.unit_end - interval.unit_start).to_string());
        output.push_str(",\"flagHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", interval.flag_word)));
        output.push_str(",\"exactBoundaryMatch\":");
        output.push_str(if *exact { "true" } else { "false" });
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn table_grid_interval_line_mark(
    candidate: &TableCandidate,
    interval: &TableCandidateInterval,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) -> Option<ShanaiLanLineMarkInterval> {
    let row_unit_start = table_source_offset_to_units(candidate.basis(), interval.source_start());
    let row_unit_end = table_source_offset_to_units(candidate.basis(), interval.source_end());
    best_line_mark_interval_for_unit_range(line_mark_intervals, row_unit_start, row_unit_end)
}

pub(crate) fn table_grid_page_mark_entry_for_line_mark_record(
    page_mark: Option<&DocumentPageMark>,
    record_index: usize,
) -> Option<&DocumentPageMarkEntry> {
    page_mark?.entries().iter().find(|entry| {
        let Some(line_start) = entry.line_start().map(|value| value as usize) else {
            return false;
        };
        let Some(line_end) = entry.line_end().map(|value| value as usize) else {
            return false;
        };
        line_start <= record_index && record_index <= line_end
    })
}

pub(crate) fn table_grid_line_header_row_for_interval<'a>(
    rows: &'a [TableCandidateLineHeaderRow],
    interval: &TableCandidateInterval,
) -> Option<&'a TableCandidateLineHeaderRow> {
    rows.iter().find(|row| row.row_index == interval.index())
}

pub(crate) fn table_grid_cell_line_header_candidate<'a>(
    basis: TextCountRangeOverlapBasis,
    row: Option<&'a TableCandidateLineHeaderRow>,
    segment: &TableCandidateColumnSegment,
) -> Option<&'a ShanaiLanLineHeader> {
    let row = row?;
    let segment_start = table_source_offset_to_units(basis, segment.source_start()?);
    let segment_end = table_source_offset_to_units(basis, segment.source_end()?);

    row.headers
        .iter()
        .filter(|header| header.end / 2 <= segment_start)
        .min_by_key(|header| segment_start.saturating_sub(header.end / 2))
        .or_else(|| {
            row.headers
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
            row.headers.iter().min_by_key(|header| {
                segment_start
                    .abs_diff(header.start / 2)
                    .min(segment_start.abs_diff(header.end / 2))
            })
        })
}

pub(crate) fn table_grid_line_header_selection_kind(
    segment_start_units: Option<usize>,
    segment_end_units: Option<usize>,
    header: &ShanaiLanLineHeader,
) -> &'static str {
    let (Some(segment_start), Some(segment_end)) = (segment_start_units, segment_end_units) else {
        return "no-segment-source-range";
    };
    let header_start = header.start / 2;
    let header_end = header.end / 2;
    if header_end <= segment_start {
        "nearest-preceding-line-header"
    } else if ranges_overlap_half_open(header_start, header_end, segment_start, segment_end) {
        "overlapping-line-header"
    } else {
        "nearest-line-header"
    }
}

pub(crate) fn push_table_grid_line_header_candidate_json(
    output: &mut String,
    basis: TextCountRangeOverlapBasis,
    row: Option<&TableCandidateLineHeaderRow>,
    segment: &TableCandidateColumnSegment,
) {
    let Some(header) = table_grid_cell_line_header_candidate(basis, row, segment) else {
        output.push_str("null");
        return;
    };
    let segment_start_units = segment
        .source_start()
        .map(|value| table_source_offset_to_units(basis, value));
    let segment_end_units = segment
        .source_end()
        .map(|value| table_source_offset_to_units(basis, value));
    let header_start_units = header.start / 2;
    let header_end_units = header.end / 2;
    output.push_str("{\"source\":\"/DocumentText line-header\"");
    output.push_str(",\"selection\":");
    output.push_str(&json_string(table_grid_line_header_selection_kind(
        segment_start_units,
        segment_end_units,
        header,
    )));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(header_start_units, header_end_units));
    output.push_str(",\"offsetUnits\":");
    output.push_str(&header.offset_units.to_string());
    output.push_str(",\"extentUnits\":");
    output.push_str(&header.extent_units.to_string());
    output.push_str(",\"fontSizeUnits\":");
    output.push_str(&header.font_size_units.to_string());
    output.push_str(",\"segmentStartMinusHeaderEndUnits\":");
    match segment_start_units {
        Some(start) => output.push_str(&(start as i64 - header_end_units as i64).to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"segmentEndMinusHeaderStartUnits\":");
    match segment_end_units {
        Some(end) => output.push_str(&(end as i64 - header_start_units as i64).to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"rawWords\":");
    push_u16_array_json(output, &header.raw_words);
    output.push_str(",\"rawWordsHex\":");
    push_u16_hex_array_json(output, &header.raw_words);
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_table_grid_cell_source_evidence_json(
    output: &mut String,
    candidate: &TableCandidate,
    interval: &TableCandidateInterval,
    segment: &TableCandidateColumnSegment,
    line_header_rows: &[TableCandidateLineHeaderRow],
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    page_mark: Option<&DocumentPageMark>,
) {
    let row_unit_start = table_source_offset_to_units(candidate.basis(), interval.source_start());
    let row_unit_end = table_source_offset_to_units(candidate.basis(), interval.source_end());
    let line_mark = table_grid_interval_line_mark(candidate, interval, line_mark_intervals);
    let page_entry = line_mark.and_then(|interval| {
        table_grid_page_mark_entry_for_line_mark_record(page_mark, interval.record_index)
    });
    let line_header_row = table_grid_line_header_row_for_interval(line_header_rows, interval);

    output.push_str("{\"source\":\"tableCellProvenance\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"rowSourceIntervalIndex\":");
    output.push_str(&interval.source_interval_index().to_string());
    output.push_str(",\"rowSourceRange\":");
    output.push_str(&source_range_json(row_unit_start, row_unit_end));
    output.push_str(",\"segmentIndex\":");
    output.push_str(&segment.index().to_string());
    output.push_str(",\"segmentKind\":");
    output.push_str(&json_string(segment.kind().as_str()));
    output.push_str(",\"segmentCharRange\":");
    output.push_str(&source_range_json(segment.char_start(), segment.char_end()));
    output.push_str(",\"lineMarkRecordIndex\":");
    match line_mark {
        Some(interval) => output.push_str(&interval.record_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkUnitRange\":");
    match line_mark {
        Some(interval) => {
            output.push_str(&source_range_json(interval.unit_start, interval.unit_end))
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"pageMarkEntryIndex\":");
    match page_entry {
        Some(entry) => output.push_str(&entry.row_index().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageIndexCandidate\":");
    match page_entry.and_then(DocumentPageMarkEntry::index) {
        Some(index) => output.push_str(&index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageLineRange\":");
    match page_entry.and_then(|entry| entry.line_start().zip(entry.line_end())) {
        Some((start, end)) => {
            output.push_str(&source_range_json(
                start as usize,
                end.saturating_add(1) as usize,
            ));
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"lineHeaderCandidate\":");
    push_table_grid_line_header_candidate_json(output, candidate.basis(), line_header_row, segment);
    output.push('}');
}

pub(crate) fn push_table_grid_segment_source_range_json(
    output: &mut String,
    candidate: &TableCandidate,
    segment: &TableCandidateColumnSegment,
) {
    match (segment.source_start(), segment.source_end()) {
        (Some(start), Some(end)) if start < end => {
            output.push_str("{\"basis\":");
            output.push_str(&json_string(candidate.basis().as_str()));
            output.push_str(",\"start\":");
            output.push_str(&start.to_string());
            output.push_str(",\"end\":");
            output.push_str(&end.to_string());
            output.push_str(",\"decoded\":false}");
        }
        _ => output.push_str("null"),
    }
}
