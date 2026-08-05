use super::*;

pub(crate) struct SuccessDataTestAnswerSheetSectionAnchor {
    pub(crate) section_label: String,
    pub(crate) row_index: usize,
    pub(crate) source_interval_index: usize,
    pub(crate) row_source_start: usize,
    pub(crate) row_source_end: usize,
    pub(crate) cell_index: usize,
    pub(crate) cell_source_start: Option<usize>,
    pub(crate) cell_source_end: Option<usize>,
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestAnswerSheetHatchedAreaCandidate {
    pub(crate) source: &'static str,
    pub(crate) top_section_label: String,
    pub(crate) bottom_section_label: String,
    pub(crate) top_row_index: usize,
    pub(crate) bottom_row_index: usize,
    pub(crate) top_source_interval_index: usize,
    pub(crate) bottom_source_interval_index: usize,
    pub(crate) empty_cell_index: usize,
    pub(crate) adjacent_answer_cell_index: usize,
    pub(crate) sheet_left_pt: f32,
    pub(crate) sheet_top_pt: f32,
    pub(crate) sheet_right_pt: f32,
    pub(crate) sheet_bottom_pt: f32,
    pub(crate) top_source_grid: Option<SuccessDataTestLineMarkPageGridCandidate>,
    pub(crate) bottom_source_grid: Option<SuccessDataTestLineMarkPageGridCandidate>,
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestAnswerSheetSourceFrameCandidate {
    pub(crate) source: &'static str,
    pub(crate) candidate_basis: &'static str,
    pub(crate) sparse_table_candidate_index: usize,
    pub(crate) section_anchor_count: usize,
    pub(crate) top_section_label: String,
    pub(crate) bottom_section_label: String,
    pub(crate) top_row_index: usize,
    pub(crate) bottom_row_index: usize,
    pub(crate) top_line_mark_record_index: usize,
    pub(crate) bottom_line_mark_record_index: usize,
    pub(crate) local_top_pt: f32,
    pub(crate) local_bottom_pt: f32,
    pub(crate) source_px_per_sheet_pt_y: f32,
    pub(crate) reference_px_per_sheet_pt_y: f32,
    pub(crate) derived_frame_top_y: f32,
    pub(crate) derived_frame_height: f32,
    pub(crate) reference_frame_top_y: f32,
    pub(crate) reference_frame_height: f32,
    pub(crate) frame_top_residual_px: f32,
    pub(crate) frame_height_residual_px: f32,
    pub(crate) same_page_mark_entry: bool,
    pub(crate) same_page_index_candidate: bool,
    pub(crate) fdm_text_triangle_label_anchor_count: usize,
    pub(crate) triangle_source_bbox: Option<ObjectFdmIndexBbox>,
}

pub(crate) fn push_answer_sheet_rule_topology_evidence_json(
    output: &mut String,
    candidate: &TableCandidate,
) {
    let section_anchors = success_data_test_answer_sheet_section_anchors(candidate);
    output.push_str("{\"source\":\"sparseTableCandidateTopology\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"basis\":\"documentTextControlRows\"");
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
        output.push_str(",\"rowSourceStart\":");
        output.push_str(&anchor.row_source_start.to_string());
        output.push_str(",\"rowSourceEnd\":");
        output.push_str(&anchor.row_source_end.to_string());
        output.push_str(",\"cellIndex\":");
        output.push_str(&anchor.cell_index.to_string());
        output.push_str(",\"cellSourceStart\":");
        push_option_usize_json(output, anchor.cell_source_start);
        output.push_str(",\"cellSourceEnd\":");
        push_option_usize_json(output, anchor.cell_source_end);
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"renderPromotionBlockedReason\":\"sparse-topology-to-physical-row-heights-unproven\",\"geometryDecoded\":false,\"decoded\":false}");
}

pub(crate) fn push_unique_f32(values: &mut Vec<f32>, value: f32) {
    if !values
        .iter()
        .any(|seen| (*seen - value).abs() < f32::EPSILON)
    {
        values.push(value);
    }
}

pub(crate) fn push_answer_sheet_hatched_area_candidate_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    area: &SuccessDataTestAnswerSheetHatchedAreaCandidate,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(area.source));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"topSectionLabel\":");
    output.push_str(&json_string(&area.top_section_label));
    output.push_str(",\"bottomSectionLabel\":");
    output.push_str(&json_string(&area.bottom_section_label));
    output.push_str(",\"topRowIndex\":");
    output.push_str(&area.top_row_index.to_string());
    output.push_str(",\"bottomRowIndex\":");
    output.push_str(&area.bottom_row_index.to_string());
    output.push_str(",\"topSourceIntervalIndex\":");
    output.push_str(&area.top_source_interval_index.to_string());
    output.push_str(",\"bottomSourceIntervalIndex\":");
    output.push_str(&area.bottom_source_interval_index.to_string());
    output.push_str(",\"emptyCellIndex\":");
    output.push_str(&area.empty_cell_index.to_string());
    output.push_str(",\"adjacentAnswerCellIndex\":");
    output.push_str(&area.adjacent_answer_cell_index.to_string());
    output.push_str(",\"sheetBBoxPt\":{\"left\":");
    output.push_str(&format!("{:.3}", area.sheet_left_pt));
    output.push_str(",\"top\":");
    output.push_str(&format!("{:.3}", area.sheet_top_pt));
    output.push_str(",\"right\":");
    output.push_str(&format!("{:.3}", area.sheet_right_pt));
    output.push_str(",\"bottom\":");
    output.push_str(&format!("{:.3}", area.sheet_bottom_pt));
    output.push_str(",\"width\":");
    output.push_str(&format!("{:.3}", area.sheet_right_pt - area.sheet_left_pt));
    output.push_str(",\"height\":");
    output.push_str(&format!("{:.3}", area.sheet_bottom_pt - area.sheet_top_pt));
    output.push_str("},\"topSourceGridCandidate\":");
    match &area.top_source_grid {
        Some(candidate) => push_success_data_test_line_mark_page_grid_candidate_json(
            output, document, layout, candidate, None, None,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"bottomSourceGridCandidate\":");
    match &area.bottom_source_grid {
        Some(candidate) => push_success_data_test_line_mark_page_grid_candidate_json(
            output, document, layout, candidate, None, None,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"hatchStyleCandidate\":{\"source\":\"referenceObservedAnswerAreaEdgeHatch\",\"sourceBacked\":false,\"referenceBacked\":true,\"decoded\":false,\"renderMode\":\"diagonal-edge-segments\",\"renderPromotionBlockedReason\":\"answer-sheet-hatch-style-source-field-undecoded\"}");
    output.push_str(",\"renderPromotionContribution\":\"merged-empty-answer-area-perimeter-candidate\",\"renderPromotionBlockedReason\":\"answer-sheet-reference-frame-coordinates-not-decoded\"}");
}

pub(crate) fn push_answer_sheet_triangle_placement_candidate_json(
    output: &mut String,
    candidate: &SuccessDataTestAnswerSheetTrianglePlacementCandidate,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(candidate.source));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false");
    output.push_str(",\"placementBasis\":");
    output.push_str(&json_string(candidate.placement_basis));
    output.push_str(",\"sourceBbox\":");
    push_object_fdm_index_bbox_json(output, candidate.source_bbox);
    output.push_str(",\"coordinateSpace\":\"pageCssPx\",\"vertices\":{\"a\":");
    push_success_data_test_answer_sheet_point_json(output, candidate.a);
    output.push_str(",\"b\":");
    push_success_data_test_answer_sheet_point_json(output, candidate.b);
    output.push_str(",\"c\":");
    push_success_data_test_answer_sheet_point_json(output, candidate.c);
    output.push_str("},\"rightAngle\":{\"start\":");
    push_success_data_test_answer_sheet_point_json(output, candidate.right_angle_start);
    output.push_str(",\"corner\":");
    push_success_data_test_answer_sheet_point_json(output, candidate.right_angle_corner);
    output.push_str(",\"end\":");
    push_success_data_test_answer_sheet_point_json(output, candidate.right_angle_end);
    output.push_str("},\"labelAnchors\":[");
    for (index, anchor) in candidate.label_anchors.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        output.push_str(&json_string(anchor.text));
        output.push_str(",\"markerOffset\":");
        output.push_str(&anchor.marker_offset.to_string());
        output.push_str(",\"indexOffset\":");
        output.push_str(&anchor.index_offset.to_string());
        output.push_str(",\"point\":");
        push_success_data_test_answer_sheet_point_json(output, anchor.point);
        output.push('}');
    }
    output.push_str("],\"renderPromotionContribution\":\"triangle-rendered-from-projected-fdm-label-slots\",\"renderPromotionBlockedReason\":\"fdmtext-source-to-sheet-transform-undecoded\"}");
}

