use super::*;
use crate::*;

#[test]
fn fdm_bbox_center_handles_extreme_bounds_without_overflow() {
    assert_eq!(
        fdm_bbox_center((i32::MIN, i32::MIN, i32::MAX, i32::MAX)),
        (-1, -1)
    );
    assert_eq!(fdm_bbox_center((-3, -3, -2, -2)), (-3, -3));
}

pub(super) fn embedded_press_state_record_payload_first_words(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    record_type: u32,
) -> Vec<u32> {
    path.state_records()
        .iter()
        .filter(|record| record.record_type() == record_type)
        .filter_map(|record| record.payload_le32_words().first().copied())
        .collect::<Vec<_>>()
}

pub(super) fn embedded_press_test_outline_path(
    commands: Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
) -> ObjectEmbeddedPressVectorPathCandidate {
    ObjectEmbeddedPressVectorPathCandidate::new(
        ObjectEmbeddedPressVectorPathKind::Outline,
        None,
        Vec::new(),
        commands,
    )
}

pub(super) fn push_embedded_press_test_line_to(
    commands: &mut Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
    from: (u32, u32),
    to: (u32, u32),
) {
    commands.push(ObjectEmbeddedPressVectorPathCommandCandidate::CubicTo {
        x1: from.0,
        y1: from.1,
        x2: to.0,
        y2: to.1,
        x3: to.0,
        y3: to.1,
    });
}

pub(super) fn test_fdm_vector_segment(
    bbox: ObjectFdmIndexBbox,
    source_width: i32,
    source_height: i32,
) -> ObjectFdmVectorSegmentCandidate {
    ObjectFdmVectorSegmentCandidate::new(
        0,
        FdmVectorSegmentHeader {
            declared_len: 10,
            command_count: 0,
            command_offsets: Vec::new(),
            bbox: Some(bbox),
            source_width,
            source_height,
        },
    )
}

pub(super) fn test_fdm_text_candidate(
    text: &str,
    bbox: ObjectFdmIndexBbox,
) -> ObjectFdmTextCandidate {
    ObjectFdmTextCandidate::new(text, 0, 0, Vec::new(), Some(bbox))
}

#[test]
fn image_payload_dimensions_reads_jpeg_sof_metadata() {
    let payload = minimal_jpeg_payload();

    let dimensions = jpeg_payload_dimensions(payload).unwrap();
    assert_eq!(dimensions.width(), 32);
    assert_eq!(dimensions.height(), 16);
    assert_eq!(image_payload_dimensions(payload), Some(dimensions));
    assert_eq!(jpeg_payload_end(payload, 0), Some(payload.len()));
    assert_eq!(
        jpeg_payload_end(b"\xff\xd8\xff\xff\xff\xfc\0\0\0\0\xff\xd9", 0),
        None
    );
}

#[test]
#[cfg(feature = "bitmap-images")]
fn document_core_projects_complete_image_payloads_as_diagnostic_svg_overlays() {
    let image_stream_path = "/EmbedItems/Embedding 1/Contents";
    let png_payload = minimal_png_payload();
    let (mut image_payload, _, _) = image_payload_with_header_fixture(png_payload.len());
    image_payload.extend_from_slice(png_payload);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (image_stream_path, &image_payload),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-image-payload-diagnostic\""));
    assert!(svg.contains("data:image/png;base64,"));
    assert!(svg.contains("data-decoded=\"false\""));
    assert!(svg.contains("data-geometry-decoded=\"false\""));
    assert!(svg.contains("data-placement-proven=\"false\""));
    assert!(svg.contains("data-diagnostic-only=\"true\""));
    assert!(svg.contains("data-diagnostic-renderable=\"true\""));
    assert!(svg.contains("data-renderable=\"false\""));
    assert!(svg.contains("data-source-path-candidate-present=\"true\""));
    assert!(svg.contains("data-declared-payload-length-present=\"true\""));
    assert!(svg.contains("data-ownership-reference-count=\"0\""));
    assert!(svg.contains("data-ownership-evidence-ready=\"false\""));
    assert!(svg.contains("data-frame-reference-row-count=\"0\""));
    assert!(svg.contains("data-frame-coordinate-row-count=\"0\""));
    assert!(svg.contains("data-frame-linked-window-row-count=\"0\""));
    assert!(svg.contains("data-frame-geometry-candidate-present=\"false\""));
    assert!(svg.contains("data-embedding-frame-trace-present=\"false\""));
    assert!(svg.contains("data-source-frame-record-geometry-present=\"false\""));
    assert!(svg.contains("data-candidate-frame-bbox-present=\"false\""));
    assert!(svg.contains("data-candidate-frame-x=\"null\""));
    assert!(svg.contains("data-candidate-frame-y=\"null\""));
    assert!(svg.contains("data-candidate-frame-width=\"null\""));
    assert!(svg.contains("data-candidate-frame-height=\"null\""));
    assert!(svg.contains("data-payload-frame-aspect-fit-present=\"false\""));
    assert!(svg.contains("data-payload-frame-aspect-delta-permille=\"null\""));
    assert!(svg.contains("data-best-payload-frame-aspect-delta-permille=\"null\""));
    assert!(svg.contains("data-current-payload-best-frame-aspect-candidate=\"false\""));
    assert!(svg.contains(
        "data-render-promotion-blocked-reason=\"image-payload-cross-stream-ownership-reference-missing\""
    ));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"imagePayloadDiagnostic\""));
    assert!(layer_tree.contains("\"sourcePath\":\"/EmbedItems/Embedding 1/Contents\""));
    assert!(layer_tree.contains("\"projectionKind\":\"diagnosticProjection\""));
    assert!(layer_tree.contains("\"placementProven\":false"));
    assert!(layer_tree.contains("\"diagnosticOnly\":true"));
    assert!(layer_tree.contains("\"diagnosticRenderable\":true"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains("\"sourcePathCandidatePresent\":true"));
    assert!(layer_tree.contains("\"declaredPayloadLengthPresent\":true"));
    assert!(layer_tree.contains("\"ownershipReferenceCount\":0"));
    assert!(layer_tree.contains("\"ownershipEvidenceReady\":false"));
    assert!(layer_tree.contains("\"frameReferenceRowCount\":0"));
    assert!(layer_tree.contains("\"frameCoordinateRowCount\":0"));
    assert!(layer_tree.contains("\"frameLinkedWindowRowCount\":0"));
    assert!(layer_tree.contains("\"frameGeometryCandidatePresent\":false"));
    assert!(layer_tree.contains("\"embeddingFrameTracePresent\":false"));
    assert!(layer_tree.contains("\"sourceFrameRecordGeometryPresent\":false"));
    assert!(layer_tree.contains("\"sourceFrameTrace\":"));
    assert!(layer_tree.contains("\"embeddingFramePresent\":false"));
    assert!(layer_tree.contains("\"frameRecordPresent\":false"));
    assert!(layer_tree.contains("\"frameRecordGeometry\":null"));
    assert!(layer_tree.contains("\"candidateFrameBBox\":null"));
    assert!(layer_tree.contains("\"payloadFrameAspectFit\":null"));
    assert!(layer_tree.contains("\"ownershipProven\":false"));
    assert!(layer_tree.contains("\"pageGeometryProven\":false"));
    assert!(layer_tree.contains("\"paintOrderDecoded\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"image-payload-cross-stream-ownership-reference-missing\""
    ));
    assert!(layer_tree.contains("\"objectEnvelope\":{\"headerStart\":0"));
    assert!(layer_tree.contains("\"headerFields\""));
    assert!(layer_tree.contains("\"sourcePathCandidate\""));
    assert!(layer_tree.contains("\"decoded\":false"));

    let overlay_images = core.get_page_overlay_images(0).unwrap();
    assert!(overlay_images.contains("\"type\":\"jtdImagePayloadCandidate\""));
    assert!(overlay_images.contains("\"sourcePath\":\"/EmbedItems/Embedding 1/Contents\""));
    assert!(overlay_images.contains("\"placementProven\":false"));
    assert!(overlay_images.contains("\"geometryDecoded\":false"));
    assert!(overlay_images.contains("\"diagnosticOnly\":true"));
    assert!(overlay_images.contains("\"diagnosticRenderable\":true"));
    assert!(overlay_images.contains("\"renderable\":false"));
    assert!(overlay_images.contains("\"sourcePathCandidatePresent\":true"));
    assert!(overlay_images.contains("\"declaredPayloadLengthPresent\":true"));
    assert!(overlay_images.contains("\"ownershipReferenceCount\":0"));
    assert!(overlay_images.contains("\"ownershipEvidenceReady\":false"));
    assert!(overlay_images.contains("\"frameReferenceRowCount\":0"));
    assert!(overlay_images.contains("\"frameCoordinateRowCount\":0"));
    assert!(overlay_images.contains("\"frameLinkedWindowRowCount\":0"));
    assert!(overlay_images.contains("\"frameGeometryCandidatePresent\":false"));
    assert!(overlay_images.contains("\"embeddingFrameTracePresent\":false"));
    assert!(overlay_images.contains("\"sourceFrameRecordGeometryPresent\":false"));
    assert!(overlay_images.contains("\"sourceFrameTrace\":"));
    assert!(overlay_images.contains("\"embeddingFramePresent\":false"));
    assert!(overlay_images.contains("\"frameRecordPresent\":false"));
    assert!(overlay_images.contains("\"frameRecordGeometry\":null"));
    assert!(overlay_images.contains("\"candidateFrameBBox\":null"));
    assert!(overlay_images.contains("\"payloadFrameAspectFit\":null"));
    assert!(overlay_images.contains("\"ownershipProven\":false"));
    assert!(overlay_images.contains("\"pageGeometryProven\":false"));
    assert!(overlay_images.contains("\"paintOrderDecoded\":false"));
    assert!(overlay_images.contains(
        "\"renderPromotionBlockedReason\":\"image-payload-cross-stream-ownership-reference-missing\""
    ));
    assert!(overlay_images.contains("\"objectEnvelope\":{\"headerStart\":0"));
    assert!(overlay_images.contains("\"decoded\":false"));
}

