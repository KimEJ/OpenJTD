use super::*;
use crate::*;

pub(crate) fn push_page_layer_shanai_lan_text_slot_json(
    output: &mut String,
    source_id: usize,
    projection: &ShanaiLanTextProjection,
    slot: &ShanaiLanTextSlot,
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
        slot.x,
        slot.y,
        text_width,
        slot.font_size * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&slot.text));
    output.push_str(&format!(
        ",\"baseline\":{:.3},\"rotation\":0.000,\"isVertical\":false,\"orientation\":\"horizontal\",\"fontFamily\":{},\"fillColor\":{},\"projectionKind\":{},\"source\":",
        shanai_lan_text_baseline_y(slot),
        json_string(font_family),
        json_string(slot.fill),
        json_string(projection.projection_kind),
    ));
    push_page_layer_source_span_json(output, source_id, &fragment);
    output.push_str(",\"fillColorBasis\":");
    output.push_str(&json_string(slot.fill_basis));
    output.push_str(",\"documentTextProperty15ColorCandidate\":");
    match slot.document_text_property_15_color_candidate.as_ref() {
        Some(candidate) => push_document_text_property_15_color_candidate_json(output, candidate),
        None => output.push_str("null"),
    }
    output.push_str(",\"textStyleLinkEvidence\":");
    push_shanai_lan_text_style_link_evidence_json(output, &slot.style_link_evidence);
    output.push_str(",\"positions\":");
    push_f64_array_json(
        output,
        &text_positions_px_for_font_size(slot.font_size, &slot.text),
    );
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false");
    output.push_str(",\"textRunFragmentContext\":");
    push_shanai_lan_text_run_fragment_context_json(output, &slot.fragment_context);
    output.push_str(",\"groupIndex\":");
    match slot.group_index {
        Some(group_index) => output.push_str(&group_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineOffsetUnits\":");
    output.push_str(&slot.line_offset_units.to_string());
    output.push_str(",\"leadingUnits\":");
    output.push_str(&slot.leading_units.to_string());
    output.push_str(",\"fragmentStartUnits\":");
    output.push_str(&slot.fragment_start_units.to_string());
    output.push_str(",\"splitFromTextRun\":");
    output.push_str(if slot.split_from_text_run {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineHeaderHex\":");
    output.push_str(&json_string(&slot.line_header_hex));
    output.push_str(",\"lineHeaderRawWords\":");
    push_u16_array_json(output, &slot.line_header_raw_words);
    output.push_str(",\"lineHeaderRawWordsHex\":");
    push_u16_hex_array_json(output, &slot.line_header_raw_words);
    output.push_str(",\"lineHeaderSameSegmentGroupRun\":");
    if let Some(run) = slot.line_header_same_segment_group_run {
        push_shanai_lan_line_header_same_segment_group_run_value_json(output, run);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"lineHeaderYPlacementCandidate\":{\"source\":\"/DocumentText line-header same-offset/extent group run\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"sameSegmentGroupRunPresent\":");
    output.push_str(if slot.line_header_same_segment_group_run.is_some() {
        "true"
    } else {
        "false"
    });
    if let Some(run) = slot.line_header_same_segment_group_run {
        output.push_str(",\"startGroupIndex\":");
        output.push_str(&run.start_group_index.to_string());
        output.push_str(",\"endGroupIndex\":");
        output.push_str(&run.end_group_index.to_string());
        output.push_str(",\"groupCount\":");
        output.push_str(&run.group_count.to_string());
        output.push_str(",\"positionInRun\":");
        output.push_str(&run.position_in_run.to_string());
    }
    output.push_str(",\"sameSegmentGroupRunTextSlotCount\":");
    match slot.line_header_same_segment_group_run_text_slot_count {
        Some(count) => output.push_str(&count.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sameSegmentGroupRunDistinctTextGroupCount\":");
    match slot.line_header_same_segment_group_run_distinct_text_group_count {
        Some(count) => output.push_str(&count.to_string()),
        None => output.push_str("null"),
    }
    let same_segment_run_ambiguous_as_row_anchor = slot
        .line_header_same_segment_group_run_distinct_text_group_count
        .is_some_and(|count| count > 1);
    output.push_str(",\"sameSegmentGroupRunAmbiguousAsRowAnchor\":");
    output.push_str(if same_segment_run_ambiguous_as_row_anchor {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedDetail\":");
    output.push_str(&json_string(if same_segment_run_ambiguous_as_row_anchor {
        "same-segment-run-spans-multiple-visible-text-rows"
    } else {
        "line-header-y-run-transform-undecoded"
    }));
    output.push_str(",\"currentGroupIndex\":");
    match slot.group_index {
        Some(group_index) => output.push_str(&group_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"line-header-y-run-placement-semantics-unproven\"}",
    );
    output.push_str(",\"textCountRangeEvidenceCount\":");
    output.push_str(&slot.text_count_range_evidence.len().to_string());
    output.push_str(",\"textCountRangeEvidence\":");
    push_shanai_lan_text_count_range_evidence_json(output, &slot.text_count_range_evidence);
    output.push_str(",\"baselineFactor\":");
    output.push_str(&format!("{:.3}", SHANAI_LAN_TEXT_BASELINE_FACTOR));
    output.push_str(",\"isParaEnd\":false,\"isLineBreakEnd\":false}");
}

pub(crate) fn push_shanai_lan_text_run_fragment_context_json(
    output: &mut String,
    context: &ShanaiLanTextRunFragmentContext,
) {
    output.push_str("{\"decoded\":false,\"source\":\"/DocumentText\",\"parentTextRunSourceSpan\":");
    push_text_source_span_json(output, &context.parent_source_span);
    output.push_str(",\"parentTextUnitCount\":");
    output.push_str(&context.parent_text_unit_count.to_string());
    output.push_str(",\"fragmentIndex\":");
    output.push_str(&context.fragment_index.to_string());
    output.push_str(",\"fragmentCount\":");
    output.push_str(&context.fragment_count.to_string());
    output.push_str(",\"fragmentSourceUnitRange\":");
    output.push_str(&source_range_json(
        context.fragment_source_start_units,
        context.fragment_source_end_units,
    ));
    output.push_str(",\"previousGapUnits\":");
    push_option_usize_json(output, context.previous_gap_units);
    output.push_str(",\"nextGapUnits\":");
    push_option_usize_json(output, context.next_gap_units);
    output.push_str(",\"styleBoundaryProven\":");
    output.push_str(if context.style_boundary_proven {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"promotionBlockedReason\":");
    output.push_str(&json_string(context.promotion_blocked_reason));
    output.push('}');
}

pub(crate) fn push_page_layer_shanai_lan_text_style_evidence_summary_json(
    output: &mut String,
    layout: PageLayout,
    projection: &ShanaiLanTextProjection,
) {
    let bbox = shanai_lan_text_projection_bbox(layout, projection);
    let mut fill_basis_counts = BTreeMap::<(&'static str, &'static str), usize>::new();
    let mut view_style_group_candidate_counts = BTreeMap::<u16, usize>::new();
    let mut group_header_family_counts =
        BTreeMap::<(String, String, &'static str, &'static str), (usize, Vec<String>)>::new();
    let mut group_header_signature_counts =
        BTreeMap::<(String, &'static str, &'static str), (usize, Vec<String>)>::new();
    let mut document_view_style_group_candidate_slot_count = 0usize;
    let mut document_text_group_header_candidate_slot_count = 0usize;
    let mut document_text_inline_style_candidate_slot_count = 0usize;
    let mut source_property_fill_color_slot_count = 0usize;
    let mut fill_color_promotion_blocked_slot_count = 0usize;
    let mut split_from_text_run_slot_count = 0usize;
    let mut multi_fragment_parent_text_run_slot_count = 0usize;
    let mut max_parent_text_run_fragment_count = 0usize;
    let mut fragment_parent_run_fill_mix_counts =
        BTreeMap::<(usize, usize), ShanaiLanFragmentParentRunFillMix>::new();

    for slot in &projection.slots {
        *fill_basis_counts
            .entry((slot.fill_basis, slot.fill))
            .or_insert(0) += 1;
        if slot.split_from_text_run {
            split_from_text_run_slot_count += 1;
        }
        if slot.fragment_context.fragment_count > 1 {
            multi_fragment_parent_text_run_slot_count += 1;
            let key = (
                slot.fragment_context.parent_source_span.byte_start(),
                slot.fragment_context.parent_source_span.byte_end(),
            );
            let mix = fragment_parent_run_fill_mix_counts.entry(key).or_default();
            mix.slot_count += 1;
            if slot.fill_basis == DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS {
                mix.source_property_fill_color_slot_count += 1;
            }
            if slot.fill_basis == "default-text-fill" {
                mix.default_fill_color_slot_count += 1;
            }
            mix.fill_color_basis.insert(slot.fill_basis);
            mix.fill_colors.insert(slot.fill);
            if mix.example_texts.len() < 5 {
                mix.example_texts.push(slot.text.clone());
            }
        }
        max_parent_text_run_fragment_count =
            max_parent_text_run_fragment_count.max(slot.fragment_context.fragment_count);
        if slot.fill_basis == DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS {
            source_property_fill_color_slot_count += 1;
        }
        let evidence = &slot.style_link_evidence;
        if let Some(group_id) = evidence.document_view_style_group_candidate {
            document_view_style_group_candidate_slot_count += 1;
            *view_style_group_candidate_counts
                .entry(group_id)
                .or_insert(0) += 1;
        }
        if evidence.document_text_inline_style_candidate.is_some() {
            document_text_inline_style_candidate_slot_count += 1;
        }
        if evidence.fill_color_promotion_blocked_reason.is_some() {
            fill_color_promotion_blocked_slot_count += 1;
        }
        let Some(candidate) = evidence.document_text_group_header_candidate.as_ref() else {
            continue;
        };
        document_text_group_header_candidate_slot_count += 1;
        let control_kind = candidate
            .raw_words
            .get(1)
            .map(|value| format!("0x{value:04x}"))
            .unwrap_or_else(|| "unknown".to_string());
        let first_field = candidate
            .field_words
            .first()
            .map(|value| format!("0x{value:04x}"))
            .unwrap_or_else(|| "unknown".to_string());
        let family = group_header_family_counts
            .entry((control_kind, first_field, slot.fill_basis, slot.fill))
            .or_insert_with(|| (0, Vec::new()));
        family.0 += 1;
        if family.1.len() < 3 {
            family.1.push(slot.text.clone());
        }

        let signature = shanai_lan_u16_words_hex_attr(&candidate.raw_words);
        let signature_entry = group_header_signature_counts
            .entry((signature, slot.fill_basis, slot.fill))
            .or_insert_with(|| (0, Vec::new()));
        signature_entry.0 += 1;
        if signature_entry.1.len() < 2 {
            signature_entry.1.push(slot.text.clone());
        }
    }

    output.push_str("{\"type\":\"shanaiLanTextStyleEvidenceSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        bbox.0, bbox.1, bbox.2, bbox.3
    ));
    output.push_str(",\"source\":\"DocumentText+DocumentTextPositionTables+DocumentViewStyles\"");
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":\"shanaiLanTextStyleEvidenceSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":false");
    output.push_str(",\"slotCount\":");
    output.push_str(&projection.slots.len().to_string());
    output.push_str(",\"textStyleLinkEvidenceCount\":");
    output.push_str(&projection.slots.len().to_string());
    output.push_str(",\"documentViewStyleGroupCandidateSlotCount\":");
    output.push_str(&document_view_style_group_candidate_slot_count.to_string());
    output.push_str(",\"documentTextGroupHeaderCandidateSlotCount\":");
    output.push_str(&document_text_group_header_candidate_slot_count.to_string());
    output.push_str(",\"documentTextInlineStyleCandidateSlotCount\":");
    output.push_str(&document_text_inline_style_candidate_slot_count.to_string());
    output.push_str(",\"sourcePropertyFillColorSlotCount\":");
    output.push_str(&source_property_fill_color_slot_count.to_string());
    output.push_str(",\"fillColorPromotionBlockedSlotCount\":");
    output.push_str(&fill_color_promotion_blocked_slot_count.to_string());
    output.push_str(",\"splitFromTextRunSlotCount\":");
    output.push_str(&split_from_text_run_slot_count.to_string());
    output.push_str(",\"multiFragmentParentTextRunSlotCount\":");
    output.push_str(&multi_fragment_parent_text_run_slot_count.to_string());
    output.push_str(",\"maxParentTextRunFragmentCount\":");
    output.push_str(&max_parent_text_run_fragment_count.to_string());
    let mixed_fragment_parent_run_count = fragment_parent_run_fill_mix_counts
        .values()
        .filter(|mix| mix.fill_color_basis.len() > 1 || mix.fill_colors.len() > 1)
        .count();
    output.push_str(",\"mixedFillMultiFragmentParentRunCount\":");
    output.push_str(&mixed_fragment_parent_run_count.to_string());
    output.push_str(
        ",\"styleLinkPromotionBlockedReason\":\"document-view-style-group-link-unproven\"",
    );
    output.push_str(",\"property15ContextGeneralizationBlockedReason\":\"text-v-property-15-role-varies-outside-shanai-lan-text-runs\"");
    output.push_str(
        ",\"groupHeaderPromotionBlockedReason\":\"document-text-group-header-semantics-unproven\"",
    );
    output.push_str(",\"fillColorBasisCounts\":");
    push_shanai_lan_fill_basis_counts_json(output, &fill_basis_counts);
    output.push_str(",\"documentViewStyleGroupCandidateCounts\":");
    push_shanai_lan_view_style_group_candidate_counts_json(
        output,
        &view_style_group_candidate_counts,
    );
    output.push_str(",\"groupHeaderFamilyByFillColorBasis\":");
    push_shanai_lan_group_header_family_counts_json(output, &group_header_family_counts);
    output.push_str(",\"groupHeaderSignatureByFillColorBasis\":");
    push_shanai_lan_group_header_signature_counts_json(output, &group_header_signature_counts);
    output.push_str(",\"multiFragmentParentRunFillMixes\":");
    push_shanai_lan_fragment_parent_run_fill_mix_counts_json(
        output,
        &fragment_parent_run_fill_mix_counts,
    );
    output.push('}');
}

pub(crate) fn shanai_lan_text_projection_bbox(
    layout: PageLayout,
    projection: &ShanaiLanTextProjection,
) -> (f32, f32, f32, f32) {
    let mut left = f32::INFINITY;
    let mut top = f32::INFINITY;
    let mut right = f32::NEG_INFINITY;
    let mut bottom = f32::NEG_INFINITY;
    for slot in &projection.slots {
        let width = text_width_px_for_font_size(slot.font_size, &slot.text)
            .max(f64::from(slot.font_size)) as f32;
        let height = slot.font_size * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR;
        left = left.min(slot.x);
        top = top.min(slot.y);
        right = right.max(slot.x + width);
        bottom = bottom.max(slot.y + height);
    }
    if !left.is_finite() || !top.is_finite() || right <= left || bottom <= top {
        return (0.0, 0.0, layout.width_px(), layout.height_px());
    }
    (left, top, right - left, bottom - top)
}

pub(crate) fn push_shanai_lan_fill_basis_counts_json(
    output: &mut String,
    counts: &BTreeMap<(&'static str, &'static str), usize>,
) {
    output.push('[');
    for (index, ((basis, fill_color), count)) in counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"fillColorBasis\":");
        output.push_str(&json_string(basis));
        output.push_str(",\"fillColor\":");
        output.push_str(&json_string(fill_color));
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_shanai_lan_view_style_group_candidate_counts_json(
    output: &mut String,
    counts: &BTreeMap<u16, usize>,
) {
    output.push('[');
    for (index, (group_id, count)) in counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"documentViewStyleGroupCandidate\":");
        output.push_str(&group_id.to_string());
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_shanai_lan_group_header_signature_counts_json(
    output: &mut String,
    counts: &BTreeMap<(String, &'static str, &'static str), (usize, Vec<String>)>,
) {
    output.push('[');
    for (index, ((signature, basis, fill_color), (count, examples))) in counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rawWordsHexKey\":");
        output.push_str(&json_string(signature));
        output.push_str(",\"fillColorBasis\":");
        output.push_str(&json_string(basis));
        output.push_str(",\"fillColor\":");
        output.push_str(&json_string(fill_color));
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push_str(",\"exampleTexts\":");
        push_json_string_array(output, examples);
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_shanai_lan_fragment_parent_run_fill_mix_counts_json(
    output: &mut String,
    counts: &BTreeMap<(usize, usize), ShanaiLanFragmentParentRunFillMix>,
) {
    output.push('[');
    for (index, ((byte_start, byte_end), mix)) in counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"parentTextRunSourceSpan\":");
        push_text_source_span_json(
            output,
            &TextSourceSpan::new(*byte_start, *byte_end, byte_start / 2, byte_end / 2),
        );
        output.push_str(",\"slotCount\":");
        output.push_str(&mix.slot_count.to_string());
        output.push_str(",\"fillColorBasisCount\":");
        output.push_str(&mix.fill_color_basis.len().to_string());
        output.push_str(",\"fillColorCount\":");
        output.push_str(&mix.fill_colors.len().to_string());
        output.push_str(",\"sourcePropertyFillColorSlotCount\":");
        output.push_str(&mix.source_property_fill_color_slot_count.to_string());
        output.push_str(",\"defaultFillColorSlotCount\":");
        output.push_str(&mix.default_fill_color_slot_count.to_string());
        output.push_str(",\"fillColorBases\":");
        let bases = mix.fill_color_basis.iter().copied().collect::<Vec<_>>();
        push_json_string_slice_array(output, &bases);
        output.push_str(",\"fillColors\":");
        let colors = mix.fill_colors.iter().copied().collect::<Vec<_>>();
        push_json_string_slice_array(output, &colors);
        output.push_str(",\"styleBoundaryProven\":false");
        output.push_str(
            ",\"renderPromotionBlockedReason\":\"document-text-fragment-style-boundary-unproven\"",
        );
        output.push_str(",\"exampleTexts\":");
        push_json_string_array(output, &mix.example_texts);
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_shanai_lan_text_style_link_evidence_json(
    output: &mut String,
    evidence: &ShanaiLanTextStyleLinkEvidence,
) {
    output.push_str("{\"decoded\":false,\"source\":");
    output.push_str(&json_string(evidence.source));
    output.push_str(",\"styleLinkProven\":");
    output.push_str(if evidence.style_link_proven {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textLayoutStyleRecordCount\":");
    output.push_str(&evidence.text_layout_style_record_count.to_string());
    output.push_str(",\"documentViewStyleGroupCount\":");
    output.push_str(&evidence.document_view_style_group_count.to_string());
    output.push_str(",\"documentViewStyleGroupCandidate\":");
    match evidence.document_view_style_group_candidate {
        Some(group_id) => output.push_str(&group_id.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"documentViewStyleGroupCandidateBasis\":");
    match evidence.document_view_style_group_candidate_basis {
        Some(basis) => output.push_str(&json_string(basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"documentTextGroupHeaderCandidate\":");
    match evidence.document_text_group_header_candidate.as_ref() {
        Some(candidate) => {
            push_shanai_lan_document_text_group_header_candidate_json(output, candidate)
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"documentTextInlineStyleCandidate\":");
    match evidence.document_text_inline_style_candidate.as_ref() {
        Some(candidate) => {
            push_shanai_lan_document_text_inline_style_candidate_json(output, candidate)
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"styleLinkPromotionBlockedReason\":");
    output.push_str(&json_string(evidence.style_link_promotion_blocked_reason));
    output.push_str(",\"fillColorPromotionBlockedReason\":");
    match evidence.fill_color_promotion_blocked_reason {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(crate) fn push_shanai_lan_document_text_group_header_candidate_json(
    output: &mut String,
    candidate: &ShanaiLanDocumentTextGroupHeaderCandidate,
) {
    output.push_str("{\"decoded\":false,\"source\":\"/DocumentText\",\"sourceSpan\":");
    push_text_source_span_json(output, &candidate.source_span);
    output.push_str(",\"rawWords\":");
    push_u16_array_json(output, &candidate.raw_words);
    output.push_str(",\"rawWordsHex\":");
    push_u16_hex_array_json(output, &candidate.raw_words);
    output.push_str(",\"fieldWords\":");
    push_u16_array_json(output, &candidate.field_words);
    output.push_str(",\"fieldWordsHex\":");
    push_u16_hex_array_json(output, &candidate.field_words);
    output.push_str(",\"distanceToTextUnits\":");
    output.push_str(&candidate.distance_to_text_units.to_string());
    output.push_str(",\"styleLinkProven\":false,\"promotionBlockedReason\":");
    output.push_str(&json_string(candidate.promotion_blocked_reason));
    output.push('}');
}

pub(crate) fn push_shanai_lan_document_text_inline_style_candidate_json(
    output: &mut String,
    candidate: &ShanaiLanDocumentTextInlineStyleCandidate,
) {
    output.push_str("{\"decoded\":false,\"source\":\"/DocumentText\",\"sourceSpan\":");
    push_text_source_span_json(output, &candidate.source_span);
    output.push_str(",\"selector\":");
    match candidate.selector {
        Some(selector) => output.push_str(&selector.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"selectorHex\":");
    match candidate.selector {
        Some(selector) => output.push_str(&json_string(&format!("0x{selector:04x}"))),
        None => output.push_str("null"),
    }
    output.push_str(",\"contextWords\":");
    push_u16_array_json(output, &candidate.context_words);
    output.push_str(",\"contextWordsHex\":");
    push_u16_hex_array_json(output, &candidate.context_words);
    output.push_str(",\"payloadWords\":");
    push_u16_array_json(output, &candidate.payload_words);
    output.push_str(",\"payloadWordsHex\":");
    push_u16_hex_array_json(output, &candidate.payload_words);
    output.push_str(",\"postInlineWords\":");
    push_u16_array_json(output, &candidate.post_inline_words);
    output.push_str(",\"postInlineWordsHex\":");
    push_u16_hex_array_json(output, &candidate.post_inline_words);
    output.push_str(",\"rawWords\":");
    push_u16_array_json(output, &candidate.raw_words);
    output.push_str(",\"rawWordsHex\":");
    push_u16_hex_array_json(output, &candidate.raw_words);
    output.push_str(",\"distanceToTextUnits\":");
    output.push_str(&candidate.distance_to_text_units.to_string());
    output.push_str(",\"styleLinkProven\":false,\"promotionBlockedReason\":");
    output.push_str(&json_string(candidate.promotion_blocked_reason));
    output.push('}');
}

pub(crate) fn push_shanai_lan_text_count_range_evidence_json(
    output: &mut String,
    evidence: &[ShanaiLanTextCountRangeEvidence],
) {
    output.push('[');
    for (index, item) in evidence.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&item.index.to_string());
        output.push_str(",\"family\":");
        output.push_str(&json_string(&item.family));
        output.push_str(",\"basis\":");
        output.push_str(&json_string(item.basis.as_str()));
        output.push_str(",\"rangeStart\":");
        output.push_str(&item.range_start.to_string());
        output.push_str(",\"rangeEnd\":");
        output.push_str(&item.range_end.to_string());
        output.push_str(",\"overlapStart\":");
        output.push_str(&item.overlap_start.to_string());
        output.push_str(",\"overlapEnd\":");
        output.push_str(&item.overlap_end.to_string());
        output.push_str(",\"declaredStart\":");
        output.push_str(&item.declared_start.to_string());
        output.push_str(",\"declaredEnd\":");
        output.push_str(&item.declared_end.to_string());
        output.push_str(",\"tailFields\":");
        push_u16_array_json(output, &item.tail_fields);
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}
