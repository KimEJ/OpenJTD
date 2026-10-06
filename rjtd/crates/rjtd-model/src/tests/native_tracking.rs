use super::*;
use crate::*;

fn document(tracking: bool) -> Document {
    let mut view = super::native_vertical::view(false, false);
    let summary = summarize_style_stream(&view);
    let font = summary
        .records()
        .iter()
        .find(|r| r.code() == 0x1006)
        .unwrap();
    let at = font.offset() + 4 + 16;
    view[at..at + 2].copy_from_slice(&(if tracking { 0x266_u16 } else { 13 }).to_be_bytes());
    let size = summary
        .records()
        .iter()
        .find(|r| r.code() == 0x1001)
        .unwrap();
    let from = size.offset() + 4;
    let old = view[from..from + size.payload_len()].to_vec();
    let mut landscape = vec![14];
    landscape.extend(29700_u32.to_be_bytes());
    landscape.extend(21000_u32.to_be_bytes());
    landscape.extend(old);
    landscape[9] = 1;
    view[size.offset() + 2..size.offset() + 4].copy_from_slice(&267_u16.to_be_bytes());
    view.splice(from..from + size.payload_len(), landscape);
    let words = "日本ABC日本DEF".encode_utf16().collect::<Vec<_>>();
    let mut text = vec![0; 32];
    text[..8].copy_from_slice(b"SsmgV.01");
    text[20..28].copy_from_slice(b"TextV.01");
    text[28..32].copy_from_slice(&(words.len() as u32).to_be_bytes());
    for w in &words {
        text.extend(w.to_be_bytes());
    }
    text.push(0);
    text.extend((words.len() as u32).to_be_bytes());
    text.push(0xff);
    let mut line = Vec::new();
    for w in [
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
        3,
        0xff4d,
        2,
    ] {
        line.extend(w.to_be_bytes());
    }
    let mut page = Vec::new();
    for w in [0_u32, 0x10, 0] {
        page.extend(w.to_be_bytes());
    }
    let mut f = [0_u16; 42];
    f[2] = 1;
    f[7] = 39;
    for i in [10, 13, 17, 18, 19] {
        f[i] = 370;
    }
    f[14] = 5;
    f[20] = 255;
    f[21] = 375;
    for w in f {
        page.extend(w.to_be_bytes());
    }
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/LineMark", &line),
        ("/PageMark", &page),
        (DOCUMENT_VIEW_STYLES_PATH, &view),
    ]))
    .unwrap()
}

#[test]
fn landscape_spacing_splits_script_runs_and_preserves_source_text() {
    let doc = document(true);
    assert_eq!(doc.character_spacing_percent_candidate(), Some(60));
    let original = doc.clone();
    let core = DocumentCore::from_document(doc);
    let svg = core.render_page_svg(0).unwrap();
    assert_eq!(svg.matches("class=\"rjtd-native-tracking\"").count(), 4);
    assert!(svg.contains("letter-spacing=\"8.391\""));
    let units = core.page_text_advance_targets(0).unwrap();
    assert_eq!(units.len(), 2);
    let measured = units.iter().map(|u| (*u, 25.0_f32)).collect();
    let resolved = core.render_page_svg_with_text_widths(0, &measured).unwrap();
    assert!(resolved.contains("rjtd-native-tracking"));
    let layer = core.get_page_layer_tree(0).unwrap();
    assert_json_brackets_balanced(&layer);
    assert_eq!(layer.matches("nativeTrackingCandidate").count(), 4);
    assert_eq!(core.document(), &original);
}

#[test]
fn unknown_spacing_and_portrait_layout_do_not_select_landscape_projection() {
    let core = DocumentCore::from_document(document(false));
    assert_eq!(core.document().character_spacing_percent_candidate(), None);
    assert!(
        !core
            .render_page_svg(0)
            .unwrap()
            .contains("rjtd-native-tracking")
    );
    let doc = document(true);
    let core = DocumentCore::from_document(doc);
    let portrait = PageLayout::new(millimeters_to_css_px(210.0), millimeters_to_css_px(297.0));
    assert!(
        native_tracking_projection(
            core.document(),
            portrait,
            1,
            WritingMode::Horizontal,
            core.page_text_lines(0).unwrap(),
            &BTreeMap::new()
        )
        .is_none()
    );
}
