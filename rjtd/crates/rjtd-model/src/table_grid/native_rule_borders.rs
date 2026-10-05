use crate::*;
use rjtd_core::document_text::DocumentTextStyleTypedValue;

/// Nominal first-page two-column rule strokes, not logical cells or printer corner glyphs.
#[derive(Debug, Clone)]
pub(crate) struct NativeRuleBorderSegment {
    pub(crate) points: [f32; 4],
    pub(crate) source_unit: usize,
    pub(crate) direction: &'static str,
    pub(crate) preset: Option<u16>,
    pub(crate) transparency: Option<u8>,
    pub(crate) color: Option<u32>,
    pub(crate) visible: bool,
    pub(crate) stroke: String,
    pub(crate) width: f32,
    pub(crate) dashed: bool,
}

pub(crate) fn native_rule_border_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    writing_mode: WritingMode,
) -> Option<Vec<NativeRuleBorderSegment>> {
    if page_number != 1 || writing_mode.is_vertical() || !layout.has_source_margins() {
        return None;
    }
    let flow = document.document_text_flow()?;
    let rows = flow
        .events()
        .iter()
        .filter(|event| event.record_class() == Some(0x0010))
        .collect::<Vec<_>>();
    if rows.len() < 3 {
        return None;
    }
    let top = rows.first()?;
    let grid_extent = top.raw_words().get(6).copied()?;
    let left = top.raw_words().get(8).copied()?;
    let lengths = [
        top.raw_words().get(12).copied()?,
        top.raw_words().get(16).copied()?,
    ];
    if grid_extent == 0 || lengths.contains(&0) {
        return None;
    }
    let centers = [
        u32::from(left) + 1,
        u32::from(left) + 3 + u32::from(lengths[0]),
        u32::from(left) + 5 + u32::from(lengths[0]) + u32::from(lengths[1]),
    ];
    if centers[2] + 3 != u32::from(grid_extent) {
        return None;
    }
    let intervals = shanai_lan_line_mark_intervals(document);
    let page_mark = document.page_marks().first()?;
    let resolver = document_text_style_resolver(document)?;
    if resolver.truncated() || !resolver.diagnostics().is_empty() {
        return None;
    }
    let font = document_default_font_size_px(document)?;
    let unit_px = layout.body_width_px() / f32::from(grid_extent);
    if !unit_px.is_finite() || unit_px <= 0.0 {
        return None;
    }
    let mut segments = Vec::new();
    let mut previous_record = None;
    let mut previous_pitch = None;
    for (row_index, row) in rows.iter().enumerate() {
        let words = row.raw_words();
        if words.len() != 27 {
            return None;
        }
        let pattern = [words[9], words[13], words[17]];
        let supported_pattern = if row_index == 0 {
            pattern == [0x16, 0x16, 0x12]
        } else if row_index + 1 == rows.len() {
            pattern == [0x15, 0x15, 0x11]
        } else {
            matches!(pattern[0], 0x13 | 0x17)
                && matches!(pattern[1], 0x13 | 0x17)
                && pattern[2] == 0x13
        };
        if !supported_pattern
            || words[3..9] != [0, 0x8f, 15, grid_extent, 0, left]
            || words[9..23]
                != [
                    pattern[0],
                    0,
                    if pattern[0] & 4 != 0 { 0x14 } else { 0 },
                    lengths[0],
                    pattern[1],
                    0,
                    if pattern[1] & 4 != 0 { 0x14 } else { 0 },
                    lengths[1],
                    pattern[2],
                    0,
                    0,
                    1,
                    0xffff,
                    0,
                ]
        {
            return None;
        }
        let matches = intervals
            .iter()
            .filter(|interval| interval.unit_start == row.unit_start())
            .collect::<Vec<_>>();
        let [interval] = matches.as_slice() else {
            return None;
        };
        if row.unit_end() > interval.unit_end
            || rows
                .get(row_index + 1)
                .is_some_and(|next| interval.unit_end != next.unit_start())
        {
            return None;
        }
        if previous_record.is_some_and(|record| interval.record_index != record + 1) {
            return None;
        }
        previous_record = Some(interval.record_index);
        let page = table_grid_page_mark_entry_for_line_mark_record(
            Some(page_mark),
            interval.record_index,
        )?;
        if page.index() != Some(0) || page.line_start() != Some(0) {
            return None;
        }
        let pitch_mm100 = *page_mark
            .entries()
            .get(page.row_index())?
            .u16_fields()
            .get(21)?;
        let pitch = hundredth_millimeters_to_css_px(u32::from(pitch_mm100));
        if !(APP_FONT_SIZE_PX..=APP_LINE_HEIGHT_PX * 1.25).contains(&pitch)
            || previous_pitch.is_some_and(|previous| previous != pitch_mm100)
        {
            return None;
        }
        previous_pitch = Some(pitch_mm100);
        let y = layout.margin_top_px() + interval.record_index as f32 * pitch + font / 2.0;
        if y - pitch / 2.0 < layout.margin_top_px() || y + pitch / 2.0 > layout.height_px() {
            return None;
        }
        for (column, state) in pattern.into_iter().enumerate() {
            let source_unit = row.unit_start() + 9 + column * 4;
            let x = layout.margin_left_px() + centers[column] as f32 * unit_px;
            for (bit, direction, points) in [
                (1, "up", [x, y - pitch / 2.0, x, y]),
                (2, "down", [x, y, x, y + pitch / 2.0]),
                (
                    4,
                    "right",
                    [
                        x,
                        y,
                        layout.margin_left_px()
                            + centers.get(column + 1).copied().unwrap_or(centers[column]) as f32
                                * unit_px,
                        y,
                    ],
                ),
            ] {
                if state & bit != 0 {
                    let property = match bit {
                        1 => 1,
                        2 => 2,
                        _ => 3,
                    };
                    segments.push(native_rule_border_segment(
                        &resolver,
                        source_unit,
                        property,
                        direction,
                        points,
                    )?);
                }
            }
        }
    }
    Some(segments)
}

