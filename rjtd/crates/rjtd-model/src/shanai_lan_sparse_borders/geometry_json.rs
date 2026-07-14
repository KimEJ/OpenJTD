use super::{types::*, *};

pub(super) fn push_vertical_candidates_json(
    output: &mut String,
    candidates: &[ShanaiLanSparseTableBorderVerticalCandidate],
) {
    output.push('[');
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"xUnit\":");
        output.push_str(&candidate.x_unit.to_string());
        output.push_str(",\"startGroupIndex\":");
        output.push_str(&candidate.start_group_index.to_string());
        output.push_str(",\"endGroupIndex\":");
        output.push_str(&candidate.end_group_index.to_string());
        output.push_str(",\"contributingRowIndexes\":");
        push_usize_array_json(output, &candidate.contributing_row_indexes);
        output.push_str(",\"contributingGroupIndexes\":");
        push_usize_array_json(output, &candidate.contributing_group_indexes);
        output.push_str(",\"contributingPairIndexes\":");
        push_usize_array_json(output, &candidate.contributing_pair_indexes);
        output.push_str(",\"matchingGapMidpointUnits\":");
        push_u32_array_json(output, &candidate.matching_gap_midpoint_units);
        output.push_str(",\"contributingSourceUnitRanges\":[");
        for (span_index, span) in candidate.contributing_source_spans.iter().enumerate() {
            if span_index > 0 {
                output.push(',');
            }
            output.push_str(&source_range_json(span.unit_start(), span.unit_end()));
        }
        output.push_str("]}");
    }

    output.push(']');
}

pub(super) fn push_cell_gap_midpoints_json(
    output: &mut String,
    midpoints: &[ShanaiLanSparseTableBorderCellGapMidpoint],
) {
    output.push('[');
    for (index, midpoint) in midpoints.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"groupIndex\":");
        output.push_str(&midpoint.group_index.to_string());
        output.push_str(",\"midpointUnit\":");
        output.push_str(&midpoint.midpoint_unit.to_string());
        output.push_str(",\"leftExtentUnit\":");
        output.push_str(&midpoint.left_extent_unit.to_string());
        output.push_str(",\"rightOffsetUnit\":");
        output.push_str(&midpoint.right_offset_unit.to_string());
        output.push_str(",\"leftSourceUnitRange\":");
        output.push_str(&source_range_json(
            midpoint.left_source_span.unit_start(),
            midpoint.left_source_span.unit_end(),
        ));
        output.push_str(",\"rightSourceUnitRange\":");
        output.push_str(&source_range_json(
            midpoint.right_source_span.unit_start(),
            midpoint.right_source_span.unit_end(),
        ));
        output.push('}');
    }

    output.push(']');
}
