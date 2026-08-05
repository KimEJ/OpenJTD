use std::fs;
use std::process::Command;

use super::support::*;

#[test]
fn text_boundary_candidates_command_reports_model_candidates() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "text-boundary-candidate\t0\tkind=controlDelimitedTextCountRange\trange=0\tbasis=byte\tdelimiter=0x001c\tintervals=1\tinterval-kind=single\tfirst=0\tlast=0\tsource=10-14\tdecoded=false\n",
            "text-boundary-candidate\t1\tkind=controlDelimitedTextCountRange\trange=0\tbasis=unit\tdelimiter=0x001c\tintervals=1\tinterval-kind=single\tfirst=1\tlast=1\tsource=8-11\tdecoded=false\n",
        )
    );
}

#[test]
fn table_candidates_command_reports_model_interval_evidence() {
    let path = text_count_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "table-candidate\t0\tkind=multiIntervalControlRangeTableCandidate\trange=0\tboundary=0\tbasis=byte\tdelimiter=0x001c\tintervals=2\tfirst=0\tlast=1\tsource=10-22\tsparse=false\tcells=0/0/0\tmax-columns=0\tinterval-details=0:source-interval=0,source=10-14,line-breaks=0,text=銀河|1:source-interval=1,source=16-22,line-breaks=0,text=鉄道\tdecoded=false\n",
            "table-candidate\t1\tkind=multiIntervalControlRangeTableCandidate\trange=0\tboundary=1\tbasis=unit\tdelimiter=0x001c\tintervals=2\tfirst=0\tlast=1\tsource=5-11\tsparse=false\tcells=0/0/0\tmax-columns=0\tinterval-details=0:source-interval=0,source=5-7,line-breaks=0,text=銀河|1:source-interval=1,source=8-11,line-breaks=0,text=鉄道\tdecoded=false\n",
        )
    );
}

#[test]
fn table_candidates_command_reports_sparse_document_text_table_evidence() {
    let path = sparse_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "kind=sparseDocumentTextControlRunTableCandidate\trange=-\tboundary=-\tbasis=unit"
    ));
    assert!(stdout.contains("\tsparse=true\tcells=4/10/14\tmax-columns=4\t"));
    assert!(stdout.contains("text=\\t\\t(1)表面積\\t"));
    assert!(stdout.contains("text=\\tＡＢ ＝ ｃｍ\\t"));
}

#[test]
fn table_candidate_context_command_reports_cell_like_shape() {
    let path = text_count_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-candidate-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "table-candidate-context\t0\trange=0\tboundary=0\tbasis=byte\tdelimiter=0x001c\tintervals=2\tsource=10-22\tshape=non-empty=2,empty=0,min-chars=2,max-chars=2,total-chars=4,line-breaks=0,cell-like=true"
    ));
    assert!(stdout.contains("0:source-interval=0,source=10-14,chars=2,line-breaks=0,text=銀河"));
    assert!(stdout.contains("1:source-interval=1,source=16-22,chars=2,line-breaks=0,text=鉄道"));
    assert!(stdout.contains(
        "table-candidate-context\t1\trange=0\tboundary=1\tbasis=unit\tdelimiter=0x001c\tintervals=2\tsource=5-11\tshape=non-empty=2,empty=0,min-chars=2,max-chars=2,total-chars=4,line-breaks=0,cell-like=true"
    ));
}

