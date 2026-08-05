use std::fs;
use std::process::Command;

use super::support::*;

#[test]
fn streams_command_lists_cfb_entries() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("streams")
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
    assert!(stdout.contains("stream\t7\t/\\x04JSRV_SegmentInformation"));
    assert!(stdout.contains("storage\t0\t/BodyText"));
    assert!(stdout.contains("stream\t5\t/BodyText/Section0"));
    assert!(stdout.contains("stream\t24\t/DocumentText"));
    assert!(stdout.contains("stream\t3\t/DocInfo"));
}

#[test]
fn info_command_reports_document_text_inventory() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("info")
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
    assert!(stdout.contains("format\tcfb-document-text"));
    assert!(stdout.contains("streams\t4"));
    assert!(stdout.contains("storages\t1"));
    assert!(stdout.contains("document_text_bytes\t24"));
    assert!(stdout.contains("compressed_document_bytes\t-"));
}

#[test]
fn info_command_reports_compressed_jttc_inventory() {
    let path = compressed_jttc_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("info")
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
    assert!(stdout.contains("format\tcfb-just-compressed-document"));
    assert!(stdout.contains("document_text_bytes\t-"));
    assert!(stdout.contains("compressed_document_bytes\t38"));
}

#[test]
fn info_command_reports_embedded_document_text_inventory() {
    let path = embedded_document_text_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("info")
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
    assert!(stdout.contains("format\tcfb-embedded-document-text"));
    assert!(stdout.contains("document_text_bytes\t-"));
    assert!(stdout.contains("embedded_document_text\tpresent"));
}

#[test]
fn dump_stream_command_writes_raw_stream_to_stdout() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("dump-stream")
        .arg(&path)
        .arg("/BodyText/Section0")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"hello");
}

#[test]
fn dump_stream_command_accepts_escaped_control_path() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("dump-stream")
        .arg(&path)
        .arg("/\\x04JSRV_SegmentInformation")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"segment");
}

#[test]
fn cfb_map_command_reports_special_chains() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cfb-map")
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
    assert!(stdout.contains("sector_size\t"));
    assert!(stdout.contains("mini_stream_cutoff\t"));
    assert!(stdout.contains("fat_sectors\t"));
    assert!(stdout.contains("directory_chain\tcomplete\t"));
    assert!(stdout.contains("root_mini_stream\t"));
    assert!(stdout.contains("mini_stream_chain\tcomplete\t"));
}

#[test]
fn cfb_dir_command_reports_raw_directory_entries() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cfb-dir")
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
    assert!(stdout.contains("\troot\t"));
    assert!(stdout.contains("\tstream\t3\t"));
    assert!(stdout.contains("\t/DocInfo\tDocInfo\t7\n"));
}

#[test]
fn stream_meta_command_reports_storage_location() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-meta")
        .arg(&path)
        .arg("/DocInfo")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("path\t/DocInfo\n"));
    assert!(stdout.contains("size\t3\n"));
    assert!(stdout.contains("storage\tmini\n"));
    assert!(stdout.contains("mini_stream_cutoff\t"));
    assert!(stdout.contains("mini_stream_bytes\t"));
}

#[test]
fn stream_chain_command_reports_sector_chain() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-chain")
        .arg(&path)
        .arg("/DocInfo")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("path\t/DocInfo\n"));
    assert!(stdout.contains("storage\tmini\n"));
    assert!(stdout.contains("declared_size\t3\n"));
    assert!(stdout.contains("sector_size\t64\n"));
    assert!(stdout.contains("offset_basis\tmini-stream\n"));
    assert!(stdout.contains("status\tcomplete\n"));
    assert!(stdout.contains("sector\t0\t"));
}
