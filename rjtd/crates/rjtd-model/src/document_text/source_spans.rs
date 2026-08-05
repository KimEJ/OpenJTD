use super::*;
use crate::*;

pub(crate) fn text_paragraph_boundary_candidates_from_layout(
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

pub(crate) fn range_visible_text_for_basis(
    entries: &[DocumentTextMapEntry],
    start: usize,
    end: usize,
    basis: TextCountRangeOverlapBasis,
) -> String {
    entries
        .iter()
        .filter(|entry| range_overlaps_entry_for_basis(entry, start, end, basis))
        .map(|entry| range_text_overlap_for_basis(entry, start, end, basis))
        .collect()
}

pub(crate) fn range_text_overlap_for_basis(
    entry: &DocumentTextMapEntry,
    start: usize,
    end: usize,
    basis: TextCountRangeOverlapBasis,
) -> String {
    if entry.kind() == DocumentTextMapKind::ControlBoundary || start >= end {
        return String::new();
    }

    let (entry_start, entry_end) = entry_range_for_basis(entry, basis);
    let overlap_start = entry_start.max(start);
    let overlap_end = entry_end.min(end);
    if overlap_start >= overlap_end {
        return String::new();
    }

    let (relative_start, relative_end) = match basis {
        TextCountRangeOverlapBasis::Byte => (
            overlap_start.saturating_sub(entry.byte_start()) / 2,
            overlap_end
                .saturating_sub(entry.byte_start())
                .saturating_add(1)
                / 2,
        ),
        TextCountRangeOverlapBasis::Unit => (
            overlap_start.saturating_sub(entry.unit_start()),
            overlap_end.saturating_sub(entry.unit_start()),
        ),
    };
    text_by_utf16_units(entry.text(), relative_start, relative_end)
}

pub(crate) fn range_text_overlap(entry: &DocumentTextMapEntry, start: usize, end: usize) -> String {
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

pub(crate) fn text_line_break_count(text: &str) -> usize {
    text.chars()
        .filter(|character| matches!(character, '\n' | '\r'))
        .count()
}

pub(crate) fn push_text_count_control_range_overlap(
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

pub(crate) fn text_control_source_intervals(
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

pub(crate) fn document_text_source_bounds(document: &Document) -> Option<TextSourceSpan> {
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

pub(crate) fn push_text_count_range_overlaps(
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

pub(crate) fn text_preview_for_source_overlap(
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

pub(crate) fn text_for_source_overlap(
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

pub(crate) fn text_by_utf16_units(text: &str, start: usize, end: usize) -> String {
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParagraphSourceTextSpan {
    pub(crate) paragraph_index: usize,
    pub(crate) char_start: usize,
    pub(crate) char_end: usize,
    pub(crate) unit_start: usize,
    pub(crate) unit_end: usize,
}

pub(crate) fn projected_text_controls(document: &Document) -> Vec<ProjectedTextControl> {
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

pub(crate) fn paragraph_source_text_spans(document: &Document) -> Vec<ParagraphSourceTextSpan> {
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
