use super::*;

pub(super) const DIRECT_TABLE_CANDIDATE_SENTINEL: usize = usize::MAX;

pub(super) const SPARSE_TABLE_CANDIDATE_SENTINEL: usize = usize::MAX - 1;

pub(super) fn table_candidates_from_text_boundaries(
    document: &Document,
    entries: &[DocumentTextMapEntry],
) -> Vec<TableCandidate> {
    let Some(bounds) = document_text_source_bounds(document) else {
        return Vec::new();
    };

    let mut table_candidates = Vec::new();
    for candidate in document.text_boundary_candidates() {
        if candidate.interval_count() <= 1 {
            continue;
        }
        let intervals = table_candidate_intervals(document, entries, &bounds, candidate);
        if intervals.len() <= 1 {
            continue;
        }
        table_candidates.push(TableCandidate::from_text_boundary_candidate(
            table_candidates.len(),
            candidate,
            intervals,
        ));
    }
    table_candidates
}

pub(super) fn table_candidates_from_document_text_controls(
    entries: &[DocumentTextMapEntry],
    start_index: usize,
) -> Vec<TableCandidate> {
    let rows = document_text_control_table_rows(entries);
    let mut candidates = Vec::new();
    let mut current_rows = Vec::new();
    let mut current_column_count = 0usize;
    let mut empty_gap_count = 0usize;

    for row in rows {
        let column_count = row.cells.len();
        if column_count == 0 {
            if !current_rows.is_empty() {
                empty_gap_count += 1;
                if empty_gap_count > DOCUMENT_TEXT_CONTROL_TABLE_MAX_EMPTY_GAP_ROWS {
                    push_document_text_control_table_candidate(
                        &mut candidates,
                        start_index,
                        &mut current_rows,
                    );
                    current_column_count = 0;
                    empty_gap_count = 0;
                }
            }
            continue;
        }

        if column_count < 2 {
            push_document_text_control_table_candidate(
                &mut candidates,
                start_index,
                &mut current_rows,
            );
            current_column_count = 0;
            empty_gap_count = 0;
            continue;
        }

        if current_rows.is_empty()
            || (document_text_control_table_row_is_compatible(
                &current_rows,
                current_column_count,
                &row,
            ) && empty_gap_count <= DOCUMENT_TEXT_CONTROL_TABLE_MAX_EMPTY_GAP_ROWS)
        {
            current_column_count = current_column_count.max(column_count);
            current_rows.push(row);
            empty_gap_count = 0;
            continue;
        }

        push_document_text_control_table_candidate(&mut candidates, start_index, &mut current_rows);
        current_column_count = 0;
        empty_gap_count = 0;

        if column_count >= 2 {
            current_column_count = column_count;
            current_rows.push(row);
        } else if column_count == 0 {
            empty_gap_count += 1;
        }
    }

    push_document_text_control_table_candidate(&mut candidates, start_index, &mut current_rows);
    candidates
}

pub(super) fn push_document_text_control_table_candidate(
    candidates: &mut Vec<TableCandidate>,
    start_index: usize,
    rows: &mut Vec<DocumentTextControlTableRow>,
) {
    if document_text_control_table_rows_are_plausible(rows) {
        candidates.push(TableCandidate::from_document_text_control_rows(
            start_index + candidates.len(),
            rows,
        ));
    }
    rows.clear();
}

pub(super) fn sparse_table_candidates_from_document_text_controls(
    entries: &[DocumentTextMapEntry],
    start_index: usize,
) -> Vec<TableCandidate> {
    let rows = sparse_document_text_control_table_rows(entries);
    let mut candidates = Vec::new();
    let mut current_rows = Vec::new();

    for row in rows {
        if sparse_document_text_control_row_is_seed(&row)
            || (!current_rows.is_empty() && sparse_document_text_control_row_is_blank(&row))
            || (!current_rows.is_empty()
                && sparse_document_text_control_row_is_soft_separator(&row))
        {
            current_rows.push(row);
            continue;
        }

        push_sparse_document_text_control_table_candidate(
            &mut candidates,
            start_index,
            &mut current_rows,
        );
    }

    push_sparse_document_text_control_table_candidate(
        &mut candidates,
        start_index,
        &mut current_rows,
    );
    candidates
}