fn native_rule_border_segment(
    resolver: &DocumentTextStyleResolver,
    source_unit: usize,
    property: u8,
    direction: &'static str,
    points: [f32; 4],
) -> Option<NativeRuleBorderSegment> {
    let value = |id| resolver.uniform_optional_value_in_range(source_unit, source_unit + 1, id);
    let preset = match value(property)? {
        None => None,
        Some(DocumentTextStyleTypedValue::U16(value)) => Some(value),
        _ => return None,
    };
    // Preset paint sizes are bounded renderer approximations, not decoded source units.
    let (width, dashed) = match preset {
        None | Some(0 | 0xffff) => (0.8, false),
        Some(3) => (2.56, false),
        Some(4) => (0.8, true),
        _ => return None,
    };
    let transparency = match value(property + 3)? {
        None => None,
        Some(DocumentTextStyleTypedValue::U8(value @ (0 | 1))) => Some(value),
        _ => return None,
    };
    let color = match value(property + 14)? {
        None => None,
        Some(DocumentTextStyleTypedValue::U32(value)) => Some(value),
        _ => return None,
    };
    let bgr = match color {
        None | Some(0xffff_ffff) => 0,
        Some(value) if value & 0xff00_0000 == 0 => value,
        _ => return None,
    };
    Some(NativeRuleBorderSegment {
        points,
        source_unit,
        direction,
        preset,
        transparency,
        color,
        visible: transparency != Some(1),
        stroke: format!(
            "#{:02x}{:02x}{:02x}",
            bgr & 0xff,
            (bgr >> 8) & 0xff,
            (bgr >> 16) & 0xff
        ),
        width,
        dashed,
    })
}

pub(crate) fn push_native_rule_borders_svg(svg: &mut String, segments: &[NativeRuleBorderSegment]) {
    svg.push_str("<g class=\"rjtd-native-rule-borders\" data-projection-kind=\"nativeRuleBorderProjection\" data-source-backed=\"true\" data-reference-backed=\"false\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-paint-decoded=\"false\">");
    for segment in segments.iter().filter(|segment| segment.visible) {
        let [x0, y0, x1, y1] = segment.points;
        let dash = if segment.dashed {
            " stroke-dasharray=\"3.2 3.2\""
        } else {
            ""
        };
        svg.push_str(&format!("<line data-source-unit=\"{}\" data-direction=\"{}\" x1=\"{x0:.3}\" y1=\"{y0:.3}\" x2=\"{x1:.3}\" y2=\"{y1:.3}\" stroke=\"{}\" stroke-width=\"{:.3}\"{dash}/>", segment.source_unit, segment.direction, segment.stroke, segment.width));
    }
    svg.push_str("</g>");
}

