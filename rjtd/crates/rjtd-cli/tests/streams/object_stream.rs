use std::fs;
use std::process::Command;

use super::support::*;

#[test]
fn so_records_command_reports_marker_fields() {
    let path = so_record_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-records")
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
    assert!(stdout.contains("record\t/Object\t2\t"));
    assert!(stdout.contains("0x00004f53,0x00000007,0x00000100"));
    assert!(stdout.contains("534f00000700000000010000"));
}

#[test]
fn object_stream_candidates_command_reports_visual_object_inventory() {
    let path = object_stream_candidates_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-stream-candidates")
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
    let summary = stdout.lines().next().unwrap_or_default();
    assert!(summary.starts_with("summary\t"));
    for field in [
        "streams=9",
        "candidates=8",
        "unreadable=0",
        "object-path=4",
        "image-path=1",
        "shape-path=2",
        "table-path=1",
        "visual-list-path=1",
        "visual-list-raster=0",
        "embedded-press-snapshot=1",
        "jseq3-formula=1",
        "jsfart-stream-profile=1",
        "so-marker=3",
        "image-signature=1",
        "svg-signature=1",
        "decoded=false",
    ] {
        assert!(
            summary.contains(field),
            "summary did not contain {field}: {summary}"
        );
    }
    assert!(stdout.contains(
        "stream=/EmbedItems/Embedding 1/JSFart2Contents\tsize=38\treasons=object-path,so-marker\timage-signatures=-\tsvg-offsets=-\tso-offsets=26\t"
    ));
    assert!(stdout.contains(
        "jsfart-stream-profile=jsfart-object-utf16le,hex=4a00,preview=JSFART.O,structured-art=false,blocked=jsfart-variant-layout-undecoded"
    ));
    assert!(stdout.contains(
        "stream=/EmbedItems/Embedding 1/\\x03EmbeddedPress\tsize=128\treasons=object-path,embedded-press-snapshot\timage-signatures=-\tsvg-offsets=-\tso-offsets=-\tvisual-list=-\tembedded-press-snapshot=JSSnapShot32,2590x460,body=3656,objects=17\t"
    ));
    assert!(stdout.contains("stream=/EmbedItems/Embedding 1/JSEQ3Contents"));
    assert!(stdout.contains("jseq3-formula"));
    assert!(stdout.contains("jseq3-formula=MATH.VAF,so=116,fields=0x00004f53,0x200e0a20"));
    assert!(stdout.contains("markers=Times New Roman@16,JustUnitMark@46,JustOubunMark@70"));
    assert!(stdout.contains(
        "stream=/EmbedItems/Embedding 2/Image.png\tsize=44\treasons=object-path,image-path,image-signature\timage-signatures=jpeg@0\t"
    ));
    assert!(stdout.contains(
        "stream=/Figure\tsize=12\treasons=shape-path,so-marker\timage-signatures=-\tsvg-offsets=-\tso-offsets=0\t"
    ));
    assert!(stdout.contains(
        "stream=/Tables/Table1\tsize=13\treasons=table-path\timage-signatures=-\tsvg-offsets=-\tso-offsets=-\t"
    ));
    assert!(stdout.contains(
        "stream=/Vector.svg\tsize=52\treasons=shape-path,svg-signature\timage-signatures=-\tsvg-offsets=21\tso-offsets=-\t"
    ));
    assert!(stdout.contains(
        "stream=/VisualList\tsize=23\treasons=visual-list-path\timage-signatures=-\tsvg-offsets=-\tso-offsets=-\tvisual-list=-\t"
    ));
}

#[test]
fn object_ownership_references_command_reports_reference_contexts() {
    let path = object_stream_candidates_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-ownership-references")
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
        "object-ownership-reference\tsource=/EmbedItems/Embedding 2/Image.png\ttarget=/Figure\tencoding=u16-le\toffset=6\ttotal=1\tmod2=0\tmod4=2\t"
    ));
    assert!(stdout.contains("window-start=0\twindow-hex=534f0000ff090200a0080002\t"));
    assert!(stdout.contains("\tle16=2\tbe16=512\tle32=144703490\tbe32=33595400\t"));
    assert!(stdout.contains(
        "summary\tsources=1\treferences=2\treported-offsets=2\ttarget-missing=0\tdecoded=false\n"
    ));
}

#[test]
fn object_ownership_reference_fields_command_groups_stride_candidates() {
    let path = object_stream_candidates_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-ownership-reference-fields")
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
        stdout.contains("summary\tsources=1\treferences=2\treported-offsets=2\tfield-groups=40\t")
    );
    assert!(stdout.contains(
        "strides=4,8,12,16,20,24,28,32,36,40,44,48,52,56,60,64,68,72,80,84\tdecoded=false"
    ));
    assert!(stdout.contains(
        "object-ownership-reference-field\ttarget=/Figure\tencoding=u16-le\tstride=12\tfield-offset=6\tmatches=1\tsource-count=1\tembedding-indexes=2\trow-indexes=0\tcross-row=0\tdecoded=false"
    ));
    assert!(stdout.contains(
        "object-ownership-reference-field\ttarget=/Figure\tencoding=u16-be\tstride=12\tfield-offset=10\tmatches=1\tsource-count=1\tembedding-indexes=2\trow-indexes=0\tcross-row=0\tdecoded=false"
    ));
}

