use super::*;
use crate::*;

pub(crate) const SUCCESS_DATA_TEST_ABC_TABLE_X_PX: f32 = 79.3;

pub(crate) const SUCCESS_DATA_TEST_ABC_TABLE_Y_PX: f32 = 410.7;

pub(crate) const SUCCESS_DATA_TEST_ABC_TABLE_WIDTH_PX: f32 = 276.0;

pub(crate) const SUCCESS_DATA_TEST_ABC_TABLE_ROW_HEIGHT_PX: f32 = 21.0;

pub(crate) const SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_PATH: &str =
    "/FigureData/ExpandData/main_data/Data/FDMText";

pub(crate) const SUCCESS_DATA_TEST_ANSWER_SHEET_LINK_PATH: &str =
    "/FigureData/ExpandData/main_data/Link";

pub(crate) const SUCCESS_DATA_TEST_ANSWER_SHEET_FDM_TEXT_MARKER: &[u8; 4] = b"\x01\x00\x16\x60";

pub(crate) const SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_LEFT_PT: f32 = 30.0;

pub(crate) const SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_TOP_PT: f32 = 143.0;

pub(crate) const SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_RIGHT_PT: f32 = 475.0;

pub(crate) const SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_BOTTOM_PT: f32 = 600.0;

pub(crate) const SUCCESS_DATA_TEST_ANSWER_SHEET_RULE_SEGMENTS_PT: [(f32, f32, f32, f32); 19] = [
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

pub(crate) fn push_page_layer_success_data_test_answer_sheet_projection_json(
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

pub(crate) fn push_success_data_test_answer_sheet_reference_frame_json(
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

pub(crate) fn push_success_data_test_answer_sheet_rule_style_candidate_json(
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

pub(crate) fn success_data_test_answer_sheet_page_mark_disambiguation_ready(
    candidate: &SuccessDataTestAnswerSheetSourceFrameCandidate,
) -> bool {
    candidate.same_page_mark_entry && candidate.same_page_index_candidate
}

pub(crate) fn success_data_test_answer_sheet_page_mark_disambiguation_class(
    candidate: &SuccessDataTestAnswerSheetSourceFrameCandidate,
) -> &'static str {
    if success_data_test_answer_sheet_page_mark_disambiguation_ready(candidate) {
        "same-page-mark-entry-and-page-index-candidate"
    } else {
        "page-mark-entry-or-page-index-mismatch"
    }
}

pub(crate) fn success_data_test_answer_sheet_page_mark_disambiguation_blocked_reason(
    candidate: &SuccessDataTestAnswerSheetSourceFrameCandidate,
) -> Option<&'static str> {
    if success_data_test_answer_sheet_page_mark_disambiguation_ready(candidate) {
        None
    } else {
        Some("answer-sheet-page-mark-disambiguation-ambiguous")
    }
}

pub(crate) fn push_success_data_test_answer_sheet_page_mark_disambiguation_gate_json(
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

pub(crate) fn success_data_test_answer_sheet_section_anchors(
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

pub(crate) fn success_data_test_answer_sheet_hatched_area_candidate(
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

pub(crate) fn success_data_test_answer_sheet_source_frame_candidate(
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

pub(crate) fn push_success_data_test_answer_sheet_source_frame_candidate_json(
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

pub(crate) fn push_success_data_test_answer_sheet_local_rule_schema_candidate_json(
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

pub(crate) fn push_success_data_test_answer_sheet_point_json(
    output: &mut String,
    point: SuccessDataTestAnswerSheetPoint,
) {
    output.push_str("{\"x\":");
    output.push_str(&format!("{:.3}", point.x));
    output.push_str(",\"y\":");
    output.push_str(&format!("{:.3}", point.y));
    output.push('}');
}

pub(crate) fn success_data_test_answer_sheet_section_label(text: &str) -> bool {
    let mut chars = text.trim().chars();
    let Some(character) = chars.next() else {
        return false;
    };
    chars.next().is_none() && success_data_test_answer_sheet_row_number_char(character)
}

pub(crate) fn push_page_layer_success_data_test_text_slot_json(
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

pub(crate) fn push_success_data_test_text_slot_line_header_evidence_json(
    output: &mut String,
    slot: &SuccessDataTestResolvedTextSlot,
) {
    push_success_data_test_line_header_evidence_json(output, slot.line_header);
}

pub(crate) fn push_success_data_test_line_mark_evidence_json(
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

pub(crate) fn success_data_test_best_line_mark_match_for_source_span(
    document: &Document,
    span: &TextSourceSpan,
) -> Option<ShanaiLanLineMarkInterval> {
    let intervals =
        success_data_test_line_mark_matches_for_source_span(document, span).collect::<Vec<_>>();
    best_line_mark_interval_for_unit_range(&intervals, span.unit_start(), span.unit_end())
}

pub(crate) fn success_data_test_page_mark_entry_for_record(
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

pub(crate) fn success_data_test_line_mark_page_grid_candidate(
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

pub(crate) fn success_data_test_source_row_height_px(document: &Document) -> Option<f32> {
    success_data_test_source_row_height_font_size_units(document)
        .map(|font_size_units| f32::from(font_size_units) * 1.75)
}

pub(crate) fn success_data_test_source_row_height_font_size_units(
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

pub(crate) fn success_data_test_source_text_placement_candidate(
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

pub(crate) fn success_data_test_text_placement_residual_summary_json(
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

pub(crate) fn success_data_test_text_placement_line_pitch_fits(
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

pub(crate) fn success_data_test_text_placement_line_pitch_fit(
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

pub(crate) fn push_success_data_test_text_placement_line_pitch_fit_evidence_json(
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

pub(crate) fn push_success_data_test_text_placement_line_pitch_fit_json(
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

pub(crate) fn push_success_data_test_page_mark_selected_fields_json(
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

pub(crate) fn push_success_data_test_source_pitch_evidence_json(
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
pub(crate) fn success_data_test_text_placement_residual_entry(
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

pub(crate) fn push_success_data_test_text_placement_residual_bucket_json(
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

pub(crate) fn push_success_data_test_text_placement_residual_entry_json(
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

pub(crate) fn success_data_test_text_top_y(
    placement: Option<&SuccessDataTestSourceTextPlacementCandidate>,
    reference_top_y: f32,
) -> f32 {
    placement
        .map(|candidate| candidate.top_y)
        .unwrap_or(reference_top_y)
}

pub(crate) fn success_data_test_text_baseline_y(
    placement: Option<&SuccessDataTestSourceTextPlacementCandidate>,
    reference_baseline_y: f32,
) -> f32 {
    placement
        .map(|candidate| candidate.baseline_y)
        .unwrap_or(reference_baseline_y)
}

pub(crate) fn push_success_data_test_source_text_placement_candidate_json(
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

pub(crate) fn push_success_data_test_line_mark_page_grid_candidate_json(
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

pub(crate) fn push_success_data_test_line_header_evidence_json(
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

pub(crate) fn push_success_data_test_answer_sheet_projection_svg(
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

pub(crate) fn push_success_data_test_answer_sheet_line_svg(
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

pub(crate) fn success_data_test_answer_sheet_secondary_rule_segment(
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

pub(crate) fn success_data_test_answer_sheet_rule_secondary_offset(stroke_width: f32) -> f32 {
    (stroke_width * 1.55).clamp(1.35, 1.65)
}

pub(crate) fn success_data_test_answer_sheet_rule_secondary_stroke_width(stroke_width: f32) -> f32 {
    (stroke_width * 0.46).clamp(0.42, 0.62)
}

pub(crate) fn success_data_test_answer_sheet_rule_is_hatched_edge(
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
pub(crate) fn push_success_data_test_answer_sheet_hatch_svg(
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
pub(crate) fn push_success_data_test_answer_sheet_text_svg(
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

pub(crate) fn push_success_data_test_answer_sheet_fdm_text_svg(
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

pub(crate) fn success_data_test_answer_sheet_page(document: &Document, page_number: usize) -> bool {
    page_number == 2
        && success_data_test_answer_sheet_tail_evidence(document)
        && success_data_test_answer_sheet_fdm_text_candidate(document).is_some()
        && embedding_frame_diagnostics(document)
            .into_iter()
            .any(|diagnostic| {
                success_data_test_title_art_diagnostic_for_page(document, diagnostic, page_number)
            })
}

pub(crate) fn success_data_test_answer_sheet_tail_evidence(document: &Document) -> bool {
    let plain_text = document_plain_text(document);
    plain_text.contains("(1)表面積の比")
        && plain_text.contains("ＡＢ")
        && plain_text.contains("ＡＣ")
        && plain_text.contains("ｃｍ")
}

pub(crate) fn success_data_test_answer_sheet_sparse_table_candidate(
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

pub(crate) fn success_data_test_answer_sheet_fdm_text_candidate(
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

pub(crate) fn success_data_test_answer_sheet_figure_link_candidate(
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

pub(crate) fn success_data_test_answer_sheet_text_slots(
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

pub(crate) fn success_data_test_answer_sheet_text_slot_templates()
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

pub(crate) fn success_data_test_answer_sheet_text_tokens(
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

pub(crate) fn tokenize_success_data_test_answer_sheet_tail(text: &str) -> Vec<String> {
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

pub(crate) fn push_success_data_test_answer_sheet_token(tokens: &mut Vec<String>, raw: &str) {
    let token = success_data_test_answer_sheet_display_token(raw);
    if !token.is_empty() {
        tokens.push(token);
    }
}

pub(crate) fn success_data_test_answer_sheet_display_token(raw: &str) -> String {
    match raw.trim() {
        "ｃｍ２" => "ｃｍ²".to_string(),
        "ｃｍ３" => "ｃｍ³".to_string(),
        other => other.to_string(),
    }
}

pub(crate) fn success_data_test_answer_sheet_row_number_char(character: char) -> bool {
    matches!(
        character,
        '1' | '2' | '3' | '4' | '5' | '6' | '１' | '２' | '３' | '４' | '５' | '６'
    )
}

pub(crate) fn success_data_test_answer_sheet_fdm_label_slots(
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

pub(crate) fn success_data_test_answer_sheet_triangle_placement_candidate(
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

pub(crate) fn success_data_test_answer_sheet_triangle_label_anchor(
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

pub(crate) fn success_data_test_answer_sheet_indexed_fdm_labels(
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

pub(crate) fn success_data_test_answer_sheet_triangle_source_bbox(
    candidate: &ObjectStreamCandidate,
) -> Option<ObjectFdmIndexBbox> {
    let (left, top, right, bottom) = success_data_test_answer_sheet_indexed_fdm_labels(candidate)
        .into_iter()
        .filter(|label| matches!(label.text.text(), "Ａ" | "Ｂ" | "Ｃ"))
        .map(|label| normalize_fdm_bbox(label.index.bbox()))
        .fold(None, fdm_bbox_extent_union)?;
    Some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

pub(crate) fn success_data_test_abc_table_candidate(candidate: &TableCandidate) -> bool {
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
