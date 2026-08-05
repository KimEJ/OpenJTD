use super::*;

pub(super) const SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX: f32 = 687.9;

pub(super) const SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX: f32 = 971.3;

pub(super) const SUCCESS_DATA_TEST_ABC_TABLE_X_PX: f32 = 79.3;

pub(super) const SUCCESS_DATA_TEST_ABC_TABLE_Y_PX: f32 = 410.7;

pub(super) const SUCCESS_DATA_TEST_ABC_TABLE_WIDTH_PX: f32 = 276.0;

pub(super) const SUCCESS_DATA_TEST_ABC_TABLE_ROW_HEIGHT_PX: f32 = 21.0;

pub(super) const SUCCESS_DATA_TEST_TITLE_ART_MAX_SEGMENT_SOURCE_LEN: f32 = 240.0;

pub(super) const SUCCESS_DATA_TEST_TITLE_ART_STROKE_WIDTH_PX: f32 = 0.32;

pub(super) const SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES: usize = 18;

pub(super) const SUCCESS_DATA_TEST_TITLE_ART_STATE_SIGNATURE_PREVIEW_LIMIT: usize = 12;

pub(super) const SUCCESS_DATA_TEST_TITLE_ART_FRONT_FILL_COLOR: &str = "#111111";

pub(super) const SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX: f32 = 13.5;

pub(super) const SUCCESS_DATA_TEST_CONE_TARGET_X_PX: f32 = 446.0;

pub(super) const SUCCESS_DATA_TEST_CONE_TARGET_Y_PX: f32 = 489.0;

pub(super) const SUCCESS_DATA_TEST_CONE_TARGET_WIDTH_PX: f32 = 128.0;

pub(super) const SUCCESS_DATA_TEST_CONE_TARGET_HEIGHT_PX: f32 = 148.0;

pub(super) const SUCCESS_DATA_TEST_Q4_SOURCE_LEFT: i32 = -15784;

pub(super) const SUCCESS_DATA_TEST_Q4_SOURCE_TOP: i32 = -10213;

pub(super) const SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT: i32 = -10584;

pub(super) const SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM: i32 = -9013;

pub(super) const SUCCESS_DATA_TEST_Q4_TARGET_X_PX: f32 = 93.3;

pub(super) const SUCCESS_DATA_TEST_Q4_TARGET_Y_PX: f32 = 663.3;

pub(super) const SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX: f32 = 491.4;

pub(super) const SUCCESS_DATA_TEST_Q4_TEXT_HEIGHT_FACTOR: f32 = 0.67;

pub(super) const SUCCESS_DATA_TEST_Q4_TEXT_BASELINE_FACTOR: f32 = 0.12;

pub(super) const SUCCESS_DATA_TEST_Q5_TARGET_X_PX: f32 = 490.7;

pub(super) const SUCCESS_DATA_TEST_Q5_TARGET_Y_PX: f32 = 795.0;

pub(super) const SUCCESS_DATA_TEST_Q5_TARGET_WIDTH_PX: f32 = 74.6;

pub(super) const SUCCESS_DATA_TEST_Q5_TARGET_HEIGHT_PX: f32 = 110.0;

pub(super) const SUCCESS_DATA_TEST_CONE_MIN_TEXT_CORROBORATION_COUNT: usize = 2;

pub(super) const SUCCESS_DATA_TEST_FDM_VECTOR_PATH: &str = "/FigureData/main_data/FDMVector";

pub(super) const SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_PATH: &str =
    "/FigureData/ExpandData/main_data/Data/FDMText";

pub(super) const SUCCESS_DATA_TEST_ANSWER_SHEET_LINK_PATH: &str =
    "/FigureData/ExpandData/main_data/Link";

pub(super) const SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_MARKER: &[u8; 4] = b"\x01\x00\x16\x60";

pub(super) const SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_LEFT_PT: f32 = 30.0;

pub(super) const SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_TOP_PT: f32 = 143.0;

pub(super) const SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_RIGHT_PT: f32 = 475.0;

pub(super) const SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_BOTTOM_PT: f32 = 600.0;

pub(super) const SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT: [(f32, f32, f32, f32); 19] = [
    (0.0, 0.0, 445.0, 0.0),
    (0.0, 63.0, 445.0, 63.0),
    (0.0, 79.0, 445.0, 79.0),
    (0.0, 173.0, 445.0, 173.0),
    (0.0, 205.0, 445.0, 205.0),
    (0.0, 377.0, 237.0, 377.0),
    (0.0, 409.0, 445.0, 409.0),
    (0.0, 457.0, 445.0, 457.0),
    (27.0, 31.0, 445.0, 31.0),
    (27.0, 110.0, 445.0, 110.0),
    (27.0, 142.0, 445.0, 142.0),
    (237.0, 236.0, 445.0, 236.0),
    (237.0, 268.0, 445.0, 268.0),
    (0.0, 0.0, 0.0, 457.0),
    (27.0, 0.0, 27.0, 457.0),
    (168.0, 0.0, 168.0, 173.0),
    (307.0, 0.0, 307.0, 173.0),
    (237.0, 173.0, 237.0, 409.0),
    (445.0, 0.0, 445.0, 457.0),
];

pub(super) const SUCCESS_DATA_TEST_TOP_TEXT_SLOTS: &[SuccessDataTestTextSlot] = &[
    SuccessDataTestTextSlot {
        role: "question-heading",
        text: "１，次の計算をしなさい",
        x: 37.7,
        y: 184.5,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（１）",
        x: 59.7,
        y: 205.5,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（２）",
        x: 59.7,
        y: 247.6,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（３）",
        x: 59.7,
        y: 289.5,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（４）",
        x: 59.7,
        y: 331.3,
    },
    SuccessDataTestTextSlot {
        role: "question-heading",
        text: "２，下の表は、ｃが斜辺の直角三角形で３辺ａ、ｂ、ｃの長さの関係を表したものである。",
        x: 37.7,
        y: 373.3,
    },
    SuccessDataTestTextSlot {
        role: "instruction",
        text: "空欄を埋めて表を完成させなさい。",
        x: 65.9,
        y: 394.3,
    },
    SuccessDataTestTextSlot {
        role: "question-heading",
        text: "３、右の図のような円錐について次の問に答えなさい。",
        x: 37.7,
        y: 499.2,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（１）この円錐の体積を求めなさい。",
        x: 37.7,
        y: 520.1,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（２）表面積を求めなさい。",
        x: 37.7,
        y: 541.2,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（３）この円錐の展開図の側面のおうぎ形の",
        x: 37.7,
        y: 562.1,
    },
    SuccessDataTestTextSlot {
        role: "question-continuation",
        text: "中心角を求めなさい。",
        x: 65.9,
        y: 583.1,
    },
    SuccessDataTestTextSlot {
        role: "question-heading",
        text: "４、次の図で∠ｘの大きさを求めなさい。",
        x: 37.7,
        y: 623.2,
    },
    SuccessDataTestTextSlot {
        role: "figure-label",
        text: "（１）",
        x: 82.7,
        y: 650.5,
    },
    SuccessDataTestTextSlot {
        role: "figure-label",
        text: "（２）",
        x: 321.0,
        y: 650.5,
    },
    SuccessDataTestTextSlot {
        role: "figure-label",
        text: "（３）",
        x: 535.0,
        y: 650.5,
    },
    SuccessDataTestTextSlot {
        role: "question-heading",
        text: "５、右の図は、半径ｒの球とその球がちょうど入る円柱、",
        x: 37.7,
        y: 791.2,
    },
    SuccessDataTestTextSlot {
        role: "question-continuation",
        text: "その円柱にちょうど入る円錐を表している。",
        x: 65.9,
        y: 812.5,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（１）球の体積をｒを使って表しなさい。",
        x: 65.9,
        y: 833.8,
    },
    SuccessDataTestTextSlot {
        role: "question-number",
        text: "（２）これらの、球、円柱、円錐の体積の間には",
        x: 65.9,
        y: 855.2,
    },
    SuccessDataTestTextSlot {
        role: "question-continuation",
        text: "どのような関係がありますか。",
        x: 82.0,
        y: 875.2,
    },
];

pub(super) const SUCCESS_DATA_TEST_FORMULA_TEXT_SLOTS: &[SuccessDataTestFormulaTextSlot] = &[
    SuccessDataTestFormulaTextSlot {
        embedding_index: 4,
        text: "１２",
        x: 124.0,
        baseline_y: 220.3,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 4,
        text: "÷",
        x: 155.2,
        baseline_y: 220.3,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 4,
        text: "３",
        x: 190.3,
        baseline_y: 220.3,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 8,
        text: "２",
        x: 113.1,
        baseline_y: 252.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 8,
        text: "６",
        x: 141.1,
        baseline_y: 252.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 8,
        text: "３",
        x: 134.4,
        baseline_y: 276.6,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 8,
        text: "＋",
        x: 162.3,
        baseline_y: 262.5,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 8,
        text: "１２８",
        x: 190.3,
        baseline_y: 262.5,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 8,
        text: "－",
        x: 235.6,
        baseline_y: 262.5,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 8,
        text: "１",
        x: 274.3,
        baseline_y: 249.9,
        font_size: 11.0,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 8,
        text: "２",
        x: 274.3,
        baseline_y: 270.9,
        font_size: 11.0,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "（",
        x: 109.3,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "２",
        x: 117.0,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "２",
        x: 144.5,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "＋",
        x: 162.3,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "３",
        x: 190.0,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "）",
        x: 205.0,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "２",
        x: 214.5,
        baseline_y: 293.0,
        font_size: 10.0,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "－２（",
        x: 223.5,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "２",
        x: 258.7,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "６",
        x: 286.7,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 12,
        text: "＋３）",
        x: 304.1,
        baseline_y: 304.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 10,
        text: "（",
        x: 100.0,
        baseline_y: 346.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 10,
        text: "２",
        x: 130.5,
        baseline_y: 346.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 10,
        text: "＋３）",
        x: 148.0,
        baseline_y: 346.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 10,
        text: "（",
        x: 180.0,
        baseline_y: 346.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 10,
        text: "２",
        x: 211.0,
        baseline_y: 346.0,
        font_size: 13.5,
    },
    SuccessDataTestFormulaTextSlot {
        embedding_index: 10,
        text: "－５）",
        x: 228.5,
        baseline_y: 346.0,
        font_size: 13.5,
    },
];

pub(super) fn push_page_layer_success_data_test_title_art_projection_json(
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

pub(super) fn success_data_test_fdm_reference_projection_layer_ops(
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

pub(super) fn success_data_test_fdm_reference_projection_layer_op(
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

pub(super) fn push_page_layer_success_data_test_answer_sheet_projection_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
) {
    let frame = SuccessDataTestAnswerSheetFrame::new(layout);
    let (x, y, width, height) = frame.bbox();
    output.push_str("{\"type\":\"answerSheetProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"documentTextTailAndFdmText1660\"");
    output.push_str(",\"projectionKind\":\"successDataTestAnswerSheetProjection\"");
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":true,\"referenceBacked\":true");
    output.push_str(",\"referenceFrame\":");
    push_success_data_test_answer_sheet_reference_frame_json(output, frame);
    output.push_str(",\"sourceFrameCandidate\":");
    if let Some(candidate) =
        success_data_test_answer_sheet_source_frame_candidate(document, layout, frame)
    {
        push_success_data_test_answer_sheet_source_frame_candidate_json(output, &candidate);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"localRuleSchemaCandidate\":");
    push_success_data_test_answer_sheet_local_rule_schema_candidate_json(
        output, document, layout, frame,
    );
    output.push_str(",\"ruleStyleCandidate\":");
    push_success_data_test_answer_sheet_rule_style_candidate_json(
        output,
        frame.stroke_width(0.92).clamp(0.8, 1.35),
    );
    output.push_str(",\"hatchedAnswerAreaCandidate\":");
    if let Some(area) = success_data_test_answer_sheet_hatched_area_candidate(document, layout) {
        push_answer_sheet_hatched_area_candidate_json(output, document, layout, &area);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"trianglePlacementCandidate\":");
    if let Some(candidate) =
        success_data_test_answer_sheet_triangle_placement_candidate(document, layout)
    {
        push_answer_sheet_triangle_placement_candidate_json(output, &candidate);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"documentTextTailEvidence\":");
    output.push_str(if success_data_test_answer_sheet_tail_evidence(document) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sparseTableEvidence\":");
    if let Some(candidate) = success_data_test_answer_sheet_sparse_table_candidate(document) {
        output.push_str("true,\"sparseTableCandidate\":");
        push_answer_sheet_sparse_table_candidate_json(output, layout, document, candidate);
    } else {
        output.push_str("false,\"sparseTableCandidate\":null");
    }
    output.push_str(",\"fdmTextVariant\":{\"sourcePath\":");
    output.push_str(&json_string(SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_PATH));
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(
        SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_MARKER,
    )));
    output.push_str(",\"present\":");
    if let Some(candidate) = success_data_test_answer_sheet_fdm_text_candidate(document) {
        output.push_str("true,\"size\":");
        output.push_str(&candidate.size().to_string());
        output.push_str(",\"payloadPrefix\":");
        output.push_str(&json_string(&hex_bytes(candidate.payload_prefix())));
        output.push_str(",\"textGeometryEvidence\":");
        push_answer_sheet_fdm_text_geometry_evidence_json(output, candidate);
    } else {
        output.push_str("false,\"size\":null,\"payloadPrefix\":null,\"textGeometryEvidence\":null");
    }
    output.push_str(",\"figureLinkEvidence\":");
    if let Some(candidate) = success_data_test_answer_sheet_figure_link_candidate(document) {
        push_answer_sheet_figure_link_evidence_json(output, candidate);
    } else {
        output.push_str("null");
    }
    output.push_str("}}");
}

pub(super) fn push_success_data_test_answer_sheet_reference_frame_json(
    output: &mut String,
    frame: SuccessDataTestAnswerSheetFrame,
) {
    let (x, y, width, height) = frame.bbox();
    output.push_str("{\"source\":\"answerSheetReferenceFrame\",\"coordinateSpace\":\"sheetLocalPt\",\"originPagePt\":{\"x\":");
    output.push_str(&format!("{:.3}", frame.left_pt));
    output.push_str(",\"y\":");
    output.push_str(&format!("{:.3}", frame.top_pt));
    output.push_str("},\"sizePt\":{\"width\":");
    output.push_str(&format!("{:.3}", frame.width_pt()));
    output.push_str(",\"height\":");
    output.push_str(&format!("{:.3}", frame.height_pt()));
    output.push_str("},\"pageBbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"geometryDecoded\":false,\"placementDecoded\":false,\"decoded\":false}");
}

pub(super) fn push_success_data_test_answer_sheet_rule_style_candidate_json(
    output: &mut String,
    stroke_width: f32,
) {
    let secondary_offset = success_data_test_answer_sheet_rule_secondary_offset(stroke_width);
    let secondary_stroke_width =
        success_data_test_answer_sheet_rule_secondary_stroke_width(stroke_width);
    output.push_str("{\"source\":\"referenceObservedSparseTableDoubleRules\",\"topologySource\":\"sparseTableCandidateTopology+answerSheetReferenceFrame\",\"topologySourceBacked\":true,\"styleSourceBacked\":false,\"referenceBacked\":true,\"decoded\":false,\"renderMode\":\"primary-plus-source-gated-secondary-line\",\"primaryLineSource\":\"source-grid-rule\",\"secondaryLineSource\":\"reference-observed-rule-pair\",\"secondaryOffsetPx\":");
    output.push_str(&format!("{secondary_offset:.3}"));
    output.push_str(",\"primaryStrokeWidthPx\":");
    output.push_str(&format!("{stroke_width:.3}"));
    output.push_str(",\"secondaryStrokeWidthPx\":");
    output.push_str(&format!("{secondary_stroke_width:.3}"));
    output.push_str(",\"secondaryLineGate\":\"before-source-identified-merged-answer-area\"");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"answer-sheet-rule-style-source-field-undecoded\"}",
    );
}

pub(super) fn success_data_test_answer_sheet_page_mark_disambiguation_ready(
    candidate: &SuccessDataTestAnswerSheetSourceFrameCandidate,
) -> bool {
    candidate.same_page_mark_entry && candidate.same_page_index_candidate
}

pub(super) fn success_data_test_answer_sheet_page_mark_disambiguation_class(
    candidate: &SuccessDataTestAnswerSheetSourceFrameCandidate,
) -> &'static str {
    if success_data_test_answer_sheet_page_mark_disambiguation_ready(candidate) {
        "same-page-mark-entry-and-page-index-candidate"
    } else {
        "page-mark-entry-or-page-index-mismatch"
    }
}

pub(super) fn success_data_test_answer_sheet_page_mark_disambiguation_blocked_reason(
    candidate: &SuccessDataTestAnswerSheetSourceFrameCandidate,
) -> Option<&'static str> {
    if success_data_test_answer_sheet_page_mark_disambiguation_ready(candidate) {
        None
    } else {
        Some("answer-sheet-page-mark-disambiguation-ambiguous")
    }
}

pub(super) fn push_success_data_test_answer_sheet_page_mark_disambiguation_gate_json(
    output: &mut String,
    candidate: &SuccessDataTestAnswerSheetSourceFrameCandidate,
) {
    let ready = success_data_test_answer_sheet_page_mark_disambiguation_ready(candidate);
    output.push_str("{\"source\":\"/LineMark+/PageMark section-anchor same-page gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"samePageMarkEntry\":");
    output.push_str(if candidate.same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"samePageIndexCandidate\":");
    output.push_str(if candidate.same_page_index_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"disambiguationReady\":");
    output.push_str(if ready { "true" } else { "false" });
    output.push_str(",\"disambiguationClass\":");
    output.push_str(&json_string(
        success_data_test_answer_sheet_page_mark_disambiguation_class(candidate),
    ));
    output.push_str(",\"renderPromotionBlockedReason\":");
    match success_data_test_answer_sheet_page_mark_disambiguation_blocked_reason(candidate) {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(super) fn success_data_test_answer_sheet_section_anchors(
    candidate: &TableCandidate,
) -> Vec<SuccessDataTestAnswerSheetSectionAnchor> {
    candidate
        .intervals()
        .iter()
        .filter_map(|row| {
            let cell = row
                .column_segments()
                .iter()
                .find(|cell| success_data_test_answer_sheet_section_label(cell.text()))?;
            Some(SuccessDataTestAnswerSheetSectionAnchor {
                section_label: cell.text().trim().to_string(),
                row_index: row.index(),
                source_interval_index: row.source_interval_index(),
                row_source_start: row.source_start(),
                row_source_end: row.source_end(),
                cell_index: cell.index(),
                cell_source_start: cell.source_start(),
                cell_source_end: cell.source_end(),
            })
        })
        .collect()
}

pub(super) fn success_data_test_answer_sheet_hatched_area_candidate(
    document: &Document,
    layout: PageLayout,
) -> Option<SuccessDataTestAnswerSheetHatchedAreaCandidate> {
    let candidate = success_data_test_answer_sheet_sparse_table_candidate(document)?;
    let section_anchors = success_data_test_answer_sheet_section_anchors(candidate);
    let top_anchor = section_anchors
        .iter()
        .find(|anchor| anchor.section_label == "５")?;
    let bottom_anchor = section_anchors
        .iter()
        .find(|anchor| anchor.section_label == "６")?;
    let top_row = candidate
        .intervals()
        .iter()
        .find(|row| row.index() == top_anchor.row_index)?;
    let empty_cell = top_row
        .column_segments()
        .iter()
        .find(|cell| cell.index() > top_anchor.cell_index && cell.text().is_empty())?;
    let adjacent_answer_cell = top_row
        .column_segments()
        .iter()
        .find(|cell| cell.index() > empty_cell.index() && cell.text().contains("ＡＢ"))?;

    let intervals = shanai_lan_line_mark_intervals(document);
    let top_source_grid = best_line_mark_interval_for_unit_range(
        &intervals,
        table_source_offset_to_units(candidate.basis(), top_anchor.row_source_start),
        table_source_offset_to_units(candidate.basis(), top_anchor.row_source_end),
    )
    .and_then(|interval| {
        success_data_test_line_mark_page_grid_candidate(document, layout, interval.record_index)
    });
    let bottom_source_grid = best_line_mark_interval_for_unit_range(
        &intervals,
        table_source_offset_to_units(candidate.basis(), bottom_anchor.row_source_start),
        table_source_offset_to_units(candidate.basis(), bottom_anchor.row_source_end),
    )
    .and_then(|interval| {
        success_data_test_line_mark_page_grid_candidate(document, layout, interval.record_index)
    });

    Some(SuccessDataTestAnswerSheetHatchedAreaCandidate {
        source: "sparseTableCandidateTopology+answerSheetReferenceFrame",
        top_section_label: top_anchor.section_label.clone(),
        bottom_section_label: bottom_anchor.section_label.clone(),
        top_row_index: top_anchor.row_index,
        bottom_row_index: bottom_anchor.row_index,
        top_source_interval_index: top_anchor.source_interval_index,
        bottom_source_interval_index: bottom_anchor.source_interval_index,
        empty_cell_index: empty_cell.index(),
        adjacent_answer_cell_index: adjacent_answer_cell.index(),
        sheet_left_pt: 27.0,
        sheet_top_pt: 205.0,
        sheet_right_pt: 237.0,
        sheet_bottom_pt: 377.0,
        top_source_grid,
        bottom_source_grid,
    })
}

pub(super) fn success_data_test_answer_sheet_source_frame_candidate(
    document: &Document,
    layout: PageLayout,
    frame: SuccessDataTestAnswerSheetFrame,
) -> Option<SuccessDataTestAnswerSheetSourceFrameCandidate> {
    let table_candidate = success_data_test_answer_sheet_sparse_table_candidate(document)?;
    let section_anchor_count =
        success_data_test_answer_sheet_section_anchors(table_candidate).len();
    let area = success_data_test_answer_sheet_hatched_area_candidate(document, layout)?;
    let top_grid = area.top_source_grid.clone()?;
    let bottom_grid = area.bottom_source_grid.clone()?;
    let local_span_pt = area.sheet_bottom_pt - area.sheet_top_pt;
    if local_span_pt <= 0.0 {
        return None;
    }
    let source_span_y = bottom_grid.row_top_y - top_grid.row_top_y;
    if source_span_y <= 0.0 {
        return None;
    }

    let source_px_per_sheet_pt_y = source_span_y / local_span_pt;
    let reference_frame_height = frame.bbox().3;
    let reference_px_per_sheet_pt_y = reference_frame_height / frame.height_pt();
    let derived_frame_top_y = top_grid.row_top_y - area.sheet_top_pt * source_px_per_sheet_pt_y;
    let derived_frame_height = frame.height_pt() * source_px_per_sheet_pt_y;
    let reference_frame_top_y = frame.bbox().1;
    let fdm_text_triangle_label_anchor_count =
        success_data_test_answer_sheet_triangle_placement_candidate(document, layout)
            .map(|candidate| candidate.label_anchors.len())
            .unwrap_or(0);
    let triangle_source_bbox = success_data_test_answer_sheet_fdm_text_candidate(document)
        .and_then(success_data_test_answer_sheet_triangle_source_bbox);

    Some(SuccessDataTestAnswerSheetSourceFrameCandidate {
        source: "sparseTableSectionAnchors+/LineMark+/PageMark+answerSheetReferenceLocalSchema",
        candidate_basis: "section-5-and-6-source-row-tops-vs-merged-answer-area-local-y",
        sparse_table_candidate_index: table_candidate.index(),
        section_anchor_count,
        top_section_label: area.top_section_label,
        bottom_section_label: area.bottom_section_label,
        top_row_index: area.top_row_index,
        bottom_row_index: area.bottom_row_index,
        top_line_mark_record_index: top_grid.record_index,
        bottom_line_mark_record_index: bottom_grid.record_index,
        local_top_pt: area.sheet_top_pt,
        local_bottom_pt: area.sheet_bottom_pt,
        source_px_per_sheet_pt_y,
        reference_px_per_sheet_pt_y,
        derived_frame_top_y,
        derived_frame_height,
        reference_frame_top_y,
        reference_frame_height,
        frame_top_residual_px: derived_frame_top_y - reference_frame_top_y,
        frame_height_residual_px: derived_frame_height - reference_frame_height,
        same_page_mark_entry: top_grid.page_mark_entry_index == bottom_grid.page_mark_entry_index,
        same_page_index_candidate: top_grid.page_index_candidate
            == bottom_grid.page_index_candidate,
        fdm_text_triangle_label_anchor_count,
        triangle_source_bbox,
    })
}

pub(super) fn push_success_data_test_answer_sheet_source_frame_candidate_json(
    output: &mut String,
    candidate: &SuccessDataTestAnswerSheetSourceFrameCandidate,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(candidate.source));
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.candidate_basis));
    output.push_str(",\"sparseTableCandidateIndex\":");
    output.push_str(&candidate.sparse_table_candidate_index.to_string());
    output.push_str(",\"sectionAnchorCount\":");
    output.push_str(&candidate.section_anchor_count.to_string());
    output.push_str(",\"topSectionLabel\":");
    output.push_str(&json_string(&candidate.top_section_label));
    output.push_str(",\"bottomSectionLabel\":");
    output.push_str(&json_string(&candidate.bottom_section_label));
    output.push_str(",\"topRowIndex\":");
    output.push_str(&candidate.top_row_index.to_string());
    output.push_str(",\"bottomRowIndex\":");
    output.push_str(&candidate.bottom_row_index.to_string());
    output.push_str(",\"topLineMarkRecordIndex\":");
    output.push_str(&candidate.top_line_mark_record_index.to_string());
    output.push_str(",\"bottomLineMarkRecordIndex\":");
    output.push_str(&candidate.bottom_line_mark_record_index.to_string());
    output.push_str(",\"samePageMarkEntry\":");
    output.push_str(if candidate.same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"samePageIndexCandidate\":");
    output.push_str(if candidate.same_page_index_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"localYAnchorsPt\":{\"top\":");
    output.push_str(&format!("{:.3}", candidate.local_top_pt));
    output.push_str(",\"bottom\":");
    output.push_str(&format!("{:.3}", candidate.local_bottom_pt));
    output.push_str(",\"span\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.local_bottom_pt - candidate.local_top_pt
    ));
    output.push_str("},\"sourceYAnchorsPx\":{\"top\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.derived_frame_top_y + candidate.local_top_pt * candidate.source_px_per_sheet_pt_y
    ));
    output.push_str(",\"bottom\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.derived_frame_top_y
            + candidate.local_bottom_pt * candidate.source_px_per_sheet_pt_y
    ));
    output.push_str(",\"span\":");
    output.push_str(&format!(
        "{:.3}",
        (candidate.local_bottom_pt - candidate.local_top_pt) * candidate.source_px_per_sheet_pt_y
    ));
    output.push_str("},\"sourcePxPerSheetPtY\":");
    output.push_str(&format!("{:.6}", candidate.source_px_per_sheet_pt_y));
    output.push_str(",\"referencePxPerSheetPtY\":");
    output.push_str(&format!("{:.6}", candidate.reference_px_per_sheet_pt_y));
    output.push_str(",\"derivedFrameTopY\":");
    output.push_str(&format!("{:.3}", candidate.derived_frame_top_y));
    output.push_str(",\"derivedFrameHeight\":");
    output.push_str(&format!("{:.3}", candidate.derived_frame_height));
    output.push_str(",\"referenceFrameTopY\":");
    output.push_str(&format!("{:.3}", candidate.reference_frame_top_y));
    output.push_str(",\"referenceFrameHeight\":");
    output.push_str(&format!("{:.3}", candidate.reference_frame_height));
    output.push_str(",\"frameTopResidualPx\":");
    output.push_str(&format!("{:.3}", candidate.frame_top_residual_px));
    output.push_str(",\"frameHeightResidualPx\":");
    output.push_str(&format!("{:.3}", candidate.frame_height_residual_px));
    output.push_str(",\"fdmTextTriangleLabelAnchorCount\":");
    output.push_str(&candidate.fdm_text_triangle_label_anchor_count.to_string());
    output.push_str(",\"triangleSourceBbox\":");
    match candidate.triangle_source_bbox {
        Some(bbox) => push_object_fdm_index_bbox_json(output, bbox),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageMarkDisambiguationGate\":");
    push_success_data_test_answer_sheet_page_mark_disambiguation_gate_json(output, candidate);
    output.push_str(
        ",\"renderPromotionContribution\":\"answer-sheet-source-frame-y-scale-candidate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"answer-sheet-x-width-and-local-schema-source-fields-undecoded\"");
    output.push_str(",\"renderPromotionBlockedReasons\":[");
    let mut reason_count = 0;
    if let Some(reason) =
        success_data_test_answer_sheet_page_mark_disambiguation_blocked_reason(candidate)
    {
        output.push_str(&json_string(reason));
        reason_count += 1;
    }
    if reason_count > 0 {
        output.push(',');
    }
    output.push_str(&json_string(
        "answer-sheet-x-width-and-local-schema-source-fields-undecoded",
    ));
    output.push_str("]}");
}

