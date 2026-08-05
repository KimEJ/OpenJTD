use super::*;

pub(super) const OBJECT_STREAM_PREFIX_PREVIEW_BYTES: usize = 16;

pub(super) const OBJECT_STREAM_REFERENCE_OFFSET_PREVIEW_LIMIT: usize = 16;

pub(super) const OBJECT_STREAM_REFERENCE_ROW_LIMIT: usize = 16;

pub(super) const OBJECT_FRAME_REFERENCE_ROW_CANDIDATES: &[ObjectFrameReferenceRowProjection] = &[
    ObjectFrameReferenceRowProjection {
        encoding: "u16-le",
        stride: 12,
        field_offset: 5,
    },
    ObjectFrameReferenceRowProjection {
        encoding: "u16-be",
        stride: 12,
        field_offset: 7,
    },
    ObjectFrameReferenceRowProjection {
        encoding: "u16-be",
        stride: 20,
        field_offset: 15,
    },
];

pub(super) const PAGE_FRAME_TITLE_OBJECT_TYPE: u16 = 1;

pub(super) const PAGE_FRAME_PATTERN_BAR_OBJECT_TYPE: u16 = 2;

pub(super) const PAGE_FRAME_PATTERN_BAR_BOTTOM_OBJECT_TYPE: u16 = 3;

pub(super) fn unknown_object_from_skipped_inline(
    segment: &SkippedInlineTextSegment,
) -> UnknownObject {
    UnknownObject::new(
        UnknownRecordKind::new(Some(DOCUMENT_TEXT_INLINE_START_TAG)),
        segment.raw_bytes().to_vec(),
    )
}

