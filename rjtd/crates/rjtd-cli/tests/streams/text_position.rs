use std::fs;
use std::process::Command;

use super::support::*;

#[test]
fn text_position_line_context_command_compares_mark_offsets_to_line_mark() {
    let path = text_position_line_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-line-context")
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
        "summary\tline-words=9\tline-tags=3\tmark-entries=3\tpage-entries=missing\tpaper-entries=missing\n"
    ));
    assert!(stdout.contains(
        "header\t30\t000000000002\tline-index=2\tword=0x1002\tprev-tag=-\tnext-tag=0x1000@4,d=2\tcontext=prev=0x0914,0x0000|next=0x0041,0x1000,0x0074,0x000d,0x1001,0x000a\n"
    ));
    assert!(stdout.contains(
        "entry\t1\t4\tline-index=4\tword=0x1000\tprev-tag=0x1002@2,d=-2\tnext-tag=0x1001@7,d=3\tcontext=prev=0x0914,0x0000,0x1002,0x0041|next=0x0074,0x000d,0x1001,0x000a\n"
    ));
    assert!(stdout.contains(
        "entry\t3\t20\tline-index=20\tword=out-of-range\tprev-tag=0x1001@7,d=-13\tnext-tag=-\tcontext=prev=0x0074,0x000d,0x1001,0x000a|next=-\n"
    ));
}

#[test]
fn text_positions_command_reports_mark_offsets() {
    let path = position_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-positions")
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
        "1\t4660\n2\t22136\n"
    );
}

#[test]
fn text_position_mark_header_command_reports_raw_header_and_entries() {
    let path = position_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-mark-header")
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
        stdout.contains("header\t30\t000000000002\tbe16=0,0,2\tle16=0,0,512\tbe32@0=0\tbe32@2=2\n")
    );
    assert!(stdout.contains("entry\t30\t0\t44\t1\t4660\t000100001234\n"));
    assert!(stdout.contains("entry\t30\t1\t50\t2\t22136\t000200005678\n"));
}

#[test]
fn text_position_mark_summary_command_reports_related_stream_metrics() {
    let path = mark_summary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-mark-summary")
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
    assert!(stdout.starts_with("summary\t30\t000000000002\t2\t2\t22136\t24\t12\t"));
    assert!(stdout.contains("len=8,words=0x0914,0x0000,0x0001,0x0000\t"));
    assert!(stdout.contains("count=2,stride=16,last=1,entries=3,family=fixed84\t"));
    assert!(stdout.contains("count=2,stride=12,last=1,entries=3\t264\t36\n"));
}

#[test]
fn text_positions_command_rejects_count_only_table() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-positions")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("missing MarkV.01 table"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn text_map_command_reports_token_ranges_and_position_hits() {
    let path = text_map_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-map")
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
    assert!(stdout.contains("10\t14\t5\t7\ttext\t-\t1\t2\t銀河\n"));
    assert!(stdout.contains("14\t16\t7\t8\tcontrol\t0x001c\t-\t-\t\n"));
}

#[test]
fn text_position_context_command_compares_byte_and_unit_offsets() {
    let path = text_map_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-context")
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
            "1\t10\thit:text(-)@10-14/5-7:銀河\thit:text(-)@18-24/9-12:鉄道\\n\tbetween:"
        )
    );
    assert!(
        stdout.contains(
            "2\t5\tbetween:-|text(-)@10-14/5-7:銀河\thit:text(-)@10-14/5-7:銀河\tbetween:"
        )
    );
}

#[test]
fn text_position_delta_scan_command_scores_unit_offsets() {
    let path = text_map_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-delta-scan")
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
    assert!(stdout.contains("delta\t0\t2\t2\t2\n"));
    assert!(stdout.contains("delta\t29\t2\t0\t0\n"));
    assert!(stdout.contains("delta\t64\t2\t0\t0\n"));
}
