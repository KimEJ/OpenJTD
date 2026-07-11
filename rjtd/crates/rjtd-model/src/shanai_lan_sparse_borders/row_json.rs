use super::{style_json::*, types::*, *};

pub(super) fn push_rows_json(output: &mut String, rows: &[ShanaiLanSparseTableBorderRow]) {
    output.push('[');
    for (row_index, row) in rows.iter().enumerate() {
        if row_index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&row.row_index.to_string());
        output.push_str(",\"groupIndex\":");
        output.push_str(&row.group_index.to_string());
        output.push_str(",\"sourceByteRange\":");
        output.push_str(&source_range_json(
            row.source_span.byte_start(),
            row.source_span.byte_end(),
        ));
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(
            row.source_span.unit_start(),
            row.source_span.unit_end(),
        ));
        output.push_str(",\"gridExtentUnits\":");
        output.push_str(&row.grid_extent_units.to_string());
        output.push_str(",\"w8Units\":");
        output.push_str(&row.w8_units.to_string());
        output.push_str(",\"lineMarkRecordIndex\":");
        push_option_usize_json(output, row.line_mark_record_index);
        output.push_str(",\"lineMarkRecordIndexDelta\":");
        push_optional_i32_json(output, row.line_mark_record_index_delta);
        output.push_str(",\"pairs\":[");
        for (pair_index, pair) in row.pairs.iter().enumerate() {
            if pair_index > 0 {
                output.push(',');
            }
            output.push_str("{\"pairIndex\":");
            output.push_str(&pair.pair_index.to_string());
            output.push_str(",\"stateCode\":");
            output.push_str(&pair.state_code.to_string());
            output.push_str(",\"stateCodeHex\":");
            output.push_str(&json_string(&format!("0x{:04x}", pair.state_code)));
            output.push_str(",\"runLength\":");
            output.push_str(&pair.run_length.to_string());
            output.push_str(",\"startUnit\":");
            output.push_str(&pair.start_unit.to_string());
            output.push_str(",\"endUnit\":");
            output.push_str(&pair.end_unit.to_string());
            output.push_str(",\"sourceByteRange\":");
            output.push_str(&source_range_json(
                pair.source_span.byte_start(),
                pair.source_span.byte_end(),
            ));
            output.push_str(",\"sourceUnitRange\":");
            output.push_str(&source_range_json(
                pair.source_span.unit_start(),
                pair.source_span.unit_end(),
            ));
            output.push_str(",\"blankRun\":");
            output.push_str(json_bool(pair.blank_run));
            output.push_str(",\"upperVerticalCandidate\":");
            output.push_str(json_bool(pair.upper_vertical_candidate));
            output.push_str(",\"lowerVerticalCandidate\":");
            output.push_str(json_bool(pair.lower_vertical_candidate));
            output.push_str(",\"topHorizontalCandidate\":");
            output.push_str(json_bool(pair.top_horizontal_candidate));
            output.push_str(",\"bottomHorizontalCandidate\":");
            output.push_str(json_bool(pair.bottom_horizontal_candidate));
            output.push_str(",\"styleSourceCovered\":");
            output.push_str(json_bool(pair.style_source_covered));
            output.push_str(",\"upperVerticalStyleCode\":");
            push_option_u16_json(output, pair.upper_vertical_style_code);
            output.push_str(",\"upperVerticalStyleCodeHex\":");
            push_optional_u16_hex_json(output, pair.upper_vertical_style_code);
            output.push_str(",\"lowerVerticalStyleCode\":");
            push_option_u16_json(output, pair.lower_vertical_style_code);
            output.push_str(",\"lowerVerticalStyleCodeHex\":");
            push_optional_u16_hex_json(output, pair.lower_vertical_style_code);
            output.push_str(",\"topHorizontalStyleCode\":");
            push_option_u16_json(output, pair.top_horizontal_style_code);
            output.push_str(",\"topHorizontalStyleCodeHex\":");
            push_optional_u16_hex_json(output, pair.top_horizontal_style_code);
            output.push_str(",\"bottomHorizontalStyleCode\":");
            push_option_u16_json(output, pair.bottom_horizontal_style_code);
            output.push_str(",\"bottomHorizontalStyleCodeHex\":");
            push_optional_u16_hex_json(output, pair.bottom_horizontal_style_code);
            output.push('}');
        }
        output.push_str("]}");
    }

    output.push(']');
}
