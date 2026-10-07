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
    DocumentTextMapKind, DocumentTextPayload, InlineTextSegment, ParsedDocumentText,
    SkippedInlineTextSegment, map_document_text, read_document_text_payload_with_budget,
};
use rjtd_core::document_text_position::{
    DocumentTextCountEntry, read_document_text_position_tables,
};
use rjtd_core::font_stream::{FontEntry, read_font_stream_with_budget};
use rjtd_core::layout_mark::{
    PAGE_MARK_PATH, PAPER_MARK_PATH, PageMark, PaperMark, read_page_mark, read_paper_mark,
};
use rjtd_core::record::UnknownRecordKind;
use rjtd_core::style_stream::{PAGE_LAYOUT_STYLE_PATH, read_style_streams_with_budget};
use rjtd_core::{Error, ParseLimits, ResourceBudget, Result};

#[cfg(feature = "rendering")]
mod app;
#[cfg(feature = "rendering")]
mod rendering;

mod block_text_model;
mod diagnostic_layout_types;
mod document_metadata;
mod document_text;
mod document_text_control_layout;
mod document_text_text_style;
#[cfg(feature = "rendering")]
mod embedded_press;
mod embedded_press_source;
#[cfg(feature = "rendering")]
mod embedded_press_title_art;
mod fdm;
mod json_export_helpers;
mod marks;
mod object_embedded_press_model;
mod object_media;
mod object_stream;
#[cfg(feature = "rendering")]
mod page_layout;
mod page_source;
mod parse;
#[cfg(feature = "rendering")]
mod shanai_lan;
#[cfg(feature = "rendering")]
mod shanai_lan_sparse_borders;
#[cfg(feature = "rendering")]
mod success_data_test;
#[cfg(feature = "rendering")]
mod success_data_test_answer_sheet_geometry;
#[cfg(feature = "rendering")]
mod success_data_test_placement_diagnostics;
mod table_candidate;
mod table_grid;
#[cfg(feature = "rendering")]
mod table_grid_diagnostics;
#[cfg(feature = "rendering")]
mod table_grid_render_projection;
mod table_text_candidate_model;

#[cfg(feature = "rendering")]
pub use app::DocumentCore;
#[cfg(feature = "rendering")]
use app::*;
#[cfg(feature = "rendering")]
use rendering::*;
#[cfg(feature = "rendering")]
pub use rendering::{PageLayout, PageTextLine};

pub use parse::{parse_document, parse_document_with_budget, parse_document_with_limits};

use block_text_model::*;
pub use block_text_model::{
    Block, Inline, StyleRef, TextRun, UnknownBlock, UnknownObject, UnknownStyle,
};
use diagnostic_layout_types::*;
#[cfg(feature = "rendering")]
use document_metadata::*;
pub use document_metadata::{
    DocumentAutoText, DocumentFont, DocumentPageMark, DocumentPageMarkEntry, DocumentPaperMark,
    DocumentPaperMarkEntry, DocumentTocEntry, Metadata, PageMarkU16GeometryProfile, RawStream,
};
pub use document_text::*;
use document_text_control_layout::*;
#[cfg(feature = "rendering")]
use embedded_press::*;
use embedded_press_source::*;
#[cfg(feature = "rendering")]
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
#[cfg(feature = "rendering")]
use page_layout::*;
use page_source::*;
#[cfg(feature = "rendering")]
use shanai_lan::*;
#[cfg(feature = "rendering")]
use success_data_test::*;
#[cfg(feature = "rendering")]
use success_data_test_answer_sheet_geometry::*;
#[cfg(feature = "rendering")]
use success_data_test_placement_diagnostics::*;
use table_candidate::*;
#[cfg(feature = "rendering")]
use table_grid::*;
#[cfg(feature = "rendering")]
use table_grid_diagnostics::*;
#[cfg(feature = "rendering")]
use table_grid_render_projection::*;
use table_text_candidate_model::*;
pub use table_text_candidate_model::{
    TableCandidate, TableCandidateColumnGridCandidate, TableCandidateColumnSegment,
    TableCandidateColumnSegmentKind, TableCandidateInterval, TableCandidateSparseTopologyCandidate,
    TableCandidateSparseTopologyColumn, TableCandidateSparseTopologyRow, TextBoundaryCandidate,
    TextControlBoundary, TextCountControlRangeOverlap, TextCountRange, TextCountRangeOverlap,
    TextCountRangeOverlapBasis, TextLayoutExactEvidence, TextSourceSpan,
};

