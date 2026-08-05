use rjtd_model::{TableCandidate, TableCandidateColumnSegment, TableCandidateInterval};

use super::primitives::{push_json_string, push_option_usize_json};

pub(crate) fn push_table_candidate_json(output: &mut String, candidate: &TableCandidate) {
    output.push_str("{\"index\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"kind\":");
    push_json_string(output, candidate.kind());
    output.push_str(",\"textBoundaryCandidateIndex\":");
    output.push_str(&candidate.text_boundary_candidate_index().to_string());
    output.push_str(",\"textCountRangeIndex\":");
    output.push_str(&candidate.text_count_range_index().to_string());
    output.push_str(",\"basis\":");
    push_json_string(output, candidate.basis().as_str());
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.delimiter_code()));
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
        push_observed_table_json(output, candidate);
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
    push_json_string(output, candidate.rule());
    output.push_str(",\"decoded\":false}");
}

fn push_sparse_observed_table_json(output: &mut String, candidate: &TableCandidate) {
    output.push_str("{\"source\":\"sparseDocumentTextControlRows\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&candidate.cell_count_candidate().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&candidate.non_empty_cell_count_candidate().to_string());
    output.push_str(",\"rows\":");
    push_sparse_table_rows_json(output, candidate.intervals());
    output.push_str(",\"topologyCandidate\":");
    if let Some(topology) = candidate.sparse_topology_candidate() {
        push_sparse_topology_candidate_json(output, candidate, &topology);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_sparse_topology_candidate_json(
    output: &mut String,
    candidate: &TableCandidate,
    topology: &rjtd_model::TableCandidateSparseTopologyCandidate,
) {
    output.push_str("{\"source\":\"sparseDocumentTextControlRows\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&topology.row_count().to_string());
    output.push_str(",\"maxColumnCountCandidate\":");
    output.push_str(&topology.max_column_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&topology.cell_count().to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&topology.empty_cell_count().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&topology.non_empty_cell_count().to_string());
    output.push_str(",\"rows\":[");
    for (index, row) in topology.rows().iter().enumerate() {
        if index > 0 {
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
        output.push_str(",\"cellCount\":");
        output.push_str(&row.cell_count().to_string());
        output.push_str(",\"emptyCellCount\":");
        output.push_str(&row.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCount\":");
        output.push_str(&row.non_empty_cell_count().to_string());
        output.push_str(",\"firstNonEmptyColumnIndex\":");
        push_option_usize_json(output, row.first_non_empty_column_index());
        output.push_str(",\"lastNonEmptyColumnIndex\":");
        push_option_usize_json(output, row.last_non_empty_column_index());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"columns\":[");
    for (index, column) in topology.columns().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&column.index().to_string());
        output.push_str(",\"observedCellCount\":");
        output.push_str(&column.observed_cell_count().to_string());
        output.push_str(",\"emptyCellCount\":");
        output.push_str(&column.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCount\":");
        output.push_str(&column.non_empty_cell_count().to_string());
        output.push_str(",\"firstNonEmptyRowIndex\":");
        push_option_usize_json(output, column.first_non_empty_row_index());
        output.push_str(",\"lastNonEmptyRowIndex\":");
        push_option_usize_json(output, column.last_non_empty_row_index());
        output.push_str(",\"sourceStart\":");
        push_option_usize_json(output, column.source_start());
        output.push_str(",\"sourceEnd\":");
        push_option_usize_json(output, column.source_end());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_sparse_table_rows_json(output: &mut String, rows: &[TableCandidateInterval]) {
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
        push_json_string(output, row.text_preview());
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
            push_json_string(output, cell.kind().as_str());
            output.push_str(",\"charStart\":");
            output.push_str(&cell.char_start().to_string());
            output.push_str(",\"charEnd\":");
            output.push_str(&cell.char_end().to_string());
            output.push_str(",\"sourceStart\":");
            push_option_usize_json(output, cell.source_start());
            output.push_str(",\"sourceEnd\":");
            push_option_usize_json(output, cell.source_end());
            output.push_str(",\"text\":");
            push_json_string(output, cell.text());
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

fn push_observed_table_json(output: &mut String, candidate: &TableCandidate) {
    let row_count = candidate.intervals().len();
    output.push_str("{\"rowCount\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"colCount\":1,\"cellCount\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"source\":\"tableCandidate\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"basis\":");
    push_json_string(output, candidate.basis().as_str());
    output.push_str(",\"delimiterCode\":");
    output.push_str(&candidate.delimiter_code().to_string());
    output.push_str(",\"delimiterCodeHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.delimiter_code()));
    output.push_str(",\"columnSplitCandidateRows\":");
    output.push_str(&candidate.column_split_candidate_row_count().to_string());
    output.push_str(",\"maxColumnSegmentCount\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"columnSegmentPatternConsistent\":");
    output.push_str(if candidate.column_segment_pattern_consistent() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"columnSegmentPatternMismatchRows\":");
    output.push_str(&candidate.column_segment_pattern_mismatch_rows().to_string());
    output.push_str(",\"columnGridCandidate\":");
    if let Some(grid) = candidate.column_segment_grid_candidate() {
        push_column_grid_candidate_json(output, candidate, &grid);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"columnSplittingDecoded\":false");
    output.push_str(",\"decoded\":false}");
}

fn push_column_grid_candidate_json(
    output: &mut String,
    candidate: &TableCandidate,
    grid: &rjtd_model::TableCandidateColumnGridCandidate,
) {
    output.push_str("{\"source\":\"columnSegments\",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&grid.row_count().to_string());
    output.push_str(",\"colCountCandidate\":");
    output.push_str(&grid.column_count().to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&grid.cell_count().to_string());
    output.push_str(",\"columnSplitCandidateRows\":");
    output.push_str(&grid.split_row_count().to_string());
    output.push_str(",\"maxColumnSegmentCount\":");
    output.push_str(&candidate.max_column_segment_count().to_string());
    output.push_str(",\"columnSegmentPatternConsistent\":true");
    output.push_str(",\"columnSegmentPatternMismatchRows\":0");
    output.push_str(",\"pattern\":[");
    for (index, kind) in grid.pattern().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, kind.as_str());
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_table_candidate_intervals_json(
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
        push_json_string(output, interval.text_preview());
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

fn push_table_candidate_column_segments_json(
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
        push_json_string(output, segment.kind().as_str());
        output.push_str(",\"charStart\":");
        output.push_str(&segment.char_start().to_string());
        output.push_str(",\"charEnd\":");
        output.push_str(&segment.char_end().to_string());
        output.push_str(",\"sourceStart\":");
        push_option_usize_json(output, segment.source_start());
        output.push_str(",\"sourceEnd\":");
        push_option_usize_json(output, segment.source_end());
        output.push_str(",\"text\":");
        push_json_string(output, segment.text());
        output.push_str(",\"charCount\":");
        output.push_str(&segment.text().chars().count().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}
