use super::*;
use crate::*;

pub(crate) fn page_text_line_fragments(
    document: &Document,
    line: &PageTextLine,
) -> Vec<PageLayerTextFragment> {
    let Some(paragraph_index) = line.paragraph_index() else {
        return vec![PageLayerTextFragment {
            text: line.text().to_string(),
            paragraph_index: None,
            char_start: line.char_start(),
            char_end: line.char_end(),
            source_span: None,
            ruby_annotation: None,
        }];
    };

    let Some(paragraph) = paragraph_by_index(document, paragraph_index) else {
        return Vec::new();
    };
    let mut fragments = paragraph_line_fragments(
        paragraph,
        paragraph_index,
        line.char_start(),
        line.char_end(),
    );
    let source_text = fragments
        .iter()
        .map(|fragment| fragment.text.as_str())
        .collect::<String>();
    if !source_text.is_empty() && line.text().starts_with(&source_text) {
        let source_len = source_text.chars().count();
        let suffix = line.text().chars().skip(source_len).collect::<String>();
        if !suffix.is_empty() {
            let suffix_len = suffix.chars().count();
            fragments.push(PageLayerTextFragment {
                text: suffix,
                paragraph_index: None,
                char_start: line.char_start() + source_len,
                char_end: line.char_start() + source_len + suffix_len,
                source_span: None,
                ruby_annotation: None,
            });
        }
    }
    fragments
}

pub(crate) fn paragraph_by_index(
    document: &Document,
    paragraph_index: usize,
) -> Option<&Paragraph> {
    document
        .blocks()
        .iter()
        .filter_map(|block| match block {
            Block::Paragraph(paragraph) => Some(paragraph),
            Block::Unknown(_) => None,
        })
        .nth(paragraph_index)
}

pub(crate) fn paragraph_line_fragments(
    paragraph: &Paragraph,
    paragraph_index: usize,
    line_start: usize,
    line_end: usize,
) -> Vec<PageLayerTextFragment> {
    let mut fragments = Vec::new();
    let mut paragraph_offset = 0usize;

    for inline in paragraph.inlines() {
        let (text, source_span, ruby_annotation) = match inline {
            Inline::Text(run) => (run.text(), run.source_span(), None),
            Inline::Ruby(ruby) => (ruby.base_text(), None, Some(ruby.annotation_text())),
            Inline::Unknown(_) => ("", None, None),
        };
        let inline_len = text.chars().count();
        let inline_start = paragraph_offset;
        let inline_end = inline_start + inline_len;
        paragraph_offset = inline_end;

        let overlap_start = inline_start.max(line_start);
        let overlap_end = inline_end.min(line_end);
        if overlap_start >= overlap_end {
            continue;
        }

        let relative_start = overlap_start - inline_start;
        let relative_end = overlap_end - inline_start;
        let annotation = if ruby_annotation.is_some()
            && overlap_start == inline_start
            && overlap_end == inline_end
        {
            ruby_annotation.map(str::to_string)
        } else {
            None
        };
        fragments.push(PageLayerTextFragment {
            text: text_by_char_range(text, relative_start, relative_end),
            paragraph_index: Some(paragraph_index),
            char_start: overlap_start,
            char_end: overlap_end,
            source_span: source_span
                .map(|span| source_span_for_char_range(text, span, relative_start, relative_end)),
            ruby_annotation: annotation,
        });
    }

    fragments
}

