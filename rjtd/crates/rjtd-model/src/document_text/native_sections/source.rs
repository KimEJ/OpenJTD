use crate::{
    Document, DocumentTextFlowEvent, DocumentTextFlowKind, PAGE_LAYOUT_STYLE_RECORD_CODE,
    page_margins_mm100_at, page_size_mm100_from_document_view_styles,
    page_size_mm100_from_page_layout_style, shanai_lan_line_mark_intervals,
};
#[cfg(any(test, feature = "rendering"))]
use crate::{DocumentTextFlow, text_by_utf16_units};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, summarize_style_stream,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeSectionSource {
    pub(super) custom: (u32, u32),
    pub(super) default: (u32, u32),
    pub(super) margins: [u16; 4],
    pub(super) custom_style_pages: Vec<bool>,
}

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

pub(super) fn native_section_source(document: &Document) -> Option<NativeSectionSource> {
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
    let custom = page_size_mm100_from_page_layout_style(style.payload())?;
    let view = document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))?;
    let default = page_size_mm100_from_document_view_styles(view.payload())?;
    let margin_payload = record
        .subrecords()
        .iter()
        .find(|sub| sub.code() == 0x4002)?
        .payload();
    if !matches!(margin_payload.len(), 39 | 40) || margin_payload.get(..3) != Some(&[0xfe, 0x80, 0])
    {
        return None;
    }
    let margins = page_margins_mm100_at(margin_payload, 3)?;
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
        || page_margins_mm100_at(bytes, 2)? != margins
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
        result.push(page == 1);
    }
    Some(NativeSectionSource {
        custom,
        default,
        margins,
        custom_style_pages: result,
    })
}

#[cfg(any(test, feature = "rendering"))]
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

#[cfg(feature = "rendering")]
pub(super) fn native_section_spacing_source(document: &Document) -> Option<(u16, u16)> {
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
    {
        return None;
    }
    let mut gaps = record.subrecords().iter().filter(|s| s.code() == 0x400a);
    let gap = gaps.next()?.payload();
    if gaps.next().is_some() || gap != [0xc3, 0, 0, 13, 0, 0, 0x50, 0x40, 1, 0] {
        return None;
    }
    Some((u16::from_be_bytes([font[5], font[6]]), 60))
}

impl Document {
    /// Bounded apply/reset and physical-source-page association candidate.
    /// General section inheritance and display placement remain unproven.
    pub fn section_source_candidate(&self) -> Option<NativeSectionSource> {
        native_section_source(self)
    }
}
impl NativeSectionSource {
    pub fn custom_size_mm100(&self) -> (u32, u32) {
        self.custom
    }
    pub fn default_size_mm100(&self) -> (u32, u32) {
        self.default
    }
    pub fn margins_mm100(&self) -> [u16; 4] {
        self.margins
    }
    pub fn custom_style_pages(&self) -> &[bool] {
        &self.custom_style_pages
    }
}
