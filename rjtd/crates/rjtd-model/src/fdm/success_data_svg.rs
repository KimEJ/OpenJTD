use super::*;
use crate::*;

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmSourceCohort {
    pub(crate) command_relative_offsets: Vec<usize>,
    pub(crate) source_vector_offset_start: Option<usize>,
    pub(crate) source_vector_offset_end: Option<usize>,
    pub(crate) source_vector_offset_count: usize,
    pub(crate) segment_backed_count: usize,
    pub(crate) raw_span_count: usize,
    pub(crate) segment_offsets: Vec<usize>,
}

impl SuccessDataTestFdmSourceCohort {
    pub(crate) fn blocked_reason(&self) -> &'static str {
        if self.raw_span_count > 0 && self.segment_backed_count > 0 {
            "mixed-raw-and-segment-cohorts"
        } else if self.segment_offsets.len() > 1 {
            "multiple-source-segment-cohorts"
        } else {
            "source-owner-candidate-unproven"
        }
    }
}

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmPrimitiveOwnershipClassification<'a> {
    pub(crate) command: &'a ObjectFdmVectorCommandCandidate,
    pub(crate) role_candidates: Vec<&'static str>,
    pub(crate) classification_basis: Vec<&'static str>,
    pub(crate) index_row_references: Vec<SuccessDataTestFdmIndexRowReference>,
}

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmIndexRowReference {
    pub(crate) row_index: usize,
    pub(crate) index_offset: usize,
    pub(crate) vector_offset: usize,
    pub(crate) valid_vector_offset: bool,
    pub(crate) offset_field: &'static str,
    pub(crate) offset_value: usize,
    pub(crate) match_kind: &'static str,
}

#[derive(Debug, Default)]
pub(crate) struct SuccessDataTestFdmIndexRowOrderPromotionGate {
    pub(crate) command_count: usize,
    pub(crate) referenced_command_relative_offsets: BTreeSet<usize>,
    pub(crate) referenced_row_indexes: BTreeSet<usize>,
    pub(crate) row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
    pub(crate) row_to_command_relative_offsets: BTreeMap<usize, BTreeSet<usize>>,
    pub(crate) reference_count: usize,
    pub(crate) valid_vector_offset_reference_count: usize,
    pub(crate) command_relative_offset_field_reference_count: usize,
    pub(crate) source_segment_relative_offset_field_reference_count: usize,
}

impl SuccessDataTestFdmIndexRowOrderPromotionGate {
    pub(crate) fn referenced_command_count(&self) -> usize {
        self.referenced_command_relative_offsets.len()
    }

    pub(crate) fn unreferenced_command_count(&self) -> usize {
        self.command_count
            .saturating_sub(self.referenced_command_count())
    }

    pub(crate) fn unique_row_index_count(&self) -> usize {
        self.referenced_row_indexes.len()
    }

    pub(crate) fn all_commands_referenced_by_index_rows_candidate(&self) -> bool {
        self.command_count > 0 && self.unreferenced_command_count() == 0
    }

    pub(crate) fn one_to_one_row_command_reference_candidate(&self) -> bool {
        self.reference_count == self.referenced_command_count()
            && self.reference_count == self.unique_row_index_count()
    }

    pub(crate) fn single_row_backs_multiple_commands_candidate(&self) -> bool {
        self.row_to_command_relative_offsets
            .values()
            .any(|offsets| offsets.len() > 1)
    }

