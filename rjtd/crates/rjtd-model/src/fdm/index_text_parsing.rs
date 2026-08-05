use super::*;
use crate::*;

pub(crate) fn attach_object_stream_fdm_index_entries(
    candidates: &mut [ObjectStreamCandidate],
    streams: &[(String, Vec<u8>)],
    budget: &mut ResourceBudget,
) -> Result<()> {
    for candidate in candidates {
        if fdm_index_path_for_vector(candidate.path()).is_none() {
            continue;
        }
        let Some((_, vector_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(candidate.path()))
        else {
            continue;
        };
        candidate.set_fdm_raw_vector_segments(fdm_raw_vector_segment_candidates(vector_stream));
        candidate.set_fdm_raw_vector_commands(fdm_raw_vector_command_candidates(vector_stream));
        let Some((actual_index_path, index_stream)) =
            fdm_index_stream_for_vector(candidate.path(), vector_stream.len(), streams)
        else {
            continue;
        };

        let all_entries = parse_fdm_index_entries(index_stream, vector_stream.len());
        let entries = fdm_index_declared_entries(index_stream, &all_entries);
        if entries.is_empty() {
            continue;
        }
        let vector_hits = image_signature_hits(vector_stream, budget)?;
        let mut fdm_entries = Vec::new();
        for entry in entries {
            let segment = fdm_vector_segment(entry.vector_offset, entries, vector_stream);
            let segment_hits =
                fdm_segment_signature_hits(&vector_hits, segment.start, segment.end, budget)?;
            let relative_hits = fdm_relative_signature_hits(&segment_hits, segment.start, budget)?;
            let vector_prefix = vector_stream
                .get(segment.start..segment.end)
                .unwrap_or_default();
            let vector_commands = fdm_vector_command_candidates(vector_prefix, segment.start);
            let connector_candidates = fdm_connector_candidates(&vector_commands);

            fdm_entries.push(ObjectFdmIndexEntryCandidate {
                index_path: actual_index_path.clone(),
                vector_path: candidate.path().to_string(),
                row_index: entry.row_index,
                index_offset: entry.index_offset,
                vector_offset: entry.vector_offset,
                next_vector_offset: segment.end,
                vector_len: segment.end.saturating_sub(segment.start),
                kind: entry.kind,
                bbox: ObjectFdmIndexBbox::new(entry.left, entry.top, entry.right, entry.bottom),
                valid_vector_offset: entry.valid_vector_offset,
                vector_prefix: vector_prefix
                    [..vector_prefix.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
                    .to_vec(),
                image_signature_hits: segment_hits,
                segment_image_signature_hits: relative_hits,
                vector_commands,
                connector_candidates,
            });
        }
        candidate.set_fdm_index_entry_candidates(fdm_entries);
    }
    Ok(())
}

pub(crate) fn attach_object_stream_fdm_text_index_entries(
    candidates: &mut [ObjectStreamCandidate],
    streams: &[(String, Vec<u8>)],
) {
    for candidate in candidates {
        let Some(index_path) = fdm_index_path_for_text(candidate.path()) else {
            continue;
        };
        let Some((_, text_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(candidate.path()))
        else {
            continue;
        };
        let Some((actual_index_path, index_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(&index_path))
        else {
            continue;
        };

        let entries = parse_fdm_text_index_entries(index_stream, text_stream);
        let fdm_text_entries = entries
            .iter()
            .map(|entry| {
                let text_record_prefix = text_stream
                    .get(entry.text_record_offset..)
                    .unwrap_or_default()[..text_stream
                    .len()
                    .saturating_sub(entry.text_record_offset)
                    .min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
                    .to_vec();
                ObjectFdmTextIndexEntryCandidate {
                    index_path: actual_index_path.clone(),
                    text_path: candidate.path().to_string(),
                    row_index: entry.row_index,
                    index_offset: entry.index_offset,
                    text_record_offset: entry.text_record_offset,
                    kind: entry.kind,
                    bbox: entry.bbox,
                    text_record_bbox: fdm_text_candidate_bbox(
                        text_stream,
                        entry.text_record_offset,
                    ),
                    valid_text_record_offset: true,
                    text_record_prefix,
                }
            })
            .collect::<Vec<_>>();
        if !fdm_text_entries.is_empty() {
            candidate.set_fdm_text_index_entry_candidates(fdm_text_entries);
        }
    }
}

pub(crate) fn fdm_text_record_marker_at(stream: &[u8], offset: usize) -> Option<[u8; 4]> {
    let marker: [u8; 4] = stream
        .get(offset..offset.saturating_add(4))?
        .try_into()
        .ok()?;
    (marker[0] == 0x01 && marker[1] == 0x00 && marker[3] == 0x60).then_some(marker)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FdmIndexEntry {
    pub(crate) row_index: usize,
    pub(crate) index_offset: usize,
    pub(crate) vector_offset: usize,
    pub(crate) kind: u16,
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
    pub(crate) valid_vector_offset: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FdmTextIndexEntry {
    pub(crate) row_index: usize,
    pub(crate) index_offset: usize,
    pub(crate) text_record_offset: usize,
    pub(crate) kind: u16,
    pub(crate) bbox: ObjectFdmIndexBbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FdmVectorSegment {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FdmProjectionViewport {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

pub(crate) fn fdm_index_path_for_vector(vector_path: &str) -> Option<String> {
    if !vector_path
        .get(vector_path.len().saturating_sub("/FDMVector".len())..)?
        .eq_ignore_ascii_case("/FDMVector")
    {
        return None;
    }
    vector_path
        .get(..vector_path.len().saturating_sub("/FDMVector".len()))
        .map(|prefix| format!("{prefix}/FDMIndex"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FdmIndexVectorPairScore {
    pub(crate) valid_declared_rows: usize,
    pub(crate) declared_rows: usize,
    pub(crate) invalid_declared_rows: usize,
}

impl FdmIndexVectorPairScore {
    pub(crate) fn compare(self, other: Self) -> Ordering {
        self.valid_declared_rows
            .cmp(&other.valid_declared_rows)
            .then_with(|| other.invalid_declared_rows.cmp(&self.invalid_declared_rows))
            .then_with(|| self.declared_rows.cmp(&other.declared_rows))
    }
}

pub(crate) fn fdm_index_stream_for_vector<'a>(
    vector_path: &str,
    vector_len: usize,
    streams: &'a [(String, Vec<u8>)],
) -> Option<(&'a String, &'a Vec<u8>)> {
    let exact_index_path = fdm_index_path_for_vector(vector_path)?;
    if let Some((path, stream)) = streams
        .iter()
        .find(|(path, _)| path.eq_ignore_ascii_case(&exact_index_path))
        && fdm_index_vector_pair_score(stream, vector_len).is_some()
    {
        return Some((path, stream));
    }

    let mut best: Option<(&String, &Vec<u8>, FdmIndexVectorPairScore)> = None;
    let mut tied_best = false;
    for (path, stream) in streams
        .iter()
        .filter(|(path, _)| stream_path_ends_with(path, "/FDMIndex"))
    {
        let Some(score) = fdm_index_vector_pair_score(stream, vector_len) else {
            continue;
        };
        match best {
            Some((_, _, best_score)) => match score.compare(best_score) {
                Ordering::Greater => {
                    best = Some((path, stream, score));
                    tied_best = false;
                }
                Ordering::Equal => tied_best = true,
                Ordering::Less => {}
            },
            None => {
                best = Some((path, stream, score));
                tied_best = false;
            }
        }
    }

    if tied_best {
        return None;
    }
    best.map(|(path, stream, _)| (path, stream))
}

pub(crate) fn fdm_index_vector_pair_score(
    index_stream: &[u8],
    vector_len: usize,
) -> Option<FdmIndexVectorPairScore> {
    let all_entries = parse_fdm_index_entries(index_stream, vector_len);
    let entries = fdm_index_declared_entries(index_stream, &all_entries);
    if entries.is_empty() {
        return None;
    }
    let valid_declared_rows = entries
        .iter()
        .filter(|entry| entry.valid_vector_offset)
        .count();
    if valid_declared_rows == 0 {
        return None;
    }
    Some(FdmIndexVectorPairScore {
        valid_declared_rows,
        declared_rows: entries.len(),
        invalid_declared_rows: entries.len().saturating_sub(valid_declared_rows),
    })
}

pub(crate) fn fdm_index_path_for_text(text_path: &str) -> Option<String> {
    if !text_path
        .get(text_path.len().saturating_sub("/FDMText".len())..)?
        .eq_ignore_ascii_case("/FDMText")
    {
        return None;
    }
    text_path
        .get(..text_path.len().saturating_sub("/FDMText".len()))
        .map(|prefix| format!("{prefix}/FDMIndex"))
}

pub(crate) fn fdm_index_declared_entries<'a>(
    index_stream: &[u8],
    entries: &'a [FdmIndexEntry],
) -> &'a [FdmIndexEntry] {
    if !index_stream.starts_with(&[0x03, 0x0b, 0x00, 0x01]) {
        return &[];
    }

    let Some(count) = read_be16_at(index_stream, FDM_INDEX_DECLARED_COUNT_OFFSET).map(usize::from)
    else {
        return &[];
    };
    if count > entries.len() {
        return &[];
    }

    &entries[..count]
}

pub(crate) fn parse_fdm_index_entries(
    index_stream: &[u8],
    vector_len: usize,
) -> Vec<FdmIndexEntry> {
    if index_stream.len() < FDM_INDEX_HEADER_BYTES {
        return Vec::new();
    }

    let entry_bytes = index_stream.len() - FDM_INDEX_HEADER_BYTES;
    let entry_count = entry_bytes / FDM_INDEX_ENTRY_BYTES;
    let mut entries = Vec::with_capacity(entry_count);
    for row_index in 0..entry_count {
        let index_offset = FDM_INDEX_HEADER_BYTES + row_index * FDM_INDEX_ENTRY_BYTES;
        let Some(vector_offset) = read_be32_at(index_stream, index_offset) else {
            continue;
        };
        let Some(kind) = read_be16_at(index_stream, index_offset + 4) else {
            continue;
        };
        let Some(left) = read_i32_be_at(index_stream, index_offset + 6) else {
            continue;
        };
        let Some(top) = read_i32_be_at(index_stream, index_offset + 10) else {
            continue;
        };
        let Some(right) = read_i32_be_at(index_stream, index_offset + 14) else {
            continue;
        };
        let Some(bottom) = read_i32_be_at(index_stream, index_offset + 18) else {
            continue;
        };
        let vector_offset = vector_offset as usize;
        entries.push(FdmIndexEntry {
            row_index,
            index_offset,
            vector_offset,
            kind,
            left,
            top,
            right,
            bottom,
            valid_vector_offset: vector_offset < vector_len,
        });
    }
    entries
}

pub(crate) fn parse_fdm_text_index_entries(
    index_stream: &[u8],
    text_stream: &[u8],
) -> Vec<FdmTextIndexEntry> {
    let mut entries = Vec::new();
    let mut offset = 0usize;
    while offset + FDM_INDEX_ENTRY_BYTES <= index_stream.len() {
        let Some(text_record_offset) =
            read_be32_at(index_stream, offset).map(|value| value as usize)
        else {
            offset += 1;
            continue;
        };
        let Some(kind) = read_be16_at(index_stream, offset + 4) else {
            offset += 1;
            continue;
        };
        if kind != FDM_TEXT_EXPANDED_INDEX_KIND
            || fdm_text_record_marker_at(text_stream, text_record_offset).is_none()
        {
            offset += 1;
            continue;
        }
        let Some(left) = read_i32_be_at(index_stream, offset + 6) else {
            offset += 1;
            continue;
        };
        let Some(top) = read_i32_be_at(index_stream, offset + 10) else {
            offset += 1;
            continue;
        };
        let Some(right) = read_i32_be_at(index_stream, offset + 14) else {
            offset += 1;
            continue;
        };
        let Some(bottom) = read_i32_be_at(index_stream, offset + 18) else {
            offset += 1;
            continue;
        };
        entries.push(FdmTextIndexEntry {
            row_index: entries.len(),
            index_offset: offset,
            text_record_offset,
            kind,
            bbox: ObjectFdmIndexBbox::new(left, top, right, bottom),
        });
        offset += FDM_INDEX_ENTRY_BYTES;
    }
    entries
}

pub(crate) fn fdm_vector_segment(
    vector_offset: usize,
    entries: &[FdmIndexEntry],
    vector_stream: &[u8],
) -> FdmVectorSegment {
    let start = vector_offset.min(vector_stream.len());
    let end = entries
        .iter()
        .filter_map(|entry| {
            (entry.vector_offset > vector_offset && entry.vector_offset <= vector_stream.len())
                .then_some(entry.vector_offset)
        })
        .min()
        .unwrap_or(vector_stream.len());
    FdmVectorSegment { start, end }
}

pub(crate) fn fdm_segment_signature_hits(
    vector_hits: &[ObjectImageSignatureHit],
    start: usize,
    end: usize,
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectImageSignatureHit>> {
    let mut hits = Vec::new();
    for hit in vector_hits
        .iter()
        .filter(|hit| hit.offset() >= start && hit.offset() < end)
    {
        reserve_image_signature_candidate(budget, hit.kind())?;
        hits.push(hit.clone());
    }
    Ok(hits)
}

pub(crate) fn fdm_relative_signature_hits(
    segment_hits: &[ObjectImageSignatureHit],
    segment_start: usize,
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectImageSignatureHit>> {
    let mut hits = Vec::new();
    for hit in segment_hits {
        reserve_image_signature_candidate(budget, hit.kind())?;
        hits.push(ObjectImageSignatureHit::new(
            hit.kind(),
            hit.offset().saturating_sub(segment_start),
        ));
    }
    Ok(hits)
}

pub(crate) fn fdm_text_candidates_from_stream(
    path: &str,
    stream: &[u8],
) -> Vec<ObjectFdmTextCandidate> {
    if !path.ends_with("/FDMText") {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    let marker_offsets = fdm_text_record_marker_offsets(stream);
    for (index, marker_offset) in marker_offsets.iter().copied().enumerate() {
        let next_record_offset = marker_offsets
            .get(index + 1)
            .copied()
            .unwrap_or(stream.len());
        let Some(marker) = stream.get(marker_offset..marker_offset.saturating_add(4)) else {
            continue;
        };
        let decoded = if marker == FDM_TEXT_RECORD_MARKER {
            fdm_text_candidate_legacy_text(stream, marker_offset, next_record_offset)
        } else if marker == FDM_TEXT_EXPANDED_RECORD_MARKER {
            fdm_text_candidate_expanded_text(stream, marker_offset, next_record_offset)
        } else {
            continue;
        };
        let Some((text, text_offset, raw_text)) = decoded else {
            continue;
        };
        if text.trim().is_empty() {
            continue;
        }

        candidates.push(ObjectFdmTextCandidate::new(
            text,
            text_offset,
            marker_offset,
            raw_text,
            fdm_text_candidate_bbox(stream, marker_offset),
        ));
    }
    candidates
}

pub(crate) fn fdm_text_mirror_anchor_agreements(
    candidates: &[ObjectStreamCandidate],
) -> Vec<FdmTextMirrorAnchorAgreement> {
    let mut agreements = Vec::new();
    for indexed in candidates {
        let indexed_texts = indexed.fdm_text_candidates();
        if indexed_texts.is_empty()
            || indexed.fdm_text_index_entry_candidates().len() != indexed_texts.len()
            || indexed_texts.iter().any(|text| text.bbox().is_none())
        {
            continue;
        }

        let indexed_record_offset_agreement = indexed
            .fdm_text_index_entry_candidates()
            .iter()
            .zip(indexed_texts)
            .all(|(entry, text)| {
                entry.valid_text_record_offset()
                    && entry.text_path() == indexed.path()
                    && entry.text_record_offset() == text.marker_offset()
            });
        let indexed_record_bbox_agreement = indexed
            .fdm_text_index_entry_candidates()
            .iter()
            .zip(indexed_texts)
            .all(|(entry, text)| text.bbox().is_some() && entry.text_record_bbox() == text.bbox());
        if !indexed_record_offset_agreement || !indexed_record_bbox_agreement {
            continue;
        }

        for mirrored in candidates {
            if indexed.path() == mirrored.path() {
                continue;
            }
            let mirrored_texts = mirrored.fdm_text_candidates();
            let ordered_text_agreement = indexed_texts.len() == mirrored_texts.len()
                && indexed_texts
                    .iter()
                    .zip(mirrored_texts)
                    .all(|(left, right)| left.text() == right.text());
            let ordered_record_bbox_agreement = indexed_texts.len() == mirrored_texts.len()
                && indexed_texts
                    .iter()
                    .zip(mirrored_texts)
                    .all(|(left, right)| {
                        left.bbox().is_some()
                            && right.bbox().is_some()
                            && left.bbox() == right.bbox()
                    });
            if !ordered_text_agreement || !ordered_record_bbox_agreement {
                continue;
            }

            agreements.push(FdmTextMirrorAnchorAgreement::new(
                indexed.path(),
                mirrored.path(),
                indexed_texts.len(),
                ordered_text_agreement,
                ordered_record_bbox_agreement,
                indexed_record_offset_agreement,
                indexed_record_bbox_agreement,
            ));
        }
    }
    agreements
}

pub(crate) fn fdm_text_record_marker_offsets(stream: &[u8]) -> Vec<usize> {
    let mut offsets = find_subslice_offsets(stream, FDM_TEXT_RECORD_MARKER);
    offsets.extend(find_subslice_offsets(
        stream,
        FDM_TEXT_EXPANDED_RECORD_MARKER,
    ));
    offsets.sort_unstable();
    offsets.dedup();
    offsets
}

pub(crate) fn figure_link_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectFigureLinkCandidate> {
    let lower = path.to_ascii_lowercase();
    if !lower.contains("/figuredata/") || !lower.ends_with("/link") {
        return None;
    }
    if stream.len() < FIGURE_LINK_HEADER_BYTES + FIGURE_LINK_ROW_BYTES {
        return None;
    }
    let row_payload_len = stream.len().checked_sub(FIGURE_LINK_HEADER_BYTES)?;
    if row_payload_len % FIGURE_LINK_ROW_BYTES != 0 {
        return None;
    }

    let declared_row_count_candidate = read_be16_at(stream, 6);
    let row_count = row_payload_len / FIGURE_LINK_ROW_BYTES;
    if row_count == 0 || declared_row_count_candidate.map(usize::from) != Some(row_count) {
        return None;
    }

    let mut rows = Vec::with_capacity(row_count);
    for row_index in 0..row_count {
        let row_start = FIGURE_LINK_HEADER_BYTES + row_index * FIGURE_LINK_ROW_BYTES;
        let row_end = row_start + FIGURE_LINK_ROW_BYTES;
        let row = stream.get(row_start..row_end)?;
        let relation_kind = read_be16_at(row, FIGURE_LINK_RELATION_KIND_CANDIDATE_OFFSET)?;
        if relation_kind != FIGURE_LINK_RELATION_KIND_CANDIDATE {
            return None;
        }
        rows.push(ObjectFigureLinkRowCandidate::new(row_index, row_start, row));
    }

    Some(ObjectFigureLinkCandidate::new(
        read_be16_fields(&stream[..FIGURE_LINK_HEADER_BYTES]),
        declared_row_count_candidate,
        FIGURE_LINK_ROW_BYTES,
        rows,
    ))
}

pub(crate) fn fdm_text_candidate_legacy_text(
    stream: &[u8],
    marker_offset: usize,
    next_record_offset: usize,
) -> Option<(String, usize, Vec<u8>)> {
    if next_record_offset <= marker_offset + FDM_TEXT_RECORD_MARKER.len() {
        return None;
    };
    let text_end = next_record_offset.checked_sub(FDM_TEXT_RECORD_TRAILER.len())?;
    if stream.get(text_end..next_record_offset) != Some(FDM_TEXT_RECORD_TRAILER.as_slice()) {
        return None;
    }
    let search_start = marker_offset.max(text_end.saturating_sub(FDM_TEXT_RECORD_BACKSCAN_BYTES));
    let delimiter_offset = (search_start..text_end).rev().find(|offset| {
        stream.get(*offset..offset.saturating_add(FDM_TEXT_RECORD_TEXT_DELIMITER.len()))
            == Some(FDM_TEXT_RECORD_TEXT_DELIMITER.as_slice())
    })?;
    let text_offset = delimiter_offset + FDM_TEXT_RECORD_TEXT_DELIMITER.len();
    let raw_text = stream.get(text_offset..text_end)?;
    let text = decode_fdm_text_bytes(raw_text)?;
    Some((text, text_offset, raw_text.to_vec()))
}

pub(crate) fn fdm_text_candidate_expanded_text(
    stream: &[u8],
    marker_offset: usize,
    next_marker_offset: usize,
) -> Option<(String, usize, Vec<u8>)> {
    let declared_len = read_be32_at(
        stream,
        marker_offset + FDM_TEXT_RECORD_DECLARED_LENGTH_OFFSET,
    )
    .map(|value| value as usize)?;
    let declared_end = marker_offset.checked_add(declared_len)?;
    let record_end = if declared_len > FDM_TEXT_EXPANDED_COUNT_OFFSET_FROM_MARKER
        && declared_end <= stream.len()
    {
        declared_end
    } else {
        next_marker_offset
    };
    let count = read_be16_at(
        stream,
        marker_offset + FDM_TEXT_EXPANDED_COUNT_OFFSET_FROM_MARKER,
    )
    .map(usize::from)?;
    let text_len = count.checked_sub(1)?.checked_mul(2)?;
    let mut matches = Vec::new();
    for delimiter_offset in marker_offset..record_end.saturating_sub(FDM_TEXT_RECORD_TRAILER.len())
    {
        if stream.get(
            delimiter_offset..delimiter_offset.saturating_add(FDM_TEXT_RECORD_TEXT_DELIMITER.len()),
        ) != Some(FDM_TEXT_RECORD_TEXT_DELIMITER.as_slice())
        {
            continue;
        }
        let text_offset = delimiter_offset + FDM_TEXT_RECORD_TEXT_DELIMITER.len();
        let Some(text_end) = text_offset.checked_add(text_len) else {
            continue;
        };
        let Some(trailer_end) = text_end.checked_add(FDM_TEXT_RECORD_TRAILER.len()) else {
            continue;
        };
        if trailer_end > record_end {
            continue;
        }
        if stream.get(text_end..trailer_end) == Some(FDM_TEXT_RECORD_TRAILER.as_slice()) {
            matches.push((text_offset, text_end));
        }
    }
    if matches.len() != 1 {
        return None;
    }
    let (text_offset, text_end) = matches[0];
    let raw_text = stream.get(text_offset..text_end)?;
    let text = decode_fdm_text_utf16be(raw_text)?;
    Some((text, text_offset, raw_text.to_vec()))
}

pub(crate) fn fdm_text_candidate_bbox(
    stream: &[u8],
    marker_offset: usize,
) -> Option<ObjectFdmIndexBbox> {
    let offset = marker_offset.saturating_add(FDM_TEXT_RECORD_BBOX_OFFSET_FROM_MARKER);
    let left = read_i32_be_at(stream, offset)?;
    let top = read_i32_be_at(stream, offset + 4)?;
    let right = read_i32_be_at(stream, offset + 8)?;
    let bottom = read_i32_be_at(stream, offset + 12)?;
    (left != right && top != bottom).then_some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

pub(crate) fn decode_fdm_text_bytes(bytes: &[u8]) -> Option<String> {
    let mut output = String::new();
    let mut offset = 0usize;
    while offset < bytes.len() {
        if bytes[offset] == 0 {
            let unit = *bytes.get(offset + 1)?;
            if unit >= 0x20 || unit == b'\r' {
                if unit != b'\r' {
                    output.push(char::from(unit));
                }
                offset += 2;
                continue;
            }
        }
        let first = bytes[offset];
        let second = *bytes.get(offset + 1)?;
        output.push(decode_fdm_text_shift_jis_pair(first, second)?);
        offset += 2;
    }
    Some(output)
}

pub(crate) fn decode_fdm_text_utf16be(bytes: &[u8]) -> Option<String> {
    let mut output = String::new();
    let mut chunks = bytes.chunks_exact(2);
    for chunk in &mut chunks {
        let code_unit = u16::from_be_bytes([chunk[0], chunk[1]]);
        if code_unit == u16::from(b'\r') {
            continue;
        }
        output.push(char::from_u32(u32::from(code_unit))?);
    }
    chunks.remainder().is_empty().then_some(output)
}

pub(crate) fn decode_fdm_text_shift_jis_pair(first: u8, second: u8) -> Option<char> {
    match (first, second) {
        (0x81, 0x8b) => Some('°'),
        (0x82, 0x60..=0x62) => char::from_u32('Ａ' as u32 + u32::from(second - 0x60)),
        (0x82, 0x4f..=0x58) => char::from_u32('０' as u32 + u32::from(second - 0x4f)),
        (0x82, 0x6e) => Some('Ｏ'),
        (0x82, 0x98) => Some('ｘ'),
        (0x87, 0x70) => Some('㎝'),
        _ => None,
    }
}

pub(crate) fn project_fdm_single_page_diagram(
    document: &Document,
    pages: &mut Vec<Vec<PageTextLine>>,
) -> bool {
    if !document_has_shanai_lan_fdm_command_evidence(document) {
        return false;
    }

    if pages.is_empty() {
        pages.push(Vec::new());
    } else {
        pages.truncate(1);
    }
    true
}
