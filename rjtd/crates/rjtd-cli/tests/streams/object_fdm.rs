use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::support::*;

#[test]
fn object_image_frame_candidates_command_prioritizes_row12_tail_coordinates() {
    let path = object_frame_row_link_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-image-frame-candidates")
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
            "object-image-frame-candidate\tsource=/EmbedItems/Embedding 2/Image.jpg\tembedding=2\tpayloads=1\tpayload-kinds=jpeg\tpayload-dimensions=jpeg@0:32x16\tdimensioned-payloads=1\tframe-rows=3\t"
        ),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(
        "object-image-frame-candidate\tsource=/EmbedItems/Embedding 2/Image.jpg\tembedding=2\tpayloads=1\tpayload-kinds=jpeg\tpayload-dimensions=jpeg@0:32x16\tdimensioned-payloads=1\tframe-rows=3\t"
    ));
    assert!(stdout.contains(
        "row-families=frame-index-mixed-row12:1,frame-index-tail-coordinate-row12:1,frame-index-tail-window20:1\trow12-tail-coordinate=1\trow12-tail-zero=0\trow20-tail-window=1\trow20-linked=1\tle-row12=1\tpreferred=row12-tail-coordinate\tcoordinate-pairs=24:258x512\tbest-coordinate-aspect-delta-permille=748\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tsources=1\tframe-linked=1\tmissing-frame=0\tframe-rows=3\tdimensioned-payloads=1\taspect-candidates=1\tpreferred=row12-tail-coordinate:1\tdecoded=false"
    ));
}

#[test]
fn object_fdm_index_command_links_index_rows_to_vector_image_hits() {
    let path = object_fdm_index_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-index")
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
            "object-fdm-index-summary\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\tindex-bytes=64\tvector-bytes=103\tdeclared-count=2\tparsed-entries=2\ttrailing-bytes=0\tentries-with-image=1\timage-hits=1\toffset-field-ref-rows=0\toffset-field-refs=0\tvector-missing=false\tdecoded=false"
        ),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(
        "object-fdm-index-entry\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=0\tindex-offset=20\tvector-offset=0\tnext-vector-offset=32\tvector-length=32\tkind=0x0b00\tbbox=1,2,3,4\tvalid-vector-offset=true\t"
    ));
    assert!(stdout.contains(
        "object-fdm-index-entry\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=1\tindex-offset=42\tvector-offset=32\tnext-vector-offset=103\tvector-length=71\tkind=0x0b00\tbbox=-1,-2,10,20\tvalid-vector-offset=true\t"
    ));
    assert!(
        stdout.contains("image-signatures=png@36\tsegment-image-signatures=png@4\toffset-field-refs=-\tdecoded=false")
    );
    assert!(stdout.contains(
        "summary\tindexes=1\tentries=2\tentries-with-image=1\timage-hits=1\toffset-field-ref-rows=0\toffset-field-refs=0\tmissing-vectors=0\tdecoded=false"
    ));
}

#[test]
fn object_fdm_image_candidates_command_reports_unplaced_image_segments() {
    let path = object_fdm_index_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-image-candidates")
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
        "object-fdm-image-candidate\tsource=/FigureData/main_data/FDMVector\tindex=/FigureData/main_data/FDMIndex\trow=1\tvector-offset=32\tnext-vector-offset=103\tvector-length=71\tkind=0x0b00\tbbox=-1,-2,10,20\tnormalized-bbox=-1,-2,10,20\tbbox-size=11x22\tbbox-order=forward\tbbox-plausible=true\timage-hits=1\tcomplete-payloads=1"
    ));
    assert!(stdout.contains(
        "image-signatures=png@36\tsegment-image-signatures=png@4\trenderable=false\treason=page-placement-unproven\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tsources=1\tcandidates=1\timage-hits=1\tcomplete-payloads=1\tbbox-plausible=1\trenderable=0\tdecoded=false"
    ));
}

#[test]
fn object_fdm_image_candidates_command_reports_signature_only_blocker() {
    let path = object_fdm_signature_only_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-image-candidates")
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
        "object-fdm-image-candidate\tsource=/FigureData/main_data/FDMVector\tindex=/FigureData/main_data/FDMIndex\trow=1\tvector-offset=32\tnext-vector-offset=53\tvector-length=21\tkind=0x0b00"
    ));
    assert!(stdout.contains(
        "image-hits=1\tcomplete-payloads=0\timage-signatures=png@36\tsegment-image-signatures=png@4\trenderable=false\treason=image-signature-without-complete-payload-role-unproven\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tsources=1\tcandidates=1\timage-hits=1\tcomplete-payloads=0\tbbox-plausible=1\trenderable=0\tdecoded=false"
    ));
}