pub(super) fn push_sparse_document_text_control_table_candidate(
    candidates: &mut Vec<TableCandidate>,
    start_index: usize,
    rows: &mut Vec<DocumentTextControlTableRow>,
) {
    if sparse_document_text_control_table_rows_are_plausible(rows) {
        candidates.push(TableCandidate::from_sparse_document_text_control_rows(
            start_index + candidates.len(),
            rows,
        ));
    }
    rows.clear();
}

pub(super) fn table_candidate_intervals(
    document: &Document,
    entries: &[DocumentTextMapEntry],
    bounds: &TextSourceSpan,
    candidate: &TextBoundaryCandidate,
) -> Vec<TableCandidateInterval> {
    text_control_source_intervals(document, bounds, candidate.delimiter_code())
        .into_iter()
        .filter(|interval| {
            (candidate.first_interval_index()..=candidate.last_interval_index())
                .contains(&interval.index)
        })
        .filter_map(|interval| {
            let (interval_start, interval_end) =
                source_interval_range(&interval, candidate.basis());
            let source_start = interval_start.max(candidate.source_start());
            let source_end = interval_end.min(candidate.source_end());
            if source_start >= source_end {
                return None;
            }
            let text =
                range_visible_text_for_basis(entries, source_start, source_end, candidate.basis());
            Some(TableCandidateInterval::new(
                0,
                interval.index,
                source_start,
                source_end,
                text,
            ))
        })
        .enumerate()
        .map(|(index, interval)| TableCandidateInterval { index, ..interval })
        .collect()
}

pub(super) fn table_candidates_json(candidates: &[TableCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

pub(super) fn push_table_candidate_json(output: &mut String, candidate: &TableCandidate) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"textBoundaryCandidateIndex\":");
    output.push_str(&candidate.text_boundary_candidate_index().to_string());
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
    output.push_str(",\"intervals\":");
    push_table_candidate_intervals_json(
        output,
        candidate.intervals(),
        candidate.is_row_like() || candidate.is_sparse_document_text_control_run_candidate(),
    );
    output.push_str(",\"cellLike\":");
    output.push_str(if candidate.is_cell_like() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowLike\":");
    output.push_str(if candidate.is_row_like() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"observedTable\":");
    if candidate.is_row_like() {
        output.push_str(&observed_table_dimensions_json(candidate));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sparse\":");
    output.push_str(
        if candidate.is_sparse_document_text_control_run_candidate() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&candidate.cell_count_candidate().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&candidate.non_empty_cell_count_candidate().to_string());
    output.push_str(",\"sparseObservedTable\":");
    if candidate.is_sparse_document_text_control_run_candidate() {
        push_sparse_observed_table_json(output, candidate);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sparseTopologyCandidate\":");
    if let Some(topology) = candidate.sparse_topology_candidate() {
        push_sparse_topology_candidate_json(output, candidate, &topology);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rule\":");
    output.push_str(&json_string(candidate.rule()));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_table_candidate_intervals_json(
    output: &mut String,
    intervals: &[TableCandidateInterval],
    emit_column_segments: bool,
) {
    output.push('[');
    for (index, interval) in intervals.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&interval.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&interval.source_interval_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&interval.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&interval.source_end().to_string());
        output.push_str(",\"textPreview\":");
        output.push_str(&json_string(interval.text_preview()));
        output.push_str(",\"textCharCount\":");
        output.push_str(&interval.text_char_count().to_string());
        output.push_str(",\"lineBreakCount\":");
        output.push_str(&interval.line_break_count().to_string());
        output.push_str(",\"columnSegments\":");
        if emit_column_segments {
            push_table_candidate_column_segments_json(output, interval.column_segments());
        } else {
            output.push_str("[]");
        }
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

pub(super) fn push_table_candidate_column_segments_json(
    output: &mut String,
    segments: &[TableCandidateColumnSegment],
) {
    output.push('[');
    for (index, segment) in segments.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&segment.index().to_string());
        output.push_str(",\"kind\":");
        output.push_str(&json_string(segment.kind().as_str()));
        output.push_str(",\"charStart\":");
        output.push_str(&segment.char_start().to_string());
        output.push_str(",\"charEnd\":");
        output.push_str(&segment.char_end().to_string());
        output.push_str(",\"sourceStart\":");
        push_option_usize_json(output, segment.source_start());
        output.push_str(",\"sourceEnd\":");
        push_option_usize_json(output, segment.source_end());
        output.push_str(",\"text\":");
        output.push_str(&json_string(segment.text()));
        output.push_str(",\"charCount\":");
        output.push_str(&segment.text().chars().count().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

pub(super) fn table_candidate_interval_non_empty_cell_count(
    interval: &TableCandidateInterval,
) -> usize {
    interval
        .column_segments()
        .iter()
        .filter(|segment| !segment.text().is_empty())
        .count()
}

pub(super) fn push_answer_sheet_sparse_table_candidate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    output.push_str("{\"source\":\"tableCandidates\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&json_string(candidate.kind()));
    output.push_str(",\"rule\":");
    output.push_str(&json_string(candidate.rule()));
    output.push_str(",\"sourceStart\":");
    output.push_str(&candidate.source_start().to_string());
    output.push_str(",\"sourceEnd\":");
    output.push_str(&candidate.source_end().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.interval_count().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&candidate.cell_count_candidate().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&candidate.non_empty_cell_count_candidate().to_string());
    output.push_str(",\"previewRows\":[");
    for (index, interval) in candidate.intervals().iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&interval.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&interval.source_interval_index().to_string());
        output.push_str(",\"textPreview\":");
        output.push_str(&json_string(interval.text_preview()));
        output.push('}');
    }
    output.push_str("],\"rows\":");
    push_sparse_table_rows_json(output, candidate.intervals());
    output.push_str(",\"topologyCandidate\":");
    if let Some(topology) = candidate.sparse_topology_candidate() {
        push_sparse_topology_candidate_json(output, candidate, &topology);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"ruleTopologyEvidence\":");
    push_answer_sheet_rule_topology_evidence_json(output, candidate);
    output.push_str(",\"layoutStreamProbe\":");
    push_table_grid_layout_stream_probe_json(output, document, candidate);
    output.push_str(",\"sectionLineMarkGeometryCandidate\":");
    push_answer_sheet_section_line_mark_geometry_candidate_json(
        output, layout, document, candidate,
    );
    output.push_str(",\"geometryDecoded\":false,\"decoded\":false}");
}

