use super::*;

pub(super) fn push_page_layer_shanai_lan_text_slot_json(
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

pub(super) fn push_shanai_lan_text_run_fragment_context_json(
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

pub(super) fn push_page_layer_shanai_lan_text_style_evidence_summary_json(
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

pub(super) fn shanai_lan_text_projection_bbox(
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

pub(super) fn push_shanai_lan_fill_basis_counts_json(
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

pub(super) fn push_shanai_lan_view_style_group_candidate_counts_json(
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

pub(super) fn push_shanai_lan_group_header_signature_counts_json(
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

pub(super) fn push_shanai_lan_fragment_parent_run_fill_mix_counts_json(
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

pub(super) fn push_shanai_lan_text_style_link_evidence_json(
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

pub(super) fn push_shanai_lan_document_text_group_header_candidate_json(
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

pub(super) fn push_shanai_lan_document_text_inline_style_candidate_json(
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

pub(super) fn push_shanai_lan_text_count_range_evidence_json(
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

pub(super) fn push_page_layer_shanai_lan_line_rule_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    rule_index: usize,
    rule: &ShanaiLanLineRule,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    let topology = shanai_lan_line_rule_topology(projection, rule);
    let (x, y, width, height) = shanai_lan_line_rule_bbox(projection, rule);
    let component = shanai_lan_line_rule_component_for_rule(projection, rule_index);
    output.push_str("{\"type\":\"documentTextLineRuleProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(projection.projection_kind));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"diagnosticOnly\":true,\"referenceBacked\":true");
    output.push_str(",\"ruleIndex\":");
    output.push_str(&rule_index.to_string());
    output.push_str(",\"projectionBasis\":\"documentTextLineHeaderGrid\",\"renderPromotionBlockedReason\":\"line-rule-placement-and-topology-unproven\"");
    output.push_str(",\"candidateSource\":");
    output.push_str(&json_string(rule.candidate_source));
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(projection.line_mark_profile));
    output.push_str(",\"lineMarkIntervalCount\":");
    output.push_str(&projection.line_mark_interval_count.to_string());
    output.push_str(",\"documentTextGroupCount\":");
    output.push_str(&projection.document_text_group_count.to_string());
    output.push_str(",\"documentTextLineHeaderCount\":");
    output.push_str(&projection.document_text_line_header_count.to_string());
    output.push_str(",\"skippedInlineLineHeaderCount\":");
    output.push_str(&projection.skipped_inline_line_header_count.to_string());
    output.push_str(",\"strokeColor\":\"#111111\",\"strokeWidth\":");
    output.push_str(&format!("{:.3}", projection.stroke_width));
    output.push_str(",\"orientation\":");
    output.push_str(&json_string(rule.orientation));
    output.push_str(",\"groupIndex\":");
    output.push_str(&rule.group_index.to_string());
    output.push_str(",\"endGroupIndex\":");
    output.push_str(&rule.end_group_index.to_string());
    output.push_str(",\"lineOffsetUnits\":");
    output.push_str(&rule.line_offset_units.to_string());
    output.push_str(",\"lineExtentUnits\":");
    output.push_str(&rule.line_extent_units.to_string());
    output.push_str(",\"lineHeaderHex\":");
    output.push_str(&json_string(&rule.line_header_hex));
    output.push_str(",\"lineHeaderRawWords\":");
    push_u16_array_json(output, &rule.line_header_raw_words);
    output.push_str(",\"lineHeaderRawWordsHex\":");
    push_u16_hex_array_json(output, &rule.line_header_raw_words);
    output.push_str(",\"topologyCandidate\":{\"orthogonalGraph\":");
    output.push_str(if topology.orthogonal_graph_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"startJunctionDegree\":");
    output.push_str(&topology.start_junction_degree.to_string());
    output.push_str(",\"endJunctionDegree\":");
    output.push_str(&topology.end_junction_degree.to_string());
    output.push_str(",\"isolatedEndpointCount\":");
    output.push_str(&topology.isolated_endpoint_count.to_string());
    output.push('}');
    output.push_str(",\"endpointAttachmentCandidates\":");
    push_shanai_lan_line_rule_endpoint_attachment_candidates_json(
        output,
        projection,
        rule,
        topology,
        text_projection,
    );
    output.push_str(",\"renderAdmissionGate\":");
    push_shanai_lan_line_rule_render_admission_gate_json(
        output,
        projection,
        rule_index,
        rule,
        topology,
        component.as_ref(),
        text_projection,
    );
    if let Some(line_mark) = rule.line_mark {
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&line_mark.record_index.to_string());
        output.push_str(",\"lineMarkUnitInterval\":");
        output.push_str(&source_range_json(line_mark.unit_start, line_mark.unit_end));
        output.push_str(",\"lineMarkFlagHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", line_mark.flag_word)));
    }
    output.push_str(",\"sourceByteRange\":");
    output.push_str(&source_range_json(
        rule.source_span.byte_start(),
        rule.source_span.byte_end(),
    ));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(
        rule.source_span.unit_start(),
        rule.source_span.unit_end(),
    ));
    output.push_str(",\"gridUnitPx\":");
    output.push_str(&format!("{:.3}", projection.grid_unit_px));
    output.push_str(",\"lineHeightPx\":");
    output.push_str(&format!("{:.3}", projection.line_height_px));
    output.push('}');
}

pub(super) fn push_page_layer_shanai_lan_line_rule_projection_summary_json(
    output: &mut String,
    layout: PageLayout,
    projection: &ShanaiLanLineRuleProjection,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    let mut candidate_source_counts = BTreeMap::<&'static str, usize>::new();
    let mut orientation_counts = BTreeMap::<&'static str, usize>::new();
    let component_summaries = shanai_lan_line_rule_graph_component_summaries(projection);
    let mut orthogonal_graph_candidate_count = 0usize;
    let mut no_isolated_endpoint_rule_count = 0usize;
    let mut one_isolated_endpoint_rule_count = 0usize;
    let mut two_isolated_endpoint_rule_count = 0usize;
    let mut line_mark_matched_rule_count = 0usize;
    let mut endpoint_attachment_within_line_height_count = 0usize;
    let mut both_endpoint_attachment_within_line_height_rule_count = 0usize;

    for rule in &projection.rules {
        *candidate_source_counts
            .entry(rule.candidate_source)
            .or_insert(0) += 1;
        *orientation_counts.entry(rule.orientation).or_insert(0) += 1;
        let topology = shanai_lan_line_rule_topology(projection, rule);
        if topology.orthogonal_graph_candidate {
            orthogonal_graph_candidate_count += 1;
        }
        match topology.isolated_endpoint_count {
            0 => no_isolated_endpoint_rule_count += 1,
            1 => one_isolated_endpoint_rule_count += 1,
            _ => two_isolated_endpoint_rule_count += 1,
        }
        if rule.line_mark.is_some() {
            line_mark_matched_rule_count += 1;
        }
        let start_attached = shanai_lan_line_rule_endpoint_attaches_to_text(
            rule.x1,
            rule.y1,
            projection,
            text_projection,
        );
        let end_attached = shanai_lan_line_rule_endpoint_attaches_to_text(
            rule.x2,
            rule.y2,
            projection,
            text_projection,
        );
        endpoint_attachment_within_line_height_count += usize::from(start_attached);
        endpoint_attachment_within_line_height_count += usize::from(end_attached);
        if start_attached && end_attached {
            both_endpoint_attachment_within_line_height_rule_count += 1;
        }
    }

    output.push_str("{\"type\":\"documentTextLineRuleProjectionSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"/DocumentText+/LineMark\"");
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":\"documentTextLineRuleProjectionSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"line-rule-placement-and-topology-unproven\"",
    );
    output.push_str(",\"renderPromotionBlockedDetail\":\"line-rule-endpoint-attachments-and-line-mark-row-boundaries-unproven\"");
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(projection.line_mark_profile));
    output.push_str(",\"ruleCount\":");
    output.push_str(&projection.rules.len().to_string());
    output.push_str(",\"candidateSourceCounts\":");
    push_static_str_count_map_json(output, &candidate_source_counts);
    output.push_str(",\"orientationCounts\":");
    push_static_str_count_map_json(output, &orientation_counts);
    output.push_str(",\"orthogonalGraphCandidateRuleCount\":");
    output.push_str(&orthogonal_graph_candidate_count.to_string());
    output.push_str(",\"noIsolatedEndpointRuleCount\":");
    output.push_str(&no_isolated_endpoint_rule_count.to_string());
    output.push_str(",\"oneIsolatedEndpointRuleCount\":");
    output.push_str(&one_isolated_endpoint_rule_count.to_string());
    output.push_str(",\"twoIsolatedEndpointRuleCount\":");
    output.push_str(&two_isolated_endpoint_rule_count.to_string());
    output.push_str(",\"lineMarkMatchedRuleCount\":");
    output.push_str(&line_mark_matched_rule_count.to_string());
    output.push_str(",\"lineMarkIntervalCount\":");
    output.push_str(&projection.line_mark_interval_count.to_string());
    output.push_str(",\"documentTextGroupCount\":");
    output.push_str(&projection.document_text_group_count.to_string());
    output.push_str(",\"documentTextLineHeaderCount\":");
    output.push_str(&projection.document_text_line_header_count.to_string());
    output.push_str(",\"skippedInlineLineHeaderCount\":");
    output.push_str(&projection.skipped_inline_line_header_count.to_string());
    output.push_str(",\"endpointCount\":");
    output.push_str(&(projection.rules.len() * 2).to_string());
    output.push_str(",\"endpointAttachmentWithinLineHeightCount\":");
    output.push_str(&endpoint_attachment_within_line_height_count.to_string());
    output.push_str(",\"bothEndpointAttachmentWithinLineHeightRuleCount\":");
    output.push_str(&both_endpoint_attachment_within_line_height_rule_count.to_string());
    output.push_str(",\"lineRuleRenderAdmissionGate\":");
    push_shanai_lan_line_rule_projection_render_admission_gate_json(
        output,
        projection,
        &component_summaries,
        orthogonal_graph_candidate_count,
        no_isolated_endpoint_rule_count,
        line_mark_matched_rule_count,
        both_endpoint_attachment_within_line_height_rule_count,
    );
    output.push_str(",\"lineRuleGraphComponentCount\":");
    output.push_str(&component_summaries.len().to_string());
    output.push_str(",\"largestLineRuleGraphComponentRuleCount\":");
    output.push_str(
        &component_summaries
            .iter()
            .map(|component| component.rule_indexes.len())
            .max()
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"lineRuleGraphComponents\":");
    push_shanai_lan_line_rule_graph_components_json(output, &component_summaries);
    output.push_str(",\"gridUnitPx\":");
    output.push_str(&format!("{:.3}", projection.grid_unit_px));
    output.push_str(",\"lineHeightPx\":");
    output.push_str(&format!("{:.3}", projection.line_height_px));
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!("{:.3}", projection.stroke_width));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_shanai_lan_line_header_grid_origin_authority_gate_json(
    output: &mut String,
    document: &Document,
    line_headers: &[ShanaiLanLineHeaderInGroup],
    selected_horizontal_rules: &BTreeSet<(usize, usize, u16, u16)>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    grid_origin_x: f32,
    grid_origin_y: f32,
    grid_unit_px: f32,
    line_height_px: f32,
    raw_max_extent_units: u16,
    max_extent_units: u16,
) {
    let selected_headers = line_headers
        .iter()
        .filter(|line_header| {
            selected_horizontal_rules.contains(&(
                line_header.header.start,
                line_header.group_index,
                line_header.header.offset_units,
                line_header.header.extent_units,
            ))
        })
        .collect::<Vec<_>>();
    let mut selected_group_indexes = selected_headers
        .iter()
        .map(|line_header| line_header.group_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let selected_line_mark_record_indexes = selected_headers
        .iter()
        .filter_map(|line_header| {
            shanai_lan_line_mark_for_header(line_mark_intervals, &line_header.header)
                .map(|line_mark| line_mark.record_index)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let group_record_pairs = selected_headers
        .iter()
        .filter_map(|line_header| {
            shanai_lan_line_mark_for_header(line_mark_intervals, &line_header.header)
                .map(|line_mark| (line_header.group_index, line_mark.record_index))
        })
        .collect::<Vec<_>>();
    let record_index_minus_group_index_values = group_record_pairs
        .iter()
        .map(|(group_index, record_index)| *record_index as i32 - *group_index as i32)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let uniform_record_index_minus_group_index =
        record_index_minus_group_index_values.len() == 1 && !group_record_pairs.is_empty();
    let line_mark_record_indexes_contiguous =
        usize_values_are_contiguous(&selected_line_mark_record_indexes);
    let line_mark_record_stride = uniform_usize_stride(&selected_line_mark_record_indexes);
    let selected_line_mark_intervals = selected_line_mark_record_indexes
        .iter()
        .filter_map(|record_index| {
            line_mark_intervals
                .iter()
                .find(|interval| interval.record_index == *record_index)
                .copied()
        })
        .collect::<Vec<_>>();
    let page_mark_entry = shanai_lan_page_mark_entry_covering_line_mark_records(
        document,
        &selected_line_mark_record_indexes,
    );
    let page_mark_entry_count = document
        .page_marks()
        .first()
        .map(|page_mark| page_mark.entries().len())
        .unwrap_or_default();
    let all_selected_headers_have_line_mark =
        !selected_headers.is_empty() && group_record_pairs.len() == selected_headers.len();
    let source_domain_row_anchor_candidate =
        all_selected_headers_have_line_mark && uniform_record_index_minus_group_index;
    let page_mark_entry_coverage_ready =
        page_mark_entry.is_some() && !selected_line_mark_record_indexes.is_empty();

    selected_group_indexes.sort_unstable();

    output.push_str("{\"basis\":\"selectedDocumentTextLineHeaders+/LineMark+/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"selectedLineHeaderCount\":");
    output.push_str(&selected_headers.len().to_string());
    output.push_str(",\"selectedGroupIndexes\":");
    push_usize_array_json(output, &selected_group_indexes);
    output.push_str(",\"selectedLineMarkRecordIndexes\":");
    push_usize_array_json(output, &selected_line_mark_record_indexes);
    output.push_str(",\"selectedLineMarkSourceUnitGate\":");
    push_shanai_lan_selected_line_mark_source_unit_gate_json(
        output,
        &selected_line_mark_record_indexes,
        &selected_line_mark_intervals,
    );
    output.push_str(",\"allSelectedHeadersHaveLineMark\":");
    output.push_str(&all_selected_headers_have_line_mark.to_string());
    output.push_str(",\"lineMarkRecordIndexesContiguous\":");
    output.push_str(&line_mark_record_indexes_contiguous.to_string());
    output.push_str(",\"lineMarkRecordStride\":");
    push_option_usize_json(output, line_mark_record_stride);
    output.push_str(",\"recordIndexMinusGroupIndexValues\":");
    push_i32_array_json(output, &record_index_minus_group_index_values);
    output.push_str(",\"uniformRecordIndexMinusGroupIndex\":");
    output.push_str(&uniform_record_index_minus_group_index.to_string());
    output.push_str(",\"sourceDomainRowAnchorCandidate\":");
    output.push_str(&source_domain_row_anchor_candidate.to_string());
    output.push_str(",\"pageMarkEntryCount\":");
    output.push_str(&page_mark_entry_count.to_string());
    output.push_str(",\"pageMarkEntryCoverageReady\":");
    output.push_str(&page_mark_entry_coverage_ready.to_string());
    output.push_str(",\"pageMarkEntryCoverage\":");
    if let Some(entry) = page_mark_entry {
        output.push_str("{\"rowIndex\":");
        output.push_str(&entry.row_index().to_string());
        output.push_str(",\"index\":");
        push_option_u32_json(output, entry.index());
        output.push_str(",\"flags\":");
        push_option_u32_json(output, entry.flags());
        output.push_str(",\"flagsHex\":");
        push_option_u32_hex_or_null_json(output, entry.flags());
        output.push_str(",\"lineStart\":");
        push_option_u32_json(output, entry.line_start());
        output.push_str(",\"lineEnd\":");
        push_option_u32_json(output, entry.line_end());
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pageMarkEntryProfileGate\":");
    push_shanai_lan_page_mark_entry_profile_gate_json(output, page_mark_entry);
    output.push_str(",\"sourceOnlyGridDomain\":{\"rawMaxExtentUnits\":");
    output.push_str(&raw_max_extent_units.to_string());
    output.push_str(",\"maxExtentUnits\":");
    output.push_str(&max_extent_units.to_string());
    output.push_str(",\"textGridColumnOriginDecoded\":false,\"textGridRowOriginDecoded\":false}");
    output.push_str(",\"currentProjection\":{\"originX\":");
    output.push_str(&format!("{grid_origin_x:.3}"));
    output.push_str(",\"originY\":");
    output.push_str(&format!("{grid_origin_y:.3}"));
    output.push_str(",\"gridUnitPx\":");
    output.push_str(&format!("{grid_unit_px:.3}"));
    output.push_str(",\"lineHeightPx\":");
    output.push_str(&format!("{line_height_px:.3}"));
    output.push_str(",\"referenceBacked\":true}");
    output.push_str(",\"sourceOnlyPageMarkYValueProbe\":");
    push_shanai_lan_page_mark_y_value_probe_json(output, page_mark_entry, grid_origin_y);
    output.push_str(",\"pageSpaceOriginCandidate\":null");
    output.push_str(",\"pageSpaceOriginCandidateReady\":false");
    output.push_str(",\"promotionReady\":false");
    output.push_str(",\"blockedReasons\":[");
    let mut reasons = Vec::new();
    if selected_headers.is_empty() {
        reasons.push("selected-line-header-run-missing");
    }
    if !all_selected_headers_have_line_mark {
        reasons.push("selected-line-header-line-mark-coverage-incomplete");
    }
    if !uniform_record_index_minus_group_index {
        reasons.push("line-mark-record-index-to-document-text-group-fit-not-uniform");
    }
    if !page_mark_entry_coverage_ready {
        reasons.push("page-mark-entry-coverage-missing");
    }
    reasons.push("document-text-grid-origin-reference-backed");
    reasons.push("line-header-visible-rule-selector-unproven");
    reasons.push("page-space-y-origin-unproven");
    for (index, reason) in reasons.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(reason));
    }
    output.push_str(
        "],\"renderPromotionBlockedReason\":\"line-header-grid-origin-authority-unproven\"}",
    );
}

pub(super) fn shanai_lan_page_mark_entry_covering_line_mark_records<'a>(
    document: &'a Document,
    line_mark_record_indexes: &[usize],
) -> Option<&'a DocumentPageMarkEntry> {
    let first = *line_mark_record_indexes.first()?;
    let last = *line_mark_record_indexes.last()?;
    document
        .page_marks()
        .first()?
        .entries()
        .iter()
        .find(|entry| {
            let (Some(start), Some(end)) = (entry.line_start(), entry.line_end()) else {
                return false;
            };
            start as usize <= first && last <= end as usize
        })
}

pub(super) fn push_shanai_lan_page_mark_entry_profile_gate_json(
    output: &mut String,
    page_mark_entry: Option<&DocumentPageMarkEntry>,
) {
    let profile = page_mark_entry.map(DocumentPageMarkEntry::u16_geometry_profile);
    let class_name = profile
        .as_ref()
        .map(PageMarkU16GeometryProfile::class_name)
        .unwrap_or("missing");
    let additive_geometry_profile = profile.is_some_and(|profile| {
        profile.non_zero_additive_unit_candidate() && !profile.selected_fields_all_zero()
    });
    let promotion_safe_profile = additive_geometry_profile;

    output.push_str("{\"source\":\"/PageMark u16 geometry profile\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"entryPresent\":");
    output.push_str(&page_mark_entry.is_some().to_string());
    output.push_str(",\"u16GeometryClass\":");
    output.push_str(&json_string(class_name));
    output.push_str(",\"additiveGeometryProfile\":");
    output.push_str(&additive_geometry_profile.to_string());
    output.push_str(",\"promotionSafeProfile\":");
    output.push_str(&promotion_safe_profile.to_string());
    output.push_str(",\"blockedReason\":");
    output.push_str(&json_string(if promotion_safe_profile {
        "page-mark-profile-still-needs-field-role-proof"
    } else {
        "page-mark-mixed-payload-profile-not-layout-origin-authority"
    }));
    output.push('}');
}

pub(super) fn push_shanai_lan_selected_line_mark_source_unit_gate_json(
    output: &mut String,
    selected_record_indexes: &[usize],
    intervals: &[ShanaiLanLineMarkInterval],
) {
    let interval_record_indexes = intervals
        .iter()
        .map(|interval| interval.record_index)
        .collect::<Vec<_>>();
    let unit_starts = intervals
        .iter()
        .map(|interval| interval.unit_start)
        .collect::<Vec<_>>();
    let unit_ends = intervals
        .iter()
        .map(|interval| interval.unit_end)
        .collect::<Vec<_>>();
    let unit_spans = intervals
        .iter()
        .map(|interval| interval.unit_end.saturating_sub(interval.unit_start))
        .collect::<Vec<_>>();
    let record_index_deltas = interval_record_indexes
        .windows(2)
        .map(|window| window[1].saturating_sub(window[0]))
        .collect::<Vec<_>>();
    let unit_start_deltas = unit_starts
        .windows(2)
        .map(|window| window[1].saturating_sub(window[0]))
        .collect::<Vec<_>>();
    let source_unit_delta_per_record = record_index_deltas
        .first()
        .copied()
        .zip(unit_start_deltas.first().copied())
        .and_then(|(record_delta, unit_delta)| {
            (record_delta > 0).then_some(unit_delta as f32 / record_delta as f32)
        });
    let all_selected_records_have_intervals =
        selected_record_indexes.len() == intervals.len() && !selected_record_indexes.is_empty();
    let stride_candidate_sample_count = record_index_deltas.len();
    let stride_candidate_ready =
        all_selected_records_have_intervals && stride_candidate_sample_count >= 2;

    output.push_str("{\"source\":\"/LineMark selected record source-unit intervals\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"selectedRecordCount\":");
    output.push_str(&selected_record_indexes.len().to_string());
    output.push_str(",\"intervalRecordCount\":");
    output.push_str(&intervals.len().to_string());
    output.push_str(",\"allSelectedRecordsHaveIntervals\":");
    output.push_str(&all_selected_records_have_intervals.to_string());
    output.push_str(",\"recordIndexes\":");
    push_usize_array_json(output, &interval_record_indexes);
    output.push_str(",\"unitStarts\":");
    push_usize_array_json(output, &unit_starts);
    output.push_str(",\"unitEnds\":");
    push_usize_array_json(output, &unit_ends);
    output.push_str(",\"unitSpans\":");
    push_usize_array_json(output, &unit_spans);
    output.push_str(",\"recordIndexDeltas\":");
    push_usize_array_json(output, &record_index_deltas);
    output.push_str(",\"unitStartDeltas\":");
    push_usize_array_json(output, &unit_start_deltas);
    output.push_str(",\"sourceUnitDeltaPerRecordEstimate\":");
    push_optional_f32_json(output, source_unit_delta_per_record);
    output.push_str(",\"strideCandidateSampleCount\":");
    output.push_str(&stride_candidate_sample_count.to_string());
    output.push_str(",\"strideCandidateReady\":");
    output.push_str(&stride_candidate_ready.to_string());
    output.push_str(",\"promotionReady\":false,\"blockedReason\":");
    output.push_str(&json_string(if stride_candidate_ready {
        "line-mark-source-unit-to-page-y-transform-unproven"
    } else {
        "line-mark-source-unit-stride-insufficient-selected-rows"
    }));
    output.push('}');
}

pub(super) fn push_shanai_lan_page_mark_y_value_probe_json(
    output: &mut String,
    page_mark_entry: Option<&DocumentPageMarkEntry>,
    current_projection_origin_y: f32,
) {
    let mut candidates = Vec::<PageMarkScopedYValueCandidate>::new();
    if let Some(entry) = page_mark_entry {
        collect_page_mark_entry_y_value_candidates(&mut candidates, entry);
    }
    let in_page_range_candidates = candidates
        .iter()
        .filter(|candidate| {
            (0.0..=SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX).contains(&candidate.value_px)
        })
        .collect::<Vec<_>>();
    let nearest_current_origin =
        nearest_page_mark_scoped_y_candidate(current_projection_origin_y, &candidates);

    output.push_str("{\"source\":\"/PageMark parsed entry y-value candidates\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"pageMarkEntryPresent\":");
    output.push_str(&page_mark_entry.is_some().to_string());
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidates.len().to_string());
    output.push_str(",\"inPageRangeCandidateCount\":");
    output.push_str(&in_page_range_candidates.len().to_string());
    output.push_str(",\"currentProjectionOriginY\":");
    output.push_str(&format!("{current_projection_origin_y:.3}"));
    output.push_str(",\"nearestCurrentProjectionOriginCandidate\":");
    if let Some((candidate, residual)) = nearest_current_origin {
        push_page_mark_scoped_y_candidate_json(output, candidate, residual);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"lineBoundaryConflictGate\":");
    push_shanai_lan_page_mark_y_line_boundary_conflict_json(
        output,
        page_mark_entry,
        nearest_current_origin.map(|(candidate, _)| candidate),
    );
    output.push_str(",\"candidatePreview\":");
    push_shanai_lan_page_mark_y_value_candidate_preview_json(output, &in_page_range_candidates);
    output.push_str(",\"selectionReady\":false,\"promotionReady\":false");
    output.push_str(",\"blockedReasons\":[\"page-mark-y-value-field-role-unproven\",\"document-text-grid-origin-reference-backed\",\"page-space-y-origin-unproven\"]");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"source-only-page-space-y-origin-unproven\"}",
    );
}

pub(super) fn push_shanai_lan_page_mark_y_line_boundary_conflict_json(
    output: &mut String,
    page_mark_entry: Option<&DocumentPageMarkEntry>,
    nearest_candidate: Option<&PageMarkScopedYValueCandidate>,
) {
    let line_start = page_mark_entry.and_then(DocumentPageMarkEntry::line_start);
    let line_end = page_mark_entry.and_then(DocumentPageMarkEntry::line_end);
    let nearest_value = nearest_candidate.map(|candidate| candidate.value);
    let matches_line_start = matches!(
        (nearest_value, line_start),
        (Some(candidate_value), Some(line_start)) if candidate_value == line_start
    );
    let matches_line_end = matches!(
        (nearest_value, line_end),
        (Some(candidate_value), Some(line_end)) if candidate_value == line_end
    );
    let line_boundary_conflict = matches_line_start || matches_line_end;

    output.push_str(
        "{\"source\":\"/PageMark parsed entry lineStart/lineEnd vs nearest y candidate\"",
    );
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"lineStart\":");
    push_option_u32_json(output, line_start);
    output.push_str(",\"lineEnd\":");
    push_option_u32_json(output, line_end);
    output.push_str(",\"nearestCandidateValue\":");
    push_option_u32_json(output, nearest_value);
    output.push_str(",\"matchesLineStart\":");
    output.push_str(&matches_line_start.to_string());
    output.push_str(",\"matchesLineEnd\":");
    output.push_str(&matches_line_end.to_string());
    output.push_str(",\"matchedBoundaryRoles\":[");
    let mut first = true;
    if matches_line_start {
        output.push_str("\"lineStart\"");
        first = false;
    }
    if matches_line_end {
        if !first {
            output.push(',');
        }
        output.push_str("\"lineEnd\"");
    }
    output.push_str("],\"lineBoundaryConflict\":");
    output.push_str(&line_boundary_conflict.to_string());
    output.push_str(",\"selectionReady\":false,\"promotionReady\":false");
    output.push_str(",\"blockedReason\":");
    output.push_str(&json_string(if line_boundary_conflict {
        "nearest-page-mark-y-candidate-overlaps-line-boundary"
    } else {
        "page-mark-y-value-field-role-unproven"
    }));
    output.push('}');
}

