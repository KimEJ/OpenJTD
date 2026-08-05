use super::*;
use crate::*;
use std::{
    collections::HashSet,
    io::{Cursor, Write},
};

pub(super) fn running_header_svg_element(svg: &str) -> &str {
    let start = svg.find("<text class=\"rjtd-running-header\"").unwrap();
    let tail = &svg[start..];
    let end = tail.find("</text>").unwrap() + "</text>".len();
    &tail[..end]
}

pub(super) fn assert_json_brackets_balanced(json: &str) {
    let mut stack = Vec::new();
    let mut in_string = false;
    let mut escaped = false;

    for (offset, byte) in json.bytes().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            match byte {
                b'\\' => escaped = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => stack.push(byte),
            b'}' => assert_eq!(stack.pop(), Some(b'{'), "unmatched }} at byte {offset}"),
            b']' => assert_eq!(stack.pop(), Some(b'['), "unmatched ] at byte {offset}"),
            _ => {}
        }
    }

    assert!(!in_string, "unterminated JSON string");
    assert!(stack.is_empty(), "unclosed JSON delimiters: {stack:?}");
}

pub(super) fn test_json_string_array(values: &[&str]) -> String {
    let mut output = String::new();
    push_json_string_slice_array(&mut output, values);
    output
}

pub(super) fn tail_after_occurrence<'a>(
    haystack: &'a str,
    marker: &str,
    occurrence: usize,
) -> &'a str {
    let mut tail = haystack;
    for index in 0..=occurrence {
        let Some((_, next_tail)) = tail.split_once(marker) else {
            panic!("missing JSON marker occurrence {index} for {marker}");
        };
        tail = next_tail;
    }
    tail
}

pub(super) fn assert_json_string_field_after(
    haystack: &str,
    marker: &str,
    occurrence: usize,
    field: &str,
    expected: &str,
) {
    let fragment = format!("\"{field}\":{}", json_string(expected));
    let tail = tail_after_occurrence(haystack, marker, occurrence);
    assert!(
        tail.contains(&fragment),
        "missing JSON field {field}={expected:?} after marker {marker}"
    );
}

pub(super) fn assert_json_number_field_after(
    haystack: &str,
    marker: &str,
    occurrence: usize,
    field: &str,
    expected: &str,
) {
    let fragment = format!("\"{field}\":{expected}");
    let tail = tail_after_occurrence(haystack, marker, occurrence);
    assert!(
        tail.contains(&fragment),
        "missing JSON field {field}={expected} after marker {marker}"
    );
}

pub(super) fn assert_json_bool_field_after(
    haystack: &str,
    marker: &str,
    occurrence: usize,
    field: &str,
    expected: bool,
) {
    let fragment = format!("\"{field}\":{}", if expected { "true" } else { "false" });
    let tail = tail_after_occurrence(haystack, marker, occurrence);
    assert!(
        tail.contains(&fragment),
        "missing JSON field {field}={expected} after marker {marker}"
    );
}

pub(super) fn assert_json_string_array_field_after(
    haystack: &str,
    marker: &str,
    occurrence: usize,
    field: &str,
    expected: &[&str],
) {
    let fragment = format!("\"{field}\":{}", test_json_string_array(expected));
    let tail = tail_after_occurrence(haystack, marker, occurrence);
    assert!(
        tail.contains(&fragment),
        "missing JSON string array field {field}={expected:?} after marker {marker}"
    );
}

#[test]
fn title_art_shadow_sweep_keeps_evenodd_inner_boundaries() {
    let mut commands = vec![ObjectEmbeddedPressVectorPathCommandCandidate::MoveTo { x: 0, y: 0 }];
    push_embedded_press_test_line_to(&mut commands, (0, 0), (100, 0));
    push_embedded_press_test_line_to(&mut commands, (100, 0), (100, 100));
    push_embedded_press_test_line_to(&mut commands, (100, 100), (0, 100));
    push_embedded_press_test_line_to(&mut commands, (0, 100), (0, 0));
    commands.push(ObjectEmbeddedPressVectorPathCommandCandidate::Close);
    commands.push(ObjectEmbeddedPressVectorPathCommandCandidate::MoveTo { x: 10, y: 10 });
    push_embedded_press_test_line_to(&mut commands, (10, 10), (10, 12));
    push_embedded_press_test_line_to(&mut commands, (10, 12), (12, 12));
    push_embedded_press_test_line_to(&mut commands, (12, 12), (12, 10));
    push_embedded_press_test_line_to(&mut commands, (12, 10), (10, 10));
    commands.push(ObjectEmbeddedPressVectorPathCommandCandidate::Close);

    let path = embedded_press_test_outline_path(commands);
    let contours = embedded_press_vector_path_evenodd_boundary_contours(&path, 1);
    assert_eq!(contours.len(), 2);
    let signed_areas = contours
        .iter()
        .map(|contour| embedded_press_sampled_contour_signed_area(contour))
        .collect::<Vec<_>>();
    assert!(signed_areas.iter().any(|area| *area > 0.0));
    assert!(signed_areas.iter().any(|area| *area < 0.0));
    let mut absolute_areas = signed_areas
        .iter()
        .map(|area| area.abs())
        .collect::<Vec<_>>();
    absolute_areas.sort_by(|a, b| a.total_cmp(b));
    assert!(absolute_areas[0] / absolute_areas[1] < 0.01);

    let sweep =
        success_data_test_title_art_shadow_sweep_path_data(&[&path], (10, 10), 0.0, 0.0, 1.0, 1.0)
            .expect("non-degenerate evenodd contours should produce side strips");
    assert_eq!(
        sweep.matches(" Z ").count(),
        8 * SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES
    );
}

