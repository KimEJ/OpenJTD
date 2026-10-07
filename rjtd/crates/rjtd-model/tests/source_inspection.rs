use rjtd_model::{
    Document, DocumentFont, DocumentSourceFontSize, RawStream, TextSourceSpan, UnknownStyle,
};

fn styled_document() -> Document {
    let mut bytes = vec![0; 32];
    bytes[..8].copy_from_slice(b"SsmgV.01");
    bytes[20..28].copy_from_slice(b"TextV.01");
    bytes[28..32].copy_from_slice(&4_u32.to_be_bytes());
    for unit in "A😀B".encode_utf16() {
        bytes.extend(unit.to_be_bytes());
    }
    bytes.extend([
        0xfe, 2, 2, 1, 0x72, 3, 2, 0, 2, 15, 4, 0, 0, 0, 255, 20, 4, 0x84, 0, 0, 0, 0xff, 0, 0,
    ]);
    bytes.extend(3_u32.to_be_bytes());
    bytes.push(0xff);
    let mut document = Document::from_plain_text("A😀B");
    document.push_raw_stream(RawStream::new("/DocumentText", bytes));
    document.push_font(DocumentFont::new("/Font", 2, 0, "MS Mincho", Vec::new()));
    document
}

#[test]
fn source_style_inspection_retains_units_font_identity_and_raw_data() {
    let document = styled_document();
    let raw = document.raw_streams()[0].bytes().to_vec();
    let span = TextSourceSpan::new(32, 40, 16, 20);
    assert_eq!(
        document.text_font_size_source_candidate(&span),
        Some(DocumentSourceFontSize::Mm100(370))
    );
    assert_eq!(document.text_foreground_bgr24_candidate(&span), Some(255));
    let style = document
        .text_character_style_source_candidate(&span)
        .unwrap();
    assert_eq!(style.flags(), Some(0x8400_0000));
    assert_eq!(style.font(), Some((2, "MS Mincho")));
    assert_eq!(style.script_candidate(), None);
    assert_eq!(document.raw_streams()[0].bytes(), raw);
}

#[test]
fn source_style_inspection_rejects_inconsistent_outside_and_ambiguous_spans() {
    let mut document = styled_document();
    for span in [
        TextSourceSpan::new(34, 40, 16, 20),
        TextSourceSpan::new(32, 42, 16, 21),
        TextSourceSpan::new(32, 32, 16, 16),
    ] {
        assert!(
            document
                .text_character_style_source_candidate(&span)
                .is_none()
        );
        assert!(document.text_font_size_source_candidate(&span).is_none());
        assert!(document.text_foreground_bgr24_candidate(&span).is_none());
    }
    let bytes = document.raw_streams()[0].bytes().to_vec();
    document.push_raw_stream(RawStream::new("/DocumentText", bytes));
    assert!(
        document
            .text_character_style_source_candidate(&TextSourceSpan::new(32, 40, 16, 20))
            .is_none()
    );
    assert_eq!(document.raw_streams().len(), 2);
}

#[test]
fn source_queries_do_not_invent_layout_or_discard_unknown_records() {
    let mut document = Document::from_plain_text("plain text");
    document.push_unknown_style(UnknownStyle::from_stream(
        "/DocumentViewStyles",
        vec![0xff, 0, 0x55],
    ));
    let original = document.clone();
    assert_eq!(document.page_size_mm100_candidate(), None);
    assert_eq!(document.page_margins_mm100_candidate(), None);
    assert_eq!(document.writing_mode_candidate(), None);
    assert!(document.section_source_candidate().is_none());
    assert!(document.running_text_source_candidate().is_none());
    assert!(document.source_line_range_candidates().is_none());
    assert!(document.paragraph_attribute_candidate(0).is_none());
    assert_eq!(document, original);
}
