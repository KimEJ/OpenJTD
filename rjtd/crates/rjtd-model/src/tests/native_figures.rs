use super::*;
use crate::*;

fn primitive(kind: u8, bounds: [i32; 4], nested: bool) -> Vec<u8> {
    let len = match kind {
        6 => 58,
        4 => 34,
        _ => 32,
    };
    let mut b = vec![0; len];
    b[..4].copy_from_slice(&[if nested { 0xff } else { 1 }, 0, kind, 0x60]);
    b[4..6].copy_from_slice(&(len as u16).to_be_bytes());
    b[8..10].copy_from_slice(&8_u16.to_be_bytes());
    let [l, t, r, bot] = bounds;
    if kind == 6 {
        b[6..8].copy_from_slice(&4_u16.to_be_bytes());
        b[16..18].copy_from_slice(&5_u16.to_be_bytes());
        for (i, (x, y)) in [
            (l + 4, t + 4),
            (r - 4, t + 4),
            (r - 4, bot - 4),
            (l + 4, bot - 4),
            (l + 4, t + 4),
        ]
        .into_iter()
        .enumerate()
        {
            b[18 + i * 8..22 + i * 8].copy_from_slice(&x.to_be_bytes());
            b[22 + i * 8..26 + i * 8].copy_from_slice(&y.to_be_bytes());
        }
    } else if kind == 4 {
        b[16..20].copy_from_slice(&((l + r) / 2).to_be_bytes());
        b[20..24].copy_from_slice(&((t + bot) / 2).to_be_bytes());
        b[24..26].copy_from_slice(&(((r - l) / 2 - 4) as u16).to_be_bytes());
        b[26..28].copy_from_slice(&(((bot - t) / 2 - 4) as u16).to_be_bytes());
    } else {
        for (i, v) in [l + 4, bot - 4, r - 4, t + 4].into_iter().enumerate() {
            b[16 + i * 4..20 + i * 4].copy_from_slice(&v.to_be_bytes());
        }
    }
    b
}

fn figure_document(filled: bool, swap: bool, wrong_order: bool) -> Document {
    let order = if swap { [1_u16, 0, 2] } else { [0, 1, 2] };
    let shapes = [
        (6_u8, [0, 0, 400, 240], 0x000000ff_u32),
        (4, [100, 100, 500, 340], 0x00ff0000),
        (1, [0, 0, 600, 320], 0),
    ];
    let mut vector = Vec::new();
    let mut index = vec![0; 20];
    index[..4].copy_from_slice(&[3, 11, 0, 1]);
    index[18..20].copy_from_slice(&3_u16.to_be_bytes());
    for id in order {
        let (kind, bounds, color) = shapes[id as usize];
        let fill = filled && kind != 1;
        let child = primitive(kind, bounds, fill);
        let offset = vector.len();
        let mut segment = Vec::new();
        if fill {
            segment = vec![0; 46];
            segment[..4].copy_from_slice(&[1, 0, 10, 0x60]);
            segment[4..6].copy_from_slice(&((46 + child.len()) as u16).to_be_bytes());
            segment[6..8].copy_from_slice(&1_u16.to_be_bytes());
            segment[16..18].copy_from_slice(&0x0800_u16.to_be_bytes());
            for (i, v) in bounds.into_iter().enumerate() {
                segment[20 + i * 4..24 + i * 4].copy_from_slice(&v.to_be_bytes());
            }
            segment[36..40].copy_from_slice(&color.to_be_bytes());
            segment[40..44].copy_from_slice(&0x00ffffff_u32.to_be_bytes());
            segment[44..46].copy_from_slice(&46_u16.to_be_bytes());
        }
        segment.extend(child);
        push_fdm_index_row(
            &mut index,
            offset as u32,
            u16::from(if fill { 10 } else { kind }) << 8,
            (bounds[0], bounds[2], bounds[1], bounds[3]),
        );
        vector.extend(segment);
    }
    let mut figure = vec![0, 1, 0, 4, 0, 2, 0, 1];
    for (id, rowid) in order
        .into_iter()
        .zip(if wrong_order { [0, 1, 2] } else { order })
    {
        let _ = id;
        for w in [0x601_u16, 12, 0, 0, 0, 1, rowid, 0x100] {
            figure.extend(w.to_be_bytes());
        }
    }
    let mut frame = vec![0, 1, 0, 4, 0, 2, 0, 1, 1, 1, 0, 4, 0, 0, 0, 3];
    for id in 0..3_u16 {
        let slot = order.iter().position(|value| *value == id).unwrap() as u16 + 1;
        let bounds = shapes[id as usize].1;
        let width = ((bounds[2] - bounds[0]) as u16) * 5 / 2;
        let height = ((bounds[3] - bounds[1]) as u16) * 5 / 2;
        for w in [
            0x102_u16,
            56,
            0,
            id,
            4,
            0,
            slot,
            0,
            1,
            1,
            0,
            0,
            0,
            0,
            200 + id * 300,
            0,
            100,
            0,
            width,
            0,
            height,
            0,
            0,
            1,
            0,
            0,
            1,
            0x4000,
            0,
            0,
        ] {
            frame.extend(w.to_be_bytes());
        }
    }
    let mut words = vec![10_u16, 10];
    for (id, tail) in [(0_u16, true), (2, true), (1, false)] {
        words.extend([
            0x1c, 0, 14, 0, 0x30, 0xffff, 0x507, 0x12, id, 0, 14, 0, 0, 0x1f, 0x1c, 1, 7, 0, 0, 1,
            0x1d, 2, 0x1e, 5, 0, 1, 0x1f,
        ]);
        if tail {
            words.push(10);
        }
    }
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
        0x915_u16, 0, 1, 0, 6, 0, 6, 0, 5, 1, 3, 1, 2, 28, 0x8002, 28, 0x8002, 28, 0x8002, 0xffdf,
        2,
    ] {
        line.extend(w.to_be_bytes());
    }
    let mut page = Vec::new();
    for w in [0_u32, 0x10, 0] {
        page.extend(w.to_be_bytes());
    }
    let mut fields = [0_u16; 42];
    fields[2] = 1;
    fields[7] = 39;
    for i in [10, 13, 17, 18, 19] {
        fields[i] = 370;
    }
    fields[14] = 222;
    fields[20] = 255;
    fields[21] = 592;
    for f in fields {
        page.extend(f.to_be_bytes());
    }
    let mut view = vec![0; 10];
    let mut margins = vec![0, 0xd8];
    for _ in 0..4 {
        margins.extend(3000_u16.to_be_bytes());
    }
    margins.extend([0; 22]);
    let mut font = vec![0; 20];
    font[0] = 0x1f;
    font[3..5].copy_from_slice(&370_u16.to_be_bytes());
    for (code, value) in [
        (0x1002_u16, margins.as_slice()),
        (0x1006, font.as_slice()),
        (0x1007, &[0]),
        (0x1008, &[0]),
    ] {
        view.extend(code.to_be_bytes());
        view.extend((value.len() as u16).to_be_bytes());
        view.extend(value);
    }
    let mut alpha = Vec::new();
    let values = if filled {
        vec![
            64_u32,
            1,
            2,
            0,
            1,
            0,
            u32::MAX,
            0,
            0,
            0,
            1,
            1,
            u32::MAX,
            0,
            0,
            0,
        ]
    } else {
        vec![16, 1, 0, 0]
    };
    for v in values {
        alpha.extend(v.to_be_bytes());
    }
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/Frame", &frame),
        ("/Figure", &figure),
        ("/LineMark", &line),
        ("/PageMark", &page),
        (DOCUMENT_VIEW_STYLES_PATH, &view),
        ("/FigureData/main_data/FDMVector", &vector),
        ("/FigureData/main_data/FDMIndex", &index),
        ("/FigureData/main_data/AlphaBlend", &alpha),
    ]))
    .unwrap()
}

