use super::*;
use crate::*;

pub(crate) fn fdm_frame_diagnostics(document: &Document) -> Vec<FdmFrameDiagnostic<'_>> {
    if !document_has_shanai_lan_fdm_frame_evidence(document) {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for entry in candidate
            .fdm_index_entry_candidates()
            .iter()
            .filter(|entry| !entry.segment_image_signature_hits().is_empty())
        {
            if let Some(frame_record) = fdm_frame_record_for_entry(document, entry) {
                diagnostics.push(FdmFrameDiagnostic {
                    candidate_index,
                    candidate,
                    entry,
                    frame_record,
                });
            }
        }
    }
    diagnostics
}

pub(crate) fn fdm_frame_record_for_entry<'a>(
    document: &'a Document,
    entry: &ObjectFdmIndexEntryCandidate,
) -> Option<&'a ObjectFrameRecordCandidate> {
    document.object_frame_records().iter().find(|record| {
        usize::from(record.object_id()) == entry.row_index()
            || record.row_index() == entry.row_index()
    })
}

pub(crate) fn fdm_command_diagnostics(document: &Document) -> Vec<FdmCommandDiagnostic<'_>> {
    if !document_has_shanai_lan_fdm_command_evidence(document) {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for entry in candidate.fdm_index_entry_candidates() {
            for command in entry
                .vector_commands()
                .iter()
                .filter(|command| command.bbox().is_some())
            {
                diagnostics.push(FdmCommandDiagnostic {
                    candidate_index,
                    candidate,
                    entry,
                    command,
                });
            }
        }
    }
    diagnostics
}

pub(crate) fn fdm_command_projection_extent(
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> Option<FdmCommandProjectionExtent> {
    let mut iter = diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.command.bbox())
        .map(normalize_fdm_bbox);
    let first = iter.next()?;
    let mut extent = FdmCommandProjectionExtent {
        left: first.0,
        top: first.1,
        right: first.2,
        bottom: first.3,
    };
    for bbox in iter {
        extent.left = extent.left.min(bbox.0);
        extent.top = extent.top.min(bbox.1);
        extent.right = extent.right.max(bbox.2);
        extent.bottom = extent.bottom.max(bbox.3);
    }
    if extent.left >= extent.right || extent.top >= extent.bottom {
        return None;
    }
    Some(extent)
}

pub(crate) fn fdm_vector_primitive_diagnostics(
    document: &Document,
) -> Vec<FdmCommandDiagnostic<'_>> {
    if !document_has_shanai_lan_fdm_command_evidence(document) {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for entry in candidate.fdm_index_entry_candidates() {
            for command in entry.vector_commands().iter().filter(|command| {
                FDM_VECTOR_RENDERED_PRIMITIVE_MARKERS.contains(command.marker())
                    && command.has_renderable_geometry()
            }) {
                diagnostics.push(FdmCommandDiagnostic {
                    candidate_index,
                    candidate,
                    entry,
                    command,
                });
            }
        }
    }
    diagnostics
}

