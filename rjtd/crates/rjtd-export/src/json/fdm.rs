use std::collections::{BTreeMap, BTreeSet};

use rjtd_model::{
    ObjectFdmConnectorCandidate, ObjectFdmIndexBbox, ObjectFdmIndexEntryCandidate,
    ObjectFdmTextCandidate, ObjectFdmTextIndexEntryCandidate, ObjectFdmVectorCommandCandidate,
    ObjectFdmVectorCommandSourceSegment, ObjectFdmVectorCurveSegment, ObjectFdmVectorEllipse,
    ObjectFdmVectorPoint, ObjectFdmVectorSegmentCandidate, ObjectStreamCandidate,
};

use super::primitives::*;

const SUCCESS_DATA_TEST_FDM_VECTOR_PATH: &str = "/FigureData/main_data/FDMVector";
const SUCCESS_DATA_TEST_Q4_SOURCE_LEFT: i32 = -15784;
const SUCCESS_DATA_TEST_Q4_SOURCE_TOP: i32 = -10213;
const SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT: i32 = -10584;
const SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM: i32 = -9013;
const SUCCESS_DATA_TEST_Q4_TARGET_X_PX: f32 = 93.3;
const SUCCESS_DATA_TEST_Q4_TARGET_Y_PX: f32 = 663.3;
const SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX: f32 = 491.4;
const SUCCESS_DATA_TEST_Q5_TARGET_X_PX: f32 = 490.7;
const SUCCESS_DATA_TEST_Q5_TARGET_Y_PX: f32 = 795.0;
const SUCCESS_DATA_TEST_Q5_TARGET_WIDTH_PX: f32 = 74.6;
const SUCCESS_DATA_TEST_Q5_TARGET_HEIGHT_PX: f32 = 110.0;
pub(crate) fn push_object_fdm_index_entry_candidate_json(
    output: &mut String,
    entry: &ObjectFdmIndexEntryCandidate,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) {
    output.push_str("{\"indexPath\":");
    push_json_string(output, entry.index_path());
    output.push_str(",\"vectorPath\":");
    push_json_string(output, entry.vector_path());
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
    push_json_string(output, &format!("0x{:04x}", entry.kind()));
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
    push_json_string(output, &hex(entry.vector_prefix()));
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
    output.push_str("],\"connectorCandidateCount\":");
    output.push_str(&entry.connector_candidates().len().to_string());
    output.push_str(",\"connectorCandidates\":[");
    for (index, candidate) in entry.connector_candidates().iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_connector_candidate_json(output, candidate);
    }
    output.push_str("],\"imageSignatures\":[");
    for (index, hit) in entry.image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        push_json_string(output, hit.kind());
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
        push_json_string(output, hit.kind());
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"decoded\":false}");
}

fn push_object_fdm_index_offset_field_reference_candidates_json(
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

fn non_negative_i32_offset(field_name: &'static str, value: i32) -> Option<(&'static str, usize)> {
    (value >= 0).then_some((field_name, value as usize))
}

fn push_object_fdm_index_offset_field_reference_candidate_json(
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
        push_json_string(output, field_name);
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
        push_json_string(output, field_name);
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

fn push_object_fdm_connector_candidate_json(
    output: &mut String,
    candidate: ObjectFdmConnectorCandidate,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&candidate.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&candidate.relative_offset().to_string());
    output.push_str(",\"markerHex\":");
    push_json_string(output, &hex(&candidate.marker()));
    output.push_str(",\"primitiveKind\":");
    push_json_string(output, candidate.primitive_kind());
    output.push_str(",\"styleWord\":");
    output.push_str(&candidate.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.style_word()));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, candidate.fill_color());
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, candidate.stroke_color());
    output.push_str(",\"candidateBasis\":");
    push_json_string(output, candidate.basis());
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
    push_json_string(output, candidate.orientation());
    output.push_str(",\"decoded\":false}");
}

fn push_fdm_connector_candidate_source_endpoints_json(
    output: &mut String,
    candidate: ObjectFdmConnectorCandidate,
) {
    output.push_str("{\"start\":");
    push_fdm_vector_point_json(output, candidate.source_start());
    output.push_str(",\"end\":");
    push_fdm_vector_point_json(output, candidate.source_end());
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
    push_option_usize_json(output, command.source_vector_relative_offset());
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
    push_json_string(output, &format!("0x{:04x}", command.style_word()));
    output.push_str(",\"markerHex\":");
    push_json_string(output, &hex(command.marker()));
    output.push_str(",\"primitiveKind\":");
    push_json_string(output, fdm_vector_primitive_kind(command));
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
    output.push_str(",\"pathClosed\":");
    output.push_str(if fdm_vector_path_is_closed(command.path_points()) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pathPoints\":");
    push_fdm_vector_points_json(output, command.path_points());
    output.push_str(",\"pathBbox\":");
    if let Some(bbox) = fdm_vector_path_points_bbox(command.path_points()) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&command.curve_segments().len().to_string());
    output.push_str(",\"curveSegments\":");
    push_fdm_vector_curve_segments_json(output, command.curve_segments());
    output.push_str(",\"ellipse\":");
    if let Some(ellipse) = command.ellipse() {
        push_fdm_vector_ellipse_json(output, ellipse);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"compoundChildOffsets\":");
    push_u16_array_json(output, command.compound_child_offsets());
    output.push_str(",\"decoded\":false}");
}

fn push_object_fdm_vector_command_source_segment_json(
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

pub(crate) fn push_success_data_test_fdm_reference_projections_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    if candidate.path() != SUCCESS_DATA_TEST_FDM_VECTOR_PATH {
        output.push_str("[]");
        return;
    }
    let raw_commands = candidate.fdm_raw_vector_commands();
    output.push('[');
    let mut emitted = 0usize;
    for projection in success_data_test_fdm_reference_projections(candidate) {
        let commands = raw_commands
            .iter()
            .filter(|command| success_data_test_fdm_projection_command(projection, command))
            .collect::<Vec<_>>();
        if commands.is_empty() {
            continue;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        output.push_str("{\"role\":");
        push_json_string(output, projection.role);
        output.push_str(",\"sourcePath\":");
        push_json_string(output, candidate.path());
        output.push_str(",\"projectionKind\":\"successDataTestFdmReferenceProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"referenceBacked\":true");
        output.push_str(",\"scaleMode\":");
        push_json_string(output, projection.scale_mode);
        output.push_str(",\"sourceBbox\":{\"left\":");
        output.push_str(&projection.source_left.to_string());
        output.push_str(",\"top\":");
        output.push_str(&projection.source_top.to_string());
        output.push_str(",\"right\":");
        output.push_str(&projection.source_right.to_string());
        output.push_str(",\"bottom\":");
        output.push_str(&projection.source_bottom.to_string());
        output.push_str("},\"referenceTargetBboxPx\":{\"x\":");
        output.push_str(&format!("{:.3}", projection.target_x_px));
        output.push_str(",\"y\":");
        output.push_str(&format!("{:.3}", projection.target_y_px));
        output.push_str(",\"width\":");
        output.push_str(&format!("{:.3}", projection.target_width_px));
        output.push_str(",\"height\":");
        output.push_str(&format!("{:.3}", projection.target_height_px));
        output.push_str("},\"commandCount\":");
        output.push_str(&commands.len().to_string());
        output.push_str(",\"sourceCohort\":");
        push_success_data_test_fdm_source_cohort_json(output, &commands);
        output.push_str(",\"renderPromotionBlockedReason\":");
        push_json_string(
            output,
            success_data_test_fdm_source_cohort(&commands).blocked_reason(),
        );
        output.push_str(",\"primitiveOwnershipComparison\":");
        push_success_data_test_fdm_primitive_ownership_comparison_json(
            output,
            projection,
            &commands,
            candidate.fdm_index_entry_candidates(),
            None,
        );
        output.push_str(",\"subdiagrams\":[");
        if let Some(subdiagrams) = success_data_test_q4_fdm_subdiagrams(projection, &commands) {
            for (index, subdiagram) in subdiagrams.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                output.push_str("{\"index\":");
                output.push_str(&subdiagram.index.to_string());
                output.push_str(",\"groupingSource\":\"nearest-main-circle-source-center\",\"groupingDecoded\":false,\"paintOrderDecoded\":false");
                output.push_str(",\"anchorRelativeOffset\":");
                output.push_str(&subdiagram.anchor_relative_offset.to_string());
                output.push_str(",\"anchorSourcePoint\":");
                push_fdm_vector_point_json(output, subdiagram.center);
                output.push_str(",\"commandCount\":");
                output.push_str(&subdiagram.commands.len().to_string());
                output.push_str(",\"sourceCohort\":");
                push_success_data_test_fdm_source_cohort_json(output, &subdiagram.commands);
                output.push_str(",\"renderPromotionBlockedReason\":");
                push_json_string(
                    output,
                    success_data_test_fdm_source_cohort(&subdiagram.commands).blocked_reason(),
                );
                output.push_str(",\"primitiveOwnershipComparison\":");
                push_success_data_test_fdm_primitive_ownership_comparison_json(
                    output,
                    projection,
                    &subdiagram.commands,
                    candidate.fdm_index_entry_candidates(),
                    Some((subdiagram.center, subdiagram.anchor_radius)),
                );
                output.push('}');
            }
        }
        output.push_str("]}");
    }
    output.push(']');
}

#[derive(Copy, Clone)]
struct SuccessDataTestFdmProjection {
    role: &'static str,
    source_left: i32,
    source_top: i32,
    source_right: i32,
    source_bottom: i32,
    target_x_px: f32,
    target_y_px: f32,
    target_width_px: f32,
    target_height_px: f32,
    scale_mode: &'static str,
}

fn success_data_test_fdm_reference_projections(
    candidate: &ObjectStreamCandidate,
) -> Vec<SuccessDataTestFdmProjection> {
    let q4_target_height_px = success_data_test_uniform_target_height_px(
        SUCCESS_DATA_TEST_Q4_SOURCE_LEFT,
        SUCCESS_DATA_TEST_Q4_SOURCE_TOP,
        SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT,
        SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM,
        SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX,
    );
    let mut projections = vec![SuccessDataTestFdmProjection {
        role: "q4-angle-diagrams",
        source_left: SUCCESS_DATA_TEST_Q4_SOURCE_LEFT,
        source_top: SUCCESS_DATA_TEST_Q4_SOURCE_TOP,
        source_right: SUCCESS_DATA_TEST_Q4_SOURCE_RIGHT,
        source_bottom: SUCCESS_DATA_TEST_Q4_SOURCE_BOTTOM,
        target_x_px: SUCCESS_DATA_TEST_Q4_TARGET_X_PX,
        target_y_px: SUCCESS_DATA_TEST_Q4_TARGET_Y_PX,
        target_width_px: SUCCESS_DATA_TEST_Q4_TARGET_WIDTH_PX,
        target_height_px: q4_target_height_px,
        scale_mode: "uniform-units-from-horizontal-span",
    }];
    if let Some(q5_projection) =
        success_data_test_q5_fdm_projection_from_segments(candidate.fdm_raw_vector_segments())
    {
        projections.push(q5_projection);
    }
    projections
}

