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

fn section_document(spacing: u16, scale: u16, gap: u16) -> Document {
    let mut rows = vec![
        "A\n".encode_utf16().collect::<Vec<_>>(),
        "日本ABC\u{000c}".encode_utf16().collect(),
        vec![0x1c, 0x20, 12, 0, 0x10, 1, 0, 1, 12, 0, 0x20, 0x1f, 10],
        "B\n".encode_utf16().collect(),
        "日本DEF\n".encode_utf16().collect(),
        vec![0x1c, 0x20, 12, 0, 0x10, 0, 0, 1, 12, 0, 0x20, 0x1f, 10],
        "\u{000c}C\n".encode_utf16().collect(),
        "日本GHI".encode_utf16().collect(),
    ];
    let units = rows.iter().flatten().copied().collect::<Vec<_>>();
    let mut text = vec![0; 32];
    text[..8].copy_from_slice(b"SsmgV.01");
    text[20..28].copy_from_slice(b"TextV.01");
    text[28..32].copy_from_slice(&(units.len() as u32).to_be_bytes());
    for word in &units {
        text.extend(word.to_be_bytes());
    }
    text.push(0);
    text.extend((units.len() as u32).to_be_bytes());
    text.push(0xff);
    let mut line = Vec::new();
    let mut line_words = vec![
        0x915,
        0,
        1,
        0,
        rows.len() as u16 + 1,
        0,
        2,
        0,
        rows.len() as u16,
    ];
    rows.last_mut().unwrap().push(0); // final source boundary extends one unit past text
    for row in &rows {
        line_words.extend([row.len() as u16, 3]);
    }
    line_words.extend([0xff4d, 2]);
    for word in line_words {
        line.extend(word.to_be_bytes());
    }
    let mut page = Vec::new();
    for word in [0_u32, 0x10, 2] {
        page.extend(word.to_be_bytes());
    }
    for (index, start, end) in [(0, 0, 1), (1, 2, 5), (2, 6, 7)] {
        let mut f = [0_u16; 42];
        f[1] = index;
        f[2] = if index == 1 { 5 } else { 1 };
        f[3] = if index == 1 { 0x100 } else { 0 };
        f[5] = start;
        f[7] = end;
        for at in [10, 13, 17, 18, 19] {
            f[at] = 370;
        }
        f[14] = if index == 1 { 5 } else { 222 };
        f[20] = 255;
        f[21] = 370 + f[14];
        for word in f {
            page.extend(word.to_be_bytes());
        }
    }
    let stock = page_layout_style_page_size_fixture(29700, 21000);
    let summary = summarize_style_stream(&stock);
    let record = &summary.records()[0];
    let size = record
        .subrecords()
        .iter()
        .find(|s| s.code() == 0x4001)
        .unwrap()
        .payload();
    let mut margin = vec![0xfe, 0x80, 0];
    for _ in 0..5 {
        margin.extend(3000_u16.to_be_bytes());
    }
    margin.extend([0, 0x3d, 0, 0, 0]);
    for _ in 0..10 {
        margin.extend(700_u16.to_be_bytes());
    }
    margin.extend([0, 0]);
    let mut font = vec![0, 0, 0xc1, 0, 0];
    font.extend(370_u16.to_be_bytes());
    font.extend([0x80, 0, 0, 0x3f, 0]);
    font.extend(spacing.to_be_bytes());
    font.extend(scale.to_be_bytes());
    font.extend(scale.to_be_bytes());
    font.extend([0x80, 0, 0x80, 0, 2, 0x80, 0, 0]);
    let mut line_profile = vec![0xc3, 0];
    line_profile.extend(gap.to_be_bytes());
    line_profile.extend([0, 0, 0x50, 0x40, 1, 0]);
    let mut payload = vec![0, 0, 0, 0];
    for (code, data) in [
        (0x4001_u16, size),
        (0x4002, margin.as_slice()),
        (0x4006, font.as_slice()),
        (0x400a, line_profile.as_slice()),
    ] {
        payload.extend(code.to_be_bytes());
        payload.extend((data.len() as u16).to_be_bytes());
        payload.extend(data);
    }
    let mut style = stock[..record.offset()].to_vec();
    style.extend(0x4444_u16.to_be_bytes());
    style.extend((payload.len() as u16).to_be_bytes());
    style.extend(payload);
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/LineMark", &line),
        ("/PageMark", &page),
        (PAGE_LAYOUT_STYLE_PATH, &style),
        (
            DOCUMENT_VIEW_STYLES_PATH,
            &super::native_vertical::view(false, false),
        ),
    ]))
    .unwrap()
}

#[test]
fn selected_middle_page_spacing_keeps_portrait_neighbors_and_source_intact() {
    let document = section_document(0x266, 100, 13);
    let original = document.clone();
    assert_eq!(document.character_spacing_percent_candidate(), None);
    let core = DocumentCore::from_document(document);
    assert_eq!(core.page_count(), 3);
    for page in [0, 2] {
        assert!(
            !core
                .render_page_svg(page)
                .unwrap()
                .contains("rjtd-native-tracking")
        );
    }
    let svg = core.render_page_svg(1).unwrap();
    assert!(svg.contains("data-spacing-basis=\"page-layout-style-4006\""));
    assert!(svg.contains("letter-spacing=\"8.391\""));
    let layer = core.get_page_layer_tree(1).unwrap();
    assert!(layer.contains("\"spacingBasis\":\"page-layout-style-4006\""));
    assert_json_brackets_balanced(&layer);
    assert!(!core.page_text_advance_targets(1).unwrap().is_empty());
    assert_eq!(core.document(), &original);
}

#[test]
fn unknown_middle_page_spacing_scale_or_line_profile_retains_fallback() {
    let known = DocumentCore::from_document(section_document(0x266, 100, 13));
    for (spacing, scale, gap) in [(0x267, 100, 13), (0x266, 99, 13), (0x266, 100, 14)] {
        let core = DocumentCore::from_document(section_document(spacing, scale, gap));
        assert_eq!(core.page_count(), 3);
        assert!(
            !core
                .render_page_svg(1)
                .unwrap()
                .contains("rjtd-native-tracking")
        );
        for page in [0, 2] {
            assert_eq!(
                core.render_page_svg(page).unwrap(),
                known.render_page_svg(page).unwrap()
            );
        }
    }
}
