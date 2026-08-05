use crate::*;

pub(crate) fn push_shanai_lan_text_projection_svg(
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

pub(crate) fn shanai_lan_text_count_range_indexes_attr(
    evidence: &[ShanaiLanTextCountRangeEvidence],
) -> String {
    evidence
        .iter()
        .map(|item| item.index.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn shanai_lan_text_count_range_bases_attr(
    evidence: &[ShanaiLanTextCountRangeEvidence],
) -> String {
    evidence
        .iter()
        .map(|item| item.basis.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn shanai_lan_line_header_raw_words_hex_attr(words: &[u16; 12]) -> String {
    shanai_lan_u16_words_hex_attr(words)
}

pub(crate) fn shanai_lan_u16_words_hex_attr(words: &[u16]) -> String {
    words
        .iter()
        .map(|word| format!("0x{word:04x}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn shanai_lan_text_baseline_y(slot: &ShanaiLanTextSlot) -> f32 {
    slot.y + slot.font_size * SHANAI_LAN_TEXT_BASELINE_FACTOR
}

pub(crate) fn document_has_shanai_lan_fdm_frame_evidence(document: &Document) -> bool {
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

pub(crate) fn document_has_shanai_lan_fdm_command_evidence(document: &Document) -> bool {
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
