use super::*;
use crate::*;

pub(crate) fn push_page_layer_table_grid_candidate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    overlay_index: usize,
    candidate: &TableCandidate,
    grid: &TableCandidateColumnGridCandidate,
) {
    let reference_layout =
        reference_table_grid_overlay_layout(layout, document, candidate, grid.column_count());
    let source_layout = table_grid_source_derived_layout_candidate(
        layout,
        document,
        lines,
        overlay_index,
        candidate,
        grid.column_count(),
    );
    let source_render_layout = source_layout
        .as_ref()
        .filter(|layout| table_grid_source_derived_layout_is_renderable(layout))
        .map(TableGridRenderLayout::from_source_derived);
    let source_render_layout_present = source_render_layout.is_some();
    let render_layout = source_render_layout.or_else(|| {
        reference_layout
            .as_ref()
            .map(TableGridRenderLayout::from_reference)
    });
    let reference_projection = render_layout
        .as_ref()
        .is_some_and(|layout| layout.reference_backed);
    let reference_fallback_admission =
        table_grid_reference_layout_visible_fallback_admission(document, candidate);
    let (
        x,
        y,
        width,
        row_height,
        column_width,
        render_column_count,
        column_widths,
        column_width_basis,
    ) = if let Some(render_layout) = render_layout.as_ref() {
        (
            render_layout.x,
            render_layout.y,
            render_layout.width,
            render_layout.row_height,
            render_layout.column_width,
            render_layout.column_count,
            render_layout.column_widths.clone(),
            render_layout.column_width_basis,
        )
    } else {
        let (x, y, width, row_height, column_width) = table_grid_overlay_layout(
            layout,
            document,
            lines,
            overlay_index,
            candidate,
            grid.column_count(),
        );
        (
            x,
            y,
            width,
            row_height,
            column_width,
            grid.column_count(),
            Vec::new(),
            "equalColumns",
        )
    };
    let height = row_height * grid.row_count() as f32;
    let projection_kind = if render_layout.is_some() {
        "tableProjection"
    } else {
        table_grid_projection_kind(false)
    };
    output.push_str("{\"type\":\"tableGridCandidate\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"tableCandidate\",\"projectionKind\":");
    output.push_str(&json_string(projection_kind));
    output.push_str(",\"referenceBacked\":");
    output.push_str(if reference_projection {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&grid.row_count().to_string());
    output.push_str(",\"colCountCandidate\":");
    output.push_str(&render_column_count.to_string());
    output.push_str(",\"cellCountCandidate\":");
    output.push_str(&(grid.row_count() * render_column_count).to_string());
    output.push_str(",\"emptyCellCountCandidate\":");
    output.push_str(&candidate.empty_cell_count_candidate().to_string());
    output.push_str(",\"nonEmptyCellCountCandidate\":");
    output.push_str(&candidate.non_empty_cell_count_candidate().to_string());
    output.push_str(",\"columnWidth\":");
    output.push_str(&format!("{column_width:.3}"));
    output.push_str(",\"columnWidthBasis\":");
    output.push_str(&json_string(column_width_basis));
    output.push_str(",\"columnWidths\":[");
    for (index, value) in column_widths.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{value:.3}"));
    }
    output.push(']');
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{row_height:.3}"));
    if let Some(render_layout) = render_layout.as_ref() {
        output.push_str(",\"strokeWidth\":");
        output.push_str(&format!("{:.3}", render_layout.stroke_width));
        output.push_str(",\"cellStrokeWidth\":");
        output.push_str(&format!("{:.3}", render_layout.cell_stroke_width));
        output.push_str(",\"strokeWidthBasis\":");
        output.push_str(&json_string(render_layout.stroke_width_basis));
        output.push_str(",\"cellTextCentered\":");
        output.push_str(if render_layout.cell_text_centered {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"cellTextAlignmentBasis\":");
        output.push_str(&json_string(render_layout.cell_text_alignment_basis));
        output.push_str(",\"cellTextXAdjustment\":");
        output.push_str(&format!("{:.3}", render_layout.cell_text_x_adjustment));
        output.push_str(",\"cellTextXAdjustmentBasis\":");
        output.push_str(&json_string(render_layout.cell_text_x_adjustment_basis));
        output.push_str(",\"cellTextBaselineFactor\":");
        output.push_str(&format!("{:.3}", render_layout.cell_text_baseline_factor));
        output.push_str(",\"cellTextBaselineBasis\":");
        output.push_str(&json_string(render_layout.cell_text_baseline_basis));
        output.push_str(",\"cellTextFontWeight\":");
        output.push_str(&json_string(render_layout.cell_text_font_weight));
        output.push_str(",\"cellTextFontWeightBasis\":");
        output.push_str(&json_string(render_layout.cell_text_font_weight_basis));
        output.push_str(",\"cellTextFontSize\":");
        output.push_str(&format!("{:.3}", render_layout.font_size));
        output.push_str(",\"cellTextFontSizeBasis\":");
        output.push_str(&json_string(render_layout.font_size_basis));
    }
    output.push_str(",\"sourceAnchorEvidence\":");
    push_table_grid_source_anchor_evidence_json(output, candidate);
    output.push_str(",\"geometryDerivationEvidence\":");
    push_table_grid_geometry_derivation_evidence_json(output, layout, document, lines, candidate);
    output.push_str(",\"sourceDerivedLayoutCandidate\":");
    push_table_grid_source_derived_layout_candidate_json(
        output,
        layout,
        document,
        candidate,
        source_layout.as_ref(),
    );
    output.push_str(",\"sourceDerivedLayoutReadiness\":");
    push_table_grid_source_derived_layout_readiness_json(
        output,
        document,
        candidate,
        grid.column_count(),
        source_layout.as_ref(),
    );
    output.push_str(",\"referenceFallbackAdmissionGate\":");
    push_table_grid_reference_fallback_admission_gate_json(
        output,
        reference_layout.is_some(),
        reference_projection,
        source_layout.as_ref(),
        source_render_layout_present,
        &reference_fallback_admission,
    );
    output.push_str(",\"pageSpaceSolver\":");
    push_table_grid_page_space_solver_json(
        output,
        layout,
        document,
        lines,
        candidate,
        grid.column_count(),
        source_layout.as_ref(),
    );
    output.push_str(",\"cells\":[");
    let mut first_cell = true;
    let line_header_rows = table_candidate_document_text_line_header_rows(document, candidate);
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    let page_mark = document.page_marks().first();
    let document_text_map = document_text_raw_stream(document).map(map_document_text);
    for (row_index, interval) in candidate.intervals().iter().enumerate() {
        let row_y = y + row_index as f32 * row_height;
        for (column_index, segment) in interval.column_segments().iter().enumerate() {
            let column_index =
                table_grid_segment_column_index(document, candidate, interval, column_index);
            if column_index >= render_column_count {
                break;
            }
            if !first_cell {
                output.push(',');
            }
            first_cell = false;
            let cell_column_width =
                table_grid_column_width(column_width, &column_widths, column_index);
            let column_x = table_grid_column_x(x, column_width, &column_widths, column_index);
            output.push_str("{\"row\":");
            output.push_str(&row_index.to_string());
            output.push_str(",\"col\":");
            output.push_str(&column_index.to_string());
            output.push_str(",\"bbox\":");
            output.push_str(&format!(
                "{{\"x\":{column_x:.3},\"y\":{row_y:.3},\"width\":{cell_column_width:.3},\"height\":{row_height:.3}}}"
            ));
            output.push_str(",\"text\":");
            output.push_str(&json_string(segment.text()));
            let render_text =
                table_grid_cell_render_text(document_text_map.as_ref(), candidate, segment);
            output.push_str(",\"renderText\":");
            output.push_str(&json_string(&render_text.text));
            output.push_str(",\"renderTextBasis\":");
            output.push_str(&json_string(render_text.basis));
            output.push_str(",\"preservesSourceWhitespace\":");
            output.push_str(if render_text.preserves_source_whitespace {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"whitespacePlacementProbe\":");
            push_table_grid_cell_whitespace_placement_probe_json(
                output,
                &render_text,
                render_layout
                    .as_ref()
                    .is_some_and(|layout| layout.cell_text_centered),
            );
            output.push_str(",\"sourceRange\":");
            push_table_grid_segment_source_range_json(output, candidate, segment);
            output.push_str(",\"sourceEvidence\":");
            push_table_grid_cell_source_evidence_json(
                output,
                candidate,
                interval,
                segment,
                &line_header_rows,
                &line_mark_intervals,
                page_mark,
            );
            output.push('}');
        }
    }
    output.push(']');
    output.push_str(",\"pattern\":[");
    for (index, kind) in grid.pattern().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(kind.as_str()));
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_source_anchor_evidence_json(
    output: &mut String,
    candidate: &TableCandidate,
) {
    output.push_str("{\"source\":\"tableCandidateColumnSegments\",\"basis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"cellSourceRangeCount\":");
    output.push_str(&table_candidate_source_anchor_count(candidate).to_string());
    output.push_str(",\"placementDerived\":false,\"geometryDecoded\":false,\"decoded\":false}");
}

pub(crate) fn push_table_grid_geometry_derivation_evidence_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) {
    let fallback_anchor_count = table_grid_fallback_text_anchor_count(document, lines, candidate);
    let layout_box_present = raw_stream_bytes(document, LAYOUT_BOX_PATH).is_some();
    let layout_box_text_present = raw_stream_bytes(document, LAYOUT_BOX_TEXT_PATH).is_some();
    let layout_box_text_position_tables_present =
        raw_stream_bytes(document, LAYOUT_BOX_TEXT_POSITION_TABLES_PATH).is_some();
    let text_count_range_count = document.text_count_ranges().len();
    let decoded_source_placement_match_count =
        table_grid_decoded_source_placement_match_count(document, candidate);
    let decoded_source_placement_required_cell_count =
        table_grid_decoded_source_placement_required_cell_count(candidate);
    let decoded_source_placement_evidence_present =
        table_grid_decoded_source_placement_evidence_present(document, candidate);
    let source_layout_evidence_present = table_grid_source_layout_evidence_present(document)
        || decoded_source_placement_evidence_present;
    let placement_authority = if decoded_source_placement_evidence_present {
        "documentTextLineHeaders"
    } else if fallback_anchor_count > 0 {
        "fallbackAnchors"
    } else {
        "none"
    };
    output.push_str("{\"source\":\"tableCandidateGeometryProbe\"");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"candidateSourceRange\":");
    output.push_str(&source_range_json(
        candidate.source_start(),
        candidate.source_end(),
    ));
    output.push_str(",\"cellSourceRangeCount\":");
    output.push_str(&table_candidate_source_anchor_count(candidate).to_string());
    output.push_str(",\"fallbackTextRunAnchorCount\":");
    output.push_str(&fallback_anchor_count.to_string());
    output.push_str(",\"fallbackTextRunProjectionKind\":\"fallback\"");
    output.push_str(",\"textCountRangeCount\":");
    output.push_str(&text_count_range_count.to_string());
    output.push_str(",\"layoutBoxPresent\":");
    output.push_str(if layout_box_present { "true" } else { "false" });
    output.push_str(",\"layoutBoxTextPresent\":");
    output.push_str(if layout_box_text_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"layoutBoxTextPositionTablesPresent\":");
    output.push_str(if layout_box_text_position_tables_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutEvidencePresent\":");
    output.push_str(if source_layout_evidence_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"placementAuthority\":");
    output.push_str(&json_string(placement_authority));
    output.push_str(",\"decodedSourcePlacementEvidence\":");
    output.push_str(if decoded_source_placement_evidence_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"decodedSourcePlacementMatchCount\":");
    output.push_str(&decoded_source_placement_match_count.to_string());
    output.push_str(",\"decodedSourcePlacementRequiredCellCount\":");
    output.push_str(&decoded_source_placement_required_cell_count.to_string());
    output.push_str(",\"documentTextLineHeaderEvidence\":");
    push_table_grid_document_text_line_header_evidence_json(output, document, candidate);
    output.push_str(",\"sparseTableSiblingEvidence\":");
    push_table_grid_sparse_table_sibling_evidence_json(output, document, candidate);
    output.push_str(",\"sparseSiblingColumnPromotionReadiness\":");
    push_table_grid_sparse_sibling_column_promotion_readiness_json(output, document, candidate);
    output.push_str(",\"sparseSiblingDerivedCompactCellGeometry\":");
    push_table_grid_sparse_sibling_derived_compact_cell_geometry_json(output, document, candidate);
    output.push_str(",\"sparseSiblingLineMarkYComparison\":");
    push_table_grid_sparse_sibling_line_mark_y_comparison_json(output, layout, document, candidate);
    output.push_str(",\"pageMarkRawRecordScanEvidence\":");
    push_table_grid_page_mark_raw_record_scan_evidence_json(output, document, candidate);
    output.push_str(",\"pageMarkRawRecordSourceRangeEvidence\":");
    push_table_grid_page_mark_raw_record_source_range_evidence_json(output, document, candidate);
    output.push_str(",\"crossTableSubrecordOrderingProbe\":");
    let cross_table_subrecord_ordering_probe =
        table_grid_cross_table_subrecord_ordering_probe(document, candidate);
    push_table_grid_cross_table_subrecord_ordering_probe_json(
        output,
        cross_table_subrecord_ordering_probe.as_ref(),
    );
    output.push_str(",\"pageMarkRawReferenceValueProbe\":");
    push_table_grid_page_mark_raw_reference_value_probe_json(output, layout, document, candidate);
    output.push_str(",\"sourceDerivedHorizontalFieldAdjustmentProbe\":");
    push_table_grid_source_derived_horizontal_field_adjustment_probe_json(
        output, layout, document, lines, candidate,
    );
    output.push_str(",\"sourceOnlyHorizontalFieldConsensus\":");
    push_table_grid_source_only_horizontal_field_consensus_json(
        output, layout, document, lines, candidate,
    );
    output.push_str(",\"pageMarkScopedYTransformProbe\":");
    push_table_grid_page_mark_scoped_y_transform_probe_json(output, layout, document, candidate);
    output.push_str(",\"lineHeaderLineMarkCouplingEvidence\":");
    push_table_grid_line_header_line_mark_coupling_evidence_json(output, document, candidate);
    output.push_str(",\"referenceBBoxResidualEvidence\":");
    push_table_grid_reference_bbox_residual_evidence_json(
        output, layout, document, lines, candidate,
    );
    output.push_str(",\"topTextAnchorEvidence\":");
    push_table_grid_top_text_anchor_evidence_json(output, layout, document, candidate);
    output.push_str(",\"topTextTableSourceGapEvidence\":");
    push_table_grid_top_text_table_source_gap_evidence_json(output, layout, document, candidate);
    output.push_str(",\"topTextAnchorResidualEvidence\":");
    push_table_grid_top_text_anchor_residual_evidence_json(output, layout, document, candidate);
    output.push_str(",\"independentPageTransformEvidence\":{\"present\":false,\"blockedReason\":\"top-text-page-coordinates-and-table-bbox-are-reference-backed\"}");
    output.push_str(",\"layoutStreamProbe\":");
    push_table_grid_layout_stream_probe_json(output, document, candidate);
    output.push_str(",\"renderPromotionBlockedReason\":");
    if decoded_source_placement_evidence_present {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-layout-position-evidence-missing"));
    }
    output.push_str(",\"placementDerived\":false,\"geometryDecoded\":false,\"decoded\":false}");
}

