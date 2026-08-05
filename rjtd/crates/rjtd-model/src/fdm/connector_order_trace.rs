use super::*;
use crate::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_page_layer_fdm_connector_candidate_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    fdm_open_stroke_axis_rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let (x, y, width, height) = metric.projected_bbox;
    output.push_str("{\"type\":\"fdmConnectorCandidateDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"fdmVectorCommandConnectorCandidate\",\"projectionKind\":\"fdmOpenPathConnectorCandidateProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":\"connector-ownership-grouping-and-paint-order-unproven\"");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(metric.basis));
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
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"parentCompoundCommand\":");
    push_fdm_connector_parent_compound_provenance_json(output, diagnostic);
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
    output.push_str(",\"styleWord\":");
    output.push_str(&diagnostic.command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!(
        "{:.3}",
        fdm_vector_stroke_width(diagnostic.command)
    ));
    output.push_str(",\"sourceEndpoints\":");
    push_fdm_connector_source_endpoints_json(output, metric);
    output.push_str(",\"projectedEndpoints\":");
    push_fdm_connector_projected_endpoints_json(output, metric);
    output.push_str(",\"projectedTextGrid\":");
    push_fdm_connector_projected_text_grid_json(output, layout, metric, line_rule_projection);
    output.push_str(",\"lineRuleAttachmentCandidates\":");
    push_fdm_connector_line_rule_attachment_candidates_json(
        output,
        layout,
        metric,
        line_rule_projection,
    );
    output.push_str(",\"lineRuleEndpointMatches\":");
    push_fdm_connector_line_rule_endpoint_matches_json(
        output,
        layout,
        metric,
        line_rule_projection,
    );
    output.push_str(",\"lineRuleEndpointMatchSummary\":");
    push_fdm_connector_line_rule_endpoint_match_summary_json(
        output,
        layout,
        metric,
        line_rule_projection,
    );
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleEndpointMatches\":");
    push_fdm_connector_open_stroke_axis_rule_endpoint_matches_json(
        output,
        layout,
        diagnostic,
        extent,
        metric,
        line_rule_projection,
        fdm_open_stroke_axis_rules,
    );
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleEndpointMatchSummary\":");
    push_fdm_connector_open_stroke_axis_rule_endpoint_match_summary_json(
        output,
        layout,
        diagnostic,
        metric,
        line_rule_projection,
        fdm_open_stroke_axis_rules,
    );
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleOwnerPromotionGate\":");
    push_fdm_connector_open_stroke_axis_rule_owner_promotion_gate_json(
        output,
        layout,
        diagnostic,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
        line_rule_projection,
        fdm_open_stroke_axis_rules,
    );
    output.push_str(",\"endpointOwnerCandidates\":");
    push_fdm_connector_endpoint_owner_candidates_json(
        output,
        layout,
        diagnostic,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );
    output.push_str(",\"endpointOwnerMatchSummary\":");
    push_fdm_connector_endpoint_owner_match_summary_json(
        output,
        layout,
        diagnostic,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );
    output.push_str(",\"sourceEndpointDistance\":");
    output.push_str(&format!("{:.3}", metric.source_endpoint_distance));
    output.push_str(",\"projectedEndpointDistance\":");
    output.push_str(&format!("{:.3}", metric.projected_endpoint_distance));
    output.push_str(",\"projectedSpan\":");
    output.push_str(&format!("{:.3}", metric.projected_span));
    output.push_str(",\"orientation\":");
    output.push_str(&json_string(metric.orientation));
    output.push_str(",\"pathPointCount\":");
    output.push_str(&diagnostic.command.path_points().len().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&diagnostic.command.curve_segments().len().to_string());
    output.push_str(",\"sourcePathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"projectionExtent\":{\"left\":");
    output.push_str(&extent.left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&extent.top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&extent.right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&extent.bottom.to_string());
    output.push('}');
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(crate) fn fdm_connector_order_trace_json(
    layout: PageLayout,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
    text_projection: Option<&ShanaiLanTextProjection>,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    fdm_open_stroke_axis_rules: &[FdmOpenStrokeAxisRule<'_>],
) -> Option<String> {
    let projection = line_rule_projection?;
    let mut selected = Vec::new();
    for diagnostic in primitive_diagnostics.iter().copied() {
        let Some(metric) = fdm_connector_candidate_metric(layout, diagnostic, extent) else {
            continue;
        };
        let Some(detail) = fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
            layout,
            diagnostic,
            metric,
            projection,
            fdm_open_stroke_axis_rules,
        ) else {
            continue;
        };
        if !detail.tight_dual_endpoint_match() || metric.orientation == "diagonal" {
            continue;
        }
        selected.push((diagnostic, metric, detail));
    }

    if selected.is_empty() {
        return None;
    }

    let mut output = format!(
        "{{\"type\":\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTrace\",\"bbox\":{{\"x\":0.0,\"y\":0.0,\"width\":{:.1},\"height\":{:.1}}}",
        layout.width_px(),
        layout.height_px()
    );
    output.push_str(",\"projectionKind\":\"fdmConnectorSourceOrderTrace\"");
    output.push_str(
        ",\"source\":\"FDMIndex+FDMVector+sameRowFdmOpenStrokeAxisRule+endpointOwnerMatch\"",
    );
    output.push_str(",\"basis\":\"sameRowFdmOpenStrokeAxisRule+tightDualEndpoint+nonDiagonalConnector+sourceOrderTrace\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true,\"sourceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":\"projected-endpoint-straight-line-paint-order-and-ownership-unproven\"");
    output.push_str(",\"selectionPredicate\":{\"requiresTightDualEndpointAxisRuleMatch\":true,\"excludesDiagonalConnectors\":true,\"rowHardcoded\":false}");
    output.push_str(",\"traceCount\":");
    output.push_str(&selected.len().to_string());
    output.push_str(",\"summary\":");
    push_fdm_connector_order_trace_summary_json(
        &mut output,
        fdm_connector_order_trace_summary(
            layout,
            &selected,
            primitive_diagnostics,
            extent,
            text_projection,
        ),
    );
    output.push_str(",\"traces\":[");
    for (index, (diagnostic, metric, detail)) in selected.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_connector_order_trace_connector_json(
            &mut output,
            layout,
            diagnostic,
            extent,
            metric,
            detail,
            primitive_diagnostics,
            text_projection,
            projection,
            fdm_open_stroke_axis_rules,
        );
    }
    output.push_str("]}");
    Some(output)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_fdm_connector_order_trace_connector_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
    projection: &ShanaiLanLineRuleProjection,
    fdm_open_stroke_axis_rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let owner_summary = fdm_connector_endpoint_owner_match_summary(
        layout,
        diagnostic,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );
    let viewport = fdm_projection_viewport(layout);
    let start_matches =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
            .map(|point| {
                fdm_connector_open_stroke_axis_rule_endpoint_matches(
                    diagnostic,
                    fdm_open_stroke_axis_rules,
                    point,
                )
            })
            .unwrap_or_default();
    let end_matches =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
            .map(|point| {
                fdm_connector_open_stroke_axis_rule_endpoint_matches(
                    diagnostic,
                    fdm_open_stroke_axis_rules,
                    point,
                )
            })
            .unwrap_or_default();

    output.push_str("{\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"fdmIndexRow\":");
    push_fdm_connector_order_trace_index_row_json(output, diagnostic);
    output.push_str(",\"imageBearingSegmentGate\":");
    push_fdm_connector_order_trace_image_bearing_gate_json(
        output,
        diagnostic,
        detail,
        owner_summary,
    );
    output.push_str(",\"connector\":");
    push_fdm_connector_order_trace_connector_command_json(output, diagnostic);
    output.push_str(",\"axisRuleMatchSummary\":{\"startMatchCount\":");
    output.push_str(&detail.summary.start_match_count.to_string());
    output.push_str(",\"endMatchCount\":");
    output.push_str(&detail.summary.end_match_count.to_string());
    output.push_str(",\"totalMatchCount\":");
    output.push_str(&detail.summary.total_match_count.to_string());
    output.push_str(",\"tightMatchCount\":");
    output.push_str(&detail.summary.tight_match_count.to_string());
    output.push_str(",\"tightDualEndpointMatch\":");
    output.push_str(if detail.tight_dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"axisRuleParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    );
    output.push('}');
    output.push_str(",\"endpointOwners\":");
    push_fdm_connector_order_trace_endpoint_owners_json(output, owner_summary);
    output.push_str(",\"axisRuleMatches\":{\"start\":");
    push_fdm_connector_order_trace_axis_match_array_json(output, &start_matches);
    output.push_str(",\"end\":");
    push_fdm_connector_order_trace_axis_match_array_json(output, &end_matches);
    output.push('}');
    output.push_str(",\"relations\":");
    push_fdm_connector_order_trace_relations_json(output, diagnostic, detail, owner_summary);
    output.push_str(",\"sourceOrderNodes\":");
    push_fdm_connector_order_trace_source_order_nodes_json(
        output,
        diagnostic,
        owner_summary,
        &start_matches,
        &end_matches,
    );
    output.push('}');
}

