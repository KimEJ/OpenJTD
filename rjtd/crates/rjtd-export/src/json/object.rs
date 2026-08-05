use rjtd_model::{
    ObjectEmbeddedPressSnapshotCandidate, ObjectEmbeddedPressVectorPathCandidate,
    ObjectEmbeddingFrameCandidate, ObjectFigureLinkCandidate, ObjectFigureLinkRowCandidate,
    ObjectFrameRecordCandidate, ObjectFrameReferenceRowCandidate, ObjectJseq3FormulaCandidate,
    ObjectJsfartArtCandidate, ObjectJsfartArtPaintCandidate, ObjectJsfartStreamProfileCandidate,
    ObjectStreamCandidate, ObjectStreamOwnershipCandidate, ObjectStreamOwnershipReferenceCandidate,
    ObjectVisualListCandidate, UnknownObject,
};

use super::fdm::{
    push_object_fdm_index_entry_candidate_json, push_object_fdm_text_candidate_json,
    push_object_fdm_text_index_entry_candidate_json, push_object_fdm_vector_command_candidate_json,
    push_object_fdm_vector_segment_candidate_json,
    push_success_data_test_fdm_reference_projections_json,
};
use super::image::push_object_image_payload_span_json;
use super::primitives::{
    hex, push_json_string, push_option_u16_hex_json, push_option_u16_json,
    push_option_u32_hex_json, push_option_u32_json, push_option_usize_json, push_u16_array_json,
    push_u32_array_json, push_u32_hex_array_json, push_usize_array_json,
};
use super::style::push_unknown_source_json;

const EMBEDDED_PRESS_RECORD_PAINT_STATE_82: u32 = 0x82;
pub(crate) fn push_unknown_object_json(output: &mut String, object: &UnknownObject) {
    output.push_str("{\"source\":");
    push_unknown_source_json(output, object.source());
    output.push_str(",\"payloadHex\":");
    push_json_string(output, &hex(object.payload()));
    output.push('}');
}

