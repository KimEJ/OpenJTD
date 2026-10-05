use super::*;
use crate::*;

/// Source-backed text placement for a narrow, verified family of horizontal
/// `DocumentText` control tables. This deliberately covers only a complete
/// first-page grid; more complex tables remain diagnostic candidates.
#[derive(Debug, Clone)]
pub(crate) struct NativeControlTableTextProjection {
    pub(crate) candidate_index: usize,
    pub(crate) slots: Vec<NativeControlTableTextSlot>,
    pub(crate) border: Option<NativeControlTableBorderProjection>,
}

#[derive(Debug, Clone)]
pub(crate) struct NativeControlTableBorderProjection {
    pub(crate) vertical_x: Vec<f32>,
    pub(crate) horizontal_y: Vec<f32>,
}

#[derive(Debug, Clone)]
pub(crate) struct NativeControlTableTextSlot {
    pub(crate) candidate_index: usize,
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) baseline_y: f32,
    pub(crate) font_size: DocumentTextFontSize,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) row_index: usize,
    pub(crate) column_index: usize,
    pub(crate) header_offset_units: u16,
    pub(crate) line_mark_record_index: usize,
    pub(crate) page_mark_pitch_mm100: u16,
}

/// Source-backed first-page horizontal text placement. Interstitial runs and
/// physical ruled spans share slots, but retain their distinct admission rules.
#[derive(Debug, Clone)]
pub(crate) struct NativeControlFlowTextProjection {
    pub(crate) projection_kind: &'static str,
    pub(crate) slots: Vec<NativeControlFlowTextSlot>,
}

#[derive(Debug, Clone)]
pub(crate) struct NativeControlFlowTextSlot {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) text_anchor: &'static str,
    pub(crate) baseline_y: f32,
    pub(crate) font_size: DocumentTextFontSize,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) line_mark_record_index: usize,
    pub(crate) page_mark_pitch_mm100: u16,
    pub(crate) leading_ascii_space_count: usize,
    pub(crate) preceding_header_offset_units: u16,
    pub(crate) preceding_header_extent_units: u16,
    pub(crate) raw_span_flags: [u16; 2],
    pub(crate) text_length_px: Option<f32>,
    pub(crate) word_justification_width_px: Option<f32>,
}

#[derive(Debug, Clone, Copy)]
struct NativeControlTableRowHeader {
    start_unit: usize,
    grid_extent: u16,
    left_edge_units: u16,
}

pub(crate) fn native_control_table_text_projections(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    writing_mode: WritingMode,
) -> Vec<NativeControlTableTextProjection> {
    if page_number != 1 || writing_mode.is_vertical() || !layout.has_source_margins() {
        return Vec::new();
    }
    document
        .table_candidates()
        .iter()
        .filter_map(|candidate| native_control_table_text_projection(document, layout, candidate))
        .collect()
}

pub(crate) fn native_control_table_text_projection_contains(
    projections: &[NativeControlTableTextProjection],
    span: &TextSourceSpan,
) -> bool {
    projections
        .iter()
        .flat_map(|projection| &projection.slots)
        .any(|slot| {
            slot.source_span.unit_start() < span.unit_end()
                && span.unit_start() < slot.source_span.unit_end()
        })
}

