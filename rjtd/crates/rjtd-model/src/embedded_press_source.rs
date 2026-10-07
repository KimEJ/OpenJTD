use crate::*;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_MAGIC: &[u8; 12] = b"JSSnapShot32";

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET: usize = 0x24;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET: usize = 0x2c;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET: usize = 0x34;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET: usize = 0x38;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET: usize = 0x3c;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET: usize = 0x48;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET: usize = 0x4c;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_VECTOR_SCAN_OFFSET: usize = 0x4a;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_VECTOR_SEGMENT_LIMIT: usize = 16_384;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_RECORD_OFFSET: usize = 0x80;

pub(crate) const EMBEDDED_PRESS_SNAPSHOT_MAX_VECTOR_PATHS: usize = 4096;

#[cfg(feature = "rendering")]
pub(crate) const EMBEDDED_PRESS_SHADOW_PAIR_BBOX_TOLERANCE_SOURCE_UNITS: i32 = 4;

pub(crate) const EMBEDDED_PRESS_RECORD_BEGIN_PATH: u32 = 0x4c;

pub(crate) const EMBEDDED_PRESS_RECORD_END_PATH: u32 = 0x4d;

pub(crate) const EMBEDDED_PRESS_RECORD_MOVE_TO: u32 = 0xd0;

pub(crate) const EMBEDDED_PRESS_RECORD_BEZIER_TO: u32 = 0xd7;

pub(crate) const EMBEDDED_PRESS_RECORD_CLOSE_PATH: u32 = 0xd1;

pub(crate) const EMBEDDED_PRESS_RECORD_TEXTURE_BEZIER: u32 = 0xc6;

#[cfg(feature = "rendering")]
pub(crate) const EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70: u32 = 0x70;

#[cfg(feature = "rendering")]
pub(crate) const EMBEDDED_PRESS_RECORD_PAINT_STATE_82: u32 = 0x82;

#[cfg(feature = "rendering")]
pub(crate) const EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5: u32 = 0x10;

#[cfg(feature = "rendering")]
pub(crate) const EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5: u32 = 0x2f;

pub(crate) fn embedded_press_snapshot_candidate_from_stream(
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

pub(crate) fn embedded_press_snapshot_vector_paths(
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

pub(crate) fn push_embedded_press_bezier_record(
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

pub(crate) fn push_embedded_press_texture_bezier_record(
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

pub(crate) fn embedded_press_texture_bezier_header(
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

pub(crate) fn embedded_press_record_points(
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

pub(crate) fn embedded_press_path_point(
    payload: &[u8],
    offset: usize,
    width: u32,
    height: u32,
) -> Option<(u32, u32)> {
    let x = read_le32_at(payload, offset)?;
    let y = read_le32_at(payload, offset + 4)?;
    (x <= width && y <= height).then_some((x, y))
}

pub(crate) fn embedded_press_snapshot_vector_segments(
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