pub(super) fn push_shanai_lan_page_mark_y_value_candidate_preview_json(
    output: &mut String,
    candidates: &[&PageMarkScopedYValueCandidate],
) {
    output.push('[');
    for (index, candidate) in candidates.iter().take(12).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_candidate_json(output, candidate, 0.0);
    }
    output.push(']');
}

pub(super) fn push_page_layer_shanai_lan_line_header_projection_candidate_summary_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    bytes: &[u8],
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    selected_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let map = map_document_text(bytes);
    let skipped_inline_spans = map
        .entries()
        .iter()
        .filter(|entry| entry.kind() == DocumentTextMapKind::SkippedInlineText)
        .map(|entry| (entry.byte_start(), entry.byte_end()))
        .collect::<Vec<_>>();
    let group_offsets = shanai_lan_text_group_offsets(bytes);
    let line_headers = shanai_lan_line_headers_in_groups(bytes, &group_offsets);
    let raw_max_extent_units = shanai_lan_text_max_extent_units(bytes).unwrap_or(0x0118);
    let max_extent_units = raw_max_extent_units
        .saturating_sub(SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS)
        .max(1);
    let viewport = fdm_projection_viewport(layout);
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let fallback_grid_unit_px = viewport.width / f32::from(max_extent_units);
    let fallback_line_height_px = 12.0 * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR * scale_y;
    let fallback_stroke_width = SHANAI_LAN_LINE_RULE_STROKE_WIDTH_PX * scale_y;
    let (grid_origin_x, grid_origin_y, grid_unit_px, line_height_px, stroke_width) =
        selected_projection
            .and_then(|projection| {
                projection
                    .rules
                    .iter()
                    .find(|rule| rule.orientation == "horizontal")
                    .map(|rule| {
                        (
                            rule.x1 - f32::from(rule.line_offset_units) * projection.grid_unit_px,
                            rule.y1 - (rule.group_index as f32 + 1.0) * projection.line_height_px,
                            projection.grid_unit_px,
                            projection.line_height_px,
                            projection.stroke_width,
                        )
                    })
            })
            .unwrap_or((
                viewport.x,
                viewport.y,
                fallback_grid_unit_px,
                fallback_line_height_px,
                fallback_stroke_width,
            ));
    let selected_horizontal_rules = selected_projection
        .map(|projection| {
            projection
                .rules
                .iter()
                .filter(|rule| rule.orientation == "horizontal")
                .map(|rule| {
                    (
                        rule.source_span.byte_start(),
                        rule.group_index,
                        rule.line_offset_units,
                        rule.line_extent_units,
                    )
                })
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let mut all_line_header_count = 0usize;
    let mut long_line_header_count = 0usize;
    let mut skipped_inline_long_line_header_count = 0usize;
    let mut selected_skipped_inline_long_line_header_count = 0usize;
    let mut group_counts = BTreeMap::<usize, usize>::new();

    output.push_str("{\"type\":\"documentTextLineHeaderProjectionCandidateSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"/DocumentText\"");
    output.push_str(",\"sourceStream\":\"/DocumentText\"");
    output.push_str(",\"projectionKind\":\"documentTextLineHeaderProjectionCandidateSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"line-header-visible-rule-selector-unproven\"",
    );
    output.push_str(
        ",\"selectorBasis\":\"current-horizontal-rule-promotion-requires-skipped-inline-text\"",
    );
    output.push_str(",\"fullSpanRenderPromotionGate\":{\"basis\":\"documentTextLineHeaderCandidate+sourceMapContext\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"renderPromotionBlockedReason\":\"line-header-segment-clipping-and-endpoint-ownership-unproven\"");
    output.push_str(",\"requiresSegmentClippingDecoded\":true,\"requiresEndpointOwnershipDecoded\":true,\"requiresPaintOrderDecoded\":true");
    output.push_str(",\"fullSpanRenderableCandidateCount\":0}");
    output.push_str(",\"gridOriginAuthorityGate\":");
    push_shanai_lan_line_header_grid_origin_authority_gate_json(
        output,
        document,
        &line_headers,
        &selected_horizontal_rules,
        line_mark_intervals,
        grid_origin_x,
        grid_origin_y,
        grid_unit_px,
        line_height_px,
        raw_max_extent_units,
        max_extent_units,
    );
    output.push_str(",\"gridUnitPx\":");
    output.push_str(&format!("{grid_unit_px:.3}"));
    output.push_str(",\"lineHeightPx\":");
    output.push_str(&format!("{line_height_px:.3}"));
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!("{stroke_width:.3}"));
    output.push_str(",\"minSegmentUnits\":");
    output.push_str(&SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS.to_string());
    output.push_str(",\"rawMaxExtentUnits\":");
    output.push_str(&raw_max_extent_units.to_string());
    output.push_str(",\"maxExtentUnits\":");
    output.push_str(&max_extent_units.to_string());
    output.push_str(",\"candidates\":[");

    let mut emitted = 0usize;
    for line_header in &line_headers {
        all_line_header_count += 1;
        let header = line_header.header;
        if header.extent_units <= header.offset_units {
            continue;
        }
        let segment_units = header.extent_units - header.offset_units;
        if segment_units < SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS {
            continue;
        }
        long_line_header_count += 1;
        *group_counts.entry(line_header.group_index).or_default() += 1;
        let skipped_inline = skipped_inline_spans
            .iter()
            .any(|(start, end)| *start <= header.start && header.end <= *end);
        if skipped_inline {
            skipped_inline_long_line_header_count += 1;
        }
        let selected_as_horizontal_rule = selected_horizontal_rules.contains(&(
            header.start,
            line_header.group_index,
            header.offset_units,
            header.extent_units,
        ));
        if selected_as_horizontal_rule && skipped_inline {
            selected_skipped_inline_long_line_header_count += 1;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        let x = grid_origin_x + f32::from(header.offset_units) * grid_unit_px;
        let y = grid_origin_y + (line_header.group_index as f32 + 1.0) * line_height_px;
        let width = f32::from(segment_units) * grid_unit_px;
        let half_stroke = stroke_width * 0.5;
        output.push_str("{\"groupIndex\":");
        output.push_str(&line_header.group_index.to_string());
        output.push_str(",\"lineOffsetUnits\":");
        output.push_str(&header.offset_units.to_string());
        output.push_str(",\"lineExtentUnits\":");
        output.push_str(&header.extent_units.to_string());
        output.push_str(",\"segmentUnits\":");
        output.push_str(&segment_units.to_string());
        output.push_str(",\"bbox\":");
        push_bbox_tuple_json(
            output,
            (
                x - half_stroke,
                y - half_stroke,
                width + stroke_width,
                stroke_width,
            ),
        );
        output.push_str(",\"candidateSource\":");
        output.push_str(&json_string(if skipped_inline {
            "skippedInlineText"
        } else {
            "documentTextLineHeader"
        }));
        output.push_str(",\"selectedAsHorizontalRule\":");
        output.push_str(if selected_as_horizontal_rule {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"fullSpanRenderPromotionGate\":{\"basis\":\"documentTextLineHeaderCandidate+sourceMapContext\",\"sourceBacked\":true,\"decoded\":false,\"renderable\":false");
        output.push_str(",\"fullSpanCandidate\":");
        output.push_str(if selected_as_horizontal_rule {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"renderPromotionBlockedReason\":\"line-header-segment-clipping-and-endpoint-ownership-unproven\"}");
        output.push_str(",\"lineMarkRecordIndex\":");
        if let Some(line_mark) = shanai_lan_line_mark_for_header(line_mark_intervals, &header) {
            output.push_str(&line_mark.record_index.to_string());
        } else {
            output.push_str("null");
        }
        output.push_str(",\"lineMarkContext\":");
        push_shanai_lan_line_header_line_mark_context_json(output, line_mark_intervals, &header);
        output.push_str(",\"documentTextMapContext\":");
        push_shanai_lan_line_header_map_context_json(output, map.entries(), &header);
        output.push_str(",\"sameSegmentGroupRun\":");
        push_shanai_lan_line_header_same_segment_group_run_json(
            output,
            &line_headers,
            line_header.group_index,
            header.offset_units,
            header.extent_units,
        );
        output.push_str(",\"lineHeaderRawWordsHex\":");
        push_u16_hex_array_json(output, &header.raw_words);
        output.push_str(",\"sourceByteRange\":");
        output.push_str(&source_range_json(header.start, header.end));
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(header.start / 2, header.end / 2));
        output.push('}');
    }
    output.push_str("],\"allLineHeaderCount\":");
    output.push_str(&all_line_header_count.to_string());
    output.push_str(",\"longLineHeaderCandidateCount\":");
    output.push_str(&long_line_header_count.to_string());
    output.push_str(",\"skippedInlineLongLineHeaderCandidateCount\":");
    output.push_str(&skipped_inline_long_line_header_count.to_string());
    output.push_str(",\"selectedSkippedInlineLongLineHeaderCandidateCount\":");
    output.push_str(&selected_skipped_inline_long_line_header_count.to_string());
    output.push_str(",\"unselectedLongLineHeaderCandidateCount\":");
    output.push_str(
        &long_line_header_count
            .saturating_sub(selected_skipped_inline_long_line_header_count)
            .to_string(),
    );
    output.push_str(",\"candidateGroupCounts\":");
    push_usize_count_map_json(output, &group_counts);
    output.push_str(",\"sameSegmentGroupRuns\":");
    push_shanai_lan_line_header_same_segment_group_runs_json(
        output,
        map.entries(),
        &line_headers,
        &skipped_inline_spans,
        &selected_horizontal_rules,
        line_mark_intervals,
    );
    output.push('}');
}

pub(super) fn push_shanai_lan_line_header_line_mark_context_json(
    output: &mut String,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    header: &ShanaiLanLineHeader,
) {
    let Some(line_mark) = shanai_lan_line_mark_for_header(line_mark_intervals, header) else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"recordIndex\":");
    output.push_str(&line_mark.record_index.to_string());
    output.push_str(",\"unitRange\":");
    output.push_str(&source_range_json(line_mark.unit_start, line_mark.unit_end));
    output.push_str(",\"flagWord\":");
    output.push_str(&line_mark.flag_word.to_string());
    output.push_str(",\"flagWordHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", line_mark.flag_word)));
    output.push_str(",\"headerUnitOffsetFromLineMarkStart\":");
    output.push_str(
        &(header.start / 2)
            .saturating_sub(line_mark.unit_start)
            .to_string(),
    );
    output.push_str(",\"headerWithinLineMark\":");
    output.push_str(
        if line_mark.unit_start <= header.start / 2 && header.end / 2 <= line_mark.unit_end {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(super) fn push_shanai_lan_line_header_map_context_json(
    output: &mut String,
    entries: &[DocumentTextMapEntry],
    header: &ShanaiLanLineHeader,
) {
    let containing = entries
        .iter()
        .find(|entry| entry.byte_start() <= header.start && header.end <= entry.byte_end());
    let previous = entries
        .iter()
        .rev()
        .find(|entry| entry.byte_end() <= header.start);
    let next = entries
        .iter()
        .find(|entry| entry.byte_start() >= header.end);

    output.push_str("{\"containingEntry\":");
    push_document_text_map_entry_brief_json(output, containing);
    output.push_str(",\"previousEntry\":");
    push_document_text_map_entry_brief_json(output, previous);
    output.push_str(",\"nextEntry\":");
    push_document_text_map_entry_brief_json(output, next);
    output.push_str(",\"insideSkippedInlineText\":");
    output.push_str(
        if containing.is_some_and(|entry| entry.kind() == DocumentTextMapKind::SkippedInlineText) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"insideTextRun\":");
    output.push_str(
        if containing.is_some_and(|entry| entry.kind() == DocumentTextMapKind::TextRun) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"adjacentToSkippedInlineText\":");
    output.push_str(
        if previous
            .or(next)
            .is_some_and(|entry| entry.kind() == DocumentTextMapKind::SkippedInlineText)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(super) fn push_shanai_lan_line_header_same_segment_group_run_json(
    output: &mut String,
    line_headers: &[ShanaiLanLineHeaderInGroup],
    group_index: usize,
    offset_units: u16,
    extent_units: u16,
) {
    if let Some(run) = shanai_lan_line_header_same_segment_group_run(
        line_headers,
        group_index,
        offset_units,
        extent_units,
    ) {
        push_shanai_lan_line_header_same_segment_group_run_value_json(output, run);
    } else {
        output.push_str("null");
    }
}

pub(super) fn shanai_lan_line_header_same_segment_group_run(
    line_headers: &[ShanaiLanLineHeaderInGroup],
    group_index: usize,
    offset_units: u16,
    extent_units: u16,
) -> Option<ShanaiLanLineHeaderSameSegmentGroupRun> {
    let groups = line_headers
        .iter()
        .filter(|line_header| {
            line_header.header.offset_units == offset_units
                && line_header.header.extent_units == extent_units
        })
        .map(|line_header| line_header.group_index)
        .collect::<BTreeSet<_>>();
    if !groups.contains(&group_index) {
        return None;
    }
    let mut start_group = group_index;
    while start_group > 0 && groups.contains(&(start_group - 1)) {
        start_group -= 1;
    }
    let mut end_group = group_index;
    while groups.contains(&(end_group + 1)) {
        end_group += 1;
    }
    let group_count = end_group.saturating_sub(start_group) + 1;
    Some(ShanaiLanLineHeaderSameSegmentGroupRun {
        offset_units,
        extent_units,
        start_group_index: start_group,
        end_group_index: end_group,
        group_count,
        position_in_run: group_index.saturating_sub(start_group),
    })
}

pub(super) fn push_shanai_lan_line_header_same_segment_group_run_value_json(
    output: &mut String,
    run: ShanaiLanLineHeaderSameSegmentGroupRun,
) {
    output.push_str("{\"basis\":\"same-offset-extent-contiguous-groups\"");
    output.push_str(",\"offsetUnits\":");
    output.push_str(&run.offset_units.to_string());
    output.push_str(",\"extentUnits\":");
    output.push_str(&run.extent_units.to_string());
    output.push_str(",\"startGroupIndex\":");
    output.push_str(&run.start_group_index.to_string());
    output.push_str(",\"endGroupIndex\":");
    output.push_str(&run.end_group_index.to_string());
    output.push_str(",\"groupCount\":");
    output.push_str(&run.group_count.to_string());
    output.push_str(",\"positionInRun\":");
    output.push_str(&run.position_in_run.to_string());
    output.push('}');
}

pub(super) fn push_shanai_lan_line_header_same_segment_group_runs_json(
    output: &mut String,
    entries: &[DocumentTextMapEntry],
    line_headers: &[ShanaiLanLineHeaderInGroup],
    skipped_inline_spans: &[(usize, usize)],
    selected_horizontal_rules: &BTreeSet<(usize, usize, u16, u16)>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) {
    let mut by_segment = BTreeMap::<(u16, u16), Vec<ShanaiLanLineHeaderInGroup>>::new();
    for line_header in line_headers {
        let header = line_header.header;
        if header.extent_units <= header.offset_units {
            continue;
        }
        if header.extent_units - header.offset_units < SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS {
            continue;
        }
        by_segment
            .entry((header.offset_units, header.extent_units))
            .or_default()
            .push(*line_header);
    }

    let mut first = true;
    output.push('[');
    for ((offset_units, extent_units), mut segment_headers) in by_segment {
        segment_headers
            .sort_by_key(|line_header| (line_header.group_index, line_header.header.start));
        let mut run_start = 0usize;
        while run_start < segment_headers.len() {
            let mut run_end = run_start;
            while run_end + 1 < segment_headers.len()
                && segment_headers[run_end + 1].group_index
                    == segment_headers[run_end].group_index + 1
            {
                run_end += 1;
            }
            if !first {
                output.push(',');
            }
            first = false;
            push_shanai_lan_line_header_same_segment_group_run_summary_json(
                output,
                entries,
                &segment_headers[run_start..=run_end],
                skipped_inline_spans,
                selected_horizontal_rules,
                line_mark_intervals,
                offset_units,
                extent_units,
            );
            run_start = run_end + 1;
        }
    }
    output.push(']');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_shanai_lan_line_header_same_segment_group_run_summary_json(
    output: &mut String,
    entries: &[DocumentTextMapEntry],
    run: &[ShanaiLanLineHeaderInGroup],
    skipped_inline_spans: &[(usize, usize)],
    selected_horizontal_rules: &BTreeSet<(usize, usize, u16, u16)>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    offset_units: u16,
    extent_units: u16,
) {
    let mut selected_horizontal_count = 0usize;
    let mut skipped_inline_count = 0usize;
    let mut no_containing_map_entry_count = 0usize;
    let mut text_run_containing_count = 0usize;
    let mut containing_entry_kind_counts = BTreeMap::<&'static str, usize>::new();
    let mut line_mark_flag_counts = BTreeMap::<String, usize>::new();

    for line_header in run {
        let header = line_header.header;
        if selected_horizontal_rules.contains(&(
            header.start,
            line_header.group_index,
            header.offset_units,
            header.extent_units,
        )) {
            selected_horizontal_count += 1;
        }
        if skipped_inline_spans
            .iter()
            .any(|(start, end)| *start <= header.start && header.end <= *end)
        {
            skipped_inline_count += 1;
        }
        match entries
            .iter()
            .find(|entry| entry.byte_start() <= header.start && header.end <= entry.byte_end())
        {
            Some(entry) => {
                *containing_entry_kind_counts
                    .entry(entry.kind().as_str())
                    .or_default() += 1;
                if entry.kind() == DocumentTextMapKind::TextRun {
                    text_run_containing_count += 1;
                }
            }
            None => no_containing_map_entry_count += 1,
        }
        if let Some(line_mark) = shanai_lan_line_mark_for_header(line_mark_intervals, &header) {
            *line_mark_flag_counts
                .entry(format!("0x{:04x}", line_mark.flag_word))
                .or_default() += 1;
        }
    }

    let start_group = run
        .first()
        .map(|line_header| line_header.group_index)
        .unwrap_or_default();
    let end_group = run
        .last()
        .map(|line_header| line_header.group_index)
        .unwrap_or(start_group);

    output.push_str("{\"basis\":\"same-offset-extent-contiguous-groups\"");
    output.push_str(",\"offsetUnits\":");
    output.push_str(&offset_units.to_string());
    output.push_str(",\"extentUnits\":");
    output.push_str(&extent_units.to_string());
    output.push_str(",\"segmentUnits\":");
    output.push_str(&extent_units.saturating_sub(offset_units).to_string());
    output.push_str(",\"startGroupIndex\":");
    output.push_str(&start_group.to_string());
    output.push_str(",\"endGroupIndex\":");
    output.push_str(&end_group.to_string());
    output.push_str(",\"groupCount\":");
    output.push_str(&run.len().to_string());
    output.push_str(",\"selectedHorizontalRuleCount\":");
    output.push_str(&selected_horizontal_count.to_string());
    output.push_str(",\"skippedInlineCount\":");
    output.push_str(&skipped_inline_count.to_string());
    output.push_str(",\"noContainingMapEntryCount\":");
    output.push_str(&no_containing_map_entry_count.to_string());
    output.push_str(",\"textRunContainingCount\":");
    output.push_str(&text_run_containing_count.to_string());
    output.push_str(",\"containingEntryKindCounts\":");
    push_static_str_count_map_json(output, &containing_entry_kind_counts);
    output.push_str(",\"lineMarkFlagCounts\":");
    push_string_count_map_json(output, &line_mark_flag_counts, "flagWordHex");
    output.push_str(",\"renderable\":false,\"renderPromotionBlockedReason\":\"line-header-run-visibility-selector-unproven\"}");
}

pub(super) fn shanai_lan_line_rule_endpoint_attaches_to_text(
    x: f32,
    y: f32,
    projection: &ShanaiLanLineRuleProjection,
    text_projection: Option<&ShanaiLanTextProjection>,
) -> bool {
    shanai_lan_nearest_text_slot_attachment(text_projection, x, y)
        .is_some_and(|(_, distance_px, _)| distance_px <= projection.line_height_px)
}

pub(super) fn push_shanai_lan_line_rule_graph_components_json(
    output: &mut String,
    components: &[ShanaiLanLineRuleGraphComponentSummary],
) {
    output.push('[');
    for (component_index, component) in components.iter().enumerate() {
        if component_index > 0 {
            output.push(',');
        }
        output.push_str("{\"componentIndex\":");
        output.push_str(&component_index.to_string());
        output.push_str(",\"ruleIndexes\":");
        push_usize_array_json(output, &component.rule_indexes);
        output.push_str(",\"bbox\":");
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
            component.bbox.0, component.bbox.1, component.bbox.2, component.bbox.3
        ));
        output.push_str(",\"ruleCount\":");
        output.push_str(&component.rule_indexes.len().to_string());
        output.push_str(",\"horizontalRuleCount\":");
        output.push_str(&component.horizontal_rule_count.to_string());
        output.push_str(",\"verticalRuleCount\":");
        output.push_str(&component.vertical_rule_count.to_string());
        output.push_str(",\"orthogonalGraphRuleCount\":");
        output.push_str(&component.orthogonal_graph_rule_count.to_string());
        output.push_str(",\"lineMarkMatchedRuleCount\":");
        output.push_str(&component.line_mark_matched_rule_count.to_string());
        output.push_str(",\"endpointCount\":");
        output.push_str(&(component.rule_indexes.len() * 2).to_string());
        output.push_str(",\"isolatedEndpointCount\":");
        output.push_str(&component.isolated_endpoint_count.to_string());
        output.push_str(",\"totalProjectedLengthPx\":");
        output.push_str(&format!("{:.3}", component.total_projected_length_px));
        output.push_str(",\"orthogonalComponentCandidate\":");
        output.push_str(json_bool(
            shanai_lan_line_rule_component_orthogonal_candidate(component),
        ));
        output.push_str(",\"lineMarkCoverageComplete\":");
        output.push_str(json_bool(
            component.line_mark_matched_rule_count == component.rule_indexes.len(),
        ));
        output.push_str(",\"renderAdmissionGate\":");
        push_shanai_lan_line_rule_component_render_admission_gate_json(output, component);
        output.push_str(",\"renderable\":false,\"renderPromotionBlockedReason\":\"line-rule-component-placement-and-style-unproven\"}");
    }
    output.push(']');
}

pub(super) fn push_shanai_lan_line_rule_projection_render_admission_gate_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    components: &[ShanaiLanLineRuleGraphComponentSummary],
    orthogonal_graph_candidate_count: usize,
    no_isolated_endpoint_rule_count: usize,
    line_mark_matched_rule_count: usize,
    both_endpoint_attachment_within_line_height_rule_count: usize,
) {
    let orthogonal_component_count = components
        .iter()
        .filter(|component| shanai_lan_line_rule_component_orthogonal_candidate(component))
        .count();
    let line_mark_coverage_complete = line_mark_matched_rule_count == projection.rules.len();
    let has_endpoint_attachment_pair = both_endpoint_attachment_within_line_height_rule_count > 0;
    let mut blocked_reasons = Vec::new();
    blocked_reasons.push("document-text-grid-origin-reference-backed");
    if orthogonal_graph_candidate_count < projection.rules.len() {
        blocked_reasons.push("line-rule-topology-partial-orthogonal-coverage");
    }
    if orthogonal_component_count < components.len() {
        blocked_reasons.push("line-rule-component-topology-unproven");
    }
    if !line_mark_coverage_complete {
        blocked_reasons.push("line-rule-line-mark-coverage-incomplete");
    }
    if no_isolated_endpoint_rule_count < projection.rules.len() {
        blocked_reasons.push("line-rule-endpoint-ownership-unproven");
    }
    if !has_endpoint_attachment_pair {
        blocked_reasons.push("line-rule-text-attachment-pair-absent");
    }
    blocked_reasons.push("line-rule-style-role-unproven");
    blocked_reasons.push("line-rule-paint-order-unproven");

    output.push_str("{\"source\":\"/DocumentText+/LineMark line-rule render admission\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementDerived\":false,\"renderable\":false,\"promotionReady\":false");
    output.push_str(",\"ruleCount\":");
    output.push_str(&projection.rules.len().to_string());
    output.push_str(",\"componentCount\":");
    output.push_str(&components.len().to_string());
    output.push_str(",\"orthogonalGraphCandidateRuleCount\":");
    output.push_str(&orthogonal_graph_candidate_count.to_string());
    output.push_str(",\"orthogonalComponentCandidateCount\":");
    output.push_str(&orthogonal_component_count.to_string());
    output.push_str(",\"lineMarkCoverageComplete\":");
    output.push_str(json_bool(line_mark_coverage_complete));
    output.push_str(",\"noIsolatedEndpointRuleCount\":");
    output.push_str(&no_isolated_endpoint_rule_count.to_string());
    output.push_str(",\"bothEndpointAttachmentWithinLineHeightRuleCount\":");
    output.push_str(&both_endpoint_attachment_within_line_height_rule_count.to_string());
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionBlockedReason\":\"line-rule-render-admission-not-ready\"}");
}

pub(super) fn push_shanai_lan_line_rule_component_render_admission_gate_json(
    output: &mut String,
    component: &ShanaiLanLineRuleGraphComponentSummary,
) {
    let orthogonal_component_candidate =
        shanai_lan_line_rule_component_orthogonal_candidate(component);
    let line_mark_coverage_complete =
        component.line_mark_matched_rule_count == component.rule_indexes.len();
    let mut blocked_reasons = Vec::new();
    blocked_reasons.push("document-text-grid-origin-reference-backed");
    if !orthogonal_component_candidate {
        blocked_reasons.push("line-rule-component-topology-unproven");
    }
    if component.isolated_endpoint_count > 0 {
        blocked_reasons.push("line-rule-component-endpoint-ownership-unproven");
    }
    if !line_mark_coverage_complete {
        blocked_reasons.push("line-rule-component-line-mark-coverage-incomplete");
    }
    blocked_reasons.push("line-rule-component-style-role-unproven");
    blocked_reasons.push("line-rule-paint-order-unproven");

    output.push_str("{\"source\":\"/DocumentText+/LineMark line-rule component render admission\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementDerived\":false,\"renderable\":false,\"promotionReady\":false");
    output.push_str(",\"orthogonalComponentCandidate\":");
    output.push_str(json_bool(orthogonal_component_candidate));
    output.push_str(",\"lineMarkCoverageComplete\":");
    output.push_str(json_bool(line_mark_coverage_complete));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"line-rule-component-render-admission-not-ready\"}",
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_shanai_lan_line_rule_render_admission_gate_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    rule_index: usize,
    rule: &ShanaiLanLineRule,
    topology: ShanaiLanLineRuleTopology,
    component: Option<&(usize, ShanaiLanLineRuleGraphComponentSummary)>,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    let start_attached = shanai_lan_line_rule_endpoint_attaches_to_text(
        rule.x1,
        rule.y1,
        projection,
        text_projection,
    );
    let end_attached = shanai_lan_line_rule_endpoint_attaches_to_text(
        rule.x2,
        rule.y2,
        projection,
        text_projection,
    );
    let component_candidate = component
        .map(|(_, component)| shanai_lan_line_rule_component_orthogonal_candidate(component))
        .unwrap_or(false);
    let component_index = component.map(|(component_index, _)| *component_index);
    let component_rule_count = component.map(|(_, component)| component.rule_indexes.len());
    let has_line_mark = rule.line_mark.is_some();
    let mut blocked_reasons = Vec::new();
    blocked_reasons.push("document-text-grid-origin-reference-backed");
    if !topology.orthogonal_graph_candidate {
        blocked_reasons.push("line-rule-topology-not-orthogonal-network");
    }
    if topology.isolated_endpoint_count > 0 {
        blocked_reasons.push("line-rule-endpoint-ownership-unproven");
    }
    if !start_attached || !end_attached {
        blocked_reasons.push("line-rule-text-attachment-pair-unproven");
    }
    if !has_line_mark {
        blocked_reasons.push("line-rule-line-mark-record-missing");
    }
    if !component_candidate {
        blocked_reasons.push("line-rule-component-topology-unproven");
    }
    blocked_reasons.push("line-rule-style-role-unproven");
    blocked_reasons.push("line-rule-paint-order-unproven");

    output.push_str("{\"source\":\"/DocumentText+/LineMark line-rule render admission\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementDerived\":false,\"renderable\":false,\"promotionReady\":false");
    output.push_str(",\"ruleIndex\":");
    output.push_str(&rule_index.to_string());
    output.push_str(",\"componentIndex\":");
    push_option_usize_json(output, component_index);
    output.push_str(",\"componentRuleCount\":");
    push_option_usize_json(output, component_rule_count);
    output.push_str(",\"lineMarkMatched\":");
    output.push_str(json_bool(has_line_mark));
    output.push_str(",\"orthogonalGraphCandidate\":");
    output.push_str(json_bool(topology.orthogonal_graph_candidate));
    output.push_str(",\"componentOrthogonalCandidate\":");
    output.push_str(json_bool(component_candidate));
    output.push_str(",\"startEndpointTextAttachmentCandidate\":");
    output.push_str(json_bool(start_attached));
    output.push_str(",\"endEndpointTextAttachmentCandidate\":");
    output.push_str(json_bool(end_attached));
    output.push_str(",\"bothEndpointTextAttachmentCandidate\":");
    output.push_str(json_bool(start_attached && end_attached));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionBlockedReason\":\"line-rule-render-admission-not-ready\"}");
}

