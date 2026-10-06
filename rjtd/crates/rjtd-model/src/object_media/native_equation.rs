use crate::*;

const CONTENTS: &str = "/EmbedItems/Embedding 1/JSEQ3Contents";
const SNAPSHOT: &str = "/EmbedItems/Embedding 1/\x03EmbeddedPress";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeEquationGlyphCandidate {
    text: String,
    x: u32,
    y: u32,
    font_size: u32,
    family: String,
    italic: bool,
    byte_start: usize,
    byte_end: usize,
    contents_offset: usize,
}

impl NativeEquationGlyphCandidate {
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn position_mm100(&self) -> [u32; 2] {
        [self.x, self.y]
    }
    pub fn font_size_mm100(&self) -> u32 {
        self.font_size
    }
    pub fn family(&self) -> &str {
        &self.family
    }
    pub fn italic(&self) -> bool {
        self.italic
    }
    pub fn snapshot_range(&self) -> [usize; 2] {
        [self.byte_start, self.byte_end]
    }
    pub fn contents_offset(&self) -> usize {
        self.contents_offset
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeEquationCandidate {
    frame: [u16; 4],
    line: usize,
    record_span: TextSourceSpan,
    glyphs: Vec<NativeEquationGlyphCandidate>,
}

impl NativeEquationCandidate {
    pub fn frame_mm100(&self) -> [u16; 4] {
        self.frame
    }
    pub fn line_mark_index(&self) -> usize {
        self.line
    }
    pub fn record_span(&self) -> &TextSourceSpan {
        &self.record_span
    }
    pub fn glyphs(&self) -> &[NativeEquationGlyphCandidate] {
        &self.glyphs
    }
}

fn raw<'a>(document: &'a Document, name: &str) -> Option<&'a [u8]> {
    let mut streams = document.raw_streams().iter().filter(|s| s.name() == name);
    let bytes = streams.next()?.bytes();
    streams.next().is_none().then_some(bytes)
}

/// Font and one-character TextOut packets in the controlled GCI profile.
/// TextOut uses the saved top reference; font/vertical metrics remain candidates.
fn snapshot_glyphs(
    bytes: &[u8],
    width: u32,
    height: u32,
) -> Option<Vec<NativeEquationGlyphCandidate>> {
    if bytes.len() > 64 * 1024
        || bytes.get(..12)? != b"JSSnapShot32"
        || bytes.get(0x2c..0x30)? != b"GCI\0"
        || read_le32_at(bytes, 0x48)? != width
        || read_le32_at(bytes, 0x4c)? != height
    {
        return None;
    }
    let mut offset = 0x80;
    let mut font = None;
    let mut selected = false;
    let mut result = Vec::new();
    let mut stage = 0;
    while offset < bytes.len() {
        let n = usize::try_from(read_le32_at(bytes, offset)?).ok()?;
        if n < 8 || n % 4 != 0 {
            return None;
        }
        let end = offset.checked_add(n)?;
        let record = bytes.get(offset..end)?;
        let kind = read_le32_at(record, 4)?;
        let p = &record[8..];
        match (kind, n) {
            (0x3c, 12) if result.is_empty() && offset == 0x80 => {
                if read_le32_at(p, 0)? != 0xcc {
                    return None;
                }
            }
            (0x24, 20) if offset == 0x8c => {
                if p != [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0] {
                    return None;
                }
            }
            (0x94, 244) if stage == 0 => {
                if read_le32_at(p, 0)? != 228
                    || read_le32_at(p, 4)? != 16
                    || read_le32_at(p, 8)? != 0
                    || read_le32_at(p, 12)? != 0xffffff
                    || read_le32_at(p, 20)? != 36
                    || read_le32_at(p, 24)? != 140
                    || read_le32_at(p, 28)? != 172
                    || read_le32_at(p, 60)? != 400
                    || !matches!(p[64], 0 | 255)
                    || p.get(65..68)? != [0, 0, 0]
                    || read_le32_at(p, 180)? != 56
                    || read_le32_at(p, 184)? != 0
                    || read_le32_at(p, 192)? != 1024
                    || read_le32_at(p, 196)? != 1024
                {
                    return None;
                }
                let family = decode_utf16le_c_string(p.get(72..136)?)?;
                if family != "Times New Roman" {
                    return None;
                }
                let size = read_le32_at(p, 188)?;
                if !(100..=1000).contains(&size) {
                    return None;
                }
                font = Some((family, size, p[64] == 255));
                selected = false;
                stage = 1;
            }
            (0x60, 16) => {
                let id = read_le32_at(p, 0)?;
                if read_le32_at(p, 4)? != 0 || !matches!(id, 3 | 16) {
                    return None;
                }
                if !((stage == 1 && id == 16) || (stage == 4 && id == 3)) {
                    return None;
                }
                selected = id == 16;
                stage = if selected { 2 } else { 5 };
            }
            (0x40, 12) if stage == 2 => {
                if read_le32_at(p, 0)? != 1 {
                    return None;
                }
                stage = 3;
            }
            (0xc8, 48) if stage == 3 => {
                if !selected
                    || read_le32_at(p, 8)? != 0
                    || read_le32_at(p, 12)? != 0
                    || read_le32_at(p, 16)? != 4
                    || read_le32_at(p, 20)? != 1
                    || p.get(24..36)?.iter().any(|b| *b != 0)
                {
                    return None;
                }
                let (family, size, italic) = font.as_ref()?;
                let ch = char::from_u32(read_le32_at(p, 36)?)?;
                if !(ch.is_ascii_alphanumeric() || matches!(ch, '+' | '-' | '=' | '(' | ')')) {
                    return None;
                }
                let x = read_le32_at(p, 0)?;
                let y = read_le32_at(p, 4)?;
                if x >= width || y >= height {
                    return None;
                }
                result.push(NativeEquationGlyphCandidate {
                    text: ch.to_string(),
                    x,
                    y,
                    font_size: *size,
                    family: family.clone(),
                    italic: *italic,
                    byte_start: offset,
                    byte_end: end,
                    contents_offset: 0,
                });
                if result.len() > 32 {
                    return None;
                }
                stage = 4;
            }
            (0x65, 12) if stage == 5 => {
                if read_le32_at(p, 0)? != 16 {
                    return None;
                }
                font = None;
                selected = false;
                stage = 0;
            }
            _ => return None,
        }
        offset = end;
    }
    (!result.is_empty() && font.is_none() && !selected && stage == 0).then_some(result)
}

