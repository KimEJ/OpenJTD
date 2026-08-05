use super::*;
use crate::*;

pub(crate) fn push_fdm_connector_parent_compound_provenance_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
) {
    let Some(provenance) = fdm_connector_parent_compound_provenance(diagnostic) else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"basis\":\"synthetic-nested-command-index+relative-offset\"");
    output.push_str(",\"sourceBacked\":true,\"decoded\":false");
    output.push_str(",\"parentCommandIndex\":");
    output.push_str(&provenance.parent.command_index().to_string());
    output.push_str(",\"parentRelativeOffset\":");
    output.push_str(&provenance.parent.relative_offset().to_string());
    output.push_str(",\"parentRecordLength\":");
    output.push_str(&provenance.parent.record_len().to_string());
    output.push_str(",\"parentDeclaredRecordLength\":");
    output.push_str(&provenance.parent.declared_record_len().to_string());
    output.push_str(",\"parentMarkerHex\":");
    output.push_str(&json_string(&hex_bytes(provenance.parent.marker())));
    output.push_str(",\"parentStyleWord\":");
    output.push_str(&provenance.parent.style_word().to_string());
    output.push_str(",\"parentStyleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        provenance.parent.style_word()
    )));
    output.push_str(",\"parentCompoundChildOffsets\":");
    push_u16_array_json(output, provenance.parent.compound_child_offsets());
    output.push_str(",\"childOffsetInParent\":");
    output.push_str(&provenance.child_offset_in_parent.to_string());
    output.push_str(",\"childOffsetTableIndex\":");
    push_option_usize_json(output, provenance.child_offset_table_index);
    output.push_str(",\"childOffsetTableMatched\":");
    output.push_str(if provenance.child_offset_table_index.is_some() {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_page_layer_fdm_connector_graph_diagnostic_summary_json(
    output: &mut String,
    layout: PageLayout,
    summary: FdmConnectorGraphDiagnosticSummary,
) {
    output.push_str("{\"type\":\"fdmConnectorGraphDiagnosticSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(
        ",\"source\":\"fdmVectorCommandConnectorCandidate+documentTextLineRuleProjection\"",
    );
    output.push_str(",\"projectionKind\":\"fdmConnectorGraphDiagnosticSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(summary.render_promotion_blocked_reason()));
    output.push_str(",\"pagePaintCoverageSummary\":");
    push_fdm_page_paint_coverage_summary_json(output, summary.page_paint_coverage_summary);
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTraceSummary\":");
    push_fdm_connector_order_trace_summary_json(
        output,
        summary.same_row_axis_rule_connector_order_trace_summary,
    );
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"lineRuleProjectionCount\":");
    output.push_str(&summary.line_rule_projection_count.to_string());
    output.push_str(",\"fdmOpenStrokeAxisRuleProjectionCount\":");
    output.push_str(
        &summary
            .fdm_open_stroke_axis_rule_projection_count
            .to_string(),
    );
    output.push_str(",\"connectorEndpointProbeCount\":");
    output.push_str(&summary.connector_endpoint_probe_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&summary.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&summary.matched_connector_count.to_string());
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"startEndpointLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .start_endpoint_line_rule_match_connector_count
            .to_string(),
    );
    output.push_str(",\"endEndpointLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .end_endpoint_line_rule_match_connector_count
            .to_string(),
    );
    output.push_str(",\"startOnlyLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .start_only_line_rule_match_connector_count
            .to_string(),
    );
    output.push_str(",\"endOnlyLineRuleMatchConnectorCount\":");
    output.push_str(&summary.end_only_line_rule_match_connector_count.to_string());
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&summary.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&summary.nearby_endpoint_match_count.to_string());
    output.push_str(",\"noThresholdedLineRuleEndpointMatchConnectorCount\":");
    output.push_str(
        &summary
            .no_thresholded_line_rule_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"singleOrMissingEndpointLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .single_or_missing_endpoint_line_rule_match_connector_count
            .to_string(),
    );
    output.push_str(",\"connectorOwnershipAndPaintOrderUnprovenConnectorCount\":");
    output.push_str(
        &summary
            .connector_ownership_and_paint_order_unproven_connector_count
            .to_string(),
    );
    output.push_str(",\"endpointOwnerCandidateConnectorCount\":");
    output.push_str(&summary.endpoint_owner_candidate_connector_count.to_string());
    output.push_str(",\"endpointOwnerProbeCount\":");
    output.push_str(&summary.endpoint_owner_probe_count.to_string());
    output.push_str(",\"totalEndpointOwnerCandidateCount\":");
    output.push_str(&summary.total_endpoint_owner_candidate_count.to_string());
    output.push_str(",\"withinProbeEndpointOwnerCandidateCount\":");
    output.push_str(
        &summary
            .within_probe_endpoint_owner_candidate_count
            .to_string(),
    );
    output.push_str(",\"fdmPrimitiveEndpointOwnerCandidateCount\":");
    output.push_str(
        &summary
            .fdm_primitive_endpoint_owner_candidate_count
            .to_string(),
    );
    output.push_str(",\"documentTextSlotEndpointOwnerCandidateCount\":");
    output.push_str(
        &summary
            .document_text_slot_endpoint_owner_candidate_count
            .to_string(),
    );
    output.push_str(",\"startEndpointOwnerWithinProbeConnectorCount\":");
    output.push_str(
        &summary
            .start_endpoint_owner_within_probe_connector_count
            .to_string(),
    );
    output.push_str(",\"endEndpointOwnerWithinProbeConnectorCount\":");
    output.push_str(
        &summary
            .end_endpoint_owner_within_probe_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointOwnerWithinProbeConnectorCount\":");
    output.push_str(
        &summary
            .dual_endpoint_owner_within_probe_connector_count
            .to_string(),
    );
    output.push_str(",\"ownerProvenConnectorCount\":");
    output.push_str(&summary.owner_proven_connector_count.to_string());
    output.push_str(",\"dualEndpointNearestFdmOwnerSameRowConnectorCount\":");
    output.push_str(
        &summary
            .dual_endpoint_nearest_fdm_owner_same_row_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointNearestFdmOwnerRowMismatchConnectorCount\":");
    output.push_str(
        &summary
            .dual_endpoint_nearest_fdm_owner_row_mismatch_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointNearestFdmOwnerSameConnectorRowCount\":");
    output.push_str(
        &summary
            .dual_endpoint_nearest_fdm_owner_same_connector_row_count
            .to_string(),
    );
    output.push_str(",\"connectorCommandBetweenNearestFdmOwnerCommandsCount\":");
    output.push_str(
        &summary
            .connector_command_between_nearest_fdm_owner_commands_count
            .to_string(),
    );
    output.push_str(",\"connectorCommandBeforeNearestFdmOwnerCommandsCount\":");
    output.push_str(
        &summary
            .connector_command_before_nearest_fdm_owner_commands_count
            .to_string(),
    );
    output.push_str(",\"connectorCommandAfterNearestFdmOwnerCommandsCount\":");
    output.push_str(
        &summary
            .connector_command_after_nearest_fdm_owner_commands_count
            .to_string(),
    );
    output.push_str(",\"orderedSameRowSameConnectorCount\":");
    output.push_str(&summary.ordered_same_row_same_connector_count.to_string());
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnectorCount\":");
    output.push_str(
        &summary
            .parent_normalized_ordered_same_row_same_connector_count
            .to_string(),
    );
    output.push_str(",\"missingEndpointOwnerCandidateConnectorCount\":");
    output.push_str(
        &summary
            .missing_endpoint_owner_candidate_connector_count
            .to_string(),
    );
    output.push_str(",\"nearestOwnerRowMismatchConnectorCount\":");
    output.push_str(
        &summary
            .nearest_owner_row_mismatch_connector_count
            .to_string(),
    );
    output.push_str(",\"ownerRowCandidateUnprovenConnectorCount\":");
    output.push_str(
        &summary
            .owner_row_candidate_unproven_connector_count
            .to_string(),
    );
    output.push_str(",\"ownerGroupingProvenConnectorCount\":");
    output.push_str(&summary.owner_grouping_proven_connector_count.to_string());
    output.push_str(",\"lineRuleEndpointMatchProvenanceSummaries\":[");
    push_fdm_connector_rule_set_match_summary_json(
        output,
        "allDocumentTextLineRules",
        None,
        summary.all_line_rule_match_summary(),
    );
    output.push(',');
    push_fdm_connector_rule_set_match_summary_json(
        output,
        "skippedInlineLineHeaderOnly",
        Some("skippedInlineLineHeader"),
        summary.skipped_inline_line_rule_match_summary,
    );
    output.push(',');
    push_fdm_connector_rule_set_match_summary_json(
        output,
        "verticalAnchorRunFromLineHeadersOnly",
        Some("verticalAnchorRunFromLineHeaders"),
        summary.vertical_anchor_line_rule_match_summary,
    );
    output.push(',');
    push_fdm_connector_rule_set_match_summary_json(
        output,
        "sameRowFdmOpenStrokeAxisRules",
        Some("fdmOpenStrokeAxisRule"),
        summary.fdm_open_stroke_axis_rule_match_summary,
    );
    output.push(']');
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleRowCohorts\":");
    push_fdm_open_stroke_axis_rule_row_cohorts_json(output, summary);
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleOwnerPromotionGateSummary\":");
    push_fdm_open_stroke_axis_rule_owner_promotion_gate_summary_json(
        output,
        summary.fdm_open_stroke_axis_rule_owner_promotion_gate_summary,
    );
    output.push_str(",\"ownerRowCohortEndpointMatchSummaries\":[");
    push_fdm_connector_owner_row_cohort_summary_json(
        output,
        "orderedSameRowSameConnector",
        summary.ordered_owner_row_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_row_cohort_summary_json(
        output,
        "notOrderedSameRowSameConnector",
        summary.non_ordered_owner_row_match_summary,
    );
    output.push(']');
    output.push_str(",\"parentNormalizedOwnerRowCohortEndpointMatchSummaries\":[");
    push_fdm_connector_owner_row_cohort_summary_json(
        output,
        "parentNormalizedOrderedSameRowSameConnector",
        summary.parent_normalized_ordered_owner_row_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_row_cohort_summary_json(
        output,
        "notParentNormalizedOrderedSameRowSameConnector",
        summary.parent_normalized_non_ordered_owner_row_match_summary,
    );
    output.push(']');
    output.push_str(",\"ownerCommandRelationEndpointMatchSummaries\":[");
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_relation_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_relation_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "nearest-owner-row-not-connector-row",
        summary.nearest_owner_row_not_connector_row_relation_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-mixed-command-namespace",
        summary.same_row_mixed_command_namespace_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-before-owner-command-span",
        summary.same_row_before_owner_command_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-between-owner-command-span",
        summary.same_row_between_owner_command_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-after-owner-command-span",
        summary.same_row_after_owner_command_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-command-relation-unclassified",
        summary.same_row_owner_command_relation_unclassified_match_summary,
    );
    output.push(']');
    output.push_str(",\"ownerSourceOrderRelationEndpointMatchSummaries\":[");
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_source_order_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_source_order_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "nearest-owner-row-not-connector-row",
        summary.nearest_owner_row_not_connector_row_source_order_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-before-owner-relative-offset-span",
        summary.same_row_before_owner_relative_offset_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-between-owner-relative-offset-span",
        summary.same_row_between_owner_relative_offset_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-after-owner-relative-offset-span",
        summary.same_row_after_owner_relative_offset_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-relative-offset-relation-unclassified",
        summary.same_row_relative_offset_relation_unclassified_match_summary,
    );
    output.push(']');
    output.push_str(",\"ownerGroupingPromotionBlockedReasonCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_candidate_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "owner-row-candidate-unproven",
        summary.owner_row_candidate_unproven_connector_count,
    );
    output.push(']');
    output.push_str(",\"graphPromotionBlockedReasonCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "no-thresholded-line-rule-endpoint-match",
        summary.no_thresholded_line_rule_endpoint_match_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "single-or-missing-endpoint-line-rule-match",
        summary.single_or_missing_endpoint_line_rule_match_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "connector-ownership-and-paint-order-unproven",
        summary.connector_ownership_and_paint_order_unproven_connector_count,
    );
    output.push_str("],\"dominantMatchedConnectorRow\":");
    push_fdm_connector_dominant_matched_connector_row_json(output, summary);
    output.push_str(",\"endpointMatchThresholds\":{\"basis\":\"documentTextLineHeaderGrid\"");
    output.push_str(",\"spanOverflowProbeUnits\":");
    output.push_str(&format!(
        "{:.3}",
        FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS
    ));
    output.push_str(",\"tightPerpendicularProbeUnits\":");
    output.push_str(&format!(
        "{:.3}",
        FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS
    ));
    output.push_str(",\"nearbyPerpendicularProbeUnits\":");
    output.push_str(&format!(
        "{:.3}",
        FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS
    ));
    output.push_str(",\"attachmentProven\":false}");
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(crate) fn push_fdm_connector_rule_set_match_summary_json(
    output: &mut String,
    rule_set: &str,
    candidate_source: Option<&str>,
    summary: FdmConnectorRuleSetMatchDiagnosticSummary,
) {
    output.push_str("{\"ruleSet\":");
    output.push_str(&json_string(rule_set));
    output.push_str(",\"candidateSource\":");
    match candidate_source {
        Some(candidate_source) => output.push_str(&json_string(candidate_source)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineRuleProjectionCount\":");
    output.push_str(&summary.line_rule_projection_count.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"connectorEndpointProbeCount\":");
    output.push_str(&summary.connector_endpoint_probe_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&summary.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&summary.matched_connector_count.to_string());
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&summary.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&summary.nearby_endpoint_match_count.to_string());
    output.push_str(",\"graphPromotionBlockedReasonCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "no-thresholded-line-rule-endpoint-match",
        summary.no_thresholded_line_rule_endpoint_match_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "single-or-missing-endpoint-line-rule-match",
        summary.single_or_missing_endpoint_line_rule_match_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "connector-ownership-and-paint-order-unproven",
        summary.connector_ownership_and_paint_order_unproven_connector_count,
    );
    output.push_str("]}");
}

