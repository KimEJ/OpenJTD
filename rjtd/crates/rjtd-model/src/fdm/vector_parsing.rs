use super::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FdmVectorSegmentHeader {
    pub(crate) declared_len: u16,
    pub(crate) command_count: u16,
    pub(crate) command_offsets: Vec<u16>,
    pub(crate) bbox: Option<ObjectFdmIndexBbox>,
    pub(crate) source_width: i32,
    pub(crate) source_height: i32,
}

pub(crate) fn fdm_raw_vector_segment_candidates(
    vector_stream: &[u8],
) -> Vec<ObjectFdmVectorSegmentCandidate> {
    let mut segments = Vec::new();
    let mut offset = 0usize;
    while offset + FDM_VECTOR_SEGMENT_HEADER_BYTES <= vector_stream.len() {
        if !vector_stream[offset..].starts_with(FDM_VECTOR_SEGMENT_MAGIC) {
            offset += 1;
            continue;
        }
        let Some(header) = fdm_vector_segment_header(&vector_stream[offset..]) else {
            offset += 1;
            continue;
        };
        let declared_len = usize::from(header.declared_len).max(1);
        segments.push(ObjectFdmVectorSegmentCandidate::new(offset, header));
        offset += declared_len;
    }
    segments
}

pub(crate) fn fdm_vector_segment_header(segment: &[u8]) -> Option<FdmVectorSegmentHeader> {
    if segment.len() < FDM_VECTOR_SEGMENT_HEADER_BYTES
        || !segment.starts_with(FDM_VECTOR_SEGMENT_MAGIC)
    {
        return None;
    }

    let declared_len = read_be16_at(segment, 4)?;
    let command_count = read_be16_at(segment, 6)?;
    let declared_len_usize = usize::from(declared_len);
    let command_count_usize = usize::from(command_count);
    if declared_len_usize < FDM_VECTOR_SEGMENT_HEADER_BYTES || declared_len_usize > segment.len() {
        return None;
    }

    let offset_table_end =
        FDM_VECTOR_SEGMENT_HEADER_BYTES + command_count_usize * FDM_VECTOR_COMMAND_OFFSET_BYTES;
    if offset_table_end > declared_len_usize {
        return None;
    }

    let mut command_offsets = Vec::with_capacity(command_count_usize);
    for command_index in 0..command_count_usize {
        let offset_start =
            FDM_VECTOR_SEGMENT_HEADER_BYTES + command_index * FDM_VECTOR_COMMAND_OFFSET_BYTES;
        let offset = read_be16_at(segment, offset_start)?;
        let offset_usize = usize::from(offset);
        if offset_usize < offset_table_end || offset_usize >= declared_len_usize {
            return None;
        }
        command_offsets.push(offset);
    }

    let bbox = Some(ObjectFdmIndexBbox::new(
        read_i32_be_at(segment, 20)?,
        read_i32_be_at(segment, 24)?,
        read_i32_be_at(segment, 28)?,
        read_i32_be_at(segment, 32)?,
    ));
    let source_width = read_i32_be_at(segment, 36).unwrap_or_default();
    let source_height = read_i32_be_at(segment, 40).unwrap_or_default();
    Some(FdmVectorSegmentHeader {
        declared_len,
        command_count,
        command_offsets,
        bbox,
        source_width,
        source_height,
    })
}

pub(crate) fn fdm_vector_command_source_segment_for_vector_offset(
    segments: &[ObjectFdmVectorSegmentCandidate],
    vector_offset: usize,
) -> Option<ObjectFdmVectorCommandSourceSegment> {
    segments.iter().find_map(|segment| {
        let segment_start = segment.relative_offset();
        let segment_end = segment_start.saturating_add(usize::from(segment.declared_len()));
        if vector_offset < segment_start || vector_offset >= segment_end {
            return None;
        }
        let header = FdmVectorSegmentHeader {
            declared_len: segment.declared_len(),
            command_count: segment.command_count(),
            command_offsets: segment.command_offsets().to_vec(),
            bbox: segment.bbox(),
            source_width: segment.source_width(),
            source_height: segment.source_height(),
        };
        fdm_vector_command_source_segment_for_local_offset(
            segment_start,
            &header,
            vector_offset.saturating_sub(segment_start),
        )
    })
}

pub(crate) fn fdm_vector_command_source_segment_for_local_offset(
    segment_relative_offset: usize,
    header: &FdmVectorSegmentHeader,
    local_offset: usize,
) -> Option<ObjectFdmVectorCommandSourceSegment> {
    let declared_len = usize::from(header.declared_len);
    if local_offset >= declared_len {
        return None;
    }
    header.command_offsets.iter().copied().enumerate().find_map(
        |(command_index, command_offset)| {
            let command_offset_usize = usize::from(command_offset);
            let next_offset = header
                .command_offsets
                .get(command_index + 1)
                .copied()
                .map(usize::from)
                .unwrap_or(declared_len);
            (command_offset_usize <= local_offset && local_offset < next_offset).then(|| {
                ObjectFdmVectorCommandSourceSegment::new(
                    segment_relative_offset,
                    local_offset,
                    header,
                    command_index,
                    command_offset,
                )
            })
        },
    )
}

