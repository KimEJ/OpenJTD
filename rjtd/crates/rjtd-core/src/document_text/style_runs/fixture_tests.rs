use super::{DocumentTextStyleEvent, DocumentTextStyleSection, parse_document_text_style_section};
use crate::document_text::read_document_text_payload;
use std::path::{Path, PathBuf};

#[test]
fn shanai_fixture_stops_at_exact_content_coverage_when_available() {
    let shanai_path = repo_root()
        .join("rjtd-testdata/local-samples")
        .join("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");
    if !shanai_path.exists() {
        return;
    }

    let shanai = parse_fixture(&shanai_path);
    assert_eq!(shanai.content_unit_count(), 6176);
    assert_eq!(style_coverage_units(&shanai), 6176_usize);
    assert!(!shanai.truncated());
    assert!(!shanai.trailing_bytes().is_empty());
}

#[test]
fn parses_local_style_fixtures_when_available() {
    let shanai_path = repo_root()
        .join("rjtd-testdata/local-samples")
        .join("ichitaro-20030315134715-success-001-success_data-shanai_lan.jtd");
    let hyo_path = repo_root()
        .join("rjtd-testdata/local-samples")
        .join("ichitaro-20030829031540-success-004-success_data-hyo.jtd");
    let page_path = repo_root()
        .join("rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid")
        .join("PAGE 01.jtd");

    if !(shanai_path.exists() && hyo_path.exists() && page_path.exists()) {
        return;
    }

    let shanai = parse_fixture(&shanai_path);
    assert_eq!(shanai.content_unit_count(), 6176);
    assert_eq!(shanai.style_start(), 12384);
    assert!(!shanai.events().is_empty());

    let hyo = parse_fixture(&hyo_path);
    assert_eq!(hyo.content_unit_count(), 3793);
    assert_eq!(hyo.style_start(), 7618);
    let property_hexes = hyo
        .events()
        .iter()
        .filter_map(|event| match event {
            DocumentTextStyleEvent::PropertyChange(change) => Some(change.properties()),
            DocumentTextStyleEvent::Run(_) => None,
        })
        .flatten()
        .filter(|property| matches!(property.property_id(), 15..=17))
        .map(|property| bytes_to_hex(property.raw_value()))
        .collect::<Vec<_>>();
    for expected in ["00ffcc99", "000099ff", "00ffcccc", "0033ffff"] {
        assert!(
            property_hexes.iter().any(|value| value == expected),
            "expected raw property value {expected} in ids 15..17, got {property_hexes:?}"
        );
    }

    let page = parse_fixture(&page_path);
    assert_eq!(page.content_unit_count(), 336);
    assert_eq!(page.style_start(), 704);
    let property_change_count = page
        .events()
        .iter()
        .filter(|event| matches!(event, DocumentTextStyleEvent::PropertyChange(_)))
        .count();
    assert_eq!(property_change_count, 0);
}

fn style_coverage_units(section: &DocumentTextStyleSection) -> usize {
    section
        .events()
        .iter()
        .map(|event| match event {
            DocumentTextStyleEvent::Run(run) => usize::try_from(run.length()).ok().unwrap_or(0),
            DocumentTextStyleEvent::PropertyChange(change) => {
                usize::try_from(change.consumed_units()).ok().unwrap_or(0)
            }
        })
        .sum()
}

fn parse_fixture(path: &Path) -> DocumentTextStyleSection {
    let bytes = std::fs::read(path).expect("read fixture");
    let payload = read_document_text_payload(&bytes).expect("read DocumentText payload");
    parse_document_text_style_section(payload.bytes())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(nibble_to_hex(byte >> 4));
        output.push(nibble_to_hex(byte & 0x0f));
    }
    output
}

fn nibble_to_hex(nibble: u8) -> char {
    match nibble {
        0..=9 => char::from(b'0' + nibble),
        10..=15 => char::from(b'a' + (nibble - 10)),
        _ => unreachable!("hex nibble"),
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .and_then(|path| path.parent())
        .expect("repo root")
        .to_path_buf()
}