fn success_data_test_uniform_target_height_px(
    source_left: i32,
    source_top: i32,
    source_right: i32,
    source_bottom: i32,
    target_width_px: f32,
) -> f32 {
    let source_width = source_right.saturating_sub(source_left).abs().max(1) as f32;
    let source_height = source_bottom.saturating_sub(source_top).abs().max(1) as f32;
    source_height / source_width * target_width_px
}

fn success_data_test_q5_fdm_projection_from_segments(
    segments: &[ObjectFdmVectorSegmentCandidate],
) -> Option<SuccessDataTestFdmProjection> {
    let nonzero_span_segments = segments
        .iter()
        .filter(|segment| {
            segment.source_width() > 0 && segment.source_height() > 0 && segment.bbox().is_some()
        })
        .collect::<Vec<_>>();
    if nonzero_span_segments.len() < 2 {
        return None;
    }

    let first_offset = nonzero_span_segments.first()?.relative_offset();
    let mut selected = nonzero_span_segments
        .iter()
        .copied()
        .filter(|segment| segment.relative_offset() != first_offset);
    let first = selected.next()?;
    let first_bbox = first.bbox().map(normalize_fdm_bbox)?;
    let (mut left, mut top, mut right, mut bottom) = first_bbox;
    for segment in selected {
        let bbox = segment.bbox().map(normalize_fdm_bbox)?;
        left = left.min(bbox.0);
        top = top.min(bbox.1);
        right = right.max(bbox.2);
        bottom = bottom.max(bbox.3);
    }

    Some(SuccessDataTestFdmProjection {
        role: "q5-solid-diagram",
        source_left: left,
        source_top: top,
        source_right: right,
        source_bottom: bottom,
        target_x_px: SUCCESS_DATA_TEST_Q5_TARGET_X_PX,
        target_y_px: SUCCESS_DATA_TEST_Q5_TARGET_Y_PX,
        target_width_px: SUCCESS_DATA_TEST_Q5_TARGET_WIDTH_PX,
        target_height_px: SUCCESS_DATA_TEST_Q5_TARGET_HEIGHT_PX,
        scale_mode: "independent-reference-box",
    })
}

fn success_data_test_fdm_projection_command(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    let Some(bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let (center_x, center_y) = fdm_bbox_center(bbox);
    center_x >= projection.source_left
        && center_x <= projection.source_right
        && center_y >= projection.source_top
        && center_y <= projection.source_bottom
}

#[derive(Debug)]
struct SuccessDataTestFdmSubdiagram<'a> {
    index: usize,
    anchor_relative_offset: usize,
    center: ObjectFdmVectorPoint,
    anchor_radius: i32,
    commands: Vec<&'a ObjectFdmVectorCommandCandidate>,
}

fn success_data_test_q4_fdm_subdiagrams<'a>(
    projection: SuccessDataTestFdmProjection,
    commands: &[&'a ObjectFdmVectorCommandCandidate],
) -> Option<Vec<SuccessDataTestFdmSubdiagram<'a>>> {
    if projection.role != "q4-angle-diagrams" {
        return None;
    }
    let mut subdiagrams = commands
        .iter()
        .filter_map(|&command| {
            let ellipse = command.ellipse()?;
            success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse)
                .then(|| SuccessDataTestFdmSubdiagram {
                    index: 0,
                    anchor_relative_offset: command.relative_offset(),
                    center: ellipse.center(),
                    anchor_radius: ellipse.radius_x().max(ellipse.radius_y()),
                    commands: Vec::new(),
                })
        })
        .collect::<Vec<_>>();
    if subdiagrams.len() < 2 {
        return None;
    }
    subdiagrams.sort_by_key(|subdiagram| {
        (
            subdiagram.center.x(),
            subdiagram.center.y(),
            subdiagram.anchor_relative_offset,
        )
    });
    for (index, subdiagram) in subdiagrams.iter_mut().enumerate() {
        subdiagram.index = index;
    }

    for &command in commands {
        let Some(center) = success_data_test_fdm_command_source_center(command) else {
            continue;
        };
        let Some((group_index, _)) = subdiagrams
            .iter()
            .enumerate()
            .map(|(index, subdiagram)| {
                (index, fdm_point_distance_squared(center, subdiagram.center))
            })
            .min_by_key(|(_, distance)| *distance)
        else {
            continue;
        };
        subdiagrams[group_index].commands.push(command);
    }

    subdiagrams
        .iter()
        .all(|subdiagram| !subdiagram.commands.is_empty())
        .then_some(subdiagrams)
}

fn success_data_test_fdm_command_source_center(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<(i32, i32)> {
    if let Some(ellipse) = command.ellipse() {
        let center = ellipse.center();
        return Some((center.x(), center.y()));
    }
    let bbox = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox)?;
    Some(fdm_bbox_center(bbox))
}

fn success_data_test_fdm_reference_ellipse_has_center_marker(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
    ellipse: ObjectFdmVectorEllipse,
) -> bool {
    if projection.role != "q4-angle-diagrams" || command.marker() != b"\x01\x00\x04\x60" {
        return false;
    }
    let source_height = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs()
        .max(1);
    ellipse.radius_x() == ellipse.radius_y()
        && ellipse.radius_x().saturating_mul(2) >= source_height.saturating_mul(4) / 5
}

fn success_data_test_fdm_reference_ellipse_is_control_marker(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
    ellipse: ObjectFdmVectorEllipse,
) -> bool {
    if projection.role != "q4-angle-diagrams" || command.marker() != b"\xff\x00\x04\x60" {
        return false;
    }
    let source_height = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs()
        .max(1);
    ellipse.radius_x() == ellipse.radius_y()
        && ellipse.radius_x().saturating_mul(6) <= source_height
}

fn fdm_point_distance_squared(left: (i32, i32), right: ObjectFdmVectorPoint) -> i64 {
    let dx = i64::from(left.0) - i64::from(right.x());
    let dy = i64::from(left.1) - i64::from(right.y());
    dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy))
}

#[derive(Debug)]
struct SuccessDataTestFdmSourceCohort {
    command_relative_offsets: Vec<usize>,
    source_vector_offset_start: Option<usize>,
    source_vector_offset_end: Option<usize>,
    source_vector_offset_count: usize,
    segment_backed_count: usize,
    raw_span_count: usize,
    segment_offsets: Vec<usize>,
}

impl SuccessDataTestFdmSourceCohort {
    fn blocked_reason(&self) -> &'static str {
        if self.raw_span_count > 0 && self.segment_backed_count > 0 {
            "mixed-raw-and-segment-cohorts"
        } else if self.segment_offsets.len() > 1 {
            "multiple-source-segment-cohorts"
        } else {
            "source-owner-candidate-unproven"
        }
    }
}

fn success_data_test_fdm_source_cohort(
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> SuccessDataTestFdmSourceCohort {
    let mut segment_offsets = std::collections::BTreeSet::new();
    let mut command_relative_offsets = Vec::new();
    let mut source_vector_offset_start: Option<usize> = None;
    let mut source_vector_offset_end: Option<usize> = None;
    let mut source_vector_offset_count = 0usize;
    let mut segment_backed_count = 0usize;
    for command in commands {
        command_relative_offsets.push(command.relative_offset());
        if let Some(source_vector_relative_offset) = command.source_vector_relative_offset() {
            source_vector_offset_count += 1;
            source_vector_offset_start = Some(
                source_vector_offset_start
                    .map(|start| start.min(source_vector_relative_offset))
                    .unwrap_or(source_vector_relative_offset),
            );
            source_vector_offset_end = Some(
                source_vector_offset_end
                    .map(|end| end.max(source_vector_relative_offset))
                    .unwrap_or(source_vector_relative_offset),
            );
        }
        if let Some(source_segment) = command.source_segment() {
            segment_backed_count += 1;
            segment_offsets.insert(source_segment.relative_offset());
        }
    }
    let raw_span_count = commands.len().saturating_sub(segment_backed_count);
    SuccessDataTestFdmSourceCohort {
        command_relative_offsets,
        source_vector_offset_start,
        source_vector_offset_end,
        source_vector_offset_count,
        segment_backed_count,
        raw_span_count,
        segment_offsets: segment_offsets.into_iter().collect(),
    }
}

fn push_success_data_test_fdm_source_cohort_json(
    output: &mut String,
    commands: &[&ObjectFdmVectorCommandCandidate],
) {
    let cohort = success_data_test_fdm_source_cohort(commands);
    output.push_str("{\"provenance\":\"fdm-vector-command\",\"ownershipBasis\":\"fdmVectorCommandProvenance\",\"ownershipProven\":false");
    output.push_str(",\"ownershipPromotionBlockedReason\":");
    push_json_string(output, cohort.blocked_reason());
    output.push_str(",\"sourceVectorOffsetStart\":");
    push_option_usize_json(output, cohort.source_vector_offset_start);
    output.push_str(",\"sourceVectorOffsetEnd\":");
    push_option_usize_json(output, cohort.source_vector_offset_end);
    output.push_str(",\"commandRelativeOffsets\":");
    push_usize_array_json(output, &cohort.command_relative_offsets);
    output.push_str(",\"sourceVectorOffsetCommandCount\":");
    output.push_str(&cohort.source_vector_offset_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&cohort.segment_backed_count.to_string());
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&cohort.raw_span_count.to_string());
    output.push_str(",\"sourceSegmentCohortCount\":");
    output.push_str(&cohort.segment_offsets.len().to_string());
    output.push_str(",\"sourceSegmentRelativeOffsets\":");
    push_usize_array_json(output, &cohort.segment_offsets);
    output.push('}');
}

#[derive(Debug)]
struct SuccessDataTestFdmPrimitiveOwnershipClassification<'a> {
    command: &'a ObjectFdmVectorCommandCandidate,
    role_candidates: Vec<&'static str>,
    classification_basis: Vec<&'static str>,
    index_row_references: Vec<SuccessDataTestFdmIndexRowReference>,
}

#[derive(Debug)]
struct SuccessDataTestFdmIndexRowReference {
    row_index: usize,
    index_offset: usize,
    vector_offset: usize,
    valid_vector_offset: bool,
    offset_field: &'static str,
    offset_value: usize,
    match_kind: &'static str,
}