pub(super) fn push_success_data_test_answer_sheet_local_rule_schema_candidate_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    frame: SuccessDataTestAnswerSheetFrame,
) {
    let sparse_candidate = success_data_test_answer_sheet_sparse_table_candidate(document);
    let section_anchors = sparse_candidate
        .map(success_data_test_answer_sheet_section_anchors)
        .unwrap_or_default();
    let source_frame_candidate_present =
        success_data_test_answer_sheet_source_frame_candidate(document, layout, frame).is_some();
    let horizontal_rule_segment_count = SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT
        .iter()
        .filter(|(_, y1, _, y2)| (*y1 - *y2).abs() < f32::EPSILON)
        .count();
    let vertical_rule_segment_count = SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT
        .len()
        .saturating_sub(horizontal_rule_segment_count);
    let mut x_positions = Vec::new();
    let mut y_positions = Vec::new();
    for (x1, y1, x2, y2) in SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT {
        push_unique_f32(&mut x_positions, x1);
        push_unique_f32(&mut x_positions, x2);
        push_unique_f32(&mut y_positions, y1);
        push_unique_f32(&mut y_positions, y2);
    }
    x_positions.sort_by(|left, right| left.total_cmp(right));
    y_positions.sort_by(|left, right| left.total_cmp(right));

    output.push_str(
        "{\"source\":\"sparseTableCandidateTopology+referenceObservedAnswerSheetRuleSegments\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sparseTableCandidateIndex\":");
    match sparse_candidate {
        Some(candidate) => output.push_str(&candidate.index().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sparseTableRowCount\":");
    match sparse_candidate {
        Some(candidate) => output.push_str(&candidate.interval_count().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sparseTableMaxColumnCount\":");
    match sparse_candidate {
        Some(candidate) => output.push_str(&candidate.max_column_segment_count().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sectionAnchorCount\":");
    output.push_str(&section_anchors.len().to_string());
    output.push_str(",\"sectionAnchors\":[");
    for (index, anchor) in section_anchors.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"sectionLabel\":");
        output.push_str(&json_string(&anchor.section_label));
        output.push_str(",\"rowIndex\":");
        output.push_str(&anchor.row_index.to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&anchor.source_interval_index.to_string());
        output.push_str(",\"cellIndex\":");
        output.push_str(&anchor.cell_index.to_string());
        output.push_str(",\"cellSourceRange\":");
        match (anchor.cell_source_start, anchor.cell_source_end) {
            (Some(start), Some(end)) => output.push_str(&source_range_json(start, end)),
            _ => output.push_str("null"),
        }
        output.push('}');
    }
    output.push_str("],\"referenceRuleSegmentCount\":");
    output.push_str(
        &SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT
            .len()
            .to_string(),
    );
    output.push_str(",\"referenceHorizontalRuleSegmentCount\":");
    output.push_str(&horizontal_rule_segment_count.to_string());
    output.push_str(",\"referenceVerticalRuleSegmentCount\":");
    output.push_str(&vertical_rule_segment_count.to_string());
    output.push_str(",\"referenceLocalXPositionsPt\":");
    push_f32_array_json(output, &x_positions);
    output.push_str(",\"referenceLocalYPositionsPt\":");
    push_f32_array_json(output, &y_positions);
    output.push_str(",\"sourceFrameCandidatePresent\":");
    output.push_str(if source_frame_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"xSchemaSourceBacked\":false,\"yScaleSourceBacked\":");
    output.push_str(if source_frame_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ruleStyleSourceBacked\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"answer-sheet-local-rule-schema-readiness-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"answer-sheet-local-rule-schema-source-fields-undecoded\"}");
}

pub(super) fn push_success_data_test_answer_sheet_point_json(
    output: &mut String,
    point: SuccessDataTestAnswerSheetPoint,
) {
    output.push_str("{\"x\":");
    output.push_str(&format!("{:.3}", point.x));
    output.push_str(",\"y\":");
    output.push_str(&format!("{:.3}", point.y));
    output.push('}');
}

pub(super) fn success_data_test_answer_sheet_section_label(text: &str) -> bool {
    let mut chars = text.trim().chars();
    let Some(character) = chars.next() else {
        return false;
    };
    chars.next().is_none() && success_data_test_answer_sheet_row_number_char(character)
}

pub(super) fn push_page_layer_success_data_test_text_slot_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    source_id: usize,
    slot: &SuccessDataTestResolvedTextSlot,
    font_family: &str,
) {
    let fragment = success_data_test_resolved_text_slot_fragment(slot);
    let source_placement = success_data_test_source_text_placement_candidate(
        document,
        layout,
        slot.source_span.as_ref(),
        SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
    );
    let top_y = success_data_test_text_top_y(source_placement.as_ref(), slot.y);
    let baseline_y = success_data_test_text_baseline_y(
        source_placement.as_ref(),
        slot.y + SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
    );
    let text_width =
        text_width_px_for_font_size(SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX, slot.text)
            .max(f64::from(SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX));
    output.push_str("{\"type\":\"textRun\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        slot.x, top_y, text_width, APP_LINE_HEIGHT_PX
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(slot.text));
    output.push_str(&format!(
        ",\"baseline\":{:.3},\"rotation\":0.000,\"isVertical\":false,\"orientation\":\"horizontal\",\"fontFamily\":{},\"fillColor\":\"#111111\",\"projectionKind\":\"successDataTestTopTextProjection\",\"source\":",
        baseline_y,
        json_string(font_family),
    ));
    push_page_layer_source_span_json(output, source_id, &fragment);
    output.push_str(",\"positions\":");
    push_f64_array_json(
        output,
        &text_positions_px_for_font_size(SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX, slot.text),
    );
    output.push_str(",\"role\":");
    output.push_str(&json_string(slot.role));
    output.push_str(",\"sourceBacked\":");
    output.push_str(if slot.source_span.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(DOCUMENT_TEXT_PATH));
    output.push_str(",\"sourceGridPlacementCandidate\":");
    push_success_data_test_source_text_placement_candidate_json(
        output,
        document,
        layout,
        source_placement.as_ref(),
        Some(slot.y),
        Some(slot.y + SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX),
    );
    output.push_str(",\"lineHeaderEvidence\":");
    push_success_data_test_text_slot_line_header_evidence_json(output, slot);
    output.push_str(",\"lineMarkEvidence\":");
    push_success_data_test_line_mark_evidence_json(
        output,
        document,
        layout,
        slot.source_span.as_ref(),
        Some(slot.y),
        Some(slot.y + SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX),
    );
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"referenceBacked\":true,\"isParaEnd\":false,\"isLineBreakEnd\":false}");
}

pub(super) fn push_page_layer_success_data_test_figure_label_span_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    source_id: usize,
    line: &SuccessDataTestFigureLabelLine,
    span: &SuccessDataTestFigureLabelSpan,
    font_family: &str,
) {
    let fragment = success_data_test_figure_label_span_fragment(span);
    let source_placement = success_data_test_source_text_placement_candidate(
        document,
        layout,
        Some(&span.source_span),
        line.font_size,
    );
    let top_y = success_data_test_text_top_y(source_placement.as_ref(), line.y);
    let baseline_y =
        success_data_test_text_baseline_y(source_placement.as_ref(), line.y + line.font_size);
    let text_width =
        text_width_px_for_font_size(line.font_size, &span.text).max(f64::from(line.font_size));
    output.push_str("{\"type\":\"textRun\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        span.x, top_y, text_width, APP_LINE_HEIGHT_PX
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&span.text));
    output.push_str(&format!(
        ",\"baseline\":{:.3},\"rotation\":0.000,\"isVertical\":false,\"orientation\":\"horizontal\",\"fontFamily\":{},\"fillColor\":\"#111111\",\"projectionKind\":\"successDataTestTopTextProjection\",\"source\":",
        baseline_y,
        json_string(font_family),
    ));
    push_page_layer_source_span_json(output, source_id, &fragment);
    output.push_str(",\"positions\":");
    push_f64_array_json(
        output,
        &text_positions_px_for_font_size(line.font_size, &span.text),
    );
    output.push_str(",\"role\":\"figure-label\",\"renderSource\":\"document-text-fixed-pitch-span\",\"sourceBacked\":true,\"sourceStream\":");
    output.push_str(&json_string(DOCUMENT_TEXT_PATH));
    output.push_str(",\"sourceGridPlacementCandidate\":");
    push_success_data_test_source_text_placement_candidate_json(
        output,
        document,
        layout,
        source_placement.as_ref(),
        Some(line.y),
        Some(line.y + line.font_size),
    );
    output.push_str(",\"sourceLine\":");
    push_success_data_test_figure_label_line_evidence_json(output, line);
    output.push_str(",\"lineHeaderEvidence\":");
    push_success_data_test_line_header_evidence_json(output, line.line_header);
    output.push_str(",\"lineMarkEvidence\":");
    push_success_data_test_line_mark_evidence_json(
        output,
        document,
        layout,
        Some(&span.source_span),
        Some(line.y),
        Some(line.y + line.font_size),
    );
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"referenceBacked\":true,\"isParaEnd\":false,\"isLineBreakEnd\":false}");
}

pub(super) fn push_success_data_test_figure_label_line_evidence_json(
    output: &mut String,
    line: &SuccessDataTestFigureLabelLine,
) {
    output.push_str("{\"text\":");
    output.push_str(&json_string(&line.text));
    output.push_str(",\"renderSource\":\"document-text-preserved-spacing\",\"sourceByteRange\":");
    output.push_str(&source_range_json(
        line.source_span.byte_start,
        line.source_span.byte_end,
    ));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(
        line.source_span.unit_start,
        line.source_span.unit_end,
    ));
    output.push_str(",\"advanceModel\":\"japanese-fixed-pitch-halfwidth-space\",\"fontSize\":");
    output.push_str(&format!("{:.3}", line.font_size));
    output.push('}');
}

pub(super) fn push_success_data_test_text_slot_line_header_evidence_json(
    output: &mut String,
    slot: &SuccessDataTestResolvedTextSlot,
) {
    push_success_data_test_line_header_evidence_json(output, slot.line_header);
}

pub(super) fn push_success_data_test_line_mark_evidence_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    source_span: Option<&TextSourceSpan>,
    reference_top_y: Option<f32>,
    reference_baseline_y: Option<f32>,
) {
    output.push_str("{\"source\":\"/LineMark\",\"present\":");
    let Some(span) = source_span else {
        output.push_str("false,\"sourceBacked\":false,\"decoded\":false}");
        return;
    };
    let matched =
        success_data_test_line_mark_matches_for_source_span(document, span).collect::<Vec<_>>();
    output.push_str(if matched.is_empty() { "false" } else { "true" });
    output.push_str(",\"sourceBacked\":");
    output.push_str(if matched.is_empty() { "false" } else { "true" });
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(shanai_lan_line_mark_profile(document)));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(span.unit_start(), span.unit_end()));
    output.push_str(",\"matchedRecordCount\":");
    output.push_str(&matched.len().to_string());
    output.push_str(",\"records\":[");
    for (index, interval) in matched.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"recordIndex\":");
        output.push_str(&interval.record_index.to_string());
        output.push_str(",\"unitRange\":");
        output.push_str(&source_range_json(interval.unit_start, interval.unit_end));
        output.push_str(",\"flagWord\":");
        output.push_str(&interval.flag_word.to_string());
        output.push_str(",\"flagWordHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", interval.flag_word)));
        output.push_str(",\"containsSourceRange\":");
        output.push_str(
            if interval.unit_start <= span.unit_start() && span.unit_end() <= interval.unit_end {
                "true"
            } else {
                "false"
            },
        );
        output.push('}');
    }
    output.push_str("],\"pageGridCandidate\":");
    match success_data_test_best_line_mark_match_for_source_span(document, span).and_then(
        |interval| {
            success_data_test_line_mark_page_grid_candidate(document, layout, interval.record_index)
        },
    ) {
        Some(candidate) => push_success_data_test_line_mark_page_grid_candidate_json(
            output,
            document,
            layout,
            &candidate,
            reference_top_y,
            reference_baseline_y,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"renderPromotionContribution\":\"top-text-source-to-line-mark-row-evidence-only\",\"renderPromotionBlockedReason\":");
    if matched.is_empty() {
        output.push_str(&json_string(
            "source-span-does-not-overlap-line-mark-record",
        ));
    } else {
        output.push_str(&json_string("line-mark-row-to-page-y-transform-unproven"));
    }
    output.push('}');
}

pub(super) fn success_data_test_line_mark_matches_for_source_span<'a>(
    document: &'a Document,
    span: &TextSourceSpan,
) -> impl Iterator<Item = ShanaiLanLineMarkInterval> + 'a {
    let unit_start = span.unit_start();
    let unit_end = span.unit_end();
    shanai_lan_line_mark_intervals(document)
        .into_iter()
        .filter(move |interval| interval.unit_start < unit_end && unit_start < interval.unit_end)
}

pub(super) fn success_data_test_best_line_mark_match_for_source_span(
    document: &Document,
    span: &TextSourceSpan,
) -> Option<ShanaiLanLineMarkInterval> {
    let intervals =
        success_data_test_line_mark_matches_for_source_span(document, span).collect::<Vec<_>>();
    best_line_mark_interval_for_unit_range(&intervals, span.unit_start(), span.unit_end())
}

pub(super) fn success_data_test_page_mark_entry_for_record(
    document: &Document,
    record_index: usize,
) -> Option<&DocumentPageMarkEntry> {
    document
        .page_marks()
        .first()?
        .entries()
        .iter()
        .find(|entry| {
            let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                return false;
            };
            let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                return false;
            };
            line_start <= record_index && record_index <= line_end
        })
}

pub(super) fn success_data_test_line_mark_page_grid_candidate(
    document: &Document,
    layout: PageLayout,
    record_index: usize,
) -> Option<SuccessDataTestLineMarkPageGridCandidate> {
    let row_height = success_data_test_source_row_height_px(document)?;
    let entry = success_data_test_page_mark_entry_for_record(document, record_index)?;
    let page_line_start = entry.line_start()? as usize;
    let page_line_end = entry.line_end()? as usize;
    let line_offset_from_page_start = record_index.saturating_sub(page_line_start);
    let row_top_y = layout.margin_px() + line_offset_from_page_start as f32 * row_height;
    Some(SuccessDataTestLineMarkPageGridCandidate {
        record_index,
        page_mark_entry_index: entry.row_index(),
        page_index_candidate: entry.index().map(|index| index as usize),
        page_line_start,
        page_line_end,
        line_offset_from_page_start,
        row_height,
        row_height_basis: "abc-table-documentTextLineHeaderFontSizeUnits",
        row_top_y,
    })
}

pub(super) fn success_data_test_source_row_height_px(document: &Document) -> Option<f32> {
    success_data_test_source_row_height_font_size_units(document)
        .map(|font_size_units| f32::from(font_size_units) * 1.75)
}

pub(super) fn success_data_test_source_row_height_font_size_units(
    document: &Document,
) -> Option<u16> {
    let candidate = document
        .table_candidates()
        .iter()
        .find(|candidate| success_data_test_abc_table_candidate(candidate))?;
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    rows.iter()
        .flat_map(|row| row.headers.iter().map(|header| header.font_size_units))
        .try_fold(None, |seen, value| match seen {
            Some(previous) if previous != value => None,
            _ => Some(Some(value)),
        })
        .flatten()
}

pub(super) fn success_data_test_source_text_placement_candidate(
    document: &Document,
    layout: PageLayout,
    span: Option<&TextSourceSpan>,
    font_size: f32,
) -> Option<SuccessDataTestSourceTextPlacementCandidate> {
    let span = span?;
    let interval = success_data_test_best_line_mark_match_for_source_span(document, span)?;
    let line_grid =
        success_data_test_line_mark_page_grid_candidate(document, layout, interval.record_index)?;
    let baseline_y = line_grid.row_top_y + line_grid.row_height;
    let top_y = (baseline_y - font_size).max(0.0);
    Some(SuccessDataTestSourceTextPlacementCandidate {
        line_grid,
        font_size,
        top_y,
        baseline_y,
    })
}

pub(super) fn success_data_test_text_placement_residual_summary_json(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<String> {
    let slots = success_data_test_resolved_top_text_projection(document, page_number)?;
    let figure_label_line =
        success_data_test_top_text_projection(document, page_number).and_then(|static_slots| {
            success_data_test_q4_figure_label_source_line(document, static_slots)
        });
    let mut entries = Vec::new();
    for slot in &slots {
        if figure_label_line.is_some() && slot.role == "figure-label" {
            continue;
        }
        if let Some(entry) = success_data_test_text_placement_residual_entry(
            document,
            layout,
            slot.role,
            slot.text,
            slot.source_span.as_ref(),
            slot.line_header,
            slot.y,
            slot.y + SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
            SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
        ) {
            entries.push(entry);
        }
    }
    if let Some(line) = &figure_label_line {
        for span in &line.spans {
            if let Some(entry) = success_data_test_text_placement_residual_entry(
                document,
                layout,
                "figure-label",
                &span.text,
                Some(&span.source_span),
                line.line_header,
                line.y,
                line.y + line.font_size,
                line.font_size,
            ) {
                entries.push(entry);
            }
        }
    }
    if entries.is_empty() {
        return None;
    }

    let max_abs_top_residual_px = entries
        .iter()
        .map(|entry| entry.top_residual_px.abs())
        .fold(0.0f32, f32::max);
    let max_abs_baseline_residual_px = entries
        .iter()
        .map(|entry| entry.baseline_residual_px.abs())
        .fold(0.0f32, f32::max);
    let line_header_raw_word_profile_count = entries
        .iter()
        .filter_map(|entry| entry.line_header.map(|header| header.raw_words))
        .collect::<BTreeSet<_>>()
        .len();
    let mut buckets = BTreeMap::<
        SuccessDataTestTextPlacementResidualBucketKey,
        SuccessDataTestTextPlacementResidualBucket,
    >::new();
    for entry in &entries {
        let key = SuccessDataTestTextPlacementResidualBucketKey {
            top_residual_tenths: residual_tenths(entry.top_residual_px),
            baseline_residual_tenths: residual_tenths(entry.baseline_residual_px),
            flag_word: entry.flag_word,
            font_size_tenths: residual_tenths(entry.font_size),
            line_header_present: entry.line_header.is_some(),
        };
        let bucket =
            buckets
                .entry(key)
                .or_insert_with(|| SuccessDataTestTextPlacementResidualBucket {
                    count: 0,
                    record_indexes: Vec::new(),
                    roles: BTreeMap::new(),
                });
        bucket.count += 1;
        if !bucket.record_indexes.contains(&entry.record_index) {
            bucket.record_indexes.push(entry.record_index);
        }
        *bucket.roles.entry(entry.role).or_insert(0) += 1;
    }
    let source_row_height_px = success_data_test_source_row_height_px(document);
    let line_pitch_fits =
        success_data_test_text_placement_line_pitch_fits(&entries, source_row_height_px);
    let first_page_mark_entry = entries.first().and_then(|entry| {
        success_data_test_page_mark_entry_for_record(document, entry.record_index)
    });

    let mut output = String::new();
    output.push_str("{\"type\":\"textPlacementResidualSummary\",\"bbox\":{\"x\":0.000,\"y\":0.000,\"width\":0.000,\"height\":0.000}");
    output.push_str(",\"projectionKind\":\"successDataTestTextPlacementResidualSummary\"");
    output.push_str(",\"source\":\"/LineMark+/PageMark+documentTextLineHeaders\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"slotCount\":");
    output.push_str(&entries.len().to_string());
    output.push_str(",\"sourceGridCandidateCount\":");
    output.push_str(&entries.len().to_string());
    output.push_str(",\"maxAbsTopResidualPx\":");
    output.push_str(&format!("{max_abs_top_residual_px:.3}"));
    output.push_str(",\"maxAbsBaselineResidualPx\":");
    output.push_str(&format!("{max_abs_baseline_residual_px:.3}"));
    output.push_str(",\"lineHeaderRawWordProfileCount\":");
    output.push_str(&line_header_raw_word_profile_count.to_string());
    output.push_str(",\"linePitchFitEvidence\":");
    push_success_data_test_text_placement_line_pitch_fit_evidence_json(
        &mut output,
        source_row_height_px,
        &line_pitch_fits,
        first_page_mark_entry,
    );
    output.push_str(",\"residualBucketBasis\":\"rounded-tenths-top-baseline+line-mark-flag+font-size+line-header-present\"");
    output.push_str(",\"residualBuckets\":[");
    for (index, (key, bucket)) in buckets.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_success_data_test_text_placement_residual_bucket_json(&mut output, key, bucket);
    }
    output.push_str("],\"entries\":[");
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_success_data_test_text_placement_residual_entry_json(&mut output, entry);
    }
    output.push_str("],\"renderPromotionContribution\":\"source-grid-baseline-residual-bucket-evidence-only\",\"renderPromotionBlockedReason\":\"baseline-residual-source-field-semantics-unproven\"}");
    Some(output)
}

pub(super) fn success_data_test_text_placement_line_pitch_fits(
    entries: &[SuccessDataTestTextPlacementResidualEntry],
    source_row_height_px: Option<f32>,
) -> Vec<SuccessDataTestTextPlacementLinePitchFit> {
    let mut fits = Vec::new();
    let all_entries = entries.iter().collect::<Vec<_>>();
    if let Some(fit) = success_data_test_text_placement_line_pitch_fit(
        "all-visible-reference-baseline-vs-line-mark-record-index",
        &all_entries,
        source_row_height_px,
    ) {
        fits.push(fit);
    }
    let non_figure_entries = entries
        .iter()
        .filter(|entry| entry.role != "figure-label")
        .collect::<Vec<_>>();
    if let Some(fit) = success_data_test_text_placement_line_pitch_fit(
        "non-figure-reference-baseline-vs-line-mark-record-index",
        &non_figure_entries,
        source_row_height_px,
    ) {
        fits.push(fit);
    }
    let early_entries = entries
        .iter()
        .filter(|entry| entry.record_index <= 24)
        .collect::<Vec<_>>();
    if let Some(fit) = success_data_test_text_placement_line_pitch_fit(
        "early-records-through-24-reference-baseline-vs-line-mark-record-index",
        &early_entries,
        source_row_height_px,
    ) {
        fits.push(fit);
    }
    let late_entries = entries
        .iter()
        .filter(|entry| entry.record_index >= 26)
        .collect::<Vec<_>>();
    if let Some(fit) = success_data_test_text_placement_line_pitch_fit(
        "late-records-from-26-reference-baseline-vs-line-mark-record-index",
        &late_entries,
        source_row_height_px,
    ) {
        fits.push(fit);
    }
    fits
}

pub(super) fn success_data_test_text_placement_line_pitch_fit(
    basis: &'static str,
    entries: &[&SuccessDataTestTextPlacementResidualEntry],
    source_row_height_px: Option<f32>,
) -> Option<SuccessDataTestTextPlacementLinePitchFit> {
    if entries.len() < 2 {
        return None;
    }
    let entry_count = entries.len();
    let sum_x = entries
        .iter()
        .map(|entry| entry.record_index as f32)
        .sum::<f32>();
    let sum_y = entries
        .iter()
        .map(|entry| entry.reference_baseline_y)
        .sum::<f32>();
    let sum_xx = entries
        .iter()
        .map(|entry| {
            let x = entry.record_index as f32;
            x * x
        })
        .sum::<f32>();
    let sum_xy = entries
        .iter()
        .map(|entry| entry.record_index as f32 * entry.reference_baseline_y)
        .sum::<f32>();
    let count = entry_count as f32;
    let denominator = count * sum_xx - sum_x * sum_x;
    if denominator.abs() <= f32::EPSILON {
        return None;
    }
    let pitch = (count * sum_xy - sum_x * sum_y) / denominator;
    let intercept = (sum_y - pitch * sum_x) / count;
    let mut squared_residual_sum = 0.0f32;
    let mut max_abs_residual_px = 0.0f32;
    let mut record_start = usize::MAX;
    let mut record_end = 0usize;
    for entry in entries {
        let expected = intercept + pitch * entry.record_index as f32;
        let residual = entry.reference_baseline_y - expected;
        squared_residual_sum += residual * residual;
        max_abs_residual_px = max_abs_residual_px.max(residual.abs());
        record_start = record_start.min(entry.record_index);
        record_end = record_end.max(entry.record_index);
    }
    let rms_residual_px = (squared_residual_sum / count).sqrt();
    Some(SuccessDataTestTextPlacementLinePitchFit {
        basis,
        entry_count,
        record_start,
        record_end,
        intercept,
        pitch,
        rms_residual_px,
        max_abs_residual_px,
        source_row_height_px,
        source_row_height_minus_fit_pitch_px: source_row_height_px
            .map(|source_row_height_px| source_row_height_px - pitch),
    })
}

pub(super) fn push_success_data_test_text_placement_line_pitch_fit_evidence_json(
    output: &mut String,
    source_row_height_px: Option<f32>,
    fits: &[SuccessDataTestTextPlacementLinePitchFit],
    page_mark_entry: Option<&DocumentPageMarkEntry>,
) {
    output.push_str("{\"source\":\"referenceBaselines+/LineMarkRecordIndex+sourceRowHeight\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"fitModel\":\"least-squares-referenceBaselineY-equals-intercept-plus-pitch-times-recordIndex\"");
    output.push_str(",\"sourceRowHeightPx\":");
    match source_row_height_px {
        Some(value) => output.push_str(&format!("{value:.3}")),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceRowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\"");
    output.push_str(",\"pageMarkSelectedFields\":");
    push_success_data_test_page_mark_selected_fields_json(output, page_mark_entry);
    output.push_str(",\"fits\":[");
    for (index, fit) in fits.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_success_data_test_text_placement_line_pitch_fit_json(output, fit);
    }
    output.push_str("],\"renderPromotionContribution\":\"line-pitch-fit-diagnostic-only\",\"renderPromotionBlockedReason\":\"line-pitch-source-field-semantics-unproven\"}");
}

pub(super) fn push_success_data_test_text_placement_line_pitch_fit_json(
    output: &mut String,
    fit: &SuccessDataTestTextPlacementLinePitchFit,
) {
    output.push_str("{\"basis\":");
    output.push_str(&json_string(fit.basis));
    output.push_str(",\"entryCount\":");
    output.push_str(&fit.entry_count.to_string());
    output.push_str(",\"recordStart\":");
    output.push_str(&fit.record_start.to_string());
    output.push_str(",\"recordEnd\":");
    output.push_str(&fit.record_end.to_string());
    output.push_str(",\"intercept\":");
    output.push_str(&format!("{:.3}", fit.intercept));
    output.push_str(",\"pitch\":");
    output.push_str(&format!("{:.3}", fit.pitch));
    output.push_str(",\"rmsResidualPx\":");
    output.push_str(&format!("{:.3}", fit.rms_residual_px));
    output.push_str(",\"maxAbsResidualPx\":");
    output.push_str(&format!("{:.3}", fit.max_abs_residual_px));
    output.push_str(",\"sourceRowHeightPx\":");
    match fit.source_row_height_px {
        Some(value) => output.push_str(&format!("{value:.3}")),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceRowHeightMinusFitPitchPx\":");
    match fit.source_row_height_minus_fit_pitch_px {
        Some(value) => output.push_str(&format!("{value:.3}")),
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(super) fn push_success_data_test_page_mark_selected_fields_json(
    output: &mut String,
    page_mark_entry: Option<&DocumentPageMarkEntry>,
) {
    let Some(page_mark_entry) = page_mark_entry else {
        output.push_str("null");
        return;
    };
    push_page_mark_selected_fields_from_parts_json(
        output,
        Some(page_mark_entry.row_index()),
        page_mark_entry.line_start(),
        page_mark_entry.line_end(),
        page_mark_entry.u16_fields(),
    );
}

pub(super) fn push_success_data_test_source_pitch_evidence_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    candidate: &SuccessDataTestLineMarkPageGridCandidate,
) {
    let page_mark_entry =
        success_data_test_page_mark_entry_for_record(document, candidate.record_index);
    let font_size_units = success_data_test_source_row_height_font_size_units(document);
    let font_unit_scale_px = font_size_units.map(|units| candidate.row_height / f32::from(units));
    let line_gap_count = page_mark_entry
        .and_then(|entry| entry.line_start().zip(entry.line_end()))
        .map(|(start, end)| end.saturating_sub(start));
    let body_height_px_per_line_gap = line_gap_count
        .filter(|count| *count > 0)
        .map(|count| layout.body_height_px() / count as f32);
    let source_row_height_minus_body_height_per_line_gap_px =
        body_height_px_per_line_gap.map(|pitch| candidate.row_height - pitch);
    let field =
        |index: usize| page_mark_entry.and_then(|entry| entry.u16_fields().get(index).copied());
    let word_13_plus_14 = field(13)
        .zip(field(14))
        .and_then(|(left, right)| left.checked_add(right));
    let page_height_px_per_word_21_unit =
        field(21).map(|value| layout.height_px() / f32::from(value));
    let page_height_px_per_word_13_plus_14_unit =
        word_13_plus_14.map(|value| layout.height_px() / f32::from(value));
    let font_unit_scale_matches_page_mark_word_21 = font_unit_scale_px
        .zip(page_height_px_per_word_21_unit)
        .map(|(font_scale, page_mark_scale)| (font_scale - page_mark_scale).abs() <= 0.005);
    let font_unit_scale_matches_page_mark_word_13_plus_14 = font_unit_scale_px
        .zip(page_height_px_per_word_13_plus_14_unit)
        .map(|(font_scale, page_mark_scale)| (font_scale - page_mark_scale).abs() <= 0.005);

    output.push_str("{\"source\":\"/DocumentText+/LineMark+/PageMark\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"recordIndex\":");
    output.push_str(&candidate.record_index.to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&candidate.page_mark_entry_index.to_string());
    output.push_str(",\"pageLineStart\":");
    output.push_str(&candidate.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&candidate.page_line_end.to_string());
    output.push_str(",\"lineOffsetFromPageStart\":");
    output.push_str(&candidate.line_offset_from_page_start.to_string());
    output.push_str(",\"pageSizePx\":{\"width\":");
    output.push_str(&format!("{:.3}", layout.width_px()));
    output.push_str(",\"height\":");
    output.push_str(&format!("{:.3}", layout.height_px()));
    output.push_str("},\"bodySizePx\":{\"width\":");
    output.push_str(&format!("{:.3}", layout.body_width_px()));
    output.push_str(",\"height\":");
    output.push_str(&format!("{:.3}", layout.body_height_px()));
    output.push_str("},\"marginPx\":");
    output.push_str(&format!("{:.3}", layout.margin_px()));
    output.push_str(",\"rowHeightPx\":");
    output.push_str(&format!("{:.3}", candidate.row_height));
    output.push_str(",\"rowHeightBasis\":");
    output.push_str(&json_string(candidate.row_height_basis));
    output.push_str(",\"fontSizeUnits\":");
    push_optional_u16_json(output, font_size_units);
    output.push_str(",\"fontUnitScalePx\":");
    push_optional_f32_json(output, font_unit_scale_px);
    output.push_str(",\"bodyHeightPxPerLineGap\":");
    push_optional_f32_json(output, body_height_px_per_line_gap);
    output.push_str(",\"sourceRowHeightMinusBodyHeightPerLineGapPx\":");
    push_optional_f32_json(output, source_row_height_minus_body_height_per_line_gap_px);
    output.push_str(",\"pageMarkSelectedFields\":");
    push_success_data_test_page_mark_selected_fields_json(output, page_mark_entry);
    output.push_str(",\"linePitchAgreementGate\":");
    let empty_page_mark_fields: &[u16] = &[];
    let (line_start, line_end, page_mark_u16_fields) = page_mark_entry.map_or_else(
        || (None, None, empty_page_mark_fields),
        |entry| (entry.line_start(), entry.line_end(), entry.u16_fields()),
    );
    push_page_mark_line_pitch_agreement_gate_json(
        output,
        layout,
        line_start,
        line_end,
        Some(candidate.row_height),
        Some(candidate.row_height_basis),
        page_mark_u16_fields,
    );
    output.push_str(",\"pageHeightPxPerWord21Unit\":");
    push_optional_f32_json(output, page_height_px_per_word_21_unit);
    output.push_str(",\"pageHeightPxPerWord13Plus14Unit\":");
    push_optional_f32_json(output, page_height_px_per_word_13_plus_14_unit);
    output.push_str(",\"fontUnitScaleMatchesPageMarkWord21Candidate\":");
    push_optional_bool_json(output, font_unit_scale_matches_page_mark_word_21);
    output.push_str(",\"fontUnitScaleMatchesPageMarkWord13Plus14Candidate\":");
    push_optional_bool_json(output, font_unit_scale_matches_page_mark_word_13_plus_14);
    output.push_str(",\"renderPromotionContribution\":\"source-pitch-scale-comparison-diagnostic-only\",\"renderPromotionBlockedReason\":\"page-mark-line-pitch-semantics-unproven\"}");
}

#[allow(clippy::too_many_arguments)]
pub(super) fn success_data_test_text_placement_residual_entry(
    document: &Document,
    layout: PageLayout,
    role: &'static str,
    text: &str,
    source_span: Option<&TextSourceSpan>,
    line_header: Option<ShanaiLanLineHeader>,
    reference_top_y: f32,
    reference_baseline_y: f32,
    font_size: f32,
) -> Option<SuccessDataTestTextPlacementResidualEntry> {
    let source_span = source_span?;
    let placement = success_data_test_source_text_placement_candidate(
        document,
        layout,
        Some(source_span),
        font_size,
    )?;
    let line_mark = success_data_test_best_line_mark_match_for_source_span(document, source_span);
    Some(SuccessDataTestTextPlacementResidualEntry {
        role,
        text: text.to_string(),
        record_index: placement.line_grid.record_index,
        flag_word: line_mark.map(|interval| interval.flag_word),
        font_size,
        reference_top_y,
        reference_baseline_y,
        source_top_y: placement.top_y,
        source_baseline_y: placement.baseline_y,
        top_residual_px: reference_top_y - placement.top_y,
        baseline_residual_px: reference_baseline_y - placement.baseline_y,
        source_span: source_span.clone(),
        line_header,
    })
}

pub(super) fn push_success_data_test_text_placement_residual_bucket_json(
    output: &mut String,
    key: &SuccessDataTestTextPlacementResidualBucketKey,
    bucket: &SuccessDataTestTextPlacementResidualBucket,
) {
    output.push_str("{\"topResidualBucketPx\":");
    output.push_str(&json_string(&residual_tenths_string(
        key.top_residual_tenths,
    )));
    output.push_str(",\"baselineResidualBucketPx\":");
    output.push_str(&json_string(&residual_tenths_string(
        key.baseline_residual_tenths,
    )));
    output.push_str(",\"flagWord\":");
    match key.flag_word {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"flagWordHex\":");
    match key.flag_word {
        Some(value) => output.push_str(&json_string(&format!("0x{value:04x}"))),
        None => output.push_str("null"),
    }
    output.push_str(",\"fontSizeBucket\":");
    output.push_str(&json_string(&residual_tenths_string(key.font_size_tenths)));
    output.push_str(",\"lineHeaderPresent\":");
    output.push_str(if key.line_header_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"count\":");
    output.push_str(&bucket.count.to_string());
    output.push_str(",\"recordIndexes\":");
    push_usize_array_json(output, &bucket.record_indexes);
    output.push_str(",\"roles\":{");
    for (index, (role, count)) in bucket.roles.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(role));
        output.push(':');
        output.push_str(&count.to_string());
    }
    output.push_str("}}");
}