#[test]
fn object_fdm_frame_links_command_connects_fdm_rows_to_frame_records() {
    let path = object_fdm_frame_link_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-frame-links")
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
            "object-fdm-frame-link\tsource=/FigureData/main_data/FDMVector\tindex=/FigureData/main_data/FDMIndex\trow=1\timage-hits=1\tcomplete-payloads=1\tframe-linked=true\tframe-source=/Frame\tframe-row=1\tframe-start=76\tframe-object-id=1\tframe-kind=0x0102\tframe-type=0x0007\tframe-geometry=100,200,300,400\tframe-size=300x400\tpayload-dimensions=png@36:1x1\tdimensioned-payloads=1\tbest-aspect-delta-permille=250\tlink-basis=fdm-row-index-to-frame-object-id\trenderable=false\treason=fdm-frame-linked-image-payload-placement-and-paint-order-unproven\tdecoded=false"
        ),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(
        "summary\tsources=1\tcandidates=1\tframe-linked=1\tmissing-frame=0\tcomplete-payloads=1\trenderable=0\tdecoded=false"
    ));
}

#[test]
fn object_fdm_index_shape_command_classifies_declared_prefix_rows() {
    let path = object_fdm_index_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-index-shape")
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
        "object-fdm-index-shape\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\tindex-bytes=64\tvector-bytes=45\theader-family=fdm-index-v1"
    ));
    assert!(stdout.contains(
        "declared-count=1\tdeclared-plausible=true\trow22-stream-rows=2\trow22-trailing-bytes=0\tdeclared-row22=1\tpost-declared-bytes=22"
    ));
    assert!(stdout.contains(
        "all-valid=1\tall-invalid=1\tall-image-rows=1\tall-image-hits=1\tdeclared-valid=1\tdeclared-invalid=0\tdeclared-image-rows=1\tdeclared-image-hits=1"
    ));
    assert!(stdout.contains(
        "first-invalid-row=1\tfirst-invalid-offset=4294967280\tshape=row22-count-prefix\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tindexes=1\theader-v1=1\tunknown-header=0\tdeclared-plausible=1\tstream-rows=2\tstream-invalid=1\tdeclared-rows=1\tdeclared-invalid=0\tdeclared-image-hits=1\tshapes=row22-count-prefix:1\tdecoded=false"
    ));
}

#[test]
fn object_fdm_index_rows_command_classifies_coordinate_like_invalid_rows() {
    let path = object_fdm_index_mixed_rows_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-index-rows")
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
        "object-fdm-index-row\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=0\tscope=declared\trole=vector-segment\tindex-offset=20\tvector-offset=32"
    ));
    assert!(stdout.contains(
        "object-fdm-index-row\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=1\tscope=declared\trole=coordinate-like-invalid\tindex-offset=42\tvector-offset=100728831"
    ));
    assert!(stdout.contains(
        "be16=0x0600,0xffff,0xd3c0,0xffff,0xd5bc,0xffff,0xc028,0xffff,0xc221,0x0000,0x0040\ti16=1536,-1,-11328,-1,-10820,-1,-16344,-1,-15839,0,64"
    ));
    assert!(stdout.contains(
        "object-fdm-index-row\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=2\tscope=post-declared\trole=coordinate-like-invalid"
    ));
    assert!(stdout.contains(
        "object-fdm-index-rows-summary\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\tindex-bytes=86\tvector-bytes=45\theader-family=fdm-index-v1\tdeclared-count=2\trows=3\tdeclared-rows=2\tpost-declared-rows=1\traw-rows=0\tvalid-rows=1\tinvalid-rows=2\timage-hits=1\toffset-field-ref-rows=0\toffset-field-refs=0\troles=coordinate-like-invalid:2,vector-segment:1\tvector-missing=false\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tindexes=1\trows=3\tdeclared-rows=2\tpost-declared-rows=1\traw-rows=0\tvalid-rows=1\tinvalid-rows=2\timage-hits=1\toffset-field-ref-rows=0\toffset-field-refs=0\tmissing-vectors=0\troles=coordinate-like-invalid:2,vector-segment:1\tdecoded=false"
    ));
}

#[test]
fn local_success_data_test_fdm_index_rows_report_offset_field_references_when_available() {
    let sample_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("rjtd-testdata/local-samples")
        .join("ichitaro-20030228030923-success-002-success_data-test.jtd");
    if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
        return;
    }

    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-index-rows")
        .arg(&sample_path)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("offset-field-ref-rows="),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("offset-field-refs=bbox.left:command:308"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("offset-field-refs=bbox.left:segment:1864->[1924,1958,1992,2024]"),
        "stdout: {stdout}"
    );
}
