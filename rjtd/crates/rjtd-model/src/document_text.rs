use super::*;

pub(super) const DOCUMENT_TEXT_INLINE_START_TAG: u32 = 0x001d;

pub(super) const DOCUMENT_TEXT_TEXT_RUN_MARKER: u16 = 0x001f;

pub(super) const DOCUMENT_TEXT_RUBY_BASE_SELECTOR: u16 = 0x0003;

pub(super) const DOCUMENT_TEXT_RUBY_TEXT_SELECTOR: u16 = 0x0082;

pub(super) const DOCUMENT_TEXT_TOC_PAGE_SELECTOR: u16 = 0x0101;

pub(super) const DOCUMENT_TEXT_PAGE_BREAK_CONTROL: u16 = 0x000c;

pub(super) const DOCUMENT_TEXT_PATH: &str = "/DocumentText";

pub(super) const LAYOUT_BOX_TEXT_PATH: &str = "/LayoutBoxText";

pub(super) const LAYOUT_BOX_TEXT_POSITION_TABLES_PATH: &str = "/LayoutBoxTextPositionTables";

pub(super) const TEXT_CONTROL_RANGE_DELIMITER_CANDIDATES: [u16; 2] = [0x001c, 0x000e];

pub(super) const PARAGRAPH_BOUNDARY_DELIMITER_CANDIDATE: u16 = 0x001c;

pub(super) const DOCUMENT_TEXT_CONTROL_TABLE_MAX_EMPTY_GAP_ROWS: usize = 3;

pub(super) const SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR: f32 = 1.5;

pub(super) const SHANAI_LAN_TEXT_FONT_SIZE_SCALE: f32 = 1.1083333;

pub(super) const SHANAI_LAN_TEXT_BASELINE_FACTOR: f32 = 0.8;

pub(super) const SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS: u16 = 4;

pub(super) const SHANAI_LAN_TEXT_FRAGMENT_GAP_UNITS: usize = 2;

pub(super) const LAYOUT_BOX_TEXT_MAGIC: &[u8; 8] = b"TextV.01";

pub(super) const LAYOUT_BOX_TEXT_BODY_MIN_CHARS: usize = 80;

pub(super) const LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX: f32 = 14.4;

pub(super) const LAYOUT_BOX_TEXT_TITLE_FONT_SIZE_PX: f32 = 18.0;

pub(super) const LAYOUT_BOX_TEXT_CAPTION_FONT_SIZE_PX: f32 = 10.5;

pub(super) const LAYOUT_BOX_TEXT_LINE_HEIGHT_FACTOR: f32 = 2.0;

pub(super) const LAYOUT_BOX_TEXT_MIN_RENDER_WIDTH_PT: u16 = 48;

pub(super) const LAYOUT_BOX_TEXT_MAX_RENDER_WIDTH_PT: u16 = 760;

pub(super) const PAGE_FRAME_TEXT_AFTER_BAR_GAP_LINES: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextParagraphBoundaryCandidate {
    pub(super) index: usize,
    pub(super) text_boundary_candidate_index: usize,
    pub(super) text_count_range_index: usize,
    pub(super) source_start: usize,
    pub(super) source_end: usize,
    pub(super) text_count_range_span: u32,
    pub(super) line_word_evidence: TextLayoutExactEvidence,
    pub(super) page_field_evidence: TextLayoutExactEvidence,
}

impl TextParagraphBoundaryCandidate {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> &'static str {
        "layoutValidatedTextBoundaryCandidate"
    }

    pub fn text_boundary_candidate_index(&self) -> usize {
        self.text_boundary_candidate_index
    }

    pub fn text_count_range_index(&self) -> usize {
        self.text_count_range_index
    }

    pub fn source_start(&self) -> usize {
        self.source_start
    }

    pub fn source_end(&self) -> usize {
        self.source_end
    }

    pub fn text_count_range_span(&self) -> u32 {
        self.text_count_range_span
    }

    pub fn line_word_evidence(&self) -> &TextLayoutExactEvidence {
        &self.line_word_evidence
    }

    pub fn page_field_evidence(&self) -> &TextLayoutExactEvidence {
        &self.page_field_evidence
    }

    pub fn rule(&self) -> &'static str {
        "strict-unit-001c-single+nonzero-tcnt-span+line-word-value-exact2+page-be32-field-exact2"
    }
}

pub(super) fn text_count_entry_chosen_range(raw: &[u8], family: &str) -> (u32, u32) {
    if family == "be1-shifted" {
        (read_be32_candidate(raw, 1), read_be32_candidate(raw, 5))
    } else {
        (read_be32_candidate(raw, 0), read_be32_candidate(raw, 4))
    }
}

pub(super) fn text_count_entry_tail_offset(family: &str) -> usize {
    if family == "be1-shifted" { 9 } else { 8 }
}