pub(crate) fn push_object_frame_record_candidate_json(
    output: &mut String,
    record: &ObjectFrameRecordCandidate,
) {
    output.push_str("{\"sourcePath\":");
    push_json_string(output, record.source_path());
    output.push_str(",\"rowIndex\":");
    output.push_str(&record.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&record.row_start().to_string());
    output.push_str(",\"recordLen\":");
    output.push_str(&record.record_len().to_string());
    output.push_str(",\"recordKind\":");
    output.push_str(&record.record_kind().to_string());
    output.push_str(",\"recordKindHex\":");
    push_json_string(output, &format!("0x{:04x}", record.record_kind()));
    output.push_str(",\"declaredRecordBytes\":");
    output.push_str(&record.declared_record_bytes().to_string());
    output.push_str(",\"objectId\":");
    output.push_str(&record.object_id().to_string());
    output.push_str(",\"objectType\":");
    output.push_str(&record.object_type().to_string());
    output.push_str(",\"objectTypeHex\":");
    push_json_string(output, &format!("0x{:04x}", record.object_type()));
    output.push_str(",\"geometry\":{\"x\":");
    output.push_str(&record.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&record.y().to_string());
    output.push_str(",\"width\":");
    output.push_str(&record.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&record.height().to_string());
    output.push_str("},\"rowPrefixHex\":");
    push_json_string(output, &hex(record.row_prefix()));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_embedding_frame_candidate_json(
    output: &mut String,
    frame: &ObjectEmbeddingFrameCandidate,
) {
    output.push_str("{\"sourcePath\":");
    push_json_string(output, frame.source_path());
    output.push_str(",\"rowIndex\":");
    output.push_str(&frame.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&frame.row_start().to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    push_json_string(output, frame.class_name());
    output.push_str(",\"primarySize\":{\"width\":");
    output.push_str(&frame.primary_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&frame.primary_height().to_string());
    output.push_str("},\"frameRef\":");
    output.push_str(&frame.frame_ref().to_string());
    output.push_str(",\"frameSize\":{\"width\":");
    output.push_str(&frame.frame_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&frame.frame_height().to_string());
    output.push_str("},\"rowPrefixHex\":");
    push_json_string(output, &hex(frame.row_prefix()));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_stream_candidate_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    output.push_str("{\"path\":");
    push_json_string(output, candidate.path());
    output.push_str(",\"size\":");
    output.push_str(&candidate.size().to_string());
    output.push_str(",\"reasons\":[");
    for (index, reason) in candidate.reasons().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, reason.as_str());
    }
    output.push_str("],\"ownershipCandidate\":");
    if let Some(ownership) = candidate.ownership_candidate() {
        push_object_stream_ownership_candidate_json(output, ownership);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"ownershipReferences\":[");
    for (index, reference) in candidate
        .ownership_reference_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_object_stream_ownership_reference_candidate_json(output, reference);
    }
    output.push_str("],\"frameReferenceRows\":[");
    for (index, row) in candidate
        .frame_reference_row_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_object_frame_reference_row_candidate_json(output, row);
    }
    output.push_str("],\"figureLink\":");
    if let Some(link) = candidate.figure_link_candidate() {
        push_object_figure_link_candidate_json(output, link);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"fdmIndexEntries\":[");
    for (index, entry) in candidate.fdm_index_entry_candidates().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_index_entry_candidate_json(
            output,
            entry,
            candidate.fdm_raw_vector_commands(),
        );
    }
    output.push_str("],\"fdmTextIndexEntries\":[");
    for (index, entry) in candidate
        .fdm_text_index_entry_candidates()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_text_index_entry_candidate_json(output, entry);
    }
    output.push_str("],\"fdmRawVectorSegmentCount\":");
    output.push_str(&candidate.fdm_raw_vector_segments().len().to_string());
    output.push_str(",\"fdmRawVectorSegments\":[");
    for (index, segment) in candidate.fdm_raw_vector_segments().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_vector_segment_candidate_json(output, segment);
    }
    output.push_str("],\"fdmRawVectorCommandCount\":");
    output.push_str(&candidate.fdm_raw_vector_commands().len().to_string());
    output.push_str(",\"fdmRawVectorCommands\":[");
    for (index, command) in candidate.fdm_raw_vector_commands().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_vector_command_candidate_json(output, command);
    }
    output.push_str("],\"successDataTestFdmReferenceProjections\":");
    push_success_data_test_fdm_reference_projections_json(output, candidate);
    output.push_str(",\"fdmTextCount\":");
    output.push_str(&candidate.fdm_text_candidates().len().to_string());
    output.push_str(",\"fdmTextCandidates\":[");
    for (index, text) in candidate.fdm_text_candidates().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_text_candidate_json(output, text);
    }
    output.push_str("],\"imageSignatures\":[");
    for (index, hit) in candidate.image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        push_json_string(output, hit.kind());
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"imagePayloads\":[");
    for (index, span) in candidate.image_payload_spans().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_payload_span_json(output, span);
    }
    output.push_str("],\"svgOffsets\":");
    push_usize_array_json(output, candidate.svg_offsets());
    output.push_str(",\"soOffsets\":");
    push_usize_array_json(output, candidate.so_offsets());
    output.push_str(",\"visualList\":");
    if let Some(visual_list) = candidate.visual_list_candidate() {
        push_object_visual_list_candidate_json(output, visual_list);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddedPressSnapshot\":");
    if let Some(snapshot) = candidate.embedded_press_snapshot_candidate() {
        push_object_embedded_press_snapshot_candidate_json(output, snapshot);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"jseq3Formula\":");
    if let Some(formula) = candidate.jseq3_formula_candidate() {
        push_object_jseq3_formula_candidate_json(output, formula);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"jsfartStreamProfile\":");
    if let Some(profile) = candidate.jsfart_stream_profile_candidate() {
        push_object_jsfart_stream_profile_candidate_json(output, profile);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"jsfartArt\":");
    if let Some(art) = candidate.jsfart_art_candidate() {
        push_object_jsfart_art_candidate_json(output, art);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"payloadPrefixHex\":");
    push_json_string(output, &hex(candidate.payload_prefix()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_jsfart_stream_profile_candidate_json(
    output: &mut String,
    profile: &ObjectJsfartStreamProfileCandidate,
) {
    output.push_str("{\"format\":\"JSFart2Contents\",\"source\":\"stream-prefix\",\"sourceCandidateType\":\"objectStream\",\"magicFamily\":");
    push_json_string(output, profile.magic_family());
    output.push_str(",\"magicFamilyHex\":");
    push_json_string(output, profile.magic_family_hex());
    output.push_str(",\"magicOffset\":");
    output.push_str(&profile.magic_offset().to_string());
    output.push_str(",\"magicAsciiOrUtf16Preview\":");
    push_json_string(output, profile.magic_ascii_or_utf16_preview());
    output.push_str(",\"headerPrefixHex\":");
    push_json_string(output, &hex(profile.header_prefix()));
    output.push_str(",\"structuredArtCandidatePresent\":");
    output.push_str(if profile.structured_art_candidate_present() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"renderable\":false,\"decoded\":false,\"renderPromotionBlockedReason\":");
    push_json_string(output, profile.render_promotion_blocked_reason());
    output.push('}');
}

fn push_object_figure_link_candidate_json(output: &mut String, link: &ObjectFigureLinkCandidate) {
    output.push_str("{\"headerWordsBe\":");
    push_u16_array_json(output, link.header_words_be());
    output.push_str(",\"declaredRowCountCandidate\":");
    push_option_u16_json(output, link.declared_row_count_candidate());
    output.push_str(",\"rowStride\":");
    output.push_str(&link.row_stride().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&link.rows().len().to_string());
    output.push_str(",\"rows\":[");
    for (index, row) in link.rows().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_figure_link_row_candidate_json(output, row);
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

fn push_object_figure_link_row_candidate_json(
    output: &mut String,
    row: &ObjectFigureLinkRowCandidate,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&row.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&row.row_start().to_string());
    output.push_str(",\"wordsBe\":");
    push_u16_array_json(output, row.words_be());
    output.push_str(",\"groupIndexCandidate\":");
    push_option_u16_json(output, row.group_index_candidate());
    output.push_str(",\"sourceIdCandidate\":");
    push_option_u16_json(output, row.source_id_candidate());
    output.push_str(",\"relationKindCandidate\":");
    push_option_u16_json(output, row.relation_kind_candidate());
    output.push_str(",\"relationKindCandidateHex\":");
    push_option_u16_hex_json(output, row.relation_kind_candidate());
    output.push_str(",\"targetRowIndexCandidate\":");
    push_option_u16_json(output, row.target_row_index_candidate());
    output.push_str(",\"rowHex\":");
    push_json_string(output, &hex(row.row()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_jsfart_art_candidate_json(output: &mut String, art: &ObjectJsfartArtCandidate) {
    output.push_str("{\"format\":\"JSFart2Contents\",\"magic\":");
    push_json_string(output, art.magic());
    output.push_str(",\"magicOffset\":");
    output.push_str(&art.magic_offset().to_string());
    output.push_str(",\"width\":");
    output.push_str(&art.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&art.height().to_string());
    output.push_str(",\"frameCandidate\":");
    if let Some(frame) = art.frame_candidate() {
        output.push_str("{\"left\":");
        output.push_str(&frame.left().to_string());
        output.push_str(",\"top\":");
        output.push_str(&frame.top().to_string());
        output.push_str(",\"right\":");
        output.push_str(&frame.right().to_string());
        output.push_str(",\"bottom\":");
        output.push_str(&frame.bottom().to_string());
        output.push_str(",\"contentLeft\":");
        output.push_str(&frame.content_left().to_string());
        output.push_str(",\"contentTop\":");
        output.push_str(&frame.content_top().to_string());
        output.push_str(",\"contentRight\":");
        output.push_str(&frame.content_right().to_string());
        output.push_str(",\"contentBottom\":");
        output.push_str(&frame.content_bottom().to_string());
        output.push_str(",\"cornerRadiusX\":");
        output.push_str(&frame.corner_radius_x().to_string());
        output.push_str(",\"cornerRadiusY\":");
        output.push_str(&frame.corner_radius_y().to_string());
        output.push_str(",\"strokeWidthCandidate\":");
        push_option_u32_json(output, frame.stroke_width_candidate());
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"paintCandidate\":");
    if let Some(paint) = art.paint_candidate() {
        push_object_jsfart_art_paint_candidate_json(output, paint);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"headerPrefixHex\":");
    push_json_string(output, &hex(art.header_prefix()));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

fn push_object_jsfart_art_paint_candidate_json(
    output: &mut String,
    paint: &ObjectJsfartArtPaintCandidate,
) {
    output.push_str("{\"styleWord1\":");
    output.push_str(&paint.style_word_1().to_string());
    output.push_str(",\"styleWord1Hex\":");
    push_json_string(output, &format!("0x{:08x}", paint.style_word_1()));
    output.push_str(",\"styleWord2\":");
    output.push_str(&paint.style_word_2().to_string());
    output.push_str(",\"styleWord2Hex\":");
    push_json_string(output, &format!("0x{:08x}", paint.style_word_2()));
    output.push_str(",\"paintColorCandidate\":");
    output.push_str(&paint.paint_color_candidate().to_string());
    output.push_str(",\"paintColorCandidateHex\":");
    push_json_string(output, &format!("0x{:08x}", paint.paint_color_candidate()));
    output.push_str(",\"paintFlagCandidate\":");
    output.push_str(&paint.paint_flag_candidate().to_string());
    output.push_str(",\"paintFlagCandidateHex\":");
    push_json_string(output, &format!("0x{:08x}", paint.paint_flag_candidate()));
    output.push_str(",\"effectWordCandidate\":");
    output.push_str(&paint.effect_word_candidate().to_string());
    output.push_str(",\"effectWordCandidateHex\":");
    push_json_string(output, &format!("0x{:08x}", paint.effect_word_candidate()));
    output.push_str(",\"decoded\":false}");
}

fn push_object_jseq3_formula_candidate_json(
    output: &mut String,
    formula: &ObjectJseq3FormulaCandidate,
) {
    output.push_str("{\"format\":\"JSEQ3Contents\",\"magic\":");
    push_json_string(output, formula.magic());
    output.push_str(",\"magicOffset\":");
    output.push_str(&formula.magic_offset().to_string());
    output.push_str(",\"soTrailerOffset\":");
    push_option_usize_json(output, formula.so_trailer_offset());
    output.push_str(",\"soTrailerLength\":");
    push_option_usize_json(output, formula.so_trailer_length());
    output.push_str(",\"soTrailerFields\":");
    push_u32_array_json(output, formula.so_trailer_fields());
    output.push_str(",\"textMarkers\":[");
    for (index, marker) in formula.text_markers().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        push_json_string(output, marker.text());
        output.push_str(",\"offset\":");
        output.push_str(&marker.offset().to_string());
        output.push_str(",\"encoding\":");
        push_json_string(output, marker.encoding());
        output.push('}');
    }
    output.push_str("],\"headerPrefixHex\":");
    push_json_string(output, &hex(formula.header_prefix()));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

fn push_object_embedded_press_snapshot_candidate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    output.push_str("{\"format\":\"JSSnapShot32\",\"magic\":");
    push_json_string(output, snapshot.magic());
    output.push_str(",\"bodyLengthCandidate\":");
    output.push_str(&snapshot.body_length_candidate().to_string());
    output.push_str(",\"formatMarker\":");
    push_json_string(output, snapshot.format_marker());
    output.push_str(",\"objectCountCandidate\":");
    output.push_str(&snapshot.object_count_candidate().to_string());
    output.push_str(",\"objectTableOffsetCandidate\":");
    output.push_str(&snapshot.object_table_offset_candidate().to_string());
    output.push_str(",\"payloadLengthCandidate\":");
    output.push_str(&snapshot.payload_length_candidate().to_string());
    output.push_str(",\"width\":");
    output.push_str(&snapshot.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&snapshot.height().to_string());
    output.push_str(",\"vectorSegmentCount\":");
    output.push_str(&snapshot.vector_segments().len().to_string());
    output.push_str(",\"vectorPathCount\":");
    output.push_str(&snapshot.vector_paths().len().to_string());
    output.push_str(",\"textureBezierHeaderSummary\":");
    push_embedded_press_texture_bezier_header_summary_json(output, snapshot);
    output.push_str(",\"paintStateTransitions\":");
    push_embedded_press_paint_state_transitions_json(output, snapshot);
    output.push_str(",\"stateRecordSummary\":");
    push_embedded_press_state_record_summary_json(output, snapshot);
    output.push_str(",\"vectorSegmentPreview\":");
    push_object_embedded_press_snapshot_vector_segment_preview_json(output, snapshot);
    output.push_str(",\"headerPrefixHex\":");
    push_json_string(output, &hex(snapshot.header_prefix()));
    output.push_str(",\"renderable\":");
    output.push_str(if snapshot.vector_segments().is_empty() {
        "false"
    } else {
        "true"
    });
    output.push_str(",\"decoded\":false}");
}

fn push_object_embedded_press_snapshot_vector_segment_preview_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    output.push('[');
    for (index, segment) in snapshot.vector_segments().iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"x1\":");
        output.push_str(&segment.x1().to_string());
        output.push_str(",\"y1\":");
        output.push_str(&segment.y1().to_string());
        output.push_str(",\"x2\":");
        output.push_str(&segment.x2().to_string());
        output.push_str(",\"y2\":");
        output.push_str(&segment.y2().to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn push_embedded_press_texture_bezier_header_summary_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let mut path_count = 0usize;
    let mut first_header = None;
    let mut homogeneous = true;
    for path in snapshot.vector_paths() {
        let Some(header) = path.texture_bezier_header() else {
            continue;
        };
        path_count += 1;
        match first_header {
            Some(first) if first != header => homogeneous = false,
            None => first_header = Some(header),
            _ => {}
        }
    }

    let Some(header) = first_header else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"pathCount\":");
    output.push_str(&path_count.to_string());
    output.push_str(",\"pointCount\":");
    output.push_str(&header.point_count().to_string());
    output.push_str(",\"byteCount\":");
    output.push_str(&header.byte_count().to_string());
    output.push_str(",\"flags\":");
    output.push_str(&header.flags().to_string());
    output.push_str(",\"flagsHex\":");
    push_json_string(output, &format!("0x{:08x}", header.flags()));
    output.push_str(",\"homogeneous\":");
    output.push_str(if homogeneous { "true" } else { "false" });
    output.push('}');
}

fn push_embedded_press_paint_state_transitions_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let mut ranges = Vec::new();
    let mut current_48_word0 = None;
    let mut current_70_word0 = None;
    let mut current_70_word3 = None;
    let mut current_82_word5 = None;

    for (path_index, path) in snapshot.vector_paths().iter().enumerate() {
        if let Some(value) = embedded_press_path_state_word(path, 0x48, 0) {
            current_48_word0 = Some(value);
        }
        if let Some(value) = embedded_press_path_state_word(path, 0x70, 0) {
            current_70_word0 = Some(value);
        }
        if let Some(value) = embedded_press_path_state_word(path, 0x70, 3) {
            current_70_word3 = Some(value);
        }
        if let Some(value) =
            embedded_press_path_state_word(path, EMBEDDED_PRESS_RECORD_PAINT_STATE_82, 5)
        {
            current_82_word5 = Some(value);
        }

        let key = (
            path.kind(),
            current_48_word0,
            current_70_word0,
            current_70_word3,
            current_82_word5,
        );
        match ranges.last_mut() {
            Some((_, end, known_key)) if *known_key == key => *end = path_index,
            _ => ranges.push((path_index, path_index, key)),
        }
    }

    output.push('[');
    for (range_index, (start, end, key)) in ranges.iter().enumerate() {
        if range_index > 0 {
            output.push(',');
        }
        let paths = &snapshot.vector_paths()[*start..=*end];
        let explicit_state_path_count = paths
            .iter()
            .filter(|path| !path.state_records().is_empty())
            .count();
        let texture_header_count = paths
            .iter()
            .filter(|path| path.texture_bezier_header().is_some())
            .count();

        output.push_str("{\"pathKind\":");
        push_json_string(output, key.0.as_str());
        output.push_str(",\"startPathIndex\":");
        output.push_str(&start.to_string());
        output.push_str(",\"endPathIndex\":");
        output.push_str(&end.to_string());
        output.push_str(",\"pathCount\":");
        output.push_str(&(end - start + 1).to_string());
        output.push_str(",\"explicitStatePathCount\":");
        output.push_str(&explicit_state_path_count.to_string());
        output.push_str(",\"inheritedStatePathCount\":");
        output.push_str(&(end - start + 1 - explicit_state_path_count).to_string());
        output.push_str(",\"textureBezierHeaderCount\":");
        output.push_str(&texture_header_count.to_string());
        output.push_str(",\"currentState\":{\"record48Word0\":");
        push_option_u32_hex_json(output, key.1);
        output.push_str(",\"record70Word0\":");
        push_option_u32_hex_json(output, key.2);
        output.push_str(",\"record70Word3\":");
        push_option_u32_hex_json(output, key.3);
        output.push_str(",\"record82Word5\":");
        push_option_u32_hex_json(output, key.4);
        output.push_str("},\"explicitStateValues\":{\"record48Word0\":");
        push_u32_hex_array_json(
            output,
            &embedded_press_path_state_word_values(paths, 0x48, 0),
        );
        output.push_str(",\"record70Word0\":");
        push_u32_hex_array_json(
            output,
            &embedded_press_path_state_word_values(paths, 0x70, 0),
        );
        output.push_str(",\"record70Word3\":");
        push_u32_hex_array_json(
            output,
            &embedded_press_path_state_word_values(paths, 0x70, 3),
        );
        output.push_str(",\"record82Word5\":");
        push_u32_hex_array_json(
            output,
            &embedded_press_path_state_word_values(paths, EMBEDDED_PRESS_RECORD_PAINT_STATE_82, 5),
        );
        output.push_str("},\"decoded\":false}");
    }
    output.push(']');
}

fn embedded_press_path_state_word(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    record_type: u32,
    word_index: usize,
) -> Option<u32> {
    path.state_records()
        .iter()
        .rev()
        .find(|record| record.record_type() == record_type)
        .and_then(|record| record.payload_le32_words().get(word_index).copied())
}

fn embedded_press_path_state_word_values(
    paths: &[ObjectEmbeddedPressVectorPathCandidate],
    record_type: u32,
    word_index: usize,
) -> Vec<u32> {
    paths
        .iter()
        .filter_map(|path| embedded_press_path_state_word(path, record_type, word_index))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn push_embedded_press_state_record_summary_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let mut type_counts = std::collections::BTreeMap::<u32, usize>::new();
    let mut state_record_count = 0usize;
    for path in snapshot.vector_paths() {
        for record in path.state_records() {
            state_record_count += 1;
            *type_counts.entry(record.record_type()).or_default() += 1;
        }
    }

    output.push_str("{\"pathCount\":");
    output.push_str(&snapshot.vector_paths().len().to_string());
    output.push_str(",\"stateRecordCount\":");
    output.push_str(&state_record_count.to_string());
    output.push_str(",\"recordTypes\":[");
    for (index, (record_type, count)) in type_counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"recordType\":");
        output.push_str(&record_type.to_string());
        output.push_str(",\"recordTypeHex\":");
        push_json_string(output, &format!("0x{record_type:08x}"));
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"paintState82Preview\":[");

    let mut preview_count = 0usize;
    for (path_index, path) in snapshot.vector_paths().iter().enumerate() {
        for (record_index, record) in path.state_records().iter().enumerate() {
            if record.record_type() != 0x82 || preview_count >= 8 {
                continue;
            }
            let words = record.payload_le32_words();
            if preview_count > 0 {
                output.push(',');
            }
            output.push_str("{\"pathIndex\":");
            output.push_str(&path_index.to_string());
            output.push_str(",\"pathKind\":");
            push_json_string(output, path.kind().as_str());
            output.push_str(",\"recordIndex\":");
            output.push_str(&record_index.to_string());
            output.push_str(",\"offset\":");
            output.push_str(&record.offset().to_string());
            output.push_str(",\"payloadWordCount\":");
            output.push_str(&words.len().to_string());
            output.push_str(",\"payloadLe32WordsPreview\":");
            let preview_len = words.len().min(8);
            push_u32_array_json(output, &words[..preview_len]);
            output.push_str(",\"word3Candidate\":");
            push_option_u32_json(output, words.get(3).copied());
            output.push_str(",\"word3CandidateHex\":");
            push_option_u32_hex_json(output, words.get(3).copied());
            output.push_str(",\"word5Candidate\":");
            push_option_u32_json(output, words.get(5).copied());
            output.push_str(",\"word5CandidateHex\":");
            push_option_u32_hex_json(output, words.get(5).copied());
            output.push_str(",\"decoded\":false}");
            preview_count += 1;
        }
    }
    output.push_str("],\"decoded\":false}");
}

