use super::*;
use crate::*;

pub(crate) fn fdm_connector_endpoint_owner_probe_radius_px(
    text_projection: Option<&ShanaiLanTextProjection>,
) -> f32 {
    text_projection
        .map(|projection| projection.line_height_px)
        .filter(|value| *value > 0.0)
        .unwrap_or(FDM_CONNECTOR_ENDPOINT_OWNER_PROBE_RADIUS_PX)
}

pub(crate) fn fdm_connector_endpoint_owner_candidates<'a>(
    point: (f32, f32),
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'a>,
    extent: FdmCommandProjectionExtent,
    primitive_diagnostics: &'a [FdmCommandDiagnostic<'a>],
    text_projection: Option<&'a ShanaiLanTextProjection>,
) -> Vec<FdmConnectorEndpointOwnerCandidate<'a>> {
    let mut candidates = Vec::new();
    for diagnostic in primitive_diagnostics.iter().copied() {
        if fdm_command_diagnostic_same_command(connector, diagnostic)
            || fdm_connector_candidate_metric(layout, diagnostic, extent).is_some()
        {
            continue;
        }
        let Some(bbox) = fdm_path_diagnostic_bbox(layout, diagnostic, extent) else {
            continue;
        };
        let distance_px = distance_from_point_to_bbox(point.0, point.1, bbox);
        candidates.push(FdmConnectorEndpointOwnerCandidate::Primitive {
            diagnostic,
            bbox,
            distance_px,
        });
    }

    if let Some(projection) = text_projection {
        for slot in &projection.slots {
            let bbox = shanai_lan_text_slot_bbox(slot);
            let distance_px = distance_from_point_to_bbox(point.0, point.1, bbox);
            candidates.push(FdmConnectorEndpointOwnerCandidate::TextSlot {
                slot,
                bbox,
                distance_px,
            });
        }
    }

    candidates.sort_by(|left, right| {
        left.distance_px()
            .partial_cmp(&right.distance_px())
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.rank().cmp(&right.rank()))
    });
    candidates.truncate(FDM_CONNECTOR_ENDPOINT_OWNER_CANDIDATE_LIMIT);
    candidates
}

pub(crate) fn fdm_connector_endpoint_owner_match_summary(
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
) -> FdmConnectorEndpointOwnerMatchSummary {
    let probe_radius_px = fdm_connector_endpoint_owner_probe_radius_px(text_projection);
    let start = fdm_connector_endpoint_owner_candidates(
        metric.projected_start,
        layout,
        connector,
        extent,
        primitive_diagnostics,
        text_projection,
    );
    let end = fdm_connector_endpoint_owner_candidates(
        metric.projected_end,
        layout,
        connector,
        extent,
        primitive_diagnostics,
        text_projection,
    );
    let connector_command_index = connector.command.command_index();
    let connector_parent_relative_offset = fdm_command_normalized_parent_relative_offset(connector);
    let mut summary = FdmConnectorEndpointOwnerMatchSummary {
        start_candidate_count: start.len(),
        end_candidate_count: end.len(),
        total_candidate_count: start.len() + end.len(),
        connector_command_index,
        connector_parent_command_index: fdm_command_parent_command_index(connector_command_index),
        connector_synthetic_nested_command: fdm_command_index_is_synthetic_nested(
            connector_command_index,
        ),
        connector_relative_offset: connector.command.relative_offset(),
        connector_parent_relative_offset,
        ..Default::default()
    };
    accumulate_fdm_connector_endpoint_owner_candidates(&mut summary, &start, probe_radius_px, true);
    accumulate_fdm_connector_endpoint_owner_candidates(&mut summary, &end, probe_radius_px, false);
    if let Some(start_owner) = fdm_connector_nearest_within_probe_fdm_owner(&start, probe_radius_px)
    {
        let command_index = start_owner.command.command_index();
        summary.start_nearest_fdm_owner_row_index = Some(start_owner.entry.row_index());
        summary.start_nearest_fdm_owner_command_index = Some(command_index);
        summary.start_nearest_fdm_owner_parent_command_index =
            Some(fdm_command_parent_command_index(command_index));
        summary.start_nearest_fdm_owner_synthetic_nested_command =
            fdm_command_index_is_synthetic_nested(command_index);
        summary.start_nearest_fdm_owner_relative_offset =
            Some(start_owner.command.relative_offset());
        summary.start_nearest_fdm_owner_parent_relative_offset =
            fdm_command_normalized_parent_relative_offset(start_owner);
    }
    if let Some(end_owner) = fdm_connector_nearest_within_probe_fdm_owner(&end, probe_radius_px) {
        let command_index = end_owner.command.command_index();
        summary.end_nearest_fdm_owner_row_index = Some(end_owner.entry.row_index());
        summary.end_nearest_fdm_owner_command_index = Some(command_index);
        summary.end_nearest_fdm_owner_parent_command_index =
            Some(fdm_command_parent_command_index(command_index));
        summary.end_nearest_fdm_owner_synthetic_nested_command =
            fdm_command_index_is_synthetic_nested(command_index);
        summary.end_nearest_fdm_owner_relative_offset = Some(end_owner.command.relative_offset());
        summary.end_nearest_fdm_owner_parent_relative_offset =
            fdm_command_normalized_parent_relative_offset(end_owner);
    }
    if let (Some(start_row), Some(end_row)) = (
        summary.start_nearest_fdm_owner_row_index,
        summary.end_nearest_fdm_owner_row_index,
    ) {
        summary.nearest_fdm_owner_rows_match = start_row == end_row;
        summary.nearest_fdm_owner_row_matches_connector_row =
            start_row == connector.entry.row_index() && end_row == connector.entry.row_index();
    }
    if let (Some(start_command), Some(end_command)) = (
        summary.start_nearest_fdm_owner_command_index,
        summary.end_nearest_fdm_owner_command_index,
    ) {
        summary.mixed_top_level_vs_nested_order_namespace = summary
            .connector_synthetic_nested_command
            != summary.start_nearest_fdm_owner_synthetic_nested_command
            || summary.connector_synthetic_nested_command
                != summary.end_nearest_fdm_owner_synthetic_nested_command
            || summary.start_nearest_fdm_owner_synthetic_nested_command
                != summary.end_nearest_fdm_owner_synthetic_nested_command;
        let low = start_command.min(end_command);
        let high = start_command.max(end_command);
        let connector_command = summary.connector_command_index;
        summary.connector_command_between_nearest_fdm_owner_commands =
            low <= connector_command && connector_command <= high;
        summary.connector_command_before_nearest_fdm_owner_commands = connector_command < low;
        summary.connector_command_after_nearest_fdm_owner_commands = connector_command > high;

        let start_parent = fdm_command_parent_command_index(start_command);
        let end_parent = fdm_command_parent_command_index(end_command);
        let low_parent = start_parent.min(end_parent);
        let high_parent = start_parent.max(end_parent);
        let connector_parent = summary.connector_parent_command_index;
        summary.connector_parent_command_between_nearest_fdm_owner_parent_commands =
            low_parent <= connector_parent && connector_parent <= high_parent;
        summary.connector_parent_command_before_nearest_fdm_owner_parent_commands =
            connector_parent < low_parent;
        summary.connector_parent_command_after_nearest_fdm_owner_parent_commands =
            connector_parent > high_parent;
    }
    if let (Some(start_relative_offset), Some(end_relative_offset)) = (
        summary.start_nearest_fdm_owner_relative_offset,
        summary.end_nearest_fdm_owner_relative_offset,
    ) {
        let low = start_relative_offset.min(end_relative_offset);
        let high = start_relative_offset.max(end_relative_offset);
        let connector_relative_offset = summary.connector_relative_offset;
        summary.connector_relative_offset_between_nearest_fdm_owner_offsets =
            low <= connector_relative_offset && connector_relative_offset <= high;
        summary.connector_relative_offset_before_nearest_fdm_owner_offsets =
            connector_relative_offset < low;
        summary.connector_relative_offset_after_nearest_fdm_owner_offsets =
            connector_relative_offset > high;
    }
    if let (
        Some(connector_parent_relative_offset),
        Some(start_parent_relative_offset),
        Some(end_parent_relative_offset),
    ) = (
        summary.connector_parent_relative_offset,
        summary.start_nearest_fdm_owner_parent_relative_offset,
        summary.end_nearest_fdm_owner_parent_relative_offset,
    ) {
        let low = start_parent_relative_offset.min(end_parent_relative_offset);
        let high = start_parent_relative_offset.max(end_parent_relative_offset);
        summary.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets =
            low <= connector_parent_relative_offset && connector_parent_relative_offset <= high;
        summary.connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets =
            connector_parent_relative_offset < low;
        summary.connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets =
            connector_parent_relative_offset > high;
    }
    summary
}

