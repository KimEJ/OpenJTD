use super::*;
use crate::*;

pub(crate) fn push_table_grid_piecewise_record_family_gap_table_json(
    output: &mut String,
    table: &TableGridCrossTableRowBoundaryOffsetTable,
) {
    output.push_str("{\"tableCandidateIndex\":");
    output.push_str(&table.table_candidate_index.to_string());
    output.push_str(",\"sourceRange\":");
    output.push_str(&source_range_json(table.source_start, table.source_end));
    output.push_str(",\"rowCount\":");
    output.push_str(&table.row_count.to_string());
    output.push_str(",\"previousRecordIndexes\":");
    push_usize_array_json(output, &table.line_mark_record_indexes);
    output.push_str(",\"selectedRecordIndexes\":");
    push_usize_array_json(output, &table.selected_spacing_record_indexes);
    output.push_str(",\"previousPageMarkLineOffsetsFromEntryStart\":");
    push_usize_array_json(output, &table.page_mark_line_offsets_from_entry_start);
    output.push_str(",\"selectedPageMarkLineOffsetsFromEntryStart\":");
    push_usize_array_json(
        output,
        &table.selected_spacing_page_mark_line_offsets_from_entry_start,
    );
    output.push_str(",\"previousRecordYTopPx\":");
    push_f32_array_json(output, &table.line_mark_record_y_tops_px);
    output.push_str(",\"selectedRecordYTopPx\":");
    push_f32_array_json(output, &table.selected_spacing_record_y_tops_px);
    output.push_str(",\"selectedMinusPreviousRecordIndexGaps\":");
    push_i32_array_json(output, &table.selected_minus_previous_record_index_gaps);
    output.push_str(",\"selectedMinusPreviousRecordYDeltaPx\":");
    push_f32_array_json(output, &table.selected_minus_previous_record_y_delta_px);
    output.push_str(",\"previousStartResidualUnits\":");
    push_i32_array_json(output, &table.start_residual_units);
    output.push_str(",\"previousEndResidualUnits\":");
    push_i32_array_json(output, &table.end_residual_units);
    output.push_str(",\"previousSpanResidualUnits\":");
    push_i32_array_json(output, &table.span_residual_units);
    output.push_str(",\"selectedStartResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_start_residual_units);
    output.push_str(",\"selectedEndResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_end_residual_units);
    output.push_str(",\"selectedSpanResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_span_residual_units);
    output.push('}');
}

pub(crate) fn push_table_grid_piecewise_record_family_gap_transition_json(
    output: &mut String,
    previous: &TableGridCrossTableRowBoundaryOffsetTable,
    next: &TableGridCrossTableRowBoundaryOffsetTable,
) {
    let previous_family_gap = previous
        .line_mark_record_indexes
        .last()
        .copied()
        .zip(next.line_mark_record_indexes.first().copied())
        .map(|(left, right)| signed_usize_delta_i32(right, left));
    let selected_family_gap = previous
        .selected_spacing_record_indexes
        .last()
        .copied()
        .zip(next.selected_spacing_record_indexes.first().copied())
        .map(|(left, right)| signed_usize_delta_i32(right, left));
    let previous_family_y_gap = previous
        .line_mark_record_y_tops_px
        .last()
        .copied()
        .zip(next.line_mark_record_y_tops_px.first().copied())
        .map(|(left, right)| right - left);
    let selected_family_y_gap = previous
        .selected_spacing_record_y_tops_px
        .last()
        .copied()
        .zip(next.selected_spacing_record_y_tops_px.first().copied())
        .map(|(left, right)| right - left);

    output.push_str("{\"fromTableCandidateIndex\":");
    output.push_str(&previous.table_candidate_index.to_string());
    output.push_str(",\"toTableCandidateIndex\":");
    output.push_str(&next.table_candidate_index.to_string());
    output.push_str(",\"sourceRangeGapUnits\":");
    output.push_str(
        &next
            .source_start
            .saturating_sub(previous.source_end)
            .to_string(),
    );
    output.push_str(",\"rowSourceStartGapUnits\":");
    let row_source_start_gap = previous
        .row_source_start_units
        .last()
        .copied()
        .zip(next.row_source_start_units.first().copied())
        .map(|(left, right)| signed_usize_delta_i32(right, left));
    push_optional_i32_json(output, row_source_start_gap);
    output.push_str(",\"previousFamilyRecordGap\":");
    push_optional_i32_json(output, previous_family_gap);
    output.push_str(",\"selectedFamilyRecordGap\":");
    push_optional_i32_json(output, selected_family_gap);
    output.push_str(",\"selectedMinusPreviousFamilyRecordGapDelta\":");
    push_optional_i32_json(
        output,
        selected_family_gap
            .zip(previous_family_gap)
            .map(|(selected, previous)| selected.saturating_sub(previous)),
    );
    output.push_str(",\"previousFamilyYGapPx\":");
    push_optional_f32_json(output, previous_family_y_gap);
    output.push_str(",\"selectedFamilyYGapPx\":");
    push_optional_f32_json(output, selected_family_y_gap);
    output.push_str(",\"selectedMinusPreviousFamilyYGapDeltaPx\":");
    push_optional_f32_json(
        output,
        selected_family_y_gap
            .zip(previous_family_y_gap)
            .map(|(selected, previous)| selected - previous),
    );
    output.push_str(",\"samePageMarkEntry\":");
    output.push_str(
        if previous.page_mark_records_within_single_entry
            && next.page_mark_records_within_single_entry
            && previous.selected_spacing_records_within_single_entry
            && next.selected_spacing_records_within_single_entry
        {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(crate) fn push_table_grid_source_unit_to_page_line_index_piecewise_fit_json(
    output: &mut String,
    probe: &TableGridCrossTableRowBoundaryOffsetProbe,
) {
    output.push_str("{\"source\":\"/DocumentText row source units+/LineMark previous-row-span records table-piecewise\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(
        ",\"fitBasis\":\"per-related-table-rowSourceStartUnits-to-lineMarkRecordIndexes\"",
    );
    output.push_str(",\"groupingBasis\":\"crossTableRowBoundaryOffsetConsistency.tables\"");
    output.push_str(",\"globalFitExact\":");
    output.push_str(if probe.source_unit_to_page_line_index_exact {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allTableFitsExact\":");
    output.push_str(
        if probe.source_unit_to_page_line_index_piecewise_all_tables_exact {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"maxTableFitResidualRecordIndexes\":");
    push_optional_f32_json(
        output,
        probe.source_unit_to_page_line_index_piecewise_max_abs_residual,
    );
    output.push_str(",\"samePageMarkEntryContinuity\":");
    output.push_str(if probe.all_records_within_single_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pieceCount\":");
    output.push_str(
        &probe
            .source_unit_to_page_line_index_piecewise_tables
            .len()
            .to_string(),
    );
    output.push_str(",\"transitionCount\":");
    output.push_str(
        &probe
            .source_unit_to_page_line_index_piecewise_transitions
            .len()
            .to_string(),
    );
    output.push_str(",\"pieces\":[");
    for (index, table) in probe
        .source_unit_to_page_line_index_piecewise_tables
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_source_unit_to_page_line_index_piecewise_table_json(output, table);
    }
    output.push_str("],\"transitions\":[");
    for (index, transition) in probe
        .source_unit_to_page_line_index_piecewise_transitions
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_source_unit_to_page_line_index_piecewise_transition_json(
            output, transition,
        );
    }
    output.push(']');
    output.push_str(",\"renderPromotionContribution\":");
    output.push_str(&json_string(
        "source-unit-to-page-line-piecewise-fit-diagnostic-only",
    ));
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("piecewise-fit-does-not-decode-page-y-origin"));
    output.push('}');
}

pub(crate) fn push_table_grid_source_unit_to_page_line_index_piecewise_table_json(
    output: &mut String,
    table: &TableGridSourceUnitToPageLineIndexPiecewiseTable,
) {
    output.push_str("{\"tableCandidateIndex\":");
    output.push_str(&table.table_candidate_index.to_string());
    output.push_str(",\"sourceRange\":");
    output.push_str(&source_range_json(table.source_start, table.source_end));
    output.push_str(",\"rowCount\":");
    output.push_str(&table.row_count.to_string());
    output.push_str(",\"rowSourceStartUnits\":");
    push_usize_array_json(output, &table.row_source_start_units);
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &table.line_mark_record_indexes);
    output.push_str(",\"slopeRecordIndexesPerSourceUnit\":");
    push_optional_f32_json(output, table.slope_record_indexes_per_source_unit);
    output.push_str(",\"interceptRecordIndex\":");
    push_optional_f32_json(output, table.intercept_record_index);
    output.push_str(",\"fittedRecordIndexes\":");
    push_f32_array_json(output, &table.fitted_record_indexes);
    output.push_str(",\"residualRecordIndexes\":");
    push_f32_array_json(output, &table.residual_record_indexes);
    output.push_str(",\"maxAbsResidualRecordIndexes\":");
    push_optional_f32_json(output, table.max_abs_residual_record_indexes);
    output.push_str(",\"exactFit\":");
    output.push_str(if table.exact_fit { "true" } else { "false" });
    output.push_str(",\"pageMarkRecordsWithinSingleEntry\":");
    output.push_str(if table.page_mark_records_within_single_entry {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_table_grid_source_unit_to_page_line_index_piecewise_transition_json(
    output: &mut String,
    transition: &TableGridSourceUnitToPageLineIndexPiecewiseTransition,
) {
    output.push_str("{\"fromTableCandidateIndex\":");
    output.push_str(&transition.from_table_candidate_index.to_string());
    output.push_str(",\"toTableCandidateIndex\":");
    output.push_str(&transition.to_table_candidate_index.to_string());
    output.push_str(",\"previousLastSourceUnit\":");
    output.push_str(&transition.previous_last_source_unit.to_string());
    output.push_str(",\"nextFirstSourceUnit\":");
    output.push_str(&transition.next_first_source_unit.to_string());
    output.push_str(",\"sourceRangeGapUnits\":");
    output.push_str(&transition.source_range_gap_units.to_string());
    output.push_str(",\"rowSourceStartGapUnits\":");
    output.push_str(&transition.row_source_start_gap_units.to_string());
    output.push_str(",\"previousLastRecordIndex\":");
    output.push_str(&transition.previous_last_record_index.to_string());
    output.push_str(",\"nextFirstRecordIndex\":");
    output.push_str(&transition.next_first_record_index.to_string());
    output.push_str(",\"lineMarkRecordGap\":");
    output.push_str(&transition.line_mark_record_gap.to_string());
    output.push_str(",\"samePageMarkEntry\":");
    output.push_str(if transition.same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_table_grid_source_unit_to_page_line_index_fit_row_json(
    output: &mut String,
    row: &TableGridSourceUnitToPageLineIndexFitRow,
) {
    output.push_str("{\"tableCandidateIndex\":");
    output.push_str(&row.table_candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"rowSourceStartUnits\":");
    output.push_str(&row.row_source_start_units.to_string());
    output.push_str(",\"lineMarkRecordIndex\":");
    output.push_str(&row.line_mark_record_index.to_string());
    output.push_str(",\"fittedRecordIndex\":");
    output.push_str(&format!("{:.3}", row.fitted_record_index));
    output.push_str(",\"residualRecordIndex\":");
    output.push_str(&format!("{:.3}", row.residual_record_index));
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_raw_reference_value_probe_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        output.push_str("null");
        return;
    };
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let Some(reference_layout) =
        diagnostic_reference_table_grid_overlay_layout(layout, document, candidate, column_count)
    else {
        output.push_str("null");
        return;
    };
    let row_count = candidate.intervals().len();
    if row_count == 0 {
        output.push_str("null");
        return;
    }

    let record_headers = page_mark_record_headers(page_mark_bytes);
    const TOLERANCE_PX: f32 = 2.0;
    let mut total_hit_count = 0usize;
    let mut row_top_hit_count = 0usize;
    let mut hit_record_contexts = Vec::new();
    output.push_str("{\"source\":\"/PageMark raw numeric scan+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"tolerancePx\":");
    output.push_str(&format!("{TOLERANCE_PX:.3}"));
    output.push_str(",\"referenceBBox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        reference_layout.x,
        reference_layout.y,
        reference_layout.width,
        reference_layout.row_height * row_count as f32
    ));
    output.push_str(",\"rowTopTargets\":[");
    for row_index in 0..row_count {
        if row_index > 0 {
            output.push(',');
        }
        let target = reference_layout.y + row_index as f32 * reference_layout.row_height;
        let hits = page_mark_raw_numeric_hits_near(page_mark_bytes, target, TOLERANCE_PX);
        if !hits.is_empty() {
            row_top_hit_count += 1;
            total_hit_count += hits.len();
            for hit in &hits {
                if let Some(context) = page_mark_raw_numeric_hit_record_context(
                    &record_headers,
                    page_mark_bytes.len(),
                    hit.byte_offset,
                ) {
                    hit_record_contexts.push(context);
                }
            }
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"targetPx\":");
        output.push_str(&format!("{target:.3}"));
        output.push_str(",\"roundedTarget\":");
        output.push_str(&(target.round() as i32).to_string());
        output.push_str(",\"hitCount\":");
        output.push_str(&hits.len().to_string());
        output.push_str(",\"hits\":[");
        for (hit_index, hit) in hits.iter().take(12).enumerate() {
            if hit_index > 0 {
                output.push(',');
            }
            push_page_mark_raw_numeric_hit_json(output, hit, page_mark_bytes, &record_headers);
        }
        output.push_str("]}");
    }
    output.push(']');
    output.push_str(",\"rowTopTargetCount\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"rowTopTargetHitCount\":");
    output.push_str(&row_top_hit_count.to_string());
    output.push_str(",\"allRowTopTargetsHit\":");
    output.push_str(if row_top_hit_count == row_count {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"totalHitCount\":");
    output.push_str(&total_hit_count.to_string());
    output.push_str(",\"rawHitRecordContextSummary\":");
    push_page_mark_raw_numeric_hit_record_context_summary_json(output, &hit_record_contexts);
    output.push_str(",\"renderPromotionContribution\":\"reference-value-probe-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-raw-numeric-values-are-reference-probe-not-source-transform",
    ));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_line_mark_stride_promotion_readiness_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
    rows: &[TableCandidateLineHeaderRow],
    rows_with_headers: usize,
    raw_header_count: usize,
    stride: &TableGridLineMarkPageOriginStrideCandidate,
    reference_layout: &TableGridReferenceLayout,
) {
    let row_count = rows
        .len()
        .min(stride.raw_record_index_row_tops.len())
        .min(stride.stride_collapsed_row_tops.len());
    let candidate_row_count = candidate.intervals().len();
    let candidate_segment_count = candidate.cell_count_candidate();
    let matched_segment_count = sibling
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    let sparse_topology_complete = sibling.rows.len() == candidate_row_count
        && matched_segment_count == candidate_segment_count;
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    let all_rows_have_line_mark = stride.line_mark_record_indexes.len() == candidate_row_count;
    let uniform_record_stride = uniform_usize_stride(&stride.line_mark_record_indexes);

    let reference_row_tops = (0..row_count)
        .map(|row| reference_layout.y + row as f32 * reference_layout.row_height)
        .collect::<Vec<_>>();
    let raw_residuals = stride
        .raw_record_index_row_tops
        .iter()
        .take(row_count)
        .zip(&reference_row_tops)
        .map(|(candidate, reference)| candidate - reference)
        .collect::<Vec<_>>();
    let raw_max_abs = max_abs_f32(&raw_residuals);
    let reference_validation_threshold_px = 8.0f32;
    let raw_record_index_reference_fit =
        raw_max_abs.is_some_and(|value| value <= reference_validation_threshold_px);

    let (post_gap_match_count, post_gap_exact_count) =
        table_grid_sparse_sibling_post_row_gap_line_mark_correlation_counts(
            document, candidate, sibling,
        );
    let post_gap_correlation_complete =
        post_gap_match_count == candidate_row_count && post_gap_exact_count == candidate_row_count;
    let (raw_page_mark_scan_header_count, raw_page_mark_single_header_matched) =
        table_grid_page_mark_raw_scan_header_match_summary(
            document,
            &stride.line_mark_record_indexes,
        );
    let subrecord_span_readiness =
        table_grid_page_mark_subrecord_line_span_readiness(document, candidate);
    let subrecord_span_selected_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            !readiness.selected_post_row_gap_span_targets.is_empty()
                && readiness.selected_post_row_gap_span_hit_count
                    == readiness.selected_post_row_gap_span_targets.len()
        });
    let subrecord_span_previous_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            !readiness.previous_row_span_targets.is_empty()
                && readiness.previous_row_span_hit_count
                    == readiness.previous_row_span_targets.len()
        });
    let subrecord_span_compact_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            !readiness.compact_row_span_targets.is_empty()
                && readiness.compact_row_span_hit_count == readiness.compact_row_span_targets.len()
        });

    let mut blocked_reasons = Vec::new();
    if !all_rows_have_line_mark {
        blocked_reasons.push("line-mark-row-match-incomplete");
    }
    if uniform_record_stride.is_none() {
        blocked_reasons.push("line-mark-record-stride-not-uniform");
    }
    if !sparse_topology_complete {
        blocked_reasons.push("sparse-sibling-topology-incomplete");
    }
    if rows_with_headers < candidate_row_count {
        blocked_reasons.push("partial-line-header-font-evidence");
    }
    if matched_cell_header_count < candidate_segment_count {
        blocked_reasons.push("line-header-cell-geometry-incomplete");
    }
    if post_gap_correlation_complete {
        blocked_reasons.push("line-mark-spans-post-row-gaps-not-visible-row-heights");
    }
    if raw_page_mark_single_header_matched != Some(true) {
        blocked_reasons.push("page-mark-raw-record-scan-does-not-isolate-table-y");
    }
    if subrecord_span_selected_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-selected-post-row-gaps");
    }
    if subrecord_span_previous_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-previous-row-spans");
    }
    if subrecord_span_compact_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-compact-row-spans");
    }
    if subrecord_span_selected_complete
        || subrecord_span_previous_complete
        || subrecord_span_compact_complete
    {
        blocked_reasons.push("page-mark-subrecord-spans-do-not-decode-page-y-origin");
    }
    if !raw_record_index_reference_fit {
        blocked_reasons.push("raw-record-index-y-fails-current-reference-table");
    } else {
        blocked_reasons.push("raw-record-index-y-fit-is-reference-backed-only");
    }
    blocked_reasons.push("decoded-line-mark-stride-to-page-y-transform-missing");

    output.push_str("{\"source\":\"/LineMark+/PageMark+sparseSiblingEvidence\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":false");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate_row_count.to_string());
    output.push_str(",\"candidateSegmentCount\":");
    output.push_str(&candidate_segment_count.to_string());
    output.push_str(",\"allRowsHaveLineMark\":");
    output.push_str(if all_rows_have_line_mark {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &stride.line_mark_record_indexes);
    output.push_str(",\"uniformRecordStride\":");
    output.push_str(if uniform_record_stride.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"recordStride\":");
    push_option_usize_json(output, uniform_record_stride);
    output.push_str(",\"sparseTopologyComplete\":");
    output.push_str(if sparse_topology_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedSparseRowCount\":");
    output.push_str(&sibling.rows.len().to_string());
    output.push_str(",\"matchedSparseSegmentCount\":");
    output.push_str(&matched_segment_count.to_string());
    output.push_str(",\"lineHeaderRowsWithHeaders\":");
    output.push_str(&rows_with_headers.to_string());
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&raw_header_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&matched_cell_header_count.to_string());
    output.push_str(",\"postRowGapCorrelationComplete\":");
    output.push_str(if post_gap_correlation_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"postRowGapMatchCount\":");
    output.push_str(&post_gap_match_count.to_string());
    output.push_str(",\"postRowGapExactSpanMatchCount\":");
    output.push_str(&post_gap_exact_count.to_string());
    output.push_str(",\"rawPageMarkScanHeaderCount\":");
    push_option_usize_json(output, raw_page_mark_scan_header_count);
    output.push_str(",\"rawPageMarkSingleHeaderMatched\":");
    push_optional_bool_json(output, raw_page_mark_single_header_matched);
    output.push_str(",\"subrecordLineSpanReadiness\":");
    push_table_grid_page_mark_subrecord_line_span_readiness_json(
        output,
        subrecord_span_readiness.as_ref(),
    );
    output.push_str(",\"referenceValidationThresholdPx\":");
    output.push_str(&format!("{reference_validation_threshold_px:.3}"));
    output.push_str(",\"rawRecordIndexReferenceFit\":");
    output.push_str(if raw_record_index_reference_fit {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rawRecordIndexMaxAbsResidualPx\":");
    push_optional_f32_json(output, raw_max_abs);
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"stride-promotion-gate-diagnostic-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-stride-y-transform-not-source-decoded",
    ));
    output.push('}');
}

