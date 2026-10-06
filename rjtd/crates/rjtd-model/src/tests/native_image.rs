use super::*;
use crate::*;

fn image_document(mode: NativeImageMode, conflicting_control: bool, truncate: bool) -> Document {
    let mut png = std::io::Cursor::new(Vec::new());
    let image = image::RgbImage::from_fn(2, 1, |x, _| {
        image::Rgb(if x == 0 { [255, 0, 0] } else { [0, 0, 255] })
    });
    image.write_to(&mut png, image::ImageFormat::Png).unwrap();
    let png = png.into_inner();
    let mut contents = vec![0; 20];
    contents[16..20].copy_from_slice(&(png.len() as u32).to_le_bytes());
    contents.extend(png);
    contents.extend([0; 44]);
    if truncate {
        contents.truncate(28);
    }
    let inline = mode == NativeImageMode::Inline;
    let mut words = Vec::new();
    if inline {
        words.extend("LEFT ".encode_utf16());
    }
    let object_inline = if conflicting_control { !inline } else { inline };
    words.extend([
        0x1c,
        0,
        14,
        0,
        0x30,
        0xffff,
        if object_inline { 0x107 } else { 0x507 },
        if object_inline { 0x10 } else { 0x12 },
        0,
        0,
        14,
        0,
        0,
        0x1f,
        0x1c,
        1,
        7,
        0,
        0,
        1,
        0x1d,
        2,
        0x1e,
        5,
        0,
        1,
        0x1f,
    ]);
    words.extend(if inline { "RIGHT" } else { "LEFT RIGHT" }.encode_utf16());
    let mut text = vec![0; 32];
    text[..8].copy_from_slice(b"SsmgV.01");
    text[20..28].copy_from_slice(b"TextV.01");
    text[28..32].copy_from_slice(&(words.len() as u32).to_be_bytes());
    for word in &words {
        text.extend(word.to_be_bytes());
    }
    text.push(0);
    text.extend((words.len() as u32).to_be_bytes());
    text.push(0xff);
    let mut frame = vec![0, 1, 0, 4, 0, 2, 0, 1, 1, 1, 0, 4, 0, 0, 0, 1];
    for word in [
        0x102_u16,
        56,
        0,
        0,
        1,
        0,
        1,
        0,
        u16::from(!inline),
        1,
        0,
        0,
        0,
        0,
        1200,
        0,
        0,
        0,
        3000,
        0,
        1500,
        0,
        200,
        if mode == NativeImageMode::Front { 4 } else { 1 },
        0,
        0,
        4,
        0x304,
        0,
        0,
    ] {
        frame.extend(word.to_be_bytes());
    }
    let mut line = Vec::new();
    for word in [
        0x915_u16,
        0,
        1,
        0,
        2,
        0,
        2,
        0,
        1,
        words.len() as u16 + 1,
        0x8003,
        0xfffe,
        2,
    ] {
        line.extend(word.to_be_bytes());
    }
    let mut page = Vec::new();
    for word in [0_u32, 0x10, 0] {
        page.extend(word.to_be_bytes());
    }
    let extent = if inline { 1500 } else { 370 };
    let mut fields = [0_u16; 42];
    fields[2] = 1;
    fields[7] = 39;
    for i in [10, 13, 17, 18] {
        fields[i] = extent;
    }
    fields[14] = 222;
    fields[19] = 370;
    fields[20] = 255;
    fields[21] = 592;
    for field in fields {
        page.extend(field.to_be_bytes());
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
        ("/EmbedItems/Embedding 1/Contents", &contents),
    ]))
    .unwrap()
}

#[test]
fn image_mode_requires_matching_frame_control_and_complete_single_payload() {
    for mode in [
        NativeImageMode::Inline,
        NativeImageMode::Wrap,
        NativeImageMode::Front,
    ] {
        let document = image_document(mode, false, false);
        let bindings = document.image_frame_candidates();
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].mode(), mode);
        assert_eq!(bindings[0].geometry_mm100(), [1200, 0, 3000, 1500]);
        assert!(
            image_document(mode, true, false)
                .image_frame_candidates()
                .is_empty()
        );
        assert!(
            image_document(mode, false, true)
                .image_frame_candidates()
                .is_empty()
        );
        let mut duplicate = document.clone();
        duplicate.push_object_frame_record(document.object_frame_records()[0].clone());
        assert!(duplicate.image_frame_candidates().is_empty());
        let mut fractional = document.clone();
        fractional.object_frame_records[0].raw_bytes[31] = 1;
        assert!(fractional.image_frame_candidates().is_empty());
    }
}

#[test]
fn native_image_and_text_share_bbox_order_source_ranges_and_measurement_targets() {
    for mode in [
        NativeImageMode::Inline,
        NativeImageMode::Wrap,
        NativeImageMode::Front,
    ] {
        let document = image_document(mode, false, false);
        let original = document.clone();
        let core = DocumentCore::from_document(document);
        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("rjtd-native-image\""));
        assert!(!svg.contains("rjtd-image-payload-diagnostic"));
        let image_at = svg.find("rjtd-native-image\"").unwrap();
        let text_at = svg.find("rjtd-native-image-text").unwrap();
        assert_eq!(image_at > text_at, mode == NativeImageMode::Front);
        let layer = core.get_page_layer_tree(0).unwrap();
        assert_json_brackets_balanced(&layer);
        assert!(layer.contains("\"modeCandidate\""));
        assert!(!layer.contains("imagePayloadDiagnostic"));
        let targets = core.page_text_advance_targets(0).unwrap();
        assert_eq!(targets.len(), 10);
        assert!(targets.windows(2).all(|p| p[0] < p[1]));
        let measured = targets.iter().map(|u| (*u, 8.0_f32)).collect();
        let resolved = core.render_page_svg_with_text_widths(0, &measured).unwrap();
        assert!(resolved.contains("rjtd-native-image\""));
        assert_eq!(core.document(), &original);
    }
}

#[test]
fn image_projection_rejects_edited_text_and_keeps_vertical_fallback() {
    let document = image_document(NativeImageMode::Wrap, false, false);
    let mut core = DocumentCore::from_document(document);
    let lines = core.page_text_lines(0).unwrap();
    assert!(
        native_image_projection(
            core.document(),
            core.page_layout,
            1,
            WritingMode::VerticalRl,
            lines,
            &BTreeMap::new()
        )
        .is_none()
    );
    let mut edited = core.document().clone();
    if let Block::Paragraph(p) = &mut edited.blocks[0]
        && let Inline::Text(t) = &mut p.inlines[0]
    {
        t.text.push('X');
    }
    core = DocumentCore::from_document(edited);
    assert!(
        !core
            .render_page_svg(0)
            .unwrap()
            .contains("rjtd-native-image\"")
    );
}
