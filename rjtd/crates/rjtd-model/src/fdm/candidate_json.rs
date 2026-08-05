use super::*;
use crate::*;

pub(crate) fn fdm_text_mirror_anchor_agreements_json(
    agreements: &[FdmTextMirrorAnchorAgreement],
) -> String {
    let mut output = String::from("[");
    for (index, agreement) in agreements.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_text_mirror_anchor_agreement_json(&mut output, agreement);
    }
    output.push(']');
    output
}

pub(crate) fn push_fdm_text_mirror_anchor_agreement_json(
    output: &mut String,
    agreement: &FdmTextMirrorAnchorAgreement,
) {
    output.push_str(
        "{\"source\":\"FDMText mirrored record sequence+FDMTextIndex row-to-record links\"",
    );
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false");
    output.push_str(",\"diagnosticOnly\":true,\"renderable\":false,\"placementProven\":false");
    output.push_str(",\"indexedTextPath\":");
    output.push_str(&json_string(agreement.indexed_text_path()));
    output.push_str(",\"mirroredTextPath\":");
    output.push_str(&json_string(agreement.mirrored_text_path()));
    output.push_str(",\"textRecordCount\":");
    output.push_str(&agreement.text_record_count().to_string());
    output.push_str(",\"orderedTextAgreement\":");
    output.push_str(&agreement.ordered_text_agreement().to_string());
    output.push_str(",\"orderedRecordBboxAgreement\":");
    output.push_str(&agreement.ordered_record_bbox_agreement().to_string());
    output.push_str(",\"indexedRecordOffsetAgreement\":");
    output.push_str(&agreement.indexed_record_offset_agreement().to_string());
    output.push_str(",\"indexedRecordBboxAgreement\":");
    output.push_str(&agreement.indexed_record_bbox_agreement().to_string());
    output.push_str(",\"sourceAnchorTraceReady\":");
    output.push_str(&agreement.source_anchor_trace_ready().to_string());
    output.push_str(
        ",\"sourceToPageTransformDecoded\":false,\"roleDecoded\":false,\"paintOrderDecoded\":false",
    );
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"fdmtext-source-to-page-transform-undecoded\"}",
    );
}

pub(crate) fn push_fdm_index_segment_bbox_axis_pair_gate_json(
    output: &mut String,
    gate: FdmIndexSegmentBboxAxisPairGate,
) {
    output.push_str("{\"source\":\"FDMIndex raw bbox fields+FDMVector segment header bbox\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false");
    output.push_str(",\"diagnosticOnly\":true,\"renderable\":false,\"placementProven\":false");
    output.push_str(",\"validIndexRowCount\":");
    output.push_str(&gate.valid_index_row_count().to_string());
    output.push_str(",\"linkedRowCount\":");
    output.push_str(&gate.linked_row_count().to_string());
    output.push_str(",\"axisPairOrderAgreementRowCount\":");
    output.push_str(&gate.axis_pair_order_agreement_row_count().to_string());
    output.push_str(",\"axisPairOrderAgreementComplete\":");
    output.push_str(&gate.axis_pair_order_agreement_complete().to_string());
    output.push_str(",\"normalizationInputSourceBacked\":");
    output.push_str(&gate.axis_pair_order_agreement_complete().to_string());
    output.push_str(
        ",\"fieldOrderDecoded\":false,\"pageTransformDecoded\":false,\"objectRoleDecoded\":false",
    );
    let blocked_reason = if gate.axis_pair_order_agreement_complete() {
        "fdm-index-axis-pair-does-not-decode-page-transform-or-object-role"
    } else {
        "fdm-index-axis-pair-order-incomplete"
    };
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(blocked_reason));
    output.push('}');
}

