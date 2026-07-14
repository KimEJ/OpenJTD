use super::{style_json::*, types::*, *};

pub(super) fn push_horizontal_candidates_json(
    output: &mut String,
    candidates: &[ShanaiLanSparseTableBorderHorizontalCandidate],
) {
    output.push('[');
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&candidate.row_index.to_string());
        output.push_str(",\"groupIndex\":");
        output.push_str(&candidate.group_index.to_string());
        output.push_str(",\"pairIndex\":");
        output.push_str(&candidate.pair_index.to_string());
        output.push_str(",\"edgeKind\":");
        output.push_str(&json_string(candidate.edge_kind.as_str()));
        output.push_str(",\"stateCode\":");
        output.push_str(&candidate.state_code.to_string());
        output.push_str(",\"stateCodeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", candidate.state_code)));
        output.push_str(",\"edgeStyleCode\":");
        push_option_u16_json(output, candidate.edge_style_code);
        output.push_str(",\"edgeStyleCodeHex\":");
        push_optional_u16_hex_json(output, candidate.edge_style_code);
        output.push_str(",\"startUnit\":");
        output.push_str(&candidate.start_unit.to_string());
        output.push_str(",\"endUnit\":");
        output.push_str(&candidate.end_unit.to_string());
        output.push_str(",\"sourceByteRange\":");
        output.push_str(&source_range_json(
            candidate.source_span.byte_start(),
            candidate.source_span.byte_end(),
        ));
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(
            candidate.source_span.unit_start(),
            candidate.source_span.unit_end(),
        ));
        output.push('}');
    }

    output.push(']');
}

pub(super) fn push_junction_candidates_json(
    output: &mut String,
    candidates: &[ShanaiLanSparseTableBorderJunctionCandidate],
) {
    output.push('[');
    for (index, junction) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&junction.row_index.to_string());
        output.push_str(",\"groupIndex\":");
        output.push_str(&junction.group_index.to_string());
        output.push_str(",\"pairIndex\":");
        output.push_str(&junction.pair_index.to_string());
        output.push_str(",\"xUnit\":");
        output.push_str(&junction.x_unit.to_string());
        output.push_str(",\"stateCode\":");
        output.push_str(&junction.state_code.to_string());
        output.push_str(",\"stateCodeHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", junction.state_code)));
        output.push_str(",\"upperVerticalCandidate\":");
        output.push_str(json_bool(junction.upper_vertical_candidate));
        output.push_str(",\"lowerVerticalCandidate\":");
        output.push_str(json_bool(junction.lower_vertical_candidate));
        output.push_str(",\"topHorizontalCandidate\":");
        output.push_str(json_bool(junction.top_horizontal_candidate));
        output.push_str(",\"bottomHorizontalCandidate\":");
        output.push_str(json_bool(junction.bottom_horizontal_candidate));
        output.push_str(",\"upperVerticalStyleCode\":");
        push_option_u16_json(output, junction.upper_vertical_style_code);
        output.push_str(",\"upperVerticalStyleCodeHex\":");
        push_optional_u16_hex_json(output, junction.upper_vertical_style_code);
        output.push_str(",\"lowerVerticalStyleCode\":");
        push_option_u16_json(output, junction.lower_vertical_style_code);
        output.push_str(",\"lowerVerticalStyleCodeHex\":");
        push_optional_u16_hex_json(output, junction.lower_vertical_style_code);
        output.push_str(",\"topHorizontalStyleCode\":");
        push_option_u16_json(output, junction.top_horizontal_style_code);
        output.push_str(",\"topHorizontalStyleCodeHex\":");
        push_optional_u16_hex_json(output, junction.top_horizontal_style_code);
        output.push_str(",\"bottomHorizontalStyleCode\":");
        push_option_u16_json(output, junction.bottom_horizontal_style_code);
        output.push_str(",\"bottomHorizontalStyleCodeHex\":");
        push_optional_u16_hex_json(output, junction.bottom_horizontal_style_code);
        output.push_str(",\"sourceByteRange\":");
        output.push_str(&source_range_json(
            junction.source_span.byte_start(),
            junction.source_span.byte_end(),
        ));
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(
            junction.source_span.unit_start(),
            junction.source_span.unit_end(),
        ));
        output.push('}');
    }

    output.push(']');
}
