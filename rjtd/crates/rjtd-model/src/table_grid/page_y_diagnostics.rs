use super::*;
use crate::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_reference_calibration_replacement_gate_json(
    output: &mut String,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    source_layout_renderable: bool,
    horizontal_solver_ready: bool,
    source_column_split_ready: bool,
    page_space_horizontal_transform_ready: bool,
    row_height_solver_ready: bool,
    y_origin_solver_ready: bool,
) {
    let source_layout_present = source_layout.is_some();
    let replacement_ready = source_layout_renderable
        && horizontal_solver_ready
        && source_column_split_ready
        && page_space_horizontal_transform_ready
        && row_height_solver_ready
        && y_origin_solver_ready;
    let mut blocked_reasons = Vec::new();
    if !source_layout_present {
        blocked_reasons.push("source-derived-layout-candidate-absent");
    }
    if !horizontal_solver_ready {
        blocked_reasons.push("table-horizontal-source-transform-incomplete");
    }
    if !source_column_split_ready {
        blocked_reasons.push("source-column-split-not-ready");
    }
    if !page_space_horizontal_transform_ready {
        blocked_reasons.push("table-horizontal-page-space-transform-incomplete");
    }
    if !row_height_solver_ready {
        blocked_reasons.push("table-row-height-source-transform-incomplete");
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("source-page-y-transform-not-decoded");
    }
    if source_layout_present && !source_layout_renderable {
        blocked_reasons.push("source-derived-layout-not-renderable");
    }

    output.push_str("{\"source\":\"table-page-space-v1 reference calibration replacement gate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"replacementReady\":");
    output.push_str(if replacement_ready { "true" } else { "false" });
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout_present {
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
    output.push_str(",\"horizontalSolverReady\":");
    output.push_str(if horizontal_solver_ready {
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
    output.push_str(",\"pageSpaceHorizontalTransformReady\":");
    output.push_str(if page_space_horizontal_transform_ready {
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
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"reference-calibration-replacement-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if replacement_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-table-page-space-not-ready"));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_source_page_y_transform_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
    y_origin_solver_ready: bool,
    line_mark_rows_exact_and_contiguous: bool,
) {
    let layout = page_layout_from_document(document);
    let cross_table_ordering_probe =
        table_grid_cross_table_subrecord_ordering_probe(document, candidate);
    let cross_table_row_boundary_offset_probe =
        table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let selected_complete = subrecord_span_readiness.as_ref().is_some_and(|readiness| {
        !readiness.selected_post_row_gap_span_targets.is_empty()
            && readiness.selected_post_row_gap_span_hit_count
                == readiness.selected_post_row_gap_span_targets.len()
    });
    let previous_complete = subrecord_span_readiness.as_ref().is_some_and(|readiness| {
        !readiness.previous_row_span_targets.is_empty()
            && readiness.previous_row_span_hit_count == readiness.previous_row_span_targets.len()
    });
    let compact_complete = subrecord_span_readiness.as_ref().is_some_and(|readiness| {
        !readiness.compact_row_span_targets.is_empty()
            && readiness.compact_row_span_hit_count == readiness.compact_row_span_targets.len()
    });
    let selected_ordered_unique_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            readiness
                .selected_post_row_gap_span_coverage
                .ordered_unique_coverage_complete
        });
    let previous_ordered_unique_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            readiness
                .previous_row_span_coverage
                .ordered_unique_coverage_complete
        });

    let mut blocked_reasons = Vec::new();
    match source_layout {
        Some(layout) => {
            if layout.line_mark_page_origin.is_none() {
                blocked_reasons.push("line-mark-page-origin-candidate-absent");
            }
            if layout.line_mark_page_origin_stride.is_some() {
                blocked_reasons.push("line-mark-record-stride-to-page-y-transform-unproven");
            }
            if layout.page_origin_authority != "lineMarkPageGrid" {
                blocked_reasons.push("page-origin-authority-not-renderable-line-mark-page-grid");
            }
            if !line_mark_rows_exact_and_contiguous {
                blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
            }
        }
        None => blocked_reasons.push("source-derived-layout-candidate-absent"),
    }
    if selected_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-selected-post-row-gaps");
        if !selected_ordered_unique_complete {
            blocked_reasons
                .push("page-mark-subrecord-selected-post-row-gap-candidates-not-row-unique");
        }
    }
    if previous_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-previous-row-spans");
        if !previous_ordered_unique_complete {
            blocked_reasons.push("page-mark-subrecord-previous-row-span-candidates-not-row-unique");
        }
    }
    if compact_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-compact-row-spans");
    }
    if subrecord_span_readiness.is_some() {
        blocked_reasons.push("page-mark-subrecord-spans-do-not-decode-page-y-origin");
    }
    if cross_table_ordering_probe
        .as_ref()
        .is_some_and(|probe| !probe.monotonic_raw_record_scan_index)
    {
        blocked_reasons.push("page-mark-cross-table-raw-record-order-regression");
    }
    if cross_table_ordering_probe
        .as_ref()
        .is_some_and(|probe| !probe.cross_table_ordering_consistent)
    {
        blocked_reasons.push("page-mark-cross-table-subrecord-ordering-unproven");
    }
    if cross_table_row_boundary_offset_probe
        .as_ref()
        .is_some_and(|probe| probe.all_offsets_require_transform)
    {
        blocked_reasons.push("cross-table-row-boundary-offset-transform-required");
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("decoded-line-mark-page-y-transform-missing");
    }

    output.push_str(
        "{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark y-origin promotion gate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":");
    output.push_str(if y_origin_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
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
    output.push_str(",\"subrecordLineSpanReadinessPresent\":");
    output.push_str(if subrecord_span_readiness.is_some() {
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
    output.push_str(",\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if previous_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanOrderedCoverage\":");
    match subrecord_span_readiness {
        Some(readiness) => push_table_grid_page_mark_subrecord_line_span_coverage_json(
            output,
            &readiness.selected_post_row_gap_span_coverage,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"previousRowSpanOrderedCoverage\":");
    match subrecord_span_readiness {
        Some(readiness) => push_table_grid_page_mark_subrecord_line_span_coverage_json(
            output,
            &readiness.previous_row_span_coverage,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"crossTableSubrecordOrderingProbe\":");
    push_table_grid_cross_table_subrecord_ordering_probe_summary_json(
        output,
        cross_table_ordering_probe.as_ref(),
    );
    output.push_str(",\"crossTableRowBoundaryOffsetConsistency\":");
    push_table_grid_cross_table_row_boundary_offset_probe_summary_json(
        output,
        layout,
        document,
        cross_table_row_boundary_offset_probe.as_ref(),
    );
    output.push_str(",\"lineMarkStrideToPageYPromotionReadiness\":");
    push_table_grid_line_mark_stride_to_page_y_promotion_readiness_json(
        output,
        document,
        candidate,
        source_layout,
        subrecord_span_readiness,
        cross_table_ordering_probe.as_ref(),
        selected_complete,
        selected_ordered_unique_complete,
        y_origin_solver_ready,
    );
    output.push_str(",\"sourceOnlyPageYOriginHypothesis\":");
    push_table_grid_source_only_page_y_origin_hypothesis_json(
        output,
        document,
        candidate,
        source_layout,
        subrecord_span_readiness,
        cross_table_ordering_probe.as_ref(),
        cross_table_row_boundary_offset_probe.as_ref(),
        selected_complete,
        selected_ordered_unique_complete,
        previous_complete,
        previous_ordered_unique_complete,
        compact_complete,
        y_origin_solver_ready,
        line_mark_rows_exact_and_contiguous,
    );
    output.push_str(",\"sourceOnlyPageYOriginCandidateAgreementGate\":");
    push_table_grid_source_only_page_y_origin_candidate_agreement_gate_json(
        output,
        document,
        candidate,
        source_layout,
        cross_table_row_boundary_offset_probe.as_ref(),
        subrecord_span_readiness,
    );
    output.push_str(",\"sourceOnlyPageYOriginDomainGate\":");
    push_table_grid_source_only_page_y_origin_domain_gate_json(
        output,
        source_layout,
        cross_table_row_boundary_offset_probe.as_ref(),
    );
    output.push_str(",\"sourceOnlyPageMarkAbsoluteYSlotGate\":");
    push_table_grid_source_only_page_mark_absolute_y_slot_gate_json(
        output,
        document,
        candidate,
        cross_table_row_boundary_offset_probe.as_ref(),
        subrecord_span_readiness,
    );
    output.push_str(",\"lineDomainPostRowGapProjectionProbe\":");
    push_table_grid_line_domain_post_row_gap_projection_probe_json(
        output,
        layout,
        document,
        candidate,
        cross_table_row_boundary_offset_probe.as_ref(),
        subrecord_span_readiness,
    );
    output.push_str(",\"sourceOnlyPageYRenderAdmissionGate\":");
    push_table_grid_source_only_page_y_render_admission_gate_json(
        output,
        document,
        candidate,
        source_layout,
        subrecord_span_readiness,
        cross_table_ordering_probe.as_ref(),
        cross_table_row_boundary_offset_probe.as_ref(),
        y_origin_solver_ready,
        line_mark_rows_exact_and_contiguous,
        selected_complete,
        selected_ordered_unique_complete,
        previous_complete,
        previous_ordered_unique_complete,
        compact_complete,
    );
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-page-y-transform-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if y_origin_solver_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-page-y-transform-not-decoded"));
    }
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_source_only_page_y_render_admission_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
    cross_table_ordering_probe: Option<&TableGridCrossTableSubrecordOrderingProbe>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    y_origin_solver_ready: bool,
    line_mark_rows_exact_and_contiguous: bool,
    selected_complete: bool,
    selected_ordered_unique_complete: bool,
    previous_complete: bool,
    previous_ordered_unique_complete: bool,
    compact_complete: bool,
) {
    let source_layout_candidate_present = source_layout.is_some();
    let direct_line_mark_page_origin_present = source_layout.is_some_and(|layout| {
        layout.line_mark_page_origin.is_some() && layout.page_origin_authority == "lineMarkPageGrid"
    });
    let line_mark_page_origin_stride_present =
        source_layout.is_some_and(|layout| layout.line_mark_page_origin_stride.is_some());
    let cross_table_line_domain_present = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| !probe.combined_line_mark_record_y_tops_px.is_empty());
    let cross_table_order_regresses = cross_table_ordering_probe.is_some_and(|probe| {
        !probe.monotonic_raw_record_scan_index
            || !probe.monotonic_line_start_candidate
            || probe.family_reused_after_later_family
    });
    let cross_table_row_boundary_offset_transform_required = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| probe.all_offsets_require_transform);
    let direct_line_mark_origin_admissible = direct_line_mark_page_origin_present
        && line_mark_rows_exact_and_contiguous
        && y_origin_solver_ready;
    let source_only_page_y_admission_class = if direct_line_mark_origin_admissible {
        "direct-line-mark-page-grid"
    } else if line_mark_page_origin_stride_present && !direct_line_mark_page_origin_present {
        "flow-y-stride-only-diagnostic"
    } else {
        "not-admissible"
    };

    let page_mark_absolute_y_slot_agreement =
        table_grid_source_only_page_mark_absolute_y_slot_agreement(
            document,
            candidate,
            cross_table_row_boundary_offset_probe,
            subrecord_span_readiness,
        );
    let page_mark_absolute_y_slot_semantics_ready =
        page_mark_absolute_y_slot_agreement.semantics_ready();
    let page_mark_absolute_y_slot_blocked_reason =
        table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(
            &page_mark_absolute_y_slot_agreement,
        );
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe,
        );
    let source_gap_to_page_line_gap_table_family_transform_required =
        cross_table_line_domain_present || cross_table_row_boundary_offset_transform_required;

    let y_supports = table_grid_source_only_page_y_origin_candidate_supports(
        document,
        candidate,
        source_layout,
        cross_table_row_boundary_offset_probe,
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
    let y_selector_uses_single_support_fallback =
        y_candidate_best_group.is_none() && y_fallback_selector_group.is_some();
    let y_selector_support_count = y_selector_group.map(|supports| supports.len()).unwrap_or(0);
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
    let cross_table_previous_row_span_table_candidate_count = y_groups
        .values()
        .flatten()
        .filter(|support| {
            table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(support)
        })
        .filter_map(|support| support.table_candidate_index)
        .collect::<BTreeSet<_>>()
        .len();
    let y_selector_support_fragmented_by_table = !y_selector_uses_single_support_fallback
        && cross_table_previous_row_span_table_candidate_count > 1
        && y_selector_table_candidate_indexes.len()
            < cross_table_previous_row_span_table_candidate_count;
    let y_selector_support_blocked_reasons = y_selector_group
        .map(|supports| table_grid_source_only_page_y_origin_supports_blocked_reasons(supports))
        .unwrap_or_default();
    let y_selector_blocked_reason = if y_selector_group.is_none() {
        "source-y-origin-selector-absent"
    } else if y_selector_uses_single_support_fallback {
        "source-y-origin-selector-single-support-fallback-not-render-admissible"
    } else if y_selector_support_fragmented_by_table {
        "source-y-origin-selector-fragmented-by-table-not-render-admissible"
    } else if !y_unique_best_supported {
        "source-y-origin-selector-agreement-unproven"
    } else if !y_selector_support_blocked_reasons.is_empty() {
        "source-y-origin-selector-support-blocked"
    } else {
        "none"
    };

    let admission_ready = direct_line_mark_origin_admissible;
    let mut blocked_reasons = Vec::new();
    if !admission_ready {
        if !source_layout_candidate_present {
            blocked_reasons.push("source-derived-layout-candidate-absent");
        }
        if !direct_line_mark_page_origin_present {
            blocked_reasons.push("direct-line-mark-page-origin-absent");
        }
        if line_mark_page_origin_stride_present {
            blocked_reasons.push("line-mark-record-stride-to-page-y-transform-unproven");
        }
        if source_layout.is_some_and(|layout| layout.page_origin_authority != "lineMarkPageGrid") {
            blocked_reasons.push("page-origin-authority-not-renderable-line-mark-page-grid");
        }
        if !line_mark_rows_exact_and_contiguous {
            blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
        }
        if cross_table_line_domain_present {
            blocked_reasons.push("cross-table-line-domain-not-page-space-origin");
        }
        if selected_complete {
            blocked_reasons.push("selected-post-row-gap-spans-not-page-y-origin");
            if !selected_ordered_unique_complete {
                blocked_reasons.push("selected-post-row-gap-coverage-not-row-unique");
            }
        }
        if previous_complete {
            blocked_reasons.push("previous-row-span-spans-require-page-origin-transform");
            if !previous_ordered_unique_complete {
                blocked_reasons.push("previous-row-span-coverage-not-row-unique");
            }
        }
        if compact_complete {
            blocked_reasons.push("compact-row-span-spans-do-not-decode-page-y-origin");
        }
        if cross_table_order_regresses {
            blocked_reasons.push("source-order-vs-subrecord-order-contradiction");
        }
        if cross_table_row_boundary_offset_transform_required {
            blocked_reasons.push("cross-table-row-boundary-offset-transform-required");
        }
        if page_mark_absolute_y_slot_blocked_reason != "none" {
            blocked_reasons.push(page_mark_absolute_y_slot_blocked_reason);
        }
        if y_selector_blocked_reason != "none" {
            blocked_reasons.push(y_selector_blocked_reason);
        }
        if source_gap_to_page_line_gap_table_family_transform_required
            && let Some(reason) =
                source_gap_to_page_line_gap_readiness_hints.table_family_transform_blocked_reason()
        {
            blocked_reasons.push(reason);
        }
        if !y_origin_solver_ready {
            blocked_reasons.push("decoded-line-mark-page-y-transform-missing");
        }
    }

    output.push_str(
        "{\"source\":\"sourcePageYTransformGate source-only page-y render admission gate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"admissionReady\":");
    output.push_str(if admission_ready { "true" } else { "false" });
    output.push_str(",\"directLineMarkOriginAdmissible\":");
    output.push_str(if direct_line_mark_origin_admissible {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageOriginAuthority\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.page_origin_authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(if direct_line_mark_page_origin_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkPageOriginStridePresent\":");
    output.push_str(if line_mark_page_origin_stride_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableLineDomainPresent\":");
    output.push_str(if cross_table_line_domain_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyPageYAdmissionClass\":");
    output.push_str(&json_string(source_only_page_y_admission_class));
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if previous_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"compactRowSpanComplete\":");
    output.push_str(if compact_complete { "true" } else { "false" });
    output.push_str(",\"sourceOnlySelectorPresent\":");
    output.push_str(if y_selector_group.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlySelectorSingleSupportFallback\":");
    output.push_str(if y_selector_uses_single_support_fallback {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlySelectorSupportCount\":");
    output.push_str(&y_selector_support_count.to_string());
    output.push_str(",\"sourceOnlySelectorSupportFragmentedByTable\":");
    output.push_str(if y_selector_support_fragmented_by_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlySelectorBlockedReason\":");
    output.push_str(&json_string(y_selector_blocked_reason));
    output.push_str(",\"sourceOnlySelectorSupportBlockedReasons\":");
    push_json_string_slice_array(output, &y_selector_support_blocked_reasons);
    output.push_str(",\"sourceGapToPageLineGapTransformAdmissionGate\":");
    push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
        output,
        "sourceOnlyPageYRenderAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate",
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"sourceGapToPageLineGapTableFamilyTransformRequired\":");
    output.push_str(
        if source_gap_to_page_line_gap_table_family_transform_required {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sourceGapToPageLineGapTableFamilyTransformStable\":");
    output.push_str(
        if source_gap_to_page_line_gap_readiness_hints
            .table_family_source_gap_to_page_line_gap_transform_stable()
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sourceGapToPageLineGapTableFamilyTransformBlockedReason\":");
    match source_gap_to_page_line_gap_readiness_hints.table_family_transform_blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
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
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"source-only-page-y-render-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if admission_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-page-y-render-admission-not-ready"));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_line_domain_post_row_gap_projection_probe_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let line_domain_y = cross_table_row_boundary_offset_probe.and_then(|probe| {
        probe
            .tables
            .iter()
            .find(|table| table.table_candidate_index == candidate.index())
            .and_then(|table| table.line_mark_record_y_tops_px.first().copied())
            .or_else(|| probe.combined_line_mark_record_y_tops_px.first().copied())
    });
    let selected_span_units = subrecord_span_readiness.and_then(|readiness| {
        readiness
            .selected_post_row_gap_span_targets
            .first()
            .copied()
    });
    let projected_y = line_domain_y
        .zip(selected_span_units)
        .map(|(line_domain_y, span_units)| line_domain_y + span_units as f32);
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let reference_layout =
        diagnostic_reference_table_grid_overlay_layout(layout, document, candidate, column_count);
    let reference_top_y = reference_layout.as_ref().map(|layout| layout.y);
    let residual_px = projected_y
        .zip(reference_top_y)
        .map(|(projected_y, reference_top_y)| projected_y - reference_top_y);
    let within_two_px = residual_px.is_some_and(|residual| residual.abs() <= 2.0);
    let selected_complete = subrecord_span_readiness.is_some_and(|readiness| {
        !readiness.selected_post_row_gap_span_targets.is_empty()
            && readiness.selected_post_row_gap_span_hit_count
                == readiness.selected_post_row_gap_span_targets.len()
    });
    let selected_ordered_unique_complete = subrecord_span_readiness.is_some_and(|readiness| {
        readiness
            .selected_post_row_gap_span_coverage
            .ordered_unique_coverage_complete
    });

    let mut source_only_blocked_reasons = Vec::new();
    if line_domain_y.is_none() {
        source_only_blocked_reasons.push("line-domain-y-candidate-absent");
    }
    if selected_span_units.is_none() {
        source_only_blocked_reasons.push("selected-post-row-gap-span-candidate-absent");
    }
    source_only_blocked_reasons.push("cross-domain-source-units-treated-as-px");
    source_only_blocked_reasons.push("selected-spacing-records-are-post-row-gap-family");
    if !selected_complete {
        source_only_blocked_reasons.push("selected-post-row-gap-span-incomplete");
    }
    if !selected_ordered_unique_complete {
        source_only_blocked_reasons.push("selected-post-row-gap-span-not-ordered-unique");
    }
    let mut blocked_reasons = source_only_blocked_reasons.clone();
    if reference_top_y.is_some() {
        blocked_reasons.push("reference-only-validation");
    }
    blocked_reasons.push("page-y-origin-transform-undecoded");
    let mut source_only_projection_blocked_reasons = source_only_blocked_reasons.clone();
    source_only_projection_blocked_reasons.push("page-y-origin-transform-undecoded");

    output.push_str(
        "{\"source\":\"sourcePageYTransformGate line-domain + post-row-gap span projection\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":");
    output.push_str(if reference_top_y.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"projectionKind\":\"line-domain-y-plus-post-row-gap-unit-as-px\"");
    output.push_str(",\"selectionReady\":false,\"promotionReady\":false");
    output.push_str(",\"lineDomainY\":");
    push_optional_f32_json(output, line_domain_y);
    output.push_str(",\"selectedPostRowGapSpanFirstUnits\":");
    push_optional_usize_json(output, selected_span_units);
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"projectedY\":");
    push_optional_f32_json(output, projected_y);
    output.push_str(",\"referenceTableTopY\":");
    push_optional_f32_json(output, reference_top_y);
    output.push_str(",\"residualPx\":");
    push_optional_f32_json(output, residual_px);
    output.push_str(",\"absResidualPx\":");
    push_optional_f32_json(output, residual_px.map(f32::abs));
    output.push_str(",\"withinTwoPx\":");
    output.push_str(if within_two_px { "true" } else { "false" });
    output.push_str(",\"sourceOnlyProjectionDomainGate\":");
    push_table_grid_line_domain_post_row_gap_source_only_projection_domain_gate_json(
        output,
        line_domain_y,
        selected_span_units,
        selected_complete,
        selected_ordered_unique_complete,
        projected_y,
        &source_only_projection_blocked_reasons,
    );
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"line-domain-post-row-gap-projection-probe\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-domain-post-row-gap-projection-crosses-source-unit-domain",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_line_domain_post_row_gap_source_only_projection_domain_gate_json(
    output: &mut String,
    line_domain_y: Option<f32>,
    selected_span_units: Option<usize>,
    selected_complete: bool,
    selected_ordered_unique_complete: bool,
    projected_y: Option<f32>,
    blocked_reasons: &[&str],
) {
    let source_projection_present = line_domain_y.is_some() && selected_span_units.is_some();
    output.push_str("{\"source\":\"sourcePageYTransformGate source-only line-domain/post-row-gap projection domain gate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"sourceProjectionPresent\":");
    output.push_str(if source_projection_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineDomainPresent\":");
    output.push_str(if line_domain_y.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanPresent\":");
    output.push_str(if selected_span_units.is_some() {
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
    output.push_str(
        ",\"sourceUnitDomain\":\"line-mark-record-y-plus-page-mark-subrecord-gap-units\"",
    );
    output.push_str(",\"lineDomainY\":");
    push_optional_f32_json(output, line_domain_y);
    output.push_str(",\"selectedPostRowGapSpanFirstUnits\":");
    push_optional_usize_json(output, selected_span_units);
    output.push_str(",\"projectedY\":");
    push_optional_f32_json(output, projected_y);
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-only-line-domain-post-row-gap-projection-domain-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if blocked_reasons.is_empty() {
        output.push_str("null");
    } else if blocked_reasons.contains(&"cross-domain-source-units-treated-as-px") {
        output.push_str(&json_string(
            "line-domain-post-row-gap-projection-crosses-source-unit-domain",
        ));
    } else {
        output.push_str(&json_string(blocked_reasons[0]));
    }
    output.push('}');
}

pub(crate) fn table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(
    agreement: &TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement,
) -> &'static str {
    if agreement.semantics_ready() {
        "none"
    } else if agreement.best_absolute_y_slot.is_some()
        && agreement.line_domain_projected_y.is_some()
        && !agreement.agrees
    {
        "line-domain-projection-disagrees-with-page-mark-absolute-y-slot"
    } else if agreement.best_absolute_y_slot.is_none() {
        "page-mark-absolute-y-slot-absent"
    } else if agreement.line_domain_projected_y.is_none() {
        "line-domain-plus-span-projection-absent"
    } else {
        "page-mark-absolute-y-slot-semantics-unproven"
    }
}

pub(crate) fn table_grid_source_only_page_mark_absolute_y_slot_agreement(
    document: &Document,
    candidate: &TableCandidate,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
    let line_domain_y = cross_table_row_boundary_offset_probe.and_then(|probe| {
        probe
            .tables
            .iter()
            .find(|table| table.table_candidate_index == candidate.index())
            .and_then(|table| table.line_mark_record_y_tops_px.first().copied())
            .or_else(|| probe.combined_line_mark_record_y_tops_px.first().copied())
    });
    let selected_span_units = subrecord_span_readiness.and_then(|readiness| {
        readiness
            .selected_post_row_gap_span_targets
            .first()
            .copied()
    });
    let line_domain_projected_y = line_domain_y
        .zip(selected_span_units)
        .map(|(line_domain_y, span_units)| line_domain_y + span_units as f32);
    let candidates = table_grid_source_only_page_mark_absolute_y_slot_candidates(
        document,
        subrecord_span_readiness,
    );
    let best_absolute_y_slot = candidates
        .iter()
        .min_by(|left, right| {
            let left_residual = line_domain_projected_y
                .map(|projected_y| (left.value_px - projected_y).abs())
                .unwrap_or(0.0);
            let right_residual = line_domain_projected_y
                .map(|projected_y| (right.value_px - projected_y).abs())
                .unwrap_or(0.0);
            option_f32_order(Some(left_residual), Some(right_residual))
                .then_with(|| left.byte_offset.cmp(&right.byte_offset))
        })
        .cloned();
    let absolute_y_slot_y = best_absolute_y_slot.as_ref().map(|slot| slot.value_px);
    let residual_px = line_domain_projected_y
        .zip(absolute_y_slot_y)
        .map(|(projected_y, absolute_y)| projected_y - absolute_y);
    let agrees = residual_px.is_some_and(|residual| residual.abs() <= 2.0);

    TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
        line_domain_y,
        selected_span_units,
        line_domain_projected_y,
        candidates,
        best_absolute_y_slot,
        residual_px,
        agrees,
    }
}

pub(crate) fn push_table_grid_source_only_page_mark_absolute_y_slot_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let agreement = table_grid_source_only_page_mark_absolute_y_slot_agreement(
        document,
        candidate,
        cross_table_row_boundary_offset_probe,
        subrecord_span_readiness,
    );
    let absolute_y_slot_y = agreement
        .best_absolute_y_slot
        .as_ref()
        .map(|slot| slot.value_px);
    let lineage_class = if absolute_y_slot_y.is_some() {
        "page-mark-absolute-y-slot"
    } else if agreement.line_domain_projected_y.is_some() {
        "line-domain-plus-span-projection"
    } else {
        "no-source-absolute-y-slot"
    };

    let mut blocked_reasons = Vec::new();
    if agreement.best_absolute_y_slot.is_none() {
        blocked_reasons.push("page-mark-absolute-y-slot-absent");
    }
    if agreement.line_domain_projected_y.is_none() {
        blocked_reasons.push("line-domain-plus-span-projection-absent");
    }
    if agreement.best_absolute_y_slot.is_some()
        && agreement.line_domain_projected_y.is_some()
        && !agreement.agrees
    {
        blocked_reasons.push("line-domain-projection-disagrees-with-page-mark-absolute-y-slot");
    }
    if !agreement.semantics_ready() {
        blocked_reasons.push("page-mark-absolute-y-slot-semantics-unproven");
    }
    blocked_reasons.push("page-y-origin-transform-undecoded");

    output.push_str(
        "{\"source\":\"/PageMark raw u16 subrecord scan+/LineMark source page-line projection\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(
        ",\"projectionKind\":\"line-domain-y-plus-post-row-gap-vs-page-mark-absolute-y-slot\"",
    );
    output.push_str(",\"lineDomainY\":");
    push_optional_f32_json(output, agreement.line_domain_y);
    output.push_str(",\"selectedPostRowGapSpanFirstUnits\":");
    push_optional_usize_json(output, agreement.selected_span_units);
    output.push_str(",\"lineDomainProjectedY\":");
    push_optional_f32_json(output, agreement.line_domain_projected_y);
    output.push_str(",\"absoluteYSlotPresent\":");
    output.push_str(if absolute_y_slot_y.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bestAbsoluteYSlot\":");
    match agreement.best_absolute_y_slot.as_ref() {
        Some(slot) => push_table_grid_source_only_page_mark_absolute_y_slot_candidate_json(
            output,
            slot,
            Some(&agreement.candidates),
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"absoluteYSlotY\":");
    push_optional_f32_json(output, absolute_y_slot_y);
    output.push_str(",\"lineDomainProjectionVsAbsoluteYSlotResidualPx\":");
    push_optional_f32_json(output, agreement.residual_px);
    output.push_str(",\"lineDomainProjectionAgreesWithAbsoluteYSlot\":");
    output.push_str(if agreement.agrees { "true" } else { "false" });
    output.push_str(",\"lineageClass\":");
    output.push_str(&json_string(lineage_class));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-only-page-mark-absolute-y-slot-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if blocked_reasons.is_empty() {
        output.push_str("null");
    } else {
        output.push_str(&json_string(blocked_reasons[0]));
    }
    output.push('}');
}

pub(crate) fn table_grid_source_only_page_mark_absolute_y_slot_candidates(
    document: &Document,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> Vec<TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate> {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        return Vec::new();
    };
    let Some(readiness) = subrecord_span_readiness else {
        return Vec::new();
    };
    let record_headers = page_mark_record_headers(page_mark_bytes);
    let mut candidates = Vec::new();
    let mut seen = BTreeSet::new();
    for subrecord_byte_offset in readiness
        .selected_post_row_gap_span_coverage
        .unique_candidate_byte_offsets
        .iter()
        .copied()
    {
        if !seen.insert(subrecord_byte_offset) {
            continue;
        }
        let Some(subrecord) =
            page_mark_raw_u16_subrecord_candidate_at(page_mark_bytes, subrecord_byte_offset)
        else {
            continue;
        };
        let field_index = 2usize;
        let byte_offset = subrecord.byte_offset + field_index * 2;
        let Some((raw_record_scan_index, raw_record_index, tail_block16_word_index)) =
            page_mark_raw_subrecord_record_context(&record_headers, byte_offset)
        else {
            continue;
        };
        candidates.push(TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate {
            source: "rawRecordHeaderTailU16Subrecord",
            interpretation: "direct-u16-px",
            field_index,
            tail_block16_word_index: Some(tail_block16_word_index),
            raw_record_scan_index: Some(raw_record_scan_index),
            raw_record_index: Some(raw_record_index),
            byte_offset,
            subrecord_byte_offset,
            subrecord_line_start_candidate: subrecord.words[4],
            subrecord_line_end_candidate: subrecord.words[6],
            value: subrecord.words[field_index],
            value_px: f32::from(subrecord.words[field_index]),
        });
    }
    candidates.sort_by(|left, right| {
        left.byte_offset
            .cmp(&right.byte_offset)
            .then_with(|| left.value.cmp(&right.value))
    });
    candidates
}

pub(crate) fn push_table_grid_source_only_page_mark_absolute_y_slot_candidate_json(
    output: &mut String,
    candidate: &TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate,
    all_candidates: Option<&[TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate]>,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(candidate.source));
    output.push_str(",\"interpretation\":");
    output.push_str(&json_string(candidate.interpretation));
    output.push_str(",\"fieldIndex\":");
    output.push_str(&candidate.field_index.to_string());
    output.push_str(",\"tailBlock16WordIndex\":");
    push_option_usize_json(output, candidate.tail_block16_word_index);
    output.push_str(",\"rawRecordScanIndexes\":");
    let raw_record_scan_indexes = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .filter_map(|candidate| candidate.raw_record_scan_index)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_usize_array_json(output, &raw_record_scan_indexes);
    output.push_str(",\"rawRecordIndexes\":");
    let raw_record_indexes = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .filter_map(|candidate| candidate.raw_record_index)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_u32_array_json(output, &raw_record_indexes);
    output.push_str(",\"byteOffsets\":");
    let byte_offsets = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .map(|candidate| candidate.byte_offset)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_usize_array_json(output, &byte_offsets);
    output.push_str(",\"subrecordByteOffsets\":");
    let subrecord_byte_offsets = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .map(|candidate| candidate.subrecord_byte_offset)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_usize_array_json(output, &subrecord_byte_offsets);
    output.push_str(",\"subrecordLineStartCandidates\":");
    let line_start_candidates = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .map(|candidate| candidate.subrecord_line_start_candidate)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_u16_array_json(output, &line_start_candidates);
    output.push_str(",\"subrecordLineEndCandidates\":");
    let line_end_candidates = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .map(|candidate| candidate.subrecord_line_end_candidate)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_u16_array_json(output, &line_end_candidates);
    output.push_str(",\"value\":");
    output.push_str(&candidate.value.to_string());
    output.push_str(",\"valuePx\":");
    output.push_str(&format!("{:.3}", candidate.value_px));
    output.push('}');
}

pub(crate) fn table_grid_source_gap_to_page_line_gap_readiness_hints(
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
) -> TableGridSourceGapToPageLineGapReadinessHints {
    let source_range_gap_units = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.source_range_gap_units)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let row_source_start_gap_units = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.row_source_start_gap_units)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let previous_family_record_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.line_mark_record_gap)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let segment_offset_gap_units = row_source_start_gap_units
        .iter()
        .copied()
        .zip(source_range_gap_units.iter().copied())
        .map(|(row_source_start_gap, source_range_gap)| {
            row_source_start_gap_minus_source_range_gap_units(
                row_source_start_gap,
                source_range_gap,
            )
        })
        .collect::<Vec<_>>();
    let source_range_gap_minus_page_line_gap_units = source_range_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(source_range_gap, page_line_gap)| {
            source_range_gap_minus_page_line_gap_units(source_range_gap, page_line_gap)
        })
        .collect::<Vec<_>>();
    let row_source_start_gap_minus_page_line_gap_units = row_source_start_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(row_source_start_gap, page_line_gap)| {
            row_source_start_gap.saturating_sub(page_line_gap)
        })
        .collect::<Vec<_>>();
    let segment_offset_gap_minus_page_line_gap_units = segment_offset_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(segment_offset_gap, page_line_gap)| segment_offset_gap.saturating_sub(page_line_gap))
        .collect::<Vec<_>>();
    let source_range_units_per_page_line_gap =
        ratio_usize_by_i32(&source_range_gap_units, &previous_family_record_gaps);
    let row_source_start_units_per_page_line_gap =
        ratio_i32_by_i32(&row_source_start_gap_units, &previous_family_record_gaps);
    let segment_offset_units_per_page_line_gap =
        ratio_i32_by_i32(&segment_offset_gap_units, &previous_family_record_gaps);
    let same_page_mark_entry_transition_count = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .filter(|transition| transition.same_page_mark_entry)
                .count()
        })
        .unwrap_or(0);
    let transition_count = previous_family_record_gaps.len();
    let all_transitions_same_page_mark_entry =
        transition_count > 0 && same_page_mark_entry_transition_count == transition_count;
    let source_range_gap_to_page_line_gap_max_abs_delta_units =
        max_abs_i32(&source_range_gap_minus_page_line_gap_units);
    let row_source_start_gap_to_page_line_gap_max_abs_delta_units =
        max_abs_i32(&row_source_start_gap_minus_page_line_gap_units);
    let segment_offset_gap_to_page_line_gap_max_abs_delta_units =
        max_abs_i32(&segment_offset_gap_minus_page_line_gap_units);
    let best_candidate = [
        (
            "direct-source-range-gap",
            source_range_gap_to_page_line_gap_max_abs_delta_units,
        ),
        (
            "direct-row-source-start-gap",
            row_source_start_gap_to_page_line_gap_max_abs_delta_units,
        ),
        (
            "segment-offset-gap",
            segment_offset_gap_to_page_line_gap_max_abs_delta_units,
        ),
    ]
    .into_iter()
    .filter_map(|(kind, max_abs_delta)| max_abs_delta.map(|delta| (kind, delta)))
    .min_by_key(|(_, delta)| *delta);
    let affine_row_source_start_gap_fit = affine_row_source_start_gap_fit(
        &previous_family_record_gaps,
        &row_source_start_gap_units,
        all_transitions_same_page_mark_entry,
    );

    TableGridSourceGapToPageLineGapReadinessHints {
        transition_count,
        same_page_mark_entry_transition_count,
        all_transitions_same_page_mark_entry,
        source_range_gap_to_page_line_gap_max_abs_delta_units,
        row_source_start_gap_to_page_line_gap_max_abs_delta_units,
        segment_offset_gap_to_page_line_gap_max_abs_delta_units,
        best_candidate_transform_kind: best_candidate.map(|(kind, _)| kind),
        best_candidate_max_abs_delta_units: best_candidate.map(|(_, delta)| delta),
        source_range_units_per_page_line_gap_spread: f32_value_spread(
            &source_range_units_per_page_line_gap,
        ),
        row_source_start_units_per_page_line_gap_spread: f32_value_spread(
            &row_source_start_units_per_page_line_gap,
        ),
        segment_offset_units_per_page_line_gap_spread: f32_value_spread(
            &segment_offset_units_per_page_line_gap,
        ),
        affine_row_source_start_gap_fit,
    }
}

