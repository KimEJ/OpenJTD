use super::source::{DocumentImageFrameCandidate, NativeImageMode, native_image_binding};
use crate::*;

pub(crate) struct NativeImageText {
    fragment: PageLayerTextFragment,
    x: f32,
    baseline: f32,
    width: f32,
    scale: f32,
}

pub(crate) struct NativeImageProjection {
    binding: DocumentImageFrameCandidate,
    pub(crate) bbox: [f32; 4],
    data_uri: String,
    font_size: f32,
    font_family: String,
    text: Vec<NativeImageText>,
}

pub(crate) fn native_image_projection(
    document: &Document,
    layout: PageLayout,
    page: usize,
    mode: WritingMode,
    lines: &[PageTextLine],
    measured: &BTreeMap<usize, f32>,
) -> Option<NativeImageProjection> {
    if page != 1
        || mode.is_vertical()
        || !layout.has_source_margins()
        || lines.len() != 1
        || document.blocks().len() != 1
        || document
            .unknown_styles()
            .iter()
            .any(|s| s.name() == Some(PAGE_LAYOUT_STYLE_PATH))
    {
        return None;
    }
    let binding = native_image_binding(document)?;
    let image = &document.object_stream_candidates()[binding.image_index].image_payload_spans()
        [binding.payload_index];
    let data_uri = image_payload_svg_data_uri(image)?;
    let font_size = document_default_font_size_px(document)?;
    let bbox = [
        layout.margin_left_px() + hundredth_millimeters_to_css_px(u32::from(binding.x)),
        layout.margin_top_px(),
        hundredth_millimeters_to_css_px(u32::from(binding.width)),
        hundredth_millimeters_to_css_px(u32::from(binding.height)),
    ];
    if bbox[0] <= layout.margin_left_px()
        || bbox[0] + bbox[2] >= layout.width_px() - layout.margin_right_px()
        || bbox[1] + bbox[3] >= layout.height_px() - layout.margin_bottom_px()
    {
        return None;
    }
    let intervals = shanai_lan_line_mark_intervals(document);
    let [interval] = intervals.as_slice() else {
        return None;
    };
    let flow = document.document_text_flow()?;
    if interval.unit_start != flow.source_span().unit_start()
        || interval.unit_end != flow.source_span().unit_end().checked_add(1)?
        || interval.flag_word != 0x8003
    {
        return None;
    }
    let mark = document.page_marks().first()?;
    let entry = mark.entries().first()?;
    let fields = entry.u16_fields();
    let base_mm100 = (font_size * 2540.0 / 96.0).round() as u16;
    let extent = if binding.mode == NativeImageMode::Inline {
        binding.height
    } else {
        base_mm100
    };
    if mark.family() != "fixed84"
        || entry.index() != Some(0)
        || entry.flags() != Some(0x10000)
        || entry.line_start() != Some(0)
        || [10, 13, 17, 18]
            .into_iter()
            .any(|i| fields.get(i) != Some(&extent))
        || fields.get(19) != Some(&base_mm100)
    {
        return None;
    }
    let resolver = document_text_style_resolver(document)?;
    let mut font_family = None;
    let paragraph = paragraph_by_index(document, 0)?;
    let actual = paragraph
        .inlines()
        .iter()
        .map(|inline| {
            if let Inline::Text(run) = inline {
                Some(run.text())
            } else {
                None
            }
        })
        .collect::<Option<Vec<_>>>()?
        .concat();
    let expected = flow
        .events()
        .iter()
        .filter(|e| e.kind() == DocumentTextFlowKind::Text)
        .map(DocumentTextFlowEvent::text)
        .collect::<String>();
    if actual != expected || lines[0].text() != actual {
        return None;
    }
    let mut glyphs = Vec::new();
    for fragment in page_text_line_style_fragments(document, &lines[0], Some(&resolver)) {
        let span =
            native_visible_text_span(document, &fragment.text, fragment.source_span.as_ref()?)?;
        if (document_text_font_size(&resolver, &span, Some(font_size))?.px - font_size).abs() > 0.01
            || fragment.ruby_annotation.is_some()
        {
            return None;
        }
        let style = document_text_character_style(document, &resolver, &span);
        if style.bold || style.italic || style.underline || style.script.is_some() {
            return None;
        }
        if document_text_foreground_color(&resolver, &span).is_some_and(|color| color != "#000000")
        {
            return None;
        }
        let family = style.font.as_ref().map_or_else(
            || document_font_family_css(document),
            |(_, family)| family.clone(),
        );
        if font_family
            .as_ref()
            .is_some_and(|previous| previous != &family)
        {
            return None;
        }
        font_family = Some(family);
        for (i, ch) in fragment.text.chars().enumerate() {
            let unit = span.unit_start() + i;
            let width = measured
                .get(&unit)
                .copied()
                .filter(|w| w.is_finite() && *w > 0.0 && *w <= font_size * 2.0)
                .unwrap_or(font_size * if ch == ' ' { 1.0 / 3.0 } else { 0.55 });
            glyphs.push((
                PageLayerTextFragment {
                    text: ch.to_string(),
                    paragraph_index: Some(0),
                    char_start: fragment.char_start + i,
                    char_end: fragment.char_start + i + 1,
                    source_span: Some(span.subspan_by_units(i, i + 1)),
                    ruby_annotation: None,
                },
                width,
            ));
        }
    }
    let baseline = if binding.mode == NativeImageMode::Inline {
        bbox[1] + bbox[3]
    } else {
        bbox[1] + font_size
    };
    let margin = hundredth_millimeters_to_css_px(u32::from(binding.wrap_margin));
    let mut x = layout.margin_left_px();
    let before_width = glyphs
        .iter()
        .filter(|(f, _)| {
            f.source_span
                .as_ref()
                .is_some_and(|s| s.unit_end() <= binding.record_span.unit_start())
        })
        .map(|(_, w)| *w)
        .sum::<f32>();
    let before_scale = if binding.mode == NativeImageMode::Inline {
        (bbox[0] - x) / before_width
    } else {
        1.0
    };
    if !before_scale.is_finite() || !(0.5..=2.0).contains(&before_scale) {
        return None;
    }
    let mut text = Vec::new();
    for (fragment, width) in glyphs {
        let before = fragment.source_span.as_ref()?.unit_end() <= binding.record_span.unit_start();
        let scale = if before && binding.mode == NativeImageMode::Inline {
            before_scale
        } else {
            1.0
        };
        if binding.mode == NativeImageMode::Inline
            && fragment.source_span.as_ref()?.unit_start() >= binding.cache_end
            && x < bbox[0] + bbox[2]
        {
            x = bbox[0] + bbox[2];
        }
        if binding.mode == NativeImageMode::Wrap
            && x < bbox[0] + bbox[2] + margin
            && x + width > bbox[0] - margin
        {
            x = bbox[0] + bbox[2] + margin;
        }
        if x + width * scale > layout.width_px() - layout.margin_right_px() {
            return None;
        }
        text.push(NativeImageText {
            fragment,
            x,
            baseline,
            width: width * scale,
            scale,
        });
        x += width * scale;
    }
    Some(NativeImageProjection {
        binding,
        bbox,
        data_uri,
        font_size,
        font_family: font_family?,
        text,
    })
}