pub(crate) fn fdm_raw_vector_command_candidates(
    vector_stream: &[u8],
) -> Vec<ObjectFdmVectorCommandCandidate> {
    let segments = fdm_raw_vector_segment_candidates(vector_stream);
    let mut commands = Vec::new();
    let mut offset = 0usize;
    while offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2 <= vector_stream.len() {
        let marker = [
            vector_stream[offset],
            vector_stream[offset + 1],
            vector_stream[offset + 2],
            vector_stream[offset + 3],
        ];
        if !FDM_VECTOR_RENDERED_PRIMITIVE_MARKERS.contains(&marker) {
            offset += 1;
            continue;
        }
        let Some(record_len) = read_be16_at(
            vector_stream,
            offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET,
        )
        .map(usize::from) else {
            offset += 1;
            continue;
        };
        if record_len < FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2
            || offset + record_len > vector_stream.len()
        {
            offset += 1;
            continue;
        }
        let record = &vector_stream[offset..offset + record_len];
        if let Some(command) = ObjectFdmVectorCommandCandidate::new(
            commands.len(),
            offset,
            record,
            offset + record_len,
            None,
        ) && command.has_renderable_geometry()
        {
            let command = command.with_source_vector_relative_offset(offset);
            let command = if let Some(source_segment) =
                fdm_vector_command_source_segment_for_vector_offset(&segments, offset)
            {
                command.with_source_segment(source_segment)
            } else {
                command
            };
            commands.push(command);
        }
        offset += record_len.max(1);
    }
    commands
}

pub(crate) fn fdm_vector_command_candidates(
    segment: &[u8],
    segment_relative_offset: usize,
) -> Vec<ObjectFdmVectorCommandCandidate> {
    let Some(header) = fdm_vector_segment_header(segment) else {
        return Vec::new();
    };
    let segment_len = usize::from(header.declared_len);
    let offsets = header
        .command_offsets
        .iter()
        .map(|offset| usize::from(*offset))
        .collect::<Vec<_>>();

    let mut commands = Vec::new();
    for (command_index, relative_offset) in offsets.iter().enumerate() {
        let next_offset = offsets
            .get(command_index + 1)
            .copied()
            .unwrap_or(segment_len);
        if next_offset <= *relative_offset || next_offset > segment_len {
            continue;
        }
        let Some(record) = segment.get(*relative_offset..next_offset) else {
            continue;
        };
        let Some(command) = ObjectFdmVectorCommandCandidate::new(
            command_index,
            *relative_offset,
            record,
            next_offset,
            None,
        ) else {
            continue;
        };
        let command =
            command.with_source_vector_relative_offset(segment_relative_offset + *relative_offset);
        let command = if let Some(source_segment) =
            fdm_vector_command_source_segment_for_local_offset(
                segment_relative_offset,
                &header,
                *relative_offset,
            ) {
            command.with_source_segment(source_segment)
        } else {
            command
        };
        commands.push(command);
        commands.extend(fdm_vector_nested_primitive_command_candidates(
            command_index,
            *relative_offset,
            segment_relative_offset,
            &header,
            record,
        ));
    }
    commands
}

