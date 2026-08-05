use super::*;
use crate::*;

pub(crate) fn push_page_layer_fdm_text_mask_cohort_summary_json(
    output: &mut String,
    layout: PageLayout,
    cohorts: &[FdmTextMaskCohortDiagnosticSummary],
    text_projection: &ShanaiLanTextProjection,
) {
    output.push_str("{\"type\":\"fdmTextMaskCohortSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive+documentTextGroupLineProjection\"");
    output.push_str(",\"projectionKind\":\"fdmTextMaskCohortSummary\"");
    output.push_str(",\"basis\":\"fdmVectorClosedFillCohort+documentTextRightNeighbor\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"fdm-text-mask-document-text-alignment-unproven\"",
    );
    output.push_str(",\"candidatePredicate\":{\"minPrimitiveCount\":");
    output.push_str(&FDM_TEXT_MASK_COHORT_MIN_PRIMITIVES.to_string());
    output.push_str(",\"maxCohorts\":");
    output.push_str(&FDM_TEXT_MASK_COHORT_LIMIT.to_string());
    output.push_str(",\"rightNeighborMaxGapFactor\":");
    output.push_str(&format!("{FDM_TEXT_MASK_RIGHT_NEIGHBOR_MAX_GAP_FACTOR:.3}"));
    output.push_str(",\"requiresClosedFillPrimitive\":true,\"requiresBlackOrWhiteFill\":true}");
    output.push_str(",\"cohortCount\":");
    output.push_str(&cohorts.len().to_string());
    output.push_str(",\"rightNeighborCandidateCount\":");
    output.push_str(
        &cohorts
            .iter()
            .filter(|cohort| {
                fdm_text_mask_cohort_right_neighbor_text_slot(cohort, text_projection).is_some()
            })
            .count()
            .to_string(),
    );
    output.push_str(",\"topTextLikeComponentCandidateCount\":");
    output.push_str(
        &cohorts
            .iter()
            .filter(|cohort| cohort.top_text_like_component.is_some())
            .count()
            .to_string(),
    );
    output.push_str(",\"componentRightNeighborCandidateCount\":");
    output.push_str(
        &cohorts
            .iter()
            .filter(|cohort| {
                cohort
                    .top_text_like_component
                    .and_then(|component| component.projected_bbox)
                    .and_then(|bbox| {
                        fdm_text_mask_bbox_right_neighbor_text_slot(bbox, text_projection)
                    })
                    .is_some()
            })
            .count()
            .to_string(),
    );
    output.push_str(",\"cohorts\":[");
    for (index, cohort) in cohorts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_text_mask_cohort_json(output, cohort, text_projection);
    }
    output.push_str("]}");
}

pub(crate) fn push_page_layer_fdm_text_mask_source_transform_candidate_summary_json(
    output: &mut String,
    layout: PageLayout,
    cohorts: &[FdmTextMaskCohortDiagnosticSummary],
    text_projection: &ShanaiLanTextProjection,
) {
    let candidates = fdm_text_mask_source_transform_candidates(cohorts, text_projection);
    let bridge_candidate_count = candidates
        .iter()
        .filter(|candidate| candidate.metrics.source_bbox_within_pre_fragment_projection)
        .count();
    let row_anchor_ambiguous_count = candidates
        .iter()
        .filter(|candidate| {
            candidate
                .slot
                .line_header_same_segment_group_run_distinct_text_group_count
                .is_some_and(|count| count > 1)
        })
        .count();
    let slot_not_split_count = candidates
        .iter()
        .filter(|candidate| !candidate.slot.split_from_text_run)
        .count();
    let cohort_component_agreement_count = candidates
        .iter()
        .filter(|candidate| candidate.cohort_component_agreement)
        .count();

    output.push_str("{\"type\":\"fdmTextMaskSourceTransformCandidateSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"fdmVectorClosedFillComponent+/DocumentText pre-fragment span\"");
    output.push_str(",\"projectionKind\":\"fdmTextMaskSourceTransformCandidateSummary\"");
    output.push_str(",\"basis\":\"topTextLikeComponent+preFragmentGridOffset\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"renderPromotionBlockedReason\":\"fdm-source-to-document-text-transform-reference-backed-and-row-anchor-unproven\"");
    output.push_str(",\"candidatePredicate\":{\"requiresTopTextLikeComponent\":true,\"requiresComponentSourceBbox\":true,\"requiresRightNeighborDocumentTextSlot\":true,\"requiresPreFragmentSpan\":true}");
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidates.len().to_string());
    output.push_str(",\"preFragmentBridgeCandidateCount\":");
    output.push_str(&bridge_candidate_count.to_string());
    output.push_str(",\"cohortComponentAgreementCount\":");
    output.push_str(&cohort_component_agreement_count.to_string());
    output.push_str(",\"rowAnchorAmbiguousCandidateCount\":");
    output.push_str(&row_anchor_ambiguous_count.to_string());
    output.push_str(",\"slotNotSplitCandidateCount\":");
    output.push_str(&slot_not_split_count.to_string());
    output.push_str(",\"sourceUnitsPerTextGridUnitXRange\":");
    push_fdm_text_mask_source_transform_ratio_range_json(output, &candidates);
    output.push_str(",\"promotionGate\":{\"sourceBacked\":true,\"referenceBacked\":true,\"promotionReady\":false,\"blockedReasons\":[\"document-text-grid-origin-reference-backed\",\"line-header-y-run-placement-semantics-unproven\",\"document-text-pre-fragment-fdm-mask-role-unproven\",\"fdm-text-mask-to-document-text-baseline-transform-unproven\",\"fdm-source-transform-cross-sample-support-missing\"],\"renderPromotionBlockedReason\":\"fdm-source-to-document-text-transform-reference-backed-and-row-anchor-unproven\"}");
    output.push_str(",\"candidates\":[");
    for (index, candidate) in candidates.iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_text_mask_source_transform_candidate_json(output, *candidate);
    }
    output.push_str("]}");
}

