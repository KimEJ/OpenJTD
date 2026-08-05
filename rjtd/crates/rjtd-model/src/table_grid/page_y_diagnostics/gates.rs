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
        for reason in page_mark_absolute_y_slot_agreement.field_quantization_blocked_reasons() {
            if !blocked_reasons.contains(&reason) {
                blocked_reasons.push(reason);
            }
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
    output.push_str(",\"sourceOnlyPageMarkFieldQuantizationGate\":");
    push_table_grid_source_only_page_mark_field_quantization_gate_json(
        output,
        &page_mark_absolute_y_slot_agreement,
    );
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
    } else if agreement.field_quantization_refutes_page_space_px() {
        "page-mark-absolute-y-slot-field-quantized-not-page-space-px"
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
    let field_quantization =
        table_grid_source_only_page_mark_field_quantization(document, subrecord_span_readiness);

    TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
        line_domain_y,
        selected_span_units,
        line_domain_projected_y,
        candidates,
        best_absolute_y_slot,
        residual_px,
        agrees,
        field_quantization,
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
    blocked_reasons.extend(agreement.field_quantization_blocked_reasons());
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
    output.push_str(",\"sourceOnlyPageMarkFieldQuantizationGate\":");
    push_table_grid_source_only_page_mark_field_quantization_gate_json(output, &agreement);
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

/// Word index currently interpreted by the absolute-y-slot candidate path.
/// Words 3/5/7 are structurally zero and words 4/6 form the line range;
/// words 0/1 remain unknown and are not decoded by this gate.
pub(crate) const PAGE_MARK_ABSOLUTE_Y_SLOT_FIELD_INDEX: usize = 2;

/// Byte boundary tested for the absolute-y-slot field. A page-space px value has
/// no reason to be a whole multiple of 256 with a zero low byte.
pub(crate) const PAGE_MARK_ABSOLUTE_Y_SLOT_FIELD_QUANTUM_UNITS: u16 = 256;

/// Reads the absolute-y-slot field once per matched line-mark record (duplicates
/// kept, so per-row repetition stays visible) and reports whether the values can
/// be direct page-space px at all.
pub(crate) fn table_grid_source_only_page_mark_field_quantization(
    document: &Document,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> Option<TableGridSourceOnlyPageMarkFieldQuantization> {
    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let readiness = subrecord_span_readiness?;
    let record_headers = page_mark_record_headers(page_mark_bytes);
    let field_index = PAGE_MARK_ABSOLUTE_Y_SLOT_FIELD_INDEX;
    let quantum_units = PAGE_MARK_ABSOLUTE_Y_SLOT_FIELD_QUANTUM_UNITS;

    let mut row_values = Vec::new();
    let mut raw_record_scan_indexes = Vec::new();
    let mut tail_block16_word_indexes = BTreeSet::new();
    for subrecord_byte_offset in readiness
        .selected_post_row_gap_span_coverage
        .matched_candidate_byte_offsets
        .iter()
        .copied()
    {
        let Some(subrecord) =
            page_mark_raw_u16_subrecord_candidate_at(page_mark_bytes, subrecord_byte_offset)
        else {
            continue;
        };
        let byte_offset = subrecord.byte_offset + field_index * 2;
        let Some((raw_record_scan_index, _, tail_block16_word_index)) =
            page_mark_raw_subrecord_record_context(&record_headers, byte_offset)
        else {
            continue;
        };
        row_values.push(subrecord.words[field_index]);
        raw_record_scan_indexes.push(raw_record_scan_index);
        tail_block16_word_indexes.insert(tail_block16_word_index);
    }
    if row_values.is_empty() {
        return None;
    }

    let distinct_values = row_values.iter().copied().collect::<BTreeSet<_>>();
    let high_byte_values = row_values.iter().map(|v| v >> 8).collect::<BTreeSet<_>>();
    let all_values_multiple_of_quantum = row_values.iter().all(|v| v % quantum_units == 0);
    let low_byte_all_zero = row_values.iter().all(|value| (*value & 0x00ff) == 0);
    let mut values_by_raw_record_scan_index: BTreeMap<usize, u16> = BTreeMap::new();
    let mut values_constant_per_raw_record_scan_index = true;
    for (scan_index, value) in raw_record_scan_indexes
        .iter()
        .copied()
        .zip(row_values.iter().copied())
    {
        match values_by_raw_record_scan_index.get(&scan_index) {
            Some(seen) => values_constant_per_raw_record_scan_index &= *seen == value,
            None => {
                values_by_raw_record_scan_index.insert(scan_index, value);
            }
        }
    }
    let value_row_distinct = distinct_values.len() == row_values.len();
    let quantized = all_values_multiple_of_quantum && low_byte_all_zero;

    Some(TableGridSourceOnlyPageMarkFieldQuantization {
        field_index,
        tail_block16_word_index: tail_block16_word_indexes
            .iter()
            .copied()
            .next()
            .filter(|_| tail_block16_word_indexes.len() == 1),
        quantum_units,
        value_count: row_values.len(),
        distinct_values: distinct_values.into_iter().collect(),
        all_values_multiple_of_quantum,
        low_byte_all_zero,
        high_byte_values: high_byte_values.into_iter().collect(),
        raw_record_scan_indexes,
        values_constant_per_raw_record_scan_index,
        value_row_distinct,
        // Quantization refutes the direct-px interpretation. Repetition alone is
        // diagnostic evidence, not a refutation: a table-origin field could repeat.
        page_space_px_plausible: !quantized,
        row_values,
    })
}

pub(crate) fn push_table_grid_source_only_page_mark_field_quantization_gate_json(
    output: &mut String,
    agreement: &TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement,
) {
    let Some(quantization) = agreement.field_quantization.as_ref() else {
        output.push_str("null");
        return;
    };
    let blocked_reasons = agreement.field_quantization_blocked_reasons();

    output.push_str("{\"source\":\"/PageMark raw u16 subrecord field scan\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"fieldIndex\":");
    output.push_str(&quantization.field_index.to_string());
    output.push_str(",\"tailBlock16WordIndex\":");
    push_option_usize_json(output, quantization.tail_block16_word_index);
    output.push_str(",\"quantumUnits\":");
    output.push_str(&quantization.quantum_units.to_string());
    output.push_str(",\"valueCount\":");
    output.push_str(&quantization.value_count.to_string());
    output.push_str(",\"rowValues\":");
    push_u16_array_json(output, &quantization.row_values);
    output.push_str(",\"distinctValues\":");
    push_u16_array_json(output, &quantization.distinct_values);
    output.push_str(",\"allValuesMultipleOfQuantum\":");
    output.push_str(json_bool(quantization.all_values_multiple_of_quantum));
    output.push_str(",\"lowByteAllZero\":");
    output.push_str(json_bool(quantization.low_byte_all_zero));
    output.push_str(",\"highByteValues\":");
    push_u16_array_json(output, &quantization.high_byte_values);
    output.push_str(",\"rawRecordScanIndexes\":");
    push_usize_array_json(output, &quantization.raw_record_scan_indexes);
    output.push_str(",\"valuesConstantPerRawRecordScanIndex\":");
    output.push_str(json_bool(
        quantization.values_constant_per_raw_record_scan_index,
    ));
    output.push_str(",\"valueRowDistinct\":");
    output.push_str(json_bool(quantization.value_row_distinct));
    output.push_str(",\"pageSpacePxPlausible\":");
    output.push_str(json_bool(quantization.page_space_px_plausible));
    output.push_str(",\"highByteVsRowCountRelationship\":\"undecoded-diagnostic-comparison\"");
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-only-page-mark-field-quantization-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    match blocked_reasons.first() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
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
        let field_index = PAGE_MARK_ABSOLUTE_Y_SLOT_FIELD_INDEX;
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