pub(crate) fn push_object_fdm_text_candidate_json(
    output: &mut String,
    candidate: &ObjectFdmTextCandidate,
) {
    output.push_str("{\"text\":");
    output.push_str(&json_string(candidate.text()));
    output.push_str(",\"textOffset\":");
    output.push_str(&candidate.text_offset().to_string());
    output.push_str(",\"markerOffset\":");
    output.push_str(&candidate.marker_offset().to_string());
    output.push_str(",\"rawTextHex\":");
    output.push_str(&json_string(&hex_bytes(candidate.raw_text())));
    output.push_str(",\"bbox\":");
    if let Some(bbox) = candidate.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_fdm_text_index_entry_candidate_json(
    output: &mut String,
    candidate: &ObjectFdmTextIndexEntryCandidate,
) {
    output.push_str("{\"indexPath\":");
    output.push_str(&json_string(candidate.index_path()));
    output.push_str(",\"textPath\":");
    output.push_str(&json_string(candidate.text_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&candidate.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&candidate.index_offset().to_string());
    output.push_str(",\"textRecordOffset\":");
    output.push_str(&candidate.text_record_offset().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&candidate.kind().to_string());
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", candidate.kind())));
    output.push_str(",\"validTextRecordOffset\":");
    output.push_str(if candidate.valid_text_record_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bbox\":");
    push_object_fdm_index_bbox_json(output, candidate.bbox());
    output.push_str(",\"textRecordBbox\":");
    if let Some(bbox) = candidate.text_record_bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"textRecordPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(candidate.text_record_prefix())));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_fdm_index_entry_candidate_json(
    output: &mut String,
    entry: &ObjectFdmIndexEntryCandidate,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) {
    output.push_str("{\"indexPath\":");
    output.push_str(&json_string(entry.index_path()));
    output.push_str(",\"vectorPath\":");
    output.push_str(&json_string(entry.vector_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&entry.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&entry.index_offset().to_string());
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
    push_object_fdm_index_bbox_json(output, entry.bbox());
    output.push_str(",\"validVectorOffset\":");
    output.push_str(if entry.valid_vector_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"offsetFieldReferenceCandidates\":");
    push_object_fdm_index_offset_field_reference_candidates_json(output, entry, raw_commands);
    output.push_str(",\"vectorPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(entry.vector_prefix())));
    output.push_str(",\"vectorCommandCount\":");
    output.push_str(&entry.vector_commands().len().to_string());
    output.push_str(",\"vectorCommandBboxCount\":");
    output.push_str(
        &entry
            .vector_commands()
            .iter()
            .filter(|command| command.bbox().is_some())
            .count()
            .to_string(),
    );
    output.push_str(",\"vectorCommands\":[");
    for (index, command) in entry.vector_commands().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_vector_command_candidate_json(output, command);
    }
    output.push(']');
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&entry.connector_candidates().len().to_string());
    output.push_str(",\"connectorCandidates\":[");
    for (index, candidate) in entry.connector_candidates().iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_connector_candidate_json(output, candidate);
    }
    output.push(']');
    output.push_str(",\"imageSignatures\":[");
    for (index, hit) in entry.image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"segmentImageSignatures\":[");
    for (index, hit) in entry.segment_image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"decoded\":false}");
}

pub(crate) fn push_object_fdm_index_offset_field_reference_candidates_json(
    output: &mut String,
    entry: &ObjectFdmIndexEntryCandidate,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) {
    let bbox = entry.bbox();
    let fields = [
        Some(("vectorOffset", entry.vector_offset())),
        non_negative_i32_offset("bbox.left", bbox.left()),
        non_negative_i32_offset("bbox.top", bbox.top()),
        non_negative_i32_offset("bbox.right", bbox.right()),
        non_negative_i32_offset("bbox.bottom", bbox.bottom()),
    ];
    output.push('[');
    let mut emitted = 0usize;
    for field in fields.into_iter().flatten() {
        emitted += push_object_fdm_index_offset_field_reference_candidate_json(
            output,
            emitted,
            field.0,
            field.1,
            raw_commands,
        );
    }
    output.push(']');
}

pub(crate) fn push_object_fdm_index_offset_field_reference_candidate_json(
    output: &mut String,
    emitted: usize,
    field_name: &str,
    field_value: usize,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) -> usize {
    let command_matches = raw_commands
        .iter()
        .filter(|command| command.relative_offset() == field_value)
        .map(ObjectFdmVectorCommandCandidate::relative_offset)
        .collect::<Vec<_>>();
    let segment_matches = raw_commands
        .iter()
        .filter(|command| {
            command
                .source_segment()
                .is_some_and(|segment| segment.relative_offset() == field_value)
        })
        .map(ObjectFdmVectorCommandCandidate::relative_offset)
        .collect::<Vec<_>>();

    let mut local_emitted = 0usize;
    if !command_matches.is_empty() {
        if emitted + local_emitted > 0 {
            output.push(',');
        }
        output.push_str("{\"offsetField\":");
        output.push_str(&json_string(field_name));
        output.push_str(",\"offsetValue\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"matchKind\":\"command-relative-offset-field\"");
        output.push_str(",\"referenceSource\":\"fdmRawVectorCommands.relativeOffset\"");
        output.push_str(",\"matchedCommandRelativeOffsets\":");
        push_usize_array_json(output, &command_matches);
        output.push_str(",\"decoded\":false}");
        local_emitted += 1;
    }
    if !segment_matches.is_empty() {
        if emitted + local_emitted > 0 {
            output.push(',');
        }
        output.push_str("{\"offsetField\":");
        output.push_str(&json_string(field_name));
        output.push_str(",\"offsetValue\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"matchKind\":\"source-segment-relative-offset-field\"");
        output
            .push_str(",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\"");
        output.push_str(",\"sourceSegmentRelativeOffset\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"sourceSegmentBackedCommandCount\":");
        output.push_str(&segment_matches.len().to_string());
        output.push_str(",\"matchedCommandRelativeOffsets\":");
        push_usize_array_json(output, &segment_matches);
        output.push_str(",\"decoded\":false}");
        local_emitted += 1;
    }
    local_emitted
}