pub(crate) fn native_control_flow_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    writing_mode: WritingMode,
    table_projections: &[NativeControlTableTextProjection],
) -> Option<NativeControlFlowTextProjection> {
    if page_number != 1
        || writing_mode.is_vertical()
        || !layout.has_source_margins()
        || table_projections.len() != 2
    {
        return None;
    }

    let mut table_ranges = table_projections
        .iter()
        .map(|projection| {
            let start = projection
                .slots
                .iter()
                .map(|slot| slot.source_span.unit_start())
                .min()?;
            let end = projection
                .slots
                .iter()
                .map(|slot| slot.source_span.unit_end())
                .max()?;
            Some((start, end))
        })
        .collect::<Option<Vec<_>>>()?;
    table_ranges.sort_unstable();
    let [(first_start, first_end), (second_start, second_end)] = table_ranges.as_slice() else {
        return None;
    };
    if first_start >= first_end || first_end >= second_start || second_start >= second_end {
        return None;
    }

    let bytes = document_text_raw_stream(document)?;
    let text_map = map_document_text(bytes);
    let entries = text_map
        .entries()
        .iter()
        .filter(|entry| {
            entry.kind() == DocumentTextMapKind::TextRun
                && *first_end <= entry.unit_start()
                && entry.unit_end() <= *second_start
        })
        .collect::<Vec<_>>();
    let [entry] = entries.as_slice() else {
        return None;
    };
    let header = native_flow_preceding_header(bytes, *first_end * 2, entry.byte_start())?;

    let mut text_lines = native_flow_source_lines(entry)?;
    if text_lines.len() != 2 {
        return None;
    }
    let leading_space_count = text_lines.first()?.leading_ascii_space_count;
    if leading_space_count == 0
        || text_lines
            .iter()
            .any(|line| line.leading_ascii_space_count != leading_space_count)
    {
        return None;
    }

    let intervals = shanai_lan_line_mark_intervals(document);
    let page_mark = document.page_marks().first()?;
    let mut page_entry = None;
    for line in &mut text_lines {
        let interval = intervals
            .iter()
            .filter(|interval| {
                interval.unit_start <= line.source_span.unit_start()
                    && line.source_span.unit_end() <= interval.unit_end
            })
            .collect::<Vec<_>>();
        let [interval] = interval.as_slice() else {
            return None;
        };
        let entry = table_grid_page_mark_entry_for_line_mark_record(
            Some(page_mark),
            interval.record_index,
        )?;
        if entry.line_start()? != 0 || entry.index()? != 0 {
            return None;
        }
        if let Some(previous) = page_entry
            && previous != entry.row_index()
        {
            return None;
        }
        page_entry = Some(entry.row_index());
        line.line_mark_record_index = interval.record_index;
    }
    let pitch_mm100 = page_mark
        .entries()
        .get(page_entry?)?
        .u16_fields()
        .get(21)
        .copied()?;
    let pitch = hundredth_millimeters_to_css_px(u32::from(pitch_mm100));
    if !(APP_FONT_SIZE_PX..=APP_LINE_HEIGHT_PX * 1.25).contains(&pitch) {
        return None;
    }

    let default_font = document_default_font_size_px(document)?;
    let resolver = document_text_style_resolver(document)?;
    let slots = text_lines
        .into_iter()
        .map(|line| {
            let font_size =
                document_text_font_size(&resolver, &line.source_span, Some(default_font))?;
            let baseline_y =
                layout.margin_top_px() + line.line_mark_record_index as f32 * pitch + font_size.px;
            (baseline_y <= layout.height_px()).then_some(NativeControlFlowTextSlot {
                text: line.text,
                x: layout.margin_left_px(),
                text_anchor: "start",
                baseline_y,
                font_size,
                source_span: line.source_span,
                line_mark_record_index: line.line_mark_record_index,
                page_mark_pitch_mm100: pitch_mm100,
                leading_ascii_space_count: line.leading_ascii_space_count,
                preceding_header_offset_units: header.offset_units,
                preceding_header_extent_units: header.extent_units,
                raw_span_flags: [header.raw_words[6], header.raw_words[7]],
                text_length_px: None,
                word_justification_width_px: None,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(NativeControlFlowTextProjection {
        projection_kind: "nativeControlFlowTextProjection",
        slots,
    })
}

/// Place physical text runs from source events without synthesizing logical
/// rows or interpreting the opaque span continuation flags.
pub(crate) fn native_rule_flow_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    writing_mode: WritingMode,
    tables: &[NativeControlTableTextProjection],
    interstitial: Option<&NativeControlFlowTextProjection>,
) -> Option<NativeControlFlowTextProjection> {
    if page_number != 1 || writing_mode.is_vertical() || !layout.has_source_margins() {
        return None;
    }
    let flow = document.document_text_flow()?;
    let intervals = shanai_lan_line_mark_intervals(document);
    let page_mark = document.page_marks().first()?;
    let resolver = document_text_style_resolver(document)?;
    let default_font = document_default_font_size_px(document)?;
    let english_justification = document.english_justification_candidate();
    let mut parent = None;
    let mut declaration = None;
    let mut slots = Vec::new();
    for event in flow.events() {
        match event.kind() {
            DocumentTextFlowKind::Record => {
                let words = event.raw_words();
                match event.record_class() {
                    Some(0x0010) => {
                        parent = (words.len() >= 13
                            && words[3] == 0
                            && words[4] == 0x008f
                            && words[6] > 0)
                            .then_some((event.unit_start(), words[6]));
                        declaration = None;
                    }
                    Some(0x0030) => {
                        declaration = (words.len() == 12 && words[4] < words[5]).then_some(event);
                    }
                    _ => {
                        parent = None;
                        declaration = None;
                    }
                }
            }
            DocumentTextFlowKind::Control => {
                parent = None;
                declaration = None;
            }
            DocumentTextFlowKind::Text => {
                let (Some((parent_start, grid_extent)), Some(header)) = (parent, declaration)
                else {
                    continue;
                };
                declaration = None;
                if header.unit_end() != event.unit_start() || !event.text().is_ascii() {
                    continue;
                }
                let words = header.raw_words();
                if words[5] > grid_extent {
                    continue;
                }
                let parts = source_text_parts(event.text(), Some(event.source_span()));
                let parts = parts
                    .iter()
                    .filter(|part| !part.text.is_empty())
                    .collect::<Vec<_>>();
                let [part] = parts.as_slice() else {
                    continue;
                };
                if !part.text.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
                    continue;
                }
                let span = part.source_span.as_ref()?;
                if native_control_table_text_projection_contains(tables, span)
                    || native_control_flow_text_projection_contains(interstitial, span)
                {
                    continue;
                }
                let containing = intervals
                    .iter()
                    .filter(|interval| {
                        interval.unit_start == parent_start
                            && interval.unit_start <= span.unit_start()
                            && span.unit_end() <= interval.unit_end
                    })
                    .collect::<Vec<_>>();
                let [interval] = containing.as_slice() else {
                    continue;
                };
                let Some(page) = table_grid_page_mark_entry_for_line_mark_record(
                    Some(page_mark),
                    interval.record_index,
                ) else {
                    continue;
                };
                if page.line_start() != Some(0) || page.index() != Some(0) {
                    continue;
                }
                let Some(pitch_mm100) = page_mark
                    .entries()
                    .get(page.row_index())
                    .and_then(|entry| entry.u16_fields().get(21))
                    .copied()
                else {
                    continue;
                };
                let pitch = hundredth_millimeters_to_css_px(u32::from(pitch_mm100));
                if !(APP_FONT_SIZE_PX..=APP_LINE_HEIGHT_PX * 1.25).contains(&pitch) {
                    continue;
                }
                let Some(font_size) = document_text_font_size(&resolver, span, Some(default_font))
                else {
                    continue;
                };
                let leading = part.text.chars().take_while(|c| *c == ' ').count();
                let contextual_spacing = words[6] == 0 && words[7] == 2;
                let word_justification = if contextual_spacing {
                    english_justification?
                } else {
                    false
                };
                let text = part.text.trim_start_matches(' ');
                let text = if word_justification {
                    text.trim_end_matches(' ')
                } else {
                    text
                };
                if text.is_empty() {
                    continue;
                }
                let unit_px = layout.body_width_px() / f32::from(grid_extent);
                let text_length_px = match words[6] {
                    0 => None,
                    1 | 2 if leading == 0 && !text.ends_with(' ') && matches!(words[7], 0 | 2) => {
                        None
                    }
                    3 if leading == 0 && !text.ends_with(' ') && text.len() > 1 => {
                        Some(f32::from(words[5] - words[4]) * unit_px)
                    }
                    0x00ff if words[7] == 0 => None,
                    _ => return None,
                };
                let left = layout.margin_left_px() + f32::from(words[4]) * unit_px;
                let right = layout.margin_left_px() + f32::from(words[5]) * unit_px;
                let (x, text_anchor) = match words[6] {
                    1 => ((left + right) / 2.0, "middle"),
                    2 => (right, "end"),
                    _ => (left + leading as f32 * 2.0 * unit_px, "start"),
                };
                let word_justification_width_px = if word_justification {
                    // Repeated padding and single-word fallback tracking are not decoded.
                    if !text.contains(' ') || text.contains("  ") {
                        return None;
                    }
                    let width = (f32::from(words[5] - words[4]) - leading as f32 * 2.0) * unit_px;
                    if !width.is_finite() || width <= 0.0 {
                        return None;
                    }
                    Some(width)
                } else {
                    None
                };
                let baseline_y =
                    layout.margin_top_px() + interval.record_index as f32 * pitch + font_size.px;
                if !x.is_finite()
                    || !baseline_y.is_finite()
                    || x > layout.width_px()
                    || baseline_y > layout.height_px()
                {
                    continue;
                }
                slots.push(NativeControlFlowTextSlot {
                    text: text.to_string(),
                    x,
                    text_anchor,
                    baseline_y,
                    font_size,
                    source_span: span.subspan_by_units(leading, leading + text.len()),
                    line_mark_record_index: interval.record_index,
                    page_mark_pitch_mm100: pitch_mm100,
                    leading_ascii_space_count: leading,
                    preceding_header_offset_units: words[4],
                    preceding_header_extent_units: words[5],
                    raw_span_flags: [words[6], words[7]],
                    text_length_px,
                    word_justification_width_px,
                });
            }
            _ => {
                declaration = None;
            }
        }
    }
    (!slots.is_empty()).then_some(NativeControlFlowTextProjection {
        projection_kind: "nativeRuleFlowTextProjection",
        slots,
    })
}

pub(crate) fn native_control_flow_text_projection_contains(
    projection: Option<&NativeControlFlowTextProjection>,
    span: &TextSourceSpan,
) -> bool {
    projection.is_some_and(|projection| {
        projection.slots.iter().any(|slot| {
            slot.source_span.unit_start() < span.unit_end()
                && span.unit_start() < slot.source_span.unit_end()
        })
    })
}

pub(crate) fn native_control_flow_text_projection_overlaps_candidate(
    projection: Option<&NativeControlFlowTextProjection>,
    candidate: &TableCandidate,
) -> bool {
    projection.is_some_and(|projection| {
        projection
            .slots
            .iter()
            .any(|slot| table_candidate_overlaps_source_span(candidate, &slot.source_span))
    })
}

pub(crate) fn push_native_control_table_text_svg(
    svg: &mut String,
    projections: &[NativeControlTableTextProjection],
    font_family: &str,
) {
    for projection in projections {
        if let Some(border) = &projection.border {
            svg.push_str("<g class=\"rjtd-native-control-table-border\" data-projection-kind=\"nativeControlTableBorderProjection\" data-source-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"false\">");
            for x in &border.vertical_x {
                let (Some(y0), Some(y1)) =
                    (border.horizontal_y.first(), border.horizontal_y.last())
                else {
                    continue;
                };
                svg.push_str(&format!("<line x1=\"{x:.1}\" y1=\"{y0:.1}\" x2=\"{x:.1}\" y2=\"{y1:.1}\" stroke=\"#000000\" stroke-width=\"0.8\"/>"));
            }
            for y in &border.horizontal_y {
                let (Some(x0), Some(x1)) = (border.vertical_x.first(), border.vertical_x.last())
                else {
                    continue;
                };
                svg.push_str(&format!("<line x1=\"{x0:.1}\" y1=\"{y:.1}\" x2=\"{x1:.1}\" y2=\"{y:.1}\" stroke=\"#000000\" stroke-width=\"0.8\"/>"));
            }
            svg.push_str("</g>");
        }
        svg.push_str(&format!(
            "<g class=\"rjtd-native-control-table-text\" data-table-candidate-index=\"{}\" data-projection-kind=\"nativeControlTableTextProjection\" data-source-backed=\"true\" data-reference-backed=\"false\" data-decoded=\"false\" data-geometry-decoded=\"false\">",
            projection.candidate_index
        ));
        for slot in &projection.slots {
            svg.push_str(&format!(
                "<g data-row=\"{}\" data-column=\"{}\" data-header-offset-units=\"{}\" data-line-mark-record-index=\"{}\" data-page-mark-pitch-mm100=\"{}\" data-font-size-basis=\"{}\">",
                slot.row_index,
                slot.column_index,
                slot.header_offset_units,
                slot.line_mark_record_index,
                slot.page_mark_pitch_mm100,
                slot.font_size.basis,
            ));
            push_svg_text_run(
                svg,
                "rjtd-text rjtd-native-control-table-cell",
                slot.x,
                slot.baseline_y,
                font_family,
                slot.font_size.px,
                fallback_text_fill_color(),
                &slot.text,
                None,
            );
            svg.push_str("</g>");
        }
        svg.push_str("</g>");
    }
}

pub(crate) fn push_native_control_flow_text_svg(
    svg: &mut String,
    projection: Option<&NativeControlFlowTextProjection>,
    font_family: &str,
    measured_widths: &BTreeMap<usize, f32>,
) {
    let Some(projection) = projection else {
        return;
    };
    let class = if projection.projection_kind == "nativeRuleFlowTextProjection" {
        "rjtd-native-rule-flow-text"
    } else {
        "rjtd-native-control-flow-text"
    };
    svg.push_str(&format!("<g class=\"{class}\" data-projection-kind=\"{}\" data-source-backed=\"true\" data-reference-backed=\"false\" data-decoded=\"false\" data-geometry-decoded=\"false\">", projection.projection_kind));
    for slot in &projection.slots {
        let text_anchor = slot.text_anchor;
        svg.push_str(&format!(
            "<g text-anchor=\"{text_anchor}\" data-line-mark-record-index=\"{}\" data-page-mark-pitch-mm100=\"{}\" data-leading-ascii-space-count=\"{}\" data-preceding-header-offset-units=\"{}\" data-preceding-header-extent-units=\"{}\" data-font-size-basis=\"{}\" data-span-raw-word6=\"{}\" data-span-raw-word7=\"{}\">",
            slot.line_mark_record_index,
            slot.page_mark_pitch_mm100,
            slot.leading_ascii_space_count,
            slot.preceding_header_offset_units,
            slot.preceding_header_extent_units,
            slot.font_size.basis,
            slot.raw_span_flags[0], slot.raw_span_flags[1],
        ));
        if let Some(width) = slot.text_length_px {
            svg.push_str(&format!("<text class=\"rjtd-text rjtd-native-control-flow-line\" x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\" fill=\"{}\" letter-spacing=\"0\" xml:space=\"preserve\" textLength=\"{width:.3}\" lengthAdjust=\"spacing\">{}</text>", slot.x, slot.baseline_y, escape_xml(font_family), slot.font_size.px, fallback_text_fill_color(), escape_xml(&svg_visual_text(&slot.text))));
        } else if let Some(width) = slot.word_justification_width_px {
            let unit = slot.source_span.unit_start();
            let spacing = measured_widths
                .get(&unit)
                .and_then(|natural| english_word_spacing_px(width, *natural, &slot.text));
            let spacing_attr = spacing
                .map(|value| format!(" word-spacing=\"{value:.6}\""))
                .unwrap_or_default();
            svg.push_str(&format!("<text id=\"rjtd-word-spacing-{unit}\" class=\"rjtd-text rjtd-native-control-flow-line\" x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\" fill=\"{}\" letter-spacing=\"0\" xml:space=\"preserve\" data-source-unit-start=\"{unit}\" data-word-justification-width=\"{width:.6}\" data-word-spacing-resolved=\"{}\"{spacing_attr}>{}</text>", slot.x, slot.baseline_y, escape_xml(font_family), slot.font_size.px, fallback_text_fill_color(), spacing.is_some(), escape_xml(&slot.text)));
        } else {
            svg.push_str(&format!("<text class=\"rjtd-text rjtd-native-control-flow-line\" x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\" fill=\"{}\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>", slot.x, slot.baseline_y, escape_xml(font_family), slot.font_size.px, fallback_text_fill_color(), escape_xml(&svg_visual_text(&slot.text))));
        }
        svg.push_str("</g>");
    }
    svg.push_str("</g>");
}

#[derive(Debug)]
struct NativeControlFlowSourceLine {
    text: String,
    source_span: TextSourceSpan,
    leading_ascii_space_count: usize,
    line_mark_record_index: usize,
}

fn english_word_spacing_px(target: f32, natural: f32, text: &str) -> Option<f32> {
    let spaces = text.bytes().filter(|byte| *byte == b' ').count();
    if !target.is_finite()
        || !natural.is_finite()
        || natural <= 0.0
        || target < natural
        || spaces == 0
    {
        return None;
    }
    Some((target - natural) / spaces as f32)
}

/// Exact first/last text-run line positions around an admitted ruled flow only.
pub(crate) fn native_rule_body_top_y(
    document: &Document,
    layout: PageLayout,
    admitted_rule_placement: bool,
    span: &TextSourceSpan,
) -> Option<f32> {
    if !admitted_rule_placement {
        return None;
    }
    let flow = document.document_text_flow()?;
    let mut text = flow
        .events()
        .iter()
        .filter(|event| event.kind() == DocumentTextFlowKind::Text);
    let first = text.clone().next()?;
    let last = text.next_back()?;
    let event = if span.unit_start() == first.unit_start() {
        first
    } else if span.unit_start() == last.unit_start() {
        last
    } else {
        return None;
    };
    if !event.text().is_ascii() {
        return None;
    }
    let intervals = shanai_lan_line_mark_intervals(document);
    let hits = intervals
        .iter()
        .filter(|interval| {
            interval.unit_start == span.unit_start() && span.unit_end() <= interval.unit_end
        })
        .collect::<Vec<_>>();
    let [interval] = hits.as_slice() else {
        return None;
    };
    let page_mark = document.page_marks().first()?;
    let page =
        table_grid_page_mark_entry_for_line_mark_record(Some(page_mark), interval.record_index)?;
    if page.index() != Some(0) || page.line_start() != Some(0) {
        return None;
    }
    let pitch = *page_mark
        .entries()
        .get(page.row_index())?
        .u16_fields()
        .get(21)?;
    let pitch = hundredth_millimeters_to_css_px(u32::from(pitch));
    if !(APP_FONT_SIZE_PX..=APP_LINE_HEIGHT_PX * 1.25).contains(&pitch) {
        return None;
    }
    let top = layout.margin_top_px() + interval.record_index as f32 * pitch;
    (top.is_finite() && top < layout.height_px()).then_some(top)
}

fn native_flow_preceding_header(
    bytes: &[u8],
    range_start: usize,
    entry_byte_start: usize,
) -> Option<ShanaiLanLineHeader> {
    let start = range_start.min(entry_byte_start) & !1;
    let headers = (start..entry_byte_start)
        .step_by(2)
        .filter_map(|offset| shanai_lan_line_header_at(bytes, offset))
        .filter(|header| header.end <= entry_byte_start)
        .filter(|header| header.offset_units == 0 && header.extent_units != 0)
        .collect::<Vec<_>>();
    let [header] = headers.as_slice() else {
        return None;
    };
    Some(*header)
}

fn native_flow_source_lines(
    entry: &DocumentTextMapEntry,
) -> Option<Vec<NativeControlFlowSourceLine>> {
    if entry.text().contains('\r') || !entry.text().is_ascii() {
        return None;
    }
    let mut offset_units = entry.unit_start();
    let mut lines = Vec::new();
    for chunk in entry.text().split_inclusive('\n') {
        let text = chunk.strip_suffix('\n').unwrap_or(chunk);
        let chunk_units = chunk.encode_utf16().count();
        let text_units = text.encode_utf16().count();
        if !text.is_empty() {
            if !text
                .chars()
                .all(|character| character.is_ascii_graphic() || character == ' ')
            {
                return None;
            }
            let leading_ascii_space_count = text
                .chars()
                .take_while(|character| *character == ' ')
                .count();
            if leading_ascii_space_count == text.chars().count() {
                return None;
            }
            lines.push(NativeControlFlowSourceLine {
                text: text.to_string(),
                source_span: TextSourceSpan::new(
                    offset_units * 2,
                    (offset_units + text_units) * 2,
                    offset_units,
                    offset_units + text_units,
                ),
                leading_ascii_space_count,
                line_mark_record_index: 0,
            });
        }
        offset_units = offset_units.checked_add(chunk_units)?;
    }
    (offset_units == entry.unit_end()).then_some(lines)
}

fn native_control_table_text_projection(
    document: &Document,
    layout: PageLayout,
    candidate: &TableCandidate,
) -> Option<NativeControlTableTextProjection> {
    if candidate.basis() != TextCountRangeOverlapBasis::Unit
        || !candidate.is_document_text_control_run_candidate()
    {
        return None;
    }
    let grid = candidate.column_segment_grid_candidate()?;
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    if rows.len() != grid.row_count() || rows.len() != candidate.intervals().len() {
        return None;
    }
    let bytes = document_text_raw_stream(document)?;
    let parents = rows
        .iter()
        .map(|row| native_parent_row_header(bytes, row.source_start))
        .collect::<Option<Vec<_>>>()?;
    let grid_extent = parents.first()?.grid_extent;
    if grid_extent == 0
        || parents.iter().any(|parent| {
            parent.grid_extent != grid_extent
                || parent.left_edge_units != parents[0].left_edge_units
        })
    {
        return None;
    }
    let resolved = table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows);
    if resolved.len() != rows.len() {
        return None;
    }
    let page_mark = document.page_marks().first()?;
    let mut page_entry = None;
    for ((row, parent), resolved) in rows.iter().zip(&parents).zip(&resolved) {
        let interval = resolved.interval;
        if interval.unit_start != parent.start_unit || interval.unit_end != row.source_end {
            return None;
        }
        let entry = table_grid_page_mark_entry_for_line_mark_record(
            Some(page_mark),
            interval.record_index,
        )?;
        if entry.line_start()? != 0 || entry.index()? != 0 {
            return None;
        }
        if let Some(previous) = page_entry
            && previous != entry.row_index()
        {
            return None;
        }
        page_entry = Some(entry.row_index());
    }
    let pitch_mm100 = page_mark
        .entries()
        .get(page_entry?)?
        .u16_fields()
        .get(21)
        .copied()?;
    let pitch = hundredth_millimeters_to_css_px(u32::from(pitch_mm100));
    if !(APP_FONT_SIZE_PX..=APP_LINE_HEIGHT_PX * 1.25).contains(&pitch) {
        return None;
    }
    let unit_px = layout.body_width_px() / f32::from(grid_extent);
    if !unit_px.is_finite() || unit_px <= 0.0 {
        return None;
    }
    let default_font = document_default_font_size_px(document)?;
    let resolver = document_text_style_resolver(document)?;
    let document_text_map = document_text_raw_stream(document).map(map_document_text)?;
    let fallback_flow;
    let flow = if let Some(flow) = document.document_text_flow() {
        flow
    } else {
        let bytes = document_text_raw_stream(document)?;
        fallback_flow = DocumentTextFlow::from_map("/DocumentText", bytes, &document_text_map);
        &fallback_flow
    };
    let sparse_rows = sparse_document_text_control_table_rows(flow);
    let first_headers = native_matched_cell_headers(rows.first()?, candidate.intervals().first()?)?;
    let first_offsets = first_headers
        .iter()
        .map(|header| header.offset_units)
        .collect::<Vec<_>>();
    let first_ends = first_headers
        .iter()
        .map(|header| header.extent_units)
        .collect::<Vec<_>>();
    if first_offsets.len() != grid.column_count()
        || first_offsets.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return None;
    }

    let mut slots = Vec::new();
    for (((row_index, interval), row), resolved) in candidate
        .intervals()
        .iter()
        .enumerate()
        .zip(&rows)
        .zip(&resolved)
    {
        let headers = native_matched_cell_headers(row, interval)?;
        if row.expected_cell_count != grid.column_count()
            || headers.len() != grid.column_count()
            || headers
                .iter()
                .any(|header| header.extent_units > grid_extent)
            || headers
                .iter()
                .map(|header| header.offset_units)
                .collect::<Vec<_>>()
                != first_offsets
            || headers
                .iter()
                .map(|header| header.extent_units)
                .collect::<Vec<_>>()
                != first_ends
            || interval.column_segments().len() != grid.column_count()
        {
            return None;
        }
        for (column_index, (segment, header)) in
            interval.column_segments().iter().zip(headers).enumerate()
        {
            let (start, end) = (segment.source_start()?, segment.source_end()?);
            if header.end / 2 != start || end <= start || segment.text().contains(['\n', '\r']) {
                return None;
            }
            let span = TextSourceSpan::new(start * 2, end * 2, start, end);
            let font_size = document_text_font_size(&resolver, &span, Some(default_font))?;
            let raw_text =
                table_grid_segment_source_raw_text(Some(&document_text_map), candidate, segment)
                    .unwrap_or_else(|| segment.text().to_string());
            let leading_spaces = raw_text
                .chars()
                .take_while(|character| *character == ' ')
                .count();
            if raw_text.trim_matches(' ') != segment.text() {
                return None;
            }
            let x = layout.margin_left_px()
                + (f32::from(header.offset_units) + leading_spaces as f32 * 2.0) * unit_px;
            let baseline_y = layout.margin_top_px()
                + resolved.interval.record_index as f32 * pitch
                + font_size.px;
            if !x.is_finite()
                || !baseline_y.is_finite()
                || x > layout.width_px()
                || baseline_y > layout.height_px()
            {
                return None;
            }
            slots.push(NativeControlTableTextSlot {
                candidate_index: candidate.index(),
                text: segment.text().to_string(),
                x,
                baseline_y,
                font_size,
                source_span: span,
                row_index,
                column_index,
                header_offset_units: header.offset_units,
                line_mark_record_index: resolved.interval.record_index,
                page_mark_pitch_mm100: pitch_mm100,
            });
        }
    }
    let border = native_control_table_border_projection(
        layout,
        grid_extent,
        parents[0].left_edge_units,
        &first_ends,
        &slots,
        native_control_table_trailing_empty_row_record(
            bytes,
            resolved.last()?.interval.unit_end,
            grid_extent,
            parents[0].left_edge_units,
            &sparse_rows,
            &shanai_lan_line_mark_intervals(document),
        ),
    );
    Some(NativeControlTableTextProjection {
        candidate_index: candidate.index(),
        slots,
        border,
    })
}