pub(crate) fn push_fdm_open_stroke_axis_rule_row_cohorts_json(
    output: &mut String,
    summary: FdmConnectorGraphDiagnosticSummary,
) {
    let row_cohorts = summary
        .fdm_open_stroke_axis_rule_row_cohorts
        .iter()
        .take(summary.fdm_open_stroke_axis_rule_row_cohort_count)
        .copied()
        .collect::<Vec<_>>();
    let tight_non_diagonal_dual_candidate_count = row_cohorts
        .iter()
        .map(|row| row.non_diagonal_tight_dual_endpoint_match_connector_count())
        .sum::<usize>();
    let tight_non_diagonal_dual_row_cohort_count = row_cohorts
        .iter()
        .filter(|row| row.non_diagonal_tight_dual_endpoint_match_connector_count() > 0)
        .count();
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+connectorRowIndex\"");
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"",
    );
    output.push_str(",\"rowCohortLimit\":");
    output.push_str(&FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT.to_string());
    output.push_str(",\"rowCohortCount\":");
    output.push_str(
        &summary
            .fdm_open_stroke_axis_rule_row_cohort_count
            .to_string(),
    );
    output.push_str(",\"renderReadinessPredicate\":");
    push_fdm_open_stroke_axis_rule_render_readiness_predicate_json(
        output,
        tight_non_diagonal_dual_candidate_count,
        tight_non_diagonal_dual_row_cohort_count,
    );
    output.push_str(",\"rowCohorts\":[");
    for (index, row) in row_cohorts.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_open_stroke_axis_rule_row_cohort_json(output, row);
    }
    output.push_str("]}");
}