pub(crate) fn push_table_grid_top_text_anchor_evidence_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(slots) = success_data_test_resolved_top_text_projection(document, 1) else {
        output.push_str("null");
        return;
    };
    if slots.is_empty() {
        output.push_str("null");
        return;
    }
    let source_backed_run_count = slots
        .iter()
        .filter(|slot| slot.source_span.is_some())
        .count();
    let line_header_backed_run_count = slots
        .iter()
        .filter(|slot| slot.line_header.is_some())
        .count();
    let line_mark_backed_run_count = slots
        .iter()
        .filter(|slot| {
            slot.source_span.as_ref().is_some_and(|span| {
                success_data_test_line_mark_matches_for_source_span(document, span)
                    .next()
                    .is_some()
            })
        })
        .count();
    output.push_str("{\"source\":");
    output.push_str(&json_string(DOCUMENT_TEXT_PATH));
    output.push_str(",\"present\":true");
    output.push_str(",\"referenceBackedCoordinateCount\":");
    output.push_str(&slots.len().to_string());
    output.push_str(",\"sourceBackedRunCount\":");
    output.push_str(&source_backed_run_count.to_string());
    output.push_str(",\"lineHeaderBackedRunCount\":");
    output.push_str(&line_header_backed_run_count.to_string());
    output.push_str(",\"lineMarkBackedRunCount\":");
    output.push_str(&line_mark_backed_run_count.to_string());
    output.push_str(",\"directTableTransformDerived\":false");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "top-text-anchors-reference-backed-not-independent-page-transform",
    ));
    output.push_str(",\"referenceCoordinateProbe\":");
    push_table_grid_top_text_reference_coordinate_probe_json(
        output, layout, document, candidate, &slots,
    );
    output.push_str(",\"lineHeaderAnchors\":[");
    let mut first = true;
    for slot in slots.iter().filter(|slot| slot.line_header.is_some()) {
        if !first {
            output.push(',');
        }
        first = false;
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
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_top_text_table_source_gap_evidence_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let Some(slots) = success_data_test_resolved_top_text_projection(document, 1) else {
        output.push_str("null");
        return;
    };
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let Some(first_row) = rows.first() else {
        output.push_str("null");
        return;
    };
    let table_unit_start =
        table_source_offset_to_units(candidate.basis(), candidate.source_start());
    let table_unit_end = table_source_offset_to_units(candidate.basis(), candidate.source_end());
    let first_row_unit_start =
        table_source_offset_to_units(candidate.basis(), first_row.source_start);
    let first_row_unit_end = table_source_offset_to_units(candidate.basis(), first_row.source_end);
    let table_min_offset_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.offset_units))
        .min();
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
    let Some(anchor) = slots
        .iter()
        .filter_map(|slot| {
            let span = slot.source_span.as_ref()?;
            let header = slot.line_header?;
            if span.unit_end() > table_unit_start {
                return None;
            }
            let full_width = Some(header.offset_units) == table_min_offset_units
                && Some(header.extent_units) == table_max_extent_units;
            if !full_width {
                return None;
            }
            Some((slot, span, header, table_unit_start - span.unit_end()))
        })
        .min_by_key(|(_, _, _, gap)| *gap)
    else {
        output.push_str("null");
        return;
    };
    let (slot, span, header, source_gap_after_anchor_text_units) = anchor;
    let first_row_header = first_row.headers.first();
    let line_mark_record_indexes =
        table_grid_line_mark_record_indexes_for_rows(document, candidate);

    output.push_str("{\"source\":\"/DocumentText\"");
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"anchorSelection\":\"nearest-preceding-full-width-line-header\"");
    output.push_str(",\"anchorRole\":");
    output.push_str(&json_string(slot.role));
    output.push_str(",\"anchorText\":");
    output.push_str(&json_string(slot.text));
    output.push_str(",\"anchorSourceUnitRange\":");
    output.push_str(&source_range_json(span.unit_start(), span.unit_end()));
    output.push_str(",\"anchorLineHeaderUnitRange\":");
    output.push_str(&source_range_json(header.start / 2, header.end / 2));
    output.push_str(",\"anchorOffsetUnits\":");
    output.push_str(&header.offset_units.to_string());
    output.push_str(",\"anchorExtentUnits\":");
    output.push_str(&header.extent_units.to_string());
    output.push_str(",\"anchorFontSizeUnits\":");
    output.push_str(&header.font_size_units.to_string());
    output.push_str(",\"sharedFullExtentWithTable\":");
    output.push_str(
        if Some(header.offset_units) == table_min_offset_units
            && Some(header.extent_units) == table_max_extent_units
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sharedFontSizeWithTable\":");
    output.push_str(if Some(header.font_size_units) == table_font_size_units {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceTablePlacementCoherenceGate\":");
    let source_top_text_placement_readiness =
        push_table_grid_source_table_placement_coherence_gate_json(
            output,
            TableGridSourceTablePlacementCoherenceInput {
                layout,
                document,
                candidate,
                rows: &rows,
                anchor_span: span,
                anchor_header: header,
                table_min_offset_units,
                table_max_extent_units,
                table_font_size_units,
                source_gap_after_anchor_text_units,
            },
        );
    output.push_str(",\"tableCandidateUnitRange\":");
    output.push_str(&source_range_json(table_unit_start, table_unit_end));
    output.push_str(",\"firstRowUnitRange\":");
    output.push_str(&source_range_json(first_row_unit_start, first_row_unit_end));
    output.push_str(",\"firstRowLineHeaderUnitRange\":");
    match first_row_header {
        Some(header) => output.push_str(&source_range_json(header.start / 2, header.end / 2)),
        None => output.push_str("null"),
    }
    output.push_str(",\"firstRowLineMarkRecordIndex\":");
    match line_mark_record_indexes.first() {
        Some(index) => output.push_str(&index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceGapAfterAnchorTextUnits\":");
    output.push_str(&source_gap_after_anchor_text_units.to_string());
    output.push_str(",\"sourceGapAfterAnchorLineHeaderUnits\":");
    output.push_str(&table_unit_start.saturating_sub(header.end / 2).to_string());
    output.push_str(",\"firstRowHeaderGapAfterAnchorTextUnits\":");
    match first_row_header {
        Some(first_header) => {
            output.push_str(&((first_header.start / 2).saturating_sub(span.unit_end())).to_string())
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"firstRowHeaderGapAfterTableStartUnits\":");
    match first_row_header {
        Some(first_header) => output
            .push_str(&((first_header.start / 2).saturating_sub(table_unit_start)).to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceTopTextPlacementReady\":");
    output.push_str(if source_top_text_placement_readiness.ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"readinessBlockedReasons\":");
    push_json_string_slice_array(output, &source_top_text_placement_readiness.blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":");
    if source_top_text_placement_readiness.ready {
        output.push_str(&json_string("source-top-text-placement-readiness"));
    } else {
        output.push_str(&json_string(
            "prompt-to-table-source-adjacency-evidence-only",
        ));
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    match source_top_text_placement_readiness.blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(crate) fn table_grid_source_top_text_placement_readiness_for_candidate(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) -> Option<TableGridSourceTopTextPlacementReadiness> {
    let slots = success_data_test_resolved_top_text_projection(document, 1)?;
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    rows.first()?;
    let table_unit_start =
        table_source_offset_to_units(candidate.basis(), candidate.source_start());
    let table_min_offset_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.offset_units))
        .min();
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
    let (_, span, header, source_gap_after_anchor_text_units) = slots
        .iter()
        .filter_map(|slot| {
            let span = slot.source_span.as_ref()?;
            let header = slot.line_header?;
            if span.unit_end() > table_unit_start {
                return None;
            }
            let full_width = Some(header.offset_units) == table_min_offset_units
                && Some(header.extent_units) == table_max_extent_units;
            full_width.then_some((slot, span, header, table_unit_start - span.unit_end()))
        })
        .min_by_key(|(_, _, _, gap)| *gap)?;
    let mut scratch = String::new();
    Some(push_table_grid_source_table_placement_coherence_gate_json(
        &mut scratch,
        TableGridSourceTablePlacementCoherenceInput {
            layout,
            document,
            candidate,
            rows: &rows,
            anchor_span: span,
            anchor_header: header,
            table_min_offset_units,
            table_max_extent_units,
            table_font_size_units,
            source_gap_after_anchor_text_units,
        },
    ))
}

pub(crate) fn push_table_grid_source_table_placement_coherence_gate_json(
    output: &mut String,
    input: TableGridSourceTablePlacementCoherenceInput<'_>,
) -> TableGridSourceTopTextPlacementReadiness {
    let TableGridSourceTablePlacementCoherenceInput {
        layout,
        document,
        candidate,
        rows,
        anchor_span,
        anchor_header,
        table_min_offset_units,
        table_max_extent_units,
        table_font_size_units,
        source_gap_after_anchor_text_units,
    } = input;
    let first_row = rows.first();
    let matched_column_count = rows
        .iter()
        .map(|row| row.matched_cell_count)
        .min()
        .unwrap_or(0);
    let line_header_rows_homogeneous = table_grid_line_header_rows_are_homogeneous(rows);
    let line_mark_rows_exact_and_contiguous =
        table_grid_line_mark_rows_are_exact_and_contiguous(document, candidate, rows);
    let candidate_basis = if line_mark_rows_exact_and_contiguous {
        TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader
    } else {
        TableGridUnitBBoxBasis::MatchedCells
    };
    let selected_range = first_row.and_then(|row| {
        table_grid_unit_bbox_range_for_row(row, matched_column_count, candidate_basis)
    });
    let (row_agreement_count, all_rows_agree) = selected_range
        .map(|range| {
            table_grid_unit_bbox_row_agreement_summary(
                rows,
                matched_column_count,
                candidate_basis,
                range,
            )
        })
        .unwrap_or((0, false));
    let first_trailing_header =
        first_row.and_then(|row| row.headers.get(matched_column_count).copied());
    let second_trailing_header =
        first_row.and_then(|row| row.headers.get(matched_column_count + 1).copied());
    let last_matched_header = first_row.and_then(|row| {
        row.headers
            .get(matched_column_count.saturating_sub(1))
            .copied()
    });
    let selected_width_units = selected_range.map(|(start, end)| end.saturating_sub(start));
    let full_extent_units = match (table_min_offset_units, table_max_extent_units) {
        (Some(start), Some(end)) => Some(end.saturating_sub(start)),
        _ => None,
    };
    let full_extent_trailing_after_selected_units = match (selected_range, table_max_extent_units) {
        (Some((_, selected_end)), Some(full_end)) => Some(full_end.saturating_sub(selected_end)),
        _ => None,
    };
    let first_trailing_gap_after_matched_units = match (last_matched_header, first_trailing_header)
    {
        (Some(last), Some(first)) => Some(first.offset_units.saturating_sub(last.extent_units)),
        _ => None,
    };
    let second_trailing_gap_after_first_trailing_units =
        match (first_trailing_header, second_trailing_header) {
            (Some(first), Some(second)) => {
                Some(second.offset_units.saturating_sub(first.extent_units))
            }
            _ => None,
        };
    let visible_range_uses_first_trailing_header =
        matches!(
            candidate_basis,
            TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader
        ) && match (selected_range, first_trailing_header) {
            (Some((_, selected_end)), Some(first)) => selected_end == first.extent_units,
            _ => false,
        };
    let full_extent_includes_second_trailing_header =
        match (second_trailing_header, table_max_extent_units) {
            (Some(second), Some(max_extent)) => second.extent_units == max_extent,
            _ => false,
        };
    let shared_full_extent_with_table = Some(anchor_header.offset_units) == table_min_offset_units
        && Some(anchor_header.extent_units) == table_max_extent_units;
    let shared_font_size_with_table = Some(anchor_header.font_size_units) == table_font_size_units;
    let coherent_with_top_text_anchor = source_gap_after_anchor_text_units <= 4
        && shared_full_extent_with_table
        && shared_font_size_with_table;
    let selected_closes_at_first_trailing_header = match (selected_range, first_trailing_header) {
        (Some((_, selected_end)), Some(first)) => selected_end == first.extent_units,
        _ => false,
    };
    let second_trailing_is_full_line_remainder = match (
        selected_range,
        second_trailing_header,
        table_max_extent_units,
    ) {
        (Some((_, selected_end)), Some(second), Some(full_end)) => {
            second.offset_units > selected_end && second.extent_units == full_end
        }
        _ => false,
    };
    let trailing_headers_coherent = first_trailing_header.is_some()
        && second_trailing_header.is_some()
        && visible_range_uses_first_trailing_header
        && full_extent_includes_second_trailing_header
        && first_trailing_gap_after_matched_units.is_some_and(|gap| gap > 0)
        && second_trailing_gap_after_first_trailing_units.is_some_and(|gap| gap > 0);
    let anchor_source_grid = success_data_test_source_text_placement_candidate(
        document,
        layout,
        Some(anchor_span),
        SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
    );
    let table_page_origin = table_font_size_units.and_then(|font_size_units| {
        table_grid_line_mark_page_origin_candidate(
            layout,
            document,
            candidate,
            rows,
            f32::from(font_size_units) * 1.75,
        )
    });
    let first_table_line_mark_record_index =
        table_grid_line_mark_record_indexes_for_rows(document, candidate)
            .first()
            .copied();
    let line_mark_record_gap_after_anchor =
        match (&anchor_source_grid, first_table_line_mark_record_index) {
            (Some(anchor), Some(first_table_record_index)) => Some(signed_usize_delta_i32(
                first_table_record_index,
                anchor.line_grid.record_index,
            )),
            _ => None,
        };
    let same_page_mark_entry = match (&anchor_source_grid, &table_page_origin) {
        (Some(anchor), Some(table_origin)) => {
            anchor.line_grid.page_mark_entry_index == table_origin.page_mark_entry_index
        }
        _ => false,
    };
    let source_page_grid_coupling_ready = anchor_source_grid.is_some()
        && table_page_origin.is_some()
        && same_page_mark_entry
        && line_mark_record_gap_after_anchor == Some(1);

    let mut blocked_reasons = Vec::new();
    if !coherent_with_top_text_anchor {
        blocked_reasons.push("top-text-anchor-not-coherent-with-table-full-extent");
    }
    if !trailing_headers_coherent {
        blocked_reasons.push("trailing-header-width-semantics-not-coherent");
    }
    if !all_rows_agree {
        blocked_reasons.push("table-width-candidate-not-row-stable");
    }
    if !line_header_rows_homogeneous {
        blocked_reasons.push("line-header-rows-not-homogeneous");
    }
    if !line_mark_rows_exact_and_contiguous {
        blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
    }
    let source_placement_coherence_ready = blocked_reasons.is_empty();
    let visible_width_semantics_ready = source_placement_coherence_ready
        && selected_closes_at_first_trailing_header
        && second_trailing_is_full_line_remainder
        && all_rows_agree
        && line_header_rows_homogeneous
        && line_mark_rows_exact_and_contiguous;
    let mut readiness_blocked_reasons = Vec::new();
    if !source_placement_coherence_ready {
        readiness_blocked_reasons.push("source-placement-coherence-unproven");
    }
    if !visible_width_semantics_ready {
        readiness_blocked_reasons.push("source-total-width-semantics-unproven");
    }
    if !source_page_grid_coupling_ready {
        readiness_blocked_reasons.push("page-space-y-origin-unproven");
    }
    let source_top_text_placement_ready = source_placement_coherence_ready
        && visible_width_semantics_ready
        && source_page_grid_coupling_ready
        && readiness_blocked_reasons.is_empty();

    output.push_str("{\"source\":\"topTextLineHeaders+documentTextTableLineHeaders\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateSourceUnitRange\":");
    output.push_str(&source_range_json(
        table_source_offset_to_units(candidate.basis(), candidate.source_start()),
        table_source_offset_to_units(candidate.basis(), candidate.source_end()),
    ));
    output.push_str(",\"sourceGapAfterAnchorTextUnits\":");
    output.push_str(&source_gap_after_anchor_text_units.to_string());
    output.push_str(",\"coherentWithTopTextAnchor\":");
    output.push_str(if coherent_with_top_text_anchor {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sharedFullExtentWithTable\":");
    output.push_str(if shared_full_extent_with_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sharedFontSizeWithTable\":");
    output.push_str(if shared_font_size_with_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateWidthBasis\":");
    output.push_str(&json_string(candidate_basis.as_str()));
    output.push_str(",\"selectedXUnitRange\":");
    match selected_range {
        Some((start, end)) => {
            output.push_str(&source_range_json(usize::from(start), usize::from(end)))
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"selectedWidthUnits\":");
    push_optional_u16_json(output, selected_width_units);
    output.push_str(",\"fullExtentUnits\":");
    push_optional_u16_json(output, full_extent_units);
    output.push_str(",\"fullExtentTrailingAfterSelectedUnits\":");
    push_optional_u16_json(output, full_extent_trailing_after_selected_units);
    output.push_str(",\"rowAgreementCount\":");
    output.push_str(&row_agreement_count.to_string());
    output.push_str(",\"allRowsAgree\":");
    output.push_str(if all_rows_agree { "true" } else { "false" });
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"firstTrailingHeaderUnitRange\":");
    match first_trailing_header {
        Some(header) => output.push_str(&source_range_json(
            usize::from(header.offset_units),
            usize::from(header.extent_units),
        )),
        None => output.push_str("null"),
    }
    output.push_str(",\"secondTrailingHeaderUnitRange\":");
    match second_trailing_header {
        Some(header) => output.push_str(&source_range_json(
            usize::from(header.offset_units),
            usize::from(header.extent_units),
        )),
        None => output.push_str("null"),
    }
    output.push_str(",\"firstTrailingGapAfterMatchedCellsUnits\":");
    push_optional_u16_json(output, first_trailing_gap_after_matched_units);
    output.push_str(",\"secondTrailingGapAfterFirstTrailingUnits\":");
    push_optional_u16_json(output, second_trailing_gap_after_first_trailing_units);
    output.push_str(",\"visibleRangeUsesFirstTrailingHeader\":");
    output.push_str(if visible_range_uses_first_trailing_header {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fullExtentIncludesSecondTrailingHeader\":");
    output.push_str(if full_extent_includes_second_trailing_header {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"trailingHeadersCoherent\":");
    output.push_str(if trailing_headers_coherent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementCoherenceReady\":");
    output.push_str(if source_placement_coherence_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"sourceTopTextPlacementReadinessGate\":");
    output.push_str("{\"source\":\"topTextLineHeaders+documentTextTableLineHeaders+/LineMark\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateSourceUnitRange\":");
    output.push_str(&source_range_json(
        table_source_offset_to_units(candidate.basis(), candidate.source_start()),
        table_source_offset_to_units(candidate.basis(), candidate.source_end()),
    ));
    output.push_str(",\"anchorSelection\":\"nearest-preceding-full-width-line-header\"");
    output.push_str(",\"sourceGapAfterAnchorTextUnits\":");
    output.push_str(&source_gap_after_anchor_text_units.to_string());
    output.push_str(",\"coherentWithTopTextAnchor\":");
    output.push_str(if coherent_with_top_text_anchor {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sharedFullExtentWithTable\":");
    output.push_str(if shared_full_extent_with_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sharedFontSizeWithTable\":");
    output.push_str(if shared_font_size_with_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateWidthBasis\":");
    output.push_str(&json_string(candidate_basis.as_str()));
    output.push_str(",\"selectedXUnitRange\":");
    match selected_range {
        Some((start, end)) => {
            output.push_str(&source_range_json(usize::from(start), usize::from(end)))
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"selectedWidthUnits\":");
    push_optional_u16_json(output, selected_width_units);
    output.push_str(",\"fullExtentUnits\":");
    push_optional_u16_json(output, full_extent_units);
    output.push_str(",\"trailingHeadersCoherent\":");
    output.push_str(if trailing_headers_coherent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allRowsAgree\":");
    output.push_str(if all_rows_agree { "true" } else { "false" });
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementCoherenceReady\":");
    output.push_str(if source_placement_coherence_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceVisibleWidthVsFullExtentGate\":");
    output.push_str("{\"source\":\"documentTextLineHeaders visible-width vs full-extent gate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"selectedXUnitRange\":");
    match selected_range {
        Some((start, end)) => {
            output.push_str(&source_range_json(usize::from(start), usize::from(end)))
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"selectedWidthUnits\":");
    push_optional_u16_json(output, selected_width_units);
    output.push_str(",\"fullExtentUnits\":");
    push_optional_u16_json(output, full_extent_units);
    output.push_str(",\"fullExtentTrailingAfterSelectedUnits\":");
    push_optional_u16_json(output, full_extent_trailing_after_selected_units);
    output.push_str(",\"firstTrailingHeaderUnitRange\":");
    match first_trailing_header {
        Some(header) => output.push_str(&source_range_json(
            usize::from(header.offset_units),
            usize::from(header.extent_units),
        )),
        None => output.push_str("null"),
    }
    output.push_str(",\"secondTrailingHeaderUnitRange\":");
    match second_trailing_header {
        Some(header) => output.push_str(&source_range_json(
            usize::from(header.offset_units),
            usize::from(header.extent_units),
        )),
        None => output.push_str("null"),
    }
    output.push_str(",\"selectedClosesAtFirstTrailingHeader\":");
    output.push_str(if selected_closes_at_first_trailing_header {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"secondTrailingIsFullLineRemainder\":");
    output.push_str(if second_trailing_is_full_line_remainder {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allRowsAgree\":");
    output.push_str(if all_rows_agree { "true" } else { "false" });
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"visibleWidthSemanticsReady\":");
    output.push_str(if visible_width_semantics_ready {
        "true"
    } else {
        "false"
    });
    output
        .push_str(",\"renderPromotionContribution\":\"source-visible-width-vs-full-extent-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if visible_width_semantics_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-total-width-semantics-unproven"));
    }
    output.push('}');
    output.push_str(",\"sourceTopTextPageGridCouplingGate\":");
    output.push_str("{\"source\":\"topTextAnchor+/LineMark+/PageMark+tableLineMarkPageOrigin\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"anchorSourceGridPresent\":");
    output.push_str(if anchor_source_grid.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"tableLineMarkPageOriginPresent\":");
    output.push_str(if table_page_origin.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"anchorLineMarkRecordIndex\":");
    push_optional_usize_json(
        output,
        anchor_source_grid
            .as_ref()
            .map(|anchor| anchor.line_grid.record_index),
    );
    output.push_str(",\"firstTableLineMarkRecordIndex\":");
    push_optional_usize_json(output, first_table_line_mark_record_index);
    output.push_str(",\"lineMarkRecordGapAfterAnchor\":");
    push_optional_i32_json(output, line_mark_record_gap_after_anchor);
    output.push_str(",\"anchorPageMarkEntryIndex\":");
    push_optional_usize_json(
        output,
        anchor_source_grid
            .as_ref()
            .map(|anchor| anchor.line_grid.page_mark_entry_index),
    );
    output.push_str(",\"tablePageMarkEntryIndex\":");
    push_optional_usize_json(
        output,
        table_page_origin
            .as_ref()
            .map(|origin| origin.page_mark_entry_index),
    );
    output.push_str(",\"samePageMarkEntry\":");
    output.push_str(if same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"anchorBaselineY\":");
    push_optional_f32_json(
        output,
        anchor_source_grid.as_ref().map(|anchor| anchor.baseline_y),
    );
    output.push_str(",\"tableTopY\":");
    push_optional_f32_json(output, table_page_origin.as_ref().map(|origin| origin.y));
    output.push_str(",\"tableTopMinusAnchorBaselinePx\":");
    push_optional_f32_json(
        output,
        table_page_origin.as_ref().and_then(|origin| {
            anchor_source_grid
                .as_ref()
                .map(|anchor| origin.y - anchor.baseline_y)
        }),
    );
    output.push_str(",\"expectedAdjacentRows\":");
    output.push_str(if line_mark_record_gap_after_anchor == Some(1) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePageGridCouplingReady\":");
    output.push_str(if source_page_grid_coupling_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"source-top-text-page-grid-coupling\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if source_page_grid_coupling_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("page-space-y-origin-unproven"));
    }
    output.push('}');
    output.push_str(",\"sourceTopTextPlacementReady\":");
    output.push_str(if source_top_text_placement_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"readinessBlockedReasons\":");
    push_json_string_slice_array(output, &readiness_blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"source-top-text-placement-readiness-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if source_top_text_placement_ready {
        output.push_str("null");
    } else {
        match readiness_blocked_reasons.first() {
            Some(reason) => output.push_str(&json_string(reason)),
            None => output.push_str("null"),
        }
    }
    output.push('}');
    output.push_str(
        ",\"renderPromotionContribution\":\"source-top-text-trailing-header-placement-coherence\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if visible_width_semantics_ready {
        output.push_str("null");
    } else if source_placement_coherence_ready {
        output.push_str(&json_string("source-total-width-semantics-unproven"));
    } else {
        output.push_str(&json_string("source-placement-coherence-unproven"));
    }
    output.push('}');
    TableGridSourceTopTextPlacementReadiness {
        ready: source_top_text_placement_ready,
        blocked_reasons: readiness_blocked_reasons,
    }
}

pub(crate) fn push_table_grid_top_text_reference_coordinate_probe_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    slots: &[SuccessDataTestResolvedTextSlot],
) {
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
    let Some(max_extent_units) =
        table_candidate_document_text_line_header_rows(document, candidate)
            .iter()
            .flat_map(|row| row.headers.iter().map(|header| header.extent_units))
            .max()
            .filter(|extent| *extent > 0)
    else {
        output.push_str("null");
        return;
    };
    let anchor_slots = slots
        .iter()
        .filter(|slot| slot.line_header.is_some())
        .collect::<Vec<_>>();
    if anchor_slots.is_empty() {
        output.push_str("null");
        return;
    }

    let table_unit_width_px = reference_layout.width / f32::from(max_extent_units);
    let mut residuals = Vec::new();
    for slot in anchor_slots {
        let header = slot.line_header.unwrap();
        let projected_x_from_table_left =
            reference_layout.x + f32::from(header.offset_units) * table_unit_width_px;
        let residual_px = slot.x - projected_x_from_table_left;
        residuals.push((slot, header, projected_x_from_table_left, residual_px));
    }
    let max_abs_x_residual_px = residuals
        .iter()
        .map(|(_, _, _, residual)| residual.abs())
        .fold(0.0_f32, f32::max);

    output.push_str("{\"source\":\"referenceCoordinates+documentTextLineHeaders\"");
    output.push_str(",\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("probe-uses-reference-coordinates"));
    output.push_str(",\"tableBbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        reference_layout.x,
        reference_layout.y,
        reference_layout.width,
        reference_layout.row_height * candidate.intervals().len() as f32
    ));
    output.push_str(",\"basis\":\"table-reference-width-divided-by-max-line-header-extent\"");
    output.push_str(",\"tableMaxExtentUnits\":");
    output.push_str(&max_extent_units.to_string());
    output.push_str(",\"tableUnitWidthPx\":");
    output.push_str(&format!("{table_unit_width_px:.3}"));
    output.push_str(",\"anchorCount\":");
    output.push_str(&residuals.len().to_string());
    output.push_str(",\"maxAbsXResidualPx\":");
    output.push_str(&format!("{max_abs_x_residual_px:.3}"));
    output.push_str(",\"consistentWithSingleTableTransform\":false");
    output.push_str(",\"residuals\":[");
    for (index, (slot, header, projected_x, residual_px)) in residuals.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        output.push_str(&json_string(slot.text));
        output.push_str(",\"referenceX\":");
        output.push_str(&format!("{:.3}", slot.x));
        output.push_str(",\"referenceY\":");
        output.push_str(&format!("{:.3}", slot.y));
        output.push_str(",\"lineOffsetUnits\":");
        output.push_str(&header.offset_units.to_string());
        output.push_str(",\"lineExtentUnits\":");
        output.push_str(&header.extent_units.to_string());
        output.push_str(",\"projectedXFromTableLeft\":");
        output.push_str(&format!("{projected_x:.3}"));
        output.push_str(",\"xResidualPx\":");
        output.push_str(&format!("{residual_px:.3}"));
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn push_table_grid_reference_bbox_residual_evidence_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) {
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
    let Some(first_row) = rows.first() else {
        output.push_str("null");
        return;
    };
    if first_row.matched_cell_count == 0 || first_row.headers.len() < first_row.matched_cell_count {
        output.push_str("null");
        return;
    }

    let matched_headers = first_row
        .headers
        .iter()
        .take(first_row.matched_cell_count)
        .collect::<Vec<_>>();
    let matched_cell_span_units = matched_headers
        .iter()
        .map(|header| header.extent_units.saturating_sub(header.offset_units))
        .collect::<Vec<_>>();
    let matched_cell_gap_units = matched_headers
        .windows(2)
        .map(|pair| pair[1].offset_units.saturating_sub(pair[0].extent_units))
        .collect::<Vec<_>>();
    let tail_gap_units = first_row
        .headers
        .get(first_row.matched_cell_count)
        .and_then(|tail| {
            matched_headers
                .last()
                .map(|last| tail.offset_units.saturating_sub(last.extent_units))
        });
    let max_extent_units = first_row
        .headers
        .iter()
        .map(|header| header.extent_units)
        .max()
        .unwrap_or(0);
    let tail_span_units = matched_headers
        .last()
        .map(|last| max_extent_units.saturating_sub(last.extent_units));
    let reference_width_px_per_full_extent_unit = if max_extent_units > 0 {
        Some(reference_layout.width / f32::from(max_extent_units))
    } else {
        None
    };
    let reference_column_px_per_matched_unit = matched_cell_span_units
        .iter()
        .enumerate()
        .map(|(index, span)| {
            if *span == 0 {
                0.0
            } else {
                reference_layout.column_width_at(index) / f32::from(*span)
            }
        })
        .map(f64::from)
        .collect::<Vec<_>>();
    let first_span = matched_cell_span_units.first().copied();
    let equal_reference_columns_conflict_with_unit_spans =
        reference_layout.column_widths.is_empty()
            && first_span.is_some()
            && matched_cell_span_units
                .iter()
                .any(|span| Some(*span) != first_span);
    let reference_bbox_blocked_reason = if equal_reference_columns_conflict_with_unit_spans {
        "reference-bbox-conflicts-with-nonuniform-source-unit-spans"
    } else if reference_layout.column_widths.is_empty() {
        "reference-bbox-is-not-source-placement"
    } else {
        "reference-bbox-uses-source-column-widths-but-not-source-placement"
    };

    output.push_str("{\"source\":\"documentTextLineHeaders+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(reference_bbox_blocked_reason));
    output.push_str(",\"matchedCellSpanUnits\":");
    push_u16_array_json(output, &matched_cell_span_units);
    output.push_str(",\"matchedCellGapUnits\":");
    push_u16_array_json(output, &matched_cell_gap_units);
    output.push_str(",\"tailGapUnits\":");
    push_optional_u16_json(output, tail_gap_units);
    output.push_str(",\"tailSpanUnits\":");
    push_optional_u16_json(output, tail_span_units);
    output.push_str(",\"referenceColumnWidthPx\":");
    output.push_str(&format!("{:.3}", reference_layout.column_width));
    output.push_str(",\"referenceColumnWidthBasis\":");
    output.push_str(&json_string(reference_layout.column_width_basis));
    output.push_str(",\"referenceColumnWidthsPx\":");
    push_f32_array_json(output, &reference_layout.column_widths);
    output.push_str(",\"referenceTableWidthPx\":");
    output.push_str(&format!("{:.3}", reference_layout.width));
    output.push_str(",\"referenceWidthPxPerFullExtentUnit\":");
    match reference_width_px_per_full_extent_unit {
        Some(value) => output.push_str(&format!("{value:.3}")),
        None => output.push_str("null"),
    }
    output.push_str(",\"unitBBoxCandidateComparisons\":");
    push_table_grid_reference_unit_bbox_candidate_comparisons_json(
        output,
        &rows,
        first_row.matched_cell_count,
        reference_layout.width,
        max_extent_units,
    );
    output.push_str(",\"referenceVerticalComparison\":");
    push_table_grid_reference_vertical_comparison_json(
        output,
        document,
        candidate,
        &rows,
        reference_layout.clone(),
    );
    output.push_str(",\"sourceDerivedHorizontalComparison\":");
    push_table_grid_source_derived_horizontal_comparison_json(
        output,
        layout,
        document,
        lines,
        candidate,
        reference_layout,
    );
    output.push_str(",\"referenceColumnPxPerMatchedUnit\":");
    push_f64_array_json(output, &reference_column_px_per_matched_unit);
    output.push_str(",\"equalReferenceColumnsConflictWithUnitSpans\":");
    output.push_str(if equal_reference_columns_conflict_with_unit_spans {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_table_grid_reference_vertical_comparison_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
    reference_layout: TableGridReferenceLayout,
) {
    if rows.is_empty() {
        output.push_str("null");
        return;
    }
    let homogeneous_font_size_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.font_size_units))
        .try_fold(None, |seen, value| match seen {
            Some(previous) if previous != value => None,
            _ => Some(Some(value)),
        })
        .flatten();
    let Some(font_size_units) = homogeneous_font_size_units else {
        output.push_str("null");
        return;
    };

    let source_row_height_px = f32::from(font_size_units) * 1.75;
    let source_table_height_px = source_row_height_px * rows.len() as f32;
    let reference_table_height_px = reference_layout.row_height * rows.len() as f32;
    let matched_record_indexes = table_grid_line_mark_record_indexes_for_rows(document, candidate);
    let line_mark_record_span = matched_record_indexes
        .first()
        .zip(matched_record_indexes.last())
        .map(|(start, end)| end.saturating_sub(*start).saturating_add(1));

    output.push_str("{\"source\":\"/DocumentText+/LineMark+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourceRowHeightBasis\":\"documentTextLineHeaderFontSizeUnits\"");
    output.push_str(",\"homogeneousFontSizeUnits\":");
    output.push_str(&font_size_units.to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"lineMarkRecordSpan\":");
    push_optional_usize_json(output, line_mark_record_span);
    output.push_str(",\"sourceDerivedRowHeightPx\":");
    output.push_str(&format!("{source_row_height_px:.3}"));
    output.push_str(",\"sourceDerivedTableHeightPx\":");
    output.push_str(&format!("{source_table_height_px:.3}"));
    output.push_str(",\"referenceRowHeightPx\":");
    output.push_str(&format!("{:.3}", reference_layout.row_height));
    output.push_str(",\"referenceTableHeightPx\":");
    output.push_str(&format!("{reference_table_height_px:.3}"));
    output.push_str(",\"rowHeightResidualPx\":");
    output.push_str(&format!(
        "{:.3}",
        reference_layout.row_height - source_row_height_px
    ));
    output.push_str(",\"tableHeightResidualPx\":");
    output.push_str(&format!(
        "{:.3}",
        reference_table_height_px - source_table_height_px
    ));
    output.push_str(",\"renderPromotionContribution\":\"row-height-corroboration-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("page-space-y-origin-unproven"));
    output.push('}');
}