pub(super) fn shanai_lan_line_rule_component_for_rule(
    projection: &ShanaiLanLineRuleProjection,
    rule_index: usize,
) -> Option<(usize, ShanaiLanLineRuleGraphComponentSummary)> {
    shanai_lan_line_rule_graph_component_summaries(projection)
        .into_iter()
        .enumerate()
        .find(|(_, component)| component.rule_indexes.contains(&rule_index))
}

pub(super) fn shanai_lan_line_rule_component_orthogonal_candidate(
    component: &ShanaiLanLineRuleGraphComponentSummary,
) -> bool {
    !component.rule_indexes.is_empty()
        && component.horizontal_rule_count > 0
        && component.vertical_rule_count > 0
        && component.orthogonal_graph_rule_count == component.rule_indexes.len()
        && component.line_mark_matched_rule_count == component.rule_indexes.len()
}

pub(super) fn shanai_lan_line_rule_topology(
    projection: &ShanaiLanLineRuleProjection,
    rule: &ShanaiLanLineRule,
) -> ShanaiLanLineRuleTopology {
    let start_junction_degree =
        shanai_lan_line_rule_junction_degree(&projection.rules, rule.x1, rule.y1);
    let end_junction_degree =
        shanai_lan_line_rule_junction_degree(&projection.rules, rule.x2, rule.y2);
    let isolated_endpoint_count =
        usize::from(start_junction_degree <= 1) + usize::from(end_junction_degree <= 1);
    ShanaiLanLineRuleTopology {
        start_junction_degree,
        end_junction_degree,
        isolated_endpoint_count,
        orthogonal_graph_candidate: matches!(rule.orientation, "horizontal" | "vertical")
            && isolated_endpoint_count < 2,
    }
}

