use super::source::{native_section_source, native_section_spacing_source};
use crate::*;

/// Bounded single explicit style, apply/reset and portrait/landscape/portrait pages.
/// Other page-style associations retain the global fallback.
pub(crate) fn native_section_layouts(
    document: &Document,
    base: PageLayout,
) -> Option<Vec<PageLayout>> {
    let source = native_section_source(document)?;
    let custom = page_layout_from_size_mm100(source.custom);
    let default = page_layout_from_size_mm100(source.default);
    if (default.width_px() - custom.height_px()).abs() > 0.1
        || (default.height_px() - custom.width_px()).abs() > 0.1
        || default.width_px() >= default.height_px()
    {
        return None;
    }
    let margins = source
        .margins
        .map(|value| hundredth_millimeters_to_css_px(u32::from(value)));
    Some(
        source
            .custom_style_pages
            .into_iter()
            .map(|landscape| {
                let size = if landscape { custom } else { default };
                PageLayout {
                    width_px: size.width_px(),
                    height_px: size.height_px(),
                    landscape,
                    source_margins: Some(margins),
                    ..base
                }
            })
            .collect(),
    )
}

pub(crate) fn native_section_spacing_candidate(
    document: &Document,
    layout: PageLayout,
    page: usize,
) -> Option<u16> {
    if page != 2
        || modern_view_writing_mode(document.unknown_styles()) != Some(WritingMode::Horizontal)
    {
        return None;
    }
    let layouts = native_section_layouts(document, layout)?;
    let selected = layouts.get(page - 1)?;
    if (selected.width_px() - layout.width_px()).abs() > 0.1
        || (selected.height_px() - layout.height_px()).abs() > 0.1
        || (selected.margin_left_px() - layout.margin_left_px()).abs() > 0.1
        || (selected.margin_right_px() - layout.margin_right_px()).abs() > 0.1
        || (selected.margin_top_px() - layout.margin_top_px()).abs() > 0.1
        || (selected.margin_bottom_px() - layout.margin_bottom_px()).abs() > 0.1
    {
        return None;
    }
    let (font_mm100, spacing) = native_section_spacing_source(document)?;
    if (hundredth_millimeters_to_css_px(u32::from(font_mm100))
        - document_default_font_size_px(document)?)
    .abs()
        > 0.01
    {
        return None;
    }
    Some(spacing)
}

impl DocumentCore {
    pub(crate) fn page_layout_for(&self, page: usize) -> PageLayout {
        native_section_layouts(&self.document, self.page_layout)
            .and_then(|layouts| layouts.get(page).copied())
            .unwrap_or(self.page_layout)
    }

    pub fn page_size_px(&self, page_num: u32) -> Result<(f64, f64)> {
        self.page_lines(page_num)?;
        let layout = self.page_layout_for(page_num as usize);
        Ok((f64::from(layout.width_px()), f64::from(layout.height_px())))
    }
}