pub(crate) fn fdm_connector_nearest_within_probe_fdm_owner<'a>(
    candidates: &[FdmConnectorEndpointOwnerCandidate<'a>],
    probe_radius_px: f32,
) -> Option<FdmCommandDiagnostic<'a>> {
    candidates.iter().find_map(|candidate| match candidate {
        FdmConnectorEndpointOwnerCandidate::Primitive {
            diagnostic,
            distance_px,
            ..
        } if *distance_px <= probe_radius_px => Some(*diagnostic),
        _ => None,
    })
}

pub(crate) fn accumulate_fdm_connector_endpoint_owner_candidates(
    summary: &mut FdmConnectorEndpointOwnerMatchSummary,
    candidates: &[FdmConnectorEndpointOwnerCandidate<'_>],
    probe_radius_px: f32,
    start: bool,
) {
    for candidate in candidates {
        match candidate {
            FdmConnectorEndpointOwnerCandidate::Primitive { .. } => {
                summary.fdm_primitive_candidate_count += 1;
            }
            FdmConnectorEndpointOwnerCandidate::TextSlot { .. } => {
                summary.document_text_slot_candidate_count += 1;
            }
        }
        if candidate.distance_px() <= probe_radius_px {
            summary.within_probe_candidate_count += 1;
            if start {
                summary.start_within_probe_count += 1;
            } else {
                summary.end_within_probe_count += 1;
            }
        }
    }
}

impl FdmConnectorEndpointOwnerMatchSummary {
    pub(crate) fn dual_endpoint_owner_candidate(self) -> bool {
        self.start_within_probe_count > 0 && self.end_within_probe_count > 0
    }

    pub(crate) fn ownership_promotion_blocked_reason(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else {
            "endpoint-owner-candidate-unproven"
        }
    }

    pub(crate) fn owner_grouping_promotion_blocked_reason(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else {
            "owner-row-candidate-unproven"
        }
    }

    pub(crate) fn owner_command_relation(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else if self.mixed_top_level_vs_nested_order_namespace {
            "same-row-mixed-command-namespace"
        } else if self.connector_command_before_nearest_fdm_owner_commands {
            "same-row-before-owner-command-span"
        } else if self.connector_command_between_nearest_fdm_owner_commands {
            "same-row-between-owner-command-span"
        } else if self.connector_command_after_nearest_fdm_owner_commands {
            "same-row-after-owner-command-span"
        } else {
            "same-row-command-relation-unclassified"
        }
    }

    pub(crate) fn owner_source_order_relation(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else if self.connector_relative_offset_before_nearest_fdm_owner_offsets {
            "same-row-before-owner-relative-offset-span"
        } else if self.connector_relative_offset_between_nearest_fdm_owner_offsets {
            "same-row-between-owner-relative-offset-span"
        } else if self.connector_relative_offset_after_nearest_fdm_owner_offsets {
            "same-row-after-owner-relative-offset-span"
        } else {
            "same-row-relative-offset-relation-unclassified"
        }
    }

    pub(crate) fn owner_parent_command_relation(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else if self.connector_parent_command_before_nearest_fdm_owner_parent_commands {
            "same-row-before-owner-parent-command-span"
        } else if self.connector_parent_command_between_nearest_fdm_owner_parent_commands {
            "same-row-between-owner-parent-command-span"
        } else if self.connector_parent_command_after_nearest_fdm_owner_parent_commands {
            "same-row-after-owner-parent-command-span"
        } else {
            "same-row-parent-command-relation-unclassified"
        }
    }

    pub(crate) fn owner_parent_source_order_relation(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else if self.connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets {
            "same-row-before-owner-parent-relative-offset-span"
        } else if self.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets {
            "same-row-between-owner-parent-relative-offset-span"
        } else if self.connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets {
            "same-row-after-owner-parent-relative-offset-span"
        } else {
            "same-row-parent-relative-offset-relation-unclassified"
        }
    }

    pub(crate) fn parent_normalized_ordered_same_row_same_connector(self) -> bool {
        self.nearest_fdm_owner_rows_match
            && self.nearest_fdm_owner_row_matches_connector_row
            && self.connector_parent_command_between_nearest_fdm_owner_parent_commands
            && self.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets
    }
}

pub(crate) fn fdm_command_diagnostic_same_command(
    left: FdmCommandDiagnostic<'_>,
    right: FdmCommandDiagnostic<'_>,
) -> bool {
    left.candidate_index == right.candidate_index
        && left.entry.row_index() == right.entry.row_index()
        && left.command.command_index() == right.command.command_index()
}

pub(crate) fn fdm_command_index_is_synthetic_nested(command_index: usize) -> bool {
    command_index >= 1000
}

pub(crate) fn fdm_command_parent_command_index(command_index: usize) -> usize {
    if fdm_command_index_is_synthetic_nested(command_index) {
        command_index / 1000
    } else {
        command_index
    }
}

pub(crate) fn fdm_command_normalized_parent_relative_offset(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> Option<usize> {
    if !fdm_command_index_is_synthetic_nested(diagnostic.command.command_index()) {
        return Some(diagnostic.command.relative_offset());
    }
    fdm_connector_parent_compound_provenance(diagnostic)
        .map(|provenance| provenance.parent.relative_offset())
}

pub(crate) fn fdm_connector_parent_compound_provenance(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> Option<FdmConnectorParentCompoundProvenance<'_>> {
    let command_index = diagnostic.command.command_index();
    if !fdm_command_index_is_synthetic_nested(command_index) {
        return None;
    }
    let parent_command_index = fdm_command_parent_command_index(command_index);
    let parent = diagnostic.entry.vector_commands().iter().find(|command| {
        command.command_index() == parent_command_index
            && command.marker() == FDM_VECTOR_COMMAND_BBOX_MARKER
    })?;
    let child_offset_in_parent = diagnostic
        .command
        .relative_offset()
        .checked_sub(parent.relative_offset())?;
    let child_offset_table_index = parent
        .compound_child_offsets()
        .iter()
        .position(|offset| usize::from(*offset) == child_offset_in_parent);
    Some(FdmConnectorParentCompoundProvenance {
        parent,
        child_offset_in_parent,
        child_offset_table_index,
    })
}

pub(crate) fn fdm_connector_graph_diagnostic_summary(
    layout: PageLayout,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
    text_projection: Option<&ShanaiLanTextProjection>,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) -> Option<FdmConnectorGraphDiagnosticSummary> {
    let projection = line_rule_projection?;
    let open_stroke_axis_rules =
        fdm_open_stroke_axis_rules(layout, primitive_diagnostics, extent, projection);
    let mut summary = FdmConnectorGraphDiagnosticSummary {
        page_paint_coverage_summary: fdm_page_paint_coverage_summary(
            layout,
            primitive_diagnostics,
            extent,
        ),
        line_rule_projection_count: projection.rules.len(),
        fdm_open_stroke_axis_rule_projection_count: open_stroke_axis_rules.len(),
        ..Default::default()
    };
    let mut row_summaries: BTreeMap<usize, FdmConnectorMatchedRowDiagnosticSummary> =
        BTreeMap::new();
    let mut open_stroke_axis_rule_row_summaries: BTreeMap<
        usize,
        FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    > = BTreeMap::new();
    let mut strict_order_trace_candidates = Vec::new();
    summary
        .skipped_inline_line_rule_match_summary
        .line_rule_projection_count = projection
        .rules
        .iter()
        .filter(|rule| rule.candidate_source == "skippedInlineLineHeader")
        .count();
    summary
        .vertical_anchor_line_rule_match_summary
        .line_rule_projection_count = projection
        .rules
        .iter()
        .filter(|rule| rule.candidate_source == "verticalAnchorRunFromLineHeaders")
        .count();
    summary
        .fdm_open_stroke_axis_rule_match_summary
        .line_rule_projection_count = open_stroke_axis_rules.len();

    for diagnostic in primitive_diagnostics.iter().copied() {
        let Some(metric) = fdm_connector_candidate_metric(layout, diagnostic, extent) else {
            continue;
        };
        let Some(endpoint_summary) =
            fdm_connector_line_rule_endpoint_match_summary(layout, metric, Some(projection))
        else {
            continue;
        };
        summary.connector_candidate_count += 1;
        summary.connector_endpoint_probe_count += 2;
        summary.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
        summary.tight_endpoint_match_count += endpoint_summary.tight_match_count;
        summary.nearby_endpoint_match_count +=
            endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
        if endpoint_summary.total_match_count > 0 {
            summary.matched_connector_count += 1;
        }
        if endpoint_summary.dual_endpoint_match() {
            summary.dual_endpoint_match_connector_count += 1;
        }
        if endpoint_summary.start_match_count > 0 {
            summary.start_endpoint_line_rule_match_connector_count += 1;
        }
        if endpoint_summary.end_match_count > 0 {
            summary.end_endpoint_line_rule_match_connector_count += 1;
        }
        if endpoint_summary.start_match_count > 0 && endpoint_summary.end_match_count == 0 {
            summary.start_only_line_rule_match_connector_count += 1;
        }
        if endpoint_summary.end_match_count > 0 && endpoint_summary.start_match_count == 0 {
            summary.end_only_line_rule_match_connector_count += 1;
        }
        let row_summary = row_summaries
            .entry(diagnostic.entry.row_index())
            .or_default();
        row_summary.connector_candidate_count += 1;
        row_summary.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
        row_summary.tight_endpoint_match_count += endpoint_summary.tight_match_count;
        row_summary.nearby_endpoint_match_count +=
            endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
        if endpoint_summary.total_match_count > 0 {
            row_summary.matched_connector_count += 1;
        }
        if endpoint_summary.dual_endpoint_match() {
            row_summary.dual_endpoint_match_connector_count += 1;
        }
        if endpoint_summary.start_match_count > 0 && endpoint_summary.end_match_count == 0 {
            row_summary.start_only_match_connector_count += 1;
        }
        if endpoint_summary.end_match_count > 0 && endpoint_summary.start_match_count == 0 {
            row_summary.end_only_match_connector_count += 1;
        }
        if let Some(skipped_inline_endpoint_summary) =
            fdm_connector_line_rule_endpoint_match_summary_for_candidate_source(
                layout,
                metric,
                projection,
                "skippedInlineLineHeader",
            )
        {
            accumulate_fdm_connector_rule_set_match_summary(
                &mut summary.skipped_inline_line_rule_match_summary,
                skipped_inline_endpoint_summary,
            );
        }
        if let Some(vertical_anchor_endpoint_summary) =
            fdm_connector_line_rule_endpoint_match_summary_for_candidate_source(
                layout,
                metric,
                projection,
                "verticalAnchorRunFromLineHeaders",
            )
        {
            accumulate_fdm_connector_rule_set_match_summary(
                &mut summary.vertical_anchor_line_rule_match_summary,
                vertical_anchor_endpoint_summary,
            );
        }
        let open_stroke_endpoint_detail = fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
            layout,
            diagnostic,
            metric,
            projection,
            &open_stroke_axis_rules,
        );
        if let Some(open_stroke_endpoint_detail) = open_stroke_endpoint_detail {
            accumulate_fdm_connector_rule_set_match_summary(
                &mut summary.fdm_open_stroke_axis_rule_match_summary,
                open_stroke_endpoint_detail.summary,
            );
            if open_stroke_endpoint_detail.tight_dual_endpoint_match()
                && metric.orientation != "diagonal"
            {
                strict_order_trace_candidates.push((
                    diagnostic,
                    metric,
                    open_stroke_endpoint_detail,
                ));
            }
            let open_stroke_row_summary = open_stroke_axis_rule_row_summaries
                .entry(diagnostic.entry.row_index())
                .or_insert_with(|| FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
                    row_index: diagnostic.entry.row_index(),
                    ..Default::default()
                });
            accumulate_fdm_open_stroke_axis_rule_row_cohort_summary(
                open_stroke_row_summary,
                diagnostic,
                metric,
                open_stroke_endpoint_detail,
            );
        }
        let owner_summary = fdm_connector_endpoint_owner_match_summary(
            layout,
            diagnostic,
            extent,
            metric,
            primitive_diagnostics,
            text_projection,
        );
        if owner_summary.total_candidate_count > 0 {
            summary.endpoint_owner_candidate_connector_count += 1;
        }
        summary.endpoint_owner_probe_count += 2;
        summary.total_endpoint_owner_candidate_count += owner_summary.total_candidate_count;
        summary.within_probe_endpoint_owner_candidate_count +=
            owner_summary.within_probe_candidate_count;
        summary.fdm_primitive_endpoint_owner_candidate_count +=
            owner_summary.fdm_primitive_candidate_count;
        summary.document_text_slot_endpoint_owner_candidate_count +=
            owner_summary.document_text_slot_candidate_count;
        if owner_summary.start_within_probe_count > 0 {
            summary.start_endpoint_owner_within_probe_connector_count += 1;
        }
        if owner_summary.end_within_probe_count > 0 {
            summary.end_endpoint_owner_within_probe_connector_count += 1;
        }
        if owner_summary.dual_endpoint_owner_candidate() {
            summary.dual_endpoint_owner_within_probe_connector_count += 1;
        }
        if owner_summary.nearest_fdm_owner_rows_match {
            summary.dual_endpoint_nearest_fdm_owner_same_row_connector_count += 1;
        } else if owner_summary.dual_endpoint_owner_candidate() {
            summary.dual_endpoint_nearest_fdm_owner_row_mismatch_connector_count += 1;
        }
        if owner_summary.nearest_fdm_owner_row_matches_connector_row {
            summary.dual_endpoint_nearest_fdm_owner_same_connector_row_count += 1;
        }
        if owner_summary.connector_command_between_nearest_fdm_owner_commands {
            summary.connector_command_between_nearest_fdm_owner_commands_count += 1;
        }
        if owner_summary.connector_command_before_nearest_fdm_owner_commands {
            summary.connector_command_before_nearest_fdm_owner_commands_count += 1;
        }
        if owner_summary.connector_command_after_nearest_fdm_owner_commands {
            summary.connector_command_after_nearest_fdm_owner_commands_count += 1;
        }
        if open_stroke_endpoint_detail
            .map(|detail| detail.summary.dual_endpoint_match())
            .unwrap_or(false)
        {
            accumulate_fdm_open_stroke_axis_rule_owner_promotion_gate_summary(
                &mut summary.fdm_open_stroke_axis_rule_owner_promotion_gate_summary,
                owner_summary,
            );
            let open_stroke_row_summary = open_stroke_axis_rule_row_summaries
                .entry(diagnostic.entry.row_index())
                .or_insert_with(|| FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
                    row_index: diagnostic.entry.row_index(),
                    ..Default::default()
                });
            accumulate_fdm_open_stroke_axis_rule_owner_promotion_gate_summary(
                &mut open_stroke_row_summary.owner_promotion_gate_summary,
                owner_summary,
            );
        }
        if owner_summary.parent_normalized_ordered_same_row_same_connector() {
            summary.parent_normalized_ordered_same_row_same_connector_count += 1;
            accumulate_fdm_connector_owner_row_cohort_match_summary(
                &mut summary.parent_normalized_ordered_owner_row_match_summary,
                endpoint_summary,
            );
        } else {
            accumulate_fdm_connector_owner_row_cohort_match_summary(
                &mut summary.parent_normalized_non_ordered_owner_row_match_summary,
                endpoint_summary,
            );
        }
        accumulate_fdm_connector_owner_command_relation_match_summary(
            &mut summary,
            owner_summary.owner_command_relation(),
            endpoint_summary,
        );
        accumulate_fdm_connector_owner_source_order_relation_match_summary(
            &mut summary,
            owner_summary.owner_source_order_relation(),
            endpoint_summary,
        );
        if owner_summary.nearest_fdm_owner_rows_match
            && owner_summary.nearest_fdm_owner_row_matches_connector_row
            && !owner_summary.mixed_top_level_vs_nested_order_namespace
            && owner_summary.connector_command_between_nearest_fdm_owner_commands
        {
            summary.ordered_same_row_same_connector_count += 1;
            accumulate_fdm_connector_owner_row_cohort_match_summary(
                &mut summary.ordered_owner_row_match_summary,
                endpoint_summary,
            );
        } else {
            accumulate_fdm_connector_owner_row_cohort_match_summary(
                &mut summary.non_ordered_owner_row_match_summary,
                endpoint_summary,
            );
        }
        match owner_summary.owner_grouping_promotion_blocked_reason() {
            "missing-endpoint-owner-candidate" => {
                summary.missing_endpoint_owner_candidate_connector_count += 1;
            }
            "nearest-owner-row-mismatch" => {
                summary.nearest_owner_row_mismatch_connector_count += 1;
            }
            "owner-row-candidate-unproven" => {
                summary.owner_row_candidate_unproven_connector_count += 1;
            }
            _ => {}
        }
        match endpoint_summary.graph_promotion_blocked_reason() {
            "no-thresholded-line-rule-endpoint-match" => {
                summary.no_thresholded_line_rule_endpoint_match_connector_count += 1;
            }
            "single-or-missing-endpoint-line-rule-match" => {
                summary.single_or_missing_endpoint_line_rule_match_connector_count += 1;
            }
            "connector-ownership-and-paint-order-unproven" => {
                summary.connector_ownership_and_paint_order_unproven_connector_count += 1;
            }
            _ => {}
        }
    }

    if let Some((row_index, row_summary)) = row_summaries
        .iter()
        .filter(|(_, row_summary)| row_summary.matched_connector_count > 0)
        .max_by(|(left_row_index, left), (right_row_index, right)| {
            left.matched_connector_count
                .cmp(&right.matched_connector_count)
                .then(
                    left.total_thresholded_endpoint_match_count
                        .cmp(&right.total_thresholded_endpoint_match_count),
                )
                .then(
                    left.dual_endpoint_match_connector_count
                        .cmp(&right.dual_endpoint_match_connector_count),
                )
                .then(
                    left.tight_endpoint_match_count
                        .cmp(&right.tight_endpoint_match_count),
                )
                .then_with(|| right_row_index.cmp(left_row_index))
        })
    {
        summary.dominant_matched_connector_row_index = Some(*row_index);
        summary.dominant_matched_connector_row_connector_candidate_count =
            row_summary.connector_candidate_count;
        summary.dominant_matched_connector_row_total_thresholded_endpoint_match_count =
            row_summary.total_thresholded_endpoint_match_count;
        summary.dominant_matched_connector_row_matched_connector_count =
            row_summary.matched_connector_count;
        summary.dominant_matched_connector_row_dual_endpoint_match_connector_count =
            row_summary.dual_endpoint_match_connector_count;
        summary.dominant_matched_connector_row_start_only_match_connector_count =
            row_summary.start_only_match_connector_count;
        summary.dominant_matched_connector_row_end_only_match_connector_count =
            row_summary.end_only_match_connector_count;
        summary.dominant_matched_connector_row_tight_endpoint_match_count =
            row_summary.tight_endpoint_match_count;
        summary.dominant_matched_connector_row_nearby_endpoint_match_count =
            row_summary.nearby_endpoint_match_count;
    }

    let mut open_stroke_axis_rule_row_cohorts = open_stroke_axis_rule_row_summaries
        .into_values()
        .collect::<Vec<_>>();
    open_stroke_axis_rule_row_cohorts.sort_by(|left, right| {
        right
            .dual_endpoint_match_connector_count
            .cmp(&left.dual_endpoint_match_connector_count)
            .then(
                right
                    .non_diagonal_dual_endpoint_match_connector_count()
                    .cmp(&left.non_diagonal_dual_endpoint_match_connector_count()),
            )
            .then(
                right
                    .tight_dual_endpoint_match_connector_count
                    .cmp(&left.tight_dual_endpoint_match_connector_count),
            )
            .then(
                right
                    .total_thresholded_endpoint_match_count
                    .cmp(&left.total_thresholded_endpoint_match_count),
            )
            .then_with(|| left.row_index.cmp(&right.row_index))
    });
    summary.fdm_open_stroke_axis_rule_row_cohort_count = open_stroke_axis_rule_row_cohorts
        .len()
        .min(FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT);
    for (index, row_summary) in open_stroke_axis_rule_row_cohorts
        .into_iter()
        .take(FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT)
        .enumerate()
    {
        summary.fdm_open_stroke_axis_rule_row_cohorts[index] = row_summary;
    }
    summary.same_row_axis_rule_connector_order_trace_summary = fdm_connector_order_trace_summary(
        layout,
        &strict_order_trace_candidates,
        primitive_diagnostics,
        extent,
        text_projection,
    );

    if summary.connector_candidate_count == 0 {
        None
    } else {
        Some(summary)
    }
}