#[cfg(feature = "rendering")]
use document_text_text_style::document_text_style_resolver;
#[cfg(feature = "rendering")]
use document_text_text_style::{
    DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS, DocumentTextCharacterStyle, DocumentTextFontSize,
    DocumentTextProperty15ColorCandidate, document_default_font_size_px,
    document_text_character_style, document_text_font_size, document_text_foreground_color,
    document_text_property_15_color_candidate,
};
#[cfg(all(test, feature = "rendering"))]
use rjtd_core::document_text::read_document_text_payload;
#[cfg(all(test, feature = "rendering"))]
use shanai_lan_sparse_borders::shanai_lan_source_page_transform_candidate_from_raw_fields;
#[cfg(feature = "rendering")]
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
    document_text_flow: Option<DocumentTextFlow>,
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
            document_text_flow: None,
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
        Self::from_document_text_payload_with_map(payload, &map)
    }

    fn from_document_text_payload_with_map(
        payload: &DocumentTextPayload,
        map: &DocumentTextMap,
    ) -> Self {
        let mut spans = DocumentTextSourceSpans::new(map.entries());
        let mut builder = DocumentTextModelBuilder::default();
        let flow = DocumentTextFlow::from_map(payload.source_name(), payload.bytes(), map);
        let tatechuyoko = native_tatechuyoko_candidates(&flow);

        for element in payload.parsed_text().elements() {
            match element {
                DocumentTextElement::TextRun(text) => builder
                    .push_text_run_with_span(text, spans.next(DocumentTextMapKind::TextRun, text)),
                DocumentTextElement::InlineText(segment) => builder.push_inline_text_with_span(
                    segment,
                    spans.next(DocumentTextMapKind::InlineText, segment.text()),
                ),
                DocumentTextElement::SkippedInlineText(segment) => {
                    let span = spans.next(DocumentTextMapKind::SkippedInlineText, segment.text());
                    builder.push_skipped_inline_with_span(segment, span.clone());
                    if let Some(candidate) = tatechuyoko.iter().find(|c| {
                        span.as_ref().is_some_and(|span| {
                            c.value_span().unit_start() == span.unit_start() + 1
                                && c.value_span().unit_end() + 1 == span.unit_end()
                        }) && c.text() == segment.text()
                    }) {
                        builder.push_text(
                            candidate.text(),
                            ModelTextSource::Inline,
                            Some(candidate.value_span().clone()),
                        );
                    }
                }
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
        document.document_text_flow = Some(flow);
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

    pub fn document_text_flow(&self) -> Option<&DocumentTextFlow> {
        self.document_text_flow.as_ref()
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
        let source_container = payload.decompressed_container().unwrap_or(data);
        if payload.decompressed_container().is_some() {
            reserve_and_verify_cfb_streams(source_container, budget)?;
        }
        let map = map_document_text(payload.bytes());
        let mut document = Document::from_document_text_payload_with_map(&payload, &map);
        for entry in document_text_toc_entries(map.entries()) {
            document.push_toc_entry(entry);
        }
        document.push_raw_stream(RawStream::new(
            payload.source_name(),
            payload.bytes().to_vec(),
        ));
        if payload.decompressed_container().is_some()
            && let Ok(stream) =
                read_cfb_stream(data, rjtd_core::document_text::COMPRESSED_DOCUMENT_PATH)
        {
            document.push_raw_stream(RawStream::new(
                rjtd_core::document_text::COMPRESSED_DOCUMENT_PATH,
                stream,
            ));
        }
        if document.toc_entries().is_empty() {
            for entry in native_toc_cached_entries(&document) {
                document.push_toc_entry(entry);
            }
        }
        if let Ok(line_mark) = read_cfb_stream(source_container, LINE_MARK_PATH) {
            document.push_raw_stream(RawStream::new(LINE_MARK_PATH, line_mark));
        }
        for stream_name in [
            PAGE_MARK_PATH,
            PAPER_MARK_PATH,
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            LAYOUT_BOX_PATH,
            LAYOUT_BOX_TEXT_PATH,
            LAYOUT_BOX_TEXT_POSITION_TABLES_PATH,
            "/Footnote",
            "/FootnoteLink",
            "/MarkTag",
            rjtd_core::header_stream::HEADER_PATH,
            "/Frame",
            "/Figure",
            "/FigureData/main_data/FDMVector",
            "/FigureData/main_data/FDMIndex",
            "/FigureData/main_data/AlphaBlend",
            "/EmbedItems/EmbeddingInfo",
            "/EmbedItems/Embedding 1/JSEQ3Contents",
            "/EmbedItems/Embedding 1/\x03EmbeddedPress",
        ] {
            if let Ok(stream) = read_cfb_stream(source_container, stream_name) {
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
        if let Ok(auto_text_info) = read_auto_text_info(source_container) {
            for entry in auto_text_info.entries() {
                document.push_auto_text(DocumentAutoText::from_auto_text_entry(
                    auto_text_info.name(),
                    entry,
                ));
            }
        }
        if let Ok(page_mark) = read_page_mark(source_container) {
            document.push_page_mark(DocumentPageMark::from_page_mark(PAGE_MARK_PATH, &page_mark));
        }
        if let Ok(paper_mark) = read_paper_mark(source_container) {
            document.push_paper_mark(DocumentPaperMark::from_paper_mark(
                PAPER_MARK_PATH,
                &paper_mark,
            ));
        }
        for candidate in object_stream_candidates_from_cfb(source_container, budget)? {
            document.push_object_stream_candidate(candidate);
        }
        let object_frame_records = object_frame_records_from_cfb(source_container, budget)?;
        for record in object_frame_records {
            document.push_object_frame_record(record);
        }
        let object_embedding_frames = object_embedding_frames_from_cfb(source_container, budget)?;
        for frame in object_embedding_frames {
            document.push_object_embedding_frame(frame);
        }
        if let Ok(position_tables) = read_document_text_position_tables(source_container) {
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
            for candidate in text_paragraph_boundary_candidates_from_layout(
                &document,
                map.entries(),
                source_container,
            ) {
                document.push_text_paragraph_boundary_candidate(candidate);
            }
        }
        if let Some(flow) = document.document_text_flow() {
            let start = document.table_candidates().len();
            let mut projections = table_candidates_from_document_text_controls(flow, start);
            projections.extend(sparse_table_candidates_from_document_text_controls(
                flow,
                start + projections.len(),
            ));
            for candidate in projections {
                document.push_table_candidate(candidate);
            }
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

#[cfg(test)]
mod tests;

pub use document_text_text_style::{DocumentSourceCharacterStyleCandidate, DocumentSourceFontSize};

#[cfg(feature = "rendering")]
use rjtd_core::document_text::{DocumentTextStyleResolver, parse_document_text_row_headers};
#[cfg(feature = "rendering")]
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, StyleStreamRecordSummary, StyleStreamSubrecordSummary,
    TEXT_LAYOUT_STYLE_PATH, summarize_style_stream,
};

/// Version of the model crate, independently of application rendering.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
