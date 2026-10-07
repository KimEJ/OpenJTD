use super::source::*;
use crate::*;

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
    let plain_records = flow
        .events()
        .iter()
        .filter(|event| native_plain_paragraph_fixed_pitch(event).is_some())
        .count();
    let plain_pitches = if plain_records == 0 {
        Vec::new()
    } else {
        document
            .blocks()
            .iter()
            .enumerate()
            .filter_map(|(index, _)| {
                let (from, to) = native_paragraph_source_bounds(document, index)?;
                let event = flow
                    .events()
                    .iter()
                    .find(|event| event.unit_end() == from)?;
                Some((from, to, native_plain_paragraph_fixed_pitch(event)?))
            })
            .collect::<Vec<_>>()
    };
    if plain_records != plain_pitches.len()
        || (!plain_pitches.is_empty()
            && (modern_source_writing_mode(document) != Some(WritingMode::Horizontal)
                || flow
                    .events()
                    .iter()
                    .any(|event| native_rule_parent_offset(event).is_some())))
    {
        return None;
    }
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
        let fixed = header.and_then(native_rule_fixed_pitch).or_else(|| {
            plain_pitches
                .iter()
                .find(|(from, to, _)| interval.unit_start < *to && *from < interval.unit_end)
                .map(|(_, _, pitch)| *pitch)
        });
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
