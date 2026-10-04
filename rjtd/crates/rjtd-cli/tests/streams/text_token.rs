use std::fs;
use std::path::Path;
use std::process::Command;

use super::support::*;

#[test]
fn cat_command_extracts_document_text_runs() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cat")
        .arg(&path)
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
#[ignore = "requires local document samples"]
fn cat_command_extracts_native_ichitaro_paragraphs() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../rjtd-testdata/local-samples/native-fixtures/native-text-001.jtd");
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cat")
        .arg(path)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    // This case checks paragraph text, not native indentation or page geometry.
    let paragraphs = stdout.lines().map(str::trim_start).collect::<Vec<_>>();
    assert_eq!(
        paragraphs,
        [
            "OpenJTD Native Roundtrip 001",
            "これはOpenJTDのネイティブ作成確認用文書です。",
            "文書末尾確認 END-001",
        ]
    );
}

#[test]
#[ignore = "requires local document samples"]
fn native_control_table_keeps_leading_body_text_out_of_its_rows() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../rjtd-testdata/local-samples/native-fixtures/native-table-002.jtd");
    let document = rjtd_model::parse_document(&fs::read(&path).unwrap()).unwrap();
    let table = document
        .table_candidates()
        .iter()
        .find(|candidate| candidate.kind() == "documentTextControlRunTableCandidate")
        .unwrap();
    assert_eq!(table.interval_count(), 3);
    assert_eq!(table.cell_count_candidate(), 6);
    let cells = table
        .intervals()
        .iter()
        .flat_map(|row| row.column_segments())
        .map(|cell| cell.text())
        .collect::<Vec<_>>();
    assert_eq!(cells, ["R1C1", "R1C2", "R2C1", "R2C2", "R3C1", "R3C2"]);

    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .args(["page-svg"])
        .arg(path)
        .arg("0")
        .output()
        .unwrap();
    assert!(output.status.success());
    let svg = String::from_utf8(output.stdout).unwrap();
    assert_eq!(svg.matches("rjtd-native-control-table-cell").count(), 6);
    assert_eq!(svg.matches("BEFORE-TABLE").count(), 1);
    assert_eq!(svg.matches("AFTER-TABLE").count(), 1);
}

#[test]
#[ignore = "requires local document samples"]
fn native_two_row_text_only_table_has_four_cells() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../rjtd-testdata/local-samples/native-fixtures/native-table-003.jtd");
    let document = rjtd_model::parse_document(&fs::read(&path).unwrap()).unwrap();
    let table = document
        .table_candidates()
        .iter()
        .find(|candidate| candidate.kind() == "documentTextControlRunTableCandidate")
        .expect("fully framed native text-only table");
    assert_eq!(table.interval_count(), 2);
    let cells = table
        .intervals()
        .iter()
        .flat_map(|row| row.column_segments())
        .map(|cell| cell.text())
        .collect::<Vec<_>>();
    assert_eq!(cells, ["CELL-A", "CELL-B", "CELL-C", "CELL-D"]);
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-svg")
        .arg(path)
        .arg("0")
        .output()
        .unwrap();
    assert!(output.status.success());
    let svg = String::from_utf8(output.stdout).unwrap();
    assert_eq!(svg.matches("rjtd-native-control-table-cell").count(), 4);
    assert_eq!(svg.matches("BEFORE-TEXT-TABLE").count(), 1);
    assert_eq!(svg.matches("AFTER-TEXT-TABLE").count(), 1);
}

#[test]
#[ignore = "requires local document samples"]
fn native_single_row_table_preserves_cells_and_border() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../rjtd-testdata/local-samples/native-fixtures/native-table-004.jtd");
    let document = rjtd_model::parse_document(&fs::read(&path).unwrap()).unwrap();
    let table = document
        .table_candidates()
        .iter()
        .find(|candidate| candidate.kind() == "documentTextControlRunTableCandidate")
        .expect("fully framed single native row");
    assert_eq!(table.interval_count(), 1);
    let cells = table.intervals()[0]
        .column_segments()
        .iter()
        .map(|cell| cell.text())
        .collect::<Vec<_>>();
    assert_eq!(cells, ["LEFT", "RIGHT"]);
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-svg")
        .arg(path)
        .arg("0")
        .output()
        .unwrap();
    assert!(output.status.success());
    let svg = String::from_utf8(output.stdout).unwrap();
    assert_eq!(svg.matches("rjtd-native-control-table-cell").count(), 2);
    assert_eq!(svg.matches("rjtd-native-control-table-border").count(), 1);
    assert_eq!(svg.matches("BEFORE-ONE-ROW").count(), 1);
    assert_eq!(svg.matches("AFTER-ONE-ROW").count(), 1);
}