#[test]
#[cfg(feature = "bitmap-images")]
fn image_payload_render_gate_preserves_source_frame_trace_without_promotion() {
    let image_stream_path = "/EmbedItems/Embedding 24/Contents";
    let png_payload = minimal_png_payload();
    let (mut image_payload, _, _) = image_payload_with_header_fixture(png_payload.len());
    image_payload.extend_from_slice(png_payload);

    let mut frame = frame_stream_fixture();
    frame[7..9].copy_from_slice(&24u16.to_be_bytes());
    let embedding_info = embedding_info_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (image_stream_path, &image_payload),
        (EMBEDDING_INFO_PATH, &embedding_info),
        ("/Frame", &frame),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"imagePayloadDiagnostic\""));
    assert!(layer_tree.contains("\"ownershipReferenceCount\":"));
    assert!(layer_tree.contains("\"ownershipEvidenceReady\":true"));
    assert!(layer_tree.contains("\"ownershipProven\":true"));
    assert!(layer_tree.contains("\"frameReferenceRowCount\":"));
    assert!(layer_tree.contains("\"frameCoordinateRowCount\":"));
    assert!(layer_tree.contains("\"frameGeometryCandidatePresent\":true"));
    assert!(layer_tree.contains("\"embeddingFrameTracePresent\":true"));
    assert!(layer_tree.contains("\"sourceFrameRecordGeometryPresent\":true"));
    assert!(layer_tree.contains("\"sourceFrameTrace\":"));
    assert!(layer_tree.contains("\"ownershipEmbeddingIndex\":24"));
    assert!(layer_tree.contains("\"embeddingFrameRef\":1"));
    assert!(layer_tree.contains("\"frameRecordPresent\":true"));
    assert!(layer_tree.contains("\"frameRecordGeometry\":{\"sourcePath\":\"/Frame\""));
    assert!(layer_tree.contains("\"width\":13260"));
    assert!(layer_tree.contains("\"height\":1327"));
    assert!(
        layer_tree.contains("\"candidateFrameBBox\":{\"source\":\"EmbeddingInfo+/FrameRecord\"")
    );
    assert!(
        layer_tree.contains(
            "\"renderPromotionBlockedReason\":\"page-assignment-and-paint-order-unproven\""
        )
    );
    assert!(
        layer_tree.contains(
            "\"payloadFrameAspectFit\":{\"source\":\"imagePayloadDimensions+/FrameRecord\""
        )
    );
    assert!(layer_tree.contains("\"payloadWidth\":1"));
    assert!(layer_tree.contains("\"payloadHeight\":1"));
    assert!(layer_tree.contains("\"aspectDeltaPermille\":899"));
    assert!(layer_tree.contains("\"bestPayloadAspectDeltaPermille\":899"));
    assert!(layer_tree.contains("\"currentPayloadBestFrameAspectCandidate\":true"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"payload-selection-page-assignment-and-paint-order-unproven\""
    ));
    assert!(layer_tree.contains("\"pageGeometryProven\":false"));
    assert!(layer_tree.contains("\"paintOrderDecoded\":false"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven\""
    ));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("data-ownership-proven=\"true\""));
    assert!(svg.contains("data-frame-reference-row-count=\""));
    assert!(svg.contains("data-frame-coordinate-row-count=\""));
    assert!(svg.contains("data-frame-geometry-candidate-present=\"true\""));
    assert!(svg.contains("data-embedding-frame-trace-present=\"true\""));
    assert!(svg.contains("data-source-frame-record-geometry-present=\"true\""));
    assert!(svg.contains("data-candidate-frame-bbox-present=\"true\""));
    assert!(!svg.contains("data-candidate-frame-x=\"null\""));
    assert!(!svg.contains("data-candidate-frame-y=\"null\""));
    assert!(!svg.contains("data-candidate-frame-width=\"null\""));
    assert!(!svg.contains("data-candidate-frame-height=\"null\""));
    assert!(svg.contains("data-payload-frame-aspect-fit-present=\"true\""));
    assert!(svg.contains("data-payload-frame-aspect-delta-permille=\"899\""));
    assert!(svg.contains("data-best-payload-frame-aspect-delta-permille=\"899\""));
    assert!(svg.contains("data-current-payload-best-frame-aspect-candidate=\"true\""));
    assert!(svg.contains("data-renderable=\"false\""));
    assert!(svg.contains(
        "data-render-promotion-blocked-reason=\"image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven\""
    ));

    let overlay_images = core.get_page_overlay_images(0).unwrap();
    assert!(overlay_images.contains("\"ownershipProven\":true"));
    assert!(overlay_images.contains("\"frameGeometryCandidatePresent\":true"));
    assert!(overlay_images.contains("\"embeddingFrameTracePresent\":true"));
    assert!(overlay_images.contains("\"sourceFrameRecordGeometryPresent\":true"));
    assert!(overlay_images.contains("\"frameRecordGeometry\":{\"sourcePath\":\"/Frame\""));
    assert!(
        overlay_images
            .contains("\"candidateFrameBBox\":{\"source\":\"EmbeddingInfo+/FrameRecord\"")
    );
    assert!(
        overlay_images.contains(
            "\"payloadFrameAspectFit\":{\"source\":\"imagePayloadDimensions+/FrameRecord\""
        )
    );
    assert!(overlay_images.contains("\"currentPayloadBestFrameAspectCandidate\":true"));
    assert!(overlay_images.contains("\"renderable\":false"));
}

