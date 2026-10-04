use crate::{
    DocumentTextMap, DocumentTextMapKind, TextCountRangeOverlapBasis, TextSourceSpan, read_be16_at,
    text_by_utf16_units,
};

/// Original payload order and framing, independent of row/cell projections and
/// edits to the fallback paragraph model. Record fields remain undecoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextFlow {
    source: String,
    source_span: TextSourceSpan,
    events: Vec<DocumentTextFlowEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentTextFlowKind {
    Text,
    Inline,
    SkippedInline,
    Control,
    Record,
    Opaque,
}

impl DocumentTextFlowKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Inline => "inline",
            Self::SkippedInline => "skipped-inline",
            Self::Control => "control",
            Self::Record => "record",
            Self::Opaque => "opaque",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextFlowEvent {
    kind: DocumentTextFlowKind,
    source_span: TextSourceSpan,
    selector: Option<u16>,
    code: Option<u16>,
    text: String,
    raw_words: Vec<u16>,
}

impl DocumentTextFlow {
    pub(crate) fn from_map(source: &str, bytes: &[u8], map: &DocumentTextMap) -> Self {
        let (start, end) = map.content_span().map_or((0, bytes.len() / 2), |span| {
            (span.unit_start(), span.unit_end())
        });
        let mut events = Vec::new();
        let mut cursor = start;
        for entry in map.entries() {
            if entry.unit_start() < cursor || entry.unit_end() > end {
                continue;
            }
            Self::push_opaque(&mut events, bytes, cursor, entry.unit_start());
            let mut event = DocumentTextFlowEvent {
                kind: match entry.kind() {
                    DocumentTextMapKind::TextRun => DocumentTextFlowKind::Text,
                    DocumentTextMapKind::InlineText => DocumentTextFlowKind::Inline,
                    DocumentTextMapKind::SkippedInlineText => DocumentTextFlowKind::SkippedInline,
                    DocumentTextMapKind::ControlBoundary => DocumentTextFlowKind::Control,
                },
                source_span: TextSourceSpan::from_document_text_entry(entry),
                selector: entry.selector(),
                code: entry.code(),
                text: entry.text().to_string(),
                raw_words: Vec::new(),
            };
            if entry.code() == Some(0x001c)
                && let Some(record_end) = framed_record_end(bytes, entry.unit_start(), end)
            {
                event.kind = DocumentTextFlowKind::Record;
                event.source_span = unit_span(entry.unit_start(), record_end);
                event.raw_words = words(bytes, entry.unit_start(), record_end);
            } else if event.kind == DocumentTextFlowKind::Control {
                event.raw_words = words(bytes, entry.unit_start(), entry.unit_end());
            }
            cursor = event.source_span.unit_end();
            events.push(event);
        }
        Self::push_opaque(&mut events, bytes, cursor, end);
        Self {
            source: source.to_string(),
            source_span: unit_span(start, end),
            events,
        }
    }

