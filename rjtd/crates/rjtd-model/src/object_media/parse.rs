use super::*;
use crate::*;

pub(crate) fn object_embedding_frames_from_cfb(
    data: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectEmbeddingFrameCandidate>> {
    let Ok(stream) = read_cfb_stream(data, EMBEDDING_INFO_PATH) else {
        return Ok(Vec::new());
    };

    object_embedding_frames_from_stream(EMBEDDING_INFO_PATH, &stream, budget)
}

pub(crate) fn object_embedding_frames_from_stream(
    path: &str,
    stream: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectEmbeddingFrameCandidate>> {
    let Some(declared_count) = read_le32_at(stream, 0).map(|value| value as usize) else {
        return Ok(Vec::new());
    };

    let mut frames = Vec::new();
    let mut cursor = EMBEDDING_INFO_HEADER_BYTES;
    for row_index in 0..declared_count {
        let Some(class_len_offset) = cursor.checked_add(EMBEDDING_INFO_CLASS_LENGTH_OFFSET) else {
            break;
        };
        let Some(class_len) = read_le32_at(stream, class_len_offset).map(|value| value as usize)
        else {
            break;
        };
        let Some(class_start) = cursor.checked_add(EMBEDDING_INFO_CLASS_START_OFFSET) else {
            break;
        };
        let Some(class_end) = class_start.checked_add(class_len) else {
            break;
        };
        let Some(row_end) = class_end.checked_add(EMBEDDING_INFO_TRAILING_BYTES) else {
            break;
        };
        let Some(row) = stream.get(cursor..row_end) else {
            break;
        };
        let Some(class_bytes) = stream.get(class_start..class_end) else {
            break;
        };
        let trailing = &stream[class_end..row_end];
        budget.reserve_record(row.len())?;
        let Some(class_name) = decode_utf16le_c_string(class_bytes) else {
            break;
        };
        if class_name.is_empty() || class_len == 0 || class_len % 2 != 0 {
            break;
        }
        let Some(frame) =
            ObjectEmbeddingFrameCandidate::new(path, row_index, cursor, row, class_name, trailing)
        else {
            break;
        };
        if embedding_frame_candidate_is_plausible(&frame) {
            frames.push(frame);
        }
        cursor = row_end;
    }

    Ok(frames)
}

pub(crate) fn embedding_frame_candidate_is_plausible(
    frame: &ObjectEmbeddingFrameCandidate,
) -> bool {
    frame.embedding_index() > 0
        && frame.frame_ref() > 0
        && frame.frame_width() > 0
        && frame.frame_height() > 0
        && frame.frame_width() <= 200_000
        && frame.frame_height() <= 200_000
        && frame.class_name().chars().all(|character| {
            character == '.'
                || character == '_'
                || character == '-'
                || character.is_ascii_alphanumeric()
        })
}

pub(crate) fn object_stream_embedding_reference_patterns(
    embedding_index: usize,
) -> Vec<(&'static str, Vec<u8>)> {
    let mut patterns = Vec::new();
    if let Ok(index) = u16::try_from(embedding_index) {
        patterns.push(("u16-le", index.to_le_bytes().to_vec()));
        patterns.push(("u16-be", index.to_be_bytes().to_vec()));
    }
    if let Ok(index) = u32::try_from(embedding_index) {
        patterns.push(("u32-le", index.to_le_bytes().to_vec()));
        patterns.push(("u32-be", index.to_be_bytes().to_vec()));
    }
    patterns
}

pub(crate) fn jsfart_stream_profile_candidate_from_stream(
    path: &str,
    stream: &[u8],
    structured_art_candidate_present: bool,
) -> Option<ObjectJsfartStreamProfileCandidate> {
    if !path.ends_with("/JSFart2Contents") {
        return None;
    }

    let header_prefix = stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec();
    let magic_family_hex = hex_bytes(&stream[..stream.len().min(2)]);
    let preview = utf16le_printable_preview(&header_prefix);
    let magic_family = jsfart_stream_magic_family(stream, &preview);
    let render_promotion_blocked_reason = if structured_art_candidate_present {
        "structured-jsfart-art-still-paint-authority-unproven"
    } else {
        "jsfart-variant-layout-undecoded"
    };

    Some(ObjectJsfartStreamProfileCandidate::new(
        magic_family,
        magic_family_hex,
        0,
        preview,
        header_prefix,
        structured_art_candidate_present,
        render_promotion_blocked_reason,
    ))
}

pub(crate) fn jsfart_stream_magic_family(stream: &[u8], utf16le_preview: &str) -> &'static str {
    if stream.starts_with(JSFART2_CONTENTS_MAGIC_UTF16LE) {
        "mstudio-ocx-utf16le"
    } else if utf16le_preview.starts_with("JSFART.") {
        "jsfart-object-utf16le"
    } else if stream.get(..2).is_some_and(|prefix| prefix == [0x00, 0x00]) {
        "zero-prefix"
    } else if !utf16le_preview.is_empty() {
        "utf16le-text-prefix"
    } else {
        "binary-prefix"
    }
}

pub(crate) fn jsfart_art_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectJsfartArtCandidate> {
    if !path.ends_with("/JSFart2Contents") {
        return None;
    }
    if stream.get(..JSFART2_CONTENTS_MAGIC_UTF16LE.len())? != JSFART2_CONTENTS_MAGIC_UTF16LE {
        return None;
    }

    let width = read_le32_at(stream, JSFART2_ART_WIDTH_OFFSET)?;
    let height = read_le32_at(stream, JSFART2_ART_HEIGHT_OFFSET)?;
    let frame_candidate = jsfart_art_frame_candidate_from_stream(stream, width, height);
    let paint_candidate = jsfart_art_paint_candidate_from_stream(stream);
    Some(ObjectJsfartArtCandidate::new(
        0,
        width,
        height,
        frame_candidate,
        paint_candidate,
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
    ))
}

pub(crate) fn jsfart_art_paint_candidate_from_stream(
    stream: &[u8],
) -> Option<ObjectJsfartArtPaintCandidate> {
    Some(ObjectJsfartArtPaintCandidate::new(
        read_le32_at(stream, JSFART2_ART_STYLE_WORD_1_OFFSET)?,
        read_le32_at(stream, JSFART2_ART_STYLE_WORD_2_OFFSET)?,
        read_le32_at(stream, JSFART2_ART_PAINT_COLOR_CANDIDATE_OFFSET)?,
        read_le32_at(stream, JSFART2_ART_PAINT_FLAG_CANDIDATE_OFFSET)?,
        read_le32_at(stream, JSFART2_ART_EFFECT_WORD_CANDIDATE_OFFSET)?,
    ))
}

pub(crate) fn jsfart_art_frame_candidate_from_stream(
    stream: &[u8],
    width: u32,
    height: u32,
) -> Option<ObjectJsfartArtFrameCandidate> {
    if width == 0 || height == 0 {
        return None;
    }

    let content_left = read_le32_at(stream, JSFART2_ART_FRAME_LEFT_OFFSET)?;
    let content_top = read_le32_at(stream, JSFART2_ART_FRAME_TOP_OFFSET)?;
    let content_right = read_le32_at(stream, JSFART2_ART_FRAME_RIGHT_OFFSET)?;
    let content_bottom = read_le32_at(stream, JSFART2_ART_FRAME_BOTTOM_OFFSET)?;
    if !(content_left < content_right
        && content_top < content_bottom
        && content_right <= width
        && content_bottom <= height)
    {
        return None;
    }

    let corner_radius_x = content_left;
    let corner_radius_y = content_top;
    let stroke_width_candidate = read_le32_at(stream, JSFART2_ART_STROKE_WIDTH_CANDIDATE_OFFSET)
        .filter(|value| *value > 0 && *value <= height);

    Some(ObjectJsfartArtFrameCandidate::new(
        0,
        0,
        width,
        height,
        content_left,
        content_top,
        content_right,
        content_bottom,
        corner_radius_x,
        corner_radius_y,
        stroke_width_candidate,
    ))
}

pub(crate) fn jseq3_formula_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectJseq3FormulaCandidate> {
    if !path.ends_with("/JSEQ3Contents") {
        return None;
    }
    if stream.get(..JSEQ3_CONTENTS_MAGIC_UTF16LE.len())? != JSEQ3_CONTENTS_MAGIC_UTF16LE {
        return None;
    }

    let so_trailer_offset = jseq3_so_trailer_offset(stream);
    let so_trailer_length = so_trailer_offset.map(|offset| stream.len().saturating_sub(offset));
    let so_trailer_fields = so_trailer_offset
        .and_then(|offset| stream.get(offset..))
        .map(jseq3_so_trailer_fields)
        .unwrap_or_default();
    let text_markers = jseq3_text_marker_candidates(stream);
    let text_tokens = jseq3_text_token_candidates(stream);
    let text_runs = jseq3_text_run_candidates(stream, &text_tokens);
    Some(ObjectJseq3FormulaCandidate::new(
        0,
        so_trailer_offset,
        so_trailer_length,
        so_trailer_fields,
        text_markers,
        text_tokens,
        text_runs,
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
    ))
}

pub(crate) fn jseq3_so_trailer_offset(stream: &[u8]) -> Option<usize> {
    find_subslice_offsets(stream, SO_RECORD_MARKER)
        .into_iter()
        .find(|offset| {
            offset.saturating_add(JSEQ3_SO_FIELD_COUNT * JSEQ3_SO_FIELD_BYTES) <= stream.len()
                && offset.saturating_add(JSEQ3_SO_TRAILER_BYTES) >= stream.len()
        })
}

pub(crate) fn jseq3_so_trailer_fields(trailer: &[u8]) -> Vec<u32> {
    (0..JSEQ3_SO_FIELD_COUNT)
        .filter_map(|index| read_le32_at(trailer, index * JSEQ3_SO_FIELD_BYTES))
        .collect()
}

pub(crate) fn jseq3_text_marker_candidates(stream: &[u8]) -> Vec<ObjectJseq3TextMarkerCandidate> {
    let mut candidates = Vec::new();
    for marker in JSEQ3_TEXT_MARKERS {
        let encoded = utf16le_bytes(marker);
        for offset in find_subslice_offsets(stream, &encoded) {
            candidates.push(ObjectJseq3TextMarkerCandidate::new(
                *marker, offset, "utf-16le",
            ));
        }
    }
    candidates.sort_by_key(|candidate| candidate.offset());
    candidates
}

pub(crate) fn jseq3_text_token_candidates(stream: &[u8]) -> Vec<ObjectJseq3TextTokenCandidate> {
    let mut tokens = Vec::new();
    let mut offset = 0usize;
    while offset + 2 <= stream.len() {
        let Some(unit) = read_le16_at(stream, offset) else {
            break;
        };
        if let Some(character) = char::from_u32(u32::from(unit))
            && JSEQ3_TEXT_TOKEN_CHARS.contains(character)
        {
            tokens.push(ObjectJseq3TextTokenCandidate::new(
                character.to_string(),
                offset,
                "utf-16le",
            ));
        }
        offset += 2;
    }
    tokens
}

pub(crate) fn jseq3_text_run_candidates(
    stream: &[u8],
    tokens: &[ObjectJseq3TextTokenCandidate],
) -> Vec<ObjectJseq3TextRunCandidate> {
    let layout_tokens = tokens
        .iter()
        .filter(|token| token.offset() >= JSEQ3_TEXT_RUN_SCAN_MIN_OFFSET)
        .collect::<Vec<_>>();
    let mut runs = Vec::new();
    let mut index = 0usize;
    while index < layout_tokens.len() {
        let mut end_index = index + 1;
        while end_index < layout_tokens.len()
            && layout_tokens[end_index].offset()
                == layout_tokens[end_index - 1]
                    .offset()
                    .saturating_add(JSEQ3_TEXT_RUN_CONTIGUOUS_STRIDE_BYTES)
        {
            end_index += 1;
        }
        let slice = &layout_tokens[index..end_index];
        let Some(first) = slice.first() else {
            break;
        };
        let Some(last) = slice.last() else {
            break;
        };
        let text = slice
            .iter()
            .map(|token| token.text())
            .collect::<Vec<_>>()
            .join("");
        let token_offsets = slice.iter().map(|token| token.offset()).collect::<Vec<_>>();
        let context_start = first
            .offset()
            .saturating_sub(JSEQ3_TEXT_RUN_CONTEXT_BEFORE_BYTES);
        let context_fields = (0..JSEQ3_TEXT_RUN_CONTEXT_FIELD_COUNT)
            .filter_map(|field_index| {
                let offset = context_start.saturating_add(field_index * 4);
                stream
                    .get(offset..offset.saturating_add(4))
                    .and_then(|bytes| bytes.try_into().ok())
                    .map(i32::from_le_bytes)
            })
            .collect::<Vec<_>>();
        runs.push(ObjectJseq3TextRunCandidate::new(
            text,
            first.offset(),
            last.offset().saturating_add(2),
            token_offsets,
            context_start,
            context_fields,
        ));
        index = end_index;
    }
    runs
}

pub(crate) fn visual_list_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectVisualListCandidate> {
    if !path.to_ascii_lowercase().contains("visuallist") {
        return None;
    }
    if stream.get(VISUAL_LIST_MAGIC_OFFSET..VISUAL_LIST_MAGIC_OFFSET + VISUAL_LIST_MAGIC.len())?
        != VISUAL_LIST_MAGIC
    {
        return None;
    }
    let declared_size = read_be32_at(stream, 0)? as usize;
    let version = read_be32_at(stream, VISUAL_LIST_VERSION_OFFSET)?;
    let flags = read_be32_at(stream, VISUAL_LIST_FLAGS_OFFSET)?;
    let width = read_be32_at(stream, VISUAL_LIST_WIDTH_OFFSET)?;
    let height = read_be32_at(stream, VISUAL_LIST_HEIGHT_OFFSET)?;
    let row_stride = read_be32_at(stream, VISUAL_LIST_ROW_STRIDE_OFFSET)?;
    let bit_depth = read_be32_at(stream, VISUAL_LIST_BIT_DEPTH_OFFSET)?;
    let x_pixels_per_meter = read_be32_at(stream, VISUAL_LIST_X_PPM_OFFSET)?;
    let y_pixels_per_meter = read_be32_at(stream, VISUAL_LIST_Y_PPM_OFFSET)?;
    let rle_data_len = read_be32_at(stream, VISUAL_LIST_RLE_LENGTH_OFFSET)? as usize;
    let rle_data_end = VISUAL_LIST_HEADER_BYTES.checked_add(rle_data_len)?;
    let rle_data = stream.get(VISUAL_LIST_HEADER_BYTES..rle_data_end)?;
    let pixels = decode_visual_list_rle8(width, height, rle_data)?;
    Some(ObjectVisualListCandidate::new(
        declared_size,
        version,
        flags,
        width,
        height,
        row_stride,
        bit_depth,
        x_pixels_per_meter,
        y_pixels_per_meter,
        VISUAL_LIST_HEADER_BYTES,
        rle_data_len,
        pixels,
    ))
}

pub(crate) fn decode_visual_list_rle8(width: u32, height: u32, data: &[u8]) -> Option<Vec<u8>> {
    if width == 0 || height == 0 || width > 10_000 || height > 10_000 {
        return None;
    }
    let width = usize::try_from(width).ok()?;
    let height = usize::try_from(height).ok()?;
    let total_pixels = width.checked_mul(height)?;
    if total_pixels > 16_000_000 {
        return None;
    }

    let fill = visual_list_default_pixel(data);
    let mut pixels = Vec::with_capacity(total_pixels);
    let mut row = Vec::with_capacity(width);
    let mut offset = 0usize;
    while offset + 1 < data.len() && pixels.len() < total_pixels {
        let count = data[offset];
        let value = data[offset + 1];
        offset += 2;
        if count != 0 {
            row.extend(std::iter::repeat_n(value, count as usize));
            continue;
        }

        match value {
            0 => flush_visual_list_row(&mut pixels, &mut row, width, height, fill),
            1 => break,
            2 => {
                if offset + 1 >= data.len() {
                    return None;
                }
                let dx = data[offset] as usize;
                let dy = data[offset + 1] as usize;
                offset += 2;
                row.extend(std::iter::repeat_n(fill, dx));
                for _ in 0..dy {
                    flush_visual_list_row(&mut pixels, &mut row, width, height, fill);
                }
            }
            literal_len => {
                let literal_len = literal_len as usize;
                let literal_end = offset.checked_add(literal_len)?;
                row.extend_from_slice(data.get(offset..literal_end)?);
                offset = literal_end;
                if literal_len % 2 == 1 {
                    offset = offset.checked_add(1)?;
                    if offset > data.len() {
                        return None;
                    }
                }
            }
        }
    }

    if !row.is_empty() && pixels.len() < total_pixels {
        flush_visual_list_row(&mut pixels, &mut row, width, height, fill);
    }
    while pixels.len() < total_pixels {
        pixels.extend(std::iter::repeat_n(fill, width));
    }
    pixels.truncate(total_pixels);
    Some(pixels)
}

pub(crate) fn visual_list_default_pixel(data: &[u8]) -> u8 {
    if data.len() >= 2 && data[0] != 0 {
        data[1]
    } else {
        0xff
    }
}

pub(crate) fn flush_visual_list_row(
    pixels: &mut Vec<u8>,
    row: &mut Vec<u8>,
    width: usize,
    height: usize,
    fill: u8,
) {
    if pixels.len() >= width.saturating_mul(height) {
        row.clear();
        return;
    }
    if row.len() < width {
        row.extend(std::iter::repeat_n(fill, width - row.len()));
    }
    pixels.extend(row.iter().copied().take(width));
    row.clear();
}

pub(crate) fn image_signature_hits(
    stream: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectImageSignatureHit>> {
    let mut hits = Vec::new();
    push_signature_hits(&mut hits, stream, "png", b"\x89PNG\r\n\x1a\n", true, budget)?;
    push_signature_hits(&mut hits, stream, "jpeg", b"\xff\xd8\xff", true, budget)?;
    push_signature_hits(&mut hits, stream, "gif87a", b"GIF87a", true, budget)?;
    push_signature_hits(&mut hits, stream, "gif89a", b"GIF89a", true, budget)?;
    push_signature_hits(&mut hits, stream, "tiff-le", b"II\x2a\0", true, budget)?;
    push_signature_hits(&mut hits, stream, "tiff-be", b"MM\0\x2a", true, budget)?;
    push_signature_hits(
        &mut hits,
        stream,
        "wmf-placeable",
        b"\xd7\xcd\xc6\x9a",
        true,
        budget,
    )?;
    push_signature_hits(&mut hits, stream, "bmp", b"BM", false, budget)?;

    hits.sort_by(|left, right| {
        left.offset()
            .cmp(&right.offset())
            .then_with(|| left.kind().cmp(right.kind()))
    });
    Ok(hits)
}

pub(crate) fn image_payload_spans(
    stream: &[u8],
    hits: &[ObjectImageSignatureHit],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectImagePayloadSpan>> {
    let mut candidates = hits
        .iter()
        .filter_map(|hit| image_payload_candidate(stream, hit));
    let mut previous_end = None;
    let mut candidate = candidates.next();
    let mut spans = Vec::new();

    while let Some(current) = candidate {
        candidate = candidates.next();
        let next_start = candidate.as_ref().map(|next| next.start);
        let header_start = previous_end
            .filter(|end| *end <= current.start)
            .unwrap_or(0);
        let trailer_end = next_start
            .filter(|start| *start >= current.end)
            .unwrap_or(stream.len());
        let Some(payload) = stream.get(current.start..current.end) else {
            previous_end = Some(current.end);
            continue;
        };
        let dimensions = image_payload_dimensions(payload);
        if let Some(dimensions) = dimensions {
            budget.check_image_dimensions(dimensions.width(), dimensions.height())?;
        }
        let retained_bytes = image_payload_retained_bytes(
            payload.len(),
            header_start,
            current.start,
            current.end,
            trailer_end,
        )?;
        budget.reserve_image(retained_bytes)?;
        let envelope = image_payload_envelope(
            stream,
            header_start,
            current.start,
            current.end,
            trailer_end,
        );
        spans.push(ObjectImagePayloadSpan::new_with_dimensions(
            current.kind,
            current.mime,
            ObjectImagePayloadLocation::new(current.signature_offset, current.start, current.end),
            true,
            payload.to_vec(),
            dimensions,
            envelope,
        ));
        previous_end = Some(current.end);
    }
    Ok(spans)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImagePayloadCandidate<'a> {
    pub(crate) kind: &'a str,
    pub(crate) mime: &'static str,
    pub(crate) signature_offset: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

pub(crate) fn image_payload_candidate<'a>(
    stream: &[u8],
    hit: &'a ObjectImageSignatureHit,
) -> Option<ImagePayloadCandidate<'a>> {
    let end = match hit.kind() {
        "jpeg" => jpeg_payload_end(stream, hit.offset())?,
        "png" => png_payload_end(stream, hit.offset())?,
        "gif87a" | "gif89a" => gif_payload_end(stream, hit.offset())?,
        "bmp" => bmp_payload_end(stream, hit.offset())?,
        _ => return None,
    };

    Some(ImagePayloadCandidate {
        kind: hit.kind(),
        mime: image_mime_for_kind(hit.kind()),
        signature_offset: hit.offset(),
        start: hit.offset(),
        end,
    })
}

pub(crate) fn image_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    png_payload_dimensions(payload)
        .or_else(|| gif_payload_dimensions(payload))
        .or_else(|| bmp_payload_dimensions(payload))
        .or_else(|| jpeg_payload_dimensions(payload))
}

pub(crate) fn image_payload_envelope(
    stream: &[u8],
    header_start: usize,
    header_end: usize,
    trailer_start: usize,
    trailer_end: usize,
) -> ObjectImagePayloadEnvelope {
    let header_start = header_start.min(header_end).min(stream.len());
    let header_end = header_end.min(stream.len());
    let trailer_start = trailer_start.min(stream.len());
    let trailer_end = trailer_end.max(trailer_start).min(stream.len());
    let header = stream[header_start..header_end].to_vec();
    let trailer = stream[trailer_start..trailer_end].to_vec();
    let declared_payload_length =
        image_declared_payload_length(&header, header_start, trailer_start - header_end);

    ObjectImagePayloadEnvelope::new(
        header_start,
        header_end,
        trailer_start,
        trailer_end,
        declared_payload_length,
        header,
        trailer,
    )
}

pub(crate) fn image_header_field_candidates(
    header_start: usize,
    header: &[u8],
) -> ObjectImageHeaderFieldCandidates {
    let prefix_len = header.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES);
    let prefix = &header[..prefix_len];
    let mut u16_le_prefix = Vec::new();
    for relative_offset in (0..prefix.len()).step_by(2) {
        if relative_offset + 2 <= prefix.len() {
            u16_le_prefix.push(ObjectImageNumericHeaderField::new(
                header_start + relative_offset,
                u16::from_le_bytes([prefix[relative_offset], prefix[relative_offset + 1]]) as u64,
            ));
        }
    }

    let mut u32_le_prefix = Vec::new();
    for relative_offset in (0..prefix.len()).step_by(4) {
        if relative_offset + 4 <= prefix.len() {
            u32_le_prefix.push(ObjectImageNumericHeaderField::new(
                header_start + relative_offset,
                u32::from_le_bytes([
                    prefix[relative_offset],
                    prefix[relative_offset + 1],
                    prefix[relative_offset + 2],
                    prefix[relative_offset + 3],
                ]) as u64,
            ));
        }
    }

    ObjectImageHeaderFieldCandidates::new(
        u16_le_prefix,
        u32_le_prefix,
        image_source_path_candidate(header_start, header),
    )
}