    pub(crate) fn row_order_matches_command_order_candidate(&self) -> bool {
        success_data_test_fdm_row_command_pairs_are_monotonic(&self.row_command_pairs)
    }
}

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmOffsetFieldAuthorityGate {
    pub(crate) command_count: usize,
    pub(crate) reference_count: usize,
    pub(crate) valid_vector_offset_reference_count: usize,
    pub(crate) command_relative_offset_field_reference_count: usize,
    pub(crate) source_segment_relative_offset_field_reference_count: usize,
    pub(crate) unclassified_offset_field_reference_count: usize,
    pub(crate) raw_span_command_count: usize,
    pub(crate) segment_backed_command_count: usize,
    pub(crate) mixed_offset_field_namespaces: bool,
    pub(crate) mixed_command_provenance_cohorts: bool,
    pub(crate) all_references_use_command_relative_offset_field: bool,
    pub(crate) all_references_use_source_segment_relative_offset_field: bool,
    pub(crate) render_promotion_blocked_reason: &'static str,
}

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmRowFanoutSegmentOwnerGate {
    pub(crate) command_count: usize,
    pub(crate) reference_count: usize,
    pub(crate) unique_row_index_count: usize,
    pub(crate) command_relative_offset_field_reference_count: usize,
    pub(crate) source_segment_relative_offset_field_reference_count: usize,
    pub(crate) fanout_row_count: usize,
    pub(crate) fanout_reference_count: usize,
    pub(crate) fanout_command_relative_offset_field_reference_count: usize,
    pub(crate) fanout_source_segment_relative_offset_field_reference_count: usize,
    pub(crate) max_row_fanout: usize,
    pub(crate) multi_command_row_indexes: Vec<usize>,
    pub(crate) rows_with_multiple_command_refs: Vec<SuccessDataTestFdmRowFanoutSegmentOwnerRow>,
    pub(crate) one_to_one_row_command_reference_candidate: bool,
    pub(crate) single_row_backs_multiple_commands_candidate: bool,
    pub(crate) mixed_offset_field_namespaces: bool,
    pub(crate) mixed_command_provenance_cohorts: bool,
    pub(crate) fanout_rows_use_command_relative_offset_fields: bool,
    pub(crate) fanout_rows_use_source_segment_offset_fields: bool,
    pub(crate) raw_span_command_count: usize,
    pub(crate) segment_backed_command_count: usize,
    pub(crate) render_promotion_blocked_reason: &'static str,
}

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmRowFanoutSegmentOwnerRow {
    pub(crate) row_index: usize,
    pub(crate) command_reference_count: usize,
    pub(crate) command_relative_offsets: Vec<usize>,
    pub(crate) match_kinds: Vec<&'static str>,
}

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmPrimitiveOwnershipGate {
    pub(crate) row_command_gap_p95: Option<f32>,
    pub(crate) row_direction_mismatch: bool,
    pub(crate) multi_command_single_row: bool,
    pub(crate) all_commands_referenced_by_index_rows_candidate: bool,
    pub(crate) one_to_one_row_command_reference_candidate: bool,
    pub(crate) mixed_raw_and_segment_cohorts: bool,
    pub(crate) raw_span_command_count: usize,
    pub(crate) segment_backed_command_count: usize,
    pub(crate) ownership_proven: bool,
    pub(crate) render_ownership_blocked_reason: &'static str,
    pub(crate) render_ownership_blocked_reasons: Vec<&'static str>,
}

#[derive(Debug, Default)]
pub(crate) struct SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
    pub(crate) role_candidate: &'static str,
    pub(crate) reference_count: usize,
    pub(crate) valid_vector_offset_reference_count: usize,
    pub(crate) valid_command_relative_offset_field_reference_count: usize,
    pub(crate) valid_source_segment_relative_offset_field_reference_count: usize,
    pub(crate) command_relative_offset_field_reference_count: usize,
    pub(crate) source_segment_relative_offset_field_reference_count: usize,
    pub(crate) command_relative_offsets: BTreeSet<usize>,
    pub(crate) row_indexes: BTreeSet<usize>,
    pub(crate) row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
}

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmRolePaintOrderContinuityProfile {
    pub(crate) span_min: Option<usize>,
    pub(crate) span_max: Option<usize>,
    pub(crate) role_command_count: usize,
    pub(crate) command_count_in_span: usize,
    pub(crate) interleaved_non_role_command_count: usize,
    pub(crate) max_command_offset_gap: usize,
    pub(crate) continuity_score: f32,
}

impl SuccessDataTestFdmRolePaintOrderContinuityProfile {
    pub(crate) fn span_contiguous_candidate(&self) -> bool {
        self.role_command_count > 0
            && self.command_count_in_span == self.role_command_count
            && self.interleaved_non_role_command_count == 0
    }

    pub(crate) fn continuity_blocked(&self) -> bool {
        !self.span_contiguous_candidate()
    }

    pub(crate) fn paint_order_authority_pending(&self) -> bool {
        self.span_contiguous_candidate()
    }