#[test]
fn success_data_test_cone_projection_requires_text_corroboration() {
    let segments = vec![test_fdm_vector_segment(
        ObjectFdmIndexBbox::new(-12000, -12000, -10800, -10600),
        1200,
        1400,
    )];
    let unrelated_text = vec![test_fdm_text_candidate(
        "outside",
        ObjectFdmIndexBbox::new(0, 0, 100, 100),
    )];
    let single_matching_text = vec![test_fdm_text_candidate(
        "9cm",
        ObjectFdmIndexBbox::new(-11900, -11900, -11800, -11800),
    )];

    assert!(success_data_test_cone_fdm_projection_from_segments(&segments, &[]).is_none());
    assert!(
        success_data_test_cone_fdm_projection_from_segments(&segments, &unrelated_text).is_none()
    );
    assert!(
        success_data_test_cone_fdm_projection_from_segments(&segments, &single_matching_text)
            .is_none()
    );
}

#[test]
fn success_data_test_cone_projection_uses_matching_text_corroboration() {
    let segments = vec![test_fdm_vector_segment(
        ObjectFdmIndexBbox::new(-12000, -12000, -10800, -10600),
        1200,
        1400,
    )];
    let text_candidates = vec![
        test_fdm_text_candidate(
            "9cm",
            ObjectFdmIndexBbox::new(-11900, -11900, -11800, -11800),
        ),
        test_fdm_text_candidate(
            "3cm",
            ObjectFdmIndexBbox::new(-11000, -10800, -10900, -10700),
        ),
        test_fdm_text_candidate("outside", ObjectFdmIndexBbox::new(0, 0, 100, 100)),
    ];

    let projection =
        success_data_test_cone_fdm_projection_from_segments(&segments, &text_candidates)
            .expect("two source text bboxes should corroborate the cone vector span");

    assert_eq!(projection.role, "q3-cone-diagram");
    assert_eq!(projection.text_corroboration_count, 2);
    assert_eq!(projection.source_left, -12000);
    assert_eq!(projection.source_top, -12000);
    assert_eq!(projection.source_right, -10800);
    assert_eq!(projection.source_bottom, -10600);
}

#[test]
fn max_abs_i32_handles_i32_min_conservatively() {
    assert_eq!(max_abs_i32(&[i32::MIN]), Some(i32::MAX));
    assert_eq!(max_abs_i32(&[i32::MIN, -4, 6]), Some(i32::MAX));
}

#[test]
fn signed_usize_delta_i32_saturates_at_signed_bounds() {
    let signed_max = i32::MAX as usize;
    let signed_overflow = signed_max + 1;

    assert_eq!(signed_usize_delta_i32(0, 0), 0);
    assert_eq!(signed_usize_delta_i32(signed_max, 0), i32::MAX);
    assert_eq!(signed_usize_delta_i32(0, signed_max), -i32::MAX);
    assert_eq!(signed_usize_delta_i32(signed_overflow, 0), i32::MAX);
    assert_eq!(signed_usize_delta_i32(0, signed_overflow), i32::MIN);
    assert_eq!(signed_usize_delta_i32(usize::MAX, 0), i32::MAX);
    assert_eq!(signed_usize_delta_i32(0, usize::MAX), i32::MIN);
}