    fn push_opaque(
        events: &mut Vec<DocumentTextFlowEvent>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) {
        if start < end {
            events.push(DocumentTextFlowEvent {
                kind: DocumentTextFlowKind::Opaque,
                source_span: unit_span(start, end),
                selector: None,
                code: None,
                text: String::new(),
                raw_words: words(bytes, start, end),
            });
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn source_span(&self) -> &TextSourceSpan {
        &self.source_span
    }
    pub fn events(&self) -> &[DocumentTextFlowEvent] {
        &self.events
    }

    pub(crate) fn text_for_range(
        &self,
        start: usize,
        end: usize,
        basis: TextCountRangeOverlapBasis,
    ) -> String {
        self.events
            .iter()
            .filter(|event| !event.text().is_empty())
            .map(|event| {
                let span = event.source_span();
                let (source_start, source_end) = match basis {
                    TextCountRangeOverlapBasis::Byte => (span.byte_start(), span.byte_end()),
                    TextCountRangeOverlapBasis::Unit => (span.unit_start(), span.unit_end()),
                };
                let from = source_start.max(start);
                let to = source_end.min(end);
                if from >= to {
                    return String::new();
                }
                let (from, to) = match basis {
                    TextCountRangeOverlapBasis::Byte => {
                        ((from - source_start) / 2, (to - source_start).div_ceil(2))
                    }
                    TextCountRangeOverlapBasis::Unit => (from - source_start, to - source_start),
                };
                text_by_utf16_units(event.text(), from, to)
            })
            .collect()
    }

    pub(crate) fn record_at(&self, start: usize) -> Option<&DocumentTextFlowEvent> {
        let index = self
            .events
            .binary_search_by_key(&start, DocumentTextFlowEvent::unit_start)
            .ok()?;
        let event = &self.events[index];
        (event.kind == DocumentTextFlowKind::Record).then_some(event)
    }
}

impl DocumentTextFlowEvent {
    pub fn kind(&self) -> DocumentTextFlowKind {
        self.kind
    }
    pub fn source_span(&self) -> &TextSourceSpan {
        &self.source_span
    }
    pub fn unit_start(&self) -> usize {
        self.source_span.unit_start()
    }
    pub fn unit_end(&self) -> usize {
        self.source_span.unit_end()
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
    pub fn raw_words(&self) -> &[u16] {
        &self.raw_words
    }
    pub fn record_class(&self) -> Option<u16> {
        (self.kind == DocumentTextFlowKind::Record).then(|| self.raw_words[1])
    }
}

fn unit_span(start: usize, end: usize) -> TextSourceSpan {
    TextSourceSpan::new(start * 2, end * 2, start, end)
}

fn words(bytes: &[u8], start: usize, end: usize) -> Vec<u16> {
    bytes[start * 2..end * 2]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|word| u16::from_be_bytes(*word))
        .collect()
}

// Only framing is established here. Class fields, grid flags, and logical row
// relationships remain opaque; neither wrapping nor repeated spans imply cells.
fn framed_record_end(bytes: &[u8], start: usize, limit: usize) -> Option<usize> {
    let offset = start.checked_mul(2)?;
    let class = read_be16_at(bytes, offset + 2)?;
    let length = usize::from(read_be16_at(bytes, offset + 4)?);
    let end = start.checked_add(length)?;
    if length < 7 || end > limit {
        return None;
    }
    let tail = (end - 4) * 2;
    (read_be16_at(bytes, tail) == Some(length as u16)
        && read_be16_at(bytes, tail + 2) == Some(0)
        && read_be16_at(bytes, tail + 4) == Some(class)
        && read_be16_at(bytes, tail + 6) == Some(0x001f))
    .then_some(end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table_candidates_from_document_text_controls;
    use rjtd_core::document_text::map_document_text;

    fn named_stream(body: &[u16], tail: &[u16]) -> Vec<u8> {
        let mut bytes = b"SsmgV.01".to_vec();
        bytes.extend_from_slice(&[0; 12]);
        bytes.extend_from_slice(b"TextV.01");
        bytes.extend_from_slice(&(body.len() as u32).to_be_bytes());
        bytes.extend(body.iter().chain(tail).flat_map(|word| word.to_be_bytes()));
        bytes
    }

    #[test]
    fn source_flow_preserves_wrapping_repeated_records_and_opaque_fields_in_order() {
        let declaration = [
            0x001c, 0x0030, 12, 0, 2, 40, 0x7777, 0, 12, 0, 0x0030, 0x001f,
        ];
        let mut body = "BEFORE\n".encode_utf16().collect::<Vec<_>>();
        body.extend(declaration);
        body.extend(" A\nB😀\t".encode_utf16());
        body.extend(declaration);
        body.extend(" C".encode_utf16());
        body.push(0x000e);
        body.extend("AFTER".encode_utf16());
        let bytes = named_stream(&body, &declaration);
        let flow = DocumentTextFlow::from_map("/DocumentText", &bytes, &map_document_text(&bytes));
        let records = flow
            .events()
            .iter()
            .filter(|event| event.kind() == DocumentTextFlowKind::Record)
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 2);
        assert!(
            records
                .iter()
                .all(|record| record.raw_words() == declaration)
        );
        let text = flow
            .events()
            .iter()
            .map(DocumentTextFlowEvent::text)
            .collect::<String>();
        assert_eq!(text, "BEFORE\n A\nB😀\t CAFTER");
        let mut cursor = 16;
        for event in flow.events() {
            assert_eq!(event.unit_start(), cursor);
            assert_eq!(event.source_span().byte_start(), cursor * 2);
            cursor = event.unit_end();
        }
        assert_eq!(cursor, 16 + body.len());
        assert_eq!(flow.source_span().byte_end(), 32 + body.len() * 2);
        // Unknown flags do not become cells or disappear during source parsing.
        assert!(table_candidates_from_document_text_controls(&flow, 0).is_empty());
    }

    #[test]
    fn source_flow_does_not_frame_a_record_using_bytes_beyond_declared_content() {
        let record = [0x001c, 0x0030, 12, 0, 2, 40, 0, 0, 12, 0, 0x0030, 0x001f];
        let bytes = named_stream(&record[..4], &record[4..]);
        let flow = DocumentTextFlow::from_map("/DocumentText", &bytes, &map_document_text(&bytes));
        assert!(
            flow.events()
                .iter()
                .all(|event| event.kind() != DocumentTextFlowKind::Record)
        );
        assert_eq!(flow.source_span().unit_end(), 20);
        assert_eq!(flow.events().last().unwrap().unit_end(), 20);
        assert_eq!(flow.events()[0].kind(), DocumentTextFlowKind::Control);
        assert_eq!(flow.events()[0].raw_words(), [0x001c]);
        assert_eq!(flow.events()[1].kind(), DocumentTextFlowKind::Opaque);
        assert_eq!(flow.events()[1].raw_words(), [0x0030, 12, 0]);
    }

    #[test]
    fn source_flow_keeps_embedded_record_markers_out_of_visible_events() {
        let header = [
            0x001c, 0x0010, 15, 0, 0x008f, 7, 40, 0, 0, 0x001f, 0x0041, 15, 0, 0x0010, 0x001f,
        ];
        let mut body = header.to_vec();
        body.extend("BODY".encode_utf16());
        let bytes = named_stream(&body, &[]);
        let flow = DocumentTextFlow::from_map("/DocumentText", &bytes, &map_document_text(&bytes));
        assert_eq!(flow.events().len(), 2);
        assert_eq!(flow.events()[0].raw_words(), header);
        assert_eq!(flow.events()[1].text(), "BODY");
        assert_eq!(flow.events()[1].unit_start(), 31);
    }
}
