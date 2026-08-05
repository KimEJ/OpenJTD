use super::*;
use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct ResolvedJseqFormulaTextSlot {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) baseline_y: f32,
    pub(crate) font_size: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct JseqFormulaVectorAlignment {
    pub(crate) cell_unit: f32,
    pub(crate) dx: f32,
    pub(crate) dy: f32,
    pub(crate) path_stroke_source_unit: f32,
    pub(crate) path_stroke_width: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ImagePayloadDiagnostic<'a> {
    pub(crate) candidate_index: usize,
    pub(crate) payload_index: usize,
    pub(crate) document: &'a Document,
    pub(crate) candidate: &'a ObjectStreamCandidate,
    pub(crate) span: &'a ObjectImagePayloadSpan,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EmbeddingFrameDiagnostic<'a> {
    pub(crate) frame_index: usize,
    pub(crate) frame: &'a ObjectEmbeddingFrameCandidate,
    pub(crate) frame_record: Option<&'a ObjectFrameRecordCandidate>,
    pub(crate) embedded_press_snapshot: Option<&'a ObjectEmbeddedPressSnapshotCandidate>,
    pub(crate) jseq3_formula: Option<&'a ObjectJseq3FormulaCandidate>,
}

pub(crate) fn page_overlay_images_json(core: &DocumentCore) -> String {
    let mut diagnostics = image_payload_overlay_diagnostics_json(&core.document);
    diagnostics.extend(fdm_image_overlay_diagnostics_json(&core.document));
    if diagnostics.is_empty() {
        return "{\"behind\":[],\"front\":[],\"imageCount\":0}".to_string();
    }

    format!(
        "{{\"behind\":[],\"front\":[],\"imageCount\":0,\"unplacedDiagnostics\":[{}],\"diagnosticCount\":{}}}",
        diagnostics.join(","),
        diagnostics.len()
    )
}

pub(crate) fn image_payload_overlay_diagnostics_json(document: &Document) -> Vec<String> {
    image_payload_diagnostics(document)
        .into_iter()
        .map(|diagnostic| {
            let mut output = String::new();
            output.push_str("{\"type\":\"jtdImagePayloadCandidate\",\"sourcePath\":");
            output.push_str(&json_string(diagnostic.candidate.path()));
            output.push_str(",\"objectCandidateIndex\":");
            output.push_str(&diagnostic.candidate_index.to_string());
            output.push_str(",\"payloadIndex\":");
            output.push_str(&diagnostic.payload_index.to_string());
            output.push_str(",\"kind\":");
            output.push_str(&json_string(diagnostic.span.kind()));
            output.push_str(",\"mime\":");
            output.push_str(&json_string(diagnostic.span.mime()));
            output.push_str(",\"signatureOffset\":");
            output.push_str(&diagnostic.span.signature_offset().to_string());
            output.push_str(",\"length\":");
            output.push_str(&diagnostic.span.len().to_string());
            output.push_str(",\"dimensions\":");
            push_object_image_dimensions_json(&mut output, diagnostic.span.dimensions());
            output.push_str(",\"objectEnvelope\":");
            push_object_image_payload_envelope_json(&mut output, diagnostic.span.envelope());
            output.push_str(",\"placementProven\":false,\"geometryDecoded\":false");
            push_image_payload_render_gate_json(&mut output, diagnostic);
            output.push_str(",\"decoded\":false}");
            output
        })
        .collect()
}

pub(crate) fn push_image_payload_render_gate_json(
    output: &mut String,
    diagnostic: ImagePayloadDiagnostic<'_>,
) {
    let source_path_candidate_present = image_payload_source_path_candidate_present(diagnostic);
    let declared_payload_length_present = diagnostic
        .span
        .envelope()
        .declared_payload_length()
        .is_some();
    let ownership_evidence_ready = image_payload_ownership_evidence_ready(diagnostic);
    let ownership_proven = ownership_evidence_ready;
    let frame_reference_row_count = diagnostic.candidate.frame_reference_row_candidates().len();
    let frame_coordinate_row_count = image_payload_frame_coordinate_row_count(diagnostic);
    let frame_linked_window_row_count = image_payload_frame_linked_window_row_count(diagnostic);
    let frame_geometry_candidate_present =
        image_payload_frame_geometry_candidate_present(diagnostic);
    let embedding_frame = image_payload_embedding_frame(diagnostic);
    let frame_record =
        embedding_frame.and_then(|frame| embedding_frame_record(diagnostic.document, frame));
    let source_frame_record_geometry_present =
        frame_record.is_some_and(image_payload_source_frame_record_has_geometry);
    let payload_frame_aspect_delta_permille =
        image_payload_frame_payload_aspect_delta_permille(frame_record, diagnostic.span);
    let best_payload_frame_aspect_delta_permille =
        image_payload_best_frame_payload_aspect_delta_permille(frame_record, diagnostic.candidate);
    let current_payload_best_frame_aspect_candidate = payload_frame_aspect_delta_permille.is_some()
        && payload_frame_aspect_delta_permille == best_payload_frame_aspect_delta_permille;

    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true");
    output.push_str(",\"sourcePathCandidatePresent\":");
    output.push_str(json_bool(source_path_candidate_present));
    output.push_str(",\"declaredPayloadLengthPresent\":");
    output.push_str(json_bool(declared_payload_length_present));
    output.push_str(",\"ownershipCandidate\":");
    if let Some(ownership) = diagnostic.candidate.ownership_candidate() {
        push_object_stream_ownership_candidate_json(output, ownership);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"ownershipReferenceCount\":");
    output.push_str(
        &diagnostic
            .candidate
            .ownership_reference_candidates()
            .len()
            .to_string(),
    );
    output.push_str(",\"ownershipEvidenceReady\":");
    output.push_str(json_bool(ownership_evidence_ready));
    output.push_str(",\"frameReferenceRowCount\":");
    output.push_str(&frame_reference_row_count.to_string());
    output.push_str(",\"frameCoordinateRowCount\":");
    output.push_str(&frame_coordinate_row_count.to_string());
    output.push_str(",\"frameLinkedWindowRowCount\":");
    output.push_str(&frame_linked_window_row_count.to_string());
    output.push_str(",\"frameGeometryCandidatePresent\":");
    output.push_str(json_bool(frame_geometry_candidate_present));
    output.push_str(",\"embeddingFrameTracePresent\":");
    output.push_str(json_bool(embedding_frame.is_some()));
    output.push_str(",\"sourceFrameRecordGeometryPresent\":");
    output.push_str(json_bool(source_frame_record_geometry_present));
    output.push_str(",\"sourceFrameTrace\":");
    push_image_payload_source_frame_trace_json(output, diagnostic, embedding_frame, frame_record);
    output.push_str(",\"candidateFrameBBox\":");
    push_image_payload_candidate_frame_bbox_json(output, frame_record);
    output.push_str(",\"payloadFrameAspectFit\":");
    push_image_payload_frame_aspect_fit_json(
        output,
        diagnostic,
        frame_record,
        payload_frame_aspect_delta_permille,
        best_payload_frame_aspect_delta_permille,
        current_payload_best_frame_aspect_candidate,
    );
    output.push_str(",\"ownershipProven\":");
    output.push_str(json_bool(ownership_proven));
    output.push_str(",\"pageGeometryProven\":false,\"paintOrderDecoded\":false,\"diagnosticRenderable\":true,\"renderable\":false");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(image_payload_render_promotion_blocked_reason(
        diagnostic,
    )));
}

