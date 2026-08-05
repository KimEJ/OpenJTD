use super::*;
use crate::*;

pub(crate) fn push_table_grid_source_derived_horizontal_comparison_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    reference_layout: TableGridReferenceLayout,
) {
    let Some(grid) = candidate.column_segment_grid_candidate() else {
        output.push_str("null");
        return;
    };
    let Some(source_layout) = source_derived_table_grid_overlay_layout(
        layout,
        document,
        lines,
        0,
        candidate,
        grid.column_count(),
    ) else {
        output.push_str("null");
        return;
    };

    let source_right = source_layout.x + source_layout.width;
    let reference_right = reference_layout.x + reference_layout.width;
    let width_residual = reference_layout.width - source_layout.width;
    let x_residual = reference_layout.x - source_layout.x;
    let right_residual = reference_right - source_right;

    output.push_str("{\"source\":\"sourceDerivedLayoutCandidate+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourceBBox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        source_layout.x, source_layout.y, source_layout.width, source_layout.height
    ));
    output.push_str(",\"referenceBBox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        reference_layout.x,
        reference_layout.y,
        reference_layout.width,
        reference_layout.row_height * source_layout.row_count as f32
    ));
    output.push_str(",\"xResidualPx\":");
    output.push_str(&format!("{x_residual:.3}"));
    output.push_str(",\"widthResidualPx\":");
    output.push_str(&format!("{width_residual:.3}"));
    output.push_str(",\"rightResidualPx\":");
    output.push_str(&format!("{right_residual:.3}"));
    output.push_str(",\"widthResidualAbsPx\":");
    output.push_str(&format!("{:.3}", width_residual.abs()));
    output.push_str(",\"xResidualAbsPx\":");
    output.push_str(&format!("{:.3}", x_residual.abs()));
    output.push_str(",\"xUnitRangeBasis\":");
    output.push_str(&json_string(source_layout.x_unit_range_basis));
    output.push_str(",\"xUnitRange\":");
    output.push_str(&source_range_json(
        usize::from(source_layout.x_unit_start),
        usize::from(source_layout.x_unit_end),
    ));
    output.push_str(",\"pageOriginAuthority\":");
    output.push_str(&json_string(source_layout.page_origin_authority));
    output.push_str(",\"widthAgreementStrong\":");
    output.push_str(if width_residual.abs() <= 1.5 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"xOriginAgreementStrong\":");
    output.push_str(if x_residual.abs() <= 1.5 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"originResidualEvidence\":");
    push_table_grid_source_origin_residual_evidence_json(
        output,
        document,
        candidate,
        &source_layout,
        &reference_layout,
    );
    output.push_str(",\"lineMarkStrideYComparison\":");
    push_table_grid_line_mark_stride_y_reference_comparison_json(
        output,
        &source_layout,
        &reference_layout,
    );
    output
        .push_str(",\"renderPromotionContribution\":\"source-width-corroboration-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("horizontal-x-origin-unproven"));
    output.push('}');
}