pub(crate) fn accumulate_fdm_connector_rule_set_match_summary(
    target: &mut FdmConnectorRuleSetMatchDiagnosticSummary,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    target.connector_candidate_count += 1;
    target.connector_endpoint_probe_count += 2;
    target.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
    target.tight_endpoint_match_count += endpoint_summary.tight_match_count;
    target.nearby_endpoint_match_count +=
        endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
    if endpoint_summary.total_match_count > 0 {
        target.matched_connector_count += 1;
    }
    if endpoint_summary.dual_endpoint_match() {
        target.dual_endpoint_match_connector_count += 1;
    }
    match endpoint_summary.graph_promotion_blocked_reason() {
        "no-thresholded-line-rule-endpoint-match" => {
            target.no_thresholded_line_rule_endpoint_match_connector_count += 1;
        }
        "single-or-missing-endpoint-line-rule-match" => {
            target.single_or_missing_endpoint_line_rule_match_connector_count += 1;
        }
        "connector-ownership-and-paint-order-unproven" => {
            target.connector_ownership_and_paint_order_unproven_connector_count += 1;
        }
        _ => {}
    }
}

pub(crate) fn accumulate_fdm_open_stroke_axis_rule_row_cohort_summary(
    target: &mut FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    connector: FdmCommandDiagnostic<'_>,
    metric: FdmConnectorCandidateMetric,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
) {
    let endpoint_summary = detail.summary;
    accumulate_fdm_open_stroke_axis_rule_row_cohort_segment_gate(
        target,
        connector,
        endpoint_summary,
    );
    target.connector_candidate_count += 1;
    target.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
    target.tight_endpoint_match_count += endpoint_summary.tight_match_count;
    target.nearby_endpoint_match_count +=
        endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
    if endpoint_summary.total_match_count > 0 {
        target.matched_connector_count += 1;
        accumulate_fdm_open_stroke_marker_style_profile(
            &mut target.matched_connector_marker_style_profile,
            connector.command,
        );
        accumulate_fdm_open_stroke_marker_style_profile_from_profile(
            &mut target.axis_rule_endpoint_match_marker_style_profile,
            detail.axis_rule_endpoint_match_marker_style_profile,
        );
        accumulate_projected_bbox_union_milli(
            &mut target.matched_projected_bbox_x_min_milli,
            &mut target.matched_projected_bbox_y_min_milli,
            &mut target.matched_projected_bbox_x_max_milli,
            &mut target.matched_projected_bbox_y_max_milli,
            metric.projected_bbox,
        );
    }
    if !endpoint_summary.dual_endpoint_match() {
        return;
    }

    target.dual_endpoint_match_connector_count += 1;
    accumulate_fdm_open_stroke_axis_rule_source_order_gate(target, connector, detail);
    accumulate_fdm_open_stroke_marker_style_profile(
        &mut target.dual_connector_marker_style_profile,
        connector.command,
    );
    accumulate_projected_bbox_union_milli(
        &mut target.dual_projected_bbox_x_min_milli,
        &mut target.dual_projected_bbox_y_min_milli,
        &mut target.dual_projected_bbox_x_max_milli,
        &mut target.dual_projected_bbox_y_max_milli,
        metric.projected_bbox,
    );
    match metric.orientation {
        "horizontal" => {
            target.horizontal_dual_endpoint_match_connector_count += 1;
            if detail.tight_dual_endpoint_match() {
                target.horizontal_tight_dual_endpoint_match_connector_count += 1;
            }
        }
        "vertical" => {
            target.vertical_dual_endpoint_match_connector_count += 1;
            if detail.tight_dual_endpoint_match() {
                target.vertical_tight_dual_endpoint_match_connector_count += 1;
            }
        }
        _ => {
            target.diagonal_dual_endpoint_match_connector_count += 1;
            if detail.tight_dual_endpoint_match() {
                target.diagonal_tight_dual_endpoint_match_connector_count += 1;
            }
        }
    }
    if detail.tight_dual_endpoint_match() {
        target.tight_dual_endpoint_match_connector_count += 1;
        if metric.orientation != "diagonal" {
            accumulate_fdm_open_stroke_marker_style_profile(
                &mut target.tight_non_diagonal_dual_connector_marker_style_profile,
                connector.command,
            );
            accumulate_projected_bbox_union_milli(
                &mut target.tight_non_diagonal_dual_projected_bbox_x_min_milli,
                &mut target.tight_non_diagonal_dual_projected_bbox_y_min_milli,
                &mut target.tight_non_diagonal_dual_projected_bbox_x_max_milli,
                &mut target.tight_non_diagonal_dual_projected_bbox_y_max_milli,
                metric.projected_bbox,
            );
        }
    }
}

