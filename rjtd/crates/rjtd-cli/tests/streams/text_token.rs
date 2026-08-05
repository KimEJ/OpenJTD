use std::fs;
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
