use super::{types::*, *};

pub(super) fn shanai_lan_sparse_table_border_row(
    row_index: usize,
    group_index: usize,
    record: &rjtd_core::document_text::DocumentTextRowHeaderRecord,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    style_resolver: &ShanaiLanSparseTableBorderStyleResolver,
) -> ShanaiLanSparseTableBorderRow {
    let line_mark_record_index = line_mark_intervals
        .iter()
        .find(|interval| interval.record_index == group_index + 1)
        .map(|interval| interval.record_index);
    let pairs = record
        .pairs()
        .iter()
        .enumerate()
        .map(|(pair_index, pair)| {
            let style_state = style_resolver.state_at_unit(pair.source_span().unit_start());
            ShanaiLanSparseTableBorderPair {
                pair_index,
                source_span: text_source_span_from_document_text_units(
                    pair.source_span().unit_start(),
                    pair.source_span().unit_end(),
                ),
                state_code: pair.state_code(),
                run_length: pair.run_length(),
                start_unit: pair.start_unit(),
                end_unit: pair.end_unit(),
                blank_run: pair.classification()
                    == DocumentTextRowHeaderPairClassification::BlankRun,
                upper_vertical_candidate: pair.state_code() & 0x0001 != 0,
                lower_vertical_candidate: pair.state_code() & 0x0002 != 0,
                top_horizontal_candidate: pair.state_code() & 0x0004 != 0,
                bottom_horizontal_candidate: pair.state_code() & 0x0008 != 0,
                style_source_covered: style_state.is_some(),
                upper_vertical_style_code: style_state.map(|state| state.upper_vertical_style_code),
                lower_vertical_style_code: style_state.map(|state| state.lower_vertical_style_code),
                top_horizontal_style_code: style_state.map(|state| state.top_horizontal_style_code),
                bottom_horizontal_style_code: style_state
                    .map(|state| state.bottom_horizontal_style_code),
            }
        })
        .collect::<Vec<_>>();

    ShanaiLanSparseTableBorderRow {
        row_index,
        group_index,
        source_span: text_source_span_from_document_text_units(
            record.source_span().unit_start(),
            record.source_span().unit_end(),
        ),
        grid_extent_units: record.fixed_fields().grid_extent(),
        w8_units: record.fixed_fields().w8(),
        line_mark_record_index,
        line_mark_record_index_delta: line_mark_record_index
            .map(|record_index| record_index as i32 - group_index as i32),
        pairs,
    }
}

pub(super) fn shanai_lan_sparse_table_border_horizontal_candidates(
    rows: &[ShanaiLanSparseTableBorderRow],
) -> Vec<ShanaiLanSparseTableBorderHorizontalCandidate> {
    let mut candidates = Vec::new();
    for row in rows {
        for pair in &row.pairs {
            if pair.run_length == 0 || pair.blank_run {
                continue;
            }
            if pair.top_horizontal_candidate {
                candidates.push(ShanaiLanSparseTableBorderHorizontalCandidate {
                    row_index: row.row_index,
                    group_index: row.group_index,
                    pair_index: pair.pair_index,
                    state_code: pair.state_code,
                    start_unit: pair.start_unit,
                    end_unit: pair.end_unit,
                    source_span: pair.source_span.clone(),
                    edge_kind: ShanaiLanSparseTableBorderHorizontalEdgeKind::Top,
                    edge_style_code: pair.top_horizontal_style_code,
                });
            }
            if pair.bottom_horizontal_candidate {
                candidates.push(ShanaiLanSparseTableBorderHorizontalCandidate {
                    row_index: row.row_index,
                    group_index: row.group_index,
                    pair_index: pair.pair_index,
                    state_code: pair.state_code,
                    start_unit: pair.start_unit,
                    end_unit: pair.end_unit,
                    source_span: pair.source_span.clone(),
                    edge_kind: ShanaiLanSparseTableBorderHorizontalEdgeKind::Bottom,
                    edge_style_code: pair.bottom_horizontal_style_code,
                });
            }
        }
    }
    candidates
}

pub(super) fn shanai_lan_sparse_table_border_junction_candidates(
    rows: &[ShanaiLanSparseTableBorderRow],
) -> Vec<ShanaiLanSparseTableBorderJunctionCandidate> {
    let mut candidates = Vec::new();
    for row in rows {
        for pair in &row.pairs {
            if pair.run_length != 0
                || (!pair.upper_vertical_candidate
                    && !pair.lower_vertical_candidate
                    && !pair.top_horizontal_candidate
                    && !pair.bottom_horizontal_candidate)
            {
                continue;
            }
            candidates.push(ShanaiLanSparseTableBorderJunctionCandidate {
                row_index: row.row_index,
                group_index: row.group_index,
                pair_index: pair.pair_index,
                state_code: pair.state_code,
                x_unit: pair.start_unit,
                source_span: pair.source_span.clone(),
                upper_vertical_candidate: pair.upper_vertical_candidate,
                lower_vertical_candidate: pair.lower_vertical_candidate,
                top_horizontal_candidate: pair.top_horizontal_candidate,
                bottom_horizontal_candidate: pair.bottom_horizontal_candidate,
                upper_vertical_style_code: pair.upper_vertical_style_code,
                lower_vertical_style_code: pair.lower_vertical_style_code,
                top_horizontal_style_code: pair.top_horizontal_style_code,
                bottom_horizontal_style_code: pair.bottom_horizontal_style_code,
            });
        }
    }
    candidates
}

