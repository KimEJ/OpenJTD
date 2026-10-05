use crate::*;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NativeParagraphAttributes {
    pub(crate) first_mm100: u16,
    pub(crate) continuing_mm100: u16,
    pub(crate) after_permille: u16,
}

pub(crate) fn native_paragraph_attributes(
    event: &DocumentTextFlowEvent,
) -> Option<NativeParagraphAttributes> {
    if event.record_class() != Some(0x0010) {
        return None;
    }
    let words = event.raw_words();
    let (offset, after) = match words.len() {
        17 if words.get(3..7) == Some(&[0, 0x26, 5, 1]) => (0, 0),
        25 if words.get(3..11) == Some(&[0, 0x22, 2, 0, 0, 0x23, 2, 0])
            && words.get(12..15) == Some(&[0x26, 5, 1]) =>
        {
            (8, words[11])
        }
        _ => return None,
    };
    if words[8 + offset] != 0
        || words[10 + offset] != 0
        || words[11 + offset..13 + offset] != [0xffff, 0]
        || words[7 + offset] > 10_000
        || words[9 + offset] > 10_000
        || after > 2_000
    {
        return None;
    }
    Some(NativeParagraphAttributes {
        first_mm100: words[9 + offset],
        continuing_mm100: words[7 + offset],
        after_permille: after,
    })
}

pub(crate) fn native_paragraph_source_bounds(
    document: &Document,
    index: usize,
) -> Option<(usize, usize)> {
    let paragraph = paragraph_by_index(document, index)?;
    let mut spans = paragraph
        .inlines()
        .iter()
        .filter_map(|inline| match inline {
            Inline::Text(run) => run.source_span(),
            _ => None,
        });
    let first = spans.next()?;
    let last = spans.next_back().unwrap_or(first);
    Some((first.unit_start(), last.unit_end()))
}

pub(crate) fn native_paragraph_attrs_for_index(
    document: &Document,
    index: usize,
) -> NativeParagraphAttributes {
    let Some((start, _)) = native_paragraph_source_bounds(document, index) else {
        return NativeParagraphAttributes::default();
    };
    document
        .document_text_flow()
        .into_iter()
        .flat_map(DocumentTextFlow::events)
        .find(|event| event.unit_end() == start)
        .and_then(native_paragraph_attributes)
        .unwrap_or_default()
}

pub(crate) fn native_paragraph_line_x(
    document: &Document,
    layout: PageLayout,
    line: &PageTextLine,
) -> Option<f32> {
    line.native_line_mark_index?;
    let attrs = native_paragraph_attrs_for_index(document, line.paragraph_index()?);
    let indent = if line.char_start() == 0 {
        attrs.first_mm100
    } else {
        attrs.continuing_mm100
    };
    let x = layout.margin_left_px() + hundredth_millimeters_to_css_px(u32::from(indent));
    (x < layout.width_px() - layout.margin_right_px()).then_some(x)
}

pub(crate) fn native_paragraph_after_space(
    document: &Document,
    from: usize,
    to: usize,
) -> Option<f32> {
    let font = document_default_font_size_px(document)?;
    let mut extra = 0.0;
    for (index, _) in document
        .blocks()
        .iter()
        .filter(|block| matches!(block, Block::Paragraph(_)))
        .enumerate()
    {
        let (_, end) = native_paragraph_source_bounds(document, index)?;
        if from < end && end <= to {
            extra += font
                * f32::from(native_paragraph_attrs_for_index(document, index).after_permille)
                / 1000.0;
        }
    }
    Some(extra)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraph_indent_requires_the_observed_unit_and_zero_right_fields() {
        let event = |mode, right| {
            let words: [u16; 17] = [
                0x1c, 0x10, 17, 0, 0x26, 5, mode, 0, right, 1000, 0, 0xffff, 0, 17, 0, 0x10, 0x1f,
            ];
            let mut bytes = b"SsmgV.01".to_vec();
            bytes.extend([0; 12]);
            bytes.extend(b"TextV.01");
            bytes.extend(17_u32.to_be_bytes());
            bytes.extend(words.into_iter().flat_map(u16::to_be_bytes));
            DocumentTextFlow::from_map("/DocumentText", &bytes, &map_document_text(&bytes)).events()
                [0]
            .clone()
        };
        assert_eq!(
            native_paragraph_attributes(&event(1, 0))
                .unwrap()
                .first_mm100,
            1000
        );
        assert!(native_paragraph_attributes(&event(2, 0)).is_none());
        assert!(native_paragraph_attributes(&event(1, 1)).is_none());
    }

    #[test]
    #[ignore = "requires private native paragraph attribute pairs"]
    fn native_paragraph_lines_keep_first_continuing_indent_and_after_space() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/native-fixtures");
        for (name, indents, after) in [
            ("paragraph-indent-base", [0, 0, 0, 0, 0, 0], 0),
            ("paragraph-first-indent", [1000, 0, 0, 0, 0, 0], 0),
            ("paragraph-hanging-indent", [0, 1000, 1000, 0, 0, 0], 0),
            ("paragraph-after-space", [0, 0, 0, 0, 0, 0], 1000),
        ] {
            let doc =
                parse_document(&std::fs::read(root.join(format!("{name}.jtd"))).unwrap()).unwrap();
            let layout = page_layout_with_source_margins(&doc, page_layout_from_document(&doc));
            assert_eq!(
                native_paragraph_attrs_for_index(&doc, 0).after_permille,
                after
            );
            let plan = native_page_line_plan(&doc, layout, WritingMode::Horizontal)
                .unwrap_or_else(|| panic!("{name}"));
            let pages = native_pages_from_plan(&doc, &plan);
            assert_eq!(pages.len(), 1);
            assert_eq!(pages[0].len(), 6);
            for (index, line) in pages[0].iter().enumerate() {
                let x = native_paragraph_line_x(&doc, layout, line).unwrap();
                assert!(
                    (x - layout.margin_left_px() - hundredth_millimeters_to_css_px(indents[index]))
                        .abs()
                        < 0.001,
                    "{name}/{index}"
                );
                let (_, y, _) =
                    native_rule_line_placement(&doc, layout, line.native_line_mark_index.unwrap())
                        .unwrap();
                let extra = if index >= 3 {
                    document_default_font_size_px(&doc).unwrap() * f32::from(after) / 1000.0
                } else {
                    0.0
                };
                assert!(
                    (y - layout.margin_top_px()
                        - index as f32 * hundredth_millimeters_to_css_px(592)
                        - extra)
                        .abs()
                        < 0.002,
                    "{name}/{index}"
                );
            }
            let core = DocumentCore::from_document(doc);
            assert_eq!(core.page_count(), 1);
            let svg = core.render_page_svg(0).unwrap();
            assert_eq!(svg.matches("PARA-A").count(), 1);
            assert_eq!(svg.matches("PARA-B").count(), 1);
            assert_eq!(
                core.get_page_layer_tree(0)
                    .unwrap()
                    .matches("\"pageAssignmentCandidate\":true")
                    .count(),
                6
            );
        }
    }
}