#[test]
fn parser_preserves_object_stream_candidates_as_model_evidence() {
    let image_stream_path = "/EmbedItems/Embedding 3/Contents";
    let jpeg_payload = minimal_jpeg_payload();
    let (mut image_payload, signature_offset, payload_end) =
        image_payload_with_header_fixture(jpeg_payload.len());
    image_payload.extend_from_slice(jpeg_payload);
    image_payload.extend_from_slice(b"tail");
    let so_offset = image_payload.len();
    image_payload.extend_from_slice(b"SO\0\0");
    let svg_payload = b"<svg viewBox=\"0 0 10 10\"></svg>".to_vec();
    let figure_reference_payload = b"\x03\0\0\0ref\0\x03".to_vec();
    let mut jsfart_payload = Vec::new();
    for code_unit in "JSFART.OBJECT".encode_utf16() {
        jsfart_payload.extend_from_slice(&code_unit.to_le_bytes());
    }
    jsfart_payload.extend_from_slice(&[0x11, 0x22, 0x33, 0x44]);
    let frame_suffix_row = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
    ];
    let mut frame_payload = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00];
    frame_payload.extend_from_slice(&frame_suffix_row);
    frame_payload.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    frame_payload.extend_from_slice(&frame_suffix_row);
    let figure_link_payload = [
        0x00, 0x0b, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x16, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04,
        0x00, 0x16, 0x00, 0x00, 0x00, 0x08,
    ];
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (image_stream_path, &image_payload),
        ("/FigureData/main_data/FDMVector", &figure_reference_payload),
        (
            "/FigureData/ExpandData/main_data/Link",
            &figure_link_payload,
        ),
        ("/Frame", &frame_payload),
        ("/Vector.svg", &svg_payload),
        ("/VisualList", b"BMDV visual payload"),
        ("/EmbedItems/Embedding 1/JSFart2Contents", &jsfart_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.object_stream_candidates().len(), 7);
    let image_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == image_stream_path)
        .unwrap();
    assert_eq!(image_candidate.size(), image_payload.len());
    assert!(
        image_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::ObjectPath)
    );
    assert!(
        image_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::ImageSignature)
    );
    assert!(
        image_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::SoMarker)
    );
    let ownership = image_candidate.ownership_candidate().unwrap();
    assert_eq!(ownership.basis(), "stream-path");
    assert_eq!(ownership.family(), "embed-items");
    assert_eq!(ownership.storage_path(), Some("/EmbedItems/Embedding 3"));
    assert_eq!(ownership.embedding_index(), Some(3));
    assert_eq!(ownership.stream_role(), "contents");
    assert_eq!(image_candidate.image_signature_hits()[0].kind(), "jpeg");
    assert_eq!(
        image_candidate.image_signature_hits()[0].offset(),
        signature_offset
    );
    assert_eq!(image_candidate.image_payload_spans().len(), 1);
    let image_span = &image_candidate.image_payload_spans()[0];
    assert_eq!(image_span.kind(), "jpeg");
    assert_eq!(image_span.mime(), "image/jpeg");
    assert_eq!(image_span.signature_offset(), signature_offset);
    assert_eq!(image_span.start(), signature_offset);
    assert_eq!(image_span.end(), payload_end);
    assert_eq!(image_span.len(), jpeg_payload.len());
    assert!(image_span.complete());
    assert_eq!(
        image_span.dimensions(),
        Some(ObjectImageDimensions::new(32, 16))
    );
    assert_eq!(
        image_span.payload(),
        &image_payload[signature_offset..payload_end]
    );
    assert_eq!(image_span.envelope().header_start(), 0);
    assert_eq!(image_span.envelope().header_end(), signature_offset);
    assert_eq!(
        image_span.envelope().header(),
        &image_payload[..signature_offset]
    );
    assert_eq!(image_span.envelope().trailer_start(), payload_end);
    assert_eq!(image_span.envelope().trailer_end(), image_payload.len());
    assert_eq!(
        image_span.envelope().trailer(),
        &image_payload[payload_end..]
    );
    let declared_length = image_span.envelope().declared_payload_length().unwrap();
    assert_eq!(declared_length.offset(), signature_offset - 4);
    assert_eq!(declared_length.value(), jpeg_payload.len());
    assert_eq!(declared_length.endian(), "le32");
    let header_fields = image_span.envelope().header_fields();
    assert_eq!(header_fields.u16_le_prefix()[0].value(), 9);
    assert_eq!(header_fields.u16_le_prefix()[1].value(), 1);
    assert_eq!(header_fields.u32_le_prefix()[0].value(), 0x0001_0009);
    let source_path = header_fields.source_path_candidate().unwrap();
    assert_eq!(source_path.length_offset(), 16);
    assert_eq!(source_path.declared_length(), b"C:\\TEMP\\A.JPG".len());
    assert_eq!(source_path.bytes_start(), 17);
    assert_eq!(source_path.text_lossy(), "C:\\TEMP\\A.JPG");
    assert_eq!(image_candidate.so_offsets(), &[so_offset]);
    assert_eq!(
        image_candidate.payload_prefix(),
        &image_payload[..image_payload.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
    );
    let references = image_candidate.ownership_reference_candidates();
    assert!(references.iter().any(|reference| {
        reference.target_path() == "/FigureData/main_data/FDMVector"
            && reference.encoding() == "u32-le"
            && reference.total_matches() == 1
            && reference.offsets() == [0]
    }));
    let frame_rows = image_candidate.frame_reference_row_candidates();
    assert_eq!(frame_rows.len(), 2);
    assert_eq!(frame_rows[0].target_path(), "/Frame");
    assert_eq!(frame_rows[0].encoding(), "u16-be");
    assert_eq!(frame_rows[0].stride(), 20);
    assert_eq!(frame_rows[0].field_offset(), 15);
    assert_eq!(frame_rows[0].offset(), 15);
    assert_eq!(frame_rows[0].row_start(), 0);
    assert_eq!(frame_rows[0].family(), "frame-index-tail-window20");
    let suffix_link = frame_rows[0].suffix_link().unwrap();
    assert_eq!(suffix_link.relation(), "same-candidate");
    assert_eq!(
        suffix_link.suffix_family(),
        "frame-index-tail-coordinate-row12"
    );
    assert_eq!(suffix_link.matched_row_start(), 24);
    assert_eq!(suffix_link.matched_row_index(), 2);
    assert_eq!(frame_rows[1].stride(), 12);
    assert_eq!(frame_rows[1].field_offset(), 7);
    assert_eq!(frame_rows[1].family(), "frame-index-tail-coordinate-row12");

    let svg_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/Vector.svg")
        .unwrap();
    assert!(
        svg_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::ShapePath)
    );
    assert!(
        svg_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::SvgSignature)
    );
    assert_eq!(svg_candidate.svg_offsets(), &[0]);

    let visual_list_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/VisualList")
        .unwrap();
    assert!(
        visual_list_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::VisualListPath)
    );
    assert_eq!(visual_list_candidate.payload_prefix(), b"BMDV visual payl");

    let jsfart_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 1/JSFart2Contents")
        .unwrap();
    assert!(
        jsfart_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::ObjectPath)
    );
    assert!(jsfart_candidate.jsfart_art_candidate().is_none());
    let jsfart_profile = jsfart_candidate
        .jsfart_stream_profile_candidate()
        .expect("non-MSTUDIO JSFart2Contents should still preserve a source profile");
    assert_eq!(jsfart_profile.magic_family(), "jsfart-object-utf16le");
    assert_eq!(jsfart_profile.magic_family_hex(), "4a00");
    assert_eq!(jsfart_profile.magic_offset(), 0);
    assert_eq!(jsfart_profile.magic_ascii_or_utf16_preview(), "JSFART.O");
    assert_eq!(
        jsfart_profile.header_prefix(),
        &jsfart_payload[..jsfart_payload.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
    );
    assert!(!jsfart_profile.structured_art_candidate_present());
    assert_eq!(
        jsfart_profile.render_promotion_blocked_reason(),
        "jsfart-variant-layout-undecoded"
    );
    let object_json = object_stream_candidates_json(document.object_stream_candidates());
    assert!(object_json.contains("\"jsfartStreamProfile\":{\"format\":\"JSFart2Contents\""));
    assert!(object_json.contains("\"magicFamily\":\"jsfart-object-utf16le\""));
    assert!(object_json.contains("\"magicFamilyHex\":\"4a00\""));
    assert!(object_json.contains("\"magicAsciiOrUtf16Preview\":\"JSFART.O\""));
    assert!(object_json.contains("\"structuredArtCandidatePresent\":false"));
    assert!(
        object_json
            .contains("\"renderPromotionBlockedReason\":\"jsfart-variant-layout-undecoded\"")
    );

    let link_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/ExpandData/main_data/Link")
        .unwrap();
    assert!(
        link_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::FigureLink)
    );
    let link = link_candidate.figure_link_candidate().unwrap();
    assert_eq!(link.header_words_be(), &[0x000b, 0x0001, 0x0000, 0x0002]);
    assert_eq!(link.declared_row_count_candidate(), Some(2));
    assert_eq!(link.row_stride(), 14);
    assert_eq!(link.rows().len(), 2);
    assert_eq!(link.rows()[0].row_index(), 0);
    assert_eq!(link.rows()[0].row_start(), 8);
    assert_eq!(
        link.rows()[0].words_be(),
        &[0x0000, 0x0001, 0x0000, 0x0003, 0x0016, 0x0000, 0x0007]
    );
    assert_eq!(link.rows()[0].group_index_candidate(), Some(1));
    assert_eq!(link.rows()[0].source_id_candidate(), Some(3));
    assert_eq!(link.rows()[0].relation_kind_candidate(), Some(0x0016));
    assert_eq!(link.rows()[0].target_row_index_candidate(), Some(7));
}

