use super::*;
use rjtd_core::font_stream::FONT_STREAM_PATH;
use std::{
    collections::HashSet,
    fs,
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

fn running_header_svg_element(svg: &str) -> &str {
    let start = svg.find("<text class=\"rjtd-running-header\"").unwrap();
    let tail = &svg[start..];
    let end = tail.find("</text>").unwrap() + "</text>".len();
    &tail[..end]
}

fn assert_json_brackets_balanced(json: &str) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalSampleCapability {
    UsesReferenceBackedColumnGridProjection,
}

#[derive(Debug, Clone, Copy)]
struct LocalSampleFixture {
    file_name: &'static str,
    capabilities: &'static [LocalSampleCapability],
}

const SHANAI_LAN_LOCAL_SAMPLE_CAPABILITIES: &[LocalSampleCapability] =
    &[LocalSampleCapability::UsesReferenceBackedColumnGridProjection];

const LOCAL_SAMPLE_FIXTURES: &[LocalSampleFixture] = &[LocalSampleFixture {
    file_name: "ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd",
    capabilities: SHANAI_LAN_LOCAL_SAMPLE_CAPABILITIES,
}];

impl LocalSampleFixture {
    fn has_capability(self, capability: LocalSampleCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

fn local_samples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("rjtd-testdata/local-samples")
}

fn local_sample_fixture_for_path(path: &Path) -> Option<LocalSampleFixture> {
    let file_name = path.file_name().and_then(|name| name.to_str())?;
    LOCAL_SAMPLE_FIXTURES
        .iter()
        .copied()
        .find(|fixture| fixture.file_name == file_name)
}

fn local_sample_has_capability(path: &Path, capability: LocalSampleCapability) -> bool {
    local_sample_fixture_for_path(path)
        .map(|fixture| fixture.has_capability(capability))
        .unwrap_or(false)
}

fn test_json_string_array(values: &[&str]) -> String {
    let mut output = String::new();
    push_json_string_slice_array(&mut output, values);
    output
}

fn test_table_grid_cross_table_row_boundary_offset_table(
    table_candidate_index: usize,
    source_start: usize,
    source_end: usize,
    line_mark_record_indexes: Vec<usize>,
    selected_spacing_record_indexes: Vec<usize>,
    row_source_start_units: Vec<usize>,
) -> TableGridCrossTableRowBoundaryOffsetTable {
    let row_count = row_source_start_units
        .len()
        .max(line_mark_record_indexes.len());
    let line_mark_record_y_tops_px = line_mark_record_indexes
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();
    let selected_spacing_record_y_tops_px = selected_spacing_record_indexes
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();

    TableGridCrossTableRowBoundaryOffsetTable {
        table_candidate_index,
        source_start,
        source_end,
        row_count,
        line_mark_record_indexes: line_mark_record_indexes.clone(),
        page_mark_line_offsets_from_entry_start: vec![0; line_mark_record_indexes.len()],
        page_mark_records_within_single_entry: true,
        line_mark_record_y_tops_px,
        selected_spacing_record_indexes: selected_spacing_record_indexes.clone(),
        selected_spacing_page_mark_line_offsets_from_entry_start: vec![
            0;
            selected_spacing_record_indexes.len()
        ],
        selected_spacing_records_within_single_entry: true,
        selected_spacing_record_y_tops_px,
        selected_spacing_line_mark_start_units: row_source_start_units.clone(),
        selected_spacing_line_mark_end_units: row_source_start_units.clone(),
        selected_spacing_start_residual_units: vec![0; row_source_start_units.len()],
        selected_spacing_end_residual_units: vec![0; row_source_start_units.len()],
        selected_spacing_span_residual_units: vec![0; row_source_start_units.len()],
        selected_minus_previous_record_index_gaps: selected_spacing_record_indexes
            .iter()
            .copied()
            .zip(line_mark_record_indexes.iter().copied())
            .map(|(selected, previous)| signed_usize_delta_i32(selected, previous))
            .collect::<Vec<_>>(),
        selected_minus_previous_record_y_delta_px: selected_spacing_record_indexes
            .iter()
            .copied()
            .zip(line_mark_record_indexes.iter().copied())
            .map(|(selected, previous)| selected as f32 - previous as f32)
            .collect::<Vec<_>>(),
        row_source_start_units: row_source_start_units.clone(),
        row_source_end_units: row_source_start_units.clone(),
        line_mark_start_units: row_source_start_units.clone(),
        line_mark_end_units: row_source_start_units.clone(),
        start_residual_units: vec![0; row_source_start_units.len()],
        end_residual_units: vec![0; row_source_start_units.len()],
        span_residual_units: vec![0; row_source_start_units.len()],
        row_boundary_offset_candidate_units: None,
        offset_normalized_start_residual_units: vec![0; row_source_start_units.len()],
        offset_normalized_end_residual_units: vec![0; row_source_start_units.len()],
        offset_normalized_exact_boundary_aligned: false,
        exact_boundary_aligned: false,
        span_only_match: false,
    }
}

fn test_table_grid_source_unit_to_page_line_index_piecewise_transition(
    source_range_gap_units: usize,
    row_source_start_gap_units: i32,
    line_mark_record_gap: i32,
) -> TableGridSourceUnitToPageLineIndexPiecewiseTransition {
    TableGridSourceUnitToPageLineIndexPiecewiseTransition {
        from_table_candidate_index: 0,
        to_table_candidate_index: 1,
        previous_last_source_unit: 0,
        next_first_source_unit: 0,
        source_range_gap_units,
        row_source_start_gap_units,
        previous_last_record_index: 0,
        next_first_record_index: 0,
        line_mark_record_gap,
        same_page_mark_entry: true,
    }
}

fn test_table_grid_cross_table_row_boundary_offset_probe(
    tables: Vec<TableGridCrossTableRowBoundaryOffsetTable>,
    transitions: Vec<TableGridSourceUnitToPageLineIndexPiecewiseTransition>,
) -> TableGridCrossTableRowBoundaryOffsetProbe {
    let related_table_candidate_indexes = tables
        .iter()
        .map(|table| table.table_candidate_index)
        .collect::<Vec<_>>();
    let combined_line_mark_record_indexes = tables
        .iter()
        .flat_map(|table| table.line_mark_record_indexes.iter().copied())
        .collect::<Vec<_>>();
    let combined_line_mark_record_y_tops_px = tables
        .iter()
        .flat_map(|table| table.line_mark_record_y_tops_px.iter().copied())
        .collect::<Vec<_>>();
    let source_unit_to_page_line_index_source_units = tables
        .iter()
        .flat_map(|table| table.row_source_start_units.iter().copied())
        .collect::<Vec<_>>();

    TableGridCrossTableRowBoundaryOffsetProbe {
        current_table_candidate_index: related_table_candidate_indexes
            .first()
            .copied()
            .unwrap_or(0),
        sparse_table_candidate_index: 0,
        related_table_candidate_indexes,
        related_table_count: tables.len(),
        table_count_with_previous_row_span_alignment: tables.len(),
        row_boundary_offset_candidate_units: Vec::new(),
        stable_row_boundary_offset_candidate_units: None,
        all_related_tables_have_offset_candidate: false,
        all_offsets_stable: false,
        all_offsets_require_transform: false,
        all_offset_normalized_boundaries_exact: false,
        combined_line_mark_record_indexes,
        page_mark_entry_index: Some(0),
        page_index_candidate: Some(0),
        page_line_start: Some(0),
        page_line_end: Some(0),
        page_mark_u16_field_count: 0,
        page_mark_u16_field_preview: Vec::new(),
        combined_line_offsets_from_page_start: Vec::new(),
        combined_line_offsets_monotonic: true,
        combined_line_mark_record_y_pitch_px: Some(23.298),
        combined_line_mark_record_y_pitch_basis: Some("test"),
        combined_line_mark_record_y_tops_px,
        combined_line_mark_record_y_span_px: None,
        source_unit_to_page_line_index_source_units,
        source_unit_to_page_line_index_slope: None,
        source_unit_to_page_line_index_intercept: None,
        source_unit_to_page_line_index_fitted_indexes: Vec::new(),
        source_unit_to_page_line_index_residual_indexes: Vec::new(),
        source_unit_to_page_line_index_max_abs_residual: None,
        source_unit_to_page_line_index_exact: false,
        source_unit_to_page_line_index_rows: Vec::new(),
        source_unit_to_page_line_index_piecewise_max_abs_residual: None,
        source_unit_to_page_line_index_piecewise_all_tables_exact: false,
        source_unit_to_page_line_index_piecewise_tables: Vec::new(),
        source_unit_to_page_line_index_piecewise_transitions: transitions,
        all_records_within_single_page_mark_entry: true,
        tables,
    }
}

fn tail_after_occurrence<'a>(haystack: &'a str, marker: &str, occurrence: usize) -> &'a str {
    let mut tail = haystack;
    for index in 0..=occurrence {
        let Some((_, next_tail)) = tail.split_once(marker) else {
            panic!("missing JSON marker occurrence {index} for {marker}");
        };
        tail = next_tail;
    }
    tail
}

fn assert_json_string_field_after(
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

fn assert_json_number_field_after(
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

fn assert_json_bool_field_after(
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

fn assert_json_string_array_field_after(
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
fn fdm_bbox_center_handles_extreme_bounds_without_overflow() {
    assert_eq!(
        fdm_bbox_center((i32::MIN, i32::MIN, i32::MAX, i32::MAX)),
        (-1, -1)
    );
    assert_eq!(fdm_bbox_center((-3, -3, -2, -2)), (-3, -3));
}

fn embedded_press_state_record_payload_first_words(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    record_type: u32,
) -> Vec<u32> {
    path.state_records()
        .iter()
        .filter(|record| record.record_type() == record_type)
        .filter_map(|record| record.payload_le32_words().first().copied())
        .collect::<Vec<_>>()
}

fn embedded_press_test_outline_path(
    commands: Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
) -> ObjectEmbeddedPressVectorPathCandidate {
    ObjectEmbeddedPressVectorPathCandidate::new(
        ObjectEmbeddedPressVectorPathKind::Outline,
        None,
        Vec::new(),
        commands,
    )
}

fn push_embedded_press_test_line_to(
    commands: &mut Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
    from: (u32, u32),
    to: (u32, u32),
) {
    commands.push(ObjectEmbeddedPressVectorPathCommandCandidate::CubicTo {
        x1: from.0,
        y1: from.1,
        x2: to.0,
        y2: to.1,
        x3: to.0,
        y3: to.1,
    });
}

fn test_fdm_vector_segment(
    bbox: ObjectFdmIndexBbox,
    source_width: i32,
    source_height: i32,
) -> ObjectFdmVectorSegmentCandidate {
    ObjectFdmVectorSegmentCandidate::new(
        0,
        FdmVectorSegmentHeader {
            declared_len: 10,
            command_count: 0,
            command_offsets: Vec::new(),
            bbox: Some(bbox),
            source_width,
            source_height,
        },
    )
}

fn test_fdm_text_candidate(text: &str, bbox: ObjectFdmIndexBbox) -> ObjectFdmTextCandidate {
    ObjectFdmTextCandidate::new(text, 0, 0, Vec::new(), Some(bbox))
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
fn preserves_unknown_blocks() {
    let unknown = UnknownBlock::new(UnknownRecordKind::new(Some(7)), vec![1, 2, 3]);
    let document = Document::new(Metadata::default(), vec![Block::Unknown(unknown)]);

    assert_eq!(document.blocks().len(), 1);
    match &document.blocks()[0] {
        Block::Unknown(block) => assert_eq!(block.payload(), &[1, 2, 3]),
        Block::Paragraph(_) => panic!("expected unknown block"),
    }
}

#[test]
fn builds_document_from_plain_text_lines() {
    let document = Document::from_plain_text("銀河鉄道\r\n\r\n午后の授業\n");

    assert_eq!(document.blocks().len(), 2);
    match &document.blocks()[1] {
        Block::Paragraph(paragraph) => match &paragraph.inlines()[0] {
            Inline::Text(text) => assert_eq!(text.text(), "午后の授業"),
            Inline::Ruby(_) => panic!("expected text inline"),
            Inline::Unknown(_) => panic!("expected text inline"),
        },
        Block::Unknown(_) => panic!("expected paragraph"),
    }
}

#[test]
fn document_core_renders_text_svg_pages() {
    let document = Document::from_plain_text("銀河鉄道\n午后の授業");
    let core = DocumentCore::from_document(document);

    assert_eq!(core.page_count(), 1);
    assert!(core.get_document_info().contains("\"engine\":\"rjtd\""));
    assert!(core.plain_text().contains("銀河鉄道"));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.starts_with("<svg "));
    assert!(svg.contains("銀河鉄道"));
    assert!(!svg.contains(">1/1</text>"));

    let lines = core.page_text_lines(0).unwrap();
    assert_eq!(lines[0].text(), "銀河鉄道");
    assert_eq!(lines[0].paragraph_index(), Some(0));
    assert_eq!(lines[0].char_start(), 0);
    assert_eq!(lines[0].char_end(), 4);
}

#[test]
fn document_core_renders_column_grid_candidates_as_diagnostic_svg_overlay() {
    let mut document = Document::from_plain_text("本文");
    let intervals = vec![
        TableCandidateInterval::new(
            0,
            0,
            0,
            50,
            "　　売掛金2,441,9973,983,602△1,541,6042,766,830".to_string(),
        ),
        TableCandidateInterval::new(
            1,
            1,
            51,
            100,
            "　　買掛金1,111,1112,222,222△3,333,3334,444,444".to_string(),
        ),
    ];
    document.push_table_candidate(TableCandidate {
        index: 0,
        text_boundary_candidate_index: 0,
        text_count_range_index: 0,
        basis: TextCountRangeOverlapBasis::Unit,
        delimiter_code: 0x000e,
        interval_count: intervals.len(),
        first_interval_index: 0,
        last_interval_index: intervals.len() - 1,
        source_start: 0,
        source_end: 100,
        intervals,
    });
    let core = DocumentCore::from_document(document);

    let svg = core.render_page_svg(0).unwrap();
    assert!(!svg.contains("class=\"rjtd-column-grid-candidate\""));
    assert!(!svg.contains("data-col-count-candidate=\"5\""));
    assert!(!svg.contains(">売掛金<"));
    assert!(!svg.contains(">2,441,997<"));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"tableGridCandidate\""));
    assert!(layer_tree.contains("\"projectionKind\":\"diagnosticProjection\""));
    assert!(layer_tree.contains("\"decoded\":false"));
    assert!(layer_tree.contains("\"geometryDecoded\":false"));
    assert!(layer_tree.contains("\"colCountCandidate\":5"));
}

#[test]
fn document_core_reports_rhwp_shaped_page_and_layer_info() {
    let document = Document::from_plain_text("銀河鉄道\n午后の授業");
    let mut core = DocumentCore::from_document(document.clone());
    core.set_file_name("sample.jtd");

    let document_info = core.get_document_info();
    assert!(document_info.contains("\"sourceFormat\":\"jtd\""));
    assert!(document_info.contains("\"fileName\":\"sample.jtd\""));
    assert!(document_info.contains("\"sectionCount\":1"));
    assert!(document_info.contains("\"writingMode\":\"horizontal\""));
    assert!(document_info.contains("\"writingModeDecoded\":false"));
    assert!(document_info.contains(
        "\"writingModeDecision\":{\"selected\":\"horizontal\",\"source\":\"default-horizontal\""
    ));
    assert!(document_info.contains("\"computedBeforeRuntimeOverride\":\"horizontal\""));
    assert!(document_info.contains("\"documentViewStylesCandidate\":null"));
    assert!(document_info.contains("\"sourceDocumentLayoutHintCandidate\":null"));
    assert!(document_info.contains("\"paperMarkCandidate\":null"));
    assert!(document_info.contains("\"documentViewStylesDisagreesWithSelected\":false"));
    assert!(document_info.contains("\"writingModeCandidateFromDocumentViewStyles\":null"));
    assert!(document_info.contains("\"writingModeCandidateFromDocumentViewStylesDecoded\":false"));
    assert!(
        document_info.contains("\"writingModeCandidateFromDocumentViewStylesSourceBacked\":false")
    );
    assert!(
        document_info
            .contains("\"writingModeCandidateFromDocumentViewStylesFirstRecordCode\":null")
    );
    assert!(document_info.contains("\"writingModeCandidateFromPaperMark\":null"));
    assert!(document_info.contains("\"writingModeCandidateDecoded\":false"));
    assert!(document_info.contains("\"textControlBoundaryCount\":0"));
    assert!(document_info.contains("\"textControlBoundaries\":[]"));

    let page_info = core.get_page_info(0).unwrap();
    assert!(page_info.contains("\"pageIndex\":0"));
    assert!(page_info.contains("\"pageNumber\":1"));
    assert!(page_info.contains("\"width\":794.0"));
    assert!(page_info.contains("\"columns\":[{\"x\":72.0,\"width\":650.0}]"));
    assert!(page_info.contains("\"layoutMarkEvidence\":null"));

    assert!(
        core.get_page_def(0)
            .unwrap()
            .contains("\"landscape\":false")
    );
    assert!(
        core.get_section_def(0)
            .unwrap()
            .contains("\"hideHeader\":false")
    );
    assert!(
        core.get_page_border_fill(0)
            .unwrap()
            .contains("\"fillType\":\"none\"")
    );
    core.set_dpi(120.0);
    assert_eq!(core.get_dpi(), 120.0);
    core.set_show_paragraph_marks(true);
    core.set_show_control_codes(true);
    core.set_show_transparent_borders(true);
    core.set_clip_enabled(false);

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert_json_brackets_balanced(&layer_tree);
    assert!(layer_tree.contains("]},\"textSources\""));
    assert!(layer_tree.contains("\"schema\":{\"major\":1,\"minor\":0}"));
    assert!(layer_tree.contains("\"resourceTable\":{\"major\":1,\"minor\":0}"));
    assert!(layer_tree.contains("\"writingMode\":\"horizontal\""));
    assert!(layer_tree.contains("\"writingModeDecoded\":false"));
    assert!(layer_tree.contains("\"outputOptions\":{"));
    assert!(layer_tree.contains("\"showParagraphMarks\":true"));
    assert!(layer_tree.contains("\"showControlCodes\":true"));
    assert!(layer_tree.contains("\"showTransparentBorders\":true"));
    assert!(layer_tree.contains("\"clipEnabled\":false"));
    assert!(layer_tree.contains("\"pageWidth\":794.0"));
    assert!(layer_tree.contains("\"root\":{\"kind\":\"leaf\""));
    assert!(layer_tree.contains("\"type\":\"pageBackground\""));
    assert!(layer_tree.contains("\"backgroundColor\":\"#ffffff\""));
    assert!(layer_tree.contains("\"type\":\"textRun\""));
    assert!(layer_tree.contains("\"isVertical\":false"));
    assert!(layer_tree.contains("\"orientation\":\"horizontal\""));
    assert!(layer_tree.contains("\"textSources\":["));
    assert!(layer_tree.contains("\"fontResources\":{\"blobs\":[],\"faces\":[]}"));
    assert!(layer_tree.contains("\"knownFeatures\":["));
    assert!(layer_tree.contains("\"sourceTextPreserved\":true"));
    assert!(layer_tree.contains("\"textV2\":{\"diagnostics\":[]"));

    let print_layer_tree = core.get_page_layer_tree_with_profile(0, "print").unwrap();
    assert!(print_layer_tree.contains("\"profile\":\"print\""));

    assert_eq!(
        core.get_page_overlay_images(0).unwrap(),
        "{\"behind\":[],\"front\":[],\"imageCount\":0}"
    );
    let replay_plan = core.get_canvaskit_replay_plan(0, "compatibility").unwrap();
    assert!(replay_plan.contains("\"mode\":\"compat\""));
    assert!(replay_plan.contains("\"totalItems\":3"));
    assert!(replay_plan.contains("\"directItems\":3"));
    assert!(replay_plan.contains("\"path\":\"root/leaf/0\""));
    assert!(replay_plan.contains("\"opType\":\"pageBackground\""));
    assert!(replay_plan.contains("\"replayPlane\":\"background\""));
    assert!(replay_plan.contains("\"feature\":\"pageBackground\""));
    assert!(replay_plan.contains("\"path\":\"root/leaf/1\""));
    assert!(replay_plan.contains("\"opType\":\"textRun\""));
    assert!(replay_plan.contains("\"replayPlane\":\"flow\""));
    assert!(replay_plan.contains("\"feature\":\"textRun\""));
    assert!(replay_plan.contains("\"status\":\"direct\""));
    assert!(replay_plan.contains("\"reason\":\"directReplaySupported\""));
    assert!(replay_plan.contains("\"detail\":\"projectionKind=fallback;sourceId=0\""));
    let invalid_mode = core.get_canvaskit_replay_plan(0, "canvas2d").unwrap_err();
    assert!(invalid_mode.to_string().contains("canvas2d"));
    assert!(
        invalid_mode
            .to_string()
            .contains("allowed modes: default, compat")
    );
    assert_eq!(core.get_source_format(), "jtd");
    assert_eq!(
        core.convert_to_editable(),
        "{\"ok\":true,\"converted\":false}"
    );

    let cursor_rect = core.get_cursor_rect(0, 0, 0).unwrap();
    assert!(cursor_rect.contains("\"pageIndex\":0"));
    assert!(cursor_rect.contains("\"x\":72.0"));
    assert!(cursor_rect.contains("\"y\":72.0"));
    assert!(cursor_rect.contains("\"height\":23.0"));

    let hit = core.hit_test(0, 72.0, 72.0).unwrap();
    assert!(hit.contains("\"hit\":true"));
    assert!(hit.contains("\"paragraphIndex\":0"));
    assert!(hit.contains("\"charOffset\":0"));

    let line_info = core.get_line_info(0, 0, 1).unwrap();
    assert!(line_info.contains("\"lineIndex\":0"));
    assert!(line_info.contains("\"lineCount\":1"));
    assert!(line_info.contains("\"charStart\":0"));
    assert!(line_info.contains("\"charEnd\":4"));

    let moved = core.move_vertical(0, 0, 0, 1, -1.0).unwrap();
    assert!(moved.contains("\"paragraphIndex\":1"));
    assert!(moved.contains("\"preferredX\":72.0"));
}

#[test]
fn document_core_projects_vertical_writing_mode_to_svg_and_layer_tree() {
    let document = Document::from_plain_text("縦書き\n本文");
    let mut core = DocumentCore::from_document(document.clone());
    core.set_writing_mode(WritingMode::VerticalRl);

    assert_eq!(core.writing_mode(), WritingMode::VerticalRl);
    assert!(
        core.get_document_info()
            .contains("\"writingMode\":\"vertical-rl\"")
    );

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("writing-mode=\"vertical-rl\""));
    assert!(svg.contains(">縦書き</text>"));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"writingMode\":\"vertical-rl\""));
    assert!(layer_tree.contains("\"writingModeDecoded\":false"));
    assert!(layer_tree.contains("\"isVertical\":true"));
    assert!(layer_tree.contains("\"orientation\":\"vertical-rl\""));
    assert!(layer_tree.contains("\"projectionKind\":\"fallback\""));
}

#[test]
fn document_core_decodes_page_size_from_document_view_styles() {
    let view_styles = document_view_styles_page_size_fixture(14_800, 21_000);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (DOCUMENT_VIEW_STYLES_PATH, &view_styles),
    ]);
    let mut core = DocumentCore::from_bytes(&bytes).unwrap();

    assert!((core.page_width_px() - 559.4).abs() < 0.2);
    assert!((core.page_height_px() - 793.7).abs() < 0.2);
    assert!(
        core.get_page_def(0)
            .unwrap()
            .contains("\"landscape\":false")
    );

    core.set_file_name("a5.jtd");
    assert_eq!(core.writing_mode(), WritingMode::Horizontal);
    assert!((core.page_width_px() - 559.4).abs() < 0.2);
    assert!((core.page_height_px() - 793.7).abs() < 0.2);
    assert!(
        core.get_page_def(0)
            .unwrap()
            .contains("\"landscape\":false")
    );
}

#[test]
fn document_core_prefers_page_layout_style_over_document_view_styles_page_size() {
    let view_styles = document_view_styles_page_size_fixture(16_395, 29_700);
    let page_layout_style = page_layout_style_page_size_fixture(21_000, 29_700);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (DOCUMENT_VIEW_STYLES_PATH, &view_styles),
        (PAGE_LAYOUT_STYLE_PATH, &page_layout_style),
    ]);

    let core = DocumentCore::from_bytes(&bytes).unwrap();

    assert!((core.page_width_px() - 793.7).abs() < 0.2);
    assert!((core.page_height_px() - 1122.5).abs() < 0.2);
    assert!(
        core.get_page_def(0)
            .unwrap()
            .contains("\"landscape\":false")
    );
}

fn document_view_styles_sequential_fixture(first_code: u16) -> Vec<u8> {
    // Build a minimal sequential style stream with 4 records.
    // The sequential record parser requires >= 4 records to accept a sequence.
    // Each record: code u16be, length u16be, payload bytes.
    let mut bytes = vec![0u8; 10]; // 10-byte header prefix
    for i in 0..4u16 {
        let code = first_code + i;
        bytes.extend_from_slice(&code.to_be_bytes()); // code
        bytes.extend_from_slice(&0x0001_u16.to_be_bytes()); // length = 1
        bytes.push(0x00); // payload
    }
    bytes
}

#[test]
fn document_core_keeps_document_view_styles_writing_mode_candidate_diagnostic_only() {
    // 0x1001 is observed in both vertical and horizontal reference-PDF
    // samples, so it must remain a candidate rather than render authority.
    let vertical_styles = document_view_styles_sequential_fixture(0x1001);
    let bytes_v = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (DOCUMENT_VIEW_STYLES_PATH, &vertical_styles),
    ]);
    let core_v = DocumentCore::from_bytes(&bytes_v).unwrap();
    assert_eq!(core_v.writing_mode(), WritingMode::Horizontal);
    let info_v = core_v.get_document_info();
    assert!(info_v.contains("\"writingMode\":\"horizontal\""));
    assert!(info_v.contains(
        "\"writingModeDecision\":{\"selected\":\"horizontal\",\"source\":\"default-horizontal\""
    ));
    assert!(info_v.contains("\"computedBeforeRuntimeOverride\":\"horizontal\""));
    assert!(info_v.contains("\"documentViewStylesCandidate\":\"vertical-rl\""));
    assert!(info_v.contains("\"sourceDocumentLayoutHintCandidate\":null"));
    assert!(info_v.contains("\"paperMarkCandidate\":null"));
    assert!(info_v.contains("\"documentViewStylesDisagreesWithSelected\":true"));
    assert!(info_v.contains("\"writingModeCandidateFromDocumentViewStyles\":\"vertical-rl\""));
    assert!(info_v.contains("\"writingModeCandidateFromDocumentViewStylesSourceBacked\":true"));
    assert!(info_v.contains("\"writingModeCandidateFromDocumentViewStylesFirstRecordCode\":4097"));
    assert!(
        info_v.contains(
            "\"writingModeCandidateFromDocumentViewStylesFirstRecordCodeHex\":\"0x1001\""
        )
    );

    // DocumentViewStyles whose first sequential record is 0x1002 (not 0x1001) → horizontal
    let horizontal_styles = document_view_styles_sequential_fixture(0x1002);
    let bytes_h = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (DOCUMENT_VIEW_STYLES_PATH, &horizontal_styles),
    ]);
    let core_h = DocumentCore::from_bytes(&bytes_h).unwrap();
    assert_eq!(core_h.writing_mode(), WritingMode::Horizontal);
    let info_h = core_h.get_document_info();
    assert!(info_h.contains("\"writingMode\":\"horizontal\""));
    assert!(info_h.contains(
        "\"writingModeDecision\":{\"selected\":\"horizontal\",\"source\":\"default-horizontal\""
    ));
    assert!(info_h.contains("\"computedBeforeRuntimeOverride\":\"horizontal\""));
    assert!(info_h.contains("\"documentViewStylesCandidate\":\"horizontal\""));
    assert!(info_h.contains("\"sourceDocumentLayoutHintCandidate\":null"));
    assert!(info_h.contains("\"paperMarkCandidate\":null"));
    assert!(info_h.contains("\"documentViewStylesDisagreesWithSelected\":false"));
    assert!(info_h.contains("\"writingModeCandidateFromDocumentViewStyles\":\"horizontal\""));
    assert!(info_h.contains("\"writingModeCandidateFromDocumentViewStylesSourceBacked\":true"));
    assert!(info_h.contains("\"writingModeCandidateFromDocumentViewStylesFirstRecordCode\":4098"));
    assert!(
        info_h.contains(
            "\"writingModeCandidateFromDocumentViewStylesFirstRecordCodeHex\":\"0x1002\""
        )
    );
}

#[test]
fn document_core_reports_paper_mark_flag_bit_diagnostics_without_render_promotion() {
    let vertical_paper_mark = paper_mark_fixture(&[(0, 0x0001_0000), (1, 0x0001_0001)]);
    let vertical_bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (PAPER_MARK_PATH, &vertical_paper_mark),
    ]);
    let vertical_core = DocumentCore::from_bytes(&vertical_bytes).unwrap();

    assert_eq!(vertical_core.writing_mode(), WritingMode::Horizontal);
    let vertical_info = vertical_core.get_document_info();
    assert_json_brackets_balanced(&vertical_info);
    assert!(vertical_info.contains("\"writingMode\":\"horizontal\""));
    assert!(vertical_info.contains("\"writingModeCandidateFromPaperMark\":\"vertical-rl\""));
    assert!(vertical_info.contains("\"writingModeCandidateDecoded\":false"));
    assert!(vertical_info.contains("\"paperMarkFlagBit0VerticalCandidate\":true"));
    assert!(vertical_info.contains("\"paperMarkFlagBit17IndexStepCandidate\":false"));
    assert!(vertical_info.contains(
        "\"paperMarkWritingModeCandidateEvidence\":[\"paper-mark-flag-bit0-vertical-corpus-consistent\"]"
    ));
    assert!(vertical_info.contains(
        "\"paperMarkWritingModeCandidateBlockers\":[\"paper-mark-writing-mode-flag-semantics-unproven\"]"
    ));

    let index_step_paper_mark = paper_mark_fixture(&[(0, 0x0002_0000), (2, 0x0002_0010)]);
    let index_step_bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (PAPER_MARK_PATH, &index_step_paper_mark),
    ]);
    let index_step_core = DocumentCore::from_bytes(&index_step_bytes).unwrap();
    let index_step_info = index_step_core.get_document_info();
    assert_json_brackets_balanced(&index_step_info);
    assert!(index_step_info.contains("\"writingModeCandidateFromPaperMark\":null"));
    assert!(index_step_info.contains("\"paperMarkFlagBit0VerticalCandidate\":false"));
    assert!(index_step_info.contains("\"paperMarkFlagBit17IndexStepCandidate\":true"));
    assert!(
        index_step_core
            .get_page_def(0)
            .unwrap()
            .contains("\"landscape\":false")
    );
}

#[test]
fn document_core_does_not_apply_layout_hints_from_filename_only() {
    for file_name in [
        "a5.jtd",
        "a6.jtd",
        "b6.jtd",
        "ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd",
    ] {
        let mut core = DocumentCore::from_document(Document::from_plain_text("銀河鉄道の夜"));
        core.set_file_name(file_name);

        assert_eq!(core.writing_mode(), WritingMode::Horizontal);
        assert!((core.page_width_px() - f64::from(APP_PAGE_WIDTH_PX)).abs() < 0.2);
        assert!((core.page_height_px() - f64::from(APP_PAGE_HEIGHT_PX)).abs() < 0.2);
        assert!(
            core.get_document_info()
                .contains("\"writingMode\":\"horizontal\"")
        );

        let page_def = core.get_page_def(0).unwrap();
        assert!(page_def.contains("\"landscape\":false"));

        let svg = core.render_page_svg(0).unwrap();
        assert!(!svg.contains("writing-mode=\"vertical-rl\""));
        assert!(svg.contains(&format!("width=\"{:.1}\"", core.page_width_px())));
        assert!(svg.contains(&format!("height=\"{:.1}\"", core.page_height_px())));

        let layer_tree = core.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"writingMode\":\"horizontal\""));
        assert!(layer_tree.contains(&format!("\"pageWidth\":{:.1}", core.page_width_px())));
        assert!(layer_tree.contains(&format!("\"pageHeight\":{:.1}", core.page_height_px())));
    }
}

#[test]
fn document_core_normalizes_decoded_page_size_to_portrait_without_source_landscape_evidence() {
    let view_styles = document_view_styles_page_size_fixture(21_000, 14_800);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (DOCUMENT_VIEW_STYLES_PATH, &view_styles),
    ]);

    let core = DocumentCore::from_bytes(&bytes).unwrap();

    assert!((core.page_width_px() - 559.4).abs() < 0.2);
    assert!((core.page_height_px() - 793.7).abs() < 0.2);
    assert!(
        core.get_page_def(0)
            .unwrap()
            .contains("\"landscape\":false")
    );
}

#[test]
fn document_core_does_not_override_decoded_page_size_from_filename_only() {
    let view_styles = document_view_styles_page_size_fixture(12_800, 18_800);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (DOCUMENT_VIEW_STYLES_PATH, &view_styles),
    ]);
    let mut core = DocumentCore::from_bytes(&bytes).unwrap();

    assert!((core.page_width_px() - 483.8).abs() < 0.2);
    assert!((core.page_height_px() - 710.6).abs() < 0.2);

    core.set_file_name("46.jtd");

    assert_eq!(core.writing_mode(), WritingMode::Horizontal);
    assert!((core.page_width_px() - 483.8).abs() < 0.2);
    assert!((core.page_height_px() - 710.6).abs() < 0.2);
    assert!(
        core.get_page_def(0)
            .unwrap()
            .contains("\"landscape\":false")
    );
}

#[test]
fn document_core_uses_form_feed_control_as_forced_page_break() {
    let bytes = cfb_with_document_text(document_text_with_page_break());
    let mut core = DocumentCore::from_bytes(&bytes).unwrap();
    core.set_writing_mode(WritingMode::VerticalRl);

    assert_eq!(core.page_count(), 2);
    assert!(
        core.document()
            .text_control_boundaries()
            .iter()
            .any(|boundary| boundary.code() == DOCUMENT_TEXT_PAGE_BREAK_CONTROL)
    );
    assert!(
        core.page_text_lines(0).unwrap()[0]
            .text()
            .contains("銀河鉄道の夜")
    );
    assert!(
        !core
            .page_text_lines(0)
            .unwrap()
            .iter()
            .any(|line| line.text().contains("目次"))
    );
    assert!(core.page_text_lines(1).unwrap()[0].text().contains("目次"));

    let first_page = core.render_page_svg(0).unwrap();
    let second_page = core.render_page_svg(1).unwrap();
    assert!(!first_page.contains(">1/2</text>"));
    assert!(!second_page.contains(">2/2</text>"));
    assert!(first_page.contains("writing-mode=\"vertical-rl\""));
}

#[test]
fn document_core_projects_a5_ginga_front_matter_from_reference_pdf() {
    let document = Document::from_plain_text(
        "銀河鉄道の夜\t\t\t\t宮沢 賢治\n目次\n一、午后の授業\n二、活版所\n銀河鉄道の夜\n一、午后の授業\nではみなさんは",
    );
    let mut core = DocumentCore::from_document(document);
    core.set_file_name("a5.jtd");

    assert_eq!(core.page_count(), 6);
    assert!(
        core.page_text_lines(0).unwrap()[0]
            .text()
            .contains("銀河鉄道の夜")
    );
    assert!(core.page_text_lines(1).unwrap().is_empty());
    assert_eq!(core.page_text_lines(2).unwrap()[0].text(), "目次");
    assert!(core.page_text_lines(3).unwrap().is_empty());
    assert_eq!(core.page_text_lines(4).unwrap()[0].text(), "銀河鉄道の夜");
    assert_eq!(core.page_text_lines(5).unwrap()[0].text(), "");
    assert_eq!(core.page_text_lines(5).unwrap()[1].text(), "");
    assert_eq!(core.page_text_lines(5).unwrap()[2].text(), "一、午后の授業");
    assert_eq!(core.page_text_lines(5).unwrap()[3].text(), "");
    assert_eq!(core.page_text_lines(5).unwrap()[4].text(), "");
    assert_eq!(core.page_text_lines(5).unwrap()[5].text(), "ではみなさんは");

    let title_page = core.render_page_svg(0).unwrap();
    assert!(title_page.contains("class=\"rjtd-text\""));
    assert!(title_page.contains("銀河鉄道の夜"));
    assert!(title_page.contains("　　"));
    assert!(!title_page.contains("rjtd-page-number-projection"));

    let body_page = core.render_page_svg(5).unwrap();
    assert!(!body_page.contains("class=\"rjtd-page-number-projection\""));
    assert!(!body_page.contains("class=\"rjtd-running-header-projection\""));
    assert!(body_page.contains("一、午后の授業"));
}

#[test]
fn parser_preserves_auto_text_info_candidates() {
    let auto_text = auto_text_info_fixture("銀河鉄道の夜");
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (rjtd_core::auto_text_info::AUTO_TEXT_INFO_PATH, &auto_text),
    ]);

    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.auto_texts().len(), 1);
    assert_eq!(document.auto_texts()[0].source_stream(), "/AutoTextInfo");
    assert_eq!(document.auto_texts()[0].text(), "銀河鉄道の夜");

    let mut core = DocumentCore::from_document(document);
    core.set_file_name("a5.jtd");
    let info = core.get_document_info();
    assert!(info.contains("\"autoTextCount\":1"));
    assert!(info.contains("\"text\":\"銀河鉄道の夜\""));
}

#[test]
fn document_core_renders_running_decorations_from_model_evidence() {
    let mut document = Document::from_plain_text(&format!(
        "{}\n{}",
        "銀河鉄道の夜\t\t\t\t宮沢 賢治\n目次\n一、午后の授業\n二、活版所\n銀河鉄道の夜\n一、午后の授業",
        "ではみなさんは、そういうふうに川だと云われたりしていました。".repeat(120)
    ));
    document.push_auto_text(DocumentAutoText::new("/AutoTextInfo", 84, "銀河鉄道の夜"));
    document.push_unknown_style(UnknownStyle::from_stream(
        PAGE_LAYOUT_STYLE_PATH,
        ssmg_page_layout_style_with_subrecords_fixture(),
    ));
    let mut core = DocumentCore::from_document(document);
    core.set_file_name("a5.jtd");

    assert!(core.page_count() >= 7);
    let even_page = core.render_page_svg(5).unwrap();
    assert!(even_page.contains("class=\"rjtd-page-number\""));
    assert!(even_page.contains("data-side=\"left\""));
    assert!(even_page.contains(">6</text>"));
    assert!(even_page.contains("class=\"rjtd-running-header\""));
    assert!(even_page.contains("一、午后の授業"));
    let even_header = running_header_svg_element(&even_page);
    assert!(even_header.contains("text-anchor=\"start\""));
    assert!(!even_header.contains("writing-mode=\"vertical-rl\""));

    let odd_page = core.render_page_svg(6).unwrap();
    assert!(odd_page.contains("data-side=\"right\""));
    assert!(odd_page.contains(">7</text>"));
    assert!(odd_page.contains("銀河鉄道の夜"));
    let odd_header = running_header_svg_element(&odd_page);
    assert!(odd_header.contains("text-anchor=\"end\""));
    assert!(!odd_header.contains("writing-mode=\"vertical-rl\""));

    let layer_tree = core.get_page_layer_tree(5).unwrap();
    assert_json_brackets_balanced(&layer_tree);
    assert!(layer_tree.contains("]},\"textSources\""));
    assert!(layer_tree.contains("\"type\":\"pageDecoration\""));
    assert!(layer_tree.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\""));
    assert!(layer_tree.contains("\"sidePolicyDecoded\":false"));
    assert!(layer_tree.contains("\"facingPagesCandidate\":true"));
    assert!(layer_tree.contains("\"pairedSlotPairs\":[\"0x32/0x33\"]"));
    assert!(layer_tree.contains("\"headerText\":\"一、午后の授業\""));
    assert!(layer_tree.contains("\"pageNumber\":6"));
}

fn assert_local_ginga_sample_facing_page_decoration(
    sample_name: &str,
    expected_page_count: Option<u32>,
) {
    let samples_dir = local_samples_dir();
    let sample_path = samples_dir.join(format!("{sample_name}.jtd"));
    let reference_pdf_path = samples_dir.join(format!("{sample_name}.pdf"));
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let document = parse_document(&bytes).unwrap();
    assert!(
        document
            .auto_texts()
            .iter()
            .any(|auto_text| auto_text.text() == "銀河鉄道の夜"),
        "{sample_name} should preserve running title text from /AutoTextInfo"
    );
    assert_eq!(
        document.toc_entries().first().unwrap().page_label(),
        "6",
        "{sample_name} first body chapter should start on visible page 6"
    );
    assert!(
        !document_page_decoration_paired_slot_pairs(&document).is_empty(),
        "{sample_name} should preserve active /PageLayoutStyle paired slots"
    );
    let has_page_paper_mark_pair =
        !document.page_marks().is_empty() && !document.paper_marks().is_empty();

    let mut renamed_core = DocumentCore::from_document(document.clone());
    renamed_core.set_file_name("renamed-ginga-layout.jtd");
    assert_eq!(renamed_core.writing_mode(), WritingMode::VerticalRl);
    if let Some(expected_page_count) = expected_page_count {
        assert_eq!(
            renamed_core.page_count(),
            expected_page_count,
            "{sample_name} should keep page count without relying on its file name"
        );
    }

    let mut core = DocumentCore::from_document(document);
    core.set_file_name(sample_path.to_string_lossy());
    if let Some(expected_page_count) = expected_page_count {
        assert_eq!(
            core.page_count(),
            expected_page_count,
            "{sample_name} should match the local reference PDF page count"
        );
    }
    assert!(
        core.page_count() >= 7,
        "{sample_name} needs enough pages for odd/even decoration checks"
    );

    let page_six = core.render_page_svg(5).unwrap();
    assert!(page_six.contains("class=\"rjtd-page-number\""));
    assert!(page_six.contains("data-side=\"left\""));
    assert!(page_six.contains(">6</text>"));
    assert!(page_six.contains("一、午后の授業"));

    let page_six_layer_tree = core.get_page_layer_tree(5).unwrap();
    assert_json_brackets_balanced(&page_six_layer_tree);
    assert!(page_six_layer_tree.contains("\"type\":\"pageDecoration\""));
    assert!(page_six_layer_tree.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\""));
    assert!(page_six_layer_tree.contains("\"sidePolicyDecoded\":false"));
    assert!(page_six_layer_tree.contains("\"facingPagesCandidate\":true"));
    assert!(
        page_six_layer_tree.contains(
            "\"pairedSlotPairs\":[\"0x32/0x33\",\"0x34/0x35\",\"0x36/0x37\",\"0x38/0x39\"]"
        )
    );
    assert!(page_six_layer_tree.contains("\"side\":\"left\""));
    assert!(page_six_layer_tree.contains("\"pageNumber\":6"));
    assert!(page_six_layer_tree.contains("\"headerText\":\"一、午后の授業\""));
    if has_page_paper_mark_pair {
        assert!(
            page_six_layer_tree
                .contains("\"layoutMarkEvidence\":{\"source\":\"/PageMark+/PaperMark\"")
        );
        assert!(page_six_layer_tree.contains("\"pageMarkEntryIndex\":5"));
        assert!(page_six_layer_tree.contains("\"paperMarkEntryIndex\":5"));
        assert!(page_six_layer_tree.contains("\"rowIndexAligned\":true"));
        assert!(page_six_layer_tree.contains("\"markIndexAligned\":true"));
        assert!(page_six_layer_tree.contains("\"entryCountAligned\":true"));
        assert!(
            page_six_layer_tree.contains(
                "\"renderPromotionBlockedReason\":\"paper-mark-flag-semantics-undecoded\""
            )
        );
    }

    let page_seven = core.render_page_svg(6).unwrap();
    assert!(page_seven.contains("class=\"rjtd-page-number\""));
    assert!(page_seven.contains("data-side=\"right\""));
    assert!(page_seven.contains(">7</text>"));
    assert!(page_seven.contains("銀河鉄道の夜"));

    let page_seven_layer_tree = core.get_page_layer_tree(6).unwrap();
    assert_json_brackets_balanced(&page_seven_layer_tree);
    assert!(page_seven_layer_tree.contains("\"type\":\"pageDecoration\""));
    assert!(page_seven_layer_tree.contains("\"side\":\"right\""));
    assert!(page_seven_layer_tree.contains("\"pageNumber\":7"));
    assert!(page_seven_layer_tree.contains("\"headerText\":\"銀河鉄道の夜\""));
}

#[test]
fn local_a_size_and_b_size_samples_render_facing_page_decorations_when_reference_pdfs_are_available()
 {
    for (sample_name, expected_page_count) in [("a6", Some(114)), ("b6", None)] {
        assert_local_ginga_sample_facing_page_decoration(sample_name, expected_page_count);
    }
}

#[test]
fn local_a5_sample_renders_facing_page_decorations_when_reference_pdf_is_available() {
    let samples_dir = local_samples_dir();
    let sample_path = samples_dir.join("a5.jtd");
    let reference_pdf_path = samples_dir.join("a5.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let document = parse_document(&bytes).unwrap();
    assert!(document.toc_entries().len() >= 9);
    assert_eq!(document.toc_entries()[0].title(), "一、午后の授業");
    assert_eq!(document.toc_entries()[0].page_label(), "6");
    let last_toc_entry = document.toc_entries().last().unwrap();
    assert_eq!(last_toc_entry.title(), "九、ジョバンニの切符");
    assert_eq!(last_toc_entry.page_label(), "42");
    assert_eq!(document.page_marks().len(), 1);
    let page_mark = &document.page_marks()[0];
    assert_eq!(page_mark.source_stream(), PAGE_MARK_PATH);
    assert_eq!(page_mark.family(), "fixed84");
    assert_eq!(page_mark.header_count(), 74);
    assert_eq!(page_mark.header_stride(), 16);
    assert_eq!(page_mark.header_last_index(), 73);
    assert_eq!(page_mark.entries().len(), 75);
    assert_eq!(page_mark.entries()[5].index(), Some(5));
    assert_eq!(page_mark.entries()[5].line_start(), Some(23));
    assert_eq!(page_mark.entries()[5].line_end(), Some(40));
    assert_eq!(page_mark.entries()[41].index(), Some(41));
    assert_eq!(page_mark.entries()[41].line_start(), Some(608));
    assert_eq!(document.paper_marks().len(), 1);
    let paper_mark = &document.paper_marks()[0];
    assert_eq!(paper_mark.source_stream(), PAPER_MARK_PATH);
    assert_eq!(paper_mark.header_count(), 74);
    assert_eq!(paper_mark.header_stride(), 12);
    assert_eq!(paper_mark.header_last_index(), 73);
    assert_eq!(paper_mark.entries().len(), 75);
    assert_eq!(paper_mark.entries()[0].index(), 0);
    assert_eq!(paper_mark.entries()[0].flags(), 0x0001_0010);
    assert_eq!(paper_mark.entries()[4].flags(), 0x0001_0011);
    assert_eq!(paper_mark.entries()[0].raw_len(), 8);

    let mut renamed_core = DocumentCore::from_document(document.clone());
    renamed_core.set_file_name("renamed-a5-ginga.jtd");
    assert_eq!(renamed_core.writing_mode(), WritingMode::VerticalRl);
    assert_eq!(renamed_core.page_count(), 72);
    assert!((renamed_core.page_width_px() - 559.4).abs() < 0.2);
    assert!((renamed_core.page_height_px() - 793.7).abs() < 0.2);
    let renamed_document_info = renamed_core.get_document_info();
    assert!(
        renamed_document_info
            .contains("\"writingModeCandidateFromDocumentViewStyles\":\"vertical-rl\"")
    );
    assert!(
        renamed_document_info.contains(
            "\"writingModeCandidateFromDocumentViewStylesFirstRecordCodeHex\":\"0x1001\""
        )
    );
    assert!(renamed_document_info.contains(
        "\"writingModeDecision\":{\"selected\":\"vertical-rl\",\"source\":\"source-document-layout-hint\""
    ));
    assert!(
        renamed_document_info.contains("\"sourceDocumentLayoutHintCandidate\":\"vertical-rl\"")
    );
    assert!(
        renamed_document_info
            .contains("\"sourceDocumentLayoutHintBasis\":\"ginga-front-matter-evidence\"")
    );
    assert!(renamed_document_info.contains("\"paperMarkCandidate\":\"vertical-rl\""));
    assert!(renamed_document_info.contains("\"paperMarkDisagreesWithSelected\":false"));

    let mut core = DocumentCore::from_document(document);
    core.set_file_name(sample_path.to_string_lossy());

    assert_eq!(core.page_count(), 72);

    let page_six = core.render_page_svg(5).unwrap();
    assert!(page_six.contains("class=\"rjtd-page-number\""));
    assert!(page_six.contains("data-side=\"left\""));
    assert!(page_six.contains(">6</text>"));
    assert!(page_six.contains("class=\"rjtd-running-header\""));
    assert!(page_six.contains("一、午后の授業"));
    let page_six_header = running_header_svg_element(&page_six);
    assert!(page_six_header.contains("text-anchor=\"start\""));
    assert!(!page_six_header.contains("writing-mode=\"vertical-rl\""));

    let page_seven = core.render_page_svg(6).unwrap();
    assert!(page_seven.contains("data-side=\"right\""));
    assert!(page_seven.contains(">7</text>"));
    assert!(page_seven.contains("銀河鉄道の夜"));
    let page_seven_header = running_header_svg_element(&page_seven);
    assert!(page_seven_header.contains("text-anchor=\"end\""));
    assert!(!page_seven_header.contains("writing-mode=\"vertical-rl\""));

    let page_six_layer_tree = core.get_page_layer_tree(5).unwrap();
    assert_json_brackets_balanced(&page_six_layer_tree);
    assert!(page_six_layer_tree.contains("]},\"textSources\""));
    assert!(page_six_layer_tree.contains("\"type\":\"pageDecoration\""));
    assert!(
        page_six_layer_tree
            .contains("\"source\":\"autoTextInfo+pageLayoutStylePairedSlots+documentText\"")
    );
    assert!(page_six_layer_tree.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\""));
    assert!(page_six_layer_tree.contains("\"sidePolicyDecoded\":false"));
    assert!(page_six_layer_tree.contains("\"facingPagesCandidate\":true"));
    assert!(
        page_six_layer_tree.contains(
            "\"pairedSlotPairs\":[\"0x32/0x33\",\"0x34/0x35\",\"0x36/0x37\",\"0x38/0x39\"]"
        )
    );
    assert!(page_six_layer_tree.contains("\"slotEvidence\""));
    assert!(page_six_layer_tree.contains("\"slot\":\"0x32\""));
    assert!(page_six_layer_tree.contains("\"part05First\":\"0x04\""));
    assert!(page_six_layer_tree.contains("\"part05NonZero\":true"));
    assert!(page_six_layer_tree.contains("\"part06Hex\":\"03020a0003e8\""));
    assert!(page_six_layer_tree.contains("\"side\":\"left\""));
    assert!(page_six_layer_tree.contains("\"bbox\":{\"x\":72.000"));
    assert!(page_six_layer_tree.contains("\"pageNumber\":6"));
    assert!(page_six_layer_tree.contains("\"headerText\":\"一、午后の授業\""));
    assert!(
        page_six_layer_tree.contains("\"layoutMarkEvidence\":{\"source\":\"/PageMark+/PaperMark\"")
    );
    assert!(page_six_layer_tree.contains("\"pageIndex\":5"));
    assert!(page_six_layer_tree.contains("\"pageMarkEntryIndex\":5"));
    assert!(page_six_layer_tree.contains("\"pageMarkLineStart\":23"));
    assert!(page_six_layer_tree.contains("\"pageMarkLineEnd\":40"));
    assert!(page_six_layer_tree.contains("\"pageMarkU16Fields\":[0,5,1,0,0,23,0,40,0,0,353"));
    assert!(
        page_six_layer_tree.contains("\"pageMarkU16GeometryHypotheses\":{\"source\":\"/PageMark\"")
    );
    assert!(page_six_layer_tree.contains(
        "\"word20Is0x00ff\":false,\"word13PlusWord14\":599,\"word13PlusWord14EqualsWord21\":true"
    ));
    assert!(page_six_layer_tree.contains(
        "\"word21MinusWord13\":246,\"word21MinusWord13EqualsWord14\":true,\"word19EqualsWord13\":true,\"selectedFieldsAllZero\":false,\"nonZeroAdditiveUnitCandidate\":true"
    ));
    assert!(page_six_layer_tree.contains(
        "\"layoutComparisons\":{\"pageWidthPx\":559.370,\"pageHeightPx\":793.701,\"pageMarginPx\":72.000,\"bodyWidthPx\":415.370"
    ));
    assert!(page_six_layer_tree.contains(
        "\"pagePitchEvidence\":{\"source\":\"/PageMark+PageLayout\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false,\"pageMarkEntryIndex\":5,\"pageMarkIndex\":5,\"lineStart\":23,\"lineEnd\":40,\"lineCount\":18,\"lineGapCount\":17"
    ));
    assert!(page_six_layer_tree.contains(
        "\"pageHeightPxPerLineCount\":44.094,\"pageHeightPxPerLineGap\":46.688,\"bodyHeightPxPerLineCount\":36.094,\"bodyHeightPxPerLineGap\":38.218"
    ));
    assert!(page_six_layer_tree.contains(
        "\"linePitchAgreementGate\":{\"source\":\"/PageMark body line-gap pitch+source row height\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"rowHeightCandidatePresent\":false,\"rowHeightPx\":null,\"rowHeightBasis\":null"
    ));
    assert!(page_six_layer_tree.contains(
        "\"pitchAgreementReady\":false,\"renderPromotionContribution\":\"page-mark-line-pitch-agreement-candidate\",\"renderPromotionBlockedReason\":\"source-row-height-candidate-absent\""
    ));
    assert!(page_six_layer_tree.contains(
        "\"pageMarkSelectedFields\":{\"source\":\"/PageMark\",\"entryIndex\":5,\"lineStart\":23,\"lineEnd\":40,\"lineCount\":18,\"lineGapCount\":17,\"u16GeometryClass\":\"additive-row\""
    ));
    assert!(page_six_layer_tree.contains(
        "\"renderPromotionContribution\":\"page-mark-line-gap-pitch-diagnostic-only\",\"renderPromotionBlockedReason\":\"page-mark-line-pitch-semantics-unproven\""
    ));
    assert!(page_six_layer_tree.contains("\"paperMarkEntryIndex\":5"));
    assert!(page_six_layer_tree.contains("\"paperMarkFlagsHex\":\"0x00010010\""));
    assert!(page_six_layer_tree.contains("\"rowIndexAligned\":true"));
    assert!(page_six_layer_tree.contains("\"markIndexAligned\":true"));
    assert!(page_six_layer_tree.contains("\"entryCountAligned\":true"));
    assert!(
        page_six_layer_tree
            .contains("\"renderPromotionContribution\":\"page-row-association-evidence-only\"")
    );
    assert!(
        page_six_layer_tree
            .contains("\"renderPromotionBlockedReason\":\"paper-mark-flag-semantics-undecoded\"")
    );
    let page_six_info = core.get_page_info(5).unwrap();
    assert!(page_six_info.contains("\"layoutMarkEvidence\":{\"source\":\"/PageMark+/PaperMark\""));
    assert!(page_six_info.contains("\"pageMarkEntryIndex\":5"));
    assert!(page_six_info.contains("\"pageMarkU16Fields\":[0,5,1,0,0,23,0,40,0,0,353"));
    assert!(page_six_info.contains(
        "\"word20Is0x00ff\":false,\"word13PlusWord14\":599,\"word13PlusWord14EqualsWord21\":true"
    ));
    assert!(page_six_info.contains(
        "\"word21MinusWord13\":246,\"word21MinusWord13EqualsWord14\":true,\"word19EqualsWord13\":true,\"selectedFieldsAllZero\":false,\"nonZeroAdditiveUnitCandidate\":true"
    ));
    assert!(page_six_info.contains(
        "\"pagePitchEvidence\":{\"source\":\"/PageMark+PageLayout\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false"
    ));
    assert!(page_six_info.contains(
        "\"lineStart\":23,\"lineEnd\":40,\"lineCount\":18,\"lineGapCount\":17,\"pageSizePx\":{\"width\":559.370,\"height\":793.701},\"bodySizePx\":{\"width\":415.370,\"height\":649.701},\"marginPx\":72.000"
    ));
    assert!(page_six_info.contains(
        "\"pageHeightPxPerLineCount\":44.094,\"pageHeightPxPerLineGap\":46.688,\"bodyHeightPxPerLineCount\":36.094,\"bodyHeightPxPerLineGap\":38.218"
    ));
    assert!(page_six_info.contains(
        "\"linePitchAgreementGate\":{\"source\":\"/PageMark body line-gap pitch+source row height\""
    ));
    assert!(page_six_info.contains(
        "\"rowHeightCandidatePresent\":false,\"rowHeightPx\":null,\"rowHeightBasis\":null"
    ));
    assert!(page_six_info.contains(
        "\"pitchAgreementReady\":false,\"renderPromotionContribution\":\"page-mark-line-pitch-agreement-candidate\",\"renderPromotionBlockedReason\":\"source-row-height-candidate-absent\""
    ));
    assert!(page_six_info.contains("\"paperMarkEntryIndex\":5"));
    assert!(page_six_info.contains("\"paperMarkFlagsHex\":\"0x00010010\""));
    assert!(page_six_info.contains("\"rowIndexAligned\":true"));
    assert!(page_six_info.contains("\"markIndexAligned\":true"));
    assert!(page_six_info.contains("\"entryCountAligned\":true"));
    let document_info = core.get_document_info();
    assert!(document_info.contains("\"pageMarkCount\":1"));
    assert!(document_info.contains("\"family\":\"fixed84\""));
    assert!(document_info.contains("\"entryCount\":75"));
    assert!(document_info.contains("\"lineStart\":23"));
    assert!(document_info.contains("\"paperMarkCount\":1"));
    assert!(document_info.contains("\"sourceStream\":\"/PaperMark\""));
    assert!(document_info.contains("\"headerStride\":12"));
    assert!(document_info.contains("\"flagsHex\":\"0x00010010\""));
    assert!(document_info.contains("\"writingModeCandidateFromPaperMark\":\"vertical-rl\""));
    assert!(document_info.contains("\"writingModeCandidateDecoded\":false"));
    assert!(document_info.contains("\"paperMarkFlagBit0VerticalCandidate\":true"));
    assert!(document_info.contains("\"paperMarkFlagBit17IndexStepCandidate\":false"));
    assert!(document_info.contains(
        "\"paperMarkWritingModeCandidateEvidence\":[\"paper-mark-flag-bit0-vertical-corpus-consistent\"]"
    ));
    assert!(document_info.contains(
        "\"paperMarkWritingModeCandidateBlockers\":[\"paper-mark-writing-mode-flag-semantics-unproven\"]"
    ));

    let page_seven_layer_tree = core.get_page_layer_tree(6).unwrap();
    assert!(page_seven_layer_tree.contains("\"side\":\"right\""));
    assert!(page_seven_layer_tree.contains("\"bbox\":{\"x\":487.370"));
    assert!(page_seven_layer_tree.contains("\"pageNumber\":7"));
    assert!(page_seven_layer_tree.contains("\"headerText\":\"銀河鉄道の夜\""));

    let page_six_lines = core.page_text_lines(5).unwrap();
    assert_eq!(page_six_lines[0].text(), "");
    assert_eq!(page_six_lines[1].text(), "");
    assert_eq!(page_six_lines[2].text(), "一、午后の授業");
    assert_eq!(page_six_lines[3].text(), "");
    assert_eq!(page_six_lines[4].text(), "");
    assert!(
        page_six_lines
            .iter()
            .any(|line| line.text().contains("大きな望遠鏡"))
    );
    assert!(
        !page_six_lines
            .iter()
            .any(|line| line.text().contains("やっぱり星だ"))
    );
    let page_seven_lines = core.page_text_lines(6).unwrap();
    assert!(
        page_seven_lines
            .iter()
            .any(|line| line.text().contains("やっぱり星だ"))
    );

    let toc_page = core.page_text_lines(2).unwrap();
    assert!(toc_page.iter().any(|line| line.text().contains('…')));
    assert!(toc_page.iter().any(|line| line.text().contains("42")));
    let toc_svg = core.render_page_svg(2).unwrap();
    assert!(toc_svg.contains("…"));
    assert!(toc_svg.contains("42"));
    assert!(toc_svg.contains("ごご"));
    assert!(toc_svg.contains("きっぷ"));

    let final_page = core.render_page_svg(71).unwrap();
    assert!(final_page.contains("銀河鉄道の夜"));
    assert!(!final_page.contains("︂"));
    assert!(!final_page.contains("class=\"rjtd-page-number\""));
    assert!(!final_page.contains("class=\"rjtd-running-header\""));
    let final_page_lines = core.page_text_lines(71).unwrap();
    assert_eq!(final_page_lines.len(), 16);
    assert_eq!(final_page_lines[0].text(), "銀河鉄道の夜");
    assert_eq!(final_page_lines[1].text(), "");
    assert!(final_page_lines[2].text().contains("初版発行"));
    assert_eq!(final_page_lines[3].text(), "");
    assert!(final_page_lines[11].text().contains("Printed in Japan"));
    assert_eq!(final_page_lines[12].text(), "");
    assert_eq!(
        final_page_lines[13].text(),
        "※弊社から販売・流通をご希望の場合は、記載事項に"
    );
    assert_eq!(
        final_page_lines[14].text(),
        "規定がございます。「流通なし」の場合は、ご自由に"
    );
    assert_eq!(final_page_lines[15].text(), "記載していただけます。");

    let final_page_layer_tree = core.get_page_layer_tree(71).unwrap();
    assert_json_brackets_balanced(&final_page_layer_tree);
    assert!(final_page_layer_tree.contains("\"x\":429.870"));
    assert!(final_page_layer_tree.contains("\"y\":380.976"));
    assert!(!final_page_layer_tree.contains("︂"));
}

#[test]
fn document_core_preserves_tabs_as_visible_svg_spacing() {
    assert_eq!(display_column_width('\t'), APP_TAB_COLUMNS);
    assert_eq!(svg_visual_text("A\tB"), "A　　B");
}

#[test]
fn document_core_renders_ruby_annotations_in_svg_and_layer_tree() {
    let bytes = cfb_with_document_text(document_text_with_ruby());
    let mut core = DocumentCore::from_bytes(&bytes).unwrap();
    core.set_writing_mode(WritingMode::VerticalRl);

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-ruby\""));
    assert!(svg.contains("ごご"));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"rubyText\":\"ごご\""));
    assert!(layer_tree.contains("\"type\":\"ruby\""));
}

#[test]
fn document_core_reports_jtd_validation_warnings() {
    let empty = DocumentCore::from_document(Document::default());
    assert_eq!(
        empty.get_validation_warnings(),
        "{\"count\":0,\"summary\":{},\"warnings\":[]}"
    );

    let core = DocumentCore::from_document(Document::from_plain_text("銀河鉄道\n午后の授業"));
    let warnings = core.get_validation_warnings();

    assert!(warnings.contains("\"count\":2"));
    assert!(warnings.contains("\"JTD text layout uses fallback pagination\":2"));
    assert!(warnings.contains("\"kind\":\"JtdFallbackTextPagination\""));
    assert!(warnings.contains("\"section\":0,\"paragraph\":0"));
    assert!(warnings.contains("\"section\":0,\"paragraph\":1"));
    assert!(warnings.contains("\"cell\":null"));
}

#[test]
fn parser_surfaces_preserved_jtd_data_as_validation_warnings() {
    let position_table = text_count_table_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            &position_table,
        ),
        (rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH, &[1, 2, 3]),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    let warnings = core.get_validation_warnings();

    assert!(warnings.contains("\"count\":5"));
    assert!(warnings.contains("\"JTD text layout uses fallback pagination\":1"));
    assert!(warnings.contains("\"JTD raw stream preserved but not decoded\":1"));
    assert!(warnings.contains("\"JTD style stream preserved but not decoded\":1"));
    assert!(warnings.contains("\"JTD text-count range preserved as diagnostic data\":2"));
    assert!(warnings.contains("\"kind\":\"JtdRawStreamPreserved\""));
    assert!(warnings.contains("\"kind\":\"JtdUnknownStylePreserved\""));
    assert!(warnings.contains("\"kind\":\"JtdTextCountRangeDiagnosticOnly\""));
}

#[test]
fn image_payload_dimensions_reads_jpeg_sof_metadata() {
    let payload = minimal_jpeg_payload();

    let dimensions = jpeg_payload_dimensions(payload).unwrap();
    assert_eq!(dimensions.width(), 32);
    assert_eq!(dimensions.height(), 16);
    assert_eq!(image_payload_dimensions(payload), Some(dimensions));
    assert_eq!(jpeg_payload_end(payload, 0), Some(payload.len()));
    assert_eq!(
        jpeg_payload_end(b"\xff\xd8\xff\xff\xff\xfc\0\0\0\0\xff\xd9", 0),
        None
    );
}

#[test]
#[cfg(feature = "bitmap-images")]
fn document_core_projects_complete_image_payloads_as_diagnostic_svg_overlays() {
    let image_stream_path = "/EmbedItems/Embedding 1/Contents";
    let png_payload = minimal_png_payload();
    let (mut image_payload, _, _) = image_payload_with_header_fixture(png_payload.len());
    image_payload.extend_from_slice(png_payload);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (image_stream_path, &image_payload),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-image-payload-diagnostic\""));
    assert!(svg.contains("data:image/png;base64,"));
    assert!(svg.contains("data-decoded=\"false\""));
    assert!(svg.contains("data-geometry-decoded=\"false\""));
    assert!(svg.contains("data-placement-proven=\"false\""));
    assert!(svg.contains("data-diagnostic-only=\"true\""));
    assert!(svg.contains("data-diagnostic-renderable=\"true\""));
    assert!(svg.contains("data-renderable=\"false\""));
    assert!(svg.contains("data-source-path-candidate-present=\"true\""));
    assert!(svg.contains("data-declared-payload-length-present=\"true\""));
    assert!(svg.contains("data-ownership-reference-count=\"0\""));
    assert!(svg.contains("data-ownership-evidence-ready=\"false\""));
    assert!(svg.contains("data-frame-reference-row-count=\"0\""));
    assert!(svg.contains("data-frame-coordinate-row-count=\"0\""));
    assert!(svg.contains("data-frame-linked-window-row-count=\"0\""));
    assert!(svg.contains("data-frame-geometry-candidate-present=\"false\""));
    assert!(svg.contains("data-embedding-frame-trace-present=\"false\""));
    assert!(svg.contains("data-source-frame-record-geometry-present=\"false\""));
    assert!(svg.contains("data-candidate-frame-bbox-present=\"false\""));
    assert!(svg.contains("data-candidate-frame-x=\"null\""));
    assert!(svg.contains("data-candidate-frame-y=\"null\""));
    assert!(svg.contains("data-candidate-frame-width=\"null\""));
    assert!(svg.contains("data-candidate-frame-height=\"null\""));
    assert!(svg.contains("data-payload-frame-aspect-fit-present=\"false\""));
    assert!(svg.contains("data-payload-frame-aspect-delta-permille=\"null\""));
    assert!(svg.contains("data-best-payload-frame-aspect-delta-permille=\"null\""));
    assert!(svg.contains("data-current-payload-best-frame-aspect-candidate=\"false\""));
    assert!(svg.contains(
        "data-render-promotion-blocked-reason=\"image-payload-cross-stream-ownership-reference-missing\""
    ));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"imagePayloadDiagnostic\""));
    assert!(layer_tree.contains("\"sourcePath\":\"/EmbedItems/Embedding 1/Contents\""));
    assert!(layer_tree.contains("\"projectionKind\":\"diagnosticProjection\""));
    assert!(layer_tree.contains("\"placementProven\":false"));
    assert!(layer_tree.contains("\"diagnosticOnly\":true"));
    assert!(layer_tree.contains("\"diagnosticRenderable\":true"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains("\"sourcePathCandidatePresent\":true"));
    assert!(layer_tree.contains("\"declaredPayloadLengthPresent\":true"));
    assert!(layer_tree.contains("\"ownershipReferenceCount\":0"));
    assert!(layer_tree.contains("\"ownershipEvidenceReady\":false"));
    assert!(layer_tree.contains("\"frameReferenceRowCount\":0"));
    assert!(layer_tree.contains("\"frameCoordinateRowCount\":0"));
    assert!(layer_tree.contains("\"frameLinkedWindowRowCount\":0"));
    assert!(layer_tree.contains("\"frameGeometryCandidatePresent\":false"));
    assert!(layer_tree.contains("\"embeddingFrameTracePresent\":false"));
    assert!(layer_tree.contains("\"sourceFrameRecordGeometryPresent\":false"));
    assert!(layer_tree.contains("\"sourceFrameTrace\":"));
    assert!(layer_tree.contains("\"embeddingFramePresent\":false"));
    assert!(layer_tree.contains("\"frameRecordPresent\":false"));
    assert!(layer_tree.contains("\"frameRecordGeometry\":null"));
    assert!(layer_tree.contains("\"candidateFrameBBox\":null"));
    assert!(layer_tree.contains("\"payloadFrameAspectFit\":null"));
    assert!(layer_tree.contains("\"ownershipProven\":false"));
    assert!(layer_tree.contains("\"pageGeometryProven\":false"));
    assert!(layer_tree.contains("\"paintOrderDecoded\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"image-payload-cross-stream-ownership-reference-missing\""
    ));
    assert!(layer_tree.contains("\"objectEnvelope\":{\"headerStart\":0"));
    assert!(layer_tree.contains("\"headerFields\""));
    assert!(layer_tree.contains("\"sourcePathCandidate\""));
    assert!(layer_tree.contains("\"decoded\":false"));

    let overlay_images = core.get_page_overlay_images(0).unwrap();
    assert!(overlay_images.contains("\"type\":\"jtdImagePayloadCandidate\""));
    assert!(overlay_images.contains("\"sourcePath\":\"/EmbedItems/Embedding 1/Contents\""));
    assert!(overlay_images.contains("\"placementProven\":false"));
    assert!(overlay_images.contains("\"geometryDecoded\":false"));
    assert!(overlay_images.contains("\"diagnosticOnly\":true"));
    assert!(overlay_images.contains("\"diagnosticRenderable\":true"));
    assert!(overlay_images.contains("\"renderable\":false"));
    assert!(overlay_images.contains("\"sourcePathCandidatePresent\":true"));
    assert!(overlay_images.contains("\"declaredPayloadLengthPresent\":true"));
    assert!(overlay_images.contains("\"ownershipReferenceCount\":0"));
    assert!(overlay_images.contains("\"ownershipEvidenceReady\":false"));
    assert!(overlay_images.contains("\"frameReferenceRowCount\":0"));
    assert!(overlay_images.contains("\"frameCoordinateRowCount\":0"));
    assert!(overlay_images.contains("\"frameLinkedWindowRowCount\":0"));
    assert!(overlay_images.contains("\"frameGeometryCandidatePresent\":false"));
    assert!(overlay_images.contains("\"embeddingFrameTracePresent\":false"));
    assert!(overlay_images.contains("\"sourceFrameRecordGeometryPresent\":false"));
    assert!(overlay_images.contains("\"sourceFrameTrace\":"));
    assert!(overlay_images.contains("\"embeddingFramePresent\":false"));
    assert!(overlay_images.contains("\"frameRecordPresent\":false"));
    assert!(overlay_images.contains("\"frameRecordGeometry\":null"));
    assert!(overlay_images.contains("\"candidateFrameBBox\":null"));
    assert!(overlay_images.contains("\"payloadFrameAspectFit\":null"));
    assert!(overlay_images.contains("\"ownershipProven\":false"));
    assert!(overlay_images.contains("\"pageGeometryProven\":false"));
    assert!(overlay_images.contains("\"paintOrderDecoded\":false"));
    assert!(overlay_images.contains(
        "\"renderPromotionBlockedReason\":\"image-payload-cross-stream-ownership-reference-missing\""
    ));
    assert!(overlay_images.contains("\"objectEnvelope\":{\"headerStart\":0"));
    assert!(overlay_images.contains("\"decoded\":false"));
}

#[test]
#[cfg(feature = "bitmap-images")]
fn image_payload_render_gate_preserves_source_frame_trace_without_promotion() {
    let image_stream_path = "/EmbedItems/Embedding 24/Contents";
    let png_payload = minimal_png_payload();
    let (mut image_payload, _, _) = image_payload_with_header_fixture(png_payload.len());
    image_payload.extend_from_slice(png_payload);

    let mut frame = frame_stream_fixture();
    frame[7..9].copy_from_slice(&24u16.to_be_bytes());
    let embedding_info = embedding_info_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (image_stream_path, &image_payload),
        (EMBEDDING_INFO_PATH, &embedding_info),
        ("/Frame", &frame),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"imagePayloadDiagnostic\""));
    assert!(layer_tree.contains("\"ownershipReferenceCount\":"));
    assert!(layer_tree.contains("\"ownershipEvidenceReady\":true"));
    assert!(layer_tree.contains("\"ownershipProven\":true"));
    assert!(layer_tree.contains("\"frameReferenceRowCount\":"));
    assert!(layer_tree.contains("\"frameCoordinateRowCount\":"));
    assert!(layer_tree.contains("\"frameGeometryCandidatePresent\":true"));
    assert!(layer_tree.contains("\"embeddingFrameTracePresent\":true"));
    assert!(layer_tree.contains("\"sourceFrameRecordGeometryPresent\":true"));
    assert!(layer_tree.contains("\"sourceFrameTrace\":"));
    assert!(layer_tree.contains("\"ownershipEmbeddingIndex\":24"));
    assert!(layer_tree.contains("\"embeddingFrameRef\":1"));
    assert!(layer_tree.contains("\"frameRecordPresent\":true"));
    assert!(layer_tree.contains("\"frameRecordGeometry\":{\"sourcePath\":\"/Frame\""));
    assert!(layer_tree.contains("\"width\":13260"));
    assert!(layer_tree.contains("\"height\":1327"));
    assert!(
        layer_tree.contains("\"candidateFrameBBox\":{\"source\":\"EmbeddingInfo+/FrameRecord\"")
    );
    assert!(
        layer_tree.contains(
            "\"renderPromotionBlockedReason\":\"page-assignment-and-paint-order-unproven\""
        )
    );
    assert!(
        layer_tree.contains(
            "\"payloadFrameAspectFit\":{\"source\":\"imagePayloadDimensions+/FrameRecord\""
        )
    );
    assert!(layer_tree.contains("\"payloadWidth\":1"));
    assert!(layer_tree.contains("\"payloadHeight\":1"));
    assert!(layer_tree.contains("\"aspectDeltaPermille\":899"));
    assert!(layer_tree.contains("\"bestPayloadAspectDeltaPermille\":899"));
    assert!(layer_tree.contains("\"currentPayloadBestFrameAspectCandidate\":true"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"payload-selection-page-assignment-and-paint-order-unproven\""
    ));
    assert!(layer_tree.contains("\"pageGeometryProven\":false"));
    assert!(layer_tree.contains("\"paintOrderDecoded\":false"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven\""
    ));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("data-ownership-proven=\"true\""));
    assert!(svg.contains("data-frame-reference-row-count=\""));
    assert!(svg.contains("data-frame-coordinate-row-count=\""));
    assert!(svg.contains("data-frame-geometry-candidate-present=\"true\""));
    assert!(svg.contains("data-embedding-frame-trace-present=\"true\""));
    assert!(svg.contains("data-source-frame-record-geometry-present=\"true\""));
    assert!(svg.contains("data-candidate-frame-bbox-present=\"true\""));
    assert!(!svg.contains("data-candidate-frame-x=\"null\""));
    assert!(!svg.contains("data-candidate-frame-y=\"null\""));
    assert!(!svg.contains("data-candidate-frame-width=\"null\""));
    assert!(!svg.contains("data-candidate-frame-height=\"null\""));
    assert!(svg.contains("data-payload-frame-aspect-fit-present=\"true\""));
    assert!(svg.contains("data-payload-frame-aspect-delta-permille=\"899\""));
    assert!(svg.contains("data-best-payload-frame-aspect-delta-permille=\"899\""));
    assert!(svg.contains("data-current-payload-best-frame-aspect-candidate=\"true\""));
    assert!(svg.contains("data-renderable=\"false\""));
    assert!(svg.contains(
        "data-render-promotion-blocked-reason=\"image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven\""
    ));

    let overlay_images = core.get_page_overlay_images(0).unwrap();
    assert!(overlay_images.contains("\"ownershipProven\":true"));
    assert!(overlay_images.contains("\"frameGeometryCandidatePresent\":true"));
    assert!(overlay_images.contains("\"embeddingFrameTracePresent\":true"));
    assert!(overlay_images.contains("\"sourceFrameRecordGeometryPresent\":true"));
    assert!(overlay_images.contains("\"frameRecordGeometry\":{\"sourcePath\":\"/Frame\""));
    assert!(
        overlay_images
            .contains("\"candidateFrameBBox\":{\"source\":\"EmbeddingInfo+/FrameRecord\"")
    );
    assert!(
        overlay_images.contains(
            "\"payloadFrameAspectFit\":{\"source\":\"imagePayloadDimensions+/FrameRecord\""
        )
    );
    assert!(overlay_images.contains("\"currentPayloadBestFrameAspectCandidate\":true"));
    assert!(overlay_images.contains("\"renderable\":false"));
}

#[test]
fn parser_preserves_object_stream_candidates_as_model_evidence() {
    let image_stream_path = "/EmbedItems/Embedding 3/Contents";
    let jpeg_payload = minimal_jpeg_payload();
    let (mut image_payload, signature_offset, payload_end) =
        image_payload_with_header_fixture(jpeg_payload.len());
    image_payload.extend_from_slice(jpeg_payload);
    image_payload.extend_from_slice(b"tail");
    let so_offset = image_payload.len();
    image_payload.extend_from_slice(b"SO\0\0");
    let svg_payload = b"<svg viewBox=\"0 0 10 10\"></svg>".to_vec();
    let figure_reference_payload = b"\x03\0\0\0ref\0\x03".to_vec();
    let mut jsfart_payload = Vec::new();
    for code_unit in "JSFART.OBJECT".encode_utf16() {
        jsfart_payload.extend_from_slice(&code_unit.to_le_bytes());
    }
    jsfart_payload.extend_from_slice(&[0x11, 0x22, 0x33, 0x44]);
    let frame_suffix_row = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
    ];
    let mut frame_payload = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00];
    frame_payload.extend_from_slice(&frame_suffix_row);
    frame_payload.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    frame_payload.extend_from_slice(&frame_suffix_row);
    let figure_link_payload = [
        0x00, 0x0b, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x16, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04,
        0x00, 0x16, 0x00, 0x00, 0x00, 0x08,
    ];
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (image_stream_path, &image_payload),
        ("/FigureData/main_data/FDMVector", &figure_reference_payload),
        (
            "/FigureData/ExpandData/main_data/Link",
            &figure_link_payload,
        ),
        ("/Frame", &frame_payload),
        ("/Vector.svg", &svg_payload),
        ("/VisualList", b"BMDV visual payload"),
        ("/EmbedItems/Embedding 1/JSFart2Contents", &jsfart_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.object_stream_candidates().len(), 7);
    let image_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == image_stream_path)
        .unwrap();
    assert_eq!(image_candidate.size(), image_payload.len());
    assert!(
        image_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::ObjectPath)
    );
    assert!(
        image_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::ImageSignature)
    );
    assert!(
        image_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::SoMarker)
    );
    let ownership = image_candidate.ownership_candidate().unwrap();
    assert_eq!(ownership.basis(), "stream-path");
    assert_eq!(ownership.family(), "embed-items");
    assert_eq!(ownership.storage_path(), Some("/EmbedItems/Embedding 3"));
    assert_eq!(ownership.embedding_index(), Some(3));
    assert_eq!(ownership.stream_role(), "contents");
    assert_eq!(image_candidate.image_signature_hits()[0].kind(), "jpeg");
    assert_eq!(
        image_candidate.image_signature_hits()[0].offset(),
        signature_offset
    );
    assert_eq!(image_candidate.image_payload_spans().len(), 1);
    let image_span = &image_candidate.image_payload_spans()[0];
    assert_eq!(image_span.kind(), "jpeg");
    assert_eq!(image_span.mime(), "image/jpeg");
    assert_eq!(image_span.signature_offset(), signature_offset);
    assert_eq!(image_span.start(), signature_offset);
    assert_eq!(image_span.end(), payload_end);
    assert_eq!(image_span.len(), jpeg_payload.len());
    assert!(image_span.complete());
    assert_eq!(
        image_span.dimensions(),
        Some(ObjectImageDimensions::new(32, 16))
    );
    assert_eq!(
        image_span.payload(),
        &image_payload[signature_offset..payload_end]
    );
    assert_eq!(image_span.envelope().header_start(), 0);
    assert_eq!(image_span.envelope().header_end(), signature_offset);
    assert_eq!(
        image_span.envelope().header(),
        &image_payload[..signature_offset]
    );
    assert_eq!(image_span.envelope().trailer_start(), payload_end);
    assert_eq!(image_span.envelope().trailer_end(), image_payload.len());
    assert_eq!(
        image_span.envelope().trailer(),
        &image_payload[payload_end..]
    );
    let declared_length = image_span.envelope().declared_payload_length().unwrap();
    assert_eq!(declared_length.offset(), signature_offset - 4);
    assert_eq!(declared_length.value(), jpeg_payload.len());
    assert_eq!(declared_length.endian(), "le32");
    let header_fields = image_span.envelope().header_fields();
    assert_eq!(header_fields.u16_le_prefix()[0].value(), 9);
    assert_eq!(header_fields.u16_le_prefix()[1].value(), 1);
    assert_eq!(header_fields.u32_le_prefix()[0].value(), 0x0001_0009);
    let source_path = header_fields.source_path_candidate().unwrap();
    assert_eq!(source_path.length_offset(), 16);
    assert_eq!(source_path.declared_length(), b"C:\\TEMP\\A.JPG".len());
    assert_eq!(source_path.bytes_start(), 17);
    assert_eq!(source_path.text_lossy(), "C:\\TEMP\\A.JPG");
    assert_eq!(image_candidate.so_offsets(), &[so_offset]);
    assert_eq!(
        image_candidate.payload_prefix(),
        &image_payload[..image_payload.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
    );
    let references = image_candidate.ownership_reference_candidates();
    assert!(references.iter().any(|reference| {
        reference.target_path() == "/FigureData/main_data/FDMVector"
            && reference.encoding() == "u32-le"
            && reference.total_matches() == 1
            && reference.offsets() == [0]
    }));
    let frame_rows = image_candidate.frame_reference_row_candidates();
    assert_eq!(frame_rows.len(), 2);
    assert_eq!(frame_rows[0].target_path(), "/Frame");
    assert_eq!(frame_rows[0].encoding(), "u16-be");
    assert_eq!(frame_rows[0].stride(), 20);
    assert_eq!(frame_rows[0].field_offset(), 15);
    assert_eq!(frame_rows[0].offset(), 15);
    assert_eq!(frame_rows[0].row_start(), 0);
    assert_eq!(frame_rows[0].family(), "frame-index-tail-window20");
    let suffix_link = frame_rows[0].suffix_link().unwrap();
    assert_eq!(suffix_link.relation(), "same-candidate");
    assert_eq!(
        suffix_link.suffix_family(),
        "frame-index-tail-coordinate-row12"
    );
    assert_eq!(suffix_link.matched_row_start(), 24);
    assert_eq!(suffix_link.matched_row_index(), 2);
    assert_eq!(frame_rows[1].stride(), 12);
    assert_eq!(frame_rows[1].field_offset(), 7);
    assert_eq!(frame_rows[1].family(), "frame-index-tail-coordinate-row12");

    let svg_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/Vector.svg")
        .unwrap();
    assert!(
        svg_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::ShapePath)
    );
    assert!(
        svg_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::SvgSignature)
    );
    assert_eq!(svg_candidate.svg_offsets(), &[0]);

    let visual_list_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/VisualList")
        .unwrap();
    assert!(
        visual_list_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::VisualListPath)
    );
    assert_eq!(visual_list_candidate.payload_prefix(), b"BMDV visual payl");

    let jsfart_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 1/JSFart2Contents")
        .unwrap();
    assert!(
        jsfart_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::ObjectPath)
    );
    assert!(jsfart_candidate.jsfart_art_candidate().is_none());
    let jsfart_profile = jsfart_candidate
        .jsfart_stream_profile_candidate()
        .expect("non-MSTUDIO JSFart2Contents should still preserve a source profile");
    assert_eq!(jsfart_profile.magic_family(), "jsfart-object-utf16le");
    assert_eq!(jsfart_profile.magic_family_hex(), "4a00");
    assert_eq!(jsfart_profile.magic_offset(), 0);
    assert_eq!(jsfart_profile.magic_ascii_or_utf16_preview(), "JSFART.O");
    assert_eq!(
        jsfart_profile.header_prefix(),
        &jsfart_payload[..jsfart_payload.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
    );
    assert!(!jsfart_profile.structured_art_candidate_present());
    assert_eq!(
        jsfart_profile.render_promotion_blocked_reason(),
        "jsfart-variant-layout-undecoded"
    );
    let object_json = object_stream_candidates_json(document.object_stream_candidates());
    assert!(object_json.contains("\"jsfartStreamProfile\":{\"format\":\"JSFart2Contents\""));
    assert!(object_json.contains("\"magicFamily\":\"jsfart-object-utf16le\""));
    assert!(object_json.contains("\"magicFamilyHex\":\"4a00\""));
    assert!(object_json.contains("\"magicAsciiOrUtf16Preview\":\"JSFART.O\""));
    assert!(object_json.contains("\"structuredArtCandidatePresent\":false"));
    assert!(
        object_json
            .contains("\"renderPromotionBlockedReason\":\"jsfart-variant-layout-undecoded\"")
    );

    let link_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/ExpandData/main_data/Link")
        .unwrap();
    assert!(
        link_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::FigureLink)
    );
    let link = link_candidate.figure_link_candidate().unwrap();
    assert_eq!(link.header_words_be(), &[0x000b, 0x0001, 0x0000, 0x0002]);
    assert_eq!(link.declared_row_count_candidate(), Some(2));
    assert_eq!(link.row_stride(), 14);
    assert_eq!(link.rows().len(), 2);
    assert_eq!(link.rows()[0].row_index(), 0);
    assert_eq!(link.rows()[0].row_start(), 8);
    assert_eq!(
        link.rows()[0].words_be(),
        &[0x0000, 0x0001, 0x0000, 0x0003, 0x0016, 0x0000, 0x0007]
    );
    assert_eq!(link.rows()[0].group_index_candidate(), Some(1));
    assert_eq!(link.rows()[0].source_id_candidate(), Some(3));
    assert_eq!(link.rows()[0].relation_kind_candidate(), Some(0x0016));
    assert_eq!(link.rows()[0].target_row_index_candidate(), Some(7));
}

#[test]
fn parser_decodes_bmdv_visual_list_metadata_and_projects_raster_layer() {
    let visual_list = visual_list_bmdv_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/VisualList", &visual_list),
    ]);

    let core = DocumentCore::from_bytes(&bytes).unwrap();
    let candidate = core
        .document()
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/VisualList")
        .unwrap();
    let visual_list = candidate.visual_list_candidate().unwrap();

    assert_eq!(visual_list.declared_size(), 88);
    assert_eq!(visual_list.magic_offset(), 4);
    assert_eq!(visual_list.magic(), "BMDV");
    assert_eq!(visual_list.version(), 1);
    assert_eq!(visual_list.width(), 10);
    assert_eq!(visual_list.height(), 2);
    assert_eq!(visual_list.row_stride(), 10);
    assert_eq!(visual_list.bit_depth(), 8);
    assert_eq!(visual_list.rle_data_offset(), 0x50);
    assert_eq!(visual_list.rle_data_len(), 8);
    assert_eq!(visual_list.pixels().len(), 20);
    assert_eq!(&visual_list.pixels()[..10], &[0x11; 10]);
    assert_eq!(&visual_list.pixels()[10..], &[0x22; 10]);

    let info = core.get_document_info();
    assert!(info.contains("\"visualList\":{\"format\":\"BMDV\""));
    assert!(info.contains("\"declaredSize\":88"));
    assert!(info.contains("\"rleEncoding\":\"bmp-rle8-like\""));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"visualListRasterDiagnostic\""));
    assert!(layer_tree.contains("\"projectionKind\":\"visualListRasterProjection\""));
    assert!(layer_tree.contains("\"sourcePath\":\"/VisualList\""));
    assert!(layer_tree.contains("\"naturalWidth\":10"));
    assert!(layer_tree.contains("\"naturalHeight\":2"));
    assert!(layer_tree.contains("\"placementProven\":true"));
    assert!(layer_tree.contains("\"decoded\":false"));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-visual-list-raster-diagnostic\""));
    assert!(svg.contains("data-source-path=\"/VisualList\""));
    assert!(svg.contains("data-projection=\"rle8-raster\""));
    assert!(svg.contains("data-fallback-projection=\"horizontal-runs\""));
    assert!(svg.contains("class=\"rjtd-visual-list-rle8-raster\""));
    assert!(svg.contains("data-projection=\"visualListRle8RasterImage\""));
    assert!(svg.contains("data-suppressed-dark-foreground=\"false\""));
    assert!(svg.contains("data:image/png;base64,"));
    assert!(svg.contains("data-format=\"BMDV\""));
}

#[test]
fn parser_preserves_embedding_info_frame_candidates_and_projects_diagnostics() {
    let embedding_info = embedding_info_fixture();
    let frame = frame_stream_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (EMBEDDING_INFO_PATH, &embedding_info),
        ("/Frame", &frame),
    ]);

    let document = parse_document(&bytes).unwrap();
    assert_eq!(document.object_embedding_frames().len(), 1);
    let frame = &document.object_embedding_frames()[0];
    assert_eq!(frame.source_path(), EMBEDDING_INFO_PATH);
    assert_eq!(frame.row_index(), 0);
    assert_eq!(frame.row_start(), EMBEDDING_INFO_HEADER_BYTES);
    assert_eq!(frame.embedding_index(), 24);
    assert_eq!(frame.class_name(), "JSFart.Art.2");
    assert_eq!(frame.primary_width(), 13260);
    assert_eq!(frame.primary_height(), 1327);
    assert_eq!(frame.frame_ref(), 1);
    assert_eq!(frame.frame_width(), 13260);
    assert_eq!(frame.frame_height(), 1327);

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"objectEmbeddingFrameCount\":1"));
    assert!(info.contains("\"className\":\"JSFart.Art.2\""));
    assert!(info.contains("\"frameRef\":1"));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert_json_brackets_balanced(&layer_tree);
    assert!(layer_tree.contains("\"type\":\"embeddingFrameDiagnostic\""));
    assert!(layer_tree.contains("\"source\":\"embedItemsEmbeddingInfo+frame\""));
    assert!(layer_tree.contains("\"embeddingIndex\":24"));
    assert!(layer_tree.contains("\"className\":\"JSFart.Art.2\""));
    assert!(layer_tree.contains("\"frameRef\":1"));
    assert!(layer_tree.contains("\"placementProven\":false"));
    assert!(layer_tree.contains("\"renderable\":false"));

    let svg = core.render_page_svg(0).unwrap();
    assert!(!svg.contains("class=\"rjtd-embedding-frame-diagnostic\""));
    assert!(!svg.contains("data-embedding-index=\"24\""));
}

#[test]
fn parser_preserves_embedded_press_snapshot_metadata_as_object_evidence() {
    let snapshot = embedded_press_snapshot_fixture(2590, 460, 3656, 3560);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/EmbedItems/Embedding 4/\x03EmbeddedPress", &snapshot),
    ]);

    let document = parse_document(&bytes).unwrap();
    let candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/\x03EmbeddedPress")
        .expect("EmbeddedPress stream should be preserved as object evidence");
    assert!(
        candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::EmbeddedPressSnapshot)
    );
    let snapshot = candidate
        .embedded_press_snapshot_candidate()
        .expect("JSSnapShot32 metadata should be decoded into the model");
    assert_eq!(snapshot.magic(), "JSSnapShot32");
    assert_eq!(snapshot.format_marker(), "GCI");
    assert_eq!(snapshot.body_length_candidate(), 3656);
    assert_eq!(snapshot.object_count_candidate(), 17);
    assert_eq!(snapshot.object_table_offset_candidate(), 74);
    assert_eq!(snapshot.payload_length_candidate(), 3560);
    assert_eq!(snapshot.width(), 2590);
    assert_eq!(snapshot.height(), 460);

    let info = DocumentCore::from_document(document).get_document_info();
    assert!(info.contains("\"embeddedPressSnapshot\":{\"format\":\"JSSnapShot32\""));
    assert!(info.contains("\"width\":2590"));
    assert!(info.contains("\"height\":460"));
    assert!(info.contains("\"renderable\":false"));
}

#[test]
fn local_tmogi3_2_projects_layout_box_text_when_reference_pdf_is_available() {
    let sample_dir = local_samples_dir();
    let sample_path = sample_dir.join("ichitaro-20030120133129-0007-sp-dat-tmogi3_2.jtd");
    let reference_pdf_path = sample_dir.join("ichitaro-20030120133129-0007-sp-dat-tmogi3_2.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let document = parse_document(&bytes).unwrap();

    assert!(
        document
            .raw_streams()
            .iter()
            .any(|stream| stream.name() == LAYOUT_BOX_PATH)
    );
    assert!(
        document
            .raw_streams()
            .iter()
            .any(|stream| stream.name() == LAYOUT_BOX_TEXT_PATH)
    );
    assert!(
        document
            .raw_streams()
            .iter()
            .any(|stream| stream.name() == LAYOUT_BOX_TEXT_POSITION_TABLES_PATH)
    );

    let mut core = DocumentCore::from_document(document);
    core.set_file_name(sample_path.to_string_lossy());
    assert_eq!(core.writing_mode(), WritingMode::Horizontal);
    assert!((core.page_width_px() - 793.7).abs() < 0.2);
    assert!((core.page_height_px() - 1122.5).abs() < 0.2);
    let document_info = core.get_document_info();
    assert!(document_info.contains(
        "\"writingModeDecision\":{\"selected\":\"horizontal\",\"source\":\"default-horizontal\""
    ));
    assert!(
        document_info.contains("\"writingModeCandidateFromDocumentViewStyles\":\"vertical-rl\"")
    );
    assert!(
        document_info.contains(
            "\"writingModeCandidateFromDocumentViewStylesFirstRecordCodeHex\":\"0x1001\""
        )
    );
    assert!(document_info.contains("\"documentViewStylesDisagreesWithSelected\":true"));
    let layer_tree = core.get_page_layer_tree(0).unwrap();
    let svg = core.render_page_svg(0).unwrap();

    assert_json_brackets_balanced(&layer_tree);
    assert!(layer_tree.contains("\"sourceStream\":\"/LayoutBoxText\""));
    assert!(layer_tree.contains("\"projectionKind\":\"layoutBoxTextProjection\""));
    assert!(layer_tree.contains("\"pageAssignmentDecoded\":false"));
    assert!(layer_tree.contains("\"positionTablePresent\":true"));
    assert!(layer_tree.contains("\"type\":\"pageFrameShape\""));
    assert!(layer_tree.contains("\"role\":\"titleRoundedFrame\""));
    assert!(layer_tree.contains("\"role\":\"horizontalPatternBar\""));
    assert!(layer_tree.contains("\"rowIndex\":1"));
    assert!(layer_tree.contains("\"placementBasis\":\"frameRecordBottomOriginFields\""));
    assert!(layer_tree.contains("\"type\":\"pageMarkSeparator\""));
    assert!(layer_tree.contains("\"projectionKind\":\"pageMarkSectionSeparatorProjection\""));
    assert!(layer_tree.contains("\"sourceRecordOffset\":228"));
    assert!(layer_tree.contains("\"sourceYCentipoints\":33618"));
    assert!(layer_tree.contains("\"sourceAdvanceCentipoints\":534"));
    assert!(
        layer_tree
            .contains("\"placementBasis\":\"pageMarkCentipointInsideLayoutBoxCaptionBodyGap\"")
    );
    assert!(layer_tree.contains("\"text\":\"タイピング科目\""));
    assert!(layer_tree.contains("\"placementBasis\":\"pageFrameTitleCenter\""));
    assert!(layer_tree.contains("世の中には忘れられない顔"));
    assert!(layer_tree.contains("\"role\":\"body\""));
    assert!(layer_tree.contains("\"text\":\"（制限時間10分）\""));
    assert!(svg.contains("rjtd-layout-box-text-projection"));
    assert!(svg.contains("rjtd-page-frame-projection"));
    assert!(svg.contains("rjtd-title-rounded-frame"));
    assert!(svg.contains("rjtd-horizontal-pattern-bar"));
    assert!(svg.contains("rjtd-page-mark-separator"));
    assert!(svg.contains("data-source-y-centipoints=\"33618\""));
    assert!(svg.contains("data-source-advance-centipoints=\"534\""));
    assert!(svg.contains("frameRecordBottomOriginFields"));
    assert!(svg.contains("タイピング科目"));
    assert!(svg.contains("世の中には忘れられない顔"));
}

#[test]
fn local_fax02_preserves_visual_list_when_reference_pdf_is_available() {
    let sample_dir = local_samples_dir();
    let sample_path = sample_dir.join("fax02.jtt");
    let reference_pdf_path = sample_dir.join("fax02.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let document = parse_document(&bytes).unwrap();
    let visual_list_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/VisualList")
        .expect("/VisualList must be preserved as model evidence");

    assert_eq!(visual_list_candidate.size(), 2296);
    assert!(
        visual_list_candidate
            .reasons()
            .contains(&ObjectStreamCandidateReason::VisualListPath)
    );
    assert_eq!(
        &visual_list_candidate.payload_prefix()[..4],
        b"\x00\x00\x08\xf8"
    );
    assert_eq!(&visual_list_candidate.payload_prefix()[4..8], b"BMDV");

    let visual_list = visual_list_candidate
        .visual_list_candidate()
        .expect("fax02 /VisualList must expose BMDV raster metadata");
    assert_eq!(visual_list.declared_size(), 2296);
    assert_eq!(visual_list.width(), 120);
    assert_eq!(visual_list.height(), 169);
    assert_eq!(visual_list.row_stride(), 120);
    assert_eq!(visual_list.bit_depth(), 8);
    assert_eq!(visual_list.rle_data_offset(), 0x50);
    assert_eq!(visual_list.rle_data_len(), 2216);
    assert_eq!(visual_list.pixels().len(), 120 * 169);

    let mut core = DocumentCore::from_document(document);
    core.set_file_name(sample_path.to_string_lossy());
    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"visualListRasterDiagnostic\""));
    assert!(layer_tree.contains("\"sourcePath\":\"/VisualList\""));
    assert!(layer_tree.contains("\"naturalWidth\":120"));
    assert!(layer_tree.contains("\"naturalHeight\":169"));
    assert!(layer_tree.contains("\"titleBand\":{"));
    assert!(layer_tree.contains("\"projectionKind\":\"visualListFillBandProjection\""));
    assert!(layer_tree.contains("\"placementProven\":true"));
    assert!(layer_tree.contains("\"type\":\"formTextProjection\""));
    assert!(layer_tree.contains("\"projectionKind\":\"visualListFormProjection\""));
    assert!(layer_tree.contains("\"role\":\"title\""));
    assert!(layer_tree.contains("\"role\":\"left-fax-label\""));
    assert!(layer_tree.contains("\"role\":\"right-tel-label\""));
    assert!(layer_tree.contains("\"role\":\"right-fax-label\""));
    assert!(layer_tree.contains("\"text\":\"FAX送付のご案内\""));
    assert!(layer_tree.contains("\"text\":\"TEL：\""));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-visual-list-raster-diagnostic\""));
    assert!(svg.contains("data-source-path=\"/VisualList\""));
    assert!(svg.contains("data-projection=\"rle8-raster\""));
    assert!(svg.contains("data-fallback-projection=\"horizontal-runs\""));
    assert!(svg.contains("class=\"rjtd-visual-list-rle8-raster\""));
    assert!(svg.contains("data-projection=\"visualListRle8RasterImage\""));
    assert!(svg.contains("data-suppressed-dark-foreground=\"true\""));
    assert!(svg.contains("data:image/png;base64,"));
    assert!(!svg.contains("class=\"rjtd-visual-list-horizontal-run\""));
    assert!(!svg.contains("class=\"rjtd-visual-list-fill-band\""));
    assert!(!svg.contains("data-projection=\"visualListTitleBandHatch\""));
    assert!(svg.contains("class=\"rjtd-observed-form-text-projection\""));
    assert!(svg.contains("data-projection=\"visualListFormProjection\""));
    assert!(svg.contains("data-role=\"title\""));
    assert!(svg.contains("data-role=\"right-tel-label\""));
    assert!(svg.contains(">FAX送付のご案内</text>"));
    assert!(svg.contains(">TEL：</text>"));
}

#[test]
fn local_success_data_test_preserves_embedding_frame_candidates_when_reference_pdf_is_available() {
    let sample_dir = local_samples_dir();
    let sample_path = sample_dir.join("ichitaro-20030228030923-success-002-success_data-test.jtd");
    let reference_pdf_path =
        sample_dir.join("ichitaro-20030228030923-success-002-success_data-test.pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let document = parse_document(&bytes).unwrap();

    let abc_table = document
        .table_candidates()
        .iter()
        .find(|candidate| success_data_test_abc_table_candidate(candidate))
        .expect("a/b/c Pythagorean table should be preserved as one candidate");
    assert_eq!(abc_table.intervals().len(), 3);
    assert_eq!(abc_table.source_start(), 519);
    assert_eq!(abc_table.source_end(), 924);
    assert_eq!(abc_table.cell_count_candidate(), 15);
    assert_eq!(abc_table.non_empty_cell_count_candidate(), 11);
    assert_eq!(abc_table.empty_cell_count_candidate(), 4);
    assert_eq!(abc_table.intervals()[0].text_preview(), "ａ\t１\t１\t７\t");
    assert_eq!(
        abc_table.intervals()[1].text_preview(),
        "ｂ\t１\t\t２４\t０.８"
    );
    assert_eq!(abc_table.intervals()[2].text_preview(), "ｃ\t\t２\t\t１");
    assert_eq!(
        abc_table.intervals()[0]
            .column_segments()
            .iter()
            .map(|segment| segment.text())
            .collect::<Vec<_>>(),
        ["ａ", "１", "１", "７", ""]
    );
    assert_eq!(
        abc_table.intervals()[1]
            .column_segments()
            .iter()
            .map(|segment| segment.text())
            .collect::<Vec<_>>(),
        ["ｂ", "１", "", "２４", "０.８"]
    );
    assert_eq!(
        abc_table.intervals()[2]
            .column_segments()
            .iter()
            .map(|segment| segment.text())
            .collect::<Vec<_>>(),
        ["ｃ", "", "２", "", "１"]
    );
    let abc_grid = abc_table
        .column_segment_grid_candidate()
        .expect("a/b/c table should retain a compatible column grid candidate");
    assert_eq!(abc_grid.row_count(), 3);
    assert_eq!(abc_grid.column_count(), 5);
    assert_eq!(
        table_grid_decoded_source_placement_required_cell_count(abc_table),
        15
    );
    assert_eq!(
        table_grid_decoded_source_placement_match_count(&document, abc_table),
        15
    );
    assert!(table_grid_decoded_source_placement_evidence_present(
        &document, abc_table
    ));
    let abc_line_header_rows = table_candidate_document_text_line_header_rows(&document, abc_table);
    assert_eq!(abc_line_header_rows.len(), 3);
    assert!(
        abc_line_header_rows
            .iter()
            .all(|row| row.raw_header_count() == 7 && row.matched_cell_count == 5)
    );
    assert_eq!(abc_line_header_rows[0].headers[0].offset_units, 0);
    assert_eq!(abc_line_header_rows[0].headers[0].extent_units, 10);
    assert_eq!(abc_line_header_rows[0].headers[1].offset_units, 14);
    assert_eq!(abc_line_header_rows[0].headers[6].extent_units, 174);

    assert_eq!(document.object_embedding_frames().len(), 6);
    let first_art = document
        .object_embedding_frames()
        .iter()
        .find(|frame| frame.embedding_index() == 24)
        .expect("embedding 24 should be preserved from /EmbedItems/EmbeddingInfo");
    assert_eq!(first_art.source_path(), EMBEDDING_INFO_PATH);
    assert_eq!(first_art.class_name(), "JSFart.Art.2");
    assert_eq!(first_art.primary_width(), 13260);
    assert_eq!(first_art.primary_height(), 1327);
    assert_eq!(first_art.frame_ref(), 1);
    assert_eq!(first_art.frame_width(), 13260);
    assert_eq!(first_art.frame_height(), 1327);
    assert_eq!(
        success_data_test_title_art_frame_refs(&document),
        vec![1, 16]
    );
    let title_pages = embedding_frame_diagnostics(&document)
        .into_iter()
        .filter_map(|diagnostic| {
            success_data_test_title_art_page_number(&document, diagnostic)
                .map(|page_number| (diagnostic.frame.frame_ref(), page_number))
        })
        .collect::<Vec<_>>();
    assert_eq!(title_pages, vec![(1, 1), (16, 2)]);

    let jseq = document
        .object_embedding_frames()
        .iter()
        .find(|frame| frame.embedding_index() == 4)
        .expect("JSEQ formula/document embedding should be preserved");
    assert_eq!(jseq.class_name(), "JSEQ.Document.3");
    assert_eq!(jseq.frame_ref(), 2);
    assert_eq!(jseq.frame_width(), 2590);
    assert_eq!(jseq.frame_height(), 460);

    let snapshot_candidates = document
        .object_stream_candidates()
        .iter()
        .filter(|candidate| candidate.embedded_press_snapshot_candidate().is_some())
        .collect::<Vec<_>>();
    assert_eq!(snapshot_candidates.len(), 6);
    let mut state_82_family_values = BTreeSet::new();
    let mut state_82_color_values = BTreeSet::new();
    let mut state_46_values = BTreeSet::new();
    let mut state_60_selector_values = BTreeSet::new();
    let mut state_65_selector_values = BTreeSet::new();
    for snapshot in snapshot_candidates
        .iter()
        .filter_map(|candidate| candidate.embedded_press_snapshot_candidate())
    {
        for path in snapshot.vector_paths() {
            for record in path.state_records() {
                let words = record.payload_le32_words();
                match record.record_type() {
                    0x82 => {
                        assert_eq!(words.get(3), Some(&0x00ff_ffff));
                        state_82_color_values.insert(words[3]);
                        state_82_family_values.insert(words[5]);
                    }
                    0x46 => {
                        assert_eq!(words.len(), 1);
                        assert!(matches!(words[0], 0 | 1));
                        state_46_values.insert(words[0]);
                    }
                    0x60 => {
                        assert_eq!(words.len(), 2);
                        assert!(matches!(words[0], 0x03 | 0x10 | 0x11));
                        assert_eq!(words[1], 0);
                        state_60_selector_values.insert(words[0]);
                    }
                    0x65 => {
                        assert_eq!(words.len(), 1);
                        assert!(matches!(words[0], 0x10 | 0x11));
                        state_65_selector_values.insert(words[0]);
                    }
                    _ => {}
                }
            }
        }
    }
    assert_eq!(
        state_82_color_values.into_iter().collect::<Vec<_>>(),
        vec![0x00ff_ffff]
    );
    assert_eq!(
        state_82_family_values.into_iter().collect::<Vec<_>>(),
        vec![0x10, 0x2f]
    );
    assert_eq!(state_46_values.into_iter().collect::<Vec<_>>(), vec![0, 1]);
    assert_eq!(
        state_60_selector_values.into_iter().collect::<Vec<_>>(),
        vec![0x03, 0x10, 0x11]
    );
    assert_eq!(
        state_65_selector_values.into_iter().collect::<Vec<_>>(),
        vec![0x10, 0x11]
    );
    let emb24_snapshot = snapshot_candidates
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 24/\x03EmbeddedPress")
        .and_then(|candidate| candidate.embedded_press_snapshot_candidate())
        .expect("Embedding 24 snapshot should expose JSSnapShot32 metadata");
    assert_eq!(emb24_snapshot.width(), 13260);
    assert_eq!(emb24_snapshot.height(), 1327);
    assert_eq!(emb24_snapshot.body_length_candidate(), 113332);
    assert_eq!(emb24_snapshot.vector_segments().len(), 9895);
    assert_eq!(emb24_snapshot.vector_paths().len(), 552);
    assert_eq!(
        embedded_press_snapshot_vector_path_kind_count(
            emb24_snapshot,
            ObjectEmbeddedPressVectorPathKind::Outline
        ),
        22
    );
    assert_eq!(
        embedded_press_snapshot_vector_path_kind_count(
            emb24_snapshot,
            ObjectEmbeddedPressVectorPathKind::Texture
        ),
        530
    );
    assert_eq!(
        embedded_press_snapshot_vector_path_state_record_count(emb24_snapshot),
        338
    );
    let state_record_counts = embedded_press_snapshot_state_record_type_counts(emb24_snapshot);
    assert_eq!(state_record_counts.get(&0x40), Some(&33));
    assert_eq!(state_record_counts.get(&0x46), Some(&76));
    assert_eq!(state_record_counts.get(&0x48), Some(&33));
    assert_eq!(state_record_counts.get(&0x60), Some(&66));
    assert_eq!(state_record_counts.get(&0x65), Some(&64));
    assert_eq!(state_record_counts.get(&0x70), Some(&33));
    assert_eq!(state_record_counts.get(&0x82), Some(&33));
    let first_shadow_outline = &emb24_snapshot.vector_paths()[0];
    assert_eq!(
        first_shadow_outline.kind(),
        ObjectEmbeddedPressVectorPathKind::Outline
    );
    assert_eq!(
        first_shadow_outline
            .state_records()
            .iter()
            .map(ObjectEmbeddedPressStateRecordCandidate::record_type)
            .collect::<Vec<_>>(),
        vec![0x40, 0x48, 0x46, 0x82, 0x70, 0x46, 0x60, 0x60]
    );
    assert_eq!(
        first_shadow_outline.state_records()[3].payload_le32_words()[5],
        0x2f
    );
    assert_eq!(
        embedded_press_state_record_payload_first_words(first_shadow_outline, 0x46),
        vec![0, 1]
    );
    let first_texture_path = &emb24_snapshot.vector_paths()[11];
    assert_eq!(
        first_texture_path.kind(),
        ObjectEmbeddedPressVectorPathKind::Texture
    );
    assert_eq!(
        first_texture_path.state_records()[6].payload_le32_words()[5],
        0x2f
    );
    assert_eq!(
        embedded_press_state_record_payload_first_words(first_texture_path, 0x46),
        vec![0, 0]
    );
    let later_texture_path = &emb24_snapshot.vector_paths()[27];
    assert_eq!(
        later_texture_path.kind(),
        ObjectEmbeddedPressVectorPathKind::Texture
    );
    assert_eq!(
        later_texture_path.state_records()[5].payload_le32_words()[5],
        0x2f
    );
    assert_eq!(
        embedded_press_state_record_payload_first_words(later_texture_path, 0x46),
        vec![0]
    );
    let first_main_outline = &emb24_snapshot.vector_paths()[541];
    assert_eq!(
        first_main_outline.kind(),
        ObjectEmbeddedPressVectorPathKind::Outline
    );
    assert_eq!(
        first_main_outline.state_records()[5].payload_le32_words()[5],
        0x10
    );
    assert_eq!(
        embedded_press_state_record_payload_first_words(first_main_outline, 0x46),
        vec![0, 1]
    );
    let texture_headers = emb24_snapshot
        .vector_paths()
        .iter()
        .filter_map(ObjectEmbeddedPressVectorPathCandidate::texture_bezier_header)
        .collect::<Vec<_>>();
    assert_eq!(texture_headers.len(), 530);
    assert!(texture_headers.iter().all(|header| {
        header.point_count() == 13 && header.byte_count() == 104 && header.flags() == 1
    }));
    assert!(success_data_test_title_art_rendered_segment_count(emb24_snapshot) > 0);
    assert_eq!(
        success_data_test_title_art_rendered_path_count(emb24_snapshot),
        22
    );
    assert_eq!(
        success_data_test_title_art_shadow_path_count(emb24_snapshot),
        11
    );
    let title_partition = embedded_press_title_art_shadow_path_partition(emb24_snapshot)
        .expect("title art outlines should be partitioned from EmbeddedPress paint state");
    assert_eq!(
        title_partition.strategy,
        "embedded-press-source-order-outline-texture-outline"
    );
    assert_eq!(title_partition.shadow_paths.len(), 11);
    assert_eq!(title_partition.main_paths.len(), 11);
    assert_eq!(title_partition.offset, (100, 100));
    let interstitial_texture_paths =
        success_data_test_title_art_interstitial_texture_paths(emb24_snapshot, &title_partition)
            .expect("title art texture block should sit between shadow and main outlines");
    assert_eq!(interstitial_texture_paths.len(), 530);
    let state_tagged_texture_paths =
        success_data_test_title_art_state_tagged_texture_paths(emb24_snapshot);
    assert_eq!(state_tagged_texture_paths.len(), 11);
    assert_eq!(
        embedded_press_vector_path_state_word5_values(&state_tagged_texture_paths),
        vec![EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5]
    );
    let shadow_texture_paths = success_data_test_title_art_shadow_texture_paths(emb24_snapshot);
    assert_eq!(shadow_texture_paths.len(), 11);
    assert_eq!(
        embedded_press_vector_path_state_word5_values(&shadow_texture_paths),
        vec![EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5]
    );
    let effective_shadow_texture_paths =
        success_data_test_title_art_effective_texture_paths_for_word5(
            emb24_snapshot,
            EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5,
        );
    assert_eq!(effective_shadow_texture_paths.len(), 530);
    let effective_front_texture_paths =
        success_data_test_title_art_effective_texture_paths_for_word5(
            emb24_snapshot,
            EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5,
        );
    let front_texture_paths = success_data_test_title_art_front_texture_paths(emb24_snapshot);
    assert_eq!(
        front_texture_paths.len(),
        effective_front_texture_paths.len(),
        "front texture selection must use inherited current paint-state ownership"
    );
    assert!(
        front_texture_paths.is_empty(),
        "shadow-state texture paths must not be replayed over the main title face"
    );
    assert_eq!(
        success_data_test_title_art_effective_texture_word5_values(emb24_snapshot),
        vec![EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5]
    );
    let emb4_snapshot = snapshot_candidates
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/\x03EmbeddedPress")
        .and_then(|candidate| candidate.embedded_press_snapshot_candidate())
        .expect("Embedding 4 snapshot should expose JSSnapShot32 metadata");
    assert_eq!(emb4_snapshot.width(), 2590);
    assert_eq!(emb4_snapshot.height(), 460);
    assert_eq!(emb4_snapshot.vector_segments().len(), 51);

    let jseq_candidates = document
        .object_stream_candidates()
        .iter()
        .filter(|candidate| candidate.jseq3_formula_candidate().is_some())
        .collect::<Vec<_>>();
    assert_eq!(jseq_candidates.len(), 4);
    let emb4_formula = jseq_candidates
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 4/JSEQ3Contents")
        .and_then(|candidate| candidate.jseq3_formula_candidate())
        .expect("Embedding 4 JSEQ3Contents should expose MATH.VAF metadata");
    assert_eq!(emb4_formula.magic(), "MATH.VAF");
    assert_eq!(emb4_formula.magic_offset(), 0);
    assert_eq!(emb4_formula.so_trailer_offset(), Some(1658));
    assert_eq!(emb4_formula.so_trailer_length(), Some(62));
    assert_eq!(emb4_formula.so_trailer_fields()[0], 0x0000_4f53);
    assert_eq!(emb4_formula.so_trailer_fields()[1], 0x200e_0a20);
    assert!(
        emb4_formula
            .text_markers()
            .iter()
            .any(|marker| marker.text() == "Times New Roman" && marker.offset() == 892)
    );
    assert_eq!(emb4_formula.text_tokens().len(), 4);
    assert_eq!(emb4_formula.text_tokens()[0].text(), "１");
    assert_eq!(emb4_formula.text_tokens()[1].text(), "２");
    assert_eq!(emb4_formula.text_tokens()[2].text(), "÷");
    assert_eq!(emb4_formula.text_tokens()[3].text(), "３");
    assert_eq!(emb4_formula.text_runs().len(), 3);
    assert_eq!(emb4_formula.text_runs()[0].text(), "１２");
    assert_eq!(emb4_formula.text_runs()[0].start_offset(), 556);
    assert_eq!(emb4_formula.text_runs()[0].end_offset(), 586);
    assert_eq!(emb4_formula.text_runs()[0].token_offsets(), &[556, 584]);
    assert_eq!(emb4_formula.text_runs()[0].context_start_offset(), 492);
    assert_eq!(emb4_formula.text_runs()[0].context_fields_le32()[14], 296);
    assert_eq!(emb4_formula.text_runs()[0].context_fields_le32()[15], 148);
    assert_eq!(emb4_formula.text_runs()[1].text(), "÷");
    assert_eq!(emb4_formula.text_runs()[2].text(), "３");

    let title_jsfart_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/EmbedItems/Embedding 24/JSFart2Contents")
        .expect("Embedding 24 JSFart2Contents should be preserved as object evidence");
    let title_jsfart_profile = title_jsfart_candidate
        .jsfart_stream_profile_candidate()
        .expect("Embedding 24 JSFart2Contents should expose a source stream profile");
    assert_eq!(title_jsfart_profile.magic_family(), "mstudio-ocx-utf16le");
    assert_eq!(title_jsfart_profile.magic_family_hex(), "4d00");
    assert_eq!(
        title_jsfart_profile.magic_ascii_or_utf16_preview(),
        "MSTUDIO."
    );
    assert!(title_jsfart_profile.structured_art_candidate_present());
    assert_eq!(
        title_jsfart_profile.render_promotion_blocked_reason(),
        "structured-jsfart-art-still-paint-authority-unproven"
    );
    let title_jsfart_art = title_jsfart_candidate
        .jsfart_art_candidate()
        .expect("Embedding 24 JSFart2Contents should expose title art metadata");
    assert_eq!(title_jsfart_art.magic(), "MSTUDIO.OCX");
    assert_eq!(title_jsfart_art.width(), 13260);
    assert_eq!(title_jsfart_art.height(), 1327);
    let title_frame = title_jsfart_art
        .frame_candidate()
        .expect("JSFart2Contents should expose title frame geometry");
    assert_eq!(title_frame.left(), 0);
    assert_eq!(title_frame.top(), 0);
    assert_eq!(title_frame.right(), 13260);
    assert_eq!(title_frame.bottom(), 1327);
    assert_eq!(title_frame.content_left(), 114);
    assert_eq!(title_frame.content_top(), 105);
    assert_eq!(title_frame.content_right(), 13145);
    assert_eq!(title_frame.content_bottom(), 1159);
    assert_eq!(title_frame.corner_radius_x(), 114);
    assert_eq!(title_frame.corner_radius_y(), 105);
    assert_eq!(title_frame.stroke_width_candidate(), Some(100));
    let title_paint = title_jsfart_art
        .paint_candidate()
        .expect("JSFart2Contents should expose observed paint/style header fields");
    assert_eq!(title_paint.style_word_1(), 0x0214_1030);
    assert_eq!(title_paint.style_word_2(), 0x0214_1018);
    assert_eq!(title_paint.paint_color_candidate(), 0x00ff_ffff);
    assert_eq!(title_paint.paint_flag_candidate(), 1);
    assert_eq!(title_paint.effect_word_candidate(), 0x0000_000a);

    let fdm_text = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMText")
        .expect("success_data test should expose FDMText labels");
    assert_eq!(fdm_text.fdm_text_candidates().len(), 15);
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "９㎝"
                && candidate.marker_offset() == 0
                && candidate.text_offset() == 0x00c4
                && candidate.raw_text() == [0x82, 0x58, 0x87, 0x70]
                && candidate.bbox()
                    == Some(ObjectFdmIndexBbox::new(-12004, -11540, -11692, -11336)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "３㎝"
                && candidate.marker_offset() == 204
                && candidate.text_offset() == 0x0190
                && candidate.raw_text() == [0x82, 0x52, 0x87, 0x70]
                && candidate.bbox()
                    == Some(ObjectFdmIndexBbox::new(-11200, -10738, -10888, -10534)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "110°"
                && candidate.marker_offset() == 2054
                && candidate.bbox() == Some(ObjectFdmIndexBbox::new(-15254, -9453, -14961, -9284)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "30°"
                && candidate.bbox() == Some(ObjectFdmIndexBbox::new(-13488, -9410, -13251, -9241)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "160°"
                && candidate.bbox() == Some(ObjectFdmIndexBbox::new(-11278, -9823, -10985, -9654)))
    );
    assert!(
        fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "Ａ"
                && candidate.marker_offset() == 2474
                && candidate.bbox()
                    == Some(ObjectFdmIndexBbox::new(-13494, -12097, -13380, -11928)))
    );
    let expanded_fdm_text = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/ExpandData/main_data/Data/FDMText")
        .expect("success_data answer sheet should expose expanded FDMText labels");
    assert_eq!(expanded_fdm_text.fdm_text_candidates().len(), 15);
    assert_eq!(
        expanded_fdm_text.fdm_text_index_entry_candidates().len(),
        15
    );
    assert!(
        expanded_fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "９㎝"
                && candidate.marker_offset() == 0
                && candidate.text_offset() == 0x00c6
                && candidate.raw_text() == [0xff, 0x19, 0x33, 0x9d]
                && candidate.bbox()
                    == Some(ObjectFdmIndexBbox::new(-12004, -11540, -11692, -11336)))
    );
    assert!(
        expanded_fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "110°"
                && candidate.marker_offset() == 0x1566
                && candidate.text_offset() == 0x1638
                && candidate.raw_text() == [0x00, 0x31, 0x00, 0x31, 0x00, 0x30, 0x00, 0xb0])
    );
    assert!(
        expanded_fdm_text
            .fdm_text_candidates()
            .iter()
            .any(|candidate| candidate.text() == "Ａ"
                && candidate.marker_offset() == 0x19b8
                && candidate.raw_text() == [0xff, 0x21])
    );
    let first_text_index = &expanded_fdm_text.fdm_text_index_entry_candidates()[0];
    assert_eq!(first_text_index.index_offset(), 26);
    assert_eq!(first_text_index.text_record_offset(), 0);
    assert_eq!(first_text_index.kind(), 0x1600);
    assert_eq!(
        first_text_index.bbox(),
        ObjectFdmIndexBbox::new(-12104, -11592, -11640, -11236)
    );
    assert_eq!(
        first_text_index.text_record_bbox(),
        Some(ObjectFdmIndexBbox::new(-12004, -11540, -11692, -11336))
    );
    let last_text_index = expanded_fdm_text
        .fdm_text_index_entry_candidates()
        .last()
        .expect("expanded FDMText index must preserve the last text record link");
    assert_eq!(last_text_index.index_offset(), 334);
    assert_eq!(last_text_index.text_record_offset(), 0x1df8);
    assert_eq!(
        last_text_index.text_record_bbox(),
        Some(ObjectFdmIndexBbox::new(-15918, -10511, -15804, -10342))
    );

    let fdm_vector = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
        .expect("success_data test should expose FDMVector commands");
    assert_eq!(fdm_vector.fdm_raw_vector_segments().len(), 5);
    let first_fdm_segment = &fdm_vector.fdm_raw_vector_segments()[0];
    assert_eq!(first_fdm_segment.relative_offset(), 0);
    assert_eq!(first_fdm_segment.declared_len(), 176);
    assert_eq!(first_fdm_segment.command_count(), 2);
    assert_eq!(first_fdm_segment.command_offsets(), &[56, 90]);
    assert_eq!(
        first_fdm_segment.bbox(),
        Some(ObjectFdmIndexBbox::new(-11840, -12064, -10720, -10510))
    );
    assert_eq!(first_fdm_segment.source_width(), 1120);
    assert_eq!(first_fdm_segment.source_height(), 1554);
    assert!(
        fdm_vector
            .fdm_raw_vector_segments()
            .iter()
            .any(|segment| segment.relative_offset() == 1864
                && segment.declared_len() == 236
                && segment.command_count() == 4
                && segment.command_offsets() == [60, 94, 128, 160])
    );
    assert_eq!(fdm_vector.fdm_raw_vector_commands().len(), 37);
    assert!(
        fdm_vector
            .fdm_raw_vector_commands()
            .iter()
            .any(|command| command.marker() == b"\x01\x00\x04\x60")
    );
    assert!(
        fdm_vector
            .fdm_raw_vector_commands()
            .iter()
            .filter_map(fdm_vector_command_source_bbox)
            .map(normalize_fdm_bbox)
            .any(|bbox| bbox == (-15784, -10213, -14584, -9013))
    );
    let q3_dashed_curve = fdm_vector
        .fdm_raw_vector_commands()
        .iter()
        .find(|command| command.relative_offset() == 208)
        .expect("Q3 dashed guide curve command should be preserved from FDMVector");
    assert_eq!(q3_dashed_curve.marker(), b"\x01\x00\x09\x60");
    assert_eq!(q3_dashed_curve.style_word(), 0x0120);
    assert_eq!(q3_dashed_curve.path_points().len(), 2);
    assert_eq!(q3_dashed_curve.curve_segments().len(), 1);
    assert_eq!(q3_dashed_curve.source_vector_relative_offset(), Some(208));
    assert_eq!(q3_dashed_curve.source_segment(), None);
    let q5_segment_polyline = fdm_vector
        .fdm_raw_vector_commands()
        .iter()
        .find(|command| command.relative_offset() == 1992)
        .expect("Q5 segment-backed polyline command should preserve segment provenance");
    assert_eq!(
        q5_segment_polyline.source_vector_relative_offset(),
        Some(1992)
    );
    let q5_source_segment = q5_segment_polyline
        .source_segment()
        .expect("Q5 polyline should be linked to its FDMVector segment header");
    assert_eq!(q5_source_segment.relative_offset(), 1864);
    assert_eq!(q5_source_segment.local_offset(), 128);
    assert_eq!(q5_source_segment.declared_len(), 236);
    assert_eq!(q5_source_segment.command_count(), 4);
    assert_eq!(q5_source_segment.command_index(), 2);
    assert_eq!(q5_source_segment.command_offset(), 128);

    let mut renamed_core = DocumentCore::from_document(document.clone());
    renamed_core.set_file_name("renamed-success-data-test.jtd");
    assert_eq!(renamed_core.writing_mode(), WritingMode::Horizontal);
    assert!((renamed_core.page_width_px() - 687.9).abs() < 0.2);
    assert!((renamed_core.page_height_px() - 971.3).abs() < 0.2);
    assert_eq!(renamed_core.page_count(), 2);
    assert!(
        renamed_core
            .get_page_layer_tree(0)
            .unwrap()
            .contains("\"projectionKind\":\"successDataTestFdmReferenceProjection\"")
    );

    let mut core = DocumentCore::from_document(document.clone());
    core.set_file_name(sample_path.to_string_lossy());
    assert_eq!(core.page_count(), 2);
    let info = core.get_document_info();
    assert!(info.contains("\"pageMarkCount\":1"));
    assert!(info.contains("\"rawLength\":84,\"rawHex\":\"00000000000100000000000000000027"));
    assert!(info.contains("\"u16Fields\":[0,0,1,0,0,0,0,39,0,0,370,0"));
    assert!(info.contains("\"u16FieldsHex\":[\"0x0000\",\"0x0000\",\"0x0001\",\"0x0000\""));
    assert!(info.contains("\"u16GeometryClass\":\"additive-boundary\""));
    assert!(info.contains(
        "\"u16GeometryHypotheses\":{\"source\":\"/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"profile\":\"additive-boundary\""
    ));
    assert!(info.contains(
        "\"word20Is0x00ff\":true,\"word13PlusWord14\":555,\"word13PlusWord14EqualsWord21\":true,\"word21MinusWord13\":185,\"word21MinusWord13EqualsWord14\":true,\"word19EqualsWord13\":true,\"selectedFieldsAllZero\":false,\"nonZeroAdditiveUnitCandidate\":true,\"layoutComparisons\":null"
    ));
    assert!(info.contains("\"u32Fields\":[0,65536,0,39,0,24248320,370,12124160"));
    assert!(
        info.contains(
            "\"u32FieldsHex\":[\"0x00000000\",\"0x00010000\",\"0x00000000\",\"0x00000027\""
        )
    );
    let sparse_answer_candidate = core
        .document()
        .table_candidates()
        .iter()
        .find(|candidate| {
            candidate.is_sparse_document_text_control_run_candidate()
                && candidate
                    .intervals()
                    .iter()
                    .any(|interval| interval.text_preview().contains("(1)表面積の比"))
        })
        .expect("page 2 answer sheet should be preserved as sparse table evidence");
    assert_eq!(
        sparse_answer_candidate.rule(),
        "sparse-document-text-001c-cells-with-000e-row-breaks"
    );
    assert_eq!(sparse_answer_candidate.interval_count(), 39);
    assert_eq!(sparse_answer_candidate.source_start(), 2902);
    assert_eq!(sparse_answer_candidate.source_end(), 5419);
    assert_eq!(sparse_answer_candidate.max_column_segment_count(), 11);
    assert_eq!(sparse_answer_candidate.non_empty_cell_count_candidate(), 30);
    assert_eq!(sparse_answer_candidate.empty_cell_count_candidate(), 136);
    let sparse_answer_topology = sparse_answer_candidate.sparse_topology_candidate().unwrap();
    assert_eq!(sparse_answer_topology.row_count(), 39);
    assert_eq!(sparse_answer_topology.max_column_count(), 11);
    assert_eq!(sparse_answer_topology.cell_count(), 166);
    assert_eq!(sparse_answer_topology.non_empty_cell_count(), 30);
    assert_eq!(sparse_answer_topology.empty_cell_count(), 136);
    assert_eq!(sparse_answer_topology.columns().len(), 11);
    assert_eq!(
        sparse_answer_topology.columns()[0].non_empty_cell_count(),
        0
    );
    assert_eq!(
        sparse_answer_topology.columns()[0].observed_cell_count(),
        38
    );
    assert!(
        sparse_answer_topology
            .rows()
            .iter()
            .any(|row| row.non_empty_cell_count() >= 3)
    );
    assert!(
        sparse_answer_candidate
            .intervals()
            .iter()
            .any(|interval| interval.text_preview().contains("ＡＢ ＝ ｃｍ"))
    );
    assert!(info.contains("\"kind\":\"sparseDocumentTextControlRunTableCandidate\""));
    assert!(info.contains("\"sparse\":true"));
    assert!(info.contains("\"sparseObservedTable\":{\"source\":\"sparseDocumentTextControlRows\""));
    assert!(info.contains("\"topologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\""));
    assert!(
        info.contains("\"sparseTopologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\"")
    );
    assert!(info.contains("\"columns\":["));
    assert!(info.contains("\"emptyCellCountCandidate\":136"));
    assert!(info.contains("\"objectEmbeddingFrameCount\":6"));
    assert!(info.contains("\"embeddingIndex\":24"));
    assert!(info.contains("\"className\":\"JSFart.Art.2\""));
    assert!(info.contains("\"className\":\"JSEQ.Document.3\""));
    assert!(info.contains("\"jsfartStreamProfile\":{\"format\":\"JSFart2Contents\""));
    assert!(info.contains("\"magicFamily\":\"mstudio-ocx-utf16le\""));
    assert!(info.contains("\"magicFamilyHex\":\"4d00\""));
    assert!(info.contains("\"structuredArtCandidatePresent\":true"));
    assert!(info.contains(
        "\"renderPromotionBlockedReason\":\"structured-jsfart-art-still-paint-authority-unproven\""
    ));
    assert!(info.contains("\"jsfartArt\":{\"format\":\"JSFart2Contents\""));
    assert!(info.contains("\"frameCandidate\":{\"left\":0,\"top\":0"));
    assert!(info.contains(
        "\"contentLeft\":114,\"contentTop\":105,\"contentRight\":13145,\"contentBottom\":1159"
    ));
    assert!(info.contains("\"paintCandidate\":{\"styleWord1\":34869296"));
    assert!(info.contains("\"styleWord1Hex\":\"0x02141030\""));
    assert!(info.contains("\"styleWord2Hex\":\"0x02141018\""));
    assert!(info.contains("\"paintColorCandidateHex\":\"0x00ffffff\""));
    assert!(info.contains("\"effectWordCandidateHex\":\"0x0000000a\""));
    assert!(info.contains("\"jseq3Formula\":{\"format\":\"JSEQ3Contents\""));
    assert!(info.contains("\"textRuns\":[{\"text\":\"１２\",\"startOffset\":556"));
    assert!(info.contains("\"soTrailerOffset\":1658"));
    assert!(info.contains("\"text\":\"Times New Roman\""));
    assert!(info.contains("\"textTokens\":[{\"text\":\"１\",\"offset\":556"));
    assert!(info.contains("\"fdmRawVectorSegmentCount\":5"));
    assert!(info.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":308,\"matchKind\":\"command-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.relativeOffset\",\"matchedCommandRelativeOffsets\":[308],\"decoded\":false}]"));
    assert!(info.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":690,\"matchKind\":\"source-segment-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\",\"sourceSegmentRelativeOffset\":690,\"sourceSegmentBackedCommandCount\":1,\"matchedCommandRelativeOffsets\":[874],\"decoded\":false}]"));
    assert!(info.contains("\"offsetFieldReferenceCandidates\":[{\"offsetField\":\"bbox.left\",\"offsetValue\":1864,\"matchKind\":\"source-segment-relative-offset-field\",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\",\"sourceSegmentRelativeOffset\":1864,\"sourceSegmentBackedCommandCount\":4,\"matchedCommandRelativeOffsets\":[1924,1958,1992,2024],\"decoded\":false}]"));
    assert!(info.contains("\"relativeOffset\":1864,\"declaredLength\":236,\"commandCount\":4"));
    assert!(info.contains("\"commandOffsets\":[60,94,128,160]"));
    assert!(info.contains("\"fdmRawVectorCommandCount\":37"));
    assert!(info.contains("\"fdmRawVectorCommands\":["));
    assert!(info.contains("\"sourceVectorRelativeOffset\":208,\"sourceSegment\":null"));
    assert!(info.contains(
        "\"sourceVectorRelativeOffset\":1992,\"sourceSegment\":{\"relativeOffset\":1864,\"localOffset\":128,\"declaredLength\":236,\"commandCount\":4,\"commandIndex\":2,\"commandOffset\":128}"
    ));
    assert!(
        info.contains(
            "\"successDataTestFdmReferenceProjections\":[{\"role\":\"q4-angle-diagrams\""
        )
    );
    assert!(
        info.contains("\"referenceTargetBboxPx\":{\"x\":93.300,\"y\":663.300,\"width\":491.400")
    );
    assert!(info.contains(
        "\"commandRelativeOffsets\":[308,342,374,406,438,470,504,538,570,602,634,874,1048,1126,1158,1190,1430,1604,1730,1780]"
    ));
    assert!(info.contains("\"renderPromotionBlockedReason\":\"mixed-raw-and-segment-cohorts\""));
    assert!(info.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":20,\"mainCircleAnchorCount\":3,\"lineCandidateCount\":11,\"radialLineCandidateCount\":0,\"chordCandidateCount\":0,\"arcCandidateCount\":6,\"connectorCandidateCount\":8,\"surfaceBoundaryCandidateCount\":2"));
    assert!(info.contains(
        "\"indexRowReferenceCandidateCount\":20,\"validVectorOffsetIndexRowReferenceCount\":0"
    ));
    assert_json_string_field_after(
        &info,
        "\"ownershipGate\":{",
        0,
        "renderOwnershipBlockedReason",
        "mixed-raw-and-segment-cohorts",
    );
    assert_json_string_array_field_after(
        &info,
        "\"ownershipGate\":{",
        0,
        "renderOwnershipBlockedReasons",
        &["mixed-raw-and-segment-cohorts"],
    );
    assert_json_number_field_after(&info, "\"ownershipGate\":{", 0, "commandCount", "20");
    assert_json_number_field_after(&info, "\"ownershipGate\":{", 0, "rawSpanCommandCount", "18");
    assert_json_number_field_after(
        &info,
        "\"ownershipGate\":{",
        0,
        "segmentBackedCommandCount",
        "2",
    );
    assert_json_bool_field_after(
        &info,
        "\"ownershipGate\":{",
        0,
        "oneToOneRowCommandReferenceCandidate",
        true,
    );
    assert_json_string_field_after(
        &info,
        "\"offsetFieldAuthorityGate\":{",
        0,
        "renderPromotionBlockedReason",
        "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
    );
    assert_json_number_field_after(
        &info,
        "\"offsetFieldAuthorityGate\":{",
        0,
        "commandRelativeOffsetFieldReferenceCount",
        "18",
    );
    assert_json_number_field_after(
        &info,
        "\"offsetFieldAuthorityGate\":{",
        0,
        "sourceSegmentRelativeOffsetFieldReferenceCount",
        "2",
    );
    assert_json_string_field_after(
        &info,
        "\"rowFanoutSegmentOwnerGate\":{",
        0,
        "renderPromotionBlockedReason",
        "fdm-index-row-fanout-segment-owner-offset-namespace-mixed",
    );
    assert_json_number_field_after(
        &info,
        "\"rowFanoutSegmentOwnerGate\":{",
        0,
        "maxRowFanout",
        "1",
    );
    assert_json_bool_field_after(
        &info,
        "\"rowFanoutSegmentOwnerGate\":{",
        0,
        "singleRowBacksMultipleCommandsCandidate",
        false,
    );
    assert_json_string_field_after(
        &info,
        "\"primitiveOwnershipAdmissionGate\":{",
        0,
        "renderPromotionBlockedReason",
        "mixed-raw-and-segment-cohorts",
    );
    assert_json_string_array_field_after(
        &info,
        "\"primitiveOwnershipAdmissionGate\":{",
        0,
        "renderPromotionBlockedReasons",
        &[
            "mixed-raw-and-segment-cohorts",
            "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
            "fdm-index-row-fanout-segment-owner-offset-namespace-mixed",
            "fdm-index-role-vector-offset-authority-valid-vector-offset-missing",
            "fdm-index-role-valid-vector-offset-missing",
            "role-paint-order-continuity-unproven",
        ],
    );
    assert_json_number_field_after(
        &info,
        "\"primitiveOwnershipAdmissionGate\":{",
        0,
        "rolePaintOrderBlockedGroupCount",
        "6",
    );
    assert_json_string_field_after(
        &info,
        "\"indexRowOrderPromotionGate\":{",
        0,
        "renderPromotionBlockedReason",
        "fdm-index-row-order-valid-vector-offset-missing",
    );
    assert_json_string_array_field_after(
        &info,
        "\"indexRowOrderPromotionGate\":{",
        0,
        "renderPromotionBlockedReasons",
        &[
            "fdm-index-row-order-valid-vector-offset-missing",
            "fdm-index-row-order-offset-namespace-mixed",
            "role-paint-order-continuity-unproven",
        ],
    );
    assert_json_number_field_after(
        &info,
        "\"indexRowOrderPromotionGate\":{",
        0,
        "uniqueRowIndexCount",
        "20",
    );
    assert!(info.contains("\"renderPaintOrderBasisCandidate\":\"fdm-index-row-command-pairs\",\"renderPaintOrderBasisDecoded\":false"));
    assert!(info.contains("\"roleCandidate\":\"main-circle-anchor\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":3,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":3,\"sourceSegmentRelativeOffsetFieldReferenceCount\":0,\"commandRelativeOffsets\":[308,470,504],\"rowIndexes\":[7,12,13],\"uniqueCommandRelativeOffsetCount\":3,\"uniqueRowIndexCount\":3,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":7,\"commandRelativeOffset\":308,\"matchKind\":\"command-relative-offset-field\"}"));
    assert!(info.contains("\"paintOrderContinuityProfile\":{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false,\"commandRelativeOffsetSpanMin\":308,\"commandRelativeOffsetSpanMax\":504,\"roleCommandCount\":3,\"commandCountInSpan\":7,\"interleavedNonRoleCommandCount\":4,\"hasInterleavedNonRoleCommands\":true,\"maxCommandOffsetGap\":162,\"commandOffsetContinuityScore\":0.429,\"spanContiguousCandidate\":false,\"paintOrderAuthorityPending\":false,\"continuityBlocked\":true,\"renderPromotionBlockedReason\":\"role-span-interleaved-non-role-commands\"}"));
    assert!(info.contains("\"roleCandidate\":\"radial-line-candidate\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":2,\"sourceSegmentRelativeOffsetFieldReferenceCount\":0,\"commandRelativeOffsets\":[342,406],\"rowIndexes\":[8,10],\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":2,\"oneToOneRowCommandReferenceCandidate\":true,\"singleRowBacksMultipleCommandsCandidate\":false,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":8,\"commandRelativeOffset\":342,\"matchKind\":\"command-relative-offset-field\"},{\"rowIndex\":10,\"commandRelativeOffset\":406,\"matchKind\":\"command-relative-offset-field\"}],\"roleVectorOffsetAuthorityGate\":"));
    assert!(info.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":7,\"mainCircleAnchorCount\":1,\"lineCandidateCount\":4,\"radialLineCandidateCount\":2,\"chordCandidateCount\":2,\"arcCandidateCount\":2,\"connectorCandidateCount\":2,\"surfaceBoundaryCandidateCount\":2"));
    assert!(info.contains("\"relativeOffset\":374,\"primitiveKind\":\"polyline\",\"markerHex\":\"01000160\",\"sourceSegmentBacked\":false,\"sourceSegmentRelativeOffset\":null,\"roleCandidates\":[\"line-candidate\",\"chord-candidate\",\"connector-candidate\"]"));
    assert!(info.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":9,\"indexOffset\":218,\"vectorOffset\":3663724543,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":374,\"matchKind\":\"command-relative-offset-field\",\"decoded\":false}]"));
    assert!(info.contains("\"relativeOffset\":1430,\"primitiveKind\":\"ellipse\",\"markerHex\":\"ff000460\",\"sourceSegmentBacked\":true,\"sourceSegmentRelativeOffset\":1246,\"roleCandidates\":[\"arc-candidate\",\"control-ellipse-marker\"]"));
    assert!(info.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":32,\"indexOffset\":724,\"vectorOffset\":3671785471,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":1246,\"matchKind\":\"source-segment-relative-offset-field\",\"decoded\":false}]"));
    assert!(info.contains(
        "\"subdiagrams\":[{\"index\":0,\"groupingSource\":\"nearest-main-circle-source-center\""
    ));
    assert!(info.contains("\"role\":\"q5-solid-diagram\""));
    assert!(info.contains(
        "\"referenceTargetBboxPx\":{\"x\":490.700,\"y\":795.000,\"width\":74.600,\"height\":110.000}"
    ));
    assert!(info.contains("\"commandRelativeOffsets\":[1830,1924,1958,1992,2024,2156,2190]"));
    assert!(info.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":7,\"mainCircleAnchorCount\":0,\"lineCandidateCount\":2,\"radialLineCandidateCount\":0,\"chordCandidateCount\":0,\"arcCandidateCount\":4,\"connectorCandidateCount\":3,\"surfaceBoundaryCandidateCount\":1"));
    assert!(info.contains(
        "\"indexRowReferenceCandidateCount\":7,\"validVectorOffsetIndexRowReferenceCount\":0"
    ));
    assert_json_string_field_after(
        &info,
        "\"ownershipGate\":{",
        1,
        "renderOwnershipBlockedReason",
        "multi-command-single-index-row",
    );
    assert_json_string_array_field_after(
        &info,
        "\"ownershipGate\":{",
        1,
        "renderOwnershipBlockedReasons",
        &[
            "multi-command-single-index-row",
            "mixed-raw-and-segment-cohorts",
            "row-command-reference-not-one-to-one",
        ],
    );
    assert_json_number_field_after(&info, "\"ownershipGate\":{", 1, "commandCount", "7");
    assert_json_number_field_after(&info, "\"ownershipGate\":{", 1, "rawSpanCommandCount", "1");
    assert_json_number_field_after(
        &info,
        "\"ownershipGate\":{",
        1,
        "segmentBackedCommandCount",
        "6",
    );
    assert_json_bool_field_after(
        &info,
        "\"ownershipGate\":{",
        1,
        "oneToOneRowCommandReferenceCandidate",
        false,
    );
    assert_json_string_field_after(
        &info,
        "\"offsetFieldAuthorityGate\":{",
        1,
        "renderPromotionBlockedReason",
        "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
    );
    assert_json_number_field_after(
        &info,
        "\"offsetFieldAuthorityGate\":{",
        1,
        "commandRelativeOffsetFieldReferenceCount",
        "1",
    );
    assert_json_number_field_after(
        &info,
        "\"offsetFieldAuthorityGate\":{",
        1,
        "sourceSegmentRelativeOffsetFieldReferenceCount",
        "6",
    );
    assert_json_string_field_after(
        &info,
        "\"rowFanoutSegmentOwnerGate\":{",
        1,
        "renderPromotionBlockedReason",
        "fdm-index-row-fanout-segment-owner-multi-command-single-row",
    );
    assert_json_number_field_after(
        &info,
        "\"rowFanoutSegmentOwnerGate\":{",
        1,
        "maxRowFanout",
        "4",
    );
    assert_json_bool_field_after(
        &info,
        "\"rowFanoutSegmentOwnerGate\":{",
        1,
        "singleRowBacksMultipleCommandsCandidate",
        true,
    );
    assert_json_string_field_after(
        &info,
        "\"primitiveOwnershipAdmissionGate\":{",
        1,
        "renderPromotionBlockedReason",
        "multi-command-single-index-row",
    );
    assert_json_string_array_field_after(
        &info,
        "\"primitiveOwnershipAdmissionGate\":{",
        1,
        "renderPromotionBlockedReasons",
        &[
            "multi-command-single-index-row",
            "mixed-raw-and-segment-cohorts",
            "row-command-reference-not-one-to-one",
            "fdm-index-offset-field-authority-mixed-command-and-segment-fields",
            "fdm-index-row-fanout-segment-owner-multi-command-single-row",
            "fdm-index-role-row-fanout-multi-command-single-row",
            "fdm-index-role-vector-offset-authority-valid-vector-offset-missing",
            "fdm-index-role-valid-vector-offset-missing",
            "role-paint-order-continuity-unproven",
            "role-paint-order-authority-unproven",
        ],
    );
    assert_json_number_field_after(
        &info,
        "\"primitiveOwnershipAdmissionGate\":{",
        1,
        "rolePaintOrderBlockedGroupCount",
        "2",
    );
    assert_json_number_field_after(
        &info,
        "\"primitiveOwnershipAdmissionGate\":{",
        1,
        "rolePaintOrderAuthorityPendingGroupCount",
        "2",
    );
    assert_json_string_field_after(
        &info,
        "\"indexRowOrderPromotionGate\":{",
        1,
        "renderPromotionBlockedReason",
        "fdm-index-row-order-reference-not-one-to-one",
    );
    assert_json_string_array_field_after(
        &info,
        "\"indexRowOrderPromotionGate\":{",
        1,
        "renderPromotionBlockedReasons",
        &[
            "fdm-index-row-order-reference-not-one-to-one",
            "fdm-index-row-order-single-row-backs-multiple-commands",
            "fdm-index-row-order-valid-vector-offset-missing",
            "fdm-index-row-order-offset-namespace-mixed",
            "role-paint-order-continuity-unproven",
            "role-paint-order-authority-unproven",
        ],
    );
    assert_json_number_field_after(
        &info,
        "\"indexRowOrderPromotionGate\":{",
        1,
        "uniqueRowIndexCount",
        "3",
    );
    assert!(info.contains("\"roleCandidate\":\"line-candidate\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"commandRelativeOffsets\":[1992,2024],\"rowIndexes\":[40],\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":1,\"oneToOneRowCommandReferenceCandidate\":false,\"singleRowBacksMultipleCommandsCandidate\":true,\"rowOrderMatchesCommandOrderCandidate\":true,\"rowCommandPairs\":[{\"rowIndex\":40,\"commandRelativeOffset\":1992,\"matchKind\":\"source-segment-relative-offset-field\"},{\"rowIndex\":40,\"commandRelativeOffset\":2024,\"matchKind\":\"source-segment-relative-offset-field\"}],\"roleVectorOffsetAuthorityGate\":{\"basis\":\"fdm-index-role-vector-offset-authority-gate\",\"source\":\"FDMIndex.vectorOffset+FDMIndex role offset fields\",\"decoded\":false,\"sourceBacked\":true,\"roleCandidate\":\"line-candidate\",\"roleVectorOffsetAuthorityDecoded\":false,\"renderPromotionContribution\":\"fdm-index-role-vector-offset-authority-gate\",\"renderPromotionBlockedReason\":\"fdm-index-role-vector-offset-authority-valid-vector-offset-missing\",\"referenceCount\":2,\"validVectorOffsetReferenceCount\":0,\"invalidVectorOffsetReferenceCount\":2,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"validCommandRelativeOffsetFieldReferenceCount\":0,\"validSourceSegmentRelativeOffsetFieldReferenceCount\":0,\"invalidCommandRelativeOffsetFieldReferenceCount\":0,\"invalidSourceSegmentRelativeOffsetFieldReferenceCount\":2,\"allValidReferencesUseCommandRelativeOffsetField\":false,\"allValidReferencesUseSourceSegmentRelativeOffsetField\":false,\"mixedOffsetNamespacesAmongValidReferences\":false,\"allReferencesHaveInvalidVectorOffset\":true},\"roleFanoutSegmentOwnerGate\":{\"basis\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex role row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true,\"roleCandidate\":\"line-candidate\",\"roleOwnershipDecoded\":false,\"segmentOwnerDecoded\":false,\"renderPromotionContribution\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"renderPromotionBlockedReason\":\"fdm-index-role-row-fanout-multi-command-single-row\",\"referenceCount\":2,\"uniqueCommandRelativeOffsetCount\":2,\"uniqueRowIndexCount\":1,\"commandRelativeOffsetFieldReferenceCount\":0,\"sourceSegmentRelativeOffsetFieldReferenceCount\":2,\"fanoutRowCount\":1,\"fanoutReferenceCount\":2,\"fanoutCommandRelativeOffsetFieldReferenceCount\":0,\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":2,\"maxRowFanout\":2,\"oneToOneRowCommandReferenceCandidate\":false,\"singleRowBacksMultipleCommandsCandidate\":true,\"mixedOffsetFieldNamespaces\":false,\"fanoutRowsUseCommandRelativeOffsetFields\":false,\"fanoutRowsUseSourceSegmentOffsetFields\":true,\"rowsWithMultipleCommandRefs\":[{\"rowIndex\":40,\"commandReferenceCount\":2,\"commandRelativeOffsets\":[1992,2024],\"matchKinds\":[\"source-segment-relative-offset-field\"]}]}"));
    assert!(info.contains("\"paintOrderContinuityProfile\":{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false,\"commandRelativeOffsetSpanMin\":1992,\"commandRelativeOffsetSpanMax\":2024,\"roleCommandCount\":2,\"commandCountInSpan\":2,\"interleavedNonRoleCommandCount\":0,\"hasInterleavedNonRoleCommands\":false,\"maxCommandOffsetGap\":32,\"commandOffsetContinuityScore\":1.000,\"spanContiguousCandidate\":true,\"paintOrderAuthorityPending\":true,\"continuityBlocked\":false,\"renderPromotionBlockedReason\":\"role-paint-order-authority-unproven\"}"));
    assert!(info.contains("\"relativeOffset\":1992,\"primitiveKind\":\"polyline\",\"markerHex\":\"ff000160\",\"sourceSegmentBacked\":true,\"sourceSegmentRelativeOffset\":1864,\"roleCandidates\":[\"line-candidate\",\"connector-candidate\"]"));
    assert!(info.contains("\"indexRowReferenceCandidates\":[{\"rowIndex\":40,\"indexOffset\":900,\"vectorOffset\":3729719295,\"validVectorOffset\":false,\"offsetField\":\"bbox.left\",\"offsetValue\":1864,\"matchKind\":\"source-segment-relative-offset-field\",\"decoded\":false}]"));
    assert!(info.contains("\"primitiveKind\":\"cubicBezier\""));
    assert!(info.contains("\"primitiveKind\":\"ellipse\""));
    assert!(info.contains("\"curveSegmentCount\":1"));
    assert!(info.contains("\"fdmTextCount\":15"));
    assert!(info.contains("\"text\":\"９㎝\",\"textOffset\":196,\"markerOffset\":0"));
    assert!(info.contains("\"text\":\"110°\",\"textOffset\":2260,\"markerOffset\":2054"));
    assert!(info.contains("\"sourceSpanCandidate\":{\"width\":782,\"height\":1135}"));

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"embeddingFrameDiagnostic\""));
    assert!(layer_tree.contains("\"sourcePath\":\"/EmbedItems/EmbeddingInfo\""));
    assert!(layer_tree.contains("\"embeddingIndex\":24"));
    assert!(layer_tree.contains("\"frameRef\":1"));
    assert!(layer_tree.contains(
        "\"pageAssociation\":{\"source\":\"JSFart.Art.2 frameRef source order\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"frameRefs\":[1,16],\"sourceOrderIndex\":0,\"pageNumber\":1}"
    ));
    assert!(layer_tree.contains("\"matchedFrameRecord\":{"));
    assert!(layer_tree.contains("\"linkedJseq3Formula\":{\"format\":\"JSEQ3Contents\""));
    assert!(layer_tree.contains("\"textMarkerCount\":4"));
    assert!(layer_tree.contains("\"textTokenCount\":4"));
    assert!(layer_tree.contains("\"textRunCount\":3"));
    assert!(layer_tree.contains("\"embeddedPressSnapshot\":{\"format\":\"JSSnapShot32\""));
    assert!(layer_tree.contains("\"vectorSegmentCount\":51"));
    assert!(layer_tree.contains("\"vectorSegmentCount\":51,\"renderable\":false"));
    assert!(layer_tree.contains("\"type\":\"titleArtProjection\""));
    assert!(layer_tree.contains("\"projectionKind\":\"successDataTestTitleArtProjection\""));
    assert!(layer_tree.contains("\"source\":\"jsfartArtEmbeddedPressSnapshot\""));
    assert!(layer_tree.contains("\"bbox\":{\"x\":76.687,\"y\":106.887"));
    assert!(layer_tree.contains("\"type\":\"fdmReferenceProjection\""));
    assert!(layer_tree.contains("\"projectionKind\":\"successDataTestFdmReferenceProjection\""));
    assert!(layer_tree.contains("\"role\":\"q4-angle-diagrams\""));
    assert!(layer_tree.contains("\"role\":\"q5-solid-diagram\""));
    assert!(layer_tree.contains("\"sourceCohort\":{\"provenance\":\"fdm-vector-command\",\"ownershipBasis\":\"fdmVectorCommandProvenance\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"mixed-raw-and-segment-cohorts\",\"sourceVectorOffsetStart\":308,\"sourceVectorOffsetEnd\":1780,\"commandRelativeOffsets\":[308,342,374,406,438,470,504,538,570,602,634,874,1048,1126,1158,1190,1430,1604,1730,1780],\"sourceVectorOffsetCommandCount\":20,\"segmentBackedCommandCount\":2,\"rawSpanCommandCount\":18,\"sourceSegmentCohortCount\":2,\"sourceSegmentRelativeOffsets\":[690,1246]}"));
    assert!(layer_tree.contains("\"primitiveOwnershipComparison\":{\"basis\":\"fdmVectorCommandProvenance+sourceGeometryLocalSubdiagram\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"primitive-role-and-paint-order-unproven\",\"commandCount\":20,\"mainCircleAnchorCount\":3,\"lineCandidateCount\":11,\"radialLineCandidateCount\":0,\"chordCandidateCount\":0,\"arcCandidateCount\":6,\"connectorCandidateCount\":8,\"surfaceBoundaryCandidateCount\":2"));
    assert!(layer_tree.contains(
        "\"indexRowReferenceCandidateCount\":20,\"validVectorOffsetIndexRowReferenceCount\":0"
    ));
    assert!(layer_tree.contains("\"offsetFieldAuthorityGate\":{\"basis\":\"fdm-index-offset-field-authority-gate\",\"source\":\"FDMIndex row offset fields+FDMVector command provenance\",\"decoded\":false,\"sourceBacked\":true,\"offsetFieldAuthorityDecoded\":false,\"renderPromotionContribution\":\"fdm-index-offset-field-authority-gate\",\"renderPromotionBlockedReason\":\"fdm-index-offset-field-authority-mixed-command-and-segment-fields\",\"commandCount\":20,\"referenceCount\":20"));
    assert!(layer_tree.contains("\"rowFanoutSegmentOwnerGate\":{\"basis\":\"fdm-index-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true,\"rowFanoutDecoded\":false,\"segmentOwnerDecoded\":false,\"renderPromotionContribution\":\"fdm-index-row-fanout-segment-owner-gate\",\"renderPromotionBlockedReason\":\"fdm-index-row-fanout-segment-owner-offset-namespace-mixed\",\"commandCount\":20,\"referenceCount\":20"));
    assert!(layer_tree.contains("\"indexRowOrderPromotionGate\":{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false,\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\",\"renderPromotionBlockedReason\":\"fdm-index-row-order-valid-vector-offset-missing\",\"renderPromotionBlockedReasons\":[\"fdm-index-row-order-valid-vector-offset-missing\",\"fdm-index-row-order-offset-namespace-mixed\",\"role-paint-order-continuity-unproven\"],\"commandCount\":20,\"referencedCommandCount\":20,\"unreferencedCommandCount\":0,\"uniqueRowIndexCount\":20,\"referenceCount\":20"));
    assert!(layer_tree.contains("\"subdiagrams\":[{\"index\":0,\"role\":\"q4-angle-diagrams\",\"groupingSource\":\"nearest-main-circle-source-center\",\"groupingDecoded\":false,\"paintOrderDecoded\":false,\"anchorRelativeOffset\":308,\"anchorSourcePoint\":{\"x\":-15184,\"y\":-9613},\"commandCount\":7,\"sourceCohort\":{\"provenance\":\"fdm-vector-command\",\"ownershipBasis\":\"fdmVectorCommandProvenance\",\"ownershipProven\":false,\"ownershipPromotionBlockedReason\":\"source-owner-candidate-unproven\""));
    assert!(layer_tree.contains("\"lineCandidateCount\":4,\"radialLineCandidateCount\":2,\"chordCandidateCount\":2,\"arcCandidateCount\":2,\"connectorCandidateCount\":2,\"surfaceBoundaryCandidateCount\":2"));
    assert!(layer_tree.contains("\"sourceVectorOffsetStart\":1830,\"sourceVectorOffsetEnd\":2190,\"commandRelativeOffsets\":[1830,1924,1958,1992,2024,2156,2190],\"sourceVectorOffsetCommandCount\":7,\"segmentBackedCommandCount\":6,\"rawSpanCommandCount\":1,\"sourceSegmentCohortCount\":2,\"sourceSegmentRelativeOffsets\":[1864,2100]"));
    assert!(layer_tree.contains("\"lineCandidateCount\":2,\"radialLineCandidateCount\":0,\"chordCandidateCount\":0,\"arcCandidateCount\":4,\"connectorCandidateCount\":3,\"surfaceBoundaryCandidateCount\":1"));
    assert!(layer_tree.contains(
        "\"indexRowReferenceCandidateCount\":7,\"validVectorOffsetIndexRowReferenceCount\":0"
    ));
    assert!(layer_tree.contains("\"offsetFieldAuthorityGate\":{\"basis\":\"fdm-index-offset-field-authority-gate\",\"source\":\"FDMIndex row offset fields+FDMVector command provenance\",\"decoded\":false,\"sourceBacked\":true,\"offsetFieldAuthorityDecoded\":false,\"renderPromotionContribution\":\"fdm-index-offset-field-authority-gate\",\"renderPromotionBlockedReason\":\"fdm-index-offset-field-authority-mixed-command-and-segment-fields\",\"commandCount\":7,\"referenceCount\":7"));
    assert!(layer_tree.contains("\"rowFanoutSegmentOwnerGate\":{\"basis\":\"fdm-index-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true,\"rowFanoutDecoded\":false,\"segmentOwnerDecoded\":false,\"renderPromotionContribution\":\"fdm-index-row-fanout-segment-owner-gate\",\"renderPromotionBlockedReason\":\"fdm-index-row-fanout-segment-owner-multi-command-single-row\",\"commandCount\":7,\"referenceCount\":7"));
    assert!(layer_tree.contains("\"indexRowOrderPromotionGate\":{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false,\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\",\"renderPromotionBlockedReason\":\"fdm-index-row-order-reference-not-one-to-one\",\"renderPromotionBlockedReasons\":[\"fdm-index-row-order-reference-not-one-to-one\",\"fdm-index-row-order-single-row-backs-multiple-commands\",\"fdm-index-row-order-valid-vector-offset-missing\",\"fdm-index-row-order-offset-namespace-mixed\",\"role-paint-order-continuity-unproven\",\"role-paint-order-authority-unproven\"],\"commandCount\":7,\"referencedCommandCount\":7,\"unreferencedCommandCount\":0,\"uniqueRowIndexCount\":3,\"referenceCount\":7"));
    assert!(!layer_tree.contains("\"type\":\"pageFrameShape\""));
    assert!(!layer_tree.contains("\"role\":\"titleRoundedFrame\""));
    assert!(layer_tree.contains("\"placementMode\":\"frameRecordContentOffsetAnchor\""));
    assert!(layer_tree.contains("\"contentLeftAdjustment\":{\"sourceUnits\":114,\"cssPx\":4.309}"));
    assert!(layer_tree.contains(
        "\"horizontalPlacementGate\":{\"source\":\"JSFart2Contents.frameCandidate\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"basis\":\"jsfartContentLeft\",\"frameRecordX\":80.995,\"contentLeftOnlyX\":76.687,\"frameX\":76.687,\"pathX\":76.687,\"candidateFrameX\":72.907,\"candidatePathX\":80.995,\"candidateBasis\":\"jsfartFrameOuterEdgePlusFrameRecordContentOrigin\",\"contentLeftAdjustmentCssPx\":4.309,\"strokeWidthCandidateSourceUnits\":100,\"strokeOuterAdjustmentCssPx\":3.780,\"renderPromoted\":false,\"renderPromotionBlockedReason\":\"frame-content-split-horizontal-semantics-unproven\"}"
    ));
    assert!(layer_tree.contains(
        "\"sourceFrameRenderTrace\":{\"source\":\"JSFart2Contents.frameCandidate+/Frame\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"frameRef\":1,\"frameRecordObjectId\":1,\"frameRefMatchesObjectId\":true,\"sourceOuterWidthUnits\":13260,\"frameRecordWidthUnits\":13260,\"outerWidthMatchesFrameRecord\":true"
    ));
    assert!(layer_tree.contains(
        "\"sourceOuterHeightUnits\":1327,\"frameRecordHeightUnits\":1327,\"outerHeightMatchesFrameRecord\":true,\"sourceContentWidthUnits\":13031,\"sourceContentHeightUnits\":1054,\"horizontalPlacementBasis\":\"jsfartContentLeft\",\"selectedFrameX\":76.687,\"candidateFrameX\":72.907,\"frameScaleYBasis\":\"jsfartContentHeight\",\"frameScaleYSourceUnits\":1054,\"traceConclusion\":\"frame-record-and-jsfart-outer-size-agree\",\"renderPromotionBlockedReason\":\"frame-content-split-horizontal-semantics-unproven\""
    ));
    assert!(layer_tree.contains("\"contentTopAdjustment\":{\"sourceUnits\":105,\"cssPx\":4.996}"));
    assert!(layer_tree.contains(
        "\"verticalStrokeCenterAdjustment\":{\"cssPx\":1.067,\"source\":\"jsfart-frame-stroke-centered-on-border\"}"
    ));
    assert!(layer_tree.contains(
        "\"embeddingFrameSize\":{\"primaryWidth\":13260,\"primaryHeight\":1327,\"frameWidth\":13260,\"frameHeight\":1327}"
    ));
    assert!(layer_tree.contains("\"sourceScale\":{\"x\":0.037795,\"y\":0.037795}"));
    assert!(layer_tree.contains("\"frameScale\":{\"x\":0.037795,\"y\":0.047585,\"yBasis\":\"jsfartContentHeight\",\"ySourceUnits\":1054}"));
    assert!(layer_tree.contains(
        "\"pathScaleDiagnostic\":{\"source\":\"embeddedPressMainOutlinePathSampledBbox\",\"pixelChange\":false,\"scaleComparisonDecoded\":false,\"currentRendererPathScale\":\"sourceScale\",\"frameClipScale\":\"frameScale\",\"renderPromotionBlockedReason\":\"title-art-y-scale-basis-unproven\""
    ));
    assert!(layer_tree.contains("\"sourceScaleBbox\":{\"x\":"));
    assert!(layer_tree.contains("\"frameScaleBbox\":{\"x\":"));
    assert!(layer_tree.contains("\"frameRecordRect\":{\"sourcePath\":\"/Frame\""));
    assert!(layer_tree.contains("\"objectTypeHex\":\"0x002b\""));
    assert!(
        layer_tree
            .contains("\"sourceUnits\":{\"x\":2143,\"y\":2932,\"width\":13260,\"height\":1327}")
    );
    assert!(layer_tree.contains("\"cssPx\":{\"x\":80.995,\"y\":110.816"));
    assert!(layer_tree.contains("\"vectorSegmentCount\":9895"));
    assert!(layer_tree.contains("\"vectorPathCount\":552"));
    assert!(layer_tree.contains("\"outlinePathCount\":22"));
    assert!(layer_tree.contains("\"texturePathCount\":530"));
    assert!(layer_tree.contains("\"textureBezierHeaderSummary\":{\"pathCount\":530,\"pointCount\":13,\"byteCount\":104,\"flags\":1,\"flagsHex\":\"0x00000001\",\"homogeneous\":true}"));
    assert!(layer_tree.contains("\"paintStateTransitions\":["));
    assert!(layer_tree.contains(
        "\"pathKind\":\"outline\",\"startPathIndex\":0,\"endPathIndex\":10,\"pathCount\":11"
    ));
    assert!(layer_tree.contains(
        "\"currentState\":{\"record48Word0\":\"0x00000001\",\"record70Word0\":\"0x0000002c\",\"record70Word3\":\"0x0000000a\",\"record82Word5\":\"0x0000002f\"}"
    ));
    assert!(layer_tree.contains(
        "\"pathKind\":\"texture\",\"startPathIndex\":11,\"endPathIndex\":540,\"pathCount\":530"
    ));
    assert!(layer_tree.contains(
        "\"pathKind\":\"outline\",\"startPathIndex\":541,\"endPathIndex\":551,\"pathCount\":11"
    ));
    assert!(layer_tree.contains(
        "\"titleArtPaintStateSequence\":{\"source\":\"embeddedPressVectorPathSourceOrder\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"pathCount\":552,\"explicitTransitionCount\":33"
    ));
    assert!(layer_tree.contains(
        "\"pathKindRuns\":[{\"pathKind\":\"outline\",\"startPathIndex\":0,\"endPathIndex\":10,\"pathCount\":11},{\"pathKind\":\"texture\",\"startPathIndex\":11,\"endPathIndex\":540,\"pathCount\":530},{\"pathKind\":\"outline\",\"startPathIndex\":541,\"endPathIndex\":551,\"pathCount\":11}]"
    ));
    assert!(layer_tree.contains(
        "\"frontErasePaintTransitionGate\":{\"source\":\"embeddedPressVectorPathSourceOrder+stateTransitions\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"partitionPresent\":true,\"interstitialTexturePathCount\":530,\"explicitStateTexturePathCount\":11,\"inheritedTexturePathCount\":519"
    ));
    assert!(layer_tree.contains(
        "\"shadowLastPathIndex\":10,\"interstitialFirstPathIndex\":11,\"interstitialLastPathIndex\":540,\"mainFirstPathIndex\":541,\"shadowToInterstitialBoundaryAdjacent\":true,\"interstitialToMainBoundaryAdjacent\":true"
    ));
    assert!(layer_tree.contains(
        "\"record48SeparatesShadowFromTextureAndMain\":true,\"record48SeparatesTextureFromMain\":false,\"record70Word0SeparatesTextureFromMain\":false,\"record82Word5SeparatesTextureFromMain\":true,\"record82Word5MatchesShadow\":true,\"record82Word3IsWhitePaintCandidate\":true"
    ));
    assert!(layer_tree.contains(
        "\"paintIntentInference\":\"shadow-state-texture-inside-main-boundary-ambiguous\",\"transitionBoundaryClass\":\"source-order-bracketed-interstitial-texture-block\",\"promotionReady\":false,\"renderPromotionBlockedReason\":\"front-erase-transition-boundary-main-state-not-separated\""
    ));
    assert!(layer_tree.contains(
        "\"pathIndex\":11,\"pathKind\":\"texture\",\"sourceOrderRole\":\"interstitialTextureBlock\",\"stateSourcePathIndex\":11,\"nextExplicitPathIndex\":27,\"inheritedSpanEndPathIndex\":26,\"inheritedPathCount\":16,\"inheritedTexturePathCount\":16"
    ));
    assert!(layer_tree.contains(
        "\"stateRecords\":[{\"recordIndex\":0,\"recordType\":70,\"recordTypeHex\":\"0x46\",\"recordOffset\":18708,\"payloadByteLength\":4,\"wordCount\":1,\"words\":[\"0x00\"]"
    ));
    assert!(layer_tree.contains(
        "{\"recordIndex\":6,\"recordType\":130,\"recordTypeHex\":\"0x82\",\"recordOffset\":18780,\"payloadByteLength\":36,\"wordCount\":9,\"words\":[\"0x1c\",\"0x10\",\"0x00\",\"0xffffff\",\"0x00\",\"0x2f\""
    ));
    assert!(layer_tree.contains(
        "\"pathIndex\":63,\"pathKind\":\"texture\",\"sourceOrderRole\":\"interstitialTextureBlock\",\"stateSourcePathIndex\":63,\"nextExplicitPathIndex\":154,\"inheritedSpanEndPathIndex\":153,\"inheritedPathCount\":91,\"inheritedTexturePathCount\":91"
    ));
    assert!(layer_tree.contains(
        "\"record46Word0Sequence\":[\"0x00\",\"0x00\"],\"record48Word0Sequence\":[\"0x00\"],\"record60Word0Sequence\":[\"0x10\",\"0x11\"],\"record65Word0Sequence\":[\"0x10\",\"0x11\"],\"record70Word0Sequence\":[\"0x1c\"]"
    ));
    assert!(layer_tree.contains(
        "\"record82Word0Sequence\":[\"0x1c\"],\"record82Word3Sequence\":[\"0xffffff\"],\"record82Word5Sequence\":[\"0x2f\"],\"textureBezierHeader\":{\"pointCount\":13,\"byteCount\":104,\"flags\":1,\"flagsHex\":\"0x00000001\"}"
    ));
    assert!(layer_tree.contains(
        "\"pathIndex\":541,\"pathKind\":\"outline\",\"sourceOrderRole\":\"mainOutlines\",\"stateSourcePathIndex\":541,\"nextExplicitPathIndex\":542,\"inheritedSpanEndPathIndex\":541,\"inheritedPathCount\":1,\"inheritedTexturePathCount\":0"
    ));
    assert!(layer_tree.contains(
        "{\"recordIndex\":5,\"recordType\":130,\"recordTypeHex\":\"0x82\",\"recordOffset\":94960,\"payloadByteLength\":36,\"wordCount\":9,\"words\":[\"0x1c\",\"0x10\",\"0x00\",\"0xffffff\",\"0x00\",\"0x10\""
    ));
    assert!(layer_tree.contains("\"renderedSegmentCount\":"));
    assert!(layer_tree.contains("\"renderedPathCount\":22"));
    assert!(layer_tree.contains("\"renderedTexturePathCount\":530"));
    assert!(layer_tree.contains("\"stateTaggedTexturePathCount\":11"));
    assert!(layer_tree.contains("\"stateTaggedTextureWord5Values\":[\"0x2f\"]"));
    assert!(layer_tree.contains("\"frontTexturePathCount\":0"));
    assert!(layer_tree.contains("\"frontTextureWord5Values\":[]"));
    assert!(layer_tree.contains("\"effectiveFrontTextureWord5Values\":[]"));
    assert!(layer_tree.contains("\"frontEraseTexturePathCount\":530"));
    assert!(layer_tree.contains("\"frontEraseTextureWord5Values\":[\"0x2f\"]"));
    assert!(layer_tree.contains("\"frontEraseTextureRecord70Word0Values\":[\"0x1c\"]"));
    assert!(layer_tree.contains("\"frontEraseTextureOpacity\":0.470"));
    assert!(
        layer_tree
            .contains("\"frontEraseTextureOpacitySource\":\"embedded-press-0x82-word5-percent\"")
    );
    assert!(layer_tree.contains(
        "\"frontEraseTextureDirectGrayCandidate\":{\"source\":\"embeddedPressRecord82Word5DirectGrayProbe\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"word5\":47,\"fillColor\":\"#787878\",\"renderPromotionBlockedReason\":\"direct-gray-channel-probe-not-proven-as-paint-semantics\"}"
    ));
    assert!(layer_tree.contains(
        "\"frontEraseTextureSourcePaintCandidate\":{\"source\":\"frontEraseTextureSourcePaintProbe\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"paintColor\":\"#ffffff\",\"paintColorSource\":\"JSFart2Contents.paintColorCandidate\",\"solidPaintFillColor\":\"#ffffff\",\"activePrecompositedFillColor\":\"#818181\",\"activeFillColorSource\":\"source-paint-with-front-erase-opacity-over-front-fill\",\"renderPromotionBlockedReason\":\"solid-source-paint-semantics-unproven\"}"
    ));
    assert!(layer_tree.contains(
        "\"frontEraseTextureSpanCoverageProbe\":{\"source\":\"embeddedPressExplicitTextureStateSpans\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"texturePathCount\":530,\"explicitStateTexturePathCount\":11,\"inheritedTexturePathCount\":519,\"spanCount\":11,\"spanPathCounts\":[16,36,91,55,16,55,73,22,64,52,50],\"minSpanPathCount\":16,\"maxSpanPathCount\":91,\"meanSpanPathCount\":48.182,\"coverageConclusion\":\"explicit-state-spans-cover-all-front-erase-texture-paths\",\"renderPromotionBlockedReason\":\"span-density-and-clip-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"spans\":[{\"statePathIndex\":11,\"inheritedSpanEndPathIndex\":26,\"pathCount\":16,\"texturePathCount\":16,\"record48Word0Values\":[\"0x00\"],\"record70Word0Values\":[\"0x1c\"],\"record82Word3Values\":[\"0xffffff\"],\"record82Word5Values\":[\"0x2f\"]}"
    ));
    assert!(layer_tree.contains(
        "{\"statePathIndex\":491,\"inheritedSpanEndPathIndex\":540,\"pathCount\":50,\"texturePathCount\":50,\"record48Word0Values\":[\"0x00\"],\"record70Word0Values\":[\"0x1c\"],\"record82Word3Values\":[\"0xffffff\"],\"record82Word5Values\":[\"0x2f\"]}"
    ));
    assert!(layer_tree.contains(
        "\"frontEraseTexturePathSource\":\"interstitial-between-shadow-and-main-outlines\""
    ));
    assert!(layer_tree.contains(
        "\"frontEraseTextureStateSummary\":{\"role\":\"frontEraseTextureCandidate\",\"pathKind\":\"texture\",\"pathCount\":530,\"firstPathIndex\":11,\"lastPathIndex\":540,\"explicitStatePathCount\":11,\"inheritedStatePathCount\":519,\"stateRecordCount\":100"
    ));
    assert!(layer_tree.contains(
        "\"recordTypeHex\":\"0x82\",\"recordCount\":11,\"maxWordCount\":9,\"columns\":[{\"wordIndex\":0,\"values\":[\"0x1c\"]},{\"wordIndex\":1,\"values\":[\"0x10\"]}"
    ));
    assert!(layer_tree.contains(
        "\"frontEraseTextureRoleGate\":{\"source\":\"embeddedPressPathStateRecordComparison\",\"decoded\":false,\"pixelChange\":true,\"frontEraseTexturePathCount\":530,\"interstitialTexturePathCount\":530,\"record48SeparatesShadowFromTextureAndMain\":true,\"record48SeparatesTextureFromMain\":false,\"sourceOrderFrontEraseCandidate\":true"
    ));
    assert!(layer_tree.contains(
        "\"candidateBasis\":\"source-order-interstitial-front-erase-texture\",\"visibleRenderPathCount\":0,\"renderPromoted\":false"
    ));
    assert!(layer_tree.contains(
        "\"frontTexturePromotionBasis\":\"source-order-interstitial-front-erase-texture\",\"frontTexturePromotionRisk\":\"source-order-texture-shares-record48-with-main-outline\""
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"front-erase-texture-over-main-face-semantics-unproven\",\"frontEraseVisibleProbeGate\":{\"source\":\"frontEraseTextureVisibleAB+visualReview\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"currentVisiblePathCount\":0,\"allVisiblePathCount\":530,\"explicitStateVisiblePathCount\":11"
    ));
    assert!(layer_tree.contains(
        "\"sourcePaintRenderTrace\":{\"source\":\"JSFart2Contents.paintCandidateRawWords+frontFillRenderColorGate\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"rawPaintCandidate\":{\"styleWord1Hex\":\"0x02141030\",\"styleWord2Hex\":\"0x02141018\",\"paintColorCandidateHex\":\"0x00ffffff\",\"paintColorCss\":\"#ffffff\",\"paintFlagCandidateHex\":\"0x00000001\",\"effectWordCandidateHex\":\"0x0000000a\"}"
    ));
    assert!(layer_tree.contains(
        "\"selectedRenderFillColor\":\"#111111\",\"selectedRenderFillSource\":\"conservative-front-fill-fallback-source-paint-mismatch\",\"selectedRenderFillSourceBacked\":false,\"sourcePaintColor\":\"#ffffff\",\"sourcePaintColorSource\":\"JSFart2Contents.paintColorCandidate\",\"sourcePaintColorMatchesRenderFill\":false,\"renderTexturePathSource\":\"source-order-interstitial-front-erase-texture\",\"renderPromotionBlockedReason\":\"front-erase-texture-over-main-face-semantics-unproven\",\"traceConclusion\":\"source-paint-present-but-render-fill-not-promoted\""
    ));
    let title_texture_do_not_render_chain = [
        (
            "source paint is present but must not promote the black front fill",
            "\"sourcePaintRenderTrace\":{\"source\":\"JSFart2Contents.paintCandidateRawWords+frontFillRenderColorGate\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false",
        ),
        (
            "source paint remains separate from the selected conservative render fill",
            "\"selectedRenderFillColor\":\"#111111\",\"selectedRenderFillSource\":\"conservative-front-fill-fallback-source-paint-mismatch\",\"selectedRenderFillSourceBacked\":false,\"sourcePaintColor\":\"#ffffff\"",
        ),
        (
            "source-order front erase remains blocked despite having texture paths",
            "\"renderTexturePathSource\":\"source-order-interstitial-front-erase-texture\",\"renderPromotionBlockedReason\":\"front-erase-texture-over-main-face-semantics-unproven\",\"traceConclusion\":\"source-paint-present-but-render-fill-not-promoted\"",
        ),
        (
            "visible front erase probe stays non-rendering even when all texture paths improve RMS",
            "\"frontEraseVisibleProbeGate\":{\"source\":\"frontEraseTextureVisibleAB+visualReview\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"currentVisiblePathCount\":0,\"allVisiblePathCount\":530,\"explicitStateVisiblePathCount\":11",
        ),
        (
            "all-visible probe is rejected as gray overpaint rather than knockout proof",
            "\"allVisibleRmsImproves\":true,\"allVisibleVisualRejected\":true,\"allVisibleRejectedReason\":\"gray-overpaint-not-distressed-knockout\",\"explicitStateOnlyMaterialImprovement\":false,\"renderPromotionBlockedReason\":\"front-erase-visible-rms-improvement-is-not-knockout-proof\"",
        ),
        (
            "texture bbox relation is evidence only, not knockout authority",
            "\"textureContainedByMainBbox\":false,\"textureContainedByShadowBbox\":false,\"textureContainedBySideSweepBbox\":true,\"frontFaceKnockoutDecoded\":false,\"clipSemanticsDecoded\":false,\"roleConclusion\":\"texture-bbox-overlaps-main-and-shadow-outline-bboxes\",\"renderPromotionBlockedReason\":\"texture-source-bbox-relation-is-bbox-only-not-knockout-proof\"",
        ),
    ];
    for (label, needle) in title_texture_do_not_render_chain {
        assert!(
            layer_tree.contains(needle),
            "missing title texture do-not-render chain evidence: {label}"
        );
    }
    assert!(layer_tree.contains(
        "\"currentTitleTightRms\":76.034,\"allVisibleTitleTightRms\":67.651,\"explicitStateVisibleTitleTightRms\":76.016,\"currentTopCropRms\":51.191,\"allVisibleTopCropRms\":48.814,\"explicitStateVisibleTopCropRms\":51.186"
    ));
    assert!(layer_tree.contains(
        "\"allVisibleRmsImproves\":true,\"allVisibleVisualRejected\":true,\"allVisibleRejectedReason\":\"gray-overpaint-not-distressed-knockout\",\"explicitStateOnlyMaterialImprovement\":false,\"renderPromotionBlockedReason\":\"front-erase-visible-rms-improvement-is-not-knockout-proof\""
    ));
    assert!(layer_tree.contains(
        "\"titleTexturePaintPhaseGate\":{\"source\":\"embeddedPressPathStateRecordComparison\",\"basis\":\"record46-word0-paint-phase-candidate\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"visibleRenderPathCount\":0"
    ));
    assert!(layer_tree.contains(
        "\"texturePathCount\":530,\"shadowOutlinePathCount\":11,\"mainOutlinePathCount\":11"
    ));
    assert!(layer_tree.contains("\"textureRecord46Word0Values\":[\"0x00\"]"));
    assert!(layer_tree.contains("\"shadowOutlineRecord46Word0Values\":[\"0x00\",\"0x01\"]"));
    assert!(layer_tree.contains("\"mainOutlineRecord46Word0Values\":[\"0x00\",\"0x01\"]"));
    assert!(layer_tree.contains("\"textureRecord48Word0Values\":[\"0x00\"]"));
    assert!(layer_tree.contains("\"shadowOutlineRecord48Word0Values\":[\"0x01\"]"));
    assert!(layer_tree.contains("\"mainOutlineRecord48Word0Values\":[\"0x00\"]"));
    assert!(layer_tree.contains("\"record46OneAppearsOnlyOnOutlines\":true"));
    assert!(layer_tree.contains("\"textureAndMainShareRecord46Zero\":true"));
    assert!(layer_tree.contains("\"textureAndShadowShareRecord46Zero\":true"));
    assert!(layer_tree.contains("\"record46SeparatesTextureFromOutlines\":false"));
    assert!(layer_tree.contains("\"record48SeparatesShadowFromTextureAndMain\":true"));
    assert!(layer_tree.contains("\"record48SeparatesTextureFromMain\":false"));
    assert!(layer_tree.contains("\"textureRecord60Word0Values\":[\"0x10\",\"0x11\"]"));
    assert!(layer_tree.contains("\"shadowOutlineRecord60Word0Values\":[\"0x10\",\"0x11\"]"));
    assert!(layer_tree.contains("\"mainOutlineRecord60Word0Values\":[\"0x10\",\"0x11\"]"));
    assert!(layer_tree.contains("\"textureRecord65Word0Values\":[\"0x10\",\"0x11\"]"));
    assert!(layer_tree.contains("\"shadowOutlineRecord65Word0Values\":[\"0x10\",\"0x11\"]"));
    assert!(layer_tree.contains("\"mainOutlineRecord65Word0Values\":[\"0x10\",\"0x11\"]"));
    assert!(layer_tree.contains("\"record60SharedAcrossRoles\":true"));
    assert!(layer_tree.contains("\"record65SharedAcrossRoles\":true"));
    assert!(layer_tree.contains("\"record60SeparatesTextureFromOutlines\":false"));
    assert!(layer_tree.contains("\"record65SeparatesTextureFromOutlines\":false"));
    assert!(layer_tree.contains(
        "\"promotionProofPolicy\":\"record46-must-separate-texture-from-outlines-and-record48-must-separate-main-role\""
    ));
    assert!(layer_tree.contains("\"record46PromotionProofReady\":false"));
    assert!(layer_tree.contains(
        "\"promotionProofBlockedReasons\":[\"record46-texture-outline-value-sets-overlap-or-missing\",\"record46-zero-shared-by-texture-and-main-outline\",\"record46-zero-shared-by-texture-and-shadow-outline\",\"record48-texture-main-role-separation-missing\",\"record60-shared-across-roles\",\"record65-shared-across-roles\"]"
    ));
    assert!(layer_tree.contains(
        "\"candidateBasis\":\"record46-one-outline-paint-phase-candidate\",\"renderPromotionBlockedReason\":\"record46-paint-phase-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"titleShadowPaintWordGate\":{\"source\":\"embeddedPressRecord70RoleComparison\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false"
    ));
    assert!(layer_tree.contains(
        "\"record70Word0SeparatesShadowFromTextureAndMain\":true,\"record70Word3SeparatesShadowFromTextureAndMain\":true,\"record70Word7SeparatesShadowFromTextureAndMain\":true,\"record70Word1SharedAcrossRoles\":true"
    ));
    assert!(layer_tree.contains(
        "\"record70Word0SeparatesTextureFromMain\":false,\"record70Word3SeparatesTextureFromMain\":false,\"record70Word7SeparatesTextureFromMain\":false"
    ));
    assert!(layer_tree.contains(
        "\"shadowEffectCandidate\":{\"basis\":\"record70.word0-percent-black-on-white\",\"word0\":44,\"opacity\":0.440,\"fillColor\":\"#8f8f8f\"}"
    ));
    assert!(layer_tree.contains(
        "\"interstitialTextureEffectCandidate\":{\"basis\":\"record70.word0-percent-black-over-shadow\",\"word0\":28,\"opacity\":0.280,\"baseFillColor\":\"#8f8f8f\",\"fillColor\":\"#676767\",\"renderPromoted\":false,\"renderPromotionBlockedReason\":\"record70-separates-shadow-but-not-interstitial-texture-from-main\"}"
    ));
    assert!(layer_tree.contains("\"renderPromotionBlockedReason\":\"none\""));
    assert!(layer_tree.contains(
        "{\"role\":\"shadowOutlines\",\"pathKind\":\"outline\",\"pathCount\":11,\"record70Word0Values\":[\"0x2c\"],\"record70Word1Values\":[\"0x11\"],\"record70Word3Values\":[\"0x0a\"],\"record70Word7Values\":[\"0x1c\"]}"
    ));
    assert!(layer_tree.contains(
        "{\"role\":\"interstitialTextureBlock\",\"pathKind\":\"texture\",\"pathCount\":530,\"record70Word0Values\":[\"0x1c\"],\"record70Word1Values\":[\"0x11\"],\"record70Word3Values\":[\"0x00\"],\"record70Word7Values\":[\"0x00\"]}"
    ));
    assert!(layer_tree.contains(
        "{\"role\":\"mainOutlines\",\"pathKind\":\"outline\",\"pathCount\":11,\"record70Word0Values\":[\"0x1c\"],\"record70Word1Values\":[\"0x11\"],\"record70Word3Values\":[\"0x00\"],\"record70Word7Values\":[\"0x00\"]}"
    ));
    assert!(layer_tree.contains(
        "\"titlePaintRoleSeparationMatrix\":{\"source\":\"embeddedPressRoleStateWordMatrix\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":false,\"rolePartitionBasis\":\"embeddedPressPathSourceOrder\",\"recordWordCandidateCount\":28,\"sharedAcrossAllRecordWordCount\":18,\"shadowUniqueRecordWordCount\":4,\"textureUniqueRecordWordCount\":0,\"mainUniqueRecordWordCount\":1"
    ));
    assert!(layer_tree.contains(
        "\"textureMainDisjointRecordWordCount\":1,\"shadowTextureSharedMainDisjointRecordWordCount\":1,\"missingRoleValueRecordWordCount\":4,\"textureOnlySeparatorPresent\":false,\"mainOnlySeparatorPresent\":true"
    ));
    assert!(layer_tree.contains(
        "\"matrixConclusion\":\"record-words-separate-main-from-shadow-state-texture-but-not-interstitial-texture-only\",\"renderPromotionBlockedReason\":\"no-record-word-separates-interstitial-texture-from-both-outline-roles\""
    ));
    assert!(layer_tree.contains(
        "{\"recordType\":130,\"recordTypeHex\":\"0x82\",\"wordIndex\":5,\"shadowValues\":[\"0x2f\"],\"textureValues\":[\"0x2f\"],\"mainValues\":[\"0x10\"],\"presentInAllRoles\":true,\"sharedAcrossAllRoles\":false,\"shadowDisjointFromTextureAndMain\":false,\"textureDisjointFromShadowAndMain\":false,\"mainDisjointFromShadowAndTexture\":true,\"textureMainDisjoint\":true,\"shadowTextureSharedMainDisjoint\":true,\"interpretation\":\"main-vs-shadow-state-texture-candidate\"}"
    ));
    assert!(layer_tree.contains(
        "{\"recordType\":70,\"recordTypeHex\":\"0x46\",\"wordIndex\":0,\"shadowValues\":[\"0x00\",\"0x01\"],\"textureValues\":[\"0x00\"],\"mainValues\":[\"0x00\",\"0x01\"],\"presentInAllRoles\":true,\"sharedAcrossAllRoles\":false,\"shadowDisjointFromTextureAndMain\":false,\"textureDisjointFromShadowAndMain\":false,\"mainDisjointFromShadowAndTexture\":false,\"textureMainDisjoint\":false,\"shadowTextureSharedMainDisjoint\":false,\"interpretation\":\"overlapping-or-ambiguous\"}"
    ));
    assert!(layer_tree.contains(
        "{\"recordType\":72,\"recordTypeHex\":\"0x48\",\"wordIndex\":0,\"shadowValues\":[\"0x01\"],\"textureValues\":[\"0x00\"],\"mainValues\":[\"0x00\"],\"presentInAllRoles\":true,\"sharedAcrossAllRoles\":false,\"shadowDisjointFromTextureAndMain\":true,\"textureDisjointFromShadowAndMain\":false,\"mainDisjointFromShadowAndTexture\":false,\"textureMainDisjoint\":false,\"shadowTextureSharedMainDisjoint\":false,\"interpretation\":\"shadow-vs-non-shadow-candidate\"}"
    ));
    assert!(layer_tree.contains(
        "{\"recordType\":112,\"recordTypeHex\":\"0x70\",\"wordIndex\":0,\"shadowValues\":[\"0x2c\"],\"textureValues\":[\"0x1c\"],\"mainValues\":[\"0x1c\"],\"presentInAllRoles\":true,\"sharedAcrossAllRoles\":false,\"shadowDisjointFromTextureAndMain\":true,\"textureDisjointFromShadowAndMain\":false,\"mainDisjointFromShadowAndTexture\":false,\"textureMainDisjoint\":false,\"shadowTextureSharedMainDisjoint\":false,\"interpretation\":\"shadow-vs-non-shadow-candidate\"}"
    ));
    assert!(layer_tree.contains(
        "\"groups\":[{\"role\":\"shadowOutlines\",\"pathKind\":\"outline\",\"pathCount\":11,\"explicitStatePathCount\":11,\"inheritedStatePathCount\":0,\"record46Word0Values\":[\"0x00\",\"0x01\"],\"record48Word0Values\":[\"0x01\"]"
    ));
    assert!(layer_tree.contains(
        "{\"role\":\"interstitialTextureBlock\",\"pathKind\":\"texture\",\"pathCount\":530,\"explicitStatePathCount\":11,\"inheritedStatePathCount\":519,\"record46Word0Values\":[\"0x00\"],\"record48Word0Values\":[\"0x00\"]"
    ));
    assert!(layer_tree.contains(
        "{\"role\":\"mainOutlines\",\"pathKind\":\"outline\",\"pathCount\":11,\"explicitStatePathCount\":11,\"inheritedStatePathCount\":0,\"record46Word0Values\":[\"0x00\",\"0x01\"],\"record48Word0Values\":[\"0x00\"]"
    ));
    assert!(layer_tree.contains("\"extrusionTexturePathCount\":530"));
    assert!(layer_tree.contains(
        "\"extrusionTextureClipGate\":{\"source\":\"embeddedPressOutlineTextureOutlineClipArbitration\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":true,\"diagnosticOnly\":true,\"renderPromoted\":true,\"texturePathCount\":530,\"selectedClipSource\":\"source-shadow-outline\",\"selectedClipBasis\":\"current-renderer-shadow-outline-clip\""
    ));
    assert!(layer_tree.contains(
        "\"alternativeClipSource\":\"long-shadow-side-sweep\",\"alternativeRejected\":true,\"alternativeRejectedBy\":\"historical-poppler-crop-ab\",\"alternativeRejectedReason\":\"long-shadow-side-sweep-texture-clip-worsened-title-crops\",\"frontFaceKnockoutDecoded\":false,\"clipSemanticsDecoded\":false,\"remainingBlockedReason\":\"texture-clip-and-knockout-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"titleTextureGeometryRoleGate\":{\"source\":\"embeddedPressSourceBboxRoleComparison\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"diagnosticOnly\":true,\"renderPromoted\":false,\"partitionPresent\":true,\"texturePathCount\":530,\"shadowOutlinePathCount\":11,\"mainOutlinePathCount\":11"
    ));
    assert!(layer_tree.contains(
        "\"textureBbox\":{\"left\":272,\"top\":202,\"right\":13032,\"bottom\":1311},\"shadowOutlineBbox\":{\"left\":372,\"top\":302,\"right\":13032,\"bottom\":1311},\"mainOutlineBbox\":{\"left\":272,\"top\":202,\"right\":12932,\"bottom\":1211},\"sideSweepBbox\":{\"left\":272,\"top\":202,\"right\":13032,\"bottom\":1311}"
    ));
    assert!(layer_tree.contains(
        "\"textureArea\":14150840,\"textureMainOverlapArea\":12773940,\"textureShadowOverlapArea\":12773940,\"textureSideSweepOverlapArea\":14150840,\"textureMainOverlapRatio\":0.903,\"textureShadowOverlapRatio\":0.903,\"textureSideSweepOverlapRatio\":1.000"
    ));
    assert!(layer_tree.contains(
        "\"textureContainedByMainBbox\":false,\"textureContainedByShadowBbox\":false,\"textureContainedBySideSweepBbox\":true,\"frontFaceKnockoutDecoded\":false,\"clipSemanticsDecoded\":false,\"roleConclusion\":\"texture-bbox-overlaps-main-and-shadow-outline-bboxes\",\"renderPromotionBlockedReason\":\"texture-source-bbox-relation-is-bbox-only-not-knockout-proof\""
    ));
    assert!(layer_tree.contains("\"extrusionTextureRecord70Word0Values\":[\"0x1c\"]"));
    assert!(layer_tree.contains("\"extrusionTextureEffectCandidateFillColor\":\"#676767\""));
    assert!(layer_tree.contains("\"extrusionTextureEffectCandidateOpacity\":0.280"));
    assert!(layer_tree.contains("\"extrusionTextureEffectCandidateWord0\":28"));
    assert!(layer_tree.contains(
        "\"extrusionTextureEffectCandidateSource\":\"embedded-press-interstitial-0x70-word0-percent-black-over-shadow\""
    ));
    assert!(layer_tree.contains(
        "\"extrusionTextureEffectRenderPromoted\":false,\"extrusionTextureEffectRenderPromotionBlockedReason\":\"record70-separates-shadow-but-not-interstitial-texture-from-main\""
    ));
    assert!(layer_tree.contains("\"shadowTexturePathCount\":11"));
    assert!(layer_tree.contains("\"shadowTextureWord5Values\":[\"0x2f\"]"));
    assert!(layer_tree.contains("\"shadowEffectFillColor\":\"#8f8f8f\""));
    assert!(layer_tree.contains("\"shadowEffectOpacity\":0.440"));
    assert!(layer_tree.contains("\"shadowEffectWord0\":44"));
    assert!(
        layer_tree.contains(
            "\"shadowEffectSource\":\"embedded-press-0x70-word0-percent-black-on-white\""
        )
    );
    assert!(layer_tree.contains("\"effectiveShadowTexturePathCount\":530"));
    assert!(layer_tree.contains("\"effectiveFrontTexturePathCount\":0"));
    assert!(layer_tree.contains("\"effectiveTextureWord5Values\":[\"0x2f\"]"));
    assert!(layer_tree.contains("\"textureStateInheritance\":\"embeddedPressCurrentPaintState\""));
    assert!(layer_tree.contains("\"paintStateSummaries\":["));
    assert!(layer_tree.contains("\"role\":\"shadowOutlines\""));
    assert!(layer_tree.contains("\"role\":\"interstitialTextureBlock\""));
    assert!(layer_tree.contains("\"role\":\"mainOutlines\""));
    assert!(layer_tree.contains("\"firstPathIndex\":11,\"lastPathIndex\":540"));
    assert!(layer_tree.contains("\"explicitStatePathCount\":11,\"inheritedStatePathCount\":519"));
    assert!(layer_tree.contains("\"statePayloadWordColumns\":["));
    assert!(
        layer_tree
            .contains("\"recordTypeHex\":\"0x70\",\"recordCount\":11,\"uniquePayloadCount\":1")
    );
    assert!(layer_tree.contains("\"words\":[\"0x2c\",\"0x11\",\"0x00\",\"0x0a\""));
    assert!(layer_tree.contains(
        "\"recordTypeHex\":\"0x48\",\"recordCount\":11,\"maxWordCount\":1,\"columns\":[{\"wordIndex\":0,\"values\":[\"0x01\"]}]"
    ));
    assert!(layer_tree.contains(
        "\"recordTypeHex\":\"0x48\",\"recordCount\":11,\"maxWordCount\":1,\"columns\":[{\"wordIndex\":0,\"values\":[\"0x00\"]}]"
    ));
    assert!(layer_tree.contains(
        "\"recordTypeHex\":\"0x70\",\"recordCount\":11,\"maxWordCount\":13,\"columns\":[{\"wordIndex\":0,\"values\":[\"0x2c\"]"
    ));
    assert!(layer_tree.contains("{\"wordIndex\":3,\"values\":[\"0x0a\"]}"));
    assert!(layer_tree.contains("\"words\":[\"0x1c\",\"0x11\",\"0x00\",\"0x00\""));
    assert!(layer_tree.contains("{\"wordIndex\":0,\"values\":[\"0x1c\"]}"));
    assert!(layer_tree.contains("\"words\":[\"0x1c\",\"0x10\",\"0x00\",\"0xffffff\""));
    assert!(layer_tree.contains("\"paintStateColor\":\"#ffffff\""));
    assert!(layer_tree.contains(
        "\"frontPaintCandidate\":{\"source\":\"JSFart2Contents+EmbeddedPressPaintState\",\"decoded\":false,\"sourceBacked\":true,\"paintColor\":\"#ffffff\",\"paintColorSource\":\"JSFart2Contents.paintColorCandidate\",\"renderFillColor\":\"#111111\",\"renderFillColorSource\":\"conservative-front-fill-fallback-source-paint-mismatch\",\"renderFillColorSourceBacked\":false,\"sourcePaintColorMatchesRenderFill\":false,\"renderFillColorPromotionBlockedReason\":\"source-paint-color-does-not-match-render-fill\""
    ));
    assert!(layer_tree.contains(
        "\"frontPaintArbitrationGate\":{\"source\":\"JSFart2Contents+EmbeddedPressPaintState+frontEraseTextureProbes\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"diagnosticOnly\":true,\"renderPromoted\":false,\"candidateCount\":4,\"selectedRenderPolicy\":\"conservative-front-fill\",\"selectedRenderFillColor\":\"#111111\""
    ));
    assert!(layer_tree.contains(
        "\"sourcePaintCandidatePresent\":true,\"sourcePaintCandidateMatchesRenderFill\":false,\"directGrayCandidatePresent\":true,\"textureSourcePaintCandidatePresent\":true,\"frontEraseTextureSpanCandidatePresent\":true,\"frontEraseTextureSpanCount\":11"
    ));
    assert!(layer_tree.contains(
        "\"frontEraseTransitionBoundaryClass\":\"source-order-bracketed-interstitial-texture-block\",\"frontErasePaintIntentInference\":\"shadow-state-texture-inside-main-boundary-ambiguous\""
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReasons\":[\"source-paint-color-does-not-match-render-fill\",\"front-erase-texture-over-main-face-semantics-unproven\",\"front-erase-transition-boundary-main-state-not-separated\"],\"renderPromotionBlockedReason\":\"front-paint-candidate-arbitration-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"frontFillWindingGate\":{\"source\":\"embeddedPressContourWinding+popplerTitleCropAB\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":true,\"diagnosticOnly\":false,\"renderPromoted\":true,\"pathCount\":11,\"multiContourPathCount\":11,\"oppositeSignedContourPathCount\":4"
    ));
    assert!(layer_tree.contains(
        "\"selectedFillRule\":\"evenodd\",\"selectedFillRuleSource\":\"embedded-press-evenodd-boundary-contours\",\"previousFillRule\":\"nonzero\",\"rejectedFillRule\":\"nonzero\",\"rejectedBy\":\"poppler-title-tight-ab\""
    ));
    assert!(layer_tree.contains(
        "\"nonzeroTitleTightRms\":78.059,\"evenoddTitleTightRms\":76.034,\"rmsImprovement\":2.025,\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"mainStateTexturePathCount\":0,\"frontEraseTexturePathCount\":530,\"renderTexturePathSource\":\"source-order-interstitial-front-erase-texture\",\"renderPathCount\":530,\"visibleRenderPathCount\":0,\"renderClipRule\":\"nonzero\",\"renderClipRuleSource\":\"embedded-press-nonzero-winding\",\"renderClipRulePixelChange\":true,\"renderPromotionBlockedReason\":\"front-erase-texture-over-main-face-semantics-unproven\""
    ));
    assert!(layer_tree.contains("\"sourceFrameCandidate\":{\"source\":\"JSFart2Contents\""));
    assert!(layer_tree.contains("\"contentLeft\":114,\"contentTop\":105"));
    assert!(layer_tree.contains("\"cornerRadiusX\":114"));
    assert!(layer_tree.contains("\"cornerRadiusY\":105"));
    assert!(layer_tree.contains("\"sourcePaintCandidate\":{\"styleWord1\":34869296"));
    assert!(layer_tree.contains("\"styleWord1Hex\":\"0x02141030\""));
    assert!(layer_tree.contains("\"styleWord2Hex\":\"0x02141018\""));
    assert!(layer_tree.contains("\"paintColorCandidateHex\":\"0x00ffffff\""));
    assert!(layer_tree.contains("\"effectWordCandidateHex\":\"0x0000000a\""));
    assert!(layer_tree.contains("\"renderable\":true"));
    assert!(layer_tree.contains("\"type\":\"tableGridCandidate\""));
    assert!(layer_tree.contains("\"projectionKind\":\"tableProjection\""));
    assert!(layer_tree.contains("\"referenceBacked\":false"));
    assert!(layer_tree.contains("\"rowCount\":3"));
    assert!(layer_tree.contains("\"colCountCandidate\":5"));
    assert!(layer_tree.contains("\"columnWidthBasis\":\"documentTextLineHeaderCellSlotUnits\""));
    assert!(layer_tree.contains("\"columnWidths\":[49.370,56.423,56.423,56.423,56.423]"));
    assert!(layer_tree.contains("\"cellCountCandidate\":15"));
    assert!(layer_tree.contains("\"emptyCellCountCandidate\":4"));
    assert!(layer_tree.contains("\"bbox\":{\"x\":78.251,\"y\":411.421"));
    assert!(layer_tree.contains(
        "\"sourceAnchorEvidence\":{\"source\":\"tableCandidateColumnSegments\",\"basis\":\"unit\",\"cellSourceRangeCount\":11"
    ));
    assert!(
        layer_tree.contains("\"geometryDerivationEvidence\":{\"source\":\"tableCandidateGeometryProbe\",\"candidateBasis\":\"unit\"")
    );
    assert!(layer_tree.contains("\"fallbackTextRunAnchorCount\":11"));
    assert!(layer_tree.contains("\"textCountRangeCount\":0"));
    assert!(layer_tree.contains("\"layoutBoxPresent\":false"));
    assert!(layer_tree.contains("\"sourceLayoutEvidencePresent\":true"));
    assert!(layer_tree.contains("\"placementAuthority\":\"documentTextLineHeaders\""));
    assert!(layer_tree.contains("\"decodedSourcePlacementEvidence\":true"));
    assert!(layer_tree.contains("\"decodedSourcePlacementMatchCount\":15"));
    assert!(layer_tree.contains("\"decodedSourcePlacementRequiredCellCount\":15"));
    assert!(layer_tree.contains(
        "\"documentTextLineHeaderEvidence\":{\"source\":\"/DocumentText\",\"present\":true"
    ));
    assert!(layer_tree.contains("\"rawHeaderCount\":21"));
    assert!(layer_tree.contains("\"matchedCellHeaderCount\":15"));
    assert!(layer_tree.contains(
        "\"rawWords\":[28,48,12,0,0,10,255,0,12,0,48,31],\"rawWordsHex\":[\"0x001c\",\"0x0030\",\"0x000c\",\"0x0000\",\"0x0000\",\"0x000a\",\"0x00ff\",\"0x0000\",\"0x000c\",\"0x0000\",\"0x0030\",\"0x001f\"]"
    ));
    assert!(layer_tree.contains(
        "\"unitGeometryCandidate\":{\"source\":\"documentTextLineHeaders\",\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"page-space-origin-and-unit-scale-unproven\""
    ));
    assert!(layer_tree.contains("\"rowsHomogeneous\":true"));
    assert!(layer_tree.contains("\"matchedColumnCount\":5"));
    assert!(layer_tree.contains("\"rawHeaderCountPerRow\":7"));
    assert!(layer_tree.contains("\"minOffsetUnits\":0"));
    assert!(layer_tree.contains("\"maxExtentUnits\":174"));
    assert!(layer_tree.contains("\"homogeneousFontSizeUnits\":12"));
    assert!(layer_tree.contains("\"matchedCellOffsetUnits\":[0,14,28,44,60]"));
    assert!(layer_tree.contains("\"matchedCellExtentUnits\":[10,24,40,56,72]"));
    assert!(layer_tree.contains("\"trailingHeaderOffsetUnits\":[76,92]"));
    assert!(layer_tree.contains("\"trailingHeaderExtentUnits\":[88,174]"));
    assert!(layer_tree.contains(
        "\"tableUnitBBoxCandidates\":[{\"source\":\"documentTextLineHeaders\",\"basis\":\"matched-cells\""
    ));
    assert!(layer_tree.contains(
        "\"xUnitRange\":{\"start\":0,\"end\":72},\"widthUnits\":72,\"rowAgreementCount\":3,\"allRowsAgree\":true,\"trailingHeaderIncluded\":false"
    ));
    assert!(layer_tree.contains(
        "\"includedTrailingHeaderCount\":0,\"columnSpanUnits\":[10,10,12,12,12],\"columnSlotWidthUnits\":[14,14,16,16,12],\"trailingSlotWidthUnits\":[]"
    ));
    assert!(
        layer_tree.contains(
            "\"basis\":\"matched-cells-plus-first-trailing-header\",\"sourceBacked\":true"
        )
    );
    assert!(layer_tree.contains(
        "\"xUnitRange\":{\"start\":0,\"end\":88},\"widthUnits\":88,\"rowAgreementCount\":3,\"allRowsAgree\":true,\"trailingHeaderIncluded\":true"
    ));
    assert!(layer_tree.contains(
        "\"includedTrailingHeaderCount\":1,\"columnSpanUnits\":[10,10,12,12,12],\"columnSlotWidthUnits\":[14,14,16,16,16],\"trailingSlotWidthUnits\":[12]"
    ));
    assert!(layer_tree.contains("\"basis\":\"full-line-header-extent\",\"sourceBacked\":true"));
    assert!(layer_tree.contains(
        "\"xUnitRange\":{\"start\":0,\"end\":174},\"widthUnits\":174,\"rowAgreementCount\":3,\"allRowsAgree\":true,\"trailingHeaderIncluded\":true"
    ));
    assert!(layer_tree.contains(
        "\"includedTrailingHeaderCount\":2,\"columnSpanUnits\":[10,10,12,12,12],\"columnSlotWidthUnits\":[14,14,16,16,16],\"trailingSlotWidthUnits\":[16,82]"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"table-horizontal-unit-span-candidate-only\",\"renderPromotionBlockedReason\":\"page-space-unit-scale-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"lineHeaderLineMarkCouplingEvidence\":{\"source\":\"/DocumentText+/LineMark\",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"lineHeaderRowCount\":3,\"lineMarkIntervalCount\":78,\"coupledRowCount\":3,\"exactSourceRangeMatchCount\":3,\"lineHeaderRecordContainmentCount\":3"
    ));
    assert!(layer_tree.contains(
        "\"allRowsCoupled\":true,\"allRowsExactSourceRangeMatched\":true,\"contiguousLineMarkRecords\":true"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[16,17,18],\"uniformLineMarkRecordStride\":true,\"lineMarkRecordStride\":1,\"interleavedLineMarkRecordCountBetweenRows\":0"
    ));
    assert!(layer_tree.contains("\"lineHeaderRowsHomogeneous\":true"));
    assert!(layer_tree.contains("\"lineMarkRecordRange\":{\"start\":16,\"end\":19}"));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"row-boundary-line-header-coupling-evidence-only\",\"renderPromotionBlockedReason\":\"line-mark-units-not-page-y-coordinate-transform\""
    ));
    assert!(layer_tree.contains(
        "\"rowSourceUnitRange\":{\"start\":519,\"end\":655},\"lineHeaderCount\":7,\"matchedCellHeaderCount\":5,\"lineMarkRecordIndex\":16,\"lineMarkUnitRange\":{\"start\":519,\"end\":655},\"exactSourceRangeMatch\":true,\"lineHeaderRecordsContained\":true"
    ));
    assert!(layer_tree.contains("\"lineHeaderRecordUnitRanges\":[{\"start\":558,\"end\":570}"));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidate\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"projectionKind\":\"sourceDerivedDiagnosticProjection\",\"bbox\":{\"x\":78.251,\"y\":411.421,\"width\":275.063,\"height\":63.000}"
    ));
    assert!(layer_tree.contains("\"columnCount\":5,\"rowCount\":3"));
    assert!(layer_tree.contains("\"columnWidths\":[49.370,56.423,56.423,56.423,56.423]"));
    assert!(layer_tree.contains(
        "\"xUnitRangeBasis\":\"matched-cells-plus-first-trailing-header\",\"xUnitRange\":{\"start\":0,\"end\":88}"
    ));
    assert!(layer_tree.contains(
        "\"xOriginInsetUnits\":2.000,\"xOriginInsetBasis\":\"uniform-intercell-gap-half\""
    ));
    assert!(layer_tree.contains(
        "\"horizontalUnitTransformReadiness\":{\"source\":\"documentTextLineHeaders\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"selectedXUnitRangeBasis\":\"matched-cells-plus-first-trailing-header\",\"selectedXUnitRange\":{\"start\":0,\"end\":88},\"selectedWidthUnits\":88,\"fullExtentUnits\":174,\"selectedWidthRatioToFullExtent\":0.506"
    ));
    assert!(layer_tree.contains(
        "\"rowAgreementCount\":3,\"allRowsAgree\":true,\"trailingHeaderIncluded\":true,\"includedTrailingHeaderCount\":1"
    ));
    assert!(layer_tree.contains(
        "\"columnSpanUnits\":[10,10,12,12,12],\"columnSlotWidthUnits\":[14,14,16,16,16],\"trailingSlotWidthUnits\":[12],\"xOriginInsetUnits\":2.000,\"xOriginInsetBasis\":\"uniform-intercell-gap-half\""
    ));
    assert!(layer_tree.contains(
        "\"totalWidthSemanticsGate\":{\"source\":\"documentTextLineHeaders total-width semantics gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"selectedWidthUnits\":88,\"fullExtentUnits\":174,\"fullExtentTrailingUnits\":86,\"selectedEqualsFullExtent\":false,\"selectedIsSubsetOfFullExtent\":true,\"trailingHeaderIncluded\":true,\"includedTrailingHeaderCount\":1,\"trailingSlotEvidencePresent\":true,\"trailingSlotWidthUnits\":[12],\"selectedVisibleRangeSourceEvidenceReady\":true,\"sourcePlacementCoherenceGateRequired\":false,\"sourcePlacementCoherenceGateEvidencePresent\":true,\"sourcePlacementCoherenceGateResolved\":true,\"sourcePlacementCoherenceGateBlockedReasons\":[],\"renderPromotionNextGate\":null,\"renderWidthBasisCandidate\":\"selected-visible-range-with-trailing-header-evidence\",\"renderPromotionContribution\":\"source-total-width-semantics-gate\",\"renderPromotionBlockedReason\":null}"
    ));
    assert!(
        layer_tree
            .contains("\"sourceOnlyUnitTransformReady\":true,\"pageSpaceUnitScaleDecoded\":true")
    );
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"selected-table-horizontal-unit-transform-readiness\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"rowHeight\":21.000,\"rowHeightBasis\":\"documentTextLineHeaderFontSizeUnits\",\"pageOriginAuthority\":\"lineMarkPageGrid\",\"anchorLineIndex\":15"
    ));
    assert!(layer_tree.contains("\"lineMarkRowsExactAndContiguous\":true"));
    assert!(layer_tree.contains("\"lineHeaderRowsHomogeneous\":true"));
    assert!(layer_tree.contains("\"renderPromotionBlockedReason\":\"none\""));
    assert!(layer_tree.contains(
        "\"pageSpaceSolver\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\",\"solverVersion\":\"table-page-space-v1\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"solverStage\":\"renderable-source-page-space\",\"sourcePlacementEvidencePresent\":true,\"candidateRowCount\":3,\"requestedColumnCount\":5,\"commonMatchedColumnCount\":5,\"matchedCellHeaderCount\":15,\"requiredCellHeaderCount\":15"
    ));
    assert!(layer_tree.contains(
        "\"horizontalSolverReady\":true,\"rowHeightSolverReady\":true,\"yOriginSolverReady\":true,\"lineHeaderRowsHomogeneous\":true,\"lineMarkRowRecordSelection\":\"selected-overlap-record\",\"lineMarkRowsExactAndContiguous\":true"
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidatePresent\":true,\"sourceDerivedLayoutRenderable\":true,\"pageOriginAuthority\":\"lineMarkPageGrid\",\"lineMarkPageOriginPresent\":true,\"lineMarkPageOriginStridePresent\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyAxisAdmissionGate\":{\"source\":\"pageSpaceHorizontalTransformGate+sourcePageYTransformGate source-only selector coupling\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"admissionReady\":true,\"activeSourceLayoutAdmissionReady\":true,\"activeSourceLayoutAdmissionBasis\":\"source-derived-page-space-solver\",\"sourceOnlySelectorFallbackIgnoredByActiveSourceLayout\":true"
    ));
    assert!(layer_tree.contains(
        "\"horizontalSelectorCandidatePresent\":false,\"horizontalSelectorInBestAgreementGroup\":false,\"horizontalCandidateCount\":5,\"horizontalAgreementGroupCount\":4,\"horizontalBestSupportCount\":2,\"horizontalUniqueBestSupported\":true"
    ));
    assert!(layer_tree.contains(
        "\"ySelectorSingleSupportFallback\":true,\"ySelectorSupportFragmentedByTable\":false,\"ySelectorSupportCount\":1,\"ySelectorCrossTableSupportPresent\":false,\"ySelectorAgreementAdmissible\":false,\"ySelectorAdmissionBlockedReason\":\"none\""
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[],\"renderPromotionContribution\":\"source-only-axis-selector-admission-gate\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"sourceFrameAdmissionGate\":{\"source\":\"sourceDerivedLayoutCandidate+sourceFrameAdmission\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":true,\"sourceFrameDecoded\":true,\"renderFrameBasis\":\"page-body-frame+documentTextLineHeaderUnitTransform\",\"selectedX\":78.251,\"selectedWidth\":275.063"
    ));
    assert!(layer_tree.contains(
        "\"pageBodyFrameX\":72.000,\"pageBodyFrameWidth\":543.874,\"pageBodyUnitPx\":3.126,\"selectedXWithoutInset\":72.000,\"selectedXWithInset\":78.251"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkRawAgreementPresent\":true,\"pageMarkRawAgreementSelectedX\":171.000,\"pageMarkRawAgreementSelectedWidth\":548.000,\"pageMarkRawAgreementSupportCount\":2,\"pageMarkRawAgreementFrameBases\":[\"page-mark-word14-word21-first-slot-adjusted\",\"page-mark-word14-first-slot-word21-half-slot\"]"
    ));
    assert!(layer_tree.contains(
        "\"sourceFrameVsPageMarkAgreementXResidualPx\":-92.749,\"sourceFrameVsPageMarkAgreementWidthResidualPx\":-272.937,\"pageMarkRawAgreementConflictsWithRenderFrame\":true,\"pageMarkRawAgreementRenderPromotionBlockedReason\":\"page-mark-horizontal-field-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"sourceTopTextPlacementCoherenceMirror\":{\"source\":\"topTextTableSourceGapEvidence.sourceTablePlacementCoherenceGate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"sourceTopTextPlacementEvidencePresent\":true,\"sourceTopTextPlacementReady\":true,\"readinessBlockedReasons\":[],\"renderPromotionContribution\":\"source-horizontal-frame-top-text-placement-coherence\",\"renderPromotionBlockedReason\":null}"
    ));
    assert!(layer_tree.contains(
        "\"sourcePageYTransformGate\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark y-origin promotion gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":true,\"lineMarkRowsExactAndContiguous\":true,\"pageOriginAuthority\":\"lineMarkPageGrid\",\"lineMarkPageOriginPresent\":true,\"lineMarkPageOriginStridePresent\":false,\"subrecordLineSpanReadinessPresent\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYRenderAdmissionGate\":{\"source\":\"sourcePageYTransformGate source-only page-y render admission gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"admissionReady\":true,\"directLineMarkOriginAdmissible\":true,\"sourceLayoutCandidatePresent\":true,\"pageOriginAuthority\":\"lineMarkPageGrid\",\"lineMarkRowsExactAndContiguous\":true,\"lineMarkPageOriginPresent\":true,\"lineMarkPageOriginStridePresent\":false"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkAbsoluteYSlotBlockedReason\":\"page-mark-absolute-y-slot-absent\",\"pageMarkAbsoluteYSlotResidualPx\":null,\"blockedReasons\":[],\"renderPromotionContribution\":\"source-only-page-y-render-admission-gate\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"referenceFallbackAdmissionGate\":{\"source\":\"table_grid_reference_layout_visible_fallback_allowed+sourceOnlyPageYRenderAdmissionGate\",\"diagnosticOnly\":true"
    ));
    assert!(layer_tree.contains(
        "\"referenceLayoutPresent\":false,\"referenceFallbackAllowed\":false,\"referenceFallbackUsed\":false,\"sourceLayoutCandidatePresent\":true,\"sourceRenderLayoutPresent\":true,\"sourceLayoutRenderable\":true,\"sourceOnlyPageYAdmissionReady\":true,\"sourceOnlyPageYAdmissionBasis\":\"line-mark-page-grid-direct-origin\",\"sourceReplacementBlockedReason\":null,\"blockedReason\":\"active-source-layout-admission-suppresses-reference-fallback\""
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[],\"renderPromotionContribution\":\"source-page-y-transform-gate\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"renderPromoted\":true,\"renderPromotionAuthority\":\"source-derived-page-space-solver\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkPageOriginCandidate\":{\"source\":\"/LineMark+/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":true,\"y\":411.421,\"firstLineMarkRecordIndex\":16,\"lastLineMarkRecordIndex\":18"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":39,\"pageMarkU16Fields\":[0,0,1,0,0,0,0,39,0,0,370,0"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkU16FieldsHex\":[\"0x0000\",\"0x0000\",\"0x0001\",\"0x0000\",\"0x0000\",\"0x0000\",\"0x0000\",\"0x0027\""
    ));
    assert!(layer_tree.contains(
        "\"pageMarkU16GeometryHypotheses\":{\"source\":\"/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"selectedFields\":[{\"wordIndex\":10,\"value\":370,\"hex\":\"0x0172\"},{\"wordIndex\":13,\"value\":370,\"hex\":\"0x0172\"},{\"wordIndex\":14,\"value\":185,\"hex\":\"0x00b9\""
    ));
    assert!(layer_tree.contains(
        "\"word20Is0x00ff\":true,\"word13PlusWord14\":555,\"word13PlusWord14EqualsWord21\":true"
    ));
    assert!(layer_tree.contains(
        "\"word21MinusWord13\":185,\"word21MinusWord13EqualsWord14\":true,\"word19EqualsWord13\":true,\"selectedFieldsAllZero\":false,\"nonZeroAdditiveUnitCandidate\":true"
    ));
    assert!(layer_tree.contains("\"pageWidthPxPerWord21Unit\":1.239"));
    assert!(layer_tree.contains("\"pageHeightPxPerWord21Unit\":1.750"));
    assert!(layer_tree.contains("\"bodyWidthPxPerWord21Unit\":0.980"));
    assert!(layer_tree.contains("\"bodyWidthPxPerWord13Unit\":1.470"));
    assert!(layer_tree.contains("\"marginPxPerWord14Unit\":0.389"));
    assert!(layer_tree.contains(
        "\"pageHeightPxPerWord13Plus14Unit\":1.750,\"bodyWidthPxPerWord13Plus14Unit\":0.980"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"page-mark-u16-horizontal-geometry-candidate-only\",\"renderPromotionBlockedReason\":\"page-mark-u16-geometry-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"lineOffsetFromPageStart\":16,\"linePitchPx\":21.214,\"linePitchBasis\":\"pageMarkBodyLineGap\",\"rowHeight\":21.000,\"renderPromotionContribution\":\"source-backed-page-y-origin\",\"renderPromotionBlockedReason\":null}"
    ));
    assert!(layer_tree.contains(
        "\"linePitchAgreementGate\":{\"source\":\"/PageMark body line-gap pitch+source row height\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"rowHeightCandidatePresent\":true,\"rowHeightPx\":21.000,\"rowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\""
    ));
    assert!(layer_tree.contains(
        "\"rowHeightResidualPx\":-0.214,\"absRowHeightResidualPx\":0.214,\"tolerancePx\":0.500,\"pageMarkU16GeometryClass\":\"additive-boundary\",\"pitchAgreementReady\":true"
    ));
    assert!(layer_tree.contains("\"renderPromotionBlockedReason\":\"none\""));
    assert!(layer_tree.contains(
        "\"referenceBBoxResidualEvidence\":{\"source\":\"documentTextLineHeaders+referenceTableBBox\",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"reference-bbox-uses-source-column-widths-but-not-source-placement\""
    ));
    assert!(layer_tree.contains("\"matchedCellSpanUnits\":[10,10,12,12,12]"));
    assert!(layer_tree.contains("\"matchedCellGapUnits\":[4,4,4,4]"));
    assert!(layer_tree.contains("\"tailGapUnits\":4"));
    assert!(layer_tree.contains("\"tailSpanUnits\":102"));
    assert!(layer_tree.contains("\"referenceColumnWidthPx\":55.198"));
    assert!(
        layer_tree
            .contains("\"referenceColumnWidthBasis\":\"documentTextLineHeaderCellSlotUnits\"")
    );
    assert!(
        layer_tree.contains("\"referenceColumnWidthsPx\":[49.537,56.613,56.613,56.613,56.613]")
    );
    assert!(layer_tree.contains("\"referenceTableWidthPx\":275.990"));
    assert!(layer_tree.contains("\"referenceWidthPxPerFullExtentUnit\":1.586"));
    assert!(layer_tree.contains(
        "\"unitBBoxCandidateComparisons\":[{\"basis\":\"matched-cells\",\"xUnitRange\":{\"start\":0,\"end\":72},\"widthUnits\":72,\"referenceWidthPxPerUnit\":3.833,\"widthRatioToFullLineExtent\":0.414"
    ));
    assert!(layer_tree.contains(
        "{\"basis\":\"matched-cells-plus-first-trailing-header\",\"xUnitRange\":{\"start\":0,\"end\":88},\"widthUnits\":88,\"referenceWidthPxPerUnit\":3.136,\"widthRatioToFullLineExtent\":0.506"
    ));
    assert!(layer_tree.contains(
        "{\"basis\":\"full-line-header-extent\",\"xUnitRange\":{\"start\":0,\"end\":174},\"widthUnits\":174,\"referenceWidthPxPerUnit\":1.586,\"widthRatioToFullLineExtent\":1.000"
    ));
    assert!(layer_tree.contains(
        "\"referenceVerticalComparison\":{\"source\":\"/DocumentText+/LineMark+referenceTableBBox\",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceRowHeightBasis\":\"documentTextLineHeaderFontSizeUnits\",\"homogeneousFontSizeUnits\":12,\"rowCount\":3,\"lineMarkRecordSpan\":3"
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedRowHeightPx\":21.000,\"sourceDerivedTableHeightPx\":63.000,\"referenceRowHeightPx\":21.001,\"referenceTableHeightPx\":63.003"
    ));
    assert!(layer_tree.contains(
        "\"rowHeightResidualPx\":0.001,\"tableHeightResidualPx\":0.003,\"renderPromotionContribution\":\"row-height-corroboration-evidence-only\",\"renderPromotionBlockedReason\":\"page-space-y-origin-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedHorizontalComparison\":{\"source\":\"sourceDerivedLayoutCandidate+referenceTableBBox\",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceBBox\":{\"x\":78.251,\"y\":411.421,\"width\":275.063,\"height\":63.000},\"referenceBBox\":{\"x\":79.297,\"y\":410.716,\"width\":275.990,\"height\":63.003}"
    ));
    assert!(layer_tree.contains(
        "\"xResidualPx\":1.046,\"widthResidualPx\":0.927,\"rightResidualPx\":1.972,\"widthResidualAbsPx\":0.927,\"xResidualAbsPx\":1.046"
    ));
    assert!(layer_tree.contains(
        "\"widthAgreementStrong\":true,\"xOriginAgreementStrong\":true,\"originResidualEvidence\":{\"source\":\"sourceDerivedLayoutCandidate+referenceTableBBox+rawLayoutFields\""
    ));
    assert!(layer_tree.contains(
        "\"xResidualInTableUnits\":0.335,\"xResidualInFullExtentUnits\":0.661,\"yResidualInRows\":-0.034"
    ));
    assert!(layer_tree.contains(
        "\"sourceFields\":{\"xUnitRangeBasis\":\"matched-cells-plus-first-trailing-header\",\"xUnitRange\":{\"start\":0,\"end\":88},\"tableSpanUnits\":88,\"fullExtentUnits\":174,\"homogeneousFontSizeUnits\":12"
    ));
    assert!(layer_tree.contains("\"firstLineHeaderRawWords\":[28,48,12,0,0,10,255,0,12,0,48,31]"));
    assert!(layer_tree.contains(
        "\"pageMarkSelectedFields\":[{\"wordIndex\":10,\"value\":370,\"hex\":\"0x0172\"},{\"wordIndex\":13,\"value\":370,\"hex\":\"0x0172\"},{\"wordIndex\":14,\"value\":185,\"hex\":\"0x00b9\"}"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"origin-residual-targeted-source-field-comparison\",\"renderPromotionBlockedReason\":\"origin-residual-not-explained-by-decoded-source-field\""
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"source-width-corroboration-evidence-only\",\"renderPromotionBlockedReason\":\"horizontal-x-origin-unproven\""
    ));
    assert!(
        layer_tree.contains("\"referenceColumnPxPerMatchedUnit\":[4.954,5.661,4.718,4.718,4.718]")
    );
    assert!(layer_tree.contains("\"equalReferenceColumnsConflictWithUnitSpans\":false"));
    assert!(layer_tree.contains(
        "\"topTextAnchorEvidence\":{\"source\":\"/DocumentText\",\"present\":true,\"referenceBackedCoordinateCount\":21,\"sourceBackedRunCount\":21,\"lineHeaderBackedRunCount\":2,\"lineMarkBackedRunCount\":21,\"directTableTransformDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"top-text-anchors-reference-backed-not-independent-page-transform\""
    ));
    assert!(layer_tree.contains(
        "\"referenceCoordinateProbe\":{\"source\":\"referenceCoordinates+documentTextLineHeaders\",\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(
        layer_tree
            .contains("\"renderPromotionBlockedReason\":\"probe-uses-reference-coordinates\"")
    );
    assert!(layer_tree.contains(
        "\"basis\":\"table-reference-width-divided-by-max-line-header-extent\",\"tableMaxExtentUnits\":174,\"tableUnitWidthPx\":1.586,\"anchorCount\":2,\"maxAbsXResidualPx\":289.036,\"consistentWithSingleTableTransform\":false"
    ));
    assert!(layer_tree.contains("\"projectedXFromTableLeft\":326.736,\"xResidualPx\":-289.036"));
    assert!(layer_tree.contains("\"projectedXFromTableLeft\":79.297,\"xResidualPx\":-13.397"));
    assert!(layer_tree.contains(
        "\"lineHeaderAnchors\":[{\"text\":\"１，次の計算をしなさい\",\"sourceUnitRange\":{\"start\":264,\"end\":275},\"lineHeaderUnitRange\":{\"start\":251,\"end\":263},\"offsetUnits\":156,\"extentUnits\":174,\"fontSizeUnits\":12}"
    ));
    assert!(layer_tree.contains(
        "{\"text\":\"空欄を埋めて表を完成させなさい。\",\"sourceUnitRange\":{\"start\":501,\"end\":517},\"lineHeaderUnitRange\":{\"start\":487,\"end\":499},\"offsetUnits\":0,\"extentUnits\":174,\"fontSizeUnits\":12}"
    ));
    assert!(layer_tree.contains(
        "\"topTextTableSourceGapEvidence\":{\"source\":\"/DocumentText\",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"anchorSelection\":\"nearest-preceding-full-width-line-header\",\"anchorRole\":\"instruction\",\"anchorText\":\"空欄を埋めて表を完成させなさい。\""
    ));
    assert!(layer_tree.contains(
        "\"anchorSourceUnitRange\":{\"start\":501,\"end\":517},\"anchorLineHeaderUnitRange\":{\"start\":487,\"end\":499},\"anchorOffsetUnits\":0,\"anchorExtentUnits\":174,\"anchorFontSizeUnits\":12"
    ));
    assert!(layer_tree.contains(
        "\"sharedFullExtentWithTable\":true,\"sharedFontSizeWithTable\":true,\"sourceTablePlacementCoherenceGate\""
    ));
    assert!(layer_tree.contains(
        "\"sourceTablePlacementCoherenceGate\":{\"source\":\"topTextLineHeaders+documentTextTableLineHeaders\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"candidateSourceUnitRange\":{\"start\":519,\"end\":924},\"sourceGapAfterAnchorTextUnits\":2,\"coherentWithTopTextAnchor\":true,\"sharedFullExtentWithTable\":true,\"sharedFontSizeWithTable\":true"
    ));
    assert!(layer_tree.contains(
        "\"candidateWidthBasis\":\"matched-cells-plus-first-trailing-header\",\"selectedXUnitRange\":{\"start\":0,\"end\":88},\"selectedWidthUnits\":88,\"fullExtentUnits\":174,\"fullExtentTrailingAfterSelectedUnits\":86"
    ));
    assert!(layer_tree.contains(
        "\"firstTrailingHeaderUnitRange\":{\"start\":76,\"end\":88},\"secondTrailingHeaderUnitRange\":{\"start\":92,\"end\":174},\"firstTrailingGapAfterMatchedCellsUnits\":4,\"secondTrailingGapAfterFirstTrailingUnits\":4"
    ));
    assert!(layer_tree.contains(
        "\"visibleRangeUsesFirstTrailingHeader\":true,\"fullExtentIncludesSecondTrailingHeader\":true,\"trailingHeadersCoherent\":true,\"sourcePlacementCoherenceReady\":true,\"blockedReasons\":[],\"sourceTopTextPlacementReadinessGate\""
    ));
    assert!(layer_tree.contains(
        "\"sourceTopTextPlacementReadinessGate\":{\"source\":\"topTextLineHeaders+documentTextTableLineHeaders+/LineMark\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"candidateSourceUnitRange\":{\"start\":519,\"end\":924},\"anchorSelection\":\"nearest-preceding-full-width-line-header\",\"sourceGapAfterAnchorTextUnits\":2,\"coherentWithTopTextAnchor\":true,\"sharedFullExtentWithTable\":true,\"sharedFontSizeWithTable\":true"
    ));
    assert!(layer_tree.contains(
        "\"candidateWidthBasis\":\"matched-cells-plus-first-trailing-header\",\"selectedXUnitRange\":{\"start\":0,\"end\":88},\"selectedWidthUnits\":88,\"fullExtentUnits\":174,\"trailingHeadersCoherent\":true,\"allRowsAgree\":true"
    ));
    assert!(layer_tree.contains(
        "\"lineHeaderRowsHomogeneous\":true,\"lineMarkRowsExactAndContiguous\":true,\"sourcePlacementCoherenceReady\":true,\"sourceVisibleWidthVsFullExtentGate\""
    ));
    assert!(layer_tree.contains(
        "\"sourceVisibleWidthVsFullExtentGate\":{\"source\":\"documentTextLineHeaders visible-width vs full-extent gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"selectedXUnitRange\":{\"start\":0,\"end\":88},\"selectedWidthUnits\":88,\"fullExtentUnits\":174,\"fullExtentTrailingAfterSelectedUnits\":86,\"firstTrailingHeaderUnitRange\":{\"start\":76,\"end\":88},\"secondTrailingHeaderUnitRange\":{\"start\":92,\"end\":174}"
    ));
    assert!(layer_tree.contains(
        "\"selectedClosesAtFirstTrailingHeader\":true,\"secondTrailingIsFullLineRemainder\":true,\"allRowsAgree\":true,\"lineHeaderRowsHomogeneous\":true,\"lineMarkRowsExactAndContiguous\":true,\"visibleWidthSemanticsReady\":true,\"renderPromotionContribution\":\"source-visible-width-vs-full-extent-gate\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"sourceTopTextPageGridCouplingGate\":{\"source\":\"topTextAnchor+/LineMark+/PageMark+tableLineMarkPageOrigin\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"anchorSourceGridPresent\":true,\"tableLineMarkPageOriginPresent\":true,\"anchorLineMarkRecordIndex\":15,\"firstTableLineMarkRecordIndex\":16,\"lineMarkRecordGapAfterAnchor\":1"
    ));
    assert!(layer_tree.contains(
        "\"anchorPageMarkEntryIndex\":0,\"tablePageMarkEntryIndex\":0,\"samePageMarkEntry\":true"
    ));
    assert!(layer_tree.contains(
        "\"expectedAdjacentRows\":true,\"sourcePageGridCouplingReady\":true,\"renderPromotionContribution\":\"source-top-text-page-grid-coupling\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"sourceTopTextPlacementReady\":true,\"readinessBlockedReasons\":[],\"renderPromotionContribution\":\"source-top-text-placement-readiness-gate\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"source-top-text-trailing-header-placement-coherence\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"tableCandidateUnitRange\":{\"start\":519,\"end\":924},\"firstRowUnitRange\":{\"start\":519,\"end\":655}"
    ));
    assert!(layer_tree.contains(
        "\"firstRowLineHeaderUnitRange\":{\"start\":558,\"end\":570},\"firstRowLineMarkRecordIndex\":16,\"sourceGapAfterAnchorTextUnits\":2,\"sourceGapAfterAnchorLineHeaderUnits\":20"
    ));
    assert!(layer_tree.contains(
        "\"firstRowHeaderGapAfterAnchorTextUnits\":41,\"firstRowHeaderGapAfterTableStartUnits\":39,\"sourceTopTextPlacementReady\":true,\"readinessBlockedReasons\":[],\"renderPromotionContribution\":\"source-top-text-placement-readiness\",\"renderPromotionBlockedReason\":null"
    ));
    assert!(layer_tree.contains(
        "\"topTextAnchorResidualEvidence\":{\"source\":\"successDataTestTopTextProjection+documentTextLineHeaders+referenceTableBBox\",\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(
        layer_tree.contains("\"sharedFullExtentAnchorCount\":2,\"sharedFontSizeAnchorCount\":2")
    );
    assert!(
        layer_tree.contains(
            "\"tableLeftMinusAnchorLeftPx\":13.397,\"tableTopMinusAnchorBaselinePx\":2.916"
        )
    );
    assert!(layer_tree.contains(
        "\"independentPageTransformEvidence\":{\"present\":false,\"blockedReason\":\"top-text-page-coordinates-and-table-bbox-are-reference-backed\"}"
    ));
    assert!(layer_tree.contains("\"matchedRowCount\":3"));
    assert!(layer_tree.contains("\"sourceStart\":558,\"sourceEnd\":570"));
    assert!(layer_tree.contains("\"offsetUnits\":92,\"extentUnits\":174"));
    assert!(layer_tree.contains("\"layoutStreamProbe\":{\"lineMarkPresent\":true"));
    assert!(layer_tree.contains("\"lineMarkByteLength\":334"));
    assert!(layer_tree.contains("\"lineMarkWordCount\":167"));
    assert!(layer_tree.contains("\"lineMarkProfile\":\"be16-delta-v1\""));
    assert!(layer_tree.contains("\"lineMarkIntervalCount\":78"));
    assert!(layer_tree.contains(
        "\"lineMarkRowEvidence\":{\"source\":\"/LineMark\",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"candidateBasis\":\"unit\",\"candidateUnitRange\":{\"start\":519,\"end\":924},\"candidateRowCount\":3,\"matchedRowCount\":3,\"exactRowMatchCount\":3"
    ));
    assert!(layer_tree.contains("\"rowCountMatchesCandidate\":true"));
    assert!(layer_tree.contains("\"contiguousRecordIndexes\":true"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"line-mark-units-not-y-page-coordinate-transform\""
    ));
    assert!(
        layer_tree.contains(
            "\"lineMarkRecordIndex\":16,\"lineMarkUnitRange\":{\"start\":519,\"end\":655}"
        )
    );
    assert!(
        layer_tree.contains(
            "\"lineMarkRecordIndex\":18,\"lineMarkUnitRange\":{\"start\":791,\"end\":924}"
        )
    );
    assert!(layer_tree.contains("\"candidateStartWithinLineMarkWordIndex\":false"));
    assert!(layer_tree.contains("\"candidateEndWithinLineMarkWordIndex\":false"));
    assert!(layer_tree.contains("\"pageMarkPresent\":true"));
    assert!(layer_tree.contains("\"pageMarkEntryCount\":2"));
    assert!(layer_tree.contains(
        "\"pageMarkLineMarkRecordEvidence\":{\"source\":\"/PageMark+/LineMark\",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkMatchedRowCount\":3,\"rowPageMatchCount\":3,\"allRowsPageMatched\":true,\"singlePageMatched\":true"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordRange\":{\"start\":16,\"end\":19},\"matchedPageMarkEntryIndex\":0"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"page-association-evidence-only\",\"renderPromotionBlockedReason\":\"page-mark-line-index-not-y-coordinate\""
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndex\":16,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":39,\"withinPageLineRange\":true"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndex\":18,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":39,\"withinPageLineRange\":true"
    ));
    assert!(layer_tree.contains(
        "\"paperMarkPageAssociationEvidence\":{\"source\":\"/PageMark+/PaperMark+/LineMark\",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkMatchedRowCount\":3,\"paperMarkMatchedRowCount\":3,\"singlePageMatched\":true,\"singlePaperMarkMatched\":true"
    ));
    assert!(layer_tree.contains(
        "\"matchedPageMarkEntryIndex\":0,\"matchedPaperMarkEntryIndex\":0,\"matchedPaperMarkIndex\":0,\"matchedPaperMarkFlags\":65552,\"matchedPaperMarkFlagsHex\":\"0x00010010\""
    ));
    assert!(layer_tree.contains(
        "\"pagePaperMarkEntryCountAligned\":false,\"renderPromotionContribution\":\"paper-mark-page-row-evidence-only\",\"renderPromotionBlockedReason\":\"paper-mark-flag-semantics-undecoded\""
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndex\":16,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"paperMarkEntryIndex\":0,\"paperMarkIndex\":0,\"paperMarkFlags\":65552,\"paperMarkFlagsHex\":\"0x00010010\""
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndex\":18,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"paperMarkEntryIndex\":0,\"paperMarkIndex\":0,\"paperMarkFlags\":65552,\"paperMarkFlagsHex\":\"0x00010010\""
    ));
    assert!(layer_tree.contains("\"candidateRangeDirectPageMarkLineHitCount\":0"));
    assert!(layer_tree.contains("\"paperMarkPresent\":true"));
    assert!(layer_tree.contains("\"paperMarkByteLength\":76"));
    assert!(layer_tree.contains("\"paperMarkEntryCount\":8"));
    assert!(layer_tree.contains("\"paperMarkHeaderCount\":8"));
    assert!(layer_tree.contains("\"paperMarkHeaderStride\":12"));
    assert!(layer_tree.contains("\"paperMarkHeaderLastIndex\":7"));
    assert!(layer_tree.contains("\"pagePaperMarkEntryCountAligned\":false"));
    assert!(layer_tree.contains("\"objectFrameRecordCount\":56"));
    assert!(layer_tree.contains("\"objectFrameSourceUnitLinkCount\":0"));
    assert!(layer_tree.contains("\"directPlacementEvidence\":false"));
    assert!(layer_tree.contains("\"renderPromotionBlockedReason\":null"));
    assert!(layer_tree.contains("\"sourceRange\":{\"basis\":\"unit\",\"start\":582,\"end\":585"));
    assert!(layer_tree.contains(
        "\"text\":\"ａ\",\"renderText\":\"  ａ\",\"renderTextBasis\":\"documentTextSourceRangePreservedWhitespace\",\"preservesSourceWhitespace\":true"
    ));
    assert!(layer_tree.contains(
        "\"text\":\"０.８\",\"renderText\":\"０.８\",\"renderTextBasis\":\"normalizedTableSegmentText\",\"preservesSourceWhitespace\":false"
    ));
    assert!(
        diagnostic_reference_table_grid_overlay_layout(
            PageLayout::new(
                SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX,
                SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
            ),
            &document,
            abc_table,
            abc_grid.column_count()
        )
        .is_some()
    );
    assert!(
        reference_table_grid_overlay_layout(
            PageLayout::new(
                SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX,
                SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
            ),
            &document,
            abc_table,
            abc_grid.column_count()
        )
        .is_none()
    );
    let mut document_with_unproven_layout = document.clone();
    document_with_unproven_layout.push_text_count_range(TextCountRange {
        index: 999,
        family: "test-unproven-source-layout".to_string(),
        start: abc_table.source_start() as u32,
        end: abc_table.source_end() as u32,
        declared_start: abc_table.source_start() as u32,
        declared_end: abc_table.source_end() as u32,
        tail_fields: Vec::new(),
        document_text_overlaps: Vec::new(),
        control_range_overlaps: Vec::new(),
        raw: Vec::new(),
    });
    let abc_table_with_unproven_layout = document_with_unproven_layout
        .table_candidates()
        .iter()
        .find(|candidate| success_data_test_abc_table_candidate(candidate))
        .unwrap();
    assert!(table_grid_source_layout_evidence_present(
        &document_with_unproven_layout
    ));
    assert!(table_grid_decoded_source_placement_evidence_present(
        &document_with_unproven_layout,
        abc_table_with_unproven_layout
    ));
    assert!(
        diagnostic_success_data_test_reference_table_grid_overlay_layout(
            PageLayout::new(
                SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX,
                SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
            ),
            &document_with_unproven_layout,
            abc_table_with_unproven_layout
        )
        .is_some()
    );
    assert!(
        diagnostic_reference_table_grid_overlay_layout(
            PageLayout::new(
                SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX,
                SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
            ),
            &document_with_unproven_layout,
            abc_table_with_unproven_layout,
            abc_table_with_unproven_layout
                .column_segment_grid_candidate()
                .unwrap()
                .column_count()
        )
        .is_some()
    );
    assert!(
        reference_table_grid_overlay_layout(
            PageLayout::new(
                SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX,
                SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX
            ),
            &document_with_unproven_layout,
            abc_table_with_unproven_layout,
            abc_table_with_unproven_layout
                .column_segment_grid_candidate()
                .unwrap()
                .column_count()
        )
        .is_none()
    );
    assert!(!table_grid_reference_layout_visible_fallback_allowed(
        &document_with_unproven_layout,
        abc_table_with_unproven_layout
    ));
    let unproven_core = DocumentCore::from_document(document_with_unproven_layout.clone());
    let unproven_lines = unproven_core.page_lines(0).unwrap();
    let unproven_page_layout = PageLayout::new(
        SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX,
        SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX,
    );
    let abc_unproven_column_count = abc_table_with_unproven_layout
        .column_segment_grid_candidate()
        .unwrap()
        .column_count();
    let source_unproven_layout = source_derived_table_grid_overlay_layout(
        unproven_page_layout,
        &document_with_unproven_layout,
        unproven_lines,
        0,
        abc_table_with_unproven_layout,
        abc_unproven_column_count,
    )
    .unwrap();
    let fallback_unproven_layout = table_grid_overlay_layout(
        unproven_page_layout,
        &document_with_unproven_layout,
        unproven_lines,
        0,
        abc_table_with_unproven_layout,
        abc_unproven_column_count,
    );
    assert!((fallback_unproven_layout.0 - source_unproven_layout.x).abs() < 0.001);
    assert!((fallback_unproven_layout.1 - source_unproven_layout.y).abs() < 0.001);
    assert!((fallback_unproven_layout.2 - source_unproven_layout.width).abs() < 0.001);
    assert!((fallback_unproven_layout.3 - source_unproven_layout.row_height).abs() < 0.001);
    assert!((fallback_unproven_layout.4 - source_unproven_layout.column_width).abs() < 0.001);
    assert!(layer_tree.contains("\"projectionKind\":\"successDataTestTopTextProjection\""));
    assert!(layer_tree.contains("\"role\":\"question-heading\""));
    assert!(layer_tree.contains("\"text\":\"２，下の表は、ｃが斜辺の直角三角形で３辺ａ、ｂ、ｃの長さの関係を表したものである。\""));
    assert!(layer_tree.contains("\"sourceBacked\":true"));
    assert!(layer_tree.contains("\"sourceStream\":\"/DocumentText\""));
    assert!(layer_tree.contains("\"jtdUnitRange\":{\"start\":264,\"end\":275}"));
    assert!(layer_tree.contains("\"jtdUnitRange\":{\"start\":426,\"end\":467}"));
    assert!(layer_tree.contains("\"jtdUnitRange\":{\"start\":1506,\"end\":1509}"));
    assert!(layer_tree.contains("\"renderSource\":\"document-text-fixed-pitch-span\""));
    assert!(layer_tree.contains(
        "\"sourceLine\":{\"text\":\"   （１）                     （２）                   （３）\",\"renderSource\":\"document-text-preserved-spacing\""
    ));
    assert!(
        layer_tree.contains(
            "\"advanceModel\":\"japanese-fixed-pitch-halfwidth-space\",\"fontSize\":14.000"
        )
    );
    assert!(layer_tree.contains("\"bbox\":{\"x\":247.700,\"y\":646.000"));
    assert!(layer_tree.contains(
        "\"sourceGridPlacementCandidate\":{\"source\":\"/LineMark+/PageMark+documentTextLineHeaders\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":true,\"recordIndex\":27"
    ));
    assert!(layer_tree.contains(
        "\"rowTopY\":639.000,\"rowHeight\":21.000,\"rowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\",\"sourcePitchEvidence\":{\"source\":\"/DocumentText+/LineMark+/PageMark\""
    ));
    assert!(layer_tree.contains(
        "\"recordIndex\":27,\"pageMarkEntryIndex\":0,\"pageLineStart\":0,\"pageLineEnd\":39,\"lineOffsetFromPageStart\":27,\"pageSizePx\":{\"width\":687.874,\"height\":971.339},\"bodySizePx\":{\"width\":543.874,\"height\":827.339},\"marginPx\":72.000"
    ));
    assert!(layer_tree.contains(
        "\"rowHeightPx\":21.000,\"rowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\",\"fontSizeUnits\":12,\"fontUnitScalePx\":1.750,\"bodyHeightPxPerLineGap\":21.214,\"sourceRowHeightMinusBodyHeightPerLineGapPx\":-0.214"
    ));
    assert!(layer_tree.contains(
        "\"pageHeightPxPerWord21Unit\":1.750,\"pageHeightPxPerWord13Plus14Unit\":1.750,\"fontUnitScaleMatchesPageMarkWord21Candidate\":true,\"fontUnitScaleMatchesPageMarkWord13Plus14Candidate\":true"
    ));
    assert!(layer_tree.contains(
        "\"fontSize\":14.000,\"topY\":646.000,\"baselineY\":660.000,\"referenceResidualEvidence\":{\"diagnosticOnly\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"referenceTopY\":644.200,\"referenceBaselineY\":658.200,\"topMinusSourceTopPx\":-1.800,\"baselineMinusSourceBaselinePx\":-1.800"
    ));
    assert!(layer_tree.contains(
        "\"lineHeaderEvidence\":{\"source\":\"/DocumentText\",\"present\":true,\"sourceByteRange\":{\"start\":502,\"end\":526},\"sourceUnitRange\":{\"start\":251,\"end\":263},\"offsetUnits\":156,\"extentUnits\":174,\"fontSizeUnits\":12,\"rawWords\":[28,48,12,0,156,174,255,0,12,0,48,31],\"rawWordsHex\":[\"0x001c\",\"0x0030\",\"0x000c\",\"0x0000\",\"0x009c\",\"0x00ae\",\"0x00ff\",\"0x0000\",\"0x000c\",\"0x0000\",\"0x0030\",\"0x001f\"]}"
    ));
    assert!(layer_tree.contains(
        "\"type\":\"textPlacementResidualSummary\",\"bbox\":{\"x\":0.000,\"y\":0.000,\"width\":0.000,\"height\":0.000},\"projectionKind\":\"successDataTestTextPlacementResidualSummary\""
    ));
    assert!(layer_tree.contains(
        "\"slotCount\":21,\"sourceGridCandidateCount\":21,\"maxAbsTopResidualPx\":2.300,\"maxAbsBaselineResidualPx\":2.300,\"lineHeaderRawWordProfileCount\":2"
    ));
    assert!(layer_tree.contains(
        "\"linePitchFitEvidence\":{\"source\":\"referenceBaselines+/LineMarkRecordIndex+sourceRowHeight\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true"
    ));
    assert!(layer_tree.contains(
        "\"sourceRowHeightPx\":21.000,\"sourceRowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\",\"pageMarkSelectedFields\":{\"source\":\"/PageMark\",\"entryIndex\":0,\"lineStart\":0,\"lineEnd\":39,\"lineCount\":40,\"lineGapCount\":39,\"u16GeometryClass\":\"additive-boundary\""
    ));
    assert!(layer_tree.contains(
        "{\"wordIndex\":21,\"value\":555,\"hex\":\"0x022b\",\"perLineCount\":13.875,\"perLineGapCount\":14.231}"
    ));
    assert!(layer_tree.contains(
        "\"basis\":\"all-visible-reference-baseline-vs-line-mark-record-index\",\"entryCount\":21,\"recordStart\":5,\"recordEnd\":38,\"intercept\":93.712,\"pitch\":20.926,\"rmsResidualPx\":0.473"
    ));
    assert!(layer_tree.contains(
        "\"basis\":\"early-records-through-24-reference-baseline-vs-line-mark-record-index\",\"entryCount\":12,\"recordStart\":5,\"recordEnd\":24,\"intercept\":93.175,\"pitch\":20.976,\"rmsResidualPx\":0.057"
    ));
    assert!(layer_tree.contains(
        "\"basis\":\"late-records-from-26-reference-baseline-vs-line-mark-record-index\",\"entryCount\":9,\"recordStart\":26,\"recordEnd\":38,\"intercept\":90.890,\"pitch\":21.006,\"rmsResidualPx\":0.318"
    ));
    assert!(layer_tree.contains(
        "\"residualBucketBasis\":\"rounded-tenths-top-baseline+line-mark-flag+font-size+line-header-present\""
    ));
    assert!(layer_tree.contains(
        "\"topResidualBucketPx\":\"-1.8\",\"baselineResidualBucketPx\":\"-1.8\",\"flagWord\":32770,\"flagWordHex\":\"0x8002\",\"fontSizeBucket\":\"14.0\",\"lineHeaderPresent\":false,\"count\":3,\"recordIndexes\":[27],\"roles\":{\"figure-label\":3}"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"baseline-residual-source-field-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"lineMarkEvidence\":{\"source\":\"/LineMark\",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"lineMarkProfile\":\"be16-delta-v1\",\"sourceUnitRange\":{\"start\":264,\"end\":275},\"matchedRecordCount\":1,\"records\":[{\"recordIndex\":5,\"unitRange\":{\"start\":264,\"end\":276},\"flagWord\":2,\"flagWordHex\":\"0x0002\",\"containsSourceRange\":true}]"
    ));
    assert!(layer_tree.contains(
        "\"pageGridCandidate\":{\"source\":\"/LineMark+/PageMark+documentTextLineHeaders\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":true,\"recordIndex\":5,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":39,\"lineOffsetFromPageStart\":5,\"rowHeight\":21.000,\"rowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\",\"sourcePitchEvidence\""
    ));
    assert!(layer_tree.contains(
        "\"recordIndex\":5,\"pageMarkEntryIndex\":0,\"pageLineStart\":0,\"pageLineEnd\":39,\"lineOffsetFromPageStart\":5,\"pageSizePx\":{\"width\":687.874,\"height\":971.339},\"bodySizePx\":{\"width\":543.874,\"height\":827.339},\"marginPx\":72.000"
    ));
    assert!(layer_tree.contains(
        "\"rowHeightPx\":21.000,\"rowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\",\"fontSizeUnits\":12,\"fontUnitScalePx\":1.750,\"bodyHeightPxPerLineGap\":21.214,\"sourceRowHeightMinusBodyHeightPerLineGapPx\":-0.214"
    ));
    assert!(layer_tree.contains(
        "\"rowTopY\":177.000,\"baselineY\":198.000,\"baselineBasis\":\"lineMarkRowTopPlusSourceRowHeight\""
    ));
    assert!(layer_tree.contains(
        "\"referenceTopY\":184.500,\"referenceBaselineY\":198.000,\"topMinusRowTopPx\":7.500,\"baselineMinusRowTopPx\":21.000"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkEvidence\":{\"source\":\"/LineMark\",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"lineMarkProfile\":\"be16-delta-v1\",\"sourceUnitRange\":{\"start\":501,\"end\":517},\"matchedRecordCount\":1,\"records\":[{\"recordIndex\":15,\"unitRange\":{\"start\":468,\"end\":519},\"flagWord\":2,\"flagWordHex\":\"0x0002\",\"containsSourceRange\":true}]"
    ));
    assert!(layer_tree.contains(
        "\"recordIndex\":15,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":39,\"lineOffsetFromPageStart\":15,\"rowHeight\":21.000,\"rowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\",\"sourcePitchEvidence\""
    ));
    assert!(layer_tree.contains(
        "\"rowTopY\":387.000,\"baselineY\":408.000,\"baselineBasis\":\"lineMarkRowTopPlusSourceRowHeight\""
    ));
    assert!(layer_tree.contains(
        "\"sourceUnitRange\":{\"start\":1506,\"end\":1509},\"matchedRecordCount\":1,\"records\":[{\"recordIndex\":27,\"unitRange\":{\"start\":1307,\"end\":1532},\"flagWord\":32770,\"flagWordHex\":\"0x8002\",\"containsSourceRange\":true}]"
    ));
    assert!(layer_tree.contains(
        "\"recordIndex\":27,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":39,\"lineOffsetFromPageStart\":27,\"rowHeight\":21.000,\"rowHeightBasis\":\"abc-table-documentTextLineHeaderFontSizeUnits\",\"sourcePitchEvidence\""
    ));
    assert!(layer_tree.contains(
        "\"rowTopY\":639.000,\"baselineY\":660.000,\"baselineBasis\":\"lineMarkRowTopPlusSourceRowHeight\""
    ));
    assert!(layer_tree.contains(
        "\"cellTextXAdjustment\":-1.750,\"cellTextXAdjustmentBasis\":\"documentTextLineHeaderFontUnitPxStrokeCompensation\""
    ));
    assert!(layer_tree.contains(
        "\"cellTextBaselineFactor\":0.770,\"cellTextBaselineBasis\":\"documentTextLineHeaderFontSizeUnitsBaselineCandidate\""
    ));
    assert!(layer_tree.contains(
        "\"cellTextFontWeight\":\"400\",\"cellTextFontWeightBasis\":\"regularTableCellFallbackNoBoldEvidence\""
    ));
    assert!(layer_tree.contains(
        "\"cellTextFontSize\":13.300,\"cellTextFontSizeBasis\":\"documentTextLineHeaderFontSizeUnitsScaledToAppFont\""
    ));
    assert!(
        layer_tree
            .contains("\"lineHeaderEvidence\":{\"source\":\"/DocumentText\",\"present\":false}")
    );
    assert!(layer_tree.contains("\"placementProven\":false"));
    assert!(layer_tree.contains("\"renderable\":false"));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-success-data-test-top-text-projection\""));
    assert!(svg.contains("data-projection-kind=\"successDataTestTopTextProjection\""));
    assert!(svg.contains("data-source-grid-render-required=\"true\""));
    assert!(svg.contains("data-source-grid-rendered-slot-count=\"21\""));
    assert!(svg.contains("data-unbacked-slot-count=\"0\""));
    assert!(svg.contains("data-reference-fallback-rendered-count=\"0\""));
    assert!(!svg.contains("referenceFallback"));
    assert!(svg.contains("class=\"rjtd-success-data-test-top-text\""));
    assert!(svg.contains("data-render-source=\"document-text-fixed-pitch-span\""));
    assert!(svg.contains("data-advance-model=\"japanese-fixed-pitch-halfwidth-space\""));
    assert!(svg.contains("data-source-unit-start=\"1506\" data-source-unit-end=\"1509\""));
    assert!(svg.contains("data-y-basis=\"lineMarkPageGrid\""));
    assert!(svg.contains("data-source-grid-baseline-y=\"660.000\""));
    assert!(svg.contains("x=\"247.7\" y=\"660.0\""));
    assert!(svg.contains(">１，次の計算をしなさい</text>"));
    assert!(svg.contains("表したものである。</text>"));
    assert!(svg.contains("class=\"rjtd-column-grid-candidate\""));
    assert!(svg.contains("data-projection-kind=\"tableProjection\""));
    assert!(svg.contains("data-reference-fallback-admitted=\"false\""));
    assert!(svg.contains("data-reference-fallback-used=\"false\""));
    assert!(svg.contains(
        "data-reference-fallback-blocked-reason=\"active-source-layout-admission-suppresses-reference-fallback\""
    ));
    assert!(svg.contains("data-source-derived-layout-candidate=\"true\""));
    assert!(svg.contains("data-source-derived-layout-reference-backed=\"false\""));
    assert!(svg.contains("data-column-width-basis=\"documentTextLineHeaderCellSlotUnits\""));
    assert!(svg.contains("data-stroke-width=\"1.750\""));
    assert!(svg.contains("data-cell-stroke-width=\"1.750\""));
    assert!(svg.contains("data-stroke-width-basis=\"documentTextLineHeaderFontUnitPx\""));
    assert!(svg.contains("data-cell-text-centered=\"true\""));
    assert!(svg.contains(
        "data-cell-text-alignment-basis=\"documentTextLineHeaderCellSlotCenterCandidate\""
    ));
    assert!(svg.contains("data-cell-text-x-adjustment=\"-1.750\""));
    assert!(svg.contains(
        "data-cell-text-x-adjustment-basis=\"documentTextLineHeaderFontUnitPxStrokeCompensation\""
    ));
    assert!(svg.contains("data-cell-text-baseline-factor=\"0.770\""));
    assert!(svg.contains(
        "data-cell-text-baseline-basis=\"documentTextLineHeaderFontSizeUnitsBaselineCandidate\""
    ));
    assert!(svg.contains("data-cell-text-font-weight=\"400\""));
    assert!(
        svg.contains("data-cell-text-font-weight-basis=\"regularTableCellFallbackNoBoldEvidence\"")
    );
    assert!(svg.contains("data-cell-text-font-size=\"13.300\""));
    assert!(svg.contains(
        "data-cell-text-font-size-basis=\"documentTextLineHeaderFontSizeUnitsScaledToAppFont\""
    ));
    assert!(svg.contains("data-decoded-source-placement-evidence=\"true\""));
    assert!(svg.contains("data-col-count-candidate=\"5\""));
    assert!(svg.contains("data-render-text-basis=\"documentTextSourceRangePreservedWhitespace\" xml:space=\"preserve\" data-render-text-preserves-source-whitespace=\"true\" text-anchor=\"middle\" data-source-range-basis=\"unit\" data-source-start=\"582\" data-source-end=\"585\""));
    assert!(svg.contains(">  ａ</text>"));
    assert!(svg.contains(">  ｃ</text>"));
    assert!(svg.contains(">０.８<"));
    assert_eq!(svg.matches(">０.８</text>").count(), 1);
    assert!(svg.contains("data-whitespace-placement-probe=\"true\""));
    assert!(svg.contains("data-cell-text-centered-with-source-whitespace=\"true\""));
    assert!(svg.contains("data-render-text-leading-whitespace-chars=\"2\""));
    assert!(svg.contains(
        "data-render-trim-candidate-blocked-reason=\"table-cell-whitespace-position-semantics-unproven\""
    ));
    assert!(svg.contains("data-render-trim-candidate-text=\"０.８\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-art\""));
    assert!(svg.contains("data-projection=\"successDataTestTitleArtProjection\""));
    assert!(svg.contains("data-source=\"jsfartArtEmbeddedPressSnapshot\""));
    assert!(svg.contains("data-placement-mode=\"frameRecordContentOffsetAnchor\""));
    assert!(svg.contains("data-horizontal-placement-basis=\"jsfartContentLeft\""));
    assert!(svg.contains("data-horizontal-placement-render-promoted=\"false\""));
    assert!(svg.contains("data-horizontal-content-left-only-x=\"76.687\""));
    assert!(svg.contains("data-horizontal-frame-x=\"76.687\""));
    assert!(svg.contains("data-horizontal-path-x=\"76.687\""));
    assert!(svg.contains("data-horizontal-candidate-frame-x=\"72.907\""));
    assert!(svg.contains("data-horizontal-candidate-path-x=\"80.995\""));
    assert!(svg.contains(
        "data-horizontal-candidate-basis=\"jsfartFrameOuterEdgePlusFrameRecordContentOrigin\""
    ));
    assert!(svg.contains("data-horizontal-stroke-width-source-units=\"100\""));
    assert!(svg.contains("data-horizontal-stroke-outer-adjustment-css-px=\"3.780\""));
    assert!(svg.contains(
        "data-title-source-frame-trace-source=\"JSFart2Contents.frameCandidate+/Frame\""
    ));
    assert!(svg.contains("data-title-source-frame-trace-source-backed=\"true\""));
    assert!(svg.contains("data-title-source-frame-trace-render-promoted=\"false\""));
    assert!(svg.contains("data-title-source-frame-trace-frame-ref=\"1\""));
    assert!(svg.contains("data-title-source-frame-trace-frame-record-object-id=\"1\""));
    assert!(svg.contains("data-title-source-frame-trace-frame-ref-matches-object-id=\"true\""));
    assert!(svg.contains("data-title-source-frame-trace-source-outer-width-units=\"13260\""));
    assert!(svg.contains("data-title-source-frame-trace-frame-record-width-units=\"13260\""));
    assert!(
        svg.contains("data-title-source-frame-trace-outer-width-matches-frame-record=\"true\"")
    );
    assert!(svg.contains("data-title-source-frame-trace-source-outer-height-units=\"1327\""));
    assert!(svg.contains("data-title-source-frame-trace-frame-record-height-units=\"1327\""));
    assert!(
        svg.contains("data-title-source-frame-trace-outer-height-matches-frame-record=\"true\"")
    );
    assert!(svg.contains(
        "data-title-source-frame-trace-horizontal-placement-basis=\"jsfartContentLeft\""
    ));
    assert!(svg.contains("data-title-source-frame-trace-selected-frame-x=\"76.687\""));
    assert!(svg.contains("data-title-source-frame-trace-candidate-frame-x=\"72.907\""));
    assert!(
        svg.contains("data-title-source-frame-trace-frame-scale-y-basis=\"jsfartContentHeight\"")
    );
    assert!(svg.contains("data-title-source-frame-trace-frame-scale-y-units=\"1054\""));
    assert!(svg.contains(
        "data-title-source-frame-trace-conclusion=\"frame-record-and-jsfart-outer-size-agree\""
    ));
    assert!(svg.contains(
        "data-title-source-frame-trace-render-blocked-reason=\"frame-content-split-horizontal-semantics-unproven\""
    ));
    assert!(svg.contains("data-embedding-index=\"24\""));
    assert!(svg.contains("data-class-name=\"JSFart.Art.2\""));
    assert!(svg.contains("data-frame-ref=\"1\""));
    assert!(!svg.contains("rjtd-title-rounded-frame"));
    let answer_layer_tree = core.get_page_layer_tree(1).unwrap();
    assert!(answer_layer_tree.contains("\"type\":\"titleArtProjection\""));
    assert!(answer_layer_tree.contains("\"embeddingIndex\":3"));
    assert!(answer_layer_tree.contains("\"frameRef\":16"));
    assert!(answer_layer_tree.contains(
        "\"pageAssociation\":{\"source\":\"JSFart.Art.2 frameRef source order\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"frameRefs\":[1,16],\"sourceOrderIndex\":1,\"pageNumber\":2}"
    ));
    assert!(answer_layer_tree.contains("\"projectionKind\":\"successDataTestTitleArtProjection\""));
    assert!(answer_layer_tree.contains("\"type\":\"answerSheetProjection\""));
    assert!(
        answer_layer_tree.contains("\"projectionKind\":\"successDataTestAnswerSheetProjection\"")
    );
    assert!(
        answer_layer_tree
            .contains("\"sourcePath\":\"/FigureData/ExpandData/main_data/Data/FDMText\"")
    );
    assert!(answer_layer_tree.contains("\"markerHex\":\"01001660\""));
    assert!(answer_layer_tree.contains("\"textGeometryEvidence\":{\"source\":\"FDMText\""));
    assert!(answer_layer_tree.contains("\"textCount\":15"));
    assert!(answer_layer_tree.contains("\"indexedTextCount\":15"));
    assert!(answer_layer_tree.contains("\"textIndexEntries\":["));
    assert!(answer_layer_tree.contains("\"indexOffset\":290,\"textRecordOffset\":6584"));
    assert!(answer_layer_tree.contains("\"indexOffset\":334,\"textRecordOffset\":7672"));
    assert!(answer_layer_tree.contains(
        "\"bboxExtent\":{\"left\":-15918,\"top\":-12097,\"right\":-10888,\"bottom\":-9241"
    ));
    assert!(answer_layer_tree.contains(
        "\"indexBboxExtent\":{\"left\":-16018,\"top\":-15704,\"right\":-9510,\"bottom\":-9141"
    ));
    assert!(answer_layer_tree.contains(
        "\"triangleSourceBboxCandidate\":{\"left\":-16018,\"top\":-15704,\"right\":-10611,\"bottom\":-10242"
    ));
    assert!(answer_layer_tree.contains(
        "\"trianglePlacementCandidate\":{\"source\":\"FDMTextIndex+projectedFdmLabelSlots\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false"
    ));
    assert!(answer_layer_tree.contains(
        "\"placementBasis\":\"projected-fdm-label-slot-anchors\",\"sourceBbox\":{\"left\":-16018,\"top\":-15704,\"right\":-10611,\"bottom\":-10242}"
    ));
    assert!(answer_layer_tree.contains(
        "\"vertices\":{\"a\":{\"x\":320.017,\"y\":507.956},\"b\":{\"x\":95.954,\"y\":636.003},\"c\":{\"x\":320.017,\"y\":636.003}}"
    ));
    assert!(
        answer_layer_tree.contains(
            "\"labelAnchors\":[{\"text\":\"Ａ\",\"markerOffset\":6584,\"indexOffset\":290"
        )
    );
    assert!(answer_layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"fdmtext-source-to-sheet-transform-undecoded\""
    ));
    assert!(answer_layer_tree.contains("\"text\":\"９㎝\""));
    assert!(answer_layer_tree.contains("\"text\":\"Ａ\""));
    assert!(
        answer_layer_tree.contains(
            "\"figureLinkEvidence\":{\"source\":\"figureLink\",\"sourcePath\":\"/FigureData/ExpandData/main_data/Link\""
        )
    );
    assert!(answer_layer_tree.contains("\"declaredRowCountCandidate\":15"));
    assert!(answer_layer_tree.contains("\"rowStride\":14"));
    assert!(answer_layer_tree.contains("\"relationKinds\":[{\"kind\":22,\"kindHex\":\"0x0016\"}]"));
    assert!(answer_layer_tree.contains("\"documentTextTailEvidence\":true"));
    assert!(answer_layer_tree.contains("\"sparseTableEvidence\":true"));
    assert!(
        answer_layer_tree.contains("\"referenceFrame\":{\"source\":\"answerSheetReferenceFrame\"")
    );
    assert!(answer_layer_tree.contains(
        "\"sourceFrameCandidate\":{\"source\":\"sparseTableSectionAnchors+/LineMark+/PageMark+answerSheetReferenceLocalSchema\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"candidateBasis\":\"section-5-and-6-source-row-tops-vs-merged-answer-area-local-y\",\"sparseTableCandidateIndex\":2,\"sectionAnchorCount\":6,\"topSectionLabel\":\"５\",\"bottomSectionLabel\":\"６\",\"topRowIndex\":17,\"bottomRowIndex\":31,\"topLineMarkRecordIndex\":59,\"bottomLineMarkRecordIndex\":70,\"samePageMarkEntry\":true,\"samePageIndexCandidate\":true"
    ));
    assert!(answer_layer_tree.contains(
        "\"localYAnchorsPt\":{\"top\":205.000,\"bottom\":377.000,\"span\":172.000},\"sourceYAnchorsPx\":{\"top\":471.000,\"bottom\":702.000,\"span\":231.000},\"sourcePxPerSheetPtY\":1.343023,\"referencePxPerSheetPtY\":1.333386,\"derivedFrameTopY\":195.680,\"derivedFrameHeight\":613.762,\"referenceFrameTopY\":190.674,\"referenceFrameHeight\":609.358,\"frameTopResidualPx\":5.006,\"frameHeightResidualPx\":4.404"
    ));
    assert!(answer_layer_tree.contains(
        "\"fdmTextTriangleLabelAnchorCount\":3,\"triangleSourceBbox\":{\"left\":-16018,\"top\":-15704,\"right\":-10611,\"bottom\":-10242}"
    ));
    assert!(answer_layer_tree.contains(
        "\"pageMarkDisambiguationGate\":{\"source\":\"/LineMark+/PageMark section-anchor same-page gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"samePageMarkEntry\":true,\"samePageIndexCandidate\":true,\"disambiguationReady\":true,\"disambiguationClass\":\"same-page-mark-entry-and-page-index-candidate\",\"renderPromotionBlockedReason\":null}"
    ));
    assert!(answer_layer_tree.contains(
        "\"renderPromotionContribution\":\"answer-sheet-source-frame-y-scale-candidate\",\"renderPromotionBlockedReason\":\"answer-sheet-x-width-and-local-schema-source-fields-undecoded\",\"renderPromotionBlockedReasons\":[\"answer-sheet-x-width-and-local-schema-source-fields-undecoded\"]"
    ));
    assert!(answer_layer_tree.contains(
        "\"localRuleSchemaCandidate\":{\"source\":\"sparseTableCandidateTopology+referenceObservedAnswerSheetRuleSegments\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"sparseTableCandidateIndex\":2,\"sparseTableRowCount\":39,\"sparseTableMaxColumnCount\":11,\"sectionAnchorCount\":6"
    ));
    assert!(answer_layer_tree.contains(
        "\"sectionLabel\":\"１\",\"rowIndex\":1,\"sourceIntervalIndex\":15,\"cellIndex\":2,\"cellSourceRange\":{\"start\":3076,\"end\":3079}"
    ));
    assert!(answer_layer_tree.contains(
        "\"referenceRuleSegmentCount\":19,\"referenceHorizontalRuleSegmentCount\":13,\"referenceVerticalRuleSegmentCount\":6,\"referenceLocalXPositionsPt\":[0.000,27.000,168.000,237.000,307.000,445.000],\"referenceLocalYPositionsPt\":[0.000,31.000,63.000,79.000,110.000,142.000,173.000,205.000,236.000,268.000,377.000,409.000,457.000]"
    ));
    assert!(answer_layer_tree.contains(
        "\"sourceFrameCandidatePresent\":true,\"xSchemaSourceBacked\":false,\"yScaleSourceBacked\":true,\"ruleStyleSourceBacked\":false,\"renderPromotionContribution\":\"answer-sheet-local-rule-schema-readiness-gate\",\"renderPromotionBlockedReason\":\"answer-sheet-local-rule-schema-source-fields-undecoded\""
    ));
    assert!(answer_layer_tree.contains(
        "\"ruleStyleCandidate\":{\"source\":\"referenceObservedSparseTableDoubleRules\",\"topologySource\":\"sparseTableCandidateTopology+answerSheetReferenceFrame\",\"topologySourceBacked\":true,\"styleSourceBacked\":false,\"referenceBacked\":true,\"decoded\":false,\"renderMode\":\"primary-plus-source-gated-secondary-line\""
    ));
    assert!(
        answer_layer_tree
            .contains("\"secondaryLineGate\":\"before-source-identified-merged-answer-area\"")
    );
    assert!(answer_layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"answer-sheet-rule-style-source-field-undecoded\""
    ));
    assert!(answer_layer_tree.contains(
        "\"hatchedAnswerAreaCandidate\":{\"source\":\"sparseTableCandidateTopology+answerSheetReferenceFrame\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(answer_layer_tree.contains(
        "\"topSectionLabel\":\"５\",\"bottomSectionLabel\":\"６\",\"topRowIndex\":17,\"bottomRowIndex\":31,\"topSourceIntervalIndex\":31,\"bottomSourceIntervalIndex\":45"
    ));
    assert!(answer_layer_tree.contains(
        "\"emptyCellIndex\":2,\"adjacentAnswerCellIndex\":3,\"sheetBBoxPt\":{\"left\":27.000,\"top\":205.000,\"right\":237.000,\"bottom\":377.000"
    ));
    assert!(answer_layer_tree.contains(
        "\"hatchStyleCandidate\":{\"source\":\"referenceObservedAnswerAreaEdgeHatch\",\"sourceBacked\":false,\"referenceBacked\":true,\"decoded\":false,\"renderMode\":\"diagonal-edge-segments\""
    ));
    assert!(answer_layer_tree.contains("\"coordinateSpace\":\"sheetLocalPt\""));
    assert!(answer_layer_tree.contains("\"originPagePt\":{\"x\":30.000,\"y\":143.000}"));
    assert!(answer_layer_tree.contains("\"sizePt\":{\"width\":445.000,\"height\":457.000}"));
    assert!(answer_layer_tree.contains("\"sparseTableCandidate\":{\"source\":\"tableCandidates\""));
    assert!(answer_layer_tree.contains("\"kind\":\"sparseDocumentTextControlRunTableCandidate\""));
    assert!(answer_layer_tree.contains("\"rowCount\":39"));
    assert!(answer_layer_tree.contains("\"maxColumnCountCandidate\":11"));
    assert!(answer_layer_tree.contains("\"emptyCellCountCandidate\":136"));
    assert!(
        answer_layer_tree
            .contains("\"topologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\"")
    );
    assert!(
        answer_layer_tree
            .contains("\"ruleTopologyEvidence\":{\"source\":\"sparseTableCandidateTopology\"")
    );
    assert!(
        answer_layer_tree
            .contains("\"layoutStreamProbe\":{\"lineMarkPresent\":true,\"lineMarkByteLength\":334")
    );
    assert!(answer_layer_tree.contains(
        "\"lineMarkRowEvidence\":{\"source\":\"/LineMark\",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"candidateBasis\":\"unit\",\"candidateUnitRange\":{\"start\":2902,\"end\":5419},\"candidateRowCount\":39,\"matchedRowCount\":39"
    ));
    assert!(answer_layer_tree.contains(
        "\"sectionLineMarkGeometryCandidate\":{\"source\":\"sparseTableCandidateTopology+/LineMark+/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tableCandidateIndex\":2,\"sectionAnchorCount\":6,\"lineMarkIntervalCount\":78,\"matchedSectionAnchorCount\":6"
    ));
    assert!(answer_layer_tree.contains(
        "\"sectionLabel\":\"５\",\"rowIndex\":17,\"sourceIntervalIndex\":31,\"rowSourceUnitRange\":{\"start\":4177,\"end\":4243},\"cellIndex\":1,\"lineMarkRecordIndex\":59"
    ));
    assert!(answer_layer_tree.contains(
        "\"sectionLabel\":\"６\",\"rowIndex\":31,\"sourceIntervalIndex\":45,\"rowSourceUnitRange\":{\"start\":5093,\"end\":5162},\"cellIndex\":2,\"lineMarkRecordIndex\":70"
    ));
    assert!(answer_layer_tree.contains("\"sectionAnchorCount\":6"));
    assert!(
        answer_layer_tree
            .contains("\"sectionLabel\":\"５\",\"rowIndex\":17,\"sourceIntervalIndex\":31")
    );
    assert!(answer_layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"sparse-topology-to-physical-row-heights-unproven\""
    ));
    assert!(answer_layer_tree.contains("\"columns\":["));
    assert!(answer_layer_tree.contains("\"observedCellCount\":38"));
    assert!(answer_layer_tree.contains("\"firstNonEmptyColumnIndex\":3"));
    assert!(answer_layer_tree.contains("\"textPreview\":\"\\t\\t\\t(1)表面積の比"));
    assert!(answer_layer_tree.contains("\"rows\":["));
    assert!(answer_layer_tree.contains("\"cells\":["));
    assert!(answer_layer_tree.contains("\"cellCount\":7"));
    assert!(answer_layer_tree.contains("\"text\":\"(1)表面積の比\""));
    assert!(answer_layer_tree.contains("\"empty\":true"));
    assert!(answer_layer_tree.contains("\"sourceStart\":"));
    assert!(answer_layer_tree.contains("\"sourceEnd\":"));
    assert!(!answer_layer_tree.contains("その円柱にちょうど入る円錐"));
    let answer_svg = core.render_page_svg(1).unwrap();
    assert!(answer_svg.contains("class=\"rjtd-success-data-test-title-art\""));
    assert!(answer_svg.contains("data-embedding-index=\"3\""));
    assert!(answer_svg.contains("data-frame-ref=\"16\""));
    assert!(answer_svg.contains("class=\"rjtd-success-data-test-answer-sheet\""));
    assert!(answer_svg.contains("data-projection=\"successDataTestAnswerSheetProjection\""));
    assert!(answer_svg.contains("data-fdm-text-marker-hex=\"01001660\""));
    assert!(answer_svg.contains("data-sparse-table-evidence=\"true\""));
    assert!(answer_svg.contains("data-sparse-table-row-count=\"39\""));
    assert!(answer_svg.contains("data-sparse-table-max-columns=\"11\""));
    assert!(answer_svg.contains("data-sparse-table-empty-cells=\"136\""));
    assert!(answer_svg.contains("data-rule-topology-evidence=\"true\""));
    assert!(answer_svg.contains("data-rule-section-anchor-count=\"6\""));
    assert!(answer_svg.contains("data-source-frame-candidate=\"true\""));
    assert!(answer_svg.contains(
        "data-source-frame-source=\"sparseTableSectionAnchors+/LineMark+/PageMark+answerSheetReferenceLocalSchema\""
    ));
    assert!(answer_svg.contains(
        "data-source-frame-basis=\"section-5-and-6-source-row-tops-vs-merged-answer-area-local-y\""
    ));
    assert!(answer_svg.contains("data-source-frame-y-scale-px-per-pt=\"1.343023\""));
    assert!(answer_svg.contains("data-source-frame-derived-top-y=\"195.680\""));
    assert!(answer_svg.contains("data-source-frame-top-residual-px=\"5.006\""));
    assert!(answer_svg.contains("data-source-frame-same-page-mark-entry=\"true\""));
    assert!(answer_svg.contains("data-source-frame-same-page-index-candidate=\"true\""));
    assert!(answer_svg.contains("data-source-frame-page-mark-disambiguation-ready=\"true\""));
    assert!(answer_svg.contains(
        "data-source-frame-page-mark-disambiguation-class=\"same-page-mark-entry-and-page-index-candidate\""
    ));
    assert!(answer_svg.contains(
        "data-source-frame-render-promotion-blocked-reason=\"answer-sheet-x-width-and-local-schema-source-fields-undecoded\""
    ));
    assert!(answer_svg.contains("data-local-rule-schema-candidate=\"true\""));
    assert!(answer_svg.contains(
        "data-local-rule-schema-source=\"sparseTableCandidateTopology+referenceObservedAnswerSheetRuleSegments\""
    ));
    assert!(answer_svg.contains("data-local-rule-schema-x-source-backed=\"false\""));
    assert!(answer_svg.contains("data-local-rule-schema-y-scale-source-backed=\"true\""));
    assert!(answer_svg.contains("data-local-rule-segment-count=\"19\""));
    assert!(answer_svg.contains("data-local-rule-horizontal-segment-count=\"13\""));
    assert!(answer_svg.contains("data-local-rule-vertical-segment-count=\"6\""));
    assert!(answer_svg.contains(
        "data-local-rule-schema-render-promotion-blocked-reason=\"answer-sheet-local-rule-schema-source-fields-undecoded\""
    ));
    assert!(answer_svg.contains("data-coordinate-space=\"sheetLocalPt\""));
    assert!(answer_svg.contains("data-reference-frame-source=\"answerSheetReferenceFrame\""));
    assert!(answer_svg.contains("data-reference-frame-width-pt=\"445.0\""));
    assert!(
        answer_svg
            .contains("data-source=\"sparseTableCandidateTopology+answerSheetReferenceFrame\"")
    );
    assert!(answer_svg.contains("data-edge=\"left\""));
    assert!(answer_svg.contains("data-edge=\"right\""));
    assert!(answer_svg.contains("data-empty-cell-index=\"2\""));
    assert!(answer_svg.contains("data-adjacent-answer-cell-index=\"3\""));
    assert!(answer_svg.contains("data-hatch-render-mode=\"diagonal-edge-segments\""));
    assert!(answer_svg.contains("data-hatch-style-source-backed=\"false\""));
    assert!(answer_svg.contains("class=\"rjtd-success-data-test-answer-sheet-hatch-segment\""));
    assert!(
        answer_svg.contains("data-rule-style-source=\"referenceObservedSparseTableDoubleRules\"")
    );
    assert!(answer_svg.contains("data-rule-style-source-backed=\"false\""));
    assert!(answer_svg.contains("data-rule-style-reference-backed=\"true\""));
    assert!(answer_svg.contains("data-rule-style-decoded=\"false\""));
    assert!(
        answer_svg.contains("data-rule-render-mode=\"primary-plus-source-gated-secondary-line\"")
    );
    assert!(
        answer_svg.contains(
            "data-rule-secondary-line-gate=\"before-source-identified-merged-answer-area\""
        )
    );
    assert!(answer_svg.contains("data-rule-render-mode=\"source-grid-primary-line\""));
    assert!(answer_svg.contains("data-rule-render-mode=\"reference-observed-secondary-line\""));
    assert_eq!(
        answer_svg
            .matches("data-rule-render-mode=\"source-grid-primary-line\"")
            .count(),
        19
    );
    assert_eq!(
        answer_svg
            .matches("data-rule-render-mode=\"reference-observed-secondary-line\"")
            .count(),
        13
    );
    let answer_tokens = success_data_test_answer_sheet_text_tokens(core.document())
        .expect("answer sheet visible labels should be derived from DocumentText tail");
    assert_eq!(answer_tokens[0], "(1)表面積の比");
    assert_eq!(answer_tokens[5], "ｃｍ²");
    assert_eq!(answer_tokens[10], "ｃｍ³");
    assert_eq!(answer_tokens[27], "ＡＢ　＝");
    assert_eq!(answer_tokens[29], "ＡＣ　＝");
    assert!(answer_svg.contains("data-source=\"DocumentText\""));
    assert!(answer_svg.contains("data-source-token-index=\"0\""));
    assert!(answer_svg.contains("data-source-token-index=\"27\""));
    assert!(answer_svg.contains(">ＡＢ　＝</text>"));
    assert!(answer_svg.contains(">表面積</text>"));
    assert!(answer_svg.contains("class=\"rjtd-success-data-test-answer-sheet-fdm-text\""));
    assert!(answer_svg.contains("data-marker-offset=\"6584\""));
    assert!(answer_svg.contains("data-index-offset=\"290\""));
    assert!(answer_svg.contains("data-source-left=\"-13594\""));
    assert!(answer_svg.contains("data-marker-offset=\"7128\""));
    assert!(answer_svg.contains("data-marker-offset=\"7672\""));
    assert!(answer_svg.contains("class=\"rjtd-success-data-test-answer-sheet-triangle\""));
    assert!(answer_svg.contains("data-source=\"FDMTextIndex+projectedFdmLabelSlots\""));
    assert!(answer_svg.contains("data-placement-basis=\"projected-fdm-label-slot-anchors\""));
    assert!(answer_svg.contains("data-placement-proven=\"false\""));
    assert!(answer_svg.contains(
        "data-render-promotion-blocked-reason=\"fdmtext-source-to-sheet-transform-undecoded\""
    ));
    assert!(answer_svg.contains("data-source-left=\"-16018\""));
    assert!(answer_svg.contains("d=\"M 96.0 636.0 L 320.0 508.0 L 320.0 636.0 Z\""));
    assert!(!answer_svg.contains(">その円柱にちょうど入る円錐を表している。</text>"));
    assert!(svg.contains("data-source-scale-x=\"0.037795\""));
    assert!(svg.contains("data-source-scale-y=\"0.037795\""));
    assert!(svg.contains("data-frame-scale-y=\"0.047585\""));
    assert!(svg.contains("data-frame-scale-y-basis=\"jsfartContentHeight\""));
    assert!(svg.contains("data-frame-scale-y-units=\"1054\""));
    assert!(svg.contains("data-main-outline-scale-diagnostic=\"source-scale-vs-frame-scale\""));
    assert!(svg.contains("data-main-outline-scale-diagnostic-pixel-change=\"false\""));
    assert!(svg.contains("data-content-left-adjustment-source-units=\"114\""));
    assert!(svg.contains("data-content-left-adjustment-css-px=\"4.309\""));
    assert!(svg.contains("data-content-top-adjustment-source-units=\"105\""));
    assert!(svg.contains("data-content-top-adjustment-css-px=\"3.969\""));
    assert!(svg.contains("data-frame-content-top-adjustment-css-px=\"4.996\""));
    assert!(svg.contains("data-vertical-stroke-center-adjustment-css-px=\"1.067\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-frame\""));
    assert!(svg.contains("data-source=\"JSFart2Contents\""));
    assert!(svg.contains("data-source-left=\"0\""));
    assert!(svg.contains("data-source-content-left=\"114\""));
    assert!(svg.contains("data-source-content-bottom=\"1159\""));
    assert!(svg.contains("data-source-corner-radius-x=\"114\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-art-paths\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-art-extrusion-path\""));
    assert!(svg.contains("data-title-layer=\"extrusion\""));
    assert!(svg.contains("data-title-face=\"long-shadow-side-sweep\""));
    assert!(svg.contains("data-title-side-source=\"contour-edge-strip\""));
    assert!(svg.contains("data-title-compositing=\"shadow-under-front-face\""));
    assert!(svg.contains("data-title-shadow-effect-opacity=\"0.440\""));
    assert!(svg.contains("data-title-shadow-effect-word0=\"0x2c\""));
    assert!(svg.contains(
        "data-title-shadow-fill-source=\"embedded-press-0x70-word0-percent-black-on-white\""
    ));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-art-shadow-face-path\""));
    assert!(svg.contains("data-title-layer=\"shadow-face\""));
    assert!(svg.contains("data-title-face=\"source-shadow-outline\""));
    assert!(svg.contains("data-title-compositing=\"source-order-shadow-under-front-face\""));
    assert!(svg.contains("data-title-fill-rule-source=\"embedded-press-nonzero-winding\""));
    assert!(svg.contains("fill=\"#8f8f8f\""));
    assert!(!svg.contains("rjtd-success-data-test-title-art-extrusion-mask-24"));
    assert!(!svg.contains("mask=\"url(#rjtd-success-data-test-title-art-extrusion-mask-24)\""));
    assert!(!svg.contains("data-title-face=\"paired-outline-ribbon\""));
    assert!(!svg.contains("data-title-layer=\"extrusion-shadow\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-art-textures\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-art-texture-path\""));
    assert!(svg.contains("data-title-layer=\"extrusion-texture\""));
    assert!(svg.contains(
        "data-title-texture-path-source=\"interstitial-between-shadow-and-main-outlines\""
    ));
    assert!(svg.contains("data-title-texture-clip-source=\"source-shadow-outline\""));
    assert!(svg.contains(
        "data-title-texture-clip-gate-source=\"embeddedPressOutlineTextureOutlineClipArbitration\""
    ));
    assert!(svg.contains("data-title-texture-clip-gate-reference-backed=\"true\""));
    assert!(svg.contains("data-title-texture-clip-gate-render-promoted=\"true\""));
    assert!(svg.contains("data-title-texture-clip-gate-path-count=\"530\""));
    assert!(svg.contains("data-title-texture-selected-clip-source=\"source-shadow-outline\""));
    assert!(svg.contains("data-title-texture-alternative-clip-source=\"long-shadow-side-sweep\""));
    assert!(svg.contains("data-title-texture-alternative-clip-rejected=\"true\""));
    assert!(svg.contains(
        "data-title-texture-alternative-clip-rejected-reason=\"long-shadow-side-sweep-texture-clip-worsened-title-crops\""
    ));
    assert!(svg.contains("data-title-texture-front-face-knockout-decoded=\"false\""));
    assert!(svg.contains("data-title-texture-clip-semantics-decoded=\"false\""));
    assert!(svg.contains(
        "data-title-texture-clip-semantics-blocked-reason=\"texture-clip-and-knockout-semantics-unproven\""
    ));
    assert!(svg.contains(
        "data-title-texture-geometry-role-gate-source=\"embeddedPressSourceBboxRoleComparison\""
    ));
    assert!(svg.contains("data-title-texture-geometry-role-gate-source-backed=\"true\""));
    assert!(svg.contains("data-title-texture-geometry-role-gate-reference-backed=\"false\""));
    assert!(svg.contains("data-title-texture-geometry-role-gate-render-promoted=\"false\""));
    assert!(svg.contains(
        "data-title-texture-geometry-role-conclusion=\"texture-bbox-overlaps-main-and-shadow-outline-bboxes\""
    ));
    assert!(svg.contains(
        "data-title-texture-geometry-role-blocked-reason=\"texture-source-bbox-relation-is-bbox-only-not-knockout-proof\""
    ));
    assert!(svg.contains("data-title-texture-geometry-main-overlap-ratio=\"0.903\""));
    assert!(svg.contains("data-title-texture-geometry-shadow-overlap-ratio=\"0.903\""));
    assert!(svg.contains("data-title-texture-geometry-side-sweep-overlap-ratio=\"1.000\""));
    assert!(svg.contains("data-title-texture-geometry-contained-by-main-bbox=\"false\""));
    assert!(svg.contains("data-title-texture-geometry-contained-by-shadow-bbox=\"false\""));
    assert!(svg.contains("data-title-texture-geometry-contained-by-side-sweep-bbox=\"true\""));
    assert!(svg.contains(
        "data-title-texture-source=\"embedded-press-texture-bezier-flags-1-filled-source-paths\""
    ));
    assert!(svg.contains("data-title-texture-rendering=\"filled-source-paths\""));
    assert!(svg.contains("data-title-texture-render-fill=\"#111111\""));
    assert!(svg.contains("data-title-texture-effect-candidate-opacity=\"0.280\""));
    assert!(svg.contains("data-title-texture-effect-candidate-word0=\"0x1c\""));
    assert!(svg.contains("data-title-texture-effect-candidate-base-fill=\"#8f8f8f\""));
    assert!(svg.contains("data-title-texture-effect-candidate-fill=\"#676767\""));
    assert!(svg.contains(
        "data-title-texture-effect-candidate-source=\"embedded-press-interstitial-0x70-word0-percent-black-over-shadow\""
    ));
    assert!(svg.contains(
        "data-title-texture-effect-render-promoted=\"false\" data-title-texture-effect-render-promotion-blocked-reason=\"record70-separates-shadow-but-not-interstitial-texture-from-main\""
    ));
    assert!(svg.contains("data-texture-path-count=\"530\""));
    assert!(svg.contains("data-title-effective-shadow-texture-path-count=\"530\""));
    assert!(svg.contains("data-title-effective-texture-state-word5-values=\"0x2f\""));
    assert!(svg.contains(
        "data-title-texture-state-source=\"embedded-press-current-paint-state-inheritance\""
    ));
    assert!(svg.contains("data-texture-bezier-header-count=\"530\""));
    assert!(svg.contains("data-texture-bezier-point-count=\"13\""));
    assert!(svg.contains("data-texture-bezier-byte-count=\"104\""));
    assert!(svg.contains("data-texture-bezier-flags-hex=\"0x00000001\""));
    assert!(svg.contains("data-texture-bezier-homogeneous=\"true\""));
    assert!(!svg.contains("data-texture-hatch-distance-cm=\"0.08\""));
    assert!(!svg.contains("data-texture-hatch-line-width-cm=\"0.01\""));
    assert!(!svg.contains("width=\"3.0\" height=\"3.0\"><path d=\"M 0.0 3.0 L 3.0 0.0\""));
    assert!(!svg.contains("-texture-pattern"));
    assert!(svg.contains("fill=\"#111111\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-art-path\""));
    assert!(svg.contains("data-title-layer=\"front-fill\""));
    assert!(svg.contains("data-title-fill-source=\"raw-embedded-press-path\""));
    assert!(
        svg.contains("data-title-fill-rule-source=\"embedded-press-evenodd-boundary-contours\"")
    );
    assert!(svg.contains(
        "data-title-front-fill-winding-gate-source=\"embeddedPressContourWinding+popplerTitleCropAB\""
    ));
    assert!(svg.contains("data-title-front-fill-winding-source-backed=\"true\""));
    assert!(svg.contains("data-title-front-fill-winding-reference-backed=\"true\""));
    assert!(svg.contains("data-title-front-fill-winding-render-promoted=\"true\""));
    assert!(svg.contains("data-title-front-fill-selected-rule=\"evenodd\""));
    assert!(svg.contains(
        "data-title-front-fill-selected-rule-source=\"embedded-press-evenodd-boundary-contours\""
    ));
    assert!(svg.contains("data-title-front-fill-previous-rule=\"nonzero\""));
    assert!(svg.contains("data-title-front-fill-path-count=\"11\""));
    assert!(svg.contains("data-title-front-fill-multi-contour-path-count=\"11\""));
    assert!(svg.contains("data-title-front-fill-opposite-signed-contour-path-count=\"4\""));
    assert!(svg.contains("data-title-front-fill-nonzero-title-tight-rms=\"78.059\""));
    assert!(svg.contains("data-title-front-fill-evenodd-title-tight-rms=\"76.034\""));
    assert!(svg.contains("data-title-front-fill-rms-improvement=\"2.025\""));
    assert!(svg.contains("data-title-front-fill-render-color=\"#111111\""));
    assert!(svg.contains(
        "data-title-front-fill-render-color-source=\"conservative-front-fill-fallback-source-paint-mismatch\""
    ));
    assert!(svg.contains("data-title-front-fill-render-color-source-backed=\"false\""));
    assert!(
        svg.contains("data-title-front-fill-source-paint-color-matches-render-color=\"false\"")
    );
    assert!(svg.contains(
        "data-title-front-fill-render-color-promotion-blocked-reason=\"source-paint-color-does-not-match-render-fill\""
    ));
    assert!(svg.contains(
        "data-title-front-paint-source-trace-source=\"JSFart2Contents.paintCandidateRawWords+frontFillRenderColorGate\""
    ));
    assert!(svg.contains("data-title-front-paint-source-trace-source-backed=\"true\""));
    assert!(svg.contains("data-title-front-paint-source-trace-render-promoted=\"false\""));
    assert!(svg.contains("data-title-front-paint-source-trace-style-word1=\"0x02141030\""));
    assert!(svg.contains("data-title-front-paint-source-trace-style-word2=\"0x02141018\""));
    assert!(svg.contains("data-title-front-paint-source-trace-paint-color=\"0x00ffffff\""));
    assert!(svg.contains("data-title-front-paint-source-trace-paint-color-css=\"#ffffff\""));
    assert!(svg.contains("data-title-front-paint-source-trace-paint-flag=\"0x00000001\""));
    assert!(svg.contains("data-title-front-paint-source-trace-effect-word=\"0x0000000a\""));
    assert!(svg.contains("data-title-front-paint-source-trace-selected-fill=\"#111111\""));
    assert!(
        svg.contains("data-title-front-paint-source-trace-source-paint-matches-render=\"false\"")
    );
    assert!(svg.contains(
        "data-title-front-paint-source-trace-render-texture-path-source=\"source-order-interstitial-front-erase-texture\""
    ));
    assert!(svg.contains(
        "data-title-front-paint-source-trace-render-blocked-reason=\"front-erase-texture-over-main-face-semantics-unproven\""
    ));
    assert!(svg.contains(
        "data-title-front-paint-source-trace-conclusion=\"source-paint-present-but-render-fill-not-promoted\""
    ));
    assert!(svg.contains(
        "data-title-front-paint-arbitration-source=\"JSFart2Contents+EmbeddedPressPaintState+frontEraseTextureProbes\""
    ));
    assert!(svg.contains("data-title-front-paint-arbitration-policy=\"conservative-front-fill\""));
    assert!(svg.contains("data-title-front-paint-arbitration-candidate-count=\"4\""));
    assert!(svg.contains("data-title-front-paint-arbitration-selected-fill=\"#111111\""));
    assert!(svg.contains("data-title-front-paint-arbitration-source-paint-present=\"true\""));
    assert!(
        svg.contains("data-title-front-paint-arbitration-source-paint-matches-render=\"false\"")
    );
    assert!(svg.contains("data-title-front-paint-arbitration-direct-gray-present=\"true\""));
    assert!(
        svg.contains("data-title-front-paint-arbitration-texture-source-paint-present=\"true\"")
    );
    assert!(svg.contains("data-title-front-paint-arbitration-span-candidate-present=\"true\""));
    assert!(svg.contains("data-title-front-paint-arbitration-span-count=\"11\""));
    assert!(svg.contains(
        "data-title-front-paint-arbitration-transition-boundary=\"source-order-bracketed-interstitial-texture-block\""
    ));
    assert!(svg.contains(
        "data-title-front-paint-arbitration-paint-intent=\"shadow-state-texture-inside-main-boundary-ambiguous\""
    ));
    assert!(svg.contains(
        "data-title-front-paint-arbitration-blocked-reason=\"front-paint-candidate-arbitration-unproven\""
    ));
    assert!(!svg.contains("temporary-front-fill-pending-source-paint-role"));
    assert!(svg.contains("data-title-front-paint-candidate-source-backed=\"true\""));
    assert!(svg.contains("data-title-front-paint-candidate-color=\"#ffffff\""));
    assert!(svg.contains(
        "data-title-front-paint-candidate-source=\"JSFart2Contents.paintColorCandidate\""
    ));
    assert!(svg.contains("data-title-front-paint-main-state-texture-path-count=\"0\""));
    assert!(svg.contains("data-title-front-paint-front-erase-texture-path-count=\"530\""));
    assert!(svg.contains(
        "data-title-front-paint-render-texture-path-source=\"source-order-interstitial-front-erase-texture\""
    ));
    assert!(svg.contains("data-title-front-paint-render-path-count=\"530\""));
    assert!(svg.contains("data-title-front-paint-visible-render-path-count=\"0\""));
    assert!(svg.contains(
        "data-title-front-paint-render-promotion-blocked-reason=\"front-erase-texture-over-main-face-semantics-unproven\""
    ));
    assert!(svg.contains(
        "data-title-front-erase-visible-probe-source=\"frontEraseTextureVisibleAB+visualReview\""
    ));
    assert!(svg.contains("data-title-front-erase-visible-probe-source-backed=\"true\""));
    assert!(svg.contains("data-title-front-erase-visible-probe-reference-backed=\"true\""));
    assert!(svg.contains("data-title-front-erase-visible-probe-render-promoted=\"false\""));
    assert!(svg.contains("data-title-front-erase-current-visible-path-count=\"0\""));
    assert!(svg.contains("data-title-front-erase-all-visible-path-count=\"530\""));
    assert!(svg.contains("data-title-front-erase-explicit-state-visible-path-count=\"11\""));
    assert!(svg.contains("data-title-front-erase-current-title-tight-rms=\"76.034\""));
    assert!(svg.contains("data-title-front-erase-all-visible-title-tight-rms=\"67.651\""));
    assert!(svg.contains("data-title-front-erase-explicit-state-title-tight-rms=\"76.016\""));
    assert!(svg.contains("data-title-front-erase-all-visible-visual-rejected=\"true\""));
    assert!(svg.contains(
        "data-title-front-erase-all-visible-rejected-reason=\"gray-overpaint-not-distressed-knockout\""
    ));
    assert!(svg.contains(
        "data-title-front-erase-render-promotion-blocked-reason=\"front-erase-visible-rms-improvement-is-not-knockout-proof\""
    ));
    assert!(svg.contains("fill-rule=\"nonzero\""));
    assert!(svg.contains("clip-rule=\"nonzero\""));
    assert!(svg.contains("data-title-clip-rule-source=\"embedded-press-nonzero-winding\""));
    assert!(svg.contains("fill-rule=\"evenodd\""));
    assert!(!svg.contains("clip-rule=\"evenodd\""));
    assert!(!svg.contains("data-title-layer=\"hatch-texture\""));
    assert!(svg.contains("stroke=\"#111111\""));
    assert!(svg.contains("fill=\"none\""));
    assert!(svg.contains("fill-rule=\"nonzero\""));
    assert!(svg.contains("data-path-kind=\"outline\""));
    assert!(svg.contains("data-path-kind=\"texture\""));
    assert!(svg.contains("data-vector-segment-count=\"9895\""));
    assert!(svg.contains("data-vector-path-count=\"552\""));
    assert!(svg.contains("data-rendered-path-count=\"22\""));
    assert!(svg.contains("data-texture-path-count=\"530\""));
    assert!(svg.contains(
        "data-shadow-pairing-strategy=\"embedded-press-source-order-outline-texture-outline\""
    ));
    assert!(svg.contains("data-shadow-path-count=\"11\""));
    assert!(svg.contains("data-main-path-count=\"11\""));
    assert!(svg.contains("data-shadow-offset-source-x=\"100\""));
    assert!(svg.contains("data-shadow-offset-source-y=\"100\""));
    assert!(svg.contains("rjtd-success-data-test-title-art-main-face-clip-24"));
    assert!(svg.contains("class=\"rjtd-success-data-test-title-art-front-textures\""));
    assert!(!svg.contains("class=\"rjtd-success-data-test-title-art-front-texture-path\""));
    assert!(svg.contains("data-title-layer=\"front-texture\""));
    assert!(svg.contains(
        "data-title-texture-path-source=\"source-order-interstitial-front-erase-texture\""
    ));
    assert!(svg.contains(
        "data-title-texture-rendering=\"source-opacity-front-erase-precomposited-filled-source-paths\""
    ));
    assert!(svg.contains("data-title-texture-render-fill=\"#818181\""));
    assert!(svg.contains("data-title-texture-opacity=\"0.470\""));
    assert!(svg.contains("data-title-texture-render-opacity=\"1.000\""));
    assert!(svg.contains("data-title-texture-opacity-application=\"precomposited-fill\""));
    assert!(svg.contains("data-visible-render-path-count=\"0\""));
    assert!(svg.contains("data-render-promoted=\"false\""));
    assert!(svg.contains(
        "data-render-promotion-blocked-reason=\"front-erase-texture-over-main-face-semantics-unproven\""
    ));
    assert!(svg.contains(
        "data-title-texture-source-paint-candidate-source=\"frontEraseTextureSourcePaintProbe\""
    ));
    assert!(svg.contains("data-title-texture-source-paint-candidate-color=\"#ffffff\""));
    assert!(svg.contains(
        "data-title-texture-source-paint-candidate-color-source=\"JSFart2Contents.paintColorCandidate\""
    ));
    assert!(svg.contains("data-title-texture-solid-paint-candidate-fill=\"#ffffff\""));
    assert!(svg.contains("data-title-texture-active-precomposited-fill=\"#818181\""));
    assert!(svg.contains("data-title-texture-solid-paint-render-promoted=\"false\""));
    assert!(svg.contains(
        "data-title-texture-solid-paint-render-promotion-blocked-reason=\"solid-source-paint-semantics-unproven\""
    ));
    assert!(svg.contains(
        "data-title-texture-span-coverage-source=\"embeddedPressExplicitTextureStateSpans\""
    ));
    assert!(svg.contains("data-title-texture-span-count=\"11\""));
    assert!(
        svg.contains("data-title-texture-span-path-counts=\"16,36,91,55,16,55,73,22,64,52,50\"")
    );
    assert!(svg.contains("data-title-texture-explicit-state-span-path-count=\"11\""));
    assert!(svg.contains("data-title-texture-inherited-span-path-count=\"519\""));
    assert!(svg.contains("data-title-texture-span-render-promoted=\"false\""));
    assert!(svg.contains(
        "data-title-texture-span-render-promotion-blocked-reason=\"span-density-and-clip-semantics-unproven\""
    ));
    assert!(svg.contains(
        "data-title-front-erase-transition-gate=\"embeddedPressVectorPathSourceOrder+stateTransitions\""
    ));
    assert!(svg.contains(
        "data-title-front-erase-transition-boundary-class=\"source-order-bracketed-interstitial-texture-block\""
    ));
    assert!(svg.contains(
        "data-title-front-erase-paint-intent-inference=\"shadow-state-texture-inside-main-boundary-ambiguous\""
    ));
    assert!(svg.contains("data-title-front-erase-transition-promotion-ready=\"false\""));
    assert!(svg.contains(
        "data-title-front-erase-transition-blocked-reason=\"front-erase-transition-boundary-main-state-not-separated\""
    ));
    assert!(svg.contains("data-title-front-erase-record48-separates-texture-from-main=\"false\""));
    assert!(
        svg.contains("data-title-front-erase-record70-word0-separates-texture-from-main=\"false\"")
    );
    assert!(
        svg.contains("data-title-front-erase-record82-word5-separates-texture-from-main=\"true\"")
    );
    assert!(svg.contains("data-title-front-erase-record82-word5-matches-shadow=\"true\""));
    assert!(svg.contains("data-title-front-erase-record82-word3-white-paint-candidate=\"true\""));
    assert!(svg.contains(
        "data-title-texture-direct-gray-candidate-source=\"embeddedPressRecord82Word5DirectGrayProbe\""
    ));
    assert!(svg.contains("data-title-texture-direct-gray-candidate-word5=\"0x2f\""));
    assert!(svg.contains("data-title-texture-direct-gray-candidate-fill=\"#787878\""));
    assert!(svg.contains("data-title-texture-direct-gray-render-promoted=\"false\""));
    assert!(svg.contains(
        "data-title-texture-direct-gray-render-promotion-blocked-reason=\"direct-gray-channel-probe-not-proven-as-paint-semantics\""
    ));
    assert!(
        !svg.contains(
            "data-title-texture-rendering=\"paint-state-color-front-erase-source-paths\""
        )
    );
    assert!(!svg.contains("data-title-texture-source=\"embedded-press-0x82-word3-white-paint\""));
    assert!(svg.contains("data-texture-path-count=\"530\""));
    assert!(svg.contains(
        "data-title-texture-opacity-source=\"embedded-press-front-erase-texture-opacity\""
    ));
    assert!(!svg.contains("data-title-texture-source=\"embedded-press-paint-state-0x82-word3\""));
    assert_eq!(
        svg.matches("data-frame-y-basis=\"topTextSourceGrid\"")
            .count(),
        4
    );
    assert!(!svg.contains("data-frame-y-basis=\"lineAnchorFallback\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-cone-diagram\""));
    assert!(svg.contains("data-text-corroboration-source=\"FDMText\""));
    assert!(svg.contains("data-text-corroboration-count=\"3\""));
    assert!(svg.contains("data-min-text-corroboration-count=\"2\""));
    assert!(svg.contains("data-source-left=\"-12004\" data-source-top=\"-12064\" data-source-right=\"-10720\" data-source-bottom=\"-10510\""));
    assert!(svg.contains("data-marker-hex=\"01000960\""));
    assert!(svg.contains("data-style-word=\"0x0120\""));
    assert!(svg.contains(
        "data-relative-offset=\"208\" data-source-vector-relative-offset=\"208\" data-source-segment-backed=\"false\""
    ));
    assert!(svg.contains("stroke-dasharray=\"4 4\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-fdm-reference-projection\""));
    assert!(svg.contains("data-role=\"q4-angle-diagrams\""));
    assert!(svg.contains("data-scale-mode=\"uniform-units-from-horizontal-span\""));
    assert!(svg.contains(
        "data-role=\"q4-angle-diagrams\" data-source-path=\"/FigureData/main_data/FDMVector\" data-projection=\"successDataTestFdmReferenceProjection\""
    ));
    assert!(svg.contains(
        "data-command-count=\"20\" data-source-provenance=\"fdm-vector-command\" data-source-ownership-basis=\"fdmVectorCommandProvenance\" data-source-ownership-proven=\"false\" data-source-ownership-promotion-blocked-reason=\"mixed-raw-and-segment-cohorts\" data-source-vector-offset-start=\"308\" data-source-vector-offset-end=\"1780\" data-command-relative-offsets=\"308,342,374,406,438,470,504,538,570,602,634,874,1048,1126,1158,1190,1430,1604,1730,1780\" data-source-vector-offset-command-count=\"20\" data-source-segment-backed-command-count=\"2\" data-source-raw-span-command-count=\"18\" data-source-segment-cohort-count=\"2\" data-source-segment-relative-offsets=\"690,1246\""
    ));
    assert!(svg.contains(
        "data-index-row-order-basis=\"fdm-index-row-reference-command-order\" data-index-row-order-decoded=\"false\" data-index-row-order-ownership-proven=\"false\" data-index-row-order-paint-order-decoded=\"false\""
    ));
    assert!(svg.contains("data-index-row-order-render-promotion-blocked-reason=\"fdm-index-row-order-valid-vector-offset-missing\" data-index-row-order-render-promotion-blocked-reasons=\"fdm-index-row-order-valid-vector-offset-missing,fdm-index-row-order-offset-namespace-mixed,role-paint-order-continuity-unproven\""));
    assert!(svg.contains(
        "data-index-row-order-command-count=\"20\" data-index-row-order-referenced-command-count=\"20\" data-index-row-order-unreferenced-command-count=\"0\" data-index-row-order-unique-row-index-count=\"20\" data-index-row-order-reference-count=\"20\""
    ));
    assert!(svg.contains(
        "data-index-row-order-command-relative-offset-field-reference-count=\"18\" data-index-row-order-source-segment-relative-offset-field-reference-count=\"2\" data-index-row-order-all-commands-referenced=\"true\" data-index-row-order-one-to-one-row-command=\"true\" data-index-row-order-single-row-backs-multiple-commands=\"false\" data-index-row-order-matches-command-order=\"true\""
    ));
    assert!(svg.contains(
        "data-render-command-order-basis=\"fdm-index-row-command-pairs\" data-render-command-order-promoted=\"true\" data-render-command-order-blocked-reason=\"\" data-render-command-count=\"20\" data-source-command-count=\"20\" data-render-command-relative-offsets=\"308,342,374,406,438,470,504,538,570,602,634,874,1048,1126,1158,1190,1430,1604,1730,1780\""
    ));
    assert!(svg.contains("class=\"rjtd-success-data-test-fdm-subdiagram\""));
    assert!(svg.contains("data-grouping-source=\"nearest-main-circle-source-center\""));
    assert!(svg.contains("data-grouping-decoded=\"false\""));
    assert!(svg.contains("data-paint-order-decoded=\"false\""));
    assert!(svg.contains(
        "data-subdiagram-index=\"0\" data-anchor-relative-offset=\"308\" data-anchor-source-x=\"-15184\" data-anchor-source-y=\"-9613\" data-command-count=\"7\" data-source-provenance=\"fdm-vector-command\" data-source-ownership-basis=\"fdmVectorCommandProvenance\" data-source-ownership-proven=\"false\" data-source-ownership-promotion-blocked-reason=\"source-owner-candidate-unproven\" data-source-vector-offset-start=\"308\" data-source-vector-offset-end=\"1604\" data-command-relative-offsets=\"308,342,374,406,438,1048,1604\" data-source-vector-offset-command-count=\"7\" data-source-segment-backed-command-count=\"0\" data-source-raw-span-command-count=\"7\" data-source-segment-cohort-count=\"0\" data-source-segment-relative-offsets=\"\""
    ));
    assert!(svg.contains(
        "data-subdiagram-index=\"1\" data-anchor-relative-offset=\"470\" data-anchor-source-x=\"-13184\" data-anchor-source-y=\"-9613\" data-command-count=\"6\" data-source-provenance=\"fdm-vector-command\" data-source-ownership-basis=\"fdmVectorCommandProvenance\" data-source-ownership-proven=\"false\" data-source-ownership-promotion-blocked-reason=\"mixed-raw-and-segment-cohorts\" data-source-vector-offset-start=\"470\" data-source-vector-offset-end=\"1780\" data-command-relative-offsets=\"470,1126,1158,1190,1430,1780\" data-source-vector-offset-command-count=\"6\" data-source-segment-backed-command-count=\"1\" data-source-raw-span-command-count=\"5\" data-source-segment-cohort-count=\"1\" data-source-segment-relative-offsets=\"1246\""
    ));
    assert!(svg.contains(
        "data-subdiagram-index=\"2\" data-anchor-relative-offset=\"504\" data-anchor-source-x=\"-11184\" data-anchor-source-y=\"-9613\" data-command-count=\"7\" data-source-provenance=\"fdm-vector-command\" data-source-ownership-basis=\"fdmVectorCommandProvenance\" data-source-ownership-proven=\"false\" data-source-ownership-promotion-blocked-reason=\"mixed-raw-and-segment-cohorts\" data-source-vector-offset-start=\"504\" data-source-vector-offset-end=\"1730\" data-command-relative-offsets=\"504,538,570,602,634,874,1730\" data-source-vector-offset-command-count=\"7\" data-source-segment-backed-command-count=\"1\" data-source-raw-span-command-count=\"6\" data-source-segment-cohort-count=\"1\" data-source-segment-relative-offsets=\"690\""
    ));
    assert!(svg.contains("data-render-source=\"q4-small-ellipse-angle-arc\""));
    assert!(svg.contains("data-render-source=\"q4-main-circle-center-nearby-line-endpoint\""));
    assert!(svg.contains("data-local-command-scope=\"source-main-circle-subdiagram\""));
    assert!(svg.contains("data-stroke-width-source=\"fdm-vector-style\""));
    assert!(svg.contains(
        "data-relative-offset=\"538\" data-source-vector-relative-offset=\"538\" data-source-segment-backed=\"false\" data-stroke-width-source=\"fdm-vector-style\" data-stroke-width=\"0.500\""
    ));
    assert!(svg.contains(
        "data-relative-offset=\"1992\" data-source-vector-relative-offset=\"1992\" data-source-segment-backed=\"true\" data-source-segment-relative-offset=\"1864\" data-source-segment-local-offset=\"128\" data-source-segment-declared-length=\"236\" data-source-segment-command-count=\"4\" data-source-segment-command-index=\"2\" data-source-segment-command-offset=\"128\" data-stroke-width-source=\"fdm-vector-style\" data-stroke-width=\"0.500\""
    ));
    assert!(svg.contains("data-role=\"q5-solid-diagram\""));
    assert!(svg.contains(
        "data-command-count=\"7\" data-source-provenance=\"fdm-vector-command\" data-source-ownership-basis=\"fdmVectorCommandProvenance\" data-source-ownership-proven=\"false\" data-source-ownership-promotion-blocked-reason=\"mixed-raw-and-segment-cohorts\" data-source-vector-offset-start=\"1830\" data-source-vector-offset-end=\"2190\" data-command-relative-offsets=\"1830,1924,1958,1992,2024,2156,2190\" data-source-vector-offset-command-count=\"7\" data-source-segment-backed-command-count=\"6\" data-source-raw-span-command-count=\"1\" data-source-segment-cohort-count=\"2\" data-source-segment-relative-offsets=\"1864,2100\""
    ));
    assert!(svg.contains(
        "data-index-row-order-command-count=\"7\" data-index-row-order-referenced-command-count=\"7\" data-index-row-order-unreferenced-command-count=\"0\" data-index-row-order-unique-row-index-count=\"3\" data-index-row-order-reference-count=\"7\""
    ));
    assert!(svg.contains(
        "data-index-row-order-command-relative-offset-field-reference-count=\"1\" data-index-row-order-source-segment-relative-offset-field-reference-count=\"6\" data-index-row-order-all-commands-referenced=\"true\" data-index-row-order-one-to-one-row-command=\"false\" data-index-row-order-single-row-backs-multiple-commands=\"true\" data-index-row-order-matches-command-order=\"true\""
    ));
    assert!(svg.contains(
        "data-render-command-order-basis=\"fdm-vector-projection-filter-order\" data-render-command-order-promoted=\"false\" data-render-command-order-blocked-reason=\"fdm-index-row-fanout-primitive-ownership-unproven\" data-render-command-count=\"7\" data-source-command-count=\"7\" data-render-command-relative-offsets=\"1830,1924,1958,1992,2024,2156,2190\""
    ));
    assert!(svg.contains("class=\"rjtd-success-data-test-fdm-text-projection\""));
    assert!(svg.contains("data-projection=\"successDataTestFdmTextProjection\""));
    assert!(svg.contains("data-source-path=\"/FigureData/main_data/FDMText\""));
    assert!(svg.contains("data-role=\"q3-cone-diagram\""));
    assert!(svg.contains("data-text-count=\"3\""));
    assert!(svg.contains("data-text-count=\"9\""));
    assert!(svg.contains("class=\"rjtd-success-data-test-fdm-text\""));
    assert!(svg.contains("data-role=\"q4-angle-diagrams\""));
    assert!(svg.contains("font-size=\"10.7\""));
    assert!(svg.contains(">９㎝</text>"));
    assert!(svg.contains(">３㎝</text>"));
    assert!(svg.contains(">110°</text>"));
    assert!(svg.contains(">30°</text>"));
    assert!(svg.contains(">160°</text>"));
    assert!(svg.contains(">ｘ</text>"));
    assert!(svg.contains(">Ｏ</text>"));
    assert!(svg.contains("data-source-left=\"-11578\" data-source-top=\"-8778\" data-source-right=\"-10796\" data-source-bottom=\"-7643\""));
    assert!(svg.contains("data-marker-hex=\"01000460\""));
    assert!(svg.contains("data-marker-hex=\"01000160\""));
    assert!(svg.contains("class=\"rjtd-jseq-formula-projection\""));
    assert!(svg.contains("data-projection=\"jseqFormulaPathProjection\""));
    assert!(svg.contains("data-vector-bearing-source=\"jseq3TextRunContextCellMetric\""));
    assert!(svg.contains("data-vector-bearing-cell-unit=\"148.00\""));
    assert!(svg.contains("data-vector-bearing-dx=\"6.22\""));
    assert!(svg.contains("data-vector-bearing-dy=\"2.33\""));
    assert!(svg.contains("data-vector-path-stroke-source-unit=\"37.00\""));
    assert!(svg.contains("data-vector-path-stroke-width=\"0.47\""));
    assert!(svg.contains("data-vector-segment-count=\"51\""));
    assert!(svg.contains("class=\"rjtd-jseq-formula-segments\""));
    assert!(svg.contains("class=\"rjtd-jseq-formula-segment\""));
    assert!(svg.contains("data-rendered-segment-count=\""));
    assert!(svg.contains("class=\"rjtd-jseq-formula-path\""));
    assert!(svg.contains("class=\"rjtd-jseq-formula-text-projection\""));
    assert!(svg.contains("data-projection=\"jseqFormulaTextTokenProjection\""));
    assert!(svg.contains("class=\"rjtd-jseq-formula-text\""));
    assert!(svg.contains(">１２</text>"));
    assert!(svg.contains(">÷</text>"));
    assert!(svg.contains(">１２８</text>"));
    assert!(svg.contains(">－２(</text>"));
    assert!(svg.contains(">＋３)</text>"));
    assert!(svg.contains(">－５)</text>"));
    assert!(!svg.contains(">－２（</text>"));
    assert!(!svg.contains(">＋３）</text>"));
    assert!(!svg.contains(">－５）</text>"));
    assert!(!svg.contains("class=\"rjtd-embedding-frame-diagnostic\""));
    assert!(!svg.contains("data-source-path=\"/EmbedItems/EmbeddingInfo\""));
    assert!(!svg.contains("data-class-name=\"JSEQ.Document.3\""));
    assert!(!svg.contains("data-linked-jseq3-formula=\"true\""));
    assert!(!svg.contains("class=\"rjtd-embedded-press-snapshot-vector\""));
    assert!(!svg.contains("data-projection=\"embeddedPressSnapshotVectorProjection\""));
}

#[test]
fn local_success_data_test_reports_source_backed_fdm_text_mirror_anchor_agreement() {
    let sample_path =
        local_samples_dir().join("ichitaro-20030228030923-success-002-success_data-test.jtd");
    if !sample_path.exists() {
        return;
    }

    let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
    let document_info = DocumentCore::from_document(document).get_document_info();

    assert!(document_info.contains("\"fdmTextMirrorAnchorAgreements\":[{"));
    assert!(
        document_info
            .contains("\"indexedTextPath\":\"/FigureData/ExpandData/main_data/Data/FDMText\"")
    );
    assert!(document_info.contains("\"mirroredTextPath\":\"/FigureData/main_data/FDMText\""));
    assert!(document_info.contains("\"textRecordCount\":15"));
    assert!(document_info.contains("\"orderedTextAgreement\":true"));
    assert!(document_info.contains("\"orderedRecordBboxAgreement\":true"));
    assert!(document_info.contains("\"indexedRecordOffsetAgreement\":true"));
    assert!(document_info.contains("\"indexedRecordBboxAgreement\":true"));
    assert!(document_info.contains("\"sourceToPageTransformDecoded\":false"));
    assert!(document_info.contains(
        "\"renderPromotionBlockedReason\":\"fdmtext-source-to-page-transform-undecoded\""
    ));
}

#[test]
fn parser_links_fdm_index_rows_to_fdm_vector_segments() {
    let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    index_payload[18..20].copy_from_slice(&2u16.to_be_bytes());
    push_fdm_index_row(&mut index_payload, 0, 0x1001, (-1, 2, 3, 4));
    push_fdm_index_row(&mut index_payload, 32, 0x2002, (-10, -20, 30, 40));

    let mut vector_payload = vec![0xaa; 32];
    vector_payload.extend_from_slice(b"lead");
    let image_offset = vector_payload.len();
    vector_payload.extend_from_slice(minimal_jpeg_payload());
    let vector_len = vector_payload.len();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/FigureData/main_data/FDMIndex", &index_payload),
        ("/FigureData/main_data/FDMVector", &vector_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    let vector_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
        .unwrap();
    assert_eq!(vector_candidate.fdm_index_entry_candidates().len(), 2);

    let first = &vector_candidate.fdm_index_entry_candidates()[0];
    assert_eq!(first.index_path(), "/FigureData/main_data/FDMIndex");
    assert_eq!(first.vector_path(), "/FigureData/main_data/FDMVector");
    assert_eq!(first.row_index(), 0);
    assert_eq!(first.index_offset(), FDM_INDEX_HEADER_BYTES);
    assert_eq!(first.vector_offset(), 0);
    assert_eq!(first.next_vector_offset(), 32);
    assert_eq!(first.vector_len(), 32);
    assert_eq!(first.kind(), 0x1001);
    assert_eq!(first.bbox(), ObjectFdmIndexBbox::new(-1, 2, 3, 4));
    assert!(first.valid_vector_offset());
    assert!(first.image_signature_hits().is_empty());
    assert!(first.segment_image_signature_hits().is_empty());

    let second = &vector_candidate.fdm_index_entry_candidates()[1];
    assert_eq!(second.row_index(), 1);
    assert_eq!(
        second.index_offset(),
        FDM_INDEX_HEADER_BYTES + FDM_INDEX_ENTRY_BYTES
    );
    assert_eq!(second.vector_offset(), 32);
    assert_eq!(second.next_vector_offset(), vector_len);
    assert_eq!(second.vector_len(), vector_len - 32);
    assert_eq!(second.kind(), 0x2002);
    assert_eq!(second.bbox(), ObjectFdmIndexBbox::new(-10, -20, 30, 40));
    assert!(second.valid_vector_offset());
    assert!(second.vector_prefix().starts_with(b"lead\xff\xd8\xff"));
    assert_eq!(second.image_signature_hits()[0].kind(), "jpeg");
    assert_eq!(second.image_signature_hits()[0].offset(), image_offset);
    assert_eq!(second.segment_image_signature_hits()[0].kind(), "jpeg");
    assert_eq!(second.segment_image_signature_hits()[0].offset(), 4);

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"fdmIndexEntries\":["));
    assert!(info.contains("\"indexPath\":\"/FigureData/main_data/FDMIndex\""));
    assert!(info.contains("\"kindHex\":\"0x2002\""));
    assert!(info.contains("\"bbox\":{\"left\":-10,\"top\":-20,\"right\":30,\"bottom\":40}"));
    assert!(info.contains("\"segmentImageSignatures\":[{\"kind\":\"jpeg\",\"offset\":4}]"));

    let overlay_images = core.get_page_overlay_images(0).unwrap();
    assert!(overlay_images.contains("\"imageCount\":0"));
    assert!(overlay_images.contains("\"unplacedDiagnostics\":["));
    assert!(overlay_images.contains("\"type\":\"jtdFdmVectorImageCandidate\""));
    assert!(overlay_images.contains("\"sourcePath\":\"/FigureData/main_data/FDMVector\""));
    assert!(overlay_images.contains("\"indexPath\":\"/FigureData/main_data/FDMIndex\""));
    assert!(overlay_images.contains("\"rowIndex\":1"));
    assert!(
        overlay_images
            .contains("\"normalizedBbox\":{\"left\":-10,\"top\":-20,\"right\":30,\"bottom\":40}")
    );
    assert!(overlay_images.contains("\"bboxPlausible\":true"));
    assert!(overlay_images.contains("\"completePayloads\":1"));
    assert!(overlay_images.contains("\"placementProven\":false"));
    assert!(overlay_images.contains("\"renderable\":false"));
}

#[test]
fn parser_links_root_fdm_index_to_nested_fdm_vector_by_content_score() {
    let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    index_payload[18..20].copy_from_slice(&2u16.to_be_bytes());
    push_fdm_index_row(&mut index_payload, 0, 0x1001, (1, 2, 3, 4));
    push_fdm_index_row(&mut index_payload, 32, 0x2002, (10, 20, 30, 40));

    let mut decoy_index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    decoy_index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    decoy_index_payload[18..20].copy_from_slice(&1u16.to_be_bytes());
    push_fdm_index_row(
        &mut decoy_index_payload,
        0xffff_fff0,
        0x9999,
        (-1, -2, -3, -4),
    );

    let mut vector_payload = vec![0xaa; 32];
    vector_payload.extend_from_slice(b"lead");
    vector_payload.extend_from_slice(minimal_jpeg_payload());
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/FDMIndex", &index_payload),
        ("/FigureData/other/FDMIndex", &decoy_index_payload),
        ("/FigureData/main_data/FDMVector", &vector_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    let vector_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
        .unwrap();
    assert_eq!(vector_candidate.fdm_index_entry_candidates().len(), 2);
    let first = &vector_candidate.fdm_index_entry_candidates()[0];
    assert_eq!(first.index_path(), "/FDMIndex");
    assert_eq!(first.vector_path(), "/FigureData/main_data/FDMVector");
    assert_eq!(first.vector_offset(), 0);
    assert_eq!(first.kind(), 0x1001);
    assert!(first.valid_vector_offset());
    let second = &vector_candidate.fdm_index_entry_candidates()[1];
    assert_eq!(second.index_path(), "/FDMIndex");
    assert_eq!(second.kind(), 0x2002);
    assert!(second.vector_prefix().starts_with(b"lead\xff\xd8\xff"));

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"indexPath\":\"/FDMIndex\""));
    assert!(info.contains("\"vectorPath\":\"/FigureData/main_data/FDMVector\""));
    assert!(!info.contains("\"indexPath\":\"/FigureData/other/FDMIndex\""));
}

#[test]
fn document_core_projects_shanai_lan_fdm_frame_diagnostics() {
    let jpeg_payload = minimal_jpeg_payload();
    let mut vector_payload = Vec::new();
    let mut offsets = Vec::new();
    for row_index in 0..34 {
        offsets.push(vector_payload.len() as u32);
        if row_index == 23 || row_index == 33 {
            vector_payload.extend_from_slice(jpeg_payload);
        } else {
            vector_payload.extend_from_slice(&[0xaa, 0xbb, 0xcc, 0xdd]);
        }
    }

    let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    index_payload[18..20].copy_from_slice(&(offsets.len() as u16).to_be_bytes());
    for (row_index, vector_offset) in offsets.into_iter().enumerate() {
        let bbox = if row_index == 23 {
            (0, 0, 2238, 1843)
        } else if row_index == 33 {
            (0, 0, 1310, 618)
        } else {
            (0, 0, 1, 1)
        };
        push_fdm_index_row(&mut index_payload, vector_offset, 0x0b00, bbox);
    }

    let mut frame_payload = vec![
        0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x01, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x02,
    ];
    frame_payload.extend_from_slice(&frame_record_fixture(23, 0x0003, (14435, 402, 2238, 1843)));
    frame_payload.extend_from_slice(&frame_record_fixture(33, 0x0024, (10985, 127, 1310, 618)));

    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture_for("社内LAN構成図")),
        ("/FigureData/main_data/FDMIndex", &index_payload),
        ("/FigureData/main_data/FDMVector", &vector_payload),
        ("/Frame", &frame_payload),
    ]);
    let mut core = DocumentCore::from_bytes(&bytes).unwrap();
    core.set_file_name("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"fdmFrameDiagnostic\""));
    assert!(layer_tree.contains("\"source\":\"fdmIndex+frame\""));
    assert!(layer_tree.contains("\"projectionKind\":\"fdmFrameDiagnosticProjection\""));
    assert!(layer_tree.contains("\"referenceBacked\":true"));
    assert!(layer_tree.contains("\"placementProven\":false"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"fdm-frame-linked-image-payload-placement-and-paint-order-unproven\""
    ));
    assert!(
        layer_tree
            .contains("\"imagePayloadExtractionStatus\":\"complete-payload-in-fdm-index-segment\"")
    );
    assert!(layer_tree.contains("\"rowIndex\":23"));
    assert!(layer_tree.contains("\"objectTypeHex\":\"0x0003\""));
    assert!(layer_tree.contains("\"bbox\":{\"x\":601.469,\"y\":402.000,\"width\":93.252"));
    assert!(layer_tree.contains("\"rowIndex\":33"));
    assert!(layer_tree.contains("\"objectTypeHex\":\"0x0024\""));
    assert!(layer_tree.contains("\"bbox\":{\"x\":457.716,\"y\":127.000,\"width\":54.584"));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-fdm-frame-diagnostics\""));
    assert!(svg.contains("data-projection=\"fdmFrameDiagnosticProjection\""));
    assert!(svg.contains(
        "data-image-payload-extraction-status=\"complete-payload-in-fdm-index-segment\""
    ));
    assert!(
        svg.contains("data-render-promotion-blocked-reason=\"fdm-frame-linked-image-payload-placement-and-paint-order-unproven\"")
    );
    assert!(svg.contains("data-row-index=\"23\""));
    assert!(svg.contains("data-row-index=\"33\""));
    assert!(svg.contains("FDM row 23"));
    assert!(svg.contains("FDM row 33"));
}

#[test]
fn local_shanai_lan_preserves_fdm_frame_diagnostics_when_reference_pdf_is_available() {
    let sample_path =
        local_samples_dir().join("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");
    let reference_pdf_path = sample_path.with_extension("pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let document = parse_document(&bytes).unwrap();
    let command_diagnostics = fdm_command_diagnostics(&document);
    assert!(command_diagnostics.iter().any(|diagnostic| {
        !diagnostic.entry.segment_image_signature_hits().is_empty()
            && fdm_command_diagnostic_svg_style(*diagnostic).basis
                == "fdm-index-segment-image-signature"
    }));
    assert!(command_diagnostics.iter().any(|diagnostic| {
        diagnostic.entry.segment_image_signature_hits().is_empty()
            && diagnostic.entry.image_signature_hits().is_empty()
            && fdm_command_diagnostic_svg_style(*diagnostic).basis
                == "fdm-index-command-diagnostic-default"
    }));
    let mut renamed_core = DocumentCore::from_document(document.clone());
    renamed_core.file_name = "renamed-lan-diagram.jtd".to_string();
    renamed_core.refresh_pages();
    assert!((renamed_core.page_width_px() - 1122.5).abs() < 0.2);
    assert!((renamed_core.page_height_px() - 793.7).abs() < 0.2);
    assert_eq!(renamed_core.page_count(), 1);
    assert!(
        renamed_core
            .get_page_layer_tree(0)
            .unwrap()
            .contains("\"type\":\"fdmVectorPrimitiveProjection\"")
    );

    let mut core = DocumentCore::from_document(document);
    core.set_file_name(sample_path.to_string_lossy());

    assert!((core.page_width_px() - 1122.5).abs() < 0.2);
    assert!((core.page_height_px() - 793.7).abs() < 0.2);
    assert_eq!(core.page_count(), 1);

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"type\":\"fdmFrameDiagnostic\""));
    assert!(layer_tree.contains("\"source\":\"fdmIndex+frame\""));
    assert!(layer_tree.contains("\"projectionKind\":\"fdmFrameDiagnosticProjection\""));
    assert!(layer_tree.contains("\"referenceBacked\":true"));
    assert!(layer_tree.contains("\"placementProven\":false"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"image-signature-without-complete-payload-role-unproven\""
    ));
    assert!(
        layer_tree
            .contains("\"imagePayloadExtractionStatus\":\"signature-without-complete-payload\"")
    );
    assert!(layer_tree.contains("\"rowIndex\":23"));
    assert!(layer_tree.contains("\"rowIndex\":33"));
    assert!(layer_tree.contains("\"objectTypeHex\":\"0x0003\""));
    assert!(layer_tree.contains("\"objectTypeHex\":\"0x0024\""));
    assert!(layer_tree.contains("\"bbox\":{\"x\":601.469,\"y\":402.000,\"width\":93.252"));
    assert!(layer_tree.contains("\"bbox\":{\"x\":457.716,\"y\":127.000,\"width\":54.584"));
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmVectorCommandDiagnostic\"")
            .count(),
        334
    );
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmVectorPrimitiveProjection\"")
            .count(),
        1889
    );
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmProjectionExtentSummary\"")
            .count(),
        1
    );
    assert!(layer_tree.contains("\"projectionKind\":\"fdmProjectionExtentSummary\""));
    assert!(layer_tree.contains("\"activeRenderExtentBasis\":\"fdmVectorCommandBboxExtent\""));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"fdm-vector-page-placement-transform-source-fields-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"commandDiagnosticCount\":334,\"renderedPrimitiveDiagnosticCount\":1889,\"fdmIndexEntryCount\":39"
    ));
    assert!(layer_tree.contains(
        "\"activeCommandExtent\":{\"left\":-16154,\"top\":-16224,\"right\":-5612,\"bottom\":-9344,\"spanX\":10542,\"spanY\":6880}"
    ));
    assert!(layer_tree.contains(
        "\"renderedPrimitiveExtent\":{\"left\":-16154,\"top\":-16224,\"right\":-5453,\"bottom\":-9344,\"spanX\":10701,\"spanY\":6880}"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexEntryExtent\":{\"left\":-16154,\"top\":-16224,\"right\":-5576,\"bottom\":-9336,\"spanX\":10578,\"spanY\":6888}"
    ));
    assert!(layer_tree.contains(
        "\"extentAgreement\":{\"commandMatchesRenderedPrimitives\":false,\"commandMatchesFdmIndexEntries\":false,\"renderedPrimitivesMatchFdmIndexEntries\":false}"
    ));
    assert!(layer_tree.contains(
        "\"extentResiduals\":{\"commandVsRenderedPrimitives\":{\"leftDelta\":0,\"topDelta\":0,\"rightDelta\":159,\"bottomDelta\":0,\"maxAbsDelta\":159},\"commandVsFdmIndexEntries\":{\"leftDelta\":0,\"topDelta\":0,\"rightDelta\":36,\"bottomDelta\":8,\"maxAbsDelta\":36},\"renderedPrimitivesVsFdmIndexEntries\":{\"leftDelta\":0,\"topDelta\":0,\"rightDelta\":-123,\"bottomDelta\":8,\"maxAbsDelta\":123}}"
    ));
    assert!(
        layer_tree
            .matches("\"type\":\"fdmConnectorCandidateDiagnostic\"")
            .count()
            >= 40
    );
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmConnectorCandidateDiagnostic\"")
            .count(),
        67
    );
    assert!(layer_tree.contains("\"source\":\"fdmVectorCommandConnectorCandidate\""));
    assert!(layer_tree.contains("\"projectionKind\":\"fdmOpenPathConnectorCandidateProjection\""));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"connector-ownership-grouping-and-paint-order-unproven\""
    ));
    assert!(layer_tree.contains("\"candidateBasis\":\"long-open-endpoint-path\""));
    assert!(layer_tree.contains("\"geometryDecoded\":true"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains("\"sourceEndpoints\":{\"start\":{\"x\":"));
    assert!(layer_tree.contains("\"projectedEndpoints\":{\"start\":{\"x\":"));
    assert!(layer_tree.contains(
        "\"parentCompoundCommand\":{\"basis\":\"synthetic-nested-command-index+relative-offset\""
    ));
    assert!(layer_tree.contains("\"parentMarkerHex\":\"ff000a60\""));
    assert!(layer_tree.contains("\"parentCompoundChildOffsets\":[48]"));
    assert!(layer_tree.contains("\"childOffsetInParent\":48"));
    assert!(layer_tree.contains("\"childOffsetTableMatched\":true"));
    assert!(layer_tree.contains(
        "\"projectedTextGrid\":{\"basis\":\"documentTextLineHeaderGrid\",\"start\":{\"xUnits\":"
    ));
    assert!(layer_tree.contains("\"groupIndexFloat\":"));
    assert_eq!(
        layer_tree.matches("\"endpointOwnerCandidates\"").count(),
        67
    );
    assert!(layer_tree.contains(
        "\"endpointOwnerCandidates\":{\"basis\":\"fdmPrimitiveProjection+documentTextGroupLineProjection\""
    ));
    assert!(layer_tree.contains("\"ownershipProven\":false"));
    assert!(layer_tree.contains("\"probeRadiusPx\":18.000"));
    assert!(layer_tree.contains("\"candidateLimit\":3"));
    assert!(layer_tree.contains("\"kind\":\"fdmPrimitive\""));
    assert!(layer_tree.contains("\"kind\":\"documentTextSlot\""));
    assert!(layer_tree.contains("\"ownerProven\":false"));
    assert!(layer_tree.contains("\"sourceBacked\":true"));
    assert!(layer_tree.contains("\"withinProbeRadius\":true"));
    assert!(layer_tree.contains("\"sourceByteRange\":{\"start\":"));
    assert!(layer_tree.contains("\"sourceUnitRange\":{\"start\":"));
    assert_eq!(
        layer_tree.matches("\"endpointOwnerMatchSummary\"").count(),
        67
    );
    assert!(layer_tree.contains("\"startCandidateCount\":3"));
    assert!(layer_tree.contains("\"endCandidateCount\":3"));
    assert!(layer_tree.contains("\"totalCandidateCount\":6"));
    assert!(layer_tree.contains("\"dualEndpointOwnerCandidate\":true"));
    assert!(layer_tree.contains("\"dualEndpointOwnerCandidate\":false"));
    assert!(layer_tree.contains("\"connectorParentCommandIndex\":"));
    assert!(layer_tree.contains("\"connectorSyntheticNestedCommand\":false"));
    assert!(layer_tree.contains("\"connectorRelativeOffset\":"));
    assert!(layer_tree.contains("\"connectorRelativeOffset\":1246"));
    assert!(layer_tree.contains("\"startNearestFdmOwner\":{\"rowIndex\":"));
    assert!(layer_tree.contains("\"endNearestFdmOwner\":{\"rowIndex\":"));
    assert!(layer_tree.contains("\"parentCommandIndex\":"));
    assert!(layer_tree.contains("\"syntheticNestedCommand\":true"));
    assert!(layer_tree.contains(
        "\"startNearestFdmOwner\":{\"rowIndex\":23,\"commandIndex\":4001,\"parentCommandIndex\":4,\"syntheticNestedCommand\":true,\"relativeOffset\":988}"
    ));
    assert!(layer_tree.contains("\"nearestFdmOwnerRowsMatch\":true"));
    assert!(layer_tree.contains("\"nearestFdmOwnerRowsMatch\":false"));
    assert!(layer_tree.contains("\"nearestFdmOwnerRowMatchesConnectorRow\":true"));
    assert!(layer_tree.contains("\"mixedTopLevelVsNestedOrderNamespace\":true"));
    assert!(layer_tree.contains("\"mixedTopLevelVsNestedOrderNamespace\":false"));
    assert!(layer_tree.contains("\"connectorCommandBetweenNearestFdmOwnerCommands\":true"));
    assert!(layer_tree.contains("\"connectorCommandBetweenNearestFdmOwnerCommands\":false"));
    assert!(layer_tree.contains("\"connectorCommandBeforeNearestFdmOwnerCommands\":true"));
    assert!(layer_tree.contains("\"connectorCommandBeforeNearestFdmOwnerCommands\":false"));
    assert!(layer_tree.contains("\"connectorCommandAfterNearestFdmOwnerCommands\":true"));
    assert!(layer_tree.contains("\"connectorCommandAfterNearestFdmOwnerCommands\":false"));
    assert!(layer_tree.contains("\"connectorRelativeOffsetBetweenNearestFdmOwnerOffsets\":false"));
    assert!(layer_tree.contains("\"connectorRelativeOffsetBeforeNearestFdmOwnerOffsets\":true"));
    assert!(layer_tree.contains("\"connectorRelativeOffsetBeforeNearestFdmOwnerOffsets\":false"));
    assert!(layer_tree.contains("\"connectorRelativeOffsetAfterNearestFdmOwnerOffsets\":true"));
    assert!(layer_tree.contains("\"connectorRelativeOffsetAfterNearestFdmOwnerOffsets\":false"));
    assert!(layer_tree.contains("\"connectorParentRelativeOffset\":"));
    assert!(layer_tree.contains("\"startNearestFdmOwnerParentRelativeOffset\":"));
    assert!(layer_tree.contains("\"endNearestFdmOwnerParentRelativeOffset\":"));
    assert!(layer_tree.contains("\"connectorParentCommandBetweenNearestFdmOwnerParentCommands\":"));
    assert!(
        layer_tree.contains("\"connectorParentRelativeOffsetAfterNearestFdmOwnerParentOffsets\":")
    );
    assert!(layer_tree.contains("\"ownerParentCommandRelation\":"));
    assert!(layer_tree.contains("\"ownerParentSourceOrderRelation\":"));
    assert!(layer_tree.contains("\"parentNormalizedOrderedSameRowSameConnector\":"));
    assert!(layer_tree.contains("\"ownerGroupingProven\":false"));
    assert!(
        layer_tree
            .contains("\"ownerGroupingPromotionBlockedReason\":\"owner-row-candidate-unproven\"")
    );
    assert!(
        layer_tree
            .contains("\"ownerGroupingPromotionBlockedReason\":\"nearest-owner-row-mismatch\"")
    );
    assert!(
        layer_tree
            .contains("\"ownershipPromotionBlockedReason\":\"endpoint-owner-candidate-unproven\"")
    );
    assert!(
        layer_tree
            .contains("\"ownershipPromotionBlockedReason\":\"missing-endpoint-owner-candidate\"")
    );
    assert!(
        layer_tree
            .contains("\"lineRuleAttachmentCandidates\":{\"basis\":\"documentTextLineHeaderGrid\"")
    );
    assert!(layer_tree.contains("\"nearestLineRule\":{\"ruleIndex\":"));
    assert!(layer_tree.contains("\"distanceGrid\":"));
    assert!(layer_tree.contains("\"axisDelta\":"));
    assert!(layer_tree.contains("\"inlineDelta\":"));
    assert!(layer_tree.contains("\"closestPoint\":{\"xUnits\":"));
    assert!(
        layer_tree
            .contains("\"lineRuleEndpointMatches\":{\"basis\":\"documentTextLineHeaderGrid\"")
    );
    assert!(layer_tree.contains("\"tier\":\"tight\""));
    assert!(layer_tree.contains("\"perpendicularGroupDelta\":"));
    assert!(layer_tree.contains("\"spanOverflowUnits\":"));
    assert!(layer_tree.contains("\"inSpanAxis\":"));
    assert!(layer_tree.contains("\"lineRuleEndpointMatchSummary\":{\"startMatchCount\":"));
    assert!(layer_tree.contains("\"matchedEndpointCount\":"));
    assert!(layer_tree.contains("\"dualEndpointMatch\":false"));
    assert!(layer_tree.contains(
        "\"graphPromotionBlockedReason\":\"single-or-missing-endpoint-line-rule-match\""
    ));
    assert_eq!(
        layer_tree
            .matches("\"sameRowFdmOpenStrokeAxisRuleEndpointMatches\"")
            .count(),
        67
    );
    assert_eq!(
        layer_tree
            .matches("\"sameRowFdmOpenStrokeAxisRuleEndpointMatchSummary\"")
            .count(),
        67
    );
    assert_eq!(
        layer_tree
            .matches("\"sameRowFdmOpenStrokeAxisRuleOwnerPromotionGate\"")
            .count(),
        67
    );
    assert!(layer_tree.contains(
        "\"sameRowFdmOpenStrokeAxisRuleEndpointMatches\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+documentTextLineHeaderGrid\""
    ));
    assert!(layer_tree.contains(
        "\"sameRowFdmOpenStrokeAxisRuleEndpointMatchSummary\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule\",\"source\":\"fdmVectorCommandPrimitive\""
    ));
    assert!(layer_tree.contains(
        "\"sameRowFdmOpenStrokeAxisRuleOwnerPromotionGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+endpointOwnerMatchSummary\",\"decoded\":false,\"sourceBacked\":true,\"renderable\":false"
    ));
    assert!(layer_tree.contains("\"axisRuleDualEndpointMatch\":true"));
    assert!(layer_tree.contains("\"axisRuleMatchedEndpointCount\":2"));
    assert!(layer_tree.contains("\"ownerCommandRelation\":\"same-row-mixed-command-namespace\""));
    assert!(
        layer_tree
            .contains("\"ownerSourceOrderRelation\":\"same-row-after-owner-relative-offset-span\"")
    );
    assert!(layer_tree.contains("\"parentNormalizedOrderedSameRowSameConnector\":false"));
    assert!(layer_tree.contains(
        "\"sameRowFdmOpenStrokeAxisRuleEndpointMatchSummary\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule\",\"source\":\"fdmVectorCommandPrimitive\",\"startMatchCount\":"
    ));
    assert!(layer_tree.contains("\"axisRuleIndex\":"));
    assert!(layer_tree.contains("\"ruleCommandIndex\":"));
    assert!(layer_tree.contains("\"ruleRelativeOffset\":"));
    assert!(layer_tree.contains("\"sameRowAsConnector\":true"));
    assert!(layer_tree.contains(
        "\"dualEndpointMatch\":true,\"graphPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\""
    ));
    assert_eq!(
        layer_tree
            .matches("\"type\":\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTrace\"")
            .count(),
        1
    );
    assert!(layer_tree.contains("\"projectionKind\":\"fdmConnectorSourceOrderTrace\""));
    assert!(layer_tree.contains(
        "\"basis\":\"sameRowFdmOpenStrokeAxisRule+tightDualEndpoint+nonDiagonalConnector+sourceOrderTrace\""
    ));
    assert!(layer_tree.contains(
        "\"selectionPredicate\":{\"requiresTightDualEndpointAxisRuleMatch\":true,\"excludesDiagonalConnectors\":true,\"rowHardcoded\":false}"
    ));
    assert!(layer_tree.contains(
        "\"summary\":{\"basis\":\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTrace+relationCounts\""
    ));
    assert!(layer_tree.contains("\"promotionReady\":false"));
    assert!(
        layer_tree
            .contains("\"readinessBlockedReason\":\"image-signature-fragment-role-unproven\"")
    );
    assert!(layer_tree.contains("\"sourceSegmentMatchesIndexEntryCount\":"));
    assert!(layer_tree.contains("\"entryConnectorCandidateCount\":"));
    assert!(layer_tree.contains("\"imageBearingSegmentCount\":"));
    assert!(layer_tree.contains("\"imageBearingCompletePayloadSegmentCount\":0"));
    assert!(layer_tree.contains("\"imageBearingSignatureWithoutPayloadSegmentCount\":5"));
    assert!(layer_tree.contains("\"parentNormalizedOrderedSameRowSameConnectorCount\":0"));
    assert!(layer_tree.contains(
        "\"bboxRelationCounts\":{\"contained\":16,\"overlaps\":0,\"disjoint\":0,\"missing\":0}"
    ));
    assert!(layer_tree.contains(
        "\"imageBearingBboxRelationCounts\":{\"contained\":5,\"overlaps\":0,\"disjoint\":0,\"missing\":0}"
    ));
    assert!(layer_tree.contains(
        "\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTraceSummary\":{\"basis\":\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTrace+relationCounts\""
    ));
    assert!(layer_tree.contains(
        "\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTraceSummary\":{\"basis\":\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTrace+relationCounts\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"promotionReady\":false,\"readinessBlockedReason\":\"image-signature-fragment-role-unproven\",\"traceCount\":16"
    ));
    assert!(
        layer_tree.contains(
            "\"pagePaintCoverageSummary\":{\"basis\":\"fdmVectorPrimitivePaintCoverage\""
        )
    );
    assert!(layer_tree.contains("\"largeSpanFilteredPrimitiveCount\":"));
    assert!(layer_tree.contains("\"closedFillPrimitiveCount\":"));
    assert!(layer_tree.contains("\"pageFillCandidateCount\":"));
    assert!(layer_tree.contains("\"maxPageCoverageRatio\":"));
    assert!(layer_tree.contains("\"pageFillSpanFilterMaxPageRatio\":0.280000"));
    assert!(!layer_tree.contains("\"pageFillCandidate\":true"));
    assert!(layer_tree.contains("\"connectorVsAxisRuleParentSpanCounts\":{"));
    assert!(layer_tree.contains(
        "\"imageBearingConnectorVsAxisRuleParentSpanCounts\":{\"before\":4,\"between\":0,\"after\":1,\"missing\":0}"
    ));
    assert!(layer_tree.contains(
        "\"imageBearingConnectorVsSegmentImageSignatureRangeCounts\":{\"before\":0,\"inside\":4,\"after\":1,\"missing\":0}"
    ));
    assert!(layer_tree.contains("\"ownerVsAxisRuleParentSpanCounts\":{"));
    assert_eq!(
        layer_tree.matches("\"imageBearingSegmentGate\"").count(),
        16
    );
    assert!(layer_tree.contains(
        "\"imageBearingSegmentGate\":{\"basis\":\"FDMIndex.imageSignature+FDMVector.connectorBbox+sameRowAxisRuleParentSpan\""
    ));
    assert!(layer_tree.contains(
        "\"imageBearingSegmentGate\":{\"basis\":\"FDMIndex.imageSignature+FDMVector.connectorBbox+sameRowAxisRuleParentSpan\",\"source\":\"FDMIndex.segmentImageSignatures+FDMVector.commandSourceBbox\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"image-signature-without-complete-payload-role-unproven\",\"imageBearingSegmentCandidate\":true,\"connectorParent\":{\"commandIndex\":"
    ));
    assert!(layer_tree.contains("\"axisRuleParentRelativeOffsetRange\":"));
    assert!(layer_tree.contains("\"endpointOwnerParentRelativeOffsetRange\":"));
    assert!(
        layer_tree.contains("\"endpointOwnerParentRelations\":{\"connectorVsOwnerParentSpan\":")
    );
    assert!(layer_tree.contains("\"endpointOwnerParentRelationToAxisRuleParentSpan\":"));
    assert!(layer_tree.contains("\"segmentImageSignatureCommandContexts\":["));
    assert!(layer_tree.contains(
        "{\"kind\":\"jpeg\",\"offset\":2453,\"relationToTraceConnectorCommand\":\"before-command-record\",\"containingCommandCount\":2"
    ));
    assert!(layer_tree.contains(
        "\"commandIndex\":10001,\"relativeOffset\":2414,\"recordEnd\":2598,\"recordLength\":184,\"declaredRecordLength\":184,\"offsetInCommand\":39,\"markerHex\":\"ff000960\",\"primitiveKind\":\"cubicBezier\",\"styleWordHex\":\"0x80e5\",\"syntheticNestedCommand\":true,\"sameAsTraceConnector\":false"
    ));
    assert!(layer_tree.contains(
        "{\"kind\":\"jpeg\",\"offset\":4389,\"relationToTraceConnectorCommand\":\"after-command-record\",\"containingCommandCount\":1"
    ));
    assert!(layer_tree.contains(
        "{\"kind\":\"jpeg\",\"offset\":5957,\"relationToTraceConnectorCommand\":\"after-command-record\",\"containingCommandCount\":1"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"image-signature-without-complete-payload-role-unproven\""
    ));
    assert!(
        layer_tree.contains("\"payloadExtractionStatus\":\"signature-without-complete-payload\"")
    );
    assert!(layer_tree.contains(
        "\"connectorVsSegmentImageSignatureRange\":\"connector-inside-segment-image-signature-range\""
    ));
    assert!(layer_tree.contains(
        "\"connectorVsSegmentImageSignatureRange\":\"connector-after-segment-image-signature-range\""
    ));
    assert!(
        layer_tree.contains(
            "\"connectorVsImageSignatureRange\":\"connector-inside-image-signature-range\""
        )
    );
    assert!(
        layer_tree.contains(
            "\"connectorVsImageSignatureRange\":\"connector-after-image-signature-range\""
        )
    );
    assert!(layer_tree.contains("\"nearestSegmentImageSignatureDistance\":99"));
    assert!(layer_tree.contains("\"nearestImageSignatureDistance\":99"));
    assert!(layer_tree.contains("\"completeImagePayloadSpanCount\":0"));
    assert!(layer_tree.contains("\"segmentCompleteImagePayloadSpanCount\":0"));
    assert!(layer_tree.contains("\"imageSignatureOffsetRange\":{\"start\":144737,\"end\":148241}"));
    assert!(
        layer_tree.contains("\"segmentImageSignatureOffsetRange\":{\"start\":2453,\"end\":5957}")
    );
    assert!(layer_tree.contains("\"sourceOrderNodes\":["));
    assert!(layer_tree.contains("\"role\":\"connector\""));
    assert!(layer_tree.contains("\"role\":\"nearestFdmOwner\""));
    assert!(layer_tree.contains("\"role\":\"axisRule\""));
    assert!(layer_tree.contains("\"fdmIndexRow\":{\"rowIndex\":"));
    assert!(layer_tree.contains("\"indexOffset\":"));
    assert!(layer_tree.contains("\"kindHex\":"));
    assert!(layer_tree.contains("\"entryConnectorCandidateCount\":"));
    assert!(layer_tree.contains("\"connector\":{\"commandIndex\":"));
    assert!(layer_tree.contains("\"sourceSegmentMatchesIndexEntry\":true"));
    assert!(layer_tree.contains("\"entryConnectorCandidate\":true"));
    assert!(layer_tree.contains("\"axisRuleMatches\":{\"start\":["));
    assert!(layer_tree.contains("\"ruleParentRelativeOffset\":"));
    assert!(layer_tree.contains("\"relations\":{\"connectorVsOwnerParentSpan\":"));
    assert!(layer_tree.contains("\"connectorVsAxisRuleParentSpan\":"));
    assert!(layer_tree.contains("\"ownerParentSpanVsAxisRuleParentSpan\":"));
    assert!(layer_tree.contains("\"bboxRelationToFdmIndex\":"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"projected-endpoint-straight-line-paint-order-and-ownership-unproven\""
    ));
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmConnectorGraphDiagnosticSummary\"")
            .count(),
        1
    );
    assert!(layer_tree.contains("\"projectionKind\":\"fdmConnectorGraphDiagnosticSummary\""));
    assert!(layer_tree.contains(
        "\"source\":\"fdmVectorCommandConnectorCandidate+documentTextLineRuleProjection\""
    ));
    assert!(
        layer_tree
            .contains("\"renderPromotionBlockedReason\":\"connector-parent-command-outside-nearest-owner-parent-command-span\"")
    );
    assert!(layer_tree.contains("\"connectorCandidateCount\":67"));
    assert!(layer_tree.contains("\"lineRuleProjectionCount\":16"));
    assert!(layer_tree.contains("\"fdmOpenStrokeAxisRuleProjectionCount\":224"));
    assert!(layer_tree.contains("\"connectorEndpointProbeCount\":134"));
    assert!(layer_tree.contains("\"totalThresholdedEndpointMatchCount\":10"));
    assert!(layer_tree.contains("\"matchedConnectorCount\":9"));
    assert!(layer_tree.contains("\"dualEndpointMatchConnectorCount\":0"));
    assert!(layer_tree.contains("\"startEndpointLineRuleMatchConnectorCount\":4"));
    assert!(layer_tree.contains("\"endEndpointLineRuleMatchConnectorCount\":5"));
    assert!(layer_tree.contains("\"startOnlyLineRuleMatchConnectorCount\":4"));
    assert!(layer_tree.contains("\"endOnlyLineRuleMatchConnectorCount\":5"));
    assert!(layer_tree.contains("\"tightEndpointMatchCount\":2"));
    assert!(layer_tree.contains("\"nearbyEndpointMatchCount\":8"));
    assert!(layer_tree.contains("\"noThresholdedLineRuleEndpointMatchConnectorCount\":58"));
    assert!(layer_tree.contains("\"singleOrMissingEndpointLineRuleMatchConnectorCount\":9"));
    assert!(layer_tree.contains("\"connectorOwnershipAndPaintOrderUnprovenConnectorCount\":0"));
    assert!(layer_tree.contains("\"endpointOwnerCandidateConnectorCount\":67"));
    assert!(layer_tree.contains("\"endpointOwnerProbeCount\":134"));
    assert!(layer_tree.contains("\"totalEndpointOwnerCandidateCount\":402"));
    assert!(layer_tree.contains("\"withinProbeEndpointOwnerCandidateCount\":372"));
    assert!(layer_tree.contains("\"fdmPrimitiveEndpointOwnerCandidateCount\":401"));
    assert!(layer_tree.contains("\"documentTextSlotEndpointOwnerCandidateCount\":1"));
    assert!(layer_tree.contains("\"startEndpointOwnerWithinProbeConnectorCount\":67"));
    assert!(layer_tree.contains("\"endEndpointOwnerWithinProbeConnectorCount\":66"));
    assert!(layer_tree.contains("\"dualEndpointOwnerWithinProbeConnectorCount\":66"));
    assert!(layer_tree.contains("\"ownerProvenConnectorCount\":0"));
    assert!(layer_tree.contains("\"dualEndpointNearestFdmOwnerSameRowConnectorCount\":65"));
    assert!(layer_tree.contains("\"dualEndpointNearestFdmOwnerRowMismatchConnectorCount\":1"));
    assert!(layer_tree.contains("\"dualEndpointNearestFdmOwnerSameConnectorRowCount\":65"));
    assert!(layer_tree.contains("\"connectorCommandBetweenNearestFdmOwnerCommandsCount\":8"));
    assert!(layer_tree.contains("\"connectorCommandBeforeNearestFdmOwnerCommandsCount\":43"));
    assert!(layer_tree.contains("\"connectorCommandAfterNearestFdmOwnerCommandsCount\":15"));
    assert!(layer_tree.contains("\"orderedSameRowSameConnectorCount\":0"));
    assert!(layer_tree.contains("\"parentNormalizedOrderedSameRowSameConnectorCount\":"));
    assert!(layer_tree.contains("\"missingEndpointOwnerCandidateConnectorCount\":1"));
    assert!(layer_tree.contains("\"nearestOwnerRowMismatchConnectorCount\":1"));
    assert!(layer_tree.contains("\"ownerRowCandidateUnprovenConnectorCount\":65"));
    assert!(layer_tree.contains("\"ownerGroupingProvenConnectorCount\":0"));
    assert!(layer_tree.contains("\"lineRuleEndpointMatchProvenanceSummaries\":["));
    assert!(layer_tree.contains("\"ruleSet\":\"allDocumentTextLineRules\""));
    assert!(layer_tree.contains("\"ruleSet\":\"skippedInlineLineHeaderOnly\""));
    assert!(layer_tree.contains("\"candidateSource\":\"skippedInlineLineHeader\""));
    assert!(
        layer_tree.contains(
            "\"ruleSet\":\"verticalAnchorRunFromLineHeadersOnly\",\"candidateSource\":\"verticalAnchorRunFromLineHeaders\",\"lineRuleProjectionCount\":9"
        )
    );
    assert!(
        layer_tree.contains(
            "\"ruleSet\":\"skippedInlineLineHeaderOnly\",\"candidateSource\":\"skippedInlineLineHeader\",\"lineRuleProjectionCount\":7,\"connectorCandidateCount\":67,\"connectorEndpointProbeCount\":134,\"totalThresholdedEndpointMatchCount\":9"
        )
    );
    assert!(
        layer_tree.contains(
            "\"ruleSet\":\"verticalAnchorRunFromLineHeadersOnly\",\"candidateSource\":\"verticalAnchorRunFromLineHeaders\",\"lineRuleProjectionCount\":9,\"connectorCandidateCount\":67,\"connectorEndpointProbeCount\":134,\"totalThresholdedEndpointMatchCount\":1"
        )
    );
    assert!(
        layer_tree.contains(
            "\"ruleSet\":\"sameRowFdmOpenStrokeAxisRules\",\"candidateSource\":\"fdmOpenStrokeAxisRule\",\"lineRuleProjectionCount\":224,\"connectorCandidateCount\":67,\"connectorEndpointProbeCount\":134,\"totalThresholdedEndpointMatchCount\":254"
        )
    );
    assert!(layer_tree.contains(
        "\"ruleSet\":\"sameRowFdmOpenStrokeAxisRules\",\"candidateSource\":\"fdmOpenStrokeAxisRule\",\"lineRuleProjectionCount\":224,\"connectorCandidateCount\":67,\"connectorEndpointProbeCount\":134,\"totalThresholdedEndpointMatchCount\":254,\"matchedConnectorCount\":52,\"dualEndpointMatchConnectorCount\":21,\"tightEndpointMatchCount\":170,\"nearbyEndpointMatchCount\":84"
    ));
    assert!(layer_tree.contains(
        "\"ruleSet\":\"sameRowFdmOpenStrokeAxisRules\",\"candidateSource\":\"fdmOpenStrokeAxisRule\",\"lineRuleProjectionCount\":224"
    ));
    assert!(
        layer_tree
            .contains("{\"reason\":\"connector-ownership-and-paint-order-unproven\",\"count\":21}")
    );
    assert!(layer_tree.contains(
        "\"sameRowFdmOpenStrokeAxisRuleRowCohorts\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+connectorRowIndex\""
    ));
    assert!(layer_tree.contains("\"rowCohortLimit\":16,\"rowCohortCount\":11"));
    assert!(layer_tree.contains(
        "\"renderReadinessPredicate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+tightDualEndpoint+nonDiagonalConnector\""
    ));
    assert!(layer_tree.contains(
        "\"requiresTightDualEndpointMatch\":true,\"excludesDiagonalConnectors\":true,\"requiresDualEndpointOwnerCandidate\":true"
    ));
    assert!(layer_tree.contains(
        "\"candidateCount\":16,\"rowCohortCount\":6,\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"rowCohorts\":[{\"rowIndex\":33,\"connectorCandidateCount\":14,\"totalThresholdedEndpointMatchCount\":33,\"matchedConnectorCount\":14,\"fdmIndexSegmentGate\":"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexSegmentGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexSegmentMembership\",\"source\":\"FDMIndex.vectorOffset+FDMVector.sourceSegment\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-image-bearing-segment-role-unproven\",\"rowIndex\":33,\"vectorOffset\":142284,\"vectorLength\":7222,\"validVectorOffset\":true,\"sourceSegmentRelativeOffset\":142284,\"sourceSegmentCommandCount\":64,\"imageSignatureCount\":3,\"segmentImageSignatureCount\":3,\"imageBearingSegmentCandidate\":true,\"sourceSegmentBackedConnectorCount\":14,\"sourceSegmentMatchesIndexEntryConnectorCount\":14,\"sourceSegmentMissingConnectorCount\":0,\"dualEndpointSourceSegmentBackedConnectorCount\":7,\"dualEndpointSourceSegmentMatchesIndexEntryConnectorCount\":7,\"dualEndpointImageBearingSegmentConnectorCount\":7}"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexConnectorCompositionGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexConnectorComposition\",\"source\":\"FDMIndex.vectorCommands+FDMIndex.connectorCandidates\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-image-bearing-segment-role-unproven\",\"rowIndex\":33,\"vectorCommandCount\":85,\"connectorCandidateCount\":14,\"nonConnectorCommandCount\":71,\"rowCohortConnectorCandidateCount\":14,\"connectorOnlySegmentCandidate\":false,\"connectorDominantSegmentCandidate\":false,\"connectorCandidateDensityPermille\":164,\"matchedConnectorCoveragePermille\":1000,\"dualEndpointMatchedConnectorCoveragePermille\":500}"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexBboxRelationGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexBboxRelation\",\"source\":\"FDMIndex.bbox+FDMVector.commandSourceBbox\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-image-bearing-segment-role-unproven\",\"rowIndex\":33,\"indexBbox\":{\"left\":-9590,\"top\":-10344,\"right\":-8517,\"bottom\":-9336},\"containsConnectorCount\":14,\"overlapsConnectorCount\":0,\"disjointConnectorCount\":0,\"sourceBboxMissingConnectorCount\":0,\"dualEndpointContainsConnectorCount\":7,\"dualEndpointOverlapsConnectorCount\":0,\"dualEndpointDisjointConnectorCount\":0,\"dualEndpointSourceBboxMissingConnectorCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"axisRuleSourceOrderGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+parentRelativeSourceOrder\",\"source\":\"FDMVector.commandRelativeOffset+compoundParentRelativeOffset\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"mixed-connector-axis-rule-parent-span-paint-order-unproven\",\"rowIndex\":33,\"dualEndpointConnectorCount\":7,\"sourceOrderBackedDualEndpointConnectorCount\":7,\"connectorParentRelativeOffsetRange\":{\"start\":2638,\"end\":6056},\"axisRuleParentRelativeOffsetRange\":{\"start\":3894,\"end\":5886},\"connectorBeforeAxisRuleParentSpanCount\":5,\"connectorBetweenAxisRuleParentSpanCount\":1,\"connectorAfterAxisRuleParentSpanCount\":1,\"connectorAxisRuleParentSpanUnclassifiedCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexSegmentGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexSegmentMembership\",\"source\":\"FDMIndex.vectorOffset+FDMVector.sourceSegment\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-segment-ownership-and-paint-order-unproven\",\"rowIndex\":3,\"vectorOffset\":26954"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexConnectorCompositionGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexConnectorComposition\",\"source\":\"FDMIndex.vectorCommands+FDMIndex.connectorCandidates\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-contained-composition-internal-stroke-role-unproven\",\"rowIndex\":3,\"vectorCommandCount\":84,\"connectorCandidateCount\":5,\"nonConnectorCommandCount\":79,\"rowCohortConnectorCandidateCount\":5,\"connectorOnlySegmentCandidate\":false,\"connectorDominantSegmentCandidate\":false,\"connectorCandidateDensityPermille\":59,\"matchedConnectorCoveragePermille\":1000,\"dualEndpointMatchedConnectorCoveragePermille\":1000}"
    ));
    assert!(layer_tree.contains(
        "\"axisRuleSourceOrderGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+parentRelativeSourceOrder\",\"source\":\"FDMVector.commandRelativeOffset+compoundParentRelativeOffset\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"mixed-connector-axis-rule-parent-span-paint-order-unproven\",\"rowIndex\":3,\"dualEndpointConnectorCount\":5,\"sourceOrderBackedDualEndpointConnectorCount\":5,\"connectorParentRelativeOffsetRange\":{\"start\":1138,\"end\":5242},\"axisRuleParentRelativeOffsetRange\":{\"start\":4058,\"end\":5640},\"connectorBeforeAxisRuleParentSpanCount\":2,\"connectorBetweenAxisRuleParentSpanCount\":3,\"connectorAfterAxisRuleParentSpanCount\":0,\"connectorAxisRuleParentSpanUnclassifiedCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexConnectorCompositionGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexConnectorComposition\",\"source\":\"FDMIndex.vectorCommands+FDMIndex.connectorCandidates\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-contained-composition-internal-stroke-role-unproven\",\"rowIndex\":0,\"vectorCommandCount\":271,\"connectorCandidateCount\":11,\"nonConnectorCommandCount\":260,\"rowCohortConnectorCandidateCount\":24,\"connectorOnlySegmentCandidate\":false,\"connectorDominantSegmentCandidate\":false,\"connectorCandidateDensityPermille\":40,\"matchedConnectorCoveragePermille\":1000,\"dualEndpointMatchedConnectorCoveragePermille\":166}"
    ));
    assert!(layer_tree.contains(
        "\"axisRuleSourceOrderGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+parentRelativeSourceOrder\",\"source\":\"FDMVector.commandRelativeOffset+compoundParentRelativeOffset\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"connector-between-axis-rule-parent-span-paint-order-unproven\",\"rowIndex\":0,\"dualEndpointConnectorCount\":4,\"sourceOrderBackedDualEndpointConnectorCount\":4,\"connectorParentRelativeOffsetRange\":{\"start\":7644,\"end\":9734},\"axisRuleParentRelativeOffsetRange\":{\"start\":3460,\"end\":12022},\"connectorBeforeAxisRuleParentSpanCount\":0,\"connectorBetweenAxisRuleParentSpanCount\":4,\"connectorAfterAxisRuleParentSpanCount\":0,\"connectorAxisRuleParentSpanUnclassifiedCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexBboxRelationGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexBboxRelation\",\"source\":\"FDMIndex.bbox+FDMVector.commandSourceBbox\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-bbox-contained-internal-stroke-role-unproven\",\"rowIndex\":3,\"indexBbox\":{\"left\":-9342,\"top\":-15790,\"right\":-8384,\"bottom\":-15057},\"containsConnectorCount\":5,\"overlapsConnectorCount\":0,\"disjointConnectorCount\":0,\"sourceBboxMissingConnectorCount\":0,\"dualEndpointContainsConnectorCount\":5,\"dualEndpointOverlapsConnectorCount\":0,\"dualEndpointDisjointConnectorCount\":0,\"dualEndpointSourceBboxMissingConnectorCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexConnectorCompositionGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexConnectorComposition\",\"source\":\"FDMIndex.vectorCommands+FDMIndex.connectorCandidates\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-contained-composition-internal-stroke-role-unproven\",\"rowIndex\":37,\"vectorCommandCount\":48,\"connectorCandidateCount\":4,\"nonConnectorCommandCount\":44,\"rowCohortConnectorCandidateCount\":4,\"connectorOnlySegmentCandidate\":false,\"connectorDominantSegmentCandidate\":false,\"connectorCandidateDensityPermille\":83,\"matchedConnectorCoveragePermille\":1000,\"dualEndpointMatchedConnectorCoveragePermille\":750}"
    ));
    assert!(layer_tree.contains(
        "\"axisRuleSourceOrderGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+parentRelativeSourceOrder\",\"source\":\"FDMVector.commandRelativeOffset+compoundParentRelativeOffset\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"mixed-connector-axis-rule-parent-span-paint-order-unproven\",\"rowIndex\":37,\"dualEndpointConnectorCount\":3,\"sourceOrderBackedDualEndpointConnectorCount\":3,\"connectorParentRelativeOffsetRange\":{\"start\":4352,\"end\":5782},\"axisRuleParentRelativeOffsetRange\":{\"start\":4790,\"end\":5750},\"connectorBeforeAxisRuleParentSpanCount\":2,\"connectorBetweenAxisRuleParentSpanCount\":0,\"connectorAfterAxisRuleParentSpanCount\":1,\"connectorAxisRuleParentSpanUnclassifiedCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"axisRuleSourceOrderGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+parentRelativeSourceOrder\",\"source\":\"FDMVector.commandRelativeOffset+compoundParentRelativeOffset\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"connector-before-axis-rule-parent-span-paint-order-unproven\",\"rowIndex\":10,\"dualEndpointConnectorCount\":1,\"sourceOrderBackedDualEndpointConnectorCount\":1,\"connectorParentRelativeOffsetRange\":{\"start\":1298,\"end\":1298},\"axisRuleParentRelativeOffsetRange\":{\"start\":1376,\"end\":2056},\"connectorBeforeAxisRuleParentSpanCount\":1,\"connectorBetweenAxisRuleParentSpanCount\":0,\"connectorAfterAxisRuleParentSpanCount\":0,\"connectorAxisRuleParentSpanUnclassifiedCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"fdmIndexBboxRelationGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexBboxRelation\",\"source\":\"FDMIndex.bbox+FDMVector.commandSourceBbox\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"fdm-index-bbox-contained-internal-stroke-role-unproven\",\"rowIndex\":37,\"indexBbox\":{\"left\":-12762,\"top\":-10144,\"right\":-11614,\"bottom\":-9488},\"containsConnectorCount\":4,\"overlapsConnectorCount\":0,\"disjointConnectorCount\":0,\"sourceBboxMissingConnectorCount\":0,\"dualEndpointContainsConnectorCount\":3,\"dualEndpointOverlapsConnectorCount\":0,\"dualEndpointDisjointConnectorCount\":0,\"dualEndpointSourceBboxMissingConnectorCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"sourceVectorRelativeOffset\":144922,\"sourceSegment\":{\"relativeOffset\":142284,\"localOffset\":2638,\"declaredLength\":7222,\"commandCount\":64,\"commandIndex\":11,\"commandOffset\":2638}"
    ));
    assert!(layer_tree.contains(
        "\"matchedProjectedBboxUnion\":{\"x\":682.702,\"y\":618.639,\"width\":102.306,\"height\":97.362}"
    ));
    assert!(layer_tree.contains(
        "\"dualEndpointMatchConnectorCount\":7,\"dualEndpointProjectedBboxUnion\":{\"x\":684.639,\"y\":618.639,\"width\":97.850,\"height\":97.362}"
    ));
    assert!(layer_tree.contains("\"nonDiagonalDualEndpointMatchConnectorCount\":5"));
    assert!(layer_tree.contains(
        "\"tightDualEndpointMatchConnectorCount\":6,\"nonDiagonalTightDualEndpointMatchConnectorCount\":5"
    ));
    assert!(layer_tree.contains(
        "\"tightNonDiagonalDualEndpointProjectedBboxUnion\":{\"x\":685.802,\"y\":622.380,\"width\":85.255,\"height\":93.621}"
    ));
    assert!(layer_tree.contains(
        "\"horizontalDualEndpointMatchConnectorCount\":4,\"verticalDualEndpointMatchConnectorCount\":1,\"diagonalDualEndpointMatchConnectorCount\":2"
    ));
    assert!(layer_tree.contains(
        "\"horizontalTightDualEndpointMatchConnectorCount\":4,\"verticalTightDualEndpointMatchConnectorCount\":1,\"diagonalTightDualEndpointMatchConnectorCount\":1"
    ));
    assert!(layer_tree.contains(
        "\"matchedConnectorMarkerStyleProfile\":{\"basis\":\"fdm-vector-marker+style-word\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"commandCount\":14"
    ));
    assert!(layer_tree.contains(
        "\"dualConnectorMarkerStyleProfile\":{\"basis\":\"fdm-vector-marker+style-word\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"commandCount\":7,\"markerFamilyCounts\":[{\"reason\":\"line-marker\",\"count\":1},{\"reason\":\"path-marker\",\"count\":6}"
    ));
    assert!(layer_tree.contains(
        "\"tightNonDiagonalDualConnectorMarkerStyleProfile\":{\"basis\":\"fdm-vector-marker+style-word\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"commandCount\":5,\"markerFamilyCounts\":[{\"reason\":\"line-marker\",\"count\":1},{\"reason\":\"path-marker\",\"count\":4}"
    ));
    assert!(layer_tree.contains(
        "\"axisRuleEndpointMatchMarkerStyleProfile\":{\"basis\":\"fdm-vector-marker+style-word\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"commandCount\":33,\"markerFamilyCounts\":[{\"reason\":\"line-marker\",\"count\":17},{\"reason\":\"path-marker\",\"count\":16}"
    ));
    assert!(layer_tree.contains(
        "\"styleWordCounts\":[{\"reason\":\"0x0000\",\"count\":0},{\"reason\":\"0x0005\",\"count\":17},{\"reason\":\"0x0080\",\"count\":5},{\"reason\":\"0x00a0\",\"count\":7},{\"reason\":\"other-style\",\"count\":4}]"
    ));
    assert!(layer_tree.contains(
        "\"roleGate\":{\"basis\":\"fdm-vector-marker-style-profile\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"markerFamilyDiversityCount\":2,\"styleWordDiversityCount\":4,\"dominantMarkerFamily\":\"line-marker\",\"dominantMarkerFamilyCount\":17,\"dominantStyleWord\":\"0x0005\",\"dominantStyleWordCount\":17,\"markerFamilyHomogeneous\":false,\"styleWordHomogeneous\":false,\"homogeneousMarkerStyleCandidate\":false,\"renderPromotionBlockedReason\":\"mixed-marker-family-and-style-word-role-unproven\"}"
    ));
    assert!(layer_tree.contains(
        "\"markerStyleAgreementGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+markerStyleAgreement\",\"source\":\"FDMVector.marker+styleWord\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false"
    ));
    assert!(layer_tree.contains("\"markerStyleAgreementCandidate\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"connector-axis-rule-marker-and-style-dominance-mismatch\""
    ));
    assert!(layer_tree.contains(
        "\"ownerPromotionGate\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+endpointOwnerMatchSummary+parentNormalizedOrderGate\""
    ));
    assert!(layer_tree.contains(
        "\"axisRuleDualEndpointMatchConnectorCount\":7,\"dualEndpointOwnerCandidateCount\":7,\"nearestFdmOwnerRowsMatchCount\":7,\"nearestFdmOwnerRowMatchesConnectorRowCount\":7,\"mixedTopLevelVsNestedOrderNamespaceCount\":7"
    ));
    assert!(layer_tree.contains(
        "\"ownerParentCommandRelationCounts\":[{\"reason\":\"missing-endpoint-owner-candidate\",\"count\":0},{\"reason\":\"nearest-owner-row-mismatch\",\"count\":0},{\"reason\":\"nearest-owner-row-not-connector-row\",\"count\":0},{\"reason\":\"same-row-before-owner-parent-command-span\",\"count\":0},{\"reason\":\"same-row-between-owner-parent-command-span\",\"count\":0},{\"reason\":\"same-row-after-owner-parent-command-span\",\"count\":7},{\"reason\":\"same-row-parent-command-relation-unclassified\",\"count\":0}]"
    ));
    assert!(layer_tree.contains(
        "\"ownerParentSourceOrderRelationCounts\":[{\"reason\":\"missing-endpoint-owner-candidate\",\"count\":0},{\"reason\":\"nearest-owner-row-mismatch\",\"count\":0},{\"reason\":\"nearest-owner-row-not-connector-row\",\"count\":0},{\"reason\":\"same-row-before-owner-parent-relative-offset-span\",\"count\":0},{\"reason\":\"same-row-between-owner-parent-relative-offset-span\",\"count\":0},{\"reason\":\"same-row-after-owner-parent-relative-offset-span\",\"count\":7},{\"reason\":\"same-row-parent-relative-offset-relation-unclassified\",\"count\":0}]"
    ));
    assert!(layer_tree.contains(
        "\"axisRuleDualEndpointMatchConnectorCount\":5,\"dualEndpointOwnerCandidateCount\":5,\"nearestFdmOwnerRowsMatchCount\":5,\"nearestFdmOwnerRowMatchesConnectorRowCount\":5,\"mixedTopLevelVsNestedOrderNamespaceCount\":4"
    ));
    assert!(layer_tree.contains(
        "\"rowIndex\":3,\"connectorCandidateCount\":5,\"totalThresholdedEndpointMatchCount\":57"
    ));
    assert!(layer_tree.contains(
        "\"axisRuleEndpointMatchMarkerStyleProfile\":{\"basis\":\"fdm-vector-marker+style-word\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"commandCount\":57,\"markerFamilyCounts\":[{\"reason\":\"line-marker\",\"count\":43},{\"reason\":\"path-marker\",\"count\":14}"
    ));
    assert!(layer_tree.contains(
        "\"dominantMarkerFamily\":\"line-marker\",\"dominantMarkerFamilyCount\":43,\"dominantStyleWord\":\"0x0005\",\"dominantStyleWordCount\":43,\"markerFamilyHomogeneous\":false,\"styleWordHomogeneous\":false,\"homogeneousMarkerStyleCandidate\":false,\"renderPromotionBlockedReason\":\"mixed-marker-family-and-style-word-role-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"sameRowFdmOpenStrokeAxisRuleOwnerPromotionGateSummary\":{\"basis\":\"sameRowFdmOpenStrokeAxisRule+endpointOwnerMatchSummary+parentNormalizedOrderGate\""
    ));
    assert!(layer_tree.contains(
        "\"axisRuleDualEndpointMatchConnectorCount\":21,\"dualEndpointOwnerCandidateCount\":21,\"nearestFdmOwnerRowsMatchCount\":21,\"nearestFdmOwnerRowMatchesConnectorRowCount\":21,\"mixedTopLevelVsNestedOrderNamespaceCount\":18"
    ));
    assert!(layer_tree.contains(
        "\"parentNormalizedOrderGateBlockedReason\":\"connector-parent-command-outside-nearest-owner-parent-command-span\""
    ));
    assert!(layer_tree.contains("\"parentNormalizedOrderedSameRowSameConnectorCount\":0"));
    assert!(layer_tree.contains(
        "\"ownerParentCommandRelationCounts\":[{\"reason\":\"missing-endpoint-owner-candidate\",\"count\":0},{\"reason\":\"nearest-owner-row-mismatch\",\"count\":0},{\"reason\":\"nearest-owner-row-not-connector-row\",\"count\":0},{\"reason\":\"same-row-before-owner-parent-command-span\",\"count\":1},{\"reason\":\"same-row-between-owner-parent-command-span\",\"count\":0},{\"reason\":\"same-row-after-owner-parent-command-span\",\"count\":20},{\"reason\":\"same-row-parent-command-relation-unclassified\",\"count\":0}]"
    ));
    assert!(layer_tree.contains(
        "\"ownerParentSourceOrderRelationCounts\":[{\"reason\":\"missing-endpoint-owner-candidate\",\"count\":0},{\"reason\":\"nearest-owner-row-mismatch\",\"count\":0},{\"reason\":\"nearest-owner-row-not-connector-row\",\"count\":0},{\"reason\":\"same-row-before-owner-parent-relative-offset-span\",\"count\":1},{\"reason\":\"same-row-between-owner-parent-relative-offset-span\",\"count\":0},{\"reason\":\"same-row-after-owner-parent-relative-offset-span\",\"count\":20},{\"reason\":\"same-row-parent-relative-offset-relation-unclassified\",\"count\":0}]"
    ));
    assert!(layer_tree.contains("\"ownerRowCohortEndpointMatchSummaries\":["));
    assert!(layer_tree.contains("\"parentNormalizedOwnerRowCohortEndpointMatchSummaries\":["));
    assert!(layer_tree.contains("\"cohort\":\"parentNormalizedOrderedSameRowSameConnector\""));
    assert!(
        layer_tree.contains(
            "\"cohort\":\"orderedSameRowSameConnector\",\"connectorCandidateCount\":0,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"cohort\":\"notOrderedSameRowSameConnector\",\"connectorCandidateCount\":67,\"totalThresholdedEndpointMatchCount\":10"
        )
    );
    assert!(layer_tree.contains("\"ownerCommandRelation\":\"same-row-mixed-command-namespace\""));
    assert!(layer_tree.contains("\"ownerCommandRelation\":\"same-row-before-owner-command-span\""));
    assert!(layer_tree.contains("\"ownerCommandRelation\":\"same-row-after-owner-command-span\""));
    assert!(
        layer_tree
            .contains("\"ownerSourceOrderRelation\":\"same-row-after-owner-relative-offset-span\"")
    );
    assert!(
        layer_tree.contains(
            "\"ownerSourceOrderRelation\":\"same-row-before-owner-relative-offset-span\""
        )
    );
    assert!(layer_tree.contains("\"ownerCommandRelationEndpointMatchSummaries\":["));
    assert!(
        layer_tree.contains(
            "\"relation\":\"missing-endpoint-owner-candidate\",\"connectorCandidateCount\":1,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"nearest-owner-row-mismatch\",\"connectorCandidateCount\":1,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"same-row-mixed-command-namespace\",\"connectorCandidateCount\":49,\"totalThresholdedEndpointMatchCount\":10,\"matchedConnectorCount\":9"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"same-row-before-owner-command-span\",\"connectorCandidateCount\":1,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"same-row-between-owner-command-span\",\"connectorCandidateCount\":0,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"same-row-after-owner-command-span\",\"connectorCandidateCount\":15,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(layer_tree.contains("\"ownerSourceOrderRelationEndpointMatchSummaries\":["));
    assert!(
        layer_tree.contains(
            "\"relation\":\"missing-endpoint-owner-candidate\",\"connectorCandidateCount\":1,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"nearest-owner-row-mismatch\",\"connectorCandidateCount\":1,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"same-row-before-owner-relative-offset-span\",\"connectorCandidateCount\":1,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"same-row-between-owner-relative-offset-span\",\"connectorCandidateCount\":0,\"totalThresholdedEndpointMatchCount\":0"
        )
    );
    assert!(
        layer_tree.contains(
            "\"relation\":\"same-row-after-owner-relative-offset-span\",\"connectorCandidateCount\":64,\"totalThresholdedEndpointMatchCount\":10,\"matchedConnectorCount\":9"
        )
    );
    assert!(layer_tree.contains(
        "\"ownerGroupingPromotionBlockedReasonCounts\":[{\"reason\":\"missing-endpoint-owner-candidate\",\"count\":1"
    ));
    assert!(layer_tree.contains("{\"reason\":\"nearest-owner-row-mismatch\",\"count\":1}"));
    assert!(layer_tree.contains("{\"reason\":\"owner-row-candidate-unproven\",\"count\":65}"));
    assert!(layer_tree.contains(
        "\"graphPromotionBlockedReasonCounts\":[{\"reason\":\"no-thresholded-line-rule-endpoint-match\",\"count\":58"
    ));
    assert!(
        layer_tree
            .contains("{\"reason\":\"single-or-missing-endpoint-line-rule-match\",\"count\":9}")
    );
    assert!(
        layer_tree
            .contains("{\"reason\":\"connector-ownership-and-paint-order-unproven\",\"count\":0}")
    );
    assert!(
        layer_tree.contains(
            "\"dominantMatchedConnectorRow\":{\"basis\":\"fdmConnectorCandidateRowIndex+lineRuleEndpointMatchSummary\",\"rowIndex\":33,\"connectorCandidateCount\":14,\"totalThresholdedEndpointMatchCount\":8,\"matchedConnectorCount\":7,\"dualEndpointMatchConnectorCount\":0,\"startOnlyLineRuleMatchConnectorCount\":2,\"endOnlyLineRuleMatchConnectorCount\":5,\"tightEndpointMatchCount\":2,\"nearbyEndpointMatchCount\":6,\"renderPromotionBlockedReason\":\"dominant-row-still-lacks-dual-endpoint-line-rule-match\"}"
        )
    );
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmOpenStrokeCohortSummary\"")
            .count(),
        1
    );
    assert!(layer_tree.contains("\"projectionKind\":\"fdmOpenStrokeCohortSummary\""));
    assert!(layer_tree.contains("\"basis\":\"open-stroke-row-source-cohorts\""));
    assert!(layer_tree.contains("\"source\":\"fdmVectorCommandPrimitive\""));
    assert!(layer_tree.contains("\"diagnosticOnly\":true"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"open-stroke-role-and-paint-order-unproven\""
    ));
    assert!(layer_tree.contains("\"primitiveCount\":1889"));
    assert!(layer_tree.contains("\"openStrokeCount\":901"));
    assert!(layer_tree.contains("\"connectorCandidateCount\":67"));
    assert!(layer_tree.contains("\"horizontalCount\":267"));
    assert!(layer_tree.contains("\"verticalCount\":145"));
    assert!(layer_tree.contains("\"diagonalCount\":489"));
    assert!(layer_tree.contains("\"lineMarkerCount\":588"));
    assert!(layer_tree.contains("\"nonLineMarkerCount\":313"));
    assert!(layer_tree.contains("\"rowCount\":22"));
    assert!(layer_tree.contains("\"rowCohortLimit\":16"));
    assert!(layer_tree.contains("\"rowCohortCount\":16"));
    assert!(layer_tree.contains(
        "\"dominantConnectorRow\":{\"basis\":\"fdmOpenStrokeRowConnectorCandidateCount\",\"rowIndex\":0,\"connectorCandidateCount\":24,\"openStrokeCount\":155,\"horizontalCount\":34,\"verticalCount\":43,\"renderPromotionBlockedReason\":\"dominant-open-stroke-row-role-unproven\"}"
    ));
    assert!(layer_tree.contains(
        "\"rowCohorts\":[{\"rowIndex\":0,\"openStrokeCount\":155,\"connectorCandidateCount\":24,\"horizontalCount\":34,\"verticalCount\":43"
    ));
    assert!(layer_tree.contains(
        "\"markerStyleProfile\":{\"basis\":\"fdm-vector-marker+style-word\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"commandCount\":155,\"markerFamilyCounts\":[{\"reason\":\"line-marker\",\"count\":73},{\"reason\":\"path-marker\",\"count\":82}"
    ));
    assert!(layer_tree.contains(
        "\"styleWordCounts\":[{\"reason\":\"0x0000\",\"count\":152},{\"reason\":\"0x0005\",\"count\":0},{\"reason\":\"0x0080\",\"count\":0},{\"reason\":\"0x00a0\",\"count\":3},{\"reason\":\"other-style\",\"count\":0}]"
    ));
    assert!(layer_tree.contains(
        "\"roleGate\":{\"basis\":\"fdm-vector-marker-style-profile\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"markerFamilyDiversityCount\":2,\"styleWordDiversityCount\":2,\"dominantMarkerFamily\":\"path-marker\",\"dominantMarkerFamilyCount\":82,\"dominantStyleWord\":\"0x0000\",\"dominantStyleWordCount\":152,\"markerFamilyHomogeneous\":false,\"styleWordHomogeneous\":false,\"homogeneousMarkerStyleCandidate\":false,\"renderPromotionBlockedReason\":\"mixed-marker-family-and-style-word-role-unproven\"}"
    ));
    assert!(layer_tree.contains(
        "{\"rowIndex\":33,\"openStrokeCount\":47,\"connectorCandidateCount\":14,\"horizontalCount\":18,\"verticalCount\":8"
    ));
    assert!(layer_tree.contains(
        "\"markerStyleProfile\":{\"basis\":\"fdm-vector-marker+style-word\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"commandCount\":47,\"markerFamilyCounts\":[{\"reason\":\"line-marker\",\"count\":21},{\"reason\":\"path-marker\",\"count\":26}"
    ));
    assert!(layer_tree.contains(
        "\"styleWordCounts\":[{\"reason\":\"0x0000\",\"count\":0},{\"reason\":\"0x0005\",\"count\":21},{\"reason\":\"0x0080\",\"count\":12},{\"reason\":\"0x00a0\",\"count\":8},{\"reason\":\"other-style\",\"count\":6}]"
    ));
    assert!(layer_tree.contains(
        "\"markerFamilyDiversityCount\":2,\"styleWordDiversityCount\":4,\"dominantMarkerFamily\":\"path-marker\",\"dominantMarkerFamilyCount\":26,\"dominantStyleWord\":\"0x0005\",\"dominantStyleWordCount\":21,\"markerFamilyHomogeneous\":false,\"styleWordHomogeneous\":false,\"homogeneousMarkerStyleCandidate\":false,\"renderPromotionBlockedReason\":\"mixed-marker-family-and-style-word-role-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"sourceBboxUnion\":{\"left\":-9582,\"top\":-10333,\"right\":-8526,\"bottom\":-9344}"
    ));
    assert!(layer_tree.contains(
        "\"projectedBboxUnion\":{\"x\":682.702,\"y\":618.639,\"width\":102.306,\"height\":97.362}"
    ));
    assert!(
        layer_tree
            .contains("\"endpointMatchThresholds\":{\"basis\":\"documentTextLineHeaderGrid\"")
    );
    assert!(layer_tree.contains("\"projectedEndpointDistance\":"));
    assert!(layer_tree.contains("\"projectedSpan\":"));
    assert!(layer_tree.contains("\"orientation\":\"horizontal\""));
    assert!(layer_tree.contains("\"orientation\":\"vertical\""));
    assert!(layer_tree.contains("\"orientation\":\"diagonal\""));
    assert!(layer_tree.contains("\"rowIndex\":0,\"commandIndex\":150"));
    assert!(layer_tree.contains("\"source\":\"fdmVectorCommand\""));
    assert!(layer_tree.contains("\"projectionKind\":\"fdmCommandBBoxReferenceProjection\""));
    assert!(layer_tree.contains("\"markerHex\":\"ff000a60\""));
    assert_eq!(
        layer_tree
            .matches("\"type\":\"documentTextLineRuleProjection\"")
            .count(),
        16
    );
    assert!(layer_tree.contains("\"source\":\"/DocumentText\""));
    assert!(layer_tree.contains("\"projectionKind\":\"documentTextLineRuleProjection\""));
    assert!(layer_tree.contains("\"diagnosticOnly\":true"));
    assert!(layer_tree.contains("\"renderable\":false"));
    assert!(layer_tree.contains("\"projectionBasis\":\"documentTextLineHeaderGrid\""));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"line-rule-placement-and-topology-unproven\""
    ));
    assert!(layer_tree.contains("\"ruleIndex\":0"));
    assert!(layer_tree.contains("\"topologyCandidate\":{\"orthogonalGraph\":"));
    assert!(layer_tree.contains("\"startJunctionDegree\":"));
    assert!(layer_tree.contains("\"endJunctionDegree\":"));
    assert!(layer_tree.contains("\"isolatedEndpointCount\":"));
    assert!(layer_tree.contains("\"endpointAttachmentCandidates\":{\"start\":"));
    assert!(layer_tree.contains("\"attachmentProven\":false"));
    assert!(layer_tree.contains("\"nearestTextSlot\":{\"text\":"));
    assert!(layer_tree.contains("\"distancePx\":"));
    assert!(layer_tree.contains("\"probeRadiusPx\":"));
    assert!(layer_tree.contains("\"withinLineHeight\":"));
    assert_eq!(
        layer_tree
            .matches("\"renderAdmissionGate\":{\"source\":\"/DocumentText+/LineMark line-rule render admission\"")
            .count(),
        16
    );
    assert!(layer_tree.contains(
        "\"componentIndex\":2,\"componentRuleCount\":4,\"lineMarkMatched\":true,\"orthogonalGraphCandidate\":true,\"componentOrthogonalCandidate\":true"
    ));
    assert!(layer_tree.contains("\"bothEndpointTextAttachmentCandidate\":false"));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"document-text-grid-origin-reference-backed\",\"line-rule-endpoint-ownership-unproven\",\"line-rule-text-attachment-pair-unproven\",\"line-rule-style-role-unproven\",\"line-rule-paint-order-unproven\"],\"renderPromotionBlockedReason\":\"line-rule-render-admission-not-ready\""
    ));
    assert!(layer_tree.contains("\"candidateSource\":\"skippedInlineLineHeader\""));
    assert!(layer_tree.contains("\"candidateSource\":\"verticalAnchorRunFromLineHeaders\""));
    assert!(layer_tree.contains("\"lineMarkProfile\":\"be16-delta-v1\""));
    assert!(layer_tree.contains("\"lineMarkIntervalCount\":40"));
    assert!(layer_tree.contains("\"lineHeaderRawWords\":[28,48,"));
    assert!(layer_tree.contains("\"lineHeaderRawWordsHex\":[\"0x001c\",\"0x0030\""));
    assert!(layer_tree.contains("\"documentTextGroupCount\":37"));
    assert!(layer_tree.contains("\"documentTextLineHeaderCount\":171"));
    assert!(layer_tree.contains("\"skippedInlineLineHeaderCount\":7"));
    assert!(layer_tree.contains("\"type\":\"documentTextLineRuleProjectionSummary\""));
    assert!(layer_tree.contains("\"projectionKind\":\"documentTextLineRuleProjectionSummary\""));
    assert!(layer_tree.contains("\"ruleCount\":16"));
    assert!(layer_tree.contains(
        "\"candidateSourceCounts\":[{\"key\":\"skippedInlineLineHeader\",\"count\":7},{\"key\":\"verticalAnchorRunFromLineHeaders\",\"count\":9}]"
    ));
    assert!(layer_tree.contains(
        "\"orientationCounts\":[{\"key\":\"horizontal\",\"count\":7},{\"key\":\"vertical\",\"count\":9}]"
    ));
    assert!(layer_tree.contains("\"orthogonalGraphCandidateRuleCount\":11"));
    assert!(layer_tree.contains("\"noIsolatedEndpointRuleCount\":3"));
    assert!(layer_tree.contains("\"oneIsolatedEndpointRuleCount\":8"));
    assert!(layer_tree.contains("\"twoIsolatedEndpointRuleCount\":5"));
    assert!(layer_tree.contains("\"lineMarkMatchedRuleCount\":16"));
    assert!(layer_tree.contains("\"endpointCount\":32"));
    assert!(layer_tree.contains("\"endpointAttachmentWithinLineHeightCount\":2"));
    assert!(layer_tree.contains("\"bothEndpointAttachmentWithinLineHeightRuleCount\":0"));
    assert!(layer_tree.contains(
        "\"lineRuleRenderAdmissionGate\":{\"source\":\"/DocumentText+/LineMark line-rule render admission\""
    ));
    assert!(layer_tree.contains(
        "\"orthogonalGraphCandidateRuleCount\":11,\"orthogonalComponentCandidateCount\":3,\"lineMarkCoverageComplete\":true,\"noIsolatedEndpointRuleCount\":3,\"bothEndpointAttachmentWithinLineHeightRuleCount\":0"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"document-text-grid-origin-reference-backed\",\"line-rule-topology-partial-orthogonal-coverage\",\"line-rule-component-topology-unproven\",\"line-rule-endpoint-ownership-unproven\",\"line-rule-text-attachment-pair-absent\",\"line-rule-style-role-unproven\",\"line-rule-paint-order-unproven\"],\"renderPromotionBlockedReason\":\"line-rule-render-admission-not-ready\""
    ));
    assert!(layer_tree.contains("\"lineRuleGraphComponentCount\":6"));
    assert!(layer_tree.contains("\"largestLineRuleGraphComponentRuleCount\":4"));
    assert!(layer_tree.contains("\"lineRuleGraphComponents\":["));
    assert!(layer_tree.contains(
        "\"ruleIndexes\":[2,5,13,14],\"bbox\":{\"x\":363.038,\"y\":433.500,\"width\":261.430,\"height\":200.400},\"ruleCount\":4,\"horizontalRuleCount\":2,\"verticalRuleCount\":2,\"orthogonalGraphRuleCount\":4,\"lineMarkMatchedRuleCount\":4,\"endpointCount\":8,\"isolatedEndpointCount\":3,\"totalProjectedLengthPx\":765.858"
    ));
    assert!(layer_tree.contains(
        "\"orthogonalComponentCandidate\":true,\"lineMarkCoverageComplete\":true,\"renderAdmissionGate\":{\"source\":\"/DocumentText+/LineMark line-rule component render admission\""
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"document-text-grid-origin-reference-backed\",\"line-rule-component-endpoint-ownership-unproven\",\"line-rule-component-style-role-unproven\",\"line-rule-paint-order-unproven\"],\"renderPromotionBlockedReason\":\"line-rule-component-render-admission-not-ready\""
    ));
    assert!(layer_tree.contains(
        "\"ruleIndexes\":[3,6,15],\"bbox\":{\"x\":725.679,\"y\":415.500,\"width\":357.641,\"height\":200.400},\"ruleCount\":3,\"horizontalRuleCount\":2,\"verticalRuleCount\":1,\"orthogonalGraphRuleCount\":3,\"lineMarkMatchedRuleCount\":3,\"endpointCount\":6,\"isolatedEndpointCount\":3,\"totalProjectedLengthPx\":908.482"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"line-rule-component-placement-and-style-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedDetail\":\"line-rule-endpoint-attachments-and-line-mark-row-boundaries-unproven\""
    ));
    assert_eq!(
        layer_tree
            .matches("\"type\":\"documentTextLineHeaderProjectionCandidateSummary\"")
            .count(),
        1
    );
    assert!(
        layer_tree
            .contains("\"projectionKind\":\"documentTextLineHeaderProjectionCandidateSummary\"")
    );
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"line-header-visible-rule-selector-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"selectorBasis\":\"current-horizontal-rule-promotion-requires-skipped-inline-text\""
    ));
    assert!(layer_tree.contains(
        "\"fullSpanRenderPromotionGate\":{\"basis\":\"documentTextLineHeaderCandidate+sourceMapContext\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false,\"renderPromotionBlockedReason\":\"line-header-segment-clipping-and-endpoint-ownership-unproven\",\"requiresSegmentClippingDecoded\":true,\"requiresEndpointOwnershipDecoded\":true,\"requiresPaintOrderDecoded\":true,\"fullSpanRenderableCandidateCount\":0}"
    ));
    assert!(layer_tree.contains(
        "\"gridOriginAuthorityGate\":{\"basis\":\"selectedDocumentTextLineHeaders+/LineMark+/PageMark\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false,\"selectedLineHeaderCount\":7"
    ));
    assert!(
        layer_tree
            .contains("\"selectedGroupIndexes\":[21,31],\"selectedLineMarkRecordIndexes\":[22,32]")
    );
    assert!(layer_tree.contains(
        "\"selectedLineMarkSourceUnitGate\":{\"source\":\"/LineMark selected record source-unit intervals\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false,\"selectedRecordCount\":2,\"intervalRecordCount\":2,\"allSelectedRecordsHaveIntervals\":true,\"recordIndexes\":[22,32],\"unitStarts\":[3908,5483],\"unitEnds\":[4121,5615],\"unitSpans\":[213,132],\"recordIndexDeltas\":[10],\"unitStartDeltas\":[1575],\"sourceUnitDeltaPerRecordEstimate\":157.500,\"strideCandidateSampleCount\":1,\"strideCandidateReady\":false,\"promotionReady\":false,\"blockedReason\":\"line-mark-source-unit-stride-insufficient-selected-rows\"}"
    ));
    assert!(layer_tree.contains(
        "\"allSelectedHeadersHaveLineMark\":true,\"lineMarkRecordIndexesContiguous\":false,\"lineMarkRecordStride\":10,\"recordIndexMinusGroupIndexValues\":[1],\"uniformRecordIndexMinusGroupIndex\":true,\"sourceDomainRowAnchorCandidate\":true"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkEntryCoverage\":{\"rowIndex\":0,\"index\":0,\"flags\":65536,\"flagsHex\":\"0x00010000\",\"lineStart\":0,\"lineEnd\":39}"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkEntryProfileGate\":{\"source\":\"/PageMark u16 geometry profile\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false,\"entryPresent\":true,\"u16GeometryClass\":\"mixed-payload\",\"additiveGeometryProfile\":false,\"promotionSafeProfile\":false,\"blockedReason\":\"page-mark-mixed-payload-profile-not-layout-origin-authority\"}"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyGridDomain\":{\"rawMaxExtentUnits\":280,\"maxExtentUnits\":276,\"textGridColumnOriginDecoded\":false,\"textGridRowOriginDecoded\":false}"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageMarkYValueProbe\":{\"source\":\"/PageMark parsed entry y-value candidates\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false,\"pageMarkEntryPresent\":true,\"candidateCount\":58,\"inPageRangeCandidateCount\":58,\"currentProjectionOriginY\":38.700"
    ));
    assert!(layer_tree.contains(
        "\"nearestCurrentProjectionOriginCandidate\":{\"source\":\"parsedEntryU16\",\"interpretation\":\"direct-u16-px\",\"wordIndex\":7,\"byteOffset\":14,\"value\":39,\"valuePx\":39.000,\"residualPx\":0.300}"
    ));
    assert!(layer_tree.contains(
        "\"lineBoundaryConflictGate\":{\"source\":\"/PageMark parsed entry lineStart/lineEnd vs nearest y candidate\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false,\"lineStart\":0,\"lineEnd\":39,\"nearestCandidateValue\":39,\"matchesLineStart\":false,\"matchesLineEnd\":true,\"matchedBoundaryRoles\":[\"lineEnd\"],\"lineBoundaryConflict\":true,\"selectionReady\":false,\"promotionReady\":false,\"blockedReason\":\"nearest-page-mark-y-candidate-overlaps-line-boundary\"}"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"page-mark-y-value-field-role-unproven\",\"document-text-grid-origin-reference-backed\",\"page-space-y-origin-unproven\"],\"renderPromotionBlockedReason\":\"source-only-page-space-y-origin-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"pageSpaceOriginCandidate\":null,\"pageSpaceOriginCandidateReady\":false,\"promotionReady\":false,\"blockedReasons\":[\"document-text-grid-origin-reference-backed\",\"line-header-visible-rule-selector-unproven\",\"page-space-y-origin-unproven\"],\"renderPromotionBlockedReason\":\"line-header-grid-origin-authority-unproven\""
    ));
    assert!(layer_tree.contains("\"allLineHeaderCount\":171"));
    assert!(layer_tree.contains("\"longLineHeaderCandidateCount\":103"));
    assert!(layer_tree.contains("\"skippedInlineLongLineHeaderCandidateCount\":7"));
    assert!(layer_tree.contains("\"selectedSkippedInlineLongLineHeaderCandidateCount\":7"));
    assert!(layer_tree.contains("\"unselectedLongLineHeaderCandidateCount\":96"));
    assert!(layer_tree.contains(
        "\"groupIndex\":17,\"lineOffsetUnits\":130,\"lineExtentUnits\":280,\"segmentUnits\":150,\"bbox\":{\"x\":525.856,\"y\":361.500,\"width\":557.464,\"height\":2.400},\"candidateSource\":\"documentTextLineHeader\",\"selectedAsHorizontalRule\":false"
    ));
    assert!(layer_tree.contains(
        "\"selectedAsHorizontalRule\":false,\"fullSpanRenderPromotionGate\":{\"basis\":\"documentTextLineHeaderCandidate+sourceMapContext\",\"sourceBacked\":true,\"decoded\":false,\"renderable\":false,\"fullSpanCandidate\":false,\"renderPromotionBlockedReason\":\"line-header-segment-clipping-and-endpoint-ownership-unproven\"}"
    ));
    assert!(layer_tree.contains(
        "\"groupIndex\":31,\"lineOffsetUnits\":0,\"lineExtentUnits\":84,\"segmentUnits\":84,\"bbox\":{\"x\":44.801,\"y\":613.501,\"width\":313.236,\"height\":2.400},\"candidateSource\":\"skippedInlineText\",\"selectedAsHorizontalRule\":true"
    ));
    assert!(layer_tree.contains(
        "\"selectedAsHorizontalRule\":true,\"fullSpanRenderPromotionGate\":{\"basis\":\"documentTextLineHeaderCandidate+sourceMapContext\",\"sourceBacked\":true,\"decoded\":false,\"renderable\":false,\"fullSpanCandidate\":true,\"renderPromotionBlockedReason\":\"line-header-segment-clipping-and-endpoint-ownership-unproven\"}"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkContext\":{\"recordIndex\":18,\"unitRange\":{\"start\":3333,\"end\":3470},\"flagWord\":2,\"flagWordHex\":\"0x0002\",\"headerUnitOffsetFromLineMarkStart\":109,\"headerWithinLineMark\":true},\"documentTextMapContext\":{\"containingEntry\":null"
    ));
    assert!(layer_tree.contains(
        "\"sameSegmentGroupRun\":{\"basis\":\"same-offset-extent-contiguous-groups\",\"offsetUnits\":130,\"extentUnits\":280,\"startGroupIndex\":17,\"endGroupIndex\":19,\"groupCount\":3,\"positionInRun\":0}"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkContext\":{\"recordIndex\":22,\"unitRange\":{\"start\":3908,\"end\":4121},\"flagWord\":32770,\"flagWordHex\":\"0x8002\",\"headerUnitOffsetFromLineMarkStart\":135,\"headerWithinLineMark\":true},\"documentTextMapContext\":{\"containingEntry\":{\"kind\":\"skipped-inline\""
    ));
    assert!(
        layer_tree
            .contains("\"selector\":143,\"selectorHex\":\"0x008f\",\"code\":null,\"codeHex\":null")
    );
    assert!(layer_tree.contains(
        "\"insideSkippedInlineText\":true,\"insideTextRun\":false,\"adjacentToSkippedInlineText\":false},\"sameSegmentGroupRun\":{\"basis\":\"same-offset-extent-contiguous-groups\",\"offsetUnits\":16,\"extentUnits\":46,\"startGroupIndex\":20,\"endGroupIndex\":23,\"groupCount\":4,\"positionInRun\":1}"
    ));
    assert!(layer_tree.contains("\"sameSegmentGroupRuns\":["));
    assert!(layer_tree.contains(
        "\"offsetUnits\":130,\"extentUnits\":280,\"segmentUnits\":150,\"startGroupIndex\":17,\"endGroupIndex\":19,\"groupCount\":3,\"selectedHorizontalRuleCount\":0,\"skippedInlineCount\":0,\"noContainingMapEntryCount\":3"
    ));
    assert!(layer_tree.contains(
        "\"offsetUnits\":184,\"extentUnits\":280,\"segmentUnits\":96,\"startGroupIndex\":20,\"endGroupIndex\":31,\"groupCount\":12,\"selectedHorizontalRuleCount\":2,\"skippedInlineCount\":2,\"noContainingMapEntryCount\":10"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"line-header-run-visibility-selector-unproven\""
    ));
    assert!(layer_tree.contains("\"orientation\":\"horizontal\""));
    assert!(layer_tree.contains("\"orientation\":\"vertical\""));
    assert!(layer_tree.contains("\"lineOffsetUnits\":92"));
    assert!(layer_tree.contains("\"lineExtentUnits\":156"));
    assert!(layer_tree.contains("\"lineOffsetUnits\":84"));
    assert!(layer_tree.contains("\"lineExtentUnits\":84"));
    assert!(!layer_tree.contains("\"lineOffsetUnits\":86,\"lineExtentUnits\":88"));
    assert!(layer_tree.contains("\"lineMarkRecordIndex\":22"));
    assert!(layer_tree.contains("\"lineMarkRecordIndex\":32"));
    assert!(layer_tree.contains("\"lineMarkUnitInterval\":{\"start\":3908,\"end\":4121}"));
    assert!(layer_tree.contains("\"lineMarkUnitInterval\":{\"start\":5483,\"end\":5615}"));
    assert!(layer_tree.contains("\"lineMarkFlagHex\":\"0x8002\""));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidate\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\""
    ));
    assert!(layer_tree.contains("\"lineMarkRowsExactAndContiguous\":false"));
    assert!(layer_tree.contains("\"lineHeaderRowsHomogeneous\":false"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"line-mark-rows-not-exact-source-boundaries\""
    ));
    assert!(layer_tree.contains(
        "\"pageSpaceSolver\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\",\"solverVersion\":\"table-page-space-v1\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"solverStage\":\"blocked-row-height-transform\",\"sourcePlacementEvidencePresent\":true,\"candidateRowCount\":2,\"requestedColumnCount\":3,\"commonMatchedColumnCount\":3,\"matchedCellHeaderCount\":6,\"requiredCellHeaderCount\":6"
    ));
    assert!(layer_tree.contains(
        "\"horizontalSolverReady\":true,\"rowHeightSolverReady\":false,\"yOriginSolverReady\":false,\"lineHeaderRowsHomogeneous\":false,\"lineMarkRowRecordSelection\":\"selected-overlap-record\",\"lineMarkRowsExactAndContiguous\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidatePresent\":true,\"sourceDerivedLayoutRenderable\":false,\"pageOriginAuthority\":\"lineMarkPageGrid\",\"lineMarkPageOriginPresent\":true,\"lineMarkPageOriginStridePresent\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourcePageYTransformGate\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark y-origin promotion gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":false,\"lineMarkRowsExactAndContiguous\":false,\"pageOriginAuthority\":\"lineMarkPageGrid\",\"lineMarkPageOriginPresent\":true,\"lineMarkPageOriginStridePresent\":false,\"subrecordLineSpanReadinessPresent\":false"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"line-mark-rows-not-exact-source-boundaries\",\"decoded-line-mark-page-y-transform-missing\"],\"renderPromotionContribution\":\"source-page-y-transform-gate\",\"renderPromotionBlockedReason\":\"source-page-y-transform-not-decoded\""
    ));
    assert!(layer_tree.contains(
        "\"renderPromoted\":false,\"renderPromotionAuthority\":null,\"renderPromotionBlockedReason\":\"line-mark-rows-not-exact-source-boundaries\""
    ));
    assert!(layer_tree.contains("\"groupIndex\":21"));
    assert!(layer_tree.contains("\"groupIndex\":31"));
    assert!(layer_tree.contains("\"endGroupIndex\":31"));
    assert!(layer_tree.contains("\"strokeWidth\":2.400"));
    assert!(layer_tree.contains("\"source\":\"fdmVectorCommandPrimitive\""));
    assert!(layer_tree.contains("\"projectionKind\":\"fdmVectorPrimitiveReferenceProjection\""));
    assert!(layer_tree.contains("\"geometryDecoded\":true"));
    assert!(layer_tree.contains("\"renderable\":true"));
    assert!(layer_tree.contains("\"paintCoverage\":{\"bboxAreaPx\":"));
    assert!(layer_tree.contains("\"pageCoverageRatio\":"));
    assert!(layer_tree.contains("\"viewportCoverageRatio\":"));
    assert!(layer_tree.contains("\"pageFillCandidateReason\":"));
    assert!(layer_tree.contains("\"fillPaintPresent\":"));
    assert!(
        layer_tree.contains("\"pageFillCandidateBasis\":\"closed-fill-and-large-span-filter\"")
    );
    assert!(layer_tree.contains("\"pagePaintSourceEvidenceProven\":false"));
    assert!(layer_tree.contains("\"pageFillCandidate\":false"));
    assert!(layer_tree.contains("\"decoded\":false"));
    assert!(layer_tree.contains("\"markerHex\":\"ff000160\""));
    assert!(layer_tree.contains("\"markerHex\":\"ff000460\""));
    assert!(layer_tree.contains("\"markerHex\":\"ff000660\""));
    assert!(layer_tree.contains("\"markerHex\":\"ff000960\""));
    assert!(layer_tree.contains("\"markerHex\":\"00000460\""));
    assert!(layer_tree.contains("\"markerHex\":\"00000660\""));
    assert!(layer_tree.contains("\"markerHex\":\"00000960\""));
    assert!(layer_tree.contains("\"primitiveKind\":\"ellipse\""));
    assert!(layer_tree.contains("\"primitiveKind\":\"cubicBezier\""));
    assert!(layer_tree.contains("\"curveSegmentCount\":2"));
    assert!(layer_tree.contains(
        "\"ellipse\":{\"center\":{\"x\":-6130,\"y\":-13098},\"radiusX\":510,\"radiusY\":510"
    ));
    assert!(layer_tree.contains("\"styleWordHex\":\"0x0088\""));
    assert!(layer_tree.contains("\"compoundChildOffsets\":[72,194,316"));
    assert!(layer_tree.contains(
        "\"compoundChildLayoutGate\":{\"source\":\"FDMVector compound prefix child-offset table+child declared lengths\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false"
    ));
    assert!(layer_tree.contains("\"firstChildMatchesPrefixEnd\":true"));
    assert!(layer_tree.contains("\"childOffsetsStrictlyIncreasing\":true"));
    assert!(layer_tree.contains("\"childRecordsFitParent\":true"));
    assert!(layer_tree.contains("\"childRecordsDoNotOverlap\":true"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"compound-child-boundaries-do-not-prove-connector-ownership-or-paint-order\""
    ));
    assert!(layer_tree.contains("\"fillColor\":\"#000000\""));
    assert!(layer_tree.contains("\"fillColor\":\"#ffffff\""));
    assert!(layer_tree.contains("\"commandIndex\":2007,\"markerHex\":\"00000960\""));
    assert!(layer_tree.contains(
        "\"fillColor\":\"#ffffff\",\"renderFillKind\":\"solid\",\"renderFillColor\":\"#000000\""
    ));
    assert!(layer_tree.contains("\"renderCounterOverlay\":true"));
    assert!(layer_tree.contains(
        "\"fillColor\":\"#000000\",\"renderFillKind\":\"solid\",\"renderFillColor\":\"#ffffff\""
    ));
    assert!(layer_tree.contains(
        "\"fillColor\":\"#7a7acc\",\"renderFillKind\":\"linearGradient\",\"renderFillColor\":\"#7a7acc\",\"renderGradient\":{\"from\":\"#003366\",\"to\":\"#7a7acc\"}"
    ));
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmTextMaskCohortSummary\"")
            .count(),
        1
    );
    assert!(layer_tree.contains("\"projectionKind\":\"fdmTextMaskCohortSummary\""));
    assert!(
        layer_tree.contains("\"basis\":\"fdmVectorClosedFillCohort+documentTextRightNeighbor\"")
    );
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"fdm-text-mask-document-text-alignment-unproven\""
    ));
    assert!(layer_tree.contains("\"cohortCount\":24"));
    assert!(layer_tree.contains("\"rightNeighborCandidateCount\":7"));
    assert!(layer_tree.contains("\"topTextLikeComponentCandidateCount\":3"));
    assert!(layer_tree.contains("\"componentRightNeighborCandidateCount\":1"));
    assert!(layer_tree.contains(
        "\"rowIndex\":5,\"primitiveCount\":22,\"blackFillPrimitiveCount\":12,\"whiteFillPrimitiveCount\":10,\"counterOverlayCount\":0"
    ));
    assert!(layer_tree.contains(
        "\"projectedBbox\":{\"x\":465.107,\"y\":84.575,\"width\":34.683,\"height\":76.196},\"sourceBbox\":{\"left\":-11828,\"top\":-15758,\"right\":-11470,\"bottom\":-14984},\"rightNeighborTextSlotCandidate\":{\"source\":\"/DocumentText\",\"sourceBacked\":true,\"decoded\":false,\"candidateRelation\":\"right-neighbor-overlapping-y\",\"text\":\"5\""
    ));
    assert!(layer_tree.contains(
        "\"horizontalGapPx\":12.464,\"verticalOverlapPx\":19.950,\"centerDeltaYPx\":-19.998"
    ));
    assert!(layer_tree.contains(
        "\"alignmentPromotionBlockedReason\":\"fdm-text-mask-to-document-text-baseline-transform-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"componentCount\":6,\"topTextLikeComponentCandidate\":{\"source\":\"fdmVectorClosedFillComponent\""
    ));
    assert!(layer_tree.contains(
        "\"componentIndex\":0,\"primitiveCount\":7,\"blackFillPrimitiveCount\":7,\"whiteFillPrimitiveCount\":0,\"counterOverlayCount\":0"
    ));
    assert!(layer_tree.contains(
        "\"projectedBbox\":{\"x\":465.204,\"y\":84.575,\"width\":34.586,\"height\":10.435},\"sourceBbox\":{\"left\":-11827,\"top\":-15758,\"right\":-11470,\"bottom\":-15652},\"rightNeighborTextSlotCandidate\":{\"source\":\"/DocumentText\",\"sourceBacked\":true,\"decoded\":false,\"candidateRelation\":\"component-right-neighbor-overlapping-y\",\"text\":\"5\""
    ));
    assert!(layer_tree.contains(
        "\"horizontalGapPx\":12.464,\"verticalOverlapPx\":2.310,\"centerDeltaYPx\":12.882"
    ));
    assert!(layer_tree.contains(
        "\"leadingWhitespaceBridgeCandidate\":{\"source\":\"fdmTextMaskBbox+/DocumentText pre-fragment span\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderPromoted\":false,\"candidateClass\":\"fdm-bbox-inside-document-text-pre-fragment-projection\",\"bridgeCandidate\":true"
    ));
    assert!(layer_tree.contains(
        "\"parentTextRunSourceUnitRange\":{\"start\":578,\"end\":613},\"preFragmentSourceByteRange\":{\"start\":1156,\"end\":1198},\"preFragmentSourceUnitRange\":{\"start\":578,\"end\":599},\"preFragmentUnitCount\":21"
    ));
    assert!(layer_tree.contains(
        "\"preFragmentProjectionGridUnits\":42,\"preFragmentProjectedWidthPx\":155.418,\"lineStartX\":356.837,\"textStartX\":512.255,\"sourceBboxOffsetFromLineStartPx\":108.368,\"sourceBboxEndOffsetFromLineStartPx\":142.954"
    ));
    assert!(layer_tree.contains(
        "\"sourceBboxWithinPreFragmentProjection\":true,\"sourceBboxBeginsAfterLineStart\":true,\"sourceBboxEndsBeforeTextStart\":true,\"sourceBboxRightToTextStartGapPx\":12.464,\"textBaselineMinusSourceBottomPx\":8.330,\"renderPromotionBlockedReason\":\"document-text-pre-fragment-fdm-mask-role-unproven\""
    ));
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmTextMaskRightNeighborPromotionReadiness\"")
            .count(),
        7
    );
    assert!(layer_tree.contains(
        "\"cohortSlot\":{\"slotIndex\":6,\"text\":\"5\",\"bbox\":{\"x\":512.255,\"y\":92.700,\"width\":13.300,\"height\":19.950},\"horizontalGapPx\":12.464,\"verticalOverlapPx\":19.950,\"centerDeltaYPx\":-19.998,\"groupIndex\":2,\"sourceUnitRange\":{\"start\":599,\"end\":600},\"splitFromTextRun\":false},\"componentSlot\":{\"slotIndex\":6,\"text\":\"5\""
    ));
    assert!(layer_tree.contains(
        "\"cohortComponentAgreement\":true,\"bestGapPx\":12.464,\"secondBestGapPx\":null,\"gapMarginPx\":null,\"splitFromTextRun\":false,\"fragmentCount\":1,\"preFragmentUnitCount\":21,\"sourceBboxWithinPreFragmentProjection\":true,\"sourceBboxBeginsAfterLineStart\":true,\"sourceBboxEndsBeforeTextStart\":true,\"sameSegmentGroupRunDistinctTextGroupCount\":4,\"rowAnchorAmbiguous\":true,\"baselineResidualPx\":8.330,\"promotionReady\":false"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"document-text-slot-not-split-from-text-run\",\"line-header-y-run-placement-semantics-unproven\",\"document-text-pre-fragment-fdm-mask-role-unproven\",\"fdm-text-mask-to-document-text-baseline-transform-unproven\",\"fdm-text-mask-promotion-cross-sample-support-missing\"],\"renderPromotionBlockedReason\":\"fdm-text-mask-right-neighbor-promotion-readiness-blocked\""
    ));
    assert_eq!(
        layer_tree
            .matches("\"type\":\"fdmTextMaskSourceTransformCandidateSummary\"")
            .count(),
        1
    );
    assert!(
        layer_tree.contains("\"projectionKind\":\"fdmTextMaskSourceTransformCandidateSummary\"")
    );
    assert!(layer_tree.contains(
        "\"candidateCount\":1,\"preFragmentBridgeCandidateCount\":1,\"cohortComponentAgreementCount\":1,\"rowAnchorAmbiguousCandidateCount\":1,\"slotNotSplitCandidateCount\":1"
    ));
    assert!(
        layer_tree.contains("\"sourceUnitsPerTextGridUnitXRange\":{\"min\":38.196,\"max\":38.196}")
    );
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"fdm-source-to-document-text-transform-reference-backed-and-row-anchor-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"rowIndex\":5,\"candidateClass\":\"top-text-like-component-to-document-text-pre-fragment\",\"componentIndex\":0,\"slotIndex\":6,\"slotText\":\"5\""
    ));
    assert!(layer_tree.contains(
        "\"currentProjectionGridOffsetRange\":{\"start\":29.285,\"end\":38.632,\"span\":9.347}"
    ));
    assert!(layer_tree.contains(
        "\"sourceXTransformCandidate\":{\"sourceUnitsPerTextGridUnit\":38.196,\"lineStartSourceX\":-12945.565,\"textStartSourceX\":-11341.348,\"sourceGapToTextStartX\":128.652,\"transformAuthorityProven\":false}"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"fdm-text-mask-component-to-document-text-alignment-unproven\""
    ));
    assert!(layer_tree.contains("\"rowIndex\":23,\"commandIndex\":8001"));
    assert!(layer_tree.contains("\"renderStrokeColor\":\"#ffffff\""));
    assert!(layer_tree.contains("\"type\":\"textRun\",\"bbox\":{\"x\":46.001,\"y\":38.700"));
    assert!(layer_tree.contains("\"projectionKind\":\"documentTextGroupLineProjection\""));
    assert!(layer_tree.contains("\"sourceStream\":\"/DocumentText\""));
    assert!(layer_tree.contains("\"text\":\"社内LAN構成図"));
    assert!(layer_tree.contains("\"fillColor\":\"#008000\""));
    assert!(layer_tree.contains("\"text\":\"ファイルサーバ\""));
    assert!(layer_tree.contains("\"bbox\":{\"x\":430.845,\"y\":56.700"));
    assert!(layer_tree.contains("\"baselineFactor\":0.800"));
    assert!(layer_tree.contains("\"text\":\"ルーター\""));
    assert!(layer_tree.contains("\"groupIndex\":15"));
    assert!(layer_tree.contains("\"text\":\"DHCP機能\""));
    assert!(layer_tree.contains("\"text\":\"NASサーバ\""));
    assert!(layer_tree.contains("\"text\":\"ﾍﾟﾝﾌﾟﾛｯﾀｰ"));
    assert!(layer_tree.contains("\"text\":\"ｲﾝｸｼﾞｪｯﾄﾌﾟﾛｯﾀｰ\""));
    assert!(layer_tree.contains("\"text\":\"5\""));
    assert!(layer_tree.contains(
        "\"lineHeaderSameSegmentGroupRun\":{\"basis\":\"same-offset-extent-contiguous-groups\",\"offsetUnits\":84,\"extentUnits\":174,\"startGroupIndex\":1,\"endGroupIndex\":6,\"groupCount\":6,\"positionInRun\":1}"
    ));
    assert!(layer_tree.contains(
        "\"lineHeaderYPlacementCandidate\":{\"source\":\"/DocumentText line-header same-offset/extent group run\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderPromoted\":false,\"sameSegmentGroupRunPresent\":true,\"startGroupIndex\":1,\"endGroupIndex\":6,\"groupCount\":6,\"positionInRun\":1,\"sameSegmentGroupRunTextSlotCount\":5,\"sameSegmentGroupRunDistinctTextGroupCount\":4,\"sameSegmentGroupRunAmbiguousAsRowAnchor\":true,\"renderPromotionBlockedDetail\":\"same-segment-run-spans-multiple-visible-text-rows\",\"currentGroupIndex\":2,\"renderPromotionBlockedReason\":\"line-header-y-run-placement-semantics-unproven\"}"
    ));
    assert!(layer_tree.contains(
        "\"lineHeaderSameSegmentGroupRun\":{\"basis\":\"same-offset-extent-contiguous-groups\",\"offsetUnits\":0,\"extentUnits\":84,\"startGroupIndex\":27,\"endGroupIndex\":31,\"groupCount\":5,\"positionInRun\":2}"
    ));
    assert!(
        layer_tree
            .contains("\"fillColorBasis\":\"document-text-style-property-15-text-run-candidate\"")
    );
    assert!(layer_tree.contains("\"type\":\"shanaiLanTextStyleEvidenceSummary\""));
    assert!(layer_tree.contains("\"projectionKind\":\"shanaiLanTextStyleEvidenceSummary\""));
    assert!(layer_tree.contains("\"slotCount\":38"));
    assert!(layer_tree.contains("\"textStyleLinkEvidenceCount\":38"));
    assert!(layer_tree.contains("\"documentViewStyleGroupCandidateSlotCount\":8"));
    assert!(layer_tree.contains("\"documentTextGroupHeaderCandidateSlotCount\":35"));
    assert!(layer_tree.contains("\"documentTextInlineStyleCandidateSlotCount\":1"));
    assert!(layer_tree.contains("\"sourcePropertyFillColorSlotCount\":16"));
    assert!(layer_tree.contains("\"fillColorPromotionBlockedSlotCount\":0"));
    assert!(layer_tree.contains("\"splitFromTextRunSlotCount\":14"));
    assert!(layer_tree.contains("\"multiFragmentParentTextRunSlotCount\":14"));
    assert!(layer_tree.contains("\"maxParentTextRunFragmentCount\":5"));
    assert!(layer_tree.contains("\"mixedFillMultiFragmentParentRunCount\":2"));
    assert!(layer_tree.contains("\"fillColorBasisCounts\":[{\"fillColorBasis\":\"default-text-fill\",\"fillColor\":\"#111111\",\"count\":22},{\"fillColorBasis\":\"document-text-style-property-15-text-run-candidate\",\"fillColor\":\"#000066\",\"count\":1},{\"fillColorBasis\":\"document-text-style-property-15-text-run-candidate\",\"fillColor\":\"#000080\",\"count\":14},{\"fillColorBasis\":\"document-text-style-property-15-text-run-candidate\",\"fillColor\":\"#008000\",\"count\":1}]"));
    assert!(layer_tree.contains(
        "\"documentViewStyleGroupCandidateCounts\":[{\"documentViewStyleGroupCandidate\":3,\"count\":8}]"
    ));
    assert!(layer_tree.contains(
        "\"groupHeaderPromotionBlockedReason\":\"document-text-group-header-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"controlKindHex\":\"0x0010\",\"firstFieldWordHex\":\"0x0017\",\"fillColorBasis\":\"default-text-fill\",\"fillColor\":\"#111111\",\"count\":3"
    ));
    assert!(layer_tree.contains(
        "\"controlKindHex\":\"0x0010\",\"firstFieldWordHex\":\"0x0017\",\"fillColorBasis\":\"document-text-style-property-15-text-run-candidate\",\"fillColor\":\"#000080\",\"count\":2"
    ));
    assert!(layer_tree.contains(
        "\"parentTextRunSourceSpan\":{\"byteStart\":12060,\"byteEnd\":12384,\"unitStart\":6030,\"unitEnd\":6192},\"slotCount\":5,\"fillColorBasisCount\":1,\"fillColorCount\":1,\"sourcePropertyFillColorSlotCount\":5,\"defaultFillColorSlotCount\":0"
    ));
    assert!(layer_tree.contains(
        "\"fillColorBases\":[\"default-text-fill\"],\"fillColors\":[\"#111111\"],\"styleBoundaryProven\":false,\"renderPromotionBlockedReason\":\"document-text-fragment-style-boundary-unproven\""
    ));
    assert!(layer_tree.contains("\"textStyleLinkEvidence\":{\"decoded\":false,\"source\":\"DocumentText+DocumentTextPositionTables+DocumentViewStyles\",\"styleLinkProven\":false,\"textLayoutStyleRecordCount\":0,\"documentViewStyleGroupCount\":9"));
    assert!(layer_tree.contains("\"documentViewStyleGroupCandidate\":3,\"documentViewStyleGroupCandidateBasis\":\"document-text-position-count-tail-field-f7\""));
    assert!(layer_tree.contains("\"documentTextGroupHeaderCandidate\":{\"decoded\":false,\"source\":\"/DocumentText\",\"sourceSpan\":{\"byteStart\":286,\"byteEnd\":324,\"unitStart\":143,\"unitEnd\":162}"));
    assert!(layer_tree.contains("\"rawWordsHex\":[\"0x001c\",\"0x0010\",\"0x0013\",\"0x0000\",\"0x008f\",\"0x0007\",\"0x0118\",\"0x0000\",\"0x0052\",\"0x0008\",\"0x005b\",\"0x0000\",\"0x0068\",\"0xffff\",\"0x0000\",\"0x0013\",\"0x0000\",\"0x0010\",\"0x001f\"]"));
    assert!(layer_tree.contains("\"fieldWordsHex\":[\"0x0013\",\"0x0000\",\"0x008f\",\"0x0007\",\"0x0118\",\"0x0000\",\"0x0052\",\"0x0008\",\"0x005b\",\"0x0000\",\"0x0068\",\"0xffff\",\"0x0000\",\"0x0013\",\"0x0000\",\"0x0010\"]"));
    assert!(
        layer_tree.contains(
            "\"promotionBlockedReason\":\"document-text-group-header-semantics-unproven\""
        )
    );
    assert!(layer_tree.contains("\"documentTextInlineStyleCandidate\":{\"decoded\":false,\"source\":\"/DocumentText\",\"sourceSpan\":{\"byteStart\":60,\"byteEnd\":86,\"unitStart\":30,\"unitEnd\":43},\"selector\":1,\"selectorHex\":\"0x0001\""));
    assert!(layer_tree.contains("\"payloadWordsHex\":[\"0x0002\"]"));
    assert!(
        layer_tree.contains("\"postInlineWordsHex\":[\"0x0005\",\"0x0000\",\"0x0001\",\"0x001f\"]")
    );
    assert!(layer_tree.contains("\"rawWordsHex\":[\"0x001c\",\"0x0001\",\"0x0007\",\"0x0000\",\"0x0000\",\"0x0001\",\"0x001d\",\"0x0002\",\"0x001e\",\"0x0005\",\"0x0000\",\"0x0001\",\"0x001f\"]"));
    assert!(layer_tree.contains(
        "\"promotionBlockedReason\":\"document-text-inline-control-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"styleLinkPromotionBlockedReason\":\"document-view-style-group-link-unproven\""
    ));
    assert!(layer_tree.contains("\"documentTextProperty15ColorCandidate\":{\"source\":\"/DocumentText style section\",\"propertyId\":15,\"packedBgrHex\":\"0x00008000\",\"cssColor\":\"#008000\",\"sourceBacked\":true,\"colorEncodingDecoded\":true,\"propertyRoleDecoded\":false,\"contextGate\":\"shanai-lan-text-projection\",\"renderPromoted\":true}"));
    assert!(layer_tree.contains("\"textCountRangeEvidenceCount\":3"));
    assert!(layer_tree.contains(
        "\"basis\":\"byte\",\"rangeStart\":3603,\"rangeEnd\":4078,\"overlapStart\":3986,\"overlapEnd\":3996"
    ));
    assert!(layer_tree.contains(
        "\"basis\":\"unit\",\"rangeStart\":3603,\"rangeEnd\":4078,\"overlapStart\":3612,\"overlapEnd\":3618"
    ));
    assert!(layer_tree.contains("\"bbox\":{\"x\":245.824,\"y\":704.701"));
    assert!(layer_tree.contains("\"fragmentStartUnits\":17"));
    assert!(layer_tree.contains("\"splitFromTextRun\":true"));
    assert!(layer_tree.contains("\"textRunFragmentContext\":{\"decoded\":false,\"source\":\"/DocumentText\",\"parentTextRunSourceSpan\":{\"byteStart\":12060,\"byteEnd\":12384,\"unitStart\":6030,\"unitEnd\":6192}"));
    assert!(layer_tree.contains("\"parentTextUnitCount\":162,\"fragmentIndex\":4,\"fragmentCount\":5,\"fragmentSourceUnitRange\":{\"start\":81,\"end\":95},\"previousGapUnits\":2,\"nextGapUnits\":null"));
    assert!(
        layer_tree.contains(
            "\"promotionBlockedReason\":\"document-text-fragment-style-boundary-unproven\""
        )
    );
    assert!(layer_tree.contains("\"fillColor\":\"#000080\""));
    assert!(layer_tree.contains("\"strokeColor\":\"#dddddd\""));
    assert!(layer_tree.contains("\"pathClosed\":true"));
    assert!(layer_tree.contains("\"strokeWidth\":0.139"));
    assert!(layer_tree.contains("\"strokeWidth\":0.500"));
    assert!(layer_tree.contains("\"strokeWidth\":2.250"));
    assert!(layer_tree.contains("\"pathPointCount\":2"));
    assert!(layer_tree.contains("\"pathPointCount\":3"));
    assert!(layer_tree.contains(
        "\"projectionViewport\":{\"x\":46.001,\"y\":38.700,\"width\":1021.318,\"height\":677.301}"
    ));
    assert!(layer_tree.contains(
        "\"projectionExtent\":{\"left\":-16154,\"top\":-16224,\"right\":-5612,\"bottom\":-9344}"
    ));

    let document_info = core.get_document_info();
    assert!(document_info.contains(
        "\"fdmOpenStrokeCohortSummary\":{\"projectionKind\":\"fdmOpenStrokeCohortSummary\""
    ));
    assert!(document_info.contains("\"openStrokeCount\":901"));
    assert!(document_info.contains("\"connectorCandidateCount\":67"));
    assert!(document_info.contains(
        "\"dominantConnectorRow\":{\"basis\":\"fdmOpenStrokeRowConnectorCandidateCount\",\"rowIndex\":0,\"connectorCandidateCount\":24"
    ));
    assert!(
        document_info
            .contains("{\"rowIndex\":33,\"openStrokeCount\":47,\"connectorCandidateCount\":14")
    );
    assert!(document_info.contains("\"connectorCandidateCount\":"));
    assert!(document_info.contains("\"connectorCandidates\":[{"));
    assert!(document_info.contains("\"candidateBasis\":\"long-open-source-path\""));
    assert!(document_info.contains("\"sourceEndpoints\":{\"start\":{\"x\":"));
    assert!(document_info.contains("\"endpointDistanceSquared\":"));
    assert!(document_info.contains("\"fillColor\":"));
    assert!(document_info.contains("\"strokeColor\":"));
    assert!(document_info.contains("\"pathSegmentCount\":"));
    assert!(document_info.contains("\"orthogonalSegmentCount\":"));
    assert!(document_info.contains("\"diagonalSegmentCount\":"));
    assert!(document_info.contains("\"compoundChildOffsetCount\":"));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-fdm-vector-primitives\""));
    assert!(!svg.contains("class=\"rjtd-column-grid-candidate\""));
    assert!(svg.contains("class=\"rjtd-fdm-vector-primitive\""));
    assert!(svg.contains("data-projection=\"fdmVectorPrimitiveReferenceProjection\""));
    assert!(svg.contains("data-geometry-decoded=\"true\""));
    assert!(svg.contains("data-renderable=\"true\""));
    assert!(svg.contains("data-marker-hex=\"ff000160\""));
    assert!(svg.contains("data-marker-hex=\"ff000460\""));
    assert!(svg.contains("data-marker-hex=\"ff000660\""));
    assert!(svg.contains("data-marker-hex=\"ff000960\""));
    assert!(svg.contains("data-marker-hex=\"00000460\""));
    assert!(svg.contains("data-marker-hex=\"00000660\""));
    assert!(svg.contains("data-marker-hex=\"00000960\""));
    assert!(svg.contains("data-primitive-kind=\"ellipse\""));
    assert!(svg.contains("data-primitive-kind=\"cubicBezier\""));
    assert!(svg.contains("<ellipse class=\"rjtd-fdm-vector-primitive\""));
    assert!(svg.contains("<path class=\"rjtd-fdm-vector-primitive\""));
    assert!(svg.contains(" C "));
    assert!(svg.contains("data-style-word=\"0x0088\""));
    assert!(svg.contains("data-fill-color=\"#ffffff\""));
    assert!(svg.contains("data-command-index=\"2007\""));
    assert!(svg.contains("data-render-fill-color=\"#000000\""));
    assert!(svg.contains(
        "data-fill-color=\"#000000\" data-render-fill-kind=\"solid\" data-render-fill-color=\"#ffffff\""
    ));
    assert!(svg.contains("data-render-fill-kind=\"linearGradient\""));
    assert!(svg.contains("<linearGradient id=\"rjtd-fdm-gradient-2-49001\""));
    assert!(svg.contains("<linearGradient id=\"rjtd-fdm-gradient-23-8001\""));
    assert!(svg.contains("stop-color=\"#003366\""));
    assert!(svg.contains("stop-color=\"#7a7acc\""));
    assert!(svg.contains("fill=\"url(#rjtd-fdm-gradient-2-49001)\""));
    assert!(svg.contains("fill=\"url(#rjtd-fdm-gradient-23-8001)\""));
    assert!(svg.contains("data-page-coverage-ratio=\""));
    assert!(svg.contains("data-viewport-coverage-ratio=\""));
    assert!(svg.contains("data-page-fill-candidate=\"false\""));
    assert!(!svg.contains("data-page-fill-candidate=\"true\""));
    assert!(svg.contains("data-page-fill-candidate-basis=\"closed-fill-and-large-span-filter\""));
    assert!(svg.contains("data-page-fill-candidate-reason=\""));
    assert!(svg.contains("data-page-fill-render-promotion-blocked-reason=\""));
    assert!(svg.contains("class=\"rjtd-fdm-vector-counter-overlay\""));
    assert!(!svg.contains("class=\"rjtd-shanai-lan-line-rules\""));
    assert!(!svg.contains("class=\"rjtd-shanai-lan-line-rule\""));
    assert!(!svg.contains("data-projection-kind=\"documentTextLineRuleProjection\""));
    assert!(!svg.contains("fdmConnectorCandidateDiagnostic"));
    assert!(!svg.contains("fdmOpenPathConnectorCandidateProjection"));
    assert!(!svg.contains("data-source=\"fdmVectorCommandConnectorCandidate\""));
    assert!(!svg.contains("class=\"rjtd-fdm-open-stroke-axis-rule-connector-readiness\""));
    assert!(!svg.contains("data-source=\"fdmOpenStrokeAxisRuleConnectorReadiness\""));
    assert!(svg.contains("data-row-index=\"17\" data-command-index=\"2009\""));
    assert!(svg.contains("data-render-stroke-color=\"#ffffff\""));
    assert!(svg.contains("data-stroke-width=\"0.139\""));
    assert!(svg.contains("stroke-width=\"0.500\""));
    assert!(svg.contains("data-path-closed=\"true\""));
    assert!(svg.contains("class=\"rjtd-shanai-lan-text-projection\""));
    assert!(svg.contains("data-projection-kind=\"documentTextGroupLineProjection\""));
    assert!(svg.contains("data-baseline-factor=\"0.800\""));
    assert!(svg.contains("data-group-index=\"15\""));
    assert!(svg.contains("data-split-from-text-run=\"true\""));
    assert!(svg.contains("data-parent-text-run-byte-range=\"12060..12384\""));
    assert!(svg.contains("data-parent-text-run-unit-range=\"6030..6192\""));
    assert!(svg.contains("data-fragment-index=\"4\""));
    assert!(svg.contains("data-fragment-count=\"5\""));
    assert!(svg.contains("data-fragment-source-unit-range=\"81..95\""));
    assert!(svg.contains("data-previous-gap-units=\"2\""));
    assert!(svg.contains(
        "data-fragment-style-blocked-reason=\"document-text-fragment-style-boundary-unproven\""
    ));
    assert!(svg.contains("data-line-header-raw-words-hex=\"0x001c,0x0030"));
    assert!(svg.contains("data-line-header-same-segment-run-present=\"true\""));
    assert!(svg.contains(
        "data-line-header-same-segment-run-start-group=\"1\" data-line-header-same-segment-run-end-group=\"6\" data-line-header-same-segment-run-group-count=\"6\" data-line-header-same-segment-run-position=\"1\" data-line-header-same-segment-run-text-slot-count=\"5\" data-line-header-same-segment-run-distinct-text-group-count=\"4\" data-line-header-same-segment-run-ambiguous-row-anchor=\"true\" data-line-header-y-placement-blocked-detail=\"same-segment-run-spans-multiple-visible-text-rows\" data-line-header-y-placement-blocked-reason=\"line-header-y-run-placement-semantics-unproven\""
    ));
    assert!(svg.contains(
        "data-line-header-same-segment-run-start-group=\"27\" data-line-header-same-segment-run-end-group=\"31\" data-line-header-same-segment-run-group-count=\"5\" data-line-header-same-segment-run-position=\"2\" data-line-header-same-segment-run-text-slot-count=\"4\" data-line-header-same-segment-run-distinct-text-group-count=\"2\""
    ));
    assert!(
        svg.contains(
            "data-fill-color-basis=\"document-text-style-property-15-text-run-candidate\""
        )
    );
    assert!(svg.contains("data-document-text-property-15-color-candidate=\"true\""));
    assert!(svg.contains("data-document-text-property-15-packed-bgr=\"0x00008000\""));
    assert!(svg.contains("data-style-link-proven=\"false\""));
    assert!(
        svg.contains("data-style-link-blocked-reason=\"document-view-style-group-link-unproven\"")
    );
    assert!(svg.contains("data-text-layout-style-record-count=\"0\""));
    assert!(svg.contains("data-document-view-style-group-count=\"9\""));
    assert!(svg.contains("data-document-view-style-group-candidate=\"3\""));
    assert!(svg.contains(
        "data-document-view-style-group-candidate-basis=\"document-text-position-count-tail-field-f7\""
    ));
    assert!(svg.contains("data-document-text-group-header-candidate=\"true\""));
    assert!(svg.contains("data-document-text-group-header-raw-words-hex=\"0x001c,0x0010,0x0013,0x0000,0x008f,0x0007,0x0118,0x0000,0x0052,0x0008,0x005b,0x0000,0x0068,0xffff,0x0000,0x0013,0x0000,0x0010,0x001f\""));
    assert!(svg.contains(
        "data-document-text-group-header-blocked-reason=\"document-text-group-header-semantics-unproven\""
    ));
    assert!(svg.contains("data-document-text-inline-style-candidate=\"true\""));
    assert!(svg.contains("data-document-text-inline-style-selector=\"0x0001\""));
    assert!(svg.contains("data-document-text-inline-style-raw-words-hex=\"0x001c,0x0001,0x0007,0x0000,0x0000,0x0001,0x001d,0x0002,0x001e,0x0005,0x0000,0x0001,0x001f\""));
    assert!(svg.contains("data-document-text-inline-style-blocked-reason=\"document-text-inline-control-semantics-unproven\""));
    assert!(svg.contains("data-document-text-property-15-role-decoded=\"false\""));
    assert!(svg.contains("data-text-count-range-evidence-count=\"3\""));
    assert!(svg.contains("data-text-count-range-indexes=\"0,1,2\""));
    assert!(svg.contains("data-text-count-range-bases=\"unit,unit,unit\""));
    assert!(svg.contains(">ルーター</text>"));
    assert!(svg.contains(">DHCP機能</text>"));
    assert!(svg.contains(">NASサーバ</text>"));
    assert!(svg.contains(">ｲﾝｸｼﾞｪｯﾄﾌﾟﾛｯﾀｰ</text>"));
    assert!(svg.contains("fill=\"#000000\""));
    assert!(svg.contains("fill=\"#008000\""));
    assert!(svg.contains("fill=\"#000080\""));
    assert!(svg.contains("data-row-index=\"23\""));
    assert!(svg.contains("data-row-index=\"33\""));
    assert!(!svg.contains("class=\"rjtd-fdm-command-diagnostics\""));
    assert!(!svg.contains("class=\"rjtd-fdm-frame-diagnostics\""));
}

#[test]
fn local_shanai_lan_reports_fdm_index_segment_axis_pair_gate_when_available() {
    let sample_path =
        local_samples_dir().join("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");
    if !sample_path.exists() {
        return;
    }

    let document = parse_document(&fs::read(sample_path).unwrap()).unwrap();
    let document_info = DocumentCore::from_document(document).get_document_info();

    assert!(document_info.contains("\"fdmIndexSegmentBboxAxisPairGate\":{\"source\":"));
    assert!(document_info.contains("\"linkedRowCount\":39"));
    assert!(document_info.contains("\"axisPairOrderAgreementRowCount\":39"));
    assert!(document_info.contains("\"axisPairOrderAgreementComplete\":true"));
    assert!(document_info.contains("\"decoded\":false"));
    assert!(document_info.contains("\"diagnosticOnly\":true"));
    assert!(document_info.contains("\"renderable\":false"));
    assert!(document_info.contains(
        "\"renderPromotionBlockedReason\":\"fdm-index-axis-pair-does-not-decode-page-transform-or-object-role\""
    ));
}

#[test]
fn fdm_connector_line_rule_endpoint_matches_horizontal_tight_span() {
    let projection = shanai_lan_line_rule_projection_fixture(vec![shanai_lan_line_rule_fixture(
        "horizontal",
        21,
        21,
        92,
        156,
    )]);
    let point = FdmConnectorTextGridPoint {
        x_units: 120.0,
        group_index_float: 21.75,
    };

    let matches = fdm_connector_line_rule_endpoint_matches(&projection, point);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].0, 0);
    assert_eq!(matches[0].3, "tight");
    assert!((matches[0].2.axis_delta - 0.75).abs() < 0.001);
    assert_eq!(matches[0].2.inline_delta, 0.0);
}

#[test]
fn fdm_compound_child_layout_requires_ordered_non_overlapping_declared_records() {
    let mut record = vec![0_u8; 80];
    record[..4].copy_from_slice(FDM_VECTOR_COMMAND_BBOX_MARKER);
    record[4..6].copy_from_slice(&80_u16.to_be_bytes());
    record[36..40].copy_from_slice(&0_u32.to_be_bytes());
    record[40..44].copy_from_slice(&0_u32.to_be_bytes());
    record[44..46].copy_from_slice(&48_u16.to_be_bytes());
    record[46..48].copy_from_slice(&64_u16.to_be_bytes());
    record[48..52].copy_from_slice(FDM_VECTOR_COMMAND_NESTED_LINE_MARKER);
    record[52..54].copy_from_slice(&16_u16.to_be_bytes());
    record[64..68].copy_from_slice(FDM_VECTOR_COMMAND_NESTED_LINE_MARKER);
    record[68..70].copy_from_slice(&16_u16.to_be_bytes());

    let layout = fdm_vector_compound_child_layout(&record).expect("valid compound layout");

    assert_eq!(layout.child_offsets(), &[48, 64]);
    assert!(layout.first_child_matches_prefix_end());
    assert!(layout.child_offsets_strictly_increasing());
    assert!(layout.child_records_fit_parent());
    assert!(layout.child_records_do_not_overlap());

    record[46..48].copy_from_slice(&60_u16.to_be_bytes());
    record[60..64].copy_from_slice(FDM_VECTOR_COMMAND_NESTED_LINE_MARKER);
    record[64..66].copy_from_slice(&16_u16.to_be_bytes());
    let overlapping_layout =
        fdm_vector_compound_child_layout(&record).expect("overlapping child table");

    assert!(!overlapping_layout.child_records_do_not_overlap());
    assert!(!overlapping_layout.is_valid_for_nested_projection());
}

#[test]
fn fdm_connector_line_rule_endpoint_matches_vertical_nearby_span() {
    let projection = shanai_lan_line_rule_projection_fixture(vec![shanai_lan_line_rule_fixture(
        "vertical", 10, 15, 84, 84,
    )]);
    let point = FdmConnectorTextGridPoint {
        x_units: 85.5,
        group_index_float: 12.25,
    };

    let matches = fdm_connector_line_rule_endpoint_matches(&projection, point);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].0, 0);
    assert_eq!(matches[0].3, "nearby");
    assert!((matches[0].2.axis_delta - 1.5).abs() < 0.001);
    assert_eq!(matches[0].2.inline_delta, 0.0);
}

#[test]
fn fdm_connector_line_rule_endpoint_matches_rejects_distant_points() {
    let projection = shanai_lan_line_rule_projection_fixture(vec![shanai_lan_line_rule_fixture(
        "horizontal",
        21,
        21,
        92,
        156,
    )]);

    let row_distant = FdmConnectorTextGridPoint {
        x_units: 120.0,
        group_index_float: 23.25,
    };
    let span_distant = FdmConnectorTextGridPoint {
        x_units: 159.5,
        group_index_float: 21.0,
    };

    assert!(fdm_connector_line_rule_endpoint_matches(&projection, row_distant).is_empty());
    assert!(fdm_connector_line_rule_endpoint_matches(&projection, span_distant).is_empty());
}

#[test]
fn fdm_connector_line_rule_endpoint_match_summary_blocks_single_endpoint() {
    let summary = FdmConnectorLineRuleEndpointMatchSummary {
        start_match_count: 1,
        end_match_count: 0,
        total_match_count: 1,
        tight_match_count: 1,
    };

    assert_eq!(summary.matched_endpoint_count(), 1);
    assert!(!summary.dual_endpoint_match());
    assert_eq!(
        summary.graph_promotion_blocked_reason(),
        "single-or-missing-endpoint-line-rule-match"
    );

    let none = FdmConnectorLineRuleEndpointMatchSummary {
        start_match_count: 0,
        end_match_count: 0,
        total_match_count: 0,
        tight_match_count: 0,
    };
    assert_eq!(
        none.graph_promotion_blocked_reason(),
        "no-thresholded-line-rule-endpoint-match"
    );

    let graph = FdmConnectorGraphDiagnosticSummary {
        connector_candidate_count: 67,
        line_rule_projection_count: 16,
        connector_endpoint_probe_count: 134,
        total_thresholded_endpoint_match_count: 10,
        matched_connector_count: 9,
        dual_endpoint_match_connector_count: 0,
        tight_endpoint_match_count: 2,
        nearby_endpoint_match_count: 8,
        no_thresholded_line_rule_endpoint_match_connector_count: 58,
        single_or_missing_endpoint_line_rule_match_connector_count: 9,
        connector_ownership_and_paint_order_unproven_connector_count: 0,
        ..Default::default()
    };
    assert_eq!(
        graph.render_promotion_blocked_reason(),
        "no-dual-endpoint-line-rule-match"
    );

    let graph_with_dual = FdmConnectorGraphDiagnosticSummary {
        dual_endpoint_match_connector_count: 1,
        ..graph
    };
    assert_eq!(
        graph_with_dual.render_promotion_blocked_reason(),
        "connector-ownership-grouping-and-paint-order-unproven"
    );

    let graph_with_axis_rule_dual = FdmConnectorGraphDiagnosticSummary {
        fdm_open_stroke_axis_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary {
            dual_endpoint_match_connector_count: 21,
            ..Default::default()
        },
        ..graph
    };
    assert_eq!(
        graph_with_axis_rule_dual.render_promotion_blocked_reason(),
        "same-row-axis-rule-parent-normalized-order-unproven"
    );

    let graph_with_axis_rule_owner_gate = FdmConnectorGraphDiagnosticSummary {
        fdm_open_stroke_axis_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary {
            dual_endpoint_match_connector_count: 21,
            ..Default::default()
        },
        fdm_open_stroke_axis_rule_owner_promotion_gate_summary:
            FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
                dual_endpoint_match_connector_count: 21,
                dual_endpoint_owner_candidate_count: 21,
                nearest_fdm_owner_rows_match_count: 21,
                nearest_fdm_owner_row_matches_connector_row_count: 21,
                after_owner_parent_command_span_count: 20,
                after_owner_parent_relative_offset_span_count: 20,
                before_owner_parent_command_span_count: 1,
                before_owner_parent_relative_offset_span_count: 1,
                ..Default::default()
            },
        ..graph
    };
    assert_eq!(
        graph_with_axis_rule_owner_gate.render_promotion_blocked_reason(),
        "connector-parent-command-outside-nearest-owner-parent-command-span"
    );
}

#[test]
fn fdm_connector_parent_normalized_order_requires_parent_relative_offset_between_nearest_owner_parents()
 {
    let mut summary = FdmConnectorEndpointOwnerMatchSummary {
        start_within_probe_count: 1,
        end_within_probe_count: 1,
        nearest_fdm_owner_rows_match: true,
        nearest_fdm_owner_row_matches_connector_row: true,
        connector_parent_command_between_nearest_fdm_owner_parent_commands: true,
        connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets: false,
        ..Default::default()
    };

    assert!(!summary.parent_normalized_ordered_same_row_same_connector());

    summary.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets = true;

    assert!(summary.parent_normalized_ordered_same_row_same_connector());

    let gate = FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
        dual_endpoint_match_connector_count: 1,
        dual_endpoint_owner_candidate_count: 1,
        nearest_fdm_owner_rows_match_count: 1,
        nearest_fdm_owner_row_matches_connector_row_count: 1,
        between_owner_parent_command_span_count: 1,
        between_owner_parent_relative_offset_span_count: 0,
        parent_normalized_ordered_same_row_same_connector_count: 0,
        ..Default::default()
    };
    assert_eq!(
        gate.parent_normalized_order_gate_blocked_reason(),
        "connector-parent-relative-offset-outside-nearest-owner-parent-relative-offset-span"
    );

    let partial_gate = FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_owner_candidate_count: 2,
        nearest_fdm_owner_rows_match_count: 2,
        nearest_fdm_owner_row_matches_connector_row_count: 2,
        between_owner_parent_command_span_count: 1,
        between_owner_parent_relative_offset_span_count: 1,
        parent_normalized_ordered_same_row_same_connector_count: 1,
        after_owner_parent_command_span_count: 1,
        after_owner_parent_relative_offset_span_count: 1,
        ..Default::default()
    };
    assert_eq!(
        partial_gate.parent_normalized_order_gate_blocked_reason(),
        "connector-parent-command-outside-nearest-owner-parent-command-span"
    );

    let axis_disagreement_gate = FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_owner_candidate_count: 2,
        nearest_fdm_owner_rows_match_count: 2,
        nearest_fdm_owner_row_matches_connector_row_count: 2,
        between_owner_parent_command_span_count: 2,
        between_owner_parent_relative_offset_span_count: 2,
        parent_normalized_ordered_same_row_same_connector_count: 1,
        ..Default::default()
    };
    assert_eq!(
        axis_disagreement_gate.parent_normalized_order_gate_blocked_reason(),
        "parent-command-source-order-axis-disagreement"
    );
}

#[test]
fn fdm_axis_rule_source_order_gate_classifies_parent_relative_offset_spans() {
    let no_dual = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary::default();
    assert_eq!(
        no_dual.axis_rule_source_order_gate_blocked_reason(),
        "no-same-row-axis-rule-dual-endpoint-match"
    );

    let missing = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_axis_rule_source_order_backed_connector_count: 1,
        ..Default::default()
    };
    assert_eq!(
        missing.axis_rule_source_order_gate_blocked_reason(),
        "axis-rule-source-order-evidence-missing"
    );

    let before = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_axis_rule_source_order_backed_connector_count: 2,
        dual_endpoint_connector_before_axis_rule_parent_span_count: 2,
        ..Default::default()
    };
    assert_eq!(
        before.axis_rule_source_order_gate_blocked_reason(),
        "connector-before-axis-rule-parent-span-paint-order-unproven"
    );

    let between = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_axis_rule_source_order_backed_connector_count: 2,
        dual_endpoint_connector_between_axis_rule_parent_span_count: 2,
        ..Default::default()
    };
    assert_eq!(
        between.axis_rule_source_order_gate_blocked_reason(),
        "connector-between-axis-rule-parent-span-paint-order-unproven"
    );

    let mixed = FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
        dual_endpoint_match_connector_count: 2,
        dual_endpoint_axis_rule_source_order_backed_connector_count: 2,
        dual_endpoint_connector_before_axis_rule_parent_span_count: 1,
        dual_endpoint_connector_between_axis_rule_parent_span_count: 1,
        ..Default::default()
    };
    assert_eq!(
        mixed.axis_rule_source_order_gate_blocked_reason(),
        "mixed-connector-axis-rule-parent-span-paint-order-unproven"
    );
}

#[test]
fn shanai_lan_table_candidate_exposes_sparse_border_diagnostics() {
    let sample_path =
        local_samples_dir().join("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");
    if !sample_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let mut core = DocumentCore::from_bytes(&bytes).unwrap();
    core.set_file_name(sample_path.to_string_lossy());

    let layer_tree = core.get_page_layer_tree(0).unwrap();

    assert!(layer_tree.contains("\"type\":\"documentTextSparseTableBorderTopologyDiagnostic\""));
    assert!(layer_tree.contains(
        "\"diagnosticOnly\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":true,\"geometryDecoded\":true,\"placementDerived\":true"
    ));
    assert!(layer_tree.contains("\"renderable\":true"));
    assert!(layer_tree.contains("\"blockers\":[]"));
    assert!(
        layer_tree.contains(
            "\"styleSectionCoverage\":{\"sectionPresent\":true,\"contentUnitCount\":6176"
        )
    );
    assert!(layer_tree.contains(
        "\"rowIndex\":0,\"groupIndex\":0,\"pairIndex\":0,\"edgeKind\":\"bottom\",\"stateCode\":8,\"stateCodeHex\":\"0x0008\",\"edgeStyleCode\":4"
    ));
    assert!(layer_tree.contains(
        "\"rowIndex\":2,\"groupIndex\":2,\"pairIndex\":0,\"edgeKind\":\"bottom\",\"stateCode\":8,\"stateCodeHex\":\"0x0008\",\"edgeStyleCode\":6"
    ));
    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("class=\"rjtd-document-text-sparse-table-borders\""));
    assert!(svg.contains("data-style-code=\"3\""));
    assert!(svg.contains("data-style-code=\"4\""));
    assert!(svg.contains("data-style-code=\"6\""));
    assert!(svg.contains("stroke-width=\"0.80\""));
    assert!(svg.contains("stroke-width=\"2.56\""));
    assert!(svg.contains("stroke-dasharray=\"3.2 3.2\""));
    for junction_x in [68, 83, 129, 134, 157, 182] {
        assert!(
            layer_tree.contains(&format!("\"xUnit\":{junction_x}")),
            "missing sparse vertical junction x={junction_x}: {layer_tree}"
        );
    }
    for midpoint_x in [68, 90, 129, 134, 157] {
        assert!(
            layer_tree.contains(&format!("\"midpointUnit\":{midpoint_x}")),
            "missing supporting cell-gap midpoint x={midpoint_x}: {layer_tree}"
        );
    }
}

#[test]
fn shanai_lan_border_probe_aligns_line_mark_record_with_group_index() {
    let sample_paths = [
        local_samples_dir().join("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd"),
        local_samples_dir().join("ichitaro-20030706232827-success-001-success_data-shanai_lan.jtd"),
    ];

    for (sample_index, sample_path) in sample_paths.into_iter().enumerate() {
        if !sample_path.exists() {
            return;
        }

        let bytes = fs::read(&sample_path).unwrap();
        let mut core = DocumentCore::from_bytes(&bytes).unwrap();
        core.set_file_name(sample_path.to_string_lossy());

        let layer_tree = core.get_page_layer_tree(0).unwrap();

        assert!(
            layer_tree.contains("\"type\":\"documentTextSparseTableBorderTopologyDiagnostic\"")
        );
        assert!(layer_tree.contains("\"rowIndex\":21,\"groupIndex\":21"));
        assert!(
            layer_tree.contains("\"lineMarkRecordIndex\":22,\"lineMarkRecordIndexDelta\":1")
                || layer_tree
                    .contains("\"lineMarkRecordIndex\":null,\"lineMarkRecordIndexDelta\":null")
        );
        if sample_index == 0 {
            assert!(layer_tree.contains("\"pageOriginAuthority\":\"source-backed\""));
            assert!(layer_tree.contains("\"renderable\":true"));
            assert!(layer_tree.contains("\"blockers\":[]"));
        } else {
            assert!(layer_tree.contains("\"pageOriginAuthority\":\"blocked\""));
            assert!(layer_tree.contains("\"renderable\":false"));
            assert!(layer_tree.contains("\"blockers\":[\"style-section-truncated\",\"source-page-transform-candidate-absent\"]"));
        }
        assert!(layer_tree.contains("\"stableGridExtentUnits\":280"));
        assert!(layer_tree.contains("\"rowIndex\":0,\"groupIndex\":0,\"pairIndex\":0,\"edgeKind\":\"bottom\",\"stateCode\":8,\"stateCodeHex\":\"0x0008\""));
        assert!(layer_tree.contains("\"rowIndex\":2,\"groupIndex\":2,\"pairIndex\":0,\"edgeKind\":\"bottom\",\"stateCode\":8,\"stateCodeHex\":\"0x0008\""));
        assert!(layer_tree.contains("\"rowIndex\":21,\"groupIndex\":21,\"pairIndex\":7,\"edgeKind\":\"bottom\",\"stateCode\":8,\"stateCodeHex\":\"0x0008\""));
    }
}

#[test]
fn shanai_lan_sparse_border_diagnostic_omits_truncated_row_tails_without_panicking() {
    let mut document_text = row_header_record_bytes(
        [0x0000, 0x008f, 0x0011, 0x0118, 0x0000, 0x0050],
        &[
            0x0008, 0x0003, 0x0013, 0x0000, 0x0000, 0x0046, 0x0013, 0x0000, 0x0000, 0x0017, 0x0021,
            0x0000, 0x0000, 0x0060, 0xffff, 0x0000,
        ],
    );
    document_text.extend_from_slice(&row_header_record_bytes(
        [0x0000, 0x008f, 0x000f, 0x0118, 0x0000, 0x0020],
        &[0x0023, 0x0000, 0x0000, 0x0020, 0x7777],
    ));
    let bytes = cfb_with_streams(&[(DOCUMENT_TEXT_PATH, &document_text)]);
    let document = parse_document(&bytes).unwrap();

    let diagnostic = shanai_lan_sparse_table_border_topology_diagnostic(&document).unwrap();

    assert_eq!(diagnostic.rows.len(), 1);
    assert_eq!(diagnostic.rows[0].group_index, 0);
    assert_eq!(diagnostic.rows[0].pairs[0].state_code, 0x0008);
    assert_eq!(diagnostic.rows[0].pairs[0].start_unit, 81);
    assert_eq!(diagnostic.rows[0].pairs[0].end_unit, 84);
}

#[test]
fn page_01_row_headers_preserve_proven_sparse_topology_state_families() {
    let sample_path =
        local_samples_dir().join("ichitaro-source-y-probe/corpus/page01-grid/PAGE 01.jtd");
    if !sample_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let payload = read_document_text_payload(&bytes).unwrap();
    let records = parse_document_text_row_headers(payload.bytes());
    let pair_shapes = records
        .iter()
        .filter(|record| record.fixed_fields().subtype() == 0x008f && record.geometry_complete())
        .map(|record| {
            record
                .pairs()
                .iter()
                .map(|pair| (pair.state_code(), pair.run_length()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert!(
        pair_shapes
            .iter()
            .any(|shape| shape.starts_with(&[(0x0016, 0), (0x0014, 22)]))
    );
    assert!(
        pair_shapes
            .iter()
            .any(|shape| shape.starts_with(&[(0x0017, 0), (0x0014, 22)]))
    );
    assert!(
        pair_shapes
            .iter()
            .any(|shape| shape.starts_with(&[(0x0015, 0), (0x0014, 22)]))
    );
    assert!(
        pair_shapes
            .iter()
            .any(|shape| shape.starts_with(&[(0x0013, 0), (0x0000, 22)]))
    );

    let mut core = DocumentCore::from_bytes(&bytes).unwrap();
    core.set_file_name(sample_path.to_string_lossy());
    let svg = core.render_page_svg(0).unwrap();
    assert!(!svg.contains("rjtd-document-text-sparse-table-borders"));
}

#[test]
fn source_page_transform_candidate_accepts_guarded_synthetic_old_format_fields() {
    let fields = [
        0u16, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 370, 105, 0, 0, 0, 0, 0, 0, 475,
    ];

    let candidate = shanai_lan_source_page_transform_candidate_from_raw_fields(
        0,
        29700,
        21000,
        1140 << 8,
        2130 << 8,
        1140 << 8,
        &fields,
    )
    .unwrap();

    assert_eq!(candidate.page_mark_entry_index, 0);
    assert_eq!(candidate.x_origin_left_mm100, 1140);
    assert_eq!(candidate.x_origin_right_mm100, 1140);
    assert_eq!(candidate.y_origin_mm100, 2130);
    assert_eq!(candidate.row_pitch_addend_a_mm100, 370);
    assert_eq!(candidate.row_pitch_addend_b_mm100, 105);
    assert_eq!(candidate.row_pitch_mm100, 475);
    assert_eq!(candidate.page_mark_w21_mm100, Some(475));
}

fn shanai_lan_line_rule_projection_fixture(
    rules: Vec<ShanaiLanLineRule>,
) -> ShanaiLanLineRuleProjection {
    ShanaiLanLineRuleProjection {
        source: "/DocumentText",
        projection_kind: "documentTextLineRuleProjection",
        line_mark_profile: SHANAI_LAN_LINE_MARK_PROFILE_ABSENT,
        line_mark_interval_count: 0,
        document_text_group_count: 0,
        document_text_line_header_count: 0,
        skipped_inline_line_header_count: 0,
        grid_unit_px: 3.0,
        line_height_px: 18.0,
        stroke_width: SHANAI_LAN_LINE_RULE_STROKE_WIDTH_PX,
        rules,
    }
}

fn shanai_lan_line_rule_fixture(
    orientation: &'static str,
    group_index: usize,
    end_group_index: usize,
    line_offset_units: u16,
    line_extent_units: u16,
) -> ShanaiLanLineRule {
    ShanaiLanLineRule {
        x1: 0.0,
        y1: 0.0,
        x2: 0.0,
        y2: 0.0,
        orientation,
        candidate_source: "test",
        source_span: TextSourceSpan::new(0, 0, 0, 0),
        group_index,
        end_group_index,
        line_offset_units,
        line_extent_units,
        line_header_hex: String::new(),
        line_header_raw_words: [0; 12],
        line_mark: None,
    }
}

#[test]
fn shanai_lan_text_fragments_use_wide_spaces_for_projection_spacing() {
    let fragments = shanai_lan_visible_text_fragments(
        "ﾌﾟﾘﾝﾄｻｰﾊﾞ \u{3000}\u{3000}\u{3000}\u{3000}\u{3000}ｸﾗｲｱﾝﾄ\n",
    );

    assert_eq!(fragments.len(), 2);
    assert_eq!(fragments[0].text, "ﾌﾟﾘﾝﾄｻｰﾊﾞ");
    assert_eq!(fragments[0].fragment_start_units, 0);
    assert_eq!(fragments[1].text, "ｸﾗｲｱﾝﾄ");
    assert_eq!(fragments[1].source_start_units, 15);
    assert_eq!(fragments[1].fragment_start_units, 20);

    let fragments = shanai_lan_visible_text_fragments(
        "\u{3000}\u{3000}\u{3000}\u{3000}  ﾍﾟﾝﾌﾟﾛｯﾀｰ        ｲﾝｸｼﾞｪｯﾄﾌﾟﾛｯﾀｰ          ｽｷｬﾅｰ      \u{3000}\u{3000}\u{3000}\u{3000}  ﾓﾉｸﾛﾚｰｻﾞｰﾌﾟﾘﾝﾀｰ  ｶﾗｰﾚｰｻﾞｰﾌﾟﾘﾝﾀｰ\n",
    );

    assert_eq!(fragments.len(), 5);
    assert_eq!(fragments[2].text, "ｽｷｬﾅｰ");
    assert_eq!(fragments[2].fragment_start_units, 41);
    assert_eq!(fragments[3].text, "ﾓﾉｸﾛﾚｰｻﾞｰﾌﾟﾘﾝﾀｰ");
    assert_eq!(fragments[3].source_start_units, 64);
    assert_eq!(fragments[3].fragment_start_units, 62);
    assert_eq!(fragments[4].text, "ｶﾗｰﾚｰｻﾞｰﾌﾟﾘﾝﾀｰ");
    assert_eq!(fragments[4].source_start_units, 81);
    assert_eq!(fragments[4].fragment_start_units, 79);
}

#[test]
fn shanai_lan_line_rules_use_only_skipped_inline_headers() {
    let layout = PageLayout::new(1122.5, 793.7);
    let mut bytes = Vec::new();
    extend_units(
        &mut bytes,
        &[
            0x001c, 0x0010, 0x0001, 0x0000, 0x008f, 0x0000, 0x0010, 0x001f, 0x001c, 0x0030, 0x000c,
            0x0000, 0x0000, 0x0118, 0x00ff, 0x0000, 0x000c, 0x0000, 0x0030, 0x001f, 0x0020, 0x0020,
            0x76ee, 0x3044, 0x000a,
        ],
    );
    extend_units(
        &mut bytes,
        &[
            0x001c, 0x0010, 0x0001, 0x0000, 0x008f, 0x0000, 0x0010, 0x001f, 0x001c, 0x001d, 0x001c,
            0x0030, 0x000c, 0x0000, 0x0056, 0x0058, 0x00ff, 0x0000, 0x000c, 0x0000, 0x0030, 0x001f,
            0x001c, 0x0030, 0x000c, 0x0000, 0x005c, 0x009c, 0x00ff, 0x0000, 0x000c, 0x0000, 0x0030,
            0x001f, 0x001e,
        ],
    );

    let projection = shanai_lan_document_text_line_rule_projection_from_bytes(
        &bytes,
        layout,
        &[],
        SHANAI_LAN_LINE_MARK_PROFILE_ABSENT,
    )
    .unwrap();

    assert_eq!(projection.rules.len(), 1);
    assert_eq!(projection.source, "/DocumentText");
    assert_eq!(projection.projection_kind, "documentTextLineRuleProjection");
    assert_eq!(
        projection.line_mark_profile,
        SHANAI_LAN_LINE_MARK_PROFILE_ABSENT
    );
    assert_eq!(projection.line_mark_interval_count, 0);
    assert_eq!(projection.document_text_group_count, 2);
    assert_eq!(projection.document_text_line_header_count, 3);
    assert_eq!(projection.skipped_inline_line_header_count, 1);
    let rule = &projection.rules[0];
    assert_eq!(rule.candidate_source, "skippedInlineLineHeader");
    assert_eq!(rule.group_index, 1);
    assert_eq!(rule.line_offset_units, 92);
    assert_eq!(rule.line_extent_units, 156);
    assert!((rule.x1 - 386.4).abs() < 0.2);
    assert!((rule.x2 - 623.3).abs() < 0.2);
    assert!((rule.y1 - 74.7).abs() < 0.2);
}

#[test]
fn shanai_lan_line_rules_add_vertical_runs_from_hidden_anchors() {
    let layout = PageLayout::new(1122.5, 793.7);
    let mut bytes = Vec::new();
    extend_units(
        &mut bytes,
        &[
            0x001c, 0x0010, 0x0001, 0x0000, 0x008f, 0x0000, 0x0010, 0x001f, 0x001c, 0x0030, 0x000c,
            0x0000, 0x0000, 0x0054, 0x00ff, 0x0000, 0x000c, 0x0000, 0x0030, 0x001f, 0x000a,
        ],
    );
    extend_units(
        &mut bytes,
        &[
            0x001c, 0x0010, 0x0001, 0x0000, 0x008f, 0x0000, 0x0010, 0x001f, 0x001c, 0x001d, 0x001c,
            0x0030, 0x000c, 0x0000, 0x0000, 0x0054, 0x00ff, 0x0000, 0x000c, 0x0000, 0x0030, 0x001f,
            0x001c, 0x0030, 0x000c, 0x0000, 0x005c, 0x009c, 0x00ff, 0x0000, 0x000c, 0x0000, 0x0030,
            0x001f, 0x001e,
        ],
    );

    let projection = shanai_lan_document_text_line_rule_projection_from_bytes(
        &bytes,
        layout,
        &[],
        SHANAI_LAN_LINE_MARK_PROFILE_ABSENT,
    )
    .unwrap();

    assert_eq!(
        projection
            .rules
            .iter()
            .filter(|rule| rule.orientation == "horizontal")
            .count(),
        2
    );
    let vertical = projection
        .rules
        .iter()
        .find(|rule| rule.orientation == "vertical" && rule.line_offset_units == 84)
        .unwrap();
    assert_eq!(vertical.group_index, 0);
    assert_eq!(vertical.end_group_index, 1);
    assert_eq!(
        vertical.candidate_source,
        "verticalAnchorRunFromLineHeaders"
    );
    assert_eq!(vertical.line_extent_units, 84);
    let expected_x = SHANAI_LAN_REFERENCE_CONTENT_LEFT_PX
        + 84.0 * SHANAI_LAN_REFERENCE_CONTENT_WIDTH_PX
            / (156.0 - f32::from(SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS));
    assert!((vertical.x1 - expected_x).abs() < 0.2);
    assert!((vertical.x2 - expected_x).abs() < 0.2);
    assert!((vertical.y1 - 56.7).abs() < 0.2);
    assert!((vertical.y2 - 74.7).abs() < 0.2);
}

#[test]
fn shanai_lan_line_mark_intervals_use_positive_deltas_after_header() {
    let mut bytes = Vec::new();
    extend_units(
        &mut bytes,
        &[
            0x0908, 0x0000, 0x0001, 0x0000, 0x0003, 0x0000, 0x0002, 0x0000, 0x0002, 0x007f, 0x8003,
            0x0071, 0x0002, 0xe7de, 0x0003,
        ],
    );

    let intervals = shanai_lan_line_mark_intervals_from_bytes(&bytes);

    assert_eq!(
        intervals,
        vec![
            ShanaiLanLineMarkInterval {
                record_index: 0,
                unit_start: 16,
                unit_end: 143,
                flag_word: 0x8003,
            },
            ShanaiLanLineMarkInterval {
                record_index: 1,
                unit_start: 143,
                unit_end: 256,
                flag_word: 0x0002,
            },
        ]
    );
}

#[test]
fn shanai_lan_line_mark_profile_distinguishes_observed_payload_families() {
    let mut be_delta_bytes = Vec::new();
    extend_units(
        &mut be_delta_bytes,
        &[
            0x0908, 0x0000, 0x0001, 0x0000, 0x0001, 0x0000, 0x0002, 0x0000, 0x0002, 0x007f, 0x8003,
        ],
    );
    assert_eq!(
        shanai_lan_line_mark_profile_from_bytes(&be_delta_bytes),
        SHANAI_LAN_LINE_MARK_PROFILE_BE_DELTA_V1
    );

    let mut macro_style_bytes = vec![0x1a, 0x00, 0x02, 0x01];
    for unit in "MacrosStreamStyle3".encode_utf16() {
        macro_style_bytes.extend_from_slice(&unit.to_le_bytes());
    }
    assert_eq!(
        shanai_lan_line_mark_profile_from_bytes(&macro_style_bytes),
        SHANAI_LAN_LINE_MARK_PROFILE_MACRO_STYLE
    );

    assert_eq!(
        shanai_lan_line_mark_profile_from_bytes(b"not-a-known-line-mark-profile"),
        SHANAI_LAN_LINE_MARK_PROFILE_UNPARSED
    );
}

#[test]
fn parser_preserves_frame_records_for_fdm_link_diagnostics() {
    let mut frame_payload = vec![
        0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x01, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x02,
    ];
    frame_payload.extend_from_slice(&frame_record_fixture(0, 0x0004, (11, 22, 33, 44)));
    frame_payload.extend_from_slice(&frame_record_fixture(1, 0x0007, (100, 200, 300, 400)));
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/Frame", &frame_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.object_frame_records().len(), 2);
    let record = &document.object_frame_records()[1];
    assert_eq!(record.source_path(), "/Frame");
    assert_eq!(record.row_index(), 1);
    assert_eq!(record.row_start(), 76);
    assert_eq!(record.record_len(), 60);
    assert_eq!(record.record_kind(), 0x0102);
    assert_eq!(record.declared_record_bytes(), 0x0038);
    assert_eq!(record.object_id(), 1);
    assert_eq!(record.object_type(), 0x0007);
    assert_eq!(record.x(), 100);
    assert_eq!(record.y(), 200);
    assert_eq!(record.width(), 300);
    assert_eq!(record.height(), 400);
    assert_eq!(record.corner_radius(), 0);
    assert_eq!(record.style_id(), 0);

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"objectFrameRecordCount\":2"));
    assert!(info.contains("\"objectFrameRecords\":["));
    assert!(info.contains("\"sourcePath\":\"/Frame\""));
    assert!(info.contains("\"rowIndex\":1"));
    assert!(info.contains("\"rowStart\":76"));
    assert!(info.contains("\"recordKindHex\":\"0x0102\""));
    assert!(info.contains("\"objectTypeHex\":\"0x0007\""));
    assert!(info.contains(
        "\"geometry\":{\"x\":100,\"y\":200,\"width\":300,\"height\":400,\"cornerRadius\":0}"
    ));
    assert!(info.contains("\"styleId\":0"));
}

#[test]
fn parser_limits_fdm_index_entries_to_declared_prefix_rows() {
    let mut index_payload = vec![0; FDM_INDEX_HEADER_BYTES];
    index_payload[..4].copy_from_slice(&[0x03, 0x0b, 0x00, 0x01]);
    index_payload[18..20].copy_from_slice(&1u16.to_be_bytes());
    push_fdm_index_row(&mut index_payload, 32, 0x0b00, (1, 2, 3, 4));
    push_fdm_index_row(&mut index_payload, 0xffff_fff0, 0xffff, (-1, -2, -3, -4));

    let mut vector_payload = vec![0xaa; 32];
    vector_payload.extend_from_slice(b"lead");
    let image_offset = vector_payload.len();
    vector_payload.extend_from_slice(b"\xff\xd8\xffpayload\xff\xd9");
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/FigureData/main_data/FDMIndex", &index_payload),
        ("/FigureData/main_data/FDMVector", &vector_payload),
    ]);

    let document = parse_document(&bytes).unwrap();

    let vector_candidate = document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == "/FigureData/main_data/FDMVector")
        .unwrap();
    assert_eq!(vector_candidate.fdm_index_entry_candidates().len(), 1);
    let entry = &vector_candidate.fdm_index_entry_candidates()[0];
    assert_eq!(entry.row_index(), 0);
    assert_eq!(entry.vector_offset(), 32);
    assert_eq!(entry.kind(), 0x0b00);
    assert_eq!(entry.image_signature_hits()[0].offset(), image_offset);
    assert_eq!(entry.segment_image_signature_hits()[0].offset(), 4);
}

#[test]
fn document_core_reports_object_stream_candidates_as_diagnostics() {
    let image_stream_path = "/EmbedItems/Embedding 3/Contents";
    let jpeg_payload = minimal_jpeg_payload();
    let (mut image_payload, signature_offset, _) =
        image_payload_with_header_fixture(jpeg_payload.len());
    image_payload.extend_from_slice(jpeg_payload);
    let figure_reference_payload = b"\x03\0\0\0ref\0\x03".to_vec();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (image_stream_path, &image_payload),
        ("/FigureData/main_data/FDMVector", &figure_reference_payload),
    ]);

    let core = DocumentCore::from_bytes(&bytes).unwrap();
    let info = core.get_document_info();
    let warnings = core.get_validation_warnings();

    assert!(info.contains("\"objectStreamCandidateCount\":2"));
    assert!(info.contains("\"path\":\"/EmbedItems/Embedding 3/Contents\""));
    assert!(info.contains("\"ownershipCandidate\":{\"basis\":\"stream-path\",\"family\":\"embed-items\",\"storagePath\":\"/EmbedItems/Embedding 3\",\"embeddingIndex\":3,\"streamRole\":\"contents\",\"decoded\":false}"));
    assert!(info.contains("\"ownershipReferences\":["));
    assert!(info.contains("\"targetPath\":\"/FigureData/main_data/FDMVector\""));
    assert!(info.contains("\"encoding\":\"u32-le\",\"totalMatches\":1,\"offsets\":[0]"));
    assert!(info.contains("\"frameReferenceRows\":[]"));
    assert!(info.contains("\"fdmIndexEntries\":[]"));
    assert!(info.contains(&format!(
        "\"imageSignatures\":[{{\"kind\":\"jpeg\",\"offset\":{signature_offset}}}]"
    )));
    assert!(info.contains(&format!(
        "\"imagePayloads\":[{{\"kind\":\"jpeg\",\"mime\":\"image/jpeg\",\"signatureOffset\":{signature_offset}"
    )));
    assert!(info.contains(&format!("\"declaredPayloadLength\":{}", jpeg_payload.len())));
    assert!(info.contains(&format!(
        "\"declaredPayloadLengthOffset\":{}",
        signature_offset - 4
    )));
    assert!(info.contains("\"sourcePathCandidate\""));
    assert!(info.contains("\"textLossy\":\"C:\\\\TEMP\\\\A.JPG\""));
    assert!(warnings.contains("\"JTD object stream candidate preserved as diagnostic data\":2"));
    assert!(warnings.contains("\"kind\":\"JtdObjectStreamCandidateDiagnosticOnly\""));
}

#[test]
fn parser_surfaces_control_range_evidence_as_validation_warning() {
    let position_table = text_count_table_fixture_with_ranges(&[(10, 14)]);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_with_control_boundary()),
        (
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            &position_table,
        ),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    let warnings = core.get_validation_warnings();

    assert!(
        warnings
            .contains("\"JTD text-count control-range overlap preserved as diagnostic data\":1")
    );
    assert!(warnings.contains("\"JTD text-boundary candidate preserved as diagnostic data\""));
    assert!(warnings.contains("\"kind\":\"JtdTextCountControlRangeDiagnosticOnly\""));
    assert!(warnings.contains("\"kind\":\"JtdTextBoundaryCandidateDiagnosticOnly\""));
}

#[test]
fn parser_preserves_multi_interval_table_candidates_as_diagnostics() {
    let position_table = text_count_table_fixture_with_ranges(&[(0, 30)]);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_with_control_boundary()),
        (
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            &position_table,
        ),
    ]);
    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.table_candidates().len(), 2);
    let byte_candidate = &document.table_candidates()[0];
    let unit_candidate = &document.table_candidates()[1];
    assert_eq!(
        byte_candidate.kind(),
        "multiIntervalControlRangeTableCandidate"
    );
    assert_eq!(byte_candidate.basis(), TextCountRangeOverlapBasis::Byte);
    assert_eq!(byte_candidate.interval_count(), 2);
    assert_eq!(byte_candidate.first_interval_index(), 0);
    assert_eq!(byte_candidate.last_interval_index(), 1);
    assert_eq!(
        byte_candidate.rule(),
        "control-delimited-text-count-range-with-multiple-intervals"
    );
    assert_eq!(unit_candidate.basis(), TextCountRangeOverlapBasis::Unit);
    assert_eq!(unit_candidate.interval_count(), 2);

    let byte_intervals = byte_candidate.intervals();
    assert_eq!(byte_intervals.len(), 2);
    assert_eq!(byte_intervals[0].index(), 0);
    assert_eq!(byte_intervals[0].source_interval_index(), 0);
    assert_eq!(byte_intervals[0].text_preview(), "銀河");
    assert_eq!(byte_intervals[0].line_break_count(), 0);
    assert!(byte_intervals[0].source_start() < byte_intervals[0].source_end());
    assert!(byte_intervals[0].source_start() >= byte_candidate.source_start());
    assert!(byte_intervals[0].source_end() <= byte_candidate.source_end());
    assert_eq!(byte_intervals[1].index(), 1);
    assert_eq!(byte_intervals[1].source_interval_index(), 1);
    assert_eq!(byte_intervals[1].text_preview(), "鉄道");
    assert_eq!(byte_intervals[1].line_break_count(), 0);
    assert!(byte_intervals[1].source_start() < byte_intervals[1].source_end());
    assert!(byte_intervals[1].source_start() >= byte_candidate.source_start());
    assert!(byte_intervals[1].source_end() <= byte_candidate.source_end());

    let unit_intervals = unit_candidate.intervals();
    assert_eq!(unit_intervals.len(), 2);
    assert_eq!(unit_intervals[0].source_interval_index(), 0);
    assert_eq!(unit_intervals[0].text_preview(), "銀河");
    assert_eq!(unit_intervals[1].source_interval_index(), 1);
    assert_eq!(unit_intervals[1].text_preview(), "鉄道");

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"tableCandidateCount\":2"));
    assert!(info.contains("\"tableCandidates\":[{\"index\":0"));
    assert!(info.contains("\"kind\":\"multiIntervalControlRangeTableCandidate\""));
    assert!(info.contains("\"textBoundaryCandidateIndex\":0"));
    assert!(info.contains("\"intervalCount\":2"));
    assert!(info.contains("\"intervals\":[{\"index\":0"));
    assert!(info.contains("\"sourceIntervalIndex\":0"));
    assert!(info.contains("\"textPreview\":\"銀河\""));
    assert!(info.contains("\"textPreview\":\"鉄道\""));
    assert!(info.contains("\"lineBreakCount\":0"));
    assert!(info.contains("\"columnSegments\":[]"));
    assert!(info.contains("\"cellLike\":true"));
    assert!(info.contains("\"rowLike\":true"));
    assert!(info.contains("\"observedTable\":{\"rowCount\":2,\"colCount\":1,\"cellCount\":2"));
    assert!(info.contains("\"columnSplitCandidateRows\":0"));
    assert!(info.contains("\"maxColumnSegmentCount\":0"));
    assert!(info.contains("\"columnSegmentPatternConsistent\":false"));
    assert!(info.contains("\"columnSegmentPatternMismatchRows\":0"));
    assert!(info.contains("\"columnGridCandidate\":null"));
    assert!(
        info.contains("\"rule\":\"control-delimited-text-count-range-with-multiple-intervals\"")
    );
    assert_eq!(
        core.get_table_dimensions(0, 0, 0).unwrap(),
        "{\"rowCount\":2,\"colCount\":1,\"cellCount\":2,\"source\":\"tableCandidate\",\"tableCandidateIndex\":0,\"basis\":\"byte\",\"delimiterCode\":28,\"delimiterCodeHex\":\"0x001c\",\"columnSplitCandidateRows\":0,\"maxColumnSegmentCount\":0,\"columnSegmentPatternConsistent\":false,\"columnSegmentPatternMismatchRows\":0,\"columnGridCandidate\":null,\"columnSplittingDecoded\":false,\"decoded\":false}"
    );

    let warnings = core.get_validation_warnings();
    assert!(warnings.contains("\"JTD table candidate preserved as diagnostic data\":2"));
    assert!(warnings.contains("\"kind\":\"JtdTableCandidateDiagnosticOnly\""));
}

#[test]
fn table_grid_cross_table_subrecord_ordering_helpers_detect_regressions() {
    assert!(usize_values_are_monotonic_non_decreasing(&[2, 2, 3]));
    assert!(!usize_values_are_monotonic_non_decreasing(&[3, 3, 2]));
    assert!(u16_values_are_monotonic_non_decreasing(&[85, 85, 242]));
    assert!(!u16_values_are_monotonic_non_decreasing(&[242, 85]));
    assert!(values_reused_after_different_value(&[174, 734, 174]));
    assert!(!values_reused_after_different_value(&[414, 414, 414]));

    let unstable_cross_table_hints = TableGridSourceGapToPageLineGapReadinessHints {
        transition_count: 3,
        same_page_mark_entry_transition_count: 3,
        all_transitions_same_page_mark_entry: true,
        source_range_gap_to_page_line_gap_max_abs_delta_units: Some(147),
        row_source_start_gap_to_page_line_gap_max_abs_delta_units: Some(674),
        segment_offset_gap_to_page_line_gap_max_abs_delta_units: Some(105),
        best_candidate_transform_kind: Some("segment-offset-gap"),
        best_candidate_max_abs_delta_units: Some(105),
        source_range_units_per_page_line_gap_spread: Some(12.25),
        row_source_start_units_per_page_line_gap_spread: Some(42.125),
        segment_offset_units_per_page_line_gap_spread: Some(29.875),
        affine_row_source_start_gap_fit: None,
    };
    assert_eq!(
        unstable_cross_table_hints.table_family_transform_blocked_reason(),
        Some("source-gap-to-page-line-gap-transform-unstable-across-table-family")
    );

    let tsaiten_affine_fit = affine_row_source_start_gap_fit(&[8, 2, 5], &[303, 160, 230], true)
        .expect("tsaiten transition gaps should fit a scoped affine candidate");
    assert_eq!(tsaiten_affine_fit.numerator_slope, 143);
    assert_eq!(tsaiten_affine_fit.denominator_slope, 6);
    assert_eq!(tsaiten_affine_fit.numerator_intercept, 671);
    assert_eq!(tsaiten_affine_fit.denominator_intercept, 6);
    assert!((tsaiten_affine_fit.max_abs_residual - 1.0).abs() < 0.001);
    assert_eq!(tsaiten_affine_fit.sample_count, 3);
    assert!(tsaiten_affine_fit.family_scoped);
    assert!(tsaiten_affine_fit.fit_stable);
    assert_eq!(
        tsaiten_affine_fit.blocked_reason(),
        "affine-row-source-start-gap-family-transform-authority-unproven"
    );

    assert_eq!(affine_row_source_start_gap_fit(&[], &[], true), None);
    assert_eq!(affine_row_source_start_gap_fit(&[1], &[10], true), None);
    assert_eq!(
        affine_row_source_start_gap_fit(&[1, 2], &[10, 20], true),
        None
    );
    assert_eq!(
        affine_row_source_start_gap_fit(&[1, 2, 3], &[10, 20], true),
        None
    );
    assert_eq!(
        affine_row_source_start_gap_fit(&[1, 2, 3, 4], &[10, 20, 30], true),
        None
    );
    assert_eq!(
        affine_row_source_start_gap_fit(&[1, 2, 3], &[10, 20, 30, 40], true),
        None
    );
    assert_eq!(
        affine_row_source_start_gap_fit(&[8, 2, 5], &[303, 160, 230], false),
        None
    );
    assert_eq!(
        affine_row_source_start_gap_fit(
            &[i32::MAX, i32::MAX - 1, i32::MAX - 2],
            &[i32::MAX, i32::MAX - 1, i32::MAX - 2],
            true
        ),
        None
    );
    assert_eq!(
        affine_row_source_start_gap_fit(&[i32::MIN, 0, i32::MAX], &[i32::MAX, 0, i32::MIN], true),
        None
    );

    let contradicted_affine_fit = affine_row_source_start_gap_fit(&[1, 2, 3], &[10, 20, 80], true)
        .expect("contradicted points should still emit a diagnostic fit");
    assert!(!contradicted_affine_fit.fit_stable);

    let absolute_y_slot_candidate = TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate {
        source: "rawRecordHeaderTailU16Subrecord",
        interpretation: "direct-u16-px",
        field_index: 2,
        tail_block16_word_index: Some(11),
        raw_record_scan_index: Some(2),
        raw_record_index: Some(2),
        byte_offset: 178,
        subrecord_byte_offset: 174,
        subrecord_line_start_candidate: 85,
        subrecord_line_end_candidate: 140,
        value: 768,
        value_px: 768.0,
    };
    let absolute_y_slot_disagreement = TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
        line_domain_y: Some(817.539),
        selected_span_units: Some(58),
        line_domain_projected_y: Some(875.539),
        candidates: vec![absolute_y_slot_candidate.clone()],
        best_absolute_y_slot: Some(absolute_y_slot_candidate),
        residual_px: Some(107.539),
        agrees: false,
    };
    assert_eq!(
        table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(
            &absolute_y_slot_disagreement
        ),
        "line-domain-projection-disagrees-with-page-mark-absolute-y-slot"
    );
}

#[test]
fn table_grid_source_gap_to_page_line_gap_readiness_hints_preserve_tsaiten_characterization() {
    let hints = table_grid_source_gap_to_page_line_gap_readiness_hints(Some(
        &test_table_grid_cross_table_row_boundary_offset_probe(
            Vec::new(),
            vec![
                test_table_grid_source_unit_to_page_line_index_piecewise_transition(190, 303, 8),
                test_table_grid_source_unit_to_page_line_index_piecewise_transition(72, 160, 2),
                test_table_grid_source_unit_to_page_line_index_piecewise_transition(138, 230, 5),
            ],
        ),
    ));

    assert_eq!(hints.transition_count, 3);
    assert_eq!(hints.same_page_mark_entry_transition_count, 3);
    assert!(hints.all_transitions_same_page_mark_entry);
    assert_eq!(
        hints.source_range_gap_to_page_line_gap_max_abs_delta_units,
        Some(182)
    );
    assert_eq!(
        hints.row_source_start_gap_to_page_line_gap_max_abs_delta_units,
        Some(295)
    );
    assert_eq!(
        hints.segment_offset_gap_to_page_line_gap_max_abs_delta_units,
        Some(105)
    );
    assert_eq!(
        hints.best_candidate_transform_kind,
        Some("segment-offset-gap")
    );
    assert_eq!(hints.best_candidate_max_abs_delta_units, Some(105));
    assert_eq!(
        hints.source_range_units_per_page_line_gap_spread,
        Some(12.25)
    );
    assert_eq!(
        hints.row_source_start_units_per_page_line_gap_spread,
        Some(42.125)
    );
    assert_eq!(
        hints.segment_offset_units_per_page_line_gap_spread,
        Some(29.875)
    );

    let affine_fit = hints
        .affine_row_source_start_gap_fit
        .expect("tsaiten transition gaps should preserve the affine diagnostic fit");
    assert_eq!(affine_fit.numerator_slope, 143);
    assert_eq!(affine_fit.denominator_slope, 6);
    assert_eq!(affine_fit.numerator_intercept, 671);
    assert_eq!(affine_fit.denominator_intercept, 6);
    assert_eq!(affine_fit.max_abs_residual_ceiling_units(), 1);
    assert!((affine_fit.max_abs_residual - 1.0).abs() < 0.001);
}

#[test]
fn max_abs_i32_handles_i32_min_conservatively() {
    assert_eq!(max_abs_i32(&[i32::MIN]), Some(i32::MAX));
    assert_eq!(max_abs_i32(&[i32::MIN, -4, 6]), Some(i32::MAX));
}

#[test]
fn table_grid_source_gap_to_page_line_gap_readiness_hints_handles_extreme_overflow_inputs() {
    let positive_overflow_hints = table_grid_source_gap_to_page_line_gap_readiness_hints(Some(
        &test_table_grid_cross_table_row_boundary_offset_probe(
            Vec::new(),
            vec![
                test_table_grid_source_unit_to_page_line_index_piecewise_transition(
                    0,
                    i32::MAX,
                    -1,
                ),
            ],
        ),
    ));
    let negative_overflow_hints = table_grid_source_gap_to_page_line_gap_readiness_hints(Some(
        &test_table_grid_cross_table_row_boundary_offset_probe(
            Vec::new(),
            vec![
                test_table_grid_source_unit_to_page_line_index_piecewise_transition(
                    0,
                    i32::MIN,
                    i32::MAX,
                ),
            ],
        ),
    ));

    assert_eq!(
        positive_overflow_hints.row_source_start_gap_to_page_line_gap_max_abs_delta_units,
        Some(i32::MAX)
    );
    assert_eq!(
        positive_overflow_hints.segment_offset_gap_to_page_line_gap_max_abs_delta_units,
        Some(i32::MAX)
    );
    assert_eq!(
        negative_overflow_hints.row_source_start_gap_to_page_line_gap_max_abs_delta_units,
        Some(i32::MAX)
    );
    assert_eq!(
        negative_overflow_hints.segment_offset_gap_to_page_line_gap_max_abs_delta_units,
        Some(i32::MAX)
    );
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
fn push_table_grid_piecewise_record_family_gap_transition_json_saturates_opposite_signed_family_gap_delta_overflow()
 {
    let positive_previous = test_table_grid_cross_table_row_boundary_offset_table(
        0,
        0,
        0,
        vec![usize::MAX],
        vec![0],
        vec![0],
    );
    let positive_next = test_table_grid_cross_table_row_boundary_offset_table(
        1,
        0,
        0,
        vec![0],
        vec![usize::MAX],
        vec![0],
    );
    let negative_previous = test_table_grid_cross_table_row_boundary_offset_table(
        0,
        0,
        0,
        vec![0],
        vec![usize::MAX],
        vec![0],
    );
    let negative_next = test_table_grid_cross_table_row_boundary_offset_table(
        1,
        0,
        0,
        vec![usize::MAX],
        vec![0],
        vec![0],
    );

    let mut positive_output = String::new();
    push_table_grid_piecewise_record_family_gap_transition_json(
        &mut positive_output,
        &positive_previous,
        &positive_next,
    );
    let mut negative_output = String::new();
    push_table_grid_piecewise_record_family_gap_transition_json(
        &mut negative_output,
        &negative_previous,
        &negative_next,
    );

    assert!(
        positive_output.contains("\"previousFamilyRecordGap\":-2147483648"),
        "{positive_output}"
    );
    assert!(
        positive_output.contains("\"selectedFamilyRecordGap\":2147483647"),
        "{positive_output}"
    );
    assert!(
        positive_output.contains("\"selectedMinusPreviousFamilyRecordGapDelta\":2147483647"),
        "{positive_output}"
    );
    assert!(
        negative_output.contains("\"previousFamilyRecordGap\":2147483647"),
        "{negative_output}"
    );
    assert!(
        negative_output.contains("\"selectedFamilyRecordGap\":-2147483648"),
        "{negative_output}"
    );
    assert!(
        negative_output.contains("\"selectedMinusPreviousFamilyRecordGapDelta\":-2147483648"),
        "{negative_output}"
    );
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

#[test]
fn push_table_grid_source_only_page_y_transition_semantics_readiness_json_preserves_tsaiten_characterization()
 {
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
                190,
                190,
                vec![8],
                vec![8],
                vec![303],
            ),
            test_table_grid_cross_table_row_boundary_offset_table(
                2,
                262,
                262,
                vec![10],
                vec![10],
                vec![463],
            ),
            test_table_grid_cross_table_row_boundary_offset_table(
                3,
                400,
                400,
                vec![15],
                vec![15],
                vec![693],
            ),
        ],
        vec![
            test_table_grid_source_unit_to_page_line_index_piecewise_transition(190, 303, 8),
            test_table_grid_source_unit_to_page_line_index_piecewise_transition(72, 160, 2),
            test_table_grid_source_unit_to_page_line_index_piecewise_transition(138, 230, 5),
        ],
    );
    let mut output = String::new();

    push_table_grid_source_only_page_y_transition_semantics_readiness_json(
        &mut output,
        Some(&probe),
        3,
    );

    assert!(output.contains("\"selectedMinusPreviousFamilyRecordGapDeltas\":[0,0,0]"));
    assert!(output.contains("\"sourceRangeGapMinusPageLineGapUnits\":[182,70,133]"));
    assert!(output.contains("\"rowSourceStartGapMinusPageLineGapUnits\":[295,158,225]"));
    assert!(output.contains("\"segmentOffsetGapMinusPageLineGapUnits\":[105,86,87]"));
    assert!(output.contains("\"bestCandidateTransformKind\":\"segment-offset-gap\""));
    assert!(output.contains("\"bestCandidateMaxAbsDeltaUnits\":105"));
    assert!(output.contains("\"numeratorSlope\":143"));
    assert!(output.contains("\"denominatorSlope\":6"));
    assert!(output.contains("\"numeratorIntercept\":671"));
    assert!(output.contains("\"denominatorIntercept\":6"));
    assert!(output.contains("\"maxAbsResidual\":1.000"));
}

#[test]
fn push_table_grid_source_only_page_y_transition_semantics_readiness_json_handles_selected_minus_previous_family_gap_overflow()
 {
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
                0,
                0,
                vec![0],
                vec![usize::MAX],
                vec![0],
            ),
            test_table_grid_cross_table_row_boundary_offset_table(
                2,
                0,
                0,
                vec![0],
                vec![0],
                vec![0],
            ),
        ],
        vec![
            test_table_grid_source_unit_to_page_line_index_piecewise_transition(0, 0, -i32::MAX),
            test_table_grid_source_unit_to_page_line_index_piecewise_transition(0, 0, i32::MAX),
        ],
    );
    let mut output = String::new();

    push_table_grid_source_only_page_y_transition_semantics_readiness_json(
        &mut output,
        Some(&probe),
        2,
    );

    assert!(
        output.contains("\"previousFamilyRecordGaps\":[-2147483647,2147483647]"),
        "{output}"
    );
    assert!(
        output.contains("\"selectedFamilyRecordGaps\":[2147483647,-2147483648]"),
        "{output}"
    );
    assert!(
        output.contains("\"selectedMinusPreviousFamilyRecordGapDeltas\":[2147483647,-2147483648]"),
        "{output}"
    );
}

#[test]
fn push_table_grid_source_only_page_y_transition_semantics_readiness_json_handles_direct_gap_overflow()
 {
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
                0,
                0,
                vec![0],
                vec![0],
                vec![0],
            ),
        ],
        vec![
            test_table_grid_source_unit_to_page_line_index_piecewise_transition(0, i32::MAX, -1),
            test_table_grid_source_unit_to_page_line_index_piecewise_transition(
                0,
                i32::MIN,
                i32::MAX,
            ),
        ],
    );
    let mut output = String::new();

    push_table_grid_source_only_page_y_transition_semantics_readiness_json(
        &mut output,
        Some(&probe),
        2,
    );

    assert!(output.contains("\"rowSourceStartGapMinusPageLineGapUnits\":[2147483647,-2147483648]"));
    assert!(output.contains("\"segmentOffsetGapMinusPageLineGapUnits\":[2147483647,-2147483648]"));
}

#[test]
fn local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available()
{
    let sample_path = local_samples_dir().join("ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd");
    let reference_pdf_path = sample_path.with_extension("pdf");
    if !sample_path.exists() || !reference_pdf_path.exists() {
        return;
    }

    let bytes = fs::read(&sample_path).unwrap();
    let document = parse_document(&bytes).unwrap();
    let direct_candidates = document
        .table_candidates()
        .iter()
        .filter(|candidate| candidate.kind() == "documentTextControlRunTableCandidate")
        .collect::<Vec<_>>();

    assert!(direct_candidates.len() >= 2);
    let scoring_table = direct_candidates[0];
    assert_eq!(
        scoring_table.rule(),
        "document-text-001c-cells-with-000e-row-breaks"
    );
    assert_eq!(scoring_table.basis(), TextCountRangeOverlapBasis::Unit);
    assert_eq!(scoring_table.delimiter_code(), TABLE_ROW_DELIMITER_CONTROL);
    assert_eq!(scoring_table.interval_count(), 4);
    assert_eq!(
        scoring_table
            .column_segment_grid_candidate()
            .unwrap()
            .column_count(),
        3
    );
    assert_eq!(
        scoring_table.intervals()[0].text_preview(),
        "級\t配点\t合格点"
    );
    assert_eq!(
        scoring_table.intervals()[1].text_preview(),
        "３級\t250点\t235点以上"
    );

    let mut renamed_core = DocumentCore::from_document(document.clone());
    renamed_core.set_file_name("renamed-tsaiten.jtd");
    assert_eq!(renamed_core.writing_mode(), WritingMode::Horizontal);
    assert!((renamed_core.page_width_px() - 793.7).abs() < 0.2);
    assert!((renamed_core.page_height_px() - 1122.5).abs() < 0.2);
    assert!(
        renamed_core
            .render_page_svg(0)
            .unwrap()
            .contains("data-projection=\"tsaitenReferenceProjection\"")
    );
    let tsaiten_page_layout = PageLayout::new(
        renamed_core.page_width_px() as f32,
        renamed_core.page_height_px() as f32,
    );
    let renamed_lines = renamed_core.page_lines(0).unwrap();
    let scoring_column_count = scoring_table
        .column_segment_grid_candidate()
        .unwrap()
        .column_count();
    let scoring_generic_overlay = table_grid_overlay_layout(
        tsaiten_page_layout,
        &document,
        renamed_lines,
        0,
        scoring_table,
        scoring_column_count,
    );
    let scoring_legacy_reference_overlay = tsaiten_table_grid_overlay_layout(
        tsaiten_page_layout,
        &document,
        scoring_table,
        scoring_column_count,
    )
    .unwrap();
    assert!((scoring_generic_overlay.0 - tsaiten_page_layout.margin_px()).abs() < 0.001);
    assert!((scoring_generic_overlay.2 - tsaiten_page_layout.body_width_px()).abs() < 0.001);
    assert!((scoring_generic_overlay.0 - scoring_legacy_reference_overlay.0).abs() > 1.0);
    assert!((scoring_generic_overlay.2 - scoring_legacy_reference_overlay.2).abs() > 1.0);

    let mut core = DocumentCore::from_document(document);
    core.set_file_name(sample_path.to_string_lossy());
    let info = core.get_document_info();
    assert!(info.contains("\"kind\":\"documentTextControlRunTableCandidate\""));
    assert!(info.contains("\"rule\":\"document-text-001c-cells-with-000e-row-breaks\""));
    assert!(info.contains("\"textPreview\":\"級\\t配点\\t合格点\""));
    assert!(info.contains("\"family\":\"count-plus-one-variable\""));
    assert!(info.contains(
        "\"u16SubrecordScan\":{\"source\":\"/PageMark raw u16 subrecord scan\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false"
    ));
    assert!(info.contains(
        "\"entryRelativeByteOffset\":162,\"streamByteOffset\":174,\"wordIndex\":81,\"words\":[2,5,768,0,85,0,140,0],\"wordsHex\":[\"0x0002\",\"0x0005\",\"0x0300\",\"0x0000\",\"0x0055\",\"0x0000\",\"0x008c\",\"0x0000\"]"
    ));
    assert!(info.contains(
        "\"entryRelativeByteOffset\":48,\"streamByteOffset\":334,\"wordIndex\":24,\"words\":[4,1,768,0,192,0,241,0],\"wordsHex\":[\"0x0004\",\"0x0001\",\"0x0300\",\"0x0000\",\"0x00c0\",\"0x0000\",\"0x00f1\",\"0x0000\"]"
    ));
    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"projectionKind\":\"tsaitenReferenceProjection\""));
    assert!(layer_tree.contains("\"role\":\"document-heading\""));
    assert!(layer_tree.contains("\"role\":\"title-box\""));
    assert!(layer_tree.contains("\"role\":\"document-format-table\""));
    assert!(layer_tree.contains("\"text\":\"＜採点原則＞\""));
    assert!(layer_tree.contains("\"text\":\"タイピング科目採点方法\""));
    assert!(layer_tree.contains("\"type\":\"tableGridCandidate\""));
    assert!(layer_tree.contains("\"projectionKind\":\"tableProjection\""));
    assert!(layer_tree.contains("\"referenceBacked\":true"));
    assert!(layer_tree.contains("\"bbox\":{\"x\":174.000,\"y\":301.005"));
    assert!(layer_tree.contains("\"bbox\":{\"x\":174.000,\"y\":768.014"));
    assert!(layer_tree.contains(
        "\"columnWidthBasis\":\"documentTextLineHeaderCellSlotUnits\",\"columnWidths\":[378.342,175.659]"
    ));
    assert!(layer_tree.contains("\"colCountCandidate\":3"));
    assert!(layer_tree.contains("\"cells\":["));
    assert!(layer_tree.contains("\"text\":\"級\""));
    assert!(layer_tree.contains("\"text\":\"235点以上\""));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidate\":{\"source\":\"documentTextLineHeaders+fallbackTextAnchors\",\"sourceBacked\":true,\"referenceBacked\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutReadiness\":{\"source\":\"sourceDerivedLayoutGate+documentTextLineHeaders+/LineMark\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"sourcePlacementEvidencePresent\":false,\"candidateRowCount\":4,\"requestedColumnCount\":3,\"lineHeaderRowCount\":4,\"rawHeaderCount\":2,\"matchedRowCount\":0,\"fullMatchedRowCount\":0,\"matchedCellHeaderCount\":0,\"requiredCellHeaderCount\":12,\"commonMatchedColumnCount\":0,\"rowsWithoutHeaders\":[1,2],\"rowsWithoutMatchedCellHeaders\":[0,1,2,3],\"rowsWithPartialCellHeaderCoverage\":[],\"lineHeaderRowsHomogeneous\":false,\"lineMarkRowRecordSelection\":\"previous-compact-row-span-record\",\"lineMarkRowsExactAndContiguous\":false,\"sourceDerivedLayoutCandidatePresent\":true,\"sourceDerivedLayoutRenderable\":false,\"sourceDerivedLayoutBlockedReason\":\"sparse-sibling-derived-candidate-render-ineligible\""
    ));
    assert!(layer_tree.contains(
        "\"referenceFallbackAdmissionGate\":{\"source\":\"table_grid_reference_layout_visible_fallback_allowed+sourceOnlyPageYRenderAdmissionGate\",\"diagnosticOnly\":true"
    ));
    assert_eq!(
        layer_tree.matches("\"referenceFallbackUsed\":true").count(),
        2
    );
    assert!(layer_tree.contains(
        "\"referenceLayoutPresent\":true,\"referenceFallbackAllowed\":true,\"referenceFallbackUsed\":true,\"sourceLayoutCandidatePresent\":true,\"sourceRenderLayoutPresent\":false,\"sourceLayoutRenderable\":false,\"sourceOnlyPageYAdmissionReady\":false,\"sourceOnlyPageYAdmissionBasis\":null,\"sourceReplacementBlockedReason\":\"source-derived-layout-not-renderable\""
    ));
    assert!(layer_tree.contains(
        "\"referenceLayoutPresent\":true,\"referenceFallbackAllowed\":true,\"referenceFallbackUsed\":true,\"sourceLayoutCandidatePresent\":true,\"sourceRenderLayoutPresent\":false,\"sourceLayoutRenderable\":false,\"sourceOnlyPageYAdmissionReady\":false,\"sourceOnlyPageYAdmissionBasis\":null,\"sourceReplacementBlockedReason\":\"source-page-y-render-admission-not-ready\""
    ));
    assert!(layer_tree.contains(
        "\"rejectionReasons\":[\"source-placement-evidence-missing\",\"line-header-cell-geometry-incomplete\",\"no-common-matched-cell-header-columns\",\"line-header-rows-not-homogeneous\",\"line-mark-rows-not-exact-source-boundaries\",\"sparse-sibling-derived-candidate-render-ineligible\",\"source-derived-layout-not-renderable\"],\"renderPromotionContribution\":\"source-derived-layout-readiness-gate\",\"renderPromotionBlockedReason\":\"source-derived-layout-not-renderable\""
    ));
    assert!(layer_tree.contains(
        "\"pageSpaceSolver\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\",\"solverVersion\":\"table-page-space-v1\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"solverStage\":\"blocked-horizontal-transform\",\"sourcePlacementEvidencePresent\":false,\"candidateRowCount\":4,\"requestedColumnCount\":3,\"commonMatchedColumnCount\":0,\"matchedCellHeaderCount\":0,\"requiredCellHeaderCount\":12"
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidatePresent\":true,\"sourceDerivedLayoutRenderable\":false,\"pageOriginAuthority\":\"none\",\"lineMarkPageOriginPresent\":false,\"lineMarkPageOriginStridePresent\":false"
    ));
    assert!(layer_tree.contains(
        "\"referenceCalibrationReplacementGate\":{\"source\":\"table-page-space-v1 reference calibration replacement gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"replacementReady\":false,\"sourceLayoutCandidatePresent\":true,\"sourceLayoutRenderable\":false,\"horizontalSolverReady\":false,\"sourceColumnSplitReady\":false,\"pageSpaceHorizontalTransformReady\":false,\"rowHeightSolverReady\":false,\"yOriginSolverReady\":false"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"table-horizontal-source-transform-incomplete\",\"source-column-split-not-ready\",\"table-horizontal-page-space-transform-incomplete\",\"table-row-height-source-transform-incomplete\",\"source-page-y-transform-not-decoded\",\"source-derived-layout-not-renderable\"],\"renderPromotionContribution\":\"reference-calibration-replacement-gate\",\"renderPromotionBlockedReason\":\"source-table-page-space-not-ready\""
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyAxisAdmissionGate\":{\"source\":\"pageSpaceHorizontalTransformGate+sourcePageYTransformGate source-only selector coupling\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"admissionReady\":false,\"activeSourceLayoutAdmissionReady\":false,\"activeSourceLayoutAdmissionBasis\":null,\"sourceOnlySelectorFallbackIgnoredByActiveSourceLayout\":false,\"sourceLayoutCandidatePresent\":true,\"sourceLayoutRenderable\":false,\"horizontalAxisReady\":false,\"horizontalSelectorCandidatePresent\":true,\"horizontalSelectorInBestAgreementGroup\":true,\"horizontalCandidateCount\":4,\"horizontalAgreementGroupCount\":3,\"horizontalBestSupportCount\":2,\"horizontalUniqueBestSupported\":true,\"horizontalBestSupportedSelectedX\":174.000,\"horizontalBestSupportedSelectedWidth\":421.000"
    ));
    assert!(layer_tree.contains(
        "\"horizontalBestSupportedFrameBases\":[\"page-mark-word14-first-slot-word15-half-gap\",\"page-mark-word14-first-slot-word15-half-gap\"],\"yAxisReady\":false,\"ySelectorCandidatePresent\":true,\"ySelectorSingleSupportFallback\":false,\"ySelectorSupportFragmentedByTable\":true,\"ySelectorSupportCount\":2,\"ySelectorCrossTableSupportPresent\":true,\"ySelectorAgreementAdmissible\":false,\"ySelectorAdmissionBlockedReason\":\"source-y-origin-selector-fragmented-by-table-not-render-admissible\",\"ySelectorSupportBlockedReasons\":[\"cross-table-row-boundary-offset-transform-required\",\"page-line-gap-projection-does-not-decode-table-y-origin\"],\"sourceGapToPageLineGapTransformAdmissionGate\":{\"source\":\"sourceOnlyAxisAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate\""
    ));
    assert!(layer_tree.contains(
        "\"pageMarkAbsoluteYSlotSemanticsReady\":false,\"pageMarkAbsoluteYSlotBlockedReason\":\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"pageMarkAbsoluteYSlotResidualPx\":-723.913,\"yCandidateCount\":9,\"yAgreementGroupCount\":8,\"yBestSupportCount\":2,\"yUniqueBestSupported\":true,\"ySelectedOriginBasis\":\"cross-table-combined-previous-row-span-first-record\",\"ySelectedY\":235.087,\"ySelectedRowHeight\":23.298,\"ySelectorTableCandidateIndexes\":[0]"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyAxisCandidateBBox\":{\"source\":\"sourceOnlyAxisAdmissionGate.sourceOnlyAxisCandidateBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"candidatePresent\":true,\"bboxPresent\":true,\"horizontalCandidatePresent\":true,\"yCandidatePresent\":true,\"rowHeightCandidatePresent\":true,\"rowCount\":4,\"horizontalFrameBasis\":\"page-mark-word14-first-slot-word15-half-gap\",\"yOriginBasis\":\"cross-table-combined-previous-row-span-first-record\",\"rowHeight\":23.298,\"bbox\":{\"x\":174.000,\"y\":235.087,\"width\":421.000,\"height\":93.192}"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"source-column-split-not-ready\",\"source-row-height-not-ready\",\"source-horizontal-axis-not-render-admissible\",\"source-y-origin-selector-fragmented-by-table\",\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"source-y-axis-not-render-admissible\",\"source-derived-layout-not-renderable\"],\"renderPromotionContribution\":\"source-only-axis-selector-admission-gate\",\"renderPromotionBlockedReason\":\"source-page-space-axis-selector-coupling-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"pageSpaceHorizontalTransformGate\":{\"source\":\"documentTextLineHeaders+/LineMark page-space horizontal transform gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":false,\"sourceLayoutCandidatePresent\":true,\"sourceColumnSplitReady\":false,\"xUnitAllRowsAgree\":false,\"fullExtentUnitsPresent\":false,\"sourceFrameDecoded\":false,\"pageOriginAuthority\":\"none\""
    ));
    assert!(layer_tree.contains(
        "\"sourceFrameHypotheses\":[{\"frameBasis\":\"page-mark-word14-first-slot-word15-direct\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateBasis\":\"page-mark-u16-fields+cross-table-line-header-slots\",\"pageMarkXWord14\":194,\"pageMarkWidthWord\":423,\"firstColumnSlotUnits\":20,\"xAdjustmentUnits\":20.000,\"widthAdjustmentUnits\":0.000,\"xAdjustmentBasis\":\"cross-table-first-column-slot\",\"widthAdjustmentBasis\":\"none\",\"selectedX\":174.000,\"selectedWidth\":423.000"
    ));
    assert!(layer_tree.contains(
        "{\"frameBasis\":\"page-mark-word14-first-slot-word15-half-gap\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateBasis\":\"page-mark-u16-fields+cross-table-line-header-slots\",\"pageMarkXWord14\":194,\"pageMarkWidthWord\":423,\"firstColumnSlotUnits\":20,\"xAdjustmentUnits\":20.000,\"widthAdjustmentUnits\":2.000,\"xAdjustmentBasis\":\"cross-table-first-column-slot\",\"widthAdjustmentBasis\":\"cross-table-half-first-intercell-gap\",\"selectedX\":174.000,\"selectedWidth\":421.000"
    ));
    assert!(layer_tree.contains(
        "\"sourceFrameCandidateAgreementGate\":{\"source\":\"pageSpaceHorizontalTransformGate.sourceFrameHypotheses agreement\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"selectionReady\":false,\"candidateCount\":4,\"agreementGroupCount\":3,\"bestSupportCount\":2,\"uniqueBestSupported\":true,\"sourceOnlyUniqueSelectionCandidatePresent\":true,\"sourceOnlyUniqueSelectionDiagnosticOnly\":true,\"sourceOnlyUniqueSelectionPromotionReady\":false,\"sourceOnlyUniqueSelectionPromotionBlockedReason\":\"source-horizontal-field-semantics-unproven\",\"bestSupportedSelectedX\":174.000,\"bestSupportedSelectedWidth\":421.000,\"bestSupportedFrameBases\":[\"page-mark-word14-first-slot-word15-half-gap\",\"page-mark-word14-first-slot-word15-half-gap\"]"
    ));
    assert!(layer_tree.contains(
        "{\"selectedX\":174.000,\"selectedWidth\":421.000,\"supportCount\":2,\"frameBases\":[\"page-mark-word14-first-slot-word15-half-gap\",\"page-mark-word14-first-slot-word15-half-gap\"],\"contributions\":[\"source-only-horizontal-field-consensus\",\"source-only-horizontal-field-selector\"]"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"source-horizontal-field-semantics-still-unproven\"],\"renderPromotionContribution\":\"source-horizontal-frame-candidate-agreement-gate\",\"renderPromotionBlockedReason\":\"source-horizontal-field-semantics-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"source-column-split-not-ready\",\"source-x-unit-range-not-row-stable\",\"source-full-line-extent-units-missing\",\"page-space-horizontal-frame-not-decoded\",\"line-mark-rows-not-exact-source-boundaries\",\"sparse-sibling-derived-candidate-render-ineligible\"],\"renderPromotionContribution\":\"source-page-space-horizontal-transform-gate\",\"renderPromotionBlockedReason\":\"table-horizontal-page-space-transform-incomplete\""
    ));
    assert!(layer_tree.contains(
        "\"sourcePageYTransformGate\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark y-origin promotion gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":false,\"lineMarkRowsExactAndContiguous\":false,\"pageOriginAuthority\":\"none\",\"lineMarkPageOriginPresent\":false,\"lineMarkPageOriginStridePresent\":false,\"subrecordLineSpanReadinessPresent\":true"
    ));
    assert!(layer_tree.contains(
        "\"selectedPostRowGapSpanOrderedCoverage\":{\"policy\":\"one-tolerance-hit-with-unique-subrecord-candidate-per-line-mark-record\",\"matchedRecordIndexes\":[8,10,12],\"matchedCandidateByteOffsets\":[414,414,414],\"uniqueCandidateByteOffsets\":[414],\"duplicateCandidateByteOffsets\":[414],\"matchedRecordCount\":3,\"uniqueCandidateCount\":1,\"duplicateCandidateReuseCount\":1,\"orderedUniqueCoverageComplete\":false}"
    ));
    assert!(layer_tree.contains(
        "\"subrecordSpanRoleGate\":{\"source\":\"/PageMark raw u16 subrecord line-span role classifier\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"dominantSpanRole\":\"selected-post-row-gap\",\"dominantSpanRoleHitCount\":3,\"selectedPostRowGapSpanHitCount\":3,\"selectedPostRowGapSpanTargetCount\":4,\"selectedPostRowGapSpanComplete\":false,\"rowSpanHitCount\":0,\"rowSpanTargetCount\":4,\"previousRowSpanHitCount\":0,\"compactRowSpanHitCount\":0,\"rowSpanComplete\":false,\"selectedPostRowGapRoleDominant\":true,\"rowSpanRoleDominant\":false"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"subrecord-spans-prefer-post-row-gap-family\",\"selected-post-row-gap-span-incomplete\",\"row-span-family-not-covered-by-subrecords\",\"post-row-gap-match-is-not-visible-row-height\",\"subrecord-span-role-semantics-unproven\",\"page-y-origin-transform-undecoded\"],\"renderPromotionContribution\":\"page-mark-subrecord-span-role-gate\",\"renderPromotionBlockedReason\":\"page-mark-subrecord-spans-match-post-row-gaps-not-row-heights\""
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYOriginHypothesis\":{\"source\":\"sourcePageYTransformGate source-only page-y origin hypothesis\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"candidatePresent\":true,\"candidateKind\":\"cross-table-page-line-domain\",\"yOriginReadinessClass\":\"cross-table-line-domain-only\",\"originDecisionReady\":false"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkPageOriginPresent\":false,\"lineMarkPageOriginStridePresent\":false,\"pageMarkAbsoluteYSlotCandidatePresent\":true,\"pageMarkAbsoluteYSlotY\":1024.000,\"pageMarkAbsoluteYSlotBlockedReason\":\"page-mark-absolute-y-slot-semantics-unproven\",\"pageOriginAuthority\":\"none\""
    ));
    assert!(layer_tree.contains(
        "\"sourceGapToPageLineGapReadinessHints\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapReadinessHints\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"transitionCount\":3,\"samePageMarkEntryTransitionCount\":3,\"allTransitionsSamePageMarkEntry\":true,\"sourceRangeGapToPageLineGapMaxAbsDeltaUnits\":182,\"rowSourceStartGapToPageLineGapMaxAbsDeltaUnits\":295,\"segmentOffsetGapToPageLineGapMaxAbsDeltaUnits\":105,\"bestCandidateTransformKind\":\"segment-offset-gap\",\"bestCandidateMaxAbsDeltaUnits\":105"
    ));
    assert!(layer_tree.contains(
        "\"transformCandidateCount\":4,\"exactTransformCandidateCount\":0,\"bestCandidateTransitionCoverageCount\":3,\"bestCandidateUnitsPerPageLineGapSpread\":29.875,\"lowestSpreadCandidateTransformKind\":\"direct-source-range-gap\",\"lowestSpreadUnitsPerPageLineGapSpread\":12.250"
    ));
    assert!(layer_tree.contains(
        "\"transformCandidateSummaries\":[{\"transformKind\":\"direct-source-range-gap\",\"selected\":false,\"stable\":false,\"transitionCoverageCount\":3,\"maxAbsDeltaUnits\":182,\"unitsPerPageLineGapSpread\":12.250,\"declineReason\":\"higher-max-delta-than-selected-transform\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-transform-not-stable\"}"
    ));
    assert!(layer_tree.contains(
        "{\"transformKind\":\"segment-offset-gap\",\"selected\":true,\"stable\":false,\"transitionCoverageCount\":3,\"maxAbsDeltaUnits\":105,\"unitsPerPageLineGapSpread\":29.875,\"declineReason\":null,\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-transform-not-stable\"}"
    ));
    assert!(layer_tree.contains(
        "\"sourceRangeUnitsPerPageLineGapSpread\":12.250,\"rowSourceStartUnitsPerPageLineGapSpread\":42.125,\"segmentOffsetUnitsPerPageLineGapSpread\":29.875,\"sourceGapToPageLineGapTransformStable\":false,\"tableFamilySourceGapToPageLineGapTransformStable\":false,\"tableFamilyTransformBlockedReason\":\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-transform-not-stable\""
    ));
    assert!(layer_tree.contains(
        "\"crossTableLineDomainEvidence\":{\"present\":true,\"allRecordsWithinSinglePageMarkEntry\":true,\"allOffsetsStable\":true,\"allOffsetsRequireTransform\":true,\"stableRowBoundaryOffsetCandidateUnits\":-82,\"piecewiseAllTablesExact\":false,\"piecewiseMaxAbsResidualRecordIndexes\":0.106,\"combinedLineMarkRecordIndexes\":[7,9,11,13,21,23,25,27,32,34,36]"
    ));
    assert!(layer_tree.contains(
        "\"crossTablePreviousRowSpanSelectorPresent\":true,\"crossTablePreviousRowSpanSupportCount\":5,\"crossTablePreviousRowSpanSelectionReady\":false,\"crossTablePreviousRowSpanReadinessInputs\":{\"previousRowSpanComplete\":false"
    ));
    assert!(layer_tree.contains(
        "\"crossTableOffsetsStable\":true,\"crossTableOffsetsRequireTransform\":true,\"piecewiseAllTablesExact\":false,\"crossTableOrderingConsistent\":false,\"crossTableOrderRegresses\":true,\"decodedPageYOriginPresent\":false"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"source-only-page-y-origin-hypothesis\",\"renderPromotionBlockedReason\":\"source-page-y-origin-inference-pending\""
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYOriginCandidateAgreementGate\":{\"source\":\"sourcePageYTransformGate.sourcePageYOriginHypotheses agreement\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"candidateCount\":9,\"agreementGroupCount\":8,\"bestSupportCount\":2,\"uniqueBestSupported\":true,\"bestSupportedSelectedY\":235.087,\"bestSupportedRowHeight\":23.298,\"bestSupportedOriginBases\":[\"cross-table-combined-previous-row-span-first-record\",\"cross-table-previous-row-span-table-first-row\"],\"bestSupportedTableCandidateIndexes\":[0],\"bestSupportedTableCandidateCount\":1,\"bestSupportedCoversMultipleTableCandidates\":false"
    ));
    assert!(layer_tree.contains(
        "\"crossTablePreviousRowSpanSupportCount\":5,\"crossTablePreviousRowSpanTableCandidateIndexes\":[0,1,2,3],\"crossTablePreviousRowSpanTableCandidateCount\":4,\"crossTablePreviousRowSpanUniqueBestSupported\":true,\"crossTablePreviousRowSpanReady\":false,\"crossTablePreviousRowSpanBestGroupCoversMultipleTables\":false,\"crossTablePreviousRowSpanBestGroupTableCoverageRatio\":0.250,\"crossTablePreviousRowSpanSupportFragmentedByTable\":true,\"crossTablePreviousRowSpanReadinessBlockedReasons\":[\"cross-table-row-boundary-offset-transform-required\",\"page-line-gap-projection-does-not-decode-table-y-origin\"]"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYOriginSelector\":{\"source\":\"sourceOnlyPageYOriginCandidateAgreementGate best-supported group selector\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false,\"tableCandidateIndex\":0,\"selectionBasis\":\"best-supported-source-only-y-origin-agreement-group\",\"singleSupportFallback\":false,\"selectedOriginBasis\":\"cross-table-combined-previous-row-span-first-record\",\"selectedY\":235.087,\"selectedRowHeight\":23.298,\"supportCount\":2,\"supportOriginBases\":[\"cross-table-combined-previous-row-span-first-record\",\"cross-table-previous-row-span-table-first-row\"],\"supportTableCandidateIndexes\":[0],\"supportCoversMultipleTableCandidates\":false,\"supportFragmentedByTable\":true"
    ));
    assert!(layer_tree.contains(
        "\"supportBlockedReasons\":[\"cross-table-row-boundary-offset-transform-required\",\"page-line-gap-projection-does-not-decode-table-y-origin\"],\"renderPromotionContribution\":\"source-only-page-y-origin-selector\",\"renderPromotionBlockedReason\":\"cross-table-previous-row-span-support-fragmented-by-table\""
    ));
    assert!(layer_tree.contains(
        "{\"selectedY\":235.087,\"rowHeight\":23.298,\"supportCount\":2,\"originBases\":[\"cross-table-combined-previous-row-span-first-record\",\"cross-table-previous-row-span-table-first-row\"],\"tableCandidateIndexes\":[0],\"contributions\":[\"cross-table-row-boundary-offset-diagnostic-only\",\"cross-table-row-boundary-offset-diagnostic-only\"]"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"source-page-y-origin-field-semantics-still-unproven\",\"source-page-y-origin-best-support-not-cross-table\",\"cross-table-previous-row-span-support-fragmented-by-table\"],\"renderPromotionContribution\":\"source-page-y-origin-candidate-agreement-gate\",\"renderPromotionBlockedReason\":\"source-page-y-origin-agreement-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYOriginDomainGate\":{\"source\":\"sourcePageYTransformGate.sourceOnlyPageYOriginDomainGate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"directLineMarkPageSpaceOriginPresent\":false,\"crossTableLineDomainPresent\":true,\"crossTableLineDomainRecordCount\":11,\"crossTableLineDomainTableCount\":4,\"combinedLineMarkRecordYPitchPx\":23.298"
    ));
    assert!(layer_tree.contains(
        "\"stableSelectedMinusPreviousRecordIndexGap\":1,\"stableSelectedMinusPreviousRecordYDeltaPx\":23.298,\"selectedSpacingRecordsArePostRowGapFamily\":true,\"piecewiseTransitionCount\":3,\"piecewiseTransitionRecordGaps\":[8,2,5],\"samePageMarkEntryTransitionCount\":3,\"transitionSemanticsReadiness\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"transitionCount\":3,\"samePageMarkEntryTransitionCount\":3,\"allTransitionsSamePageMarkEntry\":true"
    ));
    assert!(layer_tree.contains(
        "\"transitionEvidenceDomain\":\"page-mark-line-index\",\"transitionPairs\":[{\"fromTableCandidateIndex\":0,\"toTableCandidateIndex\":1,\"sourceRangeGapUnits\":190,\"rowSourceStartGapUnits\":303,\"previousFamilyRecordGap\":8,\"selectedFamilyRecordGap\":8,\"selectedMinusPreviousFamilyRecordGapDelta\":0"
    ));
    assert!(layer_tree.contains(
        "\"previousAndSelectedTransitionRecordGapsAgree\":true,\"previousAndSelectedTransitionYGapsAgree\":true"
    ));
    assert!(layer_tree.contains(
        "\"sourceRangeUnitsPerPreviousRecordGap\":[23.750,36.000,27.600],\"rowSourceStartUnitsPerPreviousRecordGap\":[37.875,80.000,46.000],\"previousYGapPxPerRecordGap\":[23.298,23.298,23.298],\"sourceRangeGapRatioStable\":false,\"rowSourceStartGapRatioStable\":false,\"previousYGapRatioStable\":true"
    ));
    assert!(layer_tree.contains(
        "\"sourceGapToPageLineGapDirectMapDiagnostic\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapDirectMapDiagnostic\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false"
    ));
    assert!(layer_tree.contains(
        "\"gapBasis\":\"same-page-mark-entry lineMarkRecordGap as page-mark-line-index gap\",\"sourceRangeGapUnits\":[190,72,138],\"rowSourceStartGapUnits\":[303,160,230],\"pageLineGaps\":[8,2,5],\"sourceRangeGapMinusPageLineGapUnits\":[182,70,133],\"rowSourceStartGapMinusPageLineGapUnits\":[295,158,225]"
    ));
    assert!(layer_tree.contains(
        "\"sourceRangeGapEqualsPageLineGap\":[false,false,false],\"rowSourceStartGapEqualsPageLineGap\":[false,false,false],\"allSourceRangeGapsEqualPageLineGaps\":false,\"allRowSourceStartGapsEqualPageLineGaps\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceRangeUnitsPerPageLineGap\":[23.750,36.000,27.600],\"rowSourceStartUnitsPerPageLineGap\":[37.875,80.000,46.000],\"sourceRangeUnitsPerPageLineGapStable\":false,\"rowSourceStartUnitsPerPageLineGapStable\":false,\"renderPromotionContribution\":\"source-gap-to-page-line-gap-direct-map-diagnostic-only\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-direct-map-not-decoded\"}"
    ));
    assert!(layer_tree.contains(
        "\"sourceGapToPageLineGapSegmentOffsetDiagnostic\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapSegmentOffsetDiagnostic\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false"
    ));
    assert!(layer_tree.contains(
        "\"offsetBasis\":\"rowSourceStartGapUnits minus sourceRangeGapUnits\",\"sourceRangeGapUnits\":[190,72,138],\"rowSourceStartGapUnits\":[303,160,230],\"segmentOffsetGapUnits\":[113,88,92],\"pageLineGaps\":[8,2,5],\"segmentOffsetGapMinusPageLineGapUnits\":[105,86,87]"
    ));
    assert!(layer_tree.contains(
        "\"segmentOffsetGapEqualsPageLineGap\":[false,false,false],\"allSegmentOffsetsEqualPageLineGaps\":false,\"segmentOffsetUnitsPerPageLineGap\":[14.125,44.000,18.400],\"segmentOffsetUnitsPerPageLineGapStable\":false,\"segmentOffsetTransformDecoded\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceGapToPageLineGapTransformReadiness\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.sourceGapToPageLineGapTransformReadiness\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false"
    ));
    assert!(layer_tree.contains(
        "\"transformDomain\":\"source-unit-gap-to-page-mark-line-index-gap\",\"candidateTransformCount\":3,\"acceptedTransformKind\":null,\"directMapDeclined\":true,\"declinedTransformKinds\":[\"direct-source-range-gap\",\"direct-row-source-start-gap\",\"segment-offset-gap\"]"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"source-range-gap-not-equal-page-line-gap\",\"row-source-start-gap-not-equal-page-line-gap\",\"source-range-gap-ratio-not-stable\",\"row-source-start-gap-ratio-not-stable\",\"segment-offset-gap-not-equal-page-line-gap\",\"segment-offset-gap-ratio-not-stable\",\"source-gap-to-page-line-gap-segment-offset-transform-missing\",\"source-gap-to-page-line-gap-transform-undecoded\"],\"nextRequiredEvidence\":\"decode source-gap unit domain or segment transition offset rule before page-space y promotion\",\"renderPromotionContribution\":\"source-gap-to-page-line-gap-transform-readiness\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-transform-not-decoded\"}"
    ));
    assert!(layer_tree.contains(
        "\"sourceRangeGapUnits\":[190,72,138],\"rowSourceStartGapUnits\":[303,160,230],\"previousFamilyRecordGaps\":[8,2,5],\"selectedFamilyRecordGaps\":[8,2,5],\"selectedMinusPreviousFamilyRecordGapDeltas\":[0,0,0]"
    ));
    assert!(layer_tree.contains(
        "\"sourceGapToPageLineGapDecoded\":false,\"pageSpaceTransitionOriginDecoded\":false,\"blockedReasons\":[\"previous-and-selected-family-transitions-share-line-domain-gaps\",\"source-gap-to-page-line-gap-transform-missing\",\"source-range-gap-to-page-line-gap-ratio-not-stable\",\"row-source-start-gap-to-page-line-gap-ratio-not-stable\",\"source-gap-to-page-line-gap-segment-offset-transform-missing\",\"table-family-transition-rule-undecoded\",\"page-space-transition-origin-undecoded\"],\"renderPromotionContribution\":\"table-family-transition-semantics-readiness\",\"renderPromotionBlockedReason\":\"table-family-transition-semantics-undecoded\"}"
    ));
    assert!(layer_tree.contains(
        "\"sourceGapToPageLineGapTransformAdmissionGate\":{\"source\":\"sourceOnlyPageYOriginDomainGate.sourceGapToPageLineGapTransformAdmissionGate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"transformDomain\":\"source-unit-gap-to-page-mark-line-index-gap\",\"canDecodeSourceTransform\":false,\"tableFamilyTransformStable\":false,\"tableFamilyTransformBlockedReason\":\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"transitionCount\":3,\"allTransitionsSamePageMarkEntry\":true,\"bestCandidateTransformKind\":\"segment-offset-gap\",\"bestCandidateMaxAbsDeltaUnits\":105"
    ));
    assert!(layer_tree.contains(
        "\"affineRowSourceStartGapFit\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.affineRowSourceStartGapFit\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"numeratorSlope\":143,\"denominatorSlope\":6,\"numeratorIntercept\":671,\"denominatorIntercept\":6,\"maxAbsResidual\":1.000,\"sampleCount\":3,\"familyScoped\":true,\"fitStable\":true,\"blockedReason\":\"affine-row-source-start-gap-family-transform-authority-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"bestCandidateMaxAbsDeltaUnits\":105,\"transformCandidateCount\":4,\"exactTransformCandidateCount\":0,\"bestCandidateTransitionCoverageCount\":3,\"bestCandidateUnitsPerPageLineGapSpread\":29.875,\"lowestSpreadCandidateTransformKind\":\"direct-source-range-gap\",\"lowestSpreadUnitsPerPageLineGapSpread\":12.250,\"declinedTransformCandidates\":[{\"transformKind\":\"direct-source-range-gap\",\"selected\":false,\"stable\":false,\"transitionCoverageCount\":3,\"maxAbsDeltaUnits\":182"
    ));
    assert!(layer_tree.contains(
        "\"transformKind\":\"affine-row-source-start-gap\",\"selected\":false,\"stable\":true,\"transitionCoverageCount\":3,\"maxAbsDeltaUnits\":1,\"unitsPerPageLineGapSpread\":null,\"affineRowSourceStartGapFit\":{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.affineRowSourceStartGapFit\""
    ));
    assert!(layer_tree.contains(
        "\"declineReason\":\"affine-row-source-start-gap-family-transform-authority-unproven\",\"renderPromotionBlockedReason\":\"affine-row-source-start-gap-family-transform-authority-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"declaredBlockers\":[\"source-gap-to-page-line-gap-transform-not-stable\",\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"source-gap-to-page-line-gap-transform-undecoded\"],\"renderPromotionContribution\":\"source-gap-to-page-line-gap-transform-admission-gate\",\"renderPromotionBlockedReason\":\"source-gap-to-page-line-gap-transform-not-stable\"},\"lineDomainRequiresOffsetTransform\":true,\"pageSpaceOriginDecoded\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageMarkAbsoluteYSlotGate\":{\"source\":\"/PageMark raw u16 subrecord scan+/LineMark source page-line projection\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"projectionKind\":\"line-domain-y-plus-post-row-gap-vs-page-mark-absolute-y-slot\""
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"direct-line-mark-page-space-origin-absent\",\"cross-table-evidence-is-page-mark-line-domain\",\"line-domain-to-page-space-origin-transform-required\",\"table-family-transition-semantics-undecoded\",\"selected-spacing-records-are-post-row-gap-family\",\"page-space-table-origin-undecoded\"],\"renderPromotionContribution\":\"source-page-y-origin-domain-gate\",\"renderPromotionBlockedReason\":\"source-page-y-line-domain-not-page-space-origin\""
    ));
    assert!(layer_tree.contains(
        "\"lineDomainPostRowGapProjectionProbe\":{\"source\":\"sourcePageYTransformGate line-domain + post-row-gap span projection\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"projectionKind\":\"line-domain-y-plus-post-row-gap-unit-as-px\",\"selectionReady\":false,\"promotionReady\":false,\"lineDomainY\":235.087,\"selectedPostRowGapSpanFirstUnits\":65,\"selectedPostRowGapSpanComplete\":false,\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":false,\"projectedY\":300.087,\"referenceTableTopY\":301.005,\"residualPx\":-0.919,\"absResidualPx\":0.919,\"withinTwoPx\":true"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyProjectionDomainGate\":{\"source\":\"sourcePageYTransformGate source-only line-domain/post-row-gap projection domain gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"sourceProjectionPresent\":true,\"lineDomainPresent\":true,\"selectedPostRowGapSpanPresent\":true,\"selectedPostRowGapSpanComplete\":false,\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":false,\"sourceUnitDomain\":\"line-mark-record-y-plus-page-mark-subrecord-gap-units\",\"lineDomainY\":235.087,\"selectedPostRowGapSpanFirstUnits\":65,\"projectedY\":300.087,\"blockedReasons\":[\"cross-domain-source-units-treated-as-px\",\"selected-spacing-records-are-post-row-gap-family\",\"selected-post-row-gap-span-incomplete\",\"selected-post-row-gap-span-not-ordered-unique\",\"page-y-origin-transform-undecoded\"],\"renderPromotionContribution\":\"source-only-line-domain-post-row-gap-projection-domain-gate\",\"renderPromotionBlockedReason\":\"line-domain-post-row-gap-projection-crosses-source-unit-domain\"}"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"cross-domain-source-units-treated-as-px\",\"selected-spacing-records-are-post-row-gap-family\",\"selected-post-row-gap-span-incomplete\",\"selected-post-row-gap-span-not-ordered-unique\",\"reference-only-validation\",\"page-y-origin-transform-undecoded\"],\"renderPromotionContribution\":\"line-domain-post-row-gap-projection-probe\",\"renderPromotionBlockedReason\":\"line-domain-post-row-gap-projection-crosses-source-unit-domain\""
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYRenderAdmissionGate\":{\"source\":\"sourcePageYTransformGate source-only page-y render admission gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"admissionReady\":false,\"directLineMarkOriginAdmissible\":false,\"sourceLayoutCandidatePresent\":true,\"pageOriginAuthority\":\"none\",\"lineMarkRowsExactAndContiguous\":false,\"lineMarkPageOriginPresent\":false,\"lineMarkPageOriginStridePresent\":false,\"crossTableLineDomainPresent\":true"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlySelectorBlockedReason\":\"source-y-origin-selector-fragmented-by-table-not-render-admissible\",\"sourceOnlySelectorSupportBlockedReasons\":[\"cross-table-row-boundary-offset-transform-required\",\"page-line-gap-projection-does-not-decode-table-y-origin\"]"
    ));
    assert!(layer_tree.contains(
        "\"sourceGapToPageLineGapTransformAdmissionGate\":{\"source\":\"sourceOnlyPageYRenderAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"transformDomain\":\"source-unit-gap-to-page-mark-line-index-gap\",\"canDecodeSourceTransform\":false,\"tableFamilyTransformStable\":false,\"tableFamilyTransformBlockedReason\":\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"transitionCount\":3,\"allTransitionsSamePageMarkEntry\":true,\"bestCandidateTransformKind\":\"segment-offset-gap\",\"bestCandidateMaxAbsDeltaUnits\":105"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkAbsoluteYSlotSemanticsReady\":false,\"pageMarkAbsoluteYSlotBlockedReason\":\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"pageMarkAbsoluteYSlotResidualPx\":-723.913"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"direct-line-mark-page-origin-absent\",\"page-origin-authority-not-renderable-line-mark-page-grid\",\"line-mark-rows-not-exact-source-boundaries\",\"cross-table-line-domain-not-page-space-origin\",\"source-order-vs-subrecord-order-contradiction\",\"cross-table-row-boundary-offset-transform-required\",\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"source-y-origin-selector-fragmented-by-table-not-render-admissible\",\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"decoded-line-mark-page-y-transform-missing\"],\"renderPromotionContribution\":\"source-only-page-y-render-admission-gate\",\"renderPromotionBlockedReason\":\"source-page-y-render-admission-not-ready\""
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"line-mark-page-origin-candidate-absent\",\"page-origin-authority-not-renderable-line-mark-page-grid\",\"line-mark-rows-not-exact-source-boundaries\",\"page-mark-subrecord-spans-do-not-decode-page-y-origin\",\"page-mark-cross-table-raw-record-order-regression\",\"page-mark-cross-table-subrecord-ordering-unproven\",\"cross-table-row-boundary-offset-transform-required\",\"decoded-line-mark-page-y-transform-missing\"],\"renderPromotionContribution\":\"source-page-y-transform-gate\",\"renderPromotionBlockedReason\":\"source-page-y-transform-not-decoded\""
    ));
    assert!(layer_tree.contains(
        "\"crossTableRowBoundaryOffsetConsistency\":{\"source\":\"/LineMark previous row-span boundaries+cross-table sparse sibling order\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"currentTableCandidateIndex\":0,\"sparseTableCandidateIndex\":4,\"relatedTableCandidateIndexes\":[0,1,2,3],\"relatedTableCount\":4,\"tableCountWithPreviousRowSpanAlignment\":4,\"rowBoundaryOffsetCandidateUnits\":[-82,-82,-82,-82],\"stableRowBoundaryOffsetCandidateUnits\":-82,\"allRelatedTablesHaveOffsetCandidate\":true,\"allOffsetsStable\":true,\"allOffsetsRequireTransform\":true"
    ));
    assert!(layer_tree.contains(
        "\"offsetNormalizationPolicy\":\"row-source-boundaries-plus-stable-offset-must-equal-previous-line-mark-boundaries\",\"allOffsetNormalizedBoundariesExact\":true"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkLineDomainPolicy\":\"previous-row-span-records-must-share-one-page-mark-entry-and-monotonic-line-offsets\",\"combinedLineMarkRecordIndexes\":[7,9,11,13,21,23,25,27,32,34,36],\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":42,\"pageMarkU16FieldCount\":137"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkU16FieldPreview\":[0,0,1,0,0,0,0,42,0,0,564,0,0,564,194,423,223,564,564,370,255,564,0,0]"
    ));
    assert!(layer_tree.contains(
        "\"combinedLineOffsetsFromPageStart\":[7,9,11,13,21,23,25,27,32,34,36],\"combinedLineOffsetsMonotonic\":true"
    ));
    assert!(layer_tree.contains(
        "\"combinedLineMarkRecordYProjection\":{\"source\":\"/PageMark line range+page layout body line gap\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"combinedLineMarkRecordYPitchPx\":23.298,\"combinedLineMarkRecordYPitchBasis\":\"pageMarkBodyLineGap\""
    ));
    assert!(layer_tree.contains(
        "\"combinedLineMarkRecordYTopPx\":[235.087,281.683,328.279,374.875,561.260,607.856,654.452,701.048,817.539,864.135,910.731],\"combinedLineMarkRecordYSpanPx\":675.645"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"page-line-gap-projection-does-not-decode-table-y-origin\"},\"sourceUnitToPageLineIndexFit\":{\"source\":\"/DocumentText row source units+/LineMark previous-row-span records\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"fitBasis\":\"rowSourceStartUnits-to-lineMarkRecordIndexes\""
    ));
    assert!(layer_tree.contains(
        "\"rowSourceStartUnits\":[304,476,654,832,1135,1307,1467,1627,1857,2026,2170],\"lineMarkRecordIndexes\":[7,9,11,13,21,23,25,27,32,34,36],\"slopeRecordIndexesPerSourceUnit\":0.016,\"interceptRecordIndex\":1.220"
    ));
    assert!(layer_tree.contains(
        "\"fittedRecordIndexes\":[6.147,8.935,11.821,14.706,19.618,22.406,24.999,27.593,31.321,34.060,36.394],\"residualRecordIndexes\":[0.853,0.065,-0.821,-1.706,1.382,0.594,0.001,-0.593,0.679,-0.060,-0.394],\"maxAbsResidualRecordIndexes\":1.706,\"exactFit\":false,\"rows\":[{\"tableCandidateIndex\":0,\"rowIndex\":0,\"rowSourceStartUnits\":304,\"lineMarkRecordIndex\":7,\"fittedRecordIndex\":6.147,\"residualRecordIndex\":0.853}"
    ));
    assert!(layer_tree.contains(
        "{\"tableCandidateIndex\":0,\"rowIndex\":3,\"rowSourceStartUnits\":832,\"lineMarkRecordIndex\":13,\"fittedRecordIndex\":14.706,\"residualRecordIndex\":-1.706},{\"tableCandidateIndex\":1,\"rowIndex\":0,\"rowSourceStartUnits\":1135,\"lineMarkRecordIndex\":21,\"fittedRecordIndex\":19.618,\"residualRecordIndex\":1.382}"
    ));
    assert!(layer_tree.contains(
        "{\"tableCandidateIndex\":3,\"rowIndex\":2,\"rowSourceStartUnits\":2170,\"lineMarkRecordIndex\":36,\"fittedRecordIndex\":36.394,\"residualRecordIndex\":-0.394}],\"renderPromotionBlockedReason\":\"source-unit-to-page-line-affine-fit-not-exact\"},\"sourceUnitToPageLineIndexPiecewiseFit\":{\"source\":\"/DocumentText row source units+/LineMark previous-row-span records table-piecewise\""
    ));
    assert!(layer_tree.contains(
        "\"fitBasis\":\"per-related-table-rowSourceStartUnits-to-lineMarkRecordIndexes\",\"groupingBasis\":\"crossTableRowBoundaryOffsetConsistency.tables\",\"globalFitExact\":false,\"allTableFitsExact\":false,\"maxTableFitResidualRecordIndexes\":0.106,\"samePageMarkEntryContinuity\":true,\"pieceCount\":4,\"transitionCount\":3"
    ));
    assert!(layer_tree.contains(
        "\"pieces\":[{\"tableCandidateIndex\":0,\"sourceRange\":{\"start\":304,\"end\":945},\"rowCount\":4,\"rowSourceStartUnits\":[304,476,654,832],\"lineMarkRecordIndexes\":[7,9,11,13],\"slopeRecordIndexesPerSourceUnit\":0.011,\"interceptRecordIndex\":3.570,\"fittedRecordIndexes\":[7.021,8.973,10.993,13.013],\"residualRecordIndexes\":[-0.021,0.027,0.007,-0.013],\"maxAbsResidualRecordIndexes\":0.027,\"exactFit\":false,\"pageMarkRecordsWithinSingleEntry\":true}"
    ));
    assert!(layer_tree.contains(
        "{\"tableCandidateIndex\":3,\"sourceRange\":{\"start\":1857,\"end\":2261},\"rowCount\":3,\"rowSourceStartUnits\":[1857,2026,2170],\"lineMarkRecordIndexes\":[32,34,36],\"slopeRecordIndexesPerSourceUnit\":0.013,\"interceptRecordIndex\":8.270,\"fittedRecordIndexes\":[31.951,34.106,35.943],\"residualRecordIndexes\":[0.049,-0.106,0.057],\"maxAbsResidualRecordIndexes\":0.106,\"exactFit\":false,\"pageMarkRecordsWithinSingleEntry\":true}"
    ));
    assert!(layer_tree.contains(
        "\"transitions\":[{\"fromTableCandidateIndex\":0,\"toTableCandidateIndex\":1,\"previousLastSourceUnit\":832,\"nextFirstSourceUnit\":1135,\"sourceRangeGapUnits\":190,\"rowSourceStartGapUnits\":303,\"previousLastRecordIndex\":13,\"nextFirstRecordIndex\":21,\"lineMarkRecordGap\":8,\"samePageMarkEntry\":true}"
    ));
    assert!(layer_tree.contains(
        "{\"fromTableCandidateIndex\":2,\"toTableCandidateIndex\":3,\"previousLastSourceUnit\":1627,\"nextFirstSourceUnit\":1857,\"sourceRangeGapUnits\":138,\"rowSourceStartGapUnits\":230,\"previousLastRecordIndex\":27,\"nextFirstRecordIndex\":32,\"lineMarkRecordGap\":5,\"samePageMarkEntry\":true}],\"renderPromotionContribution\":\"source-unit-to-page-line-piecewise-fit-diagnostic-only\",\"renderPromotionBlockedReason\":\"piecewise-fit-does-not-decode-page-y-origin\"},\"piecewiseRecordFamilyGapYDiagnostic\":{\"source\":\"/DocumentText row source units+/LineMark families (selected-spacing vs previous-row-span)+piecewise transitions\""
    ));
    assert!(layer_tree.contains(
        "\"recordFamilyInterpretation\":\"selected-records-match-post-row-gaps-previous-records-match-row-spans\",\"stableSelectedMinusPreviousRecordIndexGap\":1,\"allSelectedRecordsOneAfterPrevious\":true,\"stableSelectedMinusPreviousRecordYDeltaPx\":23.298,\"allRecordFamiliesWithinSinglePageMarkEntry\":true"
    ));
    assert!(layer_tree.contains(
        "\"tables\":[{\"tableCandidateIndex\":0,\"sourceRange\":{\"start\":304,\"end\":945},\"rowCount\":4,\"previousRecordIndexes\":[7,9,11,13],\"selectedRecordIndexes\":[8,10,12,14],\"previousPageMarkLineOffsetsFromEntryStart\":[7,9,11,13],\"selectedPageMarkLineOffsetsFromEntryStart\":[8,10,12,14]"
    ));
    assert!(layer_tree.contains(
        "\"selectedMinusPreviousRecordIndexGaps\":[1,1,1,1],\"selectedMinusPreviousRecordYDeltaPx\":[23.298,23.298,23.298,23.298],\"previousStartResidualUnits\":[-82,-82,-82,-82],\"previousEndResidualUnits\":[-82,-82,-82,-82],\"previousSpanResidualUnits\":[0,0,0,0],\"selectedStartResidualUnits\":[25,31,31,31],\"selectedEndResidualUnits\":[-17,-17,-17,-15],\"selectedSpanResidualUnits\":[-42,-48,-48,-46]"
    ));
    assert!(layer_tree.contains(
        "{\"tableCandidateIndex\":3,\"sourceRange\":{\"start\":1857,\"end\":2261},\"rowCount\":3,\"previousRecordIndexes\":[32,34,36],\"selectedRecordIndexes\":[33,35,37],\"previousPageMarkLineOffsetsFromEntryStart\":[32,34,36],\"selectedPageMarkLineOffsetsFromEntryStart\":[33,35,37]"
    ));
    assert!(layer_tree.contains(
        "\"transitions\":[{\"fromTableCandidateIndex\":0,\"toTableCandidateIndex\":1,\"sourceRangeGapUnits\":190,\"rowSourceStartGapUnits\":303,\"previousFamilyRecordGap\":8,\"selectedFamilyRecordGap\":8,\"selectedMinusPreviousFamilyRecordGapDelta\":0"
    ));
    assert!(layer_tree.contains(
        "{\"fromTableCandidateIndex\":2,\"toTableCandidateIndex\":3,\"sourceRangeGapUnits\":138,\"rowSourceStartGapUnits\":230,\"previousFamilyRecordGap\":5,\"selectedFamilyRecordGap\":5,\"selectedMinusPreviousFamilyRecordGapDelta\":0"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"source-unit-to-page-line-family-gap-piecewise-diagnostic-only\",\"renderPromotionBlockedReason\":\"piecewise-family-gap-y-comparison-blocks-page-y-origin\"},\"sourceOnlyPageMarkSlotScopedSubrecordYSequenceProbe\":{\"source\":\"/PageMark raw u16 subrecord scan+/LineMark source page-line projection\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false"
    ));
    assert!(layer_tree.contains(
        "\"referenceBBoxUsed\":false,\"selectionReady\":false,\"grouping\":\"fieldIndex+tailBlock16WordIndex\",\"sourceYTargetBasis\":\"page-mark-line-range-plus-page-layout-body-line-gap\",\"tolerancePx\":2.000,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":42"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[7,9,11,13,21,23,25,27,32,34,36],\"sourceLineMarkRecordYTopPx\":["
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"source-only-page-mark-slot-sequence-diagnostic-only\",\"renderPromotionBlockedReason\":\"page-mark-source-y-slot-candidates-do-not-decode-page-y-origin\"},\"allRecordsWithinSinglePageMarkEntry\":true"
    ));
    assert!(layer_tree.contains(
        "\"tableCandidateIndex\":3,\"sourceRange\":{\"start\":1857,\"end\":2261},\"rowCount\":3,\"lineMarkRecordIndexes\":[32,34,36],\"pageMarkLineOffsetsFromEntryStart\":[32,34,36],\"pageMarkRecordsWithinSingleEntry\":true,\"lineMarkRecordYTopPx\":[817.539,864.135,910.731],\"selectedSpacingRecordIndexes\":[33,35,37],\"selectedSpacingPageMarkLineOffsetsFromEntryStart\":[33,35,37],\"selectedSpacingRecordsWithinSingleEntry\":true"
    ));
    assert!(layer_tree.contains(
        "\"selectedSpacingLineMarkStartUnits\":[1886,2036,2179],\"selectedSpacingLineMarkEndUnits\":[1944,2088,2233],\"selectedSpacingStartResidualUnits\":[29,10,9],\"selectedSpacingEndResidualUnits\":[-24,-30,-28],\"selectedSpacingSpanResidualUnits\":[-53,-40,-37],\"selectedMinusPreviousRecordIndexGaps\":[1,1,1],\"selectedMinusPreviousRecordYDeltaPx\":[23.298,23.298,23.298],\"rowSourceStartUnits\":[1857,2026,2170],\"rowSourceEndUnits\":[1968,2118,2261],\"lineMarkStartUnits\":[1775,1944,2088],\"lineMarkEndUnits\":[1886,2036,2179],\"startResidualUnits\":[-82,-82,-82],\"endResidualUnits\":[-82,-82,-82],\"spanResidualUnits\":[0,0,0],\"rowBoundaryOffsetCandidateUnits\":-82,\"offsetNormalizedStartResidualUnits\":[0,0,0],\"offsetNormalizedEndResidualUnits\":[0,0,0],\"offsetNormalizedExactBoundaryAligned\":true,\"exactBoundaryAligned\":false,\"spanOnlyMatch\":true"
    ));
    assert!(layer_tree.contains(
        "\"renderPromoted\":false,\"renderPromotionAuthority\":null,\"renderPromotionBlockedReason\":\"sparse-sibling-derived-candidate-render-ineligible\""
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[8,10,12,14],\"uniformLineMarkRecordStride\":true,\"lineMarkRecordStride\":2,\"interleavedLineMarkRecordCountBetweenRows\":1"
    ));
    assert!(layer_tree.contains(
        "\"sparseTableSiblingEvidence\":{\"source\":\"sparseDocumentTextControlRunTableCandidate\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"sparseTableCandidateIndex\":4"
    ));
    assert!(layer_tree.contains(
        "\"candidateSourceRange\":{\"start\":304,\"end\":945},\"sparseSourceRange\":{\"start\":304,\"end\":2315},\"sourceRangeContainsCandidate\":true"
    ));
    assert!(layer_tree.contains(
        "\"candidateRowCount\":4,\"matchedRowCount\":4,\"allCandidateRowsMatched\":true,\"candidateSegmentCount\":12,\"matchedSegmentCount\":12,\"allCandidateSegmentsMatched\":true,\"sharedSourceIntervalIndexes\":[1,3,5,7],\"compactToSparseColumnOffsetCandidate\":3,\"matchedSparseColumnIndexes\":[3,4,5]"
    ));
    assert!(layer_tree.contains(
        "\"sparseSiblingColumnPromotionReadiness\":{\"source\":\"sparseTableSiblingEvidence+documentTextLineHeaders column promotion readiness\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"columnSplitReady\":false,\"requestedColumnCount\":3,\"candidateRowCount\":4,\"matchedRowCount\":4,\"candidateSegmentCount\":12,\"matchedSegmentCount\":12,\"sparseTopologyComplete\":true,\"compactToSparseColumnOffsetCandidate\":3,\"matchedSparseColumnIndexes\":[3,4,5],\"compactLineHeaderCellCoverageComplete\":false,\"decodedSourcePlacementMatchCount\":0,\"decodedSourcePlacementRequiredCellCount\":12,\"sourceLineHeaderColumnWidthsPresent\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceColumnWidthBasis\":null,\"sourceColumnWidthFractions\":[],\"blockedReasons\":[\"compact-line-header-cell-geometry-incomplete\",\"source-line-header-column-widths-missing\"],\"renderPromotionContribution\":\"sparse-sibling-column-readiness-gate\",\"renderPromotionBlockedReason\":\"source-column-split-not-ready\""
    ));
    assert!(layer_tree.contains(
        "\"sparseSiblingDerivedCompactCellGeometry\":{\"source\":\"sparseTableSiblingEvidence compact cell geometry prerequisite\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"candidateRowCount\":4,\"matchedRowCount\":4,\"candidateSegmentCount\":12,\"matchedSegmentCount\":12,\"compactToSparseColumnOffsetCandidate\":3,\"matchedSparseColumnIndexes\":[3,4,5],\"derivedMatchedCellCount\":12,\"requiredCellCount\":12,\"derivedCellGeometryCoverageComplete\":true,\"sourcePlacementPrerequisiteReady\":true,\"renderPromotionContribution\":\"sparse-sibling-derived-geometry-prerequisite\",\"renderPromotionBlockedReason\":\"sparse-sibling-derived-geometry-diagnostic-only\""
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidate\":{\"source\":\"sparseTableSiblingEvidence compact cell geometry candidate\",\"sourceBacked\":true,\"referenceBacked\":false"
    ));
    assert!(layer_tree.contains("\"provenance\":\"sparseSiblingDerived\""));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"sparse-sibling-derived-candidate-render-ineligible\""
    ));
    assert!(layer_tree.contains(
        "\"decodedSourcePlacementEvidence\":false,\"decodedSourcePlacementMatchCount\":0,\"decodedSourcePlacementRequiredCellCount\":12"
    ));
    assert!(layer_tree.contains(
        "\"compactRow\":0,\"sparseRow\":0,\"sourceIntervalIndex\":1,\"sourceRange\":{\"start\":304,\"end\":411},\"compactCellCount\":3,\"sparseCellCount\":7,\"sparseEmptyCellCount\":4,\"sparseNonEmptyCellCount\":3,\"firstNonEmptySparseColumnIndex\":3,\"lastNonEmptySparseColumnIndex\":5,\"compactToSparseColumnOffset\":3"
    ));
    assert!(layer_tree.contains(
        "\"sparseSiblingLineMarkYComparison\":{\"source\":\"sparseTableSiblingEvidence+/LineMark+/PageMark+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"sparseTableCandidateIndex\":4,\"sharedSourceIntervalIndexes\":[1,3,5,7]"
    ));
    assert!(layer_tree.contains(
        "\"sourceRowHeightBasis\":\"partialDocumentTextLineHeaderFontSizeUnits\",\"homogeneousFontSizeUnits\":12,\"lineHeaderRowCount\":4,\"lineHeaderRowsWithHeaders\":2,\"rawHeaderCount\":2,\"sourceRowHeightPx\":21.000"
    ));
    assert!(layer_tree.contains(
        "\"postRowGapLineMarkCorrelation\":{\"source\":\"sparseTableSiblingEvidence+/LineMark\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"rowCount\":4,\"matchedGapCount\":4,\"exactSpanMatchCount\":4,\"allRowsExactSpanMatched\":true"
    ));
    assert!(layer_tree.contains(
        "\"compactRow\":0,\"sparseRow\":0,\"sourceIntervalIndex\":1,\"lineMarkRecordIndex\":8,\"lineMarkUnitRange\":{\"start\":329,\"end\":394},\"lineMarkSpanUnits\":65,\"postRowGapSourceRange\":{\"start\":411,\"end\":476},\"postRowGapUnits\":65,\"postRowGapKind\":\"between-matched-sparse-rows\",\"gapSparseRowIndexes\":[1],\"gapSparseSourceIntervalIndexes\":[2],\"lineMarkSpanMinusGapUnits\":0,\"exactSpanMatch\":true"
    ));
    assert!(layer_tree.contains(
        "\"compactRow\":3,\"sparseRow\":6,\"sourceIntervalIndex\":7,\"lineMarkRecordIndex\":14,\"lineMarkUnitRange\":{\"start\":863,\"end\":930},\"lineMarkSpanUnits\":67,\"postRowGapSourceRange\":{\"start\":945,\"end\":1012},\"postRowGapUnits\":67,\"postRowGapKind\":\"trailing-empty-sparse-rows\",\"gapSparseRowIndexes\":[7],\"gapSparseSourceIntervalIndexes\":[8],\"lineMarkSpanMinusGapUnits\":0,\"exactSpanMatch\":true"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRowGapSequenceEvidence\":{\"source\":\"/LineMark+sparseTableSiblingEvidence\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"lineMarkProfile\":\"be16-delta-v1\",\"lineMarkStreamByteLength\":2846,\"lineMarkWordCount\":1423,\"lineMarkTagCount\":0,\"lineMarkTagFamilyCounts\":{\"0x1000\":0,\"0x1001\":0,\"0x1002\":0},\"tagPayloadCorrelation\":\"not-applicable-no-line-mark-tags\""
    ));
    assert!(layer_tree.contains(
        "\"selectedRecordPostRowGapSpanMatchCount\":4,\"allSelectedRecordsMatchPostRowGapSpan\":true,\"previousRecordRowSpanMatchCount\":4,\"allPreviousRecordsMatchRowSpan\":true,\"nextRecordNextRowSpanMatchCount\":3,\"rowsWithNextRow\":3,\"sequenceInterpretationCandidate\":\"alternating-row-span-record-then-post-row-gap-record\""
    ));
    assert!(layer_tree.contains(
        "\"rowSourceUnitRange\":{\"start\":304,\"end\":411},\"rowSpanUnits\":107,\"selectedLineMarkRecord\":{\"recordIndex\":8,\"byteOffset\":50,\"wordIndex\":25,\"delta\":65,\"unitRange\":{\"start\":329,\"end\":394},\"flagWord\":2,\"flagWordHex\":\"0x0002\""
    ));
    assert!(layer_tree.contains(
        "\"selectedRecordMatchesPostRowGapSpan\":true,\"previousLineMarkRecord\":{\"recordIndex\":7,\"byteOffset\":46,\"wordIndex\":23,\"delta\":107,\"unitRange\":{\"start\":222,\"end\":329},\"flagWord\":2,\"flagWordHex\":\"0x0002\""
    ));
    assert!(layer_tree.contains(
        "\"previousRecordMatchesRowSpan\":true,\"nextLineMarkRecord\":{\"recordIndex\":9,\"byteOffset\":54,\"wordIndex\":27,\"delta\":113,\"unitRange\":{\"start\":394,\"end\":507},\"flagWord\":2,\"flagWordHex\":\"0x0002\""
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRowGapSequenceYComparison\":{\"source\":\"/LineMark row/gap sequence+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"rowCountCompared\":4,\"referenceRowTops\":[301.005,333.206,365.406,397.607],\"selectedSpacingRecordIndexes\":[8,10,12,14],\"previousRowSpanRecordIndexes\":[7,9,11,13]"
    ));
    assert!(layer_tree.contains(
        "\"selectedSpacingRecordCandidate\":{\"family\":\"selected-spacing-records\",\"spanInterpretation\":\"post-row-gap-span\",\"recordIndexes\":[8,10,12,14],\"uniformRecordStride\":true,\"recordStride\":2,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":42"
    ));
    assert!(layer_tree.contains(
        "\"rowHeightPitchResidualsPx\":[-61.005,-51.206,-41.406,-31.607],\"rowHeightPitchMeanAbsResidualPx\":46.306,\"rowHeightPitchMaxAbsResidualPx\":61.005,\"pageLinePitchPx\":23.298,\"pageLinePitchRowTops\":[258.385,304.981,351.577,398.173],\"pageLinePitchResidualsPx\":[-42.621,-28.225,-13.829,0.566]"
    ));
    assert!(layer_tree.contains(
        "\"previousRowSpanRecordCandidate\":{\"family\":\"previous-row-span-records\",\"spanInterpretation\":\"compact-row-span\",\"recordIndexes\":[7,9,11,13],\"uniformRecordStride\":true,\"recordStride\":2,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":42"
    ));
    assert!(layer_tree.contains(
        "\"rowHeightPitchResidualsPx\":[-82.005,-72.206,-62.406,-52.607],\"rowHeightPitchMeanAbsResidualPx\":67.306,\"rowHeightPitchMaxAbsResidualPx\":82.005,\"pageLinePitchPx\":23.298,\"pageLinePitchRowTops\":[235.087,281.683,328.279,374.875],\"pageLinePitchResidualsPx\":[-65.919,-51.523,-37.127,-22.732]"
    ));
    assert!(layer_tree.contains(
        "\"bestCandidate\":\"selected-spacing-records-page-line-pitch\",\"bestCandidateMaxAbsResidualPx\":42.621,\"renderPromotionContribution\":\"row-gap-record-family-y-diagnostic-only\""
    ));
    assert!(layer_tree.contains(
        "\"comparison\":{\"source\":\"sparseSiblingLineMarkPageOriginStrideCandidate+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"lineMarkRecordIndexes\":[7,9,11,13],\"recordStride\":2"
    ));
    assert!(layer_tree.contains(
        "\"rawRecordIndexResidualsPx\":[-82.005,-72.206,-62.406,-52.607],\"rawRecordIndexMeanAbsResidualPx\":67.306,\"rawRecordIndexMaxAbsResidualPx\":82.005"
    ));
    assert!(layer_tree.contains(
        "\"recordIndexAffineFit\":{\"source\":\"/LineMark record indexes+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"lineMarkRecordIndexes\":[7,9,11,13],\"recordStride\":2,\"referenceSlopePxPerRecord\":16.100,\"referenceInterceptPx\":188.303"
    ));
    assert!(layer_tree.contains(
        "\"sourceRawSlopePxPerRecord\":21.000,\"sourceRawSlopeResidualPxPerRecord\":4.900,\"sourceStrideCollapsedSlopePxPerRecord\":10.500,\"sourceStrideCollapsedSlopeResidualPxPerRecord\":-5.600"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkStridePromotionReadiness\":{\"source\":\"/LineMark+/PageMark+sparseSiblingEvidence\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":false,\"candidateRowCount\":4,\"candidateSegmentCount\":12,\"allRowsHaveLineMark\":true,\"lineMarkRecordIndexes\":[7,9,11,13],\"uniformRecordStride\":true,\"recordStride\":2"
    ));
    assert!(layer_tree.contains(
        "\"matchedCellHeaderCount\":0,\"postRowGapCorrelationComplete\":true,\"postRowGapMatchCount\":4,\"postRowGapExactSpanMatchCount\":4,\"rawPageMarkScanHeaderCount\":15,\"rawPageMarkSingleHeaderMatched\":true"
    ));
    assert!(layer_tree.contains(
        "\"subrecordLineSpanReadiness\":{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"spanToleranceUnits\":3,\"selectedSpacingRecordIndexes\":[8,10,12,14],\"previousRowSpanRecordIndexes\":[7,9,11,13],\"selectedPostRowGapSpanTargets\":[65,65,65,67],\"postRowGapSpanTargets\":[65,65,65,67],\"previousRowSpanTargets\":[107,113,113,113],\"compactRowSpanTargets\":[107,113,113,113],\"candidateCount\":7,\"selectedPostRowGapSpanHitCount\":3,\"previousRowSpanHitCount\":0,\"compactRowSpanHitCount\":0,\"selectedPostRowGapSpanComplete\":false,\"previousRowSpanComplete\":false,\"compactRowSpanComplete\":false,\"selectedPostRowGapSpanMaxAbsResidualUnits\":5,\"previousRowSpanMaxAbsResidualUnits\":51,\"compactRowSpanMaxAbsResidualUnits\":51,\"subrecordSpanRoleGate\":"
    ));
    assert!(layer_tree.contains(
        "\"referenceValidationThresholdPx\":8.000,\"rawRecordIndexReferenceFit\":false,\"rawRecordIndexMaxAbsResidualPx\":82.005"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"partial-line-header-font-evidence\",\"line-header-cell-geometry-incomplete\",\"line-mark-spans-post-row-gaps-not-visible-row-heights\",\"raw-record-index-y-fails-current-reference-table\",\"decoded-line-mark-stride-to-page-y-transform-missing\"]"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkRawRecordScanEvidence\":{\"source\":\"/PageMark raw record scan+/LineMark\",\"present\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"diagnosticOnly\":true,\"streamByteLength\":1108,\"parsedPageMarkFamily\":\"count-plus-one-variable\",\"parsedPageMarkEntryCount\":4,\"scannedRecordHeaderCount\":15"
    ));
    assert!(layer_tree.contains(
        "\"candidateRowCount\":4,\"rowLineMarkMatchCount\":4,\"rowScannedRecordHeaderMatchCount\":4,\"allRowsHaveLineMark\":true,\"allRowsHaveScannedRecordHeader\":true,\"singleScannedRecordHeaderMatched\":true,\"matchedScannedRecordHeaderIndex\":0,\"lineMarkRecordIndexes\":[8,10,12,14]"
    ));
    assert!(layer_tree.contains(
        "\"recordHeaders\":[{\"scanIndex\":0,\"byteOffset\":12,\"nextByteOffset\":92,\"recordPayloadByteLength\":80,\"index\":0,\"flags\":65536,\"flagsHex\":\"0x00010000\",\"lineStart\":0,\"lineEnd\":42,\"lineCount\":43}"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkRawRecordSourceRangeEvidence\":{\"source\":\"/PageMark raw record headers+table source unit ranges\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"recordHeaderCount\":15,\"candidateRowCount\":4,\"rowSourceCoverageCount\":3,\"allRowsHaveHeaderCoverage\":false,\"totalOverlappingHeaderCount\":8,\"matchedScanIndexes\":[4,5,6,7,8,9,11,14],\"matchedScanIndexesMonotonic\":true"
    ));
    assert!(layer_tree.contains(
        "\"row\":0,\"sourceUnitRange\":{\"start\":304,\"end\":411},\"overlappingHeaderCount\":3,\"overlappingHeaders\":[{\"scanIndex\":4,\"recordIndex\":6,\"recordLineStart\":305,\"recordLineEnd\":347,\"overlapUnitRange\":{\"start\":305,\"end\":347},\"overlapUnits\":43}"
    ));
    assert!(layer_tree.contains(
        "\"row\":3,\"sourceUnitRange\":{\"start\":832,\"end\":945},\"overlappingHeaderCount\":0,\"overlappingHeaders\":[]"
    ));
    assert!(layer_tree.contains(
        "\"crossTableSubrecordOrderingProbe\":{\"source\":\"/PageMark raw u16 subrecord line ranges+cross-table sparse sibling order\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"currentTableCandidateIndex\":0,\"relatedTableCandidateIndexes\":[0,1,2,3],\"relatedTableCount\":4,\"sourceOrderingBasis\":\"tableCandidate.source_start\",\"relatedTableSourceRanges\":[{\"start\":304,\"end\":945},{\"start\":1135,\"end\":1395},{\"start\":1467,\"end\":1719},{\"start\":1857,\"end\":2261}],\"sourceOrderMatchesProbeOrder\":true,\"combinedMatchedRowCount\":9,\"combinedLineMarkRecordIndexes\":[8,10,12,22,26,28,33,35,37],\"combinedMatchedByteOffsets\":[414,414,414,174,414,174,174,734,174],\"combinedRawRecordScanIndexes\":[3,3,3,2,3,2,2,6,2]"
    ));
    assert!(layer_tree.contains(
        "\"combinedLineStartCandidates\":[242,242,242,85,242,85,85,437,85],\"combinedLineEndCandidates\":[304,304,304,140,304,140,140,489,140],\"combinedField2Values\":[1024,1024,1024,768,1024,768,768,256,768],\"monotonicRawRecordScanIndex\":false,\"monotonicLineStartCandidate\":false,\"familyReusedAfterLaterFamily\":true,\"crossTableOrderingConsistent\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceOrderVsSubrecordOrderContradiction\":true,\"sourceOrderContradictionReasons\":[\"raw-record-scan-index-regresses-under-source-order\",\"subrecord-line-start-regresses-under-source-order\",\"subrecord-family-reused-after-later-family-under-source-order\"]"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkRawReferenceValueProbe\":{\"source\":\"/PageMark raw numeric scan+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tolerancePx\":2.000,\"referenceBBox\":{\"x\":174.000,\"y\":301.005,\"width\":421.000,\"height\":128.802},\"rowTopTargets\":[{\"row\":0,\"targetPx\":301.005,\"roundedTarget\":301,\"hitCount\":0,\"hits\":[]}"
    ));
    assert!(layer_tree.contains(
        "\"rowTopTargetCount\":4,\"rowTopTargetHitCount\":0,\"allRowTopTargetsHit\":false,\"totalHitCount\":0,\"rawHitRecordContextSummary\":{\"source\":\"/PageMark raw numeric scan context\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"hitContextCount\":0,\"distinctRecordIndexes\":[],\"allHitsInSingleRecordHeader\":false,\"distinctTailBlock16WordIndexes\":[],\"allHitsShareTailBlock16WordIndex\":false"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"reference-value-probe-only\",\"renderPromotionBlockedReason\":\"page-mark-raw-numeric-values-are-reference-probe-not-source-transform\""
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedHorizontalFieldAdjustmentProbe\":{\"source\":\"/PageMark selected u16 fields+referenceTableBBox+documentTextLineHeaders\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tableCandidateIndex\":0,\"pageMarkFieldSource\":\"crossTableRowBoundaryOffsetConsistency\",\"sourceLayoutCandidatePresent\":false,\"referenceBBox\":{\"x\":174.000,\"y\":301.005,\"width\":421.000,\"height\":128.802,\"right\":595.001}"
    ));
    assert!(layer_tree.contains(
        "\"wordIndex\":15,\"value\":423,\"valuePx\":423.000,\"target\":\"width\",\"targetPx\":421.000,\"residualPx\":2.000,\"absResidualPx\":2.000,\"withinTwoPx\":true"
    ));
    assert!(layer_tree.contains(
        "\"bestDirectWidthField\":{\"wordIndex\":15,\"value\":423,\"targetPx\":421.000,\"residualPx\":2.000,\"absResidualPx\":2.000}"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyHorizontalFieldConsensus\":{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark horizontal fields\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tableCandidateIndex\":0,\"pageMarkFieldSource\":\"crossTableRowBoundaryOffsetConsistency\",\"sparseTableCandidateIndex\":4,\"relatedTableCandidateIndexes\":[0,1,2,3],\"sourceDerivedRelatedTableCandidateIndexes\":[1,2,3],\"sourceDerivedRelatedTableCount\":3,\"stableFirstColumnSlotUnits\":20,\"stableFirstMatchedCellSpanUnits\":16,\"stableFirstIntercellGapUnits\":4,\"stableXUnitRange\":{\"start\":0,\"end\":100},\"stableFullExtentUnits\":144,\"allRelatedLayoutsHaveStableUnitFrame\":true"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyHorizontalFieldSelector\":{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark horizontal fields\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false,\"tableCandidateIndex\":0,\"pageMarkFieldSource\":\"crossTableRowBoundaryOffsetConsistency\",\"compactColumnCount\":3,\"selectionBasis\":\"compact-three-column-page-mark-word15-half-gap\",\"selectedFrameBasis\":\"page-mark-word14-first-slot-word15-half-gap\",\"pageMarkXWord14\":194,\"pageMarkWidthWord\":423,\"firstColumnSlotUnits\":20,\"firstIntercellGapUnits\":4,\"xAdjustmentUnits\":20.000,\"widthAdjustmentUnits\":2.000,\"xAdjustmentBasis\":\"cross-table-first-column-slot\",\"widthAdjustmentBasis\":\"cross-table-half-first-intercell-gap\",\"selectedX\":174.000,\"selectedWidth\":421.000"
    ));
    assert!(layer_tree.contains(
        "\"sourceWidthFieldRoleGate\":{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark width-field role gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false,\"pageMarkFieldSource\":\"crossTableRowBoundaryOffsetConsistency\",\"compactColumnCount\":3,\"pageMarkXWord14\":194,\"pageMarkWord15\":423,\"pageMarkWord21\":564,\"firstColumnSlotUnits\":20,\"firstIntercellGapUnits\":4,\"selectedWidthWordIndex\":15,\"selectedWidthWord\":423,\"selectedWidthFieldRole\":\"compact-three-column-visible-width\""
    ));
    assert!(layer_tree.contains(
        "\"selectedWidthAdjustmentBasis\":\"cross-table-half-first-intercell-gap\",\"selectedFrameBasis\":\"page-mark-word14-first-slot-word15-half-gap\",\"selectionBasis\":\"compact-three-column-page-mark-word15-half-gap\",\"twoColumnWidthCandidatePresent\":false,\"threeColumnWidthCandidatePresent\":true,\"selectorMatchesCompactColumnCount\":true,\"renderPromotionContribution\":\"source-horizontal-width-field-role-gate\",\"renderPromotionBlockedReason\":\"width-field-role-semantics-needs-cross-sample-validation\""
    ));
    assert!(layer_tree.contains(
        "\"frameBasis\":\"page-mark-word14-first-slot-word15-half-gap\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateBasis\":\"page-mark-u16-fields+cross-table-line-header-slots\",\"pageMarkXWord14\":194,\"pageMarkWidthWord\":423,\"firstColumnSlotUnits\":20,\"xAdjustmentUnits\":20.000,\"widthAdjustmentUnits\":2.000,\"xAdjustmentBasis\":\"cross-table-first-column-slot\",\"widthAdjustmentBasis\":\"cross-table-half-first-intercell-gap\",\"selectedX\":174.000,\"selectedWidth\":421.000"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkScopedYTransformProbe\":{\"source\":\"/PageMark scoped raw fields+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tolerancePx\":2.000,\"lineMarkRecordIndexes\":[8,10,12,14],\"parsedEntryMatchCount\":4,\"singleParsedPageMarkEntryMatched\":true,\"matchedParsedPageMarkEntryIndex\":0,\"rawHeaderMatchCount\":4,\"singleRawRecordHeaderMatched\":true,\"matchedRawRecordHeaderIndex\":0"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[8,10,12,14],\"parsedEntryMatchCount\":4,\"singleParsedPageMarkEntryMatched\":true,\"matchedParsedPageMarkEntryIndex\":0,\"rawHeaderMatchCount\":4,\"singleRawRecordHeaderMatched\":true,\"matchedRawRecordHeaderIndex\":0,\"referenceBBoxUsed\":true,\"referenceTargetBasis\":\"referenceTableBBox.rowTopTargets\",\"sourceOnlyReplacementBlockedReason\":\"page-mark-scoped-y-transform-targets-reference-backed\""
    ));
    assert!(layer_tree.contains(
        "\"rowDeltaCandidatePolicy\":\"adjacent-ordered-candidate-value-delta\",\"rowDeltaNearestCandidates\""
    ));
    assert!(layer_tree.contains(
        "\"targetPx\":301.005,\"nearestCandidate\":{\"source\":\"parsedEntryU16\",\"interpretation\":\"centipoint-to-css-px\",\"wordIndex\":25,\"byteOffset\":50,\"value\":22366,\"valuePx\":298.213,\"residualPx\":-2.792}"
    ));
    assert!(layer_tree.contains(
        "\"rowTopHitSummary\":{\"targetCount\":4,\"targetHitCount\":0,\"hitCount\":0,\"hits\":[]}"
    ));
    assert!(layer_tree.contains(
        "\"sharedFieldFamilyResiduals\":{\"source\":\"/PageMark scoped field families+/LineMark+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"parsedPageMarkFamily\":\"count-plus-one-variable\",\"familyKind\":\"u16-subrecord-field\",\"familyCount\":32,\"bestTableTopFamily\":null,\"bestRowTopFamily\":null,\"bestRowDeltaFamily\":null"
    ));
    assert!(layer_tree.contains(
        "\"rowDeltaResidualBasis\":\"adjacent-ordered-member-value-delta\",\"rowDeltaCandidateDeltasPx\""
    ));
    assert!(layer_tree.contains(
        "\"slotScopedSubrecordYSequenceComparison\":{\"source\":\"/PageMark raw u16 subrecord scan+/LineMark+/PageMark+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"grouping\":\"fieldIndex+tailBlock16WordIndex\",\"matchedRawRecordHeaderIndex\":0,\"lineMarkRecordIndexes\":[8,10,12,14],\"referenceRowTops\":[301.005,333.206,365.406,397.607],\"referenceRowDeltas\":[32.201,32.201,32.201],\"subrecordLineRangeCandidatePolicy\":\"words4To6WithinScannedRecordHeaderMaxLineEnd\",\"subrecordLineRangeMaxCandidate\":709,\"pageScaleCandidates\":{\"source\":\"/PageMark selected fields+layout\",\"pageWidthPx\":793.701,\"pageHeightPx\":1122.520,\"pageHeightPxPerWord21Unit\":1.990,\"pageHeightPxPerWord13Plus14Unit\":1.481,\"word21\":564,\"word13Plus14\":758},\"slotCount\":112,\"sameHeaderSlotCount\":0,\"sameHeaderBestTableTopSlot\":null,\"sameHeaderBestRowTopSlot\":null,\"sameHeaderBestRowDeltaSlot\":null,\"foreignHeaderSlotCount\":112,\"foreignHeaderBestTableTopSlot\":null,\"bestTableTopSlot\":null,\"bestRowTopSlot\":null,\"bestRowDeltaSlot\":null"
    ));
    assert!(layer_tree.contains(
        "\"orderedLineMarkRecordCoveragePolicy\":\"one-ordered-subrecord-member-per-line-mark-record\",\"bestOrderedLineMarkRecordCoverageSlot\""
    ));
    assert!(layer_tree.contains("\"orderedLineMarkRecordCoverageCount\":"));
    assert!(layer_tree.contains(
        "\"previousRowSpanScopedYTransformProbe\":{\"source\":\"/PageMark scoped raw fields+alternateLineMarkRecordSet+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"recordSet\":\"previous-row-span-line-mark-records\",\"tolerancePx\":2.000,\"lineMarkRecordIndexes\":[7,9,11,13],\"parsedEntryMatchCount\":4,\"singleParsedPageMarkEntryMatched\":true,\"matchedParsedPageMarkEntryIndex\":0,\"rawHeaderMatchCount\":4,\"singleRawRecordHeaderMatched\":true,\"matchedRawRecordHeaderIndex\":0"
    ));
    assert!(layer_tree.contains(
        "\"rowTopHitSummary\":{\"targetCount\":4,\"targetHitCount\":0,\"hitCount\":0,\"hits\":[]},\"sharedFieldFamilyResiduals\":{\"source\":\"/PageMark scoped field families+/LineMark+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"parsedPageMarkFamily\":\"count-plus-one-variable\",\"familyKind\":\"u16-subrecord-field\",\"familyCount\":32,\"bestTableTopFamily\":null,\"bestRowTopFamily\":null,\"bestRowDeltaFamily\":null"
    ));
    assert!(layer_tree.contains(
        "\"matchedRawRecordHeaderIndex\":0,\"lineMarkRecordIndexes\":[7,9,11,13],\"referenceRowTops\":[301.005,333.206,365.406,397.607],\"referenceRowDeltas\":[32.201,32.201,32.201],\"subrecordLineRangeCandidatePolicy\":\"words4To6WithinScannedRecordHeaderMaxLineEnd\",\"subrecordLineRangeMaxCandidate\":709"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[7,9,11,13],\"referenceRowTops\":[301.005,333.206,365.406,397.607],\"referenceRowDeltas\":[32.201,32.201,32.201],\"subrecordLineRangeCandidatePolicy\":\"words4To6WithinScannedRecordHeaderMaxLineEnd\",\"subrecordLineRangeMaxCandidate\":709,\"pageScaleCandidates\":{\"source\":\"/PageMark selected fields+layout\",\"pageWidthPx\":793.701,\"pageHeightPx\":1122.520,\"pageHeightPxPerWord21Unit\":1.990,\"pageHeightPxPerWord13Plus14Unit\":1.481,\"word21\":564,\"word13Plus14\":758},\"slotCount\":112,\"sameHeaderSlotCount\":0,\"sameHeaderBestTableTopSlot\":null,\"sameHeaderBestRowTopSlot\":null,\"sameHeaderBestRowDeltaSlot\":null,\"foreignHeaderSlotCount\":112,\"foreignHeaderBestTableTopSlot\":null,\"bestTableTopSlot\":null,\"bestRowTopSlot\":null,\"bestRowDeltaSlot\":null"
    ));
    assert!(layer_tree.contains(
        "\"absoluteVsSpanLineageGate\":{\"source\":\"/PageMark y-candidate lineage: reference table-top vs source line-span\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":true,\"selectionReady\":false,\"promotionReady\":false,\"lineageClassification\":\"reference-absolute-table-top-vs-source-line-span-correlation\",\"absoluteTableTopProbe\":{\"source\":\"referenceTableBBox.rowTopTargets\",\"referenceBacked\":true,\"sourceBacked\":false,\"referenceTableTopY\":301.005"
    ));
    assert!(layer_tree.contains(
        "\"sourceSpanProbe\":{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\",\"referenceBacked\":false,\"sourceBacked\":true,\"spanEvidencePositional\":false,\"present\":true,\"selectedSpanRole\":\"post-row-gap\",\"previousSpanRole\":\"compact-row-span\",\"selectedPostRowGapSpanTargets\":[65,65,65,67],\"selectedPostRowGapSpanHitCount\":3,\"selectedPostRowGapSpanComplete\":false,\"previousRowSpanTargets\":[107,113,113,113]"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyLineageConclusion\":\"source-spans-corroborate-spacing-or-row-span-lengths-not-absolute-y\",\"blockedReasons\":[\"absolute-table-top-targets-reference-backed\",\"source-subrecord-spans-are-line-span-targets\",\"source-only-absolute-table-top-field-unproven\"],\"renderPromotionContribution\":\"page-mark-y-candidate-lineage-gate\",\"renderPromotionBlockedReason\":\"absolute-top-evidence-reference-backed-span-evidence-non-positional\""
    ));
    assert!(layer_tree.contains(
        "\"subrecordLineSpanCorrelation\":{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"spanToleranceUnits\":3,\"selectedSpacingRecordIndexes\":[8,10,12,14],\"previousRowSpanRecordIndexes\":[7,9,11,13],\"selectedPostRowGapSpanTargets\":[65,65,65,67],\"postRowGapSpanTargets\":[65,65,65,67],\"previousRowSpanTargets\":[107,113,113,113],\"compactRowSpanTargets\":[107,113,113,113]"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[33,35,37],\"uniformLineMarkRecordStride\":true,\"lineMarkRecordStride\":2,\"interleavedLineMarkRecordCountBetweenRows\":1"
    ));
    assert!(layer_tree.contains(
        "\"candidateRowCount\":3,\"matchedRowCount\":3,\"allCandidateRowsMatched\":true,\"candidateSegmentCount\":6,\"matchedSegmentCount\":6,\"allCandidateSegmentsMatched\":true,\"sharedSourceIntervalIndexes\":[20,22,24],\"compactToSparseColumnOffsetCandidate\":3,\"matchedSparseColumnIndexes\":[3,4]"
    ));
    assert!(layer_tree.contains(
        "\"sparseSiblingColumnPromotionReadiness\":{\"source\":\"sparseTableSiblingEvidence+documentTextLineHeaders column promotion readiness\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"columnSplitReady\":true,\"requestedColumnCount\":2,\"candidateRowCount\":3,\"matchedRowCount\":3,\"candidateSegmentCount\":6,\"matchedSegmentCount\":6,\"sparseTopologyComplete\":true,\"compactToSparseColumnOffsetCandidate\":3,\"matchedSparseColumnIndexes\":[3,4],\"compactLineHeaderCellCoverageComplete\":true,\"decodedSourcePlacementMatchCount\":6,\"decodedSourcePlacementRequiredCellCount\":6,\"sourceLineHeaderColumnWidthsPresent\":true,\"sourceColumnWidthBasis\":\"documentTextLineHeaderCellSlotUnits\",\"sourceColumnWidthFractions\":[0.683,0.317],\"blockedReasons\":[],\"renderPromotionContribution\":\"sparse-sibling-column-readiness-gate\",\"renderPromotionBlockedReason\":null}"
    ));
    assert!(layer_tree.contains(
        "\"sourceRowHeightBasis\":\"documentTextLineHeaderFontSizeUnits\",\"homogeneousFontSizeUnits\":12,\"lineHeaderRowCount\":3,\"lineHeaderRowsWithHeaders\":3,\"rawHeaderCount\":12,\"sourceRowHeightPx\":21.000"
    ));
    assert!(layer_tree.contains(
        "\"postRowGapLineMarkCorrelation\":{\"source\":\"sparseTableSiblingEvidence+/LineMark\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"rowCount\":3,\"matchedGapCount\":3,\"exactSpanMatchCount\":3,\"allRowsExactSpanMatched\":true"
    ));
    assert!(layer_tree.contains(
        "\"compactRow\":2,\"sparseRow\":23,\"sourceIntervalIndex\":24,\"lineMarkRecordIndex\":37,\"lineMarkUnitRange\":{\"start\":2179,\"end\":2233},\"lineMarkSpanUnits\":54,\"postRowGapSourceRange\":{\"start\":2261,\"end\":2315},\"postRowGapUnits\":54,\"postRowGapKind\":\"trailing-empty-sparse-rows\",\"gapSparseRowIndexes\":[24,25],\"gapSparseSourceIntervalIndexes\":[25,26],\"lineMarkSpanMinusGapUnits\":0,\"exactSpanMatch\":true"
    ));
    assert!(layer_tree.contains(
        "\"selectedRecordPostRowGapSpanMatchCount\":3,\"allSelectedRecordsMatchPostRowGapSpan\":true,\"previousRecordRowSpanMatchCount\":3,\"allPreviousRecordsMatchRowSpan\":true,\"nextRecordNextRowSpanMatchCount\":2,\"rowsWithNextRow\":2,\"sequenceInterpretationCandidate\":\"alternating-row-span-record-then-post-row-gap-record\""
    ));
    assert!(layer_tree.contains(
        "\"rowSourceUnitRange\":{\"start\":1857,\"end\":1968},\"rowSpanUnits\":111,\"selectedLineMarkRecord\":{\"recordIndex\":33,\"byteOffset\":150,\"wordIndex\":75,\"delta\":58,\"unitRange\":{\"start\":1886,\"end\":1944},\"flagWord\":2,\"flagWordHex\":\"0x0002\""
    ));
    assert!(layer_tree.contains(
        "\"selectedRecordMatchesPostRowGapSpan\":true,\"previousLineMarkRecord\":{\"recordIndex\":32,\"byteOffset\":146,\"wordIndex\":73,\"delta\":111,\"unitRange\":{\"start\":1775,\"end\":1886},\"flagWord\":2,\"flagWordHex\":\"0x0002\""
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRowGapSequenceYComparison\":{\"source\":\"/LineMark row/gap sequence+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"rowCountCompared\":3,\"referenceRowTops\":[768.014,805.314,842.615],\"selectedSpacingRecordIndexes\":[33,35,37],\"previousRowSpanRecordIndexes\":[32,34,36]"
    ));
    assert!(layer_tree.contains(
        "\"selectedSpacingRecordCandidate\":{\"family\":\"selected-spacing-records\",\"spanInterpretation\":\"post-row-gap-span\",\"recordIndexes\":[33,35,37],\"uniformRecordStride\":true,\"recordStride\":2,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":42"
    ));
    assert!(layer_tree.contains(
        "\"rowHeightPitchResidualsPx\":[-3.014,1.686,6.385],\"rowHeightPitchMeanAbsResidualPx\":3.695,\"rowHeightPitchMaxAbsResidualPx\":6.385,\"pageLinePitchPx\":23.298,\"pageLinePitchRowTops\":[840.837,887.433,934.029],\"pageLinePitchResidualsPx\":[72.823,82.119,91.414]"
    ));
    assert!(layer_tree.contains(
        "\"previousRowSpanRecordCandidate\":{\"family\":\"previous-row-span-records\",\"spanInterpretation\":\"compact-row-span\",\"recordIndexes\":[32,34,36],\"uniformRecordStride\":true,\"recordStride\":2,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":42"
    ));
    assert!(layer_tree.contains(
        "\"rowHeightPitchResidualsPx\":[-24.014,-19.314,-14.615],\"rowHeightPitchMeanAbsResidualPx\":19.314,\"rowHeightPitchMaxAbsResidualPx\":24.014,\"pageLinePitchPx\":23.298,\"pageLinePitchRowTops\":[817.539,864.135,910.731],\"pageLinePitchResidualsPx\":[49.525,58.821,68.116]"
    ));
    assert!(layer_tree.contains(
        "\"bestCandidate\":\"selected-spacing-records-row-height-pitch\",\"bestCandidateMaxAbsResidualPx\":6.385,\"renderPromotionContribution\":\"row-gap-record-family-y-diagnostic-only\""
    ));
    assert!(layer_tree.contains(
        "\"lineMarkPageOriginStrideCandidate\":{\"source\":\"/LineMark+/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"diagnosticOnly\":true,\"lineMarkRecordIndexes\":[32,34,36],\"recordStride\":2"
    ));
    assert!(layer_tree.contains(
        "\"rawRecordIndexRowTops\":[744.000,786.000,828.000],\"strideCollapsedRowTops\":[408.000,429.000,450.000]"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkStrideYComparison\":{\"source\":\"lineMarkPageOriginStrideCandidate+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"lineMarkRecordIndexes\":[32,34,36],\"recordStride\":2"
    ));
    assert!(layer_tree.contains(
        "\"referenceColumnWidthBasis\":\"documentTextLineHeaderCellSlotUnits\",\"referenceColumnWidthsPx\":[378.342,175.659]"
    ));
    assert!(layer_tree.contains(
        "\"referenceColumnPxPerMatchedUnit\":[23.646,2.196],\"equalReferenceColumnsConflictWithUnitSpans\":false"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"reference-bbox-uses-source-column-widths-but-not-source-placement\""
    ));
    assert!(layer_tree.contains(
        "\"pageMarkSelectedFields\":[{\"wordIndex\":10,\"value\":564,\"hex\":\"0x0234\"},{\"wordIndex\":13,\"value\":564,\"hex\":\"0x0234\"},{\"wordIndex\":14,\"value\":194,\"hex\":\"0x00c2\""
    ));
    assert!(layer_tree.contains(
        "\"pageMarkRawFieldReferenceComparison\":{\"source\":\"/PageMark selected u16 fields+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"comparisonBasis\":\"direct-u16-px-near-reference\",\"word14DirectPx\":194.000,\"referenceX\":174.000,\"word14MinusReferenceXPx\":20.000,\"word21DirectPx\":564.000,\"referenceWidth\":554.001,\"word21MinusReferenceWidthPx\":9.999,\"firstColumnSlotUnits\":20,\"firstMatchedCellSpanUnits\":16,\"firstIntercellGapUnits\":4,\"word14MinusReferenceXInFirstSlotUnits\":1.000,\"word21MinusReferenceWidthInHalfFirstSlotUnits\":1.000"
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedHorizontalFieldAdjustmentProbe\":{\"source\":\"/PageMark selected u16 fields+referenceTableBBox+documentTextLineHeaders\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tableCandidateIndex\":3,\"pageMarkFieldSource\":\"sourceDerivedLayoutCandidate\",\"sourceLayoutCandidatePresent\":true,\"referenceBBox\":{\"x\":174.000,\"y\":768.014,\"width\":554.001,\"height\":111.902,\"right\":728.001}"
    ));
    assert!(layer_tree.contains(
        "\"lineHeaderSlotEvidence\":{\"rowCount\":3,\"matchedRowCount\":3,\"rawHeaderCount\":12,\"firstColumnSlotUnits\":20,\"firstMatchedCellSpanUnits\":16,\"firstIntercellGapUnits\":4}"
    ));
    assert!(layer_tree.contains(
        "\"slotAdjustedFieldTargetComparisons\":[{\"wordIndex\":14,\"value\":194,\"valuePx\":194.000,\"target\":\"x\",\"targetPx\":174.000,\"adjustmentBasis\":\"line-header-first-column-slot\",\"adjustmentUnits\":20.000,\"adjustedValuePx\":174.000,\"residualPx\":-0.000,\"absResidualPx\":0.000},{\"wordIndex\":21,\"value\":564,\"valuePx\":564.000,\"target\":\"width\",\"targetPx\":554.001,\"adjustmentBasis\":\"line-header-half-first-column-slot\",\"adjustmentUnits\":10.000,\"adjustedValuePx\":554.000,\"residualPx\":-0.001,\"absResidualPx\":0.001}]"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyHorizontalFieldConsensus\":{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark horizontal fields\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tableCandidateIndex\":3,\"pageMarkFieldSource\":\"sourceDerivedLayoutCandidate\",\"sparseTableCandidateIndex\":4,\"relatedTableCandidateIndexes\":[0,1,2,3],\"sourceDerivedRelatedTableCandidateIndexes\":[1,2,3],\"sourceDerivedRelatedTableCount\":3,\"stableFirstColumnSlotUnits\":20"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyHorizontalFieldSelector\":{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark horizontal fields\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false,\"tableCandidateIndex\":3,\"pageMarkFieldSource\":\"sourceDerivedLayoutCandidate\",\"compactColumnCount\":2,\"selectionBasis\":\"compact-two-column-page-mark-word21-half-slot\",\"selectedFrameBasis\":\"page-mark-word14-first-slot-word21-half-slot\",\"pageMarkXWord14\":194,\"pageMarkWidthWord\":564,\"firstColumnSlotUnits\":20,\"firstIntercellGapUnits\":4,\"xAdjustmentUnits\":20.000,\"widthAdjustmentUnits\":10.000,\"xAdjustmentBasis\":\"cross-table-first-column-slot\",\"widthAdjustmentBasis\":\"cross-table-half-first-column-slot\",\"selectedX\":174.000,\"selectedWidth\":554.000"
    ));
    assert!(layer_tree.contains(
        "\"sourceWidthFieldRoleGate\":{\"source\":\"sparseSiblingSourceDerivedLayouts+/PageMark width-field role gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false,\"pageMarkFieldSource\":\"sourceDerivedLayoutCandidate\",\"compactColumnCount\":2,\"pageMarkXWord14\":194,\"pageMarkWord15\":423,\"pageMarkWord21\":564,\"firstColumnSlotUnits\":20,\"firstIntercellGapUnits\":4,\"selectedWidthWordIndex\":21,\"selectedWidthWord\":564,\"selectedWidthFieldRole\":\"compact-two-column-visible-width\""
    ));
    assert!(layer_tree.contains(
        "\"selectedWidthAdjustmentBasis\":\"cross-table-half-first-column-slot\",\"selectedFrameBasis\":\"page-mark-word14-first-slot-word21-half-slot\",\"selectionBasis\":\"compact-two-column-page-mark-word21-half-slot\",\"twoColumnWidthCandidatePresent\":true,\"threeColumnWidthCandidatePresent\":false,\"selectorMatchesCompactColumnCount\":true,\"renderPromotionContribution\":\"source-horizontal-width-field-role-gate\",\"renderPromotionBlockedReason\":\"width-field-role-semantics-needs-cross-sample-validation\""
    ));
    assert!(layer_tree.contains(
        "\"frameBasis\":\"page-mark-word14-first-slot-word21-half-slot\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateBasis\":\"page-mark-u16-fields+cross-table-line-header-slots\",\"pageMarkXWord14\":194,\"pageMarkWidthWord\":564,\"firstColumnSlotUnits\":20,\"xAdjustmentUnits\":20.000,\"widthAdjustmentUnits\":10.000,\"xAdjustmentBasis\":\"cross-table-first-column-slot\",\"widthAdjustmentBasis\":\"cross-table-half-first-column-slot\",\"selectedX\":174.000,\"selectedWidth\":554.000"
    ));
    assert!(layer_tree.contains(
        "\"rawRecordIndexResidualsPx\":[-24.014,-19.314,-14.615],\"rawRecordIndexMeanAbsResidualPx\":19.314,\"rawRecordIndexMaxAbsResidualPx\":24.014"
    ));
    assert!(layer_tree.matches("\"recordIndexAffineFit\"").count() >= 2);
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[32,34,36],\"recordStride\":2,\"rowCountCompared\":3,\"referenceRowTops\":[768.014,805.314,842.615]"
    ));
    assert!(layer_tree.contains(
        "\"sourceRawSlopeResidualPxPerRecord\":2.350,\"sourceStrideCollapsedSlopePxPerRecord\":10.500,\"sourceStrideCollapsedSlopeResidualPxPerRecord\":-8.150"
    ));
    assert!(layer_tree.contains(
        "\"strideCollapsedResidualsPx\":[-360.014,-376.314,-392.615],\"strideCollapsedMeanAbsResidualPx\":376.314,\"strideCollapsedMaxAbsResidualPx\":392.615"
    ));
    assert!(layer_tree.contains(
        "\"bestYHypothesisCandidate\":\"raw-record-index\",\"bestHypothesisMaxAbsResidualPx\":24.014"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkStridePromotionReadiness\":{\"source\":\"/LineMark+/PageMark+sparseSiblingEvidence\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":false,\"candidateRowCount\":3,\"candidateSegmentCount\":6,\"allRowsHaveLineMark\":true,\"lineMarkRecordIndexes\":[32,34,36],\"uniformRecordStride\":true,\"recordStride\":2"
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutReadiness\":{\"source\":\"sourceDerivedLayoutGate+documentTextLineHeaders+/LineMark\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"sourcePlacementEvidencePresent\":true,\"candidateRowCount\":3,\"requestedColumnCount\":2,\"lineHeaderRowCount\":3,\"rawHeaderCount\":12,\"matchedRowCount\":3,\"fullMatchedRowCount\":3,\"matchedCellHeaderCount\":6,\"requiredCellHeaderCount\":6,\"commonMatchedColumnCount\":2,\"rowsWithoutHeaders\":[],\"rowsWithoutMatchedCellHeaders\":[],\"rowsWithPartialCellHeaderCoverage\":[],\"lineHeaderRowsHomogeneous\":true,\"lineMarkRowRecordSelection\":\"previous-compact-row-span-record\",\"lineMarkRowsExactAndContiguous\":false,\"sourceDerivedLayoutCandidatePresent\":true,\"sourceDerivedLayoutRenderable\":false,\"sourceDerivedLayoutBlockedReason\":\"line-mark-record-stride-to-page-y-transform-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"rejectionReasons\":[\"line-mark-rows-not-exact-source-boundaries\",\"line-mark-record-stride-to-page-y-transform-unproven\",\"source-derived-layout-not-renderable\"],\"renderPromotionContribution\":\"source-derived-layout-readiness-gate\",\"renderPromotionBlockedReason\":\"source-derived-layout-not-renderable\""
    ));
    assert!(layer_tree.contains(
        "\"pageSpaceSolver\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\",\"solverVersion\":\"table-page-space-v1\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"solverStage\":\"blocked-y-origin-transform\",\"sourcePlacementEvidencePresent\":true,\"candidateRowCount\":3,\"requestedColumnCount\":2,\"commonMatchedColumnCount\":2,\"matchedCellHeaderCount\":6,\"requiredCellHeaderCount\":6"
    ));
    assert!(layer_tree.contains(
        "\"horizontalSolverReady\":true,\"rowHeightSolverReady\":true,\"yOriginSolverReady\":false,\"lineHeaderRowsHomogeneous\":true,\"lineMarkRowRecordSelection\":\"previous-compact-row-span-record\",\"lineMarkRowsExactAndContiguous\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidatePresent\":true,\"sourceDerivedLayoutRenderable\":false,\"pageOriginAuthority\":\"lineMarkPageGridStrideRawRecordIndex\",\"lineMarkPageOriginPresent\":false,\"lineMarkPageOriginStridePresent\":true"
    ));
    assert!(layer_tree.contains(
        "\"referenceCalibrationReplacementGate\":{\"source\":\"table-page-space-v1 reference calibration replacement gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"replacementReady\":false,\"sourceLayoutCandidatePresent\":true,\"sourceLayoutRenderable\":false,\"horizontalSolverReady\":true,\"sourceColumnSplitReady\":true,\"pageSpaceHorizontalTransformReady\":false,\"rowHeightSolverReady\":true,\"yOriginSolverReady\":false"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"table-horizontal-page-space-transform-incomplete\",\"source-page-y-transform-not-decoded\",\"source-derived-layout-not-renderable\"],\"renderPromotionContribution\":\"reference-calibration-replacement-gate\",\"renderPromotionBlockedReason\":\"source-table-page-space-not-ready\""
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyAxisAdmissionGate\":{\"source\":\"pageSpaceHorizontalTransformGate+sourcePageYTransformGate source-only selector coupling\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"admissionReady\":false,\"activeSourceLayoutAdmissionReady\":false,\"activeSourceLayoutAdmissionBasis\":null,\"sourceOnlySelectorFallbackIgnoredByActiveSourceLayout\":false,\"sourceLayoutCandidatePresent\":true,\"sourceLayoutRenderable\":false,\"horizontalAxisReady\":false,\"horizontalSelectorCandidatePresent\":true,\"horizontalSelectorInBestAgreementGroup\":true,\"horizontalCandidateCount\":6,\"horizontalAgreementGroupCount\":4,\"horizontalBestSupportCount\":3,\"horizontalUniqueBestSupported\":true,\"horizontalBestSupportedSelectedX\":174.000,\"horizontalBestSupportedSelectedWidth\":554.000"
    ));
    assert!(layer_tree.contains(
        "\"horizontalBestSupportedFrameBases\":[\"page-mark-word14-word21-first-slot-adjusted\",\"page-mark-word14-first-slot-word21-half-slot\",\"page-mark-word14-first-slot-word21-half-slot\"],\"yAxisReady\":false,\"ySelectorCandidatePresent\":true,\"ySelectorSingleSupportFallback\":true,\"ySelectorSupportFragmentedByTable\":false,\"ySelectorSupportCount\":1,\"ySelectorCrossTableSupportPresent\":false,\"ySelectorAgreementAdmissible\":false,\"ySelectorAdmissionBlockedReason\":\"source-y-origin-selector-single-support-fallback-not-render-admissible\",\"ySelectorSupportBlockedReasons\":[\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"page-mark-absolute-y-slot-semantics-unproven\"],\"sourceGapToPageLineGapTransformAdmissionGate\":{\"source\":\"sourceOnlyAxisAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate\""
    ));
    assert!(layer_tree.contains(
        "\"source\":\"sourceOnlyAxisAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"transformDomain\":\"source-unit-gap-to-page-mark-line-index-gap\",\"canDecodeSourceTransform\":false,\"tableFamilyTransformStable\":false,\"tableFamilyTransformBlockedReason\":\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"transitionCount\":3,\"allTransitionsSamePageMarkEntry\":true,\"bestCandidateTransformKind\":\"segment-offset-gap\",\"bestCandidateMaxAbsDeltaUnits\":105,\"transformCandidateCount\":4,\"exactTransformCandidateCount\":0"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkAbsoluteYSlotSemanticsReady\":false,\"pageMarkAbsoluteYSlotBlockedReason\":\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"pageMarkAbsoluteYSlotResidualPx\":107.539,\"yCandidateCount\":12,\"yAgreementGroupCount\":11,\"yBestSupportCount\":2,\"yUniqueBestSupported\":true,\"ySelectedOriginBasis\":\"page-mark-absolute-y-slot-field2-tail-block16-word11\",\"ySelectedY\":768.000,\"ySelectedRowHeight\":null,\"ySelectorTableCandidateIndexes\":[3]"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyAxisCandidateBBox\":{\"source\":\"sourceOnlyAxisAdmissionGate.sourceOnlyAxisCandidateBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"candidatePresent\":true,\"bboxPresent\":true,\"horizontalCandidatePresent\":true,\"yCandidatePresent\":true,\"rowHeightCandidatePresent\":true,\"rowCount\":3,\"horizontalFrameBasis\":\"page-mark-word14-word21-first-slot-adjusted\",\"yOriginBasis\":\"page-mark-absolute-y-slot-field2-tail-block16-word11\",\"rowHeight\":21.000,\"bbox\":{\"x\":174.000,\"y\":768.000,\"width\":554.000,\"height\":63.000}"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"source-horizontal-axis-not-render-admissible\",\"source-y-origin-selector-single-support-fallback\",\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"source-y-axis-not-render-admissible\",\"source-derived-layout-not-renderable\"],\"renderPromotionContribution\":\"source-only-axis-selector-admission-gate\",\"renderPromotionBlockedReason\":\"source-page-space-axis-selector-coupling-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"pageSpaceHorizontalTransformGate\":{\"source\":\"documentTextLineHeaders+/LineMark page-space horizontal transform gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":false,\"sourceLayoutCandidatePresent\":true,\"sourceColumnSplitReady\":true,\"xUnitAllRowsAgree\":true,\"fullExtentUnitsPresent\":true,\"sourceFrameDecoded\":false,\"pageOriginAuthority\":\"lineMarkPageGridStrideRawRecordIndex\",\"lineMarkRowsExactAndContiguous\":false,\"sourceDerivedLayoutBlockedReason\":\"line-mark-record-stride-to-page-y-transform-unproven\",\"xUnitRangeBasis\":\"matched-cells\",\"xUnitRange\":{\"start\":0,\"end\":100},\"fullExtentUnits\":144,\"xOriginInsetBasis\":\"none\""
    ));
    assert!(layer_tree.contains(
        "\"sourceFrameHypotheses\":[{\"frameBasis\":\"page-body-frame\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"frameX\":72.000,\"frameWidth\":649.701,\"selectedX\":72.000,\"selectedWidth\":451.181"
    ));
    assert!(layer_tree.contains(
        "{\"frameBasis\":\"page-media-box\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"frameX\":0.000,\"frameWidth\":793.701,\"selectedX\":0.000,\"selectedWidth\":551.181"
    ));
    assert!(layer_tree.contains(
        "{\"frameBasis\":\"page-mark-word14-word21-direct\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateBasis\":\"page-mark-direct-u16-px\",\"pageMarkWord14\":194,\"pageMarkWord21\":564,\"firstColumnSlotUnits\":null,\"xAdjustmentUnits\":0.000,\"widthAdjustmentUnits\":0.000,\"adjustmentBasis\":\"none\",\"selectedX\":194.000,\"selectedWidth\":564.000"
    ));
    assert!(layer_tree.contains(
        "{\"frameBasis\":\"page-mark-word14-word21-first-slot-adjusted\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateBasis\":\"page-mark-direct-u16-px\",\"pageMarkWord14\":194,\"pageMarkWord21\":564,\"firstColumnSlotUnits\":20,\"xAdjustmentUnits\":20.000,\"widthAdjustmentUnits\":10.000,\"adjustmentBasis\":\"line-header-first-column-slot\",\"selectedX\":174.000,\"selectedWidth\":554.000"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"source-horizontal-page-mark-raw-field-hypothesis\",\"renderPromotionBlockedReason\":\"page-mark-raw-horizontal-field-semantics-unproven\"},{\"frameBasis\":\"page-mark-word14-first-slot-word15-direct\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateBasis\":\"page-mark-u16-fields+cross-table-line-header-slots\",\"pageMarkXWord14\":194,\"pageMarkWidthWord\":423,\"firstColumnSlotUnits\":20,\"xAdjustmentUnits\":20.000,\"widthAdjustmentUnits\":0.000,\"xAdjustmentBasis\":\"cross-table-first-column-slot\",\"widthAdjustmentBasis\":\"none\",\"selectedX\":174.000,\"selectedWidth\":423.000"
    ));
    assert!(layer_tree.contains(
        "\"sourceFrameCandidateAgreementGate\":{\"source\":\"pageSpaceHorizontalTransformGate.sourceFrameHypotheses agreement\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"selectionReady\":false,\"candidateCount\":6,\"agreementGroupCount\":4,\"bestSupportCount\":3,\"uniqueBestSupported\":true,\"sourceOnlyUniqueSelectionCandidatePresent\":true,\"sourceOnlyUniqueSelectionDiagnosticOnly\":true,\"sourceOnlyUniqueSelectionPromotionReady\":false,\"sourceOnlyUniqueSelectionPromotionBlockedReason\":\"source-horizontal-field-semantics-unproven\",\"bestSupportedSelectedX\":174.000,\"bestSupportedSelectedWidth\":554.000,\"bestSupportedFrameBases\":[\"page-mark-word14-word21-first-slot-adjusted\",\"page-mark-word14-first-slot-word21-half-slot\",\"page-mark-word14-first-slot-word21-half-slot\"]"
    ));
    assert!(layer_tree.contains(
        "\"agreementGroups\":[{\"selectedX\":174.000,\"selectedWidth\":421.000,\"supportCount\":1,\"frameBases\":[\"page-mark-word14-first-slot-word15-half-gap\"]"
    ));
    assert!(layer_tree.contains(
        "{\"selectedX\":174.000,\"selectedWidth\":554.000,\"supportCount\":3,\"frameBases\":[\"page-mark-word14-word21-first-slot-adjusted\",\"page-mark-word14-first-slot-word21-half-slot\",\"page-mark-word14-first-slot-word21-half-slot\"],\"contributions\":[\"source-horizontal-page-mark-raw-field-hypothesis\",\"source-only-horizontal-field-consensus\",\"source-only-horizontal-field-selector\"]"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"page-space-horizontal-frame-not-decoded\",\"line-mark-rows-not-exact-source-boundaries\",\"line-mark-record-stride-to-page-y-transform-unproven\"],\"renderPromotionContribution\":\"source-page-space-horizontal-transform-gate\",\"renderPromotionBlockedReason\":\"table-horizontal-page-space-transform-incomplete\""
    ));
    assert!(layer_tree.contains(
        "\"sourcePageYTransformGate\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark y-origin promotion gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":false,\"lineMarkRowsExactAndContiguous\":false,\"pageOriginAuthority\":\"lineMarkPageGridStrideRawRecordIndex\",\"lineMarkPageOriginPresent\":false,\"lineMarkPageOriginStridePresent\":true,\"subrecordLineSpanReadinessPresent\":true,\"selectedPostRowGapSpanComplete\":true,\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":false"
    ));
    assert!(layer_tree.contains(
        "\"selectedPostRowGapSpanOrderedCoverage\":{\"policy\":\"one-tolerance-hit-with-unique-subrecord-candidate-per-line-mark-record\",\"matchedRecordIndexes\":[33,35,37],\"matchedCandidateByteOffsets\":[174,734,174],\"uniqueCandidateByteOffsets\":[174,734],\"duplicateCandidateByteOffsets\":[174],\"matchedRecordCount\":3,\"uniqueCandidateCount\":2,\"duplicateCandidateReuseCount\":1,\"orderedUniqueCoverageComplete\":false}"
    ));
    assert!(layer_tree.contains(
        "\"subrecordSpanRoleGate\":{\"source\":\"/PageMark raw u16 subrecord line-span role classifier\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"dominantSpanRole\":\"selected-post-row-gap\",\"dominantSpanRoleHitCount\":3,\"selectedPostRowGapSpanHitCount\":3,\"selectedPostRowGapSpanTargetCount\":3,\"selectedPostRowGapSpanComplete\":true,\"rowSpanHitCount\":0,\"rowSpanTargetCount\":3,\"previousRowSpanHitCount\":0,\"compactRowSpanHitCount\":0,\"rowSpanComplete\":false,\"selectedPostRowGapRoleDominant\":true,\"rowSpanRoleDominant\":false"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkStrideToPageYPromotionReadiness\":{\"source\":\"/LineMark+/PageMark stride-to-page-y promotion readiness\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"promotionReady\":false,\"strideCandidatePresent\":true,\"lineMarkPageOriginPresent\":false,\"selectedPostRowGapSpanComplete\":true,\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":false,\"sourceOnlyStrideRowCoverage\":{\"source\":\"/LineMark source unit ranges+table row source unit ranges\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateSpace\":\"documentTextSourceUnits\",\"policy\":\"previous-line-mark-record-span-equals-table-row-source-span\",\"candidateRowCount\":3,\"matchedRowCount\":3,\"allRowsCovered\":true,\"lineMarkRecordSelection\":\"previous-compact-row-span-record\",\"lineMarkRecordIndexes\":[32,34,36],\"uniformLineMarkRecordStride\":true,\"lineMarkRecordStride\":2,\"matchesStrideCandidateRecordIndexes\":true,\"rowSpanUnits\":[111,92,91],\"lineMarkSpanUnits\":[111,92,91],\"rowSpanResidualUnits\":[0,0,0],\"pageYTransformDecoded\":false,\"renderPromotionContribution\":\"source-only-line-mark-row-span-coverage\",\"renderPromotionBlockedReason\":null}"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYOriginHypothesis\":{\"source\":\"sourcePageYTransformGate source-only page-y origin hypothesis\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"candidatePresent\":true,\"candidateKind\":\"line-mark-page-origin-stride\",\"yOriginReadinessClass\":\"stride-only\",\"originDecisionReady\":false,\"yOriginReadinessBlockedReasons\":[\"line-mark-page-origin-stride-present\",\"stride-origin-needs-direct-line-origin-rule\",\"direct-line-mark-page-origin-absent\",\"decoded-page-y-origin-missing\"],\"lineMarkPageOriginPresent\":false,\"lineMarkPageOriginStridePresent\":true,\"pageMarkAbsoluteYSlotCandidatePresent\":true,\"pageMarkAbsoluteYSlotY\":768.000,\"pageMarkAbsoluteYSlotBlockedReason\":\"page-mark-absolute-y-slot-semantics-unproven\",\"pageOriginAuthority\":\"lineMarkPageGridStrideRawRecordIndex\""
    ));
    assert!(layer_tree.contains(
        "\"pageMarkAbsoluteYSlotOrigin\":{\"source\":\"rawRecordHeaderTailU16Subrecord\",\"interpretation\":\"direct-u16-px\",\"fieldIndex\":2,\"tailBlock16WordIndex\":11"
    ));
    assert!(layer_tree.contains(
        "\"strideLineMarkPageOrigin\":{\"lineMarkRecordIndexes\":[32,34,36],\"recordStride\":2,\"firstLineMarkRecordIndex\":32,\"lastLineMarkRecordIndex\":36,\"pageMarkEntryIndex\":0,\"pageLineStart\":0,\"pageLineEnd\":42"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"stride-origin-needs-page-origin-rule\",\"line-mark-record-stride-to-page-y-transform-unproven\",\"line-mark-rows-not-exact-source-boundaries\",\"page-origin-authority-not-renderable-line-mark-page-grid\""
    ));
    assert!(layer_tree.contains(
        "\"originBases\":[\"line-mark-stride-raw-record-index-first-row\"],\"tableCandidateIndexes\":[],\"contributions\":[\"source-only-line-mark-stride-page-y-origin\"],\"blockedReasons\":[\"stride-origin-needs-page-origin-rule\"]"
    ));
    assert!(layer_tree.contains(
        "\"originBases\":[\"line-mark-stride-collapsed-record-index-first-row\"],\"tableCandidateIndexes\":[],\"contributions\":[\"source-only-line-mark-stride-page-y-origin\"],\"blockedReasons\":[\"line-mark-record-stride-to-page-y-transform-unproven\"]"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYOriginSelector\":{\"source\":\"sourceOnlyPageYOriginCandidateAgreementGate best-supported group selector\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false,\"tableCandidateIndex\":1,\"selectionBasis\":\"single-support-source-only-y-origin-fallback\",\"singleSupportFallback\":true,\"selectedOriginBasis\":\"line-mark-stride-raw-record-index-first-row\",\"selectedY\":534.000,\"selectedRowHeight\":21.000"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYOriginSelector\":{\"source\":\"sourceOnlyPageYOriginCandidateAgreementGate best-supported group selector\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false,\"tableCandidateIndex\":2,\"selectionBasis\":\"single-support-source-only-y-origin-fallback\",\"singleSupportFallback\":true,\"selectedOriginBasis\":\"line-mark-stride-raw-record-index-first-row\",\"selectedY\":618.000,\"selectedRowHeight\":21.000"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYOriginSelector\":{\"source\":\"sourceOnlyPageYOriginCandidateAgreementGate best-supported group selector\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"selectionReady\":false,\"tableCandidateIndex\":3,\"selectionBasis\":\"single-support-source-only-y-origin-fallback\",\"singleSupportFallback\":true,\"selectedOriginBasis\":\"page-mark-absolute-y-slot-field2-tail-block16-word11\",\"selectedY\":768.000,\"selectedRowHeight\":null,\"supportCount\":1,\"supportOriginBases\":[\"page-mark-absolute-y-slot-field2-tail-block16-word11\"],\"supportTableCandidateIndexes\":[3],\"supportCoversMultipleTableCandidates\":false,\"supportFragmentedByTable\":false"
    ));
    assert!(layer_tree.contains(
        "\"supportBlockedReasons\":[\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"page-mark-absolute-y-slot-semantics-unproven\",\"single-source-y-origin-support-unproven\"],\"renderPromotionContribution\":\"source-only-page-y-origin-selector\",\"renderPromotionBlockedReason\":\"single-source-y-origin-support-unproven\""
    ));
    assert!(layer_tree.contains(
        "{\"selectedY\":768.000,\"rowHeight\":null,\"supportCount\":1,\"originBases\":[\"page-mark-absolute-y-slot-field2-tail-block16-word11\"],\"tableCandidateIndexes\":[3],\"contributions\":[\"source-only-page-mark-absolute-y-slot-y-origin\"],\"blockedReasons\":[\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"page-mark-absolute-y-slot-semantics-unproven\"]}"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRowBoundaryAlignment\":{\"source\":\"/LineMark source unit boundaries+table row source unit boundaries\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"coordinateSpace\":\"documentTextSourceUnits\",\"policy\":\"line-mark-start-end-compared-to-table-row-source-start-end\",\"candidateRowCount\":3,\"rowBoundaryOffsetCandidateFamily\":\"previous-row-span-records\",\"rowBoundaryOffsetCandidateUnits\":-82,\"rowBoundaryOffsetCandidateStable\":true,\"rowBoundaryOffsetCandidateRequiresTransform\":true"
    ));
    assert!(layer_tree.contains("\"previousRowSpanRecordAlignmentOffsetNormalizedExact\":true"));
    assert!(layer_tree.contains(
        "\"selectedSpacingRecordAlignment\":{\"family\":\"selected-spacing-records\",\"spanInterpretation\":\"selected-record-overlaps-row-and-matches-post-row-gap-span\",\"rowCount\":3,\"lineMarkRecordIndexes\":[33,35,37],\"uniformLineMarkRecordStride\":true,\"lineMarkRecordStride\":2,\"matchesStrideCandidateRecordIndexes\":false,\"rowSourceStartUnits\":[1857,2026,2170],\"rowSourceEndUnits\":[1968,2118,2261],\"lineMarkStartUnits\":[1886,2036,2179],\"lineMarkEndUnits\":[1944,2088,2233],\"startResidualUnits\":[29,10,9],\"endResidualUnits\":[-24,-30,-28],\"spanResidualUnits\":[-53,-40,-37],\"exactBoundaryMatchCount\":0,\"exactBoundaryAligned\":false"
    ));
    assert!(layer_tree.contains(
        "\"previousRowSpanRecordAlignment\":{\"family\":\"previous-row-span-records\",\"spanInterpretation\":\"previous-record-span-equals-compact-row-span\",\"rowCount\":3,\"lineMarkRecordIndexes\":[32,34,36],\"uniformLineMarkRecordStride\":true,\"lineMarkRecordStride\":2,\"matchesStrideCandidateRecordIndexes\":true,\"rowSourceStartUnits\":[1857,2026,2170],\"rowSourceEndUnits\":[1968,2118,2261],\"lineMarkStartUnits\":[1775,1944,2088],\"lineMarkEndUnits\":[1886,2036,2179],\"startResidualUnits\":[-82,-82,-82],\"endResidualUnits\":[-82,-82,-82],\"spanResidualUnits\":[0,0,0],\"exactBoundaryMatchCount\":0,\"exactBoundaryAligned\":false,\"startResidualStable\":true,\"endResidualStable\":true,\"spanResidualStable\":true,\"stableStartResidualUnits\":-82,\"stableEndResidualUnits\":-82,\"stableSpanResidualUnits\":0,\"rowBoundaryOffsetCandidateUnits\":-82,\"offsetNormalizationPolicy\":\"line-mark-boundary-minus-row-source-boundary-minus-stable-offset\",\"offsetNormalizedStartResidualUnits\":[0,0,0],\"offsetNormalizedEndResidualUnits\":[0,0,0],\"offsetNormalizedExactBoundaryMatchCount\":3,\"offsetNormalizedExactBoundaryAligned\":true,\"spanOnlyMatch\":true"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"source-only-line-mark-row-boundary-alignment\",\"renderPromotionBlockedReason\":\"line-mark-row-boundaries-require-source-offset-transform\""
    ));
    assert!(layer_tree.contains(
        "\"pageMarkEntryLineBoundsCoverage\":{\"source\":\"/LineMark record indexes+/PageMark entry line bounds\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"sourceDomain\":\"line-mark-record-index\",\"pageMarkDomain\":\"page-mark-line-index\",\"candidateRowCount\":3,\"lineMarkRecordIndexes\":[32,34,36],\"recordStride\":2,\"pageMarkEntryIndex\":0,\"pageIndexCandidate\":0,\"pageLineStart\":0,\"pageLineEnd\":42,\"lineOffsetsFromPageStart\":[32,34,36],\"rowCountMatchesStrideCandidate\":true,\"allLineMarkRecordsWithinPageMarkEntry\":true,\"coverageReady\":true,\"sourceRangeCoverageEvaluated\":false,\"sourceRangeCoverageSkippedReason\":\"document-text-unit-ranges-are-not-page-mark-line-indexes\",\"pageYTransformDecoded\":false,\"renderPromotionContribution\":\"source-only-stride-row-page-mark-entry-coverage\",\"renderPromotionBlockedReason\":null}"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkSubrecordLineRangeRecordCoverage\":{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark record indexes\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"policy\":\"subrecord-line-start-end-must-contain-line-mark-record-index\",\"candidateCount\":7,\"selectedSpacingRecordIndexes\":[33,35,37],\"previousRowSpanRecordIndexes\":[32,34,36],\"selectedCoveredRecordIndexes\":[],\"previousCoveredRecordIndexes\":[],\"selectedContainingCandidateByteOffsets\":[],\"previousContainingCandidateByteOffsets\":[],\"selectedCoverageComplete\":false,\"previousCoverageComplete\":false"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionContribution\":\"page-mark-subrecord-line-range-record-coverage\",\"renderPromotionBlockedReason\":\"page-mark-subrecord-line-ranges-do-not-cover-line-mark-records\""
    ));
    assert!(layer_tree.contains(
        "\"rawRecordSourceRangeCoverageDomain\":\"legacy-cross-domain-document-text-unit-range-vs-page-mark-line-index\",\"rawRecordSourceRangeCoverageUsableForPromotion\":false,\"rawRecordSourceRangeCoverage\":{\"candidateRowCount\":3,\"rowSourceCoverageCount\":0,\"allRowsHaveHeaderCoverage\":false,\"totalOverlappingHeaderCount\":0,\"matchedScanIndexes\":[],\"matchedScanIndexesMonotonic\":true},\"crossTableOrderingConsistent\":false,\"sourceOrderVsSubrecordOrderContradiction\":true"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"line-mark-page-origin-candidate-absent\",\"selected-post-row-gap-subrecord-coverage-not-ordered-unique\",\"cross-table-subrecord-ordering-inconsistent\",\"source-order-vs-subrecord-order-contradiction\",\"decoded-page-y-origin-missing\"],\"renderPromotionContribution\":\"line-mark-stride-to-page-y-readiness-gate\",\"renderPromotionBlockedReason\":\"stride-y-hypothesis-needs-source-only-validation\""
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"line-mark-page-origin-candidate-absent\",\"line-mark-record-stride-to-page-y-transform-unproven\",\"page-origin-authority-not-renderable-line-mark-page-grid\",\"line-mark-rows-not-exact-source-boundaries\",\"page-mark-subrecord-spans-fit-selected-post-row-gaps\",\"page-mark-subrecord-selected-post-row-gap-candidates-not-row-unique\",\"page-mark-subrecord-spans-do-not-decode-page-y-origin\",\"page-mark-cross-table-raw-record-order-regression\",\"page-mark-cross-table-subrecord-ordering-unproven\",\"cross-table-row-boundary-offset-transform-required\",\"decoded-line-mark-page-y-transform-missing\"],\"renderPromotionContribution\":\"source-page-y-transform-gate\",\"renderPromotionBlockedReason\":\"source-page-y-transform-not-decoded\""
    ));
    assert!(layer_tree.contains(
        "\"lineDomainPostRowGapProjectionProbe\":{\"source\":\"sourcePageYTransformGate line-domain + post-row-gap span projection\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"projectionKind\":\"line-domain-y-plus-post-row-gap-unit-as-px\",\"selectionReady\":false,\"promotionReady\":false,\"lineDomainY\":817.539,\"selectedPostRowGapSpanFirstUnits\":58,\"selectedPostRowGapSpanComplete\":true,\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":false,\"projectedY\":875.539,\"referenceTableTopY\":768.014,\"residualPx\":107.525,\"absResidualPx\":107.525,\"withinTwoPx\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyProjectionDomainGate\":{\"source\":\"sourcePageYTransformGate source-only line-domain/post-row-gap projection domain gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"sourceProjectionPresent\":true,\"lineDomainPresent\":true,\"selectedPostRowGapSpanPresent\":true,\"selectedPostRowGapSpanComplete\":true,\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":false,\"sourceUnitDomain\":\"line-mark-record-y-plus-page-mark-subrecord-gap-units\",\"lineDomainY\":817.539,\"selectedPostRowGapSpanFirstUnits\":58,\"projectedY\":875.539,\"blockedReasons\":[\"cross-domain-source-units-treated-as-px\",\"selected-spacing-records-are-post-row-gap-family\",\"selected-post-row-gap-span-not-ordered-unique\",\"page-y-origin-transform-undecoded\"],\"renderPromotionContribution\":\"source-only-line-domain-post-row-gap-projection-domain-gate\",\"renderPromotionBlockedReason\":\"line-domain-post-row-gap-projection-crosses-source-unit-domain\"}"
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageMarkAbsoluteYSlotGate\":{\"source\":\"/PageMark raw u16 subrecord scan+/LineMark source page-line projection\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false,\"projectionKind\":\"line-domain-y-plus-post-row-gap-vs-page-mark-absolute-y-slot\",\"lineDomainY\":817.539,\"selectedPostRowGapSpanFirstUnits\":58,\"lineDomainProjectedY\":875.539,\"absoluteYSlotPresent\":true,\"bestAbsoluteYSlot\":{\"source\":\"rawRecordHeaderTailU16Subrecord\",\"interpretation\":\"direct-u16-px\",\"fieldIndex\":2,\"tailBlock16WordIndex\":11"
    ));
    assert!(layer_tree.contains(
        "\"absoluteYSlotY\":768.000,\"lineDomainProjectionVsAbsoluteYSlotResidualPx\":107.539,\"lineDomainProjectionAgreesWithAbsoluteYSlot\":false,\"lineageClass\":\"page-mark-absolute-y-slot\""
    ));
    assert!(layer_tree.contains(
        "\"lineageClass\":\"page-mark-absolute-y-slot\",\"blockedReasons\":[\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"page-mark-absolute-y-slot-semantics-unproven\",\"page-y-origin-transform-undecoded\"],\"renderPromotionContribution\":\"source-only-page-mark-absolute-y-slot-gate\",\"renderPromotionBlockedReason\":\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\""
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlyPageYRenderAdmissionGate\":{\"source\":\"sourcePageYTransformGate source-only page-y render admission gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"admissionReady\":false,\"directLineMarkOriginAdmissible\":false,\"sourceLayoutCandidatePresent\":true,\"pageOriginAuthority\":\"lineMarkPageGridStrideRawRecordIndex\",\"lineMarkRowsExactAndContiguous\":false,\"lineMarkPageOriginPresent\":false,\"lineMarkPageOriginStridePresent\":true,\"crossTableLineDomainPresent\":true"
    ));
    assert!(layer_tree.contains(
        "\"selectedPostRowGapSpanComplete\":true,\"selectedPostRowGapSpanOrderedUniqueCoverageComplete\":false,\"previousRowSpanComplete\":false,\"previousRowSpanOrderedUniqueCoverageComplete\":false,\"compactRowSpanComplete\":false,\"sourceOnlySelectorPresent\":true,\"sourceOnlySelectorSingleSupportFallback\":true,\"sourceOnlySelectorSupportCount\":1,\"sourceOnlySelectorSupportFragmentedByTable\":false,\"sourceOnlySelectorBlockedReason\":\"source-y-origin-selector-single-support-fallback-not-render-admissible\""
    ));
    assert!(layer_tree.contains(
        "\"sourceOnlySelectorSupportBlockedReasons\":[\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"page-mark-absolute-y-slot-semantics-unproven\"],\"sourceGapToPageLineGapTransformAdmissionGate\":{\"source\":\"sourceOnlyPageYRenderAdmissionGate.sourceGapToPageLineGapTransformAdmissionGate\""
    ));
    assert!(layer_tree.contains(
        "\"pageMarkAbsoluteYSlotBlockedReason\":\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"pageMarkAbsoluteYSlotResidualPx\":107.539,\"blockedReasons\":[\"direct-line-mark-page-origin-absent\",\"line-mark-record-stride-to-page-y-transform-unproven\",\"page-origin-authority-not-renderable-line-mark-page-grid\",\"line-mark-rows-not-exact-source-boundaries\",\"cross-table-line-domain-not-page-space-origin\",\"selected-post-row-gap-spans-not-page-y-origin\",\"selected-post-row-gap-coverage-not-row-unique\",\"source-order-vs-subrecord-order-contradiction\",\"cross-table-row-boundary-offset-transform-required\",\"line-domain-projection-disagrees-with-page-mark-absolute-y-slot\",\"source-y-origin-selector-single-support-fallback-not-render-admissible\",\"source-gap-to-page-line-gap-transform-unstable-across-table-family\",\"decoded-line-mark-page-y-transform-missing\"]"
    ));
    assert!(layer_tree.contains(
        "\"renderPromoted\":false,\"renderPromotionAuthority\":null,\"renderPromotionBlockedReason\":\"line-mark-record-stride-to-page-y-transform-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"matchedCellHeaderCount\":6,\"postRowGapCorrelationComplete\":true,\"postRowGapMatchCount\":3,\"postRowGapExactSpanMatchCount\":3,\"rawPageMarkScanHeaderCount\":15,\"rawPageMarkSingleHeaderMatched\":true"
    ));
    assert!(layer_tree.contains(
        "\"subrecordLineSpanReadiness\":{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"spanToleranceUnits\":3,\"selectedSpacingRecordIndexes\":[33,35,37],\"previousRowSpanRecordIndexes\":[32,34,36],\"selectedPostRowGapSpanTargets\":[58,52,54],\"postRowGapSpanTargets\":[58,52,54],\"previousRowSpanTargets\":[111,92,91],\"compactRowSpanTargets\":[111,92,91],\"candidateCount\":7,\"selectedPostRowGapSpanHitCount\":3,\"previousRowSpanHitCount\":0,\"compactRowSpanHitCount\":0,\"selectedPostRowGapSpanComplete\":true,\"previousRowSpanComplete\":false,\"compactRowSpanComplete\":false,\"selectedPostRowGapSpanMaxAbsResidualUnits\":3,\"previousRowSpanMaxAbsResidualUnits\":49,\"compactRowSpanMaxAbsResidualUnits\":49,\"subrecordSpanRoleGate\":"
    ));
    assert!(layer_tree.contains(
        "\"referenceValidationThresholdPx\":8.000,\"rawRecordIndexReferenceFit\":false,\"rawRecordIndexMaxAbsResidualPx\":24.014"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"line-mark-spans-post-row-gaps-not-visible-row-heights\",\"page-mark-subrecord-spans-fit-selected-post-row-gaps\",\"page-mark-subrecord-spans-do-not-decode-page-y-origin\",\"raw-record-index-y-fails-current-reference-table\",\"decoded-line-mark-stride-to-page-y-transform-missing\"]"
    ));
    assert!(layer_tree.contains(
        "\"candidateRowCount\":3,\"rowLineMarkMatchCount\":3,\"rowScannedRecordHeaderMatchCount\":3,\"allRowsHaveLineMark\":true,\"allRowsHaveScannedRecordHeader\":true,\"singleScannedRecordHeaderMatched\":true,\"matchedScannedRecordHeaderIndex\":0,\"lineMarkRecordIndexes\":[33,35,37]"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkRawRecordSourceRangeEvidence\":{\"source\":\"/PageMark raw record headers+table source unit ranges\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"recordHeaderCount\":15,\"candidateRowCount\":3,\"rowSourceCoverageCount\":0,\"allRowsHaveHeaderCoverage\":false,\"totalOverlappingHeaderCount\":0,\"matchedScanIndexes\":[],\"matchedScanIndexesMonotonic\":true"
    ));
    assert!(layer_tree.contains(
        "\"row\":0,\"sourceUnitRange\":{\"start\":1857,\"end\":1968},\"overlappingHeaderCount\":0,\"overlappingHeaders\":[]"
    ));
    assert!(layer_tree.contains(
        "\"tableCandidateIndex\":3,\"sourceRange\":{\"start\":1857,\"end\":2261},\"rowCount\":3,\"matchedRowCount\":3,\"matchedByteOffsets\":[174,734,174],\"rawRecordScanIndexes\":[2,6,2]"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkRawReferenceValueProbe\":{\"source\":\"/PageMark raw numeric scan+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tolerancePx\":2.000,\"referenceBBox\":{\"x\":174.000,\"y\":768.014,\"width\":554.001,\"height\":111.902},\"rowTopTargets\":[{\"row\":0,\"targetPx\":768.014,\"roundedTarget\":768,\"hitCount\":2,\"hits\":[{\"kind\":\"u16be\",\"byteOffset\":178,\"valueIndex\":89,\"value\":768,\"hex\":\"0x0300\",\"residualPx\":-0.014,\"recordContext\":{\"source\":\"/PageMark raw record scan\",\"scanIndex\":2,\"recordByteOffset\":140,\"recordNextByteOffset\":252,\"recordIndex\":2,\"recordLineStart\":85,\"recordLineEnd\":85,\"recordRelativeByteOffset\":38,\"recordTailRelativeByteOffset\":22,\"recordTailWordIndex\":11,\"recordTailBlock16Index\":0,\"recordTailBlock16WordIndex\":11"
    ));
    assert!(layer_tree.contains(
        "\"enclosingSubrecord\":{\"source\":\"/PageMark raw u16 subrecord scan\",\"byteOffset\":174,\"fieldIndex\":2,\"fieldRole\":\"unknown-u16-field-2\",\"words\":[2,5,768,0,85,0,140,0],\"wordsHex\":[\"0x0002\",\"0x0005\",\"0x0300\",\"0x0000\",\"0x0055\",\"0x0000\",\"0x008c\",\"0x0000\"],\"u32Fields\":[131077,50331648,5570560,9175040],\"u32FieldsHex\":[\"0x00020005\",\"0x03000000\",\"0x00550000\",\"0x008c0000\"],\"decoded\":false,\"geometryDecoded\":false}"
    ));
    assert!(layer_tree.contains(
        "\"contextU16BE\":{\"source\":\"/PageMark raw u16 window\",\"wordWindowStartByteOffset\":166,\"wordWindowCenterByteOffset\":178,\"words\":[526,0,0,0,2,5,768,0,85,0,140,0,0],\"wordsHex\":[\"0x020e\",\"0x0000\",\"0x0000\",\"0x0000\",\"0x0002\",\"0x0005\",\"0x0300\",\"0x0000\",\"0x0055\",\"0x0000\",\"0x008c\",\"0x0000\",\"0x0000\"]}"
    ));
    assert!(layer_tree.contains(
        "\"byteOffset\":338,\"valueIndex\":169,\"value\":768,\"hex\":\"0x0300\",\"residualPx\":-0.014,\"recordContext\":{\"source\":\"/PageMark raw record scan\",\"scanIndex\":3,\"recordByteOffset\":252,\"recordNextByteOffset\":492,\"recordIndex\":3,\"recordLineStart\":141,\"recordLineEnd\":191,\"recordRelativeByteOffset\":86,\"recordTailRelativeByteOffset\":70,\"recordTailWordIndex\":35,\"recordTailBlock16Index\":2,\"recordTailBlock16WordIndex\":3"
    ));
    assert!(layer_tree.contains(
        "\"enclosingSubrecord\":{\"source\":\"/PageMark raw u16 subrecord scan\",\"byteOffset\":334,\"fieldIndex\":2,\"fieldRole\":\"unknown-u16-field-2\",\"words\":[4,1,768,0,192,0,241,0],\"wordsHex\":[\"0x0004\",\"0x0001\",\"0x0300\",\"0x0000\",\"0x00c0\",\"0x0000\",\"0x00f1\",\"0x0000\"],\"u32Fields\":[262145,50331648,12582912,15794176],\"u32FieldsHex\":[\"0x00040001\",\"0x03000000\",\"0x00c00000\",\"0x00f10000\"],\"decoded\":false,\"geometryDecoded\":false}"
    ));
    assert!(layer_tree.contains(
        "\"rowTopTargetCount\":3,\"rowTopTargetHitCount\":1,\"allRowTopTargetsHit\":false,\"totalHitCount\":2,\"rawHitRecordContextSummary\":{\"source\":\"/PageMark raw numeric scan context\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"hitContextCount\":2,\"distinctRecordIndexes\":[2,3],\"allHitsInSingleRecordHeader\":false,\"distinctTailBlock16WordIndexes\":[3,11],\"allHitsShareTailBlock16WordIndex\":false"
    ));
    assert!(layer_tree.contains(
        "\"pageMarkScopedYTransformProbe\":{\"source\":\"/PageMark scoped raw fields+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"tolerancePx\":2.000,\"lineMarkRecordIndexes\":[33,35,37],\"parsedEntryMatchCount\":3,\"singleParsedPageMarkEntryMatched\":true,\"matchedParsedPageMarkEntryIndex\":0,\"rawHeaderMatchCount\":3,\"singleRawRecordHeaderMatched\":true,\"matchedRawRecordHeaderIndex\":0"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[33,35,37],\"parsedEntryMatchCount\":3,\"singleParsedPageMarkEntryMatched\":true,\"matchedParsedPageMarkEntryIndex\":0,\"rawHeaderMatchCount\":3,\"singleRawRecordHeaderMatched\":true,\"matchedRawRecordHeaderIndex\":0,\"referenceBBoxUsed\":true,\"referenceTargetBasis\":\"referenceTableBBox.rowTopTargets\",\"sourceOnlyReplacementBlockedReason\":\"page-mark-scoped-y-transform-targets-reference-backed\""
    ));
    assert!(layer_tree.contains(
        "\"targetPx\":768.014,\"nearestCandidate\":{\"source\":\"parsedEntryU16\",\"interpretation\":\"direct-u16-px\",\"wordIndex\":83,\"byteOffset\":166,\"value\":768,\"valuePx\":768.000,\"residualPx\":-0.014}"
    ));
    assert!(
        layer_tree.contains(
            "\"rowTopHitSummary\":{\"targetCount\":3,\"targetHitCount\":2,\"hitCount\":3"
        )
    );
    assert!(layer_tree.contains(
        "\"bestTableTopFamily\":{\"source\":\"rawRecordHeaderTailU16Subrecord\",\"interpretation\":\"direct-u16-px\",\"familyKind\":\"u16-subrecord-field\",\"fieldIndex\":2,\"memberCount\":12,\"rawRecordIndexes\":[2,3,8,14],\"rawRecordScanIndexes\":[2,3,6,13]"
    ));
    assert!(layer_tree.contains(
        "\"tableTopHitRawRecordIndexes\":[2,3],\"tableTopHitByteOffsets\":[178,338],\"rowLineRangeCoverageCount\":0,\"tableTopHitLineRangeCoverageCount\":0,\"tableTopResidualsPx\":[-0.014,-0.014],\"tableTopHitCount\":2,\"tableTopMeanAbsResidualPx\":0.014,\"tableTopMaxAbsResidualPx\":0.014,\"rowTopResidualsPx\":[-0.014,-37.314,-74.615],\"rowTopCoverageCount\":1"
    ));
    assert!(layer_tree.contains(
        "\"sampleMembers\":[{\"wordIndex\":89,\"byteOffset\":178,\"rawRecordIndex\":2,\"rawRecordScanIndex\":2,\"tailBlock16WordIndex\":11,\"subrecordLineStartCandidate\":85,\"subrecordLineEndCandidate\":140,\"value\":768,\"valuePx\":768.000}"
    ));
    assert!(layer_tree.contains(
        "\"bestTableTopSlot\":{\"source\":\"rawRecordHeaderTailU16Subrecord\",\"interpretation\":\"direct-u16-px\",\"grouping\":\"fieldIndex+tailBlock16WordIndex\",\"fieldIndex\":2,\"tailBlock16WordIndex\":11,\"memberCount\":3,\"rawRecordIndexes\":[2,3,14],\"rawRecordScanIndexes\":[2,3,13],\"byteOffsets\":[178,418,1074],\"rowLineRangeCoverageCount\":0,\"tableTopResidualsPx\":[-0.014],\"tableTopHitCount\":1,\"rowTopResidualsPx\":[-0.014,-37.314,-74.615],\"rowTopCoverageCount\":1"
    ));
    assert!(layer_tree.contains(
        "\"sameHeaderSlotCount\":0,\"sameHeaderBestTableTopSlot\":null,\"sameHeaderBestRowTopSlot\":null,\"sameHeaderBestRowDeltaSlot\":null,\"foreignHeaderSlotCount\":112,\"foreignHeaderBestTableTopSlot\":{\"source\":\"rawRecordHeaderTailU16Subrecord\",\"interpretation\":\"direct-u16-px\",\"grouping\":\"fieldIndex+tailBlock16WordIndex\",\"fieldIndex\":2,\"tailBlock16WordIndex\":11"
    ));
    assert!(layer_tree.contains(
        "\"previousRowSpanScopedYTransformProbe\":{\"source\":\"/PageMark scoped raw fields+alternateLineMarkRecordSet+referenceTableBBox\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"recordSet\":\"previous-row-span-line-mark-records\",\"tolerancePx\":2.000,\"lineMarkRecordIndexes\":[32,34,36],\"parsedEntryMatchCount\":3,\"singleParsedPageMarkEntryMatched\":true,\"matchedParsedPageMarkEntryIndex\":0,\"rawHeaderMatchCount\":3,\"singleRawRecordHeaderMatched\":true,\"matchedRawRecordHeaderIndex\":0"
    ));
    assert!(layer_tree.contains(
        "\"rowTopHitSummary\":{\"targetCount\":3,\"targetHitCount\":2,\"hitCount\":3,\"hits\":[{\"targetIndex\":0,\"targetPx\":768.014,\"candidate\":{\"source\":\"parsedEntryU16\",\"interpretation\":\"direct-u16-px\",\"wordIndex\":83,\"byteOffset\":166,\"value\":768,\"valuePx\":768.000,\"residualPx\":-0.014}"
    ));
    assert!(layer_tree.contains(
        "\"targetIndex\":1,\"targetPx\":805.314,\"candidate\":{\"source\":\"parsedEntryU16\",\"interpretation\":\"centipoint-to-css-px\",\"wordIndex\":132,\"byteOffset\":264,\"value\":60312,\"valuePx\":804.160,\"residualPx\":-1.154}"
    ));
    assert!(layer_tree.contains(
        "\"matchedRawRecordHeaderIndex\":0,\"lineMarkRecordIndexes\":[32,34,36],\"referenceRowTops\":[768.014,805.314,842.615],\"referenceRowDeltas\":[37.301,37.301],\"subrecordLineRangeCandidatePolicy\":\"words4To6WithinScannedRecordHeaderMaxLineEnd\",\"subrecordLineRangeMaxCandidate\":709"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRecordIndexes\":[32,34,36],\"referenceRowTops\":[768.014,805.314,842.615],\"referenceRowDeltas\":[37.301,37.301],\"subrecordLineRangeCandidatePolicy\":\"words4To6WithinScannedRecordHeaderMaxLineEnd\",\"subrecordLineRangeMaxCandidate\":709,\"pageScaleCandidates\":{\"source\":\"/PageMark selected fields+layout\",\"pageWidthPx\":793.701,\"pageHeightPx\":1122.520,\"pageHeightPxPerWord21Unit\":1.990,\"pageHeightPxPerWord13Plus14Unit\":1.481,\"word21\":564,\"word13Plus14\":758},\"slotCount\":112,\"sameHeaderSlotCount\":0,\"sameHeaderBestTableTopSlot\":null,\"sameHeaderBestRowTopSlot\":null,\"sameHeaderBestRowDeltaSlot\":null,\"foreignHeaderSlotCount\":112,\"foreignHeaderBestTableTopSlot\":{\"source\":\"rawRecordHeaderTailU16Subrecord\",\"interpretation\":\"direct-u16-px\",\"grouping\":\"fieldIndex+tailBlock16WordIndex\",\"fieldIndex\":2,\"tailBlock16WordIndex\":11"
    ));
    assert!(layer_tree.contains(
        "\"absoluteVsSpanLineageGate\":{\"source\":\"/PageMark y-candidate lineage: reference table-top vs source line-span\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":true,\"selectionReady\":false,\"promotionReady\":false,\"lineageClassification\":\"reference-absolute-table-top-vs-source-line-span-correlation\",\"absoluteTableTopProbe\":{\"source\":\"referenceTableBBox.rowTopTargets\",\"referenceBacked\":true,\"sourceBacked\":false,\"referenceTableTopY\":768.014"
    ));
    assert!(layer_tree.contains(
        "\"sourceSpanProbe\":{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\",\"referenceBacked\":false,\"sourceBacked\":true,\"spanEvidencePositional\":false,\"present\":true,\"selectedSpanRole\":\"post-row-gap\",\"previousSpanRole\":\"compact-row-span\",\"selectedPostRowGapSpanTargets\":[58,52,54],\"selectedPostRowGapSpanHitCount\":3,\"selectedPostRowGapSpanComplete\":true,\"previousRowSpanTargets\":[111,92,91]"
    ));
    assert!(layer_tree.contains(
        "\"blockedReasons\":[\"absolute-table-top-targets-reference-backed\",\"source-subrecord-spans-are-line-span-targets\",\"selected-post-row-gap-spans-do-not-decode-y-origin\",\"source-only-absolute-table-top-field-unproven\"],\"renderPromotionContribution\":\"page-mark-y-candidate-lineage-gate\",\"renderPromotionBlockedReason\":\"absolute-top-evidence-reference-backed-span-evidence-non-positional\""
    ));
    assert!(layer_tree.contains(
        "\"subrecordLineSpanCorrelation\":{\"source\":\"/PageMark raw u16 subrecord line ranges+/LineMark+sparseTableSiblingEvidence\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"spanToleranceUnits\":3,\"selectedSpacingRecordIndexes\":[33,35,37],\"previousRowSpanRecordIndexes\":[32,34,36],\"selectedPostRowGapSpanTargets\":[58,52,54],\"postRowGapSpanTargets\":[58,52,54],\"previousRowSpanTargets\":[111,92,91],\"compactRowSpanTargets\":[111,92,91]"
    ));
    assert!(layer_tree.contains(
        "\"rowDeltaCoverageCount\":0,\"rowDeltaMeanAbsResidualPx\":37.301,\"rowDeltaMaxAbsResidualPx\":37.301"
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"line-mark-record-stride-to-page-y-transform-unproven\""
    ));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"stride-y-hypothesis-needs-cross-table-validation\""
    ));
    assert!(layer_tree.contains(
        "\"pageMarkU16GeometryHypotheses\":{\"source\":\"/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"profile\":\"mixed-payload\""
    ));
    assert!(layer_tree.contains(
        "\"selectedFields\":[{\"wordIndex\":10,\"value\":564,\"hex\":\"0x0234\"},{\"wordIndex\":13,\"value\":564,\"hex\":\"0x0234\"},{\"wordIndex\":14,\"value\":194,\"hex\":\"0x00c2\""
    ));
    assert!(layer_tree.contains(
        "\"sourceDerivedLayoutCandidate\":{\"source\":\"documentTextLineHeaders+/LineMark+/PageMark\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"provenance\":\"decodedCompactPlacement\",\"projectionKind\":\"sourceDerivedDiagnosticProjection\",\"bbox\":{\"x\":72.000,\"y\":744.000,\"width\":451.181,\"height\":63.000}"
    ));
    assert!(layer_tree.contains(
        "\"horizontalUnitTransformReadiness\":{\"source\":\"documentTextLineHeaders\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"selectedXUnitRangeBasis\":\"matched-cells\",\"selectedXUnitRange\":{\"start\":0,\"end\":100},\"selectedWidthUnits\":100,\"fullExtentUnits\":144,\"selectedWidthRatioToFullExtent\":0.694"
    ));
    assert!(layer_tree.contains(
        "\"rowAgreementCount\":3,\"allRowsAgree\":true,\"trailingHeaderIncluded\":false,\"includedTrailingHeaderCount\":0,\"columnSpanUnits\":[16,80],\"columnSlotWidthUnits\":[20,80],\"trailingSlotWidthUnits\":[],\"xOriginInsetUnits\":0.000,\"xOriginInsetBasis\":\"none\""
    ));
    assert!(layer_tree.contains(
        "\"totalWidthSemanticsGate\":{\"source\":\"documentTextLineHeaders total-width semantics gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"selectedWidthUnits\":100,\"fullExtentUnits\":144,\"fullExtentTrailingUnits\":44,\"selectedEqualsFullExtent\":false,\"selectedIsSubsetOfFullExtent\":true,\"trailingHeaderIncluded\":false,\"includedTrailingHeaderCount\":0,\"trailingSlotEvidencePresent\":false,\"trailingSlotWidthUnits\":[],\"selectedVisibleRangeSourceEvidenceReady\":false,\"sourcePlacementCoherenceGateRequired\":false,\"sourcePlacementCoherenceGateEvidencePresent\":false,\"sourcePlacementCoherenceGateResolved\":false,\"sourcePlacementCoherenceGateBlockedReasons\":[],\"renderPromotionNextGate\":\"source-total-width-semantics-decoder\",\"renderWidthBasisCandidate\":\"selected-visible-range-subset-of-full-extent\",\"renderPromotionContribution\":\"source-total-width-semantics-gate\",\"renderPromotionBlockedReason\":\"source-total-width-semantics-unproven\"}"
    ));
    assert!(
        layer_tree
            .contains("\"sourceOnlyUnitTransformReady\":true,\"pageSpaceUnitScaleDecoded\":false")
    );
    assert!(layer_tree.contains(
        "\"pageOriginAuthority\":\"lineMarkPageGridStrideRawRecordIndex\",\"anchorLineIndex\":23,\"lineMarkPageOriginCandidate\":null"
    ));
    assert!(layer_tree.contains(
        "\"lineMarkRowRecordSelection\":\"previous-compact-row-span-record\",\"lineMarkRowsExactAndContiguous\":false"
    ));
    assert!(layer_tree.contains(
        "\"sourceEvidence\":{\"source\":\"tableCellProvenance\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"rowSourceIntervalIndex\":20"
    ));
    assert!(layer_tree.contains(
        "\"lineHeaderCandidate\":{\"source\":\"/DocumentText line-header\",\"selection\":\"nearest-preceding-line-header\",\"sourceUnitRange\":{\"start\":1902,\"end\":1914},\"offsetUnits\":20,\"extentUnits\":100,\"fontSizeUnits\":12"
    ));
    assert!(layer_tree.contains("\"matchedCellSpanUnits\":[16,80]"));
    assert!(layer_tree.contains(
        "\"renderPromotionBlockedReason\":\"line-mark-record-stride-to-page-y-transform-unproven\""
    ));
    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("data-projection=\"tsaitenReferenceProjection\""));
    assert!(svg.contains("data-role=\"document-heading\""));
    assert!(svg.contains("data-role=\"title-box\""));
    assert!(svg.contains("data-role=\"document-format-table\""));
    assert!(svg.contains("＜採点原則＞"));
    assert!(svg.contains("class=\"rjtd-column-grid-candidate\""));
    assert!(svg.contains("data-projection-kind=\"tableProjection\""));
    assert!(svg.contains("data-reference-backed=\"true\""));
    assert_eq!(
        svg.matches("data-reference-fallback-used=\"true\"").count(),
        2
    );
    assert!(svg.contains("data-reference-fallback-admitted=\"true\""));
    assert!(svg.contains("data-reference-fallback-blocked-reason=\"none\""));
    assert!(svg.contains("data-source-derived-layout-candidate=\"true\""));
    assert!(svg.contains("data-source-layout-evidence-present=\"true\""));
    assert!(svg.contains(
        "data-render-promotion-blocked-reason=\"line-mark-record-stride-to-page-y-transform-unproven\""
    ));
    assert!(svg.contains("data-row-source-interval-index=\"20\""));
    assert!(svg.contains("data-line-mark-record-index=\"33\""));
    assert!(svg.contains("data-page-mark-entry-index=\"0\""));
    assert!(svg.contains("data-line-header-offset-units=\"20\""));
    assert!(!svg.contains("data-projection-kind=\"sourceDerivedDiagnosticProjection\""));
    assert!(svg.contains("data-col-count-candidate=\"3\""));
    assert!(svg.contains("235点以上"));
    assert_eq!(svg.matches(">235点以上</text>").count(), 1);
}

#[test]
fn document_core_exposes_row_like_table_candidate_read_api() {
    let position_table = text_count_table_fixture_with_ranges(&[(0, 30)]);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_with_control_boundary()),
        (
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            &position_table,
        ),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    assert_eq!(
        core.get_table_dimensions(0, 0, 0).unwrap(),
        "{\"rowCount\":2,\"colCount\":1,\"cellCount\":2,\"source\":\"tableCandidate\",\"tableCandidateIndex\":0,\"basis\":\"byte\",\"delimiterCode\":28,\"delimiterCodeHex\":\"0x001c\",\"columnSplitCandidateRows\":0,\"maxColumnSegmentCount\":0,\"columnSegmentPatternConsistent\":false,\"columnSegmentPatternMismatchRows\":0,\"columnGridCandidate\":null,\"columnSplittingDecoded\":false,\"decoded\":false}"
    );
    assert_eq!(
        core.get_cell_info(0, 0, 0, 1).unwrap(),
        "{\"row\":1,\"col\":0,\"rowSpan\":1,\"colSpan\":1,\"source\":\"tableCandidateInterval\",\"sourceIntervalIndex\":1,\"sourceStart\":16,\"sourceEnd\":22,\"decoded\":false}"
    );
    assert_eq!(core.get_cell_paragraph_count(0, 0, 0, 0).unwrap(), 1);
    assert_eq!(core.get_cell_paragraph_count(0, 0, 0, 9).unwrap(), 0);
    assert_eq!(core.get_cell_paragraph_length(0, 0, 0, 1, 0).unwrap(), 2);
    assert_eq!(core.get_cell_paragraph_length(0, 0, 0, 1, 1).unwrap(), 0);
    assert_eq!(core.get_text_in_cell(0, 0, 0, 1, 0, 0, 10).unwrap(), "鉄道");
    assert_eq!(core.get_text_in_cell(0, 0, 0, 1, 0, 1, 1).unwrap(), "道");
    assert_eq!(
        core.get_line_info_in_cell(0, 0, 0, 1, 0, 0).unwrap(),
        "{\"lineIndex\":0,\"lineCount\":1,\"charStart\":0,\"charEnd\":2}"
    );
    assert_eq!(
        core.get_table_signature(0, 0, 0).unwrap(),
        "rjtd-table-candidate:0:byte:0x001c:2x1"
    );
}

#[test]
fn table_row_column_segments_split_finance_numeric_runs() {
    let segments = table_row_column_segments("　　売掛金2,441,9973,983,602△1,541,6042,766,830");

    assert_eq!(segments.len(), 5);
    assert_eq!(segments[0].kind(), TableCandidateColumnSegmentKind::Label);
    assert_eq!(segments[0].text(), "売掛金");
    assert_eq!(segments[1].kind(), TableCandidateColumnSegmentKind::Value);
    assert_eq!(segments[1].text(), "2,441,997");
    assert_eq!(segments[2].text(), "3,983,602");
    assert_eq!(segments[3].text(), "△1,541,604");
    assert_eq!(segments[4].text(), "2,766,830");

    let total_segments = table_row_column_segments(
        "      投資その他の資産合計4,249,16115.54,988,33217.2△  739,1706,241,65318.9",
    );
    assert_eq!(total_segments[0].text(), "投資その他の資産合計");
    assert_eq!(total_segments[1].text(), "4,249,161");
    assert_eq!(total_segments[2].text(), "15.5");
    assert_eq!(total_segments[3].text(), "4,988,332");
    assert_eq!(total_segments[4].text(), "17.2");
    assert_eq!(total_segments[5].text(), "△  739,170");
}

#[test]
fn table_candidate_reports_column_segment_pattern_mismatches() {
    let intervals = vec![
        TableCandidateInterval::new(
            0,
            0,
            0,
            50,
            "     (1)投資有価証券1,033,242996,74536,4961,353,292".to_string(),
        ),
        TableCandidateInterval::new(
            1,
            1,
            51,
            100,
            "     (2)投資不動産1,939,4812,176,479△  236,9972,973,984".to_string(),
        ),
        TableCandidateInterval::new(
            2,
            2,
            101,
            165,
            "      投資その他の資産合計4,249,16115.54,988,33217.2△  739,1706,241,65318.9"
                .to_string(),
        ),
    ];
    let candidate = TableCandidate {
        index: 0,
        text_boundary_candidate_index: 0,
        text_count_range_index: 0,
        basis: TextCountRangeOverlapBasis::Unit,
        delimiter_code: 0x000e,
        interval_count: intervals.len(),
        first_interval_index: 0,
        last_interval_index: intervals.len() - 1,
        source_start: 0,
        source_end: 165,
        intervals,
    };

    assert_eq!(candidate.column_split_candidate_row_count(), 3);
    assert_eq!(candidate.max_column_segment_count(), 8);
    assert!(!candidate.column_segment_pattern_consistent());
    assert_eq!(candidate.column_segment_pattern_mismatch_rows(), 1);
    assert_eq!(candidate.column_segment_grid_candidate(), None);
}

#[test]
fn table_candidate_reports_column_segment_grid_candidate_for_consistent_rows() {
    let intervals = vec![
        TableCandidateInterval::new(
            0,
            0,
            0,
            50,
            "　　売掛金2,441,9973,983,602△1,541,6042,766,830".to_string(),
        ),
        TableCandidateInterval::new(
            1,
            1,
            51,
            100,
            "　　買掛金1,111,1112,222,222△3,333,3334,444,444".to_string(),
        ),
    ];
    let candidate = TableCandidate {
        index: 0,
        text_boundary_candidate_index: 0,
        text_count_range_index: 0,
        basis: TextCountRangeOverlapBasis::Unit,
        delimiter_code: 0x000e,
        interval_count: intervals.len(),
        first_interval_index: 0,
        last_interval_index: intervals.len() - 1,
        source_start: 0,
        source_end: 100,
        intervals,
    };

    let grid = candidate.column_segment_grid_candidate().unwrap();
    assert_eq!(grid.row_count(), 2);
    assert_eq!(grid.column_count(), 5);
    assert_eq!(grid.cell_count(), 10);
    assert_eq!(grid.split_row_count(), 2);
    assert_eq!(
        grid.pattern(),
        &[
            TableCandidateColumnSegmentKind::Label,
            TableCandidateColumnSegmentKind::Value,
            TableCandidateColumnSegmentKind::Value,
            TableCandidateColumnSegmentKind::Value,
            TableCandidateColumnSegmentKind::Value
        ]
    );

    let json = observed_table_dimensions_json(&candidate);
    assert!(json.contains("\"colCount\":1"));
    assert!(json.contains("\"columnGridCandidate\":{\"source\":\"columnSegments\""));
    assert!(json.contains("\"rowCount\":2"));
    assert!(json.contains("\"colCountCandidate\":5"));
    assert!(json.contains("\"cellCountCandidate\":10"));
    assert!(json.contains("\"pattern\":[\"label\",\"value\",\"value\",\"value\",\"value\"]"));
    assert!(json.contains("\"geometryDecoded\":false"));
}

#[test]
fn document_text_control_two_row_three_column_table_exposes_column_grid_candidate() {
    let payload = document_text_with_two_row_control_table();
    let map = map_document_text(&payload);
    let candidates = table_candidates_from_document_text_controls(map.entries(), 0);

    assert_eq!(candidates.len(), 1);
    let candidate = &candidates[0];
    assert_eq!(candidate.kind(), "documentTextControlRunTableCandidate");
    assert_eq!(candidate.interval_count(), 2);
    assert_eq!(candidate.cell_count_candidate(), 6);
    assert_eq!(candidate.non_empty_cell_count_candidate(), 6);

    let grid = candidate.column_segment_grid_candidate().unwrap();
    assert_eq!(grid.row_count(), 2);
    assert_eq!(grid.column_count(), 3);
    assert_eq!(grid.cell_count(), 6);
    assert_eq!(grid.split_row_count(), 2);
}

#[test]
fn document_text_control_table_accepts_configured_empty_gap_boundary() {
    let payload = document_text_with_table_row_gap(DOCUMENT_TEXT_CONTROL_TABLE_MAX_EMPTY_GAP_ROWS);
    let map = map_document_text(&payload);
    let candidates = table_candidates_from_document_text_controls(map.entries(), 0);

    assert_eq!(candidates.len(), 1);
    let candidate = &candidates[0];
    assert_eq!(candidate.kind(), "documentTextControlRunTableCandidate");
    assert_eq!(candidate.interval_count(), 6);
    assert_eq!(candidate.cell_count_candidate(), 18);
    assert_eq!(candidate.non_empty_cell_count_candidate(), 18);

    let grid = candidate.column_segment_grid_candidate().unwrap();
    assert_eq!(grid.row_count(), 6);
    assert_eq!(grid.column_count(), 3);
    assert_eq!(grid.cell_count(), 18);
}

#[test]
fn document_text_control_table_splits_gap_larger_than_boundary() {
    let payload =
        document_text_with_table_row_gap(DOCUMENT_TEXT_CONTROL_TABLE_MAX_EMPTY_GAP_ROWS + 1);
    let map = map_document_text(&payload);
    let candidates = table_candidates_from_document_text_controls(map.entries(), 0);

    assert_eq!(candidates.len(), 2);
    for candidate in &candidates {
        assert_eq!(candidate.kind(), "documentTextControlRunTableCandidate");
        assert_eq!(candidate.interval_count(), 3);
        assert_eq!(candidate.cell_count_candidate(), 9);
        assert_eq!(candidate.non_empty_cell_count_candidate(), 9);

        let grid = candidate.column_segment_grid_candidate().unwrap();
        assert_eq!(grid.row_count(), 3);
        assert_eq!(grid.column_count(), 3);
        assert_eq!(grid.cell_count(), 9);
    }
}

#[test]
fn sparse_document_text_controls_preserve_empty_cells_as_table_evidence() {
    let payload = document_text_with_sparse_table_rows();
    let map = map_document_text(&payload);
    let candidates = sparse_table_candidates_from_document_text_controls(map.entries(), 7);

    assert_eq!(candidates.len(), 1);
    let candidate = &candidates[0];
    assert_eq!(candidate.index(), 7);
    assert_eq!(
        candidate.kind(),
        "sparseDocumentTextControlRunTableCandidate"
    );
    assert!(candidate.is_sparse_document_text_control_run_candidate());
    assert_eq!(
        candidate.rule(),
        "sparse-document-text-001c-cells-with-000e-row-breaks"
    );
    assert_eq!(candidate.interval_count(), 4);
    assert!(!candidate.is_row_like());
    assert_eq!(candidate.max_column_segment_count(), 4);
    assert_eq!(candidate.cell_count_candidate(), 14);
    assert_eq!(candidate.non_empty_cell_count_candidate(), 4);
    assert_eq!(candidate.empty_cell_count_candidate(), 10);
    assert_eq!(candidate.intervals()[0].text_preview(), "\t\t(1)表面積\t");
    assert_eq!(candidate.intervals()[2].text_preview(), "\tＡＢ ＝ ｃｍ\t");
    let topology = candidate.sparse_topology_candidate().unwrap();
    assert_eq!(topology.row_count(), 4);
    assert_eq!(topology.max_column_count(), 4);
    assert_eq!(topology.cell_count(), 14);
    assert_eq!(topology.empty_cell_count(), 10);
    assert_eq!(topology.non_empty_cell_count(), 4);
    assert_eq!(topology.rows()[0].first_non_empty_column_index(), Some(2));
    assert_eq!(topology.rows()[0].last_non_empty_column_index(), Some(2));
    assert_eq!(topology.rows()[2].first_non_empty_column_index(), Some(1));
    assert_eq!(topology.columns()[0].non_empty_cell_count(), 0);
    assert_eq!(topology.columns()[1].non_empty_cell_count(), 3);
    assert_eq!(topology.columns()[2].non_empty_cell_count(), 1);
    assert_eq!(topology.columns()[3].empty_cell_count(), 2);

    let json = table_candidates_json(&candidates);
    assert!(json.contains("\"sparse\":true"));
    assert!(json.contains("\"sparseObservedTable\":{\"source\":\"sparseDocumentTextControlRows\""));
    assert!(json.contains("\"topologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\""));
    assert!(
        json.contains("\"sparseTopologyCandidate\":{\"source\":\"sparseDocumentTextControlRows\"")
    );
    assert!(json.contains("\"firstNonEmptyColumnIndex\":2"));
    assert!(json.contains("\"observedCellCount\":4"));
    assert!(json.contains("\"firstNonEmptyRowIndex\":1"));
    assert!(json.contains("\"emptyCellCountCandidate\":10"));
    assert!(json.contains("\"rows\":["));
    assert!(json.contains("\"columns\":["));
    assert!(json.contains("\"cells\":["));
    assert!(json.contains("\"empty\":true"));
    assert!(json.contains("\"columnSegments\":[{\"index\":0,\"kind\":\"label\""));
    assert!(json.contains("\"sourceStart\":"));
    assert!(json.contains("\"sourceEnd\":"));
}

#[test]
fn local_samples_produce_validation_warning_json_when_available() {
    let sample_dir = local_samples_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut sample_count = 0usize;
    let mut warning_sample_count = 0usize;
    let mut control_boundary_count = 0usize;
    let mut control_range_overlap_count = 0usize;
    let mut text_boundary_candidate_count = 0usize;
    let mut projected_control_count = 0usize;
    let mut page_control_layout_count = 0usize;
    let mut failures = Vec::new();

    let no_jtd_samples = fs::read_dir(&sample_dir).unwrap().all(|entry| {
        !entry
            .unwrap()
            .path()
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| matches!(ext, "jtd" | "jtt" | "jttc"))
    });
    if no_jtd_samples {
        return;
    }

    for entry in fs::read_dir(&sample_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if !matches!(extension, "jtd" | "jtt" | "jttc") {
            continue;
        }

        sample_count += 1;
        let bytes = fs::read(&path).unwrap();
        match DocumentCore::from_bytes(&bytes) {
            Ok(core) => {
                control_boundary_count += core.document().text_control_boundaries().len();
                control_range_overlap_count += core
                    .document()
                    .text_count_ranges()
                    .iter()
                    .map(|range| range.control_range_overlaps().len())
                    .sum::<usize>();
                text_boundary_candidate_count += core.document().text_boundary_candidates().len();
                if !core.document().text_boundary_candidates().is_empty() {
                    let info = core.get_document_info();
                    assert!(info.contains("\"textBoundaryCandidateCount\":"));
                    assert!(info.contains("\"textBoundaryCandidates\":["));
                    assert!(info.contains("\"kind\":\"controlDelimitedTextCountRange\""));
                }
                let projected_controls = projected_text_controls(core.document());
                projected_control_count += projected_controls.len();
                if !projected_controls.is_empty() {
                    for page in 0..core.page_count() {
                        let layout = core.get_page_control_layout(page).unwrap();
                        assert!(layout.starts_with("{\"controls\":["));
                        if layout.contains("\"type\":\"jtdControl\"") {
                            assert!(layout.contains("\"source\":\"textControlBoundary\""));
                            assert!(layout.contains("\"decoded\":false"));
                            page_control_layout_count += 1;
                            break;
                        }
                    }
                }
                let warnings = core.get_validation_warnings();
                assert!(warnings.starts_with("{\"count\":"));
                assert!(warnings.contains("\"summary\":{"));
                assert!(warnings.contains("\"warnings\":["));
                if !warnings.contains("\"count\":0") {
                    warning_sample_count += 1;
                }
            }
            Err(error) => failures.push(format!("{}: {error}", path.display())),
        }
    }

    assert_eq!(failures, Vec::<String>::new());
    if sample_count == 0 {
        return;
    }
    assert!(warning_sample_count > 0);
    assert!(control_boundary_count > 0);
    if control_range_overlap_count == 0 {
        return;
    }
    assert!(text_boundary_candidate_count > 0);
    assert_eq!(text_boundary_candidate_count, control_range_overlap_count);
    assert!(projected_control_count > 0);
    assert!(page_control_layout_count > 0);
}

#[test]
fn local_samples_project_column_grid_candidates_to_svg_and_layer_tree_when_available() {
    let sample_dir = local_samples_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut _sample_count = 0usize;
    let mut files_with_grid = 0usize;
    let mut grid_candidate_count = 0usize;
    let mut svg_overlay_count = 0usize;
    let mut layer_op_count = 0usize;
    let mut source_derived_layout_count = 0usize;
    let mut source_derived_svg_overlay_count = 0usize;
    let mut failures = Vec::new();

    let no_jtd_samples = fs::read_dir(&sample_dir).unwrap().all(|entry| {
        !entry
            .unwrap()
            .path()
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| matches!(ext, "jtd" | "jtt" | "jttc"))
    });
    if no_jtd_samples {
        return;
    }

    for entry in fs::read_dir(&sample_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if !matches!(extension, "jtd" | "jtt" | "jttc") {
            continue;
        }
        if !path.with_extension("pdf").exists() {
            continue;
        }

        _sample_count += 1;
        let bytes = fs::read(&path).unwrap();
        match DocumentCore::from_bytes(&bytes) {
            Ok(core) => {
                let current_grid_count = core
                    .document()
                    .table_candidates()
                    .iter()
                    .filter(|candidate| candidate.column_segment_grid_candidate().is_some())
                    .count();
                if current_grid_count == 0 {
                    continue;
                }

                files_with_grid += 1;
                grid_candidate_count += current_grid_count;
                let svg = core.render_page_svg(0).unwrap();
                let layer_tree = core.get_page_layer_tree(0).unwrap();
                svg_overlay_count += svg.matches("class=\"rjtd-column-grid-candidate\"").count();
                source_derived_svg_overlay_count += svg
                    .matches("data-projection-kind=\"sourceDerivedDiagnosticProjection\"")
                    .count();
                layer_op_count += layer_tree
                    .matches("\"type\":\"tableGridCandidate\"")
                    .count();
                source_derived_layout_count += layer_tree
                    .matches(
                        "\"sourceDerivedLayoutCandidate\":{\"source\":\"documentTextLineHeaders+fallbackTextAnchors\"",
                    )
                    .count();

                if svg.contains("class=\"rjtd-column-grid-candidate\"") {
                    assert!(
                        svg.contains("data-projection-kind=\"tableProjection\"")
                            || svg.contains(
                                "data-projection-kind=\"sourceDerivedDiagnosticProjection\""
                            )
                    );
                    assert!(svg.contains("data-source-derived-layout-candidate=\""));
                    assert!(svg.contains("data-decoded=\"false\""));
                    assert!(svg.contains("data-geometry-decoded=\"false\""));
                    assert!(svg.contains("data-col-count-candidate=\""));
                }
                if svg.contains("data-projection-kind=\"sourceDerivedDiagnosticProjection\"") {
                    assert!(svg.contains("data-reference-backed=\"false\""));
                    assert!(svg.contains("data-placement-derived-from-source=\"true\""));
                }
                if local_sample_has_capability(
                    &path,
                    LocalSampleCapability::UsesReferenceBackedColumnGridProjection,
                ) {
                    assert!(
                        !svg.contains("data-projection-kind=\"sourceDerivedDiagnosticProjection\"")
                    );
                }
                assert!(
                    layer_tree.contains("\"projectionKind\":\"diagnosticProjection\"")
                        || layer_tree.contains("\"projectionKind\":\"tableProjection\"")
                );
                assert!(layer_tree.contains("\"decoded\":false"));
                assert!(layer_tree.contains("\"geometryDecoded\":false"));
            }
            Err(error) => failures.push(format!("{}: {error}", path.display())),
        }
    }

    assert_eq!(failures, Vec::<String>::new());
    if files_with_grid == 0 {
        return;
    }
    assert!(svg_overlay_count <= grid_candidate_count);
    assert_eq!(layer_op_count, grid_candidate_count);
    if source_derived_layout_count == 0 {
        return;
    }
    assert!(source_derived_svg_overlay_count <= source_derived_layout_count);
}

#[test]
#[cfg(feature = "bitmap-images")]
fn local_samples_project_image_payload_diagnostics_when_available() {
    let sample_dir = local_samples_dir();
    if !sample_dir.exists() {
        return;
    }

    let mut _sample_count = 0usize;
    let mut files_with_images = 0usize;
    let mut image_payload_count = 0usize;
    let mut projected_payload_count = 0usize;
    let mut svg_overlay_count = 0usize;
    let mut layer_op_count = 0usize;
    let mut overlay_json_count = 0usize;
    let mut ownership_proven_count = 0usize;
    let mut frame_geometry_candidate_count = 0usize;
    let mut embedding_frame_trace_count = 0usize;
    let mut source_frame_record_geometry_count = 0usize;
    let mut candidate_frame_bbox_count = 0usize;
    let mut payload_frame_aspect_fit_count = 0usize;
    let mut final_gate_blocker_count = 0usize;
    let mut failures = Vec::new();

    let no_jtd_samples = fs::read_dir(&sample_dir).unwrap().all(|entry| {
        !entry
            .unwrap()
            .path()
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| matches!(ext, "jtd" | "jtt" | "jttc"))
    });
    if no_jtd_samples {
        return;
    }

    for entry in fs::read_dir(&sample_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if !matches!(extension, "jtd" | "jtt" | "jttc") {
            continue;
        }

        _sample_count += 1;
        let bytes = fs::read(&path).unwrap();
        match DocumentCore::from_bytes(&bytes) {
            Ok(core) => {
                let current_payload_count = image_payload_diagnostics(core.document()).len();
                if current_payload_count == 0 {
                    continue;
                }

                files_with_images += 1;
                image_payload_count += current_payload_count;
                projected_payload_count +=
                    current_payload_count.min(APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS);
                let svg = core.render_page_svg(0).unwrap();
                let layer_tree = core.get_page_layer_tree(0).unwrap();
                let overlay_images = core.get_page_overlay_images(0).unwrap();
                svg_overlay_count += svg
                    .matches("class=\"rjtd-image-payload-diagnostic\"")
                    .count();
                layer_op_count += layer_tree
                    .matches("\"type\":\"imagePayloadDiagnostic\"")
                    .count();
                overlay_json_count += overlay_images
                    .matches("\"type\":\"jtdImagePayloadCandidate\"")
                    .count();
                ownership_proven_count += layer_tree.matches("\"ownershipProven\":true").count();
                frame_geometry_candidate_count += layer_tree
                    .matches("\"frameGeometryCandidatePresent\":true")
                    .count();
                embedding_frame_trace_count += layer_tree
                    .matches("\"embeddingFrameTracePresent\":true")
                    .count();
                source_frame_record_geometry_count += layer_tree
                    .matches("\"sourceFrameRecordGeometryPresent\":true")
                    .count();
                candidate_frame_bbox_count += layer_tree
                    .matches("\"candidateFrameBBox\":{\"source\":\"EmbeddingInfo+/FrameRecord\"")
                    .count();
                payload_frame_aspect_fit_count += layer_tree
                    .matches("\"payloadFrameAspectFit\":{\"source\":\"imagePayloadDimensions+/FrameRecord\"")
                    .count();
                final_gate_blocker_count += layer_tree
                    .matches(
                        "\"renderPromotionBlockedReason\":\"image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven\"",
                    )
                    .count();

                assert!(svg.contains("data:image/png;base64,"));
                assert!(svg.contains("data-decoded=\"false\""));
                assert!(svg.contains("data-geometry-decoded=\"false\""));
                assert!(svg.contains("data-placement-proven=\"false\""));
                assert!(svg.contains("data-diagnostic-renderable=\"true\""));
                assert!(svg.contains("data-renderable=\"false\""));
                assert!(svg.contains("data-frame-reference-row-count=\""));
                assert!(svg.contains("data-frame-coordinate-row-count=\""));
                assert!(svg.contains("data-frame-linked-window-row-count=\""));
                assert!(svg.contains("data-frame-geometry-candidate-present=\""));
                assert!(svg.contains("data-embedding-frame-trace-present=\""));
                assert!(svg.contains("data-source-frame-record-geometry-present=\""));
                assert!(svg.contains("data-candidate-frame-bbox-present=\""));
                assert!(svg.contains("data-candidate-frame-x=\""));
                assert!(svg.contains("data-candidate-frame-y=\""));
                assert!(svg.contains("data-candidate-frame-width=\""));
                assert!(svg.contains("data-candidate-frame-height=\""));
                assert!(svg.contains("data-payload-frame-aspect-fit-present=\""));
                assert!(svg.contains("data-payload-frame-aspect-delta-permille=\""));
                assert!(svg.contains("data-best-payload-frame-aspect-delta-permille=\""));
                assert!(svg.contains("data-current-payload-best-frame-aspect-candidate=\""));
                assert!(layer_tree.contains("\"placementProven\":false"));
                assert!(layer_tree.contains("\"diagnosticRenderable\":true"));
                assert!(layer_tree.contains("\"renderable\":false"));
                assert!(layer_tree.contains("\"ownershipProven\":"));
                assert!(layer_tree.contains("\"frameReferenceRowCount\":"));
                assert!(layer_tree.contains("\"frameCoordinateRowCount\":"));
                assert!(layer_tree.contains("\"frameLinkedWindowRowCount\":"));
                assert!(layer_tree.contains("\"frameGeometryCandidatePresent\":"));
                assert!(layer_tree.contains("\"embeddingFrameTracePresent\":"));
                assert!(layer_tree.contains("\"sourceFrameRecordGeometryPresent\":"));
                assert!(layer_tree.contains("\"sourceFrameTrace\":"));
                assert!(layer_tree.contains("\"candidateFrameBBox\":"));
                assert!(layer_tree.contains("\"payloadFrameAspectFit\":"));
                assert!(layer_tree.contains("\"pageGeometryProven\":false"));
                assert!(layer_tree.contains("\"paintOrderDecoded\":false"));
                assert!(layer_tree.contains("\"renderPromotionBlockedReason\":"));
                assert!(layer_tree.contains("\"objectEnvelope\":"));
                assert!(overlay_images.contains("\"placementProven\":false"));
                assert!(overlay_images.contains("\"geometryDecoded\":false"));
                assert!(overlay_images.contains("\"diagnosticRenderable\":true"));
                assert!(overlay_images.contains("\"renderable\":false"));
                assert!(overlay_images.contains("\"ownershipProven\":"));
                assert!(overlay_images.contains("\"frameReferenceRowCount\":"));
                assert!(overlay_images.contains("\"frameCoordinateRowCount\":"));
                assert!(overlay_images.contains("\"frameLinkedWindowRowCount\":"));
                assert!(overlay_images.contains("\"frameGeometryCandidatePresent\":"));
                assert!(overlay_images.contains("\"embeddingFrameTracePresent\":"));
                assert!(overlay_images.contains("\"sourceFrameRecordGeometryPresent\":"));
                assert!(overlay_images.contains("\"sourceFrameTrace\":"));
                assert!(overlay_images.contains("\"candidateFrameBBox\":"));
                assert!(overlay_images.contains("\"payloadFrameAspectFit\":"));
                assert!(overlay_images.contains("\"pageGeometryProven\":false"));
                assert!(overlay_images.contains("\"paintOrderDecoded\":false"));
                assert!(overlay_images.contains("\"renderPromotionBlockedReason\":"));
                assert!(overlay_images.contains("\"objectEnvelope\":"));
            }
            Err(error) => failures.push(format!("{}: {error}", path.display())),
        }
    }

    assert_eq!(failures, Vec::<String>::new());
    if files_with_images == 0 {
        return;
    }
    assert_eq!(svg_overlay_count, projected_payload_count);
    assert_eq!(layer_op_count, projected_payload_count);
    assert_eq!(overlay_json_count, image_payload_count);
    assert!(ownership_proven_count > 0);
    assert!(frame_geometry_candidate_count > 0);
    assert!(embedding_frame_trace_count > 0);
    assert!(source_frame_record_geometry_count > 0);
    assert!(candidate_frame_bbox_count > 0);
    assert!(payload_frame_aspect_fit_count > 0);
    assert!(final_gate_blocker_count > 0);
}

#[test]
fn document_core_edits_body_paragraphs_and_rebuilds_pages() {
    let document = Document::from_plain_text("銀河鉄道\n午后");
    let mut core = DocumentCore::from_document(document);

    assert_eq!(core.get_section_count(), 1);
    assert_eq!(core.get_paragraph_count(0).unwrap(), 2);
    assert_eq!(core.get_paragraph_length(0, 0).unwrap(), 4);
    assert_eq!(core.get_text_range(0, 0, 1, 2).unwrap(), "河鉄");

    let inserted = core.insert_text(0, 0, 4, "の夜").unwrap();
    assert_eq!(inserted, "{\"ok\":true,\"charOffset\":6}");
    assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河鉄道の夜");
    assert!(core.render_page_svg(0).unwrap().contains("銀河鉄道の夜"));
    assert_eq!(
        core.get_caret_position(),
        "{\"sectionIndex\":0,\"paragraphIndex\":0,\"charOffset\":6}"
    );

    let split = core.split_paragraph(0, 0, 2).unwrap();
    assert_eq!(split, "{\"ok\":true,\"paraIdx\":1,\"charOffset\":0}");
    assert_eq!(core.get_paragraph_count(0).unwrap(), 3);
    assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河");
    assert_eq!(core.get_text_range(0, 1, 0, 10).unwrap(), "鉄道の夜");

    let deleted = core.delete_text(0, 1, 0, 2).unwrap();
    assert_eq!(deleted, "{\"ok\":true,\"charOffset\":0}");
    assert_eq!(core.get_text_range(0, 1, 0, 10).unwrap(), "の夜");

    let merged = core.merge_paragraph(0, 1).unwrap();
    assert_eq!(merged, "{\"ok\":true,\"paraIdx\":0,\"charOffset\":2}");
    assert_eq!(core.get_paragraph_count(0).unwrap(), 2);
    assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河の夜");
}

#[test]
fn document_core_reports_default_formatting_for_app_panels() {
    let document = Document::from_plain_text("銀河鉄道");
    let mut core = DocumentCore::from_document(document);

    let char_props = core.get_char_properties_at(0, 0, 0).unwrap();
    assert!(char_props.contains("\"fontFamily\":\"Hiragino Sans\""));
    assert!(char_props.contains("\"bold\":false"));

    let para_props = core.get_para_properties_at(0, 0).unwrap();
    assert!(para_props.contains("\"alignment\":\"left\""));
    assert!(para_props.contains("\"lineSpacing\":160"));

    assert_eq!(
        core.apply_char_format(0, 0, 0, 2, "{\"bold\":true}")
            .unwrap(),
        "{\"ok\":true}"
    );
    assert_eq!(
        core.apply_para_format(0, 0, "{\"alignment\":\"center\"}")
            .unwrap(),
        "{\"ok\":true}"
    );
    assert_eq!(core.find_or_create_font_id("Hiragino Sans"), 0);
    let style_list = core.get_style_list();
    assert!(style_list.contains("\"name\":\"Normal\""));
    assert!(style_list.contains("\"sourceStreamCount\":0"));
    let style_detail = core.get_style_detail(0).unwrap();
    assert!(style_detail.contains("\"charProps\""));
    assert!(style_detail.contains("\"decoded\":false"));
    assert!(style_detail.contains("\"sourceStreams\":[]"));
    assert_eq!(
        core.get_style_at(0, 0).unwrap(),
        "{\"id\":0,\"name\":\"Normal\"}"
    );
    assert_eq!(core.apply_style(0, 0, 0).unwrap(), "{\"ok\":true}");
    assert_eq!(core.get_numbering_list(), "[]");
    assert_eq!(core.get_bullet_list(), "[]");
    assert_eq!(core.ensure_default_numbering(), 0);
    assert_eq!(core.ensure_default_bullet("*"), 0);
}

#[test]
fn document_core_supports_body_selection_and_internal_clipboard() {
    let document = Document::from_plain_text("銀河鉄道\n午后の授業\n星めぐり");
    let mut core = DocumentCore::from_document(document);

    let rects = core.get_selection_rects(0, 0, 1, 1, 2).unwrap();
    assert!(rects.starts_with("[{\"pageIndex\":0"));
    assert!(rects.contains("\"height\":23.0"));

    let copied = core.copy_selection(0, 0, 2, 1, 2).unwrap();
    assert_eq!(copied, "{\"ok\":true,\"text\":\"鉄道\\n午后\"}");
    assert!(core.has_internal_clipboard());
    assert_eq!(core.get_clipboard_text(), "鉄道\n午后");

    let pasted = core.paste_internal(0, 2, 0).unwrap();
    assert_eq!(pasted, "{\"ok\":true,\"paraIdx\":3,\"charOffset\":2}");
    assert_eq!(core.get_text_range(0, 2, 0, 10).unwrap(), "鉄道");
    assert_eq!(core.get_text_range(0, 3, 0, 10).unwrap(), "午后星めぐり");

    let deleted = core.delete_range(0, 0, 1, 1, 1).unwrap();
    assert_eq!(deleted, "{\"ok\":true,\"paraIdx\":0,\"charOffset\":1}");
    assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀后の授業");

    core.clear_clipboard();
    assert!(!core.has_internal_clipboard());
    assert_eq!(core.get_clipboard_text(), "");
    assert!(!core.clipboard_has_control());
}

#[test]
fn document_core_saves_restores_and_discards_snapshots() {
    let document = Document::from_plain_text("銀河鉄道\n午后の授業");
    let mut core = DocumentCore::from_document(document);
    core.set_file_name("sample.jtd");
    core.set_dpi(120.0);
    core.set_writing_mode(WritingMode::VerticalRl);
    core.copy_selection(0, 0, 0, 0, 2).unwrap();

    let snapshot_id = core.save_snapshot();
    assert_eq!(snapshot_id, 1);

    core.insert_text(0, 0, 4, "の夜").unwrap();
    core.set_file_name("edited.jtd");
    core.set_dpi(144.0);
    core.set_writing_mode(WritingMode::Horizontal);
    core.set_show_control_codes(true);
    core.set_show_transparent_borders(true);
    core.clear_clipboard();
    assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河鉄道の夜");

    let restored = core.restore_snapshot(snapshot_id).unwrap();
    assert_eq!(restored, "{\"ok\":true,\"pageCount\":1}");
    assert_eq!(core.get_text_range(0, 0, 0, 10).unwrap(), "銀河鉄道");
    assert_eq!(core.file_name(), "sample.jtd");
    assert_eq!(core.get_dpi(), 120.0);
    assert_eq!(core.writing_mode(), WritingMode::VerticalRl);
    assert_eq!(core.get_clipboard_text(), "銀河");
    assert!(!core.get_show_control_codes());
    assert!(!core.get_show_transparent_borders());

    core.discard_snapshot(snapshot_id);
    assert!(core.restore_snapshot(snapshot_id).is_err());
}

#[test]
fn document_core_searches_and_replaces_body_text() {
    let document = Document::from_plain_text("Alpha alpha\nBeta Alpha");
    let mut core = DocumentCore::from_document(document);

    assert_eq!(
        core.search_all_text("Alpha", true, false),
        "[{\"sec\":0,\"para\":0,\"charOffset\":0,\"length\":5},{\"sec\":0,\"para\":1,\"charOffset\":5,\"length\":5}]"
    );
    assert_eq!(
        core.search_all_text("alpha", false, false),
        "[{\"sec\":0,\"para\":0,\"charOffset\":0,\"length\":5},{\"sec\":0,\"para\":0,\"charOffset\":6,\"length\":5},{\"sec\":0,\"para\":1,\"charOffset\":5,\"length\":5}]"
    );
    assert_eq!(
        core.search_text("Alpha", 0, 1, 5, true, true).unwrap(),
        "{\"found\":true,\"wrapped\":true,\"sec\":0,\"para\":0,\"charOffset\":0,\"length\":5}"
    );
    assert_eq!(
        core.search_text("Alpha", 0, 0, 0, false, true).unwrap(),
        "{\"found\":true,\"wrapped\":true,\"sec\":0,\"para\":1,\"charOffset\":5,\"length\":5}"
    );

    assert_eq!(
        core.replace_text(0, 0, 6, 5, "omega").unwrap(),
        "{\"ok\":true,\"charOffset\":6,\"newLength\":5}"
    );
    assert_eq!(core.get_text_range(0, 0, 0, 20).unwrap(), "Alpha omega");

    assert_eq!(
        core.replace_one("Alpha", "A", true).unwrap(),
        "{\"ok\":true,\"sec\":0,\"para\":0,\"charOffset\":0,\"newLength\":1}"
    );
    assert_eq!(core.get_text_range(0, 0, 0, 20).unwrap(), "A omega");

    assert_eq!(
        core.replace_all("Alpha", "X", true).unwrap(),
        "{\"ok\":true,\"count\":1}"
    );
    assert_eq!(core.get_text_range(0, 1, 0, 20).unwrap(), "Beta X");
}

#[test]
fn document_core_exposes_view_and_navigation_fallbacks() {
    let document = Document::from_plain_text("銀河鉄道\n午后");
    let mut core = DocumentCore::from_document(document);

    assert!(!core.get_show_control_codes());
    core.set_show_paragraph_marks(true);
    core.set_show_control_codes(true);
    core.set_show_transparent_borders(true);
    core.set_clip_enabled(false);
    assert!(core.get_show_control_codes());
    assert!(core.get_show_transparent_borders());

    assert_eq!(
        core.get_position_of_page(0).unwrap(),
        "{\"ok\":true,\"sec\":0,\"para\":0,\"charOffset\":0}"
    );
    assert_eq!(
        core.get_page_of_position(0, 1).unwrap(),
        "{\"ok\":true,\"page\":0}"
    );
    assert_eq!(core.get_control_text_positions(0, 0), "[]");
    assert_eq!(
        core.find_nearest_control_backward(0, 0, 4),
        "{\"type\":\"none\"}"
    );
    assert_eq!(
        core.find_nearest_control_forward(0, 0, 0),
        "{\"type\":\"none\"}"
    );
    assert_eq!(
        core.find_next_editable_control(0, 0, -1, 1),
        "{\"type\":\"body\",\"sec\":0,\"para\":1}"
    );
    assert_eq!(
        core.find_next_editable_control(0, 1, -1, 1),
        "{\"type\":\"none\"}"
    );
    assert_eq!(
        core.navigate_next_editable(0, 0, 0, 1, "[]"),
        "{\"type\":\"text\",\"sec\":0,\"para\":0,\"charOffset\":1,\"context\":[]}"
    );
    assert_eq!(
        core.navigate_next_editable(0, 0, 0, -1, "[]"),
        "{\"type\":\"boundary\"}"
    );
}

#[test]
fn document_core_projects_preserved_text_controls_for_navigation() {
    let core =
        DocumentCore::from_bytes(&cfb_with_document_text(document_text_with_inline())).unwrap();

    assert_eq!(core.get_control_text_positions(0, 0), "[2]");
    assert_eq!(core.get_control_text_positions(0, 1), "[]");
    assert_eq!(core.get_control_text_positions(0, 99), "[]");
    assert_eq!(
        core.find_nearest_control_forward(0, 0, 0),
        "{\"type\":\"jtdControl\",\"sec\":0,\"para\":0,\"ci\":0,\"charPos\":2,\"code\":28,\"codeHex\":\"0x001c\",\"decoded\":false}"
    );
    assert_eq!(
        core.find_nearest_control_backward(0, 0, 3),
        "{\"type\":\"jtdControl\",\"sec\":0,\"para\":0,\"ci\":0,\"charPos\":2,\"code\":28,\"codeHex\":\"0x001c\",\"decoded\":false}"
    );
    assert_eq!(
        core.find_nearest_control_forward(0, 0, 2),
        "{\"type\":\"none\"}"
    );
    assert_eq!(
        core.find_nearest_control_backward(0, 0, 2),
        "{\"type\":\"none\"}"
    );

    let layout = core.get_page_control_layout(0).unwrap();
    assert!(layout.starts_with("{\"controls\":[{"));
    assert!(layout.contains("\"type\":\"jtdControl\""));
    assert!(layout.contains("\"x\":"));
    assert!(layout.contains("\"y\":"));
    assert!(layout.contains("\"w\":"));
    assert!(layout.contains("\"h\":"));
    assert!(layout.contains("\"secIdx\":0"));
    assert!(layout.contains("\"paraIdx\":0"));
    assert!(layout.contains("\"controlIdx\":0"));
    assert!(layout.contains("\"charPos\":2"));
    assert!(layout.contains("\"codeHex\":\"0x001c\""));
    assert!(layout.contains("\"decoded\":false"));
    assert!(layout.contains("\"source\":\"textControlBoundary\""));
}

#[test]
fn document_core_exposes_absent_table_and_cell_fallbacks() {
    let document = Document::from_plain_text("銀河鉄道");
    let mut core = DocumentCore::from_document(document);

    assert_eq!(
        core.get_column_def(0).unwrap(),
        "{\"columnCount\":1,\"columnType\":0,\"sameWidth\":true,\"spacing\":0}"
    );
    assert_eq!(
        core.get_table_dimensions(0, 0, 0).unwrap(),
        "{\"rowCount\":0,\"colCount\":0,\"cellCount\":0}"
    );
    assert_eq!(
        core.get_table_dimensions_by_path(0, 0, "[]").unwrap(),
        "{\"rowCount\":0,\"colCount\":0,\"cellCount\":0}"
    );
    assert_eq!(
        core.get_cell_info(0, 0, 0, 0).unwrap(),
        "{\"row\":0,\"col\":0,\"rowSpan\":1,\"colSpan\":1}"
    );
    assert_eq!(
        core.get_cell_info_by_path(0, 0, "[]").unwrap(),
        "{\"row\":0,\"col\":0,\"rowSpan\":1,\"colSpan\":1}"
    );
    assert!(
        core.get_cell_properties(0, 0, 0, 0)
            .unwrap()
            .contains("\"isHeader\":false")
    );
    assert!(
        core.get_table_properties(0, 0, 0)
            .unwrap()
            .contains("\"repeatHeader\":false")
    );
    assert_eq!(core.get_table_cell_bboxes(0, 0, 0, None).unwrap(), "[]");
    assert_eq!(
        core.get_table_cell_bboxes_by_path(0, 0, "[]").unwrap(),
        "[]"
    );
    assert!(
        core.get_cursor_rect_in_cell(0, 0, 0, 0, 0, 0)
            .unwrap()
            .contains("\"height\":23.0")
    );
    assert_eq!(
        core.get_line_info_in_cell(0, 0, 0, 0, 0, 0).unwrap(),
        "{\"lineIndex\":0,\"lineCount\":1,\"charStart\":0,\"charEnd\":0}"
    );
    assert_eq!(core.get_cell_paragraph_count(0, 0, 0, 0).unwrap(), 0);
    assert_eq!(core.get_cell_paragraph_length(0, 0, 0, 0, 0).unwrap(), 0);
    assert_eq!(core.get_text_in_cell(0, 0, 0, 0, 0, 0, 10).unwrap(), "");
    assert_eq!(
        core.insert_text_in_cell(0, 0, 0, 0, 0, 0, "x").unwrap(),
        "{\"ok\":false,\"charOffset\":0}"
    );
    assert_eq!(
        core.delete_text_in_cell(0, 0, 0, 0, 0, 0, 1).unwrap(),
        "{\"ok\":false,\"charOffset\":0}"
    );
    assert_eq!(
        core.create_table(0, 0, 0, 2, 2).unwrap(),
        "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
    );
    assert_eq!(
        core.insert_table_row(0, 0, 0, 0, true).unwrap(),
        "{\"ok\":false,\"rowCount\":0,\"colCount\":0}"
    );
    assert_eq!(
        core.delete_table_column(0, 0, 0, 0).unwrap(),
        "{\"ok\":false,\"rowCount\":0,\"colCount\":0}"
    );
    assert_eq!(
        core.merge_table_cells(0, 0, 0, 0, 0, 0, 1).unwrap(),
        "{\"ok\":false,\"cellCount\":0}"
    );
    assert_eq!(
        core.split_table_cell(0, 0, 0, 0, 0).unwrap(),
        "{\"ok\":false,\"cellCount\":0}"
    );
    assert_eq!(
        core.get_selection_rects_in_cell(0, 0, 0, 0, 0, 0, 0, 0)
            .unwrap(),
        "[]"
    );
    assert_eq!(
        core.copy_selection_in_cell(0, 0, 0, 0, 0, 0, 0, 0).unwrap(),
        "{\"ok\":false,\"text\":\"\"}"
    );
    assert_eq!(
        core.delete_range_in_cell(0, 0, 0, 0, 0, 0, 0, 0).unwrap(),
        "{\"ok\":false,\"paraIdx\":0,\"charOffset\":0}"
    );
    assert!(
        core.get_cell_char_properties_at(0, 0, 0, 0, 0, 0)
            .unwrap()
            .contains("\"fontFamily\":\"Hiragino Sans\"")
    );
    assert!(
        core.get_cell_para_properties_at(0, 0, 0, 0, 0)
            .unwrap()
            .contains("\"alignment\":\"left\"")
    );
    assert_eq!(
        core.get_cell_style_at(0, 0, 0, 0, 0).unwrap(),
        "{\"id\":0,\"name\":\"Normal\"}"
    );
    assert_eq!(
        core.apply_char_format_in_cell(0, 0, 0, 0, 0, 0, 0, "{}")
            .unwrap(),
        "{\"ok\":false}"
    );
    assert_eq!(
        core.apply_para_format_in_cell(0, 0, 0, 0, 0, "{}").unwrap(),
        "{\"ok\":false}"
    );
    assert_eq!(
        core.apply_cell_style(0, 0, 0, 0, 0, 0).unwrap(),
        "{\"ok\":false}"
    );
    assert_eq!(
        core.evaluate_table_formula(0, 0, 0, 0, 0, "=SUM(A1:A2)", false)
            .unwrap(),
        "{\"ok\":false,\"value\":\"\",\"formula\":\"=SUM(A1:A2)\"}"
    );
}

#[test]
fn document_core_exposes_absent_object_bookmark_and_form_fallbacks() {
    let document = Document::from_plain_text("銀河鉄道");
    let mut core = DocumentCore::from_document(document);

    assert_eq!(core.get_paragraph_stable_id(0, 0).unwrap(), "rjtd-p0");
    core.ensure_paragraph_stable_ids();
    assert!(
        core.debug_dump_stable_ids(0, 0, 1)
            .unwrap()
            .contains("\"stableId\":\"rjtd-p0\"")
    );
    assert_eq!(core.get_table_signature(0, 0, 0).unwrap(), "");
    assert!(
        core.get_shape_bbox(0, 0, 0)
            .unwrap()
            .contains("\"width\":0.0")
    );
    assert_eq!(
        core.insert_picture(0, 0, 0, "", &[], 1, 1, 1, 1, "png", "", None, None)
            .unwrap(),
        "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
    );
    assert!(
        core.get_picture_properties(0, 0, 0)
            .unwrap()
            .contains("\"effect\":\"none\"")
    );
    assert_eq!(
        core.set_picture_properties(0, 0, 0, "{}").unwrap(),
        "{\"ok\":false}"
    );
    assert_eq!(
        core.delete_picture_control(0, 0, 0).unwrap(),
        "{\"ok\":false}"
    );
    assert!(
        core.get_cell_shape_properties_by_path(0, 0, "[]", 0)
            .unwrap()
            .contains("\"description\":\"\"")
    );
    assert!(
        core.get_equation_properties(0, 0, 0, -1, -1)
            .unwrap()
            .contains("\"script\":\"\"")
    );
    assert!(
        core.render_equation_preview("x+y", 1000, 0)
            .contains(">x+y<")
    );
    assert_eq!(
        core.create_shape_control("{}").unwrap(),
        "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
    );
    assert_eq!(
        core.change_shape_z_order(0, 0, 0, "front").unwrap(),
        "{\"ok\":false,\"zOrder\":0}"
    );
    assert_eq!(
        core.group_shapes("{}"),
        "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
    );
    assert_eq!(core.ungroup_shape(0, 0, 0).unwrap(), "{\"ok\":false}");
    assert_eq!(
        core.insert_equation(0, 0, 0, "x", 1000, 0).unwrap(),
        "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
    );
    assert_eq!(
        core.get_form_object_at(0, 0.0, 0.0).unwrap(),
        "{\"found\":false}"
    );
    assert_eq!(core.get_form_value(0, 0, 0).unwrap(), "{\"ok\":false}");
    assert_eq!(
        core.set_form_value_in_cell(0, 0, 0, 0, 0, 0, "{}").unwrap(),
        "{\"ok\":false}"
    );
    assert_eq!(core.copy_control(0, 0, "", 0).unwrap(), "{\"ok\":false}");
    assert_eq!(
        core.paste_control(0, 0, 0).unwrap(),
        "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
    );
    assert!(core.get_control_image_data(0, 0, "", 0).unwrap().is_empty());
    assert_eq!(core.get_control_image_mime(0, 0, "", 0).unwrap(), "");
    assert_eq!(core.get_bookmarks(), "[]");
    assert!(
        core.add_bookmark(0, 0, 0, "mark")
            .unwrap()
            .contains("\"ok\":false")
    );
    assert!(core.export_hwp().is_empty());
    assert!(core.export_hwpx().is_empty());
    assert!(core.export_hwp_verify().contains("\"ok\":false"));
    assert_eq!(
        core.insert_page_break(0, 0, 0).unwrap(),
        "{\"ok\":false,\"charOffset\":0}"
    );
    assert_eq!(
        core.insert_column_break(0, 0, 0).unwrap(),
        "{\"ok\":false,\"charOffset\":0}"
    );
    assert_eq!(
        core.set_column_def(0, 1, 0, 1, 0).unwrap(),
        "{\"ok\":true,\"pageCount\":1}"
    );
    assert_eq!(core.create_style("{}"), 0);
    assert!(core.update_style(0, "{}"));
    assert!(!core.delete_style(1));
    assert_eq!(core.create_numbering("{}"), 0);
    assert_eq!(
        core.insert_text_in_footnote(0, 0, 0, 0, 0, "x").unwrap(),
        "{\"ok\":false,\"charOffset\":0}"
    );
    assert_eq!(
        core.get_selection_rects_in_footnote(0, 0, 0, 0, 0, 0)
            .unwrap(),
        "[]"
    );
    assert!(
        core.get_para_properties_in_hf(0, true, 0, 0)
            .unwrap()
            .contains("\"alignment\":\"left\"")
    );
    assert_eq!(
        core.insert_field_in_hf(0, true, 0, 0, 0, 0).unwrap(),
        "{\"ok\":false,\"charOffset\":0}"
    );
    assert_eq!(
        core.apply_hf_template(0, true, 0, 0).unwrap(),
        "{\"ok\":false}"
    );
    assert_eq!(
        core.export_selection_html(0, 0, 0, 0, 2).unwrap(),
        "<p>銀河</p>"
    );
    assert_eq!(
        core.paste_html(0, 0, 0, "<p>x</p>").unwrap(),
        "{\"ok\":false,\"charOffset\":0}"
    );
}

#[test]
fn document_core_exposes_absent_field_header_footer_and_note_fallbacks() {
    let document = Document::from_plain_text("銀河鉄道");
    let mut core = DocumentCore::from_document(document);

    assert_eq!(core.get_field_list(), "[]");
    assert_eq!(
        core.get_field_value(7),
        "{\"ok\":false,\"fieldId\":7,\"value\":\"\"}"
    );
    assert_eq!(
        core.get_field_value_by_name("name"),
        "{\"ok\":false,\"fieldId\":0,\"name\":\"name\",\"value\":\"\"}"
    );
    assert_eq!(
        core.set_field_value(7, "value"),
        "{\"ok\":false,\"fieldId\":7,\"oldValue\":\"\",\"newValue\":\"value\"}"
    );
    assert_eq!(core.get_field_info_at(0, 0, 0), "{\"inField\":false}");
    assert_eq!(core.remove_field_at(0, 0, 0), "{\"ok\":false}");
    assert!(!core.set_active_field(0, 0, 0));
    core.clear_active_field();
    assert_eq!(core.get_click_here_props(1), "{\"ok\":false}");
    assert_eq!(
        core.update_click_here_props(1, "guide", "memo", "name", true),
        "{\"ok\":false}"
    );

    assert_eq!(
        core.get_header_footer(0, true, 0).unwrap(),
        "{\"ok\":true,\"exists\":false}"
    );
    assert_eq!(
        core.create_header_footer(0, true, 0).unwrap(),
        "{\"ok\":false,\"exists\":false}"
    );
    assert_eq!(
        core.insert_text_in_header_footer(0, true, 0, 0, 0, "x")
            .unwrap(),
        "{\"ok\":false}"
    );
    assert_eq!(
        core.get_header_footer_para_info(0, true, 0, 0).unwrap(),
        "{\"ok\":false,\"paraCount\":0,\"charCount\":0}"
    );
    assert!(
        core.get_cursor_rect_in_header_footer(0, true, 0, 0, 0, -1)
            .unwrap()
            .contains("\"pageIndex\":0")
    );
    assert_eq!(
        core.get_header_footer_list(0, true, 0),
        "{\"ok\":true,\"items\":[],\"currentIndex\":-1}"
    );
    assert_eq!(
        core.toggle_hide_header_footer(0, true).unwrap(),
        "{\"ok\":false,\"hidden\":false}"
    );
    assert_eq!(
        core.navigate_header_footer_by_page(0, true, 1),
        "{\"ok\":false}"
    );

    assert_eq!(core.insert_footnote(0, 0, 0).unwrap(), "{\"ok\":false}");
    assert_eq!(core.insert_endnote(0, 0, 0).unwrap(), "{\"ok\":false}");
    assert!(
        core.get_endnote_shape(0)
            .unwrap()
            .contains("\"numberFormat\":\"digit\"")
    );
    assert_eq!(core.apply_endnote_shape(0, "{}").unwrap(), "{\"ok\":false}");
    assert_eq!(
        core.get_footnote_info(0, 0, 0).unwrap(),
        "{\"ok\":false,\"paraCount\":0,\"totalTextLen\":0,\"number\":0,\"texts\":[]}"
    );
    assert!(
        core.delete_footnote(0, 0, 0)
            .unwrap()
            .contains("\"ok\":false")
    );
    assert_eq!(core.get_page_footnote_info(0, 0).unwrap(), "{\"ok\":false}");
    assert_eq!(core.get_note_edit_info(0, 0, 0).unwrap(), "{\"ok\":false}");
    assert_eq!(
        core.get_note_equation_properties(0, 0, 0, 0, 0),
        "{\"ok\":false}"
    );
    assert_eq!(
        core.set_note_equation_properties(0, 0, 0, 0, 0, "{}"),
        "{\"ok\":false}"
    );
}

#[test]
fn document_core_rejects_out_of_range_app_page_queries() {
    let document = Document::from_plain_text("銀河鉄道");
    let core = DocumentCore::from_document(document);

    assert!(core.get_page_info(1).is_err());
    assert!(core.get_page_layer_tree(1).is_err());
    assert!(core.get_page_overlay_images(1).is_err());
    assert!(core.get_page_def(1).is_err());
    assert!(core.get_section_def(1).is_err());
    assert!(core.get_page_border_fill(1).is_err());
    assert!(core.get_cursor_rect(0, 1, 0).is_err());
    assert!(core.get_line_info(0, 1, 0).is_err());
    assert!(core.hit_test(1, 72.0, 72.0).is_err());
}

#[test]
fn document_core_renders_raw_stream_notice_when_text_is_empty() {
    let mut document = Document::default();
    document.push_raw_stream(RawStream::new("/DocumentText", vec![0, 1]));
    let core = DocumentCore::from_document(document);

    let svg = core.render_page_svg(0).unwrap();

    assert!(svg.contains("No extractable text"));
    assert!(svg.contains("/DocumentText"));
}

#[test]
fn builds_document_from_structured_document_text_elements() {
    let parsed = rjtd_core::document_text::parse_document_text(&document_text_with_inline());
    let document = Document::from_document_text(&parsed);

    assert_eq!(document.blocks().len(), 2);
    assert_eq!(document.text_control_boundaries().len(), 1);
    assert_eq!(document.text_control_boundaries()[0].index(), 0);
    assert_eq!(document.text_control_boundaries()[0].code(), 0x001c);
    assert!(
        document.text_control_boundaries()[0]
            .source_span()
            .is_none()
    );
    match &document.blocks()[0] {
        Block::Paragraph(paragraph) => {
            assert_eq!(paragraph.inlines().len(), 3);
            assert_text_inline(&paragraph.inlines()[0], "一、");
            assert_text_inline(&paragraph.inlines()[1], "午后");
            assert_text_inline(&paragraph.inlines()[2], "の授業");
        }
        Block::Unknown(_) => panic!("expected paragraph"),
    }
    match &document.blocks()[1] {
        Block::Paragraph(paragraph) => assert_text_inline(&paragraph.inlines()[0], "二、"),
        Block::Unknown(_) => panic!("expected paragraph"),
    }
}

#[test]
fn preserves_skipped_inline_text_as_unknown_object() {
    let parsed =
        rjtd_core::document_text::parse_document_text(&document_text_with_skipped_inline());
    let document = Document::from_document_text(&parsed);

    assert_eq!(document.blocks().len(), 1);
    assert_eq!(document.unknown_objects().len(), 1);
    assert_eq!(
        document.unknown_objects()[0].source().tag(),
        Some(DOCUMENT_TEXT_INLINE_START_TAG)
    );
    assert!(!document.unknown_objects()[0].payload().is_empty());
    match &document.blocks()[0] {
        Block::Paragraph(paragraph) => assert_text_inline(&paragraph.inlines()[0], "本文"),
        Block::Unknown(_) => panic!("expected paragraph"),
    }
}

#[test]
fn promotes_ruby_base_and_annotation_to_structured_inline() {
    let parsed = rjtd_core::document_text::parse_document_text(&document_text_with_ruby());
    let document = Document::from_document_text(&parsed);

    assert!(document.unknown_objects().is_empty());
    assert_eq!(document.blocks().len(), 1);
    match &document.blocks()[0] {
        Block::Paragraph(paragraph) => {
            assert_eq!(paragraph.inlines().len(), 3);
            assert_text_inline(&paragraph.inlines()[0], "一、");
            assert_ruby_inline(&paragraph.inlines()[1], "午后", "ごご");
            assert_text_inline(&paragraph.inlines()[2], "の授業");
        }
        Block::Unknown(_) => panic!("expected paragraph"),
    }
}

#[test]
fn parser_builds_model_and_preserves_raw_document_text_stream() {
    let bytes = cfb_with_document_text(document_text_fixture());
    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.blocks().len(), 1);
    match &document.blocks()[0] {
        Block::Paragraph(paragraph) => match &paragraph.inlines()[0] {
            Inline::Text(run) => {
                let span = run.source_span().unwrap();
                assert_eq!(span.byte_start(), 10);
                assert_eq!(span.byte_end(), 14);
                assert_eq!(span.unit_start(), 5);
                assert_eq!(span.unit_end(), 7);
            }
            Inline::Ruby(_) => panic!("expected text inline"),
            Inline::Unknown(_) => panic!("expected text inline"),
        },
        Block::Unknown(_) => panic!("expected paragraph"),
    }
    assert_eq!(document.raw_streams().len(), 1);
    assert_eq!(document.raw_streams()[0].name(), "/DocumentText");
    assert_eq!(document.raw_streams()[0].bytes(), &document_text_fixture());

    let layer_tree = DocumentCore::from_document(document)
        .get_page_layer_tree(0)
        .unwrap();
    assert!(layer_tree.contains("\"stableSourceKey\":\"section:0/para:0/char:0\""));
    assert!(layer_tree.contains("\"jtdByteRange\":{\"start\":10,\"end\":14}"));
    assert!(layer_tree.contains("\"jtdUnitRange\":{\"start\":5,\"end\":7}"));
}

#[test]
fn parser_preserves_line_mark_stream_for_layout_corroboration() {
    let line_mark = line_mark_words_0_to_20();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        ("/LineMark", &line_mark),
    ]);

    let document = parse_document(&bytes).unwrap();

    let raw_stream = document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/LineMark")
        .unwrap();
    assert_eq!(raw_stream.bytes(), line_mark.as_slice());
}

#[test]
fn parser_preserves_layout_box_streams_for_box_text_projection() {
    let layout_box = layout_box_record_fixture(50, 120, 320);
    let layout_box_text = layout_box_text_plain_block_fixture("本文テキスト");
    let layout_box_text_positions = layout_box_text_position_tables_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (LAYOUT_BOX_PATH, &layout_box),
        (LAYOUT_BOX_TEXT_PATH, &layout_box_text),
        (
            LAYOUT_BOX_TEXT_POSITION_TABLES_PATH,
            &layout_box_text_positions,
        ),
    ]);

    let document = parse_document(&bytes).unwrap();

    assert!(
        document
            .raw_streams()
            .iter()
            .any(|stream| stream.name() == LAYOUT_BOX_PATH && stream.bytes() == layout_box)
    );
    assert!(
        document.raw_streams().iter().any(
            |stream| stream.name() == LAYOUT_BOX_TEXT_PATH && stream.bytes() == layout_box_text
        )
    );
    assert!(document.raw_streams().iter().any(|stream| stream.name()
        == LAYOUT_BOX_TEXT_POSITION_TABLES_PATH
        && stream.bytes() == layout_box_text_positions));
}

#[test]
fn layout_box_text_projection_decodes_plain_textv_blocks() {
    let mut document = Document::from_plain_text("既存本文");
    document.push_raw_stream(RawStream::new(
        LAYOUT_BOX_PATH,
        layout_box_record_fixture(50, 120, 320),
    ));
    document.push_raw_stream(RawStream::new(
        LAYOUT_BOX_TEXT_PATH,
        layout_box_text_plain_block_fixture(
            "世の中には忘れられない顔がある。逆にすぐ忘れる顔もある。",
        ),
    ));
    document.push_raw_stream(RawStream::new(
        LAYOUT_BOX_TEXT_POSITION_TABLES_PATH,
        layout_box_text_position_tables_fixture(),
    ));
    let core = DocumentCore::from_document(document);

    let layer_tree = core.get_page_layer_tree(0).unwrap();
    let svg = core.render_page_svg(0).unwrap();

    assert!(layer_tree.contains("\"sourceStream\":\"/LayoutBoxText\""));
    assert!(layer_tree.contains("\"projectionKind\":\"layoutBoxTextProjection\""));
    assert!(layer_tree.contains("\"layoutFields\""));
    assert!(layer_tree.contains("世の中には忘れられない顔"));
    assert!(svg.contains("rjtd-layout-box-text-projection"));
    assert!(svg.contains("世の中には忘れられない顔"));
}

#[test]
fn parser_preserves_font_stream_entries_as_document_fonts() {
    let font_stream = font_stream_fixture(&[(1, "Times New Roman", 18), (2, "ＭＳ 明朝", 18)]);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (FONT_STREAM_PATH, &font_stream),
    ]);

    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.fonts().len(), 2);
    assert_eq!(document.fonts()[0].source_stream(), FONT_STREAM_PATH);
    assert_eq!(document.fonts()[0].id(), 1);
    assert_eq!(document.fonts()[0].name(), "Times New Roman");
    assert_eq!(document.fonts()[1].name(), "ＭＳ 明朝");
    assert!(!document.fonts()[0].raw().is_empty());

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"fallbackFont\":\"ＭＳ 明朝\""));
    assert!(info.contains("\"fontsUsed\":[\"Times New Roman\",\"ＭＳ 明朝\"]"));
    assert!(info.contains("\"fontCount\":2"));
    assert!(info.contains("\"sourceStream\":\"/Font\""));

    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("font-family=\"&apos;ＭＳ 明朝&apos;, &apos;MS Mincho&apos;"));
    assert!(svg.contains("&apos;Hiragino Mincho ProN&apos;"));
    assert!(!svg.contains("&amp;apos;"));
    let layer_tree = core.get_page_layer_tree(0).unwrap();
    assert!(layer_tree.contains("\"fontFamily\":\"'ＭＳ 明朝', 'MS Mincho'"));
}

#[test]
fn parser_preserves_document_text_control_boundaries_with_source_spans() {
    let bytes = cfb_with_document_text(document_text_with_inline());
    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.text_control_boundaries().len(), 1);
    let boundary = &document.text_control_boundaries()[0];
    assert_eq!(boundary.index(), 0);
    assert_eq!(boundary.code(), 0x001c);
    let span = boundary.source_span().unwrap();
    assert_eq!(span.byte_start(), 6);
    assert_eq!(span.byte_end(), 8);
    assert_eq!(span.unit_start(), 3);
    assert_eq!(span.unit_end(), 4);

    let info = DocumentCore::from_document(document).get_document_info();
    assert!(info.contains("\"textControlBoundaryCount\":1"));
    assert!(info.contains("\"codeHex\":\"0x001c\""));
    assert!(
        info.contains(
            "\"sourceSpan\":{\"byteStart\":6,\"byteEnd\":8,\"unitStart\":3,\"unitEnd\":4}"
        )
    );
    assert!(info.contains("\"decoded\":false"));
}

#[test]
fn parser_preserves_text_count_ranges_as_observed_model_data() {
    let position_table = text_count_table_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            &position_table,
        ),
    ]);
    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.text_count_ranges().len(), 2);
    let first = &document.text_count_ranges()[0];
    assert_eq!(first.index(), 0);
    assert_eq!(first.family(), "be0");
    assert_eq!(first.start(), 0x1234);
    assert_eq!(first.end(), 0x1250);
    assert_eq!(first.span(), 0x1c);
    assert_eq!(first.declared_start(), 0x1234);
    assert_eq!(first.declared_end(), 0x1250);
    assert_eq!(first.tail_fields()[..2], [0x0101, 0x0005]);
    assert!(first.document_text_overlaps().is_empty());
    assert_eq!(first.raw().len(), 29);

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"textCountRangeCount\":2"));
    assert!(info.contains("\"family\":\"be0\""));
    assert!(info.contains("\"tailFields\":[257,5"));
    assert!(info.contains("\"documentTextOverlaps\":[]"));
    assert!(info.contains("\"controlRangeOverlaps\":[]"));
    assert!(info.contains("\"decoded\":false"));
}

#[test]
fn parser_maps_text_count_ranges_to_source_text_overlaps() {
    let position_table = text_count_table_fixture_with_ranges(&[(10, 14), (5, 7)]);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            &position_table,
        ),
    ]);
    let document = parse_document(&bytes).unwrap();

    let byte_overlaps = document.text_count_ranges()[0].document_text_overlaps();
    assert_eq!(byte_overlaps.len(), 1);
    assert_eq!(byte_overlaps[0].basis(), TextCountRangeOverlapBasis::Byte);
    assert_eq!(byte_overlaps[0].block_index(), 0);
    assert_eq!(byte_overlaps[0].inline_index(), 0);
    assert_eq!(byte_overlaps[0].source_start(), 10);
    assert_eq!(byte_overlaps[0].source_end(), 14);
    assert_eq!(byte_overlaps[0].text(), "銀河");

    let unit_overlaps = document.text_count_ranges()[1].document_text_overlaps();
    assert_eq!(unit_overlaps.len(), 1);
    assert_eq!(unit_overlaps[0].basis(), TextCountRangeOverlapBasis::Unit);
    assert_eq!(unit_overlaps[0].source_start(), 5);
    assert_eq!(unit_overlaps[0].source_end(), 7);
    assert_eq!(unit_overlaps[0].text(), "銀河");

    let info = DocumentCore::from_document(document).get_document_info();
    assert!(info.contains("\"documentTextOverlaps\":[{\"basis\":\"byte\""));
    assert!(info.contains("\"documentTextOverlaps\":[{\"basis\":\"unit\""));
}

#[test]
fn parser_maps_text_count_ranges_to_control_range_overlaps() {
    let position_table = text_count_table_fixture_with_ranges(&[(10, 14), (5, 7)]);
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_with_control_boundary()),
        (
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            &position_table,
        ),
    ]);
    let document = parse_document(&bytes).unwrap();

    let first = document.text_count_ranges()[0].control_range_overlaps();
    assert_eq!(first.len(), 2);
    assert_eq!(first[0].basis(), TextCountRangeOverlapBasis::Byte);
    assert_eq!(first[0].delimiter_code(), 0x001c);
    assert_eq!(first[0].range_count(), 1);
    assert_eq!(first[0].first_range_index(), 0);
    assert_eq!(first[0].last_range_index(), 0);
    assert_eq!(first[0].source_start(), 10);
    assert_eq!(first[0].source_end(), 14);
    assert_eq!(first[1].basis(), TextCountRangeOverlapBasis::Unit);
    assert_eq!(first[1].delimiter_code(), 0x001c);
    assert_eq!(first[1].source_start(), 8);
    assert_eq!(first[1].source_end(), 11);

    let second = document.text_count_ranges()[1].control_range_overlaps();
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].basis(), TextCountRangeOverlapBasis::Unit);
    assert_eq!(second[0].delimiter_code(), 0x001c);
    assert_eq!(second[0].first_range_index(), 0);

    let candidates = document.text_boundary_candidates();
    assert_eq!(candidates.len(), 3);
    assert_eq!(candidates[0].index(), 0);
    assert_eq!(candidates[0].kind(), "controlDelimitedTextCountRange");
    assert_eq!(candidates[0].text_count_range_index(), 0);
    assert_eq!(candidates[0].basis(), TextCountRangeOverlapBasis::Byte);
    assert_eq!(candidates[0].delimiter_code(), 0x001c);
    assert_eq!(candidates[0].interval_count(), 1);
    assert_eq!(candidates[0].first_interval_index(), 0);
    assert_eq!(candidates[0].last_interval_index(), 0);
    assert_eq!(candidates[0].source_start(), 10);
    assert_eq!(candidates[0].source_end(), 14);

    let info = DocumentCore::from_document(document).get_document_info();
    assert!(info.contains("\"controlRangeOverlaps\":[{\"basis\":\"byte\""));
    assert!(info.contains("\"delimiterCodeHex\":\"0x001c\""));
    assert!(info.contains("\"rangeCount\":1"));
    assert!(info.contains("\"textBoundaryCandidateCount\":3"));
    assert!(info.contains("\"textBoundaryCandidates\":[{\"index\":0"));
    assert!(info.contains("\"kind\":\"controlDelimitedTextCountRange\""));
    assert!(info.contains("\"textCountRangeIndex\":0"));
    assert!(info.contains("\"intervalCount\":1"));
    assert!(info.contains("\"decoded\":false"));
}

#[test]
fn parser_preserves_layout_validated_paragraph_boundary_candidates() {
    let position_table = text_count_table_fixture_with_ranges(&[(9, 12)]);
    let line_mark = line_mark_words_0_to_20();
    let page_mark = page_mark_fields_0_to_20();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_with_control_boundary()),
        (
            rjtd_core::document_text_position::DOCUMENT_TEXT_POSITION_TABLES_PATH,
            &position_table,
        ),
        ("/LineMark", &line_mark),
        ("/PageMark", &page_mark),
    ]);
    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.text_paragraph_boundary_candidates().len(), 1);
    let candidate = &document.text_paragraph_boundary_candidates()[0];
    assert_eq!(candidate.index(), 0);
    assert_eq!(candidate.kind(), "layoutValidatedTextBoundaryCandidate");
    assert_eq!(candidate.text_count_range_index(), 0);
    assert_eq!(candidate.source_start(), 8);
    assert_eq!(candidate.source_end(), 11);
    assert_eq!(candidate.text_count_range_span(), 3);
    assert_eq!(candidate.line_word_evidence().target(), "line-word-value");
    assert_eq!(candidate.line_word_evidence().base(), "unit");
    assert_eq!(candidate.line_word_evidence().delta(), 0);
    assert_eq!(candidate.page_field_evidence().target(), "page-be32-field");
    assert_eq!(candidate.page_field_evidence().base(), "unit");
    assert_eq!(candidate.page_field_evidence().delta(), 0);

    let core = DocumentCore::from_document(document);
    let info = core.get_document_info();
    assert!(info.contains("\"textParagraphBoundaryCandidateCount\":1"));
    assert!(info.contains("\"textParagraphBoundaryCandidates\":[{\"index\":0"));
    assert!(info.contains("\"textBoundaryCandidateIndex\":1"));
    assert!(info.contains("\"textCountRangeSpan\":3"));
    assert!(info.contains("\"target\":\"line-word-value\""));
    assert!(info.contains("\"target\":\"page-be32-field\""));
    assert!(info.contains("\"decoded\":false"));
    assert!(
        core.get_validation_warnings()
            .contains("\"kind\":\"JtdTextParagraphBoundaryCandidateDiagnosticOnly\"")
    );
}

#[test]
fn parser_preserves_observed_style_streams_as_unknown_styles() {
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH, &[1, 2, 3]),
        (rjtd_core::style_stream::DOCUMENT_EDIT_STYLES_PATH, &[4, 5]),
    ]);
    let document = parse_document(&bytes).unwrap();

    assert_eq!(document.unknown_styles().len(), 2);
    assert_eq!(
        document.unknown_styles()[0].name(),
        Some(rjtd_core::style_stream::DOCUMENT_EDIT_STYLES_PATH)
    );
    assert_eq!(document.unknown_styles()[0].payload(), &[4, 5]);
    assert_eq!(
        document.unknown_styles()[1].name(),
        Some(rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH)
    );
    assert_eq!(document.unknown_styles()[1].payload(), &[1, 2, 3]);
}

#[test]
fn document_core_reports_preserved_style_stream_sources() {
    let ssmg_style = ssmg_style_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH, &ssmg_style),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    let document_info = core.get_document_info();
    assert!(document_info.contains("\"styleStreamCount\":1"));
    assert!(document_info.contains("\"textCountRangeCount\":0"));
    assert!(document_info.contains("\"styleCandidateCount\":0"));
    assert!(document_info.contains("\"styleCandidateNames\":[]"));
    assert!(document_info.contains("\"name\":\"/TextLayoutStyle\""));
    assert!(document_info.contains("\"size\":24"));
    assert!(document_info.contains("\"family\":\"ssmg\""));
    assert!(document_info.contains("\"headerU32Be\":[28,256,32]"));
    assert!(document_info.contains("\"recordLayout\":\"none\""));
    assert!(document_info.contains("\"recordCount\":0"));

    let style_list = core.get_style_list();
    assert!(style_list.contains("\"sourceStreamCount\":1"));

    let style_detail = core.get_style_detail(0).unwrap();
    assert!(style_detail.contains("\"decoded\":false"));
    assert!(style_detail.contains("\"sourceStreams\":["));
    assert!(style_detail.contains("\"name\":\"/TextLayoutStyle\""));
    assert!(style_detail.contains("\"headerU16Be\":[1,2]"));
    assert!(style_detail.contains("\"records\":[]"));
}

#[test]
fn document_core_reports_preserved_style_subrecords() {
    let page_style = ssmg_page_layout_style_with_subrecords_fixture();
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (rjtd_core::style_stream::PAGE_LAYOUT_STYLE_PATH, &page_style),
    ]);
    let core = DocumentCore::from_bytes(&bytes).unwrap();

    let document_info = core.get_document_info();
    assert!(document_info.contains("\"name\":\"/PageLayoutStyle\""));
    assert!(document_info.contains("\"recordCount\":1"));
    assert!(document_info.contains("\"subrecordCount\":6"));
    assert!(document_info.contains("\"codeHex\":\"0x3105\""));
    assert!(document_info.contains("\"codeHex\":\"0x3205\""));
    assert!(document_info.contains("\"codeHex\":\"0x3305\""));
    assert!(document_info.contains("\"payloadHex\":\"0400\""));
    assert!(document_info.contains("\"decoded\":false"));
}

#[test]
fn document_core_reports_text_style_label_candidates() {
    let ssmg_style = ssmg_style_with_label_fixture("本文");
    let bytes = cfb_with_streams(&[
        ("/DocumentText", &document_text_fixture()),
        (rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH, &ssmg_style),
    ]);
    let mut core = DocumentCore::from_bytes(&bytes).unwrap();

    let document_info = core.get_document_info();
    assert!(document_info.contains("\"styleCandidateCount\":1"));
    assert!(document_info.contains("\"styleCandidateNames\":[\"本文\"]"));

    let style_list = core.get_style_list();
    assert!(style_list.contains("\"candidateCount\":1"));
    assert!(style_list.contains("\"id\":1"));
    assert!(style_list.contains("\"name\":\"本文\""));
    assert!(style_list.contains("\"jtdCandidate\":true"));
    assert!(style_list.contains("\"sourceStream\":\"/TextLayoutStyle\""));
    assert!(style_list.contains("\"sourceOffset\":276"));
    assert!(style_list.contains("\"sourceCodeHex\":\"0x5555\""));

    let style_detail = core.get_style_detail(1).unwrap();
    assert!(style_detail.contains("\"name\":\"本文\""));
    assert!(style_detail.contains("\"decoded\":false"));
    assert!(style_detail.contains("\"charProps\":"));
    assert!(style_detail.contains("\"paraProps\":"));
    assert_eq!(
        core.get_style_at(0, 0).unwrap(),
        "{\"id\":0,\"name\":\"Normal\"}"
    );

    let applied = core.apply_style(0, 0, 1).unwrap();
    assert!(applied.contains("\"ok\":true"));
    assert!(applied.contains("\"decoded\":false"));
    assert!(applied.contains("\"styleId\":1"));

    let style_at = core.get_style_at(0, 0).unwrap();
    assert!(style_at.contains("\"id\":1"));
    assert!(style_at.contains("\"name\":\"本文\""));
    assert!(style_at.contains("\"jtdCandidate\":true"));

    let first_paragraph = match &core.document().blocks()[0] {
        Block::Paragraph(paragraph) => paragraph,
        Block::Unknown(_) => panic!("expected first block to be a paragraph"),
    };
    assert_eq!(first_paragraph.style().map(StyleRef::id), Some("1"));

    core.split_paragraph(0, 0, 1).unwrap();
    let split_style = core.get_style_at(0, 1).unwrap();
    assert!(split_style.contains("\"id\":1"));
    assert!(split_style.contains("\"name\":\"本文\""));

    assert_eq!(core.apply_style(0, 1, 0).unwrap(), "{\"ok\":true}");
    assert_eq!(
        core.get_style_at(0, 1).unwrap(),
        "{\"id\":0,\"name\":\"Normal\"}"
    );
}

fn document_text_fixture() -> Vec<u8> {
    document_text_fixture_for("銀河")
}

#[test]
fn page_output_preflight_matches_standard_pagination_shape() {
    let documents = [
        Document::from_plain_text(""),
        Document::from_plain_text("single line"),
        Document::from_plain_text("first\n\nthird"),
        Document::from_plain_text(&"x".repeat(200)),
    ];

    for document in documents {
        let pages =
            paginate_document_text(&document, PageLayout::default(), WritingMode::Horizontal);
        let shape =
            page_output_shape(&document, PageLayout::default(), WritingMode::Horizontal).unwrap();
        assert_eq!(shape.pages, pages.len());
        assert_eq!(shape.lines, pages.iter().map(Vec::len).sum());
    }
}

#[test]
fn front_matter_projection_preflight_reserves_additional_page_headroom() {
    let document = Document::from_plain_text(
        "宮沢賢治 銀河鉄道の夜\n目次\n第一章\n銀河鉄道の夜\n一、午后の授業\n本文",
    );
    let normal =
        page_output_shape(&document, PageLayout::default(), WritingMode::VerticalRl).unwrap();
    let construction =
        page_construction_shape(&document, PageLayout::default(), WritingMode::VerticalRl).unwrap();

    assert_eq!(
        ginga_front_matter_indices_in_document(&document),
        ginga_front_matter_indices(&document_paragraph_texts(&document))
    );
    assert!(construction.pages > normal.pages);
    assert!(construction.lines > normal.lines);
}

#[test]
fn front_matter_projection_does_not_charge_characters_as_pages() {
    let text = format!(
        "宮沢賢治 銀河鉄道の夜\n目次\n第一章\n銀河鉄道の夜\n一、午后の授業\n{}",
        "本".repeat(17_000)
    );
    let core = DocumentCore::from_document_with_limits(
        Document::from_plain_text(&text),
        ParseLimits::DEFAULT,
    )
    .unwrap();

    assert_eq!(core.writing_mode(), WritingMode::VerticalRl);
    assert!(core.page_count() < 1_000);
}

fn document_text_fixture_for(text: &str) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x1f]);
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn paper_mark_fixture(entries: &[(u32, u32)]) -> Vec<u8> {
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

fn visual_list_bmdv_fixture() -> Vec<u8> {
    let rle = [0x0a, 0x11, 0x00, 0x00, 0x0a, 0x22, 0x00, 0x00];
    let mut bytes = vec![0; VISUAL_LIST_HEADER_BYTES];
    let declared_size = VISUAL_LIST_HEADER_BYTES + rle.len();
    bytes[0..4].copy_from_slice(&(declared_size as u32).to_be_bytes());
    bytes[VISUAL_LIST_MAGIC_OFFSET..VISUAL_LIST_MAGIC_OFFSET + VISUAL_LIST_MAGIC.len()]
        .copy_from_slice(VISUAL_LIST_MAGIC);
    bytes[VISUAL_LIST_VERSION_OFFSET..VISUAL_LIST_VERSION_OFFSET + 4]
        .copy_from_slice(&1u32.to_be_bytes());
    bytes[VISUAL_LIST_FLAGS_OFFSET..VISUAL_LIST_FLAGS_OFFSET + 4]
        .copy_from_slice(&0x0001_0100u32.to_be_bytes());
    bytes[VISUAL_LIST_WIDTH_OFFSET..VISUAL_LIST_WIDTH_OFFSET + 4]
        .copy_from_slice(&10u32.to_be_bytes());
    bytes[VISUAL_LIST_HEIGHT_OFFSET..VISUAL_LIST_HEIGHT_OFFSET + 4]
        .copy_from_slice(&2u32.to_be_bytes());
    bytes[VISUAL_LIST_ROW_STRIDE_OFFSET..VISUAL_LIST_ROW_STRIDE_OFFSET + 4]
        .copy_from_slice(&10u32.to_be_bytes());
    bytes[VISUAL_LIST_BIT_DEPTH_OFFSET..VISUAL_LIST_BIT_DEPTH_OFFSET + 4]
        .copy_from_slice(&8u32.to_be_bytes());
    bytes[VISUAL_LIST_X_PPM_OFFSET..VISUAL_LIST_X_PPM_OFFSET + 4]
        .copy_from_slice(&3779u32.to_be_bytes());
    bytes[VISUAL_LIST_Y_PPM_OFFSET..VISUAL_LIST_Y_PPM_OFFSET + 4]
        .copy_from_slice(&3779u32.to_be_bytes());
    bytes[VISUAL_LIST_RLE_LENGTH_OFFSET..VISUAL_LIST_RLE_LENGTH_OFFSET + 4]
        .copy_from_slice(&(rle.len() as u32).to_be_bytes());
    bytes.extend_from_slice(&rle);
    bytes
}

fn embedding_info_fixture() -> Vec<u8> {
    let class_name = "JSFart.Art.2";
    let mut class_bytes = Vec::new();
    for unit in class_name.encode_utf16() {
        class_bytes.extend_from_slice(&unit.to_le_bytes());
    }
    class_bytes.extend_from_slice(&0u16.to_le_bytes());

    let mut bytes = vec![0; EMBEDDING_INFO_HEADER_BYTES];
    bytes[0..4].copy_from_slice(&1u32.to_le_bytes());
    let row_start = bytes.len();
    bytes.resize(row_start + EMBEDDING_INFO_CLASS_START_OFFSET, 0);
    bytes[row_start + EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET
        ..row_start + EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET + 4]
        .copy_from_slice(&24u32.to_le_bytes());
    bytes[row_start + EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET
        ..row_start + EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET + 2]
        .copy_from_slice(&13260u16.to_le_bytes());
    bytes[row_start + EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET
        ..row_start + EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET + 2]
        .copy_from_slice(&1327u16.to_le_bytes());
    bytes[row_start + EMBEDDING_INFO_CLASS_LENGTH_OFFSET
        ..row_start + EMBEDDING_INFO_CLASS_LENGTH_OFFSET + 4]
        .copy_from_slice(&(class_bytes.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&class_bytes);

    let trailing_start = bytes.len();
    bytes.resize(trailing_start + EMBEDDING_INFO_TRAILING_BYTES, 0);
    bytes[trailing_start + EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET
        ..trailing_start + EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET + 4]
        .copy_from_slice(&1u32.to_le_bytes());
    bytes[trailing_start + EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET
        ..trailing_start + EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET + 4]
        .copy_from_slice(&13260u32.to_le_bytes());
    bytes[trailing_start + EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET
        ..trailing_start + EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET + 4]
        .copy_from_slice(&1327u32.to_le_bytes());
    bytes
}

fn embedded_press_snapshot_fixture(
    width: u32,
    height: u32,
    body_length: u32,
    payload_length: u32,
) -> Vec<u8> {
    let mut bytes = vec![0; 0x80];
    bytes[..EMBEDDED_PRESS_SNAPSHOT_MAGIC.len()].copy_from_slice(EMBEDDED_PRESS_SNAPSHOT_MAGIC);
    bytes[0x0c..0x10].copy_from_slice(&[0x00, 0xd5, 0xf6, 0x77]);
    bytes[0x10..0x14].copy_from_slice(&u32::MAX.to_le_bytes());
    bytes[0x20..0x24].copy_from_slice(&32u32.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET
        ..EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET + 4]
        .copy_from_slice(&body_length.to_le_bytes());
    bytes[0x28..0x2c].copy_from_slice(&65536u32.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET..EMBEDDED_PRESS_SNAPSHOT_FORMAT_OFFSET + 4]
        .copy_from_slice(b"GCI\0");
    bytes[EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET
        ..EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET + 4]
        .copy_from_slice(&17u32.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET
        ..EMBEDDED_PRESS_SNAPSHOT_OBJECT_TABLE_OFFSET + 4]
        .copy_from_slice(&74u32.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET
        ..EMBEDDED_PRESS_SNAPSHOT_PAYLOAD_LENGTH_OFFSET + 4]
        .copy_from_slice(&payload_length.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET..EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET + 4]
        .copy_from_slice(&width.to_le_bytes());
    bytes[EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET..EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET + 4]
        .copy_from_slice(&height.to_le_bytes());
    bytes[0x50..0x54].copy_from_slice(&100u32.to_le_bytes());
    bytes[0x54..0x58].copy_from_slice(&1u32.to_le_bytes());
    bytes[0x58..0x5c].copy_from_slice(&100u32.to_le_bytes());
    bytes[0x5c..0x60].copy_from_slice(&1u32.to_le_bytes());
    bytes[0x60..0x64].copy_from_slice(&4u32.to_le_bytes());
    bytes
}

fn frame_stream_fixture() -> Vec<u8> {
    let mut bytes = vec![0; FRAME_RECORD_HEADER_BYTES];
    bytes[FRAME_RECORD_DECLARED_COUNT_OFFSET..FRAME_RECORD_DECLARED_COUNT_OFFSET + 2]
        .copy_from_slice(&2u16.to_be_bytes());
    bytes.resize(FRAME_RECORD_HEADER_BYTES + FRAME_RECORD_BYTES, 0);

    let row_start = FRAME_RECORD_HEADER_BYTES + FRAME_RECORD_BYTES;
    bytes.resize(row_start + FRAME_RECORD_BYTES, 0);
    bytes[row_start..row_start + 2].copy_from_slice(&0x1001u16.to_be_bytes());
    bytes[row_start + 2..row_start + 4].copy_from_slice(&60u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_ID_OFFSET..row_start + FRAME_RECORD_ID_OFFSET + 2]
        .copy_from_slice(&24u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_TYPE_OFFSET..row_start + FRAME_RECORD_TYPE_OFFSET + 2]
        .copy_from_slice(&0x0002u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_X_OFFSET..row_start + FRAME_RECORD_X_OFFSET + 2]
        .copy_from_slice(&2143u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_Y_OFFSET..row_start + FRAME_RECORD_Y_OFFSET + 2]
        .copy_from_slice(&2932u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_WIDTH_OFFSET..row_start + FRAME_RECORD_WIDTH_OFFSET + 2]
        .copy_from_slice(&13260u16.to_be_bytes());
    bytes[row_start + FRAME_RECORD_HEIGHT_OFFSET..row_start + FRAME_RECORD_HEIGHT_OFFSET + 2]
        .copy_from_slice(&1327u16.to_be_bytes());
    bytes
}

fn font_stream_fixture(entries: &[(u16, &str, usize)]) -> Vec<u8> {
    let mut bytes = b"FontV.01".to_vec();
    bytes.extend_from_slice(&(entries.len() as u16).to_be_bytes());
    for (id, name, suffix_len) in entries {
        bytes.extend_from_slice(&font_entry_fixture(*id, name, *suffix_len));
    }
    bytes
}

fn font_entry_fixture(id: u16, name: &str, suffix_len: usize) -> Vec<u8> {
    let mut entry = vec![0; 30];
    entry[0..2].copy_from_slice(&id.to_be_bytes());
    entry[20..22].copy_from_slice(&0x0190u16.to_be_bytes());
    for unit in name.encode_utf16() {
        entry.extend_from_slice(&unit.to_be_bytes());
    }
    entry.extend_from_slice(&[0, 0]);
    entry.resize(entry.len() + suffix_len, 0);
    entry
}

fn minimal_jpeg_payload() -> &'static [u8] {
    &[
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x10,
        0x00, 0x20, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff, 0xda, 0x00,
        0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x00, 0xff, 0xd9,
    ]
}

#[cfg(feature = "bitmap-images")]
fn minimal_png_payload() -> &'static [u8] {
    &[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8,
        0xcf, 0xc0, 0x00, 0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xdd, 0x8d, 0xb0, 0x00, 0x00, 0x00,
        0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ]
}

fn image_payload_with_header_fixture(payload_len: usize) -> (Vec<u8>, usize, usize) {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&9_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&0x1234_u32.to_le_bytes());
    bytes.extend_from_slice(&0x5678_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());

    let source_path = b"C:\\TEMP\\A.JPG";
    bytes.push(source_path.len() as u8);
    bytes.extend_from_slice(source_path);
    bytes.push(0);
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.extend_from_slice(&(payload_len as u32).to_le_bytes());

    let signature_offset = bytes.len();
    (bytes, signature_offset, signature_offset + payload_len)
}

fn document_text_with_control_boundary() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(
        &mut bytes,
        &[
            0x001f, 0x9280, 0x6cb3, 0x001c, 0x001f, 0x9244, 0x9053, 0x000a,
        ],
    );
    bytes
}

fn document_text_with_page_break() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    for unit in "銀河鉄道の夜\t\t\t\t宮沢 賢治".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(&mut bytes, &[DOCUMENT_TEXT_PAGE_BREAK_CONTROL, 0x001f]);
    for unit in "目次".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn document_text_with_sparse_table_rows() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    append_sparse_table_row(&mut bytes, &["", "", "(1)表面積", ""]);
    append_sparse_table_row(&mut bytes, &["", "１", "", ""]);
    append_sparse_table_row(&mut bytes, &["", "ＡＢ　＝　ｃｍ", ""]);
    append_sparse_table_row(&mut bytes, &["", "ＡＣ　＝　ｃｍ", ""]);
    bytes
}

fn document_text_with_two_row_control_table() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    append_sparse_table_row(&mut bytes, &["R01C01", "R01C02", "R01C03"]);
    append_sparse_table_row(&mut bytes, &["R02C01", "R02C02", "R02C03"]);
    bytes
}

fn document_text_with_table_row_gap(empty_rows: usize) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    for row_index in 1..=3 {
        append_sparse_table_row(
            &mut bytes,
            &[
                &format!("A{row_index}C01"),
                &format!("A{row_index}C02"),
                &format!("A{row_index}C03"),
            ],
        );
    }
    for _ in 0..empty_rows {
        append_empty_table_row(&mut bytes);
    }
    for row_index in 1..=3 {
        append_sparse_table_row(
            &mut bytes,
            &[
                &format!("B{row_index}C01"),
                &format!("B{row_index}C02"),
                &format!("B{row_index}C03"),
            ],
        );
    }
    bytes
}

fn append_empty_table_row(bytes: &mut Vec<u8>) {
    extend_units(bytes, &[TABLE_ROW_DELIMITER_CONTROL]);
}

fn append_sparse_table_row(bytes: &mut Vec<u8>, cells: &[&str]) {
    for (cell_index, cell) in cells.iter().enumerate() {
        if cell_index > 0 {
            extend_units(bytes, &[TABLE_CELL_DELIMITER_CONTROL, 0x001f]);
        } else if !cell.is_empty() {
            extend_units(bytes, &[0x001f]);
        }
        if !cell.is_empty() {
            for unit in cell.encode_utf16() {
                bytes.extend_from_slice(&unit.to_be_bytes());
            }
        }
    }
    extend_units(bytes, &[TABLE_ROW_DELIMITER_CONTROL]);
}

fn layout_box_text_plain_block_fixture(text: &str) -> Vec<u8> {
    let units = text.encode_utf16().collect::<Vec<_>>();
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.resize(20, 0);
    bytes.extend_from_slice(LAYOUT_BOX_TEXT_MAGIC);
    bytes.extend_from_slice(&(units.len() as u32).to_be_bytes());
    for unit in units {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn layout_box_text_position_tables_fixture() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.resize(20, 0);
    bytes.extend_from_slice(b"TCntV.01");
    bytes.resize(280, 0);
    bytes
}

fn layout_box_record_fixture(x_pt: u16, y_pt: u16, width_pt: u16) -> Vec<u8> {
    let mut bytes = vec![0; 128];
    bytes[0..LAYOUT_BOX_RECORD_PREFIX.len()].copy_from_slice(LAYOUT_BOX_RECORD_PREFIX);
    bytes[LAYOUT_BOX_RECORD_ORIGIN_FIELD_OFFSET..LAYOUT_BOX_RECORD_ORIGIN_FIELD_OFFSET + 2]
        .copy_from_slice(&41u16.to_be_bytes());
    bytes[LAYOUT_BOX_RECORD_X_FIELD_OFFSET..LAYOUT_BOX_RECORD_X_FIELD_OFFSET + 2]
        .copy_from_slice(&x_pt.to_be_bytes());
    bytes[LAYOUT_BOX_RECORD_Y_FIELD_OFFSET..LAYOUT_BOX_RECORD_Y_FIELD_OFFSET + 2]
        .copy_from_slice(&y_pt.to_be_bytes());
    bytes[LAYOUT_BOX_RECORD_WIDTH_FIELD_OFFSET..LAYOUT_BOX_RECORD_WIDTH_FIELD_OFFSET + 2]
        .copy_from_slice(&width_pt.to_be_bytes());
    bytes
}

fn document_view_styles_page_size_fixture(width_mm100: u32, height_mm100: u32) -> Vec<u8> {
    let mut bytes = vec![0; 32];
    bytes[0..4].copy_from_slice(&0x0001_0002_u32.to_be_bytes());
    bytes[4..8].copy_from_slice(&0x1000_0000_u32.to_be_bytes());
    bytes[8..12].copy_from_slice(&0x040e_1001_u32.to_be_bytes());
    bytes[12..16].copy_from_slice(&0x010a_0600_u32.to_be_bytes());
    bytes[16..20].copy_from_slice(&(width_mm100 << 8).to_be_bytes());
    bytes[20..24].copy_from_slice(&((height_mm100 << 8) | 0x04).to_be_bytes());
    bytes
}

fn page_layout_style_page_size_fixture(width_mm100: u32, height_mm100: u32) -> Vec<u8> {
    let mut bytes = ssmg_style_fixture();
    bytes.resize(0x114, 0);
    let mut payload = Vec::new();
    payload.extend_from_slice(&0u16.to_be_bytes());
    payload.extend_from_slice(&[0, 0]);
    let mut page_size_payload = vec![0; PAGE_LAYOUT_STYLE_PAGE_SIZE_HEIGHT_OFFSET + 4];
    page_size_payload
        [PAGE_LAYOUT_STYLE_PAGE_SIZE_WIDTH_OFFSET..PAGE_LAYOUT_STYLE_PAGE_SIZE_WIDTH_OFFSET + 4]
        .copy_from_slice(&(width_mm100 << 8).to_be_bytes());
    page_size_payload
        [PAGE_LAYOUT_STYLE_PAGE_SIZE_HEIGHT_OFFSET..PAGE_LAYOUT_STYLE_PAGE_SIZE_HEIGHT_OFFSET + 4]
        .copy_from_slice(&((height_mm100 << 8) | 0x04).to_be_bytes());
    payload.extend_from_slice(&PAGE_LAYOUT_STYLE_PAGE_SIZE_SUBRECORD_CODE.to_be_bytes());
    payload.extend_from_slice(&(page_size_payload.len() as u16).to_be_bytes());
    payload.extend_from_slice(&page_size_payload);
    for code in [0x4002u16, 0x4006] {
        payload.extend_from_slice(&code.to_be_bytes());
        payload.extend_from_slice(&1u16.to_be_bytes());
        payload.push(0);
    }

    bytes.extend_from_slice(&PAGE_LAYOUT_STYLE_RECORD_CODE.to_be_bytes());
    bytes.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    bytes.extend_from_slice(&payload);
    bytes
}

fn text_count_table_fixture() -> Vec<u8> {
    text_count_table_fixture_with_ranges(&[(0x1234, 0x1250), (0x2000, 0x2400)])
}

fn text_count_table_fixture_with_ranges(entries: &[(u32, u32)]) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    bytes.extend_from_slice(&[0x00, 0x00, 0x01, 0x00]);
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    bytes.extend_from_slice(b"TCntV.01");
    bytes.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]);
    bytes.extend_from_slice(&(entries.len() as u16).to_be_bytes());
    bytes.extend_from_slice(&[0x00, 0x24]);
    for (index, (start, end)) in entries.iter().enumerate() {
        let mut entry = [0; 29];
        entry[0..4].copy_from_slice(&start.to_be_bytes());
        entry[4..8].copy_from_slice(&end.to_be_bytes());
        entry[8..12].copy_from_slice(&[0x01 + index as u8, 0x01 + index as u8, 0x00, 0x05]);
        bytes.extend_from_slice(&entry);
    }
    bytes
}

fn line_mark_words_0_to_20() -> Vec<u8> {
    let mut bytes = Vec::new();
    for word in 0..20u16 {
        bytes.extend_from_slice(&word.to_be_bytes());
    }
    bytes
}

fn page_mark_fields_0_to_20() -> Vec<u8> {
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

fn ssmg_style_fixture() -> Vec<u8> {
    vec![
        b'S', b's', b'm', b'g', b'V', b'.', b'0', b'1', 0, 0, 0, 0x1c, 0, 0, 1, 0, 0, 0, 0, 0x20,
        0, 1, 0, 2,
    ]
}

fn ssmg_style_with_label_fixture(label: &str) -> Vec<u8> {
    let mut bytes = ssmg_style_fixture();
    bytes.resize(0x114, 0);
    let label_units = label.encode_utf16().collect::<Vec<_>>();
    let payload_len = 2 + label_units.len() * 2;
    bytes.extend_from_slice(&0x5555u16.to_be_bytes());
    bytes.extend_from_slice(&(payload_len as u16).to_be_bytes());
    bytes.extend_from_slice(&(label_units.len() as u16).to_be_bytes());
    for unit in label_units {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn ssmg_page_layout_style_with_subrecords_fixture() -> Vec<u8> {
    let mut bytes = ssmg_style_fixture();
    bytes.resize(0x114, 0);
    let label_units = "ページ".encode_utf16().collect::<Vec<_>>();
    let mut payload = Vec::new();
    payload.extend_from_slice(&(label_units.len() as u16).to_be_bytes());
    for unit in label_units {
        payload.extend_from_slice(&unit.to_be_bytes());
    }
    payload.extend_from_slice(&[0, 0]);
    payload.extend_from_slice(&[0x31, 0x04, 0, 1, 0xaa]);
    payload.extend_from_slice(&[0x31, 0x05, 0, 2, 0x04, 0x00]);
    payload.extend_from_slice(&[0x31, 0x06, 0, 1, 0xbb]);
    payload.extend_from_slice(&[0x31, 0x07, 0, 1, 0xcc]);
    payload.extend_from_slice(&[0x32, 0x05, 0, 2, 0x04, 0x00]);
    payload.extend_from_slice(&[0x33, 0x05, 0, 2, 0x04, 0x00]);

    bytes.extend_from_slice(&0x4444u16.to_be_bytes());
    bytes.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    bytes.extend_from_slice(&payload);
    bytes
}

fn auto_text_info_fixture(text: &str) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.resize(84, 0);
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn document_text_with_inline() -> Vec<u8> {
    let mut bytes = vec![0x00, 0x1f];
    for unit in "一、".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(
        &mut bytes,
        &[0x001c, 0x0001, 0x0007, 0x0000, 0x0000, 0x0003, 0x001d],
    );
    for unit in "午后".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(&mut bytes, &[0x001e, 0x0005, 0x0000, 0x0001, 0x001f]);
    for unit in "の授業\n二、".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn document_text_with_skipped_inline() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x1f]);
    for unit in "本文".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(
        &mut bytes,
        &[0x001c, 0x0001, 0x0007, 0x0000, 0x0001, 0x0082, 0x001d],
    );
    for unit in "ふりがな".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(&mut bytes, &[0x001e]);
    bytes
}

fn document_text_with_ruby() -> Vec<u8> {
    let mut bytes = vec![0x00, 0x1f];
    for unit in "一、".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(
        &mut bytes,
        &[0x001c, 0x0001, 0x0007, 0x0000, 0x0000, 0x0003, 0x001d],
    );
    for unit in "午后".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(&mut bytes, &[0x001e, 0x0005, 0x0000, 0x0001, 0x001f]);
    extend_units(
        &mut bytes,
        &[0x001c, 0x0001, 0x0007, 0x0000, 0x0001, 0x0082, 0x001d],
    );
    for unit in "ごご".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(&mut bytes, &[0x001e, 0x0005, 0x0000, 0x0001, 0x001f]);
    for unit in "の授業".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn assert_text_inline(inline: &Inline, expected: &str) {
    match inline {
        Inline::Text(text) => assert_eq!(text.text(), expected),
        Inline::Ruby(_) => panic!("expected text inline"),
        Inline::Unknown(_) => panic!("expected text inline"),
    }
}

fn assert_ruby_inline(inline: &Inline, expected_base: &str, expected_annotation: &str) {
    match inline {
        Inline::Ruby(ruby) => {
            assert_eq!(ruby.base_text(), expected_base);
            assert_eq!(ruby.annotation_text(), expected_annotation);
            assert_eq!(
                ruby.annotation_source().source().tag(),
                Some(DOCUMENT_TEXT_INLINE_START_TAG)
            );
            assert!(!ruby.annotation_source().payload().is_empty());
        }
        Inline::Text(_) => panic!("expected ruby inline"),
        Inline::Unknown(_) => panic!("expected ruby inline"),
    }
}

fn row_header_record_bytes(fixed_words: [u16; 6], payload_words: &[u16]) -> Vec<u8> {
    let total_len_words = (3 + fixed_words.len() + payload_words.len() + 4) as u16;
    let mut words = vec![0x001c, 0x0010, total_len_words];
    words.extend_from_slice(&fixed_words);
    words.extend_from_slice(payload_words);
    words.extend_from_slice(&[total_len_words, 0x0000, 0x0010, 0x001f]);
    let mut bytes = Vec::new();
    extend_units(&mut bytes, &words);
    bytes
}

fn extend_units(bytes: &mut Vec<u8>, units: &[u16]) {
    for unit in units {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
}

fn cfb_with_document_text(payload: Vec<u8>) -> Vec<u8> {
    cfb_with_streams(&[("/DocumentText", &payload)])
}

fn cfb_with_streams(streams: &[(&str, &[u8])]) -> Vec<u8> {
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

fn push_fdm_index_row(
    bytes: &mut Vec<u8>,
    vector_offset: u32,
    kind: u16,
    bbox: (i32, i32, i32, i32),
) {
    bytes.extend_from_slice(&vector_offset.to_be_bytes());
    bytes.extend_from_slice(&kind.to_be_bytes());
    bytes.extend_from_slice(&bbox.0.to_be_bytes());
    bytes.extend_from_slice(&bbox.1.to_be_bytes());
    bytes.extend_from_slice(&bbox.2.to_be_bytes());
    bytes.extend_from_slice(&bbox.3.to_be_bytes());
}

fn frame_record_fixture(
    object_id: u16,
    object_type: u16,
    geometry: (u16, u16, u16, u16),
) -> Vec<u8> {
    let mut row = vec![0; FRAME_RECORD_BYTES];
    row[0..2].copy_from_slice(&0x0102_u16.to_be_bytes());
    row[2..4].copy_from_slice(&0x0038_u16.to_be_bytes());
    row[FRAME_RECORD_ID_OFFSET..FRAME_RECORD_ID_OFFSET + 2]
        .copy_from_slice(&object_id.to_be_bytes());
    row[FRAME_RECORD_TYPE_OFFSET..FRAME_RECORD_TYPE_OFFSET + 2]
        .copy_from_slice(&object_type.to_be_bytes());
    row[FRAME_RECORD_X_OFFSET..FRAME_RECORD_X_OFFSET + 2]
        .copy_from_slice(&geometry.0.to_be_bytes());
    row[FRAME_RECORD_Y_OFFSET..FRAME_RECORD_Y_OFFSET + 2]
        .copy_from_slice(&geometry.1.to_be_bytes());
    row[FRAME_RECORD_WIDTH_OFFSET..FRAME_RECORD_WIDTH_OFFSET + 2]
        .copy_from_slice(&geometry.2.to_be_bytes());
    row[FRAME_RECORD_HEIGHT_OFFSET..FRAME_RECORD_HEIGHT_OFFSET + 2]
        .copy_from_slice(&geometry.3.to_be_bytes());
    row
}

fn create_parent_storages(
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
