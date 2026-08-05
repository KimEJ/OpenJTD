use super::*;
use crate::*;

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