pub(crate) fn fdm_connector_order_trace_summary(
    layout: PageLayout,
    selected: &[(
        FdmCommandDiagnostic<'_>,
        FdmConnectorCandidateMetric,
        FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
    )],
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
    text_projection: Option<&ShanaiLanTextProjection>,
) -> FdmConnectorOrderTraceSummary {
    let mut summary = FdmConnectorOrderTraceSummary {
        trace_count: selected.len(),
        ..Default::default()
    };

    for (diagnostic, metric, detail) in selected.iter().copied() {
        let owner_summary = fdm_connector_endpoint_owner_match_summary(
            layout,
            diagnostic,
            extent,
            metric,
            primitive_diagnostics,
            text_projection,
        );
        if diagnostic
            .command
            .source_segment()
            .map(|segment| segment.relative_offset() == diagnostic.entry.vector_offset())
            .unwrap_or(false)
        {
            summary.source_segment_matches_index_entry_count += 1;
        }
        if fdm_connector_command_matches_entry_connector_candidate(diagnostic) {
            summary.entry_connector_candidate_count += 1;
        }
        let image_bearing_segment = fdm_connector_image_bearing_segment_candidate(diagnostic);
        if image_bearing_segment {
            summary.image_bearing_segment_count += 1;
            if fdm_connector_segment_complete_image_payload_span_count(diagnostic) > 0 {
                summary.image_bearing_complete_payload_segment_count += 1;
            } else {
                summary.image_bearing_signature_without_payload_segment_count += 1;
            }
        }
        if owner_summary.parent_normalized_ordered_same_row_same_connector() {
            summary.parent_normalized_ordered_same_row_same_connector_count += 1;
        }

        let bbox_relation = fdm_connector_fdm_index_bbox_relation(diagnostic);
        match bbox_relation {
            "contained-in-fdm-index-bbox" => {
                summary.bbox_contained_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_bbox_contained_count += 1;
                }
            }
            "overlaps-fdm-index-bbox" => {
                summary.bbox_overlaps_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_bbox_overlaps_count += 1;
                }
            }
            "disjoint-from-fdm-index-bbox" => {
                summary.bbox_disjoint_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_bbox_disjoint_count += 1;
                }
            }
            _ => {
                summary.bbox_missing_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_bbox_missing_count += 1;
                }
            }
        }

        let connector_axis_rule_relation = fdm_connector_axis_rule_parent_span_relation(
            owner_summary.connector_parent_relative_offset,
            detail.axis_rule_match_parent_relative_offset_min,
            detail.axis_rule_match_parent_relative_offset_max,
        );
        match connector_axis_rule_relation {
            "connector-before-axis-rule-parent-span" => {
                summary.connector_before_axis_rule_parent_span_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_connector_before_axis_rule_parent_span_count += 1;
                }
            }
            "connector-between-axis-rule-parent-span" => {
                summary.connector_between_axis_rule_parent_span_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_connector_between_axis_rule_parent_span_count += 1;
                }
            }
            "connector-after-axis-rule-parent-span" => {
                summary.connector_after_axis_rule_parent_span_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_connector_after_axis_rule_parent_span_count += 1;
                }
            }
            _ => {
                summary.connector_axis_rule_parent_span_missing_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_connector_axis_rule_parent_span_missing_count += 1;
                }
            }
        }
        if image_bearing_segment {
            match fdm_connector_relation_to_segment_image_signature_range(diagnostic) {
                "connector-before-segment-image-signature-range" => {
                    summary.image_bearing_connector_before_segment_signature_range_count += 1;
                }
                "connector-inside-segment-image-signature-range" => {
                    summary.image_bearing_connector_inside_segment_signature_range_count += 1;
                }
                "connector-after-segment-image-signature-range" => {
                    summary.image_bearing_connector_after_segment_signature_range_count += 1;
                }
                _ => {
                    summary.image_bearing_connector_segment_signature_range_missing_count += 1;
                }
            }
        }

        match fdm_owner_axis_rule_parent_span_relation(owner_summary, detail) {
            "owner-parent-span-before-axis-rule-parent-span" => {
                summary.owner_parent_span_before_axis_rule_parent_span_count += 1;
            }
            "owner-parent-span-after-axis-rule-parent-span" => {
                summary.owner_parent_span_after_axis_rule_parent_span_count += 1;
            }
            "owner-parent-span-inside-axis-rule-parent-span" => {
                summary.owner_parent_span_inside_axis_rule_parent_span_count += 1;
            }
            "axis-rule-parent-span-inside-owner-parent-span" => {
                summary.axis_rule_parent_span_inside_owner_parent_span_count += 1;
            }
            "owner-parent-span-overlaps-axis-rule-parent-span" => {
                summary.owner_parent_span_overlaps_axis_rule_parent_span_count += 1;
            }
            _ => summary.owner_axis_rule_parent_span_missing_count += 1,
        }
    }

    summary
}

