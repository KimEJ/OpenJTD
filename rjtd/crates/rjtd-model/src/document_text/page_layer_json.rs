use super::*;
use crate::*;

pub(crate) fn text_style_candidates(styles: &[UnknownStyle]) -> Vec<StyleCandidate> {
    let mut candidates = Vec::new();

    for style in styles {
        if style.name() != Some(TEXT_LAYOUT_STYLE_PATH) {
            continue;
        }

        let summary = summarize_style_stream(style.payload());
        for (record_index, record) in summary.records().iter().enumerate() {
            let Some(label) = record.label() else {
                continue;
            };
            let trimmed = label.trim();
            if trimmed.is_empty() {
                continue;
            }

            candidates.push(StyleCandidate {
                id: candidates.len() as u32 + 1,
                name: trimmed.to_string(),
                source_stream: TEXT_LAYOUT_STYLE_PATH.to_string(),
                source_record_index: record_index,
                source_offset: record.offset(),
                source_code: record.code(),
                payload_len: record.payload_len(),
            });
        }
    }

    candidates
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_page_layer_text_run_json(
    output: &mut String,
    source_id: usize,
    placement: PageLayerTextPlacement,
    layout: PageLayout,
    writing_mode: WritingMode,
    font_family: &str,
    fill_color: &str,
    fragment: &PageLayerTextFragment,
) {
    let (width, height) = if writing_mode.is_vertical() {
        (
            APP_LINE_HEIGHT_PX as f64,
            vertical_text_advance_px(&fragment.text),
        )
    } else {
        (
            text_width_px(layout, &fragment.text),
            APP_LINE_HEIGHT_PX as f64,
        )
    };
    output.push_str("{\"type\":\"textRun\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{width:.3},\"height\":{height:.3}}}",
        placement.x, placement.y
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&fragment.text));
    if let Some(annotation) = &fragment.ruby_annotation {
        output.push_str(",\"rubyText\":");
        output.push_str(&json_string(annotation));
    }
    if fragment.paragraph_index.is_some() {
        output.push_str(",\"paragraphCharRange\":");
        output.push_str(&source_range_json(fragment.char_start, fragment.char_end));
    }
    output.push_str(&format!(
        ",\"baseline\":{:.3},\"rotation\":0.000,\"isVertical\":{},\"orientation\":\"{}\",\"fontFamily\":{},\"fillColor\":{},\"projectionKind\":\"fallback\",\"source\":",
        placement.baseline,
        writing_mode.is_vertical(),
        writing_mode.as_str(),
        json_string(font_family),
        json_string(fill_color)
    ));
    push_page_layer_source_span_json(output, source_id, fragment);
    output.push_str(",\"positions\":");
    push_f64_array_json(
        output,
        &text_positions_px_for_mode(layout, writing_mode, &fragment.text),
    );
    output.push_str(",\"isParaEnd\":false,\"isLineBreakEnd\":false}");
}

pub(crate) fn push_page_layer_text_source_json(
    output: &mut Vec<String>,
    source_id: usize,
    fragment: &PageLayerTextFragment,
) {
    let mut source = format!(
        "{{\"id\":{},\"text\":{},\"utf8Range\":{},\"utf16Range\":{}",
        source_id,
        json_string(&fragment.text),
        source_range_json(0, fragment.text.len()),
        source_range_json(0, fragment.text.encode_utf16().count())
    );
    if let Some(paragraph_index) = fragment.paragraph_index {
        source.push_str(",\"stableSourceKey\":");
        source.push_str(&json_string(&format!(
            "section:0/para:{paragraph_index}/char:{}",
            fragment.char_start
        )));
        source.push_str(",\"paragraphCharRange\":");
        source.push_str(&source_range_json(fragment.char_start, fragment.char_end));
    }
    if let Some(span) = &fragment.source_span {
        source.push_str(",\"jtdByteRange\":");
        source.push_str(&source_range_json(span.byte_start(), span.byte_end()));
        source.push_str(",\"jtdUnitRange\":");
        source.push_str(&source_range_json(span.unit_start(), span.unit_end()));
    }
    source.push_str(",\"annotations\":[");
    if let Some(annotation) = &fragment.ruby_annotation {
        source.push_str("{\"type\":\"ruby\",\"text\":");
        source.push_str(&json_string(annotation));
        source.push('}');
    }
    source.push_str("]}");
    output.push(source);
}

pub(crate) fn push_page_layer_observed_form_text_slot_json(
    output: &mut String,
    layout: PageLayout,
    projection: &ObservedFormTextProjection,
    slot: &ObservedFormTextSlot,
) {
    let text_width = text_width_px(layout, &slot.text) as f32 * (slot.font_size / APP_FONT_SIZE_PX);
    let x = match slot.anchor {
        "middle" => slot.x - (text_width / 2.0),
        "end" => slot.x - text_width,
        _ => slot.x,
    };
    let y = slot.y - slot.font_size;
    output.push_str("{\"type\":\"formTextProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{:.3},\"height\":{:.3}}}",
        text_width.max(slot.font_size),
        slot.font_size * 1.35
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(projection.projection_kind));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":true");
    output.push_str(",\"role\":");
    output.push_str(&json_string(slot.role));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&slot.text));
    output.push_str(",\"fontSize\":");
    output.push_str(&format!("{:.3}", slot.font_size));
    output.push_str(",\"fontWeight\":");
    output.push_str(&json_string(slot.font_weight));
    output.push_str(",\"textAnchor\":");
    output.push_str(&json_string(slot.anchor));
    output.push('}');
}