#[test]
fn source_gap_readiness_rejects_oversized_source_range_false_zero_transform_authority() {
    let oversized_source_range_gap = i32::MAX as usize + 1;
    let probe = test_table_grid_cross_table_row_boundary_offset_probe(
        vec![
            test_table_grid_cross_table_row_boundary_offset_table(
                0,
                0,
                0,
                vec![0],
                vec![0],
                vec![0],
            ),
            test_table_grid_cross_table_row_boundary_offset_table(
                1,
                oversized_source_range_gap,
                oversized_source_range_gap,
                vec![i32::MAX as usize],
                vec![i32::MAX as usize],
                vec![0],
            ),
        ],
        vec![
            test_table_grid_source_unit_to_page_line_index_piecewise_transition(
                oversized_source_range_gap,
                0,
                i32::MAX,
            ),
        ],
    );

    let hints = table_grid_source_gap_to_page_line_gap_readiness_hints(Some(&probe));

    let mut output = String::new();
    push_table_grid_source_only_page_y_transition_semantics_readiness_json(
        &mut output,
        Some(&probe),
        1,
    );
    let mut admission_output = String::new();
    push_table_grid_source_gap_to_page_line_gap_transform_admission_gate_json(
        &mut admission_output,
        "test",
        &hints,
    );

    assert_eq!(
        (
            hints.source_range_gap_to_page_line_gap_max_abs_delta_units,
            hints.source_gap_to_page_line_gap_transform_stable(),
            hints.table_family_source_gap_to_page_line_gap_transform_stable(),
            output.contains("\"sourceGapToPageLineGapTransformStable\":true"),
            output.contains("\"tableFamilySourceGapToPageLineGapTransformStable\":true"),
            admission_output.contains("\"canDecodeSourceTransform\":true"),
        ),
        (Some(i32::MAX), false, false, false, false, false)
    );
    assert!(output.contains("\"sourceRangeGapMinusPageLineGapUnits\":[2147483647]"));
    assert!(output.contains("\"segmentOffsetGapUnits\":[-2147483648]"));
    assert!(output.contains("\"segmentOffsetGapMinusPageLineGapUnits\":[-2147483648]"));
    assert!(output.contains("\"sourceGapToPageLineGapTransformStable\":false"));
    assert!(output.contains("\"tableFamilySourceGapToPageLineGapTransformStable\":false"));
    assert!(!output.contains("\"canDecodeSourceTransform\":true"));
}

pub(super) fn paper_mark_fixture(entries: &[(u32, u32)]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&u32::try_from(entries.len()).unwrap().to_be_bytes());
    bytes.extend_from_slice(&0x0cu32.to_be_bytes());
    bytes.extend_from_slice(
        &entries
            .last()
            .map(|(index, _)| *index)
            .unwrap_or(0)
            .to_be_bytes(),
    );
    for (index, flags) in entries {
        bytes.extend_from_slice(&index.to_be_bytes());
        bytes.extend_from_slice(&flags.to_be_bytes());
    }
    bytes
}

pub(super) fn line_mark_words_0_to_20() -> Vec<u8> {
    let mut bytes = Vec::new();
    for word in 0..20u16 {
        bytes.extend_from_slice(&word.to_be_bytes());
    }
    bytes
}

pub(super) fn page_mark_fields_0_to_20() -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&19u32.to_be_bytes());
    bytes.extend_from_slice(&0x10u32.to_be_bytes());
    bytes.extend_from_slice(&18u32.to_be_bytes());
    for index in 0..20u32 {
        let mut entry = [0; 84];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        bytes.extend_from_slice(&entry);
    }
    bytes
}

pub(super) fn row_header_record_bytes(fixed_words: [u16; 6], payload_words: &[u16]) -> Vec<u8> {
    let total_len_words = (3 + fixed_words.len() + payload_words.len() + 4) as u16;
    let mut words = vec![0x001c, 0x0010, total_len_words];
    words.extend_from_slice(&fixed_words);
    words.extend_from_slice(payload_words);
    words.extend_from_slice(&[total_len_words, 0x0000, 0x0010, 0x001f]);
    let mut bytes = Vec::new();
    extend_units(&mut bytes, &words);
    bytes
}

pub(super) fn extend_units(bytes: &mut Vec<u8>, units: &[u16]) {
    for unit in units {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
}

pub(super) fn cfb_with_streams(streams: &[(&str, &[u8])]) -> Vec<u8> {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut storages = HashSet::new();
    for (path, payload) in streams {
        create_parent_storages(&mut compound, path, &mut storages);
        compound
            .create_stream(path)
            .unwrap()
            .write_all(payload)
            .unwrap();
    }
    compound.into_inner().into_inner()
}

pub(super) fn create_parent_storages(
    compound: &mut cfb::CompoundFile<Cursor<Vec<u8>>>,
    path: &str,
    storages: &mut HashSet<String>,
) {
    let segments = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.len() <= 1 {
        return;
    }

    let mut current = String::new();
    for segment in &segments[..segments.len() - 1] {
        current.push('/');
        current.push_str(segment);
        if storages.insert(current.clone()) {
            compound.create_storage(&current).unwrap();
        }
    }
}
