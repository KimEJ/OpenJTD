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
    if writing_mode.is_vertical() || !layout.has_source_margins() {
        return None;
    }
    let flow = document.document_text_flow()?;
    let has_rules = flow
        .events()
        .iter()
        .any(|event| native_rule_parent_offset(event).is_some());
    let toc_scope = native_toc_source_scope(document);
    if has_rules && !native_rule_grid_admitted(document, layout, writing_mode) {
        return None;
    }
    let mut event_index = 0;
    while let Some(event) = flow.events().get(event_index) {
        if !has_rules
            && inline_cache_group(
                flow.events()
                    .get(event_index..event_index + 4)
                    .unwrap_or(&[]),
            )
        {
            event_index += 4;
            continue;
        }
        if match event.kind() {
            DocumentTextFlowKind::Text => event
                .text()
                .chars()
                .any(|c| c != '\n' && !native_rule_character_supported(c)),
            DocumentTextFlowKind::Record => {
                !(native_rule_parent_offset(event).is_some()
                    || native_rule_fixed_pitch(event).is_some()
                    || native_paragraph_attributes(event).is_some()
                    || (!has_rules && heading_or_number_record(event))
                    || (!has_rules
                        && toc_scope.is_some_and(|(from, to)| {
                            from <= event.unit_start() && event.unit_end() <= to
                        })
                        && native_toc_context_record(event))
                    || (has_rules && event.record_class() == Some(0x0030)))
            }
            DocumentTextFlowKind::Control => {
                event.code() != Some(0x000e) && (event.code() != Some(0x000c) || has_rules)
            }
            _ => true,
        } {
            return None;
        }
        event_index += 1;
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
            let span = native_visible_text_span(document, run.text(), run.source_span()?)?;
            let visible_span = native_rule_visible_span(run.text(), &span);
            document_text_font_size(&resolver, &visible_span, Some(default_font))?;
            runs.push((paragraph_index, offset, span, run.text()));
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
        let first_run =
            runs.partition_point(|(_, _, span, _)| span.unit_end() <= interval.unit_start);
        for (paragraph, offset, span, text) in runs[first_run..]
            .iter()
            .take_while(|(_, _, span, _)| span.unit_start() < interval.unit_end)
        {
            let from = span.unit_start().max(interval.unit_start);
            let to = span.unit_end().min(interval.unit_end);
            if from >= to {
                continue;
            }
            let start = offset + char_offset_at_utf16_unit(text, from - span.unit_start())?;
            let end = offset + char_offset_at_utf16_unit(text, to - span.unit_start())?;
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

fn char_offset_at_utf16_unit(text: &str, offset: usize) -> Option<usize> {
    let mut units = 0;
    for (index, character) in text.chars().enumerate() {
        if units == offset {
            return Some(index);
        }
        units += character.len_utf16();
    }
    (units == offset).then_some(text.chars().count())
}

fn inline_cache_group(events: &[DocumentTextFlowEvent]) -> bool {
    let [start, prefix, text, suffix] = events else {
        return false;
    };
    start.kind() == DocumentTextFlowKind::Control
        && start.code() == Some(0x1c)
        && prefix.kind() == DocumentTextFlowKind::Opaque
        && prefix.raw_words() == [1, 7, 0, 0, 1]
        && text.kind() == DocumentTextFlowKind::Inline
        && text.selector() == Some(1)
        && text
            .text()
            .chars()
            .all(|c| native_rule_character_supported(c) || c == '●')
        && suffix.kind() == DocumentTextFlowKind::Opaque
        && suffix.raw_words() == [5, 0, 1, 0x1f]
        && events
            .windows(2)
            .all(|pair| pair[0].unit_end() == pair[1].unit_start())
}

fn heading_or_number_record(event: &DocumentTextFlowEvent) -> bool {
    let words = event.raw_words();
    if words.len() != 13 {
        return false;
    }
    (words[..6] == [0x1c, 0x10, 13, 0, 0x2e, 1]
        && (1..=3).contains(&words[6])
        && words[7..] == [0xffff, 0, 13, 0, 0x10, 0x1f])
        || (words[..8] == [0x1c, 0, 13, 0, 10, 0, 391, 0x2010]
            && words[8] != 0xffff
            && words[9..] == [13, 0, 0, 0x1f])
}

pub(crate) fn native_toc_section_record(event: &DocumentTextFlowEvent) -> Option<u16> {
    let words = event.raw_words();
    if event.kind() != DocumentTextFlowKind::Record
        || words.len() != 12
        || words[..4] != [0x1c, 0x20, 12, 0]
        || words[5..] != [0, 0, 0, 12, 0, 0x20, 0x1f]
        || !matches!(words[4], 0x30 | 0x31)
    {
        return None;
    }
    Some(words[4])
}

pub(crate) fn native_toc_source_scope(document: &Document) -> Option<(usize, usize)> {
    let sections = document
        .document_text_flow()?
        .events()
        .iter()
        .filter_map(|event| native_toc_section_record(event).map(|kind| (kind, event)))
        .collect::<Vec<_>>();
    let [(0x30, start), (0x31, end)] = sections.as_slice() else {
        return None;
    };
    (start.unit_end() < end.unit_start()).then_some((start.unit_start(), end.unit_end()))
}

fn native_toc_context_record(event: &DocumentTextFlowEvent) -> bool {
    if native_toc_section_record(event).is_some() {
        return true;
    }
    let words = event.raw_words();
    // Admit only the controlled tab/leader framing. Horizontal stops and
    // leader paint remain undecoded; this path owns source pages and rows only.
    words
        == [
            0x1c, 0, 17, 0, 9, 375, 31, 0x90, 0, 2, 0xf81e, 0, 0, 17, 0, 0, 0x1f,
        ]
        || (words.len() == 18
            && words[..8] == [0x1c, 0, 18, 0, 21, 0, 23, 0x90]
            && matches!(words[8], 1 | 100)
            && words[9..] == [2, 0xa77c, 0, 0, 0, 18, 0, 0, 0x1f])
}

pub(crate) fn native_toc_setting_line(
    flow: &DocumentTextFlow,
    scope: Option<(usize, usize)>,
    from: usize,
    to: usize,
) -> bool {
    scope.is_some_and(|(start, end)| start <= from && to <= end.saturating_add(1))
        && flow
            .events()
            .iter()
            .any(|event| event.unit_start() == from && native_toc_section_record(event).is_some())
        && flow
            .events()
            .iter()
            .filter(|event| event.unit_start() < to && from < event.unit_end())
            .all(|event| {
                if matches!(
                    event.kind(),
                    DocumentTextFlowKind::Text | DocumentTextFlowKind::Inline
                ) {
                    return matches!(
                        text_by_utf16_units(
                            event.text(),
                            from.max(event.unit_start()) - event.unit_start(),
                            to.min(event.unit_end()) - event.unit_start(),
                        )
                        .as_str(),
                        "" | "\n" | "\r" | "\r\n"
                    );
                }
                native_toc_section_record(event).is_some()
            })
}

pub(crate) fn native_toc_cached_entries(document: &Document) -> Vec<DocumentTocEntry> {
    let Some((from, to)) = native_toc_source_scope(document) else {
        return Vec::new();
    };
    let Some(flow) = document.document_text_flow() else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    for (index, _) in document
        .blocks()
        .iter()
        .filter(|block| matches!(block, Block::Paragraph(_)))
        .enumerate()
    {
        let Some((start, end)) = native_paragraph_source_bounds(document, index) else {
            continue;
        };
        if start < from || end > to {
            continue;
        }
        let Some(paragraph) = paragraph_by_index(document, index) else {
            return Vec::new();
        };
        let [Inline::Text(title), Inline::Text(label)] = paragraph.inlines() else {
            return Vec::new();
        };
        let Some(title_span) = title
            .source_span()
            .and_then(|span| native_visible_text_span(document, title.text(), span))
        else {
            return Vec::new();
        };
        let Some(label_span) = label
            .source_span()
            .and_then(|span| native_visible_text_span(document, label.text(), span))
        else {
            return Vec::new();
        };
        let records = flow
            .events()
            .iter()
            .filter(|event| {
                event.kind() == DocumentTextFlowKind::Record
                    && title_span.unit_end() <= event.unit_start()
                    && event.unit_end() <= label_span.unit_start()
            })
            .collect::<Vec<_>>();
        if title.text().is_empty()
            || label.text().is_empty()
            || !matches!(records.len(), 1 | 2)
            || records[0].raw_words().get(2) != Some(&17)
            || (records.len() == 2 && records[1].raw_words().get(2) != Some(&18))
            || records
                .iter()
                .any(|event| !native_toc_context_record(event))
        {
            return Vec::new();
        }
        entries.push(DocumentTocEntry::new(
            title.text(),
            label.text(),
            TextSourceSpan::new(
                title_span.byte_start(),
                label_span.byte_end(),
                title_span.unit_start(),
                label_span.unit_end(),
            ),
        ));
    }
    entries
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