fn push_success_data_test_fdm_primitive_ownership_comparison_json(
    output: &mut String,
    projection: SuccessDataTestFdmProjection,
    commands: &[&ObjectFdmVectorCommandCandidate],
    index_entries: &[ObjectFdmIndexEntryCandidate],
    anchor: Option<(ObjectFdmVectorPoint, i32)>,
) {
    let classifications = commands
        .iter()
        .map(|&command| {
            success_data_test_fdm_primitive_ownership_classification(
                projection,
                command,
                index_entries,
                anchor,
            )
        })
        .collect::<Vec<_>>();
    output.push_str("{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false");
    output.push_str(
        ",\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\"",
    );
    output.push_str(",\"commandCount\":");
    output.push_str(&classifications.len().to_string());
    push_success_data_test_fdm_role_count_json(
        output,
        "mainCircleAnchorCount",
        &classifications,
        "main-circle-anchor",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "lineCandidateCount",
        &classifications,
        "line-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "radialLineCandidateCount",
        &classifications,
        "radial-line-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "chordCandidateCount",
        &classifications,
        "chord-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "arcCandidateCount",
        &classifications,
        "arc-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "connectorCandidateCount",
        &classifications,
        "connector-candidate",
    );
    push_success_data_test_fdm_role_count_json(
        output,
        "surfaceBoundaryCandidateCount",
        &classifications,
        "surface-boundary-candidate",
    );
    output.push_str(",\"indexRowReferenceCandidateCount\":");
    output.push_str(
        &classifications
            .iter()
            .map(|classification| classification.index_row_references.len())
            .sum::<usize>()
            .to_string(),
    );
    output.push_str(",\"validVectorOffsetIndexRowReferenceCount\":");
    output.push_str(
        &classifications
            .iter()
            .flat_map(|classification| classification.index_row_references.iter())
            .filter(|reference| reference.valid_vector_offset)
            .count()
            .to_string(),
    );
    output.push_str(",\"ownershipGate\":");
    push_success_data_test_fdm_primitive_ownership_gate_json(output, &classifications);
    output.push_str(",\"offsetFieldAuthorityGate\":");
    push_success_data_test_fdm_offset_field_authority_gate_json(output, &classifications);
    output.push_str(",\"rowFanoutSegmentOwnerGate\":");
    push_success_data_test_fdm_row_fanout_segment_owner_gate_json(output, &classifications);
    output.push_str(",\"primitiveOwnershipAdmissionGate\":");
    push_success_data_test_fdm_primitive_ownership_admission_gate_json(output, &classifications);
    output.push_str(",\"indexRowOrderPromotionGate\":");
    push_success_data_test_fdm_index_row_order_promotion_gate_json(output, &classifications);
    output.push_str(",\"indexRowReferenceRoleCandidateGroups\":");
    push_success_data_test_fdm_index_row_reference_role_candidate_groups_json(
        output,
        &classifications,
    );
    output.push_str(",\"classifications\":[");
    for (index, classification) in classifications.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"relativeOffset\":");
        output.push_str(&classification.command.relative_offset().to_string());
        output.push_str(",\"primitiveKind\":");
        push_json_string(output, fdm_vector_primitive_kind(classification.command));
        output.push_str(",\"markerHex\":");
        push_json_string(output, &hex(classification.command.marker()));
        output.push_str(",\"sourceSegmentBacked\":");
        output.push_str(if classification.command.source_segment().is_some() {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"sourceSegmentRelativeOffset\":");
        push_option_usize_json(
            output,
            classification
                .command
                .source_segment()
                .map(|segment| segment.relative_offset()),
        );
        output.push_str(",\"roleCandidates\":");
        push_json_string_slice_array(output, &classification.role_candidates);
        output.push_str(",\"classificationBasis\":");
        push_json_string_slice_array(output, &classification.classification_basis);
        output.push_str(",\"indexRowReferenceCandidates\":");
        push_success_data_test_fdm_index_row_references_json(
            output,
            &classification.index_row_references,
        );
        output.push('}');
    }
    output.push_str("]}");
}

fn push_success_data_test_fdm_role_count_json(
    output: &mut String,
    field_name: &str,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
    role: &str,
) {
    let count = classifications
        .iter()
        .filter(|classification| classification.role_candidates.contains(&role))
        .count();
    output.push(',');
    push_json_string(output, field_name);
    output.push(':');
    output.push_str(&count.to_string());
}

#[derive(Debug, Default)]
struct SuccessDataTestFdmIndexRowOrderPromotionGate {
    command_count: usize,
    referenced_command_relative_offsets: BTreeSet<usize>,
    referenced_row_indexes: BTreeSet<usize>,
    row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
    row_to_command_relative_offsets: BTreeMap<usize, BTreeSet<usize>>,
    reference_count: usize,
    valid_vector_offset_reference_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
}

impl SuccessDataTestFdmIndexRowOrderPromotionGate {
    fn referenced_command_count(&self) -> usize {
        self.referenced_command_relative_offsets.len()
    }

    fn unreferenced_command_count(&self) -> usize {
        self.command_count
            .saturating_sub(self.referenced_command_count())
    }

    fn unique_row_index_count(&self) -> usize {
        self.referenced_row_indexes.len()
    }

    fn all_commands_referenced_by_index_rows_candidate(&self) -> bool {
        self.command_count > 0 && self.unreferenced_command_count() == 0
    }

    fn one_to_one_row_command_reference_candidate(&self) -> bool {
        self.reference_count == self.referenced_command_count()
            && self.reference_count == self.unique_row_index_count()
    }

    fn single_row_backs_multiple_commands_candidate(&self) -> bool {
        self.row_to_command_relative_offsets
            .values()
            .any(|offsets| offsets.len() > 1)
    }

    fn row_order_matches_command_order_candidate(&self) -> bool {
        success_data_test_fdm_row_command_pairs_are_monotonic(&self.row_command_pairs)
    }
}

#[derive(Debug)]
struct SuccessDataTestFdmOffsetFieldAuthorityGate {
    command_count: usize,
    reference_count: usize,
    valid_vector_offset_reference_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
    unclassified_offset_field_reference_count: usize,
    raw_span_command_count: usize,
    segment_backed_command_count: usize,
    mixed_offset_field_namespaces: bool,
    mixed_command_provenance_cohorts: bool,
    all_references_use_command_relative_offset_field: bool,
    all_references_use_source_segment_relative_offset_field: bool,
    render_promotion_blocked_reason: &'static str,
}

fn success_data_test_fdm_offset_field_authority_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmOffsetFieldAuthorityGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let unclassified_offset_field_reference_count = order_gate
        .reference_count
        .saturating_sub(order_gate.command_relative_offset_field_reference_count)
        .saturating_sub(order_gate.source_segment_relative_offset_field_reference_count);
    let mixed_offset_field_namespaces = order_gate.command_relative_offset_field_reference_count
        > 0
        && order_gate.source_segment_relative_offset_field_reference_count > 0;
    let mixed_command_provenance_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let all_references_use_command_relative_offset_field = order_gate.reference_count > 0
        && order_gate.command_relative_offset_field_reference_count == order_gate.reference_count;
    let all_references_use_source_segment_relative_offset_field = order_gate.reference_count > 0
        && order_gate.source_segment_relative_offset_field_reference_count
            == order_gate.reference_count;
    let render_promotion_blocked_reason = if mixed_offset_field_namespaces {
        "fdm-index-offset-field-authority-mixed-command-and-segment-fields"
    } else if mixed_command_provenance_cohorts {
        "fdm-index-offset-field-authority-mixed-raw-and-segment-cohorts"
    } else if unclassified_offset_field_reference_count > 0 {
        "fdm-index-offset-field-authority-unclassified-fields"
    } else if order_gate.valid_vector_offset_reference_count == 0 {
        "fdm-index-offset-field-authority-valid-vector-offset-missing"
    } else {
        "fdm-index-offset-field-authority-semantics-unproven"
    };

    SuccessDataTestFdmOffsetFieldAuthorityGate {
        command_count: order_gate.command_count,
        reference_count: order_gate.reference_count,
        valid_vector_offset_reference_count: order_gate.valid_vector_offset_reference_count,
        command_relative_offset_field_reference_count: order_gate
            .command_relative_offset_field_reference_count,
        source_segment_relative_offset_field_reference_count: order_gate
            .source_segment_relative_offset_field_reference_count,
        unclassified_offset_field_reference_count,
        raw_span_command_count,
        segment_backed_command_count,
        mixed_offset_field_namespaces,
        mixed_command_provenance_cohorts,
        all_references_use_command_relative_offset_field,
        all_references_use_source_segment_relative_offset_field,
        render_promotion_blocked_reason,
    }
}

