use std::fs;
use std::process::Command;

use super::support::*;

#[test]
fn stream_words_command_reports_big_endian_words() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-words")
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
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "0\t0\t0x646f\n");
}

#[test]
fn stream_word_frequencies_command_reports_big_endian_counts() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-word-frequencies")
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
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "1\t0x6865\n1\t0x6c6c\n"
    );
}

#[test]
fn stream_dwords_command_reports_big_endian_dwords() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-dwords")
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
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "0\t0\t0x68656c6c\n"
    );
}

#[test]
fn stream_dword_frequencies_command_reports_big_endian_counts() {
    let path = raw_stream_path(b"AAAABBBBAAAAzz");
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-dword-frequencies")
        .arg(&path)
        .arg("/Raw")
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
        "2\t0x41414141\n1\t0x42424242\n"
    );
}

#[test]
fn stream_text_probe_reports_ascii_and_utf16_candidates() {
    let path = text_probe_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-text-probe")
        .arg(&path)
        .arg("/Raw")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("ascii\t1\tVer.2.3\n"));
    assert!(stdout.contains("utf16le\t"));
    assert!(stdout.contains("Layout"));
    assert!(stdout.contains("utf16be\t"));
    assert!(stdout.contains("Wide"));
}

#[test]
fn stream_find_command_reports_exact_stream_matches() {
    let path = duplicate_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-find")
        .arg(&path)
        .arg("/Needle")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("needle\t/Needle\t6\n"));
    assert!(stdout.contains("match\t/Haystack\t2\t6\n"));
    assert!(stdout.contains("match\t/Haystack\t8\t6\n"));
    assert!(stdout.contains("match\t/Needle\t0\t6\n"));
}

#[test]
fn stream_find_bytes_command_reports_hex_matches() {
    let path = duplicate_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-find-bytes")
        .arg(&path)
        .arg("0x6e 65_65 64")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("needle\t6e656564\t4\n"));
    assert!(stdout.contains("match\t/Haystack\t2\t4\n"));
    assert!(stdout.contains("match\t/Haystack\t8\t4\n"));
    assert!(stdout.contains("match\t/Needle\t0\t4\n"));
}
