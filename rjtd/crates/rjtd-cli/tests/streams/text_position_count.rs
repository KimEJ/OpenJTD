use std::fs;
use std::process::Command;

use super::support::*;

#[test]
fn text_position_counts_command_reports_tcnt_entries() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-counts")
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
    assert!(stdout.contains("header\t1\t0\t2\t36\t2\n"));
    assert!(stdout.contains(
        "entry\t0\t4660\t4688\t0000123400001250010100050000000000000000000000010000000000\n"
    ));
    assert!(stdout.contains(
        "entry\t1\t8192\t9216\t0000200000002400010100060000000000000000000000010000000000\n"
    ));
}

#[test]
fn text_position_count_context_command_compares_tcnt_fields() {
    let path = text_count_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-context")
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
    assert!(stdout.contains("0\t10\t13\thit:text(-)@10-14/5-7:銀河\thit:text(-)@10-14/5-7:銀河\t"));
    assert!(stdout.contains(
        "1\t5\t6\tbetween:-|text(-)@10-14/5-7:銀河\tbetween:-|text(-)@10-14/5-7:銀河\thit:text(-)@10-14/5-7:銀河\thit:text(-)@10-14/5-7:銀河"
    ));
}

#[test]
fn text_position_count_tail_context_command_compares_tail_fields() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-context")
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
    assert!(stdout.contains("tail-context\t0\tbe0\t100\t112\tt1=5\tt2=6\ttspan=1\t"));
    assert!(
        stdout.contains("t1-unit=hit:text(-)@10-14/5-7:銀河\tt2-unit=hit:text(-)@10-14/5-7:銀河")
    );
    assert!(stdout.contains("tail-context\t1\tbe1-shifted\t38602\t38602\tt1=9\tt2=11\ttspan=2\t"));
    assert!(stdout.contains(
        "t1-unit=hit:text(-)@18-24/9-12:鉄道\\n\tt2-unit=hit:text(-)@18-24/9-12:鉄道\\n"
    ));
}

#[test]
fn text_position_count_clusters_command_groups_duplicate_ranges() {
    let path = text_count_cluster_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-clusters")
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
    assert!(stdout.contains("10\t13\t2\t0,1\t2\t"));
    assert!(stdout.contains("010100050000000000000000000000010000000000"));
    assert!(stdout.contains("010100060000000000000000000000010000000000"));
    assert!(stdout.contains("20\t24\t1\t2\t1\t"));
}

#[test]
fn text_position_count_candidates_command_reports_shifted_fields() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-candidates")
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
    assert!(stdout.contains("0\t4660\t4688\t1192960\t1200129\t"));
    assert!(stdout.contains("1\t8192\t9216\t2097152\t2359297\t"));
}

#[test]
fn text_position_count_family_command_classifies_be0_entries() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-family")
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
        stdout.contains("family\t0\tbe0\t4660\t4688\t4660\t4688\t1192960\t1200129\tlead=0x00\t")
    );
    assert!(stdout.contains("tail=010100050000000000000000000000010000000000\n"));
}

#[test]
fn text_position_count_family_command_classifies_shifted_entries() {
    let path = shifted_text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-family")
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
        "family\t0\tbe1-shifted\t38602\t38602\t150\t3388997782\t38602\t38602\tlead=0x00\t"
    ));
    assert!(stdout.contains("tail=01010041004f0100000100000000000001000000\n"));
}

#[test]
fn text_position_count_fields_command_expands_tail_words() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-fields")
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
        "fields\t0\tbe0\t4660\t4688\t28\tlead=0x00\ttail-offset=8\ttail-be16=0x0101,0x0005,0x0000,0x0000,0x0000,0x0000,0x0000,0x0001,0x0000,0x0000\ttail-extra=00"
    ));
}

