use super::*;
use crate::*;

fn document(second_kind: u16) -> Document {
    document_with_cache(second_kind, 5)
}

fn document_with_cache(second_kind: u16, cache_word: u16) -> Document {
    let mut rows = vec![vec![
        0x1c, 0x20, 12, 0, 0x30, 0, 0, 0, 12, 0, 0x20, 0x1f, 10,
    ]];
    for (title, label, leader) in [
        ("FIRST", "1", Some(1)),
        ("SECOND", "22", Some(second_kind)),
        ("THIRD", "333", None),
    ] {
        let mut row = title.encode_utf16().collect::<Vec<_>>();
        row.extend([
            0x1c, 0, 17, 0, 9, 375, 31, 0x90, 0, 2, 0xf81e, 0, 0, 17, 0, 0, 0x1f,
        ]);
        row.extend([0x1c, 1, 7, 0, 0, 1, 0x1d, cache_word, 0x1e, 5, 0, 1, 0x1f]);
        if let Some(kind) = leader {
            row.extend([
                0x1c, 0, 18, 0, 21, 0, 23, 0x90, kind, 2, 0xa77c, 0, 0, 0, 18, 0, 0, 0x1f,
            ]);
            row.extend([0x1c, 1, 7, 0, 0, 1, 0x1d, 3, 0x1e, 5, 0, 1, 0x1f]);
        }
        row.extend(label.encode_utf16());
        row.push(10);
        rows.push(row);
    }
    rows.push(vec![
        0x1c, 0x20, 12, 0, 0x31, 0, 0, 0, 12, 0, 0x20, 0x1f, 10,
    ]);
    rows.push("BODY".encode_utf16().collect());
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
    for (index, row) in rows.iter().enumerate() {
        line_words.extend([row.len() as u16 + u16::from(index + 1 == rows.len()), 3]);
    }
    line_words.extend([0xff4d, 2]);
    let line = line_words
        .into_iter()
        .flat_map(u16::to_be_bytes)
        .collect::<Vec<_>>();
    let mut page = Vec::new();
    for word in [0_u32, 0x10, 0] {
        page.extend(word.to_be_bytes());
    }
    let mut f = [0_u16; 42];
    f[2] = 1;
    f[7] = 39;
    for i in [10, 13, 17, 18, 19] {
        f[i] = 370;
    }
    f[14] = 222;
    f[20] = 255;
    f[21] = 592;
    for word in f {
        page.extend(word.to_be_bytes());
    }
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/LineMark", &line),
        ("/PageMark", &page),
        (
            DOCUMENT_VIEW_STYLES_PATH,
            &super::native_vertical::view(false, false),
        ),
    ]))
    .unwrap()
}

#[test]
fn saved_toc_distinguishes_solid_dotted_and_adjacent_labels_with_backend_advances() {
    let doc = document(100);
    assert_eq!(doc.toc_entries().len(), 3);
    let original = doc.clone();
    let core = DocumentCore::from_document(doc);
    let svg = core.render_page_svg(0).unwrap();
    assert_eq!(svg.matches("class=\"rjtd-native-toc-text\"").count(), 6);
    assert_eq!(svg.matches("class=\"rjtd-native-toc-leader\"").count(), 2);
    assert!(svg.contains("data-leader-kind-candidate=\"1\""));
    assert!(svg.contains("data-leader-kind-candidate=\"100\""));
    assert!(svg.contains(">BODY</text>"));
    let layout = core.page_layout_for(0);
    let lines = core.page_text_lines(0).unwrap();
    let mut measured = BTreeMap::new();
    for entry in core.document().toc_entries() {
        measured.insert(entry.source_span().unit_start(), 40.0);
        measured.insert(
            entry.source_span().unit_end() - entry.page_label().len(),
            12.0,
        );
    }
    let rendered = core.render_page_svg_with_text_widths(0, &measured).unwrap();
    let right = layout.width_px() - layout.margin_right_px() - 12.0;
    assert_eq!(rendered.matches(&format!("x=\"{right:.3}\"")).count(), 2);
    let adjacent =
        layout.margin_left_px() + 40.0 + document_default_font_size_px(core.document()).unwrap();
    assert!(rendered.contains(&format!("x=\"{adjacent:.3}\"")));
    for line in lines.iter().filter(|line| {
        line.text().ends_with("1") || line.text().ends_with("22") || line.text().ends_with("333")
    }) {
        assert!(
            native_toc_row(
                core.document(),
                layout,
                1,
                WritingMode::Horizontal,
                line,
                &measured
            )
            .is_some()
        );
    }
    let layer = core.get_page_layer_tree(0).unwrap();
    assert_json_brackets_balanced(&layer);
    assert_eq!(layer.matches("nativeTocCandidate").count(), 6);
    assert_eq!(layer.matches("tocLeaderCandidate").count(), 2);
    assert_eq!(core.document(), &original);
}

#[test]
fn unknown_toc_context_and_overlapping_widths_keep_fallback_text() {
    let unknown = DocumentCore::from_document(document(101));
    assert!(unknown.document().toc_entries().is_empty());
    let svg = unknown.render_page_svg(0).unwrap();
    assert!(!svg.contains("rjtd-native-toc"));
    assert!(svg.contains("SECOND"));
    let other_cache = DocumentCore::from_document(document_with_cache(100, 2));
    assert_eq!(other_cache.document().toc_entries().len(), 3);
    assert!(
        !other_cache
            .render_page_svg(0)
            .unwrap()
            .contains("rjtd-native-toc")
    );
    let core = DocumentCore::from_document(document(100));
    let entry = &core.document().toc_entries()[0];
    let measured = BTreeMap::from([(entry.source_span().unit_start(), 10000.0)]);
    let svg = core.render_page_svg_with_text_widths(0, &measured).unwrap();
    assert_eq!(svg.matches("class=\"rjtd-native-toc-text\"").count(), 4);
    assert!(svg.contains(">FIRST</text>"));
    for line in core.page_text_lines(0).unwrap() {
        assert!(
            native_toc_row(
                core.document(),
                core.page_layout_for(0),
                1,
                WritingMode::VerticalRl,
                line,
                &BTreeMap::new()
            )
            .is_none()
        );
    }
}