pub(crate) fn push_fdm_open_stroke_axis_rule_render_readiness_predicate_json(
    output: &mut String,
    candidate_count: usize,
    row_cohort_count: usize,
) {
    output.push_str(
        "{\"basis\":\"sameRowFdmOpenStrokeAxisRule+tightDualEndpoint+nonDiagonalConnector\"",
    );
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive+fdmConnectorEndpointOwnerMatch\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"requiresTightDualEndpointMatch\":true");
    output.push_str(",\"excludesDiagonalConnectors\":true");
    output.push_str(",\"requiresDualEndpointOwnerCandidate\":true");
    output.push_str(",\"requiresNearestFdmOwnerRowsMatch\":true");
    output.push_str(",\"requiresNearestFdmOwnerRowMatchesConnectorRow\":true");
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidate_count.to_string());
    output.push_str(",\"rowCohortCount\":");
    output.push_str(&row_cohort_count.to_string());
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"}",
    );
}

pub(crate) fn push_fdm_open_stroke_axis_rule_row_cohort_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&row.connector_candidate_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&row.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&row.matched_connector_count.to_string());
    output.push_str(",\"fdmIndexSegmentGate\":");
    push_fdm_open_stroke_axis_rule_index_segment_gate_json(output, row);
    output.push_str(",\"fdmIndexConnectorCompositionGate\":");
    push_fdm_open_stroke_axis_rule_index_connector_composition_gate_json(output, row);
    output.push_str(",\"fdmIndexBboxRelationGate\":");
    push_fdm_open_stroke_axis_rule_index_bbox_relation_gate_json(output, row);
    output.push_str(",\"axisRuleSourceOrderGate\":");
    push_fdm_open_stroke_axis_rule_source_order_gate_json(output, row);
    output.push_str(",\"matchedProjectedBboxUnion\":");
    push_optional_bbox_milli_json(
        output,
        row.matched_projected_bbox_x_min_milli,
        row.matched_projected_bbox_y_min_milli,
        row.matched_projected_bbox_x_max_milli,
        row.matched_projected_bbox_y_max_milli,
    );
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&row.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"dualEndpointProjectedBboxUnion\":");
    push_optional_bbox_milli_json(
        output,
        row.dual_projected_bbox_x_min_milli,
        row.dual_projected_bbox_y_min_milli,
        row.dual_projected_bbox_x_max_milli,
        row.dual_projected_bbox_y_max_milli,
    );
    output.push_str(",\"nonDiagonalDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.non_diagonal_dual_endpoint_match_connector_count()
            .to_string(),
    );
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&row.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&row.nearby_endpoint_match_count.to_string());
    output.push_str(",\"tightDualEndpointMatchConnectorCount\":");
    output.push_str(&row.tight_dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"nonDiagonalTightDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.non_diagonal_tight_dual_endpoint_match_connector_count()
            .to_string(),
    );
    output.push_str(",\"tightNonDiagonalDualEndpointProjectedBboxUnion\":");
    push_optional_bbox_milli_json(
        output,
        row.tight_non_diagonal_dual_projected_bbox_x_min_milli,
        row.tight_non_diagonal_dual_projected_bbox_y_min_milli,
        row.tight_non_diagonal_dual_projected_bbox_x_max_milli,
        row.tight_non_diagonal_dual_projected_bbox_y_max_milli,
    );
    output.push_str(",\"horizontalDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.horizontal_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"verticalDualEndpointMatchConnectorCount\":");
    output.push_str(&row.vertical_dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"diagonalDualEndpointMatchConnectorCount\":");
    output.push_str(&row.diagonal_dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"horizontalTightDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.horizontal_tight_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"verticalTightDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.vertical_tight_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"diagonalTightDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.diagonal_tight_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"matchedConnectorMarkerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(
        output,
        row.matched_connector_marker_style_profile,
    );
    output.push_str(",\"dualConnectorMarkerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(output, row.dual_connector_marker_style_profile);
    output.push_str(",\"tightNonDiagonalDualConnectorMarkerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(
        output,
        row.tight_non_diagonal_dual_connector_marker_style_profile,
    );
    output.push_str(",\"axisRuleEndpointMatchMarkerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(
        output,
        row.axis_rule_endpoint_match_marker_style_profile,
    );
    output.push_str(",\"markerStyleAgreementGate\":");
    push_fdm_open_stroke_axis_rule_marker_style_agreement_gate_json(output, row);
    output.push_str(",\"ownerPromotionGate\":");
    if row
        .owner_promotion_gate_summary
        .dual_endpoint_match_connector_count
        > 0
    {
        push_fdm_open_stroke_axis_rule_owner_promotion_gate_summary_json(
            output,
            row.owner_promotion_gate_summary,
        );
    } else {
        output.push_str("null");
    }
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"}",
    );
}

