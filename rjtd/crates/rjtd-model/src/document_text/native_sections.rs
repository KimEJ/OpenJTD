use crate::*;

pub(crate) fn native_section_marker(event: &DocumentTextFlowEvent) -> Option<u16> {
    let words = event.raw_words();
    if event.kind() != DocumentTextFlowKind::Record
        || words.len() != 12
        || words[..5] != [0x1c, 0x20, 12, 0, 0x10]
        || words[6..] != [0, 1, 12, 0, 0x20, 0x1f]
        || words[5] > 1
    {
        return None;
    }
    Some(words[5])
}

/// Bounded single explicit style, apply/reset and portrait/landscape/portrait pages.
/// Other page-style associations retain the global fallback.
pub(crate) fn native_section_layouts(
    document: &Document,
    base: PageLayout,
) -> Option<Vec<PageLayout>> {
    let styles = document
        .unknown_styles()
        .iter()
        .filter(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        .collect::<Vec<_>>();
    let [style] = styles.as_slice() else {
        return None;
    };
    let summary = summarize_style_stream(style.payload());
    if summary.records().len() != 1 {
        return None;
    }
    let records = summary
        .records()
        .iter()
        .filter(|record| record.code() == PAGE_LAYOUT_STYLE_RECORD_CODE)
        .collect::<Vec<_>>();
    let [record] = records.as_slice() else {
        return None;
    };
    let custom = page_layout_from_page_layout_style(style.payload())?;
    let view = document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))?;
    let default = page_layout_from_document_view_styles(view.payload())?;
    if (default.width_px() - custom.height_px()).abs() > 0.1
        || (default.height_px() - custom.width_px()).abs() > 0.1
        || default.width_px() >= default.height_px()
    {
        return None;
    }
    let margin_payload = record
        .subrecords()
        .iter()
        .find(|sub| sub.code() == 0x4002)?
        .payload();
    if !matches!(margin_payload.len(), 39 | 40) || margin_payload.get(..3) != Some(&[0xfe, 0x80, 0])
    {
        return None;
    }
    let margins = page_margins_at(margin_payload, 3)?;
    let view_summary = summarize_style_stream(view.payload());
    let view_margin = view_summary
        .records()
        .iter()
        .find(|record| record.code() == 0x1002)?;
    let offset = view_margin.offset().checked_add(4)?;
    let bytes = view
        .payload()
        .get(offset..offset.checked_add(view_margin.payload_len())?)?;
    if bytes.len() != 32
        || bytes.get(..2) != Some(&[0, 0xd8])
        || page_margins_at(bytes, 2)? != margins
    {
        return None;
    }
    let flow = document.document_text_flow()?;
    let changes = flow
        .events()
        .iter()
        .filter(|event| {
            event.record_class() == Some(0x20) && event.raw_words().get(4) == Some(&0x10)
        })
        .map(|event| native_section_marker(event).map(|id| (id, event)))
        .collect::<Option<Vec<_>>>()?;
    let [(1, apply), (0, reset)] = changes.as_slice() else {
        return None;
    };
    let intervals = shanai_lan_line_mark_intervals(document);
    let mark = document.page_marks().first()?;
    if mark.family() != "fixed84" {
        return None;
    }
    let last = intervals.last()?.record_index;
    let entries = mark
        .entries()
        .iter()
        .filter(|entry| {
            entry
                .line_start()
                .is_some_and(|start| start as usize <= last)
        })
        .collect::<Vec<_>>();
    if entries.len() != 3 {
        return None;
    }
    let mut result = Vec::new();
    for (page, entry) in entries.iter().enumerate() {
        if entry.index()? as usize != page
            || entry.flags()? != if page == 1 { 0x50100 } else { 0x10000 }
        {
            return None;
        }
        let start = entry.line_start()? as usize;
        let end = entry.line_end()? as usize;
        if start > end
            || (page > 0 && entries[page - 1].line_end()?.checked_add(1)? as usize != start)
        {
            return None;
        }
        if page == 1
            && (intervals.get(start)?.unit_start != apply.unit_start()
                || intervals.get(end)?.unit_start != reset.unit_start())
        {
            return None;
        }
        let size = if page == 1 { custom } else { default };
        result.push(PageLayout {
            width_px: size.width_px(),
            height_px: size.height_px(),
            landscape: page == 1,
            source_margins: Some(margins),
            ..base
        });
    }
    Some(result)
}

