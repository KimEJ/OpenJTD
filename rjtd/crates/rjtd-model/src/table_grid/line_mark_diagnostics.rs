use super::*;
use crate::*;

pub(crate) fn push_table_grid_line_mark_stride_y_reference_comparison_json(
    output: &mut String,
    source_layout: &TableGridSourceDerivedLayout,
    reference_layout: &TableGridReferenceLayout,
) {
    let Some(stride) = source_layout.line_mark_page_origin_stride.as_ref() else {
        output.push_str("null");
        return;
    };
    let row_count = source_layout
        .row_count
        .min(stride.raw_record_index_row_tops.len())
        .min(stride.stride_collapsed_row_tops.len());
    push_table_grid_line_mark_stride_y_reference_comparison_fields_json(
        output,
        "lineMarkPageOriginStrideCandidate+referenceTableBBox",
        row_count,
        stride,
        reference_layout,
    );
}

pub(crate) fn push_table_grid_line_mark_stride_y_reference_comparison_fields_json(
    output: &mut String,
    source: &str,
    row_count: usize,
    stride: &TableGridLineMarkPageOriginStrideCandidate,
    reference_layout: &TableGridReferenceLayout,
) {
    if row_count == 0 {
        output.push_str("null");
        return;
    }

    let reference_row_tops = (0..row_count)
        .map(|row| reference_layout.y + row as f32 * reference_layout.row_height)
        .collect::<Vec<_>>();
    let raw_residuals = stride
        .raw_record_index_row_tops
        .iter()
        .take(row_count)
        .zip(&reference_row_tops)
        .map(|(candidate, reference)| candidate - reference)
        .collect::<Vec<_>>();
    let collapsed_residuals = stride
        .stride_collapsed_row_tops
        .iter()
        .take(row_count)
        .zip(&reference_row_tops)
        .map(|(candidate, reference)| candidate - reference)
        .collect::<Vec<_>>();
    let raw_max_abs = max_abs_f32(&raw_residuals);
    let collapsed_max_abs = max_abs_f32(&collapsed_residuals);
    let raw_mean_abs = mean_abs_f32(&raw_residuals);
    let collapsed_mean_abs = mean_abs_f32(&collapsed_residuals);
    let best_hypothesis = match (raw_max_abs, collapsed_max_abs) {
        (Some(raw), Some(collapsed)) if raw <= collapsed => "raw-record-index",
        (Some(_), Some(_)) => "stride-collapsed-record-index",
        (Some(_), None) => "raw-record-index",
        (None, Some(_)) => "stride-collapsed-record-index",
        (None, None) => "none",
    };

    output.push_str("{\"source\":");
    output.push_str(&json_string(source));
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &stride.line_mark_record_indexes);
    output.push_str(",\"recordStride\":");
    output.push_str(&stride.record_stride.to_string());
    output.push_str(",\"rowCountCompared\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"referenceRowTops\":");
    push_f32_array_json(output, &reference_row_tops);
    output.push_str(",\"rawRecordIndexRowTops\":");
    push_f32_array_json(output, &stride.raw_record_index_row_tops[..row_count]);
    output.push_str(",\"rawRecordIndexResidualsPx\":");
    push_f32_array_json(output, &raw_residuals);
    output.push_str(",\"rawRecordIndexMeanAbsResidualPx\":");
    push_optional_f32_json(output, raw_mean_abs);
    output.push_str(",\"rawRecordIndexMaxAbsResidualPx\":");
    push_optional_f32_json(output, raw_max_abs);
    output.push_str(",\"strideCollapsedRowTops\":");
    push_f32_array_json(output, &stride.stride_collapsed_row_tops[..row_count]);
    output.push_str(",\"strideCollapsedResidualsPx\":");
    push_f32_array_json(output, &collapsed_residuals);
    output.push_str(",\"strideCollapsedMeanAbsResidualPx\":");
    push_optional_f32_json(output, collapsed_mean_abs);
    output.push_str(",\"strideCollapsedMaxAbsResidualPx\":");
    push_optional_f32_json(output, collapsed_max_abs);
    output.push_str(",\"recordIndexAffineFit\":");
    push_table_grid_line_mark_record_index_affine_fit_json(
        output,
        row_count,
        stride,
        &reference_row_tops,
    );
    output.push_str(",\"bestYHypothesisCandidate\":");
    output.push_str(&json_string(best_hypothesis));
    output.push_str(",\"bestHypothesisMaxAbsResidualPx\":");
    let best_max_abs = match best_hypothesis {
        "raw-record-index" => raw_max_abs,
        "stride-collapsed-record-index" => collapsed_max_abs,
        _ => None,
    };
    push_optional_f32_json(output, best_max_abs);
    output.push_str(",\"renderPromotionContribution\":\"stride-y-residual-diagnostic-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "stride-y-hypothesis-needs-cross-table-validation",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_line_mark_record_index_affine_fit_json(
    output: &mut String,
    row_count: usize,
    stride: &TableGridLineMarkPageOriginStrideCandidate,
    reference_row_tops: &[f32],
) {
    let line_mark_record_indexes = stride
        .line_mark_record_indexes
        .iter()
        .take(row_count)
        .map(|record_index| *record_index as f32)
        .collect::<Vec<_>>();
    if line_mark_record_indexes.len() != row_count || reference_row_tops.len() != row_count {
        output.push_str("null");
        return;
    }
    let Some((reference_slope, reference_intercept)) =
        affine_fit_f32(&line_mark_record_indexes, reference_row_tops)
    else {
        output.push_str("null");
        return;
    };
    let fitted_row_tops = line_mark_record_indexes
        .iter()
        .map(|record_index| reference_intercept + reference_slope * record_index)
        .collect::<Vec<_>>();
    let fit_residuals = fitted_row_tops
        .iter()
        .zip(reference_row_tops)
        .map(|(fit, reference)| fit - reference)
        .collect::<Vec<_>>();
    let reference_row_deltas = reference_row_tops
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();
    let source_raw_slope = slope_from_indexed_tops(
        &line_mark_record_indexes,
        &stride.raw_record_index_row_tops[..row_count],
    );
    let source_collapsed_slope = slope_from_indexed_tops(
        &line_mark_record_indexes,
        &stride.stride_collapsed_row_tops[..row_count],
    );
    let reference_row_height = mean_f32(&reference_row_deltas);
    let reference_px_per_record_stride =
        reference_row_height.map(|row_height| row_height / stride.record_stride as f32);

    output.push_str("{\"source\":\"/LineMark record indexes+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &stride.line_mark_record_indexes[..row_count]);
    output.push_str(",\"recordStride\":");
    output.push_str(&stride.record_stride.to_string());
    output.push_str(",\"referenceSlopePxPerRecord\":");
    output.push_str(&format!("{reference_slope:.3}"));
    output.push_str(",\"referenceInterceptPx\":");
    output.push_str(&format!("{reference_intercept:.3}"));
    output.push_str(",\"referenceFittedRowTops\":");
    push_f32_array_json(output, &fitted_row_tops);
    output.push_str(",\"referenceFitResidualsPx\":");
    push_f32_array_json(output, &fit_residuals);
    output.push_str(",\"referenceFitMeanAbsResidualPx\":");
    push_optional_f32_json(output, mean_abs_f32(&fit_residuals));
    output.push_str(",\"referenceFitMaxAbsResidualPx\":");
    push_optional_f32_json(output, max_abs_f32(&fit_residuals));
    output.push_str(",\"referenceRowHeightPx\":");
    push_optional_f32_json(output, reference_row_height);
    output.push_str(",\"referencePxPerRecordStride\":");
    push_optional_f32_json(output, reference_px_per_record_stride);
    output.push_str(",\"sourceRawSlopePxPerRecord\":");
    push_optional_f32_json(output, source_raw_slope);
    output.push_str(",\"sourceRawSlopeResidualPxPerRecord\":");
    push_optional_f32_json(
        output,
        source_raw_slope.map(|slope| slope - reference_slope),
    );
    output.push_str(",\"sourceStrideCollapsedSlopePxPerRecord\":");
    push_optional_f32_json(output, source_collapsed_slope);
    output.push_str(",\"sourceStrideCollapsedSlopeResidualPxPerRecord\":");
    push_optional_f32_json(
        output,
        source_collapsed_slope.map(|slope| slope - reference_slope),
    );
    output.push_str(",\"renderPromotionContribution\":\"record-index-affine-fit-diagnostic-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "record-index-affine-fit-is-reference-backed-not-source-transform",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_sparse_sibling_line_mark_y_comparison_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(sibling) = table_grid_sparse_table_sibling_evidence(document, candidate) else {
        output.push_str("null");
        return;
    };
    if sibling.rows.len() != candidate.intervals().len()
        || sibling
            .rows
            .iter()
            .map(|row| row.segments.len())
            .sum::<usize>()
            != candidate.cell_count_candidate()
    {
        output.push_str("null");
        return;
    }
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let Some(reference_layout) =
        diagnostic_reference_table_grid_overlay_layout(layout, document, candidate, column_count)
    else {
        output.push_str("null");
        return;
    };
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    if rows.len() != candidate.intervals().len() {
        output.push_str("null");
        return;
    }
    let Some((font_size_units, rows_with_headers, raw_header_count)) =
        table_grid_line_header_font_size_units_candidate(&rows)
    else {
        output.push_str("null");
        return;
    };
    let row_height = f32::from(font_size_units) * 1.75;
    let Some(stride) = table_grid_line_mark_page_origin_stride_candidate(
        layout, document, candidate, &rows, row_height,
    ) else {
        output.push_str("null");
        return;
    };
    let row_count = rows
        .len()
        .min(stride.raw_record_index_row_tops.len())
        .min(stride.stride_collapsed_row_tops.len());
    if row_count == 0 {
        output.push_str("null");
        return;
    }
    let shared_source_interval_indexes = sibling
        .rows
        .iter()
        .map(|row| row.source_interval_index)
        .collect::<Vec<_>>();
    let matched_sparse_column_indexes = table_grid_sparse_sibling_matched_sparse_column_indexes(
        &sibling.rows,
        candidate.max_column_segment_count(),
    );

    output.push_str(
        "{\"source\":\"sparseTableSiblingEvidence+/LineMark+/PageMark+referenceTableBBox\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sparseTableCandidateIndex\":");
    output.push_str(&sibling.sparse_candidate.index().to_string());
    output.push_str(",\"sharedSourceIntervalIndexes\":");
    push_usize_array_json(output, &shared_source_interval_indexes);
    output.push_str(",\"compactToSparseColumnOffsetCandidate\":");
    push_option_usize_json(output, sibling.compact_to_sparse_column_offset);
    output.push_str(",\"matchedSparseColumnIndexes\":");
    push_usize_array_json(output, &matched_sparse_column_indexes);
    output.push_str(",\"sourceRowHeightBasis\":");
    output.push_str(&json_string(if rows_with_headers == rows.len() {
        "documentTextLineHeaderFontSizeUnits"
    } else {
        "partialDocumentTextLineHeaderFontSizeUnits"
    }));
    output.push_str(",\"homogeneousFontSizeUnits\":");
    output.push_str(&font_size_units.to_string());
    output.push_str(",\"lineHeaderRowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"lineHeaderRowsWithHeaders\":");
    output.push_str(&rows_with_headers.to_string());
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&raw_header_count.to_string());
    output.push_str(",\"sourceRowHeightPx\":");
    output.push_str(&format!("{row_height:.3}"));
    output.push_str(",\"postRowGapLineMarkCorrelation\":");
    push_table_grid_sparse_sibling_post_row_gap_line_mark_correlation_json(
        output, document, candidate, &sibling,
    );
    output.push_str(",\"lineMarkRowGapSequenceEvidence\":");
    push_table_grid_line_mark_row_gap_sequence_evidence_json(output, document, candidate, &sibling);
    output.push_str(",\"lineMarkRowGapSequenceYComparison\":");
    push_table_grid_line_mark_row_gap_sequence_y_comparison_json(
        output,
        layout,
        document,
        candidate,
        &sibling,
        row_height,
        &reference_layout,
    );
    output.push_str(",\"comparison\":");
    push_table_grid_line_mark_stride_y_reference_comparison_fields_json(
        output,
        "sparseSiblingLineMarkPageOriginStrideCandidate+referenceTableBBox",
        row_count,
        &stride,
        &reference_layout,
    );
    output.push_str(",\"lineMarkStridePromotionReadiness\":");
    push_table_grid_line_mark_stride_promotion_readiness_json(
        output,
        document,
        candidate,
        &sibling,
        &rows,
        rows_with_headers,
        raw_header_count,
        &stride,
        &reference_layout,
    );
    output.push_str(
        ",\"renderPromotionContribution\":\"sparse-sibling-stride-y-residual-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "sparse-sibling-y-hypothesis-needs-page-space-transform",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_sparse_sibling_post_row_gap_line_mark_correlation_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
) {
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() || sibling.rows.is_empty() {
        output.push_str("null");
        return;
    }

    let mut row_evidence = Vec::new();
    for (row_index, row) in sibling.rows.iter().enumerate() {
        let Some(interval) = candidate
            .intervals()
            .iter()
            .find(|interval| interval.index() == row.compact_row_index)
        else {
            continue;
        };
        let Some(line_mark) =
            table_grid_interval_line_mark(candidate, interval, &line_mark_intervals)
        else {
            continue;
        };
        let Some(gap) = table_grid_sparse_sibling_post_row_gap(sibling, row_index) else {
            continue;
        };
        let line_mark_span_units = line_mark.unit_end.saturating_sub(line_mark.unit_start);
        let post_row_gap_units = gap.source_end.saturating_sub(gap.source_start);
        let span_minus_gap_units = line_mark_span_units as isize - post_row_gap_units as isize;
        row_evidence.push((
            row,
            line_mark,
            gap,
            line_mark_span_units,
            post_row_gap_units,
            span_minus_gap_units,
        ));
    }

    if row_evidence.is_empty() {
        output.push_str("null");
        return;
    }

    let exact_span_match_count = row_evidence
        .iter()
        .filter(|(_, _, _, _, _, residual)| *residual == 0)
        .count();
    output.push_str("{\"source\":\"sparseTableSiblingEvidence+/LineMark\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"rowCount\":");
    output.push_str(&sibling.rows.len().to_string());
    output.push_str(",\"matchedGapCount\":");
    output.push_str(&row_evidence.len().to_string());
    output.push_str(",\"exactSpanMatchCount\":");
    output.push_str(&exact_span_match_count.to_string());
    output.push_str(",\"allRowsExactSpanMatched\":");
    output.push_str(if exact_span_match_count == sibling.rows.len() {
        "true"
    } else {
        "false"
    });
    output.push_str(
        ",\"renderPromotionContribution\":\"line-mark-span-post-row-gap-correlation-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-span-correlates-with-post-row-gap-not-page-height",
    ));
    output.push_str(",\"rows\":[");
    for (index, (row, line_mark, gap, line_mark_span_units, post_row_gap_units, residual)) in
        row_evidence.iter().enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"compactRow\":");
        output.push_str(&row.compact_row_index.to_string());
        output.push_str(",\"sparseRow\":");
        output.push_str(&row.sparse_row_index.to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&row.source_interval_index.to_string());
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&line_mark.record_index.to_string());
        output.push_str(",\"lineMarkUnitRange\":");
        output.push_str(&source_range_json(line_mark.unit_start, line_mark.unit_end));
        output.push_str(",\"lineMarkSpanUnits\":");
        output.push_str(&line_mark_span_units.to_string());
        output.push_str(",\"postRowGapSourceRange\":");
        output.push_str(&source_range_json(gap.source_start, gap.source_end));
        output.push_str(",\"postRowGapUnits\":");
        output.push_str(&post_row_gap_units.to_string());
        output.push_str(",\"postRowGapKind\":");
        output.push_str(&json_string(gap.kind));
        output.push_str(",\"gapSparseRowIndexes\":");
        push_usize_array_json(output, &gap.sparse_row_indexes);
        output.push_str(",\"gapSparseSourceIntervalIndexes\":");
        push_usize_array_json(output, &gap.sparse_source_interval_indexes);
        output.push_str(",\"lineMarkSpanMinusGapUnits\":");
        output.push_str(&residual.to_string());
        output.push_str(",\"exactSpanMatch\":");
        output.push_str(if *residual == 0 { "true" } else { "false" });
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_page_mark_raw_record_scan_evidence_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        output.push_str("null");
        return;
    };
    let record_headers = page_mark_record_headers(page_mark_bytes);
    if record_headers.is_empty() {
        output.push_str("null");
        return;
    }

    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        output.push_str("null");
        return;
    }

    let mut row_matches = Vec::new();
    for (row_index, row) in candidate.intervals().iter().enumerate() {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start());
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end());
        let Some(line_mark) = best_line_mark_interval_for_unit_range(
            &line_mark_intervals,
            row_unit_start,
            row_unit_end,
        ) else {
            row_matches.push((row_index, None, None));
            continue;
        };
        let record_header_index = record_headers.iter().position(|header| {
            header.line_start as usize <= line_mark.record_index
                && line_mark.record_index <= header.line_end as usize
        });
        row_matches.push((row_index, Some(line_mark), record_header_index));
    }

    let line_mark_record_indexes = row_matches
        .iter()
        .filter_map(|(_, line_mark, _)| line_mark.map(|line_mark| line_mark.record_index))
        .collect::<Vec<_>>();
    let matched_record_header_indexes = row_matches
        .iter()
        .filter_map(|(_, _, header_index)| *header_index)
        .collect::<Vec<_>>();
    let row_line_mark_match_count = line_mark_record_indexes.len();
    let row_header_match_count = matched_record_header_indexes.len();
    let all_rows_have_line_mark = row_line_mark_match_count == candidate.intervals().len();
    let all_rows_have_scanned_header = row_header_match_count == candidate.intervals().len();
    let first_record_header_index = matched_record_header_indexes.first().copied();
    let single_scanned_record_header_matched = first_record_header_index.is_some()
        && matched_record_header_indexes
            .iter()
            .all(|index| Some(*index) == first_record_header_index);
    let parsed_page_mark = document.page_marks().first();

    output.push_str("{\"source\":\"/PageMark raw record scan+/LineMark\"");
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"diagnosticOnly\":true");
    output.push_str(",\"streamByteLength\":");
    output.push_str(&page_mark_bytes.len().to_string());
    output.push_str(",\"parsedPageMarkFamily\":");
    match parsed_page_mark {
        Some(page_mark) => output.push_str(&json_string(page_mark.family())),
        None => output.push_str("null"),
    }
    output.push_str(",\"parsedPageMarkEntryCount\":");
    match parsed_page_mark {
        Some(page_mark) => output.push_str(&page_mark.entries().len().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"scannedRecordHeaderCount\":");
    output.push_str(&record_headers.len().to_string());
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"rowLineMarkMatchCount\":");
    output.push_str(&row_line_mark_match_count.to_string());
    output.push_str(",\"rowScannedRecordHeaderMatchCount\":");
    output.push_str(&row_header_match_count.to_string());
    output.push_str(",\"allRowsHaveLineMark\":");
    output.push_str(if all_rows_have_line_mark {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allRowsHaveScannedRecordHeader\":");
    output.push_str(if all_rows_have_scanned_header {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleScannedRecordHeaderMatched\":");
    output.push_str(if single_scanned_record_header_matched {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedScannedRecordHeaderIndex\":");
    push_option_usize_json(
        output,
        first_record_header_index.filter(|_| single_scanned_record_header_matched),
    );
    push_line_mark_record_stride_fields_json(output, &line_mark_record_indexes);
    output.push_str(",\"recordHeaders\":[");
    for (index, header) in record_headers.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let next_offset = record_headers
            .get(index + 1)
            .map(|next| next.offset)
            .unwrap_or(page_mark_bytes.len());
        output.push_str("{\"scanIndex\":");
        output.push_str(&index.to_string());
        output.push_str(",\"byteOffset\":");
        output.push_str(&header.offset.to_string());
        output.push_str(",\"nextByteOffset\":");
        output.push_str(&next_offset.to_string());
        output.push_str(",\"recordPayloadByteLength\":");
        output.push_str(&next_offset.saturating_sub(header.offset).to_string());
        output.push_str(",\"index\":");
        output.push_str(&header.index.to_string());
        output.push_str(",\"flags\":");
        output.push_str(&header.flags.to_string());
        output.push_str(",\"flagsHex\":");
        output.push_str(&json_string(&format!("0x{:08x}", header.flags)));
        output.push_str(",\"lineStart\":");
        output.push_str(&header.line_start.to_string());
        output.push_str(",\"lineEnd\":");
        output.push_str(&header.line_end.to_string());
        output.push_str(",\"lineCount\":");
        output.push_str(
            &header
                .line_end
                .saturating_sub(header.line_start)
                .saturating_add(1)
                .to_string(),
        );
        output.push('}');
    }
    output.push_str("],\"rows\":[");
    for (index, (row_index, line_mark, record_header_index)) in row_matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"lineMarkRecordIndex\":");
        match line_mark {
            Some(line_mark) => output.push_str(&line_mark.record_index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"scannedRecordHeaderIndex\":");
        push_option_usize_json(output, *record_header_index);
        output.push_str(",\"scannedRecordIndex\":");
        match record_header_index.and_then(|index| record_headers.get(index)) {
            Some(header) => output.push_str(&header.index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"scannedRecordLineStart\":");
        match record_header_index.and_then(|index| record_headers.get(index)) {
            Some(header) => output.push_str(&header.line_start.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"scannedRecordLineEnd\":");
        match record_header_index.and_then(|index| record_headers.get(index)) {
            Some(header) => output.push_str(&header.line_end.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"withinScannedRecordHeader\":");
        output.push_str(if record_header_index.is_some() {
            "true"
        } else {
            "false"
        });
        output.push('}');
    }
    output.push_str(
        "],\"renderPromotionContribution\":\"page-mark-raw-record-header-correlation-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-raw-record-scan-does-not-decode-y-transform",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_raw_record_source_range_evidence_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(summary) =
        table_grid_page_mark_raw_record_source_range_coverage_summary(document, candidate)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/PageMark raw record headers+table source unit ranges\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"recordHeaderCount\":");
    output.push_str(&summary.record_header_count.to_string());
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&summary.candidate_row_count.to_string());
    output.push_str(",\"rowSourceCoverageCount\":");
    output.push_str(&summary.row_source_coverage_count.to_string());
    output.push_str(",\"allRowsHaveHeaderCoverage\":");
    output.push_str(if summary.all_rows_have_header_coverage {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"totalOverlappingHeaderCount\":");
    output.push_str(&summary.total_overlapping_header_count.to_string());
    output.push_str(",\"matchedScanIndexes\":");
    push_usize_array_json(output, &summary.matched_scan_indexes);
    output.push_str(",\"matchedScanIndexesMonotonic\":");
    output.push_str(if summary.matched_scan_indexes_monotonic {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rows\":[");
    for (row_output_index, row) in summary.rows.iter().enumerate() {
        if row_output_index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row.row_index.to_string());
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(row.source_start, row.source_end));
        output.push_str(",\"overlappingHeaderCount\":");
        output.push_str(&row.matches.len().to_string());
        output.push_str(",\"overlappingHeaders\":[");
        for (match_index, match_) in row.matches.iter().enumerate() {
            if match_index > 0 {
                output.push(',');
            }
            output.push_str("{\"scanIndex\":");
            output.push_str(&match_.scan_index.to_string());
            output.push_str(",\"recordIndex\":");
            output.push_str(&match_.header.index.to_string());
            output.push_str(",\"recordLineStart\":");
            output.push_str(&match_.header.line_start.to_string());
            output.push_str(",\"recordLineEnd\":");
            output.push_str(&match_.header.line_end.to_string());
            output.push_str(",\"overlapUnitRange\":");
            output.push_str(&source_range_json(match_.overlap_start, match_.overlap_end));
            output.push_str(",\"overlapUnits\":");
            output.push_str(&match_.overlap_units.to_string());
            output.push('}');
        }
        output.push_str("]}");
    }
    output.push_str(
        "],\"renderPromotionContribution\":\"page-mark-record-source-range-coverage-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-record-source-ranges-do-not-decode-page-y-transform",
    ));
    output.push('}');
}

pub(crate) fn table_grid_page_mark_raw_record_source_range_coverage_summary(
    document: &Document,
    candidate: &TableCandidate,
) -> Option<TableGridPageMarkRawRecordSourceRangeCoverageSummary> {
    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let record_headers = page_mark_record_headers(page_mark_bytes);
    if record_headers.is_empty() {
        return None;
    }

    let rows = candidate
        .intervals()
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            let source_start = table_source_offset_to_units(candidate.basis(), row.source_start());
            let source_end = table_source_offset_to_units(candidate.basis(), row.source_end());
            let matches = record_headers
                .iter()
                .enumerate()
                .filter_map(|(scan_index, header)| {
                    let header_start = header.line_start as usize;
                    let header_end = header.line_end as usize;
                    let overlap_start = source_start.max(header_start);
                    let overlap_end = source_end.min(header_end);
                    (overlap_start <= overlap_end).then_some(
                        TableGridPageMarkRawRecordSourceRangeCoverageMatch {
                            scan_index,
                            header: *header,
                            overlap_start,
                            overlap_end,
                            overlap_units: overlap_end
                                .saturating_sub(overlap_start)
                                .saturating_add(1),
                        },
                    )
                })
                .collect::<Vec<_>>();
            TableGridPageMarkRawRecordSourceRangeCoverageRow {
                row_index,
                source_start,
                source_end,
                matches,
            }
        })
        .collect::<Vec<_>>();

    let row_source_coverage_count = rows.iter().filter(|row| !row.matches.is_empty()).count();
    let total_overlapping_header_count = rows.iter().map(|row| row.matches.len()).sum::<usize>();
    let all_rows_have_header_coverage = !rows.is_empty() && row_source_coverage_count == rows.len();
    let matched_scan_indexes = rows
        .iter()
        .flat_map(|row| row.matches.iter().map(|match_| match_.scan_index))
        .collect::<Vec<_>>();
    let matched_scan_indexes_monotonic =
        usize_values_are_monotonic_non_decreasing(&matched_scan_indexes);

    Some(TableGridPageMarkRawRecordSourceRangeCoverageSummary {
        record_header_count: record_headers.len(),
        candidate_row_count: rows.len(),
        row_source_coverage_count,
        all_rows_have_header_coverage,
        total_overlapping_header_count,
        matched_scan_indexes,
        matched_scan_indexes_monotonic,
        rows,
    })
}