#[test]
fn parser_decodes_bmdv_visual_list_metadata_and_projects_raster_layer() {
    let visual_list = visual_list_bmdv_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/VisualList", &visual_list),
    ]);

    let core = DocumentCore::from_bytes(&bytes).unwrap();
    let candidate = core
        .document()
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/VisualList")
        .unwrap();
    let visual_list = candidate.visual_list_candidate().unwrap();

    assert_eq!(visual_list.declared_size(), 88);
    assert_eq!(visual_list.magic_offset(), 4);
    assert_eq!(visual_list.magic(), "BMDV");
    assert_eq!(visual_list.version(), 1);
    assert_eq!(visual_list.width(), 10);
    assert_eq!(visual_list.height(), 2);
    assert_eq!(visual_list.row_stride(), 10);
    assert_eq!(visual_list.bit_depth(), 8);
    assert_eq!(visual_list.rle_data_offset(), 0x50);
    assert_eq!(visual_list.rle_data_len(), 8);
    assert_eq!(visual_list.pixels().len(), 20);
    assert_eq!(&visual_list.pixels()[..10], &[0x11; 10]);
    assert_eq!(&visual_list.pixels()[10..], &[0x22; 10]);

    let info = core.get_document_info();
    assert!(info.contains("\"visualList\":{\"format\":\"BMDV\""));
    assert!(info.contains("\"declaredSize\":88"));
    assert!(info.contains("\"rleEncoding\":\"bmp-rle8-like\""));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"visualListRasterDiagnostic\""));
    assert!(layer_tree.contains("\"projectionKind\":\"visualListRasterProjection\""));
    assert!(layer_tree.contains("\"sourcePath\":\"/VisualList\""));
    assert!(layer_tree.contains("\"naturalWidth\":10"));
    assert!(layer_tree.contains("\"naturalHeight\":2"));
    assert!(layer_tree.contains("\"placementProven\":true"));
    assert!(layer_tree.contains("\"decoded\":false"));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-visual-list-raster-diagnostic\""));
    assert!(svg.contains("data-source-path=\"/VisualList\""));
    assert!(svg.contains("data-projection=\"rle8-raster\""));
    assert!(svg.contains("data-fallback-projection=\"horizontal-runs\""));
    assert!(svg.contains("class=\"rjtd-visual-list-rle8-raster\""));
    assert!(svg.contains("data-projection=\"visualListRle8RasterImage\""));
    assert!(svg.contains("data-suppressed-dark-foreground=\"false\""));
    assert!(svg.contains("data:image/png;base64,"));
    assert!(svg.contains("data-format=\"BMDV\""));
}

#[test]
fn parser_preserves_embedding_info_frame_candidates_and_projects_diagnostics() {
    let embedding_info = embedding_info_fixture();
    let frame = frame_stream_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (EMBEDDING_INFO_PATH, &embedding_info),
        ("/Frame", &frame),
    ]);

    let document = parse_document(&bytes).unwrap();
    assert_eq!(document.object_embedding_frames().len(), 1);
    let frame = &document.object_embedding_frames()[0];
    assert_eq!(frame.source_path(), EMBEDDING_INFO_PATH);
    assert_eq!(frame.row_index(), 0);
    assert_eq!(frame.row_start(), EMBEDDING_INFO_HEADER_BYTES);
    assert_eq!(frame.embedding_index(), 24);
    assert_eq!(frame.class_name(), "JSFart.Art.2");
    assert_eq!(frame.primary_width(), 13260);
    assert_eq!(frame.primary_height(), 1327);
    assert_eq!(frame.frame_ref(), 1);
    assert_eq!(frame.frame_width(), 13260);
    assert_eq!(frame.frame_height(), 1327);

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"objectEmbeddingFrameCount\":1"));
    assert!(info.contains("\"className\":\"JSFart.Art.2\""));
    assert!(info.contains("\"frameRef\":1"));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert_json_brackets_balanced(&layer_tree);
    assert!(layer_tree.contains("\"type\":\"embeddingFrameDiagnostic\""));
    assert!(layer_tree.contains("\"source\":\"embedItemsEmbeddingInfo+frame\""));
    assert!(layer_tree.contains("\"embeddingIndex\":24"));
    assert!(layer_tree.contains("\"className\":\"JSFart.Art.2\""));
    assert!(layer_tree.contains("\"frameRef\":1"));
    assert!(layer_tree.contains("\"placementProven\":false"));
    assert!(layer_tree.contains("\"renderable\":false"));

    let svg = core.render_page_svg(0).unwrap();
    assert!(!svg.contains("class=\"rjtd-embedding-frame-diagnostic\""));
    assert!(!svg.contains("data-embedding-index=\"24\""));
}

#[test]
fn parser_preserves_embedded_press_snapshot_metadata_as_object_evidence() {
    let snapshot = embedded_press_snapshot_fixture(2590, 460, 3656, 3560);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/EmbedItems/Embedding 4/\x03EmbeddedPress", &snapshot),
    ]);

    let document = parse_document(&bytes).unwrap();
    let candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/\x03EmbeddedPress")
        .expect("EmbeddedPress stream should be preserved as object evidence");
    assert!(
        candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::EmbeddedPressSnapshot)
    );
    let snapshot = candidate
        .embedded_press_snapshot_candidate()
        .expect("JSSnapShot32 metadata should be decoded into the model");
    assert_eq!(snapshot.magic(), "JSSnapShot32");
    assert_eq!(snapshot.format_marker(), "GCI");
    assert_eq!(snapshot.body_length_candidate(), 3656);
    assert_eq!(snapshot.object_count_candidate(), 17);
    assert_eq!(snapshot.object_table_offset_candidate(), 74);
    assert_eq!(snapshot.payload_length_candidate(), 3560);
    assert_eq!(snapshot.width(), 2590);
    assert_eq!(snapshot.height(), 460);

    let info = DocumentCore::from_document(document).get_document_info();
    assert!(info.contains("\"embeddedPressSnapshot\":{\"format\":\"JSSnapShot32\""));
    assert!(info.contains("\"width\":2590"));
    assert!(info.contains("\"height\":460"));
    assert!(info.contains("\"renderable\":false"));
}

