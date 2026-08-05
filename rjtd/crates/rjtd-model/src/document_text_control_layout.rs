use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModelTextSource {
    TextRun,
    Inline,
}

pub(crate) struct DocumentTextSourceSpans<'a> {
    pub(crate) entries: &'a [DocumentTextMapEntry],
    pub(crate) index: usize,
}

impl<'a> DocumentTextSourceSpans<'a> {
    pub(crate) fn new(entries: &'a [DocumentTextMapEntry]) -> Self {
        Self { entries, index: 0 }
    }

    pub(crate) fn next(&mut self, kind: DocumentTextMapKind, text: &str) -> Option<TextSourceSpan> {
        while let Some(entry) = self.entries.get(self.index) {
            self.index += 1;
            if entry.kind() == kind && (text.is_empty() || entry.text() == text) {
                return Some(TextSourceSpan::from_document_text_entry(entry));
            }
        }
        None
    }

    pub(crate) fn next_control(&mut self, code: u16) -> Option<TextSourceSpan> {
        while let Some(entry) = self.entries.get(self.index) {
            self.index += 1;
            if entry.kind() == DocumentTextMapKind::ControlBoundary && entry.code() == Some(code) {
                return Some(TextSourceSpan::from_document_text_entry(entry));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DocumentTextTocRow {
    pub(crate) title: String,
    pub(crate) page_label: Option<String>,
    pub(crate) byte_start: Option<usize>,
    pub(crate) byte_end: usize,
    pub(crate) unit_start: Option<usize>,
    pub(crate) unit_end: usize,
}

impl DocumentTextTocRow {
    pub(crate) fn push_entry_span(&mut self, entry: &DocumentTextMapEntry) {
        self.byte_start = Some(
            self.byte_start
                .map_or(entry.byte_start(), |start| start.min(entry.byte_start())),
        );
        self.byte_end = self.byte_end.max(entry.byte_end());
        self.unit_start = Some(
            self.unit_start
                .map_or(entry.unit_start(), |start| start.min(entry.unit_start())),
        );
        self.unit_end = self.unit_end.max(entry.unit_end());
    }

    pub(crate) fn push_visible_text(&mut self, entry: &DocumentTextMapEntry) {
        self.push_entry_span(entry);
        self.title.push_str(&entry.text().replace(['\r', '\n'], ""));
    }

    pub(crate) fn push_page_label(&mut self, entry: &DocumentTextMapEntry) {
        self.push_entry_span(entry);
        let label = entry
            .text()
            .chars()
            .filter(|character| character.is_ascii_digit())
            .collect::<String>();
        if !label.is_empty() {
            self.page_label = Some(label);
        }
    }

    pub(crate) fn into_toc_entry(self) -> Option<DocumentTocEntry> {
        let title = self.title.trim().to_string();
        let page_label = self.page_label?;
        if title.is_empty()
            || !page_label
                .chars()
                .all(|character| character.is_ascii_digit())
            || !is_short_chapter_title(&title)
        {
            return None;
        }
        Some(DocumentTocEntry::new(
            title,
            page_label,
            TextSourceSpan::new(
                self.byte_start?,
                self.byte_end,
                self.unit_start?,
                self.unit_end,
            ),
        ))
    }
}

pub(crate) struct SourceTextPart {
    pub(crate) text: String,
    pub(crate) source_span: Option<TextSourceSpan>,
    pub(crate) break_after: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TextControlSourceInterval {
    pub(crate) index: usize,
    pub(crate) byte_start: usize,
    pub(crate) byte_end: usize,
    pub(crate) unit_start: usize,
    pub(crate) unit_end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DocumentTextControlTableCell {
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DocumentTextControlTableRow {
    pub(crate) index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) cells: Vec<DocumentTextControlTableCell>,
}

#[derive(Debug, Clone)]
pub(crate) struct PendingDocumentTextControlCell {
    pub(crate) source_start: Option<usize>,
    pub(crate) source_end: usize,
    pub(crate) text: String,
}

impl PendingDocumentTextControlCell {
    pub(crate) fn new() -> Self {
        Self {
            source_start: None,
            source_end: 0,
            text: String::new(),
        }
    }

    pub(crate) fn push_text(&mut self, entry: &DocumentTextMapEntry) {
        if self.source_start.is_none() {
            self.source_start = Some(entry.unit_start());
        }
        self.source_end = entry.unit_end();
        self.text.push_str(entry.text());
    }

    pub(crate) fn finish(&mut self) -> Option<DocumentTextControlTableCell> {
        let text = clean_table_control_cell_text(&self.text);
        let source_start = self.source_start.take()?;
        let source_end = self.source_end.max(source_start);
        self.source_end = 0;
        self.text.clear();
        if text.is_empty() {
            return None;
        }
        Some(DocumentTextControlTableCell {
            source_start,
            source_end,
            text,
        })
    }

    pub(crate) fn finish_preserving_empty(
        &mut self,
        fallback_source_start: usize,
        fallback_source_end: usize,
        include_empty: bool,
    ) -> Option<DocumentTextControlTableCell> {
        let text = clean_table_control_cell_text(&self.text);
        let source_start = self.source_start.take().unwrap_or(fallback_source_start);
        let source_end = self.source_end.max(source_start).max(fallback_source_end);
        self.source_end = 0;
        self.text.clear();
        if text.is_empty() && !include_empty {
            return None;
        }
        Some(DocumentTextControlTableCell {
            source_start,
            source_end,
            text,
        })
    }
}

pub(crate) fn table_control_cell_has_value_marker(text: &str) -> bool {
    text.chars()
        .any(|character| character.is_ascii_digit() || matches!(character, '０'..='９'))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SparseDocumentTextControlRowShape {
    pub(crate) column_count: usize,
    pub(crate) empty_cells: usize,
    pub(crate) non_empty_cells: usize,
    pub(crate) text_char_count: usize,
}

impl SparseDocumentTextControlRowShape {
    pub(crate) fn from_row(row: &DocumentTextControlTableRow) -> Self {
        let mut empty_cells = 0usize;
        let mut non_empty_cells = 0usize;
        let mut text_char_count = 0usize;
        for cell in &row.cells {
            if cell.text.is_empty() {
                empty_cells += 1;
            } else {
                non_empty_cells += 1;
                text_char_count += cell.text.chars().count();
            }
        }
        Self {
            column_count: row.cells.len(),
            empty_cells,
            non_empty_cells,
            text_char_count,
        }
    }
}

pub(crate) fn clean_table_control_cell_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn source_interval_range(
    interval: &TextControlSourceInterval,
    basis: TextCountRangeOverlapBasis,
) -> (usize, usize) {
    match basis {
        TextCountRangeOverlapBasis::Byte => (interval.byte_start, interval.byte_end),
        TextCountRangeOverlapBasis::Unit => (interval.unit_start, interval.unit_end),
    }
}

pub(crate) fn is_strict_unit_001c_single_boundary_candidate(
    entries: &[DocumentTextMapEntry],
    candidate: &TextBoundaryCandidate,
) -> bool {
    candidate.basis() == TextCountRangeOverlapBasis::Unit
        && candidate.delimiter_code() == PARAGRAPH_BOUNDARY_DELIMITER_CANDIDATE
        && candidate.interval_count() == 1
        && range_starts_after_control_gap(entries, candidate.source_start())
        && range_ends_on_aligned_text(entries, candidate.source_end())
        && !range_visible_text(entries, candidate.source_start(), candidate.source_end()).is_empty()
        && text_line_break_count(&range_visible_text(
            entries,
            candidate.source_start(),
            candidate.source_end(),
        )) <= 1
}

pub(crate) fn best_layout_exact2_evidence_for_points(
    candidate: &TextBoundaryCandidate,
    target: &'static str,
    points: &[usize],
) -> Option<TextLayoutExactEvidence> {
    let points = points.iter().copied().collect::<BTreeSet<_>>();
    let mut best: Option<TextLayoutExactEvidence> = None;
    for base in layout_map_bases() {
        let start = base.apply(candidate.source_start());
        let end = base.apply(candidate.source_end());
        for point in &points {
            let delta = *point as isize - start;
            if !(LAYOUT_MAP_DELTA_MIN..=LAYOUT_MAP_DELTA_MAX).contains(&delta) {
                continue;
            }
            let mapped_end = end + delta;
            if mapped_end < 0 || !points.contains(&(mapped_end as usize)) {
                continue;
            }
            let evidence = TextLayoutExactEvidence::new(target, base.name(), delta);
            let replace = best.as_ref().is_none_or(|best| {
                delta.unsigned_abs() < best.delta().unsigned_abs()
                    || (delta.unsigned_abs() == best.delta().unsigned_abs()
                        && base.name() < best.base())
            });
            if replace {
                best = Some(evidence);
            }
        }
    }
    best
}

#[derive(Clone, Copy)]
pub(crate) enum LayoutMapBase {
    Unit,
    UnitTimes2,
    UnitDiv2Floor,
    UnitDiv2Ceil,
}

impl LayoutMapBase {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Unit => "unit",
            Self::UnitTimes2 => "unit-times-2",
            Self::UnitDiv2Floor => "unit-div2-floor",
            Self::UnitDiv2Ceil => "unit-div2-ceil",
        }
    }

    pub(crate) fn apply(self, value: usize) -> isize {
        match self {
            Self::Unit => value as isize,
            Self::UnitTimes2 => (value as isize) * 2,
            Self::UnitDiv2Floor => (value / 2) as isize,
            Self::UnitDiv2Ceil => value.div_ceil(2) as isize,
        }
    }
}

pub(crate) fn be16_words(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
}

pub(crate) fn range_starts_after_control_gap(
    entries: &[DocumentTextMapEntry],
    offset: usize,
) -> bool {
    let touches_entry = entries.iter().any(|entry| {
        entry.unit_start() == offset || (entry.unit_start() < offset && offset < entry.unit_end())
    });
    !touches_entry
        && previous_unit_entry(entries, offset)
            .is_some_and(|entry| entry.kind() == DocumentTextMapKind::ControlBoundary)
}

pub(crate) fn range_ends_on_aligned_text(entries: &[DocumentTextMapEntry], offset: usize) -> bool {
    entries.iter().any(|entry| {
        if !matches!(
            entry.kind(),
            DocumentTextMapKind::TextRun | DocumentTextMapKind::InlineText
        ) {
            return false;
        }
        entry.unit_end() == offset
            || (entry.unit_start() < offset
                && offset < entry.unit_end()
                && range_text_overlap(entry, offset, entry.unit_end())
                    .chars()
                    .all(|character| matches!(character, '\n' | '\r')))
    })
}

pub(crate) fn previous_unit_entry(
    entries: &[DocumentTextMapEntry],
    offset: usize,
) -> Option<&DocumentTextMapEntry> {
    entries
        .iter()
        .filter(|entry| entry.unit_end() <= offset)
        .max_by_key(|entry| entry.unit_end())
}

pub(crate) fn range_visible_text(
    entries: &[DocumentTextMapEntry],
    start: usize,
    end: usize,
) -> String {
    entries
        .iter()
        .filter(|entry| range_overlaps_entry(entry, start, end))
        .map(|entry| range_text_overlap(entry, start, end))
        .collect()
}

pub(crate) fn range_overlaps_entry_for_basis(
    entry: &DocumentTextMapEntry,
    start: usize,
    end: usize,
    basis: TextCountRangeOverlapBasis,
) -> bool {
    if start >= end {
        return false;
    }
    let (entry_start, entry_end) = entry_range_for_basis(entry, basis);
    entry_start < end && entry_end > start
}

pub(crate) fn entry_range_for_basis(
    entry: &DocumentTextMapEntry,
    basis: TextCountRangeOverlapBasis,
) -> (usize, usize) {
    match basis {
        TextCountRangeOverlapBasis::Byte => (entry.byte_start(), entry.byte_end()),
        TextCountRangeOverlapBasis::Unit => (entry.unit_start(), entry.unit_end()),
    }
}

pub(crate) fn range_overlaps_entry(entry: &DocumentTextMapEntry, start: usize, end: usize) -> bool {
    if start == end {
        return entry.unit_start() <= start && start <= entry.unit_end();
    }
    start < entry.unit_end() && end > entry.unit_start()
}

pub(crate) fn source_interval_overlaps(
    interval: &TextControlSourceInterval,
    basis: TextCountRangeOverlapBasis,
    start: usize,
    end: usize,
) -> bool {
    let (interval_start, interval_end) = match basis {
        TextCountRangeOverlapBasis::Byte => (interval.byte_start, interval.byte_end),
        TextCountRangeOverlapBasis::Unit => (interval.unit_start, interval.unit_end),
    };
    if start == end {
        return interval_start <= start && start <= interval_end;
    }
    start < interval_end && end > interval_start
}

pub(crate) fn source_span_range(
    span: &TextSourceSpan,
    basis: TextCountRangeOverlapBasis,
) -> (usize, usize) {
    match basis {
        TextCountRangeOverlapBasis::Byte => (span.byte_start(), span.byte_end()),
        TextCountRangeOverlapBasis::Unit => (span.unit_start(), span.unit_end()),
    }
}

pub(crate) fn table_row_column_segments(text: &str) -> Vec<TableCandidateColumnSegment> {
    let chars = text.chars().collect::<Vec<_>>();
    let value_spans = finance_value_spans(&chars);
    if value_spans.len() < 2 {
        return Vec::new();
    }

    let mut segments = Vec::new();
    if let Some((start, end)) = trim_char_span(&chars, 0, value_spans[0].0) {
        segments.push(TableCandidateColumnSegment::new(
            segments.len(),
            TableCandidateColumnSegmentKind::Label,
            start,
            end,
            None,
            None,
            chars[start..end].iter().collect(),
        ));
    }

    for (start, end) in value_spans {
        if let Some((start, end)) = trim_char_span(&chars, start, end) {
            segments.push(TableCandidateColumnSegment::new(
                segments.len(),
                TableCandidateColumnSegmentKind::Value,
                start,
                end,
                None,
                None,
                chars[start..end].iter().collect(),
            ));
        }
    }

    segments
}

pub(crate) fn finance_value_spans(chars: &[char]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut index = 0usize;
    while index < chars.len() {
        if chars[index] == '△' {
            let mut value_start = index + 1;
            while value_start < chars.len() && chars[value_start].is_whitespace() {
                value_start += 1;
            }
            if let Some(end) = parse_finance_value_end(chars, value_start) {
                spans.push((index, end));
                index = end;
                continue;
            }
        }

        if chars[index] == '－' {
            spans.push((index, index + 1));
            index += 1;
            continue;
        }

        if let Some(end) = parse_finance_value_end(chars, index) {
            spans.push((index, end));
            index = end;
            continue;
        }

        index += 1;
    }
    spans
}

pub(crate) fn parse_finance_value_end(chars: &[char], start: usize) -> Option<usize> {
    parse_decimal_value_end(chars, start).or_else(|| parse_comma_number_end(chars, start))
}

pub(crate) fn parse_decimal_value_end(chars: &[char], start: usize) -> Option<usize> {
    if !chars
        .get(start)
        .is_some_and(|character| character.is_ascii_digit())
    {
        return None;
    }
    let mut index = start;
    while index < chars.len() && chars[index].is_ascii_digit() {
        index += 1;
    }
    if chars.get(index) != Some(&'.') {
        return None;
    }
    let decimal_start = index + 1;
    if !chars
        .get(decimal_start)
        .is_some_and(|character| character.is_ascii_digit())
    {
        return None;
    }
    Some((decimal_start + 1).min(chars.len()))
}

pub(crate) fn parse_comma_number_end(chars: &[char], start: usize) -> Option<usize> {
    if !chars
        .get(start)
        .is_some_and(|character| character.is_ascii_digit())
    {
        return None;
    }

    let mut index = start;
    let mut leading_digits = 0usize;
    while index < chars.len() && chars[index].is_ascii_digit() && leading_digits < 3 {
        index += 1;
        leading_digits += 1;
    }
    if leading_digits == 0 || chars.get(index) != Some(&',') {
        return None;
    }

    let mut group_count = 0usize;
    while chars.get(index) == Some(&',') {
        let group_start = index + 1;
        let group_end = group_start + 3;
        if group_end > chars.len()
            || !chars[group_start..group_end]
                .iter()
                .all(|character| character.is_ascii_digit())
        {
            break;
        }
        index = group_end;
        group_count += 1;
    }

    (group_count > 0).then_some(index)
}

pub(crate) fn trim_char_span(chars: &[char], start: usize, end: usize) -> Option<(usize, usize)> {
    let mut start = start.min(chars.len());
    let mut end = end.min(chars.len());
    while start < end && chars[start].is_whitespace() {
        start += 1;
    }
    while end > start && chars[end - 1].is_whitespace() {
        end -= 1;
    }
    (start < end).then_some((start, end))
}

pub(crate) fn preview_text(text: &str, max_chars: usize) -> String {
    let mut preview = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}
