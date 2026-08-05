use super::*;
use crate::*;
use std::fs;

#[test]
fn local_samples_produce_validation_warning_json_when_available() {
    let sample_dir = local_samples_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut sample_count = 0usize;
    let mut warning_sample_count = 0usize;
    let mut control_boundary_count = 0usize;
    let mut control_range_overlap_count = 0usize;
    let mut text_boundary_candidate_count = 0usize;
    let mut projected_control_count = 0usize;
    let mut page_control_layout_count = 0usize;
    let mut failures = Vec::new();

    let no_jtd_samples = fs::read_dir(&sample_dir).unwrap().all(|entry| {
        !entry
            .unwrap()
            .path()
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| matches!(ext, "jtd" | "jtt" | "jttc"))
    });
    if no_jtd_samples {
        return;
    }

    for entry in fs::read_dir(&sample_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if !matches!(extension, "jtd" | "jtt" | "jttc") {
            continue;
        }

        sample_count += 1;
        let bytes = fs::read(&path).unwrap();
        match DocumentCore::from_bytes(&bytes) {
            Ok(core) => {
                control_boundary_count += core.document().text_control_boundaries().len();
                control_range_overlap_count += core
                    .document()
                    .text_count_ranges()
                    .iter()
                    .map(|range| range.control_range_overlaps().len())
                    .sum::<usize>();
                text_boundary_candidate_count += core.document().text_boundary_candidates().len();
                if !core.document().text_boundary_candidates().is_empty() {
                    let info = core.get_document_info();
                    assert!(info.contains("\"textBoundaryCandidateCount\":"));
                    assert!(info.contains("\"textBoundaryCandidates\":["));
                    assert!(info.contains("\"kind\":\"controlDelimitedTextCountRange\""));
                }
                let projected_controls = projected_text_controls(core.document());
                projected_control_count += projected_controls.len();
                if !projected_controls.is_empty() {
                    for page in 0..core.page_count() {
                        let layout = core.get_page_control_layout(page).unwrap();
                        assert!(layout.starts_with("{\"controls\":["));
                        if layout.contains("\"type\":\"jtdControl\"") {
                            assert!(layout.contains("\"source\":\"textControlBoundary\""));
                            assert!(layout.contains("\"decoded\":false"));
                            page_control_layout_count += 1;
                            break;
                        }
                    }
                }
                let warnings = core.get_validation_warnings();
                assert!(warnings.starts_with("{\"count\":"));
                assert!(warnings.contains("\"summary\":{"));
                assert!(warnings.contains("\"warnings\":["));
                if !warnings.contains("\"count\":0") {
                    warning_sample_count += 1;
                }
            }
            Err(error) => failures.push(format!("{}: {error}", path.display())),
        }
    }

    assert_eq!(failures, Vec::<String>::new());
    if sample_count == 0 {
        return;
    }
    assert!(warning_sample_count > 0);
    assert!(control_boundary_count > 0);
    if control_range_overlap_count == 0 {
        return;
    }
    assert!(text_boundary_candidate_count > 0);
    assert_eq!(text_boundary_candidate_count, control_range_overlap_count);
    assert!(projected_control_count > 0);
    assert!(page_control_layout_count > 0);
}

