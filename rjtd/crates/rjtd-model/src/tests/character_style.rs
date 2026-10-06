use crate::*;

type StyleProperty<'a> = (u8, &'a [u8]);
type StyledPart<'a> = (&'a str, &'a [StyleProperty<'a>]);

fn styled_document(parts: &[StyledPart<'_>]) -> Document {
    let text = parts.iter().map(|(text, _)| *text).collect::<String>();
    let units = text.encode_utf16().count();
    let mut bytes = vec![0; 32];
    bytes[..8].copy_from_slice(b"SsmgV.01");
    bytes[20..28].copy_from_slice(b"TextV.01");
    bytes[28..32].copy_from_slice(&(units as u32).to_be_bytes());
    for unit in text.encode_utf16() {
        bytes.extend(unit.to_be_bytes());
    }
    for (text, properties) in parts {
        let mut length = text.encode_utf16().count();
        if !properties.is_empty() {
            bytes.push(0xfe);
            for (id, value) in *properties {
                bytes.extend([*id, value.len() as u8]);
                bytes.extend(*value);
            }
            bytes.extend([0xff, 0]);
            length -= 1;
        }
        if length > 0 {
            bytes.push(0);
            bytes.extend((length as u32).to_be_bytes());
        }
    }
    bytes.push(0xff);
    let paragraph = Paragraph::new(
        vec![Inline::Text(TextRun::with_source_span(
            text,
            None,
            Some(TextSourceSpan::new(32, 32 + units * 2, 16, 16 + units)),
        ))],
        None,
    );
    let mut document = Document::new(Metadata::default(), vec![Block::Paragraph(paragraph)]);
    document.push_raw_stream(RawStream::new("/DocumentText", bytes));
    let mut styles = vec![0; 32];
    let default = [
        0x1f, 0, 0, 1, 0x72, 0, 1, 0xff, 0xfe, 0xff, 0xfe, 0xff, 0xfd, 3, 0, 2, 0, 13, 0, 0,
    ];
    for (code, value) in [
        (0x1006_u16, default.as_slice()),
        (0x1007, &[1]),
        (0x1008, &[2]),
        (0x1009, &[3]),
    ] {
        styles.extend(code.to_be_bytes());
        styles.extend((value.len() as u16).to_be_bytes());
        styles.extend(value);
    }
    document.push_unknown_style(UnknownStyle::from_stream("/DocumentViewStyles", styles));
    document.push_font(DocumentFont::new("/Font", 1, 0, "MS Gothic", Vec::new()));
    document.push_font(DocumentFont::new("/Font", 2, 0, "MS Mincho", Vec::new()));
    document
}

#[test]
fn renders_character_style_boundaries_without_splitting_utf16_pairs_or_losing_resets() {
    let document = styled_document(&[
        ("A", &[]),
        ("B", &[(20, &[0x84, 0, 0, 0])]),
        ("C", &[(20, &[0x90, 0, 0, 0])]),
        ("D", &[(13, &[0, 1]), (20, &[0x80, 0, 0, 0x10])]),
        (
            "😀",
            &[
                (4, &[50]),
                (5, &[50]),
                (13, &[0, 0]),
                (20, &[0x80, 0, 12, 0]),
            ],
        ),
        ("E", &[(20, &[0x80, 0, 4, 0])]),
        (
            "F",
            &[
                (4, &[0]),
                (5, &[0]),
                (15, &[0, 0, 0, 255]),
                (20, &[0x80, 0, 0, 0]),
            ],
        ),
        ("G", &[(2, &[1, 0xee]), (15, &[255; 4])]),
    ]);
    let raw = document.raw_streams()[0].bytes().to_vec();
    let core = DocumentCore::from_document(document);
    let svg = core.render_page_svg(0).unwrap();
    for attribute in [
        "data-bold-paint-candidate=\"true\"",
        "data-italic-paint-candidate=\"true\"",
        "text-decoration=\"underline\"",
        "data-script-candidate=\"super\"",
        "data-script-candidate=\"sub\"",
        "fill=\"#ff0000\"",
        "font-size=\"18.7\"",
    ] {
        assert!(svg.contains(attribute), "missing {attribute}");
    }
    assert!(svg.contains(">😀</text>"));
    assert_eq!(
        core.page_text_advance_targets(0).unwrap(),
        [16, 17, 18, 19, 20, 22, 23, 24]
    );
    let measured = core
        .render_page_svg_with_text_widths(0, &BTreeMap::from([(16, 20.0), (17, 30.0)]))
        .unwrap();
    let run_x = |svg: &str, text: &str| {
        let node = svg[..svg.find(&format!(">{text}</text>")).unwrap()]
            .rsplit("<text")
            .next()
            .unwrap();
        node.split("x=\"")
            .nth(1)
            .unwrap()
            .split('"')
            .next()
            .unwrap()
            .parse::<f32>()
            .unwrap()
    };
    assert!((run_x(&measured, "B") - run_x(&measured, "A") - 20.0).abs() < 0.01);
    assert!((run_x(&measured, "C") - run_x(&measured, "B") - 30.0).abs() < 0.01);
    let layer = core.get_page_layer_tree(0).unwrap();
    assert!(layer.contains("\"scriptCandidate\":\"super\""));
    assert!(layer.contains("\"jtdUnitRange\":{\"start\":20,\"end\":22}"));
    assert!(layer.contains("\"characterStyleDecoded\":false"));
    assert_eq!(core.document.raw_streams()[0].bytes(), raw);
}

#[test]
fn resolves_character_font_ids_and_rejects_unknown_or_ambiguous_style_profiles() {
    let mut document = styled_document(&[
        ("A", &[(3, &[0, 2]), (20, &[0x80, 0, 0, 0])]),
        ("B", &[(3, &[255, 255]), (20, &[0, 0, 0, 0])]),
        ("C", &[(3, &[0, 9]), (20, &[0x84, 0, 0, 1])]),
        ("D", &[(4, &[50]), (5, &[25]), (20, &[0x80, 0, 12, 0])]),
    ]);
    let resolver = document_text_style_resolver(&document).unwrap();
    let style = |document: &Document, unit| {
        document_text_character_style(
            document,
            &resolver,
            &TextSourceSpan::new(unit * 2, unit * 2 + 2, unit, unit + 1),
        )
    };
    assert_eq!(style(&document, 16).font.unwrap().0, 2);
    assert_eq!(style(&document, 17).font.unwrap().0, 1);
    assert!(style(&document, 18).font.is_none());
    assert!(style(&document, 18).flags.is_none());
    assert!(!style(&document, 18).bold);
    assert!(style(&document, 19).script.is_none());
    document.push_font(DocumentFont::new("/Font", 2, 10, "duplicate", Vec::new()));
    assert!(style(&document, 16).font.is_none());
}