pub(crate) fn push_fdm_text_mask_source_transform_ratio_range_json(
    output: &mut String,
    candidates: &[FdmTextMaskSourceTransformCandidate<'_>],
) {
    let mut ratios = candidates
        .iter()
        .map(|candidate| candidate.source_units_per_text_grid_unit_x)
        .filter(|ratio| ratio.is_finite());
    let Some(first) = ratios.next() else {
        output.push_str("null");
        return;
    };
    let (mut min_ratio, mut max_ratio) = (first, first);
    for ratio in ratios {
        min_ratio = min_ratio.min(ratio);
        max_ratio = max_ratio.max(ratio);
    }
    output.push_str("{\"min\":");
    output.push_str(&format!("{min_ratio:.3}"));
    output.push_str(",\"max\":");
    output.push_str(&format!("{max_ratio:.3}"));
    output.push('}');
}

pub(crate) fn push_fdm_text_mask_source_transform_candidate_json(
    output: &mut String,
    candidate: FdmTextMaskSourceTransformCandidate<'_>,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&candidate.row_index.to_string());
    output.push_str(",\"candidateClass\":");
    output.push_str(&json_string(candidate.candidate_class));
    output.push_str(",\"componentIndex\":");
    push_option_usize_json(output, candidate.component_index);
    output.push_str(",\"slotIndex\":");
    output.push_str(&candidate.slot_index.to_string());
    output.push_str(",\"slotText\":");
    output.push_str(&json_string(&candidate.slot.text));
    output.push_str(",\"slotSourceUnitRange\":");
    output.push_str(&source_range_json(
        candidate.slot.source_span.unit_start(),
        candidate.slot.source_span.unit_end(),
    ));
    output.push_str(",\"slotGroupIndex\":");
    match candidate.slot.group_index {
        Some(group_index) => output.push_str(&group_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceBbox\":");
    push_fdm_normalized_bbox_json(output, candidate.source_bbox);
    output.push_str(",\"currentProjectedBbox\":");
    push_bbox_tuple_json(output, candidate.projected_bbox);
    output.push_str(",\"currentProjectionGridOffsetRange\":{\"start\":");
    output.push_str(&format!("{:.3}", candidate.current_projection_grid_start));
    output.push_str(",\"end\":");
    output.push_str(&format!("{:.3}", candidate.current_projection_grid_end));
    output.push_str(",\"span\":");
    output.push_str(&format!("{:.3}", candidate.current_projection_grid_span));
    output.push('}');
    output.push_str(",\"sourceXTransformCandidate\":{\"sourceUnitsPerTextGridUnit\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.source_units_per_text_grid_unit_x
    ));
    output.push_str(",\"lineStartSourceX\":");
    output.push_str(&format!("{:.3}", candidate.line_start_source_x));
    output.push_str(",\"textStartSourceX\":");
    output.push_str(&format!("{:.3}", candidate.text_start_source_x));
    output.push_str(",\"sourceGapToTextStartX\":");
    output.push_str(&format!("{:.3}", candidate.source_gap_to_text_start_x));
    output.push_str(",\"transformAuthorityProven\":false}");
    output.push_str(",\"preFragmentBridge\":{\"preFragmentUnitCount\":");
    output.push_str(&candidate.metrics.pre_fragment_unit_count.to_string());
    output.push_str(",\"preFragmentGridUnits\":");
    output.push_str(&candidate.metrics.pre_fragment_grid_units.to_string());
    output.push_str(",\"sourceBboxWithinPreFragmentProjection\":");
    output.push_str(
        &candidate
            .metrics
            .source_bbox_within_pre_fragment_projection
            .to_string(),
    );
    output.push_str(",\"sourceBboxRightToTextStartGapPx\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.metrics.source_bbox_right_to_text_start_px
    ));
    output.push_str(",\"baselineResidualPx\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.metrics.text_baseline_minus_source_bottom_px
    ));
    output.push('}');
    output.push_str(",\"cohortComponentAgreement\":");
    output.push_str(&candidate.cohort_component_agreement.to_string());
    output.push_str(",\"rowAnchorAmbiguous\":");
    output.push_str(
        &candidate
            .slot
            .line_header_same_segment_group_run_distinct_text_group_count
            .is_some_and(|count| count > 1)
            .to_string(),
    );
    output.push_str(",\"splitFromTextRun\":");
    output.push_str(&candidate.slot.split_from_text_run.to_string());
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"fdm-source-transform-candidate-diagnostic-only\"}",
    );
}

pub(crate) fn push_fdm_text_mask_cohort_json(
    output: &mut String,
    cohort: &FdmTextMaskCohortDiagnosticSummary,
    text_projection: &ShanaiLanTextProjection,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&cohort.row_index.to_string());
    output.push_str(",\"primitiveCount\":");
    output.push_str(&cohort.primitive_count.to_string());
    output.push_str(",\"blackFillPrimitiveCount\":");
    output.push_str(&cohort.black_fill_primitive_count.to_string());
    output.push_str(",\"whiteFillPrimitiveCount\":");
    output.push_str(&cohort.white_fill_primitive_count.to_string());
    output.push_str(",\"counterOverlayCount\":");
    output.push_str(&cohort.counter_overlay_count.to_string());
    output.push_str(",\"commandIndexMin\":");
    push_option_usize_json(output, cohort.command_index_min);
    output.push_str(",\"commandIndexMax\":");
    push_option_usize_json(output, cohort.command_index_max);
    output.push_str(",\"relativeOffsetMin\":");
    push_option_usize_json(output, cohort.relative_offset_min);
    output.push_str(",\"relativeOffsetMax\":");
    push_option_usize_json(output, cohort.relative_offset_max);
    output.push_str(",\"projectedBbox\":");
    if let Some(bbox) = cohort.projected_bbox {
        push_bbox_tuple_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceBbox\":");
    if let Some(bbox) = cohort.source_bbox {
        push_fdm_normalized_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rightNeighborTextSlotCandidate\":");
    push_fdm_text_mask_right_neighbor_text_slot_json(
        output,
        cohort.projected_bbox,
        text_projection,
        "right-neighbor-overlapping-y",
    );
    output.push_str(",\"componentCount\":");
    output.push_str(&cohort.component_count.to_string());
    output.push_str(",\"topTextLikeComponentCandidate\":");
    if let Some(component) = cohort.top_text_like_component {
        push_fdm_text_mask_component_json(output, component, text_projection);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rightNeighborPromotionReadiness\":");
    push_fdm_text_mask_right_neighbor_promotion_readiness_json(output, cohort, text_projection);
    output.push('}');
}