    pub(crate) fn render_promotion_blocked_reason(&self) -> &'static str {
        if self.continuity_blocked() {
            "role-span-interleaved-non-role-commands"
        } else {
            "role-paint-order-authority-unproven"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SuccessDataTestFdmIndexRowCommandPair {
    pub(crate) row_index: usize,
    pub(crate) command_relative_offset: usize,
    pub(crate) match_kind: &'static str,
}

#[derive(Debug)]
pub(crate) struct SuccessDataTestFdmSubdiagram<'a> {
    pub(crate) index: usize,
    pub(crate) anchor_relative_offset: usize,
    pub(crate) center: ObjectFdmVectorPoint,
    pub(crate) anchor_radius: i32,
    pub(crate) commands: Vec<&'a ObjectFdmVectorCommandCandidate>,
}

pub(crate) fn fdm_point_distance(a: ObjectFdmVectorPoint, b: ObjectFdmVectorPoint) -> f32 {
    let dx = (a.x() - b.x()) as f32;
    let dy = (a.y() - b.y()) as f32;
    (dx * dx + dy * dy).sqrt()
}

pub(crate) fn fdm_point_distance_squared(a: ObjectFdmVectorPoint, b: ObjectFdmVectorPoint) -> i64 {
    let dx = i64::from(a.x() - b.x());
    let dy = i64::from(a.y() - b.y());
    dx * dx + dy * dy
}

pub(crate) fn fdm_point_segment_distance(
    point: ObjectFdmVectorPoint,
    start: ObjectFdmVectorPoint,
    end: ObjectFdmVectorPoint,
) -> f32 {
    let px = point.x() as f32;
    let py = point.y() as f32;
    let sx = start.x() as f32;
    let sy = start.y() as f32;
    let ex = end.x() as f32;
    let ey = end.y() as f32;
    let dx = ex - sx;
    let dy = ey - sy;
    let length_squared = dx * dx + dy * dy;
    if length_squared <= f32::EPSILON {
        return ((px - sx) * (px - sx) + (py - sy) * (py - sy)).sqrt();
    }
    let t = (((px - sx) * dx + (py - sy) * dy) / length_squared).clamp(0.0, 1.0);
    let x = sx + t * dx;
    let y = sy + t * dy;
    ((px - x) * (px - x) + (py - y) * (py - y)).sqrt()
}

pub(crate) fn push_fdm_frame_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let diagnostics = fdm_frame_diagnostics(document);
    if diagnostics.is_empty() {
        return;
    }

    svg.push_str("<g class=\"rjtd-fdm-frame-diagnostics\" data-source=\"fdmIndex+frame\" data-projection=\"fdmFrameDiagnosticProjection\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"false\">");
    for diagnostic in diagnostics {
        let Some((x, y, width, height)) = fdm_frame_diagnostic_bbox(layout, diagnostic) else {
            continue;
        };
        svg.push_str(&format!(
            "<g class=\"rjtd-fdm-frame-diagnostic\" data-source-path=\"{}\" data-object-candidate-index=\"{}\" data-row-index=\"{}\" data-frame-object-id=\"{}\" data-frame-type=\"0x{:04x}\" data-image-payload-extraction-status=\"{}\" data-render-promotion-blocked-reason=\"{}\" data-projection-kind=\"fdmFrameDiagnosticProjection\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"false\">",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.candidate_index,
            diagnostic.entry.row_index(),
            diagnostic.frame_record.object_id(),
            diagnostic.frame_record.object_type(),
            escape_xml(fdm_entry_image_payload_extraction_status(
                diagnostic.candidate,
                diagnostic.entry,
            )),
            escape_xml(fdm_entry_frame_render_blocked_reason(
                diagnostic.candidate,
                diagnostic.entry,
            ))
        ));
        svg.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"#eaf5ff\" fill-opacity=\"0.18\" stroke=\"#0a66b7\" stroke-width=\"1.2\" stroke-dasharray=\"5 3\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Yu Gothic, Meiryo, sans-serif\" font-size=\"9.0\" fill=\"#0a66b7\" letter-spacing=\"0\">FDM row {}</text>",
            x + 3.0,
            (y - 4.0).max(10.0),
            diagnostic.entry.row_index()
        ));
        svg.push_str("</g>");
    }
    svg.push_str("</g>");
}

