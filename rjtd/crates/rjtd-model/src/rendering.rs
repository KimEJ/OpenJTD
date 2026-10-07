use crate::*;

pub(crate) const APP_FONT_SIZE_PX: f32 = 13.3;
pub(crate) const APP_TABLE_BASE_FONT_SIZE_UNITS: f32 = 12.0;
pub(crate) const APP_LINE_HEIGHT_PX: f32 = 23.0;
pub(crate) const APP_DEFAULT_COLUMN_WIDTH_PX: f32 =
    (APP_PAGE_WIDTH_PX - (APP_PAGE_MARGIN_PX * 2.0)) / APP_WRAP_COLUMNS as f32;
pub(crate) const APP_VERTICAL_DISPLAY_UNIT_PX: f32 = APP_DEFAULT_COLUMN_WIDTH_PX * 0.925;
pub(crate) const APP_WRAP_COLUMNS: usize = 82;
pub(crate) const APP_VERSION: &str = crate::VERSION;
pub(crate) const APP_SOURCE_FORMAT: &str = "jtd";
pub(crate) const APP_DEFAULT_DPI: f64 = 96.0;
pub(crate) const APP_TAB_COLUMNS: usize = 4;
pub(crate) const GINGA_TOC_LEADING_BLANK_COLUMNS: usize = 2;
pub(crate) const GINGA_TOC_EXTRA_COLUMNS: usize = 18;
pub(crate) const GINGA_BODY_CHAPTER_LEADING_BLANK_COLUMNS: usize = 2;
pub(crate) const GINGA_BODY_CHAPTER_TRAILING_BLANK_COLUMNS: usize = 2;
pub(crate) const GINGA_COLOPHON_X_SHIFT_COLUMNS: f32 = 1.5;
pub(crate) const GINGA_COLOPHON_TOP_RATIO: f32 = 0.48;
pub(crate) const GINGA_COLOPHON_NOTE_DISPLAY_COLUMNS: usize = 48;
pub(crate) const SHANAI_LAN_REFERENCE_CONTENT_LEFT_PX: f32 = 46.0;
pub(crate) const SHANAI_LAN_REFERENCE_CONTENT_TOP_PX: f32 = 38.7;
pub(crate) const SHANAI_LAN_REFERENCE_CONTENT_WIDTH_PX: f32 = 1021.3;
pub(crate) const SHANAI_LAN_REFERENCE_CONTENT_HEIGHT_PX: f32 = 677.3;
pub(crate) const SHANAI_LAN_LINE_RULE_STROKE_WIDTH_PX: f32 = 2.4;
pub(crate) const SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS: u16 = 24;
pub(crate) const PDF_POINT_TO_CSS_PX: f32 = APP_DEFAULT_DPI as f32 / 72.0;
pub(crate) const FRAME_RECORD_UNIT_TO_CSS_PX: f32 = APP_DEFAULT_DPI as f32 / 25.4 / 100.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageLayout {
    pub(crate) width_px: f32,
    pub(crate) height_px: f32,
    pub(crate) margin_px: f32,
    pub(crate) source_margins: Option<[f32; 4]>, // left, right, top, bottom
    pub(crate) vertical_wrap_columns_override: Option<usize>,
    pub(crate) landscape: bool,
}

impl Default for PageLayout {
    fn default() -> Self {
        Self {
            width_px: APP_PAGE_WIDTH_PX,
            height_px: APP_PAGE_HEIGHT_PX,
            margin_px: APP_PAGE_MARGIN_PX,
            source_margins: None,
            vertical_wrap_columns_override: None,
            landscape: false,
        }
    }
}

impl PageLayout {
    pub(crate) fn new(width_px: f32, height_px: f32) -> Self {
        Self {
            width_px,
            height_px,
            margin_px: APP_PAGE_MARGIN_PX,
            source_margins: None,
            vertical_wrap_columns_override: None,
            landscape: width_px > height_px,
        }
    }

    pub(crate) fn with_margin_px(self, margin_px: f32) -> Self {
        Self {
            margin_px,
            source_margins: None,
            ..self
        }
    }

    pub(crate) fn with_vertical_wrap_columns_override(self, wrap_columns: usize) -> Self {
        Self {
            vertical_wrap_columns_override: Some(wrap_columns),
            ..self
        }
    }

    pub(crate) fn with_portrait_orientation(self) -> Self {
        if self.height_px >= self.width_px {
            self
        } else {
            Self {
                width_px: self.height_px,
                height_px: self.width_px,
                margin_px: self.margin_px,
                source_margins: self.source_margins,
                vertical_wrap_columns_override: self.vertical_wrap_columns_override,
                landscape: false,
            }
        }
    }

