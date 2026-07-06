use std::fs;
use std::path::Path;

use crate::probe_format::{
    file_name, format_optional_i32, format_optional_isize, format_optional_u16,
    format_optional_usize, format_optional_usize_list, format_row_cell_offsets,
    format_uniform_delta,
};
use crate::probe_line_diff::{LineDetail, line_detail, line_diff_lines};
use crate::probe_page_diff::{page_diff_lines, page_tuples};
use crate::probe_signals::{JtdSignal, analyze_jtd};

pub fn source_y_probe_compare_lines(
    base_path: &Path,
    candidate_path: &Path,
) -> Result<Vec<String>, String> {
    let base_bytes =
        fs::read(base_path).map_err(|error| format!("reading {}: {error}", base_path.display()))?;
    let candidate_bytes = fs::read(candidate_path)
        .map_err(|error| format!("reading {}: {error}", candidate_path.display()))?;

    let base_signal = analyze_jtd(&base_bytes);
    let candidate_signal = analyze_jtd(&candidate_bytes);
    let base_line = line_detail(&base_bytes);
    let candidate_line = line_detail(&candidate_bytes);
    let base_pages = page_tuples(&base_bytes);
    let candidate_pages = page_tuples(&candidate_bytes);

    let mut lines = Vec::new();
    lines.push(format!(
        "summary\tbase={}\tcandidate={}\tsourceSignatureSame={}",
        file_name(base_path),
        file_name(candidate_path),
        base_signal.source_signature_hash == candidate_signal.source_signature_hash
    ));
    lines.extend(line_diff_lines(
        &base_line,
        &candidate_line,
        &base_signal,
        &candidate_signal,
    ));
    lines.extend(page_diff_lines(
        &base_pages,
        &candidate_pages,
        &base_signal,
        &candidate_signal,
    ));
    lines.push(table_summary_line(&base_signal, &candidate_signal));
    if let Some(line) = table_line_header_summary_line(&base_signal, &candidate_signal) {
        lines.push(line);
    }
    if let Some(line) = table_flow_y_summary_line(
        (&base_signal, &base_line),
        (&candidate_signal, &candidate_line),
    ) {
        lines.push(line);
        if let Some(line) = table_flow_y_admission_summary_line(
            (&base_signal, &base_line),
            (&candidate_signal, &candidate_line),
        ) {
            lines.push(line);
        }
        if let Some(line) = table_flow_y_hypothesis_line(
            (&base_signal, &base_line),
            (&candidate_signal, &candidate_line),
        ) {
            lines.push(line);
        }
    }
    lines.push("admission\tready=false\treason=direct-source-diff-diagnostic-only".to_string());
    Ok(lines)
}

fn table_summary_line(base: &JtdSignal, candidate: &JtdSignal) -> String {
    format!(
        "table-summary\tbaseCandidates={}\tcandidateCandidates={}\tbaseSparseCandidates={}\tcandidateSparseCandidates={}\tbaseNonEmptyCells={}\tcandidateNonEmptyCells={}\ttableSignatureSame={}",
        base.table_candidate_count,
        candidate.table_candidate_count,
        base.sparse_table_candidate_count,
        candidate.sparse_table_candidate_count,
        base.table_non_empty_cell_count,
        candidate.table_non_empty_cell_count,
        base.table_signature == candidate.table_signature
    )
}

fn table_line_header_summary_line(base: &JtdSignal, candidate: &JtdSignal) -> Option<String> {
    if base.table_line_header_rows.is_empty() && candidate.table_line_header_rows.is_empty() {
        return None;
    }

    let base_first = first_cell_offset(base);
    let candidate_first = first_cell_offset(candidate);
    let delta = base_first
        .zip(candidate_first)
        .map(|(base, candidate)| i32::from(candidate).saturating_sub(i32::from(base)));

    Some(format!(
        "table-line-header-summary\tbaseRows={}\tcandidateRows={}\tbaseFirstCellOffset={}\tcandidateFirstCellOffset={}\tfirstCellOffsetDelta={}\tbaseCellOffsets={}\tcandidateCellOffsets={}\tlineHeaderSignatureSame={}",
        base.table_line_header_rows.len(),
        candidate.table_line_header_rows.len(),
        format_optional_u16(base_first),
        format_optional_u16(candidate_first),
        format_optional_i32(delta),
        format_row_cell_offsets(&base.table_line_header_rows),
        format_row_cell_offsets(&candidate.table_line_header_rows),
        base.table_line_header_signature == candidate.table_line_header_signature
    ))
}

fn first_cell_offset(signal: &JtdSignal) -> Option<u16> {
    signal
        .table_line_header_rows
        .iter()
        .find_map(|row| row.cell_offsets.first().copied())
}

