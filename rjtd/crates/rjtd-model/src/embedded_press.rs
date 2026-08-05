use super::*;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_MAGIC: &[u8; 12] = b"JSSnapShot32";

pub(super) const EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET: usize = 0x24;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET: usize = 0x2c;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET: usize = 0x34;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET: usize = 0x38;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET: usize = 0x3c;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET: usize = 0x48;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET: usize = 0x4c;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_VECTOR_SCAN_OFFSET: usize = 0x4a;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_VECTOR_SEGMENT_LIMIT: usize = 16_384;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_RECORD_OFFSET: usize = 0x80;

pub(super) const EMBEDDED_PRESS_SNAPSHOT_MAX_VECTOR_PATHS: usize = 4096;

pub(super) const EMBEDDED_PRESS_SHADOW_PAIR_BBOX_TOLERANCE_SOURCE_UNITS: i32 = 4;

pub(super) const EMBEDDED_PRESS_RECORD_BEGIN_PATH: u32 = 0x4c;

pub(super) const EMBEDDED_PRESS_RECORD_END_PATH: u32 = 0x4d;

pub(super) const EMBEDDED_PRESS_RECORD_MOVE_TO: u32 = 0xd0;

pub(super) const EMBEDDED_PRESS_RECORD_BEZIER_TO: u32 = 0xd7;

pub(super) const EMBEDDED_PRESS_RECORD_CLOSE_PATH: u32 = 0xd1;

pub(super) const EMBEDDED_PRESS_RECORD_TEXTURE_BEZIER: u32 = 0xc6;

pub(super) const EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70: u32 = 0x70;

pub(super) const EMBEDDED_PRESS_RECORD_PAINT_STATE_82: u32 = 0x82;

pub(super) const EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5: u32 = 0x10;

pub(super) const EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5: u32 = 0x2f;

pub(super) fn embedded_press_snapshot_candidate_from_stream(
    stream: &[u8],
) -> Option<ObjectEmbeddedPressSnapshotCandidate> {
    if stream.get(..EMBEDDED_PRESS_SNAPSHOT_MAGIC.len())? != EMBEDDED_PRESS_SNAPSHOT_MAGIC {
        return None;
    }
    let body_length_candidate = read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET)?;
    let format_marker = stream
        .get(EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET..EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET + 4)
        .map(|bytes| {
            bytes
                .iter()
                .copied()
                .filter(|byte| byte.is_ascii_graphic())
                .map(char::from)
                .collect::<String>()
        })?;
    let object_count_candidate = read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET)?;
    let object_table_offset_candidate =
        read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET)?;
    let payload_length_candidate =
        read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET)?;
    let width = read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET)?;
    let height = read_le32_at(stream, EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET)?;
    if width == 0 || height == 0 || body_length_candidate == 0 || payload_length_candidate == 0 {
        return None;
    }
    let vector_segments = embedded_press_snapshot_vector_segments(stream, width, height);
    let vector_paths = embedded_press_snapshot_vector_paths(stream, width, height);
    Some(ObjectEmbeddedPressSnapshotCandidate::new(
        body_length_candidate,
        format_marker,
        object_count_candidate,
        object_table_offset_candidate,
        payload_length_candidate,
        width,
        height,
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
        vector_segments,
        vector_paths,
    ))
}

pub(super) fn embedded_press_snapshot_vector_paths(
    stream: &[u8],
    width: u32,
    height: u32,
) -> Vec<ObjectEmbeddedPressVectorPathCandidate> {
    if width == 0 || height == 0 || EMBEDDED_PRESS_SNAPSHOT_RECORD_OFFSET + 8 > stream.len() {
        return Vec::new();
    }

    let mut paths = Vec::new();
    let mut current = None;
    let mut pending_state_records = Vec::new();
    let mut offset = EMBEDDED_PRESS_SNAPSHOT_RECORD_OFFSET;
    while offset + 8 <= stream.len() && paths.len() < EMBEDDED_PRESS_SNAPSHOT_MAX_VECTOR_PATHS {
        let Some(record_size) = read_le32_at(stream, offset).map(|value| value as usize) else {
            break;
        };
        let Some(record_type) = read_le32_at(stream, offset + 4) else {
            break;
        };
        if record_size < 8
            || record_size % 4 != 0
            || offset
                .checked_add(record_size)
                .is_none_or(|end| end > stream.len())
        {
            break;
        }

        let payload = &stream[offset + 8..offset + record_size];
        match record_type {
            EMBEDDED_PRESS_RECORD_BEGIN_PATH => {
                if let Some(path) = current
                    .take()
                    .and_then(ObjectEmbeddedPressVectorPathBuilder::finish)
                {
                    paths.push(path);
                }
                current = Some(ObjectEmbeddedPressVectorPathBuilder::new(std::mem::take(
                    &mut pending_state_records,
                )));
            }
            EMBEDDED_PRESS_RECORD_END_PATH => {
                if let Some(path) = current
                    .take()
                    .and_then(ObjectEmbeddedPressVectorPathBuilder::finish)
                {
                    paths.push(path);
                }
            }
            EMBEDDED_PRESS_RECORD_MOVE_TO => {
                if let Some(builder) = current.as_mut()
                    && let Some((x, y)) = embedded_press_path_point(payload, 0, width, height)
                {
                    builder.push(ObjectEmbeddedPressVectorPathCommandCandidate::MoveTo { x, y });
                }
            }
            EMBEDDED_PRESS_RECORD_BEZIER_TO => {
                if let Some(builder) = current.as_mut() {
                    push_embedded_press_bezier_record(builder, payload, 8, width, height);
                }
            }
            EMBEDDED_PRESS_RECORD_CLOSE_PATH => {
                if let Some(builder) = current.as_mut() {
                    builder.push(ObjectEmbeddedPressVectorPathCommandCandidate::Close);
                }
            }
            EMBEDDED_PRESS_RECORD_TEXTURE_BEZIER => {
                if let Some(builder) = current.as_mut()
                    && let Some(header) = embedded_press_texture_bezier_header(payload)
                {
                    builder.mark_texture(header);
                    push_embedded_press_texture_bezier_record(builder, payload, width, height);
                }
            }
            _ => {
                let state_record = ObjectEmbeddedPressStateRecordCandidate::new(
                    record_type,
                    offset,
                    payload.to_vec(),
                );
                if let Some(builder) = current.as_mut() {
                    builder.state_records.push(state_record);
                } else {
                    pending_state_records.push(state_record);
                }
            }
        }

        offset += record_size;
    }

    if let Some(path) = current.and_then(ObjectEmbeddedPressVectorPathBuilder::finish) {
        paths.push(path);
    }

    paths
}

pub(super) fn push_embedded_press_bezier_record(
    builder: &mut ObjectEmbeddedPressVectorPathBuilder,
    payload: &[u8],
    points_offset: usize,
    width: u32,
    height: u32,
) {
    let Some(points) = embedded_press_record_points(payload, points_offset, width, height) else {
        return;
    };
    for chunk in points.chunks(3) {
        let [(x1, y1), (x2, y2), (x3, y3)] = chunk else {
            break;
        };
        builder.push(ObjectEmbeddedPressVectorPathCommandCandidate::CubicTo {
            x1: *x1,
            y1: *y1,
            x2: *x2,
            y2: *y2,
            x3: *x3,
            y3: *y3,
        });
    }
}

pub(super) fn push_embedded_press_texture_bezier_record(
    builder: &mut ObjectEmbeddedPressVectorPathBuilder,
    payload: &[u8],
    width: u32,
    height: u32,
) {
    let Some(points) = embedded_press_record_points(payload, 12, width, height) else {
        return;
    };
    let Some((x, y)) = points.first().copied() else {
        return;
    };
    builder.push(ObjectEmbeddedPressVectorPathCommandCandidate::MoveTo { x, y });
    for chunk in points[1..].chunks(3) {
        let [(x1, y1), (x2, y2), (x3, y3)] = chunk else {
            break;
        };
        builder.push(ObjectEmbeddedPressVectorPathCommandCandidate::CubicTo {
            x1: *x1,
            y1: *y1,
            x2: *x2,
            y2: *y2,
            x3: *x3,
            y3: *y3,
        });
    }
    builder.push(ObjectEmbeddedPressVectorPathCommandCandidate::Close);
}

