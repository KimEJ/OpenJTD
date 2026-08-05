use super::*;
use crate::*;

pub(crate) const SUCCESS_DATA_TEST_TITLE_ART_MAX_SEGMENT_SOURCE_LEN: f32 = 240.0;

pub(crate) const SUCCESS_DATA_TEST_TITLE_ART_STROKE_WIDTH_PX: f32 = 0.32;

pub(crate) const SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES: usize = 18;

pub(crate) const SUCCESS_DATA_TEST_TITLE_ART_STATE_SIGNATURE_PREVIEW_LIMIT: usize = 12;

pub(crate) const SUCCESS_DATA_TEST_TITLE_ART_FRONT_FILL_COLOR: &str = "#111111";

pub(crate) fn push_page_layer_success_data_test_title_art_projection_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    lines: &[PageTextLine],
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) {
    let Some((frame_record_x, frame_record_y, width, height)) =
        embedding_frame_render_bbox(layout, lines, document, diagnostic)
    else {
        return;
    };
    let Some(snapshot) = diagnostic.embedded_press_snapshot else {
        return;
    };
    let scale_x = if snapshot.width() == 0 {
        0.0
    } else {
        width / snapshot.width() as f32
    };
    let source_frame_candidate = success_data_test_title_art_jsfart_frame_candidate(
        document,
        diagnostic.frame.embedding_index(),
    );
    let source_paint_candidate = success_data_test_title_art_jsfart_art_candidate(
        document,
        diagnostic.frame.embedding_index(),
    )
    .and_then(ObjectJsfartArtCandidate::paint_candidate);
    let scale_y = if snapshot.height() == 0 {
        0.0
    } else {
        height / snapshot.height() as f32
    };
    let (frame_scale_y, frame_scale_y_basis, frame_scale_y_source_units) =
        success_data_test_title_art_frame_vertical_scale(height, snapshot, source_frame_candidate);
    let render_height = snapshot.height() as f32 * frame_scale_y;
    let horizontal_placement = success_data_test_title_art_horizontal_placement(
        frame_record_x,
        source_frame_candidate,
        scale_x,
    );
    let content_left_adjustment = horizontal_placement.content_left_adjustment;
    let source_content_top_adjustment =
        source_frame_candidate.map_or(0.0, |frame| frame.content_top() as f32 * scale_y);
    let content_top_adjustment =
        source_frame_candidate.map_or(0.0, |frame| frame.content_top() as f32 * frame_scale_y);
    let vertical_stroke_center_adjustment = source_frame_candidate.map_or(0.0, |frame| {
        success_data_test_title_art_frame_stroke_width(frame, scale_x, frame_scale_y) * 0.5
    });
    let x = horizontal_placement.frame_x;
    let path_x = horizontal_placement.path_x;
    let y = (frame_record_y - content_top_adjustment + vertical_stroke_center_adjustment).max(0.0);
    let source_path_y = (frame_record_y - source_content_top_adjustment
        + vertical_stroke_center_adjustment)
        .max(0.0);

    output.push_str("{\"type\":\"titleArtProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{render_height:.3}}}"
    ));
    output.push_str(",\"source\":\"jsfartArtEmbeddedPressSnapshot\",\"projectionKind\":\"successDataTestTitleArtProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":true,\"referenceBacked\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.frame.source_path()));
    output.push_str(",\"frameCandidateIndex\":");
    output.push_str(&diagnostic.frame_index.to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&diagnostic.frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    output.push_str(&json_string(diagnostic.frame.class_name()));
    output.push_str(",\"frameRef\":");
    output.push_str(&diagnostic.frame.frame_ref().to_string());
    let title_frame_refs = success_data_test_title_art_frame_refs(document);
    let page_number_from_source_order = title_frame_refs
        .iter()
        .position(|frame_ref| *frame_ref == diagnostic.frame.frame_ref())
        .map(|index| index + 1);
    output.push_str(",\"pageAssociation\":{\"source\":\"JSFart.Art.2 frameRef source order\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"frameRefs\":");
    push_u32_array_json(output, &title_frame_refs);
    output.push_str(",\"sourceOrderIndex\":");
    push_option_usize_json(output, page_number_from_source_order.map(|page| page - 1));
    output.push_str(",\"pageNumber\":");
    push_option_usize_json(output, page_number_from_source_order);
    output.push('}');
    output.push_str(",\"placementMode\":\"frameRecordContentOffsetAnchor\"");
    output.push_str(",\"contentLeftAdjustment\":{\"sourceUnits\":");
    if let Some(frame) = source_frame_candidate {
        output.push_str(&frame.content_left().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"cssPx\":");
    output.push_str(&format!("{content_left_adjustment:.3}"));
    output.push('}');
    output.push_str(",\"horizontalPlacementGate\":");
    push_success_data_test_title_art_horizontal_placement_json(output, horizontal_placement);
    output.push_str(",\"sourceFrameRenderTrace\":");
    push_success_data_test_title_art_source_frame_render_trace_json(
        output,
        source_frame_candidate,
        diagnostic.frame_record,
        diagnostic.frame.frame_ref(),
        horizontal_placement,
        frame_scale_y_basis,
        frame_scale_y_source_units,
    );
    output.push_str(",\"contentTopAdjustment\":{\"sourceUnits\":");
    if let Some(frame) = source_frame_candidate {
        output.push_str(&frame.content_top().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"cssPx\":");
    output.push_str(&format!("{content_top_adjustment:.3}"));
    output.push('}');
    output.push_str(",\"verticalStrokeCenterAdjustment\":{\"cssPx\":");
    output.push_str(&format!("{vertical_stroke_center_adjustment:.3}"));
    output.push_str(",\"source\":\"jsfart-frame-stroke-centered-on-border\"}");
    output.push_str(",\"embeddingFrameSize\":{\"primaryWidth\":");
    output.push_str(&diagnostic.frame.primary_width().to_string());
    output.push_str(",\"primaryHeight\":");
    output.push_str(&diagnostic.frame.primary_height().to_string());
    output.push_str(",\"frameWidth\":");
    output.push_str(&diagnostic.frame.frame_width().to_string());
    output.push_str(",\"frameHeight\":");
    output.push_str(&diagnostic.frame.frame_height().to_string());
    output.push('}');
    output.push_str(",\"sourceScale\":{\"x\":");
    if snapshot.width() == 0 {
        output.push_str("null");
    } else {
        output.push_str(&format!("{scale_x:.6}"));
    }
    output.push_str(",\"y\":");
    if snapshot.height() == 0 {
        output.push_str("null");
    } else {
        output.push_str(&format!("{scale_y:.6}"));
    }
    output.push('}');
    output.push_str(",\"frameScale\":{\"x\":");
    if snapshot.width() == 0 {
        output.push_str("null");
    } else {
        output.push_str(&format!("{scale_x:.6}"));
    }
    output.push_str(",\"y\":");
    if frame_scale_y_source_units == 0 {
        output.push_str("null");
    } else {
        output.push_str(&format!("{frame_scale_y:.6}"));
    }
    output.push_str(",\"yBasis\":");
    output.push_str(&json_string(frame_scale_y_basis));
    output.push_str(",\"ySourceUnits\":");
    if frame_scale_y_source_units == 0 {
        output.push_str("null");
    } else {
        output.push_str(&frame_scale_y_source_units.to_string());
    }
    output.push('}');
    output.push_str(",\"pathScaleDiagnostic\":");
    push_success_data_test_title_art_path_scale_bbox_diagnostic_json(
        output,
        snapshot,
        path_x,
        source_path_y,
        y,
        scale_x,
        scale_y,
        frame_scale_y,
    );
    output.push_str(",\"frameRecordRect\":");
    if let Some(record) = diagnostic.frame_record {
        let record_x = hundredth_millimeters_to_css_px(u32::from(record.x()));
        let record_y = hundredth_millimeters_to_css_px(u32::from(record.y()));
        let record_width = hundredth_millimeters_to_css_px(u32::from(record.width()));
        let record_height = hundredth_millimeters_to_css_px(u32::from(record.height()));
        output.push_str("{\"sourcePath\":");
        output.push_str(&json_string(record.source_path()));
        output.push_str(",\"rowIndex\":");
        output.push_str(&record.row_index().to_string());
        output.push_str(",\"objectId\":");
        output.push_str(&record.object_id().to_string());
        output.push_str(",\"objectType\":");
        output.push_str(&record.object_type().to_string());
        output.push_str(",\"objectTypeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", record.object_type())));
        output.push_str(",\"sourceUnits\":{\"x\":");
        output.push_str(&record.x().to_string());
        output.push_str(",\"y\":");
        output.push_str(&record.y().to_string());
        output.push_str(",\"width\":");
        output.push_str(&record.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&record.height().to_string());
        output.push_str("},\"cssPx\":{\"x\":");
        output.push_str(&format!("{record_x:.3}"));
        output.push_str(",\"y\":");
        output.push_str(&format!("{record_y:.3}"));
        output.push_str(",\"width\":");
        output.push_str(&format!("{record_width:.3}"));
        output.push_str(",\"height\":");
        output.push_str(&format!("{record_height:.3}"));
        output.push_str("}}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"snapshot\":{\"format\":\"JSSnapShot32\",\"width\":");
    output.push_str(&snapshot.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&snapshot.height().to_string());
    output.push_str(",\"vectorSegmentCount\":");
    output.push_str(&snapshot.vector_segments().len().to_string());
    output.push_str(",\"vectorPathCount\":");
    output.push_str(&snapshot.vector_paths().len().to_string());
    output.push_str(",\"outlinePathCount\":");
    output.push_str(
        &embedded_press_snapshot_vector_path_kind_count(
            snapshot,
            ObjectEmbeddedPressVectorPathKind::Outline,
        )
        .to_string(),
    );
    output.push_str(",\"texturePathCount\":");
    output.push_str(
        &embedded_press_snapshot_vector_path_kind_count(
            snapshot,
            ObjectEmbeddedPressVectorPathKind::Texture,
        )
        .to_string(),
    );
    output.push_str(",\"vectorPathStateRecordCount\":");
    output.push_str(&embedded_press_snapshot_vector_path_state_record_count(snapshot).to_string());
    output.push_str(",\"vectorPathStateRecordTypes\":");
    push_embedded_press_state_record_type_summary_json(output, snapshot);
    output.push_str(",\"textureBezierHeaderSummary\":");
    push_embedded_press_texture_bezier_header_summary_json(output, snapshot);
    output.push_str(",\"paintStateTransitions\":");
    push_embedded_press_paint_state_transitions_json(output, snapshot);
    output.push_str(",\"titleArtPaintStateSequence\":");
    push_success_data_test_title_art_paint_state_sequence_json(output, snapshot);
    output.push_str(",\"renderedSegmentCount\":");
    output.push_str(&success_data_test_title_art_rendered_segment_count(snapshot).to_string());
    output.push_str(",\"renderedPathCount\":");
    output.push_str(&success_data_test_title_art_rendered_path_count(snapshot).to_string());
    output.push_str(",\"renderedTexturePathCount\":");
    output.push_str(&success_data_test_title_art_rendered_texture_path_count(snapshot).to_string());
    let state_tagged_texture_paths =
        success_data_test_title_art_state_tagged_texture_paths(snapshot);
    output.push_str(",\"stateTaggedTexturePathCount\":");
    output.push_str(&state_tagged_texture_paths.len().to_string());
    output.push_str(",\"stateTaggedTextureWord5Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_vector_path_state_word5_values(&state_tagged_texture_paths),
    );
    let front_texture_paths = success_data_test_title_art_front_texture_paths(snapshot);
    let effective_front_texture_word5_values =
        success_data_test_title_art_effective_front_texture_word5_values(snapshot);
    output.push_str(",\"frontTexturePathCount\":");
    output.push_str(&front_texture_paths.len().to_string());
    output.push_str(",\"frontTextureWord5Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_vector_path_state_word5_values(&front_texture_paths),
    );
    output.push_str(",\"effectiveFrontTextureWord5Values\":");
    push_u32_hex_array_json(output, &effective_front_texture_word5_values);
    let front_erase_texture_paths = success_data_test_title_art_front_erase_texture_paths(snapshot);
    output.push_str(",\"frontEraseTexturePathCount\":");
    output.push_str(&front_erase_texture_paths.len().to_string());
    output.push_str(",\"frontEraseTextureWord5Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_vector_path_state_word5_values(&front_erase_texture_paths),
    );
    output.push_str(",\"frontEraseTextureRecord70Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(
            &front_erase_texture_paths,
            EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70,
        ),
    );
    output.push_str(",\"frontEraseTextureOpacity\":");
    let front_erase_opacity =
        embedded_press_title_art_front_erase_texture_opacity(&front_erase_texture_paths);
    if let Some((opacity, _source)) = front_erase_opacity {
        output.push_str(&format!("{opacity:.3}"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"frontEraseTextureOpacitySource\":");
    match front_erase_opacity {
        Some((_opacity, source)) => output.push_str(&json_string(source)),
        None => output.push_str("null"),
    }
    output.push_str(",\"frontEraseTextureDirectGrayCandidate\":");
    push_embedded_press_title_art_direct_gray_candidate_json(output, &front_erase_texture_paths);
    output.push_str(",\"frontEraseTextureSourcePaintCandidate\":");
    push_embedded_press_title_art_source_paint_candidate_json(
        output,
        &front_erase_texture_paths,
        source_paint_candidate,
    );
    output.push_str(",\"frontEraseTextureSpanCoverageProbe\":");
    push_embedded_press_title_art_front_erase_texture_span_coverage_probe_json(
        output,
        snapshot,
        &front_erase_texture_paths,
    );
    output.push_str(",\"frontEraseTexturePathSource\":");
    match success_data_test_title_art_front_erase_texture_path_source(&front_erase_texture_paths) {
        Some(source) => output.push_str(&json_string(source)),
        None => output.push_str("null"),
    }
    output.push_str(",\"frontEraseTextureStateSummary\":");
    push_success_data_test_title_art_paint_state_summary_json(
        output,
        snapshot,
        "frontEraseTextureCandidate",
        &front_erase_texture_paths,
    );
    output.push_str(",\"frontEraseTextureRoleGate\":");
    push_success_data_test_title_art_front_texture_role_gate_json(
        output,
        snapshot,
        &front_erase_texture_paths,
    );
    output.push_str(",\"titleTexturePaintPhaseGate\":");
    push_success_data_test_title_art_texture_paint_phase_gate_json(output, snapshot);
    output.push_str(",\"titleShadowPaintWordGate\":");
    push_success_data_test_title_art_shadow_paint_word_gate_json(output, snapshot);
    output.push_str(",\"titlePaintRoleSeparationMatrix\":");
    push_success_data_test_title_art_paint_role_separation_matrix_json(output, snapshot);
    let shadow_partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let shadow_effect = shadow_partition
        .as_ref()
        .and_then(|partition| embedded_press_title_art_shadow_effect(&partition.shadow_paths));
    let extrusion_texture_paths = shadow_partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();
    output.push_str(",\"extrusionTexturePathCount\":");
    output.push_str(&extrusion_texture_paths.len().to_string());
    output.push_str(",\"extrusionTextureClipGate\":");
    push_success_data_test_title_art_extrusion_texture_clip_gate_json(
        output,
        extrusion_texture_paths.len(),
        "source-shadow-outline",
    );
    output.push_str(",\"titleTextureGeometryRoleGate\":");
    push_success_data_test_title_art_texture_geometry_role_gate_json(
        output,
        snapshot,
        &extrusion_texture_paths,
    );
    output.push_str(",\"extrusionTextureRecord70Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(
            &extrusion_texture_paths,
            EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70,
        ),
    );
    let extrusion_texture_effect = shadow_effect.as_ref().and_then(|effect| {
        embedded_press_title_art_texture_effect(&extrusion_texture_paths, &effect.fill_color)
    });
    output.push_str(",\"extrusionTextureEffectCandidateFillColor\":");
    if let Some(effect) = extrusion_texture_effect.as_ref() {
        output.push_str(&json_string(&effect.fill_color));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"extrusionTextureEffectCandidateOpacity\":");
    if let Some(effect) = extrusion_texture_effect.as_ref() {
        output.push_str(&format!("{:.3}", effect.opacity));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"extrusionTextureEffectCandidateWord0\":");
    if let Some(effect) = extrusion_texture_effect.as_ref() {
        output.push_str(&effect.word0.to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"extrusionTextureEffectCandidateSource\":");
    if extrusion_texture_effect.is_some() {
        output.push_str(&json_string(
            "embedded-press-interstitial-0x70-word0-percent-black-over-shadow",
        ));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"extrusionTextureEffectRenderPromoted\":");
    output.push_str("false");
    output.push_str(",\"extrusionTextureEffectRenderPromotionBlockedReason\":");
    if extrusion_texture_effect.is_some() {
        output.push_str(&json_string(
            "record70-separates-shadow-but-not-interstitial-texture-from-main",
        ));
    } else {
        output.push_str("null");
    }
    let shadow_texture_paths = success_data_test_title_art_shadow_texture_paths(snapshot);
    output.push_str(",\"shadowTexturePathCount\":");
    output.push_str(&shadow_texture_paths.len().to_string());
    output.push_str(",\"shadowTextureWord5Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_vector_path_state_word5_values(&shadow_texture_paths),
    );
    output.push_str(",\"shadowEffectFillColor\":");
    if let Some(effect) = shadow_effect.as_ref() {
        output.push_str(&json_string(&effect.fill_color));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"shadowEffectOpacity\":");
    if let Some(effect) = shadow_effect.as_ref() {
        output.push_str(&format!("{:.3}", effect.opacity));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"shadowEffectWord0\":");
    if let Some(effect) = shadow_effect.as_ref() {
        output.push_str(&effect.word0.to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"shadowEffectSource\":");
    if shadow_effect.is_some() {
        output.push_str(&json_string(
            "embedded-press-0x70-word0-percent-black-on-white",
        ));
    } else {
        output.push_str("null");
    }
    let effective_shadow_texture_paths =
        success_data_test_title_art_effective_texture_paths_for_word5(
            snapshot,
            EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5,
        );
    output.push_str(",\"effectiveShadowTexturePathCount\":");
    output.push_str(&effective_shadow_texture_paths.len().to_string());
    let effective_front_texture_paths =
        success_data_test_title_art_effective_texture_paths_for_word5(
            snapshot,
            EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5,
        );
    output.push_str(",\"effectiveFrontTexturePathCount\":");
    output.push_str(&effective_front_texture_paths.len().to_string());
    output.push_str(",\"effectiveTextureWord5Values\":");
    push_u32_hex_array_json(
        output,
        &success_data_test_title_art_effective_texture_word5_values(snapshot),
    );
    output.push_str(",\"textureStateInheritance\":\"embeddedPressCurrentPaintState\"");
    output.push_str(",\"paintStateSummaries\":");
    push_success_data_test_title_art_paint_state_summaries_json(output, snapshot);
    output.push_str(",\"paintStateColor\":");
    if let Some(color) = embedded_press_snapshot_paint_state_color_hex(snapshot) {
        output.push_str(&json_string(&color));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"frontPaintCandidate\":");
    push_success_data_test_title_art_front_paint_candidate_json(
        output,
        snapshot,
        source_paint_candidate,
    );
    output.push_str(",\"frontFillWindingGate\":");
    push_success_data_test_title_art_front_fill_winding_gate_json(output, snapshot);
    output.push_str(",\"sourceFrameCandidate\":");
    if let Some(frame) = source_frame_candidate {
        output.push_str("{\"source\":\"JSFart2Contents\",\"left\":");
        output.push_str(&frame.left().to_string());
        output.push_str(",\"top\":");
        output.push_str(&frame.top().to_string());
        output.push_str(",\"right\":");
        output.push_str(&frame.right().to_string());
        output.push_str(",\"bottom\":");
        output.push_str(&frame.bottom().to_string());
        output.push_str(",\"contentLeft\":");
        output.push_str(&frame.content_left().to_string());
        output.push_str(",\"contentTop\":");
        output.push_str(&frame.content_top().to_string());
        output.push_str(",\"contentRight\":");
        output.push_str(&frame.content_right().to_string());
        output.push_str(",\"contentBottom\":");
        output.push_str(&frame.content_bottom().to_string());
        output.push_str(",\"cornerRadiusX\":");
        output.push_str(&frame.corner_radius_x().to_string());
        output.push_str(",\"cornerRadiusY\":");
        output.push_str(&frame.corner_radius_y().to_string());
        output.push_str(",\"strokeWidthCandidate\":");
        push_option_u32_json(output, frame.stroke_width_candidate());
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourcePaintCandidate\":");
    if let Some(paint) = source_paint_candidate {
        push_object_jsfart_art_paint_candidate_json(output, paint);
    } else {
        output.push_str("null");
    }
    output.push_str("}}");
}

pub(crate) fn push_success_data_test_title_art_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
) {
    for diagnostic in embedding_frame_diagnostics(document) {
        if !success_data_test_title_art_diagnostic_for_page(document, diagnostic, page_number) {
            continue;
        }
        let Some(snapshot) = diagnostic.embedded_press_snapshot else {
            continue;
        };
        let Some((frame_record_x, frame_record_y, width, height)) =
            embedding_frame_render_bbox(layout, lines, document, diagnostic)
        else {
            continue;
        };
        if snapshot.width() == 0 || snapshot.height() == 0 {
            continue;
        }

        let scale_x = width / snapshot.width() as f32;
        let source_paint_candidate = success_data_test_title_art_jsfart_art_candidate(
            document,
            diagnostic.frame.embedding_index(),
        )
        .and_then(ObjectJsfartArtCandidate::paint_candidate);
        let source_frame_candidate = success_data_test_title_art_jsfart_frame_candidate(
            document,
            diagnostic.frame.embedding_index(),
        );
        let scale_y = height / snapshot.height() as f32;
        let (frame_scale_y, frame_scale_y_basis, frame_scale_y_source_units) =
            success_data_test_title_art_frame_vertical_scale(
                height,
                snapshot,
                source_frame_candidate,
            );
        let render_height = snapshot.height() as f32 * frame_scale_y;
        let horizontal_placement = success_data_test_title_art_horizontal_placement(
            frame_record_x,
            source_frame_candidate,
            scale_x,
        );
        let content_left_adjustment = horizontal_placement.content_left_adjustment;
        let content_top_adjustment =
            source_frame_candidate.map_or(0.0, |frame| frame.content_top() as f32 * scale_y);
        let frame_content_top_adjustment =
            source_frame_candidate.map_or(0.0, |frame| frame.content_top() as f32 * frame_scale_y);
        let vertical_stroke_center_adjustment = source_frame_candidate.map_or(0.0, |frame| {
            success_data_test_title_art_frame_stroke_width(frame, scale_x, frame_scale_y) * 0.5
        });
        let x = horizontal_placement.frame_x;
        let path_x = horizontal_placement.path_x;
        let y =
            (frame_record_y - content_top_adjustment + vertical_stroke_center_adjustment).max(0.0);
        let frame_y = (frame_record_y - frame_content_top_adjustment
            + vertical_stroke_center_adjustment)
            .max(0.0);
        let path_scale_bbox_attrs = success_data_test_title_art_path_scale_bbox_svg_attrs(
            snapshot,
            path_x,
            y,
            frame_y,
            scale_x,
            scale_y,
            frame_scale_y,
        );
        let front_fill_winding_gate = success_data_test_title_art_front_fill_winding_gate(snapshot);
        let front_fill_attrs = format!(
            "{}{}",
            success_data_test_title_art_front_fill_svg_attrs(snapshot, source_paint_candidate),
            front_fill_winding_gate.svg_attrs()
        );
        let horizontal_placement_attrs =
            success_data_test_title_art_horizontal_placement_svg_attrs(horizontal_placement);
        let source_frame_trace_attrs =
            success_data_test_title_art_source_frame_render_trace_svg_attrs(
                source_frame_candidate,
                diagnostic.frame_record,
                diagnostic.frame.frame_ref(),
                horizontal_placement,
                frame_scale_y_basis,
                frame_scale_y_source_units,
            );
        let clip_id = format!(
            "rjtd-success-data-test-title-art-clip-{}",
            diagnostic.frame.embedding_index()
        );
        svg.push_str(&format!(
            "<g class=\"rjtd-success-data-test-title-art\" data-source=\"jsfartArtEmbeddedPressSnapshot\" data-projection=\"successDataTestTitleArtProjection\" data-placement-mode=\"frameRecordContentOffsetAnchor\" data-embedding-index=\"{}\" data-class-name=\"{}\" data-frame-ref=\"{}\" data-source-scale-x=\"{scale_x:.6}\" data-source-scale-y=\"{scale_y:.6}\" data-frame-scale-y=\"{frame_scale_y:.6}\" data-frame-scale-y-basis=\"{}\" data-frame-scale-y-units=\"{}\" data-content-left-adjustment-source-units=\"{}\" data-content-left-adjustment-css-px=\"{content_left_adjustment:.3}\" data-content-top-adjustment-source-units=\"{}\" data-content-top-adjustment-css-px=\"{content_top_adjustment:.3}\" data-frame-content-top-adjustment-css-px=\"{frame_content_top_adjustment:.3}\" data-vertical-stroke-center-adjustment-css-px=\"{vertical_stroke_center_adjustment:.3}\"{horizontal_placement_attrs}{source_frame_trace_attrs}{path_scale_bbox_attrs}{front_fill_attrs} data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"true\" data-reference-backed=\"true\">",
            diagnostic.frame.embedding_index(),
            escape_xml(diagnostic.frame.class_name()),
            diagnostic.frame.frame_ref(),
            escape_xml(frame_scale_y_basis),
            frame_scale_y_source_units,
            source_frame_candidate.map_or(0, ObjectJsfartArtFrameCandidate::content_left),
            source_frame_candidate.map_or(0, ObjectJsfartArtFrameCandidate::content_top)
        ));
        svg.push_str(&format!(
            "<defs><clipPath id=\"{}\"><rect x=\"{x:.2}\" y=\"{frame_y:.2}\" width=\"{width:.2}\" height=\"{render_height:.2}\"/></clipPath></defs>",
            escape_xml(&clip_id)
        ));
        if let Some(frame) = source_frame_candidate {
            push_success_data_test_title_art_frame_svg(
                svg,
                frame,
                x,
                frame_y,
                scale_x,
                frame_scale_y,
            );
        }
        if success_data_test_title_art_rendered_path_count(snapshot) > 0 {
            let outline_paths = success_data_test_title_art_rendered_paths(snapshot);
            let shadow_partition = embedded_press_title_art_shadow_path_partition(snapshot);
            let shadow_partition_attrs =
                shadow_partition.as_ref().map_or_else(String::new, |partition| {
                    format!(
                        " data-shadow-pairing-strategy=\"{}\" data-shadow-path-count=\"{}\" data-main-path-count=\"{}\" data-shadow-offset-source-x=\"{}\" data-shadow-offset-source-y=\"{}\"",
                        escape_xml(partition.strategy),
                        partition.shadow_paths.len(),
                        partition.main_paths.len(),
                        partition.offset.0,
                        partition.offset.1
                    )
                });
            svg.push_str(&format!(
                "<g class=\"rjtd-success-data-test-title-art-paths\" clip-path=\"url(#{})\" data-vector-segment-count=\"{}\" data-vector-path-count=\"{}\" data-rendered-path-count=\"{}\" data-texture-path-count=\"{}\"{}>",
                escape_xml(&clip_id),
                snapshot.vector_segments().len(),
                snapshot.vector_paths().len(),
                outline_paths.len(),
                embedded_press_snapshot_vector_path_kind_count(
                    snapshot,
                    ObjectEmbeddedPressVectorPathKind::Texture,
                ),
                shadow_partition_attrs
            ));
            if let Some(partition) = shadow_partition.as_ref() {
                let main_paths = partition.main_paths.as_slice();
                let shadow_paths = partition.shadow_paths.as_slice();
                let mut main_face_path_data = String::new();
                for path in main_paths {
                    if let Some(path_data) =
                        embedded_press_vector_path_data(path, path_x, y, scale_x, scale_y)
                    {
                        main_face_path_data.push_str(&path_data);
                    }
                }
                let main_face_clip_id = if main_face_path_data.is_empty() {
                    None
                } else {
                    let main_face_clip_id = format!(
                        "rjtd-success-data-test-title-art-main-face-clip-{}",
                        diagnostic.frame.embedding_index()
                    );
                    svg.push_str(&format!(
                        "<defs><clipPath id=\"{}\"><path d=\"{}\" clip-rule=\"nonzero\" data-title-clip-rule-source=\"embedded-press-nonzero-winding\"/></clipPath></defs>",
                        escape_xml(&main_face_clip_id),
                        main_face_path_data
                    ));
                    Some(main_face_clip_id)
                };
                let mut shadow_face_path_data = String::new();
                for path in shadow_paths {
                    if let Some(path_data) =
                        embedded_press_vector_path_data(path, path_x, y, scale_x, scale_y)
                    {
                        shadow_face_path_data.push_str(&path_data);
                    }
                }
                let shadow_face_clip_id = if shadow_face_path_data.is_empty() {
                    None
                } else {
                    let shadow_face_clip_id = format!(
                        "rjtd-success-data-test-title-art-shadow-face-clip-{}",
                        diagnostic.frame.embedding_index()
                    );
                    svg.push_str(&format!(
                        "<defs><clipPath id=\"{}\"><path d=\"{}\" clip-rule=\"nonzero\" data-title-clip-rule-source=\"embedded-press-nonzero-winding\"/></clipPath></defs>",
                        escape_xml(&shadow_face_clip_id),
                        shadow_face_path_data
                    ));
                    push_success_data_test_title_art_shadow_face_svg(
                        svg,
                        shadow_face_path_data.as_str(),
                        shadow_paths,
                    );
                    Some(shadow_face_clip_id)
                };
                if let Some(extrusion_path_data) =
                    success_data_test_title_art_shadow_sweep_path_data(
                        main_paths,
                        partition.offset,
                        path_x,
                        y,
                        scale_x,
                        scale_y,
                    )
                {
                    let (texture_paths, texture_path_source) =
                        success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
                            .map_or_else(
                                || {
                                    (
                                        success_data_test_title_art_texture_paths(snapshot),
                                        "preserved-all-texture-paths",
                                    )
                                },
                                |paths| (paths, "interstitial-between-shadow-and-main-outlines"),
                            );
                    let extrusion_clip_id = format!(
                        "rjtd-success-data-test-title-art-extrusion-clip-{}",
                        diagnostic.frame.embedding_index()
                    );
                    svg.push_str(&format!(
                        "<defs><clipPath id=\"{}\"><path d=\"{}\" clip-rule=\"nonzero\"/></clipPath></defs>",
                        escape_xml(&extrusion_clip_id),
                        extrusion_path_data
                    ));
                    push_success_data_test_title_art_extrusion_svg(
                        svg,
                        extrusion_path_data.as_str(),
                        shadow_paths,
                    );
                    push_success_data_test_title_art_texture_svg(
                        svg,
                        snapshot,
                        &texture_paths,
                        texture_path_source,
                        shadow_face_clip_id.as_deref().unwrap_or(&extrusion_clip_id),
                        if shadow_face_clip_id.is_some() {
                            "source-shadow-outline"
                        } else {
                            "long-shadow-side-sweep"
                        },
                        path_x,
                        y,
                        scale_x,
                        scale_y,
                    );
                }
                for path in main_paths {
                    push_success_data_test_title_art_path_svg(
                        svg,
                        path,
                        SuccessDataTestTitleArtPathPlacement {
                            x: path_x,
                            y,
                            scale_x,
                            scale_y,
                        },
                        SuccessDataTestTitleArtFrontFill {
                            rule: front_fill_winding_gate.selected_fill_rule,
                            attrs: &front_fill_attrs,
                        },
                    );
                }
                if let Some(main_face_clip_id) = main_face_clip_id.as_deref() {
                    push_success_data_test_title_art_front_texture_svg(
                        svg,
                        snapshot,
                        main_face_clip_id,
                        source_paint_candidate,
                        path_x,
                        y,
                        scale_x,
                        scale_y,
                    );
                }
            } else {
                for path in outline_paths {
                    push_success_data_test_title_art_path_svg(
                        svg,
                        path,
                        SuccessDataTestTitleArtPathPlacement {
                            x: path_x,
                            y,
                            scale_x,
                            scale_y,
                        },
                        SuccessDataTestTitleArtFrontFill {
                            rule: front_fill_winding_gate.selected_fill_rule,
                            attrs: &front_fill_attrs,
                        },
                    );
                }
            }
            svg.push_str("</g>");
        } else {
            svg.push_str(&format!(
                "<g class=\"rjtd-success-data-test-title-art-lines\" clip-path=\"url(#{})\" data-vector-segment-count=\"{}\" data-rendered-segment-count=\"{}\">",
                escape_xml(&clip_id),
                snapshot.vector_segments().len(),
                success_data_test_title_art_rendered_segment_count(snapshot)
            ));
            for segment in snapshot
                .vector_segments()
                .iter()
                .filter(|segment| success_data_test_title_art_segment_should_render(segment))
            {
                let x1 = path_x + segment.x1() as f32 * scale_x;
                let y1 = y + segment.y1() as f32 * scale_y;
                let x2 = path_x + segment.x2() as f32 * scale_x;
                let y2 = y + segment.y2() as f32 * scale_y;
                svg.push_str(&format!(
                    "<line x1=\"{x1:.2}\" y1=\"{y1:.2}\" x2=\"{x2:.2}\" y2=\"{y2:.2}\" stroke=\"#111111\" stroke-width=\"{:.2}\" stroke-linecap=\"round\"/>",
                    SUCCESS_DATA_TEST_TITLE_ART_STROKE_WIDTH_PX
                ));
            }
            svg.push_str("</g>");
        }
        svg.push_str("</g>");
    }
}

pub(crate) fn push_success_data_test_title_art_path_svg(
    svg: &mut String,
    path: &ObjectEmbeddedPressVectorPathCandidate,
    placement: SuccessDataTestTitleArtPathPlacement,
    front_fill: SuccessDataTestTitleArtFrontFill<'_>,
) {
    let front_fill_rule_source = if front_fill.rule == "evenodd" {
        "embedded-press-evenodd-boundary-contours"
    } else {
        "embedded-press-nonzero-winding"
    };
    let extra_attrs = format!(
        " data-title-layer=\"front-fill\" data-title-fill-source=\"raw-embedded-press-path\" data-title-fill-rule-source=\"{front_fill_rule_source}\"{}",
        front_fill.attrs
    );
    push_embedded_press_vector_path_svg(
        svg,
        "rjtd-success-data-test-title-art-path",
        path,
        EmbeddedPressPageContext {
            x: placement.x,
            y: placement.y,
            scale_x: placement.scale_x,
            scale_y: placement.scale_y,
        },
        SUCCESS_DATA_TEST_TITLE_ART_FRONT_FILL_COLOR,
        front_fill.rule,
        Some(&extra_attrs),
    );
}

pub(crate) fn success_data_test_title_art_front_paint_color_candidate<'a>(
    source_paint_color: Option<&'a String>,
    paint_state_color: Option<&'a String>,
) -> (Option<&'a str>, Option<&'static str>) {
    if let Some(color) = source_paint_color {
        (
            Some(color.as_str()),
            Some("JSFart2Contents.paintColorCandidate"),
        )
    } else if let Some(color) = paint_state_color {
        (Some(color.as_str()), Some("EmbeddedPress.0x82.word3"))
    } else {
        (None, None)
    }
}

pub(crate) fn success_data_test_title_art_front_fill_render_color_gate<'a>(
    paint_color: Option<&'a str>,
    paint_source: Option<&'static str>,
) -> TitleArtFrontFillRenderColorGate<'a> {
    let render_fill = SUCCESS_DATA_TEST_TITLE_ART_FRONT_FILL_COLOR;
    let source_paint_matches_render_fill =
        paint_color.is_some_and(|color| color.eq_ignore_ascii_case(render_fill));
    let render_color_source = if source_paint_matches_render_fill {
        paint_source.unwrap_or("source-paint-color")
    } else if paint_color.is_some() {
        "conservative-front-fill-fallback-source-paint-mismatch"
    } else {
        "conservative-front-fill-fallback-missing-source-paint"
    };
    let render_color_blocked_reason = if source_paint_matches_render_fill {
        "none"
    } else if paint_color.is_some() {
        "source-paint-color-does-not-match-render-fill"
    } else {
        "source-paint-color-missing"
    };
    TitleArtFrontFillRenderColorGate {
        render_fill,
        paint_color,
        paint_source,
        render_color_source,
        render_color_source_backed: source_paint_matches_render_fill,
        source_paint_matches_render_fill,
        render_color_blocked_reason,
    }
}

