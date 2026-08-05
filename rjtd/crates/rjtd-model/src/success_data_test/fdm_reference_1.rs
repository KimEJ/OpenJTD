use super::*;
use crate::*;

pub(crate) const SUCCESS_DATA_TEST_CONE_TARGET_X_PX: f32 = 446.0;

pub(crate) const SUCCESS_DATA_TEST_CONE_TARGET_Y_PX: f32 = 489.0;

pub(crate) const SUCCESS_DATA_TEST_CONE_TARGET_WIDTH_PX: f32 = 128.0;

pub(crate) const SUCCESS_DATA_TEST_CONE_TARGET_HEIGHT_PX: f32 = 148.0;

pub(crate) const SUCCESS_DATA_TEST_Q4_SOURCE_LEFT: i32 = -15784;

pub(crate) const SUCCESS_DATA_TEST_Q4_SOURCE_TOP: i32 = -10213;

pub(crate) const SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT: i32 = -10584;

pub(crate) const SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM: i32 = -9013;

pub(crate) const SUCCESS_DATA_TEST_Q4_TARGET_X_PX: f32 = 93.3;

pub(crate) const SUCCESS_DATA_TEST_Q4_TARGET_Y_PX: f32 = 663.3;

pub(crate) const SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX: f32 = 491.4;

pub(crate) const SUCCESS_DATA_TEST_Q4_TEXT_HEIGHT_FACTOR: f32 = 0.67;

pub(crate) const SUCCESS_DATA_TEST_Q4_TEXT_BASELINE_FACTOR: f32 = 0.12;

pub(crate) const SUCCESS_DATA_TEST_Q5_TARGET_X_PX: f32 = 490.7;

pub(crate) const SUCCESS_DATA_TEST_Q5_TARGET_Y_PX: f32 = 795.0;

pub(crate) const SUCCESS_DATA_TEST_Q5_TARGET_WIDTH_PX: f32 = 74.6;

pub(crate) const SUCCESS_DATA_TEST_Q5_TARGET_HEIGHT_PX: f32 = 110.0;

pub(crate) const SUCCESS_DATA_TEST_CONE_MIN_TEXT_CORROBORATION_COUNT: usize = 2;

pub(crate) const SUCCESS_DATA_TEST_FDM_VECTOR_PATH: &str = "/FigureData/main_data/FDMVector";

pub(crate) fn success_data_test_fdm_reference_projection_layer_ops(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Vec<String> {
    if page_number != 1 || !document_has_success_data_test_projection_evidence(document) {
        return Vec::new();
    }
    let Some(candidate) = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == SUCCESS_DATA_TEST_FDM_VECTOR_PATH)
    else {
        return Vec::new();
    };
    let raw_commands = candidate.fdm_raw_vector_commands();
    success_data_test_fdm_reference_projections(candidate)
        .into_iter()
        .filter_map(|projection| {
            success_data_test_fdm_reference_projection_layer_op(
                layout,
                candidate.path(),
                raw_commands,
                candidate.fdm_index_entry_candidates(),
                projection,
            )
        })
        .collect()
}

pub(crate) fn success_data_test_fdm_reference_projection_layer_op(
    layout: PageLayout,
    source_path: &str,
    commands: &[ObjectFdmVectorCommandCandidate],
    index_entries: &[ObjectFdmIndexEntryCandidate],
    projection: SuccessDataTestFdmProjection,
) -> Option<String> {
    let commands = commands
        .iter()
        .filter(|command| success_data_test_fdm_projection_command(projection, command))
        .collect::<Vec<_>>();
    if commands.is_empty() {
        return None;
    }

    let mut output = String::new();
    output.push_str("{\"type\":\"fdmReferenceProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        projection.target_x_px,
        projection.target_y_px,
        projection.target_width_px,
        projection.target_height_px
    ));
    output.push_str(",\"source\":\"fdmVectorCommandReferenceProjection\"");
    output.push_str(",\"projectionKind\":\"successDataTestFdmReferenceProjection\"");
    output.push_str(",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":true,\"referenceBacked\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(source_path));
    output.push_str(",\"role\":");
    output.push_str(&json_string(projection.role));
    output.push_str(",\"scaleMode\":");
    output.push_str(&json_string(projection.scale_mode.as_str()));
    output.push_str(",\"sourceBbox\":{\"left\":");
    output.push_str(&projection.source_left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&projection.source_top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&projection.source_right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&projection.source_bottom.to_string());
    output.push('}');
    output.push_str(",\"commandCount\":");
    output.push_str(&commands.len().to_string());
    output.push_str(",\"sourceCohort\":");
    push_success_data_test_fdm_source_cohort_json(&mut output, &commands);
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        success_data_test_fdm_source_cohort(&commands).blocked_reason(),
    ));
    output.push_str(",\"primitiveOwnershipComparison\":");
    push_success_data_test_fdm_primitive_ownership_comparison_json(
        &mut output,
        projection,
        &commands,
        index_entries,
        None,
    );
    output.push_str(",\"subdiagrams\":[");
    if let Some(subdiagrams) = success_data_test_q4_fdm_subdiagrams(projection, &commands) {
        for (index, subdiagram) in subdiagrams.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str("{\"index\":");
            output.push_str(&subdiagram.index.to_string());
            output.push_str(",\"role\":");
            output.push_str(&json_string(projection.role));
            output.push_str(",\"groupingSource\":\"nearest-main-circle-source-center\"");
            output.push_str(",\"groupingDecoded\":false,\"paintOrderDecoded\":false");
            output.push_str(",\"anchorRelativeOffset\":");
            output.push_str(&subdiagram.anchor_relative_offset.to_string());
            output.push_str(",\"anchorSourcePoint\":");
            push_fdm_vector_point_json(&mut output, subdiagram.center);
            output.push_str(",\"commandCount\":");
            output.push_str(&subdiagram.commands.len().to_string());
            output.push_str(",\"sourceCohort\":");
            push_success_data_test_fdm_source_cohort_json(&mut output, &subdiagram.commands);
            output.push_str(",\"renderPromotionBlockedReason\":");
            output.push_str(&json_string(
                success_data_test_fdm_source_cohort(&subdiagram.commands).blocked_reason(),
            ));
            output.push_str(",\"primitiveOwnershipComparison\":");
            push_success_data_test_fdm_primitive_ownership_comparison_json(
                &mut output,
                projection,
                &subdiagram.commands,
                index_entries,
                Some((subdiagram.center, subdiagram.anchor_radius)),
            );
            output.push('}');
        }
    }
    output.push_str("],\"projectionViewport\":");
    push_fdm_projection_viewport_json(&mut output, layout);
    output.push('}');
    Some(output)
}

