#[cfg(feature = "rendering")]
mod render;
mod source;

#[cfg(feature = "rendering")]
pub(crate) use render::*;
pub(crate) use source::*;

#[cfg(all(test, feature = "rendering"))]
mod tests {
    use super::*;
    use crate::*;

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

pub use source::NativeParagraphAttributes as DocumentParagraphAttributeCandidate;