pub(super) fn embedded_press_texture_bezier_header(
    payload: &[u8],
) -> Option<ObjectEmbeddedPressTextureBezierHeaderCandidate> {
    let point_count = read_le32_at(payload, 0)?;
    let byte_count = read_le32_at(payload, 4)?;
    let flags = read_le32_at(payload, 8)?;
    if point_count == 0
        || byte_count != point_count.checked_mul(8)?
        || 12usize
            .checked_add(byte_count as usize)
            .is_none_or(|end| end > payload.len())
    {
        return None;
    }
    Some(ObjectEmbeddedPressTextureBezierHeaderCandidate::new(
        point_count,
        byte_count,
        flags,
    ))
}

pub(super) fn embedded_press_record_points(
    payload: &[u8],
    points_offset: usize,
    width: u32,
    height: u32,
) -> Option<Vec<(u32, u32)>> {
    let count = read_le32_at(payload, 0)? as usize;
    let byte_count = read_le32_at(payload, 4)? as usize;
    if count == 0
        || byte_count != count.checked_mul(8)?
        || points_offset
            .checked_add(byte_count)
            .is_none_or(|end| end > payload.len())
    {
        return None;
    }
    let mut points = Vec::with_capacity(count);
    for index in 0..count {
        let offset = points_offset + index * 8;
        let point = embedded_press_path_point(payload, offset, width, height)?;
        points.push(point);
    }
    Some(points)
}

pub(super) fn embedded_press_path_point(
    payload: &[u8],
    offset: usize,
    width: u32,
    height: u32,
) -> Option<(u32, u32)> {
    let x = read_le32_at(payload, offset)?;
    let y = read_le32_at(payload, offset + 4)?;
    (x <= width && y <= height).then_some((x, y))
}

pub(super) fn embedded_press_snapshot_vector_segments(
    stream: &[u8],
    width: u32,
    height: u32,
) -> Vec<ObjectEmbeddedPressVectorSegmentCandidate> {
    if width == 0 || height == 0 || EMBEDDED_PRESS_SNAPSHOT_VECTOR_SCAN_OFFSET + 8 > stream.len() {
        return Vec::new();
    }

    let mut values = Vec::new();
    let mut offset = EMBEDDED_PRESS_SNAPSHOT_VECTOR_SCAN_OFFSET;
    while offset + 4 <= stream.len() {
        let raw = read_i32_le_at(stream, offset).unwrap_or_default();
        values.push(if raw.rem_euclid(65_536) == 0 {
            Some(raw / 65_536)
        } else {
            None
        });
        offset += 4;
    }

    let mut pairs = Vec::new();
    for index in 0..values.len().saturating_sub(1) {
        let Some(x) = values[index] else {
            continue;
        };
        let Some(y) = values[index + 1] else {
            continue;
        };
        if x >= 0 && y >= 0 && (x as u32) <= width && (y as u32) <= height {
            pairs.push((index, x as u32, y as u32));
        }
    }

    let max_delta = width.max(height);
    let mut segments = Vec::new();
    for window in pairs.windows(2) {
        let (first_index, x1, y1) = window[0];
        let (second_index, x2, y2) = window[1];
        if second_index != first_index + 2 {
            continue;
        }
        let delta = x1.abs_diff(x2) + y1.abs_diff(y2);
        if !(3..=max_delta).contains(&delta) {
            continue;
        }
        segments.push(ObjectEmbeddedPressVectorSegmentCandidate::new(
            x1, y1, x2, y2,
        ));
        if segments.len() >= EMBEDDED_PRESS_SNAPSHOT_VECTOR_SEGMENT_LIMIT {
            break;
        }
    }

    segments
}

