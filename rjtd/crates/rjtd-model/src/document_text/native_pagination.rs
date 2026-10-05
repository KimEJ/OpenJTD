use crate::*;

/// Physical source lines retain paragraph/character addressing, not table rows.
pub(crate) struct NativePageLinePlan {
    page: usize,
    record: usize,
    paragraph: Option<usize>,
    start: usize,
    end: usize,
}

pub(crate) fn native_page_line_plan(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Option<Vec<NativePageLinePlan>> {
    if document
        .unknown_styles()
        .iter()
        .any(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
    {
        return None;
    }
    if !native_rule_grid_admitted(document, layout, writing_mode) {
        return None;
    }
    let flow = document.document_text_flow()?;
    if flow.events().iter().any(|event| match event.kind() {
        DocumentTextFlowKind::Text => event
            .text()
            .chars()
            .any(|c| c != '\n' && !native_rule_character_supported(c)),
        DocumentTextFlowKind::Record => !matches!(event.record_class(), Some(0x0010 | 0x0030)),
        DocumentTextFlowKind::Control => event.code() != Some(0x000e),
        _ => true,
    }) {
        return None;
    }
    let mut runs = Vec::new();
    let resolver = document_text_style_resolver(document)?;
    let default_font = document_default_font_size_px(document)?;
    for (paragraph_index, block) in document.blocks().iter().enumerate() {
        let Block::Paragraph(paragraph) = block else {
            return None;
        };
        let mut offset = 0;
        for inline in paragraph.inlines() {
            let Inline::Text(run) = inline else {
                return None;
            };
            let span = run.source_span()?;
            let visible_span = native_rule_visible_span(run.text(), span);
            document_text_font_size(&resolver, &visible_span, Some(default_font))?;
            if span.unit_end() - span.unit_start() != run.text().chars().count()
                || flow.text_for_range(
                    span.unit_start(),
                    span.unit_end(),
                    TextCountRangeOverlapBasis::Unit,
                ) != run.text()
            {
                return None;
            }
            runs.push((paragraph_index, offset, span));
            offset += run.text().chars().count();
        }
    }
    if runs
        .windows(2)
        .any(|pair| pair[0].2.unit_end() > pair[1].2.unit_start())
    {
        return None;
    }
    let intervals = shanai_lan_line_mark_intervals(document);
    if intervals.first()?.unit_start != flow.source_span().unit_start()
        || intervals.last()?.unit_end != flow.source_span().unit_end().checked_add(1)?
    {
        return None;
    }
    let mut plan = Vec::with_capacity(intervals.len());
    let mut previous_page = 0;
    for (expected_record, interval) in intervals.iter().enumerate() {
        if expected_record != interval.record_index {
            return None;
        }
        let (page, _, _) = native_rule_line_placement(document, layout, interval.record_index)?;
        let page = page.checked_sub(1)?;
        if expected_record == 0 && page != 0 {
            return None;
        }
        if page != previous_page && page != previous_page + 1 {
            return None;
        }
        previous_page = page;
        let mut range: Option<(usize, usize, usize)> = None;
        let first_run = runs.partition_point(|(_, _, span)| span.unit_end() <= interval.unit_start);
        for (paragraph, offset, span) in runs[first_run..]
            .iter()
            .take_while(|(_, _, span)| span.unit_start() < interval.unit_end)
        {
            let from = span.unit_start().max(interval.unit_start);
            let to = span.unit_end().min(interval.unit_end);
            if from >= to {
                continue;
            }
            let start = offset + from - span.unit_start();
            let end = offset + to - span.unit_start();
            if let Some((previous, _, previous_end)) = range {
                if previous != *paragraph || previous_end != start {
                    return None;
                }
                range.as_mut()?.2 = end;
            } else {
                range = Some((*paragraph, start, end));
            }
        }
        let (paragraph, start, end) = range.map_or((None, 0, 0), |(paragraph, start, end)| {
            (Some(paragraph), start, end)
        });
        plan.push(NativePageLinePlan {
            page,
            record: interval.record_index,
            paragraph,
            start,
            end,
        });
    }
    Some(plan)
}

pub(crate) fn native_page_output_shape(plan: &[NativePageLinePlan]) -> PageOutputShape {
    PageOutputShape {
        pages: plan.last().map_or(1, |line| line.page + 1),
        lines: plan.len(),
    }
}

pub(crate) fn native_pages_from_plan(
    document: &Document,
    plan: &[NativePageLinePlan],
) -> Vec<Vec<PageTextLine>> {
    let mut pages = vec![Vec::new(); native_page_output_shape(plan).pages];
    let paragraphs = document_paragraph_texts(document);
    for entry in plan {
        let text = entry
            .paragraph
            .and_then(|index| paragraphs.get(index))
            .map(|(_, text)| text_by_char_range(text, entry.start, entry.end))
            .unwrap_or_default();
        let mut line = PageTextLine::new(text, entry.paragraph, entry.start, entry.end);
        line.native_line_mark_index = Some(entry.record);
        pages[entry.page].push(line);
    }
    pages
}

#[cfg(test)]
mod tests {
    use super::*;

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