#[test]
fn text_position_count_fields_command_expands_shifted_tail_words() {
    let path = shifted_text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-fields")
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
        "fields\t0\tbe1-shifted\t38602\t38602\t0\tlead=0x00\ttail-offset=9\ttail-be16=0x0101,0x0041,0x004f,0x0100,0x0001,0x0000,0x0000,0x0000,0x0100,0x0000\ttail-extra=-"
    ));
}

#[test]
fn text_position_count_field_deltas_command_compares_tail_range_to_chosen_range() {
    let path = text_count_delta_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-field-deltas")
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
        "delta\t0\tbe0\t100\t112\t12\ttail-offset=8\tt1=10\tt2=22\ttspan=12\tspan-relation=eq\tstart-minus-t1=90\tend-minus-t2=90\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001"
    ));
    assert!(stdout.contains(
        "delta\t1\tbe1-shifted\t38602\t38602\t0\ttail-offset=9\tt1=65\tt2=79\ttspan=14\tspan-relation=gt\tstart-minus-t1=38537\tend-minus-t2=38523\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001"
    ));
}

#[test]
fn text_position_count_tail_delta_scan_command_scores_unit_offsets() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-delta-scan")
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
    assert!(stdout.contains("delta\t0\t2\t4\t4\t4\t2\t2\n"));
    assert!(stdout.contains("delta\t29\t2\t4\t0\t0\t0\t0\n"));
    assert!(stdout.contains("delta\t64\t2\t4\t0\t0\t0\t0\n"));
}

#[test]
fn text_position_count_tail_delta_groups_command_summarizes_pattern_scores() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-delta-groups")
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
        "group\tbe0\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\trows=1\tendpoints=2\tbest-unit=0:2:1\tbest-text=0:2:1\td0=2:2:1:1\td29=0:0:0:0\td30=0:0:0:0\n"
    ));
    assert!(stdout.contains(
        "group\tbe1-shifted\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\trows=1\tendpoints=2\tbest-unit=0:2:1\tbest-text=0:2:1\td0=2:2:1:1\td29=0:0:0:0\td30=0:0:0:0\n"
    ));
}

#[test]
fn text_position_count_tail_row_deltas_command_reports_per_row_scores() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-row-deltas")
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
    assert!(stdout.contains("summary\tentries=2\tdoc-bytes=24\tdoc-units=12\n"));
    assert!(stdout.contains(
        "row\t0\tbe0\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\tstart=100\tend=112\tspan=12\tt1=5\tt2=6\ttspan=1\tbest-unit=0:2:1\tbest-text=0:2:1\td0=2:2:1:1\td29=0:0:0:0\td30=0:0:0:0\n"
    ));
    assert!(stdout.contains(
        "row\t1\tbe1-shifted\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\tstart=38602\tend=38602\tspan=0\tt1=9\tt2=11\ttspan=2\tbest-unit=0:2:1\tbest-text=0:2:1\td0=2:2:1:1\td29=0:0:0:0\td30=0:0:0:0\n"
    ));
}

#[test]
fn text_position_count_tail_row_context_command_reports_chosen_and_tail_contexts() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-row-context")
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
        "row-context\t0\tbe0\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\tstart=100\tend=112\tt1=5\tt2=6\tbest-unit=0:2:1\tbest-text=0:2:1"
    ));
    assert!(stdout.contains("start-byte=between:text(-)@18-24/9-12:鉄道\\n|-\t"));
    assert!(stdout.contains(
        "t1-unit-best=hit:text(-)@10-14/5-7:銀河\tt2-unit-best=hit:text(-)@10-14/5-7:銀河"
    ));
    assert!(stdout.contains(
        "row-context\t1\tbe1-shifted\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\tstart=38602\tend=38602\tt1=9\tt2=11\tbest-unit=0:2:1\tbest-text=0:2:1"
    ));
    assert!(stdout.contains(
        "t1-unit-best=hit:text(-)@18-24/9-12:鉄道\\n\tt2-unit-best=hit:text(-)@18-24/9-12:鉄道\\n"
    ));
}

