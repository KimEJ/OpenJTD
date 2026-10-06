use crate::*;

/// Source-linked text in the bounded one-entry FootnoteLink profile.
/// Link roles, logical note numbering, and page placement remain candidates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentFootnoteTextCandidate {
    marker: String,
    text: String,
    source_span: TextSourceSpan,
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

fn note_record(event: &DocumentTextFlowEvent, kind: u16, id: u16) -> bool {
    let words = event.raw_words();
    event.kind() == DocumentTextFlowKind::Record
        && words.len() == 13
        && words[..6] == [0x1c, 0, 13, 0, if kind == 0x30 { 15 } else { 14 }, 0]
        && words[7..] == [kind, id, 13, 0, 0, 0x1f]
}

fn linked_footnote_text(document: &Document) -> Option<DocumentFootnoteTextCandidate> {
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
        note_anchor_offset,
        body_record_offset,
    })
}
