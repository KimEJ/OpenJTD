use super::*;
use crate::*;

fn running_document(facing: bool, cover_off: bool, number_flag: u32) -> Document {
    let values: Vec<(u32, &str)> = if facing {
        vec![
            (0, "- ? -"),
            (1, "ODD<&"),
            (2, "EVEN"),
            (5, "BOTTOM"),
            (6, "BOTTOM"),
        ]
    } else {
        vec![(0, "- ? -"), (1, "TOP<&"), (5, "BOTTOM")]
    };
    let mut header = vec![0; 16 + values.len() * 536 + 8];
    header[..8].copy_from_slice(b"SsmgV.01");
    header[8..12].copy_from_slice(&(values.len() as u32).to_be_bytes());
    header[12..16].copy_from_slice(&256_u32.to_be_bytes());
    header[16..20].copy_from_slice(&(values.len() as u32 * 2).to_be_bytes());
    let table = 16 + values.len() * 512;
    for (index, (id, text)) in values.iter().enumerate() {
        let slot = 16 + index * 512;
        let words = text.encode_utf16().collect::<Vec<_>>();
        header[slot + 4..slot + 12].copy_from_slice(b"TextV.01");
        header[slot + 12..slot + 16].copy_from_slice(&(words.len() as u32).to_be_bytes());
        for (i, word) in words.iter().enumerate() {
            header[slot + 16 + 2 * i..slot + 18 + 2 * i].copy_from_slice(&word.to_be_bytes());
        }
        let end = slot + 16 + words.len() * 2;
        header[end + 1..end + 5].copy_from_slice(&(words.len() as u32).to_be_bytes());
        header[slot + 260..slot + 268].copy_from_slice(b"TCntV.01");
        for (i, value) in [*id, 0, 17 + words.len() as u32 * 2, 1, 1, index as u32 * 2]
            .into_iter()
            .enumerate()
        {
            let at = table + 4 + index * 24 + i * 4;
            header[at..at + 4].copy_from_slice(&value.to_be_bytes());
        }
    }
    let mut view = vec![0, 1, 0, 2, 0x10, 0, 0, 0, 0, 0];
    let mut record = |code: u16, payload: &[u8]| {
        view.extend(code.to_be_bytes());
        view.extend((payload.len() as u16).to_be_bytes());
        view.extend(payload);
    };
    let mut size = vec![0; 258];
    size[..6].copy_from_slice(&[0, 4, 1, 0, 0, 0]);
    for (offset, value) in [(126, 21000_u32), (130, 29700), (154, 21000), (158, 29700)] {
        size[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    record(0x1001, &size);
    let mut margins = vec![0; 32];
    margins[1] = 0xd8;
    for offset in [2, 4, 6, 8] {
        margins[offset..offset + 2].copy_from_slice(&3000_u16.to_be_bytes());
    }
    record(0x1002, &margins);
    let mut font = vec![0; 21];
    font[0] = 0x1f;
    font[3..5].copy_from_slice(&370_u16.to_be_bytes());
    record(0x1006, &font);
    record(
        0x1007,
        &[
            0x84,
            7,
            0,
            0,
            0x84,
            if facing || cover_off { 0x3b } else { 0x0b },
            7,
            5,
            1,
            1,
            0,
        ],
    );
    record(
        0x1011,
        if cover_off {
            &[2, 0, 0, 0, 1, 0, 0x10, 0, 0]
        } else {
            &[0, 0, 0x10, 0, 0]
        },
    );
    let mode = if facing { 3 } else { 0 };
    for group in [0x31, 0x32, 0x33, 0x36, 0x37] {
        record(
            group * 256 + 4,
            &[0, 0, 0x40, 0x80, 0, 0x30, 0x80, 0, 0x80, 0, 2, 0x80, 0, 0],
        );
        record(group * 256 + 5, &[0, 0, 0, 0, 0, 0, 0x40, 0, 0]);
        let payload = match group {
            0x31 => vec![3, 0, 9, 1, 7, 8],
            0x32 => vec![3, mode, 0x0b, if facing { 2 } else { 1 }, 0, 7, 0xd0],
            0x33 if facing => vec![3, mode, 0x0b, 3, 0, 7, 0xd0],
            0x33 => vec![3, 0, 0x0a, 0, 7, 0xd0],
            0x36 => vec![3, mode, 0x19, if facing { 2 } else { 1 }, 0, 100, 3, 0xe8],
            0x37 if facing => vec![3, mode, 0x19, 3, 0, 100, 3, 0xe8],
            _ => vec![3, 0, 0x18, 0, 100, 3, 0xe8],
        };
        record(group * 256 + 6, &payload);
        let mut p = [0; 21];
        p[1] = 0x3f;
        p[19] = u8::from(group == 0x31);
        record(group * 256 + 7, &p);
    }
    let extent = view.len() as u32 - 6;
    view[6..10].copy_from_slice(&extent.to_be_bytes());
    view.extend([0xff, 0xff, 0, 0, 0x10, 1]);
    view.extend(24_u32.to_be_bytes());
    view.extend([
        0x20, 4, 0, 0x10, 0, 0, 0x44, 0xff, 0xfe, 0x80, 0, 0x30, 0x80, 0, 0x80, 0, 2, 0x80, 0, 0,
        0xff, 0xff, 0, 0,
    ]);
    view.extend(0x1002_u16.to_be_bytes());
    view.extend(2026_u32.to_be_bytes());
    let mut settings = vec![0; 2026];
    settings[..8].copy_from_slice(&[0, 3, 0, 8, 0, 0, 0, 1]);
    settings[8..12].copy_from_slice(&number_flag.to_be_bytes());
    view.extend(settings);
    let mut text = vec![0; 32];
    text[..8].copy_from_slice(b"SsmgV.01");
    text[20..28].copy_from_slice(b"TextV.01");
    text[28..32].copy_from_slice(&4_u32.to_be_bytes());
    for word in "BODY".encode_utf16() {
        text.extend(word.to_be_bytes());
    }
    text.push(0);
    text.extend(4_u32.to_be_bytes());
    text.push(0xff);
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/Header", &header),
        ("/DocumentViewStyles", &view),
    ]))
    .unwrap()
}