pub(super) fn shanai_lan_line_rule_graph_component_summaries(
    projection: &ShanaiLanLineRuleProjection,
) -> Vec<ShanaiLanLineRuleGraphComponentSummary> {
    let mut adjacency = vec![Vec::<usize>::new(); projection.rules.len()];
    for left_index in 0..projection.rules.len() {
        for right_index in (left_index + 1)..projection.rules.len() {
            if shanai_lan_line_rules_touch(
                &projection.rules[left_index],
                &projection.rules[right_index],
            ) {
                adjacency[left_index].push(right_index);
                adjacency[right_index].push(left_index);
            }
        }
    }

    let mut seen = vec![false; projection.rules.len()];
    let mut components = Vec::new();
    for start_index in 0..projection.rules.len() {
        if seen[start_index] {
            continue;
        }
        let mut stack = vec![start_index];
        seen[start_index] = true;
        let mut rule_indexes = Vec::new();
        while let Some(index) = stack.pop() {
            rule_indexes.push(index);
            for neighbor in adjacency[index].iter().copied() {
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
        rule_indexes.sort_unstable();
        components.push(shanai_lan_line_rule_graph_component_summary(
            projection,
            rule_indexes,
        ));
    }
    components
}

pub(super) fn shanai_lan_line_rule_graph_component_summary(
    projection: &ShanaiLanLineRuleProjection,
    rule_indexes: Vec<usize>,
) -> ShanaiLanLineRuleGraphComponentSummary {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut horizontal_rule_count = 0usize;
    let mut vertical_rule_count = 0usize;
    let mut orthogonal_graph_rule_count = 0usize;
    let mut line_mark_matched_rule_count = 0usize;
    let mut isolated_endpoint_count = 0usize;
    let mut total_projected_length_px = 0.0f32;

    for rule_index in rule_indexes.iter().copied() {
        let rule = &projection.rules[rule_index];
        let (x, y, width, height) = shanai_lan_line_rule_bbox(projection, rule);
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
        match rule.orientation {
            "horizontal" => horizontal_rule_count += 1,
            "vertical" => vertical_rule_count += 1,
            _ => {}
        }
        if rule.line_mark.is_some() {
            line_mark_matched_rule_count += 1;
        }
        let topology = shanai_lan_line_rule_topology(projection, rule);
        if topology.orthogonal_graph_candidate {
            orthogonal_graph_rule_count += 1;
        }
        isolated_endpoint_count += topology.isolated_endpoint_count;
        total_projected_length_px += (rule.x2 - rule.x1).abs() + (rule.y2 - rule.y1).abs();
    }

    ShanaiLanLineRuleGraphComponentSummary {
        rule_indexes,
        bbox: (min_x, min_y, max_x - min_x, max_y - min_y),
        horizontal_rule_count,
        vertical_rule_count,
        orthogonal_graph_rule_count,
        line_mark_matched_rule_count,
        isolated_endpoint_count,
        total_projected_length_px,
    }
}

pub(super) fn shanai_lan_line_rules_touch(
    left: &ShanaiLanLineRule,
    right: &ShanaiLanLineRule,
) -> bool {
    let (left_start, left_end) = shanai_lan_line_rule_endpoints(left);
    let (right_start, right_end) = shanai_lan_line_rule_endpoints(right);
    shanai_lan_line_rule_contains_point(right, left_start.0, left_start.1)
        || shanai_lan_line_rule_contains_point(right, left_end.0, left_end.1)
        || shanai_lan_line_rule_contains_point(left, right_start.0, right_start.1)
        || shanai_lan_line_rule_contains_point(left, right_end.0, right_end.1)
}

pub(super) fn shanai_lan_line_rule_endpoints(rule: &ShanaiLanLineRule) -> ((f32, f32), (f32, f32)) {
    ((rule.x1, rule.y1), (rule.x2, rule.y2))
}

pub(super) fn shanai_lan_line_rule_bbox(
    projection: &ShanaiLanLineRuleProjection,
    rule: &ShanaiLanLineRule,
) -> (f32, f32, f32, f32) {
    let x = rule.x1.min(rule.x2) - projection.stroke_width * 0.5;
    let y = rule.y1.min(rule.y2) - projection.stroke_width * 0.5;
    let width = (rule.x2 - rule.x1).abs() + projection.stroke_width;
    let height = (rule.y2 - rule.y1).abs() + projection.stroke_width;
    (x, y, width, height)
}

pub(super) fn shanai_lan_line_rule_junction_degree(
    rules: &[ShanaiLanLineRule],
    x: f32,
    y: f32,
) -> usize {
    rules
        .iter()
        .filter(|rule| shanai_lan_line_rule_contains_point(rule, x, y))
        .count()
}

pub(super) fn shanai_lan_line_rule_contains_point(
    rule: &ShanaiLanLineRule,
    x: f32,
    y: f32,
) -> bool {
    const EPSILON: f32 = 0.75;
    let min_x = rule.x1.min(rule.x2) - EPSILON;
    let max_x = rule.x1.max(rule.x2) + EPSILON;
    let min_y = rule.y1.min(rule.y2) - EPSILON;
    let max_y = rule.y1.max(rule.y2) + EPSILON;
    if !(min_x..=max_x).contains(&x) || !(min_y..=max_y).contains(&y) {
        return false;
    }
    match rule.orientation {
        "horizontal" => (rule.y1 - y).abs() <= EPSILON,
        "vertical" => (rule.x1 - x).abs() <= EPSILON,
        _ => false,
    }
}

pub(super) fn push_shanai_lan_line_rule_endpoint_attachment_candidates_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    rule: &ShanaiLanLineRule,
    topology: ShanaiLanLineRuleTopology,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    output.push_str("{\"start\":");
    push_shanai_lan_line_rule_endpoint_attachment_candidate_json(
        output,
        projection,
        rule.x1,
        rule.y1,
        topology.start_junction_degree,
        text_projection,
    );
    output.push_str(",\"end\":");
    push_shanai_lan_line_rule_endpoint_attachment_candidate_json(
        output,
        projection,
        rule.x2,
        rule.y2,
        topology.end_junction_degree,
        text_projection,
    );
    output.push('}');
}

pub(super) fn push_shanai_lan_line_rule_endpoint_attachment_candidate_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    x: f32,
    y: f32,
    junction_degree: usize,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    output.push_str("{\"point\":");
    output.push_str(&format!("{{\"x\":{x:.3},\"y\":{y:.3}}}"));
    output.push_str(",\"junctionDegree\":");
    output.push_str(&junction_degree.to_string());
    output.push_str(",\"attachmentProven\":false,\"nearestTextSlot\":");
    if let Some((slot, distance_px, bbox)) =
        shanai_lan_nearest_text_slot_attachment(text_projection, x, y)
    {
        output.push_str("{\"text\":");
        output.push_str(&json_string(&slot.text));
        output.push_str(",\"distancePx\":");
        output.push_str(&format!("{distance_px:.3}"));
        output.push_str(",\"probeRadiusPx\":");
        output.push_str(&format!("{:.3}", projection.line_height_px));
        output.push_str(",\"withinLineHeight\":");
        output.push_str(if distance_px <= projection.line_height_px {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"bbox\":");
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
            bbox.0, bbox.1, bbox.2, bbox.3
        ));
        output.push_str(",\"groupIndex\":");
        match slot.group_index {
            Some(group_index) => output.push_str(&group_index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"sourceByteRange\":");
        output.push_str(&source_range_json(
            slot.source_span.byte_start(),
            slot.source_span.byte_end(),
        ));
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(
            slot.source_span.unit_start(),
            slot.source_span.unit_end(),
        ));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn shanai_lan_nearest_text_slot_attachment<'a>(
    text_projection: Option<&'a ShanaiLanTextProjection>,
    x: f32,
    y: f32,
) -> Option<ShanaiLanTextSlotAttachment<'a>> {
    text_projection?
        .slots
        .iter()
        .map(|slot| {
            let bbox = shanai_lan_text_slot_bbox(slot);
            let distance_px = distance_from_point_to_bbox(x, y, bbox);
            (slot, distance_px, bbox)
        })
        .min_by(|left, right| left.1.partial_cmp(&right.1).unwrap_or(Ordering::Equal))
}