impl Document {
    pub fn equation_candidates(&self) -> Vec<NativeEquationCandidate> {
        native_equation_binding(self).into_iter().collect()
    }
}

fn native_equation_binding(document: &Document) -> Option<NativeEquationCandidate> {
    let [frame] = document.object_frame_records() else {
        return None;
    };
    let [embedding] = document.object_embedding_frames() else {
        return None;
    };
    if frame.source_path() != "/Frame"
        || frame.object_id() != 0
        || frame.object_type() != 1
        || embedding.class_name() != "JSEQ.Document.3"
        || embedding.embedding_index() != 1
        || embedding.frame_ref() != 0
        || embedding.frame_width() != u32::from(frame.width())
        || embedding.frame_height() != u32::from(frame.height())
        || frame.x() != 0
        || frame.y() != 0
    {
        return None;
    }
    let row = frame.raw_bytes();
    let profile = [
        0x102_u16, 56, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 1, 0, 0, 4,
        0x300, 0, 0,
    ];
    if row.len() != 60
        || row
            .as_chunks::<2>()
            .0
            .iter()
            .zip(profile)
            .enumerate()
            .any(|(i, (w, v))| !matches!(i, 14 | 16 | 18 | 20) && u16::from_be_bytes(*w) != v)
    {
        return None;
    }
    let contents = raw(document, CONTENTS)?;
    if contents.len() > 64 * 1024 || contents.get(..16)? != JSEQ3_CONTENTS_MAGIC_UTF16LE {
        return None;
    }
    let snapshot = raw(document, SNAPSHOT)?;
    let mut glyphs = snapshot_glyphs(
        snapshot,
        u32::from(frame.width()),
        u32::from(frame.height()),
    )?;
    // Same character/color packets in the editable formula corroborate cached glyphs.
    let mut cursor = 0x80;
    for glyph in &mut glyphs {
        let ch = glyph.text.chars().next()? as u32;
        let mut packet = ch.to_le_bytes().to_vec();
        packet.extend([0, 0, 0, 0, 255, 255, 255, 0]);
        let offset = contents
            .get(cursor..)?
            .windows(packet.len())
            .position(|w| w == packet)?
            + cursor;
        if offset % 2 != 0 {
            return None;
        }
        glyph.contents_offset = offset;
        cursor = offset + packet.len();
    }
    let flow = document.document_text_flow()?;
    let events = flow.events();
    let [before, record, start, prefix, value, suffix, after] = events else {
        return None;
    };
    if before.kind() != DocumentTextFlowKind::Text
        || after.kind() != DocumentTextFlowKind::Text
        || !before.text().ends_with('\n')
        || !after.text().starts_with('\n')
        || before.text().chars().filter(|c| *c == '\n').count() != 1
        || after.text().chars().filter(|c| *c == '\n').count() != 1
        || record.raw_words()
            != [
                0x1c, 0, 14, 0, 0x30, 0xffff, 0x507, 0x12, 0, 0, 14, 0, 0, 0x1f,
            ]
        || start.kind() != DocumentTextFlowKind::Control
        || start.code() != Some(0x1c)
        || prefix.raw_words() != [1, 7, 0, 0, 1]
        || value.kind() != DocumentTextFlowKind::Inline
        || value.selector() != Some(1)
        || !value.text().is_empty()
        || suffix.raw_words() != [5, 0, 1, 0x1f]
        || !events
            .windows(2)
            .all(|p| p[0].unit_end() == p[1].unit_start())
    {
        return None;
    }
    let source = document_text_raw_stream(document)?;
    if source.get(value.source_span().byte_start()..value.source_span().byte_end())?
        != [0, 0x1d, 0, 2, 0, 0x1e]
    {
        return None;
    }
    let intervals = shanai_lan_line_mark_intervals(document);
    let [first, middle, last] = intervals.as_slice() else {
        return None;
    };
    if first.unit_start != flow.source_span().unit_start()
        || middle.unit_start != record.unit_start()
        || middle.flag_word != 0x8002
        || last.unit_start != suffix.unit_end() + 1
        || last.unit_end != flow.source_span().unit_end() + 1
    {
        return None;
    }
    Some(NativeEquationCandidate {
        frame: [frame.x(), frame.y(), frame.width(), frame.height()],
        line: middle.record_index,
        record_span: record.source_span().clone(),
        glyphs,
    })
}

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
