use super::source::*;
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
    let sections = native_section_layouts(document, layout);
    if document
        .unknown_styles()
        .iter()
        .any(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        && sections.is_none()
    {
        return None;
    }
    if (writing_mode.is_vertical()
        && modern_source_writing_mode(document) != Some(WritingMode::VerticalRl))
        || !layout.has_source_margins()
    {
        return None;
    }
    let flow = document.document_text_flow()?;
    let has_rules = flow
        .events()
        .iter()
        .any(|event| native_rule_parent_offset(event).is_some());
    let toc_scope = native_toc_source_scope(document);
    let fields = document.text_field_candidates();
    let tatechuyoko = document.tatechuyoko_candidates();
    if writing_mode.is_vertical() && (has_rules || !fields.is_empty()) {
        return None;
    }
    if has_rules && !native_rule_grid_admitted(document, layout, writing_mode) {
        return None;
    }
    let mut event_index = 0;
    while let Some(event) = flow.events().get(event_index) {
        if let Some(end) = native_tatechuyoko_end(&tatechuyoko, event.unit_start()) {
            while flow
                .events()
                .get(event_index)
                .is_some_and(|event| event.unit_start() < end)
            {
                event_index += 1;
            }
            continue;
        }
        if !has_rules
            && let Some(end) = native_field_end(&fields, flow.events(), event.unit_start())
        {
            while flow
                .events()
                .get(event_index)
                .is_some_and(|event| event.unit_start() < end)
            {
                event_index += 1;
            }
            continue;
        }
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
                    || (!has_rules && sections.is_some() && native_section_marker(event).is_some())
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
    let runs = native_page_source_runs(document)?;
    let resolver = document_text_style_resolver(document)?;
    let default_font = document_default_font_size_px(document)?;
    for (_, _, span, text) in &runs {
        let visible_span = native_rule_visible_span(text, span);
        document_text_font_size(&resolver, &visible_span, Some(default_font))?;
    }
    let intervals = native_page_source_intervals(document)?;
    let mut plan = Vec::with_capacity(intervals.len());
    let mut previous_page = 0;
    for (expected_record, interval) in intervals.iter().enumerate() {
        let source = native_page_source_line_range(interval, &runs)?;
        let (page, _, _) = native_rule_line_placement(document, layout, source.record)?;
        let page = page.checked_sub(1)?;
        if expected_record == 0 && page != 0 {
            return None;
        }
        if page != previous_page && page != previous_page + 1 {
            return None;
        }
        previous_page = page;
        plan.push(NativePageLinePlan {
            page,
            record: source.record,
            paragraph: source.paragraph,
            start: source.start,
            end: source.end,
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