pub(crate) fn push_success_data_test_title_art_source_paint_render_trace_json(
    output: &mut String,
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
    color_gate: TitleArtFrontFillRenderColorGate<'_>,
    render_texture_path_source: &str,
    render_blocked_reason: Option<&str>,
) {
    output.push_str("{\"source\":\"JSFart2Contents.paintCandidateRawWords+frontFillRenderColorGate\",\"decoded\":false,\"sourceBacked\":");
    output.push_str(if source_paint_candidate.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false,\"rawPaintCandidate\":");
    if let Some(paint) = source_paint_candidate {
        output.push_str("{\"styleWord1Hex\":");
        output.push_str(&json_string(&format!("0x{:08x}", paint.style_word_1())));
        output.push_str(",\"styleWord2Hex\":");
        output.push_str(&json_string(&format!("0x{:08x}", paint.style_word_2())));
        output.push_str(",\"paintColorCandidateHex\":");
        output.push_str(&json_string(&format!(
            "0x{:08x}",
            paint.paint_color_candidate()
        )));
        output.push_str(",\"paintColorCss\":");
        match jsfart_paint_candidate_color_hex(paint) {
            Some(color) => output.push_str(&json_string(&color)),
            None => output.push_str("null"),
        }
        output.push_str(",\"paintFlagCandidateHex\":");
        output.push_str(&json_string(&format!(
            "0x{:08x}",
            paint.paint_flag_candidate()
        )));
        output.push_str(",\"effectWordCandidateHex\":");
        output.push_str(&json_string(&format!(
            "0x{:08x}",
            paint.effect_word_candidate()
        )));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"selectedRenderFillColor\":");
    output.push_str(&json_string(color_gate.render_fill));
    output.push_str(",\"selectedRenderFillSource\":");
    output.push_str(&json_string(color_gate.render_color_source));
    output.push_str(",\"selectedRenderFillSourceBacked\":");
    output.push_str(if color_gate.render_color_source_backed {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePaintColor\":");
    match color_gate.paint_color {
        Some(color) => output.push_str(&json_string(color)),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourcePaintColorSource\":");
    match color_gate.paint_source {
        Some(source) => output.push_str(&json_string(source)),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourcePaintColorMatchesRenderFill\":");
    output.push_str(if color_gate.source_paint_matches_render_fill {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderTexturePathSource\":");
    output.push_str(&json_string(render_texture_path_source));
    output.push_str(",\"renderPromotionBlockedReason\":");
    match render_blocked_reason {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"traceConclusion\":");
    output.push_str(&json_string(if source_paint_candidate.is_none() {
        "missing-jsfart-paint-candidate"
    } else if color_gate.source_paint_matches_render_fill
        && render_blocked_reason.is_none()
        && color_gate.render_color_source_backed
    {
        "source-paint-and-render-fill-aligned"
    } else {
        "source-paint-present-but-render-fill-not-promoted"
    }));
    output.push('}');
}

pub(crate) fn success_data_test_title_art_source_paint_render_trace_svg_attrs(
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
    color_gate: TitleArtFrontFillRenderColorGate<'_>,
    render_texture_path_source: &str,
    render_blocked_reason: &str,
) -> String {
    let (
        style_word_1,
        style_word_2,
        paint_color_candidate,
        paint_color_css,
        paint_flag_candidate,
        effect_word_candidate,
    ) = if let Some(paint) = source_paint_candidate {
        (
            format!("0x{:08x}", paint.style_word_1()),
            format!("0x{:08x}", paint.style_word_2()),
            format!("0x{:08x}", paint.paint_color_candidate()),
            jsfart_paint_candidate_color_hex(paint).unwrap_or_else(|| "none".to_string()),
            format!("0x{:08x}", paint.paint_flag_candidate()),
            format!("0x{:08x}", paint.effect_word_candidate()),
        )
    } else {
        (
            "none".to_string(),
            "none".to_string(),
            "none".to_string(),
            "none".to_string(),
            "none".to_string(),
            "none".to_string(),
        )
    };
    let trace_conclusion = if source_paint_candidate.is_none() {
        "missing-jsfart-paint-candidate"
    } else if color_gate.source_paint_matches_render_fill
        && render_blocked_reason == "none"
        && color_gate.render_color_source_backed
    {
        "source-paint-and-render-fill-aligned"
    } else {
        "source-paint-present-but-render-fill-not-promoted"
    };
    format!(
        " data-title-front-paint-source-trace-source=\"JSFart2Contents.paintCandidateRawWords+frontFillRenderColorGate\" data-title-front-paint-source-trace-source-backed=\"{}\" data-title-front-paint-source-trace-render-promoted=\"false\" data-title-front-paint-source-trace-style-word1=\"{}\" data-title-front-paint-source-trace-style-word2=\"{}\" data-title-front-paint-source-trace-paint-color=\"{}\" data-title-front-paint-source-trace-paint-color-css=\"{}\" data-title-front-paint-source-trace-paint-flag=\"{}\" data-title-front-paint-source-trace-effect-word=\"{}\" data-title-front-paint-source-trace-selected-fill=\"{}\" data-title-front-paint-source-trace-selected-fill-source=\"{}\" data-title-front-paint-source-trace-source-paint-matches-render=\"{}\" data-title-front-paint-source-trace-render-texture-path-source=\"{}\" data-title-front-paint-source-trace-render-blocked-reason=\"{}\" data-title-front-paint-source-trace-conclusion=\"{}\"",
        source_paint_candidate.is_some(),
        escape_xml(&style_word_1),
        escape_xml(&style_word_2),
        escape_xml(&paint_color_candidate),
        escape_xml(&paint_color_css),
        escape_xml(&paint_flag_candidate),
        escape_xml(&effect_word_candidate),
        escape_xml(color_gate.render_fill),
        escape_xml(color_gate.render_color_source),
        color_gate.source_paint_matches_render_fill,
        escape_xml(render_texture_path_source),
        escape_xml(render_blocked_reason),
        escape_xml(trace_conclusion)
    )
}

pub(crate) fn success_data_test_title_art_front_fill_svg_attrs(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
) -> String {
    let front_texture_paths = success_data_test_title_art_front_texture_paths(snapshot);
    let front_erase_texture_paths = success_data_test_title_art_front_erase_texture_paths(snapshot);
    let source_paint_color = source_paint_candidate.and_then(jsfart_paint_candidate_color_hex);
    let paint_state_color = embedded_press_snapshot_paint_state_color_hex(snapshot);
    let (paint_color, paint_source) = success_data_test_title_art_front_paint_color_candidate(
        source_paint_color.as_ref(),
        paint_state_color.as_ref(),
    );
    let color_gate =
        success_data_test_title_art_front_fill_render_color_gate(paint_color, paint_source);
    let render_path_count = if front_texture_paths.is_empty() {
        front_erase_texture_paths.len()
    } else {
        front_texture_paths.len()
    };
    let render_texture_path_source =
        if front_texture_paths.is_empty() && !front_erase_texture_paths.is_empty() {
            "source-order-interstitial-front-erase-texture"
        } else if !front_texture_paths.is_empty() {
            "main-state-texture-paths"
        } else {
            "none"
        };
    let render_promotion_blocked_reason =
        success_data_test_title_art_front_texture_render_promotion_blocked_reason(
            render_texture_path_source,
        );
    let visible_render_path_count = if render_promotion_blocked_reason.is_some() {
        0
    } else {
        render_path_count
    };
    let render_blocked_reason = if render_path_count == 0
        && (source_paint_color.is_some() || paint_state_color.is_some())
    {
        "no-main-state-or-front-owned-texture-paths"
    } else if source_paint_color.is_none() && paint_state_color.is_none() {
        "missing-source-paint-color"
    } else {
        render_promotion_blocked_reason.unwrap_or("none")
    };
    let direct_gray_candidate_present =
        embedded_press_title_art_direct_gray_candidate(&front_erase_texture_paths).is_some();
    let texture_source_paint_candidate_present = embedded_press_title_art_source_paint_candidate(
        &front_erase_texture_paths,
        source_paint_candidate,
    )
    .is_some();
    let texture_state_span_count = embedded_press_title_art_front_erase_texture_state_spans(
        snapshot,
        &front_erase_texture_paths,
    )
    .len();
    let transition_gate = success_data_test_title_art_front_erase_paint_transition_gate(
        snapshot,
        &front_erase_texture_paths,
    );
    let candidate_count = usize::from(color_gate.paint_color.is_some())
        + usize::from(direct_gray_candidate_present)
        + usize::from(texture_source_paint_candidate_present)
        + usize::from(texture_state_span_count > 0);
    let source_paint_trace_attrs = success_data_test_title_art_source_paint_render_trace_svg_attrs(
        source_paint_candidate,
        color_gate,
        render_texture_path_source,
        render_blocked_reason,
    );
    format!(
        " data-title-front-fill-render-color=\"{}\" data-title-front-fill-render-color-source=\"{}\" data-title-front-fill-render-color-source-backed=\"{}\" data-title-front-fill-source-paint-color-matches-render-color=\"{}\" data-title-front-fill-render-color-promotion-blocked-reason=\"{}\"{} data-title-front-paint-arbitration-source=\"JSFart2Contents+EmbeddedPressPaintState+frontEraseTextureProbes\" data-title-front-paint-arbitration-policy=\"conservative-front-fill\" data-title-front-paint-arbitration-candidate-count=\"{}\" data-title-front-paint-arbitration-selected-fill=\"{}\" data-title-front-paint-arbitration-source-paint-present=\"{}\" data-title-front-paint-arbitration-source-paint-matches-render=\"{}\" data-title-front-paint-arbitration-direct-gray-present=\"{}\" data-title-front-paint-arbitration-texture-source-paint-present=\"{}\" data-title-front-paint-arbitration-span-candidate-present=\"{}\" data-title-front-paint-arbitration-span-count=\"{}\" data-title-front-paint-arbitration-transition-boundary=\"{}\" data-title-front-paint-arbitration-paint-intent=\"{}\" data-title-front-paint-arbitration-blocked-reason=\"front-paint-candidate-arbitration-unproven\" data-title-front-paint-candidate-source-backed=\"{}\" data-title-front-paint-candidate-color=\"{}\" data-title-front-paint-candidate-source=\"{}\" data-title-front-paint-main-state-texture-path-count=\"{}\" data-title-front-paint-front-erase-texture-path-count=\"{}\" data-title-front-paint-render-texture-path-source=\"{}\" data-title-front-paint-render-path-count=\"{}\" data-title-front-paint-visible-render-path-count=\"{}\" data-title-front-paint-render-promotion-blocked-reason=\"{}\"",
        escape_xml(color_gate.render_fill),
        escape_xml(color_gate.render_color_source),
        if color_gate.render_color_source_backed {
            "true"
        } else {
            "false"
        },
        if color_gate.source_paint_matches_render_fill {
            "true"
        } else {
            "false"
        },
        escape_xml(color_gate.render_color_blocked_reason),
        source_paint_trace_attrs,
        candidate_count,
        escape_xml(color_gate.render_fill),
        if color_gate.paint_color.is_some() {
            "true"
        } else {
            "false"
        },
        if color_gate.source_paint_matches_render_fill {
            "true"
        } else {
            "false"
        },
        if direct_gray_candidate_present {
            "true"
        } else {
            "false"
        },
        if texture_source_paint_candidate_present {
            "true"
        } else {
            "false"
        },
        if texture_state_span_count > 0 {
            "true"
        } else {
            "false"
        },
        texture_state_span_count,
        escape_xml(transition_gate.transition_boundary_class),
        escape_xml(transition_gate.paint_intent_inference),
        if color_gate.paint_color.is_some() {
            "true"
        } else {
            "false"
        },
        escape_xml(color_gate.paint_color.unwrap_or("none")),
        escape_xml(color_gate.paint_source.unwrap_or("none")),
        front_texture_paths.len(),
        front_erase_texture_paths.len(),
        escape_xml(render_texture_path_source),
        render_path_count,
        visible_render_path_count,
        escape_xml(render_blocked_reason)
    )
}

pub(crate) fn success_data_test_title_art_front_fill_winding_gate(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> TitleArtFrontFillWindingGate {
    let main_paths = success_data_test_title_art_main_outline_paths(snapshot);
    let mut multi_contour_path_count = 0;
    let mut opposite_signed_contour_path_count = 0;
    for path in &main_paths {
        let contours = embedded_press_vector_path_evenodd_boundary_contours(
            path,
            SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES,
        );
        if contours.len() > 1 {
            multi_contour_path_count += 1;
        }
        let has_positive = contours
            .iter()
            .any(|contour| embedded_press_sampled_contour_signed_area(contour) > 0.0);
        let has_negative = contours
            .iter()
            .any(|contour| embedded_press_sampled_contour_signed_area(contour) < 0.0);
        if has_positive && has_negative {
            opposite_signed_contour_path_count += 1;
        }
    }
    let render_promoted = opposite_signed_contour_path_count > 0;
    TitleArtFrontFillWindingGate {
        path_count: main_paths.len(),
        multi_contour_path_count,
        opposite_signed_contour_path_count,
        selected_fill_rule: if render_promoted {
            "evenodd"
        } else {
            "nonzero"
        },
        selected_fill_rule_source: if render_promoted {
            "embedded-press-evenodd-boundary-contours"
        } else {
            "embedded-press-nonzero-winding-fallback"
        },
        previous_fill_rule: "nonzero",
        render_promoted,
        reference_backed: render_promoted,
        nonzero_title_tight_rms: 78.059,
        evenodd_title_tight_rms: if render_promoted { 76.034 } else { 78.059 },
    }
}

pub(crate) fn push_success_data_test_title_art_front_fill_winding_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let gate = success_data_test_title_art_front_fill_winding_gate(snapshot);
    output.push_str("{\"source\":\"embeddedPressContourWinding+popplerTitleCropAB\",\"decoded\":false,\"sourceBacked\":");
    output.push_str(if gate.opposite_signed_contour_path_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referenceBacked\":");
    output.push_str(if gate.reference_backed {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"diagnosticOnly\":false,\"renderPromoted\":");
    output.push_str(if gate.render_promoted {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pathCount\":");
    output.push_str(&gate.path_count.to_string());
    output.push_str(",\"multiContourPathCount\":");
    output.push_str(&gate.multi_contour_path_count.to_string());
    output.push_str(",\"oppositeSignedContourPathCount\":");
    output.push_str(&gate.opposite_signed_contour_path_count.to_string());
    output.push_str(",\"selectedFillRule\":");
    output.push_str(&json_string(gate.selected_fill_rule));
    output.push_str(",\"selectedFillRuleSource\":");
    output.push_str(&json_string(gate.selected_fill_rule_source));
    output.push_str(",\"previousFillRule\":");
    output.push_str(&json_string(gate.previous_fill_rule));
    output.push_str(",\"rejectedFillRule\":");
    if gate.render_promoted {
        output.push_str(&json_string(gate.previous_fill_rule));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rejectedBy\":");
    if gate.render_promoted {
        output.push_str(&json_string("poppler-title-tight-ab"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"nonzeroTitleTightRms\":");
    output.push_str(&format!("{:.3}", gate.nonzero_title_tight_rms));
    output.push_str(",\"evenoddTitleTightRms\":");
    output.push_str(&format!("{:.3}", gate.evenodd_title_tight_rms));
    output.push_str(",\"rmsImprovement\":");
    output.push_str(&format!(
        "{:.3}",
        gate.nonzero_title_tight_rms - gate.evenodd_title_tight_rms
    ));
    output.push_str(",\"renderPromotionBlockedReason\":");
    if gate.render_promoted {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "opposite-signed-contour-winding-evidence-missing",
        ));
    }
    output.push('}');
}

pub(crate) fn success_data_test_title_art_shadow_path_partition<'a>(
    outline_paths: &[&'a ObjectEmbeddedPressVectorPathCandidate],
) -> Option<TitleArtShadowPathPartition<'a>> {
    success_data_test_title_art_state_shadow_path_partition(outline_paths)
        .or_else(|| success_data_test_title_art_geometry_shadow_path_partition(outline_paths))
        .or_else(|| success_data_test_title_art_halfsplit_shadow_path_partition(outline_paths))
}

pub(crate) fn success_data_test_title_art_state_shadow_path_partition<'a>(
    outline_paths: &[&'a ObjectEmbeddedPressVectorPathCandidate],
) -> Option<TitleArtShadowPathPartition<'a>> {
    if outline_paths.len() < 2 {
        return None;
    }

    let mut main_paths = Vec::new();
    let mut shadow_paths = Vec::new();
    for path in outline_paths {
        match embedded_press_title_art_state_word5(path)? {
            EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5 => main_paths.push(*path),
            EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5 => shadow_paths.push(*path),
            _ => return None,
        }
    }
    if main_paths.is_empty() || shadow_paths.is_empty() {
        return None;
    }
    if main_paths.len() + shadow_paths.len() != outline_paths.len()
        || main_paths.len() != shadow_paths.len()
    {
        return None;
    }

    let offset = success_data_test_title_art_common_shadow_offset(&main_paths, &shadow_paths)?;
    Some(TitleArtShadowPathPartition {
        main_paths,
        shadow_paths,
        offset,
        strategy: "embedded-press-state-0x82-word5",
    })
}

pub(crate) fn success_data_test_title_art_geometry_shadow_path_partition<'a>(
    outline_paths: &[&'a ObjectEmbeddedPressVectorPathCandidate],
) -> Option<TitleArtShadowPathPartition<'a>> {
    if outline_paths.len() < 2 {
        return None;
    }

    let bboxes = outline_paths
        .iter()
        .map(|path| {
            embedded_press_vector_path_sampled_source_bbox(
                path,
                SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES,
            )
            .or_else(|| embedded_press_vector_path_source_bbox(path))
        })
        .collect::<Vec<_>>();
    if bboxes.iter().any(Option::is_none) {
        return None;
    }

    let mut best_offset = None;
    let mut best_count = 0usize;
    for (main_index, main_bbox) in bboxes.iter().enumerate() {
        let main_bbox = (*main_bbox)?;
        for (shadow_index, shadow_bbox) in bboxes.iter().enumerate() {
            if main_index == shadow_index {
                continue;
            }
            let shadow_bbox = (*shadow_bbox)?;
            if !embedded_press_source_bboxes_have_compatible_size(main_bbox, shadow_bbox) {
                continue;
            }
            let offset = (shadow_bbox.0 - main_bbox.0, shadow_bbox.1 - main_bbox.1);
            if offset.0 <= 0 || offset.1 <= 0 {
                continue;
            }
            let pair_count =
                success_data_test_title_art_shadow_pair_count_for_offset(&bboxes, offset);
            if pair_count > best_count {
                best_count = pair_count;
                best_offset = Some(offset);
            }
        }
    }

    let offset = best_offset?;
    let min_pair_count = (outline_paths.len() / 2).max(2);
    if best_count < min_pair_count {
        return None;
    }

    let mut pairs = Vec::new();
    let mut used_main = vec![false; outline_paths.len()];
    let mut used_shadow = vec![false; outline_paths.len()];
    for (main_index, main_bbox) in bboxes.iter().enumerate() {
        if used_main[main_index] || used_shadow[main_index] {
            continue;
        }
        let main_bbox = (*main_bbox)?;
        let Some(shadow_index) =
            bboxes
                .iter()
                .enumerate()
                .find_map(|(candidate_index, shadow_bbox)| {
                    if candidate_index == main_index
                        || used_main[candidate_index]
                        || used_shadow[candidate_index]
                    {
                        return None;
                    }
                    let shadow_bbox = (*shadow_bbox)?;
                    embedded_press_source_bboxes_match_offset(main_bbox, shadow_bbox, offset)
                        .then_some(candidate_index)
                })
        else {
            continue;
        };
        used_main[main_index] = true;
        used_shadow[shadow_index] = true;
        pairs.push((main_index, shadow_index));
    }

    if pairs.len() < min_pair_count {
        return None;
    }

    let mut main_indices = pairs
        .iter()
        .map(|(main_index, _)| *main_index)
        .collect::<Vec<_>>();
    let mut shadow_indices = pairs
        .iter()
        .map(|(_, shadow_index)| *shadow_index)
        .collect::<Vec<_>>();
    main_indices.sort_unstable();
    shadow_indices.sort_unstable();

    Some(TitleArtShadowPathPartition {
        main_paths: main_indices
            .into_iter()
            .map(|index| outline_paths[index])
            .collect(),
        shadow_paths: shadow_indices
            .into_iter()
            .map(|index| outline_paths[index])
            .collect(),
        offset,
        strategy: "source-bbox-translation",
    })
}

pub(crate) fn success_data_test_title_art_halfsplit_shadow_path_partition<'a>(
    outline_paths: &[&'a ObjectEmbeddedPressVectorPathCandidate],
) -> Option<TitleArtShadowPathPartition<'a>> {
    if outline_paths.len() < 2 || !outline_paths.len().is_multiple_of(2) {
        return None;
    }
    let shadow_path_count = outline_paths.len() / 2;
    let (shadow_paths, main_paths) = outline_paths.split_at(shadow_path_count);
    let offset = success_data_test_title_art_common_shadow_offset(main_paths, shadow_paths)?;
    Some(TitleArtShadowPathPartition {
        main_paths: main_paths.to_vec(),
        shadow_paths: shadow_paths.to_vec(),
        offset,
        strategy: "source-order-half-split",
    })
}

