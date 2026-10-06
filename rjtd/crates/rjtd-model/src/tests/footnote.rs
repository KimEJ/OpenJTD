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
    let mut body = "BODY".encode_utf16().collect::<Vec<_>>();
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
