use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeImageMode {
    Inline,
    Wrap,
    Front,
}

impl NativeImageMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Wrap => "wrap",
            Self::Front => "front",
        }
    }
}

/// Single-slot PNG/frame association in the controlled horizontal profile.
/// Other associations and alignment fields retain raw/candidate status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentImageFrameCandidate {
    mode: NativeImageMode,
    record_span: TextSourceSpan,
    cache_end: usize,
    image_index: usize,
    payload_index: usize,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    wrap_margin: u16,
}

impl DocumentImageFrameCandidate {
    pub fn mode(&self) -> NativeImageMode {
        self.mode
    }
    pub fn record_span(&self) -> &TextSourceSpan {
        &self.record_span
    }
    pub fn image_index(&self) -> usize {
        self.image_index
    }
    pub fn payload_index(&self) -> usize {
        self.payload_index
    }
    pub fn geometry_mm100(&self) -> [u16; 4] {
        [self.x, self.y, self.width, self.height]
    }
    pub fn wrap_margin_mm100(&self) -> u16 {
        self.wrap_margin
    }
}

impl Document {
    pub fn image_frame_candidates(&self) -> Vec<DocumentImageFrameCandidate> {
        native_image_binding(self).into_iter().collect()
    }
}

fn native_image_binding(document: &Document) -> Option<DocumentImageFrameCandidate> {
    if !document.object_embedding_frames().is_empty()
        || document.object_stream_candidates().iter().any(|c| {
            !c.fdm_raw_vector_commands().is_empty()
                || c.jseq3_formula_candidate().is_some()
                || c.embedded_press_snapshot_candidate().is_some()
        })
    {
        return None;
    }
    let frames = document
        .object_stream_candidates()
        .iter()
        .filter(|c| c.path() == "/Frame")
        .collect::<Vec<_>>();
    let [source] = frames.as_slice() else {
        return None;
    };
    if source.size() != 76
        || source.payload_prefix() != [0, 1, 0, 4, 0, 2, 0, 1, 1, 1, 0, 4, 0, 0, 0, 1]
    {
        return None;
    }
    let [frame] = document.object_frame_records() else {
        return None;
    };
    if frame.source_path() != "/Frame"
        || frame.row_index() != 0
        || frame.row_start() != 16
        || frame.object_id() != 0
        || frame.object_type() != 1
    {
        return None;
    }
    let bytes = frame.raw_bytes();
    let profile = [
        0x102_u16, 56, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 1, 0, 0, 4,
        0x304, 0, 0,
    ];
    if bytes.len() != 60
        || bytes
            .as_chunks::<2>()
            .0
            .iter()
            .zip(profile)
            .enumerate()
            .any(|(i, (word, expected))| {
                !matches!(i, 8 | 14 | 16 | 18 | 20 | 23) && u16::from_be_bytes(*word) != expected
            })
    {
        return None;
    }
    let mode = match (read_be16_at(bytes, 16)?, read_be16_at(bytes, 46)?) {
        (0, 1) => NativeImageMode::Inline,
        (1, 1) => NativeImageMode::Wrap,
        (1, 4) => NativeImageMode::Front,
        _ => return None,
    };
    let images = document
        .object_stream_candidates()
        .iter()
        .enumerate()
        .flat_map(|(index, c)| {
            c.image_payload_spans()
                .iter()
                .enumerate()
                .map(move |(payload, span)| (index, payload, c, span))
        })
        .collect::<Vec<_>>();
    let [(image_index, payload_index, image, span)] = images.as_slice() else {
        return None;
    };
    if image.path() != "/EmbedItems/Embedding 1/Contents"
        || !span.complete()
        || span.mime() != "image/png"
        || span
            .envelope()
            .declared_payload_length()
            .map(ObjectImageDeclaredLengthCandidate::value)
            != Some(span.len())
    {
        return None;
    }
    let size = span.dimensions()?;
    if size.width() == 0 || size.height() == 0 {
        return None;
    }
    if frame.width() == 0
        || frame.height() == 0
        || frame.y() != 0
        || u64::from(frame.width()) * u64::from(size.height())
            != u64::from(frame.height()) * u64::from(size.width())
    {
        return None;
    }
    let flow = document.document_text_flow()?;
    let records = flow
        .events()
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind() == DocumentTextFlowKind::Record)
        .collect::<Vec<_>>();
    let [(index, record)] = records.as_slice() else {
        return None;
    };
    let context = if mode == NativeImageMode::Inline {
        (0x107, 0x10)
    } else {
        (0x507, 0x12)
    };
    if record.raw_words()
        != [
            0x1c, 0, 14, 0, 0x30, 0xffff, context.0, context.1, 0, 0, 14, 0, 0, 0x1f,
        ]
    {
        return None;
    }
    let group = flow.events().get(index + 1..index + 5)?;
    let [start, prefix, value, suffix] = group else {
        return None;
    };
    if start.kind() != DocumentTextFlowKind::Control
        || start.code() != Some(0x1c)
        || prefix.kind() != DocumentTextFlowKind::Opaque
        || prefix.raw_words() != [1, 7, 0, 0, 1]
        || value.kind() != DocumentTextFlowKind::Inline
        || value.selector() != Some(1)
        || !value.text().is_empty()
        || value.unit_end().checked_sub(value.unit_start())? != 3
        || suffix.kind() != DocumentTextFlowKind::Opaque
        || suffix.raw_words() != [5, 0, 1, 0x1f]
        || record.unit_end() != start.unit_start()
        || !group
            .windows(2)
            .all(|pair| pair[0].unit_end() == pair[1].unit_start())
    {
        return None;
    }
    let raw = document_text_raw_stream(document)?;
    if raw.get(value.source_span().byte_start()..value.source_span().byte_end())?
        != [0, 0x1d, 0, 2, 0, 0x1e]
    {
        return None;
    }
    let text_events = flow
        .events()
        .iter()
        .filter(|e| e.kind() == DocumentTextFlowKind::Text)
        .collect::<Vec<_>>();
    if flow.events().len()
        != if mode == NativeImageMode::Inline {
            7
        } else {
            6
        }
        || text_events.len()
            != if mode == NativeImageMode::Inline {
                2
            } else {
                1
            }
        || text_events.iter().any(|e| {
            e.text().is_empty()
                || e.text().len() > 256
                || !e.text().chars().all(|c| c.is_ascii_graphic() || c == ' ')
        })
    {
        return None;
    }
    if mode == NativeImageMode::Inline {
        if text_events[0].unit_start() != flow.source_span().unit_start()
            || text_events[0].unit_end() != record.unit_start()
            || text_events[1].unit_start() != suffix.unit_end()
        {
            return None;
        }
    } else if record.unit_start() != flow.source_span().unit_start()
        || text_events[0].unit_start() != suffix.unit_end()
    {
        return None;
    }
    Some(DocumentImageFrameCandidate {
        mode,
        record_span: record.source_span().clone(),
        cache_end: suffix.unit_end(),
        image_index: *image_index,
        payload_index: *payload_index,
        x: frame.x(),
        y: frame.y(),
        width: frame.width(),
        height: frame.height(),
        wrap_margin: 200,
    })
}

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
