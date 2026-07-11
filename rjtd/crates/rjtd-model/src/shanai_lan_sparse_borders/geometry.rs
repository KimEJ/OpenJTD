use super::{types::*, *};

pub(super) fn shanai_lan_sparse_table_border_cell_gap_midpoints(
    bytes: &[u8],
    group_offsets: &[usize],
    rows: &[ShanaiLanSparseTableBorderRow],
) -> Vec<ShanaiLanSparseTableBorderCellGapMidpoint> {
    let admitted_groups = rows
        .iter()
        .map(|row| row.group_index)
        .collect::<BTreeSet<_>>();
    let mut by_group = BTreeMap::<usize, Vec<ShanaiLanLineHeader>>::new();
    for line_header in shanai_lan_line_headers_in_groups(bytes, group_offsets) {
        if admitted_groups.contains(&line_header.group_index) {
            by_group
                .entry(line_header.group_index)
                .or_default()
                .push(line_header.header);
        }
    }

    let mut midpoints = Vec::new();
    for (group_index, mut headers) in by_group {
        headers.sort_by_key(|header| (header.offset_units, header.extent_units, header.start));
        for pair in headers.windows(2) {
            let [left, right] = pair else {
                continue;
            };
            if left.extent_units >= right.offset_units {
                continue;
            }
            let midpoint_sum = u32::from(left.extent_units) + u32::from(right.offset_units);
            if midpoint_sum % 2 != 0 {
                continue;
            }
            midpoints.push(ShanaiLanSparseTableBorderCellGapMidpoint {
                group_index,
                midpoint_unit: midpoint_sum / 2,
                left_extent_unit: left.extent_units,
                right_offset_unit: right.offset_units,
                left_source_span: TextSourceSpan::new(
                    left.start,
                    left.end,
                    left.start / 2,
                    left.end / 2,
                ),
                right_source_span: TextSourceSpan::new(
                    right.start,
                    right.end,
                    right.start / 2,
                    right.end / 2,
                ),
            });
        }
    }
    midpoints
}

pub(super) fn shanai_lan_sparse_table_border_vertical_candidates(
    junctions: &[ShanaiLanSparseTableBorderJunctionCandidate],
    cell_gap_midpoints: &[ShanaiLanSparseTableBorderCellGapMidpoint],
) -> Vec<ShanaiLanSparseTableBorderVerticalCandidate> {
    let mut grouped = BTreeMap::<u32, Vec<ShanaiLanSparseTableBorderJunctionCandidate>>::new();
    for junction in junctions {
        grouped
            .entry(junction.x_unit)
            .or_default()
            .push(junction.clone());
    }

    let mut verticals = Vec::new();
    for (x_unit, mut group) in grouped {
        group.sort_by_key(|junction| {
            (
                junction.group_index,
                junction.row_index,
                junction.pair_index,
            )
        });
        let mut run_start = 0usize;
        while run_start + 1 < group.len() {
            let mut run_end = run_start;
            while run_end + 1 < group.len()
                && group[run_end].lower_vertical_candidate
                && group[run_end + 1].upper_vertical_candidate
                && group[run_end + 1].group_index == group[run_end].group_index + 1
            {
                run_end += 1;
            }
            if run_end > run_start {
                let contributing = &group[run_start..=run_end];
                let matching_gap_midpoint_units = cell_gap_midpoints
                    .iter()
                    .filter(|midpoint| midpoint.midpoint_unit == x_unit)
                    .map(|midpoint| midpoint.midpoint_unit)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();
                verticals.push(ShanaiLanSparseTableBorderVerticalCandidate {
                    x_unit,
                    start_group_index: contributing
                        .first()
                        .map(|junction| junction.group_index)
                        .unwrap_or_default(),
                    end_group_index: contributing
                        .last()
                        .map(|junction| junction.group_index)
                        .unwrap_or_default(),
                    contributing_row_indexes: contributing
                        .iter()
                        .map(|junction| junction.row_index)
                        .collect(),
                    contributing_group_indexes: contributing
                        .iter()
                        .map(|junction| junction.group_index)
                        .collect(),
                    contributing_pair_indexes: contributing
                        .iter()
                        .map(|junction| junction.pair_index)
                        .collect(),
                    contributing_source_spans: contributing
                        .iter()
                        .map(|junction| junction.source_span.clone())
                        .collect(),
                    matching_gap_midpoint_units,
                });
            }
            run_start = run_end.saturating_add(1);
        }
    }
    verticals
}
