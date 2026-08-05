use super::*;
use crate::*;

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

#[test]
fn document_core_preserves_tabs_as_visible_svg_spacing() {
    assert_eq!(display_column_width('\t'), APP_TAB_COLUMNS);
    assert_eq!(svg_visual_text("A\tB"), "A　　B");
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
