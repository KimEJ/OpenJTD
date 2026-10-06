use super::*;
use crate::*;

// Degenerate one-literal LH5 blocks keep this generated fixture independent
// of a compressor dependency or private document bytes.
fn compressed_stream(inner: &[u8]) -> Vec<u8> {
    let mut packed = Vec::new();
    let mut byte = 0_u8;
    let mut used = 0;
    for literal in inner {
        for (value, count) in [
            (1_u32, 16),
            (0, 5),
            (0, 5),
            (0, 9),
            (u32::from(*literal), 9),
            (0, 4),
            (0, 4),
        ] {
            for shift in (0..count).rev() {
                byte = (byte << 1) | (((value >> shift) & 1) as u8);
                used += 1;
                if used == 8 {
                    packed.push(byte);
                    byte = 0;
                    used = 0;
                }
            }
        }
    }
    if used != 0 {
        packed.push(byte << (8 - used));
    }
    let mut stream = rjtd_core::compressed_document::JUST_COMPRESSED_DOCUMENT_MAGIC.to_vec();
    stream.extend([22, 0, b'-', b'l', b'h', b'5', b'-']);
    stream.extend((packed.len() as u32).to_le_bytes());
    stream.extend((inner.len() as u32).to_le_bytes());
    stream.extend([0, 0, 0, 0, 0x20, 0, 0, 0, 0]);
    stream.extend(packed);
    stream
}

fn fixture() -> (Vec<u8>, Vec<u8>, Vec<u8>, usize) {
    let mut text = vec![0; 32];
    text[..8].copy_from_slice(b"SsmgV.01");
    text[20..28].copy_from_slice(b"TextV.01");
    text[28..32].copy_from_slice(&4_u32.to_be_bytes());
    for unit in "BODY".encode_utf16() {
        text.extend(unit.to_be_bytes());
    }
    let inner = cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/LineMark", b"line"),
        ("/PageMark", b"page"),
        ("/MarkTag", b"bookmark"),
        ("/Footnote", b"note"),
    ]);
    let stream = compressed_stream(&inner);
    let outer = cfb_with_streams(&[("/JSCompDocument", &stream)]);
    (outer, inner, stream, text.len() + 4 + 4 + 8 + 4)
}

#[test]
fn compressed_model_reuses_inner_container_for_auxiliary_streams_and_keeps_wrapper() {
    let (outer, inner, stream, _) = fixture();
    let payload = rjtd_core::document_text::read_document_text_payload(&outer).unwrap();
    assert_eq!(payload.decompressed_container(), Some(inner.as_slice()));
    let document = parse_document(&outer).unwrap();
    for (name, bytes) in [
        ("/LineMark", b"line".as_slice()),
        ("/PageMark", b"page"),
        ("/MarkTag", b"bookmark"),
        ("/Footnote", b"note"),
        ("/JSCompDocument", stream.as_slice()),
    ] {
        assert_eq!(
            document
                .raw_streams()
                .iter()
                .find(|entry| entry.name() == name)
                .unwrap()
                .bytes(),
            bytes
        );
    }
    let Block::Paragraph(paragraph) = &document.blocks()[0] else {
        panic!("paragraph");
    };
    let Inline::Text(run) = &paragraph.inlines()[0] else {
        panic!("text");
    };
    assert_eq!(run.text(), "BODY");
}

#[test]
fn compressed_inner_streams_share_limits_without_charging_a_new_decompression() {
    let (outer, inner, stream, inner_stream_bytes) = fixture();
    let limits = ParseLimits::DEFAULT
        .with_max_input_bytes(outer.len())
        .with_max_streams(6)
        .with_max_stream_bytes(stream.len() + inner_stream_bytes)
        .with_max_total_decompressed_bytes(inner.len() * 3);
    assert!(parse_document_with_limits(&outer, limits).is_ok());
    assert!(matches!(
        parse_document_with_limits(&outer, limits.with_max_streams(5)),
        Err(Error::ResourceLimit {
            resource: "document streams",
            ..
        })
    ));
    assert!(matches!(
        parse_document_with_limits(
            &outer,
            limits.with_max_stream_bytes(stream.len() + inner_stream_bytes - 1)
        ),
        Err(Error::ResourceLimit {
            resource: "document stream bytes",
            ..
        })
    ));
    assert!(matches!(
        parse_document_with_limits(
            &outer,
            limits.with_max_total_decompressed_bytes(inner.len() * 3 - 1)
        ),
        Err(Error::ResourceLimit {
            resource: "total LH5 decompressed bytes",
            ..
        })
    ));
}