pub(super) fn shanai_lan_sparse_table_border_style_coverage(
    style_resolver: &ShanaiLanSparseTableBorderStyleResolver,
    rows: &[ShanaiLanSparseTableBorderRow],
    horizontal_candidates: &[ShanaiLanSparseTableBorderHorizontalCandidate],
    junction_candidates: &[ShanaiLanSparseTableBorderJunctionCandidate],
) -> ShanaiLanSparseTableBorderStyleCoverage {
    let mut relevant_source_units = rows
        .iter()
        .flat_map(|row| row.pairs.iter())
        .filter(|pair| {
            pair.upper_vertical_candidate
                || pair.lower_vertical_candidate
                || pair.top_horizontal_candidate
                || pair.bottom_horizontal_candidate
        })
        .map(|pair| pair.source_span.unit_start())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    relevant_source_units.sort_unstable();

    let (covered_source_units, uncovered_source_units): (Vec<_>, Vec<_>) = relevant_source_units
        .iter()
        .copied()
        .partition(|unit| style_resolver.state_at_unit(*unit).is_some());

    let horizontal_renderable_count = horizontal_candidates
        .iter()
        .filter(|candidate| {
            shanai_lan_sparse_table_border_style_code_admitted(candidate.edge_style_code)
        })
        .count();
    let vertical_renderable_half_count = junction_candidates
        .iter()
        .map(|candidate| {
            usize::from(
                candidate.upper_vertical_candidate
                    && shanai_lan_sparse_table_border_style_code_admitted(
                        candidate.upper_vertical_style_code,
                    ),
            ) + usize::from(
                candidate.lower_vertical_candidate
                    && shanai_lan_sparse_table_border_style_code_admitted(
                        candidate.lower_vertical_style_code,
                    ),
            )
        })
        .sum();

    ShanaiLanSparseTableBorderStyleCoverage {
        section_present: style_resolver.section_present,
        content_unit_count: style_resolver.content_unit_count,
        style_start: style_resolver.style_start,
        event_count: style_resolver.event_count,
        truncated: style_resolver.truncated,
        diagnostics: style_resolver.diagnostics.clone(),
        relevant_source_units,
        covered_source_units,
        uncovered_source_units,
        horizontal_renderable_count,
        vertical_renderable_half_count,
    }
}

pub(crate) fn shanai_lan_sparse_table_border_style_code_admitted(style_code: Option<u16>) -> bool {
    matches!(style_code, Some(3 | 4 | 6))
}

pub(crate) fn shanai_lan_sparse_table_border_stroke(style_code: u16) -> Option<(f32, Option<f32>)> {
    match style_code {
        3 => Some((SPARSE_TABLE_BORDER_THICK_STROKE_WIDTH_PX, None)),
        4 => Some((
            SPARSE_TABLE_BORDER_THIN_STROKE_WIDTH_PX,
            Some(SPARSE_TABLE_BORDER_DASH_LENGTH_PX),
        )),
        6 => Some((
            SPARSE_TABLE_BORDER_THICK_STROKE_WIDTH_PX,
            Some(SPARSE_TABLE_BORDER_DASH_LENGTH_PX),
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_border_style_admission_is_limited_to_observed_codes() {
        assert_eq!(
            shanai_lan_sparse_table_border_stroke(3),
            Some((SPARSE_TABLE_BORDER_THICK_STROKE_WIDTH_PX, None))
        );
        assert_eq!(
            shanai_lan_sparse_table_border_stroke(4),
            Some((
                SPARSE_TABLE_BORDER_THIN_STROKE_WIDTH_PX,
                Some(SPARSE_TABLE_BORDER_DASH_LENGTH_PX)
            ))
        );
        assert_eq!(
            shanai_lan_sparse_table_border_stroke(6),
            Some((
                SPARSE_TABLE_BORDER_THICK_STROKE_WIDTH_PX,
                Some(SPARSE_TABLE_BORDER_DASH_LENGTH_PX)
            ))
        );
        for code in [0, 1, 2, 5, 7, u16::MAX] {
            assert!(!shanai_lan_sparse_table_border_style_code_admitted(Some(
                code
            )));
            assert_eq!(shanai_lan_sparse_table_border_stroke(code), None);
        }
    }
}