pub(crate) fn push_fdm_text_mask_component_json(
    output: &mut String,
    component: FdmTextMaskComponentDiagnosticSummary,
    text_projection: &ShanaiLanTextProjection,
) {
    output.push_str("{\"source\":\"fdmVectorClosedFillComponent\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"fdm-text-mask-component-to-document-text-alignment-unproven\"",
    );
    output.push_str(",\"componentIndex\":");
    output.push_str(&component.component_index.to_string());
    output.push_str(",\"primitiveCount\":");
    output.push_str(&component.primitive_count.to_string());
    output.push_str(",\"blackFillPrimitiveCount\":");
    output.push_str(&component.black_fill_primitive_count.to_string());
    output.push_str(",\"whiteFillPrimitiveCount\":");
    output.push_str(&component.white_fill_primitive_count.to_string());
    output.push_str(",\"counterOverlayCount\":");
    output.push_str(&component.counter_overlay_count.to_string());
    output.push_str(",\"commandIndexMin\":");
    push_option_usize_json(output, component.command_index_min);
    output.push_str(",\"commandIndexMax\":");
    push_option_usize_json(output, component.command_index_max);
    output.push_str(",\"relativeOffsetMin\":");
    push_option_usize_json(output, component.relative_offset_min);
    output.push_str(",\"relativeOffsetMax\":");
    push_option_usize_json(output, component.relative_offset_max);
    output.push_str(",\"projectedBbox\":");
    if let Some(bbox) = component.projected_bbox {
        push_bbox_tuple_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceBbox\":");
    if let Some(bbox) = component.source_bbox {
        push_fdm_normalized_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rightNeighborTextSlotCandidate\":");
    push_fdm_text_mask_right_neighbor_text_slot_json(
        output,
        component.projected_bbox,
        text_projection,
        "component-right-neighbor-overlapping-y",
    );
    output.push('}');
}

