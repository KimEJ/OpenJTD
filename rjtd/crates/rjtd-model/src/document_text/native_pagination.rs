mod render;
mod source;

pub(crate) use render::*;
pub(crate) use source::{
    native_toc_cached_entries, native_toc_context_record, native_toc_setting_line,
    native_toc_source_scope,
};

#[cfg(test)]
mod tests {
    use super::source::*;
    use super::*;
    use crate::*;

    #[test]
    fn source_page_admission_requires_complete_cache_and_known_heading_profiles() {
        let flow = |units: &[u16]| {
            let mut bytes = vec![0; 32];
            bytes[..8].copy_from_slice(b"SsmgV.01");
            bytes[20..28].copy_from_slice(b"TextV.01");
            bytes[28..32].copy_from_slice(&(units.len() as u32).to_be_bytes());
            for word in units {
                bytes.extend(word.to_be_bytes());
            }
            DocumentTextFlow::from_map("/DocumentText", &bytes, &map_document_text(&bytes))
        };
        let mut cache = [0x1c, 1, 7, 0, 0, 1, 0x1d, 65, 0x1e, 5, 0, 1, 0x1f];
        assert!(inline_cache_group(flow(&cache).events()));
        cache[5] = 2;
        assert!(!inline_cache_group(flow(&cache).events()));
        let mut heading = [0x1c, 0x10, 13, 0, 0x2e, 1, 1, 0xffff, 0, 13, 0, 0x10, 0x1f];
        assert!(heading_or_number_record(&flow(&heading).events()[0]));
        heading[6] = 4;
        assert!(!heading_or_number_record(&flow(&heading).events()[0]));
        assert_eq!(char_offset_at_utf16_unit("A😀B", 3), Some(2));
        assert_eq!(char_offset_at_utf16_unit("A😀B", 2), None);
    }

    #[test]
    fn source_pagination_leaves_unframed_paragraphs_on_the_fallback_path() {
        let doc = Document::new(
            Metadata::default(),
            vec![Block::Paragraph(Paragraph::from_text("SIMPLE BODY"))],
        );
        let layout = PageLayout::default();
        assert!(native_page_line_plan(&doc, layout, WritingMode::Horizontal).is_none());
        let pages = paginate_document_text(&doc, layout, WritingMode::Horizontal);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0][0].text(), "SIMPLE BODY");
        assert!(pages[0][0].native_line_mark_index.is_none());
    }

    #[test]
    #[ignore = "requires private native pagination pairs"]
    fn native_source_pages_preserve_page_ranges_and_table_text() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/native-fixtures");
        for (name, page_count, line_count, table_pages) in [
            ("table-near-bottom", 1, 40, [0, 0, 0]),
            ("table-page2", 2, 53, [1, 1, 1]),
            ("table-cross-page", 2, 44, [0, 0, 1]),
        ] {
            let doc =
                parse_document(&std::fs::read(root.join(format!("{name}.jtd"))).unwrap()).unwrap();
            let layout = page_layout_with_source_margins(&doc, page_layout_from_document(&doc));
            let plan = native_page_line_plan(&doc, layout, WritingMode::Horizontal)
                .unwrap_or_else(|| panic!("source page plan rejected {name}"));
            assert_eq!(plan.len(), line_count, "{name}");
            let shape = page_construction_shape(&doc, layout, WritingMode::Horizontal).unwrap();
            assert_eq!(shape.pages, page_count);
            assert_eq!(shape.lines, line_count);
            for page in 1..=page_count {
                let rules =
                    native_rule_border_projection(&doc, layout, page, WritingMode::Horizontal)
                        .unwrap();
                assert!(
                    rules
                        .iter()
                        .all(|segment| segment.points[1] >= layout.margin_top_px())
                );
            }
            assert!(matches!(
                DocumentCore::from_document_with_limits(
                    doc.clone(),
                    ParseLimits::DEFAULT.with_max_page_lines(line_count - 1)
                ),
                Err(Error::ResourceLimit {
                    resource: "document page lines",
                    ..
                })
            ));
            assert!(matches!(
                DocumentCore::from_document_with_limits(
                    doc.clone(),
                    ParseLimits::DEFAULT.with_max_pages(page_count - 1)
                ),
                Err(Error::ResourceLimit {
                    resource: "document pages",
                    ..
                })
            ));
            let mut edited = doc.clone();
            if let Block::Paragraph(paragraph) = &mut edited.blocks[0] {
                *paragraph = Paragraph::from_text("EDITED".to_string());
            }
            assert!(native_page_line_plan(&edited, layout, WritingMode::Horizontal).is_none());
            let core = DocumentCore::from_document(doc);
            assert_eq!(core.page_count() as usize, page_count, "{name}");
            for page in 0..page_count {
                let svg = core.render_page_svg(page as u32).unwrap();
                for (row, owner) in table_pages.iter().enumerate() {
                    for column in 1..=2 {
                        let label = format!(">R0{}C0{column}</text>", row + 1);
                        assert_eq!(
                            svg.matches(&label).count(),
                            usize::from(page == *owner),
                            "{name}: {page}/{label}"
                        );
                    }
                }
                assert_eq!(
                    svg.matches("BODY-AFTER").count(),
                    usize::from(page + 1 == page_count),
                    "{name}"
                );
            }
            assert_eq!(core.pages.iter().map(Vec::len).sum::<usize>(), line_count);
            assert!(
                core.pages
                    .iter()
                    .flatten()
                    .all(|line| line.native_line_mark_index.is_some())
            );
        }
    }
}