#[test]
fn parser_links_fdm_index_rows_to_fdm_vector_segments() {
    let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    index_payload[18..20].copy_from_slice(&2u16.to_be_bytes());
    push_fdm_index_row(&mut index_payload, 0, 0x1001, (-1, 2, 3, 4));
    push_fdm_index_row(&mut index_payload, 32, 0x2002, (-10, -20, 30, 40));

    let mut vector_payload = vec![0xaa; 32];
    vector_payload.extend_from_slice(b"lead");
    let image_offset = vector_payload.len();
    vector_payload.extend_from_slice(minimal_jpeg_payload());
    let vector_len = vector_payload.len();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/FigureData/main_data/FDMIndex", &index_payload),
        ("/FigureData/main_data/FDMVector", &vector_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    let vector_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
        .unwrap();
    assert_eq!(vector_candidate.fdm_index_entry_candidates().len(), 2);

    let first = &vector_candidate.fdm_index_entry_candidates()[0];
    assert_eq!(first.index_path(), "/FigureData/main_data/FDMIndex");
    assert_eq!(first.vector_path(), "/FigureData/main_data/FDMVector");
    assert_eq!(first.row_index(), 0);
    assert_eq!(first.index_offset(), FDM_INDEX_HEADER_BYTES);
    assert_eq!(first.vector_offset(), 0);
    assert_eq!(first.next_vector_offset(), 32);
    assert_eq!(first.vector_len(), 32);
    assert_eq!(first.kind(), 0x1001);
    assert_eq!(first.bbox(), ObjectFdmIndexBbox::new(-1, 2, 3, 4));
    assert!(first.valid_vector_offset());
    assert!(first.image_signature_hits().is_empty());
    assert!(first.segment_image_signature_hits().is_empty());

    let second = &vector_candidate.fdm_index_entry_candidates()[1];
    assert_eq!(second.row_index(), 1);
    assert_eq!(
        second.index_offset(),
        FDM_INDEX_HEADER_BYTES + FDM_INDEX_ENTRY_BYTES
    );
    assert_eq!(second.vector_offset(), 32);
    assert_eq!(second.next_vector_offset(), vector_len);
    assert_eq!(second.vector_len(), vector_len - 32);
    assert_eq!(second.kind(), 0x2002);
    assert_eq!(second.bbox(), ObjectFdmIndexBbox::new(-10, -20, 30, 40));
    assert!(second.valid_vector_offset());
    assert!(second.vector_prefix().starts_with(b"lead\xff\xd8\xff"));
    assert_eq!(second.image_signature_hits()[0].kind(), "jpeg");
    assert_eq!(second.image_signature_hits()[0].offset(), image_offset);
    assert_eq!(second.segment_image_signature_hits()[0].kind(), "jpeg");
    assert_eq!(second.segment_image_signature_hits()[0].offset(), 4);

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"fdmIndexEntries\":["));
    assert!(info.contains("\"indexPath\":\"/FigureData/main_data/FDMIndex\""));
    assert!(info.contains("\"kindHex\":\"0x2002\""));
    assert!(info.contains("\"bbox\":{\"left\":-10,\"top\":-20,\"right\":30,\"bottom\":40}"));
    assert!(info.contains("\"segmentImageSignatures\":[{\"kind\":\"jpeg\",\"offset\":4}]"));

    let overlay_images = core.get_page_overlay_images(0).unwrap();
    assert!(overlay_images.contains("\"imageCount\":0"));
    assert!(overlay_images.contains("\"unplacedDiagnostics\":["));
    assert!(overlay_images.contains("\"type\":\"jtdFdmVectorImageCandidate\""));
    assert!(overlay_images.contains("\"sourcePath\":\"/FigureData/main_data/FDMVector\""));
    assert!(overlay_images.contains("\"indexPath\":\"/FigureData/main_data/FDMIndex\""));
    assert!(overlay_images.contains("\"rowIndex\":1"));
    assert!(
        overlay_images
            .contains("\"normalizedBbox\":{\"left\":-10,\"top\":-20,\"right\":30,\"bottom\":40}")
    );
    assert!(overlay_images.contains("\"bboxPlausible\":true"));
    assert!(overlay_images.contains("\"completePayloads\":1"));
    assert!(overlay_images.contains("\"placementProven\":false"));
    assert!(overlay_images.contains("\"renderable\":false"));
}

#[test]
fn parser_links_root_fdm_index_to_nested_fdm_vector_by_content_score() {
    let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    index_payload[18..20].copy_from_slice(&2u16.to_be_bytes());
    push_fdm_index_row(&mut index_payload, 0, 0x1001, (1, 2, 3, 4));
    push_fdm_index_row(&mut index_payload, 32, 0x2002, (10, 20, 30, 40));

    let mut decoy_index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    decoy_index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    decoy_index_payload[18..20].copy_from_slice(&1u16.to_be_bytes());
    push_fdm_index_row(
        &mut decoy_index_payload,
        0xffff_fff0,
        0x9999,
        (-1, -2, -3, -4),
    );

    let mut vector_payload = vec![0xaa; 32];
    vector_payload.extend_from_slice(b"lead");
    vector_payload.extend_from_slice(minimal_jpeg_payload());
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/FDMIndex", &index_payload),
        ("/FigureData/other/FDMIndex", &decoy_index_payload),
        ("/FigureData/main_data/FDMVector", &vector_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    let vector_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
        .unwrap();
    assert_eq!(vector_candidate.fdm_index_entry_candidates().len(), 2);
    let first = &vector_candidate.fdm_index_entry_candidates()[0];
    assert_eq!(first.index_path(), "/FDMIndex");
    assert_eq!(first.vector_path(), "/FigureData/main_data/FDMVector");
    assert_eq!(first.vector_offset(), 0);
    assert_eq!(first.kind(), 0x1001);
    assert!(first.valid_vector_offset());
    let second = &vector_candidate.fdm_index_entry_candidates()[1];
    assert_eq!(second.index_path(), "/FDMIndex");
    assert_eq!(second.kind(), 0x2002);
    assert!(second.vector_prefix().starts_with(b"lead\xff\xd8\xff"));

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"indexPath\":\"/FDMIndex\""));
    assert!(info.contains("\"vectorPath\":\"/FigureData/main_data/FDMVector\""));
    assert!(!info.contains("\"indexPath\":\"/FigureData/other/FDMIndex\""));
}

#[test]
fn fdm_connector_line_rule_endpoint_matches_horizontal_tight_span() {
    let projection = shanai_lan_line_rule_projection_fixture(vec![shanai_lan_line_rule_fixture(
        "horizontal",
        21,
        21,
        92,
        156,
    )]);
    let point = FdmConnectorTextGridPoint {
        x_units: 120.0,
        group_index_float: 21.75,
    };

    let matches = fdm_connector_line_rule_endpoint_matches(&projection, point);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].0, 0);
    assert_eq!(matches[0].3, "tight");
    assert!((matches[0].2.axis_delta - 0.75).abs() < 0.001);
    assert_eq!(matches[0].2.inline_delta, 0.0);
}

#[test]
fn fdm_compound_child_layout_requires_ordered_non_overlapping_declared_records() {
    let mut record = vec![0_u8; 80];
    record[..4].copy_from_slice(FDM_VECTOR_COMMAND_BBOX_MARKER);
    record[4..6].copy_from_slice(&80_u16.to_be_bytes());
    record[36..40].copy_from_slice(&0_u32.to_be_bytes());
    record[40..44].copy_from_slice(&0_u32.to_be_bytes());
    record[44..46].copy_from_slice(&48_u16.to_be_bytes());
    record[46..48].copy_from_slice(&64_u16.to_be_bytes());
    record[48..52].copy_from_slice(FDM_VECTOR_COMMAND_NESTED_LINE_MARKER);
    record[52..54].copy_from_slice(&16_u16.to_be_bytes());
    record[64..68].copy_from_slice(FDM_VECTOR_COMMAND_NESTED_LINE_MARKER);
    record[68..70].copy_from_slice(&16_u16.to_be_bytes());

    let layout = fdm_vector_compound_child_layout(&record).expect("valid compound layout");

    assert_eq!(layout.child_offsets(), &[48, 64]);
    assert!(layout.first_child_matches_prefix_end());
    assert!(layout.child_offsets_strictly_increasing());
    assert!(layout.child_records_fit_parent());
    assert!(layout.child_records_do_not_overlap());

    record[46..48].copy_from_slice(&60_u16.to_be_bytes());
    record[60..64].copy_from_slice(FDM_VECTOR_COMMAND_NESTED_LINE_MARKER);
    record[64..66].copy_from_slice(&16_u16.to_be_bytes());
    let overlapping_layout =
        fdm_vector_compound_child_layout(&record).expect("overlapping child table");

    assert!(!overlapping_layout.child_records_do_not_overlap());
    assert!(!overlapping_layout.is_valid_for_nested_projection());
}

#[test]
fn fdm_connector_line_rule_endpoint_matches_vertical_nearby_span() {
    let projection = shanai_lan_line_rule_projection_fixture(vec![shanai_lan_line_rule_fixture(
        "vertical", 10, 15, 84, 84,
    )]);
    let point = FdmConnectorTextGridPoint {
        x_units: 85.5,
        group_index_float: 12.25,
    };

    let matches = fdm_connector_line_rule_endpoint_matches(&projection, point);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].0, 0);
    assert_eq!(matches[0].3, "nearby");
    assert!((matches[0].2.axis_delta - 1.5).abs() < 0.001);
    assert_eq!(matches[0].2.inline_delta, 0.0);
}