pub(crate) fn fdm_image_overlay_diagnostics_json(document: &Document) -> Vec<String> {
    let mut diagnostics = Vec::new();
    for candidate in document.object_stream_candidates() {
        for entry in candidate
            .fdm_index_entry_candidates()
            .iter()
            .filter(|entry| !entry.segment_image_signature_hits().is_empty())
        {
            let bbox = entry.bbox();
            let normalized = normalize_fdm_bbox(bbox);
            let bbox_width = normalized.2.saturating_sub(normalized.0);
            let bbox_height = normalized.3.saturating_sub(normalized.1);
            let mut output = String::new();
            output.push_str("{\"type\":\"jtdFdmVectorImageCandidate\",\"sourcePath\":");
            output.push_str(&json_string(candidate.path()));
            output.push_str(",\"indexPath\":");
            output.push_str(&json_string(entry.index_path()));
            output.push_str(",\"vectorPath\":");
            output.push_str(&json_string(entry.vector_path()));
            output.push_str(",\"rowIndex\":");
            output.push_str(&entry.row_index().to_string());
            output.push_str(",\"vectorOffset\":");
            output.push_str(&entry.vector_offset().to_string());
            output.push_str(",\"nextVectorOffset\":");
            output.push_str(&entry.next_vector_offset().to_string());
            output.push_str(",\"vectorLength\":");
            output.push_str(&entry.vector_len().to_string());
            output.push_str(",\"kind\":");
            output.push_str(&entry.kind().to_string());
            output.push_str(",\"kindHex\":");
            output.push_str(&json_string(&format!("0x{:04x}", entry.kind())));
            output.push_str(",\"bbox\":");
            push_object_fdm_index_bbox_json(&mut output, bbox);
            output.push_str(",\"normalizedBbox\":");
            push_fdm_normalized_bbox_json(&mut output, normalized);
            output.push_str(",\"bboxWidth\":");
            output.push_str(&bbox_width.to_string());
            output.push_str(",\"bboxHeight\":");
            output.push_str(&bbox_height.to_string());
            output.push_str(",\"bboxOrder\":");
            output.push_str(&json_string(fdm_bbox_order(bbox)));
            output.push_str(",\"bboxPlausible\":");
            output.push_str(if fdm_bbox_is_plausible(bbox) {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"imageSignatures\":");
            push_object_image_signature_hits_json(&mut output, entry.image_signature_hits());
            output.push_str(",\"segmentImageSignatures\":");
            push_object_image_signature_hits_json(
                &mut output,
                entry.segment_image_signature_hits(),
            );
            output.push_str(",\"completePayloads\":");
            output.push_str(&fdm_entry_complete_payload_count(candidate, entry).to_string());
            output.push_str(",\"placementProven\":false,\"renderable\":false,\"reason\":\"page-placement-unproven\",\"decoded\":false}");
            diagnostics.push(output);
        }
    }
    diagnostics
}

pub(crate) fn fdm_entry_complete_payload_count(
    candidate: &ObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> usize {
    candidate
        .image_payload_spans()
        .iter()
        .filter(|span| {
            span.complete()
                && span.signature_offset() >= entry.vector_offset()
                && span.signature_offset() < entry.next_vector_offset()
        })
        .count()
}

pub(crate) fn fdm_entry_image_payload_extraction_status(
    candidate: &ObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> &'static str {
    if entry.image_signature_hits().is_empty() && entry.segment_image_signature_hits().is_empty() {
        "no-image-signature"
    } else if fdm_entry_complete_payload_count(candidate, entry) > 0 {
        "complete-payload-in-fdm-index-segment"
    } else if candidate
        .image_payload_spans()
        .iter()
        .any(|span| span.complete())
    {
        "complete-payload-elsewhere-in-vector-stream"
    } else {
        "signature-without-complete-payload"
    }
}

pub(crate) fn fdm_entry_frame_render_blocked_reason(
    candidate: &ObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> &'static str {
    match fdm_entry_image_payload_extraction_status(candidate, entry) {
        "signature-without-complete-payload" => {
            "image-signature-without-complete-payload-role-unproven"
        }
        "no-image-signature" => "fdm-frame-image-payload-absent",
        "complete-payload-in-fdm-index-segment" => {
            "fdm-frame-linked-image-payload-placement-and-paint-order-unproven"
        }
        _ => "fdm-frame-image-placement-and-paint-order-unproven",
    }
}

pub(crate) fn fdm_index_segment_bbox_axis_pair_gate(
    candidate: &ObjectStreamCandidate,
) -> Option<FdmIndexSegmentBboxAxisPairGate> {
    let valid_index_row_count = candidate
        .fdm_index_entry_candidates()
        .iter()
        .filter(|entry| entry.valid_vector_offset())
        .count();
    if valid_index_row_count == 0 {
        return None;
    }

    let mut linked_row_count = 0usize;
    let mut axis_pair_order_agreement_row_count = 0usize;
    for entry in candidate
        .fdm_index_entry_candidates()
        .iter()
        .filter(|entry| entry.valid_vector_offset())
    {
        let Some(segment_bbox) = candidate
            .fdm_raw_vector_segments()
            .iter()
            .find(|segment| segment.relative_offset() == entry.vector_offset())
            .and_then(ObjectFdmVectorSegmentCandidate::bbox)
        else {
            continue;
        };
        linked_row_count += 1;
        let index_bbox = entry.bbox();
        if index_bbox.left() == segment_bbox.left()
            && index_bbox.top() == segment_bbox.right()
            && index_bbox.right() == segment_bbox.top()
            && index_bbox.bottom() == segment_bbox.bottom()
        {
            axis_pair_order_agreement_row_count += 1;
        }
    }

    (linked_row_count > 0).then_some(FdmIndexSegmentBboxAxisPairGate::new(
        valid_index_row_count,
        linked_row_count,
        axis_pair_order_agreement_row_count,
    ))
}

pub(crate) fn normalize_fdm_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.right()),
        bbox.top().min(bbox.bottom()),
        bbox.left().max(bbox.right()),
        bbox.top().max(bbox.bottom()),
    )
}

