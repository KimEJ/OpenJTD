#[cfg(feature = "rendering")]
use crate::native_rule_character_supported;
use crate::{
    Block, Document, DocumentTextFlowEvent, DocumentTextFlowKind, DocumentTocEntry, Inline,
    ShanaiLanLineMarkInterval, TextSourceSpan, native_paragraph_source_bounds,
    native_visible_text_span, paragraph_by_index, shanai_lan_line_mark_intervals,
};
#[cfg(any(test, feature = "rendering"))]
use crate::{DocumentTextFlow, text_by_utf16_units};

pub(super) type NativeSourceTextRun<'a> = (usize, usize, TextSourceSpan, &'a str);

pub(super) fn native_page_source_runs(document: &Document) -> Option<Vec<NativeSourceTextRun<'_>>> {
    let mut runs = Vec::new();
    for (paragraph_index, block) in document.blocks().iter().enumerate() {
        let Block::Paragraph(paragraph) = block else {
            return None;
        };
        let mut offset = 0;
        for inline in paragraph.inlines() {
            let Inline::Text(run) = inline else {
                return None;
            };
            let span = native_visible_text_span(document, run.text(), run.source_span()?)?;
            runs.push((paragraph_index, offset, span, run.text()));
            offset += run.text().chars().count();
        }
    }
    if runs
        .windows(2)
        .any(|pair| pair[0].2.unit_end() > pair[1].2.unit_start())
    {
        return None;
    }
    Some(runs)
}

/// Logical paragraph/character ranges linked to complete physical source rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeSourceLineRange {
    pub(super) record: usize,
    pub(super) paragraph: Option<usize>,
    pub(super) start: usize,
    pub(super) end: usize,
    source_units: (usize, usize),
}

pub(super) fn native_page_source_intervals(
    document: &Document,
) -> Option<Vec<ShanaiLanLineMarkInterval>> {
    let flow = document.document_text_flow()?;
    let intervals = shanai_lan_line_mark_intervals(document);
    if intervals.first()?.unit_start != flow.source_span().unit_start()
        || intervals.last()?.unit_end != flow.source_span().unit_end().checked_add(1)?
    {
        return None;
    }
    if intervals
        .iter()
        .enumerate()
        .any(|(expected, interval)| expected != interval.record_index)
    {
        return None;
    }
    Some(intervals)
}

