use super::support::cfb_with_streams;
use crate::*;

fn framed_note(kind: u16, id: u16) -> Vec<u16> {
    vec![
        0x1c,
        0,
        13,
        0,
        if kind == 0x30 { 15 } else { 14 },
        0,
        123,
        kind,
        id,
        13,
        0,
        0,
        0x1f,
    ]
}

fn marker(text: &str) -> Vec<u16> {
    let mut units = vec![0x1c, 1, 7, 0, 0, 1, 0x1d];
    units.extend(text.encode_utf16());
    units.extend([0x1e, 5, 0, 1, 0x1f]);
    units
}

fn text_stream(units: &[u16]) -> Vec<u8> {
    let mut bytes = vec![0; 32];
    bytes[..8].copy_from_slice(b"SsmgV.01");
    bytes[20..28].copy_from_slice(b"TextV.01");
    bytes[28..32].copy_from_slice(&(units.len() as u32).to_be_bytes());
    for unit in units {
        bytes.extend(unit.to_be_bytes());
    }
    bytes.push(0xff);
    bytes
}

fn footnote_streams() -> [Vec<u8>; 3] {
    footnote_streams_with_body("BODY")
}

fn footnote_streams_with_body(value: &str) -> [Vec<u8>; 3] {
    let mut body = value.encode_utf16().collect::<Vec<_>>();
    let body_offset = body.len() as u32;
    body.extend(framed_note(0x10, 0));
    body.extend(marker("*1"));
    let mut note = framed_note(0x30, 0);
    note.extend(marker("*1"));
    let note_anchor = note.len() as u32 - 1;
    note.extend("A😀B\r".encode_utf16());
    note.extend(framed_note(0x30, 0xffff));
    note.extend(marker("EDITOR-TEMPLATE"));
    let mut link = vec![
        0, 1, 0, 1, 0, 0, 0, 44, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255,
        255, 1, 0, 255, 255, 255, 255, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0,
    ];
    link[10..14].copy_from_slice(&note_anchor.to_be_bytes());
    link[18..22].copy_from_slice(&body_offset.to_be_bytes());
    [text_stream(&body), text_stream(&note), link]
}

fn parse_streams(streams: &[Vec<u8>; 3]) -> Document {
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &streams[0]),
        ("/Footnote", &streams[1]),
        ("/FootnoteLink", &streams[2]),
        ("/MarkTag", &[0xff]),
    ]))
    .unwrap()
}

#[test]
fn preserves_linked_footnote_text_and_auxiliary_streams_without_promoting_placement() {
    let streams = footnote_streams();
    let document = parse_streams(&streams);
    let notes = document.footnote_text_candidates();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].marker(), "*1");
    assert_eq!(notes[0].text(), "A😀B");
    assert_eq!(notes[0].note_anchor_offset(), 26);
    assert_eq!(notes[0].body_record_offset(), 4);
    let note_marker = notes[0].note_marker_span();
    assert_eq!(
        &streams[1][note_marker.byte_start()..note_marker.byte_end()],
        &[0, 0x1d, 0, b'*', 0, b'1', 0, 0x1e]
    );
    for (name, original) in [("/Footnote", &streams[1]), ("/FootnoteLink", &streams[2])] {
        assert_eq!(
            document
                .raw_streams()
                .iter()
                .find(|stream| stream.name() == name)
                .unwrap()
                .bytes(),
            original
        );
    }
    assert_eq!(
        document
            .raw_streams()
            .iter()
            .find(|stream| stream.name() == "/MarkTag")
            .unwrap()
            .bytes(),
        &[0xff]
    );
    let body = document
        .blocks()
        .iter()
        .filter_map(|block| match block {
            Block::Paragraph(paragraph) => Some(paragraph),
            _ => None,
        })
        .flat_map(|paragraph| paragraph.inlines())
        .filter_map(|inline| match inline {
            Inline::Text(run) => Some(run.text()),
            _ => None,
        })
        .collect::<String>();
    assert_eq!(body, "BODY*1");
}

