use crate::*;

pub(crate) struct NativeVerticalRun {
    fragment: PageLayerTextFragment,
    column: f32,
    top: f32,
    font_size: f32,
    advance: f32,
    spacing: f32,
    tate: bool,
    record: usize,
}

/// Physical source columns and the controlled 60% character-spacing profile.
/// The percent association is a candidate, not a general binary-unit decoder.
pub(crate) fn native_vertical_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    lines: &[PageTextLine],
    writing_mode: WritingMode,
) -> Option<Vec<NativeVerticalRun>> {
    if !writing_mode.is_vertical()
        || modern_source_writing_mode(document) != Some(WritingMode::VerticalRl)
        || !layout.has_source_margins()
    {
        return None;
    }
    let view = document
        .unknown_styles()
        .iter()
        .find(|s| s.name() == Some(DOCUMENT_VIEW_STYLES_PATH))?;
    let summary = summarize_style_stream(view.payload());
    let payload = |code| {
        let r = summary.records().iter().find(|r| r.code() == code)?;
        let at = r.offset().checked_add(4)?;
        view.payload().get(at..at.checked_add(r.payload_len())?)
    };
    let font = payload(0x1006)?;
    if document.character_spacing_percent_candidate() != Some(60) {
        return None;
    }
    let font_size = document_default_font_size_px(document)?;
    let default_font_id = u16::from_be_bytes(font.get(5..7)?.try_into().ok()?);
    let resolver = document_text_style_resolver(document)?;
    let candidates = document.tatechuyoko_candidates();
    let mut output = Vec::new();
    for line in lines {
        let record = line.native_line_mark_index?;
        let (page, top, _) = native_rule_line_placement(document, layout, record)?;
        if page != page_number {
            return None;
        }
        let column = layout.width_px()
            - layout.margin_right_px()
            - font_size / 2.0
            - (top - layout.margin_top_px());
        let mut y = layout.margin_top_px();
        for fragment in page_text_line_style_fragments(document, line, Some(&resolver)) {
            if fragment.text.is_empty() {
                continue;
            }
            let span =
                native_visible_text_span(document, &fragment.text, fragment.source_span.as_ref()?)?;
            if fragment.ruby_annotation.is_some()
                || (document_text_font_size(&resolver, &span, Some(font_size))?.px - font_size)
                    .abs()
                    > 0.01
            {
                return None;
            }
            let style = document_text_character_style(document, &resolver, &span);
            if style.bold
                || style.italic
                || style.underline
                || style.script.is_some()
                || style
                    .font
                    .as_ref()
                    .is_some_and(|(id, _)| *id != default_font_id)
            {
                return None;
            }
            if document_text_foreground_color(&resolver, &span)
                .is_some_and(|color| color != "#000000")
            {
                return None;
            }
            let tate = candidates.iter().find(|c| c.value_span() == &span);
            if candidates.iter().any(|c| {
                c.value_span().unit_start() < span.unit_end()
                    && span.unit_start() < c.value_span().unit_end()
            }) && tate.is_none()
            {
                return None;
            }
            let mut parts = Vec::new();
            if tate.is_some() {
                parts.push((0, fragment.text.chars().count(), true));
            } else {
                let chars = fragment.text.chars().collect::<Vec<_>>();
                let mut start = 0;
                while start < chars.len() {
                    let ascii = chars[start].is_ascii();
                    let end = start
                        + chars[start..]
                            .iter()
                            .take_while(|c| c.is_ascii() == ascii)
                            .count();
                    parts.push((start, end, false));
                    start = end;
                }
            }
            for (start, end, tate) in parts {
                let text = text_by_char_range(&fragment.text, start, end);
                let units_start = fragment.text.chars().take(start).map(char::len_utf16).sum();
                let units_end = fragment.text.chars().take(end).map(char::len_utf16).sum();
                let ascii = text.is_ascii();
                let spacing = if ascii || tate { 0.0 } else { font_size * 0.6 };
                let advance = if tate {
                    font_size * 1.6
                } else if ascii {
                    text_width_px_for_font_size(font_size, &text) as f32
                } else {
                    text.chars().count() as f32 * (font_size + spacing)
                };
                output.push(NativeVerticalRun {
                    fragment: PageLayerTextFragment {
                        text,
                        paragraph_index: fragment.paragraph_index,
                        char_start: fragment.char_start + start,
                        char_end: fragment.char_start + end,
                        source_span: Some(span.subspan_by_units(units_start, units_end)),
                        ruby_annotation: None,
                    },
                    column,
                    top: y,
                    font_size,
                    advance,
                    spacing,
                    tate,
                    record,
                });
                y += advance;
            }
        }
        if y > layout.height_px() - layout.margin_bottom_px() + font_size {
            return None;
        }
    }
    Some(output)
}

pub(crate) fn push_native_vertical_svg(
    svg: &mut String,
    runs: &[NativeVerticalRun],
    font_family: &str,
) {
    for run in runs {
        let mode = if run.tate {
            "horizontal-tb"
        } else {
            "vertical-rl"
        };
        let anchor = if run.tate { "middle" } else { "start" };
        let y = run.top + if run.tate { run.font_size } else { 0.0 };
        svg.push_str(&format!("<text class=\"rjtd-native-vertical-text\" data-source=\"DocumentViewStyles+LineMark+PageMark\" data-line-mark-record-index=\"{}\" data-source-unit-start=\"{}\" data-tatechuyoko-candidate=\"{}\" data-character-spacing-percent-candidate=\"60\" data-decoded=\"false\" data-geometry-decoded=\"false\" x=\"{:.3}\" y=\"{y:.3}\" writing-mode=\"{mode}\" text-anchor=\"{anchor}\" font-family=\"{}\" font-size=\"{:.3}\" letter-spacing=\"{:.3}\" fill=\"#111111\" xml:space=\"preserve\">{}</text>", run.record, run.fragment.source_span.as_ref().map_or(0, TextSourceSpan::unit_start), run.tate, run.column, escape_xml(font_family), run.font_size, run.spacing, escape_xml(&run.fragment.text)));
    }
}

pub(crate) fn push_native_vertical_layer_json(
    output: &mut String,
    sources: &mut Vec<String>,
    runs: &[NativeVerticalRun],
    font_family: &str,
) {
    for run in runs {
        let source_id = sources.len();
        output.push_str(&format!(",{{\"type\":\"textRun\",\"bbox\":{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}},\"text\":{},\"baseline\":{:.3},\"fontSize\":{:.3},\"fontFamily\":{},\"isVertical\":{},\"orientation\":{},\"rotation\":0.0,\"textAnchor\":{},\"tatechuyokoCandidate\":{},\"characterSpacingPercentCandidate\":60,\"lineMarkRecordIndex\":{},\"projectionKind\":\"nativeVerticalCandidate\",\"decoded\":false,\"geometryDecoded\":false,\"positionsDecoded\":false,\"source\":",run.column - run.font_size / 2.0, run.top, run.font_size, if run.tate { run.font_size } else { run.advance }, json_string(&run.fragment.text), if run.tate { run.top + run.font_size } else { run.column }, run.font_size, json_string(font_family), !run.tate, json_string(if run.tate { "horizontal-tb" } else { "vertical-rl" }), json_string(if run.tate { "middle" } else { "start" }), run.tate, run.record));
        push_page_layer_source_span_json(output, source_id, &run.fragment);
        output.push('}');
        push_page_layer_text_source_json(sources, source_id, &run.fragment);
    }
}
