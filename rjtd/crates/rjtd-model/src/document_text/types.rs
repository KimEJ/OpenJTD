use crate::*;

pub(crate) const DOCUMENT_TEXT_INLINE_START_TAG: u32 = 0x001d;

pub(crate) const DOCUMENT_TEXT_TEXT_RUN_MARKER: u16 = 0x001f;

pub(crate) const DOCUMENT_TEXT_RUBY_BASE_SELECTOR: u16 = 0x0003;

pub(crate) const DOCUMENT_TEXT_RUBY_TEXT_SELECTOR: u16 = 0x0082;

pub(crate) const DOCUMENT_TEXT_TOC_PAGE_SELECTOR: u16 = 0x0101;

pub(crate) const DOCUMENT_TEXT_PAGE_BREAK_CONTROL: u16 = 0x000c;

pub(crate) const DOCUMENT_TEXT_PATH: &str = "/DocumentText";

pub(crate) const LAYOUT_BOX_TEXT_PATH: &str = "/LayoutBoxText";

pub(crate) const LAYOUT_BOX_TEXT_POSITION_TABLES_PATH: &str = "/LayoutBoxTextPositionTables";

pub(crate) const TEXT_CONTROL_RANGE_DELIMITER_CANDIDATES: [u16; 2] = [0x001c, 0x000e];

pub(crate) const PARAGRAPH_BOUNDARY_DELIMITER_CANDIDATE: u16 = 0x001c;

pub(crate) const DOCUMENT_TEXT_CONTROL_TABLE_MAX_EMPTY_GAP_ROWS: usize = 3;

pub(crate) const SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR: f32 = 1.5;

pub(crate) const SHANAI_LAN_TEXT_FONT_SIZE_SCALE: f32 = 1.1083333;

pub(crate) const SHANAI_LAN_TEXT_BASELINE_FACTOR: f32 = 0.8;

pub(crate) const SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS: u16 = 4;

pub(crate) const SHANAI_LAN_TEXT_FRAGMENT_GAP_UNITS: usize = 2;

pub(crate) const LAYOUT_BOX_TEXT_MAGIC: &[u8; 8] = b"TextV.01";

pub(crate) const LAYOUT_BOX_TEXT_BODY_MIN_CHARS: usize = 80;

pub(crate) const LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX: f32 = 14.4;

pub(crate) const LAYOUT_BOX_TEXT_TITLE_FONT_SIZE_PX: f32 = 18.0;

pub(crate) const LAYOUT_BOX_TEXT_CAPTION_FONT_SIZE_PX: f32 = 10.5;

pub(crate) const LAYOUT_BOX_TEXT_LINE_HEIGHT_FACTOR: f32 = 2.0;

pub(crate) const LAYOUT_BOX_TEXT_MIN_RENDER_WIDTH_PT: u16 = 48;

pub(crate) const LAYOUT_BOX_TEXT_MAX_RENDER_WIDTH_PT: u16 = 760;

pub(crate) const PAGE_FRAME_TEXT_AFTER_BAR_GAP_LINES: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextParagraphBoundaryCandidate {
    pub(crate) index: usize,
    pub(crate) text_boundary_candidate_index: usize,
    pub(crate) text_count_range_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) text_count_range_span: u32,
    pub(crate) line_word_evidence: TextLayoutExactEvidence,
    pub(crate) page_field_evidence: TextLayoutExactEvidence,
}

impl TextParagraphBoundaryCandidate {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> &'static str {
        "layoutValidatedTextBoundaryCandidate"
    }

    pub fn text_boundary_candidate_index(&self) -> usize {
        self.text_boundary_candidate_index
    }

    pub fn text_count_range_index(&self) -> usize {
        self.text_count_range_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn text_count_range_span(&self) -> u32 {
        self.text_count_range_span
    }

    pub fn line_word_evidence(&self) -> &TextLayoutExactEvidence {
        &self.line_word_evidence
    }

    pub fn page_field_evidence(&self) -> &TextLayoutExactEvidence {
        &self.page_field_evidence
    }

    pub fn rule(&self) -> &'static str {
        "strict-unit-001c-single+nonzero-tcnt-span+line-word-value-exact2+page-be32-field-exact2"
    }
}

pub(crate) fn text_count_entry_chosen_range(raw: &[u8], family: &str) -> (u32, u32) {
    if family == "be1-shifted" {
        (read_be32_candidate(raw, 1), read_be32_candidate(raw, 5))
    } else {
        (read_be32_candidate(raw, 0), read_be32_candidate(raw, 4))
    }
}

pub(crate) fn text_count_entry_tail_offset(family: &str) -> usize {
    if family == "be1-shifted" { 9 } else { 8 }
}

pub(crate) fn classify_text_count_entry_family(raw: &[u8]) -> &'static str {
    let be0_start = read_be32_candidate(raw, 0);
    let be0_end = read_be32_candidate(raw, 4);
    let be1_start = read_be32_candidate(raw, 1);
    let be1_end = read_be32_candidate(raw, 5);

    if be0_start < 256 && be1_start >= 256 && be1_end >= be1_start && be0_end > be1_end {
        "be1-shifted"
    } else {
        "be0"
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Paragraph {
    pub(crate) inlines: Vec<Inline>,
    pub(crate) style: Option<StyleRef>,
}

impl Paragraph {
    pub fn new(inlines: Vec<Inline>, style: Option<StyleRef>) -> Self {
        Self { inlines, style }
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        Self::new(vec![Inline::Text(TextRun::new(text, None))], None)
    }

    pub fn inlines(&self) -> &[Inline] {
        &self.inlines
    }

    pub fn style(&self) -> Option<&StyleRef> {
        self.style.as_ref()
    }

    pub(crate) fn set_style(&mut self, style: Option<StyleRef>) {
        self.style = style;
    }

    pub(crate) fn set_text(&mut self, text: impl Into<String>) {
        self.inlines = vec![Inline::Text(TextRun::new(text, None))];
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RubyAnnotation {
    pub(crate) base_text: String,
    pub(crate) annotation_text: String,
    pub(crate) annotation_selector: u16,
    pub(crate) annotation_source: UnknownObject,
}

impl RubyAnnotation {
    pub fn new(
        base_text: impl Into<String>,
        annotation_text: impl Into<String>,
        annotation_selector: u16,
        annotation_source: UnknownObject,
    ) -> Self {
        Self {
            base_text: base_text.into(),
            annotation_text: annotation_text.into(),
            annotation_selector,
            annotation_source,
        }
    }

    pub fn base_text(&self) -> &str {
        &self.base_text
    }

    pub fn annotation_text(&self) -> &str {
        &self.annotation_text
    }

    pub fn annotation_selector(&self) -> u16 {
        self.annotation_selector
    }

    pub fn annotation_source(&self) -> &UnknownObject {
        &self.annotation_source
    }
}
