use super::*;
use crate::*;
use std::fs;

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
