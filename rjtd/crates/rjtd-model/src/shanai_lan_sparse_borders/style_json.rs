use super::{types::*, *};

pub(super) fn push_shanai_lan_source_page_transform_candidate_json(
    output: &mut String,
    candidate: Option<&ShanaiLanSourcePageTransformCandidate>,
    stable_grid_extent_units: u16,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };
    let usable_width_mm100 = candidate
        .page_width_mm100
        .saturating_sub(candidate.x_origin_left_mm100)
        .saturating_sub(candidate.x_origin_right_mm100);
    let x_step_mm100 = if stable_grid_extent_units == 0 {
        None
    } else {
        Some(usable_width_mm100 as f32 / f32::from(stable_grid_extent_units))
    };
    output.push_str("{\"source\":\"/DocumentViewStyles+/PageMark\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":true,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"guardPolicy\":\"old-format-page-size-fields-valid+x-origin-mirror+page-mark-pitch-plausible\"");
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&candidate.page_mark_entry_index.to_string());
    output.push_str(",\"pageWidthMm100\":");
    output.push_str(&candidate.page_width_mm100.to_string());
    output.push_str(",\"pageHeightMm100\":");
    output.push_str(&candidate.page_height_mm100.to_string());
    output.push_str(",\"xOriginLeftMm100\":");
    output.push_str(&candidate.x_origin_left_mm100.to_string());
    output.push_str(",\"xOriginRightMm100\":");
    output.push_str(&candidate.x_origin_right_mm100.to_string());
    output.push_str(",\"yOriginMm100\":");
    output.push_str(&candidate.y_origin_mm100.to_string());
    output.push_str(",\"rowPitchAddendAMm100\":");
    output.push_str(&candidate.row_pitch_addend_a_mm100.to_string());
    output.push_str(",\"rowPitchAddendBMm100\":");
    output.push_str(&candidate.row_pitch_addend_b_mm100.to_string());
    output.push_str(",\"rowPitchMm100\":");
    output.push_str(&candidate.row_pitch_mm100.to_string());
    output.push_str(",\"pageMarkW21Mm100\":");
    push_option_u16_json(output, candidate.page_mark_w21_mm100);
    output.push_str(",\"pageMarkW21MatchesPitch\":");
    output.push_str(json_bool(
        candidate
            .page_mark_w21_mm100
            .is_some_and(|value| u32::from(value) == candidate.row_pitch_mm100),
    ));
    output.push_str(",\"xFormula\":{\"originMm100\":");
    output.push_str(&candidate.x_origin_left_mm100.to_string());
    output.push_str(",\"usableWidthMm100\":");
    output.push_str(&usable_width_mm100.to_string());
    output.push_str(",\"stableGridExtentUnits\":");
    output.push_str(&stable_grid_extent_units.to_string());
    output.push_str(",\"pairUnitStepMm100\":");
    push_optional_f32_json(output, x_step_mm100);
    output.push_str(",\"formula\":");
    output.push_str(&json_string(
        "x_mm100 = x_origin_left_mm100 + (pair_unit + 1) * usable_width_mm100 / stable_grid_extent_units",
    ));
    output.push_str("},\"yFormula\":{\"rowAnchorMm100\":");
    output.push_str(&candidate.y_origin_mm100.to_string());
    output.push_str(",\"pitchMm100\":");
    output.push_str(&candidate.row_pitch_mm100.to_string());
    output.push_str(",\"topOffsetMm100\":");
    output.push_str(&candidate.row_pitch_mm100.to_string());
    output.push_str(",\"bottomOffsetMm100Numerator\":");
    output.push_str(&candidate.row_pitch_mm100.to_string());
    output.push_str(",\"bottomOffsetMm100Denominator\":");
    output.push_str(&SOURCE_PAGE_TRANSFORM_BOTTOM_OFFSET_DENOMINATOR.to_string());
    output.push_str(",\"topYFormula\":");
    output.push_str(&json_string(
        "top_y_mm100(g) = row_anchor_mm100 + g*pitch_mm100 - pitch_mm100",
    ));
    output.push_str(",\"bottomYFormula\":");
    output.push_str(&json_string(
        "bottom_y_mm100(g) = row_anchor_mm100 + g*pitch_mm100 - pitch_mm100/2",
    ));
    output.push_str("},\"bitMapping\":{\"0x1\":\"upper-vertical-connector-bottom_y(g-1)-to-top_y(g)\",\"0x2\":\"lower-vertical-connector-top_y(g)-to-bottom_y(g)\",\"0x4\":\"horizontal-right-top-edge-at-top_y(g)\",\"0x8\":\"horizontal-right-bottom-edge-at-bottom_y(g)\"}}");
}