pub(crate) fn image_payload_source_path_candidate_present(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> bool {
    diagnostic
        .span
        .envelope()
        .header_fields()
        .source_path_candidate()
        .is_some()
}

pub(crate) fn image_payload_ownership_evidence_ready(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> bool {
    diagnostic.candidate.ownership_candidate().is_some()
        && image_payload_source_path_candidate_present(diagnostic)
        && !diagnostic
            .candidate
            .ownership_reference_candidates()
            .is_empty()
}

pub(crate) fn image_payload_frame_coordinate_row_count(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> usize {
    diagnostic
        .candidate
        .frame_reference_row_candidates()
        .iter()
        .filter(|row| row.family() == "frame-index-tail-coordinate-row12")
        .count()
}

pub(crate) fn image_payload_frame_linked_window_row_count(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> usize {
    diagnostic
        .candidate
        .frame_reference_row_candidates()
        .iter()
        .filter(|row| row.family() == "frame-index-tail-window20" && row.suffix_link().is_some())
        .count()
}

pub(crate) fn image_payload_frame_geometry_candidate_present(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> bool {
    image_payload_frame_coordinate_row_count(diagnostic) > 0
        || image_payload_source_frame_record(diagnostic)
            .is_some_and(image_payload_source_frame_record_has_geometry)
}

pub(crate) fn image_payload_embedding_frame(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> Option<&ObjectEmbeddingFrameCandidate> {
    let embedding_index = diagnostic
        .candidate
        .ownership_candidate()
        .and_then(ObjectStreamOwnershipCandidate::embedding_index)?;
    diagnostic
        .document
        .object_embedding_frames()
        .iter()
        .find(|frame| frame.embedding_index() == embedding_index)
}

pub(crate) fn image_payload_source_frame_record(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> Option<&ObjectFrameRecordCandidate> {
    let frame = image_payload_embedding_frame(diagnostic)?;
    embedding_frame_record(diagnostic.document, frame)
}

pub(crate) fn embedding_frame_record<'a>(
    document: &'a Document,
    frame: &ObjectEmbeddingFrameCandidate,
) -> Option<&'a ObjectFrameRecordCandidate> {
    document
        .object_frame_records()
        .iter()
        .find(|record| record.row_index() as u32 == frame.frame_ref())
}

pub(crate) fn image_payload_source_frame_record_has_geometry(
    record: &ObjectFrameRecordCandidate,
) -> bool {
    record.width() > 0 && record.height() > 0
}

pub(crate) fn push_image_payload_source_frame_trace_json(
    output: &mut String,
    diagnostic: ImagePayloadDiagnostic<'_>,
    embedding_frame: Option<&ObjectEmbeddingFrameCandidate>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
) {
    let ownership_embedding_index = diagnostic
        .candidate
        .ownership_candidate()
        .and_then(ObjectStreamOwnershipCandidate::embedding_index);

    output.push_str("{\"ownershipEmbeddingIndex\":");
    push_optional_usize_json(output, ownership_embedding_index);
    output.push_str(",\"embeddingFramePresent\":");
    output.push_str(json_bool(embedding_frame.is_some()));
    output.push_str(",\"embeddingFrameRef\":");
    if let Some(frame) = embedding_frame {
        output.push_str(&frame.frame_ref().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"frameRecordPresent\":");
    output.push_str(json_bool(frame_record.is_some()));
    output.push_str(",\"frameRecordGeometry\":");
    if let Some(record) = frame_record {
        output.push_str("{\"sourcePath\":");
        output.push_str(&json_string(record.source_path()));
        output.push_str(",\"rowIndex\":");
        output.push_str(&record.row_index().to_string());
        output.push_str(",\"rowStart\":");
        output.push_str(&record.row_start().to_string());
        output.push_str(",\"objectId\":");
        output.push_str(&record.object_id().to_string());
        output.push_str(",\"objectType\":");
        output.push_str(&record.object_type().to_string());
        output.push_str(",\"x\":");
        output.push_str(&record.x().to_string());
        output.push_str(",\"y\":");
        output.push_str(&record.y().to_string());
        output.push_str(",\"width\":");
        output.push_str(&record.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&record.height().to_string());
        output.push_str(",\"decoded\":false}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_image_payload_candidate_frame_bbox_json(
    output: &mut String,
    frame_record: Option<&ObjectFrameRecordCandidate>,
) {
    let Some(record) =
        frame_record.filter(|record| image_payload_source_frame_record_has_geometry(record))
    else {
        output.push_str("null");
        return;
    };
    let (x, y, width, height) = image_payload_candidate_frame_bbox(record);
    output.push_str("{\"source\":\"EmbeddingInfo+/FrameRecord\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"placementProven\":false,\"renderable\":false");
    output.push_str(",\"x\":");
    output.push_str(&format!("{x:.3}"));
    output.push_str(",\"y\":");
    output.push_str(&format!("{y:.3}"));
    output.push_str(",\"width\":");
    output.push_str(&format!("{width:.3}"));
    output.push_str(",\"height\":");
    output.push_str(&format!("{height:.3}"));
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"page-assignment-and-paint-order-unproven\"}",
    );
}

pub(crate) fn image_payload_candidate_frame_bbox(
    record: &ObjectFrameRecordCandidate,
) -> (f32, f32, f32, f32) {
    (
        frame_record_unit_to_css_px(record.x()),
        frame_record_unit_to_css_px(record.y()),
        frame_record_unit_to_css_px(record.width()),
        frame_record_unit_to_css_px(record.height()),
    )
}

pub(crate) fn push_image_payload_frame_aspect_fit_json(
    output: &mut String,
    diagnostic: ImagePayloadDiagnostic<'_>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
    payload_frame_aspect_delta_permille: Option<u64>,
    best_payload_frame_aspect_delta_permille: Option<u64>,
    current_payload_best_frame_aspect_candidate: bool,
) {
    let Some(record) = frame_record else {
        output.push_str("null");
        return;
    };
    let Some(dimensions) = diagnostic.span.dimensions() else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"imagePayloadDimensions+/FrameRecord\"");
    output.push_str(",\"frameWidth\":");
    output.push_str(&record.width().to_string());
    output.push_str(",\"frameHeight\":");
    output.push_str(&record.height().to_string());
    output.push_str(",\"payloadWidth\":");
    output.push_str(&dimensions.width().to_string());
    output.push_str(",\"payloadHeight\":");
    output.push_str(&dimensions.height().to_string());
    output.push_str(",\"aspectDeltaPermille\":");
    push_optional_u64_json(output, payload_frame_aspect_delta_permille);
    output.push_str(",\"bestPayloadAspectDeltaPermille\":");
    push_optional_u64_json(output, best_payload_frame_aspect_delta_permille);
    output.push_str(",\"currentPayloadBestFrameAspectCandidate\":");
    output.push_str(json_bool(current_payload_best_frame_aspect_candidate));
    output.push_str(
        ",\"renderPromotionContribution\":\"payload-to-frame-aspect-fit-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"payload-selection-page-assignment-and-paint-order-unproven\"");
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn image_payload_frame_payload_aspect_delta_permille(
    frame_record: Option<&ObjectFrameRecordCandidate>,
    span: &ObjectImagePayloadSpan,
) -> Option<u64> {
    let record = frame_record?;
    let dimensions = span.dimensions()?;
    aspect_delta_permille(
        u128::from(record.width()),
        u128::from(record.height()),
        u128::from(dimensions.width()),
        u128::from(dimensions.height()),
    )
}

pub(crate) fn image_payload_best_frame_payload_aspect_delta_permille(
    frame_record: Option<&ObjectFrameRecordCandidate>,
    candidate: &ObjectStreamCandidate,
) -> Option<u64> {
    candidate
        .image_payload_spans()
        .iter()
        .filter_map(|span| image_payload_frame_payload_aspect_delta_permille(frame_record, span))
        .min()
}

pub(crate) fn image_payload_render_promotion_blocked_reason(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> &'static str {
    if diagnostic.candidate.ownership_candidate().is_none() {
        "image-payload-stream-ownership-candidate-missing"
    } else if !image_payload_source_path_candidate_present(diagnostic) {
        "image-payload-envelope-source-path-candidate-missing"
    } else if diagnostic
        .candidate
        .ownership_reference_candidates()
        .is_empty()
    {
        "image-payload-cross-stream-ownership-reference-missing"
    } else if diagnostic
        .candidate
        .frame_reference_row_candidates()
        .is_empty()
    {
        "image-payload-frame-reference-row-missing"
    } else if !image_payload_frame_geometry_candidate_present(diagnostic) {
        "image-payload-frame-geometry-candidate-missing"
    } else if image_payload_embedding_frame(diagnostic).is_none() {
        "image-payload-embedding-frame-trace-missing"
    } else if image_payload_source_frame_record(diagnostic).is_none() {
        "image-payload-frame-record-trace-missing"
    } else if !image_payload_source_frame_record(diagnostic)
        .is_some_and(image_payload_source_frame_record_has_geometry)
    {
        "image-payload-frame-record-geometry-missing"
    } else {
        "image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven"
    }
}

pub(crate) fn image_payload_diagnostics(document: &Document) -> Vec<ImagePayloadDiagnostic<'_>> {
    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for (payload_index, span) in candidate.image_payload_spans().iter().enumerate() {
            if svg_embeddable_image_payload(span) {
                diagnostics.push(ImagePayloadDiagnostic {
                    candidate_index,
                    payload_index,
                    document,
                    candidate,
                    span,
                });
            }
        }
    }
    diagnostics
}

pub(crate) fn visual_list_diagnostics(document: &Document) -> Vec<VisualListDiagnostic<'_>> {
    document
        .object_stream_candidates()
        .iter()
        .enumerate()
        .filter_map(|(candidate_index, candidate)| {
            candidate
                .visual_list_candidate()
                .map(|visual_list| VisualListDiagnostic {
                    candidate_index,
                    candidate,
                    visual_list,
                })
        })
        .collect()
}

pub(crate) fn embedding_frame_diagnostics(
    document: &Document,
) -> Vec<EmbeddingFrameDiagnostic<'_>> {
    document
        .object_embedding_frames()
        .iter()
        .enumerate()
        .map(|(frame_index, frame)| {
            let frame_record = embedding_frame_record(document, frame);
            let jseq3_path = format!(
                "/EmbedItems/Embedding {}/JSEQ3Contents",
                frame.embedding_index()
            );
            let jseq3_formula = document
                .object_stream_candidates()
                .iter()
                .find(|candidate| candidate.path() == jseq3_path)
                .and_then(ObjectStreamCandidate::jseq3_formula_candidate);
            let snapshot_path = format!(
                "/EmbedItems/Embedding {}/\x03EmbeddedPress",
                frame.embedding_index()
            );
            let embedded_press_snapshot = document
                .object_stream_candidates()
                .iter()
                .find(|candidate| candidate.path() == snapshot_path)
                .and_then(ObjectStreamCandidate::embedded_press_snapshot_candidate);
            EmbeddingFrameDiagnostic {
                frame_index,
                frame,
                frame_record,
                embedded_press_snapshot,
                jseq3_formula,
            }
        })
        .collect()
}

pub(crate) fn svg_embeddable_image_payload(span: &ObjectImagePayloadSpan) -> bool {
    image_payload_svg_data_uri(span).is_some()
}

pub(crate) fn push_object_image_signature_hits_json(
    output: &mut String,
    hits: &[ObjectImageSignatureHit],
) {
    output.push('[');
    for (index, hit) in hits.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_page_layer_image_payload_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    overlay_index: usize,
    diagnostic: ImagePayloadDiagnostic<'_>,
) {
    let (x, y, width, height) =
        image_payload_overlay_layout(layout, overlay_index, diagnostic.span);
    let dimensions = diagnostic.span.dimensions().unwrap();
    output.push_str("{\"type\":\"imagePayloadDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"objectStreamCandidate\",\"projectionKind\":\"diagnosticProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"payloadIndex\":");
    output.push_str(&diagnostic.payload_index.to_string());
    output.push_str(",\"mime\":");
    output.push_str(&json_string(diagnostic.span.mime()));
    output.push_str(",\"naturalWidth\":");
    output.push_str(&dimensions.width().to_string());
    output.push_str(",\"naturalHeight\":");
    output.push_str(&dimensions.height().to_string());
    output.push_str(",\"payloadLength\":");
    output.push_str(&diagnostic.span.len().to_string());
    output.push_str(",\"objectEnvelope\":");
    push_object_image_payload_envelope_json(output, diagnostic.span.envelope());
    push_image_payload_render_gate_json(output, diagnostic);
    output.push('}');
}

pub(crate) fn push_page_layer_visual_list_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: VisualListDiagnostic<'_>,
) {
    output.push_str("{\"type\":\"visualListRasterDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"objectStreamCandidate\",\"projectionKind\":\"visualListRasterProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":true,\"renderable\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"naturalWidth\":");
    output.push_str(&diagnostic.visual_list.width().to_string());
    output.push_str(",\"naturalHeight\":");
    output.push_str(&diagnostic.visual_list.height().to_string());
    output.push_str(",\"bitDepth\":");
    output.push_str(&diagnostic.visual_list.bit_depth().to_string());
    output.push_str(",\"horizontalRunCount\":");
    output.push_str(
        &visual_list_horizontal_runs(diagnostic.visual_list)
            .len()
            .to_string(),
    );
    output.push_str(",\"titleBand\":");
    let runs = visual_list_horizontal_runs(diagnostic.visual_list);
    if let Some(band) = visual_list_title_band(diagnostic.visual_list, &runs) {
        let scale_x = layout.width_px() / diagnostic.visual_list.width() as f32;
        let scale_y = layout.height_px() / diagnostic.visual_list.height() as f32;
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3},\"projectionKind\":\"visualListFillBandProjection\",\"decoded\":false}}",
            band.x * scale_x,
            band.y * scale_y,
            band.width * scale_x,
            band.height * scale_y
        ));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rleDataOffset\":");
    output.push_str(&diagnostic.visual_list.rle_data_offset().to_string());
    output.push_str(",\"rleDataLength\":");
    output.push_str(&diagnostic.visual_list.rle_data_len().to_string());
    output.push('}');
}

