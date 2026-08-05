use super::*;
use crate::*;

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