#[test]
fn local_samples_project_column_grid_candidates_to_svg_and_layer_tree_when_available() {
    let sample_dir = local_samples_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut _sample_count = 0usize;
    let mut files_with_grid = 0usize;
    let mut grid_candidate_count = 0usize;
    let mut svg_overlay_count = 0usize;
    let mut layer_op_count = 0usize;
    let mut source_derived_layout_count = 0usize;
    let mut source_derived_svg_overlay_count = 0usize;
    let mut failures = Vec::new();

    let no_jtd_samples = fs::read_dir(&sample_dir).unwrap().all(|entry| {
        !entry
            .unwrap()
            .path()
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| matches!(ext, "jtd" | "jtt" | "jttc"))
    });
    if no_jtd_samples {
        return;
    }

    for entry in fs::read_dir(&sample_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if !matches!(extension, "jtd" | "jtt" | "jttc") {
            continue;
        }
        if !path.with_extension("pdf").exists() {
            continue;
        }

        _sample_count += 1;
        let bytes = fs::read(&path).unwrap();
        match DocumentCore::from_bytes(&bytes) {
            Ok(core) => {
                let current_grid_count = core
                    .document()
                    .table_candidates()
                    .iter()
                    .filter(|candidate| candidate.column_segment_grid_candidate().is_some())
                    .count();
                if current_grid_count == 0 {
                    continue;
                }

                files_with_grid += 1;
                grid_candidate_count += current_grid_count;
                let svg = core.render_page_svg(0).unwrap();
                let layer_tree = core.get_page_layer_tree(0).unwrap();
                svg_overlay_count += svg.matches("class=\"rjtd-column-grid-candidate\"").count();
                source_derived_svg_overlay_count += svg
                    .matches("data-projection-kind=\"sourceDerivedDiagnosticProjection\"")
                    .count();
                layer_op_count += layer_tree
                    .matches("\"type\":\"tableGridCandidate\"")
                    .count();
                source_derived_layout_count += layer_tree
                    .matches(
                        "\"sourceDerivedLayoutCandidate\":{\"source\":\"documentTextLineHeaders+fallbackTextAnchors\"",
                    )
                    .count();

                if svg.contains("class=\"rjtd-column-grid-candidate\"") {
                    assert!(
                        svg.contains("data-projection-kind=\"tableProjection\"")
                            || svg.contains(
                                "data-projection-kind=\"sourceDerivedDiagnosticProjection\""
                            )
                    );
                    assert!(svg.contains("data-source-derived-layout-candidate=\""));
                    assert!(svg.contains("data-decoded=\"false\""));
                    assert!(svg.contains("data-geometry-decoded=\"false\""));
                    assert!(svg.contains("data-col-count-candidate=\""));
                }
                if svg.contains("data-projection-kind=\"sourceDerivedDiagnosticProjection\"") {
                    assert!(svg.contains("data-reference-backed=\"false\""));
                    assert!(svg.contains("data-placement-derived-from-source=\"true\""));
                }
                if local_sample_has_capability(
                    &path,
                    LocalSampleCapability::UsesReferenceBackedColumnGridProjection,
                ) {
                    assert!(
                        !svg.contains("data-projection-kind=\"sourceDerivedDiagnosticProjection\"")
                    );
                }
                assert!(
                    layer_tree.contains("\"projectionKind\":\"diagnosticProjection\"")
                        || layer_tree.contains("\"projectionKind\":\"tableProjection\"")
                );
                assert!(layer_tree.contains("\"decoded\":false"));
                assert!(layer_tree.contains("\"geometryDecoded\":false"));
            }
            Err(error) => failures.push(format!("{}: {error}", path.display())),
        }
    }

    assert_eq!(failures, Vec::<String>::new());
    if files_with_grid == 0 {
        return;
    }
    assert!(svg_overlay_count <= grid_candidate_count);
    assert_eq!(layer_op_count, grid_candidate_count);
    if source_derived_layout_count == 0 {
        return;
    }
    assert!(source_derived_svg_overlay_count <= source_derived_layout_count);
}

