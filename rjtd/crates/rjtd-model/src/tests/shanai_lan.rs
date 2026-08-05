use super::*;
use crate::*;
use std::fs;

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

pub(super) fn shanai_lan_line_rule_projection_fixture(
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

pub(super) fn shanai_lan_line_rule_fixture(
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