pub(crate) fn normalize_fdm_index_entry_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.top()),
        bbox.right().min(bbox.bottom()),
        bbox.left().max(bbox.top()),
        bbox.right().max(bbox.bottom()),
    )
}

pub(crate) fn fdm_bbox_center(bbox: (i32, i32, i32, i32)) -> (i32, i32) {
    let center_x = i64::from(bbox.0) + (i64::from(bbox.2) - i64::from(bbox.0)) / 2;
    let center_y = i64::from(bbox.1) + (i64::from(bbox.3) - i64::from(bbox.1)) / 2;
    (center_x as i32, center_y as i32)
}

pub(crate) fn fdm_bbox_extent_union(
    current: Option<(i32, i32, i32, i32)>,
    next: (i32, i32, i32, i32),
) -> Option<(i32, i32, i32, i32)> {
    match current {
        Some((left, top, right, bottom)) => Some((
            left.min(next.0),
            top.min(next.1),
            right.max(next.2),
            bottom.max(next.3),
        )),
        None => Some(next),
    }
}

pub(crate) fn push_fdm_normalized_bbox_json(output: &mut String, bbox: (i32, i32, i32, i32)) {
    output.push_str("{\"left\":");
    output.push_str(&bbox.0.to_string());
    output.push_str(",\"top\":");
    output.push_str(&bbox.1.to_string());
    output.push_str(",\"right\":");
    output.push_str(&bbox.2.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&bbox.3.to_string());
    output.push('}');
}

pub(crate) fn fdm_bbox_order(bbox: ObjectFdmIndexBbox) -> &'static str {
    match (bbox.left() <= bbox.right(), bbox.top() <= bbox.bottom()) {
        (true, true) => "forward",
        (false, true) => "inverted-x",
        (true, false) => "inverted-y",
        (false, false) => "inverted-xy",
    }
}

pub(crate) fn fdm_bbox_is_plausible(bbox: ObjectFdmIndexBbox) -> bool {
    let normalized = normalize_fdm_bbox(bbox);
    let width = normalized.2.saturating_sub(normalized.0);
    let height = normalized.3.saturating_sub(normalized.1);
    width > 0 && height > 0 && width <= 200_000 && height <= 200_000
}