fn push_success_data_test_fdm_offset_field_authority_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_offset_field_authority_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-offset-field-authority-gate\",\"source\":\"FDMIndex row offset fields+FDMVector command provenance\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"offsetFieldAuthorityDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-offset-field-authority-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, gate.render_promotion_blocked_reason);
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&gate.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"unclassifiedOffsetFieldReferenceCount\":");
    output.push_str(&gate.unclassified_offset_field_reference_count.to_string());
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if gate.mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedCommandProvenanceCohorts\":");
    output.push_str(if gate.mixed_command_provenance_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesUseCommandRelativeOffsetField\":");
    output.push_str(if gate.all_references_use_command_relative_offset_field {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesUseSourceSegmentRelativeOffsetField\":");
    output.push_str(
        if gate.all_references_use_source_segment_relative_offset_field {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

#[derive(Debug)]
struct SuccessDataTestFdmRowFanoutSegmentOwnerGate {
    command_count: usize,
    reference_count: usize,
    unique_row_index_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
    fanout_row_count: usize,
    fanout_reference_count: usize,
    fanout_command_relative_offset_field_reference_count: usize,
    fanout_source_segment_relative_offset_field_reference_count: usize,
    max_row_fanout: usize,
    multi_command_row_indexes: Vec<usize>,
    rows_with_multiple_command_refs: Vec<SuccessDataTestFdmRowFanoutSegmentOwnerRow>,
    one_to_one_row_command_reference_candidate: bool,
    single_row_backs_multiple_commands_candidate: bool,
    mixed_offset_field_namespaces: bool,
    mixed_command_provenance_cohorts: bool,
    fanout_rows_use_command_relative_offset_fields: bool,
    fanout_rows_use_source_segment_offset_fields: bool,
    raw_span_command_count: usize,
    segment_backed_command_count: usize,
    render_promotion_blocked_reason: &'static str,
}

#[derive(Debug)]
struct SuccessDataTestFdmRowFanoutSegmentOwnerRow {
    row_index: usize,
    command_reference_count: usize,
    command_relative_offsets: Vec<usize>,
    match_kinds: Vec<&'static str>,
}

fn success_data_test_fdm_row_fanout_segment_owner_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmRowFanoutSegmentOwnerGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let mut multi_command_row_indexes = Vec::new();
    let mut fanout_reference_count = 0usize;
    let mut fanout_command_relative_offset_field_reference_count = 0usize;
    let mut fanout_source_segment_relative_offset_field_reference_count = 0usize;
    let mut max_row_fanout = 0usize;
    let mut rows_with_multiple_command_refs = Vec::new();
    for (row_index, command_offsets) in &order_gate.row_to_command_relative_offsets {
        max_row_fanout = max_row_fanout.max(command_offsets.len());
        if command_offsets.len() <= 1 {
            continue;
        }
        multi_command_row_indexes.push(*row_index);
        let row_pairs = order_gate
            .row_command_pairs
            .iter()
            .filter(|pair| pair.row_index == *row_index)
            .collect::<Vec<_>>();
        for pair in &row_pairs {
            fanout_reference_count += 1;
            match pair.match_kind {
                "command-relative-offset-field" => {
                    fanout_command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    fanout_source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
        rows_with_multiple_command_refs.push(SuccessDataTestFdmRowFanoutSegmentOwnerRow {
            row_index: *row_index,
            command_reference_count: row_pairs.len(),
            command_relative_offsets: row_pairs
                .iter()
                .map(|pair| pair.command_relative_offset)
                .collect(),
            match_kinds: row_pairs
                .iter()
                .map(|pair| pair.match_kind)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        });
    }
    let mixed_offset_field_namespaces = order_gate.command_relative_offset_field_reference_count
        > 0
        && order_gate.source_segment_relative_offset_field_reference_count > 0;
    let mixed_command_provenance_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let single_row_backs_multiple_commands_candidate =
        order_gate.single_row_backs_multiple_commands_candidate();
    let one_to_one_row_command_reference_candidate =
        order_gate.one_to_one_row_command_reference_candidate();
    let fanout_rows_use_command_relative_offset_fields = fanout_reference_count > 0
        && fanout_command_relative_offset_field_reference_count == fanout_reference_count;
    let fanout_rows_use_source_segment_offset_fields = fanout_reference_count > 0
        && fanout_source_segment_relative_offset_field_reference_count == fanout_reference_count;
    let render_promotion_blocked_reason = if single_row_backs_multiple_commands_candidate {
        "fdm-index-row-fanout-segment-owner-multi-command-single-row"
    } else if !one_to_one_row_command_reference_candidate {
        "fdm-index-row-fanout-segment-owner-not-one-to-one"
    } else if mixed_offset_field_namespaces {
        "fdm-index-row-fanout-segment-owner-offset-namespace-mixed"
    } else if mixed_command_provenance_cohorts {
        "fdm-index-row-fanout-segment-owner-mixed-raw-and-segment-cohorts"
    } else {
        "fdm-index-row-fanout-segment-owner-semantics-unproven"
    };

    SuccessDataTestFdmRowFanoutSegmentOwnerGate {
        command_count: order_gate.command_count,
        reference_count: order_gate.reference_count,
        unique_row_index_count: order_gate.unique_row_index_count(),
        command_relative_offset_field_reference_count: order_gate
            .command_relative_offset_field_reference_count,
        source_segment_relative_offset_field_reference_count: order_gate
            .source_segment_relative_offset_field_reference_count,
        fanout_row_count: multi_command_row_indexes.len(),
        fanout_reference_count,
        fanout_command_relative_offset_field_reference_count,
        fanout_source_segment_relative_offset_field_reference_count,
        max_row_fanout,
        multi_command_row_indexes,
        rows_with_multiple_command_refs,
        one_to_one_row_command_reference_candidate,
        single_row_backs_multiple_commands_candidate,
        mixed_offset_field_namespaces,
        mixed_command_provenance_cohorts,
        fanout_rows_use_command_relative_offset_fields,
        fanout_rows_use_source_segment_offset_fields,
        raw_span_command_count,
        segment_backed_command_count,
        render_promotion_blocked_reason,
    }
}

fn push_success_data_test_fdm_row_fanout_segment_owner_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_row_fanout_segment_owner_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"rowFanoutDecoded\":false,\"segmentOwnerDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-row-fanout-segment-owner-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, gate.render_promotion_blocked_reason);
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&gate.unique_row_index_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutRowCount\":");
    output.push_str(&gate.fanout_row_count.to_string());
    output.push_str(",\"fanoutReferenceCount\":");
    output.push_str(&gate.fanout_reference_count.to_string());
    output.push_str(",\"fanoutCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .fanout_command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .fanout_source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"maxRowFanout\":");
    output.push_str(&gate.max_row_fanout.to_string());
    output.push_str(",\"multiCommandRowIndexes\":");
    push_usize_array_json(output, &gate.multi_command_row_indexes);
    output.push_str(",\"rowsWithMultipleCommandRefs\":");
    push_success_data_test_fdm_row_fanout_segment_owner_rows_json(
        output,
        &gate.rows_with_multiple_command_refs,
    );
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if gate.single_row_backs_multiple_commands_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if gate.mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedCommandProvenanceCohorts\":");
    output.push_str(if gate.mixed_command_provenance_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseCommandRelativeOffsetFields\":");
    output.push_str(if gate.fanout_rows_use_command_relative_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseSourceSegmentOffsetFields\":");
    output.push_str(if gate.fanout_rows_use_source_segment_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push('}');
}

fn push_success_data_test_fdm_row_fanout_segment_owner_rows_json(
    output: &mut String,
    rows: &[SuccessDataTestFdmRowFanoutSegmentOwnerRow],
) {
    output.push('[');
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&row.row_index.to_string());
        output.push_str(",\"commandReferenceCount\":");
        output.push_str(&row.command_reference_count.to_string());
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(output, &row.command_relative_offsets);
        output.push_str(",\"matchKinds\":");
        push_json_string_slice_array(output, &row.match_kinds);
        output.push('}');
    }
    output.push(']');
}

#[derive(Debug)]
struct SuccessDataTestFdmPrimitiveOwnershipGate {
    row_command_gap_p95: Option<f32>,
    row_direction_mismatch: bool,
    multi_command_single_row: bool,
    all_commands_referenced_by_index_rows_candidate: bool,
    one_to_one_row_command_reference_candidate: bool,
    mixed_raw_and_segment_cohorts: bool,
    raw_span_command_count: usize,
    segment_backed_command_count: usize,
    ownership_proven: bool,
    render_ownership_blocked_reason: &'static str,
    render_ownership_blocked_reasons: Vec<&'static str>,
}

fn success_data_test_fdm_primitive_ownership_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmPrimitiveOwnershipGate {
    let order_gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let raw_span_command_count = classifications
        .iter()
        .filter(|classification| classification.command.source_segment().is_none())
        .count();
    let segment_backed_command_count = classifications.len().saturating_sub(raw_span_command_count);
    let row_direction_mismatch = !order_gate.row_order_matches_command_order_candidate();
    let multi_command_single_row = order_gate.single_row_backs_multiple_commands_candidate();
    let all_commands_referenced_by_index_rows_candidate =
        order_gate.all_commands_referenced_by_index_rows_candidate();
    let one_to_one_row_command_reference_candidate =
        order_gate.one_to_one_row_command_reference_candidate();
    let mixed_raw_and_segment_cohorts =
        raw_span_command_count > 0 && segment_backed_command_count > 0;
    let mut render_ownership_blocked_reasons = Vec::new();
    if row_direction_mismatch {
        render_ownership_blocked_reasons.push("row-command-direction-mismatch");
    }
    if !all_commands_referenced_by_index_rows_candidate {
        render_ownership_blocked_reasons.push("index-row-reference-coverage-incomplete");
    }
    if multi_command_single_row {
        render_ownership_blocked_reasons.push("multi-command-single-index-row");
    }
    if mixed_raw_and_segment_cohorts {
        render_ownership_blocked_reasons.push("mixed-raw-and-segment-cohorts");
    }
    if !one_to_one_row_command_reference_candidate {
        render_ownership_blocked_reasons.push("row-command-reference-not-one-to-one");
    }
    let render_ownership_blocked_reason = render_ownership_blocked_reasons
        .first()
        .copied()
        .unwrap_or("fdm-index-row-ownership-unproven");

    SuccessDataTestFdmPrimitiveOwnershipGate {
        row_command_gap_p95: success_data_test_fdm_command_gap_p95(
            &order_gate.referenced_command_relative_offsets,
        ),
        row_direction_mismatch,
        multi_command_single_row,
        all_commands_referenced_by_index_rows_candidate,
        one_to_one_row_command_reference_candidate,
        mixed_raw_and_segment_cohorts,
        raw_span_command_count,
        segment_backed_command_count,
        ownership_proven: false,
        render_ownership_blocked_reason,
        render_ownership_blocked_reasons,
    }
}

fn success_data_test_fdm_command_gap_p95(offsets: &BTreeSet<usize>) -> Option<f32> {
    let mut gaps = Vec::new();
    let mut previous_offset = None;
    for offset in offsets.iter().copied() {
        if let Some(previous) = previous_offset {
            gaps.push(offset.saturating_sub(previous));
        }
        previous_offset = Some(offset);
    }
    if gaps.is_empty() {
        return None;
    }
    gaps.sort_unstable();
    let rank = ((gaps.len() as f32) * 0.95).ceil() as usize;
    let index = rank.saturating_sub(1).min(gaps.len() - 1);
    Some(gaps[index] as f32)
}

fn push_success_data_test_fdm_primitive_ownership_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_primitive_ownership_gate(classifications);
    output.push_str("{\"basis\":\"fdm-index-row-reference-primitive-ownership-gate\",\"source\":\"FDMIndex row references+FDMVector command provenance\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"ownershipProven\":");
    output.push_str(if gate.ownership_proven {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintOrderDecoded\":false,\"renderOwnershipPromoted\":false");
    output.push_str(",\"renderOwnershipBlockedReason\":");
    push_json_string(output, gate.render_ownership_blocked_reason);
    output.push_str(",\"renderOwnershipBlockedReasons\":");
    push_json_string_slice_array(output, &gate.render_ownership_blocked_reasons);
    output.push_str(",\"rowCommandGapP95\":");
    push_option_f32_json(output, gate.row_command_gap_p95);
    output.push_str(",\"rowDirectionMismatch\":");
    output.push_str(if gate.row_direction_mismatch {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"multiCommandSingleRow\":");
    output.push_str(if gate.multi_command_single_row {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allCommandsReferencedByIndexRowsCandidate\":");
    output.push_str(if gate.all_commands_referenced_by_index_rows_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedRawAndSegmentCohorts\":");
    output.push_str(if gate.mixed_raw_and_segment_cohorts {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rawSpanCommandCount\":");
    output.push_str(&gate.raw_span_command_count.to_string());
    output.push_str(",\"segmentBackedCommandCount\":");
    output.push_str(&gate.segment_backed_command_count.to_string());
    output.push('}');
}

fn push_success_data_test_fdm_primitive_ownership_admission_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let ownership_gate = success_data_test_fdm_primitive_ownership_gate(classifications);
    let offset_field_gate = success_data_test_fdm_offset_field_authority_gate(classifications);
    let row_fanout_gate = success_data_test_fdm_row_fanout_segment_owner_gate(classifications);
    let role_groups =
        success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);

    let mut role_fanout_blocked_role_candidates = Vec::new();
    let mut role_vector_offset_authority_blocked_role_candidates = Vec::new();
    let mut role_vector_offset_authority_blocked_reasons = Vec::new();
    let mut role_valid_vector_offset_missing_role_candidates = Vec::new();
    let mut role_paint_order_blocked_role_candidates = Vec::new();
    let mut role_paint_order_authority_pending_role_candidates = Vec::new();
    for group in role_groups.values() {
        if success_data_test_fdm_role_group_single_row_backs_multiple_commands(group) {
            role_fanout_blocked_role_candidates.push(group.role_candidate);
        }
        let role_vector_offset_authority_blocked_reason =
            success_data_test_fdm_role_vector_offset_authority_blocked_reason(group);
        push_unique_static_str(
            &mut role_vector_offset_authority_blocked_reasons,
            role_vector_offset_authority_blocked_reason,
        );
        role_vector_offset_authority_blocked_role_candidates.push(group.role_candidate);
        if group.valid_vector_offset_reference_count == 0 && group.reference_count > 0 {
            role_valid_vector_offset_missing_role_candidates.push(group.role_candidate);
        }
        let paint_order_profile =
            success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
        if paint_order_profile.continuity_blocked() {
            role_paint_order_blocked_role_candidates.push(group.role_candidate);
        } else if paint_order_profile.paint_order_authority_pending() {
            role_paint_order_authority_pending_role_candidates.push(group.role_candidate);
        }
    }

    let role_fanout_blocked_group_count = role_fanout_blocked_role_candidates.len();
    let role_vector_offset_authority_blocked_group_count =
        role_vector_offset_authority_blocked_role_candidates.len();
    let role_valid_vector_offset_missing_group_count =
        role_valid_vector_offset_missing_role_candidates.len();
    let role_paint_order_blocked_group_count = role_paint_order_blocked_role_candidates.len();
    let role_paint_order_authority_pending_group_count =
        role_paint_order_authority_pending_role_candidates.len();
    let mut render_promotion_blocked_reasons = Vec::new();
    for reason in &ownership_gate.render_ownership_blocked_reasons {
        push_unique_static_str(&mut render_promotion_blocked_reasons, reason);
    }
    push_unique_static_str(
        &mut render_promotion_blocked_reasons,
        offset_field_gate.render_promotion_blocked_reason,
    );
    push_unique_static_str(
        &mut render_promotion_blocked_reasons,
        row_fanout_gate.render_promotion_blocked_reason,
    );
    if role_fanout_blocked_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "fdm-index-role-row-fanout-multi-command-single-row",
        );
    }
    for reason in &role_vector_offset_authority_blocked_reasons {
        push_unique_static_str(&mut render_promotion_blocked_reasons, reason);
    }
    if role_valid_vector_offset_missing_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "fdm-index-role-valid-vector-offset-missing",
        );
    }
    if role_paint_order_blocked_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "role-paint-order-continuity-unproven",
        );
    }
    if role_paint_order_authority_pending_group_count > 0 {
        push_unique_static_str(
            &mut render_promotion_blocked_reasons,
            "role-paint-order-authority-unproven",
        );
    }
    let render_admission_ready = render_promotion_blocked_reasons.is_empty();
    let render_promotion_blocked_reason = render_promotion_blocked_reasons
        .first()
        .copied()
        .unwrap_or("none");

    output.push_str("{\"basis\":\"fdm-primitive-ownership-admission-gate\",\"source\":\"ownershipGate+offsetFieldAuthorityGate+rowFanoutSegmentOwnerGate+roleFanoutSegmentOwnerGate+paintOrderContinuityProfile\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"ownershipProven\":false,\"paintOrderDecoded\":false");
    output.push_str(",\"renderAdmissionReady\":");
    output.push_str(if render_admission_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"fdm-primitive-ownership-admission-gate\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, render_promotion_blocked_reason);
    output.push_str(",\"renderPromotionBlockedReasons\":");
    push_json_string_slice_array(output, &render_promotion_blocked_reasons);
    output.push_str(",\"commandCount\":");
    output.push_str(
        &ownership_gate
            .raw_span_command_count
            .saturating_add(ownership_gate.segment_backed_command_count)
            .to_string(),
    );
    output.push_str(",\"referenceCount\":");
    output.push_str(&offset_field_gate.reference_count.to_string());
    output.push_str(",\"roleGroupCount\":");
    output.push_str(&role_groups.len().to_string());
    output.push_str(",\"ownershipGateBlockedReason\":");
    push_json_string(output, ownership_gate.render_ownership_blocked_reason);
    output.push_str(",\"offsetFieldAuthorityBlockedReason\":");
    push_json_string(output, offset_field_gate.render_promotion_blocked_reason);
    output.push_str(",\"rowFanoutSegmentOwnerBlockedReason\":");
    push_json_string(output, row_fanout_gate.render_promotion_blocked_reason);
    output.push_str(",\"projectionRowFanoutBlocked\":");
    output.push_str(
        if row_fanout_gate.single_row_backs_multiple_commands_candidate {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"roleFanoutBlockedGroupCount\":");
    output.push_str(&role_fanout_blocked_group_count.to_string());
    output.push_str(",\"roleFanoutBlockedRoleCandidates\":");
    push_json_string_slice_array(output, &role_fanout_blocked_role_candidates);
    output.push_str(",\"roleVectorOffsetAuthorityBlockedGroupCount\":");
    output.push_str(&role_vector_offset_authority_blocked_group_count.to_string());
    output.push_str(",\"roleVectorOffsetAuthorityBlockedRoleCandidates\":");
    push_json_string_slice_array(
        output,
        &role_vector_offset_authority_blocked_role_candidates,
    );
    output.push_str(",\"roleVectorOffsetAuthorityBlockedReasons\":");
    push_json_string_slice_array(output, &role_vector_offset_authority_blocked_reasons);
    output.push_str(",\"roleValidVectorOffsetMissingGroupCount\":");
    output.push_str(&role_valid_vector_offset_missing_group_count.to_string());
    output.push_str(",\"roleValidVectorOffsetMissingRoleCandidates\":");
    push_json_string_slice_array(output, &role_valid_vector_offset_missing_role_candidates);
    output.push_str(",\"rolePaintOrderBlockedGroupCount\":");
    output.push_str(&role_paint_order_blocked_group_count.to_string());
    output.push_str(",\"rolePaintOrderBlockedRoleCandidates\":");
    push_json_string_slice_array(output, &role_paint_order_blocked_role_candidates);
    output.push_str(",\"rolePaintOrderAuthorityPendingGroupCount\":");
    output.push_str(&role_paint_order_authority_pending_group_count.to_string());
    output.push_str(",\"rolePaintOrderAuthorityPendingRoleCandidates\":");
    push_json_string_slice_array(output, &role_paint_order_authority_pending_role_candidates);
    output.push('}');
}

fn push_unique_static_str(values: &mut Vec<&'static str>, value: &'static str) {
    if value != "none" && !values.contains(&value) {
        values.push(value);
    }
}

fn success_data_test_fdm_index_row_order_promotion_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmIndexRowOrderPromotionGate {
    let mut gate = SuccessDataTestFdmIndexRowOrderPromotionGate {
        command_count: classifications.len(),
        ..SuccessDataTestFdmIndexRowOrderPromotionGate::default()
    };

    for classification in classifications {
        for reference in &classification.index_row_references {
            gate.reference_count += 1;
            gate.referenced_command_relative_offsets
                .insert(classification.command.relative_offset());
            gate.referenced_row_indexes.insert(reference.row_index);
            gate.row_command_pairs
                .insert(SuccessDataTestFdmIndexRowCommandPair {
                    row_index: reference.row_index,
                    command_relative_offset: classification.command.relative_offset(),
                    match_kind: reference.match_kind,
                });
            gate.row_to_command_relative_offsets
                .entry(reference.row_index)
                .or_default()
                .insert(classification.command.relative_offset());
            if reference.valid_vector_offset {
                gate.valid_vector_offset_reference_count += 1;
            }
            match reference.match_kind {
                "command-relative-offset-field" => {
                    gate.command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    gate.source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
    }
    gate
}

fn push_success_data_test_fdm_index_row_order_promotion_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let render_promotion_blocked_reasons =
        success_data_test_fdm_index_row_order_promotion_blocked_reasons(classifications, &gate);
    let render_promotion_blocked_reason = render_promotion_blocked_reasons
        .first()
        .copied()
        .unwrap_or("none");
    output.push_str("{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, render_promotion_blocked_reason);
    output.push_str(",\"renderPromotionBlockedReasons\":");
    push_json_string_slice_array(output, &render_promotion_blocked_reasons);
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referencedCommandCount\":");
    output.push_str(&gate.referenced_command_count().to_string());
    output.push_str(",\"unreferencedCommandCount\":");
    output.push_str(&gate.unreferenced_command_count().to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&gate.unique_row_index_count().to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&gate.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"allCommandsReferencedByIndexRowsCandidate\":");
    output.push_str(if gate.all_commands_referenced_by_index_rows_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if gate.single_row_backs_multiple_commands_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowOrderMatchesCommandOrderCandidate\":");
    output.push_str(if gate.row_order_matches_command_order_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referencedCommandRelativeOffsets\":");
    push_usize_array_json(
        output,
        &gate
            .referenced_command_relative_offsets
            .iter()
            .copied()
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"referencedRowIndexes\":");
    push_usize_array_json(
        output,
        &gate
            .referenced_row_indexes
            .iter()
            .copied()
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"rowCommandPairs\":");
    push_success_data_test_fdm_index_row_command_pairs_json(output, &gate.row_command_pairs);
    output.push_str(
        ",\"renderPaintOrderBasisCandidate\":\"fdm-index-row-command-pairs\",\"renderPaintOrderBasisDecoded\":false",
    );
    output.push('}');
}

fn success_data_test_fdm_index_row_order_promotion_blocked_reasons(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
    gate: &SuccessDataTestFdmIndexRowOrderPromotionGate,
) -> Vec<&'static str> {
    let mut reasons = Vec::new();
    if !gate.all_commands_referenced_by_index_rows_candidate() {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-reference-coverage-incomplete",
        );
    }
    if !gate.one_to_one_row_command_reference_candidate() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-reference-not-one-to-one");
    }
    if gate.single_row_backs_multiple_commands_candidate() {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-single-row-backs-multiple-commands",
        );
    }
    if !gate.row_order_matches_command_order_candidate() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-non-monotonic");
    }
    if gate.reference_count > 0 && gate.valid_vector_offset_reference_count == 0 {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-valid-vector-offset-missing",
        );
    }
    if gate.command_relative_offset_field_reference_count > 0
        && gate.source_segment_relative_offset_field_reference_count > 0
    {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-offset-namespace-mixed");
    }

    let role_groups =
        success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);
    let mut role_paint_order_continuity_blocked = false;
    let mut role_paint_order_authority_pending = false;
    for group in role_groups.values() {
        let profile =
            success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
        role_paint_order_continuity_blocked |= profile.continuity_blocked();
        role_paint_order_authority_pending |= profile.paint_order_authority_pending();
    }
    if role_paint_order_continuity_blocked {
        push_unique_static_str(&mut reasons, "role-paint-order-continuity-unproven");
    }
    if role_paint_order_authority_pending {
        push_unique_static_str(&mut reasons, "role-paint-order-authority-unproven");
    }
    if reasons.is_empty() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-paint-authority-unproven");
    }
    reasons
}

