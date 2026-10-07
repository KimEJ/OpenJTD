use super::source::*;
use crate::*;

/// Literal paragraph rows in the corroborated no-layout-mark note profile.
/// The known 600 line-gap profile retains a 60% gap; general units stay unproven.
pub(crate) fn linked_footnote_body_line_top(
    document: &Document,
    layout: PageLayout,
    page: usize,
    line: &PageTextLine,
) -> Option<f32> {
    if page != 1
        || !layout.has_source_margins()
        || !document.page_marks().is_empty()
        || document
            .raw_streams()
            .iter()
            .any(|s| s.name() == "/LineMark")
        || document
            .unknown_styles()
            .iter()
            .any(|s| s.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        || !document.table_candidates().is_empty()
        || !document.object_frame_records().is_empty()
        || !document.object_stream_candidates().is_empty()
        || document.english_justification_candidate() != Some(true)
    {
        return None;
    }
    let note = linked_footnote_text(document)?;
    let resolver = document_text_style_resolver(document)?;
    let span = native_visible_text_span(document, note.marker(), note.body_marker_span())?;
    linked_footnote_marker_script_basis(document, &resolver, &span)?;
    let font = document_default_font_size_px(document)?;
    if (font - hundredth_millimeters_to_css_px(370)).abs() > 0.01 {
        return None;
    }
    let flow = document.document_text_flow()?;
    if [2, 3, 4, 5].into_iter().any(|id| {
        resolver.uniform_optional_value_in_range(
            flow.source_span().unit_start(),
            flow.source_span().unit_end(),
            id,
        ) != Some(None)
    }) {
        return None;
    }
    let mut paragraphs = Vec::new();
    for block in document.blocks() {
        let Block::Paragraph(paragraph) = block else {
            return None;
        };
        if paragraph
            .inlines()
            .iter()
            .any(|inline| matches!(inline, Inline::Unknown(_)))
        {
            return None;
        }
        let text = paragraph_text(paragraph);
        if text.is_empty()
            || text.contains(['\n', '\r'])
            || text_width_px_for_font_size(font, &text) as f32
                > layout.width_px() - layout.margin_left_px() - layout.margin_right_px()
        {
            return None;
        }
        paragraphs.push(text);
    }
    if paragraphs.len() != 2 {
        return None;
    }
    let literal = flow
        .events()
        .iter()
        .filter(|event| {
            event.kind() == DocumentTextFlowKind::Text
                || (event.kind() == DocumentTextFlowKind::Inline
                    && matches!(event.selector(), Some(1 | 3)))
        })
        .map(|e| e.text())
        .collect::<String>();
    if literal != paragraphs.join("\n")
        || flow.events().iter().any(|event| {
            (event.kind() == DocumentTextFlowKind::Control && event.code() == Some(12))
                || (event.kind() == DocumentTextFlowKind::Record
                    && !note_record(event, 0x10, 0)
                    && event.raw_words() != [0x1c, 0, 12, 0, 5, 0, 517, 512, 12, 0, 0, 0x1f])
        })
    {
        return None;
    }
    let index = line.paragraph_index()?;
    if line.char_start() != 0 || line.text() != paragraphs.get(index)? {
        return None;
    }
    let top = layout.margin_top_px() + index as f32 * font * 1.6;
    (top + font * 1.6 <= layout.height_px() - layout.margin_bottom_px()).then_some(top)
}