pub(crate) fn bbox_axis_gap(
    left_start: f32,
    left_end: f32,
    right_start: f32,
    right_end: f32,
) -> f32 {
    if left_end < right_start {
        right_start - left_end
    } else if right_end < left_start {
        left_start - right_end
    } else {
        0.0
    }
}

pub(crate) fn projected_bbox_viewport_coverage_ratio(
    layout: PageLayout,
    width: f32,
    height: f32,
) -> f32 {
    let viewport = fdm_projection_viewport(layout);
    let viewport_area = viewport.width * viewport.height;
    if viewport_area <= 0.0 {
        return 0.0;
    }
    ((width.max(0.0) * height.max(0.0)) / viewport_area).clamp(0.0, 1.0)
}

pub(crate) fn ratio_to_ppm(value: f32) -> u32 {
    (value.clamp(0.0, 1.0) * 1_000_000.0).round() as u32
}

pub(crate) fn push_ratio_ppm_json(output: &mut String, ratio_ppm: u32) {
    output.push_str(&format!("{:.6}", ratio_ppm as f32 / 1_000_000.0));
}

pub(crate) fn push_bbox_tuple_json(output: &mut String, bbox: (f32, f32, f32, f32)) {
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        bbox.0, bbox.1, bbox.2, bbox.3
    ));
}