pub(crate) fn push_fdm_command_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let diagnostics = fdm_command_diagnostics(document);
    let Some(extent) = fdm_command_projection_extent(&diagnostics) else {
        return;
    };

    svg.push_str("<g class=\"rjtd-fdm-command-diagnostics\" data-source=\"fdmVectorCommand\" data-projection=\"fdmCommandBBoxReferenceProjection\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"false\">");
    for diagnostic in diagnostics {
        let Some((x, y, width, height)) = fdm_command_diagnostic_bbox(layout, diagnostic, extent)
        else {
            continue;
        };
        let style = fdm_command_diagnostic_svg_style(diagnostic);
        svg.push_str(&format!(
            "<rect class=\"rjtd-fdm-command-diagnostic\" data-source-path=\"{}\" data-row-index=\"{}\" data-command-index=\"{}\" data-marker-hex=\"{}\" data-diagnostic-style-basis=\"{}\" data-image-signature-count=\"{}\" data-segment-image-signature-count=\"{}\" data-valid-vector-offset=\"{}\" data-projection-kind=\"fdmCommandBBoxReferenceProjection\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"false\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.65\" stroke-opacity=\"{}\"/>",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.entry.row_index(),
            diagnostic.command.command_index(),
            hex_bytes(diagnostic.command.marker()),
            style.basis,
            diagnostic.entry.image_signature_hits().len(),
            diagnostic.entry.segment_image_signature_hits().len(),
            diagnostic.entry.valid_vector_offset(),
            style.stroke,
            style.opacity
        ));
    }
    svg.push_str("</g>");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FdmCommandDiagnosticSvgStyle {
    pub(crate) stroke: &'static str,
    pub(crate) opacity: &'static str,
    pub(crate) basis: &'static str,
}