pub(crate) fn fdm_vector_nested_primitive_command_candidates(
    parent_command_index: usize,
    parent_relative_offset: usize,
    segment_relative_offset: usize,
    header: &FdmVectorSegmentHeader,
    record: &[u8],
) -> Vec<ObjectFdmVectorCommandCandidate> {
    if !record.starts_with(FDM_VECTOR_COMMAND_BBOX_MARKER) {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    let mut scan_offset = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16;
    let style_context = fdm_vector_compound_style_context(record);
    let table_offsets = fdm_vector_compound_child_offsets(record);
    for (nested_index, nested_offset) in table_offsets.iter().copied().enumerate() {
        let nested_offset = usize::from(nested_offset);
        if let Some(candidate) = fdm_vector_nested_primitive_command_candidate_at(
            parent_command_index,
            nested_index,
            parent_relative_offset,
            segment_relative_offset,
            header,
            record,
            nested_offset,
            style_context,
        ) {
            candidates.push(candidate);
        }
    }
    if !candidates.is_empty() {
        return candidates;
    }

    let mut nested_index = 0usize;
    while scan_offset + 8 <= record.len() {
        let Some((nested_offset, _marker)) =
            find_fdm_vector_nested_primitive_marker(record, scan_offset)
        else {
            break;
        };
        if let Some(candidate) = fdm_vector_nested_primitive_command_candidate_at(
            parent_command_index,
            nested_index,
            parent_relative_offset,
            segment_relative_offset,
            header,
            record,
            nested_offset,
            style_context,
        ) {
            scan_offset = nested_offset + candidate.declared_record_len() as usize;
            candidates.push(candidate);
        } else {
            scan_offset = nested_offset + 1;
        };
        nested_index += 1;
    }
    candidates
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn fdm_vector_nested_primitive_command_candidate_at(
    parent_command_index: usize,
    nested_index: usize,
    parent_relative_offset: usize,
    segment_relative_offset: usize,
    header: &FdmVectorSegmentHeader,
    record: &[u8],
    nested_offset: usize,
    style_context: Option<FdmVectorStyleContext>,
) -> Option<ObjectFdmVectorCommandCandidate> {
    if nested_offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2 > record.len()
        || !FDM_VECTOR_NESTED_PRIMITIVE_MARKERS
            .iter()
            .any(|marker| record[nested_offset..].starts_with(marker))
    {
        return None;
    }
    let nested_len = read_be16_at(
        record,
        nested_offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET,
    )
    .map(usize::from)?;
    if nested_len < FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2
        || nested_offset + nested_len > record.len()
    {
        return None;
    }

    let child_relative_offset = parent_relative_offset + nested_offset;
    let child_next_offset = child_relative_offset + nested_len;
    let synthetic_command_index = parent_command_index * 1000 + nested_index + 1;
    ObjectFdmVectorCommandCandidate::new(
        synthetic_command_index,
        child_relative_offset,
        &record[nested_offset..nested_offset + nested_len],
        child_next_offset,
        style_context,
    )
    .map(|command| {
        let command = command
            .with_source_vector_relative_offset(segment_relative_offset + child_relative_offset);
        if let Some(source_segment) = fdm_vector_command_source_segment_for_local_offset(
            segment_relative_offset,
            header,
            child_relative_offset,
        ) {
            command.with_source_segment(source_segment)
        } else {
            command
        }
    })
    .filter(ObjectFdmVectorCommandCandidate::has_renderable_geometry)
}

pub(crate) fn fdm_vector_compound_style_context(record: &[u8]) -> Option<FdmVectorStyleContext> {
    if !record.starts_with(FDM_VECTOR_COMMAND_BBOX_MARKER) {
        return None;
    }

    let prefix_start = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16;
    let (first_child_offset, _) = find_fdm_vector_nested_primitive_marker(record, prefix_start)?;
    if first_child_offset <= prefix_start {
        return None;
    }
    let prefix = record.get(prefix_start..first_child_offset)?;
    let fill_color = fdm_vector_prefix_color(prefix, 0);
    let stroke_color = fdm_vector_prefix_color(prefix, 4);
    let gradient_colors = fdm_vector_compound_gradient_context(record, fill_color, stroke_color);
    if fill_color.is_none() && stroke_color.is_none() && gradient_colors.is_none() {
        None
    } else {
        Some(FdmVectorStyleContext {
            fill_color,
            stroke_color,
            gradient_colors,
        })
    }
}

pub(crate) fn fdm_vector_compound_gradient_context(
    record: &[u8],
    fill_color: Option<u32>,
    stroke_color: Option<u32>,
) -> Option<FdmVectorGradientContext> {
    if read_be16_at(record, 6)? != 0x0001 {
        return None;
    }
    let fill_color = fill_color?;
    let stroke_color = stroke_color?;
    if fill_color == stroke_color {
        return None;
    }
    if fdm_vector_compound_child_offsets(record).len() != 1 {
        return None;
    }
    Some(FdmVectorGradientContext::new(stroke_color, fill_color))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FdmCompoundChildLayout {
    pub(crate) child_offsets: Vec<u16>,
    pub(crate) first_child_matches_prefix_end: bool,
    pub(crate) child_offsets_strictly_increasing: bool,
    pub(crate) child_records_fit_parent: bool,
    pub(crate) child_records_do_not_overlap: bool,
}

impl FdmCompoundChildLayout {
    pub(crate) fn child_offsets(&self) -> &[u16] {
        &self.child_offsets
    }

    pub(crate) fn first_child_matches_prefix_end(&self) -> bool {
        self.first_child_matches_prefix_end
    }

    pub(crate) fn child_offsets_strictly_increasing(&self) -> bool {
        self.child_offsets_strictly_increasing
    }

    pub(crate) fn child_records_fit_parent(&self) -> bool {
        self.child_records_fit_parent
    }

    pub(crate) fn child_records_do_not_overlap(&self) -> bool {
        self.child_records_do_not_overlap
    }

    pub(crate) fn is_valid_for_nested_projection(&self) -> bool {
        self.first_child_matches_prefix_end
            && self.child_offsets_strictly_increasing
            && self.child_records_fit_parent
            && self.child_records_do_not_overlap
    }
}

pub(crate) fn fdm_vector_compound_child_layout(record: &[u8]) -> Option<FdmCompoundChildLayout> {
    let prefix = fdm_vector_compound_prefix(record)?;
    if prefix.len() < 10 || prefix.len() % 2 != 0 {
        return None;
    }
    let child_offsets = prefix[8..]
        .chunks_exact(2)
        .filter_map(|chunk| read_be16_at(chunk, 0))
        .collect::<Vec<_>>();
    if child_offsets.is_empty() {
        return None;
    }
    let first_child_offset = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16 + prefix.len();
    let first_child_matches_prefix_end = child_offsets
        .first()
        .is_some_and(|offset| usize::from(*offset) == first_child_offset);
    let child_offsets_strictly_increasing = child_offsets.windows(2).all(|pair| pair[0] < pair[1]);
    let child_records = child_offsets
        .iter()
        .map(|offset| {
            let offset = usize::from(*offset);
            let marker_valid = offset < record.len()
                && FDM_VECTOR_NESTED_PRIMITIVE_MARKERS
                    .iter()
                    .any(|marker| record[offset..].starts_with(marker));
            let declared_end = marker_valid
                .then(|| {
                    read_be16_at(record, offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET)
                        .map(usize::from)
                        .and_then(|length| {
                            (length >= FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2)
                                .then_some(offset.saturating_add(length))
                        })
                })
                .flatten();
            (offset, declared_end)
        })
        .collect::<Vec<_>>();
    let child_records_fit_parent = child_records
        .iter()
        .all(|(_, end)| end.is_some_and(|end| end <= record.len()));
    let child_records_do_not_overlap = child_records
        .windows(2)
        .all(|pair| pair[0].1.is_some_and(|first_end| first_end <= pair[1].0));

    Some(FdmCompoundChildLayout {
        child_offsets,
        first_child_matches_prefix_end,
        child_offsets_strictly_increasing,
        child_records_fit_parent,
        child_records_do_not_overlap,
    })
}

pub(crate) fn fdm_vector_compound_child_offsets(record: &[u8]) -> Vec<u16> {
    let Some(prefix) = fdm_vector_compound_prefix(record) else {
        return Vec::new();
    };
    if prefix.len() < 10 || prefix.len() % 2 != 0 {
        return Vec::new();
    }
    let offsets = prefix[8..]
        .chunks_exact(2)
        .filter_map(|chunk| read_be16_at(chunk, 0))
        .collect::<Vec<_>>();
    if offsets.is_empty() {
        return Vec::new();
    }
    let first_child_offset = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16 + prefix.len();
    if offsets
        .first()
        .is_some_and(|offset| usize::from(*offset) == first_child_offset)
        && offsets.iter().all(|offset| {
            usize::from(*offset) >= first_child_offset && usize::from(*offset) <= record.len()
        })
    {
        offsets
            .into_iter()
            .filter(|offset| {
                let offset = usize::from(*offset);
                offset < record.len()
                    && FDM_VECTOR_NESTED_PRIMITIVE_MARKERS
                        .iter()
                        .any(|marker| record[offset..].starts_with(marker))
            })
            .collect()
    } else {
        Vec::new()
    }
}

pub(crate) fn fdm_vector_compound_prefix(record: &[u8]) -> Option<&[u8]> {
    if !record.starts_with(FDM_VECTOR_COMMAND_BBOX_MARKER) {
        return None;
    }
    let prefix_start = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16;
    let (first_child_offset, _) = find_fdm_vector_nested_primitive_marker(record, prefix_start)?;
    if first_child_offset <= prefix_start {
        return None;
    }
    record.get(prefix_start..first_child_offset)
}

pub(crate) fn fdm_vector_prefix_color(prefix: &[u8], offset: usize) -> Option<u32> {
    let color = read_be32_at(prefix, offset)?;
    if color > 0x00ff_ffff {
        return None;
    }
    if color == 0
        || color == 0x00ff_ffff
        || color >= 0x0001_0000
        || fdm_vector_is_grayscale_color(color)
    {
        Some(color)
    } else {
        None
    }
}

pub(crate) fn fdm_vector_is_grayscale_color(color: u32) -> bool {
    let red = (color >> 16) & 0xff;
    let green = (color >> 8) & 0xff;
    let blue = color & 0xff;
    red == green && green == blue
}

pub(crate) fn find_fdm_vector_nested_primitive_marker(
    record: &[u8],
    start_offset: usize,
) -> Option<(usize, [u8; 4])> {
    let mut best: Option<(usize, [u8; 4])> = None;
    for marker in FDM_VECTOR_NESTED_PRIMITIVE_MARKERS {
        let Some(position) = find_subslice_offsets(&record[start_offset..], &marker)
            .into_iter()
            .next()
        else {
            continue;
        };
        let offset = start_offset + position;
        if best.is_none_or(|(best_offset, _)| offset < best_offset) {
            best = Some((offset, marker));
        }
    }
    best
}

pub(crate) fn fdm_vector_command_bbox(record: &[u8]) -> Option<ObjectFdmIndexBbox> {
    if !record.starts_with(FDM_VECTOR_COMMAND_BBOX_MARKER)
        || record.len() < FDM_VECTOR_COMMAND_BBOX_OFFSET + 16
    {
        return None;
    }
    let left = read_i32_be_at(record, FDM_VECTOR_COMMAND_BBOX_OFFSET)?;
    let top = read_i32_be_at(record, FDM_VECTOR_COMMAND_BBOX_OFFSET + 4)?;
    let right = read_i32_be_at(record, FDM_VECTOR_COMMAND_BBOX_OFFSET + 8)?;
    let bottom = read_i32_be_at(record, FDM_VECTOR_COMMAND_BBOX_OFFSET + 12)?;
    Some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

pub(crate) fn fdm_vector_command_ellipse(
    record: &[u8],
    marker: [u8; 4],
) -> Option<ObjectFdmVectorEllipse> {
    if !FDM_VECTOR_COMMAND_ELLIPSE_MARKERS.contains(&marker)
        || record.len() < FDM_VECTOR_COMMAND_ELLIPSE_RADIUS_OFFSET + 4
    {
        return None;
    }

    let center_x = read_i32_be_at(record, FDM_VECTOR_COMMAND_ELLIPSE_CENTER_OFFSET)?;
    let center_y = read_i32_be_at(record, FDM_VECTOR_COMMAND_ELLIPSE_CENTER_OFFSET + 4)?;
    let radius_x = read_be16_at(record, FDM_VECTOR_COMMAND_ELLIPSE_RADIUS_OFFSET).map(i32::from)?;
    let radius_y =
        read_be16_at(record, FDM_VECTOR_COMMAND_ELLIPSE_RADIUS_OFFSET + 2).map(i32::from)?;
    if radius_x <= 0 || radius_y <= 0 {
        return None;
    }
    let color = read_be32_at(record, FDM_VECTOR_COMMAND_ELLIPSE_COLOR_OFFSET);
    Some(ObjectFdmVectorEllipse::new(
        ObjectFdmVectorPoint::new(center_x, center_y),
        radius_x,
        radius_y,
        color,
    ))
}

pub(crate) fn fdm_vector_command_curve_segments(
    record: &[u8],
    marker: [u8; 4],
    points: &[ObjectFdmVectorPoint],
) -> Vec<ObjectFdmVectorCurveSegment> {
    if !fdm_vector_marker_is_bezier_curve(&marker) || points.len() < 2 {
        return Vec::new();
    }

    let controls_start = FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET + points.len() * 8;
    let segment_count = points.len().saturating_sub(1);
    let mut segments = Vec::with_capacity(segment_count);
    for segment_index in 0..segment_count {
        let offset = controls_start + segment_index * 16;
        if offset + 16 > record.len() {
            break;
        }
        let Some(control_1_dx) = read_i32_be_at(record, offset) else {
            break;
        };
        let Some(control_1_dy) = read_i32_be_at(record, offset + 4) else {
            break;
        };
        let Some(control_2_dx) = read_i32_be_at(record, offset + 8) else {
            break;
        };
        let Some(control_2_dy) = read_i32_be_at(record, offset + 12) else {
            break;
        };
        let control_1 = points[segment_index].offset(control_1_dx, control_1_dy);
        let control_2 = points[segment_index + 1].offset(control_2_dx, control_2_dy);
        segments.push(ObjectFdmVectorCurveSegment::new(control_1, control_2));
    }
    segments
}

pub(crate) fn fdm_vector_command_path_points(
    record: &[u8],
    marker: [u8; 4],
) -> Vec<ObjectFdmVectorPoint> {
    if fdm_vector_marker_is_line(&marker) {
        if record.len() < FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET + 16 {
            return Vec::new();
        }
        let Some(x1) = read_i32_be_at(record, FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET) else {
            return Vec::new();
        };
        let Some(y1) = read_i32_be_at(record, FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET + 4) else {
            return Vec::new();
        };
        let Some(x2) = read_i32_be_at(record, FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET + 8) else {
            return Vec::new();
        };
        let Some(y2) = read_i32_be_at(record, FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET + 12) else {
            return Vec::new();
        };
        if x1 == x2 && y1 == y2 {
            return Vec::new();
        }
        return vec![
            ObjectFdmVectorPoint::new(x1, y1),
            ObjectFdmVectorPoint::new(x2, y2),
        ];
    }

    if !FDM_VECTOR_COMMAND_PATH_MARKERS.contains(&marker)
        || record.len() < FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET
    {
        return Vec::new();
    }
    let Some(point_count) =
        read_be16_at(record, FDM_VECTOR_COMMAND_PATH_POINT_COUNT_OFFSET).map(usize::from)
    else {
        return Vec::new();
    };
    let points_end = FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET + point_count * 8;
    if point_count < 2 || points_end > record.len() {
        return Vec::new();
    }

    let mut points = Vec::with_capacity(point_count);
    for index in 0..point_count {
        let offset = FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET + index * 8;
        let Some(x) = read_i32_be_at(record, offset) else {
            return Vec::new();
        };
        let Some(y) = read_i32_be_at(record, offset + 4) else {
            return Vec::new();
        };
        points.push(ObjectFdmVectorPoint::new(x, y));
    }
    points
}

pub(crate) fn fdm_vector_path_points_bbox(
    points: &[ObjectFdmVectorPoint],
) -> Option<ObjectFdmIndexBbox> {
    let mut iter = points.iter();
    let first = *iter.next()?;
    let mut left = first.x();
    let mut top = first.y();
    let mut right = first.x();
    let mut bottom = first.y();
    for point in iter {
        left = left.min(point.x());
        top = top.min(point.y());
        right = right.max(point.x());
        bottom = bottom.max(point.y());
    }
    Some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

pub(crate) fn fdm_vector_ellipse_bbox(ellipse: ObjectFdmVectorEllipse) -> ObjectFdmIndexBbox {
    let center = ellipse.center();
    ObjectFdmIndexBbox::new(
        center.x().saturating_sub(ellipse.radius_x()),
        center.y().saturating_sub(ellipse.radius_y()),
        center.x().saturating_add(ellipse.radius_x()),
        center.y().saturating_add(ellipse.radius_y()),
    )
}

pub(crate) fn fdm_vector_command_source_bbox(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmIndexBbox> {
    if !command.path_points().is_empty() {
        let mut points =
            Vec::with_capacity(command.path_points().len() + command.curve_segments().len() * 2);
        points.extend_from_slice(command.path_points());
        for segment in command.curve_segments() {
            points.push(segment.control_1());
            points.push(segment.control_2());
        }
        let bbox = fdm_vector_path_points_bbox(&points)?;
        return Some(bbox);
    }
    command.ellipse().map(fdm_vector_ellipse_bbox)
}

pub(crate) fn fdm_connector_candidates(
    commands: &[ObjectFdmVectorCommandCandidate],
) -> Vec<ObjectFdmConnectorCandidate> {
    commands
        .iter()
        .filter_map(fdm_connector_candidate_from_command)
        .collect()
}

pub(crate) fn fdm_connector_candidate_from_command(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmConnectorCandidate> {
    if command.ellipse().is_some() || fdm_vector_path_is_closed(command.path_points()) {
        return None;
    }
    let source_start = *command.path_points().first()?;
    let source_end = *command.path_points().last()?;
    let source_bbox = fdm_vector_command_source_bbox(command)?;
    let normalized = normalize_fdm_bbox(source_bbox);
    let source_width = normalized.2.saturating_sub(normalized.0);
    let source_height = normalized.3.saturating_sub(normalized.1);
    let source_span = source_width.max(source_height);
    if source_span < FDM_CONNECTOR_CANDIDATE_MIN_SOURCE_SPAN_UNITS {
        return None;
    }
    let endpoint_dx = source_end.x().saturating_sub(source_start.x());
    let endpoint_dy = source_end.y().saturating_sub(source_start.y());
    let dx = i64::from(endpoint_dx);
    let dy = i64::from(endpoint_dy);
    let endpoint_distance_squared = (dx.saturating_mul(dx) + dy.saturating_mul(dy)) as u64;
    let (path_segment_count, orthogonal_segment_count, diagonal_segment_count) =
        fdm_connector_path_segment_counts(command.path_points());

    Some(ObjectFdmConnectorCandidate {
        command_index: command.command_index(),
        relative_offset: command.relative_offset(),
        marker: *command.marker(),
        style_word: command.style_word(),
        primitive_kind: fdm_vector_primitive_kind(command),
        fill_color: command.fill_color(),
        stroke_color: command.stroke_color(),
        source_start,
        source_end,
        source_bbox,
        source_span,
        endpoint_dx,
        endpoint_dy,
        endpoint_distance_squared,
        path_point_count: command.path_points().len(),
        path_segment_count,
        orthogonal_segment_count,
        diagonal_segment_count,
        curve_segment_count: command.curve_segments().len(),
        compound_child_offset_count: command.compound_child_offsets().len(),
        axis_aligned: endpoint_dx == 0 || endpoint_dy == 0,
        orientation: fdm_connector_orientation(endpoint_dx as f32, endpoint_dy as f32),
        basis: "long-open-source-path",
    })
}

pub(crate) fn fdm_connector_path_segment_counts(
    points: &[ObjectFdmVectorPoint],
) -> (usize, usize, usize) {
    let mut path_segment_count = 0usize;
    let mut orthogonal_segment_count = 0usize;
    let mut diagonal_segment_count = 0usize;
    for pair in points.windows(2) {
        let start = pair[0];
        let end = pair[1];
        path_segment_count += 1;
        if start.x() == end.x() || start.y() == end.y() {
            orthogonal_segment_count += 1;
        } else {
            diagonal_segment_count += 1;
        }
    }
    (
        path_segment_count,
        orthogonal_segment_count,
        diagonal_segment_count,
    )
}

pub(crate) fn fdm_vector_path_is_closed(points: &[ObjectFdmVectorPoint]) -> bool {
    points.len() >= 3 && points.first() == points.last()
}

pub(crate) fn fdm_vector_primitive_is_closed(command: &ObjectFdmVectorCommandCandidate) -> bool {
    command.ellipse().is_some() || fdm_vector_path_is_closed(command.path_points())
}

pub(crate) fn fdm_vector_marker_is_bezier_curve(marker: &[u8; 4]) -> bool {
    marker == b"\xff\x00\x09\x60" || marker == b"\x00\x00\x09\x60" || marker == b"\x01\x00\x09\x60"
}

pub(crate) fn fdm_vector_marker_is_line(marker: &[u8; 4]) -> bool {
    marker == FDM_VECTOR_COMMAND_LINE_MARKER
        || marker == FDM_VECTOR_COMMAND_NESTED_LINE_MARKER
        || marker == FDM_VECTOR_COMMAND_INDEXED_LINE_MARKER
}

pub(crate) fn fdm_vector_primitive_kind(command: &ObjectFdmVectorCommandCandidate) -> &'static str {
    if command.ellipse().is_some() {
        "ellipse"
    } else if !command.curve_segments().is_empty() {
        "cubicBezier"
    } else if fdm_vector_marker_is_bezier_curve(command.marker()) {
        "quadraticBezier"
    } else {
        "polyline"
    }
}

pub(crate) fn fdm_vector_stroke_width(command: &ObjectFdmVectorCommandCandidate) -> f32 {
    if command.ellipse().is_some() {
        return if command.style_word() == 0x0010 {
            2.250
        } else {
            0.720
        };
    }
    if fdm_vector_marker_is_bezier_curve(command.marker()) && command.style_word() == 0x0010 {
        return 2.250;
    }
    if fdm_vector_path_is_closed(command.path_points()) && command.fill_color().is_some() {
        return 0.139;
    }
    if fdm_vector_marker_is_line(command.marker()) {
        return 0.500;
    }
    match command.style_word() & 0x000f {
        0x0004 => 0.410,
        0x0005 => 0.480,
        0x0008 => 0.410,
        _ => 0.500,
    }
}

pub(crate) fn fdm_vector_render_stroke_color(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> String {
    diagnostic
        .command
        .stroke_color()
        .and_then(fdm_vector_css_color)
        .unwrap_or_else(|| {
            if fdm_vector_uncolored_path_uses_light_stroke(diagnostic, diagnostics) {
                "#ffffff".to_string()
            } else {
                "#111111".to_string()
            }
        })
}

pub(crate) fn fdm_vector_render_fill_color(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> String {
    if !fdm_vector_primitive_is_closed(diagnostic.command) {
        return "none".to_string();
    }
    let Some(fill_color) = diagnostic.command.fill_color() else {
        return "none".to_string();
    };
    if fdm_vector_filled_path_is_text_mask_outer(diagnostic, diagnostics) {
        return "#ffffff".to_string();
    }
    if fdm_vector_filled_path_is_text_mask_inner(diagnostic, diagnostics) {
        return "#000000".to_string();
    }
    if fdm_vector_filled_path_is_compound_hole(diagnostic, diagnostics) {
        return fdm_vector_containing_fill_color(diagnostic, diagnostics)
            .and_then(fdm_vector_css_color)
            .unwrap_or_else(|| "#111111".to_string());
    }
    fdm_vector_css_color(fill_color).unwrap_or_else(|| "none".to_string())
}

pub(crate) fn fdm_vector_linear_gradient_colors(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<(String, String)> {
    if !fdm_vector_primitive_is_closed(command) {
        return None;
    }
    let gradient = command.gradient_colors()?;
    let from = fdm_vector_css_color(gradient.start_color())?;
    let to = fdm_vector_css_color(gradient.end_color())?;
    (from != to).then_some((from, to))
}

pub(crate) fn fdm_vector_filled_path_is_text_mask_outer(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    let command = diagnostic.command;
    if !fdm_vector_text_mask_candidate(command)
        || !command.fill_color().is_some_and(fdm_vector_color_is_black)
    {
        return false;
    }
    let Some(outer_bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let outer_area = fdm_bbox_area(outer_bbox);
    if outer_area == 0 {
        return false;
    }

    diagnostics.iter().any(|other| {
        if other.candidate_index != diagnostic.candidate_index
            || other.entry.row_index() != diagnostic.entry.row_index()
            || other.command.command_index() == command.command_index()
            || !fdm_vector_text_mask_candidate(other.command)
            || !other
                .command
                .fill_color()
                .is_some_and(fdm_vector_color_is_white)
        {
            return false;
        }
        let Some(inner_bbox) =
            fdm_vector_command_source_bbox(other.command).map(normalize_fdm_bbox)
        else {
            return false;
        };
        fdm_vector_text_mask_area_ratio(outer_bbox, inner_bbox).is_some_and(|ratio| {
            fdm_bbox_contains(outer_bbox, inner_bbox)
                && ratio >= FDM_VECTOR_TEXT_MASK_MIN_INNER_AREA_RATIO
        })
    })
}

pub(crate) fn fdm_vector_filled_path_is_text_mask_inner(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    let command = diagnostic.command;
    if !fdm_vector_text_mask_candidate(command)
        || !command.fill_color().is_some_and(fdm_vector_color_is_white)
    {
        return false;
    }
    let Some(inner_bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };

    diagnostics.iter().any(|other| {
        if other.candidate_index != diagnostic.candidate_index
            || other.entry.row_index() != diagnostic.entry.row_index()
            || other.command.command_index() == command.command_index()
            || !fdm_vector_text_mask_candidate(other.command)
            || !other
                .command
                .fill_color()
                .is_some_and(fdm_vector_color_is_black)
        {
            return false;
        }
        let Some(outer_bbox) =
            fdm_vector_command_source_bbox(other.command).map(normalize_fdm_bbox)
        else {
            return false;
        };
        fdm_vector_text_mask_area_ratio(outer_bbox, inner_bbox).is_some_and(|ratio| {
            fdm_bbox_contains(outer_bbox, inner_bbox)
                && ratio >= FDM_VECTOR_TEXT_MASK_MIN_INNER_AREA_RATIO
        })
    })
}

pub(crate) fn fdm_vector_text_mask_candidate(command: &ObjectFdmVectorCommandCandidate) -> bool {
    command.marker() == b"\x00\x00\x06\x60"
        && command.style_word() == 0x0008
        && command.stroke_color().is_none()
        && command.ellipse().is_none()
        && command.fill_color().is_some()
        && fdm_vector_path_is_closed(command.path_points())
}

pub(crate) fn fdm_vector_text_mask_area_ratio(
    outer_bbox: (i32, i32, i32, i32),
    inner_bbox: (i32, i32, i32, i32),
) -> Option<f64> {
    let outer_area = fdm_bbox_area(outer_bbox);
    let inner_area = fdm_bbox_area(inner_bbox);
    if outer_area == 0 || inner_area == 0 {
        return None;
    }
    let ratio = inner_area as f64 / outer_area as f64;
    (FDM_VECTOR_TEXT_MASK_MIN_INNER_AREA_RATIO..=FDM_VECTOR_TEXT_MASK_MAX_INNER_AREA_RATIO)
        .contains(&ratio)
        .then_some(ratio)
}

pub(crate) fn fdm_vector_filled_path_is_counter_overlay(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    fdm_vector_filled_path_is_compound_hole(diagnostic, diagnostics)
        || fdm_vector_filled_path_is_text_mask_inner(diagnostic, diagnostics)
}

pub(crate) fn fdm_vector_filled_path_is_compound_hole(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    let command = diagnostic.command;
    if command.ellipse().is_some()
        || command.command_index() < 1000
        || command.fill_color().is_none()
        || !fdm_vector_path_is_closed(command.path_points())
    {
        return false;
    }
    let Some(inner_bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let inner_area = fdm_bbox_area(inner_bbox);
    if inner_area == 0 {
        return false;
    }
    let parent_command_index = command.command_index() / 1000;
    diagnostics.iter().any(|other| {
        other.candidate_index == diagnostic.candidate_index
            && other.entry.row_index() == diagnostic.entry.row_index()
            && other.command.command_index() / 1000 == parent_command_index
            && other.command.command_index() != command.command_index()
            && other.command.command_index() >= 1000
            && other.command.ellipse().is_none()
            && other.command.fill_color() == command.fill_color()
            && fdm_vector_path_is_closed(other.command.path_points())
            && fdm_vector_command_source_bbox(other.command)
                .map(normalize_fdm_bbox)
                .is_some_and(|outer_bbox| {
                    fdm_bbox_contains(outer_bbox, inner_bbox)
                        && fdm_bbox_area(outer_bbox) > inner_area
                })
    })
}

pub(crate) fn fdm_vector_containing_fill_color(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> Option<u32> {
    let inner_bbox = fdm_vector_command_source_bbox(diagnostic.command).map(normalize_fdm_bbox)?;
    diagnostics
        .iter()
        .filter(|other| {
            other.candidate_index == diagnostic.candidate_index
                && other.entry.row_index() == diagnostic.entry.row_index()
                && other.command.command_index() != diagnostic.command.command_index()
                && other.command.fill_color() != diagnostic.command.fill_color()
                && other.command.fill_color().is_some()
        })
        .filter_map(|other| {
            let outer_bbox =
                fdm_vector_command_source_bbox(other.command).map(normalize_fdm_bbox)?;
            fdm_bbox_contains(outer_bbox, inner_bbox)
                .then_some((fdm_bbox_area(outer_bbox), other.command.fill_color()?))
        })
        .min_by_key(|(area, _)| *area)
        .map(|(_, color)| color)
}

pub(crate) fn fdm_vector_uncolored_path_uses_light_stroke(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    let command = diagnostic.command;
    if command.marker() != b"\xff\x00\x06\x60"
        || command.style_word() != 0x0004
        || command.fill_color().is_some()
        || command.stroke_color().is_some()
        || !fdm_vector_path_is_closed(command.path_points())
    {
        return false;
    }
    let Some(inner_bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };

    diagnostics.iter().any(|other| {
        other.candidate_index == diagnostic.candidate_index
            && other.entry.row_index() == diagnostic.entry.row_index()
            && other
                .command
                .fill_color()
                .is_some_and(fdm_vector_color_is_black)
            && fdm_vector_command_source_bbox(other.command)
                .map(normalize_fdm_bbox)
                .is_some_and(|outer_bbox| fdm_bbox_contains(outer_bbox, inner_bbox))
    })
}

pub(crate) fn fdm_vector_color_is_black(color: u32) -> bool {
    color & 0x00ff_ffff == 0
}

pub(crate) fn fdm_vector_color_is_white(color: u32) -> bool {
    color & 0x00ff_ffff == 0x00ff_ffff
}

pub(crate) fn fdm_bbox_contains(outer: (i32, i32, i32, i32), inner: (i32, i32, i32, i32)) -> bool {
    outer.0 <= inner.0 && outer.1 <= inner.1 && outer.2 >= inner.2 && outer.3 >= inner.3
}

pub(crate) fn fdm_bbox_intersects(left: (i32, i32, i32, i32), right: (i32, i32, i32, i32)) -> bool {
    left.0 < right.2 && right.0 < left.2 && left.1 < right.3 && right.1 < left.3
}

pub(crate) fn fdm_bbox_area(bbox: (i32, i32, i32, i32)) -> i64 {
    let width = i64::from(bbox.2.saturating_sub(bbox.0).max(0));
    let height = i64::from(bbox.3.saturating_sub(bbox.1).max(0));
    width.saturating_mul(height)
}
