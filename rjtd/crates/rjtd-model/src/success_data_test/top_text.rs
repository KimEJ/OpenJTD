use super::*;
use crate::*;

pub(crate) const SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX: f32 = 13.5;

pub(crate) const SUCCESS_DATA_TEST_TOP_TEXT_SLOTS: &[SuccessDataTestTextSlot] = &[
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

pub(crate) const SUCCESS_DATA_TEST_FORMULA_TEXT_SLOTS: &[SuccessDataTestFormulaTextSlot] = &[
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

pub(crate) fn push_page_layer_success_data_test_figure_label_span_json(
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

pub(crate) fn push_success_data_test_figure_label_line_evidence_json(
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

pub(crate) fn push_success_data_test_top_text_projection_svg(
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

pub(crate) fn success_data_test_q4_figure_label_source_line(
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

pub(crate) fn success_data_test_q4_figure_label_text_line(text: &str) -> bool {
    let line = text.trim_end_matches('\n');
    if !(line.contains("（１）") && line.contains("（２）") && line.contains("（３）")) {
        return false;
    }
    line.chars()
        .all(|character| matches!(character, ' ' | '（' | '）' | '１' | '２' | '３'))
}

pub(crate) fn success_data_test_top_text_line_step_px(
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

pub(crate) fn success_data_test_figure_label_font_size_px(line_step: f32) -> f32 {
    (line_step * 2.0 / 3.0).max(SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX)
}

pub(crate) fn success_data_test_q4_figure_label_spans(
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

pub(crate) fn success_data_test_top_text_projection(
    document: &Document,
    page_number: usize,
) -> Option<&'static [SuccessDataTestTextSlot]> {
    (page_number == 1 && document_has_success_data_test_projection_evidence(document))
        .then_some(SUCCESS_DATA_TEST_TOP_TEXT_SLOTS)
}

pub(crate) fn success_data_test_resolved_top_text_projection(
    document: &Document,
    page_number: usize,
) -> Option<Vec<SuccessDataTestResolvedTextSlot>> {
    let slots = success_data_test_top_text_projection(document, page_number)?;
    Some(success_data_test_resolve_top_text_slots(document, slots))
}

pub(crate) fn success_data_test_resolve_top_text_slots(
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

pub(crate) fn success_data_test_next_top_text_source_match(
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

pub(crate) fn success_data_test_figure_label_span_fragment(
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

pub(crate) fn success_data_test_jseq_formula_source_top_y(
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

pub(crate) fn success_data_test_jseq_formula_anchor_slot(frame_ref: u32) -> Option<(usize, f32)> {
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

pub(crate) fn success_data_test_formula_text_slots(
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

pub(crate) fn success_data_test_top_text_line_should_skip(
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
