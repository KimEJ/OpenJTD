use super::*;
use crate::*;

fn field_document(link: bool, argument: &str, subtype: u16) -> Document {
    let mut words = if link {
        vec![0x1c, 0, 12, 0, 0x48, 0, 805, 1, 12, 0, 0, 0x1f]
    } else {
        let mut words = vec![0x1c, 0, 28, 0, 0x33, 110, 1039, 0x98, subtype, 0xffff];
        words.resize(24, 0);
        words.extend([28, 0, 0, 0x1f]);
        words
    };
    let mut group = |value: &str, index: u16| {
        words.extend([0x1c, 1, 7, 0, index, u16::from(index == 0), 0x1d]);
        words.extend(value.encode_utf16());
        words.extend([0x1e, 5, 0, 1, 0x1f]);
    };
    group(if link { "LINK" } else { "0000/00/00" }, 0);
    group(argument, 1);
    if link {
        group("", 2);
    }
    let mut bytes = vec![0; 32];
    bytes[..8].copy_from_slice(b"SsmgV.01");
    bytes[20..28].copy_from_slice(b"TextV.01");
    bytes[28..32].copy_from_slice(&(words.len() as u32).to_be_bytes());
    for word in &words {
        bytes.extend(word.to_be_bytes());
    }
    if link {
        bytes.extend([
            0xfe, 13, 2, 0, 1, 15, 4, 0, 255, 0, 0, 20, 4, 0x80, 0, 0, 0x10, 0xff, 0,
        ]);
        bytes.extend([
            0xfe, 13, 2, 0, 0, 15, 4, 255, 255, 255, 255, 20, 4, 0, 0, 0, 0, 0xff, 0, 0,
        ]);
        bytes.extend(((words.len() - 2) as u32).to_be_bytes());
    } else {
        bytes.push(0);
        bytes.extend((words.len() as u32).to_be_bytes());
    }
    bytes.push(0xff);
    parse_document(&cfb_with_streams(&[("/DocumentText", &bytes)])).unwrap()
}

#[test]
fn printing_date_context_changes_display_without_rewriting_cached_source() {
    let document = field_document(false, "DATE", 6);
    let fields = document.text_field_candidates();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].kind(), DocumentTextFieldKind::PrintingDate);
    let original = document.clone();
    let mut core = DocumentCore::from_document(document);
    assert!(
        core.render_page_svg(0)
            .unwrap()
            .contains(">0000/00/00</text>")
    );
    core.set_print_date("2024/02/29").unwrap();
    assert!(
        core.render_page_svg(0)
            .unwrap()
            .contains(">2024/02/29</text>")
    );
    assert!(core.set_print_date("2023/02/29").is_err());
    assert!(core.set_print_date("2024/04/31").is_err());
    assert!(core.set_print_date("0000/00/00").is_err());
    let layer = core.get_page_layer_tree(0).unwrap();
    assert!(layer.contains("\"fieldValueFromContext\":true"));
    assert!(layer.contains("\"cachedFieldText\":\"0000/00/00\""));
    assert_eq!(core.document, original);
    assert!(
        field_document(false, "OTHER", 6)
            .text_field_candidates()
            .is_empty()
    );
    assert!(
        field_document(false, "DATE", 7)
            .text_field_candidates()
            .is_empty()
    );
}

#[test]
fn hyperlinks_bind_record_paint_to_visible_cache_and_reject_unknown_targets() {
    let document = field_document(true, "https://example.com", 0);
    let fields = document.text_field_candidates();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].kind(), DocumentTextFieldKind::Hyperlink);
    let core = DocumentCore::from_document(document);
    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("<a href=\"https://example.com\""));
    assert!(svg.contains("fill=\"#0000ff\""));
    assert!(svg.contains("text-decoration=\"underline\""));
    assert!(
        core.get_page_layer_tree(0)
            .unwrap()
            .contains("\"fieldKindCandidate\":\"hyperlink\"")
    );
    for target in ["javascript:alert(1)", "https://", "https://example.com\n"] {
        assert!(
            field_document(true, target, 0)
                .text_field_candidates()
                .is_empty()
        );
    }
}
