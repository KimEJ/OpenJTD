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

mod block_text_model;
mod diagnostic_layout_types;
mod document_core_editing;
mod document_metadata;
mod document_text;
mod document_text_control_layout;
mod document_text_text_style;
mod editor_navigation_support;
mod embedded_press;
mod embedded_press_title_art;
mod fdm;
mod footnote_field_editing;
mod json_export_helpers;
mod marks;
mod object_embedded_press_model;
mod object_media;
mod object_shape_editing;
mod object_stream;
mod page_layout;
mod parse;
mod search_render_editing;
mod shanai_lan;
mod shanai_lan_sparse_borders;
mod success_data_test;
mod success_data_test_answer_sheet_geometry;
mod success_data_test_placement_diagnostics;
mod table_candidate;
mod table_cell_editing;
mod table_grid;
mod table_grid_diagnostics;
mod table_grid_render_projection;
mod table_text_candidate_model;

pub use parse::{parse_document, parse_document_with_budget, parse_document_with_limits};

use block_text_model::*;
pub use block_text_model::{
    Block, Inline, StyleRef, TextRun, UnknownBlock, UnknownObject, UnknownStyle,
};
use diagnostic_layout_types::*;
use document_metadata::*;
pub use document_metadata::{
    DocumentAutoText, DocumentFont, DocumentPageMark, DocumentPageMarkEntry, DocumentPaperMark,
    DocumentPaperMarkEntry, DocumentTocEntry, Metadata, PageMarkU16GeometryProfile,
};
pub use document_text::*;
use document_text_control_layout::*;
use editor_navigation_support::*;
use embedded_press::*;
use embedded_press_title_art::*;
pub use fdm::*;
use json_export_helpers::*;
pub use marks::*;
use object_embedded_press_model::*;
pub use object_embedded_press_model::{
    ObjectEmbeddedPressStateRecordCandidate, ObjectEmbeddedPressTextureBezierHeaderCandidate,
    ObjectEmbeddedPressVectorPathCandidate, ObjectEmbeddedPressVectorPathCommandCandidate,
    ObjectEmbeddedPressVectorPathKind, ObjectEmbeddedPressVectorSegmentCandidate,
    ObjectFigureLinkCandidate, ObjectFigureLinkRowCandidate, ObjectFrameRecordCandidate,
    ObjectFrameReferenceRowCandidate, ObjectFrameReferenceRowLink, ObjectStreamCandidate,
    ObjectStreamCandidateEvidence, ObjectStreamCandidateReason, ObjectStreamOwnershipCandidate,
    ObjectStreamOwnershipReferenceCandidate, ObjectVisualListCandidate,
};
pub use object_media::*;
use object_stream::*;
use page_layout::*;
use shanai_lan::*;
use success_data_test::*;
use success_data_test_answer_sheet_geometry::*;
use success_data_test_placement_diagnostics::*;
use table_candidate::*;
use table_grid::*;
use table_grid_diagnostics::*;
use table_grid_render_projection::*;
use table_text_candidate_model::*;
pub use table_text_candidate_model::{
    TableCandidate, TableCandidateColumnGridCandidate, TableCandidateColumnSegment,
    TableCandidateColumnSegmentKind, TableCandidateInterval, TableCandidateSparseTopologyCandidate,
    TableCandidateSparseTopologyColumn, TableCandidateSparseTopologyRow, TextBoundaryCandidate,
    TextControlBoundary, TextCountControlRangeOverlap, TextCountRange, TextCountRangeOverlap,
    TextCountRangeOverlapBasis, TextLayoutExactEvidence, TextSourceSpan,
};

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

#[cfg(test)]
mod tests;
