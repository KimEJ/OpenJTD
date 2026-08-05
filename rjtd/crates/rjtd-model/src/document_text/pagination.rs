use super::*;
use crate::*;

pub(crate) fn paginate_document_text(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Vec<Vec<PageTextLine>> {
    let wrap_columns = layout.wrap_columns(writing_mode);
    let lines_per_page = layout.lines_per_page(writing_mode);
    let forced_breaks = projected_page_breaks(document);
    let mut pages = Vec::new();
    let mut current_page = Vec::new();
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                let text = paragraph_text(paragraph);
                let paragraph_breaks = forced_breaks
                    .get(&paragraph_index)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);
                let wrapped = wrap_text_line(&text, paragraph_index, wrap_columns);
                let mut forced_at_paragraph_end = false;
                if wrapped.is_empty() {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        PageTextLine::new(String::new(), Some(paragraph_index), 0, 0),
                        lines_per_page,
                    );
                    if paragraph_breaks.contains(&0) {
                        force_page_break(&mut pages, &mut current_page);
                        forced_at_paragraph_end = true;
                    }
                } else {
                    for line in wrapped {
                        let segments = split_line_at_page_breaks(line, paragraph_breaks);
                        for segment in segments {
                            push_paginated_line(
                                &mut pages,
                                &mut current_page,
                                segment.line,
                                lines_per_page,
                            );
                            if segment.break_after {
                                force_page_break(&mut pages, &mut current_page);
                                forced_at_paragraph_end = true;
                            } else {
                                forced_at_paragraph_end = false;
                            }
                        }
                    }
                }
                if !forced_at_paragraph_end && !writing_mode.is_vertical() {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        PageTextLine::new(String::new(), None, 0, 0),
                        lines_per_page,
                    );
                }
                paragraph_index += 1;
            }
            Block::Unknown(_) => {
                push_paginated_line(
                    &mut pages,
                    &mut current_page,
                    PageTextLine::new("[UnknownBlock preserved by rjtd]".to_string(), None, 0, 0),
                    lines_per_page,
                );
                push_paginated_line(
                    &mut pages,
                    &mut current_page,
                    PageTextLine::new(String::new(), None, 0, 0),
                    lines_per_page,
                );
            }
        }
    }

    while current_page
        .last()
        .is_some_and(|line| line.text().is_empty() && line.paragraph_index().is_none())
    {
        current_page.pop();
    }

    if !current_page.is_empty() {
        pages.push(current_page);
    }

    if pages.is_empty() {
        if !document.raw_streams().is_empty() {
            let raw_streams = document
                .raw_streams()
                .iter()
                .map(|stream| stream.name())
                .collect::<Vec<_>>()
                .join(", ");
            return vec![vec![PageTextLine::new(
                format!("[rjtd] No extractable text. Preserved raw streams: {raw_streams}"),
                None,
                0,
                0,
            )]];
        }
        return vec![Vec::new()];
    }

    pages
}

pub(crate) fn blank_page_text_line() -> PageTextLine {
    PageTextLine::new(String::new(), None, 0, 0)
}

pub(crate) fn split_page_text_line_by_display_columns(
    line: PageTextLine,
    max_columns: usize,
) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    let mut text = String::new();
    let mut width = 0usize;
    let mut line_start = line.char_start();
    let mut char_offset = line.char_start();

    for character in line.text().chars() {
        let char_width = display_column_width(character);
        if width > 0 && width + char_width > max_columns {
            lines.push(PageTextLine::new(
                std::mem::take(&mut text),
                line.paragraph_index(),
                line_start,
                char_offset,
            ));
            width = 0;
            line_start = char_offset;
        }
        text.push(character);
        width += char_width;
        char_offset += 1;
    }

    if !text.is_empty() {
        lines.push(PageTextLine::new(
            text,
            line.paragraph_index(),
            line_start,
            char_offset,
        ));
    }

    lines
}

pub(crate) fn vertical_page_text_placement(
    layout: PageLayout,
    lines: &[PageTextLine],
) -> VerticalPageTextPlacement {
    if is_ginga_colophon_page(lines) {
        return VerticalPageTextPlacement {
            x_shift_px: -(APP_LINE_HEIGHT_PX * GINGA_COLOPHON_X_SHIFT_COLUMNS),
            y_start_px: (layout.height_px() * GINGA_COLOPHON_TOP_RATIO).max(layout.margin_px()),
        };
    }

    VerticalPageTextPlacement {
        x_shift_px: 0.0,
        y_start_px: layout.margin_px(),
    }
}

pub(crate) fn document_paragraph_texts(document: &Document) -> Vec<(usize, String)> {
    let mut paragraph_index = 0usize;
    let mut paragraphs = Vec::new();
    for block in document.blocks() {
        if let Block::Paragraph(paragraph) = block {
            paragraphs.push((paragraph_index, paragraph_text(paragraph)));
            paragraph_index += 1;
        }
    }
    paragraphs
}

pub(crate) fn document_auto_text_title(document: &Document) -> Option<&str> {
    document
        .auto_texts()
        .iter()
        .map(DocumentAutoText::text)
        .map(str::trim)
        .find(|text| !text.is_empty())
}

pub(crate) fn page_has_exact_text_line(lines: &[PageTextLine], text: &str) -> bool {
    lines.iter().any(|line| line.text().trim() == text)
}

