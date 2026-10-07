use crate::Document;
use rjtd_core::header_stream::{HEADER_PATH, HeaderTextCandidate, parse_header_text_candidates};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, StyleStreamRecordLayout,
    parse_document_edit_style_sections, summarize_style_stream,
};

impl Document {
    pub fn header_text_candidates(&self) -> Vec<HeaderTextCandidate> {
        let mut streams = self
            .raw_streams()
            .iter()
            .filter(|stream| stream.name() == HEADER_PATH);
        let Some(stream) = streams.next() else {
            return Vec::new();
        };
        if streams.next().is_some() {
            return Vec::new();
        }
        parse_header_text_candidates(stream.bytes()).unwrap_or_default()
    }
}

pub(super) struct NativeRunningSource {
    pub(super) slots: Vec<HeaderTextCandidate>,
    pub(super) facing: bool,
    pub(super) cover_off: bool,
    pub(super) page_number_enabled: bool,
    pub(super) header_mm100: [u16; 2],
    pub(super) footer_mm100: [u16; 2],
    pub(super) number_mm100: u16,
}

pub(super) fn native_running_source(document: &Document) -> Option<NativeRunningSource> {
    if document
        .unknown_styles()
        .iter()
        .any(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
    {
        return None;
    }
    let slots = document.header_text_candidates();
    let ids = slots
        .iter()
        .map(HeaderTextCandidate::slot_id)
        .collect::<Vec<_>>();
    let facing = match ids.as_slice() {
        [0, 1, 5] => false,
        [0, 1, 2, 5, 6] => true,
        _ => return None,
    };
    if slots[0].text() != "- ? -" {
        return None;
    }
    let mut views = document
        .unknown_styles()
        .iter()
        .filter(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH));
    let view = views.next()?;
    if views.next().is_some() || view.payload().len() > 64 * 1024 {
        return None;
    }
    let bytes = view.payload();
    let summary = summarize_style_stream(bytes);
    if summary.record_layout() != StyleStreamRecordLayout::Sequential
        || summary.records().first()?.offset() != 10
        || bytes.get(..6) != Some(&[0, 1, 0, 2, 0x10, 0])
    {
        return None;
    }
    let record = |code| {
        let mut matches = summary
            .records()
            .iter()
            .filter(|record| record.code() == code);
        let item = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        let from = item.offset().checked_add(4)?;
        bytes.get(from..from.checked_add(item.payload_len())?)
    };
    let cover_off = match record(0x1011)? {
        [0, 0, 0x10, 0, 0] => false,
        [2, 0, 0, 0, 1, 0, 0x10, 0, 0] if !facing => true,
        _ => return None,
    };
    let mode = if facing { 3 } else { 0 };
    if record(0x1007)?
        != if facing || cover_off {
            &[0x84, 7, 0, 0, 0x84, 0x3b, 7, 5, 1, 1, 0]
        } else {
            &[0x84, 7, 0, 0, 0x84, 0x0b, 7, 5, 1, 1, 0]
        }
    {
        return None;
    }
    let last = summary.records().last()?;
    let end = last.offset().checked_add(4 + last.payload_len())?;
    // The sequential section's declared extent locates its extended trailer.
    if usize::try_from(u32::from_be_bytes(bytes.get(6..10)?.try_into().ok()?))
        .ok()?
        .checked_add(6)?
        != end
    {
        return None;
    }
    let trailer = parse_document_edit_style_sections(bytes.get(end..)?)?;
    let [format, settings] = trailer.sections() else {
        return None;
    };
    if trailer.header() != &[0xff, 0xff, 0, 0]
        || !trailer.trailing_bytes().is_empty()
        || format.section_code() != 0x1001
        || format.payload()
            != [
                0x20, 4, 0, 0x10, 0, 0, 0x44, 0xff, 0xfe, 0x80, 0, 0x30, 0x80, 0, 0x80, 0, 2, 0x80,
                0, 0, 0xff, 0xff, 0, 0,
            ]
        || settings.section_code() != 0x1002
        || settings.payload().len() != 2026
        || settings.payload().get(..8) != Some(&[0, 3, 0, 8, 0, 0, 0, 1])
    {
        return None;
    }
    let page_number_enabled = match settings.payload().get(8..12)? {
        [0, 0, 0, 0] => false,
        [0, 0, 0, 2] => true,
        _ => return None,
    };
    for group in [0x31, 0x32, 0x33, 0x36, 0x37] {
        if record(group * 256 + 4)? != [0, 0, 0x40, 0x80, 0, 0x30, 0x80, 0, 0x80, 0, 2, 0x80, 0, 0]
            || record(group * 256 + 5)? != [0, 0, 0, 0, 0, 0, 0x40, 0, 0]
        {
            return None;
        }
        let p = record(group * 256 + 7)?;
        let mut expected = [0; 21];
        expected[1] = 0x3f;
        expected[19] = u8::from(group == 0x31);
        if p != expected {
            return None;
        }
    }
    let align = if facing { 2 } else { 1 };
    let primary_header = record(0x3206)?;
    let primary_footer = record(0x3606)?;
    if primary_header.get(..5) != Some(&[3, mode, 0x0b, align, 0])
        || primary_header.len() != 7
        || primary_footer.get(..6) != Some(&[3, mode, 0x19, align, 0, 100])
        || primary_footer.len() != 8
    {
        return None;
    }
    let secondary_header = record(0x3306)?;
    let secondary_footer = record(0x3706)?;
    if facing {
        if secondary_header.get(..5) != Some(&[3, mode, 0x0b, 3, 0])
            || secondary_header.len() != 7
            || secondary_footer.get(..6) != Some(&[3, mode, 0x19, 3, 0, 100])
            || secondary_footer.len() != 8
        {
            return None;
        }
    } else if secondary_header.get(..4) != Some(&[3, 0, 0x0a, 0])
        || secondary_header.len() != 6
        || secondary_footer.get(..5) != Some(&[3, 0, 0x18, 0, 100])
        || secondary_footer.len() != 7
    {
        return None;
    }
    let distance = |payload: &[u8]| {
        let n = payload.len();
        let value = u16::from_be_bytes([payload[n - 2], payload[n - 1]]);
        (value > 0 && value <= 10000).then_some(value)
    };
    let header_mm100 = [distance(primary_header)?, distance(secondary_header)?];
    let footer_mm100 = [distance(primary_footer)?, distance(secondary_footer)?];
    let number = record(0x3106)?;
    if number.len() != 6 || number.get(..4) != Some(&[3, 0, 9, 1]) {
        return None;
    }
    let number_mm100 = distance(number)?;
    Some(NativeRunningSource {
        slots,
        facing,
        cover_off,
        page_number_enabled,
        header_mm100,
        footer_mm100,
        number_mm100,
    })
}