pub(crate) fn push_fdm_text_mask_right_neighbor_promotion_readiness_json(
    output: &mut String,
    cohort: &FdmTextMaskCohortDiagnosticSummary,
    text_projection: &ShanaiLanTextProjection,
) {
    let cohort_candidates = cohort
        .projected_bbox
        .map(|bbox| fdm_text_mask_bbox_right_neighbor_text_slot_candidates(bbox, text_projection))
        .unwrap_or_default();
    let component_bbox = cohort
        .top_text_like_component
        .and_then(|component| component.projected_bbox);
    let component_candidates = component_bbox
        .map(|bbox| fdm_text_mask_bbox_right_neighbor_text_slot_candidates(bbox, text_projection))
        .unwrap_or_default();
    let cohort_best = cohort_candidates.first().copied();
    let component_best = component_candidates.first().copied();
    let Some((selected, selected_bbox, selected_candidates)) = component_best
        .and_then(|candidate| Some((candidate, component_bbox?, &component_candidates)))
        .or_else(|| {
            cohort_best
                .and_then(|candidate| Some((candidate, cohort.projected_bbox?, &cohort_candidates)))
        })
    else {
        output.push_str("null");
        return;
    };
    let metrics =
        fdm_text_mask_pre_fragment_bridge_metrics(selected_bbox, text_projection, selected.slot);
    let cohort_component_agreement =
        cohort_best
            .zip(component_best)
            .is_some_and(|(cohort, component)| {
                fdm_text_mask_right_neighbor_candidates_same_slot(cohort, component)
            });
    let second_best = selected_candidates.get(1).copied();
    let gap_margin_px = second_best.map(|candidate| candidate.gap_px.abs() - selected.gap_px.abs());
    let row_anchor_ambiguous = selected
        .slot
        .line_header_same_segment_group_run_distinct_text_group_count
        .is_some_and(|count| count > 1);

    output.push_str("{\"type\":\"fdmTextMaskRightNeighborPromotionReadiness\"");
    output.push_str(",\"source\":\"fdmVectorClosedFillCohort+/DocumentText\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"cohortSlot\":");
    push_fdm_text_mask_right_neighbor_readiness_slot_json(output, cohort_best);
    output.push_str(",\"componentSlot\":");
    push_fdm_text_mask_right_neighbor_readiness_slot_json(output, component_best);
    output.push_str(",\"cohortComponentAgreement\":");
    output.push_str(&cohort_component_agreement.to_string());
    output.push_str(",\"bestGapPx\":");
    output.push_str(&format!("{:.3}", selected.gap_px));
    output.push_str(",\"secondBestGapPx\":");
    if let Some(second_best) = second_best {
        output.push_str(&format!("{:.3}", second_best.gap_px));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"gapMarginPx\":");
    if let Some(gap_margin_px) = gap_margin_px {
        output.push_str(&format!("{gap_margin_px:.3}"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"splitFromTextRun\":");
    output.push_str(&selected.slot.split_from_text_run.to_string());
    output.push_str(",\"fragmentCount\":");
    output.push_str(&selected.slot.fragment_context.fragment_count.to_string());
    output.push_str(",\"preFragmentUnitCount\":");
    output.push_str(&metrics.pre_fragment_unit_count.to_string());
    output.push_str(",\"sourceBboxWithinPreFragmentProjection\":");
    output.push_str(
        &metrics
            .source_bbox_within_pre_fragment_projection
            .to_string(),
    );
    output.push_str(",\"sourceBboxBeginsAfterLineStart\":");
    output.push_str(&metrics.source_begins_after_line_start.to_string());
    output.push_str(",\"sourceBboxEndsBeforeTextStart\":");
    output.push_str(&metrics.source_ends_before_text_start.to_string());
    output.push_str(",\"sameSegmentGroupRunDistinctTextGroupCount\":");
    push_option_usize_json(
        output,
        selected
            .slot
            .line_header_same_segment_group_run_distinct_text_group_count,
    );
    output.push_str(",\"rowAnchorAmbiguous\":");
    output.push_str(&row_anchor_ambiguous.to_string());
    output.push_str(",\"baselineResidualPx\":");
    output.push_str(&format!(
        "{:.3}",
        metrics.text_baseline_minus_source_bottom_px
    ));
    output.push_str(",\"promotionReady\":false,\"blockedReasons\":");
    push_fdm_text_mask_promotion_blocked_reasons_json(
        output,
        cohort_best,
        component_best,
        cohort_component_agreement,
        selected.slot,
        metrics,
        row_anchor_ambiguous,
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"fdm-text-mask-right-neighbor-promotion-readiness-blocked\"}");
}

pub(crate) fn push_fdm_text_mask_right_neighbor_readiness_slot_json(
    output: &mut String,
    candidate: Option<FdmTextMaskRightNeighborCandidate<'_>>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"slotIndex\":");
    output.push_str(&candidate.slot_index.to_string());
    output.push_str(",\"text\":");
    output.push_str(&json_string(&candidate.slot.text));
    output.push_str(",\"bbox\":");
    push_bbox_tuple_json(output, candidate.bbox);
    output.push_str(",\"horizontalGapPx\":");
    output.push_str(&format!("{:.3}", candidate.gap_px));
    output.push_str(",\"verticalOverlapPx\":");
    output.push_str(&format!("{:.3}", candidate.vertical_overlap_px));
    output.push_str(",\"centerDeltaYPx\":");
    output.push_str(&format!("{:.3}", candidate.center_delta_y_px));
    output.push_str(",\"groupIndex\":");
    match candidate.slot.group_index {
        Some(group_index) => output.push_str(&group_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(
        candidate.slot.source_span.unit_start(),
        candidate.slot.source_span.unit_end(),
    ));
    output.push_str(",\"splitFromTextRun\":");
    output.push_str(&candidate.slot.split_from_text_run.to_string());
    output.push('}');
}

pub(crate) fn push_fdm_text_mask_promotion_blocked_reasons_json(
    output: &mut String,
    cohort_best: Option<FdmTextMaskRightNeighborCandidate<'_>>,
    component_best: Option<FdmTextMaskRightNeighborCandidate<'_>>,
    cohort_component_agreement: bool,
    selected_slot: &ShanaiLanTextSlot,
    metrics: FdmTextMaskPreFragmentBridgeMetrics,
    row_anchor_ambiguous: bool,
) {
    let mut reasons = Vec::<&str>::new();
    if cohort_best.is_none() {
        reasons.push("fdm-text-mask-cohort-right-neighbor-missing");
    }
    if component_best.is_none() {
        reasons.push("fdm-text-mask-component-right-neighbor-missing");
    }
    if !cohort_component_agreement {
        reasons.push("fdm-text-mask-cohort-component-slot-disagreement");
    }
    if !selected_slot.split_from_text_run {
        reasons.push("document-text-slot-not-split-from-text-run");
    }
    if metrics.pre_fragment_unit_count == 0 {
        reasons.push("document-text-pre-fragment-empty");
    }
    if !metrics.source_bbox_within_pre_fragment_projection {
        reasons.push("fdm-bbox-outside-document-text-pre-fragment-projection");
    }
    if row_anchor_ambiguous {
        reasons.push("line-header-y-run-placement-semantics-unproven");
    }
    reasons.push("document-text-pre-fragment-fdm-mask-role-unproven");
    reasons.push("fdm-text-mask-to-document-text-baseline-transform-unproven");
    reasons.push("fdm-text-mask-promotion-cross-sample-support-missing");

    output.push('[');
    for (index, reason) in reasons.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(reason));
    }
    output.push(']');
}

pub(crate) fn fdm_text_mask_right_neighbor_candidates_same_slot(
    left: FdmTextMaskRightNeighborCandidate<'_>,
    right: FdmTextMaskRightNeighborCandidate<'_>,
) -> bool {
    left.slot_index == right.slot_index
}

pub(crate) fn push_fdm_text_mask_right_neighbor_text_slot_json(
    output: &mut String,
    source_bbox: Option<(f32, f32, f32, f32)>,
    text_projection: &ShanaiLanTextProjection,
    candidate_relation: &'static str,
) {
    let Some(source_bbox) = source_bbox else {
        output.push_str("null");
        return;
    };
    let Some((slot, bbox, gap_px, vertical_overlap_px, center_delta_y_px)) =
        fdm_text_mask_bbox_right_neighbor_text_slot(source_bbox, text_projection)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/DocumentText\",\"sourceBacked\":true,\"decoded\":false,\"candidateRelation\":");
    output.push_str(&json_string(candidate_relation));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&slot.text));
    output.push_str(",\"bbox\":");
    push_bbox_tuple_json(output, bbox);
    output.push_str(",\"horizontalGapPx\":");
    output.push_str(&format!("{gap_px:.3}"));
    output.push_str(",\"verticalOverlapPx\":");
    output.push_str(&format!("{vertical_overlap_px:.3}"));
    output.push_str(",\"centerDeltaYPx\":");
    output.push_str(&format!("{center_delta_y_px:.3}"));
    output.push_str(",\"groupIndex\":");
    match slot.group_index {
        Some(group_index) => output.push_str(&group_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineOffsetUnits\":");
    output.push_str(&slot.line_offset_units.to_string());
    output.push_str(",\"leadingUnits\":");
    output.push_str(&slot.leading_units.to_string());
    output.push_str(",\"fragmentStartUnits\":");
    output.push_str(&slot.fragment_start_units.to_string());
    output.push_str(",\"sourceByteRange\":");
    output.push_str(&source_range_json(
        slot.source_span.byte_start(),
        slot.source_span.byte_end(),
    ));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(
        slot.source_span.unit_start(),
        slot.source_span.unit_end(),
    ));
    output.push_str(",\"leadingWhitespaceBridgeCandidate\":");
    push_fdm_text_mask_leading_whitespace_bridge_candidate_json(
        output,
        source_bbox,
        text_projection,
        slot,
    );
    output.push_str(",\"lineHeaderYPlacementCandidate\":");
    if slot.line_header_same_segment_group_run.is_some() {
        output.push_str("{\"renderPromotionBlockedReason\":\"line-header-y-run-placement-semantics-unproven\",\"renderPromotionBlockedDetail\":");
        output.push_str(&json_string(
            if slot
                .line_header_same_segment_group_run_distinct_text_group_count
                .is_some_and(|count| count > 1)
            {
                "same-segment-run-spans-multiple-visible-text-rows"
            } else {
                "line-header-y-run-transform-undecoded"
            },
        ));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"alignmentPromotionBlockedReason\":\"fdm-text-mask-to-document-text-baseline-transform-unproven\"}");
}