pub(crate) fn push_table_grid_page_mark_raw_record_source_range_coverage_summary_json(
    output: &mut String,
    summary: &TableGridPageMarkRawRecordSourceRangeCoverageSummary,
) {
    output.push_str("{\"candidateRowCount\":");
    output.push_str(&summary.candidate_row_count.to_string());
    output.push_str(",\"rowSourceCoverageCount\":");
    output.push_str(&summary.row_source_coverage_count.to_string());
    output.push_str(",\"allRowsHaveHeaderCoverage\":");
    output.push_str(if summary.all_rows_have_header_coverage {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"totalOverlappingHeaderCount\":");
    output.push_str(&summary.total_overlapping_header_count.to_string());
    output.push_str(",\"matchedScanIndexes\":");
    push_usize_array_json(output, &summary.matched_scan_indexes);
    output.push_str(",\"matchedScanIndexesMonotonic\":");
    output.push_str(if summary.matched_scan_indexes_monotonic {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_scoped_y_transform_probe_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        output.push_str("null");
        return;
    };
    let Some(page_mark) = document.page_marks().first() else {
        output.push_str("null");
        return;
    };
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let Some(reference_layout) =
        diagnostic_reference_table_grid_overlay_layout(layout, document, candidate, column_count)
    else {
        output.push_str("null");
        return;
    };
    let row_count = candidate.intervals().len();
    if row_count == 0 {
        output.push_str("null");
        return;
    }

    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        output.push_str("null");
        return;
    }
    let record_headers = page_mark_record_headers(page_mark_bytes);
    let mut line_mark_record_indexes = Vec::new();
    let mut parsed_entry_indexes = Vec::new();
    let mut raw_header_indexes = Vec::new();
    for row in candidate.intervals() {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start());
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end());
        let Some(line_mark) = best_line_mark_interval_for_unit_range(
            &line_mark_intervals,
            row_unit_start,
            row_unit_end,
        ) else {
            continue;
        };
        line_mark_record_indexes.push(line_mark.record_index);
        if let Some(entry_index) = page_mark.entries().iter().position(|entry| {
            let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                return false;
            };
            let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                return false;
            };
            line_start <= line_mark.record_index && line_mark.record_index <= line_end
        }) {
            parsed_entry_indexes.push(entry_index);
        }
        if let Some(header_index) = record_headers.iter().position(|header| {
            header.line_start as usize <= line_mark.record_index
                && line_mark.record_index <= header.line_end as usize
        }) {
            raw_header_indexes.push(header_index);
        }
    }

    let single_parsed_entry_index = single_usize_value(&parsed_entry_indexes);
    let single_raw_header_index = single_usize_value(&raw_header_indexes);
    let mut value_candidates = Vec::new();
    if let Some(entry_index) = single_parsed_entry_index
        && let Some(entry) = page_mark.entries().get(entry_index)
    {
        collect_page_mark_entry_y_value_candidates(&mut value_candidates, entry);
    }
    if let Some(header_index) = single_raw_header_index
        && let Some(header) = record_headers.get(header_index)
    {
        let next_offset = record_headers
            .get(header_index + 1)
            .map(|next| next.offset)
            .unwrap_or(page_mark_bytes.len());
        collect_page_mark_raw_header_y_value_candidates(
            &mut value_candidates,
            page_mark_bytes,
            *header,
            next_offset,
        );
    }

    let row_top_targets = (0..row_count)
        .map(|row_index| reference_layout.y + row_index as f32 * reference_layout.row_height)
        .collect::<Vec<_>>();
    let row_delta_targets = row_top_targets
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();
    const TOLERANCE_PX: f32 = 2.0;

    output.push_str("{\"source\":\"/PageMark scoped raw fields+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"tolerancePx\":");
    output.push_str(&format!("{TOLERANCE_PX:.3}"));
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &line_mark_record_indexes);
    output.push_str(",\"parsedEntryMatchCount\":");
    output.push_str(&parsed_entry_indexes.len().to_string());
    output.push_str(",\"singleParsedPageMarkEntryMatched\":");
    output.push_str(if single_parsed_entry_index.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedParsedPageMarkEntryIndex\":");
    push_option_usize_json(output, single_parsed_entry_index);
    output.push_str(",\"rawHeaderMatchCount\":");
    output.push_str(&raw_header_indexes.len().to_string());
    output.push_str(",\"singleRawRecordHeaderMatched\":");
    output.push_str(if single_raw_header_index.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedRawRecordHeaderIndex\":");
    push_option_usize_json(output, single_raw_header_index);
    output.push_str(",\"referenceBBoxUsed\":true");
    output.push_str(",\"referenceTargetBasis\":\"referenceTableBBox.rowTopTargets\"");
    output.push_str(
        ",\"sourceOnlyReplacementBlockedReason\":\"page-mark-scoped-y-transform-targets-reference-backed\"",
    );
    output.push_str(",\"valueCandidateCount\":");
    output.push_str(&value_candidates.len().to_string());
    output.push_str(",\"rowTopTargets\":");
    push_f32_array_json(output, &row_top_targets);
    output.push_str(",\"rowDeltaTargets\":");
    push_f32_array_json(output, &row_delta_targets);
    output.push_str(",\"rowTopNearestCandidates\":");
    push_page_mark_scoped_nearest_y_candidates_json(output, &row_top_targets, &value_candidates);
    output.push_str(",\"rowDeltaCandidatePolicy\":");
    output.push_str(&json_string("adjacent-ordered-candidate-value-delta"));
    output.push_str(",\"rowDeltaNearestCandidates\":");
    push_page_mark_scoped_nearest_delta_candidates_json(
        output,
        &row_delta_targets,
        &value_candidates,
    );
    output.push_str(",\"rowTopHitSummary\":");
    push_page_mark_scoped_y_hit_summary_json(
        output,
        &row_top_targets,
        &value_candidates,
        TOLERANCE_PX,
    );
    output.push_str(",\"rowDeltaHitSummary\":");
    push_page_mark_scoped_delta_hit_summary_json(
        output,
        &row_delta_targets,
        &value_candidates,
        TOLERANCE_PX,
    );
    output.push_str(",\"sharedFieldFamilyResiduals\":");
    push_page_mark_scoped_y_shared_field_family_residuals_json(
        output,
        page_mark.family(),
        page_mark,
        page_mark_bytes,
        &record_headers,
        single_parsed_entry_index,
        &line_mark_record_indexes,
        &row_top_targets,
        &row_delta_targets,
        TOLERANCE_PX,
    );
    output.push_str(",\"slotScopedSubrecordYSequenceComparison\":");
    push_page_mark_slot_scoped_subrecord_y_sequence_comparison_json(
        output,
        layout,
        page_mark,
        page_mark_bytes,
        &record_headers,
        single_parsed_entry_index,
        single_raw_header_index,
        &line_mark_record_indexes,
        &row_top_targets,
        &row_delta_targets,
        TOLERANCE_PX,
    );
    output.push_str(",\"previousRowSpanScopedYTransformProbe\":");
    let previous_row_span_record_indexes =
        table_grid_previous_row_span_line_mark_record_indexes(document, candidate);
    push_page_mark_scoped_y_record_set_probe_json(
        output,
        "previous-row-span-line-mark-records",
        layout,
        page_mark,
        page_mark_bytes,
        &record_headers,
        &previous_row_span_record_indexes,
        &row_top_targets,
        &row_delta_targets,
        TOLERANCE_PX,
    );
    output.push_str(",\"absoluteVsSpanLineageGate\":");
    push_table_grid_page_mark_y_candidate_lineage_gate_json(
        output,
        &reference_layout,
        &row_top_targets,
        &row_delta_targets,
        &value_candidates,
        document,
        candidate,
        TOLERANCE_PX,
    );
    output.push_str(",\"subrecordLineSpanCorrelation\":");
    push_table_grid_page_mark_subrecord_line_span_correlation_json(
        output,
        page_mark,
        page_mark_bytes,
        &record_headers,
        document,
        candidate,
    );
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-scoped-y-transform-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-scoped-y-transform-field-family-unproven",
    ));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_page_mark_y_candidate_lineage_gate_json(
    output: &mut String,
    reference_layout: &TableGridReferenceLayout,
    row_top_targets: &[f32],
    row_delta_targets: &[f32],
    value_candidates: &[PageMarkScopedYValueCandidate],
    document: &Document,
    candidate: &TableCandidate,
    tolerance_px: f32,
) {
    let (row_top_target_hit_count, row_top_total_hit_count) =
        page_mark_scoped_y_target_hit_counts(row_top_targets, value_candidates, tolerance_px);
    let (row_delta_target_hit_count, row_delta_total_hit_count) =
        page_mark_scoped_delta_target_hit_counts(row_delta_targets, value_candidates, tolerance_px);
    let subrecord_span_readiness =
        table_grid_page_mark_subrecord_line_span_readiness(document, candidate);
    let selected_complete = subrecord_span_readiness.as_ref().is_some_and(|readiness| {
        !readiness.selected_post_row_gap_span_targets.is_empty()
            && readiness.selected_post_row_gap_span_hit_count
                == readiness.selected_post_row_gap_span_targets.len()
    });
    let previous_complete = subrecord_span_readiness.as_ref().is_some_and(|readiness| {
        !readiness.previous_row_span_targets.is_empty()
            && readiness.previous_row_span_hit_count == readiness.previous_row_span_targets.len()
    });
    let compact_complete = subrecord_span_readiness.as_ref().is_some_and(|readiness| {
        !readiness.compact_row_span_targets.is_empty()
            && readiness.compact_row_span_hit_count == readiness.compact_row_span_targets.len()
    });

    let mut blocked_reasons = Vec::new();
    blocked_reasons.push("absolute-table-top-targets-reference-backed");
    if subrecord_span_readiness.is_some() {
        blocked_reasons.push("source-subrecord-spans-are-line-span-targets");
    } else {
        blocked_reasons.push("source-subrecord-span-correlation-absent");
    }
    if selected_complete {
        blocked_reasons.push("selected-post-row-gap-spans-do-not-decode-y-origin");
    }
    if previous_complete {
        blocked_reasons.push("previous-row-span-spans-do-not-decode-y-origin");
    }
    if compact_complete {
        blocked_reasons.push("compact-row-spans-do-not-decode-y-origin");
    }
    blocked_reasons.push("source-only-absolute-table-top-field-unproven");

    output.push_str(
        "{\"source\":\"/PageMark y-candidate lineage: reference table-top vs source line-span\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output
        .push_str(",\"referenceBBoxUsed\":true,\"selectionReady\":false,\"promotionReady\":false");
    output.push_str(",\"lineageClassification\":");
    output.push_str(&json_string(
        "reference-absolute-table-top-vs-source-line-span-correlation",
    ));
    output.push_str(",\"absoluteTableTopProbe\":{\"source\":\"referenceTableBBox.rowTopTargets\",\"referenceBacked\":true,\"sourceBacked\":false");
    output.push_str(",\"referenceTableTopY\":");
    output.push_str(&format!("{:.3}", reference_layout.y));
    output.push_str(",\"referenceRowHeight\":");
    output.push_str(&format!("{:.3}", reference_layout.row_height));
    output.push_str(",\"rowTopTargets\":");
    push_f32_array_json(output, row_top_targets);
    output.push_str(",\"rowDeltaTargets\":");
    push_f32_array_json(output, row_delta_targets);
    output.push_str(",\"scopedRowTopTargetHitCount\":");
    output.push_str(&row_top_target_hit_count.to_string());
    output.push_str(",\"scopedRowTopTotalHitCount\":");
    output.push_str(&row_top_total_hit_count.to_string());
    output.push_str(",\"scopedRowDeltaTargetHitCount\":");
    output.push_str(&row_delta_target_hit_count.to_string());
    output.push_str(",\"scopedRowDeltaTotalHitCount\":");
    output.push_str(&row_delta_total_hit_count.to_string());
    output.push_str(",\"sourceOnlyAbsoluteTopDecoded\":false}");
    output.push_str(",\"sourceSpanProbe\":{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\",\"referenceBacked\":false,\"sourceBacked\":true,\"spanEvidencePositional\":false,\"present\":");
    output.push_str(if subrecord_span_readiness.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedSpanRole\":\"post-row-gap\"");
    output.push_str(",\"previousSpanRole\":\"compact-row-span\"");
    output.push_str(",\"selectedPostRowGapSpanTargets\":");
    match subrecord_span_readiness.as_ref() {
        Some(readiness) => {
            push_usize_array_json(output, &readiness.selected_post_row_gap_span_targets)
        }
        None => output.push_str("[]"),
    }
    output.push_str(",\"selectedPostRowGapSpanHitCount\":");
    output.push_str(
        &subrecord_span_readiness
            .as_ref()
            .map(|readiness| readiness.selected_post_row_gap_span_hit_count)
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanTargets\":");
    match subrecord_span_readiness.as_ref() {
        Some(readiness) => push_usize_array_json(output, &readiness.previous_row_span_targets),
        None => output.push_str("[]"),
    }
    output.push_str(",\"previousRowSpanHitCount\":");
    output.push_str(
        &subrecord_span_readiness
            .as_ref()
            .map(|readiness| readiness.previous_row_span_hit_count)
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"compactRowSpanTargets\":");
    match subrecord_span_readiness.as_ref() {
        Some(readiness) => push_usize_array_json(output, &readiness.compact_row_span_targets),
        None => output.push_str("[]"),
    }
    output.push_str(",\"compactRowSpanHitCount\":");
    output.push_str(
        &subrecord_span_readiness
            .as_ref()
            .map(|readiness| readiness.compact_row_span_hit_count)
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"compactRowSpanComplete\":");
    output.push_str(if compact_complete { "true" } else { "false" });
    output.push('}');
    output.push_str(",\"sourceOnlyLineageConclusion\":");
    output.push_str(&json_string(
        "source-spans-corroborate-spacing-or-row-span-lengths-not-absolute-y",
    ));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"page-mark-y-candidate-lineage-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "absolute-top-evidence-reference-backed-span-evidence-non-positional",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_subrecord_line_span_correlation_json(
    output: &mut String,
    page_mark: &DocumentPageMark,
    page_mark_bytes: &[u8],
    record_headers: &[PageMarkRecordHeader],
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(sibling) = table_grid_sparse_table_sibling_evidence(document, candidate) else {
        output.push_str("null");
        return;
    };
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        output.push_str("null");
        return;
    }
    let rows =
        table_grid_line_mark_row_gap_sequence_rows(candidate, &sibling, &line_mark_intervals);
    if rows.is_empty() {
        output.push_str("null");
        return;
    }

    let selected_record_indexes = rows
        .iter()
        .map(|row| row.selected_line_mark.record_index)
        .collect::<Vec<_>>();
    let previous_record_indexes = rows
        .iter()
        .filter_map(|row| {
            row.previous_line_mark
                .map(|line_mark| line_mark.record_index)
        })
        .collect::<Vec<_>>();
    let selected_post_row_gap_spans = rows
        .iter()
        .map(|row| line_mark_interval_span_units(row.selected_line_mark))
        .collect::<Vec<_>>();
    let previous_row_spans = rows
        .iter()
        .filter_map(|row| row.previous_line_mark.map(line_mark_interval_span_units))
        .collect::<Vec<_>>();
    let compact_row_spans = rows
        .iter()
        .map(|row| {
            row.row_source_end_units
                .saturating_sub(row.row_source_start_units)
        })
        .collect::<Vec<_>>();
    let post_row_gap_spans = rows
        .iter()
        .filter_map(|row| {
            row.post_row_gap
                .as_ref()
                .map(|gap| gap.source_end.saturating_sub(gap.source_start))
        })
        .collect::<Vec<_>>();
    let candidates = page_mark_raw_subrecord_line_span_candidates(
        page_mark_bytes,
        record_headers,
        page_mark_subrecord_line_range_max_candidate(page_mark, record_headers),
    );
    if candidates.is_empty() {
        output.push_str("null");
        return;
    }

    let selected_nearest =
        page_mark_subrecord_nearest_line_span_matches(&selected_post_row_gap_spans, &candidates);
    let previous_nearest =
        page_mark_subrecord_nearest_line_span_matches(&previous_row_spans, &candidates);
    let compact_nearest =
        page_mark_subrecord_nearest_line_span_matches(&compact_row_spans, &candidates);
    let selected_coverage = table_grid_page_mark_subrecord_line_span_coverage(
        &selected_record_indexes,
        &selected_nearest,
    );
    let previous_coverage = table_grid_page_mark_subrecord_line_span_coverage(
        &previous_record_indexes,
        &previous_nearest,
    );
    let selected_hit_count = selected_nearest
        .iter()
        .filter(|match_| {
            match_.residual_units.abs() <= TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS
        })
        .count();
    let previous_hit_count = previous_nearest
        .iter()
        .filter(|match_| {
            match_.residual_units.abs() <= TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS
        })
        .count();
    let compact_hit_count = compact_nearest
        .iter()
        .filter(|match_| {
            match_.residual_units.abs() <= TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS
        })
        .count();

    output.push_str("{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"spanToleranceUnits\":");
    output.push_str(&TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS.to_string());
    output.push_str(",\"selectedSpacingRecordIndexes\":");
    push_usize_array_json(output, &selected_record_indexes);
    output.push_str(",\"previousRowSpanRecordIndexes\":");
    push_usize_array_json(output, &previous_record_indexes);
    output.push_str(",\"selectedPostRowGapSpanTargets\":");
    push_usize_array_json(output, &selected_post_row_gap_spans);
    output.push_str(",\"postRowGapSpanTargets\":");
    push_usize_array_json(output, &post_row_gap_spans);
    output.push_str(",\"previousRowSpanTargets\":");
    push_usize_array_json(output, &previous_row_spans);
    output.push_str(",\"compactRowSpanTargets\":");
    push_usize_array_json(output, &compact_row_spans);
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidates.len().to_string());
    output.push_str(",\"selectedPostRowGapSpanHitCount\":");
    output.push_str(&selected_hit_count.to_string());
    output.push_str(",\"previousRowSpanHitCount\":");
    output.push_str(&previous_hit_count.to_string());
    output.push_str(",\"compactRowSpanHitCount\":");
    output.push_str(&compact_hit_count.to_string());
    output.push_str(",\"selectedPostRowGapSpanMaxAbsResidualUnits\":");
    push_optional_i32_json(
        output,
        max_abs_i32(
            &selected_nearest
                .iter()
                .map(|match_| match_.residual_units)
                .collect::<Vec<_>>(),
        ),
    );
    output.push_str(",\"previousRowSpanMaxAbsResidualUnits\":");
    push_optional_i32_json(
        output,
        max_abs_i32(
            &previous_nearest
                .iter()
                .map(|match_| match_.residual_units)
                .collect::<Vec<_>>(),
        ),
    );
    output.push_str(",\"compactRowSpanMaxAbsResidualUnits\":");
    push_optional_i32_json(
        output,
        max_abs_i32(
            &compact_nearest
                .iter()
                .map(|match_| match_.residual_units)
                .collect::<Vec<_>>(),
        ),
    );
    output.push_str(",\"nearestSelectedPostRowGapSpanCandidates\":");
    push_page_mark_subrecord_line_span_matches_json(output, &selected_nearest);
    output.push_str(",\"nearestPreviousRowSpanCandidates\":");
    push_page_mark_subrecord_line_span_matches_json(output, &previous_nearest);
    output.push_str(",\"nearestCompactRowSpanCandidates\":");
    push_page_mark_subrecord_line_span_matches_json(output, &compact_nearest);
    output.push_str(",\"selectedPostRowGapSpanOrderedCoverage\":");
    push_table_grid_page_mark_subrecord_line_span_coverage_json(output, &selected_coverage);
    output.push_str(",\"previousRowSpanOrderedCoverage\":");
    push_table_grid_page_mark_subrecord_line_span_coverage_json(output, &previous_coverage);
    output.push_str(",\"sampleCandidates\":[");
    for (index, candidate) in candidates.iter().take(12).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_raw_subrecord_line_span_candidate_json(output, candidate);
    }
    output.push_str(
        "],\"renderPromotionContribution\":\"page-mark-subrecord-line-span-correlation-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "subrecord-line-span-correlation-does-not-decode-page-y-transform",
    ));
    output.push('}');
}

pub(crate) const TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS: i32 = 3;

pub(crate) fn table_grid_page_mark_subrecord_line_span_readiness(
    document: &Document,
    candidate: &TableCandidate,
) -> Option<TableGridPageMarkSubrecordLineSpanReadiness> {
    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let page_mark = document.page_marks().first()?;
    let record_headers = page_mark_record_headers(page_mark_bytes);
    if record_headers.is_empty() {
        return None;
    }
    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate)?;
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        return None;
    }
    let rows =
        table_grid_line_mark_row_gap_sequence_rows(candidate, &sibling, &line_mark_intervals);
    if rows.is_empty() {
        return None;
    }

    let selected_record_indexes = rows
        .iter()
        .map(|row| row.selected_line_mark.record_index)
        .collect::<Vec<_>>();
    let previous_record_indexes = rows
        .iter()
        .filter_map(|row| {
            row.previous_line_mark
                .map(|line_mark| line_mark.record_index)
        })
        .collect::<Vec<_>>();
    let selected_post_row_gap_span_targets = rows
        .iter()
        .map(|row| line_mark_interval_span_units(row.selected_line_mark))
        .collect::<Vec<_>>();
    let previous_row_span_targets = rows
        .iter()
        .filter_map(|row| row.previous_line_mark.map(line_mark_interval_span_units))
        .collect::<Vec<_>>();
    let compact_row_span_targets = rows
        .iter()
        .map(|row| {
            row.row_source_end_units
                .saturating_sub(row.row_source_start_units)
        })
        .collect::<Vec<_>>();
    let post_row_gap_span_targets = rows
        .iter()
        .filter_map(|row| {
            row.post_row_gap
                .as_ref()
                .map(|gap| gap.source_end.saturating_sub(gap.source_start))
        })
        .collect::<Vec<_>>();
    let candidates = page_mark_raw_subrecord_line_span_candidates(
        page_mark_bytes,
        &record_headers,
        page_mark_subrecord_line_range_max_candidate(page_mark, &record_headers),
    );
    if candidates.is_empty() {
        return None;
    }

    let selected_nearest = page_mark_subrecord_nearest_line_span_matches(
        &selected_post_row_gap_span_targets,
        &candidates,
    );
    let previous_nearest =
        page_mark_subrecord_nearest_line_span_matches(&previous_row_span_targets, &candidates);
    let compact_nearest =
        page_mark_subrecord_nearest_line_span_matches(&compact_row_span_targets, &candidates);
    let selected_post_row_gap_span_coverage = table_grid_page_mark_subrecord_line_span_coverage(
        &selected_record_indexes,
        &selected_nearest,
    );
    let previous_row_span_coverage = table_grid_page_mark_subrecord_line_span_coverage(
        &previous_record_indexes,
        &previous_nearest,
    );
    let hit_count = |matches: &[PageMarkSubrecordLineSpanMatch<'_>]| {
        matches
            .iter()
            .filter(|match_| {
                match_.residual_units.abs() <= TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS
            })
            .count()
    };
    let max_abs_residual = |matches: &[PageMarkSubrecordLineSpanMatch<'_>]| {
        max_abs_i32(
            &matches
                .iter()
                .map(|match_| match_.residual_units)
                .collect::<Vec<_>>(),
        )
    };

    Some(TableGridPageMarkSubrecordLineSpanReadiness {
        selected_record_indexes,
        previous_record_indexes,
        selected_post_row_gap_span_targets,
        post_row_gap_span_targets,
        previous_row_span_targets,
        compact_row_span_targets,
        candidate_count: candidates.len(),
        selected_post_row_gap_span_hit_count: hit_count(&selected_nearest),
        previous_row_span_hit_count: hit_count(&previous_nearest),
        compact_row_span_hit_count: hit_count(&compact_nearest),
        selected_post_row_gap_span_max_abs_residual_units: max_abs_residual(&selected_nearest),
        previous_row_span_max_abs_residual_units: max_abs_residual(&previous_nearest),
        compact_row_span_max_abs_residual_units: max_abs_residual(&compact_nearest),
        selected_post_row_gap_span_coverage,
        previous_row_span_coverage,
    })
}

