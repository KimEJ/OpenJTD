use super::*;
use crate::*;

pub(crate) fn fdm_open_stroke_axis_rules<'a>(
    layout: PageLayout,
    diagnostics: &'a [FdmCommandDiagnostic<'a>],
    extent: FdmCommandProjectionExtent,
    projection: &ShanaiLanLineRuleProjection,
) -> Vec<FdmOpenStrokeAxisRule<'a>> {
    let viewport = fdm_projection_viewport(layout);
    diagnostics
        .iter()
        .copied()
        .filter_map(|diagnostic| {
            if fdm_vector_primitive_is_closed(diagnostic.command)
                || fdm_connector_candidate_metric(layout, diagnostic, extent).is_some()
            {
                return None;
            }

            let (x, y, width, height) = fdm_path_unfiltered_bbox(layout, diagnostic, extent)?;
            if width.max(height) < FDM_OPEN_STROKE_AXIS_RULE_MIN_PROJECTED_SPAN_PX {
                return None;
            }

            if width >= height * 2.0 {
                let center_y = y + height * 0.5;
                Some(FdmOpenStrokeAxisRule {
                    diagnostic,
                    orientation: "horizontal",
                    line_offset_units: (x - viewport.x) / projection.grid_unit_px,
                    line_extent_units: (x + width - viewport.x) / projection.grid_unit_px,
                    group_index: ((center_y - viewport.y) / projection.line_height_px) - 1.0,
                    end_group_index: ((center_y - viewport.y) / projection.line_height_px) - 1.0,
                })
            } else if height >= width * 2.0 {
                let center_x = x + width * 0.5;
                Some(FdmOpenStrokeAxisRule {
                    diagnostic,
                    orientation: "vertical",
                    line_offset_units: (center_x - viewport.x) / projection.grid_unit_px,
                    line_extent_units: (center_x - viewport.x) / projection.grid_unit_px,
                    group_index: ((y - viewport.y) / projection.line_height_px) - 1.0,
                    end_group_index: ((y + height - viewport.y) / projection.line_height_px) - 1.0,
                })
            } else {
                None
            }
        })
        .filter(|rule| {
            rule.line_offset_units.is_finite()
                && rule.line_extent_units.is_finite()
                && rule.group_index.is_finite()
                && rule.end_group_index.is_finite()
        })
        .collect()
}

pub(crate) fn fdm_connector_open_stroke_axis_rule_endpoint_match_summary(
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    metric: FdmConnectorCandidateMetric,
    projection: &ShanaiLanLineRuleProjection,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) -> Option<FdmConnectorLineRuleEndpointMatchSummary> {
    fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
        layout, connector, metric, projection, rules,
    )
    .map(|detail| detail.summary)
}

pub(crate) fn fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    metric: FdmConnectorCandidateMetric,
    projection: &ShanaiLanLineRuleProjection,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) -> Option<FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail> {
    let viewport = fdm_projection_viewport(layout);
    let start =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)?;
    let end = fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)?;
    let start_matches =
        fdm_connector_open_stroke_axis_rule_endpoint_matches(connector, rules, start);
    let end_matches = fdm_connector_open_stroke_axis_rule_endpoint_matches(connector, rules, end);
    let mut axis_rule_endpoint_match_marker_style_profile =
        FdmOpenStrokeMarkerStyleProfile::default();
    let mut axis_rule_match_parent_relative_offset_min = None;
    let mut axis_rule_match_parent_relative_offset_max = None;
    for (_, rule, _, _) in start_matches.iter().chain(end_matches.iter()) {
        accumulate_fdm_open_stroke_marker_style_profile(
            &mut axis_rule_endpoint_match_marker_style_profile,
            rule.diagnostic.command,
        );
        if let Some(relative_offset) =
            fdm_command_normalized_parent_relative_offset(rule.diagnostic)
        {
            accumulate_usize_range(
                &mut axis_rule_match_parent_relative_offset_min,
                &mut axis_rule_match_parent_relative_offset_max,
                relative_offset,
            );
        }
    }
    let tight_match_count = start_matches
        .iter()
        .chain(end_matches.iter())
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    let start_tight_match_count = start_matches
        .iter()
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    let end_tight_match_count = end_matches
        .iter()
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    Some(FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail {
        summary: FdmConnectorLineRuleEndpointMatchSummary {
            start_match_count: start_matches.len(),
            end_match_count: end_matches.len(),
            total_match_count: start_matches.len() + end_matches.len(),
            tight_match_count,
        },
        start_tight_match_count,
        end_tight_match_count,
        axis_rule_endpoint_match_marker_style_profile,
        axis_rule_match_parent_relative_offset_min,
        axis_rule_match_parent_relative_offset_max,
    })
}