pub(crate) fn fdm_text_mask_pre_fragment_bridge_metrics(
    source_bbox: (f32, f32, f32, f32),
    text_projection: &ShanaiLanTextProjection,
    slot: &ShanaiLanTextSlot,
) -> FdmTextMaskPreFragmentBridgeMetrics {
    let parent_span = &slot.fragment_context.parent_source_span;
    let pre_fragment_unit_count = slot
        .source_span
        .unit_start()
        .saturating_sub(parent_span.unit_start());
    let pre_fragment_grid_units = (slot.leading_units + slot.fragment_start_units) * 2;
    let pre_fragment_projected_width_px =
        pre_fragment_grid_units as f32 * text_projection.grid_unit_px;
    let line_start_x = slot.x - pre_fragment_projected_width_px;
    let text_start_x = slot.x;
    let source_right = source_bbox.0 + source_bbox.2;
    let source_bottom = source_bbox.1 + source_bbox.3;
    let source_begins_after_line_start = source_bbox.0 >= line_start_x - 0.5;
    let source_ends_before_text_start = source_right <= text_start_x + 0.5;
    let source_bbox_within_pre_fragment_projection =
        source_begins_after_line_start && source_ends_before_text_start;
    let text_baseline_y = shanai_lan_text_baseline_y(slot);

    FdmTextMaskPreFragmentBridgeMetrics {
        pre_fragment_unit_count,
        pre_fragment_grid_units,
        pre_fragment_projected_width_px,
        line_start_x,
        text_start_x,
        source_begins_after_line_start,
        source_ends_before_text_start,
        source_bbox_within_pre_fragment_projection,
        source_bbox_right_to_text_start_px: text_start_x - source_right,
        text_baseline_minus_source_bottom_px: text_baseline_y - source_bottom,
    }
}

pub(crate) fn push_fdm_text_mask_leading_whitespace_bridge_candidate_json(
    output: &mut String,
    source_bbox: (f32, f32, f32, f32),
    text_projection: &ShanaiLanTextProjection,
    slot: &ShanaiLanTextSlot,
) {
    let parent_span = &slot.fragment_context.parent_source_span;
    let metrics = fdm_text_mask_pre_fragment_bridge_metrics(source_bbox, text_projection, slot);
    if metrics.pre_fragment_unit_count == 0
        && slot.leading_units == 0
        && slot.fragment_start_units == 0
    {
        output.push_str("null");
        return;
    }

    output.push_str("{\"source\":\"fdmTextMaskBbox+/DocumentText pre-fragment span\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output
        .push_str(",\"candidateClass\":\"fdm-bbox-inside-document-text-pre-fragment-projection\"");
    output.push_str(",\"bridgeCandidate\":");
    output.push_str(
        &metrics
            .source_bbox_within_pre_fragment_projection
            .to_string(),
    );
    output.push_str(",\"parentTextRunSourceByteRange\":");
    output.push_str(&source_range_json(
        parent_span.byte_start(),
        parent_span.byte_end(),
    ));
    output.push_str(",\"parentTextRunSourceUnitRange\":");
    output.push_str(&source_range_json(
        parent_span.unit_start(),
        parent_span.unit_end(),
    ));
    output.push_str(",\"preFragmentSourceByteRange\":");
    output.push_str(&source_range_json(
        parent_span.byte_start(),
        slot.source_span.byte_start(),
    ));
    output.push_str(",\"preFragmentSourceUnitRange\":");
    output.push_str(&source_range_json(
        parent_span.unit_start(),
        slot.source_span.unit_start(),
    ));
    output.push_str(",\"preFragmentUnitCount\":");
    output.push_str(&metrics.pre_fragment_unit_count.to_string());
    output.push_str(",\"leadingDisplayUnits\":");
    output.push_str(&slot.leading_units.to_string());
    output.push_str(",\"fragmentStartUnits\":");
    output.push_str(&slot.fragment_start_units.to_string());
    output.push_str(",\"preFragmentProjectionGridUnits\":");
    output.push_str(&metrics.pre_fragment_grid_units.to_string());
    output.push_str(",\"preFragmentProjectedWidthPx\":");
    output.push_str(&format!("{:.3}", metrics.pre_fragment_projected_width_px));
    output.push_str(",\"lineStartX\":");
    output.push_str(&format!("{:.3}", metrics.line_start_x));
    output.push_str(",\"textStartX\":");
    output.push_str(&format!("{:.3}", metrics.text_start_x));
    output.push_str(",\"sourceBboxOffsetFromLineStartPx\":");
    output.push_str(&format!("{:.3}", source_bbox.0 - metrics.line_start_x));
    output.push_str(",\"sourceBboxEndOffsetFromLineStartPx\":");
    output.push_str(&format!(
        "{:.3}",
        source_bbox.0 + source_bbox.2 - metrics.line_start_x
    ));
    output.push_str(",\"sourceBboxWithinPreFragmentProjection\":");
    output.push_str(
        &metrics
            .source_bbox_within_pre_fragment_projection
            .to_string(),
    );
    output.push_str(",\"sourceBboxBeginsAfterLineStart\":");
    output.push_str(&metrics.source_begins_after_line_start.to_string());
    output.push_str(",\"sourceBboxEndsBeforeTextStart\":");
    output.push_str(&metrics.source_ends_before_text_start.to_string());
    output.push_str(",\"sourceBboxRightToTextStartGapPx\":");
    output.push_str(&format!(
        "{:.3}",
        metrics.source_bbox_right_to_text_start_px
    ));
    output.push_str(",\"textBaselineMinusSourceBottomPx\":");
    output.push_str(&format!(
        "{:.3}",
        metrics.text_baseline_minus_source_bottom_px
    ));
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"document-text-pre-fragment-fdm-mask-role-unproven\"}",
    );
}