pub(crate) fn push_answer_sheet_fdm_text_geometry_evidence_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    let text_candidates = candidate.fdm_text_candidates();
    let index_entries = candidate.fdm_text_index_entry_candidates();
    let indexed_text_count = index_entries.len();
    let bbox_extent = text_candidates
        .iter()
        .filter_map(|candidate| candidate.bbox().map(normalize_fdm_bbox))
        .fold(None, fdm_bbox_extent_union);
    let index_bbox_extent = index_entries
        .iter()
        .map(|entry| normalize_fdm_bbox(entry.bbox()))
        .fold(None, fdm_bbox_extent_union);

    output.push_str("{\"source\":\"FDMText\",\"sourcePath\":");
    output.push_str(&json_string(candidate.path()));
    output.push_str(",\"textCount\":");
    output.push_str(&text_candidates.len().to_string());
    output.push_str(",\"indexedTextCount\":");
    output.push_str(&indexed_text_count.to_string());
    output.push_str(",\"bboxExtent\":");
    match bbox_extent {
        Some((left, top, right, bottom)) => {
            output.push_str(&format!(
                "{{\"left\":{left},\"top\":{top},\"right\":{right},\"bottom\":{bottom},\"width\":{},\"height\":{}}}",
                right - left,
                bottom - top
            ));
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"labels\":[");
    for (index, text) in text_candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&index.to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(text.text()));
        output.push_str(",\"markerOffset\":");
        output.push_str(&text.marker_offset().to_string());
        output.push_str(",\"textOffset\":");
        output.push_str(&text.text_offset().to_string());
        output.push_str(",\"bbox\":");
        match text.bbox().map(normalize_fdm_bbox) {
            Some((left, top, right, bottom)) => {
                output.push_str(&format!(
                    "{{\"left\":{left},\"top\":{top},\"right\":{right},\"bottom\":{bottom},\"width\":{},\"height\":{}}}",
                    right - left,
                    bottom - top
                ));
            }
            None => output.push_str("null"),
        }
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"textIndexEntries\":[");
    for (index, entry) in index_entries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_answer_sheet_fdm_text_index_entry_json(output, entry);
    }
    output.push_str("],\"indexBboxExtent\":");
    match index_bbox_extent {
        Some((left, top, right, bottom)) => {
            output.push_str(&format!(
                "{{\"left\":{left},\"top\":{top},\"right\":{right},\"bottom\":{bottom},\"width\":{},\"height\":{}}}",
                right - left,
                bottom - top
            ));
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"triangleSourceBboxCandidate\":");
    if let Some(bbox) = success_data_test_answer_sheet_triangle_source_bbox(candidate) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"geometryDecoded\":true,\"placementDecoded\":false,\"decoded\":false}");
}

pub(crate) fn push_answer_sheet_fdm_text_index_entry_json(
    output: &mut String,
    entry: &ObjectFdmTextIndexEntryCandidate,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&entry.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&entry.index_offset().to_string());
    output.push_str(",\"textRecordOffset\":");
    output.push_str(&entry.text_record_offset().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&entry.kind().to_string());
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", entry.kind())));
    output.push_str(",\"bbox\":");
    push_object_fdm_index_bbox_json(output, entry.bbox());
    output.push_str(",\"textRecordBbox\":");
    if let Some(bbox) = entry.text_record_bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_answer_sheet_figure_link_evidence_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    let Some(link) = candidate.figure_link_candidate() else {
        output.push_str("null");
        return;
    };

    let mut relation_kinds = BTreeSet::new();
    for row in link.rows() {
        if let Some(kind) = row.relation_kind_candidate() {
            relation_kinds.insert(kind);
        }
    }

    output.push_str("{\"source\":\"figureLink\",\"sourcePath\":");
    output.push_str(&json_string(candidate.path()));
    output.push_str(",\"declaredRowCountCandidate\":");
    push_option_u16_json(output, link.declared_row_count_candidate());
    output.push_str(",\"rowStride\":");
    output.push_str(&link.row_stride().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&link.rows().len().to_string());
    output.push_str(",\"relationKinds\":[");
    for (index, kind) in relation_kinds.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&kind.to_string());
        output.push_str(",\"kindHex\":");
        output.push_str(&json_string(&format!("0x{kind:04x}")));
        output.push('}');
    }
    output.push_str("],\"rows\":[");
    for (index, row) in link.rows().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&row.row_index().to_string());
        output.push_str(",\"rowStart\":");
        output.push_str(&row.row_start().to_string());
        output.push_str(",\"sourceIdCandidate\":");
        push_option_u16_json(output, row.source_id_candidate());
        output.push_str(",\"relationKindCandidate\":");
        push_option_u16_json(output, row.relation_kind_candidate());
        output.push_str(",\"relationKindCandidateHex\":");
        push_option_u16_hex_json(output, row.relation_kind_candidate());
        output.push_str(",\"targetRowIndexCandidate\":");
        push_option_u16_json(output, row.target_row_index_candidate());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