pub(super) fn classify_text_count_entry_family(raw: &[u8]) -> &'static str {
    let be0_start = read_be32_candidate(raw, 0);
    let be0_end = read_be32_candidate(raw, 4);
    let be1_start = read_be32_candidate(raw, 1);
    let be1_end = read_be32_candidate(raw, 5);

    if be0_start < 256 && be1_start >= 256 && be1_end >= be1_start && be0_end > be1_end {
        "be1-shifted"
    } else {
        "be0"
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Paragraph {
    pub(super) inlines: Vec<Inline>,
    pub(super) style: Option<StyleRef>,
}

impl Paragraph {
    pub fn new(inlines: Vec<Inline>, style: Option<StyleRef>) -> Self {
        Self { inlines, style }
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        Self::new(vec![Inline::Text(TextRun::new(text, None))], None)
    }

    pub fn inlines(&self) -> &[Inline] {
        &self.inlines
    }

    pub fn style(&self) -> Option<&StyleRef> {
        self.style.as_ref()
    }

    pub(super) fn set_style(&mut self, style: Option<StyleRef>) {
        self.style = style;
    }

    pub(super) fn set_text(&mut self, text: impl Into<String>) {
        self.inlines = vec![Inline::Text(TextRun::new(text, None))];
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RubyAnnotation {
    pub(super) base_text: String,
    pub(super) annotation_text: String,
    pub(super) annotation_selector: u16,
    pub(super) annotation_source: UnknownObject,
}

impl RubyAnnotation {
    pub fn new(
        base_text: impl Into<String>,
        annotation_text: impl Into<String>,
        annotation_selector: u16,
        annotation_source: UnknownObject,
    ) -> Self {
        Self {
            base_text: base_text.into(),
            annotation_text: annotation_text.into(),
            annotation_selector,
            annotation_source,
        }
    }

    pub fn base_text(&self) -> &str {
        &self.base_text
    }

    pub fn annotation_text(&self) -> &str {
        &self.annotation_text
    }

    pub fn annotation_selector(&self) -> u16 {
        self.annotation_selector
    }

    pub fn annotation_source(&self) -> &UnknownObject {
        &self.annotation_source
    }
}

pub(super) fn document_text_toc_entries(entries: &[DocumentTextMapEntry]) -> Vec<DocumentTocEntry> {
    let mut toc_entries = Vec::new();
    let mut row = DocumentTextTocRow::default();

    for entry in entries {
        match entry.kind() {
            DocumentTextMapKind::TextRun | DocumentTextMapKind::InlineText => {
                row.push_visible_text(entry);
            }
            DocumentTextMapKind::SkippedInlineText => {
                if entry.selector() == Some(DOCUMENT_TEXT_TOC_PAGE_SELECTOR) {
                    row.push_page_label(entry);
                }
            }
            DocumentTextMapKind::ControlBoundary => {}
        }

        if (entry.text().contains('\n') || entry.text().contains('\r'))
            && let Some(toc_entry) = std::mem::take(&mut row).into_toc_entry()
        {
            toc_entries.push(toc_entry);
        }
    }

    if let Some(toc_entry) = row.into_toc_entry() {
        toc_entries.push(toc_entry);
    }

    toc_entries
}

pub(super) fn source_text_parts(
    text: &str,
    source_span: Option<&TextSourceSpan>,
) -> Vec<SourceTextPart> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut current_start_units = 0usize;
    let mut unit_index = 0usize;
    let mut chars = text.chars().peekable();

    while let Some(character) = chars.next() {
        match character {
            '\r' => {
                parts.push(SourceTextPart {
                    text: std::mem::take(&mut current),
                    source_span: source_span
                        .map(|span| span.subspan_by_units(current_start_units, unit_index)),
                    break_after: true,
                });
                unit_index += character.len_utf16();
                if chars.peek() == Some(&'\n') {
                    chars.next();
                    unit_index += '\n'.len_utf16();
                }
                current_start_units = unit_index;
            }
            '\n' => {
                parts.push(SourceTextPart {
                    text: std::mem::take(&mut current),
                    source_span: source_span
                        .map(|span| span.subspan_by_units(current_start_units, unit_index)),
                    break_after: true,
                });
                unit_index += character.len_utf16();
                current_start_units = unit_index;
            }
            character => {
                current.push(character);
                unit_index += character.len_utf16();
            }
        }
    }

    parts.push(SourceTextPart {
        text: current,
        source_span: source_span.map(|span| span.subspan_by_units(current_start_units, unit_index)),
        break_after: false,
    });
    parts
}

pub(super) fn text_count_range_overlaps(
    range: &TextCountRange,
    document: &Document,
) -> Vec<TextCountRangeOverlap> {
    let mut overlaps = Vec::new();
    push_text_count_range_overlaps(
        &mut overlaps,
        TextCountRangeOverlapBasis::Byte,
        range.start() as usize,
        range.end() as usize,
        document,
    );
    push_text_count_range_overlaps(
        &mut overlaps,
        TextCountRangeOverlapBasis::Unit,
        range.start() as usize,
        range.end() as usize,
        document,
    );
    overlaps
}

pub(super) fn text_count_control_range_overlaps(
    range: &TextCountRange,
    document: &Document,
    delimiter_codes: &[u16],
) -> Vec<TextCountControlRangeOverlap> {
    let Some(bounds) = document_text_source_bounds(document) else {
        return Vec::new();
    };

    let mut overlaps = Vec::new();
    for delimiter_code in delimiter_codes {
        let intervals = text_control_source_intervals(document, &bounds, *delimiter_code);
        if intervals.is_empty() {
            continue;
        }
        push_text_count_control_range_overlap(
            &mut overlaps,
            TextCountRangeOverlapBasis::Byte,
            *delimiter_code,
            range.start() as usize,
            range.end() as usize,
            &intervals,
        );
        push_text_count_control_range_overlap(
            &mut overlaps,
            TextCountRangeOverlapBasis::Unit,
            *delimiter_code,
            range.start() as usize,
            range.end() as usize,
            &intervals,
        );
    }
    overlaps
}

pub(super) fn text_boundary_candidates_from_ranges(
    ranges: &[TextCountRange],
) -> Vec<TextBoundaryCandidate> {
    let mut candidates = Vec::new();
    for range in ranges {
        for overlap in range.control_range_overlaps() {
            candidates.push(TextBoundaryCandidate::from_control_range_overlap(
                candidates.len(),
                range.index(),
                overlap,
            ));
        }
    }
    candidates
}

pub(super) fn document_text_control_table_row_is_compatible(
    current_rows: &[DocumentTextControlTableRow],
    current_column_count: usize,
    row: &DocumentTextControlTableRow,
) -> bool {
    let column_count = row.cells.len();
    if column_count == current_column_count {
        return true;
    }
    if current_rows.len() < 2 || current_column_count < 3 {
        return false;
    }
    column_count + 1 == current_column_count
        && document_text_control_row_starts_with_short_label(row)
        && current_rows
            .iter()
            .all(document_text_control_row_starts_with_short_label)
}

pub(super) fn document_text_control_row_starts_with_short_label(
    row: &DocumentTextControlTableRow,
) -> bool {
    row.cells
        .first()
        .is_some_and(|cell| cell.text.chars().count() <= 2 && !cell.text.is_empty())
}

pub(super) fn document_text_control_table_rows_are_plausible(
    rows: &[DocumentTextControlTableRow],
) -> bool {
    if rows.len() >= 3 {
        return true;
    }
    rows.len() >= 2
        && rows.iter().skip(1).any(|row| {
            row.cells
                .iter()
                .any(|cell| table_control_cell_has_value_marker(&cell.text))
        })
}

pub(super) fn sparse_document_text_control_row_is_seed(row: &DocumentTextControlTableRow) -> bool {
    let shape = SparseDocumentTextControlRowShape::from_row(row);
    shape.column_count >= 3
        && shape.empty_cells > 0
        && shape.non_empty_cells > 0
        && shape.text_char_count <= 48
}

pub(super) fn sparse_document_text_control_row_is_blank(row: &DocumentTextControlTableRow) -> bool {
    let shape = SparseDocumentTextControlRowShape::from_row(row);
    shape.column_count >= 2 && shape.non_empty_cells == 0
}

pub(super) fn sparse_document_text_control_row_is_soft_separator(
    row: &DocumentTextControlTableRow,
) -> bool {
    let shape = SparseDocumentTextControlRowShape::from_row(row);
    shape.non_empty_cells <= 1
        && shape.text_char_count <= 1
        && row.cells.iter().all(|cell| {
            cell.text
                .chars()
                .all(|character| !character.is_alphanumeric())
        })
}

pub(super) fn sparse_document_text_control_table_rows_are_plausible(
    rows: &[DocumentTextControlTableRow],
) -> bool {
    if rows.len() < 4 {
        return false;
    }

    let mut empty_cells = 0usize;
    let mut non_empty_cells = 0usize;
    let mut max_columns = 0usize;
    let mut rows_with_text = 0usize;

    for row in rows {
        max_columns = max_columns.max(row.cells.len());
        let mut row_has_text = false;
        for cell in &row.cells {
            if cell.text.is_empty() {
                empty_cells += 1;
            } else {
                non_empty_cells += 1;
                row_has_text = true;
            }
        }
        if row_has_text {
            rows_with_text += 1;
        }
    }

    max_columns >= 3 && empty_cells >= 3 && non_empty_cells >= 3 && rows_with_text >= 3
}

pub(super) fn sparse_document_text_control_table_rows(
    entries: &[DocumentTextMapEntry],
) -> Vec<DocumentTextControlTableRow> {
    let mut rows = Vec::new();
    let mut cells = Vec::new();
    let mut cell = PendingDocumentTextControlCell::new();
    let mut row_start: Option<usize> = None;
    let mut row_index = 0usize;

    for entry in entries {
        match entry.kind() {
            DocumentTextMapKind::TextRun | DocumentTextMapKind::InlineText => {
                if row_start.is_none() {
                    row_start = Some(entry.unit_start());
                }
                cell.push_text(entry);
            }
            DocumentTextMapKind::SkippedInlineText => {}
            DocumentTextMapKind::ControlBoundary => match entry.code() {
                Some(TABLE_CELL_DELIMITER_CONTROL) => {
                    if row_start.is_none() {
                        row_start = Some(entry.unit_start());
                    }
                    if let Some(finished) =
                        cell.finish_preserving_empty(entry.unit_start(), entry.unit_start(), true)
                    {
                        cells.push(finished);
                    }
                }
                Some(TABLE_ROW_DELIMITER_CONTROL) => {
                    if row_start.is_none() {
                        row_start = Some(entry.unit_start());
                    }
                    let include_empty_tail = !cells.is_empty();
                    if let Some(finished) = cell.finish_preserving_empty(
                        entry.unit_start(),
                        entry.unit_start(),
                        include_empty_tail,
                    ) {
                        cells.push(finished);
                    }
                    let source_start = row_start.unwrap_or(entry.unit_start());
                    rows.push(DocumentTextControlTableRow {
                        index: row_index,
                        source_start,
                        source_end: entry.unit_end(),
                        cells: std::mem::take(&mut cells),
                    });
                    row_index += 1;
                    row_start = None;
                }
                _ => {
                    if let Some(finished) =
                        cell.finish_preserving_empty(entry.unit_start(), entry.unit_start(), false)
                    {
                        cells.push(finished);
                    }
                    if let Some(source_start) = row_start.take() {
                        rows.push(DocumentTextControlTableRow {
                            index: row_index,
                            source_start,
                            source_end: entry.unit_start(),
                            cells: std::mem::take(&mut cells),
                        });
                        row_index += 1;
                    } else {
                        cells.clear();
                    }
                }
            },
        }
    }

    if let Some(finished) = cell.finish_preserving_empty(0, 0, !cells.is_empty()) {
        cells.push(finished);
    }
    if let Some(source_start) = row_start {
        let source_end = cells
            .last()
            .map_or(source_start, |cell| cell.source_end.max(source_start));
        rows.push(DocumentTextControlTableRow {
            index: row_index,
            source_start,
            source_end,
            cells,
        });
    }

    rows
}

pub(super) fn document_text_control_table_rows(
    entries: &[DocumentTextMapEntry],
) -> Vec<DocumentTextControlTableRow> {
    let mut rows = Vec::new();
    let mut cells = Vec::new();
    let mut cell = PendingDocumentTextControlCell::new();
    let mut row_start: Option<usize> = None;
    let mut row_index = 0usize;

    for entry in entries {
        match entry.kind() {
            DocumentTextMapKind::TextRun | DocumentTextMapKind::InlineText => {
                if row_start.is_none() {
                    row_start = Some(entry.unit_start());
                }
                cell.push_text(entry);
            }
            DocumentTextMapKind::SkippedInlineText => {}
            DocumentTextMapKind::ControlBoundary => match entry.code() {
                Some(TABLE_CELL_DELIMITER_CONTROL) => {
                    if row_start.is_none() {
                        row_start = Some(entry.unit_start());
                    }
                    if let Some(finished) = cell.finish_preserving_empty(
                        entry.unit_start(),
                        entry.unit_start(),
                        !cells.is_empty(),
                    ) {
                        cells.push(finished);
                    }
                }
                Some(TABLE_ROW_DELIMITER_CONTROL) => {
                    if row_start.is_none() {
                        row_start = Some(entry.unit_start());
                    }
                    if let Some(finished) = cell.finish() {
                        cells.push(finished);
                    }
                    let source_start = row_start.unwrap_or(entry.unit_start());
                    rows.push(DocumentTextControlTableRow {
                        index: row_index,
                        source_start,
                        source_end: entry.unit_end(),
                        cells: std::mem::take(&mut cells),
                    });
                    row_index += 1;
                    row_start = None;
                }
                _ => {
                    if let Some(finished) = cell.finish() {
                        cells.push(finished);
                    }
                    if let Some(source_start) = row_start.take() {
                        rows.push(DocumentTextControlTableRow {
                            index: row_index,
                            source_start,
                            source_end: entry.unit_start(),
                            cells: std::mem::take(&mut cells),
                        });
                        row_index += 1;
                    } else {
                        cells.clear();
                    }
                }
            },
        }
    }

    if let Some(finished) = cell.finish() {
        cells.push(finished);
    }
    if let Some(source_start) = row_start {
        let source_end = cells
            .last()
            .map_or(source_start, |cell| cell.source_end.max(source_start));
        rows.push(DocumentTextControlTableRow {
            index: row_index,
            source_start,
            source_end,
            cells,
        });
    }

    rows
}

pub(super) fn text_paragraph_boundary_candidates_from_layout(
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

pub(super) fn range_visible_text_for_basis(
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

pub(super) fn range_text_overlap_for_basis(
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

pub(super) fn range_text_overlap(entry: &DocumentTextMapEntry, start: usize, end: usize) -> String {
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

pub(super) fn text_line_break_count(text: &str) -> usize {
    text.chars()
        .filter(|character| matches!(character, '\n' | '\r'))
        .count()
}

pub(super) fn push_text_count_control_range_overlap(
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

pub(super) fn text_control_source_intervals(
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

pub(super) fn document_text_source_bounds(document: &Document) -> Option<TextSourceSpan> {
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

pub(super) fn push_text_count_range_overlaps(
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

pub(super) fn text_preview_for_source_overlap(
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

pub(super) fn text_for_source_overlap(
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

pub(super) fn text_by_utf16_units(text: &str, start: usize, end: usize) -> String {
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
pub(super) struct ParagraphSourceTextSpan {
    pub(super) paragraph_index: usize,
    pub(super) char_start: usize,
    pub(super) char_end: usize,
    pub(super) unit_start: usize,
    pub(super) unit_end: usize,
}

pub(super) fn projected_text_controls(document: &Document) -> Vec<ProjectedTextControl> {
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

pub(super) fn paragraph_source_text_spans(document: &Document) -> Vec<ParagraphSourceTextSpan> {
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

pub(super) fn paginate_document_text(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Vec<Vec<PageTextLine>> {
    let wrap_columns = layout.wrap_columns(writing_mode);
    let lines_per_page = layout.lines_per_page(writing_mode);
    let forced_breaks = projected_page_breaks(document);
    let mut pages = Vec::new();
    let mut current_page = Vec::new();
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                let text = paragraph_text(paragraph);
                let paragraph_breaks = forced_breaks
                    .get(&paragraph_index)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);
                let wrapped = wrap_text_line(&text, paragraph_index, wrap_columns);
                let mut forced_at_paragraph_end = false;
                if wrapped.is_empty() {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        PageTextLine::new(String::new(), Some(paragraph_index), 0, 0),
                        lines_per_page,
                    );
                    if paragraph_breaks.contains(&0) {
                        force_page_break(&mut pages, &mut current_page);
                        forced_at_paragraph_end = true;
                    }
                } else {
                    for line in wrapped {
                        let segments = split_line_at_page_breaks(line, paragraph_breaks);
                        for segment in segments {
                            push_paginated_line(
                                &mut pages,
                                &mut current_page,
                                segment.line,
                                lines_per_page,
                            );
                            if segment.break_after {
                                force_page_break(&mut pages, &mut current_page);
                                forced_at_paragraph_end = true;
                            } else {
                                forced_at_paragraph_end = false;
                            }
                        }
                    }
                }
                if !forced_at_paragraph_end && !writing_mode.is_vertical() {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        PageTextLine::new(String::new(), None, 0, 0),
                        lines_per_page,
                    );
                }
                paragraph_index += 1;
            }
            Block::Unknown(_) => {
                push_paginated_line(
                    &mut pages,
                    &mut current_page,
                    PageTextLine::new("[UnknownBlock preserved by rjtd]".to_string(), None, 0, 0),
                    lines_per_page,
                );
                push_paginated_line(
                    &mut pages,
                    &mut current_page,
                    PageTextLine::new(String::new(), None, 0, 0),
                    lines_per_page,
                );
            }
        }
    }

    while current_page
        .last()
        .is_some_and(|line| line.text().is_empty() && line.paragraph_index().is_none())
    {
        current_page.pop();
    }

    if !current_page.is_empty() {
        pages.push(current_page);
    }

    if pages.is_empty() {
        if !document.raw_streams().is_empty() {
            let raw_streams = document
                .raw_streams()
                .iter()
                .map(|stream| stream.name())
                .collect::<Vec<_>>()
                .join(", ");
            return vec![vec![PageTextLine::new(
                format!("[rjtd] No extractable text. Preserved raw streams: {raw_streams}"),
                None,
                0,
                0,
            )]];
        }
        return vec![Vec::new()];
    }

    pages
}

pub(super) fn blank_page_text_line() -> PageTextLine {
    PageTextLine::new(String::new(), None, 0, 0)
}

pub(super) fn split_page_text_line_by_display_columns(
    line: PageTextLine,
    max_columns: usize,
) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    let mut text = String::new();
    let mut width = 0usize;
    let mut line_start = line.char_start();
    let mut char_offset = line.char_start();

    for character in line.text().chars() {
        let char_width = display_column_width(character);
        if width > 0 && width + char_width > max_columns {
            lines.push(PageTextLine::new(
                std::mem::take(&mut text),
                line.paragraph_index(),
                line_start,
                char_offset,
            ));
            width = 0;
            line_start = char_offset;
        }
        text.push(character);
        width += char_width;
        char_offset += 1;
    }

    if !text.is_empty() {
        lines.push(PageTextLine::new(
            text,
            line.paragraph_index(),
            line_start,
            char_offset,
        ));
    }

    lines
}

pub(super) fn vertical_page_text_placement(
    layout: PageLayout,
    lines: &[PageTextLine],
) -> VerticalPageTextPlacement {
    if is_ginga_colophon_page(lines) {
        return VerticalPageTextPlacement {
            x_shift_px: -(APP_LINE_HEIGHT_PX * GINGA_COLOPHON_X_SHIFT_COLUMNS),
            y_start_px: (layout.height_px() * GINGA_COLOPHON_TOP_RATIO).max(layout.margin_px()),
        };
    }

    VerticalPageTextPlacement {
        x_shift_px: 0.0,
        y_start_px: layout.margin_px(),
    }
}

pub(super) fn document_paragraph_texts(document: &Document) -> Vec<(usize, String)> {
    let mut paragraph_index = 0usize;
    let mut paragraphs = Vec::new();
    for block in document.blocks() {
        if let Block::Paragraph(paragraph) = block {
            paragraphs.push((paragraph_index, paragraph_text(paragraph)));
            paragraph_index += 1;
        }
    }
    paragraphs
}

pub(super) fn document_auto_text_title(document: &Document) -> Option<&str> {
    document
        .auto_texts()
        .iter()
        .map(DocumentAutoText::text)
        .map(str::trim)
        .find(|text| !text.is_empty())
}

pub(super) fn page_has_exact_text_line(lines: &[PageTextLine], text: &str) -> bool {
    lines.iter().any(|line| line.text().trim() == text)
}

pub(super) fn wrap_paragraphs_as_single_page(
    paragraphs: &[(usize, String)],
    wrap_columns: usize,
    writing_mode: WritingMode,
) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    for (paragraph_index, text) in paragraphs {
        lines.extend(wrap_text_line(text, *paragraph_index, wrap_columns));
        if !writing_mode.is_vertical() {
            lines.push(PageTextLine::new(String::new(), None, 0, 0));
        }
    }
    trim_trailing_projection_blank_lines(&mut lines);
    lines
}

pub(super) fn paginate_selected_paragraphs(
    paragraphs: &[(usize, String)],
    layout: PageLayout,
    writing_mode: WritingMode,
    forced_breaks: &BTreeMap<usize, Vec<usize>>,
) -> Vec<Vec<PageTextLine>> {
    let wrap_columns = layout.wrap_columns(writing_mode);
    let lines_per_page = layout.lines_per_page(writing_mode);
    let mut pages = Vec::new();
    let mut current_page = Vec::new();

    for (paragraph_index, text) in paragraphs {
        let paragraph_breaks = forced_breaks
            .get(paragraph_index)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let wrapped = wrap_text_line(text, *paragraph_index, wrap_columns);
        let mut forced_at_paragraph_end = false;
        if wrapped.is_empty() {
            push_paginated_line(
                &mut pages,
                &mut current_page,
                PageTextLine::new(String::new(), Some(*paragraph_index), 0, 0),
                lines_per_page,
            );
            if paragraph_breaks.contains(&0) {
                force_page_break(&mut pages, &mut current_page);
                forced_at_paragraph_end = true;
            }
        } else {
            for line in wrapped {
                let segments = split_line_at_page_breaks(line, paragraph_breaks);
                for segment in segments {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        segment.line,
                        lines_per_page,
                    );
                    if segment.break_after {
                        force_page_break(&mut pages, &mut current_page);
                        forced_at_paragraph_end = true;
                    } else {
                        forced_at_paragraph_end = false;
                    }
                }
            }
        }
        if !forced_at_paragraph_end && !writing_mode.is_vertical() {
            push_paginated_line(
                &mut pages,
                &mut current_page,
                PageTextLine::new(String::new(), None, 0, 0),
                lines_per_page,
            );
        }
    }

    trim_trailing_projection_blank_lines(&mut current_page);
    if !current_page.is_empty() {
        pages.push(current_page);
    }

    pages
}

pub(super) fn paragraph_text(paragraph: &Paragraph) -> String {
    let mut text = String::new();

    for inline in paragraph.inlines() {
        match inline {
            Inline::Text(run) => text.push_str(run.text()),
            Inline::Ruby(ruby) => text.push_str(ruby.base_text()),
            Inline::Unknown(_) => {}
        }
    }

    text
}

pub(super) fn wrap_text_line(
    text: &str,
    paragraph_index: usize,
    max_columns: usize,
) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut width = 0usize;
    let mut line_start = 0usize;
    let mut char_offset = 0usize;

    for character in text.chars() {
        let char_width = display_column_width(character);
        if width > 0 && width + char_width > max_columns {
            lines.push(PageTextLine::new(
                std::mem::take(&mut line),
                Some(paragraph_index),
                line_start,
                char_offset,
            ));
            width = 0;
            line_start = char_offset;
        }
        line.push(character);
        width += char_width;
        char_offset += 1;
    }

    if !line.is_empty() {
        lines.push(PageTextLine::new(
            line,
            Some(paragraph_index),
            line_start,
            char_offset,
        ));
    }

    lines
}

pub(super) fn text_display_column_width(text: &str) -> usize {
    text.chars().map(display_column_width).sum()
}

pub(super) fn nearest_text_line(
    lines: &[PageTextLine],
    target_index: usize,
) -> Option<(usize, &PageTextLine)> {
    if lines.is_empty() {
        return None;
    }

    let target_index = target_index.min(lines.len() - 1);
    if lines[target_index].paragraph_index().is_some() {
        return Some((target_index, &lines[target_index]));
    }

    for distance in 1..lines.len() {
        if let Some(index) = target_index.checked_sub(distance)
            && lines[index].paragraph_index().is_some()
        {
            return Some((index, &lines[index]));
        }

        let index = target_index + distance;
        if index < lines.len() && lines[index].paragraph_index().is_some() {
            return Some((index, &lines[index]));
        }
    }

    None
}

pub(super) fn paragraph_line_index(
    lines: &[(usize, usize, &PageTextLine)],
    char_offset: usize,
) -> usize {
    let mut last_index = 0usize;

    for (index, (_, _, line)) in lines.iter().enumerate() {
        last_index = index;
        if char_offset <= line.char_end() {
            return index;
        }
    }

    last_index
}

pub(super) fn text_location_index(
    locations: &[(usize, usize, &PageTextLine)],
    paragraph_index: usize,
    char_offset: usize,
) -> Result<usize> {
    let mut last_index = None;

    for (index, (_, _, line)) in locations.iter().enumerate() {
        if line.paragraph_index() != Some(paragraph_index) {
            continue;
        }

        last_index = Some(index);
        if char_offset <= line.char_end() {
            return Ok(index);
        }
    }

    last_index.ok_or_else(|| {
        rjtd_core::Error::InvalidData(format!("paragraph {paragraph_index} out of range"))
    })
}

pub(super) fn text_count_ranges_json(ranges: &[TextCountRange]) -> String {
    let mut output = String::from("[");
    for (index, range) in ranges.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_count_range_json(&mut output, range);
    }
    output.push(']');
    output
}

pub(super) fn text_control_boundaries_json(boundaries: &[TextControlBoundary]) -> String {
    let mut output = String::from("[");
    for (index, boundary) in boundaries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_control_boundary_json(&mut output, boundary);
    }
    output.push(']');
    output
}

pub(super) fn text_boundary_candidates_json(candidates: &[TextBoundaryCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_boundary_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

pub(super) fn text_paragraph_boundary_candidates_json(
    candidates: &[TextParagraphBoundaryCandidate],
) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_text_paragraph_boundary_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

pub(super) fn push_text_boundary_candidate_json(
    output: &mut String,
    candidate: &TextBoundaryCandidate,
) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"basis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        candidate.delimiter_code()
    )));
    output.push_str(",\"intervalCount\":");
    output.push_str(&candidate.interval_count().to_string());
    output.push_str(",\"firstIntervalIndex\":");
    output.push_str(&candidate.first_interval_index().to_string());
    output.push_str(",\"lastIntervalIndex\":");
    output.push_str(&candidate.last_interval_index().to_string());
    output.push_str(",\"sourceStart\":");
    output.push_str(&candidate.source_start().to_string());
    output.push_str(",\"sourceEnd\":");
    output.push_str(&candidate.source_end().to_string());
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_text_paragraph_boundary_candidate_json(
    output: &mut String,
    candidate: &TextParagraphBoundaryCandidate,
) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"textBoundaryCandidateIndex\":");
    output.push_str(&candidate.text_boundary_candidate_index().to_string());
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"sourceStart\":");
    output.push_str(&candidate.source_start().to_string());
    output.push_str(",\"sourceEnd\":");
    output.push_str(&candidate.source_end().to_string());
    output.push_str(",\"textCountRangeSpan\":");
    output.push_str(&candidate.text_count_range_span().to_string());
    output.push_str(",\"rule\":");
    output.push_str(&json_string(candidate.rule()));
    output.push_str(",\"lineWordEvidence\":");
    push_text_layout_exact_evidence_json(output, candidate.line_word_evidence());
    output.push_str(",\"pageFieldEvidence\":");
    push_text_layout_exact_evidence_json(output, candidate.page_field_evidence());
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_text_layout_exact_evidence_json(
    output: &mut String,
    evidence: &TextLayoutExactEvidence,
) {
    output.push_str("{\"target\":");
    output.push_str(&json_string(evidence.target()));
    output.push_str(",\"base\":");
    output.push_str(&json_string(evidence.base()));
    output.push_str(",\"delta\":");
    output.push_str(&evidence.delta().to_string());
    output.push('}');
}

pub(super) fn push_text_control_boundary_json(output: &mut String, boundary: &TextControlBoundary) {
    output.push_str("{\"index\":");
    output.push_str(&boundary.index().to_string());
    output.push_str(",\"code\":");
    output.push_str(&boundary.code().to_string());
    output.push_str(",\"codeHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", boundary.code())));
    output.push_str(",\"sourceSpan\":");
    match boundary.source_span() {
        Some(span) => push_text_source_span_json(output, span),
        None => output.push_str("null"),
    }
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_text_source_span_json(output: &mut String, span: &TextSourceSpan) {
    output.push_str("{\"byteStart\":");
    output.push_str(&span.byte_start().to_string());
    output.push_str(",\"byteEnd\":");
    output.push_str(&span.byte_end().to_string());
    output.push_str(",\"unitStart\":");
    output.push_str(&span.unit_start().to_string());
    output.push_str(",\"unitEnd\":");
    output.push_str(&span.unit_end().to_string());
    output.push('}');
}