pub(crate) fn fdm_text_mask_source_transform_candidates<'a>(
    cohorts: &[FdmTextMaskCohortDiagnosticSummary],
    text_projection: &'a ShanaiLanTextProjection,
) -> Vec<FdmTextMaskSourceTransformCandidate<'a>> {
    if text_projection.grid_unit_px <= 0.0 {
        return Vec::new();
    }

    cohorts
        .iter()
        .filter_map(|cohort| {
            let component = cohort.top_text_like_component?;
            let projected_bbox = component.projected_bbox?;
            let source_bbox = component.source_bbox?;
            if projected_bbox.2 <= 0.0 {
                return None;
            }

            let component_candidates = fdm_text_mask_bbox_right_neighbor_text_slot_candidates(
                projected_bbox,
                text_projection,
            );
            let selected = component_candidates.first().copied()?;
            let cohort_component_agreement = cohort
                .projected_bbox
                .map(|bbox| {
                    fdm_text_mask_bbox_right_neighbor_text_slot_candidates(bbox, text_projection)
                })
                .and_then(|candidates| candidates.first().copied())
                .is_some_and(|cohort_candidate| {
                    fdm_text_mask_right_neighbor_candidates_same_slot(cohort_candidate, selected)
                });
            let metrics = fdm_text_mask_pre_fragment_bridge_metrics(
                projected_bbox,
                text_projection,
                selected.slot,
            );
            if metrics.pre_fragment_unit_count == 0 {
                return None;
            }

            let current_projection_grid_start =
                (projected_bbox.0 - metrics.line_start_x) / text_projection.grid_unit_px;
            let current_projection_grid_end = (projected_bbox.0 + projected_bbox.2
                - metrics.line_start_x)
                / text_projection.grid_unit_px;
            let current_projection_grid_span =
                current_projection_grid_end - current_projection_grid_start;
            if current_projection_grid_span <= 0.0 {
                return None;
            }

            let source_span_x = (source_bbox.2 - source_bbox.0).max(1) as f32;
            let source_units_per_text_grid_unit_x = source_span_x / current_projection_grid_span;
            let line_start_source_x = source_bbox.0 as f32
                - current_projection_grid_start * source_units_per_text_grid_unit_x;
            let text_start_source_x = line_start_source_x
                + metrics.pre_fragment_grid_units as f32 * source_units_per_text_grid_unit_x;
            let source_gap_to_text_start_x = text_start_source_x - source_bbox.2 as f32;

            [
                current_projection_grid_start,
                current_projection_grid_end,
                current_projection_grid_span,
                source_units_per_text_grid_unit_x,
                line_start_source_x,
                text_start_source_x,
                source_gap_to_text_start_x,
            ]
            .into_iter()
            .all(f32::is_finite)
            .then_some(FdmTextMaskSourceTransformCandidate {
                row_index: cohort.row_index,
                candidate_class: "top-text-like-component-to-document-text-pre-fragment",
                component_index: Some(component.component_index),
                slot_index: selected.slot_index,
                slot: selected.slot,
                source_bbox,
                projected_bbox,
                metrics,
                cohort_component_agreement,
                current_projection_grid_start,
                current_projection_grid_end,
                current_projection_grid_span,
                source_units_per_text_grid_unit_x,
                line_start_source_x,
                text_start_source_x,
                source_gap_to_text_start_x,
            })
        })
        .collect()
}

pub(crate) fn fdm_text_mask_cohort_summaries(
    layout: PageLayout,
    diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
) -> Vec<FdmTextMaskCohortDiagnosticSummary> {
    let mut by_row = BTreeMap::<usize, FdmTextMaskCohortDiagnosticSummary>::new();
    let mut primitives_by_row =
        BTreeMap::<usize, Vec<FdmTextMaskPrimitiveDiagnosticSummary>>::new();
    for diagnostic in diagnostics.iter().copied() {
        if !fdm_text_mask_cohort_primitive_candidate(diagnostic.command) {
            continue;
        }
        let Some(bbox) = fdm_path_diagnostic_bbox(layout, diagnostic, extent) else {
            continue;
        };
        if bbox.2 <= 0.0 || bbox.3 <= 0.0 {
            continue;
        }
        let source_bbox =
            fdm_vector_command_source_bbox(diagnostic.command).map(normalize_fdm_bbox);
        let row = by_row
            .entry(diagnostic.entry.row_index())
            .or_insert_with(|| FdmTextMaskCohortDiagnosticSummary {
                row_index: diagnostic.entry.row_index(),
                ..Default::default()
            });
        row.primitive_count += 1;
        let black_fill = diagnostic
            .command
            .fill_color()
            .is_some_and(fdm_vector_color_is_black);
        let white_fill = diagnostic
            .command
            .fill_color()
            .is_some_and(fdm_vector_color_is_white);
        let counter_overlay = fdm_vector_filled_path_is_counter_overlay(diagnostic, diagnostics);
        if black_fill {
            row.black_fill_primitive_count += 1;
        }
        if white_fill {
            row.white_fill_primitive_count += 1;
        }
        if counter_overlay {
            row.counter_overlay_count += 1;
        }
        update_optional_usize_min_max(
            &mut row.command_index_min,
            &mut row.command_index_max,
            diagnostic.command.command_index(),
        );
        update_optional_usize_min_max(
            &mut row.relative_offset_min,
            &mut row.relative_offset_max,
            diagnostic.command.relative_offset(),
        );
        if let Some(source_bbox) = source_bbox {
            row.source_bbox = fdm_bbox_extent_union(row.source_bbox, source_bbox);
        }
        row.projected_bbox = bbox_tuple_union(row.projected_bbox, bbox);
        primitives_by_row
            .entry(diagnostic.entry.row_index())
            .or_default()
            .push(FdmTextMaskPrimitiveDiagnosticSummary {
                command_index: diagnostic.command.command_index(),
                relative_offset: diagnostic.command.relative_offset(),
                source_bbox,
                projected_bbox: bbox,
                black_fill,
                white_fill,
                counter_overlay,
            });
    }

    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let text_line_height_px = 12.0 * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR * scale_y;
    let mut cohorts = by_row
        .into_values()
        .filter_map(|mut cohort| {
            let components = primitives_by_row
                .get(&cohort.row_index)
                .map(|primitives| fdm_text_mask_component_summaries(primitives))
                .unwrap_or_default();
            cohort.component_count = components.len();
            cohort.top_text_like_component =
                fdm_text_mask_top_text_like_component(&components, text_line_height_px);
            (cohort.primitive_count >= FDM_TEXT_MASK_COHORT_MIN_PRIMITIVES
                && cohort.projected_bbox.is_some())
            .then_some(cohort)
        })
        .collect::<Vec<_>>();
    cohorts.sort_by(|left, right| {
        let left_bbox = left.projected_bbox.unwrap_or_default();
        let right_bbox = right.projected_bbox.unwrap_or_default();
        left_bbox
            .1
            .partial_cmp(&right_bbox.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left_bbox
                    .0
                    .partial_cmp(&right_bbox.0)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.row_index.cmp(&right.row_index))
    });
    cohorts.truncate(FDM_TEXT_MASK_COHORT_LIMIT);
    cohorts
}

