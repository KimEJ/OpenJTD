use crate::*;

pub(crate) struct NativeTrackingRun {
    fragment: PageLayerTextFragment,
    x: f32,
    baseline: f32,
    font: f32,
    spacing: f32,
    family: String,
    advance: f32,
    line: usize,
}

/// Source physical rows and the corroborated global landscape 60% spacing profile.
/// Latin advances remain backend-owned; Japanese fullwidth cells stay candidates.
pub(crate) fn native_tracking_projection(
    document: &Document,
    layout: PageLayout,
    page: usize,
    mode: WritingMode,
    lines: &[PageTextLine],
    measured: &BTreeMap<usize, f32>,
) -> Option<Vec<NativeTrackingRun>> {
    if mode.is_vertical()
        || layout.width_px() <= layout.height_px()
        || modern_source_writing_mode(document) != Some(WritingMode::Horizontal)
        || document.character_spacing_percent_candidate() != Some(60)
        || !layout.has_source_margins()
    {
        return None;
    }
    if !document.table_candidates().is_empty()
        || !document.text_field_candidates().is_empty()
        || !document.footnote_text_candidates().is_empty()
        || !document.toc_entries().is_empty()
        || !document.object_frame_records().is_empty()
    {
        return None;
    }
    let font = document_default_font_size_px(document)?;
    let resolver = document_text_style_resolver(document)?;
    let mut runs = Vec::new();
    for line in lines {
        let record = line.native_line_mark_index?;
        let (p, top, _) = native_rule_line_placement(document, layout, record)?;
        if p != page {
            return None;
        }
        let mut x = layout.margin_left_px();
        for fragment in page_text_line_style_fragments(document, line, Some(&resolver)) {
            if fragment.text.is_empty() {
                continue;
            }
            let span =
                native_visible_text_span(document, &fragment.text, fragment.source_span.as_ref()?)?;
            if (document_text_font_size(&resolver, &span, Some(font))?.px - font).abs() > 0.01
                || fragment.ruby_annotation.is_some()
            {
                return None;
            }
            let style = document_text_character_style(document, &resolver, &span);
            if style.bold
                || style.italic
                || style.underline
                || style.script.is_some()
                || document_text_foreground_color(&resolver, &span).is_some_and(|c| c != "#000000")
            {
                return None;
            }
            let family = style
                .font
                .as_ref()
                .map_or_else(|| document_font_family_css(document), |(_, f)| f.clone());
            let chars = fragment.text.chars().collect::<Vec<_>>();
            let mut start = 0;
            while start < chars.len() {
                let ascii = chars[start].is_ascii();
                let end = start
                    + chars[start..]
                        .iter()
                        .take_while(|c| c.is_ascii() == ascii)
                        .count();
                let text = chars[start..end].iter().collect::<String>();
                let u0 = chars[..start].iter().map(|c| c.len_utf16()).sum();
                let u1 = chars[..end].iter().map(|c| c.len_utf16()).sum();
                let source = span.subspan_by_units(u0, u1);
                let spacing = if ascii { 0.0 } else { font * 0.6 };
                let advance = if ascii {
                    measured
                        .get(&source.unit_start())
                        .copied()
                        .filter(|w| w.is_finite() && *w > 0.0)
                        .unwrap_or_else(|| text_width_px_for_font_size(font, &text) as f32)
                } else {
                    (end - start) as f32 * (font + spacing)
                };
                runs.push(NativeTrackingRun {
                    fragment: PageLayerTextFragment {
                        text,
                        paragraph_index: fragment.paragraph_index,
                        char_start: fragment.char_start + start,
                        char_end: fragment.char_start + end,
                        source_span: Some(source),
                        ruby_annotation: None,
                    },
                    x,
                    baseline: top + font,
                    font,
                    spacing,
                    family: family.clone(),
                    advance,
                    line: record,
                });
                x += advance;
                start = end;
            }
        }
        if x > layout.width_px() - layout.margin_right_px() + font {
            return None;
        }
    }
    Some(runs)
}

pub(crate) fn native_tracking_units(runs: &[NativeTrackingRun]) -> Vec<usize> {
    runs.iter()
        .filter(|r| r.spacing == 0.0)
        .filter_map(|r| {
            r.fragment
                .source_span
                .as_ref()
                .map(TextSourceSpan::unit_start)
        })
        .collect()
}

pub(crate) fn push_native_tracking_svg(svg: &mut String, runs: &[NativeTrackingRun]) {
    for r in runs {
        let unit = r
            .fragment
            .source_span
            .as_ref()
            .map_or(0, TextSourceSpan::unit_start);
        svg.push_str(&format!("<text class=\"rjtd-native-tracking\" id=\"rjtd-text-advance-{unit}\" data-source-unit-start=\"{unit}\" data-character-spacing-percent-candidate=\"60\" data-line-mark-record-index=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\" letter-spacing=\"{:.3}\" fill=\"#111111\" xml:space=\"preserve\">{}</text>",r.line,r.x,r.baseline,escape_xml(&r.family),r.font,r.spacing,escape_xml(&r.fragment.text)));
    }
}

pub(crate) fn push_native_tracking_layer_json(
    out: &mut String,
    sources: &mut Vec<String>,
    runs: &[NativeTrackingRun],
) {
    for r in runs {
        let id = sources.len();
        out.push_str(&format!(",{{\"type\":\"textRun\",\"bbox\":{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}},\"text\":{},\"baseline\":{:.3},\"fontSize\":{:.3},\"fontFamily\":{},\"letterSpacingCandidate\":{:.3},\"characterSpacingPercentCandidate\":60,\"lineMarkRecordIndex\":{},\"projectionKind\":\"nativeTrackingCandidate\",\"decoded\":false,\"geometryDecoded\":false,\"positionsDecoded\":false,\"source\":",r.x,r.baseline-r.font,r.advance,r.font,json_string(&r.fragment.text),r.baseline,r.font,json_string(&r.family),r.spacing,r.line));
        push_page_layer_source_span_json(out, id, &r.fragment);
        out.push('}');
        push_page_layer_text_source_json(sources, id, &r.fragment);
    }
}