fn native_control_table_border_projection(
    layout: PageLayout,
    grid_extent: u16,
    left_edge_units: u16,
    header_ends: &[u16],
    slots: &[NativeControlTableTextSlot],
    trailing_empty_row_record: Option<usize>,
) -> Option<NativeControlTableBorderProjection> {
    let first = slots.first()?;
    if slots
        .iter()
        .any(|slot| (slot.font_size.px - first.font_size.px).abs() > f32::EPSILON)
    {
        return None;
    }
    let mut rows = slots
        .iter()
        .filter(|slot| slot.column_index == 0)
        .collect::<Vec<_>>();
    rows.sort_by_key(|slot| slot.row_index);
    if rows.is_empty()
        || (rows.len() == 1 && trailing_empty_row_record.is_none())
        || rows
            .windows(2)
            .any(|pair| pair[0].line_mark_record_index >= pair[1].line_mark_record_index)
    {
        return None;
    }
    let mut records = Vec::with_capacity(rows.len() + 1);
    records.push(rows.first()?.line_mark_record_index.checked_sub(1)?);
    records.extend(
        rows.windows(2)
            .map(|pair| pair[1].line_mark_record_index - 1),
    );
    records.push(
        trailing_empty_row_record.unwrap_or(rows.last()?.line_mark_record_index.checked_add(1)?),
    );
    let unit_px = layout.body_width_px() / f32::from(grid_extent);
    let vertical_x = std::iter::once(left_edge_units.checked_add(1)?)
        .chain(
            header_ends
                .iter()
                .map(|end| end.checked_add(1))
                .collect::<Option<Vec<_>>>()?,
        )
        .map(|unit| layout.margin_left_px() + f32::from(unit) * unit_px)
        .collect::<Vec<_>>();
    let pitch = hundredth_millimeters_to_css_px(u32::from(first.page_mark_pitch_mm100));
    let horizontal_y = records
        .into_iter()
        .map(|record| layout.margin_top_px() + record as f32 * pitch + first.font_size.px / 2.0)
        .collect::<Vec<_>>();
    Some(NativeControlTableBorderProjection {
        vertical_x,
        horizontal_y,
    })
}

