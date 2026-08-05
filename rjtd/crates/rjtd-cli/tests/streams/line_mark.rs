use std::fs;
use std::process::Command;

use super::support::*;

#[test]
fn line_mark_tags_command_reports_tag_contexts() {
    let path = line_mark_tags_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("line-mark-tags")
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
        "tag\t3\t6\t0x1002\tprev=0x0914,0x0000,0x0001\tnext=0x0077,0x0002,0x1000,0x0074,0x1001,0x000d\n"
    ));
    assert!(stdout.contains(
        "tag\t6\t12\t0x1000\tprev=0x0001,0x1002,0x0077,0x0002\tnext=0x0074,0x1001,0x000d\n"
    ));
    assert!(stdout.contains("tag\t8\t16\t0x1001\tprev=0x0077,0x0002,0x1000,0x0074\tnext=0x000d\n"));
}

#[test]
fn line_mark_intervals_command_reports_delta_records() {
    let path = line_mark_intervals_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("line-mark-intervals")
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
        "summary\tlen=30\twords=15\tprofile=be16-delta-v1\tdeclared-count=3\tmax-records=3\tparsed-records=2\tbase-unit=16"
    ));
    assert!(stdout.contains(
        "line-mark-interval\trecord=0\tbyte=18\tword=9\tdelta=5\tflag=0x0002\tunit-start=16\tunit-end=21"
    ));
    assert!(stdout.contains(
        "line-mark-interval\trecord=1\tbyte=22\tword=11\tdelta=8\tflag=0x8002\tunit-start=21\tunit-end=29"
    ));
    assert!(stdout.contains(
        "line-mark-interval-stop\trecord=2\tbyte=26\tdelta=0\tflag=0x0000\treason=non-positive-delta"
    ));
}

#[test]
fn line_mark_text_context_command_compares_tag_words_to_document_text() {
    let path = line_mark_text_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("line-mark-text-context")
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
        "tag\t3\t6\t0x1002\tline-byte=hit:text(-)@2-10/1-5:ABtC\tline-unit=hit:text(-)@2-10/1-5:ABtC\tnext0=0x0041\tdoc-word-hits=1\tfirst-doc-unit=1\tfirst-doc-context=hit:text(-)@2-10/1-5:ABtC"
    ));
    assert!(stdout.contains(
        "tag\t6\t12\t0x1000\tline-byte=between:text(-)@2-10/1-5:ABtC|-\tline-unit=between:text(-)@2-10/1-5:ABtC|-\tnext0=0x0074\tdoc-word-hits=1\tfirst-doc-unit=3\tfirst-doc-context=hit:text(-)@2-10/1-5:ABtC"
    ));
}