fn push_object_visual_list_candidate_json(
    output: &mut String,
    visual_list: &ObjectVisualListCandidate,
) {
    output.push_str("{\"format\":\"BMDV\",\"declaredSize\":");
    output.push_str(&visual_list.declared_size().to_string());
    output.push_str(",\"magicOffset\":");
    output.push_str(&visual_list.magic_offset().to_string());
    output.push_str(",\"magic\":");
    push_json_string(output, visual_list.magic());
    output.push_str(",\"version\":");
    output.push_str(&visual_list.version().to_string());
    output.push_str(",\"flags\":");
    output.push_str(&visual_list.flags().to_string());
    output.push_str(",\"width\":");
    output.push_str(&visual_list.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&visual_list.height().to_string());
    output.push_str(",\"rowStride\":");
    output.push_str(&visual_list.row_stride().to_string());
    output.push_str(",\"bitDepth\":");
    output.push_str(&visual_list.bit_depth().to_string());
    output.push_str(",\"xPixelsPerMeter\":");
    output.push_str(&visual_list.x_pixels_per_meter().to_string());
    output.push_str(",\"yPixelsPerMeter\":");
    output.push_str(&visual_list.y_pixels_per_meter().to_string());
    output.push_str(",\"rleDataOffset\":");
    output.push_str(&visual_list.rle_data_offset().to_string());
    output.push_str(",\"rleDataLength\":");
    output.push_str(&visual_list.rle_data_len().to_string());
    output.push_str(",\"pixelCount\":");
    output.push_str(&visual_list.pixels().len().to_string());
    output.push_str(",\"rleEncoding\":\"bmp-rle8-like\",\"renderable\":true,\"decoded\":false}");
}

