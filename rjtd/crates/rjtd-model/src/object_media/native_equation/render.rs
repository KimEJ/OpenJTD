use super::source::{NativeEquationCandidate, native_equation_binding};
use crate::*;

pub(crate) struct NativeEquationProjection {
    candidate: NativeEquationCandidate,
    bbox: [f32; 4],
    body: Vec<(PageLayerTextFragment, f32)>,
}

pub(crate) fn native_equation_projection(
    document: &Document,
    layout: PageLayout,
    page: usize,
    mode: WritingMode,
    lines: &[PageTextLine],
) -> Option<NativeEquationProjection> {
    if page != 1
        || mode.is_vertical()
        || !layout.has_source_margins()
        || document
            .unknown_styles()
            .iter()
            .any(|s| s.name() == Some(PAGE_LAYOUT_STYLE_PATH))
    {
        return None;
    }
    let candidate = native_equation_binding(document)?;
    let (p, top, _) = native_rule_line_placement(document, layout, candidate.line)?;
    if p != page {
        return None;
    }
    let bbox = [
        layout.margin_left_px() + hundredth_millimeters_to_css_px(u32::from(candidate.frame[0])),
        top + hundredth_millimeters_to_css_px(u32::from(candidate.frame[1])),
        hundredth_millimeters_to_css_px(u32::from(candidate.frame[2])),
        hundredth_millimeters_to_css_px(u32::from(candidate.frame[3])),
    ];
    if bbox[0] + bbox[2] > layout.width_px() - layout.margin_right_px()
        || bbox[1] + bbox[3] > layout.height_px() - layout.margin_bottom_px()
    {
        return None;
    }
    let text_lines = lines
        .iter()
        .filter(|l| !l.text().is_empty())
        .collect::<Vec<_>>();
    let [before, after] = text_lines.as_slice() else {
        return None;
    };
    let paragraphs = document_paragraph_texts(document);
    if paragraphs.len() != 2 || before.text() != paragraphs[0].1 || after.text() != paragraphs[1].1
    {
        return None;
    }
    let font = document_default_font_size_px(document)?;
    let resolver = document_text_style_resolver(document)?;
    let mut body = Vec::new();
    for (line, baseline) in [
        (before, layout.margin_top_px() + font),
        (
            after,
            bbox[1]
                + bbox[3]
                + hundredth_millimeters_to_css_px(u32::from(
                    document
                        .page_marks()
                        .first()?
                        .entries()
                        .first()?
                        .u16_fields()[14],
                ))
                + font,
        ),
    ] {
        let fragments = page_text_line_style_fragments(document, line, Some(&resolver));
        if fragments.len() != 1 {
            return None;
        }
        for f in fragments {
            let span = native_visible_text_span(document, &f.text, f.source_span.as_ref()?)?;
            if (document_text_font_size(&resolver, &span, Some(font))?.px - font).abs() > 0.01
                || f.ruby_annotation.is_some()
            {
                return None;
            }
            let style = document_text_character_style(document, &resolver, &span);
            if style.bold
                || style.italic
                || style.underline
                || style.script.is_some()
                || document_text_foreground_color(&resolver, &span)
                    .is_some_and(|color| color != "#000000")
            {
                return None;
            }
            body.push((f, baseline));
        }
    }
    Some(NativeEquationProjection {
        candidate,
        bbox,
        body,
    })
}

pub(crate) fn push_native_equation_svg(
    svg: &mut String,
    p: &NativeEquationProjection,
    document: &Document,
) {
    let default_font = document_default_font_size_px(document).unwrap_or(APP_FONT_SIZE_PX);
    let resolver = document_text_style_resolver(document);
    let default_family = document_font_family_css(document);
    for (fragment, baseline) in &p.body {
        let style = fragment
            .source_span
            .as_ref()
            .zip(resolver.as_ref())
            .map(|(s, r)| document_text_character_style(document, r, s))
            .unwrap_or_default();
        let family = style
            .font
            .as_ref()
            .map_or(default_family.as_str(), |(_, f)| f.as_str());
        push_svg_text_run(
            svg,
            "rjtd-native-equation-body",
            p.bbox[0],
            *baseline,
            family,
            default_font,
            "#111111",
            &fragment.text,
            None,
            None,
            None,
        );
    }
    for glyph in &p.candidate.glyphs {
        let x = p.bbox[0] + hundredth_millimeters_to_css_px(glyph.x);
        let y = p.bbox[1] + hundredth_millimeters_to_css_px(glyph.y);
        let size = hundredth_millimeters_to_css_px(glyph.font_size);
        svg.push_str(&format!("<text class=\"rjtd-native-equation-glyph\" data-source=\"JSEQ3Contents+EmbeddedPress\" data-snapshot-offset=\"{}\" data-contents-offset=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" x=\"{x:.3}\" y=\"{y:.3}\" dominant-baseline=\"text-before-edge\" font-family=\"{}\" font-size=\"{size:.3}\" font-style=\"{}\" fill=\"#000000\">{}</text>",glyph.byte_start,glyph.contents_offset,escape_xml(&format!("'{}', 'Times New Roman', Times, serif",glyph.family)),if glyph.italic{"italic"}else{"normal"},escape_xml(&glyph.text)));
    }
}

pub(crate) fn push_native_equation_layer_json(
    out: &mut String,
    sources: &mut Vec<String>,
    p: &NativeEquationProjection,
    document: &Document,
) {
    let font = document_default_font_size_px(document).unwrap_or(APP_FONT_SIZE_PX);
    let resolver = document_text_style_resolver(document);
    let default_family = document_font_family_css(document);
    for (fragment, baseline) in &p.body {
        let id = sources.len();
        let style = fragment
            .source_span
            .as_ref()
            .zip(resolver.as_ref())
            .map(|(s, r)| document_text_character_style(document, r, s))
            .unwrap_or_default();
        let family = style
            .font
            .as_ref()
            .map_or(default_family.as_str(), |(_, f)| f.as_str());
        out.push_str(&format!(",{{\"type\":\"textRun\",\"text\":{},\"bbox\":{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{font:.3}}},\"baseline\":{baseline:.3},\"fontSize\":{font:.3},\"fontFamily\":{},\"projectionKind\":\"nativeEquationBodyCandidate\",\"decoded\":false,\"geometryDecoded\":false,\"source\":",json_string(&fragment.text),p.bbox[0],baseline-font,text_width_px_for_font_size(font,&fragment.text),json_string(family)));
        push_page_layer_source_span_json(out, id, fragment);
        out.push('}');
        push_page_layer_text_source_json(sources, id, fragment);
    }
    let [x, y, width, height] = p.bbox;
    out.push_str(&format!(",{{\"type\":\"nativeEquationCandidate\",\"bbox\":{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}},\"lineMarkRecordIndex\":{},\"glyphs\":[",p.candidate.line));
    for (i, g) in p.candidate.glyphs.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!("{{\"text\":{},\"x\":{:.3},\"top\":{:.3},\"fontSize\":{:.3},\"fontFamily\":{},\"italic\":{},\"snapshotByteStart\":{},\"contentsOffset\":{}}}",json_string(&g.text),x+hundredth_millimeters_to_css_px(g.x),y+hundredth_millimeters_to_css_px(g.y),hundredth_millimeters_to_css_px(g.font_size),json_string(&g.family),g.italic,g.byte_start,g.contents_offset));
    }
    out.push_str("],\"decoded\":false,\"geometryDecoded\":false,\"baselineDecoded\":false,\"referenceBacked\":false}");
}