#[derive(Debug, Default)]
struct SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
    role_candidate: &'static str,
    reference_count: usize,
    valid_vector_offset_reference_count: usize,
    valid_command_relative_offset_field_reference_count: usize,
    valid_source_segment_relative_offset_field_reference_count: usize,
    command_relative_offset_field_reference_count: usize,
    source_segment_relative_offset_field_reference_count: usize,
    command_relative_offsets: BTreeSet<usize>,
    row_indexes: BTreeSet<usize>,
    row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
}

#[derive(Debug)]
struct SuccessDataTestFdmRolePaintOrderContinuityProfile {
    span_min: Option<usize>,
    span_max: Option<usize>,
    role_command_count: usize,
    command_count_in_span: usize,
    interleaved_non_role_command_count: usize,
    max_command_offset_gap: usize,
    continuity_score: f32,
}

impl SuccessDataTestFdmRolePaintOrderContinuityProfile {
    fn span_contiguous_candidate(&self) -> bool {
        self.role_command_count > 0
            && self.command_count_in_span == self.role_command_count
            && self.interleaved_non_role_command_count == 0
    }

    fn continuity_blocked(&self) -> bool {
        !self.span_contiguous_candidate()
    }

    fn paint_order_authority_pending(&self) -> bool {
        self.span_contiguous_candidate()
    }

