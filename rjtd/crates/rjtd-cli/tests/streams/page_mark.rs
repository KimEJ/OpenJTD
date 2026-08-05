use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::support::*;

#[test]
fn paper_marks_command_reports_header_and_entries() {
    let path = paper_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("paper-marks")
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
        "header\t2\t12\t1\t3\nentry\t0\t0x00010010\nentry\t1\t0x00010011\nentry\t2\t0x00010000\n"
    );
}

#[test]
fn paper_mark_shape_command_reports_row_candidates() {
    let path = paper_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("paper-mark-shape")
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
    assert!(stdout.contains("stream\t36\t36\tmini\n"));
    assert!(stdout.contains("alignment\tu32\ttrue\n"));
    assert!(stdout.contains("header\t2\t12\t1\n"));
    assert!(stdout.contains("classification\tfixed8\t3\t8\t0\n"));
    assert!(stdout.contains("candidate\tfixed8\t3\t8\t0\n"));
}

#[test]
fn page_marks_command_reports_header_and_raw_entries() {
    let path = page_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-marks")
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
        stdout.starts_with(
            "header\t2\t16\t1\t3\nfamily\tfixed84\t84\t0\nentry\t0\t0\t0000000000010000"
        )
    );
    assert!(stdout.contains("\tu16Class=zero-sentinel\n"));
    assert!(stdout.contains("\nentry\t2\t2\t0000000200010002"));
}

#[test]
fn page_mark_u16_profile_command_reports_class_counts_and_tuples() {
    let path = page_mark_u16_profile_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-mark-u16-profile")
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
    assert!(stdout.starts_with(
        "summary\tentries=4\tzero-sentinel=1\tadditive-row=1\tadditive-boundary=1\tmixed-payload=1\tdecoded=false\n"
    ));
    assert!(stdout.contains("profile\tzero-sentinel\t1\n"));
    assert!(stdout.contains("profile\tadditive-row\t1\n"));
    assert!(stdout.contains("profile\tadditive-boundary\t1\n"));
    assert!(stdout.contains("profile\tmixed-payload\t1\n"));
    assert!(
        stdout.contains("tuple\tadditive-row\t1\tw10=353/0x0161\tw13=353/0x0161\tw14=246/0x00f6")
    );
    assert!(
        stdout.contains(
            "tuple\tadditive-boundary\t1\tw10=370/0x0172\tw13=370/0x0172\tw14=185/0x00b9"
        )
    );
    assert!(stdout.contains("tuple\tmixed-payload\t1\tw10=0/0x0000\tw13=1/0x0001\tw14=2/0x0002"));
}

#[test]
fn local_pdf_backed_page_mark_u16_profiles_stay_stable_when_available() {
    let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("rjtd-testdata/local-samples");
    if !sample_dir.exists() {
        return;
    }

    let cases = [
        (
            "a5.jtd",
            "summary\tentries=75\tzero-sentinel=2\tadditive-row=68\tadditive-boundary=4\tmixed-payload=1\tdecoded=false",
            "tuple\tadditive-row\t68\tw10=353/0x0161\tw13=353/0x0161\tw14=246/0x00f6",
        ),
        (
            "46.jtd",
            "summary\tentries=97\tzero-sentinel=2\tadditive-row=90\tadditive-boundary=4\tmixed-payload=1\tdecoded=false",
            "tuple\tadditive-row\t90\tw10=353/0x0161\tw13=353/0x0161\tw14=246/0x00f6",
        ),
        (
            "ichitaro-20030228030923-success-002-success_data-test.jtd",
            "summary\tentries=2\tzero-sentinel=0\tadditive-row=0\tadditive-boundary=2\tmixed-payload=0\tdecoded=false",
            "tuple\tadditive-boundary\t2\tw10=370/0x0172\tw13=370/0x0172\tw14=185/0x00b9",
        ),
    ];

    let mut checked = 0usize;
    for (file_name, expected_summary, expected_tuple_prefix) in cases {
        let sample_path = sample_dir.join(file_name);
        if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
            continue;
        }

        let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
            .arg("page-mark-u16-profile")
            .arg(&sample_path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{} stderr: {}",
            file_name,
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.contains(expected_summary),
            "{} stdout: {}",
            file_name,
            stdout
        );
        assert!(
            stdout.contains(expected_tuple_prefix),
            "{} stdout: {}",
            file_name,
            stdout
        );
        checked += 1;
    }

    if sample_dir.join("a5.jtd").exists() && sample_dir.join("a5.pdf").exists() {
        assert!(checked >= 1);
    }
}