pub(crate) fn table_grid_sparse_sibling_post_row_gap_line_mark_correlation_counts(
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
) -> (usize, usize) {
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        return (0, 0);
    }

    let mut match_count = 0usize;
    let mut exact_count = 0usize;
    for (row_index, row) in sibling.rows.iter().enumerate() {
        let Some(interval) = candidate
            .intervals()
            .iter()
            .find(|interval| interval.index() == row.compact_row_index)
        else {
            continue;
        };
        let Some(line_mark) =
            table_grid_interval_line_mark(candidate, interval, &line_mark_intervals)
        else {
            continue;
        };
        let Some(gap) = table_grid_sparse_sibling_post_row_gap(sibling, row_index) else {
            continue;
        };
        match_count += 1;
        let line_mark_span_units = line_mark.unit_end.saturating_sub(line_mark.unit_start);
        let post_row_gap_units = gap.source_end.saturating_sub(gap.source_start);
        if line_mark_span_units == post_row_gap_units {
            exact_count += 1;
        }
    }
    (match_count, exact_count)
}

pub(crate) fn table_grid_page_mark_raw_scan_header_match_summary(
    document: &Document,
    line_mark_record_indexes: &[usize],
) -> (Option<usize>, Option<bool>) {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        return (None, None);
    };
    let record_headers = page_mark_record_headers(page_mark_bytes);
    if record_headers.is_empty() || line_mark_record_indexes.is_empty() {
        return (Some(record_headers.len()), None);
    }
    let matched_indexes = line_mark_record_indexes
        .iter()
        .filter_map(|record_index| {
            record_headers.iter().position(|header| {
                header.line_start as usize <= *record_index
                    && *record_index <= header.line_end as usize
            })
        })
        .collect::<Vec<_>>();
    let single_header_matched = matched_indexes.len() == line_mark_record_indexes.len()
        && matched_indexes.first().is_some_and(|first| {
            matched_indexes
                .iter()
                .all(|matched_index| matched_index == first)
        });
    (Some(record_headers.len()), Some(single_header_matched))
}