pub(crate) fn push_fdm_connector_order_trace_summary_json(
    output: &mut String,
    summary: FdmConnectorOrderTraceSummary,
) {
    output
        .push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTrace+relationCounts\"");
    output.push_str(",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true");
    output.push_str(",\"promotionReady\":");
    output.push_str(if summary.promotion_ready() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"readinessBlockedReason\":");
    output.push_str(&json_string(summary.readiness_blocked_reason()));
    output.push_str(",\"traceCount\":");
    output.push_str(&summary.trace_count.to_string());
    output.push_str(",\"sourceSegmentMatchesIndexEntryCount\":");
    output.push_str(&summary.source_segment_matches_index_entry_count.to_string());
    output.push_str(",\"entryConnectorCandidateCount\":");
    output.push_str(&summary.entry_connector_candidate_count.to_string());
    output.push_str(",\"imageBearingSegmentCount\":");
    output.push_str(&summary.image_bearing_segment_count.to_string());
    output.push_str(",\"imageBearingCompletePayloadSegmentCount\":");
    output.push_str(
        &summary
            .image_bearing_complete_payload_segment_count
            .to_string(),
    );
    output.push_str(",\"imageBearingSignatureWithoutPayloadSegmentCount\":");
    output.push_str(
        &summary
            .image_bearing_signature_without_payload_segment_count
            .to_string(),
    );
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnectorCount\":");
    output.push_str(
        &summary
            .parent_normalized_ordered_same_row_same_connector_count
            .to_string(),
    );
    output.push_str(",\"bboxRelationCounts\":{");
    output.push_str("\"contained\":");
    output.push_str(&summary.bbox_contained_count.to_string());
    output.push_str(",\"overlaps\":");
    output.push_str(&summary.bbox_overlaps_count.to_string());
    output.push_str(",\"disjoint\":");
    output.push_str(&summary.bbox_disjoint_count.to_string());
    output.push_str(",\"missing\":");
    output.push_str(&summary.bbox_missing_count.to_string());
    output.push('}');
    output.push_str(",\"imageBearingBboxRelationCounts\":{");
    output.push_str("\"contained\":");
    output.push_str(&summary.image_bearing_bbox_contained_count.to_string());
    output.push_str(",\"overlaps\":");
    output.push_str(&summary.image_bearing_bbox_overlaps_count.to_string());
    output.push_str(",\"disjoint\":");
    output.push_str(&summary.image_bearing_bbox_disjoint_count.to_string());
    output.push_str(",\"missing\":");
    output.push_str(&summary.image_bearing_bbox_missing_count.to_string());
    output.push('}');
    output.push_str(",\"connectorVsAxisRuleParentSpanCounts\":{");
    output.push_str("\"before\":");
    output.push_str(
        &summary
            .connector_before_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"between\":");
    output.push_str(
        &summary
            .connector_between_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"after\":");
    output.push_str(
        &summary
            .connector_after_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"missing\":");
    output.push_str(
        &summary
            .connector_axis_rule_parent_span_missing_count
            .to_string(),
    );
    output.push('}');
    output.push_str(",\"imageBearingConnectorVsAxisRuleParentSpanCounts\":{");
    output.push_str("\"before\":");
    output.push_str(
        &summary
            .image_bearing_connector_before_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"between\":");
    output.push_str(
        &summary
            .image_bearing_connector_between_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"after\":");
    output.push_str(
        &summary
            .image_bearing_connector_after_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"missing\":");
    output.push_str(
        &summary
            .image_bearing_connector_axis_rule_parent_span_missing_count
            .to_string(),
    );
    output.push('}');
    output.push_str(",\"imageBearingConnectorVsSegmentImageSignatureRangeCounts\":{");
    output.push_str("\"before\":");
    output.push_str(
        &summary
            .image_bearing_connector_before_segment_signature_range_count
            .to_string(),
    );
    output.push_str(",\"inside\":");
    output.push_str(
        &summary
            .image_bearing_connector_inside_segment_signature_range_count
            .to_string(),
    );
    output.push_str(",\"after\":");
    output.push_str(
        &summary
            .image_bearing_connector_after_segment_signature_range_count
            .to_string(),
    );
    output.push_str(",\"missing\":");
    output.push_str(
        &summary
            .image_bearing_connector_segment_signature_range_missing_count
            .to_string(),
    );
    output.push('}');
    output.push_str(",\"ownerVsAxisRuleParentSpanCounts\":{");
    output.push_str("\"before\":");
    output.push_str(
        &summary
            .owner_parent_span_before_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"after\":");
    output.push_str(
        &summary
            .owner_parent_span_after_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"ownerInsideAxis\":");
    output.push_str(
        &summary
            .owner_parent_span_inside_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"axisInsideOwner\":");
    output.push_str(
        &summary
            .axis_rule_parent_span_inside_owner_parent_span_count
            .to_string(),
    );
    output.push_str(",\"overlaps\":");
    output.push_str(
        &summary
            .owner_parent_span_overlaps_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"missing\":");
    output.push_str(
        &summary
            .owner_axis_rule_parent_span_missing_count
            .to_string(),
    );
    output.push_str("}}");
}