pub(crate) fn fdm_command_diagnostic_svg_style(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> FdmCommandDiagnosticSvgStyle {
    if !diagnostic.entry.segment_image_signature_hits().is_empty() {
        FdmCommandDiagnosticSvgStyle {
            stroke: "#d9432f",
            opacity: "0.82",
            basis: "fdm-index-segment-image-signature",
        }
    } else if !diagnostic.entry.image_signature_hits().is_empty() {
        FdmCommandDiagnosticSvgStyle {
            stroke: "#d9432f",
            opacity: "0.82",
            basis: "fdm-index-image-signature",
        }
    } else {
        FdmCommandDiagnosticSvgStyle {
            stroke: "#4d95ff",
            opacity: "0.44",
            basis: "fdm-index-command-diagnostic-default",
        }
    }
}

pub(crate) fn push_fdm_vector_primitive_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) -> bool {
    if page_number != 1 {
        return false;
    }

    let command_diagnostics = fdm_command_diagnostics(document);
    let Some(extent) = fdm_command_projection_extent(&command_diagnostics) else {
        return false;
    };
    let diagnostics = fdm_vector_primitive_diagnostics(document);
    if diagnostics.is_empty() {
        return false;
    }

    let group_start = svg.len();
    let mut rendered = false;
    let mut counter_overlays = String::new();
    svg.push_str("<g class=\"rjtd-fdm-vector-primitives\" data-source=\"fdmVectorCommandPrimitive\" data-projection=\"fdmVectorPrimitiveReferenceProjection\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\">");
    for diagnostic in diagnostics.iter().copied() {
        let Some((path_x, path_y, path_width, path_height)) =
            fdm_path_diagnostic_bbox(layout, diagnostic, extent)
        else {
            continue;
        };

        let path_closed = fdm_vector_primitive_is_closed(diagnostic.command);
        let fill = fdm_vector_render_fill_color(diagnostic, &diagnostics);
        let gradient = fdm_vector_linear_gradient_colors(diagnostic.command);
        let stroke = fdm_vector_render_stroke_color(diagnostic, &diagnostics);
        let data_fill = diagnostic
            .command
            .fill_color()
            .and_then(fdm_vector_css_color)
            .unwrap_or_else(|| "none".to_string());
        let data_stroke = diagnostic
            .command
            .stroke_color()
            .and_then(fdm_vector_css_color)
            .unwrap_or_else(|| "none".to_string());
        let stroke_width = fdm_vector_stroke_width(diagnostic.command);
        let primitive_kind = fdm_vector_primitive_kind(diagnostic.command);
        let paint_coverage = fdm_vector_paint_coverage(
            layout,
            diagnostic,
            &diagnostics,
            (path_x, path_y, path_width, path_height),
        );
        let page_coverage_ratio = paint_coverage.page_coverage_ratio;
        let viewport_coverage_ratio = paint_coverage.viewport_coverage_ratio;

        if let Some(ellipse) = diagnostic.command.ellipse() {
            let Some((cx, cy, rx, ry)) = fdm_projected_ellipse(layout, extent, ellipse) else {
                continue;
            };
            let ellipse_color = ellipse
                .color()
                .and_then(fdm_vector_primitive_css_color)
                .unwrap_or_else(|| "#111111".to_string());
            let fill = if fdm_vector_ellipse_should_fill(ellipse) {
                ellipse_color.as_str()
            } else {
                "none"
            };
            let stroke = if fdm_vector_ellipse_should_fill(ellipse) {
                "none"
            } else {
                ellipse_color.as_str()
            };
            svg.push_str(&format!(
                "<ellipse class=\"rjtd-fdm-vector-primitive\" data-source-path=\"{}\" data-row-index=\"{}\" data-command-index=\"{}\" data-marker-hex=\"{}\" data-primitive-kind=\"{}\" data-style-word=\"0x{:04x}\" data-fill-color=\"{}\" data-stroke-color=\"{}\" data-stroke-width=\"{:.3}\" data-path-closed=\"{}\" data-point-count=\"{}\" data-page-coverage-ratio=\"{page_coverage_ratio:.6}\" data-viewport-coverage-ratio=\"{viewport_coverage_ratio:.6}\" data-page-fill-candidate=\"{}\" data-page-fill-candidate-basis=\"{}\" data-page-fill-candidate-reason=\"{}\" data-page-fill-render-promotion-blocked-reason=\"{}\" data-projection-kind=\"fdmVectorPrimitiveReferenceProjection\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" rx=\"{rx:.1}\" ry=\"{ry:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.3}\" stroke-opacity=\"0.92\"/>",
                escape_xml(diagnostic.candidate.path()),
                diagnostic.entry.row_index(),
                diagnostic.command.command_index(),
                hex_bytes(diagnostic.command.marker()),
                primitive_kind,
                diagnostic.command.style_word(),
                data_fill,
                data_stroke,
                stroke_width,
                path_closed,
                diagnostic.command.path_points().len(),
                paint_coverage.page_fill_candidate,
                paint_coverage.page_fill_candidate_basis,
                paint_coverage.page_fill_candidate_reason,
                paint_coverage.render_promotion_blocked_reason,
                fill,
                stroke,
                stroke_width
            ));
            rendered = true;
            continue;
        }

        let Some(path_data) = fdm_projected_path_data(layout, extent, diagnostic.command) else {
            continue;
        };
        let fill_paint = if let Some((gradient_from, gradient_to)) = gradient.as_ref() {
            let gradient_id = format!(
                "rjtd-fdm-gradient-{}-{}",
                diagnostic.entry.row_index(),
                diagnostic.command.command_index()
            );
            svg.push_str(&format!(
                "<defs><linearGradient id=\"{gradient_id}\" gradientUnits=\"userSpaceOnUse\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"><stop offset=\"0%\" stop-color=\"{gradient_from}\"/><stop offset=\"100%\" stop-color=\"{gradient_to}\"/></linearGradient></defs>",
                path_x,
                path_y + path_height,
                path_x + path_width,
                path_y
            ));
            format!("url(#{gradient_id})")
        } else {
            fill.clone()
        };
        svg.push_str(&format!(
            "<path class=\"rjtd-fdm-vector-primitive\" data-source-path=\"{}\" data-row-index=\"{}\" data-command-index=\"{}\" data-marker-hex=\"{}\" data-primitive-kind=\"{}\" data-style-word=\"0x{:04x}\" data-fill-color=\"{}\" data-render-fill-kind=\"{}\" data-render-fill-color=\"{}\" data-stroke-color=\"{}\" data-render-stroke-color=\"{}\" data-stroke-width=\"{:.3}\" data-path-closed=\"{}\" data-point-count=\"{}\" data-page-coverage-ratio=\"{page_coverage_ratio:.6}\" data-viewport-coverage-ratio=\"{viewport_coverage_ratio:.6}\" data-page-fill-candidate=\"{}\" data-page-fill-candidate-basis=\"{}\" data-page-fill-candidate-reason=\"{}\" data-page-fill-render-promotion-blocked-reason=\"{}\" data-projection-kind=\"fdmVectorPrimitiveReferenceProjection\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\" d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.3}\" stroke-opacity=\"0.92\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.entry.row_index(),
            diagnostic.command.command_index(),
            hex_bytes(diagnostic.command.marker()),
            primitive_kind,
            diagnostic.command.style_word(),
            data_fill,
            if gradient.is_some() {
                "linearGradient"
            } else if fill == "none" {
                "none"
            } else {
                "solid"
            },
            fill,
            data_stroke,
            stroke,
            stroke_width,
            path_closed,
            diagnostic.command.path_points().len(),
            paint_coverage.page_fill_candidate,
            paint_coverage.page_fill_candidate_basis,
            paint_coverage.page_fill_candidate_reason,
            paint_coverage.render_promotion_blocked_reason,
            path_data,
            fill_paint,
            stroke,
            stroke_width
        ));
        if fdm_vector_filled_path_is_counter_overlay(diagnostic, &diagnostics) {
            counter_overlays.push_str(&format!(
                "<path class=\"rjtd-fdm-vector-counter-overlay\" data-source-path=\"{}\" data-row-index=\"{}\" data-command-index=\"{}\" data-marker-hex=\"{}\" data-render-counter-overlay=\"true\" data-render-fill-color=\"{}\" data-projection-kind=\"fdmVectorPrimitiveReferenceProjection\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\" d=\"{}\" fill=\"{}\" stroke=\"none\"/>",
                escape_xml(diagnostic.candidate.path()),
                diagnostic.entry.row_index(),
                diagnostic.command.command_index(),
                hex_bytes(diagnostic.command.marker()),
                fill,
                path_data,
                fill
            ));
        }
        rendered = true;
    }
    svg.push_str(&counter_overlays);
    svg.push_str("</g>");
    if !rendered {
        svg.truncate(group_start);
    }
    rendered
}