pub(super) fn native_page_source_line_range(
    interval: &ShanaiLanLineMarkInterval,
    runs: &[NativeSourceTextRun<'_>],
) -> Option<NativeSourceLineRange> {
    let mut range: Option<(usize, usize, usize)> = None;
    let first_run = runs.partition_point(|(_, _, span, _)| span.unit_end() <= interval.unit_start);
    for (paragraph, offset, span, text) in runs[first_run..]
        .iter()
        .take_while(|(_, _, span, _)| span.unit_start() < interval.unit_end)
    {
        let from = span.unit_start().max(interval.unit_start);
        let to = span.unit_end().min(interval.unit_end);
        if from >= to {
            continue;
        }
        let start = offset + char_offset_at_utf16_unit(text, from - span.unit_start())?;
        let end = offset + char_offset_at_utf16_unit(text, to - span.unit_start())?;
        if let Some((previous, _, previous_end)) = range {
            if previous != *paragraph || previous_end != start {
                return None;
            }
            range.as_mut()?.2 = end;
        } else {
            range = Some((*paragraph, start, end));
        }
    }
    let (paragraph, start, end) = range.map_or((None, 0, 0), |(paragraph, start, end)| {
        (Some(paragraph), start, end)
    });
    Some(NativeSourceLineRange {
        source_units: (interval.unit_start, interval.unit_end),
        record: interval.record_index,
        paragraph,
        start,
        end,
    })
}

pub(super) fn char_offset_at_utf16_unit(text: &str, offset: usize) -> Option<usize> {
    let mut units = 0;
    for (index, character) in text.chars().enumerate() {
        if units == offset {
            return Some(index);
        }
        units += character.len_utf16();
    }
    (units == offset).then_some(text.chars().count())
}

#[cfg(feature = "rendering")]
pub(super) fn inline_cache_group(events: &[DocumentTextFlowEvent]) -> bool {
    let [start, prefix, text, suffix] = events else {
        return false;
    };
    start.kind() == DocumentTextFlowKind::Control
        && start.code() == Some(0x1c)
        && prefix.kind() == DocumentTextFlowKind::Opaque
        && prefix.raw_words() == [1, 7, 0, 0, 1]
        && text.kind() == DocumentTextFlowKind::Inline
        && text.selector() == Some(1)
        && text
            .text()
            .chars()
            .all(|c| native_rule_character_supported(c) || c == '●')
        && suffix.kind() == DocumentTextFlowKind::Opaque
        && suffix.raw_words() == [5, 0, 1, 0x1f]
        && events
            .windows(2)
            .all(|pair| pair[0].unit_end() == pair[1].unit_start())
}

#[cfg(feature = "rendering")]
pub(super) fn heading_or_number_record(event: &DocumentTextFlowEvent) -> bool {
    let words = event.raw_words();
    if words.len() != 13 {
        return false;
    }
    (words[..6] == [0x1c, 0x10, 13, 0, 0x2e, 1]
        && (1..=3).contains(&words[6])
        && words[7..] == [0xffff, 0, 13, 0, 0x10, 0x1f])
        || (words[..8] == [0x1c, 0, 13, 0, 10, 0, 391, 0x2010]
            && words[8] != 0xffff
            && words[9..] == [13, 0, 0, 0x1f])
}

pub(crate) fn native_toc_section_record(event: &DocumentTextFlowEvent) -> Option<u16> {
    let words = event.raw_words();
    if event.kind() != DocumentTextFlowKind::Record
        || words.len() != 12
        || words[..4] != [0x1c, 0x20, 12, 0]
        || words[5..] != [0, 0, 0, 12, 0, 0x20, 0x1f]
        || !matches!(words[4], 0x30 | 0x31)
    {
        return None;
    }
    Some(words[4])
}

pub(crate) fn native_toc_source_scope(document: &Document) -> Option<(usize, usize)> {
    let sections = document
        .document_text_flow()?
        .events()
        .iter()
        .filter_map(|event| native_toc_section_record(event).map(|kind| (kind, event)))
        .collect::<Vec<_>>();
    let [(0x30, start), (0x31, end)] = sections.as_slice() else {
        return None;
    };
    (start.unit_end() < end.unit_start()).then_some((start.unit_start(), end.unit_end()))
}

pub(crate) fn native_toc_context_record(event: &DocumentTextFlowEvent) -> bool {
    if native_toc_section_record(event).is_some() {
        return true;
    }
    let words = event.raw_words();
    // Admit only the controlled title context and optional leader framing.
    // General tab stops and leader metrics remain undecoded.
    words
        == [
            0x1c, 0, 17, 0, 9, 375, 31, 0x90, 0, 2, 0xf81e, 0, 0, 17, 0, 0, 0x1f,
        ]
        || (words.len() == 18
            && words[..8] == [0x1c, 0, 18, 0, 21, 0, 23, 0x90]
            && matches!(words[8], 1 | 100)
            && words[9..] == [2, 0xa77c, 0, 0, 0, 18, 0, 0, 0x1f])
}

#[cfg(any(test, feature = "rendering"))]
pub(crate) fn native_toc_setting_line(
    flow: &DocumentTextFlow,
    scope: Option<(usize, usize)>,
    from: usize,
    to: usize,
) -> bool {
    scope.is_some_and(|(start, end)| start <= from && to <= end.saturating_add(1))
        && flow
            .events()
            .iter()
            .any(|event| event.unit_start() == from && native_toc_section_record(event).is_some())
        && flow
            .events()
            .iter()
            .filter(|event| event.unit_start() < to && from < event.unit_end())
            .all(|event| {
                if matches!(
                    event.kind(),
                    DocumentTextFlowKind::Text | DocumentTextFlowKind::Inline
                ) {
                    return matches!(
                        text_by_utf16_units(
                            event.text(),
                            from.max(event.unit_start()) - event.unit_start(),
                            to.min(event.unit_end()) - event.unit_start(),
                        )
                        .as_str(),
                        "" | "\n" | "\r" | "\r\n"
                    );
                }
                native_toc_section_record(event).is_some()
            })
}

pub(crate) fn native_toc_cached_entries(document: &Document) -> Vec<DocumentTocEntry> {
    let Some((from, to)) = native_toc_source_scope(document) else {
        return Vec::new();
    };
    let Some(flow) = document.document_text_flow() else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    for (index, _) in document
        .blocks()
        .iter()
        .filter(|block| matches!(block, Block::Paragraph(_)))
        .enumerate()
    {
        let Some((start, end)) = native_paragraph_source_bounds(document, index) else {
            continue;
        };
        if start < from || end > to {
            continue;
        }
        let Some(paragraph) = paragraph_by_index(document, index) else {
            return Vec::new();
        };
        let [Inline::Text(title), Inline::Text(label)] = paragraph.inlines() else {
            return Vec::new();
        };
        let Some(title_span) = title
            .source_span()
            .and_then(|span| native_visible_text_span(document, title.text(), span))
        else {
            return Vec::new();
        };
        let Some(label_span) = label
            .source_span()
            .and_then(|span| native_visible_text_span(document, label.text(), span))
        else {
            return Vec::new();
        };
        let records = flow
            .events()
            .iter()
            .filter(|event| {
                event.kind() == DocumentTextFlowKind::Record
                    && title_span.unit_end() <= event.unit_start()
                    && event.unit_end() <= label_span.unit_start()
            })
            .collect::<Vec<_>>();
        if title.text().is_empty()
            || label.text().is_empty()
            || !matches!(records.len(), 1 | 2)
            || records[0].raw_words().get(2) != Some(&17)
            || (records.len() == 2 && records[1].raw_words().get(2) != Some(&18))
            || records
                .iter()
                .any(|event| !native_toc_context_record(event))
        {
            return Vec::new();
        }
        entries.push(DocumentTocEntry::new(
            title.text(),
            label.text(),
            TextSourceSpan::new(
                title_span.byte_start(),
                label_span.byte_end(),
                title_span.unit_start(),
                label_span.unit_end(),
            ),
        ));
    }
    entries
}

impl Document {
    /// Logical ranges linked to complete physical source rows, not rendered pages.
    /// Unsupported/ruby/ambiguous row associations remain None and retain raw data.
    pub fn source_line_range_candidates(&self) -> Option<Vec<NativeSourceLineRange>> {
        let runs = native_page_source_runs(self)?;
        native_page_source_intervals(self)?
            .iter()
            .map(|interval| native_page_source_line_range(interval, &runs))
            .collect()
    }
}
impl NativeSourceLineRange {
    pub fn record_index(&self) -> usize {
        self.record
    }
    pub fn paragraph_index(&self) -> Option<usize> {
        self.paragraph
    }
    pub fn char_range(&self) -> (usize, usize) {
        (self.start, self.end)
    }
    pub fn source_unit_range(&self) -> (usize, usize) {
        self.source_units
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_source_ranges_keep_utf16_pairs_and_reject_cross_paragraph_rows() {
        let runs = [(0, 0, TextSourceSpan::new(32, 40, 16, 20), "A😀B")];
        let interval = |from, to| ShanaiLanLineMarkInterval {
            record_index: 3,
            unit_start: from,
            unit_end: to,
            flag_word: 0,
        };
        let first = native_page_source_line_range(&interval(16, 19), &runs).unwrap();
        assert_eq!(
            (first.record, first.paragraph, first.start, first.end),
            (3, Some(0), 0, 2)
        );
        let last = native_page_source_line_range(&interval(19, 20), &runs).unwrap();
        assert_eq!((last.start, last.end), (2, 3));
        assert!(native_page_source_line_range(&interval(16, 18), &runs).is_none());
        let crossing = [
            (0, 0, TextSourceSpan::new(32, 34, 16, 17), "A"),
            (1, 0, TextSourceSpan::new(34, 36, 17, 18), "B"),
        ];
        assert!(native_page_source_line_range(&interval(16, 18), &crossing).is_none());
    }
}