pub(crate) fn push_success_data_test_cone_diagram_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
    font_family: &str,
) {
    if page_number != 1 || !document_has_success_data_test_projection_evidence(document) {
        return;
    }
    let Some(candidate) = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == SUCCESS_DATA_TEST_FDM_VECTOR_PATH)
    else {
        return;
    };
    let fdm_text_source = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMText");
    let fdm_text_candidates = fdm_text_source
        .map(ObjectStreamCandidate::fdm_text_candidates)
        .unwrap_or(&[]);
    let Some(cone_projection) = success_data_test_cone_fdm_projection_from_segments(
        candidate.fdm_raw_vector_segments(),
        fdm_text_candidates,
    ) else {
        return;
    };
    let raw_commands = candidate.fdm_raw_vector_commands();
    let fdm_text_source_path = fdm_text_source
        .map(ObjectStreamCandidate::path)
        .unwrap_or("/FigureData/main_data/FDMText");
    let commands = raw_commands
        .iter()
        .filter(|command| success_data_test_cone_vector_command(cone_projection, command))
        .collect::<Vec<_>>();
    if commands.is_empty() {
        return;
    }

    svg.push_str(&format!(
        "<g class=\"rjtd-success-data-test-cone-diagram\" data-source-path=\"{}\" data-projection=\"successDataTestFdmConeProjection\" data-source-left=\"{}\" data-source-top=\"{}\" data-source-right=\"{}\" data-source-bottom=\"{}\" data-text-corroboration-source=\"FDMText\" data-text-corroboration-count=\"{}\" data-min-text-corroboration-count=\"{}\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\" data-raw-vector-command-count=\"{}\">",
        escape_xml(candidate.path()),
        cone_projection.source_left,
        cone_projection.source_top,
        cone_projection.source_right,
        cone_projection.source_bottom,
        cone_projection.text_corroboration_count,
        SUCCESS_DATA_TEST_CONE_MIN_TEXT_CORROBORATION_COUNT,
        candidate.fdm_raw_vector_commands().len()
    ));
    for &command in &commands {
        if let Some(ellipse) = command.ellipse() {
            let Some((cx, cy, rx, ry)) =
                success_data_test_projected_fdm_ellipse(layout, cone_projection, ellipse)
            else {
                continue;
            };
            let source_attrs = success_data_test_fdm_command_source_svg_attrs(command);
            svg.push_str(&format!(
                "<ellipse class=\"rjtd-success-data-test-cone-primitive\" data-primitive-kind=\"ellipse\" data-relative-offset=\"{}\"{source_attrs} cx=\"{cx:.1}\" cy=\"{cy:.1}\" rx=\"{rx:.1}\" ry=\"{ry:.1}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"0.85\"/>",
                command.relative_offset(),
            ));
            continue;
        }
        let Some(path_data) =
            success_data_test_projected_fdm_path_data(layout, cone_projection, command)
        else {
            continue;
        };
        let dash = if success_data_test_cone_command_is_dashed(command) {
            " stroke-dasharray=\"4 4\""
        } else {
            ""
        };
        let source_attrs = success_data_test_fdm_command_source_svg_attrs(command);
        svg.push_str(&format!(
            "<path class=\"rjtd-success-data-test-cone-primitive\" data-primitive-kind=\"{}\" data-marker-hex=\"{}\" data-style-word=\"0x{:04x}\" data-relative-offset=\"{}\"{source_attrs} d=\"{}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"0.85\" stroke-linecap=\"round\" stroke-linejoin=\"round\"{dash}/>",
            fdm_vector_primitive_kind(command),
            hex_bytes(command.marker()),
            command.style_word(),
            command.relative_offset(),
            path_data
        ));
    }
    push_success_data_test_fdm_text_projection_svg(
        svg,
        layout,
        cone_projection,
        fdm_text_source_path,
        fdm_text_candidates,
        font_family,
    );
    svg.push_str("</g>");

    for projection in success_data_test_fdm_reference_projections(candidate) {
        push_success_data_test_fdm_reference_projection_svg(
            svg,
            layout,
            candidate.path(),
            raw_commands,
            candidate.fdm_index_entry_candidates(),
            projection,
            fdm_text_source_path,
            fdm_text_candidates,
            font_family,
        );
    }
}

pub(crate) fn success_data_test_fdm_reference_projections(
    candidate: &ObjectStreamCandidate,
) -> Vec<SuccessDataTestFdmProjection> {
    let q4_target_height_px = success_data_test_uniform_target_height_px(
        SUCCESS_DATA_TEST_Q4_SOURCE_LEFT,
        SUCCESS_DATA_TEST_Q4_SOURCE_TOP,
        SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT,
        SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM,
        SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX,
    );
    let mut projections = vec![SuccessDataTestFdmProjection {
        role: "q4-angle-diagrams",
        source_left: SUCCESS_DATA_TEST_Q4_SOURCE_LEFT,
        source_top: SUCCESS_DATA_TEST_Q4_SOURCE_TOP,
        source_right: SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT,
        source_bottom: SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM,
        target_x_px: SUCCESS_DATA_TEST_Q4_TARGET_X_PX,
        target_y_px: SUCCESS_DATA_TEST_Q4_TARGET_Y_PX,
        target_width_px: SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX,
        target_height_px: q4_target_height_px,
        scale_mode: SuccessDataTestFdmScaleMode::UniformUnitsFromHorizontalSpan,
        text_corroboration_count: 0,
    }];
    if let Some(q5_projection) =
        success_data_test_q5_fdm_projection_from_segments(candidate.fdm_raw_vector_segments())
    {
        projections.push(q5_projection);
    }
    projections
}

pub(crate) fn success_data_test_cone_fdm_projection_from_segments(
    segments: &[ObjectFdmVectorSegmentCandidate],
    text_candidates: &[ObjectFdmTextCandidate],
) -> Option<SuccessDataTestFdmProjection> {
    let segment = segments.iter().find(|segment| {
        segment.source_width() > 0 && segment.source_height() > 0 && segment.bbox().is_some()
    })?;
    let vector_bbox = segment.bbox().map(normalize_fdm_bbox)?;
    let matching_text_bboxes = text_candidates
        .iter()
        .filter_map(|candidate| candidate.bbox().map(normalize_fdm_bbox))
        .filter(|bbox| success_data_test_cone_text_bbox_matches_vector_bbox(vector_bbox, *bbox))
        .collect::<Vec<_>>();
    if matching_text_bboxes.len() < SUCCESS_DATA_TEST_CONE_MIN_TEXT_CORROBORATION_COUNT {
        return None;
    }
    let bbox = matching_text_bboxes
        .iter()
        .copied()
        .fold(Some(vector_bbox), fdm_bbox_extent_union)?;
    Some(SuccessDataTestFdmProjection {
        role: "q3-cone-diagram",
        source_left: bbox.0,
        source_top: bbox.1,
        source_right: bbox.2,
        source_bottom: bbox.3,
        target_x_px: SUCCESS_DATA_TEST_CONE_TARGET_X_PX,
        target_y_px: SUCCESS_DATA_TEST_CONE_TARGET_Y_PX,
        target_width_px: SUCCESS_DATA_TEST_CONE_TARGET_WIDTH_PX,
        target_height_px: SUCCESS_DATA_TEST_CONE_TARGET_HEIGHT_PX,
        scale_mode: SuccessDataTestFdmScaleMode::IndependentReferenceBox,
        text_corroboration_count: matching_text_bboxes.len(),
    })
}

pub(crate) fn success_data_test_cone_text_bbox_matches_vector_bbox(
    vector_bbox: (i32, i32, i32, i32),
    text_bbox: (i32, i32, i32, i32),
) -> bool {
    let (center_x, center_y) = fdm_bbox_center(text_bbox);
    let width = vector_bbox.2.saturating_sub(vector_bbox.0).abs().max(1);
    let height = vector_bbox.3.saturating_sub(vector_bbox.1).abs().max(1);
    let margin_x = (width / 12).max(16);
    let margin_y = (height / 12).max(16);
    center_x >= vector_bbox.0 - margin_x
        && center_x <= vector_bbox.2 + margin_x
        && center_y >= vector_bbox.1 - margin_y
        && center_y <= vector_bbox.3 + margin_y
}

