use super::source::native_running_source;
use crate::*;
use rjtd_core::header_stream::HeaderTextCandidate;

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
    if writing_mode.is_vertical() || page_number == 0 {
        return None;
    }
    let source = native_running_source(document)?;
    let slots = source.slots;
    let facing = source.facing;
    let cover_off = source.cover_off;
    let page_number_enabled = source.page_number_enabled;
    // Preserve the projection's existing arithmetic order and admission tolerance.
    let distance = |value| f32::from(value) * 96.0 / 2540.0;
    let header_offset = distance(source.header_mm100[0]);
    let footer_offset = distance(source.footer_mm100[0]);
    if (distance(source.header_mm100[1]) - header_offset).abs() > 0.01
        || (distance(source.footer_mm100[1]) - footer_offset).abs() > 0.01
    {
        return None;
    }
    let number_offset = distance(source.number_mm100);
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