pub(super) fn push_shanai_lan_sparse_table_border_style_coverage_json(
    output: &mut String,
    coverage: &ShanaiLanSparseTableBorderStyleCoverage,
) {
    output.push_str("{\"sectionPresent\":");
    output.push_str(json_bool(coverage.section_present));
    output.push_str(",\"contentUnitCount\":");
    output.push_str(&coverage.content_unit_count.to_string());
    output.push_str(",\"styleStart\":");
    output.push_str(&coverage.style_start.to_string());
    output.push_str(",\"eventCount\":");
    output.push_str(&coverage.event_count.to_string());
    output.push_str(",\"truncated\":");
    output.push_str(json_bool(coverage.truncated));
    output.push_str(",\"relevantSourceUnitCount\":");
    output.push_str(&coverage.relevant_source_units.len().to_string());
    output.push_str(",\"coveredSourceUnitCount\":");
    output.push_str(&coverage.covered_source_units.len().to_string());
    output.push_str(",\"uncoveredSourceUnitCount\":");
    output.push_str(&coverage.uncovered_source_units.len().to_string());
    output.push_str(",\"relevantSourceUnits\":");
    push_usize_array_json(output, &coverage.relevant_source_units);
    output.push_str(",\"coveredSourceUnits\":");
    push_usize_array_json(output, &coverage.covered_source_units);
    output.push_str(",\"uncoveredSourceUnits\":");
    push_usize_array_json(output, &coverage.uncovered_source_units);
    output.push_str(",\"horizontalRenderableCount\":");
    output.push_str(&coverage.horizontal_renderable_count.to_string());
    output.push_str(",\"verticalRenderableHalfCount\":");
    output.push_str(&coverage.vertical_renderable_half_count.to_string());
    output.push_str(",\"admittedRenderSegmentCount\":");
    output.push_str(&coverage.admitted_render_segment_count().to_string());
    output.push_str(",\"diagnostics\":[");
    for (index, diagnostic) in coverage.diagnostics.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"offset\":");
        output.push_str(&diagnostic.offset().to_string());
        output.push_str(",\"kind\":");
        output.push_str(&json_string(document_text_style_diagnostic_kind_name(
            diagnostic.kind(),
        )));
        output.push('}');
    }
    output.push_str("]}");
}

fn document_text_style_diagnostic_kind_name(kind: DocumentTextStyleDiagnosticKind) -> &'static str {
    match kind {
        DocumentTextStyleDiagnosticKind::HeaderTooShort => "header-too-short",
        DocumentTextStyleDiagnosticKind::StyleStartOverflow => "style-start-overflow",
        DocumentTextStyleDiagnosticKind::StyleStartPastEnd => "style-start-past-end",
        DocumentTextStyleDiagnosticKind::ZeroLengthRun => "zero-length-run",
        DocumentTextStyleDiagnosticKind::UnexpectedMarker => "unexpected-marker",
        DocumentTextStyleDiagnosticKind::TruncatedRun => "truncated-run",
        DocumentTextStyleDiagnosticKind::TruncatedProperty => "truncated-property",
        DocumentTextStyleDiagnosticKind::TruncatedPropertyValue => "truncated-property-value",
        DocumentTextStyleDiagnosticKind::TruncatedPropertyTerminator => {
            "truncated-property-terminator"
        }
        DocumentTextStyleDiagnosticKind::CursorOverflow => "cursor-overflow",
        DocumentTextStyleDiagnosticKind::CursorPastContentEnd => "cursor-past-content-end",
    }
}

pub(super) fn push_optional_u16_hex_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => output.push_str(&json_string(&format!("0x{value:04x}"))),
        None => output.push_str("null"),
    }
}