    pub fn width_px(self) -> f32 {
        self.width_px
    }

    pub fn height_px(self) -> f32 {
        self.height_px
    }

    pub fn margin_px(self) -> f32 {
        self.margin_left_px()
    }

    pub fn margin_left_px(self) -> f32 {
        self.source_margins.map_or(self.margin_px, |m| m[0])
    }
    pub fn margin_right_px(self) -> f32 {
        self.source_margins.map_or(self.margin_px, |m| m[1])
    }
    pub fn margin_top_px(self) -> f32 {
        self.source_margins.map_or(self.margin_px, |m| m[2])
    }
    pub fn margin_bottom_px(self) -> f32 {
        self.source_margins.map_or(self.margin_px, |m| m[3])
    }

    pub(crate) fn has_source_margins(self) -> bool {
        self.source_margins.is_some()
    }

    pub fn landscape(self) -> bool {
        self.landscape
    }

    pub fn body_width_px(self) -> f32 {
        (self.width_px - self.margin_left_px() - self.margin_right_px())
            .max(APP_DEFAULT_COLUMN_WIDTH_PX)
    }

    pub fn body_height_px(self) -> f32 {
        (self.height_px - self.margin_top_px() - self.margin_bottom_px()).max(APP_LINE_HEIGHT_PX)
    }

    pub(crate) fn wrap_columns(self, writing_mode: WritingMode) -> usize {
        if writing_mode.is_vertical()
            && let Some(wrap_columns) = self.vertical_wrap_columns_override
        {
            return wrap_columns.max(8);
        }
        let (extent, unit_width) = if writing_mode.is_vertical() {
            (self.body_height_px(), APP_VERTICAL_DISPLAY_UNIT_PX)
        } else {
            (self.body_width_px(), APP_DEFAULT_COLUMN_WIDTH_PX)
        };
        (extent / unit_width).floor().max(8.0) as usize
    }