#[test]
fn table_cell_like_candidates_command_filters_strict_candidates() {
    let path = text_count_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-cell-like-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "table-cell-like-candidate\t0\trange=0\tboundary=0\tbasis=byte\tdelimiter=0x001c\tintervals=2\tsource=10-22\tshape=non-empty=2,empty=0,min-chars=2,max-chars=2,total-chars=4,line-breaks=0,cell-like=true\ttexts=0:source-interval=0,source=10-14,chars=2,text=銀河|1:source-interval=1,source=16-22,chars=2,text=鉄道\tcolumn-split-candidate-rows=0\tmax-column-segment-count=0\tcolumn-segment-pattern-consistent=false\tcolumn-segment-pattern-mismatch-rows=0\tcolumn-grid-candidate=false\tcolumn-grid-shape=-\tcolumn-grid-pattern=-\tinterval-column-segments=-\tdecoded=false\n",
            "table-cell-like-candidate\t1\trange=0\tboundary=1\tbasis=unit\tdelimiter=0x001c\tintervals=2\tsource=5-11\tshape=non-empty=2,empty=0,min-chars=2,max-chars=2,total-chars=4,line-breaks=0,cell-like=true\ttexts=0:source-interval=0,source=5-7,chars=2,text=銀河|1:source-interval=1,source=8-11,chars=2,text=鉄道\tcolumn-split-candidate-rows=0\tmax-column-segment-count=0\tcolumn-segment-pattern-consistent=false\tcolumn-segment-pattern-mismatch-rows=0\tcolumn-grid-candidate=false\tcolumn-grid-shape=-\tcolumn-grid-pattern=-\tinterval-column-segments=-\tdecoded=false\n",
        )
    );
}

#[test]
fn table_cell_like_candidates_command_reports_column_segment_diagnostics() {
    let path = text_count_finance_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-cell-like-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("table-cell-like-candidate\t0\t"));
    assert!(stdout.contains("\tshape=non-empty=2,empty=0,"));
    assert!(stdout.contains("\tcolumn-split-candidate-rows=2\t"));
    assert!(stdout.contains("\tmax-column-segment-count=5\t"));
    assert!(stdout.contains("\tcolumn-segment-pattern-consistent=true\t"));
    assert!(stdout.contains("\tcolumn-segment-pattern-mismatch-rows=0\t"));
    assert!(stdout.contains("\tcolumn-grid-candidate=true\t"));
    assert!(stdout.contains("\tcolumn-grid-shape=2x5\t"));
    assert!(stdout.contains("\tcolumn-grid-pattern=label|value|value|value|value\t"));
    assert!(stdout.contains("interval-column-segments=0=0:label:2-5:売掛金"));
    assert!(stdout.contains("1:value:5-14:2,441,997"));
    assert!(stdout.contains("3:value:23-33:△1,541,604"));
    assert!(stdout.contains(";1=0:label:0-6:流動資産合計"));
    assert!(stdout.contains("4:value:28-34:17,327"));
}

#[test]
fn text_boundary_candidate_context_command_reports_text_context() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-candidate-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "text-boundary-candidate-context\t0\trange=0\tbasis=byte\tdelimiter=0x001c\tintervals=1\tinterval-kind=single\tsource=10-14\tline-breaks=0\ttext=entries=1,text=1,inline=0,skipped=0,control=0,preview=銀河"
    ));
    assert!(stdout.contains(
        "edges=inside=1,full=1,partial=0,start-edge=aligned:text(-)@10-14/5-7:銀河,end-edge=aligned:text(-)@10-14/5-7:銀河"
    ));
    assert!(stdout.contains(
        "text-boundary-candidate-context\t1\trange=0\tbasis=unit\tdelimiter=0x001c\tintervals=1\tinterval-kind=single\tsource=8-11\tline-breaks=0\ttext=entries=1,text=1,inline=0,skipped=0,control=0,preview=鉄道\\n"
    ));
    assert!(stdout.contains(
        "edges=inside=1,full=0,partial=1,start-edge=gap:control(0x001c)@14-16/7-8:|text(-)@18-24/9-12:鉄道\\n"
    ));
}

#[test]
fn text_boundary_candidate_agreement_command_compares_byte_and_unit_candidates() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-candidate-agreement")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "text-boundary-candidate-agreement\t0\trange=0\tdelimiter=0x001c\tbyte-index=0\tunit-index=1\tbyte-intervals=1\tunit-intervals=1\tbyte-interval-kind=single\tunit-interval-kind=single\tbyte-edge-good=false\tunit-edge-good=false\tbyte-line-breaks=0\tunit-line-breaks=0\ttext-match=false\tline-break-match=true\tbyte-text=銀河\tunit-text=鉄道\tdecoded=false\n"
    );
}