pub(crate) fn accumulate_fdm_open_stroke_axis_rule_source_order_gate(
    target: &mut FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    connector: FdmCommandDiagnostic<'_>,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
) {
    let Some(connector_parent_relative_offset) =
        fdm_command_normalized_parent_relative_offset(connector)
    else {
        target.dual_endpoint_connector_axis_rule_parent_span_unclassified_count += 1;
        return;
    };
    let (Some(axis_rule_min), Some(axis_rule_max)) = (
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    ) else {
        target.dual_endpoint_connector_axis_rule_parent_span_unclassified_count += 1;
        return;
    };

    target.dual_endpoint_axis_rule_source_order_backed_connector_count += 1;
    accumulate_usize_range(
        &mut target.dual_endpoint_connector_parent_relative_offset_min,
        &mut target.dual_endpoint_connector_parent_relative_offset_max,
        connector_parent_relative_offset,
    );
    accumulate_usize_range(
        &mut target.dual_endpoint_axis_rule_parent_relative_offset_min,
        &mut target.dual_endpoint_axis_rule_parent_relative_offset_max,
        axis_rule_min,
    );
    accumulate_usize_range(
        &mut target.dual_endpoint_axis_rule_parent_relative_offset_min,
        &mut target.dual_endpoint_axis_rule_parent_relative_offset_max,
        axis_rule_max,
    );

    if connector_parent_relative_offset < axis_rule_min {
        target.dual_endpoint_connector_before_axis_rule_parent_span_count += 1;
    } else if connector_parent_relative_offset > axis_rule_max {
        target.dual_endpoint_connector_after_axis_rule_parent_span_count += 1;
    } else {
        target.dual_endpoint_connector_between_axis_rule_parent_span_count += 1;
    }
}