pub(super) fn shanai_lan_text_slot_bbox(slot: &ShanaiLanTextSlot) -> (f32, f32, f32, f32) {
    let text_width =
        text_width_px_for_font_size(slot.font_size, &slot.text).max(f64::from(slot.font_size));
    (
        slot.x,
        slot.y,
        text_width as f32,
        slot.font_size * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR,
    )
}

pub(super) fn shanai_lan_text_style_link_evidence(
    document: &Document,
    document_text_bytes: &[u8],
    text_entry: &DocumentTextMapEntry,
    text_count_range_evidence: &[ShanaiLanTextCountRangeEvidence],
) -> ShanaiLanTextStyleLinkEvidence {
    let text_layout_style_record_count =
        style_stream_record_count(document, TEXT_LAYOUT_STYLE_PATH);
    let document_view_style_group_count = document_view_style_group_count(document);
    let document_view_style_group_candidate =
        shanai_lan_document_view_style_group_candidate(text_count_range_evidence);
    let document_view_style_group_candidate_basis =
        document_view_style_group_candidate.map(|_| "document-text-position-count-tail-field-f7");
    let document_text_group_header_candidate =
        shanai_lan_document_text_group_header_candidate(document_text_bytes, text_entry);
    let document_text_inline_style_candidate =
        shanai_lan_document_text_inline_style_candidate(document_text_bytes, text_entry);
    ShanaiLanTextStyleLinkEvidence {
        source: "DocumentText+DocumentTextPositionTables+DocumentViewStyles",
        style_link_proven: false,
        text_layout_style_record_count,
        document_view_style_group_count,
        document_view_style_group_candidate,
        document_view_style_group_candidate_basis,
        document_text_group_header_candidate,
        document_text_inline_style_candidate,
        style_link_promotion_blocked_reason: "document-view-style-group-link-unproven",
        fill_color_promotion_blocked_reason: None,
    }
}

pub(super) fn shanai_lan_document_text_group_header_candidate(
    bytes: &[u8],
    entry: &DocumentTextMapEntry,
) -> Option<ShanaiLanDocumentTextGroupHeaderCandidate> {
    let units = document_text_units(bytes);
    let header_start = (0..entry.unit_start()).rev().find(|index| {
        units.get(*index).copied() == Some(0x001c) && units.get(index + 1).copied() == Some(0x0010)
    })?;
    let text_marker_index = (header_start + 2..entry.unit_start())
        .find(|index| units.get(*index).copied() == Some(DOCUMENT_TEXT_TEXT_RUN_MARKER))?;
    let raw_words = units.get(header_start..=text_marker_index)?.to_vec();
    if raw_words.len() < 3 || raw_words.len() > 64 {
        return None;
    }
    let field_words = raw_words
        .get(2..raw_words.len().saturating_sub(1))
        .unwrap_or_default()
        .to_vec();
    let source_span = TextSourceSpan::new(
        header_start * 2,
        (text_marker_index + 1) * 2,
        header_start,
        text_marker_index + 1,
    );
    Some(ShanaiLanDocumentTextGroupHeaderCandidate {
        source_span,
        raw_words,
        field_words,
        distance_to_text_units: entry.unit_start().saturating_sub(text_marker_index + 1),
        promotion_blocked_reason: "document-text-group-header-semantics-unproven",
    })
}