pub(crate) fn table_grid_sparse_sibling_post_row_gap(
    sibling: &TableGridSparseSiblingEvidence<'_>,
    row_index: usize,
) -> Option<TableGridSparseSiblingPostRowGap> {
    let row = sibling.rows.get(row_index)?;
    let source_start = row.source_end;
    let (source_end, sparse_row_indexes, sparse_source_interval_indexes, kind) =
        if let Some(next_row) = sibling.rows.get(row_index + 1) {
            let gap_rows = sibling
                .sparse_candidate
                .intervals()
                .iter()
                .filter(|interval| {
                    row.sparse_row_index < interval.index()
                        && interval.index() < next_row.sparse_row_index
                })
                .collect::<Vec<_>>();
            (
                next_row.source_start,
                gap_rows
                    .iter()
                    .map(|interval| interval.index())
                    .collect::<Vec<_>>(),
                gap_rows
                    .iter()
                    .map(|interval| interval.source_interval_index())
                    .collect::<Vec<_>>(),
                "between-matched-sparse-rows",
            )
        } else {
            let trailing_empty_rows = sibling
                .sparse_candidate
                .intervals()
                .iter()
                .filter(|interval| interval.index() > row.sparse_row_index)
                .take_while(|interval| table_candidate_interval_non_empty_cell_count(interval) == 0)
                .collect::<Vec<_>>();
            let last_empty_row = trailing_empty_rows.last()?;
            (
                last_empty_row.source_end(),
                trailing_empty_rows
                    .iter()
                    .map(|interval| interval.index())
                    .collect::<Vec<_>>(),
                trailing_empty_rows
                    .iter()
                    .map(|interval| interval.source_interval_index())
                    .collect::<Vec<_>>(),
                "trailing-empty-sparse-rows",
            )
        };
    if source_end <= source_start {
        return None;
    }
    Some(TableGridSparseSiblingPostRowGap {
        source_start,
        source_end,
        sparse_row_indexes,
        sparse_source_interval_indexes,
        kind,
    })
}