pub(crate) fn push_fdm_open_stroke_axis_rule_marker_style_agreement_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    let connector_profile = row.tight_non_diagonal_dual_connector_marker_style_profile;
    let axis_rule_profile = row.axis_rule_endpoint_match_marker_style_profile;
    let (connector_marker_family, connector_marker_family_count) =
        connector_profile.dominant_marker_family();
    let (axis_rule_marker_family, axis_rule_marker_family_count) =
        axis_rule_profile.dominant_marker_family();
    let (connector_style_word, connector_style_word_count) =
        connector_profile.dominant_style_word();
    let (axis_rule_style_word, axis_rule_style_word_count) =
        axis_rule_profile.dominant_style_word();
    let dominant_marker_family_matches = connector_profile.command_count > 0
        && axis_rule_profile.command_count > 0
        && connector_marker_family == axis_rule_marker_family;
    let dominant_style_word_matches = connector_profile.command_count > 0
        && axis_rule_profile.command_count > 0
        && connector_style_word == axis_rule_style_word;
    let marker_style_agreement_candidate =
        dominant_marker_family_matches && dominant_style_word_matches;

    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+markerStyleAgreement\"");
    output.push_str(",\"source\":\"FDMVector.marker+styleWord\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"connectorProfile\":\"tightNonDiagonalDualConnectorMarkerStyleProfile\"");
    output.push_str(",\"axisRuleProfile\":\"axisRuleEndpointMatchMarkerStyleProfile\"");
    output.push_str(",\"connectorCommandCount\":");
    output.push_str(&connector_profile.command_count.to_string());
    output.push_str(",\"axisRuleCommandCount\":");
    output.push_str(&axis_rule_profile.command_count.to_string());
    output.push_str(",\"connectorDominantMarkerFamily\":");
    output.push_str(&json_string(connector_marker_family));
    output.push_str(",\"connectorDominantMarkerFamilyCount\":");
    output.push_str(&connector_marker_family_count.to_string());
    output.push_str(",\"axisRuleDominantMarkerFamily\":");
    output.push_str(&json_string(axis_rule_marker_family));
    output.push_str(",\"axisRuleDominantMarkerFamilyCount\":");
    output.push_str(&axis_rule_marker_family_count.to_string());
    output.push_str(",\"dominantMarkerFamilyMatches\":");
    output.push_str(if dominant_marker_family_matches {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorDominantStyleWord\":");
    output.push_str(&json_string(connector_style_word));
    output.push_str(",\"connectorDominantStyleWordCount\":");
    output.push_str(&connector_style_word_count.to_string());
    output.push_str(",\"axisRuleDominantStyleWord\":");
    output.push_str(&json_string(axis_rule_style_word));
    output.push_str(",\"axisRuleDominantStyleWordCount\":");
    output.push_str(&axis_rule_style_word_count.to_string());
    output.push_str(",\"dominantStyleWordMatches\":");
    output.push_str(if dominant_style_word_matches {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorMarkerFamilyHomogeneous\":");
    output.push_str(if connector_profile.marker_family_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"axisRuleMarkerFamilyHomogeneous\":");
    output.push_str(if axis_rule_profile.marker_family_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorStyleWordHomogeneous\":");
    output.push_str(if connector_profile.style_word_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"axisRuleStyleWordHomogeneous\":");
    output.push_str(if axis_rule_profile.style_word_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"markerStyleAgreementCandidate\":");
    output.push_str(if marker_style_agreement_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        fdm_open_stroke_axis_rule_marker_style_agreement_blocked_reason(
            connector_profile,
            axis_rule_profile,
        ),
    ));
    output.push('}');
}

pub(crate) fn fdm_open_stroke_axis_rule_marker_style_agreement_blocked_reason(
    connector_profile: FdmOpenStrokeMarkerStyleProfile,
    axis_rule_profile: FdmOpenStrokeMarkerStyleProfile,
) -> &'static str {
    if connector_profile.command_count == 0 {
        "connector-marker-style-profile-empty"
    } else if axis_rule_profile.command_count == 0 {
        "axis-rule-marker-style-profile-empty"
    } else if connector_profile.dominant_marker_family().0
        != axis_rule_profile.dominant_marker_family().0
        && connector_profile.dominant_style_word().0 != axis_rule_profile.dominant_style_word().0
    {
        "connector-axis-rule-marker-and-style-dominance-mismatch"
    } else if connector_profile.dominant_marker_family().0
        != axis_rule_profile.dominant_marker_family().0
    {
        "connector-axis-rule-marker-family-dominance-mismatch"
    } else if connector_profile.dominant_style_word().0 != axis_rule_profile.dominant_style_word().0
    {
        "connector-axis-rule-style-word-dominance-mismatch"
    } else if !connector_profile.homogeneous_marker_style_candidate()
        || !axis_rule_profile.homogeneous_marker_style_candidate()
    {
        "matched-dominant-marker-style-still-mixed"
    } else {
        "marker-style-agreement-still-needs-owner-and-paint-order"
    }
}

pub(crate) fn push_fdm_open_stroke_axis_rule_index_segment_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexSegmentMembership\"");
    output.push_str(",\"source\":\"FDMIndex.vectorOffset+FDMVector.sourceSegment\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(row.fdm_index_segment_gate_blocked_reason()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"vectorOffset\":");
    push_option_usize_json(output, row.fdm_index_vector_offset);
    output.push_str(",\"vectorLength\":");
    push_option_usize_json(output, row.fdm_index_vector_len);
    output.push_str(",\"validVectorOffset\":");
    output.push_str(if row.fdm_index_valid_vector_offset {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceSegmentRelativeOffset\":");
    push_option_usize_json(output, row.fdm_index_source_segment_relative_offset);
    output.push_str(",\"sourceSegmentCommandCount\":");
    push_option_usize_json(output, row.fdm_index_source_segment_command_count);
    output.push_str(",\"imageSignatureCount\":");
    output.push_str(&row.fdm_index_image_signature_count.to_string());
    output.push_str(",\"segmentImageSignatureCount\":");
    output.push_str(&row.fdm_index_segment_image_signature_count.to_string());
    output.push_str(",\"imageBearingSegmentCandidate\":");
    output.push_str(if row.image_bearing_segment_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceSegmentBackedConnectorCount\":");
    output.push_str(&row.source_segment_backed_connector_count.to_string());
    output.push_str(",\"sourceSegmentMatchesIndexEntryConnectorCount\":");
    output.push_str(
        &row.source_segment_matches_index_entry_connector_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentMissingConnectorCount\":");
    output.push_str(&row.source_segment_missing_connector_count.to_string());
    output.push_str(",\"dualEndpointSourceSegmentBackedConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_source_segment_backed_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointSourceSegmentMatchesIndexEntryConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_source_segment_matches_index_entry_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointImageBearingSegmentConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_image_bearing_segment_connector_count
            .to_string(),
    );
    output.push('}');
}

pub(crate) fn push_fdm_open_stroke_axis_rule_source_order_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+parentRelativeSourceOrder\"");
    output.push_str(",\"source\":\"FDMVector.commandRelativeOffset+compoundParentRelativeOffset\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        row.axis_rule_source_order_gate_blocked_reason(),
    ));
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"dualEndpointConnectorCount\":");
    output.push_str(&row.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"sourceOrderBackedDualEndpointConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_axis_rule_source_order_backed_connector_count
            .to_string(),
    );
    output.push_str(",\"connectorParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        row.dual_endpoint_connector_parent_relative_offset_min,
        row.dual_endpoint_connector_parent_relative_offset_max,
    );
    output.push_str(",\"axisRuleParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        row.dual_endpoint_axis_rule_parent_relative_offset_min,
        row.dual_endpoint_axis_rule_parent_relative_offset_max,
    );
    output.push_str(",\"connectorBeforeAxisRuleParentSpanCount\":");
    output.push_str(
        &row.dual_endpoint_connector_before_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"connectorBetweenAxisRuleParentSpanCount\":");
    output.push_str(
        &row.dual_endpoint_connector_between_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"connectorAfterAxisRuleParentSpanCount\":");
    output.push_str(
        &row.dual_endpoint_connector_after_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"connectorAxisRuleParentSpanUnclassifiedCount\":");
    output.push_str(
        &row.dual_endpoint_connector_axis_rule_parent_span_unclassified_count
            .to_string(),
    );
    output.push('}');
}

