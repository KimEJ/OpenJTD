use super::*;
use crate::*;
use rjtd_core::font_stream::FONT_STREAM_PATH;

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

pub(super) fn document_view_styles_sequential_fixture(first_code: u16) -> Vec<u8> {
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

pub(super) fn document_text_fixture() -> Vec<u8> {
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

pub(super) fn document_text_fixture_for(text: &str) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x1f]);
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

pub(super) fn font_stream_fixture(entries: &[(u16, &str, usize)]) -> Vec<u8> {
    let mut bytes = b"FontV.01".to_vec();
    bytes.extend_from_slice(&(entries.len() as u16).to_be_bytes());
    for (id, name, suffix_len) in entries {
        bytes.extend_from_slice(&font_entry_fixture(*id, name, *suffix_len));
    }
    bytes
}

pub(super) fn font_entry_fixture(id: u16, name: &str, suffix_len: usize) -> Vec<u8> {
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

pub(super) fn document_text_with_control_boundary() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(
        &mut bytes,
        &[
            0x001f, 0x9280, 0x6cb3, 0x001c, 0x001f, 0x9244, 0x9053, 0x000a,
        ],
    );
    bytes
}

pub(super) fn document_text_with_page_break() -> Vec<u8> {
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

pub(super) fn document_text_with_two_row_control_table() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    append_sparse_table_row(&mut bytes, &["R01C01", "R01C02", "R01C03"]);
    append_sparse_table_row(&mut bytes, &["R02C01", "R02C02", "R02C03"]);
    bytes
}

pub(super) fn layout_box_text_plain_block_fixture(text: &str) -> Vec<u8> {
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

pub(super) fn layout_box_text_position_tables_fixture() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.resize(20, 0);
    bytes.extend_from_slice(b"TCntV.01");
    bytes.resize(280, 0);
    bytes
}

pub(super) fn layout_box_record_fixture(x_pt: u16, y_pt: u16, width_pt: u16) -> Vec<u8> {
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

pub(super) fn document_view_styles_page_size_fixture(
    width_mm100: u32,
    height_mm100: u32,
) -> Vec<u8> {
    let mut bytes = vec![0; 32];
    bytes[0..4].copy_from_slice(&0x0001_0002_u32.to_be_bytes());
    bytes[4..8].copy_from_slice(&0x1000_0000_u32.to_be_bytes());
    bytes[8..12].copy_from_slice(&0x040e_1001_u32.to_be_bytes());
    bytes[12..16].copy_from_slice(&0x010a_0600_u32.to_be_bytes());
    bytes[16..20].copy_from_slice(&(width_mm100 << 8).to_be_bytes());
    bytes[20..24].copy_from_slice(&((height_mm100 << 8) | 0x04).to_be_bytes());
    bytes
}

pub(super) fn page_layout_style_page_size_fixture(width_mm100: u32, height_mm100: u32) -> Vec<u8> {
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

pub(super) fn text_count_table_fixture() -> Vec<u8> {
    text_count_table_fixture_with_ranges(&[(0x1234, 0x1250), (0x2000, 0x2400)])
}

pub(super) fn text_count_table_fixture_with_ranges(entries: &[(u32, u32)]) -> Vec<u8> {
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

pub(super) fn ssmg_style_fixture() -> Vec<u8> {
    vec![
        b'S', b's', b'm', b'g', b'V', b'.', b'0', b'1', 0, 0, 0, 0x1c, 0, 0, 1, 0, 0, 0, 0, 0x20,
        0, 1, 0, 2,
    ]
}

pub(super) fn ssmg_style_with_label_fixture(label: &str) -> Vec<u8> {
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

pub(super) fn ssmg_page_layout_style_with_subrecords_fixture() -> Vec<u8> {
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

pub(super) fn auto_text_info_fixture(text: &str) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.resize(84, 0);
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

pub(super) fn document_text_with_inline() -> Vec<u8> {
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

pub(super) fn document_text_with_skipped_inline() -> Vec<u8> {
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

pub(super) fn document_text_with_ruby() -> Vec<u8> {
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

pub(super) fn assert_text_inline(inline: &Inline, expected: &str) {
    match inline {
        Inline::Text(text) => assert_eq!(text.text(), expected),
        Inline::Ruby(_) => panic!("expected text inline"),
        Inline::Unknown(_) => panic!("expected text inline"),
    }
}

pub(super) fn assert_ruby_inline(inline: &Inline, expected_base: &str, expected_annotation: &str) {
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

pub(super) fn cfb_with_document_text(payload: Vec<u8>) -> Vec<u8> {
    cfb_with_streams(&[("/DocumentText", &payload)])
}