fn native_control_table_trailing_empty_row_record(
    bytes: &[u8],
    start_unit: usize,
    grid_extent: u16,
    left_edge_units: u16,
    rows: &[DocumentTextControlTableRow],
    intervals: &[ShanaiLanLineMarkInterval],
) -> Option<usize> {
    let start = rows.iter().position(|row| row.source_start == start_unit)?;
    let mut final_record = None;
    let mut next_unit = start_unit;
    for row in rows.iter().skip(start).take(64) {
        if row.source_start != next_unit || row.cells.iter().any(|cell| !cell.text.is_empty()) {
            break;
        }
        let Some(parent) = native_parent_row_header(bytes, row.source_start) else {
            break;
        };
        if parent.start_unit != row.source_start
            || parent.grid_extent != grid_extent
            || parent.left_edge_units != left_edge_units
        {
            break;
        }
        let Some(interval) = intervals.iter().find(|interval| {
            interval.unit_start == row.source_start && interval.unit_end == row.source_end
        }) else {
            break;
        };
        final_record = Some(interval.record_index);
        next_unit = row.source_end;
    }
    final_record
}

fn native_matched_cell_headers<'a>(
    row: &'a TableCandidateLineHeaderRow,
    interval: &TableCandidateInterval,
) -> Option<Vec<&'a ShanaiLanLineHeader>> {
    let mut headers: Vec<&ShanaiLanLineHeader> = Vec::new();
    for segment in interval.column_segments() {
        let start = segment.source_start()?;
        let header = row
            .headers
            .iter()
            .filter(|header| header.end / 2 == start)
            .collect::<Vec<_>>();
        if header.len() != 1
            || headers
                .iter()
                .any(|existing| existing.start == header[0].start)
        {
            return None;
        }
        headers.push(header[0]);
    }
    Some(headers)
}

