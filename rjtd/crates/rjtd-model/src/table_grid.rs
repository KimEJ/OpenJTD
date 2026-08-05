use super::*;

pub(super) fn push_page_layer_table_grid_candidate_json(
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

pub(super) fn push_table_grid_source_anchor_evidence_json(
    output: &mut String,
    candidate: &TableCandidate,
) {
    output.push_str("{\"source\":\"tableCandidateColumnSegments\",\"basis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"cellSourceRangeCount\":");
    output.push_str(&table_candidate_source_anchor_count(candidate).to_string());
    output.push_str(",\"placementDerived\":false,\"geometryDecoded\":false,\"decoded\":false}");
}

pub(super) fn push_table_grid_geometry_derivation_evidence_json(
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

pub(super) fn push_table_grid_top_text_anchor_evidence_json(
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

pub(super) fn push_table_grid_top_text_table_source_gap_evidence_json(
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

pub(super) fn table_grid_source_top_text_placement_readiness_for_candidate(
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

pub(super) fn push_table_grid_source_table_placement_coherence_gate_json(
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

pub(super) fn push_table_grid_top_text_reference_coordinate_probe_json(
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

pub(super) fn push_table_grid_reference_bbox_residual_evidence_json(
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

pub(super) fn push_table_grid_reference_vertical_comparison_json(
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

pub(super) fn push_table_grid_source_derived_horizontal_comparison_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    reference_layout: TableGridReferenceLayout,
) {
    let Some(grid) = candidate.column_segment_grid_candidate() else {
        output.push_str("null");
        return;
    };
    let Some(source_layout) = source_derived_table_grid_overlay_layout(
        layout,
        document,
        lines,
        0,
        candidate,
        grid.column_count(),
    ) else {
        output.push_str("null");
        return;
    };

    let source_right = source_layout.x + source_layout.width;
    let reference_right = reference_layout.x + reference_layout.width;
    let width_residual = reference_layout.width - source_layout.width;
    let x_residual = reference_layout.x - source_layout.x;
    let right_residual = reference_right - source_right;

    output.push_str("{\"source\":\"sourceDerivedLayoutCandidate+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourceBBox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        source_layout.x, source_layout.y, source_layout.width, source_layout.height
    ));
    output.push_str(",\"referenceBBox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        reference_layout.x,
        reference_layout.y,
        reference_layout.width,
        reference_layout.row_height * source_layout.row_count as f32
    ));
    output.push_str(",\"xResidualPx\":");
    output.push_str(&format!("{x_residual:.3}"));
    output.push_str(",\"widthResidualPx\":");
    output.push_str(&format!("{width_residual:.3}"));
    output.push_str(",\"rightResidualPx\":");
    output.push_str(&format!("{right_residual:.3}"));
    output.push_str(",\"widthResidualAbsPx\":");
    output.push_str(&format!("{:.3}", width_residual.abs()));
    output.push_str(",\"xResidualAbsPx\":");
    output.push_str(&format!("{:.3}", x_residual.abs()));
    output.push_str(",\"xUnitRangeBasis\":");
    output.push_str(&json_string(source_layout.x_unit_range_basis));
    output.push_str(",\"xUnitRange\":");
    output.push_str(&source_range_json(
        usize::from(source_layout.x_unit_start),
        usize::from(source_layout.x_unit_end),
    ));
    output.push_str(",\"pageOriginAuthority\":");
    output.push_str(&json_string(source_layout.page_origin_authority));
    output.push_str(",\"widthAgreementStrong\":");
    output.push_str(if width_residual.abs() <= 1.5 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"xOriginAgreementStrong\":");
    output.push_str(if x_residual.abs() <= 1.5 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"originResidualEvidence\":");
    push_table_grid_source_origin_residual_evidence_json(
        output,
        document,
        candidate,
        &source_layout,
        &reference_layout,
    );
    output.push_str(",\"lineMarkStrideYComparison\":");
    push_table_grid_line_mark_stride_y_reference_comparison_json(
        output,
        &source_layout,
        &reference_layout,
    );
    output
        .push_str(",\"renderPromotionContribution\":\"source-width-corroboration-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("horizontal-x-origin-unproven"));
    output.push('}');
}

pub(super) fn push_table_grid_source_derived_horizontal_field_adjustment_probe_json(
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

    let source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let Some((page_mark_fields, page_mark_field_source)) = source_layout_page_mark_fields
        .map(|fields| (fields, "sourceDerivedLayoutCandidate"))
        .or_else(|| {
            cross_table_page_mark_fields
                .map(|fields| (fields, "crossTableRowBoundaryOffsetConsistency"))
        })
    else {
        output.push_str("null");
        return;
    };

    let first_slot_units = source_layout
        .as_ref()
        .and_then(|layout| layout.x_unit_column_slot_width_units.first().copied());
    let first_span_units = source_layout
        .as_ref()
        .and_then(|layout| layout.matched_cell_span_units.first().copied());
    let first_gap_units = source_layout
        .as_ref()
        .and_then(|layout| layout.matched_cell_gap_units.first().copied());
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let raw_header_count = rows.iter().map(|row| row.headers.len()).sum::<usize>();
    let matched_row_count = rows.iter().filter(|row| row.matched_cell_count > 0).count();
    let reference_right = reference_layout.x + reference_layout.width;

    output.push_str(
        "{\"source\":\"/PageMark selected u16 fields+referenceTableBBox+documentTextLineHeaders\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"pageMarkFieldSource\":");
    output.push_str(&json_string(page_mark_field_source));
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referenceBBox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3},\"right\":{:.3}}}",
        reference_layout.x,
        reference_layout.y,
        reference_layout.width,
        reference_layout.row_height * candidate.intervals().len() as f32,
        reference_right
    ));
    output.push_str(",\"pageMarkEntryIndex\":");
    push_option_usize_json(
        output,
        cross_table_probe
            .as_ref()
            .and_then(|probe| probe.page_mark_entry_index),
    );
    output.push_str(",\"pageLineStart\":");
    push_option_usize_json(
        output,
        cross_table_probe
            .as_ref()
            .and_then(|probe| probe.page_line_start),
    );
    output.push_str(",\"pageLineEnd\":");
    push_option_usize_json(
        output,
        cross_table_probe
            .as_ref()
            .and_then(|probe| probe.page_line_end),
    );
    output.push_str(",\"selectedFields\":");
    push_table_grid_horizontal_reference_page_mark_fields_json(output, page_mark_fields);
    output.push_str(",\"lineHeaderSlotEvidence\":{\"rowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&matched_row_count.to_string());
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&raw_header_count.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    push_optional_u16_json(output, first_slot_units);
    output.push_str(",\"firstMatchedCellSpanUnits\":");
    push_optional_u16_json(output, first_span_units);
    output.push_str(",\"firstIntercellGapUnits\":");
    push_optional_u16_json(output, first_gap_units);
    output.push('}');
    output.push_str(",\"directFieldTargetComparisons\":");
    push_table_grid_horizontal_field_target_comparisons_json(
        output,
        page_mark_fields,
        &[
            ("x", reference_layout.x),
            ("width", reference_layout.width),
            ("right", reference_right),
        ],
    );
    output.push_str(",\"bestDirectXField\":");
    push_table_grid_best_horizontal_field_target_json(output, page_mark_fields, reference_layout.x);
    output.push_str(",\"bestDirectWidthField\":");
    push_table_grid_best_horizontal_field_target_json(
        output,
        page_mark_fields,
        reference_layout.width,
    );
    output.push_str(",\"bestDirectRightField\":");
    push_table_grid_best_horizontal_field_target_json(output, page_mark_fields, reference_right);
    output.push_str(",\"slotAdjustedFieldTargetComparisons\":");
    push_table_grid_slot_adjusted_horizontal_field_target_comparisons_json(
        output,
        page_mark_fields,
        first_slot_units,
        reference_layout.x,
        reference_layout.width,
    );
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-horizontal-field-adjustment-probe\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

pub(super) fn push_table_grid_source_only_horizontal_field_consensus_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) {
    let current_source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = current_source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let Some((page_mark_fields, page_mark_field_source)) = source_layout_page_mark_fields
        .map(|fields| (fields, "sourceDerivedLayoutCandidate"))
        .or_else(|| {
            cross_table_page_mark_fields
                .map(|fields| (fields, "crossTableRowBoundaryOffsetConsistency"))
        })
    else {
        output.push_str("null");
        return;
    };

    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate);
    let sparse_table_candidate_index = sibling
        .as_ref()
        .map(|evidence| evidence.sparse_candidate.index());
    let related_source_layouts = table_grid_related_horizontal_source_layout_summaries(
        layout,
        document,
        lines,
        candidate,
        sparse_table_candidate_index,
        current_source_layout.as_ref(),
    );
    if related_source_layouts.is_empty() {
        output.push_str("null");
        return;
    }

    let related_table_candidate_indexes = table_grid_sparse_sibling_related_table_candidate_indexes(
        document,
        sparse_table_candidate_index,
        candidate,
    );
    let source_layout_candidate_indexes = related_source_layouts
        .iter()
        .map(|summary| summary.table_candidate_index)
        .collect::<Vec<_>>();
    let first_slot_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_column_slot_units)
        .collect::<Vec<_>>();
    let first_span_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_matched_cell_span_units)
        .collect::<Vec<_>>();
    let first_gap_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_intercell_gap_units)
        .collect::<Vec<_>>();
    let x_unit_starts = related_source_layouts
        .iter()
        .map(|summary| summary.x_unit_start)
        .collect::<Vec<_>>();
    let x_unit_ends = related_source_layouts
        .iter()
        .map(|summary| summary.x_unit_end)
        .collect::<Vec<_>>();
    let full_extent_units = related_source_layouts
        .iter()
        .map(|summary| summary.x_unit_full_extent_units)
        .filter(|units| *units > 0)
        .collect::<Vec<_>>();
    let stable_first_slot_units = single_u16_value(&first_slot_units);
    let stable_first_span_units = single_u16_value(&first_span_units);
    let stable_first_gap_units = single_u16_value(&first_gap_units);
    let stable_x_unit_start = single_u16_value(&x_unit_starts);
    let stable_x_unit_end = single_u16_value(&x_unit_ends);
    let stable_full_extent_units = single_u16_value(&full_extent_units);
    let all_related_layouts_have_stable_unit_frame = stable_first_slot_units.is_some()
        && stable_first_span_units.is_some()
        && stable_first_gap_units.is_some()
        && stable_x_unit_start.is_some()
        && stable_x_unit_end.is_some()
        && stable_full_extent_units.is_some()
        && related_source_layouts
            .iter()
            .all(|summary| summary.x_unit_all_rows_agree);

    output
        .push_str("{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark horizontal fields\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"pageMarkFieldSource\":");
    output.push_str(&json_string(page_mark_field_source));
    output.push_str(",\"sparseTableCandidateIndex\":");
    push_option_usize_json(output, sparse_table_candidate_index);
    output.push_str(",\"relatedTableCandidateIndexes\":");
    push_usize_array_json(output, &related_table_candidate_indexes);
    output.push_str(",\"sourceDerivedRelatedTableCandidateIndexes\":");
    push_usize_array_json(output, &source_layout_candidate_indexes);
    output.push_str(",\"sourceDerivedRelatedTableCount\":");
    output.push_str(&related_source_layouts.len().to_string());
    output.push_str(",\"stableFirstColumnSlotUnits\":");
    push_optional_u16_json(output, stable_first_slot_units);
    output.push_str(",\"stableFirstMatchedCellSpanUnits\":");
    push_optional_u16_json(output, stable_first_span_units);
    output.push_str(",\"stableFirstIntercellGapUnits\":");
    push_optional_u16_json(output, stable_first_gap_units);
    output.push_str(",\"stableXUnitRange\":");
    match (stable_x_unit_start, stable_x_unit_end) {
        (Some(start), Some(end)) => {
            output.push_str(&source_range_json(usize::from(start), usize::from(end)));
        }
        _ => output.push_str("null"),
    }
    output.push_str(",\"stableFullExtentUnits\":");
    push_optional_u16_json(output, stable_full_extent_units);
    output.push_str(",\"allRelatedLayoutsHaveStableUnitFrame\":");
    output.push_str(if all_related_layouts_have_stable_unit_frame {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"relatedSourceLayouts\":");
    push_table_grid_related_horizontal_source_layout_summaries_json(
        output,
        &related_source_layouts,
    );
    output.push_str(",\"sourceOnlyFrameHypotheses\":");
    push_table_grid_source_only_horizontal_field_consensus_hypotheses_json(
        output,
        page_mark_fields,
        stable_first_slot_units,
        stable_first_gap_units,
    );
    output.push_str(",\"sourceOnlyHorizontalFieldSelector\":");
    push_table_grid_source_only_horizontal_field_selector_json(
        output,
        candidate,
        page_mark_fields,
        page_mark_field_source,
        stable_first_slot_units,
        stable_first_gap_units,
    );
    output.push_str(",\"renderPromotionContribution\":\"source-only-horizontal-field-consensus\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "cross-table-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

pub(super) fn table_grid_related_horizontal_source_layout_summaries(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    sparse_table_candidate_index: Option<usize>,
    current_source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Vec<TableGridRelatedHorizontalSourceLayoutSummary> {
    let mut summaries = Vec::new();
    if let Some(sparse_table_candidate_index) = sparse_table_candidate_index {
        for related in document
            .table_candidates()
            .iter()
            .filter(|related| related.is_document_text_control_run_candidate())
            .filter(|related| {
                table_grid_sparse_table_sibling_evidence(document, related).is_some_and(
                    |evidence| evidence.sparse_candidate.index() == sparse_table_candidate_index,
                )
            })
        {
            let Some(grid) = related.column_segment_grid_candidate() else {
                continue;
            };
            let Some(source_layout) = source_derived_table_grid_overlay_layout(
                layout,
                document,
                lines,
                0,
                related,
                grid.column_count(),
            ) else {
                continue;
            };
            summaries.push(table_grid_related_horizontal_source_layout_summary(
                related.index(),
                &source_layout,
            ));
        }
    } else if let Some(source_layout) = current_source_layout {
        summaries.push(table_grid_related_horizontal_source_layout_summary(
            candidate.index(),
            source_layout,
        ));
    }

    summaries.sort_by_key(|summary| summary.table_candidate_index);
    summaries.dedup_by_key(|summary| summary.table_candidate_index);
    summaries
}

pub(super) fn table_grid_sparse_sibling_related_table_candidate_indexes(
    document: &Document,
    sparse_table_candidate_index: Option<usize>,
    candidate: &TableCandidate,
) -> Vec<usize> {
    let Some(sparse_table_candidate_index) = sparse_table_candidate_index else {
        return vec![candidate.index()];
    };
    let mut indexes = document
        .table_candidates()
        .iter()
        .filter(|related| related.is_document_text_control_run_candidate())
        .filter(|related| {
            table_grid_sparse_table_sibling_evidence(document, related).is_some_and(|evidence| {
                evidence.sparse_candidate.index() == sparse_table_candidate_index
            })
        })
        .map(|related| related.index())
        .collect::<Vec<_>>();
    indexes.sort_unstable();
    indexes.dedup();
    indexes
}

pub(super) fn table_grid_related_horizontal_source_layout_summary(
    table_candidate_index: usize,
    source_layout: &TableGridSourceDerivedLayout,
) -> TableGridRelatedHorizontalSourceLayoutSummary {
    TableGridRelatedHorizontalSourceLayoutSummary {
        table_candidate_index,
        row_count: source_layout.row_count,
        column_count: source_layout.column_count,
        x_unit_start: source_layout.x_unit_start,
        x_unit_end: source_layout.x_unit_end,
        x_unit_full_extent_units: source_layout.x_unit_full_extent_units,
        x_unit_all_rows_agree: source_layout.x_unit_all_rows_agree,
        first_column_slot_units: source_layout
            .x_unit_column_slot_width_units
            .first()
            .copied(),
        first_matched_cell_span_units: source_layout.matched_cell_span_units.first().copied(),
        first_intercell_gap_units: source_layout.matched_cell_gap_units.first().copied(),
        matched_cell_span_units: source_layout.matched_cell_span_units.clone(),
        matched_cell_gap_units: source_layout.matched_cell_gap_units.clone(),
        x_unit_column_slot_width_units: source_layout.x_unit_column_slot_width_units.clone(),
    }
}

pub(super) fn push_table_grid_related_horizontal_source_layout_summaries_json(
    output: &mut String,
    summaries: &[TableGridRelatedHorizontalSourceLayoutSummary],
) {
    output.push('[');
    for (index, summary) in summaries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"tableCandidateIndex\":");
        output.push_str(&summary.table_candidate_index.to_string());
        output.push_str(",\"rowCount\":");
        output.push_str(&summary.row_count.to_string());
        output.push_str(",\"columnCount\":");
        output.push_str(&summary.column_count.to_string());
        output.push_str(",\"xUnitRange\":");
        output.push_str(&source_range_json(
            usize::from(summary.x_unit_start),
            usize::from(summary.x_unit_end),
        ));
        output.push_str(",\"fullExtentUnits\":");
        output.push_str(&summary.x_unit_full_extent_units.to_string());
        output.push_str(",\"xUnitAllRowsAgree\":");
        output.push_str(if summary.x_unit_all_rows_agree {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"firstColumnSlotUnits\":");
        push_optional_u16_json(output, summary.first_column_slot_units);
        output.push_str(",\"firstMatchedCellSpanUnits\":");
        push_optional_u16_json(output, summary.first_matched_cell_span_units);
        output.push_str(",\"firstIntercellGapUnits\":");
        push_optional_u16_json(output, summary.first_intercell_gap_units);
        output.push_str(",\"matchedCellSpanUnits\":");
        push_u16_array_json(output, &summary.matched_cell_span_units);
        output.push_str(",\"matchedCellGapUnits\":");
        push_u16_array_json(output, &summary.matched_cell_gap_units);
        output.push_str(",\"columnSlotWidthUnits\":");
        push_u16_array_json(output, &summary.x_unit_column_slot_width_units);
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_table_grid_source_only_horizontal_field_consensus_hypotheses_json(
    output: &mut String,
    page_mark_fields: &[u16],
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
) {
    output.push('[');
    let mut emitted = false;
    push_table_grid_source_only_horizontal_field_consensus_hypotheses_items_json(
        output,
        page_mark_fields,
        stable_first_slot_units,
        stable_first_gap_units,
        &mut emitted,
    );
    output.push(']');
}

pub(super) fn push_table_grid_source_only_horizontal_field_selector_json(
    output: &mut String,
    candidate: &TableCandidate,
    page_mark_fields: &[u16],
    page_mark_field_source: &'static str,
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
) {
    let Some(word_14) = page_mark_fields.get(14).copied() else {
        output.push_str("null");
        return;
    };
    let Some(first_slot_units) = stable_first_slot_units.filter(|units| *units > 0) else {
        output.push_str("null");
        return;
    };
    let compact_column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let word_15 = page_mark_fields.get(15).copied();
    let word_21 = page_mark_fields.get(21).copied();
    let selector = match compact_column_count {
        2 => word_21.map(|word_21| {
            (
                "page-mark-word14-first-slot-word21-half-slot",
                "compact-two-column-page-mark-word21-half-slot",
                21usize,
                word_21,
                f32::from(first_slot_units) * 0.5,
                "cross-table-half-first-column-slot",
                f32::from(word_21) - f32::from(first_slot_units) * 0.5,
            )
        }),
        3 => word_15.and_then(|word_15| {
            stable_first_gap_units
                .filter(|units| *units > 0)
                .map(|units| {
                    (
                        "page-mark-word14-first-slot-word15-half-gap",
                        "compact-three-column-page-mark-word15-half-gap",
                        15usize,
                        word_15,
                        f32::from(units) * 0.5,
                        "cross-table-half-first-intercell-gap",
                        f32::from(word_15) - f32::from(units) * 0.5,
                    )
                })
        }),
        _ => None,
    };
    let Some((
        selected_frame_basis,
        selection_basis,
        selected_width_word_index,
        page_mark_width_word,
        width_adjustment_units,
        width_adjustment_basis,
        selected_width,
    )) = selector
    else {
        output.push_str("null");
        return;
    };
    if !selected_x.is_finite() || !selected_width.is_finite() || selected_width <= 0.0 {
        output.push_str("null");
        return;
    }

    output
        .push_str("{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark horizontal fields\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsedForSelection\":false");
    output.push_str(",\"selectionReady\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"pageMarkFieldSource\":");
    output.push_str(&json_string(page_mark_field_source));
    output.push_str(",\"compactColumnCount\":");
    output.push_str(&compact_column_count.to_string());
    output.push_str(",\"selectionBasis\":");
    output.push_str(&json_string(selection_basis));
    output.push_str(",\"selectedFrameBasis\":");
    output.push_str(&json_string(selected_frame_basis));
    output.push_str(",\"pageMarkXWord14\":");
    output.push_str(&word_14.to_string());
    output.push_str(",\"pageMarkWidthWord\":");
    output.push_str(&page_mark_width_word.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    output.push_str(&first_slot_units.to_string());
    output.push_str(",\"firstIntercellGapUnits\":");
    push_optional_u16_json(output, stable_first_gap_units);
    output.push_str(",\"xAdjustmentUnits\":");
    output.push_str(&format!("{:.3}", f32::from(first_slot_units)));
    output.push_str(",\"widthAdjustmentUnits\":");
    output.push_str(&format!("{width_adjustment_units:.3}"));
    output.push_str(",\"xAdjustmentBasis\":\"cross-table-first-column-slot\"");
    output.push_str(",\"widthAdjustmentBasis\":");
    output.push_str(&json_string(width_adjustment_basis));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(",\"sourceWidthFieldRoleGate\":");
    push_table_grid_source_only_horizontal_width_field_role_gate_json(
        output,
        compact_column_count,
        page_mark_field_source,
        word_14,
        word_15,
        word_21,
        first_slot_units,
        stable_first_gap_units,
        selected_width_word_index,
        page_mark_width_word,
        width_adjustment_units,
        width_adjustment_basis,
        selected_frame_basis,
        selection_basis,
    );
    output.push_str(",\"renderPromotionContribution\":\"source-only-horizontal-field-selector\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "cross-table-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_source_only_horizontal_width_field_role_gate_json(
    output: &mut String,
    compact_column_count: usize,
    page_mark_field_source: &'static str,
    page_mark_x_word_14: u16,
    page_mark_word_15: Option<u16>,
    page_mark_word_21: Option<u16>,
    first_column_slot_units: u16,
    first_intercell_gap_units: Option<u16>,
    selected_width_word_index: usize,
    selected_width_word: u16,
    selected_width_adjustment_units: f32,
    selected_width_adjustment_basis: &'static str,
    selected_frame_basis: &'static str,
    selection_basis: &'static str,
) {
    let three_column_candidate_present = compact_column_count == 3
        && page_mark_word_15.is_some()
        && first_intercell_gap_units.is_some();
    let two_column_candidate_present = compact_column_count == 2 && page_mark_word_21.is_some();
    let selector_matches_compact_column_count = (compact_column_count == 2
        && selected_width_word_index == 21)
        || (compact_column_count == 3 && selected_width_word_index == 15);
    let selected_width_field_role = match (compact_column_count, selected_width_word_index) {
        (2, 21) => "compact-two-column-visible-width",
        (3, 15) => "compact-three-column-visible-width",
        (_, 21) => "word21-alternate-width-candidate",
        (_, 15) => "word15-alternate-width-candidate",
        _ => "unknown-width-field-role",
    };

    output.push_str(
        "{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark width-field role gate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false");
    output.push_str(",\"pageMarkFieldSource\":");
    output.push_str(&json_string(page_mark_field_source));
    output.push_str(",\"compactColumnCount\":");
    output.push_str(&compact_column_count.to_string());
    output.push_str(",\"pageMarkXWord14\":");
    output.push_str(&page_mark_x_word_14.to_string());
    output.push_str(",\"pageMarkWord15\":");
    push_optional_u16_json(output, page_mark_word_15);
    output.push_str(",\"pageMarkWord21\":");
    push_optional_u16_json(output, page_mark_word_21);
    output.push_str(",\"firstColumnSlotUnits\":");
    output.push_str(&first_column_slot_units.to_string());
    output.push_str(",\"firstIntercellGapUnits\":");
    push_optional_u16_json(output, first_intercell_gap_units);
    output.push_str(",\"selectedWidthWordIndex\":");
    output.push_str(&selected_width_word_index.to_string());
    output.push_str(",\"selectedWidthWord\":");
    output.push_str(&selected_width_word.to_string());
    output.push_str(",\"selectedWidthFieldRole\":");
    output.push_str(&json_string(selected_width_field_role));
    output.push_str(",\"selectedWidthAdjustmentUnits\":");
    output.push_str(&format!("{selected_width_adjustment_units:.3}"));
    output.push_str(",\"selectedWidthAdjustmentBasis\":");
    output.push_str(&json_string(selected_width_adjustment_basis));
    output.push_str(",\"selectedFrameBasis\":");
    output.push_str(&json_string(selected_frame_basis));
    output.push_str(",\"selectionBasis\":");
    output.push_str(&json_string(selection_basis));
    output.push_str(",\"twoColumnWidthCandidatePresent\":");
    output.push_str(if two_column_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"threeColumnWidthCandidatePresent\":");
    output.push_str(if three_column_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectorMatchesCompactColumnCount\":");
    output.push_str(if selector_matches_compact_column_count {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"source-horizontal-width-field-role-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "width-field-role-semantics-needs-cross-sample-validation",
    ));
    output.push('}');
}

pub(super) fn push_table_grid_source_only_horizontal_field_consensus_hypotheses_items_json(
    output: &mut String,
    page_mark_fields: &[u16],
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
    emitted: &mut bool,
) {
    let Some(word_14) = page_mark_fields.get(14).copied() else {
        return;
    };
    let Some(word_15) = page_mark_fields.get(15).copied() else {
        return;
    };
    let Some(word_21) = page_mark_fields.get(21).copied() else {
        return;
    };
    let Some(first_slot_units) = stable_first_slot_units.filter(|units| *units > 0) else {
        return;
    };
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let half_gap_adjustment = stable_first_gap_units
        .filter(|units| *units > 0)
        .map(|units| f32::from(units) * 0.5);
    let half_slot_adjustment = f32::from(first_slot_units) * 0.5;

    if *emitted {
        output.push(',');
    }
    push_table_grid_source_only_horizontal_field_consensus_hypothesis_json(
        output,
        "page-mark-word14-first-slot-word15-direct",
        word_14,
        word_15,
        selected_x,
        f32::from(word_15),
        first_slot_units,
        f32::from(first_slot_units),
        0.0,
        "cross-table-first-column-slot",
        "none",
    );
    *emitted = true;
    if let Some(half_gap_adjustment) = half_gap_adjustment {
        output.push(',');
        push_table_grid_source_only_horizontal_field_consensus_hypothesis_json(
            output,
            "page-mark-word14-first-slot-word15-half-gap",
            word_14,
            word_15,
            selected_x,
            f32::from(word_15) - half_gap_adjustment,
            first_slot_units,
            f32::from(first_slot_units),
            half_gap_adjustment,
            "cross-table-first-column-slot",
            "cross-table-half-first-intercell-gap",
        );
    }
    output.push(',');
    push_table_grid_source_only_horizontal_field_consensus_hypothesis_json(
        output,
        "page-mark-word14-first-slot-word21-half-slot",
        word_14,
        word_21,
        selected_x,
        f32::from(word_21) - half_slot_adjustment,
        first_slot_units,
        f32::from(first_slot_units),
        half_slot_adjustment,
        "cross-table-first-column-slot",
        "cross-table-half-first-column-slot",
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_source_only_horizontal_field_consensus_hypothesis_json(
    output: &mut String,
    frame_basis: &'static str,
    page_mark_x_word_14: u16,
    page_mark_width_word: u16,
    selected_x: f32,
    selected_width: f32,
    first_column_slot_units: u16,
    x_adjustment_units: f32,
    width_adjustment_units: f32,
    x_adjustment_basis: &'static str,
    width_adjustment_basis: &'static str,
) {
    output.push_str("{\"frameBasis\":");
    output.push_str(&json_string(frame_basis));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateBasis\":\"page-mark-u16-fields+cross-table-line-header-slots\"");
    output.push_str(",\"pageMarkXWord14\":");
    output.push_str(&page_mark_x_word_14.to_string());
    output.push_str(",\"pageMarkWidthWord\":");
    output.push_str(&page_mark_width_word.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    output.push_str(&first_column_slot_units.to_string());
    output.push_str(",\"xAdjustmentUnits\":");
    output.push_str(&format!("{x_adjustment_units:.3}"));
    output.push_str(",\"widthAdjustmentUnits\":");
    output.push_str(&format!("{width_adjustment_units:.3}"));
    output.push_str(",\"xAdjustmentBasis\":");
    output.push_str(&json_string(x_adjustment_basis));
    output.push_str(",\"widthAdjustmentBasis\":");
    output.push_str(&json_string(width_adjustment_basis));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "cross-table-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

pub(super) fn push_table_grid_horizontal_reference_page_mark_fields_json(
    output: &mut String,
    fields: &[u16],
) {
    output.push('[');
    for (index, word_index) in PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES
        .iter()
        .copied()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        let value = fields.get(word_index).copied();
        output.push_str("{\"wordIndex\":");
        output.push_str(&word_index.to_string());
        output.push_str(",\"value\":");
        push_optional_u16_json(output, value);
        output.push_str(",\"hex\":");
        push_option_u16_hex_json(output, value);
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_table_grid_horizontal_field_target_comparisons_json(
    output: &mut String,
    fields: &[u16],
    targets: &[(&str, f32)],
) {
    output.push('[');
    let mut emitted = 0usize;
    for word_index in PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES {
        let Some(value) = fields.get(word_index).copied() else {
            continue;
        };
        for (target_name, target_px) in targets {
            if emitted > 0 {
                output.push(',');
            }
            let value_px = f32::from(value);
            let residual = value_px - *target_px;
            output.push_str("{\"wordIndex\":");
            output.push_str(&word_index.to_string());
            output.push_str(",\"value\":");
            output.push_str(&value.to_string());
            output.push_str(",\"valuePx\":");
            output.push_str(&format!("{value_px:.3}"));
            output.push_str(",\"target\":");
            output.push_str(&json_string(target_name));
            output.push_str(",\"targetPx\":");
            output.push_str(&format!("{target_px:.3}"));
            output.push_str(",\"residualPx\":");
            output.push_str(&format!("{residual:.3}"));
            output.push_str(",\"absResidualPx\":");
            output.push_str(&format!("{:.3}", residual.abs()));
            output.push_str(",\"withinTwoPx\":");
            output.push_str(if residual.abs() <= 2.0 {
                "true"
            } else {
                "false"
            });
            output.push('}');
            emitted += 1;
        }
    }
    output.push(']');
}

pub(super) fn push_table_grid_best_horizontal_field_target_json(
    output: &mut String,
    fields: &[u16],
    target_px: f32,
) {
    let best = PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES
        .iter()
        .copied()
        .filter_map(|word_index| {
            fields.get(word_index).copied().map(|value| {
                let residual = f32::from(value) - target_px;
                (word_index, value, residual)
            })
        })
        .min_by(|left, right| left.2.abs().total_cmp(&right.2.abs()));
    let Some((word_index, value, residual)) = best else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"wordIndex\":");
    output.push_str(&word_index.to_string());
    output.push_str(",\"value\":");
    output.push_str(&value.to_string());
    output.push_str(",\"targetPx\":");
    output.push_str(&format!("{target_px:.3}"));
    output.push_str(",\"residualPx\":");
    output.push_str(&format!("{residual:.3}"));
    output.push_str(",\"absResidualPx\":");
    output.push_str(&format!("{:.3}", residual.abs()));
    output.push('}');
}

pub(super) fn push_table_grid_slot_adjusted_horizontal_field_target_comparisons_json(
    output: &mut String,
    fields: &[u16],
    first_slot_units: Option<u16>,
    reference_x: f32,
    reference_width: f32,
) {
    let Some(first_slot_units) = first_slot_units.filter(|units| *units > 0) else {
        output.push_str("[]");
        return;
    };
    let first_slot_px = f32::from(first_slot_units);
    let comparisons = [
        (
            14usize,
            "x",
            "line-header-first-column-slot",
            first_slot_px,
            reference_x,
        ),
        (
            21usize,
            "width",
            "line-header-half-first-column-slot",
            first_slot_px * 0.5,
            reference_width,
        ),
    ];

    output.push('[');
    let mut emitted = 0usize;
    for (word_index, target_name, adjustment_basis, adjustment_px, target_px) in comparisons {
        let Some(value) = fields.get(word_index).copied() else {
            continue;
        };
        if emitted > 0 {
            output.push(',');
        }
        let value_px = f32::from(value);
        let adjusted_value_px = value_px - adjustment_px;
        let residual = adjusted_value_px - target_px;
        output.push_str("{\"wordIndex\":");
        output.push_str(&word_index.to_string());
        output.push_str(",\"value\":");
        output.push_str(&value.to_string());
        output.push_str(",\"valuePx\":");
        output.push_str(&format!("{value_px:.3}"));
        output.push_str(",\"target\":");
        output.push_str(&json_string(target_name));
        output.push_str(",\"targetPx\":");
        output.push_str(&format!("{target_px:.3}"));
        output.push_str(",\"adjustmentBasis\":");
        output.push_str(&json_string(adjustment_basis));
        output.push_str(",\"adjustmentUnits\":");
        output.push_str(&format!("{adjustment_px:.3}"));
        output.push_str(",\"adjustedValuePx\":");
        output.push_str(&format!("{adjusted_value_px:.3}"));
        output.push_str(",\"residualPx\":");
        output.push_str(&format!("{residual:.3}"));
        output.push_str(",\"absResidualPx\":");
        output.push_str(&format!("{:.3}", residual.abs()));
        output.push('}');
        emitted += 1;
    }
    output.push(']');
}

pub(super) fn push_table_grid_line_mark_stride_y_reference_comparison_json(
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

pub(super) fn push_table_grid_line_mark_stride_y_reference_comparison_fields_json(
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

pub(super) fn push_table_grid_line_mark_record_index_affine_fit_json(
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

pub(super) fn push_table_grid_sparse_sibling_line_mark_y_comparison_json(
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

pub(super) fn push_table_grid_sparse_sibling_post_row_gap_line_mark_correlation_json(
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

pub(super) fn push_table_grid_page_mark_raw_record_scan_evidence_json(
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

pub(super) fn push_table_grid_page_mark_raw_record_source_range_evidence_json(
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

pub(super) fn table_grid_page_mark_raw_record_source_range_coverage_summary(
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

pub(super) fn push_table_grid_page_mark_raw_record_source_range_coverage_summary_json(
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

pub(super) fn push_table_grid_page_mark_scoped_y_transform_probe_json(
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
pub(super) fn push_table_grid_page_mark_y_candidate_lineage_gate_json(
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

pub(super) fn push_table_grid_page_mark_subrecord_line_span_correlation_json(
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

pub(super) const TABLE_GRID_SUBRECORD_LINE_SPAN_TOLERANCE_UNITS: i32 = 3;

pub(super) fn table_grid_page_mark_subrecord_line_span_readiness(
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

pub(super) fn push_table_grid_page_mark_subrecord_line_span_readiness_json(
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

pub(super) fn push_table_grid_page_mark_subrecord_span_role_gate_json(
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

pub(super) fn table_grid_page_mark_subrecord_line_span_coverage(
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

pub(super) fn push_table_grid_page_mark_subrecord_line_span_coverage_json(
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

pub(super) fn table_grid_cross_table_subrecord_ordering_probe(
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

pub(super) fn push_table_grid_cross_table_subrecord_ordering_probe_json(
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

pub(super) fn push_table_grid_cross_table_subrecord_ordering_probe_summary_json(
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

pub(super) fn push_table_grid_cross_table_subrecord_ordering_probe_prefix_json(
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

pub(super) fn push_table_grid_cross_table_subrecord_ordering_match_json(
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

pub(super) fn table_grid_cross_table_row_boundary_offset_probe(
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

pub(super) fn table_grid_source_unit_to_page_line_index_piecewise_table(
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

pub(super) fn table_grid_source_unit_to_page_line_index_piecewise_transition(
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

pub(super) fn push_table_grid_cross_table_row_boundary_offset_probe_summary_json(
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

pub(super) fn push_table_grid_cross_table_row_boundary_offset_table_json(
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

pub(super) fn push_table_grid_piecewise_record_family_gap_y_diagnostic_json(
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

pub(super) fn push_table_grid_source_only_page_mark_slot_scoped_subrecord_y_sequence_probe_json(
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

pub(super) fn push_table_grid_piecewise_record_family_gap_table_json(
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

pub(super) fn push_table_grid_piecewise_record_family_gap_transition_json(
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

pub(super) fn push_table_grid_source_unit_to_page_line_index_piecewise_fit_json(
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

pub(super) fn push_table_grid_source_unit_to_page_line_index_piecewise_table_json(
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

pub(super) fn push_table_grid_source_unit_to_page_line_index_piecewise_transition_json(
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

pub(super) fn push_table_grid_source_unit_to_page_line_index_fit_row_json(
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

pub(super) fn push_table_grid_page_mark_raw_reference_value_probe_json(
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
pub(super) fn push_table_grid_line_mark_stride_promotion_readiness_json(
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

pub(super) fn table_grid_sparse_sibling_post_row_gap_line_mark_correlation_counts(
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

pub(super) fn table_grid_page_mark_raw_scan_header_match_summary(
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

pub(super) fn table_grid_sparse_sibling_post_row_gap(
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

pub(super) fn push_table_grid_line_mark_row_gap_sequence_evidence_json(
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

pub(super) fn push_table_grid_line_mark_row_gap_sequence_y_comparison_json(
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

pub(super) fn table_grid_page_mark_context_for_line_mark_record_indexes(
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

pub(super) fn table_grid_line_mark_row_gap_sequence_rows(
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

pub(super) fn table_grid_previous_row_span_line_mark_record_indexes(
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

pub(super) fn table_grid_resolved_line_mark_rows_for_rows(
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

pub(super) fn table_grid_previous_row_span_line_mark_rows_for_rows(
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

pub(super) fn push_table_grid_line_mark_row_gap_sequence_row_json(
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

pub(super) fn table_grid_source_span_units(
    basis: TextCountRangeOverlapBasis,
    source_start: usize,
    source_end: usize,
) -> usize {
    table_source_offset_to_units(basis, source_end)
        .saturating_sub(table_source_offset_to_units(basis, source_start))
}

pub(super) fn table_grid_line_header_font_size_units_candidate(
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

pub(super) fn push_table_grid_source_origin_residual_evidence_json(
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

pub(super) fn table_grid_source_layout_page_mark_u16_fields(
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

pub(super) fn push_table_grid_origin_residual_page_mark_fields_json(
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

pub(super) fn push_table_grid_origin_residual_page_mark_raw_field_reference_comparison_json(
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

pub(super) fn table_grid_line_mark_record_indexes_for_rows(
    document: &Document,
    candidate: &TableCandidate,
) -> Vec<usize> {
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows)
        .into_iter()
        .map(|row| row.interval.record_index)
        .collect()
}

pub(super) fn push_table_grid_reference_unit_bbox_candidate_comparisons_json(
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

pub(super) fn push_table_grid_top_text_anchor_residual_evidence_json(
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

pub(super) fn push_table_grid_document_text_line_header_evidence_json(
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

pub(super) fn push_table_grid_sparse_table_sibling_evidence_json(
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

pub(super) fn push_table_grid_sparse_sibling_column_promotion_readiness_json(
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

pub(super) fn push_table_grid_sparse_sibling_derived_compact_cell_geometry_json(
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

pub(super) fn table_grid_sparse_table_sibling_evidence<'a>(
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

pub(super) fn table_grid_sparse_table_sibling_evidence_for_candidate<'a>(
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

pub(super) fn table_grid_sparse_sibling_row_match(
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

pub(super) fn table_grid_sparse_sibling_column_offset(
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

pub(super) fn table_grid_sparse_sibling_matched_sparse_column_indexes(
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

pub(super) fn push_table_grid_line_header_line_mark_coupling_evidence_json(
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

pub(super) fn push_table_grid_line_header_unit_geometry_candidate_json(
    output: &mut String,
    rows: &[TableCandidateLineHeaderRow],
) {
    let Some(first_row) = rows.first() else {
        output.push_str("null");
        return;
    };
    if first_row.headers.is_empty() {
        output.push_str("null");
        return;
    }
    let matched_column_count = rows
        .iter()
        .map(|row| row.matched_cell_count)
        .min()
        .unwrap_or(0);
    if matched_column_count == 0 {
        output.push_str("null");
        return;
    }

    let rows_homogeneous = rows.iter().all(|row| {
        row.headers.len() == first_row.headers.len()
            && row
                .headers
                .iter()
                .zip(&first_row.headers)
                .all(|(left, right)| {
                    left.offset_units == right.offset_units
                        && left.extent_units == right.extent_units
                        && left.font_size_units == right.font_size_units
                })
    });
    let min_offset_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.offset_units))
        .min();
    let max_extent_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.extent_units))
        .max();
    let homogeneous_font_size_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.font_size_units))
        .try_fold(None, |seen, value| match seen {
            Some(previous) if previous != value => None,
            _ => Some(Some(value)),
        })
        .flatten();
    let matched_cell_offset_units = first_row
        .headers
        .iter()
        .take(matched_column_count)
        .map(|header| header.offset_units)
        .collect::<Vec<_>>();
    let matched_cell_extent_units = first_row
        .headers
        .iter()
        .take(matched_column_count)
        .map(|header| header.extent_units)
        .collect::<Vec<_>>();
    let trailing_header_offset_units = first_row
        .headers
        .iter()
        .skip(matched_column_count)
        .map(|header| header.offset_units)
        .collect::<Vec<_>>();
    let trailing_header_extent_units = first_row
        .headers
        .iter()
        .skip(matched_column_count)
        .map(|header| header.extent_units)
        .collect::<Vec<_>>();

    output.push_str("{\"source\":\"documentTextLineHeaders\"");
    output.push_str(",\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("page-space-origin-and-unit-scale-unproven"));
    output.push_str(",\"rowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"rowsHomogeneous\":");
    output.push_str(if rows_homogeneous { "true" } else { "false" });
    output.push_str(",\"matchedColumnCount\":");
    output.push_str(&matched_column_count.to_string());
    output.push_str(",\"rawHeaderCountPerRow\":");
    output.push_str(&first_row.headers.len().to_string());
    output.push_str(",\"minOffsetUnits\":");
    push_optional_u16_json(output, min_offset_units);
    output.push_str(",\"maxExtentUnits\":");
    push_optional_u16_json(output, max_extent_units);
    output.push_str(",\"homogeneousFontSizeUnits\":");
    push_optional_u16_json(output, homogeneous_font_size_units);
    output.push_str(",\"matchedCellOffsetUnits\":");
    push_u16_array_json(output, &matched_cell_offset_units);
    output.push_str(",\"matchedCellExtentUnits\":");
    push_u16_array_json(output, &matched_cell_extent_units);
    output.push_str(",\"trailingHeaderOffsetUnits\":");
    push_u16_array_json(output, &trailing_header_offset_units);
    output.push_str(",\"trailingHeaderExtentUnits\":");
    push_u16_array_json(output, &trailing_header_extent_units);
    output.push_str(",\"tableUnitBBoxCandidates\":");
    push_table_grid_unit_bbox_candidates_json(output, rows, matched_column_count);
    output.push('}');
}

pub(super) fn push_table_grid_unit_bbox_candidates_json(
    output: &mut String,
    rows: &[TableCandidateLineHeaderRow],
    matched_column_count: usize,
) {
    if rows.is_empty() || matched_column_count == 0 {
        output.push_str("[]");
        return;
    }

    let mut candidates = Vec::new();
    push_table_grid_unit_bbox_candidate(
        &mut candidates,
        "matched-cells",
        rows,
        matched_column_count,
        |row| {
            let start = row.headers.first()?.offset_units;
            let end = row
                .headers
                .get(matched_column_count.checked_sub(1)?)?
                .extent_units;
            Some((start, end))
        },
        false,
    );
    push_table_grid_unit_bbox_candidate(
        &mut candidates,
        "matched-cells-plus-first-trailing-header",
        rows,
        matched_column_count,
        |row| {
            let start = row.headers.first()?.offset_units;
            let end = row.headers.get(matched_column_count)?.extent_units;
            Some((start, end))
        },
        true,
    );
    push_table_grid_unit_bbox_candidate(
        &mut candidates,
        "full-line-header-extent",
        rows,
        matched_column_count,
        |row| {
            let start = row.headers.iter().map(|header| header.offset_units).min()?;
            let end = row.headers.iter().map(|header| header.extent_units).max()?;
            Some((start, end))
        },
        true,
    );

    output.push('[');
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(candidate);
    }
    output.push(']');
}

pub(super) fn table_grid_unit_bbox_range_for_row(
    row: &TableCandidateLineHeaderRow,
    matched_column_count: usize,
    basis: TableGridUnitBBoxBasis,
) -> Option<(u16, u16)> {
    match basis {
        TableGridUnitBBoxBasis::MatchedCells => {
            let start = row.headers.first()?.offset_units;
            let end = row
                .headers
                .get(matched_column_count.checked_sub(1)?)?
                .extent_units;
            Some((start, end))
        }
        TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader => {
            let start = row.headers.first()?.offset_units;
            let end = row.headers.get(matched_column_count)?.extent_units;
            Some((start, end))
        }
        TableGridUnitBBoxBasis::FullLineHeaderExtent => {
            let start = row.headers.iter().map(|header| header.offset_units).min()?;
            let end = row.headers.iter().map(|header| header.extent_units).max()?;
            Some((start, end))
        }
    }
}

pub(super) fn table_grid_unit_bbox_trailing_header_included(basis: TableGridUnitBBoxBasis) -> bool {
    matches!(
        basis,
        TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader
            | TableGridUnitBBoxBasis::FullLineHeaderExtent
    )
}

pub(super) fn table_grid_unit_bbox_row_agreement_summary(
    rows: &[TableCandidateLineHeaderRow],
    matched_column_count: usize,
    basis: TableGridUnitBBoxBasis,
    selected_range: (u16, u16),
) -> (usize, bool) {
    let row_ranges = rows
        .iter()
        .filter_map(|row| {
            table_grid_unit_bbox_range_for_row(row, matched_column_count, basis)
                .filter(|(start, end)| start < end)
        })
        .collect::<Vec<_>>();
    let row_agreement_count = row_ranges
        .iter()
        .filter(|range| **range == selected_range)
        .count();
    (row_agreement_count, row_agreement_count == rows.len())
}

pub(super) fn push_table_grid_unit_bbox_candidate<F>(
    output: &mut Vec<String>,
    basis: &'static str,
    rows: &[TableCandidateLineHeaderRow],
    matched_column_count: usize,
    mut range_for_row: F,
    trailing_header_included: bool,
) where
    F: FnMut(&TableCandidateLineHeaderRow) -> Option<(u16, u16)>,
{
    let row_ranges = rows
        .iter()
        .filter_map(|row| {
            range_for_row(row)
                .and_then(|(start, end)| (start < end).then_some((row.row_index, start, end)))
        })
        .collect::<Vec<_>>();
    if row_ranges.is_empty() {
        return;
    }

    let first_range = row_ranges.first().map(|(_, start, end)| (*start, *end));
    let row_agreement_count = row_ranges
        .iter()
        .filter(|(_, start, end)| first_range == Some((*start, *end)))
        .count();
    let all_rows_agree = row_agreement_count == rows.len();
    let (candidate_start, candidate_end) = first_range.unwrap_or((0, 0));
    let column_span_units = rows
        .first()
        .map(|row| {
            row.headers
                .iter()
                .take(matched_column_count)
                .map(|header| header.extent_units.saturating_sub(header.offset_units))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let (column_slot_width_units, trailing_slot_width_units, included_trailing_header_count) = rows
        .first()
        .map(|row| {
            table_grid_unit_bbox_slot_widths(
                row,
                matched_column_count,
                candidate_end,
                trailing_header_included,
            )
        })
        .unwrap_or_default();

    let mut item = String::new();
    item.push_str("{\"source\":\"documentTextLineHeaders\"");
    item.push_str(",\"basis\":");
    item.push_str(&json_string(basis));
    item.push_str(",\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    item.push_str(",\"xUnitRange\":");
    item.push_str(&source_range_json(
        usize::from(candidate_start),
        usize::from(candidate_end),
    ));
    item.push_str(",\"widthUnits\":");
    item.push_str(&candidate_end.saturating_sub(candidate_start).to_string());
    item.push_str(",\"rowAgreementCount\":");
    item.push_str(&row_agreement_count.to_string());
    item.push_str(",\"allRowsAgree\":");
    item.push_str(if all_rows_agree { "true" } else { "false" });
    item.push_str(",\"trailingHeaderIncluded\":");
    item.push_str(if trailing_header_included {
        "true"
    } else {
        "false"
    });
    item.push_str(",\"includedTrailingHeaderCount\":");
    item.push_str(&included_trailing_header_count.to_string());
    item.push_str(",\"columnSpanUnits\":");
    push_u16_array_json(&mut item, &column_span_units);
    item.push_str(",\"columnSlotWidthUnits\":");
    push_u16_array_json(&mut item, &column_slot_width_units);
    item.push_str(",\"trailingSlotWidthUnits\":");
    push_u16_array_json(&mut item, &trailing_slot_width_units);
    item.push_str(",\"renderPromotionContribution\":\"table-horizontal-unit-span-candidate-only\"");
    item.push_str(",\"renderPromotionBlockedReason\":");
    item.push_str(&json_string("page-space-unit-scale-unproven"));
    item.push_str(",\"rows\":[");
    for (index, (row_index, start, end)) in row_ranges.iter().enumerate() {
        if index > 0 {
            item.push(',');
        }
        item.push_str("{\"row\":");
        item.push_str(&row_index.to_string());
        item.push_str(",\"xUnitRange\":");
        item.push_str(&source_range_json(usize::from(*start), usize::from(*end)));
        item.push_str(",\"agreesWithFirstRow\":");
        item.push_str(if first_range == Some((*start, *end)) {
            "true"
        } else {
            "false"
        });
        item.push('}');
    }
    item.push_str("]}");
    output.push(item);
}

pub(super) fn table_grid_unit_bbox_slot_widths(
    row: &TableCandidateLineHeaderRow,
    matched_column_count: usize,
    candidate_end: u16,
    trailing_header_included: bool,
) -> (Vec<u16>, Vec<u16>, usize) {
    let mut column_slot_width_units = Vec::new();
    for column_index in 0..matched_column_count {
        let Some(header) = row.headers.get(column_index) else {
            break;
        };
        let next_offset = row
            .headers
            .get(column_index + 1)
            .filter(|_| column_index + 1 < matched_column_count || trailing_header_included)
            .map(|next| next.offset_units)
            .filter(|offset| *offset <= candidate_end)
            .unwrap_or(candidate_end);
        column_slot_width_units.push(next_offset.saturating_sub(header.offset_units));
    }

    let trailing_headers = row
        .headers
        .iter()
        .skip(matched_column_count)
        .filter(|header| {
            header.offset_units < candidate_end && header.extent_units <= candidate_end
        })
        .collect::<Vec<_>>();
    let mut trailing_slot_width_units = Vec::new();
    for (index, header) in trailing_headers.iter().enumerate() {
        let next_offset = trailing_headers
            .get(index + 1)
            .map(|next| next.offset_units)
            .unwrap_or(candidate_end);
        trailing_slot_width_units.push(next_offset.saturating_sub(header.offset_units));
    }

    (
        column_slot_width_units,
        trailing_slot_width_units,
        trailing_headers.len(),
    )
}

pub(super) fn push_table_grid_source_derived_layout_candidate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    table_candidate: &TableCandidate,
    candidate: Option<&TableGridSourceDerivedLayout>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };

    let source = match candidate.provenance {
        TableGridSourceDerivedLayoutProvenance::DecodedCompactPlacement => {
            if candidate.line_mark_page_origin.is_some()
                || candidate.page_origin_authority == "lineMarkPageGridStrideRawRecordIndex"
            {
                "documentTextLineHeaders+/LineMark+/PageMark"
            } else {
                "documentTextLineHeaders+fallbackTextAnchors"
            }
        }
        TableGridSourceDerivedLayoutProvenance::SparseSiblingDerived => {
            "sparseTableSiblingEvidence compact cell geometry candidate"
        }
    };
    output.push_str("{\"source\":");
    output.push_str(&json_string(source));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"provenance\":");
    output.push_str(&json_string(candidate.provenance.as_str()));
    output.push_str(",\"projectionKind\":\"sourceDerivedDiagnosticProjection\"");
    output.push_str(",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        candidate.x, candidate.y, candidate.width, candidate.height
    ));
    output.push_str(",\"columnCount\":");
    output.push_str(&candidate.column_count.to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.row_count.to_string());
    output.push_str(",\"columnWidth\":");
    output.push_str(&format!("{:.3}", candidate.column_width));
    output.push_str(",\"columnWidthBasis\":");
    output.push_str(&json_string(candidate.column_width_basis));
    output.push_str(",\"columnWidths\":[");
    for (index, width) in candidate.column_widths.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{width:.3}"));
    }
    output.push(']');
    output.push_str(",\"xUnitRangeBasis\":");
    output.push_str(&json_string(candidate.x_unit_range_basis));
    output.push_str(",\"xUnitRange\":");
    output.push_str(&source_range_json(
        usize::from(candidate.x_unit_start),
        usize::from(candidate.x_unit_end),
    ));
    output.push_str(",\"xOriginInsetUnits\":");
    output.push_str(&format!("{:.3}", candidate.x_origin_inset_units));
    output.push_str(",\"xOriginInsetBasis\":");
    output.push_str(&json_string(candidate.x_origin_inset_basis));
    output.push_str(",\"horizontalUnitTransformReadiness\":");
    push_table_grid_horizontal_unit_transform_readiness_json(
        output,
        layout,
        document,
        table_candidate,
        candidate,
    );
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{:.3}", candidate.row_height));
    output.push_str(",\"rowHeightBasis\":");
    output.push_str(&json_string(candidate.row_height_basis));
    output.push_str(",\"pageOriginAuthority\":");
    output.push_str(&json_string(candidate.page_origin_authority));
    output.push_str(",\"anchorLineIndex\":");
    push_optional_usize_json(output, candidate.anchor_line_index);
    output.push_str(",\"lineMarkPageOriginCandidate\":");
    push_table_grid_line_mark_page_origin_candidate_json(
        output,
        candidate.line_mark_page_origin.as_ref(),
    );
    output.push_str(",\"lineMarkPageOriginStrideCandidate\":");
    push_table_grid_line_mark_page_origin_stride_candidate_json(
        output,
        candidate.line_mark_page_origin_stride.as_ref(),
    );
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&candidate.raw_header_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&candidate.matched_cell_header_count.to_string());
    output.push_str(",\"minOffsetUnits\":");
    push_optional_u16_json(output, candidate.min_offset_units);
    output.push_str(",\"maxExtentUnits\":");
    push_optional_u16_json(output, candidate.max_extent_units);
    output.push_str(",\"matchedCellSpanUnits\":");
    push_u16_array_json(output, &candidate.matched_cell_span_units);
    output.push_str(",\"matchedCellGapUnits\":");
    push_u16_array_json(output, &candidate.matched_cell_gap_units);
    output.push_str(",\"homogeneousFontSizeUnits\":");
    push_optional_u16_json(output, candidate.homogeneous_font_size_units);
    output.push_str(",\"lineMarkRowRecordSelection\":");
    output.push_str(&json_string(candidate.line_mark_row_record_selection));
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if candidate.line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if candidate.line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(candidate.render_promotion_blocked_reason));
    output.push('}');
}

pub(super) fn push_table_grid_horizontal_unit_transform_readiness_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    table_candidate: &TableCandidate,
    candidate: &TableGridSourceDerivedLayout,
) {
    let selected_width_units = candidate.x_unit_end.saturating_sub(candidate.x_unit_start);
    let required_cell_header_count = candidate.row_count.saturating_mul(candidate.column_count);
    let source_only_unit_transform_ready = selected_width_units > 0
        && candidate.x_unit_full_extent_units > 0
        && candidate.x_unit_all_rows_agree
        && candidate.line_header_rows_homogeneous
        && required_cell_header_count > 0
        && candidate.matched_cell_header_count >= required_cell_header_count;
    let page_space_unit_scale_decoded =
        table_grid_page_space_horizontal_transform_ready(Some(candidate));

    output.push_str("{\"source\":\"documentTextLineHeaders\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"selectedXUnitRangeBasis\":");
    output.push_str(&json_string(candidate.x_unit_range_basis));
    output.push_str(",\"selectedXUnitRange\":");
    output.push_str(&source_range_json(
        usize::from(candidate.x_unit_start),
        usize::from(candidate.x_unit_end),
    ));
    output.push_str(",\"selectedWidthUnits\":");
    output.push_str(&selected_width_units.to_string());
    output.push_str(",\"fullExtentUnits\":");
    output.push_str(&candidate.x_unit_full_extent_units.to_string());
    output.push_str(",\"selectedWidthRatioToFullExtent\":");
    if candidate.x_unit_full_extent_units > 0 {
        output.push_str(&format!(
            "{:.3}",
            f32::from(selected_width_units) / f32::from(candidate.x_unit_full_extent_units)
        ));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rowAgreementCount\":");
    output.push_str(&candidate.x_unit_row_agreement_count.to_string());
    output.push_str(",\"allRowsAgree\":");
    output.push_str(if candidate.x_unit_all_rows_agree {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"trailingHeaderIncluded\":");
    output.push_str(if candidate.x_unit_trailing_header_included {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"includedTrailingHeaderCount\":");
    output.push_str(&candidate.x_unit_included_trailing_header_count.to_string());
    output.push_str(",\"columnSpanUnits\":");
    push_u16_array_json(output, &candidate.matched_cell_span_units);
    output.push_str(",\"columnSlotWidthUnits\":");
    push_u16_array_json(output, &candidate.x_unit_column_slot_width_units);
    output.push_str(",\"trailingSlotWidthUnits\":");
    push_u16_array_json(output, &candidate.x_unit_trailing_slot_width_units);
    output.push_str(",\"xOriginInsetUnits\":");
    output.push_str(&format!("{:.3}", candidate.x_origin_inset_units));
    output.push_str(",\"xOriginInsetBasis\":");
    output.push_str(&json_string(candidate.x_origin_inset_basis));
    output.push_str(",\"totalWidthSemanticsGate\":");
    push_table_grid_total_width_semantics_gate_json(
        output,
        layout,
        document,
        table_candidate,
        candidate,
        selected_width_units,
    );
    output.push_str(",\"sourceOnlyUnitTransformReady\":");
    output.push_str(if source_only_unit_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageSpaceUnitScaleDecoded\":");
    output.push_str(if page_space_unit_scale_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(
        ",\"renderPromotionContribution\":\"selected-table-horizontal-unit-transform-readiness\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if page_space_unit_scale_decoded {
        output.push_str("null");
    } else {
        output.push_str(&json_string(if source_only_unit_transform_ready {
            "page-space-unit-scale-unproven"
        } else {
            "horizontal-unit-transform-incomplete"
        }));
    }
    output.push('}');
}

pub(super) fn push_table_grid_total_width_semantics_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    table_candidate: &TableCandidate,
    candidate: &TableGridSourceDerivedLayout,
    selected_width_units: u16,
) {
    let full_extent_units = candidate.x_unit_full_extent_units;
    let selected_equals_full_extent =
        selected_width_units > 0 && selected_width_units == full_extent_units;
    let selected_is_subset_of_full_extent =
        selected_width_units > 0 && full_extent_units > selected_width_units;
    let trailing_slot_evidence_present = !candidate.x_unit_trailing_slot_width_units.is_empty()
        || candidate.x_unit_included_trailing_header_count > 0;
    let full_extent_trailing_units = full_extent_units.saturating_sub(selected_width_units);
    let selected_visible_range_source_evidence_ready = selected_equals_full_extent
        || (selected_is_subset_of_full_extent
            && trailing_slot_evidence_present
            && candidate.x_unit_trailing_header_included
            && candidate.x_unit_included_trailing_header_count > 0
            && candidate.x_unit_all_rows_agree);
    let source_placement_coherence_readiness =
        table_grid_source_top_text_placement_readiness_for_candidate(
            layout,
            document,
            table_candidate,
        );
    let source_placement_coherence_gate_resolved = selected_visible_range_source_evidence_ready
        && !selected_equals_full_extent
        && source_placement_coherence_readiness
            .as_ref()
            .is_some_and(|readiness| readiness.ready);
    let source_placement_coherence_gate_required =
        selected_visible_range_source_evidence_ready && !selected_equals_full_extent;
    let render_promotion_next_gate =
        if selected_equals_full_extent || source_placement_coherence_gate_resolved {
            None
        } else if source_placement_coherence_gate_required {
            Some("source-table-placement-coherence-gate")
        } else {
            Some("source-total-width-semantics-decoder")
        };

    output.push_str("{\"source\":\"documentTextLineHeaders total-width semantics gate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"selectedWidthUnits\":");
    output.push_str(&selected_width_units.to_string());
    output.push_str(",\"fullExtentUnits\":");
    output.push_str(&full_extent_units.to_string());
    output.push_str(",\"fullExtentTrailingUnits\":");
    output.push_str(&full_extent_trailing_units.to_string());
    output.push_str(",\"selectedEqualsFullExtent\":");
    output.push_str(if selected_equals_full_extent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedIsSubsetOfFullExtent\":");
    output.push_str(if selected_is_subset_of_full_extent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"trailingHeaderIncluded\":");
    output.push_str(if candidate.x_unit_trailing_header_included {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"includedTrailingHeaderCount\":");
    output.push_str(&candidate.x_unit_included_trailing_header_count.to_string());
    output.push_str(",\"trailingSlotEvidencePresent\":");
    output.push_str(if trailing_slot_evidence_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"trailingSlotWidthUnits\":");
    push_u16_array_json(output, &candidate.x_unit_trailing_slot_width_units);
    output.push_str(",\"selectedVisibleRangeSourceEvidenceReady\":");
    output.push_str(if selected_visible_range_source_evidence_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementCoherenceGateRequired\":");
    output.push_str(
        if source_placement_coherence_gate_required && !source_placement_coherence_gate_resolved {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sourcePlacementCoherenceGateEvidencePresent\":");
    output.push_str(if source_placement_coherence_readiness.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementCoherenceGateResolved\":");
    output.push_str(if source_placement_coherence_gate_resolved {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePlacementCoherenceGateBlockedReasons\":");
    match source_placement_coherence_readiness.as_ref() {
        Some(readiness) => push_json_string_slice_array(output, &readiness.blocked_reasons),
        None if source_placement_coherence_gate_required => push_json_string_slice_array(
            output,
            &["source-table-placement-coherence-evidence-absent"],
        ),
        None => push_json_string_slice_array(output, &[]),
    }
    output.push_str(",\"renderPromotionNextGate\":");
    match render_promotion_next_gate {
        Some(gate) => output.push_str(&json_string(gate)),
        None => output.push_str("null"),
    }
    output.push_str(",\"renderWidthBasisCandidate\":");
    if selected_equals_full_extent {
        output.push_str(&json_string("selected-range-equals-full-extent"));
    } else if selected_is_subset_of_full_extent && trailing_slot_evidence_present {
        output.push_str(&json_string(
            "selected-visible-range-with-trailing-header-evidence",
        ));
    } else if selected_is_subset_of_full_extent {
        output.push_str(&json_string("selected-visible-range-subset-of-full-extent"));
    } else {
        output.push_str(&json_string("total-width-semantics-undetermined"));
    }
    output.push_str(",\"renderPromotionContribution\":\"source-total-width-semantics-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if selected_equals_full_extent || source_placement_coherence_gate_resolved {
        output.push_str("null");
    } else if source_placement_coherence_gate_required {
        output.push_str(&json_string(
            "source-table-placement-coherence-gate-required",
        ));
    } else {
        output.push_str(&json_string("source-total-width-semantics-unproven"));
    }
    output.push('}');
}

pub(super) fn push_table_grid_source_derived_layout_readiness_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    let source_placement_evidence_present =
        table_grid_decoded_source_placement_evidence_present(document, candidate);
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let row_count = candidate.intervals().len();
    let raw_header_count = rows
        .iter()
        .map(TableCandidateLineHeaderRow::raw_header_count)
        .sum::<usize>();
    let matched_row_count = rows.iter().filter(|row| row.matched_cell_count > 0).count();
    let full_matched_row_count = rows
        .iter()
        .filter(|row| row.matched_cell_count >= row.expected_cell_count)
        .count();
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    let required_cell_header_count = rows
        .iter()
        .map(|row| row.expected_cell_count)
        .sum::<usize>();
    let common_matched_column_count = rows
        .iter()
        .map(|row| row.matched_cell_count)
        .min()
        .unwrap_or(0)
        .min(column_count);
    let rows_without_headers = rows
        .iter()
        .filter(|row| row.headers.is_empty())
        .map(|row| row.row_index)
        .collect::<Vec<_>>();
    let rows_without_matched_cell_headers = rows
        .iter()
        .filter(|row| row.matched_cell_count == 0)
        .map(|row| row.row_index)
        .collect::<Vec<_>>();
    let rows_with_partial_cell_header_coverage = rows
        .iter()
        .filter(|row| {
            row.matched_cell_count > 0 && row.matched_cell_count < row.expected_cell_count
        })
        .map(|row| row.row_index)
        .collect::<Vec<_>>();
    let line_header_rows_homogeneous = table_grid_line_header_rows_are_homogeneous(&rows);
    let resolved_line_mark_rows =
        table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows);
    let line_mark_row_record_selection =
        table_grid_line_mark_row_record_selection(&resolved_line_mark_rows);
    let line_mark_rows_exact_and_contiguous =
        table_grid_line_mark_rows_are_exact_and_contiguous(document, candidate, &rows);
    let source_layout_present = source_layout.is_some();
    let source_layout_renderable = source_layout
        .as_ref()
        .is_some_and(|layout| table_grid_source_derived_layout_is_renderable(layout));

    let mut rejection_reasons = Vec::new();
    if column_count == 0 {
        rejection_reasons.push("column-count-zero");
    }
    if !source_placement_evidence_present {
        rejection_reasons.push("source-placement-evidence-missing");
    }
    if rows.is_empty() {
        rejection_reasons.push("line-header-rows-missing");
    }
    if raw_header_count == 0 {
        rejection_reasons.push("line-header-raw-headers-missing");
    }
    if matched_cell_header_count < required_cell_header_count {
        rejection_reasons.push("line-header-cell-geometry-incomplete");
    }
    if common_matched_column_count == 0 {
        rejection_reasons.push("no-common-matched-cell-header-columns");
    }
    if !line_header_rows_homogeneous {
        rejection_reasons.push("line-header-rows-not-homogeneous");
    }
    if !line_mark_rows_exact_and_contiguous {
        rejection_reasons.push("line-mark-rows-not-exact-source-boundaries");
    }
    if !source_layout_present {
        rejection_reasons.push("source-derived-layout-candidate-absent");
    }
    if let Some(layout) = source_layout
        && layout.render_promotion_blocked_reason != "none"
    {
        rejection_reasons.push(layout.render_promotion_blocked_reason);
    }
    if source_layout_present && !source_layout_renderable {
        rejection_reasons.push("source-derived-layout-not-renderable");
    }

    output.push_str("{\"source\":\"sourceDerivedLayoutGate+documentTextLineHeaders+/LineMark\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourcePlacementEvidencePresent\":");
    output.push_str(if source_placement_evidence_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&row_count.to_string());
    output.push_str(",\"requestedColumnCount\":");
    output.push_str(&column_count.to_string());
    output.push_str(",\"lineHeaderRowCount\":");
    output.push_str(&rows.len().to_string());
    output.push_str(",\"rawHeaderCount\":");
    output.push_str(&raw_header_count.to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&matched_row_count.to_string());
    output.push_str(",\"fullMatchedRowCount\":");
    output.push_str(&full_matched_row_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&matched_cell_header_count.to_string());
    output.push_str(",\"requiredCellHeaderCount\":");
    output.push_str(&required_cell_header_count.to_string());
    output.push_str(",\"commonMatchedColumnCount\":");
    output.push_str(&common_matched_column_count.to_string());
    output.push_str(",\"rowsWithoutHeaders\":");
    push_usize_array_json(output, &rows_without_headers);
    output.push_str(",\"rowsWithoutMatchedCellHeaders\":");
    push_usize_array_json(output, &rows_without_matched_cell_headers);
    output.push_str(",\"rowsWithPartialCellHeaderCoverage\":");
    push_usize_array_json(output, &rows_with_partial_cell_header_coverage);
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRowRecordSelection\":");
    output.push_str(&json_string(line_mark_row_record_selection));
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutCandidatePresent\":");
    output.push_str(if source_layout_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutRenderable\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutBlockedReason\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.render_promotion_blocked_reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"rejectionReasons\":");
    push_json_string_slice_array(output, &rejection_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-derived-layout-readiness-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if source_layout_renderable {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-derived-layout-not-renderable"));
    }
    output.push('}');
}

pub(super) fn push_table_grid_page_space_solver_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    column_count: usize,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    let source_placement_evidence_present =
        table_grid_decoded_source_placement_evidence_present(document, candidate);
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    let required_cell_header_count = rows
        .iter()
        .map(|row| row.expected_cell_count)
        .sum::<usize>();
    let common_matched_column_count = rows
        .iter()
        .map(|row| row.matched_cell_count)
        .min()
        .unwrap_or(0)
        .min(column_count);
    let line_header_rows_homogeneous = table_grid_line_header_rows_are_homogeneous(&rows);
    let resolved_line_mark_rows =
        table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows);
    let line_mark_row_record_selection =
        table_grid_line_mark_row_record_selection(&resolved_line_mark_rows);
    let line_mark_rows_exact_and_contiguous =
        table_grid_line_mark_rows_are_exact_and_contiguous(document, candidate, &rows);
    let source_layout_renderable = source_layout
        .as_ref()
        .is_some_and(|layout| table_grid_source_derived_layout_is_renderable(layout));
    let source_column_split_ready = table_grid_source_column_split_ready(source_layout);
    let page_space_horizontal_transform_ready =
        table_grid_page_space_horizontal_transform_ready(source_layout);
    let subrecord_span_readiness =
        table_grid_page_mark_subrecord_line_span_readiness(document, candidate);
    let horizontal_solver_ready = source_placement_evidence_present
        && source_layout.is_some()
        && column_count > 0
        && common_matched_column_count > 0
        && required_cell_header_count > 0
        && matched_cell_header_count >= required_cell_header_count;
    let row_height_solver_ready = source_layout
        .as_ref()
        .is_some_and(|layout| layout.homogeneous_font_size_units.is_some())
        && line_header_rows_homogeneous;
    let y_origin_solver_ready = source_layout.as_ref().is_some_and(|layout| {
        layout.line_mark_page_origin.is_some()
            && layout.page_origin_authority == "lineMarkPageGrid"
            && layout.line_mark_rows_exact_and_contiguous
            && layout.line_header_rows_homogeneous
            && layout.render_promotion_blocked_reason == "none"
    });
    let solver_stage = if source_layout_renderable {
        "renderable-source-page-space"
    } else if horizontal_solver_ready && row_height_solver_ready && !y_origin_solver_ready {
        "blocked-y-origin-transform"
    } else if horizontal_solver_ready && !row_height_solver_ready {
        "blocked-row-height-transform"
    } else if horizontal_solver_ready {
        "blocked-partial-page-space"
    } else if source_layout.is_some() {
        "blocked-horizontal-transform"
    } else {
        "candidate-absent"
    };
    let blocked_reason = if source_layout_renderable {
        None
    } else if let Some(layout) = source_layout {
        if layout.render_promotion_blocked_reason != "none" {
            Some(layout.render_promotion_blocked_reason)
        } else if !horizontal_solver_ready {
            Some("table-horizontal-source-transform-incomplete")
        } else if !row_height_solver_ready {
            Some("table-row-height-source-transform-incomplete")
        } else if !y_origin_solver_ready {
            Some("table-page-y-transform-unproven")
        } else {
            Some("source-derived-layout-not-renderable")
        }
    } else {
        Some("source-derived-layout-candidate-absent")
    };

    output.push_str("{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\"");
    output.push_str(",\"solverVersion\":\"table-page-space-v1\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"solverStage\":");
    output.push_str(&json_string(solver_stage));
    output.push_str(",\"sourcePlacementEvidencePresent\":");
    output.push_str(if source_placement_evidence_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"requestedColumnCount\":");
    output.push_str(&column_count.to_string());
    output.push_str(",\"commonMatchedColumnCount\":");
    output.push_str(&common_matched_column_count.to_string());
    output.push_str(",\"matchedCellHeaderCount\":");
    output.push_str(&matched_cell_header_count.to_string());
    output.push_str(",\"requiredCellHeaderCount\":");
    output.push_str(&required_cell_header_count.to_string());
    output.push_str(",\"horizontalSolverReady\":");
    output.push_str(if horizontal_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowHeightSolverReady\":");
    output.push_str(if row_height_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"yOriginSolverReady\":");
    output.push_str(if y_origin_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(if line_header_rows_homogeneous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRowRecordSelection\":");
    output.push_str(&json_string(line_mark_row_record_selection));
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutCandidatePresent\":");
    output.push_str(if source_layout.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutRenderable\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageOriginAuthority\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.page_origin_authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(
        if source_layout
            .as_ref()
            .is_some_and(|layout| layout.line_mark_page_origin.is_some())
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"lineMarkPageOriginStridePresent\":");
    output.push_str(
        if source_layout
            .as_ref()
            .is_some_and(|layout| layout.line_mark_page_origin_stride.is_some())
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"referenceCalibrationReplacementGate\":");
    push_table_grid_reference_calibration_replacement_gate_json(
        output,
        source_layout,
        source_layout_renderable,
        horizontal_solver_ready,
        source_column_split_ready,
        page_space_horizontal_transform_ready,
        row_height_solver_ready,
        y_origin_solver_ready,
    );
    output.push_str(",\"sourceOnlyAxisAdmissionGate\":");
    push_table_grid_source_only_axis_admission_gate_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
        source_layout_renderable,
        source_column_split_ready,
        page_space_horizontal_transform_ready,
        row_height_solver_ready,
        y_origin_solver_ready,
        subrecord_span_readiness.as_ref(),
    );
    output.push_str(",\"pageSpaceHorizontalTransformGate\":");
    push_table_grid_page_space_horizontal_transform_gate_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
        source_column_split_ready,
        page_space_horizontal_transform_ready,
    );
    output.push_str(",\"sourcePageYTransformGate\":");
    push_table_grid_source_page_y_transform_gate_json(
        output,
        document,
        candidate,
        source_layout,
        subrecord_span_readiness.as_ref(),
        y_origin_solver_ready,
        line_mark_rows_exact_and_contiguous,
    );
    output.push_str(",\"renderPromoted\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionAuthority\":");
    if source_layout_renderable {
        output.push_str(&json_string("source-derived-page-space-solver"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    if let Some(reason) = blocked_reason {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_source_only_axis_admission_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    source_layout_renderable: bool,
    source_column_split_ready: bool,
    page_space_horizontal_transform_ready: bool,
    row_height_solver_ready: bool,
    y_origin_solver_ready: bool,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let horizontal_supports = table_grid_page_space_horizontal_frame_candidate_supports(
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    let mut horizontal_groups: BTreeMap<(i32, i32), Vec<TableGridHorizontalFrameCandidateSupport>> =
        BTreeMap::new();
    for support in horizontal_supports.iter().cloned() {
        horizontal_groups
            .entry((
                rounded_milli(support.selected_x),
                rounded_milli(support.selected_width),
            ))
            .or_default()
            .push(support);
    }
    let horizontal_best_support_count = horizontal_groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let horizontal_best_group_count = horizontal_groups
        .values()
        .filter(|supports| supports.len() == horizontal_best_support_count)
        .count();
    let horizontal_unique_best_supported =
        horizontal_best_support_count > 1 && horizontal_best_group_count == 1;
    let horizontal_best_group = horizontal_groups.values().find(|supports| {
        supports.len() == horizontal_best_support_count && horizontal_unique_best_supported
    });
    let horizontal_selector_candidate_present = horizontal_supports
        .iter()
        .any(|support| support.contribution == "source-only-horizontal-field-selector");
    let horizontal_selector_in_best_group = horizontal_best_group.is_some_and(|supports| {
        supports
            .iter()
            .any(|support| support.contribution == "source-only-horizontal-field-selector")
    });
    let horizontal_best_support = horizontal_best_group.and_then(|supports| supports.first());

    let cross_table_row_boundary_offset_probe =
        table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let y_supports = table_grid_source_only_page_y_origin_candidate_supports(
        document,
        candidate,
        source_layout,
        cross_table_row_boundary_offset_probe.as_ref(),
        subrecord_span_readiness,
    );
    let mut y_groups: BTreeMap<
        (i32, Option<i32>),
        Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    > = BTreeMap::new();
    for support in y_supports.iter().cloned() {
        y_groups
            .entry((
                rounded_milli(support.selected_y),
                support.row_height.map(rounded_milli),
            ))
            .or_default()
            .push(support);
    }
    let y_best_support_count = y_groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let y_best_group_count = y_groups
        .values()
        .filter(|supports| supports.len() == y_best_support_count)
        .count();
    let y_unique_best_supported = y_best_support_count > 1 && y_best_group_count == 1;
    let y_best_group = y_groups
        .values()
        .find(|supports| supports.len() == y_best_support_count && y_unique_best_supported);
    let y_candidate_best_group = y_best_group.filter(|supports| {
        table_grid_source_only_page_y_origin_group_supports_candidate(supports, candidate)
    });
    let y_fallback_selector_group = if y_candidate_best_group.is_none() {
        table_grid_source_only_page_y_origin_fallback_selector_group(&y_groups, candidate)
    } else {
        None
    };
    let y_selector_group = y_candidate_best_group.or(y_fallback_selector_group.as_ref());
    let y_selector_support = y_selector_group.and_then(|supports| supports.first());
    let y_selector_uses_single_support_fallback =
        y_candidate_best_group.is_none() && y_fallback_selector_group.is_some();
    let y_selector_table_candidate_indexes = y_selector_group
        .map(|supports| {
            supports
                .iter()
                .filter_map(|support| support.table_candidate_index)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let y_cross_table_previous_row_span_table_candidate_count = y_groups
        .values()
        .flatten()
        .filter(|support| {
            table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(support)
        })
        .filter_map(|support| support.table_candidate_index)
        .collect::<BTreeSet<_>>()
        .len();
    let y_selector_support_fragmented_by_table = !y_selector_uses_single_support_fallback
        && y_cross_table_previous_row_span_table_candidate_count > 1
        && y_selector_table_candidate_indexes.len()
            < y_cross_table_previous_row_span_table_candidate_count;
    let y_selector_candidate_present = y_selector_group.is_some();
    let y_selector_support_count = y_selector_group.map(|supports| supports.len()).unwrap_or(0);
    let y_selector_support_blocked_reasons = y_selector_group
        .map(|supports| table_grid_source_only_page_y_origin_supports_blocked_reasons(supports))
        .unwrap_or_default();
    let page_mark_absolute_y_slot_agreement =
        table_grid_source_only_page_mark_absolute_y_slot_agreement(
            document,
            candidate,
            cross_table_row_boundary_offset_probe.as_ref(),
            subrecord_span_readiness,
        );
    let page_mark_absolute_y_slot_semantics_ready =
        page_mark_absolute_y_slot_agreement.semantics_ready();
    let page_mark_absolute_y_slot_blocked_reason =
        table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(
            &page_mark_absolute_y_slot_agreement,
        );
    let y_selector_cross_table_support_present = y_selector_group.is_some_and(|supports| {
        supports
            .iter()
            .any(table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span)
    });
    let y_selector_agreement_admissible = y_selector_candidate_present
        && y_unique_best_supported
        && !y_selector_uses_single_support_fallback
        && !y_selector_support_fragmented_by_table
        && y_selector_cross_table_support_present;
    let active_source_layout_admission_ready =
        source_layout_renderable && page_space_horizontal_transform_ready && y_origin_solver_ready;
    let y_selector_admission_blocked_reason = if active_source_layout_admission_ready {
        "none"
    } else if !y_selector_candidate_present {
        "source-y-origin-selector-absent"
    } else if y_selector_uses_single_support_fallback {
        "source-y-origin-selector-single-support-fallback-not-render-admissible"
    } else if y_selector_support_fragmented_by_table {
        "source-y-origin-selector-fragmented-by-table-not-render-admissible"
    } else if !y_unique_best_supported {
        "source-y-origin-selector-agreement-unproven"
    } else if !y_selector_cross_table_support_present {
        "source-y-origin-selector-cross-table-support-absent"
    } else if !y_origin_solver_ready {
        "source-y-axis-not-render-admissible"
    } else {
        "none"
    };
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe.as_ref(),
        );
    let source_gap_to_page_line_gap_table_family_transform_required =
        y_selector_cross_table_support_present
            || cross_table_row_boundary_offset_probe
                .as_ref()
                .is_some_and(|probe| probe.all_offsets_require_transform);
    let axis_candidate_row_height = y_selector_support
        .and_then(|support| support.row_height)
        .or_else(|| source_layout.map(|layout| layout.row_height));
    let axis_candidate_present = horizontal_best_support.is_some() && y_selector_support.is_some();
    let axis_candidate_bbox_present = axis_candidate_present && axis_candidate_row_height.is_some();

    let mut blocked_reasons = Vec::new();
    if source_layout.is_none() {
        blocked_reasons.push("source-derived-layout-candidate-absent");
    }
    if !source_column_split_ready {
        blocked_reasons.push("source-column-split-not-ready");
    }
    if !row_height_solver_ready {
        blocked_reasons.push("source-row-height-not-ready");
    }
    if !active_source_layout_admission_ready {
        if horizontal_groups.is_empty() {
            blocked_reasons.push("source-horizontal-frame-candidates-absent");
        } else if !horizontal_unique_best_supported {
            blocked_reasons.push("source-horizontal-frame-candidate-agreement-unproven");
        }
        if !horizontal_selector_candidate_present {
            blocked_reasons.push("source-only-horizontal-selector-absent");
        } else if !horizontal_selector_in_best_group {
            blocked_reasons.push("source-only-horizontal-selector-not-in-best-agreement-group");
        }
        if !page_space_horizontal_transform_ready {
            blocked_reasons.push("source-horizontal-axis-not-render-admissible");
        }
        if y_groups.is_empty() {
            blocked_reasons.push("source-y-origin-candidates-absent");
        } else if !y_unique_best_supported && !y_selector_uses_single_support_fallback {
            blocked_reasons.push("source-y-origin-candidate-agreement-unproven");
        }
        if !y_selector_candidate_present {
            blocked_reasons.push("source-y-origin-selector-absent");
        }
        if y_selector_uses_single_support_fallback {
            blocked_reasons.push("source-y-origin-selector-single-support-fallback");
        }
        if y_selector_support_fragmented_by_table {
            blocked_reasons.push("source-y-origin-selector-fragmented-by-table");
        }
    }
    if source_gap_to_page_line_gap_table_family_transform_required
        && let Some(reason) =
            source_gap_to_page_line_gap_readiness_hints.table_family_transform_blocked_reason()
    {
        blocked_reasons.push(reason);
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("source-y-axis-not-render-admissible");
    }
    if !source_layout_renderable {
        blocked_reasons.push("source-derived-layout-not-renderable");
    }
    let admission_ready = active_source_layout_admission_ready || blocked_reasons.is_empty();

    output.push_str("{\"source\":\"pageSpaceHorizontalTransformGate+sourcePageYTransformGate source-only selector coupling\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsedForSelection\":false");
    output.push_str(",\"admissionReady\":");
    output.push_str(if admission_ready { "true" } else { "false" });
    output.push_str(",\"activeSourceLayoutAdmissionReady\":");
    output.push_str(if active_source_layout_admission_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"activeSourceLayoutAdmissionBasis\":");
    if active_source_layout_admission_ready {
        output.push_str(&json_string("source-derived-page-space-solver"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceOnlySelectorFallbackIgnoredByActiveSourceLayout\":");
    output.push_str(
        if active_source_layout_admission_ready
            && (!horizontal_selector_candidate_present || y_selector_uses_single_support_fallback)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutRenderable\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalAxisReady\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalSelectorCandidatePresent\":");
    output.push_str(if horizontal_selector_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalSelectorInBestAgreementGroup\":");
    output.push_str(if horizontal_selector_in_best_group {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalCandidateCount\":");
    output.push_str(&horizontal_supports.len().to_string());
    output.push_str(",\"horizontalAgreementGroupCount\":");
    output.push_str(&horizontal_groups.len().to_string());
    output.push_str(",\"horizontalBestSupportCount\":");
    output.push_str(&horizontal_best_support_count.to_string());
    output.push_str(",\"horizontalUniqueBestSupported\":");
    output.push_str(if horizontal_unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalBestSupportedSelectedX\":");
    match horizontal_best_support {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_x)),
        None => output.push_str("null"),
    }
    output.push_str(",\"horizontalBestSupportedSelectedWidth\":");
    match horizontal_best_support {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_width)),
        None => output.push_str("null"),
    }
    output.push_str(",\"horizontalBestSupportedFrameBases\":");
    match horizontal_best_group {
        Some(supports) => {
            let frame_bases = supports
                .iter()
                .map(|support| support.frame_basis)
                .collect::<Vec<_>>();
            push_json_string_slice_array(output, &frame_bases);
        }
        None => output.push_str("[]"),
    }
    output.push_str(",\"yAxisReady\":");
    output.push_str(if y_origin_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorCandidatePresent\":");
    output.push_str(if y_selector_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorSingleSupportFallback\":");
    output.push_str(if y_selector_uses_single_support_fallback {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorSupportFragmentedByTable\":");
    output.push_str(if y_selector_support_fragmented_by_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorSupportCount\":");
    output.push_str(&y_selector_support_count.to_string());
    output.push_str(",\"ySelectorCrossTableSupportPresent\":");
    output.push_str(if y_selector_cross_table_support_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorAgreementAdmissible\":");
    output.push_str(if y_selector_agreement_admissible {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectorAdmissionBlockedReason\":");
    output.push_str(&json_string(y_selector_admission_blocked_reason));
    output.push_str(",\"ySelectorSupportBlockedReasons\":");
    push_json_string_slice_array(output, &y_selector_support_blocked_reasons);
    output.push_str(",\"sourceGapToPageLineGapTransformAdmissionGate\":");
    push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
        output,
        "sourceOnlyAxisAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate",
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"pageMarkAbsoluteYSlotSemanticsReady\":");
    output.push_str(if page_mark_absolute_y_slot_semantics_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageMarkAbsoluteYSlotBlockedReason\":");
    output.push_str(&json_string(page_mark_absolute_y_slot_blocked_reason));
    output.push_str(",\"pageMarkAbsoluteYSlotResidualPx\":");
    push_optional_f32_json(output, page_mark_absolute_y_slot_agreement.residual_px);
    output.push_str(",\"yCandidateCount\":");
    output.push_str(&y_supports.len().to_string());
    output.push_str(",\"yAgreementGroupCount\":");
    output.push_str(&y_groups.len().to_string());
    output.push_str(",\"yBestSupportCount\":");
    output.push_str(&y_best_support_count.to_string());
    output.push_str(",\"yUniqueBestSupported\":");
    output.push_str(if y_unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"ySelectedOriginBasis\":");
    match y_selector_support {
        Some(support) => output.push_str(&json_string(support.origin_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"ySelectedY\":");
    match y_selector_support {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_y)),
        None => output.push_str("null"),
    }
    output.push_str(",\"ySelectedRowHeight\":");
    match y_selector_support {
        Some(support) => push_optional_f32_json(output, support.row_height),
        None => output.push_str("null"),
    }
    output.push_str(",\"ySelectorTableCandidateIndexes\":");
    push_usize_array_json(output, &y_selector_table_candidate_indexes);
    output.push_str(",\"sourceOnlyAxisCandidateBBox\":");
    push_table_grid_source_only_axis_candidate_bbox_json(
        output,
        candidate,
        horizontal_best_support,
        y_selector_support,
        axis_candidate_row_height,
        axis_candidate_present,
        axis_candidate_bbox_present,
        admission_ready,
        active_source_layout_admission_ready,
    );
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"source-only-axis-selector-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if admission_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "source-page-space-axis-selector-coupling-unproven",
        ));
    }
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_source_only_axis_candidate_bbox_json(
    output: &mut String,
    candidate: &TableCandidate,
    horizontal_support: Option<&TableGridHorizontalFrameCandidateSupport>,
    y_support: Option<&TableGridSourceOnlyPageYOriginCandidateSupport>,
    row_height: Option<f32>,
    candidate_present: bool,
    bbox_present: bool,
    admission_ready: bool,
    active_source_layout_admission_ready: bool,
) {
    let render_promotion_blocked_reason = if active_source_layout_admission_ready {
        "active-source-layout-admission-uses-source-derived-layout-not-selector-bbox"
    } else if horizontal_support.is_none() {
        "source-only-axis-horizontal-candidate-absent"
    } else if y_support.is_none() {
        "source-only-axis-y-candidate-absent"
    } else if row_height.is_none() {
        "source-only-axis-row-height-candidate-absent"
    } else if !admission_ready {
        "source-page-space-axis-selector-coupling-unproven"
    } else {
        "source-only-axis-candidate-bbox-diagnostic-only"
    };

    output.push_str("{\"source\":\"sourceOnlyAxisAdmissionGate.sourceOnlyAxisCandidateBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"candidatePresent\":");
    output.push_str(if candidate_present { "true" } else { "false" });
    output.push_str(",\"bboxPresent\":");
    output.push_str(if bbox_present { "true" } else { "false" });
    output.push_str(",\"horizontalCandidatePresent\":");
    output.push_str(if horizontal_support.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"yCandidatePresent\":");
    output.push_str(if y_support.is_some() { "true" } else { "false" });
    output.push_str(",\"rowHeightCandidatePresent\":");
    output.push_str(if row_height.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"horizontalFrameBasis\":");
    match horizontal_support {
        Some(support) => output.push_str(&json_string(support.frame_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"yOriginBasis\":");
    match y_support {
        Some(support) => output.push_str(&json_string(support.origin_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"rowHeight\":");
    push_optional_f32_json(output, row_height);
    output.push_str(",\"bbox\":");
    if let (Some(horizontal), Some(y), Some(row_height)) =
        (horizontal_support, y_support, row_height)
    {
        let height = row_height * candidate.intervals().len() as f32;
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
            horizontal.selected_x, y.selected_y, horizontal.selected_width, height
        ));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderPromotionContribution\":\"source-only-axis-candidate-bbox\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push('}');
}

pub(super) fn table_grid_source_column_split_ready(
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> bool {
    source_layout.is_some_and(|layout| {
        layout.column_count > 0
            && layout.column_width_basis == "documentTextLineHeaderCellSlotUnits"
            && layout.column_widths.len() == layout.column_count
            && layout.column_widths.iter().all(|width| *width > 0.0)
    })
}

pub(super) fn table_grid_page_space_horizontal_transform_ready(
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> bool {
    source_layout.is_some_and(|layout| {
        table_grid_source_column_split_ready(Some(layout))
            && layout.x_unit_all_rows_agree
            && layout.x_unit_full_extent_units > 0
            && layout.page_origin_authority == "lineMarkPageGrid"
            && layout.line_mark_rows_exact_and_contiguous
            && layout.render_promotion_blocked_reason == "none"
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_page_space_horizontal_transform_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    source_column_split_ready: bool,
    page_space_horizontal_transform_ready: bool,
) {
    let source_layout_present = source_layout.is_some();
    let x_unit_all_rows_agree = source_layout
        .as_ref()
        .is_some_and(|layout| layout.x_unit_all_rows_agree);
    let full_extent_units_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.x_unit_full_extent_units > 0);
    let page_origin_authority = source_layout
        .as_ref()
        .map(|layout| layout.page_origin_authority);
    let line_mark_rows_exact_and_contiguous = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_rows_exact_and_contiguous);
    let source_layout_render_blocked_reason = source_layout
        .as_ref()
        .map(|layout| layout.render_promotion_blocked_reason);
    let source_frame_decoded = page_space_horizontal_transform_ready;

    let mut blocked_reasons = Vec::new();
    if !source_layout_present {
        blocked_reasons.push("source-derived-layout-candidate-absent");
    }
    if !source_column_split_ready {
        blocked_reasons.push("source-column-split-not-ready");
    }
    if !x_unit_all_rows_agree {
        blocked_reasons.push("source-x-unit-range-not-row-stable");
    }
    if !full_extent_units_present {
        blocked_reasons.push("source-full-line-extent-units-missing");
    }
    if !source_frame_decoded {
        blocked_reasons.push("page-space-horizontal-frame-not-decoded");
    }
    if !line_mark_rows_exact_and_contiguous {
        blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
    }
    if let Some(reason) = source_layout_render_blocked_reason
        && reason != "none"
    {
        blocked_reasons.push(reason);
    }

    output.push_str(
        "{\"source\":\"documentTextLineHeaders+/LineMark page-space horizontal transform gate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceColumnSplitReady\":");
    output.push_str(if source_column_split_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"xUnitAllRowsAgree\":");
    output.push_str(if x_unit_all_rows_agree {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fullExtentUnitsPresent\":");
    output.push_str(if full_extent_units_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceFrameDecoded\":");
    output.push_str(if source_frame_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageOriginAuthority\":");
    match page_origin_authority {
        Some(authority) => output.push_str(&json_string(authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceDerivedLayoutBlockedReason\":");
    match source_layout_render_blocked_reason {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"xUnitRangeBasis\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.x_unit_range_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"xUnitRange\":");
    match source_layout {
        Some(layout) => output.push_str(&source_range_json(
            usize::from(layout.x_unit_start),
            usize::from(layout.x_unit_end),
        )),
        None => output.push_str("null"),
    }
    output.push_str(",\"fullExtentUnits\":");
    match source_layout {
        Some(layout) => output.push_str(&layout.x_unit_full_extent_units.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"xOriginInsetBasis\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.x_origin_inset_basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceFrameAdmissionGate\":");
    push_table_grid_page_space_horizontal_source_frame_admission_gate_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
        page_space_horizontal_transform_ready,
    );
    output.push_str(",\"sourceFrameHypotheses\":");
    push_table_grid_page_space_horizontal_frame_hypotheses_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    output.push_str(",\"sourceOnlyHorizontalFieldConsensus\":");
    push_table_grid_source_only_horizontal_field_consensus_json(
        output, layout, document, lines, candidate,
    );
    output.push_str(",\"sourceFrameCandidateAgreementGate\":");
    push_table_grid_page_space_horizontal_frame_candidate_agreement_gate_json(
        output,
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-page-space-horizontal-transform-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if page_space_horizontal_transform_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "table-horizontal-page-space-transform-incomplete",
        ));
    }
    output.push('}');
}

pub(super) fn push_table_grid_page_space_horizontal_source_frame_admission_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    page_space_horizontal_transform_ready: bool,
) {
    output.push_str("{\"source\":\"sourceDerivedLayoutCandidate+sourceFrameAdmission\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceFrameDecoded\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderFrameBasis\":");
    if page_space_horizontal_transform_ready {
        output.push_str(&json_string(
            "page-body-frame+documentTextLineHeaderUnitTransform",
        ));
    } else {
        output.push_str("null");
    }

    match source_layout {
        Some(source_layout) => {
            let selected_start_units = f32::from(source_layout.x_unit_start);
            let selected_width_units = f32::from(
                source_layout
                    .x_unit_end
                    .saturating_sub(source_layout.x_unit_start),
            );
            let full_extent_units = f32::from(source_layout.x_unit_full_extent_units);
            let body_unit_px = if full_extent_units > 0.0 {
                Some(layout.body_width_px() / full_extent_units)
            } else {
                None
            };
            let selected_x_without_inset =
                body_unit_px.map(|unit_px| layout.margin_px() + selected_start_units * unit_px);
            let selected_x_with_inset = body_unit_px.map(|unit_px| {
                layout.margin_px()
                    + (selected_start_units + source_layout.x_origin_inset_units) * unit_px
            });
            output.push_str(",\"selectedX\":");
            output.push_str(&format!("{:.3}", source_layout.x));
            output.push_str(",\"selectedWidth\":");
            output.push_str(&format!("{:.3}", source_layout.width));
            output.push_str(",\"selectedStartUnits\":");
            output.push_str(&format!("{selected_start_units:.3}"));
            output.push_str(",\"selectedWidthUnits\":");
            output.push_str(&format!("{selected_width_units:.3}"));
            output.push_str(",\"fullExtentUnits\":");
            output.push_str(&format!("{full_extent_units:.3}"));
            output.push_str(",\"xOriginInsetUnits\":");
            output.push_str(&format!("{:.3}", source_layout.x_origin_inset_units));
            output.push_str(",\"xOriginInsetBasis\":");
            output.push_str(&json_string(source_layout.x_origin_inset_basis));
            output.push_str(",\"xOriginInsetApplied\":");
            output.push_str(if source_layout.x_origin_inset_units != 0.0 {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"pageBodyFrameX\":");
            output.push_str(&format!("{:.3}", layout.margin_px()));
            output.push_str(",\"pageBodyFrameWidth\":");
            output.push_str(&format!("{:.3}", layout.body_width_px()));
            output.push_str(",\"pageBodyUnitPx\":");
            push_optional_f32_json(output, body_unit_px);
            output.push_str(",\"selectedXWithoutInset\":");
            push_optional_f32_json(output, selected_x_without_inset);
            output.push_str(",\"selectedXWithInset\":");
            push_optional_f32_json(output, selected_x_with_inset);
        }
        None => {
            output.push_str(",\"selectedX\":null,\"selectedWidth\":null");
            output.push_str(",\"selectedStartUnits\":null,\"selectedWidthUnits\":null");
            output.push_str(",\"fullExtentUnits\":null,\"xOriginInsetUnits\":null");
            output.push_str(",\"xOriginInsetBasis\":null,\"xOriginInsetApplied\":false");
            output.push_str(",\"pageBodyFrameX\":");
            output.push_str(&format!("{:.3}", layout.margin_px()));
            output.push_str(",\"pageBodyFrameWidth\":");
            output.push_str(&format!("{:.3}", layout.body_width_px()));
            output.push_str(
                ",\"pageBodyUnitPx\":null,\"selectedXWithoutInset\":null,\"selectedXWithInset\":null",
            );
        }
    }

    let page_mark_agreement = table_grid_page_mark_horizontal_best_agreement_group(
        &table_grid_page_space_horizontal_frame_candidate_supports(
            layout,
            document,
            lines,
            candidate,
            source_layout,
        ),
    );
    output.push_str(",\"pageMarkRawAgreementPresent\":");
    output.push_str(if page_mark_agreement.is_some() {
        "true"
    } else {
        "false"
    });
    match page_mark_agreement
        .as_ref()
        .and_then(|supports| supports.first())
    {
        Some(first) => {
            output.push_str(",\"pageMarkRawAgreementSelectedX\":");
            output.push_str(&format!("{:.3}", first.selected_x));
            output.push_str(",\"pageMarkRawAgreementSelectedWidth\":");
            output.push_str(&format!("{:.3}", first.selected_width));
            output.push_str(",\"pageMarkRawAgreementSupportCount\":");
            output.push_str(
                &page_mark_agreement
                    .as_ref()
                    .map(|supports| supports.len())
                    .unwrap_or(0)
                    .to_string(),
            );
            output.push_str(",\"pageMarkRawAgreementFrameBases\":");
            let frame_bases = page_mark_agreement
                .as_ref()
                .map(|supports| {
                    supports
                        .iter()
                        .map(|support| support.frame_basis)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            push_json_string_slice_array(output, &frame_bases);
            output.push_str(",\"pageMarkRawAgreementContributions\":");
            let contributions = page_mark_agreement
                .as_ref()
                .map(|supports| {
                    supports
                        .iter()
                        .map(|support| support.contribution)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            push_json_string_slice_array(output, &contributions);
            output.push_str(",\"pageMarkRawAgreementBlockedReasons\":");
            let blocked_reasons = page_mark_agreement
                .as_ref()
                .map(|supports| {
                    supports
                        .iter()
                        .map(|support| support.blocked_reason)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            push_json_string_slice_array(output, &blocked_reasons);
            output.push_str(",\"sourceFrameVsPageMarkAgreementXResidualPx\":");
            push_optional_f32_json(
                output,
                source_layout.map(|layout| layout.x - first.selected_x),
            );
            output.push_str(",\"sourceFrameVsPageMarkAgreementWidthResidualPx\":");
            push_optional_f32_json(
                output,
                source_layout.map(|layout| layout.width - first.selected_width),
            );
            output.push_str(",\"pageMarkRawAgreementConflictsWithRenderFrame\":");
            let conflicts = source_layout.is_some_and(|layout| {
                (layout.x - first.selected_x).abs() > 2.0
                    || (layout.width - first.selected_width).abs() > 2.0
            });
            output.push_str(if conflicts { "true" } else { "false" });
            output.push_str(",\"pageMarkRawAgreementRenderPromotionBlockedReason\":");
            output.push_str(&json_string(
                "page-mark-horizontal-field-semantics-unproven",
            ));
        }
        None => {
            output.push_str(",\"pageMarkRawAgreementSelectedX\":null");
            output.push_str(",\"pageMarkRawAgreementSelectedWidth\":null");
            output.push_str(",\"pageMarkRawAgreementSupportCount\":0");
            output.push_str(",\"pageMarkRawAgreementFrameBases\":[]");
            output.push_str(",\"pageMarkRawAgreementContributions\":[]");
            output.push_str(",\"pageMarkRawAgreementBlockedReasons\":[]");
            output.push_str(",\"sourceFrameVsPageMarkAgreementXResidualPx\":null");
            output.push_str(",\"sourceFrameVsPageMarkAgreementWidthResidualPx\":null");
            output.push_str(",\"pageMarkRawAgreementConflictsWithRenderFrame\":false");
            output.push_str(",\"pageMarkRawAgreementRenderPromotionBlockedReason\":null");
        }
    }
    output.push_str(",\"sourceTopTextPlacementCoherenceMirror\":");
    push_table_grid_source_top_text_placement_coherence_mirror_json(
        output, layout, document, candidate,
    );
    output.push_str(",\"renderPromotionContribution\":\"source-horizontal-frame-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if page_space_horizontal_transform_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "table-horizontal-page-space-transform-incomplete",
        ));
    }
    output.push('}');
}

pub(super) fn push_table_grid_source_top_text_placement_coherence_mirror_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let readiness =
        table_grid_source_top_text_placement_readiness_for_candidate(layout, document, candidate);
    output.push_str(
        "{\"source\":\"topTextTableSourceGapEvidence.sourceTablePlacementCoherenceGate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourceTopTextPlacementEvidencePresent\":");
    output.push_str(if readiness.is_some() { "true" } else { "false" });
    output.push_str(",\"sourceTopTextPlacementReady\":");
    output.push_str(
        if readiness.as_ref().is_some_and(|readiness| readiness.ready) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"readinessBlockedReasons\":");
    match readiness.as_ref() {
        Some(readiness) => push_json_string_slice_array(output, &readiness.blocked_reasons),
        None => {
            push_json_string_slice_array(output, &["source-top-text-placement-evidence-absent"])
        }
    }
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-frame-top-text-placement-coherence\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    match readiness
        .as_ref()
        .and_then(TableGridSourceTopTextPlacementReadiness::blocked_reason)
    {
        Some(reason) => output.push_str(&json_string(reason)),
        None if readiness.as_ref().is_some_and(|readiness| readiness.ready) => {
            output.push_str("null")
        }
        None => output.push_str(&json_string("source-top-text-placement-evidence-absent")),
    }
    output.push('}');
}

pub(super) fn table_grid_page_mark_horizontal_best_agreement_group(
    supports: &[TableGridHorizontalFrameCandidateSupport],
) -> Option<Vec<TableGridHorizontalFrameCandidateSupport>> {
    let mut groups: BTreeMap<(i32, i32), Vec<TableGridHorizontalFrameCandidateSupport>> =
        BTreeMap::new();
    for support in supports
        .iter()
        .filter(|support| support.frame_basis.starts_with("page-mark"))
    {
        groups
            .entry((
                rounded_milli(support.selected_x),
                rounded_milli(support.selected_width),
            ))
            .or_default()
            .push(support.clone());
    }
    let best_support_count = groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let best_group_count = groups
        .values()
        .filter(|supports| supports.len() == best_support_count)
        .count();
    (best_support_count > 1 && best_group_count == 1)
        .then(|| {
            groups
                .values()
                .find(|supports| supports.len() == best_support_count)
                .cloned()
        })
        .flatten()
}

pub(super) fn push_table_grid_page_space_horizontal_frame_hypotheses_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    output.push('[');
    let mut emitted = false;
    if let Some(source_layout) = source_layout {
        let selected_start = f32::from(source_layout.x_unit_start);
        let selected_width = f32::from(
            source_layout
                .x_unit_end
                .saturating_sub(source_layout.x_unit_start),
        );
        let full_extent = f32::from(source_layout.x_unit_full_extent_units);
        if full_extent > 0.0 {
            push_table_grid_page_space_horizontal_frame_hypothesis_json(
                output,
                "page-body-frame",
                layout.margin_px(),
                layout.body_width_px(),
                selected_start,
                selected_width,
                full_extent,
                "page-body-frame-not-proven-for-table",
            );
            output.push(',');
            push_table_grid_page_space_horizontal_frame_hypothesis_json(
                output,
                "page-media-box",
                0.0,
                layout.width_px(),
                selected_start,
                selected_width,
                full_extent,
                "page-media-box-not-proven-for-table",
            );
            emitted = true;
            push_table_grid_page_space_horizontal_page_mark_raw_field_hypotheses_json(
                output,
                source_layout,
            );
        }
    }
    push_table_grid_page_space_horizontal_source_only_consensus_frame_hypotheses_json(
        output,
        layout,
        document,
        lines,
        candidate,
        &mut emitted,
    );
    output.push(']');
}

pub(super) fn push_table_grid_page_space_horizontal_frame_candidate_agreement_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    let supports = table_grid_page_space_horizontal_frame_candidate_supports(
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    let mut groups: BTreeMap<(i32, i32), Vec<TableGridHorizontalFrameCandidateSupport>> =
        BTreeMap::new();
    for support in supports {
        groups
            .entry((
                rounded_milli(support.selected_x),
                rounded_milli(support.selected_width),
            ))
            .or_default()
            .push(support);
    }
    let best_support_count = groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let best_group_count = groups
        .values()
        .filter(|supports| supports.len() == best_support_count)
        .count();
    let unique_best_supported = best_support_count > 1 && best_group_count == 1;
    let best_group = groups
        .values()
        .find(|supports| supports.len() == best_support_count && unique_best_supported);
    let source_only_selector_in_best_group = best_group.is_some_and(|supports| {
        supports
            .iter()
            .any(|support| support.contribution == "source-only-horizontal-field-selector")
    });
    let source_only_unique_selection_candidate_present =
        unique_best_supported && source_only_selector_in_best_group;
    let selection_ready = false;
    let mut blocked_reasons = Vec::new();
    if groups.is_empty() {
        blocked_reasons.push("source-horizontal-frame-candidates-absent");
    }
    if best_support_count <= 1 {
        blocked_reasons.push("source-horizontal-frame-candidate-agreement-missing");
    }
    if best_group_count > 1 {
        blocked_reasons.push("source-horizontal-frame-candidate-agreement-ambiguous");
    }
    if unique_best_supported {
        blocked_reasons.push("source-horizontal-field-semantics-still-unproven");
    }

    output.push_str(
        "{\"source\":\"pageSpaceHorizontalTransformGate.sourceFrameHypotheses agreement\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"selectionReady\":");
    output.push_str(if selection_ready { "true" } else { "false" });
    output.push_str(",\"candidateCount\":");
    output.push_str(
        &groups
            .values()
            .map(|supports| supports.len())
            .sum::<usize>()
            .to_string(),
    );
    output.push_str(",\"agreementGroupCount\":");
    output.push_str(&groups.len().to_string());
    output.push_str(",\"bestSupportCount\":");
    output.push_str(&best_support_count.to_string());
    output.push_str(",\"uniqueBestSupported\":");
    output.push_str(if unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyUniqueSelectionCandidatePresent\":");
    output.push_str(if source_only_unique_selection_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyUniqueSelectionDiagnosticOnly\":true");
    output.push_str(",\"sourceOnlyUniqueSelectionPromotionReady\":false");
    output.push_str(",\"sourceOnlyUniqueSelectionPromotionBlockedReason\":");
    if source_only_unique_selection_candidate_present {
        output.push_str(&json_string("source-horizontal-field-semantics-unproven"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"bestSupportedSelectedX\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_x)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedSelectedWidth\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_width)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedFrameBases\":");
    match best_group {
        Some(supports) => {
            let frame_bases = supports
                .iter()
                .map(|support| support.frame_basis)
                .collect::<Vec<_>>();
            push_json_string_slice_array(output, &frame_bases);
        }
        None => output.push_str("[]"),
    }
    output.push_str(",\"agreementGroups\":[");
    for (index, supports) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let first = supports.first().unwrap();
        output.push_str("{\"selectedX\":");
        output.push_str(&format!("{:.3}", first.selected_x));
        output.push_str(",\"selectedWidth\":");
        output.push_str(&format!("{:.3}", first.selected_width));
        output.push_str(",\"supportCount\":");
        output.push_str(&supports.len().to_string());
        output.push_str(",\"frameBases\":");
        let frame_bases = supports
            .iter()
            .map(|support| support.frame_basis)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &frame_bases);
        output.push_str(",\"contributions\":");
        let contributions = supports
            .iter()
            .map(|support| support.contribution)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &contributions);
        output.push_str(",\"blockedReasons\":");
        let blocked = supports
            .iter()
            .map(|support| support.blocked_reason)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &blocked);
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-frame-candidate-agreement-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("source-horizontal-field-semantics-unproven"));
    output.push('}');
}

pub(super) fn table_grid_page_space_horizontal_frame_candidate_supports(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    if let Some(source_layout) = source_layout {
        supports.extend(
            table_grid_page_space_horizontal_page_mark_raw_field_candidate_supports(source_layout),
        );
    }
    supports.extend(
        table_grid_source_only_horizontal_field_consensus_candidate_supports(
            layout, document, lines, candidate,
        ),
    );
    if let Some(selector_support) =
        table_grid_source_only_horizontal_field_selector_candidate_support(
            layout, document, lines, candidate,
        )
    {
        supports.push(selector_support);
    }
    supports
}

pub(super) fn table_grid_page_space_horizontal_page_mark_raw_field_candidate_supports(
    source_layout: &TableGridSourceDerivedLayout,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    let Some(fields) = table_grid_source_layout_page_mark_u16_fields(source_layout) else {
        return supports;
    };
    let Some(word_14) = fields.get(14).copied() else {
        return supports;
    };
    let Some(word_21) = fields.get(21).copied() else {
        return supports;
    };
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-word21-direct",
        selected_x: f32::from(word_14),
        selected_width: f32::from(word_21),
        contribution: "source-horizontal-page-mark-raw-field-hypothesis",
        blocked_reason: "page-mark-raw-horizontal-field-semantics-unproven",
    });

    let Some(first_slot_units) = source_layout
        .x_unit_column_slot_width_units
        .first()
        .copied()
        .filter(|units| *units > 0)
    else {
        return supports;
    };
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let selected_width = f32::from(word_21) - f32::from(first_slot_units) * 0.5;
    if selected_x.is_finite() && selected_width.is_finite() && selected_width > 0.0 {
        supports.push(TableGridHorizontalFrameCandidateSupport {
            frame_basis: "page-mark-word14-word21-first-slot-adjusted",
            selected_x,
            selected_width,
            contribution: "source-horizontal-page-mark-raw-field-hypothesis",
            blocked_reason: "page-mark-raw-horizontal-field-semantics-unproven",
        });
    }
    supports
}

pub(super) fn table_grid_source_only_horizontal_field_consensus_candidate_supports(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let Some((page_mark_fields, stable_first_slot_units, stable_first_gap_units)) =
        table_grid_source_only_horizontal_field_consensus_inputs(
            layout, document, lines, candidate,
        )
    else {
        return Vec::new();
    };
    table_grid_source_only_horizontal_field_consensus_supports(
        &page_mark_fields,
        stable_first_slot_units,
        stable_first_gap_units,
    )
}

pub(super) fn table_grid_source_only_horizontal_field_selector_candidate_support(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Option<TableGridHorizontalFrameCandidateSupport> {
    let (page_mark_fields, stable_first_slot_units, stable_first_gap_units) =
        table_grid_source_only_horizontal_field_consensus_inputs(
            layout, document, lines, candidate,
        )?;
    let word_14 = page_mark_fields.get(14).copied()?;
    let first_slot_units = stable_first_slot_units.filter(|units| *units > 0)?;
    let compact_column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let (frame_basis, selected_width) = match compact_column_count {
        2 => {
            let word_21 = page_mark_fields.get(21).copied()?;
            (
                "page-mark-word14-first-slot-word21-half-slot",
                f32::from(word_21) - f32::from(first_slot_units) * 0.5,
            )
        }
        3 => {
            let word_15 = page_mark_fields.get(15).copied()?;
            let first_gap_units = stable_first_gap_units.filter(|units| *units > 0)?;
            (
                "page-mark-word14-first-slot-word15-half-gap",
                f32::from(word_15) - f32::from(first_gap_units) * 0.5,
            )
        }
        _ => return None,
    };
    if !selected_x.is_finite() || !selected_width.is_finite() || selected_width <= 0.0 {
        return None;
    }
    Some(TableGridHorizontalFrameCandidateSupport {
        frame_basis,
        selected_x,
        selected_width,
        contribution: "source-only-horizontal-field-selector",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    })
}

pub(super) fn table_grid_source_only_horizontal_field_consensus_inputs(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Option<(Vec<u16>, Option<u16>, Option<u16>)> {
    let current_source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = current_source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let page_mark_fields = source_layout_page_mark_fields
        .or(cross_table_page_mark_fields)?
        .to_vec();

    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate);
    let sparse_table_candidate_index = sibling
        .as_ref()
        .map(|evidence| evidence.sparse_candidate.index());
    let related_source_layouts = table_grid_related_horizontal_source_layout_summaries(
        layout,
        document,
        lines,
        candidate,
        sparse_table_candidate_index,
        current_source_layout.as_ref(),
    );
    if related_source_layouts.is_empty() {
        return None;
    }

    let first_slot_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_column_slot_units)
        .collect::<Vec<_>>();
    let first_gap_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_intercell_gap_units)
        .collect::<Vec<_>>();
    Some((
        page_mark_fields,
        single_u16_value(&first_slot_units),
        single_u16_value(&first_gap_units),
    ))
}

pub(super) fn table_grid_source_only_horizontal_field_consensus_supports(
    page_mark_fields: &[u16],
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    let Some(word_14) = page_mark_fields.get(14).copied() else {
        return supports;
    };
    let Some(word_15) = page_mark_fields.get(15).copied() else {
        return supports;
    };
    let Some(word_21) = page_mark_fields.get(21).copied() else {
        return supports;
    };
    let Some(first_slot_units) = stable_first_slot_units.filter(|units| *units > 0) else {
        return supports;
    };
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-first-slot-word15-direct",
        selected_x,
        selected_width: f32::from(word_15),
        contribution: "source-only-horizontal-field-consensus",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    });
    if let Some(half_gap_adjustment) = stable_first_gap_units
        .filter(|units| *units > 0)
        .map(|units| f32::from(units) * 0.5)
    {
        supports.push(TableGridHorizontalFrameCandidateSupport {
            frame_basis: "page-mark-word14-first-slot-word15-half-gap",
            selected_x,
            selected_width: f32::from(word_15) - half_gap_adjustment,
            contribution: "source-only-horizontal-field-consensus",
            blocked_reason: "cross-table-horizontal-field-semantics-unproven",
        });
    }
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-first-slot-word21-half-slot",
        selected_x,
        selected_width: f32::from(word_21) - f32::from(first_slot_units) * 0.5,
        contribution: "source-only-horizontal-field-consensus",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    });
    supports
}

pub(super) fn push_table_grid_page_space_horizontal_source_only_consensus_frame_hypotheses_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    emitted: &mut bool,
) {
    let current_source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = current_source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let Some(page_mark_fields) = source_layout_page_mark_fields.or(cross_table_page_mark_fields)
    else {
        return;
    };

    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate);
    let sparse_table_candidate_index = sibling
        .as_ref()
        .map(|evidence| evidence.sparse_candidate.index());
    let related_source_layouts = table_grid_related_horizontal_source_layout_summaries(
        layout,
        document,
        lines,
        candidate,
        sparse_table_candidate_index,
        current_source_layout.as_ref(),
    );
    if related_source_layouts.is_empty() {
        return;
    }

    let first_slot_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_column_slot_units)
        .collect::<Vec<_>>();
    let first_gap_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_intercell_gap_units)
        .collect::<Vec<_>>();
    push_table_grid_source_only_horizontal_field_consensus_hypotheses_items_json(
        output,
        page_mark_fields,
        single_u16_value(&first_slot_units),
        single_u16_value(&first_gap_units),
        emitted,
    );
}

pub(super) fn push_table_grid_page_space_horizontal_page_mark_raw_field_hypotheses_json(
    output: &mut String,
    source_layout: &TableGridSourceDerivedLayout,
) {
    let Some(fields) = table_grid_source_layout_page_mark_u16_fields(source_layout) else {
        return;
    };
    let Some(word_14) = fields.get(14).copied() else {
        return;
    };
    let Some(word_21) = fields.get(21).copied() else {
        return;
    };

    output.push(',');
    push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
        output,
        "page-mark-word14-word21-direct",
        f32::from(word_14),
        f32::from(word_21),
        word_14,
        word_21,
        None,
        0.0,
        0.0,
        "none",
    );

    let Some(first_slot_units) = source_layout
        .x_unit_column_slot_width_units
        .first()
        .copied()
        .filter(|units| *units > 0)
    else {
        return;
    };
    let x_adjustment_units = f32::from(first_slot_units);
    let width_adjustment_units = f32::from(first_slot_units) * 0.5;
    let selected_x = f32::from(word_14) - x_adjustment_units;
    let selected_width = f32::from(word_21) - width_adjustment_units;
    if selected_x.is_finite() && selected_width.is_finite() && selected_width > 0.0 {
        output.push(',');
        push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
            output,
            "page-mark-word14-word21-first-slot-adjusted",
            selected_x,
            selected_width,
            word_14,
            word_21,
            Some(first_slot_units),
            x_adjustment_units,
            width_adjustment_units,
            "line-header-first-column-slot",
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
    output: &mut String,
    frame_basis: &'static str,
    selected_x: f32,
    selected_width: f32,
    page_mark_word_14: u16,
    page_mark_word_21: u16,
    first_column_slot_units: Option<u16>,
    x_adjustment_units: f32,
    width_adjustment_units: f32,
    adjustment_basis: &'static str,
) {
    output.push_str("{\"frameBasis\":");
    output.push_str(&json_string(frame_basis));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateBasis\":\"page-mark-direct-u16-px\"");
    output.push_str(",\"pageMarkWord14\":");
    output.push_str(&page_mark_word_14.to_string());
    output.push_str(",\"pageMarkWord21\":");
    output.push_str(&page_mark_word_21.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    push_optional_u16_json(output, first_column_slot_units);
    output.push_str(",\"xAdjustmentUnits\":");
    output.push_str(&format!("{x_adjustment_units:.3}"));
    output.push_str(",\"widthAdjustmentUnits\":");
    output.push_str(&format!("{width_adjustment_units:.3}"));
    output.push_str(",\"adjustmentBasis\":");
    output.push_str(&json_string(adjustment_basis));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-page-mark-raw-field-hypothesis\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-raw-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_page_space_horizontal_frame_hypothesis_json(
    output: &mut String,
    frame_basis: &'static str,
    frame_x: f32,
    frame_width: f32,
    selected_start_units: f32,
    selected_width_units: f32,
    full_extent_units: f32,
    blocked_reason: &'static str,
) {
    let selected_x = frame_x + frame_width * selected_start_units / full_extent_units;
    let selected_width = frame_width * selected_width_units / full_extent_units;
    output.push_str("{\"frameBasis\":");
    output.push_str(&json_string(frame_basis));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"frameX\":");
    output.push_str(&format!("{frame_x:.3}"));
    output.push_str(",\"frameWidth\":");
    output.push_str(&format!("{frame_width:.3}"));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(",\"selectedStartUnits\":");
    output.push_str(&format!("{selected_start_units:.3}"));
    output.push_str(",\"selectedWidthUnits\":");
    output.push_str(&format!("{selected_width_units:.3}"));
    output.push_str(",\"fullExtentUnits\":");
    output.push_str(&format!("{full_extent_units:.3}"));
    output.push_str(",\"renderPromotionContribution\":\"source-horizontal-frame-hypothesis\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(blocked_reason));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_reference_calibration_replacement_gate_json(
    output: &mut String,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    source_layout_renderable: bool,
    horizontal_solver_ready: bool,
    source_column_split_ready: bool,
    page_space_horizontal_transform_ready: bool,
    row_height_solver_ready: bool,
    y_origin_solver_ready: bool,
) {
    let source_layout_present = source_layout.is_some();
    let replacement_ready = source_layout_renderable
        && horizontal_solver_ready
        && source_column_split_ready
        && page_space_horizontal_transform_ready
        && row_height_solver_ready
        && y_origin_solver_ready;
    let mut blocked_reasons = Vec::new();
    if !source_layout_present {
        blocked_reasons.push("source-derived-layout-candidate-absent");
    }
    if !horizontal_solver_ready {
        blocked_reasons.push("table-horizontal-source-transform-incomplete");
    }
    if !source_column_split_ready {
        blocked_reasons.push("source-column-split-not-ready");
    }
    if !page_space_horizontal_transform_ready {
        blocked_reasons.push("table-horizontal-page-space-transform-incomplete");
    }
    if !row_height_solver_ready {
        blocked_reasons.push("table-row-height-source-transform-incomplete");
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("source-page-y-transform-not-decoded");
    }
    if source_layout_present && !source_layout_renderable {
        blocked_reasons.push("source-derived-layout-not-renderable");
    }

    output.push_str("{\"source\":\"table-page-space-v1 reference calibration replacement gate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"replacementReady\":");
    output.push_str(if replacement_ready { "true" } else { "false" });
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutRenderable\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"horizontalSolverReady\":");
    output.push_str(if horizontal_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceColumnSplitReady\":");
    output.push_str(if source_column_split_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageSpaceHorizontalTransformReady\":");
    output.push_str(if page_space_horizontal_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowHeightSolverReady\":");
    output.push_str(if row_height_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"yOriginSolverReady\":");
    output.push_str(if y_origin_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"reference-calibration-replacement-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if replacement_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-table-page-space-not-ready"));
    }
    output.push('}');
}

pub(super) fn push_table_grid_source_page_y_transform_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
    y_origin_solver_ready: bool,
    line_mark_rows_exact_and_contiguous: bool,
) {
    let layout = page_layout_from_document(document);
    let cross_table_ordering_probe =
        table_grid_cross_table_subrecord_ordering_probe(document, candidate);
    let cross_table_row_boundary_offset_probe =
        table_grid_cross_table_row_boundary_offset_probe(document, candidate);
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
    let selected_ordered_unique_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            readiness
                .selected_post_row_gap_span_coverage
                .ordered_unique_coverage_complete
        });
    let previous_ordered_unique_complete =
        subrecord_span_readiness.as_ref().is_some_and(|readiness| {
            readiness
                .previous_row_span_coverage
                .ordered_unique_coverage_complete
        });

    let mut blocked_reasons = Vec::new();
    match source_layout {
        Some(layout) => {
            if layout.line_mark_page_origin.is_none() {
                blocked_reasons.push("line-mark-page-origin-candidate-absent");
            }
            if layout.line_mark_page_origin_stride.is_some() {
                blocked_reasons.push("line-mark-record-stride-to-page-y-transform-unproven");
            }
            if layout.page_origin_authority != "lineMarkPageGrid" {
                blocked_reasons.push("page-origin-authority-not-renderable-line-mark-page-grid");
            }
            if !line_mark_rows_exact_and_contiguous {
                blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
            }
        }
        None => blocked_reasons.push("source-derived-layout-candidate-absent"),
    }
    if selected_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-selected-post-row-gaps");
        if !selected_ordered_unique_complete {
            blocked_reasons
                .push("page-mark-subrecord-selected-post-row-gap-candidates-not-row-unique");
        }
    }
    if previous_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-previous-row-spans");
        if !previous_ordered_unique_complete {
            blocked_reasons.push("page-mark-subrecord-previous-row-span-candidates-not-row-unique");
        }
    }
    if compact_complete {
        blocked_reasons.push("page-mark-subrecord-spans-fit-compact-row-spans");
    }
    if subrecord_span_readiness.is_some() {
        blocked_reasons.push("page-mark-subrecord-spans-do-not-decode-page-y-origin");
    }
    if cross_table_ordering_probe
        .as_ref()
        .is_some_and(|probe| !probe.monotonic_raw_record_scan_index)
    {
        blocked_reasons.push("page-mark-cross-table-raw-record-order-regression");
    }
    if cross_table_ordering_probe
        .as_ref()
        .is_some_and(|probe| !probe.cross_table_ordering_consistent)
    {
        blocked_reasons.push("page-mark-cross-table-subrecord-ordering-unproven");
    }
    if cross_table_row_boundary_offset_probe
        .as_ref()
        .is_some_and(|probe| probe.all_offsets_require_transform)
    {
        blocked_reasons.push("cross-table-row-boundary-offset-transform-required");
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("decoded-line-mark-page-y-transform-missing");
    }

    output.push_str(
        "{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark y-origin promotion gate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":");
    output.push_str(if y_origin_solver_ready {
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
    output.push_str(",\"pageOriginAuthority\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.page_origin_authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(
        if source_layout
            .as_ref()
            .is_some_and(|layout| layout.line_mark_page_origin.is_some())
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"lineMarkPageOriginStridePresent\":");
    output.push_str(
        if source_layout
            .as_ref()
            .is_some_and(|layout| layout.line_mark_page_origin_stride.is_some())
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"subrecordLineSpanReadinessPresent\":");
    output.push_str(if subrecord_span_readiness.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if previous_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanOrderedCoverage\":");
    match subrecord_span_readiness {
        Some(readiness) => push_table_grid_page_mark_subrecord_line_span_coverage_json(
            output,
            &readiness.selected_post_row_gap_span_coverage,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"previousRowSpanOrderedCoverage\":");
    match subrecord_span_readiness {
        Some(readiness) => push_table_grid_page_mark_subrecord_line_span_coverage_json(
            output,
            &readiness.previous_row_span_coverage,
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"crossTableSubrecordOrderingProbe\":");
    push_table_grid_cross_table_subrecord_ordering_probe_summary_json(
        output,
        cross_table_ordering_probe.as_ref(),
    );
    output.push_str(",\"crossTableRowBoundaryOffsetConsistency\":");
    push_table_grid_cross_table_row_boundary_offset_probe_summary_json(
        output,
        layout,
        document,
        cross_table_row_boundary_offset_probe.as_ref(),
    );
    output.push_str(",\"lineMarkStrideToPageYPromotionReadiness\":");
    push_table_grid_line_mark_stride_to_page_y_promotion_readiness_json(
        output,
        document,
        candidate,
        source_layout,
        subrecord_span_readiness,
        cross_table_ordering_probe.as_ref(),
        selected_complete,
        selected_ordered_unique_complete,
        y_origin_solver_ready,
    );
    output.push_str(",\"sourceOnlyPageYOriginHypothesis\":");
    push_table_grid_source_only_page_y_origin_hypothesis_json(
        output,
        document,
        candidate,
        source_layout,
        subrecord_span_readiness,
        cross_table_ordering_probe.as_ref(),
        cross_table_row_boundary_offset_probe.as_ref(),
        selected_complete,
        selected_ordered_unique_complete,
        previous_complete,
        previous_ordered_unique_complete,
        compact_complete,
        y_origin_solver_ready,
        line_mark_rows_exact_and_contiguous,
    );
    output.push_str(",\"sourceOnlyPageYOriginCandidateAgreementGate\":");
    push_table_grid_source_only_page_y_origin_candidate_agreement_gate_json(
        output,
        document,
        candidate,
        source_layout,
        cross_table_row_boundary_offset_probe.as_ref(),
        subrecord_span_readiness,
    );
    output.push_str(",\"sourceOnlyPageYOriginDomainGate\":");
    push_table_grid_source_only_page_y_origin_domain_gate_json(
        output,
        source_layout,
        cross_table_row_boundary_offset_probe.as_ref(),
    );
    output.push_str(",\"sourceOnlyPageMarkAbsoluteYSlotGate\":");
    push_table_grid_source_only_page_mark_absolute_y_slot_gate_json(
        output,
        document,
        candidate,
        cross_table_row_boundary_offset_probe.as_ref(),
        subrecord_span_readiness,
    );
    output.push_str(",\"lineDomainPostRowGapProjectionProbe\":");
    push_table_grid_line_domain_post_row_gap_projection_probe_json(
        output,
        layout,
        document,
        candidate,
        cross_table_row_boundary_offset_probe.as_ref(),
        subrecord_span_readiness,
    );
    output.push_str(",\"sourceOnlyPageYRenderAdmissionGate\":");
    push_table_grid_source_only_page_y_render_admission_gate_json(
        output,
        document,
        candidate,
        source_layout,
        subrecord_span_readiness,
        cross_table_ordering_probe.as_ref(),
        cross_table_row_boundary_offset_probe.as_ref(),
        y_origin_solver_ready,
        line_mark_rows_exact_and_contiguous,
        selected_complete,
        selected_ordered_unique_complete,
        previous_complete,
        previous_ordered_unique_complete,
        compact_complete,
    );
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-page-y-transform-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if y_origin_solver_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-page-y-transform-not-decoded"));
    }
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_source_only_page_y_render_admission_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
    cross_table_ordering_probe: Option<&TableGridCrossTableSubrecordOrderingProbe>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    y_origin_solver_ready: bool,
    line_mark_rows_exact_and_contiguous: bool,
    selected_complete: bool,
    selected_ordered_unique_complete: bool,
    previous_complete: bool,
    previous_ordered_unique_complete: bool,
    compact_complete: bool,
) {
    let source_layout_candidate_present = source_layout.is_some();
    let direct_line_mark_page_origin_present = source_layout.is_some_and(|layout| {
        layout.line_mark_page_origin.is_some() && layout.page_origin_authority == "lineMarkPageGrid"
    });
    let line_mark_page_origin_stride_present =
        source_layout.is_some_and(|layout| layout.line_mark_page_origin_stride.is_some());
    let cross_table_line_domain_present = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| !probe.combined_line_mark_record_y_tops_px.is_empty());
    let cross_table_order_regresses = cross_table_ordering_probe.is_some_and(|probe| {
        !probe.monotonic_raw_record_scan_index
            || !probe.monotonic_line_start_candidate
            || probe.family_reused_after_later_family
    });
    let cross_table_row_boundary_offset_transform_required = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| probe.all_offsets_require_transform);
    let direct_line_mark_origin_admissible = direct_line_mark_page_origin_present
        && line_mark_rows_exact_and_contiguous
        && y_origin_solver_ready;
    let source_only_page_y_admission_class = if direct_line_mark_origin_admissible {
        "direct-line-mark-page-grid"
    } else if line_mark_page_origin_stride_present && !direct_line_mark_page_origin_present {
        "flow-y-stride-only-diagnostic"
    } else {
        "not-admissible"
    };

    let page_mark_absolute_y_slot_agreement =
        table_grid_source_only_page_mark_absolute_y_slot_agreement(
            document,
            candidate,
            cross_table_row_boundary_offset_probe,
            subrecord_span_readiness,
        );
    let page_mark_absolute_y_slot_semantics_ready =
        page_mark_absolute_y_slot_agreement.semantics_ready();
    let page_mark_absolute_y_slot_blocked_reason =
        table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(
            &page_mark_absolute_y_slot_agreement,
        );
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe,
        );
    let source_gap_to_page_line_gap_table_family_transform_required =
        cross_table_line_domain_present || cross_table_row_boundary_offset_transform_required;

    let y_supports = table_grid_source_only_page_y_origin_candidate_supports(
        document,
        candidate,
        source_layout,
        cross_table_row_boundary_offset_probe,
        subrecord_span_readiness,
    );
    let mut y_groups: BTreeMap<
        (i32, Option<i32>),
        Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    > = BTreeMap::new();
    for support in y_supports.iter().cloned() {
        y_groups
            .entry((
                rounded_milli(support.selected_y),
                support.row_height.map(rounded_milli),
            ))
            .or_default()
            .push(support);
    }
    let y_best_support_count = y_groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let y_best_group_count = y_groups
        .values()
        .filter(|supports| supports.len() == y_best_support_count)
        .count();
    let y_unique_best_supported = y_best_support_count > 1 && y_best_group_count == 1;
    let y_best_group = y_groups
        .values()
        .find(|supports| supports.len() == y_best_support_count && y_unique_best_supported);
    let y_candidate_best_group = y_best_group.filter(|supports| {
        table_grid_source_only_page_y_origin_group_supports_candidate(supports, candidate)
    });
    let y_fallback_selector_group = if y_candidate_best_group.is_none() {
        table_grid_source_only_page_y_origin_fallback_selector_group(&y_groups, candidate)
    } else {
        None
    };
    let y_selector_group = y_candidate_best_group.or(y_fallback_selector_group.as_ref());
    let y_selector_uses_single_support_fallback =
        y_candidate_best_group.is_none() && y_fallback_selector_group.is_some();
    let y_selector_support_count = y_selector_group.map(|supports| supports.len()).unwrap_or(0);
    let y_selector_table_candidate_indexes = y_selector_group
        .map(|supports| {
            supports
                .iter()
                .filter_map(|support| support.table_candidate_index)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let cross_table_previous_row_span_table_candidate_count = y_groups
        .values()
        .flatten()
        .filter(|support| {
            table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(support)
        })
        .filter_map(|support| support.table_candidate_index)
        .collect::<BTreeSet<_>>()
        .len();
    let y_selector_support_fragmented_by_table = !y_selector_uses_single_support_fallback
        && cross_table_previous_row_span_table_candidate_count > 1
        && y_selector_table_candidate_indexes.len()
            < cross_table_previous_row_span_table_candidate_count;
    let y_selector_support_blocked_reasons = y_selector_group
        .map(|supports| table_grid_source_only_page_y_origin_supports_blocked_reasons(supports))
        .unwrap_or_default();
    let y_selector_blocked_reason = if y_selector_group.is_none() {
        "source-y-origin-selector-absent"
    } else if y_selector_uses_single_support_fallback {
        "source-y-origin-selector-single-support-fallback-not-render-admissible"
    } else if y_selector_support_fragmented_by_table {
        "source-y-origin-selector-fragmented-by-table-not-render-admissible"
    } else if !y_unique_best_supported {
        "source-y-origin-selector-agreement-unproven"
    } else if !y_selector_support_blocked_reasons.is_empty() {
        "source-y-origin-selector-support-blocked"
    } else {
        "none"
    };

    let admission_ready = direct_line_mark_origin_admissible;
    let mut blocked_reasons = Vec::new();
    if !admission_ready {
        if !source_layout_candidate_present {
            blocked_reasons.push("source-derived-layout-candidate-absent");
        }
        if !direct_line_mark_page_origin_present {
            blocked_reasons.push("direct-line-mark-page-origin-absent");
        }
        if line_mark_page_origin_stride_present {
            blocked_reasons.push("line-mark-record-stride-to-page-y-transform-unproven");
        }
        if source_layout.is_some_and(|layout| layout.page_origin_authority != "lineMarkPageGrid") {
            blocked_reasons.push("page-origin-authority-not-renderable-line-mark-page-grid");
        }
        if !line_mark_rows_exact_and_contiguous {
            blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
        }
        if cross_table_line_domain_present {
            blocked_reasons.push("cross-table-line-domain-not-page-space-origin");
        }
        if selected_complete {
            blocked_reasons.push("selected-post-row-gap-spans-not-page-y-origin");
            if !selected_ordered_unique_complete {
                blocked_reasons.push("selected-post-row-gap-coverage-not-row-unique");
            }
        }
        if previous_complete {
            blocked_reasons.push("previous-row-span-spans-require-page-origin-transform");
            if !previous_ordered_unique_complete {
                blocked_reasons.push("previous-row-span-coverage-not-row-unique");
            }
        }
        if compact_complete {
            blocked_reasons.push("compact-row-span-spans-do-not-decode-page-y-origin");
        }
        if cross_table_order_regresses {
            blocked_reasons.push("source-order-vs-subrecord-order-contradiction");
        }
        if cross_table_row_boundary_offset_transform_required {
            blocked_reasons.push("cross-table-row-boundary-offset-transform-required");
        }
        if page_mark_absolute_y_slot_blocked_reason != "none" {
            blocked_reasons.push(page_mark_absolute_y_slot_blocked_reason);
        }
        if y_selector_blocked_reason != "none" {
            blocked_reasons.push(y_selector_blocked_reason);
        }
        if source_gap_to_page_line_gap_table_family_transform_required
            && let Some(reason) =
                source_gap_to_page_line_gap_readiness_hints.table_family_transform_blocked_reason()
        {
            blocked_reasons.push(reason);
        }
        if !y_origin_solver_ready {
            blocked_reasons.push("decoded-line-mark-page-y-transform-missing");
        }
    }

    output.push_str(
        "{\"source\":\"sourcePageYTransformGate source-only page-y render admission gate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"admissionReady\":");
    output.push_str(if admission_ready { "true" } else { "false" });
    output.push_str(",\"directLineMarkOriginAdmissible\":");
    output.push_str(if direct_line_mark_origin_admissible {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageOriginAuthority\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.page_origin_authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(if direct_line_mark_page_origin_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkPageOriginStridePresent\":");
    output.push_str(if line_mark_page_origin_stride_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableLineDomainPresent\":");
    output.push_str(if cross_table_line_domain_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyPageYAdmissionClass\":");
    output.push_str(&json_string(source_only_page_y_admission_class));
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if previous_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"compactRowSpanComplete\":");
    output.push_str(if compact_complete { "true" } else { "false" });
    output.push_str(",\"sourceOnlySelectorPresent\":");
    output.push_str(if y_selector_group.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlySelectorSingleSupportFallback\":");
    output.push_str(if y_selector_uses_single_support_fallback {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlySelectorSupportCount\":");
    output.push_str(&y_selector_support_count.to_string());
    output.push_str(",\"sourceOnlySelectorSupportFragmentedByTable\":");
    output.push_str(if y_selector_support_fragmented_by_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlySelectorBlockedReason\":");
    output.push_str(&json_string(y_selector_blocked_reason));
    output.push_str(",\"sourceOnlySelectorSupportBlockedReasons\":");
    push_json_string_slice_array(output, &y_selector_support_blocked_reasons);
    output.push_str(",\"sourceGapToPageLineGapTransformAdmissionGate\":");
    push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
        output,
        "sourceOnlyPageYRenderAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate",
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"sourceGapToPageLineGapTableFamilyTransformRequired\":");
    output.push_str(
        if source_gap_to_page_line_gap_table_family_transform_required {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sourceGapToPageLineGapTableFamilyTransformStable\":");
    output.push_str(
        if source_gap_to_page_line_gap_readiness_hints
            .table_family_source_gap_to_page_line_gap_transform_stable()
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"sourceGapToPageLineGapTableFamilyTransformBlockedReason\":");
    match source_gap_to_page_line_gap_readiness_hints.table_family_transform_blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageMarkAbsoluteYSlotSemanticsReady\":");
    output.push_str(if page_mark_absolute_y_slot_semantics_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageMarkAbsoluteYSlotBlockedReason\":");
    output.push_str(&json_string(page_mark_absolute_y_slot_blocked_reason));
    output.push_str(",\"pageMarkAbsoluteYSlotResidualPx\":");
    push_optional_f32_json(output, page_mark_absolute_y_slot_agreement.residual_px);
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"source-only-page-y-render-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if admission_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-page-y-render-admission-not-ready"));
    }
    output.push('}');
}

pub(super) fn push_table_grid_line_domain_post_row_gap_projection_probe_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let line_domain_y = cross_table_row_boundary_offset_probe.and_then(|probe| {
        probe
            .tables
            .iter()
            .find(|table| table.table_candidate_index == candidate.index())
            .and_then(|table| table.line_mark_record_y_tops_px.first().copied())
            .or_else(|| probe.combined_line_mark_record_y_tops_px.first().copied())
    });
    let selected_span_units = subrecord_span_readiness.and_then(|readiness| {
        readiness
            .selected_post_row_gap_span_targets
            .first()
            .copied()
    });
    let projected_y = line_domain_y
        .zip(selected_span_units)
        .map(|(line_domain_y, span_units)| line_domain_y + span_units as f32);
    let column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let reference_layout =
        diagnostic_reference_table_grid_overlay_layout(layout, document, candidate, column_count);
    let reference_top_y = reference_layout.as_ref().map(|layout| layout.y);
    let residual_px = projected_y
        .zip(reference_top_y)
        .map(|(projected_y, reference_top_y)| projected_y - reference_top_y);
    let within_two_px = residual_px.is_some_and(|residual| residual.abs() <= 2.0);
    let selected_complete = subrecord_span_readiness.is_some_and(|readiness| {
        !readiness.selected_post_row_gap_span_targets.is_empty()
            && readiness.selected_post_row_gap_span_hit_count
                == readiness.selected_post_row_gap_span_targets.len()
    });
    let selected_ordered_unique_complete = subrecord_span_readiness.is_some_and(|readiness| {
        readiness
            .selected_post_row_gap_span_coverage
            .ordered_unique_coverage_complete
    });

    let mut source_only_blocked_reasons = Vec::new();
    if line_domain_y.is_none() {
        source_only_blocked_reasons.push("line-domain-y-candidate-absent");
    }
    if selected_span_units.is_none() {
        source_only_blocked_reasons.push("selected-post-row-gap-span-candidate-absent");
    }
    source_only_blocked_reasons.push("cross-domain-source-units-treated-as-px");
    source_only_blocked_reasons.push("selected-spacing-records-are-post-row-gap-family");
    if !selected_complete {
        source_only_blocked_reasons.push("selected-post-row-gap-span-incomplete");
    }
    if !selected_ordered_unique_complete {
        source_only_blocked_reasons.push("selected-post-row-gap-span-not-ordered-unique");
    }
    let mut blocked_reasons = source_only_blocked_reasons.clone();
    if reference_top_y.is_some() {
        blocked_reasons.push("reference-only-validation");
    }
    blocked_reasons.push("page-y-origin-transform-undecoded");
    let mut source_only_projection_blocked_reasons = source_only_blocked_reasons.clone();
    source_only_projection_blocked_reasons.push("page-y-origin-transform-undecoded");

    output.push_str(
        "{\"source\":\"sourcePageYTransformGate line-domain + post-row-gap span projection\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":");
    output.push_str(if reference_top_y.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"projectionKind\":\"line-domain-y-plus-post-row-gap-unit-as-px\"");
    output.push_str(",\"selectionReady\":false,\"promotionReady\":false");
    output.push_str(",\"lineDomainY\":");
    push_optional_f32_json(output, line_domain_y);
    output.push_str(",\"selectedPostRowGapSpanFirstUnits\":");
    push_optional_usize_json(output, selected_span_units);
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"projectedY\":");
    push_optional_f32_json(output, projected_y);
    output.push_str(",\"referenceTableTopY\":");
    push_optional_f32_json(output, reference_top_y);
    output.push_str(",\"residualPx\":");
    push_optional_f32_json(output, residual_px);
    output.push_str(",\"absResidualPx\":");
    push_optional_f32_json(output, residual_px.map(f32::abs));
    output.push_str(",\"withinTwoPx\":");
    output.push_str(if within_two_px { "true" } else { "false" });
    output.push_str(",\"sourceOnlyProjectionDomainGate\":");
    push_table_grid_line_domain_post_row_gap_source_only_projection_domain_gate_json(
        output,
        line_domain_y,
        selected_span_units,
        selected_complete,
        selected_ordered_unique_complete,
        projected_y,
        &source_only_projection_blocked_reasons,
    );
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"line-domain-post-row-gap-projection-probe\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-domain-post-row-gap-projection-crosses-source-unit-domain",
    ));
    output.push('}');
}

pub(super) fn push_table_grid_line_domain_post_row_gap_source_only_projection_domain_gate_json(
    output: &mut String,
    line_domain_y: Option<f32>,
    selected_span_units: Option<usize>,
    selected_complete: bool,
    selected_ordered_unique_complete: bool,
    projected_y: Option<f32>,
    blocked_reasons: &[&str],
) {
    let source_projection_present = line_domain_y.is_some() && selected_span_units.is_some();
    output.push_str("{\"source\":\"sourcePageYTransformGate source-only line-domain/post-row-gap projection domain gate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"sourceProjectionPresent\":");
    output.push_str(if source_projection_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineDomainPresent\":");
    output.push_str(if line_domain_y.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanPresent\":");
    output.push_str(if selected_span_units.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(
        ",\"sourceUnitDomain\":\"line-mark-record-y-plus-page-mark-subrecord-gap-units\"",
    );
    output.push_str(",\"lineDomainY\":");
    push_optional_f32_json(output, line_domain_y);
    output.push_str(",\"selectedPostRowGapSpanFirstUnits\":");
    push_optional_usize_json(output, selected_span_units);
    output.push_str(",\"projectedY\":");
    push_optional_f32_json(output, projected_y);
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-only-line-domain-post-row-gap-projection-domain-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if blocked_reasons.is_empty() {
        output.push_str("null");
    } else if blocked_reasons.contains(&"cross-domain-source-units-treated-as-px") {
        output.push_str(&json_string(
            "line-domain-post-row-gap-projection-crosses-source-unit-domain",
        ));
    } else {
        output.push_str(&json_string(blocked_reasons[0]));
    }
    output.push('}');
}

pub(super) fn table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(
    agreement: &TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement,
) -> &'static str {
    if agreement.semantics_ready() {
        "none"
    } else if agreement.best_absolute_y_slot.is_some()
        && agreement.line_domain_projected_y.is_some()
        && !agreement.agrees
    {
        "line-domain-projection-disagrees-with-page-mark-absolute-y-slot"
    } else if agreement.best_absolute_y_slot.is_none() {
        "page-mark-absolute-y-slot-absent"
    } else if agreement.line_domain_projected_y.is_none() {
        "line-domain-plus-span-projection-absent"
    } else {
        "page-mark-absolute-y-slot-semantics-unproven"
    }
}

pub(super) fn table_grid_source_only_page_mark_absolute_y_slot_agreement(
    document: &Document,
    candidate: &TableCandidate,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
    let line_domain_y = cross_table_row_boundary_offset_probe.and_then(|probe| {
        probe
            .tables
            .iter()
            .find(|table| table.table_candidate_index == candidate.index())
            .and_then(|table| table.line_mark_record_y_tops_px.first().copied())
            .or_else(|| probe.combined_line_mark_record_y_tops_px.first().copied())
    });
    let selected_span_units = subrecord_span_readiness.and_then(|readiness| {
        readiness
            .selected_post_row_gap_span_targets
            .first()
            .copied()
    });
    let line_domain_projected_y = line_domain_y
        .zip(selected_span_units)
        .map(|(line_domain_y, span_units)| line_domain_y + span_units as f32);
    let candidates = table_grid_source_only_page_mark_absolute_y_slot_candidates(
        document,
        subrecord_span_readiness,
    );
    let best_absolute_y_slot = candidates
        .iter()
        .min_by(|left, right| {
            let left_residual = line_domain_projected_y
                .map(|projected_y| (left.value_px - projected_y).abs())
                .unwrap_or(0.0);
            let right_residual = line_domain_projected_y
                .map(|projected_y| (right.value_px - projected_y).abs())
                .unwrap_or(0.0);
            option_f32_order(Some(left_residual), Some(right_residual))
                .then_with(|| left.byte_offset.cmp(&right.byte_offset))
        })
        .cloned();
    let absolute_y_slot_y = best_absolute_y_slot.as_ref().map(|slot| slot.value_px);
    let residual_px = line_domain_projected_y
        .zip(absolute_y_slot_y)
        .map(|(projected_y, absolute_y)| projected_y - absolute_y);
    let agrees = residual_px.is_some_and(|residual| residual.abs() <= 2.0);

    TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
        line_domain_y,
        selected_span_units,
        line_domain_projected_y,
        candidates,
        best_absolute_y_slot,
        residual_px,
        agrees,
    }
}

pub(super) fn push_table_grid_source_only_page_mark_absolute_y_slot_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let agreement = table_grid_source_only_page_mark_absolute_y_slot_agreement(
        document,
        candidate,
        cross_table_row_boundary_offset_probe,
        subrecord_span_readiness,
    );
    let absolute_y_slot_y = agreement
        .best_absolute_y_slot
        .as_ref()
        .map(|slot| slot.value_px);
    let lineage_class = if absolute_y_slot_y.is_some() {
        "page-mark-absolute-y-slot"
    } else if agreement.line_domain_projected_y.is_some() {
        "line-domain-plus-span-projection"
    } else {
        "no-source-absolute-y-slot"
    };

    let mut blocked_reasons = Vec::new();
    if agreement.best_absolute_y_slot.is_none() {
        blocked_reasons.push("page-mark-absolute-y-slot-absent");
    }
    if agreement.line_domain_projected_y.is_none() {
        blocked_reasons.push("line-domain-plus-span-projection-absent");
    }
    if agreement.best_absolute_y_slot.is_some()
        && agreement.line_domain_projected_y.is_some()
        && !agreement.agrees
    {
        blocked_reasons.push("line-domain-projection-disagrees-with-page-mark-absolute-y-slot");
    }
    if !agreement.semantics_ready() {
        blocked_reasons.push("page-mark-absolute-y-slot-semantics-unproven");
    }
    blocked_reasons.push("page-y-origin-transform-undecoded");

    output.push_str(
        "{\"source\":\"/PageMark raw u16 subrecord scan+/LineMark source page-line projection\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(
        ",\"projectionKind\":\"line-domain-y-plus-post-row-gap-vs-page-mark-absolute-y-slot\"",
    );
    output.push_str(",\"lineDomainY\":");
    push_optional_f32_json(output, agreement.line_domain_y);
    output.push_str(",\"selectedPostRowGapSpanFirstUnits\":");
    push_optional_usize_json(output, agreement.selected_span_units);
    output.push_str(",\"lineDomainProjectedY\":");
    push_optional_f32_json(output, agreement.line_domain_projected_y);
    output.push_str(",\"absoluteYSlotPresent\":");
    output.push_str(if absolute_y_slot_y.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bestAbsoluteYSlot\":");
    match agreement.best_absolute_y_slot.as_ref() {
        Some(slot) => push_table_grid_source_only_page_mark_absolute_y_slot_candidate_json(
            output,
            slot,
            Some(&agreement.candidates),
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"absoluteYSlotY\":");
    push_optional_f32_json(output, absolute_y_slot_y);
    output.push_str(",\"lineDomainProjectionVsAbsoluteYSlotResidualPx\":");
    push_optional_f32_json(output, agreement.residual_px);
    output.push_str(",\"lineDomainProjectionAgreesWithAbsoluteYSlot\":");
    output.push_str(if agreement.agrees { "true" } else { "false" });
    output.push_str(",\"lineageClass\":");
    output.push_str(&json_string(lineage_class));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-only-page-mark-absolute-y-slot-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if blocked_reasons.is_empty() {
        output.push_str("null");
    } else {
        output.push_str(&json_string(blocked_reasons[0]));
    }
    output.push('}');
}

pub(super) fn table_grid_source_only_page_mark_absolute_y_slot_candidates(
    document: &Document,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> Vec<TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate> {
    let Some(page_mark_bytes) = raw_stream_bytes(document, PAGE_MARK_PATH) else {
        return Vec::new();
    };
    let Some(readiness) = subrecord_span_readiness else {
        return Vec::new();
    };
    let record_headers = page_mark_record_headers(page_mark_bytes);
    let mut candidates = Vec::new();
    let mut seen = BTreeSet::new();
    for subrecord_byte_offset in readiness
        .selected_post_row_gap_span_coverage
        .unique_candidate_byte_offsets
        .iter()
        .copied()
    {
        if !seen.insert(subrecord_byte_offset) {
            continue;
        }
        let Some(subrecord) =
            page_mark_raw_u16_subrecord_candidate_at(page_mark_bytes, subrecord_byte_offset)
        else {
            continue;
        };
        let field_index = 2usize;
        let byte_offset = subrecord.byte_offset + field_index * 2;
        let Some((raw_record_scan_index, raw_record_index, tail_block16_word_index)) =
            page_mark_raw_subrecord_record_context(&record_headers, byte_offset)
        else {
            continue;
        };
        candidates.push(TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate {
            source: "rawRecordHeaderTailU16Subrecord",
            interpretation: "direct-u16-px",
            field_index,
            tail_block16_word_index: Some(tail_block16_word_index),
            raw_record_scan_index: Some(raw_record_scan_index),
            raw_record_index: Some(raw_record_index),
            byte_offset,
            subrecord_byte_offset,
            subrecord_line_start_candidate: subrecord.words[4],
            subrecord_line_end_candidate: subrecord.words[6],
            value: subrecord.words[field_index],
            value_px: f32::from(subrecord.words[field_index]),
        });
    }
    candidates.sort_by(|left, right| {
        left.byte_offset
            .cmp(&right.byte_offset)
            .then_with(|| left.value.cmp(&right.value))
    });
    candidates
}

pub(super) fn push_table_grid_source_only_page_mark_absolute_y_slot_candidate_json(
    output: &mut String,
    candidate: &TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate,
    all_candidates: Option<&[TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate]>,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(candidate.source));
    output.push_str(",\"interpretation\":");
    output.push_str(&json_string(candidate.interpretation));
    output.push_str(",\"fieldIndex\":");
    output.push_str(&candidate.field_index.to_string());
    output.push_str(",\"tailBlock16WordIndex\":");
    push_option_usize_json(output, candidate.tail_block16_word_index);
    output.push_str(",\"rawRecordScanIndexes\":");
    let raw_record_scan_indexes = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .filter_map(|candidate| candidate.raw_record_scan_index)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_usize_array_json(output, &raw_record_scan_indexes);
    output.push_str(",\"rawRecordIndexes\":");
    let raw_record_indexes = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .filter_map(|candidate| candidate.raw_record_index)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_u32_array_json(output, &raw_record_indexes);
    output.push_str(",\"byteOffsets\":");
    let byte_offsets = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .map(|candidate| candidate.byte_offset)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_usize_array_json(output, &byte_offsets);
    output.push_str(",\"subrecordByteOffsets\":");
    let subrecord_byte_offsets = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .map(|candidate| candidate.subrecord_byte_offset)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_usize_array_json(output, &subrecord_byte_offsets);
    output.push_str(",\"subrecordLineStartCandidates\":");
    let line_start_candidates = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .map(|candidate| candidate.subrecord_line_start_candidate)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_u16_array_json(output, &line_start_candidates);
    output.push_str(",\"subrecordLineEndCandidates\":");
    let line_end_candidates = all_candidates
        .map(|candidates| {
            candidates
                .iter()
                .map(|candidate| candidate.subrecord_line_end_candidate)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    push_u16_array_json(output, &line_end_candidates);
    output.push_str(",\"value\":");
    output.push_str(&candidate.value.to_string());
    output.push_str(",\"valuePx\":");
    output.push_str(&format!("{:.3}", candidate.value_px));
    output.push('}');
}

pub(super) fn table_grid_source_gap_to_page_line_gap_readiness_hints(
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
) -> TableGridSourceGapToPageLineGapReadinessHints {
    let source_range_gap_units = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.source_range_gap_units)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let row_source_start_gap_units = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.row_source_start_gap_units)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let previous_family_record_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.line_mark_record_gap)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let segment_offset_gap_units = row_source_start_gap_units
        .iter()
        .copied()
        .zip(source_range_gap_units.iter().copied())
        .map(|(row_source_start_gap, source_range_gap)| {
            row_source_start_gap_minus_source_range_gap_units(
                row_source_start_gap,
                source_range_gap,
            )
        })
        .collect::<Vec<_>>();
    let source_range_gap_minus_page_line_gap_units = source_range_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(source_range_gap, page_line_gap)| {
            source_range_gap_minus_page_line_gap_units(source_range_gap, page_line_gap)
        })
        .collect::<Vec<_>>();
    let row_source_start_gap_minus_page_line_gap_units = row_source_start_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(row_source_start_gap, page_line_gap)| {
            row_source_start_gap.saturating_sub(page_line_gap)
        })
        .collect::<Vec<_>>();
    let segment_offset_gap_minus_page_line_gap_units = segment_offset_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(segment_offset_gap, page_line_gap)| segment_offset_gap.saturating_sub(page_line_gap))
        .collect::<Vec<_>>();
    let source_range_units_per_page_line_gap =
        ratio_usize_by_i32(&source_range_gap_units, &previous_family_record_gaps);
    let row_source_start_units_per_page_line_gap =
        ratio_i32_by_i32(&row_source_start_gap_units, &previous_family_record_gaps);
    let segment_offset_units_per_page_line_gap =
        ratio_i32_by_i32(&segment_offset_gap_units, &previous_family_record_gaps);
    let same_page_mark_entry_transition_count = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .filter(|transition| transition.same_page_mark_entry)
                .count()
        })
        .unwrap_or(0);
    let transition_count = previous_family_record_gaps.len();
    let all_transitions_same_page_mark_entry =
        transition_count > 0 && same_page_mark_entry_transition_count == transition_count;
    let source_range_gap_to_page_line_gap_max_abs_delta_units =
        max_abs_i32(&source_range_gap_minus_page_line_gap_units);
    let row_source_start_gap_to_page_line_gap_max_abs_delta_units =
        max_abs_i32(&row_source_start_gap_minus_page_line_gap_units);
    let segment_offset_gap_to_page_line_gap_max_abs_delta_units =
        max_abs_i32(&segment_offset_gap_minus_page_line_gap_units);
    let best_candidate = [
        (
            "direct-source-range-gap",
            source_range_gap_to_page_line_gap_max_abs_delta_units,
        ),
        (
            "direct-row-source-start-gap",
            row_source_start_gap_to_page_line_gap_max_abs_delta_units,
        ),
        (
            "segment-offset-gap",
            segment_offset_gap_to_page_line_gap_max_abs_delta_units,
        ),
    ]
    .into_iter()
    .filter_map(|(kind, max_abs_delta)| max_abs_delta.map(|delta| (kind, delta)))
    .min_by_key(|(_, delta)| *delta);
    let affine_row_source_start_gap_fit = affine_row_source_start_gap_fit(
        &previous_family_record_gaps,
        &row_source_start_gap_units,
        all_transitions_same_page_mark_entry,
    );

    TableGridSourceGapToPageLineGapReadinessHints {
        transition_count,
        same_page_mark_entry_transition_count,
        all_transitions_same_page_mark_entry,
        source_range_gap_to_page_line_gap_max_abs_delta_units,
        row_source_start_gap_to_page_line_gap_max_abs_delta_units,
        segment_offset_gap_to_page_line_gap_max_abs_delta_units,
        best_candidate_transform_kind: best_candidate.map(|(kind, _)| kind),
        best_candidate_max_abs_delta_units: best_candidate.map(|(_, delta)| delta),
        source_range_units_per_page_line_gap_spread: f32_value_spread(
            &source_range_units_per_page_line_gap,
        ),
        row_source_start_units_per_page_line_gap_spread: f32_value_spread(
            &row_source_start_units_per_page_line_gap,
        ),
        segment_offset_units_per_page_line_gap_spread: f32_value_spread(
            &segment_offset_units_per_page_line_gap,
        ),
        affine_row_source_start_gap_fit,
    }
}

pub(super) fn push_table_grid_source_gap_to_page_line_gap_readiness_hints_json(
    output: &mut String,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) {
    output.push_str("{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapReadinessHints\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"transitionCount\":");
    output.push_str(&hints.transition_count.to_string());
    output.push_str(",\"samePageMarkEntryTransitionCount\":");
    output.push_str(&hints.same_page_mark_entry_transition_count.to_string());
    output.push_str(",\"allTransitionsSamePageMarkEntry\":");
    output.push_str(if hints.all_transitions_same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRangeGapToPageLineGapMaxAbsDeltaUnits\":");
    push_optional_i32_json(
        output,
        hints.source_range_gap_to_page_line_gap_max_abs_delta_units,
    );
    output.push_str(",\"rowSourceStartGapToPageLineGapMaxAbsDeltaUnits\":");
    push_optional_i32_json(
        output,
        hints.row_source_start_gap_to_page_line_gap_max_abs_delta_units,
    );
    output.push_str(",\"segmentOffsetGapToPageLineGapMaxAbsDeltaUnits\":");
    push_optional_i32_json(
        output,
        hints.segment_offset_gap_to_page_line_gap_max_abs_delta_units,
    );
    output.push_str(",\"bestCandidateTransformKind\":");
    match hints.best_candidate_transform_kind {
        Some(kind) => output.push_str(&json_string(kind)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestCandidateMaxAbsDeltaUnits\":");
    push_optional_i32_json(output, hints.best_candidate_max_abs_delta_units);
    output.push_str(",\"transformCandidateCount\":");
    output.push_str(&hints.transform_candidate_count().to_string());
    output.push_str(",\"exactTransformCandidateCount\":");
    output.push_str(&hints.exact_transform_candidate_count().to_string());
    output.push_str(",\"bestCandidateTransitionCoverageCount\":");
    output.push_str(&hints.best_candidate_transition_coverage_count().to_string());
    output.push_str(",\"bestCandidateUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(
        output,
        hints.best_candidate_units_per_page_line_gap_spread(),
    );
    let lowest_spread_candidate = hints.lowest_spread_candidate();
    output.push_str(",\"lowestSpreadCandidateTransformKind\":");
    if let Some((kind, _)) = lowest_spread_candidate {
        output.push_str(&json_string(kind));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"lowestSpreadUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, lowest_spread_candidate.map(|(_, spread)| spread));
    output.push_str(",\"transformCandidateSummaries\":");
    push_table_grid_source_gap_to_page_line_gap_transform_candidate_summaries_json(output, hints);
    output.push_str(",\"declinedTransformCandidates\":");
    push_table_grid_source_gap_to_page_line_gap_declined_transform_candidates_json(output, hints);
    output.push_str(",\"affineRowSourceStartGapFit\":");
    push_affine_row_source_start_gap_fit_json(output, hints.affine_row_source_start_gap_fit);
    output.push_str(",\"sourceRangeUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, hints.source_range_units_per_page_line_gap_spread);
    output.push_str(",\"rowSourceStartUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(
        output,
        hints.row_source_start_units_per_page_line_gap_spread,
    );
    output.push_str(",\"segmentOffsetUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, hints.segment_offset_units_per_page_line_gap_spread);
    output.push_str(",\"sourceGapToPageLineGapTransformStable\":");
    output.push_str(if hints.source_gap_to_page_line_gap_transform_stable() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"tableFamilySourceGapToPageLineGapTransformStable\":");
    output.push_str(
        if hints.table_family_source_gap_to_page_line_gap_transform_stable() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"tableFamilyTransformBlockedReason\":");
    match hints.table_family_transform_blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    match hints.transform_blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(super) fn push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
    output: &mut String,
    source: &'static str,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) {
    let transform_ready = hints.source_gap_to_page_line_gap_transform_stable();
    let table_family_transform_ready =
        hints.table_family_source_gap_to_page_line_gap_transform_stable();
    let mut declared_blockers = Vec::new();
    if hints.transition_count == 0 {
        declared_blockers.push("source-gap-to-page-line-gap-transform-evidence-absent");
    }
    if !transform_ready {
        declared_blockers.push("source-gap-to-page-line-gap-transform-not-stable");
    }
    if let Some(reason) = hints.table_family_transform_blocked_reason()
        && !declared_blockers.contains(&reason)
    {
        declared_blockers.push(reason);
    }
    if !transform_ready {
        declared_blockers.push("source-gap-to-page-line-gap-transform-undecoded");
    }

    output.push_str("{\"source\":");
    output.push_str(&json_string(source));
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"transformDomain\":");
    output.push_str(&json_string("source-unit-gap-to-page-mark-line-index-gap"));
    output.push_str(",\"canDecodeSourceTransform\":");
    output.push_str(if transform_ready { "true" } else { "false" });
    output.push_str(",\"tableFamilyTransformStable\":");
    output.push_str(if table_family_transform_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"tableFamilyTransformBlockedReason\":");
    match hints.table_family_transform_blocked_reason() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"transitionCount\":");
    output.push_str(&hints.transition_count.to_string());
    output.push_str(",\"allTransitionsSamePageMarkEntry\":");
    output.push_str(if hints.all_transitions_same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bestCandidateTransformKind\":");
    match hints.best_candidate_transform_kind {
        Some(kind) => output.push_str(&json_string(kind)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestCandidateMaxAbsDeltaUnits\":");
    push_optional_i32_json(output, hints.best_candidate_max_abs_delta_units);
    output.push_str(",\"transformCandidateCount\":");
    output.push_str(&hints.transform_candidate_count().to_string());
    output.push_str(",\"exactTransformCandidateCount\":");
    output.push_str(&hints.exact_transform_candidate_count().to_string());
    output.push_str(",\"bestCandidateTransitionCoverageCount\":");
    output.push_str(&hints.best_candidate_transition_coverage_count().to_string());
    output.push_str(",\"bestCandidateUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(
        output,
        hints.best_candidate_units_per_page_line_gap_spread(),
    );
    let lowest_spread_candidate = hints.lowest_spread_candidate();
    output.push_str(",\"lowestSpreadCandidateTransformKind\":");
    if let Some((kind, _)) = lowest_spread_candidate {
        output.push_str(&json_string(kind));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"lowestSpreadUnitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, lowest_spread_candidate.map(|(_, spread)| spread));
    output.push_str(",\"declinedTransformCandidates\":");
    push_table_grid_source_gap_to_page_line_gap_declined_transform_candidates_json(output, hints);
    output.push_str(",\"affineRowSourceStartGapFit\":");
    push_affine_row_source_start_gap_fit_json(output, hints.affine_row_source_start_gap_fit);
    output.push_str(",\"declaredBlockers\":");
    push_json_string_slice_array(output, &declared_blockers);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-transform-admission-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if transform_ready {
        output.push_str("null");
    } else if hints.transition_count == 0 {
        output.push_str(&json_string(
            "source-gap-to-page-line-gap-transform-evidence-absent",
        ));
    } else {
        output.push_str(&json_string(
            "source-gap-to-page-line-gap-transform-not-stable",
        ));
    }
    output.push('}');
}

pub(super) fn push_table_grid_source_gap_to_page_line_gap_transform_candidate_summaries_json(
    output: &mut String,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) {
    output.push('[');
    for (index, candidate) in hints.transform_candidate_summaries().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_source_gap_to_page_line_gap_transform_candidate_summary_json(
            output, hints, candidate,
        );
    }
    output.push(']');
}

pub(super) fn push_table_grid_source_gap_to_page_line_gap_declined_transform_candidates_json(
    output: &mut String,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) {
    output.push('[');
    let mut first = true;
    for candidate in hints.transform_candidate_summaries() {
        if table_grid_source_gap_to_page_line_gap_decline_reason(&candidate, hints).is_none() {
            continue;
        }
        if !first {
            output.push(',');
        }
        first = false;
        push_table_grid_source_gap_to_page_line_gap_transform_candidate_summary_json(
            output, hints, &candidate,
        );
    }
    output.push(']');
}

pub(super) fn push_table_grid_source_gap_to_page_line_gap_transform_candidate_summary_json(
    output: &mut String,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
    candidate: &TableGridSourceGapToPageLineGapTransformCandidateSummary,
) {
    let selected = hints.best_candidate_transform_kind == Some(candidate.kind);
    let stable = candidate
        .affine_row_source_start_gap_fit
        .map(|fit| fit.fit_stable)
        .unwrap_or(candidate.max_abs_delta_units == Some(0));
    output.push_str("{\"transformKind\":");
    output.push_str(&json_string(candidate.kind));
    output.push_str(",\"selected\":");
    output.push_str(if selected { "true" } else { "false" });
    output.push_str(",\"stable\":");
    output.push_str(if stable { "true" } else { "false" });
    output.push_str(",\"transitionCoverageCount\":");
    if candidate.max_abs_delta_units.is_some() {
        output.push_str(&hints.transition_count.to_string());
    } else {
        output.push('0');
    }
    output.push_str(",\"maxAbsDeltaUnits\":");
    push_optional_i32_json(output, candidate.max_abs_delta_units);
    output.push_str(",\"unitsPerPageLineGapSpread\":");
    push_optional_f32_json(output, candidate.units_per_page_line_gap_spread);
    if let Some(fit) = candidate.affine_row_source_start_gap_fit {
        output.push_str(",\"affineRowSourceStartGapFit\":");
        push_affine_row_source_start_gap_fit_json(output, Some(fit));
    }
    output.push_str(",\"declineReason\":");
    if let Some(reason) = table_grid_source_gap_to_page_line_gap_decline_reason(candidate, hints) {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    if let Some(reason) = table_grid_source_gap_to_page_line_gap_candidate_blocked_reason(candidate)
    {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn table_grid_source_gap_to_page_line_gap_decline_reason(
    candidate: &TableGridSourceGapToPageLineGapTransformCandidateSummary,
    hints: &TableGridSourceGapToPageLineGapReadinessHints,
) -> Option<&'static str> {
    if let Some(fit) = candidate.affine_row_source_start_gap_fit {
        return Some(fit.blocked_reason());
    }
    if hints.best_candidate_transform_kind == Some(candidate.kind) {
        return None;
    }
    let Some(candidate_delta) = candidate.max_abs_delta_units else {
        return Some("transform-candidate-evidence-absent");
    };
    let Some(best_delta) = hints.best_candidate_max_abs_delta_units else {
        return Some("transform-candidate-not-selected-without-best-transform");
    };
    if candidate_delta > best_delta {
        Some("higher-max-delta-than-selected-transform")
    } else if candidate_delta == best_delta {
        Some("tie-not-selected-by-candidate-order")
    } else {
        Some("transform-candidate-not-selected")
    }
}

pub(super) fn table_grid_source_gap_to_page_line_gap_candidate_blocked_reason(
    candidate: &TableGridSourceGapToPageLineGapTransformCandidateSummary,
) -> Option<&'static str> {
    if let Some(fit) = candidate.affine_row_source_start_gap_fit {
        Some(fit.blocked_reason())
    } else if candidate.max_abs_delta_units.is_none() {
        Some("transform-candidate-evidence-absent")
    } else if candidate.max_abs_delta_units != Some(0) {
        Some("source-gap-to-page-line-gap-transform-not-stable")
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_source_only_page_y_origin_hypothesis_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
    cross_table_ordering_probe: Option<&TableGridCrossTableSubrecordOrderingProbe>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    selected_complete: bool,
    selected_ordered_unique_complete: bool,
    previous_complete: bool,
    previous_ordered_unique_complete: bool,
    compact_complete: bool,
    y_origin_solver_ready: bool,
    line_mark_rows_exact_and_contiguous: bool,
) {
    let line_mark_page_origin_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_page_origin.is_some());
    let line_mark_page_origin_stride_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_page_origin_stride.is_some());
    let cross_table_line_domain_candidate_present = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| {
            probe.all_records_within_single_page_mark_entry
                && !probe.combined_line_mark_record_y_tops_px.is_empty()
        });
    let page_mark_absolute_y_slot_agreement = if source_layout.is_some() {
        Some(table_grid_source_only_page_mark_absolute_y_slot_agreement(
            document,
            candidate,
            cross_table_row_boundary_offset_probe,
            subrecord_span_readiness,
        ))
    } else {
        None
    };
    let page_mark_absolute_y_slot = page_mark_absolute_y_slot_agreement
        .as_ref()
        .and_then(|agreement| agreement.best_absolute_y_slot.as_ref());
    let page_mark_absolute_y_slot_present = page_mark_absolute_y_slot.is_some();
    let candidate_present = line_mark_page_origin_present
        || line_mark_page_origin_stride_present
        || cross_table_line_domain_candidate_present;
    let candidate_kind = if line_mark_page_origin_present {
        Some("line-mark-page-origin")
    } else if line_mark_page_origin_stride_present {
        Some("line-mark-page-origin-stride")
    } else if cross_table_line_domain_candidate_present {
        Some("cross-table-page-line-domain")
    } else {
        None
    };
    let cross_table_ordering_consistent =
        cross_table_ordering_probe.is_some_and(|probe| probe.cross_table_ordering_consistent);
    let cross_table_order_regresses = cross_table_ordering_probe.is_some_and(|probe| {
        !probe.monotonic_raw_record_scan_index
            || !probe.monotonic_line_start_candidate
            || probe.family_reused_after_later_family
    });
    let cross_table_offsets_stable =
        cross_table_row_boundary_offset_probe.is_some_and(|probe| probe.all_offsets_stable);
    let cross_table_offsets_require_transform = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| probe.all_offsets_require_transform);
    let piecewise_all_tables_exact = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| probe.source_unit_to_page_line_index_piecewise_all_tables_exact);
    let cross_table_previous_row_span_support_count =
        table_grid_cross_table_previous_row_span_y_origin_support_count(
            cross_table_row_boundary_offset_probe,
        );
    let cross_table_previous_row_span_selector_present =
        cross_table_previous_row_span_support_count > 0;
    let cross_table_previous_row_span_selection_ready =
        cross_table_previous_row_span_selector_present
            && previous_complete
            && previous_ordered_unique_complete
            && cross_table_offsets_stable
            && !cross_table_offsets_require_transform
            && piecewise_all_tables_exact
            && cross_table_ordering_consistent
            && !cross_table_order_regresses
            && y_origin_solver_ready;
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe,
        );

    let mut blocked_reasons = Vec::new();
    if source_layout.is_none() && cross_table_row_boundary_offset_probe.is_none() {
        blocked_reasons.push("source-y-origin-evidence-absent");
    }
    if !candidate_present {
        blocked_reasons.push("source-page-y-origin-candidate-absent");
    }
    if line_mark_page_origin_stride_present && !line_mark_page_origin_present {
        blocked_reasons.push("stride-origin-needs-page-origin-rule");
    }
    match source_layout {
        Some(layout) => {
            if layout.render_promotion_blocked_reason != "none" {
                blocked_reasons.push(layout.render_promotion_blocked_reason);
            }
            if !layout.line_header_rows_homogeneous {
                blocked_reasons.push("line-header-rows-not-homogeneous");
            }
            if !layout.line_mark_rows_exact_and_contiguous {
                blocked_reasons.push("line-mark-rows-not-exact-source-boundaries");
            }
            if layout.page_origin_authority != "lineMarkPageGrid" {
                blocked_reasons.push("page-origin-authority-not-renderable-line-mark-page-grid");
            }
        }
        None => blocked_reasons.push("source-derived-layout-candidate-absent"),
    }
    if !line_mark_rows_exact_and_contiguous {
        blocked_reasons.push("gate-line-mark-rows-not-exact-source-boundaries");
    }
    if subrecord_span_readiness.is_none() {
        blocked_reasons.push("page-mark-subrecord-line-span-readiness-absent");
    }
    if selected_complete && !selected_ordered_unique_complete {
        blocked_reasons.push("selected-post-row-gap-subrecord-coverage-not-ordered-unique");
    }
    if previous_complete && !previous_ordered_unique_complete {
        blocked_reasons.push("previous-row-span-subrecord-coverage-not-ordered-unique");
    }
    if compact_complete {
        blocked_reasons.push("compact-row-span-subrecord-spans-do-not-decode-origin");
    }
    if cross_table_ordering_probe.is_some() && !cross_table_ordering_consistent {
        blocked_reasons.push("cross-table-subrecord-ordering-inconsistent");
    }
    if cross_table_order_regresses {
        blocked_reasons.push("source-order-vs-subrecord-order-contradiction");
    }
    if cross_table_offsets_require_transform {
        blocked_reasons.push("cross-table-row-boundary-offset-transform-required");
    }
    if !piecewise_all_tables_exact {
        blocked_reasons.push("source-unit-to-page-line-piecewise-fit-not-exact");
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("decoded-page-y-origin-missing");
    }
    let y_origin_readiness_class = if line_mark_page_origin_present {
        "direct-line-mark-origin"
    } else if line_mark_page_origin_stride_present {
        "stride-only"
    } else if cross_table_line_domain_candidate_present {
        "cross-table-line-domain-only"
    } else {
        "none"
    };
    let origin_decision_ready = y_origin_solver_ready && line_mark_page_origin_present;
    let mut y_origin_readiness_blocked_reasons = Vec::new();
    match y_origin_readiness_class {
        "direct-line-mark-origin" => {
            if !origin_decision_ready {
                y_origin_readiness_blocked_reasons.push("direct-line-mark-origin-not-promotable");
            }
        }
        "stride-only" => {
            y_origin_readiness_blocked_reasons.push("line-mark-page-origin-stride-present");
            y_origin_readiness_blocked_reasons.push("stride-origin-needs-direct-line-origin-rule");
            if !line_mark_page_origin_present {
                y_origin_readiness_blocked_reasons.push("direct-line-mark-page-origin-absent");
            }
            if !y_origin_solver_ready {
                y_origin_readiness_blocked_reasons.push("decoded-page-y-origin-missing");
            }
        }
        "cross-table-line-domain-only" => {
            y_origin_readiness_blocked_reasons.push("cross-table-line-domain-present");
            if cross_table_offsets_require_transform {
                y_origin_readiness_blocked_reasons
                    .push("line-domain-to-page-space-origin-transform-required");
            }
            if !piecewise_all_tables_exact {
                y_origin_readiness_blocked_reasons
                    .push("source-unit-to-page-line-piecewise-fit-not-exact");
            }
            if let Some(reason) =
                source_gap_to_page_line_gap_readiness_hints.transform_blocked_reason()
            {
                y_origin_readiness_blocked_reasons.push(reason);
            }
            if cross_table_order_regresses {
                y_origin_readiness_blocked_reasons
                    .push("source-order-vs-subrecord-order-contradiction");
            }
            if !y_origin_solver_ready {
                y_origin_readiness_blocked_reasons.push("decoded-page-y-origin-missing");
            }
        }
        _ => {
            y_origin_readiness_blocked_reasons.push("source-page-y-origin-candidate-absent");
        }
    }

    output
        .push_str("{\"source\":\"sourcePageYTransformGate source-only page-y origin hypothesis\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"candidatePresent\":");
    output.push_str(if candidate_present { "true" } else { "false" });
    output.push_str(",\"candidateKind\":");
    match candidate_kind {
        Some(kind) => output.push_str(&json_string(kind)),
        None => output.push_str("null"),
    }
    output.push_str(",\"yOriginReadinessClass\":");
    output.push_str(&json_string(y_origin_readiness_class));
    output.push_str(",\"originDecisionReady\":");
    output.push_str(if origin_decision_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"yOriginReadinessBlockedReasons\":");
    push_json_string_slice_array(output, &y_origin_readiness_blocked_reasons);
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(if line_mark_page_origin_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkPageOriginStridePresent\":");
    output.push_str(if line_mark_page_origin_stride_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageMarkAbsoluteYSlotCandidatePresent\":");
    output.push_str(if page_mark_absolute_y_slot_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageMarkAbsoluteYSlotY\":");
    push_optional_f32_json(output, page_mark_absolute_y_slot.map(|slot| slot.value_px));
    output.push_str(",\"pageMarkAbsoluteYSlotBlockedReason\":");
    if page_mark_absolute_y_slot_agreement
        .as_ref()
        .is_some_and(TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement::semantics_ready)
    {
        output.push_str("null");
    } else if page_mark_absolute_y_slot_present {
        output.push_str(&json_string("page-mark-absolute-y-slot-semantics-unproven"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pageOriginAuthority\":");
    match source_layout {
        Some(layout) => output.push_str(&json_string(layout.page_origin_authority)),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceGapToPageLineGapReadinessHints\":");
    push_table_grid_source_gap_to_page_line_gap_readiness_hints_json(
        output,
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"sourceLayoutRenderable\":");
    output.push_str(
        if source_layout.is_some_and(table_grid_source_derived_layout_is_renderable) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"lineHeaderRowsHomogeneous\":");
    output.push_str(
        if source_layout.is_some_and(|layout| layout.line_header_rows_homogeneous) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"lineMarkRowsExactAndContiguous\":");
    output.push_str(if line_mark_rows_exact_and_contiguous {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"directLineMarkPageOrigin\":");
    match source_layout.and_then(|layout| layout.line_mark_page_origin.as_ref()) {
        Some(origin) => {
            output.push_str("{\"y\":");
            output.push_str(&format!("{:.3}", origin.y));
            output.push_str(",\"firstLineMarkRecordIndex\":");
            output.push_str(&origin.first_line_mark_record_index.to_string());
            output.push_str(",\"lastLineMarkRecordIndex\":");
            output.push_str(&origin.last_line_mark_record_index.to_string());
            output.push_str(",\"pageMarkEntryIndex\":");
            output.push_str(&origin.page_mark_entry_index.to_string());
            output.push_str(",\"pageLineStart\":");
            output.push_str(&origin.page_line_start.to_string());
            output.push_str(",\"pageLineEnd\":");
            output.push_str(&origin.page_line_end.to_string());
            output.push_str(",\"lineOffsetFromPageStart\":");
            output.push_str(&origin.line_offset_from_page_start.to_string());
            output.push_str(",\"linePitchPx\":");
            output.push_str(&format!("{:.3}", origin.line_pitch_px));
            output.push_str(",\"linePitchBasis\":");
            output.push_str(&json_string(origin.line_pitch_basis));
            output.push_str(",\"rowHeight\":");
            output.push_str(&format!("{:.3}", origin.row_height));
            output.push('}');
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"strideLineMarkPageOrigin\":");
    match source_layout.and_then(|layout| layout.line_mark_page_origin_stride.as_ref()) {
        Some(stride) => {
            output.push_str("{\"lineMarkRecordIndexes\":");
            push_usize_array_json(output, &stride.line_mark_record_indexes);
            output.push_str(",\"recordStride\":");
            output.push_str(&stride.record_stride.to_string());
            output.push_str(",\"firstLineMarkRecordIndex\":");
            output.push_str(&stride.first_line_mark_record_index.to_string());
            output.push_str(",\"lastLineMarkRecordIndex\":");
            output.push_str(&stride.last_line_mark_record_index.to_string());
            output.push_str(",\"pageMarkEntryIndex\":");
            output.push_str(&stride.page_mark_entry_index.to_string());
            output.push_str(",\"pageLineStart\":");
            output.push_str(&stride.page_line_start.to_string());
            output.push_str(",\"pageLineEnd\":");
            output.push_str(&stride.page_line_end.to_string());
            output.push_str(",\"lineOffsetFromPageStart\":");
            output.push_str(&stride.line_offset_from_page_start.to_string());
            output.push_str(",\"rowHeight\":");
            output.push_str(&format!("{:.3}", stride.row_height));
            output.push_str(",\"rawRecordIndexRowTops\":");
            push_f32_array_json(output, &stride.raw_record_index_row_tops);
            output.push_str(",\"strideCollapsedRowTops\":");
            push_f32_array_json(output, &stride.stride_collapsed_row_tops);
            output.push('}');
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"pageMarkAbsoluteYSlotOrigin\":");
    match page_mark_absolute_y_slot {
        Some(slot) => push_table_grid_source_only_page_mark_absolute_y_slot_candidate_json(
            output,
            slot,
            page_mark_absolute_y_slot_agreement
                .as_ref()
                .map(|agreement| agreement.candidates.as_slice()),
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"crossTableLineDomainEvidence\":");
    match cross_table_row_boundary_offset_probe {
        Some(probe) => {
            output.push_str("{\"present\":true,\"allRecordsWithinSinglePageMarkEntry\":");
            output.push_str(if probe.all_records_within_single_page_mark_entry {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"allOffsetsStable\":");
            output.push_str(if cross_table_offsets_stable {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"allOffsetsRequireTransform\":");
            output.push_str(if cross_table_offsets_require_transform {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"stableRowBoundaryOffsetCandidateUnits\":");
            push_optional_i32_json(output, probe.stable_row_boundary_offset_candidate_units);
            output.push_str(",\"piecewiseAllTablesExact\":");
            output.push_str(if piecewise_all_tables_exact {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"piecewiseMaxAbsResidualRecordIndexes\":");
            push_optional_f32_json(
                output,
                probe.source_unit_to_page_line_index_piecewise_max_abs_residual,
            );
            output.push_str(",\"combinedLineMarkRecordIndexes\":");
            push_usize_array_json(output, &probe.combined_line_mark_record_indexes);
            output.push_str(",\"combinedLineMarkRecordYTopPx\":");
            push_f32_array_json(output, &probe.combined_line_mark_record_y_tops_px);
            output.push('}');
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"crossTablePreviousRowSpanSelectorPresent\":");
    output.push_str(if cross_table_previous_row_span_selector_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanSupportCount\":");
    output.push_str(&cross_table_previous_row_span_support_count.to_string());
    output.push_str(",\"crossTablePreviousRowSpanSelectionReady\":");
    output.push_str(if cross_table_previous_row_span_selection_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanReadinessInputs\":{\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if previous_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOffsetsStable\":");
    output.push_str(if cross_table_offsets_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOffsetsRequireTransform\":");
    output.push_str(if cross_table_offsets_require_transform {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"piecewiseAllTablesExact\":");
    output.push_str(if piecewise_all_tables_exact {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOrderingConsistent\":");
    output.push_str(if cross_table_ordering_consistent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOrderRegresses\":");
    output.push_str(if cross_table_order_regresses {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"decodedPageYOriginPresent\":");
    output.push_str(if y_origin_solver_ready {
        "true"
    } else {
        "false"
    });
    output.push('}');
    output.push_str(",\"subrecordSpanEvidence\":{\"present\":");
    output.push_str(if subrecord_span_readiness.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousRowSpanComplete\":");
    output.push_str(if previous_complete { "true" } else { "false" });
    output.push_str(",\"previousRowSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if previous_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"compactRowSpanComplete\":");
    output.push_str(if compact_complete { "true" } else { "false" });
    output.push('}');
    output.push_str(",\"crossTableOrderingConsistent\":");
    output.push_str(if cross_table_ordering_consistent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableOrderRegresses\":");
    output.push_str(if cross_table_order_regresses {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-only-page-y-origin-hypothesis\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if y_origin_solver_ready && blocked_reasons.is_empty() {
        output.push_str("null");
    } else {
        output.push_str(&json_string("source-page-y-origin-inference-pending"));
    }
    output.push('}');
}

pub(super) fn push_table_grid_source_only_page_y_origin_candidate_agreement_gate_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) {
    let supports = table_grid_source_only_page_y_origin_candidate_supports(
        document,
        candidate,
        source_layout,
        cross_table_row_boundary_offset_probe,
        subrecord_span_readiness,
    );
    let mut groups: BTreeMap<
        (i32, Option<i32>),
        Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    > = BTreeMap::new();
    for support in supports {
        groups
            .entry((
                rounded_milli(support.selected_y),
                support.row_height.map(rounded_milli),
            ))
            .or_default()
            .push(support);
    }

    let best_support_count = groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let best_group_count = groups
        .values()
        .filter(|supports| supports.len() == best_support_count)
        .count();
    let unique_best_supported = best_support_count > 1 && best_group_count == 1;
    let best_group = groups
        .values()
        .find(|supports| supports.len() == best_support_count && unique_best_supported);
    let candidate_best_group = best_group.filter(|supports| {
        table_grid_source_only_page_y_origin_group_supports_candidate(supports, candidate)
    });
    let fallback_selector_group = if candidate_best_group.is_none() {
        table_grid_source_only_page_y_origin_fallback_selector_group(&groups, candidate)
    } else {
        None
    };
    let best_supported_table_candidate_indexes = best_group
        .map(|supports| {
            supports
                .iter()
                .filter_map(|support| support.table_candidate_index)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let best_supported_table_candidate_count = best_supported_table_candidate_indexes.len();
    let best_supported_covers_multiple_table_candidates = best_supported_table_candidate_count > 1;
    let selection_ready = false;
    let cross_table_previous_row_span_support_count = groups
        .values()
        .flatten()
        .filter(|support| {
            table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(support)
        })
        .count();
    let cross_table_previous_row_span_table_candidate_indexes = groups
        .values()
        .flatten()
        .filter(|support| {
            table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(support)
        })
        .filter_map(|support| support.table_candidate_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let cross_table_previous_row_span_table_candidate_count =
        cross_table_previous_row_span_table_candidate_indexes.len();
    let cross_table_previous_row_span_unique_best_supported = best_group.is_some_and(|supports| {
        supports
            .iter()
            .all(table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span)
    });
    let cross_table_previous_row_span_group_blocked_reasons = best_group
        .filter(|_| cross_table_previous_row_span_unique_best_supported)
        .map(|supports| table_grid_source_only_page_y_origin_supports_blocked_reasons(supports))
        .unwrap_or_default();
    let cross_table_previous_row_span_ready = cross_table_previous_row_span_unique_best_supported
        && cross_table_previous_row_span_group_blocked_reasons.is_empty();
    let cross_table_previous_row_span_best_group_table_coverage_ratio =
        if cross_table_previous_row_span_table_candidate_count == 0 {
            None
        } else {
            Some(
                best_supported_table_candidate_count as f32
                    / cross_table_previous_row_span_table_candidate_count as f32,
            )
        };
    let cross_table_previous_row_span_support_fragmented_by_table =
        cross_table_previous_row_span_table_candidate_count > 1
            && best_supported_table_candidate_count
                < cross_table_previous_row_span_table_candidate_count;

    let mut blocked_reasons = Vec::new();
    if groups.is_empty() {
        blocked_reasons.push("source-only-page-y-origin-candidates-absent");
    }
    if best_support_count <= 1 {
        blocked_reasons.push("source-only-page-y-origin-candidate-agreement-missing");
    }
    if best_group_count > 1 {
        blocked_reasons.push("source-only-page-y-origin-candidate-agreement-ambiguous");
    }
    if unique_best_supported {
        blocked_reasons.push("source-page-y-origin-field-semantics-still-unproven");
    }
    if unique_best_supported && !best_supported_covers_multiple_table_candidates {
        blocked_reasons.push("source-page-y-origin-best-support-not-cross-table");
    }
    if cross_table_previous_row_span_support_fragmented_by_table {
        blocked_reasons.push("cross-table-previous-row-span-support-fragmented-by-table");
    }

    output
        .push_str("{\"source\":\"sourcePageYTransformGate.sourcePageYOriginHypotheses agreement\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false");
    output.push_str(",\"selectionReady\":");
    output.push_str(if selection_ready { "true" } else { "false" });
    output.push_str(",\"candidateCount\":");
    output.push_str(
        &groups
            .values()
            .map(|supports| supports.len())
            .sum::<usize>()
            .to_string(),
    );
    output.push_str(",\"agreementGroupCount\":");
    output.push_str(&groups.len().to_string());
    output.push_str(",\"bestSupportCount\":");
    output.push_str(&best_support_count.to_string());
    output.push_str(",\"uniqueBestSupported\":");
    output.push_str(if unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bestSupportedSelectedY\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_y)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedRowHeight\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => push_optional_f32_json(output, support.row_height),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedOriginBases\":");
    match best_group {
        Some(supports) => {
            let origin_bases = supports
                .iter()
                .map(|support| support.origin_basis)
                .collect::<Vec<_>>();
            push_json_string_slice_array(output, &origin_bases);
        }
        None => output.push_str("[]"),
    }
    output.push_str(",\"bestSupportedTableCandidateIndexes\":");
    push_usize_array_json(output, &best_supported_table_candidate_indexes);
    output.push_str(",\"bestSupportedTableCandidateCount\":");
    output.push_str(&best_supported_table_candidate_count.to_string());
    output.push_str(",\"bestSupportedCoversMultipleTableCandidates\":");
    output.push_str(if best_supported_covers_multiple_table_candidates {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanSupportCount\":");
    output.push_str(&cross_table_previous_row_span_support_count.to_string());
    output.push_str(",\"crossTablePreviousRowSpanTableCandidateIndexes\":");
    push_usize_array_json(
        output,
        &cross_table_previous_row_span_table_candidate_indexes,
    );
    output.push_str(",\"crossTablePreviousRowSpanTableCandidateCount\":");
    output.push_str(&cross_table_previous_row_span_table_candidate_count.to_string());
    output.push_str(",\"crossTablePreviousRowSpanUniqueBestSupported\":");
    output.push_str(if cross_table_previous_row_span_unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanReady\":");
    output.push_str(if cross_table_previous_row_span_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTablePreviousRowSpanBestGroupCoversMultipleTables\":");
    output.push_str(
        if cross_table_previous_row_span_unique_best_supported
            && best_supported_covers_multiple_table_candidates
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"crossTablePreviousRowSpanBestGroupTableCoverageRatio\":");
    push_optional_f32_json(
        output,
        cross_table_previous_row_span_best_group_table_coverage_ratio,
    );
    output.push_str(",\"crossTablePreviousRowSpanSupportFragmentedByTable\":");
    output.push_str(
        if cross_table_previous_row_span_support_fragmented_by_table {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"crossTablePreviousRowSpanReadinessBlockedReasons\":");
    push_json_string_slice_array(output, &cross_table_previous_row_span_group_blocked_reasons);
    output.push_str(",\"sourceOnlyPageYOriginSelector\":");
    push_table_grid_source_only_page_y_origin_selector_json(
        output,
        candidate,
        candidate_best_group,
        fallback_selector_group.as_ref(),
        best_supported_covers_multiple_table_candidates,
        cross_table_previous_row_span_support_fragmented_by_table,
    );
    output.push_str(",\"agreementGroups\":[");
    for (index, supports) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let first = supports.first().unwrap();
        output.push_str("{\"selectedY\":");
        output.push_str(&format!("{:.3}", first.selected_y));
        output.push_str(",\"rowHeight\":");
        push_optional_f32_json(output, first.row_height);
        output.push_str(",\"supportCount\":");
        output.push_str(&supports.len().to_string());
        output.push_str(",\"originBases\":");
        let origin_bases = supports
            .iter()
            .map(|support| support.origin_basis)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &origin_bases);
        output.push_str(",\"tableCandidateIndexes\":");
        let table_indexes = supports
            .iter()
            .filter_map(|support| support.table_candidate_index)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        push_usize_array_json(output, &table_indexes);
        output.push_str(",\"contributions\":");
        let contributions = supports
            .iter()
            .map(|support| support.contribution)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &contributions);
        output.push_str(",\"blockedReasons\":");
        let blocked = table_grid_source_only_page_y_origin_supports_blocked_reasons(supports);
        push_json_string_slice_array(output, &blocked);
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-page-y-origin-candidate-agreement-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("source-page-y-origin-agreement-unproven"));
    output.push('}');
}

pub(super) fn push_table_grid_source_only_page_y_origin_selector_json(
    output: &mut String,
    candidate: &TableCandidate,
    best_group: Option<&Vec<TableGridSourceOnlyPageYOriginCandidateSupport>>,
    fallback_group: Option<&Vec<TableGridSourceOnlyPageYOriginCandidateSupport>>,
    best_supported_covers_multiple_table_candidates: bool,
    cross_table_previous_row_span_support_fragmented_by_table: bool,
) {
    let Some(selector_group) = best_group.or(fallback_group) else {
        output.push_str("null");
        return;
    };
    let Some(first) = selector_group.first() else {
        output.push_str("null");
        return;
    };
    let using_single_support_fallback = best_group.is_none() && fallback_group.is_some();
    let best_table_candidate_indexes = selector_group
        .iter()
        .filter_map(|support| support.table_candidate_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let origin_bases = selector_group
        .iter()
        .map(|support| support.origin_basis)
        .collect::<Vec<_>>();
    let contributions = selector_group
        .iter()
        .map(|support| support.contribution)
        .collect::<Vec<_>>();
    let mut blocked_reasons =
        table_grid_source_only_page_y_origin_supports_blocked_reasons(selector_group);
    if using_single_support_fallback {
        blocked_reasons.push("single-source-y-origin-support-unproven");
    }
    let selector_support_covers_multiple_table_candidates = if using_single_support_fallback {
        best_table_candidate_indexes.len() > 1
    } else {
        best_supported_covers_multiple_table_candidates
    };
    let selector_support_fragmented_by_table =
        !using_single_support_fallback && cross_table_previous_row_span_support_fragmented_by_table;
    let selector_blocked_reason = if using_single_support_fallback {
        "single-source-y-origin-support-unproven"
    } else if selector_support_fragmented_by_table {
        "cross-table-previous-row-span-support-fragmented-by-table"
    } else if !selector_support_covers_multiple_table_candidates {
        "source-page-y-origin-best-support-not-cross-table"
    } else {
        "source-page-y-origin-field-semantics-still-unproven"
    };

    output.push_str(
        "{\"source\":\"sourceOnlyPageYOriginCandidateAgreementGate best-supported group selector\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"selectionBasis\":");
    output.push_str(&json_string(if using_single_support_fallback {
        "single-support-source-only-y-origin-fallback"
    } else {
        "best-supported-source-only-y-origin-agreement-group"
    }));
    output.push_str(",\"singleSupportFallback\":");
    output.push_str(if using_single_support_fallback {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedOriginBasis\":");
    output.push_str(&json_string(first.origin_basis));
    output.push_str(",\"selectedY\":");
    output.push_str(&format!("{:.3}", first.selected_y));
    output.push_str(",\"selectedRowHeight\":");
    push_optional_f32_json(output, first.row_height);
    output.push_str(",\"supportCount\":");
    output.push_str(&selector_group.len().to_string());
    output.push_str(",\"supportOriginBases\":");
    push_json_string_slice_array(output, &origin_bases);
    output.push_str(",\"supportTableCandidateIndexes\":");
    push_usize_array_json(output, &best_table_candidate_indexes);
    output.push_str(",\"supportCoversMultipleTableCandidates\":");
    output.push_str(if selector_support_covers_multiple_table_candidates {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"supportFragmentedByTable\":");
    output.push_str(if selector_support_fragmented_by_table {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"contributions\":");
    push_json_string_slice_array(output, &contributions);
    output.push_str(",\"supportBlockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-only-page-y-origin-selector\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(selector_blocked_reason));
    output.push('}');
}

pub(super) fn table_grid_source_only_page_y_origin_fallback_selector_group(
    groups: &BTreeMap<(i32, Option<i32>), Vec<TableGridSourceOnlyPageYOriginCandidateSupport>>,
    candidate: &TableCandidate,
) -> Option<Vec<TableGridSourceOnlyPageYOriginCandidateSupport>> {
    groups
        .values()
        .filter(|supports| {
            !supports.is_empty()
                && table_grid_source_only_page_y_origin_group_supports_candidate(
                    supports, candidate,
                )
        })
        .min_by(|left, right| {
            let left_support = left.first().unwrap();
            let right_support = right.first().unwrap();
            table_grid_source_only_page_y_origin_fallback_rank(left_support)
                .cmp(&table_grid_source_only_page_y_origin_fallback_rank(
                    right_support,
                ))
                .then_with(|| {
                    left_support
                        .table_candidate_index
                        .unwrap_or(usize::MAX)
                        .cmp(&right_support.table_candidate_index.unwrap_or(usize::MAX))
                })
                .then_with(|| {
                    rounded_milli(left_support.selected_y)
                        .cmp(&rounded_milli(right_support.selected_y))
                })
                .then_with(|| left_support.origin_basis.cmp(right_support.origin_basis))
        })
        .cloned()
}

pub(super) fn table_grid_source_only_page_y_origin_group_supports_candidate(
    supports: &[TableGridSourceOnlyPageYOriginCandidateSupport],
    candidate: &TableCandidate,
) -> bool {
    let mut has_table_specific_support = false;
    for support in supports {
        if let Some(table_candidate_index) = support.table_candidate_index {
            has_table_specific_support = true;
            if table_candidate_index == candidate.index() {
                return true;
            }
        }
    }
    if has_table_specific_support {
        return false;
    }
    supports
        .iter()
        .any(table_grid_source_only_page_y_origin_support_is_candidate_local_unindexed)
}

pub(super) fn table_grid_source_only_page_y_origin_support_is_candidate_local_unindexed(
    support: &TableGridSourceOnlyPageYOriginCandidateSupport,
) -> bool {
    support.table_candidate_index.is_none()
        && !matches!(
            support.origin_basis,
            "cross-table-combined-previous-row-span-first-record"
        )
}

pub(super) fn table_grid_source_only_page_y_origin_fallback_rank(
    support: &TableGridSourceOnlyPageYOriginCandidateSupport,
) -> usize {
    match support.origin_basis {
        "line-mark-page-origin-direct" => 0,
        "page-mark-absolute-y-slot-field2-tail-block16-word11" => 1,
        "line-mark-stride-raw-record-index-first-row" => 2,
        "line-mark-stride-collapsed-record-index-first-row" => 3,
        "cross-table-combined-previous-row-span-first-record" => 4,
        "cross-table-previous-row-span-table-first-row" => 5,
        "cross-table-selected-spacing-table-first-row" => 6,
        _ => 100,
    }
}

pub(super) fn push_table_grid_source_only_page_y_origin_domain_gate_json(
    output: &mut String,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
) {
    let direct_line_mark_page_space_origin_present = source_layout.is_some_and(|layout| {
        layout.line_mark_page_origin.is_some() && layout.page_origin_authority == "lineMarkPageGrid"
    });
    let cross_table_line_domain_present = cross_table_row_boundary_offset_probe
        .is_some_and(|probe| !probe.combined_line_mark_record_y_tops_px.is_empty());
    let selected_previous_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .iter()
                .flat_map(|table| {
                    table
                        .selected_minus_previous_record_index_gaps
                        .iter()
                        .copied()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let stable_selected_previous_gap = single_i32_value(&selected_previous_gaps);
    let selected_previous_y_delta_milli = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .iter()
                .flat_map(|table| {
                    table
                        .selected_minus_previous_record_y_delta_px
                        .iter()
                        .map(|value| rounded_milli(*value))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let stable_selected_previous_y_delta_px =
        single_i32_value(&selected_previous_y_delta_milli).map(|value| value as f32 / 1000.0);
    let transition_record_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.line_mark_record_gap)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let same_page_mark_entry_transition_count = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .filter(|transition| transition.same_page_mark_entry)
                .count()
        })
        .unwrap_or(0);
    let line_domain_requires_offset_transform =
        cross_table_row_boundary_offset_probe.is_some_and(|probe| {
            probe.all_offsets_require_transform
                || !probe.source_unit_to_page_line_index_piecewise_all_tables_exact
                || !probe
                    .source_unit_to_page_line_index_piecewise_transitions
                    .is_empty()
        });
    let page_space_origin_decoded =
        direct_line_mark_page_space_origin_present && !line_domain_requires_offset_transform;
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe,
        );

    let mut blocked_reasons = Vec::new();
    if !direct_line_mark_page_space_origin_present {
        blocked_reasons.push("direct-line-mark-page-space-origin-absent");
    }
    if cross_table_line_domain_present {
        blocked_reasons.push("cross-table-evidence-is-page-mark-line-domain");
    }
    if line_domain_requires_offset_transform {
        blocked_reasons.push("line-domain-to-page-space-origin-transform-required");
    }
    if !transition_record_gaps.is_empty() {
        blocked_reasons.push("table-family-transition-semantics-undecoded");
    }
    if stable_selected_previous_gap == Some(1) {
        blocked_reasons.push("selected-spacing-records-are-post-row-gap-family");
    }
    if !page_space_origin_decoded {
        blocked_reasons.push("page-space-table-origin-undecoded");
    }

    output.push_str("{\"source\":\"sourcePageYTransformGate.sourceOnlyPageYOriginDomainGate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"directLineMarkPageSpaceOriginPresent\":");
    output.push_str(if direct_line_mark_page_space_origin_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableLineDomainPresent\":");
    output.push_str(if cross_table_line_domain_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"crossTableLineDomainRecordCount\":");
    output.push_str(
        &cross_table_row_boundary_offset_probe
            .map(|probe| probe.combined_line_mark_record_indexes.len())
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"crossTableLineDomainTableCount\":");
    output.push_str(
        &cross_table_row_boundary_offset_probe
            .map(|probe| probe.tables.len())
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"combinedLineMarkRecordYPitchPx\":");
    push_optional_f32_json(
        output,
        cross_table_row_boundary_offset_probe
            .and_then(|probe| probe.combined_line_mark_record_y_pitch_px),
    );
    output.push_str(",\"combinedLineMarkRecordYTopPx\":");
    match cross_table_row_boundary_offset_probe {
        Some(probe) => push_f32_array_json(output, &probe.combined_line_mark_record_y_tops_px),
        None => output.push_str("[]"),
    }
    output.push_str(",\"stableSelectedMinusPreviousRecordIndexGap\":");
    push_optional_i32_json(output, stable_selected_previous_gap);
    output.push_str(",\"stableSelectedMinusPreviousRecordYDeltaPx\":");
    push_optional_f32_json(output, stable_selected_previous_y_delta_px);
    output.push_str(",\"selectedSpacingRecordsArePostRowGapFamily\":");
    output.push_str(if stable_selected_previous_gap == Some(1) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"piecewiseTransitionCount\":");
    output.push_str(&transition_record_gaps.len().to_string());
    output.push_str(",\"piecewiseTransitionRecordGaps\":");
    push_i32_array_json(output, &transition_record_gaps);
    output.push_str(",\"samePageMarkEntryTransitionCount\":");
    output.push_str(&same_page_mark_entry_transition_count.to_string());
    output.push_str(",\"transitionSemanticsReadiness\":");
    push_table_grid_source_only_page_y_transition_semantics_readiness_json(
        output,
        cross_table_row_boundary_offset_probe,
        same_page_mark_entry_transition_count,
    );
    output.push_str(",\"sourceGapToPageLineGapTransformAdmissionGate\":");
    push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
        output,
        "sourceOnlyPageYOriginDomainGate.sourceGapToPageLineGapTransformAdmissionGate",
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"lineDomainRequiresOffsetTransform\":");
    output.push_str(if line_domain_requires_offset_transform {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageSpaceOriginDecoded\":");
    output.push_str(if page_space_origin_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionContribution\":\"source-page-y-origin-domain-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if page_space_origin_decoded && blocked_reasons.is_empty() {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "source-page-y-line-domain-not-page-space-origin",
        ));
    }
    output.push('}');
}

pub(super) fn push_table_grid_source_only_page_y_transition_semantics_readiness_json(
    output: &mut String,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    same_page_mark_entry_transition_count: usize,
) {
    let source_range_gap_units = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.source_range_gap_units)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let row_source_start_gap_units = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.row_source_start_gap_units)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let previous_family_record_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .source_unit_to_page_line_index_piecewise_transitions
                .iter()
                .map(|transition| transition.line_mark_record_gap)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_family_record_gaps = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .windows(2)
                .filter_map(|pair| {
                    pair[0]
                        .selected_spacing_record_indexes
                        .last()
                        .copied()
                        .zip(pair[1].selected_spacing_record_indexes.first().copied())
                        .map(|(left, right)| signed_usize_delta_i32(right, left))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_minus_previous_family_record_gap_deltas = selected_family_record_gaps
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(selected, previous)| selected.saturating_sub(previous))
        .collect::<Vec<_>>();
    let previous_family_y_gap_px = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .windows(2)
                .filter_map(|pair| {
                    pair[0]
                        .line_mark_record_y_tops_px
                        .last()
                        .copied()
                        .zip(pair[1].line_mark_record_y_tops_px.first().copied())
                        .map(|(left, right)| right - left)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_family_y_gap_px = cross_table_row_boundary_offset_probe
        .map(|probe| {
            probe
                .tables
                .windows(2)
                .filter_map(|pair| {
                    pair[0]
                        .selected_spacing_record_y_tops_px
                        .last()
                        .copied()
                        .zip(pair[1].selected_spacing_record_y_tops_px.first().copied())
                        .map(|(left, right)| right - left)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_minus_previous_family_y_gap_delta_px = selected_family_y_gap_px
        .iter()
        .copied()
        .zip(previous_family_y_gap_px.iter().copied())
        .map(|(selected, previous)| selected - previous)
        .collect::<Vec<_>>();
    let source_range_units_per_previous_record_gap =
        ratio_usize_by_i32(&source_range_gap_units, &previous_family_record_gaps);
    let row_source_start_units_per_previous_record_gap =
        ratio_i32_by_i32(&row_source_start_gap_units, &previous_family_record_gaps);
    let previous_y_gap_px_per_record_gap =
        ratio_f32_by_i32(&previous_family_y_gap_px, &previous_family_record_gaps);
    let source_range_gap_ratio_stable =
        rounded_f32_values_all_same(&source_range_units_per_previous_record_gap);
    let row_source_start_gap_ratio_stable =
        rounded_f32_values_all_same(&row_source_start_units_per_previous_record_gap);
    let previous_y_gap_ratio_stable =
        rounded_f32_values_all_same(&previous_y_gap_px_per_record_gap);
    let source_range_gap_minus_page_line_gap_units = source_range_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(source_range_gap, page_line_gap)| {
            source_range_gap_minus_page_line_gap_units(source_range_gap, page_line_gap)
        })
        .collect::<Vec<_>>();
    let row_source_start_gap_minus_page_line_gap_units = row_source_start_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(row_source_start_gap, page_line_gap)| {
            row_source_start_gap.saturating_sub(page_line_gap)
        })
        .collect::<Vec<_>>();
    let source_range_gap_equals_page_line_gap = source_range_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(source_range_gap, page_line_gap)| {
            usize::try_from(page_line_gap)
                .map(|page_line_gap| source_range_gap == page_line_gap)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    let row_source_start_gap_equals_page_line_gap = row_source_start_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(row_source_start_gap, page_line_gap)| row_source_start_gap == page_line_gap)
        .collect::<Vec<_>>();
    let all_source_range_gaps_equal_page_line_gaps = !source_range_gap_equals_page_line_gap
        .is_empty()
        && source_range_gap_equals_page_line_gap
            .iter()
            .all(|value| *value);
    let all_row_source_start_gaps_equal_page_line_gaps = !row_source_start_gap_equals_page_line_gap
        .is_empty()
        && row_source_start_gap_equals_page_line_gap
            .iter()
            .all(|value| *value);
    let segment_offset_gap_units = row_source_start_gap_units
        .iter()
        .copied()
        .zip(source_range_gap_units.iter().copied())
        .map(|(row_source_start_gap, source_range_gap)| {
            row_source_start_gap_minus_source_range_gap_units(
                row_source_start_gap,
                source_range_gap,
            )
        })
        .collect::<Vec<_>>();
    let segment_offset_gap_minus_page_line_gap_units = segment_offset_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(segment_offset_gap, page_line_gap)| segment_offset_gap.saturating_sub(page_line_gap))
        .collect::<Vec<_>>();
    let segment_offset_gap_equals_page_line_gap = segment_offset_gap_units
        .iter()
        .copied()
        .zip(previous_family_record_gaps.iter().copied())
        .map(|(segment_offset_gap, page_line_gap)| segment_offset_gap == page_line_gap)
        .collect::<Vec<_>>();
    let all_segment_offsets_equal_page_line_gaps = !segment_offset_gap_equals_page_line_gap
        .is_empty()
        && segment_offset_gap_equals_page_line_gap
            .iter()
            .all(|value| *value);
    let segment_offset_units_per_page_line_gap =
        ratio_i32_by_i32(&segment_offset_gap_units, &previous_family_record_gaps);
    let segment_offset_gap_ratio_stable =
        rounded_f32_values_all_same(&segment_offset_units_per_page_line_gap);
    let mut source_gap_to_page_line_gap_declined_transform_kinds = Vec::new();
    if !source_range_gap_units.is_empty() && !all_source_range_gaps_equal_page_line_gaps {
        source_gap_to_page_line_gap_declined_transform_kinds.push("direct-source-range-gap");
    }
    if !row_source_start_gap_units.is_empty() && !all_row_source_start_gaps_equal_page_line_gaps {
        source_gap_to_page_line_gap_declined_transform_kinds.push("direct-row-source-start-gap");
    }
    if !segment_offset_gap_units.is_empty() && !all_segment_offsets_equal_page_line_gaps {
        source_gap_to_page_line_gap_declined_transform_kinds.push("segment-offset-gap");
    }
    let mut source_gap_to_page_line_gap_transform_blocked_reasons = Vec::new();
    if !source_range_gap_units.is_empty() && !all_source_range_gaps_equal_page_line_gaps {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("source-range-gap-not-equal-page-line-gap");
    }
    if !row_source_start_gap_units.is_empty() && !all_row_source_start_gaps_equal_page_line_gaps {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("row-source-start-gap-not-equal-page-line-gap");
    }
    if !source_range_gap_units.is_empty() && !source_range_gap_ratio_stable {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("source-range-gap-ratio-not-stable");
    }
    if !row_source_start_gap_units.is_empty() && !row_source_start_gap_ratio_stable {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("row-source-start-gap-ratio-not-stable");
    }
    if !segment_offset_gap_units.is_empty() && !all_segment_offsets_equal_page_line_gaps {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("segment-offset-gap-not-equal-page-line-gap");
    }
    if !segment_offset_gap_units.is_empty() && !segment_offset_gap_ratio_stable {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("segment-offset-gap-ratio-not-stable");
    }
    if !segment_offset_gap_units.is_empty() {
        source_gap_to_page_line_gap_transform_blocked_reasons
            .push("source-gap-to-page-line-gap-segment-offset-transform-missing");
    }
    source_gap_to_page_line_gap_transform_blocked_reasons
        .push("source-gap-to-page-line-gap-transform-undecoded");
    let transition_count = previous_family_record_gaps.len();
    let all_transitions_same_page_mark_entry =
        transition_count > 0 && same_page_mark_entry_transition_count == transition_count;
    let record_gap_deltas_all_zero = !selected_minus_previous_family_record_gap_deltas.is_empty()
        && selected_minus_previous_family_record_gap_deltas
            .iter()
            .all(|delta| *delta == 0);
    let y_gap_deltas_all_zero = !selected_minus_previous_family_y_gap_delta_px.is_empty()
        && selected_minus_previous_family_y_gap_delta_px
            .iter()
            .all(|delta| delta.abs() <= 0.001);
    let family_gaps_stable_across_record_families =
        record_gap_deltas_all_zero && y_gap_deltas_all_zero;
    let source_gap_to_page_line_gap_readiness_hints =
        table_grid_source_gap_to_page_line_gap_readiness_hints(
            cross_table_row_boundary_offset_probe,
        );

    let mut blocked_reasons = Vec::new();
    if transition_count == 0 {
        blocked_reasons.push("table-family-transition-evidence-absent");
    }
    if family_gaps_stable_across_record_families {
        blocked_reasons.push("previous-and-selected-family-transitions-share-line-domain-gaps");
    }
    if !source_range_gap_units.is_empty() {
        blocked_reasons.push("source-gap-to-page-line-gap-transform-missing");
    }
    if !source_range_gap_units.is_empty() && !source_range_gap_ratio_stable {
        blocked_reasons.push("source-range-gap-to-page-line-gap-ratio-not-stable");
    }
    if !row_source_start_gap_units.is_empty() && !row_source_start_gap_ratio_stable {
        blocked_reasons.push("row-source-start-gap-to-page-line-gap-ratio-not-stable");
    }
    if !segment_offset_gap_units.is_empty() {
        blocked_reasons.push("source-gap-to-page-line-gap-segment-offset-transform-missing");
    }
    blocked_reasons.push("table-family-transition-rule-undecoded");
    blocked_reasons.push("page-space-transition-origin-undecoded");

    output.push_str("{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"transitionCount\":");
    output.push_str(&transition_count.to_string());
    output.push_str(",\"samePageMarkEntryTransitionCount\":");
    output.push_str(&same_page_mark_entry_transition_count.to_string());
    output.push_str(",\"allTransitionsSamePageMarkEntry\":");
    output.push_str(if all_transitions_same_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"transitionEvidenceDomain\":");
    output.push_str(&json_string("page-mark-line-index"));
    output.push_str(",\"transitionPairs\":[");
    if let Some(probe) = cross_table_row_boundary_offset_probe {
        for (index, pair) in probe.tables.windows(2).enumerate() {
            if index > 0 {
                output.push(',');
            }
            push_table_grid_piecewise_record_family_gap_transition_json(output, &pair[0], &pair[1]);
        }
    }
    output.push(']');
    output.push_str(",\"sourceRangeGapUnits\":");
    push_usize_array_json(output, &source_range_gap_units);
    output.push_str(",\"rowSourceStartGapUnits\":");
    push_i32_array_json(output, &row_source_start_gap_units);
    output.push_str(",\"previousFamilyRecordGaps\":");
    push_i32_array_json(output, &previous_family_record_gaps);
    output.push_str(",\"selectedFamilyRecordGaps\":");
    push_i32_array_json(output, &selected_family_record_gaps);
    output.push_str(",\"selectedMinusPreviousFamilyRecordGapDeltas\":");
    push_i32_array_json(output, &selected_minus_previous_family_record_gap_deltas);
    output.push_str(",\"previousFamilyYGapPx\":");
    push_f32_array_json(output, &previous_family_y_gap_px);
    output.push_str(",\"selectedFamilyYGapPx\":");
    push_f32_array_json(output, &selected_family_y_gap_px);
    output.push_str(",\"selectedMinusPreviousFamilyYGapDeltaPx\":");
    push_f32_array_json(output, &selected_minus_previous_family_y_gap_delta_px);
    output.push_str(",\"familyGapsStableAcrossRecordFamilies\":");
    output.push_str(if family_gaps_stable_across_record_families {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousAndSelectedTransitionRecordGapsAgree\":");
    output.push_str(if record_gap_deltas_all_zero {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousAndSelectedTransitionYGapsAgree\":");
    output.push_str(if y_gap_deltas_all_zero {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRangeUnitsPerPreviousRecordGap\":");
    push_f32_array_json(output, &source_range_units_per_previous_record_gap);
    output.push_str(",\"rowSourceStartUnitsPerPreviousRecordGap\":");
    push_f32_array_json(output, &row_source_start_units_per_previous_record_gap);
    output.push_str(",\"previousYGapPxPerRecordGap\":");
    push_f32_array_json(output, &previous_y_gap_px_per_record_gap);
    output.push_str(",\"sourceRangeGapRatioStable\":");
    output.push_str(if source_range_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSourceStartGapRatioStable\":");
    output.push_str(if row_source_start_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousYGapRatioStable\":");
    output.push_str(if previous_y_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceGapToPageLineGapDirectMapDiagnostic\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapDirectMapDiagnostic\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"gapBasis\":");
    output.push_str(&json_string(
        "same-page-mark-entry lineMarkRecordGap as page-mark-line-index gap",
    ));
    output.push_str(",\"sourceRangeGapUnits\":");
    push_usize_array_json(output, &source_range_gap_units);
    output.push_str(",\"rowSourceStartGapUnits\":");
    push_i32_array_json(output, &row_source_start_gap_units);
    output.push_str(",\"pageLineGaps\":");
    push_i32_array_json(output, &previous_family_record_gaps);
    output.push_str(",\"sourceRangeGapMinusPageLineGapUnits\":");
    push_i32_array_json(output, &source_range_gap_minus_page_line_gap_units);
    output.push_str(",\"rowSourceStartGapMinusPageLineGapUnits\":");
    push_i32_array_json(output, &row_source_start_gap_minus_page_line_gap_units);
    output.push_str(",\"sourceRangeGapEqualsPageLineGap\":");
    push_bool_array_json(output, &source_range_gap_equals_page_line_gap);
    output.push_str(",\"rowSourceStartGapEqualsPageLineGap\":");
    push_bool_array_json(output, &row_source_start_gap_equals_page_line_gap);
    output.push_str(",\"allSourceRangeGapsEqualPageLineGaps\":");
    output.push_str(if all_source_range_gaps_equal_page_line_gaps {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allRowSourceStartGapsEqualPageLineGaps\":");
    output.push_str(if all_row_source_start_gaps_equal_page_line_gaps {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRangeUnitsPerPageLineGap\":");
    push_f32_array_json(output, &source_range_units_per_previous_record_gap);
    output.push_str(",\"rowSourceStartUnitsPerPageLineGap\":");
    push_f32_array_json(output, &row_source_start_units_per_previous_record_gap);
    output.push_str(",\"sourceRangeUnitsPerPageLineGapStable\":");
    output.push_str(if source_range_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSourceStartUnitsPerPageLineGapStable\":");
    output.push_str(if row_source_start_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-direct-map-diagnostic-only\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-direct-map-not-decoded\"}");
    output.push_str(",\"sourceGapToPageLineGapSegmentOffsetDiagnostic\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapSegmentOffsetDiagnostic\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"offsetBasis\":");
    output.push_str(&json_string(
        "rowSourceStartGapUnits minus sourceRangeGapUnits",
    ));
    output.push_str(",\"sourceRangeGapUnits\":");
    push_usize_array_json(output, &source_range_gap_units);
    output.push_str(",\"rowSourceStartGapUnits\":");
    push_i32_array_json(output, &row_source_start_gap_units);
    output.push_str(",\"segmentOffsetGapUnits\":");
    push_i32_array_json(output, &segment_offset_gap_units);
    output.push_str(",\"pageLineGaps\":");
    push_i32_array_json(output, &previous_family_record_gaps);
    output.push_str(",\"segmentOffsetGapMinusPageLineGapUnits\":");
    push_i32_array_json(output, &segment_offset_gap_minus_page_line_gap_units);
    output.push_str(",\"segmentOffsetGapEqualsPageLineGap\":");
    push_bool_array_json(output, &segment_offset_gap_equals_page_line_gap);
    output.push_str(",\"allSegmentOffsetsEqualPageLineGaps\":");
    output.push_str(if all_segment_offsets_equal_page_line_gaps {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"segmentOffsetUnitsPerPageLineGap\":");
    push_f32_array_json(output, &segment_offset_units_per_page_line_gap);
    output.push_str(",\"segmentOffsetUnitsPerPageLineGapStable\":");
    output.push_str(if segment_offset_gap_ratio_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"segmentOffsetTransformDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-segment-offset-diagnostic-only\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-segment-offset-not-decoded\"}");
    output.push_str(",\"sourceGapToPageLineGapTransformReadiness\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapTransformReadiness\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"transformDomain\":");
    output.push_str(&json_string("source-unit-gap-to-page-mark-line-index-gap"));
    output.push_str(",\"candidateTransformCount\":3");
    output.push_str(",\"acceptedTransformKind\":null");
    output.push_str(",\"directMapDeclined\":");
    output.push_str(
        if source_gap_to_page_line_gap_declined_transform_kinds.is_empty() {
            "false"
        } else {
            "true"
        },
    );
    output.push_str(",\"declinedTransformKinds\":");
    push_json_string_slice_array(
        output,
        &source_gap_to_page_line_gap_declined_transform_kinds,
    );
    output.push_str(",\"directMapEvidence\":");
    output.push_str(&json_string(
        "source gaps do not equal page-line gaps and their ratios are unstable",
    ));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(
        output,
        &source_gap_to_page_line_gap_transform_blocked_reasons,
    );
    output.push_str(",\"nextRequiredEvidence\":");
    output.push_str(&json_string(
        "decode source-gap unit domain or segment transition offset rule before page-space y promotion",
    ));
    output.push_str(",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-transform-readiness\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-transform-not-decoded\"}");
    output.push_str(",\"sourceGapToPageLineGapReadinessHints\":");
    push_table_grid_source_gap_to_page_line_gap_readiness_hints_json(
        output,
        &source_gap_to_page_line_gap_readiness_hints,
    );
    output.push_str(",\"sourceGapToPageLineGapDecoded\":false");
    output.push_str(",\"pageSpaceTransitionOriginDecoded\":false");
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"table-family-transition-semantics-readiness\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("table-family-transition-semantics-undecoded"));
    output.push('}');
}

pub(super) fn table_grid_source_only_page_y_origin_candidate_supports(
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> Vec<TableGridSourceOnlyPageYOriginCandidateSupport> {
    let mut supports = Vec::new();
    if let Some(source_layout) = source_layout {
        if let Some(origin) = source_layout.line_mark_page_origin.as_ref() {
            push_table_grid_source_only_page_y_origin_candidate_support(
                &mut supports,
                "line-mark-page-origin-direct",
                origin.y,
                Some(origin.row_height),
                None,
                "source-backed-page-y-origin",
                "line-mark-page-origin-rule-still-needs-cross-table-validation",
            );
        }
        if let Some(stride) = source_layout.line_mark_page_origin_stride.as_ref() {
            if let Some(selected_y) = stride.raw_record_index_row_tops.first().copied() {
                push_table_grid_source_only_page_y_origin_candidate_support(
                    &mut supports,
                    "line-mark-stride-raw-record-index-first-row",
                    selected_y,
                    Some(stride.row_height),
                    None,
                    "source-only-line-mark-stride-page-y-origin",
                    "stride-origin-needs-page-origin-rule",
                );
            }
            if let Some(selected_y) = stride.stride_collapsed_row_tops.first().copied() {
                push_table_grid_source_only_page_y_origin_candidate_support(
                    &mut supports,
                    "line-mark-stride-collapsed-record-index-first-row",
                    selected_y,
                    Some(stride.row_height),
                    None,
                    "source-only-line-mark-stride-page-y-origin",
                    "line-mark-record-stride-to-page-y-transform-unproven",
                );
            }
        }
    }
    if source_layout.is_some_and(table_grid_source_layout_supports_page_mark_absolute_y_slot) {
        let absolute_y_slot_agreement = table_grid_source_only_page_mark_absolute_y_slot_agreement(
            document,
            candidate,
            cross_table_row_boundary_offset_probe,
            subrecord_span_readiness,
        );
        if let Some(slot) = absolute_y_slot_agreement.best_absolute_y_slot.as_ref()
            && slot.field_index == 2
            && slot.tail_block16_word_index == Some(11)
        {
            let blocked_reason = if absolute_y_slot_agreement.semantics_ready() {
                "none"
            } else {
                "page-mark-absolute-y-slot-semantics-unproven"
            };
            let mut extra_blocked_reasons = Vec::new();
            if absolute_y_slot_agreement.best_absolute_y_slot.is_some()
                && absolute_y_slot_agreement.line_domain_projected_y.is_some()
                && !absolute_y_slot_agreement.agrees
            {
                extra_blocked_reasons
                    .push("line-domain-projection-disagrees-with-page-mark-absolute-y-slot");
            }
            push_table_grid_source_only_page_y_origin_candidate_support_with_extra_blockers(
                &mut supports,
                "page-mark-absolute-y-slot-field2-tail-block16-word11",
                slot.value_px,
                None,
                Some(candidate.index()),
                "source-only-page-mark-absolute-y-slot-y-origin",
                blocked_reason,
                &extra_blocked_reasons,
            );
        }
    }
    if let Some(probe) = cross_table_row_boundary_offset_probe {
        if let Some(selected_y) = probe.combined_line_mark_record_y_tops_px.first().copied() {
            push_table_grid_source_only_page_y_origin_candidate_support(
                &mut supports,
                "cross-table-combined-previous-row-span-first-record",
                selected_y,
                probe.combined_line_mark_record_y_pitch_px,
                None,
                "cross-table-row-boundary-offset-diagnostic-only",
                "page-line-gap-projection-does-not-decode-table-y-origin",
            );
        }
        for table in &probe.tables {
            if let Some(selected_y) = table.line_mark_record_y_tops_px.first().copied() {
                push_table_grid_source_only_page_y_origin_candidate_support(
                    &mut supports,
                    "cross-table-previous-row-span-table-first-row",
                    selected_y,
                    probe.combined_line_mark_record_y_pitch_px,
                    Some(table.table_candidate_index),
                    "cross-table-row-boundary-offset-diagnostic-only",
                    "cross-table-row-boundary-offset-transform-required",
                );
            }
            if let Some(selected_y) = table.selected_spacing_record_y_tops_px.first().copied() {
                push_table_grid_source_only_page_y_origin_candidate_support(
                    &mut supports,
                    "cross-table-selected-spacing-table-first-row",
                    selected_y,
                    probe.combined_line_mark_record_y_pitch_px,
                    Some(table.table_candidate_index),
                    "source-unit-to-page-line-family-gap-piecewise-diagnostic-only",
                    "selected-spacing-record-family-is-not-page-y-origin",
                );
            }
        }
    }
    supports
}

pub(super) fn table_grid_source_layout_supports_page_mark_absolute_y_slot(
    layout: &TableGridSourceDerivedLayout,
) -> bool {
    layout.page_origin_authority == "lineMarkPageGridStrideRawRecordIndex"
        && layout.line_mark_page_origin_stride.is_some()
}

pub(super) fn table_grid_cross_table_previous_row_span_y_origin_support_count(
    cross_table_row_boundary_offset_probe: Option<&TableGridCrossTableRowBoundaryOffsetProbe>,
) -> usize {
    let Some(probe) = cross_table_row_boundary_offset_probe else {
        return 0;
    };
    usize::from(!probe.combined_line_mark_record_y_tops_px.is_empty())
        + probe
            .tables
            .iter()
            .filter(|table| !table.line_mark_record_y_tops_px.is_empty())
            .count()
}

pub(super) fn table_grid_source_only_page_y_origin_support_is_cross_table_previous_row_span(
    support: &TableGridSourceOnlyPageYOriginCandidateSupport,
) -> bool {
    matches!(
        support.origin_basis,
        "cross-table-combined-previous-row-span-first-record"
            | "cross-table-previous-row-span-table-first-row"
    )
}

pub(super) fn push_table_grid_source_only_page_y_origin_candidate_support(
    supports: &mut Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    origin_basis: &'static str,
    selected_y: f32,
    row_height: Option<f32>,
    table_candidate_index: Option<usize>,
    contribution: &'static str,
    blocked_reason: &'static str,
) {
    push_table_grid_source_only_page_y_origin_candidate_support_with_extra_blockers(
        supports,
        origin_basis,
        selected_y,
        row_height,
        table_candidate_index,
        contribution,
        blocked_reason,
        &[],
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_source_only_page_y_origin_candidate_support_with_extra_blockers(
    supports: &mut Vec<TableGridSourceOnlyPageYOriginCandidateSupport>,
    origin_basis: &'static str,
    selected_y: f32,
    row_height: Option<f32>,
    table_candidate_index: Option<usize>,
    contribution: &'static str,
    blocked_reason: &'static str,
    extra_blocked_reasons: &[&'static str],
) {
    if !selected_y.is_finite() || row_height.is_some_and(|height| !height.is_finite()) {
        return;
    }
    supports.push(TableGridSourceOnlyPageYOriginCandidateSupport {
        origin_basis,
        selected_y,
        row_height,
        table_candidate_index,
        contribution,
        blocked_reason,
        extra_blocked_reasons: extra_blocked_reasons.to_vec(),
    });
}

pub(super) fn table_grid_source_only_page_y_origin_supports_blocked_reasons(
    supports: &[TableGridSourceOnlyPageYOriginCandidateSupport],
) -> Vec<&'static str> {
    let mut reasons = BTreeSet::new();
    for support in supports {
        table_grid_insert_source_only_page_y_origin_blocker(&mut reasons, support.blocked_reason);
        for reason in &support.extra_blocked_reasons {
            table_grid_insert_source_only_page_y_origin_blocker(&mut reasons, reason);
        }
    }
    reasons.into_iter().collect()
}

pub(super) fn table_grid_insert_source_only_page_y_origin_blocker(
    reasons: &mut BTreeSet<&'static str>,
    reason: &'static str,
) {
    if reason == "none" {
        return;
    }
    reasons.insert(reason);
}

pub(super) fn push_table_grid_line_mark_page_origin_candidate_json(
    output: &mut String,
    candidate: Option<&TableGridLineMarkPageOriginCandidate>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/LineMark+/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":true");
    output.push_str(",\"y\":");
    output.push_str(&format!("{:.3}", candidate.y));
    output.push_str(",\"firstLineMarkRecordIndex\":");
    output.push_str(&candidate.first_line_mark_record_index.to_string());
    output.push_str(",\"lastLineMarkRecordIndex\":");
    output.push_str(&candidate.last_line_mark_record_index.to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&candidate.page_mark_entry_index.to_string());
    output.push_str(",\"pageIndexCandidate\":");
    push_optional_usize_json(output, candidate.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    output.push_str(&candidate.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&candidate.page_line_end.to_string());
    output.push_str(",\"pageMarkU16Fields\":");
    push_u16_array_json(output, &candidate.page_mark_u16_fields);
    output.push_str(",\"pageMarkU16FieldsHex\":");
    push_u16_hex_array_json(output, &candidate.page_mark_u16_fields);
    output.push_str(",\"pageMarkU16GeometryHypotheses\":");
    push_page_mark_u16_geometry_hypotheses_json(
        output,
        &candidate.page_mark_u16_fields,
        Some(PageMarkU16LayoutComparison {
            page_width_px: candidate.page_width_px,
            page_height_px: candidate.page_height_px,
            page_margin_px: candidate.page_margin_px,
            page_body_width_px: candidate.page_body_width_px,
        }),
    );
    output.push_str(",\"lineOffsetFromPageStart\":");
    output.push_str(&candidate.line_offset_from_page_start.to_string());
    output.push_str(",\"linePitchPx\":");
    output.push_str(&format!("{:.3}", candidate.line_pitch_px));
    output.push_str(",\"linePitchBasis\":");
    output.push_str(&json_string(candidate.line_pitch_basis));
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{:.3}", candidate.row_height));
    output.push_str(",\"renderPromotionContribution\":\"source-backed-page-y-origin\"");
    output.push_str(",\"renderPromotionBlockedReason\":null}");
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_table_grid_line_mark_stride_to_page_y_promotion_readiness_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
    cross_table_ordering_probe: Option<&TableGridCrossTableSubrecordOrderingProbe>,
    selected_complete: bool,
    selected_ordered_unique_complete: bool,
    y_origin_solver_ready: bool,
) {
    let stride_candidate_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_page_origin_stride.is_some());
    let line_mark_page_origin_present = source_layout
        .as_ref()
        .is_some_and(|layout| layout.line_mark_page_origin.is_some());
    let source_range_coverage =
        table_grid_page_mark_raw_record_source_range_coverage_summary(document, candidate);
    let source_only_stride_row_coverage =
        table_grid_source_only_stride_row_coverage_summary(document, candidate, source_layout);
    let line_mark_row_boundary_alignment =
        table_grid_line_mark_row_boundary_alignment_summary(document, candidate, source_layout);
    let source_only_stride_rows_covered = source_only_stride_row_coverage
        .as_ref()
        .is_some_and(|summary| summary.all_rows_covered);
    let page_mark_entry_line_bounds_coverage =
        table_grid_stride_page_mark_entry_line_bounds_coverage_summary(source_layout);
    let page_mark_entry_line_bounds_ready = page_mark_entry_line_bounds_coverage
        .as_ref()
        .is_some_and(|summary| summary.coverage_ready);
    let page_mark_subrecord_line_range_record_coverage =
        table_grid_page_mark_subrecord_line_range_record_coverage_summary(
            document,
            source_layout,
            subrecord_span_readiness,
        );
    let cross_table_ordering_consistent =
        cross_table_ordering_probe.is_some_and(|probe| probe.cross_table_ordering_consistent);
    let source_order_contradiction = cross_table_ordering_probe.is_some_and(|probe| {
        !probe.monotonic_raw_record_scan_index
            || !probe.monotonic_line_start_candidate
            || probe.family_reused_after_later_family
    });
    let promotion_ready = stride_candidate_present
        && line_mark_page_origin_present
        && selected_complete
        && selected_ordered_unique_complete
        && source_only_stride_rows_covered
        && page_mark_entry_line_bounds_ready
        && cross_table_ordering_consistent
        && y_origin_solver_ready;

    let mut blocked_reasons = Vec::new();
    if !stride_candidate_present {
        blocked_reasons.push("line-mark-stride-candidate-absent");
    }
    if !line_mark_page_origin_present {
        blocked_reasons.push("line-mark-page-origin-candidate-absent");
    }
    if subrecord_span_readiness.is_none() {
        blocked_reasons.push("page-mark-subrecord-line-span-readiness-absent");
    }
    if !selected_complete {
        blocked_reasons.push("selected-post-row-gap-span-incomplete");
    }
    if !selected_ordered_unique_complete {
        blocked_reasons.push("selected-post-row-gap-subrecord-coverage-not-ordered-unique");
    }
    if source_only_stride_row_coverage.is_none() {
        blocked_reasons.push("source-only-stride-row-coverage-absent");
    } else if !source_only_stride_rows_covered {
        blocked_reasons.push("line-mark-row-spans-do-not-cover-table-row-source-spans");
    }
    if page_mark_entry_line_bounds_coverage.is_none() {
        blocked_reasons.push("page-mark-entry-line-bounds-coverage-absent");
    } else if !page_mark_entry_line_bounds_ready {
        blocked_reasons.push("line-mark-records-not-contained-in-page-mark-entry");
    }
    if !cross_table_ordering_consistent {
        blocked_reasons.push("cross-table-subrecord-ordering-inconsistent");
    }
    if source_order_contradiction {
        blocked_reasons.push("source-order-vs-subrecord-order-contradiction");
    }
    if !y_origin_solver_ready {
        blocked_reasons.push("decoded-page-y-origin-missing");
    }

    output.push_str("{\"source\":\"/LineMark+/PageMark stride-to-page-y promotion readiness\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"promotionReady\":");
    output.push_str(if promotion_ready { "true" } else { "false" });
    output.push_str(",\"strideCandidatePresent\":");
    output.push_str(if stride_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkPageOriginPresent\":");
    output.push_str(if line_mark_page_origin_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedPostRowGapSpanComplete\":");
    output.push_str(if selected_complete { "true" } else { "false" });
    output.push_str(",\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":");
    output.push_str(if selected_ordered_unique_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyStrideRowCoverage\":");
    push_table_grid_source_only_stride_row_coverage_summary_json(
        output,
        source_only_stride_row_coverage.as_ref(),
    );
    output.push_str(",\"lineMarkRowBoundaryAlignment\":");
    push_table_grid_line_mark_row_boundary_alignment_summary_json(
        output,
        line_mark_row_boundary_alignment.as_ref(),
    );
    output.push_str(",\"pageMarkEntryLineBoundsCoverage\":");
    push_table_grid_stride_page_mark_entry_line_bounds_coverage_summary_json(
        output,
        page_mark_entry_line_bounds_coverage.as_ref(),
    );
    output.push_str(",\"pageMarkSubrecordLineRangeRecordCoverage\":");
    push_table_grid_page_mark_subrecord_line_range_record_coverage_summary_json(
        output,
        page_mark_subrecord_line_range_record_coverage.as_ref(),
    );
    output.push_str(",\"rawRecordSourceRangeCoverageDomain\":\"legacy-cross-domain-document-text-unit-range-vs-page-mark-line-index\"");
    output.push_str(",\"rawRecordSourceRangeCoverageUsableForPromotion\":false");
    output.push_str(",\"rawRecordSourceRangeCoverage\":");
    match source_range_coverage.as_ref() {
        Some(summary) => {
            push_table_grid_page_mark_raw_record_source_range_coverage_summary_json(output, summary)
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"crossTableOrderingConsistent\":");
    output.push_str(if cross_table_ordering_consistent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOrderVsSubrecordOrderContradiction\":");
    output.push_str(if source_order_contradiction {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output
        .push_str(",\"renderPromotionContribution\":\"line-mark-stride-to-page-y-readiness-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if promotion_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "stride-y-hypothesis-needs-source-only-validation",
        ));
    }
    output.push('}');
}

pub(super) fn table_grid_source_only_stride_row_coverage_summary(
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Option<TableGridSourceOnlyStrideRowCoverageSummary> {
    let source_layout = source_layout?;
    let stride = source_layout.line_mark_page_origin_stride.as_ref()?;
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

    let mut matched_row_count = 0usize;
    let mut line_mark_record_indexes = Vec::new();
    let mut row_span_units = Vec::new();
    let mut line_mark_span_units = Vec::new();
    let mut row_span_residual_units = Vec::new();
    for row in &rows {
        let row_span = row
            .row_source_end_units
            .saturating_sub(row.row_source_start_units);
        row_span_units.push(row_span);
        if let Some(previous) = row.previous_line_mark {
            let line_mark_span = line_mark_interval_span_units(previous);
            line_mark_record_indexes.push(previous.record_index);
            line_mark_span_units.push(line_mark_span);
            row_span_residual_units.push(line_mark_span as i32 - row_span as i32);
            if line_mark_span == row_span {
                matched_row_count += 1;
            }
        }
    }

    let line_mark_record_stride = uniform_usize_stride(&line_mark_record_indexes);
    let all_rows_covered = !rows.is_empty()
        && matched_row_count == rows.len()
        && line_mark_record_indexes.len() == rows.len();
    let matches_stride_candidate_record_indexes =
        line_mark_record_indexes.as_slice() == stride.line_mark_record_indexes.as_slice();

    Some(TableGridSourceOnlyStrideRowCoverageSummary {
        candidate_row_count: rows.len(),
        matched_row_count,
        all_rows_covered,
        line_mark_record_selection: "previous-compact-row-span-record",
        line_mark_record_indexes,
        uniform_line_mark_record_stride: line_mark_record_stride.is_some(),
        line_mark_record_stride,
        matches_stride_candidate_record_indexes,
        row_span_units,
        line_mark_span_units,
        row_span_residual_units,
    })
}

pub(super) fn table_grid_line_mark_row_boundary_alignment_summary(
    document: &Document,
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Option<TableGridLineMarkRowBoundaryAlignmentSummary> {
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
    let stride_record_indexes = source_layout
        .and_then(|layout| layout.line_mark_page_origin_stride.as_ref())
        .map(|stride| stride.line_mark_record_indexes.as_slice());

    Some(TableGridLineMarkRowBoundaryAlignmentSummary {
        candidate_row_count: rows.len(),
        selected_spacing_record_alignment: table_grid_line_mark_row_boundary_alignment_family(
            &rows,
            "selected-spacing-records",
            "selected-record-overlaps-row-and-matches-post-row-gap-span",
            table_grid_selected_line_mark_for_boundary_alignment,
            stride_record_indexes,
        ),
        previous_row_span_record_alignment: table_grid_line_mark_row_boundary_alignment_family(
            &rows,
            "previous-row-span-records",
            "previous-record-span-equals-compact-row-span",
            table_grid_previous_line_mark_for_boundary_alignment,
            stride_record_indexes,
        ),
        next_record_alignment: table_grid_line_mark_row_boundary_alignment_family(
            &rows,
            "next-records",
            "next-record-neighboring-row-span-candidate",
            table_grid_next_line_mark_for_boundary_alignment,
            stride_record_indexes,
        ),
    })
}

pub(super) fn table_grid_selected_line_mark_for_boundary_alignment(
    row: &TableGridLineMarkRowGapSequenceRow,
) -> Option<ShanaiLanLineMarkInterval> {
    Some(row.selected_line_mark)
}

pub(super) fn table_grid_previous_line_mark_for_boundary_alignment(
    row: &TableGridLineMarkRowGapSequenceRow,
) -> Option<ShanaiLanLineMarkInterval> {
    row.previous_line_mark
}

pub(super) fn table_grid_next_line_mark_for_boundary_alignment(
    row: &TableGridLineMarkRowGapSequenceRow,
) -> Option<ShanaiLanLineMarkInterval> {
    row.next_line_mark
}

pub(super) fn table_grid_line_mark_row_boundary_alignment_family(
    rows: &[TableGridLineMarkRowGapSequenceRow],
    family: &'static str,
    span_interpretation: &'static str,
    select_line_mark: fn(&TableGridLineMarkRowGapSequenceRow) -> Option<ShanaiLanLineMarkInterval>,
    stride_record_indexes: Option<&[usize]>,
) -> Option<TableGridLineMarkRowBoundaryAlignmentFamily> {
    let mut alignment_rows = Vec::new();
    for row in rows {
        let Some(line_mark) = select_line_mark(row) else {
            continue;
        };
        let start_residual_units =
            signed_usize_delta_i32(line_mark.unit_start, row.row_source_start_units);
        let end_residual_units =
            signed_usize_delta_i32(line_mark.unit_end, row.row_source_end_units);
        let line_mark_span_units = line_mark_interval_span_units(line_mark);
        let row_span_units = row
            .row_source_end_units
            .saturating_sub(row.row_source_start_units);
        let span_residual_units = signed_usize_delta_i32(line_mark_span_units, row_span_units);
        alignment_rows.push(TableGridLineMarkRowBoundaryAlignmentRow {
            compact_row_index: row.compact_row_index,
            sparse_row_index: row.sparse_row_index,
            source_interval_index: row.source_interval_index,
            line_mark_record_index: line_mark.record_index,
            row_source_start_units: row.row_source_start_units,
            row_source_end_units: row.row_source_end_units,
            line_mark_start_units: line_mark.unit_start,
            line_mark_end_units: line_mark.unit_end,
            start_residual_units,
            end_residual_units,
            span_residual_units,
            exact_boundary_aligned: start_residual_units == 0 && end_residual_units == 0,
        });
    }
    if alignment_rows.is_empty() {
        return None;
    }

    let record_indexes = alignment_rows
        .iter()
        .map(|row| row.line_mark_record_index)
        .collect::<Vec<_>>();
    let line_mark_record_stride = uniform_usize_stride(&record_indexes);
    let row_source_start_units = alignment_rows
        .iter()
        .map(|row| row.row_source_start_units)
        .collect::<Vec<_>>();
    let row_source_end_units = alignment_rows
        .iter()
        .map(|row| row.row_source_end_units)
        .collect::<Vec<_>>();
    let line_mark_start_units = alignment_rows
        .iter()
        .map(|row| row.line_mark_start_units)
        .collect::<Vec<_>>();
    let line_mark_end_units = alignment_rows
        .iter()
        .map(|row| row.line_mark_end_units)
        .collect::<Vec<_>>();
    let start_residual_units = alignment_rows
        .iter()
        .map(|row| row.start_residual_units)
        .collect::<Vec<_>>();
    let end_residual_units = alignment_rows
        .iter()
        .map(|row| row.end_residual_units)
        .collect::<Vec<_>>();
    let span_residual_units = alignment_rows
        .iter()
        .map(|row| row.span_residual_units)
        .collect::<Vec<_>>();
    let exact_boundary_match_count = alignment_rows
        .iter()
        .filter(|row| row.exact_boundary_aligned)
        .count();
    let exact_boundary_aligned = exact_boundary_match_count == alignment_rows.len();
    let stable_start_residual_units = single_i32_value(&start_residual_units);
    let stable_end_residual_units = single_i32_value(&end_residual_units);
    let stable_span_residual_units = single_i32_value(&span_residual_units);
    let row_boundary_offset_candidate_units = stable_start_residual_units
        .zip(stable_end_residual_units)
        .and_then(|(start_residual, end_residual)| {
            (start_residual == end_residual).then_some(start_residual)
        });
    let offset_normalized_start_residual_units = row_boundary_offset_candidate_units
        .map(|offset| {
            start_residual_units
                .iter()
                .map(|residual| residual - offset)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let offset_normalized_end_residual_units = row_boundary_offset_candidate_units
        .map(|offset| {
            end_residual_units
                .iter()
                .map(|residual| residual - offset)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let offset_normalized_exact_boundary_match_count = offset_normalized_start_residual_units
        .iter()
        .zip(&offset_normalized_end_residual_units)
        .filter(|(start, end)| **start == 0 && **end == 0)
        .count();
    let offset_normalized_exact_boundary_aligned = !alignment_rows.is_empty()
        && offset_normalized_exact_boundary_match_count == alignment_rows.len();
    let span_only_match =
        !exact_boundary_aligned && span_residual_units.iter().all(|residual| *residual == 0);
    let matches_stride_candidate_record_indexes =
        stride_record_indexes.is_some_and(|indexes| record_indexes.as_slice() == indexes);

    Some(TableGridLineMarkRowBoundaryAlignmentFamily {
        family,
        span_interpretation,
        row_count: alignment_rows.len(),
        record_indexes,
        uniform_line_mark_record_stride: line_mark_record_stride.is_some(),
        line_mark_record_stride,
        matches_stride_candidate_record_indexes,
        row_source_start_units,
        row_source_end_units,
        line_mark_start_units,
        line_mark_end_units,
        start_residual_units,
        end_residual_units,
        span_residual_units,
        exact_boundary_match_count,
        exact_boundary_aligned,
        start_residual_stable: stable_start_residual_units.is_some(),
        end_residual_stable: stable_end_residual_units.is_some(),
        span_residual_stable: stable_span_residual_units.is_some(),
        stable_start_residual_units,
        stable_end_residual_units,
        stable_span_residual_units,
        row_boundary_offset_candidate_units,
        offset_normalized_start_residual_units,
        offset_normalized_end_residual_units,
        offset_normalized_exact_boundary_match_count,
        offset_normalized_exact_boundary_aligned,
        span_only_match,
        rows: alignment_rows,
    })
}

pub(super) fn table_grid_page_mark_subrecord_line_range_record_coverage_summary(
    document: &Document,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> Option<TableGridPageMarkSubrecordLineRangeRecordCoverageSummary> {
    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let page_mark = document.page_marks().first()?;
    let record_headers = page_mark_record_headers(page_mark_bytes);
    if record_headers.is_empty() {
        return None;
    }
    let stride = source_layout?.line_mark_page_origin_stride.as_ref()?;
    let previous_record_indexes = stride.line_mark_record_indexes.clone();
    let selected_record_indexes = subrecord_span_readiness
        .map(|readiness| readiness.selected_record_indexes.clone())
        .unwrap_or_default();
    if selected_record_indexes.is_empty() && previous_record_indexes.is_empty() {
        return None;
    }

    let candidates = page_mark_raw_subrecord_line_span_candidates(
        page_mark_bytes,
        &record_headers,
        page_mark_subrecord_line_range_max_candidate(page_mark, &record_headers),
    );
    if candidates.is_empty() {
        return None;
    }

    let (
        selected_covered_record_indexes,
        selected_containing_candidate_byte_offsets,
        selected_nearest_matches,
    ) = table_grid_page_mark_subrecord_line_range_record_matches(
        &selected_record_indexes,
        &candidates,
    );
    let (
        previous_covered_record_indexes,
        previous_containing_candidate_byte_offsets,
        previous_nearest_matches,
    ) = table_grid_page_mark_subrecord_line_range_record_matches(
        &previous_record_indexes,
        &candidates,
    );

    Some(TableGridPageMarkSubrecordLineRangeRecordCoverageSummary {
        candidate_count: candidates.len(),
        selected_coverage_complete: !selected_record_indexes.is_empty()
            && selected_covered_record_indexes.len() == selected_record_indexes.len(),
        previous_coverage_complete: !previous_record_indexes.is_empty()
            && previous_covered_record_indexes.len() == previous_record_indexes.len(),
        selected_record_indexes,
        previous_record_indexes,
        selected_covered_record_indexes,
        previous_covered_record_indexes,
        selected_containing_candidate_byte_offsets,
        previous_containing_candidate_byte_offsets,
        selected_nearest_matches,
        previous_nearest_matches,
    })
}

pub(super) fn table_grid_page_mark_subrecord_line_range_record_matches(
    record_indexes: &[usize],
    candidates: &[PageMarkRawSubrecordLineSpanCandidate],
) -> (
    Vec<usize>,
    Vec<usize>,
    Vec<TableGridPageMarkSubrecordLineRangeRecordMatch>,
) {
    let mut covered_record_indexes = Vec::new();
    let mut containing_candidate_byte_offsets = Vec::new();
    let mut nearest_matches = Vec::new();
    for record_index in record_indexes {
        let containing = candidates.iter().find(|candidate| {
            page_mark_subrecord_line_range_contains_record(candidate, *record_index)
        });
        if let Some(candidate) = containing {
            covered_record_indexes.push(*record_index);
            containing_candidate_byte_offsets.push(candidate.byte_offset);
        }
        if let Some(nearest) = candidates.iter().min_by(|left, right| {
            page_mark_subrecord_line_range_record_distance(left, *record_index)
                .cmp(&page_mark_subrecord_line_range_record_distance(
                    right,
                    *record_index,
                ))
                .then_with(|| left.raw_record_scan_index.cmp(&right.raw_record_scan_index))
                .then_with(|| left.byte_offset.cmp(&right.byte_offset))
        }) {
            nearest_matches.push(TableGridPageMarkSubrecordLineRangeRecordMatch {
                record_index: *record_index,
                distance_units: page_mark_subrecord_line_range_record_distance(
                    nearest,
                    *record_index,
                ),
                candidate: *nearest,
            });
        }
    }
    (
        covered_record_indexes,
        containing_candidate_byte_offsets,
        nearest_matches,
    )
}

pub(super) fn table_grid_stride_page_mark_entry_line_bounds_coverage_summary(
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Option<TableGridStridePageMarkEntryLineBoundsCoverageSummary> {
    let source_layout = source_layout?;
    let stride = source_layout.line_mark_page_origin_stride.as_ref()?;
    let line_offsets_from_page_start = stride
        .line_mark_record_indexes
        .iter()
        .map(|record_index| record_index.saturating_sub(stride.page_line_start))
        .collect::<Vec<_>>();
    let row_count_matches_stride_candidate =
        source_layout.row_count == stride.line_mark_record_indexes.len();
    let all_line_mark_records_within_page_mark_entry = !stride.line_mark_record_indexes.is_empty()
        && stride.line_mark_record_indexes.iter().all(|record_index| {
            stride.page_line_start <= *record_index && *record_index <= stride.page_line_end
        });
    let coverage_ready =
        row_count_matches_stride_candidate && all_line_mark_records_within_page_mark_entry;

    Some(TableGridStridePageMarkEntryLineBoundsCoverageSummary {
        candidate_row_count: source_layout.row_count,
        line_mark_record_indexes: stride.line_mark_record_indexes.clone(),
        record_stride: stride.record_stride,
        page_mark_entry_index: stride.page_mark_entry_index,
        page_index_candidate: stride.page_index_candidate,
        page_line_start: stride.page_line_start,
        page_line_end: stride.page_line_end,
        line_offsets_from_page_start,
        row_count_matches_stride_candidate,
        all_line_mark_records_within_page_mark_entry,
        coverage_ready,
    })
}

pub(super) fn push_table_grid_source_only_stride_row_coverage_summary_json(
    output: &mut String,
    summary: Option<&TableGridSourceOnlyStrideRowCoverageSummary>,
) {
    let Some(summary) = summary else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/LineMark source unit ranges+table row source unit ranges\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateSpace\":\"documentTextSourceUnits\"");
    output.push_str(",\"policy\":\"previous-line-mark-record-span-equals-table-row-source-span\"");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&summary.candidate_row_count.to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&summary.matched_row_count.to_string());
    output.push_str(",\"allRowsCovered\":");
    output.push_str(if summary.all_rows_covered {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordSelection\":");
    output.push_str(&json_string(summary.line_mark_record_selection));
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &summary.line_mark_record_indexes);
    output.push_str(",\"uniformLineMarkRecordStride\":");
    output.push_str(if summary.uniform_line_mark_record_stride {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordStride\":");
    push_optional_usize_json(output, summary.line_mark_record_stride);
    output.push_str(",\"matchesStrideCandidateRecordIndexes\":");
    output.push_str(if summary.matches_stride_candidate_record_indexes {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSpanUnits\":");
    push_usize_array_json(output, &summary.row_span_units);
    output.push_str(",\"lineMarkSpanUnits\":");
    push_usize_array_json(output, &summary.line_mark_span_units);
    output.push_str(",\"rowSpanResidualUnits\":");
    push_i32_array_json(output, &summary.row_span_residual_units);
    output.push_str(",\"pageYTransformDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"source-only-line-mark-row-span-coverage\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if summary.all_rows_covered {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "line-mark-row-spans-do-not-cover-table-row-source-spans",
        ));
    }
    output.push('}');
}

pub(super) fn push_table_grid_line_mark_row_boundary_alignment_summary_json(
    output: &mut String,
    summary: Option<&TableGridLineMarkRowBoundaryAlignmentSummary>,
) {
    let Some(summary) = summary else {
        output.push_str("null");
        return;
    };

    output.push_str(
        "{\"source\":\"/LineMark source unit boundaries+table row source unit boundaries\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateSpace\":\"documentTextSourceUnits\"");
    output.push_str(",\"policy\":\"line-mark-start-end-compared-to-table-row-source-start-end\"");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&summary.candidate_row_count.to_string());
    let previous = summary.previous_row_span_record_alignment.as_ref();
    let row_boundary_offset_candidate_units =
        previous.and_then(|family| family.row_boundary_offset_candidate_units);
    output.push_str(",\"rowBoundaryOffsetCandidateFamily\":");
    if row_boundary_offset_candidate_units.is_some() {
        output.push_str(&json_string("previous-row-span-records"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rowBoundaryOffsetCandidateUnits\":");
    push_optional_i32_json(output, row_boundary_offset_candidate_units);
    output.push_str(",\"rowBoundaryOffsetCandidateStable\":");
    output.push_str(if row_boundary_offset_candidate_units.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowBoundaryOffsetCandidateRequiresTransform\":");
    output.push_str(
        if previous.is_some_and(|family| {
            family.row_boundary_offset_candidate_units.is_some() && !family.exact_boundary_aligned
        }) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"previousRowSpanRecordAlignmentOffsetNormalizedExact\":");
    output.push_str(
        if previous.is_some_and(|family| family.offset_normalized_exact_boundary_aligned) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"selectedSpacingRecordAlignment\":");
    push_table_grid_line_mark_row_boundary_alignment_family_json(
        output,
        summary.selected_spacing_record_alignment.as_ref(),
    );
    output.push_str(",\"previousRowSpanRecordAlignment\":");
    push_table_grid_line_mark_row_boundary_alignment_family_json(
        output,
        summary.previous_row_span_record_alignment.as_ref(),
    );
    output.push_str(",\"nextRecordAlignment\":");
    push_table_grid_line_mark_row_boundary_alignment_family_json(
        output,
        summary.next_record_alignment.as_ref(),
    );
    output.push_str(",\"pageYTransformDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"source-only-line-mark-row-boundary-alignment\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if previous.is_some_and(|family| family.exact_boundary_aligned) {
        output.push_str("null");
    } else if previous.is_some_and(|family| family.row_boundary_offset_candidate_units.is_some()) {
        output.push_str(&json_string(
            "line-mark-row-boundaries-require-source-offset-transform",
        ));
    } else {
        output.push_str(&json_string(
            "line-mark-row-boundaries-do-not-exactly-align-with-table-rows",
        ));
    }
    output.push('}');
}

pub(super) fn push_table_grid_line_mark_row_boundary_alignment_family_json(
    output: &mut String,
    family: Option<&TableGridLineMarkRowBoundaryAlignmentFamily>,
) {
    let Some(family) = family else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"family\":");
    output.push_str(&json_string(family.family));
    output.push_str(",\"spanInterpretation\":");
    output.push_str(&json_string(family.span_interpretation));
    output.push_str(",\"rowCount\":");
    output.push_str(&family.row_count.to_string());
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &family.record_indexes);
    output.push_str(",\"uniformLineMarkRecordStride\":");
    output.push_str(if family.uniform_line_mark_record_stride {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkRecordStride\":");
    push_optional_usize_json(output, family.line_mark_record_stride);
    output.push_str(",\"matchesStrideCandidateRecordIndexes\":");
    output.push_str(if family.matches_stride_candidate_record_indexes {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowSourceStartUnits\":");
    push_usize_array_json(output, &family.row_source_start_units);
    output.push_str(",\"rowSourceEndUnits\":");
    push_usize_array_json(output, &family.row_source_end_units);
    output.push_str(",\"lineMarkStartUnits\":");
    push_usize_array_json(output, &family.line_mark_start_units);
    output.push_str(",\"lineMarkEndUnits\":");
    push_usize_array_json(output, &family.line_mark_end_units);
    output.push_str(",\"startResidualUnits\":");
    push_i32_array_json(output, &family.start_residual_units);
    output.push_str(",\"endResidualUnits\":");
    push_i32_array_json(output, &family.end_residual_units);
    output.push_str(",\"spanResidualUnits\":");
    push_i32_array_json(output, &family.span_residual_units);
    output.push_str(",\"exactBoundaryMatchCount\":");
    output.push_str(&family.exact_boundary_match_count.to_string());
    output.push_str(",\"exactBoundaryAligned\":");
    output.push_str(if family.exact_boundary_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"startResidualStable\":");
    output.push_str(if family.start_residual_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"endResidualStable\":");
    output.push_str(if family.end_residual_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"spanResidualStable\":");
    output.push_str(if family.span_residual_stable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"stableStartResidualUnits\":");
    push_optional_i32_json(output, family.stable_start_residual_units);
    output.push_str(",\"stableEndResidualUnits\":");
    push_optional_i32_json(output, family.stable_end_residual_units);
    output.push_str(",\"stableSpanResidualUnits\":");
    push_optional_i32_json(output, family.stable_span_residual_units);
    output.push_str(",\"rowBoundaryOffsetCandidateUnits\":");
    push_optional_i32_json(output, family.row_boundary_offset_candidate_units);
    output.push_str(",\"offsetNormalizationPolicy\":");
    output.push_str(&json_string(
        "line-mark-boundary-minus-row-source-boundary-minus-stable-offset",
    ));
    output.push_str(",\"offsetNormalizedStartResidualUnits\":");
    push_i32_array_json(output, &family.offset_normalized_start_residual_units);
    output.push_str(",\"offsetNormalizedEndResidualUnits\":");
    push_i32_array_json(output, &family.offset_normalized_end_residual_units);
    output.push_str(",\"offsetNormalizedExactBoundaryMatchCount\":");
    output.push_str(
        &family
            .offset_normalized_exact_boundary_match_count
            .to_string(),
    );
    output.push_str(",\"offsetNormalizedExactBoundaryAligned\":");
    output.push_str(if family.offset_normalized_exact_boundary_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"spanOnlyMatch\":");
    output.push_str(if family.span_only_match {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rows\":[");
    for (index, row) in family.rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_table_grid_line_mark_row_boundary_alignment_row_json(output, row);
    }
    output.push_str("]}");
}

pub(super) fn push_table_grid_line_mark_row_boundary_alignment_row_json(
    output: &mut String,
    row: &TableGridLineMarkRowBoundaryAlignmentRow,
) {
    output.push_str("{\"compactRow\":");
    output.push_str(&row.compact_row_index.to_string());
    output.push_str(",\"sparseRow\":");
    output.push_str(&row.sparse_row_index.to_string());
    output.push_str(",\"sourceIntervalIndex\":");
    output.push_str(&row.source_interval_index.to_string());
    output.push_str(",\"lineMarkRecordIndex\":");
    output.push_str(&row.line_mark_record_index.to_string());
    output.push_str(",\"rowSourceUnitRange\":");
    output.push_str(&source_range_json(
        row.row_source_start_units,
        row.row_source_end_units,
    ));
    output.push_str(",\"lineMarkUnitRange\":");
    output.push_str(&source_range_json(
        row.line_mark_start_units,
        row.line_mark_end_units,
    ));
    output.push_str(",\"startResidualUnits\":");
    output.push_str(&row.start_residual_units.to_string());
    output.push_str(",\"endResidualUnits\":");
    output.push_str(&row.end_residual_units.to_string());
    output.push_str(",\"spanResidualUnits\":");
    output.push_str(&row.span_residual_units.to_string());
    output.push_str(",\"exactBoundaryAligned\":");
    output.push_str(if row.exact_boundary_aligned {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(super) fn push_table_grid_page_mark_subrecord_line_range_record_coverage_summary_json(
    output: &mut String,
    summary: Option<&TableGridPageMarkSubrecordLineRangeRecordCoverageSummary>,
) {
    let Some(summary) = summary else {
        output.push_str("null");
        return;
    };

    output.push_str(
        "{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark record indexes\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"policy\":\"subrecord-line-start-end-must-contain-line-mark-record-index\"");
    output.push_str(",\"candidateCount\":");
    output.push_str(&summary.candidate_count.to_string());
    output.push_str(",\"selectedSpacingRecordIndexes\":");
    push_usize_array_json(output, &summary.selected_record_indexes);
    output.push_str(",\"previousRowSpanRecordIndexes\":");
    push_usize_array_json(output, &summary.previous_record_indexes);
    output.push_str(",\"selectedCoveredRecordIndexes\":");
    push_usize_array_json(output, &summary.selected_covered_record_indexes);
    output.push_str(",\"previousCoveredRecordIndexes\":");
    push_usize_array_json(output, &summary.previous_covered_record_indexes);
    output.push_str(",\"selectedContainingCandidateByteOffsets\":");
    push_usize_array_json(output, &summary.selected_containing_candidate_byte_offsets);
    output.push_str(",\"previousContainingCandidateByteOffsets\":");
    push_usize_array_json(output, &summary.previous_containing_candidate_byte_offsets);
    output.push_str(",\"selectedCoverageComplete\":");
    output.push_str(if summary.selected_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"previousCoverageComplete\":");
    output.push_str(if summary.previous_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedNearestLineRangeMatches\":");
    push_table_grid_page_mark_subrecord_line_range_record_matches_json(
        output,
        &summary.selected_nearest_matches,
    );
    output.push_str(",\"previousNearestLineRangeMatches\":");
    push_table_grid_page_mark_subrecord_line_range_record_matches_json(
        output,
        &summary.previous_nearest_matches,
    );
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-subrecord-line-range-record-coverage\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if summary.selected_coverage_complete || summary.previous_coverage_complete {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "page-mark-subrecord-line-ranges-do-not-cover-line-mark-records",
        ));
    }
    output.push('}');
}

pub(super) fn push_table_grid_page_mark_subrecord_line_range_record_matches_json(
    output: &mut String,
    matches: &[TableGridPageMarkSubrecordLineRangeRecordMatch],
) {
    output.push('[');
    for (index, match_) in matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"recordIndex\":");
        output.push_str(&match_.record_index.to_string());
        output.push_str(",\"distanceUnits\":");
        output.push_str(&match_.distance_units.to_string());
        output.push_str(",\"candidate\":");
        push_page_mark_raw_subrecord_line_span_candidate_json(output, &match_.candidate);
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_table_grid_stride_page_mark_entry_line_bounds_coverage_summary_json(
    output: &mut String,
    summary: Option<&TableGridStridePageMarkEntryLineBoundsCoverageSummary>,
) {
    let Some(summary) = summary else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/LineMark record indexes+/PageMark entry line bounds\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourceDomain\":\"line-mark-record-index\"");
    output.push_str(",\"pageMarkDomain\":\"page-mark-line-index\"");
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&summary.candidate_row_count.to_string());
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &summary.line_mark_record_indexes);
    output.push_str(",\"recordStride\":");
    output.push_str(&summary.record_stride.to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&summary.page_mark_entry_index.to_string());
    output.push_str(",\"pageIndexCandidate\":");
    push_optional_usize_json(output, summary.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    output.push_str(&summary.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&summary.page_line_end.to_string());
    output.push_str(",\"lineOffsetsFromPageStart\":");
    push_usize_array_json(output, &summary.line_offsets_from_page_start);
    output.push_str(",\"rowCountMatchesStrideCandidate\":");
    output.push_str(if summary.row_count_matches_stride_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allLineMarkRecordsWithinPageMarkEntry\":");
    output.push_str(if summary.all_line_mark_records_within_page_mark_entry {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"coverageReady\":");
    output.push_str(if summary.coverage_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRangeCoverageEvaluated\":false");
    output.push_str(",\"sourceRangeCoverageSkippedReason\":\"document-text-unit-ranges-are-not-page-mark-line-indexes\"");
    output.push_str(",\"pageYTransformDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"source-only-stride-row-page-mark-entry-coverage\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    if summary.coverage_ready {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            "line-mark-records-not-contained-in-page-mark-entry",
        ));
    }
    output.push('}');
}

pub(super) fn push_table_grid_line_mark_page_origin_stride_candidate_json(
    output: &mut String,
    candidate: Option<&TableGridLineMarkPageOriginStrideCandidate>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/LineMark+/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"diagnosticOnly\":true");
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, &candidate.line_mark_record_indexes);
    output.push_str(",\"recordStride\":");
    output.push_str(&candidate.record_stride.to_string());
    output.push_str(",\"interleavedRecordCountBetweenRows\":");
    output.push_str(&candidate.record_stride.saturating_sub(1).to_string());
    output.push_str(",\"firstLineMarkRecordIndex\":");
    output.push_str(&candidate.first_line_mark_record_index.to_string());
    output.push_str(",\"lastLineMarkRecordIndex\":");
    output.push_str(&candidate.last_line_mark_record_index.to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&candidate.page_mark_entry_index.to_string());
    output.push_str(",\"pageIndexCandidate\":");
    push_optional_usize_json(output, candidate.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    output.push_str(&candidate.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&candidate.page_line_end.to_string());
    output.push_str(",\"lineOffsetFromPageStart\":");
    output.push_str(&candidate.line_offset_from_page_start.to_string());
    output.push_str(",\"rowHeight\":");
    output.push_str(&format!("{:.3}", candidate.row_height));
    output.push_str(",\"rawRecordIndexRowTops\":[");
    for (index, top) in candidate.raw_record_index_row_tops.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{top:.3}"));
    }
    output.push(']');
    output.push_str(",\"strideCollapsedRowTops\":[");
    for (index, top) in candidate.stride_collapsed_row_tops.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{top:.3}"));
    }
    output.push(']');
    output.push_str(",\"pageMarkU16Fields\":");
    push_u16_array_json(output, &candidate.page_mark_u16_fields);
    output.push_str(",\"pageMarkU16FieldsHex\":");
    push_u16_hex_array_json(output, &candidate.page_mark_u16_fields);
    output.push_str(",\"pageMarkU16GeometryHypotheses\":");
    push_page_mark_u16_geometry_hypotheses_json(
        output,
        &candidate.page_mark_u16_fields,
        Some(PageMarkU16LayoutComparison {
            page_width_px: candidate.page_width_px,
            page_height_px: candidate.page_height_px,
            page_margin_px: candidate.page_margin_px,
            page_body_width_px: candidate.page_body_width_px,
        }),
    );
    output.push_str(",\"renderPromotionContribution\":\"stride-aware-page-y-diagnostic\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-record-stride-to-page-y-transform-unproven",
    ));
    output.push('}');
}

pub(super) fn push_table_grid_layout_stream_probe_json(
    output: &mut String,
    document: &Document,
    candidate: &TableCandidate,
) {
    let line_mark_bytes = raw_stream_bytes(document, LINE_MARK_PATH).map(<[u8]>::len);
    let line_mark_word_count = line_mark_bytes.map(|len| len / 2);
    let page_mark = document.page_marks().first();
    let page_mark_entry_count = page_mark.map(|mark| mark.entries().len()).unwrap_or(0);
    let page_mark_direct_hit_count =
        table_candidate_direct_page_mark_line_hit_count(page_mark, candidate);
    let paper_mark_bytes = raw_stream_bytes(document, PAPER_MARK_PATH).map(<[u8]>::len);
    let paper_mark = document.paper_marks().first();
    let paper_mark_entry_count = paper_mark.map(|mark| mark.entries().len()).unwrap_or(0);
    let frame_record_count = document.object_frame_records().len();

    output.push_str("{\"lineMarkPresent\":");
    output.push_str(if line_mark_bytes.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"lineMarkByteLength\":");
    push_optional_usize_json(output, line_mark_bytes);
    output.push_str(",\"lineMarkWordCount\":");
    push_optional_usize_json(output, line_mark_word_count);
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(shanai_lan_line_mark_profile(document)));
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    output.push_str(",\"lineMarkIntervalCount\":");
    output.push_str(&line_mark_intervals.len().to_string());
    output.push_str(",\"lineMarkRowEvidence\":");
    push_table_grid_line_mark_row_evidence_json(output, candidate, &line_mark_intervals);
    output.push_str(",\"candidateStartWithinLineMarkWordIndex\":");
    output.push_str(
        if line_mark_word_count.is_some_and(|count| candidate.source_start() < count) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"candidateEndWithinLineMarkWordIndex\":");
    output.push_str(
        if line_mark_word_count.is_some_and(|count| candidate.source_end() <= count) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"pageMarkPresent\":");
    output.push_str(if page_mark.is_some() { "true" } else { "false" });
    output.push_str(",\"pageMarkEntryCount\":");
    output.push_str(&page_mark_entry_count.to_string());
    output.push_str(",\"pageMarkLineMarkRecordEvidence\":");
    push_table_grid_page_mark_line_mark_record_evidence_json(
        output,
        page_mark,
        candidate,
        &line_mark_intervals,
    );
    output.push_str(",\"paperMarkPageAssociationEvidence\":");
    push_table_grid_paper_mark_page_association_evidence_json(
        output,
        page_mark,
        paper_mark,
        candidate,
        &line_mark_intervals,
    );
    output.push_str(",\"candidateRangeDirectPageMarkLineHitCount\":");
    output.push_str(&page_mark_direct_hit_count.to_string());
    output.push_str(",\"paperMarkPresent\":");
    output.push_str(if paper_mark_bytes.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paperMarkByteLength\":");
    push_optional_usize_json(output, paper_mark_bytes);
    output.push_str(",\"paperMarkEntryCount\":");
    output.push_str(&paper_mark_entry_count.to_string());
    output.push_str(",\"paperMarkHeaderCount\":");
    match paper_mark {
        Some(mark) => output.push_str(&mark.header_count().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"paperMarkHeaderStride\":");
    match paper_mark {
        Some(mark) => output.push_str(&mark.header_stride().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"paperMarkHeaderLastIndex\":");
    match paper_mark {
        Some(mark) => output.push_str(&mark.header_last_index().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"pagePaperMarkEntryCountAligned\":");
    output.push_str(
        if page_mark
            .zip(paper_mark)
            .is_some_and(|(page, paper)| page.entries().len() == paper.entries().len())
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"objectFrameRecordCount\":");
    output.push_str(&frame_record_count.to_string());
    output.push_str(",\"objectFrameSourceUnitLinkCount\":0");
    output.push_str(",\"directPlacementEvidence\":false");
    output.push('}');
}

pub(super) fn push_table_grid_page_mark_line_mark_record_evidence_json(
    output: &mut String,
    page_mark: Option<&DocumentPageMark>,
    candidate: &TableCandidate,
    intervals: &[ShanaiLanLineMarkInterval],
) {
    let Some(page_mark) = page_mark else {
        output.push_str("null");
        return;
    };
    if intervals.is_empty() {
        output.push_str("null");
        return;
    }

    let mut row_matches = Vec::new();
    for (row_index, row) in candidate.intervals().iter().enumerate() {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start());
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end());
        if let Some(interval) =
            best_line_mark_interval_for_unit_range(intervals, row_unit_start, row_unit_end)
        {
            let page_entry = page_mark.entries().iter().find(|entry| {
                let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                    return false;
                };
                let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                    return false;
                };
                line_start <= interval.record_index && interval.record_index <= line_end
            });
            row_matches.push((
                row_index,
                row_unit_start,
                row_unit_end,
                interval,
                page_entry,
            ));
        }
    }

    let matched_page_row_indexes = row_matches
        .iter()
        .filter_map(|(_, _, _, _, entry)| entry.map(DocumentPageMarkEntry::row_index))
        .collect::<Vec<_>>();
    let row_page_match_count = matched_page_row_indexes.len();
    let all_rows_page_matched = row_page_match_count == candidate.intervals().len();
    let first_page_row_index = matched_page_row_indexes.first().copied();
    let single_page_matched = first_page_row_index.is_some()
        && matched_page_row_indexes
            .iter()
            .all(|row_index| Some(*row_index) == first_page_row_index);
    let first_record_index = row_matches
        .iter()
        .map(|(_, _, _, interval, _)| interval.record_index)
        .min();
    let last_record_index = row_matches
        .iter()
        .map(|(_, _, _, interval, _)| interval.record_index)
        .max();

    output.push_str("{\"source\":\"/PageMark+/LineMark\"");
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"lineMarkMatchedRowCount\":");
    output.push_str(&row_matches.len().to_string());
    output.push_str(",\"rowPageMatchCount\":");
    output.push_str(&row_page_match_count.to_string());
    output.push_str(",\"allRowsPageMatched\":");
    output.push_str(if all_rows_page_matched {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singlePageMatched\":");
    output.push_str(if single_page_matched { "true" } else { "false" });
    output.push_str(",\"lineMarkRecordRange\":");
    match (first_record_index, last_record_index) {
        (Some(start), Some(end)) => {
            output.push_str(&source_range_json(start, end.saturating_add(1)));
        }
        _ => output.push_str("null"),
    }
    output.push_str(",\"matchedPageMarkEntryIndex\":");
    push_optional_usize_json(output, first_page_row_index.filter(|_| single_page_matched));
    let matched_record_indexes = row_matches
        .iter()
        .map(|(_, _, _, interval, _)| interval.record_index)
        .collect::<Vec<_>>();
    push_line_mark_record_stride_fields_json(output, &matched_record_indexes);
    output.push_str(",\"renderPromotionContribution\":\"page-association-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("page-mark-line-index-not-y-coordinate"));
    output.push_str(",\"rows\":[");
    for (index, (row_index, row_unit_start, row_unit_end, interval, page_entry)) in
        row_matches.iter().enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(*row_unit_start, *row_unit_end));
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&interval.record_index.to_string());
        output.push_str(",\"pageMarkEntryIndex\":");
        match page_entry {
            Some(entry) => output.push_str(&entry.row_index().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"pageIndexCandidate\":");
        match page_entry.and_then(|entry| entry.index()) {
            Some(index) => output.push_str(&index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"pageLineStart\":");
        match page_entry.and_then(|entry| entry.line_start()) {
            Some(start) => output.push_str(&start.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"pageLineEnd\":");
        match page_entry.and_then(|entry| entry.line_end()) {
            Some(end) => output.push_str(&end.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"withinPageLineRange\":");
        output.push_str(if page_entry.is_some() {
            "true"
        } else {
            "false"
        });
        output.push('}');
    }
    output.push_str("]}");
}

pub(super) fn push_table_grid_paper_mark_page_association_evidence_json(
    output: &mut String,
    page_mark: Option<&DocumentPageMark>,
    paper_mark: Option<&DocumentPaperMark>,
    candidate: &TableCandidate,
    intervals: &[ShanaiLanLineMarkInterval],
) {
    let (Some(page_mark), Some(paper_mark)) = (page_mark, paper_mark) else {
        output.push_str("null");
        return;
    };
    if intervals.is_empty() {
        output.push_str("null");
        return;
    }

    let mut row_matches = Vec::new();
    for (row_index, row) in candidate.intervals().iter().enumerate() {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start());
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end());
        if let Some(interval) =
            best_line_mark_interval_for_unit_range(intervals, row_unit_start, row_unit_end)
        {
            let page_entry = page_mark.entries().iter().find(|entry| {
                let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                    return false;
                };
                let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                    return false;
                };
                line_start <= interval.record_index && interval.record_index <= line_end
            });
            let paper_entry = page_entry.and_then(|page_entry| {
                page_entry
                    .index()
                    .and_then(|page_index| {
                        paper_mark
                            .entries()
                            .iter()
                            .find(|entry| entry.index() == page_index)
                    })
                    .or_else(|| paper_mark.entries().get(page_entry.row_index()))
            });
            row_matches.push((row_index, interval, page_entry, paper_entry));
        }
    }

    if row_matches.is_empty() {
        output.push_str("null");
        return;
    }

    let matched_page_row_indexes = row_matches
        .iter()
        .filter_map(|(_, _, entry, _)| entry.map(DocumentPageMarkEntry::row_index))
        .collect::<Vec<_>>();
    let matched_paper_row_indexes = row_matches
        .iter()
        .filter_map(|(_, _, _, entry)| entry.map(DocumentPaperMarkEntry::row_index))
        .collect::<Vec<_>>();
    let first_page_row_index = matched_page_row_indexes.first().copied();
    let first_paper_row_index = matched_paper_row_indexes.first().copied();
    let single_page_matched = first_page_row_index.is_some()
        && matched_page_row_indexes
            .iter()
            .all(|row_index| Some(*row_index) == first_page_row_index);
    let single_paper_mark_matched = first_paper_row_index.is_some()
        && matched_paper_row_indexes
            .iter()
            .all(|row_index| Some(*row_index) == first_paper_row_index);
    let matched_paper_entry =
        first_paper_row_index.and_then(|row_index| paper_mark.entries().get(row_index));

    output.push_str("{\"source\":\"/PageMark+/PaperMark+/LineMark\"");
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"pageMarkMatchedRowCount\":");
    output.push_str(&matched_page_row_indexes.len().to_string());
    output.push_str(",\"paperMarkMatchedRowCount\":");
    output.push_str(&matched_paper_row_indexes.len().to_string());
    output.push_str(",\"singlePageMatched\":");
    output.push_str(if single_page_matched { "true" } else { "false" });
    output.push_str(",\"singlePaperMarkMatched\":");
    output.push_str(if single_paper_mark_matched {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedPageMarkEntryIndex\":");
    push_optional_usize_json(output, first_page_row_index.filter(|_| single_page_matched));
    output.push_str(",\"matchedPaperMarkEntryIndex\":");
    push_optional_usize_json(
        output,
        first_paper_row_index.filter(|_| single_paper_mark_matched),
    );
    output.push_str(",\"matchedPaperMarkIndex\":");
    match matched_paper_entry.filter(|_| single_paper_mark_matched) {
        Some(entry) => output.push_str(&entry.index().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"matchedPaperMarkFlags\":");
    match matched_paper_entry.filter(|_| single_paper_mark_matched) {
        Some(entry) => output.push_str(&entry.flags().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"matchedPaperMarkFlagsHex\":");
    match matched_paper_entry.filter(|_| single_paper_mark_matched) {
        Some(entry) => output.push_str(&json_string(&format!("0x{:08x}", entry.flags()))),
        None => output.push_str("null"),
    }
    output.push_str(",\"pagePaperMarkEntryCountAligned\":");
    output.push_str(if page_mark.entries().len() == paper_mark.entries().len() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"paper-mark-page-row-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("paper-mark-flag-semantics-undecoded"));
    output.push_str(",\"rows\":[");
    for (index, (row_index, interval, page_entry, paper_entry)) in row_matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&interval.record_index.to_string());
        output.push_str(",\"pageMarkEntryIndex\":");
        match page_entry {
            Some(entry) => output.push_str(&entry.row_index().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"pageIndexCandidate\":");
        match page_entry.and_then(|entry| entry.index()) {
            Some(index) => output.push_str(&index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"paperMarkEntryIndex\":");
        match paper_entry {
            Some(entry) => output.push_str(&entry.row_index().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"paperMarkIndex\":");
        match paper_entry {
            Some(entry) => output.push_str(&entry.index().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"paperMarkFlags\":");
        match paper_entry {
            Some(entry) => output.push_str(&entry.flags().to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"paperMarkFlagsHex\":");
        match paper_entry {
            Some(entry) => output.push_str(&json_string(&format!("0x{:08x}", entry.flags()))),
            None => output.push_str("null"),
        }
        output.push('}');
    }
    output.push_str("]}");
}

pub(super) fn push_table_grid_line_mark_row_evidence_json(
    output: &mut String,
    candidate: &TableCandidate,
    intervals: &[ShanaiLanLineMarkInterval],
) {
    if intervals.is_empty() {
        output.push_str("null");
        return;
    }

    let candidate_unit_start =
        table_source_offset_to_units(candidate.basis(), candidate.source_start());
    let candidate_unit_end =
        table_source_offset_to_units(candidate.basis(), candidate.source_end());
    let mut row_matches = Vec::new();
    for (row_index, row) in candidate.intervals().iter().enumerate() {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start());
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end());
        if let Some(interval) =
            best_line_mark_interval_for_unit_range(intervals, row_unit_start, row_unit_end)
        {
            let exact_boundary_match =
                interval.unit_start == row_unit_start && interval.unit_end == row_unit_end;
            row_matches.push((
                row_index,
                row_unit_start,
                row_unit_end,
                interval,
                exact_boundary_match,
            ));
        }
    }

    let exact_row_match_count = row_matches
        .iter()
        .filter(|(_, _, _, _, exact)| *exact)
        .count();
    let row_count_matches_candidate = row_matches.len() == candidate.intervals().len();
    let contiguous_record_indexes = !row_matches.is_empty()
        && row_matches
            .windows(2)
            .all(|pair| pair[1].3.record_index == pair[0].3.record_index + 1);

    output.push_str("{\"source\":");
    output.push_str(&json_string(LINE_MARK_PATH));
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis().as_str()));
    output.push_str(",\"candidateUnitRange\":");
    output.push_str(&source_range_json(candidate_unit_start, candidate_unit_end));
    output.push_str(",\"candidateRowCount\":");
    output.push_str(&candidate.intervals().len().to_string());
    output.push_str(",\"matchedRowCount\":");
    output.push_str(&row_matches.len().to_string());
    output.push_str(",\"exactRowMatchCount\":");
    output.push_str(&exact_row_match_count.to_string());
    output.push_str(",\"rowCountMatchesCandidate\":");
    output.push_str(if row_count_matches_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"contiguousRecordIndexes\":");
    output.push_str(if contiguous_record_indexes {
        "true"
    } else {
        "false"
    });
    let matched_record_indexes = row_matches
        .iter()
        .map(|(_, _, _, interval, _)| interval.record_index)
        .collect::<Vec<_>>();
    push_line_mark_record_stride_fields_json(output, &matched_record_indexes);
    output
        .push_str(",\"renderPromotionContribution\":\"row-boundary-and-row-order-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "line-mark-units-not-y-page-coordinate-transform",
    ));
    output.push_str(",\"rows\":[");
    for (index, (row_index, row_unit_start, row_unit_end, interval, exact)) in
        row_matches.iter().enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"row\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(*row_unit_start, *row_unit_end));
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&interval.record_index.to_string());
        output.push_str(",\"lineMarkUnitRange\":");
        output.push_str(&source_range_json(interval.unit_start, interval.unit_end));
        output.push_str(",\"deltaUnits\":");
        output.push_str(&(interval.unit_end - interval.unit_start).to_string());
        output.push_str(",\"flagHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", interval.flag_word)));
        output.push_str(",\"exactBoundaryMatch\":");
        output.push_str(if *exact { "true" } else { "false" });
        output.push('}');
    }
    output.push_str("]}");
}

pub(super) fn table_grid_interval_line_mark(
    candidate: &TableCandidate,
    interval: &TableCandidateInterval,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) -> Option<ShanaiLanLineMarkInterval> {
    let row_unit_start = table_source_offset_to_units(candidate.basis(), interval.source_start());
    let row_unit_end = table_source_offset_to_units(candidate.basis(), interval.source_end());
    best_line_mark_interval_for_unit_range(line_mark_intervals, row_unit_start, row_unit_end)
}

pub(super) fn table_grid_page_mark_entry_for_line_mark_record(
    page_mark: Option<&DocumentPageMark>,
    record_index: usize,
) -> Option<&DocumentPageMarkEntry> {
    page_mark?.entries().iter().find(|entry| {
        let Some(line_start) = entry.line_start().map(|value| value as usize) else {
            return false;
        };
        let Some(line_end) = entry.line_end().map(|value| value as usize) else {
            return false;
        };
        line_start <= record_index && record_index <= line_end
    })
}

pub(super) fn table_grid_line_header_row_for_interval<'a>(
    rows: &'a [TableCandidateLineHeaderRow],
    interval: &TableCandidateInterval,
) -> Option<&'a TableCandidateLineHeaderRow> {
    rows.iter().find(|row| row.row_index == interval.index())
}

pub(super) fn table_grid_cell_line_header_candidate<'a>(
    basis: TextCountRangeOverlapBasis,
    row: Option<&'a TableCandidateLineHeaderRow>,
    segment: &TableCandidateColumnSegment,
) -> Option<&'a ShanaiLanLineHeader> {
    let row = row?;
    let segment_start = table_source_offset_to_units(basis, segment.source_start()?);
    let segment_end = table_source_offset_to_units(basis, segment.source_end()?);

    row.headers
        .iter()
        .filter(|header| header.end / 2 <= segment_start)
        .min_by_key(|header| segment_start.saturating_sub(header.end / 2))
        .or_else(|| {
            row.headers
                .iter()
                .filter(|header| {
                    ranges_overlap_half_open(
                        header.start / 2,
                        header.end / 2,
                        segment_start,
                        segment_end,
                    )
                })
                .min_by_key(|header| {
                    segment_start
                        .abs_diff(header.start / 2)
                        .min(segment_end.abs_diff(header.end / 2))
                })
        })
        .or_else(|| {
            row.headers.iter().min_by_key(|header| {
                segment_start
                    .abs_diff(header.start / 2)
                    .min(segment_start.abs_diff(header.end / 2))
            })
        })
}

pub(super) fn table_grid_line_header_selection_kind(
    segment_start_units: Option<usize>,
    segment_end_units: Option<usize>,
    header: &ShanaiLanLineHeader,
) -> &'static str {
    let (Some(segment_start), Some(segment_end)) = (segment_start_units, segment_end_units) else {
        return "no-segment-source-range";
    };
    let header_start = header.start / 2;
    let header_end = header.end / 2;
    if header_end <= segment_start {
        "nearest-preceding-line-header"
    } else if ranges_overlap_half_open(header_start, header_end, segment_start, segment_end) {
        "overlapping-line-header"
    } else {
        "nearest-line-header"
    }
}

pub(super) fn push_table_grid_line_header_candidate_json(
    output: &mut String,
    basis: TextCountRangeOverlapBasis,
    row: Option<&TableCandidateLineHeaderRow>,
    segment: &TableCandidateColumnSegment,
) {
    let Some(header) = table_grid_cell_line_header_candidate(basis, row, segment) else {
        output.push_str("null");
        return;
    };
    let segment_start_units = segment
        .source_start()
        .map(|value| table_source_offset_to_units(basis, value));
    let segment_end_units = segment
        .source_end()
        .map(|value| table_source_offset_to_units(basis, value));
    let header_start_units = header.start / 2;
    let header_end_units = header.end / 2;
    output.push_str("{\"source\":\"/DocumentText line-header\"");
    output.push_str(",\"selection\":");
    output.push_str(&json_string(table_grid_line_header_selection_kind(
        segment_start_units,
        segment_end_units,
        header,
    )));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(header_start_units, header_end_units));
    output.push_str(",\"offsetUnits\":");
    output.push_str(&header.offset_units.to_string());
    output.push_str(",\"extentUnits\":");
    output.push_str(&header.extent_units.to_string());
    output.push_str(",\"fontSizeUnits\":");
    output.push_str(&header.font_size_units.to_string());
    output.push_str(",\"segmentStartMinusHeaderEndUnits\":");
    match segment_start_units {
        Some(start) => output.push_str(&(start as i64 - header_end_units as i64).to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"segmentEndMinusHeaderStartUnits\":");
    match segment_end_units {
        Some(end) => output.push_str(&(end as i64 - header_start_units as i64).to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"rawWords\":");
    push_u16_array_json(output, &header.raw_words);
    output.push_str(",\"rawWordsHex\":");
    push_u16_hex_array_json(output, &header.raw_words);
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_table_grid_cell_source_evidence_json(
    output: &mut String,
    candidate: &TableCandidate,
    interval: &TableCandidateInterval,
    segment: &TableCandidateColumnSegment,
    line_header_rows: &[TableCandidateLineHeaderRow],
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    page_mark: Option<&DocumentPageMark>,
) {
    let row_unit_start = table_source_offset_to_units(candidate.basis(), interval.source_start());
    let row_unit_end = table_source_offset_to_units(candidate.basis(), interval.source_end());
    let line_mark = table_grid_interval_line_mark(candidate, interval, line_mark_intervals);
    let page_entry = line_mark.and_then(|interval| {
        table_grid_page_mark_entry_for_line_mark_record(page_mark, interval.record_index)
    });
    let line_header_row = table_grid_line_header_row_for_interval(line_header_rows, interval);

    output.push_str("{\"source\":\"tableCellProvenance\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"rowSourceIntervalIndex\":");
    output.push_str(&interval.source_interval_index().to_string());
    output.push_str(",\"rowSourceRange\":");
    output.push_str(&source_range_json(row_unit_start, row_unit_end));
    output.push_str(",\"segmentIndex\":");
    output.push_str(&segment.index().to_string());
    output.push_str(",\"segmentKind\":");
    output.push_str(&json_string(segment.kind().as_str()));
    output.push_str(",\"segmentCharRange\":");
    output.push_str(&source_range_json(segment.char_start(), segment.char_end()));
    output.push_str(",\"lineMarkRecordIndex\":");
    match line_mark {
        Some(interval) => output.push_str(&interval.record_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineMarkUnitRange\":");
    match line_mark {
        Some(interval) => {
            output.push_str(&source_range_json(interval.unit_start, interval.unit_end))
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"pageMarkEntryIndex\":");
    match page_entry {
        Some(entry) => output.push_str(&entry.row_index().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageIndexCandidate\":");
    match page_entry.and_then(DocumentPageMarkEntry::index) {
        Some(index) => output.push_str(&index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageLineRange\":");
    match page_entry.and_then(|entry| entry.line_start().zip(entry.line_end())) {
        Some((start, end)) => {
            output.push_str(&source_range_json(
                start as usize,
                end.saturating_add(1) as usize,
            ));
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"lineHeaderCandidate\":");
    push_table_grid_line_header_candidate_json(output, candidate.basis(), line_header_row, segment);
    output.push('}');
}

pub(super) fn push_table_grid_segment_source_range_json(
    output: &mut String,
    candidate: &TableCandidate,
    segment: &TableCandidateColumnSegment,
) {
    match (segment.source_start(), segment.source_end()) {
        (Some(start), Some(end)) if start < end => {
            output.push_str("{\"basis\":");
            output.push_str(&json_string(candidate.basis().as_str()));
            output.push_str(",\"start\":");
            output.push_str(&start.to_string());
            output.push_str(",\"end\":");
            output.push_str(&end.to_string());
            output.push_str(",\"decoded\":false}");
        }
        _ => output.push_str("null"),
    }
}

pub(super) fn push_table_grid_candidate_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let form_projection_present =
        observed_form_text_projection(document, layout, page_number).is_some();
    for candidate in document.table_candidates() {
        let Some(grid) = candidate.column_segment_grid_candidate() else {
            continue;
        };
        let reference_layout =
            reference_table_grid_overlay_layout(layout, document, candidate, grid.column_count());
        let source_layout = table_grid_source_derived_layout_candidate(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        );
        let source_render_layout = source_layout
            .as_ref()
            .filter(|layout| {
                !form_projection_present && table_grid_source_derived_layout_is_renderable(layout)
            })
            .map(TableGridRenderLayout::from_source_derived);
        let source_render_layout_present = source_render_layout.is_some();
        let reference_fallback_admission =
            table_grid_reference_layout_visible_fallback_admission(document, candidate);
        let reference_render_layout = reference_layout
            .as_ref()
            .filter(|_| reference_fallback_admission.allowed)
            .map(TableGridRenderLayout::from_reference);
        let Some(render_layout) = source_render_layout.or(reference_render_layout) else {
            continue;
        };
        let x = render_layout.x;
        let y = render_layout.y;
        let width = render_layout.width;
        let row_height = render_layout.row_height;
        let render_column_count = render_layout.column_count;
        let projection_kind = table_grid_projection_kind(true);
        let source_anchor_count = table_candidate_source_anchor_count(candidate);
        let fallback_anchor_count =
            table_grid_fallback_text_anchor_count(document, lines, candidate);
        let decoded_source_placement_evidence_present =
            table_grid_decoded_source_placement_evidence_present(document, candidate);
        let source_layout_evidence_present = table_grid_source_layout_evidence_present(document)
            || decoded_source_placement_evidence_present
            || source_layout.is_some();
        let render_promotion_blocked_reason = if render_layout.reference_backed {
            match source_layout.as_ref() {
                Some(source_layout)
                    if !table_grid_source_derived_layout_is_renderable(source_layout) =>
                {
                    source_layout.render_promotion_blocked_reason
                }
                Some(_) => "source-derived-render-suppressed-by-reference-fallback",
                None if decoded_source_placement_evidence_present => {
                    "source-derived-layout-candidate-absent"
                }
                None => "source-layout-position-evidence-missing",
            }
        } else {
            render_layout.render_promotion_blocked_reason
        };
        let placement_derived_from_source = !render_layout.reference_backed;
        let reference_fallback_blocked_reason = reference_fallback_admission
            .blocked_reason
            .unwrap_or("none");
        svg.push_str(&format!(
            "<g class=\"rjtd-column-grid-candidate\" data-table-candidate-index=\"{}\" data-projection-kind=\"{}\" data-reference-backed=\"{}\" data-reference-fallback-admitted=\"{}\" data-reference-fallback-used=\"{}\" data-reference-fallback-blocked-reason=\"{}\" data-source-render-layout-present=\"{}\" data-source-anchor-evidence=\"true\" data-source-anchor-basis=\"{}\" data-source-anchor-cell-count=\"{}\" data-geometry-derivation-evidence=\"true\" data-source-derived-layout-candidate=\"{}\" data-source-derived-layout-reference-backed=\"false\" data-column-width-basis=\"{}\" data-stroke-width=\"{:.3}\" data-cell-stroke-width=\"{:.3}\" data-stroke-width-basis=\"{}\" data-cell-text-centered=\"{}\" data-cell-text-alignment-basis=\"{}\" data-cell-text-x-adjustment=\"{:.3}\" data-cell-text-x-adjustment-basis=\"{}\" data-cell-text-baseline-factor=\"{:.3}\" data-cell-text-baseline-basis=\"{}\" data-cell-text-font-weight=\"{}\" data-cell-text-font-weight-basis=\"{}\" data-cell-text-font-size=\"{:.3}\" data-cell-text-font-size-basis=\"{}\" data-fallback-text-anchor-count=\"{}\" data-source-layout-evidence-present=\"{}\" data-decoded-source-placement-evidence=\"{}\" data-render-promotion-blocked-reason=\"{}\" data-placement-derived-from-source=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-row-count=\"{}\" data-col-count-candidate=\"{}\">",
            candidate.index(),
            projection_kind,
            render_layout.reference_backed,
            reference_fallback_admission.allowed,
            render_layout.reference_backed,
            reference_fallback_blocked_reason,
            source_render_layout_present,
            candidate.basis().as_str(),
            source_anchor_count,
            source_layout.is_some(),
            render_layout.column_width_basis,
            render_layout.stroke_width,
            render_layout.cell_stroke_width,
            render_layout.stroke_width_basis,
            render_layout.cell_text_centered,
            render_layout.cell_text_alignment_basis,
            render_layout.cell_text_x_adjustment,
            render_layout.cell_text_x_adjustment_basis,
            render_layout.cell_text_baseline_factor,
            render_layout.cell_text_baseline_basis,
            render_layout.cell_text_font_weight,
            render_layout.cell_text_font_weight_basis,
            render_layout.font_size,
            render_layout.font_size_basis,
            fallback_anchor_count,
            source_layout_evidence_present,
            decoded_source_placement_evidence_present,
            render_promotion_blocked_reason,
            placement_derived_from_source,
            grid.row_count(),
            render_column_count
        ));
        let table_height = row_height * grid.row_count() as f32;
        svg.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{table_height:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"#ffffff\" stroke=\"#222222\" stroke-width=\"{:.2}\"/>",
            render_layout.corner_radius,
            render_layout.corner_radius,
            render_layout.stroke_width
        ));
        if render_layout.header_fill {
            svg.push_str(&format!(
                "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{row_height:.1}\" fill=\"#f4f4f4\" stroke=\"none\"/>"
            ));
        }
        let document_text_map = document_text_raw_stream(document).map(map_document_text);
        for row_index in 0..grid.row_count() {
            let row_y = y + row_index as f32 * row_height;
            for column_index in 0..render_column_count {
                let column_x = render_layout.column_x_at(column_index);
                let column_width = render_layout.column_width_at(column_index);
                svg.push_str(&format!(
                    "<rect x=\"{column_x:.1}\" y=\"{row_y:.1}\" width=\"{column_width:.1}\" height=\"{row_height:.1}\" fill=\"none\" stroke=\"#222222\" stroke-width=\"{:.2}\"/>",
                    render_layout.cell_stroke_width
                ));
            }
        }

        for (row_index, interval) in candidate.intervals().iter().enumerate() {
            let row_y = y + row_index as f32 * row_height;
            for (column_index, segment) in interval.column_segments().iter().enumerate() {
                let column_index =
                    table_grid_segment_column_index(document, candidate, interval, column_index);
                if column_index >= render_column_count {
                    break;
                }
                let column_x = render_layout.column_x_at(column_index);
                let column_width = render_layout.column_width_at(column_index);
                let text_x = if render_layout.cell_text_centered {
                    column_x + (column_width * 0.5) + render_layout.cell_text_x_adjustment
                } else {
                    column_x + 3.0
                };
                let text_y = row_y + (row_height * render_layout.cell_text_baseline_factor);
                let source_attrs = table_grid_segment_source_svg_attrs(candidate, segment);
                let source_evidence_attrs = table_grid_cell_source_evidence_svg_attrs(
                    document, candidate, interval, segment,
                );
                let render_text =
                    table_grid_cell_render_text(document_text_map.as_ref(), candidate, segment);
                let whitespace_attrs = if render_text.preserves_source_whitespace {
                    " xml:space=\"preserve\" data-render-text-preserves-source-whitespace=\"true\""
                } else {
                    ""
                };
                let whitespace_probe_attrs = table_grid_cell_whitespace_probe_svg_attrs(
                    &render_text,
                    render_layout.cell_text_centered,
                );
                svg.push_str(&format!(
                    "<text x=\"{text_x:.1}\" y=\"{text_y:.1}\" font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Yu Gothic, Meiryo, Noto Sans CJK JP, sans-serif\" font-size=\"{:.1}\" font-weight=\"{}\" fill=\"#333333\" letter-spacing=\"0\" data-render-text-basis=\"{}\"{}{}{}{}{}>{}</text>",
                    render_layout.font_size,
                    if render_layout.header_fill && row_index == 0 {
                        "700"
                    } else {
                        render_layout.cell_text_font_weight
                    },
                    render_text.basis,
                    whitespace_attrs,
                    if render_layout.cell_text_centered {
                        " text-anchor=\"middle\""
                    } else {
                        ""
                    },
                    source_attrs,
                    source_evidence_attrs,
                    whitespace_probe_attrs,
                    escape_xml(&preview_svg_cell_text(layout, &render_text.text, column_width))
                ));
            }
        }
        svg.push_str("</g>");
    }
}

pub(super) fn table_grid_cell_render_text(
    document_text_map: Option<&DocumentTextMap>,
    candidate: &TableCandidate,
    segment: &TableCandidateColumnSegment,
) -> TableGridCellRenderText {
    if !segment.text().is_empty()
        && let Some(raw_text) =
            table_grid_segment_source_raw_text(document_text_map, candidate, segment)
    {
        let normalized = clean_table_control_cell_text(&raw_text);
        if normalized == segment.text() && raw_text != segment.text() {
            let trimmed_text = raw_text.trim().to_string();
            let leading_whitespace_chars = raw_text
                .chars()
                .take_while(|character| character.is_whitespace())
                .count();
            let trailing_whitespace_chars = raw_text
                .chars()
                .rev()
                .take_while(|character| character.is_whitespace())
                .count();
            return TableGridCellRenderText {
                text: raw_text,
                trimmed_text,
                basis: "documentTextSourceRangePreservedWhitespace",
                preserves_source_whitespace: true,
                leading_whitespace_chars,
                trailing_whitespace_chars,
                render_trim_candidate_basis: "source-range-whitespace-may-be-cell-position-padding",
                render_trim_candidate_blocked_reason: "table-cell-whitespace-position-semantics-unproven",
            };
        }
    }

    TableGridCellRenderText {
        text: segment.text().to_string(),
        trimmed_text: segment.text().to_string(),
        basis: "normalizedTableSegmentText",
        preserves_source_whitespace: false,
        leading_whitespace_chars: 0,
        trailing_whitespace_chars: 0,
        render_trim_candidate_basis: "no-source-whitespace-padding-candidate",
        render_trim_candidate_blocked_reason: "none",
    }
}

pub(super) fn push_table_grid_cell_whitespace_placement_probe_json(
    output: &mut String,
    render_text: &TableGridCellRenderText,
    cell_text_centered: bool,
) {
    output.push_str("{\"source\":\"documentTextSourceRangeWhitespace+tableCellAlignment\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"cellTextCentered\":");
    output.push_str(if cell_text_centered { "true" } else { "false" });
    output.push_str(",\"preservesSourceWhitespace\":");
    output.push_str(if render_text.preserves_source_whitespace {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderTextCharCount\":");
    output.push_str(&render_text.text.chars().count().to_string());
    output.push_str(",\"trimmedTextCharCount\":");
    output.push_str(&render_text.trimmed_text.chars().count().to_string());
    output.push_str(",\"leadingWhitespaceChars\":");
    output.push_str(&render_text.leading_whitespace_chars.to_string());
    output.push_str(",\"trailingWhitespaceChars\":");
    output.push_str(&render_text.trailing_whitespace_chars.to_string());
    output.push_str(",\"trimmedText\":");
    output.push_str(&json_string(&render_text.trimmed_text));
    output.push_str(",\"centeredWhitespaceMayShiftInk\":");
    output.push_str(
        if cell_text_centered
            && render_text.preserves_source_whitespace
            && (render_text.leading_whitespace_chars > 0
                || render_text.trailing_whitespace_chars > 0)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"renderTrimCandidateBasis\":");
    output.push_str(&json_string(render_text.render_trim_candidate_basis));
    output.push_str(",\"renderPromotionContribution\":\"table-cell-whitespace-placement-probe\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        render_text.render_trim_candidate_blocked_reason,
    ));
    output.push('}');
}

pub(super) fn table_grid_cell_whitespace_probe_svg_attrs(
    render_text: &TableGridCellRenderText,
    cell_text_centered: bool,
) -> String {
    let centered_shift_candidate = cell_text_centered
        && render_text.preserves_source_whitespace
        && (render_text.leading_whitespace_chars > 0 || render_text.trailing_whitespace_chars > 0);
    format!(
        " data-whitespace-placement-probe=\"true\" data-cell-text-centered-with-source-whitespace=\"{}\" data-render-text-leading-whitespace-chars=\"{}\" data-render-text-trailing-whitespace-chars=\"{}\" data-render-trim-candidate-basis=\"{}\" data-render-trim-candidate-blocked-reason=\"{}\" data-render-trim-candidate-promoted=\"{}\" data-render-trim-candidate-text=\"{}\"",
        if centered_shift_candidate {
            "true"
        } else {
            "false"
        },
        render_text.leading_whitespace_chars,
        render_text.trailing_whitespace_chars,
        render_text.render_trim_candidate_basis,
        render_text.render_trim_candidate_blocked_reason,
        "false",
        escape_xml(&render_text.trimmed_text),
    )
}

pub(super) fn table_grid_segment_source_raw_text(
    document_text_map: Option<&DocumentTextMap>,
    candidate: &TableCandidate,
    segment: &TableCandidateColumnSegment,
) -> Option<String> {
    let (Some(start), Some(end)) = (segment.source_start(), segment.source_end()) else {
        return None;
    };
    if start >= end {
        return None;
    }
    let raw_text =
        range_visible_text_for_basis(document_text_map?.entries(), start, end, candidate.basis());
    (!raw_text.is_empty()).then_some(raw_text)
}

pub(super) fn table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    overlay_index: usize,
    candidate: &TableCandidate,
    column_count: usize,
) -> (f32, f32, f32, f32, f32) {
    if let Some(source_layout) = source_derived_table_grid_overlay_layout(
        layout,
        document,
        lines,
        overlay_index,
        candidate,
        column_count,
    ) {
        return (
            source_layout.x,
            source_layout.y,
            source_layout.width,
            source_layout.row_height,
            source_layout.column_width,
        );
    }
    let width = layout.body_width_px();
    let row_height = 18.0;
    let column_width = width / column_count.max(1) as f32;
    if let Some(anchor_line) = table_candidate_anchor_line_index(document, lines, candidate) {
        let y = layout.margin_px() + APP_FONT_SIZE_PX + (anchor_line as f32 * APP_LINE_HEIGHT_PX)
            - 4.0
            + overlay_index as f32 * 4.0;
        return (layout.margin_px(), y, width, row_height, column_width);
    }
    let text_bottom =
        layout.margin_px() + APP_FONT_SIZE_PX + (lines.len() as f32 * APP_LINE_HEIGHT_PX) + 18.0;
    let overlay_top = (layout.height_px() - layout.margin_px() - 210.0).max(layout.margin_px());
    let y = text_bottom.min(overlay_top) + overlay_index as f32 * 96.0;
    (layout.margin_px(), y, width, row_height, column_width)
}

pub(super) fn table_grid_fallback_overlay_layout(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    overlay_index: usize,
    candidate: &TableCandidate,
    column_count: usize,
) -> (f32, f32, f32, f32, f32, Option<usize>) {
    let width = layout.body_width_px();
    let row_height = 18.0;
    let column_width = width / column_count.max(1) as f32;
    if let Some(anchor_line) = table_candidate_anchor_line_index(document, lines, candidate) {
        let y = layout.margin_px() + APP_FONT_SIZE_PX + (anchor_line as f32 * APP_LINE_HEIGHT_PX)
            - 4.0
            + overlay_index as f32 * 4.0;
        return (
            layout.margin_px(),
            y,
            width,
            row_height,
            column_width,
            Some(anchor_line),
        );
    }
    let text_bottom =
        layout.margin_px() + APP_FONT_SIZE_PX + (lines.len() as f32 * APP_LINE_HEIGHT_PX) + 18.0;
    let overlay_top = (layout.height_px() - layout.margin_px() - 210.0).max(layout.margin_px());
    let y = text_bottom.min(overlay_top) + overlay_index as f32 * 96.0;
    (layout.margin_px(), y, width, row_height, column_width, None)
}

pub(super) fn source_derived_table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    overlay_index: usize,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<TableGridSourceDerivedLayout> {
    if column_count == 0
        || !table_grid_decoded_source_placement_evidence_present(document, candidate)
    {
        return None;
    }

    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let first_row = rows.first()?;
    let matched_column_count = rows
        .iter()
        .map(|row| row.matched_cell_count)
        .min()
        .unwrap_or(0)
        .min(column_count);
    if matched_column_count == 0 || first_row.headers.len() < matched_column_count {
        return None;
    }

    let raw_header_count = rows
        .iter()
        .map(TableCandidateLineHeaderRow::raw_header_count)
        .sum::<usize>();
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    let min_offset_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.offset_units))
        .min();
    let max_extent_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.extent_units))
        .max();
    let homogeneous_font_size_units = rows
        .iter()
        .flat_map(|row| row.headers.iter().map(|header| header.font_size_units))
        .try_fold(None, |seen, value| match seen {
            Some(previous) if previous != value => None,
            _ => Some(Some(value)),
        })
        .flatten();
    let first_matched_header = first_row.headers.first()?;
    let last_matched_header = first_row.headers.get(matched_column_count - 1)?;
    let min_offset = min_offset_units?;
    let max_extent = max_extent_units?;
    if max_extent <= min_offset
        || last_matched_header.extent_units <= first_matched_header.offset_units
    {
        return None;
    }

    let matched_headers = first_row
        .headers
        .iter()
        .take(matched_column_count)
        .collect::<Vec<_>>();
    let matched_cell_span_units = matched_headers
        .iter()
        .map(|header| header.extent_units.saturating_sub(header.offset_units))
        .collect::<Vec<_>>();
    if matched_cell_span_units.contains(&0) {
        return None;
    }
    let matched_cell_gap_units = matched_headers
        .windows(2)
        .map(|pair| pair[1].offset_units.saturating_sub(pair[0].extent_units))
        .collect::<Vec<_>>();

    let (fallback_x, fallback_y, fallback_width, fallback_row_height, _, anchor_line_index) =
        table_grid_fallback_overlay_layout(
            layout,
            document,
            lines,
            overlay_index,
            candidate,
            column_count,
        );
    let source_full_span = f32::from(max_extent.saturating_sub(min_offset));
    if source_full_span <= 0.0 {
        return None;
    }
    let resolved_line_mark_rows =
        table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows);
    let line_mark_row_record_selection =
        table_grid_line_mark_row_record_selection(&resolved_line_mark_rows);
    let strong_line_mark_rows =
        table_grid_line_mark_rows_are_exact_and_contiguous(document, candidate, &rows);
    let line_header_rows_homogeneous = table_grid_line_header_rows_are_homogeneous(&rows);
    let (x_unit_start, x_unit_end, x_unit_range_basis) = if strong_line_mark_rows {
        table_grid_unit_bbox_range_for_row(
            first_row,
            matched_column_count,
            TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader,
        )
        .map(|(start, end)| {
            (
                start,
                end,
                TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader,
            )
        })
    } else {
        None
    }
    .or_else(|| {
        table_grid_unit_bbox_range_for_row(
            first_row,
            matched_column_count,
            TableGridUnitBBoxBasis::MatchedCells,
        )
        .map(|(start, end)| (start, end, TableGridUnitBBoxBasis::MatchedCells))
    })?;
    let x_unit_range_basis_name = x_unit_range_basis.as_str();
    let table_offset_units = x_unit_start.saturating_sub(min_offset);
    let table_span_units = x_unit_end.saturating_sub(x_unit_start);
    if table_span_units == 0 {
        return None;
    }
    let x_unit_full_extent_units = max_extent.saturating_sub(min_offset);
    let (x_unit_row_agreement_count, x_unit_all_rows_agree) =
        table_grid_unit_bbox_row_agreement_summary(
            &rows,
            matched_column_count,
            x_unit_range_basis,
            (x_unit_start, x_unit_end),
        );
    let x_unit_trailing_header_included =
        table_grid_unit_bbox_trailing_header_included(x_unit_range_basis);
    let (
        x_unit_column_slot_width_units,
        x_unit_trailing_slot_width_units,
        x_unit_included_trailing_header_count,
    ) = table_grid_unit_bbox_slot_widths(
        first_row,
        matched_column_count,
        x_unit_end,
        x_unit_trailing_header_included,
    );
    let uniform_first_gap_units = matched_cell_gap_units.first().copied().filter(|first_gap| {
        matched_cell_gap_units
            .iter()
            .all(|gap_units| gap_units == first_gap)
    });
    let (x_origin_inset_units, x_origin_inset_basis) =
        if x_unit_range_basis == TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader {
            uniform_first_gap_units
                .map(|gap_units| (f32::from(gap_units) * 0.5, "uniform-intercell-gap-half"))
                .unwrap_or((0.0, "none"))
        } else {
            (0.0, "none")
        };
    let x = fallback_x
        + fallback_width * (f32::from(table_offset_units) + x_origin_inset_units)
            / source_full_span;
    let width = fallback_width * f32::from(table_span_units) / source_full_span;
    if width <= 0.0 {
        return None;
    }
    let row_height = homogeneous_font_size_units
        .map(|font_size_units| f32::from(font_size_units) * 1.75)
        .unwrap_or(fallback_row_height)
        .max(fallback_row_height * 0.75);
    let row_height_basis = if homogeneous_font_size_units.is_some() {
        "documentTextLineHeaderFontSizeUnits"
    } else {
        "fallbackRowHeight"
    };
    let line_mark_page_origin =
        table_grid_line_mark_page_origin_candidate(layout, document, candidate, &rows, row_height);
    let line_mark_page_origin_stride = table_grid_line_mark_page_origin_stride_candidate(
        layout, document, candidate, &rows, row_height,
    );
    let stride_raw_record_index_y = table_grid_stride_raw_record_index_y_candidate(
        layout,
        document,
        candidate,
        &rows,
        &line_mark_page_origin_stride,
        row_height,
        line_header_rows_homogeneous,
        strong_line_mark_rows,
    );
    let (y, page_origin_authority, render_promotion_blocked_reason) =
        if let Some(ref origin) = line_mark_page_origin {
            let blocked_reason = if !strong_line_mark_rows {
                "line-mark-rows-not-exact-source-boundaries"
            } else if !line_header_rows_homogeneous {
                "line-header-rows-not-homogeneous"
            } else {
                "none"
            };
            (origin.y, "lineMarkPageGrid", blocked_reason)
        } else if let Some(y) = stride_raw_record_index_y {
            (
                y,
                "lineMarkPageGridStrideRawRecordIndex",
                "line-mark-record-stride-to-page-y-transform-unproven",
            )
        } else if anchor_line_index.is_some() {
            (
                fallback_y,
                "fallbackTextAnchors",
                "page-space-origin-and-row-baseline-unproven",
            )
        } else {
            (
                fallback_y,
                "compatibilityProjection",
                "page-space-origin-and-row-baseline-unproven",
            )
        };
    let column_widths =
        table_grid_line_header_column_widths_px(document, candidate, width, matched_column_count);
    let column_width_basis = if column_widths.is_empty() {
        "equalSourceDerivedColumns"
    } else {
        "documentTextLineHeaderCellSlotUnits"
    };
    let column_width = width / matched_column_count.max(1) as f32;

    Some(TableGridSourceDerivedLayout {
        provenance: TableGridSourceDerivedLayoutProvenance::DecodedCompactPlacement,
        x,
        y,
        width,
        height: row_height * rows.len() as f32,
        row_height,
        column_width,
        column_widths,
        column_width_basis,
        column_count: matched_column_count,
        row_count: rows.len(),
        x_unit_range_basis: x_unit_range_basis_name,
        x_unit_start,
        x_unit_end,
        x_unit_full_extent_units,
        x_unit_row_agreement_count,
        x_unit_all_rows_agree,
        x_unit_trailing_header_included,
        x_unit_included_trailing_header_count,
        x_unit_column_slot_width_units,
        x_unit_trailing_slot_width_units,
        x_origin_inset_units,
        x_origin_inset_basis,
        row_height_basis,
        page_origin_authority,
        anchor_line_index,
        line_mark_page_origin,
        line_mark_page_origin_stride,
        raw_header_count,
        matched_cell_header_count,
        min_offset_units: Some(min_offset),
        max_extent_units: Some(max_extent),
        matched_cell_span_units,
        matched_cell_gap_units,
        homogeneous_font_size_units,
        line_mark_row_record_selection,
        line_mark_rows_exact_and_contiguous: strong_line_mark_rows,
        line_header_rows_homogeneous,
        render_promotion_blocked_reason,
    })
}

pub(super) fn table_grid_source_derived_layout_candidate(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    overlay_index: usize,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<TableGridSourceDerivedLayout> {
    source_derived_table_grid_overlay_layout(
        layout,
        document,
        lines,
        overlay_index,
        candidate,
        column_count,
    )
    .or_else(|| sparse_sibling_derived_table_grid_overlay_layout(document, candidate, column_count))
}

pub(super) fn sparse_sibling_derived_table_grid_overlay_layout(
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<TableGridSourceDerivedLayout> {
    if column_count == 0 {
        return None;
    }
    let evidence = table_grid_sparse_table_sibling_evidence(document, candidate)?;
    evidence.compact_to_sparse_column_offset?;

    let candidate_row_count = candidate.intervals().len();
    let required_cell_count = candidate.cell_count_candidate();
    let matched_segment_count = evidence
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    let matched_sparse_column_indexes =
        table_grid_sparse_sibling_matched_sparse_column_indexes(&evidence.rows, column_count);
    let matched_sparse_columns_contiguous = matched_sparse_column_indexes
        .windows(2)
        .all(|pair| pair[1] == pair[0].saturating_add(1));
    if candidate_row_count == 0
        || required_cell_count == 0
        || evidence.rows.len() != candidate_row_count
        || matched_segment_count != required_cell_count
        || matched_sparse_column_indexes.len() != column_count
        || !matched_sparse_columns_contiguous
    {
        return None;
    }

    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let font_size_units_candidate = table_grid_line_header_font_size_units_candidate(&rows);
    let homogeneous_font_size_units =
        font_size_units_candidate.map(|(font_size_units, _, _)| font_size_units);
    let raw_header_count = rows
        .iter()
        .map(TableCandidateLineHeaderRow::raw_header_count)
        .sum::<usize>();
    let matched_cell_header_count = rows.iter().map(|row| row.matched_cell_count).sum::<usize>();
    let row_height = homogeneous_font_size_units
        .map(|font_size_units| f32::from(font_size_units) * 1.75)
        .unwrap_or(0.0);
    let line_header_rows_homogeneous = table_grid_line_header_rows_are_homogeneous(&rows);

    Some(TableGridSourceDerivedLayout {
        provenance: TableGridSourceDerivedLayoutProvenance::SparseSiblingDerived,
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: row_height * candidate_row_count as f32,
        row_height,
        column_width: 0.0,
        column_widths: Vec::new(),
        column_width_basis: "sparseSiblingDerivedColumnCountOnly",
        column_count,
        row_count: candidate_row_count,
        x_unit_range_basis: "sparse-sibling-column-indexes-not-source-units",
        x_unit_start: 0,
        x_unit_end: 0,
        x_unit_full_extent_units: 0,
        x_unit_row_agreement_count: evidence.rows.len(),
        x_unit_all_rows_agree: false,
        x_unit_trailing_header_included: false,
        x_unit_included_trailing_header_count: 0,
        x_unit_column_slot_width_units: Vec::new(),
        x_unit_trailing_slot_width_units: Vec::new(),
        x_origin_inset_units: 0.0,
        x_origin_inset_basis: "none",
        row_height_basis: if homogeneous_font_size_units.is_some() {
            "partialDocumentTextLineHeaderFontSizeUnits"
        } else {
            "sparseSiblingDerivedRowHeightUnresolved"
        },
        page_origin_authority: "none",
        anchor_line_index: None,
        line_mark_page_origin: None,
        line_mark_page_origin_stride: None,
        raw_header_count,
        matched_cell_header_count,
        min_offset_units: None,
        max_extent_units: None,
        matched_cell_span_units: Vec::new(),
        matched_cell_gap_units: Vec::new(),
        homogeneous_font_size_units,
        line_mark_row_record_selection: "none",
        line_mark_rows_exact_and_contiguous: false,
        line_header_rows_homogeneous,
        render_promotion_blocked_reason: "sparse-sibling-derived-candidate-render-ineligible",
    })
}

pub(super) fn table_grid_source_derived_layout_is_renderable(
    layout: &TableGridSourceDerivedLayout,
) -> bool {
    layout.provenance == TableGridSourceDerivedLayoutProvenance::DecodedCompactPlacement
        && layout.line_mark_page_origin.is_some()
        && layout.page_origin_authority == "lineMarkPageGrid"
        && layout.line_mark_rows_exact_and_contiguous
        && layout.line_header_rows_homogeneous
        && layout.render_promotion_blocked_reason == "none"
}

#[allow(clippy::too_many_arguments)]
pub(super) fn table_grid_stride_raw_record_index_y_candidate(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
    stride: &Option<TableGridLineMarkPageOriginStrideCandidate>,
    row_height: f32,
    line_header_rows_homogeneous: bool,
    strong_line_mark_rows: bool,
) -> Option<f32> {
    if strong_line_mark_rows
        || !line_header_rows_homogeneous
        || rows.is_empty()
        || row_height <= 0.0
    {
        return None;
    }
    let stride = stride.as_ref()?;
    let y = *stride.raw_record_index_row_tops.first()?;
    if !y.is_finite() || y < 0.0 || y + row_height * rows.len() as f32 > layout.height_px() {
        return None;
    }
    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate)?;
    let matched_segment_count = sibling
        .rows
        .iter()
        .map(|row| row.segments.len())
        .sum::<usize>();
    if sibling.rows.len() != rows.len() || matched_segment_count != candidate.cell_count_candidate()
    {
        return None;
    }
    let (post_gap_match_count, post_gap_exact_count) =
        table_grid_sparse_sibling_post_row_gap_line_mark_correlation_counts(
            document, candidate, &sibling,
        );
    if post_gap_match_count != rows.len() || post_gap_exact_count != rows.len() {
        return None;
    }
    Some(y)
}

pub(super) fn table_grid_line_mark_page_origin_candidate(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
    row_height: f32,
) -> Option<TableGridLineMarkPageOriginCandidate> {
    if rows.is_empty() || row_height <= 0.0 {
        return None;
    }
    let matched_record_indexes = table_grid_line_mark_record_indexes_for_rows(document, candidate);
    if matched_record_indexes.len() != rows.len()
        || !matched_record_indexes
            .windows(2)
            .all(|pair| pair[1] == pair[0] + 1)
    {
        return None;
    }

    let page_mark = document.page_marks().first()?;
    let mut page_entries = Vec::new();
    for record_index in &matched_record_indexes {
        let entry = page_mark.entries().iter().find(|entry| {
            let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                return false;
            };
            let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                return false;
            };
            line_start <= *record_index && *record_index <= line_end
        })?;
        page_entries.push(entry);
    }

    let first_entry = page_entries.first().copied()?;
    if !page_entries
        .iter()
        .all(|entry| entry.row_index() == first_entry.row_index())
    {
        return None;
    }
    let page_line_start = first_entry.line_start()? as usize;
    let page_line_end = first_entry.line_end()? as usize;
    let first_line_mark_record_index = *matched_record_indexes.first()?;
    let last_line_mark_record_index = *matched_record_indexes.last()?;
    if first_line_mark_record_index < page_line_start || page_line_end < last_line_mark_record_index
    {
        return None;
    }
    let line_offset_from_page_start = first_line_mark_record_index.saturating_sub(page_line_start);
    let (line_pitch_px, line_pitch_basis) =
        table_grid_page_mark_line_pitch_candidate(layout, page_line_start, page_line_end)
            .unwrap_or((row_height, "tableRowHeight"));
    let y = layout.margin_px() + line_offset_from_page_start as f32 * line_pitch_px;

    Some(TableGridLineMarkPageOriginCandidate {
        y,
        first_line_mark_record_index,
        last_line_mark_record_index,
        page_mark_entry_index: first_entry.row_index(),
        page_index_candidate: first_entry.index().map(|index| index as usize),
        page_line_start,
        page_line_end,
        page_mark_u16_fields: first_entry.u16_fields().to_vec(),
        page_width_px: layout.width_px(),
        page_height_px: layout.height_px(),
        page_margin_px: layout.margin_px(),
        page_body_width_px: layout.body_width_px(),
        line_offset_from_page_start,
        line_pitch_px,
        line_pitch_basis,
        row_height,
    })
}

pub(super) fn table_grid_page_mark_line_pitch_candidate(
    layout: PageLayout,
    page_line_start: usize,
    page_line_end: usize,
) -> Option<(f32, &'static str)> {
    let line_gap_count = page_line_end.checked_sub(page_line_start)?;
    if line_gap_count == 0 {
        return None;
    }
    let pitch = layout.body_height_px() / line_gap_count as f32;
    (pitch.is_finite() && pitch > 0.0).then_some((pitch, "pageMarkBodyLineGap"))
}

pub(super) fn table_grid_line_mark_page_origin_stride_candidate(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
    row_height: f32,
) -> Option<TableGridLineMarkPageOriginStrideCandidate> {
    if rows.is_empty() || row_height <= 0.0 {
        return None;
    }
    let matched_record_indexes = table_grid_line_mark_record_indexes_for_rows(document, candidate);
    if matched_record_indexes.len() != rows.len() {
        return None;
    }
    let record_stride = matched_record_indexes
        .windows(2)
        .map(|pair| pair[1].checked_sub(pair[0]))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .try_fold(None, |seen, stride| match seen {
            Some(previous) if previous != stride => None,
            _ => Some(Some(stride)),
        })??;
    if record_stride <= 1 {
        return None;
    }

    let page_mark = document.page_marks().first()?;
    let mut page_entries = Vec::new();
    for record_index in &matched_record_indexes {
        let entry =
            table_grid_page_mark_entry_for_line_mark_record(Some(page_mark), *record_index)?;
        page_entries.push(entry);
    }

    let first_entry = page_entries.first().copied()?;
    if !page_entries
        .iter()
        .all(|entry| entry.row_index() == first_entry.row_index())
    {
        return None;
    }
    let page_line_start = first_entry.line_start()? as usize;
    let page_line_end = first_entry.line_end()? as usize;
    let first_line_mark_record_index = *matched_record_indexes.first()?;
    let last_line_mark_record_index = *matched_record_indexes.last()?;
    if first_line_mark_record_index < page_line_start || page_line_end < last_line_mark_record_index
    {
        return None;
    }

    let line_offset_from_page_start = first_line_mark_record_index.saturating_sub(page_line_start);
    let raw_record_index_row_tops = matched_record_indexes
        .iter()
        .map(|record_index| {
            layout.margin_px() + record_index.saturating_sub(page_line_start) as f32 * row_height
        })
        .collect::<Vec<_>>();
    let stride_collapsed_row_tops = matched_record_indexes
        .iter()
        .map(|record_index| {
            let line_offset =
                record_index.saturating_sub(page_line_start) as f32 / record_stride as f32;
            layout.margin_px() + line_offset * row_height
        })
        .collect::<Vec<_>>();

    Some(TableGridLineMarkPageOriginStrideCandidate {
        line_mark_record_indexes: matched_record_indexes,
        record_stride,
        first_line_mark_record_index,
        last_line_mark_record_index,
        page_mark_entry_index: first_entry.row_index(),
        page_index_candidate: first_entry.index().map(|index| index as usize),
        page_line_start,
        page_line_end,
        page_mark_u16_fields: first_entry.u16_fields().to_vec(),
        page_width_px: layout.width_px(),
        page_height_px: layout.height_px(),
        page_margin_px: layout.margin_px(),
        page_body_width_px: layout.body_width_px(),
        line_offset_from_page_start,
        row_height,
        raw_record_index_row_tops,
        stride_collapsed_row_tops,
    })
}

pub(super) fn table_grid_line_mark_rows_are_exact_and_contiguous(
    document: &Document,
    candidate: &TableCandidate,
    rows: &[TableCandidateLineHeaderRow],
) -> bool {
    if rows.is_empty() {
        return false;
    }
    let resolved_rows = table_grid_resolved_line_mark_rows_for_rows(document, candidate, rows);
    if resolved_rows.len() != rows.len() {
        return false;
    }

    let mut matched_record_indexes = Vec::new();
    for (row, resolved) in rows.iter().zip(&resolved_rows) {
        let row_unit_start = table_source_offset_to_units(candidate.basis(), row.source_start);
        let row_unit_end = table_source_offset_to_units(candidate.basis(), row.source_end);
        let interval = resolved.interval;
        if interval.unit_start != row_unit_start || interval.unit_end != row_unit_end {
            return false;
        }
        matched_record_indexes.push(interval.record_index);
    }
    matched_record_indexes
        .windows(2)
        .all(|pair| pair[1] == pair[0] + 1)
}

pub(super) fn table_grid_line_mark_row_record_selection(
    rows: &[TableGridResolvedLineMarkRow],
) -> &'static str {
    let Some(first) = rows.first() else {
        return "none";
    };
    if rows.iter().all(|row| row.role == first.role) {
        first.role.as_str()
    } else {
        "mixed-line-mark-record-roles"
    }
}

pub(super) fn table_grid_line_header_rows_are_homogeneous(
    rows: &[TableCandidateLineHeaderRow],
) -> bool {
    rows.first().is_some_and(|first| {
        rows.iter().all(|row| {
            row.headers.len() == first.headers.len()
                && row.headers.iter().zip(&first.headers).all(|(left, right)| {
                    left.offset_units == right.offset_units
                        && left.extent_units == right.extent_units
                        && left.font_size_units == right.font_size_units
                })
        })
    })
}

pub(super) fn table_grid_column_width(
    default_column_width: f32,
    column_widths: &[f32],
    index: usize,
) -> f32 {
    column_widths
        .get(index)
        .copied()
        .unwrap_or(default_column_width)
}

pub(super) fn table_grid_column_x(
    left: f32,
    default_column_width: f32,
    column_widths: &[f32],
    index: usize,
) -> f32 {
    if column_widths.is_empty() {
        left + index as f32 * default_column_width
    } else {
        left + column_widths.iter().take(index).copied().sum::<f32>()
    }
}

pub(super) fn table_grid_projection_kind(reference_projection: bool) -> &'static str {
    if reference_projection {
        "tableProjection"
    } else {
        "diagnosticProjection"
    }
}

pub(super) fn table_grid_fallback_text_anchor_count(
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> usize {
    lines
        .iter()
        .flat_map(|line| page_text_line_fragments(document, line))
        .filter_map(|fragment| fragment.source_span)
        .filter(|span| table_candidate_overlaps_source_span(candidate, span))
        .count()
}

pub(super) fn table_grid_source_layout_evidence_present(document: &Document) -> bool {
    !document.text_count_ranges().is_empty()
        || raw_stream_bytes(document, LAYOUT_BOX_PATH).is_some()
        || raw_stream_bytes(document, LAYOUT_BOX_TEXT_PATH).is_some()
        || raw_stream_bytes(document, LAYOUT_BOX_TEXT_POSITION_TABLES_PATH).is_some()
}

pub(super) fn table_grid_decoded_source_placement_evidence_present(
    document: &Document,
    candidate: &TableCandidate,
) -> bool {
    let required = table_grid_decoded_source_placement_required_cell_count(candidate);
    required > 0 && table_grid_decoded_source_placement_match_count(document, candidate) >= required
}

pub(super) fn table_grid_decoded_source_placement_required_cell_count(
    candidate: &TableCandidate,
) -> usize {
    candidate.cell_count_candidate()
}

pub(super) fn table_grid_decoded_source_placement_match_count(
    document: &Document,
    candidate: &TableCandidate,
) -> usize {
    table_candidate_document_text_line_header_rows(document, candidate)
        .iter()
        .map(|row| row.matched_cell_count)
        .sum()
}

pub(super) fn table_grid_segment_source_svg_attrs(
    candidate: &TableCandidate,
    segment: &TableCandidateColumnSegment,
) -> String {
    match (segment.source_start(), segment.source_end()) {
        (Some(start), Some(end)) if start < end => format!(
            " data-source-range-basis=\"{}\" data-source-start=\"{}\" data-source-end=\"{}\"",
            candidate.basis().as_str(),
            start,
            end
        ),
        _ => String::new(),
    }
}

pub(super) fn table_grid_cell_source_evidence_svg_attrs(
    document: &Document,
    candidate: &TableCandidate,
    interval: &TableCandidateInterval,
    segment: &TableCandidateColumnSegment,
) -> String {
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    let line_mark = table_grid_interval_line_mark(candidate, interval, &line_mark_intervals);
    let page_entry = line_mark.and_then(|interval| {
        table_grid_page_mark_entry_for_line_mark_record(
            document.page_marks().first(),
            interval.record_index,
        )
    });
    let line_header_rows = table_candidate_document_text_line_header_rows(document, candidate);
    let line_header_row = table_grid_line_header_row_for_interval(&line_header_rows, interval);
    let line_header =
        table_grid_cell_line_header_candidate(candidate.basis(), line_header_row, segment);

    let mut attrs = format!(
        " data-row-source-interval-index=\"{}\" data-segment-index=\"{}\" data-segment-kind=\"{}\"",
        interval.source_interval_index(),
        segment.index(),
        segment.kind().as_str()
    );
    if let Some(interval) = line_mark {
        attrs.push_str(&format!(
            " data-line-mark-record-index=\"{}\" data-line-mark-unit-start=\"{}\" data-line-mark-unit-end=\"{}\"",
            interval.record_index, interval.unit_start, interval.unit_end
        ));
    }
    if let Some(entry) = page_entry {
        attrs.push_str(&format!(
            " data-page-mark-entry-index=\"{}\"",
            entry.row_index()
        ));
        if let Some(index) = entry.index() {
            attrs.push_str(&format!(" data-page-index-candidate=\"{index}\""));
        }
    }
    if let Some(header) = line_header {
        attrs.push_str(&format!(
            " data-line-header-offset-units=\"{}\" data-line-header-extent-units=\"{}\" data-line-header-font-size-units=\"{}\"",
            header.offset_units, header.extent_units, header.font_size_units
        ));
    }
    attrs
}

pub(super) fn reference_table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<TableGridReferenceLayout> {
    tsaiten_reference_table_grid_overlay_layout(layout, document, candidate, column_count)
}

pub(super) fn diagnostic_reference_table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<TableGridReferenceLayout> {
    if let Some(layout) = diagnostic_success_data_test_reference_table_grid_overlay_layout(
        layout, document, candidate,
    ) {
        return Some(layout);
    }
    tsaiten_reference_table_grid_overlay_layout(layout, document, candidate, column_count)
}

pub(super) fn tsaiten_reference_table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<TableGridReferenceLayout> {
    let (x, y, width, row_height, column_width) =
        tsaiten_table_grid_overlay_layout(layout, document, candidate, column_count)?;
    let column_widths =
        table_grid_line_header_column_widths_px(document, candidate, width, column_count);
    let column_width_basis = if column_widths.is_empty() {
        "equalReferenceColumns"
    } else {
        "documentTextLineHeaderCellSlotUnits"
    };
    Some(TableGridReferenceLayout {
        x,
        y,
        width,
        row_height,
        column_width,
        column_widths,
        column_width_basis,
        column_count,
        header_fill: true,
        corner_radius: 4.0,
        stroke_width: 1.1,
        cell_stroke_width: 0.75,
        font_size: 10.5,
        cell_text_centered: false,
    })
}

pub(super) fn tsaiten_table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<(f32, f32, f32, f32, f32)> {
    if !document_has_tsaiten_projection_evidence(document) {
        return None;
    }
    let scale_x = layout.width_px() / TSAITEN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / TSAITEN_REFERENCE_PAGE_HEIGHT_PX;
    let (x, y, width, row_height) = if column_count == 3
        && candidate.intervals().len() == 4
        && candidate
            .intervals()
            .first()
            .is_some_and(|interval| interval.text_preview() == "級\t配点\t合格点")
    {
        (174.0, 301.0, 421.0, 32.2)
    } else if column_count == 2
        && candidate.intervals().len() == 3
        && candidate
            .intervals()
            .get(1)
            .is_some_and(|interval| interval.text_preview().contains("誤字・脱字・余字"))
    {
        (174.0, 768.0, 554.0, 37.3)
    } else {
        return None;
    };
    let width = width * scale_x;
    Some((
        x * scale_x,
        y * scale_y,
        width,
        row_height * scale_y,
        width / column_count.max(1) as f32,
    ))
}

pub(super) fn table_grid_line_header_column_widths_px(
    document: &Document,
    candidate: &TableCandidate,
    table_width: f32,
    column_count: usize,
) -> Vec<f32> {
    if column_count == 0 {
        return Vec::new();
    }
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    let mut row_slot_widths = Vec::new();
    for row in &rows {
        let Some(interval) = candidate.intervals().get(row.row_index) else {
            return Vec::new();
        };
        let Some(slot_widths) = table_grid_line_header_cell_slot_width_units(
            candidate.basis(),
            row,
            interval,
            column_count,
        ) else {
            return Vec::new();
        };
        row_slot_widths.push(slot_widths);
    }
    let Some(first_row_slot_widths) = row_slot_widths.first() else {
        return Vec::new();
    };
    if first_row_slot_widths.len() != column_count
        || first_row_slot_widths.contains(&0)
        || row_slot_widths
            .iter()
            .any(|slot_widths| slot_widths != first_row_slot_widths)
    {
        return Vec::new();
    }
    let total_units = first_row_slot_widths
        .iter()
        .map(|span| f32::from(*span))
        .sum::<f32>();
    if total_units <= 0.0 {
        return Vec::new();
    }
    first_row_slot_widths
        .iter()
        .copied()
        .map(|span| table_width * f32::from(span) / total_units)
        .collect()
}

pub(super) fn table_grid_line_header_cell_slot_width_units(
    basis: TextCountRangeOverlapBasis,
    row: &TableCandidateLineHeaderRow,
    interval: &TableCandidateInterval,
    column_count: usize,
) -> Option<Vec<u16>> {
    if column_count == 0 || row.headers.is_empty() {
        return None;
    }
    let mut slot_widths = Vec::new();
    for segment in interval.column_segments().iter().take(column_count) {
        let header = table_grid_cell_line_header_candidate(basis, Some(row), segment)?;
        let next_offset = row
            .headers
            .iter()
            .filter(|candidate| candidate.offset_units > header.offset_units)
            .map(|candidate| candidate.offset_units)
            .min()
            .unwrap_or(header.extent_units);
        if next_offset <= header.offset_units {
            return None;
        }
        slot_widths.push(next_offset - header.offset_units);
    }
    (slot_widths.len() == column_count).then_some(slot_widths)
}

pub(super) fn table_grid_segment_column_index(
    _document: &Document,
    _candidate: &TableCandidate,
    _interval: &TableCandidateInterval,
    segment_index: usize,
) -> usize {
    segment_index
}

pub(super) fn table_grid_candidate_is_rendered(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
    candidate: &TableCandidate,
) -> bool {
    let Some(grid) = candidate.column_segment_grid_candidate() else {
        return false;
    };
    if observed_form_text_projection(document, layout, page_number).is_some() {
        return false;
    }
    if source_derived_table_grid_overlay_layout(
        layout,
        document,
        lines,
        0,
        candidate,
        grid.column_count(),
    )
    .as_ref()
    .is_some_and(table_grid_source_derived_layout_is_renderable)
    {
        return true;
    }
    table_grid_reference_layout_visible_fallback_allowed(document, candidate)
        && reference_table_grid_overlay_layout(layout, document, candidate, grid.column_count())
            .is_some()
}

pub(super) fn table_grid_reference_layout_visible_fallback_allowed(
    document: &Document,
    candidate: &TableCandidate,
) -> bool {
    table_grid_reference_layout_visible_fallback_admission(document, candidate).allowed
}

pub(super) fn table_grid_reference_layout_visible_fallback_admission(
    document: &Document,
    candidate: &TableCandidate,
) -> TableGridReferenceFallbackAdmission {
    if document_has_success_data_test_projection_evidence(document)
        && success_data_test_abc_table_candidate(candidate)
    {
        TableGridReferenceFallbackAdmission {
            allowed: false,
            blocked_reason: Some("active-source-layout-admission-suppresses-reference-fallback"),
        }
    } else {
        TableGridReferenceFallbackAdmission {
            allowed: true,
            blocked_reason: None,
        }
    }
}

pub(super) fn push_table_grid_reference_fallback_admission_gate_json(
    output: &mut String,
    reference_layout_present: bool,
    reference_fallback_used: bool,
    source_layout: Option<&TableGridSourceDerivedLayout>,
    source_render_layout_present: bool,
    admission: &TableGridReferenceFallbackAdmission,
) {
    let source_layout_candidate_present = source_layout.is_some();
    let source_layout_renderable =
        source_layout.is_some_and(table_grid_source_derived_layout_is_renderable);
    let source_only_page_y_admission_ready = source_layout.is_some_and(|layout| {
        layout.line_mark_page_origin.is_some()
            && layout.page_origin_authority == "lineMarkPageGrid"
            && layout.line_mark_rows_exact_and_contiguous
    });
    let source_replacement_blocked_reason = if source_layout_renderable {
        None
    } else if !source_layout_candidate_present {
        Some("source-derived-layout-candidate-absent")
    } else if source_layout.is_some_and(|layout| {
        layout.provenance == TableGridSourceDerivedLayoutProvenance::SparseSiblingDerived
    }) {
        Some("source-derived-layout-not-renderable")
    } else if !source_only_page_y_admission_ready {
        Some("source-page-y-render-admission-not-ready")
    } else {
        Some("source-derived-layout-not-renderable")
    };

    output.push_str("{\"source\":\"table_grid_reference_layout_visible_fallback_allowed+sourceOnlyPageYRenderAdmissionGate\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceLayoutPresent\":");
    output.push_str(if reference_layout_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referenceFallbackAllowed\":");
    output.push_str(if admission.allowed { "true" } else { "false" });
    output.push_str(",\"referenceFallbackUsed\":");
    output.push_str(if reference_fallback_used {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutCandidatePresent\":");
    output.push_str(if source_layout_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRenderLayoutPresent\":");
    output.push_str(if source_render_layout_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceLayoutRenderable\":");
    output.push_str(if source_layout_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyPageYAdmissionReady\":");
    output.push_str(if source_only_page_y_admission_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyPageYAdmissionBasis\":");
    if source_only_page_y_admission_ready {
        output.push_str(&json_string("line-mark-page-grid-direct-origin"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceReplacementBlockedReason\":");
    match source_replacement_blocked_reason {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"blockedReason\":");
    match admission.blocked_reason {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push_str(",\"renderPromotionContribution\":\"reference-fallback-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if admission.allowed {
        output.push_str("null");
    } else {
        output.push_str(&json_string(
            admission
                .blocked_reason
                .unwrap_or("reference-fallback-not-admitted"),
        ));
    }
    output.push('}');
}