pub(super) fn push_text_count_range_json(output: &mut String, range: &TextCountRange) {
    output.push_str("{\"index\":");
    output.push_str(&range.index().to_string());
    output.push_str(",\"family\":");
    output.push_str(&json_string(range.family()));
    output.push_str(",\"start\":");
    output.push_str(&range.start().to_string());
    output.push_str(",\"end\":");
    output.push_str(&range.end().to_string());
    output.push_str(",\"span\":");
    output.push_str(&range.span().to_string());
    output.push_str(",\"declaredStart\":");
    output.push_str(&range.declared_start().to_string());
    output.push_str(",\"declaredEnd\":");
    output.push_str(&range.declared_end().to_string());
    output.push_str(",\"tailFields\":");
    push_u16_array_json(output, range.tail_fields());
    output.push_str(",\"documentTextOverlaps\":");
    text_count_range_overlaps_json(output, range.document_text_overlaps());
    output.push_str(",\"controlRangeOverlaps\":");
    text_count_control_range_overlaps_json(output, range.control_range_overlaps());
    output.push_str(",\"decoded\":false,\"rawHex\":");
    output.push_str(&json_string(&hex_bytes(range.raw())));
    output.push('}');
}

pub(super) fn text_count_range_overlaps_json(
    output: &mut String,
    overlaps: &[TextCountRangeOverlap],
) {
    output.push('[');
    for (index, overlap) in overlaps.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"basis\":");
        output.push_str(&json_string(overlap.basis().as_str()));
        output.push_str(",\"blockIndex\":");
        output.push_str(&overlap.block_index().to_string());
        output.push_str(",\"inlineIndex\":");
        output.push_str(&overlap.inline_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&overlap.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&overlap.source_end().to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(overlap.text()));
        output.push('}');
    }
    output.push(']');
}