pub(crate) fn native_section_setting_line(flow: &DocumentTextFlow, from: usize, to: usize) -> bool {
    flow.events()
        .iter()
        .any(|event| event.unit_start() == from && native_section_marker(event).is_some())
        && flow
            .events()
            .iter()
            .filter(|event| event.unit_start() < to && from < event.unit_end())
            .all(|event| {
                if event.kind() == DocumentTextFlowKind::Text {
                    return matches!(
                        text_by_utf16_units(
                            event.text(),
                            from.max(event.unit_start()) - event.unit_start(),
                            to.min(event.unit_end()) - event.unit_start()
                        )
                        .as_str(),
                        "" | "\n" | "\r" | "\r\n"
                    );
                }
                native_section_marker(event).is_some()
            })
}

/// Only the corroborated middle-page style: same default font, 100% scales,
/// known 60% spacing and line profile. General page-style inheritance is unproven.
pub(crate) fn native_section_spacing_candidate(
    document: &Document,
    layout: PageLayout,
    page: usize,
) -> Option<u16> {
    if page != 2
        || modern_view_writing_mode(document.unknown_styles()) != Some(WritingMode::Horizontal)
    {
        return None;
    }
    let layouts = native_section_layouts(document, layout)?;
    let selected = layouts.get(page - 1)?;
    if (selected.width_px() - layout.width_px()).abs() > 0.1
        || (selected.height_px() - layout.height_px()).abs() > 0.1
        || (selected.margin_left_px() - layout.margin_left_px()).abs() > 0.1
        || (selected.margin_right_px() - layout.margin_right_px()).abs() > 0.1
        || (selected.margin_top_px() - layout.margin_top_px()).abs() > 0.1
        || (selected.margin_bottom_px() - layout.margin_bottom_px()).abs() > 0.1
    {
        return None;
    }
    let style = document
        .unknown_styles()
        .iter()
        .find(|s| s.name() == Some(PAGE_LAYOUT_STYLE_PATH))?;
    let summary = summarize_style_stream(style.payload());
    let record = summary.records().first()?;
    let mut fonts = record.subrecords().iter().filter(|s| s.code() == 0x4006);
    let font = fonts.next()?.payload();
    if fonts.next().is_some()
        || font.len() != 26
        || font[..5] != [0, 0, 0xc1, 0, 0]
        || font[7..14] != [0x80, 0, 0, 0x3f, 0, 2, 0x66]
        || font[14..] != [0, 100, 0, 100, 0x80, 0, 0x80, 0, 2, 0x80, 0, 0]
        || (hundredth_millimeters_to_css_px(u32::from(u16::from_be_bytes([font[5], font[6]])))
            - document_default_font_size_px(document)?)
        .abs()
            > 0.01
    {
        return None;
    }
    let mut gaps = record.subrecords().iter().filter(|s| s.code() == 0x400a);
    let gap = gaps.next()?.payload();
    if gaps.next().is_some() || gap != [0xc3, 0, 0, 13, 0, 0, 0x50, 0x40, 1, 0] {
        return None;
    }
    Some(60)
}

impl DocumentCore {
    pub(crate) fn page_layout_for(&self, page: usize) -> PageLayout {
        native_section_layouts(&self.document, self.page_layout)
            .and_then(|layouts| layouts.get(page).copied())
            .unwrap_or(self.page_layout)
    }

    pub fn page_size_px(&self, page_num: u32) -> Result<(f64, f64)> {
        self.page_lines(page_num)?;
        let layout = self.page_layout_for(page_num as usize);
        Ok((f64::from(layout.width_px()), f64::from(layout.height_px())))
    }
}
