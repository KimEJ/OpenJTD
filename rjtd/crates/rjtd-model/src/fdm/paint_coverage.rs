use super::*;
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FdmPaintCoverage {
    pub(crate) bbox_area_px: f32,
    pub(crate) page_coverage_ratio: f32,
    pub(crate) viewport_coverage_ratio: f32,
    pub(crate) closed_primitive: bool,
    pub(crate) fill_paint_present: bool,
    pub(crate) page_fill_candidate: bool,
    pub(crate) page_fill_candidate_basis: &'static str,
    pub(crate) page_fill_candidate_reason: &'static str,
    pub(crate) page_paint_source_evidence_proven: bool,
    pub(crate) render_promotion_blocked_reason: &'static str,
}

pub(crate) fn fdm_vector_paint_coverage(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
    bbox: (f32, f32, f32, f32),
) -> FdmPaintCoverage {
    let (_, _, width, height) = bbox;
    let page_coverage_ratio = projected_bbox_page_coverage_ratio(layout, width, height);
    let viewport_coverage_ratio = projected_bbox_viewport_coverage_ratio(layout, width, height);
    let closed_primitive = fdm_vector_primitive_is_closed(diagnostic.command);
    let fill_paint_present = if let Some(ellipse) = diagnostic.command.ellipse() {
        ellipse.color().is_some() && fdm_vector_ellipse_should_fill(ellipse)
    } else {
        fdm_vector_linear_gradient_colors(diagnostic.command).is_some()
            || fdm_vector_render_fill_color(diagnostic, diagnostics) != "none"
    };
    let large_span_filter_met = fdm_path_span_filter_blocks(layout, diagnostic.command, bbox);
    let page_fill_candidate = closed_primitive && fill_paint_present && large_span_filter_met;
    let page_fill_candidate_reason = if !closed_primitive {
        "open-primitive-not-page-fill"
    } else if !fill_paint_present {
        "no-fill-paint"
    } else if !large_span_filter_met {
        "large-span-filter-not-met"
    } else {
        "closed-fill-large-span-filter-met"
    };
    let page_paint_source_evidence_proven = false;
    let render_promotion_blocked_reason = if page_fill_candidate {
        "fdm-page-fill-source-evidence-unproven"
    } else {
        "not-page-fill-candidate"
    };
    FdmPaintCoverage {
        bbox_area_px: width.max(0.0) * height.max(0.0),
        page_coverage_ratio,
        viewport_coverage_ratio,
        closed_primitive,
        fill_paint_present,
        page_fill_candidate,
        page_fill_candidate_basis: "closed-fill-and-large-span-filter",
        page_fill_candidate_reason,
        page_paint_source_evidence_proven,
        render_promotion_blocked_reason,
    }
}

pub(crate) fn fdm_page_paint_coverage_summary(
    layout: PageLayout,
    diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
) -> FdmPagePaintCoverageSummary {
    let mut summary = FdmPagePaintCoverageSummary::default();
    for diagnostic in diagnostics.iter().copied() {
        let Some(bbox) = fdm_path_unfiltered_bbox(layout, diagnostic, extent) else {
            continue;
        };
        let coverage = fdm_vector_paint_coverage(layout, diagnostic, diagnostics, bbox);
        summary.inspected_primitive_count += 1;
        if fdm_path_span_filter_blocks(layout, diagnostic.command, bbox) {
            summary.large_span_filtered_primitive_count += 1;
        } else {
            summary.rendered_primitive_count += 1;
        }
        if coverage.closed_primitive && coverage.fill_paint_present {
            summary.closed_fill_primitive_count += 1;
        }
        if coverage.page_fill_candidate {
            summary.page_fill_candidate_count += 1;
        }
        summary.max_page_coverage_ratio_ppm = summary
            .max_page_coverage_ratio_ppm
            .max(ratio_to_ppm(coverage.page_coverage_ratio));
        summary.max_viewport_coverage_ratio_ppm = summary
            .max_viewport_coverage_ratio_ppm
            .max(ratio_to_ppm(coverage.viewport_coverage_ratio));
    }
    summary
}

pub(crate) fn push_fdm_paint_coverage_json(output: &mut String, coverage: FdmPaintCoverage) {
    output.push_str("{\"bboxAreaPx\":");
    output.push_str(&format!("{:.3}", coverage.bbox_area_px));
    output.push_str(",\"pageCoverageRatio\":");
    output.push_str(&format!("{:.6}", coverage.page_coverage_ratio));
    output.push_str(",\"viewportCoverageRatio\":");
    output.push_str(&format!("{:.6}", coverage.viewport_coverage_ratio));
    output.push_str(",\"pageFillSpanFilterMaxPageRatio\":");
    output.push_str(&format!("{FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO:.6}"));
    output.push_str(",\"closedPrimitive\":");
    output.push_str(if coverage.closed_primitive {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fillPaintPresent\":");
    output.push_str(if coverage.fill_paint_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageFillCandidate\":");
    output.push_str(if coverage.page_fill_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageFillCandidateBasis\":");
    output.push_str(&json_string(coverage.page_fill_candidate_basis));
    output.push_str(",\"pageFillCandidateReason\":");
    output.push_str(&json_string(coverage.page_fill_candidate_reason));
    output.push_str(",\"paintPromotionGate\":{\"pagePaintSourceEvidenceProven\":");
    output.push_str(if coverage.page_paint_source_evidence_proven {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderable\":false,\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(coverage.render_promotion_blocked_reason));
    output.push('}');
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(coverage.render_promotion_blocked_reason));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_fdm_page_paint_coverage_summary_json(
    output: &mut String,
    summary: FdmPagePaintCoverageSummary,
) {
    output.push_str("{\"basis\":\"fdmVectorPrimitivePaintCoverage\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(if summary.page_fill_candidate_count == 0 {
        "no-page-fill-candidates"
    } else {
        "page-background-paint-order-and-extent-unproven"
    }));
    output.push_str(",\"inspectedPrimitiveCount\":");
    output.push_str(&summary.inspected_primitive_count.to_string());
    output.push_str(",\"renderedPrimitiveCount\":");
    output.push_str(&summary.rendered_primitive_count.to_string());
    output.push_str(",\"largeSpanFilteredPrimitiveCount\":");
    output.push_str(&summary.large_span_filtered_primitive_count.to_string());
    output.push_str(",\"closedFillPrimitiveCount\":");
    output.push_str(&summary.closed_fill_primitive_count.to_string());
    output.push_str(",\"pageFillCandidateCount\":");
    output.push_str(&summary.page_fill_candidate_count.to_string());
    output.push_str(",\"maxPageCoverageRatio\":");
    push_ratio_ppm_json(output, summary.max_page_coverage_ratio_ppm);
    output.push_str(",\"maxViewportCoverageRatio\":");
    push_ratio_ppm_json(output, summary.max_viewport_coverage_ratio_ppm);
    output.push_str(",\"pageFillCandidateBasis\":\"closed-fill-and-large-span-filter\"");
    output.push_str(",\"pageFillSpanFilterMaxPageRatio\":");
    output.push_str(&format!("{FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO:.6}"));
    output.push('}');
}
