mod render;
mod source;

pub(crate) use render::*;
pub(crate) use source::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use rjtd_core::document_text::DocumentTextStyleTypedValue;

    #[test]
    fn fixed_pitch_requires_matching_repeated_attributes() {
        let event = |second| {
            let words: [u16; 16] = [
                0x1c, 0x10, 16, 0, 0x20, 4, 8, 1000, 8, second, 0xffff, 0, 16, 0, 0x10, 0x1f,
            ];
            let mut bytes = b"SsmgV.01".to_vec();
            bytes.extend([0; 12]);
            bytes.extend(b"TextV.01");
            bytes.extend((words.len() as u32).to_be_bytes());
            bytes.extend(words.into_iter().flat_map(u16::to_be_bytes));
            let flow =
                DocumentTextFlow::from_map("/DocumentText", &bytes, &map_document_text(&bytes));
            flow.events()
                .iter()
                .find(|event| event.record_class() == Some(0x10))
                .unwrap()
                .clone()
        };
        assert_eq!(native_rule_fixed_pitch(&event(1000)), Some(1000));
        assert_eq!(native_rule_fixed_pitch(&event(999)), None);
        assert_eq!(native_rule_parent_offset(&event(1000)), None);
        let span = TextSourceSpan::new(0, 12, 0, 6);
        assert_eq!(native_rule_visible_span("  TEXT", &span).unit_start(), 2);
        assert_eq!(native_rule_visible_span("      ", &span), span);
    }

    #[test]
    #[ignore = "requires private native font and line-spacing pairs"]
    fn native_metrics_accumulate_font_height_and_fixed_pitch_without_reference_coordinates() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/native-fixtures");
        let base = hundredth_millimeters_to_css_px(592);
        let font_extra = hundredth_millimeters_to_css_px(124);
        let pitch_extra = hundredth_millimeters_to_css_px(408);
        for (name, labels, rows, font_deltas, fixed_delta) in [
            (
                "table-font14",
                vec!["R01C01", "R02C01", "R03C01", "BODY-AFTER"],
                vec![2, 4, 6, 8],
                vec![0, 1, 2, 3],
                0,
            ),
            (
                "table-body-font14",
                vec!["R01C01", "R02C01", "R03C01", "BODY-AFTER"],
                vec![2, 4, 6, 8],
                vec![1, 1, 1, 1],
                0,
            ),
            (
                "table-cell-line-spacing",
                vec!["R01C01", "LINE-B", "R02C01", "R03C01", "BODY-AFTER"],
                vec![2, 3, 5, 7, 9],
                vec![0, 0, 0, 0, 0],
                3,
            ),
            (
                "table-body-line-spacing",
                vec!["R01C01", "R02C01", "R03C01", "BODY-AFTER"],
                vec![2, 4, 6, 8],
                vec![0, 0, 0, 0],
                1,
            ),
        ] {
            let doc =
                parse_document(&std::fs::read(root.join(format!("{name}.jtd"))).unwrap()).unwrap();
            let layout = page_layout_with_source_margins(&doc, page_layout_from_document(&doc));
            if name == "table-font14" {
                let tables =
                    native_control_table_text_projections(&doc, layout, 1, WritingMode::Horizontal);
                let slots = tables[0]
                    .slots
                    .iter()
                    .filter(|slot| slot.column_index == 0)
                    .collect::<Vec<_>>();
                assert_eq!(slots.len(), 3);
                let resolver = document_text_style_resolver(&doc).unwrap();
                for (row, slot) in slots.iter().enumerate() {
                    assert_eq!(slot.leading_space_count, 3);
                    let padding = slot.source_span.subspan_by_units(0, 3);
                    assert_eq!(
                        resolver.uniform_optional_value_in_range(
                            padding.unit_start(),
                            padding.unit_end(),
                            2
                        ),
                        Some(if row == 0 {
                            None
                        } else {
                            Some(DocumentTextStyleTypedValue::U16(494))
                        })
                    );
                    if row > 0 {
                        assert!(slot.x > slots[0].x);
                        assert!(
                            (slot.padding_width_px / slots[0].padding_width_px - 494.0 / 370.0)
                                .abs()
                                < 0.0001
                        );
                    }
                }
            }
            for (index, record) in rows.iter().enumerate() {
                let (page, top, _) = native_rule_line_placement(&doc, layout, *record).unwrap();
                assert_eq!(page, 1);
                let expected = layout.margin_top_px()
                    + *record as f32 * base
                    + font_deltas[index] as f32 * font_extra
                    + if fixed_delta != 0 && *record >= fixed_delta {
                        pitch_extra
                    } else {
                        0.0
                    };
                assert!(
                    (top - expected).abs() < 0.002,
                    "{name}/{record}: {top}/{expected}"
                );
            }
            let core = DocumentCore::from_document(doc);
            if name != "table-cell-line-spacing" {
                assert!(
                    native_page_line_plan(&core.document, layout, WritingMode::Horizontal)
                        .is_some(),
                    "{name}"
                );
            }
            let svg = core.render_page_svg(0).unwrap();
            for label in labels {
                assert_eq!(
                    svg.matches(&format!(">{label}</text>")).count()
                        + svg.matches(&format!(">  {label}</text>")).count(),
                    1,
                    "{name}/{label}"
                );
            }
            assert!(!svg.contains("rjtd-column-grid-candidate"), "{name}");
            assert!(
                core.get_page_layer_tree(0)
                    .unwrap()
                    .contains("nativeRuleBorderProjection"),
                "{name}"
            );
        }
    }
}
