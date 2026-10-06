use super::support::cfb_with_streams;
use crate::*;

fn fixed_record(pitch: u16) -> Vec<u16> {
    vec![
        0x1c, 0x10, 16, 0, 0x20, 4, 8, pitch, 0, 0, 0xffff, 0, 16, 0, 0x10, 0x1f,
    ]
}

fn document(record: &[u16], vertical: bool) -> Document {
    let mut units = record.to_vec();
    units.extend("ABCDEFGHI\nJKLMNO".encode_utf16());
    let count = units.len() as u32;
    let mut text = vec![0; 32];
    text[..8].copy_from_slice(b"SsmgV.01");
    text[20..28].copy_from_slice(b"TextV.01");
    text[28..32].copy_from_slice(&(units.len() as u32).to_be_bytes());
    for word in units {
        text.extend(word.to_be_bytes());
    }
    text.push(0);
    text.extend(count.to_be_bytes());
    text.push(0xff);
    let mut line = Vec::new();
    for word in [
        0x915_u16, 0, 1, 0, 6, 0, 6, 0, 5, 19, 3, 3, 0, 4, 0, 3, 0, 4, 0, 0xff00, 2,
    ] {
        line.extend(word.to_be_bytes());
    }
    let mut page = Vec::new();
    for word in [0_u32, 0x10, 0] {
        page.extend(word.to_be_bytes());
    }
    let mut fields = [0_u16; 42];
    fields[2] = 1;
    fields[7] = 39;
    for i in [10, 13, 17, 18, 19] {
        fields[i] = 370;
    }
    fields[14] = 222;
    fields[20] = 8;
    fields[21] = 1000;
    for word in fields {
        page.extend(word.to_be_bytes());
    }
    parse_document(&cfb_with_streams(&[
        ("/DocumentText", &text),
        ("/LineMark", &line),
        ("/PageMark", &page),
        (
            DOCUMENT_VIEW_STYLES_PATH,
            &super::native_vertical::view(vertical, false),
        ),
    ]))
    .unwrap()
}

#[test]
fn plain_fixed_pitch_preserves_source_rows_and_stops_at_paragraph_end() {
    let doc = document(&fixed_record(1000), false);
    let original = doc.clone();
    let layout = page_layout_with_source_margins(&doc, page_layout_from_document(&doc));
    let plan = native_page_line_plan(&doc, layout, WritingMode::Horizontal).unwrap();
    let pages = native_pages_from_plan(&doc, &plan);
    assert_eq!(pages.len(), 1);
    assert_eq!(
        pages[0].iter().map(PageTextLine::text).collect::<Vec<_>>(),
        ["ABC", "DEF", "GHI", "JKL", "MNO"]
    );
    for (record, distance) in [0, 1000, 2000, 3000, 3592].into_iter().enumerate() {
        let (_, top, _) = native_rule_line_placement(&doc, layout, record).unwrap();
        assert!(
            (top - layout.margin_top_px() - hundredth_millimeters_to_css_px(distance)).abs()
                < 0.002
        );
    }
    let core = DocumentCore::from_document(doc);
    let svg = core.render_page_svg(0).unwrap();
    for text in ["ABC", "DEF", "GHI", "JKL", "MNO"] {
        assert_eq!(svg.matches(&format!(">{text}</text>")).count(), 1);
    }
    assert_eq!(
        core.get_page_layer_tree(0)
            .unwrap()
            .matches("\"pageAssignmentCandidate\":true")
            .count(),
        5
    );
    assert_eq!(core.document.raw_streams(), original.raw_streams());
    assert_eq!(core.document.blocks(), original.blocks());
    assert!(matches!(
        DocumentCore::from_document_with_limits(
            original,
            ParseLimits::DEFAULT.with_max_page_lines(4)
        ),
        Err(Error::ResourceLimit {
            resource: "document page lines",
            ..
        })
    ));
}

#[test]
fn plain_fixed_pitch_rejects_unknown_fields_direction_and_lost_source_binding() {
    for (index, value) in [(6, 9), (7, 0), (7, 5001), (8, 1), (9, 1), (10, 0), (14, 0)] {
        let mut words = fixed_record(1000);
        words[index] = value;
        let doc = document(&words, false);
        assert!(
            native_plain_paragraph_fixed_pitch(&doc.document_text_flow().unwrap().events()[0])
                .is_none()
        );
        let layout = page_layout_with_source_margins(&doc, page_layout_from_document(&doc));
        assert!(native_page_line_plan(&doc, layout, WritingMode::Horizontal).is_none());
    }
    let mut doc = document(&fixed_record(1000), true);
    let layout = page_layout_with_source_margins(&doc, page_layout_from_document(&doc));
    assert!(native_page_line_plan(&doc, layout, WritingMode::VerticalRl).is_none());
    doc = document(&fixed_record(1000), false);
    if let Block::Paragraph(first) = &mut doc.blocks[0] {
        *first = Paragraph::from_text("EDITED");
    }
    assert!(native_rule_line_placement(&doc, layout, 1).is_none());
    assert!(native_page_line_plan(&doc, layout, WritingMode::Horizontal).is_none());
}