pub(crate) fn push_table_grid_source_derived_horizontal_field_adjustment_probe_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) {
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

    let source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let Some((page_mark_fields, page_mark_field_source)) = source_layout_page_mark_fields
        .map(|fields| (fields, "sourceDerivedLayoutCandidate"))
        .or_else(|| {
            cross_table_page_mark_fields
                .map(|fields| (fields, "crossTableRowBoundaryOffsetConsistency"))
        })
    else {
        output.push_str("null");
        return;
    };

    let first_slot_units = source_layout
        .as_ref()
        .and_then(|layout| layout.x_unit_column_slot_width_units.first().copied());
    let first_span_units = source_layout
        .as_ref()
        .and_then(|layout| layout.matched_cell_span_units.first().copied());
    let first_gap_units = source_layout
        .as_ref()
        .and_then(|layout| layout.matched_cell_gap_units.first().copied());
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let raw_header_count = rows.iter().map(|row| row.headers.len()).sum::<usize>();
    let matched_row_count = rows.iter().filter(|row| row.matched_cell_count > 0).count();
    let reference_right = reference_layout.x + reference_layout.width;

    output.push_str(
        "{\"source\":\"/PageMark selected u16 fields+referenceTableBBox+documentTextLineHeaders\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"pageMarkFieldSource\":");
    output.push_str(&json_string(page_mark_field_source));
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referenceBBox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3},\"right\":{:.3}}}",
        reference_layout.x,
        reference_layout.y,
        reference_layout.width,
        reference_layout.row_height * candidate.intervals().len() as f32,
        reference_right
    ));
    output.push_str(",\"pageMarkEntryIndex\":");
    push_option_usize_json(
        output,
        cross_table_probe
            .as_ref()
            .and_then(|probe| probe.page_mark_entry_index),
    );
    output.push_str(",\"pageLineStart\":");
    push_option_usize_json(
        output,
        cross_table_probe
            .as_ref()
            .and_then(|probe| probe.page_line_start),
    );
    output.push_str(",\"pageLineEnd\":");
    push_option_usize_json(
        output,
        cross_table_probe
            .as_ref()
            .and_then(|probe| probe.page_line_end),
    );
    output.push_str(",\"selectedFields\":");
    push_table_grid_horizontal_reference_page_mark_fields_json(output, page_mark_fields);
    output.push_str(",\"lineHeaderSlotEvidence\":{\"rowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&matched_row_count.to_string());
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&raw_header_count.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    push_optional_u16_json(output, first_slot_units);
    output.push_str(",\"firstMatchedCellSpanUnits\":");
    push_optional_u16_json(output, first_span_units);
    output.push_str(",\"firstIntercellGapUnits\":");
    push_optional_u16_json(output, first_gap_units);
    output.push('}');
    output.push_str(",\"directFieldTargetComparisons\":");
    push_table_grid_horizontal_field_target_comparisons_json(
        output,
        page_mark_fields,
        &[
            ("x", reference_layout.x),
            ("width", reference_layout.width),
            ("right", reference_right),
        ],
    );
    output.push_str(",\"bestDirectXField\":");
    push_table_grid_best_horizontal_field_target_json(output, page_mark_fields, reference_layout.x);
    output.push_str(",\"bestDirectWidthField\":");
    push_table_grid_best_horizontal_field_target_json(
        output,
        page_mark_fields,
        reference_layout.width,
    );
    output.push_str(",\"bestDirectRightField\":");
    push_table_grid_best_horizontal_field_target_json(output, page_mark_fields, reference_right);
    output.push_str(",\"slotAdjustedFieldTargetComparisons\":");
    push_table_grid_slot_adjusted_horizontal_field_target_comparisons_json(
        output,
        page_mark_fields,
        first_slot_units,
        reference_layout.x,
        reference_layout.width,
    );
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-horizontal-field-adjustment-probe\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_source_only_horizontal_field_consensus_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) {
    let current_source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = current_source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let Some((page_mark_fields, page_mark_field_source)) = source_layout_page_mark_fields
        .map(|fields| (fields, "sourceDerivedLayoutCandidate"))
        .or_else(|| {
            cross_table_page_mark_fields
                .map(|fields| (fields, "crossTableRowBoundaryOffsetConsistency"))
        })
    else {
        output.push_str("null");
        return;
    };

    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate);
    let sparse_table_candidate_index = sibling
        .as_ref()
        .map(|evidence| evidence.sparse_candidate.index());
    let related_source_layouts = table_grid_related_horizontal_source_layout_summaries(
        layout,
        document,
        lines,
        candidate,
        sparse_table_candidate_index,
        current_source_layout.as_ref(),
    );
    if related_source_layouts.is_empty() {
        output.push_str("null");
        return;
    }

    let related_table_candidate_indexes = table_grid_sparse_sibling_related_table_candidate_indexes(
        document,
        sparse_table_candidate_index,
        candidate,
    );
    let source_layout_candidate_indexes = related_source_layouts
        .iter()
        .map(|summary| summary.table_candidate_index)
        .collect::<Vec<_>>();
    let first_slot_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_column_slot_units)
        .collect::<Vec<_>>();
    let first_span_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_matched_cell_span_units)
        .collect::<Vec<_>>();
    let first_gap_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_intercell_gap_units)
        .collect::<Vec<_>>();
    let x_unit_starts = related_source_layouts
        .iter()
        .map(|summary| summary.x_unit_start)
        .collect::<Vec<_>>();
    let x_unit_ends = related_source_layouts
        .iter()
        .map(|summary| summary.x_unit_end)
        .collect::<Vec<_>>();
    let full_extent_units = related_source_layouts
        .iter()
        .map(|summary| summary.x_unit_full_extent_units)
        .filter(|units| *units > 0)
        .collect::<Vec<_>>();
    let stable_first_slot_units = single_u16_value(&first_slot_units);
    let stable_first_span_units = single_u16_value(&first_span_units);
    let stable_first_gap_units = single_u16_value(&first_gap_units);
    let stable_x_unit_start = single_u16_value(&x_unit_starts);
    let stable_x_unit_end = single_u16_value(&x_unit_ends);
    let stable_full_extent_units = single_u16_value(&full_extent_units);
    let all_related_layouts_have_stable_unit_frame = stable_first_slot_units.is_some()
        && stable_first_span_units.is_some()
        && stable_first_gap_units.is_some()
        && stable_x_unit_start.is_some()
        && stable_x_unit_end.is_some()
        && stable_full_extent_units.is_some()
        && related_source_layouts
            .iter()
            .all(|summary| summary.x_unit_all_rows_agree);

    output
        .push_str("{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark horizontal fields\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"pageMarkFieldSource\":");
    output.push_str(&json_string(page_mark_field_source));
    output.push_str(",\"sparseTableCandidateIndex\":");
    push_option_usize_json(output, sparse_table_candidate_index);
    output.push_str(",\"relatedTableCandidateIndexes\":");
    push_usize_array_json(output, &related_table_candidate_indexes);
    output.push_str(",\"sourceDerivedRelatedTableCandidateIndexes\":");
    push_usize_array_json(output, &source_layout_candidate_indexes);
    output.push_str(",\"sourceDerivedRelatedTableCount\":");
    output.push_str(&related_source_layouts.len().to_string());
    output.push_str(",\"stableFirstColumnSlotUnits\":");
    push_optional_u16_json(output, stable_first_slot_units);
    output.push_str(",\"stableFirstMatchedCellSpanUnits\":");
    push_optional_u16_json(output, stable_first_span_units);
    output.push_str(",\"stableFirstIntercellGapUnits\":");
    push_optional_u16_json(output, stable_first_gap_units);
    output.push_str(",\"stableXUnitRange\":");
    match (stable_x_unit_start, stable_x_unit_end) {
        (Some(start), Some(end)) => {
            output.push_str(&source_range_json(usize::from(start), usize::from(end)));
        }
        _ => output.push_str("null"),
    }
    output.push_str(",\"stableFullExtentUnits\":");
    push_optional_u16_json(output, stable_full_extent_units);
    output.push_str(",\"allRelatedLayoutsHaveStableUnitFrame\":");
    output.push_str(if all_related_layouts_have_stable_unit_frame {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"relatedSourceLayouts\":");
    push_table_grid_related_horizontal_source_layout_summaries_json(
        output,
        &related_source_layouts,
    );
    output.push_str(",\"sourceOnlyFrameHypotheses\":");
    push_table_grid_source_only_horizontal_field_consensus_hypotheses_json(
        output,
        page_mark_fields,
        stable_first_slot_units,
        stable_first_gap_units,
    );
    output.push_str(",\"sourceOnlyHorizontalFieldSelector\":");
    push_table_grid_source_only_horizontal_field_selector_json(
        output,
        candidate,
        page_mark_fields,
        page_mark_field_source,
        stable_first_slot_units,
        stable_first_gap_units,
    );
    output.push_str(",\"renderPromotionContribution\":\"source-only-horizontal-field-consensus\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "cross-table-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

pub(crate) fn table_grid_related_horizontal_source_layout_summaries(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    sparse_table_candidate_index: Option<usize>,
    current_source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Vec<TableGridRelatedHorizontalSourceLayoutSummary> {
    let mut summaries = Vec::new();
    if let Some(sparse_table_candidate_index) = sparse_table_candidate_index {
        for related in document
            .table_candidates()
            .iter()
            .filter(|related| related.is_document_text_control_run_candidate())
            .filter(|related| {
                table_grid_sparse_table_sibling_evidence(document, related).is_some_and(
                    |evidence| evidence.sparse_candidate.index() == sparse_table_candidate_index,
                )
            })
        {
            let Some(grid) = related.column_segment_grid_candidate() else {
                continue;
            };
            let Some(source_layout) = source_derived_table_grid_overlay_layout(
                layout,
                document,
                lines,
                0,
                related,
                grid.column_count(),
            ) else {
                continue;
            };
            summaries.push(table_grid_related_horizontal_source_layout_summary(
                related.index(),
                &source_layout,
            ));
        }
    } else if let Some(source_layout) = current_source_layout {
        summaries.push(table_grid_related_horizontal_source_layout_summary(
            candidate.index(),
            source_layout,
        ));
    }

    summaries.sort_by_key(|summary| summary.table_candidate_index);
    summaries.dedup_by_key(|summary| summary.table_candidate_index);
    summaries
}

pub(crate) fn table_grid_sparse_sibling_related_table_candidate_indexes(
    document: &Document,
    sparse_table_candidate_index: Option<usize>,
    candidate: &TableCandidate,
) -> Vec<usize> {
    let Some(sparse_table_candidate_index) = sparse_table_candidate_index else {
        return vec![candidate.index()];
    };
    let mut indexes = document
        .table_candidates()
        .iter()
        .filter(|related| related.is_document_text_control_run_candidate())
        .filter(|related| {
            table_grid_sparse_table_sibling_evidence(document, related).is_some_and(|evidence| {
                evidence.sparse_candidate.index() == sparse_table_candidate_index
            })
        })
        .map(|related| related.index())
        .collect::<Vec<_>>();
    indexes.sort_unstable();
    indexes.dedup();
    indexes
}

pub(crate) fn table_grid_related_horizontal_source_layout_summary(
    table_candidate_index: usize,
    source_layout: &TableGridSourceDerivedLayout,
) -> TableGridRelatedHorizontalSourceLayoutSummary {
    TableGridRelatedHorizontalSourceLayoutSummary {
        table_candidate_index,
        row_count: source_layout.row_count,
        column_count: source_layout.column_count,
        x_unit_start: source_layout.x_unit_start,
        x_unit_end: source_layout.x_unit_end,
        x_unit_full_extent_units: source_layout.x_unit_full_extent_units,
        x_unit_all_rows_agree: source_layout.x_unit_all_rows_agree,
        first_column_slot_units: source_layout
            .x_unit_column_slot_width_units
            .first()
            .copied(),
        first_matched_cell_span_units: source_layout.matched_cell_span_units.first().copied(),
        first_intercell_gap_units: source_layout.matched_cell_gap_units.first().copied(),
        matched_cell_span_units: source_layout.matched_cell_span_units.clone(),
        matched_cell_gap_units: source_layout.matched_cell_gap_units.clone(),
        x_unit_column_slot_width_units: source_layout.x_unit_column_slot_width_units.clone(),
    }
}

pub(crate) fn push_table_grid_related_horizontal_source_layout_summaries_json(
    output: &mut String,
    summaries: &[TableGridRelatedHorizontalSourceLayoutSummary],
) {
    output.push('[');
    for (index, summary) in summaries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"tableCandidateIndex\":");
        output.push_str(&summary.table_candidate_index.to_string());
        output.push_str(",\"rowCount\":");
        output.push_str(&summary.row_count.to_string());
        output.push_str(",\"columnCount\":");
        output.push_str(&summary.column_count.to_string());
        output.push_str(",\"xUnitRange\":");
        output.push_str(&source_range_json(
            usize::from(summary.x_unit_start),
            usize::from(summary.x_unit_end),
        ));
        output.push_str(",\"fullExtentUnits\":");
        output.push_str(&summary.x_unit_full_extent_units.to_string());
        output.push_str(",\"xUnitAllRowsAgree\":");
        output.push_str(if summary.x_unit_all_rows_agree {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"firstColumnSlotUnits\":");
        push_optional_u16_json(output, summary.first_column_slot_units);
        output.push_str(",\"firstMatchedCellSpanUnits\":");
        push_optional_u16_json(output, summary.first_matched_cell_span_units);
        output.push_str(",\"firstIntercellGapUnits\":");
        push_optional_u16_json(output, summary.first_intercell_gap_units);
        output.push_str(",\"matchedCellSpanUnits\":");
        push_u16_array_json(output, &summary.matched_cell_span_units);
        output.push_str(",\"matchedCellGapUnits\":");
        push_u16_array_json(output, &summary.matched_cell_gap_units);
        output.push_str(",\"columnSlotWidthUnits\":");
        push_u16_array_json(output, &summary.x_unit_column_slot_width_units);
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_table_grid_source_only_horizontal_field_consensus_hypotheses_json(
    output: &mut String,
    page_mark_fields: &[u16],
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
) {
    output.push('[');
    let mut emitted = false;
    push_table_grid_source_only_horizontal_field_consensus_hypotheses_items_json(
        output,
        page_mark_fields,
        stable_first_slot_units,
        stable_first_gap_units,
        &mut emitted,
    );
    output.push(']');
}

pub(crate) fn push_table_grid_source_only_horizontal_field_selector_json(
    output: &mut String,
    candidate: &TableCandidate,
    page_mark_fields: &[u16],
    page_mark_field_source: &'static str,
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
) {
    let Some(word_14) = page_mark_fields.get(14).copied() else {
        output.push_str("null");
        return;
    };
    let Some(first_slot_units) = stable_first_slot_units.filter(|units| *units > 0) else {
        output.push_str("null");
        return;
    };
    let compact_column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let word_15 = page_mark_fields.get(15).copied();
    let word_21 = page_mark_fields.get(21).copied();
    let selector = match compact_column_count {
        2 => word_21.map(|word_21| {
            (
                "page-mark-word14-first-slot-word21-half-slot",
                "compact-two-column-page-mark-word21-half-slot",
                21usize,
                word_21,
                f32::from(first_slot_units) * 0.5,
                "cross-table-half-first-column-slot",
                f32::from(word_21) - f32::from(first_slot_units) * 0.5,
            )
        }),
        3 => word_15.and_then(|word_15| {
            stable_first_gap_units
                .filter(|units| *units > 0)
                .map(|units| {
                    (
                        "page-mark-word14-first-slot-word15-half-gap",
                        "compact-three-column-page-mark-word15-half-gap",
                        15usize,
                        word_15,
                        f32::from(units) * 0.5,
                        "cross-table-half-first-intercell-gap",
                        f32::from(word_15) - f32::from(units) * 0.5,
                    )
                })
        }),
        _ => None,
    };
    let Some((
        selected_frame_basis,
        selection_basis,
        selected_width_word_index,
        page_mark_width_word,
        width_adjustment_units,
        width_adjustment_basis,
        selected_width,
    )) = selector
    else {
        output.push_str("null");
        return;
    };
    if !selected_x.is_finite() || !selected_width.is_finite() || selected_width <= 0.0 {
        output.push_str("null");
        return;
    }

    output
        .push_str("{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark horizontal fields\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsedForSelection\":false");
    output.push_str(",\"selectionReady\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"pageMarkFieldSource\":");
    output.push_str(&json_string(page_mark_field_source));
    output.push_str(",\"compactColumnCount\":");
    output.push_str(&compact_column_count.to_string());
    output.push_str(",\"selectionBasis\":");
    output.push_str(&json_string(selection_basis));
    output.push_str(",\"selectedFrameBasis\":");
    output.push_str(&json_string(selected_frame_basis));
    output.push_str(",\"pageMarkXWord14\":");
    output.push_str(&word_14.to_string());
    output.push_str(",\"pageMarkWidthWord\":");
    output.push_str(&page_mark_width_word.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    output.push_str(&first_slot_units.to_string());
    output.push_str(",\"firstIntercellGapUnits\":");
    push_optional_u16_json(output, stable_first_gap_units);
    output.push_str(",\"xAdjustmentUnits\":");
    output.push_str(&format!("{:.3}", f32::from(first_slot_units)));
    output.push_str(",\"widthAdjustmentUnits\":");
    output.push_str(&format!("{width_adjustment_units:.3}"));
    output.push_str(",\"xAdjustmentBasis\":\"cross-table-first-column-slot\"");
    output.push_str(",\"widthAdjustmentBasis\":");
    output.push_str(&json_string(width_adjustment_basis));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(",\"sourceWidthFieldRoleGate\":");
    push_table_grid_source_only_horizontal_width_field_role_gate_json(
        output,
        compact_column_count,
        page_mark_field_source,
        word_14,
        word_15,
        word_21,
        first_slot_units,
        stable_first_gap_units,
        selected_width_word_index,
        page_mark_width_word,
        width_adjustment_units,
        width_adjustment_basis,
        selected_frame_basis,
        selection_basis,
    );
    output.push_str(",\"renderPromotionContribution\":\"source-only-horizontal-field-selector\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "cross-table-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_source_only_horizontal_width_field_role_gate_json(
    output: &mut String,
    compact_column_count: usize,
    page_mark_field_source: &'static str,
    page_mark_x_word_14: u16,
    page_mark_word_15: Option<u16>,
    page_mark_word_21: Option<u16>,
    first_column_slot_units: u16,
    first_intercell_gap_units: Option<u16>,
    selected_width_word_index: usize,
    selected_width_word: u16,
    selected_width_adjustment_units: f32,
    selected_width_adjustment_basis: &'static str,
    selected_frame_basis: &'static str,
    selection_basis: &'static str,
) {
    let three_column_candidate_present = compact_column_count == 3
        && page_mark_word_15.is_some()
        && first_intercell_gap_units.is_some();
    let two_column_candidate_present = compact_column_count == 2 && page_mark_word_21.is_some();
    let selector_matches_compact_column_count = (compact_column_count == 2
        && selected_width_word_index == 21)
        || (compact_column_count == 3 && selected_width_word_index == 15);
    let selected_width_field_role = match (compact_column_count, selected_width_word_index) {
        (2, 21) => "compact-two-column-visible-width",
        (3, 15) => "compact-three-column-visible-width",
        (_, 21) => "word21-alternate-width-candidate",
        (_, 15) => "word15-alternate-width-candidate",
        _ => "unknown-width-field-role",
    };

    output.push_str(
        "{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark width-field role gate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false");
    output.push_str(",\"pageMarkFieldSource\":");
    output.push_str(&json_string(page_mark_field_source));
    output.push_str(",\"compactColumnCount\":");
    output.push_str(&compact_column_count.to_string());
    output.push_str(",\"pageMarkXWord14\":");
    output.push_str(&page_mark_x_word_14.to_string());
    output.push_str(",\"pageMarkWord15\":");
    push_optional_u16_json(output, page_mark_word_15);
    output.push_str(",\"pageMarkWord21\":");
    push_optional_u16_json(output, page_mark_word_21);
    output.push_str(",\"firstColumnSlotUnits\":");
    output.push_str(&first_column_slot_units.to_string());
    output.push_str(",\"firstIntercellGapUnits\":");
    push_optional_u16_json(output, first_intercell_gap_units);
    output.push_str(",\"selectedWidthWordIndex\":");
    output.push_str(&selected_width_word_index.to_string());
    output.push_str(",\"selectedWidthWord\":");
    output.push_str(&selected_width_word.to_string());
    output.push_str(",\"selectedWidthFieldRole\":");
    output.push_str(&json_string(selected_width_field_role));
    output.push_str(",\"selectedWidthAdjustmentUnits\":");
    output.push_str(&format!("{selected_width_adjustment_units:.3}"));
    output.push_str(",\"selectedWidthAdjustmentBasis\":");
    output.push_str(&json_string(selected_width_adjustment_basis));
    output.push_str(",\"selectedFrameBasis\":");
    output.push_str(&json_string(selected_frame_basis));
    output.push_str(",\"selectionBasis\":");
    output.push_str(&json_string(selection_basis));
    output.push_str(",\"twoColumnWidthCandidatePresent\":");
    output.push_str(if two_column_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"threeColumnWidthCandidatePresent\":");
    output.push_str(if three_column_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectorMatchesCompactColumnCount\":");
    output.push_str(if selector_matches_compact_column_count {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"source-horizontal-width-field-role-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "width-field-role-semantics-needs-cross-sample-validation",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_source_only_horizontal_field_consensus_hypotheses_items_json(
    output: &mut String,
    page_mark_fields: &[u16],
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
    emitted: &mut bool,
) {
    let Some(word_14) = page_mark_fields.get(14).copied() else {
        return;
    };
    let Some(word_15) = page_mark_fields.get(15).copied() else {
        return;
    };
    let Some(word_21) = page_mark_fields.get(21).copied() else {
        return;
    };
    let Some(first_slot_units) = stable_first_slot_units.filter(|units| *units > 0) else {
        return;
    };
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let half_gap_adjustment = stable_first_gap_units
        .filter(|units| *units > 0)
        .map(|units| f32::from(units) * 0.5);
    let half_slot_adjustment = f32::from(first_slot_units) * 0.5;

    if *emitted {
        output.push(',');
    }
    push_table_grid_source_only_horizontal_field_consensus_hypothesis_json(
        output,
        "page-mark-word14-first-slot-word15-direct",
        word_14,
        word_15,
        selected_x,
        f32::from(word_15),
        first_slot_units,
        f32::from(first_slot_units),
        0.0,
        "cross-table-first-column-slot",
        "none",
    );
    *emitted = true;
    if let Some(half_gap_adjustment) = half_gap_adjustment {
        output.push(',');
        push_table_grid_source_only_horizontal_field_consensus_hypothesis_json(
            output,
            "page-mark-word14-first-slot-word15-half-gap",
            word_14,
            word_15,
            selected_x,
            f32::from(word_15) - half_gap_adjustment,
            first_slot_units,
            f32::from(first_slot_units),
            half_gap_adjustment,
            "cross-table-first-column-slot",
            "cross-table-half-first-intercell-gap",
        );
    }
    output.push(',');
    push_table_grid_source_only_horizontal_field_consensus_hypothesis_json(
        output,
        "page-mark-word14-first-slot-word21-half-slot",
        word_14,
        word_21,
        selected_x,
        f32::from(word_21) - half_slot_adjustment,
        first_slot_units,
        f32::from(first_slot_units),
        half_slot_adjustment,
        "cross-table-first-column-slot",
        "cross-table-half-first-column-slot",
    );
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_source_only_horizontal_field_consensus_hypothesis_json(
    output: &mut String,
    frame_basis: &'static str,
    page_mark_x_word_14: u16,
    page_mark_width_word: u16,
    selected_x: f32,
    selected_width: f32,
    first_column_slot_units: u16,
    x_adjustment_units: f32,
    width_adjustment_units: f32,
    x_adjustment_basis: &'static str,
    width_adjustment_basis: &'static str,
) {
    output.push_str("{\"frameBasis\":");
    output.push_str(&json_string(frame_basis));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateBasis\":\"page-mark-u16-fields+cross-table-line-header-slots\"");
    output.push_str(",\"pageMarkXWord14\":");
    output.push_str(&page_mark_x_word_14.to_string());
    output.push_str(",\"pageMarkWidthWord\":");
    output.push_str(&page_mark_width_word.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    output.push_str(&first_column_slot_units.to_string());
    output.push_str(",\"xAdjustmentUnits\":");
    output.push_str(&format!("{x_adjustment_units:.3}"));
    output.push_str(",\"widthAdjustmentUnits\":");
    output.push_str(&format!("{width_adjustment_units:.3}"));
    output.push_str(",\"xAdjustmentBasis\":");
    output.push_str(&json_string(x_adjustment_basis));
    output.push_str(",\"widthAdjustmentBasis\":");
    output.push_str(&json_string(width_adjustment_basis));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "cross-table-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_horizontal_reference_page_mark_fields_json(
    output: &mut String,
    fields: &[u16],
) {
    output.push('[');
    for (index, word_index) in PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES
        .iter()
        .copied()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        let value = fields.get(word_index).copied();
        output.push_str("{\"wordIndex\":");
        output.push_str(&word_index.to_string());
        output.push_str(",\"value\":");
        push_optional_u16_json(output, value);
        output.push_str(",\"hex\":");
        push_option_u16_hex_json(output, value);
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_table_grid_horizontal_field_target_comparisons_json(
    output: &mut String,
    fields: &[u16],
    targets: &[(&str, f32)],
) {
    output.push('[');
    let mut emitted = 0usize;
    for word_index in PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES {
        let Some(value) = fields.get(word_index).copied() else {
            continue;
        };
        for (target_name, target_px) in targets {
            if emitted > 0 {
                output.push(',');
            }
            let value_px = f32::from(value);
            let residual = value_px - *target_px;
            output.push_str("{\"wordIndex\":");
            output.push_str(&word_index.to_string());
            output.push_str(",\"value\":");
            output.push_str(&value.to_string());
            output.push_str(",\"valuePx\":");
            output.push_str(&format!("{value_px:.3}"));
            output.push_str(",\"target\":");
            output.push_str(&json_string(target_name));
            output.push_str(",\"targetPx\":");
            output.push_str(&format!("{target_px:.3}"));
            output.push_str(",\"residualPx\":");
            output.push_str(&format!("{residual:.3}"));
            output.push_str(",\"absResidualPx\":");
            output.push_str(&format!("{:.3}", residual.abs()));
            output.push_str(",\"withinTwoPx\":");
            output.push_str(if residual.abs() <= 2.0 {
                "true"
            } else {
                "false"
            });
            output.push('}');
            emitted += 1;
        }
    }
    output.push(']');
}

pub(crate) fn push_table_grid_best_horizontal_field_target_json(
    output: &mut String,
    fields: &[u16],
    target_px: f32,
) {
    let best = PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES
        .iter()
        .copied()
        .filter_map(|word_index| {
            fields.get(word_index).copied().map(|value| {
                let residual = f32::from(value) - target_px;
                (word_index, value, residual)
            })
        })
        .min_by(|left, right| left.2.abs().total_cmp(&right.2.abs()));
    let Some((word_index, value, residual)) = best else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"wordIndex\":");
    output.push_str(&word_index.to_string());
    output.push_str(",\"value\":");
    output.push_str(&value.to_string());
    output.push_str(",\"targetPx\":");
    output.push_str(&format!("{target_px:.3}"));
    output.push_str(",\"residualPx\":");
    output.push_str(&format!("{residual:.3}"));
    output.push_str(",\"absResidualPx\":");
    output.push_str(&format!("{:.3}", residual.abs()));
    output.push('}');
}

pub(crate) fn push_table_grid_slot_adjusted_horizontal_field_target_comparisons_json(
    output: &mut String,
    fields: &[u16],
    first_slot_units: Option<u16>,
    reference_x: f32,
    reference_width: f32,
) {
    let Some(first_slot_units) = first_slot_units.filter(|units| *units > 0) else {
        output.push_str("[]");
        return;
    };
    let first_slot_px = f32::from(first_slot_units);
    let comparisons = [
        (
            14usize,
            "x",
            "line-header-first-column-slot",
            first_slot_px,
            reference_x,
        ),
        (
            21usize,
            "width",
            "line-header-half-first-column-slot",
            first_slot_px * 0.5,
            reference_width,
        ),
    ];

    output.push('[');
    let mut emitted = 0usize;
    for (word_index, target_name, adjustment_basis, adjustment_px, target_px) in comparisons {
        let Some(value) = fields.get(word_index).copied() else {
            continue;
        };
        if emitted > 0 {
            output.push(',');
        }
        let value_px = f32::from(value);
        let adjusted_value_px = value_px - adjustment_px;
        let residual = adjusted_value_px - target_px;
        output.push_str("{\"wordIndex\":");
        output.push_str(&word_index.to_string());
        output.push_str(",\"value\":");
        output.push_str(&value.to_string());
        output.push_str(",\"valuePx\":");
        output.push_str(&format!("{value_px:.3}"));
        output.push_str(",\"target\":");
        output.push_str(&json_string(target_name));
        output.push_str(",\"targetPx\":");
        output.push_str(&format!("{target_px:.3}"));
        output.push_str(",\"adjustmentBasis\":");
        output.push_str(&json_string(adjustment_basis));
        output.push_str(",\"adjustmentUnits\":");
        output.push_str(&format!("{adjustment_px:.3}"));
        output.push_str(",\"adjustedValuePx\":");
        output.push_str(&format!("{adjusted_value_px:.3}"));
        output.push_str(",\"residualPx\":");
        output.push_str(&format!("{residual:.3}"));
        output.push_str(",\"absResidualPx\":");
        output.push_str(&format!("{:.3}", residual.abs()));
        output.push('}');
        emitted += 1;
    }
    output.push(']');
}

pub(crate) fn push_table_grid_line_header_unit_geometry_candidate_json(
    output: &mut String,
    rows: &[TableCandidateLineHeaderRow],
) {
    let Some(first_row) = rows.first() else {
        output.push_str("null");
        return;
    };
    if first_row.headers.is_empty() {
        output.push_str("null");
        return;
    }
    let matched_column_count = rows
        .iter()
        .map(|row| row.matched_cell_count)
        .min()
        .unwrap_or(0);
    if matched_column_count == 0 {
        output.push_str("null");
        return;
    }

    let rows_homogeneous = rows.iter().all(|row| {
        row.headers.len() == first_row.headers.len()
            && row
                .headers
                .iter()
                .zip(&first_row.headers)
                .all(|(left, right)| {
                    left.offset_units == right.offset_units
                        && left.extent_units == right.extent_units
                        && left.font_size_units == right.font_size_units
                })
    });
    let min_offset_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.offset_units))
        .min();
    let max_extent_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.extent_units))
        .max();
    let homogeneous_font_size_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.font_size_units))
        .try_fold(None, |seen, value| match seen {
            Some(previous) if previous != value => None,
            _ => Some(Some(value)),
        })
        .flatten();
    let matched_cell_offset_units = first_row
        .headers
        .iter()
        .take(matched_column_count)
        .map(|header| header.offset_units)
        .collect::<Vec<_>>();
    let matched_cell_extent_units = first_row
        .headers
        .iter()
        .take(matched_column_count)
        .map(|header| header.extent_units)
        .collect::<Vec<_>>();
    let trailing_header_offset_units = first_row
        .headers
        .iter()
        .skip(matched_column_count)
        .map(|header| header.offset_units)
        .collect::<Vec<_>>();
    let trailing_header_extent_units = first_row
        .headers
        .iter()
        .skip(matched_column_count)
        .map(|header| header.extent_units)
        .collect::<Vec<_>>();

    output.push_str("{\"source\":\"documentTextLineHeaders\"");
    output.push_str(",\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("page-space-origin-and-unit-scale-unproven"));
    output.push_str(",\"rowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"rowsHomogeneous\":");
    output.push_str(if rows_homogeneous { "true" } else { "false" });
    output.push_str(",\"matchedColumnCount\":");
    output.push_str(&matched_column_count.to_string());
    output.push_str(",\"rawHeaderCountPerRow\":");
    output.push_str(&first_row.headers.len().to_string());
    output.push_str(",\"minOffsetUnits\":");
    push_optional_u16_json(output, min_offset_units);
    output.push_str(",\"maxExtentUnits\":");
    push_optional_u16_json(output, max_extent_units);
    output.push_str(",\"homogeneousFontSizeUnits\":");
    push_optional_u16_json(output, homogeneous_font_size_units);
    output.push_str(",\"matchedCellOffsetUnits\":");
    push_u16_array_json(output, &matched_cell_offset_units);
    output.push_str(",\"matchedCellExtentUnits\":");
    push_u16_array_json(output, &matched_cell_extent_units);
    output.push_str(",\"trailingHeaderOffsetUnits\":");
    push_u16_array_json(output, &trailing_header_offset_units);
    output.push_str(",\"trailingHeaderExtentUnits\":");
    push_u16_array_json(output, &trailing_header_extent_units);
    output.push_str(",\"tableUnitBBoxCandidates\":");
    push_table_grid_unit_bbox_candidates_json(output, rows, matched_column_count);
    output.push('}');
}

pub(crate) fn push_table_grid_unit_bbox_candidates_json(
    output: &mut String,
    rows: &[TableCandidateLineHeaderRow],
    matched_column_count: usize,
) {
    if rows.is_empty() || matched_column_count == 0 {
        output.push_str("[]");
        return;
    }

    let mut candidates = Vec::new();
    push_table_grid_unit_bbox_candidate(
        &mut candidates,
        "matched-cells",
        rows,
        matched_column_count,
        |row| {
            let start = row.headers.first()?.offset_units;
            let end = row
                .headers
                .get(matched_column_count.checked_sub(1)?)?
                .extent_units;
            Some((start, end))
        },
        false,
    );
    push_table_grid_unit_bbox_candidate(
        &mut candidates,
        "matched-cells-plus-first-trailing-header",
        rows,
        matched_column_count,
        |row| {
            let start = row.headers.first()?.offset_units;
            let end = row.headers.get(matched_column_count)?.extent_units;
            Some((start, end))
        },
        true,
    );
    push_table_grid_unit_bbox_candidate(
        &mut candidates,
        "full-line-header-extent",
        rows,
        matched_column_count,
        |row| {
            let start = row.headers.iter().map(|header| header.offset_units).min()?;
            let end = row.headers.iter().map(|header| header.extent_units).max()?;
            Some((start, end))
        },
        true,
    );

    output.push('[');
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(candidate);
    }
    output.push(']');
}

pub(crate) fn table_grid_unit_bbox_range_for_row(
    row: &TableCandidateLineHeaderRow,
    matched_column_count: usize,
    basis: TableGridUnitBBoxBasis,
) -> Option<(u16, u16)> {
    match basis {
        TableGridUnitBBoxBasis::MatchedCells => {
            let start = row.headers.first()?.offset_units;
            let end = row
                .headers
                .get(matched_column_count.checked_sub(1)?)?
                .extent_units;
            Some((start, end))
        }
        TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader => {
            let start = row.headers.first()?.offset_units;
            let end = row.headers.get(matched_column_count)?.extent_units;
            Some((start, end))
        }
        TableGridUnitBBoxBasis::FullLineHeaderExtent => {
            let start = row.headers.iter().map(|header| header.offset_units).min()?;
            let end = row.headers.iter().map(|header| header.extent_units).max()?;
            Some((start, end))
        }
    }
}

pub(crate) fn table_grid_unit_bbox_trailing_header_included(basis: TableGridUnitBBoxBasis) -> bool {
    matches!(
        basis,
        TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader
            | TableGridUnitBBoxBasis::FullLineHeaderExtent
    )
}

pub(crate) fn table_grid_unit_bbox_row_agreement_summary(
    rows: &[TableCandidateLineHeaderRow],
    matched_column_count: usize,
    basis: TableGridUnitBBoxBasis,
    selected_range: (u16, u16),
) -> (usize, bool) {
    let row_ranges = rows
        .iter()
        .filter_map(|row| {
            table_grid_unit_bbox_range_for_row(row, matched_column_count, basis)
                .filter(|(start, end)| start < end)
        })
        .collect::<Vec<_>>();
    let row_agreement_count = row_ranges
        .iter()
        .filter(|range| **range == selected_range)
        .count();
    (row_agreement_count, row_agreement_count == rows.len())
}

pub(crate) fn push_table_grid_unit_bbox_candidate<F>(
    output: &mut Vec<String>,
    basis: &'static str,
    rows: &[TableCandidateLineHeaderRow],
    matched_column_count: usize,
    mut range_for_row: F,
    trailing_header_included: bool,
) where
    F: FnMut(&TableCandidateLineHeaderRow) -> Option<(u16, u16)>,
{
    let row_ranges = rows
        .iter()
        .filter_map(|row| {
            range_for_row(row)
                .and_then(|(start, end)| (start < end).then_some((row.row_index, start, end)))
        })
        .collect::<Vec<_>>();
    if row_ranges.is_empty() {
        return;
    }

    let first_range = row_ranges.first().map(|(_, start, end)| (*start, *end));
    let row_agreement_count = row_ranges
        .iter()
        .filter(|(_, start, end)| first_range == Some((*start, *end)))
        .count();
    let all_rows_agree = row_agreement_count == rows.len();
    let (candidate_start, candidate_end) = first_range.unwrap_or((0, 0));
    let column_span_units = rows
        .first()
        .map(|row| {
            row.headers
                .iter()
                .take(matched_column_count)
                .map(|header| header.extent_units.saturating_sub(header.offset_units))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let (column_slot_width_units, trailing_slot_width_units, included_trailing_header_count) = rows
        .first()
        .map(|row| {
            table_grid_unit_bbox_slot_widths(
                row,
                matched_column_count,
                candidate_end,
                trailing_header_included,
            )
        })
        .unwrap_or_default();

    let mut item = String::new();
    item.push_str("{\"source\":\"documentTextLineHeaders\"");
    item.push_str(",\"basis\":");
    item.push_str(&json_string(basis));
    item.push_str(",\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    item.push_str(",\"xUnitRange\":");
    item.push_str(&source_range_json(
        usize::from(candidate_start),
        usize::from(candidate_end),
    ));
    item.push_str(",\"widthUnits\":");
    item.push_str(&candidate_end.saturating_sub(candidate_start).to_string());
    item.push_str(",\"rowAgreementCount\":");
    item.push_str(&row_agreement_count.to_string());
    item.push_str(",\"allRowsAgree\":");
    item.push_str(if all_rows_agree { "true" } else { "false" });
    item.push_str(",\"trailingHeaderIncluded\":");
    item.push_str(if trailing_header_included {
        "true"
    } else {
        "false"
    });
    item.push_str(",\"includedTrailingHeaderCount\":");
    item.push_str(&included_trailing_header_count.to_string());
    item.push_str(",\"columnSpanUnits\":");
    push_u16_array_json(&mut item, &column_span_units);
    item.push_str(",\"columnSlotWidthUnits\":");
    push_u16_array_json(&mut item, &column_slot_width_units);
    item.push_str(",\"trailingSlotWidthUnits\":");
    push_u16_array_json(&mut item, &trailing_slot_width_units);
    item.push_str(",\"renderPromotionContribution\":\"table-horizontal-unit-span-candidate-only\"");
    item.push_str(",\"renderPromotionBlockedReason\":");
    item.push_str(&json_string("page-space-unit-scale-unproven"));
    item.push_str(",\"rows\":[");
    for (index, (row_index, start, end)) in row_ranges.iter().enumerate() {
        if index > 0 {
            item.push(',');
        }
        item.push_str("{\"row\":");
        item.push_str(&row_index.to_string());
        item.push_str(",\"xUnitRange\":");
        item.push_str(&source_range_json(usize::from(*start), usize::from(*end)));
        item.push_str(",\"agreesWithFirstRow\":");
        item.push_str(if first_range == Some((*start, *end)) {
            "true"
        } else {
            "false"
        });
        item.push('}');
    }
    item.push_str("]}");
    output.push(item);
}

pub(crate) fn table_grid_unit_bbox_slot_widths(
    row: &TableCandidateLineHeaderRow,
    matched_column_count: usize,
    candidate_end: u16,
    trailing_header_included: bool,
) -> (Vec<u16>, Vec<u16>, usize) {
    let mut column_slot_width_units = Vec::new();
    for column_index in 0..matched_column_count {
        let Some(header) = row.headers.get(column_index) else {
            break;
        };
        let next_offset = row
            .headers
            .get(column_index + 1)
            .filter(|_| column_index + 1 < matched_column_count || trailing_header_included)
            .map(|next| next.offset_units)
            .filter(|offset| *offset <= candidate_end)
            .unwrap_or(candidate_end);
        column_slot_width_units.push(next_offset.saturating_sub(header.offset_units));
    }

    let trailing_headers = row
        .headers
        .iter()
        .skip(matched_column_count)
        .filter(|header| {
            header.offset_units < candidate_end && header.extent_units <= candidate_end
        })
        .collect::<Vec<_>>();
    let mut trailing_slot_width_units = Vec::new();
    for (index, header) in trailing_headers.iter().enumerate() {
        let next_offset = trailing_headers
            .get(index + 1)
            .map(|next| next.offset_units)
            .unwrap_or(candidate_end);
        trailing_slot_width_units.push(next_offset.saturating_sub(header.offset_units));
    }

    (
        column_slot_width_units,
        trailing_slot_width_units,
        trailing_headers.len(),
    )
}

pub(crate) fn push_table_grid_source_derived_layout_candidate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    table_candidate: &TableCandidate,
    candidate: Option<&TableGridSourceDerivedLayout>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };

    let source = match candidate.provenance {
        TableGridSourceDerivedLayoutProvenance::DecodedCompactPlacement => {
            if candidate.line_mark_page_origin.is_some()
                || candidate.page_origin_authority == "lineMarkPageGridStrideRawRecordIndex"
            {
                "documentTextLineHeaders+/LineMark+/PageMark"
            } else {
                "documentTextLineHeaders+fallbackTextAnchors"
            }
        }
        TableGridSourceDerivedLayoutProvenance::SparseSiblingDerived => {
            "sparseTableSiblingEvidence compact cell geometry candidate"
        }
    };
    output.push_str("{\"source\":");
    output.push_str(&json_string(source));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"provenance\":");
    output.push_str(&json_string(candidate.provenance.as_str()));
    output.push_str(",\"projectionKind\":\"sourceDerivedDiagnosticProjection\"");
    output.push_str(",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        candidate.x, candidate.y, candidate.width, candidate.height
    ));
    output.push_str(",\"columnCount\":");
    output.push_str(&candidate.column_count.to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.row_count.to_string());
    output.push_str(",\"columnWidth\":");
    output.push_str(&format!("{:.3}", candidate.column_width));
    output.push_str(",\"columnWidthBasis\":");
    output.push_str(&json_string(candidate.column_width_basis));
    output.push_str(",\"columnWidths\":[");
    for (index, width) in candidate.column_widths.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{width:.3}"));
    }
    output.push(']');
    output.push_str(",\"xUnitRangeBasis\":");
    output.push_str(&json_string(candidate.x_unit_range_basis));
    output.push_str(",\"xUnitRange\":");
    output.push_str(&source_range_json(
        usize::from(candidate.x_unit_start),
        usize::from(candidate.x_unit_end),
    ));
    output.push_str(",\"xOriginInsetUnits\":");
    output.push_str(&format!("{:.3}", candidate.x_origin_inset_units));
    output.push_str(",\"xOriginInsetBasis\":");
    output.push_str(&json_string(candidate.x_origin_inset_basis));
    output.push_str(",\"horizontalUnitTransformReadiness\":");
    push_table_grid_horizontal_unit_transform_readiness_json(
        output,
        layout,
        document,
        table_candidate,
        candidate,
    );
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{:.3}", candidate.row_height));
    output.push_str(",\"rowHeightBasis\":");
    output.push_str(&json_string(candidate.row_height_basis));
    output.push_str(",\"pageOriginAuthority\":");
    output.push_str(&json_string(candidate.page_origin_authority));
    output.push_str(",\"anchorLineIndex\":");
    push_optional_usize_json(output, candidate.anchor_line_index);
    output.push_str(",\"lineMarkPageOriginCandidate\":");
    push_table_grid_line_mark_page_origin_candidate_json(
        output,
        candidate.line_mark_page_origin.as_ref(),
    );
    output.push_str(",\"lineMarkPageOriginStrideCandidate\":");
    push_table_grid_line_mark_page_origin_stride_candidate_json(
        output,
        candidate.line_mark_page_origin_stride.as_ref(),
    );
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&candidate.raw_header_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&candidate.matched_cell_header_count.to_string());
    output.push_str(",\"minOffsetUnits\":");
    push_optional_u16_json(output, candidate.min_offset_units);
    output.push_str(",\"maxExtentUnits\":");
    push_optional_u16_json(output, candidate.max_extent_units);
    output.push_str(",\"matchedCellSpanUnits\":");
    push_u16_array_json(output, &candidate.matched_cell_span_units);
    output.push_str(",\"matchedCellGapUnits\":");
    push_u16_array_json(output, &candidate.matched_cell_gap_units);
    output.push_str(",\"homogeneousFontSizeUnits\":");
    push_optional_u16_json(output, candidate.homogeneous_font_size_units);
    output.push_str(",\"lineMarkRowRecordSelection\":");
    output.push_str(&json_string(candidate.line_mark_row_record_selection));
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if candidate.line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if candidate.line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(candidate.render_promotion_blocked_reason));
    output.push('}');
}