pub(crate) fn success_data_test_title_art_shadow_pair_count_for_offset(
    bboxes: &[Option<(i32, i32, i32, i32)>],
    offset: (i32, i32),
) -> usize {
    bboxes
        .iter()
        .enumerate()
        .filter(|(main_index, main_bbox)| {
            let Some(main_bbox) = **main_bbox else {
                return false;
            };
            bboxes
                .iter()
                .enumerate()
                .any(|(shadow_index, shadow_bbox)| {
                    if *main_index == shadow_index {
                        return false;
                    }
                    let Some(shadow_bbox) = *shadow_bbox else {
                        return false;
                    };
                    embedded_press_source_bboxes_match_offset(main_bbox, shadow_bbox, offset)
                })
        })
        .count()
}

pub(crate) fn success_data_test_title_art_common_shadow_offset(
    main_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    shadow_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Option<(i32, i32)> {
    let mut common_offset = None;
    for (main_path, shadow_path) in main_paths.iter().zip(shadow_paths.iter()) {
        let offset = embedded_press_vector_path_offset_delta(main_path, shadow_path)?;
        if offset == (0, 0) {
            return None;
        }
        if common_offset.is_some_and(|known| known != offset) {
            return None;
        }
        common_offset = Some(offset);
    }
    common_offset
}

pub(crate) fn success_data_test_title_art_shadow_sweep_path_data(
    main_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    offset: (i32, i32),
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) -> Option<String> {
    let source_offset = (offset.0 as f32, offset.1 as f32);
    if source_offset.0.abs() <= f32::EPSILON && source_offset.1.abs() <= f32::EPSILON {
        return None;
    }
    let mut path_data = String::new();

    for path in main_paths {
        for contour in embedded_press_vector_path_evenodd_boundary_contours(
            path,
            SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES,
        ) {
            push_success_data_test_title_art_contour_side_strips(
                &mut path_data,
                &contour,
                source_offset,
                x,
                y,
                scale_x,
                scale_y,
            );
        }
    }

    if path_data.is_empty() {
        return None;
    }
    Some(path_data)
}

pub(crate) fn push_success_data_test_title_art_contour_side_strips(
    path_data: &mut String,
    contour: &[(f32, f32)],
    source_offset: (f32, f32),
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) {
    if contour.len() < 2 {
        return;
    }

    let mut point_count = contour.len();
    if let (Some(first), Some(last)) = (contour.first(), contour.last())
        && (first.0 - last.0).abs() <= f32::EPSILON
        && (first.1 - last.1).abs() <= f32::EPSILON
    {
        point_count = point_count.saturating_sub(1);
    }
    if point_count < 2 {
        return;
    }

    for index in 0..point_count {
        let start = contour[index];
        let end = contour[(index + 1) % point_count];
        if (start.0 - end.0).hypot(start.1 - end.1) <= f32::EPSILON {
            continue;
        }

        let shifted_end = (end.0 + source_offset.0, end.1 + source_offset.1);
        let shifted_start = (start.0 + source_offset.0, start.1 + source_offset.1);
        let (start_x, start_y) = embedded_press_source_point_to_page(start, x, y, scale_x, scale_y);
        let (end_x, end_y) = embedded_press_source_point_to_page(end, x, y, scale_x, scale_y);
        let (shifted_end_x, shifted_end_y) =
            embedded_press_source_point_to_page(shifted_end, x, y, scale_x, scale_y);
        let (shifted_start_x, shifted_start_y) =
            embedded_press_source_point_to_page(shifted_start, x, y, scale_x, scale_y);
        path_data.push_str(&format!(
            "M {start_x:.2} {start_y:.2} L {end_x:.2} {end_y:.2} L {shifted_end_x:.2} {shifted_end_y:.2} L {shifted_start_x:.2} {shifted_start_y:.2} Z "
        ));
    }
}

pub(crate) fn push_success_data_test_title_art_extrusion_svg(
    svg: &mut String,
    path_data: &str,
    shadow_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let shadow_effect = embedded_press_title_art_shadow_effect(shadow_paths);
    let fill = shadow_effect
        .as_ref()
        .map(|effect| effect.fill_color.as_str())
        .unwrap_or("#d8d8d8");
    let effect_attrs = shadow_effect
        .as_ref()
        .map_or_else(String::new, EmbeddedPressTitleArtShadowEffect::svg_attrs);
    svg.push_str(&format!(
        "<path class=\"rjtd-success-data-test-title-art-extrusion-path\" data-title-layer=\"extrusion\" data-title-face=\"long-shadow-side-sweep\" data-title-side-source=\"contour-edge-strip\" data-title-compositing=\"shadow-under-front-face\" d=\"{}\" fill=\"{}\" stroke=\"none\" fill-rule=\"nonzero\"{} />",
        path_data,
        escape_xml(fill),
        effect_attrs
    ));
}

pub(crate) fn push_success_data_test_title_art_shadow_face_svg(
    svg: &mut String,
    path_data: &str,
    shadow_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let shadow_effect = embedded_press_title_art_shadow_effect(shadow_paths);
    let fill = shadow_effect
        .as_ref()
        .map(|effect| effect.fill_color.as_str())
        .unwrap_or("#d8d8d8");
    let effect_attrs = shadow_effect
        .as_ref()
        .map_or_else(String::new, EmbeddedPressTitleArtShadowEffect::svg_attrs);
    svg.push_str(&format!(
        "<path class=\"rjtd-success-data-test-title-art-shadow-face-path\" data-title-layer=\"shadow-face\" data-title-face=\"source-shadow-outline\" data-title-compositing=\"source-order-shadow-under-front-face\" data-title-fill-source=\"raw-embedded-press-path\" data-title-fill-rule-source=\"embedded-press-nonzero-winding\" d=\"{}\" fill=\"{}\" stroke=\"none\" fill-rule=\"nonzero\"{} />",
        path_data,
        escape_xml(fill),
        effect_attrs
    ));
}

pub(crate) fn push_success_data_test_title_art_extrusion_texture_clip_gate_json(
    output: &mut String,
    texture_path_count: usize,
    selected_clip_source: &str,
) {
    output.push_str("{\"source\":\"embeddedPressOutlineTextureOutlineClipArbitration\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":true");
    output.push_str(",\"texturePathCount\":");
    output.push_str(&texture_path_count.to_string());
    output.push_str(",\"selectedClipSource\":");
    output.push_str(&json_string(selected_clip_source));
    output.push_str(",\"selectedClipBasis\":\"current-renderer-shadow-outline-clip\"");
    output.push_str(",\"alternativeClipSource\":\"long-shadow-side-sweep\"");
    output.push_str(",\"alternativeRejected\":true");
    output.push_str(",\"alternativeRejectedBy\":\"historical-poppler-crop-ab\"");
    output.push_str(
        ",\"alternativeRejectedReason\":\"long-shadow-side-sweep-texture-clip-worsened-title-crops\"",
    );
    output.push_str(",\"frontFaceKnockoutDecoded\":false");
    output.push_str(",\"clipSemanticsDecoded\":false");
    output.push_str(",\"remainingBlockedReason\":\"texture-clip-and-knockout-semantics-unproven\"");
    output.push('}');
}

pub(crate) fn success_data_test_title_art_extrusion_texture_clip_gate_svg_attrs(
    texture_path_count: usize,
    selected_clip_source: &str,
) -> String {
    format!(
        " data-title-texture-clip-gate-source=\"embeddedPressOutlineTextureOutlineClipArbitration\" data-title-texture-clip-gate-reference-backed=\"true\" data-title-texture-clip-gate-render-promoted=\"true\" data-title-texture-clip-gate-path-count=\"{}\" data-title-texture-selected-clip-source=\"{}\" data-title-texture-selected-clip-basis=\"current-renderer-shadow-outline-clip\" data-title-texture-alternative-clip-source=\"long-shadow-side-sweep\" data-title-texture-alternative-clip-rejected=\"true\" data-title-texture-alternative-clip-rejected-by=\"historical-poppler-crop-ab\" data-title-texture-alternative-clip-rejected-reason=\"long-shadow-side-sweep-texture-clip-worsened-title-crops\" data-title-texture-front-face-knockout-decoded=\"false\" data-title-texture-clip-semantics-decoded=\"false\" data-title-texture-clip-semantics-blocked-reason=\"texture-clip-and-knockout-semantics-unproven\"",
        texture_path_count,
        escape_xml(selected_clip_source)
    )
}

pub(crate) fn success_data_test_title_art_texture_geometry_role_gate(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> TitleArtTextureGeometryRoleGate {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let shadow_paths = partition
        .as_ref()
        .map(|partition| partition.shadow_paths.as_slice())
        .unwrap_or(&[]);
    let main_paths = partition
        .as_ref()
        .map(|partition| partition.main_paths.as_slice())
        .unwrap_or(&[]);
    let texture_bbox = embedded_press_title_art_paths_source_bbox(texture_paths);
    let shadow_bbox = embedded_press_title_art_paths_source_bbox(shadow_paths);
    let main_bbox = embedded_press_title_art_paths_source_bbox(main_paths);
    let side_sweep_bbox = partition
        .as_ref()
        .and_then(|partition| main_bbox.map(|bbox| (bbox, partition.offset)))
        .map(|(bbox, offset)| {
            embedded_press_source_bbox_union(bbox, embedded_press_source_bbox_offset(bbox, offset))
        });
    let texture_area = texture_bbox.map_or(0, embedded_press_source_bbox_area);
    let texture_main_overlap_area = texture_bbox.zip(main_bbox).map_or(0, |(texture, main)| {
        embedded_press_source_bbox_intersection_area(texture, main)
    });
    let texture_shadow_overlap_area = texture_bbox
        .zip(shadow_bbox)
        .map_or(0, |(texture, shadow)| {
            embedded_press_source_bbox_intersection_area(texture, shadow)
        });
    let texture_side_sweep_overlap_area = texture_bbox
        .zip(side_sweep_bbox)
        .map_or(0, |(texture, side_sweep)| {
            embedded_press_source_bbox_intersection_area(texture, side_sweep)
        });
    let texture_main_overlap_ratio =
        embedded_press_source_bbox_area_ratio(texture_main_overlap_area, texture_area);
    let texture_shadow_overlap_ratio =
        embedded_press_source_bbox_area_ratio(texture_shadow_overlap_area, texture_area);
    let texture_side_sweep_overlap_ratio =
        embedded_press_source_bbox_area_ratio(texture_side_sweep_overlap_area, texture_area);
    let texture_contained_by_main_bbox = texture_bbox
        .zip(main_bbox)
        .is_some_and(|(texture, main)| embedded_press_source_bbox_contains(main, texture));
    let texture_contained_by_shadow_bbox = texture_bbox
        .zip(shadow_bbox)
        .is_some_and(|(texture, shadow)| embedded_press_source_bbox_contains(shadow, texture));
    let texture_contained_by_side_sweep_bbox =
        texture_bbox
            .zip(side_sweep_bbox)
            .is_some_and(|(texture, side_sweep)| {
                embedded_press_source_bbox_contains(side_sweep, texture)
            });
    let role_conclusion = if texture_bbox.is_none() {
        "texture-source-bbox-missing"
    } else if texture_main_overlap_area > 0 && texture_shadow_overlap_area > 0 {
        "texture-bbox-overlaps-main-and-shadow-outline-bboxes"
    } else if texture_side_sweep_overlap_area > 0 && texture_main_overlap_area == 0 {
        "texture-bbox-aligns-with-side-sweep-bbox-only"
    } else if texture_shadow_overlap_area > 0 && texture_main_overlap_area == 0 {
        "texture-bbox-aligns-with-shadow-outline-bbox-only"
    } else {
        "texture-bbox-role-relation-inconclusive"
    };
    let render_promotion_blocked_reason = if texture_bbox.is_none() {
        "texture-source-bbox-missing"
    } else if texture_contained_by_main_bbox
        || texture_contained_by_shadow_bbox
        || texture_contained_by_side_sweep_bbox
    {
        "texture-source-bbox-relation-is-bbox-only-not-knockout-proof"
    } else {
        "texture-source-bbox-role-relation-insufficient-for-knockout"
    };

    TitleArtTextureGeometryRoleGate {
        partition_present: partition.is_some(),
        texture_path_count: texture_paths.len(),
        shadow_outline_path_count: shadow_paths.len(),
        main_outline_path_count: main_paths.len(),
        texture_bbox,
        shadow_bbox,
        main_bbox,
        side_sweep_bbox,
        texture_area,
        texture_main_overlap_area,
        texture_shadow_overlap_area,
        texture_side_sweep_overlap_area,
        texture_main_overlap_ratio,
        texture_shadow_overlap_ratio,
        texture_side_sweep_overlap_ratio,
        texture_contained_by_main_bbox,
        texture_contained_by_shadow_bbox,
        texture_contained_by_side_sweep_bbox,
        role_conclusion,
        render_promotion_blocked_reason,
    }
}

pub(crate) fn push_success_data_test_title_art_texture_geometry_role_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let gate = success_data_test_title_art_texture_geometry_role_gate(snapshot, texture_paths);
    output.push_str(
        "{\"source\":\"embeddedPressSourceBboxRoleComparison\",\"decoded\":false,\"sourceBacked\":",
    );
    output.push_str(if gate.texture_bbox.is_some() && gate.partition_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referenceBacked\":false,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"partitionPresent\":");
    output.push_str(if gate.partition_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"texturePathCount\":");
    output.push_str(&gate.texture_path_count.to_string());
    output.push_str(",\"shadowOutlinePathCount\":");
    output.push_str(&gate.shadow_outline_path_count.to_string());
    output.push_str(",\"mainOutlinePathCount\":");
    output.push_str(&gate.main_outline_path_count.to_string());
    output.push_str(",\"textureBbox\":");
    push_embedded_press_source_bbox_option_json(output, gate.texture_bbox);
    output.push_str(",\"shadowOutlineBbox\":");
    push_embedded_press_source_bbox_option_json(output, gate.shadow_bbox);
    output.push_str(",\"mainOutlineBbox\":");
    push_embedded_press_source_bbox_option_json(output, gate.main_bbox);
    output.push_str(",\"sideSweepBbox\":");
    push_embedded_press_source_bbox_option_json(output, gate.side_sweep_bbox);
    output.push_str(",\"textureArea\":");
    output.push_str(&gate.texture_area.to_string());
    output.push_str(",\"textureMainOverlapArea\":");
    output.push_str(&gate.texture_main_overlap_area.to_string());
    output.push_str(",\"textureShadowOverlapArea\":");
    output.push_str(&gate.texture_shadow_overlap_area.to_string());
    output.push_str(",\"textureSideSweepOverlapArea\":");
    output.push_str(&gate.texture_side_sweep_overlap_area.to_string());
    output.push_str(",\"textureMainOverlapRatio\":");
    output.push_str(&format!("{:.3}", gate.texture_main_overlap_ratio));
    output.push_str(",\"textureShadowOverlapRatio\":");
    output.push_str(&format!("{:.3}", gate.texture_shadow_overlap_ratio));
    output.push_str(",\"textureSideSweepOverlapRatio\":");
    output.push_str(&format!("{:.3}", gate.texture_side_sweep_overlap_ratio));
    output.push_str(",\"textureContainedByMainBbox\":");
    output.push_str(if gate.texture_contained_by_main_bbox {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textureContainedByShadowBbox\":");
    output.push_str(if gate.texture_contained_by_shadow_bbox {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textureContainedBySideSweepBbox\":");
    output.push_str(if gate.texture_contained_by_side_sweep_bbox {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"frontFaceKnockoutDecoded\":false,\"clipSemanticsDecoded\":false");
    output.push_str(",\"roleConclusion\":");
    output.push_str(&json_string(gate.role_conclusion));
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(gate.render_promotion_blocked_reason));
    output.push('}');
}

pub(crate) fn success_data_test_title_art_texture_geometry_role_gate_svg_attrs(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> String {
    let gate = success_data_test_title_art_texture_geometry_role_gate(snapshot, texture_paths);
    format!(
        " data-title-texture-geometry-role-gate-source=\"embeddedPressSourceBboxRoleComparison\" data-title-texture-geometry-role-gate-source-backed=\"{}\" data-title-texture-geometry-role-gate-reference-backed=\"false\" data-title-texture-geometry-role-gate-render-promoted=\"false\" data-title-texture-geometry-role-conclusion=\"{}\" data-title-texture-geometry-role-blocked-reason=\"{}\" data-title-texture-geometry-main-overlap-ratio=\"{:.3}\" data-title-texture-geometry-shadow-overlap-ratio=\"{:.3}\" data-title-texture-geometry-side-sweep-overlap-ratio=\"{:.3}\" data-title-texture-geometry-contained-by-main-bbox=\"{}\" data-title-texture-geometry-contained-by-shadow-bbox=\"{}\" data-title-texture-geometry-contained-by-side-sweep-bbox=\"{}\"",
        gate.texture_bbox.is_some() && gate.partition_present,
        escape_xml(gate.role_conclusion),
        escape_xml(gate.render_promotion_blocked_reason),
        gate.texture_main_overlap_ratio,
        gate.texture_shadow_overlap_ratio,
        gate.texture_side_sweep_overlap_ratio,
        gate.texture_contained_by_main_bbox,
        gate.texture_contained_by_shadow_bbox,
        gate.texture_contained_by_side_sweep_bbox
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_success_data_test_title_art_texture_svg(
    svg: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    texture_path_source: &str,
    texture_clip_id: &str,
    texture_clip_source: &str,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) {
    if texture_paths.is_empty() {
        return;
    }

    let texture_header_count = embedded_press_snapshot_texture_bezier_header_count(snapshot);
    let texture_header = embedded_press_snapshot_texture_bezier_header_summary(snapshot);
    let texture_header_homogeneous =
        embedded_press_snapshot_texture_bezier_headers_are_homogeneous(snapshot);
    let effective_shadow_texture_paths =
        success_data_test_title_art_effective_texture_paths_for_word5(
            snapshot,
            EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5,
        );
    let effective_texture_state_word5_values =
        success_data_test_title_art_effective_texture_word5_values(snapshot);
    let effective_texture_state_word5_values_attr =
        embedded_press_state_word5_values_attr(&effective_texture_state_word5_values);
    let texture_state_source = if effective_shadow_texture_paths.len() == texture_paths.len() {
        "embedded-press-current-paint-state-inheritance"
    } else {
        "preserved-texture-paths-with-partial-current-state-evidence"
    };
    let texture_effect = if texture_path_source == "interstitial-between-shadow-and-main-outlines" {
        embedded_press_title_art_shadow_path_partition(snapshot)
            .as_ref()
            .and_then(|partition| embedded_press_title_art_shadow_effect(&partition.shadow_paths))
            .and_then(|effect| {
                embedded_press_title_art_texture_effect(texture_paths, &effect.fill_color)
            })
    } else {
        None
    };
    let texture_effect_attrs = texture_effect
        .as_ref()
        .map_or_else(String::new, EmbeddedPressTitleArtTextureEffect::svg_attrs);
    let texture_clip_gate_attrs = success_data_test_title_art_extrusion_texture_clip_gate_svg_attrs(
        texture_paths.len(),
        texture_clip_source,
    );
    let texture_geometry_role_attrs =
        success_data_test_title_art_texture_geometry_role_gate_svg_attrs(snapshot, texture_paths);
    let texture_render_fill = SUCCESS_DATA_TEST_TITLE_ART_FRONT_FILL_COLOR.to_string();
    let texture_source = texture_header.filter(|header| header.flags() == 1).map_or(
        "embedded-press-texture-bezier-filled-source-paths",
        |_| "embedded-press-texture-bezier-flags-1-filled-source-paths",
    );
    let texture_header_attrs = texture_header.map_or_else(String::new, |header| {
        format!(
            " data-texture-bezier-header-count=\"{texture_header_count}\" data-texture-bezier-point-count=\"{}\" data-texture-bezier-byte-count=\"{}\" data-texture-bezier-flags=\"{}\" data-texture-bezier-flags-hex=\"0x{:08x}\" data-texture-bezier-homogeneous=\"{}\"",
            header.point_count(),
            header.byte_count(),
            header.flags(),
            header.flags(),
            texture_header_homogeneous
        )
    });
    svg.push_str(&format!(
        "<g class=\"rjtd-success-data-test-title-art-textures\" clip-path=\"url(#{})\" data-title-layer=\"extrusion-texture\" data-title-compositing=\"shadow-under-front-face\" data-title-texture-path-source=\"{}\" data-title-texture-clip-source=\"{}\" data-texture-path-count=\"{}\" data-title-effective-shadow-texture-path-count=\"{}\" data-title-effective-texture-state-word5-values=\"{}\" data-title-texture-state-source=\"{}\" data-title-texture-rendering=\"filled-source-paths\" data-title-texture-render-fill=\"{}\"{}{}{}{}>",
        escape_xml(texture_clip_id),
        escape_xml(texture_path_source),
        escape_xml(texture_clip_source),
        texture_paths.len(),
        effective_shadow_texture_paths.len(),
        escape_xml(&effective_texture_state_word5_values_attr),
        escape_xml(texture_state_source),
        escape_xml(&texture_render_fill),
        texture_effect_attrs,
        texture_clip_gate_attrs,
        texture_geometry_role_attrs,
        texture_header_attrs
    ));
    for path in texture_paths {
        push_embedded_press_vector_path_svg(
            svg,
            "rjtd-success-data-test-title-art-texture-path",
            path,
            EmbeddedPressPageContext {
                x,
                y,
                scale_x,
                scale_y,
            },
            &texture_render_fill,
            "nonzero",
            Some(&embedded_press_title_art_state_word5(path).map_or_else(
                || {
                    format!(
                        " data-title-layer=\"extrusion-texture\" data-title-texture-source=\"{texture_source}\""
                    )
                },
                |state_word5| {
                    format!(
                        " data-title-layer=\"extrusion-texture\" data-title-texture-source=\"{texture_source}\" data-title-texture-state-word5=\"0x{state_word5:02x}\""
                    )
                },
            )),
        );
    }
    svg.push_str("</g>");
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_success_data_test_title_art_front_texture_svg(
    svg: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    texture_clip_id: &str,
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) {
    let main_texture_paths = success_data_test_title_art_front_texture_paths(snapshot);
    let front_erase_texture_paths = success_data_test_title_art_front_erase_texture_paths(snapshot);
    let (texture_paths, texture_path_source, texture_rendering, texture_opacity_source) =
        if !main_texture_paths.is_empty() {
            (
                main_texture_paths,
                "main-state-texture-paths",
                "paint-state-color-filled-source-paths",
                "state-tagged-texture-paths",
            )
        } else if !front_erase_texture_paths.is_empty() {
            (
                front_erase_texture_paths,
                "source-order-interstitial-front-erase-texture",
                "source-opacity-front-erase-precomposited-filled-source-paths",
                "embedded-press-front-erase-texture-opacity",
            )
        } else {
            (
                Vec::new(),
                "none",
                "paint-state-color-filled-source-paths",
                "state-tagged-texture-paths",
            )
        };
    if texture_paths.is_empty() {
        return;
    }

    let texture_fill = source_paint_candidate
        .and_then(jsfart_paint_candidate_color_hex)
        .or_else(|| embedded_press_snapshot_paint_state_color_hex(snapshot))
        .unwrap_or_else(|| "#ffffff".to_string());
    let texture_header_count = embedded_press_snapshot_texture_bezier_header_count(snapshot);
    let texture_header = embedded_press_snapshot_texture_bezier_header_summary(snapshot);
    let texture_header_homogeneous =
        embedded_press_snapshot_texture_bezier_headers_are_homogeneous(snapshot);
    let texture_state_word5_values = embedded_press_vector_path_state_word5_values(&texture_paths);
    let texture_state_word5_values_attr =
        embedded_press_state_word5_values_attr(&texture_state_word5_values);
    let effective_texture_state_word5_values =
        success_data_test_title_art_effective_front_texture_word5_values(snapshot);
    let effective_texture_state_word5_values_attr =
        embedded_press_state_word5_values_attr(&effective_texture_state_word5_values);
    let texture_state_summary_attrs =
        embedded_press_title_art_path_state_summary_svg_attrs(snapshot, &texture_paths);
    let texture_opacity = if texture_path_source == "source-order-interstitial-front-erase-texture"
    {
        embedded_press_title_art_front_erase_texture_opacity(&texture_paths)
            .map(|(opacity, _)| opacity)
            .unwrap_or(1.0)
    } else {
        1.0
    };
    let texture_render_fill =
        if texture_path_source == "source-order-interstitial-front-erase-texture" {
            blend_css_hex_colors(
                &texture_fill,
                SUCCESS_DATA_TEST_TITLE_ART_FRONT_FILL_COLOR,
                texture_opacity,
            )
            .unwrap_or_else(|| texture_fill.clone())
        } else {
            texture_fill.clone()
        };
    let texture_render_opacity = 1.0;
    let texture_opacity_application =
        if texture_path_source == "source-order-interstitial-front-erase-texture" {
            "precomposited-fill"
        } else {
            "svg-opacity"
        };
    let render_promotion_blocked_reason =
        success_data_test_title_art_front_texture_render_promotion_blocked_reason(
            texture_path_source,
        );
    let visible_render_path_count = if render_promotion_blocked_reason.is_some() {
        0
    } else {
        texture_paths.len()
    };
    let direct_gray_attrs =
        embedded_press_title_art_direct_gray_candidate_svg_attrs(&texture_paths);
    let source_paint_candidate_attrs = embedded_press_title_art_source_paint_candidate_svg_attrs(
        &texture_paths,
        source_paint_candidate,
    );
    let span_coverage_attrs = embedded_press_title_art_front_erase_texture_span_coverage_svg_attrs(
        snapshot,
        &texture_paths,
    );
    let transition_gate_attrs =
        embedded_press_title_art_front_erase_paint_transition_gate_svg_attrs(
            snapshot,
            &texture_paths,
        );
    let visible_probe_attrs =
        success_data_test_title_art_front_erase_visible_probe_gate_svg_attrs(&texture_paths);
    let texture_group_opacity_attr = if texture_render_opacity < 0.999 {
        format!(" opacity=\"{texture_render_opacity:.3}\"")
    } else {
        String::new()
    };
    let texture_header_attrs = texture_header.map_or_else(String::new, |header| {
        format!(
            " data-texture-bezier-header-count=\"{texture_header_count}\" data-texture-bezier-point-count=\"{}\" data-texture-bezier-byte-count=\"{}\" data-texture-bezier-flags=\"{}\" data-texture-bezier-flags-hex=\"0x{:08x}\" data-texture-bezier-homogeneous=\"{}\"",
            header.point_count(),
            header.byte_count(),
            header.flags(),
            header.flags(),
            texture_header_homogeneous
        )
    });
    svg.push_str(&format!(
        "<g class=\"rjtd-success-data-test-title-art-front-textures\" clip-path=\"url(#{})\"{} data-title-layer=\"front-texture\" data-title-compositing=\"source-paint-over-front-face\" data-title-texture-path-source=\"{}\" data-title-texture-clip-source=\"source-main-outline\" data-title-texture-rendering=\"{}\" data-title-texture-paint-color=\"{}\" data-title-texture-render-fill=\"{}\" data-title-texture-opacity=\"{texture_opacity:.3}\" data-title-texture-render-opacity=\"{texture_render_opacity:.3}\" data-title-texture-opacity-source=\"{}\" data-title-texture-opacity-application=\"{}\" data-title-texture-state-word5-values=\"{}\" data-title-effective-front-texture-state-word5-values=\"{}\" data-title-front-texture-state-source=\"embedded-press-current-paint-state-inheritance\" data-texture-path-count=\"{}\" data-visible-render-path-count=\"{}\" data-render-promoted=\"{}\" data-render-promotion-blocked-reason=\"{}\"{}{}{}{}{}{}{}>",
        escape_xml(texture_clip_id),
        texture_group_opacity_attr,
        escape_xml(texture_path_source),
        escape_xml(texture_rendering),
        escape_xml(&texture_fill),
        escape_xml(&texture_render_fill),
        escape_xml(texture_opacity_source),
        escape_xml(texture_opacity_application),
        escape_xml(&texture_state_word5_values_attr),
        escape_xml(&effective_texture_state_word5_values_attr),
        texture_paths.len(),
        visible_render_path_count,
        if render_promotion_blocked_reason.is_none() {
            "true"
        } else {
            "false"
        },
        escape_xml(render_promotion_blocked_reason.unwrap_or("none")),
        texture_state_summary_attrs,
        direct_gray_attrs,
        source_paint_candidate_attrs,
        span_coverage_attrs,
        transition_gate_attrs,
        visible_probe_attrs,
        texture_header_attrs
    ));
    if render_promotion_blocked_reason.is_some() {
        svg.push_str("</g>");
        return;
    }

    for path in texture_paths {
        let texture_source_attr = "embedded-press-paint-state-0x82-word3";
        let extra_attrs = embedded_press_title_art_state_word5(path).map_or_else(
            || {
                format!(
                    " data-title-layer=\"front-texture\" data-title-texture-source=\"{texture_source_attr}\""
                )
            },
            |state_word5| {
                format!(
                    " data-title-layer=\"front-texture\" data-title-texture-source=\"{texture_source_attr}\" data-title-texture-state-word5=\"0x{state_word5:02x}\""
                )
            },
        );
        push_embedded_press_vector_path_svg(
            svg,
            "rjtd-success-data-test-title-art-front-texture-path",
            path,
            EmbeddedPressPageContext {
                x,
                y,
                scale_x,
                scale_y,
            },
            &texture_render_fill,
            "nonzero",
            Some(&extra_attrs),
        );
    }
    svg.push_str("</g>");
}

pub(crate) fn success_data_test_title_art_front_texture_render_promotion_blocked_reason(
    texture_path_source: &str,
) -> Option<&'static str> {
    if texture_path_source == "source-order-interstitial-front-erase-texture" {
        Some("front-erase-texture-over-main-face-semantics-unproven")
    } else {
        None
    }
}

pub(crate) fn success_data_test_title_art_front_erase_explicit_state_path_count(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> usize {
    paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count()
}

pub(crate) fn push_success_data_test_title_art_front_erase_visible_probe_gate_json(
    output: &mut String,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let explicit_state_path_count =
        success_data_test_title_art_front_erase_explicit_state_path_count(paths);
    output.push_str("{\"source\":\"frontEraseTextureVisibleAB+visualReview\",\"decoded\":false,\"sourceBacked\":");
    output.push_str(if paths.is_empty() { "false" } else { "true" });
    output.push_str(",\"referenceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"currentVisiblePathCount\":0");
    output.push_str(",\"allVisiblePathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"explicitStateVisiblePathCount\":");
    output.push_str(&explicit_state_path_count.to_string());
    output.push_str(",\"currentTitleTightRms\":76.034,\"allVisibleTitleTightRms\":67.651,\"explicitStateVisibleTitleTightRms\":76.016");
    output.push_str(",\"currentTopCropRms\":51.191,\"allVisibleTopCropRms\":48.814,\"explicitStateVisibleTopCropRms\":51.186");
    output.push_str(",\"allVisibleRmsImproves\":true,\"allVisibleVisualRejected\":true");
    output.push_str(",\"allVisibleRejectedReason\":\"gray-overpaint-not-distressed-knockout\"");
    output.push_str(",\"explicitStateOnlyMaterialImprovement\":false");
    output.push_str(",\"renderPromotionBlockedReason\":\"front-erase-visible-rms-improvement-is-not-knockout-proof\"}");
}

pub(crate) fn success_data_test_title_art_front_erase_visible_probe_gate_svg_attrs(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> String {
    let explicit_state_path_count =
        success_data_test_title_art_front_erase_explicit_state_path_count(paths);
    format!(
        " data-title-front-erase-visible-probe-source=\"frontEraseTextureVisibleAB+visualReview\" data-title-front-erase-visible-probe-source-backed=\"{}\" data-title-front-erase-visible-probe-reference-backed=\"true\" data-title-front-erase-visible-probe-render-promoted=\"false\" data-title-front-erase-current-visible-path-count=\"0\" data-title-front-erase-all-visible-path-count=\"{}\" data-title-front-erase-explicit-state-visible-path-count=\"{}\" data-title-front-erase-current-title-tight-rms=\"76.034\" data-title-front-erase-all-visible-title-tight-rms=\"67.651\" data-title-front-erase-explicit-state-title-tight-rms=\"76.016\" data-title-front-erase-all-visible-visual-rejected=\"true\" data-title-front-erase-all-visible-rejected-reason=\"gray-overpaint-not-distressed-knockout\" data-title-front-erase-render-promotion-blocked-reason=\"front-erase-visible-rms-improvement-is-not-knockout-proof\"",
        !paths.is_empty(),
        paths.len(),
        explicit_state_path_count
    )
}

pub(crate) fn success_data_test_title_art_state_tagged_texture_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Texture
                && !path.commands().is_empty()
                && path
                    .state_records()
                    .iter()
                    .any(|record| record.record_type() == EMBEDDED_PRESS_RECORD_PAINT_STATE_82)
        })
        .collect::<Vec<_>>()
}

pub(crate) fn success_data_test_title_art_front_texture_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    success_data_test_title_art_effective_texture_paths_for_word5(
        snapshot,
        EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5,
    )
}

pub(crate) fn success_data_test_title_art_front_erase_texture_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    let Some(partition) = embedded_press_title_art_shadow_path_partition(snapshot) else {
        return Vec::new();
    };
    let Some(paths) = success_data_test_title_art_interstitial_texture_paths(snapshot, &partition)
    else {
        return Vec::new();
    };
    if success_data_test_title_art_interstitial_front_erase_gate(snapshot, &partition, &paths) {
        paths
    } else {
        Vec::new()
    }
}

pub(crate) fn success_data_test_title_art_interstitial_front_erase_gate(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    partition: &TitleArtShadowPathPartition<'_>,
    texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> bool {
    if partition.strategy != "embedded-press-source-order-outline-texture-outline"
        || texture_paths.is_empty()
    {
        return false;
    }

    let Some(interstitial_paths) =
        success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
    else {
        return false;
    };
    if !embedded_press_vector_path_refs_match(texture_paths, &interstitial_paths) {
        return false;
    }

    embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x48) == vec![1]
        && embedded_press_title_art_state_record_word0_values(texture_paths, 0x48) == vec![0]
        && embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
            == vec![0]
        && embedded_press_title_art_state_record_word_values(
            &partition.shadow_paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        ) == vec![EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5]
        && embedded_press_title_art_state_record_word_values(
            texture_paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        ) == vec![EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5]
        && embedded_press_title_art_state_record_word_values(
            &partition.main_paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        ) == vec![EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5]
        && embedded_press_title_art_state_record_word_values(
            texture_paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            3,
        ) == vec![0x00ff_ffff]
        && success_data_test_title_art_effective_texture_paths_for_word5(
            snapshot,
            EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5,
        )
        .len()
            == texture_paths.len()
        && success_data_test_title_art_effective_texture_paths_for_word5(
            snapshot,
            EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5,
        )
        .is_empty()
        && embedded_press_title_art_front_erase_texture_opacity(texture_paths).is_some()
}

pub(crate) fn success_data_test_title_art_front_erase_texture_path_source(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Option<&'static str> {
    if paths.is_empty() {
        None
    } else if paths.iter().all(|path| !path.state_records().is_empty()) {
        Some("explicit-state-textures-between-shadow-and-main-outlines")
    } else {
        Some("interstitial-between-shadow-and-main-outlines")
    }
}

pub(crate) fn success_data_test_title_art_texture_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Texture && !path.commands().is_empty()
        })
        .collect::<Vec<_>>()
}