pub(crate) fn accumulate_fdm_open_stroke_axis_rule_row_cohort_segment_gate(
    target: &mut FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    connector: FdmCommandDiagnostic<'_>,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    accumulate_fdm_open_stroke_axis_rule_row_cohort_bbox_relation_gate(
        target,
        connector,
        endpoint_summary,
    );

    if target.fdm_index_vector_offset.is_none() {
        target.fdm_index_vector_offset = Some(connector.entry.vector_offset());
    }
    if target.fdm_index_vector_len.is_none() {
        target.fdm_index_vector_len = Some(connector.entry.vector_len());
    }
    if target.fdm_index_vector_command_count.is_none() {
        let vector_command_count = connector.entry.vector_commands().len();
        let connector_candidate_count = connector.entry.connector_candidates().len();
        target.fdm_index_vector_command_count = Some(vector_command_count);
        target.fdm_index_connector_candidate_count = Some(connector_candidate_count);
        target.fdm_index_non_connector_command_count =
            Some(vector_command_count.saturating_sub(connector_candidate_count));
    }
    target.fdm_index_valid_vector_offset |= connector.entry.valid_vector_offset();
    target.fdm_index_image_signature_count = target
        .fdm_index_image_signature_count
        .max(connector.entry.image_signature_hits().len());
    target.fdm_index_segment_image_signature_count = target
        .fdm_index_segment_image_signature_count
        .max(connector.entry.segment_image_signature_hits().len());

    let source_segment = connector.command.source_segment();
    if let Some(source_segment) = source_segment {
        if target.fdm_index_source_segment_relative_offset.is_none() {
            target.fdm_index_source_segment_relative_offset =
                Some(source_segment.relative_offset());
        }
        if target.fdm_index_source_segment_command_count.is_none() {
            target.fdm_index_source_segment_command_count =
                Some(usize::from(source_segment.command_count()));
        }
        target.source_segment_backed_connector_count += 1;
        if source_segment.relative_offset() == connector.entry.vector_offset() {
            target.source_segment_matches_index_entry_connector_count += 1;
        }
    } else {
        target.source_segment_missing_connector_count += 1;
    }

    if endpoint_summary.dual_endpoint_match() {
        if let Some(source_segment) = source_segment {
            target.dual_endpoint_source_segment_backed_connector_count += 1;
            if source_segment.relative_offset() == connector.entry.vector_offset() {
                target.dual_endpoint_source_segment_matches_index_entry_connector_count += 1;
            }
        }
        if target.image_bearing_segment_candidate() {
            target.dual_endpoint_image_bearing_segment_connector_count += 1;
        }
    }
}

