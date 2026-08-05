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