pub(crate) fn fdm_text_mask_component_summaries(
    primitives: &[FdmTextMaskPrimitiveDiagnosticSummary],
) -> Vec<FdmTextMaskComponentDiagnosticSummary> {
    let mut sorted = primitives.to_vec();
    sorted.sort_by(|left, right| {
        left.projected_bbox
            .1
            .partial_cmp(&right.projected_bbox.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left.projected_bbox
                    .0
                    .partial_cmp(&right.projected_bbox.0)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.relative_offset.cmp(&right.relative_offset))
    });

    let mut components = Vec::<FdmTextMaskComponentDiagnosticSummary>::new();
    for primitive in sorted {
        let component_index = components.iter().position(|component| {
            component.projected_bbox.is_some_and(|bbox| {
                fdm_text_mask_component_bboxes_touch(bbox, primitive.projected_bbox)
            })
        });
        match component_index {
            Some(index) => {
                update_fdm_text_mask_component_summary(&mut components[index], primitive);
                merge_touching_fdm_text_mask_components(&mut components);
            }
            None => {
                let mut component = FdmTextMaskComponentDiagnosticSummary::default();
                update_fdm_text_mask_component_summary(&mut component, primitive);
                components.push(component);
            }
        }
    }

    components.sort_by(|left, right| {
        let left_bbox = left.projected_bbox.unwrap_or_default();
        let right_bbox = right.projected_bbox.unwrap_or_default();
        left_bbox
            .1
            .partial_cmp(&right_bbox.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left_bbox
                    .0
                    .partial_cmp(&right_bbox.0)
                    .unwrap_or(Ordering::Equal)
            })
    });
    for (index, component) in components.iter_mut().enumerate() {
        component.component_index = index;
    }
    components
}

pub(crate) fn update_fdm_text_mask_component_summary(
    component: &mut FdmTextMaskComponentDiagnosticSummary,
    primitive: FdmTextMaskPrimitiveDiagnosticSummary,
) {
    component.primitive_count += 1;
    if primitive.black_fill {
        component.black_fill_primitive_count += 1;
    }
    if primitive.white_fill {
        component.white_fill_primitive_count += 1;
    }
    if primitive.counter_overlay {
        component.counter_overlay_count += 1;
    }
    update_optional_usize_min_max(
        &mut component.command_index_min,
        &mut component.command_index_max,
        primitive.command_index,
    );
    update_optional_usize_min_max(
        &mut component.relative_offset_min,
        &mut component.relative_offset_max,
        primitive.relative_offset,
    );
    if let Some(source_bbox) = primitive.source_bbox {
        component.source_bbox = fdm_bbox_extent_union(component.source_bbox, source_bbox);
    }
    component.projected_bbox = bbox_tuple_union(component.projected_bbox, primitive.projected_bbox);
}

pub(crate) fn merge_touching_fdm_text_mask_components(
    components: &mut Vec<FdmTextMaskComponentDiagnosticSummary>,
) {
    let mut index = 0usize;
    while index < components.len() {
        let mut merge_index = index + 1;
        while merge_index < components.len() {
            let Some(left_bbox) = components[index].projected_bbox else {
                break;
            };
            let Some(right_bbox) = components[merge_index].projected_bbox else {
                merge_index += 1;
                continue;
            };
            if !fdm_text_mask_component_bboxes_touch(left_bbox, right_bbox) {
                merge_index += 1;
                continue;
            }
            let right = components.remove(merge_index);
            merge_fdm_text_mask_component_summary(&mut components[index], right);
        }
        index += 1;
    }
}

pub(crate) fn merge_fdm_text_mask_component_summary(
    target: &mut FdmTextMaskComponentDiagnosticSummary,
    source: FdmTextMaskComponentDiagnosticSummary,
) {
    target.primitive_count += source.primitive_count;
    target.black_fill_primitive_count += source.black_fill_primitive_count;
    target.white_fill_primitive_count += source.white_fill_primitive_count;
    target.counter_overlay_count += source.counter_overlay_count;
    if let Some(value) = source.command_index_min {
        update_optional_usize_min_max(
            &mut target.command_index_min,
            &mut target.command_index_max,
            value,
        );
    }
    if let Some(value) = source.command_index_max {
        update_optional_usize_min_max(
            &mut target.command_index_min,
            &mut target.command_index_max,
            value,
        );
    }
    if let Some(value) = source.relative_offset_min {
        update_optional_usize_min_max(
            &mut target.relative_offset_min,
            &mut target.relative_offset_max,
            value,
        );
    }
    if let Some(value) = source.relative_offset_max {
        update_optional_usize_min_max(
            &mut target.relative_offset_min,
            &mut target.relative_offset_max,
            value,
        );
    }
    if let Some(source_bbox) = source.projected_bbox {
        target.projected_bbox = bbox_tuple_union(target.projected_bbox, source_bbox);
    }
    if let Some(source_bbox) = source.source_bbox {
        target.source_bbox = fdm_bbox_extent_union(target.source_bbox, source_bbox);
    }
}