#[test]
fn fdm_connector_line_rule_endpoint_matches_rejects_distant_points() {
    let projection = shanai_lan_line_rule_projection_fixture(vec![shanai_lan_line_rule_fixture(
        "horizontal",
        21,
        21,
        92,
        156,
    )]);

    let row_distant = FdmConnectorTextGridPoint {
        x_units: 120.0,
        group_index_float: 23.25,
    };
    let span_distant = FdmConnectorTextGridPoint {
        x_units: 159.5,
        group_index_float: 21.0,
    };

    assert!(fdm_connector_line_rule_endpoint_matches(&projection, row_distant).is_empty());
    assert!(fdm_connector_line_rule_endpoint_matches(&projection, span_distant).is_empty());
}

#[test]
fn fdm_connector_line_rule_endpoint_match_summary_blocks_single_endpoint() {
    let summary = FdmConnectorLineRuleEndpointMatchSummary {
        start_match_count: 1,
        end_match_count: 0,
        total_match_count: 1,
        tight_match_count: 1,
    };

    assert_eq!(summary.matched_endpoint_count(), 1);
    assert!(!summary.dual_endpoint_match());
    assert_eq!(
        summary.graph_promotion_blocked_reason(),
        "single-or-missing-endpoint-line-rule-match"
    );

    let none = FdmConnectorLineRuleEndpointMatchSummary {
        start_match_count: 0,
        end_match_count: 0,
        total_match_count: 0,
        tight_match_count: 0,
    };
    assert_eq!(
        none.graph_promotion_blocked_reason(),
        "no-thresholded-line-rule-endpoint-match"
    );

    let graph = FdmConnectorGraphDiagnosticSummary {
        connector_candidate_count: 67,
        line_rule_projection_count: 16,
        connector_endpoint_probe_count: 134,
        total_thresholded_endpoint_match_count: 10,
        matched_connector_count: 9,
        dual_endpoint_match_connector_count: 0,
        tight_endpoint_match_count: 2,
        nearby_endpoint_match_count: 8,
        no_thresholded_line_rule_endpoint_match_connector_count: 58,
        single_or_missing_endpoint_line_rule_match_connector_count: 9,
        connector_ownership_and_paint_order_unproven_connector_count: 0,
        ..Default::default()
    };
    assert_eq!(
        graph.render_promotion_blocked_reason(),
        "no-dual-endpoint-line-rule-match"
    );

    let graph_with_dual = FdmConnectorGraphDiagnosticSummary {
        dual_endpoint_match_connector_count: 1,
        ..graph
    };
    assert_eq!(
        graph_with_dual.render_promotion_blocked_reason(),
        "connector-ownership-grouping-and-paint-order-unproven"
    );

    let graph_with_axis_rule_dual = FdmConnectorGraphDiagnosticSummary {
        fdm_open_stroke_axis_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary {
            dual_endpoint_match_connector_count: 21,
            ..Default::default()
        },
        ..graph
    };
    assert_eq!(
        graph_with_axis_rule_dual.render_promotion_blocked_reason(),
        "same-row-axis-rule-parent-normalized-order-unproven"
    );

    let graph_with_axis_rule_owner_gate = FdmConnectorGraphDiagnosticSummary {
        fdm_open_stroke_axis_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary {
            dual_endpoint_match_connector_count: 21,
            ..Default::default()
        },
        fdm_open_stroke_axis_rule_owner_promotion_gate_summary:
            FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
                dual_endpoint_match_connector_count: 21,
                dual_endpoint_owner_candidate_count: 21,
                nearest_fdm_owner_rows_match_count: 21,
                nearest_fdm_owner_row_matches_connector_row_count: 21,
                after_owner_parent_command_span_count: 20,
                after_owner_parent_relative_offset_span_count: 20,
                before_owner_parent_command_span_count: 1,
                before_owner_parent_relative_offset_span_count: 1,
                ..Default::default()
            },
        ..graph
    };
    assert_eq!(
        graph_with_axis_rule_owner_gate.render_promotion_blocked_reason(),
        "connector-parent-command-outside-nearest-owner-parent-command-span"
    );
}

#[test]
fn fdm_connector_parent_normalized_order_requires_parent_relative_offset_between_nearest_owner_parents()
 {
    let mut summary = FdmConnectorEndpointOwnerMatchSummary {
        start_within_probe_count: 1,
        end_within_probe_count: 1,
        nearest_fdm_owner_rows_match: true,
        nearest_fdm_owner_row_matches_connector_row: true,
        connector_parent_command_between_nearest_fdm_owner_parent_commands: true,
        connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets: false,
        ..Default::default()
    };

    assert!(!summary.parent_normalized_ordered_same_row_same_connector());

    summary.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets = true;

    assert!(summary.parent_normalized_ordered_same_row_same_connector());

    let gate = FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
        dual_endpoint_match_connector_count: 1,
        dual_endpoint_owner_candidate_count: 1,
        nearest_fdm_owner_rows_match_count: 1,
        nearest_fdm_owner_row_matches_connector_row_count: 1,
        between_owner_parent_command_span_count: 1,
        between_owner_parent_relative_offset_span_count: 0,
        parent_normalized_ordered_same_row_same_connector_count: 0,
        ..Default::default()
    };
    assert_eq!(
        gate.parent_normalized_order_gate_blocked_reason(),
        "connector-parent-relative-offset-outside-nearest-owner-parent-relative-offset-span"
    );

    let partial_gate = FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_owner_candidate_count: 2,
        nearest_fdm_owner_rows_match_count: 2,
        nearest_fdm_owner_row_matches_connector_row_count: 2,
        between_owner_parent_command_span_count: 1,
        between_owner_parent_relative_offset_span_count: 1,
        parent_normalized_ordered_same_row_same_connector_count: 1,
        after_owner_parent_command_span_count: 1,
        after_owner_parent_relative_offset_span_count: 1,
        ..Default::default()
    };
    assert_eq!(
        partial_gate.parent_normalized_order_gate_blocked_reason(),
        "connector-parent-command-outside-nearest-owner-parent-command-span"
    );

    let axis_disagreement_gate = FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_owner_candidate_count: 2,
        nearest_fdm_owner_rows_match_count: 2,
        nearest_fdm_owner_row_matches_connector_row_count: 2,
        between_owner_parent_command_span_count: 2,
        between_owner_parent_relative_offset_span_count: 2,
        parent_normalized_ordered_same_row_same_connector_count: 1,
        ..Default::default()
    };
    assert_eq!(
        axis_disagreement_gate.parent_normalized_order_gate_blocked_reason(),
        "parent-command-source-order-axis-disagreement"
    );
}

#[test]
fn fdm_axis_rule_source_order_gate_classifies_parent_relative_offset_spans() {
    let no_dual = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary::default();
    assert_eq!(
        no_dual.axis_rule_source_order_gate_blocked_reason(),
        "no-same-row-axis-rule-dual-endpoint-match"
    );

    let missing = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_axis_rule_source_order_backed_connector_count: 1,
        ..Default::default()
    };
    assert_eq!(
        missing.axis_rule_source_order_gate_blocked_reason(),
        "axis-rule-source-order-evidence-missing"
    );

    let before = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_axis_rule_source_order_backed_connector_count: 2,
        dual_endpoint_connector_before_axis_rule_parent_span_count: 2,
        ..Default::default()
    };
    assert_eq!(
        before.axis_rule_source_order_gate_blocked_reason(),
        "connector-before-axis-rule-parent-span-paint-order-unproven"
    );

    let between = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_axis_rule_source_order_backed_connector_count: 2,
        dual_endpoint_connector_between_axis_rule_parent_span_count: 2,
        ..Default::default()
    };
    assert_eq!(
        between.axis_rule_source_order_gate_blocked_reason(),
        "connector-between-axis-rule-parent-span-paint-order-unproven"
    );

    let mixed = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_axis_rule_source_order_backed_connector_count: 2,
        dual_endpoint_connector_before_axis_rule_parent_span_count: 1,
        dual_endpoint_connector_between_axis_rule_parent_span_count: 1,
        ..Default::default()
    };
    assert_eq!(
        mixed.axis_rule_source_order_gate_blocked_reason(),
        "mixed-connector-axis-rule-parent-span-paint-order-unproven"
    );
}