pub(crate) fn push_table_grid_line_mark_row_gap_sequence_evidence_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
) {
    let Some(line_mark_bytes) = raw_stream_bytes(document, LINE_MARK_PATH) else {
        output.push_str("null");
        return;
    };
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() || sibling.rows.is_empty() {
        output.push_str("null");
        return;
    }

    let rows = table_grid_line_mark_row_gap_sequence_rows(candidate, sibling, &line_mark_intervals);
    if rows.is_empty() {
        output.push_str("null");
        return;
    }

    let words = be16_words(line_mark_bytes).collect::<Vec<_>>();
    let tag_family_counts = line_mark_tag_family_counts(&words);
    let tag_count = tag_family_counts.iter().sum::<usize>();
    let selected_post_gap_match_count = rows
        .iter()
        .filter(|row| {
            row.post_row_gap.as_ref().is_some_and(|gap| {
                line_mark_interval_span_units(row.selected_line_mark)
                    == table_grid_source_span_units(
                        candidate.basis(),
                        gap.source_start,
                        gap.source_end,
                    )
            })
        })
        .count();
    let previous_row_span_match_count = rows
        .iter()
        .filter(|row| {
            row.previous_line_mark.is_some_and(|previous| {
                line_mark_interval_span_units(previous)
                    == row
                        .row_source_end_units
                        .saturating_sub(row.row_source_start_units)
            })
        })
        .count();
    let next_row_span_match_count = rows
        .iter()
        .filter(|row| {
            row.next_line_mark.zip(row.next_row_span_units).is_some_and(
                |(next, next_row_span_units)| {
                    line_mark_interval_span_units(next) == next_row_span_units
                },
            )
        })
        .count();
    let rows_with_next_row = rows
        .iter()
        .filter(|row| row.next_row_span_units.is_some())
        .count();

    output.push_str("{\"source\":\"/LineMark+sparseTableSiblingEvidence\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(shanai_lan_line_mark_profile(document)));
    output.push_str(",\"lineMarkStreamByteLength\":");
    output.push_str(&line_mark_bytes.len().to_string());
    output.push_str(",\"lineMarkWordCount\":");
    output.push_str(&words.len().to_string());
    output.push_str(",\"lineMarkTagCount\":");
    output.push_str(&tag_count.to_string());
    output.push_str(",\"lineMarkTagFamilyCounts\":");
    push_line_mark_tag_family_counts_json(output, tag_family_counts);
    output.push_str(",\"tagPayloadCorrelation\":");
    output.push_str(&json_string(if tag_count == 0 {
        "not-applicable-no-line-mark-tags"
    } else {
        "nearest-tag-window-diagnostic-only"
    }));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&sibling.rows.len().to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"selectedRecordPostRowGapSpanMatchCount\":");
    output.push_str(&selected_post_gap_match_count.to_string());
    output.push_str(",\"allSelectedRecordsMatchPostRowGapSpan\":");
    output.push_str(if selected_post_gap_match_count == rows.len() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousRecordRowSpanMatchCount\":");
    output.push_str(&previous_row_span_match_count.to_string());
    output.push_str(",\"allPreviousRecordsMatchRowSpan\":");
    output.push_str(if previous_row_span_match_count == rows.len() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nextRecordNextRowSpanMatchCount\":");
    output.push_str(&next_row_span_match_count.to_string());
    output.push_str(",\"rowsWithNextRow\":");
    output.push_str(&rows_with_next_row.to_string());
    output.push_str(",\"sequenceInterpretationCandidate\":");
    output.push_str(&json_string(
        "alternating-row-span-record-then-post-row-gap-record",
    ));
    output.push_str(
        ",\"renderPromotionContribution\":\"line-mark-row-gap-sequence-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-row-gap-sequence-does-not-decode-page-y-transform",
    ));
    output.push_str(",\"rows\":[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_line_mark_row_gap_sequence_row_json(output, candidate, &words, row);
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_line_mark_row_gap_sequence_y_comparison_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
    row_height: f32,
    reference_layout: &TableGridReferenceLayout,
) {
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() || sibling.rows.is_empty() || row_height <= 0.0 {
        output.push_str("null");
        return;
    }
    let rows = table_grid_line_mark_row_gap_sequence_rows(candidate, sibling, &line_mark_intervals);
    if rows.is_empty() {
        output.push_str("null");
        return;
    }
    let selected_record_indexes = rows
        .iter()
        .map(|row| row.selected_line_mark.record_index)
        .collect::<Vec<_>>();
    let previous_record_indexes = rows
        .iter()
        .filter_map(|row| {
            row.previous_line_mark
                .map(|line_mark| line_mark.record_index)
        })
        .collect::<Vec<_>>();
    if selected_record_indexes.len() != rows.len() || previous_record_indexes.len() != rows.len() {
        output.push_str("null");
        return;
    }
    let Some(selected_context) = table_grid_page_mark_context_for_line_mark_record_indexes(
        document,
        &selected_record_indexes,
    ) else {
        output.push_str("null");
        return;
    };
    let previous_context = table_grid_page_mark_context_for_line_mark_record_indexes(
        document,
        &previous_record_indexes,
    );
    let row_count = rows.len();
    let reference_row_tops = (0..row_count)
        .map(|row| reference_layout.y + row as f32 * reference_layout.row_height)
        .collect::<Vec<_>>();
    let selected_body_line_pitch = table_grid_page_mark_line_pitch_candidate(
        layout,
        selected_context.page_line_start,
        selected_context.page_line_end,
    )
    .map(|(pitch, _)| pitch);
    let previous_body_line_pitch = previous_context.as_ref().and_then(|context| {
        table_grid_page_mark_line_pitch_candidate(
            layout,
            context.page_line_start,
            context.page_line_end,
        )
        .map(|(pitch, _)| pitch)
    });

    let selected_row_height_residuals = line_mark_record_indexes_y_residuals(
        layout,
        &selected_record_indexes,
        selected_context.page_line_start,
        row_height,
        &reference_row_tops,
    );
    let previous_row_height_residuals = previous_context.as_ref().map(|context| {
        line_mark_record_indexes_y_residuals(
            layout,
            &previous_record_indexes,
            context.page_line_start,
            row_height,
            &reference_row_tops,
        )
    });
    let selected_body_line_residuals = selected_body_line_pitch.map(|pitch| {
        line_mark_record_indexes_y_residuals(
            layout,
            &selected_record_indexes,
            selected_context.page_line_start,
            pitch,
            &reference_row_tops,
        )
    });
    let previous_body_line_residuals =
        previous_context
            .as_ref()
            .zip(previous_body_line_pitch)
            .map(|(context, pitch)| {
                line_mark_record_indexes_y_residuals(
                    layout,
                    &previous_record_indexes,
                    context.page_line_start,
                    pitch,
                    &reference_row_tops,
                )
            });
    let candidates = [
        (
            "selected-spacing-records-row-height-pitch",
            max_abs_f32(&selected_row_height_residuals),
        ),
        (
            "previous-row-span-records-row-height-pitch",
            previous_row_height_residuals
                .as_ref()
                .and_then(|residuals| max_abs_f32(residuals)),
        ),
        (
            "selected-spacing-records-page-line-pitch",
            selected_body_line_residuals
                .as_ref()
                .and_then(|residuals| max_abs_f32(residuals)),
        ),
        (
            "previous-row-span-records-page-line-pitch",
            previous_body_line_residuals
                .as_ref()
                .and_then(|residuals| max_abs_f32(residuals)),
        ),
    ];
    let best_candidate = candidates
        .iter()
        .filter_map(|(name, residual)| residual.map(|value| (*name, value)))
        .min_by(|(_, left), (_, right)| left.partial_cmp(right).unwrap_or(Ordering::Equal));

    output.push_str("{\"source\":\"/LineMark row/gap sequence+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"rowCountCompared\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"referenceRowTops\":");
    push_f32_array_json(output, &reference_row_tops);
    output.push_str(",\"selectedSpacingRecordIndexes\":");
    push_usize_array_json(output, &selected_record_indexes);
    output.push_str(",\"previousRowSpanRecordIndexes\":");
    push_usize_array_json(output, &previous_record_indexes);
    output.push_str(",\"recordFamilyInterpretation\":\"selected-records-match-post-row-gaps-previous-records-match-row-spans\"");
    output.push_str(",\"selectedSpacingRecordCandidate\":");
    push_line_mark_record_family_y_candidate_json(
        output,
        layout,
        "selected-spacing-records",
        "post-row-gap-span",
        &selected_record_indexes,
        &selected_context,
        row_height,
        selected_body_line_pitch,
        &reference_row_tops,
    );
    output.push_str(",\"previousRowSpanRecordCandidate\":");
    match previous_context.as_ref() {
        Some(context) => push_line_mark_record_family_y_candidate_json(
            output,
            layout,
            "previous-row-span-records",
            "compact-row-span",
            &previous_record_indexes,
            context,
            row_height,
            previous_body_line_pitch,
            &reference_row_tops,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestCandidate\":");
    match best_candidate {
        Some((name, _)) => output.push_str(&json_string(name)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestCandidateMaxAbsResidualPx\":");
    push_optional_f32_json(output, best_candidate.map(|(_, residual)| residual));
    output.push_str(",\"renderPromotionContribution\":\"row-gap-record-family-y-diagnostic-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-record-family-y-transform-not-source-decoded",
    ));
    output.push('}');
}

pub(crate) fn table_grid_page_mark_context_for_line_mark_record_indexes(
    document: &Document,
    record_indexes: &[usize],
) -> Option<TableGridPageMarkLineContext> {
    if record_indexes.is_empty() {
        return None;
    }
    let page_mark = document.page_marks().first()?;
    let mut entries = Vec::new();
    for record_index in record_indexes {
        entries.push(table_grid_page_mark_entry_for_line_mark_record(
            Some(page_mark),
            *record_index,
        )?);
    }
    let first_entry = entries.first().copied()?;
    if !entries
        .iter()
        .all(|entry| entry.row_index() == first_entry.row_index())
    {
        return None;
    }
    Some(TableGridPageMarkLineContext {
        page_mark_entry_index: first_entry.row_index(),
        page_index_candidate: first_entry.index().map(|index| index as usize),
        page_line_start: first_entry.line_start()? as usize,
        page_line_end: first_entry.line_end()? as usize,
        page_mark_u16_fields: first_entry.u16_fields().to_vec(),
    })
}

pub(crate) fn table_grid_line_mark_row_gap_sequence_rows(
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) -> Vec<TableGridLineMarkRowGapSequenceRow> {
    let mut rows = Vec::new();
    for (row_index, row) in sibling.rows.iter().enumerate() {
        let Some(interval) = candidate
            .intervals()
            .iter()
            .find(|interval| interval.index() == row.compact_row_index)
        else {
            continue;
        };
        let Some(selected_line_mark) =
            table_grid_interval_line_mark(candidate, interval, line_mark_intervals)
        else {
            continue;
        };
        let previous_line_mark =
            selected_line_mark
                .record_index
                .checked_sub(1)
                .and_then(|record_index| {
                    line_mark_interval_for_record(line_mark_intervals, record_index)
                });
        let next_line_mark =
            line_mark_interval_for_record(line_mark_intervals, selected_line_mark.record_index + 1);
        let next_row_span_units = sibling.rows.get(row_index + 1).map(|next_row| {
            table_grid_source_span_units(
                candidate.basis(),
                next_row.source_start,
                next_row.source_end,
            )
        });
        rows.push(TableGridLineMarkRowGapSequenceRow {
            compact_row_index: row.compact_row_index,
            sparse_row_index: row.sparse_row_index,
            source_interval_index: row.source_interval_index,
            row_source_start: row.source_start,
            row_source_end: row.source_end,
            row_source_start_units: table_source_offset_to_units(
                candidate.basis(),
                row.source_start,
            ),
            row_source_end_units: table_source_offset_to_units(candidate.basis(), row.source_end),
            selected_line_mark,
            previous_line_mark,
            next_line_mark,
            post_row_gap: table_grid_sparse_sibling_post_row_gap(sibling, row_index),
            next_row_span_units,
        });
    }
    rows
}

pub(crate) fn table_grid_previous_row_span_line_mark_record_indexes(
    document: &Document,
    candidate: &TableCandidate,
) -> Vec<usize> {
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    table_grid_previous_row_span_line_mark_rows_for_rows(
        document,
        candidate,
        &rows,
        &line_mark_intervals,
    )
    .map(|rows| {
        rows.into_iter()
            .map(|row| row.interval.record_index)
            .collect()
    })
    .unwrap_or_default()
}

pub(crate) fn table_grid_resolved_line_mark_rows_for_rows(
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
) -> Vec<TableGridResolvedLineMarkRow> {
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if rows.is_empty() || line_mark_intervals.is_empty() {
        return Vec::new();
    }

    if let Some(rows) = table_grid_previous_row_span_line_mark_rows_for_rows(
        document,
        candidate,
        rows,
        &line_mark_intervals,
    ) {
        return rows;
    }

    rows.iter()
        .filter_map(|row| {
            let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start);
            let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end);
            best_line_mark_interval_for_unit_range(
                &line_mark_intervals,
                row_unit_start,
                row_unit_end,
            )
            .map(|interval| TableGridResolvedLineMarkRow {
                interval,
                role: TableGridLineMarkRowRecordRole::SelectedOverlap,
            })
        })
        .collect()
}