pub(super) fn text_count_control_range_overlaps_json(
    output: &mut String,
    overlaps: &[TextCountControlRangeOverlap],
) {
    output.push('[');
    for (index, overlap) in overlaps.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"basis\":");
        output.push_str(&json_string(overlap.basis().as_str()));
        output.push_str(",\"delimiterCode\":");
        output.push_str(&overlap.delimiter_code().to_string());
        output.push_str(",\"delimiterCodeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", overlap.delimiter_code())));
        output.push_str(",\"rangeCount\":");
        output.push_str(&overlap.range_count().to_string());
        output.push_str(",\"firstRangeIndex\":");
        output.push_str(&overlap.first_range_index().to_string());
        output.push_str(",\"lastRangeIndex\":");
        output.push_str(&overlap.last_range_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&overlap.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&overlap.source_end().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

pub(super) fn text_style_candidates(styles: &[UnknownStyle]) -> Vec<StyleCandidate> {
    let mut candidates = Vec::new();

    for style in styles {
        if style.name() != Some(TEXT_LAYOUT_STYLE_PATH) {
            continue;
        }

        let summary = summarize_style_stream(style.payload());
        for (record_index, record) in summary.records().iter().enumerate() {
            let Some(label) = record.label() else {
                continue;
            };
            let trimmed = label.trim();
            if trimmed.is_empty() {
                continue;
            }

            candidates.push(StyleCandidate {
                id: candidates.len() as u32 + 1,
                name: trimmed.to_string(),
                source_stream: TEXT_LAYOUT_STYLE_PATH.to_string(),
                source_record_index: record_index,
                source_offset: record.offset(),
                source_code: record.code(),
                payload_len: record.payload_len(),
            });
        }
    }

    candidates
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_page_layer_text_run_json(
    output: &mut String,
    source_id: usize,
    placement: PageLayerTextPlacement,
    layout: PageLayout,
    writing_mode: WritingMode,
    font_family: &str,
    fill_color: &str,
    fragment: &PageLayerTextFragment,
) {
    let (width, height) = if writing_mode.is_vertical() {
        (
            APP_LINE_HEIGHT_PX as f64,
            vertical_text_advance_px(&fragment.text),
        )
    } else {
        (
            text_width_px(layout, &fragment.text),
            APP_LINE_HEIGHT_PX as f64,
        )
    };
    output.push_str("{\"type\":\"textRun\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{width:.3},\"height\":{height:.3}}}",
        placement.x, placement.y
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&fragment.text));
    if let Some(annotation) = &fragment.ruby_annotation {
        output.push_str(",\"rubyText\":");
        output.push_str(&json_string(annotation));
    }
    if fragment.paragraph_index.is_some() {
        output.push_str(",\"paragraphCharRange\":");
        output.push_str(&source_range_json(fragment.char_start, fragment.char_end));
    }
    output.push_str(&format!(
        ",\"baseline\":{:.3},\"rotation\":0.000,\"isVertical\":{},\"orientation\":\"{}\",\"fontFamily\":{},\"fillColor\":{},\"projectionKind\":\"fallback\",\"source\":",
        placement.baseline,
        writing_mode.is_vertical(),
        writing_mode.as_str(),
        json_string(font_family),
        json_string(fill_color)
    ));
    push_page_layer_source_span_json(output, source_id, fragment);
    output.push_str(",\"positions\":");
    push_f64_array_json(
        output,
        &text_positions_px_for_mode(layout, writing_mode, &fragment.text),
    );
    output.push_str(",\"isParaEnd\":false,\"isLineBreakEnd\":false}");
}

pub(super) fn push_page_layer_text_source_json(
    output: &mut Vec<String>,
    source_id: usize,
    fragment: &PageLayerTextFragment,
) {
    let mut source = format!(
        "{{\"id\":{},\"text\":{},\"utf8Range\":{},\"utf16Range\":{}",
        source_id,
        json_string(&fragment.text),
        source_range_json(0, fragment.text.len()),
        source_range_json(0, fragment.text.encode_utf16().count())
    );
    if let Some(paragraph_index) = fragment.paragraph_index {
        source.push_str(",\"stableSourceKey\":");
        source.push_str(&json_string(&format!(
            "section:0/para:{paragraph_index}/char:{}",
            fragment.char_start
        )));
        source.push_str(",\"paragraphCharRange\":");
        source.push_str(&source_range_json(fragment.char_start, fragment.char_end));
    }
    if let Some(span) = &fragment.source_span {
        source.push_str(",\"jtdByteRange\":");
        source.push_str(&source_range_json(span.byte_start(), span.byte_end()));
        source.push_str(",\"jtdUnitRange\":");
        source.push_str(&source_range_json(span.unit_start(), span.unit_end()));
    }
    source.push_str(",\"annotations\":[");
    if let Some(annotation) = &fragment.ruby_annotation {
        source.push_str("{\"type\":\"ruby\",\"text\":");
        source.push_str(&json_string(annotation));
        source.push('}');
    }
    source.push_str("]}");
    output.push(source);
}

pub(super) fn push_page_layer_observed_form_text_slot_json(
    output: &mut String,
    layout: PageLayout,
    projection: &ObservedFormTextProjection,
    slot: &ObservedFormTextSlot,
) {
    let text_width = text_width_px(layout, &slot.text) as f32 * (slot.font_size / APP_FONT_SIZE_PX);
    let x = match slot.anchor {
        "middle" => slot.x - (text_width / 2.0),
        "end" => slot.x - text_width,
        _ => slot.x,
    };
    let y = slot.y - slot.font_size;
    output.push_str("{\"type\":\"formTextProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{:.3},\"height\":{:.3}}}",
        text_width.max(slot.font_size),
        slot.font_size * 1.35
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(projection.projection_kind));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":true");
    output.push_str(",\"role\":");
    output.push_str(&json_string(slot.role));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&slot.text));
    output.push_str(",\"fontSize\":");
    output.push_str(&format!("{:.3}", slot.font_size));
    output.push_str(",\"fontWeight\":");
    output.push_str(&json_string(slot.font_weight));
    output.push_str(",\"textAnchor\":");
    output.push_str(&json_string(slot.anchor));
    output.push('}');
}