#[test]
fn object_frame_reference_records_command_expands_candidate_rows() {
    let path = object_frame_reference_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-frame-reference-records")
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
        "object-frame-reference-record\tsource=/EmbedItems/Embedding 2/Image.jpg\tembedding=2\ttarget=/Frame\tencoding=u16-le\tstride=12\tfield-offset=5\toffset=5\trow-index=0\trow-start=0\tcandidate=u16-le/12/5\t"
    ));
    assert!(stdout.contains("row-hex=000100000002000000010001\trow-be16=0x0001,0x0000,0x0002,0x0000,0x0001,0x0001\trow-le16=256,0,512,0,256,256\t"));
    assert!(stdout.contains(
        "row-be32=0x00010000,0x00020000,0x00010001\trow-le32=0x00000100,0x00000200,0x01000100\tdecoded=false"
    ));
    assert!(
        stdout.contains(
            "summary\tsources=1\tframe-references=4\trecords=1\tskipped=0\tcandidates=u16-le/12/5,u16-be/12/7,u16-be/20/15\tdecoded=false\n"
        ),
        "stdout: {stdout}"
    );
}

#[test]
fn object_frame_record_families_command_groups_candidate_rows() {
    let path = object_frame_reference_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-frame-record-families")
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
        "object-frame-record-family\tfamily=frame-index-flag-row12\trows=1\tcandidates=u16-le/12/5\tembeddings=2\texamples=000100000002000000010001\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tfamilies=1\trecords=1\tskipped=0\tcandidates=u16-le/12/5,u16-be/12/7,u16-be/20/15\tdecoded=false\n"
    ));
}

#[test]
fn object_frame_row_links_command_connects_window_suffix_rows() {
    let path = object_frame_row_link_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-frame-row-links")
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
        "object-frame-row-link\tsource=/EmbedItems/Embedding 2/Image.jpg\tembedding=2\trow20-family=frame-index-tail-window20\trow20-start=0\trow20-index=0\tprefix-hex=0000000000200000\tsuffix-hex=000000000102000002000000\trelation=same-source\tsuffix-family=frame-index-tail-coordinate-row12\tmatched-source=/EmbedItems/Embedding 2/Image.jpg\tmatched-row-start=24\tmatched-row-index=2\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\trow20=1\tlinked=1\tunlinked=0\trelations=same-source:1\tfamily-pairs=frame-index-tail-window20->frame-index-tail-coordinate-row12:1\tdecoded=false"
    ));
}

#[test]
fn so_record_clusters_command_groups_raw_records() {
    let path = so_record_cluster_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-record-clusters")
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
    assert!(stdout.contains("cluster\t2\t"));
    assert!(stdout.contains("0x00004f53,0x00000007,0x00000100,0x00000000,0x00000064"));
    assert!(stdout.contains("/First@0,/Second@2"));
}

#[test]
fn so_record_fields_command_reports_le_breakdown() {
    let path = so_record_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-record-fields")
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
            .contains("field\t/Object\t2\t0\t0x00004f53\t20307\t20307\t0x4f53\t20307\t0x0000\t0\n")
    );
    assert!(stdout.contains("field\t/Object\t2\t1\t0x00000007\t7\t7\t0x0007\t7\t0x0000\t0\n"));
    assert!(
        stdout.contains("field\t/Object\t2\t2\t0x00000100\t256\t256\t0x0100\t256\t0x0000\t0\n")
    );
}

#[test]
fn so_record_geometry_command_reports_coordinate_candidates() {
    let path = so_record_geometry_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-record-geometry")
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
        "candidate\t/Geometry\t0\tgeometry-like\t2559\t2208\t5018\t2208\t2459\t0\t7577\t4416\t"
    ));
    assert!(
        stdout.contains("534f0000ff090000a00800009a130000a008000000000000000000000000000000000000")
    );
}

#[test]
fn so_record_halves_command_reports_packed_pairs() {
    let path = so_record_packed_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-record-halves")
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
        "halves\t/Packed\t0\tpacked-jseq3-like\tlo_u16=2592,36122,30922,0,0,36122,7290,0\t"
    ));
    assert!(stdout.contains("hi_u16=8206,6126,20346,0,0,0,0,0\t"));
    assert!(stdout.contains("lo_i16=2592,-29414,30922,0,0,-29414,7290,0\t"));
    assert!(stdout.contains("hi_i16=8206,6126,20346,0,0,0,0,0\t"));
    assert!(
        stdout.contains("534f0000200a0e201a8dee17ca787a4f00000000000000001a8d00007a1c000000000000")
    );
}
