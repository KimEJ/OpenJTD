use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::support::*;

#[test]
fn page_layer_tree_command_reports_facing_page_decoration_evidence() {
    let path = a5_page_layer_tree_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-layer-tree")
        .arg(&path)
        .arg("5")
        .output()
        .unwrap();

    let parent = path.parent().map(PathBuf::from);
    fs::remove_file(&path).unwrap();
    if let Some(parent) = parent {
        fs::remove_dir(parent).unwrap();
    }

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_json_brackets_balanced(&stdout);
    assert!(stdout.contains("]},\"textSources\""));
    assert!(stdout.contains("\"type\":\"pageDecoration\""));
    assert!(stdout.contains("\"source\":\"autoTextInfo+pageLayoutStylePairedSlots+documentText\""));
    assert!(stdout.contains("\"projectionKind\":\"layoutStyleAutoTextProjection\""));
    assert!(stdout.contains("\"decoded\":false"));
    assert!(stdout.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\""));
    assert!(stdout.contains("\"sidePolicyDecoded\":false"));
    assert!(stdout.contains("\"facingPagesCandidate\":true"));
    assert!(stdout.contains("\"pairedSlotPairs\":[\"0x32/0x33\"]"));
    assert!(stdout.contains("\"slotEvidence\""));
    assert!(stdout.contains("\"side\":\"left\""));
    assert!(stdout.contains("\"pageNumber\":6"));
    assert!(stdout.contains("\"headerText\":\"一、午后の授業\""));
}

#[test]
fn page_info_command_reports_page_metrics_and_mark_context() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-info")
        .arg(&path)
        .arg("0")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_json_brackets_balanced(&stdout);
    assert!(stdout.contains("\"pageIndex\":0"));
    assert!(stdout.contains("\"pageNumber\":1"));
    assert!(stdout.contains("\"columns\":[{\"x\":72.0,\"width\":650.0}]"));
    assert!(stdout.contains("\"layoutMarkEvidence\":null"));
}

#[test]
fn document_info_command_reports_document_view_styles_writing_mode_candidate() {
    let path = document_info_path_with_document_view_styles(0x1001);
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("document-info")
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
    assert_json_brackets_balanced(&stdout);
    assert!(stdout.contains("\"writingMode\":\"horizontal\""));
    assert!(stdout.contains(
        "\"writingModeDecision\":{\"selected\":\"horizontal\",\"source\":\"default-horizontal\""
    ));
    assert!(stdout.contains("\"documentViewStylesCandidate\":\"vertical-rl\""));
    assert!(stdout.contains("\"documentViewStylesDisagreesWithSelected\":true"));
    assert!(stdout.contains("\"writingModeCandidateFromDocumentViewStyles\":\"vertical-rl\""));
    assert!(stdout.contains("\"writingModeCandidateFromDocumentViewStylesSourceBacked\":true"));
    assert!(stdout.contains("\"writingModeCandidateFromDocumentViewStylesFirstRecordCode\":4097"));
    assert!(
        stdout.contains(
            "\"writingModeCandidateFromDocumentViewStylesFirstRecordCodeHex\":\"0x1001\""
        )
    );
}

#[test]
fn document_info_command_reports_paper_mark_bit_diagnostics() {
    let path = paper_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("document-info")
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
    assert_json_brackets_balanced(&stdout);
    assert!(stdout.contains("\"writingModeCandidateFromPaperMark\":\"vertical-rl\""));
    assert!(stdout.contains("\"writingModeCandidateDecoded\":false"));
    assert!(stdout.contains("\"paperMarkFlagBit0VerticalCandidate\":true"));
    assert!(stdout.contains("\"paperMarkFlagBit17IndexStepCandidate\":false"));
    assert!(stdout.contains(
        "\"paperMarkWritingModeCandidateEvidence\":[\"paper-mark-flag-bit0-vertical-corpus-consistent\"]"
    ));
    assert!(stdout.contains(
        "\"paperMarkWritingModeCandidateBlockers\":[\"paper-mark-writing-mode-flag-semantics-unproven\"]"
    ));
}

#[test]
fn page_svg_command_writes_rendered_svg_page() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-svg")
        .arg(&path)
        .arg("0")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with("<svg "));
    assert!(stdout.contains("class=\"rjtd-text\""));
    assert!(stdout.contains(">銀河</text>"));
}

#[test]
fn export_command_marks_embedded_document_text_source() {
    let path = embedded_document_text_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"text\":\"Note\""));
    assert!(stdout.contains("\"rawStreams\":[{\"name\":\"/EmbeddedDocumentText\""));
}

#[test]
fn export_command_writes_json_from_document_model() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"type\":\"paragraph\""));
    assert!(stdout.contains("\"text\":\"銀河\""));
    assert!(stdout.contains("\"text\":\"鉄道\""));
    assert!(stdout.contains("\"sourceSpan\":{\"byteStart\":10,\"byteEnd\":14"));
    assert!(stdout.contains("\"rawStreams\":[{\"name\":\"/DocumentText\",\"size\":24}]"));
}

#[test]
fn export_command_writes_markdown_from_document_model() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("md")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "銀河鉄道\n\n");
}

#[test]
fn export_command_writes_text_from_document_model() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("text")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "銀河鉄道\n");
}

#[test]
fn export_command_accepts_txt_alias() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("txt")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "銀河鉄道\n");
}

#[test]
fn export_command_writes_html_from_document_model() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("html")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("<p>銀河鉄道</p>"));
    assert!(stdout.starts_with("<!DOCTYPE html>"));
}

#[test]
fn export_command_writes_pdf_from_document_model() {
    let path = tiny_cfb_path();
    let output_path = path.with_extension("pdf");
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("pdf")
        .arg("-o")
        .arg(&output_path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let pdf = fs::read(&output_path).unwrap();
    fs::remove_file(&output_path).unwrap();

    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.ends_with(b"%%EOF"));
}

#[cfg(target_os = "macos")]
#[test]
fn export_command_replaces_quarantined_pdf_output_file() {
    let path = tiny_cfb_path();
    let output_path = path.with_extension("pdf");
    fs::write(&output_path, b"stale pdf").unwrap();
    let xattr_output = Command::new("xattr")
        .arg("-w")
        .arg("com.apple.quarantine")
        .arg("0081;00000000;;")
        .arg(&output_path)
        .output()
        .unwrap();
    assert!(
        xattr_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&xattr_output.stderr)
    );

    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("pdf")
        .arg("-o")
        .arg(&output_path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let pdf = fs::read(&output_path).unwrap();
    let quarantine = Command::new("xattr")
        .arg("-p")
        .arg("com.apple.quarantine")
        .arg(&output_path)
        .output()
        .unwrap();
    fs::remove_file(&output_path).unwrap();

    assert!(pdf.starts_with(b"%PDF-"));
    assert!(
        !quarantine.status.success(),
        "quarantine xattr survived PDF output replacement: {}",
        String::from_utf8_lossy(&quarantine.stdout)
    );
}

#[test]
fn export_command_rejects_pdf_without_output_path() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("pdf")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("PDF export requires"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn export_command_rejects_unknown_format() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("docx")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("unsupported export format: docx"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
