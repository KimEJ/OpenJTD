use super::source::*;
use crate::*;

pub(crate) fn native_paragraph_line_x(
    document: &Document,
    layout: PageLayout,
    line: &PageTextLine,
) -> Option<f32> {
    line.native_line_mark_index?;
    let attrs = native_paragraph_attrs_for_index(document, line.paragraph_index()?);
    let indent = if line.char_start() == 0 {
        attrs.first_mm100
    } else {
        attrs.continuing_mm100
    };
    let x = layout.margin_left_px() + hundredth_millimeters_to_css_px(u32::from(indent));
    (x < layout.width_px() - layout.margin_right_px()).then_some(x)
}

pub(crate) fn native_paragraph_after_space(
    document: &Document,
    from: usize,
    to: usize,
) -> Option<f32> {
    let font = document_default_font_size_px(document)?;
    let mut extra = 0.0;
    for (index, _) in document
        .blocks()
        .iter()
        .filter(|block| matches!(block, Block::Paragraph(_)))
        .enumerate()
    {
        let (_, end) = native_paragraph_source_bounds(document, index)?;
        if from < end && end <= to {
            extra += font
                * f32::from(native_paragraph_attrs_for_index(document, index).after_permille)
                / 1000.0;
        }
    }
    Some(extra)
}