#[test]
fn parser_preserves_frame_records_for_fdm_link_diagnostics() {
    let mut frame_payload = vec![
        0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x01, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x02,
    ];
    frame_payload.extend_from_slice(&frame_record_fixture(0, 0x0004, (11, 22, 33, 44)));
    frame_payload.extend_from_slice(&frame_record_fixture(1, 0x0007, (100, 200, 300, 400)));
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/Frame", &frame_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.object_frame_records().len(), 2);
    let record = &document.object_frame_records()[1];
    assert_eq!(record.source_path(), "/Frame");
    assert_eq!(record.row_index(), 1);
    assert_eq!(record.row_start(), 76);
    assert_eq!(record.record_len(), 60);
    assert_eq!(record.record_kind(), 0x0102);
    assert_eq!(record.declared_record_bytes(), 0x0038);
    assert_eq!(record.object_id(), 1);
    assert_eq!(record.object_type(), 0x0007);
    assert_eq!(record.x(), 100);
    assert_eq!(record.y(), 200);
    assert_eq!(record.width(), 300);
    assert_eq!(record.height(), 400);
    assert_eq!(record.corner_radius(), 0);
    assert_eq!(record.style_id(), 0);

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"objectFrameRecordCount\":2"));
    assert!(info.contains("\"objectFrameRecords\":["));
    assert!(info.contains("\"sourcePath\":\"/Frame\""));
    assert!(info.contains("\"rowIndex\":1"));
    assert!(info.contains("\"rowStart\":76"));
    assert!(info.contains("\"recordKindHex\":\"0x0102\""));
    assert!(info.contains("\"objectTypeHex\":\"0x0007\""));
    assert!(info.contains(
        "\"geometry\":{\"x\":100,\"y\":200,\"width\":300,\"height\":400,\"cornerRadius\":0}"
    ));
    assert!(info.contains("\"styleId\":0"));
}

#[test]
fn parser_limits_fdm_index_entries_to_declared_prefix_rows() {
    let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    index_payload[18..20].copy_from_slice(&1u16.to_be_bytes());
    push_fdm_index_row(&mut index_payload, 32, 0x0b00, (1, 2, 3, 4));
    push_fdm_index_row(&mut index_payload, 0xffff_fff0, 0xffff, (-1, -2, -3, -4));

    let mut vector_payload = vec![0xaa; 32];
    vector_payload.extend_from_slice(b"lead");
    let image_offset = vector_payload.len();
    vector_payload.extend_from_slice(b"\xff\xd8\xffpayload\xff\xd9");
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/FigureData/main_data/FDMIndex", &index_payload),
        ("/FigureData/main_data/FDMVector", &vector_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    let vector_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
        .unwrap();
    assert_eq!(vector_candidate.fdm_index_entry_candidates().len(), 1);
    let entry = &vector_candidate.fdm_index_entry_candidates()[0];
    assert_eq!(entry.row_index(), 0);
    assert_eq!(entry.vector_offset(), 32);
    assert_eq!(entry.kind(), 0x0b00);
    assert_eq!(entry.image_signature_hits()[0].offset(), image_offset);
    assert_eq!(entry.segment_image_signature_hits()[0].offset(), 4);
}

#[test]
fn document_core_reports_object_stream_candidates_as_diagnostics() {
    let image_stream_path = "/EmbedItems/Embedding 3/Contents";
    let jpeg_payload = minimal_jpeg_payload();
    let (mut image_payload, signature_offset, _) =
        image_payload_with_header_fixture(jpeg_payload.len());
    image_payload.extend_from_slice(jpeg_payload);
    let figure_reference_payload = b"\x03\0\0\0ref\0\x03".to_vec();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (image_stream_path, &image_payload),
        ("/FigureData/main_data/FDMVector", &figure_reference_payload),
    ]);

    let core = DocumentCore::from_bytes(&bytes).unwrap();
    let info = core.get_document_info();
    let warnings = core.get_validation_warnings();

    assert!(info.contains("\"objectStreamCandidateCount\":2"));
    assert!(info.contains("\"path\":\"/EmbedItems/Embedding 3/Contents\""));
    assert!(info.contains("\"ownershipCandidate\":{\"basis\":\"stream-path\",\"family\":\"embed-items\",\"storagePath\":\"/EmbedItems/Embedding 3\",\"embeddingIndex\":3,\"streamRole\":\"contents\",\"decoded\":false}"));
    assert!(info.contains("\"ownershipReferences\":["));
    assert!(info.contains("\"targetPath\":\"/FigureData/main_data/FDMVector\""));
    assert!(info.contains("\"encoding\":\"u32-le\",\"totalMatches\":1,\"offsets\":[0]"));
    assert!(info.contains("\"frameReferenceRows\":[]"));
    assert!(info.contains("\"fdmIndexEntries\":[]"));
    assert!(info.contains(&format!(
        "\"imageSignatures\":[{{\"kind\":\"jpeg\",\"offset\":{signature_offset}}}]"
    )));
    assert!(info.contains(&format!(
        "\"imagePayloads\":[{{\"kind\":\"jpeg\",\"mime\":\"image/jpeg\",\"signatureOffset\":{signature_offset}"
    )));
    assert!(info.contains(&format!("\"declaredPayloadLength\":{}", jpeg_payload.len())));
    assert!(info.contains(&format!(
        "\"declaredPayloadLengthOffset\":{}",
        signature_offset - 4
    )));
    assert!(info.contains("\"sourcePathCandidate\""));
    assert!(info.contains("\"textLossy\":\"C:\\\\TEMP\\\\A.JPG\""));
    assert!(warnings.contains("\"JTD object stream candidate preserved as diagnostic data\":2"));
    assert!(warnings.contains("\"kind\":\"JtdObjectStreamCandidateDiagnosticOnly\""));
}

pub(super) fn visual_list_bmdv_fixture() -> Vec<u8> {
    let rle = [0x0a, 0x11, 0x00, 0x00, 0x0a, 0x22, 0x00, 0x00];
    let mut bytes = vec![0; VISUAL_LIST_HEADER_BYTES];
    let declared_size = VISUAL_LIST_HEADER_BYTES + rle.len();
    bytes[0..4].copy_from_slice(&(declared_size as u32).to_be_bytes());
    bytes[VISUAL_LIST_MAGIC_OFFSET..VISUAL_LIST_MAGIC_OFFSET + VISUAL_LIST_MAGIC.len()]
        .copy_from_slice(VISUAL_LIST_MAGIC);
    bytes[VISUAL_LIST_VERSION_OFFSET..VISUAL_LIST_VERSION_OFFSET + 4]
        .copy_from_slice(&1u32.to_be_bytes());
    bytes[VISUAL_LIST_FLAGS_OFFSET..VISUAL_LIST_FLAGS_OFFSET + 4]
        .copy_from_slice(&0x0001_0100u32.to_be_bytes());
    bytes[VISUAL_LIST_WIDTH_OFFSET..VISUAL_LIST_WIDTH_OFFSET + 4]
        .copy_from_slice(&10u32.to_be_bytes());
    bytes[VISUAL_LIST_HEIGHT_OFFSET..VISUAL_LIST_HEIGHT_OFFSET + 4]
        .copy_from_slice(&2u32.to_be_bytes());
    bytes[VISUAL_LIST_ROW_STRIDE_OFFSET..VISUAL_LIST_ROW_STRIDE_OFFSET + 4]
        .copy_from_slice(&10u32.to_be_bytes());
    bytes[VISUAL_LIST_BIT_DEPTH_OFFSET..VISUAL_LIST_BIT_DEPTH_OFFSET + 4]
        .copy_from_slice(&8u32.to_be_bytes());
    bytes[VISUAL_LIST_X_PPM_OFFSET..VISUAL_LIST_X_PPM_OFFSET + 4]
        .copy_from_slice(&3779u32.to_be_bytes());
    bytes[VISUAL_LIST_Y_PPM_OFFSET..VISUAL_LIST_Y_PPM_OFFSET + 4]
        .copy_from_slice(&3779u32.to_be_bytes());
    bytes[VISUAL_LIST_RLE_LENGTH_OFFSET..VISUAL_LIST_RLE_LENGTH_OFFSET + 4]
        .copy_from_slice(&(rle.len() as u32).to_be_bytes());
    bytes.extend_from_slice(&rle);
    bytes
}