pub(crate) fn fdm_connector_open_stroke_axis_rule_endpoint_matches<'a>(
    connector: FdmCommandDiagnostic<'_>,
    rules: &'a [FdmOpenStrokeAxisRule<'a>],
    point: FdmConnectorTextGridPoint,
) -> Vec<(
    usize,
    &'a FdmOpenStrokeAxisRule<'a>,
    FdmConnectorLineRuleDistance,
    &'static str,
)> {
    rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.diagnostic.entry.row_index() == connector.entry.row_index())
        .filter_map(|(rule_index, rule)| {
            let distance = fdm_open_stroke_axis_rule_distance(point, rule);
            if distance.inline_delta > FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS {
                return None;
            }
            let tier =
                if distance.axis_delta <= FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS {
                    "tight"
                } else if distance.axis_delta
                    <= FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS
                {
                    "nearby"
                } else {
                    return None;
                };
            Some((rule_index, rule, distance, tier))
        })
        .collect()
}

pub(crate) fn fdm_open_stroke_axis_rule_distance(
    point: FdmConnectorTextGridPoint,
    rule: &FdmOpenStrokeAxisRule<'_>,
) -> FdmConnectorLineRuleDistance {
    let (axis_delta, inline_delta, closest_x_units, closest_group_index) = match rule.orientation {
        "horizontal" => {
            let start = rule.line_offset_units.min(rule.line_extent_units);
            let end = rule.line_offset_units.max(rule.line_extent_units);
            let closest_x = point.x_units.clamp(start, end);
            let inline_delta = if point.x_units < start {
                start - point.x_units
            } else if point.x_units > end {
                point.x_units - end
            } else {
                0.0
            };
            (
                (point.group_index_float - rule.group_index).abs(),
                inline_delta,
                closest_x,
                rule.group_index,
            )
        }
        "vertical" => {
            let start = rule.group_index.min(rule.end_group_index);
            let end = rule.group_index.max(rule.end_group_index);
            let closest_group = point.group_index_float.clamp(start, end);
            let inline_delta = if point.group_index_float < start {
                start - point.group_index_float
            } else if point.group_index_float > end {
                point.group_index_float - end
            } else {
                0.0
            };
            (
                (point.x_units - rule.line_offset_units).abs(),
                inline_delta,
                rule.line_offset_units,
                closest_group,
            )
        }
        _ => (
            (point.x_units - rule.line_offset_units).abs(),
            (point.group_index_float - rule.group_index).abs(),
            rule.line_offset_units,
            rule.group_index,
        ),
    };
    FdmConnectorLineRuleDistance {
        axis_delta,
        inline_delta,
        distance_grid: axis_delta.hypot(inline_delta),
        closest_x_units,
        closest_group_index,
    }
}

pub(crate) fn fdm_open_stroke_cohort_summary(
    layout: PageLayout,
    diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
) -> Option<FdmOpenStrokeCohortSummary> {
    let mut summary = FdmOpenStrokeCohortSummary {
        primitive_count: diagnostics.len(),
        ..Default::default()
    };
    let mut rows: BTreeMap<usize, FdmOpenStrokeRowCohortSummary> = BTreeMap::new();

    for diagnostic in diagnostics.iter().copied() {
        if fdm_vector_primitive_is_closed(diagnostic.command) {
            continue;
        }

        let row = rows.entry(diagnostic.entry.row_index()).or_insert_with(|| {
            FdmOpenStrokeRowCohortSummary {
                row_index: diagnostic.entry.row_index(),
                ..Default::default()
            }
        });
        let metric = fdm_connector_candidate_metric(layout, diagnostic, extent);
        let orientation = metric
            .map(|metric| metric.orientation)
            .unwrap_or_else(|| fdm_open_stroke_source_orientation(diagnostic.command));

        summary.open_stroke_count += 1;
        row.open_stroke_count += 1;
        match orientation {
            "horizontal" => {
                summary.horizontal_count += 1;
                row.horizontal_count += 1;
            }
            "vertical" => {
                summary.vertical_count += 1;
                row.vertical_count += 1;
            }
            _ => {
                summary.diagonal_count += 1;
                row.diagonal_count += 1;
            }
        }
        if fdm_vector_marker_is_line(diagnostic.command.marker()) {
            summary.line_marker_count += 1;
            row.line_marker_count += 1;
        } else {
            summary.non_line_marker_count += 1;
            row.non_line_marker_count += 1;
        }
        accumulate_fdm_open_stroke_marker_style_profile(
            &mut row.marker_style_profile,
            diagnostic.command,
        );
        if metric.is_some() {
            summary.connector_candidate_count += 1;
            row.connector_candidate_count += 1;
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
        if let Some(source_bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
            row.source_bbox_union =
                fdm_bbox_extent_union(row.source_bbox_union, normalize_fdm_bbox(source_bbox));
        }
        if let Some(projected_bbox) = fdm_path_unfiltered_bbox(layout, diagnostic, extent) {
            row.projected_bbox_union = bbox_tuple_union(row.projected_bbox_union, projected_bbox);
        }
    }

    if summary.open_stroke_count == 0 {
        return None;
    }

    let mut row_cohorts = rows.into_values().collect::<Vec<_>>();
    summary.row_count = row_cohorts.len();
    row_cohorts.sort_by(|left, right| {
        right
            .connector_candidate_count
            .cmp(&left.connector_candidate_count)
            .then(right.open_stroke_count.cmp(&left.open_stroke_count))
            .then(right.horizontal_count.cmp(&left.horizontal_count))
            .then(right.vertical_count.cmp(&left.vertical_count))
            .then_with(|| left.row_index.cmp(&right.row_index))
    });

    if let Some(row) = row_cohorts
        .iter()
        .find(|row| row.connector_candidate_count > 0)
    {
        summary.dominant_connector_row_index = Some(row.row_index);
        summary.dominant_connector_row_connector_candidate_count = row.connector_candidate_count;
        summary.dominant_connector_row_open_stroke_count = row.open_stroke_count;
        summary.dominant_connector_row_horizontal_count = row.horizontal_count;
        summary.dominant_connector_row_vertical_count = row.vertical_count;
    }

    row_cohorts.truncate(FDM_OPEN_STROKE_ROW_COHORT_LIMIT);
    summary.row_cohorts = row_cohorts;
    Some(summary)
}

pub(crate) fn fdm_open_stroke_source_orientation(
    command: &ObjectFdmVectorCommandCandidate,
) -> &'static str {
    let Some(start) = command.path_points().first() else {
        return "diagonal";
    };
    let Some(end) = command.path_points().last() else {
        return "diagonal";
    };
    let dx = end.x().saturating_sub(start.x()) as f32;
    let dy = end.y().saturating_sub(start.y()) as f32;
    fdm_connector_orientation(dx, dy)
}