pub(super) fn object_stream_candidates_from_cfb(
    data: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectStreamCandidate>> {
    let Ok(entries) = inspect_cfb_entries(data) else {
        return Ok(Vec::new());
    };

    let paths = entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
        .map(|entry| entry.path())
        .collect::<BTreeSet<_>>();
    let mut candidates = Vec::new();
    let mut streams = Vec::new();
    for path in paths {
        let Ok(stream) = read_cfb_stream(data, path) else {
            continue;
        };
        if let Some(candidate) = classify_object_stream_candidate(path, &stream, budget)? {
            candidates.push(candidate);
        }
        streams.push((path.to_string(), stream));
    }
    attach_object_stream_ownership_references(&mut candidates, &streams);
    attach_object_stream_fdm_index_entries(&mut candidates, &streams, budget)?;
    attach_object_stream_fdm_text_index_entries(&mut candidates, &streams);
    Ok(candidates)
}

pub(super) fn object_frame_records_from_cfb(
    data: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectFrameRecordCandidate>> {
    let Ok(entries) = inspect_cfb_entries(data) else {
        return Ok(Vec::new());
    };

    let Some(entry) = entries.iter().find(|entry| {
        entry.kind() == EntryKind::Stream && entry.path().eq_ignore_ascii_case("/Frame")
    }) else {
        return Ok(Vec::new());
    };

    let Ok(stream) = read_cfb_stream(data, entry.path()) else {
        return Ok(Vec::new());
    };

    object_frame_records_from_stream(entry.path(), &stream, budget)
}

pub(super) fn object_frame_records_from_stream(
    path: &str,
    stream: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectFrameRecordCandidate>> {
    let Some(declared_count) =
        read_be16_at(stream, FRAME_RECORD_DECLARED_COUNT_OFFSET).map(usize::from)
    else {
        return Ok(Vec::new());
    };

    let record_bytes = declared_count
        .checked_mul(FRAME_RECORD_BYTES)
        .ok_or_else(record_bytes_overflow)?;
    let expected_len = FRAME_RECORD_HEADER_BYTES
        .checked_add(record_bytes)
        .ok_or_else(record_bytes_overflow)?;
    if stream.len() < expected_len {
        return Ok(Vec::new());
    }

    let mut records = Vec::new();
    for row_index in 0..declared_count {
        let row_offset = row_index
            .checked_mul(FRAME_RECORD_BYTES)
            .ok_or_else(record_bytes_overflow)?;
        let row_start = FRAME_RECORD_HEADER_BYTES
            .checked_add(row_offset)
            .ok_or_else(record_bytes_overflow)?;
        let row_end = row_start
            .checked_add(FRAME_RECORD_BYTES)
            .ok_or_else(record_bytes_overflow)?;
        let row = stream
            .get(row_start..row_end)
            .ok_or_else(record_bytes_overflow)?;
        budget.reserve_record(row.len())?;
        records.push(ObjectFrameRecordCandidate::new(
            path, row_index, row_start, row,
        ));
    }

    Ok(records)
}

pub(super) fn attach_object_stream_ownership_references(
    candidates: &mut [ObjectStreamCandidate],
    streams: &[(String, Vec<u8>)],
) {
    for candidate in candidates {
        let Some(embedding_index) = candidate
            .ownership_candidate()
            .and_then(ObjectStreamOwnershipCandidate::embedding_index)
        else {
            continue;
        };
        if candidate.image_payload_spans().is_empty() {
            continue;
        }

        let references =
            object_stream_ownership_references(candidate.path(), embedding_index, streams);
        let frame_rows = object_stream_frame_reference_rows(&references, streams);
        candidate.set_ownership_reference_candidates(references);
        candidate.set_frame_reference_row_candidates(frame_rows);
    }
}

pub(super) fn object_stream_ownership_references(
    source_path: &str,
    embedding_index: usize,
    streams: &[(String, Vec<u8>)],
) -> Vec<ObjectStreamOwnershipReferenceCandidate> {
    let patterns = object_stream_embedding_reference_patterns(embedding_index);
    let mut references = Vec::new();

    for (target_path, stream) in streams {
        if target_path == source_path || !is_object_reference_target_path(target_path) {
            continue;
        }

        for (encoding, pattern) in &patterns {
            let offsets = find_subslice_offsets(stream, pattern);
            if offsets.is_empty() {
                continue;
            }

            let total_matches = offsets.len();
            let offsets = offsets
                .into_iter()
                .take(OBJECT_STREAM_REFERENCE_OFFSET_PREVIEW_LIMIT)
                .collect();
            references.push(ObjectStreamOwnershipReferenceCandidate::new(
                target_path,
                *encoding,
                total_matches,
                offsets,
            ));
        }
    }

    references.sort_by(|left, right| {
        left.target_path()
            .cmp(right.target_path())
            .then_with(|| left.encoding().cmp(right.encoding()))
            .then_with(|| left.total_matches().cmp(&right.total_matches()))
    });
    references.truncate(OBJECT_STREAM_REFERENCE_ROW_LIMIT);
    references
}

pub(super) fn object_stream_frame_reference_rows(
    references: &[ObjectStreamOwnershipReferenceCandidate],
    streams: &[(String, Vec<u8>)],
) -> Vec<ObjectFrameReferenceRowCandidate> {
    let mut rows = Vec::new();

    for reference in references
        .iter()
        .filter(|reference| reference.target_path().eq_ignore_ascii_case("/Frame"))
    {
        let Some((_, target_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(reference.target_path()))
        else {
            continue;
        };

        for offset in reference.offsets() {
            for projection in OBJECT_FRAME_REFERENCE_ROW_CANDIDATES
                .iter()
                .filter(|projection| {
                    projection.encoding == reference.encoding()
                        && offset % projection.stride == projection.field_offset
                })
            {
                let pattern_len = object_reference_pattern_len(reference.encoding());
                if projection.field_offset + pattern_len > projection.stride {
                    continue;
                }
                let row_start = offset.saturating_sub(projection.field_offset);
                let Some(row_end) = row_start.checked_add(projection.stride) else {
                    continue;
                };
                let Some(row) = target_stream.get(row_start..row_end) else {
                    continue;
                };
                rows.push(ObjectFrameReferenceRowCandidate::new(
                    reference.target_path(),
                    projection.encoding,
                    projection.stride,
                    projection.field_offset,
                    ObjectFrameReferenceRowLocation::new(
                        *offset,
                        offset / projection.stride,
                        row_start,
                    ),
                    row.to_vec(),
                ));
            }
        }
    }

    attach_object_frame_row_suffix_links(&mut rows);
    rows
}

pub(super) fn object_reference_pattern_len(encoding: &str) -> usize {
    match encoding {
        "u16-le" | "u16-be" => 2,
        "u32-le" | "u32-be" => 4,
        _ => 1,
    }
}

pub(super) fn attach_object_frame_row_suffix_links(rows: &mut [ObjectFrameReferenceRowCandidate]) {
    let row12_records = rows
        .iter()
        .filter(|row| row.stride() == 12)
        .map(|row| {
            (
                row.row().to_vec(),
                row.family().to_string(),
                row.row_start(),
                row.row_index(),
            )
        })
        .collect::<Vec<_>>();

    for row in rows
        .iter_mut()
        .filter(|row| row.stride() == 20 && row.field_offset() == 15)
    {
        let Some(suffix) = row.row().get(row.row().len().saturating_sub(12)..) else {
            continue;
        };
        let Some((_, matched_family, matched_row_start, matched_row_index)) = row12_records
            .iter()
            .find(|(candidate_row, _, _, _)| candidate_row.as_slice() == suffix)
        else {
            continue;
        };
        row.set_suffix_link(ObjectFrameReferenceRowLink::new(
            "same-candidate",
            matched_family.as_str(),
            *matched_row_start,
            *matched_row_index,
        ));
    }
}

pub(super) fn classify_object_frame_reference_row(
    row: &[u8],
    encoding: &str,
    stride: usize,
    field_offset: usize,
) -> &'static str {
    let be16 = read_be16_fields(row);

    match (encoding, stride, field_offset) {
        ("u16-le", 12, 5)
            if be16.len() == 6
                && be16[1] == 0
                && be16[3] == 0
                && be16[4] <= 0x0010
                && be16[5] <= 0x0010 =>
        {
            "frame-index-flag-row12"
        }
        ("u16-le", 12, 5) => "frame-index-mixed-row12",
        ("u16-be", 12, 7)
            if be16.len() == 6
                && be16[0] == 0
                && be16[1] == 0
                && be16[2] == 0
                && be16[3] == 0
                && be16[5] == 0 =>
        {
            "frame-index-tail-zero-row12"
        }
        ("u16-be", 12, 7) if be16.len() == 6 && be16[1] == 0 && be16[3] == 0 && be16[5] == 0 => {
            "frame-index-tail-coordinate-row12"
        }
        ("u16-be", 12, 7) => "frame-index-tail-mixed-row12",
        ("u16-be", 20, 15) if be16.len() == 10 && be16[9] == 0 => "frame-index-tail-window20",
        ("u16-be", 20, 15) => "frame-index-mixed-window20",
        _ => "frame-index-unknown",
    }
}

pub(super) fn is_object_reference_target_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.contains("/figuredata/")
        || lower.ends_with("/figure")
        || lower.ends_with("/frame")
        || lower.ends_with("/layoutbox")
        || lower.ends_with("/pagemark")
        || lower.ends_with("/papermark")
}

pub(super) fn classify_object_stream_candidate(
    path: &str,
    stream: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Option<ObjectStreamCandidate>> {
    let mut reasons = Vec::new();
    push_object_path_reasons(path, &mut reasons);

    let image_signature_hits = image_signature_hits(stream, budget)?;
    let image_payload_spans = image_payload_spans(stream, &image_signature_hits, budget)?;
    let visual_list_candidate = visual_list_candidate_from_stream(path, stream);
    let figure_link_candidate = figure_link_candidate_from_stream(path, stream);
    let embedded_press_snapshot_candidate = embedded_press_snapshot_candidate_from_stream(stream);
    let fdm_text_candidates = fdm_text_candidates_from_stream(path, stream);
    let jsfart_art_candidate = jsfart_art_candidate_from_stream(path, stream);
    let jsfart_stream_profile_candidate =
        jsfart_stream_profile_candidate_from_stream(path, stream, jsfart_art_candidate.is_some());
    let jseq3_formula_candidate = jseq3_formula_candidate_from_stream(path, stream);
    if !image_signature_hits.is_empty() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::ImageSignature);
    }
    if embedded_press_snapshot_candidate.is_some() {
        push_unique_object_reason(
            &mut reasons,
            ObjectStreamCandidateReason::EmbeddedPressSnapshot,
        );
    }
    if !fdm_text_candidates.is_empty() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::FdmText);
    }
    if jsfart_art_candidate.is_some() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::JsfartArt);
    }
    if jseq3_formula_candidate.is_some() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::Jseq3Formula);
    }
    if figure_link_candidate.is_some() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::FigureLink);
    }

    let svg_offsets = svg_signature_offsets(stream);
    if !svg_offsets.is_empty() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::SvgSignature);
    }

    let so_offsets = find_subslice_offsets(stream, SO_RECORD_MARKER);
    if !so_offsets.is_empty() {
        push_unique_object_reason(&mut reasons, ObjectStreamCandidateReason::SoMarker);
    }

    if reasons.is_empty() {
        return Ok(None);
    }

    Ok(Some(ObjectStreamCandidate::new(
        path,
        stream.len(),
        ObjectStreamCandidateEvidence::new(
            reasons,
            image_signature_hits,
            image_payload_spans,
            visual_list_candidate,
            svg_offsets,
            so_offsets,
        )
        .with_figure_link_candidate(figure_link_candidate)
        .with_embedded_press_snapshot_candidate(embedded_press_snapshot_candidate)
        .with_fdm_text_candidates(fdm_text_candidates)
        .with_jsfart_stream_profile_candidate(jsfart_stream_profile_candidate)
        .with_jsfart_art_candidate(jsfart_art_candidate)
        .with_jseq3_formula_candidate(jseq3_formula_candidate),
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
    )))
}

