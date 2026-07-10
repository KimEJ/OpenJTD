use std::path::{Path, PathBuf};
use std::process::Command;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("rjtd-testdata")
        .join("local-samples")
        .join("ichitaro-source-y-probe")
}

fn baseline_sweep_dir() -> PathBuf {
    corpus_dir().join("corpus").join("baseline-sweep")
}

fn page01_grid_dir() -> PathBuf {
    corpus_dir().join("corpus").join("page01-grid")
}

fn assert_contains(stdout: &str, expected: &str) {
    assert!(
        stdout.contains(expected),
        "missing {expected:?}\nstdout:\n{stdout}"
    );
}

fn assert_source_only_page_y_admission_blocked(layer_tree: &str) {
    assert_contains(
        layer_tree,
        "\"sourceOnlyPageYAdmissionClass\":\"flow-y-stride-only-diagnostic\"",
    );
    assert_contains(
        layer_tree,
        "\"sourceOnlyPageYRenderAdmissionGate\":{\"source\":\"sourcePageYTransformGate source-only page-y render admission gate\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"admissionReady\":false",
    );
    assert_contains(
        layer_tree,
        "\"pageOriginAuthority\":\"fallbackTextAnchors\"",
    );
    assert_contains(
        layer_tree,
        "\"renderPromotionBlockedReason\":\"source-page-y-render-admission-not-ready\"",
    );
}

fn require_local_fixture(path: &Path) {
    assert!(
        path.exists(),
        "missing local probe fixture: {}. These tests are ignored by default; run them with --ignored only when ichitaro-source-y-probe is present.",
        path.display()
    );
}