#[test]
fn text_position_count_tail_field_roles_command_summarizes_field_and_pair_hits() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-field-roles")
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
        stdout.contains("summary\tposition-status=ok\tentries=2\tdoc-bytes=24\tdoc-units=12\n")
    );
    assert!(stdout.contains(
        "field\tf1\tnonzero=2\tdistinct=2\tvalues=0x0005:1,0x0009:1\tunit-d0=2\ttext-d0=2"
    ));
    assert!(stdout.contains(
        "field\tf2\tnonzero=2\tdistinct=2\tvalues=0x0006:1,0x000b:1\tunit-d0=2\ttext-d0=2"
    ));
    assert!(stdout.contains(
        "pair\tf1-f2\tpairs=2\tendpoints=4\tspan-eq=0\tspan-lt=1\tspan-gt=1\tbest-unit=0:4:2\tbest-text=0:4:2\td0=4:4:2:2"
    ));
}

#[test]
fn text_position_count_range_preview_command_reports_overlapping_text() {
    let path = text_count_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-range-preview")
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
        "range-preview\t0\tbe0\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=10\tend=13\tspan=3\tbyte-range=entries=1,text=1,inline=0,skipped=0,control=0,preview=銀河\tunit-range=entries=1,text=1,inline=0,skipped=0,control=0,preview=鉄道\\n\n"
    ));
    assert!(stdout.contains(
        "range-preview\t1\tbe0\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=5\tend=6\tspan=1\tbyte-range=entries=0,text=0,inline=0,skipped=0,control=0,preview=-\tunit-range=entries=1,text=1,inline=0,skipped=0,control=0,preview=銀河\n"
    ));
}

#[test]
fn text_position_style_context_command_reports_tail_field_style_hits() {
    let path = text_position_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-style-context")
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
        "summary\tposition-status=ok\tentries=1\ttext-style-candidates=2\tpage-style-candidates=1\tview-style-records=4\n"
    ));
    assert!(stdout.contains(
        "entry\t0\tbe0\tstart=10\tend=16\tspan=6\ttail-fields=f0=0x0202,f1=0x0001,f2=0x002f,f3=0x0100,f4=0x0000,f5=0x0000,f6=0x0000,f7=0x0001,f8=0x0000,f9=0x0000"
    ));
    assert!(stdout.contains(
        "text-style-id-hits=f1=0x0001:id1:offset276:見出し,f7=0x0001:id1:offset276:見出し"
    ));
    assert!(stdout.contains(
        "text-style-index-hits=f1=0x0001:idx1:id2:offset532:本文,f7=0x0001:idx1:id2:offset532:本文"
    ));
    assert!(stdout.contains(
        "page-style-id-hits=f1=0x0001:id1:offset276:ページ,f7=0x0001:id1:offset276:ページ"
    ));
    assert!(stdout.contains(
        "view-style-group-hits=f1=0x0001:group1:records4:codes0x3104,0x3105,0x3106,0x3107,f7=0x0001:group1:records4:codes0x3104,0x3105,0x3106,0x3107"
    ));
    assert!(stdout.contains("byte-range=entries=2,text=1,inline=0,skipped=0,control=1"));
}

#[test]
fn text_position_style_summary_command_reports_field_hit_distribution() {
    let path = text_position_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-style-summary")
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
        "summary\tposition-status=ok\tentries=1\ttext-style-candidates=2\tpage-style-candidates=1\tview-style-records=4\n"
    ));
    assert!(stdout.contains(
        "field\tf1\tnonzero=1\tdistinct=1\tvalues=0x0001:1\ttext-style-id-hits=id1:1:offset276:見出し\ttext-style-index-hits=idx1:1:id2:offset532:本文\tpage-style-id-hits=id1:1:offset276:ページ\tpage-style-index-hits=-\tview-style-group-hits=group1:1:records4:codes0x3104,0x3105,0x3106,0x3107\n"
    ));
    assert!(stdout.contains(
        "field\tf7\tnonzero=1\tdistinct=1\tvalues=0x0001:1\ttext-style-id-hits=id1:1:offset276:見出し\ttext-style-index-hits=idx1:1:id2:offset532:本文\tpage-style-id-hits=id1:1:offset276:ページ\tpage-style-index-hits=-\tview-style-group-hits=group1:1:records4:codes0x3104,0x3105,0x3106,0x3107\n"
    ));
}