pub(super) fn shanai_lan_document_text_inline_style_candidate(
    bytes: &[u8],
    entry: &DocumentTextMapEntry,
) -> Option<ShanaiLanDocumentTextInlineStyleCandidate> {
    let units = document_text_units(bytes);
    let text_marker_index = entry.unit_start().checked_sub(1)?;
    if units.get(text_marker_index).copied() != Some(DOCUMENT_TEXT_TEXT_RUN_MARKER) {
        return None;
    }

    let search_start = text_marker_index.saturating_sub(32);
    let inline_end = (search_start..text_marker_index)
        .rev()
        .find(|index| units.get(*index).copied() == Some(0x001e))?;
    let inline_start = (search_start..inline_end)
        .rev()
        .find(|index| units.get(*index).copied() == Some(DOCUMENT_TEXT_INLINE_START_TAG as u16))?;
    let context_start = inline_start.checked_sub(6)?;
    let context_words = units.get(context_start..inline_start)?.to_vec();
    if !shanai_lan_document_text_inline_style_context(&context_words) {
        return None;
    }
    let selector = context_words.get(5).copied();
    let payload_words = units.get(inline_start + 1..inline_end)?.to_vec();
    let post_inline_words = units.get(inline_end + 1..=text_marker_index)?.to_vec();
    let raw_words = units.get(context_start..=text_marker_index)?.to_vec();
    let source_span = TextSourceSpan::new(
        context_start * 2,
        (text_marker_index + 1) * 2,
        context_start,
        text_marker_index + 1,
    );
    let distance_to_text_units = entry.unit_start().saturating_sub(inline_end + 1);

    Some(ShanaiLanDocumentTextInlineStyleCandidate {
        source_span,
        selector,
        context_words,
        payload_words,
        post_inline_words,
        raw_words,
        distance_to_text_units,
        promotion_blocked_reason: "document-text-inline-control-semantics-unproven",
    })
}

pub(super) fn shanai_lan_document_text_inline_style_context(context_words: &[u16]) -> bool {
    context_words.len() == 6
        && context_words[0] == 0x001c
        && context_words[1] == 0x0001
        && context_words[2] == 0x0007
        && context_words[3] == 0x0000
}

pub(super) fn shanai_lan_document_view_style_group_candidate(
    evidence: &[ShanaiLanTextCountRangeEvidence],
) -> Option<u16> {
    let mut candidate = None;
    for item in evidence {
        let Some(group_id) = item.tail_fields.get(7).copied() else {
            continue;
        };
        if !(1..=9).contains(&group_id) {
            continue;
        }
        match candidate {
            Some(existing) if existing != group_id => return None,
            Some(_) => {}
            None => candidate = Some(group_id),
        }
    }
    candidate
}

pub(super) fn shanai_lan_text_count_range_evidence(
    document: &Document,
    span: &TextSourceSpan,
) -> Vec<ShanaiLanTextCountRangeEvidence> {
    let mut evidence = Vec::new();
    for range in document.text_count_ranges() {
        push_shanai_lan_text_count_range_evidence(
            &mut evidence,
            range,
            TextCountRangeOverlapBasis::Byte,
            span.byte_start(),
            span.byte_end(),
        );
        push_shanai_lan_text_count_range_evidence(
            &mut evidence,
            range,
            TextCountRangeOverlapBasis::Unit,
            span.unit_start(),
            span.unit_end(),
        );
    }
    evidence
}

pub(super) fn push_shanai_lan_text_count_range_evidence(
    evidence: &mut Vec<ShanaiLanTextCountRangeEvidence>,
    range: &TextCountRange,
    basis: TextCountRangeOverlapBasis,
    span_start: usize,
    span_end: usize,
) {
    let range_start = range.start() as usize;
    let range_end = range.end() as usize;
    let overlap_start = span_start.max(range_start);
    let overlap_end = span_end.min(range_end);
    if overlap_start >= overlap_end {
        return;
    }
    evidence.push(ShanaiLanTextCountRangeEvidence {
        index: range.index(),
        family: range.family().to_string(),
        basis,
        range_start,
        range_end,
        overlap_start,
        overlap_end,
        declared_start: range.declared_start(),
        declared_end: range.declared_end(),
        tail_fields: range.tail_fields().to_vec(),
    });
}

pub(super) fn shanai_lan_document_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ShanaiLanTextProjection> {
    if page_number != 1 || !document_has_shanai_lan_fdm_command_evidence(document) {
        return None;
    }

    let bytes = document_text_raw_stream(document)?;
    let style_resolver = DocumentTextStyleResolver::from_document_text_bytes(bytes);
    let map = map_document_text(bytes);
    let group_offsets = shanai_lan_text_group_offsets(bytes);
    let line_headers = shanai_lan_line_headers_in_groups(bytes, &group_offsets);
    let max_extent_units = shanai_lan_text_max_extent_units(bytes)
        .unwrap_or(0x0118)
        .saturating_sub(SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS)
        .max(1);
    let viewport = fdm_projection_viewport(layout);
    let grid_unit_px = viewport.width / f32::from(max_extent_units);
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let fallback_font_units = 12u16;
    let line_height_px =
        f32::from(fallback_font_units) * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR * scale_y;
    let mut slots = Vec::new();

    for entry in map
        .entries()
        .iter()
        .filter(|entry| entry.kind() == DocumentTextMapKind::TextRun)
    {
        let fragments = shanai_lan_visible_text_fragments(entry.text());
        if fragments.is_empty() {
            continue;
        }
        let source_span = TextSourceSpan::from_document_text_entry(entry);
        let line_header = shanai_lan_line_header_for_text_entry(bytes, entry);
        let group_index = shanai_lan_group_index_for_text_entry(&group_offsets, entry);
        let leading_units = leading_display_units(entry.text());
        let line_offset_units = line_header
            .as_ref()
            .map(|header| header.offset_units)
            .unwrap_or_default();
        let font_size_units = line_header
            .as_ref()
            .map(|header| header.font_size_units)
            .unwrap_or(fallback_font_units);
        let font_size = f32::from(font_size_units) * SHANAI_LAN_TEXT_FONT_SIZE_SCALE * scale_y;
        let y = group_index
            .map(|index| viewport.y + (index as f32 + 1.0) * line_height_px)
            .unwrap_or(viewport.y);
        let line_header_hex = line_header
            .as_ref()
            .and_then(|header| bytes.get(header.start..header.end))
            .map(hex_bytes)
            .unwrap_or_default();
        let line_header_raw_words = line_header
            .as_ref()
            .map(|header| header.raw_words)
            .unwrap_or([0; 12]);
        let line_header_same_segment_group_run = line_header.as_ref().and_then(|header| {
            group_index.and_then(|group_index| {
                shanai_lan_line_header_same_segment_group_run(
                    &line_headers,
                    group_index,
                    header.offset_units,
                    header.extent_units,
                )
            })
        });
        let fragment_count = fragments.len();
        let parent_text_unit_count = entry.text().encode_utf16().count();
        for (fragment_index, fragment) in fragments.iter().enumerate() {
            let fragment_grid_units = fragment.fragment_start_units.saturating_mul(2);
            let x = viewport.x
                + (f32::from(line_offset_units)
                    + leading_units.saturating_mul(2) as f32
                    + fragment_grid_units as f32)
                    * grid_unit_px;
            let fragment_source_span = source_span
                .subspan_by_units(fragment.source_start_units, fragment.source_end_units);
            let property_15_color_candidate =
                document_text_property_15_color_candidate(&style_resolver, &fragment_source_span);
            let fill = property_15_color_candidate
                .as_ref()
                .map(|candidate| candidate.css_color)
                .unwrap_or_else(fallback_text_fill_color);
            let fill_basis = property_15_color_candidate
                .as_ref()
                .map(|_| DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS)
                .unwrap_or("default-text-fill");
            let previous_gap_units = (fragment_index > 0).then(|| {
                fragment
                    .source_start_units
                    .saturating_sub(fragments[fragment_index - 1].source_end_units)
            });
            let next_gap_units = (fragment_index + 1 < fragment_count).then(|| {
                fragments[fragment_index + 1]
                    .source_start_units
                    .saturating_sub(fragment.source_end_units)
            });
            let text_count_range_evidence =
                shanai_lan_text_count_range_evidence(document, &fragment_source_span);
            let style_link_evidence = shanai_lan_text_style_link_evidence(
                document,
                bytes,
                entry,
                &text_count_range_evidence,
            );
            slots.push(ShanaiLanTextSlot {
                text: fragment.text.clone(),
                x,
                y,
                font_size,
                fill,
                fill_basis,
                document_text_property_15_color_candidate: property_15_color_candidate,
                style_link_evidence,
                source_span: fragment_source_span,
                fragment_context: ShanaiLanTextRunFragmentContext {
                    parent_source_span: source_span.clone(),
                    parent_text_unit_count,
                    fragment_index,
                    fragment_count,
                    fragment_source_start_units: fragment.source_start_units,
                    fragment_source_end_units: fragment.source_end_units,
                    previous_gap_units,
                    next_gap_units,
                    style_boundary_proven: false,
                    promotion_blocked_reason: "document-text-fragment-style-boundary-unproven",
                },
                text_count_range_evidence,
                group_index,
                line_offset_units,
                leading_units,
                fragment_start_units: fragment.fragment_start_units,
                split_from_text_run: fragment.split_from_text_run,
                line_header_hex: line_header_hex.clone(),
                line_header_raw_words,
                line_header_same_segment_group_run,
                line_header_same_segment_group_run_text_slot_count: None,
                line_header_same_segment_group_run_distinct_text_group_count: None,
            });
        }
    }

    attach_shanai_lan_line_header_same_segment_text_peer_counts(&mut slots);

    (!slots.is_empty()).then_some(ShanaiLanTextProjection {
        source: "/DocumentText",
        projection_kind: "documentTextGroupLineProjection",
        grid_unit_px,
        line_height_px,
        slots,
    })
}

pub(super) fn attach_shanai_lan_line_header_same_segment_text_peer_counts(
    slots: &mut [ShanaiLanTextSlot],
) {
    let mut peer_counts = BTreeMap::<(u16, u16, usize, usize), (usize, BTreeSet<usize>)>::new();
    for slot in slots.iter() {
        let Some(run) = slot.line_header_same_segment_group_run else {
            continue;
        };
        let key = (
            run.offset_units,
            run.extent_units,
            run.start_group_index,
            run.end_group_index,
        );
        let entry = peer_counts.entry(key).or_default();
        entry.0 += 1;
        if let Some(group_index) = slot.group_index {
            entry.1.insert(group_index);
        }
    }

    for slot in slots.iter_mut() {
        let Some(run) = slot.line_header_same_segment_group_run else {
            continue;
        };
        let key = (
            run.offset_units,
            run.extent_units,
            run.start_group_index,
            run.end_group_index,
        );
        if let Some((text_slot_count, group_indexes)) = peer_counts.get(&key) {
            slot.line_header_same_segment_group_run_text_slot_count = Some(*text_slot_count);
            slot.line_header_same_segment_group_run_distinct_text_group_count =
                Some(group_indexes.len());
        }
    }
}

pub(super) fn shanai_lan_document_text_line_rule_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ShanaiLanLineRuleProjection> {
    if page_number != 1 || !document_has_shanai_lan_fdm_command_evidence(document) {
        return None;
    }

    let bytes = document_text_raw_stream(document)?;
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    let line_mark_profile = shanai_lan_line_mark_profile(document);
    shanai_lan_document_text_line_rule_projection_from_bytes(
        bytes,
        layout,
        &line_mark_intervals,
        line_mark_profile,
    )
}

pub(super) fn shanai_lan_document_text_line_rule_projection_from_bytes(
    bytes: &[u8],
    layout: PageLayout,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    line_mark_profile: &'static str,
) -> Option<ShanaiLanLineRuleProjection> {
    let map = map_document_text(bytes);
    let group_offsets = shanai_lan_text_group_offsets(bytes);
    let document_text_group_count = group_offsets.len();
    let document_text_line_header_count =
        shanai_lan_line_headers_in_groups(bytes, &group_offsets).len();
    let raw_max_extent_units = shanai_lan_text_max_extent_units(bytes).unwrap_or(0x0118);
    let max_extent_units = raw_max_extent_units
        .saturating_sub(SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS)
        .max(1);
    let viewport = fdm_projection_viewport(layout);
    let grid_unit_px = viewport.width / f32::from(max_extent_units);
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let line_height_px = 12.0 * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR * scale_y;
    let mut rules = Vec::new();
    let mut anchor_units = BTreeSet::new();
    let mut skipped_inline_line_header_count = 0usize;

    for entry in map
        .entries()
        .iter()
        .filter(|entry| entry.kind() == DocumentTextMapKind::SkippedInlineText)
    {
        let mut offset = entry.byte_start();
        while offset + 24 <= entry.byte_end().min(bytes.len()) {
            if let Some(header) = shanai_lan_line_header_at(bytes, offset)
                && header.end <= entry.byte_end()
                && header.extent_units > header.offset_units
                && header.extent_units.saturating_sub(header.offset_units)
                    >= SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS
                && let Some(group_index) =
                    shanai_lan_group_index_for_text_entry(&group_offsets, entry)
            {
                let x1 = viewport.x + f32::from(header.offset_units) * grid_unit_px;
                let x2 = viewport.x + f32::from(header.extent_units) * grid_unit_px;
                let y = viewport.y + (group_index as f32 + 1.0) * line_height_px;
                let line_header_hex = bytes
                    .get(header.start..header.end)
                    .map(hex_bytes)
                    .unwrap_or_default();
                let line_mark = shanai_lan_line_mark_for_header(line_mark_intervals, &header);
                anchor_units.insert(header.offset_units);
                anchor_units.insert(header.extent_units);
                skipped_inline_line_header_count += 1;
                rules.push(ShanaiLanLineRule {
                    x1,
                    y1: y,
                    x2,
                    y2: y,
                    orientation: "horizontal",
                    candidate_source: "skippedInlineLineHeader",
                    source_span: TextSourceSpan::new(
                        header.start,
                        header.end,
                        header.start / 2,
                        header.end / 2,
                    ),
                    group_index,
                    end_group_index: group_index,
                    line_offset_units: header.offset_units,
                    line_extent_units: header.extent_units,
                    line_header_hex,
                    line_header_raw_words: header.raw_words,
                    line_mark,
                });
                offset = header.end;
            } else {
                offset += 2;
            }
        }
    }
    append_shanai_lan_vertical_anchor_line_rules(
        bytes,
        &group_offsets,
        raw_max_extent_units,
        &anchor_units,
        viewport,
        grid_unit_px,
        line_height_px,
        &mut rules,
        line_mark_intervals,
    );

    (!rules.is_empty()).then_some(ShanaiLanLineRuleProjection {
        source: "/DocumentText",
        projection_kind: "documentTextLineRuleProjection",
        line_mark_profile,
        line_mark_interval_count: line_mark_intervals.len(),
        document_text_group_count,
        document_text_line_header_count,
        skipped_inline_line_header_count,
        grid_unit_px,
        line_height_px,
        stroke_width: SHANAI_LAN_LINE_RULE_STROKE_WIDTH_PX * scale_y,
        rules,
    })
}