pub(crate) fn table_grid_previous_row_span_line_mark_rows_for_rows(
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) -> Option<Vec<TableGridResolvedLineMarkRow>> {
    if rows.is_empty() || rows.len() != candidate.intervals().len() {
        return None;
    }
    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate)?;
    let matched_segment_count = sibling
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    if sibling.rows.len() != candidate.intervals().len()
        || matched_segment_count != candidate.cell_count_candidate()
    {
        return None;
    }

    let sequence_rows =
        table_grid_line_mark_row_gap_sequence_rows(candidate, &sibling, line_mark_intervals);
    if sequence_rows.len() != rows.len() {
        return None;
    }

    let mut resolved_rows = Vec::new();
    for row in rows {
        let sequence_row = sequence_rows.iter().find(|sequence_row| {
            sequence_row.compact_row_index == row.row_index
                && sequence_row.row_source_start == row.source_start
                && sequence_row.row_source_end == row.source_end
        })?;
        let previous = sequence_row.previous_line_mark?;
        let row_span_units = sequence_row
            .row_source_end_units
            .saturating_sub(sequence_row.row_source_start_units);
        if line_mark_interval_span_units(previous) != row_span_units {
            return None;
        }
        let selected_record_matches_post_row_gap =
            sequence_row.post_row_gap.as_ref().is_some_and(|gap| {
                line_mark_interval_span_units(sequence_row.selected_line_mark)
                    == table_grid_source_span_units(
                        candidate.basis(),
                        gap.source_start,
                        gap.source_end,
                    )
            });
        if !selected_record_matches_post_row_gap {
            return None;
        }
        resolved_rows.push(TableGridResolvedLineMarkRow {
            interval: previous,
            role: TableGridLineMarkRowRecordRole::PreviousCompactRowSpan,
        });
    }

    resolved_rows
        .windows(2)
        .all(|pair| pair[0].interval.record_index < pair[1].interval.record_index)
        .then_some(resolved_rows)
}