pub(crate) fn image_source_path_candidate(
    header_start: usize,
    header: &[u8],
) -> Option<ObjectImageSourcePathCandidate> {
    let length_offset = 16;
    let declared_length = *header.get(length_offset)? as usize;
    if declared_length < 3 {
        return None;
    }
    let bytes_start = length_offset + 1;
    let declared_end = bytes_start.checked_add(declared_length)?;
    let text_bytes = header.get(bytes_start..declared_end)?;
    let raw_end = if header.get(declared_end) == Some(&0) {
        declared_end + 1
    } else if text_bytes.last() == Some(&0) {
        declared_end
    } else {
        return None;
    };
    let bytes = header.get(bytes_start..raw_end)?;
    let text_bytes = if text_bytes.last() == Some(&0) {
        &text_bytes[..text_bytes.len().saturating_sub(1)]
    } else {
        text_bytes
    };
    if !looks_like_embedded_source_path(text_bytes) {
        return None;
    }

    Some(ObjectImageSourcePathCandidate::new(
        header_start + length_offset,
        declared_length,
        header_start + bytes_start,
        header_start + raw_end,
        true,
        bytes.to_vec(),
    ))
}

pub(crate) fn image_declared_payload_length(
    header: &[u8],
    header_start: usize,
    payload_len: usize,
) -> Option<ObjectImageDeclaredLengthCandidate> {
    let offset_in_header = header.len().checked_sub(4)?;
    let value = u32::from_le_bytes([
        header[offset_in_header],
        header[offset_in_header + 1],
        header[offset_in_header + 2],
        header[offset_in_header + 3],
    ]) as usize;
    (value == payload_len).then(|| {
        ObjectImageDeclaredLengthCandidate::new(header_start + offset_in_header, value, "le32")
    })
}

