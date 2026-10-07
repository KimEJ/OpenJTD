use crate::{
    Document, DocumentTextFlow, DocumentTextFlowEvent, DocumentTextFlowKind, TextSourceSpan,
    WritingMode, map_document_text, modern_source_writing_mode, native_visible_text_span,
    read_be16_at,
};
use rjtd_core::document_text::{DocumentTextStyleResolver, DocumentTextStyleTypedValue};
use rjtd_core::style_stream::{
    StyleStreamRecordLayout, TEXT_LAYOUT_STYLE_PATH, summarize_style_stream,
};

/// Source-linked text in the bounded one-entry FootnoteLink profile.
/// Link roles, logical note numbering, and page placement remain candidates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentFootnoteTextCandidate {
    marker: String,
    text: String,
    source_span: TextSourceSpan,
    note_marker_span: TextSourceSpan,
    body_marker_span: TextSourceSpan,
    note_anchor_offset: u32,
    body_record_offset: u32,
}

impl DocumentFootnoteTextCandidate {
    pub fn marker(&self) -> &str {
        &self.marker
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn source_span(&self) -> &TextSourceSpan {
        &self.source_span
    }
    pub fn body_marker_span(&self) -> &TextSourceSpan {
        &self.body_marker_span
    }
    pub fn note_marker_span(&self) -> &TextSourceSpan {
        &self.note_marker_span
    }
    pub fn note_anchor_offset(&self) -> u32 {
        self.note_anchor_offset
    }
    pub fn body_record_offset(&self) -> u32 {
        self.body_record_offset
    }
}

impl Document {
    pub fn footnote_text_candidates(&self) -> Vec<DocumentFootnoteTextCandidate> {
        linked_footnote_text(self).into_iter().collect()
    }
}

fn unique_stream<'a>(document: &'a Document, name: &str) -> Option<&'a [u8]> {
    let mut streams = document
        .raw_streams()
        .iter()
        .filter(|stream| stream.name() == name);
    let bytes = streams.next()?.bytes();
    streams.next().is_none().then_some(bytes)
}

fn text_flow(name: &str, bytes: &[u8]) -> Option<DocumentTextFlow> {
    if bytes.get(..8) != Some(b"SsmgV.01") || bytes.get(20..28) != Some(b"TextV.01") {
        return None;
    }
    let count = usize::try_from(u32::from_be_bytes(bytes.get(28..32)?.try_into().ok()?)).ok()?;
    bytes.get(32..32_usize.checked_add(count.checked_mul(2)?)?)?;
    Some(DocumentTextFlow::from_map(
        name,
        bytes,
        &map_document_text(bytes),
    ))
}

pub(super) fn note_record(event: &DocumentTextFlowEvent, kind: u16, id: u16) -> bool {
    let words = event.raw_words();
    event.kind() == DocumentTextFlowKind::Record
        && words.len() == 13
        && words[..6] == [0x1c, 0, 13, 0, if kind == 0x30 { 15 } else { 14 }, 0]
        && words[7..] == [kind, id, 13, 0, 0, 0x1f]
}

pub(super) fn linked_footnote_text(document: &Document) -> Option<DocumentFootnoteTextCandidate> {
    let note_bytes = unique_stream(document, "/Footnote")?;
    // Bounded auxiliary text only; larger and multi-entry forms stay raw.
    if note_bytes.len() > 64 * 1024 {
        return None;
    }
    let link = unique_stream(document, "/FootnoteLink")?;
    // Other link shapes stay raw; neither record count nor address stride is guessed.
    let profile = [
        0, 1, 0, 1, 0, 0, 0, 44, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255,
        255, 1, 0, 255, 255, 255, 255, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0,
    ];
    if link.len() != profile.len()
        || link
            .iter()
            .zip(profile)
            .enumerate()
            .any(|(i, (value, expected))| {
                !(10..14).contains(&i) && !(18..22).contains(&i) && *value != expected
            })
    {
        return None;
    }
    let note_anchor_offset = u32::from_be_bytes(link[10..14].try_into().ok()?);
    let body_record_offset = u32::from_be_bytes(link[18..22].try_into().ok()?);
    let note_anchor = 16_usize.checked_add(usize::try_from(note_anchor_offset).ok()?)?;
    let body_start = 16_usize.checked_add(usize::try_from(body_record_offset).ok()?)?;
    let notes = text_flow("/Footnote", note_bytes)?;
    unique_stream(document, "/DocumentText")?;
    let body = document.document_text_flow()?;
    let records = notes
        .events()
        .iter()
        .filter(|event| event.raw_words().get(7) == Some(&0x30))
        .collect::<Vec<_>>();
    let [start, end] = records.as_slice() else {
        return None;
    };
    if !note_record(start, 0x30, 0)
        || !note_record(end, 0x30, 0xffff)
        || note_anchor <= start.unit_end()
        || note_anchor >= end.unit_start()
        || read_be16_at(note_bytes, note_anchor.checked_mul(2)?) != Some(0x1f)
        || !body
            .events()
            .iter()
            .any(|event| event.unit_start() == body_start && note_record(event, 0x10, 0))
    {
        return None;
    }
    let note_parts = notes
        .events()
        .iter()
        .filter(|event| {
            event.unit_start() >= start.unit_end() && event.unit_end() <= end.unit_start()
        })
        .collect::<Vec<_>>();
    let markers = note_parts
        .iter()
        .filter(|event| event.kind() == DocumentTextFlowKind::Inline && event.selector() == Some(1))
        .collect::<Vec<_>>();
    let [marker] = markers.as_slice() else {
        return None;
    };
    let text_parts = note_parts
        .iter()
        .filter(|event| event.kind() == DocumentTextFlowKind::Text)
        .collect::<Vec<_>>();
    if text_parts.first()?.unit_start() != note_anchor.checked_add(1)? {
        return None;
    }
    let text = text_parts
        .iter()
        .map(|event| event.text())
        .collect::<String>();
    let text = text.trim_end_matches(['\r', '\n']).to_string();
    if marker.text().is_empty() || text.is_empty() {
        return None;
    }
    let body_markers = body
        .events()
        .iter()
        .skip_while(|event| event.unit_start() < body_start + 13)
        .take_while(|event| {
            !(event.kind() == DocumentTextFlowKind::Record && event.raw_words().get(1) != Some(&1))
        })
        .filter(|event| event.kind() == DocumentTextFlowKind::Inline && event.selector() == Some(1))
        .collect::<Vec<_>>();
    let [body_marker] = body_markers.as_slice() else {
        return None;
    };
    if body_marker.text() != marker.text() {
        return None;
    }
    Some(DocumentFootnoteTextCandidate {
        marker: marker.text().to_string(),
        text,
        source_span: TextSourceSpan::new(
            start.unit_start() * 2,
            end.unit_start() * 2,
            start.unit_start(),
            end.unit_start(),
        ),
        body_marker_span: body_marker.source_span().clone(),
        note_marker_span: marker.source_span().clone(),
        note_anchor_offset,
        body_record_offset,
    })
}