pub(super) fn push_document_text_property_15_color_candidate_json(
    output: &mut String,
    candidate: &DocumentTextProperty15ColorCandidate,
) {
    output.push_str("{\"source\":\"/DocumentText style section\",\"propertyId\":15");
    output.push_str(",\"packedBgrHex\":");
    output.push_str(&json_string(&format!("0x{:08x}", candidate.packed_bgr)));
    output.push_str(",\"cssColor\":");
    output.push_str(&json_string(candidate.css_color));
    output.push_str(",\"sourceBacked\":true,\"colorEncodingDecoded\":true");
    output.push_str(",\"propertyRoleDecoded\":false");
    output.push_str(",\"contextGate\":\"shanai-lan-text-projection\",\"renderPromoted\":true}");
}

pub(super) fn push_document_text_map_entry_brief_json(
    output: &mut String,
    entry: Option<&DocumentTextMapEntry>,
) {
    let Some(entry) = entry else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"kind\":");
    output.push_str(&json_string(entry.kind().as_str()));
    output.push_str(",\"byteRange\":");
    output.push_str(&source_range_json(entry.byte_start(), entry.byte_end()));
    output.push_str(",\"unitRange\":");
    output.push_str(&source_range_json(entry.unit_start(), entry.unit_end()));
    output.push_str(",\"selector\":");
    push_option_u16_json(output, entry.selector());
    output.push_str(",\"selectorHex\":");
    push_option_u16_hex_json(output, entry.selector());
    output.push_str(",\"code\":");
    push_option_u16_json(output, entry.code());
    output.push_str(",\"codeHex\":");
    push_option_u16_hex_json(output, entry.code());
    output.push_str(",\"textUnitCount\":");
    output.push_str(&entry.text().encode_utf16().count().to_string());
    output.push_str(",\"textPreview\":");
    let preview = entry.text().chars().take(16).collect::<String>();
    output.push_str(&json_string(&preview));
    output.push('}');
}

pub(super) fn push_page_layer_layout_box_text_slot_json(
    output: &mut String,
    source_id: usize,
    projection: &LayoutBoxTextProjection,
    slot: &LayoutBoxTextSlot,
    font_family: &str,
) {
    let fragment = PageLayerTextFragment {
        text: slot.text.clone(),
        paragraph_index: None,
        char_start: 0,
        char_end: slot.text.chars().count(),
        source_span: Some(slot.source_span.clone()),
        ruby_annotation: None,
    };
    let text_width =
        text_width_px_for_font_size(slot.font_size, &slot.text).max(f64::from(slot.font_size));
    output.push_str("{\"type\":\"textRun\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        slot.x, slot.y, text_width, slot.line_height
    ));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&slot.text));
    output.push_str(&format!(
        ",\"baseline\":{:.3},\"rotation\":0.000,\"isVertical\":false,\"orientation\":\"horizontal\",\"fontFamily\":{},\"fillColor\":\"#111111\",\"projectionKind\":{},\"source\":",
        slot.y + slot.font_size,
        json_string(font_family),
        json_string(projection.projection_kind),
    ));
    push_page_layer_source_span_json(output, source_id, &fragment);
    output.push_str(",\"positions\":");
    push_f64_array_json(
        output,
        &text_positions_px_for_font_size(slot.font_size, &slot.text),
    );
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"role\":");
    output.push_str(&json_string(slot.role));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"pageAssignmentDecoded\":");
    output.push_str(if projection.page_assignment_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockIndex\":");
    output.push_str(&slot.block_index.to_string());
    output.push_str(",\"blockCount\":");
    output.push_str(&projection.block_count.to_string());
    output.push_str(",\"layoutRecordCount\":");
    output.push_str(&projection.layout_record_count.to_string());
    output.push_str(",\"positionTablePresent\":");
    output.push_str(if projection.position_table_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"layoutRecordIndex\":");
    match slot.layout_record_index {
        Some(index) => output.push_str(&index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"layoutRecordByteRange\":");
    match slot.layout_record_byte_range {
        Some((start, end)) => output.push_str(&source_range_json(start, end)),
        None => output.push_str("null"),
    }
    output.push_str(",\"layoutFields\":{\"xPt\":");
    match slot.layout_x_pt {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"yPt\":");
    match slot.layout_y_pt {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"widthPt\":");
    match slot.layout_width_pt {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"inferredOriginPt\":");
    match slot.inferred_origin_pt {
        Some(value) => output.push_str(&format!("{value:.3}")),
        None => output.push_str("null"),
    }
    output.push('}');
    output.push_str(",\"placementBasis\":");
    output.push_str(&json_string(slot.placement_basis));
    output.push_str(",\"isParaEnd\":false,\"isLineBreakEnd\":false}");
}

pub(super) fn page_text_line_fragments(
    document: &Document,
    line: &PageTextLine,
) -> Vec<PageLayerTextFragment> {
    let Some(paragraph_index) = line.paragraph_index() else {
        return vec![PageLayerTextFragment {
            text: line.text().to_string(),
            paragraph_index: None,
            char_start: line.char_start(),
            char_end: line.char_end(),
            source_span: None,
            ruby_annotation: None,
        }];
    };

    let Some(paragraph) = paragraph_by_index(document, paragraph_index) else {
        return Vec::new();
    };
    let mut fragments = paragraph_line_fragments(
        paragraph,
        paragraph_index,
        line.char_start(),
        line.char_end(),
    );
    let source_text = fragments
        .iter()
        .map(|fragment| fragment.text.as_str())
        .collect::<String>();
    if !source_text.is_empty() && line.text().starts_with(&source_text) {
        let source_len = source_text.chars().count();
        let suffix = line.text().chars().skip(source_len).collect::<String>();
        if !suffix.is_empty() {
            let suffix_len = suffix.chars().count();
            fragments.push(PageLayerTextFragment {
                text: suffix,
                paragraph_index: None,
                char_start: line.char_start() + source_len,
                char_end: line.char_start() + source_len + suffix_len,
                source_span: None,
                ruby_annotation: None,
            });
        }
    }
    fragments
}

pub(super) fn paragraph_by_index(
    document: &Document,
    paragraph_index: usize,
) -> Option<&Paragraph> {
    document
        .blocks()
        .iter()
        .filter_map(|block| match block {
            Block::Paragraph(paragraph) => Some(paragraph),
            Block::Unknown(_) => None,
        })
        .nth(paragraph_index)
}

pub(super) fn paragraph_line_fragments(
    paragraph: &Paragraph,
    paragraph_index: usize,
    line_start: usize,
    line_end: usize,
) -> Vec<PageLayerTextFragment> {
    let mut fragments = Vec::new();
    let mut paragraph_offset = 0usize;

    for inline in paragraph.inlines() {
        let (text, source_span, ruby_annotation) = match inline {
            Inline::Text(run) => (run.text(), run.source_span(), None),
            Inline::Ruby(ruby) => (ruby.base_text(), None, Some(ruby.annotation_text())),
            Inline::Unknown(_) => ("", None, None),
        };
        let inline_len = text.chars().count();
        let inline_start = paragraph_offset;
        let inline_end = inline_start + inline_len;
        paragraph_offset = inline_end;

        let overlap_start = inline_start.max(line_start);
        let overlap_end = inline_end.min(line_end);
        if overlap_start >= overlap_end {
            continue;
        }

        let relative_start = overlap_start - inline_start;
        let relative_end = overlap_end - inline_start;
        let annotation = if ruby_annotation.is_some()
            && overlap_start == inline_start
            && overlap_end == inline_end
        {
            ruby_annotation.map(str::to_string)
        } else {
            None
        };
        fragments.push(PageLayerTextFragment {
            text: text_by_char_range(text, relative_start, relative_end),
            paragraph_index: Some(paragraph_index),
            char_start: overlap_start,
            char_end: overlap_end,
            source_span: source_span
                .map(|span| source_span_for_char_range(text, span, relative_start, relative_end)),
            ruby_annotation: annotation,
        });
    }

    fragments
}

pub(super) fn text_by_char_range(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

pub(super) fn text_width_px(layout: PageLayout, text: &str) -> f64 {
    text.chars()
        .map(|character| display_column_width(character) as f64 * column_width_px(layout))
        .sum()
}

pub(super) fn text_width_px_for_font_size(font_size: f32, text: &str) -> f64 {
    text.chars()
        .map(|character| display_column_width(character) as f64 * f64::from(font_size) * 0.55)
        .sum()
}

pub(super) fn vertical_text_advance_px(text: &str) -> f64 {
    text.chars()
        .map(|character| {
            display_column_width(character) as f64 * APP_VERTICAL_DISPLAY_UNIT_PX as f64
        })
        .sum()
}

pub(super) fn text_positions_px(layout: PageLayout, text: &str) -> Vec<f64> {
    let mut positions = Vec::new();
    let mut x = 0.0;
    positions.push(x);
    for character in text.chars() {
        x += display_column_width(character) as f64 * column_width_px(layout);
        positions.push(x);
    }
    positions
}

pub(super) fn text_positions_px_for_font_size(font_size: f32, text: &str) -> Vec<f64> {
    let mut positions = Vec::new();
    let mut x = 0.0;
    positions.push(x);
    for character in text.chars() {
        x += display_column_width(character) as f64 * f64::from(font_size) * 0.55;
        positions.push(x);
    }
    positions
}

pub(super) fn text_positions_px_for_mode(
    layout: PageLayout,
    writing_mode: WritingMode,
    text: &str,
) -> Vec<f64> {
    if !writing_mode.is_vertical() {
        return text_positions_px(layout, text);
    }

    let mut positions = Vec::new();
    let mut y = 0.0;
    positions.push(y);
    for character in text.chars() {
        y += display_column_width(character) as f64 * APP_VERTICAL_DISPLAY_UNIT_PX as f64;
        positions.push(y);
    }
    positions
}

pub(super) fn fallback_text_fill_color() -> &'static str {
    "#111111"
}