pub(crate) fn accumulate_fdm_open_stroke_axis_rule_row_cohort_bbox_relation_gate(
    target: &mut FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    connector: FdmCommandDiagnostic<'_>,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    let index_bbox = normalize_fdm_index_entry_bbox(connector.entry.bbox());
    if target.fdm_index_bbox_left.is_none() {
        target.fdm_index_bbox_left = Some(index_bbox.0);
        target.fdm_index_bbox_top = Some(index_bbox.1);
        target.fdm_index_bbox_right = Some(index_bbox.2);
        target.fdm_index_bbox_bottom = Some(index_bbox.3);
    }

    let Some(connector_bbox) =
        fdm_vector_command_source_bbox(connector.command).map(normalize_fdm_bbox)
    else {
        target.fdm_index_bbox_source_bbox_missing_connector_count += 1;
        if endpoint_summary.dual_endpoint_match() {
            target.dual_endpoint_fdm_index_bbox_source_bbox_missing_connector_count += 1;
        }
        return;
    };

    let relation = if fdm_bbox_contains(index_bbox, connector_bbox) {
        "contains"
    } else if fdm_bbox_intersects(index_bbox, connector_bbox) {
        "overlaps"
    } else {
        "disjoint"
    };

    match relation {
        "contains" => {
            target.fdm_index_bbox_contains_connector_count += 1;
            if endpoint_summary.dual_endpoint_match() {
                target.dual_endpoint_fdm_index_bbox_contains_connector_count += 1;
            }
        }
        "overlaps" => {
            target.fdm_index_bbox_overlaps_connector_count += 1;
            if endpoint_summary.dual_endpoint_match() {
                target.dual_endpoint_fdm_index_bbox_overlaps_connector_count += 1;
            }
        }
        _ => {
            target.fdm_index_bbox_disjoint_connector_count += 1;
            if endpoint_summary.dual_endpoint_match() {
                target.dual_endpoint_fdm_index_bbox_disjoint_connector_count += 1;
            }
        }
    }
}