pub(super) fn push_object_path_reasons(path: &str, reasons: &mut Vec<ObjectStreamCandidateReason>) {
    let lower = path.to_ascii_lowercase();
    let segments = lower
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &[
                "embeditems",
                "embedding",
                "jsfart",
                "compobj",
                "ole",
                "object",
                "bin",
            ],
        )
    }) {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::ObjectPath);
    }

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &[
                "image", "picture", "graphic", "bitmap", "png", "jpg", "jpeg", "gif", "bmp", "tif",
                "tiff", "wmf", "emf",
            ],
        )
    }) {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::ImagePath);
    }

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &["figure", "shape", "draw", "frame", "layoutbox", "svg"],
        )
    }) {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::ShapePath);
    }

    if segments.iter().any(|segment| {
        contains_any(segment, &["table", "cell", "tbl", "hyo"])
            && !contains_any(segment, &["positiontable", "style"])
    }) {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::TablePath);
    }

    if segments.contains(&"visuallist") {
        push_unique_object_reason(reasons, ObjectStreamCandidateReason::VisualListPath);
    }
}

pub(super) fn object_stream_ownership_candidate(
    path: &str,
) -> Option<ObjectStreamOwnershipCandidate> {
    let segments = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.len() >= 3
        && segments[0].eq_ignore_ascii_case("EmbedItems")
        && segments[1].starts_with("Embedding ")
    {
        let embedding_index = segments[1]
            .strip_prefix("Embedding ")
            .and_then(|value| value.parse::<usize>().ok());
        let storage_path = Some(format!("/EmbedItems/{}", segments[1]));
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "embed-items",
            storage_path,
            embedding_index,
            embedded_stream_role(segments[2]),
        ));
    }

    if segments.len() >= 3
        && segments[0].eq_ignore_ascii_case("FigureData")
        && segments[2].eq_ignore_ascii_case("FDMVector")
    {
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "figure-data",
            Some(format!("/{}/{}", segments[0], segments[1])),
            None,
            "fdm-vector",
        ));
    }

    let last = segments.last()?;
    if last.eq_ignore_ascii_case("Figure") {
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "figure",
            None,
            None,
            "figure-stream",
        ));
    }
    if last.eq_ignore_ascii_case("Frame") {
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "frame",
            None,
            None,
            "frame-stream",
        ));
    }
    if last.eq_ignore_ascii_case("LayoutBox") {
        return Some(ObjectStreamOwnershipCandidate::new(
            "stream-path",
            "layout-box",
            None,
            None,
            "layout-box-stream",
        ));
    }

    None
}

