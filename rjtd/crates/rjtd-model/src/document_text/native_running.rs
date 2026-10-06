use crate::*;
use rjtd_core::header_stream::{HEADER_PATH, HeaderTextCandidate, parse_header_text_candidates};
use rjtd_core::style_stream::{StyleStreamRecordLayout, parse_document_edit_style_sections};

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

pub(crate) struct NativeRunningText {
    slot: HeaderTextCandidate,
    pub(crate) role: &'static str,
    text: String,
    x: f32,
    baseline: f32,
    anchor: &'static str,
    font_size: f32,
}

/// Plain slots and the controlled global horizontal view profile only.
/// Text, parity and cover suppression are source linked; glyph metrics and
/// the nominal mm100 anchors remain a decoded-false projection.
pub(crate) fn native_running_text(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
    page_number: usize,
) -> Option<Vec<NativeRunningText>> {
    if writing_mode.is_vertical()
        || page_number == 0
        || document
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
        (value > 0 && value <= 10000).then_some(f32::from(value) * 96.0 / 2540.0)
    };
    let header_offset = distance(primary_header)?;
    let footer_offset = distance(primary_footer)?;
    if (distance(secondary_header)? - header_offset).abs() > 0.01
        || (distance(secondary_footer)? - footer_offset).abs() > 0.01
    {
        return None;
    }
    let number = record(0x3106)?;
    if number.len() != 6 || number.get(..4) != Some(&[3, 0, 9, 1]) {
        return None;
    }
    let number_offset = distance(number)?;
    let font_size = document_default_font_size_px(document)?;
    let header_baseline = header_offset + font_size;
    let footer_baseline =
        layout.height_px() - layout.margin_bottom_px() + footer_offset + font_size;
    if header_baseline >= layout.margin_top_px()
        || footer_baseline >= layout.height_px()
        || footer_offset >= layout.margin_bottom_px()
    {
        return None;
    }
    if cover_off && page_number == 1 {
        return Some(Vec::new());
    }
    let even = facing && page_number.is_multiple_of(2);
    let x = if even {
        layout.width_px() - layout.margin_right_px()
    } else {
        layout.margin_left_px()
    };
    let anchor = if even { "end" } else { "start" };
    let mut output = Vec::new();
    for (id, role, baseline) in [
        (if even { 2 } else { 1 }, "header", header_baseline),
        (if even { 6 } else { 5 }, "footer", footer_baseline),
    ] {
        let slot = slots.iter().find(|slot| slot.slot_id() == id)?.clone();
        output.push(NativeRunningText {
            text: slot.text().to_string(),
            slot,
            role,
            x,
            baseline,
            anchor,
            font_size,
        });
    }
    if page_number_enabled {
        output.push(NativeRunningText {
            slot: slots[0].clone(),
            role: "pageNumber",
            text: format!("- {page_number} -"),
            x: layout.width_px() / 2.0,
            baseline: layout.height_px() - number_offset,
            anchor: "middle",
            font_size,
        });
    }
    Some(output)
}

pub(crate) fn push_native_running_svg(
    svg: &mut String,
    items: &[NativeRunningText],
    font_family: &str,
) {
    for item in items {
        svg.push_str(&format!("<text class=\"rjtd-native-running-{}\" data-source=\"/Header+DocumentViewStyles\" data-header-slot-id=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" x=\"{:.3}\" y=\"{:.3}\" text-anchor=\"{}\" font-family=\"{}\" font-size=\"{:.3}\" fill=\"#111111\" xml:space=\"preserve\">{}</text>", item.role, item.slot.slot_id(), item.x, item.baseline, item.anchor, escape_xml(font_family), item.font_size, escape_xml(&item.text)));
    }
}

pub(crate) fn push_native_running_layer_json(
    output: &mut String,
    items: &[NativeRunningText],
    font_family: &str,
) {
    for item in items {
        output.push_str(&format!(",{{\"type\":\"runningTextCandidate\",\"role\":{},\"text\":{},\"x\":{:.3},\"baseline\":{:.3},\"textAnchor\":{},\"fontSize\":{:.3},\"fontFamily\":{},\"sourceStream\":\"/Header\",\"viewStream\":\"/DocumentViewStyles\",\"slotId\":{},\"jtdByteRange\":{{\"start\":{},\"end\":{}}},\"decoded\":false,\"geometryDecoded\":false,\"positionsDecoded\":false}}", json_string(item.role), json_string(&item.text), item.x, item.baseline, json_string(item.anchor), item.font_size, json_string(font_family), item.slot.slot_id(), item.slot.byte_start(), item.slot.byte_end()));
    }
}
