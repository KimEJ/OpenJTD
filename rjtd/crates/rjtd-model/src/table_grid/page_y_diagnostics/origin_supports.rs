use super::*;
use crate::*;

pub(crate) fn table_grid_source_only_page_y_origin_candidate_supports(
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> Vec<TableGridSourceOnlyPageYOriginCandidateSupport> {
    let mut supports = Vec::new();
    if let Some(source_layout) = source_layout {
        if let Some(origin) = source_layout.line_mark_page_origin.as_ref() {
            push_table_grid_source_only_page_y_origin_candidate_support(
                &mut supports,
                "line-mark-page-origin-direct",
                origin.y,
                Some(origin.row_height),
                None,
                "source-backed-page-y-origin",
                "line-mark-page-origin-rule-still-needs-cross-table-validation",
            );
        }
        if let Some(stride) = source_layout.line_mark_page_origin_stride.as_ref() {
            if let Some(selected_y) = stride.raw_record_index_row_tops.first().copied() {
                push_table_grid_source_only_page_y_origin_candidate_support(
                    &mut supports,
                    "line-mark-stride-raw-record-index-first-row",
                    selected_y,
                    Some(stride.row_height),
                    None,
                    "source-only-line-mark-stride-page-y-origin",
                    "stride-origin-needs-page-origin-rule",
                );
            }
            if let Some(selected_y) = stride.stride_collapsed_row_tops.first().copied() {
                push_table_grid_source_only_page_y_origin_candidate_support(
                    &mut supports,
                    "line-mark-stride-collapsed-record-index-first-row",
                    selected_y,
                    Some(stride.row_height),
                    None,
                    "source-only-line-mark-stride-page-y-origin",
                    "line-mark-record-stride-to-page-y-transform-unproven",
                );
            }
        }
    }
    if source_layout.is_some_and(table_grid_source_layout_supports_page_mark_absolute_y_slot) {
        let absolute_y_slot_agreement = table_grid_source_only_page_mark_absolute_y_slot_agreement(
            document,
            candidate,
            cross_table_row_boundary_offset_probe,
            subrecord_span_readiness,
        );
        // A slot already refuted as page-space px by the record-flag alias or by
        // field quantization is not a y-origin candidate at all, so it must not
        // reach the selector as a support.
        if let Some(slot) = absolute_y_slot_agreement.best_absolute_y_slot.as_ref()
            && slot.field_index == 2
            && slot.tail_block16_word_index == Some(11)
            && !absolute_y_slot_agreement.refuted_as_page_space_px()
        {
            let blocked_reason = if absolute_y_slot_agreement.semantics_ready() {
                "none"
            } else {
                "page-mark-absolute-y-slot-semantics-unproven"
            };
            let mut extra_blocked_reasons = Vec::new();
            if absolute_y_slot_agreement.best_absolute_y_slot.is_some()
                && absolute_y_slot_agreement.line_domain_projected_y.is_some()
                && !absolute_y_slot_agreement.agrees
            {
                extra_blocked_reasons
                    .push("line-domain-projection-disagrees-with-page-mark-absolute-y-slot");
            }
            extra_blocked_reasons
                .extend(absolute_y_slot_agreement.field_quantization_blocked_reasons());
            extra_blocked_reasons
                .extend(absolute_y_slot_agreement.record_flag_alias_blocked_reasons());
            push_table_grid_source_only_page_y_origin_candidate_support_with_extra_blockers(
                &mut supports,
                "page-mark-absolute-y-slot-field2-tail-block16-word11",
                slot.value_px,
                None,
                Some(candidate.index()),
                "source-only-page-mark-absolute-y-slot-y-origin",
                blocked_reason,
                &extra_blocked_reasons,
            );
        }
    }
    if let Some(probe) = cross_table_row_boundary_offset_probe {
        if let Some(selected_y) = probe.combined_line_mark_record_y_tops_px.first().copied() {
            push_table_grid_source_only_page_y_origin_candidate_support(
                &mut supports,
                "cross-table-combined-previous-row-span-first-record",
                selected_y,
                probe.combined_line_mark_record_y_pitch_px,
                None,
                "cross-table-row-boundary-offset-diagnostic-only",
                "page-line-gap-projection-does-not-decode-table-y-origin",
            );
        }
        for table in &probe.tables {
            if let Some(selected_y) = table.line_mark_record_y_tops_px.first().copied() {
                push_table_grid_source_only_page_y_origin_candidate_support(
                    &mut supports,
                    "cross-table-previous-row-span-table-first-row",
                    selected_y,
                    probe.combined_line_mark_record_y_pitch_px,
                    Some(table.table_candidate_index),
                    "cross-table-row-boundary-offset-diagnostic-only",
                    "cross-table-row-boundary-offset-transform-required",
                );
            }
            if let Some(selected_y) = table.selected_spacing_record_y_tops_px.first().copied() {
                push_table_grid_source_only_page_y_origin_candidate_support(
                    &mut supports,
                    "cross-table-selected-spacing-table-first-row",
                    selected_y,
                    probe.combined_line_mark_record_y_pitch_px,
                    Some(table.table_candidate_index),
                    "source-unit-to-page-line-family-gap-piecewise-diagnostic-only",
                    "selected-spacing-record-family-is-not-page-y-origin",
                );
            }
        }
    }
    supports
}

pub(crate) fn table_grid_source_layout_supports_page_mark_absolute_y_slot(
    layout: &TableGridSourceDerivedLayout,
) -> bool {
    layout.page_origin_authority == "lineMarkPageGridStrideRawRecordIndex"
        && layout.line_mark_page_origin_stride.is_some()
}

pub(crate) fn table_grid_cross_table_previous_row_span_y_origin_support_count(
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
) -> usize {
    let Some(probe) = cross_table_row_boundary_offset_probe else {
        return 0;
    };
    usize::from(!probe.combined_line_mark_record_y_tops_px.is_empty())
        + probe
            .tables
            .iter()
            .filter(|table| !table.line_mark_record_y_tops_px.is_empty())
            .count()
}

pub(crate) fn table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(
    support: &TableGridSourceOnlyPageYOriginCandidateSupport,
) -> bool {
    matches!(
        support.origin_basis,
        "cross-table-combined-previous-row-span-first-record"
            | "cross-table-previous-row-span-table-first-row"
    )
}

pub(crate) fn push_table_grid_source_only_page_y_origin_candidate_support(
    supports: &mut Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    origin_basis: &'static str,
    selected_y: f32,
    row_height: Option<f32>,
    table_candidate_index: Option<usize>,
    contribution: &'static str,
    blocked_reason: &'static str,
) {
    push_table_grid_source_only_page_y_origin_candidate_support_with_extra_blockers(
        supports,
        origin_basis,
        selected_y,
        row_height,
        table_candidate_index,
        contribution,
        blocked_reason,
        &[],
    );
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_source_only_page_y_origin_candidate_support_with_extra_blockers(
    supports: &mut Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    origin_basis: &'static str,
    selected_y: f32,
    row_height: Option<f32>,
    table_candidate_index: Option<usize>,
    contribution: &'static str,
    blocked_reason: &'static str,
    extra_blocked_reasons: &[&'static str],
) {
    if !selected_y.is_finite() || row_height.is_some_and(|height| !height.is_finite()) {
        return;
    }
    supports.push(TableGridSourceOnlyPageYOriginCandidateSupport {
        origin_basis,
        selected_y,
        row_height,
        table_candidate_index,
        contribution,
        blocked_reason,
        extra_blocked_reasons: extra_blocked_reasons.to_vec(),
    });
}

pub(crate) fn table_grid_source_only_page_y_origin_supports_blocked_reasons(
    supports: &[TableGridSourceOnlyPageYOriginCandidateSupport],
) -> Vec<&'static str> {
    let mut reasons = BTreeSet::new();
    for support in supports {
        table_grid_insert_source_only_page_y_origin_blocker(&mut reasons, support.blocked_reason);
        for reason in &support.extra_blocked_reasons {
            table_grid_insert_source_only_page_y_origin_blocker(&mut reasons, reason);
        }
    }
    reasons.into_iter().collect()
}

pub(crate) fn table_grid_insert_source_only_page_y_origin_blocker(
    reasons: &mut BTreeSet<&'static str>,
    reason: &'static str,
) {
    if reason == "none" {
        return;
    }
    reasons.insert(reason);
}

pub(crate) fn push_table_grid_line_mark_page_origin_candidate_json(
    output: &mut String,
    candidate: Option<&TableGridLineMarkPageOriginCandidate>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/LineMark+/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":true");
    output.push_str(",\"y\":");
    output.push_str(&format!("{:.3}", candidate.y));
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
    output.push_str(",\"lineOffsetFromPageStart\":");
    output.push_str(&candidate.line_offset_from_page_start.to_string());
    output.push_str(",\"linePitchPx\":");
    output.push_str(&format!("{:.3}", candidate.line_pitch_px));
    output.push_str(",\"linePitchBasis\":");
    output.push_str(&json_string(candidate.line_pitch_basis));
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{:.3}", candidate.row_height));
    output.push_str(",\"renderPromotionContribution\":\"source-backed-page-y-origin\"");
    output.push_str(",\"renderPromotionBlockedReason\":null}");
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_line_mark_stride_to_page_y_promotion_readiness_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
    cross_table_ordering_probe: Option<&TableGridCrossTableSubrecordOrderingProbe>,
    selected_complete: bool,
    selected_ordered_unique_complete: bool,
    y_origin_solver_ready: bool,
) {
    let stride_candidate_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_page_origin_stride.is_some());
    let line_mark_page_origin_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_page_origin.is_some());
    let source_range_coverage =
        table_grid_page_mark_raw_record_source_range_coverage_summary(document, candidate);
    let source_only_stride_row_coverage =
        table_grid_source_only_stride_row_coverage_summary(document, candidate, source_layout);
    let line_mark_row_boundary_alignment =
        table_grid_line_mark_row_boundary_alignment_summary(document, candidate, source_layout);
    let source_only_stride_rows_covered = source_only_stride_row_coverage
        .as_ref()
        .is_some_and(|summary| summary.all_rows_covered);
    let page_mark_entry_line_bounds_coverage =
        table_grid_stride_page_mark_entry_line_bounds_coverage_summary(source_layout);
    let page_mark_entry_line_bounds_ready = page_mark_entry_line_bounds_coverage
        .as_ref()
        .is_some_and(|summary| summary.coverage_ready);
    let page_mark_subrecord_line_range_record_coverage =
        table_grid_page_mark_subrecord_line_range_record_coverage_summary(
            document,
            source_layout,
            subrecord_span_readiness,
        );
    let cross_table_ordering_consistent =
        cross_table_ordering_probe.is_some_and(|probe| probe.cross_table_ordering_consistent);
    let source_order_contradiction = cross_table_ordering_probe.is_some_and(|probe| {
        !probe.monotonic_raw_record_scan_index
            || !probe.monotonic_line_start_candidate
            || probe.family_reused_after_later_family
    });
    let promotion_ready = stride_candidate_present
        && line_mark_page_origin_present
        && selected_complete
        && selected_ordered_unique_complete
        && source_only_stride_rows_covered
        && page_mark_entry_line_bounds_ready
        && cross_table_ordering_consistent
        && y_origin_solver_ready;

    let mut blocked_reasons = Vec::new();
    if !stride_candidate_present {
        blocked_reasons.push("line-mark-stride-candidate-absent");
    }
    if !line_mark_page_origin_present {
        blocked_reasons.push("line-mark-page-origin-candidate-absent");
    }
    if subrecord_span_readiness.is_none() {
        blocked_reasons.push("page-mark-subrecord-line-span-readiness-absent");
    }
    if !selected_complete {
        blocked_reasons.push("selected-post-row-gap-span-incomplete");
    }
    if !selected_ordered_unique_complete {
        blocked_reasons.push("selected-post-row-gap-subrecord-coverage-not-ordered-unique");
    }
    if source_only_stride_row_coverage.is_none() {
        blocked_reasons.push("source-only-stride-row-coverage-absent");
    } else if !source_only_stride_rows_covered {
        blocked_reasons.push("line-mark-row-spans-do-not-cover-table-row-source-spans");
    }
    if page_mark_entry_line_bounds_coverage.is_none() {
        blocked_reasons.push("page-mark-entry-line-bounds-coverage-absent");
    } else if !page_mark_entry_line_bounds_ready {
        blocked_reasons.push("line-mark-records-not-contained-in-page-mark-entry");
    }
    if !cross_table_ordering_consistent {
        blocked_reasons.push("cross-table-subrecord-ordering-inconsistent");
    }
    if source_order_contradiction {
        blocked_reasons.push("source-order-vs-subrecord-order-contradiction");
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("decoded-page-y-origin-missing");
    }

    output.push_str("{\"source\":\"/LineMark+/PageMark stride-to-page-y promotion readiness\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":");
    output.push_str(if promotion_ready { "true" } else { "false" });
    output.push_str(",\"strideCandidatePresent\":");
    output.push_str(if stride_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(if line_mark_page_origin_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyStrideRowCoverage\":");
    push_table_grid_source_only_stride_row_coverage_summary_json(
        output,
        source_only_stride_row_coverage.as_ref(),
    );
    output.push_str(",\"lineMarkRowBoundaryAlignment\":");
    push_table_grid_line_mark_row_boundary_alignment_summary_json(
        output,
        line_mark_row_boundary_alignment.as_ref(),
    );
    output.push_str(",\"pageMarkEntryLineBoundsCoverage\":");
    push_table_grid_stride_page_mark_entry_line_bounds_coverage_summary_json(
        output,
        page_mark_entry_line_bounds_coverage.as_ref(),
    );
    output.push_str(",\"pageMarkSubrecordLineRangeRecordCoverage\":");
    push_table_grid_page_mark_subrecord_line_range_record_coverage_summary_json(
        output,
        page_mark_subrecord_line_range_record_coverage.as_ref(),
    );
    output.push_str(",\"rawRecordSourceRangeCoverageDomain\":\"legacy-cross-domain-document-text-unit-range-vs-page-mark-line-index\"");
    output.push_str(",\"rawRecordSourceRangeCoverageUsableForPromotion\":false");
    output.push_str(",\"rawRecordSourceRangeCoverage\":");
    match source_range_coverage.as_ref() {
        Some(summary) => {
            push_table_grid_page_mark_raw_record_source_range_coverage_summary_json(output, summary)
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"crossTableOrderingConsistent\":");
    output.push_str(if cross_table_ordering_consistent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOrderVsSubrecordOrderContradiction\":");
    output.push_str(if source_order_contradiction {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"line-mark-stride-to-page-y-readiness-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if promotion_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "stride-y-hypothesis-needs-source-only-validation",
        ));
    }
    output.push('}');
}