pub(crate) fn push_fdm_connector_source_endpoints_json(
    output: &mut String,
    metric: FdmConnectorCandidateMetric,
) {
    output.push_str("{\"start\":{\"x\":");
    output.push_str(&metric.source_start.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&metric.source_start.y().to_string());
    output.push_str("},\"end\":{\"x\":");
    output.push_str(&metric.source_end.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&metric.source_end.y().to_string());
    output.push_str("}}");
}

pub(crate) fn push_fdm_connector_projected_endpoints_json(
    output: &mut String,
    metric: FdmConnectorCandidateMetric,
) {
    output.push_str(&format!(
        "{{\"start\":{{\"x\":{:.3},\"y\":{:.3}}},\"end\":{{\"x\":{:.3},\"y\":{:.3}}}}}",
        metric.projected_start.0,
        metric.projected_start.1,
        metric.projected_end.0,
        metric.projected_end.1
    ));
}

pub(crate) fn push_fdm_connector_projected_text_grid_json(
    output: &mut String,
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    if projection.grid_unit_px <= 0.0 || projection.line_height_px <= 0.0 {
        output.push_str("null");
        return;
    }

    let viewport = fdm_projection_viewport(layout);
    output.push_str("{\"basis\":\"documentTextLineHeaderGrid\",\"start\":");
    if let Some(point) =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
    {
        push_fdm_connector_text_grid_point_json(output, point);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"end\":");
    if let Some(point) =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
    {
        push_fdm_connector_text_grid_point_json(output, point);
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn fdm_connector_projected_text_grid_point(
    point: (f32, f32),
    projection: &ShanaiLanLineRuleProjection,
    viewport: FdmProjectionViewport,
) -> Option<FdmConnectorTextGridPoint> {
    if projection.grid_unit_px <= 0.0 || projection.line_height_px <= 0.0 {
        return None;
    }
    let x_units = (point.0 - viewport.x) / projection.grid_unit_px;
    let group_index_float = ((point.1 - viewport.y) / projection.line_height_px) - 1.0;
    Some(FdmConnectorTextGridPoint {
        x_units,
        group_index_float,
    })
}

pub(crate) fn push_fdm_connector_text_grid_point_json(
    output: &mut String,
    point: FdmConnectorTextGridPoint,
) {
    output.push_str(&format!(
        "{{\"xUnits\":{:.3},\"groupIndexFloat\":{:.3}}}",
        point.x_units, point.group_index_float
    ));
}

pub(crate) fn push_fdm_connector_line_rule_attachment_candidates_json(
    output: &mut String,
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let viewport = fdm_projection_viewport(layout);
    let Some(start) =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
    else {
        output.push_str("null");
        return;
    };
    let Some(end) =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"basis\":\"documentTextLineHeaderGrid\",\"attachmentProven\":false");
    output.push_str(",\"start\":");
    push_fdm_connector_line_rule_attachment_candidate_json(output, projection, start);
    output.push_str(",\"end\":");
    push_fdm_connector_line_rule_attachment_candidate_json(output, projection, end);
    output.push('}');
}

pub(crate) fn push_fdm_connector_line_rule_attachment_candidate_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
) {
    output.push_str("{\"point\":");
    push_fdm_connector_text_grid_point_json(output, point);
    output.push_str(",\"nearestLineRule\":");
    if let Some((rule_index, rule, distance)) =
        fdm_connector_nearest_line_rule_match(projection, point)
    {
        output.push_str("{\"ruleIndex\":");
        output.push_str(&rule_index.to_string());
        output.push_str(",\"distanceGrid\":");
        output.push_str(&format!("{:.3}", distance.distance_grid));
        output.push_str(",\"axisDelta\":");
        output.push_str(&format!("{:.3}", distance.axis_delta));
        output.push_str(",\"inlineDelta\":");
        output.push_str(&format!("{:.3}", distance.inline_delta));
        output.push_str(",\"closestPoint\":");
        push_fdm_connector_text_grid_point_json(
            output,
            FdmConnectorTextGridPoint {
                x_units: distance.closest_x_units,
                group_index_float: distance.closest_group_index,
            },
        );
        output.push_str(",\"orientation\":");
        output.push_str(&json_string(rule.orientation));
        output.push_str(",\"candidateSource\":");
        output.push_str(&json_string(rule.candidate_source));
        output.push_str(",\"groupIndex\":");
        output.push_str(&rule.group_index.to_string());
        output.push_str(",\"endGroupIndex\":");
        output.push_str(&rule.end_group_index.to_string());
        output.push_str(",\"lineOffsetUnits\":");
        output.push_str(&rule.line_offset_units.to_string());
        output.push_str(",\"lineExtentUnits\":");
        output.push_str(&rule.line_extent_units.to_string());
        output.push_str(",\"attachmentProven\":false}");
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn push_fdm_connector_line_rule_endpoint_matches_json(
    output: &mut String,
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let viewport = fdm_projection_viewport(layout);
    let Some(start) =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
    else {
        output.push_str("null");
        return;
    };
    let Some(end) =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"basis\":\"documentTextLineHeaderGrid\",\"attachmentProven\":false");
    output.push_str(",\"start\":");
    push_fdm_connector_line_rule_endpoint_match_array_json(output, projection, start);
    output.push_str(",\"end\":");
    push_fdm_connector_line_rule_endpoint_match_array_json(output, projection, end);
    output.push('}');
}

pub(crate) fn push_fdm_connector_line_rule_endpoint_match_summary_json(
    output: &mut String,
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let Some(summary) =
        fdm_connector_line_rule_endpoint_match_summary(layout, metric, line_rule_projection)
    else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"startMatchCount\":");
    output.push_str(&summary.start_match_count.to_string());
    output.push_str(",\"endMatchCount\":");
    output.push_str(&summary.end_match_count.to_string());
    output.push_str(",\"totalMatchCount\":");
    output.push_str(&summary.total_match_count.to_string());
    output.push_str(",\"tightMatchCount\":");
    output.push_str(&summary.tight_match_count.to_string());
    output.push_str(",\"matchedEndpointCount\":");
    output.push_str(&summary.matched_endpoint_count().to_string());
    output.push_str(",\"dualEndpointMatch\":");
    output.push_str(if summary.dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"graphPromotionBlockedReason\":");
    output.push_str(&json_string(summary.graph_promotion_blocked_reason()));
    output.push('}');
}