pub(crate) fn push_page_layer_fdm_frame_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmFrameDiagnostic<'_>,
) {
    let Some((x, y, width, height)) = fdm_frame_diagnostic_bbox(layout, diagnostic) else {
        return;
    };
    output.push_str("{\"type\":\"fdmFrameDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"fdmIndex+frame\",\"projectionKind\":\"fdmFrameDiagnosticProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(fdm_entry_frame_render_blocked_reason(
        diagnostic.candidate,
        diagnostic.entry,
    )));
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"indexPath\":");
    output.push_str(&json_string(diagnostic.entry.index_path()));
    output.push_str(",\"vectorPath\":");
    output.push_str(&json_string(diagnostic.entry.vector_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&diagnostic.entry.kind().to_string());
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", diagnostic.entry.kind())));
    output.push_str(",\"imageSignatures\":");
    push_object_image_signature_hits_json(output, diagnostic.entry.image_signature_hits());
    output.push_str(",\"segmentImageSignatures\":");
    push_object_image_signature_hits_json(output, diagnostic.entry.segment_image_signature_hits());
    output.push_str(",\"completePayloads\":");
    output.push_str(
        &fdm_entry_complete_payload_count(diagnostic.candidate, diagnostic.entry).to_string(),
    );
    output.push_str(",\"imagePayloadExtractionStatus\":");
    output.push_str(&json_string(fdm_entry_image_payload_extraction_status(
        diagnostic.candidate,
        diagnostic.entry,
    )));
    output.push_str(",\"matchedFrameRecord\":{\"sourcePath\":");
    output.push_str(&json_string(diagnostic.frame_record.source_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.frame_record.row_index().to_string());
    output.push_str(",\"objectId\":");
    output.push_str(&diagnostic.frame_record.object_id().to_string());
    output.push_str(",\"recordKind\":");
    output.push_str(&diagnostic.frame_record.record_kind().to_string());
    output.push_str(",\"recordKindHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.frame_record.record_kind()
    )));
    output.push_str(",\"objectType\":");
    output.push_str(&diagnostic.frame_record.object_type().to_string());
    output.push_str(",\"objectTypeHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.frame_record.object_type()
    )));
    output.push_str(",\"geometry\":{\"x\":");
    output.push_str(&diagnostic.frame_record.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&diagnostic.frame_record.y().to_string());
    output.push_str(",\"width\":");
    output.push_str(&diagnostic.frame_record.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&diagnostic.frame_record.height().to_string());
    output.push_str("}}");
    output.push('}');
}