#[test]
#[cfg(feature = "bitmap-images")]
fn local_samples_project_image_payload_diagnostics_when_available() {
    let sample_dir = local_samples_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut _sample_count = 0usize;
    let mut files_with_images = 0usize;
    let mut image_payload_count = 0usize;
    let mut projected_payload_count = 0usize;
    let mut svg_overlay_count = 0usize;
    let mut layer_op_count = 0usize;
    let mut overlay_json_count = 0usize;
    let mut ownership_proven_count = 0usize;
    let mut frame_geometry_candidate_count = 0usize;
    let mut embedding_frame_trace_count = 0usize;
    let mut source_frame_record_geometry_count = 0usize;
    let mut candidate_frame_bbox_count = 0usize;
    let mut payload_frame_aspect_fit_count = 0usize;
    let mut final_gate_blocker_count = 0usize;
    let mut failures = Vec::new();

    let no_jtd_samples = fs::read_dir(&sample_dir).unwrap().all(|entry| {
        !entry
            .unwrap()
            .path()
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| matches!(ext, "jtd" | "jtt" | "jttc"))
    });
    if no_jtd_samples {
        return;
    }

    for entry in fs::read_dir(&sample_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if !matches!(extension, "jtd" | "jtt" | "jttc") {
            continue;
        }

        _sample_count += 1;
        let bytes = fs::read(&path).unwrap();
        match DocumentCore::from_bytes(&bytes) {
            Ok(core) => {
                let current_payload_count = image_payload_diagnostics(core.document()).len();
                if current_payload_count == 0 {
                    continue;
                }

                files_with_images += 1;
                image_payload_count += current_payload_count;
                projected_payload_count +=
                    current_payload_count.min(APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS);
                let svg = core.render_page_svg(0).unwrap();
                let layer_tree = core.get_page_layer_tree(0).unwrap();
                let overlay_images = core.get_page_overlay_images(0).unwrap();
                svg_overlay_count += svg
                    .matches("class=\"rjtd-image-payload-diagnostic\"")
                    .count();
                layer_op_count += layer_tree
                    .matches("\"type\":\"imagePayloadDiagnostic\"")
                    .count();
                overlay_json_count += overlay_images
                    .matches("\"type\":\"jtdImagePayloadCandidate\"")
                    .count();
                ownership_proven_count += layer_tree.matches("\"ownershipProven\":true").count();
                frame_geometry_candidate_count += layer_tree
                    .matches("\"frameGeometryCandidatePresent\":true")
                    .count();
                embedding_frame_trace_count += layer_tree
                    .matches("\"embeddingFrameTracePresent\":true")
                    .count();
                source_frame_record_geometry_count += layer_tree
                    .matches("\"sourceFrameRecordGeometryPresent\":true")
                    .count();
                candidate_frame_bbox_count += layer_tree
                    .matches("\"candidateFrameBBox\":{\"source\":\"EmbeddingInfo+/FrameRecord\"")
                    .count();
                payload_frame_aspect_fit_count += layer_tree
                    .matches("\"payloadFrameAspectFit\":{\"source\":\"imagePayloadDimensions+/FrameRecord\"")
                    .count();
                final_gate_blocker_count += layer_tree
                    .matches(
                        "\"renderPromotionBlockedReason\":\"image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven\"",
                    )
                    .count();

                assert!(svg.contains("data:image/png;base64,"));
                assert!(svg.contains("data-decoded=\"false\""));
                assert!(svg.contains("data-geometry-decoded=\"false\""));
                assert!(svg.contains("data-placement-proven=\"false\""));
                assert!(svg.contains("data-diagnostic-renderable=\"true\""));
                assert!(svg.contains("data-renderable=\"false\""));
                assert!(svg.contains("data-frame-reference-row-count=\""));
                assert!(svg.contains("data-frame-coordinate-row-count=\""));
                assert!(svg.contains("data-frame-linked-window-row-count=\""));
                assert!(svg.contains("data-frame-geometry-candidate-present=\""));
                assert!(svg.contains("data-embedding-frame-trace-present=\""));
                assert!(svg.contains("data-source-frame-record-geometry-present=\""));
                assert!(svg.contains("data-candidate-frame-bbox-present=\""));
                assert!(svg.contains("data-candidate-frame-x=\""));
                assert!(svg.contains("data-candidate-frame-y=\""));
                assert!(svg.contains("data-candidate-frame-width=\""));
                assert!(svg.contains("data-candidate-frame-height=\""));
                assert!(svg.contains("data-payload-frame-aspect-fit-present=\""));
                assert!(svg.contains("data-payload-frame-aspect-delta-permille=\""));
                assert!(svg.contains("data-best-payload-frame-aspect-delta-permille=\""));
                assert!(svg.contains("data-current-payload-best-frame-aspect-candidate=\""));
                assert!(layer_tree.contains("\"placementProven\":false"));
                assert!(layer_tree.contains("\"diagnosticRenderable\":true"));
                assert!(layer_tree.contains("\"renderable\":false"));
                assert!(layer_tree.contains("\"ownershipProven\":"));
                assert!(layer_tree.contains("\"frameReferenceRowCount\":"));
                assert!(layer_tree.contains("\"frameCoordinateRowCount\":"));
                assert!(layer_tree.contains("\"frameLinkedWindowRowCount\":"));
                assert!(layer_tree.contains("\"frameGeometryCandidatePresent\":"));
                assert!(layer_tree.contains("\"embeddingFrameTracePresent\":"));
                assert!(layer_tree.contains("\"sourceFrameRecordGeometryPresent\":"));
                assert!(layer_tree.contains("\"sourceFrameTrace\":"));
                assert!(layer_tree.contains("\"candidateFrameBBox\":"));
                assert!(layer_tree.contains("\"payloadFrameAspectFit\":"));
                assert!(layer_tree.contains("\"pageGeometryProven\":false"));
                assert!(layer_tree.contains("\"paintOrderDecoded\":false"));
                assert!(layer_tree.contains("\"renderPromotionBlockedReason\":"));
                assert!(layer_tree.contains("\"objectEnvelope\":"));
                assert!(overlay_images.contains("\"placementProven\":false"));
                assert!(overlay_images.contains("\"geometryDecoded\":false"));
                assert!(overlay_images.contains("\"diagnosticRenderable\":true"));
                assert!(overlay_images.contains("\"renderable\":false"));
                assert!(overlay_images.contains("\"ownershipProven\":"));
                assert!(overlay_images.contains("\"frameReferenceRowCount\":"));
                assert!(overlay_images.contains("\"frameCoordinateRowCount\":"));
                assert!(overlay_images.contains("\"frameLinkedWindowRowCount\":"));
                assert!(overlay_images.contains("\"frameGeometryCandidatePresent\":"));
                assert!(overlay_images.contains("\"embeddingFrameTracePresent\":"));
                assert!(overlay_images.contains("\"sourceFrameRecordGeometryPresent\":"));
                assert!(overlay_images.contains("\"sourceFrameTrace\":"));
                assert!(overlay_images.contains("\"candidateFrameBBox\":"));
                assert!(overlay_images.contains("\"payloadFrameAspectFit\":"));
                assert!(overlay_images.contains("\"pageGeometryProven\":false"));
                assert!(overlay_images.contains("\"paintOrderDecoded\":false"));
                assert!(overlay_images.contains("\"renderPromotionBlockedReason\":"));
                assert!(overlay_images.contains("\"objectEnvelope\":"));
            }
            Err(error) => failures.push(format!("{}: {error}", path.display())),
        }
    }

    assert_eq!(failures, Vec::<String>::new());
    if files_with_images == 0 {
        return;
    }
    assert_eq!(svg_overlay_count, projected_payload_count);
    assert_eq!(layer_op_count, projected_payload_count);
    assert_eq!(overlay_json_count, image_payload_count);
    assert!(ownership_proven_count > 0);
    assert!(frame_geometry_candidate_count > 0);
    assert!(embedding_frame_trace_count > 0);
    assert!(source_frame_record_geometry_count > 0);
    assert!(candidate_frame_bbox_count > 0);
    assert!(payload_frame_aspect_fit_count > 0);
    assert!(final_gate_blocker_count > 0);
}
