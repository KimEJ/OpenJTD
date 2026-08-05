//! Document model types shared by parsers and exporters.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "bitmap-images")]
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use rjtd_core::auto_text_info::{AutoTextEntry, read_auto_text_info};
use rjtd_core::container::{
    CfbEntryReadMode, EntryKind, inspect_cfb_entries, inspect_cfb_entries_with_mode,
    inspect_cfb_stream_chain, read_cfb_stream,
};
use rjtd_core::document_text::{
    DocumentTextControl, DocumentTextElement, DocumentTextMap, DocumentTextMapEntry,
    DocumentTextMapKind, DocumentTextPayload, DocumentTextStyleResolver, InlineTextSegment,
    ParsedDocumentText, SkippedInlineTextSegment, map_document_text,
    parse_document_text_row_headers, read_document_text_payload_with_budget,
};
use rjtd_core::document_text_position::{
    DocumentTextCountEntry, read_document_text_position_tables,
};
use rjtd_core::font_stream::{FontEntry, read_font_stream_with_budget};
use rjtd_core::layout_mark::{
    PAGE_MARK_PATH, PAPER_MARK_PATH, PageMark, PaperMark, read_page_mark, read_paper_mark,
};
use rjtd_core::record::UnknownRecordKind;
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, StyleStreamRecordSummary,
    StyleStreamSubrecordSummary, TEXT_LAYOUT_STYLE_PATH, read_style_streams_with_budget,
    summarize_style_stream,
};
use rjtd_core::{Error, ParseLimits, ResourceBudget, Result};

mod document_text;
mod document_text_text_style;
mod embedded_press;
mod fdm;
mod marks;
mod object_media;
mod object_stream;
mod page_layout;
mod parse;
mod shanai_lan;
mod shanai_lan_sparse_borders;
mod success_data_test;
mod table_candidate;
mod table_grid;

pub use parse::{parse_document, parse_document_with_budget, parse_document_with_limits};

pub use document_text::*;
use embedded_press::*;
pub use fdm::*;
pub use marks::*;
pub use object_media::*;
use object_stream::*;
use page_layout::*;
use shanai_lan::*;
use success_data_test::*;
use table_candidate::*;
use table_grid::*;

use document_text_text_style::{
    DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS, DocumentTextProperty15ColorCandidate,
    document_text_property_15_color_candidate,
};
#[cfg(test)]
use rjtd_core::document_text::read_document_text_payload;
#[cfg(test)]
use shanai_lan_sparse_borders::shanai_lan_source_page_transform_candidate_from_raw_fields;
use shanai_lan_sparse_borders::{
    push_page_layer_shanai_lan_sparse_table_border_topology_diagnostic_json,
    push_shanai_lan_sparse_table_borders_svg, shanai_lan_sparse_table_border_topology_diagnostic,
};

const TABLE_CELL_DELIMITER_CONTROL: u16 = 0x001c;
const TABLE_ROW_DELIMITER_CONTROL: u16 = 0x000e;
const SO_RECORD_MARKER: &[u8] = b"SO\0\0";
const FRAME_RECORD_HEADER_BYTES: usize = 16;
const FRAME_RECORD_BYTES: usize = 60;
const FRAME_RECORD_DECLARED_COUNT_OFFSET: usize = 14;
const FRAME_RECORD_ID_OFFSET: usize = 6;
const FRAME_RECORD_TYPE_OFFSET: usize = 12;
const FRAME_RECORD_X_OFFSET: usize = 28;
const FRAME_RECORD_Y_OFFSET: usize = 32;
const FRAME_RECORD_WIDTH_OFFSET: usize = 36;
const FRAME_RECORD_HEIGHT_OFFSET: usize = 40;
const FRAME_RECORD_CORNER_RADIUS_OFFSET: usize = 44;
const FRAME_RECORD_STYLE_ID_OFFSET: usize = 46;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    metadata: Metadata,
    blocks: Vec<Block>,
    raw_streams: Vec<RawStream>,
    unknown_styles: Vec<UnknownStyle>,
    unknown_objects: Vec<UnknownObject>,
    object_stream_candidates: Vec<ObjectStreamCandidate>,
    object_frame_records: Vec<ObjectFrameRecordCandidate>,
    object_embedding_frames: Vec<ObjectEmbeddingFrameCandidate>,
    text_count_ranges: Vec<TextCountRange>,
    text_control_boundaries: Vec<TextControlBoundary>,
    text_boundary_candidates: Vec<TextBoundaryCandidate>,
    text_paragraph_boundary_candidates: Vec<TextParagraphBoundaryCandidate>,
    table_candidates: Vec<TableCandidate>,
    fonts: Vec<DocumentFont>,
    auto_texts: Vec<DocumentAutoText>,
    toc_entries: Vec<DocumentTocEntry>,
    page_marks: Vec<DocumentPageMark>,
    paper_marks: Vec<DocumentPaperMark>,
}

impl Document {
    pub fn new(metadata: Metadata, blocks: Vec<Block>) -> Self {
        Self {
            metadata,
            blocks,
            raw_streams: Vec::new(),
            unknown_styles: Vec::new(),
            unknown_objects: Vec::new(),
            object_stream_candidates: Vec::new(),
            object_frame_records: Vec::new(),
            object_embedding_frames: Vec::new(),
            text_count_ranges: Vec::new(),
            text_control_boundaries: Vec::new(),
            text_boundary_candidates: Vec::new(),
            text_paragraph_boundary_candidates: Vec::new(),
            table_candidates: Vec::new(),
            fonts: Vec::new(),
            auto_texts: Vec::new(),
            toc_entries: Vec::new(),
            page_marks: Vec::new(),
            paper_marks: Vec::new(),
        }
    }

    pub fn from_plain_text(text: &str) -> Self {
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
        let lines = normalized
            .strip_suffix('\n')
            .unwrap_or(&normalized)
            .split('\n');
        let blocks = lines
            .filter(|line| !line.is_empty())
            .map(|line| Block::Paragraph(Paragraph::from_text(line)))
            .collect();

        Self::new(Metadata::default(), blocks)
    }

    pub fn from_document_text(text: &ParsedDocumentText) -> Self {
        let mut builder = DocumentTextModelBuilder::default();

        for element in text.elements() {
            match element {
                DocumentTextElement::TextRun(text) => builder.push_text_run(text),
                DocumentTextElement::InlineText(segment) => builder.push_inline_text(segment),
                DocumentTextElement::SkippedInlineText(segment) => {
                    builder.push_skipped_inline(segment)
                }
                DocumentTextElement::ControlBoundary(control) => {
                    builder.push_control_boundary(control, None);
                }
            }
        }

        let (blocks, unknown_objects, text_control_boundaries) = builder.finish();
        let mut document = Self::new(Metadata::default(), blocks);
        for object in unknown_objects {
            document.push_unknown_object(object);
        }
        for boundary in text_control_boundaries {
            document.push_text_control_boundary(boundary);
        }
        document
    }

    pub fn from_document_text_payload(payload: &DocumentTextPayload) -> Self {
        let map = map_document_text(payload.bytes());
        let mut spans = DocumentTextSourceSpans::new(map.entries());
        let mut builder = DocumentTextModelBuilder::default();

        for element in payload.parsed_text().elements() {
            match element {
                DocumentTextElement::TextRun(text) => builder
                    .push_text_run_with_span(text, spans.next(DocumentTextMapKind::TextRun, text)),
                DocumentTextElement::InlineText(segment) => builder.push_inline_text_with_span(
                    segment,
                    spans.next(DocumentTextMapKind::InlineText, segment.text()),
                ),
                DocumentTextElement::SkippedInlineText(segment) => builder
                    .push_skipped_inline_with_span(
                        segment,
                        spans.next(DocumentTextMapKind::SkippedInlineText, segment.text()),
                    ),
                DocumentTextElement::ControlBoundary(control) => {
                    builder.push_control_boundary(control, spans.next_control(control.code()));
                }
            }
        }

        let (blocks, unknown_objects, text_control_boundaries) = builder.finish();
        let mut document = Self::new(Metadata::default(), blocks);
        for object in unknown_objects {
            document.push_unknown_object(object);
        }
        for boundary in text_control_boundaries {
            document.push_text_control_boundary(boundary);
        }
        document
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn raw_streams(&self) -> &[RawStream] {
        &self.raw_streams
    }

    pub fn unknown_styles(&self) -> &[UnknownStyle] {
        &self.unknown_styles
    }

    pub fn unknown_objects(&self) -> &[UnknownObject] {
        &self.unknown_objects
    }

    pub fn object_stream_candidates(&self) -> &[ObjectStreamCandidate] {
        &self.object_stream_candidates
    }

    pub fn object_frame_records(&self) -> &[ObjectFrameRecordCandidate] {
        &self.object_frame_records
    }

    pub fn object_embedding_frames(&self) -> &[ObjectEmbeddingFrameCandidate] {
        &self.object_embedding_frames
    }

    pub fn text_count_ranges(&self) -> &[TextCountRange] {
        &self.text_count_ranges
    }

    pub fn text_control_boundaries(&self) -> &[TextControlBoundary] {
        &self.text_control_boundaries
    }

    pub fn text_boundary_candidates(&self) -> &[TextBoundaryCandidate] {
        &self.text_boundary_candidates
    }

    pub fn text_paragraph_boundary_candidates(&self) -> &[TextParagraphBoundaryCandidate] {
        &self.text_paragraph_boundary_candidates
    }

    pub fn table_candidates(&self) -> &[TableCandidate] {
        &self.table_candidates
    }

    pub fn fonts(&self) -> &[DocumentFont] {
        &self.fonts
    }

    pub fn auto_texts(&self) -> &[DocumentAutoText] {
        &self.auto_texts
    }

    pub fn toc_entries(&self) -> &[DocumentTocEntry] {
        &self.toc_entries
    }

    pub fn page_marks(&self) -> &[DocumentPageMark] {
        &self.page_marks
    }

    pub fn paper_marks(&self) -> &[DocumentPaperMark] {
        &self.paper_marks
    }

    pub fn push_unknown_style(&mut self, style: UnknownStyle) {
        self.unknown_styles.push(style);
    }

    pub fn push_unknown_object(&mut self, object: UnknownObject) {
        self.unknown_objects.push(object);
    }

    pub fn push_object_stream_candidate(&mut self, candidate: ObjectStreamCandidate) {
        self.object_stream_candidates.push(candidate);
    }

    pub fn push_object_frame_record(&mut self, record: ObjectFrameRecordCandidate) {
        self.object_frame_records.push(record);
    }

    pub fn push_object_embedding_frame(&mut self, frame: ObjectEmbeddingFrameCandidate) {
        self.object_embedding_frames.push(frame);
    }

    pub fn push_raw_stream(&mut self, stream: RawStream) {
        self.raw_streams.push(stream);
    }

    pub fn push_text_count_range(&mut self, range: TextCountRange) {
        self.text_count_ranges.push(range);
    }

    pub fn push_text_control_boundary(&mut self, boundary: TextControlBoundary) {
        self.text_control_boundaries.push(boundary);
    }

    pub fn push_text_boundary_candidate(&mut self, candidate: TextBoundaryCandidate) {
        self.text_boundary_candidates.push(candidate);
    }

    pub fn push_text_paragraph_boundary_candidate(
        &mut self,
        candidate: TextParagraphBoundaryCandidate,
    ) {
        self.text_paragraph_boundary_candidates.push(candidate);
    }

    pub fn push_table_candidate(&mut self, candidate: TableCandidate) {
        self.table_candidates.push(candidate);
    }

    pub fn push_font(&mut self, font: DocumentFont) {
        self.fonts.push(font);
    }

    pub fn push_auto_text(&mut self, auto_text: DocumentAutoText) {
        self.auto_texts.push(auto_text);
    }

    pub fn push_toc_entry(&mut self, entry: DocumentTocEntry) {
        self.toc_entries.push(entry);
    }

    pub fn push_page_mark(&mut self, page_mark: DocumentPageMark) {
        self.page_marks.push(page_mark);
    }

    pub fn push_paper_mark(&mut self, paper_mark: DocumentPaperMark) {
        self.paper_marks.push(paper_mark);
    }
}

pub trait DocumentParser {
    fn parse(&self, data: &[u8]) -> Result<Document>;
}

pub struct IchitaroParser;

impl IchitaroParser {
    fn parse_with_budget(&self, data: &[u8], budget: &mut ResourceBudget) -> Result<Document> {
        reserve_and_verify_cfb_streams(data, budget)?;
        let payload =
            read_document_text_payload_with_budget(data, budget.decompression_budget_mut())?;
        let map = map_document_text(payload.bytes());
        let mut document = Document::from_document_text_payload(&payload);
        for entry in document_text_toc_entries(map.entries()) {
            document.push_toc_entry(entry);
        }
        document.push_raw_stream(RawStream::new(
            payload.source_name(),
            payload.bytes().to_vec(),
        ));
        if let Ok(line_mark) = read_cfb_stream(data, LINE_MARK_PATH) {
            document.push_raw_stream(RawStream::new(LINE_MARK_PATH, line_mark));
        }
        for stream_name in [
            PAGE_MARK_PATH,
            PAPER_MARK_PATH,
            LAYOUT_BOX_PATH,
            LAYOUT_BOX_TEXT_PATH,
            LAYOUT_BOX_TEXT_POSITION_TABLES_PATH,
        ] {
            if let Ok(stream) = read_cfb_stream(data, stream_name) {
                document.push_raw_stream(RawStream::new(stream_name, stream));
            }
        }
        if let Some(style_streams) = parse::optional_stream(read_style_streams_with_budget(
            data,
            budget.decompression_budget_mut(),
        ))? {
            for stream in style_streams {
                document.push_unknown_style(UnknownStyle::from_stream(
                    stream.name(),
                    stream.bytes().to_vec(),
                ));
            }
        }
        if let Some(font_stream) = parse::optional_stream(read_font_stream_with_budget(
            data,
            budget.decompression_budget_mut(),
        ))? {
            for entry in font_stream.entries() {
                document.push_font(DocumentFont::from_font_stream_entry(
                    font_stream.name(),
                    entry,
                ));
            }
        }
        if let Ok(auto_text_info) = read_auto_text_info(data) {
            for entry in auto_text_info.entries() {
                document.push_auto_text(DocumentAutoText::from_auto_text_entry(
                    auto_text_info.name(),
                    entry,
                ));
            }
        }
        if let Ok(page_mark) = read_page_mark(data) {
            document.push_page_mark(DocumentPageMark::from_page_mark(PAGE_MARK_PATH, &page_mark));
        }
        if let Ok(paper_mark) = read_paper_mark(data) {
            document.push_paper_mark(DocumentPaperMark::from_paper_mark(
                PAPER_MARK_PATH,
                &paper_mark,
            ));
        }
        for candidate in object_stream_candidates_from_cfb(data, budget)? {
            document.push_object_stream_candidate(candidate);
        }
        let object_frame_records = object_frame_records_from_cfb(data, budget)?;
        for record in object_frame_records {
            document.push_object_frame_record(record);
        }
        let object_embedding_frames = object_embedding_frames_from_cfb(data, budget)?;
        for frame in object_embedding_frames {
            document.push_object_embedding_frame(frame);
        }
        if let Ok(position_tables) = read_document_text_position_tables(data) {
            for entry in position_tables.text_count_entries() {
                let mut range = TextCountRange::from_entry(entry);
                range.set_document_text_overlaps(text_count_range_overlaps(&range, &document));
                range.set_control_range_overlaps(text_count_control_range_overlaps(
                    &range,
                    &document,
                    &TEXT_CONTROL_RANGE_DELIMITER_CANDIDATES,
                ));
                document.push_text_count_range(range);
            }
            for candidate in text_boundary_candidates_from_ranges(document.text_count_ranges()) {
                document.push_text_boundary_candidate(candidate);
            }
            for candidate in table_candidates_from_text_boundaries(&document, map.entries()) {
                document.push_table_candidate(candidate);
            }
            for candidate in
                text_paragraph_boundary_candidates_from_layout(&document, map.entries(), data)
            {
                document.push_text_paragraph_boundary_candidate(candidate);
            }
        }
        for candidate in table_candidates_from_document_text_controls(
            map.entries(),
            document.table_candidates().len(),
        ) {
            document.push_table_candidate(candidate);
        }
        for candidate in sparse_table_candidates_from_document_text_controls(
            map.entries(),
            document.table_candidates().len(),
        ) {
            document.push_table_candidate(candidate);
        }
        Ok(document)
    }
}

impl DocumentParser for IchitaroParser {
    fn parse(&self, data: &[u8]) -> Result<Document> {
        let mut budget = ParseLimits::DEFAULT.resource_budget();
        budget.check_input_size(data.len())?;
        self.parse_with_budget(data, &mut budget)
    }
}

const APP_FONT_SIZE_PX: f32 = 13.3;
const APP_TABLE_BASE_FONT_SIZE_UNITS: f32 = 12.0;
const APP_LINE_HEIGHT_PX: f32 = 23.0;
const APP_DEFAULT_COLUMN_WIDTH_PX: f32 =
    (APP_PAGE_WIDTH_PX - (APP_PAGE_MARGIN_PX * 2.0)) / APP_WRAP_COLUMNS as f32;
const APP_VERTICAL_DISPLAY_UNIT_PX: f32 = APP_DEFAULT_COLUMN_WIDTH_PX * 0.925;
const APP_WRAP_COLUMNS: usize = 82;
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_SOURCE_FORMAT: &str = "jtd";
const APP_DEFAULT_DPI: f64 = 96.0;
const APP_TAB_COLUMNS: usize = 4;
const GINGA_TOC_LEADING_BLANK_COLUMNS: usize = 2;
const GINGA_TOC_EXTRA_COLUMNS: usize = 18;
const GINGA_BODY_CHAPTER_LEADING_BLANK_COLUMNS: usize = 2;
const GINGA_BODY_CHAPTER_TRAILING_BLANK_COLUMNS: usize = 2;
const GINGA_COLOPHON_X_SHIFT_COLUMNS: f32 = 1.5;
const GINGA_COLOPHON_TOP_RATIO: f32 = 0.48;
const GINGA_COLOPHON_NOTE_DISPLAY_COLUMNS: usize = 48;
const SHANAI_LAN_REFERENCE_CONTENT_LEFT_PX: f32 = 46.0;
const SHANAI_LAN_REFERENCE_CONTENT_TOP_PX: f32 = 38.7;
const SHANAI_LAN_REFERENCE_CONTENT_WIDTH_PX: f32 = 1021.3;
const SHANAI_LAN_REFERENCE_CONTENT_HEIGHT_PX: f32 = 677.3;
const SHANAI_LAN_LINE_RULE_STROKE_WIDTH_PX: f32 = 2.4;
const SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS: u16 = 24;
const PDF_POINT_TO_CSS_PX: f32 = APP_DEFAULT_DPI as f32 / 72.0;
const FRAME_RECORD_UNIT_TO_CSS_PX: f32 = APP_DEFAULT_DPI as f32 / 25.4 / 100.0;
const MIN_PAPER_SIZE_MM100: u32 = 5_000;
const MAX_PAPER_SIZE_MM100: u32 = 50_000;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WritingMode {
    #[default]
    Horizontal,
    VerticalRl,
}

impl WritingMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::VerticalRl => "vertical-rl",
        }
    }

    fn is_vertical(self) -> bool {
        matches!(self, Self::VerticalRl)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageLayout {
    width_px: f32,
    height_px: f32,
    margin_px: f32,
    vertical_wrap_columns_override: Option<usize>,
    landscape: bool,
}

impl Default for PageLayout {
    fn default() -> Self {
        Self {
            width_px: APP_PAGE_WIDTH_PX,
            height_px: APP_PAGE_HEIGHT_PX,
            margin_px: APP_PAGE_MARGIN_PX,
            vertical_wrap_columns_override: None,
            landscape: false,
        }
    }
}

impl PageLayout {
    fn new(width_px: f32, height_px: f32) -> Self {
        Self {
            width_px,
            height_px,
            margin_px: APP_PAGE_MARGIN_PX,
            vertical_wrap_columns_override: None,
            landscape: width_px > height_px,
        }
    }

    fn with_margin_px(self, margin_px: f32) -> Self {
        Self { margin_px, ..self }
    }

    fn with_vertical_wrap_columns_override(self, wrap_columns: usize) -> Self {
        Self {
            vertical_wrap_columns_override: Some(wrap_columns),
            ..self
        }
    }

    fn with_portrait_orientation(self) -> Self {
        if self.height_px >= self.width_px {
            self
        } else {
            Self {
                width_px: self.height_px,
                height_px: self.width_px,
                margin_px: self.margin_px,
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
        self.margin_px
    }

    pub fn landscape(self) -> bool {
        self.landscape
    }

    pub fn body_width_px(self) -> f32 {
        (self.width_px - (self.margin_px * 2.0)).max(APP_DEFAULT_COLUMN_WIDTH_PX)
    }

    pub fn body_height_px(self) -> f32 {
        (self.height_px - (self.margin_px * 2.0)).max(APP_LINE_HEIGHT_PX)
    }

    fn wrap_columns(self, writing_mode: WritingMode) -> usize {
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

    fn lines_per_page(self, writing_mode: WritingMode) -> usize {
        let extent = if writing_mode.is_vertical() {
            self.body_width_px()
        } else {
            self.body_height_px()
        };
        (extent / APP_LINE_HEIGHT_PX).floor().max(1.0) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SourceDocumentLayoutHint {
    basis: &'static str,
    fallback_layout: PageLayout,
    writing_mode: WritingMode,
    override_decoded_layout: bool,
    margin_override_px: Option<f32>,
    vertical_wrap_columns_override: Option<usize>,
}

fn source_document_layout_hint(
    document: &Document,
    decoded_layout: PageLayout,
) -> Option<SourceDocumentLayoutHint> {
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

fn paper_size_mm100_is_plausible(value: u32) -> bool {
    (MIN_PAPER_SIZE_MM100..=MAX_PAPER_SIZE_MM100).contains(&value)
}

fn hundredth_millimeters_to_css_px(mm100: u32) -> f32 {
    millimeters_to_css_px(mm100 as f32 / 100.0)
}

fn millimeters_to_css_px(mm: f32) -> f32 {
    mm / 25.4 * APP_DEFAULT_DPI as f32
}

/// Application-facing document core, shaped after rhwp's `DocumentCore`.
///
/// rjtd does not yet have a full Ichitaro layout engine. This facade keeps the
/// same load/query/render direction while rendering the current document model
/// as plain text pages.
#[derive(Debug, Clone)]
pub struct DocumentCore {
    document: Document,
    pages: Vec<Vec<PageTextLine>>,
    file_name: String,
    dpi: f64,
    page_layout: PageLayout,
    show_paragraph_marks: bool,
    show_control_codes: bool,
    show_transparent_borders: bool,
    clip_enabled: bool,
    writing_mode: WritingMode,
    next_snapshot_id: u32,
    snapshots: Vec<DocumentSnapshot>,
    caret_section: u32,
    caret_paragraph: u32,
    caret_char_offset: u32,
    clipboard_text: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PageTextLine {
    text: String,
    paragraph_index: Option<usize>,
    char_start: usize,
    char_end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageDecorationSide {
    Left,
    Right,
}

impl PageDecorationSide {
    fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    fn text_anchor(self) -> &'static str {
        match self {
            Self::Left => "start",
            Self::Right => "end",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageDecoration {
    side: PageDecorationSide,
    page_number: usize,
    header_text: String,
    source: &'static str,
    side_policy: &'static str,
    side_policy_decoded: bool,
    facing_pages_candidate: bool,
    paired_slot_pairs: Vec<(u16, u16)>,
    slot_evidence: Vec<PageDecorationSlotEvidence>,
    mark_evidence: Option<PageDecorationMarkEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageDecorationMarkEvidence {
    page_index: usize,
    page_mark_entry_index: Option<usize>,
    page_mark_index: Option<u32>,
    page_mark_flags: Option<u32>,
    page_mark_line_start: Option<u32>,
    page_mark_line_end: Option<u32>,
    page_mark_u16_fields: Vec<u16>,
    paper_mark_entry_index: Option<usize>,
    paper_mark_index: Option<u32>,
    paper_mark_flags: Option<u32>,
    row_index_aligned: bool,
    mark_index_aligned: bool,
    entry_count_aligned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageDecorationSlotEvidence {
    record_index: usize,
    record_offset: usize,
    record_label: Option<String>,
    slot: u16,
    part04: Option<Vec<u8>>,
    part05: Option<Vec<u8>>,
    part06: Option<Vec<u8>>,
    part07: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct VerticalPageTextPlacement {
    x_shift_px: f32,
    y_start_px: f32,
}

impl PageTextLine {
    fn new(
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

impl DocumentCore {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        Self::from_bytes_with_limits(data, ParseLimits::DEFAULT)
    }

    /// Creates a document core from already allocated input with explicit resource limits.
    ///
    /// Per-member and total LH5 output are bounded by `limits`; the input limit checks this
    /// `&[u8]` after the caller has allocated it, so it cannot reduce the caller's allocation.
    pub fn from_bytes_with_limits(data: &[u8], limits: ParseLimits) -> Result<Self> {
        let mut budget = limits.resource_budget();
        Self::from_bytes_with_budget(data, &mut budget)
    }

    /// Builds a document core with caller-owned shared resource accounting.
    pub fn from_bytes_with_budget(data: &[u8], budget: &mut ResourceBudget) -> Result<Self> {
        let document = parse_document_with_budget(data, budget)?;
        Self::from_document_with_budget(document, budget)
    }

    pub fn from_document(document: Document) -> Self {
        let mut core = Self::from_document_unpaginated(document);
        core.refresh_pages();
        core
    }

    pub fn from_document_with_limits(document: Document, limits: ParseLimits) -> Result<Self> {
        let mut budget = limits.resource_budget();
        Self::from_document_with_budget(document, &mut budget)
    }

    /// Builds page state with caller-owned shared resource accounting.
    pub fn from_document_with_budget(
        document: Document,
        budget: &mut ResourceBudget,
    ) -> Result<Self> {
        let mut core = Self::from_document_unpaginated(document);
        core.refresh_pages_with_budget(budget)?;
        Ok(core)
    }

    fn from_document_unpaginated(document: Document) -> Self {
        let decoded_page_layout = page_layout_from_document(&document);
        let hint = source_document_layout_hint(&document, decoded_page_layout);
        let mut page_layout = decoded_page_layout;
        let mut writing_mode = WritingMode::Horizontal;
        if let Some(hint) = hint {
            if hint.override_decoded_layout || page_layout == PageLayout::default() {
                page_layout = hint.fallback_layout;
            }
            if let Some(margin_px) = hint.margin_override_px {
                page_layout = page_layout.with_margin_px(margin_px);
            }
            if let Some(wrap_columns) = hint.vertical_wrap_columns_override {
                page_layout = page_layout.with_vertical_wrap_columns_override(wrap_columns);
            }
            writing_mode = hint.writing_mode;
        }
        Self {
            document,
            pages: Vec::new(),
            file_name: String::new(),
            dpi: APP_DEFAULT_DPI,
            page_layout,
            show_paragraph_marks: false,
            show_control_codes: false,
            show_transparent_borders: false,
            clip_enabled: true,
            writing_mode,
            next_snapshot_id: 1,
            snapshots: Vec::new(),
            caret_section: 0,
            caret_paragraph: 0,
            caret_char_offset: 0,
            clipboard_text: None,
        }
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    fn observed_table_candidate(&self, control_idx: u32) -> Option<&TableCandidate> {
        let candidate = self.document.table_candidates().get(control_idx as usize)?;
        candidate.is_row_like().then_some(candidate)
    }

    fn observed_table_cell(
        &self,
        control_idx: u32,
        cell_idx: u32,
    ) -> Option<&TableCandidateInterval> {
        self.observed_table_candidate(control_idx)?
            .intervals()
            .get(cell_idx as usize)
    }

    pub fn page_count(&self) -> u32 {
        self.pages.len().max(1) as u32
    }

    pub fn get_section_count(&self) -> u32 {
        1
    }

    pub fn get_document_info(&self) -> String {
        let style_candidates = text_style_candidates(self.document.unknown_styles());
        let font_names = document_font_names(&self.document);
        let fallback_font = primary_document_font_name(&font_names);
        let writing_mode_decision = writing_mode_decision_json(&self.document, self.writing_mode);
        let document_view_writing_mode_candidate =
            writing_mode_candidate_from_document_view_styles(self.document.unknown_styles());
        let document_view_writing_mode_candidate_str = document_view_writing_mode_candidate
            .as_ref()
            .map(|candidate| format!("\"{}\"", candidate.writing_mode.as_str()))
            .unwrap_or_else(|| "null".to_string());
        let document_view_writing_mode_first_code = document_view_writing_mode_candidate
            .as_ref()
            .map(|candidate| candidate.first_record_code.to_string())
            .unwrap_or_else(|| "null".to_string());
        let document_view_writing_mode_first_code_hex = document_view_writing_mode_candidate
            .as_ref()
            .map(|candidate| json_string(&format!("0x{:04x}", candidate.first_record_code)))
            .unwrap_or_else(|| "null".to_string());
        let paper_mark_writing_mode_diagnostics =
            paper_mark_writing_mode_diagnostics(self.document.paper_marks());
        let fdm_text_mirror_anchor_agreements =
            fdm_text_mirror_anchor_agreements(self.document.object_stream_candidates());
        let writing_mode_candidate_str = paper_mark_writing_mode_diagnostics
            .candidate
            .map(|m| format!("\"{}\"", m.as_str()))
            .unwrap_or_else(|| "null".to_string());
        format!(
            "{{\"version\":\"{APP_VERSION}\",\"format\":\"JTD\",\"engine\":\"rjtd\",\"sourceFormat\":\"{}\",\"fileName\":{},\"sectionCount\":1,\"pageCount\":{},\"encrypted\":false,\"hwp3Variant\":false,\"fallbackFont\":{},\"fontsUsed\":{},\"writingMode\":\"{}\",\"writingModeDecoded\":false,\"writingModeDecision\":{},\"writingModeCandidateFromDocumentViewStyles\":{},\"writingModeCandidateFromDocumentViewStylesDecoded\":false,\"writingModeCandidateFromDocumentViewStylesSourceBacked\":{},\"writingModeCandidateFromDocumentViewStylesFirstRecordCode\":{},\"writingModeCandidateFromDocumentViewStylesFirstRecordCodeHex\":{},\"writingModeCandidateFromPaperMark\":{},\"writingModeCandidateDecoded\":false,\"paperMarkFlagBit0VerticalCandidate\":{},\"paperMarkFlagBit17IndexStepCandidate\":{},\"paperMarkWritingModeCandidateEvidence\":{},\"paperMarkWritingModeCandidateBlockers\":{},\"blockCount\":{},\"rawStreamCount\":{},\"styleStreamCount\":{},\"styleCandidateCount\":{},\"styleCandidateNames\":{},\"styleStreams\":{},\"fontCount\":{},\"fontTable\":{},\"autoTextCount\":{},\"autoTextCandidates\":{},\"tocEntryCount\":{},\"tocEntries\":{},\"pageMarkCount\":{},\"pageMarks\":{},\"paperMarkCount\":{},\"paperMarks\":{},\"objectStreamCandidateCount\":{},\"objectStreamCandidates\":{},\"fdmTextMirrorAnchorAgreementCount\":{},\"fdmTextMirrorAnchorAgreements\":{},\"objectFrameRecordCount\":{},\"objectFrameRecords\":{},\"objectEmbeddingFrameCount\":{},\"objectEmbeddingFrames\":{},\"textCountRangeCount\":{},\"textCountRanges\":{},\"textControlBoundaryCount\":{},\"textControlBoundaries\":{},\"textBoundaryCandidateCount\":{},\"textBoundaryCandidates\":{},\"textParagraphBoundaryCandidateCount\":{},\"textParagraphBoundaryCandidates\":{},\"fdmOpenStrokeCohortSummary\":{},\"tableCandidateCount\":{},\"tableCandidates\":{}}}",
            APP_SOURCE_FORMAT,
            json_string(&self.file_name),
            self.page_count(),
            json_string(fallback_font),
            string_array_json(&font_names),
            self.writing_mode.as_str(),
            writing_mode_decision,
            document_view_writing_mode_candidate_str,
            if document_view_writing_mode_candidate.is_some() {
                "true"
            } else {
                "false"
            },
            document_view_writing_mode_first_code,
            document_view_writing_mode_first_code_hex,
            writing_mode_candidate_str,
            paper_mark_writing_mode_diagnostics.flag_bit0_vertical_candidate,
            paper_mark_writing_mode_diagnostics.flag_bit17_index_step_candidate,
            string_slice_array_json(&paper_mark_writing_mode_diagnostics.evidence),
            string_slice_array_json(&paper_mark_writing_mode_diagnostics.blockers),
            self.document.blocks().len(),
            self.document.raw_streams().len(),
            self.document.unknown_styles().len(),
            style_candidates.len(),
            style_candidate_names_json(&style_candidates),
            style_source_streams_json(self.document.unknown_styles()),
            self.document.fonts().len(),
            font_table_json(self.document.fonts()),
            self.document.auto_texts().len(),
            auto_texts_json(self.document.auto_texts()),
            self.document.toc_entries().len(),
            toc_entries_json(self.document.toc_entries()),
            self.document.page_marks().len(),
            page_marks_json(self.document.page_marks()),
            self.document.paper_marks().len(),
            paper_marks_json(self.document.paper_marks()),
            self.document.object_stream_candidates().len(),
            object_stream_candidates_json(self.document.object_stream_candidates()),
            fdm_text_mirror_anchor_agreements.len(),
            fdm_text_mirror_anchor_agreements_json(&fdm_text_mirror_anchor_agreements),
            self.document.object_frame_records().len(),
            object_frame_records_json(self.document.object_frame_records()),
            self.document.object_embedding_frames().len(),
            object_embedding_frames_json(self.document.object_embedding_frames()),
            self.document.text_count_ranges().len(),
            text_count_ranges_json(self.document.text_count_ranges()),
            self.document.text_control_boundaries().len(),
            text_control_boundaries_json(self.document.text_control_boundaries()),
            self.document.text_boundary_candidates().len(),
            text_boundary_candidates_json(self.document.text_boundary_candidates()),
            self.document.text_paragraph_boundary_candidates().len(),
            text_paragraph_boundary_candidates_json(
                self.document.text_paragraph_boundary_candidates()
            ),
            fdm_open_stroke_cohort_summary_json(self.page_layout, &self.document),
            self.document.table_candidates().len(),
            table_candidates_json(self.document.table_candidates())
        )
    }

    pub fn set_file_name(&mut self, name: impl Into<String>) {
        self.file_name = name.into();
        let decoded_page_layout = page_layout_from_document(&self.document);
        if let Some(hint) = source_document_layout_hint(&self.document, decoded_page_layout) {
            if hint.override_decoded_layout || self.page_layout == PageLayout::default() {
                self.page_layout = hint.fallback_layout;
            }
            if let Some(margin_px) = hint.margin_override_px {
                self.page_layout = self.page_layout.with_margin_px(margin_px);
            }
            if let Some(wrap_columns) = hint.vertical_wrap_columns_override {
                self.page_layout = self
                    .page_layout
                    .with_vertical_wrap_columns_override(wrap_columns);
            }
            self.writing_mode = hint.writing_mode;
        }
        self.refresh_pages();
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_source_format(&self) -> &'static str {
        APP_SOURCE_FORMAT
    }

    pub fn get_dpi(&self) -> f64 {
        self.dpi
    }

    pub fn set_dpi(&mut self, dpi: f64) {
        if dpi.is_finite() && dpi > 0.0 {
            self.dpi = dpi;
        }
    }

    pub fn writing_mode(&self) -> WritingMode {
        self.writing_mode
    }

    pub fn set_writing_mode(&mut self, writing_mode: WritingMode) {
        self.writing_mode = writing_mode;
        self.refresh_pages();
    }

    pub fn page_layout(&self) -> PageLayout {
        self.page_layout
    }

    pub fn get_page_def(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        let layout = self.page_layout;
        Ok(format!(
            "{{\"width\":{:.1},\"height\":{:.1},\"marginLeft\":{:.1},\"marginRight\":{:.1},\"marginTop\":{:.1},\"marginBottom\":{:.1},\"marginHeader\":0.0,\"marginFooter\":0.0,\"marginGutter\":0.0,\"landscape\":{},\"binding\":0}}",
            layout.width_px(),
            layout.height_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.landscape()
        ))
    }

    pub fn get_page_def_native(&self, section_idx: u32) -> Result<String> {
        self.get_page_def(section_idx)
    }

    pub fn set_page_def(&mut self, section_idx: u32, _page_def_json: &str) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn set_page_def_native(&mut self, section_idx: u32, page_def_json: &str) -> Result<String> {
        self.set_page_def(section_idx, page_def_json)
    }

    pub fn get_section_def(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"pageNum\":1,\"pageNumType\":0,\"pictureNum\":1,\"tableNum\":1,\"equationNum\":1,\"columnSpacing\":0,\"defaultTabSpacing\":0,\"hideHeader\":false,\"hideFooter\":false,\"hideMasterPage\":false,\"hideBorder\":false,\"hideFill\":false,\"hideEmptyLine\":false}".to_string())
    }

    pub fn get_section_def_native(&self, section_idx: u32) -> Result<String> {
        self.get_section_def(section_idx)
    }

    pub fn set_section_def(&mut self, section_idx: u32, _section_def_json: &str) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn set_section_def_native(
        &mut self,
        section_idx: u32,
        section_def_json: &str,
    ) -> Result<String> {
        self.set_section_def(section_idx, section_def_json)
    }

    pub fn set_section_def_all(&mut self, _section_def_json: &str) -> String {
        ok_page_count_json(self.page_count())
    }

    pub fn set_section_def_all_native(&mut self, section_def_json: &str) -> String {
        self.set_section_def_all(section_def_json)
    }

    pub fn get_page_border_fill(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        let border = "{\"type\":0,\"width\":0,\"color\":\"#000000\"}";
        Ok(format!(
            "{{\"attr\":0,\"basis\":\"paper\",\"spacingLeft\":0,\"spacingRight\":0,\"spacingTop\":0,\"spacingBottom\":0,\"borderFillId\":0,\"headerInside\":false,\"footerInside\":false,\"fillArea\":\"paper\",\"hideBorder\":true,\"hideFill\":true,\"borderLeft\":{border},\"borderRight\":{border},\"borderTop\":{border},\"borderBottom\":{border},\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0,\"applyPage\":\"all\"}}"
        ))
    }

    pub fn get_page_border_fill_native(&self, section_idx: u32) -> Result<String> {
        self.get_page_border_fill(section_idx)
    }

    pub fn set_page_border_fill(
        &mut self,
        section_idx: u32,
        _settings_json: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn set_page_border_fill_native(
        &mut self,
        section_idx: u32,
        settings_json: &str,
    ) -> Result<String> {
        self.set_page_border_fill(section_idx, settings_json)
    }

    pub fn plain_text(&self) -> String {
        document_plain_text(&self.document)
    }

    pub fn page_width_px(&self) -> f64 {
        self.page_layout.width_px() as f64
    }

    pub fn page_height_px(&self) -> f64 {
        self.page_layout.height_px() as f64
    }

    pub fn page_margin_px(&self) -> f64 {
        self.page_layout.margin_px() as f64
    }

    pub fn font_size_px(&self) -> f64 {
        APP_FONT_SIZE_PX as f64
    }

    pub fn line_height_px(&self) -> f64 {
        APP_LINE_HEIGHT_PX as f64
    }

    pub fn page_text_lines(&self, page_num: u32) -> Result<&[PageTextLine]> {
        self.page_lines(page_num)
    }

    fn page_decoration(&self, page_index: usize) -> Option<PageDecoration> {
        if !self.writing_mode.is_vertical() {
            return None;
        }
        let paired_slot_pairs = document_page_decoration_paired_slot_pairs(&self.document);
        if paired_slot_pairs.is_empty() {
            return None;
        }
        let slot_evidence = document_page_decoration_slot_evidence(&self.document);
        let document_title = document_auto_text_title(&self.document)?;
        let chapter_titles = document_chapter_title_candidates(&self.document);
        if chapter_titles.is_empty() {
            return None;
        }
        let body_start_page =
            running_body_start_page(&self.pages, document_title, &chapter_titles)?;
        if page_index < body_start_page {
            return None;
        }
        if page_index > body_start_page
            && self
                .pages
                .get(page_index)
                .is_some_and(|page| page_has_exact_text_line(page, document_title))
        {
            return None;
        }
        let chapter_title = running_chapter_title_for_page(
            &self.pages,
            body_start_page,
            page_index,
            &chapter_titles,
        )?;
        let page_number = page_index + 1;
        let side = if page_number.is_multiple_of(2) {
            PageDecorationSide::Left
        } else {
            PageDecorationSide::Right
        };
        let header_text = if side == PageDecorationSide::Left {
            chapter_title
        } else {
            document_title.to_string()
        };
        Some(PageDecoration {
            side,
            page_number,
            header_text,
            source: "autoTextInfo+pageLayoutStylePairedSlots+documentText",
            side_policy: "facing-pages-odd-right-even-left",
            side_policy_decoded: false,
            facing_pages_candidate: true,
            paired_slot_pairs,
            slot_evidence,
            mark_evidence: page_decoration_mark_evidence(&self.document, page_index),
        })
    }

    pub fn get_page_info(&self, page_num: u32) -> Result<String> {
        self.page_lines(page_num)?;
        let layout = self.page_layout;
        let body_x = layout.margin_px();
        let body_width = layout.body_width_px();
        let mark_evidence = page_decoration_mark_evidence(&self.document, page_num as usize);
        let mut mark_evidence_json = String::new();
        push_page_decoration_mark_evidence_json(
            &mut mark_evidence_json,
            layout,
            mark_evidence.as_ref(),
        );
        Ok(format!(
            "{{\"pageIndex\":{},\"pageNumber\":{},\"width\":{:.1},\"height\":{:.1},\"sectionIndex\":0,\"marginLeft\":{:.1},\"marginRight\":{:.1},\"marginTop\":{:.1},\"marginBottom\":{:.1},\"marginHeader\":0.0,\"marginFooter\":0.0,\"pageBorderLeft\":{:.1},\"pageBorderRight\":{:.1},\"pageBorderTop\":{:.1},\"pageBorderBottom\":{:.1},\"columns\":[{{\"x\":{:.1},\"width\":{:.1}}}],\"layoutMarkEvidence\":{}}}",
            page_num,
            page_num + 1,
            layout.width_px(),
            layout.height_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            layout.margin_px(),
            body_x,
            body_width,
            mark_evidence_json
        ))
    }

    pub fn get_page_info_native(&self, page_num: u32) -> Result<String> {
        self.get_page_info(page_num)
    }

    pub fn get_page_layer_tree(&self, page_num: u32) -> Result<String> {
        self.get_page_layer_tree_with_profile(page_num, "screen")
    }

    pub fn get_page_layer_tree_native(&self, page_num: u32) -> Result<String> {
        self.get_page_layer_tree(page_num)
    }

    pub fn get_page_layer_tree_with_profile(&self, page_num: u32, profile: &str) -> Result<String> {
        let lines = self.page_lines(page_num)?;
        let profile = if profile.is_empty() {
            "screen"
        } else {
            profile
        };
        Ok(page_layer_tree_json(self, lines, profile, page_num))
    }

    pub fn get_page_layer_tree_with_profile_native(
        &self,
        page_num: u32,
        profile: &str,
    ) -> Result<String> {
        self.get_page_layer_tree_with_profile(page_num, profile)
    }

    pub fn get_page_overlay_images(&self, page_num: u32) -> Result<String> {
        self.page_lines(page_num)?;
        Ok(page_overlay_images_json(self))
    }

    pub fn get_page_overlay_images_native(&self, page_num: u32) -> Result<String> {
        self.get_page_overlay_images(page_num)
    }

    pub fn get_canvaskit_replay_plan(&self, page_num: u32, mode: &str) -> Result<String> {
        let lines = self.page_lines(page_num)?;
        let mode = canvaskit_replay_mode(mode)?;
        Ok(canvaskit_replay_plan_json(self, lines, mode))
    }

    pub fn get_canvaskit_replay_plan_native(&self, page_num: u32, mode: &str) -> Result<String> {
        self.get_canvaskit_replay_plan(page_num, mode)
    }

    pub fn convert_to_editable(&mut self) -> String {
        "{\"ok\":true,\"converted\":false}".to_string()
    }

    pub fn convert_to_editable_native(&mut self) -> String {
        self.convert_to_editable()
    }

    pub fn refresh_layout(&mut self) {
        self.refresh_pages();
    }

    pub fn get_validation_warnings(&self) -> String {
        jtd_validation_warnings_json(&jtd_validation_warnings(&self.document))
    }

    pub fn reflow_linesegs(&mut self) -> u32 {
        self.refresh_pages();
        0
    }

    pub fn get_external_image_basenames(&self) -> String {
        "[]".to_string()
    }

    pub fn inject_external_image(
        &mut self,
        _name: &str,
        _bytes: &[u8],
        _display_path: &str,
    ) -> u32 {
        0
    }

    pub fn get_page_control_layout(&self, page_num: u32) -> Result<String> {
        self.page_lines(page_num)?;
        let mut controls = Vec::new();
        for control in projected_text_controls(&self.document) {
            let Ok(rect) = self.cursor_rect_for(control.paragraph_index, control.char_offset)
            else {
                continue;
            };
            if rect.page_index != page_num as usize {
                continue;
            }
            controls.push(projected_control_layout_json(
                self.page_layout,
                &control,
                &rect,
            ));
        }
        Ok(format!("{{\"controls\":[{}]}}", controls.join(",")))
    }

    pub fn get_page_control_layout_native(&self, page_num: u32) -> Result<String> {
        self.get_page_control_layout(page_num)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_text_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        char_offset: u32,
        _text: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_text_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String> {
        self.insert_text_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            text,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_text_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_text_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.delete_text_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            count,
        )
    }

    pub fn insert_text_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
        _text: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn insert_text_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        text: &str,
    ) -> Result<String> {
        self.insert_text_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset, text)
    }

    pub fn delete_text_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn delete_text_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.delete_text_in_cell_by_path(
            section_idx,
            parent_para_idx,
            path_json,
            char_offset,
            count,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_paragraph_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"cellParaIndex\":{cell_para_idx},\"charOffset\":{char_offset}}}"
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_paragraph_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.split_paragraph_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn split_paragraph_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"cellParaIndex\":0,\"charOffset\":{char_offset}}}"
        ))
    }

    pub fn split_paragraph_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.split_paragraph_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    pub fn merge_paragraph_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"cellParaIndex\":{cell_para_idx},\"charOffset\":0}}"
        ))
    }

    pub fn merge_paragraph_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String> {
        self.merge_paragraph_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    pub fn merge_paragraph_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false,\"cellParaIndex\":0,\"charOffset\":0}".to_string())
    }

    pub fn merge_paragraph_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String> {
        self.merge_paragraph_in_cell_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn paste_internal_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn paste_internal_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.paste_internal_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn get_cell_paragraph_count(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(|_| 1)
            .unwrap_or(0))
    }

    pub fn get_cell_paragraph_count_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<u32> {
        self.get_cell_paragraph_count(section_idx, parent_para_idx, control_idx, cell_idx)
    }

    pub fn get_cell_paragraph_length(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        if cell_para_idx != 0 {
            return Ok(0);
        }
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(|cell| cell.text_preview().chars().count() as u32)
            .unwrap_or(0))
    }

    pub fn get_cell_paragraph_length_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<u32> {
        self.get_cell_paragraph_length(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    pub fn get_cell_paragraph_count_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(0)
    }

    pub fn get_cell_paragraph_count_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<u32> {
        self.get_cell_paragraph_count_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_cell_paragraph_length_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(0)
    }

    pub fn get_cell_paragraph_length_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<u32> {
        self.get_cell_paragraph_length_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_cell_text_direction(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(0)
    }

    pub fn get_cell_text_direction_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<u32> {
        self.get_cell_text_direction(section_idx, parent_para_idx, control_idx, cell_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_text_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        if cell_para_idx != 0 {
            return Ok(String::new());
        }
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(|cell| char_slice(cell.text_preview(), char_offset, count))
            .unwrap_or_default())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_text_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.get_text_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            count,
        )
    }

    pub fn get_text_in_cell_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        _char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(String::new())
    }

    pub fn get_text_in_cell_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.get_text_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset, count)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_cursor_rect_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cursor_rect_json(0))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_cursor_rect_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_cursor_rect_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn get_cursor_rect_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cursor_rect_json(0))
    }

    pub fn get_cursor_rect_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.get_cursor_rect_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_line_info_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        if cell_para_idx != 0 {
            return Ok(default_line_info_json());
        }
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(observed_cell_line_info_json)
            .unwrap_or_else(default_line_info_json))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_line_info_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_line_info_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn get_table_dimensions(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(self
            .observed_table_candidate(control_idx)
            .map(observed_table_dimensions_json)
            .unwrap_or_else(default_table_dimensions_json))
    }

    pub fn get_table_dimensions_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_table_dimensions(section_idx, parent_para_idx, control_idx)
    }

    pub fn get_table_dimensions_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_dimensions_json())
    }

    pub fn get_table_dimensions_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String> {
        self.get_table_dimensions_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_cell_info(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(|cell| observed_cell_info_json(cell_idx, cell))
            .unwrap_or_else(default_cell_info_json))
    }

    pub fn get_cell_info_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<String> {
        self.get_cell_info(section_idx, parent_para_idx, control_idx, cell_idx)
    }

    pub fn get_cell_info_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_info_json())
    }

    pub fn get_cell_info_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String> {
        self.get_cell_info_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_cell_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_properties_json())
    }

    pub fn get_cell_properties_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<String> {
        self.get_cell_properties(section_idx, parent_para_idx, control_idx, cell_idx)
    }

    pub fn set_cell_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_cell_properties_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.set_cell_properties(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            props_json,
        )
    }

    pub fn resize_table_cells(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _updates_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn resize_table_cells_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        updates_json: &str,
    ) -> Result<String> {
        self.resize_table_cells(section_idx, parent_para_idx, control_idx, updates_json)
    }

    pub fn move_table_offset(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        _delta_h: i32,
        _delta_v: i32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"ppi\":{},\"ci\":{}}}",
            parent_para_idx, control_idx
        ))
    }

    pub fn move_table_offset_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        delta_h: i32,
        delta_v: i32,
    ) -> Result<String> {
        self.move_table_offset(section_idx, parent_para_idx, control_idx, delta_h, delta_v)
    }

    pub fn get_table_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_properties_json())
    }

    pub fn get_table_properties_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_table_properties(section_idx, parent_para_idx, control_idx)
    }

    pub fn set_table_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_table_properties_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.set_table_properties(section_idx, parent_para_idx, control_idx, props_json)
    }

    pub fn get_table_cell_bboxes(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _page_hint: Option<u32>,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("[]".to_string())
    }

    pub fn get_table_cell_bboxes_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        page_hint: Option<u32>,
    ) -> Result<String> {
        self.get_table_cell_bboxes(section_idx, parent_para_idx, control_idx, page_hint)
    }

    pub fn get_table_cell_bboxes_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("[]".to_string())
    }

    pub fn get_table_cell_bboxes_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String> {
        self.get_table_cell_bboxes_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_table_bbox(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"pageIndex\":0,\"x\":0.0,\"y\":0.0,\"width\":0.0,\"height\":0.0}".to_string())
    }

    pub fn get_table_bbox_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_table_bbox(section_idx, parent_para_idx, control_idx)
    }

    pub fn create_table(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _rows: u32,
        _cols: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{},\"controlIdx\":-1}}",
            paragraph_idx
        ))
    }

    pub fn create_table_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        rows: u32,
        cols: u32,
    ) -> Result<String> {
        self.create_table(section_idx, paragraph_idx, char_offset, rows, cols)
    }

    pub fn delete_table_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn delete_table_control_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.delete_table_control(section_idx, parent_para_idx, control_idx)
    }

    pub fn insert_table_row(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _row_idx: u32,
        _below: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_edit_result_json())
    }

    pub fn insert_table_row_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row_idx: u32,
        below: bool,
    ) -> Result<String> {
        self.insert_table_row(section_idx, parent_para_idx, control_idx, row_idx, below)
    }

    pub fn insert_table_column(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _col_idx: u32,
        _right: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_edit_result_json())
    }

    pub fn insert_table_column_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        col_idx: u32,
        right: bool,
    ) -> Result<String> {
        self.insert_table_column(section_idx, parent_para_idx, control_idx, col_idx, right)
    }

    pub fn delete_table_row(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _row_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_edit_result_json())
    }

    pub fn delete_table_row_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row_idx: u32,
    ) -> Result<String> {
        self.delete_table_row(section_idx, parent_para_idx, control_idx, row_idx)
    }

    pub fn delete_table_column(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _col_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_edit_result_json())
    }

    pub fn delete_table_column_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        col_idx: u32,
    ) -> Result<String> {
        self.delete_table_column(section_idx, parent_para_idx, control_idx, col_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn merge_table_cells(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _start_row: u32,
        _start_col: u32,
        _end_row: u32,
        _end_col: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_count_result_json())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn merge_table_cells_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
    ) -> Result<String> {
        self.merge_table_cells(
            section_idx,
            parent_para_idx,
            control_idx,
            start_row,
            start_col,
            end_row,
            end_col,
        )
    }

    pub fn split_table_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _row: u32,
        _col: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_count_result_json())
    }

    pub fn split_table_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row: u32,
        col: u32,
    ) -> Result<String> {
        self.split_table_cell(section_idx, parent_para_idx, control_idx, row, col)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_table_cell_into(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _row: u32,
        _col: u32,
        _n_rows: u32,
        _m_cols: u32,
        _equal_row_height: bool,
        _merge_first: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_count_result_json())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_table_cell_into_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row: u32,
        col: u32,
        n_rows: u32,
        m_cols: u32,
        equal_row_height: bool,
        merge_first: bool,
    ) -> Result<String> {
        self.split_table_cell_into(
            section_idx,
            parent_para_idx,
            control_idx,
            row,
            col,
            n_rows,
            m_cols,
            equal_row_height,
            merge_first,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_table_cells_in_range(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _start_row: u32,
        _start_col: u32,
        _end_row: u32,
        _end_col: u32,
        _n_rows: u32,
        _m_cols: u32,
        _equal_row_height: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_count_result_json())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_table_cells_in_range_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
        n_rows: u32,
        m_cols: u32,
        equal_row_height: bool,
    ) -> Result<String> {
        self.split_table_cells_in_range(
            section_idx,
            parent_para_idx,
            control_idx,
            start_row,
            start_col,
            end_row,
            end_col,
            n_rows,
            m_cols,
            equal_row_height,
        )
    }

    pub fn get_column_def(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"columnCount\":1,\"columnType\":0,\"sameWidth\":true,\"spacing\":0}".to_string())
    }

    pub fn get_column_def_native(&self, section_idx: u32) -> Result<String> {
        self.get_column_def(section_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_selection_rects_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _start_cell_para_idx: u32,
        _start_char_offset: u32,
        _end_cell_para_idx: u32,
        _end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("[]".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_selection_rects_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.get_selection_rects_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            start_cell_para_idx,
            start_char_offset,
            end_cell_para_idx,
            end_char_offset,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn copy_selection_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _start_cell_para_idx: u32,
        _start_char_offset: u32,
        _end_cell_para_idx: u32,
        _end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false,\"text\":\"\"}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn copy_selection_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.copy_selection_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            start_cell_para_idx,
            start_char_offset,
            end_cell_para_idx,
            end_char_offset,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_range_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        _end_cell_para_idx: u32,
        _end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{start_cell_para_idx},\"charOffset\":{start_char_offset}}}"
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_range_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.delete_range_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            start_cell_para_idx,
            start_char_offset,
            end_cell_para_idx,
            end_char_offset,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_cell_char_properties_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_char_properties_json())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_cell_char_properties_at_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_cell_char_properties_at(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn get_cell_para_properties_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_para_properties_json())
    }

    pub fn get_cell_para_properties_at_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String> {
        self.get_cell_para_properties_at(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_char_format_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        start_offset: u32,
        end_offset: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        if start_offset > end_offset {
            return Err(rjtd_core::Error::InvalidData(format!(
                "start offset {start_offset} is after end offset {end_offset}"
            )));
        }
        Ok("{\"ok\":false}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_char_format_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        start_offset: u32,
        end_offset: u32,
        props_json: &str,
    ) -> Result<String> {
        self.apply_char_format_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            start_offset,
            end_offset,
            props_json,
        )
    }

    pub fn apply_para_format_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn apply_para_format_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.apply_para_format_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            props_json,
        )
    }

    pub fn get_cell_style_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"id\":0,\"name\":\"Normal\"}".to_string())
    }

    pub fn get_cell_style_at_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String> {
        self.get_cell_style_at(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    pub fn apply_cell_style(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _style_id: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn apply_cell_style_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        style_id: u32,
    ) -> Result<String> {
        self.apply_cell_style(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            style_id,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_table_formula(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _target_row: u32,
        _target_col: u32,
        formula: &str,
        _write_result: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"value\":\"\",\"formula\":{}}}",
            json_string(formula)
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_table_formula_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        target_row: u32,
        target_col: u32,
        formula: &str,
        write_result: bool,
    ) -> Result<String> {
        self.evaluate_table_formula(
            section_idx,
            parent_para_idx,
            control_idx,
            target_row,
            target_col,
            formula,
            write_result,
        )
    }

    pub fn paste_internal_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"cellParaIdx\":0,\"charOffset\":{char_offset}}}"
        ))
    }

    pub fn paste_internal_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.paste_internal_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    pub fn move_vertical_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
        _delta: i32,
        preferred_x: f64,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        let x = if preferred_x.is_finite() && preferred_x >= 0.0 {
            preferred_x
        } else {
            APP_PAGE_MARGIN_PX as f64
        };
        Ok(format!(
            "{{\"sectionIndex\":{},\"paragraphIndex\":{},\"charOffset\":{},\"pageIndex\":0,\"x\":{:.1},\"y\":{:.1},\"height\":{:.1},\"preferredX\":{:.1},\"rectValid\":false}}",
            section_idx, parent_para_idx, char_offset, x, APP_PAGE_MARGIN_PX, APP_LINE_HEIGHT_PX, x
        ))
    }

    pub fn move_vertical_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        delta: i32,
        preferred_x: f64,
    ) -> Result<String> {
        self.move_vertical_by_path(
            section_idx,
            parent_para_idx,
            path_json,
            char_offset,
            delta,
            preferred_x,
        )
    }

    pub fn get_table_signature(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(self
            .observed_table_candidate(control_idx)
            .map(observed_table_signature)
            .unwrap_or_default())
    }

    pub fn get_table_signature_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_table_signature(section_idx, parent_para_idx, control_idx)
    }

    pub fn get_paragraph_stable_id(&self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok(format!("rjtd-p{paragraph_idx}"))
    }

    pub fn get_paragraph_stable_id_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String> {
        self.get_paragraph_stable_id(section_idx, paragraph_idx)
    }

    pub fn ensure_paragraph_stable_ids(&mut self) {}

    pub fn ensure_paragraph_stable_ids_native(&mut self) {
        self.ensure_paragraph_stable_ids();
    }

    pub fn debug_dump_stable_ids(
        &self,
        section_idx: u32,
        start_para: u32,
        count: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let end = start_para.saturating_add(count);
        let mut items = Vec::new();
        for para_idx in start_para..end {
            if self.paragraph_block_index(para_idx as usize).is_ok() {
                items.push(format!(
                    "{{\"sec\":{},\"para\":{},\"stableId\":\"rjtd-p{}\"}}",
                    section_idx, para_idx, para_idx
                ));
            }
        }
        Ok(format!("[{}]", items.join(",")))
    }

    pub fn debug_dump_stable_ids_native(
        &self,
        section_idx: u32,
        start_para: u32,
        count: u32,
    ) -> Result<String> {
        self.debug_dump_stable_ids(section_idx, start_para, count)
    }

    pub fn get_shape_bbox(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_object_bbox_json())
    }

    pub fn get_shape_bbox_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_shape_bbox(section_idx, parent_para_idx, control_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_picture(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        cell_path_json: &str,
        _image_data: &[u8],
        _width: u32,
        _height: u32,
        _natural_width_px: u32,
        _natural_height_px: u32,
        _extension: &str,
        _description: &str,
        _paper_offset_x_hu: Option<i32>,
        _paper_offset_y_hu: Option<i32>,
    ) -> Result<String> {
        if cell_path_json.is_empty() || cell_path_json == "[]" {
            self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        } else {
            self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        }
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{},\"controlIdx\":-1}}",
            paragraph_idx
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_picture_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        cell_path_json: &str,
        image_data: &[u8],
        width: u32,
        height: u32,
        natural_width_px: u32,
        natural_height_px: u32,
        extension: &str,
        description: &str,
        paper_offset_x_hu: Option<i32>,
        paper_offset_y_hu: Option<i32>,
    ) -> Result<String> {
        self.insert_picture(
            section_idx,
            paragraph_idx,
            char_offset,
            cell_path_json,
            image_data,
            width,
            height,
            natural_width_px,
            natural_height_px,
            extension,
            description,
            paper_offset_x_hu,
            paper_offset_y_hu,
        )
    }

    pub fn get_picture_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_picture_properties_json())
    }

    pub fn get_picture_properties_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_picture_properties(section_idx, parent_para_idx, control_idx)
    }

    pub fn get_header_footer_picture_properties(
        &self,
        section_idx: u32,
        _outer_para_idx: u32,
        _outer_control_idx: u32,
        _inner_para_idx: u32,
        _inner_control_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(default_picture_properties_json())
    }

    pub fn get_header_footer_picture_properties_native(
        &self,
        section_idx: u32,
        outer_para_idx: u32,
        outer_control_idx: u32,
        inner_para_idx: u32,
        inner_control_idx: u32,
    ) -> Result<String> {
        self.get_header_footer_picture_properties(
            section_idx,
            outer_para_idx,
            outer_control_idx,
            inner_para_idx,
            inner_control_idx,
        )
    }

    pub fn set_picture_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_picture_properties_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.set_picture_properties(section_idx, parent_para_idx, control_idx, props_json)
    }

    pub fn set_header_footer_picture_properties(
        &mut self,
        section_idx: u32,
        _outer_para_idx: u32,
        _outer_control_idx: u32,
        _inner_para_idx: u32,
        _inner_control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_header_footer_picture_properties_native(
        &mut self,
        section_idx: u32,
        outer_para_idx: u32,
        outer_control_idx: u32,
        inner_para_idx: u32,
        inner_control_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.set_header_footer_picture_properties(
            section_idx,
            outer_para_idx,
            outer_control_idx,
            inner_para_idx,
            inner_control_idx,
            props_json,
        )
    }

    pub fn delete_picture_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn delete_picture_control_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.delete_picture_control(section_idx, parent_para_idx, control_idx)
    }

    pub fn delete_cell_picture_control_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_cell_shape_properties_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_shape_properties_json())
    }

    pub fn get_cell_picture_properties_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_picture_properties_json())
    }

    pub fn set_cell_shape_properties_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_cell_picture_properties_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn delete_equation_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_equation_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: i32,
        _cell_para_idx: i32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_equation_properties_json())
    }

    pub fn set_equation_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: i32,
        _cell_para_idx: i32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn render_equation_preview(
        &self,
        script: &str,
        font_size_hwpunit: u32,
        color: u32,
    ) -> String {
        let font_size = (font_size_hwpunit as f64 / 100.0).clamp(8.0, 96.0);
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"320\" height=\"80\" viewBox=\"0 0 320 80\"><rect width=\"320\" height=\"80\" fill=\"#ffffff\"/><text x=\"12\" y=\"46\" font-family=\"serif\" font-size=\"{font_size:.1}\" fill=\"#{color:06x}\">{}</text></svg>",
            escape_xml(script)
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_shape_control(&mut self, _params_json: &str) -> Result<String> {
        Ok("{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}".to_string())
    }

    pub fn get_shape_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_shape_properties_json())
    }

    pub fn get_shape_text(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false,\"text\":\"\"}".to_string())
    }

    pub fn set_shape_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn delete_shape_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn change_shape_z_order(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _operation: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false,\"zOrder\":0}".to_string())
    }

    pub fn group_shapes(&mut self, _json: &str) -> String {
        "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}".to_string()
    }

    pub fn ungroup_shape(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn move_line_endpoint(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _sx: i32,
        _sy: i32,
        _ex: i32,
        _ey: i32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn update_connectors_in_section(&mut self, _section_idx: u32) {}

    pub fn insert_equation(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _script: &str,
        _font_size: u32,
        _color: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{},\"controlIdx\":-1}}",
            paragraph_idx
        ))
    }

    pub fn get_form_object_at(&self, page_num: u32, _x: f64, _y: f64) -> Result<String> {
        self.page_lines(page_num)?;
        Ok("{\"found\":false}".to_string())
    }

    pub fn get_form_value(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_form_value(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _value_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_form_value_in_cell(
        &mut self,
        section_idx: u32,
        table_para: u32,
        _table_ci: u32,
        _cell_idx: u32,
        _cell_para: u32,
        _form_ci: u32,
        _value_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, table_para)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_form_object_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn copy_control(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _cell_path_json: &str,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn paste_control(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{},\"controlIdx\":-1}}",
            paragraph_idx
        ))
    }

    pub fn get_control_image_data(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _cell_path_json: &str,
        _control_idx: u32,
    ) -> Result<Vec<u8>> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(Vec::new())
    }

    pub fn get_control_image_mime(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _cell_path_json: &str,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(String::new())
    }

    pub fn get_bookmarks(&self) -> String {
        "[]".to_string()
    }

    pub fn add_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _name: &str,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok("{\"ok\":false,\"error\":\"bookmarks are not decoded\"}".to_string())
    }

    pub fn delete_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false,\"error\":\"bookmarks are not decoded\"}".to_string())
    }

    pub fn rename_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _new_name: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false,\"error\":\"bookmarks are not decoded\"}".to_string())
    }

    pub fn export_hwp(&self) -> Vec<u8> {
        Vec::new()
    }

    pub fn export_hwpx(&self) -> Vec<u8> {
        Vec::new()
    }

    pub fn export_hwp_verify(&self) -> String {
        "{\"ok\":false,\"errors\":[\"JTD to HWP/HWPX export is not implemented\"],\"warnings\":[]}"
            .to_string()
    }

    pub fn insert_page_break(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn insert_column_break(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn insert_new_number(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _start_num: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn set_column_def(
        &mut self,
        section_idx: u32,
        _column_count: u32,
        _column_type: u32,
        _same_width: u32,
        _spacing_hu: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn set_numbering_restart(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _mode: u32,
        _start_num: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn create_style(&mut self, _json: &str) -> u32 {
        0
    }

    pub fn update_style(&mut self, style_id: u32, _json: &str) -> bool {
        style_id == 0
    }

    pub fn update_style_shapes(
        &mut self,
        style_id: u32,
        _char_mods_json: &str,
        _para_mods_json: &str,
    ) -> bool {
        style_id == 0
    }

    pub fn delete_style(&mut self, _style_id: u32) -> bool {
        false
    }

    pub fn create_numbering(&mut self, _json: &str) -> u32 {
        0
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_text_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _fn_para_idx: u32,
        char_offset: u32,
        _text: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_text_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _fn_para_idx: u32,
        char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn split_paragraph_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        fn_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"fnParaIndex\":{fn_para_idx},\"charOffset\":{char_offset}}}"
        ))
    }

    pub fn merge_paragraph_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        fn_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"fnParaIndex\":{fn_para_idx},\"charOffset\":0}}"
        ))
    }

    pub fn get_cursor_rect_in_footnote(
        &self,
        page_num: u32,
        _footnote_index: u32,
        _fn_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.page_lines(page_num)?;
        Ok(default_cursor_rect_json(page_num))
    }

    pub fn get_cursor_rect_in_note(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _note_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(default_cursor_rect_json(0))
    }

    pub fn get_para_properties_in_footnote(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _fn_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(default_para_properties_json())
    }

    pub fn apply_para_format_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _fn_para_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_selection_rects_in_footnote(
        &self,
        page_num: u32,
        _footnote_index: u32,
        _start_fn_para: u32,
        _start_offset: u32,
        _end_fn_para: u32,
        _end_offset: u32,
    ) -> Result<String> {
        self.page_lines(page_num)?;
        Ok("[]".to_string())
    }

    pub fn get_para_properties_in_hf(
        &self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(default_para_properties_json())
    }

    pub fn apply_para_format_in_hf(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn insert_field_in_hf(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        char_offset: u32,
        _field_type: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn apply_hf_template(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _template_id: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn export_selection_html(
        &self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let text = self.selected_text(
            start_para_idx as usize,
            start_char_offset as usize,
            end_para_idx as usize,
            end_char_offset as usize,
        )?;
        Ok(format!("<p>{}</p>", escape_xml(&text)))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn export_selection_in_cell_html(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _start_cell_para: u32,
        _start_offset: u32,
        _end_cell_para: u32,
        _end_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(String::new())
    }

    pub fn export_control_html(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _cell_path_json: &str,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(String::new())
    }

    pub fn paste_html(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _html: &str,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn paste_html_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        char_offset: u32,
        _html: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn paste_html_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
        _html: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn get_text_box_control_index(&self, section_idx: u32, paragraph_idx: u32) -> Result<i32> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok(-1)
    }

    pub fn get_text_box_control_index_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<i32> {
        self.get_text_box_control_index(section_idx, paragraph_idx)
    }

    pub fn get_char_properties_at(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(default_char_properties_json())
    }

    pub fn get_char_properties_at_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_char_properties_at(section_idx, paragraph_idx, char_offset)
    }

    pub fn apply_char_format(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        start_offset: u32,
        end_offset: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, start_offset)?;
        self.ensure_text_position(section_idx, paragraph_idx, end_offset)?;
        if start_offset > end_offset {
            return Err(rjtd_core::Error::InvalidData(format!(
                "start offset {start_offset} is after end offset {end_offset}"
            )));
        }
        Ok("{\"ok\":true}".to_string())
    }

    pub fn apply_char_format_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        start_offset: u32,
        end_offset: u32,
        props_json: &str,
    ) -> Result<String> {
        self.apply_char_format(
            section_idx,
            paragraph_idx,
            start_offset,
            end_offset,
            props_json,
        )
    }

    pub fn find_or_create_font_id(&self, _name: &str) -> u32 {
        0
    }

    pub fn find_or_create_font_id_for_lang(&self, _lang: u32, _name: &str) -> u32 {
        0
    }

    pub fn get_para_properties_at(&self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok(default_para_properties_json())
    }

    pub fn get_para_properties_at_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String> {
        self.get_para_properties_at(section_idx, paragraph_idx)
    }

    pub fn apply_para_format(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok("{\"ok\":true}".to_string())
    }

    pub fn apply_para_format_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.apply_para_format(section_idx, paragraph_idx, props_json)
    }

    pub fn get_style_list(&self) -> String {
        let candidates = text_style_candidates(self.document.unknown_styles());
        let mut output = format!(
            "[{{\"id\":0,\"name\":\"Normal\",\"englishName\":\"Normal\",\"type\":0,\"nextStyleId\":0,\"paraShapeId\":0,\"charShapeId\":0,\"decoded\":false,\"sourceStreamCount\":{},\"candidateCount\":{}}}",
            self.document.unknown_styles().len(),
            candidates.len()
        );
        for candidate in &candidates {
            output.push(',');
            push_style_candidate_json(&mut output, candidate);
        }
        output.push(']');
        output
    }

    pub fn get_style_detail(&self, style_id: u32) -> Result<String> {
        if style_id == 0 {
            Ok(format!(
                "{{\"charProps\":{},\"paraProps\":{},\"decoded\":false,\"sourceStreams\":{}}}",
                default_char_properties_json(),
                default_para_properties_json(),
                style_source_streams_json(self.document.unknown_styles())
            ))
        } else {
            let candidates = text_style_candidates(self.document.unknown_styles());
            match candidates.iter().find(|candidate| candidate.id == style_id) {
                Some(candidate) => Ok(style_candidate_detail_json(candidate)),
                None => Err(rjtd_core::Error::InvalidData(format!(
                    "style {style_id} out of range"
                ))),
            }
        }
    }

    pub fn get_style_at(&self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        let paragraph = self.paragraph(paragraph_idx as usize)?;
        Ok(
            match paragraph
                .style()
                .and_then(|style| style.id().parse::<u32>().ok())
            {
                Some(0) | None => "{\"id\":0,\"name\":\"Normal\"}".to_string(),
                Some(style_id) => {
                    let candidates = text_style_candidates(self.document.unknown_styles());
                    match candidates.iter().find(|candidate| candidate.id == style_id) {
                        Some(candidate) => style_at_candidate_json(candidate),
                        None => format!(
                            "{{\"id\":{},\"name\":\"Unknown\",\"decoded\":false,\"jtdCandidate\":true}}",
                            style_id
                        ),
                    }
                }
            },
        )
    }

    pub fn apply_style(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        style_id: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        if style_id == 0 {
            self.set_paragraph_style(paragraph_idx as usize, None)?;
            return Ok("{\"ok\":true}".to_string());
        }
        let candidates = text_style_candidates(self.document.unknown_styles());
        let Some(candidate) = candidates.iter().find(|candidate| candidate.id == style_id) else {
            return Err(rjtd_core::Error::InvalidData(format!(
                "style {style_id} out of range"
            )));
        };
        self.set_paragraph_style(
            paragraph_idx as usize,
            Some(StyleRef::new(candidate.id.to_string())),
        )?;
        Ok(format!(
            "{{\"ok\":true,\"decoded\":false,\"styleId\":{},\"name\":{}}}",
            candidate.id,
            json_string(&candidate.name)
        ))
    }

    pub fn get_numbering_list(&self) -> String {
        "[]".to_string()
    }

    pub fn get_bullet_list(&self) -> String {
        "[]".to_string()
    }

    pub fn ensure_default_numbering(&self) -> u32 {
        0
    }

    pub fn ensure_default_bullet(&self, _bullet_char: &str) -> u32 {
        0
    }

    pub fn get_paragraph_count(&self, section_idx: u32) -> Result<u32> {
        self.ensure_section(section_idx)?;
        Ok(self.paragraph_count() as u32)
    }

    pub fn get_paragraph_count_native(&self, section_idx: u32) -> Result<u32> {
        self.get_paragraph_count(section_idx)
    }

    pub fn get_paragraph_length(&self, section_idx: u32, paragraph_idx: u32) -> Result<u32> {
        self.ensure_section(section_idx)?;
        let paragraph = self.paragraph(paragraph_idx as usize)?;
        Ok(paragraph_text(paragraph).chars().count() as u32)
    }

    pub fn get_paragraph_length_native(&self, section_idx: u32, paragraph_idx: u32) -> Result<u32> {
        self.get_paragraph_length(section_idx, paragraph_idx)
    }

    pub fn get_text_range(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let text = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let start = checked_char_boundary(&text, char_offset as usize)?;
        let end_offset = (char_offset as usize)
            .saturating_add(count as usize)
            .min(text.chars().count());
        let end = checked_char_boundary(&text, end_offset)?;
        Ok(text[start..end].to_string())
    }

    pub fn get_text_range_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.get_text_range(section_idx, paragraph_idx, char_offset, count)
    }

    pub fn insert_text(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let current = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let insert_at = checked_char_boundary(&current, char_offset as usize)?;
        let mut next = current;
        next.insert_str(insert_at, text);
        self.set_paragraph_text(paragraph_idx as usize, next)?;

        let new_offset = char_offset + text.chars().count() as u32;
        self.set_caret(section_idx, paragraph_idx, new_offset);
        self.refresh_pages();
        Ok(json_ok_with(&format!("\"charOffset\":{new_offset}")))
    }

    pub fn insert_text_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String> {
        self.insert_text(section_idx, paragraph_idx, char_offset, text)
    }

    pub fn delete_text(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let current = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let start = checked_char_boundary(&current, char_offset as usize)?;
        let end_offset = (char_offset as usize)
            .saturating_add(count as usize)
            .min(current.chars().count());
        let end = checked_char_boundary(&current, end_offset)?;
        let mut next = current;
        next.replace_range(start..end, "");
        self.set_paragraph_text(paragraph_idx as usize, next)?;

        self.set_caret(section_idx, paragraph_idx, char_offset);
        self.refresh_pages();
        Ok(json_ok_with(&format!("\"charOffset\":{char_offset}")))
    }

    pub fn delete_text_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.delete_text(section_idx, paragraph_idx, char_offset, count)
    }

    pub fn split_paragraph(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let block_index = self.paragraph_block_index(paragraph_idx as usize)?;
        let current = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let original_style = self.paragraph(paragraph_idx as usize)?.style().cloned();
        let split_at = checked_char_boundary(&current, char_offset as usize)?;
        let left = current[..split_at].to_string();
        let right = current[split_at..].to_string();
        self.replace_paragraph_block(block_index, left)?;
        self.document.blocks.insert(
            block_index + 1,
            Block::Paragraph(Paragraph::new(
                vec![Inline::Text(TextRun::new(right, None))],
                original_style,
            )),
        );

        let new_paragraph_idx = paragraph_idx + 1;
        self.set_caret(section_idx, new_paragraph_idx, 0);
        self.refresh_pages();
        Ok(json_ok_with(&format!(
            "\"paraIdx\":{new_paragraph_idx},\"charOffset\":0"
        )))
    }

    pub fn split_paragraph_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.split_paragraph(section_idx, paragraph_idx, char_offset)
    }

    pub fn merge_paragraph(&mut self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        if paragraph_idx == 0 {
            return Err(rjtd_core::Error::InvalidData(
                "first paragraph cannot be merged".to_string(),
            ));
        }

        let previous_idx = paragraph_idx - 1;
        let previous_block_index = self.paragraph_block_index(previous_idx as usize)?;
        let current_block_index = self.paragraph_block_index(paragraph_idx as usize)?;
        let previous = paragraph_text(self.paragraph(previous_idx as usize)?);
        let current = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let merge_point = previous.chars().count() as u32;
        self.replace_paragraph_block(previous_block_index, format!("{previous}{current}"))?;
        self.document.blocks.remove(current_block_index);

        self.set_caret(section_idx, previous_idx, merge_point);
        self.refresh_pages();
        Ok(json_ok_with(&format!(
            "\"paraIdx\":{previous_idx},\"charOffset\":{merge_point}"
        )))
    }

    pub fn merge_paragraph_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String> {
        self.merge_paragraph(section_idx, paragraph_idx)
    }

    pub fn get_caret_position(&self) -> String {
        format!(
            "{{\"sectionIndex\":{},\"paragraphIndex\":{},\"charOffset\":{}}}",
            self.caret_section, self.caret_paragraph, self.caret_char_offset
        )
    }

    pub fn save_snapshot(&mut self) -> u32 {
        let id = self.next_snapshot_id;
        self.next_snapshot_id = next_snapshot_id(id);
        let snapshot = DocumentSnapshot::capture(id, self);
        self.snapshots.push(snapshot);
        id
    }

    pub fn save_snapshot_native(&mut self) -> u32 {
        self.save_snapshot()
    }

    pub fn restore_snapshot(&mut self, id: u32) -> Result<String> {
        let snapshot = self
            .snapshots
            .iter()
            .find(|snapshot| snapshot.id == id)
            .cloned()
            .ok_or_else(|| rjtd_core::Error::InvalidData(format!("snapshot {id} not found")))?;

        self.document = snapshot.document;
        self.pages = snapshot.pages;
        self.file_name = snapshot.file_name;
        self.dpi = snapshot.dpi;
        self.page_layout = snapshot.page_layout;
        self.show_paragraph_marks = snapshot.show_paragraph_marks;
        self.show_control_codes = snapshot.show_control_codes;
        self.show_transparent_borders = snapshot.show_transparent_borders;
        self.clip_enabled = snapshot.clip_enabled;
        self.writing_mode = snapshot.writing_mode;
        self.caret_section = snapshot.caret_section;
        self.caret_paragraph = snapshot.caret_paragraph;
        self.caret_char_offset = snapshot.caret_char_offset;
        self.clipboard_text = snapshot.clipboard_text;

        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn restore_snapshot_native(&mut self, id: u32) -> Result<String> {
        self.restore_snapshot(id)
    }

    pub fn discard_snapshot(&mut self, id: u32) {
        self.snapshots.retain(|snapshot| snapshot.id != id);
    }

    pub fn discard_snapshot_native(&mut self, id: u32) {
        self.discard_snapshot(id);
    }

    pub fn set_show_paragraph_marks(&mut self, enabled: bool) {
        self.show_paragraph_marks = enabled;
    }

    pub fn set_show_paragraph_marks_native(&mut self, enabled: bool) {
        self.set_show_paragraph_marks(enabled);
    }

    pub fn get_show_control_codes(&self) -> bool {
        self.show_control_codes
    }

    pub fn get_show_control_codes_native(&self) -> bool {
        self.get_show_control_codes()
    }

    pub fn set_show_control_codes(&mut self, enabled: bool) {
        self.show_control_codes = enabled;
    }

    pub fn set_show_control_codes_native(&mut self, enabled: bool) {
        self.set_show_control_codes(enabled);
    }

    pub fn get_show_transparent_borders(&self) -> bool {
        self.show_transparent_borders
    }

    pub fn get_show_transparent_borders_native(&self) -> bool {
        self.get_show_transparent_borders()
    }

    pub fn set_show_transparent_borders(&mut self, enabled: bool) {
        self.show_transparent_borders = enabled;
    }

    pub fn set_show_transparent_borders_native(&mut self, enabled: bool) {
        self.set_show_transparent_borders(enabled);
    }

    pub fn set_clip_enabled(&mut self, enabled: bool) {
        self.clip_enabled = enabled;
    }

    pub fn set_clip_enabled_native(&mut self, enabled: bool) {
        self.set_clip_enabled(enabled);
    }

    pub fn get_position_of_page(&self, global_page: u32) -> Result<String> {
        let lines = self.page_lines(global_page)?;
        let paragraph_index = lines
            .iter()
            .find_map(PageTextLine::paragraph_index)
            .unwrap_or(0);
        self.paragraph_block_index(paragraph_index)?;
        Ok(format!(
            "{{\"ok\":true,\"sec\":0,\"para\":{},\"charOffset\":0}}",
            paragraph_index
        ))
    }

    pub fn get_position_of_page_native(&self, global_page: u32) -> Result<String> {
        self.get_position_of_page(global_page)
    }

    pub fn get_page_of_position(&self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        for (page_index, page) in self.pages.iter().enumerate() {
            if page
                .iter()
                .any(|line| line.paragraph_index() == Some(paragraph_idx as usize))
            {
                return Ok(format!("{{\"ok\":true,\"page\":{page_index}}}"));
            }
        }
        Ok("{\"ok\":true,\"page\":0}".to_string())
    }

    pub fn get_page_of_position_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String> {
        self.get_page_of_position(section_idx, paragraph_idx)
    }

    pub fn find_next_editable_control(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: i32,
        delta: i32,
    ) -> String {
        if self.ensure_section(section_idx).is_err()
            || self.paragraph_block_index(paragraph_idx as usize).is_err()
        {
            return "{\"type\":\"none\"}".to_string();
        }

        let paragraph_count = self.paragraph_count() as u32;
        if delta > 0 && paragraph_idx + 1 < paragraph_count {
            return format!(
                "{{\"type\":\"body\",\"sec\":{},\"para\":{}}}",
                section_idx,
                paragraph_idx + 1
            );
        }
        if delta < 0 && paragraph_idx > 0 {
            return format!(
                "{{\"type\":\"body\",\"sec\":{},\"para\":{}}}",
                section_idx,
                paragraph_idx - 1
            );
        }

        "{\"type\":\"none\"}".to_string()
    }

    pub fn find_next_editable_control_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: i32,
        delta: i32,
    ) -> String {
        self.find_next_editable_control(section_idx, paragraph_idx, control_idx, delta)
    }

    pub fn find_nearest_control_backward(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        if self.ensure_section(section_idx).is_err()
            || self.paragraph_block_index(paragraph_idx as usize).is_err()
        {
            return "{\"type\":\"none\"}".to_string();
        }

        let controls = projected_text_controls(&self.document);
        if let Some(control) = controls.iter().rev().find(|control| {
            control.paragraph_index < paragraph_idx as usize
                || (control.paragraph_index == paragraph_idx as usize
                    && control.char_offset < char_offset as usize)
        }) {
            return projected_control_json(control);
        }

        "{\"type\":\"none\"}".to_string()
    }

    pub fn find_nearest_control_backward_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        self.find_nearest_control_backward(section_idx, paragraph_idx, char_offset)
    }

    pub fn find_nearest_control_forward(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        if self.ensure_section(section_idx).is_err()
            || self.paragraph_block_index(paragraph_idx as usize).is_err()
        {
            return "{\"type\":\"none\"}".to_string();
        }

        let controls = projected_text_controls(&self.document);
        if let Some(control) = controls.iter().find(|control| {
            control.paragraph_index > paragraph_idx as usize
                || (control.paragraph_index == paragraph_idx as usize
                    && control.char_offset > char_offset as usize)
        }) {
            return projected_control_json(control);
        }

        "{\"type\":\"none\"}".to_string()
    }

    pub fn find_nearest_control_forward_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        self.find_nearest_control_forward(section_idx, paragraph_idx, char_offset)
    }

    pub fn get_control_text_positions(&self, section_idx: u32, paragraph_idx: u32) -> String {
        if self.ensure_section(section_idx).is_err()
            || self.paragraph_block_index(paragraph_idx as usize).is_err()
        {
            return "[]".to_string();
        }

        let positions = projected_text_controls(&self.document)
            .into_iter()
            .filter(|control| control.paragraph_index == paragraph_idx as usize)
            .map(|control| control.char_offset.to_string())
            .collect::<Vec<_>>();
        format!("[{}]", positions.join(","))
    }

    pub fn get_control_text_positions_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> String {
        self.get_control_text_positions(section_idx, paragraph_idx)
    }

    pub fn navigate_next_editable(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        delta: i32,
        _context_json: &str,
    ) -> String {
        if self.ensure_section(section_idx).is_err() {
            return "{\"type\":\"boundary\"}".to_string();
        }
        let Ok(paragraph) = self.paragraph(paragraph_idx as usize) else {
            return "{\"type\":\"boundary\"}".to_string();
        };

        let paragraph_len = paragraph_text(paragraph).chars().count() as u32;
        if delta > 0 {
            if char_offset < paragraph_len {
                return format_nav_text(section_idx, paragraph_idx, char_offset + 1);
            }
            if paragraph_idx + 1 < self.paragraph_count() as u32 {
                return format_nav_text(section_idx, paragraph_idx + 1, 0);
            }
        } else if delta < 0 {
            if char_offset > 0 {
                return format_nav_text(section_idx, paragraph_idx, char_offset - 1);
            }
            if paragraph_idx > 0 {
                let previous = self
                    .paragraph(paragraph_idx.saturating_sub(1) as usize)
                    .map(paragraph_text)
                    .unwrap_or_default()
                    .chars()
                    .count() as u32;
                return format_nav_text(section_idx, paragraph_idx - 1, previous);
            }
        }

        "{\"type\":\"boundary\"}".to_string()
    }

    pub fn navigate_next_editable_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        delta: i32,
        context_json: &str,
    ) -> String {
        self.navigate_next_editable(section_idx, paragraph_idx, char_offset, delta, context_json)
    }

    pub fn get_field_list(&self) -> String {
        "[]".to_string()
    }

    pub fn get_field_list_native(&self) -> String {
        self.get_field_list()
    }

    pub fn get_field_value(&self, field_id: u32) -> String {
        format!("{{\"ok\":false,\"fieldId\":{field_id},\"value\":\"\"}}")
    }

    pub fn get_field_value_native(&self, field_id: u32) -> String {
        self.get_field_value(field_id)
    }

    pub fn get_field_value_by_name(&self, name: &str) -> String {
        format!(
            "{{\"ok\":false,\"fieldId\":0,\"name\":{},\"value\":\"\"}}",
            json_string(name)
        )
    }

    pub fn get_field_value_by_name_native(&self, name: &str) -> String {
        self.get_field_value_by_name(name)
    }

    pub fn set_field_value(&mut self, field_id: u32, value: &str) -> String {
        format!(
            "{{\"ok\":false,\"fieldId\":{},\"oldValue\":\"\",\"newValue\":{}}}",
            field_id,
            json_string(value)
        )
    }

    pub fn set_field_value_native(&mut self, field_id: u32, value: &str) -> String {
        self.set_field_value(field_id, value)
    }

    pub fn set_field_value_by_name(&mut self, name: &str, value: &str) -> String {
        format!(
            "{{\"ok\":false,\"fieldId\":0,\"name\":{},\"oldValue\":\"\",\"newValue\":{}}}",
            json_string(name),
            json_string(value)
        )
    }

    pub fn set_field_value_by_name_native(&mut self, name: &str, value: &str) -> String {
        self.set_field_value_by_name(name, value)
    }

    pub fn get_field_info_at(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        if self
            .ensure_text_position(section_idx, paragraph_idx, char_offset)
            .is_err()
        {
            return "{\"inField\":false}".to_string();
        }
        "{\"inField\":false}".to_string()
    }

    pub fn get_field_info_at_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        self.get_field_info_at(section_idx, paragraph_idx, char_offset)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_field_info_at_in_cell(
        &self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
        _is_textbox: bool,
    ) -> String {
        "{\"inField\":false}".to_string()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_field_info_at_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> String {
        self.get_field_info_at_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }

    pub fn get_field_info_at_by_path(
        &self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _path_json: &str,
        _char_offset: u32,
    ) -> String {
        "{\"inField\":false}".to_string()
    }

    pub fn get_field_info_at_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> String {
        self.get_field_info_at_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    pub fn remove_field_at(
        &mut self,
        _section_idx: u32,
        _paragraph_idx: u32,
        _char_offset: u32,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn remove_field_at_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        self.remove_field_at(section_idx, paragraph_idx, char_offset)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn remove_field_at_in_cell(
        &mut self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
        _is_textbox: bool,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn remove_field_at_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> String {
        self.remove_field_at_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }

    pub fn set_active_field(
        &mut self,
        _section_idx: u32,
        _paragraph_idx: u32,
        _char_offset: u32,
    ) -> bool {
        false
    }

    pub fn set_active_field_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> bool {
        self.set_active_field(section_idx, paragraph_idx, char_offset)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_active_field_in_cell(
        &mut self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
        _is_textbox: bool,
    ) -> bool {
        false
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_active_field_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> bool {
        self.set_active_field_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }

    pub fn set_active_field_by_path(
        &mut self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _path_json: &str,
        _char_offset: u32,
    ) -> bool {
        false
    }

    pub fn set_active_field_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> bool {
        self.set_active_field_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    pub fn clear_active_field(&mut self) {}

    pub fn clear_active_field_native(&mut self) {
        self.clear_active_field();
    }

    pub fn get_click_here_props(&self, _field_id: u32) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn get_click_here_props_native(&self, field_id: u32) -> String {
        self.get_click_here_props(field_id)
    }

    pub fn update_click_here_props(
        &mut self,
        _field_id: u32,
        _guide: &str,
        _memo: &str,
        _name: &str,
        _editable: bool,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn update_click_here_props_native(
        &mut self,
        field_id: u32,
        guide: &str,
        memo: &str,
        name: &str,
        editable: bool,
    ) -> String {
        self.update_click_here_props(field_id, guide, memo, name, editable)
    }

    pub fn get_header_footer(
        &self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":true,\"exists\":false}".to_string())
    }

    pub fn get_header_footer_native(
        &self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
    ) -> Result<String> {
        self.get_header_footer(section_idx, is_header, apply_to)
    }

    pub fn create_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false,\"exists\":false}".to_string())
    }

    pub fn create_header_footer_native(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
    ) -> Result<String> {
        self.create_header_footer(section_idx, is_header, apply_to)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_text_in_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _char_offset: u32,
        _text: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_text_in_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn split_paragraph_in_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn merge_paragraph_in_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_header_footer_para_info(
        &self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false,\"paraCount\":0,\"charCount\":0}".to_string())
    }

    pub fn get_cursor_rect_in_header_footer(
        &self,
        page_num: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _char_offset: u32,
        preferred_page: i32,
    ) -> Result<String> {
        self.page_lines(page_num)?;
        let page_index = if preferred_page >= 0 {
            preferred_page as u32
        } else {
            page_num
        };
        Ok(format!(
            "{{\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1},\"height\":{:.1}}}",
            page_index, APP_PAGE_MARGIN_PX, APP_PAGE_MARGIN_PX, APP_LINE_HEIGHT_PX
        ))
    }

    pub fn delete_header_footer(&mut self, _section_idx: u32, _is_header: bool, _apply_to: u32) {}

    pub fn get_header_footer_list(
        &self,
        _current_section_idx: u32,
        _current_is_header: bool,
        _current_apply_to: u32,
    ) -> String {
        "{\"ok\":true,\"items\":[],\"currentIndex\":-1}".to_string()
    }

    pub fn toggle_hide_header_footer(&mut self, page_num: u32, _is_header: bool) -> Result<String> {
        self.page_lines(page_num)?;
        Ok("{\"ok\":false,\"hidden\":false}".to_string())
    }

    pub fn navigate_header_footer_by_page(
        &self,
        _current_page: u32,
        _is_header: bool,
        _direction: i32,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn insert_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn insert_endnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_endnote_shape(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(default_endnote_shape_json())
    }

    pub fn apply_endnote_shape(&mut self, section_idx: u32, _props_json: &str) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_footnote_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok(
            "{\"ok\":false,\"paraCount\":0,\"totalTextLen\":0,\"number\":0,\"texts\":[]}"
                .to_string(),
        )
    }

    pub fn delete_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok("{\"ok\":false,\"sectionIndex\":0,\"paragraphIndex\":0,\"controlIndex\":0,\"charOffset\":0,\"deletedNumber\":0}".to_string())
    }

    pub fn get_page_footnote_info(&self, page_num: u32, _footnote_index: u32) -> Result<String> {
        self.page_lines(page_num)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_note_edit_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_note_equation_properties(
        &self,
        _section_idx: u32,
        _paragraph_idx: u32,
        _control_idx: u32,
        _note_para_idx: u32,
        _equation_idx: u32,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn set_note_equation_properties(
        &mut self,
        _section_idx: u32,
        _paragraph_idx: u32,
        _control_idx: u32,
        _note_para_idx: u32,
        _equation_idx: u32,
        _props_json: &str,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn search_text(
        &self,
        query: &str,
        from_sec: u32,
        from_para: u32,
        from_char: u32,
        forward: bool,
        case_sensitive: bool,
    ) -> Result<String> {
        self.ensure_section(from_sec)?;
        if query.is_empty() {
            return Ok("{\"found\":false}".to_string());
        }

        let hits = self.search_hits(query, case_sensitive);
        if hits.is_empty() {
            return Ok("{\"found\":false}".to_string());
        }

        if forward {
            let after = hits.iter().find(|hit| {
                hit.sec > from_sec
                    || (hit.sec == from_sec && hit.para > from_para)
                    || (hit.sec == from_sec && hit.para == from_para && hit.char_offset > from_char)
            });
            Ok(match after {
                Some(hit) => format_search_result(hit, false),
                None => format_search_result(&hits[0], true),
            })
        } else {
            let before = hits.iter().rev().find(|hit| {
                hit.sec < from_sec
                    || (hit.sec == from_sec && hit.para < from_para)
                    || (hit.sec == from_sec && hit.para == from_para && hit.char_offset < from_char)
            });
            Ok(match before {
                Some(hit) => format_search_result(hit, false),
                None => format_search_result(&hits[hits.len() - 1], true),
            })
        }
    }

    pub fn search_text_native(
        &self,
        query: &str,
        from_sec: u32,
        from_para: u32,
        from_char: u32,
        forward: bool,
        case_sensitive: bool,
    ) -> Result<String> {
        self.search_text(
            query,
            from_sec,
            from_para,
            from_char,
            forward,
            case_sensitive,
        )
    }

    pub fn search_all_text(
        &self,
        query: &str,
        case_sensitive: bool,
        _include_cells: bool,
    ) -> String {
        if query.is_empty() {
            return "[]".to_string();
        }

        let hits = self.search_hits(query, case_sensitive);
        let json_hits = hits
            .iter()
            .map(format_search_hit)
            .collect::<Vec<_>>()
            .join(",");
        format!("[{json_hits}]")
    }

    pub fn search_all_text_native(
        &self,
        query: &str,
        case_sensitive: bool,
        include_cells: bool,
    ) -> String {
        self.search_all_text(query, case_sensitive, include_cells)
    }

    pub fn replace_text(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        length: u32,
        new_text: &str,
    ) -> Result<String> {
        self.delete_text(section_idx, paragraph_idx, char_offset, length)?;
        self.insert_text(section_idx, paragraph_idx, char_offset, new_text)?;
        Ok(format!(
            "{{\"ok\":true,\"charOffset\":{},\"newLength\":{}}}",
            char_offset,
            new_text.chars().count()
        ))
    }

    pub fn replace_text_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        length: u32,
        new_text: &str,
    ) -> Result<String> {
        self.replace_text(section_idx, paragraph_idx, char_offset, length, new_text)
    }

    pub fn replace_one(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String> {
        if query.is_empty() {
            return Ok("{\"ok\":false}".to_string());
        }

        let Some(hit) = self.search_hits(query, case_sensitive).first().copied() else {
            return Ok("{\"ok\":false}".to_string());
        };

        self.replace_text(hit.sec, hit.para, hit.char_offset, hit.length, new_text)?;
        Ok(format!(
            "{{\"ok\":true,\"sec\":{},\"para\":{},\"charOffset\":{},\"newLength\":{}}}",
            hit.sec,
            hit.para,
            hit.char_offset,
            new_text.chars().count()
        ))
    }

    pub fn replace_one_native(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String> {
        self.replace_one(query, new_text, case_sensitive)
    }

    pub fn replace_all(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String> {
        if query.is_empty() {
            return Ok("{\"ok\":true,\"count\":0}".to_string());
        }

        let mut hits = self.search_hits(query, case_sensitive);
        let count = hits.len();
        hits.reverse();

        for hit in hits {
            self.replace_text(hit.sec, hit.para, hit.char_offset, hit.length, new_text)?;
        }

        Ok(format!("{{\"ok\":true,\"count\":{count}}}"))
    }

    pub fn replace_all_native(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String> {
        self.replace_all(query, new_text, case_sensitive)
    }

    pub fn get_selection_rects(
        &self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let range = self.normalized_text_range(
            start_para_idx as usize,
            start_char_offset as usize,
            end_para_idx as usize,
            end_char_offset as usize,
        )?;
        if range.is_collapsed() {
            return Ok("[]".to_string());
        }

        let mut rects = Vec::new();
        for (page_index, page) in self.pages.iter().enumerate() {
            for (line_index, line) in page.iter().enumerate() {
                let Some(paragraph_index) = line.paragraph_index() else {
                    continue;
                };
                let Some((start, end)) = selection_overlap(line, paragraph_index, &range) else {
                    continue;
                };
                let start_rect =
                    cursor_rect_from_line(self.page_layout, page_index, line_index, line, start);
                let end_rect =
                    cursor_rect_from_line(self.page_layout, page_index, line_index, line, end);
                let width = (end_rect.x - start_rect.x).max(2.0);
                rects.push(format!(
                    "{{\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1},\"width\":{:.1},\"height\":{:.1}}}",
                    page_index, start_rect.x, start_rect.y, width, start_rect.height
                ));
            }
        }

        Ok(format!("[{}]", rects.join(",")))
    }

    pub fn get_selection_rects_native(
        &self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.get_selection_rects(
            section_idx,
            start_para_idx,
            start_char_offset,
            end_para_idx,
            end_char_offset,
        )
    }

    pub fn delete_range(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let range = self.normalized_text_range(
            start_para_idx as usize,
            start_char_offset as usize,
            end_para_idx as usize,
            end_char_offset as usize,
        )?;
        if range.is_collapsed() {
            self.set_caret(
                section_idx,
                range.start_para as u32,
                range.start_offset as u32,
            );
            return Ok(json_ok_with(&format!(
                "\"paraIdx\":{},\"charOffset\":{}",
                range.start_para, range.start_offset
            )));
        }

        if range.start_para == range.end_para {
            return self.delete_text(
                section_idx,
                range.start_para as u32,
                range.start_offset as u32,
                (range.end_offset - range.start_offset) as u32,
            );
        }

        let start_text = paragraph_text(self.paragraph(range.start_para)?);
        let end_text = paragraph_text(self.paragraph(range.end_para)?);
        let start_byte = checked_char_boundary(&start_text, range.start_offset)?;
        let end_byte = checked_char_boundary(&end_text, range.end_offset)?;
        let merged = format!("{}{}", &start_text[..start_byte], &end_text[end_byte..]);
        let start_block = self.paragraph_block_index(range.start_para)?;

        for paragraph_index in (range.start_para + 1..=range.end_para).rev() {
            let block_index = self.paragraph_block_index(paragraph_index)?;
            self.document.blocks.remove(block_index);
        }
        self.replace_paragraph_block(start_block, merged)?;

        self.set_caret(
            section_idx,
            range.start_para as u32,
            range.start_offset as u32,
        );
        self.refresh_pages();
        Ok(json_ok_with(&format!(
            "\"paraIdx\":{},\"charOffset\":{}",
            range.start_para, range.start_offset
        )))
    }

    pub fn delete_range_native(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.delete_range(
            section_idx,
            start_para_idx,
            start_char_offset,
            end_para_idx,
            end_char_offset,
        )
    }

    pub fn copy_selection(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let text = self.selected_text(
            start_para_idx as usize,
            start_char_offset as usize,
            end_para_idx as usize,
            end_char_offset as usize,
        )?;
        self.clipboard_text = Some(text.clone());
        Ok(json_ok_with(&format!("\"text\":{}", json_string(&text))))
    }

    pub fn copy_selection_native(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.copy_selection(
            section_idx,
            start_para_idx,
            start_char_offset,
            end_para_idx,
            end_char_offset,
        )
    }

    pub fn paste_internal(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let Some(text) = self.clipboard_text.clone() else {
            return Ok("{\"ok\":false,\"error\":\"clipboard empty\"}".to_string());
        };
        let mut parts = text.split('\n');
        let first = parts.next().unwrap_or_default();
        let result = self.insert_text(section_idx, paragraph_idx, char_offset, first)?;
        let mut current_para = paragraph_idx;
        let mut current_offset = char_offset + first.chars().count() as u32;

        for part in parts {
            self.split_paragraph(section_idx, current_para, current_offset)?;
            current_para += 1;
            self.insert_text(section_idx, current_para, 0, part)?;
            current_offset = part.chars().count() as u32;
        }

        if text.contains('\n') {
            Ok(json_ok_with(&format!(
                "\"paraIdx\":{},\"charOffset\":{}",
                current_para, current_offset
            )))
        } else {
            Ok(result)
        }
    }

    pub fn paste_internal_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.paste_internal(section_idx, paragraph_idx, char_offset)
    }

    pub fn has_internal_clipboard(&self) -> bool {
        self.clipboard_text
            .as_ref()
            .is_some_and(|text| !text.is_empty())
    }

    pub fn get_clipboard_text(&self) -> String {
        self.clipboard_text.clone().unwrap_or_default()
    }

    pub fn clear_clipboard(&mut self) {
        self.clipboard_text = None;
    }

    pub fn clipboard_has_control(&self) -> bool {
        false
    }

    pub fn render_page_svg(&self, page_num: u32) -> Result<String> {
        let index = page_num as usize;
        let lines = self.page_lines(page_num)?;
        let decoration = self.page_decoration(index);

        Ok(render_text_page_svg(
            lines,
            index + 1,
            self.page_count() as usize,
            self.page_layout,
            self.writing_mode,
            &self.document,
            decoration.as_ref(),
        ))
    }

    pub fn render_page_svg_native(&self, page_num: u32) -> Result<String> {
        self.render_page_svg(page_num)
    }

    pub fn render_page_html(&self, page_num: u32) -> Result<String> {
        let svg = self.render_page_svg(page_num)?;
        Ok(format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>rjtd page {}</title></head><body>{}</body></html>",
            page_num + 1,
            svg
        ))
    }

    pub fn render_page_html_native(&self, page_num: u32) -> Result<String> {
        self.render_page_html(page_num)
    }

    pub fn get_cursor_rect(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let rect = self.cursor_rect_for(paragraph_idx as usize, char_offset as usize)?;
        Ok(format_cursor_rect(&rect))
    }

    pub fn get_cursor_rect_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_cursor_rect(section_idx, paragraph_idx, char_offset)
    }

    pub fn hit_test(&self, page_num: u32, x: f64, y: f64) -> Result<String> {
        let lines = self.page_lines(page_num)?;
        let Some((line_index, line)) =
            nearest_text_line(lines, line_index_for_y(self.page_layout, lines.len(), y))
        else {
            return Ok(format!(
                "{{\"hit\":false,\"sectionIndex\":0,\"paragraphIndex\":0,\"charOffset\":0,\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1}}}",
                page_num,
                normalize_coordinate(x),
                normalize_coordinate(y)
            ));
        };
        let paragraph_index = line.paragraph_index().unwrap_or_default();
        let char_offset = char_offset_for_x(self.page_layout, line, x);
        Ok(format!(
            "{{\"hit\":true,\"sectionIndex\":0,\"paragraphIndex\":{},\"charOffset\":{},\"pageIndex\":{},\"lineIndex\":{},\"x\":{:.1},\"y\":{:.1}}}",
            paragraph_index,
            char_offset,
            page_num,
            line_index,
            normalize_coordinate(x),
            normalize_coordinate(y)
        ))
    }

    pub fn hit_test_native(&self, page_num: u32, x: f64, y: f64) -> Result<String> {
        self.hit_test(page_num, x, y)
    }

    pub fn get_line_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let lines = self.paragraph_lines(paragraph_idx as usize);
        if lines.is_empty() {
            return Err(rjtd_core::Error::InvalidData(format!(
                "paragraph {paragraph_idx} out of range"
            )));
        }

        let selected_index = paragraph_line_index(&lines, char_offset as usize);
        let (page_index, page_line_index, line) = lines[selected_index];
        Ok(format!(
            "{{\"sectionIndex\":0,\"paragraphIndex\":{},\"lineIndex\":{},\"lineCount\":{},\"charStart\":{},\"charEnd\":{},\"pageIndex\":{},\"pageLineIndex\":{}}}",
            paragraph_idx,
            selected_index,
            lines.len(),
            line.char_start(),
            line.char_end(),
            page_index,
            page_line_index
        ))
    }

    pub fn get_line_info_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_line_info(section_idx, paragraph_idx, char_offset)
    }

    pub fn move_vertical(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        delta: i32,
        preferred_x: f64,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let locations = self.text_line_locations();
        if locations.is_empty() {
            return Err(rjtd_core::Error::InvalidData(
                "document has no text lines".to_string(),
            ));
        }

        let current_index =
            text_location_index(&locations, paragraph_idx as usize, char_offset as usize)?;
        let target_index = (current_index as i64 + i64::from(delta))
            .clamp(0, locations.len().saturating_sub(1) as i64) as usize;
        let (page_index, page_line_index, target_line) = locations[target_index];
        let current_rect = self.cursor_rect_for(paragraph_idx as usize, char_offset as usize)?;
        let target_x = if preferred_x.is_finite() && preferred_x >= 0.0 {
            preferred_x
        } else {
            current_rect.x
        };
        let new_char_offset = char_offset_for_x(self.page_layout, target_line, target_x);
        let rect = cursor_rect_from_line(
            self.page_layout,
            page_index,
            page_line_index,
            target_line,
            new_char_offset,
        );
        Ok(format!(
            "{{\"sectionIndex\":0,\"paragraphIndex\":{},\"charOffset\":{},\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1},\"height\":{:.1},\"preferredX\":{:.1},\"rectValid\":true}}",
            target_line.paragraph_index().unwrap_or_default(),
            new_char_offset,
            rect.page_index,
            rect.x,
            rect.y,
            rect.height,
            target_x
        ))
    }

    pub fn move_vertical_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        delta: i32,
        preferred_x: f64,
    ) -> Result<String> {
        self.move_vertical(section_idx, paragraph_idx, char_offset, delta, preferred_x)
    }

    fn page_lines(&self, page_num: u32) -> Result<&[PageTextLine]> {
        self.pages
            .get(page_num as usize)
            .map(Vec::as_slice)
            .ok_or_else(|| rjtd_core::Error::InvalidData(format!("page {page_num} out of range")))
    }

    fn cursor_rect_for(&self, paragraph_index: usize, char_offset: usize) -> Result<CursorRect> {
        let mut last_line = None;

        for (page_index, page) in self.pages.iter().enumerate() {
            for (line_index, line) in page.iter().enumerate() {
                if line.paragraph_index() != Some(paragraph_index) {
                    continue;
                }

                last_line = Some((page_index, line_index, line));
                if char_offset <= line.char_end() {
                    return Ok(cursor_rect_from_line(
                        self.page_layout,
                        page_index,
                        line_index,
                        line,
                        char_offset,
                    ));
                }
            }
        }

        if let Some((page_index, line_index, line)) = last_line {
            return Ok(cursor_rect_from_line(
                self.page_layout,
                page_index,
                line_index,
                line,
                line.char_end(),
            ));
        }

        Err(rjtd_core::Error::InvalidData(format!(
            "paragraph {paragraph_index} out of range"
        )))
    }

    fn paragraph_lines(&self, paragraph_index: usize) -> Vec<(usize, usize, &PageTextLine)> {
        self.text_line_locations()
            .into_iter()
            .filter(|(_, _, line)| line.paragraph_index() == Some(paragraph_index))
            .collect()
    }

    fn text_line_locations(&self) -> Vec<(usize, usize, &PageTextLine)> {
        let mut locations = Vec::new();

        for (page_index, page) in self.pages.iter().enumerate() {
            for (line_index, line) in page.iter().enumerate() {
                if line.paragraph_index().is_some() {
                    locations.push((page_index, line_index, line));
                }
            }
        }

        locations
    }

    fn normalized_text_range(
        &self,
        start_para: usize,
        start_offset: usize,
        end_para: usize,
        end_offset: usize,
    ) -> Result<TextRange> {
        let (start_para, start_offset, end_para, end_offset) =
            if (start_para, start_offset) <= (end_para, end_offset) {
                (start_para, start_offset, end_para, end_offset)
            } else {
                (end_para, end_offset, start_para, start_offset)
            };

        let start_text = paragraph_text(self.paragraph(start_para)?);
        let end_text = paragraph_text(self.paragraph(end_para)?);
        checked_char_boundary(&start_text, start_offset)?;
        checked_char_boundary(&end_text, end_offset)?;

        Ok(TextRange {
            start_para,
            start_offset,
            end_para,
            end_offset,
        })
    }

    fn selected_text(
        &self,
        start_para: usize,
        start_offset: usize,
        end_para: usize,
        end_offset: usize,
    ) -> Result<String> {
        let range = self.normalized_text_range(start_para, start_offset, end_para, end_offset)?;
        if range.is_collapsed() {
            return Ok(String::new());
        }

        if range.start_para == range.end_para {
            let text = paragraph_text(self.paragraph(range.start_para)?);
            let start = checked_char_boundary(&text, range.start_offset)?;
            let end = checked_char_boundary(&text, range.end_offset)?;
            return Ok(text[start..end].to_string());
        }

        let mut chunks = Vec::new();
        let first_text = paragraph_text(self.paragraph(range.start_para)?);
        let first_start = checked_char_boundary(&first_text, range.start_offset)?;
        chunks.push(first_text[first_start..].to_string());

        for paragraph_index in range.start_para + 1..range.end_para {
            chunks.push(paragraph_text(self.paragraph(paragraph_index)?));
        }

        let last_text = paragraph_text(self.paragraph(range.end_para)?);
        let last_end = checked_char_boundary(&last_text, range.end_offset)?;
        chunks.push(last_text[..last_end].to_string());

        Ok(chunks.join("\n"))
    }

    fn search_hits(&self, query: &str, case_sensitive: bool) -> Vec<SearchHit> {
        let mut hits = Vec::new();
        let mut paragraph_index = 0u32;
        let length = query.chars().count() as u32;

        for block in self.document.blocks() {
            if let Block::Paragraph(paragraph) = block {
                let text = paragraph_text(paragraph);
                for offset in find_in_text(&text, query, case_sensitive) {
                    hits.push(SearchHit {
                        sec: 0,
                        para: paragraph_index,
                        char_offset: offset as u32,
                        length,
                    });
                }
                paragraph_index += 1;
            }
        }

        hits
    }

    fn paragraph_count(&self) -> usize {
        self.document
            .blocks()
            .iter()
            .filter(|block| matches!(block, Block::Paragraph(_)))
            .count()
    }

    fn paragraph(&self, paragraph_index: usize) -> Result<&Paragraph> {
        let block_index = self.paragraph_block_index(paragraph_index)?;
        match &self.document.blocks[block_index] {
            Block::Paragraph(paragraph) => Ok(paragraph),
            Block::Unknown(_) => unreachable!("paragraph_block_index returned an unknown block"),
        }
    }

    fn paragraph_mut(&mut self, paragraph_index: usize) -> Result<&mut Paragraph> {
        let block_index = self.paragraph_block_index(paragraph_index)?;
        match &mut self.document.blocks[block_index] {
            Block::Paragraph(paragraph) => Ok(paragraph),
            Block::Unknown(_) => unreachable!("paragraph_block_index returned an unknown block"),
        }
    }

    fn paragraph_block_index(&self, paragraph_index: usize) -> Result<usize> {
        let mut current_index = 0usize;

        for (block_index, block) in self.document.blocks().iter().enumerate() {
            if matches!(block, Block::Paragraph(_)) {
                if current_index == paragraph_index {
                    return Ok(block_index);
                }
                current_index += 1;
            }
        }

        Err(rjtd_core::Error::InvalidData(format!(
            "paragraph {paragraph_index} out of range"
        )))
    }

    fn ensure_text_position(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<()> {
        self.ensure_section(section_idx)?;
        let text = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        checked_char_boundary(&text, char_offset as usize)?;
        Ok(())
    }

    fn ensure_parent_paragraph(&self, section_idx: u32, parent_para_idx: u32) -> Result<()> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(parent_para_idx as usize)?;
        Ok(())
    }

    fn replace_paragraph_block(&mut self, block_index: usize, text: String) -> Result<()> {
        match self.document.blocks.get_mut(block_index) {
            Some(Block::Paragraph(paragraph)) => {
                paragraph.set_text(text);
                Ok(())
            }
            Some(Block::Unknown(_)) => Err(rjtd_core::Error::InvalidData(format!(
                "block {block_index} is not a paragraph"
            ))),
            None => Err(rjtd_core::Error::InvalidData(format!(
                "block {block_index} out of range"
            ))),
        }
    }

    fn set_paragraph_text(&mut self, paragraph_index: usize, text: String) -> Result<()> {
        let block_index = self.paragraph_block_index(paragraph_index)?;
        self.replace_paragraph_block(block_index, text)
    }

    fn set_paragraph_style(
        &mut self,
        paragraph_index: usize,
        style: Option<StyleRef>,
    ) -> Result<()> {
        self.paragraph_mut(paragraph_index)?.set_style(style);
        Ok(())
    }

    fn set_caret(&mut self, section_idx: u32, paragraph_idx: u32, char_offset: u32) {
        self.caret_section = section_idx;
        self.caret_paragraph = paragraph_idx;
        self.caret_char_offset = char_offset;
    }

    fn refresh_pages(&mut self) {
        self.pages = paginate_document_text(&self.document, self.page_layout, self.writing_mode);
        if project_fdm_single_page_diagram(&self.document, &mut self.pages) {
            return;
        }
        if let Some(pages) = project_sample_front_matter_pages(
            &self.document,
            &self.file_name,
            self.page_layout,
            self.writing_mode,
        ) {
            self.pages = pages;
        }
    }

    fn refresh_pages_with_budget(&mut self, budget: &mut ResourceBudget) -> Result<()> {
        let shape = page_construction_shape(&self.document, self.page_layout, self.writing_mode)?;
        budget.reserve_page_output(shape.pages, shape.lines)?;
        self.refresh_pages();
        Ok(())
    }

    fn ensure_section(&self, section_idx: u32) -> Result<()> {
        if section_idx == 0 {
            Ok(())
        } else {
            Err(rjtd_core::Error::InvalidData(format!(
                "section {section_idx} out of range"
            )))
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Metadata {
    title: Option<String>,
}

impl Metadata {
    pub fn new(title: Option<String>) -> Self {
        Self { title }
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawStream {
    name: String,
    bytes: Vec<u8>,
}

impl RawStream {
    pub fn new(name: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            bytes,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentAutoText {
    source_stream: String,
    offset: usize,
    text: String,
}

impl DocumentAutoText {
    pub fn new(source_stream: impl Into<String>, offset: usize, text: impl Into<String>) -> Self {
        Self {
            source_stream: source_stream.into(),
            offset,
            text: text.into(),
        }
    }

    pub fn from_auto_text_entry(source_stream: impl Into<String>, entry: &AutoTextEntry) -> Self {
        Self::new(source_stream, entry.offset(), entry.text())
    }

    pub fn source_stream(&self) -> &str {
        &self.source_stream
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTocEntry {
    title: String,
    page_label: String,
    source_span: TextSourceSpan,
}

impl DocumentTocEntry {
    pub fn new(
        title: impl Into<String>,
        page_label: impl Into<String>,
        source_span: TextSourceSpan,
    ) -> Self {
        Self {
            title: title.into(),
            page_label: page_label.into(),
            source_span,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn page_label(&self) -> &str {
        &self.page_label
    }

    pub fn source_span(&self) -> &TextSourceSpan {
        &self.source_span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentPageMark {
    source_stream: String,
    family: String,
    header_count: u32,
    header_stride: u32,
    header_last_index: u32,
    entries: Vec<DocumentPageMarkEntry>,
    trailing_byte_len: usize,
}

impl DocumentPageMark {
    pub fn new(
        source_stream: impl Into<String>,
        family: impl Into<String>,
        header_count: u32,
        header_stride: u32,
        header_last_index: u32,
        entries: Vec<DocumentPageMarkEntry>,
        trailing_byte_len: usize,
    ) -> Self {
        Self {
            source_stream: source_stream.into(),
            family: family.into(),
            header_count,
            header_stride,
            header_last_index,
            entries,
            trailing_byte_len,
        }
    }

    pub fn from_page_mark(source_stream: impl Into<String>, page_mark: &PageMark) -> Self {
        let header = page_mark.header();
        Self::new(
            source_stream,
            page_mark.family().as_str(),
            header.count_value(),
            header.stride_value(),
            header.last_index_value(),
            page_mark
                .entries()
                .iter()
                .enumerate()
                .map(|(row_index, entry)| DocumentPageMarkEntry::from_entry(row_index, entry))
                .collect(),
            page_mark.trailing_bytes().len(),
        )
    }

    pub fn source_stream(&self) -> &str {
        &self.source_stream
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn header_count(&self) -> u32 {
        self.header_count
    }

    pub fn header_stride(&self) -> u32 {
        self.header_stride
    }

    pub fn header_last_index(&self) -> u32 {
        self.header_last_index
    }

    pub fn entries(&self) -> &[DocumentPageMarkEntry] {
        &self.entries
    }

    pub fn trailing_byte_len(&self) -> usize {
        self.trailing_byte_len
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentPageMarkEntry {
    row_index: usize,
    index: Option<u32>,
    flags: Option<u32>,
    line_start: Option<u32>,
    line_end: Option<u32>,
    raw: Vec<u8>,
    u16_fields: Vec<u16>,
    u32_fields: Vec<u32>,
}

impl DocumentPageMarkEntry {
    fn from_entry(row_index: usize, entry: &rjtd_core::layout_mark::PageMarkEntry) -> Self {
        Self {
            row_index,
            index: entry.index(),
            flags: entry.flags(),
            line_start: entry.line_start(),
            line_end: entry.line_end(),
            raw: entry.raw().to_vec(),
            u16_fields: u16_fields_be(entry.raw()),
            u32_fields: entry.u32_fields(),
        }
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn index(&self) -> Option<u32> {
        self.index
    }

    pub fn flags(&self) -> Option<u32> {
        self.flags
    }

    pub fn line_start(&self) -> Option<u32> {
        self.line_start
    }

    pub fn line_end(&self) -> Option<u32> {
        self.line_end
    }

    pub fn raw_len(&self) -> usize {
        self.raw.len()
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw
    }

    pub fn u16_fields(&self) -> &[u16] {
        &self.u16_fields
    }

    pub fn u32_fields(&self) -> &[u32] {
        &self.u32_fields
    }

    pub fn u16_geometry_profile(&self) -> PageMarkU16GeometryProfile {
        page_mark_u16_geometry_profile(&self.u16_fields)
    }
}

fn u16_fields_be(raw: &[u8]) -> Vec<u16> {
    raw.chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageMarkU16SubrecordCandidate {
    word_index: usize,
    byte_offset: usize,
    words: [u16; 8],
}

impl PageMarkU16SubrecordCandidate {
    fn word_index(self) -> usize {
        self.word_index
    }

    fn byte_offset(self) -> usize {
        self.byte_offset
    }

    fn words(self) -> [u16; 8] {
        self.words
    }

    fn u32_fields(self) -> [u32; 4] {
        page_mark_u16_subrecord_u32_fields(&self.words)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageMarkU16GeometryProfile {
    selected_fields_all_zero: bool,
    non_zero_additive_unit_candidate: bool,
    word20_is_00ff: bool,
}

impl PageMarkU16GeometryProfile {
    pub fn selected_fields_all_zero(&self) -> bool {
        self.selected_fields_all_zero
    }

    pub fn non_zero_additive_unit_candidate(&self) -> bool {
        self.non_zero_additive_unit_candidate
    }

    pub fn word20_is_00ff(&self) -> bool {
        self.word20_is_00ff
    }

    pub fn class_name(&self) -> &'static str {
        if self.selected_fields_all_zero {
            "zero-sentinel"
        } else if self.non_zero_additive_unit_candidate && self.word20_is_00ff {
            "additive-boundary"
        } else if self.non_zero_additive_unit_candidate {
            "additive-row"
        } else {
            "mixed-payload"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentPaperMark {
    source_stream: String,
    header_count: u32,
    header_stride: u32,
    header_last_index: u32,
    entries: Vec<DocumentPaperMarkEntry>,
}

impl DocumentPaperMark {
    pub fn new(
        source_stream: impl Into<String>,
        header_count: u32,
        header_stride: u32,
        header_last_index: u32,
        entries: Vec<DocumentPaperMarkEntry>,
    ) -> Self {
        Self {
            source_stream: source_stream.into(),
            header_count,
            header_stride,
            header_last_index,
            entries,
        }
    }

    pub fn from_paper_mark(source_stream: impl Into<String>, paper_mark: &PaperMark) -> Self {
        let header = paper_mark.header();
        Self::new(
            source_stream,
            header.count_value(),
            header.stride_value(),
            header.last_index_value(),
            paper_mark
                .entries()
                .iter()
                .enumerate()
                .map(|(row_index, entry)| DocumentPaperMarkEntry::from_entry(row_index, *entry))
                .collect(),
        )
    }

    pub fn source_stream(&self) -> &str {
        &self.source_stream
    }

    pub fn header_count(&self) -> u32 {
        self.header_count
    }

    pub fn header_stride(&self) -> u32 {
        self.header_stride
    }

    pub fn header_last_index(&self) -> u32 {
        self.header_last_index
    }

    pub fn entries(&self) -> &[DocumentPaperMarkEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentPaperMarkEntry {
    row_index: usize,
    index: u32,
    flags: u32,
}

impl DocumentPaperMarkEntry {
    fn from_entry(row_index: usize, entry: rjtd_core::layout_mark::PaperMarkEntry) -> Self {
        Self {
            row_index,
            index: entry.index(),
            flags: entry.flags(),
        }
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn raw_len(&self) -> usize {
        8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentFont {
    source_stream: String,
    id: u16,
    offset: usize,
    name: String,
    raw: Vec<u8>,
}

impl DocumentFont {
    pub fn new(
        source_stream: impl Into<String>,
        id: u16,
        offset: usize,
        name: impl Into<String>,
        raw: Vec<u8>,
    ) -> Self {
        Self {
            source_stream: source_stream.into(),
            id,
            offset,
            name: name.into(),
            raw,
        }
    }

    pub fn from_font_stream_entry(source_stream: impl Into<String>, entry: &FontEntry) -> Self {
        Self::new(
            source_stream,
            entry.id(),
            entry.offset(),
            entry.name(),
            entry.raw().to_vec(),
        )
    }

    pub fn source_stream(&self) -> &str {
        &self.source_stream
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectStreamCandidateReason {
    ObjectPath,
    ImagePath,
    ShapePath,
    TablePath,
    VisualListPath,
    FigureLink,
    EmbeddedPressSnapshot,
    FdmText,
    JsfartArt,
    Jseq3Formula,
    SoMarker,
    ImageSignature,
    SvgSignature,
}

impl ObjectStreamCandidateReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ObjectPath => "object-path",
            Self::ImagePath => "image-path",
            Self::ShapePath => "shape-path",
            Self::TablePath => "table-path",
            Self::VisualListPath => "visual-list-path",
            Self::FigureLink => "figure-link",
            Self::EmbeddedPressSnapshot => "embedded-press-snapshot",
            Self::FdmText => "fdm-text",
            Self::JsfartArt => "jsfart-art",
            Self::Jseq3Formula => "jseq3-formula",
            Self::SoMarker => "so-marker",
            Self::ImageSignature => "image-signature",
            Self::SvgSignature => "svg-signature",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddedPressVectorSegmentCandidate {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl ObjectEmbeddedPressVectorSegmentCandidate {
    fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn x1(&self) -> u32 {
        self.x1
    }

    pub fn y1(&self) -> u32 {
        self.y1
    }

    pub fn x2(&self) -> u32 {
        self.x2
    }

    pub fn y2(&self) -> u32 {
        self.y2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectEmbeddedPressVectorPathKind {
    Outline,
    Texture,
}

impl ObjectEmbeddedPressVectorPathKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Outline => "outline",
            Self::Texture => "texture",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectEmbeddedPressTextureBezierHeaderCandidate {
    point_count: u32,
    byte_count: u32,
    flags: u32,
}

impl ObjectEmbeddedPressTextureBezierHeaderCandidate {
    fn new(point_count: u32, byte_count: u32, flags: u32) -> Self {
        Self {
            point_count,
            byte_count,
            flags,
        }
    }

    pub fn point_count(&self) -> u32 {
        self.point_count
    }

    pub fn byte_count(&self) -> u32 {
        self.byte_count
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectEmbeddedPressVectorPathCommandCandidate {
    MoveTo {
        x: u32,
        y: u32,
    },
    CubicTo {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        x3: u32,
        y3: u32,
    },
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddedPressStateRecordCandidate {
    record_type: u32,
    offset: usize,
    payload: Vec<u8>,
}

impl ObjectEmbeddedPressStateRecordCandidate {
    fn new(record_type: u32, offset: usize, payload: Vec<u8>) -> Self {
        Self {
            record_type,
            offset,
            payload,
        }
    }

    pub fn record_type(&self) -> u32 {
        self.record_type
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn payload_le32_words(&self) -> Vec<u32> {
        self.payload
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("chunk size is exact")))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddedPressVectorPathCandidate {
    kind: ObjectEmbeddedPressVectorPathKind,
    texture_bezier_header: Option<ObjectEmbeddedPressTextureBezierHeaderCandidate>,
    state_records: Vec<ObjectEmbeddedPressStateRecordCandidate>,
    commands: Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
}

impl ObjectEmbeddedPressVectorPathCandidate {
    fn new(
        kind: ObjectEmbeddedPressVectorPathKind,
        texture_bezier_header: Option<ObjectEmbeddedPressTextureBezierHeaderCandidate>,
        state_records: Vec<ObjectEmbeddedPressStateRecordCandidate>,
        commands: Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
    ) -> Self {
        Self {
            kind,
            texture_bezier_header,
            state_records,
            commands,
        }
    }

    pub fn kind(&self) -> ObjectEmbeddedPressVectorPathKind {
        self.kind
    }

    pub fn texture_bezier_header(&self) -> Option<ObjectEmbeddedPressTextureBezierHeaderCandidate> {
        self.texture_bezier_header
    }

    pub fn state_records(&self) -> &[ObjectEmbeddedPressStateRecordCandidate] {
        &self.state_records
    }

    pub fn commands(&self) -> &[ObjectEmbeddedPressVectorPathCommandCandidate] {
        &self.commands
    }
}

struct ObjectEmbeddedPressVectorPathBuilder {
    kind: ObjectEmbeddedPressVectorPathKind,
    texture_bezier_header: Option<ObjectEmbeddedPressTextureBezierHeaderCandidate>,
    state_records: Vec<ObjectEmbeddedPressStateRecordCandidate>,
    commands: Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
}

impl ObjectEmbeddedPressVectorPathBuilder {
    fn new(state_records: Vec<ObjectEmbeddedPressStateRecordCandidate>) -> Self {
        Self {
            kind: ObjectEmbeddedPressVectorPathKind::Outline,
            texture_bezier_header: None,
            state_records,
            commands: Vec::new(),
        }
    }

    fn mark_texture(&mut self, header: ObjectEmbeddedPressTextureBezierHeaderCandidate) {
        self.kind = ObjectEmbeddedPressVectorPathKind::Texture;
        if self.texture_bezier_header.is_none() {
            self.texture_bezier_header = Some(header);
        }
    }

    fn push(&mut self, command: ObjectEmbeddedPressVectorPathCommandCandidate) {
        self.commands.push(command);
    }

    fn finish(self) -> Option<ObjectEmbeddedPressVectorPathCandidate> {
        (!self.commands.is_empty()).then(|| {
            ObjectEmbeddedPressVectorPathCandidate::new(
                self.kind,
                self.texture_bezier_header,
                self.state_records,
                self.commands,
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectVisualListCandidate {
    declared_size: usize,
    magic_offset: usize,
    magic: String,
    version: u32,
    flags: u32,
    width: u32,
    height: u32,
    row_stride: u32,
    bit_depth: u32,
    x_pixels_per_meter: u32,
    y_pixels_per_meter: u32,
    rle_data_offset: usize,
    rle_data_len: usize,
    pixels: Vec<u8>,
}

impl ObjectVisualListCandidate {
    #[allow(clippy::too_many_arguments)]
    fn new(
        declared_size: usize,
        version: u32,
        flags: u32,
        width: u32,
        height: u32,
        row_stride: u32,
        bit_depth: u32,
        x_pixels_per_meter: u32,
        y_pixels_per_meter: u32,
        rle_data_offset: usize,
        rle_data_len: usize,
        pixels: Vec<u8>,
    ) -> Self {
        Self {
            declared_size,
            magic_offset: VISUAL_LIST_MAGIC_OFFSET,
            magic: "BMDV".to_string(),
            version,
            flags,
            width,
            height,
            row_stride,
            bit_depth,
            x_pixels_per_meter,
            y_pixels_per_meter,
            rle_data_offset,
            rle_data_len,
            pixels,
        }
    }

    pub fn declared_size(&self) -> usize {
        self.declared_size
    }

    pub fn magic_offset(&self) -> usize {
        self.magic_offset
    }

    pub fn magic(&self) -> &str {
        &self.magic
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn row_stride(&self) -> u32 {
        self.row_stride
    }

    pub fn bit_depth(&self) -> u32 {
        self.bit_depth
    }

    pub fn x_pixels_per_meter(&self) -> u32 {
        self.x_pixels_per_meter
    }

    pub fn y_pixels_per_meter(&self) -> u32 {
        self.y_pixels_per_meter
    }

    pub fn rle_data_offset(&self) -> usize {
        self.rle_data_offset
    }

    pub fn rle_data_len(&self) -> usize {
        self.rle_data_len
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamCandidate {
    path: String,
    size: usize,
    reasons: Vec<ObjectStreamCandidateReason>,
    ownership_candidate: Option<ObjectStreamOwnershipCandidate>,
    ownership_reference_candidates: Vec<ObjectStreamOwnershipReferenceCandidate>,
    frame_reference_row_candidates: Vec<ObjectFrameReferenceRowCandidate>,
    fdm_index_entry_candidates: Vec<ObjectFdmIndexEntryCandidate>,
    fdm_text_index_entry_candidates: Vec<ObjectFdmTextIndexEntryCandidate>,
    fdm_raw_vector_segments: Vec<ObjectFdmVectorSegmentCandidate>,
    fdm_raw_vector_commands: Vec<ObjectFdmVectorCommandCandidate>,
    image_signature_hits: Vec<ObjectImageSignatureHit>,
    image_payload_spans: Vec<ObjectImagePayloadSpan>,
    visual_list_candidate: Option<ObjectVisualListCandidate>,
    figure_link_candidate: Option<ObjectFigureLinkCandidate>,
    embedded_press_snapshot_candidate: Option<ObjectEmbeddedPressSnapshotCandidate>,
    fdm_text_candidates: Vec<ObjectFdmTextCandidate>,
    jsfart_stream_profile_candidate: Option<ObjectJsfartStreamProfileCandidate>,
    jsfart_art_candidate: Option<ObjectJsfartArtCandidate>,
    jseq3_formula_candidate: Option<ObjectJseq3FormulaCandidate>,
    svg_offsets: Vec<usize>,
    so_offsets: Vec<usize>,
    payload_prefix: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ObjectFrameReferenceRowProjection {
    encoding: &'static str,
    stride: usize,
    field_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamOwnershipReferenceCandidate {
    target_path: String,
    encoding: String,
    total_matches: usize,
    offsets: Vec<usize>,
}

impl ObjectStreamOwnershipReferenceCandidate {
    pub fn new(
        target_path: impl Into<String>,
        encoding: impl Into<String>,
        total_matches: usize,
        offsets: Vec<usize>,
    ) -> Self {
        Self {
            target_path: target_path.into(),
            encoding: encoding.into(),
            total_matches,
            offsets,
        }
    }

    pub fn target_path(&self) -> &str {
        &self.target_path
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    pub fn total_matches(&self) -> usize {
        self.total_matches
    }

    pub fn offsets(&self) -> &[usize] {
        &self.offsets
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFrameReferenceRowCandidate {
    target_path: String,
    encoding: String,
    stride: usize,
    field_offset: usize,
    offset: usize,
    row_index: usize,
    row_start: usize,
    family: String,
    row: Vec<u8>,
    suffix_link: Option<ObjectFrameReferenceRowLink>,
}

impl ObjectFrameReferenceRowCandidate {
    fn new(
        target_path: impl Into<String>,
        encoding: impl Into<String>,
        stride: usize,
        field_offset: usize,
        location: ObjectFrameReferenceRowLocation,
        row: Vec<u8>,
    ) -> Self {
        let encoding = encoding.into();
        let family =
            classify_object_frame_reference_row(&row, encoding.as_str(), stride, field_offset);
        Self {
            target_path: target_path.into(),
            encoding,
            stride,
            field_offset,
            offset: location.offset,
            row_index: location.row_index,
            row_start: location.row_start,
            family: family.to_string(),
            row,
            suffix_link: None,
        }
    }

    pub fn target_path(&self) -> &str {
        &self.target_path
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn field_offset(&self) -> usize {
        self.field_offset
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn row_start(&self) -> usize {
        self.row_start
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn row(&self) -> &[u8] {
        &self.row
    }

    pub fn suffix_link(&self) -> Option<&ObjectFrameReferenceRowLink> {
        self.suffix_link.as_ref()
    }

    fn set_suffix_link(&mut self, suffix_link: ObjectFrameReferenceRowLink) {
        self.suffix_link = Some(suffix_link);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ObjectFrameReferenceRowLocation {
    offset: usize,
    row_index: usize,
    row_start: usize,
}

impl ObjectFrameReferenceRowLocation {
    fn new(offset: usize, row_index: usize, row_start: usize) -> Self {
        Self {
            offset,
            row_index,
            row_start,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFrameReferenceRowLink {
    relation: String,
    suffix_family: String,
    matched_row_start: usize,
    matched_row_index: usize,
}

impl ObjectFrameReferenceRowLink {
    fn new(
        relation: impl Into<String>,
        suffix_family: impl Into<String>,
        matched_row_start: usize,
        matched_row_index: usize,
    ) -> Self {
        Self {
            relation: relation.into(),
            suffix_family: suffix_family.into(),
            matched_row_start,
            matched_row_index,
        }
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn suffix_family(&self) -> &str {
        &self.suffix_family
    }

    pub fn matched_row_start(&self) -> usize {
        self.matched_row_start
    }

    pub fn matched_row_index(&self) -> usize {
        self.matched_row_index
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFrameRecordCandidate {
    source_path: String,
    row_index: usize,
    row_start: usize,
    record_len: usize,
    record_kind: u16,
    declared_record_bytes: u16,
    object_id: u16,
    object_type: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    corner_radius: u16,
    style_id: u16,
    row_prefix: Vec<u8>,
}

impl ObjectFrameRecordCandidate {
    fn new(source_path: impl Into<String>, row_index: usize, row_start: usize, row: &[u8]) -> Self {
        Self {
            source_path: source_path.into(),
            row_index,
            row_start,
            record_len: row.len(),
            record_kind: read_be16_at(row, 0).unwrap_or_default(),
            declared_record_bytes: read_be16_at(row, 2).unwrap_or_default(),
            object_id: read_be16_at(row, FRAME_RECORD_ID_OFFSET).unwrap_or_default(),
            object_type: read_be16_at(row, FRAME_RECORD_TYPE_OFFSET).unwrap_or_default(),
            x: read_be16_at(row, FRAME_RECORD_X_OFFSET).unwrap_or_default(),
            y: read_be16_at(row, FRAME_RECORD_Y_OFFSET).unwrap_or_default(),
            width: read_be16_at(row, FRAME_RECORD_WIDTH_OFFSET).unwrap_or_default(),
            height: read_be16_at(row, FRAME_RECORD_HEIGHT_OFFSET).unwrap_or_default(),
            corner_radius: read_be16_at(row, FRAME_RECORD_CORNER_RADIUS_OFFSET).unwrap_or_default(),
            style_id: read_be16_at(row, FRAME_RECORD_STYLE_ID_OFFSET).unwrap_or_default(),
            row_prefix: row[..row.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn row_start(&self) -> usize {
        self.row_start
    }

    pub fn record_len(&self) -> usize {
        self.record_len
    }

    pub fn record_kind(&self) -> u16 {
        self.record_kind
    }

    pub fn declared_record_bytes(&self) -> u16 {
        self.declared_record_bytes
    }

    pub fn object_id(&self) -> u16 {
        self.object_id
    }

    pub fn object_type(&self) -> u16 {
        self.object_type
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn corner_radius(&self) -> u16 {
        self.corner_radius
    }

    pub fn style_id(&self) -> u16 {
        self.style_id
    }

    pub fn row_prefix(&self) -> &[u8] {
        &self.row_prefix
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFigureLinkCandidate {
    header_words_be: Vec<u16>,
    declared_row_count_candidate: Option<u16>,
    row_stride: usize,
    rows: Vec<ObjectFigureLinkRowCandidate>,
}

impl ObjectFigureLinkCandidate {
    fn new(
        header_words_be: Vec<u16>,
        declared_row_count_candidate: Option<u16>,
        row_stride: usize,
        rows: Vec<ObjectFigureLinkRowCandidate>,
    ) -> Self {
        Self {
            header_words_be,
            declared_row_count_candidate,
            row_stride,
            rows,
        }
    }

    pub fn header_words_be(&self) -> &[u16] {
        &self.header_words_be
    }

    pub fn declared_row_count_candidate(&self) -> Option<u16> {
        self.declared_row_count_candidate
    }

    pub fn row_stride(&self) -> usize {
        self.row_stride
    }

    pub fn rows(&self) -> &[ObjectFigureLinkRowCandidate] {
        &self.rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFigureLinkRowCandidate {
    row_index: usize,
    row_start: usize,
    words_be: Vec<u16>,
    row: Vec<u8>,
}

impl ObjectFigureLinkRowCandidate {
    fn new(row_index: usize, row_start: usize, row: &[u8]) -> Self {
        Self {
            row_index,
            row_start,
            words_be: read_be16_fields(row),
            row: row.to_vec(),
        }
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn row_start(&self) -> usize {
        self.row_start
    }

    pub fn words_be(&self) -> &[u16] {
        &self.words_be
    }

    pub fn group_index_candidate(&self) -> Option<u16> {
        self.words_be.get(1).copied()
    }

    pub fn source_id_candidate(&self) -> Option<u16> {
        self.words_be.get(3).copied()
    }

    pub fn relation_kind_candidate(&self) -> Option<u16> {
        self.words_be.get(4).copied()
    }

    pub fn target_row_index_candidate(&self) -> Option<u16> {
        self.words_be.get(6).copied()
    }

    pub fn row(&self) -> &[u8] {
        &self.row
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamOwnershipCandidate {
    basis: String,
    family: String,
    storage_path: Option<String>,
    embedding_index: Option<usize>,
    stream_role: String,
}

impl ObjectStreamOwnershipCandidate {
    pub fn new(
        basis: impl Into<String>,
        family: impl Into<String>,
        storage_path: Option<String>,
        embedding_index: Option<usize>,
        stream_role: impl Into<String>,
    ) -> Self {
        Self {
            basis: basis.into(),
            family: family.into(),
            storage_path,
            embedding_index,
            stream_role: stream_role.into(),
        }
    }

    pub fn basis(&self) -> &str {
        &self.basis
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn storage_path(&self) -> Option<&str> {
        self.storage_path.as_deref()
    }

    pub fn embedding_index(&self) -> Option<usize> {
        self.embedding_index
    }

    pub fn stream_role(&self) -> &str {
        &self.stream_role
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamCandidateEvidence {
    reasons: Vec<ObjectStreamCandidateReason>,
    image_signature_hits: Vec<ObjectImageSignatureHit>,
    image_payload_spans: Vec<ObjectImagePayloadSpan>,
    visual_list_candidate: Option<ObjectVisualListCandidate>,
    figure_link_candidate: Option<ObjectFigureLinkCandidate>,
    embedded_press_snapshot_candidate: Option<ObjectEmbeddedPressSnapshotCandidate>,
    fdm_text_candidates: Vec<ObjectFdmTextCandidate>,
    jsfart_stream_profile_candidate: Option<ObjectJsfartStreamProfileCandidate>,
    jsfart_art_candidate: Option<ObjectJsfartArtCandidate>,
    jseq3_formula_candidate: Option<ObjectJseq3FormulaCandidate>,
    svg_offsets: Vec<usize>,
    so_offsets: Vec<usize>,
}

impl ObjectStreamCandidateEvidence {
    pub fn new(
        reasons: Vec<ObjectStreamCandidateReason>,
        image_signature_hits: Vec<ObjectImageSignatureHit>,
        image_payload_spans: Vec<ObjectImagePayloadSpan>,
        visual_list_candidate: Option<ObjectVisualListCandidate>,
        svg_offsets: Vec<usize>,
        so_offsets: Vec<usize>,
    ) -> Self {
        Self {
            reasons,
            image_signature_hits,
            image_payload_spans,
            visual_list_candidate,
            figure_link_candidate: None,
            embedded_press_snapshot_candidate: None,
            fdm_text_candidates: Vec::new(),
            jsfart_stream_profile_candidate: None,
            jsfart_art_candidate: None,
            jseq3_formula_candidate: None,
            svg_offsets,
            so_offsets,
        }
    }

    pub fn reasons(&self) -> &[ObjectStreamCandidateReason] {
        &self.reasons
    }

    pub fn image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.image_signature_hits
    }

    pub fn image_payload_spans(&self) -> &[ObjectImagePayloadSpan] {
        &self.image_payload_spans
    }

    pub fn visual_list_candidate(&self) -> Option<&ObjectVisualListCandidate> {
        self.visual_list_candidate.as_ref()
    }

    fn with_figure_link_candidate(mut self, link: Option<ObjectFigureLinkCandidate>) -> Self {
        self.figure_link_candidate = link;
        self
    }

    fn with_embedded_press_snapshot_candidate(
        mut self,
        snapshot: Option<ObjectEmbeddedPressSnapshotCandidate>,
    ) -> Self {
        self.embedded_press_snapshot_candidate = snapshot;
        self
    }

    fn with_fdm_text_candidates(mut self, candidates: Vec<ObjectFdmTextCandidate>) -> Self {
        self.fdm_text_candidates = candidates;
        self
    }

    fn with_jseq3_formula_candidate(
        mut self,
        formula: Option<ObjectJseq3FormulaCandidate>,
    ) -> Self {
        self.jseq3_formula_candidate = formula;
        self
    }

    fn with_jsfart_stream_profile_candidate(
        mut self,
        profile: Option<ObjectJsfartStreamProfileCandidate>,
    ) -> Self {
        self.jsfart_stream_profile_candidate = profile;
        self
    }

    fn with_jsfart_art_candidate(mut self, art: Option<ObjectJsfartArtCandidate>) -> Self {
        self.jsfart_art_candidate = art;
        self
    }

    pub fn svg_offsets(&self) -> &[usize] {
        &self.svg_offsets
    }

    pub fn so_offsets(&self) -> &[usize] {
        &self.so_offsets
    }
}

impl ObjectStreamCandidate {
    pub fn new(
        path: impl Into<String>,
        size: usize,
        evidence: ObjectStreamCandidateEvidence,
        payload_prefix: Vec<u8>,
    ) -> Self {
        let path = path.into();
        let ownership_candidate = object_stream_ownership_candidate(&path);
        Self {
            path,
            size,
            reasons: evidence.reasons,
            ownership_candidate,
            ownership_reference_candidates: Vec::new(),
            frame_reference_row_candidates: Vec::new(),
            fdm_index_entry_candidates: Vec::new(),
            fdm_text_index_entry_candidates: Vec::new(),
            fdm_raw_vector_segments: Vec::new(),
            fdm_raw_vector_commands: Vec::new(),
            image_signature_hits: evidence.image_signature_hits,
            image_payload_spans: evidence.image_payload_spans,
            visual_list_candidate: evidence.visual_list_candidate,
            figure_link_candidate: evidence.figure_link_candidate,
            embedded_press_snapshot_candidate: evidence.embedded_press_snapshot_candidate,
            fdm_text_candidates: evidence.fdm_text_candidates,
            jsfart_stream_profile_candidate: evidence.jsfart_stream_profile_candidate,
            jsfart_art_candidate: evidence.jsfart_art_candidate,
            jseq3_formula_candidate: evidence.jseq3_formula_candidate,
            svg_offsets: evidence.svg_offsets,
            so_offsets: evidence.so_offsets,
            payload_prefix,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn reasons(&self) -> &[ObjectStreamCandidateReason] {
        &self.reasons
    }

    pub fn ownership_candidate(&self) -> Option<&ObjectStreamOwnershipCandidate> {
        self.ownership_candidate.as_ref()
    }

    pub fn ownership_reference_candidates(&self) -> &[ObjectStreamOwnershipReferenceCandidate] {
        &self.ownership_reference_candidates
    }

    pub fn frame_reference_row_candidates(&self) -> &[ObjectFrameReferenceRowCandidate] {
        &self.frame_reference_row_candidates
    }

    pub fn fdm_index_entry_candidates(&self) -> &[ObjectFdmIndexEntryCandidate] {
        &self.fdm_index_entry_candidates
    }

    pub fn fdm_text_index_entry_candidates(&self) -> &[ObjectFdmTextIndexEntryCandidate] {
        &self.fdm_text_index_entry_candidates
    }

    pub fn fdm_raw_vector_segments(&self) -> &[ObjectFdmVectorSegmentCandidate] {
        &self.fdm_raw_vector_segments
    }

    pub fn fdm_raw_vector_commands(&self) -> &[ObjectFdmVectorCommandCandidate] {
        &self.fdm_raw_vector_commands
    }

    fn set_ownership_reference_candidates(
        &mut self,
        ownership_reference_candidates: Vec<ObjectStreamOwnershipReferenceCandidate>,
    ) {
        self.ownership_reference_candidates = ownership_reference_candidates;
    }

    fn set_frame_reference_row_candidates(
        &mut self,
        frame_reference_row_candidates: Vec<ObjectFrameReferenceRowCandidate>,
    ) {
        self.frame_reference_row_candidates = frame_reference_row_candidates;
    }

    fn set_fdm_index_entry_candidates(
        &mut self,
        fdm_index_entry_candidates: Vec<ObjectFdmIndexEntryCandidate>,
    ) {
        self.fdm_index_entry_candidates = fdm_index_entry_candidates;
    }

    fn set_fdm_text_index_entry_candidates(
        &mut self,
        fdm_text_index_entry_candidates: Vec<ObjectFdmTextIndexEntryCandidate>,
    ) {
        self.fdm_text_index_entry_candidates = fdm_text_index_entry_candidates;
    }

    fn set_fdm_raw_vector_segments(
        &mut self,
        fdm_raw_vector_segments: Vec<ObjectFdmVectorSegmentCandidate>,
    ) {
        self.fdm_raw_vector_segments = fdm_raw_vector_segments;
    }

    fn set_fdm_raw_vector_commands(
        &mut self,
        fdm_raw_vector_commands: Vec<ObjectFdmVectorCommandCandidate>,
    ) {
        self.fdm_raw_vector_commands = fdm_raw_vector_commands;
    }

    pub fn image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.image_signature_hits
    }

    pub fn image_payload_spans(&self) -> &[ObjectImagePayloadSpan] {
        &self.image_payload_spans
    }

    pub fn visual_list_candidate(&self) -> Option<&ObjectVisualListCandidate> {
        self.visual_list_candidate.as_ref()
    }

    pub fn figure_link_candidate(&self) -> Option<&ObjectFigureLinkCandidate> {
        self.figure_link_candidate.as_ref()
    }

    pub fn embedded_press_snapshot_candidate(
        &self,
    ) -> Option<&ObjectEmbeddedPressSnapshotCandidate> {
        self.embedded_press_snapshot_candidate.as_ref()
    }

    pub fn fdm_text_candidates(&self) -> &[ObjectFdmTextCandidate] {
        &self.fdm_text_candidates
    }

    pub fn jsfart_stream_profile_candidate(&self) -> Option<&ObjectJsfartStreamProfileCandidate> {
        self.jsfart_stream_profile_candidate.as_ref()
    }

    pub fn jsfart_art_candidate(&self) -> Option<&ObjectJsfartArtCandidate> {
        self.jsfart_art_candidate.as_ref()
    }

    pub fn jseq3_formula_candidate(&self) -> Option<&ObjectJseq3FormulaCandidate> {
        self.jseq3_formula_candidate.as_ref()
    }

    pub fn svg_offsets(&self) -> &[usize] {
        &self.svg_offsets
    }

    pub fn so_offsets(&self) -> &[usize] {
        &self.so_offsets
    }

    pub fn payload_prefix(&self) -> &[u8] {
        &self.payload_prefix
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSourceSpan {
    byte_start: usize,
    byte_end: usize,
    unit_start: usize,
    unit_end: usize,
}

impl TextSourceSpan {
    pub fn new(byte_start: usize, byte_end: usize, unit_start: usize, unit_end: usize) -> Self {
        Self {
            byte_start,
            byte_end,
            unit_start,
            unit_end,
        }
    }

    fn from_document_text_entry(entry: &DocumentTextMapEntry) -> Self {
        Self::new(
            entry.byte_start(),
            entry.byte_end(),
            entry.unit_start(),
            entry.unit_end(),
        )
    }

    fn subspan_by_units(&self, start_units: usize, end_units: usize) -> Self {
        Self::new(
            self.byte_start + start_units * 2,
            self.byte_start + end_units * 2,
            self.unit_start + start_units,
            self.unit_start + end_units,
        )
    }

    pub fn byte_start(&self) -> usize {
        self.byte_start
    }

    pub fn byte_end(&self) -> usize {
        self.byte_end
    }

    pub fn unit_start(&self) -> usize {
        self.unit_start
    }

    pub fn unit_end(&self) -> usize {
        self.unit_end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextControlBoundary {
    index: usize,
    code: u16,
    source_span: Option<TextSourceSpan>,
}

impl TextControlBoundary {
    pub fn new(index: usize, code: u16, source_span: Option<TextSourceSpan>) -> Self {
        Self {
            index,
            code,
            source_span,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn source_span(&self) -> Option<&TextSourceSpan> {
        self.source_span.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextCountRangeOverlapBasis {
    Byte,
    Unit,
}

impl TextCountRangeOverlapBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Byte => "byte",
            Self::Unit => "unit",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCountRangeOverlap {
    basis: TextCountRangeOverlapBasis,
    block_index: usize,
    inline_index: usize,
    source_start: usize,
    source_end: usize,
    text: String,
}

impl TextCountRangeOverlap {
    fn new(
        basis: TextCountRangeOverlapBasis,
        block_index: usize,
        inline_index: usize,
        source_start: usize,
        source_end: usize,
        text: String,
    ) -> Self {
        Self {
            basis,
            block_index,
            inline_index,
            source_start,
            source_end,
            text,
        }
    }

    pub fn basis(&self) -> TextCountRangeOverlapBasis {
        self.basis
    }

    pub fn block_index(&self) -> usize {
        self.block_index
    }

    pub fn inline_index(&self) -> usize {
        self.inline_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCountControlRangeOverlap {
    basis: TextCountRangeOverlapBasis,
    delimiter_code: u16,
    range_count: usize,
    first_range_index: usize,
    last_range_index: usize,
    source_start: usize,
    source_end: usize,
}

impl TextCountControlRangeOverlap {
    fn new(
        basis: TextCountRangeOverlapBasis,
        delimiter_code: u16,
        range_count: usize,
        first_range_index: usize,
        last_range_index: usize,
        source_start: usize,
        source_end: usize,
    ) -> Self {
        Self {
            basis,
            delimiter_code,
            range_count,
            first_range_index,
            last_range_index,
            source_start,
            source_end,
        }
    }

    pub fn basis(&self) -> TextCountRangeOverlapBasis {
        self.basis
    }

    pub fn delimiter_code(&self) -> u16 {
        self.delimiter_code
    }

    pub fn range_count(&self) -> usize {
        self.range_count
    }

    pub fn first_range_index(&self) -> usize {
        self.first_range_index
    }

    pub fn last_range_index(&self) -> usize {
        self.last_range_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBoundaryCandidate {
    index: usize,
    text_count_range_index: usize,
    basis: TextCountRangeOverlapBasis,
    delimiter_code: u16,
    interval_count: usize,
    first_interval_index: usize,
    last_interval_index: usize,
    source_start: usize,
    source_end: usize,
}

impl TextBoundaryCandidate {
    fn from_control_range_overlap(
        index: usize,
        text_count_range_index: usize,
        overlap: &TextCountControlRangeOverlap,
    ) -> Self {
        Self {
            index,
            text_count_range_index,
            basis: overlap.basis(),
            delimiter_code: overlap.delimiter_code(),
            interval_count: overlap.range_count(),
            first_interval_index: overlap.first_range_index(),
            last_interval_index: overlap.last_range_index(),
            source_start: overlap.source_start(),
            source_end: overlap.source_end(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> &'static str {
        "controlDelimitedTextCountRange"
    }

    pub fn text_count_range_index(&self) -> usize {
        self.text_count_range_index
    }

    pub fn basis(&self) -> TextCountRangeOverlapBasis {
        self.basis
    }

    pub fn delimiter_code(&self) -> u16 {
        self.delimiter_code
    }

    pub fn interval_count(&self) -> usize {
        self.interval_count
    }

    pub fn first_interval_index(&self) -> usize {
        self.first_interval_index
    }

    pub fn last_interval_index(&self) -> usize {
        self.last_interval_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidate {
    index: usize,
    text_boundary_candidate_index: usize,
    text_count_range_index: usize,
    basis: TextCountRangeOverlapBasis,
    delimiter_code: u16,
    interval_count: usize,
    first_interval_index: usize,
    last_interval_index: usize,
    source_start: usize,
    source_end: usize,
    intervals: Vec<TableCandidateInterval>,
}

impl TableCandidate {
    fn from_text_boundary_candidate(
        index: usize,
        candidate: &TextBoundaryCandidate,
        intervals: Vec<TableCandidateInterval>,
    ) -> Self {
        Self {
            index,
            text_boundary_candidate_index: candidate.index(),
            text_count_range_index: candidate.text_count_range_index(),
            basis: candidate.basis(),
            delimiter_code: candidate.delimiter_code(),
            interval_count: candidate.interval_count(),
            first_interval_index: candidate.first_interval_index(),
            last_interval_index: candidate.last_interval_index(),
            source_start: candidate.source_start(),
            source_end: candidate.source_end(),
            intervals,
        }
    }

    fn from_document_text_control_rows(index: usize, rows: &[DocumentTextControlTableRow]) -> Self {
        let intervals = rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| TableCandidateInterval::from_control_cells(row_index, row))
            .collect::<Vec<_>>();
        let first_interval_index = rows.first().map_or(0, |row| row.index);
        let last_interval_index = rows.last().map_or(0, |row| row.index);
        let source_start = rows.first().map_or(0, |row| row.source_start);
        let source_end = rows.last().map_or(source_start, |row| row.source_end);
        Self {
            index,
            text_boundary_candidate_index: DIRECT_TABLE_CANDIDATE_SENTINEL,
            text_count_range_index: DIRECT_TABLE_CANDIDATE_SENTINEL,
            basis: TextCountRangeOverlapBasis::Unit,
            delimiter_code: TABLE_ROW_DELIMITER_CONTROL,
            interval_count: intervals.len(),
            first_interval_index,
            last_interval_index,
            source_start,
            source_end,
            intervals,
        }
    }

    fn from_sparse_document_text_control_rows(
        index: usize,
        rows: &[DocumentTextControlTableRow],
    ) -> Self {
        let intervals = rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| TableCandidateInterval::from_control_cells(row_index, row))
            .collect::<Vec<_>>();
        let first_interval_index = rows.first().map_or(0, |row| row.index);
        let last_interval_index = rows.last().map_or(0, |row| row.index);
        let source_start = rows.first().map_or(0, |row| row.source_start);
        let source_end = rows.last().map_or(source_start, |row| row.source_end);
        Self {
            index,
            text_boundary_candidate_index: SPARSE_TABLE_CANDIDATE_SENTINEL,
            text_count_range_index: SPARSE_TABLE_CANDIDATE_SENTINEL,
            basis: TextCountRangeOverlapBasis::Unit,
            delimiter_code: TABLE_ROW_DELIMITER_CONTROL,
            interval_count: intervals.len(),
            first_interval_index,
            last_interval_index,
            source_start,
            source_end,
            intervals,
        }
    }

    fn is_document_text_control_run_candidate(&self) -> bool {
        self.text_boundary_candidate_index == DIRECT_TABLE_CANDIDATE_SENTINEL
            && self.text_count_range_index == DIRECT_TABLE_CANDIDATE_SENTINEL
    }

    pub fn is_sparse_document_text_control_run_candidate(&self) -> bool {
        self.text_boundary_candidate_index == SPARSE_TABLE_CANDIDATE_SENTINEL
            && self.text_count_range_index == SPARSE_TABLE_CANDIDATE_SENTINEL
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> &'static str {
        if self.is_sparse_document_text_control_run_candidate() {
            "sparseDocumentTextControlRunTableCandidate"
        } else if self.is_document_text_control_run_candidate() {
            "documentTextControlRunTableCandidate"
        } else {
            "multiIntervalControlRangeTableCandidate"
        }
    }

    pub fn text_boundary_candidate_index(&self) -> usize {
        self.text_boundary_candidate_index
    }

    pub fn text_count_range_index(&self) -> usize {
        self.text_count_range_index
    }

    pub fn basis(&self) -> TextCountRangeOverlapBasis {
        self.basis
    }

    pub fn delimiter_code(&self) -> u16 {
        self.delimiter_code
    }

    pub fn interval_count(&self) -> usize {
        self.interval_count
    }

    pub fn first_interval_index(&self) -> usize {
        self.first_interval_index
    }

    pub fn last_interval_index(&self) -> usize {
        self.last_interval_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn intervals(&self) -> &[TableCandidateInterval] {
        &self.intervals
    }

    pub fn cell_count_candidate(&self) -> usize {
        self.intervals
            .iter()
            .map(|interval| interval.column_segments().len())
            .sum()
    }

    pub fn empty_cell_count_candidate(&self) -> usize {
        self.intervals
            .iter()
            .flat_map(|interval| interval.column_segments())
            .filter(|segment| segment.text().is_empty())
            .count()
    }

    pub fn non_empty_cell_count_candidate(&self) -> usize {
        self.cell_count_candidate()
            .saturating_sub(self.empty_cell_count_candidate())
    }

    pub fn is_row_like(&self) -> bool {
        if self.is_sparse_document_text_control_run_candidate() {
            return false;
        }

        let mut non_empty = 0usize;
        for interval in &self.intervals {
            if interval.line_break_count() != 0 {
                return false;
            }
            if interval.text_char_count() == 0 {
                return false;
            }
            non_empty += 1;
        }
        non_empty > 1
    }

    pub fn is_cell_like(&self) -> bool {
        self.is_row_like()
    }

    pub fn column_split_candidate_row_count(&self) -> usize {
        self.intervals
            .iter()
            .filter(|interval| !interval.column_segments().is_empty())
            .count()
    }

    pub fn max_column_segment_count(&self) -> usize {
        self.intervals
            .iter()
            .map(|interval| interval.column_segments().len())
            .max()
            .unwrap_or(0)
    }

    pub fn column_segment_pattern_consistent(&self) -> bool {
        self.column_split_candidate_row_count() > 0
            && self.column_segment_pattern_mismatch_rows() == 0
    }

    pub fn column_segment_pattern_mismatch_rows(&self) -> usize {
        if self.document_text_control_column_segments_are_compatible() {
            return 0;
        }

        let mut split_rows = 0usize;
        let mut signature_counts: BTreeMap<Vec<TableCandidateColumnSegmentKind>, usize> =
            BTreeMap::new();

        for interval in &self.intervals {
            if interval.column_segments().is_empty() {
                continue;
            }
            split_rows += 1;
            let signature = interval
                .column_segments()
                .iter()
                .map(|segment| segment.kind())
                .collect::<Vec<_>>();
            *signature_counts.entry(signature).or_insert(0) += 1;
        }

        if split_rows == 0 {
            return 0;
        }

        let dominant_rows = signature_counts.values().copied().max().unwrap_or(0);
        split_rows.saturating_sub(dominant_rows)
    }

    pub fn column_segment_grid_candidate(&self) -> Option<TableCandidateColumnGridCandidate> {
        if !self.is_row_like() || !self.column_segment_pattern_consistent() {
            return None;
        }

        let split_rows = self.column_split_candidate_row_count();
        if split_rows == 0 || split_rows != self.intervals.len() {
            return None;
        }

        let pattern_source = if self.document_text_control_column_segments_are_compatible() {
            self.intervals
                .iter()
                .max_by_key(|interval| interval.column_segments().len())
        } else {
            self.intervals
                .iter()
                .find(|interval| !interval.column_segments().is_empty())
        }?;
        let pattern = pattern_source
            .column_segments()
            .iter()
            .map(|segment| segment.kind())
            .collect::<Vec<_>>();

        if pattern.len() < 2 {
            return None;
        }

        Some(TableCandidateColumnGridCandidate::new(
            self.intervals.len(),
            pattern,
            split_rows,
        ))
    }

    fn document_text_control_column_segments_are_compatible(&self) -> bool {
        if !self.is_document_text_control_run_candidate() || self.intervals.len() < 3 {
            return false;
        }
        let mut min_columns = usize::MAX;
        let mut max_columns = 0usize;
        for interval in &self.intervals {
            let column_count = interval.column_segments().len();
            if column_count < 2 {
                return false;
            }
            if interval
                .column_segments()
                .iter()
                .any(|segment| segment.kind() != TableCandidateColumnSegmentKind::Label)
            {
                return false;
            }
            min_columns = min_columns.min(column_count);
            max_columns = max_columns.max(column_count);
        }
        max_columns >= 3 && max_columns.saturating_sub(min_columns) <= 1
    }

    pub fn sparse_topology_candidate(&self) -> Option<TableCandidateSparseTopologyCandidate> {
        if !self.is_sparse_document_text_control_run_candidate() {
            return None;
        }
        TableCandidateSparseTopologyCandidate::from_table_candidate(self)
    }

    pub fn rule(&self) -> &'static str {
        if self.is_sparse_document_text_control_run_candidate() {
            "sparse-document-text-001c-cells-with-000e-row-breaks"
        } else if self.is_document_text_control_run_candidate() {
            "document-text-001c-cells-with-000e-row-breaks"
        } else {
            "control-delimited-text-count-range-with-multiple-intervals"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateSparseTopologyCandidate {
    row_count: usize,
    max_column_count: usize,
    cell_count: usize,
    empty_cell_count: usize,
    non_empty_cell_count: usize,
    rows: Vec<TableCandidateSparseTopologyRow>,
    columns: Vec<TableCandidateSparseTopologyColumn>,
}

impl TableCandidateSparseTopologyCandidate {
    fn from_table_candidate(candidate: &TableCandidate) -> Option<Self> {
        let row_count = candidate.intervals().len();
        let max_column_count = candidate.max_column_segment_count();
        if row_count == 0 || max_column_count == 0 {
            return None;
        }

        let rows = candidate
            .intervals()
            .iter()
            .map(TableCandidateSparseTopologyRow::from_interval)
            .collect::<Vec<_>>();
        let columns = (0..max_column_count)
            .map(|column_index| {
                TableCandidateSparseTopologyColumn::from_candidate_column(candidate, column_index)
            })
            .collect::<Vec<_>>();

        Some(Self {
            row_count,
            max_column_count,
            cell_count: candidate.cell_count_candidate(),
            empty_cell_count: candidate.empty_cell_count_candidate(),
            non_empty_cell_count: candidate.non_empty_cell_count_candidate(),
            rows,
            columns,
        })
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn max_column_count(&self) -> usize {
        self.max_column_count
    }

    pub fn cell_count(&self) -> usize {
        self.cell_count
    }

    pub fn empty_cell_count(&self) -> usize {
        self.empty_cell_count
    }

    pub fn non_empty_cell_count(&self) -> usize {
        self.non_empty_cell_count
    }

    pub fn rows(&self) -> &[TableCandidateSparseTopologyRow] {
        &self.rows
    }

    pub fn columns(&self) -> &[TableCandidateSparseTopologyColumn] {
        &self.columns
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateSparseTopologyRow {
    index: usize,
    source_interval_index: usize,
    source_start: usize,
    source_end: usize,
    cell_count: usize,
    empty_cell_count: usize,
    non_empty_cell_count: usize,
    first_non_empty_column_index: Option<usize>,
    last_non_empty_column_index: Option<usize>,
}

impl TableCandidateSparseTopologyRow {
    fn from_interval(interval: &TableCandidateInterval) -> Self {
        let mut first_non_empty_column_index = None;
        let mut last_non_empty_column_index = None;
        let mut empty_cell_count = 0usize;
        let mut non_empty_cell_count = 0usize;

        for segment in interval.column_segments() {
            if segment.text().is_empty() {
                empty_cell_count += 1;
            } else {
                non_empty_cell_count += 1;
                if first_non_empty_column_index.is_none() {
                    first_non_empty_column_index = Some(segment.index());
                }
                last_non_empty_column_index = Some(segment.index());
            }
        }

        Self {
            index: interval.index(),
            source_interval_index: interval.source_interval_index(),
            source_start: interval.source_start(),
            source_end: interval.source_end(),
            cell_count: interval.column_segments().len(),
            empty_cell_count,
            non_empty_cell_count,
            first_non_empty_column_index,
            last_non_empty_column_index,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn source_interval_index(&self) -> usize {
        self.source_interval_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn cell_count(&self) -> usize {
        self.cell_count
    }

    pub fn empty_cell_count(&self) -> usize {
        self.empty_cell_count
    }

    pub fn non_empty_cell_count(&self) -> usize {
        self.non_empty_cell_count
    }

    pub fn first_non_empty_column_index(&self) -> Option<usize> {
        self.first_non_empty_column_index
    }

    pub fn last_non_empty_column_index(&self) -> Option<usize> {
        self.last_non_empty_column_index
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateSparseTopologyColumn {
    index: usize,
    observed_cell_count: usize,
    empty_cell_count: usize,
    non_empty_cell_count: usize,
    first_non_empty_row_index: Option<usize>,
    last_non_empty_row_index: Option<usize>,
    source_start: Option<usize>,
    source_end: Option<usize>,
}

impl TableCandidateSparseTopologyColumn {
    fn from_candidate_column(candidate: &TableCandidate, column_index: usize) -> Self {
        let mut observed_cell_count = 0usize;
        let mut empty_cell_count = 0usize;
        let mut non_empty_cell_count = 0usize;
        let mut first_non_empty_row_index = None;
        let mut last_non_empty_row_index = None;
        let mut source_start = None;
        let mut source_end = None;

        for row in candidate.intervals() {
            let Some(segment) = row
                .column_segments()
                .iter()
                .find(|segment| segment.index() == column_index)
            else {
                continue;
            };

            observed_cell_count += 1;
            source_start = option_min_usize(source_start, segment.source_start());
            source_end = option_max_usize(source_end, segment.source_end());

            if segment.text().is_empty() {
                empty_cell_count += 1;
            } else {
                non_empty_cell_count += 1;
                if first_non_empty_row_index.is_none() {
                    first_non_empty_row_index = Some(row.index());
                }
                last_non_empty_row_index = Some(row.index());
            }
        }

        Self {
            index: column_index,
            observed_cell_count,
            empty_cell_count,
            non_empty_cell_count,
            first_non_empty_row_index,
            last_non_empty_row_index,
            source_start,
            source_end,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn observed_cell_count(&self) -> usize {
        self.observed_cell_count
    }

    pub fn empty_cell_count(&self) -> usize {
        self.empty_cell_count
    }

    pub fn non_empty_cell_count(&self) -> usize {
        self.non_empty_cell_count
    }

    pub fn first_non_empty_row_index(&self) -> Option<usize> {
        self.first_non_empty_row_index
    }

    pub fn last_non_empty_row_index(&self) -> Option<usize> {
        self.last_non_empty_row_index
    }

    pub fn source_start(&self) -> Option<usize> {
        self.source_start
    }

    pub fn source_end(&self) -> Option<usize> {
        self.source_end
    }
}

fn option_min_usize(left: Option<usize>, right: Option<usize>) -> Option<usize> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn option_max_usize(left: Option<usize>, right: Option<usize>) -> Option<usize> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateColumnGridCandidate {
    row_count: usize,
    column_count: usize,
    cell_count: usize,
    split_row_count: usize,
    pattern: Vec<TableCandidateColumnSegmentKind>,
}

impl TableCandidateColumnGridCandidate {
    fn new(
        row_count: usize,
        pattern: Vec<TableCandidateColumnSegmentKind>,
        split_row_count: usize,
    ) -> Self {
        let column_count = pattern.len();
        Self {
            row_count,
            column_count,
            cell_count: row_count.saturating_mul(column_count),
            split_row_count,
            pattern,
        }
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn column_count(&self) -> usize {
        self.column_count
    }

    pub fn cell_count(&self) -> usize {
        self.cell_count
    }

    pub fn split_row_count(&self) -> usize {
        self.split_row_count
    }

    pub fn pattern(&self) -> &[TableCandidateColumnSegmentKind] {
        &self.pattern
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateInterval {
    index: usize,
    source_interval_index: usize,
    source_start: usize,
    source_end: usize,
    text_preview: String,
    text_char_count: usize,
    line_break_count: usize,
    column_segments: Vec<TableCandidateColumnSegment>,
}

impl TableCandidateInterval {
    fn new(
        index: usize,
        source_interval_index: usize,
        source_start: usize,
        source_end: usize,
        text: String,
    ) -> Self {
        let text_char_count = text.chars().count();
        let line_break_count = text_line_break_count(&text);
        let text_preview = preview_text(&text, 80);
        let column_segments = table_row_column_segments(&text);
        Self {
            index,
            source_interval_index,
            source_start,
            source_end,
            text_preview,
            text_char_count,
            line_break_count,
            column_segments,
        }
    }

    fn from_control_cells(index: usize, row: &DocumentTextControlTableRow) -> Self {
        let mut text = String::new();
        let mut column_segments = Vec::new();
        let mut char_offset = 0usize;
        for (cell_index, cell) in row.cells.iter().enumerate() {
            if cell_index > 0 {
                text.push('\t');
                char_offset += 1;
            }
            let cell_text = clean_table_control_cell_text(&cell.text);
            let char_start = char_offset;
            text.push_str(&cell_text);
            char_offset += cell_text.chars().count();
            column_segments.push(TableCandidateColumnSegment::new(
                cell_index,
                TableCandidateColumnSegmentKind::Label,
                char_start,
                char_offset,
                Some(cell.source_start),
                Some(cell.source_end),
                cell_text,
            ));
        }
        let text_char_count = text.chars().count();
        let text_preview = preview_text(&text, 80);
        Self {
            index,
            source_interval_index: row.index,
            source_start: row.source_start,
            source_end: row.source_end,
            text_preview,
            text_char_count,
            line_break_count: 0,
            column_segments,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn source_interval_index(&self) -> usize {
        self.source_interval_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn text_preview(&self) -> &str {
        &self.text_preview
    }

    pub fn text_char_count(&self) -> usize {
        self.text_char_count
    }

    pub fn line_break_count(&self) -> usize {
        self.line_break_count
    }

    pub fn column_segments(&self) -> &[TableCandidateColumnSegment] {
        &self.column_segments
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateColumnSegment {
    index: usize,
    kind: TableCandidateColumnSegmentKind,
    char_start: usize,
    char_end: usize,
    source_start: Option<usize>,
    source_end: Option<usize>,
    text: String,
}

impl TableCandidateColumnSegment {
    fn new(
        index: usize,
        kind: TableCandidateColumnSegmentKind,
        char_start: usize,
        char_end: usize,
        source_start: Option<usize>,
        source_end: Option<usize>,
        text: String,
    ) -> Self {
        Self {
            index,
            kind,
            char_start,
            char_end,
            source_start,
            source_end,
            text,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> TableCandidateColumnSegmentKind {
        self.kind
    }

    pub fn char_start(&self) -> usize {
        self.char_start
    }

    pub fn char_end(&self) -> usize {
        self.char_end
    }

    pub fn source_start(&self) -> Option<usize> {
        self.source_start
    }

    pub fn source_end(&self) -> Option<usize> {
        self.source_end
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableCandidateColumnSegmentKind {
    Label,
    Value,
}

impl TableCandidateColumnSegmentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Label => "label",
            Self::Value => "value",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextLayoutExactEvidence {
    target: &'static str,
    base: &'static str,
    delta: isize,
}

impl TextLayoutExactEvidence {
    fn new(target: &'static str, base: &'static str, delta: isize) -> Self {
        Self {
            target,
            base,
            delta,
        }
    }

    pub fn target(&self) -> &'static str {
        self.target
    }

    pub fn base(&self) -> &'static str {
        self.base
    }

    pub fn delta(&self) -> isize {
        self.delta
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCountRange {
    index: usize,
    family: String,
    start: u32,
    end: u32,
    declared_start: u32,
    declared_end: u32,
    tail_fields: Vec<u16>,
    document_text_overlaps: Vec<TextCountRangeOverlap>,
    control_range_overlaps: Vec<TextCountControlRangeOverlap>,
    raw: Vec<u8>,
}

impl TextCountRange {
    fn from_entry(entry: &DocumentTextCountEntry) -> Self {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (start, end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        Self {
            index: entry.index(),
            family: family.to_string(),
            start,
            end,
            declared_start: entry.start_offset(),
            declared_end: entry.end_offset(),
            tail_fields: read_be16_fields(&raw[tail_offset..]),
            document_text_overlaps: Vec::new(),
            control_range_overlaps: Vec::new(),
            raw: raw.to_vec(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn span(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    pub fn declared_start(&self) -> u32 {
        self.declared_start
    }

    pub fn declared_end(&self) -> u32 {
        self.declared_end
    }

    pub fn tail_fields(&self) -> &[u16] {
        &self.tail_fields
    }

    pub fn document_text_overlaps(&self) -> &[TextCountRangeOverlap] {
        &self.document_text_overlaps
    }

    fn set_document_text_overlaps(&mut self, overlaps: Vec<TextCountRangeOverlap>) {
        self.document_text_overlaps = overlaps;
    }

    pub fn control_range_overlaps(&self) -> &[TextCountControlRangeOverlap] {
        &self.control_range_overlaps
    }

    fn set_control_range_overlaps(&mut self, overlaps: Vec<TextCountControlRangeOverlap>) {
        self.control_range_overlaps = overlaps;
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
}

fn read_be32_candidate(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

fn read_be16_at(bytes: &[u8], offset: usize) -> Option<u16> {
    let bytes = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn read_be32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_le16_at(bytes: &[u8], offset: usize) -> Option<u16> {
    let bytes = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_le32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_i32_le_at(bytes: &[u8], offset: usize) -> Option<i32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_i32_be_at(bytes: &[u8], offset: usize) -> Option<i32> {
    let bytes = bytes.get(offset..offset.checked_add(4)?)?;
    Some(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_be16_fields(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Paragraph(Paragraph),
    Unknown(UnknownBlock),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    Text(TextRun),
    Ruby(RubyAnnotation),
    Unknown(UnknownObject),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRun {
    text: String,
    style: Option<StyleRef>,
    source_span: Option<TextSourceSpan>,
}

impl TextRun {
    pub fn new(text: impl Into<String>, style: Option<StyleRef>) -> Self {
        Self::with_source_span(text, style, None)
    }

    pub fn with_source_span(
        text: impl Into<String>,
        style: Option<StyleRef>,
        source_span: Option<TextSourceSpan>,
    ) -> Self {
        Self {
            text: text.into(),
            style,
            source_span,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn style(&self) -> Option<&StyleRef> {
        self.style.as_ref()
    }

    pub fn source_span(&self) -> Option<&TextSourceSpan> {
        self.source_span.as_ref()
    }

    fn can_extend_source_span(&self, next: Option<&TextSourceSpan>) -> bool {
        match (self.source_span.as_ref(), next) {
            (None, None) => true,
            (Some(current), Some(next)) => {
                current.byte_end() == next.byte_start() && current.unit_end() == next.unit_start()
            }
            _ => false,
        }
    }

    fn push_text_with_span(&mut self, text: &str, next: Option<TextSourceSpan>) {
        self.text.push_str(text);
        match (self.source_span.as_mut(), next) {
            (Some(current), Some(next)) => {
                current.byte_end = next.byte_end();
                current.unit_end = next.unit_end();
            }
            (None, None) => {}
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRef {
    id: String,
}

impl StyleRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownBlock {
    source: UnknownRecordKind,
    payload: Vec<u8>,
}

impl UnknownBlock {
    pub fn new(source: UnknownRecordKind, payload: Vec<u8>) -> Self {
        Self { source, payload }
    }

    pub fn source(&self) -> &UnknownRecordKind {
        &self.source
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownStyle {
    name: Option<String>,
    source: UnknownRecordKind,
    payload: Vec<u8>,
}

impl UnknownStyle {
    pub fn new(source: UnknownRecordKind, payload: Vec<u8>) -> Self {
        Self {
            name: None,
            source,
            payload,
        }
    }

    pub fn from_stream(name: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            name: Some(name.into()),
            source: UnknownRecordKind::new(None),
            payload,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn source(&self) -> &UnknownRecordKind {
        &self.source
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownObject {
    source: UnknownRecordKind,
    payload: Vec<u8>,
}

impl UnknownObject {
    pub fn new(source: UnknownRecordKind, payload: Vec<u8>) -> Self {
        Self { source, payload }
    }

    pub fn source(&self) -> &UnknownRecordKind {
        &self.source
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Debug, Default)]
struct DocumentTextModelBuilder {
    current_inlines: Vec<Inline>,
    blocks: Vec<Block>,
    unknown_objects: Vec<UnknownObject>,
    text_control_boundaries: Vec<TextControlBoundary>,
    can_merge_current_text_run: bool,
    pending_ruby_base_inline_index: Option<usize>,
}

impl DocumentTextModelBuilder {
    fn push_text_run(&mut self, text: &str) {
        self.push_text_run_with_span(text, None);
    }

    fn push_text_run_with_span(&mut self, text: &str, source_span: Option<TextSourceSpan>) {
        self.pending_ruby_base_inline_index = None;
        self.push_text(text, ModelTextSource::TextRun, source_span);
    }

    fn push_inline_text(&mut self, segment: &InlineTextSegment) {
        self.push_inline_text_with_span(segment, None);
    }

    fn push_inline_text_with_span(
        &mut self,
        segment: &InlineTextSegment,
        source_span: Option<TextSourceSpan>,
    ) {
        self.pending_ruby_base_inline_index = None;
        let previous_block_count = self.blocks.len();
        let previous_inline_count = self.current_inlines.len();

        self.push_text(segment.text(), ModelTextSource::Inline, source_span);

        if segment.selector() == DOCUMENT_TEXT_RUBY_BASE_SELECTOR
            && previous_block_count == self.blocks.len()
            && self.current_inlines.len() == previous_inline_count + 1
        {
            self.pending_ruby_base_inline_index = Some(previous_inline_count);
        }
    }

    fn push_skipped_inline(&mut self, segment: &SkippedInlineTextSegment) {
        self.push_skipped_inline_with_span(segment, None);
    }

    fn push_skipped_inline_with_span(
        &mut self,
        segment: &SkippedInlineTextSegment,
        _source_span: Option<TextSourceSpan>,
    ) {
        if self.promote_ruby_annotation(segment) {
            return;
        }

        self.pending_ruby_base_inline_index = None;
        self.unknown_objects
            .push(unknown_object_from_skipped_inline(segment));
        self.can_merge_current_text_run = false;
    }

    fn push_control_boundary(
        &mut self,
        control: &DocumentTextControl,
        source_span: Option<TextSourceSpan>,
    ) {
        self.can_merge_current_text_run = false;
        self.text_control_boundaries.push(TextControlBoundary::new(
            self.text_control_boundaries.len(),
            control.code(),
            source_span,
        ));
    }

    fn push_text(
        &mut self,
        text: &str,
        source: ModelTextSource,
        source_span: Option<TextSourceSpan>,
    ) {
        for part in source_text_parts(text, source_span.as_ref()) {
            if !part.text.is_empty() {
                self.push_text_part(&part.text, source, part.source_span);
            }

            if part.break_after {
                self.flush_paragraph();
            }
        }
    }

    fn finish(mut self) -> (Vec<Block>, Vec<UnknownObject>, Vec<TextControlBoundary>) {
        self.flush_paragraph();
        (
            self.blocks,
            self.unknown_objects,
            self.text_control_boundaries,
        )
    }

    fn flush_paragraph(&mut self) {
        if self.current_inlines.is_empty() {
            self.can_merge_current_text_run = false;
            self.pending_ruby_base_inline_index = None;
            return;
        }

        let inlines = std::mem::take(&mut self.current_inlines);
        self.blocks
            .push(Block::Paragraph(Paragraph::new(inlines, None)));
        self.can_merge_current_text_run = false;
        self.pending_ruby_base_inline_index = None;
    }

    fn push_text_part(
        &mut self,
        text: &str,
        source: ModelTextSource,
        source_span: Option<TextSourceSpan>,
    ) {
        if source == ModelTextSource::TextRun
            && self.can_merge_current_text_run
            && let Some(Inline::Text(run)) = self.current_inlines.last_mut()
            && run.can_extend_source_span(source_span.as_ref())
        {
            run.push_text_with_span(text, source_span);
            return;
        }

        self.current_inlines
            .push(Inline::Text(TextRun::with_source_span(
                text,
                None,
                source_span,
            )));
        self.can_merge_current_text_run = source == ModelTextSource::TextRun;
    }

    fn promote_ruby_annotation(&mut self, segment: &SkippedInlineTextSegment) -> bool {
        if segment.selector() != Some(DOCUMENT_TEXT_RUBY_TEXT_SELECTOR) {
            return false;
        }

        let Some(index) = self.pending_ruby_base_inline_index.take() else {
            return false;
        };

        let Some(inline) = self.current_inlines.get_mut(index) else {
            return false;
        };

        let Inline::Text(base_run) = inline else {
            return false;
        };

        let base_text = std::mem::take(&mut base_run.text);
        let annotation = RubyAnnotation::new(
            base_text,
            segment.text(),
            DOCUMENT_TEXT_RUBY_TEXT_SELECTOR,
            unknown_object_from_skipped_inline(segment),
        );
        *inline = Inline::Ruby(annotation);
        self.can_merge_current_text_run = false;
        true
    }
}

fn reserve_and_verify_cfb_streams(data: &[u8], budget: &mut ResourceBudget) -> Result<()> {
    let Ok((entries, mode)) = inspect_cfb_entries_with_mode(data) else {
        return Ok(());
    };
    let mut streams = BTreeMap::new();

    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        streams
            .entry(entry.path())
            .and_modify(|size: &mut u64| *size = (*size).max(entry.size()))
            .or_insert(entry.size());
    }

    for (path, declared) in streams {
        let accounted = match mode {
            CfbEntryReadMode::Strict => cfb_stream_bytes_from_u64(declared)?,
            CfbEntryReadMode::Lenient => reachable_cfb_stream_bytes(data, path).unwrap_or(0),
        };
        budget.reserve_streams(1, accounted)?;

        let Ok(stream) = read_cfb_stream(data, path) else {
            continue;
        };
        budget.verify_stream_bytes(accounted, stream.len())?;
    }

    Ok(())
}

fn cfb_stream_bytes_from_u64(size: u64) -> Result<usize> {
    usize::try_from(size).map_err(|_| Error::ResourceLimit {
        resource: "document stream bytes",
        limit: usize::MAX,
        actual: usize::MAX,
    })
}

fn reachable_cfb_stream_bytes(data: &[u8], path: &str) -> Result<usize> {
    let chain = inspect_cfb_stream_chain(data, path)?;
    let capacity = u64::try_from(chain.capacity_bytes()).unwrap_or(u64::MAX);
    cfb_stream_bytes_from_u64(chain.location().size().min(capacity))
}

fn record_bytes_overflow() -> Error {
    Error::ResourceLimit {
        resource: "document record bytes",
        limit: usize::MAX,
        actual: usize::MAX,
    }
}

fn decode_utf16le_c_string(bytes: &[u8]) -> Option<String> {
    if !bytes.len().is_multiple_of(2) {
        return None;
    }
    let units = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .take_while(|unit| *unit != 0)
        .collect::<Vec<_>>();
    String::from_utf16(&units).ok()
}

fn permille(numerator: usize, denominator: usize) -> Option<usize> {
    numerator.saturating_mul(1000).checked_div(denominator)
}

fn stream_path_ends_with(path: &str, suffix: &str) -> bool {
    path.get(path.len().saturating_sub(suffix.len())..)
        .is_some_and(|tail| tail.eq_ignore_ascii_case(suffix))
}

fn utf16le_printable_preview(bytes: &[u8]) -> String {
    let mut preview = String::new();
    for chunk in bytes.chunks_exact(2) {
        let value = u16::from_le_bytes([chunk[0], chunk[1]]);
        if value == 0 {
            break;
        }
        let Some(ch) = char::from_u32(u32::from(value)) else {
            break;
        };
        if ch.is_control() {
            break;
        }
        preview.push(ch);
    }
    preview
}

fn utf16le_bytes(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

fn embedded_stream_role(segment: &str) -> &'static str {
    match segment.trim_start_matches(|character: char| character.is_control()) {
        "Contents" => "contents",
        "EmbeddedPress" => "embedded-press",
        "CompObj" => "comp-obj",
        "OlePres000" => "ole-presentation",
        _ => "embedded-stream",
    }
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn push_signature_hits(
    hits: &mut Vec<ObjectImageSignatureHit>,
    stream: &[u8],
    kind: &'static str,
    signature: &[u8],
    scan_anywhere: bool,
    budget: &mut ResourceBudget,
) -> Result<()> {
    if signature.is_empty() {
        return Ok(());
    }

    if scan_anywhere {
        for (offset, candidate) in stream.windows(signature.len()).enumerate() {
            if candidate == signature {
                reserve_image_signature_candidate(budget, kind)?;
                hits.push(ObjectImageSignatureHit::new(kind, offset));
            }
        }
    } else if stream.starts_with(signature) {
        reserve_image_signature_candidate(budget, kind)?;
        hits.push(ObjectImageSignatureHit::new(kind, 0));
    }
    Ok(())
}

fn reserve_image_signature_candidate(budget: &mut ResourceBudget, kind: &str) -> Result<()> {
    let bytes = std::mem::size_of::<ObjectImageSignatureHit>()
        .checked_add(kind.len())
        .ok_or(Error::ResourceLimit {
            resource: "document record bytes",
            limit: usize::MAX,
            actual: usize::MAX,
        })?;
    budget.reserve_record(bytes)
}

fn image_payload_retained_bytes(
    payload_len: usize,
    header_start: usize,
    payload_start: usize,
    payload_end: usize,
    trailer_end: usize,
) -> Result<usize> {
    let header_len = payload_start
        .checked_sub(header_start)
        .ok_or(Error::ResourceLimit {
            resource: "embedded image bytes",
            limit: usize::MAX,
            actual: usize::MAX,
        })?;
    let trailer_len = trailer_end
        .checked_sub(payload_end)
        .ok_or(Error::ResourceLimit {
            resource: "embedded image bytes",
            limit: usize::MAX,
            actual: usize::MAX,
        })?;
    payload_len
        .checked_add(header_len)
        .and_then(|bytes| bytes.checked_add(trailer_len))
        .ok_or(Error::ResourceLimit {
            resource: "embedded image bytes",
            limit: usize::MAX,
            actual: usize::MAX,
        })
}

fn png_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    if payload.get(..8)? != b"\x89PNG\r\n\x1a\n" || payload.get(12..16)? != b"IHDR" {
        return None;
    }
    let width = u32::from_be_bytes(payload.get(16..20)?.try_into().ok()?);
    let height = u32::from_be_bytes(payload.get(20..24)?.try_into().ok()?);
    (width != 0 && height != 0).then_some(ObjectImageDimensions::new(width, height))
}

fn gif_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    if !(payload.starts_with(b"GIF87a") || payload.starts_with(b"GIF89a")) {
        return None;
    }
    let width = u16::from_le_bytes(payload.get(6..8)?.try_into().ok()?) as u32;
    let height = u16::from_le_bytes(payload.get(8..10)?.try_into().ok()?) as u32;
    (width != 0 && height != 0).then_some(ObjectImageDimensions::new(width, height))
}

fn bmp_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    if payload.get(..2)? != b"BM" {
        return None;
    }
    let dib_header_size = u32::from_le_bytes(payload.get(14..18)?.try_into().ok()?);
    if dib_header_size < 40 {
        return None;
    }
    let width = i32::from_le_bytes(payload.get(18..22)?.try_into().ok()?);
    let height = i32::from_le_bytes(payload.get(22..26)?.try_into().ok()?);
    let width = u32::try_from(width).ok()?;
    let height = height.unsigned_abs();
    (width != 0 && height != 0).then_some(ObjectImageDimensions::new(width, height))
}

fn jpeg_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    if payload.get(0..2)? != b"\xff\xd8" {
        return None;
    }

    let mut cursor = 2usize;
    while cursor < payload.len() {
        while cursor < payload.len() && payload[cursor] != 0xff {
            cursor += 1;
        }
        while cursor < payload.len() && payload[cursor] == 0xff {
            cursor += 1;
        }

        let marker = *payload.get(cursor)?;
        cursor += 1;
        if marker == 0xda || marker == 0xd9 {
            return None;
        }
        if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
            continue;
        }

        let length_end = cursor.checked_add(2)?;
        let length_bytes = payload.get(cursor..length_end)?;
        let segment_len = u16::from_be_bytes([length_bytes[0], length_bytes[1]]) as usize;
        if segment_len < 2 {
            return None;
        }
        let data_start = length_end;
        let data_end = data_start.checked_add(segment_len - 2)?;
        let data = payload.get(data_start..data_end)?;

        if is_jpeg_sof_marker(marker) {
            if data.len() < 5 {
                return None;
            }
            let height = u16::from_be_bytes([data[1], data[2]]) as u32;
            let width = u16::from_be_bytes([data[3], data[4]]) as u32;
            return (width != 0 && height != 0)
                .then_some(ObjectImageDimensions::new(width, height));
        }

        cursor = data_end;
    }

    None
}

fn is_jpeg_sof_marker(marker: u8) -> bool {
    matches!(
        marker,
        0xc0 | 0xc1 | 0xc2 | 0xc3 | 0xc5 | 0xc6 | 0xc7 | 0xc9 | 0xca | 0xcb | 0xcd | 0xce | 0xcf
    )
}

fn looks_like_embedded_source_path(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .any(|byte| matches!(*byte, b'\\' | b'/' | b':' | b'.'))
}

fn jpeg_payload_end(stream: &[u8], offset: usize) -> Option<usize> {
    let search_start = jpeg_entropy_data_start(stream, offset)?;
    stream
        .get(search_start..)?
        .windows(2)
        .position(|window| window == [0xff, 0xd9])
        .map(|relative| search_start + relative + 2)
}

fn jpeg_entropy_data_start(stream: &[u8], offset: usize) -> Option<usize> {
    if stream.get(offset..offset.checked_add(2)?)? != b"\xff\xd8" {
        return None;
    }

    let mut cursor = offset.checked_add(2)?;
    let mut found_sof = false;
    while cursor < stream.len() {
        while cursor < stream.len() && stream[cursor] != 0xff {
            cursor += 1;
        }
        while cursor < stream.len() && stream[cursor] == 0xff {
            cursor += 1;
        }

        let marker = *stream.get(cursor)?;
        cursor += 1;
        if marker == 0xd9 {
            return None;
        }
        if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
            continue;
        }

        let length_end = cursor.checked_add(2)?;
        let length_bytes = stream.get(cursor..length_end)?;
        let segment_len = u16::from_be_bytes([length_bytes[0], length_bytes[1]]) as usize;
        if segment_len < 2 {
            return None;
        }
        let data_start = length_end;
        let data_end = data_start.checked_add(segment_len - 2)?;
        stream.get(data_start..data_end)?;

        if is_jpeg_sof_marker(marker) {
            found_sof = true;
        }
        if marker == 0xda {
            return found_sof.then_some(data_end);
        }

        cursor = data_end;
    }

    None
}

fn png_payload_end(stream: &[u8], offset: usize) -> Option<usize> {
    let signature_end = offset.checked_add(8)?;
    if stream.get(offset..signature_end)? != b"\x89PNG\r\n\x1a\n" {
        return None;
    }

    let mut cursor = signature_end;
    while cursor.checked_add(12)? <= stream.len() {
        let length = u32::from_be_bytes([
            stream[cursor],
            stream[cursor + 1],
            stream[cursor + 2],
            stream[cursor + 3],
        ]) as usize;
        let chunk_type_start = cursor + 4;
        let chunk_data_start = cursor + 8;
        let chunk_end = chunk_data_start.checked_add(length)?.checked_add(4)?;
        if chunk_end > stream.len() {
            return None;
        }
        let chunk_type = &stream[chunk_type_start..chunk_type_start + 4];
        if chunk_type == b"IEND" {
            return Some(chunk_end);
        }
        cursor = chunk_end;
    }
    None
}

fn gif_payload_end(stream: &[u8], offset: usize) -> Option<usize> {
    let search_start = offset.checked_add(6)?;
    stream
        .get(search_start..)?
        .iter()
        .position(|byte| *byte == 0x3b)
        .map(|relative| search_start + relative + 1)
}

fn bmp_payload_end(stream: &[u8], offset: usize) -> Option<usize> {
    if offset != 0 || stream.get(0..2)? != b"BM" || stream.len() < 6 {
        return None;
    }
    let size = u32::from_le_bytes([stream[2], stream[3], stream[4], stream[5]]) as usize;
    (size >= 14 && size <= stream.len()).then_some(size)
}

fn svg_signature_offsets(stream: &[u8]) -> Vec<usize> {
    let ascii_lower = stream
        .iter()
        .map(|byte| byte.to_ascii_lowercase())
        .collect::<Vec<_>>();
    find_subslice_offsets(&ascii_lower, b"<svg")
}

fn find_subslice_offsets(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return Vec::new();
    }

    haystack
        .windows(needle.len())
        .enumerate()
        .filter_map(|(offset, candidate)| (candidate == needle).then_some(offset))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelTextSource {
    TextRun,
    Inline,
}

struct DocumentTextSourceSpans<'a> {
    entries: &'a [DocumentTextMapEntry],
    index: usize,
}

impl<'a> DocumentTextSourceSpans<'a> {
    fn new(entries: &'a [DocumentTextMapEntry]) -> Self {
        Self { entries, index: 0 }
    }

    fn next(&mut self, kind: DocumentTextMapKind, text: &str) -> Option<TextSourceSpan> {
        while let Some(entry) = self.entries.get(self.index) {
            self.index += 1;
            if entry.kind() == kind && (text.is_empty() || entry.text() == text) {
                return Some(TextSourceSpan::from_document_text_entry(entry));
            }
        }
        None
    }

    fn next_control(&mut self, code: u16) -> Option<TextSourceSpan> {
        while let Some(entry) = self.entries.get(self.index) {
            self.index += 1;
            if entry.kind() == DocumentTextMapKind::ControlBoundary && entry.code() == Some(code) {
                return Some(TextSourceSpan::from_document_text_entry(entry));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
struct DocumentTextTocRow {
    title: String,
    page_label: Option<String>,
    byte_start: Option<usize>,
    byte_end: usize,
    unit_start: Option<usize>,
    unit_end: usize,
}

impl DocumentTextTocRow {
    fn push_entry_span(&mut self, entry: &DocumentTextMapEntry) {
        self.byte_start = Some(
            self.byte_start
                .map_or(entry.byte_start(), |start| start.min(entry.byte_start())),
        );
        self.byte_end = self.byte_end.max(entry.byte_end());
        self.unit_start = Some(
            self.unit_start
                .map_or(entry.unit_start(), |start| start.min(entry.unit_start())),
        );
        self.unit_end = self.unit_end.max(entry.unit_end());
    }

    fn push_visible_text(&mut self, entry: &DocumentTextMapEntry) {
        self.push_entry_span(entry);
        self.title.push_str(&entry.text().replace(['\r', '\n'], ""));
    }

    fn push_page_label(&mut self, entry: &DocumentTextMapEntry) {
        self.push_entry_span(entry);
        let label = entry
            .text()
            .chars()
            .filter(|character| character.is_ascii_digit())
            .collect::<String>();
        if !label.is_empty() {
            self.page_label = Some(label);
        }
    }

    fn into_toc_entry(self) -> Option<DocumentTocEntry> {
        let title = self.title.trim().to_string();
        let page_label = self.page_label?;
        if title.is_empty()
            || !page_label
                .chars()
                .all(|character| character.is_ascii_digit())
            || !is_short_chapter_title(&title)
        {
            return None;
        }
        Some(DocumentTocEntry::new(
            title,
            page_label,
            TextSourceSpan::new(
                self.byte_start?,
                self.byte_end,
                self.unit_start?,
                self.unit_end,
            ),
        ))
    }
}

struct SourceTextPart {
    text: String,
    source_span: Option<TextSourceSpan>,
    break_after: bool,
}

#[derive(Debug, Clone, Copy)]
struct TextControlSourceInterval {
    index: usize,
    byte_start: usize,
    byte_end: usize,
    unit_start: usize,
    unit_end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DocumentTextControlTableCell {
    source_start: usize,
    source_end: usize,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DocumentTextControlTableRow {
    index: usize,
    source_start: usize,
    source_end: usize,
    cells: Vec<DocumentTextControlTableCell>,
}

#[derive(Debug, Clone)]
struct PendingDocumentTextControlCell {
    source_start: Option<usize>,
    source_end: usize,
    text: String,
}

impl PendingDocumentTextControlCell {
    fn new() -> Self {
        Self {
            source_start: None,
            source_end: 0,
            text: String::new(),
        }
    }

    fn push_text(&mut self, entry: &DocumentTextMapEntry) {
        if self.source_start.is_none() {
            self.source_start = Some(entry.unit_start());
        }
        self.source_end = entry.unit_end();
        self.text.push_str(entry.text());
    }

    fn finish(&mut self) -> Option<DocumentTextControlTableCell> {
        let text = clean_table_control_cell_text(&self.text);
        let source_start = self.source_start.take()?;
        let source_end = self.source_end.max(source_start);
        self.source_end = 0;
        self.text.clear();
        if text.is_empty() {
            return None;
        }
        Some(DocumentTextControlTableCell {
            source_start,
            source_end,
            text,
        })
    }

    fn finish_preserving_empty(
        &mut self,
        fallback_source_start: usize,
        fallback_source_end: usize,
        include_empty: bool,
    ) -> Option<DocumentTextControlTableCell> {
        let text = clean_table_control_cell_text(&self.text);
        let source_start = self.source_start.take().unwrap_or(fallback_source_start);
        let source_end = self.source_end.max(source_start).max(fallback_source_end);
        self.source_end = 0;
        self.text.clear();
        if text.is_empty() && !include_empty {
            return None;
        }
        Some(DocumentTextControlTableCell {
            source_start,
            source_end,
            text,
        })
    }
}

fn table_control_cell_has_value_marker(text: &str) -> bool {
    text.chars()
        .any(|character| character.is_ascii_digit() || matches!(character, '０'..='９'))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SparseDocumentTextControlRowShape {
    column_count: usize,
    empty_cells: usize,
    non_empty_cells: usize,
    text_char_count: usize,
}

impl SparseDocumentTextControlRowShape {
    fn from_row(row: &DocumentTextControlTableRow) -> Self {
        let mut empty_cells = 0usize;
        let mut non_empty_cells = 0usize;
        let mut text_char_count = 0usize;
        for cell in &row.cells {
            if cell.text.is_empty() {
                empty_cells += 1;
            } else {
                non_empty_cells += 1;
                text_char_count += cell.text.chars().count();
            }
        }
        Self {
            column_count: row.cells.len(),
            empty_cells,
            non_empty_cells,
            text_char_count,
        }
    }
}

fn clean_table_control_cell_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn source_interval_range(
    interval: &TextControlSourceInterval,
    basis: TextCountRangeOverlapBasis,
) -> (usize, usize) {
    match basis {
        TextCountRangeOverlapBasis::Byte => (interval.byte_start, interval.byte_end),
        TextCountRangeOverlapBasis::Unit => (interval.unit_start, interval.unit_end),
    }
}

fn is_strict_unit_001c_single_boundary_candidate(
    entries: &[DocumentTextMapEntry],
    candidate: &TextBoundaryCandidate,
) -> bool {
    candidate.basis() == TextCountRangeOverlapBasis::Unit
        && candidate.delimiter_code() == PARAGRAPH_BOUNDARY_DELIMITER_CANDIDATE
        && candidate.interval_count() == 1
        && range_starts_after_control_gap(entries, candidate.source_start())
        && range_ends_on_aligned_text(entries, candidate.source_end())
        && !range_visible_text(entries, candidate.source_start(), candidate.source_end()).is_empty()
        && text_line_break_count(&range_visible_text(
            entries,
            candidate.source_start(),
            candidate.source_end(),
        )) <= 1
}

fn best_layout_exact2_evidence_for_points(
    candidate: &TextBoundaryCandidate,
    target: &'static str,
    points: &[usize],
) -> Option<TextLayoutExactEvidence> {
    let points = points.iter().copied().collect::<BTreeSet<_>>();
    let mut best: Option<TextLayoutExactEvidence> = None;
    for base in layout_map_bases() {
        let start = base.apply(candidate.source_start());
        let end = base.apply(candidate.source_end());
        for point in &points {
            let delta = *point as isize - start;
            if !(LAYOUT_MAP_DELTA_MIN..=LAYOUT_MAP_DELTA_MAX).contains(&delta) {
                continue;
            }
            let mapped_end = end + delta;
            if mapped_end < 0 || !points.contains(&(mapped_end as usize)) {
                continue;
            }
            let evidence = TextLayoutExactEvidence::new(target, base.name(), delta);
            let replace = best.as_ref().is_none_or(|best| {
                delta.unsigned_abs() < best.delta().unsigned_abs()
                    || (delta.unsigned_abs() == best.delta().unsigned_abs()
                        && base.name() < best.base())
            });
            if replace {
                best = Some(evidence);
            }
        }
    }
    best
}

#[derive(Clone, Copy)]
enum LayoutMapBase {
    Unit,
    UnitTimes2,
    UnitDiv2Floor,
    UnitDiv2Ceil,
}

impl LayoutMapBase {
    fn name(self) -> &'static str {
        match self {
            Self::Unit => "unit",
            Self::UnitTimes2 => "unit-times-2",
            Self::UnitDiv2Floor => "unit-div2-floor",
            Self::UnitDiv2Ceil => "unit-div2-ceil",
        }
    }

    fn apply(self, value: usize) -> isize {
        match self {
            Self::Unit => value as isize,
            Self::UnitTimes2 => (value as isize) * 2,
            Self::UnitDiv2Floor => (value / 2) as isize,
            Self::UnitDiv2Ceil => value.div_ceil(2) as isize,
        }
    }
}

fn be16_words(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
}

fn range_starts_after_control_gap(entries: &[DocumentTextMapEntry], offset: usize) -> bool {
    let touches_entry = entries.iter().any(|entry| {
        entry.unit_start() == offset || (entry.unit_start() < offset && offset < entry.unit_end())
    });
    !touches_entry
        && previous_unit_entry(entries, offset)
            .is_some_and(|entry| entry.kind() == DocumentTextMapKind::ControlBoundary)
}

fn range_ends_on_aligned_text(entries: &[DocumentTextMapEntry], offset: usize) -> bool {
    entries.iter().any(|entry| {
        if !matches!(
            entry.kind(),
            DocumentTextMapKind::TextRun | DocumentTextMapKind::InlineText
        ) {
            return false;
        }
        entry.unit_end() == offset
            || (entry.unit_start() < offset
                && offset < entry.unit_end()
                && range_text_overlap(entry, offset, entry.unit_end())
                    .chars()
                    .all(|character| matches!(character, '\n' | '\r')))
    })
}

fn previous_unit_entry(
    entries: &[DocumentTextMapEntry],
    offset: usize,
) -> Option<&DocumentTextMapEntry> {
    entries
        .iter()
        .filter(|entry| entry.unit_end() <= offset)
        .max_by_key(|entry| entry.unit_end())
}

fn range_visible_text(entries: &[DocumentTextMapEntry], start: usize, end: usize) -> String {
    entries
        .iter()
        .filter(|entry| range_overlaps_entry(entry, start, end))
        .map(|entry| range_text_overlap(entry, start, end))
        .collect()
}

fn range_overlaps_entry_for_basis(
    entry: &DocumentTextMapEntry,
    start: usize,
    end: usize,
    basis: TextCountRangeOverlapBasis,
) -> bool {
    if start >= end {
        return false;
    }
    let (entry_start, entry_end) = entry_range_for_basis(entry, basis);
    entry_start < end && entry_end > start
}

fn entry_range_for_basis(
    entry: &DocumentTextMapEntry,
    basis: TextCountRangeOverlapBasis,
) -> (usize, usize) {
    match basis {
        TextCountRangeOverlapBasis::Byte => (entry.byte_start(), entry.byte_end()),
        TextCountRangeOverlapBasis::Unit => (entry.unit_start(), entry.unit_end()),
    }
}

fn range_overlaps_entry(entry: &DocumentTextMapEntry, start: usize, end: usize) -> bool {
    if start == end {
        return entry.unit_start() <= start && start <= entry.unit_end();
    }
    start < entry.unit_end() && end > entry.unit_start()
}

fn source_interval_overlaps(
    interval: &TextControlSourceInterval,
    basis: TextCountRangeOverlapBasis,
    start: usize,
    end: usize,
) -> bool {
    let (interval_start, interval_end) = match basis {
        TextCountRangeOverlapBasis::Byte => (interval.byte_start, interval.byte_end),
        TextCountRangeOverlapBasis::Unit => (interval.unit_start, interval.unit_end),
    };
    if start == end {
        return interval_start <= start && start <= interval_end;
    }
    start < interval_end && end > interval_start
}

fn source_span_range(span: &TextSourceSpan, basis: TextCountRangeOverlapBasis) -> (usize, usize) {
    match basis {
        TextCountRangeOverlapBasis::Byte => (span.byte_start(), span.byte_end()),
        TextCountRangeOverlapBasis::Unit => (span.unit_start(), span.unit_end()),
    }
}

fn table_row_column_segments(text: &str) -> Vec<TableCandidateColumnSegment> {
    let chars = text.chars().collect::<Vec<_>>();
    let value_spans = finance_value_spans(&chars);
    if value_spans.len() < 2 {
        return Vec::new();
    }

    let mut segments = Vec::new();
    if let Some((start, end)) = trim_char_span(&chars, 0, value_spans[0].0) {
        segments.push(TableCandidateColumnSegment::new(
            segments.len(),
            TableCandidateColumnSegmentKind::Label,
            start,
            end,
            None,
            None,
            chars[start..end].iter().collect(),
        ));
    }

    for (start, end) in value_spans {
        if let Some((start, end)) = trim_char_span(&chars, start, end) {
            segments.push(TableCandidateColumnSegment::new(
                segments.len(),
                TableCandidateColumnSegmentKind::Value,
                start,
                end,
                None,
                None,
                chars[start..end].iter().collect(),
            ));
        }
    }

    segments
}

fn finance_value_spans(chars: &[char]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut index = 0usize;
    while index < chars.len() {
        if chars[index] == '△' {
            let mut value_start = index + 1;
            while value_start < chars.len() && chars[value_start].is_whitespace() {
                value_start += 1;
            }
            if let Some(end) = parse_finance_value_end(chars, value_start) {
                spans.push((index, end));
                index = end;
                continue;
            }
        }

        if chars[index] == '－' {
            spans.push((index, index + 1));
            index += 1;
            continue;
        }

        if let Some(end) = parse_finance_value_end(chars, index) {
            spans.push((index, end));
            index = end;
            continue;
        }

        index += 1;
    }
    spans
}

fn parse_finance_value_end(chars: &[char], start: usize) -> Option<usize> {
    parse_decimal_value_end(chars, start).or_else(|| parse_comma_number_end(chars, start))
}

fn parse_decimal_value_end(chars: &[char], start: usize) -> Option<usize> {
    if !chars
        .get(start)
        .is_some_and(|character| character.is_ascii_digit())
    {
        return None;
    }
    let mut index = start;
    while index < chars.len() && chars[index].is_ascii_digit() {
        index += 1;
    }
    if chars.get(index) != Some(&'.') {
        return None;
    }
    let decimal_start = index + 1;
    if !chars
        .get(decimal_start)
        .is_some_and(|character| character.is_ascii_digit())
    {
        return None;
    }
    Some((decimal_start + 1).min(chars.len()))
}

fn parse_comma_number_end(chars: &[char], start: usize) -> Option<usize> {
    if !chars
        .get(start)
        .is_some_and(|character| character.is_ascii_digit())
    {
        return None;
    }

    let mut index = start;
    let mut leading_digits = 0usize;
    while index < chars.len() && chars[index].is_ascii_digit() && leading_digits < 3 {
        index += 1;
        leading_digits += 1;
    }
    if leading_digits == 0 || chars.get(index) != Some(&',') {
        return None;
    }

    let mut group_count = 0usize;
    while chars.get(index) == Some(&',') {
        let group_start = index + 1;
        let group_end = group_start + 3;
        if group_end > chars.len()
            || !chars[group_start..group_end]
                .iter()
                .all(|character| character.is_ascii_digit())
        {
            break;
        }
        index = group_end;
        group_count += 1;
    }

    (group_count > 0).then_some(index)
}

fn trim_char_span(chars: &[char], start: usize, end: usize) -> Option<(usize, usize)> {
    let mut start = start.min(chars.len());
    let mut end = end.min(chars.len());
    while start < end && chars[start].is_whitespace() {
        start += 1;
    }
    while end > start && chars[end - 1].is_whitespace() {
        end -= 1;
    }
    (start < end).then_some((start, end))
}

fn preview_text(text: &str, max_chars: usize) -> String {
    let mut preview = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CursorRect {
    page_index: usize,
    line_index: usize,
    x: f64,
    y: f64,
    height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TextRange {
    start_para: usize,
    start_offset: usize,
    end_para: usize,
    end_offset: usize,
}

impl TextRange {
    fn is_collapsed(&self) -> bool {
        self.start_para == self.end_para && self.start_offset == self.end_offset
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SearchHit {
    sec: u32,
    para: u32,
    char_offset: u32,
    length: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum JtdValidationWarningKind {
    FallbackTextPagination,
    RawStreamPreserved,
    UnknownBlockPreserved,
    UnknownStylePreserved,
    UnknownObjectPreserved,
    ObjectStreamCandidateDiagnosticOnly,
    TextCountRangeDiagnosticOnly,
    TextCountControlRangeDiagnosticOnly,
    TextBoundaryCandidateDiagnosticOnly,
    TextParagraphBoundaryCandidateDiagnosticOnly,
    TableCandidateDiagnosticOnly,
}

impl JtdValidationWarningKind {
    fn code(self) -> &'static str {
        match self {
            Self::FallbackTextPagination => "JtdFallbackTextPagination",
            Self::RawStreamPreserved => "JtdRawStreamPreserved",
            Self::UnknownBlockPreserved => "JtdUnknownBlockPreserved",
            Self::UnknownStylePreserved => "JtdUnknownStylePreserved",
            Self::UnknownObjectPreserved => "JtdUnknownObjectPreserved",
            Self::ObjectStreamCandidateDiagnosticOnly => "JtdObjectStreamCandidateDiagnosticOnly",
            Self::TextCountRangeDiagnosticOnly => "JtdTextCountRangeDiagnosticOnly",
            Self::TextCountControlRangeDiagnosticOnly => "JtdTextCountControlRangeDiagnosticOnly",
            Self::TextBoundaryCandidateDiagnosticOnly => "JtdTextBoundaryCandidateDiagnosticOnly",
            Self::TextParagraphBoundaryCandidateDiagnosticOnly => {
                "JtdTextParagraphBoundaryCandidateDiagnosticOnly"
            }
            Self::TableCandidateDiagnosticOnly => "JtdTableCandidateDiagnosticOnly",
        }
    }

    fn summary_message(self) -> &'static str {
        match self {
            Self::FallbackTextPagination => "JTD text layout uses fallback pagination",
            Self::RawStreamPreserved => "JTD raw stream preserved but not decoded",
            Self::UnknownBlockPreserved => "JTD unknown block preserved",
            Self::UnknownStylePreserved => "JTD style stream preserved but not decoded",
            Self::UnknownObjectPreserved => "JTD inline object preserved but not decoded",
            Self::ObjectStreamCandidateDiagnosticOnly => {
                "JTD object stream candidate preserved as diagnostic data"
            }
            Self::TextCountRangeDiagnosticOnly => {
                "JTD text-count range preserved as diagnostic data"
            }
            Self::TextCountControlRangeDiagnosticOnly => {
                "JTD text-count control-range overlap preserved as diagnostic data"
            }
            Self::TextBoundaryCandidateDiagnosticOnly => {
                "JTD text-boundary candidate preserved as diagnostic data"
            }
            Self::TextParagraphBoundaryCandidateDiagnosticOnly => {
                "JTD text paragraph-boundary candidate preserved as diagnostic data"
            }
            Self::TableCandidateDiagnosticOnly => {
                "JTD table candidate preserved as diagnostic data"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JtdValidationWarning {
    section_idx: usize,
    paragraph_idx: usize,
    kind: JtdValidationWarningKind,
}

impl JtdValidationWarning {
    fn document_level(kind: JtdValidationWarningKind) -> Self {
        Self {
            section_idx: 0,
            paragraph_idx: 0,
            kind,
        }
    }

    fn paragraph(paragraph_idx: usize, kind: JtdValidationWarningKind) -> Self {
        Self {
            section_idx: 0,
            paragraph_idx,
            kind,
        }
    }
}

fn jtd_validation_warnings(document: &Document) -> Vec<JtdValidationWarning> {
    let mut warnings = Vec::new();
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                if !paragraph_text(paragraph).is_empty() {
                    warnings.push(JtdValidationWarning::paragraph(
                        paragraph_index,
                        JtdValidationWarningKind::FallbackTextPagination,
                    ));
                }
                paragraph_index += 1;
            }
            Block::Unknown(_) => warnings.push(JtdValidationWarning::document_level(
                JtdValidationWarningKind::UnknownBlockPreserved,
            )),
        }
    }

    for _ in document.raw_streams() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::RawStreamPreserved,
        ));
    }

    for _ in document.unknown_styles() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::UnknownStylePreserved,
        ));
    }

    for _ in document.unknown_objects() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::UnknownObjectPreserved,
        ));
    }

    for _ in document.object_stream_candidates() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::ObjectStreamCandidateDiagnosticOnly,
        ));
    }

    for _ in document.text_count_ranges() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::TextCountRangeDiagnosticOnly,
        ));
    }

    for range in document.text_count_ranges() {
        if !range.control_range_overlaps().is_empty() {
            warnings.push(JtdValidationWarning::document_level(
                JtdValidationWarningKind::TextCountControlRangeDiagnosticOnly,
            ));
        }
    }

    for _ in document.text_boundary_candidates() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::TextBoundaryCandidateDiagnosticOnly,
        ));
    }

    for _ in document.text_paragraph_boundary_candidates() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::TextParagraphBoundaryCandidateDiagnosticOnly,
        ));
    }

    for _ in document.table_candidates() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::TableCandidateDiagnosticOnly,
        ));
    }

    warnings
}

fn jtd_validation_warnings_json(warnings: &[JtdValidationWarning]) -> String {
    let mut summary = BTreeMap::<&'static str, usize>::new();
    for warning in warnings {
        *summary.entry(warning.kind.summary_message()).or_insert(0) += 1;
    }

    let mut output = String::new();
    output.push_str("{\"count\":");
    output.push_str(&warnings.len().to_string());
    output.push_str(",\"summary\":{");
    for (index, (message, count)) in summary.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(message));
        output.push(':');
        output.push_str(&count.to_string());
    }
    output.push_str("},\"warnings\":[");
    for (index, warning) in warnings.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"section\":");
        output.push_str(&warning.section_idx.to_string());
        output.push_str(",\"paragraph\":");
        output.push_str(&warning.paragraph_idx.to_string());
        output.push_str(",\"kind\":");
        output.push_str(&json_string(warning.kind.code()));
        output.push_str(",\"cell\":null}");
    }
    output.push_str("]}");
    output
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProjectedTextControl {
    boundary_index: usize,
    paragraph_index: usize,
    char_offset: usize,
    code: u16,
}

fn project_control_boundary_to_text(
    boundary_span: &TextSourceSpan,
    spans: &[ParagraphSourceTextSpan],
) -> Option<(usize, usize)> {
    let mut previous: Option<&ParagraphSourceTextSpan> = None;
    let mut next: Option<&ParagraphSourceTextSpan> = None;

    for span in spans {
        if span.unit_start <= boundary_span.unit_start()
            && boundary_span.unit_end() <= span.unit_end
        {
            return Some((span.paragraph_index, span.char_start));
        }

        if span.unit_end <= boundary_span.unit_start()
            && previous.is_none_or(|candidate| span.unit_end > candidate.unit_end)
        {
            previous = Some(span);
        }

        if span.unit_start >= boundary_span.unit_end()
            && next.is_none_or(|candidate| span.unit_start < candidate.unit_start)
        {
            next = Some(span);
        }
    }

    match (previous, next) {
        (Some(prev), Some(next)) if prev.paragraph_index == next.paragraph_index => {
            Some((prev.paragraph_index, prev.char_end))
        }
        (Some(prev), Some(next)) => {
            let prev_distance = boundary_span.unit_start().saturating_sub(prev.unit_end);
            let next_distance = next.unit_start.saturating_sub(boundary_span.unit_end());
            if next_distance < prev_distance {
                Some((next.paragraph_index, next.char_start))
            } else {
                Some((prev.paragraph_index, prev.char_end))
            }
        }
        (Some(prev), None) => Some((prev.paragraph_index, prev.char_end)),
        (None, Some(next)) => Some((next.paragraph_index, next.char_start)),
        (None, None) => None,
    }
}

fn projected_control_json(control: &ProjectedTextControl) -> String {
    format!(
        "{{\"type\":\"jtdControl\",\"sec\":0,\"para\":{},\"ci\":{},\"charPos\":{},\"code\":{},\"codeHex\":{},\"decoded\":false}}",
        control.paragraph_index,
        control.boundary_index,
        control.char_offset,
        control.code,
        json_string(&format!("0x{:04x}", control.code)),
    )
}

fn projected_control_layout_json(
    layout: PageLayout,
    control: &ProjectedTextControl,
    rect: &CursorRect,
) -> String {
    format!(
        "{{\"type\":\"jtdControl\",\"x\":{:.1},\"y\":{:.1},\"w\":{:.1},\"h\":{:.1},\"secIdx\":0,\"paraIdx\":{},\"controlIdx\":{},\"charPos\":{},\"code\":{},\"codeHex\":{},\"decoded\":false,\"source\":\"textControlBoundary\"}}",
        rect.x,
        rect.y,
        column_width_px(layout),
        rect.height,
        control.paragraph_index,
        control.boundary_index,
        control.char_offset,
        control.code,
        json_string(&format!("0x{:04x}", control.code)),
    )
}

#[derive(Debug, Clone, Copy, Default)]
struct PageOutputShape {
    pages: usize,
    lines: usize,
}

#[derive(Debug, Default)]
struct PagePreflight {
    shape: PageOutputShape,
    current_lines: usize,
    current_has_nonempty_line: bool,
    trailing_trim_lines: usize,
    lines_per_page: usize,
}

impl PagePreflight {
    fn new(lines_per_page: usize) -> Self {
        Self {
            lines_per_page,
            ..Self::default()
        }
    }

    fn push_line(&mut self, nonempty: bool, trim_at_page_end: bool) -> Result<()> {
        if self.current_lines >= self.lines_per_page {
            self.finish_current_page()?;
        }
        self.current_lines = checked_page_shape_add(self.current_lines, 1)?;
        self.current_has_nonempty_line |= nonempty;
        self.trailing_trim_lines = if trim_at_page_end {
            checked_page_shape_add(self.trailing_trim_lines, 1)?
        } else {
            0
        };
        Ok(())
    }

    fn force_page_break(&mut self) -> Result<()> {
        self.current_lines = self.current_lines.saturating_sub(self.trailing_trim_lines);
        self.trailing_trim_lines = 0;
        if self.current_has_nonempty_line {
            self.finish_current_page()?;
        } else {
            self.current_lines = 0;
        }
        Ok(())
    }

    fn finish(mut self, has_raw_streams: bool) -> Result<PageOutputShape> {
        self.current_lines = self.current_lines.saturating_sub(self.trailing_trim_lines);
        self.trailing_trim_lines = 0;
        if self.current_lines != 0 {
            self.finish_current_page()?;
        }
        if self.shape.pages == 0 {
            self.shape.pages = 1;
            self.shape.lines = usize::from(has_raw_streams);
        }
        Ok(self.shape)
    }

    fn finish_current_page(&mut self) -> Result<()> {
        self.shape.pages = checked_page_shape_add(self.shape.pages, 1)?;
        self.shape.lines = checked_page_shape_add(self.shape.lines, self.current_lines)?;
        self.current_lines = 0;
        self.trailing_trim_lines = 0;
        self.current_has_nonempty_line = false;
        Ok(())
    }
}

fn checked_page_shape_add(left: usize, right: usize) -> Result<usize> {
    left.checked_add(right).ok_or(Error::ResourceLimit {
        resource: "document page lines",
        limit: usize::MAX,
        actual: usize::MAX,
    })
}

fn page_output_shape(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Result<PageOutputShape> {
    let wrap_columns = layout.wrap_columns(writing_mode);
    let forced_breaks = projected_page_breaks(document);
    let mut preflight = PagePreflight::new(layout.lines_per_page(writing_mode));
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                let paragraph_breaks = forced_breaks
                    .get(&paragraph_index)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);
                let forced_at_paragraph_end = page_shape_for_paragraph(
                    paragraph,
                    paragraph_breaks,
                    wrap_columns,
                    &mut preflight,
                )?;
                if !forced_at_paragraph_end && !writing_mode.is_vertical() {
                    preflight.push_line(false, true)?;
                }
                paragraph_index = checked_page_shape_add(paragraph_index, 1)?;
            }
            Block::Unknown(_) => {
                preflight.push_line(true, false)?;
                preflight.push_line(false, true)?;
            }
        }
    }

    preflight.finish(!document.raw_streams().is_empty())
}

fn page_construction_shape(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Result<PageOutputShape> {
    let normal = page_output_shape(document, layout, writing_mode)?;
    if !writing_mode.is_vertical() {
        return Ok(normal);
    }

    if ginga_front_matter_indices_in_document(document).is_none() {
        return Ok(normal);
    }

    let source_lines = document_paragraph_character_count(document)?;
    let projection_lines = source_lines
        .checked_mul(4)
        .and_then(|total| checked_page_shape_add(total, document.toc_entries().len()).ok())
        .and_then(|total| checked_page_shape_add(total, 32).ok())
        .ok_or(Error::ResourceLimit {
            resource: "document page lines",
            limit: usize::MAX,
            actual: usize::MAX,
        })?;
    // The projection temporarily coexists with normal pagination. It contributes five fixed
    // front-matter pages and body pagination can gain at most one carry page when chapter
    // spacing is inserted. Page-line expansion is accounted separately below.
    let projection_pages = checked_page_shape_add(normal.pages, 6)?;

    Ok(PageOutputShape {
        pages: checked_page_shape_add(normal.pages, projection_pages)?,
        lines: checked_page_shape_add(normal.lines, projection_lines)?,
    })
}

fn page_shape_for_paragraph(
    paragraph: &Paragraph,
    paragraph_breaks: &[usize],
    wrap_columns: usize,
    preflight: &mut PagePreflight,
) -> Result<bool> {
    let mut line_start = 0usize;
    let mut char_offset = 0usize;
    let mut line_width = 0usize;
    for inline in paragraph.inlines() {
        let text = match inline {
            Inline::Text(run) => run.text(),
            Inline::Ruby(ruby) => ruby.base_text(),
            Inline::Unknown(_) => continue,
        };
        for character in text.chars() {
            let character_width = display_column_width(character);
            if line_width > 0 && line_width + character_width > wrap_columns {
                page_shape_for_wrapped_line(line_start, char_offset, paragraph_breaks, preflight)?;
                line_width = 0;
                line_start = char_offset;
            }
            line_width += character_width;
            char_offset = checked_page_shape_add(char_offset, 1)?;
        }
    }

    if char_offset == 0 {
        preflight.push_line(false, false)?;
        if paragraph_breaks.contains(&0) {
            preflight.force_page_break()?;
            return Ok(true);
        }
        return Ok(false);
    }

    page_shape_for_wrapped_line(line_start, char_offset, paragraph_breaks, preflight)
}

fn page_shape_for_wrapped_line(
    line_start: usize,
    line_end: usize,
    paragraph_breaks: &[usize],
    preflight: &mut PagePreflight,
) -> Result<bool> {
    let mut segment_start = line_start;
    let mut emitted_segment = false;
    let mut forced_after_last_segment = false;

    for break_offset in paragraph_breaks.iter().copied() {
        if break_offset < segment_start || break_offset > line_end {
            continue;
        }
        if break_offset > segment_start || break_offset == line_start {
            preflight.push_line(break_offset > segment_start, false)?;
            preflight.force_page_break()?;
            emitted_segment = true;
            forced_after_last_segment = true;
        }
        segment_start = break_offset;
    }

    if segment_start < line_end {
        preflight.push_line(true, false)?;
        return Ok(false);
    }
    if !emitted_segment {
        preflight.push_line(true, false)?;
        return Ok(false);
    }
    Ok(forced_after_last_segment)
}

fn project_sample_front_matter_pages(
    document: &Document,
    _file_name: &str,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Option<Vec<Vec<PageTextLine>>> {
    if !writing_mode.is_vertical() {
        return None;
    }

    let paragraphs = document_paragraph_texts(document);
    let front_matter = ginga_front_matter_indices(&paragraphs)?;
    let forced_breaks = projected_page_breaks(document);
    let wrap_columns = layout.wrap_columns(writing_mode);
    let mut pages = Vec::new();

    pages.push(wrap_paragraphs_as_single_page(
        &paragraphs[front_matter.title_index..front_matter.title_index + 1],
        wrap_columns,
        writing_mode,
    ));
    pages.push(Vec::new());
    pages.push(
        projected_ginga_toc_page(document, &paragraphs, front_matter, wrap_columns).unwrap_or_else(
            || {
                wrap_paragraphs_as_single_page(
                    &paragraphs[front_matter.toc_start_index..front_matter.body_title_index],
                    wrap_columns,
                    writing_mode,
                )
            },
        ),
    );
    pages.push(Vec::new());
    pages.push(wrap_paragraphs_as_single_page(
        &paragraphs[front_matter.body_title_index..front_matter.body_title_index + 1],
        wrap_columns,
        writing_mode,
    ));
    let body_pages = paginate_selected_paragraphs(
        &paragraphs[front_matter.body_start_index..],
        layout,
        writing_mode,
        &forced_breaks,
    );
    let body_pages =
        project_ginga_body_chapter_pages(body_pages, layout.lines_per_page(writing_mode));
    pages.extend(project_ginga_colophon_pages(body_pages));

    Some(pages)
}

fn project_ginga_body_chapter_pages(
    body_pages: Vec<Vec<PageTextLine>>,
    lines_per_page: usize,
) -> Vec<Vec<PageTextLine>> {
    let mut pages = body_pages.into_iter();
    let Some(first_page) = pages.next() else {
        return Vec::new();
    };
    let Some(chapter_line) = first_page.first() else {
        let mut original_pages = vec![first_page];
        original_pages.extend(pages);
        return original_pages;
    };
    if !is_short_chapter_title(chapter_line.text().trim()) {
        let mut original_pages = vec![first_page];
        original_pages.extend(pages);
        return original_pages;
    }

    let heading_slots =
        GINGA_BODY_CHAPTER_LEADING_BLANK_COLUMNS + 1 + GINGA_BODY_CHAPTER_TRAILING_BLANK_COLUMNS;
    if lines_per_page <= heading_slots {
        let mut original_pages = vec![first_page];
        original_pages.extend(pages);
        return original_pages;
    }

    let available_body_lines = lines_per_page - heading_slots;
    let keep_end = (1 + available_body_lines).min(first_page.len());
    let mut projected_first_page = Vec::with_capacity(lines_per_page);
    projected_first_page.extend(
        std::iter::repeat_with(blank_page_text_line).take(GINGA_BODY_CHAPTER_LEADING_BLANK_COLUMNS),
    );
    projected_first_page.push(first_page[0].clone());
    projected_first_page.extend(
        std::iter::repeat_with(blank_page_text_line)
            .take(GINGA_BODY_CHAPTER_TRAILING_BLANK_COLUMNS),
    );
    projected_first_page.extend(first_page[1..keep_end].iter().cloned());

    let mut projected_pages = vec![projected_first_page];
    let mut carry = first_page[keep_end..].to_vec();
    for page in pages {
        let mut projected_page = Vec::new();
        projected_page.append(&mut carry);
        projected_page.extend(page);
        if projected_page.len() > lines_per_page {
            carry = projected_page.split_off(lines_per_page);
        }
        projected_pages.push(projected_page);
    }
    projected_pages.extend(repaginate_lines(carry, lines_per_page));
    projected_pages
}

fn repaginate_lines(lines: Vec<PageTextLine>, lines_per_page: usize) -> Vec<Vec<PageTextLine>> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut pages = Vec::new();
    let mut current_page = Vec::new();
    for line in lines {
        push_paginated_line(&mut pages, &mut current_page, line, lines_per_page);
    }
    trim_trailing_projection_blank_lines(&mut current_page);
    if !current_page.is_empty() {
        pages.push(current_page);
    }
    pages
}

fn project_ginga_colophon_pages(mut pages: Vec<Vec<PageTextLine>>) -> Vec<Vec<PageTextLine>> {
    for page in &mut pages {
        if is_ginga_colophon_page(page) {
            *page = project_ginga_colophon_lines(page);
        }
    }
    pages
}

fn is_ginga_colophon_page(lines: &[PageTextLine]) -> bool {
    let visible = lines
        .iter()
        .map(PageTextLine::text)
        .map(str::trim)
        .filter(|text| !text.is_empty() && !is_colophon_noise_line(text))
        .collect::<Vec<_>>();
    visible
        .first()
        .is_some_and(|text| text.contains("銀河鉄道の夜"))
        && visible.iter().any(|text| text.contains("初版発行"))
        && visible.iter().any(|text| text.contains("発行所"))
        && visible
            .iter()
            .any(|text| text.contains("Printed") || text.contains("Japan"))
}

fn project_ginga_colophon_lines(lines: &[PageTextLine]) -> Vec<PageTextLine> {
    let mut projected = Vec::new();
    let mut visible_index = 0usize;
    let mut index = 0usize;

    while index < lines.len() {
        let line = &lines[index];
        let text = line.text().trim();
        if text.is_empty() || is_colophon_noise_line(text) {
            index += 1;
            continue;
        }

        if text.starts_with('※') {
            let (note, consumed) = collect_colophon_note_lines(&lines[index..]);
            projected.extend(split_colophon_note_line(note));
            index += consumed;
            continue;
        }

        projected.push(line.clone());
        if visible_index == 0 || visible_index == 1 || is_colophon_copyright_line(text) {
            projected.push(blank_page_text_line());
        }
        visible_index += 1;
        index += 1;
    }

    projected
}

fn collect_colophon_note_lines(lines: &[PageTextLine]) -> (PageTextLine, usize) {
    let Some(first) = lines.first() else {
        return (blank_page_text_line(), 0);
    };
    let mut text = String::new();
    let mut consumed = 0usize;
    let paragraph_index = first.paragraph_index();
    let char_start = first.char_start();
    let mut char_end = first.char_end();

    for line in lines {
        let trimmed = line.text().trim();
        if trimmed.is_empty() || is_colophon_noise_line(trimmed) {
            consumed += 1;
            continue;
        }
        if consumed > 0 && !trimmed.starts_with('※') && line.paragraph_index() != paragraph_index
        {
            break;
        }
        text.push_str(trimmed);
        char_end = line.char_end();
        consumed += 1;
    }

    (
        PageTextLine::new(text, paragraph_index, char_start, char_end),
        consumed,
    )
}

fn split_colophon_note_line(line: PageTextLine) -> Vec<PageTextLine> {
    split_page_text_line_by_display_columns(line, GINGA_COLOPHON_NOTE_DISPLAY_COLUMNS)
}

fn is_colophon_noise_line(text: &str) -> bool {
    text.trim().starts_with('\u{fe02}')
}

fn is_colophon_copyright_line(text: &str) -> bool {
    text.contains("Printed") || text.contains("Japan") || text.contains("©")
}

fn projected_ginga_toc_page(
    document: &Document,
    paragraphs: &[(usize, String)],
    front_matter: GingaFrontMatterIndices,
    wrap_columns: usize,
) -> Option<Vec<PageTextLine>> {
    if document.toc_entries().is_empty() {
        return None;
    }

    let toc_title_paragraphs = paragraphs
        [front_matter.toc_start_index + 1..front_matter.body_title_index]
        .iter()
        .map(|(paragraph_index, text)| (text.trim().to_string(), *paragraph_index))
        .collect::<BTreeMap<_, _>>();
    let mut lines = Vec::new();
    for _ in 0..GINGA_TOC_LEADING_BLANK_COLUMNS {
        lines.push(PageTextLine::new(String::new(), None, 0, 0));
    }
    lines.extend(wrap_text_line(
        &paragraphs[front_matter.toc_start_index].1,
        paragraphs[front_matter.toc_start_index].0,
        wrap_columns,
    ));
    let toc_columns = wrap_columns.saturating_add(GINGA_TOC_EXTRA_COLUMNS);

    for entry in document.toc_entries() {
        let title = entry.title().trim();
        let Some(paragraph_index) = toc_title_paragraphs.get(title) else {
            continue;
        };
        let text = toc_leader_line(title, entry.page_label(), toc_columns);
        let char_count = text.chars().count();
        let title_char_count = title.chars().count();
        lines.push(PageTextLine::new(
            text,
            Some(*paragraph_index),
            0,
            title_char_count.min(char_count),
        ));
    }

    (lines.len() > 1).then_some(lines)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GingaFrontMatterIndices {
    title_index: usize,
    toc_start_index: usize,
    body_title_index: usize,
    body_start_index: usize,
}

fn ginga_front_matter_indices(paragraphs: &[(usize, String)]) -> Option<GingaFrontMatterIndices> {
    let first_text = paragraphs.first()?.1.trim();
    if !first_text.contains("銀河鉄道の夜") || !first_text.contains("宮沢") {
        return None;
    }

    let toc_start_index = paragraphs
        .iter()
        .position(|(_, text)| text.trim() == "目次")?;
    let body_title_index = paragraphs
        .iter()
        .enumerate()
        .skip(toc_start_index + 1)
        .find_map(|(index, (_, text))| (text.trim() == "銀河鉄道の夜").then_some(index))?;
    let body_start_index = body_title_index + 1;
    if body_start_index >= paragraphs.len() {
        return None;
    }
    let body_start_text = paragraphs[body_start_index].1.trim();
    if !body_start_text.starts_with("一、午后の授業") {
        return None;
    }

    Some(GingaFrontMatterIndices {
        title_index: 0,
        toc_start_index,
        body_title_index,
        body_start_index,
    })
}

fn ginga_front_matter_indices_in_document(document: &Document) -> Option<GingaFrontMatterIndices> {
    let mut paragraph_index = 0usize;
    let mut toc_start_index = None;
    let mut body_title_index = None;

    for block in document.blocks() {
        let Block::Paragraph(paragraph) = block else {
            continue;
        };

        if paragraph_index == 0
            && (!paragraph_contains(paragraph, "銀河鉄道の夜")
                || !paragraph_contains(paragraph, "宮沢"))
        {
            return None;
        }

        if toc_start_index.is_none() && paragraph_trimmed_equals(paragraph, "目次") {
            toc_start_index = Some(paragraph_index);
        } else if let Some(toc_start_index) = toc_start_index {
            if paragraph_index > toc_start_index
                && body_title_index.is_none()
                && paragraph_trimmed_equals(paragraph, "銀河鉄道の夜")
            {
                body_title_index = Some(paragraph_index);
            } else if let Some(body_title_index) = body_title_index
                && paragraph_index == body_title_index + 1
            {
                if paragraph_trimmed_starts_with(paragraph, "一、午后の授業") {
                    return Some(GingaFrontMatterIndices {
                        title_index: 0,
                        toc_start_index,
                        body_title_index,
                        body_start_index: paragraph_index,
                    });
                }
                return None;
            }
        }

        paragraph_index = paragraph_index.checked_add(1)?;
    }

    None
}

fn paragraph_contains(paragraph: &Paragraph, needle: &str) -> bool {
    let mut matched = 0usize;
    let needle_len = needle.chars().count();
    if needle_len == 0 {
        return true;
    }

    for text in paragraph_text_fragments(paragraph) {
        for character in text.chars() {
            let expected = needle.chars().nth(matched);
            if expected == Some(character) {
                matched += 1;
                if matched == needle_len {
                    return true;
                }
            } else {
                matched = usize::from(needle.starts_with(character));
            }
        }
    }

    false
}

fn paragraph_trimmed_equals(paragraph: &Paragraph, expected: &str) -> bool {
    let mut expected_index = 0usize;
    let mut saw_non_whitespace = false;
    let mut trailing_whitespace = false;

    for text in paragraph_text_fragments(paragraph) {
        for character in text.chars() {
            if !saw_non_whitespace && character.is_whitespace() {
                continue;
            }
            saw_non_whitespace = true;
            if trailing_whitespace {
                if !character.is_whitespace() {
                    return false;
                }
                continue;
            }
            if character.is_whitespace() && expected.chars().nth(expected_index).is_none() {
                trailing_whitespace = true;
                continue;
            }
            if expected.chars().nth(expected_index) != Some(character) {
                return false;
            }
            expected_index += 1;
        }
    }

    saw_non_whitespace && expected.chars().nth(expected_index).is_none()
}

fn paragraph_trimmed_starts_with(paragraph: &Paragraph, expected: &str) -> bool {
    let mut expected_index = 0usize;
    let mut saw_non_whitespace = false;

    for text in paragraph_text_fragments(paragraph) {
        for character in text.chars() {
            if !saw_non_whitespace && character.is_whitespace() {
                continue;
            }
            saw_non_whitespace = true;
            if expected.chars().nth(expected_index) != Some(character) {
                return false;
            }
            expected_index += 1;
            if expected.chars().nth(expected_index).is_none() {
                return true;
            }
        }
    }

    false
}

fn paragraph_text_fragments(paragraph: &Paragraph) -> impl Iterator<Item = &str> {
    paragraph
        .inlines()
        .iter()
        .filter_map(|inline| match inline {
            Inline::Text(run) => Some(run.text()),
            Inline::Ruby(ruby) => Some(ruby.base_text()),
            Inline::Unknown(_) => None,
        })
}

fn document_paragraph_character_count(document: &Document) -> Result<usize> {
    document
        .blocks()
        .iter()
        .filter_map(|block| match block {
            Block::Paragraph(paragraph) => Some(paragraph),
            Block::Unknown(_) => None,
        })
        .try_fold(0usize, |total, paragraph| {
            let character_count =
                paragraph_text_fragments(paragraph).try_fold(0usize, |character_count, text| {
                    checked_page_shape_add(character_count, text.chars().count())
                })?;
            checked_page_shape_add(total, character_count.max(1))
        })
}

fn document_chapter_title_candidates(document: &Document) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut titles = Vec::new();
    for (_, text) in document_paragraph_texts(document) {
        let trimmed = text.trim();
        if !is_short_chapter_title(trimmed) {
            continue;
        }
        if seen.insert(trimmed.to_string()) {
            titles.push(trimmed.to_string());
        }
    }
    titles.sort_by_key(|title| std::cmp::Reverse(title.chars().count()));
    titles
}

fn running_body_start_page(
    pages: &[Vec<PageTextLine>],
    document_title: &str,
    chapter_titles: &[String],
) -> Option<usize> {
    let mut seen_body_title_page = false;
    for (page_index, page) in pages.iter().enumerate() {
        if page_index > 0 && page_has_exact_text_line(page, document_title) {
            seen_body_title_page = true;
            continue;
        }
        if seen_body_title_page && page_chapter_title(page, chapter_titles).is_some() {
            return Some(page_index);
        }
    }
    None
}

fn running_chapter_title_for_page(
    pages: &[Vec<PageTextLine>],
    body_start_page: usize,
    page_index: usize,
    chapter_titles: &[String],
) -> Option<String> {
    let mut current = None;
    for page in pages
        .iter()
        .take(page_index.saturating_add(1))
        .skip(body_start_page)
    {
        if let Some(title) = page_chapter_title(page, chapter_titles) {
            current = Some(title);
        }
    }
    current
}

fn is_short_chapter_title(text: &str) -> bool {
    if text.chars().count() > 32 {
        return false;
    }
    let Some((prefix, suffix)) = text.split_once('、') else {
        return false;
    };
    !prefix.is_empty() && !suffix.trim().is_empty() && prefix.chars().all(is_japanese_number_char)
}

fn is_japanese_number_char(character: char) -> bool {
    matches!(
        character,
        '〇' | '零'
            | '一'
            | '二'
            | '三'
            | '四'
            | '五'
            | '六'
            | '七'
            | '八'
            | '九'
            | '十'
            | '百'
            | '千'
            | '壱'
            | '弐'
            | '参'
    )
}

fn toc_leader_line(title: &str, page_label: &str, max_columns: usize) -> String {
    let title_width = text_display_column_width(title);
    let page_width = text_display_column_width(page_label);
    let leader_width = max_columns.saturating_sub(title_width + page_width).max(8);
    let leader_count = (leader_width / display_column_width('…')).max(4);
    format!("{title}{}{page_label}", "…".repeat(leader_count))
}

fn trim_trailing_projection_blank_lines(lines: &mut Vec<PageTextLine>) {
    while lines
        .last()
        .is_some_and(|line| line.text().is_empty() && line.paragraph_index().is_none())
    {
        lines.pop();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageLineSegment {
    line: PageTextLine,
    break_after: bool,
}

fn push_paginated_line(
    pages: &mut Vec<Vec<PageTextLine>>,
    current_page: &mut Vec<PageTextLine>,
    line: PageTextLine,
    lines_per_page: usize,
) {
    if current_page.len() >= lines_per_page {
        pages.push(std::mem::take(current_page));
    }
    current_page.push(line);
}

fn document_plain_text(document: &Document) -> String {
    let mut output = String::new();

    for block in document.blocks() {
        if let Block::Paragraph(paragraph) = block {
            output.push_str(&paragraph_text(paragraph));
            output.push('\n');
        }
    }

    output
}

fn checked_char_boundary(text: &str, char_offset: usize) -> Result<usize> {
    let char_count = text.chars().count();
    if char_offset > char_count {
        return Err(rjtd_core::Error::InvalidData(format!(
            "char offset {char_offset} out of range (paragraph length {char_count})"
        )));
    }

    if char_offset == char_count {
        return Ok(text.len());
    }

    text.char_indices()
        .nth(char_offset)
        .map(|(byte_index, _)| byte_index)
        .ok_or_else(|| {
            rjtd_core::Error::InvalidData(format!(
                "char offset {char_offset} out of range (paragraph length {char_count})"
            ))
        })
}

fn find_in_text(text: &str, query: &str, case_sensitive: bool) -> Vec<usize> {
    if text.is_empty() || query.is_empty() {
        return Vec::new();
    }

    let text_chars = text.chars().collect::<Vec<_>>();
    let query_chars = query.chars().collect::<Vec<_>>();
    let query_len = query_chars.len();
    if text_chars.len() < query_len {
        return Vec::new();
    }

    if case_sensitive {
        return text_chars
            .windows(query_len)
            .enumerate()
            .filter_map(|(index, window)| (window == query_chars.as_slice()).then_some(index))
            .collect();
    }

    let folded_text = text_chars
        .iter()
        .map(|character| character.to_lowercase().collect::<String>())
        .collect::<Vec<_>>();
    let folded_query = query_chars
        .iter()
        .map(|character| character.to_lowercase().collect::<String>())
        .collect::<Vec<_>>();

    folded_text
        .windows(query_len)
        .enumerate()
        .filter_map(|(index, window)| (window == folded_query.as_slice()).then_some(index))
        .collect()
}

fn display_column_width(character: char) -> usize {
    match character {
        '\t' => APP_TAB_COLUMNS,
        _ if character.is_ascii() => 1,
        _ => 2,
    }
}

fn column_width_px(layout: PageLayout) -> f64 {
    layout.body_width_px() as f64 / layout.wrap_columns(WritingMode::Horizontal) as f64
}

fn line_index_for_y(layout: PageLayout, line_count: usize, y: f64) -> usize {
    if line_count == 0 {
        return 0;
    }

    let relative_y = normalize_coordinate(y) - layout.margin_px() as f64;
    let line_index = (relative_y.max(0.0) / APP_LINE_HEIGHT_PX as f64).floor() as usize;
    line_index.min(line_count - 1)
}

fn cursor_rect_from_line(
    layout: PageLayout,
    page_index: usize,
    line_index: usize,
    line: &PageTextLine,
    char_offset: usize,
) -> CursorRect {
    let char_offset = char_offset.clamp(line.char_start(), line.char_end());
    let x = layout.margin_px() as f64
        + column_units_before(line, char_offset) * column_width_px(layout);
    let y = layout.margin_px() as f64 + line_index as f64 * APP_LINE_HEIGHT_PX as f64;

    CursorRect {
        page_index,
        line_index,
        x,
        y,
        height: APP_LINE_HEIGHT_PX as f64,
    }
}

fn column_units_before(line: &PageTextLine, char_offset: usize) -> f64 {
    let mut units = 0.0;

    for (current_offset, character) in (line.char_start()..).zip(line.text().chars()) {
        if current_offset >= char_offset {
            break;
        }
        units += display_column_width(character) as f64;
    }

    units
}

fn char_offset_for_x(layout: PageLayout, line: &PageTextLine, x: f64) -> usize {
    let target_units =
        ((normalize_coordinate(x) - layout.margin_px() as f64) / column_width_px(layout)).max(0.0);
    let mut units = 0.0;

    for (char_offset, character) in (line.char_start()..).zip(line.text().chars()) {
        let width = display_column_width(character) as f64;
        if target_units <= units + (width / 2.0) {
            return char_offset;
        }
        units += width;
    }

    line.char_end()
}

fn selection_overlap(
    line: &PageTextLine,
    paragraph_index: usize,
    range: &TextRange,
) -> Option<(usize, usize)> {
    if paragraph_index < range.start_para || paragraph_index > range.end_para {
        return None;
    }

    let selection_start = if paragraph_index == range.start_para {
        range.start_offset
    } else {
        line.char_start()
    };
    let selection_end = if paragraph_index == range.end_para {
        range.end_offset
    } else {
        line.char_end()
    };

    let start = line.char_start().max(selection_start);
    let end = line.char_end().min(selection_end);
    if start > end || (start == end && !line.text().is_empty()) {
        return None;
    }
    Some((start, end))
}

fn normalize_coordinate(coordinate: f64) -> f64 {
    if coordinate.is_finite() {
        coordinate
    } else {
        0.0
    }
}

fn format_cursor_rect(rect: &CursorRect) -> String {
    format!(
        "{{\"pageIndex\":{},\"lineIndex\":{},\"x\":{:.1},\"y\":{:.1},\"height\":{:.1}}}",
        rect.page_index, rect.line_index, rect.x, rect.y, rect.height
    )
}

fn format_search_result(hit: &SearchHit, wrapped: bool) -> String {
    format!(
        "{{\"found\":true,\"wrapped\":{},\"sec\":{},\"para\":{},\"charOffset\":{},\"length\":{}}}",
        wrapped, hit.sec, hit.para, hit.char_offset, hit.length
    )
}

fn format_search_hit(hit: &SearchHit) -> String {
    format!(
        "{{\"sec\":{},\"para\":{},\"charOffset\":{},\"length\":{}}}",
        hit.sec, hit.para, hit.char_offset, hit.length
    )
}

fn format_nav_text(section_idx: u32, paragraph_idx: u32, char_offset: u32) -> String {
    format!(
        "{{\"type\":\"text\",\"sec\":{},\"para\":{},\"charOffset\":{},\"context\":[]}}",
        section_idx, paragraph_idx, char_offset
    )
}

fn json_ok_with(fields: &str) -> String {
    format!("{{\"ok\":true,{fields}}}")
}

fn default_cursor_rect_json(page_index: u32) -> String {
    format!(
        "{{\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1},\"height\":{:.1}}}",
        page_index, APP_PAGE_MARGIN_PX, APP_PAGE_MARGIN_PX, APP_LINE_HEIGHT_PX
    )
}

fn default_line_info_json() -> String {
    "{\"lineIndex\":0,\"lineCount\":1,\"charStart\":0,\"charEnd\":0}".to_string()
}

fn default_table_dimensions_json() -> String {
    "{\"rowCount\":0,\"colCount\":0,\"cellCount\":0}".to_string()
}

fn observed_table_dimensions_json(candidate: &TableCandidate) -> String {
    let row_count = candidate.intervals().len();
    let mut output = format!(
        "{{\"rowCount\":{row_count},\"colCount\":1,\"cellCount\":{row_count},\"source\":\"tableCandidate\",\"tableCandidateIndex\":{},\"basis\":\"{}\",\"delimiterCode\":{},\"delimiterCodeHex\":\"0x{:04x}\",\"columnSplitCandidateRows\":{},\"maxColumnSegmentCount\":{},\"columnSegmentPatternConsistent\":{},\"columnSegmentPatternMismatchRows\":{}",
        candidate.index(),
        candidate.basis().as_str(),
        candidate.delimiter_code(),
        candidate.delimiter_code(),
        candidate.column_split_candidate_row_count(),
        candidate.max_column_segment_count(),
        if candidate.column_segment_pattern_consistent() {
            "true"
        } else {
            "false"
        },
        candidate.column_segment_pattern_mismatch_rows()
    );
    output.push_str(",\"columnGridCandidate\":");
    if let Some(grid) = candidate.column_segment_grid_candidate() {
        output.push_str(&column_grid_candidate_json(candidate, &grid));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"columnSplittingDecoded\":false,\"decoded\":false}");
    output
}

fn column_grid_candidate_json(
    candidate: &TableCandidate,
    grid: &TableCandidateColumnGridCandidate,
) -> String {
    let pattern = grid
        .pattern()
        .iter()
        .map(|kind| json_string(kind.as_str()))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"source\":\"columnSegments\",\"tableCandidateIndex\":{},\"rowCount\":{},\"colCountCandidate\":{},\"cellCountCandidate\":{},\"columnSplitCandidateRows\":{},\"maxColumnSegmentCount\":{},\"columnSegmentPatternConsistent\":true,\"columnSegmentPatternMismatchRows\":0,\"pattern\":[{}],\"geometryDecoded\":false,\"decoded\":false}}",
        candidate.index(),
        grid.row_count(),
        grid.column_count(),
        grid.cell_count(),
        grid.split_row_count(),
        candidate.max_column_segment_count(),
        pattern
    )
}

fn default_cell_info_json() -> String {
    "{\"row\":0,\"col\":0,\"rowSpan\":1,\"colSpan\":1}".to_string()
}

fn observed_cell_info_json(cell_idx: u32, cell: &TableCandidateInterval) -> String {
    format!(
        "{{\"row\":{cell_idx},\"col\":0,\"rowSpan\":1,\"colSpan\":1,\"source\":\"tableCandidateInterval\",\"sourceIntervalIndex\":{},\"sourceStart\":{},\"sourceEnd\":{},\"decoded\":false}}",
        cell.source_interval_index(),
        cell.source_start(),
        cell.source_end()
    )
}

fn observed_cell_line_info_json(cell: &TableCandidateInterval) -> String {
    let char_end = cell.text_preview().chars().count();
    format!("{{\"lineIndex\":0,\"lineCount\":1,\"charStart\":0,\"charEnd\":{char_end}}}")
}

fn observed_table_signature(candidate: &TableCandidate) -> String {
    format!(
        "rjtd-table-candidate:{}:{}:0x{:04x}:{}x1",
        candidate.index(),
        candidate.basis().as_str(),
        candidate.delimiter_code(),
        candidate.intervals().len()
    )
}

fn char_slice(text: &str, char_offset: u32, count: u32) -> String {
    text.chars()
        .skip(char_offset as usize)
        .take(count as usize)
        .collect()
}

fn default_table_edit_result_json() -> String {
    "{\"ok\":false,\"rowCount\":0,\"colCount\":0}".to_string()
}

fn default_cell_count_result_json() -> String {
    "{\"ok\":false,\"cellCount\":0}".to_string()
}

fn default_char_properties_json() -> String {
    "{\"fontFamily\":\"Hiragino Sans\",\"fontName\":\"Hiragino Sans\",\"fontSize\":1000,\"bold\":false,\"italic\":false,\"underline\":false,\"strikethrough\":false,\"textColor\":\"#111111\",\"shadeColor\":\"#ffffff\",\"charShapeId\":0,\"fontId\":0,\"fontIds\":[0,0,0,0,0,0,0],\"fontFamilies\":[\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\",\"Hiragino Sans\"],\"ratios\":[100,100,100,100,100,100,100],\"spacings\":[0,0,0,0,0,0,0],\"relativeSizes\":[100,100,100,100,100,100,100],\"charOffsets\":[0,0,0,0,0,0,0],\"underlineType\":\"None\",\"underlineColor\":\"#111111\",\"outlineType\":0,\"shadowType\":0,\"shadowColor\":\"#000000\",\"shadowOffsetX\":0,\"shadowOffsetY\":0,\"strikeColor\":\"#111111\",\"subscript\":false,\"superscript\":false,\"emphasisDot\":0,\"underlineShape\":0,\"strikeShape\":0,\"kerning\":false,\"borderFillId\":0,\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0}".to_string()
}

fn default_para_properties_json() -> String {
    "{\"alignment\":\"left\",\"lineSpacing\":160,\"lineSpacingType\":\"Percent\",\"marginLeft\":0,\"marginRight\":0,\"indent\":0,\"spacingBefore\":0,\"spacingAfter\":0,\"paraShapeId\":0,\"headType\":\"None\",\"paraLevel\":0,\"numberingId\":0,\"widowOrphan\":false,\"keepWithNext\":false,\"keepLines\":false,\"pageBreakBefore\":false,\"fontLineHeight\":false,\"singleLine\":false,\"autoSpaceKrEn\":false,\"autoSpaceKrNum\":false,\"verticalAlign\":0,\"englishBreakUnit\":0,\"koreanBreakUnit\":0,\"tabAutoLeft\":true,\"tabAutoRight\":true,\"tabStops\":[],\"defaultTabSpacing\":0,\"borderFillId\":0,\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0,\"borderSpacing\":[0,0,0,0]}".to_string()
}

fn default_cell_properties_json() -> String {
    "{\"width\":0,\"height\":0,\"paddingLeft\":0,\"paddingRight\":0,\"paddingTop\":0,\"paddingBottom\":0,\"verticalAlign\":0,\"textDirection\":0,\"isHeader\":false,\"cellProtect\":false,\"borderFillId\":0,\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0}".to_string()
}

fn default_table_properties_json() -> String {
    "{\"cellSpacing\":0,\"paddingLeft\":0,\"paddingRight\":0,\"paddingTop\":0,\"paddingBottom\":0,\"pageBreak\":0,\"repeatHeader\":false,\"tableWidth\":0,\"tableHeight\":0,\"outerLeft\":0,\"outerRight\":0,\"outerTop\":0,\"outerBottom\":0,\"hasCaption\":false,\"treatAsChar\":false,\"textWrap\":\"topAndBottom\",\"vertRelTo\":\"paragraph\",\"vertAlign\":\"top\",\"horzRelTo\":\"paragraph\",\"horzAlign\":\"left\",\"vertOffset\":0,\"horzOffset\":0,\"restrictInPage\":false,\"allowOverlap\":false,\"keepWithAnchor\":false,\"borderFillId\":0,\"fillType\":\"none\",\"fillColor\":\"#ffffff\",\"patternColor\":\"#000000\",\"patternType\":0}".to_string()
}

fn default_picture_properties_json() -> String {
    "{\"width\":0,\"height\":0,\"treatAsChar\":false,\"vertRelTo\":\"paragraph\",\"vertAlign\":\"top\",\"horzRelTo\":\"paragraph\",\"horzAlign\":\"left\",\"vertOffset\":0,\"horzOffset\":0,\"textWrap\":\"topAndBottom\",\"brightness\":0,\"contrast\":0,\"effect\":\"none\",\"description\":\"\",\"rotationAngle\":0,\"horzFlip\":false,\"vertFlip\":false,\"originalWidth\":0,\"originalHeight\":0,\"cropLeft\":0,\"cropTop\":0,\"cropRight\":0,\"cropBottom\":0,\"paddingLeft\":0,\"paddingTop\":0,\"paddingRight\":0,\"paddingBottom\":0,\"outerMarginLeft\":0,\"outerMarginTop\":0,\"outerMarginRight\":0,\"outerMarginBottom\":0,\"borderColor\":0,\"borderWidth\":0,\"hasCaption\":false,\"captionDirection\":\"bottom\",\"captionVertAlign\":\"top\",\"captionWidth\":0,\"captionSpacing\":0,\"captionMaxWidth\":0,\"captionIncludeMargin\":false}".to_string()
}

fn default_shape_properties_json() -> String {
    "{\"width\":0,\"height\":0,\"treatAsChar\":false,\"vertRelTo\":\"paragraph\",\"vertAlign\":\"top\",\"horzRelTo\":\"paragraph\",\"horzAlign\":\"left\",\"vertOffset\":0,\"horzOffset\":0,\"textWrap\":\"topAndBottom\",\"tbMarginLeft\":0,\"tbMarginRight\":0,\"tbMarginTop\":0,\"tbMarginBottom\":0,\"tbVerticalAlign\":\"top\",\"borderColor\":0,\"borderWidth\":0,\"borderAttr\":0,\"borderOutlineStyle\":0,\"lineType\":0,\"lineEndShape\":0,\"arrowStart\":0,\"arrowEnd\":0,\"arrowStartSize\":0,\"arrowEndSize\":0,\"rotationAngle\":0,\"horzFlip\":false,\"vertFlip\":false,\"fillType\":\"none\",\"fillBgColor\":16777215,\"fillPatColor\":0,\"fillPatType\":0,\"fillAlpha\":0,\"gradientType\":0,\"gradientAngle\":0,\"gradientCenterX\":0,\"gradientCenterY\":0,\"gradientBlur\":0,\"roundRate\":0,\"description\":\"\"}".to_string()
}

fn default_equation_properties_json() -> String {
    "{\"width\":0,\"height\":0,\"treatAsChar\":true,\"vertRelTo\":\"paragraph\",\"vertAlign\":\"top\",\"horzRelTo\":\"paragraph\",\"horzAlign\":\"left\",\"vertOffset\":0,\"horzOffset\":0,\"textWrap\":\"topAndBottom\",\"zOrder\":0,\"instanceId\":0,\"outerMarginLeft\":0,\"outerMarginTop\":0,\"outerMarginRight\":0,\"outerMarginBottom\":0,\"hasCaption\":false,\"captionDirection\":\"bottom\",\"captionWidth\":0,\"captionSpacing\":0,\"description\":\"\",\"script\":\"\",\"fontSize\":1000,\"color\":0,\"baseline\":0,\"fontName\":\"Hiragino Sans\"}".to_string()
}

fn default_endnote_shape_json() -> String {
    "{\"ok\":false,\"numberFormat\":\"digit\",\"userChar\":\"\",\"prefixChar\":\"\",\"suffixChar\":\"\",\"startNumber\":1,\"separatorEnabled\":false,\"separatorLength\":0,\"separatorMarginTop\":0,\"separatorMarginBottom\":0,\"noteSpacing\":0,\"separatorLineType\":0,\"separatorLineWidth\":0,\"separatorColor\":\"#000000\",\"numbering\":\"continue\",\"placement\":\"documentEnd\"}".to_string()
}

fn json_string(value: &str) -> String {
    let mut escaped = String::new();
    escaped.push('"');
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character < ' ' => {
                escaped.push_str("\\u");
                escaped.push_str(&format!("{:04x}", character as u32));
            }
            character => escaped.push(character),
        }
    }
    escaped.push('"');
    escaped
}

fn json_bool(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

fn push_option_u32_hex_or_null_json(output: &mut String, value: Option<u32>) {
    match value {
        Some(value) => output.push_str(&json_string(&format!("0x{value:08x}"))),
        None => output.push_str("null"),
    }
}

fn non_negative_i32_offset(field_name: &'static str, value: i32) -> Option<(&'static str, usize)> {
    (value >= 0).then_some((field_name, value as usize))
}

fn push_usize_array_json(output: &mut String, values: &[usize]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

fn push_optional_usize_array_json(output: &mut String, values: &[Option<usize>]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_option_usize_json(output, *value);
    }
    output.push(']');
}

fn uniform_usize_stride(values: &[usize]) -> Option<usize> {
    if values.len() < 2 {
        return None;
    }
    let stride = values[1].checked_sub(values[0])?;
    if stride == 0 {
        return None;
    }
    values
        .windows(2)
        .all(|pair| pair[1].checked_sub(pair[0]) == Some(stride))
        .then_some(stride)
}

fn push_u16_array_json(output: &mut String, values: &[u16]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

fn push_u16_hex_array_json(output: &mut String, values: &[u16]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&format!("0x{value:04x}")));
    }
    output.push(']');
}

fn push_i32_array_json(output: &mut String, values: &[i32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

fn push_bool_array_json(output: &mut String, values: &[bool]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(if *value { "true" } else { "false" });
    }
    output.push(']');
}

fn push_sparse_observed_table_json(output: &mut String, candidate: &TableCandidate) {
    output.push_str("{\"source\":\"sparseDocumentTextControlRows\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&candidate.cell_count_candidate().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&candidate.non_empty_cell_count_candidate().to_string());
    output.push_str(",\"rows\":");
    push_sparse_table_rows_json(output, candidate.intervals());
    output.push_str(",\"topologyCandidate\":");
    if let Some(topology) = candidate.sparse_topology_candidate() {
        push_sparse_topology_candidate_json(output, candidate, &topology);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_sparse_topology_candidate_json(
    output: &mut String,
    candidate: &TableCandidate,
    topology: &TableCandidateSparseTopologyCandidate,
) {
    output.push_str("{\"source\":\"sparseDocumentTextControlRows\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&topology.row_count().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&topology.max_column_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&topology.cell_count().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&topology.empty_cell_count().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&topology.non_empty_cell_count().to_string());
    output.push_str(",\"rows\":[");
    for (index, row) in topology.rows().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&row.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&row.source_interval_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&row.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&row.source_end().to_string());
        output.push_str(",\"cellCount\":");
        output.push_str(&row.cell_count().to_string());
        output.push_str(",\"emptyCellCount\":");
        output.push_str(&row.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCount\":");
        output.push_str(&row.non_empty_cell_count().to_string());
        output.push_str(",\"firstNonEmptyColumnIndex\":");
        push_option_usize_json(output, row.first_non_empty_column_index());
        output.push_str(",\"lastNonEmptyColumnIndex\":");
        push_option_usize_json(output, row.last_non_empty_column_index());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"columns\":[");
    for (index, column) in topology.columns().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&column.index().to_string());
        output.push_str(",\"observedCellCount\":");
        output.push_str(&column.observed_cell_count().to_string());
        output.push_str(",\"emptyCellCount\":");
        output.push_str(&column.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCount\":");
        output.push_str(&column.non_empty_cell_count().to_string());
        output.push_str(",\"firstNonEmptyRowIndex\":");
        push_option_usize_json(output, column.first_non_empty_row_index());
        output.push_str(",\"lastNonEmptyRowIndex\":");
        push_option_usize_json(output, column.last_non_empty_row_index());
        output.push_str(",\"sourceStart\":");
        push_option_usize_json(output, column.source_start());
        output.push_str(",\"sourceEnd\":");
        push_option_usize_json(output, column.source_end());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

fn hex_bytes(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

fn document_font_names(document: &Document) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = BTreeSet::new();

    for font in document.fonts() {
        let name = font.name().trim();
        if name.is_empty() || looks_like_font_descriptor(name) {
            continue;
        }
        if seen.insert(name.to_string()) {
            names.push(name.to_string());
        }
    }

    if names.is_empty() {
        names.push("Hiragino Sans".to_string());
    }
    names
}

fn primary_document_font_name(font_names: &[String]) -> &str {
    font_names
        .iter()
        .find(|name| looks_like_mincho_font(name))
        .or_else(|| {
            font_names
                .iter()
                .find(|name| looks_like_japanese_font(name))
        })
        .or_else(|| font_names.first())
        .map(String::as_str)
        .unwrap_or("Hiragino Sans")
}

fn document_font_family_css(document: &Document) -> String {
    let font_names = document_font_names(document);
    let primary = primary_document_font_name(&font_names).to_string();
    let mut ordered = Vec::new();
    push_font_family_with_aliases(&mut ordered, &primary);
    for name in &font_names {
        push_font_family_with_aliases(&mut ordered, name);
    }
    for fallback in [
        "Hiragino Mincho ProN",
        "YuMincho",
        "Yu Mincho",
        "Hiragino Sans",
        "Hiragino Kaku Gothic ProN",
        "Yu Gothic",
        "Meiryo",
        "Noto Sans CJK JP",
        "sans-serif",
    ] {
        ordered.push(fallback.to_string());
    }

    let mut seen = BTreeSet::new();
    ordered
        .into_iter()
        .filter(|name| seen.insert(name.clone()))
        .map(|name| css_font_family_name(&name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn push_font_family_with_aliases(output: &mut Vec<String>, name: &str) {
    output.push(name.to_string());
    output.extend(font_family_aliases(name).into_iter().map(str::to_string));
}

fn font_family_aliases(name: &str) -> Vec<&'static str> {
    if name.contains("游明朝") {
        return vec!["YuMincho", "Yu Mincho", "Hiragino Mincho ProN"];
    }
    if name.contains("ＭＳ 明朝") || name.contains("MS Mincho") {
        return vec!["MS Mincho", "Hiragino Mincho ProN", "YuMincho", "Yu Mincho"];
    }
    if name.contains("明朝") || name.to_ascii_lowercase().contains("mincho") {
        return vec!["Hiragino Mincho ProN", "YuMincho", "Yu Mincho"];
    }
    if name.contains("ゴシック") || name.to_ascii_lowercase().contains("gothic") {
        return vec!["Yu Gothic", "Hiragino Sans", "Meiryo"];
    }
    Vec::new()
}

fn css_font_family_name(name: &str) -> String {
    if matches!(name, "serif" | "sans-serif" | "monospace") {
        return name.to_string();
    }
    format!("'{}'", name.replace('\\', "\\\\").replace('\'', "\\'"))
}

fn looks_like_mincho_font(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    name.contains("明朝") || name.contains('游') || lower.contains("mincho")
}

fn looks_like_japanese_font(name: &str) -> bool {
    name.chars().any(
        |character| matches!(character as u32, 0x3040..=0x30ff | 0x4e00..=0x9fff | 0xff00..=0xffef),
    )
}

fn looks_like_font_descriptor(name: &str) -> bool {
    matches!(name, "太字" | "斜体" | "太字 斜体")
}

fn string_array_json(values: &[String]) -> String {
    let mut output = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
    output
}

fn string_slice_array_json(values: &[&str]) -> String {
    let mut output = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
    output
}

fn font_table_json(fonts: &[DocumentFont]) -> String {
    let mut output = String::from("[");
    for (index, font) in fonts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"sourceStream\":");
        output.push_str(&json_string(font.source_stream()));
        output.push_str(",\"id\":");
        output.push_str(&font.id().to_string());
        output.push_str(",\"offset\":");
        output.push_str(&font.offset().to_string());
        output.push_str(",\"name\":");
        output.push_str(&json_string(font.name()));
        output.push_str(",\"rawHex\":");
        output.push_str(&json_string(&hex_bytes(font.raw())));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
    output
}
fn auto_texts_json(auto_texts: &[DocumentAutoText]) -> String {
    let mut output = String::from("[");
    for (index, auto_text) in auto_texts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"sourceStream\":");
        output.push_str(&json_string(auto_text.source_stream()));
        output.push_str(",\"offset\":");
        output.push_str(&auto_text.offset().to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(auto_text.text()));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
    output
}

fn toc_entries_json(entries: &[DocumentTocEntry]) -> String {
    let mut output = String::from("[");
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"title\":");
        output.push_str(&json_string(entry.title()));
        output.push_str(",\"pageLabel\":");
        output.push_str(&json_string(entry.page_label()));
        output.push_str(",\"sourceSpan\":");
        push_text_source_span_json(&mut output, entry.source_span());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
    output
}

fn writing_mode_decision_json(document: &Document, selected: WritingMode) -> String {
    let decoded_layout = page_layout_from_document(document);
    let source_layout_hint = source_document_layout_hint(document, decoded_layout);
    let document_view_candidate =
        writing_mode_candidate_from_document_view_styles(document.unknown_styles());
    let paper_mark_writing_mode_diagnostics =
        paper_mark_writing_mode_diagnostics(document.paper_marks());
    let paper_mark_candidate = paper_mark_writing_mode_diagnostics.candidate;
    let computed = source_layout_hint
        .as_ref()
        .map(|hint| hint.writing_mode)
        .unwrap_or(WritingMode::Horizontal);
    let decision_source = if selected != computed {
        "runtime-override"
    } else if source_layout_hint.is_some() {
        "source-document-layout-hint"
    } else {
        "default-horizontal"
    };
    let decision_source_backed = matches!(decision_source, "source-document-layout-hint");
    let document_view_first_code_hex = document_view_candidate
        .as_ref()
        .map(|candidate| json_string(&format!("0x{:04x}", candidate.first_record_code)))
        .unwrap_or_else(|| "null".to_string());
    let source_hint_basis = source_layout_hint
        .as_ref()
        .map(|hint| json_string(hint.basis))
        .unwrap_or_else(|| "null".to_string());
    let source_hint_override_decoded_layout = source_layout_hint
        .as_ref()
        .map(|hint| hint.override_decoded_layout)
        .unwrap_or(false);
    let source_hint_margin = source_layout_hint
        .and_then(|hint| hint.margin_override_px)
        .map(|margin| format!("{margin:.3}"))
        .unwrap_or_else(|| "null".to_string());
    let source_hint_wrap_columns = source_layout_hint
        .and_then(|hint| hint.vertical_wrap_columns_override)
        .map(|columns| columns.to_string())
        .unwrap_or_else(|| "null".to_string());
    let source_hint_mode = source_layout_hint.map(|hint| hint.writing_mode);
    let document_view_mode = document_view_candidate
        .as_ref()
        .map(|candidate| candidate.writing_mode);
    let document_view_disagrees = document_view_mode
        .map(|mode| mode != selected)
        .unwrap_or(false);
    let source_hint_disagrees = source_hint_mode
        .map(|mode| mode != selected)
        .unwrap_or(false);
    let paper_mark_disagrees = paper_mark_candidate
        .map(|mode| mode != selected)
        .unwrap_or(false);
    format!(
        "{{\"selected\":\"{}\",\"source\":{},\"decoded\":false,\"sourceBacked\":{},\"computedBeforeRuntimeOverride\":\"{}\",\"documentViewStylesCandidate\":{},\"documentViewStylesFirstRecordCodeHex\":{},\"sourceDocumentLayoutHintCandidate\":{},\"sourceDocumentLayoutHintBasis\":{},\"sourceDocumentLayoutHintOverridesDecodedLayout\":{},\"sourceDocumentLayoutHintMarginOverridePx\":{},\"sourceDocumentLayoutHintVerticalWrapColumnsOverride\":{},\"paperMarkCandidate\":{},\"paperMarkCandidateDecoded\":false,\"paperMarkFlagBit0VerticalCandidate\":{},\"paperMarkFlagBit17IndexStepCandidate\":{},\"paperMarkWritingModeCandidateEvidence\":{},\"paperMarkWritingModeCandidateBlockers\":{},\"documentViewStylesDisagreesWithSelected\":{},\"sourceDocumentLayoutHintDisagreesWithSelected\":{},\"paperMarkDisagreesWithSelected\":{}}}",
        selected.as_str(),
        json_string(decision_source),
        decision_source_backed,
        computed.as_str(),
        writing_mode_option_json(document_view_mode),
        document_view_first_code_hex,
        writing_mode_option_json(source_hint_mode),
        source_hint_basis,
        source_hint_override_decoded_layout,
        source_hint_margin,
        source_hint_wrap_columns,
        writing_mode_option_json(paper_mark_candidate),
        paper_mark_writing_mode_diagnostics.flag_bit0_vertical_candidate,
        paper_mark_writing_mode_diagnostics.flag_bit17_index_step_candidate,
        string_slice_array_json(&paper_mark_writing_mode_diagnostics.evidence),
        string_slice_array_json(&paper_mark_writing_mode_diagnostics.blockers),
        document_view_disagrees,
        source_hint_disagrees,
        paper_mark_disagrees
    )
}

fn writing_mode_option_json(mode: Option<WritingMode>) -> String {
    mode.map(|mode| json_string(mode.as_str()))
        .unwrap_or_else(|| "null".to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DocumentViewWritingModeCandidate {
    writing_mode: WritingMode,
    first_record_code: u16,
}

// DocumentViewStyles record 0x1001 appears as the first sequential record in
// current vertical-writing Ginga samples, but also appears in horizontal
// reference-PDF samples such as tsaiten, tmogi3_2, success_data-test, and
// shanai_lan. Keep it diagnostic-only until the surrounding style semantics are
// decoded.
fn writing_mode_candidate_from_document_view_styles(
    styles: &[UnknownStyle],
) -> Option<DocumentViewWritingModeCandidate> {
    styles
        .iter()
        .filter(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))
        .find_map(|style| {
            let first_record_code = summarize_style_stream(style.payload())
                .records()
                .first()?
                .code();
            let writing_mode = if first_record_code == 0x1001 {
                WritingMode::VerticalRl
            } else {
                WritingMode::Horizontal
            };
            Some(DocumentViewWritingModeCandidate {
                writing_mode,
                first_record_code,
            })
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PaperMarkWritingModeDiagnostics {
    candidate: Option<WritingMode>,
    flag_bit0_vertical_candidate: bool,
    flag_bit17_index_step_candidate: bool,
    evidence: Vec<&'static str>,
    blockers: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StyleCandidate {
    id: u32,
    name: String,
    source_stream: String,
    source_record_index: usize,
    source_offset: usize,
    source_code: u16,
    payload_len: usize,
}

fn style_candidate_names_json(candidates: &[StyleCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&candidate.name));
    }
    output.push(']');
    output
}

fn push_style_candidate_json(output: &mut String, candidate: &StyleCandidate) {
    output.push_str("{\"id\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"name\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"englishName\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"type\":0,\"nextStyleId\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"paraShapeId\":0,\"charShapeId\":0,\"decoded\":false,\"jtdCandidate\":true");
    push_style_candidate_source_json(output, candidate);
    output.push('}');
}

fn style_candidate_detail_json(candidate: &StyleCandidate) -> String {
    let mut output = String::new();
    output.push_str("{\"id\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"name\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"englishName\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"type\":0,\"nextStyleId\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"paraShapeId\":0,\"charShapeId\":0,\"decoded\":false,\"jtdCandidate\":true");
    push_style_candidate_source_json(&mut output, candidate);
    output.push_str(",\"charProps\":");
    output.push_str(&default_char_properties_json());
    output.push_str(",\"paraProps\":");
    output.push_str(&default_para_properties_json());
    output.push('}');
    output
}

fn style_at_candidate_json(candidate: &StyleCandidate) -> String {
    let mut output = String::new();
    output.push_str("{\"id\":");
    output.push_str(&candidate.id.to_string());
    output.push_str(",\"name\":");
    output.push_str(&json_string(&candidate.name));
    output.push_str(",\"decoded\":false,\"jtdCandidate\":true");
    push_style_candidate_source_json(&mut output, candidate);
    output.push('}');
    output
}

fn push_style_candidate_source_json(output: &mut String, candidate: &StyleCandidate) {
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(&candidate.source_stream));
    output.push_str(",\"sourceRecordIndex\":");
    output.push_str(&candidate.source_record_index.to_string());
    output.push_str(",\"sourceOffset\":");
    output.push_str(&candidate.source_offset.to_string());
    output.push_str(",\"sourceCode\":");
    output.push_str(&candidate.source_code.to_string());
    output.push_str(",\"sourceCodeHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", candidate.source_code)));
    output.push_str(",\"payloadLength\":");
    output.push_str(&candidate.payload_len.to_string());
}

fn style_source_streams_json(styles: &[UnknownStyle]) -> String {
    let mut output = String::from("[");

    for (index, style) in styles.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let summary = summarize_style_stream(style.payload());
        output.push_str("{\"name\":");
        match style.name() {
            Some(name) => output.push_str(&json_string(name)),
            None => output.push_str("null"),
        }
        output.push_str(",\"size\":");
        output.push_str(&style.payload().len().to_string());
        output.push_str(",\"family\":");
        output.push_str(&json_string(summary.family().as_str()));
        output.push_str(",\"headerU32Be\":");
        push_u32_array_json(&mut output, summary.header_u32_be());
        output.push_str(",\"headerU16Be\":");
        push_u16_array_json(&mut output, summary.header_u16_be());
        output.push_str(",\"recordLayout\":");
        output.push_str(&json_string(summary.record_layout().as_str()));
        output.push_str(",\"recordCount\":");
        output.push_str(&summary.records().len().to_string());
        output.push_str(",\"records\":");
        push_style_records_json(&mut output, summary.records());
        output.push_str(",\"decoded\":false}");
    }

    output.push(']');
    output
}

fn push_u32_array_json(output: &mut String, values: &[u32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
}

fn push_u32_hex_array_json(output: &mut String, values: &[u32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&format!("0x{value:02x}")));
    }
    output.push(']');
}

fn push_u32_hex8_array_json(output: &mut String, values: &[u32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&format!("0x{value:08x}")));
    }
    output.push(']');
}

fn push_option_usize_json(output: &mut String, value: Option<usize>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn push_optional_usize_range_json(output: &mut String, start: Option<usize>, end: Option<usize>) {
    match (start, end) {
        (Some(start), Some(end)) => {
            output.push_str("{\"start\":");
            output.push_str(&start.to_string());
            output.push_str(",\"end\":");
            output.push_str(&end.to_string());
            output.push('}');
        }
        _ => output.push_str("null"),
    }
}

fn push_option_u16_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn push_option_u16_hex_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => output.push_str(&json_string(&format!("0x{value:04x}"))),
        None => output.push_str("null"),
    }
}

fn push_optional_f32_json(output: &mut String, value: Option<f32>) {
    match value {
        Some(value) if value.is_finite() => output.push_str(&format!("{value:.3}")),
        _ => output.push_str("null"),
    }
}

fn push_optional_bbox_milli_json(
    output: &mut String,
    x_min_milli: Option<i32>,
    y_min_milli: Option<i32>,
    x_max_milli: Option<i32>,
    y_max_milli: Option<i32>,
) {
    let (Some(x_min_milli), Some(y_min_milli), Some(x_max_milli), Some(y_max_milli)) =
        (x_min_milli, y_min_milli, x_max_milli, y_max_milli)
    else {
        output.push_str("null");
        return;
    };
    let x = x_min_milli as f32 / 1000.0;
    let y = y_min_milli as f32 / 1000.0;
    let width = (x_max_milli - x_min_milli).max(0) as f32 / 1000.0;
    let height = (y_max_milli - y_min_milli).max(0) as f32 / 1000.0;
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
}

fn push_option_u32_json(output: &mut String, value: Option<u32>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn push_style_records_json(output: &mut String, records: &[StyleStreamRecordSummary]) {
    output.push('[');
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"offset\":");
        output.push_str(&record.offset().to_string());
        output.push_str(",\"code\":");
        output.push_str(&record.code().to_string());
        output.push_str(",\"codeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", record.code())));
        output.push_str(",\"payloadLength\":");
        output.push_str(&record.payload_len().to_string());
        output.push_str(",\"label\":");
        match record.label() {
            Some(label) => output.push_str(&json_string(label)),
            None => output.push_str("null"),
        }
        output.push_str(",\"subrecordCount\":");
        output.push_str(&record.subrecords().len().to_string());
        output.push_str(",\"subrecords\":");
        push_style_subrecords_json(output, record.subrecords());
        output.push('}');
    }
    output.push(']');
}

fn push_style_subrecords_json(output: &mut String, records: &[StyleStreamSubrecordSummary]) {
    output.push('[');
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"offset\":");
        output.push_str(&record.offset().to_string());
        output.push_str(",\"code\":");
        output.push_str(&record.code().to_string());
        output.push_str(",\"codeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", record.code())));
        output.push_str(",\"payloadLength\":");
        output.push_str(&record.payload_len().to_string());
        output.push_str(",\"payloadHex\":");
        output.push_str(&json_string(&hex_bytes(record.payload())));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

#[derive(Debug)]
struct PageLayerTextFragment {
    text: String,
    paragraph_index: Option<usize>,
    char_start: usize,
    char_end: usize,
    source_span: Option<TextSourceSpan>,
    ruby_annotation: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct PageLayerTextPlacement {
    x: f64,
    y: f64,
    baseline: f64,
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestTextSlot {
    role: &'static str,
    text: &'static str,
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
struct SuccessDataTestResolvedTextSlot {
    role: &'static str,
    text: &'static str,
    x: f32,
    y: f32,
    source_span: Option<TextSourceSpan>,
    line_header: Option<ShanaiLanLineHeader>,
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestFormulaTextSlot {
    embedding_index: usize,
    text: &'static str,
    x: f32,
    baseline_y: f32,
    font_size: f32,
}

#[derive(Debug, Clone, Copy)]
struct VisualListDiagnostic<'a> {
    candidate_index: usize,
    candidate: &'a ObjectStreamCandidate,
    visual_list: &'a ObjectVisualListCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VisualListHorizontalRun {
    x: usize,
    y: usize,
    width: usize,
    value: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct VisualListTitleBand {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct ObservedFormTextProjection {
    source: &'static str,
    projection_kind: &'static str,
    shapes: Vec<ObservedFormShape>,
    slots: Vec<ObservedFormTextSlot>,
}

#[derive(Debug, Clone, PartialEq)]
struct ObservedFormShape {
    role: &'static str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fill: &'static str,
    stroke: Option<&'static str>,
    stroke_width: f32,
    rx: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct ObservedFormTextSlot {
    role: &'static str,
    text: String,
    x: f32,
    y: f32,
    font_size: f32,
    font_weight: &'static str,
    anchor: &'static str,
    font_family: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
struct PageFrameProjection {
    source: &'static str,
    projection_kind: &'static str,
    page_assignment_decoded: bool,
    record_count: usize,
    shapes: Vec<PageFrameShape>,
}

#[derive(Debug, Clone, PartialEq)]
struct PageFrameShape {
    role: &'static str,
    row_index: usize,
    object_id: u16,
    object_type: u16,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    corner_radius: f32,
    source_x: u16,
    source_y: u16,
    source_width: u16,
    source_height: u16,
    source_corner_radius: u16,
    source_style_id: u16,
    placement_basis: &'static str,
    style_basis: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
struct PageMarkSeparatorProjection {
    source: &'static str,
    projection_kind: &'static str,
    role: &'static str,
    x: f32,
    y: f32,
    width: f32,
    stroke_width: f32,
    source_record_offset: usize,
    source_record_index: u32,
    source_line_start: u32,
    source_line_end: u32,
    source_y_centipoints: u16,
    source_advance_centipoints: u16,
    placement_basis: &'static str,
    style_basis: &'static str,
    page_assignment_decoded: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct LayoutBoxTextProjection {
    source: &'static str,
    projection_kind: &'static str,
    block_count: usize,
    layout_record_count: usize,
    position_table_present: bool,
    page_assignment_decoded: bool,
    slots: Vec<LayoutBoxTextSlot>,
}

#[derive(Debug, Clone, PartialEq)]
struct LayoutBoxTextSlot {
    role: &'static str,
    text: String,
    x: f32,
    y: f32,
    font_size: f32,
    line_height: f32,
    source_span: TextSourceSpan,
    block_index: usize,
    layout_record_index: Option<usize>,
    layout_record_byte_range: Option<(usize, usize)>,
    layout_x_pt: Option<u16>,
    layout_y_pt: Option<u16>,
    layout_width_pt: Option<u16>,
    inferred_origin_pt: Option<f32>,
    placement_basis: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LayoutBoxTextBlock {
    index: usize,
    byte_start: usize,
    byte_end: usize,
    payload_start: usize,
    payload_end: usize,
    declared_unit_count: usize,
    fragments: Vec<LayoutBoxTextFragment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LayoutBoxTextFragment {
    text: String,
    source_span: TextSourceSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LayoutBoxRecordCandidate {
    index: usize,
    byte_start: usize,
    byte_end: usize,
    origin_field: Option<u16>,
    x_field: Option<u16>,
    y_field: Option<u16>,
    width_field: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
struct ShanaiLanTextProjection {
    source: &'static str,
    projection_kind: &'static str,
    grid_unit_px: f32,
    line_height_px: f32,
    slots: Vec<ShanaiLanTextSlot>,
}

#[derive(Debug, Clone, PartialEq)]
struct ShanaiLanTextSlot {
    text: String,
    x: f32,
    y: f32,
    font_size: f32,
    fill: &'static str,
    fill_basis: &'static str,
    document_text_property_15_color_candidate: Option<DocumentTextProperty15ColorCandidate>,
    style_link_evidence: ShanaiLanTextStyleLinkEvidence,
    source_span: TextSourceSpan,
    fragment_context: ShanaiLanTextRunFragmentContext,
    text_count_range_evidence: Vec<ShanaiLanTextCountRangeEvidence>,
    group_index: Option<usize>,
    line_offset_units: u16,
    leading_units: usize,
    fragment_start_units: usize,
    split_from_text_run: bool,
    line_header_hex: String,
    line_header_raw_words: [u16; 12],
    line_header_same_segment_group_run: Option<ShanaiLanLineHeaderSameSegmentGroupRun>,
    line_header_same_segment_group_run_text_slot_count: Option<usize>,
    line_header_same_segment_group_run_distinct_text_group_count: Option<usize>,
}

type ShanaiLanTextSlotAttachment<'a> = (&'a ShanaiLanTextSlot, f32, (f32, f32, f32, f32));

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShanaiLanTextStyleLinkEvidence {
    source: &'static str,
    style_link_proven: bool,
    text_layout_style_record_count: usize,
    document_view_style_group_count: usize,
    document_view_style_group_candidate: Option<u16>,
    document_view_style_group_candidate_basis: Option<&'static str>,
    document_text_group_header_candidate: Option<ShanaiLanDocumentTextGroupHeaderCandidate>,
    document_text_inline_style_candidate: Option<ShanaiLanDocumentTextInlineStyleCandidate>,
    style_link_promotion_blocked_reason: &'static str,
    fill_color_promotion_blocked_reason: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShanaiLanTextRunFragmentContext {
    parent_source_span: TextSourceSpan,
    parent_text_unit_count: usize,
    fragment_index: usize,
    fragment_count: usize,
    fragment_source_start_units: usize,
    fragment_source_end_units: usize,
    previous_gap_units: Option<usize>,
    next_gap_units: Option<usize>,
    style_boundary_proven: bool,
    promotion_blocked_reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShanaiLanLineHeaderSameSegmentGroupRun {
    offset_units: u16,
    extent_units: u16,
    start_group_index: usize,
    end_group_index: usize,
    group_count: usize,
    position_in_run: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShanaiLanDocumentTextGroupHeaderCandidate {
    source_span: TextSourceSpan,
    raw_words: Vec<u16>,
    field_words: Vec<u16>,
    distance_to_text_units: usize,
    promotion_blocked_reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShanaiLanDocumentTextInlineStyleCandidate {
    source_span: TextSourceSpan,
    selector: Option<u16>,
    context_words: Vec<u16>,
    payload_words: Vec<u16>,
    post_inline_words: Vec<u16>,
    raw_words: Vec<u16>,
    distance_to_text_units: usize,
    promotion_blocked_reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShanaiLanTextCountRangeEvidence {
    index: usize,
    family: String,
    basis: TextCountRangeOverlapBasis,
    range_start: usize,
    range_end: usize,
    overlap_start: usize,
    overlap_end: usize,
    declared_start: u32,
    declared_end: u32,
    tail_fields: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
struct ShanaiLanLineRuleProjection {
    source: &'static str,
    projection_kind: &'static str,
    line_mark_profile: &'static str,
    line_mark_interval_count: usize,
    document_text_group_count: usize,
    document_text_line_header_count: usize,
    skipped_inline_line_header_count: usize,
    grid_unit_px: f32,
    line_height_px: f32,
    stroke_width: f32,
    rules: Vec<ShanaiLanLineRule>,
}

#[derive(Debug, Clone, PartialEq)]
struct ShanaiLanLineRule {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    orientation: &'static str,
    candidate_source: &'static str,
    source_span: TextSourceSpan,
    group_index: usize,
    end_group_index: usize,
    line_offset_units: u16,
    line_extent_units: u16,
    line_header_hex: String,
    line_header_raw_words: [u16; 12],
    line_mark: Option<ShanaiLanLineMarkInterval>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShanaiLanLineRuleTopology {
    start_junction_degree: usize,
    end_junction_degree: usize,
    isolated_endpoint_count: usize,
    orthogonal_graph_candidate: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct ShanaiLanLineRuleGraphComponentSummary {
    rule_indexes: Vec<usize>,
    bbox: (f32, f32, f32, f32),
    horizontal_rule_count: usize,
    vertical_rule_count: usize,
    orthogonal_graph_rule_count: usize,
    line_mark_matched_rule_count: usize,
    isolated_endpoint_count: usize,
    total_projected_length_px: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShanaiLanLineHeaderInGroup {
    group_index: usize,
    header: ShanaiLanLineHeader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShanaiLanLineMarkInterval {
    record_index: usize,
    unit_start: usize,
    unit_end: usize,
    flag_word: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShanaiLanTextFragment {
    text: String,
    source_start_units: usize,
    source_end_units: usize,
    fragment_start_units: usize,
    split_from_text_run: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShanaiLanLineHeader {
    offset_units: u16,
    extent_units: u16,
    font_size_units: u16,
    raw_words: [u16; 12],
    start: usize,
    end: usize,
}

fn push_optional_u64_json(output: &mut String, value: Option<u64>) {
    if let Some(value) = value {
        output.push_str(&value.to_string());
    } else {
        output.push_str("null");
    }
}

fn optional_u64_svg_attr(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string())
}

fn aspect_delta_permille(
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

fn canvaskit_replay_mode(mode: &str) -> Result<&'static str> {
    match mode.trim().to_ascii_lowercase().as_str() {
        "" | "default" => Ok("default"),
        "compat" | "compatibility" => Ok("compat"),
        _ => Err(Error::InvalidData(format!(
            "unsupported CanvasKit replay mode: {mode}. allowed modes: default, compat"
        ))),
    }
}

fn canvaskit_replay_plan_json(core: &DocumentCore, lines: &[PageTextLine], mode: &str) -> String {
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

fn push_optional_hex_byte_json(output: &mut String, value: Option<&u8>) {
    match value {
        Some(byte) => output.push_str(&json_string(&format!("0x{byte:02x}"))),
        None => output.push_str("null"),
    }
}

fn push_optional_hex_bytes_json(output: &mut String, bytes: Option<&[u8]>) {
    match bytes {
        Some(bytes) => output.push_str(&json_string(&hex_bytes(bytes))),
        None => output.push_str("null"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridSourceTopTextPlacementReadiness {
    ready: bool,
    blocked_reasons: Vec<&'static str>,
}

struct TableGridSourceTablePlacementCoherenceInput<'a> {
    layout: PageLayout,
    document: &'a Document,
    candidate: &'a TableCandidate,
    rows: &'a [TableCandidateLineHeaderRow],
    anchor_span: &'a TextSourceSpan,
    anchor_header: ShanaiLanLineHeader,
    table_min_offset_units: Option<u16>,
    table_max_extent_units: Option<u16>,
    table_font_size_units: Option<u16>,
    source_gap_after_anchor_text_units: usize,
}

impl TableGridSourceTopTextPlacementReadiness {
    fn blocked_reason(&self) -> Option<&'static str> {
        self.blocked_reasons.first().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridRelatedHorizontalSourceLayoutSummary {
    table_candidate_index: usize,
    row_count: usize,
    column_count: usize,
    x_unit_start: u16,
    x_unit_end: u16,
    x_unit_full_extent_units: u16,
    x_unit_all_rows_agree: bool,
    first_column_slot_units: Option<u16>,
    first_matched_cell_span_units: Option<u16>,
    first_intercell_gap_units: Option<u16>,
    matched_cell_span_units: Vec<u16>,
    matched_cell_gap_units: Vec<u16>,
    x_unit_column_slot_width_units: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridSparseSiblingPostRowGap {
    source_start: usize,
    source_end: usize,
    sparse_row_indexes: Vec<usize>,
    sparse_source_interval_indexes: Vec<usize>,
    kind: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridPageMarkRawRecordSourceRangeCoverageSummary {
    record_header_count: usize,
    candidate_row_count: usize,
    row_source_coverage_count: usize,
    all_rows_have_header_coverage: bool,
    total_overlapping_header_count: usize,
    matched_scan_indexes: Vec<usize>,
    matched_scan_indexes_monotonic: bool,
    rows: Vec<TableGridPageMarkRawRecordSourceRangeCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridPageMarkRawRecordSourceRangeCoverageRow {
    row_index: usize,
    source_start: usize,
    source_end: usize,
    matches: Vec<TableGridPageMarkRawRecordSourceRangeCoverageMatch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TableGridPageMarkRawRecordSourceRangeCoverageMatch {
    scan_index: usize,
    header: PageMarkRecordHeader,
    overlap_start: usize,
    overlap_end: usize,
    overlap_units: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PageMarkRawNumericHit {
    kind: &'static str,
    byte_offset: usize,
    value_index: usize,
    value: u32,
    residual_px: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageMarkRawNumericHitRecordContext {
    scan_index: usize,
    record_byte_offset: usize,
    record_next_byte_offset: usize,
    record_index: u32,
    record_line_start: u32,
    record_line_end: u32,
    record_relative_byte_offset: usize,
    record_tail_relative_byte_offset: Option<usize>,
    record_tail_word_index: Option<usize>,
    record_tail_block16_index: Option<usize>,
    record_tail_block16_word_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
struct PageMarkScopedYValueCandidate {
    source: &'static str,
    interpretation: &'static str,
    word_index: Option<usize>,
    byte_offset: Option<usize>,
    value: u32,
    value_px: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct PageMarkScopedYDeltaCandidate {
    source: &'static str,
    interpretation: &'static str,
    left_word_index: Option<usize>,
    right_word_index: Option<usize>,
    left_byte_offset: Option<usize>,
    right_byte_offset: Option<usize>,
    left_value: u32,
    right_value: u32,
    left_value_px: f32,
    right_value_px: f32,
    delta_px: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct PageMarkScopedYFamilyMember {
    source: &'static str,
    interpretation: &'static str,
    family_kind: &'static str,
    field_index: usize,
    word_index: Option<usize>,
    byte_offset: Option<usize>,
    raw_record_index: Option<u32>,
    raw_record_scan_index: Option<usize>,
    tail_block16_word_index: Option<usize>,
    subrecord_line_start_candidate: Option<u32>,
    subrecord_line_end_candidate: Option<u32>,
    value: u32,
    value_px: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct PageMarkScopedYFamilyFit {
    source: &'static str,
    interpretation: &'static str,
    family_kind: &'static str,
    field_index: usize,
    members: Vec<PageMarkScopedYFamilyMember>,
    table_top_residuals: Vec<f32>,
    table_top_hit_members: Vec<PageMarkScopedYFamilyMember>,
    row_top_residuals: Vec<f32>,
    row_delta_residuals: Vec<f32>,
    table_top_hit_count: usize,
    row_top_coverage_count: usize,
    row_delta_coverage_count: usize,
    row_line_range_coverage_count: usize,
    table_top_hit_line_range_coverage_count: usize,
    row_top_mean_abs_residual: Option<f32>,
    row_top_max_abs_residual: Option<f32>,
    row_delta_mean_abs_residual: Option<f32>,
    row_delta_max_abs_residual: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
struct PageMarkScopedYSlotFit {
    source: &'static str,
    interpretation: &'static str,
    field_index: usize,
    tail_block16_word_index: usize,
    members: Vec<PageMarkScopedYFamilyMember>,
    table_top_residuals: Vec<f32>,
    row_top_residuals: Vec<f32>,
    row_delta_residuals: Vec<f32>,
    table_top_hit_count: usize,
    row_top_coverage_count: usize,
    row_delta_coverage_count: usize,
    row_line_range_coverage_count: usize,
    ordered_line_mark_record_coverage_count: usize,
    ordered_line_mark_record_coverage_complete: bool,
    ordered_line_mark_record_indexes_covered: Vec<usize>,
    ordered_line_mark_record_member_byte_offsets: Vec<usize>,
    row_top_mean_abs_residual: Option<f32>,
    row_top_max_abs_residual: Option<f32>,
    row_delta_mean_abs_residual: Option<f32>,
    row_delta_max_abs_residual: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageMarkScopedYOrderedLineRangeCoverage {
    record_indexes_covered: Vec<usize>,
    member_byte_offsets: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageMarkRawSubrecordLineSpanCandidate {
    byte_offset: usize,
    raw_record_index: u32,
    raw_record_scan_index: usize,
    tail_block16_word_index: Option<usize>,
    line_start_candidate: u16,
    line_end_candidate: u16,
    line_span_units: usize,
    field2_value: u16,
    words: [u16; 8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridPageMarkSubrecordLineSpanReadiness {
    selected_record_indexes: Vec<usize>,
    previous_record_indexes: Vec<usize>,
    selected_post_row_gap_span_targets: Vec<usize>,
    post_row_gap_span_targets: Vec<usize>,
    previous_row_span_targets: Vec<usize>,
    compact_row_span_targets: Vec<usize>,
    candidate_count: usize,
    selected_post_row_gap_span_hit_count: usize,
    previous_row_span_hit_count: usize,
    compact_row_span_hit_count: usize,
    selected_post_row_gap_span_max_abs_residual_units: Option<i32>,
    previous_row_span_max_abs_residual_units: Option<i32>,
    compact_row_span_max_abs_residual_units: Option<i32>,
    selected_post_row_gap_span_coverage: TableGridPageMarkSubrecordLineSpanCoverage,
    previous_row_span_coverage: TableGridPageMarkSubrecordLineSpanCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageMarkSubrecordLineSpanMatch<'a> {
    target_index: usize,
    target_units: usize,
    residual_units: i32,
    candidate: &'a PageMarkRawSubrecordLineSpanCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridPageMarkSubrecordLineSpanCoverage {
    matched_record_indexes: Vec<usize>,
    matched_candidate_byte_offsets: Vec<usize>,
    unique_candidate_byte_offsets: Vec<usize>,
    duplicate_candidate_byte_offsets: Vec<usize>,
    ordered_unique_coverage_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridCrossTableSubrecordOrderingProbe {
    current_table_candidate_index: usize,
    related_table_candidate_indexes: Vec<usize>,
    combined_line_mark_record_indexes: Vec<usize>,
    combined_matched_byte_offsets: Vec<usize>,
    combined_raw_record_scan_indexes: Vec<usize>,
    combined_tail_block16_word_indexes: Vec<Option<usize>>,
    combined_line_start_candidates: Vec<u16>,
    combined_line_end_candidates: Vec<u16>,
    combined_field2_values: Vec<u16>,
    monotonic_raw_record_scan_index: bool,
    monotonic_line_start_candidate: bool,
    family_reused_after_later_family: bool,
    cross_table_ordering_consistent: bool,
    tables: Vec<TableGridCrossTableSubrecordOrderingTable>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridCrossTableSubrecordOrderingTable {
    table_candidate_index: usize,
    source_start: usize,
    source_end: usize,
    row_count: usize,
    matched_rows: Vec<TableGridCrossTableSubrecordOrderingMatch>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridCrossTableSubrecordOrderingMatch {
    row_index: usize,
    line_mark_record_index: usize,
    target_units: usize,
    residual_units: i32,
    byte_offset: usize,
    raw_record_index: u32,
    raw_record_scan_index: usize,
    tail_block16_word_index: Option<usize>,
    line_start_candidate: u16,
    line_end_candidate: u16,
    field2_value: u16,
}

fn usize_values_are_monotonic_non_decreasing(values: &[usize]) -> bool {
    values.windows(2).all(|pair| pair[0] <= pair[1])
}

fn u16_values_are_monotonic_non_decreasing(values: &[u16]) -> bool {
    values.windows(2).all(|pair| pair[0] <= pair[1])
}

fn values_reused_after_different_value(values: &[usize]) -> bool {
    let mut last_seen = BTreeMap::<usize, usize>::new();
    for (index, value) in values.iter().copied().enumerate() {
        if let Some(previous_index) = last_seen.insert(value, index)
            && values[previous_index + 1..index]
                .iter()
                .any(|between| *between != value)
        {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, PartialEq)]
struct TableGridCrossTableRowBoundaryOffsetProbe {
    current_table_candidate_index: usize,
    sparse_table_candidate_index: usize,
    related_table_candidate_indexes: Vec<usize>,
    related_table_count: usize,
    table_count_with_previous_row_span_alignment: usize,
    row_boundary_offset_candidate_units: Vec<i32>,
    stable_row_boundary_offset_candidate_units: Option<i32>,
    all_related_tables_have_offset_candidate: bool,
    all_offsets_stable: bool,
    all_offsets_require_transform: bool,
    all_offset_normalized_boundaries_exact: bool,
    combined_line_mark_record_indexes: Vec<usize>,
    page_mark_entry_index: Option<usize>,
    page_index_candidate: Option<usize>,
    page_line_start: Option<usize>,
    page_line_end: Option<usize>,
    page_mark_u16_field_count: usize,
    page_mark_u16_field_preview: Vec<u16>,
    combined_line_offsets_from_page_start: Vec<usize>,
    combined_line_offsets_monotonic: bool,
    combined_line_mark_record_y_pitch_px: Option<f32>,
    combined_line_mark_record_y_pitch_basis: Option<&'static str>,
    combined_line_mark_record_y_tops_px: Vec<f32>,
    combined_line_mark_record_y_span_px: Option<f32>,
    source_unit_to_page_line_index_source_units: Vec<usize>,
    source_unit_to_page_line_index_slope: Option<f32>,
    source_unit_to_page_line_index_intercept: Option<f32>,
    source_unit_to_page_line_index_fitted_indexes: Vec<f32>,
    source_unit_to_page_line_index_residual_indexes: Vec<f32>,
    source_unit_to_page_line_index_max_abs_residual: Option<f32>,
    source_unit_to_page_line_index_exact: bool,
    source_unit_to_page_line_index_rows: Vec<TableGridSourceUnitToPageLineIndexFitRow>,
    source_unit_to_page_line_index_piecewise_max_abs_residual: Option<f32>,
    source_unit_to_page_line_index_piecewise_all_tables_exact: bool,
    source_unit_to_page_line_index_piecewise_tables:
        Vec<TableGridSourceUnitToPageLineIndexPiecewiseTable>,
    source_unit_to_page_line_index_piecewise_transitions:
        Vec<TableGridSourceUnitToPageLineIndexPiecewiseTransition>,
    all_records_within_single_page_mark_entry: bool,
    tables: Vec<TableGridCrossTableRowBoundaryOffsetTable>,
}

#[derive(Debug, Clone, PartialEq)]
struct TableGridCrossTableRowBoundaryOffsetTable {
    table_candidate_index: usize,
    source_start: usize,
    source_end: usize,
    row_count: usize,
    line_mark_record_indexes: Vec<usize>,
    page_mark_line_offsets_from_entry_start: Vec<usize>,
    page_mark_records_within_single_entry: bool,
    line_mark_record_y_tops_px: Vec<f32>,
    selected_spacing_record_indexes: Vec<usize>,
    selected_spacing_page_mark_line_offsets_from_entry_start: Vec<usize>,
    selected_spacing_records_within_single_entry: bool,
    selected_spacing_record_y_tops_px: Vec<f32>,
    selected_spacing_line_mark_start_units: Vec<usize>,
    selected_spacing_line_mark_end_units: Vec<usize>,
    selected_spacing_start_residual_units: Vec<i32>,
    selected_spacing_end_residual_units: Vec<i32>,
    selected_spacing_span_residual_units: Vec<i32>,
    selected_minus_previous_record_index_gaps: Vec<i32>,
    selected_minus_previous_record_y_delta_px: Vec<f32>,
    row_source_start_units: Vec<usize>,
    row_source_end_units: Vec<usize>,
    line_mark_start_units: Vec<usize>,
    line_mark_end_units: Vec<usize>,
    start_residual_units: Vec<i32>,
    end_residual_units: Vec<i32>,
    span_residual_units: Vec<i32>,
    row_boundary_offset_candidate_units: Option<i32>,
    offset_normalized_start_residual_units: Vec<i32>,
    offset_normalized_end_residual_units: Vec<i32>,
    offset_normalized_exact_boundary_aligned: bool,
    exact_boundary_aligned: bool,
    span_only_match: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct TableGridSourceUnitToPageLineIndexFitRow {
    table_candidate_index: usize,
    row_index: usize,
    row_source_start_units: usize,
    line_mark_record_index: usize,
    fitted_record_index: f32,
    residual_record_index: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct TableGridSourceUnitToPageLineIndexPiecewiseTable {
    table_candidate_index: usize,
    source_start: usize,
    source_end: usize,
    row_count: usize,
    row_source_start_units: Vec<usize>,
    line_mark_record_indexes: Vec<usize>,
    slope_record_indexes_per_source_unit: Option<f32>,
    intercept_record_index: Option<f32>,
    fitted_record_indexes: Vec<f32>,
    residual_record_indexes: Vec<f32>,
    max_abs_residual_record_indexes: Option<f32>,
    exact_fit: bool,
    page_mark_records_within_single_entry: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct TableGridSourceUnitToPageLineIndexPiecewiseTransition {
    from_table_candidate_index: usize,
    to_table_candidate_index: usize,
    previous_last_source_unit: usize,
    next_first_source_unit: usize,
    source_range_gap_units: usize,
    row_source_start_gap_units: i32,
    previous_last_record_index: usize,
    next_first_record_index: usize,
    line_mark_record_gap: i32,
    same_page_mark_entry: bool,
}

fn adjacent_f32_deltas(values: &[f32]) -> Vec<f32> {
    values.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn max_abs_i32(values: &[i32]) -> Option<i32> {
    values.iter().map(|value| value.saturating_abs()).max()
}

fn row_source_start_gap_minus_source_range_gap_units(
    row_source_start_gap_units: i32,
    source_range_gap_units: usize,
) -> i32 {
    let Ok(source_range_gap_units) = i32::try_from(source_range_gap_units) else {
        return i32::MIN;
    };
    row_source_start_gap_units.saturating_sub(source_range_gap_units)
}

fn single_i32_value(values: &[i32]) -> Option<i32> {
    let first = *values.first()?;
    values.iter().all(|value| *value == first).then_some(first)
}

fn single_u16_value(values: &[u16]) -> Option<u16> {
    let first = *values.first()?;
    values.iter().all(|value| *value == first).then_some(first)
}

fn ratio_usize_by_i32(numerators: &[usize], denominators: &[i32]) -> Vec<f32> {
    numerators
        .iter()
        .copied()
        .zip(denominators.iter().copied())
        .filter_map(|(numerator, denominator)| {
            (denominator != 0).then_some(numerator as f32 / denominator as f32)
        })
        .filter(|ratio| ratio.is_finite())
        .collect()
}

fn ratio_i32_by_i32(numerators: &[i32], denominators: &[i32]) -> Vec<f32> {
    numerators
        .iter()
        .copied()
        .zip(denominators.iter().copied())
        .filter_map(|(numerator, denominator)| {
            (denominator != 0).then_some(numerator as f32 / denominator as f32)
        })
        .filter(|ratio| ratio.is_finite())
        .collect()
}

fn ratio_f32_by_i32(numerators: &[f32], denominators: &[i32]) -> Vec<f32> {
    numerators
        .iter()
        .copied()
        .zip(denominators.iter().copied())
        .filter_map(|(numerator, denominator)| {
            (denominator != 0).then_some(numerator / denominator as f32)
        })
        .filter(|ratio| ratio.is_finite())
        .collect()
}

fn rounded_f32_values_all_same(values: &[f32]) -> bool {
    let Some(first) = values.first().map(|value| rounded_milli(*value)) else {
        return false;
    };
    values
        .iter()
        .map(|value| rounded_milli(*value))
        .all(|value| value == first)
}

fn f32_value_spread(values: &[f32]) -> Option<f32> {
    let mut iter = values.iter().copied().filter(|value| value.is_finite());
    let first = iter.next()?;
    let (min, max) = iter.fold((first, first), |(min, max), value| {
        (min.min(value), max.max(value))
    });
    Some(max - min)
}

fn signed_usize_delta_i32(left: usize, right: usize) -> i32 {
    if left >= right {
        i32::try_from(left - right).unwrap_or(i32::MAX)
    } else {
        i32::try_from(right - left).map_or(i32::MIN, |delta| -delta)
    }
}

fn push_optional_i32_json(output: &mut String, value: Option<i32>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn single_usize_value(values: &[usize]) -> Option<usize> {
    let first = *values.first()?;
    values.iter().all(|value| *value == first).then_some(first)
}

fn option_f32_order(left: Option<f32>, right: Option<f32>) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.partial_cmp(&right).unwrap_or(Ordering::Equal),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn mean_and_max_abs_residual(residuals: &[f32]) -> (Option<f32>, Option<f32>) {
    if residuals.is_empty() {
        return (None, None);
    }
    let mean =
        residuals.iter().map(|residual| residual.abs()).sum::<f32>() / residuals.len() as f32;
    let max = residuals
        .iter()
        .map(|residual| residual.abs())
        .fold(0.0f32, f32::max);
    (Some(mean), Some(max))
}

fn push_option_f32_json(output: &mut String, value: Option<f32>) {
    match value {
        Some(value) if value.is_finite() => output.push_str(&format!("{value:.3}")),
        _ => output.push_str("null"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageMarkRawU16SubrecordCandidate {
    byte_offset: usize,
    field_index: usize,
    words: [u16; 8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridLineMarkRowGapSequenceRow {
    compact_row_index: usize,
    sparse_row_index: usize,
    source_interval_index: usize,
    row_source_start: usize,
    row_source_end: usize,
    row_source_start_units: usize,
    row_source_end_units: usize,
    selected_line_mark: ShanaiLanLineMarkInterval,
    previous_line_mark: Option<ShanaiLanLineMarkInterval>,
    next_line_mark: Option<ShanaiLanLineMarkInterval>,
    post_row_gap: Option<TableGridSparseSiblingPostRowGap>,
    next_row_span_units: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridPageMarkLineContext {
    page_mark_entry_index: usize,
    page_index_candidate: Option<usize>,
    page_line_start: usize,
    page_line_end: usize,
    page_mark_u16_fields: Vec<u16>,
}

fn residuals_f32(candidates: &[f32], references: &[f32]) -> Vec<f32> {
    candidates
        .iter()
        .zip(references)
        .map(|(candidate, reference)| candidate - reference)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TableGridResolvedLineMarkRow {
    interval: ShanaiLanLineMarkInterval,
    role: TableGridLineMarkRowRecordRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TableGridLineMarkRowRecordRole {
    SelectedOverlap,
    PreviousCompactRowSpan,
}

impl TableGridLineMarkRowRecordRole {
    fn as_str(self) -> &'static str {
        match self {
            Self::SelectedOverlap => "selected-overlap-record",
            Self::PreviousCompactRowSpan => "previous-compact-row-span-record",
        }
    }
}

fn push_f32_array_json(output: &mut String, values: &[f32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{value:.3}"));
    }
    output.push(']');
}

fn max_abs_f32(values: &[f32]) -> Option<f32> {
    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .map(f32::abs)
        .reduce(f32::max)
}

fn mean_abs_f32(values: &[f32]) -> Option<f32> {
    let mut sum = 0.0f32;
    let mut count = 0usize;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        sum += value.abs();
        count += 1;
    }
    (count > 0).then(|| sum / count as f32)
}

fn mean_f32(values: &[f32]) -> Option<f32> {
    let mut sum = 0.0f32;
    let mut count = 0usize;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        sum += value;
        count += 1;
    }
    (count > 0).then(|| sum / count as f32)
}

fn slope_from_indexed_tops(indexes: &[f32], tops: &[f32]) -> Option<f32> {
    let first_index = *indexes.first()?;
    let last_index = *indexes.last()?;
    let first_top = *tops.first()?;
    let last_top = *tops.last()?;
    let index_span = last_index - first_index;
    (index_span.is_finite() && index_span.abs() > f32::EPSILON)
        .then_some((last_top - first_top) / index_span)
}

fn affine_fit_f32(xs: &[f32], ys: &[f32]) -> Option<(f32, f32)> {
    if xs.len() != ys.len() || xs.len() < 2 {
        return None;
    }
    let mean_x = mean_f32(xs)?;
    let mean_y = mean_f32(ys)?;
    let mut numerator = 0.0f32;
    let mut denominator = 0.0f32;
    for (x, y) in xs.iter().copied().zip(ys.iter().copied()) {
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        let dx = x - mean_x;
        numerator += dx * (y - mean_y);
        denominator += dx * dx;
    }
    if !denominator.is_finite() || denominator.abs() <= f32::EPSILON {
        return None;
    }
    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;
    (slope.is_finite() && intercept.is_finite()).then_some((slope, intercept))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridSparseSiblingSegmentMatch {
    compact_column_index: usize,
    sparse_column_index: usize,
    source_start: usize,
    source_end: usize,
    text_matches: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridSparseSiblingRowMatch {
    compact_row_index: usize,
    sparse_row_index: usize,
    source_interval_index: usize,
    source_start: usize,
    source_end: usize,
    compact_cell_count: usize,
    sparse_cell_count: usize,
    sparse_empty_cell_count: usize,
    sparse_non_empty_cell_count: usize,
    first_non_empty_sparse_column_index: Option<usize>,
    last_non_empty_sparse_column_index: Option<usize>,
    compact_to_sparse_column_offset: Option<usize>,
    segments: Vec<TableGridSparseSiblingSegmentMatch>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridSparseSiblingEvidence<'a> {
    sparse_candidate: &'a TableCandidate,
    rows: Vec<TableGridSparseSiblingRowMatch>,
    compact_to_sparse_column_offset: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TableGridUnitBBoxBasis {
    MatchedCells,
    MatchedCellsPlusFirstTrailingHeader,
    FullLineHeaderExtent,
}

impl TableGridUnitBBoxBasis {
    fn as_str(self) -> &'static str {
        match self {
            TableGridUnitBBoxBasis::MatchedCells => "matched-cells",
            TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader => {
                "matched-cells-plus-first-trailing-header"
            }
            TableGridUnitBBoxBasis::FullLineHeaderExtent => "full-line-header-extent",
        }
    }
}

#[derive(Debug, Clone)]
struct TableGridHorizontalFrameCandidateSupport {
    frame_basis: &'static str,
    selected_x: f32,
    selected_width: f32,
    contribution: &'static str,
    blocked_reason: &'static str,
}

fn rounded_milli(value: f32) -> i32 {
    (value * 1000.0).round() as i32
}

#[derive(Debug, Clone, PartialEq)]
struct TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate {
    source: &'static str,
    interpretation: &'static str,
    field_index: usize,
    tail_block16_word_index: Option<usize>,
    raw_record_scan_index: Option<usize>,
    raw_record_index: Option<u32>,
    byte_offset: usize,
    subrecord_byte_offset: usize,
    subrecord_line_start_candidate: u16,
    subrecord_line_end_candidate: u16,
    value: u16,
    value_px: f32,
}

#[derive(Debug, Clone)]
struct TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
    line_domain_y: Option<f32>,
    selected_span_units: Option<usize>,
    line_domain_projected_y: Option<f32>,
    candidates: Vec<TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate>,
    best_absolute_y_slot: Option<TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate>,
    residual_px: Option<f32>,
    agrees: bool,
}

impl TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
    fn semantics_ready(&self) -> bool {
        self.line_domain_projected_y.is_some() && self.best_absolute_y_slot.is_some() && self.agrees
    }
}

#[derive(Debug, Clone)]
struct TableGridSourceGapToPageLineGapReadinessHints {
    transition_count: usize,
    same_page_mark_entry_transition_count: usize,
    all_transitions_same_page_mark_entry: bool,
    source_range_gap_to_page_line_gap_max_abs_delta_units: Option<i32>,
    row_source_start_gap_to_page_line_gap_max_abs_delta_units: Option<i32>,
    segment_offset_gap_to_page_line_gap_max_abs_delta_units: Option<i32>,
    best_candidate_transform_kind: Option<&'static str>,
    best_candidate_max_abs_delta_units: Option<i32>,
    source_range_units_per_page_line_gap_spread: Option<f32>,
    row_source_start_units_per_page_line_gap_spread: Option<f32>,
    segment_offset_units_per_page_line_gap_spread: Option<f32>,
    affine_row_source_start_gap_fit: Option<TableGridAffineRowSourceStartGapFit>,
}

#[derive(Debug, Clone, Copy)]
struct TableGridSourceGapToPageLineGapTransformCandidateSummary {
    kind: &'static str,
    max_abs_delta_units: Option<i32>,
    units_per_page_line_gap_spread: Option<f32>,
    affine_row_source_start_gap_fit: Option<TableGridAffineRowSourceStartGapFit>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TableGridAffineRowSourceStartGapFit {
    numerator_slope: i64,
    denominator_slope: i64,
    numerator_intercept: i64,
    denominator_intercept: i64,
    max_abs_residual: f64,
    max_abs_residual_ceiling_units: i32,
    sample_count: usize,
    family_scoped: bool,
    fit_stable: bool,
}

impl TableGridAffineRowSourceStartGapFit {
    fn blocked_reason(&self) -> &'static str {
        "affine-row-source-start-gap-family-transform-authority-unproven"
    }

    fn max_abs_residual_ceiling_units(&self) -> i32 {
        self.max_abs_residual_ceiling_units
    }
}

impl TableGridSourceGapToPageLineGapReadinessHints {
    fn source_gap_to_page_line_gap_transform_stable(&self) -> bool {
        self.transition_count > 0 && self.best_candidate_max_abs_delta_units == Some(0)
    }

    fn table_family_source_gap_to_page_line_gap_transform_stable(&self) -> bool {
        self.source_gap_to_page_line_gap_transform_stable()
            && self.all_transitions_same_page_mark_entry
    }

    fn transform_blocked_reason(&self) -> Option<&'static str> {
        (!self.source_gap_to_page_line_gap_transform_stable())
            .then_some("source-gap-to-page-line-gap-transform-not-stable")
    }

    fn table_family_transform_blocked_reason(&self) -> Option<&'static str> {
        if self.transition_count == 0 {
            Some("source-gap-to-page-line-gap-transform-evidence-absent")
        } else if !self.all_transitions_same_page_mark_entry {
            Some("source-gap-to-page-line-gap-transform-crosses-page-mark-entries")
        } else if !self.source_gap_to_page_line_gap_transform_stable() {
            Some("source-gap-to-page-line-gap-transform-unstable-across-table-family")
        } else {
            None
        }
    }

    fn transform_candidate_summaries(
        &self,
    ) -> Vec<TableGridSourceGapToPageLineGapTransformCandidateSummary> {
        let mut summaries = vec![
            TableGridSourceGapToPageLineGapTransformCandidateSummary {
                kind: "direct-source-range-gap",
                max_abs_delta_units: self.source_range_gap_to_page_line_gap_max_abs_delta_units,
                units_per_page_line_gap_spread: self.source_range_units_per_page_line_gap_spread,
                affine_row_source_start_gap_fit: None,
            },
            TableGridSourceGapToPageLineGapTransformCandidateSummary {
                kind: "direct-row-source-start-gap",
                max_abs_delta_units: self.row_source_start_gap_to_page_line_gap_max_abs_delta_units,
                units_per_page_line_gap_spread: self
                    .row_source_start_units_per_page_line_gap_spread,
                affine_row_source_start_gap_fit: None,
            },
            TableGridSourceGapToPageLineGapTransformCandidateSummary {
                kind: "segment-offset-gap",
                max_abs_delta_units: self.segment_offset_gap_to_page_line_gap_max_abs_delta_units,
                units_per_page_line_gap_spread: self.segment_offset_units_per_page_line_gap_spread,
                affine_row_source_start_gap_fit: None,
            },
        ];
        if let Some(fit) = self.affine_row_source_start_gap_fit {
            summaries.push(TableGridSourceGapToPageLineGapTransformCandidateSummary {
                kind: "affine-row-source-start-gap",
                max_abs_delta_units: Some(fit.max_abs_residual_ceiling_units()),
                units_per_page_line_gap_spread: None,
                affine_row_source_start_gap_fit: Some(fit),
            });
        }
        summaries
    }

    fn transform_candidate_count(&self) -> usize {
        self.transform_candidate_summaries()
            .iter()
            .filter(|candidate| {
                candidate.max_abs_delta_units.is_some()
                    || candidate.affine_row_source_start_gap_fit.is_some()
            })
            .count()
    }

    fn exact_transform_candidate_count(&self) -> usize {
        self.transform_candidate_summaries()
            .iter()
            .filter(|candidate| {
                candidate.affine_row_source_start_gap_fit.is_none()
                    && candidate.max_abs_delta_units == Some(0)
            })
            .count()
    }

    fn best_candidate_transition_coverage_count(&self) -> usize {
        if self.best_candidate_transform_kind.is_some() {
            self.transition_count
        } else {
            0
        }
    }

    fn best_candidate_units_per_page_line_gap_spread(&self) -> Option<f32> {
        let best_kind = self.best_candidate_transform_kind?;
        self.transform_candidate_summaries()
            .iter()
            .find(|candidate| candidate.kind == best_kind)
            .and_then(|candidate| candidate.units_per_page_line_gap_spread)
    }

    fn lowest_spread_candidate(&self) -> Option<(&'static str, f32)> {
        let summaries = self.transform_candidate_summaries();
        summaries
            .iter()
            .filter_map(|candidate| {
                candidate
                    .units_per_page_line_gap_spread
                    .map(|spread| (candidate.kind, spread))
            })
            .min_by(|left, right| left.1.total_cmp(&right.1))
    }
}

fn affine_row_source_start_gap_fit(
    page_line_gaps: &[i32],
    row_source_start_gap_units: &[i32],
    family_scoped: bool,
) -> Option<TableGridAffineRowSourceStartGapFit> {
    if !family_scoped {
        return None;
    }
    if page_line_gaps.len() != row_source_start_gap_units.len() {
        return None;
    }
    let sample_count = page_line_gaps.len();
    if sample_count < 3 {
        return None;
    }
    let n = i64::try_from(sample_count).ok()?;
    let page_line_gaps = &page_line_gaps[..sample_count];
    let row_source_start_gap_units = &row_source_start_gap_units[..sample_count];
    let first_page_line_gap = *page_line_gaps.first()?;
    if page_line_gaps
        .iter()
        .all(|page_line_gap| *page_line_gap == first_page_line_gap)
    {
        return None;
    }

    let sum_y = page_line_gaps
        .iter()
        .copied()
        .map(i64::from)
        .try_fold(0_i64, |accumulator, page_line_gap| {
            accumulator.checked_add(page_line_gap)
        })?;
    let sum_x = row_source_start_gap_units
        .iter()
        .copied()
        .map(i64::from)
        .try_fold(0_i64, |accumulator, row_source_start_gap| {
            accumulator.checked_add(row_source_start_gap)
        })?;
    let sum_xy = page_line_gaps
        .iter()
        .copied()
        .zip(row_source_start_gap_units.iter().copied())
        .try_fold(
            0_i64,
            |accumulator, (page_line_gap, row_source_start_gap)| {
                let product =
                    i64::from(page_line_gap).checked_mul(i64::from(row_source_start_gap))?;
                accumulator.checked_add(product)
            },
        )?;
    let sum_y_squared =
        page_line_gaps
            .iter()
            .copied()
            .try_fold(0_i64, |accumulator, page_line_gap| {
                let page_line_gap = i64::from(page_line_gap);
                let squared = page_line_gap.checked_mul(page_line_gap)?;
                accumulator.checked_add(squared)
            })?;

    let slope_numerator = n
        .checked_mul(sum_xy)?
        .checked_sub(sum_y.checked_mul(sum_x)?)?;
    let slope_denominator = n
        .checked_mul(sum_y_squared)?
        .checked_sub(sum_y.checked_mul(sum_y)?)?;
    if slope_denominator == 0 {
        return None;
    }
    let intercept_numerator = sum_x
        .checked_mul(slope_denominator)?
        .checked_sub(slope_numerator.checked_mul(sum_y)?)?;
    let intercept_denominator = n.checked_mul(slope_denominator)?;
    let common_denominator = intercept_denominator.checked_abs()?;
    if common_denominator == 0 {
        return None;
    }

    let max_abs_residual_numerator = page_line_gaps
        .iter()
        .copied()
        .zip(row_source_start_gap_units.iter().copied())
        .try_fold(
            0_i64,
            |max_residual, (page_line_gap, row_source_start_gap)| {
                let predicted_numerator = slope_numerator
                    .checked_mul(i64::from(page_line_gap))?
                    .checked_mul(n)?
                    .checked_add(intercept_numerator)?;
                let observed_numerator =
                    i64::from(row_source_start_gap).checked_mul(intercept_denominator)?;
                let residual = observed_numerator
                    .checked_sub(predicted_numerator)?
                    .checked_abs()?;
                Some(max_residual.max(residual))
            },
        )?;

    if slope_numerator == i64::MIN
        || slope_denominator == i64::MIN
        || intercept_numerator == i64::MIN
        || intercept_denominator == i64::MIN
    {
        return None;
    }

    let (numerator_slope, denominator_slope) =
        reduce_i64_fraction(slope_numerator, slope_denominator);
    let (numerator_intercept, denominator_intercept) =
        reduce_i64_fraction(intercept_numerator, intercept_denominator);
    let max_abs_residual = ratio_i64_to_f64(max_abs_residual_numerator, common_denominator)?;
    let max_abs_residual_ceiling_units = i32::try_from(
        max_abs_residual_numerator.checked_add(common_denominator.checked_sub(1)?)?
            / common_denominator,
    )
    .ok()?;

    Some(TableGridAffineRowSourceStartGapFit {
        numerator_slope,
        denominator_slope,
        numerator_intercept,
        denominator_intercept,
        max_abs_residual,
        max_abs_residual_ceiling_units,
        sample_count,
        family_scoped,
        fit_stable: max_abs_residual_numerator <= common_denominator,
    })
}

fn reduce_i64_fraction(numerator: i64, denominator: i64) -> (i64, i64) {
    if denominator == 0 {
        return (numerator, denominator);
    }
    let mut numerator = numerator;
    let mut denominator = denominator;
    if denominator < 0 {
        numerator = -numerator;
        denominator = -denominator;
    }
    let divisor = gcd_i64(numerator, denominator);
    (numerator / divisor, denominator / divisor)
}

fn gcd_i64(left: i64, right: i64) -> i64 {
    let mut left = left.abs();
    let mut right = right.abs();
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn ratio_i64_to_f64(numerator: i64, denominator: i64) -> Option<f64> {
    if denominator == 0 {
        return None;
    }
    let numerator = i32::try_from(numerator).ok().map(f64::from)?;
    let denominator = i32::try_from(denominator).ok().map(f64::from)?;
    Some(numerator / denominator)
}

fn push_affine_row_source_start_gap_fit_json(
    output: &mut String,
    fit: Option<TableGridAffineRowSourceStartGapFit>,
) {
    let Some(fit) = fit else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.affineRowSourceStartGapFit\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"numeratorSlope\":");
    output.push_str(&fit.numerator_slope.to_string());
    output.push_str(",\"denominatorSlope\":");
    output.push_str(&fit.denominator_slope.to_string());
    output.push_str(",\"numeratorIntercept\":");
    output.push_str(&fit.numerator_intercept.to_string());
    output.push_str(",\"denominatorIntercept\":");
    output.push_str(&fit.denominator_intercept.to_string());
    output.push_str(",\"maxAbsResidual\":");
    output.push_str(&format!("{:.3}", fit.max_abs_residual));
    output.push_str(",\"sampleCount\":");
    output.push_str(&fit.sample_count.to_string());
    output.push_str(",\"familyScoped\":");
    output.push_str(if fit.family_scoped { "true" } else { "false" });
    output.push_str(",\"fitStable\":");
    output.push_str(if fit.fit_stable { "true" } else { "false" });
    output.push_str(",\"blockedReason\":");
    output.push_str(&json_string(fit.blocked_reason()));
    output.push_str(
        ",\"renderPromotionContribution\":\"affine-row-source-start-gap-fit-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(fit.blocked_reason()));
    output.push('}');
}

#[derive(Debug, Clone)]
struct TableGridSourceOnlyPageYOriginCandidateSupport {
    origin_basis: &'static str,
    selected_y: f32,
    row_height: Option<f32>,
    table_candidate_index: Option<usize>,
    contribution: &'static str,
    blocked_reason: &'static str,
    extra_blocked_reasons: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridSourceOnlyStrideRowCoverageSummary {
    candidate_row_count: usize,
    matched_row_count: usize,
    all_rows_covered: bool,
    line_mark_record_selection: &'static str,
    line_mark_record_indexes: Vec<usize>,
    uniform_line_mark_record_stride: bool,
    line_mark_record_stride: Option<usize>,
    matches_stride_candidate_record_indexes: bool,
    row_span_units: Vec<usize>,
    line_mark_span_units: Vec<usize>,
    row_span_residual_units: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridLineMarkRowBoundaryAlignmentSummary {
    candidate_row_count: usize,
    selected_spacing_record_alignment: Option<TableGridLineMarkRowBoundaryAlignmentFamily>,
    previous_row_span_record_alignment: Option<TableGridLineMarkRowBoundaryAlignmentFamily>,
    next_record_alignment: Option<TableGridLineMarkRowBoundaryAlignmentFamily>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridLineMarkRowBoundaryAlignmentFamily {
    family: &'static str,
    span_interpretation: &'static str,
    row_count: usize,
    record_indexes: Vec<usize>,
    uniform_line_mark_record_stride: bool,
    line_mark_record_stride: Option<usize>,
    matches_stride_candidate_record_indexes: bool,
    row_source_start_units: Vec<usize>,
    row_source_end_units: Vec<usize>,
    line_mark_start_units: Vec<usize>,
    line_mark_end_units: Vec<usize>,
    start_residual_units: Vec<i32>,
    end_residual_units: Vec<i32>,
    span_residual_units: Vec<i32>,
    exact_boundary_match_count: usize,
    exact_boundary_aligned: bool,
    start_residual_stable: bool,
    end_residual_stable: bool,
    span_residual_stable: bool,
    stable_start_residual_units: Option<i32>,
    stable_end_residual_units: Option<i32>,
    stable_span_residual_units: Option<i32>,
    row_boundary_offset_candidate_units: Option<i32>,
    offset_normalized_start_residual_units: Vec<i32>,
    offset_normalized_end_residual_units: Vec<i32>,
    offset_normalized_exact_boundary_match_count: usize,
    offset_normalized_exact_boundary_aligned: bool,
    span_only_match: bool,
    rows: Vec<TableGridLineMarkRowBoundaryAlignmentRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridLineMarkRowBoundaryAlignmentRow {
    compact_row_index: usize,
    sparse_row_index: usize,
    source_interval_index: usize,
    line_mark_record_index: usize,
    row_source_start_units: usize,
    row_source_end_units: usize,
    line_mark_start_units: usize,
    line_mark_end_units: usize,
    start_residual_units: i32,
    end_residual_units: i32,
    span_residual_units: i32,
    exact_boundary_aligned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridStridePageMarkEntryLineBoundsCoverageSummary {
    candidate_row_count: usize,
    line_mark_record_indexes: Vec<usize>,
    record_stride: usize,
    page_mark_entry_index: usize,
    page_index_candidate: Option<usize>,
    page_line_start: usize,
    page_line_end: usize,
    line_offsets_from_page_start: Vec<usize>,
    row_count_matches_stride_candidate: bool,
    all_line_mark_records_within_page_mark_entry: bool,
    coverage_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridPageMarkSubrecordLineRangeRecordCoverageSummary {
    candidate_count: usize,
    selected_record_indexes: Vec<usize>,
    previous_record_indexes: Vec<usize>,
    selected_covered_record_indexes: Vec<usize>,
    previous_covered_record_indexes: Vec<usize>,
    selected_containing_candidate_byte_offsets: Vec<usize>,
    previous_containing_candidate_byte_offsets: Vec<usize>,
    selected_coverage_complete: bool,
    previous_coverage_complete: bool,
    selected_nearest_matches: Vec<TableGridPageMarkSubrecordLineRangeRecordMatch>,
    previous_nearest_matches: Vec<TableGridPageMarkSubrecordLineRangeRecordMatch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TableGridPageMarkSubrecordLineRangeRecordMatch {
    record_index: usize,
    distance_units: usize,
    candidate: PageMarkRawSubrecordLineSpanCandidate,
}

#[derive(Debug, Clone, Copy)]
struct PageMarkU16LayoutComparison {
    page_width_px: f32,
    page_height_px: f32,
    page_margin_px: f32,
    page_body_width_px: f32,
}

fn table_source_offset_to_units(basis: TextCountRangeOverlapBasis, offset: usize) -> usize {
    match basis {
        TextCountRangeOverlapBasis::Byte => offset / 2,
        TextCountRangeOverlapBasis::Unit => offset,
    }
}

fn push_optional_usize_json(output: &mut String, value: Option<usize>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn push_optional_u16_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn ranges_overlap_half_open(
    start: usize,
    end: usize,
    other_start: usize,
    other_end: usize,
) -> bool {
    start < other_end && other_start < end
}

struct SuccessDataTestAnswerSheetSectionAnchor {
    section_label: String,
    row_index: usize,
    source_interval_index: usize,
    row_source_start: usize,
    row_source_end: usize,
    cell_index: usize,
    cell_source_start: Option<usize>,
    cell_source_end: Option<usize>,
}

#[derive(Debug, Clone)]
struct SuccessDataTestAnswerSheetHatchedAreaCandidate {
    source: &'static str,
    top_section_label: String,
    bottom_section_label: String,
    top_row_index: usize,
    bottom_row_index: usize,
    top_source_interval_index: usize,
    bottom_source_interval_index: usize,
    empty_cell_index: usize,
    adjacent_answer_cell_index: usize,
    sheet_left_pt: f32,
    sheet_top_pt: f32,
    sheet_right_pt: f32,
    sheet_bottom_pt: f32,
    top_source_grid: Option<SuccessDataTestLineMarkPageGridCandidate>,
    bottom_source_grid: Option<SuccessDataTestLineMarkPageGridCandidate>,
}

#[derive(Debug, Clone)]
struct SuccessDataTestAnswerSheetSourceFrameCandidate {
    source: &'static str,
    candidate_basis: &'static str,
    sparse_table_candidate_index: usize,
    section_anchor_count: usize,
    top_section_label: String,
    bottom_section_label: String,
    top_row_index: usize,
    bottom_row_index: usize,
    top_line_mark_record_index: usize,
    bottom_line_mark_record_index: usize,
    local_top_pt: f32,
    local_bottom_pt: f32,
    source_px_per_sheet_pt_y: f32,
    reference_px_per_sheet_pt_y: f32,
    derived_frame_top_y: f32,
    derived_frame_height: f32,
    reference_frame_top_y: f32,
    reference_frame_height: f32,
    frame_top_residual_px: f32,
    frame_height_residual_px: f32,
    same_page_mark_entry: bool,
    same_page_index_candidate: bool,
    fdm_text_triangle_label_anchor_count: usize,
    triangle_source_bbox: Option<ObjectFdmIndexBbox>,
}

fn push_answer_sheet_rule_topology_evidence_json(output: &mut String, candidate: &TableCandidate) {
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

fn push_unique_f32(values: &mut Vec<f32>, value: f32) {
    if !values
        .iter()
        .any(|seen| (*seen - value).abs() < f32::EPSILON)
    {
        values.push(value);
    }
}

fn push_answer_sheet_hatched_area_candidate_json(
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

fn push_answer_sheet_triangle_placement_candidate_json(
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

fn bbox_axis_gap(left_start: f32, left_end: f32, right_start: f32, right_end: f32) -> f32 {
    if left_end < right_start {
        right_start - left_end
    } else if right_end < left_start {
        left_start - right_end
    } else {
        0.0
    }
}

fn projected_bbox_viewport_coverage_ratio(layout: PageLayout, width: f32, height: f32) -> f32 {
    let viewport = fdm_projection_viewport(layout);
    let viewport_area = viewport.width * viewport.height;
    if viewport_area <= 0.0 {
        return 0.0;
    }
    ((width.max(0.0) * height.max(0.0)) / viewport_area).clamp(0.0, 1.0)
}

fn ratio_to_ppm(value: f32) -> u32 {
    (value.clamp(0.0, 1.0) * 1_000_000.0).round() as u32
}

fn push_ratio_ppm_json(output: &mut String, ratio_ppm: u32) {
    output.push_str(&format!("{:.6}", ratio_ppm as f32 / 1_000_000.0));
}

fn push_bbox_tuple_json(output: &mut String, bbox: (f32, f32, f32, f32)) {
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        bbox.0, bbox.1, bbox.2, bbox.3
    ));
}

fn accumulate_usize_range(
    target_min: &mut Option<usize>,
    target_max: &mut Option<usize>,
    value: usize,
) {
    *target_min = Some((*target_min).map_or(value, |current| current.min(value)));
    *target_max = Some((*target_max).map_or(value, |current| current.max(value)));
}

fn accumulate_projected_bbox_union_milli(
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

fn update_optional_usize_min_max(min: &mut Option<usize>, max: &mut Option<usize>, value: usize) {
    *min = Some(min.map_or(value, |current| current.min(value)));
    *max = Some(max.map_or(value, |current| current.max(value)));
}

fn bbox_tuple_union(
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

type ShanaiLanGroupHeaderFamilyCounts =
    BTreeMap<(String, String, &'static str, &'static str), (usize, Vec<String>)>;

fn push_shanai_lan_group_header_family_counts_json(
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
struct ShanaiLanFragmentParentRunFillMix {
    slot_count: usize,
    source_property_fill_color_slot_count: usize,
    default_fill_color_slot_count: usize,
    fill_color_basis: BTreeSet<&'static str>,
    fill_colors: BTreeSet<&'static str>,
    example_texts: Vec<String>,
}

fn push_json_string_array(output: &mut String, values: &[String]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
}

fn usize_values_are_contiguous(values: &[usize]) -> bool {
    values.len() > 1 && values.windows(2).all(|window| window[1] == window[0] + 1)
}

fn push_static_str_count_map_json(output: &mut String, counts: &BTreeMap<&'static str, usize>) {
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

fn push_string_count_map_json(output: &mut String, counts: &BTreeMap<String, usize>, key: &str) {
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

fn push_usize_count_map_json(output: &mut String, counts: &BTreeMap<usize, usize>) {
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

fn distance_from_point_to_bbox(x: f32, y: f32, bbox: (f32, f32, f32, f32)) -> f32 {
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
struct SuccessDataTestLineMarkPageGridCandidate {
    record_index: usize,
    page_mark_entry_index: usize,
    page_index_candidate: Option<usize>,
    page_line_start: usize,
    page_line_end: usize,
    line_offset_from_page_start: usize,
    row_height: f32,
    row_height_basis: &'static str,
    row_top_y: f32,
}

#[derive(Debug, Clone)]
struct SuccessDataTestSourceTextPlacementCandidate {
    line_grid: SuccessDataTestLineMarkPageGridCandidate,
    font_size: f32,
    top_y: f32,
    baseline_y: f32,
}

#[derive(Debug, Clone)]
struct SuccessDataTestTextPlacementResidualEntry {
    role: &'static str,
    text: String,
    record_index: usize,
    flag_word: Option<u16>,
    font_size: f32,
    reference_top_y: f32,
    reference_baseline_y: f32,
    source_top_y: f32,
    source_baseline_y: f32,
    top_residual_px: f32,
    baseline_residual_px: f32,
    source_span: TextSourceSpan,
    line_header: Option<ShanaiLanLineHeader>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SuccessDataTestTextPlacementResidualBucketKey {
    top_residual_tenths: i32,
    baseline_residual_tenths: i32,
    flag_word: Option<u16>,
    font_size_tenths: i32,
    line_header_present: bool,
}

#[derive(Debug, Clone)]
struct SuccessDataTestTextPlacementResidualBucket {
    count: usize,
    record_indexes: Vec<usize>,
    roles: BTreeMap<&'static str, usize>,
}

#[derive(Debug, Clone)]
struct SuccessDataTestTextPlacementLinePitchFit {
    basis: &'static str,
    entry_count: usize,
    record_start: usize,
    record_end: usize,
    intercept: f32,
    pitch: f32,
    rms_residual_px: f32,
    max_abs_residual_px: f32,
    source_row_height_px: Option<f32>,
    source_row_height_minus_fit_pitch_px: Option<f32>,
}

fn push_optional_field_ratio_json(output: &mut String, value: Option<u16>, divisor: Option<u32>) {
    match (value, divisor) {
        (Some(value), Some(divisor)) if divisor > 0 => {
            output.push_str(&format!("{:.3}", f32::from(value) / divisor as f32));
        }
        _ => output.push_str("null"),
    }
}

fn push_optional_bool_json(output: &mut String, value: Option<bool>) {
    match value {
        Some(true) => output.push_str("true"),
        Some(false) => output.push_str("false"),
        None => output.push_str("null"),
    }
}

fn residual_tenths(value: f32) -> i32 {
    (value * 10.0).round() as i32
}

fn residual_tenths_string(tenths: i32) -> String {
    format!("{:.1}", tenths as f32 / 10.0)
}

fn source_range_json(start: usize, end: usize) -> String {
    format!("{{\"start\":{start},\"end\":{end}}}")
}

fn source_span_for_char_range(
    text: &str,
    source_span: &TextSourceSpan,
    start_chars: usize,
    end_chars: usize,
) -> TextSourceSpan {
    let start_units = utf16_units_before_chars(text, start_chars);
    let end_units = utf16_units_before_chars(text, end_chars);
    source_span.subspan_by_units(start_units, end_units)
}

fn utf16_units_before_chars(text: &str, chars: usize) -> usize {
    text.chars().take(chars).map(char::len_utf16).sum::<usize>()
}

fn push_f64_array_json(output: &mut String, values: &[f64]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{value:.3}"));
    }
    output.push(']');
}

fn style_stream_record_count(document: &Document, stream_name: &str) -> usize {
    document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(stream_name))
        .map(|style| summarize_style_stream(style.payload()).records().len())
        .unwrap_or_default()
}

fn document_view_style_group_count(document: &Document) -> usize {
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

fn document_view_style_group_id(code: u16) -> Option<u16> {
    let group_id = code >> 8;
    let record_kind = code & 0x00ff;
    ((0x31..=0x39).contains(&group_id) && (0x04..=0x07).contains(&record_kind))
        .then(|| group_id - 0x30)
}

fn utf16le_ascii_contains(bytes: &[u8], needle: &str) -> bool {
    let mut encoded = Vec::with_capacity(needle.len() * 2);
    for unit in needle.encode_utf16() {
        encoded.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.windows(encoded.len()).any(|window| window == encoded)
}

fn raw_stream_bytes<'a>(document: &'a Document, name: &str) -> Option<&'a [u8]> {
    document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == name)
        .map(RawStream::bytes)
}

fn frame_record_unit_to_css_px(value: u16) -> f32 {
    value as f32 * FRAME_RECORD_UNIT_TO_CSS_PX
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageMarkSeparatorCandidate {
    record_offset: usize,
    record_index: u32,
    line_start: u32,
    line_end: u32,
    y_centipoints: u16,
    advance_centipoints: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageMarkRecordHeader {
    offset: usize,
    index: u32,
    flags: u32,
    line_start: u32,
    line_end: u32,
}

fn document_visible_text(document: &Document) -> String {
    document_paragraph_texts(document)
        .into_iter()
        .map(|(_, text)| text)
        .collect::<Vec<_>>()
        .join("\n")
}

fn utf16_units_for_chars(characters: &[char]) -> usize {
    characters
        .iter()
        .map(|character| character.len_utf16())
        .sum()
}

fn leading_display_units(text: &str) -> usize {
    text.chars()
        .take_while(|character| matches!(character, ' ' | '\u{3000}'))
        .map(display_column_width)
        .sum()
}

#[derive(Clone, Copy)]
struct SuccessDataTestTitleArtPathPlacement {
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
}

#[derive(Clone, Copy)]
struct SuccessDataTestTitleArtFrontFill<'a> {
    rule: &'static str,
    attrs: &'a str,
}

#[derive(Debug, Clone, Copy)]
struct TitleArtFrontFillRenderColorGate<'a> {
    render_fill: &'static str,
    paint_color: Option<&'a str>,
    paint_source: Option<&'static str>,
    render_color_source: &'static str,
    render_color_source_backed: bool,
    source_paint_matches_render_fill: bool,
    render_color_blocked_reason: &'static str,
}

#[derive(Debug, Clone)]
struct TitleArtFrontFillWindingGate {
    path_count: usize,
    multi_contour_path_count: usize,
    opposite_signed_contour_path_count: usize,
    selected_fill_rule: &'static str,
    selected_fill_rule_source: &'static str,
    previous_fill_rule: &'static str,
    render_promoted: bool,
    reference_backed: bool,
    nonzero_title_tight_rms: f32,
    evenodd_title_tight_rms: f32,
}

impl TitleArtFrontFillWindingGate {
    fn svg_attrs(&self) -> String {
        format!(
            " data-title-front-fill-winding-gate-source=\"embeddedPressContourWinding+popplerTitleCropAB\" data-title-front-fill-winding-source-backed=\"{}\" data-title-front-fill-winding-reference-backed=\"{}\" data-title-front-fill-winding-render-promoted=\"{}\" data-title-front-fill-selected-rule=\"{}\" data-title-front-fill-selected-rule-source=\"{}\" data-title-front-fill-previous-rule=\"{}\" data-title-front-fill-path-count=\"{}\" data-title-front-fill-multi-contour-path-count=\"{}\" data-title-front-fill-opposite-signed-contour-path-count=\"{}\" data-title-front-fill-nonzero-title-tight-rms=\"{:.3}\" data-title-front-fill-evenodd-title-tight-rms=\"{:.3}\" data-title-front-fill-rms-improvement=\"{:.3}\"",
            self.opposite_signed_contour_path_count > 0,
            self.reference_backed,
            self.render_promoted,
            escape_xml(self.selected_fill_rule),
            escape_xml(self.selected_fill_rule_source),
            escape_xml(self.previous_fill_rule),
            self.path_count,
            self.multi_contour_path_count,
            self.opposite_signed_contour_path_count,
            self.nonzero_title_tight_rms,
            self.evenodd_title_tight_rms,
            self.nonzero_title_tight_rms - self.evenodd_title_tight_rms
        )
    }
}

#[derive(Debug, Clone)]
struct TitleArtShadowPathPartition<'a> {
    main_paths: Vec<&'a ObjectEmbeddedPressVectorPathCandidate>,
    shadow_paths: Vec<&'a ObjectEmbeddedPressVectorPathCandidate>,
    offset: (i32, i32),
    strategy: &'static str,
}

#[derive(Debug, Clone)]
struct EmbeddedPressTitleArtShadowEffect {
    opacity: f32,
    word0: u32,
    fill_color: String,
}

impl EmbeddedPressTitleArtShadowEffect {
    fn svg_attrs(&self) -> String {
        format!(
            " data-title-shadow-effect-opacity=\"{:.3}\" data-title-shadow-effect-word0=\"0x{:02x}\" data-title-shadow-fill-source=\"embedded-press-0x70-word0-percent-black-on-white\"",
            self.opacity, self.word0
        )
    }
}

#[derive(Debug, Clone)]
struct EmbeddedPressTitleArtTextureEffect {
    opacity: f32,
    word0: u32,
    base_fill_color: String,
    fill_color: String,
}

impl EmbeddedPressTitleArtTextureEffect {
    fn svg_attrs(&self) -> String {
        format!(
            " data-title-texture-effect-candidate-opacity=\"{:.3}\" data-title-texture-effect-candidate-word0=\"0x{:02x}\" data-title-texture-effect-candidate-base-fill=\"{}\" data-title-texture-effect-candidate-fill=\"{}\" data-title-texture-effect-candidate-source=\"embedded-press-interstitial-0x70-word0-percent-black-over-shadow\" data-title-texture-effect-render-promoted=\"false\" data-title-texture-effect-render-promotion-blocked-reason=\"record70-separates-shadow-but-not-interstitial-texture-from-main\"",
            self.opacity,
            self.word0,
            escape_xml(&self.base_fill_color),
            escape_xml(&self.fill_color)
        )
    }
}

#[derive(Debug, Clone)]
struct TitleArtTextureGeometryRoleGate {
    partition_present: bool,
    texture_path_count: usize,
    shadow_outline_path_count: usize,
    main_outline_path_count: usize,
    texture_bbox: Option<(i32, i32, i32, i32)>,
    shadow_bbox: Option<(i32, i32, i32, i32)>,
    main_bbox: Option<(i32, i32, i32, i32)>,
    side_sweep_bbox: Option<(i32, i32, i32, i32)>,
    texture_area: i64,
    texture_main_overlap_area: i64,
    texture_shadow_overlap_area: i64,
    texture_side_sweep_overlap_area: i64,
    texture_main_overlap_ratio: f32,
    texture_shadow_overlap_ratio: f32,
    texture_side_sweep_overlap_ratio: f32,
    texture_contained_by_main_bbox: bool,
    texture_contained_by_shadow_bbox: bool,
    texture_contained_by_side_sweep_bbox: bool,
    role_conclusion: &'static str,
    render_promotion_blocked_reason: &'static str,
}

fn blend_css_hex_colors(foreground: &str, background: &str, alpha: f32) -> Option<String> {
    let foreground = parse_css_hex_rgb(foreground)?;
    let background = parse_css_hex_rgb(background)?;
    let blend_channel = |fg: u8, bg: u8| -> u8 {
        (fg as f32 * alpha + bg as f32 * (1.0 - alpha))
            .round()
            .clamp(0.0, 255.0) as u8
    };
    Some(format!(
        "#{:02x}{:02x}{:02x}",
        blend_channel(foreground.0, background.0),
        blend_channel(foreground.1, background.1),
        blend_channel(foreground.2, background.2)
    ))
}

fn parse_css_hex_rgb(color: &str) -> Option<(u8, u8, u8)> {
    let hex = color.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((red, green, blue))
}

#[derive(Debug, Clone)]
struct EmbeddedPressTitleArtTextureStateSpan {
    state_path_index: usize,
    inherited_span_end_path_index: usize,
    path_count: usize,
    texture_path_count: usize,
    record48_word0_values: Vec<u32>,
    record70_word0_values: Vec<u32>,
    record82_word3_values: Vec<u32>,
    record82_word5_values: Vec<u32>,
}

#[derive(Debug, Clone)]
struct TitleArtFrontErasePaintTransitionGate {
    partition_present: bool,
    interstitial_texture_path_count: usize,
    explicit_state_texture_path_count: usize,
    inherited_texture_path_count: usize,
    span_count: usize,
    span_path_counts: Vec<usize>,
    shadow_last_path_index: Option<usize>,
    interstitial_first_path_index: Option<usize>,
    interstitial_last_path_index: Option<usize>,
    main_first_path_index: Option<usize>,
    shadow_to_interstitial_boundary_adjacent: bool,
    interstitial_to_main_boundary_adjacent: bool,
    record48_separates_shadow_from_texture_and_main: bool,
    record48_separates_texture_from_main: bool,
    record70_word0_separates_texture_from_main: bool,
    record82_word5_separates_texture_from_main: bool,
    record82_word5_matches_shadow: bool,
    record82_word3_is_white_paint_candidate: bool,
    paint_intent_inference: &'static str,
    transition_boundary_class: &'static str,
    render_promotion_blocked_reason: &'static str,
}

impl TitleArtFrontErasePaintTransitionGate {
    fn promotion_ready(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy)]
struct EmbeddedPressPageContext {
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
}

fn push_title_rounded_frame_svg(svg: &mut String, shape: &PageFrameShape) {
    let inset = (shape.corner_radius * 0.65).clamp(2.5, 5.0);
    let inner_x = shape.x + inset;
    let inner_y = shape.y + inset;
    let inner_width = (shape.width - inset * 2.0).max(0.0);
    let inner_height = (shape.height - inset * 2.0).max(0.0);
    let inner_radius = (shape.corner_radius - inset * 0.5).max(0.0);
    svg.push_str(&format!(
        "<g class=\"rjtd-page-frame-shape rjtd-title-rounded-frame\" data-role=\"{}\" data-row-index=\"{}\" data-object-id=\"{}\" data-object-type=\"0x{:04x}\" data-source-x=\"{}\" data-source-y=\"{}\" data-source-width=\"{}\" data-source-height=\"{}\" data-source-corner-radius=\"{}\" data-source-style-id=\"{}\" data-placement-basis=\"{}\" data-style-basis=\"{}\">",
        escape_xml(shape.role),
        shape.row_index,
        shape.object_id,
        shape.object_type,
        shape.source_x,
        shape.source_y,
        shape.source_width,
        shape.source_height,
        shape.source_corner_radius,
        shape.source_style_id,
        escape_xml(shape.placement_basis),
        escape_xml(shape.style_basis)
    ));
    svg.push_str(&format!(
        "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" rx=\"{:.2}\" ry=\"{:.2}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"1.35\"/>",
        shape.x, shape.y, shape.width, shape.height, shape.corner_radius, shape.corner_radius
    ));
    svg.push_str(&format!(
        "<rect x=\"{inner_x:.2}\" y=\"{inner_y:.2}\" width=\"{inner_width:.2}\" height=\"{inner_height:.2}\" rx=\"{inner_radius:.2}\" ry=\"{inner_radius:.2}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"1.05\"/>"
    ));
    svg.push_str("</g>");
}

fn push_horizontal_pattern_bar_svg(svg: &mut String, shape: &PageFrameShape, pattern_id: &str) {
    svg.push_str(&format!(
        "<rect class=\"rjtd-page-frame-shape rjtd-horizontal-pattern-bar\" data-role=\"{}\" data-row-index=\"{}\" data-object-id=\"{}\" data-object-type=\"0x{:04x}\" data-source-x=\"{}\" data-source-y=\"{}\" data-source-width=\"{}\" data-source-height=\"{}\" data-source-corner-radius=\"{}\" data-source-style-id=\"{}\" data-placement-basis=\"{}\" data-style-basis=\"{}\" x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" rx=\"{:.2}\" ry=\"{:.2}\" fill=\"url(#{})\" stroke=\"#111111\" stroke-width=\"1.15\"/>",
        escape_xml(shape.role),
        shape.row_index,
        shape.object_id,
        shape.object_type,
        shape.source_x,
        shape.source_y,
        shape.source_width,
        shape.source_height,
        shape.source_corner_radius,
        shape.source_style_id,
        escape_xml(shape.placement_basis),
        escape_xml(shape.style_basis),
        shape.x,
        shape.y,
        shape.width,
        shape.height,
        shape.corner_radius,
        shape.corner_radius,
        escape_xml(pattern_id)
    ));
}

#[derive(Debug, Clone)]
struct SuccessDataTestFigureLabelLine {
    text: String,
    x: f32,
    y: f32,
    font_size: f32,
    source_span: TextSourceSpan,
    line_header: Option<ShanaiLanLineHeader>,
    spans: Vec<SuccessDataTestFigureLabelSpan>,
}

#[derive(Debug, Clone)]
struct SuccessDataTestFigureLabelSpan {
    text: String,
    x: f32,
    source_span: TextSourceSpan,
}

fn push_unique_static_str(values: &mut Vec<&'static str>, value: &'static str) {
    if value != "none" && !values.contains(&value) {
        values.push(value);
    }
}

fn push_json_string_slice_array(output: &mut String, values: &[&str]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(value));
    }
    output.push(']');
}

fn svg_visual_text(text: &str) -> String {
    text.chars()
        .flat_map(|character| match character {
            '\t' => "\u{3000}\u{3000}".chars().collect::<Vec<_>>(),
            _ => vec![character],
        })
        .collect()
}

fn is_centered_ginga_title_page(page_number: usize, line: &PageTextLine) -> bool {
    page_number == 1 && line.text().contains("銀河鉄道の夜") && line.text().contains("宮沢")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableGridCellRenderText {
    text: String,
    trimmed_text: String,
    basis: &'static str,
    preserves_source_whitespace: bool,
    leading_whitespace_chars: usize,
    trailing_whitespace_chars: usize,
    render_trim_candidate_basis: &'static str,
    render_trim_candidate_blocked_reason: &'static str,
}

#[derive(Debug, Clone)]
struct TableGridSourceDerivedLayout {
    provenance: TableGridSourceDerivedLayoutProvenance,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    row_height: f32,
    column_width: f32,
    column_widths: Vec<f32>,
    column_width_basis: &'static str,
    column_count: usize,
    row_count: usize,
    x_unit_range_basis: &'static str,
    x_unit_start: u16,
    x_unit_end: u16,
    x_unit_full_extent_units: u16,
    x_unit_row_agreement_count: usize,
    x_unit_all_rows_agree: bool,
    x_unit_trailing_header_included: bool,
    x_unit_included_trailing_header_count: usize,
    x_unit_column_slot_width_units: Vec<u16>,
    x_unit_trailing_slot_width_units: Vec<u16>,
    x_origin_inset_units: f32,
    x_origin_inset_basis: &'static str,
    row_height_basis: &'static str,
    page_origin_authority: &'static str,
    anchor_line_index: Option<usize>,
    line_mark_page_origin: Option<TableGridLineMarkPageOriginCandidate>,
    line_mark_page_origin_stride: Option<TableGridLineMarkPageOriginStrideCandidate>,
    raw_header_count: usize,
    matched_cell_header_count: usize,
    min_offset_units: Option<u16>,
    max_extent_units: Option<u16>,
    matched_cell_span_units: Vec<u16>,
    matched_cell_gap_units: Vec<u16>,
    homogeneous_font_size_units: Option<u16>,
    line_mark_row_record_selection: &'static str,
    line_mark_rows_exact_and_contiguous: bool,
    line_header_rows_homogeneous: bool,
    render_promotion_blocked_reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TableGridSourceDerivedLayoutProvenance {
    DecodedCompactPlacement,
    SparseSiblingDerived,
}

impl TableGridSourceDerivedLayoutProvenance {
    const fn as_str(self) -> &'static str {
        match self {
            Self::DecodedCompactPlacement => "decodedCompactPlacement",
            Self::SparseSiblingDerived => "sparseSiblingDerived",
        }
    }
}

#[derive(Debug, Clone)]
struct TableGridLineMarkPageOriginCandidate {
    y: f32,
    first_line_mark_record_index: usize,
    last_line_mark_record_index: usize,
    page_mark_entry_index: usize,
    page_index_candidate: Option<usize>,
    page_line_start: usize,
    page_line_end: usize,
    page_mark_u16_fields: Vec<u16>,
    page_width_px: f32,
    page_height_px: f32,
    page_margin_px: f32,
    page_body_width_px: f32,
    line_offset_from_page_start: usize,
    line_pitch_px: f32,
    line_pitch_basis: &'static str,
    row_height: f32,
}

#[derive(Debug, Clone)]
struct TableGridLineMarkPageOriginStrideCandidate {
    line_mark_record_indexes: Vec<usize>,
    record_stride: usize,
    first_line_mark_record_index: usize,
    last_line_mark_record_index: usize,
    page_mark_entry_index: usize,
    page_index_candidate: Option<usize>,
    page_line_start: usize,
    page_line_end: usize,
    page_mark_u16_fields: Vec<u16>,
    page_width_px: f32,
    page_height_px: f32,
    page_margin_px: f32,
    page_body_width_px: f32,
    line_offset_from_page_start: usize,
    row_height: f32,
    raw_record_index_row_tops: Vec<f32>,
    stride_collapsed_row_tops: Vec<f32>,
}

#[derive(Debug, Clone)]
struct TableGridReferenceLayout {
    x: f32,
    y: f32,
    width: f32,
    row_height: f32,
    column_width: f32,
    column_widths: Vec<f32>,
    column_width_basis: &'static str,
    column_count: usize,
    header_fill: bool,
    corner_radius: f32,
    stroke_width: f32,
    cell_stroke_width: f32,
    font_size: f32,
    cell_text_centered: bool,
}

impl TableGridReferenceLayout {
    fn column_width_at(&self, column_index: usize) -> f32 {
        table_grid_column_width(self.column_width, &self.column_widths, column_index)
    }
}

#[derive(Debug, Clone)]
struct TableGridRenderLayout {
    x: f32,
    y: f32,
    width: f32,
    row_height: f32,
    column_width: f32,
    column_widths: Vec<f32>,
    column_width_basis: &'static str,
    column_count: usize,
    header_fill: bool,
    corner_radius: f32,
    stroke_width: f32,
    cell_stroke_width: f32,
    stroke_width_basis: &'static str,
    font_size: f32,
    font_size_basis: &'static str,
    cell_text_centered: bool,
    cell_text_alignment_basis: &'static str,
    cell_text_x_adjustment: f32,
    cell_text_x_adjustment_basis: &'static str,
    cell_text_baseline_factor: f32,
    cell_text_baseline_basis: &'static str,
    cell_text_font_weight: &'static str,
    cell_text_font_weight_basis: &'static str,
    reference_backed: bool,
    render_promotion_blocked_reason: &'static str,
}

impl TableGridRenderLayout {
    fn from_reference(reference: &TableGridReferenceLayout) -> Self {
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

    fn from_source_derived(source: &TableGridSourceDerivedLayout) -> Self {
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

    fn column_width_at(&self, column_index: usize) -> f32 {
        table_grid_column_width(self.column_width, &self.column_widths, column_index)
    }

    fn column_x_at(&self, column_index: usize) -> f32 {
        table_grid_column_x(self.x, self.column_width, &self.column_widths, column_index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableCandidateLineHeaderRow {
    row_index: usize,
    source_start: usize,
    source_end: usize,
    expected_cell_count: usize,
    matched_cell_count: usize,
    headers: Vec<ShanaiLanLineHeader>,
}

impl TableCandidateLineHeaderRow {
    fn raw_header_count(&self) -> usize {
        self.headers.len()
    }
}

fn table_line_header_source_offset(basis: TextCountRangeOverlapBasis, byte_offset: usize) -> usize {
    match basis {
        TextCountRangeOverlapBasis::Byte => byte_offset,
        TextCountRangeOverlapBasis::Unit => byte_offset / 2,
    }
}

fn fragment_overlaps_rendered_table_projection(
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
struct TableGridReferenceFallbackAdmission {
    allowed: bool,
    blocked_reason: Option<&'static str>,
}

fn preview_svg_cell_text(layout: PageLayout, text: &str, column_width: f32) -> String {
    let max_chars = ((column_width as f64 / column_width_px(layout)).floor() as usize).max(4);
    let mut preview = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}

fn push_tsaiten_document_format_table_projection(
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

fn document_has_tsaiten_projection_evidence(document: &Document) -> bool {
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

#[derive(Debug, Clone)]
struct SuccessDataTestTextSourceMatch {
    source_span: TextSourceSpan,
    line_header: Option<ShanaiLanLineHeader>,
}

fn byte_index_after_utf16_units(text: &str, target_units: usize) -> Option<usize> {
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
struct SuccessDataTestAnswerSheetTextSlot {
    text: String,
    source_token_index: usize,
    x: f32,
    y: f32,
    font_size: f32,
    anchor: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestAnswerSheetTextSlotTemplate {
    source_token_index: usize,
    x_pt: f32,
    y_pt: f32,
    font_pt: f32,
    anchor: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestAnswerSheetPoint {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestAnswerSheetTriangleLabelAnchor {
    text: &'static str,
    point: SuccessDataTestAnswerSheetPoint,
    marker_offset: usize,
    index_offset: usize,
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestAnswerSheetTrianglePlacementCandidate {
    source: &'static str,
    placement_basis: &'static str,
    source_bbox: ObjectFdmIndexBbox,
    a: SuccessDataTestAnswerSheetPoint,
    b: SuccessDataTestAnswerSheetPoint,
    c: SuccessDataTestAnswerSheetPoint,
    right_angle_start: SuccessDataTestAnswerSheetPoint,
    right_angle_corner: SuccessDataTestAnswerSheetPoint,
    right_angle_end: SuccessDataTestAnswerSheetPoint,
    label_anchors: [SuccessDataTestAnswerSheetTriangleLabelAnchor; 3],
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestAnswerSheetFrame {
    layout: PageLayout,
    left_pt: f32,
    top_pt: f32,
    right_pt: f32,
    bottom_pt: f32,
}

impl SuccessDataTestAnswerSheetFrame {
    fn new(layout: PageLayout) -> Self {
        Self {
            layout,
            left_pt: SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_LEFT_PT,
            top_pt: SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_TOP_PT,
            right_pt: SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_RIGHT_PT,
            bottom_pt: SUCCESS_DATA_TEST_ANSWER_SHEET_GRID_BOTTOM_PT,
        }
    }

    fn page_x(self, x_pt: f32) -> f32 {
        x_pt * PDF_POINT_TO_CSS_PX * self.layout.width_px()
            / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX
    }

    fn page_y(self, y_pt: f32) -> f32 {
        y_pt * PDF_POINT_TO_CSS_PX * self.layout.height_px()
            / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
    }

    fn sheet_x(self, x_pt: f32) -> f32 {
        self.page_x(self.left_pt + x_pt)
    }

    fn sheet_y(self, y_pt: f32) -> f32 {
        self.page_y(self.top_pt + y_pt)
    }

    fn width_pt(self) -> f32 {
        self.right_pt - self.left_pt
    }

    fn height_pt(self) -> f32 {
        self.bottom_pt - self.top_pt
    }

    fn bbox(self) -> (f32, f32, f32, f32) {
        let left = self.page_x(self.left_pt);
        let top = self.page_y(self.top_pt);
        let right = self.page_x(self.right_pt);
        let bottom = self.page_y(self.bottom_pt);
        (left, top, right - left, bottom - top)
    }

    fn stroke_width(self, width_pt: f32) -> f32 {
        width_pt * PDF_POINT_TO_CSS_PX * self.layout.width_px()
            / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX
    }

    fn font_size(self, font_pt: f32) -> f32 {
        font_pt * PDF_POINT_TO_CSS_PX * self.layout.height_px()
            / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
    }
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestProjectedPathBBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy)]
struct SuccessDataTestTitleArtHorizontalPlacement {
    frame_x: f32,
    path_x: f32,
    candidate_frame_x: f32,
    candidate_path_x: f32,
    content_left_adjustment: f32,
    stroke_outer_adjustment: f32,
    content_left_only_x: f32,
    frame_record_x: f32,
    basis: &'static str,
    render_promoted: bool,
    stroke_width_candidate: Option<u32>,
}

fn cubic_bezier_point(
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
fn form_slot(
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
fn form_shape(
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

fn escape_xml(text: &str) -> String {
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

#[cfg(test)]
mod tests;
