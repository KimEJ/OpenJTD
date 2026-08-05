use std::fs;
use std::process::Command;

use super::support::*;

#[test]
fn style_records_command_reports_style_stream_record_summaries() {
    let path = style_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("style-records")
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
    assert!(stdout.contains("style_streams\t1\n"));
    assert!(stdout.contains(
        "stream\t/TextLayoutStyle\tbytes=286\tfamily=ssmg\trecordLayout=ssmg-slots\trecordCount=1\theaderU32Be=0x00000000,0x00000000,0x00000000\theaderU16Be=0x0000,0x0000\n"
    ));
    assert!(
        stdout.contains(
            "record\t/TextLayoutStyle\t0\toffset=276\tcode=0x5555\tpayloadLength=6\tlabel="
        )
    );
    assert!(stdout.contains("\u{672c}\u{6587}\n"));
}

#[test]
fn page_layout_style_slots_command_reports_raw_slot_parts() {
    let path = page_layout_style_slot_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-layout-style-slots")
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
    assert!(stdout.contains("summary\tstatus=ok\tstream=/PageLayoutStyle\tstream-bytes="));
    assert!(stdout.contains(
        "\trecords=1\tslots=3\tpaired-slot-pairs=0x32/0x33\tfacing-pages-candidate=true\tdecoded=false\n"
    ));
    assert!(stdout.contains(
        "record\t0\toffset=276\tcode=0x4444\tpayloadLength=43\tlabel=ページ\tsubrecords=6\n"
    ));
    assert!(stdout.contains(
        "slot\t0\t0x31\tpart05First=0x04\tpart05NonZero=true\tpart04=aa\tpart05=0400\tpart06=bb\tpart07=cc\n"
    ));
    assert!(stdout.contains(
        "slot\t0\t0x32\tpart05First=0x04\tpart05NonZero=true\tpart04=-\tpart05=0400\tpart06=-\tpart07=-\n"
    ));
    assert!(stdout.contains(
        "slot\t0\t0x33\tpart05First=0x04\tpart05NonZero=true\tpart04=-\tpart05=0400\tpart06=-\tpart07=-\n"
    ));
}

#[test]
fn style_candidates_command_reports_labeled_text_layout_records() {
    let path = style_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("style-candidates")
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
            "style_candidates\t1\n",
            "candidate\t1\t/TextLayoutStyle\t0\toffset=276\tcode=0x5555\tpayloadLength=6\tname=",
            "\u{672c}\u{6587}",
            "\n"
        )
    );
}

#[test]
fn text_layout_style_records_command_reports_payload_diagnostics() {
    let path = style_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-layout-style-records")
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
        "summary\tstatus=ok\tstream=/TextLayoutStyle\tstream-bytes=286\trecords=1\tlabeled=1\n"
    ));
    assert!(stdout.contains(
        "record\t0\tcandidate=1\toffset=276\tcode=0x5555\tpayloadLength=6\tpayloadDigest=0x"
    ));
    assert!(
        stdout.contains("\tpayloadPrefix=0002672c6587\tpayloadBe16=0x0002,0x672c,0x6587\tlabel=")
    );
    assert!(stdout.contains("\u{672c}\u{6587}\n"));
}

#[test]
fn text_layout_style_records_command_reports_missing_stream() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-layout-style-records")
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
        "summary\tstatus=missing\tstream=/TextLayoutStyle\tstream-bytes=0\trecords=0\tlabeled=0\n"
    );
}

#[test]
fn document_view_style_groups_command_reports_payload_diagnostics() {
    let path = text_position_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("document-view-style-groups")
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
            "summary\tstatus=ok\tstream-bytes=20\trecords=4\tgroups=1\tgroup-records=4\n"
        )
    );
    assert!(stdout.contains(
        "group\t1\trecords=4\tcodes=0x3104,0x3105,0x3106,0x3107\tpayloadLengths=1,1,1,1\tpayloadDigest=0x"
    ));
    assert!(
        stdout.contains("record\t1\t0\toffset=0\tcode=0x3104\tpayloadLength=1\tpayloadDigest=0x")
    );
    assert!(stdout.contains("\tpayloadPrefix=04\tpayloadBe16=-\n"));
    assert!(
        stdout.contains("record\t1\t3\toffset=15\tcode=0x3107\tpayloadLength=1\tpayloadDigest=0x")
    );
    assert!(stdout.contains("\tpayloadPrefix=07\tpayloadBe16=-\n"));
}

#[test]
fn document_view_style_groups_command_ignores_ungrouped_records() {
    let path = document_view_style_ungrouped_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("document-view-style-groups")
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
        "summary\tstatus=ok\tstream-bytes=20\trecords=4\tgroups=0\tgroup-records=0\n"
    );
}