pub(super) fn push_unique_object_reason(
    reasons: &mut Vec<ObjectStreamCandidateReason>,
    reason: ObjectStreamCandidateReason,
) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

pub(super) fn default_object_bbox_json() -> String {
    "{\"pageIndex\":0,\"x\":0.0,\"y\":0.0,\"width\":0.0,\"height\":0.0}".to_string()
}

pub(super) fn object_stream_candidates_json(candidates: &[ObjectStreamCandidate]) -> String {
    let mut output = String::from("[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_stream_candidate_json(&mut output, candidate);
    }
    output.push(']');
    output
}

pub(super) fn object_frame_records_json(records: &[ObjectFrameRecordCandidate]) -> String {
    let mut output = String::from("[");
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_frame_record_candidate_json(&mut output, record);
    }
    output.push(']');
    output
}

pub(super) fn push_object_frame_record_candidate_json(
    output: &mut String,
    record: &ObjectFrameRecordCandidate,
) {
    output.push_str("{\"sourcePath\":");
    output.push_str(&json_string(record.source_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&record.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&record.row_start().to_string());
    output.push_str(",\"recordLen\":");
    output.push_str(&record.record_len().to_string());
    output.push_str(",\"recordKind\":");
    output.push_str(&record.record_kind().to_string());
    output.push_str(",\"recordKindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", record.record_kind())));
    output.push_str(",\"declaredRecordBytes\":");
    output.push_str(&record.declared_record_bytes().to_string());
    output.push_str(",\"objectId\":");
    output.push_str(&record.object_id().to_string());
    output.push_str(",\"objectType\":");
    output.push_str(&record.object_type().to_string());
    output.push_str(",\"objectTypeHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", record.object_type())));
    output.push_str(",\"geometry\":{\"x\":");
    output.push_str(&record.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&record.y().to_string());
    output.push_str(",\"width\":");
    output.push_str(&record.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&record.height().to_string());
    output.push_str(",\"cornerRadius\":");
    output.push_str(&record.corner_radius().to_string());
    output.push_str("},\"styleId\":");
    output.push_str(&record.style_id().to_string());
    output.push_str(",\"rowPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(record.row_prefix())));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_stream_candidate_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    output.push_str("{\"path\":");
    output.push_str(&json_string(candidate.path()));
    output.push_str(",\"size\":");
    output.push_str(&candidate.size().to_string());
    output.push_str(",\"reasons\":[");
    for (index, reason) in candidate.reasons().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(reason.as_str()));
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
    output.push_str("],\"fdmIndexEntries\":[");
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
    output.push_str("],\"fdmIndexSegmentBboxAxisPairGate\":");
    if let Some(gate) = fdm_index_segment_bbox_axis_pair_gate(candidate) {
        push_fdm_index_segment_bbox_axis_pair_gate_json(output, gate);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"fdmTextIndexEntries\":[");
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
        output.push_str(&json_string(hit.kind()));
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
    output.push_str(",\"jseq3Formula\":");
    if let Some(formula) = candidate.jseq3_formula_candidate() {
        push_object_jseq3_formula_candidate_json(output, formula);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"payloadPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(candidate.payload_prefix())));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_stream_ownership_candidate_json(
    output: &mut String,
    ownership: &ObjectStreamOwnershipCandidate,
) {
    output.push_str("{\"basis\":");
    output.push_str(&json_string(ownership.basis()));
    output.push_str(",\"family\":");
    output.push_str(&json_string(ownership.family()));
    output.push_str(",\"storagePath\":");
    if let Some(storage_path) = ownership.storage_path() {
        output.push_str(&json_string(storage_path));
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
    output.push_str(&json_string(ownership.stream_role()));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_stream_ownership_reference_candidate_json(
    output: &mut String,
    reference: &ObjectStreamOwnershipReferenceCandidate,
) {
    output.push_str("{\"targetPath\":");
    output.push_str(&json_string(reference.target_path()));
    output.push_str(",\"encoding\":");
    output.push_str(&json_string(reference.encoding()));
    output.push_str(",\"totalMatches\":");
    output.push_str(&reference.total_matches().to_string());
    output.push_str(",\"offsets\":");
    push_usize_array_json(output, reference.offsets());
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_frame_reference_row_candidate_json(
    output: &mut String,
    row: &ObjectFrameReferenceRowCandidate,
) {
    output.push_str("{\"targetPath\":");
    output.push_str(&json_string(row.target_path()));
    output.push_str(",\"encoding\":");
    output.push_str(&json_string(row.encoding()));
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
    output.push_str(&json_string(row.family()));
    output.push_str(",\"rowHex\":");
    output.push_str(&json_string(&hex_bytes(row.row())));
    output.push_str(",\"suffixLink\":");
    if let Some(link) = row.suffix_link() {
        output.push_str("{\"relation\":");
        output.push_str(&json_string(link.relation()));
        output.push_str(",\"suffixFamily\":");
        output.push_str(&json_string(link.suffix_family()));
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
