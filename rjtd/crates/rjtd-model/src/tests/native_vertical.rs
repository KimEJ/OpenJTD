use super::*;
use crate::*;

pub(super) fn view(vertical: bool, unknown_flag: bool) -> Vec<u8> {
    let mut bytes = vec![0; 10];
    let mut record = |code: u16, payload: &[u8]| {
        bytes.extend(code.to_be_bytes());
        bytes.extend((payload.len() as u16).to_be_bytes());
        bytes.extend(payload);
    };
    let mut size = vec![0; 258];
    size[..6].copy_from_slice(&[0, 4, 1, 0, 0, 0]);
    for (offset, value) in [(126, 21000_u32), (130, 29700), (154, 21000), (158, 29700)] {
        size[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    record(0x1001, &size);
    let mut margins = vec![0, 0xd8];
    for _ in 0..4 {
        margins.extend(3000_u16.to_be_bytes());
    }
    margins.push(if unknown_flag {
        0x70
    } else if vertical {
        0x50
    } else {
        0x40
    });
    if vertical {
        margins.push(1);
    }
    for _ in 0..10 {
        margins.extend(700_u16.to_be_bytes());
    }
    margins.push(0);
    record(0x1002, &margins);
    let mut font = vec![0; 20];
    font[0] = 0x1f;
    font[3..5].copy_from_slice(&370_u16.to_be_bytes());
    font[6] = 2;
    font[16..18].copy_from_slice(&(if vertical { 0x266_u16 } else { 13 }).to_be_bytes());
    record(0x1006, &font);
    record(0x100b, &[2, 0, 13, 0, 4, 0, 0, 0, 8]);
    bytes
}

fn tate_document(value: &str, tag: u16, visible_cache: &str, mode: u16) -> Document {
    let mut words = vec![
        0x65e5, 0x672c, 0x1c, 0, 12, 0, tag, 0, 5, mode, 12, 0, 0, 0x1f,
    ];
    for (index, text) in [(0, visible_cache), (1, value)] {
        words.extend([
            0x1c,
            1,
            7,
            0,
            index,
            if index == 0 { 1 } else { 0x101 },
            0x1d,
        ]);
        words.extend(text.encode_utf16());
        words.extend([0x1e, 5, 0, 1, 0x1f]);
    }
    words.extend("、123日".encode_utf16());
    let mut text = vec![0; 32];
    text[..8].copy_from_slice(b"SsmgV.01");
    text[20..28].copy_from_slice(b"TextV.01");
    text[28..32].copy_from_slice(&(words.len() as u32).to_be_bytes());
    for word in &words {
        text.extend(word.to_be_bytes());
    }
    text.push(0);
    text.extend((words.len() as u32).to_be_bytes());
    text.push(0xff);
    let mut line = Vec::new();
    for word in [
        0x915_u16,
        0,
        1,
        0,
        2,
        0,
        2,
        0,
        1,
        words.len() as u16 + 1,
        2,
        0xff4d,
        2,
    ] {
        line.extend(word.to_be_bytes());
    }
    let mut page = Vec::new();
    for word in [0_u32, 0x10, 0] {
        page.extend(word.to_be_bytes());
    }
    let mut fields = [0_u16; 42];
    fields[2] = 1;
    fields[7] = 39;
    fields[10] = 370;
    fields[13] = 370;
    fields[14] = 5;
    fields[17..20].fill(370);
    fields[20] = 255;
    fields[21] = 375;
    for field in fields {
        page.extend(field.to_be_bytes());
    }
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/LineMark", &line),
        ("/PageMark", &page),
        (DOCUMENT_VIEW_STYLES_PATH, &view(true, false)),
    ]))
    .unwrap()
}

#[test]
fn modern_direction_uses_margin_profile_and_keeps_unknown_shapes_fallback() {
    for (vertical, expected) in [
        (false, WritingMode::Horizontal),
        (true, WritingMode::VerticalRl),
    ] {
        let mut doc = Document::from_plain_text("日本");
        doc.push_unknown_style(UnknownStyle::from_stream(
            DOCUMENT_VIEW_STYLES_PATH,
            view(vertical, false),
        ));
        // Editor/caret data is not required to select the corroborated view profile.
        let core = DocumentCore::from_document(doc.clone());
        assert_eq!(core.writing_mode(), expected);
        assert!((core.page_layout.margin_top_px() - millimeters_to_css_px(30.0)).abs() < 0.01);
        assert!(
            core.get_document_info()
                .contains("modern-view-margin-direction")
        );
        doc.push_unknown_style(UnknownStyle::from_stream(
            DOCUMENT_VIEW_STYLES_PATH,
            view(vertical, false),
        ));
        assert!(modern_view_writing_mode(doc.unknown_styles()).is_none());
    }
    let unknown = vec![UnknownStyle::from_stream(
        DOCUMENT_VIEW_STYLES_PATH,
        view(true, true),
    )];
    assert!(modern_view_writing_mode(&unknown).is_none());
}

#[test]
fn two_digit_tate_cache_becomes_visible_text_without_losing_raw_evidence() {
    let doc = tate_document("12", 0x47, "", 0x210);
    assert_eq!(
        paragraph_text(paragraph_by_index(&doc, 0).unwrap()),
        "日本12、123日"
    );
    assert_eq!(doc.unknown_objects().len(), 1);
    let candidates = doc.tatechuyoko_candidates();
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].text(), "12");
    let span = candidates[0].value_span();
    assert_eq!(
        native_visible_text_span(&doc, "12", span),
        Some(span.clone())
    );
    for (value, tag, base, mode) in [
        ("123", 0x47, "", 0x210),
        ("1", 0x47, "", 0x210),
        ("AB", 0x47, "", 0x210),
        ("12", 0x48, "", 0x210),
        ("12", 0x47, "", 0x211),
        ("12", 0x47, "基", 0x210),
    ] {
        assert!(
            tate_document(value, tag, base, mode)
                .tatechuyoko_candidates()
                .is_empty()
        );
    }
    let ruby = parse_document(&cfb_with_document_text(document_text_with_ruby())).unwrap();
    assert_ruby_inline(
        &paragraph_by_index(&ruby, 0).unwrap().inlines()[1],
        "午后",
        "ごご",
    );
}

#[test]
fn vertical_source_columns_and_tate_orientation_agree_in_svg_and_layer_tree() {
    let doc = tate_document("34", 0x47, "", 0x210);
    let original = doc.clone();
    let core = DocumentCore::from_document(doc);
    assert_eq!(core.writing_mode(), WritingMode::VerticalRl);
    let svg = core.render_page_svg(0).unwrap();
    assert!(svg.contains("rjtd-native-vertical-text"));
    let start = svg.find("data-tatechuyoko-candidate=\"true\"").unwrap();
    let end = svg[start..].find("</text>").unwrap();
    let tate = &svg[start..start + end];
    assert!(tate.contains("writing-mode=\"horizontal-tb\""));
    assert!(tate.ends_with(">34"));
    let layer = core.get_page_layer_tree(0).unwrap();
    assert_json_brackets_balanced(&layer);
    assert!(layer.contains("\"tatechuyokoCandidate\":true"));
    assert!(layer.contains("\"orientation\":\"vertical-rl\""));
    assert!(layer.contains("\"geometryDecoded\":false"));
    assert_eq!(core.document(), &original);
}
