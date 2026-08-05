use super::super::*;
use super::*;
use crate::*;
use std::fs;

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