pub(crate) fn accumulate_usize_range(
    target_min: &mut Option<usize>,
    target_max: &mut Option<usize>,
    value: usize,
) {
    *target_min = Some((*target_min).map_or(value, |current| current.min(value)));
    *target_max = Some((*target_max).map_or(value, |current| current.max(value)));
}

pub(crate) fn accumulate_projected_bbox_union_milli(
    x_min_target: &mut Option<i32>,
    y_min_target: &mut Option<i32>,
    x_max_target: &mut Option<i32>,
    y_max_target: &mut Option<i32>,
    bbox: (f32, f32, f32, f32),
) {
    let (x, y, width, height) = bbox;
    if !x.is_finite() || !y.is_finite() || !width.is_finite() || !height.is_finite() {
        return;
    }
    let x_min = (x * 1000.0).round() as i32;
    let y_min = (y * 1000.0).round() as i32;
    let x_max = ((x + width.max(0.0)) * 1000.0).round() as i32;
    let y_max = ((y + height.max(0.0)) * 1000.0).round() as i32;
    *x_min_target = Some((*x_min_target).map_or(x_min, |current| current.min(x_min)));
    *y_min_target = Some((*y_min_target).map_or(y_min, |current| current.min(y_min)));
    *x_max_target = Some((*x_max_target).map_or(x_max, |current| current.max(x_max)));
    *y_max_target = Some((*y_max_target).map_or(y_max, |current| current.max(y_max)));
}

pub(crate) fn update_optional_usize_min_max(
    min: &mut Option<usize>,
    max: &mut Option<usize>,
    value: usize,
) {
    *min = Some(min.map_or(value, |current| current.min(value)));
    *max = Some(max.map_or(value, |current| current.max(value)));
}

pub(crate) fn bbox_tuple_union(
    current: Option<(f32, f32, f32, f32)>,
    next: (f32, f32, f32, f32),
) -> Option<(f32, f32, f32, f32)> {
    let next_right = next.0 + next.2;
    let next_bottom = next.1 + next.3;
    match current {
        Some((left, top, width, height)) => {
            let right = left + width;
            let bottom = top + height;
            let union_left = left.min(next.0);
            let union_top = top.min(next.1);
            let union_right = right.max(next_right);
            let union_bottom = bottom.max(next_bottom);
            Some((
                union_left,
                union_top,
                (union_right - union_left).max(0.0),
                (union_bottom - union_top).max(0.0),
            ))
        }
        None => Some(next),
    }
}

pub(crate) type ShanaiLanGroupHeaderFamilyCounts =
    BTreeMap<(String, String, &'static str, &'static str), (usize, Vec<String>)>;