fn styled_footnote(body_parent: u16, note_parent: u16, scale: u16) -> Document {
    let mut streams = footnote_streams_with_body("FIRST\nBODY");
    let parsed = parse_streams(&streams);
    let note = &parsed.footnote_text_candidates()[0];
    append_marker_style(&mut streams[0], note.body_marker_span(), body_parent);
    append_marker_style(&mut streams[1], note.note_marker_span(), note_parent);
    let mut styles = b"SsmgV.01".to_vec();
    for word in [5_u32, 256, 5] {
        styles.extend(word.to_be_bytes());
    }
    for word in [1_u16, 2] {
        styles.extend(word.to_be_bytes());
    }
    let mut marker = Vec::new();
    marker.extend(0xe000_u32.to_be_bytes());
    marker.extend((-50_i16).to_be_bytes());
    for value in [60, scale, scale] {
        marker.extend(value.to_be_bytes());
    }
    marker.extend([0x80, 0, 0x80, 0, 2, 0x80, 0, 0]);
    for (offset, label, format) in [
        (0x114, "MARK", marker.as_slice()),
        (
            0x214,
            "AREA",
            &[0, 0, 0x40, 0x80, 0, 0x30, 0x80, 0, 0x80, 0, 2, 0x80, 0, 0][..],
        ),
    ] {
        let mut payload = Vec::new();
        payload.extend((label.len() as u16).to_be_bytes());
        for word in label.encode_utf16() {
            payload.extend(word.to_be_bytes());
        }
        payload.extend([0, 0]);
        for (code, data) in [
            (0x5006_u16, &[0, 0, 1, 1][..]),
            (0x5004, format),
            (0x5007, &[2, 2, 0]),
        ] {
            payload.extend(code.to_be_bytes());
            payload.extend((data.len() as u16).to_be_bytes());
            payload.extend(data);
        }
        styles.resize(offset, 0);
        styles.extend(0x5555_u16.to_be_bytes());
        styles.extend((payload.len() as u16).to_be_bytes());
        styles.extend(payload);
    }
    let mut view = super::native_vertical::view(false, false);
    let summary = summarize_style_stream(&view);
    let gap = summary
        .records()
        .iter()
        .find(|r| r.code() == 0x100b)
        .unwrap();
    view[gap.offset() + 5..gap.offset() + 7].copy_from_slice(&600_u16.to_be_bytes());
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &streams[0]),
        ("/Footnote", &streams[1]),
        ("/FootnoteLink", &streams[2]),
        (TEXT_LAYOUT_STYLE_PATH, &styles),
        (DOCUMENT_VIEW_STYLES_PATH, &view),
    ]))
    .unwrap()
}

fn append_marker_style(bytes: &mut Vec<u8>, span: &TextSourceSpan, parent: u16) {
    let units = u32::from_be_bytes(bytes[28..32].try_into().unwrap()) as usize;
    bytes.truncate(32 + units * 2);
    bytes.push(0);
    bytes.extend(((span.unit_start() + 1 - 16) as u32).to_be_bytes());
    for (id, flags) in [(parent, 0x8000_0000_u32), (0xffff, 0)] {
        bytes.extend([0xfe, 1, 2]);
        bytes.extend(id.to_be_bytes());
        bytes.extend([20, 4]);
        bytes.extend(flags.to_be_bytes());
        bytes.extend([0xff, 0]);
        if id == parent {
            bytes.push(0);
            bytes.extend(((span.unit_end() - span.unit_start() - 3) as u32).to_be_bytes());
        }
    }
    let remaining = 16 + units - span.unit_end();
    if remaining > 0 {
        bytes.push(0);
        bytes.extend((remaining as u32).to_be_bytes());
    }
    bytes.push(0xff);
}

