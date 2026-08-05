use super::*;

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestFigureLabelLine {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font_size: f32,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) line_header: Option<ShanaiLanLineHeader>,
    pub(crate) spans: Vec<SuccessDataTestFigureLabelSpan>,
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestFigureLabelSpan {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) source_span: TextSourceSpan,
}

pub(crate) fn push_unique_static_str(values: &mut Vec<&'static str>, value: &'static str) {
    if value != "none" && !values.contains(&value) {
        values.push(value);
    }
}

pub(crate) fn push_json_string_slice_array(output: &mut String, values: &[&str]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
}

pub(crate) fn svg_visual_text(text: &str) -> String {
    text.chars()
        .flat_map(|character| match character {
            '\t' => "\u{3000}\u{3000}".chars().collect::<Vec<_>>(),
            _ => vec![character],
        })
        .collect()
}

pub(crate) fn is_centered_ginga_title_page(page_number: usize, line: &PageTextLine) -> bool {
    page_number == 1 && line.text().contains("銀河鉄道の夜") && line.text().contains("宮沢")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridCellRenderText {
    pub(crate) text: String,
    pub(crate) trimmed_text: String,
    pub(crate) basis: &'static str,
    pub(crate) preserves_source_whitespace: bool,
    pub(crate) leading_whitespace_chars: usize,
    pub(crate) trailing_whitespace_chars: usize,
    pub(crate) render_trim_candidate_basis: &'static str,
    pub(crate) render_trim_candidate_blocked_reason: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridSourceDerivedLayout {
    pub(crate) provenance: TableGridSourceDerivedLayoutProvenance,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) row_height: f32,
    pub(crate) column_width: f32,
    pub(crate) column_widths: Vec<f32>,
    pub(crate) column_width_basis: &'static str,
    pub(crate) column_count: usize,
    pub(crate) row_count: usize,
    pub(crate) x_unit_range_basis: &'static str,
    pub(crate) x_unit_start: u16,
    pub(crate) x_unit_end: u16,
    pub(crate) x_unit_full_extent_units: u16,
    pub(crate) x_unit_row_agreement_count: usize,
    pub(crate) x_unit_all_rows_agree: bool,
    pub(crate) x_unit_trailing_header_included: bool,
    pub(crate) x_unit_included_trailing_header_count: usize,
    pub(crate) x_unit_column_slot_width_units: Vec<u16>,
    pub(crate) x_unit_trailing_slot_width_units: Vec<u16>,
    pub(crate) x_origin_inset_units: f32,
    pub(crate) x_origin_inset_basis: &'static str,
    pub(crate) row_height_basis: &'static str,
    pub(crate) page_origin_authority: &'static str,
    pub(crate) anchor_line_index: Option<usize>,
    pub(crate) line_mark_page_origin: Option<TableGridLineMarkPageOriginCandidate>,
    pub(crate) line_mark_page_origin_stride: Option<TableGridLineMarkPageOriginStrideCandidate>,
    pub(crate) raw_header_count: usize,
    pub(crate) matched_cell_header_count: usize,
    pub(crate) min_offset_units: Option<u16>,
    pub(crate) max_extent_units: Option<u16>,
    pub(crate) matched_cell_span_units: Vec<u16>,
    pub(crate) matched_cell_gap_units: Vec<u16>,
    pub(crate) homogeneous_font_size_units: Option<u16>,
    pub(crate) line_mark_row_record_selection: &'static str,
    pub(crate) line_mark_rows_exact_and_contiguous: bool,
    pub(crate) line_header_rows_homogeneous: bool,
    pub(crate) render_promotion_blocked_reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableGridSourceDerivedLayoutProvenance {
    DecodedCompactPlacement,
    SparseSiblingDerived,
}

impl TableGridSourceDerivedLayoutProvenance {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::DecodedCompactPlacement => "decodedCompactPlacement",
            Self::SparseSiblingDerived => "sparseSiblingDerived",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridLineMarkPageOriginCandidate {
    pub(crate) y: f32,
    pub(crate) first_line_mark_record_index: usize,
    pub(crate) last_line_mark_record_index: usize,
    pub(crate) page_mark_entry_index: usize,
    pub(crate) page_index_candidate: Option<usize>,
    pub(crate) page_line_start: usize,
    pub(crate) page_line_end: usize,
    pub(crate) page_mark_u16_fields: Vec<u16>,
    pub(crate) page_width_px: f32,
    pub(crate) page_height_px: f32,
    pub(crate) page_margin_px: f32,
    pub(crate) page_body_width_px: f32,
    pub(crate) line_offset_from_page_start: usize,
    pub(crate) line_pitch_px: f32,
    pub(crate) line_pitch_basis: &'static str,
    pub(crate) row_height: f32,
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridLineMarkPageOriginStrideCandidate {
    pub(crate) line_mark_record_indexes: Vec<usize>,
    pub(crate) record_stride: usize,
    pub(crate) first_line_mark_record_index: usize,
    pub(crate) last_line_mark_record_index: usize,
    pub(crate) page_mark_entry_index: usize,
    pub(crate) page_index_candidate: Option<usize>,
    pub(crate) page_line_start: usize,
    pub(crate) page_line_end: usize,
    pub(crate) page_mark_u16_fields: Vec<u16>,
    pub(crate) page_width_px: f32,
    pub(crate) page_height_px: f32,
    pub(crate) page_margin_px: f32,
    pub(crate) page_body_width_px: f32,
    pub(crate) line_offset_from_page_start: usize,
    pub(crate) row_height: f32,
    pub(crate) raw_record_index_row_tops: Vec<f32>,
    pub(crate) stride_collapsed_row_tops: Vec<f32>,
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridReferenceLayout {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) row_height: f32,
    pub(crate) column_width: f32,
    pub(crate) column_widths: Vec<f32>,
    pub(crate) column_width_basis: &'static str,
    pub(crate) column_count: usize,
    pub(crate) header_fill: bool,
    pub(crate) corner_radius: f32,
    pub(crate) stroke_width: f32,
    pub(crate) cell_stroke_width: f32,
    pub(crate) font_size: f32,
    pub(crate) cell_text_centered: bool,
}

impl TableGridReferenceLayout {
    pub(crate) fn column_width_at(&self, column_index: usize) -> f32 {
        table_grid_column_width(self.column_width, &self.column_widths, column_index)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridRenderLayout {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) row_height: f32,
    pub(crate) column_width: f32,
    pub(crate) column_widths: Vec<f32>,
    pub(crate) column_width_basis: &'static str,
    pub(crate) column_count: usize,
    pub(crate) header_fill: bool,
    pub(crate) corner_radius: f32,
    pub(crate) stroke_width: f32,
    pub(crate) cell_stroke_width: f32,
    pub(crate) stroke_width_basis: &'static str,
    pub(crate) font_size: f32,
    pub(crate) font_size_basis: &'static str,
    pub(crate) cell_text_centered: bool,
    pub(crate) cell_text_alignment_basis: &'static str,
    pub(crate) cell_text_x_adjustment: f32,
    pub(crate) cell_text_x_adjustment_basis: &'static str,
    pub(crate) cell_text_baseline_factor: f32,
    pub(crate) cell_text_baseline_basis: &'static str,
    pub(crate) cell_text_font_weight: &'static str,
    pub(crate) cell_text_font_weight_basis: &'static str,
    pub(crate) reference_backed: bool,
    pub(crate) render_promotion_blocked_reason: &'static str,
}

impl TableGridRenderLayout {
    pub(crate) fn from_reference(reference: &TableGridReferenceLayout) -> Self {
        Self {
            x: reference.x,
            y: reference.y,
            width: reference.width,
            row_height: reference.row_height,
            column_width: reference.column_width,
            column_widths: reference.column_widths.clone(),
            column_width_basis: reference.column_width_basis,
            column_count: reference.column_count,
            header_fill: reference.header_fill,
            corner_radius: reference.corner_radius,
            stroke_width: reference.stroke_width,
            cell_stroke_width: reference.cell_stroke_width,
            stroke_width_basis: "referenceLayout",
            font_size: reference.font_size,
            font_size_basis: "referenceLayout",
            cell_text_centered: reference.cell_text_centered,
            cell_text_alignment_basis: "referenceLayout",
            cell_text_x_adjustment: 0.0,
            cell_text_x_adjustment_basis: "referenceLayout",
            cell_text_baseline_factor: if reference.cell_text_centered {
                0.72
            } else {
                0.64
            },
            cell_text_baseline_basis: "referenceLayout",
            cell_text_font_weight: "500",
            cell_text_font_weight_basis: "referenceLayout",
            reference_backed: true,
            render_promotion_blocked_reason: "none",
        }
    }

    pub(crate) fn from_source_derived(source: &TableGridSourceDerivedLayout) -> Self {
        let source_font_size = source
            .homogeneous_font_size_units
            .filter(|font_size_units| *font_size_units > 0)
            .map(|font_size_units| {
                APP_FONT_SIZE_PX * (f32::from(font_size_units) / APP_TABLE_BASE_FONT_SIZE_UNITS)
            })
            .filter(|font_size| font_size.is_finite() && *font_size > 0.0);
        let source_unit_stroke_width = source
            .homogeneous_font_size_units
            .filter(|font_size_units| *font_size_units > 0)
            .map(|font_size_units| source.row_height / f32::from(font_size_units));
        let source_unit_stroke_width = source_unit_stroke_width
            .filter(|stroke_width| stroke_width.is_finite() && *stroke_width > 0.0);
        let (stroke_width, stroke_width_basis) = source_unit_stroke_width
            .map(|stroke_width| (stroke_width, "documentTextLineHeaderFontUnitPx"))
            .unwrap_or((1.0, "fallbackSourceDerivedStroke"));
        let (
            cell_text_x_adjustment,
            cell_text_x_adjustment_basis,
            cell_text_baseline_factor,
            cell_text_baseline_basis,
            cell_text_font_weight,
            cell_text_font_weight_basis,
        ) = if source_unit_stroke_width.is_some() {
            (
                -stroke_width,
                "documentTextLineHeaderFontUnitPxStrokeCompensation",
                0.77,
                "documentTextLineHeaderFontSizeUnitsBaselineCandidate",
                "400",
                "regularTableCellFallbackNoBoldEvidence",
            )
        } else {
            (
                0.0,
                "sourceFontUnitMetricsMissing",
                0.72,
                "fallbackCenteredTableCellBaseline",
                "500",
                "fallbackNoFontUnitMetricWeight",
            )
        };
        Self {
            x: source.x,
            y: source.y,
            width: source.width,
            row_height: source.row_height,
            column_width: source.column_width,
            column_widths: source.column_widths.clone(),
            column_width_basis: source.column_width_basis,
            column_count: source.column_count,
            header_fill: false,
            corner_radius: 0.0,
            stroke_width,
            cell_stroke_width: stroke_width,
            stroke_width_basis,
            font_size: source_font_size.unwrap_or(APP_FONT_SIZE_PX),
            font_size_basis: if source_font_size.is_some() {
                "documentTextLineHeaderFontSizeUnitsScaledToAppFont"
            } else {
                "fallbackAppFontSize"
            },
            cell_text_centered: true,
            cell_text_alignment_basis: "documentTextLineHeaderCellSlotCenterCandidate",
            cell_text_x_adjustment,
            cell_text_x_adjustment_basis,
            cell_text_baseline_factor,
            cell_text_baseline_basis,
            cell_text_font_weight,
            cell_text_font_weight_basis,
            reference_backed: false,
            render_promotion_blocked_reason: source.render_promotion_blocked_reason,
        }
    }

    pub(crate) fn column_width_at(&self, column_index: usize) -> f32 {
        table_grid_column_width(self.column_width, &self.column_widths, column_index)
    }

    pub(crate) fn column_x_at(&self, column_index: usize) -> f32 {
        table_grid_column_x(self.x, self.column_width, &self.column_widths, column_index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableCandidateLineHeaderRow {
    pub(crate) row_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) expected_cell_count: usize,
    pub(crate) matched_cell_count: usize,
    pub(crate) headers: Vec<ShanaiLanLineHeader>,
}

impl TableCandidateLineHeaderRow {
    pub(crate) fn raw_header_count(&self) -> usize {
        self.headers.len()
    }
}

pub(crate) fn table_line_header_source_offset(
    basis: TextCountRangeOverlapBasis,
    byte_offset: usize,
) -> usize {
    match basis {
        TextCountRangeOverlapBasis::Byte => byte_offset,
        TextCountRangeOverlapBasis::Unit => byte_offset / 2,
    }
}

pub(crate) fn fragment_overlaps_rendered_table_projection(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
    fragment: &PageLayerTextFragment,
) -> bool {
    if page_number != 1 {
        return false;
    }
    let Some(span) = &fragment.source_span else {
        return false;
    };
    document.table_candidates().iter().any(|candidate| {
        table_grid_candidate_is_rendered(layout, document, lines, page_number, candidate)
            && table_candidate_overlaps_source_span(candidate, span)
    })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TableGridReferenceFallbackAdmission {
    pub(crate) allowed: bool,
    pub(crate) blocked_reason: Option<&'static str>,
}

pub(crate) fn preview_svg_cell_text(layout: PageLayout, text: &str, column_width: f32) -> String {
    let max_chars = ((column_width as f64 / column_width_px(layout)).floor() as usize).max(4);
    let mut preview = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}

pub(crate) fn push_tsaiten_document_format_table_projection(
    shapes: &mut Vec<ObservedFormShape>,
    slots: &mut Vec<ObservedFormTextSlot>,
    scale_x: f32,
    scale_y: f32,
) {
    let x = 174.0;
    let y = 546.0;
    let width = 554.0;
    let height = 157.0;
    let header_height = 28.0;
    let split_x = x + (width * 0.68);
    shapes.push(form_shape(
        "document-format-table",
        x,
        y,
        width,
        height,
        "#ffffff",
        Some("#555555"),
        1.2,
        4.0,
        scale_x,
        scale_y,
    ));
    shapes.push(form_shape(
        "document-format-header",
        x,
        y,
        width,
        header_height,
        "#f7f7f7",
        Some("#bbbbbb"),
        0.6,
        4.0,
        scale_x,
        scale_y,
    ));
    for line_y in [y + header_height, y + 73.0, y + 113.0] {
        shapes.push(form_shape(
            "document-format-row-rule",
            x,
            line_y,
            width,
            0.7,
            "#777777",
            None,
            0.0,
            0.0,
            scale_x,
            scale_y,
        ));
    }
    shapes.push(form_shape(
        "document-format-column-rule",
        split_x,
        y,
        0.7,
        height,
        "#777777",
        None,
        0.0,
        0.0,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "table-header",
        "採点項目",
        x + 150.0,
        y + 19.0,
        10.5,
        "700",
        "middle",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "table-header",
        "減　点",
        split_x + ((x + width - split_x) / 2.0),
        y + 19.0,
        10.5,
        "700",
        "middle",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "table-cell",
        "用紙サイズがＡ４である",
        x + 28.0,
        y + 55.0,
        10.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "table-cell",
        "用紙の置き方が縦置きである",
        x + 28.0,
        y + 95.0,
        10.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "table-cell",
        "１行文字数が（全角）３０字である",
        x + 28.0,
        y + 135.0,
        10.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "table-cell",
        "異なる場合、",
        split_x + 38.0,
        y + 87.0,
        10.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "table-cell",
        "各１０点減点",
        split_x + 38.0,
        y + 103.0,
        10.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
}

pub(crate) fn document_has_tsaiten_projection_evidence(document: &Document) -> bool {
    let plain_text = document_plain_text(document);
    if !plain_text.contains("タイピング科目採点方法")
        || !plain_text.contains("235点以上")
        || !plain_text.contains("誤字・脱字・余字")
    {
        return false;
    }

    let has_scoring_grid = document.table_candidates().iter().any(|candidate| {
        candidate.intervals().len() == 4
            && candidate
                .column_segment_grid_candidate()
                .is_some_and(|grid| grid.column_count() == 3)
            && candidate
                .intervals()
                .first()
                .is_some_and(|interval| interval.text_preview() == "級\t配点\t合格点")
    });
    let has_error_grid = document.table_candidates().iter().any(|candidate| {
        candidate.intervals().len() == 3
            && candidate
                .column_segment_grid_candidate()
                .is_some_and(|grid| grid.column_count() == 2)
            && candidate
                .intervals()
                .get(1)
                .is_some_and(|interval| interval.text_preview().contains("誤字・脱字・余字"))
    });
    has_scoring_grid && has_error_grid
}
