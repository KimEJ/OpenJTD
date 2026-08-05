use super::*;
use crate::*;

pub(crate) fn document_text_toc_entries(entries: &[DocumentTextMapEntry]) -> Vec<DocumentTocEntry> {
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

pub(crate) fn source_text_parts(
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

pub(crate) fn text_count_range_overlaps(
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

pub(crate) fn text_count_control_range_overlaps(
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

pub(crate) fn text_boundary_candidates_from_ranges(
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

pub(crate) fn document_text_control_table_row_is_compatible(
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

pub(crate) fn document_text_control_row_starts_with_short_label(
    row: &DocumentTextControlTableRow,
) -> bool {
    row.cells
        .first()
        .is_some_and(|cell| cell.text.chars().count() <= 2 && !cell.text.is_empty())
}

pub(crate) fn document_text_control_table_rows_are_plausible(
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

pub(crate) fn sparse_document_text_control_row_is_seed(row: &DocumentTextControlTableRow) -> bool {
    let shape = SparseDocumentTextControlRowShape::from_row(row);
    shape.column_count >= 3
        && shape.empty_cells > 0
        && shape.non_empty_cells > 0
        && shape.text_char_count <= 48
}

pub(crate) fn sparse_document_text_control_row_is_blank(row: &DocumentTextControlTableRow) -> bool {
    let shape = SparseDocumentTextControlRowShape::from_row(row);
    shape.column_count >= 2 && shape.non_empty_cells == 0
}

pub(crate) fn sparse_document_text_control_row_is_soft_separator(
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

pub(crate) fn sparse_document_text_control_table_rows_are_plausible(
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

pub(crate) fn sparse_document_text_control_table_rows(
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

pub(crate) fn document_text_control_table_rows(
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