pub(super) fn push_sparse_table_rows_json(output: &mut String, rows: &[TableCandidateInterval]) {
    output.push('[');
    for (row_array_index, row) in rows.iter().enumerate() {
        if row_array_index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&row.index().to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&row.source_interval_index().to_string());
        output.push_str(",\"sourceStart\":");
        output.push_str(&row.source_start().to_string());
        output.push_str(",\"sourceEnd\":");
        output.push_str(&row.source_end().to_string());
        output.push_str(",\"textPreview\":");
        output.push_str(&json_string(row.text_preview()));
        output.push_str(",\"cellCount\":");
        output.push_str(&row.column_segments().len().to_string());
        output.push_str(",\"cells\":[");
        for (cell_array_index, cell) in row.column_segments().iter().enumerate() {
            if cell_array_index > 0 {
                output.push(',');
            }
            output.push_str("{\"index\":");
            output.push_str(&cell.index().to_string());
            output.push_str(",\"kind\":");
            output.push_str(&json_string(cell.kind().as_str()));
            output.push_str(",\"charStart\":");
            output.push_str(&cell.char_start().to_string());
            output.push_str(",\"charEnd\":");
            output.push_str(&cell.char_end().to_string());
            output.push_str(",\"sourceStart\":");
            push_option_usize_json(output, cell.source_start());
            output.push_str(",\"sourceEnd\":");
            push_option_usize_json(output, cell.source_end());
            output.push_str(",\"text\":");
            output.push_str(&json_string(cell.text()));
            output.push_str(",\"empty\":");
            output.push_str(if cell.text().is_empty() {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"decoded\":false}");
        }
        output.push_str("],\"decoded\":false}");
    }
    output.push(']');
}