pub(super) fn shanai_lan_line_mark_intervals(
    document: &Document,
) -> Vec<ShanaiLanLineMarkInterval> {
    let Some(bytes) = document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/LineMark")
        .map(RawStream::bytes)
    else {
        return Vec::new();
    };
    shanai_lan_line_mark_intervals_from_bytes(bytes)
}

pub(super) fn shanai_lan_line_mark_profile(document: &Document) -> &'static str {
    document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/LineMark")
        .map(RawStream::bytes)
        .map(shanai_lan_line_mark_profile_from_bytes)
        .unwrap_or(SHANAI_LAN_LINE_MARK_PROFILE_ABSENT)
}

pub(super) fn shanai_lan_line_mark_profile_from_bytes(bytes: &[u8]) -> &'static str {
    if !shanai_lan_line_mark_intervals_from_bytes(bytes).is_empty() {
        return SHANAI_LAN_LINE_MARK_PROFILE_BE_DELTA_V1;
    }
    if utf16le_ascii_contains(bytes, "MacrosStreamStyle") {
        return SHANAI_LAN_LINE_MARK_PROFILE_MACRO_STYLE;
    }
    SHANAI_LAN_LINE_MARK_PROFILE_UNPARSED
}

pub(super) fn shanai_lan_line_mark_intervals_from_bytes(
    bytes: &[u8],
) -> Vec<ShanaiLanLineMarkInterval> {
    let Some(count) = read_be16_at(bytes, LINE_MARK_BE_DELTA_COUNT_OFFSET).map(usize::from) else {
        return Vec::new();
    };
    if count == 0
        || bytes.len()
            < LINE_MARK_BE_DELTA_HEADER_BYTES
                + count.saturating_mul(LINE_MARK_BE_DELTA_RECORD_BYTES)
    {
        return Vec::new();
    }

    let mut intervals = Vec::new();
    let mut unit_start = LINE_MARK_BE_DELTA_BASE_UNIT;
    for record_index in 0..count {
        let offset = line_mark_be_delta_record_byte_offset(record_index);
        let Some(delta_word) = read_be16_at(bytes, offset) else {
            break;
        };
        let Some(flag_word) = read_be16_at(bytes, offset + 2) else {
            break;
        };
        let delta = delta_word as i16;
        if delta <= 0 {
            break;
        }
        let unit_end = unit_start.saturating_add(delta as usize);
        intervals.push(ShanaiLanLineMarkInterval {
            record_index,
            unit_start,
            unit_end,
            flag_word,
        });
        unit_start = unit_end;
    }
    intervals
}