pub(crate) fn push_table_grid_source_gap_to_page_line_gap_readiness_hints_json(
    output: &mut String,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) {
    output.push_str("{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapReadinessHints\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"transitionCount\":");
    output.push_str(&hints.transition_count.to_string());
    output.push_str(",\"samePageMarkEntryTransitionCount\":");
    output.push_str(&hints.same_page_mark_entry_transition_count.to_string());
    output.push_str(",\"allTransitionsSamePageMarkEntry\":");
    output.push_str(if hints.all_transitions_same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRangeGapToPageLineGapMaxAbsDeltaUnits\":");
    push_optional_i32_json(
        output,
        hints.source_range_gap_to_page_line_gap_max_abs_delta_units,
    );
    output.push_str(",\"rowSourceStartGapToPageLineGapMaxAbsDeltaUnits\":");
    push_optional_i32_json(
        output,
        hints.row_source_start_gap_to_page_line_gap_max_abs_delta_units,
    );
    output.push_str(",\"segmentOffsetGapToPageLineGapMaxAbsDeltaUnits\":");
    push_optional_i32_json(
        output,
        hints.segment_offset_gap_to_page_line_gap_max_abs_delta_units,
    );
    output.push_str(",\"bestCandidateTransformKind\":");
    match hints.best_candidate_transform_kind {
        Some(kind) => output.push_str(&json_string(kind)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestCandidateMaxAbsDeltaUnits\":");
    push_optional_i32_json(output, hints.best_candidate_max_abs_delta_units);
    output.push_str(",\"transformCandidateCount\":");
    output.push_str(&hints.transform_candidate_count().to_string());
    output.push_str(",\"exactTransformCandidateCount\":");
    output.push_str(&hints.exact_transform_candidate_count().to_string());
    output.push_str(",\"bestCandidateTransitionCoverageCount\":");
    output.push_str(&hints.best_candidate_transition_coverage_count().to_string());
    output.push_str(",\"bestCandidateUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(
        output,
        hints.best_candidate_units_per_page_line_gap_spread(),
    );
    let lowest_spread_candidate = hints.lowest_spread_candidate();
    output.push_str(",\"lowestSpreadCandidateTransformKind\":");
    if let Some((kind, _)) = lowest_spread_candidate {
        output.push_str(&json_string(kind));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"lowestSpreadUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, lowest_spread_candidate.map(|(_, spread)| spread));
    output.push_str(",\"transformCandidateSummaries\":");
    push_table_grid_source_gap_to_page_line_gap_transform_candidate_summaries_json(output, hints);
    output.push_str(",\"declinedTransformCandidates\":");
    push_table_grid_source_gap_to_page_line_gap_declined_transform_candidates_json(output, hints);
    output.push_str(",\"affineRowSourceStartGapFit\":");
    push_affine_row_source_start_gap_fit_json(output, hints.affine_row_source_start_gap_fit);
    output.push_str(",\"sourceRangeUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, hints.source_range_units_per_page_line_gap_spread);
    output.push_str(",\"rowSourceStartUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(
        output,
        hints.row_source_start_units_per_page_line_gap_spread,
    );
    output.push_str(",\"segmentOffsetUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, hints.segment_offset_units_per_page_line_gap_spread);
    output.push_str(",\"sourceGapToPageLineGapTransformStable\":");
    output.push_str(if hints.source_gap_to_page_line_gap_transform_stable() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"tableFamilySourceGapToPageLineGapTransformStable\":");
    output.push_str(
        if hints.table_family_source_gap_to_page_line_gap_transform_stable() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"tableFamilyTransformBlockedReason\":");
    match hints.table_family_transform_blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    match hints.transform_blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(crate) fn push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
    output: &mut String,
    source: &'static str,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) {
    let transform_ready = hints.source_gap_to_page_line_gap_transform_stable();
    let table_family_transform_ready =
        hints.table_family_source_gap_to_page_line_gap_transform_stable();
    let mut declared_blockers = Vec::new();
    if hints.transition_count == 0 {
        declared_blockers.push("source-gap-to-page-line-gap-transform-evidence-absent");
    }
    if !transform_ready {
        declared_blockers.push("source-gap-to-page-line-gap-transform-not-stable");
    }
    if let Some(reason) = hints.table_family_transform_blocked_reason()
        && !declared_blockers.contains(&reason)
    {
        declared_blockers.push(reason);
    }
    if !transform_ready {
        declared_blockers.push("source-gap-to-page-line-gap-transform-undecoded");
    }

    output.push_str("{\"source\":");
    output.push_str(&json_string(source));
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"transformDomain\":");
    output.push_str(&json_string("source-unit-gap-to-page-mark-line-index-gap"));
    output.push_str(",\"canDecodeSourceTransform\":");
    output.push_str(if transform_ready { "true" } else { "false" });
    output.push_str(",\"tableFamilyTransformStable\":");
    output.push_str(if table_family_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"tableFamilyTransformBlockedReason\":");
    match hints.table_family_transform_blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"transitionCount\":");
    output.push_str(&hints.transition_count.to_string());
    output.push_str(",\"allTransitionsSamePageMarkEntry\":");
    output.push_str(if hints.all_transitions_same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bestCandidateTransformKind\":");
    match hints.best_candidate_transform_kind {
        Some(kind) => output.push_str(&json_string(kind)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestCandidateMaxAbsDeltaUnits\":");
    push_optional_i32_json(output, hints.best_candidate_max_abs_delta_units);
    output.push_str(",\"transformCandidateCount\":");
    output.push_str(&hints.transform_candidate_count().to_string());
    output.push_str(",\"exactTransformCandidateCount\":");
    output.push_str(&hints.exact_transform_candidate_count().to_string());
    output.push_str(",\"bestCandidateTransitionCoverageCount\":");
    output.push_str(&hints.best_candidate_transition_coverage_count().to_string());
    output.push_str(",\"bestCandidateUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(
        output,
        hints.best_candidate_units_per_page_line_gap_spread(),
    );
    let lowest_spread_candidate = hints.lowest_spread_candidate();
    output.push_str(",\"lowestSpreadCandidateTransformKind\":");
    if let Some((kind, _)) = lowest_spread_candidate {
        output.push_str(&json_string(kind));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"lowestSpreadUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, lowest_spread_candidate.map(|(_, spread)| spread));
    output.push_str(",\"declinedTransformCandidates\":");
    push_table_grid_source_gap_to_page_line_gap_declined_transform_candidates_json(output, hints);
    output.push_str(",\"affineRowSourceStartGapFit\":");
    push_affine_row_source_start_gap_fit_json(output, hints.affine_row_source_start_gap_fit);
    output.push_str(",\"declaredBlockers\":");
    push_json_string_slice_array(output, &declared_blockers);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-transform-admission-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if transform_ready {
        output.push_str("null");
    } else if hints.transition_count == 0 {
        output.push_str(&json_string(
            "source-gap-to-page-line-gap-transform-evidence-absent",
        ));
    } else {
        output.push_str(&json_string(
            "source-gap-to-page-line-gap-transform-not-stable",
        ));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_source_gap_to_page_line_gap_transform_candidate_summaries_json(
    output: &mut String,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) {
    output.push('[');
    for (index, candidate) in hints.transform_candidate_summaries().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_source_gap_to_page_line_gap_transform_candidate_summary_json(
            output, hints, candidate,
        );
    }
    output.push(']');
}

pub(crate) fn push_table_grid_source_gap_to_page_line_gap_declined_transform_candidates_json(
    output: &mut String,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) {
    output.push('[');
    let mut first = true;
    for candidate in hints.transform_candidate_summaries() {
        if table_grid_source_gap_to_page_line_gap_decline_reason(&candidate, hints).is_none() {
            continue;
        }
        if !first {
            output.push(',');
        }
        first = false;
        push_table_grid_source_gap_to_page_line_gap_transform_candidate_summary_json(
            output, hints, &candidate,
        );
    }
    output.push(']');
}

pub(crate) fn push_table_grid_source_gap_to_page_line_gap_transform_candidate_summary_json(
    output: &mut String,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
    candidate: &TableGridSourceGapToPageLineGapTransformCandidateSummary,
) {
    let selected = hints.best_candidate_transform_kind == Some(candidate.kind);
    let stable = candidate
        .affine_row_source_start_gap_fit
        .map(|fit| fit.fit_stable)
        .unwrap_or(candidate.max_abs_delta_units == Some(0));
    output.push_str("{\"transformKind\":");
    output.push_str(&json_string(candidate.kind));
    output.push_str(",\"selected\":");
    output.push_str(if selected { "true" } else { "false" });
    output.push_str(",\"stable\":");
    output.push_str(if stable { "true" } else { "false" });
    output.push_str(",\"transitionCoverageCount\":");
    if candidate.max_abs_delta_units.is_some() {
        output.push_str(&hints.transition_count.to_string());
    } else {
        output.push('0');
    }
    output.push_str(",\"maxAbsDeltaUnits\":");
    push_optional_i32_json(output, candidate.max_abs_delta_units);
    output.push_str(",\"unitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, candidate.units_per_page_line_gap_spread);
    if let Some(fit) = candidate.affine_row_source_start_gap_fit {
        output.push_str(",\"affineRowSourceStartGapFit\":");
        push_affine_row_source_start_gap_fit_json(output, Some(fit));
    }
    output.push_str(",\"declineReason\":");
    if let Some(reason) = table_grid_source_gap_to_page_line_gap_decline_reason(candidate, hints) {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    if let Some(reason) = table_grid_source_gap_to_page_line_gap_candidate_blocked_reason(candidate)
    {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn table_grid_source_gap_to_page_line_gap_decline_reason(
    candidate: &TableGridSourceGapToPageLineGapTransformCandidateSummary,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) -> Option<&'static str> {
    if let Some(fit) = candidate.affine_row_source_start_gap_fit {
        return Some(fit.blocked_reason());
    }
    if hints.best_candidate_transform_kind == Some(candidate.kind) {
        return None;
    }
    let Some(candidate_delta) = candidate.max_abs_delta_units else {
        return Some("transform-candidate-evidence-absent");
    };
    let Some(best_delta) = hints.best_candidate_max_abs_delta_units else {
        return Some("transform-candidate-not-selected-without-best-transform");
    };
    if candidate_delta > best_delta {
        Some("higher-max-delta-than-selected-transform")
    } else if candidate_delta == best_delta {
        Some("tie-not-selected-by-candidate-order")
    } else {
        Some("transform-candidate-not-selected")
    }
}

pub(crate) fn table_grid_source_gap_to_page_line_gap_candidate_blocked_reason(
    candidate: &TableGridSourceGapToPageLineGapTransformCandidateSummary,
) -> Option<&'static str> {
    if let Some(fit) = candidate.affine_row_source_start_gap_fit {
        Some(fit.blocked_reason())
    } else if candidate.max_abs_delta_units.is_none() {
        Some("transform-candidate-evidence-absent")
    } else if candidate.max_abs_delta_units != Some(0) {
        Some("source-gap-to-page-line-gap-transform-not-stable")
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_source_only_page_y_origin_hypothesis_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
    cross_table_ordering_probe: Option<&TableGridCrossTableSubrecordOrderingProbe>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    selected_complete: bool,
    selected_ordered_unique_complete: bool,
    previous_complete: bool,
    previous_ordered_unique_complete: bool,
    compact_complete: bool,
    y_origin_solver_ready: bool,
    line_mark_rows_exact_and_contiguous: bool,
) {
    let line_mark_page_origin_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_page_origin.is_some());
    let line_mark_page_origin_stride_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_page_origin_stride.is_some());
    let cross_table_line_domain_candidate_present = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| {
            probe.all_records_within_single_page_mark_entry
                && !probe.combined_line_mark_record_y_tops_px.is_empty()
        });
    let page_mark_absolute_y_slot_agreement = if source_layout.is_some() {
        Some(table_grid_source_only_page_mark_absolute_y_slot_agreement(
            document,
            candidate,
            cross_table_row_boundary_offset_probe,
            subrecord_span_readiness,
        ))
    } else {
        None
    };
    let page_mark_absolute_y_slot = page_mark_absolute_y_slot_agreement
        .as_ref()
        .and_then(|agreement| agreement.best_absolute_y_slot.as_ref());
    let page_mark_absolute_y_slot_present = page_mark_absolute_y_slot.is_some();
    let candidate_present = line_mark_page_origin_present
        || line_mark_page_origin_stride_present
        || cross_table_line_domain_candidate_present;
    let candidate_kind = if line_mark_page_origin_present {
        Some("line-mark-page-origin")
    } else if line_mark_page_origin_stride_present {
        Some("line-mark-page-origin-stride")
    } else if cross_table_line_domain_candidate_present {
        Some("cross-table-page-line-domain")
    } else {
        None
    };
    let cross_table_ordering_consistent =
        cross_table_ordering_probe.is_some_and(|probe| probe.cross_table_ordering_consistent);
    let cross_table_order_regresses = cross_table_ordering_probe.is_some_and(|probe| {
        !probe.monotonic_raw_record_scan_index
            || !probe.monotonic_line_start_candidate
            || probe.family_reused_after_later_family
    });
    let cross_table_offsets_stable =
        cross_table_row_boundary_offset_probe.is_some_and(|probe| probe.all_offsets_stable);
    let cross_table_offsets_require_transform = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| probe.all_offsets_require_transform);
    let piecewise_all_tables_exact = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| probe.source_unit_to_page_line_index_piecewise_all_tables_exact);
    let cross_table_previous_row_span_support_count =
        table_grid_cross_table_previous_row_span_y_origin_support_count(
            cross_table_row_boundary_offset_probe,
        );
    let cross_table_previous_row_span_selector_present =
        cross_table_previous_row_span_support_count > 0;
    let cross_table_previous_row_span_selection_ready =
        cross_table_previous_row_span_selector_present
            && previous_complete
            && previous_ordered_unique_complete
            && cross_table_offsets_stable
            && !cross_table_offsets_require_transform
            && piecewise_all_tables_exact
            && cross_table_ordering_consistent
            && !cross_table_order_regresses
            && y_origin_solver_ready;
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe,
        );

    let mut blocked_reasons = Vec::new();
    if source_layout.is_none() && cross_table_row_boundary_offset_probe.is_none() {
        blocked_reasons.push("source-y-origin-evidence-absent");
    }
    if !candidate_present {
        blocked_reasons.push("source-page-y-origin-candidate-absent");
    }
    if line_mark_page_origin_stride_present && !line_mark_page_origin_present {
        blocked_reasons.push("stride-origin-needs-page-origin-rule");
    }
    match source_layout {
        Some(layout) => {
            if layout.render_promotion_blocked_reason != "none" {
                blocked_reasons.push(layout.render_promotion_blocked_reason);
            }
            if !layout.line_header_rows_homogeneous {
                blocked_reasons.push("line-header-rows-not-homogeneous");
            }
            if !layout.line_mark_rows_exact_and_contiguous {
                blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
            }
            if layout.page_origin_authority != "lineMarkPageGrid" {
                blocked_reasons.push("page-origin-authority-not-renderable-line-mark-page-grid");
            }
        }
        None => blocked_reasons.push("source-derived-layout-candidate-absent"),
    }
    if !line_mark_rows_exact_and_contiguous {
        blocked_reasons.push("gate-line-mark-rows-not-exact-source-boundaries");
    }
    if subrecord_span_readiness.is_none() {
        blocked_reasons.push("page-mark-subrecord-line-span-readiness-absent");
    }
    if selected_complete && !selected_ordered_unique_complete {
        blocked_reasons.push("selected-post-row-gap-subrecord-coverage-not-ordered-unique");
    }
    if previous_complete && !previous_ordered_unique_complete {
        blocked_reasons.push("previous-row-span-subrecord-coverage-not-ordered-unique");
    }
    if compact_complete {
        blocked_reasons.push("compact-row-span-subrecord-spans-do-not-decode-origin");
    }
    if cross_table_ordering_probe.is_some() && !cross_table_ordering_consistent {
        blocked_reasons.push("cross-table-subrecord-ordering-inconsistent");
    }
    if cross_table_order_regresses {
        blocked_reasons.push("source-order-vs-subrecord-order-contradiction");
    }
    if cross_table_offsets_require_transform {
        blocked_reasons.push("cross-table-row-boundary-offset-transform-required");
    }
    if !piecewise_all_tables_exact {
        blocked_reasons.push("source-unit-to-page-line-piecewise-fit-not-exact");
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("decoded-page-y-origin-missing");
    }
    let y_origin_readiness_class = if line_mark_page_origin_present {
        "direct-line-mark-origin"
    } else if line_mark_page_origin_stride_present {
        "stride-only"
    } else if cross_table_line_domain_candidate_present {
        "cross-table-line-domain-only"
    } else {
        "none"
    };
    let origin_decision_ready = y_origin_solver_ready && line_mark_page_origin_present;
    let mut y_origin_readiness_blocked_reasons = Vec::new();
    match y_origin_readiness_class {
        "direct-line-mark-origin" => {
            if !origin_decision_ready {
                y_origin_readiness_blocked_reasons.push("direct-line-mark-origin-not-promotable");
            }
        }
        "stride-only" => {
            y_origin_readiness_blocked_reasons.push("line-mark-page-origin-stride-present");
            y_origin_readiness_blocked_reasons.push("stride-origin-needs-direct-line-origin-rule");
            if !line_mark_page_origin_present {
                y_origin_readiness_blocked_reasons.push("direct-line-mark-page-origin-absent");
            }
            if !y_origin_solver_ready {
                y_origin_readiness_blocked_reasons.push("decoded-page-y-origin-missing");
            }
        }
        "cross-table-line-domain-only" => {
            y_origin_readiness_blocked_reasons.push("cross-table-line-domain-present");
            if cross_table_offsets_require_transform {
                y_origin_readiness_blocked_reasons
                    .push("line-domain-to-page-space-origin-transform-required");
            }
            if !piecewise_all_tables_exact {
                y_origin_readiness_blocked_reasons
                    .push("source-unit-to-page-line-piecewise-fit-not-exact");
            }
            if let Some(reason) =
                source_gap_to_page_line_gap_readiness_hints.transform_blocked_reason()
            {
                y_origin_readiness_blocked_reasons.push(reason);
            }
            if cross_table_order_regresses {
                y_origin_readiness_blocked_reasons
                    .push("source-order-vs-subrecord-order-contradiction");
            }
            if !y_origin_solver_ready {
                y_origin_readiness_blocked_reasons.push("decoded-page-y-origin-missing");
            }
        }
        _ => {
            y_origin_readiness_blocked_reasons.push("source-page-y-origin-candidate-absent");
        }
    }

    output
        .push_str("{\"source\":\"sourcePageYTransformGate source-only page-y origin hypothesis\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"candidatePresent\":");
    output.push_str(if candidate_present { "true" } else { "false" });
    output.push_str(",\"candidateKind\":");
    match candidate_kind {
        Some(kind) => output.push_str(&json_string(kind)),
        None => output.push_str("null"),
    }
    output.push_str(",\"yOriginReadinessClass\":");
    output.push_str(&json_string(y_origin_readiness_class));
    output.push_str(",\"originDecisionReady\":");
    output.push_str(if origin_decision_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"yOriginReadinessBlockedReasons\":");
    push_json_string_slice_array(output, &y_origin_readiness_blocked_reasons);
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(if line_mark_page_origin_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkPageOriginStridePresent\":");
    output.push_str(if line_mark_page_origin_stride_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageMarkAbsoluteYSlotCandidatePresent\":");
    output.push_str(if page_mark_absolute_y_slot_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageMarkAbsoluteYSlotY\":");
    push_optional_f32_json(output, page_mark_absolute_y_slot.map(|slot| slot.value_px));
    output.push_str(",\"pageMarkAbsoluteYSlotBlockedReason\":");
    if page_mark_absolute_y_slot_agreement
        .as_ref()
        .is_some_and(TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement::semantics_ready)
    {
        output.push_str("null");
    } else if page_mark_absolute_y_slot_present {
        output.push_str(&json_string("page-mark-absolute-y-slot-semantics-unproven"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pageOriginAuthority\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.page_origin_authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceGapToPageLineGapReadinessHints\":");
    push_table_grid_source_gap_to_page_line_gap_readiness_hints_json(
        output,
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"sourceLayoutRenderable\":");
    output.push_str(
        if source_layout.is_some_and(table_grid_source_derived_layout_is_renderable) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(
        if source_layout.is_some_and(|layout| layout.line_header_rows_homogeneous) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"directLineMarkPageOrigin\":");
    match source_layout.and_then(|layout| layout.line_mark_page_origin.as_ref()) {
        Some(origin) => {
            output.push_str("{\"y\":");
            output.push_str(&format!("{:.3}", origin.y));
            output.push_str(",\"firstLineMarkRecordIndex\":");
            output.push_str(&origin.first_line_mark_record_index.to_string());
            output.push_str(",\"lastLineMarkRecordIndex\":");
            output.push_str(&origin.last_line_mark_record_index.to_string());
            output.push_str(",\"pageMarkEntryIndex\":");
            output.push_str(&origin.page_mark_entry_index.to_string());
            output.push_str(",\"pageLineStart\":");
            output.push_str(&origin.page_line_start.to_string());
            output.push_str(",\"pageLineEnd\":");
            output.push_str(&origin.page_line_end.to_string());
            output.push_str(",\"lineOffsetFromPageStart\":");
            output.push_str(&origin.line_offset_from_page_start.to_string());
            output.push_str(",\"linePitchPx\":");
            output.push_str(&format!("{:.3}", origin.line_pitch_px));
            output.push_str(",\"linePitchBasis\":");
            output.push_str(&json_string(origin.line_pitch_basis));
            output.push_str(",\"rowHeight\":");
            output.push_str(&format!("{:.3}", origin.row_height));
            output.push('}');
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"strideLineMarkPageOrigin\":");
    match source_layout.and_then(|layout| layout.line_mark_page_origin_stride.as_ref()) {
        Some(stride) => {
            output.push_str("{\"lineMarkRecordIndexes\":");
            push_usize_array_json(output, &stride.line_mark_record_indexes);
            output.push_str(",\"recordStride\":");
            output.push_str(&stride.record_stride.to_string());
            output.push_str(",\"firstLineMarkRecordIndex\":");
            output.push_str(&stride.first_line_mark_record_index.to_string());
            output.push_str(",\"lastLineMarkRecordIndex\":");
            output.push_str(&stride.last_line_mark_record_index.to_string());
            output.push_str(",\"pageMarkEntryIndex\":");
            output.push_str(&stride.page_mark_entry_index.to_string());
            output.push_str(",\"pageLineStart\":");
            output.push_str(&stride.page_line_start.to_string());
            output.push_str(",\"pageLineEnd\":");
            output.push_str(&stride.page_line_end.to_string());
            output.push_str(",\"lineOffsetFromPageStart\":");
            output.push_str(&stride.line_offset_from_page_start.to_string());
            output.push_str(",\"rowHeight\":");
            output.push_str(&format!("{:.3}", stride.row_height));
            output.push_str(",\"rawRecordIndexRowTops\":");
            push_f32_array_json(output, &stride.raw_record_index_row_tops);
            output.push_str(",\"strideCollapsedRowTops\":");
            push_f32_array_json(output, &stride.stride_collapsed_row_tops);
            output.push('}');
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"pageMarkAbsoluteYSlotOrigin\":");
    match page_mark_absolute_y_slot {
        Some(slot) => push_table_grid_source_only_page_mark_absolute_y_slot_candidate_json(
            output,
            slot,
            page_mark_absolute_y_slot_agreement
                .as_ref()
                .map(|agreement| agreement.candidates.as_slice()),
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"crossTableLineDomainEvidence\":");
    match cross_table_row_boundary_offset_probe {
        Some(probe) => {
            output.push_str("{\"present\":true,\"allRecordsWithinSinglePageMarkEntry\":");
            output.push_str(if probe.all_records_within_single_page_mark_entry {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"allOffsetsStable\":");
            output.push_str(if cross_table_offsets_stable {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"allOffsetsRequireTransform\":");
            output.push_str(if cross_table_offsets_require_transform {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"stableRowBoundaryOffsetCandidateUnits\":");
            push_optional_i32_json(output, probe.stable_row_boundary_offset_candidate_units);
            output.push_str(",\"piecewiseAllTablesExact\":");
            output.push_str(if piecewise_all_tables_exact {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"piecewiseMaxAbsResidualRecordIndexes\":");
            push_optional_f32_json(
                output,
                probe.source_unit_to_page_line_index_piecewise_max_abs_residual,
            );
            output.push_str(",\"combinedLineMarkRecordIndexes\":");
            push_usize_array_json(output, &probe.combined_line_mark_record_indexes);
            output.push_str(",\"combinedLineMarkRecordYTopPx\":");
            push_f32_array_json(output, &probe.combined_line_mark_record_y_tops_px);
            output.push('}');
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"crossTablePreviousRowSpanSelectorPresent\":");
    output.push_str(if cross_table_previous_row_span_selector_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanSupportCount\":");
    output.push_str(&cross_table_previous_row_span_support_count.to_string());
    output.push_str(",\"crossTablePreviousRowSpanSelectionReady\":");
    output.push_str(if cross_table_previous_row_span_selection_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanReadinessInputs\":{\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if previous_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOffsetsStable\":");
    output.push_str(if cross_table_offsets_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOffsetsRequireTransform\":");
    output.push_str(if cross_table_offsets_require_transform {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"piecewiseAllTablesExact\":");
    output.push_str(if piecewise_all_tables_exact {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOrderingConsistent\":");
    output.push_str(if cross_table_ordering_consistent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOrderRegresses\":");
    output.push_str(if cross_table_order_regresses {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"decodedPageYOriginPresent\":");
    output.push_str(if y_origin_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push('}');
    output.push_str(",\"subrecordSpanEvidence\":{\"present\":");
    output.push_str(if subrecord_span_readiness.is_some() {
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
    output.push_str(",\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if previous_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"compactRowSpanComplete\":");
    output.push_str(if compact_complete { "true" } else { "false" });
    output.push('}');
    output.push_str(",\"crossTableOrderingConsistent\":");
    output.push_str(if cross_table_ordering_consistent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOrderRegresses\":");
    output.push_str(if cross_table_order_regresses {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-only-page-y-origin-hypothesis\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if y_origin_solver_ready && blocked_reasons.is_empty() {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-page-y-origin-inference-pending"));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_source_only_page_y_origin_candidate_agreement_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let supports = table_grid_source_only_page_y_origin_candidate_supports(
        document,
        candidate,
        source_layout,
        cross_table_row_boundary_offset_probe,
        subrecord_span_readiness,
    );
    let mut groups: BTreeMap<
        (i32, Option<i32>),
        Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    > = BTreeMap::new();
    for support in supports {
        groups
            .entry((
                rounded_milli(support.selected_y),
                support.row_height.map(rounded_milli),
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
    let candidate_best_group = best_group.filter(|supports| {
        table_grid_source_only_page_y_origin_group_supports_candidate(supports, candidate)
    });
    let fallback_selector_group = if candidate_best_group.is_none() {
        table_grid_source_only_page_y_origin_fallback_selector_group(&groups, candidate)
    } else {
        None
    };
    let best_supported_table_candidate_indexes = best_group
        .map(|supports| {
            supports
                .iter()
                .filter_map(|support| support.table_candidate_index)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let best_supported_table_candidate_count = best_supported_table_candidate_indexes.len();
    let best_supported_covers_multiple_table_candidates = best_supported_table_candidate_count > 1;
    let selection_ready = false;
    let cross_table_previous_row_span_support_count = groups
        .values()
        .flatten()
        .filter(|support| {
            table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(support)
        })
        .count();
    let cross_table_previous_row_span_table_candidate_indexes = groups
        .values()
        .flatten()
        .filter(|support| {
            table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(support)
        })
        .filter_map(|support| support.table_candidate_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let cross_table_previous_row_span_table_candidate_count =
        cross_table_previous_row_span_table_candidate_indexes.len();
    let cross_table_previous_row_span_unique_best_supported = best_group.is_some_and(|supports| {
        supports
            .iter()
            .all(table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span)
    });
    let cross_table_previous_row_span_group_blocked_reasons = best_group
        .filter(|_| cross_table_previous_row_span_unique_best_supported)
        .map(|supports| table_grid_source_only_page_y_origin_supports_blocked_reasons(supports))
        .unwrap_or_default();
    let cross_table_previous_row_span_ready = cross_table_previous_row_span_unique_best_supported
        && cross_table_previous_row_span_group_blocked_reasons.is_empty();
    let cross_table_previous_row_span_best_group_table_coverage_ratio =
        if cross_table_previous_row_span_table_candidate_count == 0 {
            None
        } else {
            Some(
                best_supported_table_candidate_count as f32
                    / cross_table_previous_row_span_table_candidate_count as f32,
            )
        };
    let cross_table_previous_row_span_support_fragmented_by_table =
        cross_table_previous_row_span_table_candidate_count > 1
            && best_supported_table_candidate_count
                < cross_table_previous_row_span_table_candidate_count;

    let mut blocked_reasons = Vec::new();
    if groups.is_empty() {
        blocked_reasons.push("source-only-page-y-origin-candidates-absent");
    }
    if best_support_count <= 1 {
        blocked_reasons.push("source-only-page-y-origin-candidate-agreement-missing");
    }
    if best_group_count > 1 {
        blocked_reasons.push("source-only-page-y-origin-candidate-agreement-ambiguous");
    }
    if unique_best_supported {
        blocked_reasons.push("source-page-y-origin-field-semantics-still-unproven");
    }
    if unique_best_supported && !best_supported_covers_multiple_table_candidates {
        blocked_reasons.push("source-page-y-origin-best-support-not-cross-table");
    }
    if cross_table_previous_row_span_support_fragmented_by_table {
        blocked_reasons.push("cross-table-previous-row-span-support-fragmented-by-table");
    }

    output
        .push_str("{\"source\":\"sourcePageYTransformGate.sourcePageYOriginHypotheses agreement\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false");
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
    output.push_str(",\"bestSupportedSelectedY\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_y)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedRowHeight\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => push_optional_f32_json(output, support.row_height),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedOriginBases\":");
    match best_group {
        Some(supports) => {
            let origin_bases = supports
                .iter()
                .map(|support| support.origin_basis)
                .collect::<Vec<_>>();
            push_json_string_slice_array(output, &origin_bases);
        }
        None => output.push_str("[]"),
    }
    output.push_str(",\"bestSupportedTableCandidateIndexes\":");
    push_usize_array_json(output, &best_supported_table_candidate_indexes);
    output.push_str(",\"bestSupportedTableCandidateCount\":");
    output.push_str(&best_supported_table_candidate_count.to_string());
    output.push_str(",\"bestSupportedCoversMultipleTableCandidates\":");
    output.push_str(if best_supported_covers_multiple_table_candidates {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanSupportCount\":");
    output.push_str(&cross_table_previous_row_span_support_count.to_string());
    output.push_str(",\"crossTablePreviousRowSpanTableCandidateIndexes\":");
    push_usize_array_json(
        output,
        &cross_table_previous_row_span_table_candidate_indexes,
    );
    output.push_str(",\"crossTablePreviousRowSpanTableCandidateCount\":");
    output.push_str(&cross_table_previous_row_span_table_candidate_count.to_string());
    output.push_str(",\"crossTablePreviousRowSpanUniqueBestSupported\":");
    output.push_str(if cross_table_previous_row_span_unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanReady\":");
    output.push_str(if cross_table_previous_row_span_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanBestGroupCoversMultipleTables\":");
    output.push_str(
        if cross_table_previous_row_span_unique_best_supported
            && best_supported_covers_multiple_table_candidates
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"crossTablePreviousRowSpanBestGroupTableCoverageRatio\":");
    push_optional_f32_json(
        output,
        cross_table_previous_row_span_best_group_table_coverage_ratio,
    );
    output.push_str(",\"crossTablePreviousRowSpanSupportFragmentedByTable\":");
    output.push_str(
        if cross_table_previous_row_span_support_fragmented_by_table {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"crossTablePreviousRowSpanReadinessBlockedReasons\":");
    push_json_string_slice_array(output, &cross_table_previous_row_span_group_blocked_reasons);
    output.push_str(",\"sourceOnlyPageYOriginSelector\":");
    push_table_grid_source_only_page_y_origin_selector_json(
        output,
        candidate,
        candidate_best_group,
        fallback_selector_group.as_ref(),
        best_supported_covers_multiple_table_candidates,
        cross_table_previous_row_span_support_fragmented_by_table,
    );
    output.push_str(",\"agreementGroups\":[");
    for (index, supports) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let first = supports.first().unwrap();
        output.push_str("{\"selectedY\":");
        output.push_str(&format!("{:.3}", first.selected_y));
        output.push_str(",\"rowHeight\":");
        push_optional_f32_json(output, first.row_height);
        output.push_str(",\"supportCount\":");
        output.push_str(&supports.len().to_string());
        output.push_str(",\"originBases\":");
        let origin_bases = supports
            .iter()
            .map(|support| support.origin_basis)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &origin_bases);
        output.push_str(",\"tableCandidateIndexes\":");
        let table_indexes = supports
            .iter()
            .filter_map(|support| support.table_candidate_index)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        push_usize_array_json(output, &table_indexes);
        output.push_str(",\"contributions\":");
        let contributions = supports
            .iter()
            .map(|support| support.contribution)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &contributions);
        output.push_str(",\"blockedReasons\":");
        let blocked = table_grid_source_only_page_y_origin_supports_blocked_reasons(supports);
        push_json_string_slice_array(output, &blocked);
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-page-y-origin-candidate-agreement-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("source-page-y-origin-agreement-unproven"));
    output.push('}');
}

pub(crate) fn push_table_grid_source_only_page_y_origin_selector_json(
    output: &mut String,
    candidate: &TableCandidate,
    best_group: Option<&Vec<TableGridSourceOnlyPageYOriginCandidateSupport>>,
    fallback_group: Option<&Vec<TableGridSourceOnlyPageYOriginCandidateSupport>>,
    best_supported_covers_multiple_table_candidates: bool,
    cross_table_previous_row_span_support_fragmented_by_table: bool,
) {
    let Some(selector_group) = best_group.or(fallback_group) else {
        output.push_str("null");
        return;
    };
    let Some(first) = selector_group.first() else {
        output.push_str("null");
        return;
    };
    let using_single_support_fallback = best_group.is_none() && fallback_group.is_some();
    let best_table_candidate_indexes = selector_group
        .iter()
        .filter_map(|support| support.table_candidate_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let origin_bases = selector_group
        .iter()
        .map(|support| support.origin_basis)
        .collect::<Vec<_>>();
    let contributions = selector_group
        .iter()
        .map(|support| support.contribution)
        .collect::<Vec<_>>();
    let mut blocked_reasons =
        table_grid_source_only_page_y_origin_supports_blocked_reasons(selector_group);
    if using_single_support_fallback {
        blocked_reasons.push("single-source-y-origin-support-unproven");
    }
    let selector_support_covers_multiple_table_candidates = if using_single_support_fallback {
        best_table_candidate_indexes.len() > 1
    } else {
        best_supported_covers_multiple_table_candidates
    };
    let selector_support_fragmented_by_table =
        !using_single_support_fallback && cross_table_previous_row_span_support_fragmented_by_table;
    let selector_blocked_reason = if using_single_support_fallback {
        "single-source-y-origin-support-unproven"
    } else if selector_support_fragmented_by_table {
        "cross-table-previous-row-span-support-fragmented-by-table"
    } else if !selector_support_covers_multiple_table_candidates {
        "source-page-y-origin-best-support-not-cross-table"
    } else {
        "source-page-y-origin-field-semantics-still-unproven"
    };

    output.push_str(
        "{\"source\":\"sourceOnlyPageYOriginCandidateAgreementGate best-supported group selector\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"selectionBasis\":");
    output.push_str(&json_string(if using_single_support_fallback {
        "single-support-source-only-y-origin-fallback"
    } else {
        "best-supported-source-only-y-origin-agreement-group"
    }));
    output.push_str(",\"singleSupportFallback\":");
    output.push_str(if using_single_support_fallback {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedOriginBasis\":");
    output.push_str(&json_string(first.origin_basis));
    output.push_str(",\"selectedY\":");
    output.push_str(&format!("{:.3}", first.selected_y));
    output.push_str(",\"selectedRowHeight\":");
    push_optional_f32_json(output, first.row_height);
    output.push_str(",\"supportCount\":");
    output.push_str(&selector_group.len().to_string());
    output.push_str(",\"supportOriginBases\":");
    push_json_string_slice_array(output, &origin_bases);
    output.push_str(",\"supportTableCandidateIndexes\":");
    push_usize_array_json(output, &best_table_candidate_indexes);
    output.push_str(",\"supportCoversMultipleTableCandidates\":");
    output.push_str(if selector_support_covers_multiple_table_candidates {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"supportFragmentedByTable\":");
    output.push_str(if selector_support_fragmented_by_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"contributions\":");
    push_json_string_slice_array(output, &contributions);
    output.push_str(",\"supportBlockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-only-page-y-origin-selector\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(selector_blocked_reason));
    output.push('}');
}

pub(crate) fn table_grid_source_only_page_y_origin_fallback_selector_group(
    groups: &BTreeMap<(i32, Option<i32>), Vec<TableGridSourceOnlyPageYOriginCandidateSupport>>,
    candidate: &TableCandidate,
) -> Option<Vec<TableGridSourceOnlyPageYOriginCandidateSupport>> {
    groups
        .values()
        .filter(|supports| {
            !supports.is_empty()
                && table_grid_source_only_page_y_origin_group_supports_candidate(
                    supports, candidate,
                )
        })
        .min_by(|left, right| {
            let left_support = left.first().unwrap();
            let right_support = right.first().unwrap();
            table_grid_source_only_page_y_origin_fallback_rank(left_support)
                .cmp(&table_grid_source_only_page_y_origin_fallback_rank(
                    right_support,
                ))
                .then_with(|| {
                    left_support
                        .table_candidate_index
                        .unwrap_or(usize::MAX)
                        .cmp(&right_support.table_candidate_index.unwrap_or(usize::MAX))
                })
                .then_with(|| {
                    rounded_milli(left_support.selected_y)
                        .cmp(&rounded_milli(right_support.selected_y))
                })
                .then_with(|| left_support.origin_basis.cmp(right_support.origin_basis))
        })
        .cloned()
}

pub(crate) fn table_grid_source_only_page_y_origin_group_supports_candidate(
    supports: &[TableGridSourceOnlyPageYOriginCandidateSupport],
    candidate: &TableCandidate,
) -> bool {
    let mut has_table_specific_support = false;
    for support in supports {
        if let Some(table_candidate_index) = support.table_candidate_index {
            has_table_specific_support = true;
            if table_candidate_index == candidate.index() {
                return true;
            }
        }
    }
    if has_table_specific_support {
        return false;
    }
    supports
        .iter()
        .any(table_grid_source_only_page_y_origin_support_is_candidate_local_unindexed)
}

pub(crate) fn table_grid_source_only_page_y_origin_support_is_candidate_local_unindexed(
    support: &TableGridSourceOnlyPageYOriginCandidateSupport,
) -> bool {
    support.table_candidate_index.is_none()
        && !matches!(
            support.origin_basis,
            "cross-table-combined-previous-row-span-first-record"
        )
}

pub(crate) fn table_grid_source_only_page_y_origin_fallback_rank(
    support: &TableGridSourceOnlyPageYOriginCandidateSupport,
) -> usize {
    match support.origin_basis {
        "line-mark-page-origin-direct" => 0,
        "page-mark-absolute-y-slot-field2-tail-block16-word11" => 1,
        "line-mark-stride-raw-record-index-first-row" => 2,
        "line-mark-stride-collapsed-record-index-first-row" => 3,
        "cross-table-combined-previous-row-span-first-record" => 4,
        "cross-table-previous-row-span-table-first-row" => 5,
        "cross-table-selected-spacing-table-first-row" => 6,
        _ => 100,
    }
}

pub(crate) fn push_table_grid_source_only_page_y_origin_domain_gate_json(
    output: &mut String,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
) {
    let direct_line_mark_page_space_origin_present = source_layout.is_some_and(|layout| {
        layout.line_mark_page_origin.is_some() && layout.page_origin_authority == "lineMarkPageGrid"
    });
    let cross_table_line_domain_present = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| !probe.combined_line_mark_record_y_tops_px.is_empty());
    let selected_previous_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .iter()
                .flat_map(|table| {
                    table
                        .selected_minus_previous_record_index_gaps
                        .iter()
                        .copied()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let stable_selected_previous_gap = single_i32_value(&selected_previous_gaps);
    let selected_previous_y_delta_milli = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .iter()
                .flat_map(|table| {
                    table
                        .selected_minus_previous_record_y_delta_px
                        .iter()
                        .map(|value| rounded_milli(*value))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let stable_selected_previous_y_delta_px =
        single_i32_value(&selected_previous_y_delta_milli).map(|value| value as f32 / 1000.0);
    let transition_record_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.line_mark_record_gap)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let same_page_mark_entry_transition_count = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .filter(|transition| transition.same_page_mark_entry)
                .count()
        })
        .unwrap_or(0);
    let line_domain_requires_offset_transform =
        cross_table_row_boundary_offset_probe.is_some_and(|probe| {
            probe.all_offsets_require_transform
                || !probe.source_unit_to_page_line_index_piecewise_all_tables_exact
                || !probe
                    .source_unit_to_page_line_index_piecewise_transitions
                    .is_empty()
        });
    let page_space_origin_decoded =
        direct_line_mark_page_space_origin_present && !line_domain_requires_offset_transform;
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe,
        );

    let mut blocked_reasons = Vec::new();
    if !direct_line_mark_page_space_origin_present {
        blocked_reasons.push("direct-line-mark-page-space-origin-absent");
    }
    if cross_table_line_domain_present {
        blocked_reasons.push("cross-table-evidence-is-page-mark-line-domain");
    }
    if line_domain_requires_offset_transform {
        blocked_reasons.push("line-domain-to-page-space-origin-transform-required");
    }
    if !transition_record_gaps.is_empty() {
        blocked_reasons.push("table-family-transition-semantics-undecoded");
    }
    if stable_selected_previous_gap == Some(1) {
        blocked_reasons.push("selected-spacing-records-are-post-row-gap-family");
    }
    if !page_space_origin_decoded {
        blocked_reasons.push("page-space-table-origin-undecoded");
    }

    output.push_str("{\"source\":\"sourcePageYTransformGate.sourceOnlyPageYOriginDomainGate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"directLineMarkPageSpaceOriginPresent\":");
    output.push_str(if direct_line_mark_page_space_origin_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableLineDomainPresent\":");
    output.push_str(if cross_table_line_domain_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableLineDomainRecordCount\":");
    output.push_str(
        &cross_table_row_boundary_offset_probe
            .map(|probe| probe.combined_line_mark_record_indexes.len())
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"crossTableLineDomainTableCount\":");
    output.push_str(
        &cross_table_row_boundary_offset_probe
            .map(|probe| probe.tables.len())
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"combinedLineMarkRecordYPitchPx\":");
    push_optional_f32_json(
        output,
        cross_table_row_boundary_offset_probe
            .and_then(|probe| probe.combined_line_mark_record_y_pitch_px),
    );
    output.push_str(",\"combinedLineMarkRecordYTopPx\":");
    match cross_table_row_boundary_offset_probe {
        Some(probe) => push_f32_array_json(output, &probe.combined_line_mark_record_y_tops_px),
        None => output.push_str("[]"),
    }
    output.push_str(",\"stableSelectedMinusPreviousRecordIndexGap\":");
    push_optional_i32_json(output, stable_selected_previous_gap);
    output.push_str(",\"stableSelectedMinusPreviousRecordYDeltaPx\":");
    push_optional_f32_json(output, stable_selected_previous_y_delta_px);
    output.push_str(",\"selectedSpacingRecordsArePostRowGapFamily\":");
    output.push_str(if stable_selected_previous_gap == Some(1) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"piecewiseTransitionCount\":");
    output.push_str(&transition_record_gaps.len().to_string());
    output.push_str(",\"piecewiseTransitionRecordGaps\":");
    push_i32_array_json(output, &transition_record_gaps);
    output.push_str(",\"samePageMarkEntryTransitionCount\":");
    output.push_str(&same_page_mark_entry_transition_count.to_string());
    output.push_str(",\"transitionSemanticsReadiness\":");
    push_table_grid_source_only_page_y_transition_semantics_readiness_json(
        output,
        cross_table_row_boundary_offset_probe,
        same_page_mark_entry_transition_count,
    );
    output.push_str(",\"sourceGapToPageLineGapTransformAdmissionGate\":");
    push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
        output,
        "sourceOnlyPageYOriginDomainGate.sourceGapToPageLineGapTransformAdmissionGate",
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"lineDomainRequiresOffsetTransform\":");
    output.push_str(if line_domain_requires_offset_transform {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageSpaceOriginDecoded\":");
    output.push_str(if page_space_origin_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-page-y-origin-domain-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if page_space_origin_decoded && blocked_reasons.is_empty() {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "source-page-y-line-domain-not-page-space-origin",
        ));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_source_only_page_y_transition_semantics_readiness_json(
    output: &mut String,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    same_page_mark_entry_transition_count: usize,
) {
    let source_range_gap_units = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.source_range_gap_units)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let row_source_start_gap_units = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.row_source_start_gap_units)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let previous_family_record_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.line_mark_record_gap)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_family_record_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .windows(2)
                .filter_map(|pair| {
                    pair[0]
                        .selected_spacing_record_indexes
                        .last()
                        .copied()
                        .zip(pair[1].selected_spacing_record_indexes.first().copied())
                        .map(|(left, right)| signed_usize_delta_i32(right, left))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_minus_previous_family_record_gap_deltas = selected_family_record_gaps
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(selected, previous)| selected.saturating_sub(previous))
        .collect::<Vec<_>>();
    let previous_family_y_gap_px = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .windows(2)
                .filter_map(|pair| {
                    pair[0]
                        .line_mark_record_y_tops_px
                        .last()
                        .copied()
                        .zip(pair[1].line_mark_record_y_tops_px.first().copied())
                        .map(|(left, right)| right - left)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_family_y_gap_px = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .windows(2)
                .filter_map(|pair| {
                    pair[0]
                        .selected_spacing_record_y_tops_px
                        .last()
                        .copied()
                        .zip(pair[1].selected_spacing_record_y_tops_px.first().copied())
                        .map(|(left, right)| right - left)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_minus_previous_family_y_gap_delta_px = selected_family_y_gap_px
        .iter()
        .copied()
        .zip(previous_family_y_gap_px.iter().copied())
        .map(|(selected, previous)| selected - previous)
        .collect::<Vec<_>>();
    let source_range_units_per_previous_record_gap =
        ratio_usize_by_i32(&source_range_gap_units, &previous_family_record_gaps);
    let row_source_start_units_per_previous_record_gap =
        ratio_i32_by_i32(&row_source_start_gap_units, &previous_family_record_gaps);
    let previous_y_gap_px_per_record_gap =
        ratio_f32_by_i32(&previous_family_y_gap_px, &previous_family_record_gaps);
    let source_range_gap_ratio_stable =
        rounded_f32_values_all_same(&source_range_units_per_previous_record_gap);
    let row_source_start_gap_ratio_stable =
        rounded_f32_values_all_same(&row_source_start_units_per_previous_record_gap);
    let previous_y_gap_ratio_stable =
        rounded_f32_values_all_same(&previous_y_gap_px_per_record_gap);
    let source_range_gap_minus_page_line_gap_units = source_range_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(source_range_gap, page_line_gap)| {
            source_range_gap_minus_page_line_gap_units(source_range_gap, page_line_gap)
        })
        .collect::<Vec<_>>();
    let row_source_start_gap_minus_page_line_gap_units = row_source_start_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(row_source_start_gap, page_line_gap)| {
            row_source_start_gap.saturating_sub(page_line_gap)
        })
        .collect::<Vec<_>>();
    let source_range_gap_equals_page_line_gap = source_range_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(source_range_gap, page_line_gap)| {
            usize::try_from(page_line_gap)
                .map(|page_line_gap| source_range_gap == page_line_gap)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    let row_source_start_gap_equals_page_line_gap = row_source_start_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(row_source_start_gap, page_line_gap)| row_source_start_gap == page_line_gap)
        .collect::<Vec<_>>();
    let all_source_range_gaps_equal_page_line_gaps = !source_range_gap_equals_page_line_gap
        .is_empty()
        && source_range_gap_equals_page_line_gap
            .iter()
            .all(|value| *value);
    let all_row_source_start_gaps_equal_page_line_gaps = !row_source_start_gap_equals_page_line_gap
        .is_empty()
        && row_source_start_gap_equals_page_line_gap
            .iter()
            .all(|value| *value);
    let segment_offset_gap_units = row_source_start_gap_units
        .iter()
        .copied()
        .zip(source_range_gap_units.iter().copied())
        .map(|(row_source_start_gap, source_range_gap)| {
            row_source_start_gap_minus_source_range_gap_units(
                row_source_start_gap,
                source_range_gap,
            )
        })
        .collect::<Vec<_>>();
    let segment_offset_gap_minus_page_line_gap_units = segment_offset_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(segment_offset_gap, page_line_gap)| segment_offset_gap.saturating_sub(page_line_gap))
        .collect::<Vec<_>>();
    let segment_offset_gap_equals_page_line_gap = segment_offset_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(segment_offset_gap, page_line_gap)| segment_offset_gap == page_line_gap)
        .collect::<Vec<_>>();
    let all_segment_offsets_equal_page_line_gaps = !segment_offset_gap_equals_page_line_gap
        .is_empty()
        && segment_offset_gap_equals_page_line_gap
            .iter()
            .all(|value| *value);
    let segment_offset_units_per_page_line_gap =
        ratio_i32_by_i32(&segment_offset_gap_units, &previous_family_record_gaps);
    let segment_offset_gap_ratio_stable =
        rounded_f32_values_all_same(&segment_offset_units_per_page_line_gap);
    let mut source_gap_to_page_line_gap_declined_transform_kinds = Vec::new();
    if !source_range_gap_units.is_empty() && !all_source_range_gaps_equal_page_line_gaps {
        source_gap_to_page_line_gap_declined_transform_kinds.push("direct-source-range-gap");
    }
    if !row_source_start_gap_units.is_empty() && !all_row_source_start_gaps_equal_page_line_gaps {
        source_gap_to_page_line_gap_declined_transform_kinds.push("direct-row-source-start-gap");
    }
    if !segment_offset_gap_units.is_empty() && !all_segment_offsets_equal_page_line_gaps {
        source_gap_to_page_line_gap_declined_transform_kinds.push("segment-offset-gap");
    }
    let mut source_gap_to_page_line_gap_transform_blocked_reasons = Vec::new();
    if !source_range_gap_units.is_empty() && !all_source_range_gaps_equal_page_line_gaps {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("source-range-gap-not-equal-page-line-gap");
    }
    if !row_source_start_gap_units.is_empty() && !all_row_source_start_gaps_equal_page_line_gaps {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("row-source-start-gap-not-equal-page-line-gap");
    }
    if !source_range_gap_units.is_empty() && !source_range_gap_ratio_stable {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("source-range-gap-ratio-not-stable");
    }
    if !row_source_start_gap_units.is_empty() && !row_source_start_gap_ratio_stable {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("row-source-start-gap-ratio-not-stable");
    }
    if !segment_offset_gap_units.is_empty() && !all_segment_offsets_equal_page_line_gaps {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("segment-offset-gap-not-equal-page-line-gap");
    }
    if !segment_offset_gap_units.is_empty() && !segment_offset_gap_ratio_stable {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("segment-offset-gap-ratio-not-stable");
    }
    if !segment_offset_gap_units.is_empty() {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("source-gap-to-page-line-gap-segment-offset-transform-missing");
    }
    source_gap_to_page_line_gap_transform_blocked_reasons
        .push("source-gap-to-page-line-gap-transform-undecoded");
    let transition_count = previous_family_record_gaps.len();
    let all_transitions_same_page_mark_entry =
        transition_count > 0 && same_page_mark_entry_transition_count == transition_count;
    let record_gap_deltas_all_zero = !selected_minus_previous_family_record_gap_deltas.is_empty()
        && selected_minus_previous_family_record_gap_deltas
            .iter()
            .all(|delta| *delta == 0);
    let y_gap_deltas_all_zero = !selected_minus_previous_family_y_gap_delta_px.is_empty()
        && selected_minus_previous_family_y_gap_delta_px
            .iter()
            .all(|delta| delta.abs() <= 0.001);
    let family_gaps_stable_across_record_families =
        record_gap_deltas_all_zero && y_gap_deltas_all_zero;
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe,
        );

    let mut blocked_reasons = Vec::new();
    if transition_count == 0 {
        blocked_reasons.push("table-family-transition-evidence-absent");
    }
    if family_gaps_stable_across_record_families {
        blocked_reasons.push("previous-and-selected-family-transitions-share-line-domain-gaps");
    }
    if !source_range_gap_units.is_empty() {
        blocked_reasons.push("source-gap-to-page-line-gap-transform-missing");
    }
    if !source_range_gap_units.is_empty() && !source_range_gap_ratio_stable {
        blocked_reasons.push("source-range-gap-to-page-line-gap-ratio-not-stable");
    }
    if !row_source_start_gap_units.is_empty() && !row_source_start_gap_ratio_stable {
        blocked_reasons.push("row-source-start-gap-to-page-line-gap-ratio-not-stable");
    }
    if !segment_offset_gap_units.is_empty() {
        blocked_reasons.push("source-gap-to-page-line-gap-segment-offset-transform-missing");
    }
    blocked_reasons.push("table-family-transition-rule-undecoded");
    blocked_reasons.push("page-space-transition-origin-undecoded");

    output.push_str("{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"transitionCount\":");
    output.push_str(&transition_count.to_string());
    output.push_str(",\"samePageMarkEntryTransitionCount\":");
    output.push_str(&same_page_mark_entry_transition_count.to_string());
    output.push_str(",\"allTransitionsSamePageMarkEntry\":");
    output.push_str(if all_transitions_same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"transitionEvidenceDomain\":");
    output.push_str(&json_string("page-mark-line-index"));
    output.push_str(",\"transitionPairs\":[");
    if let Some(probe) = cross_table_row_boundary_offset_probe {
        for (index, pair) in probe.tables.windows(2).enumerate() {
            if index > 0 {
                output.push(',');
            }
            push_table_grid_piecewise_record_family_gap_transition_json(output, &pair[0], &pair[1]);
        }
    }
    output.push(']');
    output.push_str(",\"sourceRangeGapUnits\":");
    push_usize_array_json(output, &source_range_gap_units);
    output.push_str(",\"rowSourceStartGapUnits\":");
    push_i32_array_json(output, &row_source_start_gap_units);
    output.push_str(",\"previousFamilyRecordGaps\":");
    push_i32_array_json(output, &previous_family_record_gaps);
    output.push_str(",\"selectedFamilyRecordGaps\":");
    push_i32_array_json(output, &selected_family_record_gaps);
    output.push_str(",\"selectedMinusPreviousFamilyRecordGapDeltas\":");
    push_i32_array_json(output, &selected_minus_previous_family_record_gap_deltas);
    output.push_str(",\"previousFamilyYGapPx\":");
    push_f32_array_json(output, &previous_family_y_gap_px);
    output.push_str(",\"selectedFamilyYGapPx\":");
    push_f32_array_json(output, &selected_family_y_gap_px);
    output.push_str(",\"selectedMinusPreviousFamilyYGapDeltaPx\":");
    push_f32_array_json(output, &selected_minus_previous_family_y_gap_delta_px);
    output.push_str(",\"familyGapsStableAcrossRecordFamilies\":");
    output.push_str(if family_gaps_stable_across_record_families {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousAndSelectedTransitionRecordGapsAgree\":");
    output.push_str(if record_gap_deltas_all_zero {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousAndSelectedTransitionYGapsAgree\":");
    output.push_str(if y_gap_deltas_all_zero {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRangeUnitsPerPreviousRecordGap\":");
    push_f32_array_json(output, &source_range_units_per_previous_record_gap);
    output.push_str(",\"rowSourceStartUnitsPerPreviousRecordGap\":");
    push_f32_array_json(output, &row_source_start_units_per_previous_record_gap);
    output.push_str(",\"previousYGapPxPerRecordGap\":");
    push_f32_array_json(output, &previous_y_gap_px_per_record_gap);
    output.push_str(",\"sourceRangeGapRatioStable\":");
    output.push_str(if source_range_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSourceStartGapRatioStable\":");
    output.push_str(if row_source_start_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousYGapRatioStable\":");
    output.push_str(if previous_y_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceGapToPageLineGapDirectMapDiagnostic\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapDirectMapDiagnostic\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"gapBasis\":");
    output.push_str(&json_string(
        "same-page-mark-entry lineMarkRecordGap as page-mark-line-index gap",
    ));
    output.push_str(",\"sourceRangeGapUnits\":");
    push_usize_array_json(output, &source_range_gap_units);
    output.push_str(",\"rowSourceStartGapUnits\":");
    push_i32_array_json(output, &row_source_start_gap_units);
    output.push_str(",\"pageLineGaps\":");
    push_i32_array_json(output, &previous_family_record_gaps);
    output.push_str(",\"sourceRangeGapMinusPageLineGapUnits\":");
    push_i32_array_json(output, &source_range_gap_minus_page_line_gap_units);
    output.push_str(",\"rowSourceStartGapMinusPageLineGapUnits\":");
    push_i32_array_json(output, &row_source_start_gap_minus_page_line_gap_units);
    output.push_str(",\"sourceRangeGapEqualsPageLineGap\":");
    push_bool_array_json(output, &source_range_gap_equals_page_line_gap);
    output.push_str(",\"rowSourceStartGapEqualsPageLineGap\":");
    push_bool_array_json(output, &row_source_start_gap_equals_page_line_gap);
    output.push_str(",\"allSourceRangeGapsEqualPageLineGaps\":");
    output.push_str(if all_source_range_gaps_equal_page_line_gaps {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allRowSourceStartGapsEqualPageLineGaps\":");
    output.push_str(if all_row_source_start_gaps_equal_page_line_gaps {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRangeUnitsPerPageLineGap\":");
    push_f32_array_json(output, &source_range_units_per_previous_record_gap);
    output.push_str(",\"rowSourceStartUnitsPerPageLineGap\":");
    push_f32_array_json(output, &row_source_start_units_per_previous_record_gap);
    output.push_str(",\"sourceRangeUnitsPerPageLineGapStable\":");
    output.push_str(if source_range_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSourceStartUnitsPerPageLineGapStable\":");
    output.push_str(if row_source_start_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-direct-map-diagnostic-only\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-direct-map-not-decoded\"}");
    output.push_str(",\"sourceGapToPageLineGapSegmentOffsetDiagnostic\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapSegmentOffsetDiagnostic\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"offsetBasis\":");
    output.push_str(&json_string(
        "rowSourceStartGapUnits minus sourceRangeGapUnits",
    ));
    output.push_str(",\"sourceRangeGapUnits\":");
    push_usize_array_json(output, &source_range_gap_units);
    output.push_str(",\"rowSourceStartGapUnits\":");
    push_i32_array_json(output, &row_source_start_gap_units);
    output.push_str(",\"segmentOffsetGapUnits\":");
    push_i32_array_json(output, &segment_offset_gap_units);
    output.push_str(",\"pageLineGaps\":");
    push_i32_array_json(output, &previous_family_record_gaps);
    output.push_str(",\"segmentOffsetGapMinusPageLineGapUnits\":");
    push_i32_array_json(output, &segment_offset_gap_minus_page_line_gap_units);
    output.push_str(",\"segmentOffsetGapEqualsPageLineGap\":");
    push_bool_array_json(output, &segment_offset_gap_equals_page_line_gap);
    output.push_str(",\"allSegmentOffsetsEqualPageLineGaps\":");
    output.push_str(if all_segment_offsets_equal_page_line_gaps {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"segmentOffsetUnitsPerPageLineGap\":");
    push_f32_array_json(output, &segment_offset_units_per_page_line_gap);
    output.push_str(",\"segmentOffsetUnitsPerPageLineGapStable\":");
    output.push_str(if segment_offset_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"segmentOffsetTransformDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-segment-offset-diagnostic-only\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-segment-offset-not-decoded\"}");
    output.push_str(",\"sourceGapToPageLineGapTransformReadiness\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapTransformReadiness\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"transformDomain\":");
    output.push_str(&json_string("source-unit-gap-to-page-mark-line-index-gap"));
    output.push_str(",\"candidateTransformCount\":3");
    output.push_str(",\"acceptedTransformKind\":null");
    output.push_str(",\"directMapDeclined\":");
    output.push_str(
        if source_gap_to_page_line_gap_declined_transform_kinds.is_empty() {
            "false"
        } else {
            "true"
        },
    );
    output.push_str(",\"declinedTransformKinds\":");
    push_json_string_slice_array(
        output,
        &source_gap_to_page_line_gap_declined_transform_kinds,
    );
    output.push_str(",\"directMapEvidence\":");
    output.push_str(&json_string(
        "source gaps do not equal page-line gaps and their ratios are unstable",
    ));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(
        output,
        &source_gap_to_page_line_gap_transform_blocked_reasons,
    );
    output.push_str(",\"nextRequiredEvidence\":");
    output.push_str(&json_string(
        "decode source-gap unit domain or segment transition offset rule before page-space y promotion",
    ));
    output.push_str(",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-transform-readiness\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-transform-not-decoded\"}");
    output.push_str(",\"sourceGapToPageLineGapReadinessHints\":");
    push_table_grid_source_gap_to_page_line_gap_readiness_hints_json(
        output,
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"sourceGapToPageLineGapDecoded\":false");
    output.push_str(",\"pageSpaceTransitionOriginDecoded\":false");
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"table-family-transition-semantics-readiness\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("table-family-transition-semantics-undecoded"));
    output.push('}');
}

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
        if let Some(slot) = absolute_y_slot_agreement.best_absolute_y_slot.as_ref()
            && slot.field_index == 2
            && slot.tail_block16_word_index == Some(11)
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
