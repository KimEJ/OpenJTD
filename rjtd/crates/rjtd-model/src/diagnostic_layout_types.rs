use super::*;

#[derive(Debug)]
pub(crate) struct PageLayerTextFragment {
    pub(crate) text: String,
    pub(crate) paragraph_index: Option<usize>,
    pub(crate) char_start: usize,
    pub(crate) char_end: usize,
    pub(crate) source_span: Option<TextSourceSpan>,
    pub(crate) ruby_annotation: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PageLayerTextPlacement {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) baseline: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestTextSlot {
    pub(crate) role: &'static str,
    pub(crate) text: &'static str,
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestResolvedTextSlot {
    pub(crate) role: &'static str,
    pub(crate) text: &'static str,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) source_span: Option<TextSourceSpan>,
    pub(crate) line_header: Option<ShanaiLanLineHeader>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestFormulaTextSlot {
    pub(crate) embedding_index: usize,
    pub(crate) text: &'static str,
    pub(crate) x: f32,
    pub(crate) baseline_y: f32,
    pub(crate) font_size: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VisualListDiagnostic<'a> {
    pub(crate) candidate_index: usize,
    pub(crate) candidate: &'a ObjectStreamCandidate,
    pub(crate) visual_list: &'a ObjectVisualListCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VisualListHorizontalRun {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) width: usize,
    pub(crate) value: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct VisualListTitleBand {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ObservedFormTextProjection {
    pub(crate) source: &'static str,
    pub(crate) projection_kind: &'static str,
    pub(crate) shapes: Vec<ObservedFormShape>,
    pub(crate) slots: Vec<ObservedFormTextSlot>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ObservedFormShape {
    pub(crate) role: &'static str,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) fill: &'static str,
    pub(crate) stroke: Option<&'static str>,
    pub(crate) stroke_width: f32,
    pub(crate) rx: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ObservedFormTextSlot {
    pub(crate) role: &'static str,
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font_size: f32,
    pub(crate) font_weight: &'static str,
    pub(crate) anchor: &'static str,
    pub(crate) font_family: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageFrameProjection {
    pub(crate) source: &'static str,
    pub(crate) projection_kind: &'static str,
    pub(crate) page_assignment_decoded: bool,
    pub(crate) record_count: usize,
    pub(crate) shapes: Vec<PageFrameShape>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageFrameShape {
    pub(crate) role: &'static str,
    pub(crate) row_index: usize,
    pub(crate) object_id: u16,
    pub(crate) object_type: u16,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) corner_radius: f32,
    pub(crate) source_x: u16,
    pub(crate) source_y: u16,
    pub(crate) source_width: u16,
    pub(crate) source_height: u16,
    pub(crate) source_corner_radius: u16,
    pub(crate) source_style_id: u16,
    pub(crate) placement_basis: &'static str,
    pub(crate) style_basis: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageMarkSeparatorProjection {
    pub(crate) source: &'static str,
    pub(crate) projection_kind: &'static str,
    pub(crate) role: &'static str,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) stroke_width: f32,
    pub(crate) source_record_offset: usize,
    pub(crate) source_record_index: u32,
    pub(crate) source_line_start: u32,
    pub(crate) source_line_end: u32,
    pub(crate) source_y_centipoints: u16,
    pub(crate) source_advance_centipoints: u16,
    pub(crate) placement_basis: &'static str,
    pub(crate) style_basis: &'static str,
    pub(crate) page_assignment_decoded: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LayoutBoxTextProjection {
    pub(crate) source: &'static str,
    pub(crate) projection_kind: &'static str,
    pub(crate) block_count: usize,
    pub(crate) layout_record_count: usize,
    pub(crate) position_table_present: bool,
    pub(crate) page_assignment_decoded: bool,
    pub(crate) slots: Vec<LayoutBoxTextSlot>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LayoutBoxTextSlot {
    pub(crate) role: &'static str,
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font_size: f32,
    pub(crate) line_height: f32,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) block_index: usize,
    pub(crate) layout_record_index: Option<usize>,
    pub(crate) layout_record_byte_range: Option<(usize, usize)>,
    pub(crate) layout_x_pt: Option<u16>,
    pub(crate) layout_y_pt: Option<u16>,
    pub(crate) layout_width_pt: Option<u16>,
    pub(crate) inferred_origin_pt: Option<f32>,
    pub(crate) placement_basis: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LayoutBoxTextBlock {
    pub(crate) index: usize,
    pub(crate) byte_start: usize,
    pub(crate) byte_end: usize,
    pub(crate) payload_start: usize,
    pub(crate) payload_end: usize,
    pub(crate) declared_unit_count: usize,
    pub(crate) fragments: Vec<LayoutBoxTextFragment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LayoutBoxTextFragment {
    pub(crate) text: String,
    pub(crate) source_span: TextSourceSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LayoutBoxRecordCandidate {
    pub(crate) index: usize,
    pub(crate) byte_start: usize,
    pub(crate) byte_end: usize,
    pub(crate) origin_field: Option<u16>,
    pub(crate) x_field: Option<u16>,
    pub(crate) y_field: Option<u16>,
    pub(crate) width_field: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShanaiLanTextProjection {
    pub(crate) source: &'static str,
    pub(crate) projection_kind: &'static str,
    pub(crate) grid_unit_px: f32,
    pub(crate) line_height_px: f32,
    pub(crate) slots: Vec<ShanaiLanTextSlot>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShanaiLanTextSlot {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font_size: f32,
    pub(crate) fill: &'static str,
    pub(crate) fill_basis: &'static str,
    pub(crate) document_text_property_15_color_candidate:
        Option<DocumentTextProperty15ColorCandidate>,
    pub(crate) style_link_evidence: ShanaiLanTextStyleLinkEvidence,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) fragment_context: ShanaiLanTextRunFragmentContext,
    pub(crate) text_count_range_evidence: Vec<ShanaiLanTextCountRangeEvidence>,
    pub(crate) group_index: Option<usize>,
    pub(crate) line_offset_units: u16,
    pub(crate) leading_units: usize,
    pub(crate) fragment_start_units: usize,
    pub(crate) split_from_text_run: bool,
    pub(crate) line_header_hex: String,
    pub(crate) line_header_raw_words: [u16; 12],
    pub(crate) line_header_same_segment_group_run: Option<ShanaiLanLineHeaderSameSegmentGroupRun>,
    pub(crate) line_header_same_segment_group_run_text_slot_count: Option<usize>,
    pub(crate) line_header_same_segment_group_run_distinct_text_group_count: Option<usize>,
}

pub(crate) type ShanaiLanTextSlotAttachment<'a> =
    (&'a ShanaiLanTextSlot, f32, (f32, f32, f32, f32));

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanTextStyleLinkEvidence {
    pub(crate) source: &'static str,
    pub(crate) style_link_proven: bool,
    pub(crate) text_layout_style_record_count: usize,
    pub(crate) document_view_style_group_count: usize,
    pub(crate) document_view_style_group_candidate: Option<u16>,
    pub(crate) document_view_style_group_candidate_basis: Option<&'static str>,
    pub(crate) document_text_group_header_candidate:
        Option<ShanaiLanDocumentTextGroupHeaderCandidate>,
    pub(crate) document_text_inline_style_candidate:
        Option<ShanaiLanDocumentTextInlineStyleCandidate>,
    pub(crate) style_link_promotion_blocked_reason: &'static str,
    pub(crate) fill_color_promotion_blocked_reason: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanTextRunFragmentContext {
    pub(crate) parent_source_span: TextSourceSpan,
    pub(crate) parent_text_unit_count: usize,
    pub(crate) fragment_index: usize,
    pub(crate) fragment_count: usize,
    pub(crate) fragment_source_start_units: usize,
    pub(crate) fragment_source_end_units: usize,
    pub(crate) previous_gap_units: Option<usize>,
    pub(crate) next_gap_units: Option<usize>,
    pub(crate) style_boundary_proven: bool,
    pub(crate) promotion_blocked_reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShanaiLanLineHeaderSameSegmentGroupRun {
    pub(crate) offset_units: u16,
    pub(crate) extent_units: u16,
    pub(crate) start_group_index: usize,
    pub(crate) end_group_index: usize,
    pub(crate) group_count: usize,
    pub(crate) position_in_run: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanDocumentTextGroupHeaderCandidate {
    pub(crate) source_span: TextSourceSpan,
    pub(crate) raw_words: Vec<u16>,
    pub(crate) field_words: Vec<u16>,
    pub(crate) distance_to_text_units: usize,
    pub(crate) promotion_blocked_reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanDocumentTextInlineStyleCandidate {
    pub(crate) source_span: TextSourceSpan,
    pub(crate) selector: Option<u16>,
    pub(crate) context_words: Vec<u16>,
    pub(crate) payload_words: Vec<u16>,
    pub(crate) post_inline_words: Vec<u16>,
    pub(crate) raw_words: Vec<u16>,
    pub(crate) distance_to_text_units: usize,
    pub(crate) promotion_blocked_reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanTextCountRangeEvidence {
    pub(crate) index: usize,
    pub(crate) family: String,
    pub(crate) basis: TextCountRangeOverlapBasis,
    pub(crate) range_start: usize,
    pub(crate) range_end: usize,
    pub(crate) overlap_start: usize,
    pub(crate) overlap_end: usize,
    pub(crate) declared_start: u32,
    pub(crate) declared_end: u32,
    pub(crate) tail_fields: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShanaiLanLineRuleProjection {
    pub(crate) source: &'static str,
    pub(crate) projection_kind: &'static str,
    pub(crate) line_mark_profile: &'static str,
    pub(crate) line_mark_interval_count: usize,
    pub(crate) document_text_group_count: usize,
    pub(crate) document_text_line_header_count: usize,
    pub(crate) skipped_inline_line_header_count: usize,
    pub(crate) grid_unit_px: f32,
    pub(crate) line_height_px: f32,
    pub(crate) stroke_width: f32,
    pub(crate) rules: Vec<ShanaiLanLineRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShanaiLanLineRule {
    pub(crate) x1: f32,
    pub(crate) y1: f32,
    pub(crate) x2: f32,
    pub(crate) y2: f32,
    pub(crate) orientation: &'static str,
    pub(crate) candidate_source: &'static str,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) group_index: usize,
    pub(crate) end_group_index: usize,
    pub(crate) line_offset_units: u16,
    pub(crate) line_extent_units: u16,
    pub(crate) line_header_hex: String,
    pub(crate) line_header_raw_words: [u16; 12],
    pub(crate) line_mark: Option<ShanaiLanLineMarkInterval>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShanaiLanLineRuleTopology {
    pub(crate) start_junction_degree: usize,
    pub(crate) end_junction_degree: usize,
    pub(crate) isolated_endpoint_count: usize,
    pub(crate) orthogonal_graph_candidate: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShanaiLanLineRuleGraphComponentSummary {
    pub(crate) rule_indexes: Vec<usize>,
    pub(crate) bbox: (f32, f32, f32, f32),
    pub(crate) horizontal_rule_count: usize,
    pub(crate) vertical_rule_count: usize,
    pub(crate) orthogonal_graph_rule_count: usize,
    pub(crate) line_mark_matched_rule_count: usize,
    pub(crate) isolated_endpoint_count: usize,
    pub(crate) total_projected_length_px: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShanaiLanLineHeaderInGroup {
    pub(crate) group_index: usize,
    pub(crate) header: ShanaiLanLineHeader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShanaiLanLineMarkInterval {
    pub(crate) record_index: usize,
    pub(crate) unit_start: usize,
    pub(crate) unit_end: usize,
    pub(crate) flag_word: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanTextFragment {
    pub(crate) text: String,
    pub(crate) source_start_units: usize,
    pub(crate) source_end_units: usize,
    pub(crate) fragment_start_units: usize,
    pub(crate) split_from_text_run: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShanaiLanLineHeader {
    pub(crate) offset_units: u16,
    pub(crate) extent_units: u16,
    pub(crate) font_size_units: u16,
    pub(crate) raw_words: [u16; 12],
    pub(crate) start: usize,
    pub(crate) end: usize,
}

pub(crate) fn push_optional_u64_json(output: &mut String, value: Option<u64>) {
    if let Some(value) = value {
        output.push_str(&value.to_string());
    } else {
        output.push_str("null");
    }
}

pub(crate) fn optional_u64_svg_attr(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string())
}

pub(crate) fn aspect_delta_permille(
    frame_width: u128,
    frame_height: u128,
    image_width: u128,
    image_height: u128,
) -> Option<u64> {
    if frame_width == 0 || frame_height == 0 || image_width == 0 || image_height == 0 {
        return None;
    }

    let left = frame_width.saturating_mul(image_height);
    let right = image_width.saturating_mul(frame_height);
    let denominator = left.max(right);
    if denominator == 0 {
        return None;
    }
    Some(((left.abs_diff(right).saturating_mul(1000)) / denominator) as u64)
}

pub(crate) fn canvaskit_replay_mode(mode: &str) -> Result<&'static str> {
    match mode.trim().to_ascii_lowercase().as_str() {
        "" | "default" => Ok("default"),
        "compat" | "compatibility" => Ok("compat"),
        _ => Err(Error::InvalidData(format!(
            "unsupported CanvasKit replay mode: {mode}. allowed modes: default, compat"
        ))),
    }
}

pub(crate) fn canvaskit_replay_plan_json(
    core: &DocumentCore,
    lines: &[PageTextLine],
    mode: &str,
) -> String {
    let mut items = vec![
        "{\"path\":\"root/leaf/0\",\"opType\":\"pageBackground\",\"replayPlane\":\"background\",\"feature\":\"pageBackground\",\"status\":\"direct\",\"reason\":\"directReplaySupported\",\"compatOverlayAllowed\":false,\"detail\":\"backgroundColor=#ffffff;projectionKind=fallback\"}".to_string(),
    ];
    let mut source_id = 0usize;
    let mut op_index = 1usize;

    for line in lines {
        if line.text().is_empty() {
            continue;
        }

        for fragment in page_text_line_fragments(&core.document, line) {
            if fragment.text.is_empty() {
                continue;
            }

            items.push(format!(
                "{{\"path\":\"root/leaf/{op_index}\",\"opType\":\"textRun\",\"replayPlane\":\"flow\",\"feature\":\"textRun\",\"status\":\"direct\",\"reason\":\"directReplaySupported\",\"compatOverlayAllowed\":false,\"detail\":\"projectionKind=fallback;sourceId={source_id}\"}}"
            ));
            source_id += 1;
            op_index += 1;
        }
    }

    let total_items = items.len();
    format!(
        "{{\"mode\":{},\"hiddenCanvas2dOverlayAllowed\":false,\"directReplayRequired\":true,\"summary\":{{\"totalItems\":{total_items},\"directItems\":{total_items},\"directRequiredItems\":0,\"compatOverlayItems\":0,\"textFallbackItems\":0,\"unsupportedItems\":0,\"hiddenOverlayViolations\":0}},\"items\":[{}],\"textVariants\":[]}}",
        json_string(mode),
        items.join(",")
    )
}

pub(crate) fn push_optional_hex_byte_json(output: &mut String, value: Option<&u8>) {
    match value {
        Some(byte) => output.push_str(&json_string(&format!("0x{byte:02x}"))),
        None => output.push_str("null"),
    }
}

pub(crate) fn push_optional_hex_bytes_json(output: &mut String, bytes: Option<&[u8]>) {
    match bytes {
        Some(bytes) => output.push_str(&json_string(&hex_bytes(bytes))),
        None => output.push_str("null"),
    }
}