#[test]
fn text_boundary_candidate_layout_context_command_reports_rule_selected_context() {
    let path = text_boundary_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-candidate-layout-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "summary\tunit-001c-single-candidates=1\trule-selected=0\tline-bytes=40\tline-words=20\tpage-rows=20\tpage-bytes=1692\tpaper-rows=20\tpaper-bytes=172"
        ),
        "{stdout}"
    );
    assert!(stdout.contains(
        "candidate\t1\trange=0\tselected=false\tedge-good=false\tnon-empty=true\tline-breaks=0\tsource=8-11\ttext=鉄道"
    ));
    assert!(stdout.contains(
        "line-word-start=hit:8:0x1002\tline-word-end=hit:11:0x000b\tline-byte-start=hit:4:0x0004\tline-byte-end=unaligned:11"
    ));
    assert!(stdout.contains(
        "page-row-start=hit:8\tpage-row-end=hit:11\tpage-byte-start=hit:8\tpage-byte-end=hit:11\tpaper-row-start=hit:8\tpaper-row-end=hit:11"
    ));
}

#[test]
fn text_boundary_layout_map_command_scores_candidate_offset_transforms() {
    let path = text_boundary_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-layout-map")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tunit-001c-single-candidates=1\trule-selected=0\ttarget-sets=8\tbases=4\tdelta-range=-4096..4096"
    ));
    assert!(stdout.contains(
        "best\tscope=all\ttarget=line-tag-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\tpoints=2\tcandidates=1\tendpoints=2\tvalid=2\tinvalid=0\texact=1\ttotal-distance=1\tmax-distance=1\tdecoded=false"
    ));
    assert!(stdout.contains(
        "best\tscope=all\ttarget=page-entry-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\tpoints=20\tcandidates=1\tendpoints=2\tvalid=2\tinvalid=0\texact=2\ttotal-distance=0\tmax-distance=0\tdecoded=false"
    ));
    assert!(stdout.contains(
        "best\tscope=selected\ttarget=line-tag-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\tpoints=2\tcandidates=0\tendpoints=0\tvalid=0\tinvalid=0\texact=0\ttotal-distance=-\tmax-distance=-\tdecoded=false"
    ));
}

#[test]
fn text_boundary_layout_map_rows_command_reports_candidate_local_deltas() {
    let path = text_boundary_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-layout-map-rows")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tunit-001c-single-candidates=1\trule-selected=0\ttarget-sets=8\tbases=4\tlocal-rows=32"
    ));
    assert!(stdout.contains(
        "local\tcandidate=1\trange=0\tselected=false\ttarget=line-tag-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\texact=1\ttotal-distance=1\tmax-distance=1\tstart-nearest=8:8->8:d=0\tend-nearest=11:11->12:d=1\tsource=8-11\ttext=鉄道"
    ));
    assert!(stdout.contains(
        "local\tcandidate=1\trange=0\tselected=false\ttarget=page-entry-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\texact=2\ttotal-distance=0\tmax-distance=0\tstart-nearest=8:8->8:d=0\tend-nearest=11:11->11:d=0"
    ));
    assert!(stdout.contains(
        "tcnt=index=0,family=be0,start=9,end=12,span=3,declared-start=9,declared-end=12,tail=257,5"
    ));
}

#[test]
fn text_boundary_paragraph_like_command_reports_diagnostic_classifier() {
    let path = text_boundary_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-paragraph-like")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tunit-001c-single-candidates=1\tstrict-selected=0\tparagraph-like=0\tselected-non-paragraph-like=0\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false"
    ));
    assert!(stdout.contains("candidate\t1\trange=0\tstrict-selected=false\tparagraph-like=false"));
    assert!(stdout.contains("line-word-evidence="));
    assert!(stdout.contains("page-field-evidence=page-be32-field:unit:0:8:8->8:d=0|11:11->11:d=0"));
    assert!(stdout.contains(
        "tcnt=index=0,family=be0,start=9,end=12,span=3,declared-start=9,declared-end=12,tail=257,5"
    ));
}