pub(super) fn shanai_lan_line_mark_for_header(
    intervals: &[ShanaiLanLineMarkInterval],
    header: &ShanaiLanLineHeader,
) -> Option<ShanaiLanLineMarkInterval> {
    let unit = header.start / 2;
    intervals
        .iter()
        .copied()
        .find(|interval| interval.unit_start <= unit && unit < interval.unit_end)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn append_shanai_lan_vertical_anchor_line_rules(
    bytes: &[u8],
    group_offsets: &[usize],
    raw_max_extent_units: u16,
    anchor_units: &BTreeSet<u16>,
    viewport: FdmProjectionViewport,
    grid_unit_px: f32,
    line_height_px: f32,
    rules: &mut Vec<ShanaiLanLineRule>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) {
    let hidden_groups = rules
        .iter()
        .filter(|rule| rule.orientation == "horizontal")
        .map(|rule| rule.group_index)
        .collect::<BTreeSet<_>>();
    let Some(first_hidden_group) = hidden_groups.iter().next().copied() else {
        return;
    };
    let Some(last_hidden_group) = hidden_groups.iter().next_back().copied() else {
        return;
    };

    let min_group = first_hidden_group.saturating_sub(4);
    let max_group = last_hidden_group.saturating_add(1);
    let line_headers = shanai_lan_line_headers_in_groups(bytes, group_offsets);

    for anchor_unit in anchor_units {
        if *anchor_unit == 0 || *anchor_unit == raw_max_extent_units {
            continue;
        }

        let mut support_by_group = BTreeMap::new();
        for line_header in line_headers.iter().filter(|line_header| {
            (min_group..=max_group).contains(&line_header.group_index)
                && (line_header.header.offset_units == *anchor_unit
                    || line_header.header.extent_units == *anchor_unit)
        }) {
            support_by_group
                .entry(line_header.group_index)
                .or_insert(*line_header);
        }

        let mut run_start: Option<ShanaiLanLineHeaderInGroup> = None;
        let mut previous: Option<ShanaiLanLineHeaderInGroup> = None;
        for line_header in support_by_group.values().copied() {
            match previous {
                Some(previous_header)
                    if line_header.group_index == previous_header.group_index + 1 =>
                {
                    previous = Some(line_header);
                }
                Some(previous_header) => {
                    push_shanai_lan_vertical_anchor_line_rule(
                        bytes,
                        viewport,
                        grid_unit_px,
                        line_height_px,
                        *anchor_unit,
                        run_start.unwrap_or(previous_header),
                        previous_header,
                        rules,
                        line_mark_intervals,
                    );
                    run_start = Some(line_header);
                    previous = Some(line_header);
                }
                None => {
                    run_start = Some(line_header);
                    previous = Some(line_header);
                }
            }
        }
        if let (Some(run_start), Some(previous)) = (run_start, previous) {
            push_shanai_lan_vertical_anchor_line_rule(
                bytes,
                viewport,
                grid_unit_px,
                line_height_px,
                *anchor_unit,
                run_start,
                previous,
                rules,
                line_mark_intervals,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_shanai_lan_vertical_anchor_line_rule(
    bytes: &[u8],
    viewport: FdmProjectionViewport,
    grid_unit_px: f32,
    line_height_px: f32,
    anchor_unit: u16,
    run_start: ShanaiLanLineHeaderInGroup,
    run_end: ShanaiLanLineHeaderInGroup,
    rules: &mut Vec<ShanaiLanLineRule>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) {
    if run_end.group_index <= run_start.group_index {
        return;
    }
    let x = viewport.x + f32::from(anchor_unit) * grid_unit_px;
    let y1 = viewport.y + (run_start.group_index as f32 + 1.0) * line_height_px;
    let y2 = viewport.y + (run_end.group_index as f32 + 1.0) * line_height_px;
    let source_start = run_start.header.start.min(run_end.header.start);
    let source_end = run_start.header.end.max(run_end.header.end);
    let line_header_hex = bytes
        .get(run_start.header.start..run_start.header.end)
        .map(hex_bytes)
        .unwrap_or_default();
    let line_mark = shanai_lan_line_mark_for_header(line_mark_intervals, &run_start.header);
    rules.push(ShanaiLanLineRule {
        x1: x,
        y1,
        x2: x,
        y2,
        orientation: "vertical",
        candidate_source: "verticalAnchorRunFromLineHeaders",
        source_span: TextSourceSpan::new(
            source_start,
            source_end,
            source_start / 2,
            source_end / 2,
        ),
        group_index: run_start.group_index,
        end_group_index: run_end.group_index,
        line_offset_units: anchor_unit,
        line_extent_units: anchor_unit,
        line_header_hex,
        line_header_raw_words: run_start.header.raw_words,
        line_mark,
    });
}

pub(super) fn shanai_lan_line_headers_in_groups(
    bytes: &[u8],
    group_offsets: &[usize],
) -> Vec<ShanaiLanLineHeaderInGroup> {
    let mut headers = Vec::new();
    let mut offset = 0usize;
    while offset + 24 <= bytes.len() {
        if let Some(header) = shanai_lan_line_header_at(bytes, offset) {
            if let Some(group_index) =
                shanai_lan_group_index_for_byte_offset(group_offsets, header.start)
            {
                headers.push(ShanaiLanLineHeaderInGroup {
                    group_index,
                    header,
                });
            }
            offset = header.end;
        } else {
            offset += 2;
        }
    }
    headers
}

pub(super) fn shanai_lan_text_group_offsets(bytes: &[u8]) -> Vec<usize> {
    let mut offsets = Vec::new();
    let mut offset = 0usize;
    while offset + 4 <= bytes.len() {
        if bytes[offset..].starts_with(&[0x00, 0x1c, 0x00, 0x10]) {
            offsets.push(offset);
        }
        offset += 2;
    }
    offsets
}

pub(super) fn shanai_lan_text_max_extent_units(bytes: &[u8]) -> Option<u16> {
    let mut max_extent: Option<u16> = None;
    let mut offset = 0usize;
    while offset + 24 <= bytes.len() {
        if let Some(header) = shanai_lan_line_header_at(bytes, offset) {
            max_extent = Some(max_extent.unwrap_or(0).max(header.extent_units));
            offset = header.end;
        } else {
            offset += 2;
        }
    }
    max_extent
}

pub(super) fn shanai_lan_group_index_for_text_entry(
    group_offsets: &[usize],
    entry: &DocumentTextMapEntry,
) -> Option<usize> {
    shanai_lan_group_index_for_byte_offset(group_offsets, entry.byte_start())
}

pub(super) fn shanai_lan_group_index_for_byte_offset(
    group_offsets: &[usize],
    byte_offset: usize,
) -> Option<usize> {
    group_offsets
        .iter()
        .rposition(|offset| *offset < byte_offset)
}

pub(super) fn shanai_lan_line_header_for_text_entry(
    bytes: &[u8],
    entry: &DocumentTextMapEntry,
) -> Option<ShanaiLanLineHeader> {
    let search_start = entry.byte_start().saturating_sub(64);
    let mut offset = entry.byte_start().saturating_sub(2);
    while offset >= search_start && offset + 24 <= bytes.len() {
        if let Some(header) = shanai_lan_line_header_at(bytes, offset)
            && header.end <= entry.byte_start()
        {
            return Some(header);
        }
        if offset < 2 {
            break;
        }
        offset -= 2;
    }
    None
}

pub(super) fn shanai_lan_line_header_at(
    bytes: &[u8],
    offset: usize,
) -> Option<ShanaiLanLineHeader> {
    if offset + 24 > bytes.len() || !bytes[offset..].starts_with(&[0x00, 0x1c, 0x00, 0x30]) {
        return None;
    }
    let mut words = [0u16; 12];
    for (index, chunk) in bytes[offset..offset + 24].chunks_exact(2).enumerate() {
        words[index] = u16::from_be_bytes([chunk[0], chunk[1]]);
    }
    if words[2] == 0
        || words[6] != 0x00ff
        || words[7] != 0
        || words[9] != 0
        || words[10] != 0x0030
        || words[11] != 0x001f
    {
        return None;
    }
    Some(ShanaiLanLineHeader {
        offset_units: words[4],
        extent_units: words[5],
        font_size_units: words[2],
        raw_words: words,
        start: offset,
        end: offset + 24,
    })
}

pub(super) fn shanai_lan_visible_text_fragments(text: &str) -> Vec<ShanaiLanTextFragment> {
    let characters = text.chars().collect::<Vec<_>>();
    let mut visible_start = 0usize;
    while visible_start < characters.len()
        && matches!(
            characters[visible_start],
            ' ' | '\u{3000}' | '\n' | '\r' | '\t'
        )
    {
        visible_start += 1;
    }

    let mut visible_end = characters.len();
    while visible_end > visible_start
        && matches!(
            characters[visible_end - 1],
            ' ' | '\u{3000}' | '\n' | '\r' | '\t'
        )
    {
        visible_end -= 1;
    }

    if visible_start >= visible_end {
        return Vec::new();
    }

    let visible_text = characters[visible_start..visible_end]
        .iter()
        .collect::<String>();
    if visible_text.trim_matches(|character| matches!(character, ' ' | '\u{3000}')) == "#" {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut fragment_start = visible_start;
    let mut offset = visible_start;
    while offset < visible_end {
        if matches!(characters[offset], ' ' | '\u{3000}') {
            let gap_start = offset;
            let mut gap_end = offset;
            while gap_end < visible_end && matches!(characters[gap_end], ' ' | '\u{3000}') {
                gap_end += 1;
            }
            let gap_units = shanai_lan_spacing_units_for_chars(&characters[gap_start..gap_end]);
            if gap_units >= SHANAI_LAN_TEXT_FRAGMENT_GAP_UNITS
                && fragment_start < gap_start
                && gap_end < visible_end
            {
                ranges.push((fragment_start, gap_start));
                fragment_start = gap_end;
            }
            offset = gap_end;
        } else {
            offset += 1;
        }
    }
    if fragment_start < visible_end {
        ranges.push((fragment_start, visible_end));
    }

    let split_from_text_run = ranges.len() > 1;
    ranges
        .into_iter()
        .filter_map(|(start, end)| {
            let fragment_text = characters[start..end].iter().collect::<String>();
            let trimmed =
                fragment_text.trim_matches(|character| matches!(character, ' ' | '\u{3000}'));
            if trimmed.is_empty() || trimmed == "#" {
                return None;
            }
            Some(ShanaiLanTextFragment {
                text: trimmed.to_string(),
                source_start_units: utf16_units_for_chars(&characters[..start]),
                source_end_units: utf16_units_for_chars(&characters[..end]),
                fragment_start_units: shanai_lan_spacing_units_for_chars(
                    &characters[visible_start..start],
                ),
                split_from_text_run,
            })
        })
        .collect()
}

pub(super) fn shanai_lan_spacing_units_for_chars(characters: &[char]) -> usize {
    characters
        .iter()
        .map(|character| match character {
            '\u{3000}' => 2,
            '\t' => APP_TAB_COLUMNS,
            _ => character.len_utf16(),
        })
        .sum()
}

pub(super) fn push_shanai_lan_text_projection_svg(
    svg: &mut String,
    projection: &ShanaiLanTextProjection,
    font_family: &str,
) {
    svg.push_str(&format!(
        "<g class=\"rjtd-shanai-lan-text-projection\" data-source=\"{}\" data-projection-kind=\"{}\" data-decoded=\"false\" data-placement-proven=\"false\" data-grid-unit-px=\"{:.3}\" data-line-height-px=\"{:.3}\" data-baseline-factor=\"{:.3}\">",
        escape_xml(projection.source),
        escape_xml(projection.projection_kind),
        projection.grid_unit_px,
        projection.line_height_px,
        SHANAI_LAN_TEXT_BASELINE_FACTOR
    ));
    let font_family = escape_xml(font_family);
    for slot in &projection.slots {
        let group_index = slot
            .group_index
            .map(|index| index.to_string())
            .unwrap_or_else(|| "-".to_string());
        let text_count_range_indexes =
            shanai_lan_text_count_range_indexes_attr(&slot.text_count_range_evidence);
        let text_count_range_bases =
            shanai_lan_text_count_range_bases_attr(&slot.text_count_range_evidence);
        let line_header_raw_words_hex =
            shanai_lan_line_header_raw_words_hex_attr(&slot.line_header_raw_words);
        let document_view_style_group_candidate = slot
            .style_link_evidence
            .document_view_style_group_candidate
            .map(|group_id| group_id.to_string())
            .unwrap_or_else(|| "-".to_string());
        let document_view_style_group_candidate_basis = slot
            .style_link_evidence
            .document_view_style_group_candidate_basis
            .unwrap_or("-");
        let fill_color_promotion_blocked_reason = slot
            .style_link_evidence
            .fill_color_promotion_blocked_reason
            .unwrap_or("-");
        let group_header_candidate = slot
            .style_link_evidence
            .document_text_group_header_candidate
            .as_ref();
        let group_header_candidate_present = group_header_candidate.is_some();
        let group_header_candidate_raw_words_hex = group_header_candidate
            .map(|candidate| shanai_lan_u16_words_hex_attr(&candidate.raw_words))
            .unwrap_or_default();
        let group_header_candidate_blocked_reason = group_header_candidate
            .map(|candidate| candidate.promotion_blocked_reason)
            .unwrap_or("-");
        let inline_style_candidate = slot
            .style_link_evidence
            .document_text_inline_style_candidate
            .as_ref();
        let inline_style_candidate_present = inline_style_candidate.is_some();
        let inline_style_candidate_selector = inline_style_candidate
            .and_then(|candidate| candidate.selector)
            .map(|selector| format!("0x{selector:04x}"))
            .unwrap_or_else(|| "-".to_string());
        let inline_style_candidate_raw_words_hex = inline_style_candidate
            .map(|candidate| shanai_lan_u16_words_hex_attr(&candidate.raw_words))
            .unwrap_or_default();
        let inline_style_candidate_blocked_reason = inline_style_candidate
            .map(|candidate| candidate.promotion_blocked_reason)
            .unwrap_or("-");
        let parent_text_run_byte_range = format!(
            "{}..{}",
            slot.fragment_context.parent_source_span.byte_start(),
            slot.fragment_context.parent_source_span.byte_end()
        );
        let parent_text_run_unit_range = format!(
            "{}..{}",
            slot.fragment_context.parent_source_span.unit_start(),
            slot.fragment_context.parent_source_span.unit_end()
        );
        let fragment_source_unit_range = format!(
            "{}..{}",
            slot.fragment_context.fragment_source_start_units,
            slot.fragment_context.fragment_source_end_units
        );
        let previous_gap_units = slot
            .fragment_context
            .previous_gap_units
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string());
        let next_gap_units = slot
            .fragment_context
            .next_gap_units
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string());
        let same_segment_run_present = slot.line_header_same_segment_group_run.is_some();
        let same_segment_run_start = slot
            .line_header_same_segment_group_run
            .map(|run| run.start_group_index.to_string())
            .unwrap_or_else(|| "-".to_string());
        let same_segment_run_end = slot
            .line_header_same_segment_group_run
            .map(|run| run.end_group_index.to_string())
            .unwrap_or_else(|| "-".to_string());
        let same_segment_run_count = slot
            .line_header_same_segment_group_run
            .map(|run| run.group_count.to_string())
            .unwrap_or_else(|| "-".to_string());
        let same_segment_run_position = slot
            .line_header_same_segment_group_run
            .map(|run| run.position_in_run.to_string())
            .unwrap_or_else(|| "-".to_string());
        let same_segment_run_text_slot_count = slot
            .line_header_same_segment_group_run_text_slot_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "-".to_string());
        let same_segment_run_distinct_text_group_count = slot
            .line_header_same_segment_group_run_distinct_text_group_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "-".to_string());
        let same_segment_run_ambiguous_as_row_anchor = slot
            .line_header_same_segment_group_run_distinct_text_group_count
            .is_some_and(|count| count > 1);
        let line_header_y_placement_blocked_detail = if same_segment_run_ambiguous_as_row_anchor {
            "same-segment-run-spans-multiple-visible-text-rows"
        } else {
            "line-header-y-run-transform-undecoded"
        };
        let property_15_color_candidate = slot.document_text_property_15_color_candidate.as_ref();
        let property_15_color_candidate_present = property_15_color_candidate.is_some();
        let property_15_packed_bgr = property_15_color_candidate
            .map(|candidate| format!("0x{:08x}", candidate.packed_bgr))
            .unwrap_or_else(|| "-".to_string());
        svg.push_str(&format!(
            "<text class=\"rjtd-text rjtd-shanai-lan-text\" data-source=\"{}\" data-projection-kind=\"{}\" data-group-index=\"{}\" data-line-offset-units=\"{}\" data-leading-units=\"{}\" data-fragment-start-units=\"{}\" data-split-from-text-run=\"{}\" data-parent-text-run-byte-range=\"{}\" data-parent-text-run-unit-range=\"{}\" data-parent-text-run-unit-count=\"{}\" data-fragment-index=\"{}\" data-fragment-count=\"{}\" data-fragment-source-unit-range=\"{}\" data-previous-gap-units=\"{}\" data-next-gap-units=\"{}\" data-fragment-style-boundary-proven=\"{}\" data-fragment-style-blocked-reason=\"{}\" data-line-header-hex=\"{}\" data-line-header-raw-words-hex=\"{}\" data-line-header-same-segment-run-present=\"{}\" data-line-header-same-segment-run-start-group=\"{}\" data-line-header-same-segment-run-end-group=\"{}\" data-line-header-same-segment-run-group-count=\"{}\" data-line-header-same-segment-run-position=\"{}\" data-line-header-same-segment-run-text-slot-count=\"{}\" data-line-header-same-segment-run-distinct-text-group-count=\"{}\" data-line-header-same-segment-run-ambiguous-row-anchor=\"{}\" data-line-header-y-placement-blocked-detail=\"{}\" data-line-header-y-placement-blocked-reason=\"line-header-y-run-placement-semantics-unproven\" data-document-text-property-15-color-candidate=\"{}\" data-document-text-property-15-packed-bgr=\"{}\" data-document-text-property-15-role-decoded=\"false\" data-fill-color-basis=\"{}\" data-style-link-proven=\"{}\" data-style-link-blocked-reason=\"{}\" data-text-layout-style-record-count=\"{}\" data-document-view-style-group-count=\"{}\" data-document-view-style-group-candidate=\"{}\" data-document-view-style-group-candidate-basis=\"{}\" data-document-text-group-header-candidate=\"{}\" data-document-text-group-header-raw-words-hex=\"{}\" data-document-text-group-header-blocked-reason=\"{}\" data-document-text-inline-style-candidate=\"{}\" data-document-text-inline-style-selector=\"{}\" data-document-text-inline-style-raw-words-hex=\"{}\" data-document-text-inline-style-blocked-reason=\"{}\" data-fill-color-promotion-blocked-reason=\"{}\" data-text-count-range-evidence-count=\"{}\" data-text-count-range-indexes=\"{}\" data-text-count-range-bases=\"{}\" x=\"{:.1}\" y=\"{:.1}\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"{}\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            escape_xml(projection.source),
            escape_xml(projection.projection_kind),
            escape_xml(&group_index),
            slot.line_offset_units,
            slot.leading_units,
            slot.fragment_start_units,
            slot.split_from_text_run,
            escape_xml(&parent_text_run_byte_range),
            escape_xml(&parent_text_run_unit_range),
            slot.fragment_context.parent_text_unit_count,
            slot.fragment_context.fragment_index,
            slot.fragment_context.fragment_count,
            escape_xml(&fragment_source_unit_range),
            escape_xml(&previous_gap_units),
            escape_xml(&next_gap_units),
            slot.fragment_context.style_boundary_proven,
            escape_xml(slot.fragment_context.promotion_blocked_reason),
            escape_xml(&slot.line_header_hex),
            escape_xml(&line_header_raw_words_hex),
            same_segment_run_present,
            escape_xml(&same_segment_run_start),
            escape_xml(&same_segment_run_end),
            escape_xml(&same_segment_run_count),
            escape_xml(&same_segment_run_position),
            escape_xml(&same_segment_run_text_slot_count),
            escape_xml(&same_segment_run_distinct_text_group_count),
            same_segment_run_ambiguous_as_row_anchor,
            escape_xml(line_header_y_placement_blocked_detail),
            property_15_color_candidate_present,
            escape_xml(&property_15_packed_bgr),
            escape_xml(slot.fill_basis),
            slot.style_link_evidence.style_link_proven,
            escape_xml(
                slot.style_link_evidence
                    .style_link_promotion_blocked_reason
            ),
            slot.style_link_evidence.text_layout_style_record_count,
            slot.style_link_evidence.document_view_style_group_count,
            escape_xml(&document_view_style_group_candidate),
            escape_xml(document_view_style_group_candidate_basis),
            group_header_candidate_present,
            escape_xml(&group_header_candidate_raw_words_hex),
            escape_xml(group_header_candidate_blocked_reason),
            inline_style_candidate_present,
            escape_xml(&inline_style_candidate_selector),
            escape_xml(&inline_style_candidate_raw_words_hex),
            escape_xml(inline_style_candidate_blocked_reason),
            escape_xml(fill_color_promotion_blocked_reason),
            slot.text_count_range_evidence.len(),
            escape_xml(&text_count_range_indexes),
            escape_xml(&text_count_range_bases),
            slot.x,
            shanai_lan_text_baseline_y(slot),
            font_family,
            slot.font_size,
            slot.fill,
            escape_xml(&svg_visual_text(&slot.text))
        ));
    }
    svg.push_str("</g>");
}

pub(super) fn shanai_lan_text_count_range_indexes_attr(
    evidence: &[ShanaiLanTextCountRangeEvidence],
) -> String {
    evidence
        .iter()
        .map(|item| item.index.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn shanai_lan_text_count_range_bases_attr(
    evidence: &[ShanaiLanTextCountRangeEvidence],
) -> String {
    evidence
        .iter()
        .map(|item| item.basis.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn shanai_lan_line_header_raw_words_hex_attr(words: &[u16; 12]) -> String {
    shanai_lan_u16_words_hex_attr(words)
}

pub(super) fn shanai_lan_u16_words_hex_attr(words: &[u16]) -> String {
    words
        .iter()
        .map(|word| format!("0x{word:04x}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn shanai_lan_text_baseline_y(slot: &ShanaiLanTextSlot) -> f32 {
    slot.y + slot.font_size * SHANAI_LAN_TEXT_BASELINE_FACTOR
}

pub(super) fn document_has_shanai_lan_fdm_frame_evidence(document: &Document) -> bool {
    if !document_plain_text(document).contains("社内LAN構成図") {
        return false;
    }

    let linked_image_rows = document
        .object_stream_candidates()
        .iter()
        .flat_map(ObjectStreamCandidate::fdm_index_entry_candidates)
        .filter(|entry| !entry.segment_image_signature_hits().is_empty())
        .filter(|entry| fdm_frame_record_for_entry(document, entry).is_some())
        .count();
    linked_image_rows >= 2
}

pub(super) fn document_has_shanai_lan_fdm_command_evidence(document: &Document) -> bool {
    if !document_plain_text(document).contains("社内LAN構成図") {
        return false;
    }

    let mut row_count = 0usize;
    let mut bbox_count = 0usize;
    for entry in document
        .object_stream_candidates()
        .iter()
        .flat_map(ObjectStreamCandidate::fdm_index_entry_candidates)
    {
        if !entry.vector_commands().is_empty() {
            row_count += 1;
        }
        bbox_count += entry
            .vector_commands()
            .iter()
            .filter(|command| command.bbox().is_some())
            .count();
    }
    row_count >= 30 && bbox_count >= 100
}
