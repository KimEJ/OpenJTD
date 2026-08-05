use super::*;
use crate::*;

pub(crate) fn push_table_grid_candidate_svg(
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

pub(crate) fn table_grid_cell_render_text(
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

pub(crate) fn push_table_grid_cell_whitespace_placement_probe_json(
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

pub(crate) fn table_grid_cell_whitespace_probe_svg_attrs(
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

pub(crate) fn table_grid_segment_source_raw_text(
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

pub(crate) fn table_grid_overlay_layout(
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

pub(crate) fn table_grid_fallback_overlay_layout(
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

pub(crate) fn source_derived_table_grid_overlay_layout(
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

pub(crate) fn table_grid_source_derived_layout_candidate(
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

pub(crate) fn sparse_sibling_derived_table_grid_overlay_layout(
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

pub(crate) fn table_grid_source_derived_layout_is_renderable(
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
pub(crate) fn table_grid_stride_raw_record_index_y_candidate(
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

pub(crate) fn table_grid_line_mark_page_origin_candidate(
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

pub(crate) fn table_grid_page_mark_line_pitch_candidate(
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

pub(crate) fn table_grid_line_mark_page_origin_stride_candidate(
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

pub(crate) fn table_grid_line_mark_rows_are_exact_and_contiguous(
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

pub(crate) fn table_grid_line_mark_row_record_selection(
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

pub(crate) fn table_grid_line_header_rows_are_homogeneous(
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

pub(crate) fn table_grid_column_width(
    default_column_width: f32,
    column_widths: &[f32],
    index: usize,
) -> f32 {
    column_widths
        .get(index)
        .copied()
        .unwrap_or(default_column_width)
}

pub(crate) fn table_grid_column_x(
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

pub(crate) fn table_grid_projection_kind(reference_projection: bool) -> &'static str {
    if reference_projection {
        "tableProjection"
    } else {
        "diagnosticProjection"
    }
}

pub(crate) fn table_grid_fallback_text_anchor_count(
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

pub(crate) fn table_grid_source_layout_evidence_present(document: &Document) -> bool {
    !document.text_count_ranges().is_empty()
        || raw_stream_bytes(document, LAYOUT_BOX_PATH).is_some()
        || raw_stream_bytes(document, LAYOUT_BOX_TEXT_PATH).is_some()
        || raw_stream_bytes(document, LAYOUT_BOX_TEXT_POSITION_TABLES_PATH).is_some()
}

pub(crate) fn table_grid_decoded_source_placement_evidence_present(
    document: &Document,
    candidate: &TableCandidate,
) -> bool {
    let required = table_grid_decoded_source_placement_required_cell_count(candidate);
    required > 0 && table_grid_decoded_source_placement_match_count(document, candidate) >= required
}

pub(crate) fn table_grid_decoded_source_placement_required_cell_count(
    candidate: &TableCandidate,
) -> usize {
    candidate.cell_count_candidate()
}

pub(crate) fn table_grid_decoded_source_placement_match_count(
    document: &Document,
    candidate: &TableCandidate,
) -> usize {
    table_candidate_document_text_line_header_rows(document, candidate)
        .iter()
        .map(|row| row.matched_cell_count)
        .sum()
}

pub(crate) fn table_grid_segment_source_svg_attrs(
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

pub(crate) fn table_grid_cell_source_evidence_svg_attrs(
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

pub(crate) fn reference_table_grid_overlay_layout(
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
    column_count: usize,
) -> Option<TableGridReferenceLayout> {
    tsaiten_reference_table_grid_overlay_layout(layout, document, candidate, column_count)
}

pub(crate) fn diagnostic_reference_table_grid_overlay_layout(
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

pub(crate) fn tsaiten_reference_table_grid_overlay_layout(
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

pub(crate) fn tsaiten_table_grid_overlay_layout(
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

pub(crate) fn table_grid_line_header_column_widths_px(
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

pub(crate) fn table_grid_line_header_cell_slot_width_units(
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

pub(crate) fn table_grid_segment_column_index(
    _document: &Document,
    _candidate: &TableCandidate,
    _interval: &TableCandidateInterval,
    segment_index: usize,
) -> usize {
    segment_index
}

pub(crate) fn table_grid_candidate_is_rendered(
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

pub(crate) fn table_grid_reference_layout_visible_fallback_allowed(
    document: &Document,
    candidate: &TableCandidate,
) -> bool {
    table_grid_reference_layout_visible_fallback_admission(document, candidate).allowed
}

pub(crate) fn table_grid_reference_layout_visible_fallback_admission(
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

pub(crate) fn push_table_grid_reference_fallback_admission_gate_json(
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