pub(crate) fn wrap_paragraphs_as_single_page(
    paragraphs: &[(usize, String)],
    wrap_columns: usize,
    writing_mode: WritingMode,
) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    for (paragraph_index, text) in paragraphs {
        lines.extend(wrap_text_line(text, *paragraph_index, wrap_columns));
        if !writing_mode.is_vertical() {
            lines.push(PageTextLine::new(String::new(), None, 0, 0));
        }
    }
    trim_trailing_projection_blank_lines(&mut lines);
    lines
}

pub(crate) fn paginate_selected_paragraphs(
    paragraphs: &[(usize, String)],
    layout: PageLayout,
    writing_mode: WritingMode,
    forced_breaks: &BTreeMap<usize, Vec<usize>>,
) -> Vec<Vec<PageTextLine>> {
    let wrap_columns = layout.wrap_columns(writing_mode);
    let lines_per_page = layout.lines_per_page(writing_mode);
    let mut pages = Vec::new();
    let mut current_page = Vec::new();

    for (paragraph_index, text) in paragraphs {
        let paragraph_breaks = forced_breaks
            .get(paragraph_index)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let wrapped = wrap_text_line(text, *paragraph_index, wrap_columns);
        let mut forced_at_paragraph_end = false;
        if wrapped.is_empty() {
            push_paginated_line(
                &mut pages,
                &mut current_page,
                PageTextLine::new(String::new(), Some(*paragraph_index), 0, 0),
                lines_per_page,
            );
            if paragraph_breaks.contains(&0) {
                force_page_break(&mut pages, &mut current_page);
                forced_at_paragraph_end = true;
            }
        } else {
            for line in wrapped {
                let segments = split_line_at_page_breaks(line, paragraph_breaks);
                for segment in segments {
                    push_paginated_line(
                        &mut pages,
                        &mut current_page,
                        segment.line,
                        lines_per_page,
                    );
                    if segment.break_after {
                        force_page_break(&mut pages, &mut current_page);
                        forced_at_paragraph_end = true;
                    } else {
                        forced_at_paragraph_end = false;
                    }
                }
            }
        }
        if !forced_at_paragraph_end && !writing_mode.is_vertical() {
            push_paginated_line(
                &mut pages,
                &mut current_page,
                PageTextLine::new(String::new(), None, 0, 0),
                lines_per_page,
            );
        }
    }

    trim_trailing_projection_blank_lines(&mut current_page);
    if !current_page.is_empty() {
        pages.push(current_page);
    }

    pages
}

pub(crate) fn paragraph_text(paragraph: &Paragraph) -> String {
    let mut text = String::new();

    for inline in paragraph.inlines() {
        match inline {
            Inline::Text(run) => text.push_str(run.text()),
            Inline::Ruby(ruby) => text.push_str(ruby.base_text()),
            Inline::Unknown(_) => {}
        }
    }

    text
}

pub(crate) fn wrap_text_line(
    text: &str,
    paragraph_index: usize,
    max_columns: usize,
) -> Vec<PageTextLine> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut width = 0usize;
    let mut line_start = 0usize;
    let mut char_offset = 0usize;

    for character in text.chars() {
        let char_width = display_column_width(character);
        if width > 0 && width + char_width > max_columns {
            lines.push(PageTextLine::new(
                std::mem::take(&mut line),
                Some(paragraph_index),
                line_start,
                char_offset,
            ));
            width = 0;
            line_start = char_offset;
        }
        line.push(character);
        width += char_width;
        char_offset += 1;
    }

    if !line.is_empty() {
        lines.push(PageTextLine::new(
            line,
            Some(paragraph_index),
            line_start,
            char_offset,
        ));
    }

    lines
}

pub(crate) fn text_display_column_width(text: &str) -> usize {
    text.chars().map(display_column_width).sum()
}

pub(crate) fn nearest_text_line(
    lines: &[PageTextLine],
    target_index: usize,
) -> Option<(usize, &PageTextLine)> {
    if lines.is_empty() {
        return None;
    }

    let target_index = target_index.min(lines.len() - 1);
    if lines[target_index].paragraph_index().is_some() {
        return Some((target_index, &lines[target_index]));
    }

    for distance in 1..lines.len() {
        if let Some(index) = target_index.checked_sub(distance)
            && lines[index].paragraph_index().is_some()
        {
            return Some((index, &lines[index]));
        }

        let index = target_index + distance;
        if index < lines.len() && lines[index].paragraph_index().is_some() {
            return Some((index, &lines[index]));
        }
    }

    None
}

pub(crate) fn paragraph_line_index(
    lines: &[(usize, usize, &PageTextLine)],
    char_offset: usize,
) -> usize {
    let mut last_index = 0usize;

    for (index, (_, _, line)) in lines.iter().enumerate() {
        last_index = index;
        if char_offset <= line.char_end() {
            return index;
        }
    }

    last_index
}

pub(crate) fn text_location_index(
    locations: &[(usize, usize, &PageTextLine)],
    paragraph_index: usize,
    char_offset: usize,
) -> Result<usize> {
    let mut last_index = None;

    for (index, (_, _, line)) in locations.iter().enumerate() {
        if line.paragraph_index() != Some(paragraph_index) {
            continue;
        }

        last_index = Some(index);
        if char_offset <= line.char_end() {
            return Ok(index);
        }
    }

    last_index.ok_or_else(|| {
        rjtd_core::Error::InvalidData(format!("paragraph {paragraph_index} out of range"))
    })
}