pub(crate) fn push_table_grid_line_mark_row_gap_sequence_row_json(
    output: &mut String,
    candidate: &TableCandidate,
    words: &[u16],
    row: &TableGridLineMarkRowGapSequenceRow,
) {
    let row_span_units = row
        .row_source_end_units
        .saturating_sub(row.row_source_start_units);
    let post_row_gap_units = row.post_row_gap.as_ref().map(|gap| {
        table_grid_source_span_units(candidate.basis(), gap.source_start, gap.source_end)
    });
    let selected_record_matches_post_row_gap = post_row_gap_units
        .is_some_and(|units| line_mark_interval_span_units(row.selected_line_mark) == units);
    let previous_record_matches_row_span = row
        .previous_line_mark
        .is_some_and(|previous| line_mark_interval_span_units(previous) == row_span_units);
    let next_record_matches_next_row_span = row
        .next_line_mark
        .zip(row.next_row_span_units)
        .is_some_and(|(next, next_row_span_units)| {
            line_mark_interval_span_units(next) == next_row_span_units
        });

    output.push_str("{\"compactRow\":");
    output.push_str(&row.compact_row_index.to_string());
    output.push_str(",\"sparseRow\":");
    output.push_str(&row.sparse_row_index.to_string());
    output.push_str(",\"sourceIntervalIndex\":");
    output.push_str(&row.source_interval_index.to_string());
    output.push_str(",\"rowSourceRange\":");
    output.push_str(&source_range_json(row.row_source_start, row.row_source_end));
    output.push_str(",\"rowSourceUnitRange\":");
    output.push_str(&source_range_json(
        row.row_source_start_units,
        row.row_source_end_units,
    ));
    output.push_str(",\"rowSpanUnits\":");
    output.push_str(&row_span_units.to_string());
    output.push_str(",\"selectedLineMarkRecord\":");
    push_line_mark_delta_record_json(output, words, Some(row.selected_line_mark));
    output.push_str(",\"selectedRecordMatchesPostRowGapSpan\":");
    output.push_str(if selected_record_matches_post_row_gap {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousLineMarkRecord\":");
    push_line_mark_delta_record_json(output, words, row.previous_line_mark);
    output.push_str(",\"previousRecordMatchesRowSpan\":");
    output.push_str(if previous_record_matches_row_span {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nextLineMarkRecord\":");
    push_line_mark_delta_record_json(output, words, row.next_line_mark);
    output.push_str(",\"nextRowSpanUnits\":");
    push_option_usize_json(output, row.next_row_span_units);
    output.push_str(",\"nextRecordMatchesNextRowSpan\":");
    output.push_str(if next_record_matches_next_row_span {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"postRowGap\":");
    match row.post_row_gap.as_ref() {
        Some(gap) => {
            let source_start_units =
                table_source_offset_to_units(candidate.basis(), gap.source_start);
            let source_end_units = table_source_offset_to_units(candidate.basis(), gap.source_end);
            output.push_str("{\"sourceRange\":");
            output.push_str(&source_range_json(gap.source_start, gap.source_end));
            output.push_str(",\"sourceUnitRange\":");
            output.push_str(&source_range_json(source_start_units, source_end_units));
            output.push_str(",\"spanUnits\":");
            output.push_str(
                &source_end_units
                    .saturating_sub(source_start_units)
                    .to_string(),
            );
            output.push_str(",\"kind\":");
            output.push_str(&json_string(gap.kind));
            output.push_str(",\"sparseRowIndexes\":");
            push_usize_array_json(output, &gap.sparse_row_indexes);
            output.push_str(",\"sparseSourceIntervalIndexes\":");
            push_usize_array_json(output, &gap.sparse_source_interval_indexes);
            output.push('}');
        }
        None => output.push_str("null"),
    }
    output.push('}');
}