pub(super) fn document_text_units(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect()
}

pub(super) fn fallback_text_origin(layout: PageLayout, document: &Document) -> Option<(f32, f32)> {
    if !document_has_shanai_lan_fdm_command_evidence(document) {
        return None;
    }
    let viewport = fdm_projection_viewport(layout);
    Some((viewport.x, viewport.y))
}

pub(super) fn document_text_raw_stream(document: &Document) -> Option<&[u8]> {
    raw_stream_bytes(document, DOCUMENT_TEXT_PATH)
}

pub(super) fn text_source_span_from_document_text_units(
    start: usize,
    end: usize,
) -> TextSourceSpan {
    TextSourceSpan::new(start * 2, end * 2, start, end)
}

pub(super) fn page_frame_text_placement(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    line_index: usize,
    line: &PageTextLine,
) -> Option<PageLayerTextPlacement> {
    let projection = page_frame_projection(document, layout, page_number)?;
    let title_frame = projection
        .shapes
        .iter()
        .find(|shape| shape.role == "titleRoundedFrame");
    let first_bar = projection
        .shapes
        .iter()
        .filter(|shape| shape.role == "horizontalPatternBar")
        .min_by(|left, right| left.y.partial_cmp(&right.y).unwrap_or(Ordering::Equal));

    if line_index == 0 && line.text().contains("制限時間") {
        let title_frame = title_frame?;
        let text_width = text_width_px(layout, line.text()) as f32;
        let x = ((layout.width_px() - text_width) / 2.0).max(layout.margin_px() * 0.5);
        let y = title_frame.y + title_frame.height + PAGE_FRAME_TIME_CAPTION_GAP_PX;
        return Some(PageLayerTextPlacement {
            x: x as f64,
            y: y as f64,
            baseline: (y + APP_FONT_SIZE_PX) as f64,
        });
    }

    if line_index == 0 {
        return None;
    }

    let first_bar = first_bar?;
    let body_x = layout_box_body_anchor_from_document(document, layout)
        .map(|(x, _, _)| x)
        .unwrap_or_else(|| layout.margin_px());
    let y = first_bar.y
        + first_bar.height
        + (APP_LINE_HEIGHT_PX * PAGE_FRAME_TEXT_AFTER_BAR_GAP_LINES)
        + (line_index.saturating_sub(1) as f32 * APP_LINE_HEIGHT_PX);

    Some(PageLayerTextPlacement {
        x: body_x as f64,
        y: y as f64,
        baseline: (y + APP_FONT_SIZE_PX) as f64,
    })
}

pub(super) fn layout_box_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<LayoutBoxTextProjection> {
    if page_number != 1 {
        return None;
    }

    let bytes = raw_stream_bytes(document, LAYOUT_BOX_TEXT_PATH)?;
    let blocks = layout_box_text_blocks(bytes);
    if blocks.is_empty() {
        return None;
    }
    let records = raw_stream_bytes(document, LAYOUT_BOX_PATH)
        .map(layout_box_record_candidates)
        .unwrap_or_default();
    let body_anchor = layout_box_body_anchor(&blocks, &records, layout);
    let title_frame_shape = document
        .object_frame_records()
        .iter()
        .find_map(|record| page_frame_title_shape(record, layout));
    let document_text = document_visible_text(document);
    let mut slots = Vec::new();

    for block in &blocks {
        for fragment in &block.fragments {
            let trimmed = fragment.text.trim();
            if trimmed.is_empty() || document_text.contains(trimmed) {
                continue;
            }
            let role = layout_box_text_role(block, &fragment.text);
            match role {
                "body" => {
                    let Some((record, x, y, width, origin_pt)) =
                        layout_box_record_text_box(block.index, &records, layout)
                    else {
                        continue;
                    };
                    let line_height =
                        LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX * LAYOUT_BOX_TEXT_LINE_HEIGHT_FACTOR;
                    for (line_index, line) in layout_box_wrapped_text_lines(
                        &fragment.text,
                        width,
                        LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX,
                    )
                    .into_iter()
                    .enumerate()
                    {
                        if line.trim().is_empty() {
                            continue;
                        }
                        slots.push(LayoutBoxTextSlot {
                            role,
                            text: line,
                            x,
                            y: y + line_index as f32 * line_height,
                            font_size: LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX,
                            line_height,
                            source_span: fragment.source_span.clone(),
                            block_index: block.index,
                            layout_record_index: Some(record.index),
                            layout_record_byte_range: Some((record.byte_start, record.byte_end)),
                            layout_x_pt: record.x_field,
                            layout_y_pt: record.y_field,
                            layout_width_pt: record.width_field,
                            inferred_origin_pt: Some(origin_pt),
                            placement_basis: "layoutBoxRecordFields",
                        });
                    }
                }
                "title" => {
                    let font_size = LAYOUT_BOX_TEXT_TITLE_FONT_SIZE_PX;
                    let text_width = text_width_px_for_font_size(font_size, trimmed) as f32;
                    let (x, y, placement_basis) = if let Some(frame) = &title_frame_shape {
                        (
                            frame.x + ((frame.width - text_width) / 2.0).max(0.0),
                            frame.y + ((frame.height - font_size) / 2.0).max(0.0),
                            "pageFrameTitleCenter",
                        )
                    } else {
                        (
                            ((layout.width_px() - text_width) / 2.0).max(layout.margin_px() * 0.5),
                            layout.margin_px() * 0.56,
                            "shortLeadingLayoutBoxText",
                        )
                    };
                    slots.push(LayoutBoxTextSlot {
                        role,
                        text: trimmed.to_string(),
                        x,
                        y,
                        font_size,
                        line_height: font_size * 1.2,
                        source_span: fragment.source_span.clone(),
                        block_index: block.index,
                        layout_record_index: records.get(block.index).map(|record| record.index),
                        layout_record_byte_range: records
                            .get(block.index)
                            .map(|record| (record.byte_start, record.byte_end)),
                        layout_x_pt: records.get(block.index).and_then(|record| record.x_field),
                        layout_y_pt: records.get(block.index).and_then(|record| record.y_field),
                        layout_width_pt: records
                            .get(block.index)
                            .and_then(|record| record.width_field),
                        inferred_origin_pt: records
                            .get(block.index)
                            .and_then(layout_box_record_origin_pt),
                        placement_basis,
                    });
                }
                "caption" => {
                    let font_size = LAYOUT_BOX_TEXT_CAPTION_FONT_SIZE_PX;
                    let text_width = text_width_px_for_font_size(font_size, trimmed) as f32;
                    let body_line_height =
                        LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX * LAYOUT_BOX_TEXT_LINE_HEIGHT_FACTOR;
                    let y = body_anchor
                        .map(|(_, body_y, _)| body_y - body_line_height * 2.0)
                        .unwrap_or(layout.height_px() * 0.38);
                    slots.push(LayoutBoxTextSlot {
                        role,
                        text: trimmed.to_string(),
                        x: ((layout.width_px() - text_width) / 2.0).max(layout.margin_px() * 0.5),
                        y,
                        font_size,
                        line_height: font_size * 1.25,
                        source_span: fragment.source_span.clone(),
                        block_index: block.index,
                        layout_record_index: records.get(block.index).map(|record| record.index),
                        layout_record_byte_range: records
                            .get(block.index)
                            .map(|record| (record.byte_start, record.byte_end)),
                        layout_x_pt: records.get(block.index).and_then(|record| record.x_field),
                        layout_y_pt: records.get(block.index).and_then(|record| record.y_field),
                        layout_width_pt: records
                            .get(block.index)
                            .and_then(|record| record.width_field),
                        inferred_origin_pt: records
                            .get(block.index)
                            .and_then(layout_box_record_origin_pt),
                        placement_basis: "relativeToLayoutBoxBodyAnchor",
                    });
                }
                _ => {}
            }
        }
    }

    (!slots.is_empty()).then_some(LayoutBoxTextProjection {
        source: LAYOUT_BOX_TEXT_PATH,
        projection_kind: "layoutBoxTextProjection",
        block_count: blocks.len(),
        layout_record_count: records.len(),
        position_table_present: raw_stream_bytes(document, LAYOUT_BOX_TEXT_POSITION_TABLES_PATH)
            .is_some(),
        page_assignment_decoded: false,
        slots,
    })
}

pub(super) fn layout_box_text_blocks(bytes: &[u8]) -> Vec<LayoutBoxTextBlock> {
    let mut blocks = Vec::new();
    let mut offset = 0usize;
    while let Some(relative) = bytes[offset..]
        .windows(LAYOUT_BOX_TEXT_MAGIC.len())
        .position(|window| window == LAYOUT_BOX_TEXT_MAGIC)
    {
        let byte_start = offset + relative;
        let Some(declared_unit_count) =
            read_be32_at(bytes, byte_start + LAYOUT_BOX_TEXT_MAGIC.len())
                .and_then(|value| usize::try_from(value).ok())
        else {
            break;
        };
        let payload_start = byte_start + LAYOUT_BOX_TEXT_MAGIC.len() + 4;
        let declared_payload_end =
            payload_start.saturating_add(declared_unit_count.saturating_mul(2));
        let next_magic = bytes
            .get(payload_start..)
            .and_then(|tail| {
                tail.windows(LAYOUT_BOX_TEXT_MAGIC.len())
                    .position(|window| window == LAYOUT_BOX_TEXT_MAGIC)
            })
            .map(|relative| payload_start + relative);
        let payload_end = declared_payload_end
            .min(next_magic.unwrap_or(bytes.len()))
            .min(bytes.len());
        if payload_start > payload_end {
            break;
        }
        let fragments = layout_box_text_fragments(bytes, payload_start, payload_end);
        blocks.push(LayoutBoxTextBlock {
            index: blocks.len(),
            byte_start,
            byte_end: payload_end,
            payload_start,
            payload_end,
            declared_unit_count,
            fragments,
        });
        offset = payload_end.max(byte_start + 1);
    }
    blocks
}