#[test]
#[ignore = "requires local document samples"]
fn native_single_column_table_preserves_two_rows() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../rjtd-testdata/local-samples/native-fixtures/native-table-005.jtd");
    let document = rjtd_model::parse_document(&fs::read(&path).unwrap()).unwrap();
    let table = document
        .table_candidates()
        .iter()
        .find(|candidate| candidate.kind() == "documentTextControlRunTableCandidate")
        .expect("fully framed native single column");
    assert_eq!(table.interval_count(), 2);
    let cells = table
        .intervals()
        .iter()
        .flat_map(|row| row.column_segments())
        .map(|cell| cell.text())
        .collect::<Vec<_>>();
    assert_eq!(cells, ["TOP", "BOTTOM"]);
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-svg")
        .arg(path)
        .arg("0")
        .output()
        .unwrap();
    assert!(output.status.success());
    let svg = String::from_utf8(output.stdout).unwrap();
    assert_eq!(svg.matches("rjtd-native-control-table-cell").count(), 2);
    assert_eq!(svg.matches("rjtd-native-control-table-border").count(), 1);
    assert_eq!(svg.matches("BEFORE-ONE-COLUMN").count(), 1);
    assert_eq!(svg.matches("AFTER-ONE-COLUMN").count(), 1);
}

#[test]
#[ignore = "requires local document samples"]
fn native_body_text_preserves_explicit_foreground_colors() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../rjtd-testdata/local-samples/native-fixtures/native-colors-006.jtd");
    let document = rjtd_model::parse_document(&fs::read(&path).unwrap()).unwrap();
    let bytes = document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/DocumentText")
        .unwrap()
        .bytes();
    let map = rjtd_core::document_text::map_document_text(bytes);
    let resolver =
        rjtd_core::document_text::DocumentTextStyleResolver::from_document_text_bytes(bytes);
    for (label, bgr) in [
        ("BLACK-TEXT", 0),
        ("RED-TEXT", 255),
        ("BLUE-TEXT", 0x00ff0000),
    ] {
        let entry = map
            .entries()
            .iter()
            .find(|entry| entry.text().contains(label))
            .unwrap();
        let start = entry.unit_start()
            + entry.text()[..entry.text().find(label).unwrap()]
                .encode_utf16()
                .count();
        let end = start + label.encode_utf16().count();
        assert_eq!(
            resolver.uniform_value_in_range(start, end, 15),
            Some(rjtd_core::document_text::DocumentTextStyleTypedValue::U32(
                bgr
            )),
            "{label}"
        );
    }
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-svg")
        .arg(path)
        .arg("0")
        .output()
        .unwrap();
    assert!(output.status.success());
    let svg = String::from_utf8(output.stdout).unwrap();
    for (label, color) in [
        ("BLACK-TEXT", "#000000"),
        ("RED-TEXT", "#ff0000"),
        ("BLUE-TEXT", "#0000ff"),
    ] {
        let element = svg
            .split("<text ")
            .find(|element| element.contains(label))
            .unwrap();
        assert!(
            element.contains(&format!("fill=\"{color}\"")),
            "{label}: {element}"
        );
    }
    assert_eq!(svg.matches("BLACK-TEXT").count(), 1);
    let layers = rjtd_model::DocumentCore::from_document(document)
        .get_page_layer_tree(0)
        .unwrap();
    for color in ["#000000", "#ff0000", "#0000ff"] {
        assert!(layers.contains(&format!("\"fillColor\":\"{color}\"")));
    }
}

#[test]
fn text_tokens_command_reports_structured_document_text() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-tokens")
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
        "text\t銀河\ncontrol\t0x001c\ntext\t鉄道\\n\n"
    );
}

#[test]
fn text_tokens_command_preserves_skipped_inline_text() {
    let path = skipped_inline_document_text_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-tokens")
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
    assert!(stdout.contains("text\t本文\n"));
    assert!(stdout.contains("skipped-inline\t0x0082\t24\tふりがな\n"));
}

