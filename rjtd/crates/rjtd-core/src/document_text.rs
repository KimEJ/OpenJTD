use crate::compressed_document::{
    decompress_just_compressed_document, is_just_compressed_document,
};
use crate::container::read_cfb_stream;
use crate::{Error, Result};

pub const DOCUMENT_TEXT_PATH: &str = "/DocumentText";
pub const COMPRESSED_DOCUMENT_PATH: &str = "/JSCompDocument";
pub const EMBEDDED_DOCUMENT_TEXT_PATH: &str = "/EmbeddedDocumentText";
const DOCUMENT_TEXT_MAGIC: &[u8; 8] = b"SsmgV.01";
const EMBEDDED_DOCUMENT_TEXT_MAX_SPAN: usize = 64 * 1024;
const TEXT_RUN_MARKER: u16 = 0x001f;
const INLINE_TEXT_START: u16 = 0x001d;
const INLINE_TEXT_END: u16 = 0x001e;
// 0x000e separates 0x001c/0x0030 table-cell records; reading_text stays true across it.
// 0x000a is a within-cell/intra-paragraph line break (see RFC 0009); treated as a plain
// text character ('\n') by is_control_boundary, which intentionally excludes 0x09/0x0a/0x0d.
const TEXT_ROW_DELIMITER: u16 = 0x000e;
const SKIPPED_INLINE_MAX_UNITS: usize = 256;

// RFC 0009: 0x001c record class codes (decoded:false — structure proven, semantics partial)
pub const RECORD_CLASS_INLINE_CONTEXT: u16 = 0x0000;
pub const RECORD_CLASS_PARAGRAPH_LINE: u16 = 0x0010;
pub const RECORD_CLASS_TABLE_SECTION_TRANSITION: u16 = 0x0020;
pub const RECORD_CLASS_TABLE_CELL: u16 = 0x0030;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedDocumentText {
    elements: Vec<DocumentTextElement>,
}

impl ParsedDocumentText {
    fn new(elements: Vec<DocumentTextElement>) -> Self {
        Self { elements }
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        let text = text.into();
        if text.is_empty() {
            Self::default()
        } else {
            Self::new(vec![DocumentTextElement::TextRun(text)])
        }
    }

    pub fn elements(&self) -> &[DocumentTextElement] {
        &self.elements
    }