pub(crate) fn push_fdm_open_stroke_axis_rule_index_connector_composition_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    let vector_command_count = row.fdm_index_vector_command_count.unwrap_or_default();
    let connector_candidate_count = row.fdm_index_connector_candidate_count.unwrap_or_default();
    let non_connector_command_count = row
        .fdm_index_non_connector_command_count
        .unwrap_or_default();
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexConnectorComposition\"");
    output.push_str(",\"source\":\"FDMIndex.vectorCommands+FDMIndex.connectorCandidates\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        row.fdm_index_connector_composition_gate_blocked_reason(),
    ));
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"vectorCommandCount\":");
    push_option_usize_json(output, row.fdm_index_vector_command_count);
    output.push_str(",\"connectorCandidateCount\":");
    push_option_usize_json(output, row.fdm_index_connector_candidate_count);
    output.push_str(",\"nonConnectorCommandCount\":");
    push_option_usize_json(output, row.fdm_index_non_connector_command_count);
    output.push_str(",\"rowCohortConnectorCandidateCount\":");
    output.push_str(&row.connector_candidate_count.to_string());
    output.push_str(",\"connectorOnlySegmentCandidate\":");
    output.push_str(
        if vector_command_count > 0 && non_connector_command_count == 0 {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorDominantSegmentCandidate\":");
    output.push_str(if connector_candidate_count > non_connector_command_count {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorCandidateDensityPermille\":");
    push_option_usize_json(
        output,
        permille(connector_candidate_count, vector_command_count),
    );
    output.push_str(",\"matchedConnectorCoveragePermille\":");
    push_option_usize_json(
        output,
        permille(row.matched_connector_count, row.connector_candidate_count),
    );
    output.push_str(",\"dualEndpointMatchedConnectorCoveragePermille\":");
    push_option_usize_json(
        output,
        permille(
            row.dual_endpoint_match_connector_count,
            row.connector_candidate_count,
        ),
    );
    output.push('}');
}

pub(crate) fn push_fdm_open_stroke_axis_rule_index_bbox_relation_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexBboxRelation\"");
    output.push_str(",\"source\":\"FDMIndex.bbox+FDMVector.commandSourceBbox\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        row.fdm_index_bbox_relation_gate_blocked_reason(),
    ));
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"indexBbox\":");
    if let (Some(left), Some(top), Some(right), Some(bottom)) = (
        row.fdm_index_bbox_left,
        row.fdm_index_bbox_top,
        row.fdm_index_bbox_right,
        row.fdm_index_bbox_bottom,
    ) {
        push_fdm_normalized_bbox_json(output, (left, top, right, bottom));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"containsConnectorCount\":");
    output.push_str(&row.fdm_index_bbox_contains_connector_count.to_string());
    output.push_str(",\"overlapsConnectorCount\":");
    output.push_str(&row.fdm_index_bbox_overlaps_connector_count.to_string());
    output.push_str(",\"disjointConnectorCount\":");
    output.push_str(&row.fdm_index_bbox_disjoint_connector_count.to_string());
    output.push_str(",\"sourceBboxMissingConnectorCount\":");
    output.push_str(
        &row.fdm_index_bbox_source_bbox_missing_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointContainsConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_fdm_index_bbox_contains_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointOverlapsConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_fdm_index_bbox_overlaps_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointDisjointConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_fdm_index_bbox_disjoint_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointSourceBboxMissingConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_fdm_index_bbox_source_bbox_missing_connector_count
            .to_string(),
    );
    output.push('}');
}

pub(crate) fn push_fdm_open_stroke_axis_rule_owner_promotion_gate_summary_json(
    output: &mut String,
    summary: FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary,
) {
    let parent_normalized_order_gate_blocked_reason =
        summary.parent_normalized_order_gate_blocked_reason();
    let render_promotion_blocked_reason = if parent_normalized_order_gate_blocked_reason == "none" {
        "connector-ownership-and-paint-order-unproven"
    } else {
        parent_normalized_order_gate_blocked_reason
    };

    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+endpointOwnerMatchSummary+parentNormalizedOrderGate\"");
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive+fdmConnectorEndpointOwnerMatch\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"axisRuleDualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"dualEndpointOwnerCandidateCount\":");
    output.push_str(&summary.dual_endpoint_owner_candidate_count.to_string());
    output.push_str(",\"nearestFdmOwnerRowsMatchCount\":");
    output.push_str(&summary.nearest_fdm_owner_rows_match_count.to_string());
    output.push_str(",\"nearestFdmOwnerRowMatchesConnectorRowCount\":");
    output.push_str(
        &summary
            .nearest_fdm_owner_row_matches_connector_row_count
            .to_string(),
    );
    output.push_str(",\"mixedTopLevelVsNestedOrderNamespaceCount\":");
    output.push_str(
        &summary
            .mixed_top_level_vs_nested_order_namespace_count
            .to_string(),
    );
    output.push_str(",\"parentNormalizedOrderGateBlockedReason\":");
    output.push_str(&json_string(parent_normalized_order_gate_blocked_reason));
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnectorCount\":");
    output.push_str(
        &summary
            .parent_normalized_ordered_same_row_same_connector_count
            .to_string(),
    );
    output.push_str(",\"ownerParentCommandRelationCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_candidate_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-not-connector-row",
        summary.nearest_owner_row_not_connector_row_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-before-owner-parent-command-span",
        summary.before_owner_parent_command_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-between-owner-parent-command-span",
        summary.between_owner_parent_command_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-after-owner-parent-command-span",
        summary.after_owner_parent_command_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-parent-command-relation-unclassified",
        summary.parent_command_relation_unclassified_count,
    );
    output.push(']');
    output.push_str(",\"ownerParentSourceOrderRelationCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_candidate_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-not-connector-row",
        summary.nearest_owner_row_not_connector_row_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-before-owner-parent-relative-offset-span",
        summary.before_owner_parent_relative_offset_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-between-owner-parent-relative-offset-span",
        summary.between_owner_parent_relative_offset_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-after-owner-parent-relative-offset-span",
        summary.after_owner_parent_relative_offset_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-parent-relative-offset-relation-unclassified",
        summary.parent_relative_offset_relation_unclassified_count,
    );
    output.push_str("]}");
}