pub(crate) fn accumulate_fdm_open_stroke_marker_style_profile_from_profile(
    target: &mut FdmOpenStrokeMarkerStyleProfile,
    source: FdmOpenStrokeMarkerStyleProfile,
) {
    target.command_count += source.command_count;
    target.line_marker_count += source.line_marker_count;
    target.path_marker_count += source.path_marker_count;
    target.bezier_marker_count += source.bezier_marker_count;
    target.ellipse_marker_count += source.ellipse_marker_count;
    target.other_marker_count += source.other_marker_count;
    target.style_0000_count += source.style_0000_count;
    target.style_0005_count += source.style_0005_count;
    target.style_0080_count += source.style_0080_count;
    target.style_00a0_count += source.style_00a0_count;
    target.other_style_count += source.other_style_count;
}

pub(crate) fn accumulate_fdm_open_stroke_marker_style_profile(
    target: &mut FdmOpenStrokeMarkerStyleProfile,
    command: &ObjectFdmVectorCommandCandidate,
) {
    target.command_count += 1;
    if fdm_vector_marker_is_line(command.marker()) {
        target.line_marker_count += 1;
    } else if FDM_VECTOR_COMMAND_PATH_MARKERS.contains(command.marker()) {
        target.path_marker_count += 1;
    } else if fdm_vector_marker_is_bezier_curve(command.marker()) {
        target.bezier_marker_count += 1;
    } else if FDM_VECTOR_COMMAND_ELLIPSE_MARKERS.contains(command.marker()) {
        target.ellipse_marker_count += 1;
    } else {
        target.other_marker_count += 1;
    }

    match command.style_word() {
        0x0000 => target.style_0000_count += 1,
        0x0005 => target.style_0005_count += 1,
        0x0080 => target.style_0080_count += 1,
        0x00a0 => target.style_00a0_count += 1,
        _ => target.other_style_count += 1,
    }
}

pub(crate) fn accumulate_fdm_open_stroke_axis_rule_owner_promotion_gate_summary(
    target: &mut FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary,
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
) {
    target.dual_endpoint_match_connector_count += 1;
    if owner_summary.dual_endpoint_owner_candidate() {
        target.dual_endpoint_owner_candidate_count += 1;
    }
    if owner_summary.nearest_fdm_owner_rows_match {
        target.nearest_fdm_owner_rows_match_count += 1;
    }
    if owner_summary.nearest_fdm_owner_row_matches_connector_row {
        target.nearest_fdm_owner_row_matches_connector_row_count += 1;
    }
    if owner_summary.mixed_top_level_vs_nested_order_namespace {
        target.mixed_top_level_vs_nested_order_namespace_count += 1;
    }
    if owner_summary.parent_normalized_ordered_same_row_same_connector() {
        target.parent_normalized_ordered_same_row_same_connector_count += 1;
    }

    match owner_summary.owner_parent_command_relation() {
        "missing-endpoint-owner-candidate" => {
            target.missing_endpoint_owner_candidate_count += 1;
        }
        "nearest-owner-row-mismatch" => {
            target.nearest_owner_row_mismatch_count += 1;
        }
        "nearest-owner-row-not-connector-row" => {
            target.nearest_owner_row_not_connector_row_count += 1;
        }
        "same-row-before-owner-parent-command-span" => {
            target.before_owner_parent_command_span_count += 1;
        }
        "same-row-between-owner-parent-command-span" => {
            target.between_owner_parent_command_span_count += 1;
        }
        "same-row-after-owner-parent-command-span" => {
            target.after_owner_parent_command_span_count += 1;
        }
        "same-row-parent-command-relation-unclassified" => {
            target.parent_command_relation_unclassified_count += 1;
        }
        _ => {}
    }
    match owner_summary.owner_parent_source_order_relation() {
        "same-row-before-owner-parent-relative-offset-span" => {
            target.before_owner_parent_relative_offset_span_count += 1;
        }
        "same-row-between-owner-parent-relative-offset-span" => {
            target.between_owner_parent_relative_offset_span_count += 1;
        }
        "same-row-after-owner-parent-relative-offset-span" => {
            target.after_owner_parent_relative_offset_span_count += 1;
        }
        "same-row-parent-relative-offset-relation-unclassified" => {
            target.parent_relative_offset_relation_unclassified_count += 1;
        }
        _ => {}
    }
}