    pub fn plain_text(&self) -> String {
        let mut output = String::new();
        for element in &self.elements {
            match element {
                DocumentTextElement::TextRun(text) => output.push_str(text),
                DocumentTextElement::InlineText(segment) => output.push_str(segment.text()),
                DocumentTextElement::SkippedInlineText(_) => {}
                DocumentTextElement::ControlBoundary(_) => {}
            }
        }
        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentTextElement {
    TextRun(String),
    InlineText(InlineTextSegment),
    SkippedInlineText(SkippedInlineTextSegment),
    ControlBoundary(DocumentTextControl),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DocumentTextMap {
    entries: Vec<DocumentTextMapEntry>,
}

impl DocumentTextMap {
    fn new(entries: Vec<DocumentTextMapEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[DocumentTextMapEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextMapEntry {
    byte_start: usize,
    byte_end: usize,
    unit_start: usize,
    unit_end: usize,
    kind: DocumentTextMapKind,
    selector: Option<u16>,
    code: Option<u16>,
    text: String,
}

impl DocumentTextMapEntry {
    fn new(
        unit_start: usize,
        unit_end: usize,
        kind: DocumentTextMapKind,
        selector: Option<u16>,
        code: Option<u16>,
        text: String,
    ) -> Self {
        Self {
            byte_start: unit_start * 2,
            byte_end: unit_end * 2,
            unit_start,
            unit_end,
            kind,
            selector,
            code,
            text,
        }
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

    pub fn kind(&self) -> DocumentTextMapKind {
        self.kind
    }

    pub fn selector(&self) -> Option<u16> {
        self.selector
    }

    pub fn code(&self) -> Option<u16> {
        self.code
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn contains_byte_offset(&self, offset: usize) -> bool {
        self.byte_start <= offset && offset < self.byte_end
    }

    pub fn contains_unit_offset(&self, offset: usize) -> bool {
        self.unit_start <= offset && offset < self.unit_end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentTextMapKind {
    TextRun,
    InlineText,
    SkippedInlineText,
    ControlBoundary,
}

impl DocumentTextMapKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TextRun => "text",
            Self::InlineText => "inline",
            Self::SkippedInlineText => "skipped-inline",
            Self::ControlBoundary => "control",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineTextSegment {
    selector: u16,
    text: String,
}

impl InlineTextSegment {
    fn new(selector: u16, text: String) -> Self {
        Self { selector, text }
    }

    pub fn selector(&self) -> u16 {
        self.selector
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedInlineTextSegment {
    context: Vec<u16>,
    text: String,
    raw_bytes: Vec<u8>,
}

impl SkippedInlineTextSegment {
    fn new(context: Vec<u16>, text: String, raw_bytes: Vec<u8>) -> Self {
        Self {
            context,
            text,
            raw_bytes,
        }
    }

    pub fn selector(&self) -> Option<u16> {
        self.context.last().copied()
    }

    pub fn context(&self) -> &[u16] {
        &self.context
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentTextControl {
    code: u16,
}

impl DocumentTextControl {
    fn new(code: u16) -> Self {
        Self { code }
    }

    pub fn code(&self) -> u16 {
        self.code
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextPayload {
    source_name: String,
    bytes: Vec<u8>,
    parsed_text: ParsedDocumentText,
    text: String,
}

impl DocumentTextPayload {
    fn new(
        source_name: impl Into<String>,
        bytes: Vec<u8>,
        parsed_text: ParsedDocumentText,
    ) -> Self {
        let text = parsed_text.plain_text();
        Self {
            source_name: source_name.into(),
            bytes,
            parsed_text,
            text,
        }
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn parsed_text(&self) -> &ParsedDocumentText {
        &self.parsed_text
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

pub fn read_document_text_payload(data: &[u8]) -> Result<DocumentTextPayload> {
    match read_cfb_stream(data, DOCUMENT_TEXT_PATH) {
        Ok(stream) => Ok(DocumentTextPayload::new(
            DOCUMENT_TEXT_PATH,
            stream.clone(),
            parse_document_text(&stream),
        )),
        Err(Error::NotFound(_)) => read_compressed_or_embedded_document_text(data),
        Err(error) => Err(error),
    }
}

pub fn read_document_text_stream(data: &[u8]) -> Result<Vec<u8>> {
    Ok(read_document_text_payload(data)?.bytes)
}

fn read_compressed_or_embedded_document_text(data: &[u8]) -> Result<DocumentTextPayload> {
    let stream = match read_cfb_stream(data, COMPRESSED_DOCUMENT_PATH) {
        Ok(stream) => stream,
        Err(Error::NotFound(_)) => return read_embedded_document_text(data),
        Err(error) => return Err(error),
    };

    if !is_just_compressed_document(&stream) {
        return read_embedded_document_text(data);
    }

    let inner_document = decompress_just_compressed_document(&stream)?;
    let bytes = read_cfb_stream(&inner_document, DOCUMENT_TEXT_PATH)?;
    let parsed_text = parse_document_text(&bytes);
    Ok(DocumentTextPayload::new(
        DOCUMENT_TEXT_PATH,
        bytes,
        parsed_text,
    ))
}

pub fn has_embedded_document_text(data: &[u8]) -> bool {
    embedded_document_text(data).is_some()
}

fn read_embedded_document_text(data: &[u8]) -> Result<DocumentTextPayload> {
    embedded_document_text(data)
        .ok_or_else(|| Error::NotFound(format!("stream `{DOCUMENT_TEXT_PATH}`")))
}

fn embedded_document_text(data: &[u8]) -> Option<DocumentTextPayload> {
    let offsets = find_document_text_magic_offsets(data);
    let mut bytes = Vec::new();
    let mut text_parts = Vec::new();

    for (index, start) in offsets.iter().copied().enumerate() {
        let next_start = offsets.get(index + 1).copied().unwrap_or(data.len());
        let end = next_start.min(start.saturating_add(EMBEDDED_DOCUMENT_TEXT_MAX_SPAN));
        if end <= start {
            continue;
        }
        let fragment = &data[start..end];
        let text = clean_embedded_text(&extract_document_text(fragment));
        if text.trim().is_empty() || text_parts.iter().any(|part| part == &text) {
            continue;
        }

        if !bytes.is_empty() {
            bytes.extend_from_slice(&[0, 0]);
        }
        bytes.extend_from_slice(fragment);
        text_parts.push(text);
    }

    if text_parts.is_empty() {
        None
    } else {
        Some(DocumentTextPayload::new(
            EMBEDDED_DOCUMENT_TEXT_PATH,
            bytes,
            ParsedDocumentText::from_text(text_parts.join("\n")),
        ))
    }
}

fn clean_embedded_text(text: &str) -> String {
    text.split(['\r', '\n'])
        .filter_map(|line| {
            let line = line.trim_matches('\0');
            if line.trim().is_empty() || !is_plausible_embedded_line(line) {
                None
            } else {
                Some(line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_plausible_embedded_line(line: &str) -> bool {
    let mut total = 0usize;
    let mut plausible = 0usize;

    for character in line.chars().filter(|character| !character.is_whitespace()) {
        total += 1;
        if is_plausible_embedded_character(character) {
            plausible += 1;
        }
    }

    total > 0 && plausible * 100 >= total * 80
}

fn is_plausible_embedded_character(character: char) -> bool {
    character.is_ascii_graphic()
        || matches!(
            character as u32,
            0x3000..=0x30ff
                | 0x31f0..=0x31ff
                | 0x3200..=0x33ff
                | 0x4e00..=0x9fff
                | 0xff00..=0xffef
        )
        || matches!(
            character,
            '、' | '。'
                | '・'
                | '「'
                | '」'
                | '『'
                | '』'
                | '【'
                | '】'
                | '（'
                | '）'
                | '［'
                | '］'
                | '→'
                | '←'
                | '↑'
                | '↓'
                | '～'
                | '…'
                | '◎'
                | '○'
                | '●'
                | '◆'
                | '☆'
                | '★'
                | '※'
        )
}

fn find_document_text_magic_offsets(data: &[u8]) -> Vec<usize> {
    data.windows(DOCUMENT_TEXT_MAGIC.len())
        .enumerate()
        .filter_map(|(offset, window)| (window == DOCUMENT_TEXT_MAGIC).then_some(offset))
        .collect()
}

pub fn extract_document_text(data: &[u8]) -> String {
    parse_document_text(data).plain_text()
}

// SsmgV.01 segment-count field: w[9]=0x0001 means a single raw-text TextV.01 segment
// with no paragraph records; w[9]=0x0002 is the normal paragraph-record format.
const SSMG_RAW_TEXT_SEGMENT_COUNT: u16 = 0x0001;
const SSMG_HEADER_WORDS: usize = 10; // SsmgV.01 (4) + header (4) + segment-count (2)
const TEXT_SEGMENT_NAME: &[u8; 8] = b"TextV.01";

pub fn parse_document_text(data: &[u8]) -> ParsedDocumentText {
    // SsmgV.01 w[9]=0x0001: single raw-text segment (no 0x001f paragraph markers).
    // Layout: SsmgV.01 header (10 words) + TextV.01 name (4 words) + length (2 words) + text.
    if data.starts_with(DOCUMENT_TEXT_MAGIC) {
        let units: Vec<u16> = data
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        if units.get(9) == Some(&SSMG_RAW_TEXT_SEGMENT_COUNT)
            && data.get(SSMG_HEADER_WORDS * 2..)
                .is_some_and(|rest| rest.starts_with(TEXT_SEGMENT_NAME))
        {
            return parse_raw_text_segment(&units);
        }
    }

    let units = data
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<_>>();
    let mut elements = Vec::new();
    let mut run = String::new();
    let mut reading_text = false;
    let mut index = 0;

    while index < units.len() {
        let code = units[index];
        if code == TEXT_RUN_MARKER {
            push_run(&mut elements, &mut run);
            reading_text = true;
            index += 1;
            continue;
        }

        if code == INLINE_TEXT_START {
            push_run(&mut elements, &mut run);
            reading_text = false;
            if let Some(selector) = inline_text_selector(&units, index) {
                index = push_inline_segment(&mut elements, &units, index, selector);
            } else if let Some((segment, next_index)) = read_skipped_inline_segment(&units, index) {
                elements.push(DocumentTextElement::SkippedInlineText(segment));
                index = next_index;
            } else {
                elements.push(DocumentTextElement::ControlBoundary(
                    DocumentTextControl::new(code),
                ));
                index += 1;
            }
            continue;
        }

        if reading_text {
            if is_control_boundary(code) || is_invalid_scalar(code) {
                push_run(&mut elements, &mut run);
                elements.push(DocumentTextElement::ControlBoundary(
                    DocumentTextControl::new(code),
                ));
                reading_text = code == TEXT_ROW_DELIMITER;
            } else if let Some(character) = char::from_u32(code as u32) {
                run.push(character);
            }
        }

        index += 1;
    }

    push_run(&mut elements, &mut run);
    ParsedDocumentText::new(elements)
}

pub fn map_document_text(data: &[u8]) -> DocumentTextMap {
    let units = data
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<_>>();
    let mut entries = Vec::new();
    let mut run = String::new();
    let mut run_start = 0usize;
    let mut reading_text = false;
    let mut index = 0;

    while index < units.len() {
        let code = units[index];
        if code == TEXT_RUN_MARKER {
            push_map_run(&mut entries, &mut run, run_start, index);
            reading_text = true;
            index += 1;
            continue;
        }

        if code == INLINE_TEXT_START {
            push_map_run(&mut entries, &mut run, run_start, index);
            reading_text = false;
            if let Some(selector) = inline_text_selector(&units, index) {
                index = push_mapped_inline_segment(&mut entries, &units, index, selector);
            } else if let Some((segment, next_index)) = read_skipped_inline_segment(&units, index) {
                entries.push(DocumentTextMapEntry::new(
                    index,
                    next_index,
                    DocumentTextMapKind::SkippedInlineText,
                    segment.selector(),
                    None,
                    segment.text().to_string(),
                ));
                index = next_index;
            } else {
                push_map_control(&mut entries, index, code);
                index += 1;
            }
            continue;
        }

        if reading_text {
            if is_control_boundary(code) || is_invalid_scalar(code) {
                push_map_run(&mut entries, &mut run, run_start, index);
                push_map_control(&mut entries, index, code);
                reading_text = code == TEXT_ROW_DELIMITER;
            } else if let Some(character) = char::from_u32(code as u32) {
                if run.is_empty() {
                    run_start = index;
                }
                run.push(character);
            }
        }

        index += 1;
    }

    push_map_run(&mut entries, &mut run, run_start, units.len());
    DocumentTextMap::new(entries)
}

// Parse a SsmgV.01 w[9]=0x0001 raw-text segment: TextV.01 header (14..16) gives
// the word count, then the text follows as plain UTF-16BE with no 0x001f markers.
fn parse_raw_text_segment(units: &[u16]) -> ParsedDocumentText {
    // Layout: SSMG_HEADER_WORDS=10 + TextV.01 name (4) + length field (2) = 16 words header
    const HEADER_WORDS: usize = SSMG_HEADER_WORDS + 4 + 2;
    let length = match units.get(SSMG_HEADER_WORDS + 4 + 1) {
        Some(&len) => len as usize,
        None => return ParsedDocumentText::default(),
    };
    let text_start = HEADER_WORDS;
    let text_end = text_start.saturating_add(length).min(units.len());
    let mut run = String::new();
    for &code in &units[text_start..text_end] {
        if code == 0x0000 {
            break;
        }
        if !is_invalid_scalar(code)
            && let Some(character) = char::from_u32(code as u32)
        {
            run.push(character);
        }
    }
    if run.is_empty() {
        ParsedDocumentText::default()
    } else {
        ParsedDocumentText::new(vec![DocumentTextElement::TextRun(run)])
    }
}

fn push_run(elements: &mut Vec<DocumentTextElement>, run: &mut String) {
    if !run.is_empty() {
        elements.push(DocumentTextElement::TextRun(std::mem::take(run)));
        run.clear();
    }
}

fn is_control_boundary(code: u16) -> bool {
    (code < 0x20 && !matches!(code, 0x09 | 0x0a | 0x0d)) || (0x7f..=0x9f).contains(&code)
}

fn is_invalid_scalar(code: u16) -> bool {
    (0xd800..=0xdfff).contains(&code) || code == 0xffff
}

fn inline_text_selector(units: &[u16], index: usize) -> Option<u16> {
    if index < 6 {
        return None;
    }

    let context = &units[index - 6..index];
    if context[..5] == [0x001c, 0x0001, 0x0007, 0x0000, 0x0000]
        && matches!(context[5], 0x0001 | 0x0003 | 0x0013)
    {
        Some(context[5])
    } else {
        None
    }
}

fn push_inline_segment(
    elements: &mut Vec<DocumentTextElement>,
    units: &[u16],
    start: usize,
    selector: u16,
) -> usize {
    let mut index = start + 1;
    let mut text = String::new();
    while index < units.len() {
        let code = units[index];
        if code == INLINE_TEXT_END {
            if !text.is_empty() {
                elements.push(DocumentTextElement::InlineText(InlineTextSegment::new(
                    selector, text,
                )));
            }
            return index + 1;
        }

        if !is_control_boundary(code)
            && !is_invalid_scalar(code)
            && let Some(character) = char::from_u32(code as u32)
        {
            text.push(character);
        }
        index += 1;
    }

    if !text.is_empty() {
        elements.push(DocumentTextElement::InlineText(InlineTextSegment::new(
            selector, text,
        )));
    }
    index
}

fn push_mapped_inline_segment(
    entries: &mut Vec<DocumentTextMapEntry>,
    units: &[u16],
    start: usize,
    selector: u16,
) -> usize {
    let mut index = start + 1;
    let mut text = String::new();
    while index < units.len() {
        let code = units[index];
        if code == INLINE_TEXT_END {
            entries.push(DocumentTextMapEntry::new(
                start,
                index + 1,
                DocumentTextMapKind::InlineText,
                Some(selector),
                None,
                text,
            ));
            return index + 1;
        }

        if !is_control_boundary(code)
            && !is_invalid_scalar(code)
            && let Some(character) = char::from_u32(code as u32)
        {
            text.push(character);
        }
        index += 1;
    }

    entries.push(DocumentTextMapEntry::new(
        start,
        index,
        DocumentTextMapKind::InlineText,
        Some(selector),
        None,
        text,
    ));
    index
}

fn push_map_run(
    entries: &mut Vec<DocumentTextMapEntry>,
    run: &mut String,
    run_start: usize,
    run_end: usize,
) {
    if !run.is_empty() {
        entries.push(DocumentTextMapEntry::new(
            run_start,
            run_end,
            DocumentTextMapKind::TextRun,
            None,
            None,
            std::mem::take(run),
        ));
        run.clear();
    }
}

fn push_map_control(entries: &mut Vec<DocumentTextMapEntry>, index: usize, code: u16) {
    entries.push(DocumentTextMapEntry::new(
        index,
        index + 1,
        DocumentTextMapKind::ControlBoundary,
        None,
        Some(code),
        String::new(),
    ));
}

fn read_skipped_inline_segment(
    units: &[u16],
    start: usize,
) -> Option<(SkippedInlineTextSegment, usize)> {
    if start >= units.len() || units[start] != INLINE_TEXT_START {
        return None;
    }

    let context_start = start.saturating_sub(6);
    let context = units[context_start..start].to_vec();
    let mut text = String::new();
    let mut index = start + 1;

    while index < units.len() {
        if index - start > SKIPPED_INLINE_MAX_UNITS {
            return None;
        }

        let code = units[index];
        if code == INLINE_TEXT_END {
            let raw_bytes = units_to_be_bytes(&units[context_start..=index]);
            return Some((
                SkippedInlineTextSegment::new(context, text, raw_bytes),
                index + 1,
            ));
        }

        if !is_control_boundary(code)
            && !is_invalid_scalar(code)
            && let Some(character) = char::from_u32(code as u32)
        {
            text.push(character);
        }
        index += 1;
    }

    None
}

fn units_to_be_bytes(units: &[u16]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(units.len() * 2);
    for unit in units {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::{
        DocumentTextElement, DocumentTextMapKind, EMBEDDED_DOCUMENT_TEXT_PATH,
        SKIPPED_INLINE_MAX_UNITS, extract_document_text, map_document_text, parse_document_text,
        read_document_text_payload, read_document_text_stream,
    };
    use crate::compressed_document::is_just_compressed_document;
    use std::io::{Cursor, Write};

    #[test]
    fn extracts_utf16be_runs_after_text_marker() {
        let mut bytes = b"SsmgV.01".to_vec();
        bytes.extend_from_slice(&[0x00, 0x1f]);
        for unit in "銀河".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes.extend_from_slice(&[0x00, 0x1c]);
        bytes.extend_from_slice(&[0x00, 0x1f]);
        for unit in "鉄道\n".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }

        assert_eq!(extract_document_text(&bytes), "銀河鉄道\n");
    }

    #[test]
    fn ignores_bytes_before_text_marker() {
        let mut bytes = vec![0x53, 0x73, 0x6d, 0x67, 0x00, 0x10, 0x00, 0x20];
        bytes.extend_from_slice(&[0x00, 0x1f]);
        for unit in "本文".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }

        assert_eq!(extract_document_text(&bytes), "本文");
    }

    #[test]
    fn treats_c1_control_codes_as_boundaries() {
        let mut bytes = vec![0x00, 0x1f];
        for unit in "目次".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes.extend_from_slice(&[0x00, 0x90]);
        for unit in "ignored".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }

        assert_eq!(extract_document_text(&bytes), "目次");
    }

    #[test]
    fn continues_text_after_row_delimiter_inside_text_run() {
        let mut bytes = vec![0x00, 0x1f, 0x00, 0x0e];
        for unit in "１，次の計算をしなさい\n".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(&mut bytes, &[0x001c]);

        assert_eq!(extract_document_text(&bytes), "１，次の計算をしなさい\n");

        let map = map_document_text(&bytes);
        assert_eq!(
            map.entries()[0].kind(),
            DocumentTextMapKind::ControlBoundary
        );
        assert_eq!(map.entries()[0].code(), Some(0x000e));
        assert_eq!(map.entries()[1].kind(), DocumentTextMapKind::TextRun);
        assert_eq!(map.entries()[1].unit_start(), 2);
        assert_eq!(map.entries()[1].text(), "１，次の計算をしなさい\n");
    }

    #[test]
    fn extracts_display_inline_segments_without_phonetic_annotations() {
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

        assert_eq!(extract_document_text(&bytes), "一、午后の授業");

        let parsed = parse_document_text(&bytes);
        let skipped = parsed
            .elements()
            .iter()
            .find_map(|element| match element {
                DocumentTextElement::SkippedInlineText(segment) => Some(segment),
                _ => None,
            })
            .expect("phonetic annotation should be preserved as skipped inline text");
        assert_eq!(skipped.selector(), Some(0x0082));
        assert_eq!(skipped.text(), "ごご");
        assert!(!skipped.raw_bytes().is_empty());
    }

    #[test]
    fn parses_document_text_into_structured_elements() {
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
        for unit in "の授業".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }

        let parsed = parse_document_text(&bytes);

        assert_eq!(parsed.plain_text(), "一、午后の授業");
        assert_eq!(parsed.elements().len(), 4);
        assert_eq!(
            parsed.elements()[0],
            DocumentTextElement::TextRun("一、".into())
        );
        match &parsed.elements()[1] {
            DocumentTextElement::ControlBoundary(control) => {
                assert_eq!(control.code(), 0x001c);
            }
            other => panic!("expected control boundary, got {other:?}"),
        }
        match &parsed.elements()[2] {
            DocumentTextElement::InlineText(segment) => {
                assert_eq!(segment.selector(), 0x0003);
                assert_eq!(segment.text(), "午后");
            }
            other => panic!("expected inline text segment, got {other:?}"),
        }
        assert_eq!(
            parsed.elements()[3],
            DocumentTextElement::TextRun("の授業".into())
        );
    }

    #[test]
    fn maps_document_text_elements_to_byte_and_unit_ranges() {
        let mut bytes = b"SsmgV.01".to_vec();
        bytes.extend_from_slice(&[0x00, 0x1f]);
        for unit in "銀河".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes.extend_from_slice(&[0x00, 0x1c]);
        bytes.extend_from_slice(&[0x00, 0x1f]);
        for unit in "鉄道\n".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }

        let map = map_document_text(&bytes);

        assert_eq!(map.entries().len(), 3);
        assert_eq!(map.entries()[0].kind(), DocumentTextMapKind::TextRun);
        assert_eq!(map.entries()[0].byte_start(), 10);
        assert_eq!(map.entries()[0].byte_end(), 14);
        assert_eq!(map.entries()[0].unit_start(), 5);
        assert_eq!(map.entries()[0].unit_end(), 7);
        assert_eq!(map.entries()[0].text(), "銀河");
        assert!(map.entries()[0].contains_byte_offset(10));
        assert!(map.entries()[0].contains_unit_offset(5));
        assert_eq!(
            map.entries()[1].kind(),
            DocumentTextMapKind::ControlBoundary
        );
        assert_eq!(map.entries()[1].code(), Some(0x001c));
        assert_eq!(map.entries()[2].text(), "鉄道\n");
    }

    #[test]
    fn extracts_template_placeholder_inline_segments() {
        let mut bytes = Vec::new();
        extend_units(
            &mut bytes,
            &[0x001c, 0x0001, 0x0007, 0x0000, 0x0000, 0x0001, 0x001d],
        );
        for unit in "○○○".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(&mut bytes, &[0x001e, 0x001f]);
        for unit in "賞".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }

        assert_eq!(extract_document_text(&bytes), "○○○賞");
    }

    #[test]
    fn skips_template_instruction_inline_segments() {
        let mut bytes = Vec::new();
        extend_units(
            &mut bytes,
            &[0x001c, 0x0001, 0x0007, 0x0000, 0x0001, 0x0000, 0x001d],
        );
        for unit in "名前を入力してください。".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        extend_units(&mut bytes, &[0x001e, 0x001f]);
        for unit in "本文".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }

        assert_eq!(extract_document_text(&bytes), "本文");

        let parsed = parse_document_text(&bytes);
        let skipped = parsed
            .elements()
            .iter()
            .find_map(|element| match element {
                DocumentTextElement::SkippedInlineText(segment) => Some(segment),
                _ => None,
            })
            .expect("template instruction should be preserved as skipped inline text");
        assert_eq!(skipped.selector(), Some(0x0000));
        assert_eq!(skipped.text(), "名前を入力してください。");
    }

    #[test]
    fn does_not_consume_unbounded_skipped_inline_segments() {
        let mut bytes = Vec::new();
        extend_units(
            &mut bytes,
            &[0x001c, 0x0001, 0x0007, 0x0000, 0x0001, 0x0082, 0x001d],
        );
        for _ in 0..(SKIPPED_INLINE_MAX_UNITS + 2) {
            bytes.extend_from_slice(&0x3042_u16.to_be_bytes());
        }
        bytes.extend_from_slice(&[0x00, 0x1e]);

        let parsed = parse_document_text(&bytes);

        assert!(
            parsed
                .elements()
                .iter()
                .all(|element| { !matches!(element, DocumentTextElement::SkippedInlineText(_)) })
        );
        assert!(parsed.elements().iter().any(|element| matches!(
            element,
            DocumentTextElement::ControlBoundary(control) if control.code() == 0x001d
        )));
    }

    #[test]
    fn detects_just_compressed_document_payload() {
        assert!(is_just_compressed_document(
            b"\x26\0JustCompressedDocument\0payload"
        ));
        assert!(!is_just_compressed_document(b"DocumentText"));
    }

    #[test]
    fn reports_compressed_document_when_document_text_is_absent() {
        let bytes = cfb_with_stream(
            "/JSCompDocument",
            b"\x26\0JustCompressedDocument\0-lh5-\0payload",
        );
        let error = read_document_text_stream(&bytes).unwrap_err();

        assert!(error.to_string().contains("invalid data"));
    }

    #[test]
    fn reports_missing_document_text_without_known_compressed_payload() {
        let bytes = cfb_with_stream("/Other", b"payload");
        let error = read_document_text_stream(&bytes).unwrap_err();

        assert!(error.to_string().contains("not found"));
    }

    #[test]
    fn reads_embedded_document_text_when_named_stream_is_absent() {
        let mut embedded = b"prefix SsmgV.01".to_vec();
        embedded.extend_from_slice(&[0x00, 0x1f]);
        for unit in "Note".encode_utf16() {
            embedded.extend_from_slice(&unit.to_be_bytes());
        }
        let bytes = cfb_with_stream("/JSSlipObject1", &embedded);

        let payload = read_document_text_payload(&bytes).unwrap();

        assert_eq!(payload.source_name(), EMBEDDED_DOCUMENT_TEXT_PATH);
        assert_eq!(payload.text(), "Note");
        assert!(payload.bytes().starts_with(b"SsmgV.01"));
    }

    #[test]
    fn embedded_document_text_drops_implausible_noise_lines() {
        let mut embedded = b"SsmgV.01".to_vec();
        embedded.extend_from_slice(&[0x00, 0x1f]);
        for unit in "NoteĀ́āāāā蔭\u{f706}\r本文".encode_utf16() {
            embedded.extend_from_slice(&unit.to_be_bytes());
        }
        let bytes = cfb_with_stream("/JSSlipObject1", &embedded);

        let payload = read_document_text_payload(&bytes).unwrap();

        assert_eq!(payload.text(), "本文");
    }

    fn extend_units(bytes: &mut Vec<u8>, units: &[u16]) {
        for unit in units {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
    }

    #[test]
    fn parses_raw_text_segment_format_without_paragraph_markers() {
        // SsmgV.01 w[9]=0x0001 + TextV.01 + length + raw UTF-16BE text (no 0x001f markers)
        // This matches the te.jtd format where text is stored directly in a TextV.01 segment.
        let text_content: Vec<u16> = "te\nsto\nて".encode_utf16().collect();
        let length = text_content.len() as u16;
        let mut payload: Vec<u8> = Vec::new();
        // SsmgV.01 header (10 words): magic + 4 header words + segment-count=1
        let header: &[u16] = &[0x5373, 0x6d67, 0x562e, 0x3031, 0x0000, 0x0001, 0x0000, 0x0100, 0x0000, 0x0001];
        for w in header {
            payload.extend_from_slice(&w.to_be_bytes());
        }
        // TextV.01 segment name: 8 ASCII bytes → 4 big-endian u16 words (0x5465 0x7874 0x562e 0x3031)
        for w in &[0x5465_u16, 0x7874, 0x562e, 0x3031] {
            payload.extend_from_slice(&w.to_be_bytes());
        }
        // Length field: word[14]=0x0000, word[15]=length (matches te.jtd layout)
        payload.extend_from_slice(&0x0000_u16.to_be_bytes());
        payload.extend_from_slice(&length.to_be_bytes());
        // Text content
        for w in &text_content {
            payload.extend_from_slice(&w.to_be_bytes());
        }

        let parsed = parse_document_text(&payload);
        let text = parsed.plain_text();
        assert!(text.contains("te"), "should contain 'te' but got: {text:?}");
        assert!(text.contains("sto"), "should contain 'sto' but got: {text:?}");
        assert!(text.contains('て'), "should contain 'て' but got: {text:?}");
    }

    fn cfb_with_stream(path: &str, payload: &[u8]) -> Vec<u8> {
        let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        compound
            .create_stream(path)
            .unwrap()
            .write_all(payload)
            .unwrap();
        compound.into_inner().into_inner()
    }
}