pub(crate) fn text_by_char_range(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

pub(crate) fn text_width_px(layout: PageLayout, text: &str) -> f64 {
    text.chars()
        .map(|character| display_column_width(character) as f64 * column_width_px(layout))
        .sum()
}

pub(crate) fn text_width_px_for_font_size(font_size: f32, text: &str) -> f64 {
    text.chars()
        .map(|character| display_column_width(character) as f64 * f64::from(font_size) * 0.55)
        .sum()
}

pub(crate) fn vertical_text_advance_px(text: &str) -> f64 {
    text.chars()
        .map(|character| {
            display_column_width(character) as f64 * APP_VERTICAL_DISPLAY_UNIT_PX as f64
        })
        .sum()
}

pub(crate) fn text_positions_px(layout: PageLayout, text: &str) -> Vec<f64> {
    let mut positions = Vec::new();
    let mut x = 0.0;
    positions.push(x);
    for character in text.chars() {
        x += display_column_width(character) as f64 * column_width_px(layout);
        positions.push(x);
    }
    positions
}

pub(crate) fn text_positions_px_for_font_size(font_size: f32, text: &str) -> Vec<f64> {
    let mut positions = Vec::new();
    let mut x = 0.0;
    positions.push(x);
    for character in text.chars() {
        x += display_column_width(character) as f64 * f64::from(font_size) * 0.55;
        positions.push(x);
    }
    positions
}

pub(crate) fn text_positions_px_for_mode(
    layout: PageLayout,
    writing_mode: WritingMode,
    text: &str,
) -> Vec<f64> {
    if !writing_mode.is_vertical() {
        return text_positions_px(layout, text);
    }

    let mut positions = Vec::new();
    let mut y = 0.0;
    positions.push(y);
    for character in text.chars() {
        y += display_column_width(character) as f64 * APP_VERTICAL_DISPLAY_UNIT_PX as f64;
        positions.push(y);
    }
    positions
}

pub(crate) fn fallback_text_fill_color() -> &'static str {
    "#111111"
}

pub(crate) fn document_text_units(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect()
}

pub(crate) fn fallback_text_origin(layout: PageLayout, document: &Document) -> Option<(f32, f32)> {
    if !document_has_shanai_lan_fdm_command_evidence(document) {
        return None;
    }
    let viewport = fdm_projection_viewport(layout);
    Some((viewport.x, viewport.y))
}

pub(crate) fn document_text_raw_stream(document: &Document) -> Option<&[u8]> {
    raw_stream_bytes(document, DOCUMENT_TEXT_PATH)
}

pub(crate) fn text_source_span_from_document_text_units(
    start: usize,
    end: usize,
) -> TextSourceSpan {
    TextSourceSpan::new(start * 2, end * 2, start, end)
}

pub(crate) fn page_frame_text_placement(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    line_index: usize,
    line: &PageTextLine,
) -> Option<PageLayerTextPlacement> {
    let projection = page_frame_projection(document, layout, page_number)?;
    let title_frame = projection
        .shapes
        .iter()
        .find(|shape| shape.role == "titleRoundedFrame");
    let first_bar = projection
        .shapes
        .iter()
        .filter(|shape| shape.role == "horizontalPatternBar")
        .min_by(|left, right| left.y.partial_cmp(&right.y).unwrap_or(Ordering::Equal));

    if line_index == 0 && line.text().contains("制限時間") {
        let title_frame = title_frame?;
        let text_width = text_width_px(layout, line.text()) as f32;
        let x = ((layout.width_px() - text_width) / 2.0).max(layout.margin_px() * 0.5);
        let y = title_frame.y + title_frame.height + PAGE_FRAME_TIME_CAPTION_GAP_PX;
        return Some(PageLayerTextPlacement {
            x: x as f64,
            y: y as f64,
            baseline: (y + APP_FONT_SIZE_PX) as f64,
        });
    }

    if line_index == 0 {
        return None;
    }

    let first_bar = first_bar?;
    let body_x = layout_box_body_anchor_from_document(document, layout)
        .map(|(x, _, _)| x)
        .unwrap_or_else(|| layout.margin_px());
    let y = first_bar.y
        + first_bar.height
        + (APP_LINE_HEIGHT_PX * PAGE_FRAME_TEXT_AFTER_BAR_GAP_LINES)
        + (line_index.saturating_sub(1) as f32 * APP_LINE_HEIGHT_PX);

    Some(PageLayerTextPlacement {
        x: body_x as f64,
        y: y as f64,
        baseline: (y + APP_FONT_SIZE_PX) as f64,
    })
}