pub(crate) fn push_document_text_property_15_color_candidate_json(
    output: &mut String,
    candidate: &DocumentTextProperty15ColorCandidate,
) {
    output.push_str("{\"source\":\"/DocumentText style section\",\"propertyId\":15");
    output.push_str(",\"packedBgrHex\":");
    output.push_str(&json_string(&format!("0x{:08x}", candidate.packed_bgr)));
    output.push_str(",\"cssColor\":");
    output.push_str(&json_string(candidate.css_color));
    output.push_str(",\"sourceBacked\":true,\"colorEncodingDecoded\":true");
    output.push_str(",\"propertyRoleDecoded\":false");
    output.push_str(",\"contextGate\":\"shanai-lan-text-projection\",\"renderPromoted\":true}");
}

pub(crate) fn push_document_text_map_entry_brief_json(
    output: &mut String,
    entry: Option<&DocumentTextMapEntry>,
) {
    let Some(entry) = entry else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"kind\":");
    output.push_str(&json_string(entry.kind().as_str()));
    output.push_str(",\"byteRange\":");
    output.push_str(&source_range_json(entry.byte_start(), entry.byte_end()));
    output.push_str(",\"unitRange\":");
    output.push_str(&source_range_json(entry.unit_start(), entry.unit_end()));
    output.push_str(",\"selector\":");
    push_option_u16_json(output, entry.selector());
    output.push_str(",\"selectorHex\":");
    push_option_u16_hex_json(output, entry.selector());
    output.push_str(",\"code\":");
    push_option_u16_json(output, entry.code());
    output.push_str(",\"codeHex\":");
    push_option_u16_hex_json(output, entry.code());
    output.push_str(",\"textUnitCount\":");
    output.push_str(&entry.text().encode_utf16().count().to_string());
    output.push_str(",\"textPreview\":");
    let preview = entry.text().chars().take(16).collect::<String>();
    output.push_str(&json_string(&preview));
    output.push('}');
}

pub(crate) fn push_page_layer_layout_box_text_slot_json(
    output: &mut String,
    source_id: usize,
    projection: &LayoutBoxTextProjection,
    slot: &LayoutBoxTextSlot,
    font_family: &str,
) {
    let fragment = PageLayerTextFragment {
        text: slot.text.clone(),
        paragraph_index: None,
        char_start: 0,
        char_end: slot.text.chars().count(),
        source_span: Some(slot.source_span.clone()),
        ruby_annotation: None,
    };
    let text_width =
        text_width_px_for_font_size(slot.font_size, &slot.text).max(f64::from(slot.font_size));
    output.push_str("{\"type\":\"textRun\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        slot.x, slot.y, text_width, slot.line_height
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&slot.text));
    output.push_str(&format!(
        ",\"baseline\":{:.3},\"rotation\":0.000,\"isVertical\":false,\"orientation\":\"horizontal\",\"fontFamily\":{},\"fillColor\":\"#111111\",\"projectionKind\":{},\"source\":",
        slot.y + slot.font_size,
        json_string(font_family),
        json_string(projection.projection_kind),
    ));
    push_page_layer_source_span_json(output, source_id, &fragment);
    output.push_str(",\"positions\":");
    push_f64_array_json(
        output,
        &text_positions_px_for_font_size(slot.font_size, &slot.text),
    );
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"role\":");
    output.push_str(&json_string(slot.role));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"pageAssignmentDecoded\":");
    output.push_str(if projection.page_assignment_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockIndex\":");
    output.push_str(&slot.block_index.to_string());
    output.push_str(",\"blockCount\":");
    output.push_str(&projection.block_count.to_string());
    output.push_str(",\"layoutRecordCount\":");
    output.push_str(&projection.layout_record_count.to_string());
    output.push_str(",\"positionTablePresent\":");
    output.push_str(if projection.position_table_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"layoutRecordIndex\":");
    match slot.layout_record_index {
        Some(index) => output.push_str(&index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"layoutRecordByteRange\":");
    match slot.layout_record_byte_range {
        Some((start, end)) => output.push_str(&source_range_json(start, end)),
        None => output.push_str("null"),
    }
    output.push_str(",\"layoutFields\":{\"xPt\":");
    match slot.layout_x_pt {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"yPt\":");
    match slot.layout_y_pt {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"widthPt\":");
    match slot.layout_width_pt {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"inferredOriginPt\":");
    match slot.inferred_origin_pt {
        Some(value) => output.push_str(&format!("{value:.3}")),
        None => output.push_str("null"),
    }
    output.push('}');
    output.push_str(",\"placementBasis\":");
    output.push_str(&json_string(slot.placement_basis));
    output.push_str(",\"isParaEnd\":false,\"isLineBreakEnd\":false}");
}