pub(crate) fn fdm_text_mask_component_bboxes_touch(
    left: (f32, f32, f32, f32),
    right: (f32, f32, f32, f32),
) -> bool {
    let horizontal_gap = bbox_axis_gap(left.0, left.0 + left.2, right.0, right.0 + right.2);
    let vertical_overlap = (left.1 + left.3).min(right.1 + right.3) - left.1.max(right.1);
    let max_horizontal_gap = left.3.max(right.3) * 0.85;
    vertical_overlap > 0.0 && horizontal_gap <= max_horizontal_gap
}

pub(crate) fn fdm_text_mask_top_text_like_component(
    components: &[FdmTextMaskComponentDiagnosticSummary],
    text_line_height_px: f32,
) -> Option<FdmTextMaskComponentDiagnosticSummary> {
    components
        .iter()
        .copied()
        .filter(|component| {
            let Some(bbox) = component.projected_bbox else {
                return false;
            };
            component.primitive_count >= FDM_TEXT_MASK_COMPONENT_MIN_PRIMITIVES
                && component.black_fill_primitive_count > 0
                && bbox.3 <= text_line_height_px * FDM_TEXT_MASK_COMPONENT_MAX_HEIGHT_LINE_FACTOR
        })
        .min_by(|left, right| {
            let left_bbox = left.projected_bbox.unwrap_or_default();
            let right_bbox = right.projected_bbox.unwrap_or_default();
            left_bbox
                .1
                .partial_cmp(&right_bbox.1)
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    left_bbox
                        .0
                        .partial_cmp(&right_bbox.0)
                        .unwrap_or(Ordering::Equal)
                })
        })
}

pub(crate) fn fdm_text_mask_cohort_primitive_candidate(
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    command.ellipse().is_none()
        && fdm_vector_primitive_is_closed(command)
        && command.fill_color().is_some_and(|color| {
            fdm_vector_color_is_black(color) || fdm_vector_color_is_white(color)
        })
}

pub(crate) fn fdm_text_mask_cohort_right_neighbor_text_slot<'a>(
    cohort: &FdmTextMaskCohortDiagnosticSummary,
    text_projection: &'a ShanaiLanTextProjection,
) -> Option<FdmTextMaskRightNeighborMatch<'a>> {
    fdm_text_mask_bbox_right_neighbor_text_slot(cohort.projected_bbox?, text_projection)
}

pub(crate) fn fdm_text_mask_bbox_right_neighbor_text_slot(
    source_bbox: (f32, f32, f32, f32),
    text_projection: &ShanaiLanTextProjection,
) -> Option<FdmTextMaskRightNeighborMatch<'_>> {
    fdm_text_mask_bbox_right_neighbor_text_slot_candidates(source_bbox, text_projection)
        .into_iter()
        .next()
        .map(|candidate| {
            (
                candidate.slot,
                candidate.bbox,
                candidate.gap_px,
                candidate.vertical_overlap_px,
                candidate.center_delta_y_px,
            )
        })
}

pub(crate) fn fdm_text_mask_bbox_right_neighbor_text_slot_candidates<'a>(
    source_bbox: (f32, f32, f32, f32),
    text_projection: &'a ShanaiLanTextProjection,
) -> Vec<FdmTextMaskRightNeighborCandidate<'a>> {
    let source_right = source_bbox.0 + source_bbox.2;
    let source_bottom = source_bbox.1 + source_bbox.3;
    let source_center_y = source_bbox.1 + source_bbox.3 * 0.5;
    let max_gap_px = text_projection.line_height_px * FDM_TEXT_MASK_RIGHT_NEIGHBOR_MAX_GAP_FACTOR;
    let mut candidates = text_projection
        .slots
        .iter()
        .enumerate()
        .filter_map(|(slot_index, slot)| {
            let bbox = shanai_lan_text_slot_bbox(slot);
            let gap_px = bbox.0 - source_right;
            if gap_px < -text_projection.line_height_px || gap_px > max_gap_px {
                return None;
            }
            let vertical_overlap_px =
                source_bottom.min(bbox.1 + bbox.3) - source_bbox.1.max(bbox.1);
            let text_center_y = bbox.1 + bbox.3 * 0.5;
            let center_delta_y_px = text_center_y - source_center_y;
            if vertical_overlap_px <= 0.0
                && center_delta_y_px.abs() > text_projection.line_height_px
            {
                return None;
            }
            Some(FdmTextMaskRightNeighborCandidate {
                slot_index,
                slot,
                bbox,
                gap_px,
                vertical_overlap_px: vertical_overlap_px.max(0.0),
                center_delta_y_px,
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.gap_px
            .abs()
            .partial_cmp(&right.gap_px.abs())
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left.center_delta_y_px
                    .abs()
                    .partial_cmp(&right.center_delta_y_px.abs())
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.slot_index.cmp(&right.slot_index))
    });
    candidates
}

pub(crate) fn push_fdm_vector_command_provenance_json(
    output: &mut String,
    command: &ObjectFdmVectorCommandCandidate,
) {
    output.push_str(",\"sourceVectorRelativeOffset\":");
    push_optional_usize_json(output, command.source_vector_relative_offset());
    output.push_str(",\"sourceSegment\":");
    if let Some(source_segment) = command.source_segment() {
        push_object_fdm_vector_command_source_segment_json(output, source_segment);
    } else {
        output.push_str("null");
    }
}