pub(crate) fn push_fdm_connector_open_stroke_axis_rule_endpoint_matches_json(
    output: &mut String,
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let viewport = fdm_projection_viewport(layout);
    let Some(start) =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
    else {
        output.push_str("null");
        return;
    };
    let Some(end) =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+documentTextLineHeaderGrid\"");
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\"");
    output.push_str(",\"rowScoped\":true,\"attachmentProven\":false");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"",
    );
    output.push_str(",\"start\":");
    push_fdm_connector_open_stroke_axis_rule_endpoint_match_array_json(
        output, layout, extent, connector, rules, start,
    );
    output.push_str(",\"end\":");
    push_fdm_connector_open_stroke_axis_rule_endpoint_match_array_json(
        output, layout, extent, connector, rules, end,
    );
    output.push('}');
}

pub(crate) fn push_fdm_connector_open_stroke_axis_rule_endpoint_match_summary_json(
    output: &mut String,
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let Some(detail) = fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
        layout, connector, metric, projection, rules,
    ) else {
        output.push_str("null");
        return;
    };
    let summary = detail.summary;
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule\"");
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\"");
    output.push_str(",\"startMatchCount\":");
    output.push_str(&summary.start_match_count.to_string());
    output.push_str(",\"endMatchCount\":");
    output.push_str(&summary.end_match_count.to_string());
    output.push_str(",\"totalMatchCount\":");
    output.push_str(&summary.total_match_count.to_string());
    output.push_str(",\"tightMatchCount\":");
    output.push_str(&summary.tight_match_count.to_string());
    output.push_str(",\"startTightMatchCount\":");
    output.push_str(&detail.start_tight_match_count.to_string());
    output.push_str(",\"endTightMatchCount\":");
    output.push_str(&detail.end_tight_match_count.to_string());
    output.push_str(",\"tightDualEndpointMatch\":");
    output.push_str(if detail.tight_dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedEndpointCount\":");
    output.push_str(&summary.matched_endpoint_count().to_string());
    output.push_str(",\"dualEndpointMatch\":");
    output.push_str(if summary.dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"graphPromotionBlockedReason\":");
    output.push_str(&json_string(summary.graph_promotion_blocked_reason()));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_fdm_connector_open_stroke_axis_rule_owner_promotion_gate_json(
    output: &mut String,
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let Some(axis_summary) = fdm_connector_open_stroke_axis_rule_endpoint_match_summary(
        layout, connector, metric, projection, rules,
    ) else {
        output.push_str("null");
        return;
    };
    let owner_summary = fdm_connector_endpoint_owner_match_summary(
        layout,
        connector,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );

    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+endpointOwnerMatchSummary\"");
    output.push_str(",\"decoded\":false,\"sourceBacked\":true,\"renderable\":false");
    output.push_str(",\"axisRuleDualEndpointMatch\":");
    output.push_str(if axis_summary.dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"axisRuleMatchedEndpointCount\":");
    output.push_str(&axis_summary.matched_endpoint_count().to_string());
    output.push_str(",\"axisRuleTotalMatchCount\":");
    output.push_str(&axis_summary.total_match_count.to_string());
    output.push_str(",\"axisRuleTightMatchCount\":");
    output.push_str(&axis_summary.tight_match_count.to_string());
    output.push_str(",\"parentNormalizedOrderGateBlockedReason\":");
    output.push_str(&json_string(
        FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
            dual_endpoint_match_connector_count: if axis_summary.dual_endpoint_match() {
                1
            } else {
                0
            },
            dual_endpoint_owner_candidate_count: if owner_summary.dual_endpoint_owner_candidate() {
                1
            } else {
                0
            },
            nearest_fdm_owner_rows_match_count: if owner_summary.nearest_fdm_owner_rows_match {
                1
            } else {
                0
            },
            nearest_fdm_owner_row_matches_connector_row_count: if owner_summary
                .nearest_fdm_owner_row_matches_connector_row
            {
                1
            } else {
                0
            },
            parent_normalized_ordered_same_row_same_connector_count: if owner_summary
                .parent_normalized_ordered_same_row_same_connector()
            {
                1
            } else {
                0
            },
            between_owner_parent_command_span_count: if owner_summary
                .connector_parent_command_between_nearest_fdm_owner_parent_commands
            {
                1
            } else {
                0
            },
            between_owner_parent_relative_offset_span_count: if owner_summary
                .connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets
            {
                1
            } else {
                0
            },
            ..Default::default()
        }
        .parent_normalized_order_gate_blocked_reason(),
    ));
    output.push_str(",\"dualEndpointOwnerCandidate\":");
    output.push_str(if owner_summary.dual_endpoint_owner_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nearestFdmOwnerRowsMatch\":");
    output.push_str(if owner_summary.nearest_fdm_owner_rows_match {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nearestFdmOwnerRowMatchesConnectorRow\":");
    output.push_str(
        if owner_summary.nearest_fdm_owner_row_matches_connector_row {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"mixedTopLevelVsNestedOrderNamespace\":");
    output.push_str(if owner_summary.mixed_top_level_vs_nested_order_namespace {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnector\":");
    output.push_str(
        if owner_summary.parent_normalized_ordered_same_row_same_connector() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"ownerCommandRelation\":");
    output.push_str(&json_string(owner_summary.owner_command_relation()));
    output.push_str(",\"ownerSourceOrderRelation\":");
    output.push_str(&json_string(owner_summary.owner_source_order_relation()));
    output.push_str(",\"ownerParentCommandRelation\":");
    output.push_str(&json_string(owner_summary.owner_parent_command_relation()));
    output.push_str(",\"ownerParentSourceOrderRelation\":");
    output.push_str(&json_string(
        owner_summary.owner_parent_source_order_relation(),
    ));
    output.push_str(",\"connectorParentCommandIndex\":");
    output.push_str(&owner_summary.connector_parent_command_index.to_string());
    output.push_str(",\"connectorParentRelativeOffset\":");
    push_option_usize_json(output, owner_summary.connector_parent_relative_offset);
    output.push_str(",\"startNearestFdmOwnerParentRelativeOffset\":");
    push_option_usize_json(
        output,
        owner_summary.start_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"endNearestFdmOwnerParentRelativeOffset\":");
    push_option_usize_json(
        output,
        owner_summary.end_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"startNearestFdmOwner\":");
    push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
        output,
        owner_summary.start_nearest_fdm_owner_row_index,
        owner_summary.start_nearest_fdm_owner_command_index,
        owner_summary.start_nearest_fdm_owner_parent_command_index,
        owner_summary.start_nearest_fdm_owner_synthetic_nested_command,
        owner_summary.start_nearest_fdm_owner_relative_offset,
    );
    output.push_str(",\"endNearestFdmOwner\":");
    push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
        output,
        owner_summary.end_nearest_fdm_owner_row_index,
        owner_summary.end_nearest_fdm_owner_command_index,
        owner_summary.end_nearest_fdm_owner_parent_command_index,
        owner_summary.end_nearest_fdm_owner_synthetic_nested_command,
        owner_summary.end_nearest_fdm_owner_relative_offset,
    );
    output.push_str(",\"connectorCommandBetweenNearestFdmOwnerCommands\":");
    output.push_str(
        if owner_summary.connector_command_between_nearest_fdm_owner_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandBeforeNearestFdmOwnerParentCommands\":");
    output.push_str(
        if owner_summary.connector_parent_command_before_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandBetweenNearestFdmOwnerParentCommands\":");
    output.push_str(
        if owner_summary.connector_parent_command_between_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandAfterNearestFdmOwnerParentCommands\":");
    output.push_str(
        if owner_summary.connector_parent_command_after_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorRelativeOffsetBetweenNearestFdmOwnerOffsets\":");
    output.push_str(
        if owner_summary.connector_relative_offset_between_nearest_fdm_owner_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetBeforeNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if owner_summary.connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetBetweenNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if owner_summary.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetAfterNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if owner_summary.connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"ownerGroupingProven\":false");
    output.push_str(",\"ownerGroupingPromotionBlockedReason\":");
    output.push_str(&json_string(
        owner_summary.owner_grouping_promotion_blocked_reason(),
    ));
    output.push_str(",\"ownershipPromotionBlockedReason\":");
    output.push_str(&json_string(
        owner_summary.ownership_promotion_blocked_reason(),
    ));
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"}",
    );
}