#[test]
fn text_control_context_command_reports_neighboring_controls() {
    let path = control_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-context")
        .arg(&path)
        .output()
        .unwrap();
    let filtered = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-context")
        .arg(&path)
        .arg("0x000e")
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
        "control-context\t1\t0x001c\tbyte=12-14\tunit=6-7\tprev=text(-)@10-12/5-6:A\tnext=text(-)@16-18/8-9:B\tprev-control=-\tnext-control=0x000e@3,d=2,byte=18,unit=9\n"
    ));
    assert!(stdout.contains(
        "control-context\t3\t0x000e\tbyte=18-20\tunit=9-10\tprev=text(-)@16-18/8-9:B\tnext=text(-)@22-24/11-12:C\tprev-control=0x001c@1,d=-2,byte=12,unit=6\tnext-control=-\n"
    ));

    assert!(
        filtered.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&filtered.stderr)
    );
    assert_eq!(
        String::from_utf8(filtered.stdout).unwrap(),
        "control-context\t3\t0x000e\tbyte=18-20\tunit=9-10\tprev=text(-)@16-18/8-9:B\tnext=text(-)@22-24/11-12:C\tprev-control=0x001c@1,d=-2,byte=12,unit=6\tnext-control=-\n"
    );
}

#[test]
fn text_control_clusters_command_groups_adjacent_controls() {
    let path = control_cluster_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-clusters")
        .arg(&path)
        .output()
        .unwrap();
    let filtered = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-clusters")
        .arg(&path)
        .arg("0x000e")
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
            "control-cluster\t1-2\tlen=2\tcodes=0x000e,0x001d\tbyte=12-16\tunit=6-8\tprev=text(-)@10-12/5-6:A\tnext=text(-)@18-20/9-10:B\n",
            "control-cluster\t4-4\tlen=1\tcodes=0x001c\tbyte=20-22\tunit=10-11\tprev=text(-)@18-20/9-10:B\tnext=text(-)@24-26/12-13:C\n",
        )
    );

    assert!(
        filtered.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&filtered.stderr)
    );
    assert_eq!(
        String::from_utf8(filtered.stdout).unwrap(),
        "control-cluster\t1-2\tlen=2\tcodes=0x000e,0x001d\tbyte=12-16\tunit=6-8\tprev=text(-)@10-12/5-6:A\tnext=text(-)@18-20/9-10:B\n"
    );
}

#[test]
fn text_control_ranges_command_summarizes_delimited_intervals() {
    let path = control_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-ranges")
        .arg(&path)
        .output()
        .unwrap();
    let filtered = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-ranges")
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
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "control-range\t0\tdelimiter=all\tprev=start\tnext=0x001c@1,byte=12,unit=6\tentries=0-0\tbyte=10-12\tunit=5-6\tentries=1,text=1,inline=0,skipped=0,control=0,controls=-,preview=A\n",
            "control-range\t1\tdelimiter=all\tprev=0x001c@1,byte=12,unit=6\tnext=0x000e@3,byte=18,unit=9\tentries=2-2\tbyte=14-18\tunit=7-9\tentries=1,text=1,inline=0,skipped=0,control=0,controls=-,preview=B\n",
            "control-range\t2\tdelimiter=all\tprev=0x000e@3,byte=18,unit=9\tnext=end\tentries=4-4\tbyte=20-24\tunit=10-12\tentries=1,text=1,inline=0,skipped=0,control=0,controls=-,preview=C\n",
        )
    );

    assert!(
        filtered.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&filtered.stderr)
    );
    assert_eq!(
        String::from_utf8(filtered.stdout).unwrap(),
        concat!(
            "control-range\t0\tdelimiter=0x001c\tprev=start\tnext=0x001c@1,byte=12,unit=6\tentries=0-0\tbyte=10-12\tunit=5-6\tentries=1,text=1,inline=0,skipped=0,control=0,controls=-,preview=A\n",
            "control-range\t1\tdelimiter=0x001c\tprev=0x001c@1,byte=12,unit=6\tnext=end\tentries=2-4\tbyte=14-24\tunit=7-12\tentries=3,text=2,inline=0,skipped=0,control=1,controls=0x000e:1,preview=BC\n",
        )
    );
}

#[test]
fn cat_command_reports_invalid_compressed_jttc_payload() {
    let path = compressed_jttc_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cat")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("invalid data"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn cat_command_extracts_embedded_document_text() {
    let path = embedded_document_text_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cat")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Note");
}