pub(crate) fn accumulate_fdm_connector_owner_row_cohort_match_summary(
    target: &mut FdmConnectorOwnerRowCohortDiagnosticSummary,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    target.connector_candidate_count += 1;
    target.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
    target.tight_endpoint_match_count += endpoint_summary.tight_match_count;
    target.nearby_endpoint_match_count +=
        endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
    if endpoint_summary.total_match_count > 0 {
        target.matched_connector_count += 1;
    }
    if endpoint_summary.dual_endpoint_match() {
        target.dual_endpoint_match_connector_count += 1;
    }
}

pub(crate) fn accumulate_fdm_connector_owner_command_relation_match_summary(
    target: &mut FdmConnectorGraphDiagnosticSummary,
    relation: &'static str,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    let summary = match relation {
        "missing-endpoint-owner-candidate" => {
            &mut target.missing_endpoint_owner_relation_match_summary
        }
        "nearest-owner-row-mismatch" => {
            &mut target.nearest_owner_row_mismatch_relation_match_summary
        }
        "nearest-owner-row-not-connector-row" => {
            &mut target.nearest_owner_row_not_connector_row_relation_match_summary
        }
        "same-row-mixed-command-namespace" => {
            &mut target.same_row_mixed_command_namespace_match_summary
        }
        "same-row-before-owner-command-span" => {
            &mut target.same_row_before_owner_command_span_match_summary
        }
        "same-row-between-owner-command-span" => {
            &mut target.same_row_between_owner_command_span_match_summary
        }
        "same-row-after-owner-command-span" => {
            &mut target.same_row_after_owner_command_span_match_summary
        }
        _ => &mut target.same_row_owner_command_relation_unclassified_match_summary,
    };
    accumulate_fdm_connector_owner_row_cohort_match_summary(summary, endpoint_summary);
}

pub(crate) fn accumulate_fdm_connector_owner_source_order_relation_match_summary(
    target: &mut FdmConnectorGraphDiagnosticSummary,
    relation: &'static str,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    let summary = match relation {
        "missing-endpoint-owner-candidate" => {
            &mut target.missing_endpoint_owner_source_order_match_summary
        }
        "nearest-owner-row-mismatch" => {
            &mut target.nearest_owner_row_mismatch_source_order_match_summary
        }
        "nearest-owner-row-not-connector-row" => {
            &mut target.nearest_owner_row_not_connector_row_source_order_match_summary
        }
        "same-row-before-owner-relative-offset-span" => {
            &mut target.same_row_before_owner_relative_offset_span_match_summary
        }
        "same-row-between-owner-relative-offset-span" => {
            &mut target.same_row_between_owner_relative_offset_span_match_summary
        }
        "same-row-after-owner-relative-offset-span" => {
            &mut target.same_row_after_owner_relative_offset_span_match_summary
        }
        _ => &mut target.same_row_relative_offset_relation_unclassified_match_summary,
    };
    accumulate_fdm_connector_owner_row_cohort_match_summary(summary, endpoint_summary);
}

impl FdmConnectorGraphDiagnosticSummary {
    pub(crate) fn all_line_rule_match_summary(self) -> FdmConnectorRuleSetMatchDiagnosticSummary {
        FdmConnectorRuleSetMatchDiagnosticSummary {
            line_rule_projection_count: self.line_rule_projection_count,
            connector_candidate_count: self.connector_candidate_count,
            connector_endpoint_probe_count: self.connector_endpoint_probe_count,
            total_thresholded_endpoint_match_count: self.total_thresholded_endpoint_match_count,
            matched_connector_count: self.matched_connector_count,
            dual_endpoint_match_connector_count: self.dual_endpoint_match_connector_count,
            tight_endpoint_match_count: self.tight_endpoint_match_count,
            nearby_endpoint_match_count: self.nearby_endpoint_match_count,
            no_thresholded_line_rule_endpoint_match_connector_count: self
                .no_thresholded_line_rule_endpoint_match_connector_count,
            single_or_missing_endpoint_line_rule_match_connector_count: self
                .single_or_missing_endpoint_line_rule_match_connector_count,
            connector_ownership_and_paint_order_unproven_connector_count: self
                .connector_ownership_and_paint_order_unproven_connector_count,
        }
    }

    pub(crate) fn render_promotion_blocked_reason(self) -> &'static str {
        let axis_rule_owner_gate_summary =
            self.fdm_open_stroke_axis_rule_owner_promotion_gate_summary;
        let axis_rule_owner_gate_blocked_reason =
            axis_rule_owner_gate_summary.parent_normalized_order_gate_blocked_reason();
        if self.connector_candidate_count == 0 {
            "no-connector-candidates"
        } else if self.dual_endpoint_match_connector_count == 0
            && self
                .fdm_open_stroke_axis_rule_match_summary
                .dual_endpoint_match_connector_count
                > 0
            && axis_rule_owner_gate_summary.dual_endpoint_match_connector_count > 0
            && axis_rule_owner_gate_blocked_reason != "none"
        {
            axis_rule_owner_gate_blocked_reason
        } else if self.dual_endpoint_match_connector_count == 0
            && self
                .fdm_open_stroke_axis_rule_match_summary
                .dual_endpoint_match_connector_count
                > 0
            && self.parent_normalized_ordered_same_row_same_connector_count == 0
        {
            "same-row-axis-rule-parent-normalized-order-unproven"
        } else if self.dual_endpoint_match_connector_count == 0 {
            "no-dual-endpoint-line-rule-match"
        } else {
            "connector-ownership-grouping-and-paint-order-unproven"
        }
    }
}