#[test]
fn linked_footnote_parent_style_scales_only_the_body_marker_and_preserves_source() {
    let doc = styled_footnote(1, 2, 50);
    let original = doc.clone();
    let note = &doc.footnote_text_candidates()[0];
    let span = native_visible_text_span(&doc, note.marker(), note.body_marker_span()).unwrap();
    let resolver = document_text_style_resolver(&doc).unwrap();
    let style = document_text_character_style(&doc, &resolver, &span);
    assert_eq!(style.script, Some("super"));
    assert_eq!(style.font_scale(), 0.5);
    assert_eq!(
        style.script_basis,
        Some("linked-footnote-text-layout-slots-1-2")
    );
    assert!(
        document_text_character_style(&doc, &resolver, &span.subspan_by_units(0, 1))
            .script
            .is_none()
    );
    let core = DocumentCore::from_document(doc);
    let lines = core.page_text_lines(0).unwrap();
    let tops = lines
        .iter()
        .filter_map(|line| {
            linked_footnote_body_line_top(core.document(), core.page_layout_for(0), 1, line)
        })
        .collect::<Vec<_>>();
    assert_eq!(tops.len(), 2);
    assert!((tops[1] - tops[0] - hundredth_millimeters_to_css_px(592)).abs() < 0.01);
    let svg = core.render_page_svg(0).unwrap();
    assert_eq!(svg.matches("data-script-candidate=\"super\"").count(), 1);
    assert!(svg.contains("font-size=\"7.0\""));
    assert!(svg.contains("data-script-basis=\"linked-footnote-text-layout-slots-1-2\""));
    let layer = core.get_page_layer_tree(0).unwrap();
    assert!(layer.contains("\"scriptBasis\":\"linked-footnote-text-layout-slots-1-2\""));
    super::assert_json_brackets_balanced(&layer);
    assert_eq!(core.document(), &original);
}

#[test]
fn literal_footnote_rows_reject_edits_and_unknown_line_gap_without_losing_text() {
    let mut edited = styled_footnote(1, 2, 50);
    let Block::Paragraph(paragraph) = &mut edited.blocks[0] else {
        unreachable!()
    };
    paragraph.set_text("EDITED");
    let mut unknown = styled_footnote(1, 2, 50);
    let i = unknown
        .unknown_styles
        .iter()
        .position(|s| s.name() == Some(DOCUMENT_VIEW_STYLES_PATH))
        .unwrap();
    let mut view = unknown.unknown_styles[i].payload().to_vec();
    let summary = summarize_style_stream(&view);
    let gap = summary
        .records()
        .iter()
        .find(|r| r.code() == 0x100b)
        .unwrap();
    view[gap.offset() + 5..gap.offset() + 7].copy_from_slice(&601_u16.to_be_bytes());
    unknown.unknown_styles[i] = UnknownStyle::from_stream(DOCUMENT_VIEW_STYLES_PATH, view);
    for document in [edited, unknown] {
        let core = DocumentCore::from_document(document);
        assert!(core.page_text_lines(0).unwrap().iter().all(|line| {
            linked_footnote_body_line_top(core.document(), core.page_layout_for(0), 1, line)
                .is_none()
        }));
        assert!(core.render_page_svg(0).unwrap().contains("*1"));
    }
}

#[test]
fn unknown_footnote_parent_references_and_scales_keep_normal_marker_text() {
    for (body, note, scale) in [(2, 2, 50), (1, 1, 50), (1, 2, 51)] {
        let core = DocumentCore::from_document(styled_footnote(body, note, scale));
        let svg = core.render_page_svg(0).unwrap();
        assert!(!svg.contains("linked-footnote-text-layout-slots-1-2"));
        assert!(!svg.contains("data-script-candidate=\"super\""));
        assert!(svg.contains(">*1</text>"));
        assert_eq!(core.document().footnote_text_candidates()[0].text(), "A😀B");
    }
}

#[test]
fn rejects_misaligned_unknown_truncated_and_duplicate_footnote_links_but_keeps_raw_data() {
    for offset in [0, 13, 21] {
        let mut streams = footnote_streams();
        streams[2][offset] ^= 1;
        let document = parse_streams(&streams);
        assert!(document.footnote_text_candidates().is_empty());
        assert_eq!(
            document
                .raw_streams()
                .iter()
                .find(|stream| stream.name() == "/FootnoteLink")
                .unwrap()
                .bytes(),
            streams[2]
        );
    }
    let mut streams = footnote_streams();
    streams[2].pop();
    assert!(
        parse_streams(&streams)
            .footnote_text_candidates()
            .is_empty()
    );
    let streams = footnote_streams();
    let mut document = parse_streams(&streams);
    document.push_raw_stream(RawStream::new("/Footnote", streams[1].clone()));
    assert!(document.footnote_text_candidates().is_empty());
}
