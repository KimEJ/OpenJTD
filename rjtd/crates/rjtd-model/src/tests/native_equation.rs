use super::*;
use crate::*;

fn equation_document(mismatch: bool, corrupt_state: bool) -> Document {
    let mut info = vec![0; 16];
    info[..4].copy_from_slice(&1_u32.to_le_bytes());
    let class = "JSEQ.Document.3\0"
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    let mut row = vec![0; 46];
    row[8..12].copy_from_slice(&1_u32.to_le_bytes());
    row[14..16].copy_from_slice(&1440_u16.to_le_bytes());
    row[18..20].copy_from_slice(&680_u16.to_le_bytes());
    row[42..46].copy_from_slice(&(class.len() as u32).to_le_bytes());
    row.extend(class);
    let mut tail = vec![0; 80];
    tail[4..8].copy_from_slice(&1440_u32.to_le_bytes());
    tail[8..12].copy_from_slice(&680_u32.to_le_bytes());
    row.extend(tail);
    info.extend(row);
    let mut snapshot = embedded_press_snapshot_fixture(1440, 680, 1, 1);
    let mut contents = b"M\0A\0T\0H\0.\0V\0A\0F\0".to_vec();
    contents.resize(128, 0);
    let mut packet = |kind: u32, p: &[u8]| {
        snapshot.extend((p.len() as u32 + 8).to_le_bytes());
        snapshot.extend(kind.to_le_bytes());
        snapshot.extend(p);
    };
    packet(0x3c, &0xcc_u32.to_le_bytes());
    packet(0x24, &[0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0]);
    for (i, ch) in "a2+b2=c2".chars().enumerate() {
        let sup = ch == '2';
        let mut font = vec![0; 236];
        for (offset, value) in [
            (0, 228_u32),
            (4, 16),
            (8, 0),
            (12, 0xffffff),
            (20, 36),
            (24, 140),
            (28, 172),
            (60, 400),
            (180, 56),
            (188, if sup { 240 } else { 370 }),
            (192, 1024),
            (196, 1024),
        ] {
            font[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        font[64] = if ch.is_ascii_alphabetic() { 255 } else { 0 };
        let family = "Times New Roman\0"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>();
        font[72..72 + family.len()].copy_from_slice(&family);
        packet(0x94, &font);
        let mut select = 16_u32.to_le_bytes().to_vec();
        select.extend([0; 4]);
        packet(0x60, &select);
        packet(0x40, &1_u32.to_le_bytes());
        let mut text = vec![0; 40];
        text[..4].copy_from_slice(&(100_u32 + i as u32 * 150).to_le_bytes());
        text[4..8].copy_from_slice(&240_u32.to_le_bytes());
        text[16..20].copy_from_slice(&4_u32.to_le_bytes());
        text[20..24].copy_from_slice(&1_u32.to_le_bytes());
        text[36..40].copy_from_slice(&(ch as u32).to_le_bytes());
        packet(if corrupt_state && i == 0 { 0xc9 } else { 0xc8 }, &text);
        let mut restore = 3_u32.to_le_bytes().to_vec();
        restore.extend([0; 4]);
        packet(0x60, &restore);
        packet(0x65, &16_u32.to_le_bytes());
        contents.extend((if mismatch && i == 0 { 'q' } else { ch } as u32).to_le_bytes());
        contents.extend([0, 0, 0, 0, 255, 255, 255, 0]);
    }
    let mut frame = vec![0, 1, 0, 4, 0, 2, 0, 1, 1, 1, 0, 4, 0, 0, 0, 1];
    for w in [
        0x102_u16, 56, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1440, 0, 680, 0, 200, 1, 0,
        0, 4, 0x300, 0, 0,
    ] {
        frame.extend(w.to_be_bytes());
    }
    let mut words = "BEFORE\n".encode_utf16().collect::<Vec<_>>();
    let first = words.len();
    words.extend([
        0x1c, 0, 14, 0, 0x30, 0xffff, 0x507, 0x12, 0, 0, 14, 0, 0, 0x1f, 0x1c, 1, 7, 0, 0, 1, 0x1d,
        2, 0x1e, 5, 0, 1, 0x1f,
    ]);
    words.extend("\nAFTER".encode_utf16());
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
        4,
        0,
        4,
        0,
        3,
        first as u16,
        3,
        28,
        0x8002,
        6,
        2,
        0xffdf,
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
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/Frame", &frame),
        ("/LineMark", &line),
        ("/PageMark", &page),
        (DOCUMENT_VIEW_STYLES_PATH, &view),
        ("/EmbedItems/EmbeddingInfo", &info),
        ("/EmbedItems/Embedding 1/JSEQ3Contents", &contents),
        ("/EmbedItems/Embedding 1/\x03EmbeddedPress", &snapshot),
    ]))
    .unwrap()
}

#[test]
fn zero_frame_reference_and_cached_glyphs_require_independent_source_agreement() {
    let document = equation_document(false, false);
    assert_eq!(document.object_embedding_frames().len(), 1);
    assert_eq!(document.object_embedding_frames()[0].frame_ref(), 0);
    let candidates = document.equation_candidates();
    assert_eq!(candidates.len(), 1);
    assert_eq!(
        candidates[0]
            .glyphs()
            .iter()
            .map(|g| g.text())
            .collect::<String>(),
        "a2+b2=c2"
    );
    assert_eq!(candidates[0].glyphs()[1].font_size_mm100(), 240);
    assert!(candidates[0].glyphs()[0].italic());
    assert!(!candidates[0].glyphs()[1].italic());
    assert!(
        equation_document(true, false)
            .equation_candidates()
            .is_empty()
    );
    assert!(
        equation_document(false, true)
            .equation_candidates()
            .is_empty()
    );
}

#[test]
fn equation_projection_preserves_body_text_and_raw_source_ranges() {
    let document = equation_document(false, false);
    let original = document.clone();
    let core = DocumentCore::from_document(document);
    let svg = core.render_page_svg(0).unwrap();
    assert_eq!(
        svg.matches("class=\"rjtd-native-equation-glyph\"").count(),
        8
    );
    assert!(svg.contains(">BEFORE</text>"));
    assert!(svg.contains(">AFTER</text>"));
    let layer = core.get_page_layer_tree(0).unwrap();
    assert_json_brackets_balanced(&layer);
    assert!(layer.contains("nativeEquationCandidate"));
    assert!(layer.contains("\"text\":\"BEFORE\""));
    assert!(layer.contains("\"text\":\"AFTER\""));
    assert!(layer.contains("\"baselineDecoded\":false"));
    assert_eq!(core.document(), &original);
}

#[test]
fn equation_unknown_frame_dimensions_and_edited_body_do_not_use_cached_placement() {
    let mut document = equation_document(false, false);
    document.object_embedding_frames[0].frame_width += 1;
    assert!(document.equation_candidates().is_empty());
    let mut document = equation_document(false, false);
    if let Block::Paragraph(p) = &mut document.blocks[0]
        && let Inline::Text(t) = &mut p.inlines[0]
    {
        t.text.push('X');
    }
    let core = DocumentCore::from_document(document);
    assert!(
        !core
            .render_page_svg(0)
            .unwrap()
            .contains("rjtd-native-equation-glyph")
    );
}