pub(crate) fn fdm_connector_line_rule_endpoint_match_summary(
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) -> Option<FdmConnectorLineRuleEndpointMatchSummary> {
    let projection = line_rule_projection?;
    let viewport = fdm_projection_viewport(layout);
    let start =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)?;
    let end = fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)?;
    let start_matches = fdm_connector_line_rule_endpoint_matches(projection, start);
    let end_matches = fdm_connector_line_rule_endpoint_matches(projection, end);
    let tight_match_count = start_matches
        .iter()
        .chain(end_matches.iter())
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    Some(FdmConnectorLineRuleEndpointMatchSummary {
        start_match_count: start_matches.len(),
        end_match_count: end_matches.len(),
        total_match_count: start_matches.len() + end_matches.len(),
        tight_match_count,
    })
}

pub(crate) fn fdm_connector_line_rule_endpoint_match_summary_for_candidate_source(
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    projection: &ShanaiLanLineRuleProjection,
    candidate_source: &'static str,
) -> Option<FdmConnectorLineRuleEndpointMatchSummary> {
    let viewport = fdm_projection_viewport(layout);
    let start =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)?;
    let end = fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)?;
    let start_matches = fdm_connector_line_rule_endpoint_matches_for_candidate_source(
        projection,
        start,
        candidate_source,
    );
    let end_matches = fdm_connector_line_rule_endpoint_matches_for_candidate_source(
        projection,
        end,
        candidate_source,
    );
    let tight_match_count = start_matches
        .iter()
        .chain(end_matches.iter())
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    Some(FdmConnectorLineRuleEndpointMatchSummary {
        start_match_count: start_matches.len(),
        end_match_count: end_matches.len(),
        total_match_count: start_matches.len() + end_matches.len(),
        tight_match_count,
    })
}