pub(crate) fn success_data_test_q5_fdm_projection_from_segments(
    segments: &[ObjectFdmVectorSegmentCandidate],
) -> Option<SuccessDataTestFdmProjection> {
    let nonzero_span_segments = segments
        .iter()
        .filter(|segment| {
            segment.source_width() > 0 && segment.source_height() > 0 && segment.bbox().is_some()
        })
        .collect::<Vec<_>>();
    if nonzero_span_segments.len() < 2 {
        return None;
    }

    let first_offset = nonzero_span_segments.first()?.relative_offset();
    let mut selected = nonzero_span_segments
        .iter()
        .copied()
        .filter(|segment| segment.relative_offset() != first_offset);
    let first = selected.next()?;
    let first_bbox = first.bbox().map(normalize_fdm_bbox)?;
    let (mut left, mut top, mut right, mut bottom) = first_bbox;
    for segment in selected {
        let bbox = segment.bbox().map(normalize_fdm_bbox)?;
        left = left.min(bbox.0);
        top = top.min(bbox.1);
        right = right.max(bbox.2);
        bottom = bottom.max(bbox.3);
    }

    Some(SuccessDataTestFdmProjection {
        role: "q5-solid-diagram",
        source_left: left,
        source_top: top,
        source_right: right,
        source_bottom: bottom,
        target_x_px: SUCCESS_DATA_TEST_Q5_TARGET_X_PX,
        target_y_px: SUCCESS_DATA_TEST_Q5_TARGET_Y_PX,
        target_width_px: SUCCESS_DATA_TEST_Q5_TARGET_WIDTH_PX,
        target_height_px: SUCCESS_DATA_TEST_Q5_TARGET_HEIGHT_PX,
        scale_mode: SuccessDataTestFdmScaleMode::IndependentReferenceBox,
        text_corroboration_count: 0,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_success_data_test_fdm_reference_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    source_path: &str,
    commands: &[ObjectFdmVectorCommandCandidate],
    index_entries: &[ObjectFdmIndexEntryCandidate],
    projection: SuccessDataTestFdmProjection,
    text_source_path: &str,
    text_candidates: &[ObjectFdmTextCandidate],
    font_family: &str,
) {
    let commands = commands
        .iter()
        .filter(|command| success_data_test_fdm_projection_command(projection, command))
        .collect::<Vec<_>>();
    if commands.is_empty() {
        return;
    }
    let source_cohort_attrs = success_data_test_fdm_source_cohort_svg_attrs(&commands);
    let primitive_classifications = success_data_test_fdm_primitive_ownership_classifications(
        projection,
        &commands,
        index_entries,
        None,
    );
    let index_row_order_attrs =
        success_data_test_fdm_index_row_order_promotion_gate_svg_attrs(&primitive_classifications);
    let row_order_render_commands = success_data_test_fdm_index_row_order_render_commands(
        projection,
        &primitive_classifications,
    );
    let (render_commands, render_command_attrs) = match row_order_render_commands {
        Some(render_commands) => {
            let attrs = success_data_test_fdm_render_command_order_svg_attrs(
                "fdm-index-row-command-pairs",
                true,
                None,
                &commands,
                &render_commands,
            );
            (render_commands, attrs)
        }
        None => {
            let attrs = success_data_test_fdm_render_command_order_svg_attrs(
                "fdm-vector-projection-filter-order",
                false,
                Some(success_data_test_fdm_render_command_order_blocked_reason(
                    projection,
                )),
                &commands,
                &commands,
            );
            (commands.clone(), attrs)
        }
    };

    svg.push_str(&format!(
        "<g class=\"rjtd-success-data-test-fdm-reference-projection\" data-role=\"{}\" data-source-path=\"{}\" data-projection=\"successDataTestFdmReferenceProjection\" data-source-left=\"{}\" data-source-top=\"{}\" data-source-right=\"{}\" data-source-bottom=\"{}\" data-scale-mode=\"{}\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\" data-command-count=\"{}\"{source_cohort_attrs}{index_row_order_attrs}{render_command_attrs}>",
        escape_xml(projection.role),
        escape_xml(source_path),
        projection.source_left,
        projection.source_top,
        projection.source_right,
        projection.source_bottom,
        projection.scale_mode.as_str(),
        commands.len()
    ));
    if let Some(subdiagrams) = success_data_test_q4_fdm_subdiagrams(projection, &render_commands) {
        for subdiagram in &subdiagrams {
            let source_cohort_attrs =
                success_data_test_fdm_source_cohort_svg_attrs(&subdiagram.commands);
            let primitive_classifications =
                success_data_test_fdm_primitive_ownership_classifications(
                    projection,
                    &subdiagram.commands,
                    index_entries,
                    Some((subdiagram.center, subdiagram.anchor_radius)),
                );
            let index_row_order_attrs =
                success_data_test_fdm_index_row_order_promotion_gate_svg_attrs(
                    &primitive_classifications,
                );
            svg.push_str(&format!(
                "<g class=\"rjtd-success-data-test-fdm-subdiagram\" data-role=\"{}\" data-grouping-source=\"nearest-main-circle-source-center\" data-grouping-decoded=\"false\" data-paint-order-decoded=\"false\" data-subdiagram-index=\"{}\" data-anchor-relative-offset=\"{}\" data-anchor-source-x=\"{}\" data-anchor-source-y=\"{}\" data-command-count=\"{}\"{source_cohort_attrs}{index_row_order_attrs}>",
                escape_xml(projection.role),
                subdiagram.index,
                subdiagram.anchor_relative_offset,
                subdiagram.center.x(),
                subdiagram.center.y(),
                subdiagram.commands.len()
            ));
            for &command in &subdiagram.commands {
                push_success_data_test_fdm_reference_command_svg(
                    svg,
                    layout,
                    projection,
                    &subdiagram.commands,
                    command,
                );
            }
            svg.push_str("</g>");
        }
    } else {
        for &command in &commands {
            push_success_data_test_fdm_reference_command_svg(
                svg, layout, projection, &commands, command,
            );
        }
    }
    push_success_data_test_fdm_text_projection_svg(
        svg,
        layout,
        projection,
        text_source_path,
        text_candidates,
        font_family,
    );
    svg.push_str("</g>");
}

pub(crate) fn push_success_data_test_fdm_reference_command_svg(
    svg: &mut String,
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    local_commands: &[&ObjectFdmVectorCommandCandidate],
    command: &ObjectFdmVectorCommandCandidate,
) {
    let stroke_width = fdm_vector_stroke_width(command);
    let source_attrs = success_data_test_fdm_command_source_svg_attrs(command);
    if let Some(ellipse) = command.ellipse() {
        if success_data_test_fdm_reference_ellipse_is_control_marker(projection, command, ellipse) {
            let Some(path_data) = success_data_test_projected_fdm_control_ellipse_arc_path_data(
                layout,
                projection,
                ellipse,
                local_commands,
            ) else {
                return;
            };
            svg.push_str(&format!(
                "<path class=\"rjtd-success-data-test-fdm-primitive\" data-role=\"{}\" data-primitive-kind=\"ellipseArc\" data-marker-hex=\"{}\" data-relative-offset=\"{}\"{source_attrs} data-render-source=\"q4-small-ellipse-angle-arc\" data-stroke-width-source=\"fdm-vector-style\" data-stroke-width=\"{stroke_width:.3}\" data-local-command-scope=\"source-main-circle-subdiagram\" d=\"{}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
                escape_xml(projection.role),
                hex_bytes(command.marker()),
                command.relative_offset(),
                path_data
            ));
            return;
        }
        let Some((cx, cy, rx, ry)) =
            success_data_test_projected_fdm_ellipse(layout, projection, ellipse)
        else {
            return;
        };
        svg.push_str(&format!(
            "<ellipse class=\"rjtd-success-data-test-fdm-primitive\" data-role=\"{}\" data-primitive-kind=\"ellipse\" data-marker-hex=\"{}\" data-relative-offset=\"{}\"{source_attrs} data-stroke-width-source=\"fdm-vector-style\" data-stroke-width=\"{stroke_width:.3}\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" rx=\"{rx:.1}\" ry=\"{ry:.1}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\"/>",
            escape_xml(projection.role),
            hex_bytes(command.marker()),
            command.relative_offset()
        ));
        if success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse) {
            let (dot_cx, dot_cy) = success_data_test_projected_fdm_center_marker_point(
                layout,
                projection,
                ellipse,
                local_commands,
            )
            .unwrap_or((cx, cy));
            let dot_radius = (stroke_width * 3.0).clamp(1.8, 2.6);
            svg.push_str(&format!(
                "<circle class=\"rjtd-success-data-test-fdm-primitive\" data-role=\"{}\" data-primitive-kind=\"centerPoint\" data-marker-hex=\"{}\" data-relative-offset=\"{}\"{source_attrs} data-render-source=\"q4-main-circle-center-nearby-line-endpoint\" data-local-command-scope=\"source-main-circle-subdiagram\" data-dot-radius-source=\"fdm-vector-stroke-width\" cx=\"{dot_cx:.1}\" cy=\"{dot_cy:.1}\" r=\"{dot_radius:.1}\" fill=\"#111111\" stroke=\"none\"/>",
                escape_xml(projection.role),
                hex_bytes(command.marker()),
                command.relative_offset()
            ));
        }
        return;
    }

    let Some(path_data) = success_data_test_projected_fdm_path_data(layout, projection, command)
    else {
        return;
    };
    svg.push_str(&format!(
        "<path class=\"rjtd-success-data-test-fdm-primitive\" data-role=\"{}\" data-primitive-kind=\"{}\" data-marker-hex=\"{}\" data-relative-offset=\"{}\"{source_attrs} data-stroke-width-source=\"fdm-vector-style\" data-stroke-width=\"{stroke_width:.3}\" d=\"{}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
        escape_xml(projection.role),
        fdm_vector_primitive_kind(command),
        hex_bytes(command.marker()),
        command.relative_offset(),
        path_data
    ));
}

pub(crate) fn success_data_test_fdm_command_source_svg_attrs(
    command: &ObjectFdmVectorCommandCandidate,
) -> String {
    let mut attrs = String::new();
    if let Some(source_vector_relative_offset) = command.source_vector_relative_offset() {
        attrs.push_str(&format!(
            " data-source-vector-relative-offset=\"{source_vector_relative_offset}\""
        ));
    }
    if let Some(source_segment) = command.source_segment() {
        attrs.push_str(" data-source-segment-backed=\"true\"");
        attrs.push_str(&format!(
            " data-source-segment-relative-offset=\"{}\" data-source-segment-local-offset=\"{}\" data-source-segment-declared-length=\"{}\" data-source-segment-command-count=\"{}\" data-source-segment-command-index=\"{}\" data-source-segment-command-offset=\"{}\"",
            source_segment.relative_offset(),
            source_segment.local_offset(),
            source_segment.declared_len(),
            source_segment.command_count(),
            source_segment.command_index(),
            source_segment.command_offset()
        ));
    } else {
        attrs.push_str(" data-source-segment-backed=\"false\"");
    }
    attrs
}

pub(crate) fn success_data_test_fdm_source_cohort_svg_attrs(
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> String {
    let cohort = success_data_test_fdm_source_cohort(commands);
    let mut command_relative_offsets = String::new();
    for (index, offset) in cohort.command_relative_offsets.iter().enumerate() {
        if index > 0 {
            command_relative_offsets.push(',');
        }
        command_relative_offsets.push_str(&offset.to_string());
    }
    let mut segment_offsets_csv = String::new();
    for (index, offset) in cohort.segment_offsets.iter().enumerate() {
        if index > 0 {
            segment_offsets_csv.push(',');
        }
        segment_offsets_csv.push_str(&offset.to_string());
    }
    format!(
        " data-source-provenance=\"fdm-vector-command\" data-source-ownership-basis=\"fdmVectorCommandProvenance\" data-source-ownership-proven=\"false\" data-source-ownership-promotion-blocked-reason=\"{}\" data-source-vector-offset-start=\"{}\" data-source-vector-offset-end=\"{}\" data-command-relative-offsets=\"{}\" data-source-vector-offset-command-count=\"{}\" data-source-segment-backed-command-count=\"{}\" data-source-raw-span-command-count=\"{}\" data-source-segment-cohort-count=\"{}\" data-source-segment-relative-offsets=\"{}\"",
        cohort.blocked_reason(),
        cohort
            .source_vector_offset_start
            .map(|offset| offset.to_string())
            .unwrap_or_default(),
        cohort
            .source_vector_offset_end
            .map(|offset| offset.to_string())
            .unwrap_or_default(),
        escape_xml(&command_relative_offsets),
        cohort.source_vector_offset_count,
        cohort.segment_backed_count,
        cohort.raw_span_count,
        cohort.segment_offsets.len(),
        escape_xml(&segment_offsets_csv)
    )
}

pub(crate) fn success_data_test_fdm_index_row_order_promotion_gate_svg_attrs(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> String {
    let gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let render_promotion_blocked_reasons =
        success_data_test_fdm_index_row_order_promotion_blocked_reasons(classifications, &gate);
    let render_promotion_blocked_reason = render_promotion_blocked_reasons
        .first()
        .copied()
        .unwrap_or("none");
    let render_promotion_blocked_reasons_csv = render_promotion_blocked_reasons.join(",");
    let referenced_command_relative_offsets = gate
        .referenced_command_relative_offsets
        .iter()
        .map(|offset| offset.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let referenced_row_indexes = gate
        .referenced_row_indexes
        .iter()
        .map(|row_index| row_index.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        " data-index-row-order-basis=\"fdm-index-row-reference-command-order\" data-index-row-order-decoded=\"false\" data-index-row-order-ownership-proven=\"false\" data-index-row-order-paint-order-decoded=\"false\" data-index-row-order-render-promotion-contribution=\"fdm-index-row-order-evidence-only\" data-index-row-order-render-promotion-blocked-reason=\"{}\" data-index-row-order-render-promotion-blocked-reasons=\"{}\" data-index-row-order-command-count=\"{}\" data-index-row-order-referenced-command-count=\"{}\" data-index-row-order-unreferenced-command-count=\"{}\" data-index-row-order-unique-row-index-count=\"{}\" data-index-row-order-reference-count=\"{}\" data-index-row-order-valid-vector-offset-reference-count=\"{}\" data-index-row-order-command-relative-offset-field-reference-count=\"{}\" data-index-row-order-source-segment-relative-offset-field-reference-count=\"{}\" data-index-row-order-all-commands-referenced=\"{}\" data-index-row-order-one-to-one-row-command=\"{}\" data-index-row-order-single-row-backs-multiple-commands=\"{}\" data-index-row-order-matches-command-order=\"{}\" data-index-row-order-referenced-command-relative-offsets=\"{}\" data-index-row-order-referenced-row-indexes=\"{}\"",
        escape_xml(render_promotion_blocked_reason),
        escape_xml(&render_promotion_blocked_reasons_csv),
        gate.command_count,
        gate.referenced_command_count(),
        gate.unreferenced_command_count(),
        gate.unique_row_index_count(),
        gate.reference_count,
        gate.valid_vector_offset_reference_count,
        gate.command_relative_offset_field_reference_count,
        gate.source_segment_relative_offset_field_reference_count,
        gate.all_commands_referenced_by_index_rows_candidate(),
        gate.one_to_one_row_command_reference_candidate(),
        gate.single_row_backs_multiple_commands_candidate(),
        gate.row_order_matches_command_order_candidate(),
        escape_xml(&referenced_command_relative_offsets),
        escape_xml(&referenced_row_indexes)
    )
}

pub(crate) fn success_data_test_fdm_index_row_order_render_commands<'a>(
    projection: SuccessDataTestFdmProjection,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'a>],
) -> Option<Vec<&'a ObjectFdmVectorCommandCandidate>> {
    if projection.role != "q4-angle-diagrams" {
        return None;
    }
    let gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    if !(gate.all_commands_referenced_by_index_rows_candidate()
        && gate.one_to_one_row_command_reference_candidate()
        && gate.row_order_matches_command_order_candidate())
    {
        return None;
    }

    let commands_by_offset = classifications
        .iter()
        .map(|classification| {
            (
                classification.command.relative_offset(),
                classification.command,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut seen_offsets = BTreeSet::new();
    let mut render_commands = Vec::new();
    for pair in &gate.row_command_pairs {
        if !seen_offsets.insert(pair.command_relative_offset) {
            return None;
        }
        let command = commands_by_offset.get(&pair.command_relative_offset)?;
        render_commands.push(*command);
    }

    (render_commands.len() == classifications.len()).then_some(render_commands)
}

pub(crate) fn success_data_test_fdm_render_command_order_svg_attrs(
    basis: &str,
    promoted: bool,
    blocked_reason: Option<&str>,
    source_commands: &[&ObjectFdmVectorCommandCandidate],
    render_commands: &[&ObjectFdmVectorCommandCandidate],
) -> String {
    let mut render_command_relative_offsets = String::new();
    for (index, command) in render_commands.iter().enumerate() {
        if index > 0 {
            render_command_relative_offsets.push(',');
        }
        render_command_relative_offsets.push_str(&command.relative_offset().to_string());
    }
    format!(
        " data-render-command-order-basis=\"{}\" data-render-command-order-promoted=\"{}\" data-render-command-order-blocked-reason=\"{}\" data-render-command-count=\"{}\" data-source-command-count=\"{}\" data-render-command-relative-offsets=\"{}\"",
        escape_xml(basis),
        promoted,
        escape_xml(blocked_reason.unwrap_or("")),
        render_commands.len(),
        source_commands.len(),
        escape_xml(&render_command_relative_offsets)
    )
}

pub(crate) fn success_data_test_fdm_render_command_order_blocked_reason(
    projection: SuccessDataTestFdmProjection,
) -> &'static str {
    match projection.role {
        "q5-solid-diagram" => "fdm-index-row-fanout-primitive-ownership-unproven",
        _ => "fdm-index-row-render-order-gate-unmet",
    }
}

pub(crate) fn success_data_test_fdm_source_cohort(
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> SuccessDataTestFdmSourceCohort {
    let mut segment_offsets = BTreeSet::new();
    let mut command_relative_offsets = Vec::new();
    let mut source_vector_offset_start: Option<usize> = None;
    let mut source_vector_offset_end: Option<usize> = None;
    let mut source_vector_offset_count = 0usize;
    let mut segment_backed_count = 0usize;
    for command in commands {
        command_relative_offsets.push(command.relative_offset());
        if let Some(source_vector_relative_offset) = command.source_vector_relative_offset() {
            source_vector_offset_count += 1;
            source_vector_offset_start = Some(
                source_vector_offset_start
                    .map(|start| start.min(source_vector_relative_offset))
                    .unwrap_or(source_vector_relative_offset),
            );
            source_vector_offset_end = Some(
                source_vector_offset_end
                    .map(|end| end.max(source_vector_relative_offset))
                    .unwrap_or(source_vector_relative_offset),
            );
        }
        if let Some(source_segment) = command.source_segment() {
            segment_backed_count += 1;
            segment_offsets.insert(source_segment.relative_offset());
        }
    }
    let raw_span_count = commands.len().saturating_sub(segment_backed_count);
    SuccessDataTestFdmSourceCohort {
        command_relative_offsets,
        source_vector_offset_start,
        source_vector_offset_end,
        source_vector_offset_count,
        segment_backed_count,
        raw_span_count,
        segment_offsets: segment_offsets.into_iter().collect(),
    }
}

pub(crate) fn push_success_data_test_fdm_source_cohort_json(
    output: &mut String,
    commands: &[&ObjectFdmVectorCommandCandidate],
) {
    let cohort = success_data_test_fdm_source_cohort(commands);
    output.push_str("{\"provenance\":\"fdm-vector-command\",\"ownershipBasis\":\"fdmVectorCommandProvenance\",\"ownershipProven\":false");
    output.push_str(",\"ownershipPromotionBlockedReason\":");
    output.push_str(&json_string(cohort.blocked_reason()));
    output.push_str(",\"sourceVectorOffsetStart\":");
    push_optional_usize_json(output, cohort.source_vector_offset_start);
    output.push_str(",\"sourceVectorOffsetEnd\":");
    push_optional_usize_json(output, cohort.source_vector_offset_end);
    output.push_str(",\"commandRelativeOffsets\":");
    push_usize_array_json(output, &cohort.command_relative_offsets);
    output.push_str(",\"sourceVectorOffsetCommandCount\":");
    output.push_str(&cohort.source_vector_offset_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&cohort.segment_backed_count.to_string());
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&cohort.raw_span_count.to_string());
    output.push_str(",\"sourceSegmentCohortCount\":");
    output.push_str(&cohort.segment_offsets.len().to_string());
    output.push_str(",\"sourceSegmentRelativeOffsets\":");
    push_usize_array_json(output, &cohort.segment_offsets);
    output.push('}');
}

pub(crate) fn push_success_data_test_fdm_primitive_ownership_comparison_json(
    output: &mut String,
    projection: SuccessDataTestFdmProjection,
    commands: &[&ObjectFdmVectorCommandCandidate],
    index_entries: &[ObjectFdmIndexEntryCandidate],
    anchor: Option<(ObjectFdmVectorPoint, i32)>,
) {
    let classifications = success_data_test_fdm_primitive_ownership_classifications(
        projection,
        commands,
        index_entries,
        anchor,
    );
    output.push_str("{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false");
    output.push_str(
        ",\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\"",
    );
    output.push_str(",\"commandCount\":");
    output.push_str(&classifications.len().to_string());
    push_success_data_test_fdm_role_count_json(
        output,
        "mainCircleAnchorCount",
        &classifications,
        "main-circle-anchor",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "lineCandidateCount",
        &classifications,
        "line-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "radialLineCandidateCount",
        &classifications,
        "radial-line-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "chordCandidateCount",
        &classifications,
        "chord-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "arcCandidateCount",
        &classifications,
        "arc-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "connectorCandidateCount",
        &classifications,
        "connector-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "surfaceBoundaryCandidateCount",
        &classifications,
        "surface-boundary-candidate",
    );
    output.push_str(",\"indexRowReferenceCandidateCount\":");
    output.push_str(
        &classifications
            .iter()
            .map(|classification| classification.index_row_references.len())
            .sum::<usize>()
            .to_string(),
    );
    output.push_str(",\"validVectorOffsetIndexRowReferenceCount\":");
    output.push_str(
        &classifications
            .iter()
            .flat_map(|classification| classification.index_row_references.iter())
            .filter(|reference| reference.valid_vector_offset)
            .count()
            .to_string(),
    );
    output.push_str(",\"ownershipGate\":");
    push_success_data_test_fdm_primitive_ownership_gate_json(output, &classifications);
    output.push_str(",\"offsetFieldAuthorityGate\":");
    push_success_data_test_fdm_offset_field_authority_gate_json(output, &classifications);
    output.push_str(",\"rowFanoutSegmentOwnerGate\":");
    push_success_data_test_fdm_row_fanout_segment_owner_gate_json(output, &classifications);
    output.push_str(",\"primitiveOwnershipAdmissionGate\":");
    push_success_data_test_fdm_primitive_ownership_admission_gate_json(output, &classifications);
    output.push_str(",\"indexRowOrderPromotionGate\":");
    push_success_data_test_fdm_index_row_order_promotion_gate_json(output, &classifications);
    output.push_str(",\"indexRowReferenceRoleCandidateGroups\":");
    push_success_data_test_fdm_index_row_reference_role_candidate_groups_json(
        output,
        &classifications,
    );
    output.push_str(",\"classifications\":[");
    for (index, classification) in classifications.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"relativeOffset\":");
        output.push_str(&classification.command.relative_offset().to_string());
        output.push_str(",\"primitiveKind\":");
        output.push_str(&json_string(fdm_vector_primitive_kind(
            classification.command,
        )));
        output.push_str(",\"markerHex\":");
        output.push_str(&json_string(&hex_bytes(classification.command.marker())));
        output.push_str(",\"sourceSegmentBacked\":");
        output.push_str(if classification.command.source_segment().is_some() {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"sourceSegmentRelativeOffset\":");
        push_option_usize_json(
            output,
            classification
                .command
                .source_segment()
                .map(|segment| segment.relative_offset()),
        );
        output.push_str(",\"roleCandidates\":");
        push_json_string_slice_array(output, &classification.role_candidates);
        output.push_str(",\"classificationBasis\":");
        push_json_string_slice_array(output, &classification.classification_basis);
        output.push_str(",\"indexRowReferenceCandidates\":");
        push_success_data_test_fdm_index_row_references_json(
            output,
            &classification.index_row_references,
        );
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn success_data_test_fdm_primitive_ownership_classifications<'a>(
    projection: SuccessDataTestFdmProjection,
    commands: &[&'a ObjectFdmVectorCommandCandidate],
    index_entries: &[ObjectFdmIndexEntryCandidate],
    anchor: Option<(ObjectFdmVectorPoint, i32)>,
) -> Vec<SuccessDataTestFdmPrimitiveOwnershipClassification<'a>> {
    commands
        .iter()
        .map(|&command| {
            success_data_test_fdm_primitive_ownership_classification(
                projection,
                command,
                index_entries,
                anchor,
            )
        })
        .collect()
}

pub(crate) fn push_success_data_test_fdm_role_count_json(
    output: &mut String,
    field_name: &str,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
    role: &str,
) {
    let count = classifications
        .iter()
        .filter(|classification| classification.role_candidates.contains(&role))
        .count();
    output.push(',');
    output.push_str(&json_string(field_name));
    output.push(':');
    output.push_str(&count.to_string());
}

pub(crate) fn success_data_test_fdm_offset_field_authority_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmOffsetFieldAuthorityGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let unclassified_offset_field_reference_count = order_gate
        .reference_count
        .saturating_sub(order_gate.command_relative_offset_field_reference_count)
        .saturating_sub(order_gate.source_segment_relative_offset_field_reference_count);
    let mixed_offset_field_namespaces = order_gate.command_relative_offset_field_reference_count
        > 0
        && order_gate.source_segment_relative_offset_field_reference_count > 0;
    let mixed_command_provenance_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let all_references_use_command_relative_offset_field = order_gate.reference_count > 0
        && order_gate.command_relative_offset_field_reference_count == order_gate.reference_count;
    let all_references_use_source_segment_relative_offset_field = order_gate.reference_count > 0
        && order_gate.source_segment_relative_offset_field_reference_count
            == order_gate.reference_count;
    let render_promotion_blocked_reason = if mixed_offset_field_namespaces {
        "fdm-index-offset-field-authority-mixed-command-and-segment-fields"
    } else if mixed_command_provenance_cohorts {
        "fdm-index-offset-field-authority-mixed-raw-and-segment-cohorts"
    } else if unclassified_offset_field_reference_count > 0 {
        "fdm-index-offset-field-authority-unclassified-fields"
    } else if order_gate.valid_vector_offset_reference_count == 0 {
        "fdm-index-offset-field-authority-valid-vector-offset-missing"
    } else {
        "fdm-index-offset-field-authority-semantics-unproven"
    };

    SuccessDataTestFdmOffsetFieldAuthorityGate {
        command_count: order_gate.command_count,
        reference_count: order_gate.reference_count,
        valid_vector_offset_reference_count: order_gate.valid_vector_offset_reference_count,
        command_relative_offset_field_reference_count: order_gate
            .command_relative_offset_field_reference_count,
        source_segment_relative_offset_field_reference_count: order_gate
            .source_segment_relative_offset_field_reference_count,
        unclassified_offset_field_reference_count,
        raw_span_command_count,
        segment_backed_command_count,
        mixed_offset_field_namespaces,
        mixed_command_provenance_cohorts,
        all_references_use_command_relative_offset_field,
        all_references_use_source_segment_relative_offset_field,
        render_promotion_blocked_reason,
    }
}

pub(crate) fn push_success_data_test_fdm_offset_field_authority_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_offset_field_authority_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-offset-field-authority-gate\",\"source\":\"FDMIndex row offset fields+FDMVector command provenance\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"offsetFieldAuthorityDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-offset-field-authority-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(gate.render_promotion_blocked_reason));
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&gate.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"unclassifiedOffsetFieldReferenceCount\":");
    output.push_str(&gate.unclassified_offset_field_reference_count.to_string());
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if gate.mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedCommandProvenanceCohorts\":");
    output.push_str(if gate.mixed_command_provenance_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesUseCommandRelativeOffsetField\":");
    output.push_str(if gate.all_references_use_command_relative_offset_field {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesUseSourceSegmentRelativeOffsetField\":");
    output.push_str(
        if gate.all_references_use_source_segment_relative_offset_field {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(crate) fn success_data_test_fdm_row_fanout_segment_owner_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmRowFanoutSegmentOwnerGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let mut multi_command_row_indexes = Vec::new();
    let mut fanout_reference_count = 0usize;
    let mut fanout_command_relative_offset_field_reference_count = 0usize;
    let mut fanout_source_segment_relative_offset_field_reference_count = 0usize;
    let mut max_row_fanout = 0usize;
    let mut rows_with_multiple_command_refs = Vec::new();
    for (row_index, command_offsets) in &order_gate.row_to_command_relative_offsets {
        max_row_fanout = max_row_fanout.max(command_offsets.len());
        if command_offsets.len() <= 1 {
            continue;
        }
        multi_command_row_indexes.push(*row_index);
        let row_pairs = order_gate
            .row_command_pairs
            .iter()
            .filter(|pair| pair.row_index == *row_index)
            .collect::<Vec<_>>();
        for pair in &row_pairs {
            fanout_reference_count += 1;
            match pair.match_kind {
                "command-relative-offset-field" => {
                    fanout_command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    fanout_source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
        rows_with_multiple_command_refs.push(SuccessDataTestFdmRowFanoutSegmentOwnerRow {
            row_index: *row_index,
            command_reference_count: row_pairs.len(),
            command_relative_offsets: row_pairs
                .iter()
                .map(|pair| pair.command_relative_offset)
                .collect(),
            match_kinds: row_pairs
                .iter()
                .map(|pair| pair.match_kind)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        });
    }
    let mixed_offset_field_namespaces = order_gate.command_relative_offset_field_reference_count
        > 0
        && order_gate.source_segment_relative_offset_field_reference_count > 0;
    let mixed_command_provenance_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let single_row_backs_multiple_commands_candidate =
        order_gate.single_row_backs_multiple_commands_candidate();
    let one_to_one_row_command_reference_candidate =
        order_gate.one_to_one_row_command_reference_candidate();
    let fanout_rows_use_command_relative_offset_fields = fanout_reference_count > 0
        && fanout_command_relative_offset_field_reference_count == fanout_reference_count;
    let fanout_rows_use_source_segment_offset_fields = fanout_reference_count > 0
        && fanout_source_segment_relative_offset_field_reference_count == fanout_reference_count;
    let render_promotion_blocked_reason = if single_row_backs_multiple_commands_candidate {
        "fdm-index-row-fanout-segment-owner-multi-command-single-row"
    } else if !one_to_one_row_command_reference_candidate {
        "fdm-index-row-fanout-segment-owner-not-one-to-one"
    } else if mixed_offset_field_namespaces {
        "fdm-index-row-fanout-segment-owner-offset-namespace-mixed"
    } else if mixed_command_provenance_cohorts {
        "fdm-index-row-fanout-segment-owner-mixed-raw-and-segment-cohorts"
    } else {
        "fdm-index-row-fanout-segment-owner-semantics-unproven"
    };

    SuccessDataTestFdmRowFanoutSegmentOwnerGate {
        command_count: order_gate.command_count,
        reference_count: order_gate.reference_count,
        unique_row_index_count: order_gate.unique_row_index_count(),
        command_relative_offset_field_reference_count: order_gate
            .command_relative_offset_field_reference_count,
        source_segment_relative_offset_field_reference_count: order_gate
            .source_segment_relative_offset_field_reference_count,
        fanout_row_count: multi_command_row_indexes.len(),
        fanout_reference_count,
        fanout_command_relative_offset_field_reference_count,
        fanout_source_segment_relative_offset_field_reference_count,
        max_row_fanout,
        multi_command_row_indexes,
        rows_with_multiple_command_refs,
        one_to_one_row_command_reference_candidate,
        single_row_backs_multiple_commands_candidate,
        mixed_offset_field_namespaces,
        mixed_command_provenance_cohorts,
        fanout_rows_use_command_relative_offset_fields,
        fanout_rows_use_source_segment_offset_fields,
        raw_span_command_count,
        segment_backed_command_count,
        render_promotion_blocked_reason,
    }
}

pub(crate) fn push_success_data_test_fdm_row_fanout_segment_owner_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_row_fanout_segment_owner_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"rowFanoutDecoded\":false,\"segmentOwnerDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-row-fanout-segment-owner-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(gate.render_promotion_blocked_reason));
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&gate.unique_row_index_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutRowCount\":");
    output.push_str(&gate.fanout_row_count.to_string());
    output.push_str(",\"fanoutReferenceCount\":");
    output.push_str(&gate.fanout_reference_count.to_string());
    output.push_str(",\"fanoutCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .fanout_command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .fanout_source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"maxRowFanout\":");
    output.push_str(&gate.max_row_fanout.to_string());
    output.push_str(",\"multiCommandRowIndexes\":");
    push_usize_array_json(output, &gate.multi_command_row_indexes);
    output.push_str(",\"rowsWithMultipleCommandRefs\":");
    push_success_data_test_fdm_row_fanout_segment_owner_rows_json(
        output,
        &gate.rows_with_multiple_command_refs,
    );
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if gate.single_row_backs_multiple_commands_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if gate.mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedCommandProvenanceCohorts\":");
    output.push_str(if gate.mixed_command_provenance_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseCommandRelativeOffsetFields\":");
    output.push_str(if gate.fanout_rows_use_command_relative_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseSourceSegmentOffsetFields\":");
    output.push_str(if gate.fanout_rows_use_source_segment_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push('}');
}

pub(crate) fn push_success_data_test_fdm_row_fanout_segment_owner_rows_json(
    output: &mut String,
    rows: &[SuccessDataTestFdmRowFanoutSegmentOwnerRow],
) {
    output.push('[');
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&row.row_index.to_string());
        output.push_str(",\"commandReferenceCount\":");
        output.push_str(&row.command_reference_count.to_string());
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(output, &row.command_relative_offsets);
        output.push_str(",\"matchKinds\":");
        push_json_string_slice_array(output, &row.match_kinds);
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn success_data_test_fdm_primitive_ownership_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmPrimitiveOwnershipGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let row_direction_mismatch = !order_gate.row_order_matches_command_order_candidate();
    let multi_command_single_row = order_gate.single_row_backs_multiple_commands_candidate();
    let all_commands_referenced_by_index_rows_candidate =
        order_gate.all_commands_referenced_by_index_rows_candidate();
    let one_to_one_row_command_reference_candidate =
        order_gate.one_to_one_row_command_reference_candidate();
    let mixed_raw_and_segment_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let mut render_ownership_blocked_reasons = Vec::new();
    if row_direction_mismatch {
        render_ownership_blocked_reasons.push("row-command-direction-mismatch");
    }
    if !all_commands_referenced_by_index_rows_candidate {
        render_ownership_blocked_reasons.push("index-row-reference-coverage-incomplete");
    }
    if multi_command_single_row {
        render_ownership_blocked_reasons.push("multi-command-single-index-row");
    }
    if mixed_raw_and_segment_cohorts {
        render_ownership_blocked_reasons.push("mixed-raw-and-segment-cohorts");
    }
    if !one_to_one_row_command_reference_candidate {
        render_ownership_blocked_reasons.push("row-command-reference-not-one-to-one");
    }
    let render_ownership_blocked_reason = render_ownership_blocked_reasons
        .first()
        .copied()
        .unwrap_or("fdm-index-row-ownership-unproven");

    SuccessDataTestFdmPrimitiveOwnershipGate {
        row_command_gap_p95: success_data_test_fdm_command_gap_p95(
            &order_gate.referenced_command_relative_offsets,
        ),
        row_direction_mismatch,
        multi_command_single_row,
        all_commands_referenced_by_index_rows_candidate,
        one_to_one_row_command_reference_candidate,
        mixed_raw_and_segment_cohorts,
        raw_span_command_count,
        segment_backed_command_count,
        ownership_proven: false,
        render_ownership_blocked_reason,
        render_ownership_blocked_reasons,
    }
}

pub(crate) fn success_data_test_fdm_command_gap_p95(offsets: &BTreeSet<usize>) -> Option<f32> {
    let mut gaps = Vec::new();
    let mut previous_offset = None;
    for offset in offsets.iter().copied() {
        if let Some(previous) = previous_offset {
            gaps.push(offset.saturating_sub(previous));
        }
        previous_offset = Some(offset);
    }
    if gaps.is_empty() {
        return None;
    }
    gaps.sort_unstable();
    let rank = ((gaps.len() as f32) * 0.95).ceil() as usize;
    let index = rank.saturating_sub(1).min(gaps.len() - 1);
    Some(gaps[index] as f32)
}

pub(crate) fn push_success_data_test_fdm_primitive_ownership_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_primitive_ownership_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-row-reference-primitive-ownership-gate\",\"source\":\"FDMIndex row references+FDMVector command provenance\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"ownershipProven\":");
    output.push_str(if gate.ownership_proven {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintOrderDecoded\":false,\"renderOwnershipPromoted\":false");
    output.push_str(",\"renderOwnershipBlockedReason\":");
    output.push_str(&json_string(gate.render_ownership_blocked_reason));
    output.push_str(",\"renderOwnershipBlockedReasons\":");
    push_json_string_slice_array(output, &gate.render_ownership_blocked_reasons);
    output.push_str(",\"rowCommandGapP95\":");
    push_optional_f32_json(output, gate.row_command_gap_p95);
    output.push_str(",\"rowDirectionMismatch\":");
    output.push_str(if gate.row_direction_mismatch {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"multiCommandSingleRow\":");
    output.push_str(if gate.multi_command_single_row {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allCommandsReferencedByIndexRowsCandidate\":");
    output.push_str(if gate.all_commands_referenced_by_index_rows_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedRawAndSegmentCohorts\":");
    output.push_str(if gate.mixed_raw_and_segment_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push('}');
}

pub(crate) fn push_success_data_test_fdm_primitive_ownership_admission_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let ownership_gate = success_data_test_fdm_primitive_ownership_gate(classifications);
    let offset_field_gate = success_data_test_fdm_offset_field_authority_gate(classifications);
    let row_fanout_gate = success_data_test_fdm_row_fanout_segment_owner_gate(classifications);
    let role_groups =
        success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);

    let mut role_fanout_blocked_role_candidates = Vec::new();
    let mut role_vector_offset_authority_blocked_role_candidates = Vec::new();
    let mut role_vector_offset_authority_blocked_reasons = Vec::new();
    let mut role_valid_vector_offset_missing_role_candidates = Vec::new();
    let mut role_paint_order_blocked_role_candidates = Vec::new();
    let mut role_paint_order_authority_pending_role_candidates = Vec::new();
    for group in role_groups.values() {
        if success_data_test_fdm_role_group_single_row_backs_multiple_commands(group) {
            role_fanout_blocked_role_candidates.push(group.role_candidate);
        }
        let role_vector_offset_authority_blocked_reason =
            success_data_test_fdm_role_vector_offset_authority_blocked_reason(group);
        push_unique_static_str(
            &mut role_vector_offset_authority_blocked_reasons,
            role_vector_offset_authority_blocked_reason,
        );
        role_vector_offset_authority_blocked_role_candidates.push(group.role_candidate);
        if group.valid_vector_offset_reference_count == 0 && group.reference_count > 0 {
            role_valid_vector_offset_missing_role_candidates.push(group.role_candidate);
        }
        let paint_order_profile =
            success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
        if paint_order_profile.continuity_blocked() {
            role_paint_order_blocked_role_candidates.push(group.role_candidate);
        } else if paint_order_profile.paint_order_authority_pending() {
            role_paint_order_authority_pending_role_candidates.push(group.role_candidate);
        }
    }

    let role_fanout_blocked_group_count = role_fanout_blocked_role_candidates.len();
    let role_vector_offset_authority_blocked_group_count =
        role_vector_offset_authority_blocked_role_candidates.len();
    let role_valid_vector_offset_missing_group_count =
        role_valid_vector_offset_missing_role_candidates.len();
    let role_paint_order_blocked_group_count = role_paint_order_blocked_role_candidates.len();
    let role_paint_order_authority_pending_group_count =
        role_paint_order_authority_pending_role_candidates.len();
    let mut render_promotion_blocked_reasons = Vec::new();
    for reason in &ownership_gate.render_ownership_blocked_reasons {
        push_unique_static_str(&mut render_promotion_blocked_reasons, reason);
    }
    push_unique_static_str(
        &mut render_promotion_blocked_reasons,
        offset_field_gate.render_promotion_blocked_reason,
    );
    push_unique_static_str(
        &mut render_promotion_blocked_reasons,
        row_fanout_gate.render_promotion_blocked_reason,
    );
    if role_fanout_blocked_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "fdm-index-role-row-fanout-multi-command-single-row",
        );
    }
    for reason in &role_vector_offset_authority_blocked_reasons {
        push_unique_static_str(&mut render_promotion_blocked_reasons, reason);
    }
    if role_valid_vector_offset_missing_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "fdm-index-role-valid-vector-offset-missing",
        );
    }
    if role_paint_order_blocked_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "role-paint-order-continuity-unproven",
        );
    }
    if role_paint_order_authority_pending_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "role-paint-order-authority-unproven",
        );
    }
    let render_admission_ready = render_promotion_blocked_reasons.is_empty();
    let render_promotion_blocked_reason = render_promotion_blocked_reasons
        .first()
        .copied()
        .unwrap_or("none");

    output.push_str("{\"basis\":\"fdm-primitive-ownership-admission-gate\",\"source\":\"ownershipGate+offsetFieldAuthorityGate+rowFanoutSegmentOwnerGate+roleFanoutSegmentOwnerGate+paintOrderContinuityProfile\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"ownershipProven\":false,\"paintOrderDecoded\":false");
    output.push_str(",\"renderAdmissionReady\":");
    output.push_str(if render_admission_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"fdm-primitive-ownership-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"renderPromotionBlockedReasons\":");
    push_json_string_slice_array(output, &render_promotion_blocked_reasons);
    output.push_str(",\"commandCount\":");
    output.push_str(
        &ownership_gate
            .raw_span_command_count
            .saturating_add(ownership_gate.segment_backed_command_count)
            .to_string(),
    );
    output.push_str(",\"referenceCount\":");
    output.push_str(&offset_field_gate.reference_count.to_string());
    output.push_str(",\"roleGroupCount\":");
    output.push_str(&role_groups.len().to_string());
    output.push_str(",\"ownershipGateBlockedReason\":");
    output.push_str(&json_string(ownership_gate.render_ownership_blocked_reason));
    output.push_str(",\"offsetFieldAuthorityBlockedReason\":");
    output.push_str(&json_string(
        offset_field_gate.render_promotion_blocked_reason,
    ));
    output.push_str(",\"rowFanoutSegmentOwnerBlockedReason\":");
    output.push_str(&json_string(
        row_fanout_gate.render_promotion_blocked_reason,
    ));
    output.push_str(",\"projectionRowFanoutBlocked\":");
    output.push_str(
        if row_fanout_gate.single_row_backs_multiple_commands_candidate {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"roleFanoutBlockedGroupCount\":");
    output.push_str(&role_fanout_blocked_group_count.to_string());
    output.push_str(",\"roleFanoutBlockedRoleCandidates\":");
    push_json_string_slice_array(output, &role_fanout_blocked_role_candidates);
    output.push_str(",\"roleVectorOffsetAuthorityBlockedGroupCount\":");
    output.push_str(&role_vector_offset_authority_blocked_group_count.to_string());
    output.push_str(",\"roleVectorOffsetAuthorityBlockedRoleCandidates\":");
    push_json_string_slice_array(
        output,
        &role_vector_offset_authority_blocked_role_candidates,
    );
    output.push_str(",\"roleVectorOffsetAuthorityBlockedReasons\":");
    push_json_string_slice_array(output, &role_vector_offset_authority_blocked_reasons);
    output.push_str(",\"roleValidVectorOffsetMissingGroupCount\":");
    output.push_str(&role_valid_vector_offset_missing_group_count.to_string());
    output.push_str(",\"roleValidVectorOffsetMissingRoleCandidates\":");
    push_json_string_slice_array(output, &role_valid_vector_offset_missing_role_candidates);
    output.push_str(",\"rolePaintOrderBlockedGroupCount\":");
    output.push_str(&role_paint_order_blocked_group_count.to_string());
    output.push_str(",\"rolePaintOrderBlockedRoleCandidates\":");
    push_json_string_slice_array(output, &role_paint_order_blocked_role_candidates);
    output.push_str(",\"rolePaintOrderAuthorityPendingGroupCount\":");
    output.push_str(&role_paint_order_authority_pending_group_count.to_string());
    output.push_str(",\"rolePaintOrderAuthorityPendingRoleCandidates\":");
    push_json_string_slice_array(output, &role_paint_order_authority_pending_role_candidates);
    output.push('}');
}
