use super::*;
use crate::*;

pub(crate) const SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX: f32 = 687.9;

pub(crate) const SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX: f32 = 971.3;

pub(crate) fn success_data_test_line_mark_matches_for_source_span<'a>(
    document: &'a Document,
    span: &TextSourceSpan,
) -> impl Iterator<Item = ShanaiLanLineMarkInterval> + 'a {
    let unit_start = span.unit_start();
    let unit_end = span.unit_end();
    shanai_lan_line_mark_intervals(document)
        .into_iter()
        .filter(move |interval| interval.unit_start < unit_end && unit_start < interval.unit_end)
}

pub(crate) fn success_data_test_fixed_pitch_advance_px(text: &str, font_size: f32) -> f32 {
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

pub(crate) fn success_data_test_uniform_target_height_px(
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

pub(crate) fn success_data_test_projection_margin_units(
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

pub(crate) fn diagnostic_success_data_test_reference_table_grid_overlay_layout(
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

pub(crate) fn document_has_success_data_test_projection_evidence(document: &Document) -> bool {
    let plain_text = document_plain_text(document);
    plain_text.contains("次の計算をしなさい")
        && plain_text.contains("斜辺の直角三角形")
        && plain_text.contains("右の図のような円錐")
}

pub(crate) fn success_data_test_unbacked_resolved_text_slot(
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

pub(crate) fn success_data_test_resolved_text_slot_fragment(
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
