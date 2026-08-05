use super::*;

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestTextSourceMatch {
    pub(crate) source_span: TextSourceSpan,
    pub(crate) line_header: Option<ShanaiLanLineHeader>,
}

pub(crate) fn byte_index_after_utf16_units(text: &str, target_units: usize) -> Option<usize> {
    if target_units == 0 {
        return Some(0);
    }
    let mut units = 0usize;
    for (byte_index, character) in text.char_indices() {
        if units >= target_units {
            return Some(byte_index);
        }
        units += character.len_utf16();
    }
    (units >= target_units).then_some(text.len())
}

#[derive(Debug, Clone)]
pub(crate) struct SuccessDataTestAnswerSheetTextSlot {
    pub(crate) text: String,
    pub(crate) source_token_index: usize,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) font_size: f32,
    pub(crate) anchor: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestAnswerSheetTextSlotTemplate {
    pub(crate) source_token_index: usize,
    pub(crate) x_pt: f32,
    pub(crate) y_pt: f32,
    pub(crate) font_pt: f32,
    pub(crate) anchor: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestAnswerSheetPoint {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestAnswerSheetTriangleLabelAnchor {
    pub(crate) text: &'static str,
    pub(crate) point: SuccessDataTestAnswerSheetPoint,
    pub(crate) marker_offset: usize,
    pub(crate) index_offset: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestAnswerSheetTrianglePlacementCandidate {
    pub(crate) source: &'static str,
    pub(crate) placement_basis: &'static str,
    pub(crate) source_bbox: ObjectFdmIndexBbox,
    pub(crate) a: SuccessDataTestAnswerSheetPoint,
    pub(crate) b: SuccessDataTestAnswerSheetPoint,
    pub(crate) c: SuccessDataTestAnswerSheetPoint,
    pub(crate) right_angle_start: SuccessDataTestAnswerSheetPoint,
    pub(crate) right_angle_corner: SuccessDataTestAnswerSheetPoint,
    pub(crate) right_angle_end: SuccessDataTestAnswerSheetPoint,
    pub(crate) label_anchors: [SuccessDataTestAnswerSheetTriangleLabelAnchor; 3],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestAnswerSheetFrame {
    pub(crate) layout: PageLayout,
    pub(crate) left_pt: f32,
    pub(crate) top_pt: f32,
    pub(crate) right_pt: f32,
    pub(crate) bottom_pt: f32,
}

impl SuccessDataTestAnswerSheetFrame {
    pub(crate) fn new(layout: PageLayout) -> Self {
        Self {
            layout,
            left_pt: SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_LEFT_PT,
            top_pt: SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_TOP_PT,
            right_pt: SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_RIGHT_PT,
            bottom_pt: SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_BOTTOM_PT,
        }
    }

    pub(crate) fn page_x(self, x_pt: f32) -> f32 {
        x_pt * PDF_POINT_TO_CSS_PX * self.layout.width_px()
            / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX
    }

    pub(crate) fn page_y(self, y_pt: f32) -> f32 {
        y_pt * PDF_POINT_TO_CSS_PX * self.layout.height_px()
            / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
    }

    pub(crate) fn sheet_x(self, x_pt: f32) -> f32 {
        self.page_x(self.left_pt + x_pt)
    }

    pub(crate) fn sheet_y(self, y_pt: f32) -> f32 {
        self.page_y(self.top_pt + y_pt)
    }

    pub(crate) fn width_pt(self) -> f32 {
        self.right_pt - self.left_pt
    }

    pub(crate) fn height_pt(self) -> f32 {
        self.bottom_pt - self.top_pt
    }

    pub(crate) fn bbox(self) -> (f32, f32, f32, f32) {
        let left = self.page_x(self.left_pt);
        let top = self.page_y(self.top_pt);
        let right = self.page_x(self.right_pt);
        let bottom = self.page_y(self.bottom_pt);
        (left, top, right - left, bottom - top)
    }

    pub(crate) fn stroke_width(self, width_pt: f32) -> f32 {
        width_pt * PDF_POINT_TO_CSS_PX * self.layout.width_px()
            / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX
    }

    pub(crate) fn font_size(self, font_pt: f32) -> f32 {
        font_pt * PDF_POINT_TO_CSS_PX * self.layout.height_px()
            / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestProjectedPathBBox {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuccessDataTestTitleArtHorizontalPlacement {
    pub(crate) frame_x: f32,
    pub(crate) path_x: f32,
    pub(crate) candidate_frame_x: f32,
    pub(crate) candidate_path_x: f32,
    pub(crate) content_left_adjustment: f32,
    pub(crate) stroke_outer_adjustment: f32,
    pub(crate) content_left_only_x: f32,
    pub(crate) frame_record_x: f32,
    pub(crate) basis: &'static str,
    pub(crate) render_promoted: bool,
    pub(crate) stroke_width_candidate: Option<u32>,
}

pub(crate) fn cubic_bezier_point(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    t: f32,
) -> (f32, f32) {
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;
    (
        mt2 * mt * p0.0 + 3.0 * mt2 * t * p1.0 + 3.0 * mt * t2 * p2.0 + t2 * t * p3.0,
        mt2 * mt * p0.1 + 3.0 * mt2 * t * p1.1 + 3.0 * mt * t2 * p2.1 + t2 * t * p3.1,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn form_slot(
    role: &'static str,
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    font_weight: &'static str,
    anchor: &'static str,
    font_family: &'static str,
    scale_x: f32,
    scale_y: f32,
) -> ObservedFormTextSlot {
    ObservedFormTextSlot {
        role,
        text: text.to_string(),
        x: x * scale_x,
        y: y * scale_y,
        font_size,
        font_weight,
        anchor,
        font_family,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn form_shape(
    role: &'static str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fill: &'static str,
    stroke: Option<&'static str>,
    stroke_width: f32,
    rx: f32,
    scale_x: f32,
    scale_y: f32,
) -> ObservedFormShape {
    ObservedFormShape {
        role,
        x: x * scale_x,
        y: y * scale_y,
        width: width * scale_x,
        height: height * scale_y,
        fill,
        stroke,
        stroke_width,
        rx: rx * scale_x.min(scale_y),
    }
}

pub(crate) fn escape_xml(text: &str) -> String {
    let mut escaped = String::new();
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(character),
        }
    }
    escaped
}
