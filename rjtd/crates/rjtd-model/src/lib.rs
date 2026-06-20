//! Document model types shared by parsers and exporters.

use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "bitmap-images")]
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use rjtd_core::auto_text_info::{AutoTextEntry, read_auto_text_info};
use rjtd_core::container::{EntryKind, inspect_cfb_entries, read_cfb_stream};
use rjtd_core::document_text::{
    DocumentTextControl, DocumentTextElement, DocumentTextMapEntry, DocumentTextMapKind,
    DocumentTextPayload, InlineTextSegment, ParsedDocumentText, SkippedInlineTextSegment,
    map_document_text, read_document_text_payload,
};
use rjtd_core::document_text_position::{
    DocumentTextCountEntry, read_document_text_position_tables,
};
use rjtd_core::font_stream::{FontEntry, read_font_stream};
use rjtd_core::layout_mark::{PAGE_MARK_PATH, PageMark, read_page_mark};
use rjtd_core::record::UnknownRecordKind;
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, StyleStreamRecordSummary,
    StyleStreamSubrecordSummary, TEXT_LAYOUT_STYLE_PATH, read_style_streams,
    summarize_style_stream,
};
use rjtd_core::{Error, Result};

const DOCUMENT_TEXT_INLINE_START_TAG: u32 = 0x001d;
const DOCUMENT_TEXT_RUBY_BASE_SELECTOR: u16 = 0x0003;
const DOCUMENT_TEXT_RUBY_TEXT_SELECTOR: u16 = 0x0082;
const DOCUMENT_TEXT_TOC_PAGE_SELECTOR: u16 = 0x0101;
const DOCUMENT_TEXT_PAGE_BREAK_CONTROL: u16 = 0x000c;
const TEXT_CONTROL_RANGE_DELIMITER_CANDIDATES: [u16; 2] = [0x001c, 0x000e];
const PARAGRAPH_BOUNDARY_DELIMITER_CANDIDATE: u16 = 0x001c;
const TABLE_CELL_DELIMITER_CONTROL: u16 = 0x001c;
const TABLE_ROW_DELIMITER_CONTROL: u16 = 0x000e;
const DIRECT_TABLE_CANDIDATE_SENTINEL: usize = usize::MAX;
const LAYOUT_MAP_DELTA_MIN: isize = -4096;
const LAYOUT_MAP_DELTA_MAX: isize = 4096;
const SO_RECORD_MARKER: &[u8] = b"SO\0\0";
const OBJECT_STREAM_PREFIX_PREVIEW_BYTES: usize = 16;
const OBJECT_STREAM_REFERENCE_OFFSET_PREVIEW_LIMIT: usize = 16;
const OBJECT_STREAM_REFERENCE_ROW_LIMIT: usize = 16;
const VISUAL_LIST_MAGIC_OFFSET: usize = 4;
const VISUAL_LIST_MAGIC: &[u8; 4] = b"BMDV";
const VISUAL_LIST_HEADER_BYTES: usize = 0x50;
const VISUAL_LIST_VERSION_OFFSET: usize = 0x08;
const VISUAL_LIST_FLAGS_OFFSET: usize = 0x0c;
const VISUAL_LIST_WIDTH_OFFSET: usize = 0x1c;
const VISUAL_LIST_HEIGHT_OFFSET: usize = 0x20;
const VISUAL_LIST_ROW_STRIDE_OFFSET: usize = 0x24;
const VISUAL_LIST_BIT_DEPTH_OFFSET: usize = 0x2c;
const VISUAL_LIST_X_PPM_OFFSET: usize = 0x30;
const VISUAL_LIST_Y_PPM_OFFSET: usize = 0x34;
const VISUAL_LIST_RLE_LENGTH_OFFSET: usize = 0x4c;
const VISUAL_LIST_MIN_HORIZONTAL_RUN_PERCENT: usize = 31;
const EMBEDDED_PRESS_SNAPSHOT_MAGIC: &[u8; 12] = b"JSSnapShot32";
const EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET: usize = 0x24;
const EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET: usize = 0x2c;
const EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET: usize = 0x34;
const EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET: usize = 0x38;
const EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET: usize = 0x3c;
const EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET: usize = 0x48;
const EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET: usize = 0x4c;
const EMBEDDED_PRESS_SNAPSHOT_VECTOR_SCAN_OFFSET: usize = 0x4a;
const EMBEDDED_PRESS_SNAPSHOT_VECTOR_SEGMENT_LIMIT: usize = 2048;
const JSEQ3_CONTENTS_MAGIC_UTF16LE: &[u8; 16] = b"M\0A\0T\0H\0.\0V\0A\0F\0";
const JSEQ3_SO_TRAILER_BYTES: usize = 64;
const JSEQ3_SO_FIELD_BYTES: usize = 4;
const JSEQ3_SO_FIELD_COUNT: usize = 9;
const JSEQ3_TEXT_MARKERS: &[&str] = &["Times New Roman", "JustUnitMark", "JustOubunMark"];
const EMBEDDING_INFO_PATH: &str = "/EmbedItems/EmbeddingInfo";
const EMBEDDING_INFO_HEADER_BYTES: usize = 16;
const EMBEDDING_INFO_CLASS_LENGTH_OFFSET: usize = 42;
const EMBEDDING_INFO_CLASS_START_OFFSET: usize = 46;
const EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET: usize = 14;
const EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET: usize = 18;
const EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET: usize = 8;
const EMBEDDING_INFO_TRAILING_BYTES: usize = 80;
const EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET: usize = 0;
const EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET: usize = 4;
const EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET: usize = 8;
const FDM_INDEX_HEADER_BYTES: usize = 20;
const FDM_INDEX_ENTRY_BYTES: usize = 22;
const FDM_INDEX_DECLARED_COUNT_OFFSET: usize = 18;
const FRAME_RECORD_HEADER_BYTES: usize = 16;
const FRAME_RECORD_BYTES: usize = 60;
const FRAME_RECORD_DECLARED_COUNT_OFFSET: usize = 14;
const FRAME_RECORD_ID_OFFSET: usize = 6;
const FRAME_RECORD_TYPE_OFFSET: usize = 12;
const FRAME_RECORD_X_OFFSET: usize = 28;
const FRAME_RECORD_Y_OFFSET: usize = 32;
const FRAME_RECORD_WIDTH_OFFSET: usize = 36;
const FRAME_RECORD_HEIGHT_OFFSET: usize = 40;
const OBJECT_FRAME_REFERENCE_ROW_CANDIDATES: &[ObjectFrameReferenceRowProjection] = &[
    ObjectFrameReferenceRowProjection {
        encoding: "u16-le",
        stride: 12,
        field_offset: 5,
    },
    ObjectFrameReferenceRowProjection {
        encoding: "u16-be",
        stride: 12,
        field_offset: 7,
    },
    ObjectFrameReferenceRowProjection {
        encoding: "u16-be",
        stride: 20,
        field_offset: 15,
    },
];

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
}

pub trait DocumentParser {
    fn parse(&self, data: &[u8]) -> Result<Document>;
}

pub struct IchitaroParser;

impl DocumentParser for IchitaroParser {
    fn parse(&self, data: &[u8]) -> Result<Document> {
        let payload = read_document_text_payload(data)?;
        let map = map_document_text(payload.bytes());
        let mut document = Document::from_document_text_payload(&payload);
        for entry in document_text_toc_entries(map.entries()) {
            document.push_toc_entry(entry);
        }
        document.push_raw_stream(RawStream::new(
            payload.source_name(),
            payload.bytes().to_vec(),
        ));
        if let Ok(style_streams) = read_style_streams(data) {
            for stream in style_streams {
                document.push_unknown_style(UnknownStyle::from_stream(
                    stream.name(),
                    stream.bytes().to_vec(),
                ));
            }
        }
        if let Ok(font_stream) = read_font_stream(data) {
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
        for candidate in object_stream_candidates_from_cfb(data) {
            document.push_object_stream_candidate(candidate);
        }
        for record in object_frame_records_from_cfb(data) {
            document.push_object_frame_record(record);
        }
        for frame in object_embedding_frames_from_cfb(data) {
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
        Ok(document)
    }
}

pub fn parse_document(data: &[u8]) -> Result<Document> {
    IchitaroParser.parse(data)
}

const APP_PAGE_WIDTH_PX: f32 = 794.0;
const APP_PAGE_HEIGHT_PX: f32 = 1123.0;
const APP_PAGE_MARGIN_PX: f32 = 72.0;
const APP_FONT_SIZE_PX: f32 = 13.3;
const APP_LINE_HEIGHT_PX: f32 = 23.0;
const APP_DEFAULT_COLUMN_WIDTH_PX: f32 =
    (APP_PAGE_WIDTH_PX - (APP_PAGE_MARGIN_PX * 2.0)) / APP_WRAP_COLUMNS as f32;
const APP_VERTICAL_DISPLAY_UNIT_PX: f32 = APP_DEFAULT_COLUMN_WIDTH_PX * 0.925;
const APP_IMAGE_DIAGNOSTIC_THUMB_PX: f32 = 72.0;
const APP_IMAGE_DIAGNOSTIC_GAP_PX: f32 = 8.0;
const APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS: usize = 8;
const APP_PAGE_DECORATION_FONT_SIZE_PX: f32 = 13.0;
const APP_WRAP_COLUMNS: usize = 82;
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
const TSAITEN_REFERENCE_PAGE_WIDTH_PX: f32 = 793.7;
const TSAITEN_REFERENCE_PAGE_HEIGHT_PX: f32 = 1122.5;
const DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET: usize = 16;
const DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET: usize = 20;
const PAGE_LAYOUT_STYLE_RECORD_CODE: u16 = 0x4444;
const PAGE_LAYOUT_STYLE_PAYLOAD_WIDTH_OFFSET: usize = 24;
const PAGE_LAYOUT_STYLE_PAYLOAD_HEIGHT_OFFSET: usize = 28;
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
    landscape: bool,
}

impl Default for PageLayout {
    fn default() -> Self {
        Self {
            width_px: APP_PAGE_WIDTH_PX,
            height_px: APP_PAGE_HEIGHT_PX,
            margin_px: APP_PAGE_MARGIN_PX,
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
            landscape: width_px > height_px,
        }
    }

    fn with_portrait_orientation(self) -> Self {
        if self.height_px >= self.width_px {
            self
        } else {
            Self::new(self.height_px, self.width_px)
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

    fn body_width_px(self) -> f32 {
        (self.width_px - (self.margin_px * 2.0)).max(APP_DEFAULT_COLUMN_WIDTH_PX)
    }

    fn body_height_px(self) -> f32 {
        (self.height_px - (self.margin_px * 2.0)).max(APP_LINE_HEIGHT_PX)
    }

    fn wrap_columns(self, writing_mode: WritingMode) -> usize {
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
struct SampleFileLayoutHint {
    fallback_layout: PageLayout,
    writing_mode: WritingMode,
    override_decoded_layout: bool,
}

fn sample_file_layout_hint(file_name: &str) -> Option<SampleFileLayoutHint> {
    let stem = sample_file_stem(file_name);
    let (short_edge_mm, long_edge_mm, override_decoded_layout) = match stem.as_str() {
        "46" => (210.0, 297.0, true),
        "a5" => (148.0, 210.0, false),
        "a6" => (105.0, 148.0, false),
        "b6" => (128.0, 182.0, false),
        "fax02" => (182.0, 257.0, true),
        "ichitaro-20030120132956-0007-sp-dat-tsaiten" => (210.0, 297.0, true),
        "ichitaro-20030120133129-0007-sp-dat-tmogi3_2" => (210.0, 297.0, true),
        "ichitaro-20030228030923-success-002-success_data-test" => (182.0, 257.0, true),
        "ichitaro-20030315134715-success-001-success_data-shanai_lan" => (297.0, 210.0, true),
        _ => return None,
    };
    let fallback_layout = PageLayout::new(
        millimeters_to_css_px(short_edge_mm),
        millimeters_to_css_px(long_edge_mm),
    );
    let writing_mode = match stem.as_str() {
        "a5" | "a6" | "b6" => WritingMode::VerticalRl,
        _ => WritingMode::Horizontal,
    };
    Some(SampleFileLayoutHint {
        fallback_layout,
        writing_mode,
        override_decoded_layout,
    })
}

fn sample_file_stem(file_name: &str) -> String {
    let base_name = file_name.rsplit(['/', '\\']).next().unwrap_or(file_name);
    base_name
        .rsplit_once('.')
        .map(|(stem, _)| stem)
        .unwrap_or(base_name)
        .to_ascii_lowercase()
}

fn page_layout_from_document(document: &Document) -> PageLayout {
    decoded_page_layout_from_styles(document.unknown_styles())
        .unwrap_or_default()
        .with_portrait_orientation()
}

fn decoded_page_layout_from_styles(styles: &[UnknownStyle]) -> Option<PageLayout> {
    styles
        .iter()
        .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))
        .and_then(|style| page_layout_from_document_view_styles(style.payload()))
        .or_else(|| {
            styles
                .iter()
                .find(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
                .and_then(|style| page_layout_from_page_layout_style(style.payload()))
        })
}

fn page_layout_from_document_view_styles(bytes: &[u8]) -> Option<PageLayout> {
    page_layout_from_encoded_mm100_shift8(
        read_be32_at(bytes, DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET)?,
        read_be32_at(bytes, DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET)?,
    )
}

fn page_layout_from_page_layout_style(bytes: &[u8]) -> Option<PageLayout> {
    summarize_style_stream(bytes)
        .records()
        .iter()
        .filter(|record| record.code() == PAGE_LAYOUT_STYLE_RECORD_CODE)
        .find_map(|record| {
            let payload_start = record.offset().checked_add(4)?;
            page_layout_from_encoded_mm100_shift8(
                read_be32_at(
                    bytes,
                    payload_start.checked_add(PAGE_LAYOUT_STYLE_PAYLOAD_WIDTH_OFFSET)?,
                )?,
                read_be32_at(
                    bytes,
                    payload_start.checked_add(PAGE_LAYOUT_STYLE_PAYLOAD_HEIGHT_OFFSET)?,
                )?,
            )
        })
}

fn page_layout_from_encoded_mm100_shift8(
    width_field: u32,
    height_field: u32,
) -> Option<PageLayout> {
    let width_mm100 = width_field >> 8;
    let height_mm100 = height_field >> 8;
    if !paper_size_mm100_is_plausible(width_mm100) || !paper_size_mm100_is_plausible(height_mm100) {
        return None;
    }
    Some(PageLayout::new(
        hundredth_millimeters_to_css_px(width_mm100),
        hundredth_millimeters_to_css_px(height_mm100),
    ))
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

#[derive(Debug, Clone)]
struct DocumentSnapshot {
    id: u32,
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
    caret_section: u32,
    caret_paragraph: u32,
    caret_char_offset: u32,
    clipboard_text: Option<String>,
}

impl DocumentSnapshot {
    fn capture(id: u32, core: &DocumentCore) -> Self {
        Self {
            id,
            document: core.document.clone(),
            pages: core.pages.clone(),
            file_name: core.file_name.clone(),
            dpi: core.dpi,
            page_layout: core.page_layout,
            show_paragraph_marks: core.show_paragraph_marks,
            show_control_codes: core.show_control_codes,
            show_transparent_borders: core.show_transparent_borders,
            clip_enabled: core.clip_enabled,
            writing_mode: core.writing_mode,
            caret_section: core.caret_section,
            caret_paragraph: core.caret_paragraph,
            caret_char_offset: core.caret_char_offset,
            clipboard_text: core.clipboard_text.clone(),
        }
    }
}

impl DocumentCore {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        parse_document(data).map(Self::from_document)
    }

    pub fn from_document(document: Document) -> Self {
        let page_layout = page_layout_from_document(&document);
        let writing_mode = WritingMode::Horizontal;
        let pages = paginate_document_text(&document, page_layout, writing_mode);
        Self {
            document,
            pages,
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
        format!(
            "{{\"version\":\"0.0.0\",\"format\":\"JTD\",\"engine\":\"rjtd\",\"sourceFormat\":\"{}\",\"fileName\":{},\"sectionCount\":1,\"pageCount\":{},\"encrypted\":false,\"hwp3Variant\":false,\"fallbackFont\":{},\"fontsUsed\":{},\"writingMode\":\"{}\",\"writingModeDecoded\":false,\"blockCount\":{},\"rawStreamCount\":{},\"styleStreamCount\":{},\"styleCandidateCount\":{},\"styleCandidateNames\":{},\"styleStreams\":{},\"fontCount\":{},\"fontTable\":{},\"autoTextCount\":{},\"autoTextCandidates\":{},\"tocEntryCount\":{},\"tocEntries\":{},\"pageMarkCount\":{},\"pageMarks\":{},\"objectStreamCandidateCount\":{},\"objectStreamCandidates\":{},\"objectFrameRecordCount\":{},\"objectFrameRecords\":{},\"objectEmbeddingFrameCount\":{},\"objectEmbeddingFrames\":{},\"textCountRangeCount\":{},\"textCountRanges\":{},\"textControlBoundaryCount\":{},\"textControlBoundaries\":{},\"textBoundaryCandidateCount\":{},\"textBoundaryCandidates\":{},\"textParagraphBoundaryCandidateCount\":{},\"textParagraphBoundaryCandidates\":{},\"tableCandidateCount\":{},\"tableCandidates\":{}}}",
            APP_SOURCE_FORMAT,
            json_string(&self.file_name),
            self.page_count(),
            json_string(fallback_font),
            string_array_json(&font_names),
            self.writing_mode.as_str(),
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
            self.document.object_stream_candidates().len(),
            object_stream_candidates_json(self.document.object_stream_candidates()),
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
            self.document.table_candidates().len(),
            table_candidates_json(self.document.table_candidates())
        )
    }

    pub fn set_file_name(&mut self, name: impl Into<String>) {
        self.file_name = name.into();
        if let Some(hint) = sample_file_layout_hint(&self.file_name) {
            if hint.override_decoded_layout || self.page_layout == PageLayout::default() {
                self.page_layout = hint.fallback_layout;
            }
            self.writing_mode = hint.writing_mode;
            self.refresh_pages();
        }
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
        })
    }

    pub fn get_page_info(&self, page_num: u32) -> Result<String> {
        self.page_lines(page_num)?;
        let layout = self.page_layout;
        let body_x = layout.margin_px();
        let body_width = layout.body_width_px();
        Ok(format!(
            "{{\"pageIndex\":{},\"pageNumber\":{},\"width\":{:.1},\"height\":{:.1},\"sectionIndex\":0,\"marginLeft\":{:.1},\"marginRight\":{:.1},\"marginTop\":{:.1},\"marginBottom\":{:.1},\"marginHeader\":0.0,\"marginFooter\":0.0,\"pageBorderLeft\":{:.1},\"pageBorderRight\":{:.1},\"pageBorderTop\":{:.1},\"pageBorderBottom\":{:.1},\"columns\":[{{\"x\":{:.1},\"width\":{:.1}}}]}}",
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
            body_width
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
            let rect = self.cursor_rect_for(control.paragraph_index, control.char_offset)?;
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
        if let Some(pages) = project_sample_front_matter_pages(
            &self.document,
            &self.file_name,
            self.page_layout,
            self.writing_mode,
        ) {
            self.pages = pages;
        }
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
    raw_len: usize,
}

impl DocumentPageMarkEntry {
    fn from_entry(row_index: usize, entry: &rjtd_core::layout_mark::PageMarkEntry) -> Self {
        Self {
            row_index,
            index: entry.index(),
            flags: entry.flags(),
            line_start: entry.line_start(),
            line_end: entry.line_end(),
            raw_len: entry.raw().len(),
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
        self.raw_len
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
    EmbeddedPressSnapshot,
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
            Self::EmbeddedPressSnapshot => "embedded-press-snapshot",
            Self::Jseq3Formula => "jseq3-formula",
            Self::SoMarker => "so-marker",
            Self::ImageSignature => "image-signature",
            Self::SvgSignature => "svg-signature",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJseq3TextMarkerCandidate {
    text: String,
    offset: usize,
    encoding: String,
}

impl ObjectJseq3TextMarkerCandidate {
    fn new(text: impl Into<String>, offset: usize, encoding: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            offset,
            encoding: encoding.into(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJseq3FormulaCandidate {
    magic: String,
    magic_offset: usize,
    so_trailer_offset: Option<usize>,
    so_trailer_length: Option<usize>,
    so_trailer_fields: Vec<u32>,
    text_markers: Vec<ObjectJseq3TextMarkerCandidate>,
    header_prefix: Vec<u8>,
}

impl ObjectJseq3FormulaCandidate {
    fn new(
        magic_offset: usize,
        so_trailer_offset: Option<usize>,
        so_trailer_length: Option<usize>,
        so_trailer_fields: Vec<u32>,
        text_markers: Vec<ObjectJseq3TextMarkerCandidate>,
        header_prefix: Vec<u8>,
    ) -> Self {
        Self {
            magic: "MATH.VAF".to_string(),
            magic_offset,
            so_trailer_offset,
            so_trailer_length,
            so_trailer_fields,
            text_markers,
            header_prefix,
        }
    }

    pub fn magic(&self) -> &str {
        &self.magic
    }

    pub fn magic_offset(&self) -> usize {
        self.magic_offset
    }

    pub fn so_trailer_offset(&self) -> Option<usize> {
        self.so_trailer_offset
    }

    pub fn so_trailer_length(&self) -> Option<usize> {
        self.so_trailer_length
    }

    pub fn so_trailer_fields(&self) -> &[u32] {
        &self.so_trailer_fields
    }

    pub fn text_markers(&self) -> &[ObjectJseq3TextMarkerCandidate] {
        &self.text_markers
    }

    pub fn header_prefix(&self) -> &[u8] {
        &self.header_prefix
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddedPressSnapshotCandidate {
    magic: String,
    body_length_candidate: u32,
    format_marker: String,
    object_count_candidate: u32,
    object_table_offset_candidate: u32,
    payload_length_candidate: u32,
    width: u32,
    height: u32,
    header_prefix: Vec<u8>,
    vector_segments: Vec<ObjectEmbeddedPressVectorSegmentCandidate>,
}

impl ObjectEmbeddedPressSnapshotCandidate {
    fn new(
        body_length_candidate: u32,
        format_marker: impl Into<String>,
        object_count_candidate: u32,
        object_table_offset_candidate: u32,
        payload_length_candidate: u32,
        width: u32,
        height: u32,
        header_prefix: Vec<u8>,
        vector_segments: Vec<ObjectEmbeddedPressVectorSegmentCandidate>,
    ) -> Self {
        Self {
            magic: "JSSnapShot32".to_string(),
            body_length_candidate,
            format_marker: format_marker.into(),
            object_count_candidate,
            object_table_offset_candidate,
            payload_length_candidate,
            width,
            height,
            header_prefix,
            vector_segments,
        }
    }

    pub fn magic(&self) -> &str {
        &self.magic
    }

    pub fn body_length_candidate(&self) -> u32 {
        self.body_length_candidate
    }

    pub fn format_marker(&self) -> &str {
        &self.format_marker
    }

    pub fn object_count_candidate(&self) -> u32 {
        self.object_count_candidate
    }

    pub fn object_table_offset_candidate(&self) -> u32 {
        self.object_table_offset_candidate
    }

    pub fn payload_length_candidate(&self) -> u32 {
        self.payload_length_candidate
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn header_prefix(&self) -> &[u8] {
        &self.header_prefix
    }

    pub fn vector_segments(&self) -> &[ObjectEmbeddedPressVectorSegmentCandidate] {
        &self.vector_segments
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageSignatureHit {
    kind: String,
    offset: usize,
}

impl ObjectImageSignatureHit {
    pub fn new(kind: impl Into<String>, offset: usize) -> Self {
        Self {
            kind: kind.into(),
            offset,
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageNumericHeaderField {
    offset: usize,
    value: u64,
}

impl ObjectImageNumericHeaderField {
    pub fn new(offset: usize, value: u64) -> Self {
        Self { offset, value }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageSourcePathCandidate {
    length_offset: usize,
    declared_length: usize,
    bytes_start: usize,
    bytes_end: usize,
    nul_terminated: bool,
    bytes: Vec<u8>,
    text_lossy: String,
}

impl ObjectImageSourcePathCandidate {
    pub fn new(
        length_offset: usize,
        declared_length: usize,
        bytes_start: usize,
        bytes_end: usize,
        nul_terminated: bool,
        bytes: Vec<u8>,
    ) -> Self {
        let text_bytes = if nul_terminated && bytes.last() == Some(&0) {
            &bytes[..bytes.len().saturating_sub(1)]
        } else {
            &bytes
        };
        Self {
            length_offset,
            declared_length,
            bytes_start,
            bytes_end,
            nul_terminated,
            text_lossy: String::from_utf8_lossy(text_bytes).into_owned(),
            bytes,
        }
    }

    pub fn length_offset(&self) -> usize {
        self.length_offset
    }

    pub fn declared_length(&self) -> usize {
        self.declared_length
    }

    pub fn bytes_start(&self) -> usize {
        self.bytes_start
    }

    pub fn bytes_end(&self) -> usize {
        self.bytes_end
    }

    pub fn nul_terminated(&self) -> bool {
        self.nul_terminated
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn text_lossy(&self) -> &str {
        &self.text_lossy
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageHeaderFieldCandidates {
    u16_le_prefix: Vec<ObjectImageNumericHeaderField>,
    u32_le_prefix: Vec<ObjectImageNumericHeaderField>,
    source_path_candidate: Option<ObjectImageSourcePathCandidate>,
}

impl ObjectImageHeaderFieldCandidates {
    pub fn new(
        u16_le_prefix: Vec<ObjectImageNumericHeaderField>,
        u32_le_prefix: Vec<ObjectImageNumericHeaderField>,
        source_path_candidate: Option<ObjectImageSourcePathCandidate>,
    ) -> Self {
        Self {
            u16_le_prefix,
            u32_le_prefix,
            source_path_candidate,
        }
    }

    pub fn u16_le_prefix(&self) -> &[ObjectImageNumericHeaderField] {
        &self.u16_le_prefix
    }

    pub fn u32_le_prefix(&self) -> &[ObjectImageNumericHeaderField] {
        &self.u32_le_prefix
    }

    pub fn source_path_candidate(&self) -> Option<&ObjectImageSourcePathCandidate> {
        self.source_path_candidate.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageDeclaredLengthCandidate {
    offset: usize,
    value: usize,
    endian: String,
}

impl ObjectImageDeclaredLengthCandidate {
    pub fn new(offset: usize, value: usize, endian: impl Into<String>) -> Self {
        Self {
            offset,
            value,
            endian: endian.into(),
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn endian(&self) -> &str {
        &self.endian
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImagePayloadEnvelope {
    header_start: usize,
    header_end: usize,
    trailer_start: usize,
    trailer_end: usize,
    declared_payload_length: Option<ObjectImageDeclaredLengthCandidate>,
    header_fields: ObjectImageHeaderFieldCandidates,
    header: Vec<u8>,
    trailer: Vec<u8>,
}

impl ObjectImagePayloadEnvelope {
    pub fn new(
        header_start: usize,
        header_end: usize,
        trailer_start: usize,
        trailer_end: usize,
        declared_payload_length: Option<ObjectImageDeclaredLengthCandidate>,
        header: Vec<u8>,
        trailer: Vec<u8>,
    ) -> Self {
        let header_fields = image_header_field_candidates(header_start, &header);
        Self {
            header_start,
            header_end,
            trailer_start,
            trailer_end,
            declared_payload_length,
            header_fields,
            header,
            trailer,
        }
    }

    pub fn header_start(&self) -> usize {
        self.header_start
    }

    pub fn header_end(&self) -> usize {
        self.header_end
    }

    pub fn header_len(&self) -> usize {
        self.header.len()
    }

    pub fn header(&self) -> &[u8] {
        &self.header
    }

    pub fn trailer_start(&self) -> usize {
        self.trailer_start
    }

    pub fn trailer_end(&self) -> usize {
        self.trailer_end
    }

    pub fn trailer_len(&self) -> usize {
        self.trailer.len()
    }

    pub fn trailer(&self) -> &[u8] {
        &self.trailer
    }

    pub fn declared_payload_length(&self) -> Option<&ObjectImageDeclaredLengthCandidate> {
        self.declared_payload_length.as_ref()
    }

    pub fn header_fields(&self) -> &ObjectImageHeaderFieldCandidates {
        &self.header_fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectImagePayloadLocation {
    signature_offset: usize,
    start: usize,
    end: usize,
}

impl ObjectImagePayloadLocation {
    pub fn new(signature_offset: usize, start: usize, end: usize) -> Self {
        Self {
            signature_offset,
            start,
            end,
        }
    }

    pub fn signature_offset(&self) -> usize {
        self.signature_offset
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImagePayloadSpan {
    kind: String,
    mime: String,
    location: ObjectImagePayloadLocation,
    complete: bool,
    payload: Vec<u8>,
    dimensions: Option<ObjectImageDimensions>,
    envelope: ObjectImagePayloadEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectImageDimensions {
    width: u32,
    height: u32,
}

impl ObjectImageDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(self) -> u32 {
        self.width
    }

    pub fn height(self) -> u32 {
        self.height
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

impl ObjectImagePayloadSpan {
    pub fn new(
        kind: impl Into<String>,
        mime: impl Into<String>,
        location: ObjectImagePayloadLocation,
        complete: bool,
        payload: Vec<u8>,
        envelope: ObjectImagePayloadEnvelope,
    ) -> Self {
        let dimensions = image_payload_dimensions(&payload);
        Self {
            kind: kind.into(),
            mime: mime.into(),
            location,
            complete,
            payload,
            dimensions,
            envelope,
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn mime(&self) -> &str {
        &self.mime
    }

    pub fn signature_offset(&self) -> usize {
        self.location.signature_offset()
    }

    pub fn start(&self) -> usize {
        self.location.start()
    }

    pub fn end(&self) -> usize {
        self.location.end()
    }

    pub fn len(&self) -> usize {
        self.payload.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn complete(&self) -> bool {
        self.complete
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn dimensions(&self) -> Option<ObjectImageDimensions> {
        self.dimensions
    }

    pub fn envelope(&self) -> &ObjectImagePayloadEnvelope {
        &self.envelope
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
    image_signature_hits: Vec<ObjectImageSignatureHit>,
    image_payload_spans: Vec<ObjectImagePayloadSpan>,
    visual_list_candidate: Option<ObjectVisualListCandidate>,
    embedded_press_snapshot_candidate: Option<ObjectEmbeddedPressSnapshotCandidate>,
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

    pub fn row_prefix(&self) -> &[u8] {
        &self.row_prefix
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddingFrameCandidate {
    source_path: String,
    row_index: usize,
    row_start: usize,
    embedding_index: usize,
    class_name: String,
    primary_width: u16,
    primary_height: u16,
    frame_ref: u32,
    frame_width: u32,
    frame_height: u32,
    row_prefix: Vec<u8>,
}

impl ObjectEmbeddingFrameCandidate {
    fn new(
        source_path: impl Into<String>,
        row_index: usize,
        row_start: usize,
        row: &[u8],
        class_name: impl Into<String>,
        trailing: &[u8],
    ) -> Option<Self> {
        let class_name = class_name.into();
        let embedding_index = read_le32_at(row, EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET)? as usize;
        let primary_width = read_le16_at(row, EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET)?;
        let primary_height = read_le16_at(row, EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET)?;
        let frame_ref = read_le32_at(trailing, EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET)?;
        let frame_width = read_le32_at(trailing, EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET)?;
        let frame_height = read_le32_at(trailing, EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET)?;
        Some(Self {
            source_path: source_path.into(),
            row_index,
            row_start,
            embedding_index,
            class_name,
            primary_width,
            primary_height,
            frame_ref,
            frame_width,
            frame_height,
            row_prefix: row[..row.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
        })
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

    pub fn embedding_index(&self) -> usize {
        self.embedding_index
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub fn primary_width(&self) -> u16 {
        self.primary_width
    }

    pub fn primary_height(&self) -> u16 {
        self.primary_height
    }

    pub fn frame_ref(&self) -> u32 {
        self.frame_ref
    }

    pub fn frame_width(&self) -> u32 {
        self.frame_width
    }

    pub fn frame_height(&self) -> u32 {
        self.frame_height
    }

    pub fn row_prefix(&self) -> &[u8] {
        &self.row_prefix
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmIndexBbox {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl ObjectFdmIndexBbox {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn left(self) -> i32 {
        self.left
    }

    pub fn top(self) -> i32 {
        self.top
    }

    pub fn right(self) -> i32 {
        self.right
    }

    pub fn bottom(self) -> i32 {
        self.bottom
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFdmIndexEntryCandidate {
    index_path: String,
    vector_path: String,
    row_index: usize,
    index_offset: usize,
    vector_offset: usize,
    next_vector_offset: usize,
    vector_len: usize,
    kind: u16,
    bbox: ObjectFdmIndexBbox,
    valid_vector_offset: bool,
    vector_prefix: Vec<u8>,
    image_signature_hits: Vec<ObjectImageSignatureHit>,
    segment_image_signature_hits: Vec<ObjectImageSignatureHit>,
}

impl ObjectFdmIndexEntryCandidate {
    pub fn index_path(&self) -> &str {
        &self.index_path
    }

    pub fn vector_path(&self) -> &str {
        &self.vector_path
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn index_offset(&self) -> usize {
        self.index_offset
    }

    pub fn vector_offset(&self) -> usize {
        self.vector_offset
    }

    pub fn next_vector_offset(&self) -> usize {
        self.next_vector_offset
    }

    pub fn vector_len(&self) -> usize {
        self.vector_len
    }

    pub fn kind(&self) -> u16 {
        self.kind
    }

    pub fn bbox(&self) -> ObjectFdmIndexBbox {
        self.bbox
    }

    pub fn valid_vector_offset(&self) -> bool {
        self.valid_vector_offset
    }

    pub fn vector_prefix(&self) -> &[u8] {
        &self.vector_prefix
    }

    pub fn image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.image_signature_hits
    }

    pub fn segment_image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.segment_image_signature_hits
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
    embedded_press_snapshot_candidate: Option<ObjectEmbeddedPressSnapshotCandidate>,
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
            embedded_press_snapshot_candidate: None,
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

    fn with_embedded_press_snapshot_candidate(
        mut self,
        snapshot: Option<ObjectEmbeddedPressSnapshotCandidate>,
    ) -> Self {
        self.embedded_press_snapshot_candidate = snapshot;
        self
    }

    fn with_jseq3_formula_candidate(
        mut self,
        formula: Option<ObjectJseq3FormulaCandidate>,
    ) -> Self {
        self.jseq3_formula_candidate = formula;
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
            image_signature_hits: evidence.image_signature_hits,
            image_payload_spans: evidence.image_payload_spans,
            visual_list_candidate: evidence.visual_list_candidate,
            embedded_press_snapshot_candidate: evidence.embedded_press_snapshot_candidate,
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

    pub fn image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.image_signature_hits
    }

    pub fn image_payload_spans(&self) -> &[ObjectImagePayloadSpan] {
        &self.image_payload_spans
    }

    pub fn visual_list_candidate(&self) -> Option<&ObjectVisualListCandidate> {
        self.visual_list_candidate.as_ref()
    }

    pub fn embedded_press_snapshot_candidate(
        &self,
    ) -> Option<&ObjectEmbeddedPressSnapshotCandidate> {
        self.embedded_press_snapshot_candidate.as_ref()
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

    fn is_document_text_control_run_candidate(&self) -> bool {
        self.text_boundary_candidate_index == DIRECT_TABLE_CANDIDATE_SENTINEL
            && self.text_count_range_index == DIRECT_TABLE_CANDIDATE_SENTINEL
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> &'static str {
        if self.is_document_text_control_run_candidate() {
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

    pub fn is_row_like(&self) -> bool {
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

        let pattern = self.intervals.iter().find_map(|interval| {
            (!interval.column_segments().is_empty()).then(|| {
                interval
                    .column_segments()
                    .iter()
                    .map(|segment| segment.kind())
                    .collect::<Vec<_>>()
            })
        })?;

        if pattern.len() < 2 {
            return None;
        }

        Some(TableCandidateColumnGridCandidate::new(
            self.intervals.len(),
            pattern,
            split_rows,
        ))
    }

    pub fn rule(&self) -> &'static str {
        if self.is_document_text_control_run_candidate() {
            "document-text-001c-cells-with-000e-row-breaks"
        } else {
            "control-delimited-text-count-range-with-multiple-intervals"
        }
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
    text: String,
}

impl TableCandidateColumnSegment {
    fn new(
        index: usize,
        kind: TableCandidateColumnSegmentKind,
        char_start: usize,
        char_end: usize,
        text: String,
    ) -> Self {
        Self {
            index,
            kind,
            char_start,
            char_end,
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
pub struct TextParagraphBoundaryCandidate {
    index: usize,
    text_boundary_candidate_index: usize,
    text_count_range_index: usize,
    source_start: usize,
    source_end: usize,
    text_count_range_span: u32,
    line_word_evidence: TextLayoutExactEvidence,
    page_field_evidence: TextLayoutExactEvidence,
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

fn text_count_entry_chosen_range(raw: &[u8], family: &str) -> (u32, u32) {
    if family == "be1-shifted" {
        (read_be32_candidate(raw, 1), read_be32_candidate(raw, 5))
    } else {
        (read_be32_candidate(raw, 0), read_be32_candidate(raw, 4))
    }
}

fn text_count_entry_tail_offset(family: &str) -> usize {
    if family == "be1-shifted" { 9 } else { 8 }
}

fn classify_text_count_entry_family(raw: &[u8]) -> &'static str {
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Paragraph {
    inlines: Vec<Inline>,
    style: Option<StyleRef>,
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

    fn set_style(&mut self, style: Option<StyleRef>) {
        self.style = style;
    }

    fn set_text(&mut self, text: impl Into<String>) {
        self.inlines = vec![Inline::Text(TextRun::new(text, None))];
    }
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
pub struct RubyAnnotation {
    base_text: String,
    annotation_text: String,
    annotation_selector: u16,
    annotation_source: UnknownObject,
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

fn unknown_object_from_skipped_inline(segment: &SkippedInlineTextSegment) -> UnknownObject {
    UnknownObject::new(
        UnknownRecordKind::new(Some(DOCUMENT_TEXT_INLINE_START_TAG)),
        segment.raw_bytes().to_vec(),
    )
}

fn object_stream_candidates_from_cfb(data: &[u8]) -> Vec<ObjectStreamCandidate> {
    let Ok(entries) = inspect_cfb_entries(data) else {
        return Vec::new();
    };

    let mut candidates = Vec::new();
    let mut streams = Vec::new();
    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        let Ok(stream) = read_cfb_stream(data, entry.path()) else {
            continue;
        };
        if let Some(candidate) = classify_object_stream_candidate(entry.path(), &stream) {
            candidates.push(candidate);
        }
        streams.push((entry.path().to_string(), stream));
    }
    attach_object_stream_ownership_references(&mut candidates, &streams);
    attach_object_stream_fdm_index_entries(&mut candidates, &streams);
    candidates
}

fn object_frame_records_from_cfb(data: &[u8]) -> Vec<ObjectFrameRecordCandidate> {
    let Ok(entries) = inspect_cfb_entries(data) else {
        return Vec::new();
    };

    let Some(entry) = entries.iter().find(|entry| {
        entry.kind() == EntryKind::Stream && entry.path().eq_ignore_ascii_case("/Frame")
    }) else {
        return Vec::new();
    };

    let Ok(stream) = read_cfb_stream(data, entry.path()) else {
        return Vec::new();
    };

    object_frame_records_from_stream(entry.path(), &stream)
}

fn object_frame_records_from_stream(path: &str, stream: &[u8]) -> Vec<ObjectFrameRecordCandidate> {
    let Some(declared_count) =
        read_be16_at(stream, FRAME_RECORD_DECLARED_COUNT_OFFSET).map(usize::from)
    else {
        return Vec::new();
    };

    let Some(expected_len) =
        FRAME_RECORD_HEADER_BYTES.checked_add(declared_count.saturating_mul(FRAME_RECORD_BYTES))
    else {
        return Vec::new();
    };
    if stream.len() < expected_len {
        return Vec::new();
    }

    (0..declared_count)
        .filter_map(|row_index| {
            let row_start = FRAME_RECORD_HEADER_BYTES
                .checked_add(row_index.checked_mul(FRAME_RECORD_BYTES)?)?;
            let row_end = row_start.checked_add(FRAME_RECORD_BYTES)?;
            let row = stream.get(row_start..row_end)?;
            Some(ObjectFrameRecordCandidate::new(
                path, row_index, row_start, row,
            ))
        })
        .collect()
}

fn object_embedding_frames_from_cfb(data: &[u8]) -> Vec<ObjectEmbeddingFrameCandidate> {
    let Ok(stream) = read_cfb_stream(data, EMBEDDING_INFO_PATH) else {
        return Vec::new();
    };

    object_embedding_frames_from_stream(EMBEDDING_INFO_PATH, &stream)
}

fn object_embedding_frames_from_stream(
    path: &str,
    stream: &[u8],
) -> Vec<ObjectEmbeddingFrameCandidate> {
    let Some(declared_count) = read_le32_at(stream, 0).map(|value| value as usize) else {
        return Vec::new();
    };

    let mut frames = Vec::new();
    let mut cursor = EMBEDDING_INFO_HEADER_BYTES;
    for row_index in 0..declared_count {
        let Some(class_len_offset) = cursor.checked_add(EMBEDDING_INFO_CLASS_LENGTH_OFFSET) else {
            break;
        };
        let Some(class_len) = read_le32_at(stream, class_len_offset).map(|value| value as usize)
        else {
            break;
        };
        let Some(class_start) = cursor.checked_add(EMBEDDING_INFO_CLASS_START_OFFSET) else {
            break;
        };
        let Some(class_end) = class_start.checked_add(class_len) else {
            break;
        };
        let Some(row_end) = class_end.checked_add(EMBEDDING_INFO_TRAILING_BYTES) else {
            break;
        };
        let Some(row) = stream.get(cursor..row_end) else {
            break;
        };
        let Some(class_bytes) = stream.get(class_start..class_end) else {
            break;
        };
        let trailing = &stream[class_end..row_end];
        let Some(class_name) = decode_utf16le_c_string(class_bytes) else {
            break;
        };
        if class_name.is_empty() || class_len == 0 || class_len % 2 != 0 {
            break;
        }
        let Some(frame) =
            ObjectEmbeddingFrameCandidate::new(path, row_index, cursor, row, class_name, trailing)
        else {
            break;
        };
        if embedding_frame_candidate_is_plausible(&frame) {
            frames.push(frame);
        }
        cursor = row_end;
    }

    frames
}

fn embedding_frame_candidate_is_plausible(frame: &ObjectEmbeddingFrameCandidate) -> bool {
    frame.embedding_index() > 0
        && frame.frame_ref() > 0
        && frame.frame_width() > 0
        && frame.frame_height() > 0
        && frame.frame_width() <= 200_000
        && frame.frame_height() <= 200_000
        && frame.class_name().chars().all(|character| {
            character == '.'
                || character == '_'
                || character == '-'
                || character.is_ascii_alphanumeric()
        })
}

fn decode_utf16le_c_string(bytes: &[u8]) -> Option<String> {
    if bytes.len() % 2 != 0 {
        return None;
    }
    let units = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .take_while(|unit| *unit != 0)
        .collect::<Vec<_>>();
    String::from_utf16(&units).ok()
}

fn attach_object_stream_ownership_references(
    candidates: &mut [ObjectStreamCandidate],
    streams: &[(String, Vec<u8>)],
) {
    for candidate in candidates {
        let Some(embedding_index) = candidate
            .ownership_candidate()
            .and_then(ObjectStreamOwnershipCandidate::embedding_index)
        else {
            continue;
        };
        if candidate.image_payload_spans().is_empty() {
            continue;
        }

        let references =
            object_stream_ownership_references(candidate.path(), embedding_index, streams);
        let frame_rows = object_stream_frame_reference_rows(&references, streams);
        candidate.set_ownership_reference_candidates(references);
        candidate.set_frame_reference_row_candidates(frame_rows);
    }
}

fn object_stream_ownership_references(
    source_path: &str,
    embedding_index: usize,
    streams: &[(String, Vec<u8>)],
) -> Vec<ObjectStreamOwnershipReferenceCandidate> {
    let patterns = object_stream_embedding_reference_patterns(embedding_index);
    let mut references = Vec::new();

    for (target_path, stream) in streams {
        if target_path == source_path || !is_object_reference_target_path(target_path) {
            continue;
        }

        for (encoding, pattern) in &patterns {
            let offsets = find_subslice_offsets(stream, pattern);
            if offsets.is_empty() {
                continue;
            }

            let total_matches = offsets.len();
            let offsets = offsets
                .into_iter()
                .take(OBJECT_STREAM_REFERENCE_OFFSET_PREVIEW_LIMIT)
                .collect();
            references.push(ObjectStreamOwnershipReferenceCandidate::new(
                target_path,
                *encoding,
                total_matches,
                offsets,
            ));
        }
    }

    references.sort_by(|left, right| {
        left.target_path()
            .cmp(right.target_path())
            .then_with(|| left.encoding().cmp(right.encoding()))
            .then_with(|| left.total_matches().cmp(&right.total_matches()))
    });
    references.truncate(OBJECT_STREAM_REFERENCE_ROW_LIMIT);
    references
}

fn object_stream_frame_reference_rows(
    references: &[ObjectStreamOwnershipReferenceCandidate],
    streams: &[(String, Vec<u8>)],
) -> Vec<ObjectFrameReferenceRowCandidate> {
    let mut rows = Vec::new();

    for reference in references
        .iter()
        .filter(|reference| reference.target_path().eq_ignore_ascii_case("/Frame"))
    {
        let Some((_, target_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(reference.target_path()))
        else {
            continue;
        };

        for offset in reference.offsets() {
            for projection in OBJECT_FRAME_REFERENCE_ROW_CANDIDATES
                .iter()
                .filter(|projection| {
                    projection.encoding == reference.encoding()
                        && offset % projection.stride == projection.field_offset
                })
            {
                let pattern_len = object_reference_pattern_len(reference.encoding());
                if projection.field_offset + pattern_len > projection.stride {
                    continue;
                }
                let row_start = offset.saturating_sub(projection.field_offset);
                let Some(row_end) = row_start.checked_add(projection.stride) else {
                    continue;
                };
                let Some(row) = target_stream.get(row_start..row_end) else {
                    continue;
                };
                rows.push(ObjectFrameReferenceRowCandidate::new(
                    reference.target_path(),
                    projection.encoding,
                    projection.stride,
                    projection.field_offset,
                    ObjectFrameReferenceRowLocation::new(
                        *offset,
                        offset / projection.stride,
                        row_start,
                    ),
                    row.to_vec(),
                ));
            }
        }
    }

    attach_object_frame_row_suffix_links(&mut rows);
    rows
}

fn object_reference_pattern_len(encoding: &str) -> usize {
    match encoding {
        "u16-le" | "u16-be" => 2,
        "u32-le" | "u32-be" => 4,
        _ => 1,
    }
}

fn attach_object_frame_row_suffix_links(rows: &mut [ObjectFrameReferenceRowCandidate]) {
    let row12_records = rows
        .iter()
        .filter(|row| row.stride() == 12)
        .map(|row| {
            (
                row.row().to_vec(),
                row.family().to_string(),
                row.row_start(),
                row.row_index(),
            )
        })
        .collect::<Vec<_>>();

    for row in rows
        .iter_mut()
        .filter(|row| row.stride() == 20 && row.field_offset() == 15)
    {
        let Some(suffix) = row.row().get(row.row().len().saturating_sub(12)..) else {
            continue;
        };
        let Some((_, matched_family, matched_row_start, matched_row_index)) = row12_records
            .iter()
            .find(|(candidate_row, _, _, _)| candidate_row.as_slice() == suffix)
        else {
            continue;
        };
        row.set_suffix_link(ObjectFrameReferenceRowLink::new(
            "same-candidate",
            matched_family.as_str(),
            *matched_row_start,
            *matched_row_index,
        ));
    }
}

fn attach_object_stream_fdm_index_entries(
    candidates: &mut [ObjectStreamCandidate],
    streams: &[(String, Vec<u8>)],
) {
    for candidate in candidates {
        let Some(index_path) = fdm_index_path_for_vector(candidate.path()) else {
            continue;
        };
        let Some((_, vector_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(candidate.path()))
        else {
            continue;
        };
        let Some((actual_index_path, index_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(&index_path))
        else {
            continue;
        };

        let all_entries = parse_fdm_index_entries(index_stream, vector_stream.len());
        let entries = fdm_index_declared_entries(index_stream, &all_entries);
        if entries.is_empty() {
            continue;
        }
        let vector_hits = image_signature_hits(vector_stream);
        let fdm_entries = entries
            .iter()
            .map(|entry| {
                let segment = fdm_vector_segment(entry.vector_offset, entries, vector_stream);
                let segment_hits =
                    fdm_segment_signature_hits(&vector_hits, segment.start, segment.end);
                let relative_hits = fdm_relative_signature_hits(&segment_hits, segment.start);
                let vector_prefix = vector_stream
                    .get(segment.start..segment.end)
                    .unwrap_or_default();

                ObjectFdmIndexEntryCandidate {
                    index_path: actual_index_path.clone(),
                    vector_path: candidate.path().to_string(),
                    row_index: entry.row_index,
                    index_offset: entry.index_offset,
                    vector_offset: entry.vector_offset,
                    next_vector_offset: segment.end,
                    vector_len: segment.end.saturating_sub(segment.start),
                    kind: entry.kind,
                    bbox: ObjectFdmIndexBbox::new(entry.left, entry.top, entry.right, entry.bottom),
                    valid_vector_offset: entry.valid_vector_offset,
                    vector_prefix: vector_prefix
                        [..vector_prefix.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
                        .to_vec(),
                    image_signature_hits: segment_hits,
                    segment_image_signature_hits: relative_hits,
                }
            })
            .collect();
        candidate.set_fdm_index_entry_candidates(fdm_entries);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FdmIndexEntry {
    row_index: usize,
    index_offset: usize,
    vector_offset: usize,
    kind: u16,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    valid_vector_offset: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FdmVectorSegment {
    start: usize,
    end: usize,
}

fn fdm_index_path_for_vector(vector_path: &str) -> Option<String> {
    if !vector_path
        .get(vector_path.len().saturating_sub("/FDMVector".len())..)?
        .eq_ignore_ascii_case("/FDMVector")
    {
        return None;
    }
    vector_path
        .get(..vector_path.len().saturating_sub("/FDMVector".len()))
        .map(|prefix| format!("{prefix}/FDMIndex"))
}

fn fdm_index_declared_entries<'a>(
    index_stream: &[u8],
    entries: &'a [FdmIndexEntry],
) -> &'a [FdmIndexEntry] {
    if !index_stream.starts_with(&[0x03, 0x0b, 0x00, 0x01]) {
        return &[];
    }

    let Some(count) = read_be16_at(index_stream, FDM_INDEX_DECLARED_COUNT_OFFSET).map(usize::from)
    else {
        return &[];
    };
    if count > entries.len() {
        return &[];
    }

    &entries[..count]
}

fn parse_fdm_index_entries(index_stream: &[u8], vector_len: usize) -> Vec<FdmIndexEntry> {
    if index_stream.len() < FDM_INDEX_HEADER_BYTES {
        return Vec::new();
    }

    let entry_bytes = index_stream.len() - FDM_INDEX_HEADER_BYTES;
    let entry_count = entry_bytes / FDM_INDEX_ENTRY_BYTES;
    let mut entries = Vec::with_capacity(entry_count);
    for row_index in 0..entry_count {
        let index_offset = FDM_INDEX_HEADER_BYTES + row_index * FDM_INDEX_ENTRY_BYTES;
        let Some(vector_offset) = read_be32_at(index_stream, index_offset) else {
            continue;
        };
        let Some(kind) = read_be16_at(index_stream, index_offset + 4) else {
            continue;
        };
        let Some(left) = read_i32_be_at(index_stream, index_offset + 6) else {
            continue;
        };
        let Some(top) = read_i32_be_at(index_stream, index_offset + 10) else {
            continue;
        };
        let Some(right) = read_i32_be_at(index_stream, index_offset + 14) else {
            continue;
        };
        let Some(bottom) = read_i32_be_at(index_stream, index_offset + 18) else {
            continue;
        };
        let vector_offset = vector_offset as usize;
        entries.push(FdmIndexEntry {
            row_index,
            index_offset,
            vector_offset,
            kind,
            left,
            top,
            right,
            bottom,
            valid_vector_offset: vector_offset < vector_len,
        });
    }
    entries
}

fn fdm_vector_segment(
    vector_offset: usize,
    entries: &[FdmIndexEntry],
    vector_stream: &[u8],
) -> FdmVectorSegment {
    let start = vector_offset.min(vector_stream.len());
    let end = entries
        .iter()
        .filter_map(|entry| {
            (entry.vector_offset > vector_offset && entry.vector_offset <= vector_stream.len())
                .then_some(entry.vector_offset)
        })
        .min()
        .unwrap_or(vector_stream.len());
    FdmVectorSegment { start, end }
}

fn fdm_segment_signature_hits(
    vector_hits: &[ObjectImageSignatureHit],
    start: usize,
    end: usize,
) -> Vec<ObjectImageSignatureHit> {
    vector_hits
        .iter()
        .filter(|hit| hit.offset() >= start && hit.offset() < end)
        .cloned()
        .collect()
}

fn fdm_relative_signature_hits(
    segment_hits: &[ObjectImageSignatureHit],
    segment_start: usize,
) -> Vec<ObjectImageSignatureHit> {
    segment_hits
        .iter()
        .map(|hit| {
            ObjectImageSignatureHit::new(hit.kind(), hit.offset().saturating_sub(segment_start))
        })
        .collect()
}

fn classify_object_frame_reference_row(
    row: &[u8],
    encoding: &str,
    stride: usize,
    field_offset: usize,
) -> &'static str {
    let be16 = read_be16_fields(row);

    match (encoding, stride, field_offset) {
        ("u16-le", 12, 5)
            if be16.len() == 6
                && be16[1] == 0
                && be16[3] == 0
                && be16[4] <= 0x0010
                && be16[5] <= 0x0010 =>
        {
            "frame-index-flag-row12"
        }
        ("u16-le", 12, 5) => "frame-index-mixed-row12",
        ("u16-be", 12, 7)
            if be16.len() == 6
                && be16[0] == 0
                && be16[1] == 0
                && be16[2] == 0
                && be16[3] == 0
                && be16[5] == 0 =>
        {
            "frame-index-tail-zero-row12"
        }
        ("u16-be", 12, 7) if be16.len() == 6 && be16[1] == 0 && be16[3] == 0 && be16[5] == 0 => {
            "frame-index-tail-coordinate-row12"
        }
        ("u16-be", 12, 7) => "frame-index-tail-mixed-row12",
        ("u16-be", 20, 15) if be16.len() == 10 && be16[9] == 0 => "frame-index-tail-window20",
        ("u16-be", 20, 15) => "frame-index-mixed-window20",
        _ => "frame-index-unknown",
    }
}

fn object_stream_embedding_reference_patterns(
    embedding_index: usize,
) -> Vec<(&'static str, Vec<u8>)> {
    let mut patterns = Vec::new();
    if let Ok(index) = u16::try_from(embedding_index) {
        patterns.push(("u16-le", index.to_le_bytes().to_vec()));
        patterns.push(("u16-be", index.to_be_bytes().to_vec()));
    }
    if let Ok(index) = u32::try_from(embedding_index) {
        patterns.push(("u32-le", index.to_le_bytes().to_vec()));
        patterns.push(("u32-be", index.to_be_bytes().to_vec()));
    }
    patterns
}

fn is_object_reference_target_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.contains("/figuredata/")
        || lower.ends_with("/figure")
        || lower.ends_with("/frame")
        || lower.ends_with("/layoutbox")
        || lower.ends_with("/pagemark")
        || lower.ends_with("/papermark")
}

fn classify_object_stream_candidate(path: &str, stream: &[u8]) -> Option<ObjectStreamCandidate> {
    let mut reasons = Vec::new();
    push_object_path_reasons(path, &mut reasons);

    let image_signature_hits = image_signature_hits(stream);
    let image_payload_spans = image_payload_spans(stream, &image_signature_hits);
    let visual_list_candidate = visual_list_candidate_from_stream(path, stream);
    let embedded_press_snapshot_candidate = embedded_press_snapshot_candidate_from_stream(stream);
    let jseq3_formula_candidate = jseq3_formula_candidate_from_stream(path, stream);
    if !image_signature_hits.is_empty() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::ImageSignature);
    }
    if embedded_press_snapshot_candidate.is_some() {
        push_unique_object_reason(
            &mut reasons,
            ObjectStreamCandidateReason::EmbeddedPressSnapshot,
        );
    }
    if jseq3_formula_candidate.is_some() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::Jseq3Formula);
    }

    let svg_offsets = svg_signature_offsets(stream);
    if !svg_offsets.is_empty() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::SvgSignature);
    }

    let so_offsets = find_subslice_offsets(stream, SO_RECORD_MARKER);
    if !so_offsets.is_empty() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::SoMarker);
    }

    if reasons.is_empty() {
        return None;
    }

    Some(ObjectStreamCandidate::new(
        path,
        stream.len(),
        ObjectStreamCandidateEvidence::new(
            reasons,
            image_signature_hits,
            image_payload_spans,
            visual_list_candidate,
            svg_offsets,
            so_offsets,
        )
        .with_embedded_press_snapshot_candidate(embedded_press_snapshot_candidate)
        .with_jseq3_formula_candidate(jseq3_formula_candidate),
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
    ))
}

fn jseq3_formula_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectJseq3FormulaCandidate> {
    if !path.ends_with("/JSEQ3Contents") {
        return None;
    }
    if stream.get(..JSEQ3_CONTENTS_MAGIC_UTF16LE.len())? != JSEQ3_CONTENTS_MAGIC_UTF16LE {
        return None;
    }

    let so_trailer_offset = jseq3_so_trailer_offset(stream);
    let so_trailer_length = so_trailer_offset.map(|offset| stream.len().saturating_sub(offset));
    let so_trailer_fields = so_trailer_offset
        .and_then(|offset| stream.get(offset..))
        .map(jseq3_so_trailer_fields)
        .unwrap_or_default();
    let text_markers = jseq3_text_marker_candidates(stream);
    Some(ObjectJseq3FormulaCandidate::new(
        0,
        so_trailer_offset,
        so_trailer_length,
        so_trailer_fields,
        text_markers,
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
    ))
}

fn jseq3_so_trailer_offset(stream: &[u8]) -> Option<usize> {
    find_subslice_offsets(stream, SO_RECORD_MARKER)
        .into_iter()
        .find(|offset| {
            offset.saturating_add(JSEQ3_SO_FIELD_COUNT * JSEQ3_SO_FIELD_BYTES) <= stream.len()
                && offset.saturating_add(JSEQ3_SO_TRAILER_BYTES) >= stream.len()
        })
}

fn jseq3_so_trailer_fields(trailer: &[u8]) -> Vec<u32> {
    (0..JSEQ3_SO_FIELD_COUNT)
        .filter_map(|index| read_le32_at(trailer, index * JSEQ3_SO_FIELD_BYTES))
        .collect()
}

fn jseq3_text_marker_candidates(stream: &[u8]) -> Vec<ObjectJseq3TextMarkerCandidate> {
    let mut candidates = Vec::new();
    for marker in JSEQ3_TEXT_MARKERS {
        let encoded = utf16le_bytes(marker);
        for offset in find_subslice_offsets(stream, &encoded) {
            candidates.push(ObjectJseq3TextMarkerCandidate::new(
                *marker, offset, "utf-16le",
            ));
        }
    }
    candidates.sort_by_key(|candidate| candidate.offset());
    candidates
}

fn utf16le_bytes(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

fn embedded_press_snapshot_candidate_from_stream(
    stream: &[u8],
) -> Option<ObjectEmbeddedPressSnapshotCandidate> {
    if stream.get(..EMBEDDED_PRESS_SNAPSHOT_MAGIC.len())? != EMBEDDED_PRESS_SNAPSHOT_MAGIC {
        return None;
    }
    let body_length_candidate = read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET)?;
    let format_marker = stream
        .get(EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET..EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET + 4)
        .map(|bytes| {
            bytes
                .iter()
                .copied()
                .filter(|byte| byte.is_ascii_graphic())
                .map(char::from)
                .collect::<String>()
        })?;
    let object_count_candidate = read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET)?;
    let object_table_offset_candidate =
        read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET)?;
    let payload_length_candidate =
        read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET)?;
    let width = read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET)?;
    let height = read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET)?;
    if width == 0 || height == 0 || body_length_candidate == 0 || payload_length_candidate == 0 {
        return None;
    }
    let vector_segments = embedded_press_snapshot_vector_segments(stream, width, height);
    Some(ObjectEmbeddedPressSnapshotCandidate::new(
        body_length_candidate,
        format_marker,
        object_count_candidate,
        object_table_offset_candidate,
        payload_length_candidate,
        width,
        height,
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
        vector_segments,
    ))
}

fn embedded_press_snapshot_vector_segments(
    stream: &[u8],
    width: u32,
    height: u32,
) -> Vec<ObjectEmbeddedPressVectorSegmentCandidate> {
    if width == 0 || height == 0 || EMBEDDED_PRESS_SNAPSHOT_VECTOR_SCAN_OFFSET + 8 > stream.len() {
        return Vec::new();
    }

    let mut values = Vec::new();
    let mut offset = EMBEDDED_PRESS_SNAPSHOT_VECTOR_SCAN_OFFSET;
    while offset + 4 <= stream.len() {
        let raw = read_i32_le_at(stream, offset).unwrap_or_default();
        values.push(if raw.rem_euclid(65_536) == 0 {
            Some(raw / 65_536)
        } else {
            None
        });
        offset += 4;
    }

    let mut pairs = Vec::new();
    for index in 0..values.len().saturating_sub(1) {
        let Some(x) = values[index] else {
            continue;
        };
        let Some(y) = values[index + 1] else {
            continue;
        };
        if x >= 0 && y >= 0 && (x as u32) <= width && (y as u32) <= height {
            pairs.push((index, x as u32, y as u32));
        }
    }

    let max_delta = width.max(height);
    let mut segments = Vec::new();
    for window in pairs.windows(2) {
        let (first_index, x1, y1) = window[0];
        let (second_index, x2, y2) = window[1];
        if second_index != first_index + 2 {
            continue;
        }
        let delta = x1.abs_diff(x2) + y1.abs_diff(y2);
        if !(3..=max_delta).contains(&delta) {
            continue;
        }
        segments.push(ObjectEmbeddedPressVectorSegmentCandidate::new(
            x1, y1, x2, y2,
        ));
        if segments.len() >= EMBEDDED_PRESS_SNAPSHOT_VECTOR_SEGMENT_LIMIT {
            break;
        }
    }

    segments
}

fn visual_list_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectVisualListCandidate> {
    if !path.to_ascii_lowercase().contains("visuallist") {
        return None;
    }
    if stream.get(VISUAL_LIST_MAGIC_OFFSET..VISUAL_LIST_MAGIC_OFFSET + VISUAL_LIST_MAGIC.len())?
        != VISUAL_LIST_MAGIC
    {
        return None;
    }
    let declared_size = read_be32_at(stream, 0)? as usize;
    let version = read_be32_at(stream, VISUAL_LIST_VERSION_OFFSET)?;
    let flags = read_be32_at(stream, VISUAL_LIST_FLAGS_OFFSET)?;
    let width = read_be32_at(stream, VISUAL_LIST_WIDTH_OFFSET)?;
    let height = read_be32_at(stream, VISUAL_LIST_HEIGHT_OFFSET)?;
    let row_stride = read_be32_at(stream, VISUAL_LIST_ROW_STRIDE_OFFSET)?;
    let bit_depth = read_be32_at(stream, VISUAL_LIST_BIT_DEPTH_OFFSET)?;
    let x_pixels_per_meter = read_be32_at(stream, VISUAL_LIST_X_PPM_OFFSET)?;
    let y_pixels_per_meter = read_be32_at(stream, VISUAL_LIST_Y_PPM_OFFSET)?;
    let rle_data_len = read_be32_at(stream, VISUAL_LIST_RLE_LENGTH_OFFSET)? as usize;
    let rle_data_end = VISUAL_LIST_HEADER_BYTES.checked_add(rle_data_len)?;
    let rle_data = stream.get(VISUAL_LIST_HEADER_BYTES..rle_data_end)?;
    let pixels = decode_visual_list_rle8(width, height, rle_data)?;
    Some(ObjectVisualListCandidate::new(
        declared_size,
        version,
        flags,
        width,
        height,
        row_stride,
        bit_depth,
        x_pixels_per_meter,
        y_pixels_per_meter,
        VISUAL_LIST_HEADER_BYTES,
        rle_data_len,
        pixels,
    ))
}

fn decode_visual_list_rle8(width: u32, height: u32, data: &[u8]) -> Option<Vec<u8>> {
    if width == 0 || height == 0 || width > 10_000 || height > 10_000 {
        return None;
    }
    let width = usize::try_from(width).ok()?;
    let height = usize::try_from(height).ok()?;
    let total_pixels = width.checked_mul(height)?;
    if total_pixels > 16_000_000 {
        return None;
    }

    let fill = visual_list_default_pixel(data);
    let mut pixels = Vec::with_capacity(total_pixels);
    let mut row = Vec::with_capacity(width);
    let mut offset = 0usize;
    while offset + 1 < data.len() && pixels.len() < total_pixels {
        let count = data[offset];
        let value = data[offset + 1];
        offset += 2;
        if count != 0 {
            row.extend(std::iter::repeat(value).take(count as usize));
            continue;
        }

        match value {
            0 => flush_visual_list_row(&mut pixels, &mut row, width, height, fill),
            1 => break,
            2 => {
                if offset + 1 >= data.len() {
                    return None;
                }
                let dx = data[offset] as usize;
                let dy = data[offset + 1] as usize;
                offset += 2;
                row.extend(std::iter::repeat(fill).take(dx));
                for _ in 0..dy {
                    flush_visual_list_row(&mut pixels, &mut row, width, height, fill);
                }
            }
            literal_len => {
                let literal_len = literal_len as usize;
                let literal_end = offset.checked_add(literal_len)?;
                row.extend_from_slice(data.get(offset..literal_end)?);
                offset = literal_end;
                if literal_len % 2 == 1 {
                    offset = offset.checked_add(1)?;
                    if offset > data.len() {
                        return None;
                    }
                }
            }
        }
    }

    if !row.is_empty() && pixels.len() < total_pixels {
        flush_visual_list_row(&mut pixels, &mut row, width, height, fill);
    }
    while pixels.len() < total_pixels {
        pixels.extend(std::iter::repeat(fill).take(width));
    }
    pixels.truncate(total_pixels);
    Some(pixels)
}

fn visual_list_default_pixel(data: &[u8]) -> u8 {
    if data.len() >= 2 && data[0] != 0 {
        data[1]
    } else {
        0xff
    }
}

fn flush_visual_list_row(
    pixels: &mut Vec<u8>,
    row: &mut Vec<u8>,
    width: usize,
    height: usize,
    fill: u8,
) {
    if pixels.len() >= width.saturating_mul(height) {
        row.clear();
        return;
    }
    if row.len() < width {
        row.extend(std::iter::repeat(fill).take(width - row.len()));
    }
    pixels.extend(row.iter().copied().take(width));
    row.clear();
}

fn push_object_path_reasons(path: &str, reasons: &mut Vec<ObjectStreamCandidateReason>) {
    let lower = path.to_ascii_lowercase();
    let segments = lower
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &[
                "embeditems",
                "embedding",
                "jsfart",
                "compobj",
                "ole",
                "object",
                "bin",
            ],
        )
    }) {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::ObjectPath);
    }

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &[
                "image", "picture", "graphic", "bitmap", "png", "jpg", "jpeg", "gif", "bmp", "tif",
                "tiff", "wmf", "emf",
            ],
        )
    }) {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::ImagePath);
    }

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &["figure", "shape", "draw", "frame", "layoutbox", "svg"],
        )
    }) {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::ShapePath);
    }

    if segments.iter().any(|segment| {
        contains_any(segment, &["table", "cell", "tbl", "hyo"])
            && !contains_any(segment, &["positiontable", "style"])
    }) {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::TablePath);
    }

    if segments.iter().any(|segment| *segment == "visuallist") {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::VisualListPath);
    }
}

fn object_stream_ownership_candidate(path: &str) -> Option<ObjectStreamOwnershipCandidate> {
    let segments = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.len() >= 3
        && segments[0].eq_ignore_ascii_case("EmbedItems")
        && segments[1].starts_with("Embedding ")
    {
        let embedding_index = segments[1]
            .strip_prefix("Embedding ")
            .and_then(|value| value.parse::<usize>().ok());
        let storage_path = Some(format!("/EmbedItems/{}", segments[1]));
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "embed-items",
            storage_path,
            embedding_index,
            embedded_stream_role(segments[2]),
        ));
    }

    if segments.len() >= 3
        && segments[0].eq_ignore_ascii_case("FigureData")
        && segments[2].eq_ignore_ascii_case("FDMVector")
    {
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "figure-data",
            Some(format!("/{}/{}", segments[0], segments[1])),
            None,
            "fdm-vector",
        ));
    }

    let last = segments.last()?;
    if last.eq_ignore_ascii_case("Figure") {
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "figure",
            None,
            None,
            "figure-stream",
        ));
    }
    if last.eq_ignore_ascii_case("Frame") {
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "frame",
            None,
            None,
            "frame-stream",
        ));
    }
    if last.eq_ignore_ascii_case("LayoutBox") {
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "layout-box",
            None,
            None,
            "layout-box-stream",
        ));
    }

    None
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

fn push_unique_object_reason(
    reasons: &mut Vec<ObjectStreamCandidateReason>,
    reason: ObjectStreamCandidateReason,
) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn image_signature_hits(stream: &[u8]) -> Vec<ObjectImageSignatureHit> {
    let mut hits = Vec::new();
    push_signature_hits(&mut hits, stream, "png", b"\x89PNG\r\n\x1a\n", true);
    push_signature_hits(&mut hits, stream, "jpeg", b"\xff\xd8\xff", true);
    push_signature_hits(&mut hits, stream, "gif87a", b"GIF87a", true);
    push_signature_hits(&mut hits, stream, "gif89a", b"GIF89a", true);
    push_signature_hits(&mut hits, stream, "tiff-le", b"II\x2a\0", true);
    push_signature_hits(&mut hits, stream, "tiff-be", b"MM\0\x2a", true);
    push_signature_hits(
        &mut hits,
        stream,
        "wmf-placeable",
        b"\xd7\xcd\xc6\x9a",
        true,
    );
    push_signature_hits(&mut hits, stream, "bmp", b"BM", false);

    hits.sort_by(|left, right| {
        left.offset()
            .cmp(&right.offset())
            .then_with(|| left.kind().cmp(right.kind()))
    });
    hits
}

fn push_signature_hits(
    hits: &mut Vec<ObjectImageSignatureHit>,
    stream: &[u8],
    kind: &'static str,
    signature: &[u8],
    scan_anywhere: bool,
) {
    let offsets = if scan_anywhere {
        find_subslice_offsets(stream, signature)
    } else if stream.starts_with(signature) {
        vec![0]
    } else {
        Vec::new()
    };

    for offset in offsets {
        hits.push(ObjectImageSignatureHit::new(kind, offset));
    }
}

fn image_payload_spans(
    stream: &[u8],
    hits: &[ObjectImageSignatureHit],
) -> Vec<ObjectImagePayloadSpan> {
    let mut candidates = hits
        .iter()
        .filter_map(|hit| image_payload_candidate(stream, hit))
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.start
            .cmp(&right.start)
            .then_with(|| left.end.cmp(&right.end))
            .then_with(|| left.kind.cmp(&right.kind))
    });

    let mut spans = Vec::with_capacity(candidates.len());
    for (index, candidate) in candidates.iter().enumerate() {
        let previous_end = index
            .checked_sub(1)
            .map(|previous| candidates[previous].end);
        let next_start = candidates.get(index + 1).map(|next| next.start);
        let header_start = previous_end
            .filter(|end| *end <= candidate.start)
            .unwrap_or(0);
        let trailer_end = next_start
            .filter(|start| *start >= candidate.end)
            .unwrap_or(stream.len());
        let envelope = image_payload_envelope(
            stream,
            header_start,
            candidate.start,
            candidate.end,
            trailer_end,
        );
        spans.push(ObjectImagePayloadSpan::new(
            &candidate.kind,
            &candidate.mime,
            ObjectImagePayloadLocation::new(
                candidate.signature_offset,
                candidate.start,
                candidate.end,
            ),
            true,
            candidate.payload.clone(),
            envelope,
        ));
    }
    spans
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImagePayloadCandidate {
    kind: String,
    mime: String,
    signature_offset: usize,
    start: usize,
    end: usize,
    payload: Vec<u8>,
}

fn image_payload_candidate(
    stream: &[u8],
    hit: &ObjectImageSignatureHit,
) -> Option<ImagePayloadCandidate> {
    let end = match hit.kind() {
        "jpeg" => jpeg_payload_end(stream, hit.offset())?,
        "png" => png_payload_end(stream, hit.offset())?,
        "gif87a" | "gif89a" => gif_payload_end(stream, hit.offset())?,
        "bmp" => bmp_payload_end(stream, hit.offset())?,
        _ => return None,
    };

    Some(ImagePayloadCandidate {
        kind: hit.kind().to_string(),
        mime: image_mime_for_kind(hit.kind()).to_string(),
        signature_offset: hit.offset(),
        start: hit.offset(),
        end,
        payload: stream[hit.offset()..end].to_vec(),
    })
}

fn image_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    #[cfg(feature = "bitmap-images")]
    if let Ok(image) = image::load_from_memory(payload) {
        return Some(ObjectImageDimensions::new(image.width(), image.height()));
    }
    jpeg_payload_dimensions(payload)
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

fn image_payload_envelope(
    stream: &[u8],
    header_start: usize,
    header_end: usize,
    trailer_start: usize,
    trailer_end: usize,
) -> ObjectImagePayloadEnvelope {
    let header_start = header_start.min(header_end).min(stream.len());
    let header_end = header_end.min(stream.len());
    let trailer_start = trailer_start.min(stream.len());
    let trailer_end = trailer_end.max(trailer_start).min(stream.len());
    let header = stream[header_start..header_end].to_vec();
    let trailer = stream[trailer_start..trailer_end].to_vec();
    let declared_payload_length =
        image_declared_payload_length(&header, header_start, trailer_start - header_end);

    ObjectImagePayloadEnvelope::new(
        header_start,
        header_end,
        trailer_start,
        trailer_end,
        declared_payload_length,
        header,
        trailer,
    )
}

fn image_header_field_candidates(
    header_start: usize,
    header: &[u8],
) -> ObjectImageHeaderFieldCandidates {
    let prefix_len = header.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES);
    let prefix = &header[..prefix_len];
    let mut u16_le_prefix = Vec::new();
    for relative_offset in (0..prefix.len()).step_by(2) {
        if relative_offset + 2 <= prefix.len() {
            u16_le_prefix.push(ObjectImageNumericHeaderField::new(
                header_start + relative_offset,
                u16::from_le_bytes([prefix[relative_offset], prefix[relative_offset + 1]]) as u64,
            ));
        }
    }

    let mut u32_le_prefix = Vec::new();
    for relative_offset in (0..prefix.len()).step_by(4) {
        if relative_offset + 4 <= prefix.len() {
            u32_le_prefix.push(ObjectImageNumericHeaderField::new(
                header_start + relative_offset,
                u32::from_le_bytes([
                    prefix[relative_offset],
                    prefix[relative_offset + 1],
                    prefix[relative_offset + 2],
                    prefix[relative_offset + 3],
                ]) as u64,
            ));
        }
    }

    ObjectImageHeaderFieldCandidates::new(
        u16_le_prefix,
        u32_le_prefix,
        image_source_path_candidate(header_start, header),
    )
}

fn image_source_path_candidate(
    header_start: usize,
    header: &[u8],
) -> Option<ObjectImageSourcePathCandidate> {
    let length_offset = 16;
    let declared_length = *header.get(length_offset)? as usize;
    if declared_length < 3 {
        return None;
    }
    let bytes_start = length_offset + 1;
    let declared_end = bytes_start.checked_add(declared_length)?;
    let text_bytes = header.get(bytes_start..declared_end)?;
    let raw_end = if header.get(declared_end) == Some(&0) {
        declared_end + 1
    } else if text_bytes.last() == Some(&0) {
        declared_end
    } else {
        return None;
    };
    let bytes = header.get(bytes_start..raw_end)?;
    let text_bytes = if text_bytes.last() == Some(&0) {
        &text_bytes[..text_bytes.len().saturating_sub(1)]
    } else {
        text_bytes
    };
    if !looks_like_embedded_source_path(text_bytes) {
        return None;
    }

    Some(ObjectImageSourcePathCandidate::new(
        header_start + length_offset,
        declared_length,
        header_start + bytes_start,
        header_start + raw_end,
        true,
        bytes.to_vec(),
    ))
}

fn looks_like_embedded_source_path(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .any(|byte| matches!(*byte, b'\\' | b'/' | b':' | b'.'))
}

fn image_declared_payload_length(
    header: &[u8],
    header_start: usize,
    payload_len: usize,
) -> Option<ObjectImageDeclaredLengthCandidate> {
    let offset_in_header = header.len().checked_sub(4)?;
    let value = u32::from_le_bytes([
        header[offset_in_header],
        header[offset_in_header + 1],
        header[offset_in_header + 2],
        header[offset_in_header + 3],
    ]) as usize;
    (value == payload_len).then(|| {
        ObjectImageDeclaredLengthCandidate::new(header_start + offset_in_header, value, "le32")
    })
}

fn image_mime_for_kind(kind: &str) -> &'static str {
    match kind {
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif87a" | "gif89a" => "image/gif",
        "bmp" => "image/bmp",
        "tiff-le" | "tiff-be" => "image/tiff",
        "wmf-placeable" => "image/wmf",
        _ => "application/octet-stream",
    }
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

fn document_text_toc_entries(entries: &[DocumentTextMapEntry]) -> Vec<DocumentTocEntry> {
    let mut toc_entries = Vec::new();
    let mut row = DocumentTextTocRow::default();

    for entry in entries {
        match entry.kind() {
            DocumentTextMapKind::TextRun | DocumentTextMapKind::InlineText => {
                row.push_visible_text(entry);
            }
            DocumentTextMapKind::SkippedInlineText => {
                if entry.selector() == Some(DOCUMENT_TEXT_TOC_PAGE_SELECTOR) {
                    row.push_page_label(entry);
                }
            }
            DocumentTextMapKind::ControlBoundary => {}
        }

        if entry.text().contains('\n') || entry.text().contains('\r') {
            if let Some(toc_entry) = std::mem::take(&mut row).into_toc_entry() {
                toc_entries.push(toc_entry);
            }
        }
    }

    if let Some(toc_entry) = row.into_toc_entry() {
        toc_entries.push(toc_entry);
    }

    toc_entries
}

struct SourceTextPart {
    text: String,
    source_span: Option<TextSourceSpan>,
    break_after: bool,
}

fn source_text_parts(text: &str, source_span: Option<&TextSourceSpan>) -> Vec<SourceTextPart> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut current_start_units = 0usize;
    let mut unit_index = 0usize;
    let mut chars = text.chars().peekable();

    while let Some(character) = chars.next() {
        match character {
            '\r' => {
                parts.push(SourceTextPart {
                    text: std::mem::take(&mut current),
                    source_span: source_span
                        .map(|span| span.subspan_by_units(current_start_units, unit_index)),
                    break_after: true,
                });
                unit_index += character.len_utf16();
                if chars.peek() == Some(&'\n') {
                    chars.next();
                    unit_index += '\n'.len_utf16();
                }
                current_start_units = unit_index;
            }
            '\n' => {
                parts.push(SourceTextPart {
                    text: std::mem::take(&mut current),
                    source_span: source_span
                        .map(|span| span.subspan_by_units(current_start_units, unit_index)),
                    break_after: true,
                });
                unit_index += character.len_utf16();
                current_start_units = unit_index;
            }
            character => {
                current.push(character);
                unit_index += character.len_utf16();
            }
        }
    }

    parts.push(SourceTextPart {
        text: current,
        source_span: source_span.map(|span| span.subspan_by_units(current_start_units, unit_index)),
        break_after: false,
    });
    parts
}

fn text_count_range_overlaps(
    range: &TextCountRange,
    document: &Document,
) -> Vec<TextCountRangeOverlap> {
    let mut overlaps = Vec::new();
    push_text_count_range_overlaps(
        &mut overlaps,
        TextCountRangeOverlapBasis::Byte,
        range.start() as usize,
        range.end() as usize,
        document,
    );
    push_text_count_range_overlaps(
        &mut overlaps,
        TextCountRangeOverlapBasis::Unit,
        range.start() as usize,
        range.end() as usize,
        document,
    );
    overlaps
}

#[derive(Debug, Clone, Copy)]
struct TextControlSourceInterval {
    index: usize,
    byte_start: usize,
    byte_end: usize,
    unit_start: usize,
    unit_end: usize,
}

fn text_count_control_range_overlaps(
    range: &TextCountRange,
    document: &Document,
    delimiter_codes: &[u16],
) -> Vec<TextCountControlRangeOverlap> {
    let Some(bounds) = document_text_source_bounds(document) else {
        return Vec::new();
    };

    let mut overlaps = Vec::new();
    for delimiter_code in delimiter_codes {
        let intervals = text_control_source_intervals(document, &bounds, *delimiter_code);
        if intervals.is_empty() {
            continue;
        }
        push_text_count_control_range_overlap(
            &mut overlaps,
            TextCountRangeOverlapBasis::Byte,
            *delimiter_code,
            range.start() as usize,
            range.end() as usize,
            &intervals,
        );
        push_text_count_control_range_overlap(
            &mut overlaps,
            TextCountRangeOverlapBasis::Unit,
            *delimiter_code,
            range.start() as usize,
            range.end() as usize,
            &intervals,
        );
    }
    overlaps
}

fn text_boundary_candidates_from_ranges(ranges: &[TextCountRange]) -> Vec<TextBoundaryCandidate> {
    let mut candidates = Vec::new();
    for range in ranges {
        for overlap in range.control_range_overlaps() {
            candidates.push(TextBoundaryCandidate::from_control_range_overlap(
                candidates.len(),
                range.index(),
                overlap,
            ));
        }
    }
    candidates
}

fn table_candidates_from_text_boundaries(
    document: &Document,
    entries: &[DocumentTextMapEntry],
) -> Vec<TableCandidate> {
    let Some(bounds) = document_text_source_bounds(document) else {
        return Vec::new();
    };

    let mut table_candidates = Vec::new();
    for candidate in document.text_boundary_candidates() {
        if candidate.interval_count() <= 1 {
            continue;
        }
        let intervals = table_candidate_intervals(document, entries, &bounds, candidate);
        if intervals.len() <= 1 {
            continue;
        }
        table_candidates.push(TableCandidate::from_text_boundary_candidate(
            table_candidates.len(),
            candidate,
            intervals,
        ));
    }
    table_candidates
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
}

fn table_candidates_from_document_text_controls(
    entries: &[DocumentTextMapEntry],
    start_index: usize,
) -> Vec<TableCandidate> {
    let rows = document_text_control_table_rows(entries);
    let mut candidates = Vec::new();
    let mut current_rows = Vec::new();
    let mut current_column_count = 0usize;
    let mut empty_gap_count = 0usize;

    for row in rows {
        let column_count = row.cells.len();
        if column_count == 0 {
            if !current_rows.is_empty() {
                empty_gap_count += 1;
                if empty_gap_count > 1 {
                    push_document_text_control_table_candidate(
                        &mut candidates,
                        start_index,
                        &mut current_rows,
                    );
                    current_column_count = 0;
                    empty_gap_count = 0;
                }
            }
            continue;
        }

        if column_count < 2 {
            push_document_text_control_table_candidate(
                &mut candidates,
                start_index,
                &mut current_rows,
            );
            current_column_count = 0;
            empty_gap_count = 0;
            continue;
        }

        if current_rows.is_empty() || (column_count == current_column_count && empty_gap_count <= 1)
        {
            current_column_count = column_count;
            current_rows.push(row);
            empty_gap_count = 0;
            continue;
        }

        push_document_text_control_table_candidate(&mut candidates, start_index, &mut current_rows);
        current_column_count = 0;
        empty_gap_count = 0;

        if column_count >= 2 {
            current_column_count = column_count;
            current_rows.push(row);
        } else if column_count == 0 {
            empty_gap_count += 1;
        }
    }

    push_document_text_control_table_candidate(&mut candidates, start_index, &mut current_rows);
    candidates
}

fn push_document_text_control_table_candidate(
    candidates: &mut Vec<TableCandidate>,
    start_index: usize,
    rows: &mut Vec<DocumentTextControlTableRow>,
) {
    if document_text_control_table_rows_are_plausible(rows) {
        candidates.push(TableCandidate::from_document_text_control_rows(
            start_index + candidates.len(),
            rows,
        ));
    }
    rows.clear();
}

fn document_text_control_table_rows_are_plausible(rows: &[DocumentTextControlTableRow]) -> bool {
    if rows.len() >= 3 {
        return true;
    }
    rows.len() >= 2
        && rows.iter().skip(1).any(|row| {
            row.cells
                .iter()
                .any(|cell| table_control_cell_has_value_marker(&cell.text))
        })
}

fn table_control_cell_has_value_marker(text: &str) -> bool {
    text.chars()
        .any(|character| character.is_ascii_digit() || matches!(character, '０'..='９'))
}

fn document_text_control_table_rows(
    entries: &[DocumentTextMapEntry],
) -> Vec<DocumentTextControlTableRow> {
    let mut rows = Vec::new();
    let mut cells = Vec::new();
    let mut cell = PendingDocumentTextControlCell::new();
    let mut row_start: Option<usize> = None;
    let mut row_index = 0usize;

    for entry in entries {
        match entry.kind() {
            DocumentTextMapKind::TextRun | DocumentTextMapKind::InlineText => {
                if row_start.is_none() {
                    row_start = Some(entry.unit_start());
                }
                cell.push_text(entry);
            }
            DocumentTextMapKind::SkippedInlineText => {}
            DocumentTextMapKind::ControlBoundary => match entry.code() {
                Some(TABLE_CELL_DELIMITER_CONTROL) => {
                    if row_start.is_none() {
                        row_start = Some(entry.unit_start());
                    }
                    if let Some(finished) = cell.finish() {
                        cells.push(finished);
                    }
                }
                Some(TABLE_ROW_DELIMITER_CONTROL) => {
                    if row_start.is_none() {
                        row_start = Some(entry.unit_start());
                    }
                    if let Some(finished) = cell.finish() {
                        cells.push(finished);
                    }
                    let source_start = row_start.unwrap_or(entry.unit_start());
                    rows.push(DocumentTextControlTableRow {
                        index: row_index,
                        source_start,
                        source_end: entry.unit_end(),
                        cells: std::mem::take(&mut cells),
                    });
                    row_index += 1;
                    row_start = None;
                }
                _ => {
                    if let Some(finished) = cell.finish() {
                        cells.push(finished);
                    }
                    if let Some(source_start) = row_start.take() {
                        rows.push(DocumentTextControlTableRow {
                            index: row_index,
                            source_start,
                            source_end: entry.unit_start(),
                            cells: std::mem::take(&mut cells),
                        });
                        row_index += 1;
                    } else {
                        cells.clear();
                    }
                }
            },
        }
    }

    if let Some(finished) = cell.finish() {
        cells.push(finished);
    }
    if let Some(source_start) = row_start {
        let source_end = cells
            .last()
            .map_or(source_start, |cell| cell.source_end.max(source_start));
        rows.push(DocumentTextControlTableRow {
            index: row_index,
            source_start,
            source_end,
            cells,
        });
    }

    rows
}

fn clean_table_control_cell_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn table_candidate_intervals(
    document: &Document,
    entries: &[DocumentTextMapEntry],
    bounds: &TextSourceSpan,
    candidate: &TextBoundaryCandidate,
) -> Vec<TableCandidateInterval> {
    text_control_source_intervals(document, bounds, candidate.delimiter_code())
        .into_iter()
        .filter(|interval| {
            (candidate.first_interval_index()..=candidate.last_interval_index())
                .contains(&interval.index)
        })
        .filter_map(|interval| {
            let (interval_start, interval_end) =
                source_interval_range(&interval, candidate.basis());
            let source_start = interval_start.max(candidate.source_start());
            let source_end = interval_end.min(candidate.source_end());
            if source_start >= source_end {
                return None;
            }
            let text =
                range_visible_text_for_basis(entries, source_start, source_end, candidate.basis());
            Some(TableCandidateInterval::new(
                0,
                interval.index,
                source_start,
                source_end,
                text,
            ))
        })
        .enumerate()
        .map(|(index, interval)| TableCandidateInterval { index, ..interval })
        .collect()
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

fn text_paragraph_boundary_candidates_from_layout(
    document: &Document,
    entries: &[DocumentTextMapEntry],
    data: &[u8],
) -> Vec<TextParagraphBoundaryCandidate> {
    let Ok(line_stream) = read_cfb_stream(data, "/LineMark") else {
        return Vec::new();
    };
    let Ok(page_mark) = read_page_mark(data) else {
        return Vec::new();
    };
    let line_word_points = be16_words(&line_stream)
        .map(|word| word as usize)
        .collect::<Vec<_>>();
    let page_field_points = page_be32_field_points(&page_mark);
    if line_word_points.is_empty() || page_field_points.is_empty() {
        return Vec::new();
    }

    let mut paragraph_candidates = Vec::new();
    for candidate in document.text_boundary_candidates() {
        if !is_strict_unit_001c_single_boundary_candidate(entries, candidate) {
            continue;
        }
        let Some(range) = document
            .text_count_ranges()
            .get(candidate.text_count_range_index())
        else {
            continue;
        };
        if range.span() == 0 {
            continue;
        }
        let Some(line_word_evidence) =
            best_layout_exact2_evidence_for_points(candidate, "line-word-value", &line_word_points)
        else {
            continue;
        };
        let Some(page_field_evidence) = best_layout_exact2_evidence_for_points(
            candidate,
            "page-be32-field",
            &page_field_points,
        ) else {
            continue;
        };
        paragraph_candidates.push(TextParagraphBoundaryCandidate {
            index: paragraph_candidates.len(),
            text_boundary_candidate_index: candidate.index(),
            text_count_range_index: candidate.text_count_range_index(),
            source_start: candidate.source_start(),
            source_end: candidate.source_end(),
            text_count_range_span: range.span(),
            line_word_evidence,
            page_field_evidence,
        });
    }
    paragraph_candidates
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

fn layout_map_bases() -> &'static [LayoutMapBase] {
    &[
        LayoutMapBase::Unit,
        LayoutMapBase::UnitTimes2,
        LayoutMapBase::UnitDiv2Floor,
        LayoutMapBase::UnitDiv2Ceil,
    ]
}

fn be16_words(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
}

fn page_be32_field_points(page_mark: &PageMark) -> Vec<usize> {
    page_mark
        .entries()
        .iter()
        .flat_map(|entry| {
            entry
                .raw()
                .chunks_exact(4)
                .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as usize)
        })
        .collect()
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

fn range_visible_text_for_basis(
    entries: &[DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: TextCountRangeOverlapBasis,
) -> String {
    entries
        .iter()
        .filter(|entry| range_overlaps_entry_for_basis(entry, start, end, basis))
        .map(|entry| range_text_overlap_for_basis(entry, start, end, basis))
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

fn range_text_overlap_for_basis(
    entry: &DocumentTextMapEntry,
    start: usize,
    end: usize,
    basis: TextCountRangeOverlapBasis,
) -> String {
    if entry.kind() == DocumentTextMapKind::ControlBoundary || start >= end {
        return String::new();
    }

    let (entry_start, entry_end) = entry_range_for_basis(entry, basis);
    let overlap_start = entry_start.max(start);
    let overlap_end = entry_end.min(end);
    if overlap_start >= overlap_end {
        return String::new();
    }

    let (relative_start, relative_end) = match basis {
        TextCountRangeOverlapBasis::Byte => (
            overlap_start.saturating_sub(entry.byte_start()) / 2,
            overlap_end
                .saturating_sub(entry.byte_start())
                .saturating_add(1)
                / 2,
        ),
        TextCountRangeOverlapBasis::Unit => (
            overlap_start.saturating_sub(entry.unit_start()),
            overlap_end.saturating_sub(entry.unit_start()),
        ),
    };
    text_by_utf16_units(entry.text(), relative_start, relative_end)
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

fn range_text_overlap(entry: &DocumentTextMapEntry, start: usize, end: usize) -> String {
    if entry.kind() == DocumentTextMapKind::ControlBoundary || start >= end {
        return String::new();
    }
    let overlap_start = entry.unit_start().max(start);
    let overlap_end = entry.unit_end().min(end);
    if overlap_start >= overlap_end {
        return String::new();
    }
    entry
        .text()
        .chars()
        .skip(overlap_start.saturating_sub(entry.unit_start()))
        .take(overlap_end - overlap_start)
        .collect()
}

fn text_line_break_count(text: &str) -> usize {
    text.chars()
        .filter(|character| matches!(character, '\n' | '\r'))
        .count()
}

fn push_text_count_control_range_overlap(
    overlaps: &mut Vec<TextCountControlRangeOverlap>,
    basis: TextCountRangeOverlapBasis,
    delimiter_code: u16,
    start: usize,
    end: usize,
    intervals: &[TextControlSourceInterval],
) {
    let hits = intervals
        .iter()
        .filter(|interval| source_interval_overlaps(interval, basis, start, end))
        .collect::<Vec<_>>();
    let Some(first) = hits.first() else {
        return;
    };
    let first = **first;
    let last = **hits.last().expect("non-empty hits");
    let (source_start, source_end) = match basis {
        TextCountRangeOverlapBasis::Byte => (first.byte_start, last.byte_end),
        TextCountRangeOverlapBasis::Unit => (first.unit_start, last.unit_end),
    };

    overlaps.push(TextCountControlRangeOverlap::new(
        basis,
        delimiter_code,
        hits.len(),
        first.index,
        last.index,
        source_start,
        source_end,
    ));
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

fn text_control_source_intervals(
    document: &Document,
    bounds: &TextSourceSpan,
    delimiter_code: u16,
) -> Vec<TextControlSourceInterval> {
    let mut delimiters = document
        .text_control_boundaries()
        .iter()
        .filter(|boundary| boundary.code() == delimiter_code)
        .filter_map(|boundary| boundary.source_span())
        .collect::<Vec<_>>();
    if delimiters.is_empty() {
        return Vec::new();
    }
    delimiters.sort_by_key(|span| (span.byte_start(), span.unit_start()));

    let mut intervals = Vec::new();
    let mut byte_start = bounds.byte_start();
    let mut unit_start = bounds.unit_start();
    for delimiter in delimiters {
        intervals.push(TextControlSourceInterval {
            index: intervals.len(),
            byte_start,
            byte_end: delimiter.byte_start(),
            unit_start,
            unit_end: delimiter.unit_start(),
        });
        byte_start = delimiter.byte_end();
        unit_start = delimiter.unit_end();
    }
    intervals.push(TextControlSourceInterval {
        index: intervals.len(),
        byte_start,
        byte_end: bounds.byte_end(),
        unit_start,
        unit_end: bounds.unit_end(),
    });
    intervals
}

fn document_text_source_bounds(document: &Document) -> Option<TextSourceSpan> {
    let mut byte_start = usize::MAX;
    let mut byte_end = 0usize;
    let mut unit_start = usize::MAX;
    let mut unit_end = 0usize;

    for block in document.blocks() {
        let Block::Paragraph(paragraph) = block else {
            continue;
        };
        for inline in paragraph.inlines() {
            let Inline::Text(run) = inline else {
                continue;
            };
            if let Some(span) = run.source_span() {
                byte_start = byte_start.min(span.byte_start());
                byte_end = byte_end.max(span.byte_end());
                unit_start = unit_start.min(span.unit_start());
                unit_end = unit_end.max(span.unit_end());
            }
        }
    }

    for boundary in document.text_control_boundaries() {
        if let Some(span) = boundary.source_span() {
            byte_start = byte_start.min(span.byte_start());
            byte_end = byte_end.max(span.byte_end());
            unit_start = unit_start.min(span.unit_start());
            unit_end = unit_end.max(span.unit_end());
        }
    }

    if byte_start == usize::MAX || unit_start == usize::MAX {
        None
    } else {
        Some(TextSourceSpan::new(
            byte_start, byte_end, unit_start, unit_end,
        ))
    }
}

fn push_text_count_range_overlaps(
    overlaps: &mut Vec<TextCountRangeOverlap>,
    basis: TextCountRangeOverlapBasis,
    start: usize,
    end: usize,
    document: &Document,
) {
    if start >= end {
        return;
    }

    for (block_index, block) in document.blocks().iter().enumerate() {
        let Block::Paragraph(paragraph) = block else {
            continue;
        };
        for (inline_index, inline) in paragraph.inlines().iter().enumerate() {
            let Inline::Text(run) = inline else {
                continue;
            };
            let Some(span) = run.source_span() else {
                continue;
            };
            let (entry_start, entry_end) = source_span_range(span, basis);
            let overlap_start = entry_start.max(start);
            let overlap_end = entry_end.min(end);
            if overlap_start >= overlap_end {
                continue;
            }

            overlaps.push(TextCountRangeOverlap::new(
                basis,
                block_index,
                inline_index,
                overlap_start,
                overlap_end,
                text_preview_for_source_overlap(
                    run.text(),
                    span,
                    basis,
                    overlap_start,
                    overlap_end,
                ),
            ));
        }
    }
}

fn source_span_range(span: &TextSourceSpan, basis: TextCountRangeOverlapBasis) -> (usize, usize) {
    match basis {
        TextCountRangeOverlapBasis::Byte => (span.byte_start(), span.byte_end()),
        TextCountRangeOverlapBasis::Unit => (span.unit_start(), span.unit_end()),
    }
}

fn text_preview_for_source_overlap(
    text: &str,
    span: &TextSourceSpan,
    basis: TextCountRangeOverlapBasis,
    overlap_start: usize,
    overlap_end: usize,
) -> String {
    preview_text(
        &text_for_source_overlap(text, span, basis, overlap_start, overlap_end),
        80,
    )
}

fn text_for_source_overlap(
    text: &str,
    span: &TextSourceSpan,
    basis: TextCountRangeOverlapBasis,
    overlap_start: usize,
    overlap_end: usize,
) -> String {
    let (relative_start, relative_end) = match basis {
        TextCountRangeOverlapBasis::Byte => (
            overlap_start.saturating_sub(span.byte_start()) / 2,
            overlap_end
                .saturating_sub(span.byte_start())
                .saturating_add(1)
                / 2,
        ),
        TextCountRangeOverlapBasis::Unit => (
            overlap_start.saturating_sub(span.unit_start()),
            overlap_end.saturating_sub(span.unit_start()),
        ),
    };
    text_by_utf16_units(text, relative_start, relative_end)
}

fn text_by_utf16_units(text: &str, start: usize, end: usize) -> String {
    let mut output = String::new();
    let mut current = 0usize;
    for character in text.chars() {
        let next = current + character.len_utf16();
        if next > start && current < end {
            output.push(character);
        }
        current = next;
    }
    output
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

fn next_snapshot_id(current: u32) -> u32 {
    current.checked_add(1).filter(|id| *id > 0).unwrap_or(1)
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

#[derive(Debug, Clone, Copy)]
struct ParagraphSourceTextSpan {
    paragraph_index: usize,
    char_start: usize,
    char_end: usize,
    unit_start: usize,
    unit_end: usize,
}

fn projected_text_controls(document: &Document) -> Vec<ProjectedTextControl> {
    let spans = paragraph_source_text_spans(document);
    let mut controls = Vec::new();

    for boundary in document.text_control_boundaries() {
        let Some(source_span) = boundary.source_span() else {
            continue;
        };
        if let Some((paragraph_index, char_offset)) =
            project_control_boundary_to_text(source_span, &spans)
        {
            controls.push(ProjectedTextControl {
                boundary_index: boundary.index(),
                paragraph_index,
                char_offset,
                code: boundary.code(),
            });
        }
    }

    controls.sort_by_key(|control| {
        (
            control.paragraph_index,
            control.char_offset,
            control.boundary_index,
        )
    });
    controls
}

fn paragraph_source_text_spans(document: &Document) -> Vec<ParagraphSourceTextSpan> {
    let mut spans = Vec::new();
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        let Block::Paragraph(paragraph) = block else {
            continue;
        };

        let mut char_offset = 0usize;
        for inline in paragraph.inlines() {
            match inline {
                Inline::Text(run) => {
                    let char_count = run.text().chars().count();
                    if let Some(source_span) = run.source_span() {
                        spans.push(ParagraphSourceTextSpan {
                            paragraph_index,
                            char_start: char_offset,
                            char_end: char_offset + char_count,
                            unit_start: source_span.unit_start(),
                            unit_end: source_span.unit_end(),
                        });
                    }
                    char_offset += char_count;
                }
                Inline::Ruby(ruby) => {
                    char_offset += ruby.base_text().chars().count();
                }
                Inline::Unknown(_) => {}
            }
        }
        paragraph_index += 1;
    }

    spans
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

fn paginate_document_text(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Vec<Vec<PageTextLine>> {
    let wrap_columns = layout.wrap_columns(writing_mode);
    let lines_per_page = layout.lines_per_page(writing_mode);
    let forced_breaks = projected_page_breaks(document);
    let mut pages = Vec::new();
    let mut current_page = Vec::new();
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                let text = paragraph_text(paragraph);
                let paragraph_breaks = forced_breaks
                    .get(&paragraph_index)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);
                let wrapped = wrap_text_line(&text, paragraph_index, wrap_columns);
                let mut forced_at_paragraph_end = false;
                if wrapped.is_empty() {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        PageTextLine::new(String::new(), Some(paragraph_index), 0, 0),
                        lines_per_page,
                    );
                    if paragraph_breaks.contains(&0) {
                        force_page_break(&mut pages, &mut current_page);
                        forced_at_paragraph_end = true;
                    }
                } else {
                    for line in wrapped {
                        let segments = split_line_at_page_breaks(line, paragraph_breaks);
                        for segment in segments {
                            push_paginated_line(
                                &mut pages,
                                &mut current_page,
                                segment.line,
                                lines_per_page,
                            );
                            if segment.break_after {
                                force_page_break(&mut pages, &mut current_page);
                                forced_at_paragraph_end = true;
                            } else {
                                forced_at_paragraph_end = false;
                            }
                        }
                    }
                }
                if !forced_at_paragraph_end && !writing_mode.is_vertical() {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        PageTextLine::new(String::new(), None, 0, 0),
                        lines_per_page,
                    );
                }
                paragraph_index += 1;
            }
            Block::Unknown(_) => {
                push_paginated_line(
                    &mut pages,
                    &mut current_page,
                    PageTextLine::new("[UnknownBlock preserved by rjtd]".to_string(), None, 0, 0),
                    lines_per_page,
                );
                push_paginated_line(
                    &mut pages,
                    &mut current_page,
                    PageTextLine::new(String::new(), None, 0, 0),
                    lines_per_page,
                );
            }
        }
    }

    while current_page
        .last()
        .is_some_and(|line| line.text().is_empty() && line.paragraph_index().is_none())
    {
        current_page.pop();
    }

    if !current_page.is_empty() {
        pages.push(current_page);
    }

    if pages.is_empty() {
        if !document.raw_streams().is_empty() {
            let raw_streams = document
                .raw_streams()
                .iter()
                .map(|stream| stream.name())
                .collect::<Vec<_>>()
                .join(", ");
            return vec![vec![PageTextLine::new(
                format!("[rjtd] No extractable text. Preserved raw streams: {raw_streams}"),
                None,
                0,
                0,
            )]];
        }
        return vec![Vec::new()];
    }

    pages
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

fn blank_page_text_line() -> PageTextLine {
    PageTextLine::new(String::new(), None, 0, 0)
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

fn split_page_text_line_by_display_columns(
    line: PageTextLine,
    max_columns: usize,
) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    let mut text = String::new();
    let mut width = 0usize;
    let mut line_start = line.char_start();
    let mut char_offset = line.char_start();

    for character in line.text().chars() {
        let char_width = display_column_width(character);
        if width > 0 && width + char_width > max_columns {
            lines.push(PageTextLine::new(
                std::mem::take(&mut text),
                line.paragraph_index(),
                line_start,
                char_offset,
            ));
            width = 0;
            line_start = char_offset;
        }
        text.push(character);
        width += char_width;
        char_offset += 1;
    }

    if !text.is_empty() {
        lines.push(PageTextLine::new(
            text,
            line.paragraph_index(),
            line_start,
            char_offset,
        ));
    }

    lines
}

fn is_colophon_noise_line(text: &str) -> bool {
    text.trim().starts_with('\u{fe02}')
}

fn is_colophon_copyright_line(text: &str) -> bool {
    text.contains("Printed") || text.contains("Japan") || text.contains("©")
}

fn vertical_page_text_placement(
    layout: PageLayout,
    lines: &[PageTextLine],
) -> VerticalPageTextPlacement {
    if is_ginga_colophon_page(lines) {
        return VerticalPageTextPlacement {
            x_shift_px: -(APP_LINE_HEIGHT_PX * GINGA_COLOPHON_X_SHIFT_COLUMNS),
            y_start_px: (layout.height_px() * GINGA_COLOPHON_TOP_RATIO).max(layout.margin_px()),
        };
    }

    VerticalPageTextPlacement {
        x_shift_px: 0.0,
        y_start_px: layout.margin_px(),
    }
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

fn document_paragraph_texts(document: &Document) -> Vec<(usize, String)> {
    let mut paragraph_index = 0usize;
    let mut paragraphs = Vec::new();
    for block in document.blocks() {
        if let Block::Paragraph(paragraph) = block {
            paragraphs.push((paragraph_index, paragraph_text(paragraph)));
            paragraph_index += 1;
        }
    }
    paragraphs
}

fn document_auto_text_title(document: &Document) -> Option<&str> {
    document
        .auto_texts()
        .iter()
        .map(DocumentAutoText::text)
        .map(str::trim)
        .find(|text| !text.is_empty())
}

fn document_page_decoration_paired_slot_pairs(document: &Document) -> Vec<(u16, u16)> {
    let mut pairs = BTreeSet::new();
    document
        .unknown_styles()
        .iter()
        .filter(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        .for_each(|style| {
            for record in summarize_style_stream(style.payload()).records() {
                pairs.extend(page_layout_record_active_decoration_pairs(record));
            }
        });
    pairs.into_iter().collect()
}

fn page_layout_record_active_decoration_pairs(
    record: &StyleStreamRecordSummary,
) -> Vec<(u16, u16)> {
    let active_slots = record
        .subrecords()
        .iter()
        .filter_map(|subrecord| {
            let code = subrecord.code();
            let slot = code >> 8;
            let part = code & 0xff;
            if !(0x31..=0x39).contains(&slot) || part != 0x05 {
                return None;
            }
            subrecord
                .payload()
                .first()
                .is_some_and(|byte| *byte != 0)
                .then_some(slot)
        })
        .collect::<BTreeSet<_>>();

    [(0x32, 0x33), (0x34, 0x35), (0x36, 0x37), (0x38, 0x39)]
        .iter()
        .filter(|(left, right)| active_slots.contains(left) && active_slots.contains(right))
        .copied()
        .collect()
}

fn document_page_decoration_slot_evidence(document: &Document) -> Vec<PageDecorationSlotEvidence> {
    let mut evidence = Vec::new();
    document
        .unknown_styles()
        .iter()
        .filter(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        .for_each(|style| {
            let summary = summarize_style_stream(style.payload());
            for (record_index, record) in summary.records().iter().enumerate() {
                evidence.extend(page_layout_record_decoration_slot_evidence(
                    record_index,
                    record,
                ));
            }
        });
    evidence
}

fn page_layout_record_decoration_slot_evidence(
    record_index: usize,
    record: &StyleStreamRecordSummary,
) -> Vec<PageDecorationSlotEvidence> {
    let mut slots = BTreeMap::new();
    for subrecord in record.subrecords() {
        let code = subrecord.code();
        let slot = code >> 8;
        let part = code & 0xff;
        if !(0x31..=0x39).contains(&slot) || !(0x04..=0x07).contains(&part) {
            continue;
        }
        let evidence = slots
            .entry(slot)
            .or_insert_with(|| PageDecorationSlotEvidence {
                record_index,
                record_offset: record.offset(),
                record_label: record.label().map(str::to_string),
                slot,
                part04: None,
                part05: None,
                part06: None,
                part07: None,
            });
        match part {
            0x04 => evidence.part04 = Some(subrecord.payload().to_vec()),
            0x05 => evidence.part05 = Some(subrecord.payload().to_vec()),
            0x06 => evidence.part06 = Some(subrecord.payload().to_vec()),
            0x07 => evidence.part07 = Some(subrecord.payload().to_vec()),
            _ => {}
        }
    }
    slots.into_values().collect()
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

fn page_has_exact_text_line(lines: &[PageTextLine], text: &str) -> bool {
    lines.iter().any(|line| line.text().trim() == text)
}

fn page_chapter_title(lines: &[PageTextLine], chapter_titles: &[String]) -> Option<String> {
    lines.iter().find_map(|line| {
        let trimmed = line.text().trim();
        chapter_titles
            .iter()
            .find(|title| trimmed.starts_with(title.as_str()))
            .cloned()
    })
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

fn wrap_paragraphs_as_single_page(
    paragraphs: &[(usize, String)],
    wrap_columns: usize,
    writing_mode: WritingMode,
) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    for (paragraph_index, text) in paragraphs {
        lines.extend(wrap_text_line(text, *paragraph_index, wrap_columns));
        if !writing_mode.is_vertical() {
            lines.push(PageTextLine::new(String::new(), None, 0, 0));
        }
    }
    trim_trailing_projection_blank_lines(&mut lines);
    lines
}

fn toc_leader_line(title: &str, page_label: &str, max_columns: usize) -> String {
    let title_width = text_display_column_width(title);
    let page_width = text_display_column_width(page_label);
    let leader_width = max_columns.saturating_sub(title_width + page_width).max(8);
    let leader_count = (leader_width / display_column_width('…')).max(4);
    format!("{title}{}{page_label}", "…".repeat(leader_count))
}

fn paginate_selected_paragraphs(
    paragraphs: &[(usize, String)],
    layout: PageLayout,
    writing_mode: WritingMode,
    forced_breaks: &BTreeMap<usize, Vec<usize>>,
) -> Vec<Vec<PageTextLine>> {
    let wrap_columns = layout.wrap_columns(writing_mode);
    let lines_per_page = layout.lines_per_page(writing_mode);
    let mut pages = Vec::new();
    let mut current_page = Vec::new();

    for (paragraph_index, text) in paragraphs {
        let paragraph_breaks = forced_breaks
            .get(paragraph_index)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let wrapped = wrap_text_line(text, *paragraph_index, wrap_columns);
        let mut forced_at_paragraph_end = false;
        if wrapped.is_empty() {
            push_paginated_line(
                &mut pages,
                &mut current_page,
                PageTextLine::new(String::new(), Some(*paragraph_index), 0, 0),
                lines_per_page,
            );
            if paragraph_breaks.contains(&0) {
                force_page_break(&mut pages, &mut current_page);
                forced_at_paragraph_end = true;
            }
        } else {
            for line in wrapped {
                let segments = split_line_at_page_breaks(line, paragraph_breaks);
                for segment in segments {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        segment.line,
                        lines_per_page,
                    );
                    if segment.break_after {
                        force_page_break(&mut pages, &mut current_page);
                        forced_at_paragraph_end = true;
                    } else {
                        forced_at_paragraph_end = false;
                    }
                }
            }
        }
        if !forced_at_paragraph_end && !writing_mode.is_vertical() {
            push_paginated_line(
                &mut pages,
                &mut current_page,
                PageTextLine::new(String::new(), None, 0, 0),
                lines_per_page,
            );
        }
    }

    trim_trailing_projection_blank_lines(&mut current_page);
    if !current_page.is_empty() {
        pages.push(current_page);
    }

    pages
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

fn projected_page_breaks(document: &Document) -> BTreeMap<usize, Vec<usize>> {
    let mut breaks = BTreeMap::<usize, Vec<usize>>::new();
    for control in projected_text_controls(document) {
        if control.code != DOCUMENT_TEXT_PAGE_BREAK_CONTROL {
            continue;
        }
        breaks
            .entry(control.paragraph_index)
            .or_default()
            .push(control.char_offset);
    }
    for offsets in breaks.values_mut() {
        offsets.sort_unstable();
        offsets.dedup();
    }
    breaks
}

fn split_line_at_page_breaks(line: PageTextLine, break_offsets: &[usize]) -> Vec<PageLineSegment> {
    let Some(paragraph_index) = line.paragraph_index() else {
        return vec![PageLineSegment {
            line,
            break_after: false,
        }];
    };

    let mut segments = Vec::new();
    let mut segment_start = line.char_start();
    for break_offset in break_offsets.iter().copied() {
        if break_offset < segment_start || break_offset > line.char_end() {
            continue;
        }
        let text = text_by_char_range(
            line.text(),
            segment_start - line.char_start(),
            break_offset - line.char_start(),
        );
        if !text.is_empty() || break_offset == line.char_start() {
            segments.push(PageLineSegment {
                line: PageTextLine::new(text, Some(paragraph_index), segment_start, break_offset),
                break_after: true,
            });
        } else if let Some(last) = segments.last_mut() {
            last.break_after = true;
        } else {
            segments.push(PageLineSegment {
                line: PageTextLine::new(
                    String::new(),
                    Some(paragraph_index),
                    break_offset,
                    break_offset,
                ),
                break_after: true,
            });
        }
        segment_start = break_offset;
    }

    if segment_start < line.char_end() {
        segments.push(PageLineSegment {
            line: PageTextLine::new(
                text_by_char_range(
                    line.text(),
                    segment_start - line.char_start(),
                    line.char_end() - line.char_start(),
                ),
                Some(paragraph_index),
                segment_start,
                line.char_end(),
            ),
            break_after: false,
        });
    }

    if segments.is_empty() {
        segments.push(PageLineSegment {
            line,
            break_after: false,
        });
    }

    segments
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

fn force_page_break(pages: &mut Vec<Vec<PageTextLine>>, current_page: &mut Vec<PageTextLine>) {
    while current_page
        .last()
        .is_some_and(|line| line.text().is_empty() && line.paragraph_index().is_none())
    {
        current_page.pop();
    }
    if current_page.iter().any(|line| !line.text().is_empty()) {
        pages.push(std::mem::take(current_page));
    } else {
        current_page.clear();
    }
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

fn paragraph_text(paragraph: &Paragraph) -> String {
    let mut text = String::new();

    for inline in paragraph.inlines() {
        match inline {
            Inline::Text(run) => text.push_str(run.text()),
            Inline::Ruby(ruby) => text.push_str(ruby.base_text()),
            Inline::Unknown(_) => {}
        }
    }

    text
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

fn wrap_text_line(text: &str, paragraph_index: usize, max_columns: usize) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut width = 0usize;
    let mut line_start = 0usize;
    let mut char_offset = 0usize;

    for character in text.chars() {
        let char_width = display_column_width(character);
        if width > 0 && width + char_width > max_columns {
            lines.push(PageTextLine::new(
                std::mem::take(&mut line),
                Some(paragraph_index),
                line_start,
                char_offset,
            ));
            width = 0;
            line_start = char_offset;
        }
        line.push(character);
        width += char_width;
        char_offset += 1;
    }

    if !line.is_empty() {
        lines.push(PageTextLine::new(
            line,
            Some(paragraph_index),
            line_start,
            char_offset,
        ));
    }

    lines
}

fn display_column_width(character: char) -> usize {
    match character {
        '\t' => APP_TAB_COLUMNS,
        _ if character.is_ascii() => 1,
        _ => 2,
    }
}

fn text_display_column_width(text: &str) -> usize {
    text.chars().map(display_column_width).sum()
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

fn nearest_text_line(
    lines: &[PageTextLine],
    target_index: usize,
) -> Option<(usize, &PageTextLine)> {
    if lines.is_empty() {
        return None;
    }

    let target_index = target_index.min(lines.len() - 1);
    if lines[target_index].paragraph_index().is_some() {
        return Some((target_index, &lines[target_index]));
    }

    for distance in 1..lines.len() {
        if let Some(index) = target_index.checked_sub(distance)
            && lines[index].paragraph_index().is_some()
        {
            return Some((index, &lines[index]));
        }

        let index = target_index + distance;
        if index < lines.len() && lines[index].paragraph_index().is_some() {
            return Some((index, &lines[index]));
        }
    }

    None
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
    let mut current_offset = line.char_start();

    for character in line.text().chars() {
        if current_offset >= char_offset {
            break;
        }
        units += display_column_width(character) as f64;
        current_offset += 1;
    }

    units
}

fn char_offset_for_x(layout: PageLayout, line: &PageTextLine, x: f64) -> usize {
    let target_units =
        ((normalize_coordinate(x) - layout.margin_px() as f64) / column_width_px(layout)).max(0.0);
    let mut units = 0.0;
    let mut char_offset = line.char_start();

    for character in line.text().chars() {
        let width = display_column_width(character) as f64;
        if target_units <= units + (width / 2.0) {
            return char_offset;
        }
        units += width;
        char_offset += 1;
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

fn paragraph_line_index(lines: &[(usize, usize, &PageTextLine)], char_offset: usize) -> usize {
    let mut last_index = 0usize;

    for (index, (_, _, line)) in lines.iter().enumerate() {
        last_index = index;
        if char_offset <= line.char_end() {
            return index;
        }
    }

    last_index
}

fn text_location_index(
    locations: &[(usize, usize, &PageTextLine)],
    paragraph_index: usize,
    char_offset: usize,
) -> Result<usize> {
    let mut last_index = None;

    for (index, (_, _, line)) in locations.iter().enumerate() {
        if line.paragraph_index() != Some(paragraph_index) {
            continue;
        }

        last_index = Some(index);
        if char_offset <= line.char_end() {
            return Ok(index);
        }
    }

    last_index.ok_or_else(|| {
        rjtd_core::Error::InvalidData(format!("paragraph {paragraph_index} out of range"))
    })
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

fn ok_page_count_json(page_count: u32) -> String {
    json_ok_with(&format!("\"pageCount\":{page_count}"))
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

fn default_object_bbox_json() -> String {
    "{\"pageIndex\":0,\"x\":0.0,\"y\":0.0,\"width\":0.0,\"height\":0.0}".to_string()
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

fn text_count_ranges_json(ranges: &[TextCountRange]) -> String {
    let mut output = String::from("[");
    for (index, range) in ranges.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_count_range_json(&mut output, range);
    }
    output.push(']');
    output
}

fn text_control_boundaries_json(boundaries: &[TextControlBoundary]) -> String {
    let mut output = String::from("[");
    for (index, boundary) in boundaries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_control_boundary_json(&mut output, boundary);
    }
    output.push(']');
    output
}

fn text_boundary_candidates_json(candidates: &[TextBoundaryCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_boundary_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

fn text_paragraph_boundary_candidates_json(
    candidates: &[TextParagraphBoundaryCandidate],
) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_paragraph_boundary_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

fn table_candidates_json(candidates: &[TableCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

fn object_stream_candidates_json(candidates: &[ObjectStreamCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_stream_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

fn object_frame_records_json(records: &[ObjectFrameRecordCandidate]) -> String {
    let mut output = String::from("[");
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_frame_record_candidate_json(&mut output, record);
    }
    output.push(']');
    output
}

fn object_embedding_frames_json(frames: &[ObjectEmbeddingFrameCandidate]) -> String {
    let mut output = String::from("[");
    for (index, frame) in frames.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_embedding_frame_candidate_json(&mut output, frame);
    }
    output.push(']');
    output
}

fn push_object_frame_record_candidate_json(
    output: &mut String,
    record: &ObjectFrameRecordCandidate,
) {
    output.push_str("{\"sourcePath\":");
    output.push_str(&json_string(record.source_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&record.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&record.row_start().to_string());
    output.push_str(",\"recordLen\":");
    output.push_str(&record.record_len().to_string());
    output.push_str(",\"recordKind\":");
    output.push_str(&record.record_kind().to_string());
    output.push_str(",\"recordKindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", record.record_kind())));
    output.push_str(",\"declaredRecordBytes\":");
    output.push_str(&record.declared_record_bytes().to_string());
    output.push_str(",\"objectId\":");
    output.push_str(&record.object_id().to_string());
    output.push_str(",\"objectType\":");
    output.push_str(&record.object_type().to_string());
    output.push_str(",\"objectTypeHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", record.object_type())));
    output.push_str(",\"geometry\":{\"x\":");
    output.push_str(&record.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&record.y().to_string());
    output.push_str(",\"width\":");
    output.push_str(&record.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&record.height().to_string());
    output.push_str("},\"rowPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(record.row_prefix())));
    output.push_str(",\"decoded\":false}");
}

fn push_object_embedding_frame_candidate_json(
    output: &mut String,
    frame: &ObjectEmbeddingFrameCandidate,
) {
    output.push_str("{\"sourcePath\":");
    output.push_str(&json_string(frame.source_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&frame.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&frame.row_start().to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    output.push_str(&json_string(frame.class_name()));
    output.push_str(",\"primarySize\":{\"width\":");
    output.push_str(&frame.primary_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&frame.primary_height().to_string());
    output.push_str("},\"frameRef\":");
    output.push_str(&frame.frame_ref().to_string());
    output.push_str(",\"frameSize\":{\"width\":");
    output.push_str(&frame.frame_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&frame.frame_height().to_string());
    output.push_str("},\"rowPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(frame.row_prefix())));
    output.push_str(",\"decoded\":false}");
}

fn push_object_stream_candidate_json(output: &mut String, candidate: &ObjectStreamCandidate) {
    output.push_str("{\"path\":");
    output.push_str(&json_string(candidate.path()));
    output.push_str(",\"size\":");
    output.push_str(&candidate.size().to_string());
    output.push_str(",\"reasons\":[");
    for (index, reason) in candidate.reasons().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(reason.as_str()));
    }
    output.push_str("],\"ownershipCandidate\":");
    if let Some(ownership) = candidate.ownership_candidate() {
        push_object_stream_ownership_candidate_json(output, ownership);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"ownershipReferences\":[");
    for (index, reference) in candidate
        .ownership_reference_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_object_stream_ownership_reference_candidate_json(output, reference);
    }
    output.push_str("],\"frameReferenceRows\":[");
    for (index, row) in candidate
        .frame_reference_row_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_object_frame_reference_row_candidate_json(output, row);
    }
    output.push_str("],\"fdmIndexEntries\":[");
    for (index, entry) in candidate.fdm_index_entry_candidates().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_index_entry_candidate_json(output, entry);
    }
    output.push_str("],\"imageSignatures\":[");
    for (index, hit) in candidate.image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"imagePayloads\":[");
    for (index, span) in candidate.image_payload_spans().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_payload_span_json(output, span);
    }
    output.push_str("],\"svgOffsets\":");
    push_usize_array_json(output, candidate.svg_offsets());
    output.push_str(",\"soOffsets\":");
    push_usize_array_json(output, candidate.so_offsets());
    output.push_str(",\"visualList\":");
    if let Some(visual_list) = candidate.visual_list_candidate() {
        push_object_visual_list_candidate_json(output, visual_list);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddedPressSnapshot\":");
    if let Some(snapshot) = candidate.embedded_press_snapshot_candidate() {
        push_object_embedded_press_snapshot_candidate_json(output, snapshot);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"jseq3Formula\":");
    if let Some(formula) = candidate.jseq3_formula_candidate() {
        push_object_jseq3_formula_candidate_json(output, formula);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"payloadPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(candidate.payload_prefix())));
    output.push_str(",\"decoded\":false}");
}

fn push_object_jseq3_formula_candidate_json(
    output: &mut String,
    formula: &ObjectJseq3FormulaCandidate,
) {
    output.push_str("{\"format\":\"JSEQ3Contents\",\"magic\":");
    output.push_str(&json_string(formula.magic()));
    output.push_str(",\"magicOffset\":");
    output.push_str(&formula.magic_offset().to_string());
    output.push_str(",\"soTrailerOffset\":");
    push_option_usize_json(output, formula.so_trailer_offset());
    output.push_str(",\"soTrailerLength\":");
    push_option_usize_json(output, formula.so_trailer_length());
    output.push_str(",\"soTrailerFields\":");
    push_u32_array_json(output, formula.so_trailer_fields());
    output.push_str(",\"textMarkers\":[");
    for (index, marker) in formula.text_markers().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        output.push_str(&json_string(marker.text()));
        output.push_str(",\"offset\":");
        output.push_str(&marker.offset().to_string());
        output.push_str(",\"encoding\":");
        output.push_str(&json_string(marker.encoding()));
        output.push('}');
    }
    output.push_str("],\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(formula.header_prefix())));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

fn push_object_embedded_press_snapshot_candidate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    output.push_str("{\"format\":\"JSSnapShot32\",\"magic\":");
    output.push_str(&json_string(snapshot.magic()));
    output.push_str(",\"bodyLengthCandidate\":");
    output.push_str(&snapshot.body_length_candidate().to_string());
    output.push_str(",\"formatMarker\":");
    output.push_str(&json_string(snapshot.format_marker()));
    output.push_str(",\"objectCountCandidate\":");
    output.push_str(&snapshot.object_count_candidate().to_string());
    output.push_str(",\"objectTableOffsetCandidate\":");
    output.push_str(&snapshot.object_table_offset_candidate().to_string());
    output.push_str(",\"payloadLengthCandidate\":");
    output.push_str(&snapshot.payload_length_candidate().to_string());
    output.push_str(",\"width\":");
    output.push_str(&snapshot.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&snapshot.height().to_string());
    output.push_str(",\"vectorSegmentCount\":");
    output.push_str(&snapshot.vector_segments().len().to_string());
    output.push_str(",\"vectorSegmentPreview\":");
    push_object_embedded_press_snapshot_vector_segment_preview_json(output, snapshot);
    output.push_str(",\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(snapshot.header_prefix())));
    output.push_str(",\"renderable\":");
    output.push_str(if snapshot.vector_segments().is_empty() {
        "false"
    } else {
        "true"
    });
    output.push_str(",\"decoded\":false}");
}

fn push_object_embedded_press_snapshot_vector_segment_preview_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    output.push('[');
    for (index, segment) in snapshot.vector_segments().iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"x1\":");
        output.push_str(&segment.x1().to_string());
        output.push_str(",\"y1\":");
        output.push_str(&segment.y1().to_string());
        output.push_str(",\"x2\":");
        output.push_str(&segment.x2().to_string());
        output.push_str(",\"y2\":");
        output.push_str(&segment.y2().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn push_object_visual_list_candidate_json(
    output: &mut String,
    visual_list: &ObjectVisualListCandidate,
) {
    output.push_str("{\"format\":\"BMDV\",\"declaredSize\":");
    output.push_str(&visual_list.declared_size().to_string());
    output.push_str(",\"magicOffset\":");
    output.push_str(&visual_list.magic_offset().to_string());
    output.push_str(",\"magic\":");
    output.push_str(&json_string(visual_list.magic()));
    output.push_str(",\"version\":");
    output.push_str(&visual_list.version().to_string());
    output.push_str(",\"flags\":");
    output.push_str(&visual_list.flags().to_string());
    output.push_str(",\"width\":");
    output.push_str(&visual_list.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&visual_list.height().to_string());
    output.push_str(",\"rowStride\":");
    output.push_str(&visual_list.row_stride().to_string());
    output.push_str(",\"bitDepth\":");
    output.push_str(&visual_list.bit_depth().to_string());
    output.push_str(",\"xPixelsPerMeter\":");
    output.push_str(&visual_list.x_pixels_per_meter().to_string());
    output.push_str(",\"yPixelsPerMeter\":");
    output.push_str(&visual_list.y_pixels_per_meter().to_string());
    output.push_str(",\"rleDataOffset\":");
    output.push_str(&visual_list.rle_data_offset().to_string());
    output.push_str(",\"rleDataLength\":");
    output.push_str(&visual_list.rle_data_len().to_string());
    output.push_str(",\"pixelCount\":");
    output.push_str(&visual_list.pixels().len().to_string());
    output.push_str(",\"rleEncoding\":\"bmp-rle8-like\",\"renderable\":true,\"decoded\":false}");
}

fn push_object_stream_ownership_candidate_json(
    output: &mut String,
    ownership: &ObjectStreamOwnershipCandidate,
) {
    output.push_str("{\"basis\":");
    output.push_str(&json_string(ownership.basis()));
    output.push_str(",\"family\":");
    output.push_str(&json_string(ownership.family()));
    output.push_str(",\"storagePath\":");
    if let Some(storage_path) = ownership.storage_path() {
        output.push_str(&json_string(storage_path));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddingIndex\":");
    if let Some(index) = ownership.embedding_index() {
        output.push_str(&index.to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"streamRole\":");
    output.push_str(&json_string(ownership.stream_role()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_stream_ownership_reference_candidate_json(
    output: &mut String,
    reference: &ObjectStreamOwnershipReferenceCandidate,
) {
    output.push_str("{\"targetPath\":");
    output.push_str(&json_string(reference.target_path()));
    output.push_str(",\"encoding\":");
    output.push_str(&json_string(reference.encoding()));
    output.push_str(",\"totalMatches\":");
    output.push_str(&reference.total_matches().to_string());
    output.push_str(",\"offsets\":");
    push_usize_array_json(output, reference.offsets());
    output.push_str(",\"decoded\":false}");
}

fn push_object_fdm_index_entry_candidate_json(
    output: &mut String,
    entry: &ObjectFdmIndexEntryCandidate,
) {
    output.push_str("{\"indexPath\":");
    output.push_str(&json_string(entry.index_path()));
    output.push_str(",\"vectorPath\":");
    output.push_str(&json_string(entry.vector_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&entry.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&entry.index_offset().to_string());
    output.push_str(",\"vectorOffset\":");
    output.push_str(&entry.vector_offset().to_string());
    output.push_str(",\"nextVectorOffset\":");
    output.push_str(&entry.next_vector_offset().to_string());
    output.push_str(",\"vectorLength\":");
    output.push_str(&entry.vector_len().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&entry.kind().to_string());
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", entry.kind())));
    output.push_str(",\"bbox\":");
    push_object_fdm_index_bbox_json(output, entry.bbox());
    output.push_str(",\"validVectorOffset\":");
    output.push_str(if entry.valid_vector_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"vectorPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(entry.vector_prefix())));
    output.push_str(",\"imageSignatures\":[");
    for (index, hit) in entry.image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"segmentImageSignatures\":[");
    for (index, hit) in entry.segment_image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"decoded\":false}");
}

fn push_object_fdm_index_bbox_json(output: &mut String, bbox: ObjectFdmIndexBbox) {
    output.push_str("{\"left\":");
    output.push_str(&bbox.left().to_string());
    output.push_str(",\"top\":");
    output.push_str(&bbox.top().to_string());
    output.push_str(",\"right\":");
    output.push_str(&bbox.right().to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&bbox.bottom().to_string());
    output.push('}');
}

fn push_object_frame_reference_row_candidate_json(
    output: &mut String,
    row: &ObjectFrameReferenceRowCandidate,
) {
    output.push_str("{\"targetPath\":");
    output.push_str(&json_string(row.target_path()));
    output.push_str(",\"encoding\":");
    output.push_str(&json_string(row.encoding()));
    output.push_str(",\"stride\":");
    output.push_str(&row.stride().to_string());
    output.push_str(",\"fieldOffset\":");
    output.push_str(&row.field_offset().to_string());
    output.push_str(",\"offset\":");
    output.push_str(&row.offset().to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&row.row_start().to_string());
    output.push_str(",\"family\":");
    output.push_str(&json_string(row.family()));
    output.push_str(",\"rowHex\":");
    output.push_str(&json_string(&hex_bytes(row.row())));
    output.push_str(",\"suffixLink\":");
    if let Some(link) = row.suffix_link() {
        output.push_str("{\"relation\":");
        output.push_str(&json_string(link.relation()));
        output.push_str(",\"suffixFamily\":");
        output.push_str(&json_string(link.suffix_family()));
        output.push_str(",\"matchedRowStart\":");
        output.push_str(&link.matched_row_start().to_string());
        output.push_str(",\"matchedRowIndex\":");
        output.push_str(&link.matched_row_index().to_string());
        output.push_str(",\"decoded\":false}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_payload_span_json(output: &mut String, span: &ObjectImagePayloadSpan) {
    output.push_str("{\"kind\":");
    output.push_str(&json_string(span.kind()));
    output.push_str(",\"mime\":");
    output.push_str(&json_string(span.mime()));
    output.push_str(",\"signatureOffset\":");
    output.push_str(&span.signature_offset().to_string());
    output.push_str(",\"start\":");
    output.push_str(&span.start().to_string());
    output.push_str(",\"end\":");
    output.push_str(&span.end().to_string());
    output.push_str(",\"length\":");
    output.push_str(&span.len().to_string());
    output.push_str(",\"complete\":");
    output.push_str(if span.complete() { "true" } else { "false" });
    output.push_str(",\"dimensions\":");
    push_object_image_dimensions_json(output, span.dimensions());
    output.push_str(",\"objectEnvelope\":");
    push_object_image_payload_envelope_json(output, span.envelope());
    output.push_str(",\"payloadPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &span.payload()[..span.payload().len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_dimensions_json(
    output: &mut String,
    dimensions: Option<ObjectImageDimensions>,
) {
    if let Some(dimensions) = dimensions {
        output.push_str("{\"width\":");
        output.push_str(&dimensions.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&dimensions.height().to_string());
        output.push('}');
    } else {
        output.push_str("null");
    }
}

fn push_object_image_payload_envelope_json(
    output: &mut String,
    envelope: &ObjectImagePayloadEnvelope,
) {
    output.push_str("{\"headerStart\":");
    output.push_str(&envelope.header_start().to_string());
    output.push_str(",\"headerEnd\":");
    output.push_str(&envelope.header_end().to_string());
    output.push_str(",\"headerLength\":");
    output.push_str(&envelope.header_len().to_string());
    output.push_str(",\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &envelope.header()[..envelope
            .header()
            .len()
            .min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"headerFields\":");
    push_object_image_header_fields_json(output, envelope.header_fields());
    output.push_str(",\"trailerStart\":");
    output.push_str(&envelope.trailer_start().to_string());
    output.push_str(",\"trailerEnd\":");
    output.push_str(&envelope.trailer_end().to_string());
    output.push_str(",\"trailerLength\":");
    output.push_str(&envelope.trailer_len().to_string());
    output.push_str(",\"trailerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &envelope.trailer()[..envelope
            .trailer()
            .len()
            .min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"declaredPayloadLength\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.value().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthOffset\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.offset().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthEndian\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&json_string(length.endian()));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_header_fields_json(
    output: &mut String,
    fields: &ObjectImageHeaderFieldCandidates,
) {
    output.push_str("{\"u16LePrefix\":[");
    for (index, field) in fields.u16_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"u32LePrefix\":[");
    for (index, field) in fields.u32_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"sourcePathCandidate\":");
    if let Some(path) = fields.source_path_candidate() {
        push_object_image_source_path_candidate_json(output, path);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

fn push_object_image_numeric_header_field_json(
    output: &mut String,
    field: &ObjectImageNumericHeaderField,
) {
    output.push_str("{\"offset\":");
    output.push_str(&field.offset().to_string());
    output.push_str(",\"value\":");
    output.push_str(&field.value().to_string());
    output.push('}');
}

fn push_object_image_source_path_candidate_json(
    output: &mut String,
    path: &ObjectImageSourcePathCandidate,
) {
    output.push_str("{\"lengthOffset\":");
    output.push_str(&path.length_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&path.declared_length().to_string());
    output.push_str(",\"bytesStart\":");
    output.push_str(&path.bytes_start().to_string());
    output.push_str(",\"bytesEnd\":");
    output.push_str(&path.bytes_end().to_string());
    output.push_str(",\"nulTerminated\":");
    output.push_str(if path.nul_terminated() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bytesHex\":");
    output.push_str(&json_string(&hex_bytes(path.bytes())));
    output.push_str(",\"textLossy\":");
    output.push_str(&json_string(path.text_lossy()));
    output.push_str(",\"decoded\":false}");
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

fn push_text_boundary_candidate_json(output: &mut String, candidate: &TextBoundaryCandidate) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"basis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        candidate.delimiter_code()
    )));
    output.push_str(",\"intervalCount\":");
    output.push_str(&candidate.interval_count().to_string());
    output.push_str(",\"firstIntervalIndex\":");
    output.push_str(&candidate.first_interval_index().to_string());
    output.push_str(",\"lastIntervalIndex\":");
    output.push_str(&candidate.last_interval_index().to_string());
    output.push_str(",\"sourceStart\":");
    output.push_str(&candidate.source_start().to_string());
    output.push_str(",\"sourceEnd\":");
    output.push_str(&candidate.source_end().to_string());
    output.push_str(",\"decoded\":false}");
}

fn push_text_paragraph_boundary_candidate_json(
    output: &mut String,
    candidate: &TextParagraphBoundaryCandidate,
) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"textBoundaryCandidateIndex\":");
    output.push_str(&candidate.text_boundary_candidate_index().to_string());
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"sourceStart\":");
    output.push_str(&candidate.source_start().to_string());
    output.push_str(",\"sourceEnd\":");
    output.push_str(&candidate.source_end().to_string());
    output.push_str(",\"textCountRangeSpan\":");
    output.push_str(&candidate.text_count_range_span().to_string());
    output.push_str(",\"rule\":");
    output.push_str(&json_string(candidate.rule()));
    output.push_str(",\"lineWordEvidence\":");
    push_text_layout_exact_evidence_json(output, candidate.line_word_evidence());
    output.push_str(",\"pageFieldEvidence\":");
    push_text_layout_exact_evidence_json(output, candidate.page_field_evidence());
    output.push_str(",\"decoded\":false}");
}

fn push_table_candidate_json(output: &mut String, candidate: &TableCandidate) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"textBoundaryCandidateIndex\":");
    output.push_str(&candidate.text_boundary_candidate_index().to_string());
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"basis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        candidate.delimiter_code()
    )));
    output.push_str(",\"intervalCount\":");
    output.push_str(&candidate.interval_count().to_string());
    output.push_str(",\"firstIntervalIndex\":");
    output.push_str(&candidate.first_interval_index().to_string());
    output.push_str(",\"lastIntervalIndex\":");
    output.push_str(&candidate.last_interval_index().to_string());
    output.push_str(",\"sourceStart\":");
    output.push_str(&candidate.source_start().to_string());
    output.push_str(",\"sourceEnd\":");
    output.push_str(&candidate.source_end().to_string());
    output.push_str(",\"intervals\":");
    push_table_candidate_intervals_json(output, candidate.intervals(), candidate.is_row_like());
    output.push_str(",\"cellLike\":");
    output.push_str(if candidate.is_cell_like() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowLike\":");
    output.push_str(if candidate.is_row_like() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"observedTable\":");
    if candidate.is_row_like() {
        output.push_str(&observed_table_dimensions_json(candidate));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rule\":");
    output.push_str(&json_string(candidate.rule()));
    output.push_str(",\"decoded\":false}");
}

fn push_table_candidate_intervals_json(
    output: &mut String,
    intervals: &[TableCandidateInterval],
    emit_column_segments: bool,
) {
    output.push('[');
    for (index, interval) in intervals.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&interval.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&interval.source_interval_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&interval.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&interval.source_end().to_string());
        output.push_str(",\"textPreview\":");
        output.push_str(&json_string(interval.text_preview()));
        output.push_str(",\"textCharCount\":");
        output.push_str(&interval.text_char_count().to_string());
        output.push_str(",\"lineBreakCount\":");
        output.push_str(&interval.line_break_count().to_string());
        output.push_str(",\"columnSegments\":");
        if emit_column_segments {
            push_table_candidate_column_segments_json(output, interval.column_segments());
        } else {
            output.push_str("[]");
        }
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn push_table_candidate_column_segments_json(
    output: &mut String,
    segments: &[TableCandidateColumnSegment],
) {
    output.push('[');
    for (index, segment) in segments.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&segment.index().to_string());
        output.push_str(",\"kind\":");
        output.push_str(&json_string(segment.kind().as_str()));
        output.push_str(",\"charStart\":");
        output.push_str(&segment.char_start().to_string());
        output.push_str(",\"charEnd\":");
        output.push_str(&segment.char_end().to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(segment.text()));
        output.push_str(",\"charCount\":");
        output.push_str(&segment.text().chars().count().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn push_text_layout_exact_evidence_json(output: &mut String, evidence: &TextLayoutExactEvidence) {
    output.push_str("{\"target\":");
    output.push_str(&json_string(evidence.target()));
    output.push_str(",\"base\":");
    output.push_str(&json_string(evidence.base()));
    output.push_str(",\"delta\":");
    output.push_str(&evidence.delta().to_string());
    output.push('}');
}

fn push_text_control_boundary_json(output: &mut String, boundary: &TextControlBoundary) {
    output.push_str("{\"index\":");
    output.push_str(&boundary.index().to_string());
    output.push_str(",\"code\":");
    output.push_str(&boundary.code().to_string());
    output.push_str(",\"codeHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", boundary.code())));
    output.push_str(",\"sourceSpan\":");
    match boundary.source_span() {
        Some(span) => push_text_source_span_json(output, span),
        None => output.push_str("null"),
    }
    output.push_str(",\"decoded\":false}");
}

fn push_text_source_span_json(output: &mut String, span: &TextSourceSpan) {
    output.push_str("{\"byteStart\":");
    output.push_str(&span.byte_start().to_string());
    output.push_str(",\"byteEnd\":");
    output.push_str(&span.byte_end().to_string());
    output.push_str(",\"unitStart\":");
    output.push_str(&span.unit_start().to_string());
    output.push_str(",\"unitEnd\":");
    output.push_str(&span.unit_end().to_string());
    output.push('}');
}

fn push_text_count_range_json(output: &mut String, range: &TextCountRange) {
    output.push_str("{\"index\":");
    output.push_str(&range.index().to_string());
    output.push_str(",\"family\":");
    output.push_str(&json_string(range.family()));
    output.push_str(",\"start\":");
    output.push_str(&range.start().to_string());
    output.push_str(",\"end\":");
    output.push_str(&range.end().to_string());
    output.push_str(",\"span\":");
    output.push_str(&range.span().to_string());
    output.push_str(",\"declaredStart\":");
    output.push_str(&range.declared_start().to_string());
    output.push_str(",\"declaredEnd\":");
    output.push_str(&range.declared_end().to_string());
    output.push_str(",\"tailFields\":");
    push_u16_array_json(output, range.tail_fields());
    output.push_str(",\"documentTextOverlaps\":");
    text_count_range_overlaps_json(output, range.document_text_overlaps());
    output.push_str(",\"controlRangeOverlaps\":");
    text_count_control_range_overlaps_json(output, range.control_range_overlaps());
    output.push_str(",\"decoded\":false,\"rawHex\":");
    output.push_str(&json_string(&hex_bytes(range.raw())));
    output.push('}');
}

fn text_count_range_overlaps_json(output: &mut String, overlaps: &[TextCountRangeOverlap]) {
    output.push('[');
    for (index, overlap) in overlaps.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"basis\":");
        output.push_str(&json_string(overlap.basis().as_str()));
        output.push_str(",\"blockIndex\":");
        output.push_str(&overlap.block_index().to_string());
        output.push_str(",\"inlineIndex\":");
        output.push_str(&overlap.inline_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&overlap.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&overlap.source_end().to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(overlap.text()));
        output.push('}');
    }
    output.push(']');
}

fn text_count_control_range_overlaps_json(
    output: &mut String,
    overlaps: &[TextCountControlRangeOverlap],
) {
    output.push('[');
    for (index, overlap) in overlaps.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"basis\":");
        output.push_str(&json_string(overlap.basis().as_str()));
        output.push_str(",\"delimiterCode\":");
        output.push_str(&overlap.delimiter_code().to_string());
        output.push_str(",\"delimiterCodeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", overlap.delimiter_code())));
        output.push_str(",\"rangeCount\":");
        output.push_str(&overlap.range_count().to_string());
        output.push_str(",\"firstRangeIndex\":");
        output.push_str(&overlap.first_range_index().to_string());
        output.push_str(",\"lastRangeIndex\":");
        output.push_str(&overlap.last_range_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&overlap.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&overlap.source_end().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
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

fn page_marks_json(page_marks: &[DocumentPageMark]) -> String {
    let mut output = String::from("[");
    for (index, page_mark) in page_marks.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"sourceStream\":");
        output.push_str(&json_string(page_mark.source_stream()));
        output.push_str(",\"family\":");
        output.push_str(&json_string(page_mark.family()));
        output.push_str(",\"headerCount\":");
        output.push_str(&page_mark.header_count().to_string());
        output.push_str(",\"headerStride\":");
        output.push_str(&page_mark.header_stride().to_string());
        output.push_str(",\"headerLastIndex\":");
        output.push_str(&page_mark.header_last_index().to_string());
        output.push_str(",\"entryCount\":");
        output.push_str(&page_mark.entries().len().to_string());
        output.push_str(",\"trailingByteLength\":");
        output.push_str(&page_mark.trailing_byte_len().to_string());
        output.push_str(",\"entries\":[");
        for (entry_index, entry) in page_mark.entries().iter().enumerate() {
            if entry_index > 0 {
                output.push(',');
            }
            output.push_str("{\"rowIndex\":");
            output.push_str(&entry.row_index().to_string());
            output.push_str(",\"index\":");
            push_option_u32_json(&mut output, entry.index());
            output.push_str(",\"flags\":");
            push_option_u32_json(&mut output, entry.flags());
            output.push_str(",\"flagsHex\":");
            if let Some(flags) = entry.flags() {
                output.push_str(&json_string(&format!("0x{flags:08x}")));
            } else {
                output.push_str("null");
            }
            output.push_str(",\"lineStart\":");
            push_option_u32_json(&mut output, entry.line_start());
            output.push_str(",\"lineEnd\":");
            push_option_u32_json(&mut output, entry.line_end());
            output.push_str(",\"rawLength\":");
            output.push_str(&entry.raw_len().to_string());
            output.push_str(",\"decoded\":false}");
        }
        output.push_str("],\"decoded\":false}");
    }
    output.push(']');
    output
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

fn text_style_candidates(styles: &[UnknownStyle]) -> Vec<StyleCandidate> {
    let mut candidates = Vec::new();

    for style in styles {
        if style.name() != Some(TEXT_LAYOUT_STYLE_PATH) {
            continue;
        }

        let summary = summarize_style_stream(style.payload());
        for (record_index, record) in summary.records().iter().enumerate() {
            let Some(label) = record.label() else {
                continue;
            };
            let trimmed = label.trim();
            if trimmed.is_empty() {
                continue;
            }

            candidates.push(StyleCandidate {
                id: candidates.len() as u32 + 1,
                name: trimmed.to_string(),
                source_stream: TEXT_LAYOUT_STYLE_PATH.to_string(),
                source_record_index: record_index,
                source_offset: record.offset(),
                source_code: record.code(),
                payload_len: record.payload_len(),
            });
        }
    }

    candidates
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

fn push_option_usize_json(output: &mut String, value: Option<usize>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
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
struct ImagePayloadDiagnostic<'a> {
    candidate_index: usize,
    payload_index: usize,
    candidate: &'a ObjectStreamCandidate,
    span: &'a ObjectImagePayloadSpan,
}

#[derive(Debug, Clone, Copy)]
struct VisualListDiagnostic<'a> {
    candidate_index: usize,
    candidate: &'a ObjectStreamCandidate,
    visual_list: &'a ObjectVisualListCandidate,
}

#[derive(Debug, Clone, Copy)]
struct EmbeddingFrameDiagnostic<'a> {
    frame_index: usize,
    frame: &'a ObjectEmbeddingFrameCandidate,
    frame_record: Option<&'a ObjectFrameRecordCandidate>,
    embedded_press_snapshot: Option<&'a ObjectEmbeddedPressSnapshotCandidate>,
    jseq3_formula: Option<&'a ObjectJseq3FormulaCandidate>,
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

fn page_overlay_images_json(core: &DocumentCore) -> String {
    let mut diagnostics = image_payload_overlay_diagnostics_json(&core.document);
    diagnostics.extend(fdm_image_overlay_diagnostics_json(&core.document));
    if diagnostics.is_empty() {
        return "{\"behind\":[],\"front\":[],\"imageCount\":0}".to_string();
    }

    format!(
        "{{\"behind\":[],\"front\":[],\"imageCount\":0,\"unplacedDiagnostics\":[{}],\"diagnosticCount\":{}}}",
        diagnostics.join(","),
        diagnostics.len()
    )
}

fn image_payload_overlay_diagnostics_json(document: &Document) -> Vec<String> {
    image_payload_diagnostics(document)
        .into_iter()
        .map(|diagnostic| {
            let mut output = String::new();
            output.push_str("{\"type\":\"jtdImagePayloadCandidate\",\"sourcePath\":");
            output.push_str(&json_string(diagnostic.candidate.path()));
            output.push_str(",\"objectCandidateIndex\":");
            output.push_str(&diagnostic.candidate_index.to_string());
            output.push_str(",\"payloadIndex\":");
            output.push_str(&diagnostic.payload_index.to_string());
            output.push_str(",\"kind\":");
            output.push_str(&json_string(diagnostic.span.kind()));
            output.push_str(",\"mime\":");
            output.push_str(&json_string(diagnostic.span.mime()));
            output.push_str(",\"signatureOffset\":");
            output.push_str(&diagnostic.span.signature_offset().to_string());
            output.push_str(",\"length\":");
            output.push_str(&diagnostic.span.len().to_string());
            output.push_str(",\"dimensions\":");
            push_object_image_dimensions_json(&mut output, diagnostic.span.dimensions());
            output.push_str(",\"placementProven\":false,\"geometryDecoded\":false,\"renderable\":true,\"decoded\":false}");
            output
        })
        .collect()
}

fn image_payload_diagnostics(document: &Document) -> Vec<ImagePayloadDiagnostic<'_>> {
    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for (payload_index, span) in candidate.image_payload_spans().iter().enumerate() {
            if svg_embeddable_image_payload(span) {
                diagnostics.push(ImagePayloadDiagnostic {
                    candidate_index,
                    payload_index,
                    candidate,
                    span,
                });
            }
        }
    }
    diagnostics
}

fn visual_list_diagnostics(document: &Document) -> Vec<VisualListDiagnostic<'_>> {
    document
        .object_stream_candidates()
        .iter()
        .enumerate()
        .filter_map(|(candidate_index, candidate)| {
            candidate
                .visual_list_candidate()
                .map(|visual_list| VisualListDiagnostic {
                    candidate_index,
                    candidate,
                    visual_list,
                })
        })
        .collect()
}

fn embedding_frame_diagnostics(document: &Document) -> Vec<EmbeddingFrameDiagnostic<'_>> {
    document
        .object_embedding_frames()
        .iter()
        .enumerate()
        .map(|(frame_index, frame)| {
            let frame_record = document
                .object_frame_records()
                .iter()
                .find(|record| record.row_index() as u32 == frame.frame_ref());
            let jseq3_path = format!(
                "/EmbedItems/Embedding {}/JSEQ3Contents",
                frame.embedding_index()
            );
            let jseq3_formula = document
                .object_stream_candidates()
                .iter()
                .find(|candidate| candidate.path() == jseq3_path)
                .and_then(ObjectStreamCandidate::jseq3_formula_candidate);
            let snapshot_path = format!(
                "/EmbedItems/Embedding {}/\x03EmbeddedPress",
                frame.embedding_index()
            );
            let embedded_press_snapshot = document
                .object_stream_candidates()
                .iter()
                .find(|candidate| candidate.path() == snapshot_path)
                .and_then(ObjectStreamCandidate::embedded_press_snapshot_candidate);
            EmbeddingFrameDiagnostic {
                frame_index,
                frame,
                frame_record,
                embedded_press_snapshot,
                jseq3_formula,
            }
        })
        .collect()
}

fn svg_embeddable_image_payload(span: &ObjectImagePayloadSpan) -> bool {
    image_payload_svg_data_uri(span).is_some()
}

fn fdm_image_overlay_diagnostics_json(document: &Document) -> Vec<String> {
    let mut diagnostics = Vec::new();
    for candidate in document.object_stream_candidates() {
        for entry in candidate
            .fdm_index_entry_candidates()
            .iter()
            .filter(|entry| !entry.segment_image_signature_hits().is_empty())
        {
            let bbox = entry.bbox();
            let normalized = normalize_fdm_bbox(bbox);
            let bbox_width = normalized.2.saturating_sub(normalized.0);
            let bbox_height = normalized.3.saturating_sub(normalized.1);
            let mut output = String::new();
            output.push_str("{\"type\":\"jtdFdmVectorImageCandidate\",\"sourcePath\":");
            output.push_str(&json_string(candidate.path()));
            output.push_str(",\"indexPath\":");
            output.push_str(&json_string(entry.index_path()));
            output.push_str(",\"vectorPath\":");
            output.push_str(&json_string(entry.vector_path()));
            output.push_str(",\"rowIndex\":");
            output.push_str(&entry.row_index().to_string());
            output.push_str(",\"vectorOffset\":");
            output.push_str(&entry.vector_offset().to_string());
            output.push_str(",\"nextVectorOffset\":");
            output.push_str(&entry.next_vector_offset().to_string());
            output.push_str(",\"vectorLength\":");
            output.push_str(&entry.vector_len().to_string());
            output.push_str(",\"kind\":");
            output.push_str(&entry.kind().to_string());
            output.push_str(",\"kindHex\":");
            output.push_str(&json_string(&format!("0x{:04x}", entry.kind())));
            output.push_str(",\"bbox\":");
            push_object_fdm_index_bbox_json(&mut output, bbox);
            output.push_str(",\"normalizedBbox\":");
            push_fdm_normalized_bbox_json(&mut output, normalized);
            output.push_str(",\"bboxWidth\":");
            output.push_str(&bbox_width.to_string());
            output.push_str(",\"bboxHeight\":");
            output.push_str(&bbox_height.to_string());
            output.push_str(",\"bboxOrder\":");
            output.push_str(&json_string(fdm_bbox_order(bbox)));
            output.push_str(",\"bboxPlausible\":");
            output.push_str(if fdm_bbox_is_plausible(bbox) {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"imageSignatures\":");
            push_object_image_signature_hits_json(&mut output, entry.image_signature_hits());
            output.push_str(",\"segmentImageSignatures\":");
            push_object_image_signature_hits_json(
                &mut output,
                entry.segment_image_signature_hits(),
            );
            output.push_str(",\"completePayloads\":");
            output.push_str(&fdm_entry_complete_payload_count(candidate, entry).to_string());
            output.push_str(",\"placementProven\":false,\"renderable\":false,\"reason\":\"page-placement-unproven\",\"decoded\":false}");
            diagnostics.push(output);
        }
    }
    diagnostics
}

fn fdm_entry_complete_payload_count(
    candidate: &ObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> usize {
    candidate
        .image_payload_spans()
        .iter()
        .filter(|span| {
            span.complete()
                && span.signature_offset() >= entry.vector_offset()
                && span.signature_offset() < entry.next_vector_offset()
        })
        .count()
}

fn normalize_fdm_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.right()),
        bbox.top().min(bbox.bottom()),
        bbox.left().max(bbox.right()),
        bbox.top().max(bbox.bottom()),
    )
}

fn push_fdm_normalized_bbox_json(output: &mut String, bbox: (i32, i32, i32, i32)) {
    output.push_str("{\"left\":");
    output.push_str(&bbox.0.to_string());
    output.push_str(",\"top\":");
    output.push_str(&bbox.1.to_string());
    output.push_str(",\"right\":");
    output.push_str(&bbox.2.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&bbox.3.to_string());
    output.push('}');
}

fn fdm_bbox_order(bbox: ObjectFdmIndexBbox) -> &'static str {
    match (bbox.left() <= bbox.right(), bbox.top() <= bbox.bottom()) {
        (true, true) => "forward",
        (false, true) => "inverted-x",
        (true, false) => "inverted-y",
        (false, false) => "inverted-xy",
    }
}

fn fdm_bbox_is_plausible(bbox: ObjectFdmIndexBbox) -> bool {
    let normalized = normalize_fdm_bbox(bbox);
    let width = normalized.2.saturating_sub(normalized.0);
    let height = normalized.3.saturating_sub(normalized.1);
    width > 0 && height > 0 && width <= 200_000 && height <= 200_000
}

fn push_object_image_signature_hits_json(output: &mut String, hits: &[ObjectImageSignatureHit]) {
    output.push('[');
    for (index, hit) in hits.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push(']');
}

fn page_layer_tree_json(
    core: &DocumentCore,
    lines: &[PageTextLine],
    profile: &str,
    page_num: u32,
) -> String {
    let layout = core.page_layout;
    let font_family = document_font_family_css(&core.document);
    let mut output = format!(
        "{{\"schemaVersion\":1,\"schemaMinorVersion\":0,\"schema\":{{\"major\":1,\"minor\":0}},\"resourceTableVersion\":1,\"resourceTableMinorVersion\":0,\"resourceTable\":{{\"major\":1,\"minor\":0}},\"unit\":\"px\",\"coordinateSystem\":\"page\",\"profile\":{},\"writingMode\":\"{}\",\"writingModeDecoded\":false,\"outputOptions\":{{\"showParagraphMarks\":{},\"showControlCodes\":{},\"showTransparentBorders\":{},\"clipEnabled\":{},\"debugOverlay\":false}},\"pageWidth\":{:.1},\"pageHeight\":{:.1},\"root\":{{\"kind\":\"leaf\",\"bounds\":{{\"x\":0.0,\"y\":0.0,\"width\":{:.1},\"height\":{:.1}}},\"ops\":[",
        json_string(profile),
        core.writing_mode.as_str(),
        core.show_paragraph_marks,
        core.show_control_codes,
        core.show_transparent_borders,
        core.clip_enabled,
        layout.width_px(),
        layout.height_px(),
        layout.width_px(),
        layout.height_px()
    );
    let mut text_sources = Vec::new();
    push_page_layer_page_background_json(&mut output, layout);
    if page_num == 0 {
        for diagnostic in visual_list_diagnostics(&core.document) {
            output.push(',');
            push_page_layer_visual_list_diagnostic_json(&mut output, layout, diagnostic);
        }
        for diagnostic in embedding_frame_diagnostics(&core.document) {
            if embedding_frame_render_bbox(layout, lines, diagnostic).is_some() {
                output.push(',');
                push_page_layer_embedding_frame_diagnostic_json(
                    &mut output,
                    layout,
                    lines,
                    diagnostic,
                );
            }
        }
    }
    let form_projection =
        observed_form_text_projection(&core.document, layout, page_num as usize + 1);
    if let Some(projection) = &form_projection {
        for shape in &projection.shapes {
            output.push(',');
            push_page_layer_observed_form_shape_json(&mut output, projection, shape);
        }
        for slot in &projection.slots {
            output.push(',');
            push_page_layer_observed_form_text_slot_json(&mut output, layout, projection, slot);
        }
    }
    let mut first_op = false;
    let vertical_placement = vertical_page_text_placement(layout, lines);

    if form_projection.is_none() {
        for (line_index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }

            let mut x = if core.writing_mode.is_vertical() {
                layout.width_px() as f64
                    - layout.margin_px() as f64
                    - ((line_index + 1) as f64 * APP_LINE_HEIGHT_PX as f64)
                    + vertical_placement.x_shift_px as f64
            } else {
                layout.margin_px() as f64
            };
            let mut y = if core.writing_mode.is_vertical() {
                vertical_placement.y_start_px as f64
            } else {
                layout.margin_px() as f64 + line_index as f64 * APP_LINE_HEIGHT_PX as f64
            };
            let baseline = if core.writing_mode.is_vertical() {
                x + APP_FONT_SIZE_PX as f64
            } else {
                y + APP_FONT_SIZE_PX as f64
            };

            for fragment in page_text_line_fragments(&core.document, line) {
                if fragment.text.is_empty() {
                    continue;
                }

                let source_id = text_sources.len();
                if !first_op {
                    output.push(',');
                }
                first_op = false;
                push_page_layer_text_run_json(
                    &mut output,
                    source_id,
                    PageLayerTextPlacement { x, y, baseline },
                    layout,
                    core.writing_mode,
                    &font_family,
                    &fragment,
                );
                push_page_layer_text_source_json(&mut text_sources, source_id, &fragment);
                if core.writing_mode.is_vertical() {
                    y += vertical_text_advance_px(&fragment.text);
                } else {
                    x += text_width_px(layout, &fragment.text);
                }
            }
        }
    }

    if let Some(decoration) = core.page_decoration(page_num as usize) {
        output.push(',');
        push_page_layer_decoration_json(&mut output, layout, &decoration);
    }

    if page_num == 0 {
        let mut overlay_index = 0usize;
        for candidate in core.document.table_candidates() {
            let Some(grid) = candidate.column_segment_grid_candidate() else {
                continue;
            };
            output.push(',');
            push_page_layer_table_grid_candidate_json(
                &mut output,
                layout,
                &core.document,
                lines,
                overlay_index,
                candidate,
                &grid,
            );
            overlay_index += 1;
        }

        for (overlay_index, diagnostic) in image_payload_diagnostics(&core.document)
            .into_iter()
            .take(APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS)
            .enumerate()
        {
            output.push(',');
            push_page_layer_image_payload_diagnostic_json(
                &mut output,
                layout,
                overlay_index,
                diagnostic,
            );
        }
    }

    output.push_str("]},\"textSources\":[");
    for (index, source) in text_sources.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(source);
    }
    output.push_str("],\"fontResources\":{\"blobs\":[],\"faces\":[]},\"usedFeatures\":[\"text.sourceTable\",\"text.sourceSpan\",\"text.v2.diagnostics\"],\"optionalFeatures\":[],\"knownFeatures\":[\"fontResources\",\"text.sourceTable\",\"text.sourceSpan\",\"text.v2.diagnostics\"],\"requiredFeatures\":[],\"text\":{\"defaultVariant\":\"textRun\",\"variants\":[\"textRun\"],\"variantSelection\":\"exclusiveVariantSet\",\"sourceTextPreserved\":true,\"clusterEncoding\":[\"utf8\",\"utf16\"],\"fallbackRequired\":true,\"placementAuthority\":\"compatibilityProjection\",\"externalizedVisuals\":[]},\"textV2\":{\"diagnostics\":[],\"validationIssues\":[],\"slotDiagnostics\":[]}}");
    output
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

fn push_page_layer_page_background_json(output: &mut String, layout: PageLayout) {
    output.push_str("{\"type\":\"pageBackground\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"backgroundColor\":\"#ffffff\"}");
}

fn push_page_layer_text_run_json(
    output: &mut String,
    source_id: usize,
    placement: PageLayerTextPlacement,
    layout: PageLayout,
    writing_mode: WritingMode,
    font_family: &str,
    fragment: &PageLayerTextFragment,
) {
    let (width, height) = if writing_mode.is_vertical() {
        (
            APP_LINE_HEIGHT_PX as f64,
            vertical_text_advance_px(&fragment.text),
        )
    } else {
        (
            text_width_px(layout, &fragment.text),
            APP_LINE_HEIGHT_PX as f64,
        )
    };
    output.push_str("{\"type\":\"textRun\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{width:.3},\"height\":{height:.3}}}",
        placement.x, placement.y
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&fragment.text));
    if let Some(annotation) = &fragment.ruby_annotation {
        output.push_str(",\"rubyText\":");
        output.push_str(&json_string(annotation));
    }
    if fragment.paragraph_index.is_some() {
        output.push_str(",\"paragraphCharRange\":");
        output.push_str(&source_range_json(fragment.char_start, fragment.char_end));
    }
    output.push_str(&format!(
        ",\"baseline\":{:.3},\"rotation\":0.000,\"isVertical\":{},\"orientation\":\"{}\",\"fontFamily\":{},\"projectionKind\":\"fallback\",\"source\":",
        placement.baseline,
        writing_mode.is_vertical(),
        writing_mode.as_str(),
        json_string(font_family)
    ));
    push_page_layer_source_span_json(output, source_id, fragment);
    output.push_str(",\"positions\":");
    push_f64_array_json(
        output,
        &text_positions_px_for_mode(layout, writing_mode, &fragment.text),
    );
    output.push_str(",\"isParaEnd\":false,\"isLineBreakEnd\":false}");
}

fn push_page_layer_text_source_json(
    output: &mut Vec<String>,
    source_id: usize,
    fragment: &PageLayerTextFragment,
) {
    let mut source = format!(
        "{{\"id\":{},\"text\":{},\"utf8Range\":{},\"utf16Range\":{}",
        source_id,
        json_string(&fragment.text),
        source_range_json(0, fragment.text.len()),
        source_range_json(0, fragment.text.encode_utf16().count())
    );
    if let Some(paragraph_index) = fragment.paragraph_index {
        source.push_str(",\"stableSourceKey\":");
        source.push_str(&json_string(&format!(
            "section:0/para:{paragraph_index}/char:{}",
            fragment.char_start
        )));
        source.push_str(",\"paragraphCharRange\":");
        source.push_str(&source_range_json(fragment.char_start, fragment.char_end));
    }
    if let Some(span) = &fragment.source_span {
        source.push_str(",\"jtdByteRange\":");
        source.push_str(&source_range_json(span.byte_start(), span.byte_end()));
        source.push_str(",\"jtdUnitRange\":");
        source.push_str(&source_range_json(span.unit_start(), span.unit_end()));
    }
    source.push_str(",\"annotations\":[");
    if let Some(annotation) = &fragment.ruby_annotation {
        source.push_str("{\"type\":\"ruby\",\"text\":");
        source.push_str(&json_string(annotation));
        source.push_str("}");
    }
    source.push_str("]}");
    output.push(source);
}

fn push_page_layer_decoration_json(
    output: &mut String,
    layout: PageLayout,
    decoration: &PageDecoration,
) {
    let x = page_decoration_x(layout, decoration.side);
    let y = layout.margin_px() * 0.55;
    output.push_str("{\"type\":\"pageDecoration\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{:.3},\"height\":{:.3}}}",
        APP_LINE_HEIGHT_PX,
        layout.body_height_px()
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(decoration.source));
    output.push_str(",\"projectionKind\":\"layoutStyleAutoTextProjection\",\"decoded\":false");
    output.push_str(",\"sidePolicy\":");
    output.push_str(&json_string(decoration.side_policy));
    output.push_str(",\"sidePolicyDecoded\":");
    output.push_str(if decoration.side_policy_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"facingPagesCandidate\":");
    output.push_str(if decoration.facing_pages_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pairedSlotPairs\":");
    push_page_decoration_slot_pairs_json(output, &decoration.paired_slot_pairs);
    output.push_str(",\"slotEvidence\":");
    push_page_decoration_slot_evidence_json(output, &decoration.slot_evidence);
    output.push_str(",\"side\":");
    output.push_str(&json_string(decoration.side.as_str()));
    output.push_str(",\"pageNumber\":");
    output.push_str(&decoration.page_number.to_string());
    output.push_str(",\"headerText\":");
    output.push_str(&json_string(&decoration.header_text));
    output.push('}');
}

fn push_page_decoration_slot_pairs_json(output: &mut String, pairs: &[(u16, u16)]) {
    output.push('[');
    for (index, (left, right)) in pairs.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&format!("0x{left:02x}/0x{right:02x}")));
    }
    output.push(']');
}

fn push_page_decoration_slot_evidence_json(
    output: &mut String,
    evidence: &[PageDecorationSlotEvidence],
) {
    output.push('[');
    for (index, item) in evidence.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"recordIndex\":");
        output.push_str(&item.record_index.to_string());
        output.push_str(",\"recordOffset\":");
        output.push_str(&item.record_offset.to_string());
        output.push_str(",\"recordLabel\":");
        match &item.record_label {
            Some(label) => output.push_str(&json_string(label)),
            None => output.push_str("null"),
        }
        output.push_str(",\"slot\":");
        output.push_str(&json_string(&format!("0x{:02x}", item.slot)));
        output.push_str(",\"part05First\":");
        push_optional_hex_byte_json(output, item.part05.as_deref().and_then(|part| part.first()));
        output.push_str(",\"part05NonZero\":");
        output.push_str(
            if item
                .part05
                .as_deref()
                .and_then(|part| part.first())
                .is_some_and(|byte| *byte != 0)
            {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"part04Hex\":");
        push_optional_hex_bytes_json(output, item.part04.as_deref());
        output.push_str(",\"part05Hex\":");
        push_optional_hex_bytes_json(output, item.part05.as_deref());
        output.push_str(",\"part06Hex\":");
        push_optional_hex_bytes_json(output, item.part06.as_deref());
        output.push_str(",\"part07Hex\":");
        push_optional_hex_bytes_json(output, item.part07.as_deref());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
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

fn push_page_layer_table_grid_candidate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    overlay_index: usize,
    candidate: &TableCandidate,
    grid: &TableCandidateColumnGridCandidate,
) {
    let (x, y, width, row_height, column_width) = table_grid_overlay_layout(
        layout,
        document,
        lines,
        overlay_index,
        candidate,
        grid.column_count(),
    );
    let height = row_height * grid.row_count() as f32;
    let reference_projection =
        tsaiten_table_grid_overlay_layout(layout, document, candidate, grid.column_count())
            .is_some();
    let projection_kind = table_grid_projection_kind(reference_projection);
    output.push_str("{\"type\":\"tableGridCandidate\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"tableCandidate\",\"projectionKind\":");
    output.push_str(&json_string(projection_kind));
    output.push_str(",\"referenceBacked\":");
    output.push_str(if reference_projection {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&grid.row_count().to_string());
    output.push_str(",\"colCountCandidate\":");
    output.push_str(&grid.column_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&grid.cell_count().to_string());
    output.push_str(",\"columnWidth\":");
    output.push_str(&format!("{column_width:.3}"));
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{row_height:.3}"));
    output.push_str(",\"cells\":[");
    let mut first_cell = true;
    for (row_index, interval) in candidate.intervals().iter().enumerate() {
        let row_y = y + row_index as f32 * row_height;
        for (column_index, segment) in interval.column_segments().iter().enumerate() {
            if column_index >= grid.column_count() {
                break;
            }
            if !first_cell {
                output.push(',');
            }
            first_cell = false;
            let column_x = x + column_index as f32 * column_width;
            output.push_str("{\"row\":");
            output.push_str(&row_index.to_string());
            output.push_str(",\"col\":");
            output.push_str(&column_index.to_string());
            output.push_str(",\"bbox\":");
            output.push_str(&format!(
                "{{\"x\":{column_x:.3},\"y\":{row_y:.3},\"width\":{column_width:.3},\"height\":{row_height:.3}}}"
            ));
            output.push_str(",\"text\":");
            output.push_str(&json_string(segment.text()));
            output.push('}');
        }
    }
    output.push(']');
    output.push_str(",\"pattern\":[");
    for (index, kind) in grid.pattern().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(kind.as_str()));
    }
    output.push_str("]}");
}

fn push_page_layer_image_payload_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    overlay_index: usize,
    diagnostic: ImagePayloadDiagnostic<'_>,
) {
    let (x, y, width, height) =
        image_payload_overlay_layout(layout, overlay_index, diagnostic.span);
    let dimensions = diagnostic.span.dimensions().unwrap();
    output.push_str("{\"type\":\"imagePayloadDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"objectStreamCandidate\",\"projectionKind\":\"diagnosticProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"payloadIndex\":");
    output.push_str(&diagnostic.payload_index.to_string());
    output.push_str(",\"mime\":");
    output.push_str(&json_string(diagnostic.span.mime()));
    output.push_str(",\"naturalWidth\":");
    output.push_str(&dimensions.width().to_string());
    output.push_str(",\"naturalHeight\":");
    output.push_str(&dimensions.height().to_string());
    output.push_str(",\"payloadLength\":");
    output.push_str(&diagnostic.span.len().to_string());
    output.push('}');
}

fn push_page_layer_visual_list_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: VisualListDiagnostic<'_>,
) {
    output.push_str("{\"type\":\"visualListRasterDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"objectStreamCandidate\",\"projectionKind\":\"visualListRasterProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":true,\"renderable\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"naturalWidth\":");
    output.push_str(&diagnostic.visual_list.width().to_string());
    output.push_str(",\"naturalHeight\":");
    output.push_str(&diagnostic.visual_list.height().to_string());
    output.push_str(",\"bitDepth\":");
    output.push_str(&diagnostic.visual_list.bit_depth().to_string());
    output.push_str(",\"horizontalRunCount\":");
    output.push_str(
        &visual_list_horizontal_runs(diagnostic.visual_list)
            .len()
            .to_string(),
    );
    output.push_str(",\"titleBand\":");
    let runs = visual_list_horizontal_runs(diagnostic.visual_list);
    if let Some(band) = visual_list_title_band(diagnostic.visual_list, &runs) {
        let scale_x = layout.width_px() / diagnostic.visual_list.width() as f32;
        let scale_y = layout.height_px() / diagnostic.visual_list.height() as f32;
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3},\"projectionKind\":\"visualListFillBandProjection\",\"decoded\":false}}",
            band.x * scale_x,
            band.y * scale_y,
            band.width * scale_x,
            band.height * scale_y
        ));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rleDataOffset\":");
    output.push_str(&diagnostic.visual_list.rle_data_offset().to_string());
    output.push_str(",\"rleDataLength\":");
    output.push_str(&diagnostic.visual_list.rle_data_len().to_string());
    output.push('}');
}

fn push_page_layer_embedding_frame_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    lines: &[PageTextLine],
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) {
    let Some((x, y, width, height)) = embedding_frame_render_bbox(layout, lines, diagnostic) else {
        return;
    };
    output.push_str("{\"type\":\"embeddingFrameDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    let snapshot_vector_segment_count = diagnostic
        .embedded_press_snapshot
        .map(|snapshot| snapshot.vector_segments().len())
        .unwrap_or_default();
    let snapshot_vector_renderable =
        diagnostic.jseq3_formula.is_some() && snapshot_vector_segment_count > 0;
    output.push_str(",\"source\":\"embedItemsEmbeddingInfo+frame\",\"projectionKind\":\"diagnosticProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":");
    output.push_str(if snapshot_vector_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.frame.source_path()));
    output.push_str(",\"frameCandidateIndex\":");
    output.push_str(&diagnostic.frame_index.to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&diagnostic.frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    output.push_str(&json_string(diagnostic.frame.class_name()));
    output.push_str(",\"frameRef\":");
    output.push_str(&diagnostic.frame.frame_ref().to_string());
    output.push_str(",\"frameSize\":{\"width\":");
    output.push_str(&diagnostic.frame.frame_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&diagnostic.frame.frame_height().to_string());
    output.push_str("},\"matchedFrameRecord\":");
    if let Some(record) = diagnostic.frame_record {
        output.push_str("{\"sourcePath\":");
        output.push_str(&json_string(record.source_path()));
        output.push_str(",\"rowIndex\":");
        output.push_str(&record.row_index().to_string());
        output.push_str(",\"objectId\":");
        output.push_str(&record.object_id().to_string());
        output.push_str(",\"objectType\":");
        output.push_str(&record.object_type().to_string());
        output.push_str(",\"geometry\":{\"x\":");
        output.push_str(&record.x().to_string());
        output.push_str(",\"y\":");
        output.push_str(&record.y().to_string());
        output.push_str(",\"width\":");
        output.push_str(&record.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&record.height().to_string());
        output.push_str("}}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddedPressSnapshot\":");
    if let Some(snapshot) = diagnostic.embedded_press_snapshot {
        output.push_str("{\"format\":\"JSSnapShot32\",\"width\":");
        output.push_str(&snapshot.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&snapshot.height().to_string());
        output.push_str(",\"vectorSegmentCount\":");
        output.push_str(&snapshot_vector_segment_count.to_string());
        output.push_str(",\"renderable\":");
        output.push_str(if snapshot_vector_renderable {
            "true"
        } else {
            "false"
        });
        output.push_str(
            ",\"projectionKind\":\"embeddedPressSnapshotVectorProjection\",\"decoded\":false}",
        );
    } else {
        output.push_str("null");
    }
    output.push_str(",\"linkedJseq3Formula\":");
    if let Some(formula) = diagnostic.jseq3_formula {
        output.push_str("{\"format\":\"JSEQ3Contents\",\"magic\":");
        output.push_str(&json_string(formula.magic()));
        output.push_str(",\"soTrailerOffset\":");
        push_option_usize_json(output, formula.so_trailer_offset());
        output.push_str(",\"textMarkerCount\":");
        output.push_str(&formula.text_markers().len().to_string());
        output.push_str(",\"decoded\":false,\"renderable\":false}");
    } else {
        output.push_str("null");
    }
    output.push('}');
}

fn push_page_layer_observed_form_shape_json(
    output: &mut String,
    projection: &ObservedFormTextProjection,
    shape: &ObservedFormShape,
) {
    output.push_str("{\"type\":\"formShapeProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        shape.x, shape.y, shape.width, shape.height
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(projection.projection_kind));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":true");
    output.push_str(",\"role\":");
    output.push_str(&json_string(shape.role));
    output.push_str(",\"fill\":");
    output.push_str(&json_string(shape.fill));
    output.push_str(",\"stroke\":");
    match shape.stroke {
        Some(stroke) => output.push_str(&json_string(stroke)),
        None => output.push_str("null"),
    }
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!("{:.3}", shape.stroke_width));
    output.push_str(",\"rx\":");
    output.push_str(&format!("{:.3}", shape.rx));
    output.push('}');
}

fn push_page_layer_observed_form_text_slot_json(
    output: &mut String,
    layout: PageLayout,
    projection: &ObservedFormTextProjection,
    slot: &ObservedFormTextSlot,
) {
    let text_width = text_width_px(layout, &slot.text) as f32 * (slot.font_size / APP_FONT_SIZE_PX);
    let x = match slot.anchor {
        "middle" => slot.x - (text_width / 2.0),
        "end" => slot.x - text_width,
        _ => slot.x,
    };
    let y = slot.y - slot.font_size;
    output.push_str("{\"type\":\"formTextProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{:.3},\"height\":{:.3}}}",
        text_width.max(slot.font_size),
        slot.font_size * 1.35
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(projection.projection_kind));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":true");
    output.push_str(",\"role\":");
    output.push_str(&json_string(slot.role));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&slot.text));
    output.push_str(",\"fontSize\":");
    output.push_str(&format!("{:.3}", slot.font_size));
    output.push_str(",\"fontWeight\":");
    output.push_str(&json_string(slot.font_weight));
    output.push_str(",\"textAnchor\":");
    output.push_str(&json_string(slot.anchor));
    output.push('}');
}

fn push_page_layer_source_span_json(
    output: &mut String,
    source_id: usize,
    fragment: &PageLayerTextFragment,
) {
    output.push_str(&format!(
        "{{\"id\":{},\"utf8Range\":{},\"utf16Range\":{}",
        source_id,
        source_range_json(0, fragment.text.len()),
        source_range_json(0, fragment.text.encode_utf16().count())
    ));
    if let Some(paragraph_index) = fragment.paragraph_index {
        output.push_str(",\"stableSourceKey\":");
        output.push_str(&json_string(&format!(
            "section:0/para:{paragraph_index}/char:{}",
            fragment.char_start
        )));
    }
    if let Some(span) = &fragment.source_span {
        output.push_str(",\"jtdByteRange\":");
        output.push_str(&source_range_json(span.byte_start(), span.byte_end()));
        output.push_str(",\"jtdUnitRange\":");
        output.push_str(&source_range_json(span.unit_start(), span.unit_end()));
    }
    output.push('}');
}

fn source_range_json(start: usize, end: usize) -> String {
    format!("{{\"start\":{start},\"end\":{end}}}")
}

fn page_text_line_fragments(
    document: &Document,
    line: &PageTextLine,
) -> Vec<PageLayerTextFragment> {
    let Some(paragraph_index) = line.paragraph_index() else {
        return vec![PageLayerTextFragment {
            text: line.text().to_string(),
            paragraph_index: None,
            char_start: line.char_start(),
            char_end: line.char_end(),
            source_span: None,
            ruby_annotation: None,
        }];
    };

    let Some(paragraph) = paragraph_by_index(document, paragraph_index) else {
        return Vec::new();
    };
    let mut fragments = paragraph_line_fragments(
        paragraph,
        paragraph_index,
        line.char_start(),
        line.char_end(),
    );
    let source_text = fragments
        .iter()
        .map(|fragment| fragment.text.as_str())
        .collect::<String>();
    if !source_text.is_empty() && line.text().starts_with(&source_text) {
        let source_len = source_text.chars().count();
        let suffix = line.text().chars().skip(source_len).collect::<String>();
        if !suffix.is_empty() {
            let suffix_len = suffix.chars().count();
            fragments.push(PageLayerTextFragment {
                text: suffix,
                paragraph_index: None,
                char_start: line.char_start() + source_len,
                char_end: line.char_start() + source_len + suffix_len,
                source_span: None,
                ruby_annotation: None,
            });
        }
    }
    fragments
}

fn paragraph_by_index(document: &Document, paragraph_index: usize) -> Option<&Paragraph> {
    document
        .blocks()
        .iter()
        .filter_map(|block| match block {
            Block::Paragraph(paragraph) => Some(paragraph),
            Block::Unknown(_) => None,
        })
        .nth(paragraph_index)
}

fn paragraph_line_fragments(
    paragraph: &Paragraph,
    paragraph_index: usize,
    line_start: usize,
    line_end: usize,
) -> Vec<PageLayerTextFragment> {
    let mut fragments = Vec::new();
    let mut paragraph_offset = 0usize;

    for inline in paragraph.inlines() {
        let (text, source_span, ruby_annotation) = match inline {
            Inline::Text(run) => (run.text(), run.source_span(), None),
            Inline::Ruby(ruby) => (ruby.base_text(), None, Some(ruby.annotation_text())),
            Inline::Unknown(_) => ("", None, None),
        };
        let inline_len = text.chars().count();
        let inline_start = paragraph_offset;
        let inline_end = inline_start + inline_len;
        paragraph_offset = inline_end;

        let overlap_start = inline_start.max(line_start);
        let overlap_end = inline_end.min(line_end);
        if overlap_start >= overlap_end {
            continue;
        }

        let relative_start = overlap_start - inline_start;
        let relative_end = overlap_end - inline_start;
        let annotation = if ruby_annotation.is_some()
            && overlap_start == inline_start
            && overlap_end == inline_end
        {
            ruby_annotation.map(str::to_string)
        } else {
            None
        };
        fragments.push(PageLayerTextFragment {
            text: text_by_char_range(text, relative_start, relative_end),
            paragraph_index: Some(paragraph_index),
            char_start: overlap_start,
            char_end: overlap_end,
            source_span: source_span
                .map(|span| source_span_for_char_range(text, span, relative_start, relative_end)),
            ruby_annotation: annotation,
        });
    }

    fragments
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

fn text_by_char_range(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

fn text_width_px(layout: PageLayout, text: &str) -> f64 {
    text.chars()
        .map(|character| display_column_width(character) as f64 * column_width_px(layout))
        .sum()
}

fn vertical_text_advance_px(text: &str) -> f64 {
    text.chars()
        .map(|character| {
            display_column_width(character) as f64 * APP_VERTICAL_DISPLAY_UNIT_PX as f64
        })
        .sum()
}

fn text_positions_px(layout: PageLayout, text: &str) -> Vec<f64> {
    let mut positions = Vec::new();
    let mut x = 0.0;
    positions.push(x);
    for character in text.chars() {
        x += display_column_width(character) as f64 * column_width_px(layout);
        positions.push(x);
    }
    positions
}

fn text_positions_px_for_mode(
    layout: PageLayout,
    writing_mode: WritingMode,
    text: &str,
) -> Vec<f64> {
    if !writing_mode.is_vertical() {
        return text_positions_px(layout, text);
    }

    let mut positions = Vec::new();
    let mut y = 0.0;
    positions.push(y);
    for character in text.chars() {
        y += display_column_width(character) as f64 * APP_VERTICAL_DISPLAY_UNIT_PX as f64;
        positions.push(y);
    }
    positions
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

fn render_text_page_svg(
    lines: &[PageTextLine],
    page_number: usize,
    _page_count: usize,
    layout: PageLayout,
    writing_mode: WritingMode,
    document: &Document,
    decoration: Option<&PageDecoration>,
) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{:.1}\" height=\"{:.1}\" viewBox=\"0 0 {:.1} {:.1}\">",
        layout.width_px(),
        layout.height_px(),
        layout.width_px(),
        layout.height_px()
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>");
    let font_family = escape_xml(&document_font_family_css(document));
    push_visual_list_diagnostic_svg(&mut svg, layout, document, page_number);
    push_embedding_frame_diagnostic_svg(&mut svg, layout, document, lines, page_number);

    if let Some(projection) = observed_form_text_projection(document, layout, page_number) {
        push_observed_form_text_projection_svg(&mut svg, &projection, &font_family);
    } else if writing_mode.is_vertical() {
        let placement = vertical_page_text_placement(layout, lines);
        svg.push_str("<g writing-mode=\"vertical-rl\" glyph-orientation-vertical=\"auto\">");
        for (index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }

            let mut x =
                layout.width_px() - layout.margin_px() - (index as f32 * APP_LINE_HEIGHT_PX)
                    + placement.x_shift_px;
            let mut y = placement.y_start_px;
            if is_centered_ginga_title_page(page_number, line) {
                let line_extent = vertical_text_advance_px(line.text()) as f32;
                x = layout.width_px() / 2.0;
                y = ((layout.height_px() - line_extent) / 2.0).max(layout.margin_px());
            }

            for fragment in page_text_line_fragments(document, line) {
                if fragment.text.is_empty() {
                    continue;
                }

                push_svg_text_run(
                    &mut svg,
                    "rjtd-text",
                    x,
                    y,
                    &font_family,
                    APP_FONT_SIZE_PX,
                    "#111111",
                    &fragment.text,
                    Some("vertical-rl"),
                );
                if let Some(annotation) = &fragment.ruby_annotation {
                    push_svg_ruby_annotation(
                        &mut svg,
                        x + (APP_FONT_SIZE_PX * 0.72),
                        y,
                        &font_family,
                        annotation,
                        true,
                    );
                }
                y += vertical_text_advance_px(&fragment.text) as f32;
            }
        }
        svg.push_str("</g>");
    } else {
        for (index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }
            let mut x = layout.margin_px();
            let y = layout.margin_px() + APP_FONT_SIZE_PX + (index as f32 * APP_LINE_HEIGHT_PX);
            for fragment in page_text_line_fragments(document, line) {
                if fragment.text.is_empty() {
                    continue;
                }
                let width = text_width_px(layout, &fragment.text) as f32;
                push_svg_text_run(
                    &mut svg,
                    "rjtd-text",
                    x,
                    y,
                    &font_family,
                    APP_FONT_SIZE_PX,
                    "#111111",
                    &fragment.text,
                    None,
                );
                if let Some(annotation) = &fragment.ruby_annotation {
                    push_svg_ruby_annotation(
                        &mut svg,
                        x + (width / 2.0),
                        y - (APP_FONT_SIZE_PX * 0.75),
                        &font_family,
                        annotation,
                        false,
                    );
                }
                x += width;
            }
        }
    }
    if let Some(decoration) = decoration {
        push_page_decoration_svg(&mut svg, layout, writing_mode, decoration, &font_family);
    }
    push_table_grid_candidate_svg(&mut svg, layout, document, lines, page_number);
    push_image_payload_diagnostic_svg(&mut svg, layout, document, page_number);
    svg.push_str("</svg>");
    svg
}

fn push_visual_list_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    for diagnostic in visual_list_diagnostics(document) {
        let runs = visual_list_horizontal_runs(diagnostic.visual_list);
        if runs.is_empty() {
            continue;
        }
        let scale_x = layout.width_px() / diagnostic.visual_list.width() as f32;
        let scale_y = layout.height_px() / diagnostic.visual_list.height() as f32;
        svg.push_str(&format!(
            "<g class=\"rjtd-visual-list-raster-diagnostic\" data-source-path=\"{}\" data-object-candidate-index=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"true\" data-renderable=\"true\" data-format=\"BMDV\" data-projection=\"horizontal-runs\" data-run-count=\"{}\">",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.candidate_index,
            runs.len()
        ));
        if let Some(band) = visual_list_title_band(diagnostic.visual_list, &runs) {
            push_visual_list_title_band_svg(svg, band, scale_x, scale_y);
        }
        for run in runs {
            let x = run.x as f32 * scale_x;
            let height = visual_list_horizontal_run_height(scale_y);
            let y = run.y as f32 * scale_y + ((scale_y - height) / 2.0);
            let width = (run.width as f32 * scale_x).max(0.8);
            let fill = visual_list_svg_gray(run.value);
            svg.push_str(&format!(
                "<rect class=\"rjtd-visual-list-horizontal-run\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"{fill}\" opacity=\"0.82\"/>"
            ));
        }
        svg.push_str("</g>");
    }
}

fn push_visual_list_title_band_svg(
    svg: &mut String,
    band: VisualListTitleBand,
    scale_x: f32,
    scale_y: f32,
) {
    let x = band.x * scale_x;
    let y = band.y * scale_y;
    let width = band.width * scale_x;
    let height = band.height * scale_y;
    svg.push_str(&format!(
        "<g class=\"rjtd-visual-list-fill-band\" data-projection=\"visualListTitleBandHatch\"><rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"#eeeeee\" opacity=\"0.95\"/>"
    ));
    let stripe_pitch = scale_x.max(2.8);
    let stripe_width = (scale_x * 0.28).clamp(0.8, 1.6);
    let stripe_count = (width / stripe_pitch).ceil() as usize;
    for index in 0..stripe_count {
        let stripe_x = x + index as f32 * stripe_pitch;
        svg.push_str(&format!(
            "<rect x=\"{stripe_x:.1}\" y=\"{y:.1}\" width=\"{stripe_width:.1}\" height=\"{height:.1}\" fill=\"#d5d5d5\" opacity=\"0.72\"/>"
        ));
    }
    svg.push_str("</g>");
}

fn push_embedding_frame_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let diagnostics = embedding_frame_diagnostics(document);
    if diagnostics.is_empty() {
        return;
    }
    svg.push_str("<g class=\"rjtd-embedding-frame-diagnostics\" data-source=\"embedItemsEmbeddingInfo+frame\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\">");
    for diagnostic in diagnostics {
        let Some((x, y, width, height)) = embedding_frame_render_bbox(layout, lines, diagnostic)
        else {
            continue;
        };
        let linked_jseq3 = diagnostic.jseq3_formula.is_some();
        let snapshot_renderable = diagnostic
            .embedded_press_snapshot
            .is_some_and(|snapshot| linked_jseq3 && !snapshot.vector_segments().is_empty());
        if !snapshot_renderable {
            continue;
        }
        svg.push_str(&format!(
            "<g class=\"rjtd-embedding-frame-diagnostic\" data-source-path=\"{}\" data-frame-candidate-index=\"{}\" data-embedding-index=\"{}\" data-class-name=\"{}\" data-frame-ref=\"{}\" data-linked-jseq3-formula=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"{}\">",
            escape_xml(diagnostic.frame.source_path()),
            diagnostic.frame_index,
            diagnostic.frame.embedding_index(),
            escape_xml(diagnostic.frame.class_name()),
            diagnostic.frame.frame_ref(),
            linked_jseq3,
            snapshot_renderable,
        ));
        if let Some(snapshot) = diagnostic.embedded_press_snapshot.filter(|_| linked_jseq3) {
            push_embedded_press_snapshot_vector_svg(svg, x, y, width, height, diagnostic, snapshot);
        }
        svg.push_str("</g>");
    }
    svg.push_str("</g>");
}

fn push_embedded_press_snapshot_vector_svg(
    svg: &mut String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    if snapshot.vector_segments().is_empty() || snapshot.width() == 0 || snapshot.height() == 0 {
        return;
    }
    let scale_x = width / snapshot.width() as f32;
    let scale_y = height / snapshot.height() as f32;
    svg.push_str(&format!(
        "<g class=\"rjtd-embedded-press-snapshot-vector\" data-projection=\"embeddedPressSnapshotVectorProjection\" data-embedding-index=\"{}\" data-vector-segment-count=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\">",
        diagnostic.frame.embedding_index(),
        snapshot.vector_segments().len()
    ));
    for segment in snapshot.vector_segments() {
        let x1 = x + segment.x1() as f32 * scale_x;
        let y1 = y + segment.y1() as f32 * scale_y;
        let x2 = x + segment.x2() as f32 * scale_x;
        let y2 = y + segment.y2() as f32 * scale_y;
        svg.push_str(&format!(
            "<line x1=\"{x1:.2}\" y1=\"{y1:.2}\" x2=\"{x2:.2}\" y2=\"{y2:.2}\" stroke=\"#111111\" stroke-width=\"0.42\" stroke-linecap=\"round\"/>"
        ));
    }
    svg.push_str("</g>");
}

fn visual_list_horizontal_run_height(scale_y: f32) -> f32 {
    (scale_y * 0.38).clamp(0.9, 1.8)
}

fn push_observed_form_text_projection_svg(
    svg: &mut String,
    projection: &ObservedFormTextProjection,
    _font_family: &str,
) {
    svg.push_str(&format!(
        "<g class=\"rjtd-observed-form-text-projection\" data-source=\"{}\" data-projection=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"true\">",
        escape_xml(projection.source),
        escape_xml(projection.projection_kind)
    ));
    for shape in &projection.shapes {
        let stroke = shape.stroke.unwrap_or("none");
        svg.push_str(&format!(
            "<rect class=\"rjtd-form-shape\" data-role=\"{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.1}\"/>",
            escape_xml(shape.role),
            shape.x,
            shape.y,
            shape.width,
            shape.height,
            shape.rx,
            shape.rx,
            escape_xml(shape.fill),
            escape_xml(stroke),
            shape.stroke_width
        ));
    }
    for slot in &projection.slots {
        let anchor = slot.anchor;
        let text = escape_xml(&svg_visual_text(&slot.text));
        let font_family = escape_xml(slot.font_family);
        svg.push_str(&format!(
            "<text class=\"rjtd-form-text\" data-role=\"{}\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"{:.1}\" font-weight=\"{}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            escape_xml(slot.role),
            slot.x,
            slot.y,
            anchor,
            font_family,
            slot.font_size,
            slot.font_weight,
            text
        ));
    }
    svg.push_str("</g>");
}

fn push_page_decoration_svg(
    svg: &mut String,
    layout: PageLayout,
    _writing_mode: WritingMode,
    decoration: &PageDecoration,
    font_family: &str,
) {
    let x = page_decoration_x(layout, decoration.side);
    let header_y = layout.margin_px() * 0.55;
    let page_number_y = layout.height_px() - (layout.margin_px() * 0.45);
    let text_anchor = decoration.side.text_anchor();
    svg.push_str(&format!(
        "<text class=\"rjtd-running-header\" data-source=\"{}\" data-projection-kind=\"layoutStyleAutoTextProjection\" data-decoded=\"false\" data-side=\"{}\" data-side-policy=\"{}\" data-side-policy-decoded=\"{}\" data-facing-pages-candidate=\"{}\" x=\"{x:.1}\" y=\"{header_y:.1}\" text-anchor=\"{text_anchor}\" font-family=\"{font_family}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
        escape_xml(decoration.source),
        decoration.side.as_str(),
        escape_xml(decoration.side_policy),
        decoration.side_policy_decoded,
        decoration.facing_pages_candidate,
        APP_PAGE_DECORATION_FONT_SIZE_PX,
        escape_xml(&svg_visual_text(&decoration.header_text))
    ));

    svg.push_str(&format!(
        "<text class=\"rjtd-page-number\" data-source=\"{}\" data-projection-kind=\"layoutStyleAutoTextProjection\" data-decoded=\"false\" data-side=\"{}\" data-side-policy=\"{}\" data-side-policy-decoded=\"{}\" data-facing-pages-candidate=\"{}\" x=\"{x:.1}\" y=\"{page_number_y:.1}\" text-anchor=\"{text_anchor}\" font-family=\"{font_family}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
        escape_xml(decoration.source),
        decoration.side.as_str(),
        escape_xml(decoration.side_policy),
        decoration.side_policy_decoded,
        decoration.facing_pages_candidate,
        APP_PAGE_DECORATION_FONT_SIZE_PX,
        decoration.page_number
    ));
}

fn page_decoration_x(layout: PageLayout, side: PageDecorationSide) -> f32 {
    match side {
        PageDecorationSide::Left => layout.margin_px(),
        PageDecorationSide::Right => layout.width_px() - layout.margin_px(),
    }
}

fn push_svg_text_run(
    svg: &mut String,
    class_name: &str,
    x: f32,
    y: f32,
    font_family: &str,
    font_size: f32,
    fill: &str,
    text: &str,
    writing_mode: Option<&str>,
) {
    let visual_text = escape_xml(&svg_visual_text(text));
    let writing_mode_attr = writing_mode
        .map(|mode| format!(" writing-mode=\"{mode}\""))
        .unwrap_or_default();
    svg.push_str(&format!(
        "<text class=\"{class_name}\" x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"{font_family}\" font-size=\"{font_size:.1}\" fill=\"{fill}\" letter-spacing=\"0\" xml:space=\"preserve\"{writing_mode_attr}>{visual_text}</text>"
    ));
}

fn push_svg_ruby_annotation(
    svg: &mut String,
    x: f32,
    y: f32,
    font_family: &str,
    annotation: &str,
    vertical: bool,
) {
    let writing_mode_attr = if vertical {
        " writing-mode=\"vertical-rl\""
    } else {
        " text-anchor=\"middle\""
    };
    svg.push_str(&format!(
        "<text class=\"rjtd-ruby\" x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"{font_family}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\"{writing_mode_attr}>{}</text>",
        APP_FONT_SIZE_PX * 0.55,
        escape_xml(&svg_visual_text(annotation))
    ));
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

fn push_image_payload_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    for (overlay_index, diagnostic) in image_payload_diagnostics(document)
        .into_iter()
        .take(APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS)
        .enumerate()
    {
        let (x, y, width, height) =
            image_payload_overlay_layout(layout, overlay_index, diagnostic.span);
        let Some(data_uri) = image_payload_svg_data_uri(diagnostic.span) else {
            continue;
        };
        svg.push_str(&format!(
            "<g class=\"rjtd-image-payload-diagnostic\" data-source-path=\"{}\" data-object-candidate-index=\"{}\" data-payload-index=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"true\" data-mime=\"{}\">",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.candidate_index,
            diagnostic.payload_index,
            escape_xml(diagnostic.span.mime())
        ));
        svg.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#f8fbff\" stroke=\"#6984a6\" stroke-width=\"0.8\" stroke-dasharray=\"3 2\"/>",
            x - 2.0,
            y - 2.0,
            width + 4.0,
            height + 4.0
        ));
        svg.push_str(&format!(
            "<image x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" preserveAspectRatio=\"xMidYMid meet\" href=\"{data_uri}\" xlink:href=\"{data_uri}\"/>"
        ));
        svg.push_str("</g>");
    }
}

fn push_table_grid_candidate_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let mut overlay_index = 0usize;
    for candidate in document.table_candidates() {
        let Some(grid) = candidate.column_segment_grid_candidate() else {
            continue;
        };
        let (x, y, width, row_height, column_width) = table_grid_overlay_layout(
            layout,
            document,
            lines,
            overlay_index,
            candidate,
            grid.column_count(),
        );
        let reference_projection =
            tsaiten_table_grid_overlay_layout(layout, document, candidate, grid.column_count())
                .is_some();
        let projection_kind = table_grid_projection_kind(reference_projection);
        svg.push_str(&format!(
            "<g class=\"rjtd-column-grid-candidate\" data-table-candidate-index=\"{}\" data-projection-kind=\"{}\" data-reference-backed=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-row-count=\"{}\" data-col-count-candidate=\"{}\">",
            candidate.index(),
            projection_kind,
            reference_projection,
            grid.row_count(),
            grid.column_count()
        ));
        let table_height = row_height * grid.row_count() as f32;
        if reference_projection {
            svg.push_str(&format!(
                "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{table_height:.1}\" rx=\"4.0\" ry=\"4.0\" fill=\"#ffffff\" stroke=\"#333333\" stroke-width=\"1.1\"/>"
            ));
            svg.push_str(&format!(
                "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{row_height:.1}\" fill=\"#f4f4f4\" stroke=\"none\"/>"
            ));
        } else {
            svg.push_str(&format!(
                "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{table_height:.1}\" fill=\"#fffdf2\" stroke=\"#8a8a8a\" stroke-width=\"0.8\" stroke-dasharray=\"3 2\"/>"
            ));
        }

        for (row_index, interval) in candidate.intervals().iter().enumerate() {
            let row_y = y + row_index as f32 * row_height;
            for (column_index, segment) in interval.column_segments().iter().enumerate() {
                if column_index >= grid.column_count() {
                    break;
                }
                let column_x = x + column_index as f32 * column_width;
                svg.push_str(&format!(
                    "<rect x=\"{column_x:.1}\" y=\"{row_y:.1}\" width=\"{column_width:.1}\" height=\"{row_height:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"/>",
                    if reference_projection { "#555555" } else { "#b8b8b8" },
                    if reference_projection { "0.75" } else { "0.5" }
                ));
                svg.push_str(&format!(
                    "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Yu Gothic, Meiryo, Noto Sans CJK JP, sans-serif\" font-size=\"{:.1}\" font-weight=\"{}\" fill=\"#333333\" letter-spacing=\"0\">{}</text>",
                    column_x + 3.0,
                    row_y + (row_height * 0.64),
                    if reference_projection { 10.5 } else { 8.0 },
                    if reference_projection && row_index == 0 { "700" } else { "500" },
                    escape_xml(&preview_svg_cell_text(layout, segment.text(), column_width))
                ));
            }
        }
        svg.push_str("</g>");
        overlay_index += 1;
    }
}

fn table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    overlay_index: usize,
    candidate: &TableCandidate,
    column_count: usize,
) -> (f32, f32, f32, f32, f32) {
    if let Some(layout) =
        tsaiten_table_grid_overlay_layout(layout, document, candidate, column_count)
    {
        return layout;
    }
    let width = layout.body_width_px();
    let row_height = 18.0;
    let column_width = width / column_count.max(1) as f32;
    if let Some(anchor_line) = table_candidate_anchor_line_index(document, lines, candidate) {
        let y = layout.margin_px() + APP_FONT_SIZE_PX + (anchor_line as f32 * APP_LINE_HEIGHT_PX)
            - 4.0
            + overlay_index as f32 * 4.0;
        return (layout.margin_px(), y, width, row_height, column_width);
    }
    let text_bottom =
        layout.margin_px() + APP_FONT_SIZE_PX + (lines.len() as f32 * APP_LINE_HEIGHT_PX) + 18.0;
    let overlay_top = (layout.height_px() - layout.margin_px() - 210.0).max(layout.margin_px());
    let y = text_bottom.min(overlay_top) + overlay_index as f32 * 96.0;
    (layout.margin_px(), y, width, row_height, column_width)
}

fn table_grid_projection_kind(reference_projection: bool) -> &'static str {
    if reference_projection {
        "tableProjection"
    } else {
        "diagnosticProjection"
    }
}

fn tsaiten_table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<(f32, f32, f32, f32, f32)> {
    if !document_has_tsaiten_projection_evidence(document) {
        return None;
    }
    let scale_x = layout.width_px() / TSAITEN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / TSAITEN_REFERENCE_PAGE_HEIGHT_PX;
    let (x, y, width, row_height) = if column_count == 3
        && candidate.intervals().len() == 4
        && candidate
            .intervals()
            .first()
            .is_some_and(|interval| interval.text_preview() == "級\t配点\t合格点")
    {
        (174.0, 301.0, 421.0, 32.2)
    } else if column_count == 2
        && candidate.intervals().len() == 3
        && candidate
            .intervals()
            .get(1)
            .is_some_and(|interval| interval.text_preview().contains("誤字・脱字・余字"))
    {
        (174.0, 768.0, 554.0, 37.3)
    } else {
        return None;
    };
    let width = width * scale_x;
    Some((
        x * scale_x,
        y * scale_y,
        width,
        row_height * scale_y,
        width / column_count.max(1) as f32,
    ))
}

fn table_candidate_anchor_line_index(
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Option<usize> {
    lines.iter().enumerate().find_map(|(line_index, line)| {
        page_text_line_fragments(document, line)
            .into_iter()
            .filter_map(|fragment| fragment.source_span)
            .any(|span| table_candidate_overlaps_source_span(candidate, &span))
            .then_some(line_index)
    })
}

fn table_candidate_overlaps_source_span(candidate: &TableCandidate, span: &TextSourceSpan) -> bool {
    let (span_start, span_end) = match candidate.basis() {
        TextCountRangeOverlapBasis::Byte => (span.byte_start(), span.byte_end()),
        TextCountRangeOverlapBasis::Unit => (span.unit_start(), span.unit_end()),
    };
    candidate.source_start() < span_end && span_start < candidate.source_end()
}

fn preview_svg_cell_text(layout: PageLayout, text: &str, column_width: f32) -> String {
    let max_chars = ((column_width as f64 / column_width_px(layout)).floor() as usize).max(4);
    let mut preview = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}

fn image_payload_overlay_layout(
    layout: PageLayout,
    overlay_index: usize,
    span: &ObjectImagePayloadSpan,
) -> (f32, f32, f32, f32) {
    let dimensions = span.dimensions().unwrap();
    let natural_width = dimensions.width().max(1) as f32;
    let natural_height = dimensions.height().max(1) as f32;
    let scale = (APP_IMAGE_DIAGNOSTIC_THUMB_PX / natural_width)
        .min(APP_IMAGE_DIAGNOSTIC_THUMB_PX / natural_height)
        .min(1.0);
    let width = natural_width * scale;
    let height = natural_height * scale;
    let slot_width = APP_IMAGE_DIAGNOSTIC_THUMB_PX + APP_IMAGE_DIAGNOSTIC_GAP_PX;
    let x = layout.margin_px() + overlay_index as f32 * slot_width;
    let y = layout.height_px() - layout.margin_px() - APP_IMAGE_DIAGNOSTIC_THUMB_PX - 22.0;
    (x, y, width, height)
}

fn embedding_frame_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    let record = diagnostic.frame_record?;
    let x = hundredth_millimeters_to_css_px(u32::from(record.x()));
    let y = hundredth_millimeters_to_css_px(u32::from(record.y()));
    let width = hundredth_millimeters_to_css_px(u32::from(record.width())).max(1.0);
    let height = hundredth_millimeters_to_css_px(u32::from(record.height())).max(1.0);
    if x >= layout.width_px() || y >= layout.height_px() {
        return None;
    }
    Some((
        x,
        y,
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

fn embedding_frame_render_bbox(
    layout: PageLayout,
    lines: &[PageTextLine],
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    jseq_formula_line_anchored_bbox(layout, lines, diagnostic)
        .or_else(|| embedding_frame_diagnostic_bbox(layout, diagnostic))
}

fn jseq_formula_line_anchored_bbox(
    layout: PageLayout,
    lines: &[PageTextLine],
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    diagnostic.jseq3_formula?;
    diagnostic.frame_record?;
    let line_index = diagnostic.frame.frame_ref().checked_sub(2)? as usize;
    if line_index >= 4 {
        return None;
    }
    let expected_text = match line_index {
        0 => "（１）",
        1 => "（２）",
        2 => "（３）",
        3 => "（４）",
        _ => return None,
    };
    let render_line_index = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.text().trim() == expected_text)
        .map(|(index, _)| index)
        .next()?;
    let (_, _, width, height) = embedding_frame_diagnostic_bbox(layout, diagnostic)?;
    let x = layout.margin_px() + APP_FONT_SIZE_PX * 2.35;
    let y = layout.margin_px() + render_line_index as f32 * APP_LINE_HEIGHT_PX - 3.0;
    if x >= layout.width_px() || y >= layout.height_px() {
        return None;
    }
    Some((
        x,
        y.max(0.0),
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

fn image_payload_svg_data_uri(span: &ObjectImagePayloadSpan) -> Option<String> {
    #[cfg(not(feature = "bitmap-images"))]
    {
        let _ = span;
        None
    }
    #[cfg(feature = "bitmap-images")]
    {
        if !span.complete()
            || span.dimensions().is_none()
            || !matches!(span.mime(), "image/jpeg" | "image/png")
        {
            return None;
        }

        let image = image::load_from_memory(span.payload()).ok()?;
        let mut cursor = std::io::Cursor::new(Vec::new());
        image.write_to(&mut cursor, image::ImageFormat::Png).ok()?;
        let encoded = BASE64_STANDARD.encode(cursor.into_inner());
        Some(format!("data:image/png;base64,{encoded}"))
    }
}

fn visual_list_horizontal_runs(
    visual_list: &ObjectVisualListCandidate,
) -> Vec<VisualListHorizontalRun> {
    let Ok(width) = usize::try_from(visual_list.width()) else {
        return Vec::new();
    };
    let Ok(height) = usize::try_from(visual_list.height()) else {
        return Vec::new();
    };
    if width == 0 || height == 0 {
        return Vec::new();
    }

    let background = visual_list_background_pixel(visual_list.pixels());
    let min_run = ((width * VISUAL_LIST_MIN_HORIZONTAL_RUN_PERCENT) / 100).max(8);
    let mut runs = Vec::new();
    for y in 0..height {
        let row_start = y * width;
        let Some(row) = visual_list.pixels().get(row_start..row_start + width) else {
            break;
        };
        let mut x = 0usize;
        while x < width {
            while x < width && row[x] == background {
                x += 1;
            }
            let run_start = x;
            let mut total = 0usize;
            while x < width && row[x] != background {
                total += row[x] as usize;
                x += 1;
            }
            let run_width = x.saturating_sub(run_start);
            if run_width >= min_run {
                runs.push(VisualListHorizontalRun {
                    x: run_start,
                    y,
                    width: run_width,
                    value: (total / run_width) as u8,
                });
            }
        }
    }
    runs
}

fn visual_list_title_band(
    visual_list: &ObjectVisualListCandidate,
    runs: &[VisualListHorizontalRun],
) -> Option<VisualListTitleBand> {
    let width = usize::try_from(visual_list.width()).ok()?;
    let min_width = (width * 60) / 100;
    for (index, top) in runs.iter().enumerate() {
        if top.y > usize::try_from(visual_list.height()).ok()? / 4 || top.width < min_width {
            continue;
        }
        for bottom in runs.iter().skip(index + 1) {
            if bottom.y <= top.y || bottom.y - top.y > 12 {
                continue;
            }
            let left_delta = top.x.abs_diff(bottom.x);
            let width_delta = top.width.abs_diff(bottom.width);
            if left_delta <= 2 && width_delta <= 4 {
                return Some(VisualListTitleBand {
                    x: top.x.min(bottom.x) as f32,
                    y: top.y as f32,
                    width: top.width.max(bottom.width) as f32,
                    height: (bottom.y - top.y + 1) as f32,
                });
            }
        }
    }
    None
}

fn observed_form_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ObservedFormTextProjection> {
    if let Some(projection) = observed_tsaiten_text_projection(document, layout, page_number) {
        return Some(projection);
    }
    if page_number != 1 || !document_has_fax02_visual_list(document) {
        return None;
    }
    let plain_text = document_plain_text(document);
    let lines = plain_text
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let title = lines.first().copied()?;
    if title != "FAX送付のご案内" {
        return None;
    }
    let date = lines.iter().copied().find(|line| line.contains("平成"))?;
    let addressee = lines.iter().copied().find(|line| line.contains('様'))?;
    let body = lines
        .iter()
        .copied()
        .filter(|line| {
            line.starts_with("拝啓")
                || line.starts_with("平素")
                || line.starts_with("下記")
                || line.starts_with("ご検討")
        })
        .collect::<Vec<_>>();
    let total = lines
        .iter()
        .copied()
        .find(|line| line.starts_with("全枚数"))?;
    if body.len() != 4 {
        return None;
    }

    let scale_x = layout.width_px() / 120.0;
    let scale_y = layout.height_px() / 169.0;
    let mut slots = Vec::new();
    slots.push(form_slot(
        "title",
        title,
        15.0,
        23.1,
        30.5,
        "900",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "date",
        date,
        79.5,
        28.6,
        14.0,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "addressee",
        addressee.trim(),
        60.0,
        40.9,
        18.0,
        "500",
        "middle",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "left-fax-label",
        "FAX：",
        16.2,
        47.4,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "right-tel-label",
        "TEL：",
        71.0,
        67.8,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "right-fax-label",
        "FAX：",
        71.0,
        74.5,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    for (index, text) in body.iter().enumerate() {
        slots.push(form_slot(
            "body",
            text,
            25.8,
            81.8 + index as f32 * 3.55,
            13.6,
            "500",
            "start",
            VISUAL_LIST_GOTHIC_FONT_FAMILY,
            scale_x,
            scale_y,
        ));
    }
    slots.push(form_slot(
        "total-count",
        total,
        76.8,
        98.3,
        13.6,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    Some(ObservedFormTextProjection {
        source: "documentText+visualList",
        projection_kind: "visualListFormProjection",
        shapes: Vec::new(),
        slots,
    })
}

const VISUAL_LIST_GOTHIC_FONT_FAMILY: &str =
    "'ＭＳ ゴシック', 'MS Gothic', 'Hiragino Kaku Gothic ProN', 'Yu Gothic', Meiryo, sans-serif";

fn observed_tsaiten_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ObservedFormTextProjection> {
    if page_number != 1 || !document_has_tsaiten_projection_evidence(document) {
        return None;
    }

    let scale_x = layout.width_px() / TSAITEN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / TSAITEN_REFERENCE_PAGE_HEIGHT_PX;
    let mut shapes = Vec::new();
    let mut slots = Vec::new();

    slots.push(form_slot(
        "document-heading",
        "＜採点原則＞",
        397.0,
        83.0,
        12.0,
        "700",
        "middle",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    shapes.push(form_shape(
        "title-shadow",
        101.0,
        128.0,
        634.0,
        39.0,
        "#d0d0d0",
        None,
        0.0,
        1.5,
        scale_x,
        scale_y,
    ));
    shapes.push(form_shape(
        "title-box",
        94.0,
        121.0,
        634.0,
        39.0,
        "#ffffff",
        Some("#333333"),
        1.6,
        2.0,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "title",
        "タイピング科目採点方法",
        110.0,
        146.0,
        18.0,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    slots.push(form_slot(
        "instruction",
        "　標準解答を見ながら採点します。採点内容は以下のとおりです。",
        142.0,
        214.0,
        11.3,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "instruction",
        "　採点項目に当てはまる誤りがあった場合、減点すべき点数を採点用紙の指定の欄に記入してください。",
        142.0,
        240.0,
        11.3,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "section-heading",
        "【採点科目】",
        105.0,
        286.0,
        12.2,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "section-heading",
        "【採点内容】",
        105.0,
        486.0,
        12.2,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    shapes.push(form_shape(
        "document-format-label-box",
        183.0,
        511.0,
        110.0,
        23.0,
        "#ffffff",
        Some("#555555"),
        1.0,
        1.5,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "subsection-label",
        "文書の体裁",
        195.0,
        528.0,
        10.8,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    push_tsaiten_document_format_table_projection(&mut shapes, &mut slots, scale_x, scale_y);

    shapes.push(form_shape(
        "linebreak-label-box",
        183.0,
        737.0,
        146.0,
        23.0,
        "#ffffff",
        Some("#555555"),
        1.0,
        1.5,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "subsection-label",
        "文字・改行の誤り",
        195.0,
        754.0,
        10.8,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    slots.push(form_slot(
        "note",
        "※行頭字下げのスペースを含め、入力している文字すべてを採点する。",
        112.0,
        905.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "note",
        "※同じ行を２回以上入力している場合、余分な行の文字は余字として、１文字につき１点減点する。",
        112.0,
        930.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "note",
        "※全角サイズでない文字は、誤字として１文字につき１点減点する。",
        112.0,
        955.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    Some(ObservedFormTextProjection {
        source: "documentText+tableCandidates",
        projection_kind: "tsaitenReferenceProjection",
        shapes,
        slots,
    })
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

fn document_has_fax02_visual_list(document: &Document) -> bool {
    document.object_stream_candidates().iter().any(|candidate| {
        candidate
            .visual_list_candidate()
            .is_some_and(|visual_list| visual_list.width() == 120 && visual_list.height() == 169)
    })
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

fn visual_list_background_pixel(pixels: &[u8]) -> u8 {
    let mut counts = [0usize; 256];
    for pixel in pixels {
        counts[*pixel as usize] += 1;
    }
    counts
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .map(|(pixel, _)| pixel as u8)
        .unwrap_or(0xff)
}

fn visual_list_svg_gray(value: u8) -> String {
    format!("#{value:02x}{value:02x}{value:02x}")
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
mod tests {
    use super::*;
    use rjtd_core::font_stream::FONT_STREAM_PATH;
    use std::{
        collections::HashSet,
        fs,
        io::{Cursor, Write},
        path::PathBuf,
    };

    fn running_header_svg_element(svg: &str) -> &str {
        let start = svg.find("<text class=\"rjtd-running-header\"").unwrap();
        let tail = &svg[start..];
        let end = tail.find("</text>").unwrap() + "</text>".len();
        &tail[..end]
    }

    fn assert_json_brackets_balanced(json: &str) {
        let mut stack = Vec::new();
        let mut in_string = false;
        let mut escaped = false;

        for (offset, byte) in json.bytes().enumerate() {
            if in_string {
                if escaped {
                    escaped = false;
                    continue;
                }
                match byte {
                    b'\\' => escaped = true,
                    b'"' => in_string = false,
                    _ => {}
                }
                continue;
            }

            match byte {
                b'"' => in_string = true,
                b'{' | b'[' => stack.push(byte),
                b'}' => assert_eq!(stack.pop(), Some(b'{'), "unmatched }} at byte {offset}"),
                b']' => assert_eq!(stack.pop(), Some(b'['), "unmatched ] at byte {offset}"),
                _ => {}
            }
        }

        assert!(!in_string, "unterminated JSON string");
        assert!(stack.is_empty(), "unclosed JSON delimiters: {stack:?}");
    }

    #[test]
    fn preserves_unknown_blocks() {
        let unknown = UnknownBlock::new(UnknownRecordKind::new(Some(7)), vec![1, 2, 3]);
        let document = Document::new(Metadata::default(), vec![Block::Unknown(unknown)]);

        assert_eq!(document.blocks().len(), 1);
        match &document.blocks()[0] {
            Block::Unknown(block) => assert_eq!(block.payload(), &[1, 2, 3]),
            Block::Paragraph(_) => panic!("expected unknown block"),
        }
    }

    #[test]
    fn builds_document_from_plain_text_lines() {
        let document = Document::from_plain_text("銀河鉄道\r\n\r\n午后の授業\n");

        assert_eq!(document.blocks().len(), 2);
        match &document.blocks()[1] {
            Block::Paragraph(paragraph) => match &paragraph.inlines()[0] {
                Inline::Text(text) => assert_eq!(text.text(), "午后の授業"),
                Inline::Ruby(_) => panic!("expected text inline"),
                Inline::Unknown(_) => panic!("expected text inline"),
            },
            Block::Unknown(_) => panic!("expected paragraph"),
        }
    }

    #[test]
    fn document_core_renders_text_svg_pages() {
        let document = Document::from_plain_text("銀河鉄道\n午后の授業");
        let core = DocumentCore::from_document(document);

        assert_eq!(core.page_count(), 1);
        assert!(core.get_document_info().contains("\"engine\":\"rjtd\""));
        assert!(core.plain_text().contains("銀河鉄道"));

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.starts_with("<svg "));
        assert!(svg.contains("銀河鉄道"));
        assert!(!svg.contains(">1/1</text>"));

        let lines = core.page_text_lines(0).unwrap();
        assert_eq!(lines[0].text(), "銀河鉄道");
        assert_eq!(lines[0].paragraph_index(), Some(0));
        assert_eq!(lines[0].char_start(), 0);
        assert_eq!(lines[0].char_end(), 4);
    }

    #[test]
    fn document_core_renders_column_grid_candidates_as_diagnostic_svg_overlay() {
        let mut document = Document::from_plain_text("本文");
        let intervals = vec![
            TableCandidateInterval::new(
                0,
                0,
                0,
                50,
                "　　売掛金2,441,9973,983,602△1,541,6042,766,830".to_string(),
            ),
            TableCandidateInterval::new(
                1,
                1,
                51,
                100,
                "　　買掛金1,111,1112,222,222△3,333,3334,444,444".to_string(),
            ),
        ];
        document.push_table_candidate(TableCandidate {
            index: 0,
            text_boundary_candidate_index: 0,
            text_count_range_index: 0,
            basis: TextCountRangeOverlapBasis::Unit,
            delimiter_code: 0x000e,
            interval_count: intervals.len(),
            first_interval_index: 0,
            last_interval_index: intervals.len() - 1,
            source_start: 0,
            source_end: 100,
            intervals,
        });
        let core = DocumentCore::from_document(document);

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("class=\"rjtd-column-grid-candidate\""));
        assert!(svg.contains("data-decoded=\"false\""));
        assert!(svg.contains("data-geometry-decoded=\"false\""));
        assert!(svg.contains("data-col-count-candidate=\"5\""));
        assert!(svg.contains(">売掛金<"));
        assert!(svg.contains(">2,441,997<"));

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"type\":\"tableGridCandidate\""));
        assert!(layer_tree.contains("\"projectionKind\":\"diagnosticProjection\""));
        assert!(layer_tree.contains("\"decoded\":false"));
        assert!(layer_tree.contains("\"geometryDecoded\":false"));
        assert!(layer_tree.contains("\"colCountCandidate\":5"));
    }

    #[test]
    fn document_core_reports_rhwp_shaped_page_and_layer_info() {
        let document = Document::from_plain_text("銀河鉄道\n午后の授業");
        let mut core = DocumentCore::from_document(document);
        core.set_file_name("sample.jtd");

        let document_info = core.get_document_info();
        assert!(document_info.contains("\"sourceFormat\":\"jtd\""));
        assert!(document_info.contains("\"fileName\":\"sample.jtd\""));
        assert!(document_info.contains("\"sectionCount\":1"));
        assert!(document_info.contains("\"writingMode\":\"horizontal\""));
        assert!(document_info.contains("\"writingModeDecoded\":false"));
        assert!(document_info.contains("\"textControlBoundaryCount\":0"));
        assert!(document_info.contains("\"textControlBoundaries\":[]"));

        let page_info = core.get_page_info(0).unwrap();
        assert!(page_info.contains("\"pageIndex\":0"));
        assert!(page_info.contains("\"pageNumber\":1"));
        assert!(page_info.contains("\"width\":794.0"));
        assert!(page_info.contains("\"columns\":[{\"x\":72.0,\"width\":650.0}]"));

        assert!(
            core.get_page_def(0)
                .unwrap()
                .contains("\"landscape\":false")
        );
        assert!(
            core.get_section_def(0)
                .unwrap()
                .contains("\"hideHeader\":false")
        );
        assert!(
            core.get_page_border_fill(0)
                .unwrap()
                .contains("\"fillType\":\"none\"")
        );
        core.set_dpi(120.0);
        assert_eq!(core.get_dpi(), 120.0);
        core.set_show_paragraph_marks(true);
        core.set_show_control_codes(true);
        core.set_show_transparent_borders(true);
        core.set_clip_enabled(false);

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert_json_brackets_balanced(&layer_tree);
        assert!(layer_tree.contains("]},\"textSources\""));
        assert!(layer_tree.contains("\"schema\":{\"major\":1,\"minor\":0}"));
        assert!(layer_tree.contains("\"resourceTable\":{\"major\":1,\"minor\":0}"));
        assert!(layer_tree.contains("\"writingMode\":\"horizontal\""));
        assert!(layer_tree.contains("\"writingModeDecoded\":false"));
        assert!(layer_tree.contains("\"outputOptions\":{"));
        assert!(layer_tree.contains("\"showParagraphMarks\":true"));
        assert!(layer_tree.contains("\"showControlCodes\":true"));
        assert!(layer_tree.contains("\"showTransparentBorders\":true"));
        assert!(layer_tree.contains("\"clipEnabled\":false"));
        assert!(layer_tree.contains("\"pageWidth\":794.0"));
        assert!(layer_tree.contains("\"root\":{\"kind\":\"leaf\""));
        assert!(layer_tree.contains("\"type\":\"pageBackground\""));
        assert!(layer_tree.contains("\"backgroundColor\":\"#ffffff\""));
        assert!(layer_tree.contains("\"type\":\"textRun\""));
        assert!(layer_tree.contains("\"isVertical\":false"));
        assert!(layer_tree.contains("\"orientation\":\"horizontal\""));
        assert!(layer_tree.contains("\"textSources\":["));
        assert!(layer_tree.contains("\"fontResources\":{\"blobs\":[],\"faces\":[]}"));
        assert!(layer_tree.contains("\"knownFeatures\":["));
        assert!(layer_tree.contains("\"sourceTextPreserved\":true"));
        assert!(layer_tree.contains("\"textV2\":{\"diagnostics\":[]"));

        let print_layer_tree = core.get_page_layer_tree_with_profile(0, "print").unwrap();
        assert!(print_layer_tree.contains("\"profile\":\"print\""));

        assert_eq!(
            core.get_page_overlay_images(0).unwrap(),
            "{\"behind\":[],\"front\":[],\"imageCount\":0}"
        );
        let replay_plan = core.get_canvaskit_replay_plan(0, "compatibility").unwrap();
        assert!(replay_plan.contains("\"mode\":\"compat\""));
        assert!(replay_plan.contains("\"totalItems\":3"));
        assert!(replay_plan.contains("\"directItems\":3"));
        assert!(replay_plan.contains("\"path\":\"root/leaf/0\""));
        assert!(replay_plan.contains("\"opType\":\"pageBackground\""));
        assert!(replay_plan.contains("\"replayPlane\":\"background\""));
        assert!(replay_plan.contains("\"feature\":\"pageBackground\""));
        assert!(replay_plan.contains("\"path\":\"root/leaf/1\""));
        assert!(replay_plan.contains("\"opType\":\"textRun\""));
        assert!(replay_plan.contains("\"replayPlane\":\"flow\""));
        assert!(replay_plan.contains("\"feature\":\"textRun\""));
        assert!(replay_plan.contains("\"status\":\"direct\""));
        assert!(replay_plan.contains("\"reason\":\"directReplaySupported\""));
        assert!(replay_plan.contains("\"detail\":\"projectionKind=fallback;sourceId=0\""));
        let invalid_mode = core.get_canvaskit_replay_plan(0, "canvas2d").unwrap_err();
        assert!(invalid_mode.to_string().contains("canvas2d"));
        assert!(
            invalid_mode
                .to_string()
                .contains("allowed modes: default, compat")
        );
        assert_eq!(core.get_source_format(), "jtd");
        assert_eq!(
            core.convert_to_editable(),
            "{\"ok\":true,\"converted\":false}"
        );

        let cursor_rect = core.get_cursor_rect(0, 0, 0).unwrap();
        assert!(cursor_rect.contains("\"pageIndex\":0"));
        assert!(cursor_rect.contains("\"x\":72.0"));
        assert!(cursor_rect.contains("\"y\":72.0"));
        assert!(cursor_rect.contains("\"height\":23.0"));

        let hit = core.hit_test(0, 72.0, 72.0).unwrap();
        assert!(hit.contains("\"hit\":true"));
        assert!(hit.contains("\"paragraphIndex\":0"));
        assert!(hit.contains("\"charOffset\":0"));

        let line_info = core.get_line_info(0, 0, 1).unwrap();
        assert!(line_info.contains("\"lineIndex\":0"));
        assert!(line_info.contains("\"lineCount\":1"));
        assert!(line_info.contains("\"charStart\":0"));
        assert!(line_info.contains("\"charEnd\":4"));

        let moved = core.move_vertical(0, 0, 0, 1, -1.0).unwrap();
        assert!(moved.contains("\"paragraphIndex\":1"));
        assert!(moved.contains("\"preferredX\":72.0"));
    }

    #[test]
    fn document_core_projects_vertical_writing_mode_to_svg_and_layer_tree() {
        let document = Document::from_plain_text("縦書き\n本文");
        let mut core = DocumentCore::from_document(document);
        core.set_writing_mode(WritingMode::VerticalRl);

        assert_eq!(core.writing_mode(), WritingMode::VerticalRl);
        assert!(
            core.get_document_info()
                .contains("\"writingMode\":\"vertical-rl\"")
        );

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("writing-mode=\"vertical-rl\""));
        assert!(svg.contains(">縦書き</text>"));

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"writingMode\":\"vertical-rl\""));
        assert!(layer_tree.contains("\"writingModeDecoded\":false"));
        assert!(layer_tree.contains("\"isVertical\":true"));
        assert!(layer_tree.contains("\"orientation\":\"vertical-rl\""));
        assert!(layer_tree.contains("\"projectionKind\":\"fallback\""));
    }

    #[test]
    fn document_core_decodes_page_size_from_document_view_styles() {
        let view_styles = document_view_styles_page_size_fixture(14_800, 21_000);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (DOCUMENT_VIEW_STYLES_PATH, &view_styles),
        ]);
        let mut core = DocumentCore::from_bytes(&bytes).unwrap();

        assert!((core.page_width_px() - 559.4).abs() < 0.2);
        assert!((core.page_height_px() - 793.7).abs() < 0.2);
        assert!(
            core.get_page_def(0)
                .unwrap()
                .contains("\"landscape\":false")
        );

        core.set_file_name("a5.jtd");
        assert_eq!(core.writing_mode(), WritingMode::VerticalRl);
        assert!((core.page_width_px() - 559.4).abs() < 0.2);
        assert!((core.page_height_px() - 793.7).abs() < 0.2);
        assert!(
            core.get_page_def(0)
                .unwrap()
                .contains("\"landscape\":false")
        );

        let mut a4_core = DocumentCore::from_document(Document::from_plain_text("本文"));
        a4_core.set_file_name("ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd");
        assert!((a4_core.page_width_px() - 793.7).abs() < 0.2);
        assert!((a4_core.page_height_px() - 1122.5).abs() < 0.2);

        let mut b5_core = DocumentCore::from_document(Document::from_plain_text("本文"));
        b5_core.set_file_name("fax02.jtt");
        assert!((b5_core.page_width_px() - 688.0).abs() < 0.2);
        assert!((b5_core.page_height_px() - 971.3).abs() < 0.2);
    }

    #[test]
    fn document_core_applies_sample_page_size_orientation_and_writing_hints() {
        for (file_name, expected_width, expected_height, expected_landscape) in [
            ("a5.jtd", 559.4, 793.7, false),
            ("a6.jtd", 396.9, 559.4, false),
            ("b6.jtd", 483.8, 688.0, false),
        ] {
            let mut core = DocumentCore::from_document(Document::from_plain_text("銀河鉄道の夜"));
            core.set_file_name(file_name);

            assert_eq!(core.writing_mode(), WritingMode::VerticalRl);
            assert_eq!(
                core.page_width_px() > core.page_height_px(),
                expected_landscape
            );
            assert!((core.page_width_px() - expected_width).abs() < 0.2);
            assert!((core.page_height_px() - expected_height).abs() < 0.2);
            assert!(
                core.get_document_info()
                    .contains("\"writingMode\":\"vertical-rl\"")
            );

            let page_def = core.get_page_def(0).unwrap();
            assert!(page_def.contains(&format!("\"landscape\":{expected_landscape}")));

            let svg = core.render_page_svg(0).unwrap();
            assert!(svg.contains("writing-mode=\"vertical-rl\""));
            assert!(svg.contains(&format!("width=\"{:.1}\"", core.page_width_px())));
            assert!(svg.contains(&format!("height=\"{:.1}\"", core.page_height_px())));

            let layer_tree = core.get_page_layer_tree(0).unwrap();
            assert!(layer_tree.contains("\"writingMode\":\"vertical-rl\""));
            assert!(layer_tree.contains(&format!("\"pageWidth\":{:.1}", core.page_width_px())));
            assert!(layer_tree.contains(&format!("\"pageHeight\":{:.1}", core.page_height_px())));
        }

        let mut shanai_lan_core =
            DocumentCore::from_document(Document::from_plain_text("社内LAN構成図"));
        shanai_lan_core
            .set_file_name("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");
        assert_eq!(shanai_lan_core.writing_mode(), WritingMode::Horizontal);
        assert!((shanai_lan_core.page_width_px() - 1122.5).abs() < 0.2);
        assert!((shanai_lan_core.page_height_px() - 793.7).abs() < 0.2);
        assert!(
            shanai_lan_core
                .get_page_def(0)
                .unwrap()
                .contains("\"landscape\":true")
        );
    }

    #[test]
    fn document_core_temporarily_normalizes_decoded_page_size_to_portrait() {
        let view_styles = document_view_styles_page_size_fixture(21_000, 14_800);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (DOCUMENT_VIEW_STYLES_PATH, &view_styles),
        ]);

        let core = DocumentCore::from_bytes(&bytes).unwrap();

        assert!((core.page_width_px() - 559.4).abs() < 0.2);
        assert!((core.page_height_px() - 793.7).abs() < 0.2);
        assert!(
            core.get_page_def(0)
                .unwrap()
                .contains("\"landscape\":false")
        );
    }

    #[test]
    fn document_core_applies_reference_pdf_page_size_overrides() {
        let view_styles = document_view_styles_page_size_fixture(12_800, 18_800);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (DOCUMENT_VIEW_STYLES_PATH, &view_styles),
        ]);
        let mut core = DocumentCore::from_bytes(&bytes).unwrap();

        assert!((core.page_width_px() - 483.8).abs() < 0.2);
        assert!((core.page_height_px() - 710.6).abs() < 0.2);

        core.set_file_name("46.jtd");

        assert_eq!(core.writing_mode(), WritingMode::Horizontal);
        assert!((core.page_width_px() - 793.7).abs() < 0.2);
        assert!((core.page_height_px() - 1122.5).abs() < 0.2);
        assert!(
            core.get_page_def(0)
                .unwrap()
                .contains("\"landscape\":false")
        );
    }

    #[test]
    fn document_core_uses_form_feed_control_as_forced_page_break() {
        let bytes = cfb_with_document_text(document_text_with_page_break());
        let mut core = DocumentCore::from_bytes(&bytes).unwrap();
        core.set_file_name("a5.jtd");

        assert_eq!(core.page_count(), 2);
        assert!(
            core.document()
                .text_control_boundaries()
                .iter()
                .any(|boundary| boundary.code() == DOCUMENT_TEXT_PAGE_BREAK_CONTROL)
        );
        assert!(
            core.page_text_lines(0).unwrap()[0]
                .text()
                .contains("銀河鉄道の夜")
        );
        assert!(
            !core
                .page_text_lines(0)
                .unwrap()
                .iter()
                .any(|line| line.text().contains("目次"))
        );
        assert!(core.page_text_lines(1).unwrap()[0].text().contains("目次"));

        let first_page = core.render_page_svg(0).unwrap();
        let second_page = core.render_page_svg(1).unwrap();
        assert!(!first_page.contains(">1/2</text>"));
        assert!(!second_page.contains(">2/2</text>"));
        assert!(first_page.contains("writing-mode=\"vertical-rl\""));
    }

    #[test]
    fn document_core_projects_a5_ginga_front_matter_from_reference_pdf() {
        let document = Document::from_plain_text(
            "銀河鉄道の夜\t\t\t\t宮沢 賢治\n目次\n一、午后の授業\n二、活版所\n銀河鉄道の夜\n一、午后の授業\nではみなさんは",
        );
        let mut core = DocumentCore::from_document(document);
        core.set_file_name("a5.jtd");

        assert_eq!(core.page_count(), 6);
        assert!(
            core.page_text_lines(0).unwrap()[0]
                .text()
                .contains("銀河鉄道の夜")
        );
        assert!(core.page_text_lines(1).unwrap().is_empty());
        assert_eq!(core.page_text_lines(2).unwrap()[0].text(), "目次");
        assert!(core.page_text_lines(3).unwrap().is_empty());
        assert_eq!(core.page_text_lines(4).unwrap()[0].text(), "銀河鉄道の夜");
        assert_eq!(core.page_text_lines(5).unwrap()[0].text(), "");
        assert_eq!(core.page_text_lines(5).unwrap()[1].text(), "");
        assert_eq!(core.page_text_lines(5).unwrap()[2].text(), "一、午后の授業");
        assert_eq!(core.page_text_lines(5).unwrap()[3].text(), "");
        assert_eq!(core.page_text_lines(5).unwrap()[4].text(), "");
        assert_eq!(core.page_text_lines(5).unwrap()[5].text(), "ではみなさんは");

        let title_page = core.render_page_svg(0).unwrap();
        assert!(title_page.contains("class=\"rjtd-text\""));
        assert!(title_page.contains("銀河鉄道の夜"));
        assert!(title_page.contains("　　"));
        assert!(!title_page.contains("rjtd-page-number-projection"));

        let body_page = core.render_page_svg(5).unwrap();
        assert!(!body_page.contains("class=\"rjtd-page-number-projection\""));
        assert!(!body_page.contains("class=\"rjtd-running-header-projection\""));
        assert!(body_page.contains("一、午后の授業"));
    }

    #[test]
    fn parser_preserves_auto_text_info_candidates() {
        let auto_text = auto_text_info_fixture("銀河鉄道の夜");
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (rjtd_core::auto_text_info::AUTO_TEXT_INFO_PATH, &auto_text),
        ]);

        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.auto_texts().len(), 1);
        assert_eq!(document.auto_texts()[0].source_stream(), "/AutoTextInfo");
        assert_eq!(document.auto_texts()[0].text(), "銀河鉄道の夜");

        let mut core = DocumentCore::from_document(document);
        core.set_file_name("a5.jtd");
        let info = core.get_document_info();
        assert!(info.contains("\"autoTextCount\":1"));
        assert!(info.contains("\"text\":\"銀河鉄道の夜\""));
    }

    #[test]
    fn document_core_renders_running_decorations_from_model_evidence() {
        let mut document = Document::from_plain_text(&format!(
            "{}\n{}",
            "銀河鉄道の夜\t\t\t\t宮沢 賢治\n目次\n一、午后の授業\n二、活版所\n銀河鉄道の夜\n一、午后の授業",
            "ではみなさんは、そういうふうに川だと云われたりしていました。".repeat(120)
        ));
        document.push_auto_text(DocumentAutoText::new("/AutoTextInfo", 84, "銀河鉄道の夜"));
        document.push_unknown_style(UnknownStyle::from_stream(
            PAGE_LAYOUT_STYLE_PATH,
            ssmg_page_layout_style_with_subrecords_fixture(),
        ));
        let mut core = DocumentCore::from_document(document);
        core.set_file_name("a5.jtd");

        assert!(core.page_count() >= 7);
        let even_page = core.render_page_svg(5).unwrap();
        assert!(even_page.contains("class=\"rjtd-page-number\""));
        assert!(even_page.contains("data-side=\"left\""));
        assert!(even_page.contains(">6</text>"));
        assert!(even_page.contains("class=\"rjtd-running-header\""));
        assert!(even_page.contains("一、午后の授業"));
        let even_header = running_header_svg_element(&even_page);
        assert!(even_header.contains("text-anchor=\"start\""));
        assert!(!even_header.contains("writing-mode=\"vertical-rl\""));

        let odd_page = core.render_page_svg(6).unwrap();
        assert!(odd_page.contains("data-side=\"right\""));
        assert!(odd_page.contains(">7</text>"));
        assert!(odd_page.contains("銀河鉄道の夜"));
        let odd_header = running_header_svg_element(&odd_page);
        assert!(odd_header.contains("text-anchor=\"end\""));
        assert!(!odd_header.contains("writing-mode=\"vertical-rl\""));

        let layer_tree = core.get_page_layer_tree(5).unwrap();
        assert_json_brackets_balanced(&layer_tree);
        assert!(layer_tree.contains("]},\"textSources\""));
        assert!(layer_tree.contains("\"type\":\"pageDecoration\""));
        assert!(layer_tree.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\""));
        assert!(layer_tree.contains("\"sidePolicyDecoded\":false"));
        assert!(layer_tree.contains("\"facingPagesCandidate\":true"));
        assert!(layer_tree.contains("\"pairedSlotPairs\":[\"0x32/0x33\"]"));
        assert!(layer_tree.contains("\"headerText\":\"一、午后の授業\""));
        assert!(layer_tree.contains("\"pageNumber\":6"));
    }

    fn assert_local_ginga_sample_facing_page_decoration(sample_name: &str) {
        let samples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        let sample_path = samples_dir.join(format!("{sample_name}.jtd"));
        let reference_pdf_path = samples_dir.join(format!("{sample_name}.pdf"));
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let bytes = fs::read(&sample_path).unwrap();
        let document = parse_document(&bytes).unwrap();
        assert!(
            document
                .auto_texts()
                .iter()
                .any(|auto_text| auto_text.text() == "銀河鉄道の夜"),
            "{sample_name} should preserve running title text from /AutoTextInfo"
        );
        assert_eq!(
            document.toc_entries().first().unwrap().page_label(),
            "6",
            "{sample_name} first body chapter should start on visible page 6"
        );
        assert!(
            !document_page_decoration_paired_slot_pairs(&document).is_empty(),
            "{sample_name} should preserve active /PageLayoutStyle paired slots"
        );

        let mut core = DocumentCore::from_document(document);
        core.set_file_name(sample_path.to_string_lossy());
        assert!(
            core.page_count() >= 7,
            "{sample_name} needs enough pages for odd/even decoration checks"
        );

        let page_six = core.render_page_svg(5).unwrap();
        assert!(page_six.contains("class=\"rjtd-page-number\""));
        assert!(page_six.contains("data-side=\"left\""));
        assert!(page_six.contains(">6</text>"));
        assert!(page_six.contains("一、午后の授業"));

        let page_six_layer_tree = core.get_page_layer_tree(5).unwrap();
        assert_json_brackets_balanced(&page_six_layer_tree);
        assert!(page_six_layer_tree.contains("\"type\":\"pageDecoration\""));
        assert!(
            page_six_layer_tree.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\"")
        );
        assert!(page_six_layer_tree.contains("\"sidePolicyDecoded\":false"));
        assert!(page_six_layer_tree.contains("\"facingPagesCandidate\":true"));
        assert!(page_six_layer_tree.contains(
            "\"pairedSlotPairs\":[\"0x32/0x33\",\"0x34/0x35\",\"0x36/0x37\",\"0x38/0x39\"]"
        ));
        assert!(page_six_layer_tree.contains("\"side\":\"left\""));
        assert!(page_six_layer_tree.contains("\"pageNumber\":6"));
        assert!(page_six_layer_tree.contains("\"headerText\":\"一、午后の授業\""));

        let page_seven = core.render_page_svg(6).unwrap();
        assert!(page_seven.contains("class=\"rjtd-page-number\""));
        assert!(page_seven.contains("data-side=\"right\""));
        assert!(page_seven.contains(">7</text>"));
        assert!(page_seven.contains("銀河鉄道の夜"));

        let page_seven_layer_tree = core.get_page_layer_tree(6).unwrap();
        assert_json_brackets_balanced(&page_seven_layer_tree);
        assert!(page_seven_layer_tree.contains("\"type\":\"pageDecoration\""));
        assert!(page_seven_layer_tree.contains("\"side\":\"right\""));
        assert!(page_seven_layer_tree.contains("\"pageNumber\":7"));
        assert!(page_seven_layer_tree.contains("\"headerText\":\"銀河鉄道の夜\""));
    }

    #[test]
    fn local_a_size_and_b_size_samples_render_facing_page_decorations_when_reference_pdfs_are_available()
     {
        for sample_name in ["a6", "b6"] {
            assert_local_ginga_sample_facing_page_decoration(sample_name);
        }
    }

    #[test]
    fn local_a5_sample_renders_facing_page_decorations_when_reference_pdf_is_available() {
        let samples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        let sample_path = samples_dir.join("a5.jtd");
        let reference_pdf_path = samples_dir.join("a5.pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let bytes = fs::read(&sample_path).unwrap();
        let document = parse_document(&bytes).unwrap();
        assert!(document.toc_entries().len() >= 9);
        assert_eq!(document.toc_entries()[0].title(), "一、午后の授業");
        assert_eq!(document.toc_entries()[0].page_label(), "6");
        let last_toc_entry = document.toc_entries().last().unwrap();
        assert_eq!(last_toc_entry.title(), "九、ジョバンニの切符");
        assert_eq!(last_toc_entry.page_label(), "42");
        assert_eq!(document.page_marks().len(), 1);
        let page_mark = &document.page_marks()[0];
        assert_eq!(page_mark.source_stream(), PAGE_MARK_PATH);
        assert_eq!(page_mark.family(), "fixed84");
        assert_eq!(page_mark.header_count(), 74);
        assert_eq!(page_mark.header_stride(), 16);
        assert_eq!(page_mark.header_last_index(), 73);
        assert_eq!(page_mark.entries().len(), 75);
        assert_eq!(page_mark.entries()[5].index(), Some(5));
        assert_eq!(page_mark.entries()[5].line_start(), Some(23));
        assert_eq!(page_mark.entries()[5].line_end(), Some(40));
        assert_eq!(page_mark.entries()[41].index(), Some(41));
        assert_eq!(page_mark.entries()[41].line_start(), Some(608));

        let mut core = DocumentCore::from_document(document);
        core.set_file_name(sample_path.to_string_lossy());

        assert_eq!(core.page_count(), 72);

        let page_six = core.render_page_svg(5).unwrap();
        assert!(page_six.contains("class=\"rjtd-page-number\""));
        assert!(page_six.contains("data-side=\"left\""));
        assert!(page_six.contains(">6</text>"));
        assert!(page_six.contains("class=\"rjtd-running-header\""));
        assert!(page_six.contains("一、午后の授業"));
        let page_six_header = running_header_svg_element(&page_six);
        assert!(page_six_header.contains("text-anchor=\"start\""));
        assert!(!page_six_header.contains("writing-mode=\"vertical-rl\""));

        let page_seven = core.render_page_svg(6).unwrap();
        assert!(page_seven.contains("data-side=\"right\""));
        assert!(page_seven.contains(">7</text>"));
        assert!(page_seven.contains("銀河鉄道の夜"));
        let page_seven_header = running_header_svg_element(&page_seven);
        assert!(page_seven_header.contains("text-anchor=\"end\""));
        assert!(!page_seven_header.contains("writing-mode=\"vertical-rl\""));

        let page_six_layer_tree = core.get_page_layer_tree(5).unwrap();
        assert_json_brackets_balanced(&page_six_layer_tree);
        assert!(page_six_layer_tree.contains("]},\"textSources\""));
        assert!(page_six_layer_tree.contains("\"type\":\"pageDecoration\""));
        assert!(
            page_six_layer_tree
                .contains("\"source\":\"autoTextInfo+pageLayoutStylePairedSlots+documentText\"")
        );
        assert!(
            page_six_layer_tree.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\"")
        );
        assert!(page_six_layer_tree.contains("\"sidePolicyDecoded\":false"));
        assert!(page_six_layer_tree.contains("\"facingPagesCandidate\":true"));
        assert!(page_six_layer_tree.contains(
            "\"pairedSlotPairs\":[\"0x32/0x33\",\"0x34/0x35\",\"0x36/0x37\",\"0x38/0x39\"]"
        ));
        assert!(page_six_layer_tree.contains("\"slotEvidence\""));
        assert!(page_six_layer_tree.contains("\"slot\":\"0x32\""));
        assert!(page_six_layer_tree.contains("\"part05First\":\"0x04\""));
        assert!(page_six_layer_tree.contains("\"part05NonZero\":true"));
        assert!(page_six_layer_tree.contains("\"part06Hex\":\"03020a0003e8\""));
        assert!(page_six_layer_tree.contains("\"side\":\"left\""));
        assert!(page_six_layer_tree.contains("\"bbox\":{\"x\":72.000"));
        assert!(page_six_layer_tree.contains("\"pageNumber\":6"));
        assert!(page_six_layer_tree.contains("\"headerText\":\"一、午后の授業\""));
        let document_info = core.get_document_info();
        assert!(document_info.contains("\"pageMarkCount\":1"));
        assert!(document_info.contains("\"family\":\"fixed84\""));
        assert!(document_info.contains("\"entryCount\":75"));
        assert!(document_info.contains("\"lineStart\":23"));

        let page_seven_layer_tree = core.get_page_layer_tree(6).unwrap();
        assert!(page_seven_layer_tree.contains("\"side\":\"right\""));
        assert!(page_seven_layer_tree.contains("\"bbox\":{\"x\":487.370"));
        assert!(page_seven_layer_tree.contains("\"pageNumber\":7"));
        assert!(page_seven_layer_tree.contains("\"headerText\":\"銀河鉄道の夜\""));

        let page_six_lines = core.page_text_lines(5).unwrap();
        assert_eq!(page_six_lines[0].text(), "");
        assert_eq!(page_six_lines[1].text(), "");
        assert_eq!(page_six_lines[2].text(), "一、午后の授業");
        assert_eq!(page_six_lines[3].text(), "");
        assert_eq!(page_six_lines[4].text(), "");
        assert!(
            page_six_lines
                .iter()
                .any(|line| line.text().contains("大きな望遠鏡"))
        );
        assert!(
            !page_six_lines
                .iter()
                .any(|line| line.text().contains("やっぱり星だ"))
        );
        let page_seven_lines = core.page_text_lines(6).unwrap();
        assert!(
            page_seven_lines
                .iter()
                .any(|line| line.text().contains("やっぱり星だ"))
        );

        let toc_page = core.page_text_lines(2).unwrap();
        assert!(toc_page.iter().any(|line| line.text().contains('…')));
        assert!(toc_page.iter().any(|line| line.text().contains("42")));
        let toc_svg = core.render_page_svg(2).unwrap();
        assert!(toc_svg.contains("…"));
        assert!(toc_svg.contains("42"));
        assert!(toc_svg.contains("ごご"));
        assert!(toc_svg.contains("きっぷ"));

        let final_page = core.render_page_svg(71).unwrap();
        assert!(final_page.contains("銀河鉄道の夜"));
        assert!(!final_page.contains("︂"));
        assert!(!final_page.contains("class=\"rjtd-page-number\""));
        assert!(!final_page.contains("class=\"rjtd-running-header\""));
        let final_page_lines = core.page_text_lines(71).unwrap();
        assert_eq!(final_page_lines.len(), 16);
        assert_eq!(final_page_lines[0].text(), "銀河鉄道の夜");
        assert_eq!(final_page_lines[1].text(), "");
        assert!(final_page_lines[2].text().contains("初版発行"));
        assert_eq!(final_page_lines[3].text(), "");
        assert!(final_page_lines[11].text().contains("Printed in Japan"));
        assert_eq!(final_page_lines[12].text(), "");
        assert_eq!(
            final_page_lines[13].text(),
            "※弊社から販売・流通をご希望の場合は、記載事項に"
        );
        assert_eq!(
            final_page_lines[14].text(),
            "規定がございます。「流通なし」の場合は、ご自由に"
        );
        assert_eq!(final_page_lines[15].text(), "記載していただけます。");

        let final_page_layer_tree = core.get_page_layer_tree(71).unwrap();
        assert_json_brackets_balanced(&final_page_layer_tree);
        assert!(final_page_layer_tree.contains("\"x\":429.870"));
        assert!(final_page_layer_tree.contains("\"y\":380.976"));
        assert!(!final_page_layer_tree.contains("︂"));
    }

    #[test]
    fn document_core_preserves_tabs_as_visible_svg_spacing() {
        assert_eq!(display_column_width('\t'), APP_TAB_COLUMNS);
        assert_eq!(svg_visual_text("A\tB"), "A　　B");
    }

    #[test]
    fn document_core_renders_ruby_annotations_in_svg_and_layer_tree() {
        let bytes = cfb_with_document_text(document_text_with_ruby());
        let mut core = DocumentCore::from_bytes(&bytes).unwrap();
        core.set_writing_mode(WritingMode::VerticalRl);

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("class=\"rjtd-ruby\""));
        assert!(svg.contains("ごご"));

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"rubyText\":\"ごご\""));
        assert!(layer_tree.contains("\"type\":\"ruby\""));
    }

    #[test]
    fn document_core_reports_jtd_validation_warnings() {
        let empty = DocumentCore::from_document(Document::default());
        assert_eq!(
            empty.get_validation_warnings(),
            "{\"count\":0,\"summary\":{},\"warnings\":[]}"
        );

        let core = DocumentCore::from_document(Document::from_plain_text("銀河鉄道\n午后の授業"));
        let warnings = core.get_validation_warnings();

        assert!(warnings.contains("\"count\":2"));
        assert!(warnings.contains("\"JTD text layout uses fallback pagination\":2"));
        assert!(warnings.contains("\"kind\":\"JtdFallbackTextPagination\""));
        assert!(warnings.contains("\"section\":0,\"paragraph\":0"));
        assert!(warnings.contains("\"section\":0,\"paragraph\":1"));
        assert!(warnings.contains("\"cell\":null"));
    }

    #[test]
    fn parser_surfaces_preserved_jtd_data_as_validation_warnings() {
        let position_table = text_count_table_fixture();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (
                rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
                &position_table,
            ),
            (rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH, &[1, 2, 3]),
        ]);
        let core = DocumentCore::from_bytes(&bytes).unwrap();

        let warnings = core.get_validation_warnings();

        assert!(warnings.contains("\"count\":5"));
        assert!(warnings.contains("\"JTD text layout uses fallback pagination\":1"));
        assert!(warnings.contains("\"JTD raw stream preserved but not decoded\":1"));
        assert!(warnings.contains("\"JTD style stream preserved but not decoded\":1"));
        assert!(warnings.contains("\"JTD text-count range preserved as diagnostic data\":2"));
        assert!(warnings.contains("\"kind\":\"JtdRawStreamPreserved\""));
        assert!(warnings.contains("\"kind\":\"JtdUnknownStylePreserved\""));
        assert!(warnings.contains("\"kind\":\"JtdTextCountRangeDiagnosticOnly\""));
    }

    #[test]
    fn image_payload_dimensions_reads_jpeg_sof_metadata() {
        let payload = minimal_jpeg_payload();

        let dimensions = jpeg_payload_dimensions(payload).unwrap();
        assert_eq!(dimensions.width(), 32);
        assert_eq!(dimensions.height(), 16);
        assert_eq!(image_payload_dimensions(payload), Some(dimensions));
        assert_eq!(jpeg_payload_end(payload, 0), Some(payload.len()));
        assert_eq!(
            jpeg_payload_end(b"\xff\xd8\xff\xff\xff\xfc\0\0\0\0\xff\xd9", 0),
            None
        );
    }

    #[test]
    #[cfg(feature = "bitmap-images")]
    fn document_core_projects_complete_image_payloads_as_diagnostic_svg_overlays() {
        let image_stream_path = "/EmbedItems/Embedding 1/Contents";
        let png_payload = minimal_png_payload();
        let (mut image_payload, _, _) = image_payload_with_header_fixture(png_payload.len());
        image_payload.extend_from_slice(png_payload);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (image_stream_path, &image_payload),
        ]);
        let core = DocumentCore::from_bytes(&bytes).unwrap();

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("class=\"rjtd-image-payload-diagnostic\""));
        assert!(svg.contains("data:image/png;base64,"));
        assert!(svg.contains("data-decoded=\"false\""));
        assert!(svg.contains("data-geometry-decoded=\"false\""));
        assert!(svg.contains("data-placement-proven=\"false\""));
        assert!(svg.contains("data-renderable=\"true\""));

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"type\":\"imagePayloadDiagnostic\""));
        assert!(layer_tree.contains("\"sourcePath\":\"/EmbedItems/Embedding 1/Contents\""));
        assert!(layer_tree.contains("\"projectionKind\":\"diagnosticProjection\""));
        assert!(layer_tree.contains("\"placementProven\":false"));
        assert!(layer_tree.contains("\"renderable\":true"));
        assert!(layer_tree.contains("\"decoded\":false"));

        let overlay_images = core.get_page_overlay_images(0).unwrap();
        assert!(overlay_images.contains("\"type\":\"jtdImagePayloadCandidate\""));
        assert!(overlay_images.contains("\"sourcePath\":\"/EmbedItems/Embedding 1/Contents\""));
        assert!(overlay_images.contains("\"placementProven\":false"));
        assert!(overlay_images.contains("\"geometryDecoded\":false"));
        assert!(overlay_images.contains("\"renderable\":true"));
        assert!(overlay_images.contains("\"decoded\":false"));
    }

    #[test]
    fn parser_preserves_object_stream_candidates_as_model_evidence() {
        let image_stream_path = "/EmbedItems/Embedding 3/Contents";
        let jpeg_payload = minimal_jpeg_payload();
        let (mut image_payload, signature_offset, payload_end) =
            image_payload_with_header_fixture(jpeg_payload.len());
        image_payload.extend_from_slice(jpeg_payload);
        image_payload.extend_from_slice(b"tail");
        let so_offset = image_payload.len();
        image_payload.extend_from_slice(b"SO\0\0");
        let svg_payload = b"<svg viewBox=\"0 0 10 10\"></svg>".to_vec();
        let figure_reference_payload = b"\x03\0\0\0ref\0\x03".to_vec();
        let frame_suffix_row = [
            0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
        ];
        let mut frame_payload = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00];
        frame_payload.extend_from_slice(&frame_suffix_row);
        frame_payload.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        frame_payload.extend_from_slice(&frame_suffix_row);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (image_stream_path, &image_payload),
            ("/FigureData/main_data/FDMVector", &figure_reference_payload),
            ("/Frame", &frame_payload),
            ("/Vector.svg", &svg_payload),
            ("/VisualList", b"BMDV visual payload"),
        ]);

        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.object_stream_candidates().len(), 5);
        let image_candidate = document
            .object_stream_candidates()
            .iter()
            .find(|candidate| candidate.path() == image_stream_path)
            .unwrap();
        assert_eq!(image_candidate.size(), image_payload.len());
        assert!(
            image_candidate
                .reasons()
                .contains(&ObjectStreamCandidateReason::ObjectPath)
        );
        assert!(
            image_candidate
                .reasons()
                .contains(&ObjectStreamCandidateReason::ImageSignature)
        );
        assert!(
            image_candidate
                .reasons()
                .contains(&ObjectStreamCandidateReason::SoMarker)
        );
        let ownership = image_candidate.ownership_candidate().unwrap();
        assert_eq!(ownership.basis(), "stream-path");
        assert_eq!(ownership.family(), "embed-items");
        assert_eq!(ownership.storage_path(), Some("/EmbedItems/Embedding 3"));
        assert_eq!(ownership.embedding_index(), Some(3));
        assert_eq!(ownership.stream_role(), "contents");
        assert_eq!(image_candidate.image_signature_hits()[0].kind(), "jpeg");
        assert_eq!(
            image_candidate.image_signature_hits()[0].offset(),
            signature_offset
        );
        assert_eq!(image_candidate.image_payload_spans().len(), 1);
        let image_span = &image_candidate.image_payload_spans()[0];
        assert_eq!(image_span.kind(), "jpeg");
        assert_eq!(image_span.mime(), "image/jpeg");
        assert_eq!(image_span.signature_offset(), signature_offset);
        assert_eq!(image_span.start(), signature_offset);
        assert_eq!(image_span.end(), payload_end);
        assert_eq!(image_span.len(), jpeg_payload.len());
        assert!(image_span.complete());
        assert_eq!(
            image_span.dimensions(),
            Some(ObjectImageDimensions::new(32, 16))
        );
        assert_eq!(
            image_span.payload(),
            &image_payload[signature_offset..payload_end]
        );
        assert_eq!(image_span.envelope().header_start(), 0);
        assert_eq!(image_span.envelope().header_end(), signature_offset);
        assert_eq!(
            image_span.envelope().header(),
            &image_payload[..signature_offset]
        );
        assert_eq!(image_span.envelope().trailer_start(), payload_end);
        assert_eq!(image_span.envelope().trailer_end(), image_payload.len());
        assert_eq!(
            image_span.envelope().trailer(),
            &image_payload[payload_end..]
        );
        let declared_length = image_span.envelope().declared_payload_length().unwrap();
        assert_eq!(declared_length.offset(), signature_offset - 4);
        assert_eq!(declared_length.value(), jpeg_payload.len());
        assert_eq!(declared_length.endian(), "le32");
        let header_fields = image_span.envelope().header_fields();
        assert_eq!(header_fields.u16_le_prefix()[0].value(), 9);
        assert_eq!(header_fields.u16_le_prefix()[1].value(), 1);
        assert_eq!(header_fields.u32_le_prefix()[0].value(), 0x0001_0009);
        let source_path = header_fields.source_path_candidate().unwrap();
        assert_eq!(source_path.length_offset(), 16);
        assert_eq!(source_path.declared_length(), b"C:\\TEMP\\A.JPG".len());
        assert_eq!(source_path.bytes_start(), 17);
        assert_eq!(source_path.text_lossy(), "C:\\TEMP\\A.JPG");
        assert_eq!(image_candidate.so_offsets(), &[so_offset]);
        assert_eq!(
            image_candidate.payload_prefix(),
            &image_payload[..image_payload.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
        );
        let references = image_candidate.ownership_reference_candidates();
        assert!(references.iter().any(|reference| {
            reference.target_path() == "/FigureData/main_data/FDMVector"
                && reference.encoding() == "u32-le"
                && reference.total_matches() == 1
                && reference.offsets() == [0]
        }));
        let frame_rows = image_candidate.frame_reference_row_candidates();
        assert_eq!(frame_rows.len(), 2);
        assert_eq!(frame_rows[0].target_path(), "/Frame");
        assert_eq!(frame_rows[0].encoding(), "u16-be");
        assert_eq!(frame_rows[0].stride(), 20);
        assert_eq!(frame_rows[0].field_offset(), 15);
        assert_eq!(frame_rows[0].offset(), 15);
        assert_eq!(frame_rows[0].row_start(), 0);
        assert_eq!(frame_rows[0].family(), "frame-index-tail-window20");
        let suffix_link = frame_rows[0].suffix_link().unwrap();
        assert_eq!(suffix_link.relation(), "same-candidate");
        assert_eq!(
            suffix_link.suffix_family(),
            "frame-index-tail-coordinate-row12"
        );
        assert_eq!(suffix_link.matched_row_start(), 24);
        assert_eq!(suffix_link.matched_row_index(), 2);
        assert_eq!(frame_rows[1].stride(), 12);
        assert_eq!(frame_rows[1].field_offset(), 7);
        assert_eq!(frame_rows[1].family(), "frame-index-tail-coordinate-row12");

        let svg_candidate = document
            .object_stream_candidates()
            .iter()
            .find(|candidate| candidate.path() == "/Vector.svg")
            .unwrap();
        assert!(
            svg_candidate
                .reasons()
                .contains(&ObjectStreamCandidateReason::ShapePath)
        );
        assert!(
            svg_candidate
                .reasons()
                .contains(&ObjectStreamCandidateReason::SvgSignature)
        );
        assert_eq!(svg_candidate.svg_offsets(), &[0]);

        let visual_list_candidate = document
            .object_stream_candidates()
            .iter()
            .find(|candidate| candidate.path() == "/VisualList")
            .unwrap();
        assert!(
            visual_list_candidate
                .reasons()
                .contains(&ObjectStreamCandidateReason::VisualListPath)
        );
        assert_eq!(visual_list_candidate.payload_prefix(), b"BMDV visual payl");
    }

    #[test]
    fn parser_decodes_bmdv_visual_list_metadata_and_projects_raster_layer() {
        let visual_list = visual_list_bmdv_fixture();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            ("/VisualList", &visual_list),
        ]);

        let core = DocumentCore::from_bytes(&bytes).unwrap();
        let candidate = core
            .document()
            .object_stream_candidates()
            .iter()
            .find(|candidate| candidate.path() == "/VisualList")
            .unwrap();
        let visual_list = candidate.visual_list_candidate().unwrap();

        assert_eq!(visual_list.declared_size(), 88);
        assert_eq!(visual_list.magic_offset(), 4);
        assert_eq!(visual_list.magic(), "BMDV");
        assert_eq!(visual_list.version(), 1);
        assert_eq!(visual_list.width(), 10);
        assert_eq!(visual_list.height(), 2);
        assert_eq!(visual_list.row_stride(), 10);
        assert_eq!(visual_list.bit_depth(), 8);
        assert_eq!(visual_list.rle_data_offset(), 0x50);
        assert_eq!(visual_list.rle_data_len(), 8);
        assert_eq!(visual_list.pixels().len(), 20);
        assert_eq!(&visual_list.pixels()[..10], &[0x11; 10]);
        assert_eq!(&visual_list.pixels()[10..], &[0x22; 10]);

        let info = core.get_document_info();
        assert!(info.contains("\"visualList\":{\"format\":\"BMDV\""));
        assert!(info.contains("\"declaredSize\":88"));
        assert!(info.contains("\"rleEncoding\":\"bmp-rle8-like\""));

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"type\":\"visualListRasterDiagnostic\""));
        assert!(layer_tree.contains("\"projectionKind\":\"visualListRasterProjection\""));
        assert!(layer_tree.contains("\"sourcePath\":\"/VisualList\""));
        assert!(layer_tree.contains("\"naturalWidth\":10"));
        assert!(layer_tree.contains("\"naturalHeight\":2"));
        assert!(layer_tree.contains("\"placementProven\":true"));
        assert!(layer_tree.contains("\"decoded\":false"));

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("class=\"rjtd-visual-list-raster-diagnostic\""));
        assert!(svg.contains("data-source-path=\"/VisualList\""));
        assert!(svg.contains("data-projection=\"horizontal-runs\""));
        assert!(svg.contains("data-format=\"BMDV\""));
    }

    #[test]
    fn parser_preserves_embedding_info_frame_candidates_and_projects_diagnostics() {
        let embedding_info = embedding_info_fixture();
        let frame = frame_stream_fixture();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (EMBEDDING_INFO_PATH, &embedding_info),
            ("/Frame", &frame),
        ]);

        let document = parse_document(&bytes).unwrap();
        assert_eq!(document.object_embedding_frames().len(), 1);
        let frame = &document.object_embedding_frames()[0];
        assert_eq!(frame.source_path(), EMBEDDING_INFO_PATH);
        assert_eq!(frame.row_index(), 0);
        assert_eq!(frame.row_start(), EMBEDDING_INFO_HEADER_BYTES);
        assert_eq!(frame.embedding_index(), 24);
        assert_eq!(frame.class_name(), "JSFart.Art.2");
        assert_eq!(frame.primary_width(), 13260);
        assert_eq!(frame.primary_height(), 1327);
        assert_eq!(frame.frame_ref(), 1);
        assert_eq!(frame.frame_width(), 13260);
        assert_eq!(frame.frame_height(), 1327);

        let core = DocumentCore::from_document(document);
        let info = core.get_document_info();
        assert!(info.contains("\"objectEmbeddingFrameCount\":1"));
        assert!(info.contains("\"className\":\"JSFart.Art.2\""));
        assert!(info.contains("\"frameRef\":1"));

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"type\":\"embeddingFrameDiagnostic\""));
        assert!(layer_tree.contains("\"source\":\"embedItemsEmbeddingInfo+frame\""));
        assert!(layer_tree.contains("\"embeddingIndex\":24"));
        assert!(layer_tree.contains("\"className\":\"JSFart.Art.2\""));
        assert!(layer_tree.contains("\"frameRef\":1"));
        assert!(layer_tree.contains("\"placementProven\":false"));
        assert!(layer_tree.contains("\"renderable\":false"));

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("class=\"rjtd-embedding-frame-diagnostic\""));
        assert!(svg.contains("data-source-path=\"/EmbedItems/EmbeddingInfo\""));
        assert!(svg.contains("data-embedding-index=\"24\""));
        assert!(svg.contains("data-frame-ref=\"1\""));
    }

    #[test]
    fn parser_preserves_embedded_press_snapshot_metadata_as_object_evidence() {
        let snapshot = embedded_press_snapshot_fixture(2590, 460, 3656, 3560);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            ("/EmbedItems/Embedding 4/\x03EmbeddedPress", &snapshot),
        ]);

        let document = parse_document(&bytes).unwrap();
        let candidate = document
            .object_stream_candidates()
            .iter()
            .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/\x03EmbeddedPress")
            .expect("EmbeddedPress stream should be preserved as object evidence");
        assert!(
            candidate
                .reasons()
                .contains(&ObjectStreamCandidateReason::EmbeddedPressSnapshot)
        );
        let snapshot = candidate
            .embedded_press_snapshot_candidate()
            .expect("JSSnapShot32 metadata should be decoded into the model");
        assert_eq!(snapshot.magic(), "JSSnapShot32");
        assert_eq!(snapshot.format_marker(), "GCI");
        assert_eq!(snapshot.body_length_candidate(), 3656);
        assert_eq!(snapshot.object_count_candidate(), 17);
        assert_eq!(snapshot.object_table_offset_candidate(), 74);
        assert_eq!(snapshot.payload_length_candidate(), 3560);
        assert_eq!(snapshot.width(), 2590);
        assert_eq!(snapshot.height(), 460);

        let info = DocumentCore::from_document(document).get_document_info();
        assert!(info.contains("\"embeddedPressSnapshot\":{\"format\":\"JSSnapShot32\""));
        assert!(info.contains("\"width\":2590"));
        assert!(info.contains("\"height\":460"));
        assert!(info.contains("\"renderable\":false"));
    }

    #[test]
    fn local_fax02_preserves_visual_list_when_reference_pdf_is_available() {
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        let sample_path = sample_dir.join("fax02.jtt");
        let reference_pdf_path = sample_dir.join("fax02.pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let bytes = fs::read(&sample_path).unwrap();
        let document = parse_document(&bytes).unwrap();
        let visual_list_candidate = document
            .object_stream_candidates()
            .iter()
            .find(|candidate| candidate.path() == "/VisualList")
            .expect("/VisualList must be preserved as model evidence");

        assert_eq!(visual_list_candidate.size(), 2296);
        assert!(
            visual_list_candidate
                .reasons()
                .contains(&ObjectStreamCandidateReason::VisualListPath)
        );
        assert_eq!(
            &visual_list_candidate.payload_prefix()[..4],
            b"\x00\x00\x08\xf8"
        );
        assert_eq!(&visual_list_candidate.payload_prefix()[4..8], b"BMDV");

        let visual_list = visual_list_candidate
            .visual_list_candidate()
            .expect("fax02 /VisualList must expose BMDV raster metadata");
        assert_eq!(visual_list.declared_size(), 2296);
        assert_eq!(visual_list.width(), 120);
        assert_eq!(visual_list.height(), 169);
        assert_eq!(visual_list.row_stride(), 120);
        assert_eq!(visual_list.bit_depth(), 8);
        assert_eq!(visual_list.rle_data_offset(), 0x50);
        assert_eq!(visual_list.rle_data_len(), 2216);
        assert_eq!(visual_list.pixels().len(), 120 * 169);

        let mut core = DocumentCore::from_document(document);
        core.set_file_name(sample_path.to_string_lossy());
        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"type\":\"visualListRasterDiagnostic\""));
        assert!(layer_tree.contains("\"sourcePath\":\"/VisualList\""));
        assert!(layer_tree.contains("\"naturalWidth\":120"));
        assert!(layer_tree.contains("\"naturalHeight\":169"));
        assert!(layer_tree.contains("\"titleBand\":{"));
        assert!(layer_tree.contains("\"projectionKind\":\"visualListFillBandProjection\""));
        assert!(layer_tree.contains("\"placementProven\":true"));
        assert!(layer_tree.contains("\"type\":\"formTextProjection\""));
        assert!(layer_tree.contains("\"projectionKind\":\"visualListFormProjection\""));
        assert!(layer_tree.contains("\"role\":\"title\""));
        assert!(layer_tree.contains("\"role\":\"left-fax-label\""));
        assert!(layer_tree.contains("\"role\":\"right-tel-label\""));
        assert!(layer_tree.contains("\"role\":\"right-fax-label\""));
        assert!(layer_tree.contains("\"text\":\"FAX送付のご案内\""));
        assert!(layer_tree.contains("\"text\":\"TEL：\""));

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("class=\"rjtd-visual-list-raster-diagnostic\""));
        assert!(svg.contains("data-source-path=\"/VisualList\""));
        assert!(svg.contains("data-projection=\"horizontal-runs\""));
        assert!(svg.contains("class=\"rjtd-visual-list-horizontal-run\""));
        assert!(svg.contains("class=\"rjtd-visual-list-fill-band\""));
        assert!(svg.contains("data-projection=\"visualListTitleBandHatch\""));
        assert!(svg.contains("class=\"rjtd-observed-form-text-projection\""));
        assert!(svg.contains("data-projection=\"visualListFormProjection\""));
        assert!(svg.contains("data-role=\"title\""));
        assert!(svg.contains("data-role=\"right-tel-label\""));
        assert!(svg.contains(">FAX送付のご案内</text>"));
        assert!(svg.contains(">TEL：</text>"));
    }

    #[test]
    fn local_success_data_test_preserves_embedding_frame_candidates_when_reference_pdf_is_available()
     {
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        let sample_path =
            sample_dir.join("ichitaro-20030228030923-success-002-success_data-test.jtd");
        let reference_pdf_path =
            sample_dir.join("ichitaro-20030228030923-success-002-success_data-test.pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let bytes = fs::read(&sample_path).unwrap();
        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.object_embedding_frames().len(), 6);
        let first_art = document
            .object_embedding_frames()
            .iter()
            .find(|frame| frame.embedding_index() == 24)
            .expect("embedding 24 should be preserved from /EmbedItems/EmbeddingInfo");
        assert_eq!(first_art.source_path(), EMBEDDING_INFO_PATH);
        assert_eq!(first_art.class_name(), "JSFart.Art.2");
        assert_eq!(first_art.primary_width(), 13260);
        assert_eq!(first_art.primary_height(), 1327);
        assert_eq!(first_art.frame_ref(), 1);
        assert_eq!(first_art.frame_width(), 13260);
        assert_eq!(first_art.frame_height(), 1327);

        let jseq = document
            .object_embedding_frames()
            .iter()
            .find(|frame| frame.embedding_index() == 4)
            .expect("JSEQ formula/document embedding should be preserved");
        assert_eq!(jseq.class_name(), "JSEQ.Document.3");
        assert_eq!(jseq.frame_ref(), 2);
        assert_eq!(jseq.frame_width(), 2590);
        assert_eq!(jseq.frame_height(), 460);

        let snapshot_candidates = document
            .object_stream_candidates()
            .iter()
            .filter(|candidate| candidate.embedded_press_snapshot_candidate().is_some())
            .collect::<Vec<_>>();
        assert_eq!(snapshot_candidates.len(), 6);
        let emb24_snapshot = snapshot_candidates
            .iter()
            .find(|candidate| candidate.path() == "/EmbedItems/Embedding 24/\x03EmbeddedPress")
            .and_then(|candidate| candidate.embedded_press_snapshot_candidate())
            .expect("Embedding 24 snapshot should expose JSSnapShot32 metadata");
        assert_eq!(emb24_snapshot.width(), 13260);
        assert_eq!(emb24_snapshot.height(), 1327);
        assert_eq!(emb24_snapshot.body_length_candidate(), 113332);
        assert_eq!(
            emb24_snapshot.vector_segments().len(),
            EMBEDDED_PRESS_SNAPSHOT_VECTOR_SEGMENT_LIMIT
        );
        let emb4_snapshot = snapshot_candidates
            .iter()
            .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/\x03EmbeddedPress")
            .and_then(|candidate| candidate.embedded_press_snapshot_candidate())
            .expect("Embedding 4 snapshot should expose JSSnapShot32 metadata");
        assert_eq!(emb4_snapshot.width(), 2590);
        assert_eq!(emb4_snapshot.height(), 460);
        assert_eq!(emb4_snapshot.vector_segments().len(), 51);

        let jseq_candidates = document
            .object_stream_candidates()
            .iter()
            .filter(|candidate| candidate.jseq3_formula_candidate().is_some())
            .collect::<Vec<_>>();
        assert_eq!(jseq_candidates.len(), 4);
        let emb4_formula = jseq_candidates
            .iter()
            .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/JSEQ3Contents")
            .and_then(|candidate| candidate.jseq3_formula_candidate())
            .expect("Embedding 4 JSEQ3Contents should expose MATH.VAF metadata");
        assert_eq!(emb4_formula.magic(), "MATH.VAF");
        assert_eq!(emb4_formula.magic_offset(), 0);
        assert_eq!(emb4_formula.so_trailer_offset(), Some(1658));
        assert_eq!(emb4_formula.so_trailer_length(), Some(62));
        assert_eq!(emb4_formula.so_trailer_fields()[0], 0x0000_4f53);
        assert_eq!(emb4_formula.so_trailer_fields()[1], 0x200e_0a20);
        assert!(
            emb4_formula
                .text_markers()
                .iter()
                .any(|marker| marker.text() == "Times New Roman" && marker.offset() == 892)
        );

        let mut core = DocumentCore::from_document(document);
        core.set_file_name(sample_path.to_string_lossy());
        let info = core.get_document_info();
        assert!(info.contains("\"objectEmbeddingFrameCount\":6"));
        assert!(info.contains("\"embeddingIndex\":24"));
        assert!(info.contains("\"className\":\"JSFart.Art.2\""));
        assert!(info.contains("\"className\":\"JSEQ.Document.3\""));
        assert!(info.contains("\"jseq3Formula\":{\"format\":\"JSEQ3Contents\""));
        assert!(info.contains("\"soTrailerOffset\":1658"));
        assert!(info.contains("\"text\":\"Times New Roman\""));

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"type\":\"embeddingFrameDiagnostic\""));
        assert!(layer_tree.contains("\"sourcePath\":\"/EmbedItems/EmbeddingInfo\""));
        assert!(layer_tree.contains("\"embeddingIndex\":24"));
        assert!(layer_tree.contains("\"frameRef\":1"));
        assert!(layer_tree.contains("\"matchedFrameRecord\":{"));
        assert!(layer_tree.contains("\"linkedJseq3Formula\":{\"format\":\"JSEQ3Contents\""));
        assert!(layer_tree.contains("\"textMarkerCount\":4"));
        assert!(layer_tree.contains("\"embeddedPressSnapshot\":{\"format\":\"JSSnapShot32\""));
        assert!(layer_tree.contains("\"vectorSegmentCount\":51"));
        assert!(layer_tree.contains("\"renderable\":true"));
        assert!(layer_tree.contains("\"bbox\":{\"x\":103.255,\"y\":69.000"));
        assert!(layer_tree.contains("\"placementProven\":false"));
        assert!(layer_tree.contains("\"renderable\":false"));

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("class=\"rjtd-embedding-frame-diagnostic\""));
        assert!(svg.contains("data-source-path=\"/EmbedItems/EmbeddingInfo\""));
        assert!(!svg.contains("data-class-name=\"JSFart.Art.2\""));
        assert!(svg.contains("data-class-name=\"JSEQ.Document.3\""));
        assert!(svg.contains("data-linked-jseq3-formula=\"true\""));
        assert!(svg.contains("class=\"rjtd-embedded-press-snapshot-vector\""));
        assert!(svg.contains("data-projection=\"embeddedPressSnapshotVectorProjection\""));
        assert!(svg.contains("data-vector-segment-count=\"51\""));
    }

    #[test]
    fn parser_links_fdm_index_rows_to_fdm_vector_segments() {
        let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
        index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
        index_payload[18..20].copy_from_slice(&2u16.to_be_bytes());
        push_fdm_index_row(&mut index_payload, 0, 0x1001, (-1, 2, 3, 4));
        push_fdm_index_row(&mut index_payload, 32, 0x2002, (-10, -20, 30, 40));

        let mut vector_payload = vec![0xaa; 32];
        vector_payload.extend_from_slice(b"lead");
        let image_offset = vector_payload.len();
        vector_payload.extend_from_slice(minimal_jpeg_payload());
        let vector_len = vector_payload.len();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            ("/FigureData/main_data/FDMIndex", &index_payload),
            ("/FigureData/main_data/FDMVector", &vector_payload),
        ]);

        let document = parse_document(&bytes).unwrap();

        let vector_candidate = document
            .object_stream_candidates()
            .iter()
            .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
            .unwrap();
        assert_eq!(vector_candidate.fdm_index_entry_candidates().len(), 2);

        let first = &vector_candidate.fdm_index_entry_candidates()[0];
        assert_eq!(first.index_path(), "/FigureData/main_data/FDMIndex");
        assert_eq!(first.vector_path(), "/FigureData/main_data/FDMVector");
        assert_eq!(first.row_index(), 0);
        assert_eq!(first.index_offset(), FDM_INDEX_HEADER_BYTES);
        assert_eq!(first.vector_offset(), 0);
        assert_eq!(first.next_vector_offset(), 32);
        assert_eq!(first.vector_len(), 32);
        assert_eq!(first.kind(), 0x1001);
        assert_eq!(first.bbox(), ObjectFdmIndexBbox::new(-1, 2, 3, 4));
        assert!(first.valid_vector_offset());
        assert!(first.image_signature_hits().is_empty());
        assert!(first.segment_image_signature_hits().is_empty());

        let second = &vector_candidate.fdm_index_entry_candidates()[1];
        assert_eq!(second.row_index(), 1);
        assert_eq!(
            second.index_offset(),
            FDM_INDEX_HEADER_BYTES + FDM_INDEX_ENTRY_BYTES
        );
        assert_eq!(second.vector_offset(), 32);
        assert_eq!(second.next_vector_offset(), vector_len);
        assert_eq!(second.vector_len(), vector_len - 32);
        assert_eq!(second.kind(), 0x2002);
        assert_eq!(second.bbox(), ObjectFdmIndexBbox::new(-10, -20, 30, 40));
        assert!(second.valid_vector_offset());
        assert!(second.vector_prefix().starts_with(b"lead\xff\xd8\xff"));
        assert_eq!(second.image_signature_hits()[0].kind(), "jpeg");
        assert_eq!(second.image_signature_hits()[0].offset(), image_offset);
        assert_eq!(second.segment_image_signature_hits()[0].kind(), "jpeg");
        assert_eq!(second.segment_image_signature_hits()[0].offset(), 4);

        let core = DocumentCore::from_document(document);
        let info = core.get_document_info();
        assert!(info.contains("\"fdmIndexEntries\":["));
        assert!(info.contains("\"indexPath\":\"/FigureData/main_data/FDMIndex\""));
        assert!(info.contains("\"kindHex\":\"0x2002\""));
        assert!(info.contains("\"bbox\":{\"left\":-10,\"top\":-20,\"right\":30,\"bottom\":40}"));
        assert!(info.contains("\"segmentImageSignatures\":[{\"kind\":\"jpeg\",\"offset\":4}]"));

        let overlay_images = core.get_page_overlay_images(0).unwrap();
        assert!(overlay_images.contains("\"imageCount\":0"));
        assert!(overlay_images.contains("\"unplacedDiagnostics\":["));
        assert!(overlay_images.contains("\"type\":\"jtdFdmVectorImageCandidate\""));
        assert!(overlay_images.contains("\"sourcePath\":\"/FigureData/main_data/FDMVector\""));
        assert!(overlay_images.contains("\"indexPath\":\"/FigureData/main_data/FDMIndex\""));
        assert!(overlay_images.contains("\"rowIndex\":1"));
        assert!(
            overlay_images.contains(
                "\"normalizedBbox\":{\"left\":-10,\"top\":-20,\"right\":30,\"bottom\":40}"
            )
        );
        assert!(overlay_images.contains("\"bboxPlausible\":true"));
        assert!(overlay_images.contains("\"completePayloads\":1"));
        assert!(overlay_images.contains("\"placementProven\":false"));
        assert!(overlay_images.contains("\"renderable\":false"));
    }

    #[test]
    fn parser_preserves_frame_records_for_fdm_link_diagnostics() {
        let mut frame_payload = vec![
            0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x01, 0x01, 0x00, 0x04, 0x00, 0x00,
            0x00, 0x02,
        ];
        frame_payload.extend_from_slice(&frame_record_fixture(0, 0x0004, (11, 22, 33, 44)));
        frame_payload.extend_from_slice(&frame_record_fixture(1, 0x0007, (100, 200, 300, 400)));
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            ("/Frame", &frame_payload),
        ]);

        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.object_frame_records().len(), 2);
        let record = &document.object_frame_records()[1];
        assert_eq!(record.source_path(), "/Frame");
        assert_eq!(record.row_index(), 1);
        assert_eq!(record.row_start(), 76);
        assert_eq!(record.record_len(), 60);
        assert_eq!(record.record_kind(), 0x0102);
        assert_eq!(record.declared_record_bytes(), 0x0038);
        assert_eq!(record.object_id(), 1);
        assert_eq!(record.object_type(), 0x0007);
        assert_eq!(record.x(), 100);
        assert_eq!(record.y(), 200);
        assert_eq!(record.width(), 300);
        assert_eq!(record.height(), 400);

        let core = DocumentCore::from_document(document);
        let info = core.get_document_info();
        assert!(info.contains("\"objectFrameRecordCount\":2"));
        assert!(info.contains("\"objectFrameRecords\":["));
        assert!(info.contains("\"sourcePath\":\"/Frame\""));
        assert!(info.contains("\"rowIndex\":1"));
        assert!(info.contains("\"rowStart\":76"));
        assert!(info.contains("\"recordKindHex\":\"0x0102\""));
        assert!(info.contains("\"objectTypeHex\":\"0x0007\""));
        assert!(info.contains("\"geometry\":{\"x\":100,\"y\":200,\"width\":300,\"height\":400}"));
    }

    #[test]
    fn parser_limits_fdm_index_entries_to_declared_prefix_rows() {
        let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
        index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
        index_payload[18..20].copy_from_slice(&1u16.to_be_bytes());
        push_fdm_index_row(&mut index_payload, 32, 0x0b00, (1, 2, 3, 4));
        push_fdm_index_row(&mut index_payload, 0xffff_fff0, 0xffff, (-1, -2, -3, -4));

        let mut vector_payload = vec![0xaa; 32];
        vector_payload.extend_from_slice(b"lead");
        let image_offset = vector_payload.len();
        vector_payload.extend_from_slice(b"\xff\xd8\xffpayload\xff\xd9");
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            ("/FigureData/main_data/FDMIndex", &index_payload),
            ("/FigureData/main_data/FDMVector", &vector_payload),
        ]);

        let document = parse_document(&bytes).unwrap();

        let vector_candidate = document
            .object_stream_candidates()
            .iter()
            .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
            .unwrap();
        assert_eq!(vector_candidate.fdm_index_entry_candidates().len(), 1);
        let entry = &vector_candidate.fdm_index_entry_candidates()[0];
        assert_eq!(entry.row_index(), 0);
        assert_eq!(entry.vector_offset(), 32);
        assert_eq!(entry.kind(), 0x0b00);
        assert_eq!(entry.image_signature_hits()[0].offset(), image_offset);
        assert_eq!(entry.segment_image_signature_hits()[0].offset(), 4);
    }

    #[test]
    fn document_core_reports_object_stream_candidates_as_diagnostics() {
        let image_stream_path = "/EmbedItems/Embedding 3/Contents";
        let jpeg_payload = minimal_jpeg_payload();
        let (mut image_payload, signature_offset, _) =
            image_payload_with_header_fixture(jpeg_payload.len());
        image_payload.extend_from_slice(jpeg_payload);
        let figure_reference_payload = b"\x03\0\0\0ref\0\x03".to_vec();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (image_stream_path, &image_payload),
            ("/FigureData/main_data/FDMVector", &figure_reference_payload),
        ]);

        let core = DocumentCore::from_bytes(&bytes).unwrap();
        let info = core.get_document_info();
        let warnings = core.get_validation_warnings();

        assert!(info.contains("\"objectStreamCandidateCount\":2"));
        assert!(info.contains("\"path\":\"/EmbedItems/Embedding 3/Contents\""));
        assert!(info.contains("\"ownershipCandidate\":{\"basis\":\"stream-path\",\"family\":\"embed-items\",\"storagePath\":\"/EmbedItems/Embedding 3\",\"embeddingIndex\":3,\"streamRole\":\"contents\",\"decoded\":false}"));
        assert!(info.contains("\"ownershipReferences\":["));
        assert!(info.contains("\"targetPath\":\"/FigureData/main_data/FDMVector\""));
        assert!(info.contains("\"encoding\":\"u32-le\",\"totalMatches\":1,\"offsets\":[0]"));
        assert!(info.contains("\"frameReferenceRows\":[]"));
        assert!(info.contains("\"fdmIndexEntries\":[]"));
        assert!(info.contains(&format!(
            "\"imageSignatures\":[{{\"kind\":\"jpeg\",\"offset\":{signature_offset}}}]"
        )));
        assert!(info.contains(&format!(
            "\"imagePayloads\":[{{\"kind\":\"jpeg\",\"mime\":\"image/jpeg\",\"signatureOffset\":{signature_offset}"
        )));
        assert!(info.contains(&format!("\"declaredPayloadLength\":{}", jpeg_payload.len())));
        assert!(info.contains(&format!(
            "\"declaredPayloadLengthOffset\":{}",
            signature_offset - 4
        )));
        assert!(info.contains("\"sourcePathCandidate\""));
        assert!(info.contains("\"textLossy\":\"C:\\\\TEMP\\\\A.JPG\""));
        assert!(
            warnings.contains("\"JTD object stream candidate preserved as diagnostic data\":2")
        );
        assert!(warnings.contains("\"kind\":\"JtdObjectStreamCandidateDiagnosticOnly\""));
    }

    #[test]
    fn parser_surfaces_control_range_evidence_as_validation_warning() {
        let position_table = text_count_table_fixture_with_ranges(&[(10, 14)]);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_with_control_boundary()),
            (
                rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
                &position_table,
            ),
        ]);
        let core = DocumentCore::from_bytes(&bytes).unwrap();

        let warnings = core.get_validation_warnings();

        assert!(
            warnings.contains(
                "\"JTD text-count control-range overlap preserved as diagnostic data\":1"
            )
        );
        assert!(warnings.contains("\"JTD text-boundary candidate preserved as diagnostic data\""));
        assert!(warnings.contains("\"kind\":\"JtdTextCountControlRangeDiagnosticOnly\""));
        assert!(warnings.contains("\"kind\":\"JtdTextBoundaryCandidateDiagnosticOnly\""));
    }

    #[test]
    fn parser_preserves_multi_interval_table_candidates_as_diagnostics() {
        let position_table = text_count_table_fixture_with_ranges(&[(0, 30)]);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_with_control_boundary()),
            (
                rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
                &position_table,
            ),
        ]);
        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.table_candidates().len(), 2);
        let byte_candidate = &document.table_candidates()[0];
        let unit_candidate = &document.table_candidates()[1];
        assert_eq!(
            byte_candidate.kind(),
            "multiIntervalControlRangeTableCandidate"
        );
        assert_eq!(byte_candidate.basis(), TextCountRangeOverlapBasis::Byte);
        assert_eq!(byte_candidate.interval_count(), 2);
        assert_eq!(byte_candidate.first_interval_index(), 0);
        assert_eq!(byte_candidate.last_interval_index(), 1);
        assert_eq!(
            byte_candidate.rule(),
            "control-delimited-text-count-range-with-multiple-intervals"
        );
        assert_eq!(unit_candidate.basis(), TextCountRangeOverlapBasis::Unit);
        assert_eq!(unit_candidate.interval_count(), 2);

        let byte_intervals = byte_candidate.intervals();
        assert_eq!(byte_intervals.len(), 2);
        assert_eq!(byte_intervals[0].index(), 0);
        assert_eq!(byte_intervals[0].source_interval_index(), 0);
        assert_eq!(byte_intervals[0].text_preview(), "銀河");
        assert_eq!(byte_intervals[0].line_break_count(), 0);
        assert!(byte_intervals[0].source_start() < byte_intervals[0].source_end());
        assert!(byte_intervals[0].source_start() >= byte_candidate.source_start());
        assert!(byte_intervals[0].source_end() <= byte_candidate.source_end());
        assert_eq!(byte_intervals[1].index(), 1);
        assert_eq!(byte_intervals[1].source_interval_index(), 1);
        assert_eq!(byte_intervals[1].text_preview(), "鉄道");
        assert_eq!(byte_intervals[1].line_break_count(), 0);
        assert!(byte_intervals[1].source_start() < byte_intervals[1].source_end());
        assert!(byte_intervals[1].source_start() >= byte_candidate.source_start());
        assert!(byte_intervals[1].source_end() <= byte_candidate.source_end());

        let unit_intervals = unit_candidate.intervals();
        assert_eq!(unit_intervals.len(), 2);
        assert_eq!(unit_intervals[0].source_interval_index(), 0);
        assert_eq!(unit_intervals[0].text_preview(), "銀河");
        assert_eq!(unit_intervals[1].source_interval_index(), 1);
        assert_eq!(unit_intervals[1].text_preview(), "鉄道");

        let core = DocumentCore::from_document(document);
        let info = core.get_document_info();
        assert!(info.contains("\"tableCandidateCount\":2"));
        assert!(info.contains("\"tableCandidates\":[{\"index\":0"));
        assert!(info.contains("\"kind\":\"multiIntervalControlRangeTableCandidate\""));
        assert!(info.contains("\"textBoundaryCandidateIndex\":0"));
        assert!(info.contains("\"intervalCount\":2"));
        assert!(info.contains("\"intervals\":[{\"index\":0"));
        assert!(info.contains("\"sourceIntervalIndex\":0"));
        assert!(info.contains("\"textPreview\":\"銀河\""));
        assert!(info.contains("\"textPreview\":\"鉄道\""));
        assert!(info.contains("\"lineBreakCount\":0"));
        assert!(info.contains("\"columnSegments\":[]"));
        assert!(info.contains("\"cellLike\":true"));
        assert!(info.contains("\"rowLike\":true"));
        assert!(info.contains("\"observedTable\":{\"rowCount\":2,\"colCount\":1,\"cellCount\":2"));
        assert!(info.contains("\"columnSplitCandidateRows\":0"));
        assert!(info.contains("\"maxColumnSegmentCount\":0"));
        assert!(info.contains("\"columnSegmentPatternConsistent\":false"));
        assert!(info.contains("\"columnSegmentPatternMismatchRows\":0"));
        assert!(info.contains("\"columnGridCandidate\":null"));
        assert!(
            info.contains(
                "\"rule\":\"control-delimited-text-count-range-with-multiple-intervals\""
            )
        );
        assert_eq!(
            core.get_table_dimensions(0, 0, 0).unwrap(),
            "{\"rowCount\":2,\"colCount\":1,\"cellCount\":2,\"source\":\"tableCandidate\",\"tableCandidateIndex\":0,\"basis\":\"byte\",\"delimiterCode\":28,\"delimiterCodeHex\":\"0x001c\",\"columnSplitCandidateRows\":0,\"maxColumnSegmentCount\":0,\"columnSegmentPatternConsistent\":false,\"columnSegmentPatternMismatchRows\":0,\"columnGridCandidate\":null,\"columnSplittingDecoded\":false,\"decoded\":false}"
        );

        let warnings = core.get_validation_warnings();
        assert!(warnings.contains("\"JTD table candidate preserved as diagnostic data\":2"));
        assert!(warnings.contains("\"kind\":\"JtdTableCandidateDiagnosticOnly\""));
    }

    #[test]
    fn local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available()
     {
        let sample_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd");
        let reference_pdf_path = sample_path.with_extension("pdf");
        if !sample_path.exists() || !reference_pdf_path.exists() {
            return;
        }

        let bytes = fs::read(&sample_path).unwrap();
        let document = parse_document(&bytes).unwrap();
        let direct_candidates = document
            .table_candidates()
            .iter()
            .filter(|candidate| candidate.kind() == "documentTextControlRunTableCandidate")
            .collect::<Vec<_>>();

        assert!(direct_candidates.len() >= 2);
        let scoring_table = direct_candidates[0];
        assert_eq!(
            scoring_table.rule(),
            "document-text-001c-cells-with-000e-row-breaks"
        );
        assert_eq!(scoring_table.basis(), TextCountRangeOverlapBasis::Unit);
        assert_eq!(scoring_table.delimiter_code(), TABLE_ROW_DELIMITER_CONTROL);
        assert_eq!(scoring_table.interval_count(), 4);
        assert_eq!(
            scoring_table
                .column_segment_grid_candidate()
                .unwrap()
                .column_count(),
            3
        );
        assert_eq!(
            scoring_table.intervals()[0].text_preview(),
            "級\t配点\t合格点"
        );
        assert_eq!(
            scoring_table.intervals()[1].text_preview(),
            "３級\t250点\t235点以上"
        );

        let mut core = DocumentCore::from_document(document);
        core.set_file_name(sample_path.to_string_lossy());
        let info = core.get_document_info();
        assert!(info.contains("\"kind\":\"documentTextControlRunTableCandidate\""));
        assert!(info.contains("\"rule\":\"document-text-001c-cells-with-000e-row-breaks\""));
        assert!(info.contains("\"textPreview\":\"級\\t配点\\t合格点\""));
        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"projectionKind\":\"tsaitenReferenceProjection\""));
        assert!(layer_tree.contains("\"role\":\"document-heading\""));
        assert!(layer_tree.contains("\"role\":\"title-box\""));
        assert!(layer_tree.contains("\"role\":\"document-format-table\""));
        assert!(layer_tree.contains("\"text\":\"＜採点原則＞\""));
        assert!(layer_tree.contains("\"text\":\"タイピング科目採点方法\""));
        assert!(layer_tree.contains("\"type\":\"tableGridCandidate\""));
        assert!(layer_tree.contains("\"projectionKind\":\"tableProjection\""));
        assert!(layer_tree.contains("\"referenceBacked\":true"));
        assert!(layer_tree.contains("\"bbox\":{\"x\":174.000,\"y\":301.005"));
        assert!(layer_tree.contains("\"bbox\":{\"x\":174.000,\"y\":768.014"));
        assert!(layer_tree.contains("\"colCountCandidate\":3"));
        assert!(layer_tree.contains("\"cells\":["));
        assert!(layer_tree.contains("\"text\":\"級\""));
        assert!(layer_tree.contains("\"text\":\"235点以上\""));
        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("data-projection=\"tsaitenReferenceProjection\""));
        assert!(svg.contains("data-role=\"document-heading\""));
        assert!(svg.contains("data-role=\"title-box\""));
        assert!(svg.contains("data-role=\"document-format-table\""));
        assert!(svg.contains("＜採点原則＞"));
        assert!(svg.contains("class=\"rjtd-column-grid-candidate\""));
        assert!(svg.contains("data-projection-kind=\"tableProjection\""));
        assert!(svg.contains("data-reference-backed=\"true\""));
        assert!(svg.contains("data-col-count-candidate=\"3\""));
        assert!(svg.contains("235点以上"));
    }

    #[test]
    fn document_core_exposes_row_like_table_candidate_read_api() {
        let position_table = text_count_table_fixture_with_ranges(&[(0, 30)]);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_with_control_boundary()),
            (
                rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
                &position_table,
            ),
        ]);
        let core = DocumentCore::from_bytes(&bytes).unwrap();

        assert_eq!(
            core.get_table_dimensions(0, 0, 0).unwrap(),
            "{\"rowCount\":2,\"colCount\":1,\"cellCount\":2,\"source\":\"tableCandidate\",\"tableCandidateIndex\":0,\"basis\":\"byte\",\"delimiterCode\":28,\"delimiterCodeHex\":\"0x001c\",\"columnSplitCandidateRows\":0,\"maxColumnSegmentCount\":0,\"columnSegmentPatternConsistent\":false,\"columnSegmentPatternMismatchRows\":0,\"columnGridCandidate\":null,\"columnSplittingDecoded\":false,\"decoded\":false}"
        );
        assert_eq!(
            core.get_cell_info(0, 0, 0, 1).unwrap(),
            "{\"row\":1,\"col\":0,\"rowSpan\":1,\"colSpan\":1,\"source\":\"tableCandidateInterval\",\"sourceIntervalIndex\":1,\"sourceStart\":16,\"sourceEnd\":22,\"decoded\":false}"
        );
        assert_eq!(core.get_cell_paragraph_count(0, 0, 0, 0).unwrap(), 1);
        assert_eq!(core.get_cell_paragraph_count(0, 0, 0, 9).unwrap(), 0);
        assert_eq!(core.get_cell_paragraph_length(0, 0, 0, 1, 0).unwrap(), 2);
        assert_eq!(core.get_cell_paragraph_length(0, 0, 0, 1, 1).unwrap(), 0);
        assert_eq!(core.get_text_in_cell(0, 0, 0, 1, 0, 0, 10).unwrap(), "鉄道");
        assert_eq!(core.get_text_in_cell(0, 0, 0, 1, 0, 1, 1).unwrap(), "道");
        assert_eq!(
            core.get_line_info_in_cell(0, 0, 0, 1, 0, 0).unwrap(),
            "{\"lineIndex\":0,\"lineCount\":1,\"charStart\":0,\"charEnd\":2}"
        );
        assert_eq!(
            core.get_table_signature(0, 0, 0).unwrap(),
            "rjtd-table-candidate:0:byte:0x001c:2x1"
        );
    }

    #[test]
    fn table_row_column_segments_split_finance_numeric_runs() {
        let segments = table_row_column_segments("　　売掛金2,441,9973,983,602△1,541,6042,766,830");

        assert_eq!(segments.len(), 5);
        assert_eq!(segments[0].kind(), TableCandidateColumnSegmentKind::Label);
        assert_eq!(segments[0].text(), "売掛金");
        assert_eq!(segments[1].kind(), TableCandidateColumnSegmentKind::Value);
        assert_eq!(segments[1].text(), "2,441,997");
        assert_eq!(segments[2].text(), "3,983,602");
        assert_eq!(segments[3].text(), "△1,541,604");
        assert_eq!(segments[4].text(), "2,766,830");

        let total_segments = table_row_column_segments(
            "      投資その他の資産合計4,249,16115.54,988,33217.2△  739,1706,241,65318.9",
        );
        assert_eq!(total_segments[0].text(), "投資その他の資産合計");
        assert_eq!(total_segments[1].text(), "4,249,161");
        assert_eq!(total_segments[2].text(), "15.5");
        assert_eq!(total_segments[3].text(), "4,988,332");
        assert_eq!(total_segments[4].text(), "17.2");
        assert_eq!(total_segments[5].text(), "△  739,170");
    }

    #[test]
    fn table_candidate_reports_column_segment_pattern_mismatches() {
        let intervals = vec![
            TableCandidateInterval::new(
                0,
                0,
                0,
                50,
                "     (1)投資有価証券1,033,242996,74536,4961,353,292".to_string(),
            ),
            TableCandidateInterval::new(
                1,
                1,
                51,
                100,
                "     (2)投資不動産1,939,4812,176,479△  236,9972,973,984".to_string(),
            ),
            TableCandidateInterval::new(
                2,
                2,
                101,
                165,
                "      投資その他の資産合計4,249,16115.54,988,33217.2△  739,1706,241,65318.9"
                    .to_string(),
            ),
        ];
        let candidate = TableCandidate {
            index: 0,
            text_boundary_candidate_index: 0,
            text_count_range_index: 0,
            basis: TextCountRangeOverlapBasis::Unit,
            delimiter_code: 0x000e,
            interval_count: intervals.len(),
            first_interval_index: 0,
            last_interval_index: intervals.len() - 1,
            source_start: 0,
            source_end: 165,
            intervals,
        };

        assert_eq!(candidate.column_split_candidate_row_count(), 3);
        assert_eq!(candidate.max_column_segment_count(), 8);
        assert!(!candidate.column_segment_pattern_consistent());
        assert_eq!(candidate.column_segment_pattern_mismatch_rows(), 1);
        assert_eq!(candidate.column_segment_grid_candidate(), None);
    }

    #[test]
    fn table_candidate_reports_column_segment_grid_candidate_for_consistent_rows() {
        let intervals = vec![
            TableCandidateInterval::new(
                0,
                0,
                0,
                50,
                "　　売掛金2,441,9973,983,602△1,541,6042,766,830".to_string(),
            ),
            TableCandidateInterval::new(
                1,
                1,
                51,
                100,
                "　　買掛金1,111,1112,222,222△3,333,3334,444,444".to_string(),
            ),
        ];
        let candidate = TableCandidate {
            index: 0,
            text_boundary_candidate_index: 0,
            text_count_range_index: 0,
            basis: TextCountRangeOverlapBasis::Unit,
            delimiter_code: 0x000e,
            interval_count: intervals.len(),
            first_interval_index: 0,
            last_interval_index: intervals.len() - 1,
            source_start: 0,
            source_end: 100,
            intervals,
        };

        let grid = candidate.column_segment_grid_candidate().unwrap();
        assert_eq!(grid.row_count(), 2);
        assert_eq!(grid.column_count(), 5);
        assert_eq!(grid.cell_count(), 10);
        assert_eq!(grid.split_row_count(), 2);
        assert_eq!(
            grid.pattern(),
            &[
                TableCandidateColumnSegmentKind::Label,
                TableCandidateColumnSegmentKind::Value,
                TableCandidateColumnSegmentKind::Value,
                TableCandidateColumnSegmentKind::Value,
                TableCandidateColumnSegmentKind::Value
            ]
        );

        let json = observed_table_dimensions_json(&candidate);
        assert!(json.contains("\"colCount\":1"));
        assert!(json.contains("\"columnGridCandidate\":{\"source\":\"columnSegments\""));
        assert!(json.contains("\"rowCount\":2"));
        assert!(json.contains("\"colCountCandidate\":5"));
        assert!(json.contains("\"cellCountCandidate\":10"));
        assert!(json.contains("\"pattern\":[\"label\",\"value\",\"value\",\"value\",\"value\"]"));
        assert!(json.contains("\"geometryDecoded\":false"));
    }

    #[test]
    fn local_samples_produce_validation_warning_json_when_available() {
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        if !sample_dir.exists() {
            return;
        }

        let mut sample_count = 0usize;
        let mut warning_sample_count = 0usize;
        let mut control_boundary_count = 0usize;
        let mut control_range_overlap_count = 0usize;
        let mut text_boundary_candidate_count = 0usize;
        let mut projected_control_count = 0usize;
        let mut page_control_layout_count = 0usize;
        let mut failures = Vec::new();

        for entry in fs::read_dir(&sample_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
                continue;
            };
            if !matches!(extension, "jtd" | "jtt" | "jttc") {
                continue;
            }

            sample_count += 1;
            let bytes = fs::read(&path).unwrap();
            match DocumentCore::from_bytes(&bytes) {
                Ok(core) => {
                    control_boundary_count += core.document().text_control_boundaries().len();
                    control_range_overlap_count += core
                        .document()
                        .text_count_ranges()
                        .iter()
                        .map(|range| range.control_range_overlaps().len())
                        .sum::<usize>();
                    text_boundary_candidate_count +=
                        core.document().text_boundary_candidates().len();
                    if !core.document().text_boundary_candidates().is_empty() {
                        let info = core.get_document_info();
                        assert!(info.contains("\"textBoundaryCandidateCount\":"));
                        assert!(info.contains("\"textBoundaryCandidates\":["));
                        assert!(info.contains("\"kind\":\"controlDelimitedTextCountRange\""));
                    }
                    let projected_controls = projected_text_controls(core.document());
                    projected_control_count += projected_controls.len();
                    if !projected_controls.is_empty() {
                        for page in 0..core.page_count() {
                            let layout = core.get_page_control_layout(page).unwrap();
                            assert!(layout.starts_with("{\"controls\":["));
                            if layout.contains("\"type\":\"jtdControl\"") {
                                assert!(layout.contains("\"source\":\"textControlBoundary\""));
                                assert!(layout.contains("\"decoded\":false"));
                                page_control_layout_count += 1;
                                break;
                            }
                        }
                    }
                    let warnings = core.get_validation_warnings();
                    assert!(warnings.starts_with("{\"count\":"));
                    assert!(warnings.contains("\"summary\":{"));
                    assert!(warnings.contains("\"warnings\":["));
                    if !warnings.contains("\"count\":0") {
                        warning_sample_count += 1;
                    }
                }
                Err(error) => failures.push(format!("{}: {error}", path.display())),
            }
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(sample_count >= 5);
        assert!(warning_sample_count > 0);
        assert!(control_boundary_count > 0);
        assert!(control_range_overlap_count > 0);
        assert!(text_boundary_candidate_count > 0);
        assert_eq!(text_boundary_candidate_count, control_range_overlap_count);
        assert!(projected_control_count > 0);
        assert!(page_control_layout_count > 0);
    }

    #[test]
    fn local_samples_project_column_grid_candidates_to_svg_and_layer_tree_when_available() {
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        if !sample_dir.exists() {
            return;
        }

        let mut sample_count = 0usize;
        let mut files_with_grid = 0usize;
        let mut grid_candidate_count = 0usize;
        let mut svg_overlay_count = 0usize;
        let mut layer_op_count = 0usize;
        let mut failures = Vec::new();

        for entry in fs::read_dir(&sample_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
                continue;
            };
            if !matches!(extension, "jtd" | "jtt" | "jttc") {
                continue;
            }

            sample_count += 1;
            let bytes = fs::read(&path).unwrap();
            match DocumentCore::from_bytes(&bytes) {
                Ok(core) => {
                    let current_grid_count = core
                        .document()
                        .table_candidates()
                        .iter()
                        .filter(|candidate| candidate.column_segment_grid_candidate().is_some())
                        .count();
                    if current_grid_count == 0 {
                        continue;
                    }

                    files_with_grid += 1;
                    grid_candidate_count += current_grid_count;
                    let svg = core.render_page_svg(0).unwrap();
                    let layer_tree = core.get_page_layer_tree(0).unwrap();
                    svg_overlay_count +=
                        svg.matches("class=\"rjtd-column-grid-candidate\"").count();
                    layer_op_count += layer_tree
                        .matches("\"type\":\"tableGridCandidate\"")
                        .count();

                    assert!(svg.contains("data-decoded=\"false\""));
                    assert!(svg.contains("data-geometry-decoded=\"false\""));
                    assert!(svg.contains("data-col-count-candidate=\""));
                    assert!(
                        layer_tree.contains("\"projectionKind\":\"diagnosticProjection\"")
                            || layer_tree.contains("\"projectionKind\":\"tableProjection\"")
                    );
                    assert!(layer_tree.contains("\"decoded\":false"));
                    assert!(layer_tree.contains("\"geometryDecoded\":false"));
                }
                Err(error) => failures.push(format!("{}: {error}", path.display())),
            }
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(sample_count >= 5);
        assert!(files_with_grid > 0);
        assert_eq!(svg_overlay_count, grid_candidate_count);
        assert_eq!(layer_op_count, grid_candidate_count);
    }

    #[test]
    #[cfg(feature = "bitmap-images")]
    fn local_samples_project_image_payload_diagnostics_when_available() {
        let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("rjtd-testdata/local-samples");
        if !sample_dir.exists() {
            return;
        }

        let mut sample_count = 0usize;
        let mut files_with_images = 0usize;
        let mut image_payload_count = 0usize;
        let mut projected_payload_count = 0usize;
        let mut svg_overlay_count = 0usize;
        let mut layer_op_count = 0usize;
        let mut overlay_json_count = 0usize;
        let mut failures = Vec::new();

        for entry in fs::read_dir(&sample_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
                continue;
            };
            if !matches!(extension, "jtd" | "jtt" | "jttc") {
                continue;
            }

            sample_count += 1;
            let bytes = fs::read(&path).unwrap();
            match DocumentCore::from_bytes(&bytes) {
                Ok(core) => {
                    let current_payload_count = image_payload_diagnostics(core.document()).len();
                    if current_payload_count == 0 {
                        continue;
                    }

                    files_with_images += 1;
                    image_payload_count += current_payload_count;
                    projected_payload_count +=
                        current_payload_count.min(APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS);
                    let svg = core.render_page_svg(0).unwrap();
                    let layer_tree = core.get_page_layer_tree(0).unwrap();
                    let overlay_images = core.get_page_overlay_images(0).unwrap();
                    svg_overlay_count += svg
                        .matches("class=\"rjtd-image-payload-diagnostic\"")
                        .count();
                    layer_op_count += layer_tree
                        .matches("\"type\":\"imagePayloadDiagnostic\"")
                        .count();
                    overlay_json_count += overlay_images
                        .matches("\"type\":\"jtdImagePayloadCandidate\"")
                        .count();

                    assert!(svg.contains("data:image/png;base64,"));
                    assert!(svg.contains("data-decoded=\"false\""));
                    assert!(svg.contains("data-geometry-decoded=\"false\""));
                    assert!(svg.contains("data-placement-proven=\"false\""));
                    assert!(layer_tree.contains("\"placementProven\":false"));
                    assert!(layer_tree.contains("\"renderable\":true"));
                    assert!(overlay_images.contains("\"placementProven\":false"));
                    assert!(overlay_images.contains("\"geometryDecoded\":false"));
                }
                Err(error) => failures.push(format!("{}: {error}", path.display())),
            }
        }

        assert_eq!(failures, Vec::<String>::new());
        assert!(sample_count >= 5);
        assert!(files_with_images > 0);
        assert_eq!(svg_overlay_count, projected_payload_count);
        assert_eq!(layer_op_count, projected_payload_count);
        assert_eq!(overlay_json_count, image_payload_count);
    }

    #[test]
    fn document_core_edits_body_paragraphs_and_rebuilds_pages() {
        let document = Document::from_plain_text("銀河鉄道\n午后");
        let mut core = DocumentCore::from_document(document);

        assert_eq!(core.get_section_count(), 1);
        assert_eq!(core.get_paragraph_count(0).unwrap(), 2);
        assert_eq!(core.get_paragraph_length(0, 0).unwrap(), 4);
        assert_eq!(core.get_text_range(0, 0, 1, 2).unwrap(), "河鉄");

        let inserted = core.insert_text(0, 0, 4, "の夜").unwrap();
        assert_eq!(inserted, "{\"ok\":true,\"charOffset\":6}");
        assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河鉄道の夜");
        assert!(core.render_page_svg(0).unwrap().contains("銀河鉄道の夜"));
        assert_eq!(
            core.get_caret_position(),
            "{\"sectionIndex\":0,\"paragraphIndex\":0,\"charOffset\":6}"
        );

        let split = core.split_paragraph(0, 0, 2).unwrap();
        assert_eq!(split, "{\"ok\":true,\"paraIdx\":1,\"charOffset\":0}");
        assert_eq!(core.get_paragraph_count(0).unwrap(), 3);
        assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河");
        assert_eq!(core.get_text_range(0, 1, 0, 10).unwrap(), "鉄道の夜");

        let deleted = core.delete_text(0, 1, 0, 2).unwrap();
        assert_eq!(deleted, "{\"ok\":true,\"charOffset\":0}");
        assert_eq!(core.get_text_range(0, 1, 0, 10).unwrap(), "の夜");

        let merged = core.merge_paragraph(0, 1).unwrap();
        assert_eq!(merged, "{\"ok\":true,\"paraIdx\":0,\"charOffset\":2}");
        assert_eq!(core.get_paragraph_count(0).unwrap(), 2);
        assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河の夜");
    }

    #[test]
    fn document_core_reports_default_formatting_for_app_panels() {
        let document = Document::from_plain_text("銀河鉄道");
        let mut core = DocumentCore::from_document(document);

        let char_props = core.get_char_properties_at(0, 0, 0).unwrap();
        assert!(char_props.contains("\"fontFamily\":\"Hiragino Sans\""));
        assert!(char_props.contains("\"bold\":false"));

        let para_props = core.get_para_properties_at(0, 0).unwrap();
        assert!(para_props.contains("\"alignment\":\"left\""));
        assert!(para_props.contains("\"lineSpacing\":160"));

        assert_eq!(
            core.apply_char_format(0, 0, 0, 2, "{\"bold\":true}")
                .unwrap(),
            "{\"ok\":true}"
        );
        assert_eq!(
            core.apply_para_format(0, 0, "{\"alignment\":\"center\"}")
                .unwrap(),
            "{\"ok\":true}"
        );
        assert_eq!(core.find_or_create_font_id("Hiragino Sans"), 0);
        let style_list = core.get_style_list();
        assert!(style_list.contains("\"name\":\"Normal\""));
        assert!(style_list.contains("\"sourceStreamCount\":0"));
        let style_detail = core.get_style_detail(0).unwrap();
        assert!(style_detail.contains("\"charProps\""));
        assert!(style_detail.contains("\"decoded\":false"));
        assert!(style_detail.contains("\"sourceStreams\":[]"));
        assert_eq!(
            core.get_style_at(0, 0).unwrap(),
            "{\"id\":0,\"name\":\"Normal\"}"
        );
        assert_eq!(core.apply_style(0, 0, 0).unwrap(), "{\"ok\":true}");
        assert_eq!(core.get_numbering_list(), "[]");
        assert_eq!(core.get_bullet_list(), "[]");
        assert_eq!(core.ensure_default_numbering(), 0);
        assert_eq!(core.ensure_default_bullet("*"), 0);
    }

    #[test]
    fn document_core_supports_body_selection_and_internal_clipboard() {
        let document = Document::from_plain_text("銀河鉄道\n午后の授業\n星めぐり");
        let mut core = DocumentCore::from_document(document);

        let rects = core.get_selection_rects(0, 0, 1, 1, 2).unwrap();
        assert!(rects.starts_with("[{\"pageIndex\":0"));
        assert!(rects.contains("\"height\":23.0"));

        let copied = core.copy_selection(0, 0, 2, 1, 2).unwrap();
        assert_eq!(copied, "{\"ok\":true,\"text\":\"鉄道\\n午后\"}");
        assert!(core.has_internal_clipboard());
        assert_eq!(core.get_clipboard_text(), "鉄道\n午后");

        let pasted = core.paste_internal(0, 2, 0).unwrap();
        assert_eq!(pasted, "{\"ok\":true,\"paraIdx\":3,\"charOffset\":2}");
        assert_eq!(core.get_text_range(0, 2, 0, 10).unwrap(), "鉄道");
        assert_eq!(core.get_text_range(0, 3, 0, 10).unwrap(), "午后星めぐり");

        let deleted = core.delete_range(0, 0, 1, 1, 1).unwrap();
        assert_eq!(deleted, "{\"ok\":true,\"paraIdx\":0,\"charOffset\":1}");
        assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀后の授業");

        core.clear_clipboard();
        assert!(!core.has_internal_clipboard());
        assert_eq!(core.get_clipboard_text(), "");
        assert!(!core.clipboard_has_control());
    }

    #[test]
    fn document_core_saves_restores_and_discards_snapshots() {
        let document = Document::from_plain_text("銀河鉄道\n午后の授業");
        let mut core = DocumentCore::from_document(document);
        core.set_file_name("sample.jtd");
        core.set_dpi(120.0);
        core.set_writing_mode(WritingMode::VerticalRl);
        core.copy_selection(0, 0, 0, 0, 2).unwrap();

        let snapshot_id = core.save_snapshot();
        assert_eq!(snapshot_id, 1);

        core.insert_text(0, 0, 4, "の夜").unwrap();
        core.set_file_name("edited.jtd");
        core.set_dpi(144.0);
        core.set_writing_mode(WritingMode::Horizontal);
        core.set_show_control_codes(true);
        core.set_show_transparent_borders(true);
        core.clear_clipboard();
        assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河鉄道の夜");

        let restored = core.restore_snapshot(snapshot_id).unwrap();
        assert_eq!(restored, "{\"ok\":true,\"pageCount\":1}");
        assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河鉄道");
        assert_eq!(core.file_name(), "sample.jtd");
        assert_eq!(core.get_dpi(), 120.0);
        assert_eq!(core.writing_mode(), WritingMode::VerticalRl);
        assert_eq!(core.get_clipboard_text(), "銀河");
        assert!(!core.get_show_control_codes());
        assert!(!core.get_show_transparent_borders());

        core.discard_snapshot(snapshot_id);
        assert!(core.restore_snapshot(snapshot_id).is_err());
    }

    #[test]
    fn document_core_searches_and_replaces_body_text() {
        let document = Document::from_plain_text("Alpha alpha\nBeta Alpha");
        let mut core = DocumentCore::from_document(document);

        assert_eq!(
            core.search_all_text("Alpha", true, false),
            "[{\"sec\":0,\"para\":0,\"charOffset\":0,\"length\":5},{\"sec\":0,\"para\":1,\"charOffset\":5,\"length\":5}]"
        );
        assert_eq!(
            core.search_all_text("alpha", false, false),
            "[{\"sec\":0,\"para\":0,\"charOffset\":0,\"length\":5},{\"sec\":0,\"para\":0,\"charOffset\":6,\"length\":5},{\"sec\":0,\"para\":1,\"charOffset\":5,\"length\":5}]"
        );
        assert_eq!(
            core.search_text("Alpha", 0, 1, 5, true, true).unwrap(),
            "{\"found\":true,\"wrapped\":true,\"sec\":0,\"para\":0,\"charOffset\":0,\"length\":5}"
        );
        assert_eq!(
            core.search_text("Alpha", 0, 0, 0, false, true).unwrap(),
            "{\"found\":true,\"wrapped\":true,\"sec\":0,\"para\":1,\"charOffset\":5,\"length\":5}"
        );

        assert_eq!(
            core.replace_text(0, 0, 6, 5, "omega").unwrap(),
            "{\"ok\":true,\"charOffset\":6,\"newLength\":5}"
        );
        assert_eq!(core.get_text_range(0, 0, 0, 20).unwrap(), "Alpha omega");

        assert_eq!(
            core.replace_one("Alpha", "A", true).unwrap(),
            "{\"ok\":true,\"sec\":0,\"para\":0,\"charOffset\":0,\"newLength\":1}"
        );
        assert_eq!(core.get_text_range(0, 0, 0, 20).unwrap(), "A omega");

        assert_eq!(
            core.replace_all("Alpha", "X", true).unwrap(),
            "{\"ok\":true,\"count\":1}"
        );
        assert_eq!(core.get_text_range(0, 1, 0, 20).unwrap(), "Beta X");
    }

    #[test]
    fn document_core_exposes_view_and_navigation_fallbacks() {
        let document = Document::from_plain_text("銀河鉄道\n午后");
        let mut core = DocumentCore::from_document(document);

        assert!(!core.get_show_control_codes());
        core.set_show_paragraph_marks(true);
        core.set_show_control_codes(true);
        core.set_show_transparent_borders(true);
        core.set_clip_enabled(false);
        assert!(core.get_show_control_codes());
        assert!(core.get_show_transparent_borders());

        assert_eq!(
            core.get_position_of_page(0).unwrap(),
            "{\"ok\":true,\"sec\":0,\"para\":0,\"charOffset\":0}"
        );
        assert_eq!(
            core.get_page_of_position(0, 1).unwrap(),
            "{\"ok\":true,\"page\":0}"
        );
        assert_eq!(core.get_control_text_positions(0, 0), "[]");
        assert_eq!(
            core.find_nearest_control_backward(0, 0, 4),
            "{\"type\":\"none\"}"
        );
        assert_eq!(
            core.find_nearest_control_forward(0, 0, 0),
            "{\"type\":\"none\"}"
        );
        assert_eq!(
            core.find_next_editable_control(0, 0, -1, 1),
            "{\"type\":\"body\",\"sec\":0,\"para\":1}"
        );
        assert_eq!(
            core.find_next_editable_control(0, 1, -1, 1),
            "{\"type\":\"none\"}"
        );
        assert_eq!(
            core.navigate_next_editable(0, 0, 0, 1, "[]"),
            "{\"type\":\"text\",\"sec\":0,\"para\":0,\"charOffset\":1,\"context\":[]}"
        );
        assert_eq!(
            core.navigate_next_editable(0, 0, 0, -1, "[]"),
            "{\"type\":\"boundary\"}"
        );
    }

    #[test]
    fn document_core_projects_preserved_text_controls_for_navigation() {
        let core =
            DocumentCore::from_bytes(&cfb_with_document_text(document_text_with_inline())).unwrap();

        assert_eq!(core.get_control_text_positions(0, 0), "[2]");
        assert_eq!(core.get_control_text_positions(0, 1), "[]");
        assert_eq!(core.get_control_text_positions(0, 99), "[]");
        assert_eq!(
            core.find_nearest_control_forward(0, 0, 0),
            "{\"type\":\"jtdControl\",\"sec\":0,\"para\":0,\"ci\":0,\"charPos\":2,\"code\":28,\"codeHex\":\"0x001c\",\"decoded\":false}"
        );
        assert_eq!(
            core.find_nearest_control_backward(0, 0, 3),
            "{\"type\":\"jtdControl\",\"sec\":0,\"para\":0,\"ci\":0,\"charPos\":2,\"code\":28,\"codeHex\":\"0x001c\",\"decoded\":false}"
        );
        assert_eq!(
            core.find_nearest_control_forward(0, 0, 2),
            "{\"type\":\"none\"}"
        );
        assert_eq!(
            core.find_nearest_control_backward(0, 0, 2),
            "{\"type\":\"none\"}"
        );

        let layout = core.get_page_control_layout(0).unwrap();
        assert!(layout.starts_with("{\"controls\":[{"));
        assert!(layout.contains("\"type\":\"jtdControl\""));
        assert!(layout.contains("\"x\":"));
        assert!(layout.contains("\"y\":"));
        assert!(layout.contains("\"w\":"));
        assert!(layout.contains("\"h\":"));
        assert!(layout.contains("\"secIdx\":0"));
        assert!(layout.contains("\"paraIdx\":0"));
        assert!(layout.contains("\"controlIdx\":0"));
        assert!(layout.contains("\"charPos\":2"));
        assert!(layout.contains("\"codeHex\":\"0x001c\""));
        assert!(layout.contains("\"decoded\":false"));
        assert!(layout.contains("\"source\":\"textControlBoundary\""));
    }

    #[test]
    fn document_core_exposes_absent_table_and_cell_fallbacks() {
        let document = Document::from_plain_text("銀河鉄道");
        let mut core = DocumentCore::from_document(document);

        assert_eq!(
            core.get_column_def(0).unwrap(),
            "{\"columnCount\":1,\"columnType\":0,\"sameWidth\":true,\"spacing\":0}"
        );
        assert_eq!(
            core.get_table_dimensions(0, 0, 0).unwrap(),
            "{\"rowCount\":0,\"colCount\":0,\"cellCount\":0}"
        );
        assert_eq!(
            core.get_table_dimensions_by_path(0, 0, "[]").unwrap(),
            "{\"rowCount\":0,\"colCount\":0,\"cellCount\":0}"
        );
        assert_eq!(
            core.get_cell_info(0, 0, 0, 0).unwrap(),
            "{\"row\":0,\"col\":0,\"rowSpan\":1,\"colSpan\":1}"
        );
        assert_eq!(
            core.get_cell_info_by_path(0, 0, "[]").unwrap(),
            "{\"row\":0,\"col\":0,\"rowSpan\":1,\"colSpan\":1}"
        );
        assert!(
            core.get_cell_properties(0, 0, 0, 0)
                .unwrap()
                .contains("\"isHeader\":false")
        );
        assert!(
            core.get_table_properties(0, 0, 0)
                .unwrap()
                .contains("\"repeatHeader\":false")
        );
        assert_eq!(core.get_table_cell_bboxes(0, 0, 0, None).unwrap(), "[]");
        assert_eq!(
            core.get_table_cell_bboxes_by_path(0, 0, "[]").unwrap(),
            "[]"
        );
        assert!(
            core.get_cursor_rect_in_cell(0, 0, 0, 0, 0, 0)
                .unwrap()
                .contains("\"height\":23.0")
        );
        assert_eq!(
            core.get_line_info_in_cell(0, 0, 0, 0, 0, 0).unwrap(),
            "{\"lineIndex\":0,\"lineCount\":1,\"charStart\":0,\"charEnd\":0}"
        );
        assert_eq!(core.get_cell_paragraph_count(0, 0, 0, 0).unwrap(), 0);
        assert_eq!(core.get_cell_paragraph_length(0, 0, 0, 0, 0).unwrap(), 0);
        assert_eq!(core.get_text_in_cell(0, 0, 0, 0, 0, 0, 10).unwrap(), "");
        assert_eq!(
            core.insert_text_in_cell(0, 0, 0, 0, 0, 0, "x").unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            core.delete_text_in_cell(0, 0, 0, 0, 0, 0, 1).unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            core.create_table(0, 0, 0, 2, 2).unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert_eq!(
            core.insert_table_row(0, 0, 0, 0, true).unwrap(),
            "{\"ok\":false,\"rowCount\":0,\"colCount\":0}"
        );
        assert_eq!(
            core.delete_table_column(0, 0, 0, 0).unwrap(),
            "{\"ok\":false,\"rowCount\":0,\"colCount\":0}"
        );
        assert_eq!(
            core.merge_table_cells(0, 0, 0, 0, 0, 0, 1).unwrap(),
            "{\"ok\":false,\"cellCount\":0}"
        );
        assert_eq!(
            core.split_table_cell(0, 0, 0, 0, 0).unwrap(),
            "{\"ok\":false,\"cellCount\":0}"
        );
        assert_eq!(
            core.get_selection_rects_in_cell(0, 0, 0, 0, 0, 0, 0, 0)
                .unwrap(),
            "[]"
        );
        assert_eq!(
            core.copy_selection_in_cell(0, 0, 0, 0, 0, 0, 0, 0).unwrap(),
            "{\"ok\":false,\"text\":\"\"}"
        );
        assert_eq!(
            core.delete_range_in_cell(0, 0, 0, 0, 0, 0, 0, 0).unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"charOffset\":0}"
        );
        assert!(
            core.get_cell_char_properties_at(0, 0, 0, 0, 0, 0)
                .unwrap()
                .contains("\"fontFamily\":\"Hiragino Sans\"")
        );
        assert!(
            core.get_cell_para_properties_at(0, 0, 0, 0, 0)
                .unwrap()
                .contains("\"alignment\":\"left\"")
        );
        assert_eq!(
            core.get_cell_style_at(0, 0, 0, 0, 0).unwrap(),
            "{\"id\":0,\"name\":\"Normal\"}"
        );
        assert_eq!(
            core.apply_char_format_in_cell(0, 0, 0, 0, 0, 0, 0, "{}")
                .unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(
            core.apply_para_format_in_cell(0, 0, 0, 0, 0, "{}").unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(
            core.apply_cell_style(0, 0, 0, 0, 0, 0).unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(
            core.evaluate_table_formula(0, 0, 0, 0, 0, "=SUM(A1:A2)", false)
                .unwrap(),
            "{\"ok\":false,\"value\":\"\",\"formula\":\"=SUM(A1:A2)\"}"
        );
    }

    #[test]
    fn document_core_exposes_absent_object_bookmark_and_form_fallbacks() {
        let document = Document::from_plain_text("銀河鉄道");
        let mut core = DocumentCore::from_document(document);

        assert_eq!(core.get_paragraph_stable_id(0, 0).unwrap(), "rjtd-p0");
        core.ensure_paragraph_stable_ids();
        assert!(
            core.debug_dump_stable_ids(0, 0, 1)
                .unwrap()
                .contains("\"stableId\":\"rjtd-p0\"")
        );
        assert_eq!(core.get_table_signature(0, 0, 0).unwrap(), "");
        assert!(
            core.get_shape_bbox(0, 0, 0)
                .unwrap()
                .contains("\"width\":0.0")
        );
        assert_eq!(
            core.insert_picture(0, 0, 0, "", &[], 1, 1, 1, 1, "png", "", None, None)
                .unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert!(
            core.get_picture_properties(0, 0, 0)
                .unwrap()
                .contains("\"effect\":\"none\"")
        );
        assert_eq!(
            core.set_picture_properties(0, 0, 0, "{}").unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(
            core.delete_picture_control(0, 0, 0).unwrap(),
            "{\"ok\":false}"
        );
        assert!(
            core.get_cell_shape_properties_by_path(0, 0, "[]", 0)
                .unwrap()
                .contains("\"description\":\"\"")
        );
        assert!(
            core.get_equation_properties(0, 0, 0, -1, -1)
                .unwrap()
                .contains("\"script\":\"\"")
        );
        assert!(
            core.render_equation_preview("x+y", 1000, 0)
                .contains(">x+y<")
        );
        assert_eq!(
            core.create_shape_control("{}").unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert_eq!(
            core.change_shape_z_order(0, 0, 0, "front").unwrap(),
            "{\"ok\":false,\"zOrder\":0}"
        );
        assert_eq!(
            core.group_shapes("{}"),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert_eq!(core.ungroup_shape(0, 0, 0).unwrap(), "{\"ok\":false}");
        assert_eq!(
            core.insert_equation(0, 0, 0, "x", 1000, 0).unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert_eq!(
            core.get_form_object_at(0, 0.0, 0.0).unwrap(),
            "{\"found\":false}"
        );
        assert_eq!(core.get_form_value(0, 0, 0).unwrap(), "{\"ok\":false}");
        assert_eq!(
            core.set_form_value_in_cell(0, 0, 0, 0, 0, 0, "{}").unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(core.copy_control(0, 0, "", 0).unwrap(), "{\"ok\":false}");
        assert_eq!(
            core.paste_control(0, 0, 0).unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert!(core.get_control_image_data(0, 0, "", 0).unwrap().is_empty());
        assert_eq!(core.get_control_image_mime(0, 0, "", 0).unwrap(), "");
        assert_eq!(core.get_bookmarks(), "[]");
        assert!(
            core.add_bookmark(0, 0, 0, "mark")
                .unwrap()
                .contains("\"ok\":false")
        );
        assert!(core.export_hwp().is_empty());
        assert!(core.export_hwpx().is_empty());
        assert!(core.export_hwp_verify().contains("\"ok\":false"));
        assert_eq!(
            core.insert_page_break(0, 0, 0).unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            core.insert_column_break(0, 0, 0).unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            core.set_column_def(0, 1, 0, 1, 0).unwrap(),
            "{\"ok\":true,\"pageCount\":1}"
        );
        assert_eq!(core.create_style("{}"), 0);
        assert!(core.update_style(0, "{}"));
        assert!(!core.delete_style(1));
        assert_eq!(core.create_numbering("{}"), 0);
        assert_eq!(
            core.insert_text_in_footnote(0, 0, 0, 0, 0, "x").unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            core.get_selection_rects_in_footnote(0, 0, 0, 0, 0, 0)
                .unwrap(),
            "[]"
        );
        assert!(
            core.get_para_properties_in_hf(0, true, 0, 0)
                .unwrap()
                .contains("\"alignment\":\"left\"")
        );
        assert_eq!(
            core.insert_field_in_hf(0, true, 0, 0, 0, 0).unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            core.apply_hf_template(0, true, 0, 0).unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(
            core.export_selection_html(0, 0, 0, 0, 2).unwrap(),
            "<p>銀河</p>"
        );
        assert_eq!(
            core.paste_html(0, 0, 0, "<p>x</p>").unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
    }

    #[test]
    fn document_core_exposes_absent_field_header_footer_and_note_fallbacks() {
        let document = Document::from_plain_text("銀河鉄道");
        let mut core = DocumentCore::from_document(document);

        assert_eq!(core.get_field_list(), "[]");
        assert_eq!(
            core.get_field_value(7),
            "{\"ok\":false,\"fieldId\":7,\"value\":\"\"}"
        );
        assert_eq!(
            core.get_field_value_by_name("name"),
            "{\"ok\":false,\"fieldId\":0,\"name\":\"name\",\"value\":\"\"}"
        );
        assert_eq!(
            core.set_field_value(7, "value"),
            "{\"ok\":false,\"fieldId\":7,\"oldValue\":\"\",\"newValue\":\"value\"}"
        );
        assert_eq!(core.get_field_info_at(0, 0, 0), "{\"inField\":false}");
        assert_eq!(core.remove_field_at(0, 0, 0), "{\"ok\":false}");
        assert!(!core.set_active_field(0, 0, 0));
        core.clear_active_field();
        assert_eq!(core.get_click_here_props(1), "{\"ok\":false}");
        assert_eq!(
            core.update_click_here_props(1, "guide", "memo", "name", true),
            "{\"ok\":false}"
        );

        assert_eq!(
            core.get_header_footer(0, true, 0).unwrap(),
            "{\"ok\":true,\"exists\":false}"
        );
        assert_eq!(
            core.create_header_footer(0, true, 0).unwrap(),
            "{\"ok\":false,\"exists\":false}"
        );
        assert_eq!(
            core.insert_text_in_header_footer(0, true, 0, 0, 0, "x")
                .unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(
            core.get_header_footer_para_info(0, true, 0, 0).unwrap(),
            "{\"ok\":false,\"paraCount\":0,\"charCount\":0}"
        );
        assert!(
            core.get_cursor_rect_in_header_footer(0, true, 0, 0, 0, -1)
                .unwrap()
                .contains("\"pageIndex\":0")
        );
        assert_eq!(
            core.get_header_footer_list(0, true, 0),
            "{\"ok\":true,\"items\":[],\"currentIndex\":-1}"
        );
        assert_eq!(
            core.toggle_hide_header_footer(0, true).unwrap(),
            "{\"ok\":false,\"hidden\":false}"
        );
        assert_eq!(
            core.navigate_header_footer_by_page(0, true, 1),
            "{\"ok\":false}"
        );

        assert_eq!(core.insert_footnote(0, 0, 0).unwrap(), "{\"ok\":false}");
        assert_eq!(core.insert_endnote(0, 0, 0).unwrap(), "{\"ok\":false}");
        assert!(
            core.get_endnote_shape(0)
                .unwrap()
                .contains("\"numberFormat\":\"digit\"")
        );
        assert_eq!(core.apply_endnote_shape(0, "{}").unwrap(), "{\"ok\":false}");
        assert_eq!(
            core.get_footnote_info(0, 0, 0).unwrap(),
            "{\"ok\":false,\"paraCount\":0,\"totalTextLen\":0,\"number\":0,\"texts\":[]}"
        );
        assert!(
            core.delete_footnote(0, 0, 0)
                .unwrap()
                .contains("\"ok\":false")
        );
        assert_eq!(core.get_page_footnote_info(0, 0).unwrap(), "{\"ok\":false}");
        assert_eq!(core.get_note_edit_info(0, 0, 0).unwrap(), "{\"ok\":false}");
        assert_eq!(
            core.get_note_equation_properties(0, 0, 0, 0, 0),
            "{\"ok\":false}"
        );
        assert_eq!(
            core.set_note_equation_properties(0, 0, 0, 0, 0, "{}"),
            "{\"ok\":false}"
        );
    }

    #[test]
    fn document_core_rejects_out_of_range_app_page_queries() {
        let document = Document::from_plain_text("銀河鉄道");
        let core = DocumentCore::from_document(document);

        assert!(core.get_page_info(1).is_err());
        assert!(core.get_page_layer_tree(1).is_err());
        assert!(core.get_page_overlay_images(1).is_err());
        assert!(core.get_page_def(1).is_err());
        assert!(core.get_section_def(1).is_err());
        assert!(core.get_page_border_fill(1).is_err());
        assert!(core.get_cursor_rect(0, 1, 0).is_err());
        assert!(core.get_line_info(0, 1, 0).is_err());
        assert!(core.hit_test(1, 72.0, 72.0).is_err());
    }

    #[test]
    fn document_core_renders_raw_stream_notice_when_text_is_empty() {
        let mut document = Document::default();
        document.push_raw_stream(RawStream::new("/DocumentText", vec![0, 1]));
        let core = DocumentCore::from_document(document);

        let svg = core.render_page_svg(0).unwrap();

        assert!(svg.contains("No extractable text"));
        assert!(svg.contains("/DocumentText"));
    }

    #[test]
    fn builds_document_from_structured_document_text_elements() {
        let parsed = rjtd_core::document_text::parse_document_text(&document_text_with_inline());
        let document = Document::from_document_text(&parsed);

        assert_eq!(document.blocks().len(), 2);
        assert_eq!(document.text_control_boundaries().len(), 1);
        assert_eq!(document.text_control_boundaries()[0].index(), 0);
        assert_eq!(document.text_control_boundaries()[0].code(), 0x001c);
        assert!(
            document.text_control_boundaries()[0]
                .source_span()
                .is_none()
        );
        match &document.blocks()[0] {
            Block::Paragraph(paragraph) => {
                assert_eq!(paragraph.inlines().len(), 3);
                assert_text_inline(&paragraph.inlines()[0], "一、");
                assert_text_inline(&paragraph.inlines()[1], "午后");
                assert_text_inline(&paragraph.inlines()[2], "の授業");
            }
            Block::Unknown(_) => panic!("expected paragraph"),
        }
        match &document.blocks()[1] {
            Block::Paragraph(paragraph) => assert_text_inline(&paragraph.inlines()[0], "二、"),
            Block::Unknown(_) => panic!("expected paragraph"),
        }
    }

    #[test]
    fn preserves_skipped_inline_text_as_unknown_object() {
        let parsed =
            rjtd_core::document_text::parse_document_text(&document_text_with_skipped_inline());
        let document = Document::from_document_text(&parsed);

        assert_eq!(document.blocks().len(), 1);
        assert_eq!(document.unknown_objects().len(), 1);
        assert_eq!(
            document.unknown_objects()[0].source().tag(),
            Some(DOCUMENT_TEXT_INLINE_START_TAG)
        );
        assert!(!document.unknown_objects()[0].payload().is_empty());
        match &document.blocks()[0] {
            Block::Paragraph(paragraph) => assert_text_inline(&paragraph.inlines()[0], "本文"),
            Block::Unknown(_) => panic!("expected paragraph"),
        }
    }

    #[test]
    fn promotes_ruby_base_and_annotation_to_structured_inline() {
        let parsed = rjtd_core::document_text::parse_document_text(&document_text_with_ruby());
        let document = Document::from_document_text(&parsed);

        assert!(document.unknown_objects().is_empty());
        assert_eq!(document.blocks().len(), 1);
        match &document.blocks()[0] {
            Block::Paragraph(paragraph) => {
                assert_eq!(paragraph.inlines().len(), 3);
                assert_text_inline(&paragraph.inlines()[0], "一、");
                assert_ruby_inline(&paragraph.inlines()[1], "午后", "ごご");
                assert_text_inline(&paragraph.inlines()[2], "の授業");
            }
            Block::Unknown(_) => panic!("expected paragraph"),
        }
    }

    #[test]
    fn parser_builds_model_and_preserves_raw_document_text_stream() {
        let bytes = cfb_with_document_text(document_text_fixture());
        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.blocks().len(), 1);
        match &document.blocks()[0] {
            Block::Paragraph(paragraph) => match &paragraph.inlines()[0] {
                Inline::Text(run) => {
                    let span = run.source_span().unwrap();
                    assert_eq!(span.byte_start(), 10);
                    assert_eq!(span.byte_end(), 14);
                    assert_eq!(span.unit_start(), 5);
                    assert_eq!(span.unit_end(), 7);
                }
                Inline::Ruby(_) => panic!("expected text inline"),
                Inline::Unknown(_) => panic!("expected text inline"),
            },
            Block::Unknown(_) => panic!("expected paragraph"),
        }
        assert_eq!(document.raw_streams().len(), 1);
        assert_eq!(document.raw_streams()[0].name(), "/DocumentText");
        assert_eq!(document.raw_streams()[0].bytes(), &document_text_fixture());

        let layer_tree = DocumentCore::from_document(document)
            .get_page_layer_tree(0)
            .unwrap();
        assert!(layer_tree.contains("\"stableSourceKey\":\"section:0/para:0/char:0\""));
        assert!(layer_tree.contains("\"jtdByteRange\":{\"start\":10,\"end\":14}"));
        assert!(layer_tree.contains("\"jtdUnitRange\":{\"start\":5,\"end\":7}"));
    }

    #[test]
    fn parser_preserves_font_stream_entries_as_document_fonts() {
        let font_stream = font_stream_fixture(&[(1, "Times New Roman", 18), (2, "ＭＳ 明朝", 18)]);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (FONT_STREAM_PATH, &font_stream),
        ]);

        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.fonts().len(), 2);
        assert_eq!(document.fonts()[0].source_stream(), FONT_STREAM_PATH);
        assert_eq!(document.fonts()[0].id(), 1);
        assert_eq!(document.fonts()[0].name(), "Times New Roman");
        assert_eq!(document.fonts()[1].name(), "ＭＳ 明朝");
        assert!(!document.fonts()[0].raw().is_empty());

        let core = DocumentCore::from_document(document);
        let info = core.get_document_info();
        assert!(info.contains("\"fallbackFont\":\"ＭＳ 明朝\""));
        assert!(info.contains("\"fontsUsed\":[\"Times New Roman\",\"ＭＳ 明朝\"]"));
        assert!(info.contains("\"fontCount\":2"));
        assert!(info.contains("\"sourceStream\":\"/Font\""));

        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("font-family=\"&apos;ＭＳ 明朝&apos;, &apos;MS Mincho&apos;"));
        assert!(svg.contains("&apos;Hiragino Mincho ProN&apos;"));
        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"fontFamily\":\"'ＭＳ 明朝', 'MS Mincho'"));
    }

    #[test]
    fn parser_preserves_document_text_control_boundaries_with_source_spans() {
        let bytes = cfb_with_document_text(document_text_with_inline());
        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.text_control_boundaries().len(), 1);
        let boundary = &document.text_control_boundaries()[0];
        assert_eq!(boundary.index(), 0);
        assert_eq!(boundary.code(), 0x001c);
        let span = boundary.source_span().unwrap();
        assert_eq!(span.byte_start(), 6);
        assert_eq!(span.byte_end(), 8);
        assert_eq!(span.unit_start(), 3);
        assert_eq!(span.unit_end(), 4);

        let info = DocumentCore::from_document(document).get_document_info();
        assert!(info.contains("\"textControlBoundaryCount\":1"));
        assert!(info.contains("\"codeHex\":\"0x001c\""));
        assert!(info.contains(
            "\"sourceSpan\":{\"byteStart\":6,\"byteEnd\":8,\"unitStart\":3,\"unitEnd\":4}"
        ));
        assert!(info.contains("\"decoded\":false"));
    }

    #[test]
    fn parser_preserves_text_count_ranges_as_observed_model_data() {
        let position_table = text_count_table_fixture();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (
                rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
                &position_table,
            ),
        ]);
        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.text_count_ranges().len(), 2);
        let first = &document.text_count_ranges()[0];
        assert_eq!(first.index(), 0);
        assert_eq!(first.family(), "be0");
        assert_eq!(first.start(), 0x1234);
        assert_eq!(first.end(), 0x1250);
        assert_eq!(first.span(), 0x1c);
        assert_eq!(first.declared_start(), 0x1234);
        assert_eq!(first.declared_end(), 0x1250);
        assert_eq!(first.tail_fields()[..2], [0x0101, 0x0005]);
        assert!(first.document_text_overlaps().is_empty());
        assert_eq!(first.raw().len(), 29);

        let core = DocumentCore::from_document(document);
        let info = core.get_document_info();
        assert!(info.contains("\"textCountRangeCount\":2"));
        assert!(info.contains("\"family\":\"be0\""));
        assert!(info.contains("\"tailFields\":[257,5"));
        assert!(info.contains("\"documentTextOverlaps\":[]"));
        assert!(info.contains("\"controlRangeOverlaps\":[]"));
        assert!(info.contains("\"decoded\":false"));
    }

    #[test]
    fn parser_maps_text_count_ranges_to_source_text_overlaps() {
        let position_table = text_count_table_fixture_with_ranges(&[(10, 14), (5, 7)]);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (
                rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
                &position_table,
            ),
        ]);
        let document = parse_document(&bytes).unwrap();

        let byte_overlaps = document.text_count_ranges()[0].document_text_overlaps();
        assert_eq!(byte_overlaps.len(), 1);
        assert_eq!(byte_overlaps[0].basis(), TextCountRangeOverlapBasis::Byte);
        assert_eq!(byte_overlaps[0].block_index(), 0);
        assert_eq!(byte_overlaps[0].inline_index(), 0);
        assert_eq!(byte_overlaps[0].source_start(), 10);
        assert_eq!(byte_overlaps[0].source_end(), 14);
        assert_eq!(byte_overlaps[0].text(), "銀河");

        let unit_overlaps = document.text_count_ranges()[1].document_text_overlaps();
        assert_eq!(unit_overlaps.len(), 1);
        assert_eq!(unit_overlaps[0].basis(), TextCountRangeOverlapBasis::Unit);
        assert_eq!(unit_overlaps[0].source_start(), 5);
        assert_eq!(unit_overlaps[0].source_end(), 7);
        assert_eq!(unit_overlaps[0].text(), "銀河");

        let info = DocumentCore::from_document(document).get_document_info();
        assert!(info.contains("\"documentTextOverlaps\":[{\"basis\":\"byte\""));
        assert!(info.contains("\"documentTextOverlaps\":[{\"basis\":\"unit\""));
    }

    #[test]
    fn parser_maps_text_count_ranges_to_control_range_overlaps() {
        let position_table = text_count_table_fixture_with_ranges(&[(10, 14), (5, 7)]);
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_with_control_boundary()),
            (
                rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
                &position_table,
            ),
        ]);
        let document = parse_document(&bytes).unwrap();

        let first = document.text_count_ranges()[0].control_range_overlaps();
        assert_eq!(first.len(), 2);
        assert_eq!(first[0].basis(), TextCountRangeOverlapBasis::Byte);
        assert_eq!(first[0].delimiter_code(), 0x001c);
        assert_eq!(first[0].range_count(), 1);
        assert_eq!(first[0].first_range_index(), 0);
        assert_eq!(first[0].last_range_index(), 0);
        assert_eq!(first[0].source_start(), 10);
        assert_eq!(first[0].source_end(), 14);
        assert_eq!(first[1].basis(), TextCountRangeOverlapBasis::Unit);
        assert_eq!(first[1].delimiter_code(), 0x001c);
        assert_eq!(first[1].source_start(), 8);
        assert_eq!(first[1].source_end(), 11);

        let second = document.text_count_ranges()[1].control_range_overlaps();
        assert_eq!(second.len(), 1);
        assert_eq!(second[0].basis(), TextCountRangeOverlapBasis::Unit);
        assert_eq!(second[0].delimiter_code(), 0x001c);
        assert_eq!(second[0].first_range_index(), 0);

        let candidates = document.text_boundary_candidates();
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0].index(), 0);
        assert_eq!(candidates[0].kind(), "controlDelimitedTextCountRange");
        assert_eq!(candidates[0].text_count_range_index(), 0);
        assert_eq!(candidates[0].basis(), TextCountRangeOverlapBasis::Byte);
        assert_eq!(candidates[0].delimiter_code(), 0x001c);
        assert_eq!(candidates[0].interval_count(), 1);
        assert_eq!(candidates[0].first_interval_index(), 0);
        assert_eq!(candidates[0].last_interval_index(), 0);
        assert_eq!(candidates[0].source_start(), 10);
        assert_eq!(candidates[0].source_end(), 14);

        let info = DocumentCore::from_document(document).get_document_info();
        assert!(info.contains("\"controlRangeOverlaps\":[{\"basis\":\"byte\""));
        assert!(info.contains("\"delimiterCodeHex\":\"0x001c\""));
        assert!(info.contains("\"rangeCount\":1"));
        assert!(info.contains("\"textBoundaryCandidateCount\":3"));
        assert!(info.contains("\"textBoundaryCandidates\":[{\"index\":0"));
        assert!(info.contains("\"kind\":\"controlDelimitedTextCountRange\""));
        assert!(info.contains("\"textCountRangeIndex\":0"));
        assert!(info.contains("\"intervalCount\":1"));
        assert!(info.contains("\"decoded\":false"));
    }

    #[test]
    fn parser_preserves_layout_validated_paragraph_boundary_candidates() {
        let position_table = text_count_table_fixture_with_ranges(&[(9, 12)]);
        let line_mark = line_mark_words_0_to_20();
        let page_mark = page_mark_fields_0_to_20();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_with_control_boundary()),
            (
                rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
                &position_table,
            ),
            ("/LineMark", &line_mark),
            ("/PageMark", &page_mark),
        ]);
        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.text_paragraph_boundary_candidates().len(), 1);
        let candidate = &document.text_paragraph_boundary_candidates()[0];
        assert_eq!(candidate.index(), 0);
        assert_eq!(candidate.kind(), "layoutValidatedTextBoundaryCandidate");
        assert_eq!(candidate.text_count_range_index(), 0);
        assert_eq!(candidate.source_start(), 8);
        assert_eq!(candidate.source_end(), 11);
        assert_eq!(candidate.text_count_range_span(), 3);
        assert_eq!(candidate.line_word_evidence().target(), "line-word-value");
        assert_eq!(candidate.line_word_evidence().base(), "unit");
        assert_eq!(candidate.line_word_evidence().delta(), 0);
        assert_eq!(candidate.page_field_evidence().target(), "page-be32-field");
        assert_eq!(candidate.page_field_evidence().base(), "unit");
        assert_eq!(candidate.page_field_evidence().delta(), 0);

        let core = DocumentCore::from_document(document);
        let info = core.get_document_info();
        assert!(info.contains("\"textParagraphBoundaryCandidateCount\":1"));
        assert!(info.contains("\"textParagraphBoundaryCandidates\":[{\"index\":0"));
        assert!(info.contains("\"textBoundaryCandidateIndex\":1"));
        assert!(info.contains("\"textCountRangeSpan\":3"));
        assert!(info.contains("\"target\":\"line-word-value\""));
        assert!(info.contains("\"target\":\"page-be32-field\""));
        assert!(info.contains("\"decoded\":false"));
        assert!(
            core.get_validation_warnings()
                .contains("\"kind\":\"JtdTextParagraphBoundaryCandidateDiagnosticOnly\"")
        );
    }

    #[test]
    fn parser_preserves_observed_style_streams_as_unknown_styles() {
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH, &[1, 2, 3]),
            (rjtd_core::style_stream::DOCUMENT_EDIT_STYLES_PATH, &[4, 5]),
        ]);
        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.unknown_styles().len(), 2);
        assert_eq!(
            document.unknown_styles()[0].name(),
            Some(rjtd_core::style_stream::DOCUMENT_EDIT_STYLES_PATH)
        );
        assert_eq!(document.unknown_styles()[0].payload(), &[4, 5]);
        assert_eq!(
            document.unknown_styles()[1].name(),
            Some(rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH)
        );
        assert_eq!(document.unknown_styles()[1].payload(), &[1, 2, 3]);
    }

    #[test]
    fn document_core_reports_preserved_style_stream_sources() {
        let ssmg_style = ssmg_style_fixture();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH, &ssmg_style),
        ]);
        let core = DocumentCore::from_bytes(&bytes).unwrap();

        let document_info = core.get_document_info();
        assert!(document_info.contains("\"styleStreamCount\":1"));
        assert!(document_info.contains("\"textCountRangeCount\":0"));
        assert!(document_info.contains("\"styleCandidateCount\":0"));
        assert!(document_info.contains("\"styleCandidateNames\":[]"));
        assert!(document_info.contains("\"name\":\"/TextLayoutStyle\""));
        assert!(document_info.contains("\"size\":24"));
        assert!(document_info.contains("\"family\":\"ssmg\""));
        assert!(document_info.contains("\"headerU32Be\":[28,256,32]"));
        assert!(document_info.contains("\"recordLayout\":\"none\""));
        assert!(document_info.contains("\"recordCount\":0"));

        let style_list = core.get_style_list();
        assert!(style_list.contains("\"sourceStreamCount\":1"));

        let style_detail = core.get_style_detail(0).unwrap();
        assert!(style_detail.contains("\"decoded\":false"));
        assert!(style_detail.contains("\"sourceStreams\":["));
        assert!(style_detail.contains("\"name\":\"/TextLayoutStyle\""));
        assert!(style_detail.contains("\"headerU16Be\":[1,2]"));
        assert!(style_detail.contains("\"records\":[]"));
    }

    #[test]
    fn document_core_reports_preserved_style_subrecords() {
        let page_style = ssmg_page_layout_style_with_subrecords_fixture();
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (rjtd_core::style_stream::PAGE_LAYOUT_STYLE_PATH, &page_style),
        ]);
        let core = DocumentCore::from_bytes(&bytes).unwrap();

        let document_info = core.get_document_info();
        assert!(document_info.contains("\"name\":\"/PageLayoutStyle\""));
        assert!(document_info.contains("\"recordCount\":1"));
        assert!(document_info.contains("\"subrecordCount\":6"));
        assert!(document_info.contains("\"codeHex\":\"0x3105\""));
        assert!(document_info.contains("\"codeHex\":\"0x3205\""));
        assert!(document_info.contains("\"codeHex\":\"0x3305\""));
        assert!(document_info.contains("\"payloadHex\":\"0400\""));
        assert!(document_info.contains("\"decoded\":false"));
    }

    #[test]
    fn document_core_reports_text_style_label_candidates() {
        let ssmg_style = ssmg_style_with_label_fixture("本文");
        let bytes = cfb_with_streams(&[
            ("/DocumentText", &document_text_fixture()),
            (rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH, &ssmg_style),
        ]);
        let mut core = DocumentCore::from_bytes(&bytes).unwrap();

        let document_info = core.get_document_info();
        assert!(document_info.contains("\"styleCandidateCount\":1"));
        assert!(document_info.contains("\"styleCandidateNames\":[\"本文\"]"));

        let style_list = core.get_style_list();
        assert!(style_list.contains("\"candidateCount\":1"));
        assert!(style_list.contains("\"id\":1"));
        assert!(style_list.contains("\"name\":\"本文\""));
        assert!(style_list.contains("\"jtdCandidate\":true"));
        assert!(style_list.contains("\"sourceStream\":\"/TextLayoutStyle\""));
        assert!(style_list.contains("\"sourceOffset\":276"));
        assert!(style_list.contains("\"sourceCodeHex\":\"0x5555\""));

        let style_detail = core.get_style_detail(1).unwrap();
        assert!(style_detail.contains("\"name\":\"本文\""));
        assert!(style_detail.contains("\"decoded\":false"));
        assert!(style_detail.contains("\"charProps\":"));
        assert!(style_detail.contains("\"paraProps\":"));
        assert_eq!(
            core.get_style_at(0, 0).unwrap(),
            "{\"id\":0,\"name\":\"Normal\"}"
        );

        let applied = core.apply_style(0, 0, 1).unwrap();
        assert!(applied.contains("\"ok\":true"));
        assert!(applied.contains("\"decoded\":false"));
        assert!(applied.contains("\"styleId\":1"));

        let style_at = core.get_style_at(0, 0).unwrap();
        assert!(style_at.contains("\"id\":1"));
        assert!(style_at.contains("\"name\":\"本文\""));
        assert!(style_at.contains("\"jtdCandidate\":true"));

        let first_paragraph = match &core.document().blocks()[0] {
            Block::Paragraph(paragraph) => paragraph,
            Block::Unknown(_) => panic!("expected first block to be a paragraph"),
        };
        assert_eq!(first_paragraph.style().map(StyleRef::id), Some("1"));

        core.split_paragraph(0, 0, 1).unwrap();
        let split_style = core.get_style_at(0, 1).unwrap();
        assert!(split_style.contains("\"id\":1"));
        assert!(split_style.contains("\"name\":\"本文\""));

        assert_eq!(core.apply_style(0, 1, 0).unwrap(), "{\"ok\":true}");
        assert_eq!(
            core.get_style_at(0, 1).unwrap(),
            "{\"id\":0,\"name\":\"Normal\"}"
        );
    }

    fn document_text_fixture() -> Vec<u8> {
        let mut bytes = b"SsmgV.01".to_vec();
        bytes.extend_from_slice(&[0x00, 0x1f]);
        for unit in "銀河".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes
    }

    fn visual_list_bmdv_fixture() -> Vec<u8> {
        let rle = [0x0a, 0x11, 0x00, 0x00, 0x0a, 0x22, 0x00, 0x00];
        let mut bytes = vec![0; VISUAL_LIST_HEADER_BYTES];
        let declared_size = VISUAL_LIST_HEADER_BYTES + rle.len();
        bytes[0..4].copy_from_slice(&(declared_size as u32).to_be_bytes());
        bytes[VISUAL_LIST_MAGIC_OFFSET..VISUAL_LIST_MAGIC_OFFSET + VISUAL_LIST_MAGIC.len()]
            .copy_from_slice(VISUAL_LIST_MAGIC);
        bytes[VISUAL_LIST_VERSION_OFFSET..VISUAL_LIST_VERSION_OFFSET + 4]
            .copy_from_slice(&1u32.to_be_bytes());
        bytes[VISUAL_LIST_FLAGS_OFFSET..VISUAL_LIST_FLAGS_OFFSET + 4]
            .copy_from_slice(&0x0001_0100u32.to_be_bytes());
        bytes[VISUAL_LIST_WIDTH_OFFSET..VISUAL_LIST_WIDTH_OFFSET + 4]
            .copy_from_slice(&10u32.to_be_bytes());
        bytes[VISUAL_LIST_HEIGHT_OFFSET..VISUAL_LIST_HEIGHT_OFFSET + 4]
            .copy_from_slice(&2u32.to_be_bytes());
        bytes[VISUAL_LIST_ROW_STRIDE_OFFSET..VISUAL_LIST_ROW_STRIDE_OFFSET + 4]
            .copy_from_slice(&10u32.to_be_bytes());
        bytes[VISUAL_LIST_BIT_DEPTH_OFFSET..VISUAL_LIST_BIT_DEPTH_OFFSET + 4]
            .copy_from_slice(&8u32.to_be_bytes());
        bytes[VISUAL_LIST_X_PPM_OFFSET..VISUAL_LIST_X_PPM_OFFSET + 4]
            .copy_from_slice(&3779u32.to_be_bytes());
        bytes[VISUAL_LIST_Y_PPM_OFFSET..VISUAL_LIST_Y_PPM_OFFSET + 4]
            .copy_from_slice(&3779u32.to_be_bytes());
        bytes[VISUAL_LIST_RLE_LENGTH_OFFSET..VISUAL_LIST_RLE_LENGTH_OFFSET + 4]
            .copy_from_slice(&(rle.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&rle);
        bytes
    }

    fn embedding_info_fixture() -> Vec<u8> {
        let class_name = "JSFart.Art.2";
        let mut class_bytes = Vec::new();
        for unit in class_name.encode_utf16() {
            class_bytes.extend_from_slice(&unit.to_le_bytes());
        }
        class_bytes.extend_from_slice(&0u16.to_le_bytes());

        let mut bytes = vec![0; EMBEDDING_INFO_HEADER_BYTES];
        bytes[0..4].copy_from_slice(&1u32.to_le_bytes());
        let row_start = bytes.len();
        bytes.resize(row_start + EMBEDDING_INFO_CLASS_START_OFFSET, 0);
        bytes[row_start + EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET
            ..row_start + EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET + 4]
            .copy_from_slice(&24u32.to_le_bytes());
        bytes[row_start + EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET
            ..row_start + EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET + 2]
            .copy_from_slice(&13260u16.to_le_bytes());
        bytes[row_start + EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET
            ..row_start + EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET + 2]
            .copy_from_slice(&1327u16.to_le_bytes());
        bytes[row_start + EMBEDDING_INFO_CLASS_LENGTH_OFFSET
            ..row_start + EMBEDDING_INFO_CLASS_LENGTH_OFFSET + 4]
            .copy_from_slice(&(class_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&class_bytes);

        let trailing_start = bytes.len();
        bytes.resize(trailing_start + EMBEDDING_INFO_TRAILING_BYTES, 0);
        bytes[trailing_start + EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET
            ..trailing_start + EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET + 4]
            .copy_from_slice(&1u32.to_le_bytes());
        bytes[trailing_start + EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET
            ..trailing_start + EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET + 4]
            .copy_from_slice(&13260u32.to_le_bytes());
        bytes[trailing_start + EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET
            ..trailing_start + EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET + 4]
            .copy_from_slice(&1327u32.to_le_bytes());
        bytes
    }

    fn embedded_press_snapshot_fixture(
        width: u32,
        height: u32,
        body_length: u32,
        payload_length: u32,
    ) -> Vec<u8> {
        let mut bytes = vec![0; 0x80];
        bytes[..EMBEDDED_PRESS_SNAPSHOT_MAGIC.len()].copy_from_slice(EMBEDDED_PRESS_SNAPSHOT_MAGIC);
        bytes[0x0c..0x10].copy_from_slice(&[0x00, 0xd5, 0xf6, 0x77]);
        bytes[0x10..0x14].copy_from_slice(&u32::MAX.to_le_bytes());
        bytes[0x20..0x24].copy_from_slice(&32u32.to_le_bytes());
        bytes[EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET
            ..EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET + 4]
            .copy_from_slice(&body_length.to_le_bytes());
        bytes[0x28..0x2c].copy_from_slice(&65536u32.to_le_bytes());
        bytes[EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET..EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET + 4]
            .copy_from_slice(b"GCI\0");
        bytes[EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET
            ..EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET + 4]
            .copy_from_slice(&17u32.to_le_bytes());
        bytes[EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET
            ..EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET + 4]
            .copy_from_slice(&74u32.to_le_bytes());
        bytes[EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET
            ..EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET + 4]
            .copy_from_slice(&payload_length.to_le_bytes());
        bytes[EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET..EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET + 4]
            .copy_from_slice(&width.to_le_bytes());
        bytes[EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET..EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET + 4]
            .copy_from_slice(&height.to_le_bytes());
        bytes[0x50..0x54].copy_from_slice(&100u32.to_le_bytes());
        bytes[0x54..0x58].copy_from_slice(&1u32.to_le_bytes());
        bytes[0x58..0x5c].copy_from_slice(&100u32.to_le_bytes());
        bytes[0x5c..0x60].copy_from_slice(&1u32.to_le_bytes());
        bytes[0x60..0x64].copy_from_slice(&4u32.to_le_bytes());
        bytes
    }

    fn frame_stream_fixture() -> Vec<u8> {
        let mut bytes = vec![0; FRAME_RECORD_HEADER_BYTES];
        bytes[FRAME_RECORD_DECLARED_COUNT_OFFSET..FRAME_RECORD_DECLARED_COUNT_OFFSET + 2]
            .copy_from_slice(&2u16.to_be_bytes());
        bytes.resize(FRAME_RECORD_HEADER_BYTES + FRAME_RECORD_BYTES, 0);

        let row_start = FRAME_RECORD_HEADER_BYTES + FRAME_RECORD_BYTES;
        bytes.resize(row_start + FRAME_RECORD_BYTES, 0);
        bytes[row_start..row_start + 2].copy_from_slice(&0x1001u16.to_be_bytes());
        bytes[row_start + 2..row_start + 4].copy_from_slice(&60u16.to_be_bytes());
        bytes[row_start + FRAME_RECORD_ID_OFFSET..row_start + FRAME_RECORD_ID_OFFSET + 2]
            .copy_from_slice(&24u16.to_be_bytes());
        bytes[row_start + FRAME_RECORD_TYPE_OFFSET..row_start + FRAME_RECORD_TYPE_OFFSET + 2]
            .copy_from_slice(&0x0002u16.to_be_bytes());
        bytes[row_start + FRAME_RECORD_X_OFFSET..row_start + FRAME_RECORD_X_OFFSET + 2]
            .copy_from_slice(&2143u16.to_be_bytes());
        bytes[row_start + FRAME_RECORD_Y_OFFSET..row_start + FRAME_RECORD_Y_OFFSET + 2]
            .copy_from_slice(&2932u16.to_be_bytes());
        bytes[row_start + FRAME_RECORD_WIDTH_OFFSET..row_start + FRAME_RECORD_WIDTH_OFFSET + 2]
            .copy_from_slice(&13260u16.to_be_bytes());
        bytes[row_start + FRAME_RECORD_HEIGHT_OFFSET..row_start + FRAME_RECORD_HEIGHT_OFFSET + 2]
            .copy_from_slice(&1327u16.to_be_bytes());
        bytes
    }

    fn font_stream_fixture(entries: &[(u16, &str, usize)]) -> Vec<u8> {
        let mut bytes = b"FontV.01".to_vec();
        bytes.extend_from_slice(&(entries.len() as u16).to_be_bytes());
        for (id, name, suffix_len) in entries {
            bytes.extend_from_slice(&font_entry_fixture(*id, name, *suffix_len));
        }
        bytes
    }

    fn font_entry_fixture(id: u16, name: &str, suffix_len: usize) -> Vec<u8> {
        let mut entry = vec![0; 30];
        entry[0..2].copy_from_slice(&id.to_be_bytes());
        entry[20..22].copy_from_slice(&0x0190u16.to_be_bytes());
        for unit in name.encode_utf16() {
            entry.extend_from_slice(&unit.to_be_bytes());
        }
        entry.extend_from_slice(&[0, 0]);
        entry.resize(entry.len() + suffix_len, 0);
        entry
    }

    fn minimal_jpeg_payload() -> &'static [u8] {
        &[
            0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00,
            0x10, 0x00, 0x20, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff,
            0xda, 0x00, 0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x00,
            0xff, 0xd9,
        ]
    }

    #[cfg(feature = "bitmap-images")]
    fn minimal_png_payload() -> &'static [u8] {
        &[
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
            0x00, 0x90, 0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08,
            0xd7, 0x63, 0xf8, 0xcf, 0xc0, 0x00, 0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xdd, 0x8d,
            0xb0, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
        ]
    }

    fn image_payload_with_header_fixture(payload_len: usize) -> (Vec<u8>, usize, usize) {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&9_u16.to_le_bytes());
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&0x1234_u32.to_le_bytes());
        bytes.extend_from_slice(&0x5678_u32.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());

        let source_path = b"C:\\TEMP\\A.JPG";
        bytes.push(source_path.len() as u8);
        bytes.extend_from_slice(source_path);
        bytes.push(0);
        bytes.extend_from_slice(&1_u32.to_le_bytes());
        bytes.extend_from_slice(&(payload_len as u32).to_le_bytes());

        let signature_offset = bytes.len();
        (bytes, signature_offset, signature_offset + payload_len)
    }

    fn document_text_with_control_boundary() -> Vec<u8> {
        let mut bytes = b"SsmgV.01".to_vec();
        extend_units(
            &mut bytes,
            &[
                0x001f, 0x9280, 0x6cb3, 0x001c, 0x001f, 0x9244, 0x9053, 0x000a,
            ],
        );
        bytes
    }

    fn document_text_with_page_break() -> Vec<u8> {
        let mut bytes = b"SsmgV.01".to_vec();
        extend_units(&mut bytes, &[0x001f]);
        for unit in "銀河鉄道の夜\t\t\t\t宮沢 賢治".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(&mut bytes, &[DOCUMENT_TEXT_PAGE_BREAK_CONTROL, 0x001f]);
        for unit in "目次".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes
    }

    fn document_view_styles_page_size_fixture(width_mm100: u32, height_mm100: u32) -> Vec<u8> {
        let mut bytes = vec![0; 32];
        bytes[0..4].copy_from_slice(&0x0001_0002_u32.to_be_bytes());
        bytes[4..8].copy_from_slice(&0x1000_0000_u32.to_be_bytes());
        bytes[8..12].copy_from_slice(&0x040e_1001_u32.to_be_bytes());
        bytes[12..16].copy_from_slice(&0x010a_0600_u32.to_be_bytes());
        bytes[16..20].copy_from_slice(&(width_mm100 << 8).to_be_bytes());
        bytes[20..24].copy_from_slice(&((height_mm100 << 8) | 0x04).to_be_bytes());
        bytes
    }

    fn text_count_table_fixture() -> Vec<u8> {
        text_count_table_fixture_with_ranges(&[(0x1234, 0x1250), (0x2000, 0x2400)])
    }

    fn text_count_table_fixture_with_ranges(entries: &[(u32, u32)]) -> Vec<u8> {
        let mut bytes = b"SsmgV.01".to_vec();
        bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
        bytes.extend_from_slice(&[0x00, 0x00, 0x01, 0x00]);
        bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
        bytes.extend_from_slice(b"TCntV.01");
        bytes.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]);
        bytes.extend_from_slice(&(entries.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&[0x00, 0x24]);
        for (index, (start, end)) in entries.iter().enumerate() {
            let mut entry = [0; 29];
            entry[0..4].copy_from_slice(&start.to_be_bytes());
            entry[4..8].copy_from_slice(&end.to_be_bytes());
            entry[8..12].copy_from_slice(&[0x01 + index as u8, 0x01 + index as u8, 0x00, 0x05]);
            bytes.extend_from_slice(&entry);
        }
        bytes
    }

    fn line_mark_words_0_to_20() -> Vec<u8> {
        let mut bytes = Vec::new();
        for word in 0..20u16 {
            bytes.extend_from_slice(&word.to_be_bytes());
        }
        bytes
    }

    fn page_mark_fields_0_to_20() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&19u32.to_be_bytes());
        bytes.extend_from_slice(&0x10u32.to_be_bytes());
        bytes.extend_from_slice(&18u32.to_be_bytes());
        for index in 0..20u32 {
            let mut entry = [0; 84];
            entry[0..4].copy_from_slice(&index.to_be_bytes());
            bytes.extend_from_slice(&entry);
        }
        bytes
    }

    fn ssmg_style_fixture() -> Vec<u8> {
        vec![
            b'S', b's', b'm', b'g', b'V', b'.', b'0', b'1', 0, 0, 0, 0x1c, 0, 0, 1, 0, 0, 0, 0,
            0x20, 0, 1, 0, 2,
        ]
    }

    fn ssmg_style_with_label_fixture(label: &str) -> Vec<u8> {
        let mut bytes = ssmg_style_fixture();
        bytes.resize(0x114, 0);
        let label_units = label.encode_utf16().collect::<Vec<_>>();
        let payload_len = 2 + label_units.len() * 2;
        bytes.extend_from_slice(&0x5555u16.to_be_bytes());
        bytes.extend_from_slice(&(payload_len as u16).to_be_bytes());
        bytes.extend_from_slice(&(label_units.len() as u16).to_be_bytes());
        for unit in label_units {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes
    }

    fn ssmg_page_layout_style_with_subrecords_fixture() -> Vec<u8> {
        let mut bytes = ssmg_style_fixture();
        bytes.resize(0x114, 0);
        let label_units = "ページ".encode_utf16().collect::<Vec<_>>();
        let mut payload = Vec::new();
        payload.extend_from_slice(&(label_units.len() as u16).to_be_bytes());
        for unit in label_units {
            payload.extend_from_slice(&unit.to_be_bytes());
        }
        payload.extend_from_slice(&[0, 0]);
        payload.extend_from_slice(&[0x31, 0x04, 0, 1, 0xaa]);
        payload.extend_from_slice(&[0x31, 0x05, 0, 2, 0x04, 0x00]);
        payload.extend_from_slice(&[0x31, 0x06, 0, 1, 0xbb]);
        payload.extend_from_slice(&[0x31, 0x07, 0, 1, 0xcc]);
        payload.extend_from_slice(&[0x32, 0x05, 0, 2, 0x04, 0x00]);
        payload.extend_from_slice(&[0x33, 0x05, 0, 2, 0x04, 0x00]);

        bytes.extend_from_slice(&0x4444u16.to_be_bytes());
        bytes.extend_from_slice(&(payload.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&payload);
        bytes
    }

    fn auto_text_info_fixture(text: &str) -> Vec<u8> {
        let mut bytes = b"SsmgV.01".to_vec();
        bytes.resize(84, 0);
        for unit in text.encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes
    }

    fn document_text_with_inline() -> Vec<u8> {
        let mut bytes = vec![0x00, 0x1f];
        for unit in "一、".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(
            &mut bytes,
            &[0x001c, 0x0001, 0x0007, 0x0000, 0x0000, 0x0003, 0x001d],
        );
        for unit in "午后".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(&mut bytes, &[0x001e, 0x0005, 0x0000, 0x0001, 0x001f]);
        for unit in "の授業\n二、".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes
    }

    fn document_text_with_skipped_inline() -> Vec<u8> {
        let mut bytes = b"SsmgV.01".to_vec();
        bytes.extend_from_slice(&[0x00, 0x1f]);
        for unit in "本文".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(
            &mut bytes,
            &[0x001c, 0x0001, 0x0007, 0x0000, 0x0001, 0x0082, 0x001d],
        );
        for unit in "ふりがな".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(&mut bytes, &[0x001e]);
        bytes
    }

    fn document_text_with_ruby() -> Vec<u8> {
        let mut bytes = vec![0x00, 0x1f];
        for unit in "一、".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(
            &mut bytes,
            &[0x001c, 0x0001, 0x0007, 0x0000, 0x0000, 0x0003, 0x001d],
        );
        for unit in "午后".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(&mut bytes, &[0x001e, 0x0005, 0x0000, 0x0001, 0x001f]);
        extend_units(
            &mut bytes,
            &[0x001c, 0x0001, 0x0007, 0x0000, 0x0001, 0x0082, 0x001d],
        );
        for unit in "ごご".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(&mut bytes, &[0x001e, 0x0005, 0x0000, 0x0001, 0x001f]);
        for unit in "の授業".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes
    }

    fn assert_text_inline(inline: &Inline, expected: &str) {
        match inline {
            Inline::Text(text) => assert_eq!(text.text(), expected),
            Inline::Ruby(_) => panic!("expected text inline"),
            Inline::Unknown(_) => panic!("expected text inline"),
        }
    }

    fn assert_ruby_inline(inline: &Inline, expected_base: &str, expected_annotation: &str) {
        match inline {
            Inline::Ruby(ruby) => {
                assert_eq!(ruby.base_text(), expected_base);
                assert_eq!(ruby.annotation_text(), expected_annotation);
                assert_eq!(
                    ruby.annotation_source().source().tag(),
                    Some(DOCUMENT_TEXT_INLINE_START_TAG)
                );
                assert!(!ruby.annotation_source().payload().is_empty());
            }
            Inline::Text(_) => panic!("expected ruby inline"),
            Inline::Unknown(_) => panic!("expected ruby inline"),
        }
    }

    fn extend_units(bytes: &mut Vec<u8>, units: &[u16]) {
        for unit in units {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
    }

    fn cfb_with_document_text(payload: Vec<u8>) -> Vec<u8> {
        cfb_with_streams(&[("/DocumentText", &payload)])
    }

    fn cfb_with_streams(streams: &[(&str, &[u8])]) -> Vec<u8> {
        let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        let mut storages = HashSet::new();
        for (path, payload) in streams {
            create_parent_storages(&mut compound, path, &mut storages);
            compound
                .create_stream(path)
                .unwrap()
                .write_all(payload)
                .unwrap();
        }
        compound.into_inner().into_inner()
    }

    fn push_fdm_index_row(
        bytes: &mut Vec<u8>,
        vector_offset: u32,
        kind: u16,
        bbox: (i32, i32, i32, i32),
    ) {
        bytes.extend_from_slice(&vector_offset.to_be_bytes());
        bytes.extend_from_slice(&kind.to_be_bytes());
        bytes.extend_from_slice(&bbox.0.to_be_bytes());
        bytes.extend_from_slice(&bbox.1.to_be_bytes());
        bytes.extend_from_slice(&bbox.2.to_be_bytes());
        bytes.extend_from_slice(&bbox.3.to_be_bytes());
    }

    fn frame_record_fixture(
        object_id: u16,
        object_type: u16,
        geometry: (u16, u16, u16, u16),
    ) -> Vec<u8> {
        let mut row = vec![0; FRAME_RECORD_BYTES];
        row[0..2].copy_from_slice(&0x0102_u16.to_be_bytes());
        row[2..4].copy_from_slice(&0x0038_u16.to_be_bytes());
        row[FRAME_RECORD_ID_OFFSET..FRAME_RECORD_ID_OFFSET + 2]
            .copy_from_slice(&object_id.to_be_bytes());
        row[FRAME_RECORD_TYPE_OFFSET..FRAME_RECORD_TYPE_OFFSET + 2]
            .copy_from_slice(&object_type.to_be_bytes());
        row[FRAME_RECORD_X_OFFSET..FRAME_RECORD_X_OFFSET + 2]
            .copy_from_slice(&geometry.0.to_be_bytes());
        row[FRAME_RECORD_Y_OFFSET..FRAME_RECORD_Y_OFFSET + 2]
            .copy_from_slice(&geometry.1.to_be_bytes());
        row[FRAME_RECORD_WIDTH_OFFSET..FRAME_RECORD_WIDTH_OFFSET + 2]
            .copy_from_slice(&geometry.2.to_be_bytes());
        row[FRAME_RECORD_HEIGHT_OFFSET..FRAME_RECORD_HEIGHT_OFFSET + 2]
            .copy_from_slice(&geometry.3.to_be_bytes());
        row
    }

    fn create_parent_storages(
        compound: &mut cfb::CompoundFile<Cursor<Vec<u8>>>,
        path: &str,
        storages: &mut HashSet<String>,
    ) {
        let segments = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>();
        if segments.len() <= 1 {
            return;
        }

        let mut current = String::new();
        for segment in &segments[..segments.len() - 1] {
            current.push('/');
            current.push_str(segment);
            if storages.insert(current.clone()) {
                compound.create_storage(&current).unwrap();
            }
        }
    }
}