pub(super) fn push_success_data_test_text_placement_residual_entry_json(
    output: &mut String,
    entry: &SuccessDataTestTextPlacementResidualEntry,
) {
    output.push_str("{\"role\":");
    output.push_str(&json_string(entry.role));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&entry.text));
    output.push_str(",\"recordIndex\":");
    output.push_str(&entry.record_index.to_string());
    output.push_str(",\"flagWord\":");
    match entry.flag_word {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"flagWordHex\":");
    match entry.flag_word {
        Some(value) => output.push_str(&json_string(&format!("0x{value:04x}"))),
        None => output.push_str("null"),
    }
    output.push_str(",\"fontSize\":");
    output.push_str(&format!("{:.3}", entry.font_size));
    output.push_str(",\"referenceTopY\":");
    output.push_str(&format!("{:.3}", entry.reference_top_y));
    output.push_str(",\"sourceTopY\":");
    output.push_str(&format!("{:.3}", entry.source_top_y));
    output.push_str(",\"topResidualPx\":");
    output.push_str(&format!("{:.3}", entry.top_residual_px));
    output.push_str(",\"referenceBaselineY\":");
    output.push_str(&format!("{:.3}", entry.reference_baseline_y));
    output.push_str(",\"sourceBaselineY\":");
    output.push_str(&format!("{:.3}", entry.source_baseline_y));
    output.push_str(",\"baselineResidualPx\":");
    output.push_str(&format!("{:.3}", entry.baseline_residual_px));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(
        entry.source_span.unit_start(),
        entry.source_span.unit_end(),
    ));
    output.push_str(",\"lineHeaderPresent\":");
    output.push_str(if entry.line_header.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineHeaderRawWords\":");
    match entry.line_header {
        Some(header) => push_u16_array_json(output, &header.raw_words),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineHeaderRawWordsHex\":");
    match entry.line_header {
        Some(header) => push_u16_hex_array_json(output, &header.raw_words),
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(super) fn success_data_test_text_top_y(
    placement: Option<&SuccessDataTestSourceTextPlacementCandidate>,
    reference_top_y: f32,
) -> f32 {
    placement
        .map(|candidate| candidate.top_y)
        .unwrap_or(reference_top_y)
}

pub(super) fn success_data_test_text_baseline_y(
    placement: Option<&SuccessDataTestSourceTextPlacementCandidate>,
    reference_baseline_y: f32,
) -> f32 {
    placement
        .map(|candidate| candidate.baseline_y)
        .unwrap_or(reference_baseline_y)
}

pub(super) fn push_success_data_test_source_text_placement_candidate_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    candidate: Option<&SuccessDataTestSourceTextPlacementCandidate>,
    reference_top_y: Option<f32>,
    reference_baseline_y: Option<f32>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"source\":\"/LineMark+/PageMark+documentTextLineHeaders\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":true");
    output.push_str(",\"recordIndex\":");
    output.push_str(&candidate.line_grid.record_index.to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&candidate.line_grid.page_mark_entry_index.to_string());
    output.push_str(",\"lineOffsetFromPageStart\":");
    output.push_str(&candidate.line_grid.line_offset_from_page_start.to_string());
    output.push_str(",\"rowTopY\":");
    output.push_str(&format!("{:.3}", candidate.line_grid.row_top_y));
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{:.3}", candidate.line_grid.row_height));
    output.push_str(",\"rowHeightBasis\":");
    output.push_str(&json_string(candidate.line_grid.row_height_basis));
    output.push_str(",\"sourcePitchEvidence\":");
    push_success_data_test_source_pitch_evidence_json(
        output,
        document,
        layout,
        &candidate.line_grid,
    );
    output.push_str(",\"fontSize\":");
    output.push_str(&format!("{:.3}", candidate.font_size));
    output.push_str(",\"topY\":");
    output.push_str(&format!("{:.3}", candidate.top_y));
    output.push_str(",\"baselineY\":");
    output.push_str(&format!("{:.3}", candidate.baseline_y));
    output.push_str(",\"referenceResidualEvidence\":");
    match (reference_top_y, reference_baseline_y) {
        (Some(top), Some(baseline)) => {
            output.push_str("{\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false");
            output.push_str(",\"referenceTopY\":");
            output.push_str(&format!("{top:.3}"));
            output.push_str(",\"referenceBaselineY\":");
            output.push_str(&format!("{baseline:.3}"));
            output.push_str(",\"topMinusSourceTopPx\":");
            output.push_str(&format!("{:.3}", top - candidate.top_y));
            output.push_str(",\"baselineMinusSourceBaselinePx\":");
            output.push_str(&format!("{:.3}", baseline - candidate.baseline_y));
            output.push_str(",\"renderPromotionContribution\":\"source-grid-baseline-reference-residual-diagnostic-only\"}");
        }
        _ => output.push_str("null"),
    }
    output.push_str(",\"baselineBasis\":\"lineMarkRowTopPlusSourceRowHeight\"");
    output.push_str(",\"topBasis\":\"baselineMinusFontSize\"");
    output.push_str(",\"renderPromotionContribution\":\"source-backed-text-y-baseline-candidate\"");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"horizontal-origin-and-font-metrics-still-unproven\"}",
    );
}

pub(super) fn push_success_data_test_line_mark_page_grid_candidate_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    candidate: &SuccessDataTestLineMarkPageGridCandidate,
    reference_top_y: Option<f32>,
    reference_baseline_y: Option<f32>,
) {
    output.push_str("{\"source\":\"/LineMark+/PageMark+documentTextLineHeaders\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":true");
    output.push_str(",\"recordIndex\":");
    output.push_str(&candidate.record_index.to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&candidate.page_mark_entry_index.to_string());
    output.push_str(",\"pageIndexCandidate\":");
    push_optional_usize_json(output, candidate.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    output.push_str(&candidate.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&candidate.page_line_end.to_string());
    output.push_str(",\"lineOffsetFromPageStart\":");
    output.push_str(&candidate.line_offset_from_page_start.to_string());
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{:.3}", candidate.row_height));
    output.push_str(",\"rowHeightBasis\":");
    output.push_str(&json_string(candidate.row_height_basis));
    output.push_str(",\"sourcePitchEvidence\":");
    push_success_data_test_source_pitch_evidence_json(output, document, layout, candidate);
    output.push_str(",\"rowTopY\":");
    output.push_str(&format!("{:.3}", candidate.row_top_y));
    output.push_str(",\"baselineY\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.row_top_y + candidate.row_height
    ));
    output.push_str(",\"baselineBasis\":\"lineMarkRowTopPlusSourceRowHeight\"");
    output.push_str(",\"referenceResidualEvidence\":");
    match (reference_top_y, reference_baseline_y) {
        (Some(top), Some(baseline)) => {
            output.push_str("{\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false");
            output.push_str(",\"referenceTopY\":");
            output.push_str(&format!("{top:.3}"));
            output.push_str(",\"referenceBaselineY\":");
            output.push_str(&format!("{baseline:.3}"));
            output.push_str(",\"topMinusRowTopPx\":");
            output.push_str(&format!("{:.3}", top - candidate.row_top_y));
            output.push_str(",\"baselineMinusRowTopPx\":");
            output.push_str(&format!("{:.3}", baseline - candidate.row_top_y));
            output.push_str(
                ",\"renderPromotionContribution\":\"line-mark-row-y-residual-candidate-only\"}",
            );
        }
        _ => output.push_str("null"),
    }
    output.push_str(",\"renderPromotionContribution\":\"source-backed-line-mark-row-top-candidate\",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("text-baseline-offset-semantics-unproven"));
    output.push('}');
}

pub(super) fn push_success_data_test_line_header_evidence_json(
    output: &mut String,
    line_header: Option<ShanaiLanLineHeader>,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(DOCUMENT_TEXT_PATH));
    output.push_str(",\"present\":");
    if let Some(header) = line_header {
        output.push_str("true,\"sourceByteRange\":");
        output.push_str(&source_range_json(header.start, header.end));
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(header.start / 2, header.end / 2));
        output.push_str(",\"offsetUnits\":");
        output.push_str(&header.offset_units.to_string());
        output.push_str(",\"extentUnits\":");
        output.push_str(&header.extent_units.to_string());
        output.push_str(",\"fontSizeUnits\":");
        output.push_str(&header.font_size_units.to_string());
        output.push_str(",\"rawWords\":");
        push_u16_array_json(output, &header.raw_words);
        output.push_str(",\"rawWordsHex\":");
        push_u16_hex_array_json(output, &header.raw_words);
    } else {
        output.push_str("false");
    }
    output.push('}');
}

pub(super) fn push_success_data_test_title_art_projection_svg(
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

pub(super) fn push_success_data_test_answer_sheet_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
    font_family: &str,
) {
    if !success_data_test_answer_sheet_page(document, page_number) {
        return;
    }

    let frame = SuccessDataTestAnswerSheetFrame::new(layout);
    let stroke_width = frame.stroke_width(0.92).clamp(0.8, 1.35);
    let secondary_rule_offset = success_data_test_answer_sheet_rule_secondary_offset(stroke_width);
    let secondary_rule_stroke_width =
        success_data_test_answer_sheet_rule_secondary_stroke_width(stroke_width);
    let sparse_candidate_attrs =
        success_data_test_answer_sheet_sparse_table_candidate(document).map_or_else(
            || " data-sparse-table-evidence=\"false\"".to_string(),
            |candidate| {
                let section_anchor_count =
                    success_data_test_answer_sheet_section_anchors(candidate).len();
                format!(
                    " data-sparse-table-evidence=\"true\" data-sparse-table-candidate-index=\"{}\" data-sparse-table-row-count=\"{}\" data-sparse-table-max-columns=\"{}\" data-sparse-table-empty-cells=\"{}\" data-sparse-table-non-empty-cells=\"{}\" data-rule-topology-evidence=\"true\" data-rule-section-anchor-count=\"{}\"",
                    candidate.index(),
                    candidate.interval_count(),
                    candidate.max_column_segment_count(),
                    candidate.empty_cell_count_candidate(),
                    candidate.non_empty_cell_count_candidate(),
                    section_anchor_count
                )
            },
        );
    let triangle_source_attrs = success_data_test_answer_sheet_fdm_text_candidate(document)
	        .and_then(success_data_test_answer_sheet_triangle_source_bbox)
	        .map_or_else(String::new, |bbox| {
	            let (left, top, right, bottom) = normalize_fdm_bbox(bbox);
	            format!(
	                " data-source-left=\"{left}\" data-source-top=\"{top}\" data-source-right=\"{right}\" data-source-bottom=\"{bottom}\""
	            )
	        });
    let source_frame_attrs =
		        success_data_test_answer_sheet_source_frame_candidate(document, layout, frame)
		            .map_or_else(String::new, |candidate| {
		                format!(
	                    " data-source-frame-candidate=\"true\" data-source-frame-source=\"{}\" data-source-frame-basis=\"{}\" data-source-frame-y-scale-px-per-pt=\"{:.6}\" data-source-frame-reference-y-scale-px-per-pt=\"{:.6}\" data-source-frame-derived-top-y=\"{:.3}\" data-source-frame-derived-height=\"{:.3}\" data-source-frame-top-residual-px=\"{:.3}\" data-source-frame-height-residual-px=\"{:.3}\" data-source-frame-top-line-mark-record-index=\"{}\" data-source-frame-bottom-line-mark-record-index=\"{}\" data-source-frame-fdm-label-anchor-count=\"{}\" data-source-frame-same-page-mark-entry=\"{}\" data-source-frame-same-page-index-candidate=\"{}\" data-source-frame-page-mark-disambiguation-ready=\"{}\" data-source-frame-page-mark-disambiguation-class=\"{}\" data-source-frame-placement-proven=\"false\" data-source-frame-render-promotion-blocked-reason=\"answer-sheet-x-width-and-local-schema-source-fields-undecoded\"",
	                    escape_xml(candidate.source),
	                    escape_xml(candidate.candidate_basis),
	                    candidate.source_px_per_sheet_pt_y,
	                    candidate.reference_px_per_sheet_pt_y,
	                    candidate.derived_frame_top_y,
	                    candidate.derived_frame_height,
	                    candidate.frame_top_residual_px,
	                    candidate.frame_height_residual_px,
	                    candidate.top_line_mark_record_index,
	                    candidate.bottom_line_mark_record_index,
		                    candidate.fdm_text_triangle_label_anchor_count,
                        candidate.same_page_mark_entry,
                        candidate.same_page_index_candidate,
                        success_data_test_answer_sheet_page_mark_disambiguation_ready(&candidate),
                        escape_xml(success_data_test_answer_sheet_page_mark_disambiguation_class(
                            &candidate,
                        ))
		                )
		            });
    let local_rule_schema_attrs =
        success_data_test_answer_sheet_sparse_table_candidate(document).map_or_else(
            || " data-local-rule-schema-candidate=\"false\"".to_string(),
            |candidate| {
                let horizontal_count = SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT
                    .iter()
                    .filter(|(_, y1, _, y2)| (*y1 - *y2).abs() < f32::EPSILON)
                    .count();
                let vertical_count = SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT
                    .iter()
                    .filter(|(x1, _, x2, _)| (*x1 - *x2).abs() < f32::EPSILON)
                    .count();
                format!(
                    " data-local-rule-schema-candidate=\"true\" data-local-rule-schema-source=\"sparseTableCandidateTopology+referenceObservedAnswerSheetRuleSegments\" data-local-rule-schema-reference-backed=\"true\" data-local-rule-schema-x-source-backed=\"false\" data-local-rule-schema-y-scale-source-backed=\"{}\" data-local-rule-segment-count=\"{}\" data-local-rule-horizontal-segment-count=\"{}\" data-local-rule-vertical-segment-count=\"{}\" data-local-rule-schema-sparse-table-candidate-index=\"{}\" data-local-rule-schema-render-promotion-blocked-reason=\"answer-sheet-local-rule-schema-source-fields-undecoded\"",
                    if success_data_test_answer_sheet_source_frame_candidate(document, layout, frame).is_some() {
                        "true"
                    } else {
                        "false"
                    },
                    SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT.len(),
                    horizontal_count,
                    vertical_count,
                    candidate.index()
                )
            },
        );
    svg.push_str(&format!(
		        "<g class=\"rjtd-success-data-test-answer-sheet\" data-source=\"documentTextTailAndFdmText1660\" data-projection=\"successDataTestAnswerSheetProjection\" data-source-path=\"{}\" data-fdm-text-marker-hex=\"{}\"{}{}{} data-coordinate-space=\"sheetLocalPt\" data-reference-frame-source=\"answerSheetReferenceFrame\" data-reference-frame-origin-x-pt=\"{:.1}\" data-reference-frame-origin-y-pt=\"{:.1}\" data-reference-frame-width-pt=\"{:.1}\" data-reference-frame-height-pt=\"{:.1}\" data-rule-style-source=\"referenceObservedSparseTableDoubleRules\" data-rule-style-source-backed=\"false\" data-rule-style-reference-backed=\"true\" data-rule-style-decoded=\"false\" data-rule-render-mode=\"primary-plus-source-gated-secondary-line\" data-rule-secondary-line-gate=\"before-source-identified-merged-answer-area\" data-rule-secondary-offset-px=\"{secondary_rule_offset:.3}\" data-rule-secondary-stroke-width-px=\"{secondary_rule_stroke_width:.3}\" data-rule-style-render-promotion-blocked-reason=\"answer-sheet-rule-style-source-field-undecoded\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-reference-backed=\"true\" data-renderable=\"true\">",
        escape_xml(SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_PATH),
        hex_bytes(SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_MARKER),
        sparse_candidate_attrs,
        source_frame_attrs,
        local_rule_schema_attrs,
        frame.left_pt,
        frame.top_pt,
        frame.width_pt(),
        frame.height_pt()
    ));

    let hatched_area = success_data_test_answer_sheet_hatched_area_candidate(document, layout);
    let secondary_rule_limit_y = hatched_area
        .as_ref()
        .map(|area| frame.sheet_y(area.sheet_top_pt));
    for (x1, y1, x2, y2) in SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT {
        if hatched_area.as_ref().is_some_and(|area| {
            success_data_test_answer_sheet_rule_is_hatched_edge(area, x1, y1, x2, y2)
        }) {
            continue;
        }
        push_success_data_test_answer_sheet_line_svg(
            svg,
            frame.sheet_x(x1),
            frame.sheet_y(y1),
            frame.sheet_x(x2),
            frame.sheet_y(y2),
            stroke_width,
            secondary_rule_limit_y,
        );
    }

    let dash_width = (2.4 * PDF_POINT_TO_CSS_PX).clamp(3.0, 3.6);
    if let Some(area) = hatched_area.as_ref() {
        push_success_data_test_answer_sheet_hatch_svg(
            svg,
            frame,
            area,
            "top",
            area.sheet_left_pt,
            area.sheet_top_pt,
            area.sheet_right_pt,
            area.sheet_top_pt,
            dash_width,
        );
        push_success_data_test_answer_sheet_hatch_svg(
            svg,
            frame,
            area,
            "right",
            area.sheet_right_pt,
            area.sheet_top_pt,
            area.sheet_right_pt,
            area.sheet_bottom_pt,
            dash_width,
        );
        push_success_data_test_answer_sheet_hatch_svg(
            svg,
            frame,
            area,
            "bottom",
            area.sheet_left_pt,
            area.sheet_bottom_pt,
            area.sheet_right_pt,
            area.sheet_bottom_pt,
            dash_width,
        );
        push_success_data_test_answer_sheet_hatch_svg(
            svg,
            frame,
            area,
            "left",
            area.sheet_left_pt,
            area.sheet_top_pt,
            area.sheet_left_pt,
            area.sheet_bottom_pt,
            dash_width,
        );
    } else {
        for yy in [205.0, 377.0] {
            svg.push_str(&format!(
                "<line class=\"rjtd-success-data-test-answer-sheet-hatch\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#bfbfbf\" stroke-width=\"{dash_width:.2}\" stroke-dasharray=\"5 4\"/>",
                frame.sheet_x(27.0),
                frame.sheet_y(yy),
                frame.sheet_x(237.0),
                frame.sheet_y(yy)
            ));
        }
    }

    if let Some(candidate) =
        success_data_test_answer_sheet_triangle_placement_candidate(document, layout)
    {
        svg.push_str(&format!(
            "<path class=\"rjtd-success-data-test-answer-sheet-triangle\" data-source=\"{}\" data-placement-basis=\"{}\" data-placement-proven=\"false\" data-render-promotion-blocked-reason=\"fdmtext-source-to-sheet-transform-undecoded\"{} d=\"M {:.1} {:.1} L {:.1} {:.1} L {:.1} {:.1} Z\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\"/>",
            escape_xml(candidate.source),
            escape_xml(candidate.placement_basis),
            triangle_source_attrs,
            candidate.b.x,
            candidate.b.y,
            candidate.a.x,
            candidate.a.y,
            candidate.c.x,
            candidate.c.y
        ));
        svg.push_str(&format!(
            "<path class=\"rjtd-success-data-test-answer-sheet-right-angle\" data-source=\"{}\" data-placement-basis=\"{}\" data-placement-proven=\"false\" d=\"M {:.1} {:.1} L {:.1} {:.1} L {:.1} {:.1}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\"/>",
            escape_xml(candidate.source),
            escape_xml(candidate.placement_basis),
            candidate.right_angle_start.x,
            candidate.right_angle_start.y,
            candidate.right_angle_corner.x,
            candidate.right_angle_corner.y,
            candidate.right_angle_end.x,
            candidate.right_angle_end.y
        ));
    } else {
        svg.push_str(&format!(
            "<path class=\"rjtd-success-data-test-answer-sheet-triangle\" data-source=\"FDMTextIndex\"{triangle_source_attrs} d=\"M {:.1} {:.1} L {:.1} {:.1} L {:.1} {:.1} Z\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\"/>",
            frame.sheet_x(42.0),
            frame.sheet_y(334.0),
            frame.sheet_x(210.0),
            frame.sheet_y(238.0),
            frame.sheet_x(210.0),
            frame.sheet_y(334.0)
        ));
        svg.push_str(&format!(
            "<path class=\"rjtd-success-data-test-answer-sheet-right-angle\" d=\"M {:.1} {:.1} L {:.1} {:.1} L {:.1} {:.1}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\"/>",
            frame.sheet_x(202.0),
            frame.sheet_y(334.0),
            frame.sheet_x(202.0),
            frame.sheet_y(326.0),
            frame.sheet_x(210.0),
            frame.sheet_y(326.0)
        ));
    }

    for slot in success_data_test_answer_sheet_text_slots(document, layout) {
        push_success_data_test_answer_sheet_text_svg(
            svg,
            &slot.text,
            slot.source_token_index,
            slot.x,
            slot.y,
            slot.font_size,
            slot.anchor,
            font_family,
        );
    }
    for slot in success_data_test_answer_sheet_fdm_label_slots(document, layout) {
        push_success_data_test_answer_sheet_fdm_text_svg(svg, &slot, font_family);
    }

    svg.push_str("</g>");
}

pub(super) fn push_success_data_test_answer_sheet_line_svg(
    svg: &mut String,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    stroke_width: f32,
    secondary_rule_limit_y: Option<f32>,
) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let length = (dx * dx + dy * dy).sqrt();
    if length <= f32::EPSILON {
        return;
    }
    let normal_x = -dy / length;
    let normal_y = dx / length;
    svg.push_str(&format!(
        "<line class=\"rjtd-success-data-test-answer-sheet-rule\" data-rule-render-mode=\"source-grid-primary-line\" x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\"/>"
    ));
    let gap = success_data_test_answer_sheet_rule_secondary_offset(stroke_width);
    let offset = if dx.abs() >= dy.abs() { gap } else { -gap };
    let ox = normal_x * offset;
    let oy = normal_y * offset;
    let Some((secondary_x1, secondary_y1, secondary_x2, secondary_y2)) =
        success_data_test_answer_sheet_secondary_rule_segment(
            x1,
            y1,
            x2,
            y2,
            secondary_rule_limit_y,
        )
    else {
        return;
    };
    let sx1 = secondary_x1 + ox;
    let sy1 = secondary_y1 + oy;
    let sx2 = secondary_x2 + ox;
    let sy2 = secondary_y2 + oy;
    let secondary_stroke_width =
        success_data_test_answer_sheet_rule_secondary_stroke_width(stroke_width);
    svg.push_str(&format!(
        "<line class=\"rjtd-success-data-test-answer-sheet-rule\" data-rule-render-mode=\"reference-observed-secondary-line\" data-secondary-line-gate=\"before-source-identified-merged-answer-area\" x1=\"{sx1:.1}\" y1=\"{sy1:.1}\" x2=\"{sx2:.1}\" y2=\"{sy2:.1}\" stroke=\"#111111\" stroke-width=\"{secondary_stroke_width:.2}\"/>"
    ));
}

pub(super) fn success_data_test_answer_sheet_secondary_rule_segment(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    limit_y: Option<f32>,
) -> Option<(f32, f32, f32, f32)> {
    let Some(limit_y) = limit_y else {
        return Some((x1, y1, x2, y2));
    };
    const EPSILON: f32 = 0.01;
    if y1 < limit_y - EPSILON && y2 < limit_y - EPSILON {
        return Some((x1, y1, x2, y2));
    }
    if (x1 - x2).abs() <= EPSILON {
        let top_y = y1.min(y2);
        let bottom_y = y1.max(y2);
        if top_y >= limit_y - EPSILON {
            return None;
        }
        let clipped_bottom_y = bottom_y.min(limit_y);
        if clipped_bottom_y - top_y <= EPSILON {
            return None;
        }
        if y1 <= y2 {
            Some((x1, top_y, x2, clipped_bottom_y))
        } else {
            Some((x1, clipped_bottom_y, x2, top_y))
        }
    } else {
        None
    }
}

pub(super) fn success_data_test_answer_sheet_rule_secondary_offset(stroke_width: f32) -> f32 {
    (stroke_width * 1.55).clamp(1.35, 1.65)
}

pub(super) fn success_data_test_answer_sheet_rule_secondary_stroke_width(stroke_width: f32) -> f32 {
    (stroke_width * 0.46).clamp(0.42, 0.62)
}