pub(crate) fn fdm_frame_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: FdmFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    let scale_x = layout.width_px() / SHANAI_LAN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let x = diagnostic.frame_record.x() as f32 / SHANAI_LAN_FDM_FRAME_X_DIVISOR * scale_x;
    let y = diagnostic.frame_record.y() as f32 / SHANAI_LAN_FDM_FRAME_Y_DIVISOR * scale_y;
    let width =
        diagnostic.frame_record.width() as f32 / SHANAI_LAN_FDM_FRAME_SIZE_DIVISOR * scale_x;
    let height =
        diagnostic.frame_record.height() as f32 / SHANAI_LAN_FDM_FRAME_SIZE_DIVISOR * scale_y;

    if x >= layout.width_px() || y >= layout.height_px() || width <= 0.0 || height <= 0.0 {
        return None;
    }
    Some((
        x,
        y,
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

pub(crate) fn fdm_command_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<(f32, f32, f32, f32)> {
    let bbox = normalize_fdm_bbox(diagnostic.command.bbox()?);
    let span_x = (extent.right - extent.left) as f32;
    let span_y = (extent.bottom - extent.top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let viewport = fdm_projection_viewport(layout);
    let x = viewport.x + (bbox.0 - extent.left) as f32 / span_x * viewport.width;
    let y = viewport.y + (bbox.1 - extent.top) as f32 / span_y * viewport.height;
    let width = (bbox.2 - bbox.0).max(1) as f32 / span_x * viewport.width;
    let height = (bbox.3 - bbox.1).max(1) as f32 / span_y * viewport.height;
    if x >= layout.width_px() || y >= layout.height_px() || width <= 0.0 || height <= 0.0 {
        return None;
    }
    Some((
        x,
        y,
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

pub(crate) fn fdm_path_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<(f32, f32, f32, f32)> {
    let bbox = fdm_path_unfiltered_bbox(layout, diagnostic, extent)?;
    if fdm_path_span_filter_blocks(layout, diagnostic.command, bbox) {
        return None;
    }
    Some(bbox)
}

pub(crate) fn fdm_path_span_filter_blocked(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<(f32, f32, f32, f32)> {
    let bbox = fdm_path_unfiltered_bbox(layout, diagnostic, extent)?;
    if fdm_path_span_filter_blocks(layout, diagnostic.command, bbox) {
        Some(bbox)
    } else {
        None
    }
}

pub(crate) fn fdm_path_span_filter_blocks(
    layout: PageLayout,
    command: &ObjectFdmVectorCommandCandidate,
    bbox: (f32, f32, f32, f32),
) -> bool {
    fdm_vector_path_span_filter_applies(command)
        && (bbox.2 / layout.width_px() > FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO
            || bbox.3 / layout.height_px() > FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO)
}

pub(crate) fn fdm_path_unfiltered_bbox(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<(f32, f32, f32, f32)> {
    let source_bbox = fdm_vector_command_source_bbox(diagnostic.command)?;
    let bbox = normalize_fdm_bbox(source_bbox);
    let (x1, y1) = fdm_project_source_point(layout, extent, bbox.0, bbox.1)?;
    let (x2, y2) = fdm_project_source_point(layout, extent, bbox.2, bbox.3)?;
    let width = (x2 - x1).abs().max(0.5);
    let height = (y2 - y1).abs().max(0.5);
    Some((x1.min(x2), y1.min(y2), width, height))
}

pub(crate) fn fdm_connector_candidate_metric(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<FdmConnectorCandidateMetric> {
    if diagnostic.command.ellipse().is_some()
        || fdm_vector_path_is_closed(diagnostic.command.path_points())
    {
        return None;
    }
    let source_start = *diagnostic.command.path_points().first()?;
    let source_end = *diagnostic.command.path_points().last()?;
    let projected_start =
        fdm_project_source_point(layout, extent, source_start.x(), source_start.y())?;
    let projected_end = fdm_project_source_point(layout, extent, source_end.x(), source_end.y())?;
    let projected_bbox = fdm_path_diagnostic_bbox(layout, diagnostic, extent)?;
    let source_dx = source_end.x().saturating_sub(source_start.x()) as f32;
    let source_dy = source_end.y().saturating_sub(source_start.y()) as f32;
    let projected_dx = projected_end.0 - projected_start.0;
    let projected_dy = projected_end.1 - projected_start.1;
    let source_endpoint_distance = source_dx.hypot(source_dy);
    let projected_endpoint_distance = projected_dx.hypot(projected_dy);
    let projected_span =
        projected_endpoint_distance.max(projected_bbox.2.abs().max(projected_bbox.3.abs()));
    if projected_span < FDM_CONNECTOR_CANDIDATE_MIN_PROJECTED_SPAN_PX {
        return None;
    }

    Some(FdmConnectorCandidateMetric {
        source_start,
        source_end,
        projected_start,
        projected_end,
        projected_bbox,
        source_endpoint_distance,
        projected_endpoint_distance,
        projected_span,
        orientation: fdm_connector_orientation(projected_dx, projected_dy),
        basis: if projected_endpoint_distance >= FDM_CONNECTOR_CANDIDATE_MIN_PROJECTED_SPAN_PX {
            "long-open-endpoint-path"
        } else {
            "long-open-bbox-path"
        },
    })
}

pub(crate) fn fdm_connector_orientation(dx: f32, dy: f32) -> &'static str {
    let abs_x = dx.abs();
    let abs_y = dy.abs();
    if abs_x >= abs_y * 2.0 {
        "horizontal"
    } else if abs_y >= abs_x * 2.0 {
        "vertical"
    } else {
        "diagonal"
    }
}

pub(crate) fn fdm_vector_path_span_filter_applies(
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    !fdm_vector_marker_is_line(command.marker()) || fdm_vector_path_is_closed(command.path_points())
}

pub(crate) fn fdm_projected_path_data(
    layout: PageLayout,
    extent: FdmCommandProjectionExtent,
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<String> {
    let mut points = Vec::with_capacity(command.path_points().len());
    for point in command.path_points() {
        points.push(fdm_project_source_point(
            layout,
            extent,
            point.x(),
            point.y(),
        )?);
    }
    if points.len() < 2 {
        return None;
    }

    let mut path_data = format!("M {:.1} {:.1}", points[0].0, points[0].1);
    if command.curve_segments().len() == points.len().saturating_sub(1) {
        for (index, segment) in command.curve_segments().iter().enumerate() {
            let control_1 = segment.control_1();
            let control_2 = segment.control_2();
            let end = command.path_points()[index + 1];
            let (control_1_x, control_1_y) =
                fdm_project_source_point(layout, extent, control_1.x(), control_1.y())?;
            let (control_2_x, control_2_y) =
                fdm_project_source_point(layout, extent, control_2.x(), control_2.y())?;
            let (end_x, end_y) = fdm_project_source_point(layout, extent, end.x(), end.y())?;
            path_data.push_str(&format!(
                " C {control_1_x:.1} {control_1_y:.1} {control_2_x:.1} {control_2_y:.1} {end_x:.1} {end_y:.1}"
            ));
        }
    } else if fdm_vector_marker_is_bezier_curve(command.marker()) && points.len() >= 3 {
        let mut index = 1usize;
        while index + 1 < points.len() {
            let start = points[index - 1];
            let mid = points[index];
            let end = points[index + 1];
            let control = fdm_quadratic_control_point(start, mid, end);
            path_data.push_str(&format!(
                " Q {:.1} {:.1} {:.1} {:.1}",
                control.0, control.1, end.0, end.1
            ));
            index += 2;
        }
        while index < points.len() {
            let point = points[index];
            path_data.push_str(&format!(" L {:.1} {:.1}", point.0, point.1));
            index += 1;
        }
    } else {
        for point in points.iter().skip(1) {
            path_data.push_str(&format!(" L {:.1} {:.1}", point.0, point.1));
        }
    }

    if fdm_vector_path_is_closed(command.path_points()) {
        path_data.push_str(" Z");
    }
    Some(path_data)
}

pub(crate) fn fdm_quadratic_control_point(
    start: (f32, f32),
    mid: (f32, f32),
    end: (f32, f32),
) -> (f32, f32) {
    (
        2.0 * mid.0 - (start.0 + end.0) * 0.5,
        2.0 * mid.1 - (start.1 + end.1) * 0.5,
    )
}

pub(crate) fn fdm_projected_ellipse(
    layout: PageLayout,
    extent: FdmCommandProjectionExtent,
    ellipse: ObjectFdmVectorEllipse,
) -> Option<(f32, f32, f32, f32)> {
    let center = ellipse.center();
    let (cx, cy) = fdm_project_source_point(layout, extent, center.x(), center.y())?;
    let span_x = (extent.right - extent.left) as f32;
    let span_y = (extent.bottom - extent.top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let viewport = fdm_projection_viewport(layout);
    let rx = ellipse.radius_x() as f32 / span_x * viewport.width;
    let ry = ellipse.radius_y() as f32 / span_y * viewport.height;
    if rx <= 0.0 || ry <= 0.0 {
        return None;
    }
    Some((cx, cy, rx, ry))
}

pub(crate) fn fdm_vector_ellipse_should_fill(ellipse: ObjectFdmVectorEllipse) -> bool {
    ellipse.radius_x().max(ellipse.radius_y()) <= 80
}

pub(crate) fn fdm_project_source_point(
    layout: PageLayout,
    extent: FdmCommandProjectionExtent,
    x: i32,
    y: i32,
) -> Option<(f32, f32)> {
    let span_x = (extent.right - extent.left) as f32;
    let span_y = (extent.bottom - extent.top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let viewport = fdm_projection_viewport(layout);
    Some((
        viewport.x + (x - extent.left) as f32 / span_x * viewport.width,
        viewport.y + (y - extent.top) as f32 / span_y * viewport.height,
    ))
}

pub(crate) fn fdm_projection_viewport(layout: PageLayout) -> FdmProjectionViewport {
    let scale_x = layout.width_px() / SHANAI_LAN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    FdmProjectionViewport {
        x: SHANAI_LAN_REFERENCE_CONTENT_LEFT_PX * scale_x,
        y: SHANAI_LAN_REFERENCE_CONTENT_TOP_PX * scale_y,
        width: SHANAI_LAN_REFERENCE_CONTENT_WIDTH_PX * scale_x,
        height: SHANAI_LAN_REFERENCE_CONTENT_HEIGHT_PX * scale_y,
    }
}

pub(crate) struct SuccessDataTestAnswerSheetFdmTextSlot {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font_size: f32,
    pub(crate) text_offset: usize,
    pub(crate) marker_offset: usize,
    pub(crate) index_offset: usize,
    pub(crate) source_bbox: ObjectFdmIndexBbox,
    pub(crate) text_bbox: ObjectFdmIndexBbox,
}

pub(crate) struct SuccessDataTestAnswerSheetIndexedFdmLabel<'a> {
    pub(crate) text: &'a ObjectFdmTextCandidate,
    pub(crate) index: &'a ObjectFdmTextIndexEntryCandidate,
    pub(crate) text_bbox: ObjectFdmIndexBbox,
}