#[test]
fn running_slots_bind_parity_and_share_source_evidence_in_svg_and_layer_tree() {
    let document = running_document(true, false, 2);
    let original = document.clone();
    let core = DocumentCore::from_document(document);
    let layout = core.page_layout_for(0);
    let odd = native_running_text(core.document(), layout, WritingMode::Horizontal, 1).unwrap();
    let even = native_running_text(core.document(), layout, WritingMode::Horizontal, 2).unwrap();
    assert_eq!(odd.len(), 3);
    assert_eq!(even.len(), 3);
    let mut svg = String::new();
    push_native_running_svg(&mut svg, &odd, "font");
    assert!(svg.contains("ODD&lt;&amp;"));
    assert!(svg.contains("text-anchor=\"start\""));
    assert!(svg.contains("- 1 -"));
    let mut layer = String::new();
    push_native_running_layer_json(&mut layer, &even, "font");
    assert!(layer.contains("\"slotId\":2"));
    assert!(layer.contains("\"textAnchor\":\"end\""));
    assert!(layer.contains("- 2 -"));
    assert!(layer.contains("\"geometryDecoded\":false"));
    let rendered = core.render_page_svg(0).unwrap();
    let layer = core.get_page_layer_tree(0).unwrap();
    assert!(rendered.contains("rjtd-native-running-header"));
    assert!(layer.contains("runningTextCandidate"));
    assert_json_brackets_balanced(&layer);
    assert_eq!(core.document(), &original);
}

#[test]
fn running_cover_suppression_keeps_page_numbers_and_rejects_unknown_configuration() {
    let core = DocumentCore::from_document(running_document(false, true, 2));
    let layout = core.page_layout_for(0);
    assert!(
        native_running_text(core.document(), layout, WritingMode::Horizontal, 1)
            .unwrap()
            .is_empty()
    );
    let second = native_running_text(core.document(), layout, WritingMode::Horizontal, 2).unwrap();
    let mut svg = String::new();
    push_native_running_svg(&mut svg, &second, "font");
    assert!(svg.contains("- 2 -"));
    let disabled = running_document(false, false, 0);
    assert_eq!(
        native_running_text(&disabled, layout, WritingMode::Horizontal, 1)
            .unwrap()
            .len(),
        2
    );
    assert!(
        native_running_text(
            &running_document(false, false, 4),
            layout,
            WritingMode::Horizontal,
            1
        )
        .is_none()
    );
    let mut duplicate = disabled.clone();
    duplicate.push_raw_stream(
        disabled
            .raw_streams()
            .iter()
            .find(|stream| stream.name() == "/Header")
            .unwrap()
            .clone(),
    );
    assert!(duplicate.header_text_candidates().is_empty());
    let mut explicit_style = disabled.clone();
    explicit_style.push_unknown_style(UnknownStyle::from_stream(PAGE_LAYOUT_STYLE_PATH, vec![]));
    assert!(native_running_text(&explicit_style, layout, WritingMode::Horizontal, 1).is_none());
    assert!(native_running_text(&disabled, layout, WritingMode::VerticalRl, 1).is_none());
    let mut truncated = disabled.clone();
    let view = truncated
        .unknown_styles
        .iter_mut()
        .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))
        .unwrap();
    let bytes = view.payload()[..view.payload().len() - 1].to_vec();
    *view = UnknownStyle::from_stream(DOCUMENT_VIEW_STYLES_PATH, bytes);
    assert!(native_running_text(&truncated, layout, WritingMode::Horizontal, 1).is_none());
}