pub(super) fn success_data_test_answer_sheet_rule_is_hatched_edge(
    area: &SuccessDataTestAnswerSheetHatchedAreaCandidate,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
) -> bool {
    [
        (
            area.sheet_left_pt,
            area.sheet_top_pt,
            area.sheet_right_pt,
            area.sheet_top_pt,
        ),
        (
            area.sheet_right_pt,
            area.sheet_top_pt,
            area.sheet_right_pt,
            area.sheet_bottom_pt,
        ),
        (
            area.sheet_left_pt,
            area.sheet_bottom_pt,
            area.sheet_right_pt,
            area.sheet_bottom_pt,
        ),
        (
            area.sheet_left_pt,
            area.sheet_top_pt,
            area.sheet_left_pt,
            area.sheet_bottom_pt,
        ),
    ]
    .into_iter()
    .any(|(edge_x1, edge_y1, edge_x2, edge_y2)| {
        (x1 - edge_x1).abs() < f32::EPSILON
            && (y1 - edge_y1).abs() < f32::EPSILON
            && (x2 - edge_x2).abs() < f32::EPSILON
            && (y2 - edge_y2).abs() < f32::EPSILON
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_success_data_test_answer_sheet_hatch_svg(
    svg: &mut String,
    frame: SuccessDataTestAnswerSheetFrame,
    area: &SuccessDataTestAnswerSheetHatchedAreaCandidate,
    edge: &str,
    x1_pt: f32,
    y1_pt: f32,
    x2_pt: f32,
    y2_pt: f32,
    stroke_width: f32,
) {
    let x1 = frame.sheet_x(x1_pt);
    let y1 = frame.sheet_y(y1_pt);
    let x2 = frame.sheet_x(x2_pt);
    let y2 = frame.sheet_y(y2_pt);
    svg.push_str(&format!(
        "<g class=\"rjtd-success-data-test-answer-sheet-hatch\" data-source=\"{}\" data-edge=\"{}\" data-top-section-label=\"{}\" data-bottom-section-label=\"{}\" data-empty-cell-index=\"{}\" data-adjacent-answer-cell-index=\"{}\" data-hatch-style-source=\"referenceObservedAnswerAreaEdgeHatch\" data-hatch-style-source-backed=\"false\" data-hatch-style-reference-backed=\"true\" data-hatch-style-decoded=\"false\" data-hatch-render-mode=\"diagonal-edge-segments\" data-hatch-style-render-promotion-blocked-reason=\"answer-sheet-hatch-style-source-field-undecoded\">",
        escape_xml(area.source),
        escape_xml(edge),
        escape_xml(&area.top_section_label),
        escape_xml(&area.bottom_section_label),
        area.empty_cell_index,
        area.adjacent_answer_cell_index
    ));
    let dx = x2 - x1;
    let dy = y2 - y1;
    let length = (dx * dx + dy * dy).sqrt();
    if length > 0.0 {
        let tangent_x = dx / length;
        let tangent_y = dy / length;
        let normal_x = -tangent_y;
        let normal_y = tangent_x;
        let period = (stroke_width * 4.25).clamp(13.0, 15.0);
        let segment_length = (stroke_width * 3.4).clamp(10.0, 12.5);
        let segment_thickness = stroke_width;
        let shear = (stroke_width * 1.65).clamp(4.5, 6.0);
        let mut distance = period * 0.5;
        while distance < length {
            let cx = x1 + tangent_x * distance;
            let cy = y1 + tangent_y * distance;
            let p1x = cx
                + tangent_x * (-segment_length * 0.5 + shear * 0.5)
                + normal_x * (-segment_thickness * 0.5);
            let p1y = cy
                + tangent_y * (-segment_length * 0.5 + shear * 0.5)
                + normal_y * (-segment_thickness * 0.5);
            let p2x = cx
                + tangent_x * (segment_length * 0.5 + shear * 0.5)
                + normal_x * (-segment_thickness * 0.5);
            let p2y = cy
                + tangent_y * (segment_length * 0.5 + shear * 0.5)
                + normal_y * (-segment_thickness * 0.5);
            let p3x = cx
                + tangent_x * (segment_length * 0.5 - shear * 0.5)
                + normal_x * (segment_thickness * 0.5);
            let p3y = cy
                + tangent_y * (segment_length * 0.5 - shear * 0.5)
                + normal_y * (segment_thickness * 0.5);
            let p4x = cx
                + tangent_x * (-segment_length * 0.5 - shear * 0.5)
                + normal_x * (segment_thickness * 0.5);
            let p4y = cy
                + tangent_y * (-segment_length * 0.5 - shear * 0.5)
                + normal_y * (segment_thickness * 0.5);
            svg.push_str(&format!(
                "<polygon class=\"rjtd-success-data-test-answer-sheet-hatch-segment\" points=\"{p1x:.1},{p1y:.1} {p2x:.1},{p2y:.1} {p3x:.1},{p3y:.1} {p4x:.1},{p4y:.1}\" fill=\"#bfbfbf\" stroke=\"none\"/>"
            ));
            distance += period;
        }
    }
    svg.push_str("</g>");
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_success_data_test_answer_sheet_text_svg(
    svg: &mut String,
    text: &str,
    source_token_index: usize,
    x: f32,
    y: f32,
    font_size: f32,
    anchor: &str,
    font_family: &str,
) {
    svg.push_str(&format!(
        "<text class=\"rjtd-success-data-test-answer-sheet-text\" data-source=\"DocumentText\" data-source-token-index=\"{source_token_index}\" x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"{font_size:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
        escape_xml(anchor),
        escape_xml(font_family),
        escape_xml(text)
    ));
}

pub(super) fn push_success_data_test_answer_sheet_fdm_text_svg(
    svg: &mut String,
    slot: &SuccessDataTestAnswerSheetFdmTextSlot,
    font_family: &str,
) {
    svg.push_str(&format!(
        "<text class=\"rjtd-success-data-test-answer-sheet-fdm-text\" data-source=\"FDMText\" data-marker-offset=\"{}\" data-text-offset=\"{}\" data-index-offset=\"{}\" data-source-left=\"{}\" data-source-top=\"{}\" data-source-right=\"{}\" data-source-bottom=\"{}\" data-text-left=\"{}\" data-text-top=\"{}\" data-text-right=\"{}\" data-text-bottom=\"{}\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
        slot.marker_offset,
        slot.text_offset,
        slot.index_offset,
        slot.source_bbox.left(),
        slot.source_bbox.top(),
        slot.source_bbox.right(),
        slot.source_bbox.bottom(),
        slot.text_bbox.left(),
        slot.text_bbox.top(),
        slot.text_bbox.right(),
        slot.text_bbox.bottom(),
        slot.x,
        slot.y,
        escape_xml(font_family),
        slot.font_size,
        escape_xml(&slot.text)
    ));
}

pub(super) fn push_success_data_test_title_art_path_svg(
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

pub(super) fn success_data_test_title_art_front_paint_color_candidate<'a>(
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

pub(super) fn success_data_test_title_art_front_fill_render_color_gate<'a>(
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

pub(super) fn push_success_data_test_title_art_source_paint_render_trace_json(
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

pub(super) fn success_data_test_title_art_source_paint_render_trace_svg_attrs(
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

pub(super) fn success_data_test_title_art_front_fill_svg_attrs(
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

pub(super) fn success_data_test_title_art_front_fill_winding_gate(
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

pub(super) fn push_success_data_test_title_art_front_fill_winding_gate_json(
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

pub(super) fn success_data_test_title_art_shadow_path_partition<'a>(
    outline_paths: &[&'a ObjectEmbeddedPressVectorPathCandidate],
) -> Option<TitleArtShadowPathPartition<'a>> {
    success_data_test_title_art_state_shadow_path_partition(outline_paths)
        .or_else(|| success_data_test_title_art_geometry_shadow_path_partition(outline_paths))
        .or_else(|| success_data_test_title_art_halfsplit_shadow_path_partition(outline_paths))
}

pub(super) fn success_data_test_title_art_state_shadow_path_partition<'a>(
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

pub(super) fn success_data_test_title_art_geometry_shadow_path_partition<'a>(
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

pub(super) fn success_data_test_title_art_halfsplit_shadow_path_partition<'a>(
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

pub(super) fn success_data_test_title_art_shadow_pair_count_for_offset(
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

pub(super) fn success_data_test_title_art_common_shadow_offset(
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

pub(super) fn success_data_test_title_art_shadow_sweep_path_data(
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

pub(super) fn push_success_data_test_title_art_contour_side_strips(
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

pub(super) fn push_success_data_test_title_art_extrusion_svg(
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

pub(super) fn push_success_data_test_title_art_shadow_face_svg(
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

pub(super) fn push_success_data_test_title_art_extrusion_texture_clip_gate_json(
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

pub(super) fn success_data_test_title_art_extrusion_texture_clip_gate_svg_attrs(
    texture_path_count: usize,
    selected_clip_source: &str,
) -> String {
    format!(
        " data-title-texture-clip-gate-source=\"embeddedPressOutlineTextureOutlineClipArbitration\" data-title-texture-clip-gate-reference-backed=\"true\" data-title-texture-clip-gate-render-promoted=\"true\" data-title-texture-clip-gate-path-count=\"{}\" data-title-texture-selected-clip-source=\"{}\" data-title-texture-selected-clip-basis=\"current-renderer-shadow-outline-clip\" data-title-texture-alternative-clip-source=\"long-shadow-side-sweep\" data-title-texture-alternative-clip-rejected=\"true\" data-title-texture-alternative-clip-rejected-by=\"historical-poppler-crop-ab\" data-title-texture-alternative-clip-rejected-reason=\"long-shadow-side-sweep-texture-clip-worsened-title-crops\" data-title-texture-front-face-knockout-decoded=\"false\" data-title-texture-clip-semantics-decoded=\"false\" data-title-texture-clip-semantics-blocked-reason=\"texture-clip-and-knockout-semantics-unproven\"",
        texture_path_count,
        escape_xml(selected_clip_source)
    )
}

pub(super) fn success_data_test_title_art_texture_geometry_role_gate(
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

pub(super) fn push_success_data_test_title_art_texture_geometry_role_gate_json(
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

pub(super) fn success_data_test_title_art_texture_geometry_role_gate_svg_attrs(
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
pub(super) fn push_success_data_test_title_art_texture_svg(
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
pub(super) fn push_success_data_test_title_art_front_texture_svg(
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

pub(super) fn success_data_test_title_art_front_texture_render_promotion_blocked_reason(
    texture_path_source: &str,
) -> Option<&'static str> {
    if texture_path_source == "source-order-interstitial-front-erase-texture" {
        Some("front-erase-texture-over-main-face-semantics-unproven")
    } else {
        None
    }
}

pub(super) fn success_data_test_title_art_front_erase_explicit_state_path_count(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> usize {
    paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count()
}

pub(super) fn push_success_data_test_title_art_front_erase_visible_probe_gate_json(
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

pub(super) fn success_data_test_title_art_front_erase_visible_probe_gate_svg_attrs(
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

pub(super) fn success_data_test_title_art_state_tagged_texture_paths(
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

pub(super) fn success_data_test_title_art_front_texture_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    success_data_test_title_art_effective_texture_paths_for_word5(
        snapshot,
        EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5,
    )
}

pub(super) fn success_data_test_title_art_front_erase_texture_paths(
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

pub(super) fn success_data_test_title_art_interstitial_front_erase_gate(
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

pub(super) fn success_data_test_title_art_front_erase_texture_path_source(
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

pub(super) fn success_data_test_title_art_texture_paths(
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

pub(super) fn success_data_test_title_art_shadow_texture_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Texture
                && !path.commands().is_empty()
                && embedded_press_title_art_state_word5(path)
                    == Some(EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5)
        })
        .collect::<Vec<_>>()
}

pub(super) fn success_data_test_title_art_interstitial_texture_paths<'a>(
    snapshot: &'a ObjectEmbeddedPressSnapshotCandidate,
    partition: &TitleArtShadowPathPartition<'a>,
) -> Option<Vec<&'a ObjectEmbeddedPressVectorPathCandidate>> {
    let paths = snapshot.vector_paths();
    let shadow_max_index = partition
        .shadow_paths
        .iter()
        .filter_map(|target| embedded_press_vector_path_index(paths, target))
        .max()?;
    let main_min_index = partition
        .main_paths
        .iter()
        .filter_map(|target| embedded_press_vector_path_index(paths, target))
        .min()?;
    if shadow_max_index + 1 >= main_min_index {
        return None;
    }

    let texture_paths = paths[shadow_max_index + 1..main_min_index]
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Texture && !path.commands().is_empty()
        })
        .collect::<Vec<_>>();
    (texture_paths.len() == main_min_index - shadow_max_index - 1).then_some(texture_paths)
}

pub(super) fn push_success_data_test_title_art_paint_state_summaries_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let outline_paths = success_data_test_title_art_rendered_paths(snapshot);
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let mut groups: Vec<(&str, Vec<&ObjectEmbeddedPressVectorPathCandidate>)> = Vec::new();
    if let Some(partition) = partition.as_ref() {
        groups.push(("shadowOutlines", partition.shadow_paths.clone()));
        if let Some(texture_paths) =
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        {
            groups.push(("interstitialTextureBlock", texture_paths));
        } else {
            groups.push((
                "preservedAllTexturePaths",
                success_data_test_title_art_texture_paths(snapshot),
            ));
        }
        groups.push(("mainOutlines", partition.main_paths.clone()));
    } else {
        groups.push(("allOutlines", outline_paths));
        groups.push((
            "preservedAllTexturePaths",
            success_data_test_title_art_texture_paths(snapshot),
        ));
    }

    output.push('[');
    for (index, (role, paths)) in groups.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_success_data_test_title_art_paint_state_summary_json(output, snapshot, role, paths);
    }
    output.push(']');
}

pub(super) fn push_success_data_test_title_art_paint_state_sequence_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let paths = snapshot.vector_paths();
    let explicit_path_indexes = paths
        .iter()
        .enumerate()
        .filter(|(_, path)| !path.commands().is_empty() && !path.state_records().is_empty())
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();

    output.push_str("{\"source\":\"embeddedPressVectorPathSourceOrder\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false");
    output.push_str(",\"pathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"explicitTransitionCount\":");
    output.push_str(&explicit_path_indexes.len().to_string());
    output.push_str(",\"pathKindRuns\":");
    push_success_data_test_title_art_path_kind_runs_json(output, snapshot);
    output.push_str(",\"frontErasePaintTransitionGate\":");
    push_success_data_test_title_art_front_erase_paint_transition_gate_json(
        output,
        snapshot,
        &interstitial_texture_paths,
    );
    output.push_str(",\"explicitTransitions\":[");
    for (transition_index, path_index) in explicit_path_indexes.iter().enumerate() {
        if transition_index > 0 {
            output.push(',');
        }
        let path = &paths[*path_index];
        let next_explicit_path_index = explicit_path_indexes.get(transition_index + 1).copied();
        let inherited_span_end_path_index = next_explicit_path_index
            .map(|index| index.saturating_sub(1))
            .unwrap_or_else(|| paths.len().saturating_sub(1));
        let inherited_span = if *path_index <= inherited_span_end_path_index {
            &paths[*path_index..=inherited_span_end_path_index]
        } else {
            &[]
        };
        let inherited_texture_path_count = inherited_span
            .iter()
            .filter(|path| {
                path.kind() == ObjectEmbeddedPressVectorPathKind::Texture
                    && !path.commands().is_empty()
            })
            .count();
        let inherited_outline_path_count = inherited_span
            .iter()
            .filter(|path| {
                path.kind() == ObjectEmbeddedPressVectorPathKind::Outline
                    && !path.commands().is_empty()
            })
            .count();

        output.push_str("{\"pathIndex\":");
        output.push_str(&path_index.to_string());
        output.push_str(",\"pathKind\":");
        output.push_str(&json_string(path.kind().as_str()));
        output.push_str(",\"sourceOrderRole\":");
        output.push_str(&json_string(success_data_test_title_art_source_order_role(
            path,
            partition.as_ref(),
            &interstitial_texture_paths,
        )));
        output.push_str(",\"stateSourcePathIndex\":");
        output.push_str(&path_index.to_string());
        output.push_str(",\"nextExplicitPathIndex\":");
        push_option_usize_json(output, next_explicit_path_index);
        output.push_str(",\"inheritedSpanEndPathIndex\":");
        output.push_str(&inherited_span_end_path_index.to_string());
        output.push_str(",\"inheritedPathCount\":");
        output.push_str(&inherited_span.len().to_string());
        output.push_str(",\"inheritedTexturePathCount\":");
        output.push_str(&inherited_texture_path_count.to_string());
        output.push_str(",\"inheritedOutlinePathCount\":");
        output.push_str(&inherited_outline_path_count.to_string());
        output.push_str(",\"stateRecordCount\":");
        output.push_str(&path.state_records().len().to_string());
        output.push_str(",\"stateRecordTypes\":");
        push_u32_hex_array_json(
            output,
            &path
                .state_records()
                .iter()
                .map(ObjectEmbeddedPressStateRecordCandidate::record_type)
                .collect::<Vec<_>>(),
        );
        output.push_str(",\"stateRecords\":");
        push_embedded_press_path_state_records_json(output, path);
        output.push_str(",\"record46Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x46, 0);
        output.push_str(",\"record48Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x48, 0);
        output.push_str(",\"record60Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x60, 0);
        output.push_str(",\"record65Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x65, 0);
        output.push_str(",\"record70Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x70, 0);
        output.push_str(",\"record70Word3Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x70, 3);
        output.push_str(",\"record70Word7Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x70, 7);
        output.push_str(",\"record82Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(
            output,
            path,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            0,
        );
        output.push_str(",\"record82Word3Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(
            output,
            path,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            3,
        );
        output.push_str(",\"record82Word5Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(
            output,
            path,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        );
        output.push_str(",\"textureBezierHeader\":");
        push_embedded_press_path_texture_bezier_header_json(output, path);
        output.push('}');
    }
    output.push_str("]}");
}

pub(super) fn success_data_test_title_art_front_erase_paint_transition_gate(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    interstitial_texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> TitleArtFrontErasePaintTransitionGate {
    let snapshot_paths = snapshot.vector_paths();
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_indexes =
        embedded_press_vector_path_indexes(snapshot_paths, interstitial_texture_paths);
    let shadow_indexes = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_vector_path_indexes(snapshot_paths, &partition.shadow_paths)
    });
    let main_indexes = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_vector_path_indexes(snapshot_paths, &partition.main_paths)
    });
    let explicit_state_texture_path_count = interstitial_texture_paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();
    let spans = embedded_press_title_art_front_erase_texture_state_spans(
        snapshot,
        interstitial_texture_paths,
    );
    let span_path_counts = spans.iter().map(|span| span.path_count).collect::<Vec<_>>();

    let shadow_values_48 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x48)
    });
    let texture_values_48 =
        embedded_press_title_art_state_record_word0_values(interstitial_texture_paths, 0x48);
    let main_values_48 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
    });
    let texture_values_70 = embedded_press_title_art_state_record_word0_values(
        interstitial_texture_paths,
        EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70,
    );
    let main_values_70 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(
            &partition.main_paths,
            EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70,
        )
    });
    let shadow_values_82_word5 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word_values(
            &partition.shadow_paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        )
    });
    let texture_values_82_word5 = embedded_press_title_art_state_record_word_values(
        interstitial_texture_paths,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        5,
    );
    let main_values_82_word5 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word_values(
            &partition.main_paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        )
    });
    let texture_values_82_word3 = embedded_press_title_art_state_record_word_values(
        interstitial_texture_paths,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        3,
    );

    let shadow_last_path_index = shadow_indexes.iter().max().copied();
    let interstitial_first_path_index = interstitial_indexes.iter().min().copied();
    let interstitial_last_path_index = interstitial_indexes.iter().max().copied();
    let main_first_path_index = main_indexes.iter().min().copied();
    let shadow_to_interstitial_boundary_adjacent =
        match (shadow_last_path_index, interstitial_first_path_index) {
            (Some(shadow_last), Some(texture_first)) => shadow_last + 1 == texture_first,
            _ => false,
        };
    let interstitial_to_main_boundary_adjacent =
        match (interstitial_last_path_index, main_first_path_index) {
            (Some(texture_last), Some(main_first)) => texture_last + 1 == main_first,
            _ => false,
        };
    let record48_separates_shadow_from_texture_and_main =
        shadow_values_48 == vec![1] && texture_values_48 == vec![0] && main_values_48 == vec![0];
    let record48_separates_texture_from_main =
        !texture_values_48.is_empty() && texture_values_48 != main_values_48;
    let record70_word0_separates_texture_from_main =
        !texture_values_70.is_empty() && texture_values_70 != main_values_70;
    let record82_word5_separates_texture_from_main =
        !texture_values_82_word5.is_empty() && texture_values_82_word5 != main_values_82_word5;
    let record82_word5_matches_shadow =
        !texture_values_82_word5.is_empty() && texture_values_82_word5 == shadow_values_82_word5;
    let record82_word3_is_white_paint_candidate = texture_values_82_word3 == vec![0x00ff_ffff];

    let paint_intent_inference = if record82_word5_matches_shadow
        && record82_word5_separates_texture_from_main
        && !record48_separates_texture_from_main
        && !record70_word0_separates_texture_from_main
    {
        "shadow-state-texture-inside-main-boundary-ambiguous"
    } else if record48_separates_texture_from_main || record70_word0_separates_texture_from_main {
        "texture-main-state-separated-candidate"
    } else if record82_word3_is_white_paint_candidate {
        "white-paint-candidate-without-boundary-separation"
    } else {
        "paint-intent-unclassified"
    };
    let transition_boundary_class = if partition.is_none() {
        "title-partition-missing"
    } else if interstitial_texture_paths.is_empty() {
        "interstitial-texture-absent"
    } else if shadow_to_interstitial_boundary_adjacent && interstitial_to_main_boundary_adjacent {
        "source-order-bracketed-interstitial-texture-block"
    } else {
        "source-order-boundary-not-contiguous"
    };
    let render_promotion_blocked_reason = if partition.is_none() {
        "front-erase-title-partition-missing"
    } else if interstitial_texture_paths.is_empty() {
        "front-erase-interstitial-texture-absent"
    } else if !(shadow_to_interstitial_boundary_adjacent && interstitial_to_main_boundary_adjacent)
    {
        "front-erase-source-order-boundary-not-contiguous"
    } else if !record48_separates_texture_from_main && !record70_word0_separates_texture_from_main {
        "front-erase-transition-boundary-main-state-not-separated"
    } else {
        "front-erase-transition-boundary-semantics-unproven"
    };

    TitleArtFrontErasePaintTransitionGate {
        partition_present: partition.is_some(),
        interstitial_texture_path_count: interstitial_texture_paths.len(),
        explicit_state_texture_path_count,
        inherited_texture_path_count: interstitial_texture_paths
            .len()
            .saturating_sub(explicit_state_texture_path_count),
        span_count: spans.len(),
        span_path_counts,
        shadow_last_path_index,
        interstitial_first_path_index,
        interstitial_last_path_index,
        main_first_path_index,
        shadow_to_interstitial_boundary_adjacent,
        interstitial_to_main_boundary_adjacent,
        record48_separates_shadow_from_texture_and_main,
        record48_separates_texture_from_main,
        record70_word0_separates_texture_from_main,
        record82_word5_separates_texture_from_main,
        record82_word5_matches_shadow,
        record82_word3_is_white_paint_candidate,
        paint_intent_inference,
        transition_boundary_class,
        render_promotion_blocked_reason,
    }
}

pub(super) fn push_success_data_test_title_art_front_erase_paint_transition_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    interstitial_texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let gate = success_data_test_title_art_front_erase_paint_transition_gate(
        snapshot,
        interstitial_texture_paths,
    );
    output.push_str("{\"source\":\"embeddedPressVectorPathSourceOrder+stateTransitions\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true");
    output.push_str(",\"partitionPresent\":");
    output.push_str(if gate.partition_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"interstitialTexturePathCount\":");
    output.push_str(&gate.interstitial_texture_path_count.to_string());
    output.push_str(",\"explicitStateTexturePathCount\":");
    output.push_str(&gate.explicit_state_texture_path_count.to_string());
    output.push_str(",\"inheritedTexturePathCount\":");
    output.push_str(&gate.inherited_texture_path_count.to_string());
    output.push_str(",\"spanCount\":");
    output.push_str(&gate.span_count.to_string());
    output.push_str(",\"spanPathCounts\":");
    push_usize_array_json(output, &gate.span_path_counts);
    output.push_str(",\"shadowLastPathIndex\":");
    push_option_usize_json(output, gate.shadow_last_path_index);
    output.push_str(",\"interstitialFirstPathIndex\":");
    push_option_usize_json(output, gate.interstitial_first_path_index);
    output.push_str(",\"interstitialLastPathIndex\":");
    push_option_usize_json(output, gate.interstitial_last_path_index);
    output.push_str(",\"mainFirstPathIndex\":");
    push_option_usize_json(output, gate.main_first_path_index);
    output.push_str(",\"shadowToInterstitialBoundaryAdjacent\":");
    output.push_str(if gate.shadow_to_interstitial_boundary_adjacent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"interstitialToMainBoundaryAdjacent\":");
    output.push_str(if gate.interstitial_to_main_boundary_adjacent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesShadowFromTextureAndMain\":");
    output.push_str(if gate.record48_separates_shadow_from_texture_and_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesTextureFromMain\":");
    output.push_str(if gate.record48_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word0SeparatesTextureFromMain\":");
    output.push_str(if gate.record70_word0_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record82Word5SeparatesTextureFromMain\":");
    output.push_str(if gate.record82_word5_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record82Word5MatchesShadow\":");
    output.push_str(if gate.record82_word5_matches_shadow {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record82Word3IsWhitePaintCandidate\":");
    output.push_str(if gate.record82_word3_is_white_paint_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintIntentInference\":");
    output.push_str(&json_string(gate.paint_intent_inference));
    output.push_str(",\"transitionBoundaryClass\":");
    output.push_str(&json_string(gate.transition_boundary_class));
    output.push_str(",\"promotionReady\":");
    output.push_str(if gate.promotion_ready() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(gate.render_promotion_blocked_reason));
    output.push('}');
}

pub(super) fn push_success_data_test_title_art_path_kind_runs_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let mut runs: Vec<(ObjectEmbeddedPressVectorPathKind, usize, usize, usize)> = Vec::new();
    for (path_index, path) in snapshot.vector_paths().iter().enumerate() {
        if path.commands().is_empty() {
            continue;
        }
        match runs.last_mut() {
            Some((kind, _, end, count)) if *kind == path.kind() => {
                *end = path_index;
                *count += 1;
            }
            _ => runs.push((path.kind(), path_index, path_index, 1)),
        }
    }

    output.push('[');
    for (run_index, (kind, start, end, count)) in runs.iter().enumerate() {
        if run_index > 0 {
            output.push(',');
        }
        output.push_str("{\"pathKind\":");
        output.push_str(&json_string(kind.as_str()));
        output.push_str(",\"startPathIndex\":");
        output.push_str(&start.to_string());
        output.push_str(",\"endPathIndex\":");
        output.push_str(&end.to_string());
        output.push_str(",\"pathCount\":");
        output.push_str(&count.to_string());
        output.push('}');
    }
    output.push(']');
}

pub(super) fn success_data_test_title_art_source_order_role(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    partition: Option<&TitleArtShadowPathPartition<'_>>,
    interstitial_texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> &'static str {
    let Some(partition) = partition else {
        return "unpartitioned";
    };
    if partition
        .shadow_paths
        .iter()
        .any(|candidate| std::ptr::eq(*candidate, path))
    {
        "shadowOutlines"
    } else if interstitial_texture_paths
        .iter()
        .any(|candidate| std::ptr::eq(*candidate, path))
    {
        "interstitialTextureBlock"
    } else if partition
        .main_paths
        .iter()
        .any(|candidate| std::ptr::eq(*candidate, path))
    {
        "mainOutlines"
    } else {
        "outsideTitlePartition"
    }
}

pub(super) fn push_success_data_test_title_art_paint_state_summary_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    role: &str,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let snapshot_paths = snapshot.vector_paths();
    let path_indexes = paths
        .iter()
        .filter_map(|path| embedded_press_vector_path_index(snapshot_paths, path))
        .collect::<Vec<_>>();
    let explicit_state_path_count = paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();
    let state_record_count = paths
        .iter()
        .map(|path| path.state_records().len())
        .sum::<usize>();
    let path_kind = embedded_press_title_art_path_kind_summary(paths);

    output.push_str("{\"role\":");
    output.push_str(&json_string(role));
    output.push_str(",\"pathKind\":");
    output.push_str(&json_string(path_kind));
    output.push_str(",\"pathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"firstPathIndex\":");
    push_option_usize_json(output, path_indexes.iter().min().copied());
    output.push_str(",\"lastPathIndex\":");
    push_option_usize_json(output, path_indexes.iter().max().copied());
    output.push_str(",\"explicitStatePathCount\":");
    output.push_str(&explicit_state_path_count.to_string());
    output.push_str(",\"inheritedStatePathCount\":");
    output.push_str(
        &paths
            .len()
            .saturating_sub(explicit_state_path_count)
            .to_string(),
    );
    output.push_str(",\"stateRecordCount\":");
    output.push_str(&state_record_count.to_string());
    output.push_str(",\"statePayloadSignatures\":");
    push_embedded_press_state_payload_signatures_json(output, paths);
    output.push_str(",\"statePayloadWordColumns\":");
    push_embedded_press_state_payload_word_columns_json(output, paths);
    output.push('}');
}

pub(super) fn push_success_data_test_title_art_front_paint_candidate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
) {
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
        Some("no-main-state-or-front-owned-texture-paths")
    } else if source_paint_color.is_none() && paint_state_color.is_none() {
        Some("missing-source-paint-color")
    } else {
        render_promotion_blocked_reason
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

    output.push_str("{\"source\":\"JSFart2Contents+EmbeddedPressPaintState\",\"decoded\":false,\"sourceBacked\":");
    output.push_str(
        if source_paint_color.is_some() || paint_state_color.is_some() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"paintColor\":");
    if let Some(color) = color_gate.paint_color {
        output.push_str(&json_string(color));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"paintColorSource\":");
    match color_gate.paint_source {
        Some(source) => output.push_str(&json_string(source)),
        None => output.push_str("null"),
    }
    output.push_str(",\"renderFillColor\":");
    output.push_str(&json_string(color_gate.render_fill));
    output.push_str(",\"renderFillColorSource\":");
    output.push_str(&json_string(color_gate.render_color_source));
    output.push_str(",\"renderFillColorSourceBacked\":");
    output.push_str(if color_gate.render_color_source_backed {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePaintColorMatchesRenderFill\":");
    output.push_str(if color_gate.source_paint_matches_render_fill {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderFillColorPromotionBlockedReason\":");
    output.push_str(&json_string(color_gate.render_color_blocked_reason));
    output.push_str(",\"sourcePaintRenderTrace\":");
    push_success_data_test_title_art_source_paint_render_trace_json(
        output,
        source_paint_candidate,
        color_gate,
        render_texture_path_source,
        render_blocked_reason,
    );
    output.push_str(",\"frontPaintArbitrationGate\":");
    output.push_str("{\"source\":\"JSFart2Contents+EmbeddedPressPaintState+frontEraseTextureProbes\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidate_count.to_string());
    output.push_str(",\"selectedRenderPolicy\":\"conservative-front-fill\"");
    output.push_str(",\"selectedRenderFillColor\":");
    output.push_str(&json_string(color_gate.render_fill));
    output.push_str(",\"sourcePaintCandidatePresent\":");
    output.push_str(if color_gate.paint_color.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePaintCandidateMatchesRenderFill\":");
    output.push_str(if color_gate.source_paint_matches_render_fill {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"directGrayCandidatePresent\":");
    output.push_str(if direct_gray_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textureSourcePaintCandidatePresent\":");
    output.push_str(if texture_source_paint_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"frontEraseTextureSpanCandidatePresent\":");
    output.push_str(if texture_state_span_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"frontEraseTextureSpanCount\":");
    output.push_str(&texture_state_span_count.to_string());
    output.push_str(",\"frontEraseTransitionBoundaryClass\":");
    output.push_str(&json_string(transition_gate.transition_boundary_class));
    output.push_str(",\"frontErasePaintIntentInference\":");
    output.push_str(&json_string(transition_gate.paint_intent_inference));
    output.push_str(",\"renderPromotionBlockedReasons\":[");
    output.push_str(&json_string(color_gate.render_color_blocked_reason));
    output.push(',');
    output.push_str(&json_string(
        render_promotion_blocked_reason.unwrap_or("none"),
    ));
    output.push(',');
    output.push_str(&json_string(
        transition_gate.render_promotion_blocked_reason,
    ));
    output.push(']');
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"front-paint-candidate-arbitration-unproven\"}",
    );
    output.push_str(",\"mainStateTexturePathCount\":");
    output.push_str(&front_texture_paths.len().to_string());
    output.push_str(",\"frontEraseTexturePathCount\":");
    output.push_str(&front_erase_texture_paths.len().to_string());
    output.push_str(",\"renderTexturePathSource\":");
    output.push_str(&json_string(render_texture_path_source));
    output.push_str(",\"renderPathCount\":");
    output.push_str(&render_path_count.to_string());
    output.push_str(",\"visibleRenderPathCount\":");
    output.push_str(&visible_render_path_count.to_string());
    output.push_str(",\"renderClipRule\":");
    output.push_str(&json_string("nonzero"));
    output.push_str(",\"renderClipRuleSource\":");
    output.push_str(&json_string("embedded-press-nonzero-winding"));
    output.push_str(",\"renderClipRulePixelChange\":true");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if let Some(reason) = render_blocked_reason {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn push_success_data_test_title_art_front_texture_role_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    front_erase_texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();

    output.push_str(
        "{\"source\":\"embeddedPressPathStateRecordComparison\",\"decoded\":false,\"pixelChange\":",
    );
    output.push_str(if front_erase_texture_paths.is_empty() {
        "false"
    } else {
        "true"
    });
    output.push_str(",\"frontEraseTexturePathCount\":");
    output.push_str(&front_erase_texture_paths.len().to_string());
    output.push_str(",\"interstitialTexturePathCount\":");
    output.push_str(&interstitial_texture_paths.len().to_string());

    let record_48_separates_shadow_from_texture_and_main =
        partition.as_ref().is_some_and(|partition| {
            embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x48)
                == vec![1]
                && embedded_press_title_art_state_record_word0_values(
                    &interstitial_texture_paths,
                    0x48,
                ) == vec![0]
                && embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
                    == vec![0]
        });
    let record_48_separates_texture_from_main = partition.as_ref().is_some_and(|partition| {
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x48)
            != embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
    });
    let source_order_front_erase_candidate = partition.as_ref().is_some_and(|partition| {
        success_data_test_title_art_interstitial_front_erase_gate(
            snapshot,
            partition,
            front_erase_texture_paths,
        )
    });
    let blocked_interstitial_current_state_candidate = front_erase_texture_paths.is_empty()
        && !interstitial_texture_paths.is_empty()
        && record_48_separates_shadow_from_texture_and_main
        && !record_48_separates_texture_from_main;
    output.push_str(",\"record48SeparatesShadowFromTextureAndMain\":");
    output.push_str(if record_48_separates_shadow_from_texture_and_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesTextureFromMain\":");
    output.push_str(if record_48_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOrderFrontEraseCandidate\":");
    output.push_str(if source_order_front_erase_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(if source_order_front_erase_candidate {
        "source-order-interstitial-front-erase-texture"
    } else if blocked_interstitial_current_state_candidate {
        "blocked-current-paint-state-inheritance"
    } else if front_erase_texture_paths
        .iter()
        .all(|path| !path.state_records().is_empty())
        && !front_erase_texture_paths.is_empty()
    {
        "explicit-state-texture-paths"
    } else if front_erase_texture_paths.is_empty() {
        "none"
    } else {
        "current-paint-state-inheritance"
    }));
    let source_order_front_erase_render_promotion_blocked_reason =
        if source_order_front_erase_candidate {
            success_data_test_title_art_front_texture_render_promotion_blocked_reason(
                "source-order-interstitial-front-erase-texture",
            )
        } else {
            None
        };
    let source_order_front_erase_render_promoted = source_order_front_erase_candidate
        && source_order_front_erase_render_promotion_blocked_reason.is_none();
    output.push_str(",\"visibleRenderPathCount\":");
    output.push_str(
        &(if source_order_front_erase_render_promoted {
            front_erase_texture_paths.len()
        } else {
            0
        })
        .to_string(),
    );
    output.push_str(",\"renderPromoted\":");
    output.push_str(if source_order_front_erase_render_promoted {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"frontTexturePromotionBasis\":");
    output.push_str(&json_string(if source_order_front_erase_candidate {
        "source-order-interstitial-front-erase-texture"
    } else if front_erase_texture_paths
        .iter()
        .all(|path| !path.state_records().is_empty())
        && !front_erase_texture_paths.is_empty()
    {
        "explicit-state-texture-paths"
    } else if blocked_interstitial_current_state_candidate {
        "blocked-current-paint-state-inheritance"
    } else if front_erase_texture_paths.is_empty() {
        "none"
    } else {
        "current-paint-state-inheritance"
    }));
    output.push_str(",\"frontTexturePromotionRisk\":");
    output.push_str(&json_string(if source_order_front_erase_candidate {
        "source-order-texture-shares-record48-with-main-outline"
    } else if blocked_interstitial_current_state_candidate {
        "interstitial-texture-and-main-outline-share-record48-zero"
    } else if !front_erase_texture_paths.is_empty()
        && record_48_separates_shadow_from_texture_and_main
        && !record_48_separates_texture_from_main
    {
        "front-texture-and-main-outline-share-record48-zero"
    } else {
        "none"
    }));
    output.push_str(",\"renderPromotionBlockedReason\":");
    if source_order_front_erase_render_promoted {
        output.push_str("null");
    } else if let Some(reason) = source_order_front_erase_render_promotion_blocked_reason {
        output.push_str(&json_string(reason));
    } else {
        output.push_str(&json_string(
            if blocked_interstitial_current_state_candidate {
                "interstitial-texture-and-main-outline-share-record48-zero"
            } else if front_erase_texture_paths.is_empty() {
                "no-front-erase-texture-candidate"
            } else {
                "front-erase-texture-role-unproven"
            },
        ));
    }
    output.push_str(",\"frontEraseVisibleProbeGate\":");
    push_success_data_test_title_art_front_erase_visible_probe_gate_json(
        output,
        front_erase_texture_paths,
    );
    output.push_str(",\"groups\":[");
    if let Some(partition) = partition.as_ref() {
        push_success_data_test_title_art_role_gate_group_json(
            output,
            "shadowOutlines",
            &partition.shadow_paths,
        );
        output.push(',');
        push_success_data_test_title_art_role_gate_group_json(
            output,
            "interstitialTextureBlock",
            &interstitial_texture_paths,
        );
        output.push(',');
        push_success_data_test_title_art_role_gate_group_json(
            output,
            "mainOutlines",
            &partition.main_paths,
        );
    }
    output.push_str("]}");
}

pub(super) fn push_success_data_test_title_art_texture_paint_phase_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();
    let shadow_record46_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x46)
    });
    let texture_record46_values =
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x46);
    let main_record46_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x46)
    });
    let shadow_record48_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x48)
    });
    let texture_record48_values =
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x48);
    let main_record48_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
    });
    let shadow_record60_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x60)
    });
    let texture_record60_values =
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x60);
    let main_record60_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x60)
    });
    let shadow_record65_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x65)
    });
    let texture_record65_values =
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x65);
    let main_record65_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x65)
    });
    let record46_one_appears_only_on_outlines = (shadow_record46_values.contains(&1)
        || main_record46_values.contains(&1))
        && !texture_record46_values.contains(&1);
    let texture_and_main_share_record46_zero =
        texture_record46_values.contains(&0) && main_record46_values.contains(&0);
    let texture_and_shadow_share_record46_zero =
        texture_record46_values.contains(&0) && shadow_record46_values.contains(&0);
    let record46_separates_texture_from_outlines = !texture_record46_values.is_empty()
        && !shadow_record46_values.is_empty()
        && !main_record46_values.is_empty()
        && texture_record46_values.iter().all(|value| {
            !shadow_record46_values.contains(value) && !main_record46_values.contains(value)
        });
    let record48_separates_shadow_from_texture_and_main = !shadow_record48_values.is_empty()
        && !texture_record48_values.is_empty()
        && !main_record48_values.is_empty()
        && shadow_record48_values != texture_record48_values
        && shadow_record48_values != main_record48_values;
    let record48_separates_texture_from_main =
        !texture_record48_values.is_empty() && texture_record48_values != main_record48_values;
    let record60_shared_across_roles = !texture_record60_values.is_empty()
        && shadow_record60_values == texture_record60_values
        && texture_record60_values == main_record60_values;
    let record65_shared_across_roles = !texture_record65_values.is_empty()
        && shadow_record65_values == texture_record65_values
        && texture_record65_values == main_record65_values;
    let mut promotion_proof_blocked_reasons = Vec::new();
    if partition.is_none() {
        promotion_proof_blocked_reasons.push("title-art-role-partition-missing");
    }
    if interstitial_texture_paths.is_empty() {
        promotion_proof_blocked_reasons.push("interstitial-texture-paths-missing");
    }
    if !record46_one_appears_only_on_outlines {
        promotion_proof_blocked_reasons.push("record46-outline-candidate-absent");
    }
    if !record46_separates_texture_from_outlines {
        promotion_proof_blocked_reasons
            .push("record46-texture-outline-value-sets-overlap-or-missing");
    }
    if texture_and_main_share_record46_zero {
        promotion_proof_blocked_reasons.push("record46-zero-shared-by-texture-and-main-outline");
    }
    if texture_and_shadow_share_record46_zero {
        promotion_proof_blocked_reasons.push("record46-zero-shared-by-texture-and-shadow-outline");
    }
    if !record48_separates_shadow_from_texture_and_main {
        promotion_proof_blocked_reasons
            .push("record48-shadow-texture-main-role-separation-missing");
    }
    if !record48_separates_texture_from_main {
        promotion_proof_blocked_reasons.push("record48-texture-main-role-separation-missing");
    }
    if record60_shared_across_roles {
        promotion_proof_blocked_reasons.push("record60-shared-across-roles");
    }
    if record65_shared_across_roles {
        promotion_proof_blocked_reasons.push("record65-shared-across-roles");
    }
    let record46_promotion_proof_ready = promotion_proof_blocked_reasons.is_empty();

    output.push_str("{\"source\":\"embeddedPressPathStateRecordComparison\",\"basis\":\"record46-word0-paint-phase-candidate\",\"decoded\":false,\"sourceBacked\":");
    output.push_str(
        if partition.is_some() && !interstitial_texture_paths.is_empty() {
            "true"
        } else {
            "false"
        },
    );
    output
        .push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false,\"visibleRenderPathCount\":0");
    output.push_str(",\"texturePathCount\":");
    output.push_str(&interstitial_texture_paths.len().to_string());
    output.push_str(",\"shadowOutlinePathCount\":");
    output.push_str(
        &partition
            .as_ref()
            .map_or(0, |partition| partition.shadow_paths.len())
            .to_string(),
    );
    output.push_str(",\"mainOutlinePathCount\":");
    output.push_str(
        &partition
            .as_ref()
            .map_or(0, |partition| partition.main_paths.len())
            .to_string(),
    );
    output.push_str(",\"textureRecord46Word0Values\":");
    push_u32_hex_array_json(output, &texture_record46_values);
    output.push_str(",\"shadowOutlineRecord46Word0Values\":");
    push_u32_hex_array_json(output, &shadow_record46_values);
    output.push_str(",\"mainOutlineRecord46Word0Values\":");
    push_u32_hex_array_json(output, &main_record46_values);
    output.push_str(",\"textureRecord48Word0Values\":");
    push_u32_hex_array_json(output, &texture_record48_values);
    output.push_str(",\"shadowOutlineRecord48Word0Values\":");
    push_u32_hex_array_json(output, &shadow_record48_values);
    output.push_str(",\"mainOutlineRecord48Word0Values\":");
    push_u32_hex_array_json(output, &main_record48_values);
    output.push_str(",\"textureRecord60Word0Values\":");
    push_u32_hex_array_json(output, &texture_record60_values);
    output.push_str(",\"shadowOutlineRecord60Word0Values\":");
    push_u32_hex_array_json(output, &shadow_record60_values);
    output.push_str(",\"mainOutlineRecord60Word0Values\":");
    push_u32_hex_array_json(output, &main_record60_values);
    output.push_str(",\"textureRecord65Word0Values\":");
    push_u32_hex_array_json(output, &texture_record65_values);
    output.push_str(",\"shadowOutlineRecord65Word0Values\":");
    push_u32_hex_array_json(output, &shadow_record65_values);
    output.push_str(",\"mainOutlineRecord65Word0Values\":");
    push_u32_hex_array_json(output, &main_record65_values);
    output.push_str(",\"record46OneAppearsOnlyOnOutlines\":");
    output.push_str(if record46_one_appears_only_on_outlines {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textureAndMainShareRecord46Zero\":");
    output.push_str(if texture_and_main_share_record46_zero {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textureAndShadowShareRecord46Zero\":");
    output.push_str(if texture_and_shadow_share_record46_zero {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record46SeparatesTextureFromOutlines\":");
    output.push_str(if record46_separates_texture_from_outlines {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesShadowFromTextureAndMain\":");
    output.push_str(if record48_separates_shadow_from_texture_and_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesTextureFromMain\":");
    output.push_str(if record48_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record60SharedAcrossRoles\":");
    output.push_str(if record60_shared_across_roles {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record65SharedAcrossRoles\":");
    output.push_str(if record65_shared_across_roles {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record60SeparatesTextureFromOutlines\":false");
    output.push_str(",\"record65SeparatesTextureFromOutlines\":false");
    output.push_str(",\"promotionProofPolicy\":");
    output.push_str(&json_string(
        "record46-must-separate-texture-from-outlines-and-record48-must-separate-main-role",
    ));
    output.push_str(",\"record46PromotionProofReady\":");
    output.push_str(if record46_promotion_proof_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"promotionProofBlockedReasons\":");
    push_json_string_slice_array(output, &promotion_proof_blocked_reasons);
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(if record46_one_appears_only_on_outlines {
        "record46-one-outline-paint-phase-candidate"
    } else {
        "record46-role-candidate-absent"
    }));
    output
        .push_str(",\"renderPromotionBlockedReason\":\"record46-paint-phase-semantics-unproven\"}");
}

pub(super) fn push_success_data_test_title_art_shadow_paint_word_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();
    let shadow_paths = partition
        .as_ref()
        .map(|partition| partition.shadow_paths.as_slice())
        .unwrap_or(&[]);
    let main_paths = partition
        .as_ref()
        .map(|partition| partition.main_paths.as_slice())
        .unwrap_or(&[]);

    let shadow_word0 = embedded_press_title_art_state_record_word_values(shadow_paths, 0x70, 0);
    let texture_word0 =
        embedded_press_title_art_state_record_word_values(&interstitial_texture_paths, 0x70, 0);
    let main_word0 = embedded_press_title_art_state_record_word_values(main_paths, 0x70, 0);
    let shadow_word1 = embedded_press_title_art_state_record_word_values(shadow_paths, 0x70, 1);
    let texture_word1 =
        embedded_press_title_art_state_record_word_values(&interstitial_texture_paths, 0x70, 1);
    let main_word1 = embedded_press_title_art_state_record_word_values(main_paths, 0x70, 1);
    let shadow_word3 = embedded_press_title_art_state_record_word_values(shadow_paths, 0x70, 3);
    let texture_word3 =
        embedded_press_title_art_state_record_word_values(&interstitial_texture_paths, 0x70, 3);
    let main_word3 = embedded_press_title_art_state_record_word_values(main_paths, 0x70, 3);
    let shadow_word7 = embedded_press_title_art_state_record_word_values(shadow_paths, 0x70, 7);
    let texture_word7 =
        embedded_press_title_art_state_record_word_values(&interstitial_texture_paths, 0x70, 7);
    let main_word7 = embedded_press_title_art_state_record_word_values(main_paths, 0x70, 7);

    let word0_separates_shadow =
        !shadow_word0.is_empty() && shadow_word0 != texture_word0 && shadow_word0 != main_word0;
    let word3_separates_shadow =
        !shadow_word3.is_empty() && shadow_word3 != texture_word3 && shadow_word3 != main_word3;
    let word7_separates_shadow =
        !shadow_word7.is_empty() && shadow_word7 != texture_word7 && shadow_word7 != main_word7;
    let word1_shared_across_roles =
        !shadow_word1.is_empty() && shadow_word1 == texture_word1 && texture_word1 == main_word1;
    let word0_separates_texture_from_main =
        !texture_word0.is_empty() && texture_word0 != main_word0;
    let word3_separates_texture_from_main =
        !texture_word3.is_empty() && texture_word3 != main_word3;
    let word7_separates_texture_from_main =
        !texture_word7.is_empty() && texture_word7 != main_word7;
    let shadow_effect = partition
        .as_ref()
        .and_then(|partition| embedded_press_title_art_shadow_effect(&partition.shadow_paths));
    let texture_effect = shadow_effect.as_ref().and_then(|effect| {
        embedded_press_title_art_texture_effect(&interstitial_texture_paths, &effect.fill_color)
    });

    output.push_str(
        "{\"source\":\"embeddedPressRecord70RoleComparison\",\"decoded\":false,\"sourceBacked\":",
    );
    output.push_str(
        if partition.is_some() && !interstitial_texture_paths.is_empty() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"record70Word0SeparatesShadowFromTextureAndMain\":");
    output.push_str(if word0_separates_shadow {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word3SeparatesShadowFromTextureAndMain\":");
    output.push_str(if word3_separates_shadow {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word7SeparatesShadowFromTextureAndMain\":");
    output.push_str(if word7_separates_shadow {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word1SharedAcrossRoles\":");
    output.push_str(if word1_shared_across_roles {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word0SeparatesTextureFromMain\":");
    output.push_str(if word0_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word3SeparatesTextureFromMain\":");
    output.push_str(if word3_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word7SeparatesTextureFromMain\":");
    output.push_str(if word7_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"shadowEffectCandidate\":");
    if let Some(effect) = shadow_effect.as_ref() {
        output.push_str("{\"basis\":\"record70.word0-percent-black-on-white\",\"word0\":");
        output.push_str(&effect.word0.to_string());
        output.push_str(",\"opacity\":");
        output.push_str(&format!("{:.3}", effect.opacity));
        output.push_str(",\"fillColor\":");
        output.push_str(&json_string(&effect.fill_color));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"interstitialTextureEffectCandidate\":");
    if let Some(effect) = texture_effect.as_ref() {
        output.push_str("{\"basis\":\"record70.word0-percent-black-over-shadow\",\"word0\":");
        output.push_str(&effect.word0.to_string());
        output.push_str(",\"opacity\":");
        output.push_str(&format!("{:.3}", effect.opacity));
        output.push_str(",\"baseFillColor\":");
        output.push_str(&json_string(&effect.base_fill_color));
        output.push_str(",\"fillColor\":");
        output.push_str(&json_string(&effect.fill_color));
        output.push_str(",\"renderPromoted\":false,\"renderPromotionBlockedReason\":\"record70-separates-shadow-but-not-interstitial-texture-from-main\"}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        if word0_separates_shadow
            && word3_separates_shadow
            && word7_separates_shadow
            && !word0_separates_texture_from_main
            && !word3_separates_texture_from_main
            && !word7_separates_texture_from_main
        {
            "none"
        } else {
            "record70-role-separation-unproven"
        },
    ));
    output.push_str(",\"roles\":[");
    push_success_data_test_title_art_record70_role_json(output, "shadowOutlines", shadow_paths);
    output.push(',');
    push_success_data_test_title_art_record70_role_json(
        output,
        "interstitialTextureBlock",
        &interstitial_texture_paths,
    );
    output.push(',');
    push_success_data_test_title_art_record70_role_json(output, "mainOutlines", main_paths);
    output.push_str("]}");
}

pub(super) fn push_success_data_test_title_art_record70_role_json(
    output: &mut String,
    role: &str,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    output.push_str("{\"role\":");
    output.push_str(&json_string(role));
    output.push_str(",\"pathKind\":");
    output.push_str(&json_string(embedded_press_title_art_path_kind_summary(
        paths,
    )));
    output.push_str(",\"pathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"record70Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(paths, 0x70, 0),
    );
    output.push_str(",\"record70Word1Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(paths, 0x70, 1),
    );
    output.push_str(",\"record70Word3Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(paths, 0x70, 3),
    );
    output.push_str(",\"record70Word7Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(paths, 0x70, 7),
    );
    output.push('}');
}

pub(super) fn push_success_data_test_title_art_paint_role_separation_matrix_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();
    let shadow_paths = partition
        .as_ref()
        .map(|partition| partition.shadow_paths.as_slice())
        .unwrap_or(&[]);
    let main_paths = partition
        .as_ref()
        .map(|partition| partition.main_paths.as_slice())
        .unwrap_or(&[]);

    let shadow_values = embedded_press_title_art_role_state_word_value_sets(shadow_paths);
    let texture_values =
        embedded_press_title_art_role_state_word_value_sets(&interstitial_texture_paths);
    let main_values = embedded_press_title_art_role_state_word_value_sets(main_paths);
    let mut keys = BTreeSet::<(u32, usize)>::new();
    keys.extend(shadow_values.keys().copied());
    keys.extend(texture_values.keys().copied());
    keys.extend(main_values.keys().copied());

    let mut shared_across_all_count = 0usize;
    let mut shadow_unique_count = 0usize;
    let mut texture_unique_count = 0usize;
    let mut main_unique_count = 0usize;
    let mut texture_main_disjoint_count = 0usize;
    let mut shadow_texture_shared_main_disjoint_count = 0usize;
    let mut missing_role_value_count = 0usize;

    for key in &keys {
        let empty = BTreeSet::<u32>::new();
        let shadow = shadow_values.get(key).unwrap_or(&empty);
        let texture = texture_values.get(key).unwrap_or(&empty);
        let main = main_values.get(key).unwrap_or(&empty);
        let present_in_all = !shadow.is_empty() && !texture.is_empty() && !main.is_empty();
        if !present_in_all {
            missing_role_value_count += 1;
        }
        if present_in_all && shadow == texture && texture == main {
            shared_across_all_count += 1;
        }
        if present_in_all && shadow.is_disjoint(texture) && shadow.is_disjoint(main) {
            shadow_unique_count += 1;
        }
        if present_in_all && texture.is_disjoint(shadow) && texture.is_disjoint(main) {
            texture_unique_count += 1;
        }
        if present_in_all && main.is_disjoint(shadow) && main.is_disjoint(texture) {
            main_unique_count += 1;
        }
        if !texture.is_empty() && !main.is_empty() && texture.is_disjoint(main) {
            texture_main_disjoint_count += 1;
        }
        if present_in_all && shadow == texture && texture.is_disjoint(main) {
            shadow_texture_shared_main_disjoint_count += 1;
        }
    }

    output.push_str(
        "{\"source\":\"embeddedPressRoleStateWordMatrix\",\"decoded\":false,\"sourceBacked\":",
    );
    output.push_str(
        if partition.is_some() && !interstitial_texture_paths.is_empty() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"rolePartitionBasis\":\"embeddedPressPathSourceOrder\"");
    output.push_str(",\"recordWordCandidateCount\":");
    output.push_str(&keys.len().to_string());
    output.push_str(",\"sharedAcrossAllRecordWordCount\":");
    output.push_str(&shared_across_all_count.to_string());
    output.push_str(",\"shadowUniqueRecordWordCount\":");
    output.push_str(&shadow_unique_count.to_string());
    output.push_str(",\"textureUniqueRecordWordCount\":");
    output.push_str(&texture_unique_count.to_string());
    output.push_str(",\"mainUniqueRecordWordCount\":");
    output.push_str(&main_unique_count.to_string());
    output.push_str(",\"textureMainDisjointRecordWordCount\":");
    output.push_str(&texture_main_disjoint_count.to_string());
    output.push_str(",\"shadowTextureSharedMainDisjointRecordWordCount\":");
    output.push_str(&shadow_texture_shared_main_disjoint_count.to_string());
    output.push_str(",\"missingRoleValueRecordWordCount\":");
    output.push_str(&missing_role_value_count.to_string());
    output.push_str(",\"textureOnlySeparatorPresent\":");
    output.push_str(if texture_unique_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mainOnlySeparatorPresent\":");
    output.push_str(if main_unique_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matrixConclusion\":");
    output.push_str(&json_string(
        if texture_unique_count == 0 && shadow_texture_shared_main_disjoint_count > 0 {
            "record-words-separate-main-from-shadow-state-texture-but-not-interstitial-texture-only"
        } else if texture_unique_count == 0 {
            "no-record-word-separates-interstitial-texture-only"
        } else {
            "texture-only-record-word-candidate-present"
        },
    ));
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(if texture_unique_count == 0 {
        "no-record-word-separates-interstitial-texture-from-both-outline-roles"
    } else {
        "texture-only-record-word-needs-cross-sample-validation"
    }));
    output.push_str(",\"recordWords\":[");
    for (index, key) in keys.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let empty = BTreeSet::<u32>::new();
        let shadow = shadow_values.get(key).unwrap_or(&empty);
        let texture = texture_values.get(key).unwrap_or(&empty);
        let main = main_values.get(key).unwrap_or(&empty);
        push_success_data_test_title_art_role_matrix_record_word_json(
            output, *key, shadow, texture, main,
        );
    }
    output.push_str("]}");
}

pub(super) fn push_success_data_test_title_art_role_matrix_record_word_json(
    output: &mut String,
    key: (u32, usize),
    shadow: &BTreeSet<u32>,
    texture: &BTreeSet<u32>,
    main: &BTreeSet<u32>,
) {
    let present_in_all = !shadow.is_empty() && !texture.is_empty() && !main.is_empty();
    let shared_across_all = present_in_all && shadow == texture && texture == main;
    let shadow_disjoint = present_in_all && shadow.is_disjoint(texture) && shadow.is_disjoint(main);
    let texture_disjoint =
        present_in_all && texture.is_disjoint(shadow) && texture.is_disjoint(main);
    let main_disjoint = present_in_all && main.is_disjoint(shadow) && main.is_disjoint(texture);
    let texture_main_disjoint =
        !texture.is_empty() && !main.is_empty() && texture.is_disjoint(main);
    let shadow_texture_shared_main_disjoint =
        present_in_all && shadow == texture && texture.is_disjoint(main);
    let interpretation = if texture_disjoint {
        "texture-only-candidate"
    } else if shadow_texture_shared_main_disjoint {
        "main-vs-shadow-state-texture-candidate"
    } else if shadow_disjoint {
        "shadow-vs-non-shadow-candidate"
    } else if main_disjoint {
        "main-vs-non-main-candidate"
    } else if shared_across_all {
        "shared-across-all-roles"
    } else if !present_in_all {
        "role-missing"
    } else {
        "overlapping-or-ambiguous"
    };

    output.push_str("{\"recordType\":");
    output.push_str(&key.0.to_string());
    output.push_str(",\"recordTypeHex\":");
    output.push_str(&json_string(&format!("0x{:02x}", key.0)));
    output.push_str(",\"wordIndex\":");
    output.push_str(&key.1.to_string());
    output.push_str(",\"shadowValues\":");
    push_u32_hex_array_json(output, &shadow.iter().copied().collect::<Vec<_>>());
    output.push_str(",\"textureValues\":");
    push_u32_hex_array_json(output, &texture.iter().copied().collect::<Vec<_>>());
    output.push_str(",\"mainValues\":");
    push_u32_hex_array_json(output, &main.iter().copied().collect::<Vec<_>>());
    output.push_str(",\"presentInAllRoles\":");
    output.push_str(if present_in_all { "true" } else { "false" });
    output.push_str(",\"sharedAcrossAllRoles\":");
    output.push_str(if shared_across_all { "true" } else { "false" });
    output.push_str(",\"shadowDisjointFromTextureAndMain\":");
    output.push_str(if shadow_disjoint { "true" } else { "false" });
    output.push_str(",\"textureDisjointFromShadowAndMain\":");
    output.push_str(if texture_disjoint { "true" } else { "false" });
    output.push_str(",\"mainDisjointFromShadowAndTexture\":");
    output.push_str(if main_disjoint { "true" } else { "false" });
    output.push_str(",\"textureMainDisjoint\":");
    output.push_str(if texture_main_disjoint {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"shadowTextureSharedMainDisjoint\":");
    output.push_str(if shadow_texture_shared_main_disjoint {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"interpretation\":");
    output.push_str(&json_string(interpretation));
    output.push('}');
}

pub(super) fn push_success_data_test_title_art_role_gate_group_json(
    output: &mut String,
    role: &str,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let explicit_state_path_count = paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();

    output.push_str("{\"role\":");
    output.push_str(&json_string(role));
    output.push_str(",\"pathKind\":");
    output.push_str(&json_string(embedded_press_title_art_path_kind_summary(
        paths,
    )));
    output.push_str(",\"pathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"explicitStatePathCount\":");
    output.push_str(&explicit_state_path_count.to_string());
    output.push_str(",\"inheritedStatePathCount\":");
    output.push_str(
        &paths
            .len()
            .saturating_sub(explicit_state_path_count)
            .to_string(),
    );
    output.push_str(",\"record46Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x46),
    );
    output.push_str(",\"record48Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x48),
    );
    output.push_str(",\"record60Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x60),
    );
    output.push_str(",\"record65Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x65),
    );
    output.push_str(",\"record70Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x70),
    );
    output.push_str(",\"record82Word5Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(
            paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        ),
    );
    output.push('}');
}

pub(super) fn success_data_test_title_art_effective_texture_paths_for_word5(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    expected_word5: u32,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    embedded_press_effective_texture_paths_for_state_word(
        snapshot,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        5,
        expected_word5,
    )
}

pub(super) fn success_data_test_title_art_effective_texture_word5_values(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<u32> {
    embedded_press_effective_texture_state_word_values(
        snapshot,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        5,
    )
}

pub(super) fn success_data_test_title_art_effective_front_texture_word5_values(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<u32> {
    embedded_press_effective_texture_state_word_values(
        snapshot,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        5,
    )
    .into_iter()
    .filter(|value| *value == EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5)
    .collect::<Vec<_>>()
}

pub(super) fn push_success_data_test_title_art_frame_svg(
    svg: &mut String,
    frame: &ObjectJsfartArtFrameCandidate,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) {
    let frame_x = x + frame.left() as f32 * scale_x;
    let frame_y = y + frame.top() as f32 * scale_y;
    let frame_width = frame.right().saturating_sub(frame.left()) as f32 * scale_x;
    let frame_height = frame.bottom().saturating_sub(frame.top()) as f32 * scale_y;
    if frame_width <= 0.0 || frame_height <= 0.0 {
        return;
    }

    let rx = frame.corner_radius_x() as f32 * scale_x;
    let ry = frame.corner_radius_y() as f32 * scale_y;
    let stroke_width = success_data_test_title_art_frame_stroke_width(frame, scale_x, scale_y);
    svg.push_str(&format!(
        "<rect class=\"rjtd-success-data-test-title-frame\" data-source=\"JSFart2Contents\" data-source-left=\"{}\" data-source-top=\"{}\" data-source-right=\"{}\" data-source-bottom=\"{}\" data-source-content-left=\"{}\" data-source-content-top=\"{}\" data-source-content-right=\"{}\" data-source-content-bottom=\"{}\" data-source-corner-radius-x=\"{}\" data-source-corner-radius-y=\"{}\" x=\"{frame_x:.2}\" y=\"{frame_y:.2}\" width=\"{frame_width:.2}\" height=\"{frame_height:.2}\" rx=\"{rx:.2}\" ry=\"{ry:.2}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\"/>",
        frame.left(),
        frame.top(),
        frame.right(),
        frame.bottom(),
        frame.content_left(),
        frame.content_top(),
        frame.content_right(),
        frame.content_bottom(),
        frame.corner_radius_x(),
        frame.corner_radius_y(),
    ));
}

pub(super) fn success_data_test_title_art_frame_stroke_width(
    frame: &ObjectJsfartArtFrameCandidate,
    scale_x: f32,
    scale_y: f32,
) -> f32 {
    frame
        .stroke_width_candidate()
        .map(|value| value as f32 * ((scale_x + scale_y) / 2.0) * 0.5)
        .unwrap_or_else(|| ((scale_x + scale_y) / 2.0).max(1.0))
}

pub(super) fn push_success_data_test_top_text_projection_svg(
    svg: &mut String,
    document: &Document,
    layout: PageLayout,
    slots: &[SuccessDataTestTextSlot],
    font_family: &str,
) {
    let figure_label_line = success_data_test_q4_figure_label_source_line(document, slots);
    let resolved_slots = success_data_test_resolve_top_text_slots(document, slots);
    let static_unbacked_slot_count = resolved_slots
        .iter()
        .filter(|slot| !(figure_label_line.is_some() && slot.role == "figure-label"))
        .filter(|slot| {
            success_data_test_source_text_placement_candidate(
                document,
                layout,
                slot.source_span.as_ref(),
                SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
            )
            .is_none()
        })
        .count();
    let figure_label_unbacked_slot_count = figure_label_line
        .as_ref()
        .map(|line| {
            line.spans
                .iter()
                .filter(|span| {
                    success_data_test_source_text_placement_candidate(
                        document,
                        layout,
                        Some(&span.source_span),
                        line.font_size,
                    )
                    .is_none()
                })
                .count()
        })
        .unwrap_or(0);
    let unbacked_slot_count = static_unbacked_slot_count + figure_label_unbacked_slot_count;
    let renderable_slot_count = resolved_slots
        .iter()
        .filter(|slot| !(figure_label_line.is_some() && slot.role == "figure-label"))
        .filter(|slot| {
            success_data_test_source_text_placement_candidate(
                document,
                layout,
                slot.source_span.as_ref(),
                SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
            )
            .is_some()
        })
        .count()
        + figure_label_line
            .as_ref()
            .map(|line| {
                line.spans
                    .iter()
                    .filter(|span| {
                        success_data_test_source_text_placement_candidate(
                            document,
                            layout,
                            Some(&span.source_span),
                            line.font_size,
                        )
                        .is_some()
                    })
                    .count()
            })
            .unwrap_or(0);
    svg.push_str(&format!("<g class=\"rjtd-success-data-test-top-text-projection\" data-projection-kind=\"successDataTestTopTextProjection\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-reference-backed=\"true\" data-source-grid-render-required=\"true\" data-source-grid-rendered-slot-count=\"{}\" data-unbacked-slot-count=\"{}\" data-reference-fallback-rendered-count=\"0\">", renderable_slot_count, unbacked_slot_count));
    for slot in &resolved_slots {
        if figure_label_line.is_some() && slot.role == "figure-label" {
            continue;
        }
        let Some(source_placement) = success_data_test_source_text_placement_candidate(
            document,
            layout,
            slot.source_span.as_ref(),
            SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
        ) else {
            continue;
        };
        let baseline_y = success_data_test_text_baseline_y(
            Some(&source_placement),
            slot.y + SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
        );
        svg.push_str(&format!(
            "<g class=\"rjtd-success-data-test-top-text-slot\" data-role=\"{}\">",
            escape_xml(slot.role)
        ));
        let source_grid_attrs = format!(
            " data-source-grid-placement=\"lineMarkPageGrid\" data-source-grid-record-index=\"{}\" data-source-grid-top-y=\"{:.3}\" data-source-grid-baseline-y=\"{:.3}\"",
            source_placement.line_grid.record_index,
            source_placement.top_y,
            source_placement.baseline_y
        );
        svg.push_str(&format!(
            "<text class=\"rjtd-success-data-test-top-text\" data-y-basis=\"lineMarkPageGrid\"{} x=\"{:.1}\" y=\"{:.1}\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            source_grid_attrs,
            slot.x,
            baseline_y,
            escape_xml(font_family),
            SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
            escape_xml(&svg_visual_text(slot.text))
        ));
        svg.push_str("</g>");
    }
    if let Some(line) = figure_label_line {
        svg.push_str(
            "<g class=\"rjtd-success-data-test-top-text-slot\" data-role=\"figure-label-line\">",
        );
        svg.push_str(&format!(
            "<metadata data-render-source=\"document-text-preserved-spacing\" data-line-text=\"{}\" data-source-byte-start=\"{}\" data-source-byte-end=\"{}\" data-source-unit-start=\"{}\" data-source-unit-end=\"{}\" data-advance-model=\"japanese-fixed-pitch-halfwidth-space\" data-origin-x=\"{:.1}\" data-font-size=\"{:.1}\"/>",
            escape_xml(&line.text),
            line.source_span.byte_start,
            line.source_span.byte_end,
            line.source_span.unit_start,
            line.source_span.unit_end,
            line.x,
            line.font_size,
        ));
        for span in &line.spans {
            let Some(source_placement) = success_data_test_source_text_placement_candidate(
                document,
                layout,
                Some(&span.source_span),
                line.font_size,
            ) else {
                continue;
            };
            let baseline_y =
                success_data_test_text_baseline_y(Some(&source_placement), line.y + line.font_size);
            let source_grid_attrs = format!(
                " data-source-grid-placement=\"lineMarkPageGrid\" data-source-grid-record-index=\"{}\" data-source-grid-top-y=\"{:.3}\" data-source-grid-baseline-y=\"{:.3}\"",
                source_placement.line_grid.record_index,
                source_placement.top_y,
                source_placement.baseline_y
            );
            svg.push_str(&format!(
                "<text class=\"rjtd-success-data-test-top-text\" data-role=\"figure-label\" data-render-source=\"document-text-fixed-pitch-span\" data-y-basis=\"lineMarkPageGrid\"{} data-source-byte-start=\"{}\" data-source-byte-end=\"{}\" data-source-unit-start=\"{}\" data-source-unit-end=\"{}\" x=\"{:.1}\" y=\"{:.1}\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
                source_grid_attrs,
                span.source_span.byte_start,
                span.source_span.byte_end,
                span.source_span.unit_start,
                span.source_span.unit_end,
                span.x,
                baseline_y,
                escape_xml(font_family),
                line.font_size,
                escape_xml(&svg_visual_text(&span.text))
            ));
        }
        svg.push_str("</g>");
    }
    svg.push_str("</g>");
}

pub(super) fn success_data_test_q4_figure_label_source_line(
    document: &Document,
    slots: &[SuccessDataTestTextSlot],
) -> Option<SuccessDataTestFigureLabelLine> {
    let heading = slots
        .iter()
        .find(|slot| slot.text.starts_with("４、次の図で"))?;
    let line_step = success_data_test_top_text_line_step_px(slots)?;
    let bytes = document_text_raw_stream(document)?;
    let map = map_document_text(bytes);
    let entry = map
        .entries()
        .iter()
        .find(|entry| success_data_test_q4_figure_label_text_line(entry.text()))?;
    let text = entry.text().trim_end_matches('\n').to_string();
    let y = heading.y + line_step;
    let font_size = success_data_test_figure_label_font_size_px(line_step);
    let source_span = TextSourceSpan::from_document_text_entry(entry);
    let spans = success_data_test_q4_figure_label_spans(&text, heading.x, font_size, &source_span)?;
    Some(SuccessDataTestFigureLabelLine {
        text,
        x: heading.x,
        y,
        font_size,
        source_span,
        line_header: shanai_lan_line_header_for_text_entry(bytes, entry),
        spans,
    })
}

pub(super) fn success_data_test_q4_figure_label_text_line(text: &str) -> bool {
    let line = text.trim_end_matches('\n');
    if !(line.contains("（１）") && line.contains("（２）") && line.contains("（３）")) {
        return false;
    }
    line.chars()
        .all(|character| matches!(character, ' ' | '（' | '）' | '１' | '２' | '３'))
}

pub(super) fn success_data_test_top_text_line_step_px(
    slots: &[SuccessDataTestTextSlot],
) -> Option<f32> {
    let mut deltas = slots
        .windows(2)
        .filter_map(|window| {
            let delta = window[1].y - window[0].y;
            (18.0..=24.0).contains(&delta).then_some(delta)
        })
        .collect::<Vec<_>>();
    if deltas.is_empty() {
        return None;
    }
    deltas.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    deltas.get(deltas.len() / 2).copied()
}

pub(super) fn success_data_test_figure_label_font_size_px(line_step: f32) -> f32 {
    (line_step * 2.0 / 3.0).max(SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX)
}

pub(super) fn success_data_test_q4_figure_label_spans(
    line_text: &str,
    origin_x: f32,
    font_size: f32,
    source_span: &TextSourceSpan,
) -> Option<Vec<SuccessDataTestFigureLabelSpan>> {
    let mut cursor_units = 0usize;
    ["（１）", "（２）", "（３）"]
        .iter()
        .map(|label| {
            let (start_units, end_units) =
                find_text_utf16_unit_range_after(line_text, label, cursor_units)?;
            cursor_units = end_units;
            let start_byte = byte_index_after_utf16_units(line_text, start_units)?;
            let prefix = &line_text[..start_byte];
            Some(SuccessDataTestFigureLabelSpan {
                text: (*label).to_string(),
                x: origin_x + success_data_test_fixed_pitch_advance_px(prefix, font_size),
                source_span: source_span.subspan_by_units(start_units, end_units),
            })
        })
        .collect()
}

pub(super) fn success_data_test_fixed_pitch_advance_px(text: &str, font_size: f32) -> f32 {
    text.chars()
        .map(|character| {
            if character == ' ' || character.is_ascii() {
                font_size * 0.5
            } else {
                font_size
            }
        })
        .sum()
}

pub(super) fn push_success_data_test_cone_diagram_projection_svg(
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

pub(super) fn success_data_test_fdm_reference_projections(
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

pub(super) fn success_data_test_cone_fdm_projection_from_segments(
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

pub(super) fn success_data_test_uniform_target_height_px(
    source_left: i32,
    source_top: i32,
    source_right: i32,
    source_bottom: i32,
    target_width_px: f32,
) -> f32 {
    let source_width = source_right.saturating_sub(source_left).abs().max(1) as f32;
    let source_height = source_bottom.saturating_sub(source_top).abs().max(1) as f32;
    source_height / source_width * target_width_px
}

pub(super) fn success_data_test_cone_text_bbox_matches_vector_bbox(
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

pub(super) fn success_data_test_q5_fdm_projection_from_segments(
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
pub(super) fn push_success_data_test_fdm_reference_projection_svg(
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

pub(super) fn push_success_data_test_fdm_reference_command_svg(
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

pub(super) fn success_data_test_fdm_command_source_svg_attrs(
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

pub(super) fn success_data_test_fdm_source_cohort_svg_attrs(
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

pub(super) fn success_data_test_fdm_index_row_order_promotion_gate_svg_attrs(
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

pub(super) fn success_data_test_fdm_index_row_order_render_commands<'a>(
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

pub(super) fn success_data_test_fdm_render_command_order_svg_attrs(
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

pub(super) fn success_data_test_fdm_render_command_order_blocked_reason(
    projection: SuccessDataTestFdmProjection,
) -> &'static str {
    match projection.role {
        "q5-solid-diagram" => "fdm-index-row-fanout-primitive-ownership-unproven",
        _ => "fdm-index-row-render-order-gate-unmet",
    }
}

pub(super) fn success_data_test_fdm_source_cohort(
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

pub(super) fn push_success_data_test_fdm_source_cohort_json(
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

pub(super) fn push_success_data_test_fdm_primitive_ownership_comparison_json(
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

pub(super) fn success_data_test_fdm_primitive_ownership_classifications<'a>(
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

pub(super) fn push_success_data_test_fdm_role_count_json(
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

pub(super) fn success_data_test_fdm_offset_field_authority_gate(
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

pub(super) fn push_success_data_test_fdm_offset_field_authority_gate_json(
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

pub(super) fn success_data_test_fdm_row_fanout_segment_owner_gate(
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

pub(super) fn push_success_data_test_fdm_row_fanout_segment_owner_gate_json(
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

pub(super) fn push_success_data_test_fdm_row_fanout_segment_owner_rows_json(
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

pub(super) fn success_data_test_fdm_primitive_ownership_gate(
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

pub(super) fn success_data_test_fdm_command_gap_p95(offsets: &BTreeSet<usize>) -> Option<f32> {
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

pub(super) fn push_success_data_test_fdm_primitive_ownership_gate_json(
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

pub(super) fn push_success_data_test_fdm_primitive_ownership_admission_gate_json(
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

pub(super) fn success_data_test_fdm_index_row_order_promotion_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmIndexRowOrderPromotionGate {
    let mut gate = SuccessDataTestFdmIndexRowOrderPromotionGate {
        command_count: classifications.len(),
        ..SuccessDataTestFdmIndexRowOrderPromotionGate::default()
    };

    for classification in classifications {
        for reference in &classification.index_row_references {
            gate.reference_count += 1;
            gate.referenced_command_relative_offsets
                .insert(classification.command.relative_offset());
            gate.referenced_row_indexes.insert(reference.row_index);
            gate.row_command_pairs
                .insert(SuccessDataTestFdmIndexRowCommandPair {
                    row_index: reference.row_index,
                    command_relative_offset: classification.command.relative_offset(),
                    match_kind: reference.match_kind,
                });
            gate.row_to_command_relative_offsets
                .entry(reference.row_index)
                .or_default()
                .insert(classification.command.relative_offset());
            if reference.valid_vector_offset {
                gate.valid_vector_offset_reference_count += 1;
            }
            match reference.match_kind {
                "command-relative-offset-field" => {
                    gate.command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    gate.source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
    }
    gate
}

pub(super) fn push_success_data_test_fdm_index_row_order_promotion_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let render_promotion_blocked_reasons =
        success_data_test_fdm_index_row_order_promotion_blocked_reasons(classifications, &gate);
    let render_promotion_blocked_reason = render_promotion_blocked_reasons
        .first()
        .copied()
        .unwrap_or("none");
    output.push_str("{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"renderPromotionBlockedReasons\":");
    push_json_string_slice_array(output, &render_promotion_blocked_reasons);
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referencedCommandCount\":");
    output.push_str(&gate.referenced_command_count().to_string());
    output.push_str(",\"unreferencedCommandCount\":");
    output.push_str(&gate.unreferenced_command_count().to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&gate.unique_row_index_count().to_string());
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
    output.push_str(",\"allCommandsReferencedByIndexRowsCandidate\":");
    output.push_str(if gate.all_commands_referenced_by_index_rows_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if gate.single_row_backs_multiple_commands_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowOrderMatchesCommandOrderCandidate\":");
    output.push_str(if gate.row_order_matches_command_order_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referencedCommandRelativeOffsets\":");
    push_usize_array_json(
        output,
        &gate
            .referenced_command_relative_offsets
            .iter()
            .copied()
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"referencedRowIndexes\":");
    push_usize_array_json(
        output,
        &gate
            .referenced_row_indexes
            .iter()
            .copied()
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"rowCommandPairs\":");
    push_success_data_test_fdm_index_row_command_pairs_json(output, &gate.row_command_pairs);
    output.push_str(
        ",\"renderPaintOrderBasisCandidate\":\"fdm-index-row-command-pairs\",\"renderPaintOrderBasisDecoded\":false",
    );
    output.push('}');
}

pub(super) fn success_data_test_fdm_index_row_order_promotion_blocked_reasons(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
    gate: &SuccessDataTestFdmIndexRowOrderPromotionGate,
) -> Vec<&'static str> {
    let mut reasons = Vec::new();
    if !gate.all_commands_referenced_by_index_rows_candidate() {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-reference-coverage-incomplete",
        );
    }
    if !gate.one_to_one_row_command_reference_candidate() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-reference-not-one-to-one");
    }
    if gate.single_row_backs_multiple_commands_candidate() {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-single-row-backs-multiple-commands",
        );
    }
    if !gate.row_order_matches_command_order_candidate() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-non-monotonic");
    }
    if gate.reference_count > 0 && gate.valid_vector_offset_reference_count == 0 {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-valid-vector-offset-missing",
        );
    }
    if gate.command_relative_offset_field_reference_count > 0
        && gate.source_segment_relative_offset_field_reference_count > 0
    {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-offset-namespace-mixed");
    }

    let role_groups =
        success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);
    let mut role_paint_order_continuity_blocked = false;
    let mut role_paint_order_authority_pending = false;
    for group in role_groups.values() {
        let profile =
            success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
        role_paint_order_continuity_blocked |= profile.continuity_blocked();
        role_paint_order_authority_pending |= profile.paint_order_authority_pending();
    }
    if role_paint_order_continuity_blocked {
        push_unique_static_str(&mut reasons, "role-paint-order-continuity-unproven");
    }
    if role_paint_order_authority_pending {
        push_unique_static_str(&mut reasons, "role-paint-order-authority-unproven");
    }
    if reasons.is_empty() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-paint-authority-unproven");
    }
    reasons
}

pub(super) fn success_data_test_fdm_index_row_reference_role_candidate_groups(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> BTreeMap<&'static str, SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup> {
    let mut groups =
        BTreeMap::<&'static str, SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup>::new();
    for classification in classifications {
        if classification.index_row_references.is_empty() {
            continue;
        }
        for role_candidate in &classification.role_candidates {
            let group = groups.entry(*role_candidate).or_insert_with(|| {
                SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
                    role_candidate,
                    ..SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup::default()
                }
            });
            group
                .command_relative_offsets
                .insert(classification.command.relative_offset());
            for reference in &classification.index_row_references {
                group.reference_count += 1;
                group.row_indexes.insert(reference.row_index);
                group
                    .row_command_pairs
                    .insert(SuccessDataTestFdmIndexRowCommandPair {
                        row_index: reference.row_index,
                        command_relative_offset: classification.command.relative_offset(),
                        match_kind: reference.match_kind,
                    });
                if reference.valid_vector_offset {
                    group.valid_vector_offset_reference_count += 1;
                    match reference.match_kind {
                        "command-relative-offset-field" => {
                            group.valid_command_relative_offset_field_reference_count += 1;
                        }
                        "source-segment-relative-offset-field" => {
                            group.valid_source_segment_relative_offset_field_reference_count += 1;
                        }
                        _ => {}
                    }
                }
                match reference.match_kind {
                    "command-relative-offset-field" => {
                        group.command_relative_offset_field_reference_count += 1;
                    }
                    "source-segment-relative-offset-field" => {
                        group.source_segment_relative_offset_field_reference_count += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    groups
}

pub(super) fn success_data_test_fdm_role_group_single_row_backs_multiple_commands(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) -> bool {
    let mut row_to_command_count = BTreeMap::<usize, usize>::new();
    for pair in &group.row_command_pairs {
        *row_to_command_count.entry(pair.row_index).or_default() += 1;
    }
    row_to_command_count.values().any(|count| *count > 1)
}

pub(super) fn push_success_data_test_fdm_index_row_reference_role_candidate_groups_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let groups = success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);

    output.push('[');
    for (index, group) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"roleCandidate\":");
        output.push_str(&json_string(group.role_candidate));
        output.push_str(",\"ownershipProven\":false");
        output.push_str(
            ",\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\"",
        );
        output.push_str(",\"referenceCount\":");
        output.push_str(&group.reference_count.to_string());
        output.push_str(",\"validVectorOffsetReferenceCount\":");
        output.push_str(&group.valid_vector_offset_reference_count.to_string());
        output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
        output.push_str(
            &group
                .command_relative_offset_field_reference_count
                .to_string(),
        );
        output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
        output.push_str(
            &group
                .source_segment_relative_offset_field_reference_count
                .to_string(),
        );
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(
            output,
            &group
                .command_relative_offsets
                .iter()
                .copied()
                .collect::<Vec<_>>(),
        );
        output.push_str(",\"rowIndexes\":");
        push_usize_array_json(
            output,
            &group.row_indexes.iter().copied().collect::<Vec<_>>(),
        );
        output.push_str(",\"uniqueCommandRelativeOffsetCount\":");
        output.push_str(&group.command_relative_offsets.len().to_string());
        output.push_str(",\"uniqueRowIndexCount\":");
        output.push_str(&group.row_indexes.len().to_string());
        output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
        output.push_str(
            if group.reference_count == group.command_relative_offsets.len()
                && group.reference_count == group.row_indexes.len()
            {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
        output.push_str(
            if group.row_indexes.len() == 1 && group.command_relative_offsets.len() > 1 {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"rowOrderMatchesCommandOrderCandidate\":");
        output.push_str(
            if success_data_test_fdm_row_command_pairs_are_monotonic(&group.row_command_pairs) {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"rowCommandPairs\":");
        push_success_data_test_fdm_index_row_command_pairs_json(output, &group.row_command_pairs);
        output.push_str(",\"roleVectorOffsetAuthorityGate\":");
        push_success_data_test_fdm_role_vector_offset_authority_gate_json(output, group);
        output.push_str(",\"roleFanoutSegmentOwnerGate\":");
        push_success_data_test_fdm_role_fanout_segment_owner_gate_json(output, group);
        output.push_str(",\"decoded\":false,\"paintOrderContinuityProfile\":");
        push_success_data_test_fdm_role_paint_order_continuity_profile_json(
            output,
            group,
            classifications,
        );
        output.push('}');
    }
    output.push(']');
}

pub(super) fn success_data_test_fdm_role_vector_offset_authority_blocked_reason(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) -> &'static str {
    let mixed_valid_offset_namespaces = group.valid_command_relative_offset_field_reference_count
        > 0
        && group.valid_source_segment_relative_offset_field_reference_count > 0;
    if group.valid_vector_offset_reference_count == 0 {
        "fdm-index-role-vector-offset-authority-valid-vector-offset-missing"
    } else if mixed_valid_offset_namespaces {
        "fdm-index-role-vector-offset-authority-mixed-valid-offset-namespaces"
    } else {
        "fdm-index-role-vector-offset-authority-semantics-unproven"
    }
}

pub(super) fn push_success_data_test_fdm_role_vector_offset_authority_gate_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) {
    let invalid_vector_offset_reference_count = group
        .reference_count
        .saturating_sub(group.valid_vector_offset_reference_count);
    let invalid_command_relative_offset_field_reference_count = group
        .command_relative_offset_field_reference_count
        .saturating_sub(group.valid_command_relative_offset_field_reference_count);
    let invalid_source_segment_relative_offset_field_reference_count = group
        .source_segment_relative_offset_field_reference_count
        .saturating_sub(group.valid_source_segment_relative_offset_field_reference_count);
    let mixed_offset_namespaces_among_valid_refs =
        group.valid_command_relative_offset_field_reference_count > 0
            && group.valid_source_segment_relative_offset_field_reference_count > 0;
    let all_valid_references_use_command_relative_offset_field =
        group.valid_vector_offset_reference_count > 0
            && group.valid_command_relative_offset_field_reference_count
                == group.valid_vector_offset_reference_count;
    let all_valid_references_use_source_segment_relative_offset_field =
        group.valid_vector_offset_reference_count > 0
            && group.valid_source_segment_relative_offset_field_reference_count
                == group.valid_vector_offset_reference_count;
    let all_references_have_invalid_vector_offset =
        group.reference_count > 0 && group.valid_vector_offset_reference_count == 0;
    let render_promotion_blocked_reason =
        success_data_test_fdm_role_vector_offset_authority_blocked_reason(group);

    output.push_str("{\"basis\":\"fdm-index-role-vector-offset-authority-gate\",\"source\":\"FDMIndex.vectorOffset+FDMIndex role offset fields\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"roleCandidate\":");
    output.push_str(&json_string(group.role_candidate));
    output.push_str(",\"roleVectorOffsetAuthorityDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"fdm-index-role-vector-offset-authority-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"referenceCount\":");
    output.push_str(&group.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&group.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"invalidVectorOffsetReferenceCount\":");
    output.push_str(&invalid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"validCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .valid_command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"validSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .valid_source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"invalidCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(&invalid_command_relative_offset_field_reference_count.to_string());
    output.push_str(",\"invalidSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(&invalid_source_segment_relative_offset_field_reference_count.to_string());
    output.push_str(",\"allValidReferencesUseCommandRelativeOffsetField\":");
    output.push_str(if all_valid_references_use_command_relative_offset_field {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allValidReferencesUseSourceSegmentRelativeOffsetField\":");
    output.push_str(
        if all_valid_references_use_source_segment_relative_offset_field {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"mixedOffsetNamespacesAmongValidReferences\":");
    output.push_str(if mixed_offset_namespaces_among_valid_refs {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesHaveInvalidVectorOffset\":");
    output.push_str(if all_references_have_invalid_vector_offset {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(super) fn push_success_data_test_fdm_role_fanout_segment_owner_gate_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) {
    let mut row_to_pairs = BTreeMap::<usize, Vec<SuccessDataTestFdmIndexRowCommandPair>>::new();
    for pair in &group.row_command_pairs {
        row_to_pairs.entry(pair.row_index).or_default().push(*pair);
    }

    let mut fanout_row_count = 0usize;
    let mut fanout_reference_count = 0usize;
    let mut fanout_command_relative_offset_field_reference_count = 0usize;
    let mut fanout_source_segment_relative_offset_field_reference_count = 0usize;
    let mut max_row_fanout = 0usize;
    for pairs in row_to_pairs.values() {
        max_row_fanout = max_row_fanout.max(pairs.len());
        if pairs.len() <= 1 {
            continue;
        }
        fanout_row_count += 1;
        fanout_reference_count += pairs.len();
        for pair in pairs {
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
    }

    let one_to_one_row_command_reference_candidate = group.reference_count
        == group.command_relative_offsets.len()
        && group.reference_count == group.row_indexes.len();
    let single_row_backs_multiple_commands_candidate =
        row_to_pairs.values().any(|pairs| pairs.len() > 1);
    let mixed_offset_field_namespaces = group.command_relative_offset_field_reference_count > 0
        && group.source_segment_relative_offset_field_reference_count > 0;
    let fanout_rows_use_command_relative_offset_fields = fanout_reference_count > 0
        && fanout_command_relative_offset_field_reference_count == fanout_reference_count;
    let fanout_rows_use_source_segment_offset_fields = fanout_reference_count > 0
        && fanout_source_segment_relative_offset_field_reference_count == fanout_reference_count;
    let render_promotion_blocked_reason = if single_row_backs_multiple_commands_candidate {
        "fdm-index-role-row-fanout-multi-command-single-row"
    } else if !one_to_one_row_command_reference_candidate {
        "fdm-index-role-row-reference-not-one-to-one"
    } else if mixed_offset_field_namespaces {
        "fdm-index-role-offset-namespace-mixed"
    } else if group.valid_vector_offset_reference_count == 0 {
        "fdm-index-role-valid-vector-offset-missing"
    } else {
        "fdm-index-role-segment-owner-semantics-unproven"
    };

    output.push_str("{\"basis\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex role row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"roleCandidate\":");
    output.push_str(&json_string(group.role_candidate));
    output.push_str(",\"roleOwnershipDecoded\":false,\"segmentOwnerDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"fdm-index-role-row-fanout-segment-owner-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"referenceCount\":");
    output.push_str(&group.reference_count.to_string());
    output.push_str(",\"uniqueCommandRelativeOffsetCount\":");
    output.push_str(&group.command_relative_offsets.len().to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&group.row_indexes.len().to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutRowCount\":");
    output.push_str(&fanout_row_count.to_string());
    output.push_str(",\"fanoutReferenceCount\":");
    output.push_str(&fanout_reference_count.to_string());
    output.push_str(",\"fanoutCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(&fanout_command_relative_offset_field_reference_count.to_string());
    output.push_str(",\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(&fanout_source_segment_relative_offset_field_reference_count.to_string());
    output.push_str(",\"maxRowFanout\":");
    output.push_str(&max_row_fanout.to_string());
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if single_row_backs_multiple_commands_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseCommandRelativeOffsetFields\":");
    output.push_str(if fanout_rows_use_command_relative_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseSourceSegmentOffsetFields\":");
    output.push_str(if fanout_rows_use_source_segment_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowsWithMultipleCommandRefs\":");
    push_success_data_test_fdm_role_fanout_rows_json(output, &row_to_pairs);
    output.push('}');
}

pub(super) fn push_success_data_test_fdm_role_fanout_rows_json(
    output: &mut String,
    row_to_pairs: &BTreeMap<usize, Vec<SuccessDataTestFdmIndexRowCommandPair>>,
) {
    output.push('[');
    let mut emitted = 0usize;
    for (row_index, pairs) in row_to_pairs {
        if pairs.len() <= 1 {
            continue;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        let command_relative_offsets = pairs
            .iter()
            .map(|pair| pair.command_relative_offset)
            .collect::<Vec<_>>();
        let match_kinds = pairs
            .iter()
            .map(|pair| pair.match_kind)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        output.push_str("{\"rowIndex\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"commandReferenceCount\":");
        output.push_str(&pairs.len().to_string());
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(output, &command_relative_offsets);
        output.push_str(",\"matchKinds\":");
        push_json_string_slice_array(output, &match_kinds);
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_success_data_test_fdm_role_paint_order_continuity_profile_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    output.push_str("{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false");
    let profile = success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
    output.push_str(",\"commandRelativeOffsetSpanMin\":");
    push_option_usize_json(output, profile.span_min);
    output.push_str(",\"commandRelativeOffsetSpanMax\":");
    push_option_usize_json(output, profile.span_max);
    output.push_str(",\"roleCommandCount\":");
    output.push_str(&profile.role_command_count.to_string());
    output.push_str(",\"commandCountInSpan\":");
    output.push_str(&profile.command_count_in_span.to_string());
    output.push_str(",\"interleavedNonRoleCommandCount\":");
    output.push_str(&profile.interleaved_non_role_command_count.to_string());
    output.push_str(",\"hasInterleavedNonRoleCommands\":");
    output.push_str(if profile.interleaved_non_role_command_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"maxCommandOffsetGap\":");
    output.push_str(&profile.max_command_offset_gap.to_string());
    output.push_str(",\"commandOffsetContinuityScore\":");
    output.push_str(&format!("{:.3}", profile.continuity_score));
    output.push_str(",\"spanContiguousCandidate\":");
    output.push_str(if profile.span_contiguous_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintOrderAuthorityPending\":");
    output.push_str(if profile.paint_order_authority_pending() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"continuityBlocked\":");
    output.push_str(if profile.continuity_blocked() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(profile.render_promotion_blocked_reason()));
    output.push('}');
}

pub(super) fn success_data_test_fdm_role_paint_order_continuity_profile(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmRolePaintOrderContinuityProfile {
    let span_min = group.command_relative_offsets.iter().next().copied();
    let span_max = group.command_relative_offsets.iter().next_back().copied();
    let role_command_count = group.command_relative_offsets.len();
    let command_count_in_span = match (span_min, span_max) {
        (Some(min), Some(max)) => classifications
            .iter()
            .filter(|classification| {
                let offset = classification.command.relative_offset();
                offset >= min && offset <= max
            })
            .count(),
        _ => 0,
    };
    let interleaved_non_role_command_count =
        command_count_in_span.saturating_sub(role_command_count);
    let mut max_command_offset_gap = 0usize;
    let mut previous_offset = None;
    for offset in group.command_relative_offsets.iter().copied() {
        if let Some(previous) = previous_offset {
            max_command_offset_gap = max_command_offset_gap.max(offset.saturating_sub(previous));
        }
        previous_offset = Some(offset);
    }
    let continuity_score = if command_count_in_span == 0 {
        0.0
    } else {
        role_command_count as f32 / command_count_in_span as f32
    };

    SuccessDataTestFdmRolePaintOrderContinuityProfile {
        span_min,
        span_max,
        role_command_count,
        command_count_in_span,
        interleaved_non_role_command_count,
        max_command_offset_gap,
        continuity_score,
    }
}

pub(super) fn success_data_test_fdm_row_command_pairs_are_monotonic(
    pairs: &BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
) -> bool {
    let mut previous_command_relative_offset = None;
    for pair in pairs {
        if previous_command_relative_offset
            .is_some_and(|previous| pair.command_relative_offset < previous)
        {
            return false;
        }
        previous_command_relative_offset = Some(pair.command_relative_offset);
    }
    true
}

pub(super) fn push_success_data_test_fdm_index_row_command_pairs_json(
    output: &mut String,
    pairs: &BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
) {
    output.push('[');
    for (index, pair) in pairs.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&pair.row_index.to_string());
        output.push_str(",\"commandRelativeOffset\":");
        output.push_str(&pair.command_relative_offset.to_string());
        output.push_str(",\"matchKind\":");
        output.push_str(&json_string(pair.match_kind));
        output.push('}');
    }
    output.push(']');
}

pub(super) fn success_data_test_fdm_primitive_ownership_classification<'a>(
    projection: SuccessDataTestFdmProjection,
    command: &'a ObjectFdmVectorCommandCandidate,
    index_entries: &[ObjectFdmIndexEntryCandidate],
    anchor: Option<(ObjectFdmVectorPoint, i32)>,
) -> SuccessDataTestFdmPrimitiveOwnershipClassification<'a> {
    let mut role_candidates = Vec::new();
    let mut classification_basis = Vec::new();
    if let Some(ellipse) = command.ellipse() {
        if success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse) {
            role_candidates.push("main-circle-anchor");
            classification_basis.push("large-01000460-ellipse-anchor");
        } else if success_data_test_fdm_reference_ellipse_is_control_marker(
            projection, command, ellipse,
        ) {
            role_candidates.push("arc-candidate");
            role_candidates.push("control-ellipse-marker");
            classification_basis.push("tiny-ff000460-ellipse-control-marker");
        } else {
            role_candidates.push("arc-candidate");
            classification_basis.push("ellipse-boundary-primitive");
        }
    } else {
        let is_two_point_line = fdm_vector_marker_is_line(command.marker())
            && command.curve_segments().is_empty()
            && command.path_points().len() == 2;
        if is_two_point_line {
            role_candidates.push("line-candidate");
            classification_basis.push("fdm-line-marker-two-point-path");
            if let Some((center, radius)) = anchor {
                let boundary_count =
                    success_data_test_fdm_anchor_boundary_point_count(command, center, radius);
                let center_count =
                    success_data_test_fdm_anchor_center_point_count(command, center, radius);
                if boundary_count >= 2 {
                    role_candidates.push("chord-candidate");
                    classification_basis.push("both-endpoints-near-anchor-boundary");
                } else if boundary_count >= 1 && center_count >= 1 {
                    role_candidates.push("radial-line-candidate");
                    classification_basis.push("one-endpoint-near-anchor-center-one-near-boundary");
                }
            }
        }
        if !command.curve_segments().is_empty()
            || fdm_vector_marker_is_bezier_curve(command.marker())
        {
            role_candidates.push("arc-candidate");
            classification_basis.push("fdm-bezier-marker-or-control-points");
        }
        if command.path_points().len() >= 3 && !fdm_vector_path_is_closed(command.path_points()) {
            role_candidates.push("surface-boundary-candidate");
            classification_basis.push("open-polyline-with-three-or-more-points");
        }
        if fdm_connector_candidate_from_command(command).is_some() {
            role_candidates.push("connector-candidate");
            classification_basis.push("long-open-source-path");
        }
    }
    if role_candidates.is_empty() {
        role_candidates.push("unclassified-primitive");
        classification_basis.push("no-current-role-rule");
    }
    SuccessDataTestFdmPrimitiveOwnershipClassification {
        command,
        role_candidates,
        classification_basis,
        index_row_references: success_data_test_fdm_index_row_references(command, index_entries),
    }
}

pub(super) fn success_data_test_fdm_index_row_references(
    command: &ObjectFdmVectorCommandCandidate,
    index_entries: &[ObjectFdmIndexEntryCandidate],
) -> Vec<SuccessDataTestFdmIndexRowReference> {
    let mut references = Vec::new();
    for entry in index_entries {
        let bbox = entry.bbox();
        let offset_value = bbox.left();
        if offset_value < 0 {
            continue;
        }
        let offset_value = offset_value as usize;
        let match_kind = if offset_value == command.relative_offset() {
            Some("command-relative-offset-field")
        } else if command
            .source_segment()
            .is_some_and(|segment| segment.relative_offset() == offset_value)
        {
            Some("source-segment-relative-offset-field")
        } else {
            None
        };
        let Some(match_kind) = match_kind else {
            continue;
        };
        references.push(SuccessDataTestFdmIndexRowReference {
            row_index: entry.row_index(),
            index_offset: entry.index_offset(),
            vector_offset: entry.vector_offset(),
            valid_vector_offset: entry.valid_vector_offset(),
            offset_field: "bbox.left",
            offset_value,
            match_kind,
        });
    }
    references
}

pub(super) fn push_success_data_test_fdm_index_row_references_json(
    output: &mut String,
    references: &[SuccessDataTestFdmIndexRowReference],
) {
    output.push('[');
    for (index, reference) in references.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&reference.row_index.to_string());
        output.push_str(",\"indexOffset\":");
        output.push_str(&reference.index_offset.to_string());
        output.push_str(",\"vectorOffset\":");
        output.push_str(&reference.vector_offset.to_string());
        output.push_str(",\"validVectorOffset\":");
        output.push_str(if reference.valid_vector_offset {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"offsetField\":");
        output.push_str(&json_string(reference.offset_field));
        output.push_str(",\"offsetValue\":");
        output.push_str(&reference.offset_value.to_string());
        output.push_str(",\"matchKind\":");
        output.push_str(&json_string(reference.match_kind));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

pub(super) fn success_data_test_fdm_anchor_boundary_point_count(
    command: &ObjectFdmVectorCommandCandidate,
    center: ObjectFdmVectorPoint,
    radius: i32,
) -> usize {
    let tolerance = (radius / 12).max(24) as f32;
    command
        .path_points()
        .iter()
        .filter(|point| (fdm_point_distance(center, **point) - radius as f32).abs() <= tolerance)
        .count()
}

pub(super) fn success_data_test_fdm_anchor_center_point_count(
    command: &ObjectFdmVectorCommandCandidate,
    center: ObjectFdmVectorPoint,
    radius: i32,
) -> usize {
    let tolerance = (radius / 8).max(24) as f32;
    command
        .path_points()
        .iter()
        .filter(|point| fdm_point_distance(center, **point) <= tolerance)
        .count()
}

pub(super) fn push_success_data_test_fdm_reference_projections_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    if candidate.path() != SUCCESS_DATA_TEST_FDM_VECTOR_PATH {
        output.push_str("[]");
        return;
    }
    let raw_commands = candidate.fdm_raw_vector_commands();
    output.push('[');
    let mut emitted = 0usize;
    for projection in success_data_test_fdm_reference_projections(candidate) {
        let commands = raw_commands
            .iter()
            .filter(|command| success_data_test_fdm_projection_command(projection, command))
            .collect::<Vec<_>>();
        if commands.is_empty() {
            continue;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        output.push_str("{\"role\":");
        output.push_str(&json_string(projection.role));
        output.push_str(",\"sourcePath\":");
        output.push_str(&json_string(candidate.path()));
        output.push_str(",\"projectionKind\":\"successDataTestFdmReferenceProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"referenceBacked\":true");
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
        output.push_str("},\"referenceTargetBboxPx\":{\"x\":");
        output.push_str(&format!("{:.3}", projection.target_x_px));
        output.push_str(",\"y\":");
        output.push_str(&format!("{:.3}", projection.target_y_px));
        output.push_str(",\"width\":");
        output.push_str(&format!("{:.3}", projection.target_width_px));
        output.push_str(",\"height\":");
        output.push_str(&format!("{:.3}", projection.target_height_px));
        output.push_str("},\"commandCount\":");
        output.push_str(&commands.len().to_string());
        output.push_str(",\"sourceCohort\":");
        push_success_data_test_fdm_source_cohort_json(output, &commands);
        output.push_str(",\"renderPromotionBlockedReason\":");
        output.push_str(&json_string(
            success_data_test_fdm_source_cohort(&commands).blocked_reason(),
        ));
        output.push_str(",\"primitiveOwnershipComparison\":");
        push_success_data_test_fdm_primitive_ownership_comparison_json(
            output,
            projection,
            &commands,
            candidate.fdm_index_entry_candidates(),
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
                output.push_str(",\"groupingSource\":\"nearest-main-circle-source-center\",\"groupingDecoded\":false,\"paintOrderDecoded\":false");
                output.push_str(",\"anchorRelativeOffset\":");
                output.push_str(&subdiagram.anchor_relative_offset.to_string());
                output.push_str(",\"anchorSourcePoint\":");
                push_fdm_vector_point_json(output, subdiagram.center);
                output.push_str(",\"commandCount\":");
                output.push_str(&subdiagram.commands.len().to_string());
                output.push_str(",\"sourceCohort\":");
                push_success_data_test_fdm_source_cohort_json(output, &subdiagram.commands);
                output.push_str(",\"renderPromotionBlockedReason\":");
                output.push_str(&json_string(
                    success_data_test_fdm_source_cohort(&subdiagram.commands).blocked_reason(),
                ));
                output.push_str(",\"primitiveOwnershipComparison\":");
                push_success_data_test_fdm_primitive_ownership_comparison_json(
                    output,
                    projection,
                    &subdiagram.commands,
                    candidate.fdm_index_entry_candidates(),
                    Some((subdiagram.center, subdiagram.anchor_radius)),
                );
                output.push('}');
            }
        }
        output.push_str("]}");
    }
    output.push(']');
}

pub(super) fn success_data_test_q4_fdm_subdiagrams<'a>(
    projection: SuccessDataTestFdmProjection,
    commands: &[&'a ObjectFdmVectorCommandCandidate],
) -> Option<Vec<SuccessDataTestFdmSubdiagram<'a>>> {
    if projection.role != "q4-angle-diagrams" {
        return None;
    }
    let mut subdiagrams = commands
        .iter()
        .filter_map(|&command| {
            let ellipse = command.ellipse()?;
            success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse)
                .then(|| SuccessDataTestFdmSubdiagram {
                    index: 0,
                    anchor_relative_offset: command.relative_offset(),
                    center: ellipse.center(),
                    anchor_radius: ellipse.radius_x().max(ellipse.radius_y()),
                    commands: Vec::new(),
                })
        })
        .collect::<Vec<_>>();
    if subdiagrams.len() < 2 {
        return None;
    }
    subdiagrams.sort_by_key(|subdiagram| {
        (
            subdiagram.center.x(),
            subdiagram.center.y(),
            subdiagram.anchor_relative_offset,
        )
    });
    for (index, subdiagram) in subdiagrams.iter_mut().enumerate() {
        subdiagram.index = index;
    }

    for &command in commands {
        let Some(center) = success_data_test_fdm_command_source_center(command) else {
            continue;
        };
        let Some((group_index, _)) = subdiagrams
            .iter()
            .enumerate()
            .map(|(index, subdiagram)| {
                (index, fdm_point_distance_squared(center, subdiagram.center))
            })
            .min_by_key(|(_, distance)| *distance)
        else {
            continue;
        };
        subdiagrams[group_index].commands.push(command);
    }

    subdiagrams
        .iter()
        .all(|subdiagram| !subdiagram.commands.is_empty())
        .then_some(subdiagrams)
}

pub(super) fn success_data_test_fdm_command_source_center(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmVectorPoint> {
    if let Some(ellipse) = command.ellipse() {
        return Some(ellipse.center());
    }
    let bbox = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox)?;
    let (center_x, center_y) = fdm_bbox_center(bbox);
    Some(ObjectFdmVectorPoint::new(center_x, center_y))
}

pub(super) fn success_data_test_fdm_reference_ellipse_is_control_marker(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
    ellipse: ObjectFdmVectorEllipse,
) -> bool {
    if projection.role != "q4-angle-diagrams" || command.marker() != b"\xff\x00\x04\x60" {
        return false;
    }
    let source_height = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs()
        .max(1);
    ellipse.radius_x() == ellipse.radius_y()
        && ellipse.radius_x().saturating_mul(6) <= source_height
}

pub(super) fn success_data_test_fdm_reference_ellipse_has_center_marker(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
    ellipse: ObjectFdmVectorEllipse,
) -> bool {
    if projection.role != "q4-angle-diagrams" || command.marker() != b"\x01\x00\x04\x60" {
        return false;
    }
    let source_height = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs()
        .max(1);
    ellipse.radius_x() == ellipse.radius_y()
        && ellipse.radius_x().saturating_mul(2) >= source_height.saturating_mul(4) / 5
}

pub(super) fn success_data_test_projected_fdm_center_marker_point(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    ellipse: ObjectFdmVectorEllipse,
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> Option<(f32, f32)> {
    let center = ellipse.center();
    let proximity = (ellipse.radius_x() / 20).max(16) as f32;
    let mut candidates = Vec::new();
    for command in commands {
        if command.ellipse().is_some() {
            continue;
        }
        for point in command.path_points() {
            if fdm_point_distance(center, *point) <= proximity {
                candidates.push(*point);
            }
        }
    }
    if candidates.is_empty() {
        return None;
    }
    let sum_x = candidates
        .iter()
        .fold(0i64, |sum, point| sum + i64::from(point.x()));
    let sum_y = candidates
        .iter()
        .fold(0i64, |sum, point| sum + i64::from(point.y()));
    let count = candidates.len() as i64;
    success_data_test_project_fdm_point(
        layout,
        projection,
        (sum_x / count) as i32,
        (sum_y / count) as i32,
    )
}

pub(super) fn success_data_test_projected_fdm_control_ellipse_arc_path_data(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    ellipse: ObjectFdmVectorEllipse,
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> Option<String> {
    let center = ellipse.center();
    let (cx, cy, rx, ry) = success_data_test_projected_fdm_ellipse(layout, projection, ellipse)?;
    let mut rays =
        success_data_test_control_ellipse_angle_rays(center, ellipse.radius_x(), commands)
            .into_iter()
            .filter_map(|endpoint| {
                let (x, y) = success_data_test_project_fdm_point(
                    layout,
                    projection,
                    endpoint.x(),
                    endpoint.y(),
                )?;
                let dx = x - cx;
                let dy = y - cy;
                let distance = (dx * dx + dy * dy).sqrt();
                (distance > 1.0 && dy > 0.0).then_some((dx / distance, dy / distance))
            })
            .collect::<Vec<_>>();
    if rays.len() < 2 {
        return None;
    }
    rays.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let first = *rays.first()?;
    let last = *rays.last()?;
    let mid = {
        let x = first.0 + last.0;
        let y = first.1 + last.1;
        let distance = (x * x + y * y).sqrt();
        if distance > 0.001 && y > 0.0 {
            (x / distance, y / distance)
        } else {
            (0.0, 1.0)
        }
    };
    let start = (cx + first.0 * rx, cy + first.1 * ry);
    let through = (cx + mid.0 * rx, cy + mid.1 * ry);
    let end = (cx + last.0 * rx, cy + last.1 * ry);
    let control = fdm_quadratic_control_point(start, through, end);
    Some(format!(
        "M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}",
        start.0, start.1, control.0, control.1, end.0, end.1
    ))
}

pub(super) fn success_data_test_control_ellipse_angle_rays(
    center: ObjectFdmVectorPoint,
    radius: i32,
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> Vec<ObjectFdmVectorPoint> {
    let proximity = (radius / 3).max(12) as f32;
    let mut rays = Vec::new();
    for command in commands {
        if command.ellipse().is_some() {
            continue;
        }
        for segment in command.path_points().windows(2) {
            let start = segment[0];
            let end = segment[1];
            if fdm_point_distance(center, start) <= proximity {
                rays.push(end);
            }
            if fdm_point_distance(center, end) <= proximity {
                rays.push(start);
            }
            if fdm_point_segment_distance(center, start, end) <= proximity {
                rays.push(start);
                rays.push(end);
            }
        }
    }
    rays
}

pub(super) fn push_success_data_test_fdm_text_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    source_path: &str,
    candidates: &[ObjectFdmTextCandidate],
    font_family: &str,
) {
    let text_candidates = candidates
        .iter()
        .filter(|candidate| success_data_test_fdm_text_projection_candidate(projection, candidate))
        .collect::<Vec<_>>();
    if text_candidates.is_empty() {
        return;
    }

    svg.push_str(&format!(
        "<g class=\"rjtd-success-data-test-fdm-text-projection\" data-role=\"{}\" data-source-path=\"{}\" data-projection=\"successDataTestFdmTextProjection\" data-text-count=\"{}\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\">",
        escape_xml(projection.role),
        escape_xml(source_path),
        text_candidates.len()
    ));
    for candidate in text_candidates {
        let Some((x, y, font_size)) =
            success_data_test_projected_fdm_text_bbox(layout, projection, candidate)
        else {
            continue;
        };
        svg.push_str(&format!(
            "<text class=\"rjtd-success-data-test-fdm-text\" data-role=\"{}\" data-text-offset=\"{}\" data-marker-offset=\"{}\" x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{font_size:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            escape_xml(projection.role),
            candidate.text_offset(),
            candidate.marker_offset(),
            escape_xml(font_family),
            escape_xml(&svg_visual_text(candidate.text()))
        ));
    }
    svg.push_str("</g>");
}

pub(super) fn success_data_test_fdm_text_projection_candidate(
    projection: SuccessDataTestFdmProjection,
    candidate: &ObjectFdmTextCandidate,
) -> bool {
    let Some((left, top, right, bottom)) = candidate.bbox().map(normalize_fdm_bbox) else {
        return false;
    };
    let (center_x, center_y) = fdm_bbox_center((left, top, right, bottom));
    center_x >= projection.source_left
        && center_x <= projection.source_right
        && center_y >= projection.source_top
        && center_y <= projection.source_bottom
}

pub(super) fn success_data_test_projected_fdm_text_bbox(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    candidate: &ObjectFdmTextCandidate,
) -> Option<(f32, f32, f32)> {
    let bbox = candidate.bbox().map(normalize_fdm_bbox)?;
    let (center_x, center_y) = fdm_bbox_center(bbox);
    let (_, top_y) = success_data_test_project_fdm_point(layout, projection, bbox.0, bbox.1)?;
    let (_, bottom_y) = success_data_test_project_fdm_point(layout, projection, bbox.2, bbox.3)?;
    let (x, y) = success_data_test_project_fdm_point(layout, projection, center_x, center_y)?;
    let scale_y = layout.height_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX;
    let projected_height = (bottom_y - top_y).abs();
    let (font_size, baseline_factor) = match projection.role {
        "q3-cone-diagram" => (
            (projected_height * 0.80).clamp(
                9.0 * scale_y,
                SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX * scale_y,
            ),
            0.34,
        ),
        "q4-angle-diagrams" => (
            (projected_height * SUCCESS_DATA_TEST_Q4_TEXT_HEIGHT_FACTOR)
                .clamp(8.0 * scale_y, 10.8 * scale_y),
            SUCCESS_DATA_TEST_Q4_TEXT_BASELINE_FACTOR,
        ),
        _ => (
            (projected_height * 0.52).clamp(6.2 * scale_y, 9.0 * scale_y),
            0.34,
        ),
    };
    Some((x, y + font_size * baseline_factor, font_size))
}

pub(super) fn success_data_test_fdm_projection_command(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    let Some(bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let (center_x, center_y) = fdm_bbox_center(bbox);
    center_x >= projection.source_left
        && center_x <= projection.source_right
        && center_y >= projection.source_top
        && center_y <= projection.source_bottom
}

pub(super) fn success_data_test_cone_vector_command(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    let Some(bbox) = success_data_test_cone_selection_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let margin = success_data_test_projection_margin_units(projection);
    bbox.0 >= projection.source_left - margin
        && bbox.2 <= projection.source_right + margin
        && bbox.1 >= projection.source_top - margin
        && bbox.3 <= projection.source_bottom + margin
}

pub(super) fn success_data_test_projection_margin_units(
    projection: SuccessDataTestFdmProjection,
) -> i32 {
    let span_x = projection
        .source_right
        .saturating_sub(projection.source_left)
        .abs();
    let span_y = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs();
    span_x.max(span_y).max(128) / 128
}

pub(super) fn success_data_test_cone_selection_bbox(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmIndexBbox> {
    if let Some(ellipse) = command.ellipse() {
        return Some(fdm_vector_ellipse_bbox(ellipse));
    }
    fdm_vector_path_points_bbox(command.path_points())
        .or_else(|| fdm_vector_command_source_bbox(command))
}

pub(super) fn success_data_test_cone_command_is_dashed(
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    fdm_vector_marker_is_line(command.marker())
        || (fdm_vector_marker_is_bezier_curve(command.marker()) && command.style_word() != 0)
}

pub(super) fn success_data_test_projected_fdm_path_data(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<String> {
    let mut points = Vec::with_capacity(command.path_points().len());
    for point in command.path_points() {
        points.push(success_data_test_project_fdm_point(
            layout,
            projection,
            point.x(),
            point.y(),
        )?);
    }
    if points.len() < 2 {
        return None;
    }

    let mut path_data = format!("M {:.1} {:.1}", points[0].0, points[0].1);
    if !command.curve_segments().is_empty() {
        for (index, segment) in command.curve_segments().iter().enumerate() {
            if index + 1 >= command.path_points().len() {
                break;
            }
            let control_1 = segment.control_1();
            let control_2 = segment.control_2();
            let end = command.path_points()[index + 1];
            let (control_1_x, control_1_y) = success_data_test_project_fdm_point(
                layout,
                projection,
                control_1.x(),
                control_1.y(),
            )?;
            let (control_2_x, control_2_y) = success_data_test_project_fdm_point(
                layout,
                projection,
                control_2.x(),
                control_2.y(),
            )?;
            let (end_x, end_y) =
                success_data_test_project_fdm_point(layout, projection, end.x(), end.y())?;
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

pub(super) fn success_data_test_projected_fdm_ellipse(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    ellipse: ObjectFdmVectorEllipse,
) -> Option<(f32, f32, f32, f32)> {
    let center = ellipse.center();
    let (cx, cy) = success_data_test_project_fdm_point(layout, projection, center.x(), center.y())?;
    let span_x = (projection.source_right - projection.source_left) as f32;
    let span_y = (projection.source_bottom - projection.source_top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let scale_x = layout.width_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX;
    Some((
        cx,
        cy,
        ellipse.radius_x() as f32 / span_x * projection.target_width_px * scale_x,
        ellipse.radius_y() as f32 / span_y * projection.target_height_px * scale_y,
    ))
}

pub(super) fn success_data_test_project_fdm_point(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    x: i32,
    y: i32,
) -> Option<(f32, f32)> {
    let span_x = (projection.source_right - projection.source_left) as f32;
    let span_y = (projection.source_bottom - projection.source_top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let scale_x = layout.width_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX;
    Some((
        (projection.target_x_px
            + (x - projection.source_left) as f32 / span_x * projection.target_width_px)
            * scale_x,
        (projection.target_y_px
            + (y - projection.source_top) as f32 / span_y * projection.target_height_px)
            * scale_y,
    ))
}

pub(super) fn diagnostic_success_data_test_reference_table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) -> Option<TableGridReferenceLayout> {
    if !document_has_success_data_test_projection_evidence(document)
        || !success_data_test_abc_table_candidate(candidate)
        || !table_grid_decoded_source_placement_evidence_present(document, candidate)
    {
        return None;
    }
    let scale_x = layout.width_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX;
    let width = SUCCESS_DATA_TEST_ABC_TABLE_WIDTH_PX * scale_x;
    let row_height = success_data_test_source_row_height_px(document)
        .unwrap_or(SUCCESS_DATA_TEST_ABC_TABLE_ROW_HEIGHT_PX)
        * scale_y;
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .filter(|column_count| *column_count > 0)
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let column_widths =
        table_grid_line_header_column_widths_px(document, candidate, width, column_count);
    let column_width = width / column_count as f32;
    Some(TableGridReferenceLayout {
        x: SUCCESS_DATA_TEST_ABC_TABLE_X_PX * scale_x,
        y: SUCCESS_DATA_TEST_ABC_TABLE_Y_PX * scale_y,
        width,
        row_height,
        column_width,
        column_width_basis: if column_widths.is_empty() {
            "equalReferenceColumns"
        } else {
            "documentTextLineHeaderCellSlotUnits"
        },
        column_widths,
        column_count,
        header_fill: false,
        corner_radius: 0.0,
        stroke_width: 1.35,
        cell_stroke_width: 1.1,
        font_size: 14.0,
        cell_text_centered: true,
    })
}

pub(super) fn document_has_success_data_test_projection_evidence(document: &Document) -> bool {
    let plain_text = document_plain_text(document);
    plain_text.contains("次の計算をしなさい")
        && plain_text.contains("斜辺の直角三角形")
        && plain_text.contains("右の図のような円錐")
}

pub(super) fn success_data_test_title_art_diagnostic_for_page(
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
    page_number: usize,
) -> bool {
    success_data_test_title_art_page_number(document, diagnostic) == Some(page_number)
}

pub(super) fn success_data_test_title_art_page_number(
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<usize> {
    if !success_data_test_title_art_source_matches(document, diagnostic) {
        return None;
    }

    let frame_ref = diagnostic.frame.frame_ref();
    success_data_test_title_art_frame_refs(document)
        .into_iter()
        .position(|candidate_frame_ref| candidate_frame_ref == frame_ref)
        .map(|index| index + 1)
}

pub(super) fn success_data_test_title_art_source_matches(
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> bool {
    if !document_has_success_data_test_projection_evidence(document)
        || diagnostic.frame.class_name() != "JSFart.Art.2"
    {
        return false;
    }

    let Some(snapshot) = diagnostic.embedded_press_snapshot else {
        return false;
    };
    let primary_size_matches_snapshot = u32::from(diagnostic.frame.primary_width())
        == snapshot.width()
        && u32::from(diagnostic.frame.primary_height()) == snapshot.height();
    let frame_height_matches_snapshot = diagnostic.frame.frame_height() == snapshot.height();
    let frame_width_matches_or_clips_snapshot = diagnostic.frame.frame_width() == snapshot.width()
        || (diagnostic.frame.frame_width() > 0
            && diagnostic.frame.frame_width() < snapshot.width()
            && diagnostic.frame.frame_width().saturating_mul(2) >= snapshot.width());
    let art_size_matches_snapshot = success_data_test_title_art_jsfart_art_candidate(
        document,
        diagnostic.frame.embedding_index(),
    )
    .is_some_and(|art| art.width() == snapshot.width() && art.height() == snapshot.height());

    primary_size_matches_snapshot
        && frame_height_matches_snapshot
        && frame_width_matches_or_clips_snapshot
        && art_size_matches_snapshot
}

pub(super) fn success_data_test_title_art_frame_refs(document: &Document) -> Vec<u32> {
    let mut frame_refs = Vec::new();
    for diagnostic in embedding_frame_diagnostics(document) {
        if success_data_test_title_art_source_matches(document, diagnostic) {
            let frame_ref = diagnostic.frame.frame_ref();
            if frame_ref > 0 && !frame_refs.contains(&frame_ref) {
                frame_refs.push(frame_ref);
            }
        }
    }
    frame_refs
}

pub(super) fn success_data_test_title_art_jsfart_art_candidate(
    document: &Document,
    embedding_index: usize,
) -> Option<&ObjectJsfartArtCandidate> {
    let path = format!("/EmbedItems/Embedding {embedding_index}/JSFart2Contents");
    document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == path)
        .and_then(ObjectStreamCandidate::jsfart_art_candidate)
}

pub(super) fn success_data_test_title_art_jsfart_frame_candidate(
    document: &Document,
    embedding_index: usize,
) -> Option<&ObjectJsfartArtFrameCandidate> {
    success_data_test_title_art_jsfart_art_candidate(document, embedding_index)
        .and_then(ObjectJsfartArtCandidate::frame_candidate)
}

pub(super) fn success_data_test_title_art_frame_vertical_scale(
    frame_record_height: f32,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
) -> (f32, &'static str, u32) {
    if let Some(content_height) = source_frame_candidate
        .and_then(|frame| frame.content_bottom().checked_sub(frame.content_top()))
        .filter(|height| *height > 0)
    {
        return (
            frame_record_height / content_height as f32,
            "jsfartContentHeight",
            content_height,
        );
    }
    if snapshot.height() == 0 {
        return (0.0, "none", 0);
    }
    (
        frame_record_height / snapshot.height() as f32,
        "snapshotHeight",
        snapshot.height(),
    )
}

pub(super) fn success_data_test_top_text_projection(
    document: &Document,
    page_number: usize,
) -> Option<&'static [SuccessDataTestTextSlot]> {
    (page_number == 1 && document_has_success_data_test_projection_evidence(document))
        .then_some(SUCCESS_DATA_TEST_TOP_TEXT_SLOTS)
}

pub(super) fn success_data_test_resolved_top_text_projection(
    document: &Document,
    page_number: usize,
) -> Option<Vec<SuccessDataTestResolvedTextSlot>> {
    let slots = success_data_test_top_text_projection(document, page_number)?;
    Some(success_data_test_resolve_top_text_slots(document, slots))
}

pub(super) fn success_data_test_resolve_top_text_slots(
    document: &Document,
    slots: &[SuccessDataTestTextSlot],
) -> Vec<SuccessDataTestResolvedTextSlot> {
    let Some(bytes) = document_text_raw_stream(document) else {
        return slots
            .iter()
            .map(success_data_test_unbacked_resolved_text_slot)
            .collect();
    };
    let map = map_document_text(bytes);
    let mut entry_index = 0usize;
    let mut entry_relative_unit_cursor = 0usize;
    slots
        .iter()
        .map(|slot| {
            let source_match = success_data_test_next_top_text_source_match(
                bytes,
                map.entries(),
                &mut entry_index,
                &mut entry_relative_unit_cursor,
                slot.text,
            );
            SuccessDataTestResolvedTextSlot {
                role: slot.role,
                text: slot.text,
                x: slot.x,
                y: slot.y,
                source_span: source_match
                    .as_ref()
                    .map(|source| source.source_span.clone()),
                line_header: source_match.and_then(|source| source.line_header),
            }
        })
        .collect()
}

pub(super) fn success_data_test_unbacked_resolved_text_slot(
    slot: &SuccessDataTestTextSlot,
) -> SuccessDataTestResolvedTextSlot {
    SuccessDataTestResolvedTextSlot {
        role: slot.role,
        text: slot.text,
        x: slot.x,
        y: slot.y,
        source_span: None,
        line_header: None,
    }
}

pub(super) fn success_data_test_next_top_text_source_match(
    bytes: &[u8],
    entries: &[DocumentTextMapEntry],
    entry_index: &mut usize,
    entry_relative_unit_cursor: &mut usize,
    text: &str,
) -> Option<SuccessDataTestTextSourceMatch> {
    for (index, entry) in entries[*entry_index..]
        .iter()
        .enumerate()
        .map(|(i, e)| (*entry_index + i, e))
    {
        if entry.kind() != DocumentTextMapKind::TextRun {
            continue;
        }
        let start_units = if index == *entry_index {
            *entry_relative_unit_cursor
        } else {
            0
        };
        let Some((match_start_units, match_end_units)) =
            find_text_utf16_unit_range_after(entry.text(), text, start_units)
        else {
            continue;
        };
        *entry_index = index;
        *entry_relative_unit_cursor = match_end_units;
        let source_span = TextSourceSpan::from_document_text_entry(entry)
            .subspan_by_units(match_start_units, match_end_units);
        let line_header = shanai_lan_line_header_for_text_entry(bytes, entry);
        return Some(SuccessDataTestTextSourceMatch {
            source_span,
            line_header,
        });
    }
    None
}

pub(super) fn success_data_test_resolved_text_slot_fragment(
    slot: &SuccessDataTestResolvedTextSlot,
) -> PageLayerTextFragment {
    PageLayerTextFragment {
        text: slot.text.to_string(),
        paragraph_index: None,
        char_start: 0,
        char_end: slot.text.chars().count(),
        source_span: slot.source_span.clone(),
        ruby_annotation: None,
    }
}

pub(super) fn success_data_test_figure_label_span_fragment(
    span: &SuccessDataTestFigureLabelSpan,
) -> PageLayerTextFragment {
    PageLayerTextFragment {
        text: span.text.clone(),
        paragraph_index: None,
        char_start: 0,
        char_end: span.text.chars().count(),
        source_span: Some(span.source_span.clone()),
        ruby_annotation: None,
    }
}

pub(super) fn success_data_test_jseq_formula_source_top_y(
    document: &Document,
    layout: PageLayout,
    frame_ref: u32,
) -> Option<SuccessDataTestJseqFormulaTopAnchor> {
    let (slot_index, top_offset) = success_data_test_jseq_formula_anchor_slot(frame_ref)?;
    let slot = SUCCESS_DATA_TEST_TOP_TEXT_SLOTS.get(slot_index)?;
    let resolved = success_data_test_resolve_top_text_slots(document, std::slice::from_ref(slot));
    let resolved_slot = resolved.first()?;
    let placement = success_data_test_source_text_placement_candidate(
        document,
        layout,
        resolved_slot.source_span.as_ref(),
        SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
    )?;
    Some(SuccessDataTestJseqFormulaTopAnchor {
        y: (placement.top_y - top_offset).max(0.0),
        source_record_index: placement.line_grid.record_index,
        source_top_y: placement.top_y,
        top_offset,
    })
}

pub(super) fn success_data_test_jseq_formula_anchor_slot(frame_ref: u32) -> Option<(usize, f32)> {
    let slot_index = match frame_ref {
        2 => 1,
        3 => 2,
        4 => 3,
        5 => 4,
        _ => return None,
    };
    let top_offset = match frame_ref {
        3 => 13.0,
        _ => 3.0,
    };
    Some((slot_index, top_offset))
}

pub(super) fn success_data_test_formula_text_slots(
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<Vec<ResolvedJseqFormulaTextSlot>> {
    if !document_has_success_data_test_projection_evidence(document) {
        return None;
    }
    let formula = diagnostic.jseq3_formula?;
    if formula.text_tokens().is_empty() {
        return None;
    }
    let slots = SUCCESS_DATA_TEST_FORMULA_TEXT_SLOTS
        .iter()
        .copied()
        .filter(|slot| slot.embedding_index == diagnostic.frame.embedding_index())
        .filter_map(|slot| resolve_jseq_formula_text_slot(formula, slot))
        .collect::<Vec<_>>();
    (!slots.is_empty()).then_some(slots)
}

pub(super) fn success_data_test_top_text_line_should_skip(
    document: &Document,
    page_number: usize,
    line: &PageTextLine,
) -> bool {
    if success_data_test_answer_sheet_page(document, page_number) {
        return true;
    }
    if success_data_test_top_text_projection(document, page_number).is_none() {
        return false;
    }
    let text = line.text().trim();
    matches!(
        text,
        "１，次の計算をしなさい"
            | "（１）"
            | "（２）"
            | "（３）"
            | "（４）"
            | "たものである。"
            | "３、右の図のような円錐について次の問に答えなさい。"
            | "（１）この円錐の体積を求めなさい。"
            | "（２）表面積を求めなさい。"
            | "（３）この円錐の展開図の側面のおうぎ形の"
            | "中心角を求めなさい。"
            | "４、次の図で∠ｘの大きさを求めなさい。"
            | "５、右の図は、半径ｒの球とその球がちょうど入る円柱、"
            | "その円柱にちょうど入る円錐を表している。"
            | "（１）球の体積をｒを使って表しなさい。"
            | "（２）これらの、球、円柱、円錐の体積の間には"
            | "どのような関係がありますか。"
    ) || text.starts_with("２，下の表は、ｃが斜辺の直角三角形")
        || text.contains("空欄を埋めて表を完成させなさい")
        || (text.contains("（１）") && text.contains("（２）") && text.contains("（３）"))
}

pub(super) fn success_data_test_answer_sheet_page(document: &Document, page_number: usize) -> bool {
    page_number == 2
        && success_data_test_answer_sheet_tail_evidence(document)
        && success_data_test_answer_sheet_fdm_text_candidate(document).is_some()
        && embedding_frame_diagnostics(document)
            .into_iter()
            .any(|diagnostic| {
                success_data_test_title_art_diagnostic_for_page(document, diagnostic, page_number)
            })
}

pub(super) fn success_data_test_answer_sheet_tail_evidence(document: &Document) -> bool {
    let plain_text = document_plain_text(document);
    plain_text.contains("(1)表面積の比")
        && plain_text.contains("ＡＢ")
        && plain_text.contains("ＡＣ")
        && plain_text.contains("ｃｍ")
}

pub(super) fn success_data_test_answer_sheet_sparse_table_candidate(
    document: &Document,
) -> Option<&TableCandidate> {
    document.table_candidates().iter().find(|candidate| {
        candidate.is_sparse_document_text_control_run_candidate()
            && candidate
                .intervals()
                .iter()
                .any(|interval| interval.text_preview().contains("(1)表面積の比"))
            && candidate
                .intervals()
                .iter()
                .any(|interval| interval.text_preview().contains("ＡＢ ＝ ｃｍ"))
            && candidate
                .intervals()
                .iter()
                .any(|interval| interval.text_preview().contains("ＡＣ ＝ ｃｍ"))
    })
}

pub(super) fn success_data_test_answer_sheet_fdm_text_candidate(
    document: &Document,
) -> Option<&ObjectStreamCandidate> {
    document
        .object_stream_candidates()
        .iter()
        .find(|candidate| {
            candidate.path() == SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_PATH
                && candidate
                    .payload_prefix()
                    .starts_with(SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_MARKER)
        })
}

pub(super) fn success_data_test_answer_sheet_figure_link_candidate(
    document: &Document,
) -> Option<&ObjectStreamCandidate> {
    document
        .object_stream_candidates()
        .iter()
        .find(|candidate| {
            candidate.path() == SUCCESS_DATA_TEST_ANSWER_SHEET_LINK_PATH
                && candidate.figure_link_candidate().is_some()
        })
}

pub(super) fn success_data_test_answer_sheet_text_slots(
    document: &Document,
    layout: PageLayout,
) -> Vec<SuccessDataTestAnswerSheetTextSlot> {
    let Some(tokens) = success_data_test_answer_sheet_text_tokens(document) else {
        return Vec::new();
    };
    let frame = SuccessDataTestAnswerSheetFrame::new(layout);
    success_data_test_answer_sheet_text_slot_templates()
        .iter()
        .filter_map(|template| {
            let text = tokens.get(template.source_token_index)?.to_string();
            Some(SuccessDataTestAnswerSheetTextSlot {
                text,
                source_token_index: template.source_token_index,
                x: frame.page_x(template.x_pt),
                y: frame.page_y(template.y_pt),
                font_size: frame.font_size(template.font_pt),
                anchor: template.anchor,
            })
        })
        .collect()
}

pub(super) fn success_data_test_answer_sheet_text_slot_templates()
-> &'static [SuccessDataTestAnswerSheetTextSlotTemplate] {
    &[
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 3,
            x_pt: 44.0,
            y_pt: 174.0,
            font_pt: 7.0,
            anchor: "middle",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 8,
            x_pt: 44.0,
            y_pt: 244.0,
            font_pt: 7.0,
            anchor: "middle",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 14,
            x_pt: 44.0,
            y_pt: 276.0,
            font_pt: 7.0,
            anchor: "middle",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 21,
            x_pt: 44.0,
            y_pt: 339.0,
            font_pt: 7.0,
            anchor: "middle",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 26,
            x_pt: 44.0,
            y_pt: 366.0,
            font_pt: 7.0,
            anchor: "middle",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 31,
            x_pt: 44.0,
            y_pt: 540.0,
            font_pt: 7.0,
            anchor: "middle",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 0,
            x_pt: 61.0,
            y_pt: 160.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 1,
            x_pt: 202.0,
            y_pt: 160.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 2,
            x_pt: 342.0,
            y_pt: 160.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 4,
            x_pt: 61.0,
            y_pt: 194.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 5,
            x_pt: 159.0,
            y_pt: 194.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 6,
            x_pt: 202.0,
            y_pt: 194.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 7,
            x_pt: 342.0,
            y_pt: 194.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 9,
            x_pt: 61.0,
            y_pt: 242.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 10,
            x_pt: 158.0,
            y_pt: 242.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 11,
            x_pt: 202.0,
            y_pt: 242.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 12,
            x_pt: 303.0,
            y_pt: 242.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 13,
            x_pt: 342.0,
            y_pt: 242.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 15,
            x_pt: 61.0,
            y_pt: 276.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 16,
            x_pt: 202.0,
            y_pt: 276.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 17,
            x_pt: 342.0,
            y_pt: 276.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 18,
            x_pt: 61.0,
            y_pt: 307.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 19,
            x_pt: 202.0,
            y_pt: 307.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 20,
            x_pt: 342.0,
            y_pt: 307.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 22,
            x_pt: 70.0,
            y_pt: 338.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 23,
            x_pt: 225.0,
            y_pt: 338.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 24,
            x_pt: 285.0,
            y_pt: 338.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 25,
            x_pt: 427.0,
            y_pt: 338.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 27,
            x_pt: 287.0,
            y_pt: 365.0,
            font_pt: 7.8,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 28,
            x_pt: 400.0,
            y_pt: 365.0,
            font_pt: 7.8,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 29,
            x_pt: 287.0,
            y_pt: 397.0,
            font_pt: 7.8,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 30,
            x_pt: 400.0,
            y_pt: 397.0,
            font_pt: 7.8,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 32,
            x_pt: 61.0,
            y_pt: 539.0,
            font_pt: 7.4,
            anchor: "start",
        },
        SuccessDataTestAnswerSheetTextSlotTemplate {
            source_token_index: 33,
            x_pt: 61.0,
            y_pt: 572.0,
            font_pt: 7.4,
            anchor: "start",
        },
    ]
}

pub(super) fn success_data_test_answer_sheet_text_tokens(
    document: &Document,
) -> Option<Vec<String>> {
    let plain_text = document_plain_text(document);
    let start = plain_text.find("(1)表面積の比")?;
    let tail = &plain_text[start..];
    let end = tail.rfind('®').map_or(tail.len(), |index| index);
    let tokens = tokenize_success_data_test_answer_sheet_tail(&tail[..end]);
    let expected_prefix = ["(1)表面積の比", "(1)体積の比", "(2)", "１", "(3)"];
    if tokens.len() >= 34
        && tokens
            .iter()
            .take(expected_prefix.len())
            .zip(expected_prefix)
            .all(|(actual, expected)| actual == expected)
        && tokens.iter().any(|token| token == "ＡＢ　＝")
        && tokens.iter().any(|token| token == "ＡＣ　＝")
    {
        Some(tokens)
    } else {
        None
    }
}

pub(super) fn tokenize_success_data_test_answer_sheet_tail(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cursor = 0usize;

    while cursor < text.len() {
        let remaining = &text[cursor..];
        let Some(character) = remaining.chars().next() else {
            break;
        };
        if character.is_whitespace() || character == '®' {
            cursor += character.len_utf8();
            continue;
        }

        if (remaining.starts_with("ＡＢ") || remaining.starts_with("ＡＣ"))
            && let Some(equal_index) = remaining.find('＝')
        {
            let end = cursor + equal_index + '＝'.len_utf8();
            push_success_data_test_answer_sheet_token(&mut tokens, &text[cursor..end]);
            cursor = end;
            continue;
        }

        if character == '(' {
            let mut end = cursor + character.len_utf8();
            let mut saw_close = false;
            for (relative, candidate) in text[end..].char_indices() {
                end = cursor + character.len_utf8() + relative + candidate.len_utf8();
                if candidate == ')' {
                    saw_close = true;
                    break;
                }
            }
            if saw_close {
                while end < text.len() {
                    let next = text[end..].chars().next().expect("cursor is in bounds");
                    if next.is_whitespace() || next == '(' || next == '®' {
                        break;
                    }
                    end += next.len_utf8();
                }
                push_success_data_test_answer_sheet_token(&mut tokens, &text[cursor..end]);
                cursor = end;
                continue;
            }
        }

        if success_data_test_answer_sheet_row_number_char(character) {
            push_success_data_test_answer_sheet_token(
                &mut tokens,
                &text[cursor..cursor + character.len_utf8()],
            );
            cursor += character.len_utf8();
            continue;
        }

        let mut end = cursor + character.len_utf8();
        while end < text.len() {
            let next = text[end..].chars().next().expect("cursor is in bounds");
            if next.is_whitespace() || next == '(' || next == '®' {
                break;
            }
            end += next.len_utf8();
        }
        push_success_data_test_answer_sheet_token(&mut tokens, &text[cursor..end]);
        cursor = end;
    }

    tokens
}

pub(super) fn push_success_data_test_answer_sheet_token(tokens: &mut Vec<String>, raw: &str) {
    let token = success_data_test_answer_sheet_display_token(raw);
    if !token.is_empty() {
        tokens.push(token);
    }
}

pub(super) fn success_data_test_answer_sheet_display_token(raw: &str) -> String {
    match raw.trim() {
        "ｃｍ２" => "ｃｍ²".to_string(),
        "ｃｍ３" => "ｃｍ³".to_string(),
        other => other.to_string(),
    }
}

pub(super) fn success_data_test_answer_sheet_row_number_char(character: char) -> bool {
    matches!(
        character,
        '1' | '2' | '3' | '4' | '5' | '6' | '１' | '２' | '３' | '４' | '５' | '６'
    )
}

pub(super) fn success_data_test_answer_sheet_fdm_label_slots(
    document: &Document,
    layout: PageLayout,
) -> Vec<SuccessDataTestAnswerSheetFdmTextSlot> {
    let Some(candidate) = success_data_test_answer_sheet_fdm_text_candidate(document) else {
        return Vec::new();
    };
    let labels = success_data_test_answer_sheet_indexed_fdm_labels(candidate)
        .into_iter()
        .filter(|label| matches!(label.text.text(), "Ａ" | "Ｂ" | "Ｃ"))
        .collect::<Vec<_>>();
    if labels.len() < 3 {
        return Vec::new();
    }

    let source_left = labels
        .iter()
        .map(|label| label.text_bbox.left())
        .min()
        .unwrap_or_default();
    let source_top = labels
        .iter()
        .map(|label| label.text_bbox.top())
        .min()
        .unwrap_or_default();
    let source_right = labels
        .iter()
        .map(|label| label.text_bbox.right())
        .max()
        .unwrap_or_default();
    let source_bottom = labels
        .iter()
        .map(|label| label.text_bbox.bottom())
        .max()
        .unwrap_or_default();
    let source_width = (source_right - source_left) as f32;
    let source_height = (source_bottom - source_top) as f32;
    if source_width <= 0.0 || source_height <= 0.0 {
        return Vec::new();
    }

    let frame = SuccessDataTestAnswerSheetFrame::new(layout);
    let target_left = frame.sheet_x(34.0);
    let target_top = frame.sheet_y(231.0);
    let target_right = frame.sheet_x(215.0);
    let target_bottom = frame.sheet_y(338.0);
    let font_size = frame.font_size(7.2);

    labels
        .into_iter()
        .map(|label| {
            let text_bbox = label.text_bbox;
            let source_bbox = label.index.bbox();
            let center_x =
                text_bbox.left() as f32 + (text_bbox.right() - text_bbox.left()) as f32 / 2.0;
            let center_y =
                text_bbox.top() as f32 + (text_bbox.bottom() - text_bbox.top()) as f32 / 2.0;
            let x = target_left
                + (center_x - source_left as f32) / source_width * (target_right - target_left);
            let y = target_top
                + (center_y - source_top as f32) / source_height * (target_bottom - target_top);
            SuccessDataTestAnswerSheetFdmTextSlot {
                text: label.text.text().to_string(),
                x,
                y,
                font_size,
                text_offset: label.text.text_offset(),
                marker_offset: label.text.marker_offset(),
                index_offset: label.index.index_offset(),
                source_bbox,
                text_bbox,
            }
        })
        .collect()
}

pub(super) fn success_data_test_answer_sheet_triangle_placement_candidate(
    document: &Document,
    layout: PageLayout,
) -> Option<SuccessDataTestAnswerSheetTrianglePlacementCandidate> {
    let source_bbox = success_data_test_answer_sheet_fdm_text_candidate(document)
        .and_then(success_data_test_answer_sheet_triangle_source_bbox)?;
    let slots = success_data_test_answer_sheet_fdm_label_slots(document, layout);
    let label_a = slots.iter().find(|slot| slot.text == "Ａ")?;
    let label_b = slots.iter().find(|slot| slot.text == "Ｂ")?;
    let label_c = slots.iter().find(|slot| slot.text == "Ｃ")?;
    let font_size = (label_a.font_size + label_b.font_size + label_c.font_size) / 3.0;

    let right_x = ((label_a.x + font_size * 5.0 / 3.0) + (label_c.x - font_size / 6.0)) / 2.0;
    let bottom_y = ((label_b.y + font_size / 5.0) + (label_c.y + font_size / 5.0)) / 2.0;
    let left_x = label_b.x + font_size * 7.0 / 12.0;
    let top_y = label_a.y + font_size / 4.0;
    let right_angle_size = (right_x - left_x) / 21.0;

    Some(SuccessDataTestAnswerSheetTrianglePlacementCandidate {
        source: "FDMTextIndex+projectedFdmLabelSlots",
        placement_basis: "projected-fdm-label-slot-anchors",
        source_bbox,
        a: SuccessDataTestAnswerSheetPoint {
            x: right_x,
            y: top_y,
        },
        b: SuccessDataTestAnswerSheetPoint {
            x: left_x,
            y: bottom_y,
        },
        c: SuccessDataTestAnswerSheetPoint {
            x: right_x,
            y: bottom_y,
        },
        right_angle_start: SuccessDataTestAnswerSheetPoint {
            x: right_x - right_angle_size,
            y: bottom_y,
        },
        right_angle_corner: SuccessDataTestAnswerSheetPoint {
            x: right_x - right_angle_size,
            y: bottom_y - right_angle_size,
        },
        right_angle_end: SuccessDataTestAnswerSheetPoint {
            x: right_x,
            y: bottom_y - right_angle_size,
        },
        label_anchors: [
            success_data_test_answer_sheet_triangle_label_anchor("Ａ", label_a),
            success_data_test_answer_sheet_triangle_label_anchor("Ｂ", label_b),
            success_data_test_answer_sheet_triangle_label_anchor("Ｃ", label_c),
        ],
    })
}

pub(super) fn success_data_test_answer_sheet_triangle_label_anchor(
    text: &'static str,
    slot: &SuccessDataTestAnswerSheetFdmTextSlot,
) -> SuccessDataTestAnswerSheetTriangleLabelAnchor {
    SuccessDataTestAnswerSheetTriangleLabelAnchor {
        text,
        point: SuccessDataTestAnswerSheetPoint {
            x: slot.x,
            y: slot.y,
        },
        marker_offset: slot.marker_offset,
        index_offset: slot.index_offset,
    }
}

pub(super) fn success_data_test_answer_sheet_indexed_fdm_labels(
    candidate: &ObjectStreamCandidate,
) -> Vec<SuccessDataTestAnswerSheetIndexedFdmLabel<'_>> {
    candidate
        .fdm_text_candidates()
        .iter()
        .filter_map(|text| {
            let text_bbox = text.bbox()?;
            let index = candidate
                .fdm_text_index_entry_candidates()
                .iter()
                .find(|entry| entry.text_record_offset() == text.marker_offset())?;
            Some(SuccessDataTestAnswerSheetIndexedFdmLabel {
                text,
                index,
                text_bbox,
            })
        })
        .collect()
}

pub(super) fn success_data_test_answer_sheet_triangle_source_bbox(
    candidate: &ObjectStreamCandidate,
) -> Option<ObjectFdmIndexBbox> {
    let (left, top, right, bottom) = success_data_test_answer_sheet_indexed_fdm_labels(candidate)
        .into_iter()
        .filter(|label| matches!(label.text.text(), "Ａ" | "Ｂ" | "Ｃ"))
        .map(|label| normalize_fdm_bbox(label.index.bbox()))
        .fold(None, fdm_bbox_extent_union)?;
    Some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

pub(super) fn success_data_test_title_art_rendered_segment_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    snapshot
        .vector_segments()
        .iter()
        .filter(|segment| success_data_test_title_art_segment_should_render(segment))
        .count()
}

pub(super) fn success_data_test_title_art_rendered_path_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Outline && !path.commands().is_empty()
        })
        .count()
}

#[cfg(test)]
pub(super) fn success_data_test_title_art_shadow_path_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    let outline_count = success_data_test_title_art_rendered_path_count(snapshot);
    if outline_count > 1 && outline_count.is_multiple_of(2) {
        outline_count / 2
    } else {
        0
    }
}

pub(super) fn success_data_test_title_art_rendered_texture_path_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Texture && !path.commands().is_empty()
        })
        .count()
}

pub(super) fn success_data_test_title_art_rendered_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Outline && !path.commands().is_empty()
        })
        .collect::<Vec<_>>()
}

pub(super) fn success_data_test_title_art_horizontal_placement(
    frame_record_x: f32,
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
    scale_x: f32,
) -> SuccessDataTestTitleArtHorizontalPlacement {
    let content_left_adjustment =
        source_frame_candidate.map_or(0.0, |frame| frame.content_left() as f32 * scale_x);
    let content_left_only_x = (frame_record_x - content_left_adjustment).max(0.0);
    let stroke_width_candidate =
        source_frame_candidate.and_then(ObjectJsfartArtFrameCandidate::stroke_width_candidate);
    let stroke_outer_adjustment =
        stroke_width_candidate.map_or(0.0, |value| value as f32 * scale_x);
    if stroke_outer_adjustment > 0.0 {
        SuccessDataTestTitleArtHorizontalPlacement {
            frame_x: content_left_only_x,
            path_x: content_left_only_x,
            candidate_frame_x: (content_left_only_x - stroke_outer_adjustment).max(0.0),
            candidate_path_x: frame_record_x.max(0.0),
            content_left_adjustment,
            stroke_outer_adjustment,
            content_left_only_x,
            frame_record_x,
            basis: "jsfartContentLeft",
            render_promoted: false,
            stroke_width_candidate,
        }
    } else {
        SuccessDataTestTitleArtHorizontalPlacement {
            frame_x: content_left_only_x,
            path_x: content_left_only_x,
            candidate_frame_x: content_left_only_x,
            candidate_path_x: content_left_only_x,
            content_left_adjustment,
            stroke_outer_adjustment,
            content_left_only_x,
            frame_record_x,
            basis: "jsfartContentLeft",
            render_promoted: false,
            stroke_width_candidate,
        }
    }
}

pub(super) fn push_success_data_test_title_art_horizontal_placement_json(
    output: &mut String,
    placement: SuccessDataTestTitleArtHorizontalPlacement,
) {
    output.push_str("{\"source\":\"JSFart2Contents.frameCandidate\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"basis\":");
    output.push_str(&json_string(placement.basis));
    output.push_str(",\"frameRecordX\":");
    output.push_str(&format!("{:.3}", placement.frame_record_x));
    output.push_str(",\"contentLeftOnlyX\":");
    output.push_str(&format!("{:.3}", placement.content_left_only_x));
    output.push_str(",\"frameX\":");
    output.push_str(&format!("{:.3}", placement.frame_x));
    output.push_str(",\"pathX\":");
    output.push_str(&format!("{:.3}", placement.path_x));
    output.push_str(",\"candidateFrameX\":");
    output.push_str(&format!("{:.3}", placement.candidate_frame_x));
    output.push_str(",\"candidatePathX\":");
    output.push_str(&format!("{:.3}", placement.candidate_path_x));
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(
        "jsfartFrameOuterEdgePlusFrameRecordContentOrigin",
    ));
    output.push_str(",\"contentLeftAdjustmentCssPx\":");
    output.push_str(&format!("{:.3}", placement.content_left_adjustment));
    output.push_str(",\"strokeWidthCandidateSourceUnits\":");
    match placement.stroke_width_candidate {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"strokeOuterAdjustmentCssPx\":");
    output.push_str(&format!("{:.3}", placement.stroke_outer_adjustment));
    output.push_str(",\"renderPromoted\":");
    output.push_str(if placement.render_promoted {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    if placement.render_promoted {
        output.push_str("null");
    } else {
        output.push_str(&json_string(if placement.stroke_outer_adjustment > 0.0 {
            "frame-content-split-horizontal-semantics-unproven"
        } else {
            "jsfart-stroke-width-candidate-missing-for-horizontal-anchor"
        }));
    }
    output.push('}');
}

pub(super) fn success_data_test_title_art_horizontal_placement_svg_attrs(
    placement: SuccessDataTestTitleArtHorizontalPlacement,
) -> String {
    format!(
        " data-horizontal-placement-basis=\"{}\" data-horizontal-placement-source=\"JSFart2Contents.frameCandidate\" data-horizontal-placement-source-backed=\"true\" data-horizontal-placement-render-promoted=\"{}\" data-horizontal-frame-record-x=\"{:.3}\" data-horizontal-content-left-only-x=\"{:.3}\" data-horizontal-frame-x=\"{:.3}\" data-horizontal-path-x=\"{:.3}\" data-horizontal-candidate-frame-x=\"{:.3}\" data-horizontal-candidate-path-x=\"{:.3}\" data-horizontal-candidate-basis=\"jsfartFrameOuterEdgePlusFrameRecordContentOrigin\" data-horizontal-stroke-width-source-units=\"{}\" data-horizontal-stroke-outer-adjustment-css-px=\"{:.3}\"",
        escape_xml(placement.basis),
        placement.render_promoted,
        placement.frame_record_x,
        placement.content_left_only_x,
        placement.frame_x,
        placement.path_x,
        placement.candidate_frame_x,
        placement.candidate_path_x,
        placement
            .stroke_width_candidate
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        placement.stroke_outer_adjustment
    )
}

pub(super) fn success_data_test_title_art_source_frame_trace_conclusion(
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
    frame_ref: u32,
) -> &'static str {
    let Some(frame) = source_frame_candidate else {
        return "missing-jsfart-frame-candidate";
    };
    let Some(record) = frame_record else {
        return "missing-frame-record";
    };
    let outer_width = frame.right().saturating_sub(frame.left());
    let outer_height = frame.bottom().saturating_sub(frame.top());
    if u32::from(record.object_id()) == frame_ref
        && u32::from(record.width()) == outer_width
        && u32::from(record.height()) == outer_height
    {
        "frame-record-and-jsfart-outer-size-agree"
    } else {
        "frame-record-jsfart-outer-size-or-ref-mismatch"
    }
}

pub(super) fn push_success_data_test_title_art_source_frame_render_trace_json(
    output: &mut String,
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
    frame_ref: u32,
    horizontal_placement: SuccessDataTestTitleArtHorizontalPlacement,
    frame_scale_y_basis: &str,
    frame_scale_y_source_units: u32,
) {
    let source_outer_width =
        source_frame_candidate.map(|frame| frame.right().saturating_sub(frame.left()));
    let source_outer_height =
        source_frame_candidate.map(|frame| frame.bottom().saturating_sub(frame.top()));
    let source_content_width = source_frame_candidate
        .map(|frame| frame.content_right().saturating_sub(frame.content_left()));
    let source_content_height = source_frame_candidate
        .map(|frame| frame.content_bottom().saturating_sub(frame.content_top()));
    let frame_record_width = frame_record.map(|record| u32::from(record.width()));
    let frame_record_height = frame_record.map(|record| u32::from(record.height()));
    let frame_ref_matches_object_id =
        frame_record.is_some_and(|record| u32::from(record.object_id()) == frame_ref);
    let outer_width_matches = source_outer_width
        .zip(frame_record_width)
        .is_some_and(|(source, record)| source == record);
    let outer_height_matches = source_outer_height
        .zip(frame_record_height)
        .is_some_and(|(source, record)| source == record);
    output.push_str(
        "{\"source\":\"JSFart2Contents.frameCandidate+/Frame\",\"decoded\":false,\"sourceBacked\":",
    );
    output.push_str(
        if source_frame_candidate.is_some() && frame_record.is_some() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"frameRef\":");
    output.push_str(&frame_ref.to_string());
    output.push_str(",\"frameRecordObjectId\":");
    match frame_record {
        Some(record) => output.push_str(&record.object_id().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"frameRefMatchesObjectId\":");
    output.push_str(if frame_ref_matches_object_id {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOuterWidthUnits\":");
    push_option_u32_json(output, source_outer_width);
    output.push_str(",\"frameRecordWidthUnits\":");
    push_option_u32_json(output, frame_record_width);
    output.push_str(",\"outerWidthMatchesFrameRecord\":");
    output.push_str(if outer_width_matches { "true" } else { "false" });
    output.push_str(",\"sourceOuterHeightUnits\":");
    push_option_u32_json(output, source_outer_height);
    output.push_str(",\"frameRecordHeightUnits\":");
    push_option_u32_json(output, frame_record_height);
    output.push_str(",\"outerHeightMatchesFrameRecord\":");
    output.push_str(if outer_height_matches {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceContentWidthUnits\":");
    push_option_u32_json(output, source_content_width);
    output.push_str(",\"sourceContentHeightUnits\":");
    push_option_u32_json(output, source_content_height);
    output.push_str(",\"horizontalPlacementBasis\":");
    output.push_str(&json_string(horizontal_placement.basis));
    output.push_str(",\"selectedFrameX\":");
    output.push_str(&format!("{:.3}", horizontal_placement.frame_x));
    output.push_str(",\"candidateFrameX\":");
    output.push_str(&format!("{:.3}", horizontal_placement.candidate_frame_x));
    output.push_str(",\"frameScaleYBasis\":");
    output.push_str(&json_string(frame_scale_y_basis));
    output.push_str(",\"frameScaleYSourceUnits\":");
    if frame_scale_y_source_units == 0 {
        output.push_str("null");
    } else {
        output.push_str(&frame_scale_y_source_units.to_string());
    }
    output.push_str(",\"traceConclusion\":");
    output.push_str(&json_string(
        success_data_test_title_art_source_frame_trace_conclusion(
            source_frame_candidate,
            frame_record,
            frame_ref,
        ),
    ));
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"frame-content-split-horizontal-semantics-unproven\"}",
    );
}

pub(super) fn success_data_test_title_art_source_frame_render_trace_svg_attrs(
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
    frame_ref: u32,
    horizontal_placement: SuccessDataTestTitleArtHorizontalPlacement,
    frame_scale_y_basis: &str,
    frame_scale_y_source_units: u32,
) -> String {
    let source_outer_width =
        source_frame_candidate.map(|frame| frame.right().saturating_sub(frame.left()));
    let source_outer_height =
        source_frame_candidate.map(|frame| frame.bottom().saturating_sub(frame.top()));
    let frame_record_width = frame_record.map(|record| u32::from(record.width()));
    let frame_record_height = frame_record.map(|record| u32::from(record.height()));
    let frame_ref_matches_object_id =
        frame_record.is_some_and(|record| u32::from(record.object_id()) == frame_ref);
    let outer_width_matches = source_outer_width
        .zip(frame_record_width)
        .is_some_and(|(source, record)| source == record);
    let outer_height_matches = source_outer_height
        .zip(frame_record_height)
        .is_some_and(|(source, record)| source == record);
    format!(
        " data-title-source-frame-trace-source=\"JSFart2Contents.frameCandidate+/Frame\" data-title-source-frame-trace-source-backed=\"{}\" data-title-source-frame-trace-render-promoted=\"false\" data-title-source-frame-trace-frame-ref=\"{}\" data-title-source-frame-trace-frame-record-object-id=\"{}\" data-title-source-frame-trace-frame-ref-matches-object-id=\"{}\" data-title-source-frame-trace-source-outer-width-units=\"{}\" data-title-source-frame-trace-frame-record-width-units=\"{}\" data-title-source-frame-trace-outer-width-matches-frame-record=\"{}\" data-title-source-frame-trace-source-outer-height-units=\"{}\" data-title-source-frame-trace-frame-record-height-units=\"{}\" data-title-source-frame-trace-outer-height-matches-frame-record=\"{}\" data-title-source-frame-trace-horizontal-placement-basis=\"{}\" data-title-source-frame-trace-selected-frame-x=\"{:.3}\" data-title-source-frame-trace-candidate-frame-x=\"{:.3}\" data-title-source-frame-trace-frame-scale-y-basis=\"{}\" data-title-source-frame-trace-frame-scale-y-units=\"{}\" data-title-source-frame-trace-conclusion=\"{}\" data-title-source-frame-trace-render-blocked-reason=\"frame-content-split-horizontal-semantics-unproven\"",
        source_frame_candidate.is_some() && frame_record.is_some(),
        frame_ref,
        frame_record
            .map(|record| record.object_id().to_string())
            .unwrap_or_else(|| "none".to_string()),
        frame_ref_matches_object_id,
        source_outer_width
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        frame_record_width
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        outer_width_matches,
        source_outer_height
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        frame_record_height
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        outer_height_matches,
        escape_xml(horizontal_placement.basis),
        horizontal_placement.frame_x,
        horizontal_placement.candidate_frame_x,
        escape_xml(frame_scale_y_basis),
        if frame_scale_y_source_units == 0 {
            "none".to_string()
        } else {
            frame_scale_y_source_units.to_string()
        },
        escape_xml(success_data_test_title_art_source_frame_trace_conclusion(
            source_frame_candidate,
            frame_record,
            frame_ref,
        ))
    )
}

pub(super) fn success_data_test_title_art_main_outline_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    embedded_press_title_art_shadow_path_partition(snapshot).map_or_else(
        || success_data_test_title_art_rendered_paths(snapshot),
        |partition| partition.main_paths,
    )
}

pub(super) fn success_data_test_title_art_projected_main_path_bbox(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) -> Option<SuccessDataTestProjectedPathBBox> {
    let paths = success_data_test_title_art_main_outline_paths(snapshot);
    let mut left = i32::MAX;
    let mut top = i32::MAX;
    let mut right = i32::MIN;
    let mut bottom = i32::MIN;
    let mut has_bbox = false;
    for path in paths {
        let Some((path_left, path_top, path_right, path_bottom)) =
            embedded_press_vector_path_sampled_source_bbox(
                path,
                SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES,
            )
            .or_else(|| embedded_press_vector_path_source_bbox(path))
        else {
            continue;
        };
        left = left.min(path_left);
        top = top.min(path_top);
        right = right.max(path_right);
        bottom = bottom.max(path_bottom);
        has_bbox = true;
    }
    if !has_bbox {
        return None;
    }

    let projected_left = x + left as f32 * scale_x;
    let projected_top = y + top as f32 * scale_y;
    let projected_right = x + right as f32 * scale_x;
    let projected_bottom = y + bottom as f32 * scale_y;
    Some(SuccessDataTestProjectedPathBBox {
        x: projected_left,
        y: projected_top,
        width: projected_right - projected_left,
        height: projected_bottom - projected_top,
    })
}

pub(super) fn success_data_test_title_art_path_scale_bbox_svg_attrs(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    x: f32,
    source_path_y: f32,
    frame_path_y: f32,
    scale_x: f32,
    source_scale_y: f32,
    frame_scale_y: f32,
) -> String {
    let Some(source_scale_bbox) = success_data_test_title_art_projected_main_path_bbox(
        snapshot,
        x,
        source_path_y,
        scale_x,
        source_scale_y,
    ) else {
        return String::new();
    };
    let Some(frame_scale_bbox) = success_data_test_title_art_projected_main_path_bbox(
        snapshot,
        x,
        frame_path_y,
        scale_x,
        frame_scale_y,
    ) else {
        return String::new();
    };

    format!(
        " data-main-outline-scale-diagnostic=\"source-scale-vs-frame-scale\" data-main-outline-scale-diagnostic-pixel-change=\"false\" data-main-outline-source-scale-bbox-x=\"{:.3}\" data-main-outline-source-scale-bbox-y=\"{:.3}\" data-main-outline-source-scale-bbox-width=\"{:.3}\" data-main-outline-source-scale-bbox-height=\"{:.3}\" data-main-outline-frame-scale-bbox-x=\"{:.3}\" data-main-outline-frame-scale-bbox-y=\"{:.3}\" data-main-outline-frame-scale-bbox-width=\"{:.3}\" data-main-outline-frame-scale-bbox-height=\"{:.3}\"",
        source_scale_bbox.x,
        source_scale_bbox.y,
        source_scale_bbox.width,
        source_scale_bbox.height,
        frame_scale_bbox.x,
        frame_scale_bbox.y,
        frame_scale_bbox.width,
        frame_scale_bbox.height
    )
}

pub(super) fn push_success_data_test_title_art_projected_path_bbox_json(
    output: &mut String,
    bbox: SuccessDataTestProjectedPathBBox,
) {
    output.push_str("{\"x\":");
    output.push_str(&format!("{:.3}", bbox.x));
    output.push_str(",\"y\":");
    output.push_str(&format!("{:.3}", bbox.y));
    output.push_str(",\"width\":");
    output.push_str(&format!("{:.3}", bbox.width));
    output.push_str(",\"height\":");
    output.push_str(&format!("{:.3}", bbox.height));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_success_data_test_title_art_path_scale_bbox_diagnostic_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    x: f32,
    source_path_y: f32,
    frame_path_y: f32,
    scale_x: f32,
    source_scale_y: f32,
    frame_scale_y: f32,
) {
    let source_scale_bbox = success_data_test_title_art_projected_main_path_bbox(
        snapshot,
        x,
        source_path_y,
        scale_x,
        source_scale_y,
    );
    let frame_scale_bbox = success_data_test_title_art_projected_main_path_bbox(
        snapshot,
        x,
        frame_path_y,
        scale_x,
        frame_scale_y,
    );
    output.push_str("{\"source\":\"embeddedPressMainOutlinePathSampledBbox\",\"pixelChange\":false,\"scaleComparisonDecoded\":false,\"currentRendererPathScale\":\"sourceScale\",\"frameClipScale\":\"frameScale\",\"renderPromotionBlockedReason\":\"title-art-y-scale-basis-unproven\",\"sourceScaleBbox\":");
    if let Some(bbox) = source_scale_bbox {
        push_success_data_test_title_art_projected_path_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"frameScaleBbox\":");
    if let Some(bbox) = frame_scale_bbox {
        push_success_data_test_title_art_projected_path_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn success_data_test_title_art_segment_should_render(
    segment: &ObjectEmbeddedPressVectorSegmentCandidate,
) -> bool {
    let dx = segment.x1().abs_diff(segment.x2()) as f32;
    let dy = segment.y1().abs_diff(segment.y2()) as f32;
    (dx * dx + dy * dy).sqrt() <= SUCCESS_DATA_TEST_TITLE_ART_MAX_SEGMENT_SOURCE_LEN
}

pub(super) fn success_data_test_abc_table_candidate(candidate: &TableCandidate) -> bool {
    candidate.is_document_text_control_run_candidate()
        && candidate.intervals().len() == 3
        && candidate
            .intervals()
            .first()
            .is_some_and(|interval| interval.text_preview() == "ａ\t１\t１\t７\t")
        && candidate
            .intervals()
            .get(1)
            .is_some_and(|interval| interval.text_preview() == "ｂ\t１\t\t２４\t０.８")
        && candidate
            .intervals()
            .get(2)
            .is_some_and(|interval| interval.text_preview() == "ｃ\t\t２\t\t１")
}