pub(crate) fn push_fdm_connector_order_trace_index_row_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&diagnostic.entry.index_offset().to_string());
    output.push_str(",\"vectorOffset\":");
    output.push_str(&diagnostic.entry.vector_offset().to_string());
    output.push_str(",\"vectorLength\":");
    output.push_str(&diagnostic.entry.vector_len().to_string());
    output.push_str(",\"validVectorOffset\":");
    output.push_str(if diagnostic.entry.valid_vector_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", diagnostic.entry.kind())));
    output.push_str(",\"normalizedBbox\":");
    push_fdm_normalized_bbox_json(output, normalize_fdm_bbox(diagnostic.entry.bbox()));
    output.push_str(",\"axisPairBbox\":");
    push_fdm_normalized_bbox_json(
        output,
        normalize_fdm_index_entry_bbox(diagnostic.entry.bbox()),
    );
    output.push_str(",\"imageSignatureCount\":");
    output.push_str(&diagnostic.entry.image_signature_hits().len().to_string());
    output.push_str(",\"segmentImageSignatureCount\":");
    output.push_str(
        &diagnostic
            .entry
            .segment_image_signature_hits()
            .len()
            .to_string(),
    );
    output.push_str(",\"imageBearingSegmentCandidate\":");
    output.push_str(
        if !diagnostic.entry.image_signature_hits().is_empty()
            || !diagnostic.entry.segment_image_signature_hits().is_empty()
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"vectorCommandCount\":");
    output.push_str(&diagnostic.entry.vector_commands().len().to_string());
    output.push_str(",\"entryConnectorCandidateCount\":");
    output.push_str(&diagnostic.entry.connector_candidates().len().to_string());
    output.push('}');
}

pub(crate) fn push_fdm_connector_order_trace_image_bearing_gate_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
) {
    let image_bearing = fdm_connector_image_bearing_segment_candidate(diagnostic);
    output.push_str(
        "{\"basis\":\"FDMIndex.imageSignature+FDMVector.connectorBbox+sameRowAxisRuleParentSpan\"",
    );
    output.push_str(",\"source\":\"FDMIndex.segmentImageSignatures+FDMVector.commandSourceBbox\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        fdm_connector_image_bearing_gate_blocked_reason(diagnostic),
    ));
    output.push_str(",\"imageBearingSegmentCandidate\":");
    output.push_str(if image_bearing { "true" } else { "false" });
    output.push_str(",\"connectorParent\":{\"commandIndex\":");
    output.push_str(
        &fdm_command_parent_command_index(diagnostic.command.command_index()).to_string(),
    );
    output.push_str(",\"relativeOffset\":");
    push_option_usize_json(
        output,
        fdm_command_normalized_parent_relative_offset(diagnostic),
    );
    output.push('}');
    output.push_str(",\"axisRuleParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    );
    output.push_str(",\"endpointOwnerParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        owner_summary
            .start_nearest_fdm_owner_parent_relative_offset
            .min(owner_summary.end_nearest_fdm_owner_parent_relative_offset),
        owner_summary
            .start_nearest_fdm_owner_parent_relative_offset
            .max(owner_summary.end_nearest_fdm_owner_parent_relative_offset),
    );
    output.push_str(",\"endpointOwnerParentRelations\":{\"connectorVsOwnerParentSpan\":");
    output.push_str(&json_string(
        owner_summary.owner_parent_source_order_relation(),
    ));
    output.push_str(",\"ownerParentSpanVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_owner_axis_rule_parent_span_relation(
        owner_summary,
        detail,
    )));
    output.push('}');
    output.push_str(",\"endpointOwnerParentRelationToAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_owner_axis_rule_parent_span_relation(
        owner_summary,
        detail,
    )));
    output.push_str(",\"imageSignatures\":");
    push_object_image_signature_hits_json(output, diagnostic.entry.image_signature_hits());
    output.push_str(",\"segmentImageSignatures\":");
    push_object_image_signature_hits_json(output, diagnostic.entry.segment_image_signature_hits());
    output.push_str(",\"segmentImageSignatureCommandContexts\":");
    push_fdm_connector_segment_image_signature_command_contexts_json(output, diagnostic);
    output.push_str(",\"imageSignatureOffsetRange\":");
    push_image_signature_offset_range_json(output, diagnostic.entry.image_signature_hits());
    output.push_str(",\"segmentImageSignatureOffsetRange\":");
    push_image_signature_offset_range_json(output, diagnostic.entry.segment_image_signature_hits());
    output.push_str(",\"completeImagePayloadSpanCount\":");
    output.push_str(&fdm_connector_complete_image_payload_span_count(diagnostic).to_string());
    output.push_str(",\"segmentCompleteImagePayloadSpanCount\":");
    output
        .push_str(&fdm_connector_segment_complete_image_payload_span_count(diagnostic).to_string());
    output.push_str(",\"payloadExtractionStatus\":");
    output.push_str(&json_string(fdm_connector_image_payload_extraction_status(
        diagnostic,
    )));
    output.push_str(",\"connectorVsSegmentImageSignatureRange\":");
    output.push_str(&json_string(
        fdm_connector_relation_to_segment_image_signature_range(diagnostic),
    ));
    output.push_str(",\"connectorVsImageSignatureRange\":");
    output.push_str(&json_string(
        fdm_connector_relation_to_image_signature_range(diagnostic),
    ));
    let nearest_segment_signature = fdm_connector_nearest_segment_image_signature(diagnostic);
    output.push_str(",\"nearestSegmentImageSignatureOffset\":");
    push_option_usize_json(output, nearest_segment_signature.map(|(offset, _)| offset));
    output.push_str(",\"nearestSegmentImageSignatureDistance\":");
    push_option_usize_json(
        output,
        nearest_segment_signature.map(|(_, distance)| distance),
    );
    let nearest_signature = fdm_connector_nearest_image_signature(diagnostic);
    output.push_str(",\"nearestImageSignatureOffset\":");
    push_option_usize_json(output, nearest_signature.map(|(offset, _)| offset));
    output.push_str(",\"nearestImageSignatureDistance\":");
    push_option_usize_json(output, nearest_signature.map(|(_, distance)| distance));
    output.push_str(",\"bboxRelationToFdmIndex\":");
    output.push_str(&json_string(fdm_connector_fdm_index_bbox_relation(
        diagnostic,
    )));
    output.push_str(",\"connectorVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_connector_axis_rule_parent_span_relation(
        owner_summary.connector_parent_relative_offset,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    )));
    output.push_str(",\"ownerParentSpanVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_owner_axis_rule_parent_span_relation(
        owner_summary,
        detail,
    )));
    output.push('}');
}