#[test]
fn local_pdf_backed_page_mark_pitch_profiles_stay_stable_when_available() {
    let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("rjtd-testdata/local-samples");
    if !sample_dir.exists() {
        return;
    }

    let cases = [
        (
            "a5.jtd",
            "summary\tentries=75\tpageWidthPx=559.370\tpageHeightPx=793.701\tbodyWidthPx=415.370\tbodyHeightPx=649.701\tmarginPx=72.000\tzero-sentinel=2\tadditive-row=68\tadditive-boundary=4\tmixed-payload=1\tdecoded=false",
            "entry\t5\tclass=additive-row\tpageIndex=5\tlineStart=23\tlineEnd=40\tlineCount=18\tlineGapCount=17\tpageHeightPxPerLineCount=44.094\tpageHeightPxPerLineGap=46.688\tbodyHeightPxPerLineCount=36.094\tbodyHeightPxPerLineGap=38.218",
        ),
        (
            "ichitaro-20030228030923-success-002-success_data-test.jtd",
            "summary\tentries=2\tpageWidthPx=687.874\tpageHeightPx=971.339\tbodyWidthPx=543.874\tbodyHeightPx=827.339\tmarginPx=72.000\tzero-sentinel=0\tadditive-row=0\tadditive-boundary=2\tmixed-payload=0\tdecoded=false",
            "entry\t0\tclass=additive-boundary\tpageIndex=0\tlineStart=0\tlineEnd=39\tlineCount=40\tlineGapCount=39\tpageHeightPxPerLineCount=24.283\tpageHeightPxPerLineGap=24.906\tbodyHeightPxPerLineCount=20.683\tbodyHeightPxPerLineGap=21.214",
        ),
    ];

    let mut checked = 0usize;
    for (file_name, expected_summary, expected_entry_prefix) in cases {
        let sample_path = sample_dir.join(file_name);
        if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
            continue;
        }

        let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
            .arg("page-mark-pitch-profile")
            .arg(&sample_path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{} stderr: {}",
            file_name,
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.contains(expected_summary),
            "{} stdout: {}",
            file_name,
            stdout
        );
        assert!(
            stdout.contains(expected_entry_prefix),
            "{} stdout: {}",
            file_name,
            stdout
        );
        checked += 1;
    }

    if sample_dir.join("a5.jtd").exists() && sample_dir.join("a5.pdf").exists() {
        assert!(checked >= 1);
    }
}

#[test]
fn page_marks_command_reports_variable_family_rows() {
    let path = page_mark_variable_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-marks")
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
    assert!(stdout.starts_with(
        "header\t3\t16\t2\t4\nfamily\tcount-plus-one-variable\t20\t0\nentry\t0\t0\t0000000001000000"
    ));
    assert!(stdout.contains("\nentry\t3\t3\t0000000301000003"));
}

#[test]
fn page_marks_command_reports_count_variable_family_rows() {
    let path = page_mark_count_variable_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-marks")
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
    assert!(stdout.starts_with(
        "header\t5\t16\t4\t5\nfamily\tcount-variable\t20\t0\nentry\t0\t0\t0000000002000000"
    ));
    assert!(stdout.contains("\nentry\t4\t4\t0000000402000004"));
}

#[test]
fn page_marks_command_reports_fixed84_tail_family_rows() {
    let path = page_mark_fixed84_tail_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-marks")
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
    assert!(stdout.starts_with(
        "header\t6\t16\t4\t2\nfamily\tfixed84-tail\t84\t4\nentry\t0\t0\t0000000003000000"
    ));
    assert!(stdout.contains("\nentry\t1\t1\t0000000103000001"));
    assert!(stdout.contains("\ntrailing\tdeadbeef\n"));
}

#[test]
fn page_mark_shape_command_reports_row_candidates() {
    let path = page_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-mark-shape")
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
    assert!(stdout.contains("stream\t264\t264\tmini\n"));
    assert!(stdout.contains("alignment\tu32\ttrue\n"));
    assert!(stdout.contains("header\t2\t16\t1\n"));
    assert!(stdout.contains("classification\tfixed84-count-plus-one\t3\t84\t0\n"));
    assert!(stdout.contains("candidate\tfixed84\t3\t84\t0\n"));
    assert!(stdout.contains("candidate\tcount-plus-one\t3\t84\t0\n"));
}

#[test]
fn page_mark_shape_command_classifies_variable_count_rows() {
    let path = page_mark_variable_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-mark-shape")
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
    assert!(stdout.contains("stream\t92\t92\tmini\n"));
    assert!(stdout.contains("header\t3\t16\t2\n"));
    assert!(stdout.contains("classification\tcount-plus-one-variable\t4\t20\t0\n"));
    assert!(stdout.contains("candidate\tcount-plus-one\t4\t20\t0\n"));
}

#[test]
fn page_mark_shape_command_classifies_fixed84_tail_rows() {
    let path = page_mark_fixed84_tail_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-mark-shape")
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
    assert!(stdout.contains("header\t6\t16\t4\n"));
    assert!(stdout.contains("classification\tfixed84-tail\t2\t84\t4\n"));
    assert!(stdout.contains("candidate\tfixed84\t2\t84\t4\n"));
}