pub(crate) fn native_image_text_units(projection: &NativeImageProjection) -> Vec<usize> {
    projection
        .text
        .iter()
        .filter_map(|t| {
            t.fragment
                .source_span
                .as_ref()
                .map(TextSourceSpan::unit_start)
        })
        .collect()
}

pub(crate) fn push_native_image_svg(svg: &mut String, projection: &NativeImageProjection) {
    let font_family = &projection.font_family;
    let [x, y, width, height] = projection.bbox;
    let image = format!(
        "<image class=\"rjtd-native-image\" data-mode-candidate=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" x=\"{x:.3}\" y=\"{y:.3}\" width=\"{width:.3}\" height=\"{height:.3}\" preserveAspectRatio=\"none\" href=\"{}\" xlink:href=\"{}\"/>",
        projection.binding.mode.as_str(),
        projection.data_uri,
        projection.data_uri
    );
    if projection.binding.mode != NativeImageMode::Front {
        svg.push_str(&image);
    }
    for text in &projection.text {
        svg.push_str(&format!("<g transform=\"matrix({:.6} 0 0 1 {:.6} 0)\" data-inline-cache-width-candidate=\"{}\">", text.scale, text.x * (1.0 - text.scale), text.scale != 1.0));
        let style = DocumentTextCharacterStyle {
            source_unit_start: text
                .fragment
                .source_span
                .as_ref()
                .map(TextSourceSpan::unit_start),
            ..DocumentTextCharacterStyle::default()
        };
        push_svg_text_run(
            svg,
            "rjtd-native-image-text",
            text.x,
            text.baseline,
            font_family,
            projection.font_size,
            "#111111",
            &text.fragment.text,
            None,
            Some(&style),
            None,
        );
        svg.push_str("</g>");
    }
    if projection.binding.mode == NativeImageMode::Front {
        svg.push_str(&image);
    }
}

pub(crate) fn push_native_image_layer_json(
    output: &mut String,
    sources: &mut Vec<String>,
    projection: &NativeImageProjection,
) {
    let font_family = &projection.font_family;
    let [x, y, width, height] = projection.bbox;
    let image = format!(
        ",{{\"type\":\"image\",\"bbox\":{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}},\"dataUri\":{},\"modeCandidate\":{},\"source\":\"DocumentText+Frame+singleEmbeddingContents\",\"objectCandidateIndex\":{},\"payloadIndex\":{},\"decoded\":false,\"geometryDecoded\":false,\"paintOrderDecoded\":false,\"referenceBacked\":false}}",
        json_string(&projection.data_uri),
        json_string(projection.binding.mode.as_str()),
        projection.binding.image_index,
        projection.binding.payload_index
    );
    if projection.binding.mode != NativeImageMode::Front {
        output.push_str(&image);
    }
    for text in &projection.text {
        let source = sources.len();
        output.push_str(&format!(",{{\"type\":\"textRun\",\"bbox\":{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}},\"text\":{},\"baseline\":{:.3},\"fontSize\":{:.3},\"fontFamily\":{},\"rotation\":0,\"isVertical\":false,\"inlineCacheWidthScaleCandidate\":{:.6},\"projectionKind\":\"nativeImageTextCandidate\",\"decoded\":false,\"geometryDecoded\":false,\"positionsDecoded\":false,\"source\":",text.x,text.baseline-projection.font_size,text.width,projection.font_size,json_string(&text.fragment.text),text.baseline,projection.font_size,json_string(font_family),text.scale));
        push_page_layer_source_span_json(output, source, &text.fragment);
        output.push('}');
        push_page_layer_text_source_json(sources, source, &text.fragment);
    }
    if projection.binding.mode == NativeImageMode::Front {
        output.push_str(&image);
    }
}