pub(crate) fn fdm_connector_image_bearing_segment_candidate(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> bool {
    !diagnostic.entry.image_signature_hits().is_empty()
        || !diagnostic.entry.segment_image_signature_hits().is_empty()
}

pub(crate) fn push_fdm_connector_segment_image_signature_command_contexts_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
) {
    output.push('[');
    for (index, hit) in diagnostic
        .entry
        .segment_image_signature_hits()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        let containing_commands = diagnostic
            .entry
            .vector_commands()
            .iter()
            .filter(|command| {
                fdm_offset_inside_command_record(
                    hit.offset(),
                    command.relative_offset(),
                    command.record_len(),
                )
            })
            .collect::<Vec<_>>();
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push_str(",\"relationToTraceConnectorCommand\":");
        output.push_str(&json_string(fdm_offset_relation_to_command_record(
            hit.offset(),
            diagnostic.command.relative_offset(),
            diagnostic.command.record_len(),
        )));
        output.push_str(",\"containingCommandCount\":");
        output.push_str(&containing_commands.len().to_string());
        output.push_str(",\"containingCommands\":[");
        for (command_index, command) in containing_commands.iter().enumerate() {
            if command_index > 0 {
                output.push(',');
            }
            output.push_str("{\"commandIndex\":");
            output.push_str(&command.command_index().to_string());
            output.push_str(",\"relativeOffset\":");
            output.push_str(&command.relative_offset().to_string());
            output.push_str(",\"recordEnd\":");
            output.push_str(
                &command
                    .relative_offset()
                    .saturating_add(command.record_len())
                    .to_string(),
            );
            output.push_str(",\"recordLength\":");
            output.push_str(&command.record_len().to_string());
            output.push_str(",\"declaredRecordLength\":");
            output.push_str(&command.declared_record_len().to_string());
            output.push_str(",\"offsetInCommand\":");
            output.push_str(
                &hit.offset()
                    .saturating_sub(command.relative_offset())
                    .to_string(),
            );
            output.push_str(",\"markerHex\":");
            output.push_str(&json_string(&hex_bytes(command.marker())));
            output.push_str(",\"primitiveKind\":");
            output.push_str(&json_string(fdm_vector_primitive_kind(command)));
            output.push_str(",\"styleWordHex\":");
            output.push_str(&json_string(&format!("0x{:04x}", command.style_word())));
            output.push_str(",\"syntheticNestedCommand\":");
            output.push_str(
                if fdm_command_index_is_synthetic_nested(command.command_index()) {
                    "true"
                } else {
                    "false"
                },
            );
            output.push_str(",\"sameAsTraceConnector\":");
            output.push_str(
                if command.command_index() == diagnostic.command.command_index()
                    && command.relative_offset() == diagnostic.command.relative_offset()
                {
                    "true"
                } else {
                    "false"
                },
            );
            output.push('}');
        }
        output.push_str("]}");
    }
    output.push(']');
}

pub(crate) fn fdm_offset_inside_command_record(offset: usize, start: usize, len: usize) -> bool {
    start <= offset && offset < start.saturating_add(len)
}

pub(crate) fn fdm_offset_relation_to_command_record(
    offset: usize,
    start: usize,
    len: usize,
) -> &'static str {
    if offset < start {
        "before-command-record"
    } else if offset >= start.saturating_add(len) {
        "after-command-record"
    } else {
        "inside-command-record"
    }
}

