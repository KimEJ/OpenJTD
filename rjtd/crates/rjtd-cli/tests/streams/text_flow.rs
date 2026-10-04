use std::{fs, process::Command};

use super::support::*;

#[test]
fn json_export_preserves_source_flow_beside_derived_table_candidates() {
    let path = text_count_table_candidate_path();
    let document = rjtd_model::parse_document(&fs::read(&path).unwrap()).unwrap();
    let flow = document.document_text_flow().unwrap();
    assert_eq!(flow.source(), "/DocumentText");
    assert_eq!(
        flow.events()
            .iter()
            .map(|event| event.text())
            .collect::<String>(),
        "銀河鉄道\n"
    );
    assert_eq!(document.table_candidates().len(), 2);
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .args(["export"])
        .arg(&path)
        .args(["--format", "json"])
        .output()
        .unwrap();
    fs::remove_file(&path).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json = String::from_utf8(output.stdout).unwrap();
    assert_json_brackets_balanced(&json);
    assert!(json.contains("\"documentTextFlow\":{\"source\":\"/DocumentText\",\"decoded\":false"));
    assert!(json.contains("\"kind\":\"opaque\""));
    assert!(json.contains("\"kind\":\"control\""));
    assert!(json.contains("\"sourceSpan\":{\"byteStart\":"));
    assert!(json.contains("\"tableCandidates\":["));
}
