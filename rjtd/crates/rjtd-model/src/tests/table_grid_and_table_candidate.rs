use super::*;
use crate::*;

pub(super) fn test_table_grid_cross_table_row_boundary_offset_table(
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

pub(super) fn test_table_grid_source_unit_to_page_line_index_piecewise_transition(
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

pub(super) fn test_table_grid_cross_table_row_boundary_offset_probe(
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
        best_absolute_y_slot: Some(absolute_y_slot_candidate.clone()),
        residual_px: Some(107.539),
        agrees: false,
        field_quantization: None,
    };
    assert_eq!(
        table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(
            &absolute_y_slot_disagreement
        ),
        "line-domain-projection-disagrees-with-page-mark-absolute-y-slot"
    );
    assert!(!absolute_y_slot_disagreement.field_quantization_refutes_page_space_px());
    let no_quantization = absolute_y_slot_disagreement.field_quantization_blocked_reasons();
    assert!(no_quantization.is_empty());

    // A residual small enough to "agree" must still not unlock the slot while the
    // field is 256-quantized and repeats per raw record instead of per row.
    let quantized_but_agreeing = TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
        line_domain_y: Some(768.0),
        selected_span_units: Some(0),
        line_domain_projected_y: Some(768.0),
        candidates: vec![absolute_y_slot_candidate.clone()],
        best_absolute_y_slot: Some(absolute_y_slot_candidate),
        residual_px: Some(0.0),
        agrees: true,
        field_quantization: Some(TableGridSourceOnlyPageMarkFieldQuantization {
            field_index: 2,
            tail_block16_word_index: Some(11),
            quantum_units: 256,
            value_count: 3,
            row_values: vec![768, 256, 768],
            distinct_values: vec![256, 768],
            all_values_multiple_of_quantum: true,
            low_byte_all_zero: true,
            high_byte_values: vec![1, 3],
            raw_record_scan_indexes: vec![2, 6, 2],
            values_constant_per_raw_record_scan_index: true,
            value_row_distinct: false,
            page_space_px_plausible: false,
        }),
    };
    assert!(quantized_but_agreeing.field_quantization_refutes_page_space_px());
    assert!(!quantized_but_agreeing.semantics_ready());
    assert_eq!(
        quantized_but_agreeing.field_quantization_blocked_reasons(),
        vec![
            "page-mark-absolute-y-slot-field-quantized-not-page-space-px",
            "page-mark-absolute-y-slot-field-constant-per-raw-record-not-row-distinct",
        ]
    );
    assert_eq!(
        table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(&quantized_but_agreeing),
        "page-mark-absolute-y-slot-field-quantized-not-page-space-px"
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

pub(super) fn document_text_with_sparse_table_rows() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    append_sparse_table_row(&mut bytes, &["", "", "(1)表面積", ""]);
    append_sparse_table_row(&mut bytes, &["", "１", "", ""]);
    append_sparse_table_row(&mut bytes, &["", "ＡＢ　＝　ｃｍ", ""]);
    append_sparse_table_row(&mut bytes, &["", "ＡＣ　＝　ｃｍ", ""]);
    bytes
}

pub(super) fn document_text_with_table_row_gap(empty_rows: usize) -> Vec<u8> {
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

pub(super) fn append_empty_table_row(bytes: &mut Vec<u8>) {
    extend_units(bytes, &[TABLE_ROW_DELIMITER_CONTROL]);
}

pub(super) fn append_sparse_table_row(bytes: &mut Vec<u8>, cells: &[&str]) {
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