/// The linked body marker references slot 1, while its note-area counterpart
/// references normal slot 2. Only this corroborated parent-style profile is used.
pub(crate) fn linked_footnote_marker_script_basis(
    document: &Document,
    resolver: &DocumentTextStyleResolver,
    span: &TextSourceSpan,
) -> Option<&'static str> {
    let value =
        |id| resolver.uniform_optional_value_in_range(span.unit_start(), span.unit_end(), id);
    if modern_source_writing_mode(document) != Some(WritingMode::Horizontal)
        || value(1) != Some(Some(DocumentTextStyleTypedValue::U16(1)))
        || value(20) != Some(Some(DocumentTextStyleTypedValue::U32(0x8000_0000)))
        || [2, 4, 5].into_iter().any(|id| value(id) != Some(None))
    {
        return None;
    }
    let note = linked_footnote_text(document)?;
    if native_visible_text_span(document, note.marker(), note.body_marker_span())? != *span {
        return None;
    }
    let note_span = note.note_marker_span();
    if note_span.unit_end().checked_sub(note_span.unit_start())?
        != note.marker().encode_utf16().count().checked_add(2)?
    {
        return None;
    }
    let note_resolver =
        DocumentTextStyleResolver::from_document_text_bytes(unique_stream(document, "/Footnote")?);
    if note_resolver.uniform_optional_value_in_range(
        note_span.unit_start() + 1,
        note_span.unit_end() - 1,
        1,
    ) != Some(Some(DocumentTextStyleTypedValue::U16(2)))
        || note_resolver.uniform_optional_value_in_range(
            note_span.unit_start() + 1,
            note_span.unit_end() - 1,
            20,
        ) != Some(Some(DocumentTextStyleTypedValue::U32(0x8000_0000)))
    {
        return None;
    }
    let mut styles = document
        .unknown_styles()
        .iter()
        .filter(|s| s.name() == Some(TEXT_LAYOUT_STYLE_PATH));
    let style = styles.next()?;
    if styles.next().is_some() {
        return None;
    }
    let summary = summarize_style_stream(style.payload());
    if summary.record_layout() != StyleStreamRecordLayout::SsmgSlots
        || summary.header_u32_be() != [5, 256, 5]
        || summary.header_u16_be() != [1, 2]
    {
        return None;
    }
    let [marker, area] = summary.records() else {
        return None;
    };
    for (record, offset, format) in [
        (
            marker,
            0x114,
            &[
                0, 0, 0xe0, 0, 0xff, 0xce, 0, 60, 0, 50, 0, 50, 0x80, 0, 0x80, 0, 2, 0x80, 0, 0,
            ][..],
        ),
        (
            area,
            0x214,
            &[0, 0, 0x40, 0x80, 0, 0x30, 0x80, 0, 0x80, 0, 2, 0x80, 0, 0][..],
        ),
    ] {
        if record.code() != 0x5555 || record.offset() != offset {
            return None;
        }
        let sub = record.subrecords();
        let [kind, character, paragraph] = sub else {
            return None;
        };
        if kind.code() != 0x5006
            || kind.payload() != [0, 0, 1, 1]
            || character.code() != 0x5004
            || character.payload() != format
            || paragraph.code() != 0x5007
            || paragraph.payload() != [2, 2, 0]
        {
            return None;
        }
    }
    Some("linked-footnote-text-layout-slots-1-2")
}
