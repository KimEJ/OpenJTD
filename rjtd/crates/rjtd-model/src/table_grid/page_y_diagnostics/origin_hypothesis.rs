use super::*;
use crate::*;

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
