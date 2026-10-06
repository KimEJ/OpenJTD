use crate::*;

/// Observed repeated fixed-pitch attributes, kept as candidates in source flow.
pub(crate) fn native_rule_fixed_pitch(event: &DocumentTextFlowEvent) -> Option<u16> {
    let words = event.raw_words();
    (event.record_class() == Some(0x0010)
        && words.len() >= 16
        && words[3..7] == [0, 0x20, 4, 8]
        && words[8] == 8
        && words[7] == words[9]
        && (1..=5_000).contains(&words[7]))
    .then(|| words[7])
}

pub(crate) fn native_rule_parent_offset(event: &DocumentTextFlowEvent) -> Option<usize> {
    if event.record_class() != Some(0x0010) || event.raw_words().get(3) != Some(&0) {
        return None;
    }
    match event.raw_words().get(4) {
        Some(0x008f) => Some(0),
        Some(0x20)
            if native_rule_fixed_pitch(event).is_some()
                && event.raw_words().get(10) == Some(&0x008f) =>
        {
            Some(6)
        }
        _ => None,
    }
}

pub(crate) fn native_rule_visible_span(text: &str, span: &TextSourceSpan) -> TextSourceSpan {
    let leading = text.chars().take_while(|c| *c == ' ').count();
    if leading == text.chars().count() {
        span.clone()
    } else {
        span.subspan_by_units(leading, text.encode_utf16().count())
    }
}

pub(crate) fn native_rule_line_placement(
    document: &Document,
    layout: PageLayout,
    record_index: usize,
) -> Option<(usize, f32, u16)> {
    let mark = document.page_marks().first()?;
    if mark.family() != "fixed84" {
        return None;
    }
    let entries = mark
        .entries()
        .iter()
        .filter(|entry| {
            entry
                .line_start()
                .zip(entry.line_end())
                .is_some_and(|(start, end)| {
                    start as usize <= record_index && record_index <= end as usize
                })
        })
        .collect::<Vec<_>>();
    let [entry] = entries.as_slice() else {
        return None;
    };
    let sections = native_section_layouts(document, layout);
    if entry.flags() != Some(0x10000) && !(entry.flags() == Some(0x50100) && sections.is_some()) {
        return None;
    }
    let page = (entry.index()? as usize).checked_add(1)?;
    let layout = sections
        .as_ref()
        .and_then(|layouts| layouts.get(page - 1))
        .copied()
        .unwrap_or(layout);
    let start_record = entry.line_start()? as usize;
    let font = document_default_font_size_px(document)?;
    let default_mm100 = (font * 2540.0 / 96.0).round() as u16;
    let fields = entry.u16_fields();
    if fields.get(19) != Some(&default_mm100) {
        return None;
    }
    let gap_mm100 = *fields.get(14)?;
    let base_mm100 = default_mm100.checked_add(gap_mm100)?;
    let base = hundredth_millimeters_to_css_px(u32::from(base_mm100));
    if !(APP_FONT_SIZE_PX..=APP_LINE_HEIGHT_PX * 1.25).contains(&base) {
        return None;
    }
    if fields.get(21) != Some(&base_mm100) && fields.get(20) != Some(&8) {
        return None;
    }
    let flow = document.document_text_flow()?;
    let intervals = shanai_lan_line_mark_intervals(document);
    let resolver = document_text_style_resolver(document)?;
    let toc_scope = native_toc_source_scope(document);
    let mut top = layout.margin_top_px();
    // ponytail: scan only this page's source-line prefix; cache page metrics if large ruled documents need it.
    for interval in intervals.iter().filter(|interval| {
        start_record <= interval.record_index && interval.record_index < record_index
    }) {
        if sections.is_some()
            && native_section_setting_line(flow, interval.unit_start, interval.unit_end)
        {
            continue;
        }
        if native_toc_setting_line(flow, toc_scope, interval.unit_start, interval.unit_end) {
            continue;
        }
        let mut largest = font;
        for event in flow
            .events()
            .iter()
            .filter(|event| event.kind() == DocumentTextFlowKind::Text)
        {
            let from = interval.unit_start.max(event.unit_start());
            let to = interval.unit_end.min(event.unit_end());
            if from >= to {
                continue;
            }
            let text = text_by_utf16_units(
                event.text(),
                from - event.unit_start(),
                to - event.unit_start(),
            );
            let span = TextSourceSpan::new(from * 2, to * 2, from, to);
            for part in source_text_parts(&text, Some(&span))
                .iter()
                .filter(|part| !part.text.is_empty())
            {
                let span = native_rule_visible_span(&part.text, part.source_span.as_ref()?);
                largest = largest.max(document_text_font_size(&resolver, &span, Some(font))?.px);
            }
        }
        let header = flow
            .events()
            .iter()
            .find(|event| event.unit_start() == interval.unit_start);
        let fixed = header.and_then(native_rule_fixed_pitch);
        if header.is_some_and(|event| {
            event.record_class() == Some(0x0010) && event.raw_words().get(4) == Some(&0x20)
        }) && fixed.is_none()
        {
            return None;
        }
        let advance = fixed
            .map(|pitch| hundredth_millimeters_to_css_px(u32::from(pitch)))
            .unwrap_or(largest + hundredth_millimeters_to_css_px(u32::from(gap_mm100)));
        if !advance.is_finite() || advance < largest {
            return None;
        }
        top += advance;
        top += native_paragraph_after_space(document, interval.unit_start, interval.unit_end)?;
    }
    (top.is_finite() && top < layout.height_px()).then_some((page, top, base_mm100))
}

#[cfg(test)]
mod tests {
    use super::*;

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