pub(crate) fn push_page_layer_fdm_command_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) {
    let Some((x, y, width, height)) = fdm_command_diagnostic_bbox(layout, diagnostic, extent)
    else {
        return;
    };
    output.push_str("{\"type\":\"fdmVectorCommandDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"fdmVectorCommand\",\"projectionKind\":\"fdmCommandBBoxReferenceProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"recordLength\":");
    output.push_str(&diagnostic.command.record_len().to_string());
    output.push_str(",\"declaredRecordLength\":");
    output.push_str(&diagnostic.command.declared_record_len().to_string());
    output.push_str(",\"compoundChildOffsets\":");
    push_u16_array_json(output, diagnostic.command.compound_child_offsets());
    output.push_str(",\"compoundChildLayoutGate\":");
    push_fdm_compound_child_layout_gate_json(output, diagnostic.command);
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"sourceBbox\":");
    if let Some(bbox) = diagnostic.command.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"projectionExtent\":{\"left\":");
    output.push_str(&extent.left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&extent.top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&extent.right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&extent.bottom.to_string());
    output.push('}');
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(crate) fn push_page_layer_fdm_projection_extent_summary_json(
    output: &mut String,
    layout: PageLayout,
    command_diagnostics: &[FdmCommandDiagnostic<'_>],
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    active_extent: FdmCommandProjectionExtent,
) {
    let primitive_extent = fdm_vector_primitive_source_projection_extent(primitive_diagnostics);
    let index_entry_extent = fdm_index_entry_projection_extent(command_diagnostics);
    output.push_str("{\"type\":\"fdmProjectionExtentSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"FDMVector command bboxes+FDMIndex entry bboxes\"");
    output.push_str(",\"projectionKind\":\"fdmProjectionExtentSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true,\"sourceBacked\":true");
    output.push_str(",\"activeRenderExtentBasis\":\"fdmVectorCommandBboxExtent\"");
    output.push_str(",\"renderPromotionBlockedReason\":\"fdm-vector-page-placement-transform-source-fields-unproven\"");
    output.push_str(",\"commandDiagnosticCount\":");
    output.push_str(&command_diagnostics.len().to_string());
    output.push_str(",\"renderedPrimitiveDiagnosticCount\":");
    output.push_str(&primitive_diagnostics.len().to_string());
    output.push_str(",\"fdmIndexEntryCount\":");
    output.push_str(&fdm_index_entry_count(command_diagnostics).to_string());
    output.push_str(",\"activeCommandExtent\":");
    push_fdm_command_projection_extent_json(output, active_extent);
    output.push_str(",\"renderedPrimitiveExtent\":");
    push_optional_fdm_command_projection_extent_json(output, primitive_extent);
    output.push_str(",\"fdmIndexEntryExtent\":");
    push_optional_fdm_command_projection_extent_json(output, index_entry_extent);
    output.push_str(",\"extentAgreement\":{\"commandMatchesRenderedPrimitives\":");
    output.push_str(&(primitive_extent == Some(active_extent)).to_string());
    output.push_str(",\"commandMatchesFdmIndexEntries\":");
    output.push_str(&(index_entry_extent == Some(active_extent)).to_string());
    output.push_str(",\"renderedPrimitivesMatchFdmIndexEntries\":");
    output.push_str(
        &(primitive_extent.is_some() && primitive_extent == index_entry_extent).to_string(),
    );
    output.push('}');
    output.push_str(",\"extentResiduals\":{\"commandVsRenderedPrimitives\":");
    push_fdm_command_projection_extent_residual_json(output, Some(active_extent), primitive_extent);
    output.push_str(",\"commandVsFdmIndexEntries\":");
    push_fdm_command_projection_extent_residual_json(
        output,
        Some(active_extent),
        index_entry_extent,
    );
    output.push_str(",\"renderedPrimitivesVsFdmIndexEntries\":");
    push_fdm_command_projection_extent_residual_json(output, primitive_extent, index_entry_extent);
    output.push('}');
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(crate) fn fdm_index_entry_count(diagnostics: &[FdmCommandDiagnostic<'_>]) -> usize {
    diagnostics
        .iter()
        .map(|diagnostic| (diagnostic.candidate_index, diagnostic.entry.row_index()))
        .collect::<BTreeSet<_>>()
        .len()
}

pub(crate) fn fdm_index_entry_projection_extent(
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> Option<FdmCommandProjectionExtent> {
    let mut seen = BTreeSet::<(usize, usize)>::new();
    let mut extent = None;
    for diagnostic in diagnostics {
        if !seen.insert((diagnostic.candidate_index, diagnostic.entry.row_index())) {
            continue;
        }
        extent = fdm_bbox_extent_union(
            extent,
            normalize_fdm_index_entry_bbox(diagnostic.entry.bbox()),
        );
    }
    extent.map(|(left, top, right, bottom)| FdmCommandProjectionExtent {
        left,
        top,
        right,
        bottom,
    })
}

pub(crate) fn fdm_vector_primitive_source_projection_extent(
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> Option<FdmCommandProjectionExtent> {
    let extent = diagnostics
        .iter()
        .filter_map(|diagnostic| fdm_vector_command_source_bbox(diagnostic.command))
        .map(normalize_fdm_bbox)
        .fold(None, fdm_bbox_extent_union)?;
    Some(FdmCommandProjectionExtent {
        left: extent.0,
        top: extent.1,
        right: extent.2,
        bottom: extent.3,
    })
}

pub(crate) fn push_optional_fdm_command_projection_extent_json(
    output: &mut String,
    extent: Option<FdmCommandProjectionExtent>,
) {
    if let Some(extent) = extent {
        push_fdm_command_projection_extent_json(output, extent);
    } else {
        output.push_str("null");
    }
}

pub(crate) fn push_fdm_command_projection_extent_json(
    output: &mut String,
    extent: FdmCommandProjectionExtent,
) {
    output.push_str("{\"left\":");
    output.push_str(&extent.left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&extent.top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&extent.right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&extent.bottom.to_string());
    output.push_str(",\"spanX\":");
    output.push_str(&(extent.right - extent.left).to_string());
    output.push_str(",\"spanY\":");
    output.push_str(&(extent.bottom - extent.top).to_string());
    output.push('}');
}

pub(crate) fn push_fdm_command_projection_extent_residual_json(
    output: &mut String,
    left: Option<FdmCommandProjectionExtent>,
    right: Option<FdmCommandProjectionExtent>,
) {
    let (Some(left), Some(right)) = (left, right) else {
        output.push_str("null");
        return;
    };
    let left_delta = right.left - left.left;
    let top_delta = right.top - left.top;
    let right_delta = right.right - left.right;
    let bottom_delta = right.bottom - left.bottom;
    let max_abs_delta = left_delta
        .abs()
        .max(top_delta.abs())
        .max(right_delta.abs())
        .max(bottom_delta.abs());
    output.push_str("{\"leftDelta\":");
    output.push_str(&left_delta.to_string());
    output.push_str(",\"topDelta\":");
    output.push_str(&top_delta.to_string());
    output.push_str(",\"rightDelta\":");
    output.push_str(&right_delta.to_string());
    output.push_str(",\"bottomDelta\":");
    output.push_str(&bottom_delta.to_string());
    output.push_str(",\"maxAbsDelta\":");
    output.push_str(&max_abs_delta.to_string());
    output.push('}');
}

pub(crate) fn push_page_layer_fdm_vector_primitive_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) {
    let Some((x, y, width, height)) = fdm_path_diagnostic_bbox(layout, diagnostic, extent) else {
        return;
    };
    output.push_str("{\"type\":\"fdmVectorPrimitiveProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\",\"projectionKind\":\"fdmVectorPrimitiveReferenceProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":true,\"referenceBacked\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
    output.push_str(",\"styleWord\":");
    output.push_str(&diagnostic.command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, diagnostic.command.fill_color());
    let render_fill_color = fdm_vector_render_fill_color(diagnostic, diagnostics);
    let render_gradient = fdm_vector_linear_gradient_colors(diagnostic.command);
    output.push_str(",\"renderFillKind\":");
    output.push_str(&json_string(if render_gradient.is_some() {
        "linearGradient"
    } else if render_fill_color == "none" {
        "none"
    } else {
        "solid"
    }));
    output.push_str(",\"renderFillColor\":");
    output.push_str(&json_string(&render_fill_color));
    output.push_str(",\"renderGradient\":");
    if let Some((from, to)) = render_gradient.as_ref() {
        output.push_str("{\"from\":");
        output.push_str(&json_string(from));
        output.push_str(",\"to\":");
        output.push_str(&json_string(to));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderCounterOverlay\":");
    output.push_str(
        if fdm_vector_filled_path_is_counter_overlay(diagnostic, diagnostics) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, diagnostic.command.stroke_color());
    output.push_str(",\"renderStrokeColor\":");
    output.push_str(&json_string(&fdm_vector_render_stroke_color(
        diagnostic,
        diagnostics,
    )));
    output.push_str(",\"pathClosed\":");
    output.push_str(if fdm_vector_primitive_is_closed(diagnostic.command) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!(
        "{:.3}",
        fdm_vector_stroke_width(diagnostic.command)
    ));
    output.push_str(",\"pathPointCount\":");
    output.push_str(&diagnostic.command.path_points().len().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&diagnostic.command.curve_segments().len().to_string());
    output.push_str(",\"ellipse\":");
    if let Some(ellipse) = diagnostic.command.ellipse() {
        push_fdm_vector_ellipse_json(output, ellipse);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourcePathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"paintCoverage\":");
    push_fdm_paint_coverage_json(
        output,
        fdm_vector_paint_coverage(layout, diagnostic, diagnostics, (x, y, width, height)),
    );
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(crate) fn push_page_layer_fdm_vector_primitive_large_span_blocked_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
) {
    let Some((x, y, width, height)) = fdm_path_span_filter_blocked(layout, diagnostic, extent)
    else {
        return;
    };
    output.push_str("{\"type\":\"fdmVectorPrimitiveLargeSpanBlockedDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    let paint_coverage = fdm_vector_paint_coverage(
        layout,
        diagnostic,
        primitive_diagnostics,
        (x, y, width, height),
    );
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\",\"projectionKind\":\"fdmVectorPrimitiveLargeSpanFilteredProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"diagnosticOnly\":true,\"referenceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(if paint_coverage.page_fill_candidate {
        "fdm-page-fill-source-evidence-unproven"
    } else {
        "fdm-vector-large-span-filter-unproven"
    }));
    output.push_str(",\"filterBasis\":\"projected-page-span-ratio\"");
    output.push_str(",\"largeSpanFilterMaxPageRatio\":");
    output.push_str(&format!("{FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO:.6}"));
    output.push_str(",\"pageWidthRatio\":");
    output.push_str(&format!("{:.6}", width / layout.width_px()));
    output.push_str(",\"pageHeightRatio\":");
    output.push_str(&format!("{:.6}", height / layout.height_px()));
    let viewport = fdm_projection_viewport(layout);
    output.push_str(",\"viewportWidthRatio\":");
    output.push_str(&format!("{:.6}", width / viewport.width));
    output.push_str(",\"viewportHeightRatio\":");
    output.push_str(&format!("{:.6}", height / viewport.height));
    output.push_str(",\"paintCoverage\":");
    push_fdm_paint_coverage_json(output, paint_coverage);
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
    output.push_str(",\"styleWord\":");
    output.push_str(&diagnostic.command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    output.push_str(",\"pathClosed\":");
    output.push_str(if fdm_vector_primitive_is_closed(diagnostic.command) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!(
        "{:.3}",
        fdm_vector_stroke_width(diagnostic.command)
    ));
    output.push_str(",\"pathPointCount\":");
    output.push_str(&diagnostic.command.path_points().len().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&diagnostic.command.curve_segments().len().to_string());
    output.push_str(",\"sourcePathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
        push_object_fdm_index_bbox_json(output, bbox);
        output.push_str(",\"normalizedSourcePathBbox\":");
        push_fdm_normalized_bbox_json(output, normalize_fdm_bbox(bbox));
    } else {
        output.push_str("null,\"normalizedSourcePathBbox\":null");
    }
    output.push_str(",\"projectionExtent\":{\"left\":");
    output.push_str(&extent.left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&extent.top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&extent.right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&extent.bottom.to_string());
    output.push('}');
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}