pub(super) fn table_candidate_source_anchor_count(candidate: &TableCandidate) -> usize {
    candidate
        .intervals()
        .iter()
        .flat_map(TableCandidateInterval::column_segments)
        .filter(|segment| {
            matches!(
                (segment.source_start(), segment.source_end()),
                (Some(start), Some(end)) if start < end
            )
        })
        .count()
}

pub(super) fn table_candidate_document_text_line_header_rows(
    document: &Document,
    candidate: &TableCandidate,
) -> Vec<TableCandidateLineHeaderRow> {
    if !candidate.is_document_text_control_run_candidate() {
        return Vec::new();
    }
    let Some(bytes) = document_text_raw_stream(document) else {
        return Vec::new();
    };

    candidate
        .intervals()
        .iter()
        .enumerate()
        .map(|(row_index, interval)| {
            let expected_cell_count = interval.column_segments().len();
            let headers =
                table_candidate_line_headers_for_interval(bytes, candidate.basis(), interval);
            let matched_cell_count =
                table_candidate_line_header_matched_cell_count(&headers, expected_cell_count);
            TableCandidateLineHeaderRow {
                row_index,
                source_start: interval.source_start(),
                source_end: interval.source_end(),
                expected_cell_count,
                matched_cell_count,
                headers,
            }
        })
        .collect()
}

pub(super) fn table_candidate_line_headers_for_interval(
    bytes: &[u8],
    basis: TextCountRangeOverlapBasis,
    interval: &TableCandidateInterval,
) -> Vec<ShanaiLanLineHeader> {
    let Some((mut offset, end)) = table_candidate_interval_byte_range(
        bytes,
        basis,
        interval.source_start(),
        interval.source_end(),
    ) else {
        return Vec::new();
    };
    if offset % 2 != 0 {
        offset += 1;
    }
    let mut headers = Vec::new();
    while offset + 24 <= end {
        if let Some(header) = shanai_lan_line_header_at(bytes, offset) {
            headers.push(header);
            offset = header.end;
        } else {
            offset += 2;
        }
    }
    headers
}

pub(super) fn table_candidate_interval_byte_range(
    bytes: &[u8],
    basis: TextCountRangeOverlapBasis,
    source_start: usize,
    source_end: usize,
) -> Option<(usize, usize)> {
    if source_start >= source_end {
        return None;
    }
    let (byte_start, byte_end) = match basis {
        TextCountRangeOverlapBasis::Byte => (source_start, source_end),
        TextCountRangeOverlapBasis::Unit => {
            (source_start.checked_mul(2)?, source_end.checked_mul(2)?)
        }
    };
    if byte_start >= bytes.len() || byte_start >= byte_end {
        return None;
    }
    Some((byte_start, byte_end.min(bytes.len())))
}

pub(super) fn table_candidate_line_header_matched_cell_count(
    headers: &[ShanaiLanLineHeader],
    expected_cell_count: usize,
) -> usize {
    if expected_cell_count == 0 || headers.len() < expected_cell_count {
        return 0;
    }
    let mut previous_offset = None;
    for header in headers {
        if header.extent_units <= header.offset_units || header.font_size_units == 0 {
            return 0;
        }
        if previous_offset.is_some_and(|offset| header.offset_units <= offset) {
            return 0;
        }
        previous_offset = Some(header.offset_units);
    }
    expected_cell_count
}

pub(super) fn table_candidate_anchor_line_index(
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Option<usize> {
    lines.iter().enumerate().find_map(|(line_index, line)| {
        page_text_line_fragments(document, line)
            .into_iter()
            .filter_map(|fragment| fragment.source_span)
            .any(|span| table_candidate_overlaps_source_span(candidate, &span))
            .then_some(line_index)
    })
}

pub(super) fn table_candidate_overlaps_source_span(
    candidate: &TableCandidate,
    span: &TextSourceSpan,
) -> bool {
    let (span_start, span_end) = match candidate.basis() {
        TextCountRangeOverlapBasis::Byte => (span.byte_start(), span.byte_end()),
        TextCountRangeOverlapBasis::Unit => (span.unit_start(), span.unit_end()),
    };
    candidate.source_start() < span_end && span_start < candidate.source_end()
}