pub(crate) fn push_object_fdm_connector_candidate_json(
    output: &mut String,
    candidate: ObjectFdmConnectorCandidate,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&candidate.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&candidate.relative_offset().to_string());
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(&candidate.marker())));
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(candidate.primitive_kind()));
    output.push_str(",\"styleWord\":");
    output.push_str(&candidate.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", candidate.style_word())));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, candidate.fill_color());
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, candidate.stroke_color());
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis()));
    output.push_str(",\"sourceEndpoints\":");
    push_fdm_connector_candidate_source_endpoints_json(output, candidate);
    output.push_str(",\"sourceBbox\":");
    push_object_fdm_index_bbox_json(output, candidate.source_bbox());
    output.push_str(",\"sourceSpan\":");
    output.push_str(&candidate.source_span().to_string());
    output.push_str(",\"endpointDelta\":{\"x\":");
    output.push_str(&candidate.endpoint_dx().to_string());
    output.push_str(",\"y\":");
    output.push_str(&candidate.endpoint_dy().to_string());
    output.push('}');
    output.push_str(",\"endpointDistanceSquared\":");
    output.push_str(&candidate.endpoint_distance_squared().to_string());
    output.push_str(",\"pathPointCount\":");
    output.push_str(&candidate.path_point_count().to_string());
    output.push_str(",\"pathSegmentCount\":");
    output.push_str(&candidate.path_segment_count().to_string());
    output.push_str(",\"orthogonalSegmentCount\":");
    output.push_str(&candidate.orthogonal_segment_count().to_string());
    output.push_str(",\"diagonalSegmentCount\":");
    output.push_str(&candidate.diagonal_segment_count().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&candidate.curve_segment_count().to_string());
    output.push_str(",\"compoundChildOffsetCount\":");
    output.push_str(&candidate.compound_child_offset_count().to_string());
    output.push_str(",\"axisAligned\":");
    output.push_str(if candidate.axis_aligned() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"orientation\":");
    output.push_str(&json_string(candidate.orientation()));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_fdm_connector_candidate_source_endpoints_json(
    output: &mut String,
    candidate: ObjectFdmConnectorCandidate,
) {
    output.push_str("{\"start\":");
    push_fdm_vector_point_json(output, candidate.source_start());
    output.push_str(",\"end\":");
    push_fdm_vector_point_json(output, candidate.source_end());
    output.push('}');
}

pub(crate) fn push_fdm_vector_point_json(output: &mut String, point: ObjectFdmVectorPoint) {
    output.push_str("{\"x\":");
    output.push_str(&point.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&point.y().to_string());
    output.push('}');
}

pub(crate) fn push_object_fdm_vector_command_candidate_json(
    output: &mut String,
    command: &ObjectFdmVectorCommandCandidate,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&command.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&command.relative_offset().to_string());
    output.push_str(",\"sourceVectorRelativeOffset\":");
    push_optional_usize_json(output, command.source_vector_relative_offset());
    output.push_str(",\"sourceSegment\":");
    if let Some(source_segment) = command.source_segment() {
        push_object_fdm_vector_command_source_segment_json(output, source_segment);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"recordLength\":");
    output.push_str(&command.record_len().to_string());
    output.push_str(",\"declaredRecordLength\":");
    output.push_str(&command.declared_record_len().to_string());
    output.push_str(",\"styleWord\":");
    output.push_str(&command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", command.style_word())));
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(command.marker())));
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(command)));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, command.fill_color());
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, command.stroke_color());
    output.push_str(",\"bbox\":");
    if let Some(bbox) = command.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pathPointCount\":");
    output.push_str(&command.path_points().len().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&command.curve_segments().len().to_string());
    output.push_str(",\"ellipse\":");
    if let Some(ellipse) = command.ellipse() {
        push_fdm_vector_ellipse_json(output, ellipse);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(command) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"compoundChildLayoutGate\":");
    push_fdm_compound_child_layout_gate_json(output, command);
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_fdm_compound_child_layout_gate_json(
    output: &mut String,
    command: &ObjectFdmVectorCommandCandidate,
) {
    let Some(layout) = command.compound_child_layout() else {
        output.push_str("null");
        return;
    };
    output.push_str(
        "{\"source\":\"FDMVector compound prefix child-offset table+child declared lengths\"",
    );
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"childOffsets\":");
    push_u16_array_json(output, layout.child_offsets());
    output.push_str(",\"firstChildMatchesPrefixEnd\":");
    output.push_str(&layout.first_child_matches_prefix_end().to_string());
    output.push_str(",\"childOffsetsStrictlyIncreasing\":");
    output.push_str(&layout.child_offsets_strictly_increasing().to_string());
    output.push_str(",\"childRecordsFitParent\":");
    output.push_str(&layout.child_records_fit_parent().to_string());
    output.push_str(",\"childRecordsDoNotOverlap\":");
    output.push_str(&layout.child_records_do_not_overlap().to_string());
    output.push_str(",\"nestedProjectionInputValid\":");
    output.push_str(&layout.is_valid_for_nested_projection().to_string());
    output.push_str(",\"renderPromotionBlockedReason\":\"compound-child-boundaries-do-not-prove-connector-ownership-or-paint-order\"}");
}