pub(crate) fn push_page_layer_embedding_frame_diagnostic_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    lines: &[PageTextLine],
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) {
    let Some((x, y, width, height)) =
        embedding_frame_render_bbox(layout, lines, document, diagnostic)
    else {
        return;
    };
    output.push_str("{\"type\":\"embeddingFrameDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    let snapshot_vector_segment_count = diagnostic
        .embedded_press_snapshot
        .map(|snapshot| snapshot.vector_segments().len())
        .unwrap_or_default();
    let snapshot_vector_renderable = embedding_frame_snapshot_vector_renderable(diagnostic);
    output.push_str(",\"source\":\"embedItemsEmbeddingInfo+frame\",\"projectionKind\":\"diagnosticProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":");
    output.push_str(if snapshot_vector_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.frame.source_path()));
    output.push_str(",\"frameCandidateIndex\":");
    output.push_str(&diagnostic.frame_index.to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&diagnostic.frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    output.push_str(&json_string(diagnostic.frame.class_name()));
    output.push_str(",\"frameRef\":");
    output.push_str(&diagnostic.frame.frame_ref().to_string());
    output.push_str(",\"frameSize\":{\"width\":");
    output.push_str(&diagnostic.frame.frame_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&diagnostic.frame.frame_height().to_string());
    output.push_str("},\"matchedFrameRecord\":");
    if let Some(record) = diagnostic.frame_record {
        output.push_str("{\"sourcePath\":");
        output.push_str(&json_string(record.source_path()));
        output.push_str(",\"rowIndex\":");
        output.push_str(&record.row_index().to_string());
        output.push_str(",\"objectId\":");
        output.push_str(&record.object_id().to_string());
        output.push_str(",\"objectType\":");
        output.push_str(&record.object_type().to_string());
        output.push_str(",\"geometry\":{\"x\":");
        output.push_str(&record.x().to_string());
        output.push_str(",\"y\":");
        output.push_str(&record.y().to_string());
        output.push_str(",\"width\":");
        output.push_str(&record.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&record.height().to_string());
        output.push_str("}}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddedPressSnapshot\":");
    if let Some(snapshot) = diagnostic.embedded_press_snapshot {
        output.push_str("{\"format\":\"JSSnapShot32\",\"width\":");
        output.push_str(&snapshot.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&snapshot.height().to_string());
        output.push_str(",\"vectorSegmentCount\":");
        output.push_str(&snapshot_vector_segment_count.to_string());
        output.push_str(",\"renderable\":");
        output.push_str(if snapshot_vector_renderable {
            "true"
        } else {
            "false"
        });
        output.push_str(
            ",\"projectionKind\":\"embeddedPressSnapshotVectorProjection\",\"decoded\":false}",
        );
    } else {
        output.push_str("null");
    }
    output.push_str(",\"linkedJseq3Formula\":");
    if let Some(formula) = diagnostic.jseq3_formula {
        output.push_str("{\"format\":\"JSEQ3Contents\",\"magic\":");
        output.push_str(&json_string(formula.magic()));
        output.push_str(",\"soTrailerOffset\":");
        push_option_usize_json(output, formula.so_trailer_offset());
        output.push_str(",\"textMarkerCount\":");
        output.push_str(&formula.text_markers().len().to_string());
        output.push_str(",\"textTokenCount\":");
        output.push_str(&formula.text_tokens().len().to_string());
        output.push_str(",\"textRunCount\":");
        output.push_str(&formula.text_runs().len().to_string());
        output.push_str(",\"decoded\":false,\"renderable\":false}");
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn embedding_frame_snapshot_vector_renderable(
    _diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> bool {
    // Preserve raw EmbeddedPress vector candidates as model evidence until their geometry is decoded.
    false
}

pub(crate) fn image_signature_offset_range(
    hits: &[ObjectImageSignatureHit],
) -> Option<(usize, usize)> {
    Some((
        hits.iter().map(ObjectImageSignatureHit::offset).min()?,
        hits.iter().map(ObjectImageSignatureHit::offset).max()?,
    ))
}

pub(crate) fn nearest_image_signature_offset(
    hits: &[ObjectImageSignatureHit],
    offset: usize,
) -> Option<(usize, usize)> {
    hits.iter()
        .map(|hit| {
            let signature_offset = hit.offset();
            (signature_offset, offset.abs_diff(signature_offset))
        })
        .min_by_key(|(_, distance)| *distance)
}

pub(crate) fn push_image_signature_offset_range_json(
    output: &mut String,
    hits: &[ObjectImageSignatureHit],
) {
    push_optional_usize_range_json(
        output,
        hits.iter().map(ObjectImageSignatureHit::offset).min(),
        hits.iter().map(ObjectImageSignatureHit::offset).max(),
    );
}