pub(crate) fn push_fdm_open_stroke_marker_style_profile_json(
    output: &mut String,
    profile: FdmOpenStrokeMarkerStyleProfile,
) {
    output.push_str("{\"basis\":\"fdm-vector-marker+style-word\"");
    output.push_str(",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true");
    output.push_str(",\"commandCount\":");
    output.push_str(&profile.command_count.to_string());
    output.push_str(",\"markerFamilyCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "line-marker",
        profile.line_marker_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "path-marker",
        profile.path_marker_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "bezier-marker",
        profile.bezier_marker_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "ellipse-marker",
        profile.ellipse_marker_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "other-marker",
        profile.other_marker_count,
    );
    output.push_str("],\"styleWordCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(output, "0x0000", profile.style_0000_count);
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(output, "0x0005", profile.style_0005_count);
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(output, "0x0080", profile.style_0080_count);
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(output, "0x00a0", profile.style_00a0_count);
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "other-style",
        profile.other_style_count,
    );
    output.push_str("],\"roleGate\":");
    push_fdm_open_stroke_marker_style_role_gate_json(output, profile);
    output.push('}');
}

pub(crate) fn push_fdm_open_stroke_marker_style_role_gate_json(
    output: &mut String,
    profile: FdmOpenStrokeMarkerStyleProfile,
) {
    output.push_str("{\"basis\":\"fdm-vector-marker-style-profile\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"markerFamilyDiversityCount\":");
    output.push_str(&profile.marker_family_diversity_count().to_string());
    output.push_str(",\"styleWordDiversityCount\":");
    output.push_str(&profile.style_word_diversity_count().to_string());
    let (dominant_marker_family, dominant_marker_family_count) = profile.dominant_marker_family();
    let (dominant_style_word, dominant_style_word_count) = profile.dominant_style_word();
    output.push_str(",\"dominantMarkerFamily\":");
    output.push_str(&json_string(dominant_marker_family));
    output.push_str(",\"dominantMarkerFamilyCount\":");
    output.push_str(&dominant_marker_family_count.to_string());
    output.push_str(",\"dominantStyleWord\":");
    output.push_str(&json_string(dominant_style_word));
    output.push_str(",\"dominantStyleWordCount\":");
    output.push_str(&dominant_style_word_count.to_string());
    output.push_str(",\"markerFamilyHomogeneous\":");
    output.push_str(if profile.marker_family_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"styleWordHomogeneous\":");
    output.push_str(if profile.style_word_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"homogeneousMarkerStyleCandidate\":");
    output.push_str(if profile.homogeneous_marker_style_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        profile.marker_style_role_promotion_blocked_reason(),
    ));
    output.push('}');
}

pub(crate) fn push_fdm_connector_owner_row_cohort_summary_json(
    output: &mut String,
    cohort: &str,
    summary: FdmConnectorOwnerRowCohortDiagnosticSummary,
) {
    output.push_str("{\"cohort\":");
    output.push_str(&json_string(cohort));
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&summary.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&summary.matched_connector_count.to_string());
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&summary.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&summary.nearby_endpoint_match_count.to_string());
    output.push('}');
}

pub(crate) fn push_fdm_connector_owner_command_relation_summary_json(
    output: &mut String,
    relation: &str,
    summary: FdmConnectorOwnerRowCohortDiagnosticSummary,
) {
    output.push_str("{\"relation\":");
    output.push_str(&json_string(relation));
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&summary.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&summary.matched_connector_count.to_string());
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&summary.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&summary.nearby_endpoint_match_count.to_string());
    output.push('}');
}

pub(crate) fn push_fdm_connector_graph_blocked_reason_count_json(
    output: &mut String,
    reason: &str,
    count: usize,
) {
    output.push_str("{\"reason\":");
    output.push_str(&json_string(reason));
    output.push_str(",\"count\":");
    output.push_str(&count.to_string());
    output.push('}');
}

pub(crate) fn push_fdm_connector_dominant_matched_connector_row_json(
    output: &mut String,
    summary: FdmConnectorGraphDiagnosticSummary,
) {
    let Some(row_index) = summary.dominant_matched_connector_row_index else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"basis\":\"fdmConnectorCandidateRowIndex+lineRuleEndpointMatchSummary\"");
    output.push_str(",\"rowIndex\":");
    output.push_str(&row_index.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_connector_candidate_count
            .to_string(),
    );
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_total_thresholded_endpoint_match_count
            .to_string(),
    );
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_matched_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"startOnlyLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_start_only_match_connector_count
            .to_string(),
    );
    output.push_str(",\"endOnlyLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_end_only_match_connector_count
            .to_string(),
    );
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_tight_endpoint_match_count
            .to_string(),
    );
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_nearby_endpoint_match_count
            .to_string(),
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"dominant-row-still-lacks-dual-endpoint-line-rule-match\"}");
}

pub(crate) fn push_page_layer_fdm_open_stroke_cohort_summary_json(
    output: &mut String,
    layout: PageLayout,
    summary: &FdmOpenStrokeCohortSummary,
) {
    output.push_str("{\"type\":\"fdmOpenStrokeCohortSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"projectionKind\":\"fdmOpenStrokeCohortSummary\"");
    output.push_str(",\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"placementProven\":false,\"decoded\":false");
    push_fdm_open_stroke_cohort_summary_fields_json(output, summary);
    output.push('}');
}

pub(crate) fn fdm_open_stroke_cohort_summary_json(
    layout: PageLayout,
    document: &Document,
) -> String {
    let command_diagnostics = fdm_command_diagnostics(document);
    let Some(extent) = fdm_command_projection_extent(&command_diagnostics) else {
        return "null".to_string();
    };
    let primitive_diagnostics = fdm_vector_primitive_diagnostics(document);
    let Some(summary) = fdm_open_stroke_cohort_summary(layout, &primitive_diagnostics, extent)
    else {
        return "null".to_string();
    };
    let mut output = String::from("{");
    output.push_str("\"projectionKind\":\"fdmOpenStrokeCohortSummary\"");
    output.push_str(",\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"placementProven\":false,\"decoded\":false");
    push_fdm_open_stroke_cohort_summary_fields_json(&mut output, &summary);
    output.push('}');
    output
}

pub(crate) fn push_fdm_open_stroke_cohort_summary_fields_json(
    output: &mut String,
    summary: &FdmOpenStrokeCohortSummary,
) {
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\"");
    output.push_str(",\"basis\":\"open-stroke-row-source-cohorts\"");
    output.push_str(",\"sourceBacked\":true");
    output.push_str(",\"geometryDecoded\":true");
    output.push_str(",\"ownershipProven\":false");
    output.push_str(",\"paintOrderDecoded\":false");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"open-stroke-role-and-paint-order-unproven\"",
    );
    output.push_str(",\"primitiveCount\":");
    output.push_str(&summary.primitive_count.to_string());
    output.push_str(",\"openStrokeCount\":");
    output.push_str(&summary.open_stroke_count.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"horizontalCount\":");
    output.push_str(&summary.horizontal_count.to_string());
    output.push_str(",\"verticalCount\":");
    output.push_str(&summary.vertical_count.to_string());
    output.push_str(",\"diagonalCount\":");
    output.push_str(&summary.diagonal_count.to_string());
    output.push_str(",\"lineMarkerCount\":");
    output.push_str(&summary.line_marker_count.to_string());
    output.push_str(",\"nonLineMarkerCount\":");
    output.push_str(&summary.non_line_marker_count.to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&summary.row_count.to_string());
    output.push_str(",\"rowCohortLimit\":");
    output.push_str(&FDM_OPEN_STROKE_ROW_COHORT_LIMIT.to_string());
    output.push_str(",\"rowCohortCount\":");
    output.push_str(&summary.row_cohorts.len().to_string());
    output.push_str(",\"dominantConnectorRow\":");
    push_fdm_open_stroke_dominant_connector_row_json(output, summary);
    output.push_str(",\"rowCohorts\":[");
    for (index, row) in summary.row_cohorts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_open_stroke_row_cohort_json(output, row);
    }
    output.push(']');
}