pub(super) fn push_object_embedded_press_snapshot_candidate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    output.push_str("{\"format\":\"JSSnapShot32\",\"magic\":");
    output.push_str(&json_string(snapshot.magic()));
    output.push_str(",\"bodyLengthCandidate\":");
    output.push_str(&snapshot.body_length_candidate().to_string());
    output.push_str(",\"formatMarker\":");
    output.push_str(&json_string(snapshot.format_marker()));
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
    output.push_str(",\"outlinePathCount\":");
    output.push_str(
        &embedded_press_snapshot_vector_path_kind_count(
            snapshot,
            ObjectEmbeddedPressVectorPathKind::Outline,
        )
        .to_string(),
    );
    output.push_str(",\"texturePathCount\":");
    output.push_str(
        &embedded_press_snapshot_vector_path_kind_count(
            snapshot,
            ObjectEmbeddedPressVectorPathKind::Texture,
        )
        .to_string(),
    );
    output.push_str(",\"vectorPathStateRecordCount\":");
    output.push_str(&embedded_press_snapshot_vector_path_state_record_count(snapshot).to_string());
    output.push_str(",\"vectorPathStateRecordTypes\":");
    push_embedded_press_state_record_type_summary_json(output, snapshot);
    output.push_str(",\"textureBezierHeaderSummary\":");
    push_embedded_press_texture_bezier_header_summary_json(output, snapshot);
    output.push_str(",\"paintStateTransitions\":");
    push_embedded_press_paint_state_transitions_json(output, snapshot);
    output.push_str(",\"vectorSegmentPreview\":");
    push_object_embedded_press_snapshot_vector_segment_preview_json(output, snapshot);
    output.push_str(",\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(snapshot.header_prefix())));
    output.push_str(",\"renderable\":");
    output.push_str(if snapshot.vector_segments().is_empty() {
        "false"
    } else {
        "true"
    });
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_embedded_press_snapshot_vector_segment_preview_json(
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

pub(super) fn push_embedded_press_state_record_type_summary_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    output.push('[');
    for (index, (record_type, count)) in embedded_press_snapshot_state_record_type_counts(snapshot)
        .into_iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"type\":");
        output.push_str(&record_type.to_string());
        output.push_str(",\"typeHex\":");
        output.push_str(&json_string(&format!("0x{record_type:08x}")));
        output.push_str(",\"count\":");
        output.push_str(&count.to_string());
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_embedded_press_texture_bezier_header_summary_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let path_count = embedded_press_snapshot_texture_bezier_header_count(snapshot);
    let Some(header) = embedded_press_snapshot_texture_bezier_header_summary(snapshot) else {
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
    output.push_str(&json_string(&format!("0x{:08x}", header.flags())));
    output.push_str(",\"homogeneous\":");
    output.push_str(
        if embedded_press_snapshot_texture_bezier_headers_are_homogeneous(snapshot) {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(super) fn push_embedded_press_paint_state_transitions_json(
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
        output.push_str(&json_string(key.0.as_str()));
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
        push_option_u32_hex_or_null_json(output, key.1);
        output.push_str(",\"record70Word0\":");
        push_option_u32_hex_or_null_json(output, key.2);
        output.push_str(",\"record70Word3\":");
        push_option_u32_hex_or_null_json(output, key.3);
        output.push_str(",\"record82Word5\":");
        push_option_u32_hex_or_null_json(output, key.4);
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

pub(super) fn embedded_press_path_state_word(
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

pub(super) fn embedded_press_path_state_word_values(
    paths: &[ObjectEmbeddedPressVectorPathCandidate],
    record_type: u32,
    word_index: usize,
) -> Vec<u32> {
    paths
        .iter()
        .filter_map(|path| embedded_press_path_state_word(path, record_type, word_index))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn embedded_press_title_art_shadow_path_partition(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Option<TitleArtShadowPathPartition<'_>> {
    embedded_press_title_art_source_order_shadow_path_partition(snapshot).or_else(|| {
        let outline_paths = success_data_test_title_art_rendered_paths(snapshot);
        success_data_test_title_art_shadow_path_partition(&outline_paths)
    })
}

pub(super) fn embedded_press_title_art_source_order_shadow_path_partition(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Option<TitleArtShadowPathPartition<'_>> {
    let paths = snapshot.vector_paths();
    let mut runs: Vec<(ObjectEmbeddedPressVectorPathKind, usize, usize)> = Vec::new();
    for (path_index, path) in paths.iter().enumerate() {
        if path.commands().is_empty() {
            continue;
        }
        match runs.last_mut() {
            Some((kind, _, end)) if *kind == path.kind() => *end = path_index,
            _ => runs.push((path.kind(), path_index, path_index)),
        }
    }

    for window in runs.windows(3) {
        let (first_kind, first_start, first_end) = window[0];
        let (middle_kind, _middle_start, _middle_end) = window[1];
        let (last_kind, last_start, last_end) = window[2];
        if first_kind != ObjectEmbeddedPressVectorPathKind::Outline
            || middle_kind != ObjectEmbeddedPressVectorPathKind::Texture
            || last_kind != ObjectEmbeddedPressVectorPathKind::Outline
        {
            continue;
        }

        let shadow_paths = paths[first_start..=first_end]
            .iter()
            .filter(|path| !path.commands().is_empty())
            .collect::<Vec<_>>();
        let main_paths = paths[last_start..=last_end]
            .iter()
            .filter(|path| !path.commands().is_empty())
            .collect::<Vec<_>>();
        if shadow_paths.is_empty() || shadow_paths.len() != main_paths.len() {
            continue;
        }

        let Some(offset) = success_data_test_title_art_common_shadow_offset(
            main_paths.as_slice(),
            shadow_paths.as_slice(),
        ) else {
            continue;
        };

        return Some(TitleArtShadowPathPartition {
            main_paths,
            shadow_paths,
            offset,
            strategy: "embedded-press-source-order-outline-texture-outline",
        });
    }

    None
}

pub(super) fn embedded_press_title_art_state_word5(
    path: &ObjectEmbeddedPressVectorPathCandidate,
) -> Option<u32> {
    path.state_records()
        .iter()
        .find(|record| record.record_type() == EMBEDDED_PRESS_RECORD_PAINT_STATE_82)
        .and_then(|record| record.payload_le32_words().get(5).copied())
}

pub(super) fn embedded_press_source_bboxes_match_offset(
    main_bbox: (i32, i32, i32, i32),
    shadow_bbox: (i32, i32, i32, i32),
    offset: (i32, i32),
) -> bool {
    embedded_press_source_bboxes_have_compatible_size(main_bbox, shadow_bbox)
        && (shadow_bbox.0 - main_bbox.0 - offset.0).abs()
            <= EMBEDDED_PRESS_SHADOW_PAIR_BBOX_TOLERANCE_SOURCE_UNITS
        && (shadow_bbox.1 - main_bbox.1 - offset.1).abs()
            <= EMBEDDED_PRESS_SHADOW_PAIR_BBOX_TOLERANCE_SOURCE_UNITS
        && (shadow_bbox.2 - main_bbox.2 - offset.0).abs()
            <= EMBEDDED_PRESS_SHADOW_PAIR_BBOX_TOLERANCE_SOURCE_UNITS
        && (shadow_bbox.3 - main_bbox.3 - offset.1).abs()
            <= EMBEDDED_PRESS_SHADOW_PAIR_BBOX_TOLERANCE_SOURCE_UNITS
}

pub(super) fn embedded_press_source_bboxes_have_compatible_size(
    a: (i32, i32, i32, i32),
    b: (i32, i32, i32, i32),
) -> bool {
    let a_width = a.2 - a.0;
    let a_height = a.3 - a.1;
    let b_width = b.2 - b.0;
    let b_height = b.3 - b.1;
    (a_width - b_width).abs() <= EMBEDDED_PRESS_SHADOW_PAIR_BBOX_TOLERANCE_SOURCE_UNITS
        && (a_height - b_height).abs() <= EMBEDDED_PRESS_SHADOW_PAIR_BBOX_TOLERANCE_SOURCE_UNITS
}

pub(super) fn embedded_press_title_art_shadow_effect(
    shadow_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Option<EmbeddedPressTitleArtShadowEffect> {
    let word0_values = embedded_press_title_art_state_record_word0_values(shadow_paths, 0x70);
    if word0_values.len() != 1 || word0_values[0] > 100 {
        return None;
    }
    let opacity = word0_values[0] as f32 / 100.0;
    let channel = ((1.0 - opacity) * 255.0).round().clamp(0.0, 255.0) as u8;
    Some(EmbeddedPressTitleArtShadowEffect {
        opacity,
        word0: word0_values[0],
        fill_color: format!("#{channel:02x}{channel:02x}{channel:02x}"),
    })
}

pub(super) fn embedded_press_title_art_texture_effect(
    texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    base_fill_color: &str,
) -> Option<EmbeddedPressTitleArtTextureEffect> {
    if texture_paths.is_empty() {
        return None;
    }
    let word0_values = embedded_press_title_art_state_record_word0_values(texture_paths, 0x70);
    if word0_values.len() != 1 || word0_values[0] > 100 {
        return None;
    }
    let opacity = word0_values[0] as f32 / 100.0;
    let fill_color = blend_css_hex_colors("#000000", base_fill_color, opacity)?;
    Some(EmbeddedPressTitleArtTextureEffect {
        opacity,
        word0: word0_values[0],
        base_fill_color: base_fill_color.to_string(),
        fill_color,
    })
}

pub(super) fn embedded_press_title_art_paths_source_bbox(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Option<(i32, i32, i32, i32)> {
    let mut bbox = None;
    for path in paths {
        let path_bbox = embedded_press_vector_path_sampled_source_bbox(
            path,
            SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES,
        )
        .or_else(|| embedded_press_vector_path_source_bbox(path))?;
        bbox = Some(match bbox {
            Some(current) => embedded_press_source_bbox_union(current, path_bbox),
            None => path_bbox,
        });
    }
    bbox
}

pub(super) fn embedded_press_source_bbox_union(
    a: (i32, i32, i32, i32),
    b: (i32, i32, i32, i32),
) -> (i32, i32, i32, i32) {
    (a.0.min(b.0), a.1.min(b.1), a.2.max(b.2), a.3.max(b.3))
}

pub(super) fn embedded_press_source_bbox_offset(
    bbox: (i32, i32, i32, i32),
    offset: (i32, i32),
) -> (i32, i32, i32, i32) {
    (
        bbox.0 + offset.0,
        bbox.1 + offset.1,
        bbox.2 + offset.0,
        bbox.3 + offset.1,
    )
}

pub(super) fn embedded_press_source_bbox_area(bbox: (i32, i32, i32, i32)) -> i64 {
    i64::from((bbox.2 - bbox.0).max(0)) * i64::from((bbox.3 - bbox.1).max(0))
}

pub(super) fn embedded_press_source_bbox_intersection_area(
    a: (i32, i32, i32, i32),
    b: (i32, i32, i32, i32),
) -> i64 {
    let left = a.0.max(b.0);
    let top = a.1.max(b.1);
    let right = a.2.min(b.2);
    let bottom = a.3.min(b.3);
    embedded_press_source_bbox_area((left, top, right, bottom))
}

pub(super) fn embedded_press_source_bbox_area_ratio(part: i64, whole: i64) -> f32 {
    if whole <= 0 {
        0.0
    } else {
        part as f32 / whole as f32
    }
}

pub(super) fn embedded_press_source_bbox_contains(
    outer: (i32, i32, i32, i32),
    inner: (i32, i32, i32, i32),
) -> bool {
    outer.0 <= inner.0 && outer.1 <= inner.1 && inner.2 <= outer.2 && inner.3 <= outer.3
}

pub(super) fn push_embedded_press_source_bbox_option_json(
    output: &mut String,
    bbox: Option<(i32, i32, i32, i32)>,
) {
    let Some((left, top, right, bottom)) = bbox else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"left\":");
    output.push_str(&left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&bottom.to_string());
    output.push('}');
}

pub(super) fn embedded_press_vector_path_refs_match(
    left: &[&ObjectEmbeddedPressVectorPathCandidate],
    right: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right.iter())
            .all(|(left, right)| std::ptr::eq(*left, *right))
}

pub(super) fn embedded_press_title_art_word5_percent_opacity(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Option<f32> {
    let values = embedded_press_vector_path_state_word5_values(paths);
    if values.len() == 1 && values[0] <= 100 {
        Some(values[0] as f32 / 100.0)
    } else {
        None
    }
}

pub(super) fn embedded_press_title_art_record_word0_percent_opacity(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    record_type: u32,
) -> Option<f32> {
    let values = embedded_press_title_art_state_record_word0_values(paths, record_type);
    if values.len() == 1 && values[0] <= 100 {
        Some(values[0] as f32 / 100.0)
    } else {
        None
    }
}

pub(super) fn embedded_press_title_art_front_erase_texture_opacity(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Option<(f32, &'static str)> {
    if !paths.is_empty() && paths.iter().all(|path| !path.state_records().is_empty()) {
        return Some((1.0, "embedded-press-explicit-white-paint-state"));
    }
    embedded_press_title_art_word5_percent_opacity(paths)
        .map(|opacity| (opacity, "embedded-press-0x82-word5-percent"))
        .or_else(|| {
            embedded_press_title_art_record_word0_percent_opacity(
                paths,
                EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70,
            )
            .map(|opacity| (opacity, "embedded-press-0x70-word0-percent"))
        })
}

pub(super) fn embedded_press_title_art_direct_gray_candidate(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Option<(u32, String)> {
    let values = embedded_press_vector_path_state_word5_values(paths);
    if values.len() != 1 || values[0] > 100 {
        return None;
    }
    let channel = ((values[0] as f32 / 100.0) * 255.0)
        .round()
        .clamp(0.0, 255.0) as u8;
    Some((
        values[0],
        format!("#{channel:02x}{channel:02x}{channel:02x}"),
    ))
}

pub(super) fn push_embedded_press_title_art_direct_gray_candidate_json(
    output: &mut String,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let Some((word5, fill_color)) = embedded_press_title_art_direct_gray_candidate(paths) else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"source\":\"embeddedPressRecord82Word5DirectGrayProbe\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"word5\":");
    output.push_str(&word5.to_string());
    output.push_str(",\"fillColor\":");
    output.push_str(&json_string(&fill_color));
    output.push_str(",\"renderPromotionBlockedReason\":\"direct-gray-channel-probe-not-proven-as-paint-semantics\"}");
}

pub(super) fn embedded_press_title_art_direct_gray_candidate_svg_attrs(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> String {
    let Some((word5, fill_color)) = embedded_press_title_art_direct_gray_candidate(paths) else {
        return String::new();
    };
    format!(
        " data-title-texture-direct-gray-candidate-source=\"embeddedPressRecord82Word5DirectGrayProbe\" data-title-texture-direct-gray-candidate-word5=\"0x{word5:02x}\" data-title-texture-direct-gray-candidate-fill=\"{}\" data-title-texture-direct-gray-render-promoted=\"false\" data-title-texture-direct-gray-render-promotion-blocked-reason=\"direct-gray-channel-probe-not-proven-as-paint-semantics\"",
        escape_xml(&fill_color)
    )
}

pub(super) fn embedded_press_title_art_path_paint_state_color_hex(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Option<String> {
    let mut colors = BTreeMap::<u32, usize>::new();
    for path in paths {
        for record in path
            .state_records()
            .iter()
            .filter(|record| record.record_type() == EMBEDDED_PRESS_RECORD_PAINT_STATE_82)
        {
            let Some(color) = record.payload_le32_words().get(3).copied() else {
                continue;
            };
            if color <= 0x00ff_ffff {
                *colors.entry(color).or_default() += 1;
            }
        }
    }

    colors
        .into_iter()
        .max_by_key(|(color, count)| (*count, std::cmp::Reverse(*color)))
        .map(|(color, _)| format!("#{:06x}", color & 0x00ff_ffff))
}

pub(super) fn embedded_press_title_art_source_paint_candidate(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
) -> Option<(String, &'static str, String)> {
    let (paint_color, paint_source) =
        if let Some(color) = source_paint_candidate.and_then(jsfart_paint_candidate_color_hex) {
            (color, "JSFart2Contents.paintColorCandidate")
        } else {
            let color = embedded_press_title_art_path_paint_state_color_hex(paths)?;
            (color, "EmbeddedPress.0x82.word3")
        };
    let active_fill =
        if let Some((opacity, _)) = embedded_press_title_art_front_erase_texture_opacity(paths) {
            blend_css_hex_colors(
                &paint_color,
                SUCCESS_DATA_TEST_TITLE_ART_FRONT_FILL_COLOR,
                opacity,
            )
            .unwrap_or_else(|| paint_color.clone())
        } else {
            paint_color.clone()
        };
    Some((paint_color, paint_source, active_fill))
}

pub(super) fn push_embedded_press_title_art_source_paint_candidate_json(
    output: &mut String,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
) {
    let Some((paint_color, paint_source, active_fill)) =
        embedded_press_title_art_source_paint_candidate(paths, source_paint_candidate)
    else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"source\":\"frontEraseTextureSourcePaintProbe\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"paintColor\":");
    output.push_str(&json_string(&paint_color));
    output.push_str(",\"paintColorSource\":");
    output.push_str(&json_string(paint_source));
    output.push_str(",\"solidPaintFillColor\":");
    output.push_str(&json_string(&paint_color));
    output.push_str(",\"activePrecompositedFillColor\":");
    output.push_str(&json_string(&active_fill));
    output.push_str(
        ",\"activeFillColorSource\":\"source-paint-with-front-erase-opacity-over-front-fill\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"solid-source-paint-semantics-unproven\"}");
}

pub(super) fn embedded_press_title_art_source_paint_candidate_svg_attrs(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
) -> String {
    let Some((paint_color, paint_source, active_fill)) =
        embedded_press_title_art_source_paint_candidate(paths, source_paint_candidate)
    else {
        return String::new();
    };
    format!(
        " data-title-texture-source-paint-candidate-source=\"frontEraseTextureSourcePaintProbe\" data-title-texture-source-paint-candidate-color=\"{}\" data-title-texture-source-paint-candidate-color-source=\"{}\" data-title-texture-solid-paint-candidate-fill=\"{}\" data-title-texture-active-precomposited-fill=\"{}\" data-title-texture-solid-paint-render-promoted=\"false\" data-title-texture-solid-paint-render-promotion-blocked-reason=\"solid-source-paint-semantics-unproven\"",
        escape_xml(&paint_color),
        escape_xml(paint_source),
        escape_xml(&paint_color),
        escape_xml(&active_fill)
    )
}

pub(super) fn embedded_press_title_art_front_erase_texture_state_spans(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Vec<EmbeddedPressTitleArtTextureStateSpan> {
    let snapshot_paths = snapshot.vector_paths();
    let mut path_indexes = paths
        .iter()
        .filter_map(|path| embedded_press_vector_path_index(snapshot_paths, path))
        .collect::<Vec<_>>();
    path_indexes.sort_unstable();
    let Some(last_path_index) = path_indexes.last().copied() else {
        return Vec::new();
    };
    let explicit_path_indexes = paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .filter_map(|path| embedded_press_vector_path_index(snapshot_paths, path))
        .collect::<Vec<_>>();

    explicit_path_indexes
        .iter()
        .enumerate()
        .map(|(span_index, state_path_index)| {
            let inherited_span_end_path_index = explicit_path_indexes
                .get(span_index + 1)
                .map(|index| index.saturating_sub(1))
                .unwrap_or(last_path_index);
            let span_paths = paths
                .iter()
                .filter_map(|path| {
                    let index = embedded_press_vector_path_index(snapshot_paths, path)?;
                    (*state_path_index <= index && index <= inherited_span_end_path_index)
                        .then_some(*path)
                })
                .collect::<Vec<_>>();
            let texture_path_count = span_paths
                .iter()
                .filter(|path| {
                    path.kind() == ObjectEmbeddedPressVectorPathKind::Texture
                        && !path.commands().is_empty()
                })
                .count();
            EmbeddedPressTitleArtTextureStateSpan {
                state_path_index: *state_path_index,
                inherited_span_end_path_index,
                path_count: span_paths.len(),
                texture_path_count,
                record48_word0_values: embedded_press_title_art_state_record_word_values(
                    &span_paths,
                    0x48,
                    0,
                ),
                record70_word0_values: embedded_press_title_art_state_record_word_values(
                    &span_paths,
                    EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70,
                    0,
                ),
                record82_word3_values: embedded_press_title_art_state_record_word_values(
                    &span_paths,
                    EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
                    3,
                ),
                record82_word5_values: embedded_press_title_art_state_record_word_values(
                    &span_paths,
                    EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
                    5,
                ),
            }
        })
        .collect()
}

pub(super) fn push_embedded_press_title_art_front_erase_texture_span_coverage_probe_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    if paths.is_empty() {
        output.push_str("null");
        return;
    }
    let spans = embedded_press_title_art_front_erase_texture_state_spans(snapshot, paths);
    let span_path_counts = spans.iter().map(|span| span.path_count).collect::<Vec<_>>();
    let min_span_path_count = span_path_counts.iter().min().copied();
    let max_span_path_count = span_path_counts.iter().max().copied();
    let mean_span_path_count = if span_path_counts.is_empty() {
        None
    } else {
        Some(span_path_counts.iter().sum::<usize>() as f32 / span_path_counts.len() as f32)
    };
    let explicit_state_texture_path_count = paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();

    output.push_str("{\"source\":\"embeddedPressExplicitTextureStateSpans\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"texturePathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"explicitStateTexturePathCount\":");
    output.push_str(&explicit_state_texture_path_count.to_string());
    output.push_str(",\"inheritedTexturePathCount\":");
    output.push_str(
        &paths
            .len()
            .saturating_sub(explicit_state_texture_path_count)
            .to_string(),
    );
    output.push_str(",\"spanCount\":");
    output.push_str(&spans.len().to_string());
    output.push_str(",\"spanPathCounts\":");
    push_usize_array_json(output, &span_path_counts);
    output.push_str(",\"minSpanPathCount\":");
    push_option_usize_json(output, min_span_path_count);
    output.push_str(",\"maxSpanPathCount\":");
    push_option_usize_json(output, max_span_path_count);
    output.push_str(",\"meanSpanPathCount\":");
    if let Some(mean) = mean_span_path_count {
        output.push_str(&format!("{mean:.3}"));
    } else {
        output.push_str("null");
    }
    output.push_str(
        ",\"coverageConclusion\":\"explicit-state-spans-cover-all-front-erase-texture-paths\"",
    );
    output
        .push_str(",\"renderPromotionBlockedReason\":\"span-density-and-clip-semantics-unproven\"");
    output.push_str(",\"spans\":[");
    for (index, span) in spans.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"statePathIndex\":");
        output.push_str(&span.state_path_index.to_string());
        output.push_str(",\"inheritedSpanEndPathIndex\":");
        output.push_str(&span.inherited_span_end_path_index.to_string());
        output.push_str(",\"pathCount\":");
        output.push_str(&span.path_count.to_string());
        output.push_str(",\"texturePathCount\":");
        output.push_str(&span.texture_path_count.to_string());
        output.push_str(",\"record48Word0Values\":");
        push_u32_hex_array_json(output, &span.record48_word0_values);
        output.push_str(",\"record70Word0Values\":");
        push_u32_hex_array_json(output, &span.record70_word0_values);
        output.push_str(",\"record82Word3Values\":");
        push_u32_hex_array_json(output, &span.record82_word3_values);
        output.push_str(",\"record82Word5Values\":");
        push_u32_hex_array_json(output, &span.record82_word5_values);
        output.push('}');
    }
    output.push_str("]}");
}

pub(super) fn embedded_press_title_art_front_erase_texture_span_coverage_svg_attrs(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> String {
    if paths.is_empty() {
        return String::new();
    }
    let spans = embedded_press_title_art_front_erase_texture_state_spans(snapshot, paths);
    let span_path_counts = spans.iter().map(|span| span.path_count).collect::<Vec<_>>();
    let span_path_counts_attr = span_path_counts
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let explicit_state_texture_path_count = paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();
    format!(
        " data-title-texture-span-coverage-source=\"embeddedPressExplicitTextureStateSpans\" data-title-texture-span-count=\"{}\" data-title-texture-span-path-counts=\"{}\" data-title-texture-explicit-state-span-path-count=\"{}\" data-title-texture-inherited-span-path-count=\"{}\" data-title-texture-span-render-promoted=\"false\" data-title-texture-span-render-promotion-blocked-reason=\"span-density-and-clip-semantics-unproven\"",
        spans.len(),
        escape_xml(&span_path_counts_attr),
        explicit_state_texture_path_count,
        paths
            .len()
            .saturating_sub(explicit_state_texture_path_count)
    )
}

pub(super) fn embedded_press_title_art_front_erase_paint_transition_gate_svg_attrs(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> String {
    if paths.is_empty() {
        return String::new();
    }
    let gate = success_data_test_title_art_front_erase_paint_transition_gate(snapshot, paths);
    format!(
        " data-title-front-erase-transition-gate=\"embeddedPressVectorPathSourceOrder+stateTransitions\" data-title-front-erase-transition-boundary-class=\"{}\" data-title-front-erase-paint-intent-inference=\"{}\" data-title-front-erase-transition-promotion-ready=\"{}\" data-title-front-erase-transition-blocked-reason=\"{}\" data-title-front-erase-record48-separates-texture-from-main=\"{}\" data-title-front-erase-record70-word0-separates-texture-from-main=\"{}\" data-title-front-erase-record82-word5-separates-texture-from-main=\"{}\" data-title-front-erase-record82-word5-matches-shadow=\"{}\" data-title-front-erase-record82-word3-white-paint-candidate=\"{}\"",
        escape_xml(gate.transition_boundary_class),
        escape_xml(gate.paint_intent_inference),
        gate.promotion_ready(),
        escape_xml(gate.render_promotion_blocked_reason),
        gate.record48_separates_texture_from_main,
        gate.record70_word0_separates_texture_from_main,
        gate.record82_word5_separates_texture_from_main,
        gate.record82_word5_matches_shadow,
        gate.record82_word3_is_white_paint_candidate
    )
}

pub(super) fn embedded_press_title_art_state_record_word0_values(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    record_type: u32,
) -> Vec<u32> {
    embedded_press_title_art_state_record_word_values(paths, record_type, 0)
}

pub(super) fn embedded_press_title_art_state_record_word_values(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
    record_type: u32,
    word_index: usize,
) -> Vec<u32> {
    paths
        .iter()
        .flat_map(|path| path.state_records().iter())
        .filter(|record| record.record_type() == record_type)
        .filter_map(|record| record.payload_le32_words().get(word_index).copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn embedded_press_vector_path_index(
    paths: &[ObjectEmbeddedPressVectorPathCandidate],
    target: &ObjectEmbeddedPressVectorPathCandidate,
) -> Option<usize> {
    paths.iter().position(|path| std::ptr::eq(path, target))
}

pub(super) fn embedded_press_vector_path_indexes(
    snapshot_paths: &[ObjectEmbeddedPressVectorPathCandidate],
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Vec<usize> {
    paths
        .iter()
        .filter_map(|path| embedded_press_vector_path_index(snapshot_paths, path))
        .collect()
}

pub(super) fn push_embedded_press_single_path_state_word_sequence_json(
    output: &mut String,
    path: &ObjectEmbeddedPressVectorPathCandidate,
    record_type: u32,
    word_index: usize,
) {
    let values = path
        .state_records()
        .iter()
        .filter(|record| record.record_type() == record_type)
        .filter_map(|record| record.payload_le32_words().get(word_index).copied())
        .collect::<Vec<_>>();
    push_u32_hex_array_json(output, &values);
}

pub(super) fn push_embedded_press_path_state_records_json(
    output: &mut String,
    path: &ObjectEmbeddedPressVectorPathCandidate,
) {
    output.push('[');
    for (record_index, record) in path.state_records().iter().enumerate() {
        if record_index > 0 {
            output.push(',');
        }
        let words = record.payload_le32_words();
        output.push_str("{\"recordIndex\":");
        output.push_str(&record_index.to_string());
        output.push_str(",\"recordType\":");
        output.push_str(&record.record_type().to_string());
        output.push_str(",\"recordTypeHex\":");
        output.push_str(&json_string(&format!("0x{:02x}", record.record_type())));
        output.push_str(",\"recordOffset\":");
        output.push_str(&record.offset().to_string());
        output.push_str(",\"payloadByteLength\":");
        output.push_str(&record.payload().len().to_string());
        output.push_str(",\"wordCount\":");
        output.push_str(&words.len().to_string());
        output.push_str(",\"words\":");
        push_u32_hex_array_json(output, &words);
        output.push_str(",\"payloadHex\":");
        output.push_str(&json_string(&hex_bytes(record.payload())));
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_embedded_press_path_texture_bezier_header_json(
    output: &mut String,
    path: &ObjectEmbeddedPressVectorPathCandidate,
) {
    let Some(header) = path.texture_bezier_header() else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"pointCount\":");
    output.push_str(&header.point_count().to_string());
    output.push_str(",\"byteCount\":");
    output.push_str(&header.byte_count().to_string());
    output.push_str(",\"flags\":");
    output.push_str(&header.flags().to_string());
    output.push_str(",\"flagsHex\":");
    output.push_str(&json_string(&format!("0x{:08x}", header.flags())));
    output.push('}');
}

pub(super) fn embedded_press_title_art_role_state_word_value_sets(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> BTreeMap<(u32, usize), BTreeSet<u32>> {
    let mut values = BTreeMap::<(u32, usize), BTreeSet<u32>>::new();
    for record in paths.iter().flat_map(|path| path.state_records().iter()) {
        for (word_index, word) in record.payload_le32_words().into_iter().enumerate() {
            values
                .entry((record.record_type(), word_index))
                .or_default()
                .insert(word);
        }
    }
    values
}

pub(super) fn embedded_press_title_art_path_state_summary_svg_attrs(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> String {
    let snapshot_paths = snapshot.vector_paths();
    let path_indexes = paths
        .iter()
        .filter_map(|path| embedded_press_vector_path_index(snapshot_paths, path))
        .collect::<Vec<_>>();
    let explicit_state_path_count = paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();
    let state_record_count = paths
        .iter()
        .map(|path| path.state_records().len())
        .sum::<usize>();
    format!(
        " data-title-texture-first-path-index=\"{}\" data-title-texture-last-path-index=\"{}\" data-title-texture-explicit-state-path-count=\"{}\" data-title-texture-inherited-state-path-count=\"{}\" data-title-texture-state-record-count=\"{}\"",
        path_indexes
            .iter()
            .min()
            .map(|index| index.to_string())
            .unwrap_or_default(),
        path_indexes
            .iter()
            .max()
            .map(|index| index.to_string())
            .unwrap_or_default(),
        explicit_state_path_count,
        paths.len().saturating_sub(explicit_state_path_count),
        state_record_count
    )
}

pub(super) fn embedded_press_title_art_path_kind_summary(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> &'static str {
    let mut has_outline = false;
    let mut has_texture = false;
    for path in paths {
        match path.kind() {
            ObjectEmbeddedPressVectorPathKind::Outline => has_outline = true,
            ObjectEmbeddedPressVectorPathKind::Texture => has_texture = true,
        }
    }
    match (has_outline, has_texture) {
        (true, false) => "outline",
        (false, true) => "texture",
        (false, false) => "none",
        (true, true) => "mixed",
    }
}

pub(super) fn push_embedded_press_state_payload_signatures_json(
    output: &mut String,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let mut signatures: BTreeMap<u32, BTreeMap<Vec<u32>, usize>> = BTreeMap::new();
    for record in paths.iter().flat_map(|path| path.state_records().iter()) {
        *signatures
            .entry(record.record_type())
            .or_default()
            .entry(record.payload_le32_words())
            .or_default() += 1;
    }

    output.push('[');
    for (index, (record_type, payloads)) in signatures.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let record_count = payloads.values().sum::<usize>();
        output.push_str("{\"recordType\":");
        output.push_str(&record_type.to_string());
        output.push_str(",\"recordTypeHex\":");
        output.push_str(&json_string(&format!("0x{record_type:02x}")));
        output.push_str(",\"recordCount\":");
        output.push_str(&record_count.to_string());
        output.push_str(",\"uniquePayloadCount\":");
        output.push_str(&payloads.len().to_string());
        output.push_str(",\"payloads\":[");
        for (payload_index, (words, count)) in payloads
            .iter()
            .take(SUCCESS_DATA_TEST_TITLE_ART_STATE_SIGNATURE_PREVIEW_LIMIT)
            .enumerate()
        {
            if payload_index > 0 {
                output.push(',');
            }
            output.push_str("{\"count\":");
            output.push_str(&count.to_string());
            output.push_str(",\"wordCount\":");
            output.push_str(&words.len().to_string());
            output.push_str(",\"words\":");
            push_u32_hex_array_json(output, words);
            output.push('}');
        }
        output.push_str("],\"truncated\":");
        output.push_str(
            &(payloads.len() > SUCCESS_DATA_TEST_TITLE_ART_STATE_SIGNATURE_PREVIEW_LIMIT)
                .to_string(),
        );
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_embedded_press_state_payload_word_columns_json(
    output: &mut String,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let mut columns = BTreeMap::<u32, BTreeMap<usize, BTreeSet<u32>>>::new();
    let mut max_word_counts = BTreeMap::<u32, usize>::new();
    let mut record_counts = BTreeMap::<u32, usize>::new();
    for record in paths.iter().flat_map(|path| path.state_records().iter()) {
        let record_type = record.record_type();
        let words = record.payload_le32_words();
        *record_counts.entry(record_type).or_default() += 1;
        max_word_counts
            .entry(record_type)
            .and_modify(|count| *count = (*count).max(words.len()))
            .or_insert(words.len());
        for (word_index, word) in words.into_iter().enumerate() {
            columns
                .entry(record_type)
                .or_default()
                .entry(word_index)
                .or_default()
                .insert(word);
        }
    }

    output.push('[');
    for (record_index, (record_type, word_columns)) in columns.iter().enumerate() {
        if record_index > 0 {
            output.push(',');
        }
        output.push_str("{\"recordType\":");
        output.push_str(&record_type.to_string());
        output.push_str(",\"recordTypeHex\":");
        output.push_str(&json_string(&format!("0x{record_type:02x}")));
        output.push_str(",\"recordCount\":");
        output.push_str(
            &record_counts
                .get(record_type)
                .copied()
                .unwrap_or(0)
                .to_string(),
        );
        output.push_str(",\"maxWordCount\":");
        output.push_str(
            &max_word_counts
                .get(record_type)
                .copied()
                .unwrap_or(0)
                .to_string(),
        );
        output.push_str(",\"columns\":[");
        for (column_index, (word_index, values)) in word_columns.iter().enumerate() {
            if column_index > 0 {
                output.push(',');
            }
            output.push_str("{\"wordIndex\":");
            output.push_str(&word_index.to_string());
            output.push_str(",\"values\":");
            push_u32_hex_array_json(output, &values.iter().copied().collect::<Vec<_>>());
            output.push('}');
        }
        output.push_str("]}");
    }
    output.push(']');
}

pub(super) fn embedded_press_effective_texture_paths_for_state_word(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    record_type: u32,
    word_index: usize,
    expected_value: u32,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    let mut current_value = None;
    let mut texture_paths = Vec::new();
    for path in snapshot.vector_paths() {
        if let Some(value) = embedded_press_path_state_word(path, record_type, word_index) {
            current_value = Some(value);
        }
        if path.kind() == ObjectEmbeddedPressVectorPathKind::Texture
            && !path.commands().is_empty()
            && current_value == Some(expected_value)
        {
            texture_paths.push(path);
        }
    }
    texture_paths
}

pub(super) fn embedded_press_effective_texture_state_word_values(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    record_type: u32,
    word_index: usize,
) -> Vec<u32> {
    let mut current_value = None;
    let mut values = BTreeSet::new();
    for path in snapshot.vector_paths() {
        if let Some(value) = embedded_press_path_state_word(path, record_type, word_index) {
            current_value = Some(value);
        }
        if path.kind() == ObjectEmbeddedPressVectorPathKind::Texture
            && !path.commands().is_empty()
            && let Some(value) = current_value
        {
            values.insert(value);
        }
    }
    values.into_iter().collect::<Vec<_>>()
}

pub(super) fn embedded_press_vector_path_state_word5_values(
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> Vec<u32> {
    paths
        .iter()
        .filter_map(|path| embedded_press_title_art_state_word5(path))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}

pub(super) fn embedded_press_state_word5_values_attr(values: &[u32]) -> String {
    values
        .iter()
        .map(|value| format!("0x{value:02x}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn embedded_press_snapshot_paint_state_color_hex(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Option<String> {
    let mut colors = BTreeMap::<u32, usize>::new();
    for path in snapshot.vector_paths() {
        for record in path
            .state_records()
            .iter()
            .filter(|record| record.record_type() == EMBEDDED_PRESS_RECORD_PAINT_STATE_82)
        {
            let Some(color) = record.payload_le32_words().get(3).copied() else {
                continue;
            };
            if color <= 0x00ff_ffff {
                *colors.entry(color).or_default() += 1;
            }
        }
    }

    colors
        .into_iter()
        .max_by_key(|(color, count)| (*count, std::cmp::Reverse(*color)))
        .map(|(color, _)| format!("#{:06x}", color & 0x00ff_ffff))
}

pub(super) fn push_embedded_press_vector_path_svg(
    svg: &mut String,
    class_name: &str,
    path: &ObjectEmbeddedPressVectorPathCandidate,
    ctx: EmbeddedPressPageContext,
    fill: &str,
    fill_rule: &str,
    extra_attrs: Option<&str>,
) {
    push_embedded_press_vector_path_svg_inner(
        svg,
        class_name,
        path,
        ctx,
        0.0,
        0.0,
        fill,
        fill_rule,
        None,
        extra_attrs,
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_embedded_press_vector_path_svg_with_stroke(
    svg: &mut String,
    class_name: &str,
    path: &ObjectEmbeddedPressVectorPathCandidate,
    ctx: EmbeddedPressPageContext,
    fill: &str,
    fill_rule: &str,
    stroke: &str,
    stroke_width: f32,
    extra_attrs: Option<&str>,
) {
    push_embedded_press_vector_path_svg_inner(
        svg,
        class_name,
        path,
        ctx,
        0.0,
        0.0,
        fill,
        fill_rule,
        Some((stroke, stroke_width)),
        extra_attrs,
    );
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub(super) fn push_embedded_press_vector_path_svg_with_source_offset(
    svg: &mut String,
    class_name: &str,
    path: &ObjectEmbeddedPressVectorPathCandidate,
    ctx: EmbeddedPressPageContext,
    source_offset_x: f32,
    source_offset_y: f32,
    fill: &str,
    fill_rule: &str,
    extra_attrs: Option<&str>,
) {
    push_embedded_press_vector_path_svg_inner(
        svg,
        class_name,
        path,
        ctx,
        source_offset_x,
        source_offset_y,
        fill,
        fill_rule,
        None,
        extra_attrs,
    );
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub(super) fn push_embedded_press_vector_path_svg_with_source_offset_and_stroke(
    svg: &mut String,
    class_name: &str,
    path: &ObjectEmbeddedPressVectorPathCandidate,
    ctx: EmbeddedPressPageContext,
    source_offset_x: f32,
    source_offset_y: f32,
    fill: &str,
    fill_rule: &str,
    stroke: Option<(&str, f32)>,
    extra_attrs: Option<&str>,
) {
    push_embedded_press_vector_path_svg_inner(
        svg,
        class_name,
        path,
        ctx,
        source_offset_x,
        source_offset_y,
        fill,
        fill_rule,
        stroke,
        extra_attrs,
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_embedded_press_vector_path_svg_inner(
    svg: &mut String,
    class_name: &str,
    path: &ObjectEmbeddedPressVectorPathCandidate,
    ctx: EmbeddedPressPageContext,
    source_offset_x: f32,
    source_offset_y: f32,
    fill: &str,
    fill_rule: &str,
    stroke: Option<(&str, f32)>,
    extra_attrs: Option<&str>,
) {
    let EmbeddedPressPageContext {
        x,
        y,
        scale_x,
        scale_y,
    } = ctx;
    svg.push_str(&format!(
        "<path class=\"{}\" data-path-kind=\"{}\" d=\"",
        escape_xml(class_name),
        path.kind().as_str()
    ));
    for command in path.commands() {
        match command {
            ObjectEmbeddedPressVectorPathCommandCandidate::MoveTo { x: px, y: py } => {
                let page_x = x + (*px as f32 + source_offset_x) * scale_x;
                let page_y = y + (*py as f32 + source_offset_y) * scale_y;
                svg.push_str(&format!("M {page_x:.2} {page_y:.2} "));
            }
            ObjectEmbeddedPressVectorPathCommandCandidate::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
            } => {
                let page_x1 = x + (*x1 as f32 + source_offset_x) * scale_x;
                let page_y1 = y + (*y1 as f32 + source_offset_y) * scale_y;
                let page_x2 = x + (*x2 as f32 + source_offset_x) * scale_x;
                let page_y2 = y + (*y2 as f32 + source_offset_y) * scale_y;
                let page_x3 = x + (*x3 as f32 + source_offset_x) * scale_x;
                let page_y3 = y + (*y3 as f32 + source_offset_y) * scale_y;
                svg.push_str(&format!(
                    "C {page_x1:.2} {page_y1:.2}, {page_x2:.2} {page_y2:.2}, {page_x3:.2} {page_y3:.2} "
                ));
            }
            ObjectEmbeddedPressVectorPathCommandCandidate::Close => {
                svg.push_str("Z ");
            }
        }
    }
    let stroke_attrs = if let Some((stroke, stroke_width)) =
        stroke.filter(|(_, width)| *width > 0.0)
    {
        format!(
            "\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{stroke_width:.2}\" stroke-linejoin=\"round\" stroke-linecap=\"round\" fill-rule=\"{}\"{}",
            escape_xml(fill),
            escape_xml(stroke),
            escape_xml(fill_rule),
            extra_attrs.unwrap_or("")
        )
    } else {
        format!(
            "\" fill=\"{}\" stroke=\"none\" fill-rule=\"{}\"{}",
            escape_xml(fill),
            escape_xml(fill_rule),
            extra_attrs.unwrap_or("")
        )
    };
    svg.push_str(&format!("{stroke_attrs}/>"));
}

pub(super) fn embedded_press_vector_path_data(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) -> Option<String> {
    embedded_press_vector_path_data_with_source_offset(path, x, y, scale_x, scale_y, 0.0, 0.0)
}

pub(super) fn embedded_press_vector_path_data_with_source_offset(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
    source_offset_x: f32,
    source_offset_y: f32,
) -> Option<String> {
    if path.commands().is_empty() {
        return None;
    }

    let mut path_data = String::new();
    for command in path.commands() {
        match command {
            ObjectEmbeddedPressVectorPathCommandCandidate::MoveTo { x: px, y: py } => {
                let page_x = x + (*px as f32 + source_offset_x) * scale_x;
                let page_y = y + (*py as f32 + source_offset_y) * scale_y;
                path_data.push_str(&format!("M {page_x:.2} {page_y:.2} "));
            }
            ObjectEmbeddedPressVectorPathCommandCandidate::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
            } => {
                let page_x1 = x + (*x1 as f32 + source_offset_x) * scale_x;
                let page_y1 = y + (*y1 as f32 + source_offset_y) * scale_y;
                let page_x2 = x + (*x2 as f32 + source_offset_x) * scale_x;
                let page_y2 = y + (*y2 as f32 + source_offset_y) * scale_y;
                let page_x3 = x + (*x3 as f32 + source_offset_x) * scale_x;
                let page_y3 = y + (*y3 as f32 + source_offset_y) * scale_y;
                path_data.push_str(&format!(
                    "C {page_x1:.2} {page_y1:.2}, {page_x2:.2} {page_y2:.2}, {page_x3:.2} {page_y3:.2} "
                ));
            }
            ObjectEmbeddedPressVectorPathCommandCandidate::Close => {
                path_data.push_str("Z ");
            }
        }
    }
    (!path_data.is_empty()).then_some(path_data)
}

pub(super) fn push_embedded_press_snapshot_vector_svg(
    svg: &mut String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    if snapshot.vector_segments().is_empty() || snapshot.width() == 0 || snapshot.height() == 0 {
        return;
    }
    let scale_x = width / snapshot.width() as f32;
    let scale_y = height / snapshot.height() as f32;
    svg.push_str(&format!(
        "<g class=\"rjtd-embedded-press-snapshot-vector\" data-projection=\"embeddedPressSnapshotVectorProjection\" data-embedding-index=\"{}\" data-vector-segment-count=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\">",
        diagnostic.frame.embedding_index(),
        snapshot.vector_segments().len()
    ));
    for segment in snapshot.vector_segments() {
        let x1 = x + segment.x1() as f32 * scale_x;
        let y1 = y + segment.y1() as f32 * scale_y;
        let x2 = x + segment.x2() as f32 * scale_x;
        let y2 = y + segment.y2() as f32 * scale_y;
        svg.push_str(&format!(
            "<line x1=\"{x1:.2}\" y1=\"{y1:.2}\" x2=\"{x2:.2}\" y2=\"{y2:.2}\" stroke=\"#111111\" stroke-width=\"0.42\" stroke-linecap=\"round\"/>"
        ));
    }
    svg.push_str("</g>");
}

pub(super) fn embedded_press_vector_path_offset_delta(
    main_path: &ObjectEmbeddedPressVectorPathCandidate,
    shadow_path: &ObjectEmbeddedPressVectorPathCandidate,
) -> Option<(i32, i32)> {
    let main_bbox = embedded_press_vector_path_sampled_source_bbox(
        main_path,
        SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES,
    )
    .or_else(|| embedded_press_vector_path_source_bbox(main_path))?;
    let shadow_bbox = embedded_press_vector_path_sampled_source_bbox(
        shadow_path,
        SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES,
    )
    .or_else(|| embedded_press_vector_path_source_bbox(shadow_path))?;
    let offset_x = shadow_bbox.0 - main_bbox.0;
    let offset_y = shadow_bbox.1 - main_bbox.1;
    if offset_x == 0 && offset_y == 0 {
        return None;
    }
    Some((offset_x, offset_y))
}

pub(super) fn embedded_press_vector_path_sampled_contours(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    curve_samples: usize,
) -> Vec<Vec<(f32, f32)>> {
    let samples = curve_samples.max(1);
    let mut contours = Vec::new();
    let mut contour = Vec::new();
    let mut current = None;
    let mut contour_start = None;

    for command in path.commands() {
        match command {
            ObjectEmbeddedPressVectorPathCommandCandidate::MoveTo { x, y } => {
                if contour.len() > 1 {
                    contours.push(std::mem::take(&mut contour));
                } else {
                    contour.clear();
                }
                let point = (*x as f32, *y as f32);
                contour.push(point);
                current = Some(point);
                contour_start = Some(point);
            }
            ObjectEmbeddedPressVectorPathCommandCandidate::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
            } => {
                let Some(start) = current else {
                    continue;
                };
                let c1 = (*x1 as f32, *y1 as f32);
                let c2 = (*x2 as f32, *y2 as f32);
                let end = (*x3 as f32, *y3 as f32);
                for step in 1..=samples {
                    let t = step as f32 / samples as f32;
                    contour.push(cubic_bezier_point(start, c1, c2, end, t));
                }
                current = Some(end);
            }
            ObjectEmbeddedPressVectorPathCommandCandidate::Close => {
                if let (Some(start), Some(last)) = (contour_start, current)
                    && ((start.0 - last.0).abs() > f32::EPSILON
                        || (start.1 - last.1).abs() > f32::EPSILON)
                {
                    contour.push(start);
                }
                if contour.len() > 1 {
                    contours.push(std::mem::take(&mut contour));
                } else {
                    contour.clear();
                }
                current = contour_start;
                contour_start = None;
            }
        }
    }
    if contour.len() > 1 {
        contours.push(contour);
    }

    contours
}

pub(super) fn embedded_press_vector_path_evenodd_boundary_contours(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    curve_samples: usize,
) -> Vec<Vec<(f32, f32)>> {
    embedded_press_vector_path_sampled_contours(path, curve_samples)
        .into_iter()
        .filter(|contour| {
            let area = embedded_press_sampled_contour_signed_area(contour);
            area.abs() > f32::EPSILON
        })
        .collect()
}

pub(super) fn embedded_press_sampled_contour_signed_area(contour: &[(f32, f32)]) -> f32 {
    if contour.len() < 3 {
        return 0.0;
    }
    contour
        .iter()
        .zip(contour.iter().cycle().skip(1))
        .take(contour.len())
        .map(|(a, b)| a.0 * b.1 - b.0 * a.1)
        .sum::<f32>()
        * 0.5
}

pub(super) fn embedded_press_source_point_to_page(
    point: (f32, f32),
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) -> (f32, f32) {
    (x + point.0 * scale_x, y + point.1 * scale_y)
}

pub(super) fn embedded_press_vector_path_sampled_source_bbox(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    curve_samples: usize,
) -> Option<(i32, i32, i32, i32)> {
    let contours = embedded_press_vector_path_sampled_contours(path, curve_samples);
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut has_point = false;
    for contour in contours {
        for (x, y) in contour {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            has_point = true;
        }
    }
    has_point.then(|| {
        (
            min_x.floor() as i32,
            min_y.floor() as i32,
            max_x.ceil() as i32,
            max_y.ceil() as i32,
        )
    })
}

pub(super) fn embedded_press_vector_path_source_bbox(
    path: &ObjectEmbeddedPressVectorPathCandidate,
) -> Option<(i32, i32, i32, i32)> {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut has_point = false;
    for command in path.commands() {
        match command {
            ObjectEmbeddedPressVectorPathCommandCandidate::MoveTo { x, y } => {
                let x = *x as i32;
                let y = *y as i32;
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                has_point = true;
            }
            ObjectEmbeddedPressVectorPathCommandCandidate::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
            } => {
                for (x, y) in [(*x1, *y1), (*x2, *y2), (*x3, *y3)] {
                    let x = x as i32;
                    let y = y as i32;
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    has_point = true;
                }
            }
            ObjectEmbeddedPressVectorPathCommandCandidate::Close => {}
        }
    }
    has_point.then_some((min_x, min_y, max_x, max_y))
}

pub(super) fn embedded_press_snapshot_vector_path_kind_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    kind: ObjectEmbeddedPressVectorPathKind,
) -> usize {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| path.kind() == kind)
        .count()
}

pub(super) fn embedded_press_snapshot_vector_path_state_record_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    snapshot
        .vector_paths()
        .iter()
        .map(|path| path.state_records().len())
        .sum()
}

pub(super) fn embedded_press_snapshot_state_record_type_counts(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> BTreeMap<u32, usize> {
    let mut counts = BTreeMap::new();
    for record in snapshot
        .vector_paths()
        .iter()
        .flat_map(ObjectEmbeddedPressVectorPathCandidate::state_records)
    {
        *counts.entry(record.record_type()).or_default() += 1;
    }
    counts
}

pub(super) fn embedded_press_snapshot_texture_bezier_header_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| path.texture_bezier_header().is_some())
        .count()
}

pub(super) fn embedded_press_snapshot_texture_bezier_header_summary(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Option<ObjectEmbeddedPressTextureBezierHeaderCandidate> {
    snapshot
        .vector_paths()
        .iter()
        .find_map(ObjectEmbeddedPressVectorPathCandidate::texture_bezier_header)
}

pub(super) fn embedded_press_snapshot_texture_bezier_headers_are_homogeneous(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> bool {
    let Some(first) = embedded_press_snapshot_texture_bezier_header_summary(snapshot) else {
        return true;
    };
    snapshot
        .vector_paths()
        .iter()
        .filter_map(ObjectEmbeddedPressVectorPathCandidate::texture_bezier_header)
        .all(|header| header == first)
}