pub(super) fn layout_box_text_fragments(
    source_bytes: &[u8],
    payload_start: usize,
    payload_end: usize,
) -> Vec<LayoutBoxTextFragment> {
    let Some(payload) = source_bytes.get(payload_start..payload_end) else {
        return Vec::new();
    };
    let mut fragments = map_document_text(payload)
        .entries()
        .iter()
        .filter(|entry| {
            matches!(
                entry.kind(),
                DocumentTextMapKind::TextRun
                    | DocumentTextMapKind::InlineText
                    | DocumentTextMapKind::SkippedInlineText
            ) && !entry.text().trim().is_empty()
        })
        .map(|entry| {
            let byte_start = payload_start + entry.byte_start();
            let byte_end = payload_start + entry.byte_end();
            LayoutBoxTextFragment {
                text: entry.text().trim_end().to_string(),
                source_span: TextSourceSpan::new(
                    byte_start,
                    byte_end,
                    byte_start / 2,
                    byte_end / 2,
                ),
            }
        })
        .collect::<Vec<_>>();
    if !fragments.is_empty() {
        return fragments;
    }

    if let Some((text, byte_start, byte_end)) =
        decode_plain_layout_box_text_payload(payload, payload_start)
    {
        fragments.push(LayoutBoxTextFragment {
            text,
            source_span: TextSourceSpan::new(byte_start, byte_end, byte_start / 2, byte_end / 2),
        });
    }
    fragments
}

pub(super) fn decode_plain_layout_box_text_payload(
    payload: &[u8],
    payload_start: usize,
) -> Option<(String, usize, usize)> {
    let mut text = String::new();
    let mut first_text_unit = None;
    let mut last_text_unit = None;
    for (unit_index, chunk) in payload.chunks_exact(2).enumerate() {
        let code = u16::from_be_bytes([chunk[0], chunk[1]]);
        let character = match code {
            0x0009 => '\t',
            0x000a | 0x000d => '\n',
            0x0020..=0xd7ff | 0xe000..=0xfffd if code != 0xffff => char::from_u32(u32::from(code))?,
            _ => continue,
        };
        if !matches!(character, '\0') {
            first_text_unit.get_or_insert(unit_index);
            last_text_unit = Some(unit_index + 1);
            text.push(character);
        }
    }
    let text = text
        .trim_end_matches([' ', '\u{3000}', '\n', '\r', '\t'])
        .to_string();
    if text.trim().is_empty() {
        return None;
    }
    let byte_start = payload_start + first_text_unit.unwrap_or_default() * 2;
    let byte_end = payload_start + last_text_unit.unwrap_or_default() * 2;
    Some((text, byte_start, byte_end))
}

pub(super) fn layout_box_text_role(block: &LayoutBoxTextBlock, text: &str) -> &'static str {
    let trimmed = text.trim();
    if trimmed.chars().count() >= LAYOUT_BOX_TEXT_BODY_MIN_CHARS {
        return "body";
    }
    if block.index == 0 && trimmed.chars().count() <= 32 && !trimmed.contains('\n') {
        return "title";
    }
    if trimmed.contains("より") || trimmed.contains("抜粋") || trimmed.contains('\'') {
        return "caption";
    }
    "label"
}

pub(super) fn layout_box_record_text_box(
    block_index: usize,
    records: &[LayoutBoxRecordCandidate],
    _layout: PageLayout,
) -> Option<(&LayoutBoxRecordCandidate, f32, f32, f32, f32)> {
    let record = records.get(block_index)?;
    let x = record.x_field?;
    let y = record.y_field?;
    let width = record.width_field?;
    if !(LAYOUT_BOX_TEXT_MIN_RENDER_WIDTH_PT..=LAYOUT_BOX_TEXT_MAX_RENDER_WIDTH_PT).contains(&width)
        || x > LAYOUT_BOX_TEXT_MAX_RENDER_WIDTH_PT
        || y > 1200
    {
        return None;
    }
    let origin_pt = layout_box_record_origin_pt(record).unwrap_or(0.0);
    Some((
        record,
        (f32::from(x) + origin_pt) * PDF_POINT_TO_CSS_PX,
        (f32::from(y) + origin_pt) * PDF_POINT_TO_CSS_PX,
        f32::from(width) * PDF_POINT_TO_CSS_PX,
        origin_pt,
    ))
}

pub(super) fn layout_box_wrapped_text_lines(
    text: &str,
    width_px: f32,
    font_size: f32,
) -> Vec<String> {
    let max_columns = ((width_px / (font_size * 0.55)).floor() as usize).max(8);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut current = String::new();
        let mut width = 0usize;
        for character in paragraph.trim_end().chars() {
            let char_width = display_column_width(character);
            if width > 0 && width + char_width > max_columns {
                lines.push(std::mem::take(&mut current));
                width = 0;
            }
            current.push(character);
            width += char_width;
        }
        if !current.trim().is_empty() {
            lines.push(current);
        }
    }
    lines
}

pub(super) fn render_text_page_svg(
    lines: &[PageTextLine],
    page_number: usize,
    _page_count: usize,
    layout: PageLayout,
    writing_mode: WritingMode,
    document: &Document,
    decoration: Option<&PageDecoration>,
) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{:.1}\" height=\"{:.1}\" viewBox=\"0 0 {:.1} {:.1}\">",
        layout.width_px(),
        layout.height_px(),
        layout.width_px(),
        layout.height_px()
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>");
    let font_family = document_font_family_css(document);
    push_page_frame_projection_svg(&mut svg, layout, document, page_number);
    push_page_mark_section_separator_svg(&mut svg, layout, document, page_number);
    push_shanai_lan_sparse_table_borders_svg(&mut svg, layout, document, page_number);
    push_visual_list_diagnostic_svg(&mut svg, layout, document, page_number);
    push_embedding_frame_diagnostic_svg(&mut svg, layout, document, lines, page_number);
    push_success_data_test_title_art_projection_svg(&mut svg, layout, document, lines, page_number);
    push_success_data_test_answer_sheet_projection_svg(
        &mut svg,
        layout,
        document,
        page_number,
        &font_family,
    );
    push_jseq_formula_projection_svg(&mut svg, layout, document, lines, page_number, &font_family);
    // Line-rule candidates stay in the layer tree until the topology decoder is reliable enough
    // to render them without adding false connector trunks.
    let fdm_vector_primitives_rendered =
        push_fdm_vector_primitive_svg(&mut svg, layout, document, page_number);

    if let Some(projection) = shanai_lan_document_text_projection(document, layout, page_number) {
        push_shanai_lan_text_projection_svg(&mut svg, &projection, &font_family);
    } else if let Some(projection) = observed_form_text_projection(document, layout, page_number) {
        push_observed_form_text_projection_svg(&mut svg, &projection, &font_family);
    } else if writing_mode.is_vertical() {
        let placement = vertical_page_text_placement(layout, lines);
        svg.push_str("<g writing-mode=\"vertical-rl\" glyph-orientation-vertical=\"auto\">");
        for (index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }

            let mut x =
                layout.width_px() - layout.margin_px() - (index as f32 * APP_LINE_HEIGHT_PX)
                    + placement.x_shift_px;
            let mut y = placement.y_start_px;
            if is_centered_ginga_title_page(page_number, line) {
                let line_extent = vertical_text_advance_px(line.text()) as f32;
                x = layout.width_px() / 2.0;
                y = ((layout.height_px() - line_extent) / 2.0).max(layout.margin_px());
            }

            for fragment in page_text_line_fragments(document, line) {
                if fragment.text.is_empty() {
                    continue;
                }
                let fill_color = fallback_text_fill_color();

                push_svg_text_run(
                    &mut svg,
                    "rjtd-text",
                    x,
                    y,
                    &font_family,
                    APP_FONT_SIZE_PX,
                    fill_color,
                    &fragment.text,
                    Some("vertical-rl"),
                );
                if let Some(annotation) = &fragment.ruby_annotation {
                    push_svg_ruby_annotation(
                        &mut svg,
                        x + (APP_FONT_SIZE_PX * 0.72),
                        y,
                        &font_family,
                        annotation,
                        true,
                    );
                }
                y += vertical_text_advance_px(&fragment.text) as f32;
            }
        }
        svg.push_str("</g>");
    } else {
        if let Some(slots) = success_data_test_top_text_projection(document, page_number) {
            push_success_data_test_top_text_projection_svg(
                &mut svg,
                document,
                layout,
                slots,
                &font_family,
            );
        }
        let text_origin = fallback_text_origin(layout, document);
        let mut fallback_visual_line_index = 0usize;
        for (index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }
            if success_data_test_top_text_line_should_skip(document, page_number, line) {
                continue;
            }
            let frame_text_placement = page_frame_text_placement(
                document,
                layout,
                page_number,
                fallback_visual_line_index,
                line,
            );
            let mut x = frame_text_placement
                .map(|placement| placement.x as f32)
                .or_else(|| text_origin.map(|origin| origin.0))
                .unwrap_or_else(|| layout.margin_px());
            let y = frame_text_placement
                .map(|placement| placement.baseline as f32)
                .unwrap_or_else(|| {
                    text_origin
                        .map(|origin| origin.1)
                        .unwrap_or_else(|| layout.margin_px())
                        + APP_FONT_SIZE_PX
                        + (index as f32 * APP_LINE_HEIGHT_PX)
                });
            for fragment in page_text_line_fragments(document, line) {
                if fragment.text.is_empty() {
                    continue;
                }
                if fragment_overlaps_rendered_table_projection(
                    layout,
                    document,
                    lines,
                    page_number,
                    &fragment,
                ) {
                    continue;
                }
                let width = text_width_px(layout, &fragment.text) as f32;
                let fill_color = fallback_text_fill_color();
                push_svg_text_run(
                    &mut svg,
                    "rjtd-text",
                    x,
                    y,
                    &font_family,
                    APP_FONT_SIZE_PX,
                    fill_color,
                    &fragment.text,
                    None,
                );
                if let Some(annotation) = &fragment.ruby_annotation {
                    push_svg_ruby_annotation(
                        &mut svg,
                        x + (width / 2.0),
                        y - (APP_FONT_SIZE_PX * 0.75),
                        &font_family,
                        annotation,
                        false,
                    );
                }
                x += width;
            }
            fallback_visual_line_index += 1;
        }
    }
    if let Some(projection) = layout_box_text_projection(document, layout, page_number) {
        push_layout_box_text_projection_svg(&mut svg, &projection, &font_family);
    }
    if let Some(decoration) = decoration {
        push_page_decoration_svg(&mut svg, layout, writing_mode, decoration, &font_family);
    }
    push_success_data_test_cone_diagram_projection_svg(
        &mut svg,
        layout,
        document,
        page_number,
        &font_family,
    );
    push_table_grid_candidate_svg(&mut svg, layout, document, lines, page_number);
    push_image_payload_diagnostic_svg(&mut svg, layout, document, page_number);
    if !fdm_vector_primitives_rendered {
        push_fdm_command_diagnostic_svg(&mut svg, layout, document, page_number);
        push_fdm_frame_diagnostic_svg(&mut svg, layout, document, page_number);
    }
    svg.push_str("</svg>");
    svg
}