pub(crate) fn native_rule_border_layer_json(segment: &NativeRuleBorderSegment) -> String {
    let [x0, y0, x1, y1] = segment.points;
    format!(
        "{{\"type\":\"line\",\"projectionKind\":\"nativeRuleBorderProjection\",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"paintDecoded\":false,\"sourceUnit\":{},\"direction\":\"{}\",\"rawPreset\":{},\"rawTransparency\":{},\"rawColor\":{},\"visible\":{},\"x1\":{x0:.3},\"y1\":{y0:.3},\"x2\":{x1:.3},\"y2\":{y1:.3},\"strokeColor\":\"{}\",\"strokeWidth\":{:.3},\"strokeDasharray\":{}}}",
        segment.source_unit,
        segment.direction,
        segment
            .preset
            .map_or("null".into(), |value| value.to_string()),
        segment
            .transparency
            .map_or("null".into(), |value| value.to_string()),
        segment
            .color
            .map_or("null".into(), |value| value.to_string()),
        segment.visible,
        segment.stroke,
        segment.width,
        if segment.dashed { "[3.2,3.2]" } else { "[]" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_paint_is_contextual_and_rejects_unknown_or_malformed_values() {
        let resolver = |properties: &[u8]| {
            let mut bytes = vec![0; 32];
            bytes[28..32].copy_from_slice(&1_u32.to_be_bytes());
            bytes.extend([0, b'A']);
            bytes.push(0xfe);
            bytes.extend(properties);
            bytes.extend([0xff, 0]);
            DocumentTextStyleResolver::from_document_text_bytes(&bytes)
        };
        let valid = resolver(&[2, 2, 0, 3, 5, 1, 1, 16, 4, 0, 0, 0, 255]);
        let segment = native_rule_border_segment(&valid, 16, 2, "down", [1., 2., 1., 3.]).unwrap();
        assert!(!segment.visible);
        assert_eq!(segment.stroke, "#ff0000");
        assert_eq!(segment.width, 2.56);
        let json = native_rule_border_layer_json(&segment);
        assert!(json.contains("\"rawColor\":255"));
        assert!(json.contains("\"visible\":false"));
        let mut svg = String::new();
        push_native_rule_borders_svg(&mut svg, &[segment]);
        assert!(!svg.contains("<line"));
        for properties in [
            &[2, 2, 0, 99][..],
            &[2, 1, 3],
            &[5, 1, 2],
            &[5, 1, 0x80],
            &[16, 4, 1, 0, 0, 0],
        ] {
            assert!(
                native_rule_border_segment(&resolver(properties), 16, 2, "down", [0.; 4]).is_none()
            );
        }
    }

    #[test]
    #[ignore = "requires private native border pairs"]
    fn native_rule_borders_preserve_grid_and_apply_directional_paint() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/native-fixtures");
        let mut baseline_points = None;
        for (name, visible, width, color, dashed) in [
            ("base", 30, 0.8, "#000000", false),
            ("none", 0, 0.8, "#000000", false),
            ("thick", 30, 2.56, "#000000", false),
            ("red", 30, 0.8, "#ff0000", false),
            ("dashed", 30, 0.8, "#000000", true),
            ("one-edge", 22, 0.8, "#000000", false),
        ] {
            let document = parse_document(
                &std::fs::read(root.join(format!("table-border-{name}.jtd"))).unwrap(),
            )
            .unwrap();
            let layout =
                page_layout_with_source_margins(&document, page_layout_from_document(&document));
            let segments =
                native_rule_border_projection(&document, layout, 1, WritingMode::Horizontal)
                    .unwrap_or_else(|| panic!("border projection rejected {name}"));
            let points = segments
                .iter()
                .map(|segment| segment.points)
                .collect::<Vec<_>>();
            assert_eq!(segments.len(), 30, "{name}");
            if let Some(baseline) = &baseline_points {
                assert_eq!(&points, baseline, "{name}");
            } else {
                baseline_points = Some(points);
            }
            let painted = segments
                .iter()
                .filter(|segment| segment.visible)
                .collect::<Vec<_>>();
            assert_eq!(painted.len(), visible, "{name}");
            assert!(
                painted.iter().all(|segment| segment.width == width
                    && segment.stroke == color
                    && segment.dashed == dashed),
                "{name}"
            );
            assert!(
                native_rule_border_projection(&document, layout, 2, WritingMode::Horizontal)
                    .is_none()
            );
            assert!(
                native_rule_border_projection(&document, layout, 1, WritingMode::VerticalRl)
                    .is_none()
            );
            let core = DocumentCore::from_document(document);
            let after = core
                .document
                .document_text_flow()
                .unwrap()
                .events()
                .iter()
                .rfind(|event| event.kind() == DocumentTextFlowKind::Text)
                .unwrap();
            let after_top =
                native_rule_body_top_y(&core.document, layout, true, after.source_span()).unwrap();
            assert!(
                segments.iter().all(|segment| after_top > segment.points[3]),
                "{name}"
            );
            let svg = core.render_page_svg(0).unwrap();
            let baseline = after_top + document_default_font_size_px(&core.document).unwrap();
            assert!(svg.contains(&format!("y=\"{baseline:.1}\"")), "{name}");
            assert_eq!(svg.matches("data-direction=").count(), visible, "{name}");
            assert_eq!(svg.matches("<line ").count(), visible, "{name}");
            assert!(!svg.contains("rjtd-column-grid-candidate"), "{name}");
            for text in ["A1", "A2", "B1", "B2"] {
                assert_eq!(svg.matches(&format!(">{text}</text>")).count(), 1, "{name}");
            }
            assert_eq!(
                core.get_page_layer_tree(0)
                    .unwrap()
                    .matches("nativeRuleBorderProjection")
                    .count(),
                30,
                "{name}"
            );
        }
    }
}