pub(crate) fn push_shanai_lan_group_header_family_counts_json(
    output: &mut String,
    counts: &ShanaiLanGroupHeaderFamilyCounts,
) {
    output.push('[');
    for (index, ((control_kind, first_field, basis, fill_color), (count, examples))) in
        counts.iter().enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"controlKindHex\":");
        output.push_str(&json_string(control_kind));
        output.push_str(",\"firstFieldWordHex\":");
        output.push_str(&json_string(first_field));
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

#[derive(Default)]
pub(crate) struct ShanaiLanFragmentParentRunFillMix {
    pub(crate) slot_count: usize,
    pub(crate) source_property_fill_color_slot_count: usize,
    pub(crate) default_fill_color_slot_count: usize,
    pub(crate) fill_color_basis: BTreeSet<&'static str>,
    pub(crate) fill_colors: BTreeSet<&'static str>,
    pub(crate) example_texts: Vec<String>,
}

pub(crate) fn push_json_string_array(output: &mut String, values: &[String]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
}

pub(crate) fn usize_values_are_contiguous(values: &[usize]) -> bool {
    values.len() > 1 && values.windows(2).all(|window| window[1] == window[0] + 1)
}

pub(crate) fn push_static_str_count_map_json(
    output: &mut String,
    counts: &BTreeMap<&'static str, usize>,
) {
    output.push('[');
    for (index, (key, count)) in counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"key\":");
        output.push_str(&json_string(key));
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_string_count_map_json(
    output: &mut String,
    counts: &BTreeMap<String, usize>,
    key: &str,
) {
    output.push('[');
    for (index, (value, count)) in counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push('{');
        output.push_str(&json_string(key));
        output.push(':');
        output.push_str(&json_string(value));
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_usize_count_map_json(output: &mut String, counts: &BTreeMap<usize, usize>) {
    output.push('[');
    for (index, (key, count)) in counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"groupIndex\":");
        output.push_str(&key.to_string());
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn distance_from_point_to_bbox(x: f32, y: f32, bbox: (f32, f32, f32, f32)) -> f32 {
    let (left, top, width, height) = bbox;
    let right = left + width;
    let bottom = top + height;
    let dx = if x < left {
        left - x
    } else if x > right {
        x - right
    } else {
        0.0
    };
    let dy = if y < top {
        top - y
    } else if y > bottom {
        y - bottom
    } else {
        0.0
    };
    dx.hypot(dy)
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestLineMarkPageGridCandidate {
    pub(crate) record_index: usize,
    pub(crate) page_mark_entry_index: usize,
    pub(crate) page_index_candidate: Option<usize>,
    pub(crate) page_line_start: usize,
    pub(crate) page_line_end: usize,
    pub(crate) line_offset_from_page_start: usize,
    pub(crate) row_height: f32,
    pub(crate) row_height_basis: &'static str,
    pub(crate) row_top_y: f32,
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestSourceTextPlacementCandidate {
    pub(crate) line_grid: SuccessDataTestLineMarkPageGridCandidate,
    pub(crate) font_size: f32,
    pub(crate) top_y: f32,
    pub(crate) baseline_y: f32,
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestTextPlacementResidualEntry {
    pub(crate) role: &'static str,
    pub(crate) text: String,
    pub(crate) record_index: usize,
    pub(crate) flag_word: Option<u16>,
    pub(crate) font_size: f32,
    pub(crate) reference_top_y: f32,
    pub(crate) reference_baseline_y: f32,
    pub(crate) source_top_y: f32,
    pub(crate) source_baseline_y: f32,
    pub(crate) top_residual_px: f32,
    pub(crate) baseline_residual_px: f32,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) line_header: Option<ShanaiLanLineHeader>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SuccessDataTestTextPlacementResidualBucketKey {
    pub(crate) top_residual_tenths: i32,
    pub(crate) baseline_residual_tenths: i32,
    pub(crate) flag_word: Option<u16>,
    pub(crate) font_size_tenths: i32,
    pub(crate) line_header_present: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestTextPlacementResidualBucket {
    pub(crate) count: usize,
    pub(crate) record_indexes: Vec<usize>,
    pub(crate) roles: BTreeMap<&'static str, usize>,
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestTextPlacementLinePitchFit {
    pub(crate) basis: &'static str,
    pub(crate) entry_count: usize,
    pub(crate) record_start: usize,
    pub(crate) record_end: usize,
    pub(crate) intercept: f32,
    pub(crate) pitch: f32,
    pub(crate) rms_residual_px: f32,
    pub(crate) max_abs_residual_px: f32,
    pub(crate) source_row_height_px: Option<f32>,
    pub(crate) source_row_height_minus_fit_pitch_px: Option<f32>,
}

pub(crate) fn push_optional_field_ratio_json(
    output: &mut String,
    value: Option<u16>,
    divisor: Option<u32>,
) {
    match (value, divisor) {
        (Some(value), Some(divisor)) if divisor > 0 => {
            output.push_str(&format!("{:.3}", f32::from(value) / divisor as f32));
        }
        _ => output.push_str("null"),
    }
}

pub(crate) fn push_optional_bool_json(output: &mut String, value: Option<bool>) {
    match value {
        Some(true) => output.push_str("true"),
        Some(false) => output.push_str("false"),
        None => output.push_str("null"),
    }
}

pub(crate) fn residual_tenths(value: f32) -> i32 {
    (value * 10.0).round() as i32
}

pub(crate) fn residual_tenths_string(tenths: i32) -> String {
    format!("{:.1}", tenths as f32 / 10.0)
}

pub(crate) fn source_range_json(start: usize, end: usize) -> String {
    format!("{{\"start\":{start},\"end\":{end}}}")
}

pub(crate) fn source_span_for_char_range(
    text: &str,
    source_span: &TextSourceSpan,
    start_chars: usize,
    end_chars: usize,
) -> TextSourceSpan {
    let start_units = utf16_units_before_chars(text, start_chars);
    let end_units = utf16_units_before_chars(text, end_chars);
    source_span.subspan_by_units(start_units, end_units)
}

pub(crate) fn utf16_units_before_chars(text: &str, chars: usize) -> usize {
    text.chars().take(chars).map(char::len_utf16).sum::<usize>()
}

pub(crate) fn push_f64_array_json(output: &mut String, values: &[f64]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{value:.3}"));
    }
    output.push(']');
}

pub(crate) fn style_stream_record_count(document: &Document, stream_name: &str) -> usize {
    document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(stream_name))
        .map(|style| summarize_style_stream(style.payload()).records().len())
        .unwrap_or_default()
}

pub(crate) fn document_view_style_group_count(document: &Document) -> usize {
    let Some(style) = document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))
    else {
        return 0;
    };

    summarize_style_stream(style.payload())
        .records()
        .iter()
        .filter_map(|record| document_view_style_group_id(record.code()))
        .collect::<BTreeSet<_>>()
        .len()
}

pub(crate) fn document_view_style_group_id(code: u16) -> Option<u16> {
    let group_id = code >> 8;
    let record_kind = code & 0x00ff;
    ((0x31..=0x39).contains(&group_id) && (0x04..=0x07).contains(&record_kind))
        .then(|| group_id - 0x30)
}

pub(crate) fn utf16le_ascii_contains(bytes: &[u8], needle: &str) -> bool {
    let mut encoded = Vec::with_capacity(needle.len() * 2);
    for unit in needle.encode_utf16() {
        encoded.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.windows(encoded.len()).any(|window| window == encoded)
}

pub(crate) fn raw_stream_bytes<'a>(document: &'a Document, name: &str) -> Option<&'a [u8]> {
    document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == name)
        .map(RawStream::bytes)
}

pub(crate) fn frame_record_unit_to_css_px(value: u16) -> f32 {
    value as f32 * FRAME_RECORD_UNIT_TO_CSS_PX
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageMarkSeparatorCandidate {
    pub(crate) record_offset: usize,
    pub(crate) record_index: u32,
    pub(crate) line_start: u32,
    pub(crate) line_end: u32,
    pub(crate) y_centipoints: u16,
    pub(crate) advance_centipoints: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageMarkRecordHeader {
    pub(crate) offset: usize,
    pub(crate) index: u32,
    pub(crate) flags: u32,
    pub(crate) line_start: u32,
    pub(crate) line_end: u32,
}

pub(crate) fn document_visible_text(document: &Document) -> String {
    document_paragraph_texts(document)
        .into_iter()
        .map(|(_, text)| text)
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn utf16_units_for_chars(characters: &[char]) -> usize {
    characters
        .iter()
        .map(|character| character.len_utf16())
        .sum()
}

pub(crate) fn leading_display_units(text: &str) -> usize {
    text.chars()
        .take_while(|character| matches!(character, ' ' | '\u{3000}'))
        .map(display_column_width)
        .sum()
}