fn native_parent_row_header(
    bytes: &[u8],
    row_start_unit: usize,
) -> Option<NativeControlTableRowHeader> {
    let units = bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<_>>();
    let mut matches = Vec::new();
    for start in row_start_unit.saturating_sub(512)..=row_start_unit {
        if units.get(start..start + 4)? != [0x001c, 0x0010, *units.get(start + 2)?, 0] {
            continue;
        }
        let length = usize::from(*units.get(start + 2)?);
        let end = start.checked_add(length)?;
        if length < 13 || (start != row_start_unit && end != row_start_unit) {
            continue;
        }
        if units.get(end.checked_sub(4)?..end)? != [length as u16, 0, 0x0010, 0x001f] {
            continue;
        }
        matches.push(NativeControlTableRowHeader {
            start_unit: start,
            grid_extent: *units.get(start + 6)?,
            left_edge_units: *units.get(start + 8)?,
        });
    }
    (matches.len() == 1).then(|| matches[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_word_spacing_uses_measured_advances_not_character_stretching() {
        assert_eq!(english_word_spacing_px(120.0, 60.0, "A B C"), Some(30.0));
        assert_eq!(english_word_spacing_px(60.0, 60.0, "A B C"), Some(0.0));
        for natural in [0.0, -1.0, f32::NAN, f32::INFINITY, 121.0] {
            assert_eq!(english_word_spacing_px(120.0, natural, "A B C"), None);
        }
        assert_eq!(english_word_spacing_px(120.0, 60.0, "ABC"), None);
    }

    #[test]
    #[ignore = "requires private native input pairs"]
    fn native_english_justification_changes_word_spacing_without_changing_source_flow() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/native-fixtures");
        let on =
            parse_document(&std::fs::read(root.join("wrap-auto-on-008.jtd")).unwrap()).unwrap();
        let off =
            parse_document(&std::fs::read(root.join("wrap-auto-off-008.jtd")).unwrap()).unwrap();
        assert_eq!(on.document_text_flow(), off.document_text_flow());
        assert_eq!(on.english_justification_candidate(), Some(true));
        assert_eq!(off.english_justification_candidate(), Some(false));
        for (doc, enabled) in [(on, true), (off, false)] {
            let core = DocumentCore::from_document(doc);
            let targets = core.page_word_justification_targets(0).unwrap();
            assert_eq!(targets.len(), if enabled { 2 } else { 0 });
            let raw = core.render_page_svg(0).unwrap();
            assert_eq!(
                raw.matches("data-word-spacing-resolved=\"false\"").count(),
                targets.len()
            );
            assert!(!raw.contains("lengthAdjust=\"spacing\""));
            let widths = targets
                .iter()
                .map(|(unit, width)| (*unit, width - 20.0))
                .collect();
            let measured = core.render_page_svg_with_text_widths(0, &widths).unwrap();
            assert_eq!(measured.matches(" word-spacing=\"").count(), targets.len());
            assert_eq!(
                measured
                    .matches("data-word-spacing-resolved=\"true\"")
                    .count(),
                targets.len()
            );
            assert_eq!(measured.matches(">CELL-B</text>").count(), 1);
            assert_eq!(measured.matches("END-A</text>").count(), 1);
            let layers = core.get_page_layer_tree(0).unwrap();
            assert_eq!(
                layers.matches("\"wordJustificationWidth\"").count(),
                targets.len()
            );
            let projection = native_rule_flow_text_projection(
                &core.document,
                core.page_layout,
                1,
                WritingMode::Horizontal,
                &[],
                None,
            )
            .unwrap();
            let after = core
                .document
                .document_text_flow()
                .unwrap()
                .events()
                .iter()
                .rfind(|event| event.kind() == DocumentTextFlowKind::Text)
                .unwrap();
            let after_top =
                native_rule_body_top_y(&core.document, core.page_layout, true, after.source_span())
                    .unwrap();
            assert!(
                projection
                    .slots
                    .iter()
                    .all(|slot| after_top > slot.baseline_y)
            );
            assert_eq!(
                native_rule_body_top_y(
                    &core.document,
                    core.page_layout,
                    false,
                    after.source_span()
                ),
                None
            );
            assert_eq!(
                core.page_word_justification_targets(1)
                    .unwrap_or_default()
                    .len(),
                0
            );
        }
    }

    #[test]
    fn flow_text_rendering_retains_distributed_extent_and_raw_flags() {
        let projection = NativeControlFlowTextProjection {
            projection_kind: "nativeRuleFlowTextProjection",
            slots: vec![NativeControlFlowTextSlot {
                text: "A B".into(),
                x: 20.0,
                text_anchor: "start",
                baseline_y: 40.0,
                font_size: DocumentTextFontSize {
                    px: 14.0,
                    basis: "test",
                },
                source_span: TextSourceSpan::new(48, 54, 24, 27),
                line_mark_record_index: 2,
                page_mark_pitch_mm100: 592,
                leading_ascii_space_count: 0,
                preceding_header_offset_units: 2,
                preceding_header_extent_units: 78,
                raw_span_flags: [3, 2],
                text_length_px: Some(120.0),
                word_justification_width_px: None,
            }],
        };
        let mut svg = String::new();
        push_native_control_flow_text_svg(&mut svg, Some(&projection), "A&B", &BTreeMap::new());
        assert!(svg.contains("font-family=\"A&amp;B\""));
        assert!(svg.contains("textLength=\"120.000\" lengthAdjust=\"spacing\""));
        assert!(svg.contains("data-span-raw-word6=\"3\" data-span-raw-word7=\"2\""));
        assert!(svg.contains("data-decoded=\"false\""));
        let mut json = String::new();
        push_page_layer_native_control_flow_text_slot_json(
            &mut json,
            0,
            &projection.slots[0],
            "A&B",
            projection.projection_kind,
        );
        assert!(json.contains(
            "\"textLength\":120.000,\"lengthAdjust\":\"spacing\",\"positionsDecoded\":false"
        ));
        assert!(json.contains("\"spanRawWord6\":3,\"spanRawWord7\":2"));
        assert!(json.contains("\"positions\":[0.000,56.150,112.300,120.000]"));
        assert!(json.contains("\"geometryDecoded\":false"));

        for (anchor, bbox_x) in [("middle", "8.450"), ("end", "-3.100")] {
            let mut aligned = projection.clone();
            aligned.slots[0].text_anchor = anchor;
            aligned.slots[0].text_length_px = None;
            let mut svg = String::new();
            push_native_control_flow_text_svg(&mut svg, Some(&aligned), "A&B", &BTreeMap::new());
            assert!(svg.contains(&format!("text-anchor=\"{anchor}\"")));
            let mut json = String::new();
            push_page_layer_native_control_flow_text_slot_json(
                &mut json,
                0,
                &aligned.slots[0],
                "A&B",
                aligned.projection_kind,
            );
            assert!(json.contains(&format!("\"textAnchor\":\"{anchor}\",\"anchorX\":20.000")));
            assert!(json.contains(&format!("\"bbox\":{{\"x\":{bbox_x}")));
            assert_eq!(json.matches("\"positionsDecoded\":false").count(), 1);
        }
    }

    #[test]
    #[ignore = "requires private native alignment pairs"]
    fn native_rule_flow_anchors_center_and_right_without_reconstructing_rows() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/native-fixtures");
        for (name, flag, anchor) in [
            ("table-align-center.jtd", 1, "middle"),
            ("table-align-right.jtd", 2, "end"),
        ] {
            let doc = parse_document(&std::fs::read(root.join(name)).unwrap()).unwrap();
            let layout = page_layout_with_source_margins(&doc, page_layout_from_document(&doc));
            let projection = native_rule_flow_text_projection(
                &doc,
                layout,
                1,
                WritingMode::Horizontal,
                &[],
                None,
            )
            .unwrap();
            assert_eq!(projection.slots.len(), 4);
            let long = projection
                .slots
                .iter()
                .filter(|slot| slot.text != "CELL-B")
                .collect::<Vec<_>>();
            assert_eq!(long.len(), 3);
            let unit = layout.body_width_px() / 160.0;
            let source_x = layout.margin_left_px() + if flag == 1 { 40.0 } else { 78.0 } * unit;
            assert!(long.iter().all(|slot| slot.raw_span_flags[0] == flag
                && slot.text_anchor == anchor
                && (slot.x - source_x).abs() < 0.001
                && slot.word_justification_width_px.is_none()
                && slot.text_length_px.is_none()));
            assert!(
                long.windows(2)
                    .all(|pair| pair[0].baseline_y < pair[1].baseline_y)
            );
            let core = DocumentCore::from_document(doc);
            let svg = core.render_page_svg(0).unwrap();
            assert_eq!(svg.matches(&format!("text-anchor=\"{anchor}\"")).count(), 3);
            assert_eq!(svg.matches(">CELL-A LONG").count(), 1);
            assert_eq!(svg.matches(">CELL-B</text>").count(), 1);
            assert!(!svg.contains("rjtd-column-grid-candidate"));
            assert!(!svg.contains("lengthAdjust=\"spacing\""));
            assert!(!svg.contains("data-word-justification-width"));
            assert!(core.page_word_justification_targets(0).unwrap().is_empty());
            let layers = core.get_page_layer_tree(0).unwrap();
            assert_eq!(
                layers
                    .matches(&format!("\"textAnchor\":\"{anchor}\""))
                    .count(),
                3
            );
            assert_eq!(layers.matches("nativeRuleFlowTextProjection").count(), 4);
        }
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn native_rule_flow_keeps_inherited_spacing_unproven() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/baseline-sweep/050_wrapped_one_cell.jtd");
        let document = parse_document(&std::fs::read(path).unwrap()).unwrap();
        let layout =
            page_layout_with_source_margins(&document, page_layout_from_document(&document));
        let projection = native_rule_flow_text_projection(
            &document,
            layout,
            1,
            WritingMode::Horizontal,
            &[],
            None,
        );
        // Inherited automatic spacing is not the same as explicit left alignment.
        assert!(projection.is_none());
    }

    #[test]
    #[ignore = "requires local document samples"]
    fn native_rule_flow_places_explicit_left_and_distributed_wrapped_spans() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/native-fixtures");
        for (name, flag) in [
            ("wrap-align-left-007.jtd", 0),
            ("wrap-align-default-007.jtd", 3),
        ] {
            let document = parse_document(&std::fs::read(root.join(name)).unwrap()).unwrap();
            let layout =
                page_layout_with_source_margins(&document, page_layout_from_document(&document));
            for (test_layout, page, mode) in [
                (layout, 2, WritingMode::Horizontal),
                (layout, 1, WritingMode::VerticalRl),
                (PageLayout::default(), 1, WritingMode::Horizontal),
            ] {
                assert!(native_rule_flow_text_projection(
                    &document, test_layout, page, mode, &[], None,
                ).is_none());
            }
            let projection = native_rule_flow_text_projection(
                &document,
                layout,
                1,
                WritingMode::Horizontal,
                &[],
                None,
            )
            .unwrap();
            assert_eq!(projection.slots.len(), 4, "{name}");
            assert!(projection.slots.iter().all(|slot| {
                slot.source_span.unit_end() - slot.source_span.unit_start()
                    == slot.text.encode_utf16().count()
            }));
            let long: Vec<_> = projection
                .slots
                .iter()
                .filter(|slot| slot.text != "CELL-B")
                .collect();
            assert_eq!(long.len(), 3);
            assert!(long.iter().all(|slot| slot.raw_span_flags[0] == flag));
            assert_eq!(long[0].raw_span_flags[1], 2);
            assert_eq!(long[2].raw_span_flags[1], 0);
            assert!(
                long.windows(2)
                    .all(|pair| pair[1].baseline_y > pair[0].baseline_y)
            );
            assert!(
                long.iter()
                    .all(|slot| (slot.x - 90.36 * 4.0 / 3.0).abs() < 0.3)
            );
            assert!(
                long.iter()
                    .all(|slot| slot.text_length_px.is_some() == (flag == 3))
            );
            let core = DocumentCore::from_document(document);
            let svg = core.render_page_svg(0).unwrap();
            assert_eq!(svg.matches(">CELL-A LONG").count(), 1);
            assert_eq!(svg.matches(">CELL-B</text>").count(), 1);
            assert_eq!(svg.matches("END-A</text>").count(), 1);
            assert!(!svg.contains("rjtd-column-grid-candidate"));
            assert_eq!(
                svg.matches("lengthAdjust=\"spacing\"").count(),
                if flag == 3 { 3 } else { 0 }
            );
            let layers = core.get_page_layer_tree(0).unwrap();
            assert_eq!(layers.matches("nativeRuleFlowTextProjection").count(), 4);
            if flag == 3 {
                assert!(layers.contains("\"renderSuppressedBySourceFlow\":true"));
            }
            assert_eq!(
                layers.matches("\"textLength\"").count(),
                if flag == 3 { 3 } else { 0 }
            );
        }
    }

    #[test]
    fn trailing_empty_rows_preserve_the_boundary_at_a_non_table_transition() {
        let mut bytes = vec![0; 192];
        for unit in [16, 48, 64] {
            let words = [
                0x001c_u16, 0x0010, 13, 0, 0x008f, 0, 160, 0, 14, 13, 0, 0x0010, 0x001f,
            ];
            for (index, word) in words.iter().enumerate() {
                bytes[(unit + index) * 2..(unit + index + 1) * 2]
                    .copy_from_slice(&word.to_be_bytes());
            }
        }
        let row = |index, start, end| DocumentTextControlTableRow {
            index,
            source_start: start,
            source_end: end,
            cells: Vec::new(),
        };
        let interval = |record_index, unit_start, unit_end| ShanaiLanLineMarkInterval {
            record_index,
            unit_start,
            unit_end,
            flag_word: 2,
        };
        let rows = [row(0, 16, 48), row(1, 48, 80), row(2, 80, 81)];
        let intervals = [
            interval(2, 16, 48),
            interval(3, 48, 80),
            interval(4, 80, 81),
        ];
        assert_eq!(
            native_control_table_trailing_empty_row_record(&bytes, 16, 160, 14, &rows, &intervals),
            Some(3)
        );

        // A gap in the row model must not be skipped even if a later header matches.
        assert_eq!(
            native_control_table_trailing_empty_row_record(
                &bytes,
                16,
                160,
                14,
                &[rows[0].clone(), row(1, 64, 80)],
                &intervals,
            ),
            Some(2)
        );
        assert_eq!(
            native_control_table_trailing_empty_row_record(
                &bytes,
                16,
                160,
                14,
                &rows,
                &[interval(2, 16, 47)],
            ),
            None
        );
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn local_native_control_table_projection_tracks_controlled_grid_coordinates() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid",
        );
        for name in [
            "PAGE 01",
            "PAGE 01_down_4Low",
            "PAGE 01_right_4Tick",
            "PAGE 01_DOWNTEST_1LINE",
            "../baseline-sweep/000_base_a",
            "../baseline-sweep/013_table_moved_right",
            "../baseline-sweep/020_row1_height_plus",
        ] {
            let path = root.join(format!("{name}.jtd"));
            let document = parse_document(&std::fs::read(path).unwrap()).unwrap();
            let layout =
                page_layout_with_source_margins(&document, page_layout_from_document(&document));
            let projections = native_control_table_text_projections(
                &document,
                layout,
                1,
                WritingMode::Horizontal,
            );
            assert_eq!(projections.len(), 1, "{name}");
            assert_eq!(projections[0].slots.len(), 6, "{name}");
            let row_records = projections[0]
                .slots
                .iter()
                .filter(|slot| slot.column_index == 0)
                .map(|slot| slot.line_mark_record_index)
                .collect::<Vec<_>>();
            let expected_records = match name {
                "PAGE 01" => vec![2, 4],
                "PAGE 01_down_4Low" => vec![6, 8],
                "PAGE 01_right_4Tick" => vec![2, 4],
                "PAGE 01_DOWNTEST_1LINE" => vec![3, 5],
                "../baseline-sweep/000_base_a" => vec![1, 3, 5],
                "../baseline-sweep/013_table_moved_right" => vec![1, 3, 5],
                "../baseline-sweep/020_row1_height_plus" => vec![1, 5, 7],
                _ => row_records.clone(),
            };
            assert_eq!(row_records, expected_records, "{name}");
            if name == "PAGE 01" {
                let first = &projections[0].slots[0];
                assert!((first.x - 120.472_44).abs() < 0.01);
                assert_eq!(first.header_offset_units, 2);
                assert_eq!(first.page_mark_pitch_mm100, 592);
            }
            assert!(
                projections[0]
                    .slots
                    .windows(2)
                    .any(|pair| pair[1].baseline_y > pair[0].baseline_y),
                "{name}"
            );
        }
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn native_control_table_projection_rejects_wrapped_control_rows() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/baseline-sweep/050_wrapped_one_cell.jtd");
        let document = parse_document(&std::fs::read(path).unwrap()).unwrap();
        let layout =
            page_layout_with_source_margins(&document, page_layout_from_document(&document));
        assert!(
            native_control_table_text_projections(&document, layout, 1, WritingMode::Horizontal)
                .is_empty()
        );
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn local_native_control_table_projection_admits_only_the_verified_horizontal_series() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus");
        let admitted = [
            "baseline-sweep/000_base_a",
            "baseline-sweep/001_base_b_resave",
            "baseline-sweep/010_table_moved_down_small",
            "baseline-sweep/011_table_moved_down_large",
            "baseline-sweep/013_table_moved_right",
            "baseline-sweep/020_row1_height_plus",
            "baseline-sweep/021_row2_height_plus",
            "baseline-sweep/022_row3_height_plus",
            "baseline-sweep/040_top_margin_plus",
            "page01-grid/PAGE 01",
            "page01-grid/PAGE 01_DOWNTEST_1LINE",
            "page01-grid/PAGE 01_DOWNTEST_BASE",
            "page01-grid/PAGE 01_down_1Low",
            "page01-grid/PAGE 01_down_2Low",
            "page01-grid/PAGE 01_down_3Low",
            "page01-grid/PAGE 01_down_4Low",
            "page01-grid/PAGE 01_right_1Tick",
            "page01-grid/PAGE 01_right_2Tick",
            "page01-grid/PAGE 01_right_3Tick",
            "page01-grid/PAGE 01_right_4Tick",
        ];
        for relative in admitted {
            let document =
                parse_document(&std::fs::read(root.join(format!("{relative}.jtd"))).unwrap())
                    .unwrap();
            let layout =
                page_layout_with_source_margins(&document, page_layout_from_document(&document));
            let projections = native_control_table_text_projections(
                &document,
                layout,
                1,
                WritingMode::Horizontal,
            );
            assert_eq!(projections.len(), 1, "{relative}");
            assert_eq!(projections[0].slots.len(), 6, "{relative}");
            let core = DocumentCore::from_document(document);
            let svg = core.render_page_svg(0).unwrap();
            assert!(svg.contains("rjtd-native-control-table-text"), "{relative}");
            assert_eq!(svg.matches(">R01C01</text>").count(), 1, "{relative}");
            let layers = core.get_page_layer_tree(0).unwrap();
            assert_eq!(
                layers.matches("nativeControlTableTextProjection").count(),
                6,
                "{relative}"
            );
        }

        let relative = "baseline-sweep/050_wrapped_one_cell";
        let document =
            parse_document(&std::fs::read(root.join(format!("{relative}.jtd"))).unwrap()).unwrap();
        let layout =
            page_layout_with_source_margins(&document, page_layout_from_document(&document));
        assert!(
            native_control_table_text_projections(&document, layout, 1, WritingMode::Horizontal)
                .is_empty(),
            "{relative}"
        );
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn local_native_control_flow_projection_requires_two_bracketing_native_tables() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/baseline-sweep",
        );
        let document =
            parse_document(&std::fs::read(root.join("070_two_tables_vertical.jtd")).unwrap())
                .unwrap();
        let layout =
            page_layout_with_source_margins(&document, page_layout_from_document(&document));
        let tables =
            native_control_table_text_projections(&document, layout, 1, WritingMode::Horizontal);
        assert_eq!(tables.len(), 2);
        assert_eq!(
            tables.iter().map(|table| table.slots.len()).sum::<usize>(),
            8
        );
        let flow = native_control_flow_text_projection(
            &document,
            layout,
            1,
            WritingMode::Horizontal,
            &tables,
        )
        .expect("two bracketing native control tables with a source-mapped flow run");
        assert_eq!(flow.slots.len(), 2);
        assert_eq!(
            flow.slots
                .iter()
                .map(|slot| slot.text.as_str())
                .collect::<Vec<_>>(),
            vec!["        BETWEEN01", "        BETWEEN02"]
        );
        assert_eq!(
            flow.slots
                .iter()
                .map(|slot| slot.line_mark_record_index)
                .collect::<Vec<_>>(),
            vec![7, 9]
        );
        assert!(flow.slots.iter().all(|slot| {
            slot.page_mark_pitch_mm100 == 592
                && slot.leading_ascii_space_count == 8
                && slot.preceding_header_offset_units == 0
                && slot.preceding_header_extent_units == 14
        }));

        let core = DocumentCore::from_document(document);
        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("rjtd-native-control-flow-text"));
        assert_eq!(svg.matches("BETWEEN01").count(), 1);
        assert_eq!(svg.matches("BETWEEN02").count(), 1);
        let layers = core.get_page_layer_tree(0).unwrap();
        assert_eq!(layers.matches("nativeControlFlowTextProjection").count(), 2);

        let bytes = document_text_raw_stream(&core.document).unwrap();
        let rows =
            sparse_document_text_control_table_rows(core.document.document_text_flow().unwrap());
        assert_eq!(
            native_control_table_trailing_empty_row_record(
                bytes,
                770,
                160,
                14,
                &rows,
                &shanai_lan_line_mark_intervals(&core.document),
            ),
            Some(17)
        );

        let single_table =
            parse_document(&std::fs::read(root.join("000_base_a.jtd")).unwrap()).unwrap();
        let single_layout = page_layout_with_source_margins(
            &single_table,
            page_layout_from_document(&single_table),
        );
        let single_tables = native_control_table_text_projections(
            &single_table,
            single_layout,
            1,
            WritingMode::Horizontal,
        );
        assert_eq!(single_tables.len(), 1);
        assert!(
            native_control_flow_text_projection(
                &single_table,
                single_layout,
                1,
                WritingMode::Horizontal,
                &single_tables,
            )
            .is_none()
        );
    }
}