#[test]
fn source_figure_order_color_and_anchor_bindings_preserve_raw_streams() {
    for filled in [false, true] {
        for swap in [false, true] {
            let doc = figure_document(filled, swap, false);
            let shapes = doc.figure_shape_candidates();
            assert_eq!(shapes.len(), 3);
            assert_eq!(shapes[0].kind(), if swap { "ellipse" } else { "rectangle" });
            assert_eq!(
                shapes
                    .iter()
                    .find(|s| s.kind() == "rectangle")
                    .unwrap()
                    .line_mark_index(),
                2
            );
            assert_eq!(
                shapes
                    .iter()
                    .find(|s| s.kind() == "ellipse")
                    .unwrap()
                    .line_mark_index(),
                4
            );
            if filled {
                assert_eq!(
                    shapes
                        .iter()
                        .find(|s| s.kind() == "rectangle")
                        .unwrap()
                        .fill(),
                    Some("#ff0000")
                );
                assert_eq!(
                    shapes
                        .iter()
                        .find(|s| s.kind() == "ellipse")
                        .unwrap()
                        .fill(),
                    Some("#0000ff")
                );
            } else {
                assert!(shapes.iter().all(|s| s.fill().is_none()));
            }
            assert!(
                doc.raw_streams()
                    .iter()
                    .any(|s| s.name() == "/FigureData/main_data/FDMVector")
            );
        }
    }
}

#[test]
fn shape_projection_replaces_no_text_notice_and_agrees_with_layer_order() {
    let doc = figure_document(true, true, false);
    let original = doc.clone();
    let core = DocumentCore::from_document(doc);
    let svg = core.render_page_svg(0).unwrap();
    assert_eq!(svg.matches("class=\"rjtd-native-figure\"").count(), 3);
    assert!(svg.find("<ellipse").unwrap() < svg.find("fill=\"#ff0000\"").unwrap());
    assert!(!svg.contains("No extractable text"));
    let layer = core.get_page_layer_tree(0).unwrap();
    assert_json_brackets_balanced(&layer);
    assert_eq!(
        layer.matches("\"type\":\"nativeFigureCandidate\"").count(),
        3
    );
    assert!(layer.contains("\"referenceBacked\":false"));
    assert_eq!(core.document(), &original);
}

#[test]
fn contradictory_order_and_nonopaque_paint_stay_unpromoted() {
    assert!(
        figure_document(true, true, true)
            .figure_shape_candidates()
            .is_empty()
    );
    let mut doc = figure_document(true, false, false);
    let alpha = doc
        .raw_streams
        .iter_mut()
        .find(|s| s.name() == "/FigureData/main_data/AlphaBlend")
        .unwrap();
    let mut bytes = alpha.bytes().to_vec();
    bytes[27] = 0;
    *alpha = RawStream::new("/FigureData/main_data/AlphaBlend", bytes);
    assert!(doc.figure_shape_candidates().is_empty());
    let mut doc = figure_document(false, false, false);
    doc.object_frame_records[0].raw_bytes[31] = 1;
    assert!(doc.figure_shape_candidates().is_empty());
}