pub(crate) fn push_table_grid_horizontal_unit_transform_readiness_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    table_candidate: &TableCandidate,
    candidate: &TableGridSourceDerivedLayout,
) {
    let selected_width_units = candidate.x_unit_end.saturating_sub(candidate.x_unit_start);
    let required_cell_header_count = candidate.row_count.saturating_mul(candidate.column_count);
    let source_only_unit_transform_ready = selected_width_units > 0
        && candidate.x_unit_full_extent_units > 0
        && candidate.x_unit_all_rows_agree
        && candidate.line_header_rows_homogeneous
        && required_cell_header_count > 0
        && candidate.matched_cell_header_count >= required_cell_header_count;
    let page_space_unit_scale_decoded =
        table_grid_page_space_horizontal_transform_ready(Some(candidate));

    output.push_str("{\"source\":\"documentTextLineHeaders\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"selectedXUnitRangeBasis\":");
    output.push_str(&json_string(candidate.x_unit_range_basis));
    output.push_str(",\"selectedXUnitRange\":");
    output.push_str(&source_range_json(
        usize::from(candidate.x_unit_start),
        usize::from(candidate.x_unit_end),
    ));
    output.push_str(",\"selectedWidthUnits\":");
    output.push_str(&selected_width_units.to_string());
    output.push_str(",\"fullExtentUnits\":");
    output.push_str(&candidate.x_unit_full_extent_units.to_string());
    output.push_str(",\"selectedWidthRatioToFullExtent\":");
    if candidate.x_unit_full_extent_units > 0 {
        output.push_str(&format!(
            "{:.3}",
            f32::from(selected_width_units) / f32::from(candidate.x_unit_full_extent_units)
        ));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rowAgreementCount\":");
    output.push_str(&candidate.x_unit_row_agreement_count.to_string());
    output.push_str(",\"allRowsAgree\":");
    output.push_str(if candidate.x_unit_all_rows_agree {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"trailingHeaderIncluded\":");
    output.push_str(if candidate.x_unit_trailing_header_included {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"includedTrailingHeaderCount\":");
    output.push_str(&candidate.x_unit_included_trailing_header_count.to_string());
    output.push_str(",\"columnSpanUnits\":");
    push_u16_array_json(output, &candidate.matched_cell_span_units);
    output.push_str(",\"columnSlotWidthUnits\":");
    push_u16_array_json(output, &candidate.x_unit_column_slot_width_units);
    output.push_str(",\"trailingSlotWidthUnits\":");
    push_u16_array_json(output, &candidate.x_unit_trailing_slot_width_units);
    output.push_str(",\"xOriginInsetUnits\":");
    output.push_str(&format!("{:.3}", candidate.x_origin_inset_units));
    output.push_str(",\"xOriginInsetBasis\":");
    output.push_str(&json_string(candidate.x_origin_inset_basis));
    output.push_str(",\"totalWidthSemanticsGate\":");
    push_table_grid_total_width_semantics_gate_json(
        output,
        layout,
        document,
        table_candidate,
        candidate,
        selected_width_units,
    );
    output.push_str(",\"sourceOnlyUnitTransformReady\":");
    output.push_str(if source_only_unit_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageSpaceUnitScaleDecoded\":");
    output.push_str(if page_space_unit_scale_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(
        ",\"renderPromotionContribution\":\"selected-table-horizontal-unit-transform-readiness\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if page_space_unit_scale_decoded {
        output.push_str("null");
    } else {
        output.push_str(&json_string(if source_only_unit_transform_ready {
            "page-space-unit-scale-unproven"
        } else {
            "horizontal-unit-transform-incomplete"
        }));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_total_width_semantics_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    table_candidate: &TableCandidate,
    candidate: &TableGridSourceDerivedLayout,
    selected_width_units: u16,
) {
    let full_extent_units = candidate.x_unit_full_extent_units;
    let selected_equals_full_extent =
        selected_width_units > 0 && selected_width_units == full_extent_units;
    let selected_is_subset_of_full_extent =
        selected_width_units > 0 && full_extent_units > selected_width_units;
    let trailing_slot_evidence_present = !candidate.x_unit_trailing_slot_width_units.is_empty()
        || candidate.x_unit_included_trailing_header_count > 0;
    let full_extent_trailing_units = full_extent_units.saturating_sub(selected_width_units);
    let selected_visible_range_source_evidence_ready = selected_equals_full_extent
        || (selected_is_subset_of_full_extent
            && trailing_slot_evidence_present
            && candidate.x_unit_trailing_header_included
            && candidate.x_unit_included_trailing_header_count > 0
            && candidate.x_unit_all_rows_agree);
    let source_placement_coherence_readiness =
        table_grid_source_top_text_placement_readiness_for_candidate(
            layout,
            document,
            table_candidate,
        );
    let source_placement_coherence_gate_resolved = selected_visible_range_source_evidence_ready
        && !selected_equals_full_extent
        && source_placement_coherence_readiness
            .as_ref()
            .is_some_and(|readiness| readiness.ready);
    let source_placement_coherence_gate_required =
        selected_visible_range_source_evidence_ready && !selected_equals_full_extent;
    let render_promotion_next_gate =
        if selected_equals_full_extent || source_placement_coherence_gate_resolved {
            None
        } else if source_placement_coherence_gate_required {
            Some("source-table-placement-coherence-gate")
        } else {
            Some("source-total-width-semantics-decoder")
        };

    output.push_str("{\"source\":\"documentTextLineHeaders total-width semantics gate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"selectedWidthUnits\":");
    output.push_str(&selected_width_units.to_string());
    output.push_str(",\"fullExtentUnits\":");
    output.push_str(&full_extent_units.to_string());
    output.push_str(",\"fullExtentTrailingUnits\":");
    output.push_str(&full_extent_trailing_units.to_string());
    output.push_str(",\"selectedEqualsFullExtent\":");
    output.push_str(if selected_equals_full_extent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedIsSubsetOfFullExtent\":");
    output.push_str(if selected_is_subset_of_full_extent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"trailingHeaderIncluded\":");
    output.push_str(if candidate.x_unit_trailing_header_included {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"includedTrailingHeaderCount\":");
    output.push_str(&candidate.x_unit_included_trailing_header_count.to_string());
    output.push_str(",\"trailingSlotEvidencePresent\":");
    output.push_str(if trailing_slot_evidence_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"trailingSlotWidthUnits\":");
    push_u16_array_json(output, &candidate.x_unit_trailing_slot_width_units);
    output.push_str(",\"selectedVisibleRangeSourceEvidenceReady\":");
    output.push_str(if selected_visible_range_source_evidence_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementCoherenceGateRequired\":");
    output.push_str(
        if source_placement_coherence_gate_required && !source_placement_coherence_gate_resolved {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sourcePlacementCoherenceGateEvidencePresent\":");
    output.push_str(if source_placement_coherence_readiness.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementCoherenceGateResolved\":");
    output.push_str(if source_placement_coherence_gate_resolved {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementCoherenceGateBlockedReasons\":");
    match source_placement_coherence_readiness.as_ref() {
        Some(readiness) => push_json_string_slice_array(output, &readiness.blocked_reasons),
        None if source_placement_coherence_gate_required => push_json_string_slice_array(
            output,
            &["source-table-placement-coherence-evidence-absent"],
        ),
        None => push_json_string_slice_array(output, &[]),
    }
    output.push_str(",\"renderPromotionNextGate\":");
    match render_promotion_next_gate {
        Some(gate) => output.push_str(&json_string(gate)),
        None => output.push_str("null"),
    }
    output.push_str(",\"renderWidthBasisCandidate\":");
    if selected_equals_full_extent {
        output.push_str(&json_string("selected-range-equals-full-extent"));
    } else if selected_is_subset_of_full_extent && trailing_slot_evidence_present {
        output.push_str(&json_string(
            "selected-visible-range-with-trailing-header-evidence",
        ));
    } else if selected_is_subset_of_full_extent {
        output.push_str(&json_string("selected-visible-range-subset-of-full-extent"));
    } else {
        output.push_str(&json_string("total-width-semantics-undetermined"));
    }
    output.push_str(",\"renderPromotionContribution\":\"source-total-width-semantics-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if selected_equals_full_extent || source_placement_coherence_gate_resolved {
        output.push_str("null");
    } else if source_placement_coherence_gate_required {
        output.push_str(&json_string(
            "source-table-placement-coherence-gate-required",
        ));
    } else {
        output.push_str(&json_string("source-total-width-semantics-unproven"));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_source_derived_layout_readiness_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    let source_placement_evidence_present =
        table_grid_decoded_source_placement_evidence_present(document, candidate);
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let row_count = candidate.intervals().len();
    let raw_header_count = rows
        .iter()
        .map(TableCandidateLineHeaderRow::raw_header_count)
        .sum::<usize>();
    let matched_row_count = rows.iter().filter(|row| row.matched_cell_count > 0).count();
    let full_matched_row_count = rows
        .iter()
        .filter(|row| row.matched_cell_count >= row.expected_cell_count)
        .count();
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    let required_cell_header_count = rows
        .iter()
        .map(|row| row.expected_cell_count)
        .sum::<usize>();
    let common_matched_column_count = rows
        .iter()
        .map(|row| row.matched_cell_count)
        .min()
        .unwrap_or(0)
        .min(column_count);
    let rows_without_headers = rows
        .iter()
        .filter(|row| row.headers.is_empty())
        .map(|row| row.row_index)
        .collect::<Vec<_>>();
    let rows_without_matched_cell_headers = rows
        .iter()
        .filter(|row| row.matched_cell_count == 0)
        .map(|row| row.row_index)
        .collect::<Vec<_>>();
    let rows_with_partial_cell_header_coverage = rows
        .iter()
        .filter(|row| {
            row.matched_cell_count > 0 && row.matched_cell_count < row.expected_cell_count
        })
        .map(|row| row.row_index)
        .collect::<Vec<_>>();
    let line_header_rows_homogeneous = table_grid_line_header_rows_are_homogeneous(&rows);
    let resolved_line_mark_rows =
        table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows);
    let line_mark_row_record_selection =
        table_grid_line_mark_row_record_selection(&resolved_line_mark_rows);
    let line_mark_rows_exact_and_contiguous =
        table_grid_line_mark_rows_are_exact_and_contiguous(document, candidate, &rows);
    let source_layout_present = source_layout.is_some();
    let source_layout_renderable = source_layout
        .as_ref()
        .is_some_and(|layout| table_grid_source_derived_layout_is_renderable(layout));

    let mut rejection_reasons = Vec::new();
    if column_count == 0 {
        rejection_reasons.push("column-count-zero");
    }
    if !source_placement_evidence_present {
        rejection_reasons.push("source-placement-evidence-missing");
    }
    if rows.is_empty() {
        rejection_reasons.push("line-header-rows-missing");
    }
    if raw_header_count == 0 {
        rejection_reasons.push("line-header-raw-headers-missing");
    }
    if matched_cell_header_count < required_cell_header_count {
        rejection_reasons.push("line-header-cell-geometry-incomplete");
    }
    if common_matched_column_count == 0 {
        rejection_reasons.push("no-common-matched-cell-header-columns");
    }
    if !line_header_rows_homogeneous {
        rejection_reasons.push("line-header-rows-not-homogeneous");
    }
    if !line_mark_rows_exact_and_contiguous {
        rejection_reasons.push("line-mark-rows-not-exact-source-boundaries");
    }
    if !source_layout_present {
        rejection_reasons.push("source-derived-layout-candidate-absent");
    }
    if let Some(layout) = source_layout
        && layout.render_promotion_blocked_reason != "none"
    {
        rejection_reasons.push(layout.render_promotion_blocked_reason);
    }
    if source_layout_present && !source_layout_renderable {
        rejection_reasons.push("source-derived-layout-not-renderable");
    }

    output.push_str("{\"source\":\"sourceDerivedLayoutGate+documentTextLineHeaders+/LineMark\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourcePlacementEvidencePresent\":");
    output.push_str(if source_placement_evidence_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"requestedColumnCount\":");
    output.push_str(&column_count.to_string());
    output.push_str(",\"lineHeaderRowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&raw_header_count.to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&matched_row_count.to_string());
    output.push_str(",\"fullMatchedRowCount\":");
    output.push_str(&full_matched_row_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&matched_cell_header_count.to_string());
    output.push_str(",\"requiredCellHeaderCount\":");
    output.push_str(&required_cell_header_count.to_string());
    output.push_str(",\"commonMatchedColumnCount\":");
    output.push_str(&common_matched_column_count.to_string());
    output.push_str(",\"rowsWithoutHeaders\":");
    push_usize_array_json(output, &rows_without_headers);
    output.push_str(",\"rowsWithoutMatchedCellHeaders\":");
    push_usize_array_json(output, &rows_without_matched_cell_headers);
    output.push_str(",\"rowsWithPartialCellHeaderCoverage\":");
    push_usize_array_json(output, &rows_with_partial_cell_header_coverage);
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRowRecordSelection\":");
    output.push_str(&json_string(line_mark_row_record_selection));
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutCandidatePresent\":");
    output.push_str(if source_layout_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutRenderable\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutBlockedReason\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.render_promotion_blocked_reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"rejectionReasons\":");
    push_json_string_slice_array(output, &rejection_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-derived-layout-readiness-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if source_layout_renderable {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-derived-layout-not-renderable"));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_page_space_solver_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    column_count: usize,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    let source_placement_evidence_present =
        table_grid_decoded_source_placement_evidence_present(document, candidate);
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    let required_cell_header_count = rows
        .iter()
        .map(|row| row.expected_cell_count)
        .sum::<usize>();
    let common_matched_column_count = rows
        .iter()
        .map(|row| row.matched_cell_count)
        .min()
        .unwrap_or(0)
        .min(column_count);
    let line_header_rows_homogeneous = table_grid_line_header_rows_are_homogeneous(&rows);
    let resolved_line_mark_rows =
        table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows);
    let line_mark_row_record_selection =
        table_grid_line_mark_row_record_selection(&resolved_line_mark_rows);
    let line_mark_rows_exact_and_contiguous =
        table_grid_line_mark_rows_are_exact_and_contiguous(document, candidate, &rows);
    let source_layout_renderable = source_layout
        .as_ref()
        .is_some_and(|layout| table_grid_source_derived_layout_is_renderable(layout));
    let source_column_split_ready = table_grid_source_column_split_ready(source_layout);
    let page_space_horizontal_transform_ready =
        table_grid_page_space_horizontal_transform_ready(source_layout);
    let subrecord_span_readiness =
        table_grid_page_mark_subrecord_line_span_readiness(document, candidate);
    let horizontal_solver_ready = source_placement_evidence_present
        && source_layout.is_some()
        && column_count > 0
        && common_matched_column_count > 0
        && required_cell_header_count > 0
        && matched_cell_header_count >= required_cell_header_count;
    let row_height_solver_ready = source_layout
        .as_ref()
        .is_some_and(|layout| layout.homogeneous_font_size_units.is_some())
        && line_header_rows_homogeneous;
    let y_origin_solver_ready = source_layout.as_ref().is_some_and(|layout| {
        layout.line_mark_page_origin.is_some()
            && layout.page_origin_authority == "lineMarkPageGrid"
            && layout.line_mark_rows_exact_and_contiguous
            && layout.line_header_rows_homogeneous
            && layout.render_promotion_blocked_reason == "none"
    });
    let solver_stage = if source_layout_renderable {
        "renderable-source-page-space"
    } else if horizontal_solver_ready && row_height_solver_ready && !y_origin_solver_ready {
        "blocked-y-origin-transform"
    } else if horizontal_solver_ready && !row_height_solver_ready {
        "blocked-row-height-transform"
    } else if horizontal_solver_ready {
        "blocked-partial-page-space"
    } else if source_layout.is_some() {
        "blocked-horizontal-transform"
    } else {
        "candidate-absent"
    };
    let blocked_reason = if source_layout_renderable {
        None
    } else if let Some(layout) = source_layout {
        if layout.render_promotion_blocked_reason != "none" {
            Some(layout.render_promotion_blocked_reason)
        } else if !horizontal_solver_ready {
            Some("table-horizontal-source-transform-incomplete")
        } else if !row_height_solver_ready {
            Some("table-row-height-source-transform-incomplete")
        } else if !y_origin_solver_ready {
            Some("table-page-y-transform-unproven")
        } else {
            Some("source-derived-layout-not-renderable")
        }
    } else {
        Some("source-derived-layout-candidate-absent")
    };

    output.push_str("{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\"");
    output.push_str(",\"solverVersion\":\"table-page-space-v1\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"solverStage\":");
    output.push_str(&json_string(solver_stage));
    output.push_str(",\"sourcePlacementEvidencePresent\":");
    output.push_str(if source_placement_evidence_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"requestedColumnCount\":");
    output.push_str(&column_count.to_string());
    output.push_str(",\"commonMatchedColumnCount\":");
    output.push_str(&common_matched_column_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&matched_cell_header_count.to_string());
    output.push_str(",\"requiredCellHeaderCount\":");
    output.push_str(&required_cell_header_count.to_string());
    output.push_str(",\"horizontalSolverReady\":");
    output.push_str(if horizontal_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowHeightSolverReady\":");
    output.push_str(if row_height_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"yOriginSolverReady\":");
    output.push_str(if y_origin_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRowRecordSelection\":");
    output.push_str(&json_string(line_mark_row_record_selection));
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutCandidatePresent\":");
    output.push_str(if source_layout.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutRenderable\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageOriginAuthority\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.page_origin_authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(
        if source_layout
            .as_ref()
            .is_some_and(|layout| layout.line_mark_page_origin.is_some())
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"lineMarkPageOriginStridePresent\":");
    output.push_str(
        if source_layout
            .as_ref()
            .is_some_and(|layout| layout.line_mark_page_origin_stride.is_some())
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"referenceCalibrationReplacementGate\":");
    push_table_grid_reference_calibration_replacement_gate_json(
        output,
        source_layout,
        source_layout_renderable,
        horizontal_solver_ready,
        source_column_split_ready,
        page_space_horizontal_transform_ready,
        row_height_solver_ready,
        y_origin_solver_ready,
    );
    output.push_str(",\"sourceOnlyAxisAdmissionGate\":");
    push_table_grid_source_only_axis_admission_gate_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
        source_layout_renderable,
        source_column_split_ready,
        page_space_horizontal_transform_ready,
        row_height_solver_ready,
        y_origin_solver_ready,
        subrecord_span_readiness.as_ref(),
    );
    output.push_str(",\"pageSpaceHorizontalTransformGate\":");
    push_table_grid_page_space_horizontal_transform_gate_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
        source_column_split_ready,
        page_space_horizontal_transform_ready,
    );
    output.push_str(",\"sourcePageYTransformGate\":");
    push_table_grid_source_page_y_transform_gate_json(
        output,
        document,
        candidate,
        source_layout,
        subrecord_span_readiness.as_ref(),
        y_origin_solver_ready,
        line_mark_rows_exact_and_contiguous,
    );
    output.push_str(",\"renderPromoted\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionAuthority\":");
    if source_layout_renderable {
        output.push_str(&json_string("source-derived-page-space-solver"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    if let Some(reason) = blocked_reason {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_source_only_axis_admission_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    source_layout_renderable: bool,
    source_column_split_ready: bool,
    page_space_horizontal_transform_ready: bool,
    row_height_solver_ready: bool,
    y_origin_solver_ready: bool,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let horizontal_supports = table_grid_page_space_horizontal_frame_candidate_supports(
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    let mut horizontal_groups: BTreeMap<(i32, i32), Vec<TableGridHorizontalFrameCandidateSupport>> =
        BTreeMap::new();
    for support in horizontal_supports.iter().cloned() {
        horizontal_groups
            .entry((
                rounded_milli(support.selected_x),
                rounded_milli(support.selected_width),
            ))
            .or_default()
            .push(support);
    }
    let horizontal_best_support_count = horizontal_groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let horizontal_best_group_count = horizontal_groups
        .values()
        .filter(|supports| supports.len() == horizontal_best_support_count)
        .count();
    let horizontal_unique_best_supported =
        horizontal_best_support_count > 1 && horizontal_best_group_count == 1;
    let horizontal_best_group = horizontal_groups.values().find(|supports| {
        supports.len() == horizontal_best_support_count && horizontal_unique_best_supported
    });
    let horizontal_selector_candidate_present = horizontal_supports
        .iter()
        .any(|support| support.contribution == "source-only-horizontal-field-selector");
    let horizontal_selector_in_best_group = horizontal_best_group.is_some_and(|supports| {
        supports
            .iter()
            .any(|support| support.contribution == "source-only-horizontal-field-selector")
    });
    let horizontal_best_support = horizontal_best_group.and_then(|supports| supports.first());

    let cross_table_row_boundary_offset_probe =
        table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let y_supports = table_grid_source_only_page_y_origin_candidate_supports(
        document,
        candidate,
        source_layout,
        cross_table_row_boundary_offset_probe.as_ref(),
        subrecord_span_readiness,
    );
    let mut y_groups: BTreeMap<
        (i32, Option<i32>),
        Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    > = BTreeMap::new();
    for support in y_supports.iter().cloned() {
        y_groups
            .entry((
                rounded_milli(support.selected_y),
                support.row_height.map(rounded_milli),
            ))
            .or_default()
            .push(support);
    }
    let y_best_support_count = y_groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let y_best_group_count = y_groups
        .values()
        .filter(|supports| supports.len() == y_best_support_count)
        .count();
    let y_unique_best_supported = y_best_support_count > 1 && y_best_group_count == 1;
    let y_best_group = y_groups
        .values()
        .find(|supports| supports.len() == y_best_support_count && y_unique_best_supported);
    let y_candidate_best_group = y_best_group.filter(|supports| {
        table_grid_source_only_page_y_origin_group_supports_candidate(supports, candidate)
    });
    let y_fallback_selector_group = if y_candidate_best_group.is_none() {
        table_grid_source_only_page_y_origin_fallback_selector_group(&y_groups, candidate)
    } else {
        None
    };
    let y_selector_group = y_candidate_best_group.or(y_fallback_selector_group.as_ref());
    let y_selector_support = y_selector_group.and_then(|supports| supports.first());
    let y_selector_uses_single_support_fallback =
        y_candidate_best_group.is_none() && y_fallback_selector_group.is_some();
    let y_selector_table_candidate_indexes = y_selector_group
        .map(|supports| {
            supports
                .iter()
                .filter_map(|support| support.table_candidate_index)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let y_cross_table_previous_row_span_table_candidate_count = y_groups
        .values()
        .flatten()
        .filter(|support| {
            table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(support)
        })
        .filter_map(|support| support.table_candidate_index)
        .collect::<BTreeSet<_>>()
        .len();
    let y_selector_support_fragmented_by_table = !y_selector_uses_single_support_fallback
        && y_cross_table_previous_row_span_table_candidate_count > 1
        && y_selector_table_candidate_indexes.len()
            < y_cross_table_previous_row_span_table_candidate_count;
    let y_selector_candidate_present = y_selector_group.is_some();
    let y_selector_support_count = y_selector_group.map(|supports| supports.len()).unwrap_or(0);
    let y_selector_support_blocked_reasons = y_selector_group
        .map(|supports| table_grid_source_only_page_y_origin_supports_blocked_reasons(supports))
        .unwrap_or_default();
    let page_mark_absolute_y_slot_agreement =
        table_grid_source_only_page_mark_absolute_y_slot_agreement(
            document,
            candidate,
            cross_table_row_boundary_offset_probe.as_ref(),
            subrecord_span_readiness,
        );
    let page_mark_absolute_y_slot_semantics_ready =
        page_mark_absolute_y_slot_agreement.semantics_ready();
    let page_mark_absolute_y_slot_blocked_reason =
        table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(
            &page_mark_absolute_y_slot_agreement,
        );
    let y_selector_cross_table_support_present = y_selector_group.is_some_and(|supports| {
        supports
            .iter()
            .any(table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span)
    });
    let y_selector_agreement_admissible = y_selector_candidate_present
        && y_unique_best_supported
        && !y_selector_uses_single_support_fallback
        && !y_selector_support_fragmented_by_table
        && y_selector_cross_table_support_present;
    let active_source_layout_admission_ready =
        source_layout_renderable && page_space_horizontal_transform_ready && y_origin_solver_ready;
    let y_selector_admission_blocked_reason = if active_source_layout_admission_ready {
        "none"
    } else if !y_selector_candidate_present {
        "source-y-origin-selector-absent"
    } else if y_selector_uses_single_support_fallback {
        "source-y-origin-selector-single-support-fallback-not-render-admissible"
    } else if y_selector_support_fragmented_by_table {
        "source-y-origin-selector-fragmented-by-table-not-render-admissible"
    } else if !y_unique_best_supported {
        "source-y-origin-selector-agreement-unproven"
    } else if !y_selector_cross_table_support_present {
        "source-y-origin-selector-cross-table-support-absent"
    } else if !y_origin_solver_ready {
        "source-y-axis-not-render-admissible"
    } else {
        "none"
    };
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe.as_ref(),
        );
    let source_gap_to_page_line_gap_table_family_transform_required =
        y_selector_cross_table_support_present
            || cross_table_row_boundary_offset_probe
                .as_ref()
                .is_some_and(|probe| probe.all_offsets_require_transform);
    let axis_candidate_row_height = y_selector_support
        .and_then(|support| support.row_height)
        .or_else(|| source_layout.map(|layout| layout.row_height));
    let axis_candidate_present = horizontal_best_support.is_some() && y_selector_support.is_some();
    let axis_candidate_bbox_present = axis_candidate_present && axis_candidate_row_height.is_some();

    let mut blocked_reasons = Vec::new();
    if source_layout.is_none() {
        blocked_reasons.push("source-derived-layout-candidate-absent");
    }
    if !source_column_split_ready {
        blocked_reasons.push("source-column-split-not-ready");
    }
    if !row_height_solver_ready {
        blocked_reasons.push("source-row-height-not-ready");
    }
    if !active_source_layout_admission_ready {
        if horizontal_groups.is_empty() {
            blocked_reasons.push("source-horizontal-frame-candidates-absent");
        } else if !horizontal_unique_best_supported {
            blocked_reasons.push("source-horizontal-frame-candidate-agreement-unproven");
        }
        if !horizontal_selector_candidate_present {
            blocked_reasons.push("source-only-horizontal-selector-absent");
        } else if !horizontal_selector_in_best_group {
            blocked_reasons.push("source-only-horizontal-selector-not-in-best-agreement-group");
        }
        if !page_space_horizontal_transform_ready {
            blocked_reasons.push("source-horizontal-axis-not-render-admissible");
        }
        if y_groups.is_empty() {
            blocked_reasons.push("source-y-origin-candidates-absent");
        } else if !y_unique_best_supported && !y_selector_uses_single_support_fallback {
            blocked_reasons.push("source-y-origin-candidate-agreement-unproven");
        }
        if !y_selector_candidate_present {
            blocked_reasons.push("source-y-origin-selector-absent");
        }
        if y_selector_uses_single_support_fallback {
            blocked_reasons.push("source-y-origin-selector-single-support-fallback");
        }
        if y_selector_support_fragmented_by_table {
            blocked_reasons.push("source-y-origin-selector-fragmented-by-table");
        }
    }
    if source_gap_to_page_line_gap_table_family_transform_required
        && let Some(reason) =
            source_gap_to_page_line_gap_readiness_hints.table_family_transform_blocked_reason()
    {
        blocked_reasons.push(reason);
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("source-y-axis-not-render-admissible");
    }
    if !source_layout_renderable {
        blocked_reasons.push("source-derived-layout-not-renderable");
    }
    let admission_ready = active_source_layout_admission_ready || blocked_reasons.is_empty();

    output.push_str("{\"source\":\"pageSpaceHorizontalTransformGate+sourcePageYTransformGate source-only selector coupling\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsedForSelection\":false");
    output.push_str(",\"admissionReady\":");
    output.push_str(if admission_ready { "true" } else { "false" });
    output.push_str(",\"activeSourceLayoutAdmissionReady\":");
    output.push_str(if active_source_layout_admission_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"activeSourceLayoutAdmissionBasis\":");
    if active_source_layout_admission_ready {
        output.push_str(&json_string("source-derived-page-space-solver"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceOnlySelectorFallbackIgnoredByActiveSourceLayout\":");
    output.push_str(
        if active_source_layout_admission_ready
            && (!horizontal_selector_candidate_present || y_selector_uses_single_support_fallback)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutRenderable\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalAxisReady\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalSelectorCandidatePresent\":");
    output.push_str(if horizontal_selector_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalSelectorInBestAgreementGroup\":");
    output.push_str(if horizontal_selector_in_best_group {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalCandidateCount\":");
    output.push_str(&horizontal_supports.len().to_string());
    output.push_str(",\"horizontalAgreementGroupCount\":");
    output.push_str(&horizontal_groups.len().to_string());
    output.push_str(",\"horizontalBestSupportCount\":");
    output.push_str(&horizontal_best_support_count.to_string());
    output.push_str(",\"horizontalUniqueBestSupported\":");
    output.push_str(if horizontal_unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalBestSupportedSelectedX\":");
    match horizontal_best_support {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_x)),
        None => output.push_str("null"),
    }
    output.push_str(",\"horizontalBestSupportedSelectedWidth\":");
    match horizontal_best_support {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_width)),
        None => output.push_str("null"),
    }
    output.push_str(",\"horizontalBestSupportedFrameBases\":");
    match horizontal_best_group {
        Some(supports) => {
            let frame_bases = supports
                .iter()
                .map(|support| support.frame_basis)
                .collect::<Vec<_>>();
            push_json_string_slice_array(output, &frame_bases);
        }
        None => output.push_str("[]"),
    }
    output.push_str(",\"yAxisReady\":");
    output.push_str(if y_origin_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorCandidatePresent\":");
    output.push_str(if y_selector_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorSingleSupportFallback\":");
    output.push_str(if y_selector_uses_single_support_fallback {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorSupportFragmentedByTable\":");
    output.push_str(if y_selector_support_fragmented_by_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorSupportCount\":");
    output.push_str(&y_selector_support_count.to_string());
    output.push_str(",\"ySelectorCrossTableSupportPresent\":");
    output.push_str(if y_selector_cross_table_support_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorAgreementAdmissible\":");
    output.push_str(if y_selector_agreement_admissible {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorAdmissionBlockedReason\":");
    output.push_str(&json_string(y_selector_admission_blocked_reason));
    output.push_str(",\"ySelectorSupportBlockedReasons\":");
    push_json_string_slice_array(output, &y_selector_support_blocked_reasons);
    output.push_str(",\"sourceGapToPageLineGapTransformAdmissionGate\":");
    push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
        output,
        "sourceOnlyAxisAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate",
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"pageMarkAbsoluteYSlotSemanticsReady\":");
    output.push_str(if page_mark_absolute_y_slot_semantics_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageMarkAbsoluteYSlotBlockedReason\":");
    output.push_str(&json_string(page_mark_absolute_y_slot_blocked_reason));
    output.push_str(",\"pageMarkAbsoluteYSlotResidualPx\":");
    push_optional_f32_json(output, page_mark_absolute_y_slot_agreement.residual_px);
    output.push_str(",\"yCandidateCount\":");
    output.push_str(&y_supports.len().to_string());
    output.push_str(",\"yAgreementGroupCount\":");
    output.push_str(&y_groups.len().to_string());
    output.push_str(",\"yBestSupportCount\":");
    output.push_str(&y_best_support_count.to_string());
    output.push_str(",\"yUniqueBestSupported\":");
    output.push_str(if y_unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectedOriginBasis\":");
    match y_selector_support {
        Some(support) => output.push_str(&json_string(support.origin_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"ySelectedY\":");
    match y_selector_support {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_y)),
        None => output.push_str("null"),
    }
    output.push_str(",\"ySelectedRowHeight\":");
    match y_selector_support {
        Some(support) => push_optional_f32_json(output, support.row_height),
        None => output.push_str("null"),
    }
    output.push_str(",\"ySelectorTableCandidateIndexes\":");
    push_usize_array_json(output, &y_selector_table_candidate_indexes);
    output.push_str(",\"sourceOnlyAxisCandidateBBox\":");
    push_table_grid_source_only_axis_candidate_bbox_json(
        output,
        candidate,
        horizontal_best_support,
        y_selector_support,
        axis_candidate_row_height,
        axis_candidate_present,
        axis_candidate_bbox_present,
        admission_ready,
        active_source_layout_admission_ready,
    );
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"source-only-axis-selector-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if admission_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "source-page-space-axis-selector-coupling-unproven",
        ));
    }
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_source_only_axis_candidate_bbox_json(
    output: &mut String,
    candidate: &TableCandidate,
    horizontal_support: Option<&TableGridHorizontalFrameCandidateSupport>,
    y_support: Option<&TableGridSourceOnlyPageYOriginCandidateSupport>,
    row_height: Option<f32>,
    candidate_present: bool,
    bbox_present: bool,
    admission_ready: bool,
    active_source_layout_admission_ready: bool,
) {
    let render_promotion_blocked_reason = if active_source_layout_admission_ready {
        "active-source-layout-admission-uses-source-derived-layout-not-selector-bbox"
    } else if horizontal_support.is_none() {
        "source-only-axis-horizontal-candidate-absent"
    } else if y_support.is_none() {
        "source-only-axis-y-candidate-absent"
    } else if row_height.is_none() {
        "source-only-axis-row-height-candidate-absent"
    } else if !admission_ready {
        "source-page-space-axis-selector-coupling-unproven"
    } else {
        "source-only-axis-candidate-bbox-diagnostic-only"
    };

    output.push_str("{\"source\":\"sourceOnlyAxisAdmissionGate.sourceOnlyAxisCandidateBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"candidatePresent\":");
    output.push_str(if candidate_present { "true" } else { "false" });
    output.push_str(",\"bboxPresent\":");
    output.push_str(if bbox_present { "true" } else { "false" });
    output.push_str(",\"horizontalCandidatePresent\":");
    output.push_str(if horizontal_support.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"yCandidatePresent\":");
    output.push_str(if y_support.is_some() { "true" } else { "false" });
    output.push_str(",\"rowHeightCandidatePresent\":");
    output.push_str(if row_height.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"horizontalFrameBasis\":");
    match horizontal_support {
        Some(support) => output.push_str(&json_string(support.frame_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"yOriginBasis\":");
    match y_support {
        Some(support) => output.push_str(&json_string(support.origin_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"rowHeight\":");
    push_optional_f32_json(output, row_height);
    output.push_str(",\"bbox\":");
    if let (Some(horizontal), Some(y), Some(row_height)) =
        (horizontal_support, y_support, row_height)
    {
        let height = row_height * candidate.intervals().len() as f32;
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
            horizontal.selected_x, y.selected_y, horizontal.selected_width, height
        ));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderPromotionContribution\":\"source-only-axis-candidate-bbox\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push('}');
}

pub(crate) fn table_grid_source_column_split_ready(
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> bool {
    source_layout.is_some_and(|layout| {
        layout.column_count > 0
            && layout.column_width_basis == "documentTextLineHeaderCellSlotUnits"
            && layout.column_widths.len() == layout.column_count
            && layout.column_widths.iter().all(|width| *width > 0.0)
    })
}

pub(crate) fn table_grid_page_space_horizontal_transform_ready(
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> bool {
    source_layout.is_some_and(|layout| {
        table_grid_source_column_split_ready(Some(layout))
            && layout.x_unit_all_rows_agree
            && layout.x_unit_full_extent_units > 0
            && layout.page_origin_authority == "lineMarkPageGrid"
            && layout.line_mark_rows_exact_and_contiguous
            && layout.render_promotion_blocked_reason == "none"
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_page_space_horizontal_transform_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    source_column_split_ready: bool,
    page_space_horizontal_transform_ready: bool,
) {
    let source_layout_present = source_layout.is_some();
    let x_unit_all_rows_agree = source_layout
        .as_ref()
        .is_some_and(|layout| layout.x_unit_all_rows_agree);
    let full_extent_units_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.x_unit_full_extent_units > 0);
    let page_origin_authority = source_layout
        .as_ref()
        .map(|layout| layout.page_origin_authority);
    let line_mark_rows_exact_and_contiguous = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_rows_exact_and_contiguous);
    let source_layout_render_blocked_reason = source_layout
        .as_ref()
        .map(|layout| layout.render_promotion_blocked_reason);
    let source_frame_decoded = page_space_horizontal_transform_ready;

    let mut blocked_reasons = Vec::new();
    if !source_layout_present {
        blocked_reasons.push("source-derived-layout-candidate-absent");
    }
    if !source_column_split_ready {
        blocked_reasons.push("source-column-split-not-ready");
    }
    if !x_unit_all_rows_agree {
        blocked_reasons.push("source-x-unit-range-not-row-stable");
    }
    if !full_extent_units_present {
        blocked_reasons.push("source-full-line-extent-units-missing");
    }
    if !source_frame_decoded {
        blocked_reasons.push("page-space-horizontal-frame-not-decoded");
    }
    if !line_mark_rows_exact_and_contiguous {
        blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
    }
    if let Some(reason) = source_layout_render_blocked_reason
        && reason != "none"
    {
        blocked_reasons.push(reason);
    }

    output.push_str(
        "{\"source\":\"documentTextLineHeaders+/LineMark page-space horizontal transform gate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceColumnSplitReady\":");
    output.push_str(if source_column_split_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"xUnitAllRowsAgree\":");
    output.push_str(if x_unit_all_rows_agree {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fullExtentUnitsPresent\":");
    output.push_str(if full_extent_units_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceFrameDecoded\":");
    output.push_str(if source_frame_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageOriginAuthority\":");
    match page_origin_authority {
        Some(authority) => output.push_str(&json_string(authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutBlockedReason\":");
    match source_layout_render_blocked_reason {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"xUnitRangeBasis\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.x_unit_range_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"xUnitRange\":");
    match source_layout {
        Some(layout) => output.push_str(&source_range_json(
            usize::from(layout.x_unit_start),
            usize::from(layout.x_unit_end),
        )),
        None => output.push_str("null"),
    }
    output.push_str(",\"fullExtentUnits\":");
    match source_layout {
        Some(layout) => output.push_str(&layout.x_unit_full_extent_units.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"xOriginInsetBasis\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.x_origin_inset_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceFrameAdmissionGate\":");
    push_table_grid_page_space_horizontal_source_frame_admission_gate_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
        page_space_horizontal_transform_ready,
    );
    output.push_str(",\"sourceFrameHypotheses\":");
    push_table_grid_page_space_horizontal_frame_hypotheses_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    output.push_str(",\"sourceOnlyHorizontalFieldConsensus\":");
    push_table_grid_source_only_horizontal_field_consensus_json(
        output, layout, document, lines, candidate,
    );
    output.push_str(",\"sourceFrameCandidateAgreementGate\":");
    push_table_grid_page_space_horizontal_frame_candidate_agreement_gate_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-page-space-horizontal-transform-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if page_space_horizontal_transform_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "table-horizontal-page-space-transform-incomplete",
        ));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_page_space_horizontal_source_frame_admission_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    page_space_horizontal_transform_ready: bool,
) {
    output.push_str("{\"source\":\"sourceDerivedLayoutCandidate+sourceFrameAdmission\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceFrameDecoded\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderFrameBasis\":");
    if page_space_horizontal_transform_ready {
        output.push_str(&json_string(
            "page-body-frame+documentTextLineHeaderUnitTransform",
        ));
    } else {
        output.push_str("null");
    }

    match source_layout {
        Some(source_layout) => {
            let selected_start_units = f32::from(source_layout.x_unit_start);
            let selected_width_units = f32::from(
                source_layout
                    .x_unit_end
                    .saturating_sub(source_layout.x_unit_start),
            );
            let full_extent_units = f32::from(source_layout.x_unit_full_extent_units);
            let body_unit_px = if full_extent_units > 0.0 {
                Some(layout.body_width_px() / full_extent_units)
            } else {
                None
            };
            let selected_x_without_inset =
                body_unit_px.map(|unit_px| layout.margin_px() + selected_start_units * unit_px);
            let selected_x_with_inset = body_unit_px.map(|unit_px| {
                layout.margin_px()
                    + (selected_start_units + source_layout.x_origin_inset_units) * unit_px
            });
            output.push_str(",\"selectedX\":");
            output.push_str(&format!("{:.3}", source_layout.x));
            output.push_str(",\"selectedWidth\":");
            output.push_str(&format!("{:.3}", source_layout.width));
            output.push_str(",\"selectedStartUnits\":");
            output.push_str(&format!("{selected_start_units:.3}"));
            output.push_str(",\"selectedWidthUnits\":");
            output.push_str(&format!("{selected_width_units:.3}"));
            output.push_str(",\"fullExtentUnits\":");
            output.push_str(&format!("{full_extent_units:.3}"));
            output.push_str(",\"xOriginInsetUnits\":");
            output.push_str(&format!("{:.3}", source_layout.x_origin_inset_units));
            output.push_str(",\"xOriginInsetBasis\":");
            output.push_str(&json_string(source_layout.x_origin_inset_basis));
            output.push_str(",\"xOriginInsetApplied\":");
            output.push_str(if source_layout.x_origin_inset_units != 0.0 {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"pageBodyFrameX\":");
            output.push_str(&format!("{:.3}", layout.margin_px()));
            output.push_str(",\"pageBodyFrameWidth\":");
            output.push_str(&format!("{:.3}", layout.body_width_px()));
            output.push_str(",\"pageBodyUnitPx\":");
            push_optional_f32_json(output, body_unit_px);
            output.push_str(",\"selectedXWithoutInset\":");
            push_optional_f32_json(output, selected_x_without_inset);
            output.push_str(",\"selectedXWithInset\":");
            push_optional_f32_json(output, selected_x_with_inset);
        }
        None => {
            output.push_str(",\"selectedX\":null,\"selectedWidth\":null");
            output.push_str(",\"selectedStartUnits\":null,\"selectedWidthUnits\":null");
            output.push_str(",\"fullExtentUnits\":null,\"xOriginInsetUnits\":null");
            output.push_str(",\"xOriginInsetBasis\":null,\"xOriginInsetApplied\":false");
            output.push_str(",\"pageBodyFrameX\":");
            output.push_str(&format!("{:.3}", layout.margin_px()));
            output.push_str(",\"pageBodyFrameWidth\":");
            output.push_str(&format!("{:.3}", layout.body_width_px()));
            output.push_str(
                ",\"pageBodyUnitPx\":null,\"selectedXWithoutInset\":null,\"selectedXWithInset\":null",
            );
        }
    }

    let page_mark_agreement = table_grid_page_mark_horizontal_best_agreement_group(
        &table_grid_page_space_horizontal_frame_candidate_supports(
            layout,
            document,
            lines,
            candidate,
            source_layout,
        ),
    );
    output.push_str(",\"pageMarkRawAgreementPresent\":");
    output.push_str(if page_mark_agreement.is_some() {
        "true"
    } else {
        "false"
    });
    match page_mark_agreement
        .as_ref()
        .and_then(|supports| supports.first())
    {
        Some(first) => {
            output.push_str(",\"pageMarkRawAgreementSelectedX\":");
            output.push_str(&format!("{:.3}", first.selected_x));
            output.push_str(",\"pageMarkRawAgreementSelectedWidth\":");
            output.push_str(&format!("{:.3}", first.selected_width));
            output.push_str(",\"pageMarkRawAgreementSupportCount\":");
            output.push_str(
                &page_mark_agreement
                    .as_ref()
                    .map(|supports| supports.len())
                    .unwrap_or(0)
                    .to_string(),
            );
            output.push_str(",\"pageMarkRawAgreementFrameBases\":");
            let frame_bases = page_mark_agreement
                .as_ref()
                .map(|supports| {
                    supports
                        .iter()
                        .map(|support| support.frame_basis)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            push_json_string_slice_array(output, &frame_bases);
            output.push_str(",\"pageMarkRawAgreementContributions\":");
            let contributions = page_mark_agreement
                .as_ref()
                .map(|supports| {
                    supports
                        .iter()
                        .map(|support| support.contribution)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            push_json_string_slice_array(output, &contributions);
            output.push_str(",\"pageMarkRawAgreementBlockedReasons\":");
            let blocked_reasons = page_mark_agreement
                .as_ref()
                .map(|supports| {
                    supports
                        .iter()
                        .map(|support| support.blocked_reason)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            push_json_string_slice_array(output, &blocked_reasons);
            output.push_str(",\"sourceFrameVsPageMarkAgreementXResidualPx\":");
            push_optional_f32_json(
                output,
                source_layout.map(|layout| layout.x - first.selected_x),
            );
            output.push_str(",\"sourceFrameVsPageMarkAgreementWidthResidualPx\":");
            push_optional_f32_json(
                output,
                source_layout.map(|layout| layout.width - first.selected_width),
            );
            output.push_str(",\"pageMarkRawAgreementConflictsWithRenderFrame\":");
            let conflicts = source_layout.is_some_and(|layout| {
                (layout.x - first.selected_x).abs() > 2.0
                    || (layout.width - first.selected_width).abs() > 2.0
            });
            output.push_str(if conflicts { "true" } else { "false" });
            output.push_str(",\"pageMarkRawAgreementRenderPromotionBlockedReason\":");
            output.push_str(&json_string(
                "page-mark-horizontal-field-semantics-unproven",
            ));
        }
        None => {
            output.push_str(",\"pageMarkRawAgreementSelectedX\":null");
            output.push_str(",\"pageMarkRawAgreementSelectedWidth\":null");
            output.push_str(",\"pageMarkRawAgreementSupportCount\":0");
            output.push_str(",\"pageMarkRawAgreementFrameBases\":[]");
            output.push_str(",\"pageMarkRawAgreementContributions\":[]");
            output.push_str(",\"pageMarkRawAgreementBlockedReasons\":[]");
            output.push_str(",\"sourceFrameVsPageMarkAgreementXResidualPx\":null");
            output.push_str(",\"sourceFrameVsPageMarkAgreementWidthResidualPx\":null");
            output.push_str(",\"pageMarkRawAgreementConflictsWithRenderFrame\":false");
            output.push_str(",\"pageMarkRawAgreementRenderPromotionBlockedReason\":null");
        }
    }
    output.push_str(",\"sourceTopTextPlacementCoherenceMirror\":");
    push_table_grid_source_top_text_placement_coherence_mirror_json(
        output, layout, document, candidate,
    );
    output.push_str(",\"renderPromotionContribution\":\"source-horizontal-frame-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if page_space_horizontal_transform_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "table-horizontal-page-space-transform-incomplete",
        ));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_source_top_text_placement_coherence_mirror_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let readiness =
        table_grid_source_top_text_placement_readiness_for_candidate(layout, document, candidate);
    output.push_str(
        "{\"source\":\"topTextTableSourceGapEvidence.sourceTablePlacementCoherenceGate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourceTopTextPlacementEvidencePresent\":");
    output.push_str(if readiness.is_some() { "true" } else { "false" });
    output.push_str(",\"sourceTopTextPlacementReady\":");
    output.push_str(
        if readiness.as_ref().is_some_and(|readiness| readiness.ready) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"readinessBlockedReasons\":");
    match readiness.as_ref() {
        Some(readiness) => push_json_string_slice_array(output, &readiness.blocked_reasons),
        None => {
            push_json_string_slice_array(output, &["source-top-text-placement-evidence-absent"])
        }
    }
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-frame-top-text-placement-coherence\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    match readiness
        .as_ref()
        .and_then(TableGridSourceTopTextPlacementReadiness::blocked_reason)
    {
        Some(reason) => output.push_str(&json_string(reason)),
        None if readiness.as_ref().is_some_and(|readiness| readiness.ready) => {
            output.push_str("null")
        }
        None => output.push_str(&json_string("source-top-text-placement-evidence-absent")),
    }
    output.push('}');
}

pub(crate) fn table_grid_page_mark_horizontal_best_agreement_group(
    supports: &[TableGridHorizontalFrameCandidateSupport],
) -> Option<Vec<TableGridHorizontalFrameCandidateSupport>> {
    let mut groups: BTreeMap<(i32, i32), Vec<TableGridHorizontalFrameCandidateSupport>> =
        BTreeMap::new();
    for support in supports
        .iter()
        .filter(|support| support.frame_basis.starts_with("page-mark"))
    {
        groups
            .entry((
                rounded_milli(support.selected_x),
                rounded_milli(support.selected_width),
            ))
            .or_default()
            .push(support.clone());
    }
    let best_support_count = groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let best_group_count = groups
        .values()
        .filter(|supports| supports.len() == best_support_count)
        .count();
    (best_support_count > 1 && best_group_count == 1)
        .then(|| {
            groups
                .values()
                .find(|supports| supports.len() == best_support_count)
                .cloned()
        })
        .flatten()
}

pub(crate) fn push_table_grid_page_space_horizontal_frame_hypotheses_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    output.push('[');
    let mut emitted = false;
    if let Some(source_layout) = source_layout {
        let selected_start = f32::from(source_layout.x_unit_start);
        let selected_width = f32::from(
            source_layout
                .x_unit_end
                .saturating_sub(source_layout.x_unit_start),
        );
        let full_extent = f32::from(source_layout.x_unit_full_extent_units);
        if full_extent > 0.0 {
            push_table_grid_page_space_horizontal_frame_hypothesis_json(
                output,
                "page-body-frame",
                layout.margin_px(),
                layout.body_width_px(),
                selected_start,
                selected_width,
                full_extent,
                "page-body-frame-not-proven-for-table",
            );
            output.push(',');
            push_table_grid_page_space_horizontal_frame_hypothesis_json(
                output,
                "page-media-box",
                0.0,
                layout.width_px(),
                selected_start,
                selected_width,
                full_extent,
                "page-media-box-not-proven-for-table",
            );
            emitted = true;
            push_table_grid_page_space_horizontal_page_mark_raw_field_hypotheses_json(
                output,
                source_layout,
            );
        }
    }
    push_table_grid_page_space_horizontal_source_only_consensus_frame_hypotheses_json(
        output,
        layout,
        document,
        lines,
        candidate,
        &mut emitted,
    );
    output.push(']');
}

pub(crate) fn push_table_grid_page_space_horizontal_frame_candidate_agreement_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    let supports = table_grid_page_space_horizontal_frame_candidate_supports(
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    let mut groups: BTreeMap<(i32, i32), Vec<TableGridHorizontalFrameCandidateSupport>> =
        BTreeMap::new();
    for support in supports {
        groups
            .entry((
                rounded_milli(support.selected_x),
                rounded_milli(support.selected_width),
            ))
            .or_default()
            .push(support);
    }
    let best_support_count = groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let best_group_count = groups
        .values()
        .filter(|supports| supports.len() == best_support_count)
        .count();
    let unique_best_supported = best_support_count > 1 && best_group_count == 1;
    let best_group = groups
        .values()
        .find(|supports| supports.len() == best_support_count && unique_best_supported);
    let source_only_selector_in_best_group = best_group.is_some_and(|supports| {
        supports
            .iter()
            .any(|support| support.contribution == "source-only-horizontal-field-selector")
    });
    let source_only_unique_selection_candidate_present =
        unique_best_supported && source_only_selector_in_best_group;
    let selection_ready = false;
    let mut blocked_reasons = Vec::new();
    if groups.is_empty() {
        blocked_reasons.push("source-horizontal-frame-candidates-absent");
    }
    if best_support_count <= 1 {
        blocked_reasons.push("source-horizontal-frame-candidate-agreement-missing");
    }
    if best_group_count > 1 {
        blocked_reasons.push("source-horizontal-frame-candidate-agreement-ambiguous");
    }
    if unique_best_supported {
        blocked_reasons.push("source-horizontal-field-semantics-still-unproven");
    }

    output.push_str(
        "{\"source\":\"pageSpaceHorizontalTransformGate.sourceFrameHypotheses agreement\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"selectionReady\":");
    output.push_str(if selection_ready { "true" } else { "false" });
    output.push_str(",\"candidateCount\":");
    output.push_str(
        &groups
            .values()
            .map(|supports| supports.len())
            .sum::<usize>()
            .to_string(),
    );
    output.push_str(",\"agreementGroupCount\":");
    output.push_str(&groups.len().to_string());
    output.push_str(",\"bestSupportCount\":");
    output.push_str(&best_support_count.to_string());
    output.push_str(",\"uniqueBestSupported\":");
    output.push_str(if unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyUniqueSelectionCandidatePresent\":");
    output.push_str(if source_only_unique_selection_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyUniqueSelectionDiagnosticOnly\":true");
    output.push_str(",\"sourceOnlyUniqueSelectionPromotionReady\":false");
    output.push_str(",\"sourceOnlyUniqueSelectionPromotionBlockedReason\":");
    if source_only_unique_selection_candidate_present {
        output.push_str(&json_string("source-horizontal-field-semantics-unproven"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"bestSupportedSelectedX\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_x)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedSelectedWidth\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_width)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedFrameBases\":");
    match best_group {
        Some(supports) => {
            let frame_bases = supports
                .iter()
                .map(|support| support.frame_basis)
                .collect::<Vec<_>>();
            push_json_string_slice_array(output, &frame_bases);
        }
        None => output.push_str("[]"),
    }
    output.push_str(",\"agreementGroups\":[");
    for (index, supports) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let first = supports.first().unwrap();
        output.push_str("{\"selectedX\":");
        output.push_str(&format!("{:.3}", first.selected_x));
        output.push_str(",\"selectedWidth\":");
        output.push_str(&format!("{:.3}", first.selected_width));
        output.push_str(",\"supportCount\":");
        output.push_str(&supports.len().to_string());
        output.push_str(",\"frameBases\":");
        let frame_bases = supports
            .iter()
            .map(|support| support.frame_basis)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &frame_bases);
        output.push_str(",\"contributions\":");
        let contributions = supports
            .iter()
            .map(|support| support.contribution)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &contributions);
        output.push_str(",\"blockedReasons\":");
        let blocked = supports
            .iter()
            .map(|support| support.blocked_reason)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &blocked);
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-frame-candidate-agreement-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("source-horizontal-field-semantics-unproven"));
    output.push('}');
}

pub(crate) fn table_grid_page_space_horizontal_frame_candidate_supports(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    if let Some(source_layout) = source_layout {
        supports.extend(
            table_grid_page_space_horizontal_page_mark_raw_field_candidate_supports(source_layout),
        );
    }
    supports.extend(
        table_grid_source_only_horizontal_field_consensus_candidate_supports(
            layout, document, lines, candidate,
        ),
    );
    if let Some(selector_support) =
        table_grid_source_only_horizontal_field_selector_candidate_support(
            layout, document, lines, candidate,
        )
    {
        supports.push(selector_support);
    }
    supports
}

pub(crate) fn table_grid_page_space_horizontal_page_mark_raw_field_candidate_supports(
    source_layout: &TableGridSourceDerivedLayout,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    let Some(fields) = table_grid_source_layout_page_mark_u16_fields(source_layout) else {
        return supports;
    };
    let Some(word_14) = fields.get(14).copied() else {
        return supports;
    };
    let Some(word_21) = fields.get(21).copied() else {
        return supports;
    };
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-word21-direct",
        selected_x: f32::from(word_14),
        selected_width: f32::from(word_21),
        contribution: "source-horizontal-page-mark-raw-field-hypothesis",
        blocked_reason: "page-mark-raw-horizontal-field-semantics-unproven",
    });

    let Some(first_slot_units) = source_layout
        .x_unit_column_slot_width_units
        .first()
        .copied()
        .filter(|units| *units > 0)
    else {
        return supports;
    };
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let selected_width = f32::from(word_21) - f32::from(first_slot_units) * 0.5;
    if selected_x.is_finite() && selected_width.is_finite() && selected_width > 0.0 {
        supports.push(TableGridHorizontalFrameCandidateSupport {
            frame_basis: "page-mark-word14-word21-first-slot-adjusted",
            selected_x,
            selected_width,
            contribution: "source-horizontal-page-mark-raw-field-hypothesis",
            blocked_reason: "page-mark-raw-horizontal-field-semantics-unproven",
        });
    }
    supports
}

pub(crate) fn table_grid_source_only_horizontal_field_consensus_candidate_supports(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let Some((page_mark_fields, stable_first_slot_units, stable_first_gap_units)) =
        table_grid_source_only_horizontal_field_consensus_inputs(
            layout, document, lines, candidate,
        )
    else {
        return Vec::new();
    };
    table_grid_source_only_horizontal_field_consensus_supports(
        &page_mark_fields,
        stable_first_slot_units,
        stable_first_gap_units,
    )
}

pub(crate) fn table_grid_source_only_horizontal_field_selector_candidate_support(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Option<TableGridHorizontalFrameCandidateSupport> {
    let (page_mark_fields, stable_first_slot_units, stable_first_gap_units) =
        table_grid_source_only_horizontal_field_consensus_inputs(
            layout, document, lines, candidate,
        )?;
    let word_14 = page_mark_fields.get(14).copied()?;
    let first_slot_units = stable_first_slot_units.filter(|units| *units > 0)?;
    let compact_column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let (frame_basis, selected_width) = match compact_column_count {
        2 => {
            let word_21 = page_mark_fields.get(21).copied()?;
            (
                "page-mark-word14-first-slot-word21-half-slot",
                f32::from(word_21) - f32::from(first_slot_units) * 0.5,
            )
        }
        3 => {
            let word_15 = page_mark_fields.get(15).copied()?;
            let first_gap_units = stable_first_gap_units.filter(|units| *units > 0)?;
            (
                "page-mark-word14-first-slot-word15-half-gap",
                f32::from(word_15) - f32::from(first_gap_units) * 0.5,
            )
        }
        _ => return None,
    };
    if !selected_x.is_finite() || !selected_width.is_finite() || selected_width <= 0.0 {
        return None;
    }
    Some(TableGridHorizontalFrameCandidateSupport {
        frame_basis,
        selected_x,
        selected_width,
        contribution: "source-only-horizontal-field-selector",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    })
}

pub(crate) fn table_grid_source_only_horizontal_field_consensus_inputs(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Option<(Vec<u16>, Option<u16>, Option<u16>)> {
    let current_source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = current_source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let page_mark_fields = source_layout_page_mark_fields
        .or(cross_table_page_mark_fields)?
        .to_vec();

    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate);
    let sparse_table_candidate_index = sibling
        .as_ref()
        .map(|evidence| evidence.sparse_candidate.index());
    let related_source_layouts = table_grid_related_horizontal_source_layout_summaries(
        layout,
        document,
        lines,
        candidate,
        sparse_table_candidate_index,
        current_source_layout.as_ref(),
    );
    if related_source_layouts.is_empty() {
        return None;
    }

    let first_slot_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_column_slot_units)
        .collect::<Vec<_>>();
    let first_gap_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_intercell_gap_units)
        .collect::<Vec<_>>();
    Some((
        page_mark_fields,
        single_u16_value(&first_slot_units),
        single_u16_value(&first_gap_units),
    ))
}

pub(crate) fn table_grid_source_only_horizontal_field_consensus_supports(
    page_mark_fields: &[u16],
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    let Some(word_14) = page_mark_fields.get(14).copied() else {
        return supports;
    };
    let Some(word_15) = page_mark_fields.get(15).copied() else {
        return supports;
    };
    let Some(word_21) = page_mark_fields.get(21).copied() else {
        return supports;
    };
    let Some(first_slot_units) = stable_first_slot_units.filter(|units| *units > 0) else {
        return supports;
    };
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-first-slot-word15-direct",
        selected_x,
        selected_width: f32::from(word_15),
        contribution: "source-only-horizontal-field-consensus",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    });
    if let Some(half_gap_adjustment) = stable_first_gap_units
        .filter(|units| *units > 0)
        .map(|units| f32::from(units) * 0.5)
    {
        supports.push(TableGridHorizontalFrameCandidateSupport {
            frame_basis: "page-mark-word14-first-slot-word15-half-gap",
            selected_x,
            selected_width: f32::from(word_15) - half_gap_adjustment,
            contribution: "source-only-horizontal-field-consensus",
            blocked_reason: "cross-table-horizontal-field-semantics-unproven",
        });
    }
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-first-slot-word21-half-slot",
        selected_x,
        selected_width: f32::from(word_21) - f32::from(first_slot_units) * 0.5,
        contribution: "source-only-horizontal-field-consensus",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    });
    supports
}

pub(crate) fn push_table_grid_page_space_horizontal_source_only_consensus_frame_hypotheses_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    emitted: &mut bool,
) {
    let current_source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = current_source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let Some(page_mark_fields) = source_layout_page_mark_fields.or(cross_table_page_mark_fields)
    else {
        return;
    };

    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate);
    let sparse_table_candidate_index = sibling
        .as_ref()
        .map(|evidence| evidence.sparse_candidate.index());
    let related_source_layouts = table_grid_related_horizontal_source_layout_summaries(
        layout,
        document,
        lines,
        candidate,
        sparse_table_candidate_index,
        current_source_layout.as_ref(),
    );
    if related_source_layouts.is_empty() {
        return;
    }

    let first_slot_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_column_slot_units)
        .collect::<Vec<_>>();
    let first_gap_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_intercell_gap_units)
        .collect::<Vec<_>>();
    push_table_grid_source_only_horizontal_field_consensus_hypotheses_items_json(
        output,
        page_mark_fields,
        single_u16_value(&first_slot_units),
        single_u16_value(&first_gap_units),
        emitted,
    );
}

pub(crate) fn push_table_grid_page_space_horizontal_page_mark_raw_field_hypotheses_json(
    output: &mut String,
    source_layout: &TableGridSourceDerivedLayout,
) {
    let Some(fields) = table_grid_source_layout_page_mark_u16_fields(source_layout) else {
        return;
    };
    let Some(word_14) = fields.get(14).copied() else {
        return;
    };
    let Some(word_21) = fields.get(21).copied() else {
        return;
    };

    output.push(',');
    push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
        output,
        "page-mark-word14-word21-direct",
        f32::from(word_14),
        f32::from(word_21),
        word_14,
        word_21,
        None,
        0.0,
        0.0,
        "none",
    );

    let Some(first_slot_units) = source_layout
        .x_unit_column_slot_width_units
        .first()
        .copied()
        .filter(|units| *units > 0)
    else {
        return;
    };
    let x_adjustment_units = f32::from(first_slot_units);
    let width_adjustment_units = f32::from(first_slot_units) * 0.5;
    let selected_x = f32::from(word_14) - x_adjustment_units;
    let selected_width = f32::from(word_21) - width_adjustment_units;
    if selected_x.is_finite() && selected_width.is_finite() && selected_width > 0.0 {
        output.push(',');
        push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
            output,
            "page-mark-word14-word21-first-slot-adjusted",
            selected_x,
            selected_width,
            word_14,
            word_21,
            Some(first_slot_units),
            x_adjustment_units,
            width_adjustment_units,
            "line-header-first-column-slot",
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
    output: &mut String,
    frame_basis: &'static str,
    selected_x: f32,
    selected_width: f32,
    page_mark_word_14: u16,
    page_mark_word_21: u16,
    first_column_slot_units: Option<u16>,
    x_adjustment_units: f32,
    width_adjustment_units: f32,
    adjustment_basis: &'static str,
) {
    output.push_str("{\"frameBasis\":");
    output.push_str(&json_string(frame_basis));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateBasis\":\"page-mark-direct-u16-px\"");
    output.push_str(",\"pageMarkWord14\":");
    output.push_str(&page_mark_word_14.to_string());
    output.push_str(",\"pageMarkWord21\":");
    output.push_str(&page_mark_word_21.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    push_optional_u16_json(output, first_column_slot_units);
    output.push_str(",\"xAdjustmentUnits\":");
    output.push_str(&format!("{x_adjustment_units:.3}"));
    output.push_str(",\"widthAdjustmentUnits\":");
    output.push_str(&format!("{width_adjustment_units:.3}"));
    output.push_str(",\"adjustmentBasis\":");
    output.push_str(&json_string(adjustment_basis));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-page-mark-raw-field-hypothesis\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-raw-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_page_space_horizontal_frame_hypothesis_json(
    output: &mut String,
    frame_basis: &'static str,
    frame_x: f32,
    frame_width: f32,
    selected_start_units: f32,
    selected_width_units: f32,
    full_extent_units: f32,
    blocked_reason: &'static str,
) {
    let selected_x = frame_x + frame_width * selected_start_units / full_extent_units;
    let selected_width = frame_width * selected_width_units / full_extent_units;
    output.push_str("{\"frameBasis\":");
    output.push_str(&json_string(frame_basis));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"frameX\":");
    output.push_str(&format!("{frame_x:.3}"));
    output.push_str(",\"frameWidth\":");
    output.push_str(&format!("{frame_width:.3}"));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(",\"selectedStartUnits\":");
    output.push_str(&format!("{selected_start_units:.3}"));
    output.push_str(",\"selectedWidthUnits\":");
    output.push_str(&format!("{selected_width_units:.3}"));
    output.push_str(",\"fullExtentUnits\":");
    output.push_str(&format!("{full_extent_units:.3}"));
    output.push_str(",\"renderPromotionContribution\":\"source-horizontal-frame-hypothesis\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(blocked_reason));
    output.push('}');
}
