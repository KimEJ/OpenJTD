use crate::{Document, DocumentTextFlow, DocumentTextFlowEvent, Inline, paragraph_by_index};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NativeParagraphAttributes {
    pub(crate) first_mm100: u16,
    pub(crate) continuing_mm100: u16,
    pub(crate) after_permille: u16,
}

pub(crate) fn native_paragraph_attributes(
    event: &DocumentTextFlowEvent,
) -> Option<NativeParagraphAttributes> {
    if event.record_class() != Some(0x0010) {
        return None;
    }
    let words = event.raw_words();
    let (offset, after) = match words.len() {
        17 if words.get(3..7) == Some(&[0, 0x26, 5, 1]) => (0, 0),
        25 if words.get(3..11) == Some(&[0, 0x22, 2, 0, 0, 0x23, 2, 0])
            && words.get(12..15) == Some(&[0x26, 5, 1]) =>
        {
            (8, words[11])
        }
        _ => return None,
    };
    if words[8 + offset] != 0
        || words[10 + offset] != 0
        || words[11 + offset..13 + offset] != [0xffff, 0]
        || words[7 + offset] > 10_000
        || words[9 + offset] > 10_000
        || after > 2_000
    {
        return None;
    }
    Some(NativeParagraphAttributes {
        first_mm100: words[9 + offset],
        continuing_mm100: words[7 + offset],
        after_permille: after,
    })
}

pub(crate) fn native_paragraph_source_bounds(
    document: &Document,
    index: usize,
) -> Option<(usize, usize)> {
    let paragraph = paragraph_by_index(document, index)?;
    let mut spans = paragraph
        .inlines()
        .iter()
        .filter_map(|inline| match inline {
            Inline::Text(run) => run.source_span(),
            _ => None,
        });
    let first = spans.next()?;
    let last = spans.next_back().unwrap_or(first);
    Some((first.unit_start(), last.unit_end()))
}

pub(crate) fn native_paragraph_attrs_for_index(
    document: &Document,
    index: usize,
) -> NativeParagraphAttributes {
    let Some((start, _)) = native_paragraph_source_bounds(document, index) else {
        return NativeParagraphAttributes::default();
    };
    document
        .document_text_flow()
        .into_iter()
        .flat_map(DocumentTextFlow::events)
        .find(|event| event.unit_end() == start)
        .and_then(native_paragraph_attributes)
        .unwrap_or_default()
}