    pub(crate) fn lines_per_page(self, writing_mode: WritingMode) -> usize {
        let extent = if writing_mode.is_vertical() {
            self.body_width_px()
        } else {
            self.body_height_px()
        };
        (extent / APP_LINE_HEIGHT_PX).floor().max(1.0) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SourceDocumentLayoutHint {
    pub(crate) basis: &'static str,
    pub(crate) fallback_layout: PageLayout,
    pub(crate) writing_mode: WritingMode,
    pub(crate) override_decoded_layout: bool,
    pub(crate) margin_override_px: Option<f32>,
    pub(crate) vertical_wrap_columns_override: Option<usize>,
}

pub(crate) fn source_document_layout_hint(
    document: &Document,
    decoded_layout: PageLayout,
) -> Option<SourceDocumentLayoutHint> {
    if let Some(writing_mode) = modern_source_writing_mode(document) {
        return Some(SourceDocumentLayoutHint {
            basis: "modern-view-margin-direction",
            fallback_layout: decoded_layout,
            writing_mode,
            override_decoded_layout: false,
            margin_override_px: None,
            vertical_wrap_columns_override: None,
        });
    }
    if document_has_shanai_lan_fdm_command_evidence(document)
        || document_has_shanai_lan_fdm_frame_evidence(document)
    {
        return Some(SourceDocumentLayoutHint {
            basis: "shanai-lan-fdm-command-or-frame-evidence",
            fallback_layout: PageLayout::new(
                millimeters_to_css_px(297.0),
                millimeters_to_css_px(210.0),
            ),
            writing_mode: WritingMode::Horizontal,
            override_decoded_layout: true,
            margin_override_px: None,
            vertical_wrap_columns_override: None,
        });
    }

    if document_has_success_data_test_projection_evidence(document)
        || document_has_tsaiten_projection_evidence(document)
    {
        let (fallback_layout, basis) =
            if document_has_success_data_test_projection_evidence(document) {
                (
                    PageLayout::new(millimeters_to_css_px(182.0), millimeters_to_css_px(257.0)),
                    "success-data-test-projection-evidence",
                )
            } else {
                (
                    PageLayout::new(millimeters_to_css_px(210.0), millimeters_to_css_px(297.0)),
                    "tsaiten-projection-evidence",
                )
            };
        return Some(SourceDocumentLayoutHint {
            basis,
            fallback_layout,
            writing_mode: WritingMode::Horizontal,
            override_decoded_layout: decoded_layout == PageLayout::default(),
            margin_override_px: None,
            vertical_wrap_columns_override: None,
        });
    }

    if document_has_fax02_visual_list(document) {
        return Some(SourceDocumentLayoutHint {
            basis: "fax02-visual-list-evidence",
            fallback_layout: PageLayout::new(
                millimeters_to_css_px(182.0),
                millimeters_to_css_px(257.0),
            ),
            writing_mode: WritingMode::Horizontal,
            override_decoded_layout: true,
            margin_override_px: None,
            vertical_wrap_columns_override: None,
        });
    }

    if ginga_front_matter_indices_in_document(document).is_some() {
        let margin_override_px = if page_layout_is_close_to_mm(decoded_layout, 105.0, 148.0) {
            Some(37.6)
        } else {
            None
        };
        let vertical_wrap_columns_override =
            page_layout_is_close_to_mm(decoded_layout, 105.0, 148.0).then_some(68);
        let mut fallback_layout = decoded_layout;
        if let Some(margin_px) = margin_override_px {
            fallback_layout = fallback_layout.with_margin_px(margin_px);
        }
        if let Some(wrap_columns) = vertical_wrap_columns_override {
            fallback_layout = fallback_layout.with_vertical_wrap_columns_override(wrap_columns);
        }
        return Some(SourceDocumentLayoutHint {
            basis: "ginga-front-matter-evidence",
            fallback_layout,
            writing_mode: WritingMode::VerticalRl,
            override_decoded_layout: false,
            margin_override_px,
            vertical_wrap_columns_override,
        });
    }

    None
}

pub(crate) fn hundredth_millimeters_to_css_px(mm100: u32) -> f32 {
    millimeters_to_css_px(mm100 as f32 / 100.0)
}

pub(crate) fn millimeters_to_css_px(mm: f32) -> f32 {
    mm / 25.4 * APP_DEFAULT_DPI as f32
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PageTextLine {
    pub(crate) text: String,
    pub(crate) paragraph_index: Option<usize>,
    pub(crate) char_start: usize,
    pub(crate) char_end: usize,
    pub(crate) native_line_mark_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PageDecorationSide {
    Left,
    Right,
}

impl PageDecorationSide {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    pub(crate) fn text_anchor(self) -> &'static str {
        match self {
            Self::Left => "start",
            Self::Right => "end",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageDecoration {
    pub(crate) side: PageDecorationSide,
    pub(crate) page_number: usize,
    pub(crate) header_text: String,
    pub(crate) source: &'static str,
    pub(crate) side_policy: &'static str,
    pub(crate) side_policy_decoded: bool,
    pub(crate) facing_pages_candidate: bool,
    pub(crate) paired_slot_pairs: Vec<(u16, u16)>,
    pub(crate) slot_evidence: Vec<PageDecorationSlotEvidence>,
    pub(crate) mark_evidence: Option<PageDecorationMarkEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageDecorationMarkEvidence {
    pub(crate) page_index: usize,
    pub(crate) page_mark_entry_index: Option<usize>,
    pub(crate) page_mark_index: Option<u32>,
    pub(crate) page_mark_flags: Option<u32>,
    pub(crate) page_mark_line_start: Option<u32>,
    pub(crate) page_mark_line_end: Option<u32>,
    pub(crate) page_mark_u16_fields: Vec<u16>,
    pub(crate) paper_mark_entry_index: Option<usize>,
    pub(crate) paper_mark_index: Option<u32>,
    pub(crate) paper_mark_flags: Option<u32>,
    pub(crate) row_index_aligned: bool,
    pub(crate) mark_index_aligned: bool,
    pub(crate) entry_count_aligned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageDecorationSlotEvidence {
    pub(crate) record_index: usize,
    pub(crate) record_offset: usize,
    pub(crate) record_label: Option<String>,
    pub(crate) slot: u16,
    pub(crate) part04: Option<Vec<u8>>,
    pub(crate) part05: Option<Vec<u8>>,
    pub(crate) part06: Option<Vec<u8>>,
    pub(crate) part07: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct VerticalPageTextPlacement {
    pub(crate) x_shift_px: f32,
    pub(crate) y_start_px: f32,
}

impl PageTextLine {
    pub(crate) fn new(
        text: String,
        paragraph_index: Option<usize>,
        char_start: usize,
        char_end: usize,
    ) -> Self {
        Self {
            text,
            paragraph_index,
            char_start,
            char_end,
            native_line_mark_index: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn paragraph_index(&self) -> Option<usize> {
        self.paragraph_index
    }

    pub fn char_start(&self) -> usize {
        self.char_start
    }

    pub fn char_end(&self) -> usize {
        self.char_end
    }
}