#[test]
fn text_position_count_range_boundaries_command_reports_edges_and_controls() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-range-boundaries")
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
        "range-boundary\t0\tbe0\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=10\tend=16\tspan=6\t"
    ));
    assert!(stdout.contains(
        "byte-boundary=inside=2,full=2,partial=0,start-edge=aligned:text(-)@10-14/5-7:銀河,end-edge=aligned:control(0x001c)@14-16/7-8:,first=text(-)@10-14/5-7:銀河,last=control(0x001c)@14-16/7-8:,prev=-,next=text(-)@18-24/9-12:鉄道\\n,controls=0x001c:1"
    ));
    assert!(stdout.contains(
        "unit-boundary=inside=1,full=0,partial=1,start-edge=inside:text(-)@18-24/9-12:鉄道\\n,end-edge=gap:text(-)@18-24/9-12:鉄道\\n|-,first=text(-)@18-24/9-12:鉄道\\n,last=text(-)@18-24/9-12:鉄道\\n,prev=control(0x001c)@14-16/7-8:,next=-,controls=-"
    ));
    assert!(stdout.contains(
        "range-boundary\t1\tbe0\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=7\tend=8\tspan=1\t"
    ));
    assert!(stdout.contains(
        "unit-boundary=inside=1,full=1,partial=0,start-edge=aligned:control(0x001c)@14-16/7-8:,end-edge=aligned:control(0x001c)@14-16/7-8:,first=control(0x001c)@14-16/7-8:,last=control(0x001c)@14-16/7-8:,prev=text(-)@10-14/5-7:銀河,next=text(-)@18-24/9-12:鉄道\\n,controls=0x001c:1"
    ));
}

#[test]
fn text_position_count_control_ranges_command_compares_tcnt_to_control_intervals() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-control-ranges")
        .arg(&path)
        .arg("0x001c")
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
        "count-control-range\t0\tbe0\tdelimiter=0x001c\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=10\tend=16\tspan=6\tbyte-ranges=count=1,first=0,last=0,byte=10-14,unit=5-7,entry-ranges=0-0,controls=-,preview=銀河\tunit-ranges=count=1,first=1,last=1,byte=16-24,unit=8-12,entry-ranges=2-2,controls=-,preview=鉄道\\n"
    ));
    assert!(stdout.contains(
        "count-control-range\t1\tbe0\tdelimiter=0x001c\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=7\tend=8\tspan=1\tbyte-ranges=count=0,first=-,last=-,byte=-,unit=-,entry-ranges=-,controls=-,preview=-\tunit-ranges=count=0,first=-,last=-,byte=-,unit=-,entry-ranges=-,controls=-,preview=-"
    ));
}

#[test]
fn text_position_count_layout_context_command_compares_tcnt_offsets_to_layout_streams() {
    let path = text_count_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-layout-context")
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
        "summary\tentries=2\tline-bytes=12\tline-words=6\tpage-rows=3\tpage-bytes=264\tpaper-rows=3\tpaper-bytes=36\n"
    ));
    assert!(stdout.contains(
        "entry\t0\tbe0\t2\t12\tline-word-start=hit:2:0x1002\tline-word-end=out-of-range:6\tline-byte-start=hit:1:0x0000\tline-byte-end=out-of-range:12\tpage-row-start=hit:2\tpage-row-end=out-of-range:3\tpage-byte-start=hit:2\tpage-byte-end=hit:12\tpaper-row-start=hit:2\tpaper-row-end=out-of-range:3\tpaper-byte-start=hit:2\tpaper-byte-end=hit:12\n"
    ));
}