pub(crate) fn push_object_fdm_vector_command_source_segment_json(
    output: &mut String,
    source_segment: ObjectFdmVectorCommandSourceSegment,
) {
    output.push_str("{\"relativeOffset\":");
    output.push_str(&source_segment.relative_offset().to_string());
    output.push_str(",\"localOffset\":");
    output.push_str(&source_segment.local_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&source_segment.declared_len().to_string());
    output.push_str(",\"commandCount\":");
    output.push_str(&source_segment.command_count().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&source_segment.command_index().to_string());
    output.push_str(",\"commandOffset\":");
    output.push_str(&source_segment.command_offset().to_string());
    output.push('}');
}

pub(crate) fn push_object_fdm_vector_segment_candidate_json(
    output: &mut String,
    segment: &ObjectFdmVectorSegmentCandidate,
) {
    output.push_str("{\"relativeOffset\":");
    output.push_str(&segment.relative_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&segment.declared_len().to_string());
    output.push_str(",\"commandCount\":");
    output.push_str(&segment.command_count().to_string());
    output.push_str(",\"commandOffsets\":");
    push_u16_array_json(output, segment.command_offsets());
    output.push_str(",\"bbox\":");
    if let Some(bbox) = segment.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceSpanCandidate\":{\"width\":");
    output.push_str(&segment.source_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&segment.source_height().to_string());
    output.push_str("},\"decoded\":false}");
}

pub(crate) fn push_fdm_vector_ellipse_json(output: &mut String, ellipse: ObjectFdmVectorEllipse) {
    let center = ellipse.center();
    output.push_str("{\"center\":{\"x\":");
    output.push_str(&center.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&center.y().to_string());
    output.push_str("},\"radiusX\":");
    output.push_str(&ellipse.radius_x().to_string());
    output.push_str(",\"radiusY\":");
    output.push_str(&ellipse.radius_y().to_string());
    output.push_str(",\"color\":");
    if let Some(color) = ellipse.color().and_then(fdm_vector_primitive_css_color) {
        output.push_str(&json_string(&color));
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn push_fdm_vector_optional_color_json(output: &mut String, color: Option<u32>) {
    if let Some(color) = color.and_then(fdm_vector_css_color) {
        output.push_str(&json_string(&color));
    } else {
        output.push_str("null");
    }
}

pub(crate) fn push_object_fdm_index_bbox_json(output: &mut String, bbox: ObjectFdmIndexBbox) {
    output.push_str("{\"left\":");
    output.push_str(&bbox.left().to_string());
    output.push_str(",\"top\":");
    output.push_str(&bbox.top().to_string());
    output.push_str(",\"right\":");
    output.push_str(&bbox.right().to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&bbox.bottom().to_string());
    output.push('}');
}

pub(crate) fn fdm_vector_css_color(color: u32) -> Option<String> {
    if color > 0x00ff_ffff {
        return None;
    }
    let blue = (color >> 16) & 0xff;
    let green = (color >> 8) & 0xff;
    let red = color & 0xff;
    Some(format!("#{red:02x}{green:02x}{blue:02x}"))
}

pub(crate) fn fdm_vector_primitive_css_color(color: u32) -> Option<String> {
    if color <= 0x00ff_ffff {
        return fdm_vector_css_color(color);
    }
    if color & 0xff00_0000 == 0xff00_0000 {
        return fdm_vector_css_color(color & 0x00ff_ffff);
    }
    None
}
