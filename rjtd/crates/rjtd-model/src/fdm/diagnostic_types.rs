use super::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct FdmFrameDiagnostic<'a> {
    pub(crate) candidate_index: usize,
    pub(crate) candidate: &'a ObjectStreamCandidate,
    pub(crate) entry: &'a ObjectFdmIndexEntryCandidate,
    pub(crate) frame_record: &'a ObjectFrameRecordCandidate,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FdmCommandDiagnostic<'a> {
    pub(crate) candidate_index: usize,
    pub(crate) candidate: &'a ObjectStreamCandidate,
    pub(crate) entry: &'a ObjectFdmIndexEntryCandidate,
    pub(crate) command: &'a ObjectFdmVectorCommandCandidate,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FdmConnectorParentCompoundProvenance<'a> {
    pub(crate) parent: &'a ObjectFdmVectorCommandCandidate,
    pub(crate) child_offset_in_parent: usize,
    pub(crate) child_offset_table_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FdmCommandProjectionExtent {
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FdmConnectorCandidateMetric {
    pub(crate) source_start: ObjectFdmVectorPoint,
    pub(crate) source_end: ObjectFdmVectorPoint,
    pub(crate) projected_start: (f32, f32),
    pub(crate) projected_end: (f32, f32),
    pub(crate) projected_bbox: (f32, f32, f32, f32),
    pub(crate) source_endpoint_distance: f32,
    pub(crate) projected_endpoint_distance: f32,
    pub(crate) projected_span: f32,
    pub(crate) orientation: &'static str,
    pub(crate) basis: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FdmConnectorOrderTraceNodeJson {
    pub(crate) parent_relative_offset: Option<usize>,
    pub(crate) relative_offset: Option<usize>,
    pub(crate) rank: usize,
    pub(crate) json: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmConnectorOrderTraceSummary {
    pub(crate) trace_count: usize,
    pub(crate) source_segment_matches_index_entry_count: usize,
    pub(crate) entry_connector_candidate_count: usize,
    pub(crate) image_bearing_segment_count: usize,
    pub(crate) image_bearing_complete_payload_segment_count: usize,
    pub(crate) image_bearing_signature_without_payload_segment_count: usize,
    pub(crate) parent_normalized_ordered_same_row_same_connector_count: usize,
    pub(crate) bbox_contained_count: usize,
    pub(crate) bbox_overlaps_count: usize,
    pub(crate) bbox_disjoint_count: usize,
    pub(crate) bbox_missing_count: usize,
    pub(crate) image_bearing_bbox_contained_count: usize,
    pub(crate) image_bearing_bbox_overlaps_count: usize,
    pub(crate) image_bearing_bbox_disjoint_count: usize,
    pub(crate) image_bearing_bbox_missing_count: usize,
    pub(crate) connector_before_axis_rule_parent_span_count: usize,
    pub(crate) connector_between_axis_rule_parent_span_count: usize,
    pub(crate) connector_after_axis_rule_parent_span_count: usize,
    pub(crate) connector_axis_rule_parent_span_missing_count: usize,
    pub(crate) image_bearing_connector_before_axis_rule_parent_span_count: usize,
    pub(crate) image_bearing_connector_between_axis_rule_parent_span_count: usize,
    pub(crate) image_bearing_connector_after_axis_rule_parent_span_count: usize,
    pub(crate) image_bearing_connector_axis_rule_parent_span_missing_count: usize,
    pub(crate) image_bearing_connector_before_segment_signature_range_count: usize,
    pub(crate) image_bearing_connector_inside_segment_signature_range_count: usize,
    pub(crate) image_bearing_connector_after_segment_signature_range_count: usize,
    pub(crate) image_bearing_connector_segment_signature_range_missing_count: usize,
    pub(crate) owner_parent_span_before_axis_rule_parent_span_count: usize,
    pub(crate) owner_parent_span_after_axis_rule_parent_span_count: usize,
    pub(crate) owner_parent_span_inside_axis_rule_parent_span_count: usize,
    pub(crate) axis_rule_parent_span_inside_owner_parent_span_count: usize,
    pub(crate) owner_parent_span_overlaps_axis_rule_parent_span_count: usize,
    pub(crate) owner_axis_rule_parent_span_missing_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FdmConnectorTextGridPoint {
    pub(crate) x_units: f32,
    pub(crate) group_index_float: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FdmConnectorLineRuleDistance {
    pub(crate) axis_delta: f32,
    pub(crate) inline_delta: f32,
    pub(crate) distance_grid: f32,
    pub(crate) closest_x_units: f32,
    pub(crate) closest_group_index: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FdmConnectorLineRuleEndpointMatchSummary {
    pub(crate) start_match_count: usize,
    pub(crate) end_match_count: usize,
    pub(crate) total_match_count: usize,
    pub(crate) tight_match_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail {
    pub(crate) summary: FdmConnectorLineRuleEndpointMatchSummary,
    pub(crate) start_tight_match_count: usize,
    pub(crate) end_tight_match_count: usize,
    pub(crate) axis_rule_endpoint_match_marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
    pub(crate) axis_rule_match_parent_relative_offset_min: Option<usize>,
    pub(crate) axis_rule_match_parent_relative_offset_max: Option<usize>,
}

impl FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail {
    pub(crate) fn tight_dual_endpoint_match(self) -> bool {
        self.start_tight_match_count > 0 && self.end_tight_match_count > 0
    }
}

impl FdmConnectorOrderTraceSummary {
    pub(crate) fn readiness_blocked_reason(self) -> &'static str {
        if self.trace_count == 0 {
            "no-tight-non-diagonal-dual-endpoint-axis-rule-connectors"
        } else if self.source_segment_matches_index_entry_count < self.trace_count {
            "connector-source-segment-membership-incomplete"
        } else if self.entry_connector_candidate_count < self.trace_count {
            "fdm-index-entry-connector-membership-incomplete"
        } else if self.image_bearing_segment_count > 0 {
            if self.image_bearing_complete_payload_segment_count == 0 {
                "image-signature-fragment-role-unproven"
            } else {
                "image-bearing-segment-paint-order-unproven"
            }
        } else if self.parent_normalized_ordered_same_row_same_connector_count == 0 {
            "no-parent-normalized-ordered-same-row-same-connector"
        } else if self.bbox_missing_count > 0 {
            "connector-fdm-index-bbox-relation-missing"
        } else if self.connector_axis_rule_parent_span_missing_count > 0 {
            "connector-axis-rule-parent-span-relation-missing"
        } else if self.owner_axis_rule_parent_span_missing_count > 0 {
            "owner-axis-rule-parent-span-relation-missing"
        } else {
            "connector-ownership-and-paint-order-unproven"
        }
    }

    pub(crate) fn promotion_ready(self) -> bool {
        self.readiness_blocked_reason() == "connector-ownership-and-paint-order-unproven"
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmConnectorEndpointOwnerMatchSummary {
    pub(crate) start_candidate_count: usize,
    pub(crate) end_candidate_count: usize,
    pub(crate) total_candidate_count: usize,
    pub(crate) start_within_probe_count: usize,
    pub(crate) end_within_probe_count: usize,
    pub(crate) within_probe_candidate_count: usize,
    pub(crate) fdm_primitive_candidate_count: usize,
    pub(crate) document_text_slot_candidate_count: usize,
    pub(crate) connector_command_index: usize,
    pub(crate) connector_parent_command_index: usize,
    pub(crate) connector_synthetic_nested_command: bool,
    pub(crate) connector_relative_offset: usize,
    pub(crate) connector_parent_relative_offset: Option<usize>,
    pub(crate) start_nearest_fdm_owner_row_index: Option<usize>,
    pub(crate) start_nearest_fdm_owner_command_index: Option<usize>,
    pub(crate) start_nearest_fdm_owner_parent_command_index: Option<usize>,
    pub(crate) start_nearest_fdm_owner_synthetic_nested_command: bool,
    pub(crate) start_nearest_fdm_owner_relative_offset: Option<usize>,
    pub(crate) start_nearest_fdm_owner_parent_relative_offset: Option<usize>,
    pub(crate) end_nearest_fdm_owner_row_index: Option<usize>,
    pub(crate) end_nearest_fdm_owner_command_index: Option<usize>,
    pub(crate) end_nearest_fdm_owner_parent_command_index: Option<usize>,
    pub(crate) end_nearest_fdm_owner_synthetic_nested_command: bool,
    pub(crate) end_nearest_fdm_owner_relative_offset: Option<usize>,
    pub(crate) end_nearest_fdm_owner_parent_relative_offset: Option<usize>,
    pub(crate) nearest_fdm_owner_rows_match: bool,
    pub(crate) nearest_fdm_owner_row_matches_connector_row: bool,
    pub(crate) mixed_top_level_vs_nested_order_namespace: bool,
    pub(crate) connector_command_between_nearest_fdm_owner_commands: bool,
    pub(crate) connector_command_before_nearest_fdm_owner_commands: bool,
    pub(crate) connector_command_after_nearest_fdm_owner_commands: bool,
    pub(crate) connector_relative_offset_between_nearest_fdm_owner_offsets: bool,
    pub(crate) connector_relative_offset_before_nearest_fdm_owner_offsets: bool,
    pub(crate) connector_relative_offset_after_nearest_fdm_owner_offsets: bool,
    pub(crate) connector_parent_command_between_nearest_fdm_owner_parent_commands: bool,
    pub(crate) connector_parent_command_before_nearest_fdm_owner_parent_commands: bool,
    pub(crate) connector_parent_command_after_nearest_fdm_owner_parent_commands: bool,
    pub(crate) connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets: bool,
    pub(crate) connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets: bool,
    pub(crate) connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmConnectorGraphDiagnosticSummary {
    pub(crate) page_paint_coverage_summary: FdmPagePaintCoverageSummary,
    pub(crate) connector_candidate_count: usize,
    pub(crate) line_rule_projection_count: usize,
    pub(crate) fdm_open_stroke_axis_rule_projection_count: usize,
    pub(crate) connector_endpoint_probe_count: usize,
    pub(crate) total_thresholded_endpoint_match_count: usize,
    pub(crate) matched_connector_count: usize,
    pub(crate) dual_endpoint_match_connector_count: usize,
    pub(crate) start_endpoint_line_rule_match_connector_count: usize,
    pub(crate) end_endpoint_line_rule_match_connector_count: usize,
    pub(crate) start_only_line_rule_match_connector_count: usize,
    pub(crate) end_only_line_rule_match_connector_count: usize,
    pub(crate) tight_endpoint_match_count: usize,
    pub(crate) nearby_endpoint_match_count: usize,
    pub(crate) no_thresholded_line_rule_endpoint_match_connector_count: usize,
    pub(crate) single_or_missing_endpoint_line_rule_match_connector_count: usize,
    pub(crate) connector_ownership_and_paint_order_unproven_connector_count: usize,
    pub(crate) endpoint_owner_candidate_connector_count: usize,
    pub(crate) endpoint_owner_probe_count: usize,
    pub(crate) total_endpoint_owner_candidate_count: usize,
    pub(crate) within_probe_endpoint_owner_candidate_count: usize,
    pub(crate) fdm_primitive_endpoint_owner_candidate_count: usize,
    pub(crate) document_text_slot_endpoint_owner_candidate_count: usize,
    pub(crate) start_endpoint_owner_within_probe_connector_count: usize,
    pub(crate) end_endpoint_owner_within_probe_connector_count: usize,
    pub(crate) dual_endpoint_owner_within_probe_connector_count: usize,
    pub(crate) owner_proven_connector_count: usize,
    pub(crate) dual_endpoint_nearest_fdm_owner_same_row_connector_count: usize,
    pub(crate) dual_endpoint_nearest_fdm_owner_row_mismatch_connector_count: usize,
    pub(crate) dual_endpoint_nearest_fdm_owner_same_connector_row_count: usize,
    pub(crate) connector_command_between_nearest_fdm_owner_commands_count: usize,
    pub(crate) connector_command_before_nearest_fdm_owner_commands_count: usize,
    pub(crate) connector_command_after_nearest_fdm_owner_commands_count: usize,
    pub(crate) ordered_same_row_same_connector_count: usize,
    pub(crate) parent_normalized_ordered_same_row_same_connector_count: usize,
    pub(crate) missing_endpoint_owner_candidate_connector_count: usize,
    pub(crate) nearest_owner_row_mismatch_connector_count: usize,
    pub(crate) owner_row_candidate_unproven_connector_count: usize,
    pub(crate) owner_grouping_proven_connector_count: usize,
    pub(crate) skipped_inline_line_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary,
    pub(crate) vertical_anchor_line_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary,
    pub(crate) fdm_open_stroke_axis_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary,
    pub(crate) ordered_owner_row_match_summary: FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) non_ordered_owner_row_match_summary: FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) parent_normalized_ordered_owner_row_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) parent_normalized_non_ordered_owner_row_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) missing_endpoint_owner_relation_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) nearest_owner_row_mismatch_relation_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) nearest_owner_row_not_connector_row_relation_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_mixed_command_namespace_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_before_owner_command_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_between_owner_command_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_after_owner_command_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_owner_command_relation_unclassified_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) missing_endpoint_owner_source_order_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) nearest_owner_row_mismatch_source_order_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) nearest_owner_row_not_connector_row_source_order_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_before_owner_relative_offset_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_between_owner_relative_offset_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_after_owner_relative_offset_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) same_row_relative_offset_relation_unclassified_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(crate) fdm_open_stroke_axis_rule_row_cohort_count: usize,
    pub(crate) fdm_open_stroke_axis_rule_row_cohorts:
        [FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary;
            FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT],
    pub(crate) fdm_open_stroke_axis_rule_owner_promotion_gate_summary:
        FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary,
    pub(crate) same_row_axis_rule_connector_order_trace_summary: FdmConnectorOrderTraceSummary,
    pub(crate) dominant_matched_connector_row_index: Option<usize>,
    pub(crate) dominant_matched_connector_row_connector_candidate_count: usize,
    pub(crate) dominant_matched_connector_row_total_thresholded_endpoint_match_count: usize,
    pub(crate) dominant_matched_connector_row_matched_connector_count: usize,
    pub(crate) dominant_matched_connector_row_dual_endpoint_match_connector_count: usize,
    pub(crate) dominant_matched_connector_row_start_only_match_connector_count: usize,
    pub(crate) dominant_matched_connector_row_end_only_match_connector_count: usize,
    pub(crate) dominant_matched_connector_row_tight_endpoint_match_count: usize,
    pub(crate) dominant_matched_connector_row_nearby_endpoint_match_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmPagePaintCoverageSummary {
    pub(crate) inspected_primitive_count: usize,
    pub(crate) rendered_primitive_count: usize,
    pub(crate) large_span_filtered_primitive_count: usize,
    pub(crate) closed_fill_primitive_count: usize,
    pub(crate) page_fill_candidate_count: usize,
    pub(crate) max_page_coverage_ratio_ppm: u32,
    pub(crate) max_viewport_coverage_ratio_ppm: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct FdmTextMaskCohortDiagnosticSummary {
    pub(crate) row_index: usize,
    pub(crate) primitive_count: usize,
    pub(crate) black_fill_primitive_count: usize,
    pub(crate) white_fill_primitive_count: usize,
    pub(crate) counter_overlay_count: usize,
    pub(crate) command_index_min: Option<usize>,
    pub(crate) command_index_max: Option<usize>,
    pub(crate) relative_offset_min: Option<usize>,
    pub(crate) relative_offset_max: Option<usize>,
    pub(crate) source_bbox: Option<(i32, i32, i32, i32)>,
    pub(crate) projected_bbox: Option<(f32, f32, f32, f32)>,
    pub(crate) component_count: usize,
    pub(crate) top_text_like_component: Option<FdmTextMaskComponentDiagnosticSummary>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct FdmTextMaskComponentDiagnosticSummary {
    pub(crate) component_index: usize,
    pub(crate) primitive_count: usize,
    pub(crate) black_fill_primitive_count: usize,
    pub(crate) white_fill_primitive_count: usize,
    pub(crate) counter_overlay_count: usize,
    pub(crate) command_index_min: Option<usize>,
    pub(crate) command_index_max: Option<usize>,
    pub(crate) relative_offset_min: Option<usize>,
    pub(crate) relative_offset_max: Option<usize>,
    pub(crate) source_bbox: Option<(i32, i32, i32, i32)>,
    pub(crate) projected_bbox: Option<(f32, f32, f32, f32)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FdmTextMaskPrimitiveDiagnosticSummary {
    pub(crate) command_index: usize,
    pub(crate) relative_offset: usize,
    pub(crate) source_bbox: Option<(i32, i32, i32, i32)>,
    pub(crate) projected_bbox: (f32, f32, f32, f32),
    pub(crate) black_fill: bool,
    pub(crate) white_fill: bool,
    pub(crate) counter_overlay: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmConnectorRuleSetMatchDiagnosticSummary {
    pub(crate) line_rule_projection_count: usize,
    pub(crate) connector_candidate_count: usize,
    pub(crate) connector_endpoint_probe_count: usize,
    pub(crate) total_thresholded_endpoint_match_count: usize,
    pub(crate) matched_connector_count: usize,
    pub(crate) dual_endpoint_match_connector_count: usize,
    pub(crate) tight_endpoint_match_count: usize,
    pub(crate) nearby_endpoint_match_count: usize,
    pub(crate) no_thresholded_line_rule_endpoint_match_connector_count: usize,
    pub(crate) single_or_missing_endpoint_line_rule_match_connector_count: usize,
    pub(crate) connector_ownership_and_paint_order_unproven_connector_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
    pub(crate) row_index: usize,
    pub(crate) connector_candidate_count: usize,
    pub(crate) total_thresholded_endpoint_match_count: usize,
    pub(crate) matched_connector_count: usize,
    pub(crate) dual_endpoint_match_connector_count: usize,
    pub(crate) tight_endpoint_match_count: usize,
    pub(crate) nearby_endpoint_match_count: usize,
    pub(crate) tight_dual_endpoint_match_connector_count: usize,
    pub(crate) horizontal_dual_endpoint_match_connector_count: usize,
    pub(crate) vertical_dual_endpoint_match_connector_count: usize,
    pub(crate) diagonal_dual_endpoint_match_connector_count: usize,
    pub(crate) horizontal_tight_dual_endpoint_match_connector_count: usize,
    pub(crate) vertical_tight_dual_endpoint_match_connector_count: usize,
    pub(crate) diagonal_tight_dual_endpoint_match_connector_count: usize,
    pub(crate) tight_non_diagonal_dual_projected_bbox_x_min_milli: Option<i32>,
    pub(crate) tight_non_diagonal_dual_projected_bbox_y_min_milli: Option<i32>,
    pub(crate) tight_non_diagonal_dual_projected_bbox_x_max_milli: Option<i32>,
    pub(crate) tight_non_diagonal_dual_projected_bbox_y_max_milli: Option<i32>,
    pub(crate) matched_projected_bbox_x_min_milli: Option<i32>,
    pub(crate) matched_projected_bbox_y_min_milli: Option<i32>,
    pub(crate) matched_projected_bbox_x_max_milli: Option<i32>,
    pub(crate) matched_projected_bbox_y_max_milli: Option<i32>,
    pub(crate) dual_projected_bbox_x_min_milli: Option<i32>,
    pub(crate) dual_projected_bbox_y_min_milli: Option<i32>,
    pub(crate) dual_projected_bbox_x_max_milli: Option<i32>,
    pub(crate) dual_projected_bbox_y_max_milli: Option<i32>,
    pub(crate) owner_promotion_gate_summary:
        FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary,
    pub(crate) matched_connector_marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
    pub(crate) dual_connector_marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
    pub(crate) tight_non_diagonal_dual_connector_marker_style_profile:
        FdmOpenStrokeMarkerStyleProfile,
    pub(crate) axis_rule_endpoint_match_marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
    pub(crate) fdm_index_vector_offset: Option<usize>,
    pub(crate) fdm_index_vector_len: Option<usize>,
    pub(crate) fdm_index_valid_vector_offset: bool,
    pub(crate) fdm_index_image_signature_count: usize,
    pub(crate) fdm_index_segment_image_signature_count: usize,
    pub(crate) fdm_index_vector_command_count: Option<usize>,
    pub(crate) fdm_index_connector_candidate_count: Option<usize>,
    pub(crate) fdm_index_non_connector_command_count: Option<usize>,
    pub(crate) fdm_index_source_segment_relative_offset: Option<usize>,
    pub(crate) fdm_index_source_segment_command_count: Option<usize>,
    pub(crate) fdm_index_bbox_left: Option<i32>,
    pub(crate) fdm_index_bbox_top: Option<i32>,
    pub(crate) fdm_index_bbox_right: Option<i32>,
    pub(crate) fdm_index_bbox_bottom: Option<i32>,
    pub(crate) source_segment_backed_connector_count: usize,
    pub(crate) source_segment_matches_index_entry_connector_count: usize,
    pub(crate) source_segment_missing_connector_count: usize,
    pub(crate) dual_endpoint_source_segment_backed_connector_count: usize,
    pub(crate) dual_endpoint_source_segment_matches_index_entry_connector_count: usize,
    pub(crate) dual_endpoint_image_bearing_segment_connector_count: usize,
    pub(crate) fdm_index_bbox_contains_connector_count: usize,
    pub(crate) fdm_index_bbox_overlaps_connector_count: usize,
    pub(crate) fdm_index_bbox_disjoint_connector_count: usize,
    pub(crate) fdm_index_bbox_source_bbox_missing_connector_count: usize,
    pub(crate) dual_endpoint_fdm_index_bbox_contains_connector_count: usize,
    pub(crate) dual_endpoint_fdm_index_bbox_overlaps_connector_count: usize,
    pub(crate) dual_endpoint_fdm_index_bbox_disjoint_connector_count: usize,
    pub(crate) dual_endpoint_fdm_index_bbox_source_bbox_missing_connector_count: usize,
    pub(crate) dual_endpoint_axis_rule_source_order_backed_connector_count: usize,
    pub(crate) dual_endpoint_connector_parent_relative_offset_min: Option<usize>,
    pub(crate) dual_endpoint_connector_parent_relative_offset_max: Option<usize>,
    pub(crate) dual_endpoint_axis_rule_parent_relative_offset_min: Option<usize>,
    pub(crate) dual_endpoint_axis_rule_parent_relative_offset_max: Option<usize>,
    pub(crate) dual_endpoint_connector_before_axis_rule_parent_span_count: usize,
    pub(crate) dual_endpoint_connector_between_axis_rule_parent_span_count: usize,
    pub(crate) dual_endpoint_connector_after_axis_rule_parent_span_count: usize,
    pub(crate) dual_endpoint_connector_axis_rule_parent_span_unclassified_count: usize,
}

impl FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
    pub(crate) fn non_diagonal_dual_endpoint_match_connector_count(self) -> usize {
        self.horizontal_dual_endpoint_match_connector_count
            + self.vertical_dual_endpoint_match_connector_count
    }

    pub(crate) fn non_diagonal_tight_dual_endpoint_match_connector_count(self) -> usize {
        self.horizontal_tight_dual_endpoint_match_connector_count
            + self.vertical_tight_dual_endpoint_match_connector_count
    }

    pub(crate) fn image_bearing_segment_candidate(self) -> bool {
        self.fdm_index_image_signature_count > 0 || self.fdm_index_segment_image_signature_count > 0
    }

    pub(crate) fn fdm_index_segment_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_source_segment_backed_connector_count
            < self.dual_endpoint_match_connector_count
        {
            "connector-source-segment-membership-missing"
        } else if self.dual_endpoint_source_segment_matches_index_entry_connector_count
            < self.dual_endpoint_match_connector_count
        {
            "connector-source-segment-does-not-match-fdm-index-row"
        } else if self.dual_endpoint_image_bearing_segment_connector_count > 0 {
            "fdm-index-image-bearing-segment-role-unproven"
        } else {
            "fdm-index-segment-ownership-and-paint-order-unproven"
        }
    }

    pub(crate) fn fdm_index_connector_composition_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_image_bearing_segment_connector_count > 0 {
            "fdm-index-image-bearing-segment-role-unproven"
        } else if self.fdm_index_connector_candidate_count.unwrap_or_default() == 0 {
            "fdm-index-connector-candidate-composition-missing"
        } else if self.dual_endpoint_fdm_index_bbox_contains_connector_count
            == self.dual_endpoint_match_connector_count
        {
            "fdm-index-contained-composition-internal-stroke-role-unproven"
        } else if self.dual_endpoint_fdm_index_bbox_disjoint_connector_count
            == self.dual_endpoint_match_connector_count
        {
            "fdm-index-disjoint-connector-composition-ownership-and-paint-order-unproven"
        } else {
            "fdm-index-mixed-connector-composition-ownership-and-paint-order-unproven"
        }
    }

    pub(crate) fn fdm_index_bbox_relation_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_fdm_index_bbox_source_bbox_missing_connector_count > 0 {
            "connector-source-bbox-missing"
        } else if self.dual_endpoint_image_bearing_segment_connector_count > 0 {
            "fdm-index-image-bearing-segment-role-unproven"
        } else if self.dual_endpoint_fdm_index_bbox_contains_connector_count
            == self.dual_endpoint_match_connector_count
        {
            "fdm-index-bbox-contained-internal-stroke-role-unproven"
        } else if self.dual_endpoint_fdm_index_bbox_disjoint_connector_count
            == self.dual_endpoint_match_connector_count
        {
            "fdm-index-bbox-disjoint-connector-ownership-unproven"
        } else {
            "fdm-index-bbox-mixed-connector-relation-unproven"
        }
    }

    pub(crate) fn axis_rule_source_order_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_axis_rule_source_order_backed_connector_count
            < self.dual_endpoint_match_connector_count
        {
            "axis-rule-source-order-evidence-missing"
        } else if self.dual_endpoint_connector_axis_rule_parent_span_unclassified_count > 0 {
            "axis-rule-source-order-relation-unclassified"
        } else if self.dual_endpoint_connector_before_axis_rule_parent_span_count
            == self.dual_endpoint_match_connector_count
        {
            "connector-before-axis-rule-parent-span-paint-order-unproven"
        } else if self.dual_endpoint_connector_between_axis_rule_parent_span_count
            == self.dual_endpoint_match_connector_count
        {
            "connector-between-axis-rule-parent-span-paint-order-unproven"
        } else if self.dual_endpoint_connector_after_axis_rule_parent_span_count
            == self.dual_endpoint_match_connector_count
        {
            "connector-after-axis-rule-parent-span-paint-order-unproven"
        } else {
            "mixed-connector-axis-rule-parent-span-paint-order-unproven"
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
    pub(crate) dual_endpoint_match_connector_count: usize,
    pub(crate) dual_endpoint_owner_candidate_count: usize,
    pub(crate) nearest_fdm_owner_rows_match_count: usize,
    pub(crate) nearest_fdm_owner_row_matches_connector_row_count: usize,
    pub(crate) mixed_top_level_vs_nested_order_namespace_count: usize,
    pub(crate) parent_normalized_ordered_same_row_same_connector_count: usize,
    pub(crate) missing_endpoint_owner_candidate_count: usize,
    pub(crate) nearest_owner_row_mismatch_count: usize,
    pub(crate) nearest_owner_row_not_connector_row_count: usize,
    pub(crate) before_owner_parent_command_span_count: usize,
    pub(crate) between_owner_parent_command_span_count: usize,
    pub(crate) after_owner_parent_command_span_count: usize,
    pub(crate) parent_command_relation_unclassified_count: usize,
    pub(crate) before_owner_parent_relative_offset_span_count: usize,
    pub(crate) between_owner_parent_relative_offset_span_count: usize,
    pub(crate) after_owner_parent_relative_offset_span_count: usize,
    pub(crate) parent_relative_offset_relation_unclassified_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmOpenStrokeMarkerStyleProfile {
    pub(crate) command_count: usize,
    pub(crate) line_marker_count: usize,
    pub(crate) path_marker_count: usize,
    pub(crate) bezier_marker_count: usize,
    pub(crate) ellipse_marker_count: usize,
    pub(crate) other_marker_count: usize,
    pub(crate) style_0000_count: usize,
    pub(crate) style_0005_count: usize,
    pub(crate) style_0080_count: usize,
    pub(crate) style_00a0_count: usize,
    pub(crate) other_style_count: usize,
}

impl FdmOpenStrokeMarkerStyleProfile {
    pub(crate) fn marker_family_diversity_count(self) -> usize {
        [
            self.line_marker_count,
            self.path_marker_count,
            self.bezier_marker_count,
            self.ellipse_marker_count,
            self.other_marker_count,
        ]
        .into_iter()
        .filter(|count| *count > 0)
        .count()
    }

    pub(crate) fn style_word_diversity_count(self) -> usize {
        [
            self.style_0000_count,
            self.style_0005_count,
            self.style_0080_count,
            self.style_00a0_count,
            self.other_style_count,
        ]
        .into_iter()
        .filter(|count| *count > 0)
        .count()
    }

    pub(crate) fn dominant_marker_family(self) -> (&'static str, usize) {
        [
            ("line-marker", self.line_marker_count),
            ("path-marker", self.path_marker_count),
            ("bezier-marker", self.bezier_marker_count),
            ("ellipse-marker", self.ellipse_marker_count),
            ("other-marker", self.other_marker_count),
        ]
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap_or(("none", 0))
    }

    pub(crate) fn dominant_style_word(self) -> (&'static str, usize) {
        [
            ("0x0000", self.style_0000_count),
            ("0x0005", self.style_0005_count),
            ("0x0080", self.style_0080_count),
            ("0x00a0", self.style_00a0_count),
            ("other-style", self.other_style_count),
        ]
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap_or(("none", 0))
    }

    pub(crate) fn marker_family_homogeneous(self) -> bool {
        self.command_count > 0 && self.marker_family_diversity_count() == 1
    }

    pub(crate) fn style_word_homogeneous(self) -> bool {
        self.command_count > 0 && self.style_word_diversity_count() == 1
    }

    pub(crate) fn homogeneous_marker_style_candidate(self) -> bool {
        self.marker_family_homogeneous() && self.style_word_homogeneous()
    }

    pub(crate) fn marker_style_role_promotion_blocked_reason(self) -> &'static str {
        if self.command_count == 0 {
            "marker-style-profile-empty"
        } else if !self.marker_family_homogeneous() && !self.style_word_homogeneous() {
            "mixed-marker-family-and-style-word-role-unproven"
        } else if !self.marker_family_homogeneous() {
            "mixed-marker-family-role-unproven"
        } else if !self.style_word_homogeneous() {
            "mixed-style-word-role-unproven"
        } else {
            "homogeneous-marker-style-still-needs-owner-and-paint-order"
        }
    }
}

impl FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
    pub(crate) fn parent_normalized_order_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_owner_candidate_count
            < self.dual_endpoint_match_connector_count
        {
            "missing-axis-rule-dual-endpoint-owner-candidate"
        } else if self.nearest_fdm_owner_rows_match_count < self.dual_endpoint_match_connector_count
        {
            "nearest-owner-row-mismatch"
        } else if self.nearest_fdm_owner_row_matches_connector_row_count
            < self.dual_endpoint_match_connector_count
        {
            "nearest-owner-row-not-connector-row"
        } else if self.parent_command_relation_unclassified_count > 0 {
            "connector-parent-command-relation-unclassified"
        } else if self.parent_relative_offset_relation_unclassified_count > 0 {
            "connector-parent-relative-offset-relation-unclassified"
        } else if self.between_owner_parent_command_span_count
            < self.dual_endpoint_match_connector_count
        {
            "connector-parent-command-outside-nearest-owner-parent-command-span"
        } else if self.between_owner_parent_relative_offset_span_count
            < self.dual_endpoint_match_connector_count
        {
            "connector-parent-relative-offset-outside-nearest-owner-parent-relative-offset-span"
        } else if self.parent_normalized_ordered_same_row_same_connector_count
            < self.dual_endpoint_match_connector_count
        {
            "parent-command-source-order-axis-disagreement"
        } else {
            "none"
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmConnectorOwnerRowCohortDiagnosticSummary {
    pub(crate) connector_candidate_count: usize,
    pub(crate) total_thresholded_endpoint_match_count: usize,
    pub(crate) matched_connector_count: usize,
    pub(crate) dual_endpoint_match_connector_count: usize,
    pub(crate) tight_endpoint_match_count: usize,
    pub(crate) nearby_endpoint_match_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FdmConnectorMatchedRowDiagnosticSummary {
    pub(crate) connector_candidate_count: usize,
    pub(crate) total_thresholded_endpoint_match_count: usize,
    pub(crate) matched_connector_count: usize,
    pub(crate) dual_endpoint_match_connector_count: usize,
    pub(crate) start_only_match_connector_count: usize,
    pub(crate) end_only_match_connector_count: usize,
    pub(crate) tight_endpoint_match_count: usize,
    pub(crate) nearby_endpoint_match_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct FdmOpenStrokeCohortSummary {
    pub(crate) primitive_count: usize,
    pub(crate) open_stroke_count: usize,
    pub(crate) connector_candidate_count: usize,
    pub(crate) horizontal_count: usize,
    pub(crate) vertical_count: usize,
    pub(crate) diagonal_count: usize,
    pub(crate) line_marker_count: usize,
    pub(crate) non_line_marker_count: usize,
    pub(crate) row_count: usize,
    pub(crate) dominant_connector_row_index: Option<usize>,
    pub(crate) dominant_connector_row_connector_candidate_count: usize,
    pub(crate) dominant_connector_row_open_stroke_count: usize,
    pub(crate) dominant_connector_row_horizontal_count: usize,
    pub(crate) dominant_connector_row_vertical_count: usize,
    pub(crate) row_cohorts: Vec<FdmOpenStrokeRowCohortSummary>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct FdmOpenStrokeRowCohortSummary {
    pub(crate) row_index: usize,
    pub(crate) open_stroke_count: usize,
    pub(crate) connector_candidate_count: usize,
    pub(crate) horizontal_count: usize,
    pub(crate) vertical_count: usize,
    pub(crate) diagonal_count: usize,
    pub(crate) line_marker_count: usize,
    pub(crate) non_line_marker_count: usize,
    pub(crate) command_index_min: Option<usize>,
    pub(crate) command_index_max: Option<usize>,
    pub(crate) relative_offset_min: Option<usize>,
    pub(crate) relative_offset_max: Option<usize>,
    pub(crate) source_bbox_union: Option<(i32, i32, i32, i32)>,
    pub(crate) projected_bbox_union: Option<(f32, f32, f32, f32)>,
    pub(crate) marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FdmOpenStrokeAxisRule<'a> {
    pub(crate) diagnostic: FdmCommandDiagnostic<'a>,
    pub(crate) orientation: &'static str,
    pub(crate) line_offset_units: f32,
    pub(crate) line_extent_units: f32,
    pub(crate) group_index: f32,
    pub(crate) end_group_index: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FdmConnectorEndpointOwnerCandidate<'a> {
    Primitive {
        diagnostic: FdmCommandDiagnostic<'a>,
        bbox: (f32, f32, f32, f32),
        distance_px: f32,
    },
    TextSlot {
        slot: &'a ShanaiLanTextSlot,
        bbox: (f32, f32, f32, f32),
        distance_px: f32,
    },
}

impl FdmConnectorEndpointOwnerCandidate<'_> {
    pub(crate) fn distance_px(self) -> f32 {
        match self {
            FdmConnectorEndpointOwnerCandidate::Primitive { distance_px, .. }
            | FdmConnectorEndpointOwnerCandidate::TextSlot { distance_px, .. } => distance_px,
        }
    }

    pub(crate) fn rank(self) -> usize {
        match self {
            FdmConnectorEndpointOwnerCandidate::Primitive { .. } => 0,
            FdmConnectorEndpointOwnerCandidate::TextSlot { .. } => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SuccessDataTestFdmProjection {
    pub(crate) role: &'static str,
    pub(crate) source_left: i32,
    pub(crate) source_top: i32,
    pub(crate) source_right: i32,
    pub(crate) source_bottom: i32,
    pub(crate) target_x_px: f32,
    pub(crate) target_y_px: f32,
    pub(crate) target_width_px: f32,
    pub(crate) target_height_px: f32,
    pub(crate) scale_mode: SuccessDataTestFdmScaleMode,
    pub(crate) text_corroboration_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SuccessDataTestFdmScaleMode {
    IndependentReferenceBox,
    UniformUnitsFromHorizontalSpan,
}

impl SuccessDataTestFdmScaleMode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            SuccessDataTestFdmScaleMode::IndependentReferenceBox => "independent-reference-box",
            SuccessDataTestFdmScaleMode::UniformUnitsFromHorizontalSpan => {
                "uniform-units-from-horizontal-span"
            }
        }
    }
}

pub(crate) type FdmTextMaskRightNeighborMatch<'a> =
    (&'a ShanaiLanTextSlot, (f32, f32, f32, f32), f32, f32, f32);

#[derive(Debug, Clone, Copy)]
pub(crate) struct FdmTextMaskRightNeighborCandidate<'a> {
    pub(crate) slot_index: usize,
    pub(crate) slot: &'a ShanaiLanTextSlot,
    pub(crate) bbox: (f32, f32, f32, f32),
    pub(crate) gap_px: f32,
    pub(crate) vertical_overlap_px: f32,
    pub(crate) center_delta_y_px: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FdmTextMaskPreFragmentBridgeMetrics {
    pub(crate) pre_fragment_unit_count: usize,
    pub(crate) pre_fragment_grid_units: usize,
    pub(crate) pre_fragment_projected_width_px: f32,
    pub(crate) line_start_x: f32,
    pub(crate) text_start_x: f32,
    pub(crate) source_begins_after_line_start: bool,
    pub(crate) source_ends_before_text_start: bool,
    pub(crate) source_bbox_within_pre_fragment_projection: bool,
    pub(crate) source_bbox_right_to_text_start_px: f32,
    pub(crate) text_baseline_minus_source_bottom_px: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FdmTextMaskSourceTransformCandidate<'a> {
    pub(crate) row_index: usize,
    pub(crate) candidate_class: &'static str,
    pub(crate) component_index: Option<usize>,
    pub(crate) slot_index: usize,
    pub(crate) slot: &'a ShanaiLanTextSlot,
    pub(crate) source_bbox: (i32, i32, i32, i32),
    pub(crate) projected_bbox: (f32, f32, f32, f32),
    pub(crate) metrics: FdmTextMaskPreFragmentBridgeMetrics,
    pub(crate) cohort_component_agreement: bool,
    pub(crate) current_projection_grid_start: f32,
    pub(crate) current_projection_grid_end: f32,
    pub(crate) current_projection_grid_span: f32,
    pub(crate) source_units_per_text_grid_unit_x: f32,
    pub(crate) line_start_source_x: f32,
    pub(crate) text_start_source_x: f32,
    pub(crate) source_gap_to_text_start_x: f32,
}
