use super::{candidates::*, geometry::*, transform::*, types::*, *};

pub(crate) fn shanai_lan_sparse_table_border_topology_diagnostic(
    document: &Document,
) -> Option<ShanaiLanSparseTableBorderTopologyDiagnostic> {
    let bytes = document_text_raw_stream(document)?;
    let style_resolver = ShanaiLanSparseTableBorderStyleResolver::from_document_text_bytes(bytes);
    let group_offsets = shanai_lan_text_group_offsets(bytes);
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    let parsed_rows = parse_document_text_row_headers(bytes);
    let mut admitted_rows = Vec::new();
    let mut grid_extent_counts = BTreeMap::<u16, usize>::new();

    for record in parsed_rows {
        if record.fixed_fields().subtype() != 0x008f
            || !record.geometry_complete()
            || record.raw_tail_words() != [0xffff, 0x0000]
            || !shanai_lan_row_header_source_span_valid(bytes, &record)
        {
            continue;
        }
        let Some(group_index) =
            shanai_lan_row_header_group_index(&group_offsets, record.source_span().byte_start())
        else {
            continue;
        };
        *grid_extent_counts
            .entry(record.fixed_fields().grid_extent())
            .or_default() += 1;
        admitted_rows.push((group_index, record));
    }

    let stable_grid_extent_units = grid_extent_counts
        .iter()
        .max_by_key(|(_, count)| **count)
        .map(|(grid_extent, _)| *grid_extent)?;

    let rows = admitted_rows
        .into_iter()
        .filter(|(_, record)| record.fixed_fields().grid_extent() == stable_grid_extent_units)
        .enumerate()
        .map(|(row_index, (group_index, record))| {
            shanai_lan_sparse_table_border_row(
                row_index,
                group_index,
                &record,
                &line_mark_intervals,
                &style_resolver,
            )
        })
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return None;
    }

    let horizontal_candidates = shanai_lan_sparse_table_border_horizontal_candidates(&rows);
    let junction_candidates = shanai_lan_sparse_table_border_junction_candidates(&rows);
    let cell_gap_midpoints =
        shanai_lan_sparse_table_border_cell_gap_midpoints(bytes, &group_offsets, &rows);
    let vertical_candidates = shanai_lan_sparse_table_border_vertical_candidates(
        &junction_candidates,
        &cell_gap_midpoints,
    );
    let line_mark_record_indexes = rows
        .iter()
        .filter_map(|row| row.line_mark_record_index)
        .collect::<Vec<_>>();
    let style_coverage = shanai_lan_sparse_table_border_style_coverage(
        &style_resolver,
        &rows,
        &horizontal_candidates,
        &junction_candidates,
    );
    let source_page_transform_candidate =
        shanai_lan_source_page_transform_candidate(document, &line_mark_record_indexes);
    let mut blockers = Vec::new();
    if !style_coverage.section_present {
        blockers.push("style-section-absent-or-invalid");
    }
    if !style_coverage.relevant_source_units_covered() {
        blockers.push("style-section-does-not-cover-relevant-source-units");
    }
    if style_coverage.truncated {
        blockers.push("style-section-truncated");
    }
    if source_page_transform_candidate.is_none() {
        blockers.push("source-page-transform-candidate-absent");
    }
    if style_coverage.admitted_render_segment_count() == 0 {
        blockers.push("admitted-border-style-code-absent");
    }
    let renderable = blockers.is_empty();

    Some(ShanaiLanSparseTableBorderTopologyDiagnostic {
        stable_grid_extent_units,
        rows,
        horizontal_candidates,
        junction_candidates,
        vertical_candidates,
        cell_gap_midpoints,
        style_coverage,
        source_page_transform_candidate,
        renderable,
        blockers,
    })
}

fn shanai_lan_row_header_group_index(group_offsets: &[usize], byte_start: usize) -> Option<usize> {
    group_offsets
        .iter()
        .position(|offset| *offset == byte_start)
}

fn shanai_lan_row_header_source_span_valid(
    bytes: &[u8],
    record: &rjtd_core::document_text::DocumentTextRowHeaderRecord,
) -> bool {
    let record_span = record.source_span();
    if record_span.byte_end() > bytes.len()
        || record_span.byte_start() >= record_span.byte_end()
        || record_span.unit_end() * 2 != record_span.byte_end()
    {
        return false;
    }
    record.pairs().iter().all(|pair| {
        let span = pair.source_span();
        record_span.byte_start() <= span.byte_start()
            && span.byte_end() <= record_span.byte_end()
            && span.unit_start() < span.unit_end()
            && span.byte_end() == span.unit_end() * 2
    })
}