fn run_rjtd(args: &[&str], path: &Path) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .args(args)
        .arg(path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
#[ignore = "requires local ichitaro-source-y-probe corpus"]
fn local_source_y_probe_audit_classifies_updated_probe_corpus_when_available() {
    let corpus_dir = corpus_dir();
    require_local_fixture(&corpus_dir);

    let stdout = run_rjtd(&["source-y-probe-audit"], &corpus_dir);
    for expected in [
        "summary\tcases=40\tcreated=39\tfailed=1\tomitted=0\tmissing-pairs=0",
        "compare\t040a_top_margin_20mm\tbase=040b_top_margin_30mm_baseline\tbasePolicy=rtf-margin-sweep-baseline\tsourceSignatureSame=true",
        "compare\t040c_top_margin_40mm\tbase=040b_top_margin_30mm_baseline\tbasePolicy=rtf-margin-sweep-baseline\tsourceSignatureSame=true",
        "validation\t010_table_moved_down_small\tintended=table-y-position\tresult=pass-diagnostic-signal",
        "validation\t011_table_moved_down_large\tintended=table-y-position\tresult=pass-diagnostic-signal",
        "validation\t013_table_moved_right\tintended=table-x-position\tresult=source-silent-reference-visible-or-unproven",
        "validation\t030_col1_width_plus\tintended=table-column-width\tresult=pass-diagnostic-signal",
        "validation\t032_table_width_plus_both_cols\tintended=table-width\tresult=pass-diagnostic-signal",
        "validation\t064_merged_header\tintended=merged-header\tresult=pass-diagnostic-signal",
        "validation\t074d_many_row_table_2col_simple\tintended=multi-page-table\tresult=pass-diagnostic-signal",
        "admission\tready=false\treason=diagnostic-only-corpus-insufficient-for-source-only-page-y-render-admission",
    ] {
        assert_contains(&stdout, expected);
    }
}

#[test]
#[ignore = "requires local ichitaro-source-y-probe corpus"]
fn local_source_y_probe_compare_reports_line_and_page_tuple_deltas_when_available() {
    let baseline_sweep_dir = baseline_sweep_dir();
    let base = baseline_sweep_dir.join("000_base_a.jtd");
    let row_height = baseline_sweep_dir.join("020_row1_height_plus.jtd");
    let top_margin = baseline_sweep_dir.join("040_top_margin_plus.jtd");
    require_local_fixture(&base);
    require_local_fixture(&row_height);
    require_local_fixture(&top_margin);

    let row_stdout = compare_stdout(&base, &row_height);
    assert_contains(
        &row_stdout,
        "line-summary\tbaseDeclared=9\tcandidateDeclared=11\tbaseParsed=8\tcandidateParsed=10\tlineSignatureSame=false",
    );
    assert_contains(
        &row_stdout,
        "line-delta-diff\trecord=2\tstatus=changed\tbaseDelta=52\tcandidateDelta=76",
    );
    assert_contains(
        &row_stdout,
        "admission\tready=false\treason=direct-source-diff-diagnostic-only",
    );

    let margin_stdout = compare_stdout(&base, &top_margin);
    assert_contains(
        &margin_stdout,
        "line-summary\tbaseDeclared=9\tcandidateDeclared=9\tbaseParsed=8\tcandidateParsed=8\tlineSignatureSame=true",
    );
    assert_contains(
        &margin_stdout,
        "page-summary\tbaseFamily=fixed84\tcandidateFamily=fixed84\tbaseEntries=3\tcandidateEntries=3\tpageTupleSignatureSame=false",
    );
    assert_contains(
        &margin_stdout,
        "page-tuple-diff\tentry=0\tclass=additive-boundary\tword=w14\tbase=222\tcandidate=232\tstatus=changed",
    );
    assert_contains(
        &margin_stdout,
        "page-tuple-diff\tentry=0\tclass=additive-boundary\tword=w21\tbase=592\tcandidate=602\tstatus=changed",
    );
}

#[test]
#[ignore = "requires local ichitaro-source-y-probe page01-grid corpus"]
fn local_new_probe_baseline_reports_document_text_table_candidate_when_available() {
    let baseline = page01_grid_dir().join("PAGE 01.jtd");
    require_local_fixture(&baseline);

    let stdout = table_candidates_stdout(&baseline);
    assert_contains(
        &stdout,
        "kind=documentTextControlRunTableCandidate\trange=-\tboundary=-\tbasis=unit\tdelimiter=0x000e\tintervals=2",
    );
    assert_contains(&stdout, "\tcells=6/0/6\tmax-columns=3\t");
    assert_contains(&stdout, "text=R01C01\\tR01C02\\tR01C03");
    assert_contains(&stdout, "text=R02C01\\tR02C02\\tR02C03");
}

#[test]
#[ignore = "requires local ichitaro-source-y-probe page01-grid corpus"]
fn local_new_probe_compare_reports_table_line_header_x_shift_when_available() {
    let page01_grid_dir = page01_grid_dir();
    let baseline = page01_grid_dir.join("PAGE 01.jtd");
    let right_1 = page01_grid_dir.join("PAGE 01_right_1Tick.jtd");
    let right_4 = page01_grid_dir.join("PAGE 01_right_4Tick.jtd");
    require_local_fixture(&baseline);
    require_local_fixture(&right_1);
    require_local_fixture(&right_4);

    let right_1_stdout = compare_stdout(&baseline, &right_1);
    assert_contains(
        &right_1_stdout,
        "table-line-header-summary\tbaseRows=2\tcandidateRows=2\tbaseFirstCellOffset=2\tcandidateFirstCellOffset=4\tfirstCellOffsetDelta=2",
    );

    let right_4_stdout = compare_stdout(&baseline, &right_4);
    assert_contains(
        &right_4_stdout,
        "table-line-header-summary\tbaseRows=2\tcandidateRows=2\tbaseFirstCellOffset=2\tcandidateFirstCellOffset=10\tfirstCellOffsetDelta=8",
    );
}

#[test]
#[ignore = "requires local ichitaro-source-y-probe page01-grid corpus"]
fn local_new_probe_compare_reports_table_flow_y_shift_when_available() {
    let page01_grid_dir = page01_grid_dir();
    let baseline = page01_grid_dir.join("PAGE 01.jtd");
    let down_1 = page01_grid_dir.join("PAGE 01_down_1Low.jtd");
    let down_4 = page01_grid_dir.join("PAGE 01_down_4Low.jtd");
    require_local_fixture(&baseline);
    require_local_fixture(&down_1);
    require_local_fixture(&down_4);

    let down_1_stdout = compare_stdout(&baseline, &down_1);
    assert_contains(
        &down_1_stdout,
        "table-flow-y-summary\tbaseRows=2\tcandidateRows=2\tbaseLineMarkRecords=2,4\tcandidateLineMarkRecords=3,5\tlineMarkRecordDelta=1\tuniformLineMarkRecordDelta=true\tbaseFirstRowSourceStart=99\tcandidateFirstRowSourceStart=100\tfirstRowSourceStartDelta=1",
    );
    assert_contains(
        &down_1_stdout,
        "table-flow-y-admission-summary\tbaseLineMarkRecordStride=2\tcandidateLineMarkRecordStride=2\tbaseExactSourceRangeMatchCount=0\tcandidateExactSourceRangeMatchCount=0\tbaseRowsExactAndContiguous=false\tcandidateRowsExactAndContiguous=false\tblocker=line-mark-rows-not-exact-source-boundaries",
    );
    assert_contains(
        &down_1_stdout,
        "table-flow-y-hypothesis\tstrideCorrelationObserved=true\ttransformProven=false\trenderAdmissible=false\thypothesis=line-mark-record-stride-correlates-with-flow-y\tblockedReason=line-mark-rows-not-exact-source-boundaries",
    );

    let down_4_stdout = compare_stdout(&baseline, &down_4);
    assert_contains(
        &down_4_stdout,
        "table-flow-y-summary\tbaseRows=2\tcandidateRows=2\tbaseLineMarkRecords=2,4\tcandidateLineMarkRecords=6,8\tlineMarkRecordDelta=4\tuniformLineMarkRecordDelta=true\tbaseFirstRowSourceStart=99\tcandidateFirstRowSourceStart=103\tfirstRowSourceStartDelta=4",
    );
    assert_contains(
        &down_4_stdout,
        "table-flow-y-admission-summary\tbaseLineMarkRecordStride=2\tcandidateLineMarkRecordStride=2\tbaseExactSourceRangeMatchCount=0\tcandidateExactSourceRangeMatchCount=0\tbaseRowsExactAndContiguous=false\tcandidateRowsExactAndContiguous=false\tblocker=line-mark-rows-not-exact-source-boundaries",
    );
    assert_contains(
        &down_4_stdout,
        "table-flow-y-hypothesis\tstrideCorrelationObserved=true\ttransformProven=false\trenderAdmissible=false\thypothesis=line-mark-record-stride-correlates-with-flow-y\tblockedReason=line-mark-rows-not-exact-source-boundaries",
    );
}

#[test]
#[ignore = "requires local ichitaro-source-y-probe page01-grid corpus"]
fn local_new_probe_layer_tree_keeps_flow_y_stride_diagnostic_non_admissible_when_available() {
    let down_4 = page01_grid_dir().join("PAGE 01_down_4Low.jtd");
    require_local_fixture(&down_4);

    let layer_tree = page_layer_tree_stdout(&down_4);
    assert_source_only_page_y_admission_blocked(&layer_tree);
}

#[test]
#[ignore = "requires local ichitaro-source-y-probe page01-grid corpus"]
fn local_new_probe_layer_tree_keeps_right_shift_non_admissible_when_available() {
    let right_4 = page01_grid_dir().join("PAGE 01_right_4Tick.jtd");
    require_local_fixture(&right_4);

    let layer_tree = page_layer_tree_stdout(&right_4);
    assert_contains(
        &layer_tree,
        "\"horizontalSolverReady\":true,\"rowHeightSolverReady\":true,\"yOriginSolverReady\":false",
    );
    assert_contains(
        &layer_tree,
        "\"sourceOnlyAxisAdmissionGate\":{\"source\":\"pageSpaceHorizontalTransformGate+sourcePageYTransformGate source-only selector coupling\",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsedForSelection\":false,\"admissionReady\":false",
    );
    assert_contains(
        &layer_tree,
        "\"blockedReasons\":[\"source-only-horizontal-selector-absent\",\"source-horizontal-axis-not-render-admissible\",\"source-y-origin-selector-single-support-fallback\",\"source-y-axis-not-render-admissible\",\"source-derived-layout-not-renderable\"]",
    );
    assert_source_only_page_y_admission_blocked(&layer_tree);
}

fn compare_stdout(base: &Path, candidate: &Path) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("source-y-probe-compare")
        .arg(base)
        .arg(candidate)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn page_layer_tree_stdout(path: &Path) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-layer-tree")
        .arg(path)
        .arg("0")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn table_candidates_stdout(path: &Path) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-candidates")
        .arg(path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
