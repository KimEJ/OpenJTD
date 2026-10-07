use crate::{
    Document, DocumentTextFlow, DocumentTextFlowKind, TextSourceSpan, UnknownStyle,
    modern_document_view_size_mm100, page_margins_mm100_at,
};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, StyleStreamRecordLayout,
    summarize_style_stream,
};

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

    #[cfg(feature = "rendering")]
    pub(crate) fn is_vertical(self) -> bool {
        matches!(self, Self::VerticalRl)
    }
}

/// Direction in the controlled modern view profile; first record codes alone
/// cannot distinguish its horizontal and vertical documents.
pub(crate) fn modern_view_writing_mode(styles: &[UnknownStyle]) -> Option<WritingMode> {
    let mut views = styles
        .iter()
        .filter(|s| s.name() == Some(DOCUMENT_VIEW_STYLES_PATH));
    let view = views.next()?;
    if views.next().is_some() {
        return None;
    }
    let summary = summarize_style_stream(view.payload());
    if summary.record_layout() != StyleStreamRecordLayout::Sequential {
        return None;
    }
    if [0x1001, 0x1002].into_iter().any(|code| {
        summary
            .records()
            .iter()
            .filter(|r| r.code() == code)
            .count()
            != 1
    }) {
        return None;
    }
    modern_document_view_size_mm100(view.payload())?;
    let margin = summary.records().iter().find(|r| r.code() == 0x1002)?;
    let offset = margin.offset().checked_add(4)?;
    let payload = view
        .payload()
        .get(offset..offset.checked_add(margin.payload_len())?)?;
    if payload.get(..2) != Some(&[0, 0xd8]) {
        return None;
    }
    page_margins_mm100_at(payload, 2)?;
    let (mode, rest) = match (payload.len(), payload.get(10..12)?) {
        (32, [0x40, 2]) => (WritingMode::Horizontal, &payload[11..]),
        (33, [0x50, 1]) => (WritingMode::VerticalRl, &payload[12..]),
        _ => return None,
    };
    if rest.len() != 21
        || rest[20] != 0
        || rest[..20]
            .as_chunks::<2>()
            .0
            .iter()
            .any(|word| *word != [2, 0xbc])
    {
        return None;
    }
    Some(mode)
}

pub(crate) fn modern_source_writing_mode(document: &Document) -> Option<WritingMode> {
    if document
        .unknown_styles()
        .iter()
        .any(|s| s.name() == Some(PAGE_LAYOUT_STYLE_PATH))
    {
        return None;
    }
    modern_view_writing_mode(document.unknown_styles())
}

/// The two-digit, no-fit 0x47 record and its paired caches. Other profiles stay raw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTatechuyokoCandidate {
    text: String,
    record_span: TextSourceSpan,
    value_span: TextSourceSpan,
    end: usize,
}

impl DocumentTatechuyokoCandidate {
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn record_span(&self) -> &TextSourceSpan {
        &self.record_span
    }
    pub fn value_span(&self) -> &TextSourceSpan {
        &self.value_span
    }
}

impl Document {
    pub fn tatechuyoko_candidates(&self) -> Vec<DocumentTatechuyokoCandidate> {
        self.document_text_flow()
            .map(native_tatechuyoko_candidates)
            .unwrap_or_default()
    }
}

pub(crate) fn native_tatechuyoko_candidates(
    flow: &DocumentTextFlow,
) -> Vec<DocumentTatechuyokoCandidate> {
    flow.events()
        .windows(9)
        .filter_map(|events| {
            let record = &events[0];
            if record.kind() != DocumentTextFlowKind::Record
                || record.raw_words() != [0x1c, 0, 12, 0, 0x47, 0, 5, 0x0210, 12, 0, 0, 0x1f]
                || !events
                    .windows(2)
                    .all(|pair| pair[0].unit_end() == pair[1].unit_start())
            {
                return None;
            }
            for (index, group) in [events.get(1..5)?, events.get(5..9)?]
                .into_iter()
                .enumerate()
            {
                let [start, prefix, text, suffix] = group else {
                    return None;
                };
                if start.kind() != DocumentTextFlowKind::Control
                    || start.code() != Some(0x1c)
                    || prefix.kind() != DocumentTextFlowKind::Opaque
                    || prefix.raw_words()
                        != [1, 7, 0, index as u16, if index == 0 { 1 } else { 0x101 }]
                    || text.kind()
                        != if index == 0 {
                            DocumentTextFlowKind::Inline
                        } else {
                            DocumentTextFlowKind::SkippedInline
                        }
                    || text.selector() != Some(if index == 0 { 1 } else { 0x101 })
                    || suffix.kind() != DocumentTextFlowKind::Opaque
                    || suffix.raw_words() != [5, 0, 1, 0x1f]
                    || (index == 0 && !text.text().is_empty())
                {
                    return None;
                }
            }
            let value = &events[7];
            if value.text().len() != 2
                || !value.text().bytes().all(|b| b.is_ascii_digit())
                || value.unit_end().checked_sub(value.unit_start())? != 4
            {
                return None;
            }
            Some(DocumentTatechuyokoCandidate {
                text: value.text().to_string(),
                record_span: record.source_span().clone(),
                value_span: value.source_span().subspan_by_units(1, 3),
                end: events[8].unit_end(),
            })
        })
        .collect()
}

#[cfg(feature = "rendering")]
pub(crate) fn native_tatechuyoko_end(
    candidates: &[DocumentTatechuyokoCandidate],
    unit: usize,
) -> Option<usize> {
    candidates
        .iter()
        .find(|c| c.record_span.unit_start() == unit)
        .map(|c| c.end)
}