pub(crate) fn fdm_connector_complete_image_payload_span_count(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> usize {
    diagnostic
        .candidate
        .image_payload_spans()
        .iter()
        .filter(|span| span.complete())
        .count()
}

pub(crate) fn fdm_connector_segment_complete_image_payload_span_count(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> usize {
    diagnostic
        .candidate
        .image_payload_spans()
        .iter()
        .filter(|span| {
            span.complete()
                && span.signature_offset() >= diagnostic.entry.vector_offset()
                && span.signature_offset() < diagnostic.entry.next_vector_offset()
        })
        .count()
}

pub(crate) fn fdm_connector_image_payload_extraction_status(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    if !fdm_connector_image_bearing_segment_candidate(diagnostic) {
        "no-image-signature"
    } else if fdm_connector_segment_complete_image_payload_span_count(diagnostic) > 0 {
        "complete-payload-in-fdm-index-segment"
    } else if fdm_connector_complete_image_payload_span_count(diagnostic) > 0 {
        "complete-payload-elsewhere-in-vector-stream"
    } else {
        "signature-without-complete-payload"
    }
}

pub(crate) fn fdm_connector_relation_to_segment_image_signature_range(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    let Some((min_offset, max_offset)) =
        image_signature_offset_range(diagnostic.entry.segment_image_signature_hits())
    else {
        return "no-segment-image-signature-range";
    };
    let offset = diagnostic.command.relative_offset();
    if offset < min_offset {
        "connector-before-segment-image-signature-range"
    } else if offset > max_offset {
        "connector-after-segment-image-signature-range"
    } else {
        "connector-inside-segment-image-signature-range"
    }
}

pub(crate) fn fdm_connector_relation_to_image_signature_range(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    let Some((min_offset, max_offset)) =
        image_signature_offset_range(diagnostic.entry.image_signature_hits())
    else {
        return "no-image-signature-range";
    };
    let Some(offset) = diagnostic.command.source_vector_relative_offset() else {
        return "connector-source-vector-offset-missing";
    };
    if offset < min_offset {
        "connector-before-image-signature-range"
    } else if offset > max_offset {
        "connector-after-image-signature-range"
    } else {
        "connector-inside-image-signature-range"
    }
}

pub(crate) fn fdm_connector_nearest_segment_image_signature(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> Option<(usize, usize)> {
    nearest_image_signature_offset(
        diagnostic.entry.segment_image_signature_hits(),
        diagnostic.command.relative_offset(),
    )
}

pub(crate) fn fdm_connector_nearest_image_signature(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> Option<(usize, usize)> {
    nearest_image_signature_offset(
        diagnostic.entry.image_signature_hits(),
        diagnostic.command.source_vector_relative_offset()?,
    )
}

pub(crate) fn fdm_connector_image_bearing_gate_blocked_reason(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    if !fdm_connector_image_bearing_segment_candidate(diagnostic) {
        return "not-image-bearing-segment";
    }
    if fdm_connector_segment_complete_image_payload_span_count(diagnostic) == 0 {
        return "image-signature-without-complete-payload-role-unproven";
    }
    match fdm_connector_fdm_index_bbox_relation(diagnostic) {
        "contained-in-fdm-index-bbox" => "image-bearing-contained-internal-stroke-role-unproven",
        "overlaps-fdm-index-bbox" => "image-bearing-overlapping-object-boundary-role-unproven",
        "disjoint-from-fdm-index-bbox" => "image-bearing-disjoint-external-connector-role-unproven",
        _ => "image-bearing-connector-source-bbox-missing",
    }
}

pub(crate) fn push_fdm_connector_order_trace_connector_command_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"parentCommandIndex\":");
    output.push_str(
        &fdm_command_parent_command_index(diagnostic.command.command_index()).to_string(),
    );
    output.push_str(",\"syntheticNestedCommand\":");
    output.push_str(
        if fdm_command_index_is_synthetic_nested(diagnostic.command.command_index()) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    output.push_str(",\"parentRelativeOffset\":");
    push_option_usize_json(
        output,
        fdm_command_normalized_parent_relative_offset(diagnostic),
    );
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"sourceSegmentMatchesIndexEntry\":");
    match diagnostic.command.source_segment() {
        Some(source_segment) => output.push_str(
            if source_segment.relative_offset() == diagnostic.entry.vector_offset() {
                "true"
            } else {
                "false"
            },
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"entryConnectorCandidate\":");
    output.push_str(
        if fdm_connector_command_matches_entry_connector_candidate(diagnostic) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    output.push_str(",\"parentCompoundCommand\":");
    push_fdm_connector_parent_compound_provenance_json(output, diagnostic);
    output.push_str(",\"sourcePathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
        push_object_fdm_index_bbox_json(output, bbox);
        output.push_str(",\"normalizedSourcePathBbox\":");
        push_fdm_normalized_bbox_json(output, normalize_fdm_bbox(bbox));
    } else {
        output.push_str("null,\"normalizedSourcePathBbox\":null");
    }
    output.push('}');
}

pub(crate) fn fdm_connector_command_matches_entry_connector_candidate(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> bool {
    diagnostic
        .entry
        .connector_candidates()
        .iter()
        .any(|candidate| {
            candidate.command_index() == diagnostic.command.command_index()
                && candidate.relative_offset() == diagnostic.command.relative_offset()
        })
}

pub(crate) fn push_fdm_connector_order_trace_endpoint_owners_json(
    output: &mut String,
    summary: FdmConnectorEndpointOwnerMatchSummary,
) {
    output.push_str("{\"start\":");
    push_fdm_connector_order_trace_owner_json(
        output,
        summary.start_nearest_fdm_owner_row_index,
        summary.start_nearest_fdm_owner_command_index,
        summary.start_nearest_fdm_owner_parent_command_index,
        summary.start_nearest_fdm_owner_synthetic_nested_command,
        summary.start_nearest_fdm_owner_relative_offset,
        summary.start_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"end\":");
    push_fdm_connector_order_trace_owner_json(
        output,
        summary.end_nearest_fdm_owner_row_index,
        summary.end_nearest_fdm_owner_command_index,
        summary.end_nearest_fdm_owner_parent_command_index,
        summary.end_nearest_fdm_owner_synthetic_nested_command,
        summary.end_nearest_fdm_owner_relative_offset,
        summary.end_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"dualEndpointOwnerCandidate\":");
    output.push_str(if summary.dual_endpoint_owner_candidate() {
        "true"
    } else {
        "false"
    });
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
    output.push('}');
}

pub(crate) fn push_fdm_connector_order_trace_owner_json(
    output: &mut String,
    row_index: Option<usize>,
    command_index: Option<usize>,
    parent_command_index: Option<usize>,
    synthetic_nested_command: bool,
    relative_offset: Option<usize>,
    parent_relative_offset: Option<usize>,
) {
    let (Some(row_index), Some(command_index), Some(parent_command_index)) =
        (row_index, command_index, parent_command_index)
    else {
        output.push_str("null");
        return;
    };
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
    push_option_usize_json(output, relative_offset);
    output.push_str(",\"parentRelativeOffset\":");
    push_option_usize_json(output, parent_relative_offset);
    output.push('}');
}

pub(crate) fn push_fdm_connector_order_trace_axis_match_array_json(
    output: &mut String,
    matches: &[(
        usize,
        &FdmOpenStrokeAxisRule<'_>,
        FdmConnectorLineRuleDistance,
        &'static str,
    )],
) {
    output.push('[');
    for (index, (axis_rule_index, rule, distance, tier)) in matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"axisRuleIndex\":");
        output.push_str(&axis_rule_index.to_string());
        output.push_str(",\"ruleCommandIndex\":");
        output.push_str(&rule.diagnostic.command.command_index().to_string());
        output.push_str(",\"ruleParentCommandIndex\":");
        output.push_str(
            &fdm_command_parent_command_index(rule.diagnostic.command.command_index()).to_string(),
        );
        output.push_str(",\"ruleRelativeOffset\":");
        output.push_str(&rule.diagnostic.command.relative_offset().to_string());
        output.push_str(",\"ruleParentRelativeOffset\":");
        push_option_usize_json(
            output,
            fdm_command_normalized_parent_relative_offset(rule.diagnostic),
        );
        output.push_str(",\"ruleMarkerHex\":");
        output.push_str(&json_string(&hex_bytes(rule.diagnostic.command.marker())));
        output.push_str(",\"ruleStyleWordHex\":");
        output.push_str(&json_string(&format!(
            "0x{:04x}",
            rule.diagnostic.command.style_word()
        )));
        output.push_str(",\"orientation\":");
        output.push_str(&json_string(rule.orientation));
        output.push_str(",\"tier\":");
        output.push_str(&json_string(tier));
        output.push_str(",\"distanceGrid\":");
        output.push_str(&format!("{:.3}", distance.distance_grid));
        output.push_str(",\"axisDelta\":");
        output.push_str(&format!("{:.3}", distance.axis_delta));
        output.push_str(",\"inlineDelta\":");
        output.push_str(&format!("{:.3}", distance.inline_delta));
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_fdm_connector_order_trace_relations_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
) {
    output.push_str("{\"connectorVsOwnerParentSpan\":");
    output.push_str(&json_string(
        owner_summary.owner_parent_source_order_relation(),
    ));
    output.push_str(",\"connectorVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_connector_axis_rule_parent_span_relation(
        owner_summary.connector_parent_relative_offset,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    )));
    output.push_str(",\"ownerParentSpanVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_owner_axis_rule_parent_span_relation(
        owner_summary,
        detail,
    )));
    output.push_str(",\"bboxRelationToFdmIndex\":");
    output.push_str(&json_string(fdm_connector_fdm_index_bbox_relation(
        diagnostic,
    )));
    output.push_str(",\"entryConnectorCandidate\":");
    output.push_str(
        if fdm_connector_command_matches_entry_connector_candidate(diagnostic) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnector\":");
    output.push_str(
        if owner_summary.parent_normalized_ordered_same_row_same_connector() {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(crate) fn fdm_connector_axis_rule_parent_span_relation(
    connector_parent_relative_offset: Option<usize>,
    axis_rule_min: Option<usize>,
    axis_rule_max: Option<usize>,
) -> &'static str {
    let (Some(connector), Some(axis_min), Some(axis_max)) = (
        connector_parent_relative_offset,
        axis_rule_min,
        axis_rule_max,
    ) else {
        return "connector-or-axis-rule-parent-offset-missing";
    };
    if connector < axis_min {
        "connector-before-axis-rule-parent-span"
    } else if connector > axis_max {
        "connector-after-axis-rule-parent-span"
    } else {
        "connector-between-axis-rule-parent-span"
    }
}

pub(crate) fn fdm_owner_axis_rule_parent_span_relation(
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
) -> &'static str {
    let (Some(start_owner), Some(end_owner), Some(axis_min), Some(axis_max)) = (
        owner_summary.start_nearest_fdm_owner_parent_relative_offset,
        owner_summary.end_nearest_fdm_owner_parent_relative_offset,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    ) else {
        return "owner-or-axis-rule-parent-span-missing";
    };
    let owner_min = start_owner.min(end_owner);
    let owner_max = start_owner.max(end_owner);
    if owner_max < axis_min {
        "owner-parent-span-before-axis-rule-parent-span"
    } else if owner_min > axis_max {
        "owner-parent-span-after-axis-rule-parent-span"
    } else if axis_min <= owner_min && owner_max <= axis_max {
        "owner-parent-span-inside-axis-rule-parent-span"
    } else if owner_min <= axis_min && axis_max <= owner_max {
        "axis-rule-parent-span-inside-owner-parent-span"
    } else {
        "owner-parent-span-overlaps-axis-rule-parent-span"
    }
}

pub(crate) fn fdm_connector_fdm_index_bbox_relation(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    let Some(connector_bbox) =
        fdm_vector_command_source_bbox(diagnostic.command).map(normalize_fdm_bbox)
    else {
        return "connector-source-bbox-missing";
    };
    let index_bbox = normalize_fdm_index_entry_bbox(diagnostic.entry.bbox());
    if fdm_bbox_contains(index_bbox, connector_bbox) {
        "contained-in-fdm-index-bbox"
    } else if fdm_bbox_intersects(index_bbox, connector_bbox) {
        "overlaps-fdm-index-bbox"
    } else {
        "disjoint-from-fdm-index-bbox"
    }
}

pub(crate) fn push_fdm_connector_order_trace_source_order_nodes_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
    start_matches: &[(
        usize,
        &FdmOpenStrokeAxisRule<'_>,
        FdmConnectorLineRuleDistance,
        &'static str,
    )],
    end_matches: &[(
        usize,
        &FdmOpenStrokeAxisRule<'_>,
        FdmConnectorLineRuleDistance,
        &'static str,
    )],
) {
    let mut nodes = Vec::new();
    nodes.push(fdm_connector_order_trace_node_from_diagnostic(
        "connector",
        None,
        diagnostic,
        10,
    ));
    if let Some(node) = fdm_connector_order_trace_node_from_owner_summary(
        "nearestFdmOwner",
        "start",
        owner_summary.start_nearest_fdm_owner_row_index,
        owner_summary.start_nearest_fdm_owner_command_index,
        owner_summary.start_nearest_fdm_owner_parent_command_index,
        owner_summary.start_nearest_fdm_owner_synthetic_nested_command,
        owner_summary.start_nearest_fdm_owner_relative_offset,
        owner_summary.start_nearest_fdm_owner_parent_relative_offset,
        0,
    ) {
        nodes.push(node);
    }
    if let Some(node) = fdm_connector_order_trace_node_from_owner_summary(
        "nearestFdmOwner",
        "end",
        owner_summary.end_nearest_fdm_owner_row_index,
        owner_summary.end_nearest_fdm_owner_command_index,
        owner_summary.end_nearest_fdm_owner_parent_command_index,
        owner_summary.end_nearest_fdm_owner_synthetic_nested_command,
        owner_summary.end_nearest_fdm_owner_relative_offset,
        owner_summary.end_nearest_fdm_owner_parent_relative_offset,
        1,
    ) {
        nodes.push(node);
    }
    for (_, rule, _, _) in start_matches {
        nodes.push(fdm_connector_order_trace_node_from_diagnostic(
            "axisRule",
            Some("start"),
            rule.diagnostic,
            20,
        ));
    }
    for (_, rule, _, _) in end_matches {
        nodes.push(fdm_connector_order_trace_node_from_diagnostic(
            "axisRule",
            Some("end"),
            rule.diagnostic,
            21,
        ));
    }
    nodes.sort_by(|left, right| {
        left.parent_relative_offset
            .unwrap_or(usize::MAX)
            .cmp(&right.parent_relative_offset.unwrap_or(usize::MAX))
            .then_with(|| {
                left.relative_offset
                    .unwrap_or(usize::MAX)
                    .cmp(&right.relative_offset.unwrap_or(usize::MAX))
            })
            .then_with(|| left.rank.cmp(&right.rank))
    });
    output.push('[');
    for (index, node) in nodes.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&node.json);
    }
    output.push(']');
}