fn push_object_stream_ownership_candidate_json(
    output: &mut String,
    ownership: &ObjectStreamOwnershipCandidate,
) {
    output.push_str("{\"basis\":");
    push_json_string(output, ownership.basis());
    output.push_str(",\"family\":");
    push_json_string(output, ownership.family());
    output.push_str(",\"storagePath\":");
    if let Some(storage_path) = ownership.storage_path() {
        push_json_string(output, storage_path);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddingIndex\":");
    if let Some(index) = ownership.embedding_index() {
        output.push_str(&index.to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"streamRole\":");
    push_json_string(output, ownership.stream_role());
    output.push_str(",\"decoded\":false}");
}

fn push_object_stream_ownership_reference_candidate_json(
    output: &mut String,
    reference: &ObjectStreamOwnershipReferenceCandidate,
) {
    output.push_str("{\"targetPath\":");
    push_json_string(output, reference.target_path());
    output.push_str(",\"encoding\":");
    push_json_string(output, reference.encoding());
    output.push_str(",\"totalMatches\":");
    output.push_str(&reference.total_matches().to_string());
    output.push_str(",\"offsets\":");
    push_usize_array_json(output, reference.offsets());
    output.push_str(",\"decoded\":false}");
}

fn push_object_frame_reference_row_candidate_json(
    output: &mut String,
    row: &ObjectFrameReferenceRowCandidate,
) {
    output.push_str("{\"targetPath\":");
    push_json_string(output, row.target_path());
    output.push_str(",\"encoding\":");
    push_json_string(output, row.encoding());
    output.push_str(",\"stride\":");
    output.push_str(&row.stride().to_string());
    output.push_str(",\"fieldOffset\":");
    output.push_str(&row.field_offset().to_string());
    output.push_str(",\"offset\":");
    output.push_str(&row.offset().to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&row.row_start().to_string());
    output.push_str(",\"family\":");
    push_json_string(output, row.family());
    output.push_str(",\"rowHex\":");
    push_json_string(output, &hex(row.row()));
    output.push_str(",\"suffixLink\":");
    if let Some(link) = row.suffix_link() {
        output.push_str("{\"relation\":");
        push_json_string(output, link.relation());
        output.push_str(",\"suffixFamily\":");
        push_json_string(output, link.suffix_family());
        output.push_str(",\"matchedRowStart\":");
        output.push_str(&link.matched_row_start().to_string());
        output.push_str(",\"matchedRowIndex\":");
        output.push_str(&link.matched_row_index().to_string());
        output.push_str(",\"decoded\":false}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}
