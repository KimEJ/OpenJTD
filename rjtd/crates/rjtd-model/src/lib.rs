//! Document model types shared by parsers and exporters.

use std::collections::{BTreeMap, BTreeSet};

use rjtd_core::container::{EntryKind, inspect_cfb_entries, read_cfb_stream};
use rjtd_core::document_text::{
    DocumentTextControl, DocumentTextElement, DocumentTextMapEntry, DocumentTextMapKind,
    DocumentTextPayload, InlineTextSegment, ParsedDocumentText, SkippedInlineTextSegment,
    map_document_text, read_document_text_payload,
};
use rjtd_core::document_text_position::{
    DocumentTextCountEntry, read_document_text_position_tables,
};
use rjtd_core::layout_mark::{PageMark, read_page_mark};
use rjtd_core::record::UnknownRecordKind;
use rjtd_core::style_stream::{
    StyleStreamRecordSummary, TEXT_LAYOUT_STYLE_PATH, read_style_streams, summarize_style_stream,
};
use rjtd_core::{Error, Result};

const DOCUMENT_TEXT_INLINE_START_TAG: u32 = 0x001d;
const DOCUMENT_TEXT_RUBY_BASE_SELECTOR: u16 = 0x0003;
const DOCUMENT_TEXT_RUBY_TEXT_SELECTOR: u16 = 0x0082;
const TEXT_CONTROL_RANGE_DELIMITER_CANDIDATES: [u16; 2] = [0x001c, 0x000e];
const PARAGRAPH_BOUNDARY_DELIMITER_CANDIDATE: u16 = 0x001c;
const LAYOUT_MAP_DELTA_MIN: isize = -4096;
const LAYOUT_MAP_DELTA_MAX: isize = 4096;
const SO_RECORD_MARKER: &[u8] = b"SO\0\0";
const OBJECT_STREAM_PREFIX_PREVIEW_BYTES: usize = 16;
const OBJECT_STREAM_REFERENCE_OFFSET_PREVIEW_LIMIT: usize = 16;
const OBJECT_STREAM_REFERENCE_ROW_LIMIT: usize = 16;
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
    text_count_ranges: Vec<TextCountRange>,
    text_control_boundaries: Vec<TextControlBoundary>,
    text_boundary_candidates: Vec<TextBoundaryCandidate>,
    text_paragraph_boundary_candidates: Vec<TextParagraphBoundaryCandidate>,
    table_candidates: Vec<TableCandidate>,
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
            text_count_ranges: Vec::new(),
            text_control_boundaries: Vec::new(),
            text_boundary_candidates: Vec::new(),
            text_paragraph_boundary_candidates: Vec::new(),
            table_candidates: Vec::new(),
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
        for candidate in object_stream_candidates_from_cfb(data) {
            document.push_object_stream_candidate(candidate);
        }
        for record in object_frame_records_from_cfb(data) {
            document.push_object_frame_record(record);
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
            for candidate in table_candidates_from_text_boundaries(&document) {
                document.push_table_candidate(candidate);
            }
            for candidate in
                text_paragraph_boundary_candidates_from_layout(&document, map.entries(), data)
            {
                document.push_text_paragraph_boundary_candidate(candidate);
            }
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
const APP_FONT_SIZE_PX: f32 = 15.0;
const APP_LINE_HEIGHT_PX: f32 = 23.0;
const APP_LINES_PER_PAGE: usize = 42;
const APP_WRAP_COLUMNS: usize = 82;
const APP_SOURCE_FORMAT: &str = "jtd";
const APP_DEFAULT_DPI: f64 = 96.0;

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
        let pages = paginate_document_text(&document);
        Self {
            document,
            pages,
            file_name: String::new(),
            dpi: APP_DEFAULT_DPI,
            show_paragraph_marks: false,
            show_control_codes: false,
            show_transparent_borders: false,
            clip_enabled: true,
            writing_mode: WritingMode::Horizontal,
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

    pub fn page_count(&self) -> u32 {
        self.pages.len().max(1) as u32
    }

    pub fn get_section_count(&self) -> u32 {
        1
    }

    pub fn get_document_info(&self) -> String {
        let style_candidates = text_style_candidates(self.document.unknown_styles());
        format!(
            "{{\"version\":\"0.0.0\",\"format\":\"JTD\",\"engine\":\"rjtd\",\"sourceFormat\":\"{}\",\"fileName\":{},\"sectionCount\":1,\"pageCount\":{},\"encrypted\":false,\"hwp3Variant\":false,\"fallbackFont\":\"Hiragino Sans\",\"fontsUsed\":[\"Hiragino Sans\"],\"writingMode\":\"{}\",\"writingModeDecoded\":false,\"blockCount\":{},\"rawStreamCount\":{},\"styleStreamCount\":{},\"styleCandidateCount\":{},\"styleCandidateNames\":{},\"styleStreams\":{},\"objectStreamCandidateCount\":{},\"objectStreamCandidates\":{},\"objectFrameRecordCount\":{},\"objectFrameRecords\":{},\"textCountRangeCount\":{},\"textCountRanges\":{},\"textControlBoundaryCount\":{},\"textControlBoundaries\":{},\"textBoundaryCandidateCount\":{},\"textBoundaryCandidates\":{},\"textParagraphBoundaryCandidateCount\":{},\"textParagraphBoundaryCandidates\":{},\"tableCandidateCount\":{},\"tableCandidates\":{}}}",
            APP_SOURCE_FORMAT,
            json_string(&self.file_name),
            self.page_count(),
            self.writing_mode.as_str(),
            self.document.blocks().len(),
            self.document.raw_streams().len(),
            self.document.unknown_styles().len(),
            style_candidates.len(),
            style_candidate_names_json(&style_candidates),
            style_source_streams_json(self.document.unknown_styles()),
            self.document.object_stream_candidates().len(),
            object_stream_candidates_json(self.document.object_stream_candidates()),
            self.document.object_frame_records().len(),
            object_frame_records_json(self.document.object_frame_records()),
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
    }

    pub fn get_page_def(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(format!(
            "{{\"width\":{:.1},\"height\":{:.1},\"marginLeft\":{:.1},\"marginRight\":{:.1},\"marginTop\":{:.1},\"marginBottom\":{:.1},\"marginHeader\":0.0,\"marginFooter\":0.0,\"marginGutter\":0.0,\"landscape\":false,\"binding\":0}}",
            APP_PAGE_WIDTH_PX,
            APP_PAGE_HEIGHT_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX
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
        APP_PAGE_WIDTH_PX as f64
    }

    pub fn page_height_px(&self) -> f64 {
        APP_PAGE_HEIGHT_PX as f64
    }

    pub fn page_margin_px(&self) -> f64 {
        APP_PAGE_MARGIN_PX as f64
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

    pub fn get_page_info(&self, page_num: u32) -> Result<String> {
        self.page_lines(page_num)?;
        let body_x = APP_PAGE_MARGIN_PX;
        let body_width = APP_PAGE_WIDTH_PX - (APP_PAGE_MARGIN_PX * 2.0);
        Ok(format!(
            "{{\"pageIndex\":{},\"pageNumber\":{},\"width\":{:.1},\"height\":{:.1},\"sectionIndex\":0,\"marginLeft\":{:.1},\"marginRight\":{:.1},\"marginTop\":{:.1},\"marginBottom\":{:.1},\"marginHeader\":0.0,\"marginFooter\":0.0,\"pageBorderLeft\":{:.1},\"pageBorderRight\":{:.1},\"pageBorderTop\":{:.1},\"pageBorderBottom\":{:.1},\"columns\":[{{\"x\":{:.1},\"width\":{:.1}}}]}}",
            page_num,
            page_num + 1,
            APP_PAGE_WIDTH_PX,
            APP_PAGE_HEIGHT_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
            APP_PAGE_MARGIN_PX,
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
        Ok(page_layer_tree_json(self, lines, profile))
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
            controls.push(projected_control_layout_json(&control, &rect));
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
        _control_idx: u32,
        _cell_idx: u32,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(0)
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
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(0)
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
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(String::new())
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
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_line_info_json())
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
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_dimensions_json())
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
        _control_idx: u32,
        _cell_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_info_json())
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
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(String::new())
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
                let start_rect = cursor_rect_from_line(page_index, line_index, line, start);
                let end_rect = cursor_rect_from_line(page_index, line_index, line, end);
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

        Ok(render_text_page_svg(
            lines,
            index + 1,
            self.page_count() as usize,
            self.writing_mode,
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
        let Some((line_index, line)) = nearest_text_line(lines, line_index_for_y(lines.len(), y))
        else {
            return Ok(format!(
                "{{\"hit\":false,\"sectionIndex\":0,\"paragraphIndex\":0,\"charOffset\":0,\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1}}}",
                page_num,
                normalize_coordinate(x),
                normalize_coordinate(y)
            ));
        };
        let paragraph_index = line.paragraph_index().unwrap_or_default();
        let char_offset = char_offset_for_x(line, x);
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
        let new_char_offset = char_offset_for_x(target_line, target_x);
        let rect = cursor_rect_from_line(page_index, page_line_index, target_line, new_char_offset);
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
        self.pages = paginate_document_text(&self.document);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectStreamCandidateReason {
    ObjectPath,
    ImagePath,
    ShapePath,
    TablePath,
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
            Self::SoMarker => "so-marker",
            Self::ImageSignature => "image-signature",
            Self::SvgSignature => "svg-signature",
        }
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
    svg_offsets: Vec<usize>,
    so_offsets: Vec<usize>,
}

impl ObjectStreamCandidateEvidence {
    pub fn new(
        reasons: Vec<ObjectStreamCandidateReason>,
        image_signature_hits: Vec<ObjectImageSignatureHit>,
        image_payload_spans: Vec<ObjectImagePayloadSpan>,
        svg_offsets: Vec<usize>,
        so_offsets: Vec<usize>,
    ) -> Self {
        Self {
            reasons,
            image_signature_hits,
            image_payload_spans,
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

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> &'static str {
        "multiIntervalControlRangeTableCandidate"
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

    pub fn rule(&self) -> &'static str {
        "control-delimited-text-count-range-with-multiple-intervals"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCandidateInterval {
    index: usize,
    source_interval_index: usize,
    source_start: usize,
    source_end: usize,
    text_preview: String,
    line_break_count: usize,
}

impl TableCandidateInterval {
    fn new(
        index: usize,
        source_interval_index: usize,
        source_start: usize,
        source_end: usize,
        text_preview: String,
        line_break_count: usize,
    ) -> Self {
        Self {
            index,
            source_interval_index,
            source_start,
            source_end,
            text_preview,
            line_break_count,
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

    pub fn line_break_count(&self) -> usize {
        self.line_break_count
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
    if !image_signature_hits.is_empty() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::ImageSignature);
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
            svg_offsets,
            so_offsets,
        ),
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
    ))
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

fn table_candidates_from_text_boundaries(document: &Document) -> Vec<TableCandidate> {
    let Some(bounds) = document_text_source_bounds(document) else {
        return Vec::new();
    };

    let mut table_candidates = Vec::new();
    for candidate in document.text_boundary_candidates() {
        if candidate.interval_count() <= 1 {
            continue;
        }
        let intervals = table_candidate_intervals(document, &bounds, candidate);
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

fn table_candidate_intervals(
    document: &Document,
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
            let text = text_for_source_range(document, candidate.basis(), source_start, source_end);
            let text_preview = preview_text(&text, 80);
            let line_break_count = text_line_break_count(&text);
            Some(TableCandidateInterval::new(
                0,
                interval.index,
                source_start,
                source_end,
                text_preview,
                line_break_count,
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

fn text_for_source_range(
    document: &Document,
    basis: TextCountRangeOverlapBasis,
    source_start: usize,
    source_end: usize,
) -> String {
    let mut text = String::new();
    for block in document.blocks() {
        let Block::Paragraph(paragraph) = block else {
            continue;
        };
        for inline in paragraph.inlines() {
            let Inline::Text(run) = inline else {
                continue;
            };
            let Some(span) = run.source_span() else {
                continue;
            };
            let (entry_start, entry_end) = source_span_range(span, basis);
            let overlap_start = entry_start.max(source_start);
            let overlap_end = entry_end.min(source_end);
            if overlap_start >= overlap_end {
                continue;
            }
            text.push_str(&text_for_source_overlap(
                run.text(),
                span,
                basis,
                overlap_start,
                overlap_end,
            ));
        }
    }
    text
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

fn projected_control_layout_json(control: &ProjectedTextControl, rect: &CursorRect) -> String {
    format!(
        "{{\"type\":\"jtdControl\",\"x\":{:.1},\"y\":{:.1},\"w\":{:.1},\"h\":{:.1},\"secIdx\":0,\"paraIdx\":{},\"controlIdx\":{},\"charPos\":{},\"code\":{},\"codeHex\":{},\"decoded\":false,\"source\":\"textControlBoundary\"}}",
        rect.x,
        rect.y,
        column_width_px(),
        rect.height,
        control.paragraph_index,
        control.boundary_index,
        control.char_offset,
        control.code,
        json_string(&format!("0x{:04x}", control.code)),
    )
}

fn paginate_document_text(document: &Document) -> Vec<Vec<PageTextLine>> {
    let mut lines = Vec::new();
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                let text = paragraph_text(paragraph);
                let wrapped = wrap_text_line(&text, paragraph_index, APP_WRAP_COLUMNS);
                if wrapped.is_empty() {
                    lines.push(PageTextLine::new(
                        String::new(),
                        Some(paragraph_index),
                        0,
                        0,
                    ));
                } else {
                    lines.extend(wrapped);
                }
                lines.push(PageTextLine::new(String::new(), None, 0, 0));
                paragraph_index += 1;
            }
            Block::Unknown(_) => {
                lines.push(PageTextLine::new(
                    "[UnknownBlock preserved by rjtd]".to_string(),
                    None,
                    0,
                    0,
                ));
                lines.push(PageTextLine::new(String::new(), None, 0, 0));
            }
        }
    }

    while lines
        .last()
        .is_some_and(|line| line.text().is_empty() && line.paragraph_index().is_none())
    {
        lines.pop();
    }

    if lines.is_empty() {
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

    lines
        .chunks(APP_LINES_PER_PAGE)
        .map(|chunk| chunk.to_vec())
        .collect()
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
    if character.is_ascii() { 1 } else { 2 }
}

fn column_width_px() -> f64 {
    (APP_PAGE_WIDTH_PX as f64 - (APP_PAGE_MARGIN_PX as f64 * 2.0)) / APP_WRAP_COLUMNS as f64
}

fn line_index_for_y(line_count: usize, y: f64) -> usize {
    if line_count == 0 {
        return 0;
    }

    let relative_y = normalize_coordinate(y) - APP_PAGE_MARGIN_PX as f64;
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
    page_index: usize,
    line_index: usize,
    line: &PageTextLine,
    char_offset: usize,
) -> CursorRect {
    let char_offset = char_offset.clamp(line.char_start(), line.char_end());
    let x = APP_PAGE_MARGIN_PX as f64 + column_units_before(line, char_offset) * column_width_px();
    let y = APP_PAGE_MARGIN_PX as f64 + line_index as f64 * APP_LINE_HEIGHT_PX as f64;

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

fn char_offset_for_x(line: &PageTextLine, x: f64) -> usize {
    let target_units =
        ((normalize_coordinate(x) - APP_PAGE_MARGIN_PX as f64) / column_width_px()).max(0.0);
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

fn default_cell_info_json() -> String {
    "{\"row\":0,\"col\":0,\"rowSpan\":1,\"colSpan\":1}".to_string()
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
    output.push_str(",\"payloadPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(candidate.payload_prefix())));
    output.push_str(",\"decoded\":false}");
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
    push_table_candidate_intervals_json(output, candidate.intervals());
    output.push_str(",\"rule\":");
    output.push_str(&json_string(candidate.rule()));
    output.push_str(",\"decoded\":false}");
}

fn push_table_candidate_intervals_json(output: &mut String, intervals: &[TableCandidateInterval]) {
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
        output.push_str(",\"lineBreakCount\":");
        output.push_str(&interval.line_break_count().to_string());
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
        output.push('}');
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
}

fn page_overlay_images_json(core: &DocumentCore) -> String {
    let diagnostics = fdm_image_overlay_diagnostics_json(&core.document);
    if diagnostics.is_empty() {
        return "{\"behind\":[],\"front\":[],\"imageCount\":0}".to_string();
    }

    format!(
        "{{\"behind\":[],\"front\":[],\"imageCount\":0,\"unplacedDiagnostics\":[{}],\"diagnosticCount\":{}}}",
        diagnostics.join(","),
        diagnostics.len()
    )
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

fn page_layer_tree_json(core: &DocumentCore, lines: &[PageTextLine], profile: &str) -> String {
    let mut output = format!(
        "{{\"schemaVersion\":1,\"schemaMinorVersion\":0,\"schema\":{{\"major\":1,\"minor\":0}},\"resourceTableVersion\":1,\"resourceTableMinorVersion\":0,\"resourceTable\":{{\"major\":1,\"minor\":0}},\"unit\":\"px\",\"coordinateSystem\":\"page\",\"profile\":{},\"writingMode\":\"{}\",\"writingModeDecoded\":false,\"outputOptions\":{{\"showParagraphMarks\":{},\"showControlCodes\":{},\"showTransparentBorders\":{},\"clipEnabled\":{},\"debugOverlay\":false}},\"pageWidth\":{:.1},\"pageHeight\":{:.1},\"root\":{{\"kind\":\"leaf\",\"bounds\":{{\"x\":0.0,\"y\":0.0,\"width\":{:.1},\"height\":{:.1}}},\"ops\":[",
        json_string(profile),
        core.writing_mode.as_str(),
        core.show_paragraph_marks,
        core.show_control_codes,
        core.show_transparent_borders,
        core.clip_enabled,
        APP_PAGE_WIDTH_PX,
        APP_PAGE_HEIGHT_PX,
        APP_PAGE_WIDTH_PX,
        APP_PAGE_HEIGHT_PX
    );
    let mut text_sources = Vec::new();
    push_page_layer_page_background_json(&mut output);
    let mut first_op = false;

    for (line_index, line) in lines.iter().enumerate() {
        if line.text().is_empty() {
            continue;
        }

        let mut x = if core.writing_mode.is_vertical() {
            APP_PAGE_WIDTH_PX as f64
                - APP_PAGE_MARGIN_PX as f64
                - ((line_index + 1) as f64 * APP_LINE_HEIGHT_PX as f64)
        } else {
            APP_PAGE_MARGIN_PX as f64
        };
        let mut y = if core.writing_mode.is_vertical() {
            APP_PAGE_MARGIN_PX as f64
        } else {
            APP_PAGE_MARGIN_PX as f64 + line_index as f64 * APP_LINE_HEIGHT_PX as f64
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
                x,
                y,
                baseline,
                core.writing_mode,
                &fragment,
            );
            push_page_layer_text_source_json(&mut text_sources, source_id, &fragment);
            if core.writing_mode.is_vertical() {
                y += text_width_px(&fragment.text);
            } else {
                x += text_width_px(&fragment.text);
            }
        }
    }

    output.push_str("]}},\"textSources\":[");
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

fn push_page_layer_page_background_json(output: &mut String) {
    output.push_str("{\"type\":\"pageBackground\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        APP_PAGE_WIDTH_PX, APP_PAGE_HEIGHT_PX
    ));
    output.push_str(",\"backgroundColor\":\"#ffffff\"}");
}

fn push_page_layer_text_run_json(
    output: &mut String,
    source_id: usize,
    x: f64,
    y: f64,
    baseline: f64,
    writing_mode: WritingMode,
    fragment: &PageLayerTextFragment,
) {
    let (width, height) = if writing_mode.is_vertical() {
        (APP_LINE_HEIGHT_PX as f64, text_width_px(&fragment.text))
    } else {
        (text_width_px(&fragment.text), APP_LINE_HEIGHT_PX as f64)
    };
    output.push_str("{\"type\":\"textRun\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&fragment.text));
    if fragment.paragraph_index.is_some() {
        output.push_str(",\"paragraphCharRange\":");
        output.push_str(&source_range_json(fragment.char_start, fragment.char_end));
    }
    output.push_str(&format!(
        ",\"baseline\":{baseline:.3},\"rotation\":0.000,\"isVertical\":{},\"orientation\":\"{}\",\"projectionKind\":\"fallback\",\"source\":",
        writing_mode.is_vertical(),
        writing_mode.as_str()
    ));
    push_page_layer_source_span_json(output, source_id, fragment);
    output.push_str(",\"positions\":");
    push_f64_array_json(output, &text_positions_px(&fragment.text));
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
    source.push_str(",\"annotations\":[]}");
    output.push(source);
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
        }];
    };

    let Some(paragraph) = paragraph_by_index(document, paragraph_index) else {
        return Vec::new();
    };
    paragraph_line_fragments(
        paragraph,
        paragraph_index,
        line.char_start(),
        line.char_end(),
    )
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
        let (text, source_span) = match inline {
            Inline::Text(run) => (run.text(), run.source_span()),
            Inline::Ruby(ruby) => (ruby.base_text(), None),
            Inline::Unknown(_) => ("", None),
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
        fragments.push(PageLayerTextFragment {
            text: text_by_char_range(text, relative_start, relative_end),
            paragraph_index: Some(paragraph_index),
            char_start: overlap_start,
            char_end: overlap_end,
            source_span: source_span
                .map(|span| source_span_for_char_range(text, span, relative_start, relative_end)),
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

fn text_width_px(text: &str) -> f64 {
    text.chars()
        .map(|character| display_column_width(character) as f64 * column_width_px())
        .sum()
}

fn text_positions_px(text: &str) -> Vec<f64> {
    let mut positions = Vec::new();
    let mut x = 0.0;
    positions.push(x);
    for character in text.chars() {
        x += display_column_width(character) as f64 * column_width_px();
        positions.push(x);
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
    page_count: usize,
    writing_mode: WritingMode,
) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{APP_PAGE_WIDTH_PX}\" height=\"{APP_PAGE_HEIGHT_PX}\" viewBox=\"0 0 {APP_PAGE_WIDTH_PX} {APP_PAGE_HEIGHT_PX}\">"
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>");

    if writing_mode.is_vertical() {
        svg.push_str("<g writing-mode=\"vertical-rl\" glyph-orientation-vertical=\"auto\">");
        for (index, line) in lines.iter().enumerate() {
            let x = APP_PAGE_WIDTH_PX - APP_PAGE_MARGIN_PX - (index as f32 * APP_LINE_HEIGHT_PX);
            svg.push_str(&format!(
                "<text x=\"{x}\" y=\"{APP_PAGE_MARGIN_PX}\" font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Yu Gothic, Meiryo, Noto Sans CJK JP, sans-serif\" font-size=\"{APP_FONT_SIZE_PX}\" fill=\"#111111\" letter-spacing=\"0\" writing-mode=\"vertical-rl\">{}</text>",
                escape_xml(line.text())
            ));
        }
        svg.push_str("</g>");
    } else {
        svg.push_str(&format!(
            "<text x=\"{APP_PAGE_MARGIN_PX}\" y=\"{}\" font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Yu Gothic, Meiryo, Noto Sans CJK JP, sans-serif\" font-size=\"{APP_FONT_SIZE_PX}\" fill=\"#111111\" letter-spacing=\"0\">",
            APP_PAGE_MARGIN_PX + APP_FONT_SIZE_PX
        ));

        for (index, line) in lines.iter().enumerate() {
            let y = APP_PAGE_MARGIN_PX + APP_FONT_SIZE_PX + (index as f32 * APP_LINE_HEIGHT_PX);
            svg.push_str(&format!(
                "<tspan x=\"{APP_PAGE_MARGIN_PX}\" y=\"{y}\">{}</tspan>",
                escape_xml(line.text())
            ));
        }

        svg.push_str("</text>");
    }
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"10\" fill=\"#777777\" letter-spacing=\"0\">{}/{}</text>",
        APP_PAGE_WIDTH_PX - APP_PAGE_MARGIN_PX,
        APP_PAGE_HEIGHT_PX - 36.0,
        page_number,
        page_count
    ));
    svg.push_str("</svg>");
    svg
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
    use std::{
        collections::HashSet,
        fs,
        io::{Cursor, Write},
        path::PathBuf,
    };

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
        assert!(svg.contains("1/1"));

        let lines = core.page_text_lines(0).unwrap();
        assert_eq!(lines[0].text(), "銀河鉄道");
        assert_eq!(lines[0].paragraph_index(), Some(0));
        assert_eq!(lines[0].char_start(), 0);
        assert_eq!(lines[0].char_end(), 4);
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
        ]);

        let document = parse_document(&bytes).unwrap();

        assert_eq!(document.object_stream_candidates().len(), 4);
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
        assert!(
            info.contains(
                "\"rule\":\"control-delimited-text-count-range-with-multiple-intervals\""
            )
        );
        assert_eq!(
            core.get_table_dimensions(0, 0, 0).unwrap(),
            "{\"rowCount\":0,\"colCount\":0,\"cellCount\":0}"
        );

        let warnings = core.get_validation_warnings();
        assert!(warnings.contains("\"JTD table candidate preserved as diagnostic data\":2"));
        assert!(warnings.contains("\"kind\":\"JtdTableCandidateDiagnosticOnly\""));
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

    fn minimal_jpeg_payload() -> &'static [u8] {
        &[
            0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00,
            0x10, 0x00, 0x20, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff,
            0xda, 0x00, 0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x00,
            0xff, 0xd9,
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