    fn render_promotion_blocked_reason(&self) -> &'static str {
        if self.continuity_blocked() {
            "role-span-interleaved-non-role-commands"
        } else {
            "role-paint-order-authority-unproven"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SuccessDataTestFdmIndexRowCommandPair {
    row_index: usize,
    command_relative_offset: usize,
    match_kind: &'static str,
}

fn success_data_test_fdm_index_row_reference_role_candidate_groups(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> BTreeMap<&'static str, SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup> {
    let mut groups =
        BTreeMap::<&'static str, SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup>::new();
    for classification in classifications {
        if classification.index_row_references.is_empty() {
            continue;
        }
        for role_candidate in &classification.role_candidates {
            let group = groups.entry(*role_candidate).or_insert_with(|| {
                SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
                    role_candidate,
                    ..SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup::default()
                }
            });
            group
                .command_relative_offsets
                .insert(classification.command.relative_offset());
            for reference in &classification.index_row_references {
                group.reference_count += 1;
                group.row_indexes.insert(reference.row_index);
                group
                    .row_command_pairs
                    .insert(SuccessDataTestFdmIndexRowCommandPair {
                        row_index: reference.row_index,
                        command_relative_offset: classification.command.relative_offset(),
                        match_kind: reference.match_kind,
                    });
                if reference.valid_vector_offset {
                    group.valid_vector_offset_reference_count += 1;
                    match reference.match_kind {
                        "command-relative-offset-field" => {
                            group.valid_command_relative_offset_field_reference_count += 1;
                        }
                        "source-segment-relative-offset-field" => {
                            group.valid_source_segment_relative_offset_field_reference_count += 1;
                        }
                        _ => {}
                    }
                }
                match reference.match_kind {
                    "command-relative-offset-field" => {
                        group.command_relative_offset_field_reference_count += 1;
                    }
                    "source-segment-relative-offset-field" => {
                        group.source_segment_relative_offset_field_reference_count += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    groups
}

fn success_data_test_fdm_role_group_single_row_backs_multiple_commands(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) -> bool {
    let mut row_to_command_count = BTreeMap::<usize, usize>::new();
    for pair in &group.row_command_pairs {
        *row_to_command_count.entry(pair.row_index).or_default() += 1;
    }
    row_to_command_count.values().any(|count| *count > 1)
}

fn push_success_data_test_fdm_index_row_reference_role_candidate_groups_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let groups = success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);

    output.push('[');
    for (index, group) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"roleCandidate\":");
        push_json_string(output, group.role_candidate);
        output.push_str(",\"ownershipProven\":false");
        output.push_str(
            ",\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\"",
        );
        output.push_str(",\"referenceCount\":");
        output.push_str(&group.reference_count.to_string());
        output.push_str(",\"validVectorOffsetReferenceCount\":");
        output.push_str(&group.valid_vector_offset_reference_count.to_string());
        output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
        output.push_str(
            &group
                .command_relative_offset_field_reference_count
                .to_string(),
        );
        output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
        output.push_str(
            &group
                .source_segment_relative_offset_field_reference_count
                .to_string(),
        );
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(
            output,
            &group
                .command_relative_offsets
                .iter()
                .copied()
                .collect::<Vec<_>>(),
        );
        output.push_str(",\"rowIndexes\":");
        push_usize_array_json(
            output,
            &group.row_indexes.iter().copied().collect::<Vec<_>>(),
        );
        output.push_str(",\"uniqueCommandRelativeOffsetCount\":");
        output.push_str(&group.command_relative_offsets.len().to_string());
        output.push_str(",\"uniqueRowIndexCount\":");
        output.push_str(&group.row_indexes.len().to_string());
        output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
        output.push_str(
            if group.reference_count == group.command_relative_offsets.len()
                && group.reference_count == group.row_indexes.len()
            {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
        output.push_str(
            if group.row_indexes.len() == 1 && group.command_relative_offsets.len() > 1 {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"rowOrderMatchesCommandOrderCandidate\":");
        output.push_str(
            if success_data_test_fdm_row_command_pairs_are_monotonic(&group.row_command_pairs) {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"rowCommandPairs\":");
        push_success_data_test_fdm_index_row_command_pairs_json(output, &group.row_command_pairs);
        output.push_str(",\"roleVectorOffsetAuthorityGate\":");
        push_success_data_test_fdm_role_vector_offset_authority_gate_json(output, group);
        output.push_str(",\"roleFanoutSegmentOwnerGate\":");
        push_success_data_test_fdm_role_fanout_segment_owner_gate_json(output, group);
        output.push_str(",\"decoded\":false,\"paintOrderContinuityProfile\":");
        push_success_data_test_fdm_role_paint_order_continuity_profile_json(
            output,
            group,
            classifications,
        );
        output.push('}');
    }
    output.push(']');
}

fn success_data_test_fdm_role_vector_offset_authority_blocked_reason(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) -> &'static str {
    let mixed_valid_offset_namespaces = group.valid_command_relative_offset_field_reference_count
        > 0
        && group.valid_source_segment_relative_offset_field_reference_count > 0;
    if group.valid_vector_offset_reference_count == 0 {
        "fdm-index-role-vector-offset-authority-valid-vector-offset-missing"
    } else if mixed_valid_offset_namespaces {
        "fdm-index-role-vector-offset-authority-mixed-valid-offset-namespaces"
    } else {
        "fdm-index-role-vector-offset-authority-semantics-unproven"
    }
}

fn push_success_data_test_fdm_role_vector_offset_authority_gate_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) {
    let invalid_vector_offset_reference_count = group
        .reference_count
        .saturating_sub(group.valid_vector_offset_reference_count);
    let invalid_command_relative_offset_field_reference_count = group
        .command_relative_offset_field_reference_count
        .saturating_sub(group.valid_command_relative_offset_field_reference_count);
    let invalid_source_segment_relative_offset_field_reference_count = group
        .source_segment_relative_offset_field_reference_count
        .saturating_sub(group.valid_source_segment_relative_offset_field_reference_count);
    let mixed_offset_namespaces_among_valid_refs =
        group.valid_command_relative_offset_field_reference_count > 0
            && group.valid_source_segment_relative_offset_field_reference_count > 0;
    let all_valid_references_use_command_relative_offset_field =
        group.valid_vector_offset_reference_count > 0
            && group.valid_command_relative_offset_field_reference_count
                == group.valid_vector_offset_reference_count;
    let all_valid_references_use_source_segment_relative_offset_field =
        group.valid_vector_offset_reference_count > 0
            && group.valid_source_segment_relative_offset_field_reference_count
                == group.valid_vector_offset_reference_count;
    let all_references_have_invalid_vector_offset =
        group.reference_count > 0 && group.valid_vector_offset_reference_count == 0;
    let render_promotion_blocked_reason =
        success_data_test_fdm_role_vector_offset_authority_blocked_reason(group);

    output.push_str("{\"basis\":\"fdm-index-role-vector-offset-authority-gate\",\"source\":\"FDMIndex.vectorOffset+FDMIndex role offset fields\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"roleCandidate\":");
    push_json_string(output, group.role_candidate);
    output.push_str(",\"roleVectorOffsetAuthorityDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"fdm-index-role-vector-offset-authority-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, render_promotion_blocked_reason);
    output.push_str(",\"referenceCount\":");
    output.push_str(&group.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&group.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"invalidVectorOffsetReferenceCount\":");
    output.push_str(&invalid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"validCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .valid_command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"validSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .valid_source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"invalidCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(&invalid_command_relative_offset_field_reference_count.to_string());
    output.push_str(",\"invalidSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(&invalid_source_segment_relative_offset_field_reference_count.to_string());
    output.push_str(",\"allValidReferencesUseCommandRelativeOffsetField\":");
    output.push_str(if all_valid_references_use_command_relative_offset_field {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allValidReferencesUseSourceSegmentRelativeOffsetField\":");
    output.push_str(
        if all_valid_references_use_source_segment_relative_offset_field {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"mixedOffsetNamespacesAmongValidReferences\":");
    output.push_str(if mixed_offset_namespaces_among_valid_refs {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesHaveInvalidVectorOffset\":");
    output.push_str(if all_references_have_invalid_vector_offset {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

fn push_success_data_test_fdm_role_fanout_segment_owner_gate_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) {
    let mut row_to_pairs = BTreeMap::<usize, Vec<SuccessDataTestFdmIndexRowCommandPair>>::new();
    for pair in &group.row_command_pairs {
        row_to_pairs.entry(pair.row_index).or_default().push(*pair);
    }

    let mut fanout_row_count = 0usize;
    let mut fanout_reference_count = 0usize;
    let mut fanout_command_relative_offset_field_reference_count = 0usize;
    let mut fanout_source_segment_relative_offset_field_reference_count = 0usize;
    let mut max_row_fanout = 0usize;
    for pairs in row_to_pairs.values() {
        max_row_fanout = max_row_fanout.max(pairs.len());
        if pairs.len() <= 1 {
            continue;
        }
        fanout_row_count += 1;
        fanout_reference_count += pairs.len();
        for pair in pairs {
            match pair.match_kind {
                "command-relative-offset-field" => {
                    fanout_command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    fanout_source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
    }

    let one_to_one_row_command_reference_candidate = group.reference_count
        == group.command_relative_offsets.len()
        && group.reference_count == group.row_indexes.len();
    let single_row_backs_multiple_commands_candidate =
        row_to_pairs.values().any(|pairs| pairs.len() > 1);
    let mixed_offset_field_namespaces = group.command_relative_offset_field_reference_count > 0
        && group.source_segment_relative_offset_field_reference_count > 0;
    let fanout_rows_use_command_relative_offset_fields = fanout_reference_count > 0
        && fanout_command_relative_offset_field_reference_count == fanout_reference_count;
    let fanout_rows_use_source_segment_offset_fields = fanout_reference_count > 0
        && fanout_source_segment_relative_offset_field_reference_count == fanout_reference_count;
    let render_promotion_blocked_reason = if single_row_backs_multiple_commands_candidate {
        "fdm-index-role-row-fanout-multi-command-single-row"
    } else if !one_to_one_row_command_reference_candidate {
        "fdm-index-role-row-reference-not-one-to-one"
    } else if mixed_offset_field_namespaces {
        "fdm-index-role-offset-namespace-mixed"
    } else if group.valid_vector_offset_reference_count == 0 {
        "fdm-index-role-valid-vector-offset-missing"
    } else {
        "fdm-index-role-segment-owner-semantics-unproven"
    };

    output.push_str("{\"basis\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex role row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"roleCandidate\":");
    push_json_string(output, group.role_candidate);
    output.push_str(",\"roleOwnershipDecoded\":false,\"segmentOwnerDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"fdm-index-role-row-fanout-segment-owner-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, render_promotion_blocked_reason);
    output.push_str(",\"referenceCount\":");
    output.push_str(&group.reference_count.to_string());
    output.push_str(",\"uniqueCommandRelativeOffsetCount\":");
    output.push_str(&group.command_relative_offsets.len().to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&group.row_indexes.len().to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutRowCount\":");
    output.push_str(&fanout_row_count.to_string());
    output.push_str(",\"fanoutReferenceCount\":");
    output.push_str(&fanout_reference_count.to_string());
    output.push_str(",\"fanoutCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(&fanout_command_relative_offset_field_reference_count.to_string());
    output.push_str(",\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(&fanout_source_segment_relative_offset_field_reference_count.to_string());
    output.push_str(",\"maxRowFanout\":");
    output.push_str(&max_row_fanout.to_string());
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if single_row_backs_multiple_commands_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseCommandRelativeOffsetFields\":");
    output.push_str(if fanout_rows_use_command_relative_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseSourceSegmentOffsetFields\":");
    output.push_str(if fanout_rows_use_source_segment_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowsWithMultipleCommandRefs\":");
    push_success_data_test_fdm_role_fanout_rows_json(output, &row_to_pairs);
    output.push('}');
}

fn push_success_data_test_fdm_role_fanout_rows_json(
    output: &mut String,
    row_to_pairs: &BTreeMap<usize, Vec<SuccessDataTestFdmIndexRowCommandPair>>,
) {
    output.push('[');
    let mut emitted = 0usize;
    for (row_index, pairs) in row_to_pairs {
        if pairs.len() <= 1 {
            continue;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        let command_relative_offsets = pairs
            .iter()
            .map(|pair| pair.command_relative_offset)
            .collect::<Vec<_>>();
        let match_kinds = pairs
            .iter()
            .map(|pair| pair.match_kind)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        output.push_str("{\"rowIndex\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"commandReferenceCount\":");
        output.push_str(&pairs.len().to_string());
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(output, &command_relative_offsets);
        output.push_str(",\"matchKinds\":");
        push_json_string_slice_array(output, &match_kinds);
        output.push('}');
    }
    output.push(']');
}

fn push_success_data_test_fdm_role_paint_order_continuity_profile_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    output.push_str("{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false");
    let profile = success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
    output.push_str(",\"commandRelativeOffsetSpanMin\":");
    push_option_usize_json(output, profile.span_min);
    output.push_str(",\"commandRelativeOffsetSpanMax\":");
    push_option_usize_json(output, profile.span_max);
    output.push_str(",\"roleCommandCount\":");
    output.push_str(&profile.role_command_count.to_string());
    output.push_str(",\"commandCountInSpan\":");
    output.push_str(&profile.command_count_in_span.to_string());
    output.push_str(",\"interleavedNonRoleCommandCount\":");
    output.push_str(&profile.interleaved_non_role_command_count.to_string());
    output.push_str(",\"hasInterleavedNonRoleCommands\":");
    output.push_str(if profile.interleaved_non_role_command_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"maxCommandOffsetGap\":");
    output.push_str(&profile.max_command_offset_gap.to_string());
    output.push_str(",\"commandOffsetContinuityScore\":");
    output.push_str(&format!("{:.3}", profile.continuity_score));
    output.push_str(",\"spanContiguousCandidate\":");
    output.push_str(if profile.span_contiguous_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintOrderAuthorityPending\":");
    output.push_str(if profile.paint_order_authority_pending() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"continuityBlocked\":");
    output.push_str(if profile.continuity_blocked() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    push_json_string(output, profile.render_promotion_blocked_reason());
    output.push('}');
}

fn success_data_test_fdm_role_paint_order_continuity_profile(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmRolePaintOrderContinuityProfile {
    let span_min = group.command_relative_offsets.iter().next().copied();
    let span_max = group.command_relative_offsets.iter().next_back().copied();
    let role_command_count = group.command_relative_offsets.len();
    let command_count_in_span = match (span_min, span_max) {
        (Some(min), Some(max)) => classifications
            .iter()
            .filter(|classification| {
                let offset = classification.command.relative_offset();
                offset >= min && offset <= max
            })
            .count(),
        _ => 0,
    };
    let interleaved_non_role_command_count =
        command_count_in_span.saturating_sub(role_command_count);
    let mut max_command_offset_gap = 0usize;
    let mut previous_offset = None;
    for offset in group.command_relative_offsets.iter().copied() {
        if let Some(previous) = previous_offset {
            max_command_offset_gap = max_command_offset_gap.max(offset.saturating_sub(previous));
        }
        previous_offset = Some(offset);
    }
    let continuity_score = if command_count_in_span == 0 {
        0.0
    } else {
        role_command_count as f32 / command_count_in_span as f32
    };

    SuccessDataTestFdmRolePaintOrderContinuityProfile {
        span_min,
        span_max,
        role_command_count,
        command_count_in_span,
        interleaved_non_role_command_count,
        max_command_offset_gap,
        continuity_score,
    }
}

fn success_data_test_fdm_row_command_pairs_are_monotonic(
    pairs: &BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
) -> bool {
    let mut previous_command_relative_offset = None;
    for pair in pairs {
        if previous_command_relative_offset
            .is_some_and(|previous| pair.command_relative_offset < previous)
        {
            return false;
        }
        previous_command_relative_offset = Some(pair.command_relative_offset);
    }
    true
}

fn push_success_data_test_fdm_index_row_command_pairs_json(
    output: &mut String,
    pairs: &BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
) {
    output.push('[');
    for (index, pair) in pairs.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&pair.row_index.to_string());
        output.push_str(",\"commandRelativeOffset\":");
        output.push_str(&pair.command_relative_offset.to_string());
        output.push_str(",\"matchKind\":");
        push_json_string(output, pair.match_kind);
        output.push('}');
    }
    output.push(']');
}

fn success_data_test_fdm_primitive_ownership_classification<'a>(
    projection: SuccessDataTestFdmProjection,
    command: &'a ObjectFdmVectorCommandCandidate,
    index_entries: &[ObjectFdmIndexEntryCandidate],
    anchor: Option<(ObjectFdmVectorPoint, i32)>,
) -> SuccessDataTestFdmPrimitiveOwnershipClassification<'a> {
    let mut role_candidates = Vec::new();
    let mut classification_basis = Vec::new();
    if let Some(ellipse) = command.ellipse() {
        if success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse) {
            role_candidates.push("main-circle-anchor");
            classification_basis.push("large-01000460-ellipse-anchor");
        } else if success_data_test_fdm_reference_ellipse_is_control_marker(
            projection, command, ellipse,
        ) {
            role_candidates.push("arc-candidate");
            role_candidates.push("control-ellipse-marker");
            classification_basis.push("tiny-ff000460-ellipse-control-marker");
        } else {
            role_candidates.push("arc-candidate");
            classification_basis.push("ellipse-boundary-primitive");
        }
    } else {
        let is_two_point_line = fdm_vector_marker_is_line(command.marker())
            && command.curve_segments().is_empty()
            && command.path_points().len() == 2;
        if is_two_point_line {
            role_candidates.push("line-candidate");
            classification_basis.push("fdm-line-marker-two-point-path");
            if let Some((center, radius)) = anchor {
                let boundary_count =
                    success_data_test_fdm_anchor_boundary_point_count(command, center, radius);
                let center_count =
                    success_data_test_fdm_anchor_center_point_count(command, center, radius);
                if boundary_count >= 2 {
                    role_candidates.push("chord-candidate");
                    classification_basis.push("both-endpoints-near-anchor-boundary");
                } else if boundary_count >= 1 && center_count >= 1 {
                    role_candidates.push("radial-line-candidate");
                    classification_basis.push("one-endpoint-near-anchor-center-one-near-boundary");
                }
            }
        }
        if !command.curve_segments().is_empty()
            || fdm_vector_marker_is_bezier_curve(command.marker())
        {
            role_candidates.push("arc-candidate");
            classification_basis.push("fdm-bezier-marker-or-control-points");
        }
        if command.path_points().len() >= 3 && !fdm_vector_path_is_closed(command.path_points()) {
            role_candidates.push("surface-boundary-candidate");
            classification_basis.push("open-polyline-with-three-or-more-points");
        }
        if success_data_test_fdm_connector_candidate(command) {
            role_candidates.push("connector-candidate");
            classification_basis.push("long-open-source-path");
        }
    }
    if role_candidates.is_empty() {
        role_candidates.push("unclassified-primitive");
        classification_basis.push("no-current-role-rule");
    }
    SuccessDataTestFdmPrimitiveOwnershipClassification {
        command,
        role_candidates,
        classification_basis,
        index_row_references: success_data_test_fdm_index_row_references(command, index_entries),
    }
}

fn success_data_test_fdm_index_row_references(
    command: &ObjectFdmVectorCommandCandidate,
    index_entries: &[ObjectFdmIndexEntryCandidate],
) -> Vec<SuccessDataTestFdmIndexRowReference> {
    let mut references = Vec::new();
    for entry in index_entries {
        let bbox = entry.bbox();
        let offset_value = bbox.left();
        if offset_value < 0 {
            continue;
        }
        let offset_value = offset_value as usize;
        let match_kind = if offset_value == command.relative_offset() {
            Some("command-relative-offset-field")
        } else if command
            .source_segment()
            .is_some_and(|segment| segment.relative_offset() == offset_value)
        {
            Some("source-segment-relative-offset-field")
        } else {
            None
        };
        let Some(match_kind) = match_kind else {
            continue;
        };
        references.push(SuccessDataTestFdmIndexRowReference {
            row_index: entry.row_index(),
            index_offset: entry.index_offset(),
            vector_offset: entry.vector_offset(),
            valid_vector_offset: entry.valid_vector_offset(),
            offset_field: "bbox.left",
            offset_value,
            match_kind,
        });
    }
    references
}

fn push_success_data_test_fdm_index_row_references_json(
    output: &mut String,
    references: &[SuccessDataTestFdmIndexRowReference],
) {
    output.push('[');
    for (index, reference) in references.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&reference.row_index.to_string());
        output.push_str(",\"indexOffset\":");
        output.push_str(&reference.index_offset.to_string());
        output.push_str(",\"vectorOffset\":");
        output.push_str(&reference.vector_offset.to_string());
        output.push_str(",\"validVectorOffset\":");
        output.push_str(if reference.valid_vector_offset {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"offsetField\":");
        push_json_string(output, reference.offset_field);
        output.push_str(",\"offsetValue\":");
        output.push_str(&reference.offset_value.to_string());
        output.push_str(",\"matchKind\":");
        push_json_string(output, reference.match_kind);
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

fn success_data_test_fdm_anchor_boundary_point_count(
    command: &ObjectFdmVectorCommandCandidate,
    center: ObjectFdmVectorPoint,
    radius: i32,
) -> usize {
    let tolerance = (radius / 12).max(24) as f32;
    command
        .path_points()
        .iter()
        .filter(|point| (fdm_point_distance(center, **point) - radius as f32).abs() <= tolerance)
        .count()
}

fn success_data_test_fdm_anchor_center_point_count(
    command: &ObjectFdmVectorCommandCandidate,
    center: ObjectFdmVectorPoint,
    radius: i32,
) -> usize {
    let tolerance = (radius / 8).max(24) as f32;
    command
        .path_points()
        .iter()
        .filter(|point| fdm_point_distance(center, **point) <= tolerance)
        .count()
}

fn success_data_test_fdm_connector_candidate(command: &ObjectFdmVectorCommandCandidate) -> bool {
    if command.ellipse().is_some() || fdm_vector_path_is_closed(command.path_points()) {
        return false;
    }
    let Some(bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let source_width = bbox.2.saturating_sub(bbox.0);
    let source_height = bbox.3.saturating_sub(bbox.1);
    source_width.max(source_height) >= 500
}

fn fdm_point_distance(left: ObjectFdmVectorPoint, right: ObjectFdmVectorPoint) -> f32 {
    let dx = (left.x() - right.x()) as f32;
    let dy = (left.y() - right.y()) as f32;
    (dx * dx + dy * dy).sqrt()
}

pub(crate) fn push_json_string_slice_array(output: &mut String, values: &[&str]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, value);
    }
    output.push(']');
}

fn fdm_vector_command_source_bbox(
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
        return fdm_vector_path_points_bbox(&points);
    }
    command.ellipse().map(fdm_vector_ellipse_bbox)
}

fn fdm_vector_ellipse_bbox(ellipse: ObjectFdmVectorEllipse) -> ObjectFdmIndexBbox {
    let center = ellipse.center();
    ObjectFdmIndexBbox::new(
        center.x().saturating_sub(ellipse.radius_x()),
        center.y().saturating_sub(ellipse.radius_y()),
        center.x().saturating_add(ellipse.radius_x()),
        center.y().saturating_add(ellipse.radius_y()),
    )
}

fn normalize_fdm_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.right()),
        bbox.top().min(bbox.bottom()),
        bbox.left().max(bbox.right()),
        bbox.top().max(bbox.bottom()),
    )
}

pub(crate) fn fdm_bbox_center(bbox: (i32, i32, i32, i32)) -> (i32, i32) {
    let center_x = i64::from(bbox.0) + (i64::from(bbox.2) - i64::from(bbox.0)) / 2;
    let center_y = i64::from(bbox.1) + (i64::from(bbox.3) - i64::from(bbox.1)) / 2;
    (center_x as i32, center_y as i32)
}

fn push_fdm_vector_points_json(output: &mut String, points: &[ObjectFdmVectorPoint]) {
    output.push('[');
    for (index, point) in points.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_vector_point_json(output, point);
    }
    output.push(']');
}

fn push_fdm_vector_point_json(output: &mut String, point: ObjectFdmVectorPoint) {
    output.push_str("{\"x\":");
    output.push_str(&point.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&point.y().to_string());
    output.push('}');
}

fn push_fdm_vector_curve_segments_json(
    output: &mut String,
    segments: &[ObjectFdmVectorCurveSegment],
) {
    output.push('[');
    for (index, segment) in segments.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"control1\":");
        push_fdm_vector_point_json(output, segment.control_1());
        output.push_str(",\"control2\":");
        push_fdm_vector_point_json(output, segment.control_2());
        output.push('}');
    }
    output.push(']');
}

fn push_fdm_vector_ellipse_json(output: &mut String, ellipse: ObjectFdmVectorEllipse) {
    output.push_str("{\"center\":");
    push_fdm_vector_point_json(output, ellipse.center());
    output.push_str(",\"radiusX\":");
    output.push_str(&ellipse.radius_x().to_string());
    output.push_str(",\"radiusY\":");
    output.push_str(&ellipse.radius_y().to_string());
    output.push_str(",\"color\":");
    push_fdm_vector_optional_color_json(output, ellipse.color());
    output.push('}');
}

fn push_fdm_vector_optional_color_json(output: &mut String, color: Option<u32>) {
    match color.and_then(fdm_vector_css_color) {
        Some(color) => push_json_string(output, &color),
        None => output.push_str("null"),
    }
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

pub(crate) fn push_object_fdm_text_candidate_json(
    output: &mut String,
    candidate: &ObjectFdmTextCandidate,
) {
    output.push_str("{\"text\":");
    push_json_string(output, candidate.text());
    output.push_str(",\"textOffset\":");
    output.push_str(&candidate.text_offset().to_string());
    output.push_str(",\"markerOffset\":");
    output.push_str(&candidate.marker_offset().to_string());
    output.push_str(",\"rawTextHex\":");
    push_json_string(output, &hex(candidate.raw_text()));
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
    push_json_string(output, candidate.index_path());
    output.push_str(",\"textPath\":");
    push_json_string(output, candidate.text_path());
    output.push_str(",\"rowIndex\":");
    output.push_str(&candidate.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&candidate.index_offset().to_string());
    output.push_str(",\"textRecordOffset\":");
    output.push_str(&candidate.text_record_offset().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&candidate.kind().to_string());
    output.push_str(",\"kindHex\":");
    push_json_string(output, &format!("0x{:04x}", candidate.kind()));
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
    push_json_string(output, &hex(candidate.text_record_prefix()));
    output.push_str(",\"decoded\":false}");
}

fn fdm_vector_path_points_bbox(points: &[ObjectFdmVectorPoint]) -> Option<ObjectFdmIndexBbox> {
    let first = *points.first()?;
    let mut left = first.x();
    let mut top = first.y();
    let mut right = first.x();
    let mut bottom = first.y();

    for point in points.iter().copied().skip(1) {
        left = left.min(point.x());
        top = top.min(point.y());
        right = right.max(point.x());
        bottom = bottom.max(point.y());
    }

    Some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

fn fdm_vector_path_is_closed(points: &[ObjectFdmVectorPoint]) -> bool {
    points.len() >= 2 && points.first() == points.last()
}

fn fdm_vector_primitive_kind(command: &ObjectFdmVectorCommandCandidate) -> &'static str {
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

fn fdm_vector_marker_is_bezier_curve(marker: &[u8; 4]) -> bool {
    marker == b"\xff\x00\x09\x60" || marker == b"\x00\x00\x09\x60" || marker == b"\x01\x00\x09\x60"
}

fn fdm_vector_marker_is_line(marker: &[u8; 4]) -> bool {
    marker == b"\xff\x00\x01\x60" || marker == b"\x00\x00\x01\x60" || marker == b"\x01\x00\x01\x60"
}

fn fdm_vector_css_color(color: u32) -> Option<String> {
    if color > 0x00ff_ffff {
        return None;
    }
    let blue = (color >> 16) & 0xff;
    let green = (color >> 8) & 0xff;
    let red = color & 0xff;
    Some(format!("#{red:02x}{green:02x}{blue:02x}"))
}

fn push_object_fdm_index_bbox_json(output: &mut String, bbox: ObjectFdmIndexBbox) {
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