pub(crate) fn fdm_connector_order_trace_node_from_diagnostic(
    role: &'static str,
    endpoint: Option<&'static str>,
    diagnostic: FdmCommandDiagnostic<'_>,
    rank: usize,
) -> FdmConnectorOrderTraceNodeJson {
    let parent_relative_offset = fdm_command_normalized_parent_relative_offset(diagnostic);
    let relative_offset = Some(diagnostic.command.relative_offset());
    let mut json = String::new();
    json.push_str("{\"role\":");
    json.push_str(&json_string(role));
    json.push_str(",\"endpoint\":");
    match endpoint {
        Some(endpoint) => json.push_str(&json_string(endpoint)),
        None => json.push_str("null"),
    }
    json.push_str(",\"rowIndex\":");
    json.push_str(&diagnostic.entry.row_index().to_string());
    json.push_str(",\"commandIndex\":");
    json.push_str(&diagnostic.command.command_index().to_string());
    json.push_str(",\"parentCommandIndex\":");
    json.push_str(
        &fdm_command_parent_command_index(diagnostic.command.command_index()).to_string(),
    );
    json.push_str(",\"syntheticNestedCommand\":");
    json.push_str(
        if fdm_command_index_is_synthetic_nested(diagnostic.command.command_index()) {
            "true"
        } else {
            "false"
        },
    );
    json.push_str(",\"relativeOffset\":");
    json.push_str(&diagnostic.command.relative_offset().to_string());
    json.push_str(",\"parentRelativeOffset\":");
    push_option_usize_json(&mut json, parent_relative_offset);
    json.push_str(",\"markerHex\":");
    json.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    json.push_str(",\"styleWordHex\":");
    json.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    json.push('}');
    FdmConnectorOrderTraceNodeJson {
        parent_relative_offset,
        relative_offset,
        rank,
        json,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn fdm_connector_order_trace_node_from_owner_summary(
    role: &'static str,
    endpoint: &'static str,
    row_index: Option<usize>,
    command_index: Option<usize>,
    parent_command_index: Option<usize>,
    synthetic_nested_command: bool,
    relative_offset: Option<usize>,
    parent_relative_offset: Option<usize>,
    rank: usize,
) -> Option<FdmConnectorOrderTraceNodeJson> {
    let (Some(row_index), Some(command_index), Some(parent_command_index)) =
        (row_index, command_index, parent_command_index)
    else {
        return None;
    };
    let mut json = String::new();
    json.push_str("{\"role\":");
    json.push_str(&json_string(role));
    json.push_str(",\"endpoint\":");
    json.push_str(&json_string(endpoint));
    json.push_str(",\"rowIndex\":");
    json.push_str(&row_index.to_string());
    json.push_str(",\"commandIndex\":");
    json.push_str(&command_index.to_string());
    json.push_str(",\"parentCommandIndex\":");
    json.push_str(&parent_command_index.to_string());
    json.push_str(",\"syntheticNestedCommand\":");
    json.push_str(if synthetic_nested_command {
        "true"
    } else {
        "false"
    });
    json.push_str(",\"relativeOffset\":");
    push_option_usize_json(&mut json, relative_offset);
    json.push_str(",\"parentRelativeOffset\":");
    push_option_usize_json(&mut json, parent_relative_offset);
    json.push('}');
    Some(FdmConnectorOrderTraceNodeJson {
        parent_relative_offset,
        relative_offset,
        rank,
        json,
    })
}