pub(crate) fn push_fdm_open_stroke_dominant_connector_row_json(
    output: &mut String,
    summary: &FdmOpenStrokeCohortSummary,
) {
    let Some(row_index) = summary.dominant_connector_row_index else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"basis\":\"fdmOpenStrokeRowConnectorCandidateCount\"");
    output.push_str(",\"rowIndex\":");
    output.push_str(&row_index.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(
        &summary
            .dominant_connector_row_connector_candidate_count
            .to_string(),
    );
    output.push_str(",\"openStrokeCount\":");
    output.push_str(&summary.dominant_connector_row_open_stroke_count.to_string());
    output.push_str(",\"horizontalCount\":");
    output.push_str(&summary.dominant_connector_row_horizontal_count.to_string());
    output.push_str(",\"verticalCount\":");
    output.push_str(&summary.dominant_connector_row_vertical_count.to_string());
    output
        .push_str(",\"renderPromotionBlockedReason\":\"dominant-open-stroke-row-role-unproven\"}");
}

pub(crate) fn push_fdm_open_stroke_row_cohort_json(
    output: &mut String,
    row: &FdmOpenStrokeRowCohortSummary,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"openStrokeCount\":");
    output.push_str(&row.open_stroke_count.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&row.connector_candidate_count.to_string());
    output.push_str(",\"horizontalCount\":");
    output.push_str(&row.horizontal_count.to_string());
    output.push_str(",\"verticalCount\":");
    output.push_str(&row.vertical_count.to_string());
    output.push_str(",\"diagonalCount\":");
    output.push_str(&row.diagonal_count.to_string());
    output.push_str(",\"lineMarkerCount\":");
    output.push_str(&row.line_marker_count.to_string());
    output.push_str(",\"nonLineMarkerCount\":");
    output.push_str(&row.non_line_marker_count.to_string());
    output.push_str(",\"markerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(output, row.marker_style_profile);
    output.push_str(",\"commandIndexMin\":");
    push_option_usize_json(output, row.command_index_min);
    output.push_str(",\"commandIndexMax\":");
    push_option_usize_json(output, row.command_index_max);
    output.push_str(",\"relativeOffsetMin\":");
    push_option_usize_json(output, row.relative_offset_min);
    output.push_str(",\"relativeOffsetMax\":");
    push_option_usize_json(output, row.relative_offset_max);
    output.push_str(",\"sourceBboxUnion\":");
    if let Some(bbox) = row.source_bbox_union {
        push_fdm_normalized_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"projectedBboxUnion\":");
    if let Some(bbox) = row.projected_bbox_union {
        push_bbox_tuple_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn push_fdm_connector_endpoint_owner_candidates_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    output.push_str("{\"basis\":\"fdmPrimitiveProjection+documentTextGroupLineProjection\",\"ownershipProven\":false,\"sourceBacked\":true");
    let probe_radius_px = fdm_connector_endpoint_owner_probe_radius_px(text_projection);
    output.push_str(",\"probeRadiusPx\":");
    output.push_str(&format!("{probe_radius_px:.3}"));
    output.push_str(",\"candidateLimit\":");
    output.push_str(&FDM_CONNECTOR_ENDPOINT_OWNER_CANDIDATE_LIMIT.to_string());
    output.push_str(",\"start\":");
    push_fdm_connector_endpoint_owner_candidate_array_json(
        output,
        metric.projected_start,
        layout,
        diagnostic,
        extent,
        primitive_diagnostics,
        text_projection,
        probe_radius_px,
    );
    output.push_str(",\"end\":");
    push_fdm_connector_endpoint_owner_candidate_array_json(
        output,
        metric.projected_end,
        layout,
        diagnostic,
        extent,
        primitive_diagnostics,
        text_projection,
        probe_radius_px,
    );
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_fdm_connector_endpoint_owner_candidate_array_json(
    output: &mut String,
    point: (f32, f32),
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
    probe_radius_px: f32,
) {
    let candidates = fdm_connector_endpoint_owner_candidates(
        point,
        layout,
        connector,
        extent,
        primitive_diagnostics,
        text_projection,
    );
    output.push('[');
    for (index, candidate) in candidates.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_connector_endpoint_owner_candidate_json(output, candidate, probe_radius_px);
    }
    output.push(']');
}

pub(crate) fn push_fdm_connector_endpoint_owner_candidate_json(
    output: &mut String,
    candidate: FdmConnectorEndpointOwnerCandidate<'_>,
    probe_radius_px: f32,
) {
    match candidate {
        FdmConnectorEndpointOwnerCandidate::Primitive {
            diagnostic,
            bbox,
            distance_px,
        } => {
            output.push_str("{\"kind\":\"fdmPrimitive\",\"source\":\"fdmVectorCommandPrimitive\"");
            output.push_str(",\"ownerProven\":false,\"sourceBacked\":true");
            output.push_str(",\"distancePx\":");
            output.push_str(&format!("{distance_px:.3}"));
            output.push_str(",\"withinProbeRadius\":");
            output.push_str(if distance_px <= probe_radius_px {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"bbox\":");
            push_bbox_tuple_json(output, bbox);
            output.push_str(",\"sourcePath\":");
            output.push_str(&json_string(diagnostic.candidate.path()));
            output.push_str(",\"objectCandidateIndex\":");
            output.push_str(&diagnostic.candidate_index.to_string());
            output.push_str(",\"rowIndex\":");
            output.push_str(&diagnostic.entry.row_index().to_string());
            output.push_str(",\"commandIndex\":");
            output.push_str(&diagnostic.command.command_index().to_string());
            output.push_str(",\"relativeOffset\":");
            output.push_str(&diagnostic.command.relative_offset().to_string());
            output.push_str(",\"markerHex\":");
            output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
            output.push_str(",\"primitiveKind\":");
            output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
            output.push_str(",\"sourcePathBbox\":");
            if let Some(source_bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
                push_object_fdm_index_bbox_json(output, source_bbox);
            } else {
                output.push_str("null");
            }
            output.push('}');
        }
        FdmConnectorEndpointOwnerCandidate::TextSlot {
            slot,
            bbox,
            distance_px,
        } => {
            output.push_str("{\"kind\":\"documentTextSlot\",\"source\":\"/DocumentText\"");
            output.push_str(",\"ownerProven\":false,\"sourceBacked\":true");
            output.push_str(",\"distancePx\":");
            output.push_str(&format!("{distance_px:.3}"));
            output.push_str(",\"withinProbeRadius\":");
            output.push_str(if distance_px <= probe_radius_px {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"bbox\":");
            push_bbox_tuple_json(output, bbox);
            output.push_str(",\"text\":");
            output.push_str(&json_string(&slot.text));
            output.push_str(",\"groupIndex\":");
            match slot.group_index {
                Some(group_index) => output.push_str(&group_index.to_string()),
                None => output.push_str("null"),
            }
            output.push_str(",\"lineOffsetUnits\":");
            output.push_str(&slot.line_offset_units.to_string());
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
            output.push('}');
        }
    }
}

pub(crate) fn push_fdm_connector_endpoint_owner_match_summary_json(
    output: &mut String,
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    let summary = fdm_connector_endpoint_owner_match_summary(
        layout,
        connector,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );
    output.push_str("{\"startCandidateCount\":");
    output.push_str(&summary.start_candidate_count.to_string());
    output.push_str(",\"endCandidateCount\":");
    output.push_str(&summary.end_candidate_count.to_string());
    output.push_str(",\"totalCandidateCount\":");
    output.push_str(&summary.total_candidate_count.to_string());
    output.push_str(",\"startWithinProbeCount\":");
    output.push_str(&summary.start_within_probe_count.to_string());
    output.push_str(",\"endWithinProbeCount\":");
    output.push_str(&summary.end_within_probe_count.to_string());
    output.push_str(",\"withinProbeCandidateCount\":");
    output.push_str(&summary.within_probe_candidate_count.to_string());
    output.push_str(",\"fdmPrimitiveCandidateCount\":");
    output.push_str(&summary.fdm_primitive_candidate_count.to_string());
    output.push_str(",\"documentTextSlotCandidateCount\":");
    output.push_str(&summary.document_text_slot_candidate_count.to_string());
    output.push_str(",\"dualEndpointOwnerCandidate\":");
    output.push_str(if summary.dual_endpoint_owner_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorCommandIndex\":");
    output.push_str(&summary.connector_command_index.to_string());
    output.push_str(",\"connectorParentCommandIndex\":");
    output.push_str(&summary.connector_parent_command_index.to_string());
    output.push_str(",\"connectorSyntheticNestedCommand\":");
    output.push_str(if summary.connector_synthetic_nested_command {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorRelativeOffset\":");
    output.push_str(&summary.connector_relative_offset.to_string());
    output.push_str(",\"connectorParentRelativeOffset\":");
    push_option_usize_json(output, summary.connector_parent_relative_offset);
    output.push_str(",\"startNearestFdmOwner\":");
    push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
        output,
        summary.start_nearest_fdm_owner_row_index,
        summary.start_nearest_fdm_owner_command_index,
        summary.start_nearest_fdm_owner_parent_command_index,
        summary.start_nearest_fdm_owner_synthetic_nested_command,
        summary.start_nearest_fdm_owner_relative_offset,
    );
    output.push_str(",\"endNearestFdmOwner\":");
    push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
        output,
        summary.end_nearest_fdm_owner_row_index,
        summary.end_nearest_fdm_owner_command_index,
        summary.end_nearest_fdm_owner_parent_command_index,
        summary.end_nearest_fdm_owner_synthetic_nested_command,
        summary.end_nearest_fdm_owner_relative_offset,
    );
    output.push_str(",\"nearestFdmOwnerRowsMatch\":");
    output.push_str(if summary.nearest_fdm_owner_rows_match {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nearestFdmOwnerRowMatchesConnectorRow\":");
    output.push_str(if summary.nearest_fdm_owner_row_matches_connector_row {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedTopLevelVsNestedOrderNamespace\":");
    output.push_str(if summary.mixed_top_level_vs_nested_order_namespace {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorCommandBetweenNearestFdmOwnerCommands\":");
    output.push_str(
        if summary.connector_command_between_nearest_fdm_owner_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorCommandBeforeNearestFdmOwnerCommands\":");
    output.push_str(
        if summary.connector_command_before_nearest_fdm_owner_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorCommandAfterNearestFdmOwnerCommands\":");
    output.push_str(
        if summary.connector_command_after_nearest_fdm_owner_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorRelativeOffsetBetweenNearestFdmOwnerOffsets\":");
    output.push_str(
        if summary.connector_relative_offset_between_nearest_fdm_owner_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorRelativeOffsetBeforeNearestFdmOwnerOffsets\":");
    output.push_str(
        if summary.connector_relative_offset_before_nearest_fdm_owner_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorRelativeOffsetAfterNearestFdmOwnerOffsets\":");
    output.push_str(
        if summary.connector_relative_offset_after_nearest_fdm_owner_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"startNearestFdmOwnerParentRelativeOffset\":");
    push_option_usize_json(
        output,
        summary.start_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"endNearestFdmOwnerParentRelativeOffset\":");
    push_option_usize_json(output, summary.end_nearest_fdm_owner_parent_relative_offset);
    output.push_str(",\"connectorParentCommandBetweenNearestFdmOwnerParentCommands\":");
    output.push_str(
        if summary.connector_parent_command_between_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandBeforeNearestFdmOwnerParentCommands\":");
    output.push_str(
        if summary.connector_parent_command_before_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandAfterNearestFdmOwnerParentCommands\":");
    output.push_str(
        if summary.connector_parent_command_after_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetBetweenNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if summary.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetBeforeNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if summary.connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetAfterNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if summary.connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"ownerCommandRelation\":");
    output.push_str(&json_string(summary.owner_command_relation()));
    output.push_str(",\"ownerSourceOrderRelation\":");
    output.push_str(&json_string(summary.owner_source_order_relation()));
    output.push_str(",\"ownerParentCommandRelation\":");
    output.push_str(&json_string(summary.owner_parent_command_relation()));
    output.push_str(",\"ownerParentSourceOrderRelation\":");
    output.push_str(&json_string(summary.owner_parent_source_order_relation()));
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnector\":");
    output.push_str(
        if summary.parent_normalized_ordered_same_row_same_connector() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"ownerProven\":false");
    output.push_str(",\"ownerGroupingProven\":false");
    output.push_str(",\"ownerGroupingPromotionBlockedReason\":");
    output.push_str(&json_string(
        summary.owner_grouping_promotion_blocked_reason(),
    ));
    output.push_str(",\"ownershipPromotionBlockedReason\":");
    output.push_str(&json_string(summary.ownership_promotion_blocked_reason()));
    output.push('}');
}

pub(crate) fn push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
    output: &mut String,
    row_index: Option<usize>,
    command_index: Option<usize>,
    parent_command_index: Option<usize>,
    synthetic_nested_command: bool,
    relative_offset: Option<usize>,
) {
    if let (Some(row_index), Some(command_index), Some(parent_command_index)) =
        (row_index, command_index, parent_command_index)
    {
        output.push_str("{\"rowIndex\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"commandIndex\":");
        output.push_str(&command_index.to_string());
        output.push_str(",\"parentCommandIndex\":");
        output.push_str(&parent_command_index.to_string());
        output.push_str(",\"syntheticNestedCommand\":");
        output.push_str(if synthetic_nested_command {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"relativeOffset\":");
        match relative_offset {
            Some(relative_offset) => output.push_str(&relative_offset.to_string()),
            None => output.push_str("null"),
        }
        output.push('}');
    } else {
        output.push_str("null");
    }
}