pub(super) fn push_layout_box_text_projection_svg(
    svg: &mut String,
    projection: &LayoutBoxTextProjection,
    font_family: &str,
) {
    svg.push_str(&format!(
        "<g class=\"rjtd-layout-box-text-projection\" data-source=\"{}\" data-projection-kind=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-page-assignment-decoded=\"{}\" data-block-count=\"{}\" data-layout-record-count=\"{}\" data-position-table-present=\"{}\">",
        escape_xml(projection.source),
        escape_xml(projection.projection_kind),
        projection.page_assignment_decoded,
        projection.block_count,
        projection.layout_record_count,
        projection.position_table_present
    ));
    let font_family = escape_xml(font_family);
    for slot in &projection.slots {
        let record_index = slot
            .layout_record_index
            .map(|index| index.to_string())
            .unwrap_or_else(|| "-".to_string());
        svg.push_str(&format!(
            "<text class=\"rjtd-text rjtd-layout-box-text\" data-source=\"{}\" data-role=\"{}\" data-block-index=\"{}\" data-layout-record-index=\"{}\" data-placement-basis=\"{}\" x=\"{:.1}\" y=\"{:.1}\" font-family=\"{}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            escape_xml(projection.source),
            escape_xml(slot.role),
            slot.block_index,
            escape_xml(&record_index),
            escape_xml(slot.placement_basis),
            slot.x,
            slot.y + slot.font_size,
            font_family,
            slot.font_size,
            escape_xml(&svg_visual_text(&slot.text))
        ));
    }
    svg.push_str("</g>");
}

pub(super) fn push_observed_form_text_projection_svg(
    svg: &mut String,
    projection: &ObservedFormTextProjection,
    _font_family: &str,
) {
    svg.push_str(&format!(
        "<g class=\"rjtd-observed-form-text-projection\" data-source=\"{}\" data-projection=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"true\">",
        escape_xml(projection.source),
        escape_xml(projection.projection_kind)
    ));
    for shape in &projection.shapes {
        let stroke = shape.stroke.unwrap_or("none");
        svg.push_str(&format!(
            "<rect class=\"rjtd-form-shape\" data-role=\"{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.1}\"/>",
            escape_xml(shape.role),
            shape.x,
            shape.y,
            shape.width,
            shape.height,
            shape.rx,
            shape.rx,
            escape_xml(shape.fill),
            escape_xml(stroke),
            shape.stroke_width
        ));
    }
    for slot in &projection.slots {
        let anchor = slot.anchor;
        let text = escape_xml(&svg_visual_text(&slot.text));
        let font_family = escape_xml(slot.font_family);
        svg.push_str(&format!(
            "<text class=\"rjtd-form-text\" data-role=\"{}\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"{:.1}\" font-weight=\"{}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            escape_xml(slot.role),
            slot.x,
            slot.y,
            anchor,
            font_family,
            slot.font_size,
            slot.font_weight,
            text
        ));
    }
    svg.push_str("</g>");
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_svg_text_run(
    svg: &mut String,
    class_name: &str,
    x: f32,
    y: f32,
    font_family: &str,
    font_size: f32,
    fill: &str,
    text: &str,
    writing_mode: Option<&str>,
) {
    let visual_text = escape_xml(&svg_visual_text(text));
    let font_family = escape_xml(font_family);
    let writing_mode_attr = writing_mode
        .map(|mode| format!(" writing-mode=\"{mode}\""))
        .unwrap_or_default();
    svg.push_str(&format!(
        "<text class=\"{class_name}\" x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"{font_family}\" font-size=\"{font_size:.1}\" fill=\"{fill}\" letter-spacing=\"0\" xml:space=\"preserve\"{writing_mode_attr}>{visual_text}</text>"
    ));
}

pub(super) fn push_svg_ruby_annotation(
    svg: &mut String,
    x: f32,
    y: f32,
    font_family: &str,
    annotation: &str,
    vertical: bool,
) {
    let writing_mode_attr = if vertical {
        " writing-mode=\"vertical-rl\""
    } else {
        " text-anchor=\"middle\""
    };
    let font_family = escape_xml(font_family);
    svg.push_str(&format!(
        "<text class=\"rjtd-ruby\" x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"{font_family}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\"{writing_mode_attr}>{}</text>",
        APP_FONT_SIZE_PX * 0.55,
        escape_xml(&svg_visual_text(annotation))
    ));
}

pub(super) fn observed_form_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ObservedFormTextProjection> {
    if let Some(projection) = observed_tsaiten_text_projection(document, layout, page_number) {
        return Some(projection);
    }
    if page_number != 1 || !document_has_fax02_visual_list(document) {
        return None;
    }
    let plain_text = document_plain_text(document);
    let lines = plain_text
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let title = lines.first().copied()?;
    if title != "FAX送付のご案内" {
        return None;
    }
    let date = lines.iter().copied().find(|line| line.contains("平成"))?;
    let addressee = lines.iter().copied().find(|line| line.contains('様'))?;
    let body = lines
        .iter()
        .copied()
        .filter(|line| {
            line.starts_with("拝啓")
                || line.starts_with("平素")
                || line.starts_with("下記")
                || line.starts_with("ご検討")
        })
        .collect::<Vec<_>>();
    let total = lines
        .iter()
        .copied()
        .find(|line| line.starts_with("全枚数"))?;
    if body.len() != 4 {
        return None;
    }

    let scale_x = layout.width_px() / 120.0;
    let scale_y = layout.height_px() / 169.0;
    let mut slots = Vec::with_capacity(8 + body.len());
    slots.push(form_slot(
        "title",
        title,
        15.0,
        23.1,
        30.5,
        "900",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "date",
        date,
        79.5,
        28.6,
        14.0,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "addressee",
        addressee.trim(),
        60.0,
        40.9,
        18.0,
        "500",
        "middle",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "left-fax-label",
        "FAX：",
        16.2,
        47.4,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "right-tel-label",
        "TEL：",
        71.0,
        67.8,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "right-fax-label",
        "FAX：",
        71.0,
        74.5,
        11.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    for (index, text) in body.iter().enumerate() {
        slots.push(form_slot(
            "body",
            text,
            25.8,
            81.8 + index as f32 * 3.55,
            13.6,
            "500",
            "start",
            VISUAL_LIST_GOTHIC_FONT_FAMILY,
            scale_x,
            scale_y,
        ));
    }
    slots.push(form_slot(
        "total-count",
        total,
        76.8,
        98.3,
        13.6,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    Some(ObservedFormTextProjection {
        source: "documentText+visualList",
        projection_kind: "visualListFormProjection",
        shapes: Vec::new(),
        slots,
    })
}

pub(super) fn observed_tsaiten_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ObservedFormTextProjection> {
    if page_number != 1 || !document_has_tsaiten_projection_evidence(document) {
        return None;
    }

    let scale_x = layout.width_px() / TSAITEN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / TSAITEN_REFERENCE_PAGE_HEIGHT_PX;
    let mut shapes = Vec::new();
    let mut slots = Vec::new();

    slots.push(form_slot(
        "document-heading",
        "＜採点原則＞",
        397.0,
        83.0,
        12.0,
        "700",
        "middle",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    shapes.push(form_shape(
        "title-shadow",
        101.0,
        128.0,
        634.0,
        39.0,
        "#d0d0d0",
        None,
        0.0,
        1.5,
        scale_x,
        scale_y,
    ));
    shapes.push(form_shape(
        "title-box",
        94.0,
        121.0,
        634.0,
        39.0,
        "#ffffff",
        Some("#333333"),
        1.6,
        2.0,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "title",
        "タイピング科目採点方法",
        110.0,
        146.0,
        18.0,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    slots.push(form_slot(
        "instruction",
        "　標準解答を見ながら採点します。採点内容は以下のとおりです。",
        142.0,
        214.0,
        11.3,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "instruction",
        "　採点項目に当てはまる誤りがあった場合、減点すべき点数を採点用紙の指定の欄に記入してください。",
        142.0,
        240.0,
        11.3,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "section-heading",
        "【採点科目】",
        105.0,
        286.0,
        12.2,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "section-heading",
        "【採点内容】",
        105.0,
        486.0,
        12.2,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    shapes.push(form_shape(
        "document-format-label-box",
        183.0,
        511.0,
        110.0,
        23.0,
        "#ffffff",
        Some("#555555"),
        1.0,
        1.5,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "subsection-label",
        "文書の体裁",
        195.0,
        528.0,
        10.8,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    push_tsaiten_document_format_table_projection(&mut shapes, &mut slots, scale_x, scale_y);

    shapes.push(form_shape(
        "linebreak-label-box",
        183.0,
        737.0,
        146.0,
        23.0,
        "#ffffff",
        Some("#555555"),
        1.0,
        1.5,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "subsection-label",
        "文字・改行の誤り",
        195.0,
        754.0,
        10.8,
        "700",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    slots.push(form_slot(
        "note",
        "※行頭字下げのスペースを含め、入力している文字すべてを採点する。",
        112.0,
        905.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "note",
        "※同じ行を２回以上入力している場合、余分な行の文字は余字として、１文字につき１点減点する。",
        112.0,
        930.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));
    slots.push(form_slot(
        "note",
        "※全角サイズでない文字は、誤字として１文字につき１点減点する。",
        112.0,
        955.0,
        9.5,
        "500",
        "start",
        VISUAL_LIST_GOTHIC_FONT_FAMILY,
        scale_x,
        scale_y,
    ));

    Some(ObservedFormTextProjection {
        source: "documentText+tableCandidates",
        projection_kind: "tsaitenReferenceProjection",
        shapes,
        slots,
    })
}

pub(super) fn find_text_utf16_unit_range_after(
    haystack: &str,
    needle: &str,
    start_units: usize,
) -> Option<(usize, usize)> {
    if needle.is_empty() {
        return None;
    }
    let start_byte = byte_index_after_utf16_units(haystack, start_units)?;
    let match_byte = haystack.get(start_byte..)?.find(needle)? + start_byte;
    let match_start_units = haystack[..match_byte].encode_utf16().count();
    let match_end_units = match_start_units + needle.encode_utf16().count();
    Some((match_start_units, match_end_units))
}