pub(crate) fn image_mime_for_kind(kind: &str) -> &'static str {
    match kind {
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif87a" | "gif89a" => "image/gif",
        "bmp" => "image/bmp",
        "tiff-le" | "tiff-be" => "image/tiff",
        "wmf-placeable" => "image/wmf",
        _ => "application/octet-stream",
    }
}

pub(crate) fn next_snapshot_id(current: u32) -> u32 {
    current.checked_add(1).filter(|id| *id > 0).unwrap_or(1)
}

pub(crate) fn object_embedding_frames_json(frames: &[ObjectEmbeddingFrameCandidate]) -> String {
    let mut output = String::from("[");
    for (index, frame) in frames.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_embedding_frame_candidate_json(&mut output, frame);
    }
    output.push(']');
    output
}

pub(crate) fn push_object_embedding_frame_candidate_json(
    output: &mut String,
    frame: &ObjectEmbeddingFrameCandidate,
) {
    output.push_str("{\"sourcePath\":");
    output.push_str(&json_string(frame.source_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&frame.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&frame.row_start().to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    output.push_str(&json_string(frame.class_name()));
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
    output.push_str(&json_string(&hex_bytes(frame.row_prefix())));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_jsfart_stream_profile_candidate_json(
    output: &mut String,
    profile: &ObjectJsfartStreamProfileCandidate,
) {
    output.push_str("{\"format\":\"JSFart2Contents\",\"source\":\"stream-prefix\",\"sourceCandidateType\":\"objectStream\",\"magicFamily\":");
    output.push_str(&json_string(profile.magic_family()));
    output.push_str(",\"magicFamilyHex\":");
    output.push_str(&json_string(profile.magic_family_hex()));
    output.push_str(",\"magicOffset\":");
    output.push_str(&profile.magic_offset().to_string());
    output.push_str(",\"magicAsciiOrUtf16Preview\":");
    output.push_str(&json_string(profile.magic_ascii_or_utf16_preview()));
    output.push_str(",\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(profile.header_prefix())));
    output.push_str(",\"structuredArtCandidatePresent\":");
    output.push_str(if profile.structured_art_candidate_present() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"renderable\":false,\"decoded\":false,\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(profile.render_promotion_blocked_reason()));
    output.push('}');
}

pub(crate) fn push_object_jsfart_art_candidate_json(
    output: &mut String,
    art: &ObjectJsfartArtCandidate,
) {
    output.push_str("{\"format\":\"JSFart2Contents\",\"magic\":");
    output.push_str(&json_string(art.magic()));
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
    output.push_str(&json_string(&hex_bytes(art.header_prefix())));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

pub(crate) fn push_object_jsfart_art_paint_candidate_json(
    output: &mut String,
    paint: &ObjectJsfartArtPaintCandidate,
) {
    output.push_str("{\"styleWord1\":");
    output.push_str(&paint.style_word_1().to_string());
    output.push_str(",\"styleWord1Hex\":");
    output.push_str(&json_string(&format!("0x{:08x}", paint.style_word_1())));
    output.push_str(",\"styleWord2\":");
    output.push_str(&paint.style_word_2().to_string());
    output.push_str(",\"styleWord2Hex\":");
    output.push_str(&json_string(&format!("0x{:08x}", paint.style_word_2())));
    output.push_str(",\"paintColorCandidate\":");
    output.push_str(&paint.paint_color_candidate().to_string());
    output.push_str(",\"paintColorCandidateHex\":");
    output.push_str(&json_string(&format!(
        "0x{:08x}",
        paint.paint_color_candidate()
    )));
    output.push_str(",\"paintFlagCandidate\":");
    output.push_str(&paint.paint_flag_candidate().to_string());
    output.push_str(",\"paintFlagCandidateHex\":");
    output.push_str(&json_string(&format!(
        "0x{:08x}",
        paint.paint_flag_candidate()
    )));
    output.push_str(",\"effectWordCandidate\":");
    output.push_str(&paint.effect_word_candidate().to_string());
    output.push_str(",\"effectWordCandidateHex\":");
    output.push_str(&json_string(&format!(
        "0x{:08x}",
        paint.effect_word_candidate()
    )));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_jseq3_formula_candidate_json(
    output: &mut String,
    formula: &ObjectJseq3FormulaCandidate,
) {
    output.push_str("{\"format\":\"JSEQ3Contents\",\"magic\":");
    output.push_str(&json_string(formula.magic()));
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
        output.push_str(&json_string(marker.text()));
        output.push_str(",\"offset\":");
        output.push_str(&marker.offset().to_string());
        output.push_str(",\"encoding\":");
        output.push_str(&json_string(marker.encoding()));
        output.push('}');
    }
    output.push_str("],\"textTokens\":[");
    for (index, token) in formula.text_tokens().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        output.push_str(&json_string(token.text()));
        output.push_str(",\"offset\":");
        output.push_str(&token.offset().to_string());
        output.push_str(",\"encoding\":");
        output.push_str(&json_string(token.encoding()));
        output.push('}');
    }
    output.push_str("],\"textRuns\":[");
    for (index, run) in formula.text_runs().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        output.push_str(&json_string(run.text()));
        output.push_str(",\"startOffset\":");
        output.push_str(&run.start_offset().to_string());
        output.push_str(",\"endOffset\":");
        output.push_str(&run.end_offset().to_string());
        output.push_str(",\"tokenOffsets\":");
        push_usize_array_json(output, run.token_offsets());
        output.push_str(",\"contextStartOffset\":");
        output.push_str(&run.context_start_offset().to_string());
        output.push_str(",\"contextFieldsLe32\":");
        push_i32_array_json(output, run.context_fields_le32());
        output.push('}');
    }
    output.push_str("],\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(formula.header_prefix())));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

pub(crate) fn push_object_visual_list_candidate_json(
    output: &mut String,
    visual_list: &ObjectVisualListCandidate,
) {
    output.push_str("{\"format\":\"BMDV\",\"declaredSize\":");
    output.push_str(&visual_list.declared_size().to_string());
    output.push_str(",\"magicOffset\":");
    output.push_str(&visual_list.magic_offset().to_string());
    output.push_str(",\"magic\":");
    output.push_str(&json_string(visual_list.magic()));
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

pub(crate) fn push_object_image_payload_span_json(
    output: &mut String,
    span: &ObjectImagePayloadSpan,
) {
    output.push_str("{\"kind\":");
    output.push_str(&json_string(span.kind()));
    output.push_str(",\"mime\":");
    output.push_str(&json_string(span.mime()));
    output.push_str(",\"signatureOffset\":");
    output.push_str(&span.signature_offset().to_string());
    output.push_str(",\"start\":");
    output.push_str(&span.start().to_string());
    output.push_str(",\"end\":");
    output.push_str(&span.end().to_string());
    output.push_str(",\"length\":");
    output.push_str(&span.len().to_string());
    output.push_str(",\"complete\":");
    output.push_str(if span.complete() { "true" } else { "false" });
    output.push_str(",\"dimensions\":");
    push_object_image_dimensions_json(output, span.dimensions());
    output.push_str(",\"objectEnvelope\":");
    push_object_image_payload_envelope_json(output, span.envelope());
    output.push_str(",\"payloadPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &span.payload()[..span.payload().len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_image_dimensions_json(
    output: &mut String,
    dimensions: Option<ObjectImageDimensions>,
) {
    if let Some(dimensions) = dimensions {
        output.push_str("{\"width\":");
        output.push_str(&dimensions.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&dimensions.height().to_string());
        output.push('}');
    } else {
        output.push_str("null");
    }
}

pub(crate) fn push_object_image_payload_envelope_json(
    output: &mut String,
    envelope: &ObjectImagePayloadEnvelope,
) {
    output.push_str("{\"headerStart\":");
    output.push_str(&envelope.header_start().to_string());
    output.push_str(",\"headerEnd\":");
    output.push_str(&envelope.header_end().to_string());
    output.push_str(",\"headerLength\":");
    output.push_str(&envelope.header_len().to_string());
    output.push_str(",\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &envelope.header()[..envelope
            .header()
            .len()
            .min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"headerFields\":");
    push_object_image_header_fields_json(output, envelope.header_fields());
    output.push_str(",\"trailerStart\":");
    output.push_str(&envelope.trailer_start().to_string());
    output.push_str(",\"trailerEnd\":");
    output.push_str(&envelope.trailer_end().to_string());
    output.push_str(",\"trailerLength\":");
    output.push_str(&envelope.trailer_len().to_string());
    output.push_str(",\"trailerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &envelope.trailer()[..envelope
            .trailer()
            .len()
            .min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"declaredPayloadLength\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.value().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthOffset\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.offset().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthEndian\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&json_string(length.endian()));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_image_header_fields_json(
    output: &mut String,
    fields: &ObjectImageHeaderFieldCandidates,
) {
    output.push_str("{\"u16LePrefix\":[");
    for (index, field) in fields.u16_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"u32LePrefix\":[");
    for (index, field) in fields.u32_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"sourcePathCandidate\":");
    if let Some(path) = fields.source_path_candidate() {
        push_object_image_source_path_candidate_json(output, path);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(crate) fn push_object_image_numeric_header_field_json(
    output: &mut String,
    field: &ObjectImageNumericHeaderField,
) {
    output.push_str("{\"offset\":");
    output.push_str(&field.offset().to_string());
    output.push_str(",\"value\":");
    output.push_str(&field.value().to_string());
    output.push('}');
}

pub(crate) fn push_object_image_source_path_candidate_json(
    output: &mut String,
    path: &ObjectImageSourcePathCandidate,
) {
    output.push_str("{\"lengthOffset\":");
    output.push_str(&path.length_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&path.declared_length().to_string());
    output.push_str(",\"bytesStart\":");
    output.push_str(&path.bytes_start().to_string());
    output.push_str(",\"bytesEnd\":");
    output.push_str(&path.bytes_end().to_string());
    output.push_str(",\"nulTerminated\":");
    output.push_str(if path.nul_terminated() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bytesHex\":");
    output.push_str(&json_string(&hex_bytes(path.bytes())));
    output.push_str(",\"textLossy\":");
    output.push_str(&json_string(path.text_lossy()));
    output.push_str(",\"decoded\":false}");
}