pub(super) fn embedding_info_fixture() -> Vec<u8> {
    let class_name = "JSFart.Art.2";
    let mut class_bytes = Vec::new();
    for unit in class_name.encode_utf16() {
        class_bytes.extend_from_slice(&unit.to_le_bytes());
    }
    class_bytes.extend_from_slice(&0u16.to_le_bytes());

    let mut bytes = vec![0; EMBEDDING_INFO_HEADER_BYTES];
    bytes[0..4].copy_from_slice(&1u32.to_le_bytes());
    let row_start = bytes.len();
    bytes.resize(row_start + EMBEDDING_INFO_CLASS_START_OFFSET, 0);
    bytes[row_start + EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET
        ..row_start + EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET + 4]
        .copy_from_slice(&24u32.to_le_bytes());
    bytes[row_start + EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET
        ..row_start + EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET + 2]
        .copy_from_slice(&13260u16.to_le_bytes());
    bytes[row_start + EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET
        ..row_start + EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET + 2]
        .copy_from_slice(&1327u16.to_le_bytes());
    bytes[row_start + EMBEDDING_INFO_CLASS_LENGTH_OFFSET
        ..row_start + EMBEDDING_INFO_CLASS_LENGTH_OFFSET + 4]
        .copy_from_slice(&(class_bytes.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&class_bytes);

    let trailing_start = bytes.len();
    bytes.resize(trailing_start + EMBEDDING_INFO_TRAILING_BYTES, 0);
    bytes[trailing_start + EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET
        ..trailing_start + EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET + 4]
        .copy_from_slice(&1u32.to_le_bytes());
    bytes[trailing_start + EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET
        ..trailing_start + EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET + 4]
        .copy_from_slice(&13260u32.to_le_bytes());
    bytes[trailing_start + EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET
        ..trailing_start + EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET + 4]
        .copy_from_slice(&1327u32.to_le_bytes());
    bytes
}

pub(super) fn embedded_press_snapshot_fixture(
    width: u32,
    height: u32,
    body_length: u32,
    payload_length: u32,
) -> Vec<u8> {
    let mut bytes = vec![0; 0x80];
    bytes[..EMBEDDED_PRESS_SNAPSHOT_MAGIC.len()].copy_from_slice(EMBEDDED_PRESS_SNAPSHOT_MAGIC);
    bytes[0x0c..0x10].copy_from_slice(&[0x00, 0xd5, 0xf6, 0x77]);
    bytes[0x10..0x14].copy_from_slice(&u32::MAX.to_le_bytes());
    bytes[0x20..0x24].copy_from_slice(&32u32.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET
        ..EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET + 4]
        .copy_from_slice(&body_length.to_le_bytes());
    bytes[0x28..0x2c].copy_from_slice(&65536u32.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET..EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET + 4]
        .copy_from_slice(b"GCI\0");
    bytes[EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET
        ..EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET + 4]
        .copy_from_slice(&17u32.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET
        ..EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET + 4]
        .copy_from_slice(&74u32.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET
        ..EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET + 4]
        .copy_from_slice(&payload_length.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET..EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET + 4]
        .copy_from_slice(&width.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET..EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET + 4]
        .copy_from_slice(&height.to_le_bytes());
    bytes[0x50..0x54].copy_from_slice(&100u32.to_le_bytes());
    bytes[0x54..0x58].copy_from_slice(&1u32.to_le_bytes());
    bytes[0x58..0x5c].copy_from_slice(&100u32.to_le_bytes());
    bytes[0x5c..0x60].copy_from_slice(&1u32.to_le_bytes());
    bytes[0x60..0x64].copy_from_slice(&4u32.to_le_bytes());
    bytes
}

pub(super) fn frame_stream_fixture() -> Vec<u8> {
    let mut bytes = vec![0; FRAME_RECORD_HEADER_BYTES];
    bytes[FRAME_RECORD_DECLARED_COUNT_OFFSET..FRAME_RECORD_DECLARED_COUNT_OFFSET + 2]
        .copy_from_slice(&2u16.to_be_bytes());
    bytes.resize(FRAME_RECORD_HEADER_BYTES + FRAME_RECORD_BYTES, 0);

    let row_start = FRAME_RECORD_HEADER_BYTES + FRAME_RECORD_BYTES;
    bytes.resize(row_start + FRAME_RECORD_BYTES, 0);
    bytes[row_start..row_start + 2].copy_from_slice(&0x1001u16.to_be_bytes());
    bytes[row_start + 2..row_start + 4].copy_from_slice(&60u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_ID_OFFSET..row_start + FRAME_RECORD_ID_OFFSET + 2]
        .copy_from_slice(&24u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_TYPE_OFFSET..row_start + FRAME_RECORD_TYPE_OFFSET + 2]
        .copy_from_slice(&0x0002u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_X_OFFSET..row_start + FRAME_RECORD_X_OFFSET + 2]
        .copy_from_slice(&2143u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_Y_OFFSET..row_start + FRAME_RECORD_Y_OFFSET + 2]
        .copy_from_slice(&2932u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_WIDTH_OFFSET..row_start + FRAME_RECORD_WIDTH_OFFSET + 2]
        .copy_from_slice(&13260u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_HEIGHT_OFFSET..row_start + FRAME_RECORD_HEIGHT_OFFSET + 2]
        .copy_from_slice(&1327u16.to_be_bytes());
    bytes
}

pub(super) fn minimal_jpeg_payload() -> &'static [u8] {
    &[
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x10,
        0x00, 0x20, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff, 0xda, 0x00,
        0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x00, 0xff, 0xd9,
    ]
}

#[cfg(feature = "bitmap-images")]
pub(super) fn minimal_png_payload() -> &'static [u8] {
    &[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8,
        0xcf, 0xc0, 0x00, 0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xdd, 0x8d, 0xb0, 0x00, 0x00, 0x00,
        0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ]
}

pub(super) fn image_payload_with_header_fixture(payload_len: usize) -> (Vec<u8>, usize, usize) {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&9_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&0x1234_u32.to_le_bytes());
    bytes.extend_from_slice(&0x5678_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());

    let source_path = b"C:\\TEMP\\A.JPG";
    bytes.push(source_path.len() as u8);
    bytes.extend_from_slice(source_path);
    bytes.push(0);
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.extend_from_slice(&(payload_len as u32).to_le_bytes());

    let signature_offset = bytes.len();
    (bytes, signature_offset, signature_offset + payload_len)
}

pub(super) fn push_fdm_index_row(
    bytes: &mut Vec<u8>,
    vector_offset: u32,
    kind: u16,
    bbox: (i32, i32, i32, i32),
) {
    bytes.extend_from_slice(&vector_offset.to_be_bytes());
    bytes.extend_from_slice(&kind.to_be_bytes());
    bytes.extend_from_slice(&bbox.0.to_be_bytes());
    bytes.extend_from_slice(&bbox.1.to_be_bytes());
    bytes.extend_from_slice(&bbox.2.to_be_bytes());
    bytes.extend_from_slice(&bbox.3.to_be_bytes());
}

pub(super) fn frame_record_fixture(
    object_id: u16,
    object_type: u16,
    geometry: (u16, u16, u16, u16),
) -> Vec<u8> {
    let mut row = vec![0; FRAME_RECORD_BYTES];
    row[0..2].copy_from_slice(&0x0102_u16.to_be_bytes());
    row[2..4].copy_from_slice(&0x0038_u16.to_be_bytes());
    row[FRAME_RECORD_ID_OFFSET..FRAME_RECORD_ID_OFFSET + 2]
        .copy_from_slice(&object_id.to_be_bytes());
    row[FRAME_RECORD_TYPE_OFFSET..FRAME_RECORD_TYPE_OFFSET + 2]
        .copy_from_slice(&object_type.to_be_bytes());
    row[FRAME_RECORD_X_OFFSET..FRAME_RECORD_X_OFFSET + 2]
        .copy_from_slice(&geometry.0.to_be_bytes());
    row[FRAME_RECORD_Y_OFFSET..FRAME_RECORD_Y_OFFSET + 2]
        .copy_from_slice(&geometry.1.to_be_bytes());
    row[FRAME_RECORD_WIDTH_OFFSET..FRAME_RECORD_WIDTH_OFFSET + 2]
        .copy_from_slice(&geometry.2.to_be_bytes());
    row[FRAME_RECORD_HEIGHT_OFFSET..FRAME_RECORD_HEIGHT_OFFSET + 2]
        .copy_from_slice(&geometry.3.to_be_bytes());
    row
}