pub(crate) fn push_table_grid_page_mark_subrecord_line_span_readiness_json(
    output: &mut String,
    readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let Some(readiness) = readiness else {
        output.push_str("null");
        return;
    };
    let selected_complete = !readiness.selected_post_row_gap_span_targets.is_empty()
        && readiness.selected_post_row_gap_span_hit_count
            == readiness.selected_post_row_gap_span_targets.len();
    let previous_complete = !readiness.previous_row_span_targets.is_empty()
        && readiness.previous_row_span_hit_count == readiness.previous_row_span_targets.len();
    let compact_complete = !readiness.compact_row_span_targets.is_empty()
        && readiness.compact_row_span_hit_count == readiness.compact_row_span_targets.len();

    output.push_str("{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"spanToleranceUnits\":");
    output.push_str(&TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS.to_string());
    output.push_str(",\"selectedSpacingRecordIndexes\":");
    push_usize_array_json(output, &readiness.selected_record_indexes);
    output.push_str(",\"previousRowSpanRecordIndexes\":");
    push_usize_array_json(output, &readiness.previous_record_indexes);
    output.push_str(",\"selectedPostRowGapSpanTargets\":");
    push_usize_array_json(output, &readiness.selected_post_row_gap_span_targets);
    output.push_str(",\"postRowGapSpanTargets\":");
    push_usize_array_json(output, &readiness.post_row_gap_span_targets);
    output.push_str(",\"previousRowSpanTargets\":");
    push_usize_array_json(output, &readiness.previous_row_span_targets);
    output.push_str(",\"compactRowSpanTargets\":");
    push_usize_array_json(output, &readiness.compact_row_span_targets);
    output.push_str(",\"candidateCount\":");
    output.push_str(&readiness.candidate_count.to_string());
    output.push_str(",\"selectedPostRowGapSpanHitCount\":");
    output.push_str(&readiness.selected_post_row_gap_span_hit_count.to_string());
    output.push_str(",\"previousRowSpanHitCount\":");
    output.push_str(&readiness.previous_row_span_hit_count.to_string());
    output.push_str(",\"compactRowSpanHitCount\":");
    output.push_str(&readiness.compact_row_span_hit_count.to_string());
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"compactRowSpanComplete\":");
    output.push_str(if compact_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanMaxAbsResidualUnits\":");
    push_optional_i32_json(
        output,
        readiness.selected_post_row_gap_span_max_abs_residual_units,
    );
    output.push_str(",\"previousRowSpanMaxAbsResidualUnits\":");
    push_optional_i32_json(output, readiness.previous_row_span_max_abs_residual_units);
    output.push_str(",\"compactRowSpanMaxAbsResidualUnits\":");
    push_optional_i32_json(output, readiness.compact_row_span_max_abs_residual_units);
    output.push_str(",\"subrecordSpanRoleGate\":");
    push_table_grid_page_mark_subrecord_span_role_gate_json(
        output,
        readiness,
        selected_complete,
        previous_complete,
        compact_complete,
    );
    output.push_str(",\"pageYTransformDecoded\":false");
    output.push_str(",\"selectedPostRowGapSpanOrderedCoverage\":");
    push_table_grid_page_mark_subrecord_line_span_coverage_json(
        output,
        &readiness.selected_post_row_gap_span_coverage,
    );
    output.push_str(",\"previousRowSpanOrderedCoverage\":");
    push_table_grid_page_mark_subrecord_line_span_coverage_json(
        output,
        &readiness.previous_row_span_coverage,
    );
    output
        .push_str(",\"renderPromotionContribution\":\"stride-gate-subrecord-line-span-readiness\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "subrecord-line-spans-match-line-ranges-not-page-y-origin",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_subrecord_span_role_gate_json(
    output: &mut String,
    readiness: &TableGridPageMarkSubrecordLineSpanReadiness,
    selected_complete: bool,
    previous_complete: bool,
    compact_complete: bool,
) {
    let selected_hit_count = readiness.selected_post_row_gap_span_hit_count;
    let previous_hit_count = readiness.previous_row_span_hit_count;
    let compact_hit_count = readiness.compact_row_span_hit_count;
    let row_span_hit_count = previous_hit_count.max(compact_hit_count);
    let row_span_target_count = readiness
        .previous_row_span_targets
        .len()
        .max(readiness.compact_row_span_targets.len());
    let selected_target_count = readiness.selected_post_row_gap_span_targets.len();
    let row_span_complete = previous_complete || compact_complete;
    let selected_role_dominant = selected_hit_count > 0 && selected_hit_count > row_span_hit_count;
    let row_span_role_dominant = row_span_hit_count > 0 && row_span_hit_count >= selected_hit_count;
    let (dominant_span_role, dominant_span_role_hit_count) = if selected_role_dominant {
        ("selected-post-row-gap", selected_hit_count)
    } else if previous_hit_count >= compact_hit_count && previous_hit_count > 0 {
        ("previous-row-span", previous_hit_count)
    } else if compact_hit_count > 0 {
        ("compact-row-span", compact_hit_count)
    } else {
        ("none", 0)
    };

    let mut blocked_reasons = Vec::new();
    if selected_role_dominant {
        blocked_reasons.push("subrecord-spans-prefer-post-row-gap-family");
    }
    if !selected_complete {
        blocked_reasons.push("selected-post-row-gap-span-incomplete");
    }
    if !row_span_complete {
        blocked_reasons.push("row-span-family-not-covered-by-subrecords");
    }
    if selected_role_dominant && !row_span_complete {
        blocked_reasons.push("post-row-gap-match-is-not-visible-row-height");
    }
    blocked_reasons.push("subrecord-span-role-semantics-unproven");
    blocked_reasons.push("page-y-origin-transform-undecoded");

    output.push_str("{\"source\":\"/PageMark raw u16 subrecord line-span role classifier\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"dominantSpanRole\":");
    output.push_str(&json_string(dominant_span_role));
    output.push_str(",\"dominantSpanRoleHitCount\":");
    output.push_str(&dominant_span_role_hit_count.to_string());
    output.push_str(",\"selectedPostRowGapSpanHitCount\":");
    output.push_str(&selected_hit_count.to_string());
    output.push_str(",\"selectedPostRowGapSpanTargetCount\":");
    output.push_str(&selected_target_count.to_string());
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"rowSpanHitCount\":");
    output.push_str(&row_span_hit_count.to_string());
    output.push_str(",\"rowSpanTargetCount\":");
    output.push_str(&row_span_target_count.to_string());
    output.push_str(",\"previousRowSpanHitCount\":");
    output.push_str(&previous_hit_count.to_string());
    output.push_str(",\"compactRowSpanHitCount\":");
    output.push_str(&compact_hit_count.to_string());
    output.push_str(",\"rowSpanComplete\":");
    output.push_str(if row_span_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapRoleDominant\":");
    output.push_str(if selected_role_dominant {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSpanRoleDominant\":");
    output.push_str(if row_span_role_dominant {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineSpanRoleDecoded\":false,\"pageYTransformDecoded\":false");
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"page-mark-subrecord-span-role-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-subrecord-spans-match-post-row-gaps-not-row-heights",
    ));
    output.push('}');
}

