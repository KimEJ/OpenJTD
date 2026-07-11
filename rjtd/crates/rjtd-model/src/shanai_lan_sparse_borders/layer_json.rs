use super::{candidate_json::*, geometry_json::*, row_json::*, style_json::*, types::*, *};

pub(crate) fn push_page_layer_shanai_lan_sparse_table_border_topology_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: &ShanaiLanSparseTableBorderTopologyDiagnostic,
) {
    output.push_str("{\"type\":\"documentTextSparseTableBorderTopologyDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"/DocumentText+/LineMark\",\"sourceStream\":\"/DocumentText\"");
    output.push_str(",\"projectionKind\":\"documentTextSparseTableBorderTopologyDiagnostic\"");
    output.push_str(",\"diagnosticOnly\":");
    output.push_str(json_bool(!diagnostic.renderable));
    output.push_str(",\"sourceBacked\":");
    output.push_str(json_bool(diagnostic.renderable));
    output.push_str(",\"referenceBacked\":false,\"decoded\":");
    output.push_str(json_bool(diagnostic.renderable));
    output.push_str(",\"geometryDecoded\":");
    output.push_str(json_bool(diagnostic.renderable));
    output.push_str(",\"placementDerived\":");
    output.push_str(json_bool(diagnostic.renderable));
    output.push_str(",\"renderable\":");
    output.push_str(json_bool(diagnostic.renderable));
    output.push_str(",\"pageOriginAuthority\":");
    output.push_str(&json_string(if diagnostic.renderable {
        "source-backed"
    } else {
        "blocked"
    }));
    output.push_str(",\"renderPromotionBlockedReason\":");
    if let Some(reason) = diagnostic.blockers.first() {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"stableGridExtentUnits\":");
    output.push_str(&diagnostic.stable_grid_extent_units.to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&diagnostic.rows.len().to_string());
    output.push_str(",\"horizontalCandidateCount\":");
    output.push_str(&diagnostic.horizontal_candidates.len().to_string());
    output.push_str(",\"junctionCandidateCount\":");
    output.push_str(&diagnostic.junction_candidates.len().to_string());
    output.push_str(",\"verticalCandidateCount\":");
    output.push_str(&diagnostic.vertical_candidates.len().to_string());
    output.push_str(",\"cellGapMidpointCount\":");
    output.push_str(&diagnostic.cell_gap_midpoints.len().to_string());
    output.push_str(",\"styleSectionCoverage\":");
    push_shanai_lan_sparse_table_border_style_coverage_json(output, &diagnostic.style_coverage);
    output.push_str(",\"styleSideMapping\":{");
    output.push_str("\"property1\":\"upper-vertical-half\",");
    output.push_str("\"property2\":\"lower-vertical-half\",");
    output.push_str("\"property3\":\"top-horizontal-edge\",");
    output.push_str("\"property8\":\"bottom-horizontal-edge\"}");
    output.push_str(",\"blockers\":");
    push_json_string_slice_array(output, &diagnostic.blockers);
    output.push_str(",\"sourcePageTransformCandidate\":");
    push_shanai_lan_source_page_transform_candidate_json(
        output,
        diagnostic.source_page_transform_candidate.as_ref(),
        diagnostic.stable_grid_extent_units,
    );
    output.push_str(",\"rows\":");
    push_rows_json(output, &diagnostic.rows);
    output.push_str(",\"horizontalCandidates\":");
    push_horizontal_candidates_json(output, &diagnostic.horizontal_candidates);
    output.push_str(",\"junctionCandidates\":");
    push_junction_candidates_json(output, &diagnostic.junction_candidates);
    output.push_str(",\"verticalCandidates\":");
    push_vertical_candidates_json(output, &diagnostic.vertical_candidates);
    output.push_str(",\"cellGapMidpoints\":");
    push_cell_gap_midpoints_json(output, &diagnostic.cell_gap_midpoints);
    output.push('}');
}
