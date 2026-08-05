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