impl FdmConnectorLineRuleEndpointMatchSummary {
    pub(crate) fn matched_endpoint_count(self) -> usize {
        usize::from(self.start_match_count > 0) + usize::from(self.end_match_count > 0)
    }

    pub(crate) fn dual_endpoint_match(self) -> bool {
        self.start_match_count > 0 && self.end_match_count > 0
    }

    pub(crate) fn graph_promotion_blocked_reason(self) -> &'static str {
        if self.total_match_count == 0 {
            "no-thresholded-line-rule-endpoint-match"
        } else if !self.dual_endpoint_match() {
            "single-or-missing-endpoint-line-rule-match"
        } else {
            "connector-ownership-and-paint-order-unproven"
        }
    }
}

pub(crate) fn push_fdm_connector_line_rule_endpoint_match_array_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
) {
    let mut matches = fdm_connector_line_rule_endpoint_matches(projection, point);
    matches.sort_by(|left, right| {
        fdm_connector_line_rule_tier_rank(left.3)
            .cmp(&fdm_connector_line_rule_tier_rank(right.3))
            .then_with(|| {
                left.2
                    .axis_delta
                    .partial_cmp(&right.2.axis_delta)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| {
                left.2
                    .inline_delta
                    .partial_cmp(&right.2.inline_delta)
                    .unwrap_or(Ordering::Equal)
            })
    });

    output.push('[');
    for (match_index, (rule_index, rule, distance, tier)) in matches.iter().enumerate() {
        if match_index > 0 {
            output.push(',');
        }
        output.push_str("{\"ruleIndex\":");
        output.push_str(&rule_index.to_string());
        output.push_str(",\"tier\":");
        output.push_str(&json_string(tier));
        output.push_str(",\"perpendicularGroupDelta\":");
        output.push_str(&format!("{:.3}", distance.axis_delta));
        output.push_str(",\"spanOverflowUnits\":");
        output.push_str(&format!("{:.3}", distance.inline_delta));
        output.push_str(",\"inSpanAxis\":");
        output.push_str(if distance.inline_delta <= f32::EPSILON {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"closestPoint\":");
        push_fdm_connector_text_grid_point_json(
            output,
            FdmConnectorTextGridPoint {
                x_units: distance.closest_x_units,
                group_index_float: distance.closest_group_index,
            },
        );
        output.push_str(",\"orientation\":");
        output.push_str(&json_string(rule.orientation));
        output.push_str(",\"candidateSource\":");
        output.push_str(&json_string(rule.candidate_source));
        output.push_str(",\"attachmentProven\":false}");
    }
    output.push(']');
}

pub(crate) fn push_fdm_connector_open_stroke_axis_rule_endpoint_match_array_json(
    output: &mut String,
    layout: PageLayout,
    extent: FdmCommandProjectionExtent,
    connector: FdmCommandDiagnostic<'_>,
    rules: &[FdmOpenStrokeAxisRule<'_>],
    point: FdmConnectorTextGridPoint,
) {
    let mut matches = fdm_connector_open_stroke_axis_rule_endpoint_matches(connector, rules, point);
    matches.sort_by(|left, right| {
        fdm_connector_line_rule_tier_rank(left.3)
            .cmp(&fdm_connector_line_rule_tier_rank(right.3))
            .then_with(|| {
                left.2
                    .axis_delta
                    .partial_cmp(&right.2.axis_delta)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| {
                left.2
                    .inline_delta
                    .partial_cmp(&right.2.inline_delta)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.0.cmp(&right.0))
    });

    output.push('[');
    for (match_index, (rule_index, rule, distance, tier)) in matches.iter().enumerate() {
        if match_index > 0 {
            output.push(',');
        }
        output.push_str("{\"axisRuleIndex\":");
        output.push_str(&rule_index.to_string());
        output.push_str(",\"tier\":");
        output.push_str(&json_string(tier));
        output.push_str(",\"perpendicularGroupDelta\":");
        output.push_str(&format!("{:.3}", distance.axis_delta));
        output.push_str(",\"spanOverflowUnits\":");
        output.push_str(&format!("{:.3}", distance.inline_delta));
        output.push_str(",\"distanceGrid\":");
        output.push_str(&format!("{:.3}", distance.distance_grid));
        output.push_str(",\"inSpanAxis\":");
        output.push_str(if distance.inline_delta <= f32::EPSILON {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"closestPoint\":");
        push_fdm_connector_text_grid_point_json(
            output,
            FdmConnectorTextGridPoint {
                x_units: distance.closest_x_units,
                group_index_float: distance.closest_group_index,
            },
        );
        output.push_str(",\"orientation\":");
        output.push_str(&json_string(rule.orientation));
        output.push_str(",\"ruleRowIndex\":");
        output.push_str(&rule.diagnostic.entry.row_index().to_string());
        output.push_str(",\"ruleCommandIndex\":");
        output.push_str(&rule.diagnostic.command.command_index().to_string());
        output.push_str(",\"ruleRelativeOffset\":");
        output.push_str(&rule.diagnostic.command.relative_offset().to_string());
        push_fdm_vector_command_provenance_json(output, rule.diagnostic.command);
        output.push_str(",\"ruleMarkerHex\":");
        output.push_str(&json_string(&hex_bytes(rule.diagnostic.command.marker())));
        output.push_str(",\"ruleStyleWord\":");
        output.push_str(&rule.diagnostic.command.style_word().to_string());
        output.push_str(",\"ruleStyleWordHex\":");
        output.push_str(&json_string(&format!(
            "0x{:04x}",
            rule.diagnostic.command.style_word()
        )));
        output.push_str(",\"groupIndex\":");
        output.push_str(&format!("{:.3}", rule.group_index));
        output.push_str(",\"endGroupIndex\":");
        output.push_str(&format!("{:.3}", rule.end_group_index));
        output.push_str(",\"lineOffsetUnits\":");
        output.push_str(&format!("{:.3}", rule.line_offset_units));
        output.push_str(",\"lineExtentUnits\":");
        output.push_str(&format!("{:.3}", rule.line_extent_units));
        output.push_str(",\"projectedBbox\":");
        if let Some(bbox) = fdm_path_unfiltered_bbox(layout, rule.diagnostic, extent) {
            push_bbox_tuple_json(output, bbox);
        } else {
            output.push_str("null");
        }
        output.push_str(",\"sourcePathBbox\":");
        if let Some(bbox) = fdm_vector_command_source_bbox(rule.diagnostic.command) {
            push_object_fdm_index_bbox_json(output, bbox);
        } else {
            output.push_str("null");
        }
        output.push_str(",\"sameRowAsConnector\":true,\"attachmentProven\":false}");
    }
    output.push(']');
}

pub(crate) fn fdm_connector_line_rule_endpoint_matches(
    projection: &ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
) -> Vec<(
    usize,
    &ShanaiLanLineRule,
    FdmConnectorLineRuleDistance,
    &'static str,
)> {
    projection
        .rules
        .iter()
        .enumerate()
        .filter_map(|(rule_index, rule)| {
            let distance = fdm_connector_line_rule_distance(point, rule);
            if distance.inline_delta > FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS {
                return None;
            }
            let tier =
                if distance.axis_delta <= FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS {
                    "tight"
                } else if distance.axis_delta
                    <= FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS
                {
                    "nearby"
                } else {
                    return None;
                };
            Some((rule_index, rule, distance, tier))
        })
        .collect()
}

pub(crate) fn fdm_connector_line_rule_endpoint_matches_for_candidate_source<'a>(
    projection: &'a ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
    candidate_source: &'static str,
) -> Vec<(
    usize,
    &'a ShanaiLanLineRule,
    FdmConnectorLineRuleDistance,
    &'static str,
)> {
    projection
        .rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.candidate_source == candidate_source)
        .filter_map(|(rule_index, rule)| {
            let distance = fdm_connector_line_rule_distance(point, rule);
            if distance.inline_delta > FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS {
                return None;
            }
            let tier =
                if distance.axis_delta <= FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS {
                    "tight"
                } else if distance.axis_delta
                    <= FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS
                {
                    "nearby"
                } else {
                    return None;
                };
            Some((rule_index, rule, distance, tier))
        })
        .collect()
}

pub(crate) fn fdm_connector_line_rule_tier_rank(tier: &str) -> usize {
    match tier {
        "tight" => 0,
        "nearby" => 1,
        _ => 2,
    }
}

pub(crate) fn fdm_connector_nearest_line_rule_match(
    projection: &ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
) -> Option<(usize, &ShanaiLanLineRule, FdmConnectorLineRuleDistance)> {
    projection
        .rules
        .iter()
        .enumerate()
        .map(|(rule_index, rule)| {
            (
                rule_index,
                rule,
                fdm_connector_line_rule_distance(point, rule),
            )
        })
        .min_by(|left, right| {
            left.2
                .distance_grid
                .partial_cmp(&right.2.distance_grid)
                .unwrap_or(Ordering::Equal)
        })
}

pub(crate) fn fdm_connector_line_rule_distance(
    point: FdmConnectorTextGridPoint,
    rule: &ShanaiLanLineRule,
) -> FdmConnectorLineRuleDistance {
    let (axis_delta, inline_delta, closest_x_units, closest_group_index) = match rule.orientation {
        "horizontal" => {
            let line_group = rule.group_index as f32;
            let start = f32::from(rule.line_offset_units.min(rule.line_extent_units));
            let end = f32::from(rule.line_offset_units.max(rule.line_extent_units));
            let closest_x = point.x_units.clamp(start, end);
            let inline_delta = if point.x_units < start {
                start - point.x_units
            } else if point.x_units > end {
                point.x_units - end
            } else {
                0.0
            };
            (
                (point.group_index_float - line_group).abs(),
                inline_delta,
                closest_x,
                line_group,
            )
        }
        "vertical" => {
            let line_x = f32::from(rule.line_offset_units);
            let start = rule.group_index.min(rule.end_group_index) as f32;
            let end = rule.group_index.max(rule.end_group_index) as f32;
            let closest_group = point.group_index_float.clamp(start, end);
            let inline_delta = if point.group_index_float < start {
                start - point.group_index_float
            } else if point.group_index_float > end {
                point.group_index_float - end
            } else {
                0.0
            };
            (
                (point.x_units - line_x).abs(),
                inline_delta,
                line_x,
                closest_group,
            )
        }
        _ => (
            (point.x_units - f32::from(rule.line_offset_units)).abs(),
            (point.group_index_float - rule.group_index as f32).abs(),
            f32::from(rule.line_offset_units),
            rule.group_index as f32,
        ),
    };
    FdmConnectorLineRuleDistance {
        axis_delta,
        inline_delta,
        distance_grid: axis_delta.hypot(inline_delta),
        closest_x_units,
        closest_group_index,
    }
}

pub(crate) fn push_fdm_projection_viewport_json(output: &mut String, layout: PageLayout) {
    let viewport = fdm_projection_viewport(layout);
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        viewport.x, viewport.y, viewport.width, viewport.height
    ));
}
