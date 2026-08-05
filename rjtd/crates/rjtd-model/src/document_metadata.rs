use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Metadata {
    pub(crate) title: Option<String>,
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
    pub(crate) name: String,
    pub(crate) bytes: Vec<u8>,
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
    pub(crate) source_stream: String,
    pub(crate) offset: usize,
    pub(crate) text: String,
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
    pub(crate) title: String,
    pub(crate) page_label: String,
    pub(crate) source_span: TextSourceSpan,
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
    pub(crate) source_stream: String,
    pub(crate) family: String,
    pub(crate) header_count: u32,
    pub(crate) header_stride: u32,
    pub(crate) header_last_index: u32,
    pub(crate) entries: Vec<DocumentPageMarkEntry>,
    pub(crate) trailing_byte_len: usize,
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
    pub(crate) row_index: usize,
    pub(crate) index: Option<u32>,
    pub(crate) flags: Option<u32>,
    pub(crate) line_start: Option<u32>,
    pub(crate) line_end: Option<u32>,
    pub(crate) raw: Vec<u8>,
    pub(crate) u16_fields: Vec<u16>,
    pub(crate) u32_fields: Vec<u32>,
}

impl DocumentPageMarkEntry {
    pub(crate) fn from_entry(
        row_index: usize,
        entry: &rjtd_core::layout_mark::PageMarkEntry,
    ) -> Self {
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

pub(crate) fn u16_fields_be(raw: &[u8]) -> Vec<u16> {
    raw.chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageMarkU16SubrecordCandidate {
    pub(crate) word_index: usize,
    pub(crate) byte_offset: usize,
    pub(crate) words: [u16; 8],
}

impl PageMarkU16SubrecordCandidate {
    pub(crate) fn word_index(self) -> usize {
        self.word_index
    }

    pub(crate) fn byte_offset(self) -> usize {
        self.byte_offset
    }

    pub(crate) fn words(self) -> [u16; 8] {
        self.words
    }

    pub(crate) fn u32_fields(self) -> [u32; 4] {
        page_mark_u16_subrecord_u32_fields(&self.words)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageMarkU16GeometryProfile {
    pub(crate) selected_fields_all_zero: bool,
    pub(crate) non_zero_additive_unit_candidate: bool,
    pub(crate) word20_is_00ff: bool,
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
    pub(crate) source_stream: String,
    pub(crate) header_count: u32,
    pub(crate) header_stride: u32,
    pub(crate) header_last_index: u32,
    pub(crate) entries: Vec<DocumentPaperMarkEntry>,
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
    pub(crate) row_index: usize,
    pub(crate) index: u32,
    pub(crate) flags: u32,
}

impl DocumentPaperMarkEntry {
    pub(crate) fn from_entry(
        row_index: usize,
        entry: rjtd_core::layout_mark::PaperMarkEntry,
    ) -> Self {
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
    pub(crate) source_stream: String,
    pub(crate) id: u16,
    pub(crate) offset: usize,
    pub(crate) name: String,
    pub(crate) raw: Vec<u8>,
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