#[test]
fn text_boundary_paragraph_like_style_context_command_links_layout_and_style_evidence() {
    let path = text_boundary_paragraph_like_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-paragraph-like-style-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "summary\tunit-001c-single-candidates=1\tstrict-selected=0\tparagraph-like=0\tselected-non-paragraph-like=0\ttext-style-candidates=2\tpage-style-candidates=1\tview-style-records=4\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false"
        ),
        "{stdout}"
    );
    assert!(stdout.contains(
        "candidate\t1\trange=0\tstrict-selected=false\tparagraph-like=false\tline-word-evidence=line-word-value:unit:0:8:8->8:d=0|11:11->11:d=0\tpage-field-evidence=page-be32-field:unit:0:8:8->8:d=0|11:11->11:d=0"
    ));
    assert!(stdout.contains(
        "tail-fields=f0=0x0202,f1=0x0001,f2=0x002f,f3=0x0100,f4=0x0000,f5=0x0000,f6=0x0000,f7=0x0001,f8=0x0000,f9=0x0000"
    ));
    assert!(stdout.contains(
        "text-style-id-hits=f1=0x0001:id1:offset276:見出し,f7=0x0001:id1:offset276:見出し"
    ));
    assert!(stdout.contains(
        "text-style-index-hits=f1=0x0001:idx1:id2:offset532:本文,f7=0x0001:idx1:id2:offset532:本文"
    ));
    assert!(stdout.contains("page-style-id-hits=f1=0x0001:id1:offset276:ページ"));
    assert!(stdout.contains(
        "view-style-group-hits=f1=0x0001:group1:records4:codes0x3104,0x3105,0x3106,0x3107"
    ));
    assert!(stdout.contains(
        "tcnt=index=0,family=be0,start=9,end=13,span=4,declared-start=9,declared-end=13"
    ));
}

#[test]
fn text_boundary_paragraph_like_discriminators_command_summarizes_candidate_buckets() {
    let path = text_boundary_paragraph_like_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-paragraph-like-discriminators")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tunit-001c-single-candidates=1\tstrict-selected=0\tparagraph-like=0\tselected-non-paragraph-like=0\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false\n"
    ));
    assert!(stdout.contains(
        "bucket\tparagraph-like\trows=0\tstrict-selected=0\tline-word-exact2=0\tpage-field-exact2=0\tdual-exact2=0"
    ));
    assert!(stdout.contains(
        "bucket\tstrict-non-paragraph\trows=0\tstrict-selected=0\tline-word-exact2=0\tpage-field-exact2=0\tdual-exact2=0"
    ));
    assert!(stdout.contains(
        "bucket\tnon-strict\trows=1\tstrict-selected=0\tline-word-exact2=1\tpage-field-exact2=1\tdual-exact2=1\ttext-style-hit=1\tpage-style-hit=1\tview-style-group-hit=1"
    ));
    assert!(stdout.contains(
        "source-spans=3..3\trange-spans=4..4\tfamilies=be0:1\tf0=0x0202:1\tf4=0x0000:1\tf7=0x0001:1"
    ));
    assert!(stdout.contains(
        "line-evidence=line-word-value/unit/0:1\tpage-evidence=page-be32-field/unit/0:1"
    ));
}

#[test]
fn text_paragraph_boundary_targets_command_reports_layout_hit_provenance() {
    let path = text_boundary_paragraph_like_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-paragraph-boundary-targets")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout
            .contains("summary\ttext-paragraph-boundary-candidates=1\tline-words=20\tpage-rows=20")
    );
    assert!(
        stdout.contains(
            "text-paragraph-boundary-target\t0\tboundary=1\trange=0\tsource=8-11\tspan=4"
        )
    );
    assert!(stdout.contains(
        "line-word-evidence=line-word-value:unit:0\tline-start=value=8,hits=1,refs=word8:0x0008\tline-end=value=11,hits=1,refs=word11:0x000b"
    ));
    assert!(stdout.contains(
        "page-field-evidence=page-be32-field:unit:0\tpage-start=value=8,hits=1,refs=row8:f0:0x00000008\tpage-end=value=11,hits=1,refs=row11:f0:0x0000000b"
    ));
    assert!(stdout.contains("text=鉄道"));
    assert!(stdout.contains(
        "tcnt=index=0,family=be0,start=9,end=13,span=4,declared-start=9,declared-end=13"
    ));
}