pub(crate) fn table_grid_page_mark_subrecord_line_span_coverage(
    record_indexes: &[usize],
    matches: &[PageMarkSubrecordLineSpanMatch<'_>],
) -> TableGridPageMarkSubrecordLineSpanCoverage {
    let mut matched_record_indexes = Vec::new();
    let mut matched_candidate_byte_offsets = Vec::new();
    for match_ in matches {
        if match_.residual_units.abs() > TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS {
            continue;
        }
        let Some(record_index) = record_indexes.get(match_.target_index).copied() else {
            continue;
        };
        matched_record_indexes.push(record_index);
        matched_candidate_byte_offsets.push(match_.candidate.byte_offset);
    }

    let mut candidate_counts = BTreeMap::<usize, usize>::new();
    for byte_offset in &matched_candidate_byte_offsets {
        *candidate_counts.entry(*byte_offset).or_default() += 1;
    }
    let unique_candidate_byte_offsets = candidate_counts.keys().copied().collect::<Vec<_>>();
    let duplicate_candidate_byte_offsets = candidate_counts
        .iter()
        .filter_map(|(byte_offset, count)| (*count > 1).then_some(*byte_offset))
        .collect::<Vec<_>>();
    let ordered_unique_coverage_complete = !record_indexes.is_empty()
        && matched_record_indexes.len() == record_indexes.len()
        && unique_candidate_byte_offsets.len() == record_indexes.len();

    TableGridPageMarkSubrecordLineSpanCoverage {
        matched_record_indexes,
        matched_candidate_byte_offsets,
        unique_candidate_byte_offsets,
        duplicate_candidate_byte_offsets,
        ordered_unique_coverage_complete,
    }
}

pub(crate) fn push_table_grid_page_mark_subrecord_line_span_coverage_json(
    output: &mut String,
    coverage: &TableGridPageMarkSubrecordLineSpanCoverage,
) {
    output.push_str("{\"policy\":");
    output.push_str(&json_string(
        "one-tolerance-hit-with-unique-subrecord-candidate-per-line-mark-record",
    ));
    output.push_str(",\"matchedRecordIndexes\":");
    push_usize_array_json(output, &coverage.matched_record_indexes);
    output.push_str(",\"matchedCandidateByteOffsets\":");
    push_usize_array_json(output, &coverage.matched_candidate_byte_offsets);
    output.push_str(",\"uniqueCandidateByteOffsets\":");
    push_usize_array_json(output, &coverage.unique_candidate_byte_offsets);
    output.push_str(",\"duplicateCandidateByteOffsets\":");
    push_usize_array_json(output, &coverage.duplicate_candidate_byte_offsets);
    output.push_str(",\"matchedRecordCount\":");
    output.push_str(&coverage.matched_record_indexes.len().to_string());
    output.push_str(",\"uniqueCandidateCount\":");
    output.push_str(&coverage.unique_candidate_byte_offsets.len().to_string());
    output.push_str(",\"duplicateCandidateReuseCount\":");
    output.push_str(&coverage.duplicate_candidate_byte_offsets.len().to_string());
    output.push_str(",\"orderedUniqueCoverageComplete\":");
    output.push_str(if coverage.ordered_unique_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn table_grid_cross_table_subrecord_ordering_probe(
    document: &Document,
    candidate: &TableCandidate,
) -> Option<TableGridCrossTableSubrecordOrderingProbe> {
    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let page_mark = document.page_marks().first()?;
    let record_headers = page_mark_record_headers(page_mark_bytes);
    if record_headers.is_empty() {
        return None;
    }
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        return None;
    }
    let current_sibling_index = table_grid_sparse_table_sibling_evidence(document, candidate)?
        .sparse_candidate
        .index();
    let subrecord_candidates = page_mark_raw_subrecord_line_span_candidates(
        page_mark_bytes,
        &record_headers,
        page_mark_subrecord_line_range_max_candidate(page_mark, &record_headers),
    );
    if subrecord_candidates.is_empty() {
        return None;
    }

    let mut related_candidates = document
        .table_candidates()
        .iter()
        .filter(|related| related.is_document_text_control_run_candidate())
        .filter(|related| {
            table_grid_sparse_table_sibling_evidence(document, related)
                .is_some_and(|evidence| evidence.sparse_candidate.index() == current_sibling_index)
        })
        .collect::<Vec<_>>();
    related_candidates.sort_by(|left, right| {
        left.source_start()
            .cmp(&right.source_start())
            .then_with(|| left.index().cmp(&right.index()))
    });
    if related_candidates.len() < 2 {
        return None;
    }

    let mut tables = Vec::new();
    for related in related_candidates {
        let Some(sibling) = table_grid_sparse_table_sibling_evidence(document, related) else {
            continue;
        };
        let rows =
            table_grid_line_mark_row_gap_sequence_rows(related, &sibling, &line_mark_intervals);
        if rows.is_empty() {
            continue;
        }
        let targets = rows
            .iter()
            .map(|row| line_mark_interval_span_units(row.selected_line_mark))
            .collect::<Vec<_>>();
        let nearest =
            page_mark_subrecord_nearest_line_span_matches(&targets, &subrecord_candidates);
        let mut matched_rows = Vec::new();
        for match_ in nearest {
            if match_.residual_units.abs() > TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS {
                continue;
            }
            let Some(row) = rows.get(match_.target_index) else {
                continue;
            };
            let candidate = match_.candidate;
            matched_rows.push(TableGridCrossTableSubrecordOrderingMatch {
                row_index: row.compact_row_index,
                line_mark_record_index: row.selected_line_mark.record_index,
                target_units: match_.target_units,
                residual_units: match_.residual_units,
                byte_offset: candidate.byte_offset,
                raw_record_index: candidate.raw_record_index,
                raw_record_scan_index: candidate.raw_record_scan_index,
                tail_block16_word_index: candidate.tail_block16_word_index,
                line_start_candidate: candidate.line_start_candidate,
                line_end_candidate: candidate.line_end_candidate,
                field2_value: candidate.field2_value,
            });
        }
        if matched_rows.is_empty() {
            continue;
        }
        tables.push(TableGridCrossTableSubrecordOrderingTable {
            table_candidate_index: related.index(),
            source_start: related.source_start(),
            source_end: related.source_end(),
            row_count: related.intervals().len(),
            matched_rows,
        });
    }

    if tables.len() < 2 {
        return None;
    }

    let related_table_candidate_indexes = tables
        .iter()
        .map(|table| table.table_candidate_index)
        .collect::<Vec<_>>();
    let combined_line_mark_record_indexes = tables
        .iter()
        .flat_map(|table| {
            table
                .matched_rows
                .iter()
                .map(|match_| match_.line_mark_record_index)
        })
        .collect::<Vec<_>>();
    let combined_matched_byte_offsets = tables
        .iter()
        .flat_map(|table| table.matched_rows.iter().map(|match_| match_.byte_offset))
        .collect::<Vec<_>>();
    let combined_raw_record_scan_indexes = tables
        .iter()
        .flat_map(|table| {
            table
                .matched_rows
                .iter()
                .map(|match_| match_.raw_record_scan_index)
        })
        .collect::<Vec<_>>();
    let combined_tail_block16_word_indexes = tables
        .iter()
        .flat_map(|table| {
            table
                .matched_rows
                .iter()
                .map(|match_| match_.tail_block16_word_index)
        })
        .collect::<Vec<_>>();
    let combined_line_start_candidates = tables
        .iter()
        .flat_map(|table| {
            table
                .matched_rows
                .iter()
                .map(|match_| match_.line_start_candidate)
        })
        .collect::<Vec<_>>();
    let combined_line_end_candidates = tables
        .iter()
        .flat_map(|table| {
            table
                .matched_rows
                .iter()
                .map(|match_| match_.line_end_candidate)
        })
        .collect::<Vec<_>>();
    let combined_field2_values = tables
        .iter()
        .flat_map(|table| table.matched_rows.iter().map(|match_| match_.field2_value))
        .collect::<Vec<_>>();
    let monotonic_raw_record_scan_index =
        usize_values_are_monotonic_non_decreasing(&combined_raw_record_scan_indexes);
    let monotonic_line_start_candidate =
        u16_values_are_monotonic_non_decreasing(&combined_line_start_candidates);
    let family_reused_after_later_family =
        values_reused_after_different_value(&combined_matched_byte_offsets);
    let cross_table_ordering_consistent = monotonic_raw_record_scan_index
        && monotonic_line_start_candidate
        && !family_reused_after_later_family;

    Some(TableGridCrossTableSubrecordOrderingProbe {
        current_table_candidate_index: candidate.index(),
        related_table_candidate_indexes,
        combined_line_mark_record_indexes,
        combined_matched_byte_offsets,
        combined_raw_record_scan_indexes,
        combined_tail_block16_word_indexes,
        combined_line_start_candidates,
        combined_line_end_candidates,
        combined_field2_values,
        monotonic_raw_record_scan_index,
        monotonic_line_start_candidate,
        family_reused_after_later_family,
        cross_table_ordering_consistent,
        tables,
    })
}

pub(crate) fn push_table_grid_cross_table_subrecord_ordering_probe_json(
    output: &mut String,
    probe: Option<&TableGridCrossTableSubrecordOrderingProbe>,
) {
    let Some(probe) = probe else {
        output.push_str("null");
        return;
    };

    push_table_grid_cross_table_subrecord_ordering_probe_prefix_json(output, probe);
    output.push_str(",\"tables\":[");
    for (table_index, table) in probe.tables.iter().enumerate() {
        if table_index > 0 {
            output.push(',');
        }
        output.push_str("{\"tableCandidateIndex\":");
        output.push_str(&table.table_candidate_index.to_string());
        output.push_str(",\"sourceRange\":");
        output.push_str(&source_range_json(table.source_start, table.source_end));
        output.push_str(",\"rowCount\":");
        output.push_str(&table.row_count.to_string());
        output.push_str(",\"matchedRowCount\":");
        output.push_str(&table.matched_rows.len().to_string());
        output.push_str(",\"matchedByteOffsets\":");
        push_usize_array_json(
            output,
            &table
                .matched_rows
                .iter()
                .map(|match_| match_.byte_offset)
                .collect::<Vec<_>>(),
        );
        output.push_str(",\"rawRecordScanIndexes\":");
        push_usize_array_json(
            output,
            &table
                .matched_rows
                .iter()
                .map(|match_| match_.raw_record_scan_index)
                .collect::<Vec<_>>(),
        );
        output.push_str(",\"rows\":[");
        for (row_index, match_) in table.matched_rows.iter().enumerate() {
            if row_index > 0 {
                output.push(',');
            }
            push_table_grid_cross_table_subrecord_ordering_match_json(output, match_);
        }
        output.push_str("]}");
    }
    output.push_str(
        "],\"renderPromotionContribution\":\"cross-table-subrecord-ordering-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "cross-table-page-mark-subrecord-ordering-does-not-decode-y-transform",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_cross_table_subrecord_ordering_probe_summary_json(
    output: &mut String,
    probe: Option<&TableGridCrossTableSubrecordOrderingProbe>,
) {
    let Some(probe) = probe else {
        output.push_str("null");
        return;
    };
    push_table_grid_cross_table_subrecord_ordering_probe_prefix_json(output, probe);
    output.push('}');
}

pub(crate) fn push_table_grid_cross_table_subrecord_ordering_probe_prefix_json(
    output: &mut String,
    probe: &TableGridCrossTableSubrecordOrderingProbe,
) {
    output.push_str(
        "{\"source\":\"/PageMark raw u16 subrecord line ranges+cross-table sparse sibling order\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"currentTableCandidateIndex\":");
    output.push_str(&probe.current_table_candidate_index.to_string());
    output.push_str(",\"relatedTableCandidateIndexes\":");
    push_usize_array_json(output, &probe.related_table_candidate_indexes);
    output.push_str(",\"relatedTableCount\":");
    output.push_str(&probe.related_table_candidate_indexes.len().to_string());
    output.push_str(",\"sourceOrderingBasis\":");
    output.push_str(&json_string("tableCandidate.source_start"));
    output.push_str(",\"relatedTableSourceRanges\":[");
    for (index, table) in probe.tables.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&source_range_json(table.source_start, table.source_end));
    }
    output.push(']');
    output.push_str(",\"sourceOrderMatchesProbeOrder\":");
    output.push_str(
        if probe
            .tables
            .windows(2)
            .all(|pair| pair[0].source_start <= pair[1].source_start)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"combinedMatchedRowCount\":");
    output.push_str(&probe.combined_matched_byte_offsets.len().to_string());
    output.push_str(",\"combinedLineMarkRecordIndexes\":");
    push_usize_array_json(output, &probe.combined_line_mark_record_indexes);
    output.push_str(",\"combinedMatchedByteOffsets\":");
    push_usize_array_json(output, &probe.combined_matched_byte_offsets);
    output.push_str(",\"combinedRawRecordScanIndexes\":");
    push_usize_array_json(output, &probe.combined_raw_record_scan_indexes);
    output.push_str(",\"combinedTailBlock16WordIndexes\":");
    push_optional_usize_array_json(output, &probe.combined_tail_block16_word_indexes);
    output.push_str(",\"combinedLineStartCandidates\":");
    push_u16_array_json(output, &probe.combined_line_start_candidates);
    output.push_str(",\"combinedLineEndCandidates\":");
    push_u16_array_json(output, &probe.combined_line_end_candidates);
    output.push_str(",\"combinedField2Values\":");
    push_u16_array_json(output, &probe.combined_field2_values);
    output.push_str(",\"monotonicRawRecordScanIndex\":");
    output.push_str(if probe.monotonic_raw_record_scan_index {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"monotonicLineStartCandidate\":");
    output.push_str(if probe.monotonic_line_start_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"familyReusedAfterLaterFamily\":");
    output.push_str(if probe.family_reused_after_later_family {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOrderingConsistent\":");
    output.push_str(if probe.cross_table_ordering_consistent {
        "true"
    } else {
        "false"
    });
    let mut source_order_contradiction_reasons = Vec::new();
    if !probe.monotonic_raw_record_scan_index {
        source_order_contradiction_reasons
            .push("raw-record-scan-index-regresses-under-source-order");
    }
    if !probe.monotonic_line_start_candidate {
        source_order_contradiction_reasons
            .push("subrecord-line-start-regresses-under-source-order");
    }
    if probe.family_reused_after_later_family {
        source_order_contradiction_reasons
            .push("subrecord-family-reused-after-later-family-under-source-order");
    }
    output.push_str(",\"sourceOrderVsSubrecordOrderContradiction\":");
    output.push_str(if source_order_contradiction_reasons.is_empty() {
        "false"
    } else {
        "true"
    });
    output.push_str(",\"sourceOrderContradictionReasons\":");
    push_json_string_slice_array(output, &source_order_contradiction_reasons);
}

pub(crate) fn push_table_grid_cross_table_subrecord_ordering_match_json(
    output: &mut String,
    match_: &TableGridCrossTableSubrecordOrderingMatch,
) {
    output.push_str("{\"row\":");
    output.push_str(&match_.row_index.to_string());
    output.push_str(",\"lineMarkRecordIndex\":");
    output.push_str(&match_.line_mark_record_index.to_string());
    output.push_str(",\"targetUnits\":");
    output.push_str(&match_.target_units.to_string());
    output.push_str(",\"residualUnits\":");
    output.push_str(&match_.residual_units.to_string());
    output.push_str(",\"byteOffset\":");
    output.push_str(&match_.byte_offset.to_string());
    output.push_str(",\"rawRecordIndex\":");
    output.push_str(&match_.raw_record_index.to_string());
    output.push_str(",\"rawRecordScanIndex\":");
    output.push_str(&match_.raw_record_scan_index.to_string());
    output.push_str(",\"tailBlock16WordIndex\":");
    push_option_usize_json(output, match_.tail_block16_word_index);
    output.push_str(",\"lineStartCandidate\":");
    output.push_str(&match_.line_start_candidate.to_string());
    output.push_str(",\"lineEndCandidate\":");
    output.push_str(&match_.line_end_candidate.to_string());
    output.push_str(",\"field2Value\":");
    output.push_str(&match_.field2_value.to_string());
    output.push('}');
}

pub(crate) fn table_grid_cross_table_row_boundary_offset_probe(
    document: &Document,
    candidate: &TableCandidate,
) -> Option<TableGridCrossTableRowBoundaryOffsetProbe> {
    let layout = page_layout_from_document(document);
    let sparse_table_candidate_index =
        table_grid_sparse_table_sibling_evidence(document, candidate)?
            .sparse_candidate
            .index();
    let mut related_candidates = document
        .table_candidates()
        .iter()
        .filter(|related| related.is_document_text_control_run_candidate())
        .filter(|related| {
            table_grid_sparse_table_sibling_evidence(document, related).is_some_and(|evidence| {
                evidence.sparse_candidate.index() == sparse_table_candidate_index
            })
        })
        .collect::<Vec<_>>();
    related_candidates.sort_by(|left, right| {
        left.source_start()
            .cmp(&right.source_start())
            .then_with(|| left.index().cmp(&right.index()))
    });
    if related_candidates.len() < 2 {
        return None;
    }

    let related_table_count = related_candidates.len();
    let related_table_candidate_indexes = related_candidates
        .iter()
        .map(|related| related.index())
        .collect::<Vec<_>>();
    let mut tables = Vec::new();
    for related in related_candidates {
        let Some(summary) =
            table_grid_line_mark_row_boundary_alignment_summary(document, related, None)
        else {
            continue;
        };
        let selected = summary.selected_spacing_record_alignment.clone();
        let Some(previous) = summary.previous_row_span_record_alignment else {
            continue;
        };
        let row_boundary_offset_candidate_units = previous.row_boundary_offset_candidate_units;
        let offset_normalized_start_residual_units = row_boundary_offset_candidate_units
            .map(|offset| {
                previous
                    .start_residual_units
                    .iter()
                    .map(|residual| residual - offset)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let offset_normalized_end_residual_units = row_boundary_offset_candidate_units
            .map(|offset| {
                previous
                    .end_residual_units
                    .iter()
                    .map(|residual| residual - offset)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let offset_normalized_exact_boundary_aligned = !offset_normalized_start_residual_units
            .is_empty()
            && offset_normalized_start_residual_units
                .iter()
                .all(|residual| *residual == 0)
            && offset_normalized_end_residual_units
                .iter()
                .all(|residual| *residual == 0);
        let record_indexes = previous.record_indexes;
        let page_mark_context =
            table_grid_page_mark_context_for_line_mark_record_indexes(document, &record_indexes);
        let page_mark_line_offsets_from_entry_start = page_mark_context
            .as_ref()
            .map(|context| {
                record_indexes
                    .iter()
                    .map(|record_index| record_index.saturating_sub(context.page_line_start))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let page_mark_records_within_single_entry = page_mark_context.is_some();
        let line_mark_record_y_tops_px = page_mark_context
            .as_ref()
            .and_then(|context| {
                table_grid_page_mark_line_pitch_candidate(
                    layout,
                    context.page_line_start,
                    context.page_line_end,
                )
                .map(|(pitch, _)| {
                    line_mark_record_indexes_y_tops(
                        layout,
                        &record_indexes,
                        context.page_line_start,
                        pitch,
                    )
                })
            })
            .unwrap_or_default();
        let selected_spacing_record_indexes = selected
            .as_ref()
            .map(|family| family.record_indexes.clone())
            .unwrap_or_default();
        let selected_spacing_page_mark_context =
            table_grid_page_mark_context_for_line_mark_record_indexes(
                document,
                &selected_spacing_record_indexes,
            );
        let selected_spacing_page_mark_line_offsets_from_entry_start =
            selected_spacing_page_mark_context
                .as_ref()
                .map(|context| {
                    selected_spacing_record_indexes
                        .iter()
                        .map(|record_index| record_index.saturating_sub(context.page_line_start))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
        let selected_spacing_records_within_single_entry =
            selected_spacing_page_mark_context.is_some();
        let selected_spacing_record_y_tops_px = selected_spacing_page_mark_context
            .as_ref()
            .and_then(|context| {
                table_grid_page_mark_line_pitch_candidate(
                    layout,
                    context.page_line_start,
                    context.page_line_end,
                )
                .map(|(pitch, _)| {
                    line_mark_record_indexes_y_tops(
                        layout,
                        &selected_spacing_record_indexes,
                        context.page_line_start,
                        pitch,
                    )
                })
            })
            .unwrap_or_default();
        let selected_spacing_line_mark_start_units = selected
            .as_ref()
            .map(|family| family.line_mark_start_units.clone())
            .unwrap_or_default();
        let selected_spacing_line_mark_end_units = selected
            .as_ref()
            .map(|family| family.line_mark_end_units.clone())
            .unwrap_or_default();
        let selected_spacing_start_residual_units = selected
            .as_ref()
            .map(|family| family.start_residual_units.clone())
            .unwrap_or_default();
        let selected_spacing_end_residual_units = selected
            .as_ref()
            .map(|family| family.end_residual_units.clone())
            .unwrap_or_default();
        let selected_spacing_span_residual_units = selected
            .as_ref()
            .map(|family| family.span_residual_units.clone())
            .unwrap_or_default();
        let selected_minus_previous_record_index_gaps = selected_spacing_record_indexes
            .iter()
            .copied()
            .zip(record_indexes.iter().copied())
            .map(|(selected, previous)| signed_usize_delta_i32(selected, previous))
            .collect::<Vec<_>>();
        let selected_minus_previous_record_y_delta_px = selected_spacing_record_y_tops_px
            .iter()
            .copied()
            .zip(line_mark_record_y_tops_px.iter().copied())
            .map(|(selected, previous)| selected - previous)
            .collect::<Vec<_>>();
        tables.push(TableGridCrossTableRowBoundaryOffsetTable {
            table_candidate_index: related.index(),
            source_start: related.source_start(),
            source_end: related.source_end(),
            row_count: related.intervals().len(),
            line_mark_record_indexes: record_indexes,
            page_mark_line_offsets_from_entry_start,
            page_mark_records_within_single_entry,
            line_mark_record_y_tops_px,
            selected_spacing_record_indexes,
            selected_spacing_page_mark_line_offsets_from_entry_start,
            selected_spacing_records_within_single_entry,
            selected_spacing_record_y_tops_px,
            selected_spacing_line_mark_start_units,
            selected_spacing_line_mark_end_units,
            selected_spacing_start_residual_units,
            selected_spacing_end_residual_units,
            selected_spacing_span_residual_units,
            selected_minus_previous_record_index_gaps,
            selected_minus_previous_record_y_delta_px,
            row_source_start_units: previous.row_source_start_units,
            row_source_end_units: previous.row_source_end_units,
            line_mark_start_units: previous.line_mark_start_units,
            line_mark_end_units: previous.line_mark_end_units,
            start_residual_units: previous.start_residual_units,
            end_residual_units: previous.end_residual_units,
            span_residual_units: previous.span_residual_units,
            row_boundary_offset_candidate_units,
            offset_normalized_start_residual_units,
            offset_normalized_end_residual_units,
            offset_normalized_exact_boundary_aligned,
            exact_boundary_aligned: previous.exact_boundary_aligned,
            span_only_match: previous.span_only_match,
        });
    }
    if tables.len() < 2 {
        return None;
    }

    let row_boundary_offset_candidate_units = tables
        .iter()
        .filter_map(|table| table.row_boundary_offset_candidate_units)
        .collect::<Vec<_>>();
    let all_related_tables_have_offset_candidate = tables.len() == related_table_count
        && row_boundary_offset_candidate_units.len() == tables.len();
    let stable_row_boundary_offset_candidate_units = all_related_tables_have_offset_candidate
        .then(|| single_i32_value(&row_boundary_offset_candidate_units))
        .flatten();
    let all_offsets_stable = stable_row_boundary_offset_candidate_units.is_some();
    let all_offsets_require_transform = all_offsets_stable
        && tables
            .iter()
            .all(|table| !table.exact_boundary_aligned && table.span_only_match);
    let all_offset_normalized_boundaries_exact = all_offsets_stable
        && tables
            .iter()
            .all(|table| table.offset_normalized_exact_boundary_aligned);
    let combined_line_mark_record_indexes = tables
        .iter()
        .flat_map(|table| table.line_mark_record_indexes.iter().copied())
        .collect::<Vec<_>>();
    let combined_page_mark_context = table_grid_page_mark_context_for_line_mark_record_indexes(
        document,
        &combined_line_mark_record_indexes,
    );
    let page_mark_u16_field_count = combined_page_mark_context
        .as_ref()
        .map(|context| context.page_mark_u16_fields.len())
        .unwrap_or_default();
    let page_mark_u16_field_preview = combined_page_mark_context
        .as_ref()
        .map(|context| {
            context
                .page_mark_u16_fields
                .iter()
                .copied()
                .take(24)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let combined_line_offsets_from_page_start = combined_page_mark_context
        .as_ref()
        .map(|context| {
            combined_line_mark_record_indexes
                .iter()
                .map(|record_index| record_index.saturating_sub(context.page_line_start))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let combined_line_offsets_monotonic = !combined_line_offsets_from_page_start.is_empty()
        && usize_values_are_monotonic_non_decreasing(&combined_line_offsets_from_page_start);
    let combined_line_mark_record_y_pitch =
        combined_page_mark_context.as_ref().and_then(|context| {
            table_grid_page_mark_line_pitch_candidate(
                layout,
                context.page_line_start,
                context.page_line_end,
            )
        });
    let combined_line_mark_record_y_tops_px = combined_page_mark_context
        .as_ref()
        .zip(combined_line_mark_record_y_pitch.as_ref())
        .map(|(context, (pitch, _))| {
            line_mark_record_indexes_y_tops(
                layout,
                &combined_line_mark_record_indexes,
                context.page_line_start,
                *pitch,
            )
        })
        .unwrap_or_default();
    let combined_line_mark_record_y_span_px = combined_line_mark_record_y_tops_px
        .first()
        .copied()
        .zip(combined_line_mark_record_y_tops_px.last().copied())
        .map(|(first, last)| last - first);
    let source_unit_to_page_line_index_source_units = tables
        .iter()
        .flat_map(|table| table.row_source_start_units.iter().copied())
        .collect::<Vec<_>>();
    let source_unit_fit_xs = source_unit_to_page_line_index_source_units
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();
    let source_unit_fit_ys = combined_line_mark_record_indexes
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();
    let source_unit_to_page_line_index_fit =
        affine_fit_f32(&source_unit_fit_xs, &source_unit_fit_ys);
    let source_unit_to_page_line_index_fitted_indexes = source_unit_to_page_line_index_fit
        .map(|(slope, intercept)| {
            source_unit_fit_xs
                .iter()
                .map(|source_unit| intercept + slope * source_unit)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let source_unit_to_page_line_index_residual_indexes =
        source_unit_to_page_line_index_fitted_indexes
            .iter()
            .zip(&source_unit_fit_ys)
            .map(|(fit, actual)| actual - fit)
            .collect::<Vec<_>>();
    let source_unit_to_page_line_index_max_abs_residual =
        max_abs_f32(&source_unit_to_page_line_index_residual_indexes);
    let source_unit_to_page_line_index_exact =
        source_unit_to_page_line_index_max_abs_residual.is_some_and(|residual| residual <= 0.001);
    let mut source_unit_to_page_line_index_rows = Vec::new();
    let mut combined_row_offset = 0usize;
    for table in &tables {
        for (row_index, (source_start_units, line_mark_record_index)) in table
            .row_source_start_units
            .iter()
            .copied()
            .zip(table.line_mark_record_indexes.iter().copied())
            .enumerate()
        {
            let combined_index = combined_row_offset + row_index;
            if let (Some(fitted_record_index), Some(residual_record_index)) = (
                source_unit_to_page_line_index_fitted_indexes
                    .get(combined_index)
                    .copied(),
                source_unit_to_page_line_index_residual_indexes
                    .get(combined_index)
                    .copied(),
            ) {
                source_unit_to_page_line_index_rows.push(
                    TableGridSourceUnitToPageLineIndexFitRow {
                        table_candidate_index: table.table_candidate_index,
                        row_index,
                        row_source_start_units: source_start_units,
                        line_mark_record_index,
                        fitted_record_index,
                        residual_record_index,
                    },
                );
            }
        }
        combined_row_offset += table.line_mark_record_indexes.len();
    }
    let all_records_within_single_page_mark_entry = combined_page_mark_context.is_some()
        && tables
            .iter()
            .all(|table| table.page_mark_records_within_single_entry);
    let source_unit_to_page_line_index_piecewise_tables = tables
        .iter()
        .map(table_grid_source_unit_to_page_line_index_piecewise_table)
        .collect::<Vec<_>>();
    let source_unit_to_page_line_index_piecewise_max_values =
        source_unit_to_page_line_index_piecewise_tables
            .iter()
            .filter_map(|table| table.max_abs_residual_record_indexes)
            .collect::<Vec<_>>();
    let source_unit_to_page_line_index_piecewise_max_abs_residual =
        max_abs_f32(&source_unit_to_page_line_index_piecewise_max_values);
    let source_unit_to_page_line_index_piecewise_all_tables_exact =
        !source_unit_to_page_line_index_piecewise_tables.is_empty()
            && source_unit_to_page_line_index_piecewise_tables
                .iter()
                .all(|table| table.exact_fit);
    let source_unit_to_page_line_index_piecewise_transitions = tables
        .windows(2)
        .filter_map(|pair| {
            table_grid_source_unit_to_page_line_index_piecewise_transition(
                &pair[0],
                &pair[1],
                all_records_within_single_page_mark_entry,
            )
        })
        .collect::<Vec<_>>();

    Some(TableGridCrossTableRowBoundaryOffsetProbe {
        current_table_candidate_index: candidate.index(),
        sparse_table_candidate_index,
        related_table_candidate_indexes,
        related_table_count,
        table_count_with_previous_row_span_alignment: tables.len(),
        row_boundary_offset_candidate_units,
        stable_row_boundary_offset_candidate_units,
        all_related_tables_have_offset_candidate,
        all_offsets_stable,
        all_offsets_require_transform,
        all_offset_normalized_boundaries_exact,
        combined_line_mark_record_indexes,
        page_mark_entry_index: combined_page_mark_context
            .as_ref()
            .map(|context| context.page_mark_entry_index),
        page_index_candidate: combined_page_mark_context
            .as_ref()
            .and_then(|context| context.page_index_candidate),
        page_line_start: combined_page_mark_context
            .as_ref()
            .map(|context| context.page_line_start),
        page_line_end: combined_page_mark_context
            .as_ref()
            .map(|context| context.page_line_end),
        page_mark_u16_field_count,
        page_mark_u16_field_preview,
        combined_line_offsets_from_page_start,
        combined_line_offsets_monotonic,
        combined_line_mark_record_y_pitch_px: combined_line_mark_record_y_pitch
            .map(|(pitch, _)| pitch),
        combined_line_mark_record_y_pitch_basis: combined_line_mark_record_y_pitch
            .map(|(_, basis)| basis),
        combined_line_mark_record_y_tops_px,
        combined_line_mark_record_y_span_px,
        source_unit_to_page_line_index_source_units,
        source_unit_to_page_line_index_slope: source_unit_to_page_line_index_fit
            .map(|(slope, _)| slope),
        source_unit_to_page_line_index_intercept: source_unit_to_page_line_index_fit
            .map(|(_, intercept)| intercept),
        source_unit_to_page_line_index_fitted_indexes,
        source_unit_to_page_line_index_residual_indexes,
        source_unit_to_page_line_index_max_abs_residual,
        source_unit_to_page_line_index_exact,
        source_unit_to_page_line_index_rows,
        source_unit_to_page_line_index_piecewise_max_abs_residual,
        source_unit_to_page_line_index_piecewise_all_tables_exact,
        source_unit_to_page_line_index_piecewise_tables,
        source_unit_to_page_line_index_piecewise_transitions,
        all_records_within_single_page_mark_entry,
        tables,
    })
}

pub(crate) fn table_grid_source_unit_to_page_line_index_piecewise_table(
    table: &TableGridCrossTableRowBoundaryOffsetTable,
) -> TableGridSourceUnitToPageLineIndexPiecewiseTable {
    let fit_xs = table
        .row_source_start_units
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();
    let fit_ys = table
        .line_mark_record_indexes
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();
    let fit = affine_fit_f32(&fit_xs, &fit_ys);
    let fitted_record_indexes = fit
        .map(|(slope, intercept)| {
            fit_xs
                .iter()
                .map(|source_unit| intercept + slope * source_unit)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let residual_record_indexes = fitted_record_indexes
        .iter()
        .zip(&fit_ys)
        .map(|(fit, actual)| actual - fit)
        .collect::<Vec<_>>();
    let max_abs_residual_record_indexes = max_abs_f32(&residual_record_indexes);
    let exact_fit = max_abs_residual_record_indexes.is_some_and(|residual| residual <= 0.001);

    TableGridSourceUnitToPageLineIndexPiecewiseTable {
        table_candidate_index: table.table_candidate_index,
        source_start: table.source_start,
        source_end: table.source_end,
        row_count: table.row_count,
        row_source_start_units: table.row_source_start_units.clone(),
        line_mark_record_indexes: table.line_mark_record_indexes.clone(),
        slope_record_indexes_per_source_unit: fit.map(|(slope, _)| slope),
        intercept_record_index: fit.map(|(_, intercept)| intercept),
        fitted_record_indexes,
        residual_record_indexes,
        max_abs_residual_record_indexes,
        exact_fit,
        page_mark_records_within_single_entry: table.page_mark_records_within_single_entry,
    }
}

pub(crate) fn table_grid_source_unit_to_page_line_index_piecewise_transition(
    previous: &TableGridCrossTableRowBoundaryOffsetTable,
    next: &TableGridCrossTableRowBoundaryOffsetTable,
    same_page_mark_entry: bool,
) -> Option<TableGridSourceUnitToPageLineIndexPiecewiseTransition> {
    let previous_last_source_unit = previous.row_source_start_units.last().copied()?;
    let next_first_source_unit = next.row_source_start_units.first().copied()?;
    let previous_last_record_index = previous.line_mark_record_indexes.last().copied()?;
    let next_first_record_index = next.line_mark_record_indexes.first().copied()?;

    Some(TableGridSourceUnitToPageLineIndexPiecewiseTransition {
        from_table_candidate_index: previous.table_candidate_index,
        to_table_candidate_index: next.table_candidate_index,
        previous_last_source_unit,
        next_first_source_unit,
        source_range_gap_units: next.source_start.saturating_sub(previous.source_end),
        row_source_start_gap_units: signed_usize_delta_i32(
            next_first_source_unit,
            previous_last_source_unit,
        ),
        previous_last_record_index,
        next_first_record_index,
        line_mark_record_gap: signed_usize_delta_i32(
            next_first_record_index,
            previous_last_record_index,
        ),
        same_page_mark_entry,
    })
}

pub(crate) fn push_table_grid_cross_table_row_boundary_offset_probe_summary_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
) {
    let Some(probe) = probe else {
        output.push_str("null");
        return;
    };

    output.push_str(
        "{\"source\":\"/LineMark previous row-span boundaries+cross-table sparse sibling order\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"currentTableCandidateIndex\":");
    output.push_str(&probe.current_table_candidate_index.to_string());
    output.push_str(",\"sparseTableCandidateIndex\":");
    output.push_str(&probe.sparse_table_candidate_index.to_string());
    output.push_str(",\"relatedTableCandidateIndexes\":");
    push_usize_array_json(output, &probe.related_table_candidate_indexes);
    output.push_str(",\"relatedTableCount\":");
    output.push_str(&probe.related_table_count.to_string());
    output.push_str(",\"tableCountWithPreviousRowSpanAlignment\":");
    output.push_str(
        &probe
            .table_count_with_previous_row_span_alignment
            .to_string(),
    );
    output.push_str(",\"rowBoundaryOffsetCandidateUnits\":");
    push_i32_array_json(output, &probe.row_boundary_offset_candidate_units);
    output.push_str(",\"stableRowBoundaryOffsetCandidateUnits\":");
    push_optional_i32_json(output, probe.stable_row_boundary_offset_candidate_units);
    output.push_str(",\"allRelatedTablesHaveOffsetCandidate\":");
    output.push_str(if probe.all_related_tables_have_offset_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allOffsetsStable\":");
    output.push_str(if probe.all_offsets_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allOffsetsRequireTransform\":");
    output.push_str(if probe.all_offsets_require_transform {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"offsetNormalizationPolicy\":");
    output.push_str(&json_string(
        "row-source-boundaries-plus-stable-offset-must-equal-previous-line-mark-boundaries",
    ));
    output.push_str(",\"allOffsetNormalizedBoundariesExact\":");
    output.push_str(if probe.all_offset_normalized_boundaries_exact {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageMarkLineDomainPolicy\":");
    output.push_str(&json_string(
        "previous-row-span-records-must-share-one-page-mark-entry-and-monotonic-line-offsets",
    ));
    output.push_str(",\"combinedLineMarkRecordIndexes\":");
    push_usize_array_json(output, &probe.combined_line_mark_record_indexes);
    output.push_str(",\"pageMarkEntryIndex\":");
    push_option_usize_json(output, probe.page_mark_entry_index);
    output.push_str(",\"pageIndexCandidate\":");
    push_option_usize_json(output, probe.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    push_option_usize_json(output, probe.page_line_start);
    output.push_str(",\"pageLineEnd\":");
    push_option_usize_json(output, probe.page_line_end);
    output.push_str(",\"pageMarkU16FieldCount\":");
    output.push_str(&probe.page_mark_u16_field_count.to_string());
    output.push_str(",\"pageMarkU16FieldPreview\":");
    push_u16_array_json(output, &probe.page_mark_u16_field_preview);
    output.push_str(",\"pageMarkU16FieldPreviewHex\":");
    push_u16_hex_array_json(output, &probe.page_mark_u16_field_preview);
    output.push_str(",\"combinedLineOffsetsFromPageStart\":");
    push_usize_array_json(output, &probe.combined_line_offsets_from_page_start);
    output.push_str(",\"combinedLineOffsetsMonotonic\":");
    output.push_str(if probe.combined_line_offsets_monotonic {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"combinedLineMarkRecordYProjection\":{\"source\":\"/PageMark line range+page layout body line gap\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"combinedLineMarkRecordYPitchPx\":");
    push_optional_f32_json(output, probe.combined_line_mark_record_y_pitch_px);
    output.push_str(",\"combinedLineMarkRecordYPitchBasis\":");
    match probe.combined_line_mark_record_y_pitch_basis {
        Some(basis) => output.push_str(&json_string(basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"combinedLineMarkRecordYTopPx\":");
    push_f32_array_json(output, &probe.combined_line_mark_record_y_tops_px);
    output.push_str(",\"combinedLineMarkRecordYSpanPx\":");
    push_optional_f32_json(output, probe.combined_line_mark_record_y_span_px);
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-line-gap-projection-does-not-decode-table-y-origin",
    ));
    output.push('}');
    output.push_str(",\"sourceUnitToPageLineIndexFit\":{\"source\":\"/DocumentText row source units+/LineMark previous-row-span records\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"fitBasis\":\"rowSourceStartUnits-to-lineMarkRecordIndexes\"");
    output.push_str(",\"rowSourceStartUnits\":");
    push_usize_array_json(output, &probe.source_unit_to_page_line_index_source_units);
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &probe.combined_line_mark_record_indexes);
    output.push_str(",\"slopeRecordIndexesPerSourceUnit\":");
    push_optional_f32_json(output, probe.source_unit_to_page_line_index_slope);
    output.push_str(",\"interceptRecordIndex\":");
    push_optional_f32_json(output, probe.source_unit_to_page_line_index_intercept);
    output.push_str(",\"fittedRecordIndexes\":");
    push_f32_array_json(output, &probe.source_unit_to_page_line_index_fitted_indexes);
    output.push_str(",\"residualRecordIndexes\":");
    push_f32_array_json(
        output,
        &probe.source_unit_to_page_line_index_residual_indexes,
    );
    output.push_str(",\"maxAbsResidualRecordIndexes\":");
    push_optional_f32_json(
        output,
        probe.source_unit_to_page_line_index_max_abs_residual,
    );
    output.push_str(",\"exactFit\":");
    output.push_str(if probe.source_unit_to_page_line_index_exact {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rows\":[");
    for (index, row) in probe.source_unit_to_page_line_index_rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_source_unit_to_page_line_index_fit_row_json(output, row);
    }
    output.push(']');
    output.push_str(",\"renderPromotionBlockedReason\":");
    if probe.source_unit_to_page_line_index_exact {
        output.push_str(&json_string(
            "source-unit-to-page-line-fit-still-needs-page-y-scale-and-origin",
        ));
    } else {
        output.push_str(&json_string(
            "source-unit-to-page-line-affine-fit-not-exact",
        ));
    }
    output.push('}');
    output.push_str(",\"sourceUnitToPageLineIndexPiecewiseFit\":");
    push_table_grid_source_unit_to_page_line_index_piecewise_fit_json(output, probe);
    output.push_str(",\"piecewiseRecordFamilyGapYDiagnostic\":");
    push_table_grid_piecewise_record_family_gap_y_diagnostic_json(output, probe);
    output.push_str(",\"sourceOnlyPageMarkSlotScopedSubrecordYSequenceProbe\":");
    push_table_grid_source_only_page_mark_slot_scoped_subrecord_y_sequence_probe_json(
        output, layout, document, probe,
    );
    output.push_str(",\"allRecordsWithinSinglePageMarkEntry\":");
    output.push_str(if probe.all_records_within_single_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"tables\":[");
    for (index, table) in probe.tables.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_cross_table_row_boundary_offset_table_json(output, table);
    }
    output.push_str(
        "],\"renderPromotionContribution\":\"cross-table-row-boundary-offset-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if probe.all_offsets_stable {
        output.push_str(&json_string(
            "row-boundary-offset-transform-does-not-decode-page-y-origin",
        ));
    } else {
        output.push_str(&json_string("row-boundary-offset-not-cross-table-stable"));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_cross_table_row_boundary_offset_table_json(
    output: &mut String,
    table: &TableGridCrossTableRowBoundaryOffsetTable,
) {
    output.push_str("{\"tableCandidateIndex\":");
    output.push_str(&table.table_candidate_index.to_string());
    output.push_str(",\"sourceRange\":");
    output.push_str(&source_range_json(table.source_start, table.source_end));
    output.push_str(",\"rowCount\":");
    output.push_str(&table.row_count.to_string());
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &table.line_mark_record_indexes);
    output.push_str(",\"pageMarkLineOffsetsFromEntryStart\":");
    push_usize_array_json(output, &table.page_mark_line_offsets_from_entry_start);
    output.push_str(",\"pageMarkRecordsWithinSingleEntry\":");
    output.push_str(if table.page_mark_records_within_single_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordYTopPx\":");
    push_f32_array_json(output, &table.line_mark_record_y_tops_px);
    output.push_str(",\"selectedSpacingRecordIndexes\":");
    push_usize_array_json(output, &table.selected_spacing_record_indexes);
    output.push_str(",\"selectedSpacingPageMarkLineOffsetsFromEntryStart\":");
    push_usize_array_json(
        output,
        &table.selected_spacing_page_mark_line_offsets_from_entry_start,
    );
    output.push_str(",\"selectedSpacingRecordsWithinSingleEntry\":");
    output.push_str(if table.selected_spacing_records_within_single_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedSpacingRecordYTopPx\":");
    push_f32_array_json(output, &table.selected_spacing_record_y_tops_px);
    output.push_str(",\"selectedSpacingLineMarkStartUnits\":");
    push_usize_array_json(output, &table.selected_spacing_line_mark_start_units);
    output.push_str(",\"selectedSpacingLineMarkEndUnits\":");
    push_usize_array_json(output, &table.selected_spacing_line_mark_end_units);
    output.push_str(",\"selectedSpacingStartResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_start_residual_units);
    output.push_str(",\"selectedSpacingEndResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_end_residual_units);
    output.push_str(",\"selectedSpacingSpanResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_span_residual_units);
    output.push_str(",\"selectedMinusPreviousRecordIndexGaps\":");
    push_i32_array_json(output, &table.selected_minus_previous_record_index_gaps);
    output.push_str(",\"selectedMinusPreviousRecordYDeltaPx\":");
    push_f32_array_json(output, &table.selected_minus_previous_record_y_delta_px);
    output.push_str(",\"rowSourceStartUnits\":");
    push_usize_array_json(output, &table.row_source_start_units);
    output.push_str(",\"rowSourceEndUnits\":");
    push_usize_array_json(output, &table.row_source_end_units);
    output.push_str(",\"lineMarkStartUnits\":");
    push_usize_array_json(output, &table.line_mark_start_units);
    output.push_str(",\"lineMarkEndUnits\":");
    push_usize_array_json(output, &table.line_mark_end_units);
    output.push_str(",\"startResidualUnits\":");
    push_i32_array_json(output, &table.start_residual_units);
    output.push_str(",\"endResidualUnits\":");
    push_i32_array_json(output, &table.end_residual_units);
    output.push_str(",\"spanResidualUnits\":");
    push_i32_array_json(output, &table.span_residual_units);
    output.push_str(",\"rowBoundaryOffsetCandidateUnits\":");
    push_optional_i32_json(output, table.row_boundary_offset_candidate_units);
    output.push_str(",\"offsetNormalizedStartResidualUnits\":");
    push_i32_array_json(output, &table.offset_normalized_start_residual_units);
    output.push_str(",\"offsetNormalizedEndResidualUnits\":");
    push_i32_array_json(output, &table.offset_normalized_end_residual_units);
    output.push_str(",\"offsetNormalizedExactBoundaryAligned\":");
    output.push_str(if table.offset_normalized_exact_boundary_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"exactBoundaryAligned\":");
    output.push_str(if table.exact_boundary_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"spanOnlyMatch\":");
    output.push_str(if table.span_only_match {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_table_grid_piecewise_record_family_gap_y_diagnostic_json(
    output: &mut String,
    probe: &TableGridCrossTableRowBoundaryOffsetProbe,
) {
    let selected_previous_gaps = probe
        .tables
        .iter()
        .flat_map(|table| {
            table
                .selected_minus_previous_record_index_gaps
                .iter()
                .copied()
        })
        .collect::<Vec<_>>();
    let stable_selected_previous_gap = single_i32_value(&selected_previous_gaps);
    let selected_previous_y_delta_milli = probe
        .tables
        .iter()
        .flat_map(|table| {
            table
                .selected_minus_previous_record_y_delta_px
                .iter()
                .map(|value| rounded_milli(*value))
        })
        .collect::<Vec<_>>();
    let stable_selected_previous_y_delta_px =
        single_i32_value(&selected_previous_y_delta_milli).map(|value| value as f32 / 1000.0);

    output.push_str("{\"source\":\"/DocumentText row source units+/LineMark families (selected-spacing vs previous-row-span)+piecewise transitions\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"relatedTableCandidateIndexes\":");
    push_usize_array_json(output, &probe.related_table_candidate_indexes);
    output.push_str(",\"tableCount\":");
    output.push_str(&probe.tables.len().to_string());
    output.push_str(",\"recordFamilyInterpretation\":");
    output.push_str(&json_string(
        "selected-records-match-post-row-gaps-previous-records-match-row-spans",
    ));
    output.push_str(",\"stableSelectedMinusPreviousRecordIndexGap\":");
    push_optional_i32_json(output, stable_selected_previous_gap);
    output.push_str(",\"allSelectedRecordsOneAfterPrevious\":");
    output.push_str(
        if stable_selected_previous_gap == Some(1) && !selected_previous_gaps.is_empty() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"stableSelectedMinusPreviousRecordYDeltaPx\":");
    push_optional_f32_json(output, stable_selected_previous_y_delta_px);
    output.push_str(",\"allRecordFamiliesWithinSinglePageMarkEntry\":");
    output.push_str(
        if probe.all_records_within_single_page_mark_entry
            && probe
                .tables
                .iter()
                .all(|table| table.selected_spacing_records_within_single_entry)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"tables\":[");
    for (index, table) in probe.tables.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_piecewise_record_family_gap_table_json(output, table);
    }
    output.push_str("],\"transitions\":[");
    for (index, pair) in probe.tables.windows(2).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_piecewise_record_family_gap_transition_json(output, &pair[0], &pair[1]);
    }
    output.push(']');
    output.push_str(",\"renderPromotionContribution\":");
    output.push_str(&json_string(
        "source-unit-to-page-line-family-gap-piecewise-diagnostic-only",
    ));
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "piecewise-family-gap-y-comparison-blocks-page-y-origin",
    ));
    output.push('}');
}

pub(crate) fn push_table_grid_source_only_page_mark_slot_scoped_subrecord_y_sequence_probe_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    probe: &TableGridCrossTableRowBoundaryOffsetProbe,
) {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        output.push_str("null");
        return;
    };
    let Some(page_mark) = document.page_marks().first() else {
        output.push_str("null");
        return;
    };
    if probe.combined_line_mark_record_indexes.is_empty()
        || probe.combined_line_mark_record_y_tops_px.len()
            != probe.combined_line_mark_record_indexes.len()
    {
        output.push_str("null");
        return;
    }

    let record_headers = page_mark_record_headers(page_mark_bytes);
    let raw_header_indexes = page_mark_raw_header_indexes_for_line_mark_record_indexes(
        &record_headers,
        &probe.combined_line_mark_record_indexes,
    );
    let single_raw_header_index = single_usize_value(&raw_header_indexes);
    let row_delta_targets = adjacent_f32_deltas(&probe.combined_line_mark_record_y_tops_px);
    const TOLERANCE_PX: f32 = 2.0;

    let mut members = Vec::new();
    collect_page_mark_scoped_y_family_members(
        &mut members,
        page_mark,
        probe.page_mark_entry_index,
        page_mark_bytes,
        &record_headers,
    );
    let same_header_members = single_raw_header_index
        .map(|single_raw_header_index| {
            members
                .iter()
                .filter(|member| member.raw_record_scan_index == Some(single_raw_header_index))
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let foreign_header_members = single_raw_header_index
        .map(|single_raw_header_index| {
            members
                .iter()
                .filter(|member| {
                    member
                        .raw_record_scan_index
                        .is_some_and(|scan_index| scan_index != single_raw_header_index)
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut slots = page_mark_scoped_y_slot_fits(
        members,
        &probe.combined_line_mark_record_indexes,
        &probe.combined_line_mark_record_y_tops_px,
        &row_delta_targets,
        TOLERANCE_PX,
    );
    slots.sort_by(page_mark_scoped_y_slot_fit_ordering);
    let mut same_header_slots = page_mark_scoped_y_slot_fits(
        same_header_members,
        &probe.combined_line_mark_record_indexes,
        &probe.combined_line_mark_record_y_tops_px,
        &row_delta_targets,
        TOLERANCE_PX,
    );
    same_header_slots.sort_by(page_mark_scoped_y_slot_fit_ordering);
    let mut foreign_header_slots = page_mark_scoped_y_slot_fits(
        foreign_header_members,
        &probe.combined_line_mark_record_indexes,
        &probe.combined_line_mark_record_y_tops_px,
        &row_delta_targets,
        TOLERANCE_PX,
    );
    foreign_header_slots.sort_by(page_mark_scoped_y_slot_fit_ordering);

    output.push_str(
        "{\"source\":\"/PageMark raw u16 subrecord scan+/LineMark source page-line projection\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"grouping\":\"fieldIndex+tailBlock16WordIndex\"");
    output.push_str(",\"sourceYTargetBasis\":");
    output.push_str(&json_string(
        "page-mark-line-range-plus-page-layout-body-line-gap",
    ));
    output.push_str(",\"tolerancePx\":");
    output.push_str(&format!("{TOLERANCE_PX:.3}"));
    output.push_str(",\"pageMarkEntryIndex\":");
    push_option_usize_json(output, probe.page_mark_entry_index);
    output.push_str(",\"pageIndexCandidate\":");
    push_option_usize_json(output, probe.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    push_option_usize_json(output, probe.page_line_start);
    output.push_str(",\"pageLineEnd\":");
    push_option_usize_json(output, probe.page_line_end);
    output.push_str(",\"rawHeaderMatchCount\":");
    output.push_str(&raw_header_indexes.len().to_string());
    output.push_str(",\"singleRawRecordHeaderMatched\":");
    output.push_str(if single_raw_header_index.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedRawRecordHeaderIndex\":");
    push_option_usize_json(output, single_raw_header_index);
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &probe.combined_line_mark_record_indexes);
    output.push_str(",\"sourceLineMarkRecordYTopPx\":");
    push_f32_array_json(output, &probe.combined_line_mark_record_y_tops_px);
    output.push_str(",\"sourceLineMarkRecordYDeltasPx\":");
    push_f32_array_json(output, &row_delta_targets);
    output.push_str(
        ",\"subrecordLineRangeCandidatePolicy\":\"words4To6WithinScannedRecordHeaderMaxLineEnd\"",
    );
    output.push_str(",\"subrecordLineRangeMaxCandidate\":");
    push_option_u32_json(
        output,
        page_mark_subrecord_line_range_max_candidate(page_mark, &record_headers),
    );
    output.push_str(",\"pageScaleCandidates\":");
    push_page_mark_slot_scoped_page_scale_candidates_json(
        output,
        layout,
        probe
            .page_mark_entry_index
            .and_then(|index| page_mark.entries().get(index)),
    );
    output.push_str(",\"slotCount\":");
    output.push_str(&slots.len().to_string());
    output.push_str(",\"sameHeaderSlotCount\":");
    output.push_str(&same_header_slots.len().to_string());
    output.push_str(",\"sameHeaderBestSourceTableTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &same_header_slots, |slot| {
        slot.table_top_hit_count > 0
    });
    output.push_str(",\"sameHeaderBestSourceRowTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &same_header_slots, |slot| {
        slot.row_top_coverage_count > 0
    });
    output.push_str(",\"sameHeaderBestSourceRowDeltaSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &same_header_slots, |slot| {
        slot.row_delta_coverage_count > 0
    });
    output.push_str(",\"foreignHeaderSlotCount\":");
    output.push_str(&foreign_header_slots.len().to_string());
    output.push_str(",\"foreignHeaderBestSourceTableTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &foreign_header_slots, |slot| {
        slot.table_top_hit_count > 0
    });
    output.push_str(",\"bestSourceTableTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &slots, |slot| slot.table_top_hit_count > 0);
    output.push_str(",\"bestSourceRowTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &slots, |slot| {
        slot.row_top_coverage_count > 0
    });
    output.push_str(",\"bestSourceRowDeltaSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &slots, |slot| {
        slot.row_delta_coverage_count > 0
    });
    output.push_str(",\"bestOrderedLineMarkRecordCoverageSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &slots, |slot| {
        slot.ordered_line_mark_record_coverage_count > 0
    });
    output.push_str(",\"sameHeaderSlots\":[");
    for (index, slot) in same_header_slots.iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_slot_fit_json(output, slot);
    }
    output.push(']');
    output.push_str(",\"slots\":[");
    for (index, slot) in slots.iter().take(12).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_slot_fit_json(output, slot);
    }
    output.push_str("],\"renderPromotionContribution\":\"source-only-page-mark-slot-sequence-diagnostic-only\",\"renderPromotionBlockedReason\":\"page-mark-source-y-slot-candidates-do-not-decode-page-y-origin\"}");
}

pub(crate) fn push_table_grid_piecewise_record_family_gap_table_json(
    output: &mut String,
    table: &TableGridCrossTableRowBoundaryOffsetTable,
) {
    output.push_str("{\"tableCandidateIndex\":");
    output.push_str(&table.table_candidate_index.to_string());
    output.push_str(",\"sourceRange\":");
    output.push_str(&source_range_json(table.source_start, table.source_end));
    output.push_str(",\"rowCount\":");
    output.push_str(&table.row_count.to_string());
    output.push_str(",\"previousRecordIndexes\":");
    push_usize_array_json(output, &table.line_mark_record_indexes);
    output.push_str(",\"selectedRecordIndexes\":");
    push_usize_array_json(output, &table.selected_spacing_record_indexes);
    output.push_str(",\"previousPageMarkLineOffsetsFromEntryStart\":");
    push_usize_array_json(output, &table.page_mark_line_offsets_from_entry_start);
    output.push_str(",\"selectedPageMarkLineOffsetsFromEntryStart\":");
    push_usize_array_json(
        output,
        &table.selected_spacing_page_mark_line_offsets_from_entry_start,
    );
    output.push_str(",\"previousRecordYTopPx\":");
    push_f32_array_json(output, &table.line_mark_record_y_tops_px);
    output.push_str(",\"selectedRecordYTopPx\":");
    push_f32_array_json(output, &table.selected_spacing_record_y_tops_px);
    output.push_str(",\"selectedMinusPreviousRecordIndexGaps\":");
    push_i32_array_json(output, &table.selected_minus_previous_record_index_gaps);
    output.push_str(",\"selectedMinusPreviousRecordYDeltaPx\":");
    push_f32_array_json(output, &table.selected_minus_previous_record_y_delta_px);
    output.push_str(",\"previousStartResidualUnits\":");
    push_i32_array_json(output, &table.start_residual_units);
    output.push_str(",\"previousEndResidualUnits\":");
    push_i32_array_json(output, &table.end_residual_units);
    output.push_str(",\"previousSpanResidualUnits\":");
    push_i32_array_json(output, &table.span_residual_units);
    output.push_str(",\"selectedStartResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_start_residual_units);
    output.push_str(",\"selectedEndResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_end_residual_units);
    output.push_str(",\"selectedSpanResidualUnits\":");
    push_i32_array_json(output, &table.selected_spacing_span_residual_units);
    output.push('}');
}

pub(crate) fn push_table_grid_piecewise_record_family_gap_transition_json(
    output: &mut String,
    previous: &TableGridCrossTableRowBoundaryOffsetTable,
    next: &TableGridCrossTableRowBoundaryOffsetTable,
) {
    let previous_family_gap = previous
        .line_mark_record_indexes
        .last()
        .copied()
        .zip(next.line_mark_record_indexes.first().copied())
        .map(|(left, right)| signed_usize_delta_i32(right, left));
    let selected_family_gap = previous
        .selected_spacing_record_indexes
        .last()
        .copied()
        .zip(next.selected_spacing_record_indexes.first().copied())
        .map(|(left, right)| signed_usize_delta_i32(right, left));
    let previous_family_y_gap = previous
        .line_mark_record_y_tops_px
        .last()
        .copied()
        .zip(next.line_mark_record_y_tops_px.first().copied())
        .map(|(left, right)| right - left);
    let selected_family_y_gap = previous
        .selected_spacing_record_y_tops_px
        .last()
        .copied()
        .zip(next.selected_spacing_record_y_tops_px.first().copied())
        .map(|(left, right)| right - left);

    output.push_str("{\"fromTableCandidateIndex\":");
    output.push_str(&previous.table_candidate_index.to_string());
    output.push_str(",\"toTableCandidateIndex\":");
    output.push_str(&next.table_candidate_index.to_string());
    output.push_str(",\"sourceRangeGapUnits\":");
    output.push_str(
        &next
            .source_start
            .saturating_sub(previous.source_end)
            .to_string(),
    );
    output.push_str(",\"rowSourceStartGapUnits\":");
    let row_source_start_gap = previous
        .row_source_start_units
        .last()
        .copied()
        .zip(next.row_source_start_units.first().copied())
        .map(|(left, right)| signed_usize_delta_i32(right, left));
    push_optional_i32_json(output, row_source_start_gap);
    output.push_str(",\"previousFamilyRecordGap\":");
    push_optional_i32_json(output, previous_family_gap);
    output.push_str(",\"selectedFamilyRecordGap\":");
    push_optional_i32_json(output, selected_family_gap);
    output.push_str(",\"selectedMinusPreviousFamilyRecordGapDelta\":");
    push_optional_i32_json(
        output,
        selected_family_gap
            .zip(previous_family_gap)
            .map(|(selected, previous)| selected.saturating_sub(previous)),
    );
    output.push_str(",\"previousFamilyYGapPx\":");
    push_optional_f32_json(output, previous_family_y_gap);
    output.push_str(",\"selectedFamilyYGapPx\":");
    push_optional_f32_json(output, selected_family_y_gap);
    output.push_str(",\"selectedMinusPreviousFamilyYGapDeltaPx\":");
    push_optional_f32_json(
        output,
        selected_family_y_gap
            .zip(previous_family_y_gap)
            .map(|(selected, previous)| selected - previous),
    );
    output.push_str(",\"samePageMarkEntry\":");
    output.push_str(
        if previous.page_mark_records_within_single_entry
            && next.page_mark_records_within_single_entry
            && previous.selected_spacing_records_within_single_entry
            && next.selected_spacing_records_within_single_entry
        {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(crate) fn push_table_grid_source_unit_to_page_line_index_piecewise_fit_json(
    output: &mut String,
    probe: &TableGridCrossTableRowBoundaryOffsetProbe,
) {
    output.push_str("{\"source\":\"/DocumentText row source units+/LineMark previous-row-span records table-piecewise\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(
        ",\"fitBasis\":\"per-related-table-rowSourceStartUnits-to-lineMarkRecordIndexes\"",
    );
    output.push_str(",\"groupingBasis\":\"crossTableRowBoundaryOffsetConsistency.tables\"");
    output.push_str(",\"globalFitExact\":");
    output.push_str(if probe.source_unit_to_page_line_index_exact {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allTableFitsExact\":");
    output.push_str(
        if probe.source_unit_to_page_line_index_piecewise_all_tables_exact {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"maxTableFitResidualRecordIndexes\":");
    push_optional_f32_json(
        output,
        probe.source_unit_to_page_line_index_piecewise_max_abs_residual,
    );
    output.push_str(",\"samePageMarkEntryContinuity\":");
    output.push_str(if probe.all_records_within_single_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pieceCount\":");
    output.push_str(
        &probe
            .source_unit_to_page_line_index_piecewise_tables
            .len()
            .to_string(),
    );
    output.push_str(",\"transitionCount\":");
    output.push_str(
        &probe
            .source_unit_to_page_line_index_piecewise_transitions
            .len()
            .to_string(),
    );
    output.push_str(",\"pieces\":[");
    for (index, table) in probe
        .source_unit_to_page_line_index_piecewise_tables
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_source_unit_to_page_line_index_piecewise_table_json(output, table);
    }
    output.push_str("],\"transitions\":[");
    for (index, transition) in probe
        .source_unit_to_page_line_index_piecewise_transitions
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_source_unit_to_page_line_index_piecewise_transition_json(
            output, transition,
        );
    }
    output.push(']');
    output.push_str(",\"renderPromotionContribution\":");
    output.push_str(&json_string(
        "source-unit-to-page-line-piecewise-fit-diagnostic-only",
    ));
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("piecewise-fit-does-not-decode-page-y-origin"));
    output.push('}');
}

pub(crate) fn push_table_grid_source_unit_to_page_line_index_piecewise_table_json(
    output: &mut String,
    table: &TableGridSourceUnitToPageLineIndexPiecewiseTable,
) {
    output.push_str("{\"tableCandidateIndex\":");
    output.push_str(&table.table_candidate_index.to_string());
    output.push_str(",\"sourceRange\":");
    output.push_str(&source_range_json(table.source_start, table.source_end));
    output.push_str(",\"rowCount\":");
    output.push_str(&table.row_count.to_string());
    output.push_str(",\"rowSourceStartUnits\":");
    push_usize_array_json(output, &table.row_source_start_units);
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &table.line_mark_record_indexes);
    output.push_str(",\"slopeRecordIndexesPerSourceUnit\":");
    push_optional_f32_json(output, table.slope_record_indexes_per_source_unit);
    output.push_str(",\"interceptRecordIndex\":");
    push_optional_f32_json(output, table.intercept_record_index);
    output.push_str(",\"fittedRecordIndexes\":");
    push_f32_array_json(output, &table.fitted_record_indexes);
    output.push_str(",\"residualRecordIndexes\":");
    push_f32_array_json(output, &table.residual_record_indexes);
    output.push_str(",\"maxAbsResidualRecordIndexes\":");
    push_optional_f32_json(output, table.max_abs_residual_record_indexes);
    output.push_str(",\"exactFit\":");
    output.push_str(if table.exact_fit { "true" } else { "false" });
    output.push_str(",\"pageMarkRecordsWithinSingleEntry\":");
    output.push_str(if table.page_mark_records_within_single_entry {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_table_grid_source_unit_to_page_line_index_piecewise_transition_json(
    output: &mut String,
    transition: &TableGridSourceUnitToPageLineIndexPiecewiseTransition,
) {
    output.push_str("{\"fromTableCandidateIndex\":");
    output.push_str(&transition.from_table_candidate_index.to_string());
    output.push_str(",\"toTableCandidateIndex\":");
    output.push_str(&transition.to_table_candidate_index.to_string());
    output.push_str(",\"previousLastSourceUnit\":");
    output.push_str(&transition.previous_last_source_unit.to_string());
    output.push_str(",\"nextFirstSourceUnit\":");
    output.push_str(&transition.next_first_source_unit.to_string());
    output.push_str(",\"sourceRangeGapUnits\":");
    output.push_str(&transition.source_range_gap_units.to_string());
    output.push_str(",\"rowSourceStartGapUnits\":");
    output.push_str(&transition.row_source_start_gap_units.to_string());
    output.push_str(",\"previousLastRecordIndex\":");
    output.push_str(&transition.previous_last_record_index.to_string());
    output.push_str(",\"nextFirstRecordIndex\":");
    output.push_str(&transition.next_first_record_index.to_string());
    output.push_str(",\"lineMarkRecordGap\":");
    output.push_str(&transition.line_mark_record_gap.to_string());
    output.push_str(",\"samePageMarkEntry\":");
    output.push_str(if transition.same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_table_grid_source_unit_to_page_line_index_fit_row_json(
    output: &mut String,
    row: &TableGridSourceUnitToPageLineIndexFitRow,
) {
    output.push_str("{\"tableCandidateIndex\":");
    output.push_str(&row.table_candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"rowSourceStartUnits\":");
    output.push_str(&row.row_source_start_units.to_string());
    output.push_str(",\"lineMarkRecordIndex\":");
    output.push_str(&row.line_mark_record_index.to_string());
    output.push_str(",\"fittedRecordIndex\":");
    output.push_str(&format!("{:.3}", row.fitted_record_index));
    output.push_str(",\"residualRecordIndex\":");
    output.push_str(&format!("{:.3}", row.residual_record_index));
    output.push('}');
}

pub(crate) fn push_table_grid_page_mark_raw_reference_value_probe_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        output.push_str("null");
        return;
    };
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let Some(reference_layout) =
        diagnostic_reference_table_grid_overlay_layout(layout, document, candidate, column_count)
    else {
        output.push_str("null");
        return;
    };
    let row_count = candidate.intervals().len();
    if row_count == 0 {
        output.push_str("null");
        return;
    }

    let record_headers = page_mark_record_headers(page_mark_bytes);
    const TOLERANCE_PX: f32 = 2.0;
    let mut total_hit_count = 0usize;
    let mut row_top_hit_count = 0usize;
    let mut hit_record_contexts = Vec::new();
    output.push_str("{\"source\":\"/PageMark raw numeric scan+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"tolerancePx\":");
    output.push_str(&format!("{TOLERANCE_PX:.3}"));
    output.push_str(",\"referenceBBox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        reference_layout.x,
        reference_layout.y,
        reference_layout.width,
        reference_layout.row_height * row_count as f32
    ));
    output.push_str(",\"rowTopTargets\":[");
    for row_index in 0..row_count {
        if row_index > 0 {
            output.push(',');
        }
        let target = reference_layout.y + row_index as f32 * reference_layout.row_height;
        let hits = page_mark_raw_numeric_hits_near(page_mark_bytes, target, TOLERANCE_PX);
        if !hits.is_empty() {
            row_top_hit_count += 1;
            total_hit_count += hits.len();
            for hit in &hits {
                if let Some(context) = page_mark_raw_numeric_hit_record_context(
                    &record_headers,
                    page_mark_bytes.len(),
                    hit.byte_offset,
                ) {
                    hit_record_contexts.push(context);
                }
            }
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"targetPx\":");
        output.push_str(&format!("{target:.3}"));
        output.push_str(",\"roundedTarget\":");
        output.push_str(&(target.round() as i32).to_string());
        output.push_str(",\"hitCount\":");
        output.push_str(&hits.len().to_string());
        output.push_str(",\"hits\":[");
        for (hit_index, hit) in hits.iter().take(12).enumerate() {
            if hit_index > 0 {
                output.push(',');
            }
            push_page_mark_raw_numeric_hit_json(output, hit, page_mark_bytes, &record_headers);
        }
        output.push_str("]}");
    }
    output.push(']');
    output.push_str(",\"rowTopTargetCount\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"rowTopTargetHitCount\":");
    output.push_str(&row_top_hit_count.to_string());
    output.push_str(",\"allRowTopTargetsHit\":");
    output.push_str(if row_top_hit_count == row_count {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"totalHitCount\":");
    output.push_str(&total_hit_count.to_string());
    output.push_str(",\"rawHitRecordContextSummary\":");
    push_page_mark_raw_numeric_hit_record_context_summary_json(output, &hit_record_contexts);
    output.push_str(",\"renderPromotionContribution\":\"reference-value-probe-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-raw-numeric-values-are-reference-probe-not-source-transform",
    ));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_line_mark_stride_promotion_readiness_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
    rows: &[TableCandidateLineHeaderRow],
    rows_with_headers: usize,
    raw_header_count: usize,
    stride: &TableGridLineMarkPageOriginStrideCandidate,
    reference_layout: &TableGridReferenceLayout,
) {
    let row_count = rows
        .len()
        .min(stride.raw_record_index_row_tops.len())
        .min(stride.stride_collapsed_row_tops.len());
    let candidate_row_count = candidate.intervals().len();
    let candidate_segment_count = candidate.cell_count_candidate();
    let matched_segment_count = sibling
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    let sparse_topology_complete = sibling.rows.len() == candidate_row_count
        && matched_segment_count == candidate_segment_count;
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    let all_rows_have_line_mark = stride.line_mark_record_indexes.len() == candidate_row_count;
    let uniform_record_stride = uniform_usize_stride(&stride.line_mark_record_indexes);

    let reference_row_tops = (0..row_count)
        .map(|row| reference_layout.y + row as f32 * reference_layout.row_height)
        .collect::<Vec<_>>();
    let raw_residuals = stride
        .raw_record_index_row_tops
        .iter()
        .take(row_count)
        .zip(&reference_row_tops)
        .map(|(candidate, reference)| candidate - reference)
        .collect::<Vec<_>>();
    let raw_max_abs = max_abs_f32(&raw_residuals);
    let reference_validation_threshold_px = 8.0f32;
    let raw_record_index_reference_fit =
        raw_max_abs.is_some_and(|value| value <= reference_validation_threshold_px);

    let (post_gap_match_count, post_gap_exact_count) =
        table_grid_sparse_sibling_post_row_gap_line_mark_correlation_counts(
            document, candidate, sibling,
        );
    let post_gap_correlation_complete =
        post_gap_match_count == candidate_row_count && post_gap_exact_count == candidate_row_count;
    let (raw_page_mark_scan_header_count, raw_page_mark_single_header_matched) =
        table_grid_page_mark_raw_scan_header_match_summary(
            document,
            &stride.line_mark_record_indexes,
        );
    let subrecord_span_readiness =
        table_grid_page_mark_subrecord_line_span_readiness(document, candidate);
    let subrecord_span_selected_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            !readiness.selected_post_row_gap_span_targets.is_empty()
                && readiness.selected_post_row_gap_span_hit_count
                    == readiness.selected_post_row_gap_span_targets.len()
        });
    let subrecord_span_previous_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            !readiness.previous_row_span_targets.is_empty()
                && readiness.previous_row_span_hit_count
                    == readiness.previous_row_span_targets.len()
        });
    let subrecord_span_compact_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            !readiness.compact_row_span_targets.is_empty()
                && readiness.compact_row_span_hit_count == readiness.compact_row_span_targets.len()
        });

    let mut blocked_reasons = Vec::new();
    if !all_rows_have_line_mark {
        blocked_reasons.push("line-mark-row-match-incomplete");
    }
    if uniform_record_stride.is_none() {
        blocked_reasons.push("line-mark-record-stride-not-uniform");
    }
    if !sparse_topology_complete {
        blocked_reasons.push("sparse-sibling-topology-incomplete");
    }
    if rows_with_headers < candidate_row_count {
        blocked_reasons.push("partial-line-header-font-evidence");
    }
    if matched_cell_header_count < candidate_segment_count {
        blocked_reasons.push("line-header-cell-geometry-incomplete");
    }
    if post_gap_correlation_complete {
        blocked_reasons.push("line-mark-spans-post-row-gaps-not-visible-row-heights");
    }
    if raw_page_mark_single_header_matched != Some(true) {
        blocked_reasons.push("page-mark-raw-record-scan-does-not-isolate-table-y");
    }
    if subrecord_span_selected_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-selected-post-row-gaps");
    }
    if subrecord_span_previous_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-previous-row-spans");
    }
    if subrecord_span_compact_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-compact-row-spans");
    }
    if subrecord_span_selected_complete
        || subrecord_span_previous_complete
        || subrecord_span_compact_complete
    {
        blocked_reasons.push("page-mark-subrecord-spans-do-not-decode-page-y-origin");
    }
    if !raw_record_index_reference_fit {
        blocked_reasons.push("raw-record-index-y-fails-current-reference-table");
    } else {
        blocked_reasons.push("raw-record-index-y-fit-is-reference-backed-only");
    }
    blocked_reasons.push("decoded-line-mark-stride-to-page-y-transform-missing");

    output.push_str("{\"source\":\"/LineMark+/PageMark+sparseSiblingEvidence\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":false");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate_row_count.to_string());
    output.push_str(",\"candidateSegmentCount\":");
    output.push_str(&candidate_segment_count.to_string());
    output.push_str(",\"allRowsHaveLineMark\":");
    output.push_str(if all_rows_have_line_mark {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &stride.line_mark_record_indexes);
    output.push_str(",\"uniformRecordStride\":");
    output.push_str(if uniform_record_stride.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"recordStride\":");
    push_option_usize_json(output, uniform_record_stride);
    output.push_str(",\"sparseTopologyComplete\":");
    output.push_str(if sparse_topology_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedSparseRowCount\":");
    output.push_str(&sibling.rows.len().to_string());
    output.push_str(",\"matchedSparseSegmentCount\":");
    output.push_str(&matched_segment_count.to_string());
    output.push_str(",\"lineHeaderRowsWithHeaders\":");
    output.push_str(&rows_with_headers.to_string());
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&raw_header_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&matched_cell_header_count.to_string());
    output.push_str(",\"postRowGapCorrelationComplete\":");
    output.push_str(if post_gap_correlation_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"postRowGapMatchCount\":");
    output.push_str(&post_gap_match_count.to_string());
    output.push_str(",\"postRowGapExactSpanMatchCount\":");
    output.push_str(&post_gap_exact_count.to_string());
    output.push_str(",\"rawPageMarkScanHeaderCount\":");
    push_option_usize_json(output, raw_page_mark_scan_header_count);
    output.push_str(",\"rawPageMarkSingleHeaderMatched\":");
    push_optional_bool_json(output, raw_page_mark_single_header_matched);
    output.push_str(",\"subrecordLineSpanReadiness\":");
    push_table_grid_page_mark_subrecord_line_span_readiness_json(
        output,
        subrecord_span_readiness.as_ref(),
    );
    output.push_str(",\"referenceValidationThresholdPx\":");
    output.push_str(&format!("{reference_validation_threshold_px:.3}"));
    output.push_str(",\"rawRecordIndexReferenceFit\":");
    output.push_str(if raw_record_index_reference_fit {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rawRecordIndexMaxAbsResidualPx\":");
    push_optional_f32_json(output, raw_max_abs);
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"stride-promotion-gate-diagnostic-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-stride-y-transform-not-source-decoded",
    ));
    output.push('}');
}

pub(crate) fn table_grid_sparse_sibling_post_row_gap_line_mark_correlation_counts(
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
) -> (usize, usize) {
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() {
        return (0, 0);
    }

    let mut match_count = 0usize;
    let mut exact_count = 0usize;
    for (row_index, row) in sibling.rows.iter().enumerate() {
        let Some(interval) = candidate
            .intervals()
            .iter()
            .find(|interval| interval.index() == row.compact_row_index)
        else {
            continue;
        };
        let Some(line_mark) =
            table_grid_interval_line_mark(candidate, interval, &line_mark_intervals)
        else {
            continue;
        };
        let Some(gap) = table_grid_sparse_sibling_post_row_gap(sibling, row_index) else {
            continue;
        };
        match_count += 1;
        let line_mark_span_units = line_mark.unit_end.saturating_sub(line_mark.unit_start);
        let post_row_gap_units = gap.source_end.saturating_sub(gap.source_start);
        if line_mark_span_units == post_row_gap_units {
            exact_count += 1;
        }
    }
    (match_count, exact_count)
}

pub(crate) fn table_grid_page_mark_raw_scan_header_match_summary(
    document: &Document,
    line_mark_record_indexes: &[usize],
) -> (Option<usize>, Option<bool>) {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        return (None, None);
    };
    let record_headers = page_mark_record_headers(page_mark_bytes);
    if record_headers.is_empty() || line_mark_record_indexes.is_empty() {
        return (Some(record_headers.len()), None);
    }
    let matched_indexes = line_mark_record_indexes
        .iter()
        .filter_map(|record_index| {
            record_headers.iter().position(|header| {
                header.line_start as usize <= *record_index
                    && *record_index <= header.line_end as usize
            })
        })
        .collect::<Vec<_>>();
    let single_header_matched = matched_indexes.len() == line_mark_record_indexes.len()
        && matched_indexes.first().is_some_and(|first| {
            matched_indexes
                .iter()
                .all(|matched_index| matched_index == first)
        });
    (Some(record_headers.len()), Some(single_header_matched))
}

pub(crate) fn table_grid_sparse_sibling_post_row_gap(
    sibling: &TableGridSparseSiblingEvidence<'_>,
    row_index: usize,
) -> Option<TableGridSparseSiblingPostRowGap> {
    let row = sibling.rows.get(row_index)?;
    let source_start = row.source_end;
    let (source_end, sparse_row_indexes, sparse_source_interval_indexes, kind) =
        if let Some(next_row) = sibling.rows.get(row_index + 1) {
            let gap_rows = sibling
                .sparse_candidate
                .intervals()
                .iter()
                .filter(|interval| {
                    row.sparse_row_index < interval.index()
                        && interval.index() < next_row.sparse_row_index
                })
                .collect::<Vec<_>>();
            (
                next_row.source_start,
                gap_rows
                    .iter()
                    .map(|interval| interval.index())
                    .collect::<Vec<_>>(),
                gap_rows
                    .iter()
                    .map(|interval| interval.source_interval_index())
                    .collect::<Vec<_>>(),
                "between-matched-sparse-rows",
            )
        } else {
            let trailing_empty_rows = sibling
                .sparse_candidate
                .intervals()
                .iter()
                .filter(|interval| interval.index() > row.sparse_row_index)
                .take_while(|interval| table_candidate_interval_non_empty_cell_count(interval) == 0)
                .collect::<Vec<_>>();
            let last_empty_row = trailing_empty_rows.last()?;
            (
                last_empty_row.source_end(),
                trailing_empty_rows
                    .iter()
                    .map(|interval| interval.index())
                    .collect::<Vec<_>>(),
                trailing_empty_rows
                    .iter()
                    .map(|interval| interval.source_interval_index())
                    .collect::<Vec<_>>(),
                "trailing-empty-sparse-rows",
            )
        };
    if source_end <= source_start {
        return None;
    }
    Some(TableGridSparseSiblingPostRowGap {
        source_start,
        source_end,
        sparse_row_indexes,
        sparse_source_interval_indexes,
        kind,
    })
}

pub(crate) fn push_table_grid_line_mark_row_gap_sequence_evidence_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
) {
    let Some(line_mark_bytes) = raw_stream_bytes(document, LINE_MARK_PATH) else {
        output.push_str("null");
        return;
    };
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() || sibling.rows.is_empty() {
        output.push_str("null");
        return;
    }

    let rows = table_grid_line_mark_row_gap_sequence_rows(candidate, sibling, &line_mark_intervals);
    if rows.is_empty() {
        output.push_str("null");
        return;
    }

    let words = be16_words(line_mark_bytes).collect::<Vec<_>>();
    let tag_family_counts = line_mark_tag_family_counts(&words);
    let tag_count = tag_family_counts.iter().sum::<usize>();
    let selected_post_gap_match_count = rows
        .iter()
        .filter(|row| {
            row.post_row_gap.as_ref().is_some_and(|gap| {
                line_mark_interval_span_units(row.selected_line_mark)
                    == table_grid_source_span_units(
                        candidate.basis(),
                        gap.source_start,
                        gap.source_end,
                    )
            })
        })
        .count();
    let previous_row_span_match_count = rows
        .iter()
        .filter(|row| {
            row.previous_line_mark.is_some_and(|previous| {
                line_mark_interval_span_units(previous)
                    == row
                        .row_source_end_units
                        .saturating_sub(row.row_source_start_units)
            })
        })
        .count();
    let next_row_span_match_count = rows
        .iter()
        .filter(|row| {
            row.next_line_mark.zip(row.next_row_span_units).is_some_and(
                |(next, next_row_span_units)| {
                    line_mark_interval_span_units(next) == next_row_span_units
                },
            )
        })
        .count();
    let rows_with_next_row = rows
        .iter()
        .filter(|row| row.next_row_span_units.is_some())
        .count();

    output.push_str("{\"source\":\"/LineMark+sparseTableSiblingEvidence\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(shanai_lan_line_mark_profile(document)));
    output.push_str(",\"lineMarkStreamByteLength\":");
    output.push_str(&line_mark_bytes.len().to_string());
    output.push_str(",\"lineMarkWordCount\":");
    output.push_str(&words.len().to_string());
    output.push_str(",\"lineMarkTagCount\":");
    output.push_str(&tag_count.to_string());
    output.push_str(",\"lineMarkTagFamilyCounts\":");
    push_line_mark_tag_family_counts_json(output, tag_family_counts);
    output.push_str(",\"tagPayloadCorrelation\":");
    output.push_str(&json_string(if tag_count == 0 {
        "not-applicable-no-line-mark-tags"
    } else {
        "nearest-tag-window-diagnostic-only"
    }));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&sibling.rows.len().to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"selectedRecordPostRowGapSpanMatchCount\":");
    output.push_str(&selected_post_gap_match_count.to_string());
    output.push_str(",\"allSelectedRecordsMatchPostRowGapSpan\":");
    output.push_str(if selected_post_gap_match_count == rows.len() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousRecordRowSpanMatchCount\":");
    output.push_str(&previous_row_span_match_count.to_string());
    output.push_str(",\"allPreviousRecordsMatchRowSpan\":");
    output.push_str(if previous_row_span_match_count == rows.len() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nextRecordNextRowSpanMatchCount\":");
    output.push_str(&next_row_span_match_count.to_string());
    output.push_str(",\"rowsWithNextRow\":");
    output.push_str(&rows_with_next_row.to_string());
    output.push_str(",\"sequenceInterpretationCandidate\":");
    output.push_str(&json_string(
        "alternating-row-span-record-then-post-row-gap-record",
    ));
    output.push_str(
        ",\"renderPromotionContribution\":\"line-mark-row-gap-sequence-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-row-gap-sequence-does-not-decode-page-y-transform",
    ));
    output.push_str(",\"rows\":[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_line_mark_row_gap_sequence_row_json(output, candidate, &words, row);
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_line_mark_row_gap_sequence_y_comparison_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
    row_height: f32,
    reference_layout: &TableGridReferenceLayout,
) {
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if line_mark_intervals.is_empty() || sibling.rows.is_empty() || row_height <= 0.0 {
        output.push_str("null");
        return;
    }
    let rows = table_grid_line_mark_row_gap_sequence_rows(candidate, sibling, &line_mark_intervals);
    if rows.is_empty() {
        output.push_str("null");
        return;
    }
    let selected_record_indexes = rows
        .iter()
        .map(|row| row.selected_line_mark.record_index)
        .collect::<Vec<_>>();
    let previous_record_indexes = rows
        .iter()
        .filter_map(|row| {
            row.previous_line_mark
                .map(|line_mark| line_mark.record_index)
        })
        .collect::<Vec<_>>();
    if selected_record_indexes.len() != rows.len() || previous_record_indexes.len() != rows.len() {
        output.push_str("null");
        return;
    }
    let Some(selected_context) = table_grid_page_mark_context_for_line_mark_record_indexes(
        document,
        &selected_record_indexes,
    ) else {
        output.push_str("null");
        return;
    };
    let previous_context = table_grid_page_mark_context_for_line_mark_record_indexes(
        document,
        &previous_record_indexes,
    );
    let row_count = rows.len();
    let reference_row_tops = (0..row_count)
        .map(|row| reference_layout.y + row as f32 * reference_layout.row_height)
        .collect::<Vec<_>>();
    let selected_body_line_pitch = table_grid_page_mark_line_pitch_candidate(
        layout,
        selected_context.page_line_start,
        selected_context.page_line_end,
    )
    .map(|(pitch, _)| pitch);
    let previous_body_line_pitch = previous_context.as_ref().and_then(|context| {
        table_grid_page_mark_line_pitch_candidate(
            layout,
            context.page_line_start,
            context.page_line_end,
        )
        .map(|(pitch, _)| pitch)
    });

    let selected_row_height_residuals = line_mark_record_indexes_y_residuals(
        layout,
        &selected_record_indexes,
        selected_context.page_line_start,
        row_height,
        &reference_row_tops,
    );
    let previous_row_height_residuals = previous_context.as_ref().map(|context| {
        line_mark_record_indexes_y_residuals(
            layout,
            &previous_record_indexes,
            context.page_line_start,
            row_height,
            &reference_row_tops,
        )
    });
    let selected_body_line_residuals = selected_body_line_pitch.map(|pitch| {
        line_mark_record_indexes_y_residuals(
            layout,
            &selected_record_indexes,
            selected_context.page_line_start,
            pitch,
            &reference_row_tops,
        )
    });
    let previous_body_line_residuals =
        previous_context
            .as_ref()
            .zip(previous_body_line_pitch)
            .map(|(context, pitch)| {
                line_mark_record_indexes_y_residuals(
                    layout,
                    &previous_record_indexes,
                    context.page_line_start,
                    pitch,
                    &reference_row_tops,
                )
            });
    let candidates = [
        (
            "selected-spacing-records-row-height-pitch",
            max_abs_f32(&selected_row_height_residuals),
        ),
        (
            "previous-row-span-records-row-height-pitch",
            previous_row_height_residuals
                .as_ref()
                .and_then(|residuals| max_abs_f32(residuals)),
        ),
        (
            "selected-spacing-records-page-line-pitch",
            selected_body_line_residuals
                .as_ref()
                .and_then(|residuals| max_abs_f32(residuals)),
        ),
        (
            "previous-row-span-records-page-line-pitch",
            previous_body_line_residuals
                .as_ref()
                .and_then(|residuals| max_abs_f32(residuals)),
        ),
    ];
    let best_candidate = candidates
        .iter()
        .filter_map(|(name, residual)| residual.map(|value| (*name, value)))
        .min_by(|(_, left), (_, right)| left.partial_cmp(right).unwrap_or(Ordering::Equal));

    output.push_str("{\"source\":\"/LineMark row/gap sequence+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"rowCountCompared\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"referenceRowTops\":");
    push_f32_array_json(output, &reference_row_tops);
    output.push_str(",\"selectedSpacingRecordIndexes\":");
    push_usize_array_json(output, &selected_record_indexes);
    output.push_str(",\"previousRowSpanRecordIndexes\":");
    push_usize_array_json(output, &previous_record_indexes);
    output.push_str(",\"recordFamilyInterpretation\":\"selected-records-match-post-row-gaps-previous-records-match-row-spans\"");
    output.push_str(",\"selectedSpacingRecordCandidate\":");
    push_line_mark_record_family_y_candidate_json(
        output,
        layout,
        "selected-spacing-records",
        "post-row-gap-span",
        &selected_record_indexes,
        &selected_context,
        row_height,
        selected_body_line_pitch,
        &reference_row_tops,
    );
    output.push_str(",\"previousRowSpanRecordCandidate\":");
    match previous_context.as_ref() {
        Some(context) => push_line_mark_record_family_y_candidate_json(
            output,
            layout,
            "previous-row-span-records",
            "compact-row-span",
            &previous_record_indexes,
            context,
            row_height,
            previous_body_line_pitch,
            &reference_row_tops,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestCandidate\":");
    match best_candidate {
        Some((name, _)) => output.push_str(&json_string(name)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestCandidateMaxAbsResidualPx\":");
    push_optional_f32_json(output, best_candidate.map(|(_, residual)| residual));
    output.push_str(",\"renderPromotionContribution\":\"row-gap-record-family-y-diagnostic-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-record-family-y-transform-not-source-decoded",
    ));
    output.push('}');
}

pub(crate) fn table_grid_page_mark_context_for_line_mark_record_indexes(
    document: &Document,
    record_indexes: &[usize],
) -> Option<TableGridPageMarkLineContext> {
    if record_indexes.is_empty() {
        return None;
    }
    let page_mark = document.page_marks().first()?;
    let mut entries = Vec::new();
    for record_index in record_indexes {
        entries.push(table_grid_page_mark_entry_for_line_mark_record(
            Some(page_mark),
            *record_index,
        )?);
    }
    let first_entry = entries.first().copied()?;
    if !entries
        .iter()
        .all(|entry| entry.row_index() == first_entry.row_index())
    {
        return None;
    }
    Some(TableGridPageMarkLineContext {
        page_mark_entry_index: first_entry.row_index(),
        page_index_candidate: first_entry.index().map(|index| index as usize),
        page_line_start: first_entry.line_start()? as usize,
        page_line_end: first_entry.line_end()? as usize,
        page_mark_u16_fields: first_entry.u16_fields().to_vec(),
    })
}

pub(crate) fn table_grid_line_mark_row_gap_sequence_rows(
    candidate: &TableCandidate,
    sibling: &TableGridSparseSiblingEvidence<'_>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) -> Vec<TableGridLineMarkRowGapSequenceRow> {
    let mut rows = Vec::new();
    for (row_index, row) in sibling.rows.iter().enumerate() {
        let Some(interval) = candidate
            .intervals()
            .iter()
            .find(|interval| interval.index() == row.compact_row_index)
        else {
            continue;
        };
        let Some(selected_line_mark) =
            table_grid_interval_line_mark(candidate, interval, line_mark_intervals)
        else {
            continue;
        };
        let previous_line_mark =
            selected_line_mark
                .record_index
                .checked_sub(1)
                .and_then(|record_index| {
                    line_mark_interval_for_record(line_mark_intervals, record_index)
                });
        let next_line_mark =
            line_mark_interval_for_record(line_mark_intervals, selected_line_mark.record_index + 1);
        let next_row_span_units = sibling.rows.get(row_index + 1).map(|next_row| {
            table_grid_source_span_units(
                candidate.basis(),
                next_row.source_start,
                next_row.source_end,
            )
        });
        rows.push(TableGridLineMarkRowGapSequenceRow {
            compact_row_index: row.compact_row_index,
            sparse_row_index: row.sparse_row_index,
            source_interval_index: row.source_interval_index,
            row_source_start: row.source_start,
            row_source_end: row.source_end,
            row_source_start_units: table_source_offset_to_units(
                candidate.basis(),
                row.source_start,
            ),
            row_source_end_units: table_source_offset_to_units(candidate.basis(), row.source_end),
            selected_line_mark,
            previous_line_mark,
            next_line_mark,
            post_row_gap: table_grid_sparse_sibling_post_row_gap(sibling, row_index),
            next_row_span_units,
        });
    }
    rows
}

pub(crate) fn table_grid_previous_row_span_line_mark_record_indexes(
    document: &Document,
    candidate: &TableCandidate,
) -> Vec<usize> {
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    table_grid_previous_row_span_line_mark_rows_for_rows(
        document,
        candidate,
        &rows,
        &line_mark_intervals,
    )
    .map(|rows| {
        rows.into_iter()
            .map(|row| row.interval.record_index)
            .collect()
    })
    .unwrap_or_default()
}

pub(crate) fn table_grid_resolved_line_mark_rows_for_rows(
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
) -> Vec<TableGridResolvedLineMarkRow> {
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if rows.is_empty() || line_mark_intervals.is_empty() {
        return Vec::new();
    }

    if let Some(rows) = table_grid_previous_row_span_line_mark_rows_for_rows(
        document,
        candidate,
        rows,
        &line_mark_intervals,
    ) {
        return rows;
    }

    rows.iter()
        .filter_map(|row| {
            let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start);
            let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end);
            best_line_mark_interval_for_unit_range(
                &line_mark_intervals,
                row_unit_start,
                row_unit_end,
            )
            .map(|interval| TableGridResolvedLineMarkRow {
                interval,
                role: TableGridLineMarkRowRecordRole::SelectedOverlap,
            })
        })
        .collect()
}

pub(crate) fn table_grid_previous_row_span_line_mark_rows_for_rows(
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) -> Option<Vec<TableGridResolvedLineMarkRow>> {
    if rows.is_empty() || rows.len() != candidate.intervals().len() {
        return None;
    }
    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate)?;
    let matched_segment_count = sibling
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    if sibling.rows.len() != candidate.intervals().len()
        || matched_segment_count != candidate.cell_count_candidate()
    {
        return None;
    }

    let sequence_rows =
        table_grid_line_mark_row_gap_sequence_rows(candidate, &sibling, line_mark_intervals);
    if sequence_rows.len() != rows.len() {
        return None;
    }

    let mut resolved_rows = Vec::new();
    for row in rows {
        let sequence_row = sequence_rows.iter().find(|sequence_row| {
            sequence_row.compact_row_index == row.row_index
                && sequence_row.row_source_start == row.source_start
                && sequence_row.row_source_end == row.source_end
        })?;
        let previous = sequence_row.previous_line_mark?;
        let row_span_units = sequence_row
            .row_source_end_units
            .saturating_sub(sequence_row.row_source_start_units);
        if line_mark_interval_span_units(previous) != row_span_units {
            return None;
        }
        let selected_record_matches_post_row_gap =
            sequence_row.post_row_gap.as_ref().is_some_and(|gap| {
                line_mark_interval_span_units(sequence_row.selected_line_mark)
                    == table_grid_source_span_units(
                        candidate.basis(),
                        gap.source_start,
                        gap.source_end,
                    )
            });
        if !selected_record_matches_post_row_gap {
            return None;
        }
        resolved_rows.push(TableGridResolvedLineMarkRow {
            interval: previous,
            role: TableGridLineMarkRowRecordRole::PreviousCompactRowSpan,
        });
    }

    resolved_rows
        .windows(2)
        .all(|pair| pair[0].interval.record_index < pair[1].interval.record_index)
        .then_some(resolved_rows)
}

pub(crate) fn push_table_grid_line_mark_row_gap_sequence_row_json(
    output: &mut String,
    candidate: &TableCandidate,
    words: &[u16],
    row: &TableGridLineMarkRowGapSequenceRow,
) {
    let row_span_units = row
        .row_source_end_units
        .saturating_sub(row.row_source_start_units);
    let post_row_gap_units = row.post_row_gap.as_ref().map(|gap| {
        table_grid_source_span_units(candidate.basis(), gap.source_start, gap.source_end)
    });
    let selected_record_matches_post_row_gap = post_row_gap_units
        .is_some_and(|units| line_mark_interval_span_units(row.selected_line_mark) == units);
    let previous_record_matches_row_span = row
        .previous_line_mark
        .is_some_and(|previous| line_mark_interval_span_units(previous) == row_span_units);
    let next_record_matches_next_row_span = row
        .next_line_mark
        .zip(row.next_row_span_units)
        .is_some_and(|(next, next_row_span_units)| {
            line_mark_interval_span_units(next) == next_row_span_units
        });

    output.push_str("{\"compactRow\":");
    output.push_str(&row.compact_row_index.to_string());
    output.push_str(",\"sparseRow\":");
    output.push_str(&row.sparse_row_index.to_string());
    output.push_str(",\"sourceIntervalIndex\":");
    output.push_str(&row.source_interval_index.to_string());
    output.push_str(",\"rowSourceRange\":");
    output.push_str(&source_range_json(row.row_source_start, row.row_source_end));
    output.push_str(",\"rowSourceUnitRange\":");
    output.push_str(&source_range_json(
        row.row_source_start_units,
        row.row_source_end_units,
    ));
    output.push_str(",\"rowSpanUnits\":");
    output.push_str(&row_span_units.to_string());
    output.push_str(",\"selectedLineMarkRecord\":");
    push_line_mark_delta_record_json(output, words, Some(row.selected_line_mark));
    output.push_str(",\"selectedRecordMatchesPostRowGapSpan\":");
    output.push_str(if selected_record_matches_post_row_gap {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousLineMarkRecord\":");
    push_line_mark_delta_record_json(output, words, row.previous_line_mark);
    output.push_str(",\"previousRecordMatchesRowSpan\":");
    output.push_str(if previous_record_matches_row_span {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nextLineMarkRecord\":");
    push_line_mark_delta_record_json(output, words, row.next_line_mark);
    output.push_str(",\"nextRowSpanUnits\":");
    push_option_usize_json(output, row.next_row_span_units);
    output.push_str(",\"nextRecordMatchesNextRowSpan\":");
    output.push_str(if next_record_matches_next_row_span {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"postRowGap\":");
    match row.post_row_gap.as_ref() {
        Some(gap) => {
            let source_start_units =
                table_source_offset_to_units(candidate.basis(), gap.source_start);
            let source_end_units = table_source_offset_to_units(candidate.basis(), gap.source_end);
            output.push_str("{\"sourceRange\":");
            output.push_str(&source_range_json(gap.source_start, gap.source_end));
            output.push_str(",\"sourceUnitRange\":");
            output.push_str(&source_range_json(source_start_units, source_end_units));
            output.push_str(",\"spanUnits\":");
            output.push_str(
                &source_end_units
                    .saturating_sub(source_start_units)
                    .to_string(),
            );
            output.push_str(",\"kind\":");
            output.push_str(&json_string(gap.kind));
            output.push_str(",\"sparseRowIndexes\":");
            push_usize_array_json(output, &gap.sparse_row_indexes);
            output.push_str(",\"sparseSourceIntervalIndexes\":");
            push_usize_array_json(output, &gap.sparse_source_interval_indexes);
            output.push('}');
        }
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(crate) fn table_grid_source_span_units(
    basis: TextCountRangeOverlapBasis,
    source_start: usize,
    source_end: usize,
) -> usize {
    table_source_offset_to_units(basis, source_end)
        .saturating_sub(table_source_offset_to_units(basis, source_start))
}

pub(crate) fn table_grid_line_header_font_size_units_candidate(
    rows: &[TableCandidateLineHeaderRow],
) -> Option<(u16, usize, usize)> {
    let mut font_size_units = None;
    let mut rows_with_headers = 0usize;
    let mut raw_header_count = 0usize;
    for row in rows {
        if row.headers.is_empty() {
            continue;
        }
        rows_with_headers += 1;
        raw_header_count += row.headers.len();
        for header in &row.headers {
            if header.font_size_units == 0 {
                return None;
            }
            match font_size_units {
                Some(previous) if previous != header.font_size_units => return None,
                Some(_) => {}
                None => font_size_units = Some(header.font_size_units),
            }
        }
    }
    font_size_units.map(|font_size_units| (font_size_units, rows_with_headers, raw_header_count))
}

pub(crate) fn push_table_grid_source_origin_residual_evidence_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: &TableGridSourceDerivedLayout,
    reference_layout: &TableGridReferenceLayout,
) {
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let first_header_words = rows
        .first()
        .and_then(|row| row.headers.first())
        .map(|header| header.raw_words.as_slice());
    let table_span_units = source_layout
        .x_unit_end
        .saturating_sub(source_layout.x_unit_start);
    let full_span_units = source_layout
        .max_extent_units
        .zip(source_layout.min_offset_units)
        .map(|(max_extent, min_offset)| max_extent.saturating_sub(min_offset));
    let x_residual = reference_layout.x - source_layout.x;
    let y_residual = reference_layout.y - source_layout.y;
    let width_residual = reference_layout.width - source_layout.width;
    let source_table_unit_width_px = if table_span_units > 0 {
        Some(source_layout.width / f32::from(table_span_units))
    } else {
        None
    };
    let source_full_unit_width_px = full_span_units
        .filter(|span| *span > 0)
        .map(|span| source_layout.width / f32::from(span));

    output.push_str(
        "{\"source\":\"sourceDerivedLayoutCandidate+referenceTableBBox+rawLayoutFields\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"referenceBacked\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"xResidualPx\":");
    output.push_str(&format!("{x_residual:.3}"));
    output.push_str(",\"yResidualPx\":");
    output.push_str(&format!("{y_residual:.3}"));
    output.push_str(",\"widthResidualPx\":");
    output.push_str(&format!("{width_residual:.3}"));
    output.push_str(",\"rowHeightResidualPx\":");
    output.push_str(&format!(
        "{:.3}",
        reference_layout.row_height - source_layout.row_height
    ));
    output.push_str(",\"xResidualInTableUnits\":");
    match source_table_unit_width_px.filter(|value| *value > 0.0) {
        Some(px_per_unit) => output.push_str(&format!("{:.3}", x_residual / px_per_unit)),
        None => output.push_str("null"),
    }
    output.push_str(",\"xResidualInFullExtentUnits\":");
    match source_full_unit_width_px.filter(|value| *value > 0.0) {
        Some(px_per_unit) => output.push_str(&format!("{:.3}", x_residual / px_per_unit)),
        None => output.push_str("null"),
    }
    output.push_str(",\"yResidualInRows\":");
    if source_layout.row_height > 0.0 {
        output.push_str(&format!("{:.3}", y_residual / source_layout.row_height));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceTableUnitWidthPx\":");
    match source_table_unit_width_px {
        Some(value) => output.push_str(&format!("{value:.3}")),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceFullExtentUnitWidthPx\":");
    match source_full_unit_width_px {
        Some(value) => output.push_str(&format!("{value:.3}")),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceFields\":{\"xUnitRangeBasis\":");
    output.push_str(&json_string(source_layout.x_unit_range_basis));
    output.push_str(",\"xUnitRange\":");
    output.push_str(&source_range_json(
        usize::from(source_layout.x_unit_start),
        usize::from(source_layout.x_unit_end),
    ));
    output.push_str(",\"tableSpanUnits\":");
    output.push_str(&table_span_units.to_string());
    output.push_str(",\"fullExtentUnits\":");
    match full_span_units {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"homogeneousFontSizeUnits\":");
    push_optional_u16_json(output, source_layout.homogeneous_font_size_units);
    output.push_str(",\"xOriginInsetUnits\":");
    output.push_str(&format!("{:.3}", source_layout.x_origin_inset_units));
    output.push_str(",\"xOriginInsetBasis\":");
    output.push_str(&json_string(source_layout.x_origin_inset_basis));
    output.push_str(",\"firstMatchedCellSpanUnits\":");
    match source_layout.matched_cell_span_units.first() {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"firstMatchedCellGapUnits\":");
    match source_layout.matched_cell_gap_units.first() {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"firstLineHeaderRawWords\":");
    match first_header_words {
        Some(words) => push_u16_array_json(output, words),
        None => output.push_str("null"),
    }
    output.push_str(",\"firstLineHeaderRawWordsHex\":");
    match first_header_words {
        Some(words) => push_u16_hex_array_json(output, words),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageMarkSelectedFields\":");
    let page_mark_fields = table_grid_source_layout_page_mark_u16_fields(source_layout);
    match page_mark_fields {
        Some(fields) => push_table_grid_origin_residual_page_mark_fields_json(output, fields),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageMarkRawFieldReferenceComparison\":");
    push_table_grid_origin_residual_page_mark_raw_field_reference_comparison_json(
        output,
        page_mark_fields,
        source_layout,
        reference_layout,
    );
    output.push_str(
        "},\"renderPromotionContribution\":\"origin-residual-targeted-source-field-comparison\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "origin-residual-not-explained-by-decoded-source-field",
    ));
    output.push('}');
}

pub(crate) fn table_grid_source_layout_page_mark_u16_fields(
    source_layout: &TableGridSourceDerivedLayout,
) -> Option<&[u16]> {
    source_layout
        .line_mark_page_origin
        .as_ref()
        .map(|origin| origin.page_mark_u16_fields.as_slice())
        .or_else(|| {
            source_layout
                .line_mark_page_origin_stride
                .as_ref()
                .map(|stride| stride.page_mark_u16_fields.as_slice())
        })
}

pub(crate) fn push_table_grid_origin_residual_page_mark_fields_json(
    output: &mut String,
    fields: &[u16],
) {
    output.push('[');
    for (index, word_index) in PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        let value = fields.get(*word_index).copied();
        output.push_str("{\"wordIndex\":");
        output.push_str(&word_index.to_string());
        output.push_str(",\"value\":");
        match value {
            Some(value) => output.push_str(&value.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"hex\":");
        match value {
            Some(value) => output.push_str(&json_string(&format!("0x{value:04x}"))),
            None => output.push_str("null"),
        }
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_table_grid_origin_residual_page_mark_raw_field_reference_comparison_json(
    output: &mut String,
    fields: Option<&[u16]>,
    source_layout: &TableGridSourceDerivedLayout,
    reference_layout: &TableGridReferenceLayout,
) {
    let Some(fields) = fields else {
        output.push_str("null");
        return;
    };
    let word_14 = fields.get(14).copied().map(f32::from);
    let word_21 = fields.get(21).copied().map(f32::from);
    let first_slot_units = source_layout
        .x_unit_column_slot_width_units
        .first()
        .copied();
    let first_span_units = source_layout.matched_cell_span_units.first().copied();
    let first_gap_units = source_layout.matched_cell_gap_units.first().copied();
    let word_14_x_residual = word_14.map(|value| value - reference_layout.x);
    let word_21_width_residual = word_21.map(|value| value - reference_layout.width);

    output.push_str("{\"source\":\"/PageMark selected u16 fields+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"comparisonBasis\":\"direct-u16-px-near-reference\"");
    output.push_str(",\"word14DirectPx\":");
    push_optional_f32_json(output, word_14);
    output.push_str(",\"referenceX\":");
    output.push_str(&format!("{:.3}", reference_layout.x));
    output.push_str(",\"word14MinusReferenceXPx\":");
    push_optional_f32_json(output, word_14_x_residual);
    output.push_str(",\"word21DirectPx\":");
    push_optional_f32_json(output, word_21);
    output.push_str(",\"referenceWidth\":");
    output.push_str(&format!("{:.3}", reference_layout.width));
    output.push_str(",\"word21MinusReferenceWidthPx\":");
    push_optional_f32_json(output, word_21_width_residual);
    output.push_str(",\"firstColumnSlotUnits\":");
    push_optional_u16_json(output, first_slot_units);
    output.push_str(",\"firstMatchedCellSpanUnits\":");
    push_optional_u16_json(output, first_span_units);
    output.push_str(",\"firstIntercellGapUnits\":");
    push_optional_u16_json(output, first_gap_units);
    output.push_str(",\"word14MinusReferenceXInFirstSlotUnits\":");
    push_optional_f32_json(
        output,
        word_14_x_residual
            .zip(first_slot_units)
            .and_then(|(residual, units)| (units > 0).then_some(residual / f32::from(units))),
    );
    output.push_str(",\"word21MinusReferenceWidthInHalfFirstSlotUnits\":");
    push_optional_f32_json(
        output,
        word_21_width_residual
            .zip(first_slot_units)
            .and_then(|(residual, units)| {
                (units > 0).then_some(residual / (f32::from(units) * 0.5))
            }),
    );
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-raw-horizontal-field-reference-comparison\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-raw-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

pub(crate) fn table_grid_line_mark_record_indexes_for_rows(
    document: &Document,
    candidate: &TableCandidate,
) -> Vec<usize> {
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows)
        .into_iter()
        .map(|row| row.interval.record_index)
        .collect()
}

pub(crate) fn push_table_grid_reference_unit_bbox_candidate_comparisons_json(
    output: &mut String,
    rows: &[TableCandidateLineHeaderRow],
    matched_column_count: usize,
    reference_width_px: f32,
    full_line_extent_units: u16,
) {
    if rows.is_empty() || matched_column_count == 0 || reference_width_px <= 0.0 {
        output.push_str("[]");
        return;
    }
    let Some(first_row) = rows.first() else {
        output.push_str("[]");
        return;
    };

    let candidates = [
        table_grid_unit_bbox_range_for_row(
            first_row,
            matched_column_count,
            TableGridUnitBBoxBasis::MatchedCells,
        ),
        table_grid_unit_bbox_range_for_row(
            first_row,
            matched_column_count,
            TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader,
        ),
        table_grid_unit_bbox_range_for_row(
            first_row,
            matched_column_count,
            TableGridUnitBBoxBasis::FullLineHeaderExtent,
        ),
    ];

    output.push('[');
    let mut first = true;
    for (basis, range) in [
        (TableGridUnitBBoxBasis::MatchedCells, candidates[0]),
        (
            TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader,
            candidates[1],
        ),
        (TableGridUnitBBoxBasis::FullLineHeaderExtent, candidates[2]),
    ] {
        let Some((start, end)) = range else {
            continue;
        };
        if start >= end {
            continue;
        }
        if !first {
            output.push(',');
        }
        first = false;
        let width_units = end.saturating_sub(start);
        output.push_str("{\"basis\":");
        output.push_str(&json_string(basis.as_str()));
        output.push_str(",\"xUnitRange\":");
        output.push_str(&source_range_json(usize::from(start), usize::from(end)));
        output.push_str(",\"widthUnits\":");
        output.push_str(&width_units.to_string());
        output.push_str(",\"referenceWidthPxPerUnit\":");
        output.push_str(&format!(
            "{:.3}",
            reference_width_px / f32::from(width_units)
        ));
        output.push_str(",\"widthRatioToFullLineExtent\":");
        if full_line_extent_units > 0 {
            output.push_str(&format!(
                "{:.3}",
                f32::from(width_units) / f32::from(full_line_extent_units)
            ));
        } else {
            output.push_str("null");
        }
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_table_grid_top_text_anchor_residual_evidence_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(reference_layout) = diagnostic_success_data_test_reference_table_grid_overlay_layout(
        layout, document, candidate,
    ) else {
        output.push_str("null");
        return;
    };
    let Some(slots) = success_data_test_resolved_top_text_projection(document, 1) else {
        output.push_str("null");
        return;
    };
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let table_max_extent_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.extent_units))
        .max();
    let table_font_size_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.font_size_units))
        .try_fold(None, |seen, value| match seen {
            Some(previous) if previous != value => None,
            _ => Some(Some(value)),
        })
        .flatten();
    let anchor_slots = slots
        .iter()
        .filter(|slot| slot.line_header.is_some())
        .collect::<Vec<_>>();
    if anchor_slots.is_empty() {
        output.push_str("null");
        return;
    }
    let shared_full_extent_anchor_count = anchor_slots
        .iter()
        .filter(|slot| {
            slot.line_header
                .is_some_and(|header| Some(header.extent_units) == table_max_extent_units)
        })
        .count();
    let shared_font_size_anchor_count = anchor_slots
        .iter()
        .filter(|slot| {
            slot.line_header
                .is_some_and(|header| Some(header.font_size_units) == table_font_size_units)
        })
        .count();

    output.push_str("{\"source\":\"successDataTestTopTextProjection+documentTextLineHeaders+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "top-text-page-coordinates-and-table-bbox-are-reference-backed",
    ));
    output.push_str(",\"sharedFullExtentAnchorCount\":");
    output.push_str(&shared_full_extent_anchor_count.to_string());
    output.push_str(",\"sharedFontSizeAnchorCount\":");
    output.push_str(&shared_font_size_anchor_count.to_string());
    output.push_str(",\"anchors\":[");
    for (index, slot) in anchor_slots.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let header = slot.line_header.unwrap();
        output.push_str("{\"text\":");
        output.push_str(&json_string(slot.text));
        output.push_str(",\"sourceUnitRange\":");
        match &slot.source_span {
            Some(span) => output.push_str(&source_range_json(span.unit_start(), span.unit_end())),
            None => output.push_str("null"),
        }
        output.push_str(",\"lineHeaderUnitRange\":");
        output.push_str(&source_range_json(header.start / 2, header.end / 2));
        output.push_str(",\"offsetUnits\":");
        output.push_str(&header.offset_units.to_string());
        output.push_str(",\"extentUnits\":");
        output.push_str(&header.extent_units.to_string());
        output.push_str(",\"fontSizeUnits\":");
        output.push_str(&header.font_size_units.to_string());
        output.push_str(",\"sharedFullExtentWithTable\":");
        output.push_str(if Some(header.extent_units) == table_max_extent_units {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"sharedFontSizeWithTable\":");
        output.push_str(if Some(header.font_size_units) == table_font_size_units {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"tableLeftMinusAnchorLeftPx\":");
        output.push_str(&format!("{:.3}", reference_layout.x - slot.x));
        output.push_str(",\"tableTopMinusAnchorBaselinePx\":");
        output.push_str(&format!(
            "{:.3}",
            reference_layout.y - (slot.y + SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX)
        ));
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_document_text_line_header_evidence_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let raw_header_count = rows
        .iter()
        .map(TableCandidateLineHeaderRow::raw_header_count)
        .sum::<usize>();
    let matched_row_count = rows.iter().filter(|row| row.matched_cell_count > 0).count();
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    output.push_str("{\"source\":");
    output.push_str(&json_string(DOCUMENT_TEXT_PATH));
    output.push_str(",\"present\":");
    output.push_str(if raw_header_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowRangeCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&matched_row_count.to_string());
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&raw_header_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&matched_cell_header_count.to_string());
    output.push_str(",\"unitGeometryCandidate\":");
    push_table_grid_line_header_unit_geometry_candidate_json(output, &rows);
    output.push_str(",\"rows\":[");
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row.row_index.to_string());
        output.push_str(",\"sourceRange\":");
        output.push_str(&source_range_json(row.source_start, row.source_end));
        output.push_str(",\"expectedCellCount\":");
        output.push_str(&row.expected_cell_count.to_string());
        output.push_str(",\"rawHeaderCount\":");
        output.push_str(&row.raw_header_count().to_string());
        output.push_str(",\"matchedCellCount\":");
        output.push_str(&row.matched_cell_count.to_string());
        output.push_str(",\"headers\":[");
        for (header_index, header) in row.headers.iter().enumerate() {
            if header_index > 0 {
                output.push(',');
            }
            output.push_str("{\"sourceStart\":");
            output.push_str(
                &table_line_header_source_offset(candidate.basis(), header.start).to_string(),
            );
            output.push_str(",\"sourceEnd\":");
            output.push_str(
                &table_line_header_source_offset(candidate.basis(), header.end).to_string(),
            );
            output.push_str(",\"offsetUnits\":");
            output.push_str(&header.offset_units.to_string());
            output.push_str(",\"extentUnits\":");
            output.push_str(&header.extent_units.to_string());
            output.push_str(",\"fontSizeUnits\":");
            output.push_str(&header.font_size_units.to_string());
            output.push_str(",\"rawWords\":");
            push_u16_array_json(output, &header.raw_words);
            output.push_str(",\"rawWordsHex\":");
            push_u16_hex_array_json(output, &header.raw_words);
            output.push('}');
        }
        output.push_str("]}");
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_sparse_table_sibling_evidence_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(evidence) = table_grid_sparse_table_sibling_evidence(document, candidate) else {
        output.push_str("null");
        return;
    };
    let sparse_candidate = evidence.sparse_candidate;
    let matched_row_count = evidence.rows.len();
    let candidate_row_count = candidate.intervals().len();
    let matched_segment_count = evidence
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    let candidate_segment_count = candidate.cell_count_candidate();
    let shared_source_interval_indexes = evidence
        .rows
        .iter()
        .map(|row| row.source_interval_index)
        .collect::<Vec<_>>();
    let matched_sparse_column_indexes = table_grid_sparse_sibling_matched_sparse_column_indexes(
        &evidence.rows,
        candidate.max_column_segment_count(),
    );
    let sparse_topology = sparse_candidate.sparse_topology_candidate();

    output.push_str("{\"source\":\"sparseDocumentTextControlRunTableCandidate\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sparseTableCandidateIndex\":");
    output.push_str(&sparse_candidate.index().to_string());
    output.push_str(",\"candidateSourceRange\":");
    output.push_str(&source_range_json(
        candidate.source_start(),
        candidate.source_end(),
    ));
    output.push_str(",\"sparseSourceRange\":");
    output.push_str(&source_range_json(
        sparse_candidate.source_start(),
        sparse_candidate.source_end(),
    ));
    output.push_str(",\"sourceRangeContainsCandidate\":");
    output.push_str(
        if sparse_candidate.source_start() <= candidate.source_start()
            && candidate.source_end() <= sparse_candidate.source_end()
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate_row_count.to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&matched_row_count.to_string());
    output.push_str(",\"allCandidateRowsMatched\":");
    output.push_str(if matched_row_count == candidate_row_count {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateSegmentCount\":");
    output.push_str(&candidate_segment_count.to_string());
    output.push_str(",\"matchedSegmentCount\":");
    output.push_str(&matched_segment_count.to_string());
    output.push_str(",\"allCandidateSegmentsMatched\":");
    output.push_str(if matched_segment_count == candidate_segment_count {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sharedSourceIntervalIndexes\":");
    push_usize_array_json(output, &shared_source_interval_indexes);
    output.push_str(",\"compactToSparseColumnOffsetCandidate\":");
    push_option_usize_json(output, evidence.compact_to_sparse_column_offset);
    output.push_str(",\"matchedSparseColumnIndexes\":");
    push_usize_array_json(output, &matched_sparse_column_indexes);
    output.push_str(",\"sparseMaxColumnCountCandidate\":");
    output.push_str(&sparse_candidate.max_column_segment_count().to_string());
    output.push_str(",\"sparseEmptyCellCountCandidate\":");
    output.push_str(&sparse_candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"sparseNonEmptyCellCountCandidate\":");
    output.push_str(
        &sparse_candidate
            .non_empty_cell_count_candidate()
            .to_string(),
    );
    output.push_str(",\"sparseTopologyCandidatePresent\":");
    output.push_str(if sparse_topology.is_some() {
        "true"
    } else {
        "false"
    });
    if let Some(topology) = sparse_topology.as_ref() {
        output.push_str(",\"sparseTopologySummary\":{\"rowCount\":");
        output.push_str(&topology.row_count().to_string());
        output.push_str(",\"maxColumnCountCandidate\":");
        output.push_str(&topology.max_column_count().to_string());
        output.push_str(",\"cellCountCandidate\":");
        output.push_str(&topology.cell_count().to_string());
        output.push_str(",\"emptyCellCountCandidate\":");
        output.push_str(&topology.empty_cell_count().to_string());
        output.push_str(",\"nonEmptyCellCountCandidate\":");
        output.push_str(&topology.non_empty_cell_count().to_string());
        output.push('}');
    } else {
        output.push_str(",\"sparseTopologySummary\":null");
    }
    output.push_str(
        ",\"renderPromotionContribution\":\"sparse-topology-source-interval-corroboration-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "sparse-topology-does-not-decode-page-space-geometry",
    ));
    output.push_str(",\"rows\":[");
    for (index, row) in evidence.rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"compactRow\":");
        output.push_str(&row.compact_row_index.to_string());
        output.push_str(",\"sparseRow\":");
        output.push_str(&row.sparse_row_index.to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&row.source_interval_index.to_string());
        output.push_str(",\"sourceRange\":");
        output.push_str(&source_range_json(row.source_start, row.source_end));
        output.push_str(",\"compactCellCount\":");
        output.push_str(&row.compact_cell_count.to_string());
        output.push_str(",\"sparseCellCount\":");
        output.push_str(&row.sparse_cell_count.to_string());
        output.push_str(",\"sparseEmptyCellCount\":");
        output.push_str(&row.sparse_empty_cell_count.to_string());
        output.push_str(",\"sparseNonEmptyCellCount\":");
        output.push_str(&row.sparse_non_empty_cell_count.to_string());
        output.push_str(",\"firstNonEmptySparseColumnIndex\":");
        push_option_usize_json(output, row.first_non_empty_sparse_column_index);
        output.push_str(",\"lastNonEmptySparseColumnIndex\":");
        push_option_usize_json(output, row.last_non_empty_sparse_column_index);
        output.push_str(",\"compactToSparseColumnOffset\":");
        push_option_usize_json(output, row.compact_to_sparse_column_offset);
        output.push_str(",\"matchedSegmentCount\":");
        output.push_str(&row.segments.len().to_string());
        output.push_str(",\"allCompactSegmentsMatched\":");
        output.push_str(if row.segments.len() == row.compact_cell_count {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"segments\":[");
        for (segment_index, segment) in row.segments.iter().enumerate() {
            if segment_index > 0 {
                output.push(',');
            }
            output.push_str("{\"compactColumn\":");
            output.push_str(&segment.compact_column_index.to_string());
            output.push_str(",\"sparseColumn\":");
            output.push_str(&segment.sparse_column_index.to_string());
            output.push_str(",\"sourceRange\":");
            output.push_str(&source_range_json(segment.source_start, segment.source_end));
            output.push_str(",\"textMatches\":");
            output.push_str(if segment.text_matches {
                "true"
            } else {
                "false"
            });
            output.push('}');
        }
        output.push_str("]}");
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_sparse_sibling_column_promotion_readiness_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(evidence) = table_grid_sparse_table_sibling_evidence(document, candidate) else {
        output.push_str("null");
        return;
    };
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let matched_row_count = evidence.rows.len();
    let candidate_row_count = candidate.intervals().len();
    let matched_segment_count = evidence
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    let candidate_segment_count = candidate.cell_count_candidate();
    let matched_sparse_column_indexes =
        table_grid_sparse_sibling_matched_sparse_column_indexes(&evidence.rows, column_count);
    let sparse_topology_complete = matched_row_count == candidate_row_count
        && matched_segment_count == candidate_segment_count
        && evidence.compact_to_sparse_column_offset.is_some()
        && matched_sparse_column_indexes.len() == column_count;
    let decoded_source_placement_required_cell_count =
        table_grid_decoded_source_placement_required_cell_count(candidate);
    let decoded_source_placement_match_count =
        table_grid_decoded_source_placement_match_count(document, candidate);
    let compact_line_header_cell_coverage_complete = decoded_source_placement_required_cell_count
        > 0
        && decoded_source_placement_match_count >= decoded_source_placement_required_cell_count;
    let source_column_widths =
        table_grid_line_header_column_widths_px(document, candidate, 1.0, column_count);
    let source_column_widths_present =
        source_column_widths.len() == column_count && !source_column_widths.is_empty();
    let column_split_ready = sparse_topology_complete
        && compact_line_header_cell_coverage_complete
        && source_column_widths_present;

    let mut blocked_reasons = Vec::new();
    if column_count == 0 {
        blocked_reasons.push("column-count-zero");
    }
    if !sparse_topology_complete {
        blocked_reasons.push("sparse-sibling-topology-incomplete");
    }
    if !compact_line_header_cell_coverage_complete {
        blocked_reasons.push("compact-line-header-cell-geometry-incomplete");
    }
    if !source_column_widths_present {
        blocked_reasons.push("source-line-header-column-widths-missing");
    }

    output.push_str("{\"source\":\"sparseTableSiblingEvidence+documentTextLineHeaders column promotion readiness\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"columnSplitReady\":");
    output.push_str(if column_split_ready { "true" } else { "false" });
    output.push_str(",\"requestedColumnCount\":");
    output.push_str(&column_count.to_string());
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate_row_count.to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&matched_row_count.to_string());
    output.push_str(",\"candidateSegmentCount\":");
    output.push_str(&candidate_segment_count.to_string());
    output.push_str(",\"matchedSegmentCount\":");
    output.push_str(&matched_segment_count.to_string());
    output.push_str(",\"sparseTopologyComplete\":");
    output.push_str(if sparse_topology_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"compactToSparseColumnOffsetCandidate\":");
    push_option_usize_json(output, evidence.compact_to_sparse_column_offset);
    output.push_str(",\"matchedSparseColumnIndexes\":");
    push_usize_array_json(output, &matched_sparse_column_indexes);
    output.push_str(",\"compactLineHeaderCellCoverageComplete\":");
    output.push_str(if compact_line_header_cell_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"decodedSourcePlacementMatchCount\":");
    output.push_str(&decoded_source_placement_match_count.to_string());
    output.push_str(",\"decodedSourcePlacementRequiredCellCount\":");
    output.push_str(&decoded_source_placement_required_cell_count.to_string());
    output.push_str(",\"sourceLineHeaderColumnWidthsPresent\":");
    output.push_str(if source_column_widths_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceColumnWidthBasis\":");
    if source_column_widths_present {
        output.push_str(&json_string("documentTextLineHeaderCellSlotUnits"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceColumnWidthFractions\":");
    push_f32_array_json(output, &source_column_widths);
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"sparse-sibling-column-readiness-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if column_split_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-column-split-not-ready"));
    }
    output.push('}');
}

pub(crate) fn push_table_grid_sparse_sibling_derived_compact_cell_geometry_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(evidence) = table_grid_sparse_table_sibling_evidence(document, candidate) else {
        output.push_str("null");
        return;
    };
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let candidate_row_count = candidate.intervals().len();
    let matched_row_count = evidence.rows.len();
    let required_cell_count = candidate.cell_count_candidate();
    let matched_segment_count = evidence
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    let matched_sparse_column_indexes =
        table_grid_sparse_sibling_matched_sparse_column_indexes(&evidence.rows, column_count);
    let derived_cell_geometry_coverage_complete = required_cell_count > 0
        && matched_row_count == candidate_row_count
        && matched_segment_count == required_cell_count
        && evidence.compact_to_sparse_column_offset.is_some()
        && matched_sparse_column_indexes.len() == column_count;

    output
        .push_str("{\"source\":\"sparseTableSiblingEvidence compact cell geometry prerequisite\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate_row_count.to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&matched_row_count.to_string());
    output.push_str(",\"candidateSegmentCount\":");
    output.push_str(&required_cell_count.to_string());
    output.push_str(",\"matchedSegmentCount\":");
    output.push_str(&matched_segment_count.to_string());
    output.push_str(",\"compactToSparseColumnOffsetCandidate\":");
    push_option_usize_json(output, evidence.compact_to_sparse_column_offset);
    output.push_str(",\"matchedSparseColumnIndexes\":");
    push_usize_array_json(output, &matched_sparse_column_indexes);
    output.push_str(",\"derivedMatchedCellCount\":");
    output.push_str(&matched_segment_count.to_string());
    output.push_str(",\"requiredCellCount\":");
    output.push_str(&required_cell_count.to_string());
    output.push_str(",\"derivedCellGeometryCoverageComplete\":");
    output.push_str(if derived_cell_geometry_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementPrerequisiteReady\":");
    output.push_str(if derived_cell_geometry_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(
        ",\"renderPromotionContribution\":\"sparse-sibling-derived-geometry-prerequisite\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "sparse-sibling-derived-geometry-diagnostic-only",
    ));
    output.push('}');
}

pub(crate) fn table_grid_sparse_table_sibling_evidence<'a>(
    document: &'a Document,
    candidate: &TableCandidate,
) -> Option<TableGridSparseSiblingEvidence<'a>> {
    if !candidate.is_document_text_control_run_candidate() || candidate.intervals().is_empty() {
        return None;
    }

    document
        .table_candidates()
        .iter()
        .filter(|sparse_candidate| {
            sparse_candidate.is_sparse_document_text_control_run_candidate()
                && sparse_candidate.basis() == candidate.basis()
                && sparse_candidate.delimiter_code() == candidate.delimiter_code()
                && sparse_candidate.source_start() <= candidate.source_start()
                && candidate.source_end() <= sparse_candidate.source_end()
        })
        .filter_map(|sparse_candidate| {
            table_grid_sparse_table_sibling_evidence_for_candidate(candidate, sparse_candidate)
        })
        .max_by_key(|evidence| {
            let matched_segment_count = evidence
                .rows
                .iter()
                .map(|row| row.segments.len())
                .sum::<usize>();
            (
                evidence.rows.len() == candidate.intervals().len(),
                matched_segment_count == candidate.cell_count_candidate(),
                evidence.compact_to_sparse_column_offset.is_some(),
                evidence.rows.len(),
                matched_segment_count,
            )
        })
}

pub(crate) fn table_grid_sparse_table_sibling_evidence_for_candidate<'a>(
    candidate: &TableCandidate,
    sparse_candidate: &'a TableCandidate,
) -> Option<TableGridSparseSiblingEvidence<'a>> {
    let mut rows = Vec::new();
    for compact_interval in candidate.intervals() {
        let sparse_interval = sparse_candidate
            .intervals()
            .iter()
            .find(|sparse_interval| {
                sparse_interval.source_interval_index() == compact_interval.source_interval_index()
                    && sparse_interval.source_start() == compact_interval.source_start()
                    && sparse_interval.source_end() == compact_interval.source_end()
            })?;
        rows.push(table_grid_sparse_sibling_row_match(
            compact_interval,
            sparse_interval,
        ));
    }
    if rows.is_empty() {
        return None;
    }
    let compact_to_sparse_column_offset = rows
        .iter()
        .map(|row| row.compact_to_sparse_column_offset)
        .try_fold(None, |seen, value| match (seen, value) {
            (None, Some(value)) => Some(Some(value)),
            (Some(previous), Some(value)) if previous == value => Some(Some(previous)),
            (Some(previous), None) => Some(Some(previous)),
            (None, None) => Some(None),
            _ => None,
        })?;

    Some(TableGridSparseSiblingEvidence {
        sparse_candidate,
        rows,
        compact_to_sparse_column_offset,
    })
}

pub(crate) fn table_grid_sparse_sibling_row_match(
    compact_interval: &TableCandidateInterval,
    sparse_interval: &TableCandidateInterval,
) -> TableGridSparseSiblingRowMatch {
    let mut segments = Vec::new();
    for compact_segment in compact_interval.column_segments() {
        let Some(source_start) = compact_segment.source_start() else {
            continue;
        };
        let Some(source_end) = compact_segment.source_end() else {
            continue;
        };
        let Some(sparse_segment) =
            sparse_interval
                .column_segments()
                .iter()
                .find(|sparse_segment| {
                    sparse_segment.source_start() == Some(source_start)
                        && sparse_segment.source_end() == Some(source_end)
                        && !sparse_segment.text().is_empty()
                })
        else {
            continue;
        };
        segments.push(TableGridSparseSiblingSegmentMatch {
            compact_column_index: compact_segment.index(),
            sparse_column_index: sparse_segment.index(),
            source_start,
            source_end,
            text_matches: compact_segment.text() == sparse_segment.text(),
        });
    }

    let sparse_empty_cell_count = sparse_interval
        .column_segments()
        .iter()
        .filter(|segment| segment.text().is_empty())
        .count();
    let sparse_non_empty_cell_count = sparse_interval
        .column_segments()
        .len()
        .saturating_sub(sparse_empty_cell_count);
    let first_non_empty_sparse_column_index = sparse_interval
        .column_segments()
        .iter()
        .find(|segment| !segment.text().is_empty())
        .map(TableCandidateColumnSegment::index);
    let last_non_empty_sparse_column_index = sparse_interval
        .column_segments()
        .iter()
        .rev()
        .find(|segment| !segment.text().is_empty())
        .map(TableCandidateColumnSegment::index);
    let compact_to_sparse_column_offset = table_grid_sparse_sibling_column_offset(&segments);

    TableGridSparseSiblingRowMatch {
        compact_row_index: compact_interval.index(),
        sparse_row_index: sparse_interval.index(),
        source_interval_index: compact_interval.source_interval_index(),
        source_start: compact_interval.source_start(),
        source_end: compact_interval.source_end(),
        compact_cell_count: compact_interval.column_segments().len(),
        sparse_cell_count: sparse_interval.column_segments().len(),
        sparse_empty_cell_count,
        sparse_non_empty_cell_count,
        first_non_empty_sparse_column_index,
        last_non_empty_sparse_column_index,
        compact_to_sparse_column_offset,
        segments,
    }
}

pub(crate) fn table_grid_sparse_sibling_column_offset(
    segments: &[TableGridSparseSiblingSegmentMatch],
) -> Option<usize> {
    let first = segments.first()?;
    let offset = first
        .sparse_column_index
        .checked_sub(first.compact_column_index)?;
    segments
        .iter()
        .all(|segment| {
            segment
                .sparse_column_index
                .checked_sub(segment.compact_column_index)
                == Some(offset)
        })
        .then_some(offset)
}

pub(crate) fn table_grid_sparse_sibling_matched_sparse_column_indexes(
    rows: &[TableGridSparseSiblingRowMatch],
    compact_column_count: usize,
) -> Vec<usize> {
    let Some(first_row) = rows.first() else {
        return Vec::new();
    };
    let columns = first_row
        .segments
        .iter()
        .take(compact_column_count)
        .map(|segment| segment.sparse_column_index)
        .collect::<Vec<_>>();
    if columns.len() != compact_column_count {
        return Vec::new();
    }
    let all_rows_agree = rows.iter().all(|row| {
        row.segments
            .iter()
            .take(compact_column_count)
            .map(|segment| segment.sparse_column_index)
            .eq(columns.iter().copied())
    });
    if all_rows_agree { columns } else { Vec::new() }
}

pub(crate) fn push_table_grid_line_header_line_mark_coupling_evidence_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    if rows.is_empty() || line_mark_intervals.is_empty() {
        output.push_str("null");
        return;
    }

    let mut row_matches = Vec::new();
    for row in &rows {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start);
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end);
        let line_mark = best_line_mark_interval_for_unit_range(
            &line_mark_intervals,
            row_unit_start,
            row_unit_end,
        );
        row_matches.push((row, row_unit_start, row_unit_end, line_mark));
    }

    let coupled_row_count = row_matches
        .iter()
        .filter(|(_, _, _, line_mark)| line_mark.is_some())
        .count();
    let exact_source_range_match_count = row_matches
        .iter()
        .filter(|(_, row_unit_start, row_unit_end, line_mark)| {
            line_mark.as_ref().is_some_and(|interval| {
                interval.unit_start == *row_unit_start && interval.unit_end == *row_unit_end
            })
        })
        .count();
    let line_header_record_containment_count = row_matches
        .iter()
        .filter(|(row, row_unit_start, row_unit_end, _)| {
            !row.headers.is_empty()
                && row.headers.iter().all(|header| {
                    let header_unit_start = header.start / 2;
                    let header_unit_end = header.end / 2;
                    *row_unit_start <= header_unit_start && header_unit_end <= *row_unit_end
                })
        })
        .count();
    let all_rows_coupled = coupled_row_count == rows.len();
    let all_rows_exact_source_range_matched = exact_source_range_match_count == rows.len();
    let matched_record_indexes = row_matches
        .iter()
        .filter_map(|(_, _, _, line_mark)| line_mark.map(|interval| interval.record_index))
        .collect::<Vec<_>>();
    let contiguous_line_mark_records = matched_record_indexes
        .windows(2)
        .all(|pair| pair[1] == pair[0] + 1);
    let rows_homogeneous = rows.first().is_some_and(|first| {
        rows.iter().all(|row| {
            row.headers.len() == first.headers.len()
                && row.headers.iter().zip(&first.headers).all(|(left, right)| {
                    left.offset_units == right.offset_units
                        && left.extent_units == right.extent_units
                        && left.font_size_units == right.font_size_units
                })
        })
    });

    output.push_str("{\"source\":\"/DocumentText+/LineMark\"");
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"lineHeaderRowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"lineMarkIntervalCount\":");
    output.push_str(&line_mark_intervals.len().to_string());
    output.push_str(",\"coupledRowCount\":");
    output.push_str(&coupled_row_count.to_string());
    output.push_str(",\"exactSourceRangeMatchCount\":");
    output.push_str(&exact_source_range_match_count.to_string());
    output.push_str(",\"lineHeaderRecordContainmentCount\":");
    output.push_str(&line_header_record_containment_count.to_string());
    output.push_str(",\"allRowsCoupled\":");
    output.push_str(if all_rows_coupled { "true" } else { "false" });
    output.push_str(",\"allRowsExactSourceRangeMatched\":");
    output.push_str(if all_rows_exact_source_range_matched {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"contiguousLineMarkRecords\":");
    output.push_str(if contiguous_line_mark_records {
        "true"
    } else {
        "false"
    });
    push_line_mark_record_stride_fields_json(output, &matched_record_indexes);
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if rows_homogeneous { "true" } else { "false" });
    output.push_str(",\"lineMarkRecordRange\":");
    match (
        matched_record_indexes.first().copied(),
        matched_record_indexes.last().copied(),
    ) {
        (Some(start), Some(end)) => {
            output.push_str(&source_range_json(start, end.saturating_add(1)));
        }
        _ => output.push_str("null"),
    }
    output.push_str(
        ",\"renderPromotionContribution\":\"row-boundary-line-header-coupling-evidence-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-units-not-page-y-coordinate-transform",
    ));
    output.push_str(",\"rows\":[");
    for (index, (row, row_unit_start, row_unit_end, line_mark)) in row_matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row.row_index.to_string());
        output.push_str(",\"rowSourceUnitRange\":");
        output.push_str(&source_range_json(*row_unit_start, *row_unit_end));
        output.push_str(",\"lineHeaderCount\":");
        output.push_str(&row.headers.len().to_string());
        output.push_str(",\"matchedCellHeaderCount\":");
        output.push_str(&row.matched_cell_count.to_string());
        output.push_str(",\"lineMarkRecordIndex\":");
        match line_mark {
            Some(interval) => output.push_str(&interval.record_index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"lineMarkUnitRange\":");
        match line_mark {
            Some(interval) => {
                output.push_str(&source_range_json(interval.unit_start, interval.unit_end));
            }
            None => output.push_str("null"),
        }
        output.push_str(",\"exactSourceRangeMatch\":");
        output.push_str(
            if line_mark.as_ref().is_some_and(|interval| {
                interval.unit_start == *row_unit_start && interval.unit_end == *row_unit_end
            }) {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"lineHeaderRecordsContained\":");
        output.push_str(
            if !row.headers.is_empty()
                && row.headers.iter().all(|header| {
                    let header_unit_start = header.start / 2;
                    let header_unit_end = header.end / 2;
                    *row_unit_start <= header_unit_start && header_unit_end <= *row_unit_end
                })
            {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"lineHeaderRecordUnitRanges\":[");
        for (header_index, header) in row.headers.iter().enumerate() {
            if header_index > 0 {
                output.push(',');
            }
            output.push_str(&source_range_json(header.start / 2, header.end / 2));
        }
        output.push_str("]}");
    }
    output.push_str("]}");
}