fn table_flow_y_summary_line(
    base: (&JtdSignal, &LineDetail),
    candidate: (&JtdSignal, &LineDetail),
) -> Option<String> {
    if base.0.table_line_header_rows.is_empty() && candidate.0.table_line_header_rows.is_empty() {
        return None;
    }

    let base_records = row_line_mark_records(base.0, base.1);
    let candidate_records = row_line_mark_records(candidate.0, candidate.1);
    let record_delta = uniform_record_delta(&base_records, &candidate_records);
    let first_source_start_delta = optional_usize_delta(
        first_row_source_start(base.0),
        first_row_source_start(candidate.0),
    );

    Some(format!(
        "table-flow-y-summary\tbaseRows={}\tcandidateRows={}\tbaseLineMarkRecords={}\tcandidateLineMarkRecords={}\tlineMarkRecordDelta={}\tuniformLineMarkRecordDelta={}\tbaseFirstRowSourceStart={}\tcandidateFirstRowSourceStart={}\tfirstRowSourceStartDelta={}",
        base.0.table_line_header_rows.len(),
        candidate.0.table_line_header_rows.len(),
        format_optional_usize_list(&base_records),
        format_optional_usize_list(&candidate_records),
        format_optional_isize(record_delta),
        format_uniform_delta(record_delta, &base_records, &candidate_records),
        format_optional_usize(first_row_source_start(base.0)),
        format_optional_usize(first_row_source_start(candidate.0)),
        format_optional_isize(first_source_start_delta)
    ))
}

fn row_line_mark_records(signal: &JtdSignal, line: &LineDetail) -> Vec<Option<usize>> {
    signal
        .table_line_header_rows
        .iter()
        .map(|row| line.record_index_containing(row.source_start, row.source_end))
        .collect()
}

fn table_flow_y_admission_summary_line(
    base: (&JtdSignal, &LineDetail),
    candidate: (&JtdSignal, &LineDetail),
) -> Option<String> {
    if base.0.table_line_header_rows.is_empty() && candidate.0.table_line_header_rows.is_empty() {
        return None;
    }

    let base_rows_exact = rows_exact_and_contiguous(base.0, base.1);
    let candidate_rows_exact = rows_exact_and_contiguous(candidate.0, candidate.1);
    let blocker = if base_rows_exact && candidate_rows_exact {
        "-"
    } else {
        "line-mark-rows-not-exact-source-boundaries"
    };

    Some(format!(
        "table-flow-y-admission-summary\tbaseLineMarkRecordStride={}\tcandidateLineMarkRecordStride={}\tbaseExactSourceRangeMatchCount={}\tcandidateExactSourceRangeMatchCount={}\tbaseRowsExactAndContiguous={}\tcandidateRowsExactAndContiguous={}\tblocker={}",
        format_optional_isize(
            base.1
                .record_stride_containing(&base.0.table_line_header_rows)
        ),
        format_optional_isize(
            candidate
                .1
                .record_stride_containing(&candidate.0.table_line_header_rows)
        ),
        base.1
            .exact_source_range_match_count(&base.0.table_line_header_rows),
        candidate
            .1
            .exact_source_range_match_count(&candidate.0.table_line_header_rows),
        base_rows_exact,
        candidate_rows_exact,
        blocker
    ))
}

fn rows_exact_and_contiguous(signal: &JtdSignal, line: &LineDetail) -> bool {
    line.rows_exact_and_contiguous(&signal.table_line_header_rows)
}

fn table_flow_y_hypothesis_line(
    base: (&JtdSignal, &LineDetail),
    candidate: (&JtdSignal, &LineDetail),
) -> Option<String> {
    if base.0.table_line_header_rows.is_empty() && candidate.0.table_line_header_rows.is_empty() {
        return None;
    }

    let base_records = row_line_mark_records(base.0, base.1);
    let candidate_records = row_line_mark_records(candidate.0, candidate.1);
    let record_delta = uniform_record_delta(&base_records, &candidate_records);
    let first_source_start_delta = optional_usize_delta(
        first_row_source_start(base.0),
        first_row_source_start(candidate.0),
    );
    let stride_correlation_observed =
        record_delta.is_some_and(|delta| delta != 0 && Some(delta) == first_source_start_delta);
    if !stride_correlation_observed {
        return None;
    }

    let rows_exact = rows_exact_and_contiguous(base.0, base.1)
        && rows_exact_and_contiguous(candidate.0, candidate.1);
    let blocked_reason = if rows_exact {
        "source-page-y-transform-not-proven"
    } else {
        "line-mark-rows-not-exact-source-boundaries"
    };

    Some(format!(
        "table-flow-y-hypothesis\tstrideCorrelationObserved=true\ttransformProven=false\trenderAdmissible=false\thypothesis=line-mark-record-stride-correlates-with-flow-y\tblockedReason={blocked_reason}"
    ))
}

fn uniform_record_delta(
    base_records: &[Option<usize>],
    candidate_records: &[Option<usize>],
) -> Option<isize> {
    if base_records.len() != candidate_records.len() || base_records.is_empty() {
        return None;
    }
    let mut deltas = base_records
        .iter()
        .zip(candidate_records)
        .map(|(base, candidate)| optional_usize_delta(*base, *candidate));
    let first_delta = deltas.next()??;
    if deltas.all(|delta| delta == Some(first_delta)) {
        Some(first_delta)
    } else {
        None
    }
}

fn first_row_source_start(signal: &JtdSignal) -> Option<usize> {
    signal
        .table_line_header_rows
        .first()
        .map(|row| row.source_start)
}

fn optional_usize_delta(base: Option<usize>, candidate: Option<usize>) -> Option<isize> {
    let base = isize::try_from(base?).ok()?;
    let candidate = isize::try_from(candidate?).ok()?;
    Some(candidate.saturating_sub(base))
}
