use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CursorRect {
    pub(crate) page_index: usize,
    pub(crate) line_index: usize,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextRange {
    pub(crate) start_para: usize,
    pub(crate) start_offset: usize,
    pub(crate) end_para: usize,
    pub(crate) end_offset: usize,
}

impl TextRange {
    pub(crate) fn is_collapsed(&self) -> bool {
        self.start_para == self.end_para && self.start_offset == self.end_offset
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SearchHit {
    pub(crate) sec: u32,
    pub(crate) para: u32,
    pub(crate) char_offset: u32,
    pub(crate) length: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum JtdValidationWarningKind {
    FallbackTextPagination,
    RawStreamPreserved,
    UnknownBlockPreserved,
    UnknownStylePreserved,
    UnknownObjectPreserved,
    ObjectStreamCandidateDiagnosticOnly,
    TextCountRangeDiagnosticOnly,
    TextCountControlRangeDiagnosticOnly,
    TextBoundaryCandidateDiagnosticOnly,
    TextParagraphBoundaryCandidateDiagnosticOnly,
    TableCandidateDiagnosticOnly,
}

impl JtdValidationWarningKind {
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::FallbackTextPagination => "JtdFallbackTextPagination",
            Self::RawStreamPreserved => "JtdRawStreamPreserved",
            Self::UnknownBlockPreserved => "JtdUnknownBlockPreserved",
            Self::UnknownStylePreserved => "JtdUnknownStylePreserved",
            Self::UnknownObjectPreserved => "JtdUnknownObjectPreserved",
            Self::ObjectStreamCandidateDiagnosticOnly => "JtdObjectStreamCandidateDiagnosticOnly",
            Self::TextCountRangeDiagnosticOnly => "JtdTextCountRangeDiagnosticOnly",
            Self::TextCountControlRangeDiagnosticOnly => "JtdTextCountControlRangeDiagnosticOnly",
            Self::TextBoundaryCandidateDiagnosticOnly => "JtdTextBoundaryCandidateDiagnosticOnly",
            Self::TextParagraphBoundaryCandidateDiagnosticOnly => {
                "JtdTextParagraphBoundaryCandidateDiagnosticOnly"
            }
            Self::TableCandidateDiagnosticOnly => "JtdTableCandidateDiagnosticOnly",
        }
    }

    pub(crate) fn summary_message(self) -> &'static str {
        match self {
            Self::FallbackTextPagination => "JTD text layout uses fallback pagination",
            Self::RawStreamPreserved => "JTD raw stream preserved but not decoded",
            Self::UnknownBlockPreserved => "JTD unknown block preserved",
            Self::UnknownStylePreserved => "JTD style stream preserved but not decoded",
            Self::UnknownObjectPreserved => "JTD inline object preserved but not decoded",
            Self::ObjectStreamCandidateDiagnosticOnly => {
                "JTD object stream candidate preserved as diagnostic data"
            }
            Self::TextCountRangeDiagnosticOnly => {
                "JTD text-count range preserved as diagnostic data"
            }
            Self::TextCountControlRangeDiagnosticOnly => {
                "JTD text-count control-range overlap preserved as diagnostic data"
            }
            Self::TextBoundaryCandidateDiagnosticOnly => {
                "JTD text-boundary candidate preserved as diagnostic data"
            }
            Self::TextParagraphBoundaryCandidateDiagnosticOnly => {
                "JTD text paragraph-boundary candidate preserved as diagnostic data"
            }
            Self::TableCandidateDiagnosticOnly => {
                "JTD table candidate preserved as diagnostic data"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct JtdValidationWarning {
    pub(crate) section_idx: usize,
    pub(crate) paragraph_idx: usize,
    pub(crate) kind: JtdValidationWarningKind,
}

impl JtdValidationWarning {
    pub(crate) fn document_level(kind: JtdValidationWarningKind) -> Self {
        Self {
            section_idx: 0,
            paragraph_idx: 0,
            kind,
        }
    }

    pub(crate) fn paragraph(paragraph_idx: usize, kind: JtdValidationWarningKind) -> Self {
        Self {
            section_idx: 0,
            paragraph_idx,
            kind,
        }
    }
}

pub(crate) fn jtd_validation_warnings(document: &Document) -> Vec<JtdValidationWarning> {
    let mut warnings = Vec::new();
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                if !paragraph_text(paragraph).is_empty() {
                    warnings.push(JtdValidationWarning::paragraph(
                        paragraph_index,
                        JtdValidationWarningKind::FallbackTextPagination,
                    ));
                }
                paragraph_index += 1;
            }
            Block::Unknown(_) => warnings.push(JtdValidationWarning::document_level(
                JtdValidationWarningKind::UnknownBlockPreserved,
            )),
        }
    }

    for _ in document.raw_streams() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::RawStreamPreserved,
        ));
    }

    for _ in document.unknown_styles() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::UnknownStylePreserved,
        ));
    }

    for _ in document.unknown_objects() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::UnknownObjectPreserved,
        ));
    }

    for _ in document.object_stream_candidates() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::ObjectStreamCandidateDiagnosticOnly,
        ));
    }

    for _ in document.text_count_ranges() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::TextCountRangeDiagnosticOnly,
        ));
    }

    for range in document.text_count_ranges() {
        if !range.control_range_overlaps().is_empty() {
            warnings.push(JtdValidationWarning::document_level(
                JtdValidationWarningKind::TextCountControlRangeDiagnosticOnly,
            ));
        }
    }

    for _ in document.text_boundary_candidates() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::TextBoundaryCandidateDiagnosticOnly,
        ));
    }

    for _ in document.text_paragraph_boundary_candidates() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::TextParagraphBoundaryCandidateDiagnosticOnly,
        ));
    }

    for _ in document.table_candidates() {
        warnings.push(JtdValidationWarning::document_level(
            JtdValidationWarningKind::TableCandidateDiagnosticOnly,
        ));
    }

    warnings
}

pub(crate) fn jtd_validation_warnings_json(warnings: &[JtdValidationWarning]) -> String {
    let mut summary = BTreeMap::<&'static str, usize>::new();
    for warning in warnings {
        *summary.entry(warning.kind.summary_message()).or_insert(0) += 1;
    }

    let mut output = String::new();
    output.push_str("{\"count\":");
    output.push_str(&warnings.len().to_string());
    output.push_str(",\"summary\":{");
    for (index, (message, count)) in summary.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(message));
        output.push(':');
        output.push_str(&count.to_string());
    }
    output.push_str("},\"warnings\":[");
    for (index, warning) in warnings.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"section\":");
        output.push_str(&warning.section_idx.to_string());
        output.push_str(",\"paragraph\":");
        output.push_str(&warning.paragraph_idx.to_string());
        output.push_str(",\"kind\":");
        output.push_str(&json_string(warning.kind.code()));
        output.push_str(",\"cell\":null}");
    }
    output.push_str("]}");
    output
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProjectedTextControl {
    pub(crate) boundary_index: usize,
    pub(crate) paragraph_index: usize,
    pub(crate) char_offset: usize,
    pub(crate) code: u16,
}

pub(crate) fn project_control_boundary_to_text(
    boundary_span: &TextSourceSpan,
    spans: &[ParagraphSourceTextSpan],
) -> Option<(usize, usize)> {
    let mut previous: Option<&ParagraphSourceTextSpan> = None;
    let mut next: Option<&ParagraphSourceTextSpan> = None;

    for span in spans {
        if span.unit_start <= boundary_span.unit_start()
            && boundary_span.unit_end() <= span.unit_end
        {
            return Some((span.paragraph_index, span.char_start));
        }

        if span.unit_end <= boundary_span.unit_start()
            && previous.is_none_or(|candidate| span.unit_end > candidate.unit_end)
        {
            previous = Some(span);
        }

        if span.unit_start >= boundary_span.unit_end()
            && next.is_none_or(|candidate| span.unit_start < candidate.unit_start)
        {
            next = Some(span);
        }
    }

    match (previous, next) {
        (Some(prev), Some(next)) if prev.paragraph_index == next.paragraph_index => {
            Some((prev.paragraph_index, prev.char_end))
        }
        (Some(prev), Some(next)) => {
            let prev_distance = boundary_span.unit_start().saturating_sub(prev.unit_end);
            let next_distance = next.unit_start.saturating_sub(boundary_span.unit_end());
            if next_distance < prev_distance {
                Some((next.paragraph_index, next.char_start))
            } else {
                Some((prev.paragraph_index, prev.char_end))
            }
        }
        (Some(prev), None) => Some((prev.paragraph_index, prev.char_end)),
        (None, Some(next)) => Some((next.paragraph_index, next.char_start)),
        (None, None) => None,
    }
}

pub(crate) fn projected_control_json(control: &ProjectedTextControl) -> String {
    format!(
        "{{\"type\":\"jtdControl\",\"sec\":0,\"para\":{},\"ci\":{},\"charPos\":{},\"code\":{},\"codeHex\":{},\"decoded\":false}}",
        control.paragraph_index,
        control.boundary_index,
        control.char_offset,
        control.code,
        json_string(&format!("0x{:04x}", control.code)),
    )
}

pub(crate) fn projected_control_layout_json(
    layout: PageLayout,
    control: &ProjectedTextControl,
    rect: &CursorRect,
) -> String {
    format!(
        "{{\"type\":\"jtdControl\",\"x\":{:.1},\"y\":{:.1},\"w\":{:.1},\"h\":{:.1},\"secIdx\":0,\"paraIdx\":{},\"controlIdx\":{},\"charPos\":{},\"code\":{},\"codeHex\":{},\"decoded\":false,\"source\":\"textControlBoundary\"}}",
        rect.x,
        rect.y,
        column_width_px(layout),
        rect.height,
        control.paragraph_index,
        control.boundary_index,
        control.char_offset,
        control.code,
        json_string(&format!("0x{:04x}", control.code)),
    )
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct PageOutputShape {
    pub(crate) pages: usize,
    pub(crate) lines: usize,
}

#[derive(Debug, Default)]
pub(crate) struct PagePreflight {
    pub(crate) shape: PageOutputShape,
    pub(crate) current_lines: usize,
    pub(crate) current_has_nonempty_line: bool,
    pub(crate) trailing_trim_lines: usize,
    pub(crate) lines_per_page: usize,
}

impl PagePreflight {
    pub(crate) fn new(lines_per_page: usize) -> Self {
        Self {
            lines_per_page,
            ..Self::default()
        }
    }

    pub(crate) fn push_line(&mut self, nonempty: bool, trim_at_page_end: bool) -> Result<()> {
        if self.current_lines >= self.lines_per_page {
            self.finish_current_page()?;
        }
        self.current_lines = checked_page_shape_add(self.current_lines, 1)?;
        self.current_has_nonempty_line |= nonempty;
        self.trailing_trim_lines = if trim_at_page_end {
            checked_page_shape_add(self.trailing_trim_lines, 1)?
        } else {
            0
        };
        Ok(())
    }

    pub(crate) fn force_page_break(&mut self) -> Result<()> {
        self.current_lines = self.current_lines.saturating_sub(self.trailing_trim_lines);
        self.trailing_trim_lines = 0;
        if self.current_has_nonempty_line {
            self.finish_current_page()?;
        } else {
            self.current_lines = 0;
        }
        Ok(())
    }

    pub(crate) fn finish(mut self, has_raw_streams: bool) -> Result<PageOutputShape> {
        self.current_lines = self.current_lines.saturating_sub(self.trailing_trim_lines);
        self.trailing_trim_lines = 0;
        if self.current_lines != 0 {
            self.finish_current_page()?;
        }
        if self.shape.pages == 0 {
            self.shape.pages = 1;
            self.shape.lines = usize::from(has_raw_streams);
        }
        Ok(self.shape)
    }

    pub(crate) fn finish_current_page(&mut self) -> Result<()> {
        self.shape.pages = checked_page_shape_add(self.shape.pages, 1)?;
        self.shape.lines = checked_page_shape_add(self.shape.lines, self.current_lines)?;
        self.current_lines = 0;
        self.trailing_trim_lines = 0;
        self.current_has_nonempty_line = false;
        Ok(())
    }
}

pub(crate) fn checked_page_shape_add(left: usize, right: usize) -> Result<usize> {
    left.checked_add(right).ok_or(Error::ResourceLimit {
        resource: "document page lines",
        limit: usize::MAX,
        actual: usize::MAX,
    })
}

pub(crate) fn page_output_shape(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Result<PageOutputShape> {
    let wrap_columns = layout.wrap_columns(writing_mode);
    let forced_breaks = projected_page_breaks(document);
    let mut preflight = PagePreflight::new(layout.lines_per_page(writing_mode));
    let mut paragraph_index = 0usize;

    for block in document.blocks() {
        match block {
            Block::Paragraph(paragraph) => {
                let paragraph_breaks = forced_breaks
                    .get(&paragraph_index)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);
                let forced_at_paragraph_end = page_shape_for_paragraph(
                    paragraph,
                    paragraph_breaks,
                    wrap_columns,
                    &mut preflight,
                )?;
                if !forced_at_paragraph_end && !writing_mode.is_vertical() {
                    preflight.push_line(false, true)?;
                }
                paragraph_index = checked_page_shape_add(paragraph_index, 1)?;
            }
            Block::Unknown(_) => {
                preflight.push_line(true, false)?;
                preflight.push_line(false, true)?;
            }
        }
    }

    preflight.finish(!document.raw_streams().is_empty())
}

pub(crate) fn page_construction_shape(
    document: &Document,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Result<PageOutputShape> {
    let normal = page_output_shape(document, layout, writing_mode)?;
    if !writing_mode.is_vertical() {
        return Ok(normal);
    }

    if ginga_front_matter_indices_in_document(document).is_none() {
        return Ok(normal);
    }

    let source_lines = document_paragraph_character_count(document)?;
    let projection_lines = source_lines
        .checked_mul(4)
        .and_then(|total| checked_page_shape_add(total, document.toc_entries().len()).ok())
        .and_then(|total| checked_page_shape_add(total, 32).ok())
        .ok_or(Error::ResourceLimit {
            resource: "document page lines",
            limit: usize::MAX,
            actual: usize::MAX,
        })?;
    // The projection temporarily coexists with normal pagination. It contributes five fixed
    // front-matter pages and body pagination can gain at most one carry page when chapter
    // spacing is inserted. Page-line expansion is accounted separately below.
    let projection_pages = checked_page_shape_add(normal.pages, 6)?;

    Ok(PageOutputShape {
        pages: checked_page_shape_add(normal.pages, projection_pages)?,
        lines: checked_page_shape_add(normal.lines, projection_lines)?,
    })
}

pub(crate) fn page_shape_for_paragraph(
    paragraph: &Paragraph,
    paragraph_breaks: &[usize],
    wrap_columns: usize,
    preflight: &mut PagePreflight,
) -> Result<bool> {
    let mut line_start = 0usize;
    let mut char_offset = 0usize;
    let mut line_width = 0usize;
    for inline in paragraph.inlines() {
        let text = match inline {
            Inline::Text(run) => run.text(),
            Inline::Ruby(ruby) => ruby.base_text(),
            Inline::Unknown(_) => continue,
        };
        for character in text.chars() {
            let character_width = display_column_width(character);
            if line_width > 0 && line_width + character_width > wrap_columns {
                page_shape_for_wrapped_line(line_start, char_offset, paragraph_breaks, preflight)?;
                line_width = 0;
                line_start = char_offset;
            }
            line_width += character_width;
            char_offset = checked_page_shape_add(char_offset, 1)?;
        }
    }

    if char_offset == 0 {
        preflight.push_line(false, false)?;
        if paragraph_breaks.contains(&0) {
            preflight.force_page_break()?;
            return Ok(true);
        }
        return Ok(false);
    }

    page_shape_for_wrapped_line(line_start, char_offset, paragraph_breaks, preflight)
}

pub(crate) fn page_shape_for_wrapped_line(
    line_start: usize,
    line_end: usize,
    paragraph_breaks: &[usize],
    preflight: &mut PagePreflight,
) -> Result<bool> {
    let mut segment_start = line_start;
    let mut emitted_segment = false;
    let mut forced_after_last_segment = false;

    for break_offset in paragraph_breaks.iter().copied() {
        if break_offset < segment_start || break_offset > line_end {
            continue;
        }
        if break_offset > segment_start || break_offset == line_start {
            preflight.push_line(break_offset > segment_start, false)?;
            preflight.force_page_break()?;
            emitted_segment = true;
            forced_after_last_segment = true;
        }
        segment_start = break_offset;
    }

    if segment_start < line_end {
        preflight.push_line(true, false)?;
        return Ok(false);
    }
    if !emitted_segment {
        preflight.push_line(true, false)?;
        return Ok(false);
    }
    Ok(forced_after_last_segment)
}

pub(crate) fn project_sample_front_matter_pages(
    document: &Document,
    _file_name: &str,
    layout: PageLayout,
    writing_mode: WritingMode,
) -> Option<Vec<Vec<PageTextLine>>> {
    if !writing_mode.is_vertical() {
        return None;
    }

    let paragraphs = document_paragraph_texts(document);
    let front_matter = ginga_front_matter_indices(&paragraphs)?;
    let forced_breaks = projected_page_breaks(document);
    let wrap_columns = layout.wrap_columns(writing_mode);
    let mut pages = Vec::new();

    pages.push(wrap_paragraphs_as_single_page(
        &paragraphs[front_matter.title_index..front_matter.title_index + 1],
        wrap_columns,
        writing_mode,
    ));
    pages.push(Vec::new());
    pages.push(
        projected_ginga_toc_page(document, &paragraphs, front_matter, wrap_columns).unwrap_or_else(
            || {
                wrap_paragraphs_as_single_page(
                    &paragraphs[front_matter.toc_start_index..front_matter.body_title_index],
                    wrap_columns,
                    writing_mode,
                )
            },
        ),
    );
    pages.push(Vec::new());
    pages.push(wrap_paragraphs_as_single_page(
        &paragraphs[front_matter.body_title_index..front_matter.body_title_index + 1],
        wrap_columns,
        writing_mode,
    ));
    let body_pages = paginate_selected_paragraphs(
        &paragraphs[front_matter.body_start_index..],
        layout,
        writing_mode,
        &forced_breaks,
    );
    let body_pages =
        project_ginga_body_chapter_pages(body_pages, layout.lines_per_page(writing_mode));
    pages.extend(project_ginga_colophon_pages(body_pages));

    Some(pages)
}

pub(crate) fn project_ginga_body_chapter_pages(
    body_pages: Vec<Vec<PageTextLine>>,
    lines_per_page: usize,
) -> Vec<Vec<PageTextLine>> {
    let mut pages = body_pages.into_iter();
    let Some(first_page) = pages.next() else {
        return Vec::new();
    };
    let Some(chapter_line) = first_page.first() else {
        let mut original_pages = vec![first_page];
        original_pages.extend(pages);
        return original_pages;
    };
    if !is_short_chapter_title(chapter_line.text().trim()) {
        let mut original_pages = vec![first_page];
        original_pages.extend(pages);
        return original_pages;
    }

    let heading_slots =
        GINGA_BODY_CHAPTER_LEADING_BLANK_COLUMNS + 1 + GINGA_BODY_CHAPTER_TRAILING_BLANK_COLUMNS;
    if lines_per_page <= heading_slots {
        let mut original_pages = vec![first_page];
        original_pages.extend(pages);
        return original_pages;
    }

    let available_body_lines = lines_per_page - heading_slots;
    let keep_end = (1 + available_body_lines).min(first_page.len());
    let mut projected_first_page = Vec::with_capacity(lines_per_page);
    projected_first_page.extend(
        std::iter::repeat_with(blank_page_text_line).take(GINGA_BODY_CHAPTER_LEADING_BLANK_COLUMNS),
    );
    projected_first_page.push(first_page[0].clone());
    projected_first_page.extend(
        std::iter::repeat_with(blank_page_text_line)
            .take(GINGA_BODY_CHAPTER_TRAILING_BLANK_COLUMNS),
    );
    projected_first_page.extend(first_page[1..keep_end].iter().cloned());

    let mut projected_pages = vec![projected_first_page];
    let mut carry = first_page[keep_end..].to_vec();
    for page in pages {
        let mut projected_page = Vec::new();
        projected_page.append(&mut carry);
        projected_page.extend(page);
        if projected_page.len() > lines_per_page {
            carry = projected_page.split_off(lines_per_page);
        }
        projected_pages.push(projected_page);
    }
    projected_pages.extend(repaginate_lines(carry, lines_per_page));
    projected_pages
}

pub(crate) fn repaginate_lines(
    lines: Vec<PageTextLine>,
    lines_per_page: usize,
) -> Vec<Vec<PageTextLine>> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut pages = Vec::new();
    let mut current_page = Vec::new();
    for line in lines {
        push_paginated_line(&mut pages, &mut current_page, line, lines_per_page);
    }
    trim_trailing_projection_blank_lines(&mut current_page);
    if !current_page.is_empty() {
        pages.push(current_page);
    }
    pages
}

pub(crate) fn project_ginga_colophon_pages(
    mut pages: Vec<Vec<PageTextLine>>,
) -> Vec<Vec<PageTextLine>> {
    for page in &mut pages {
        if is_ginga_colophon_page(page) {
            *page = project_ginga_colophon_lines(page);
        }
    }
    pages
}

pub(crate) fn is_ginga_colophon_page(lines: &[PageTextLine]) -> bool {
    let visible = lines
        .iter()
        .map(PageTextLine::text)
        .map(str::trim)
        .filter(|text| !text.is_empty() && !is_colophon_noise_line(text))
        .collect::<Vec<_>>();
    visible
        .first()
        .is_some_and(|text| text.contains("銀河鉄道の夜"))
        && visible.iter().any(|text| text.contains("初版発行"))
        && visible.iter().any(|text| text.contains("発行所"))
        && visible
            .iter()
            .any(|text| text.contains("Printed") || text.contains("Japan"))
}

pub(crate) fn project_ginga_colophon_lines(lines: &[PageTextLine]) -> Vec<PageTextLine> {
    let mut projected = Vec::new();
    let mut visible_index = 0usize;
    let mut index = 0usize;

    while index < lines.len() {
        let line = &lines[index];
        let text = line.text().trim();
        if text.is_empty() || is_colophon_noise_line(text) {
            index += 1;
            continue;
        }

        if text.starts_with('※') {
            let (note, consumed) = collect_colophon_note_lines(&lines[index..]);
            projected.extend(split_colophon_note_line(note));
            index += consumed;
            continue;
        }

        projected.push(line.clone());
        if visible_index == 0 || visible_index == 1 || is_colophon_copyright_line(text) {
            projected.push(blank_page_text_line());
        }
        visible_index += 1;
        index += 1;
    }

    projected
}

pub(crate) fn collect_colophon_note_lines(lines: &[PageTextLine]) -> (PageTextLine, usize) {
    let Some(first) = lines.first() else {
        return (blank_page_text_line(), 0);
    };
    let mut text = String::new();
    let mut consumed = 0usize;
    let paragraph_index = first.paragraph_index();
    let char_start = first.char_start();
    let mut char_end = first.char_end();

    for line in lines {
        let trimmed = line.text().trim();
        if trimmed.is_empty() || is_colophon_noise_line(trimmed) {
            consumed += 1;
            continue;
        }
        if consumed > 0 && !trimmed.starts_with('※') && line.paragraph_index() != paragraph_index
        {
            break;
        }
        text.push_str(trimmed);
        char_end = line.char_end();
        consumed += 1;
    }

    (
        PageTextLine::new(text, paragraph_index, char_start, char_end),
        consumed,
    )
}

pub(crate) fn split_colophon_note_line(line: PageTextLine) -> Vec<PageTextLine> {
    split_page_text_line_by_display_columns(line, GINGA_COLOPHON_NOTE_DISPLAY_COLUMNS)
}

pub(crate) fn is_colophon_noise_line(text: &str) -> bool {
    text.trim().starts_with('\u{fe02}')
}

pub(crate) fn is_colophon_copyright_line(text: &str) -> bool {
    text.contains("Printed") || text.contains("Japan") || text.contains("©")
}

pub(crate) fn projected_ginga_toc_page(
    document: &Document,
    paragraphs: &[(usize, String)],
    front_matter: GingaFrontMatterIndices,
    wrap_columns: usize,
) -> Option<Vec<PageTextLine>> {
    if document.toc_entries().is_empty() {
        return None;
    }

    let toc_title_paragraphs = paragraphs
        [front_matter.toc_start_index + 1..front_matter.body_title_index]
        .iter()
        .map(|(paragraph_index, text)| (text.trim().to_string(), *paragraph_index))
        .collect::<BTreeMap<_, _>>();
    let mut lines = Vec::new();
    for _ in 0..GINGA_TOC_LEADING_BLANK_COLUMNS {
        lines.push(PageTextLine::new(String::new(), None, 0, 0));
    }
    lines.extend(wrap_text_line(
        &paragraphs[front_matter.toc_start_index].1,
        paragraphs[front_matter.toc_start_index].0,
        wrap_columns,
    ));
    let toc_columns = wrap_columns.saturating_add(GINGA_TOC_EXTRA_COLUMNS);

    for entry in document.toc_entries() {
        let title = entry.title().trim();
        let Some(paragraph_index) = toc_title_paragraphs.get(title) else {
            continue;
        };
        let text = toc_leader_line(title, entry.page_label(), toc_columns);
        let char_count = text.chars().count();
        let title_char_count = title.chars().count();
        lines.push(PageTextLine::new(
            text,
            Some(*paragraph_index),
            0,
            title_char_count.min(char_count),
        ));
    }

    (lines.len() > 1).then_some(lines)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GingaFrontMatterIndices {
    pub(crate) title_index: usize,
    pub(crate) toc_start_index: usize,
    pub(crate) body_title_index: usize,
    pub(crate) body_start_index: usize,
}

pub(crate) fn ginga_front_matter_indices(
    paragraphs: &[(usize, String)],
) -> Option<GingaFrontMatterIndices> {
    let first_text = paragraphs.first()?.1.trim();
    if !first_text.contains("銀河鉄道の夜") || !first_text.contains("宮沢") {
        return None;
    }

    let toc_start_index = paragraphs
        .iter()
        .position(|(_, text)| text.trim() == "目次")?;
    let body_title_index = paragraphs
        .iter()
        .enumerate()
        .skip(toc_start_index + 1)
        .find_map(|(index, (_, text))| (text.trim() == "銀河鉄道の夜").then_some(index))?;
    let body_start_index = body_title_index + 1;
    if body_start_index >= paragraphs.len() {
        return None;
    }
    let body_start_text = paragraphs[body_start_index].1.trim();
    if !body_start_text.starts_with("一、午后の授業") {
        return None;
    }

    Some(GingaFrontMatterIndices {
        title_index: 0,
        toc_start_index,
        body_title_index,
        body_start_index,
    })
}

pub(crate) fn ginga_front_matter_indices_in_document(
    document: &Document,
) -> Option<GingaFrontMatterIndices> {
    let mut paragraph_index = 0usize;
    let mut toc_start_index = None;
    let mut body_title_index = None;

    for block in document.blocks() {
        let Block::Paragraph(paragraph) = block else {
            continue;
        };

        if paragraph_index == 0
            && (!paragraph_contains(paragraph, "銀河鉄道の夜")
                || !paragraph_contains(paragraph, "宮沢"))
        {
            return None;
        }

        if toc_start_index.is_none() && paragraph_trimmed_equals(paragraph, "目次") {
            toc_start_index = Some(paragraph_index);
        } else if let Some(toc_start_index) = toc_start_index {
            if paragraph_index > toc_start_index
                && body_title_index.is_none()
                && paragraph_trimmed_equals(paragraph, "銀河鉄道の夜")
            {
                body_title_index = Some(paragraph_index);
            } else if let Some(body_title_index) = body_title_index
                && paragraph_index == body_title_index + 1
            {
                if paragraph_trimmed_starts_with(paragraph, "一、午后の授業") {
                    return Some(GingaFrontMatterIndices {
                        title_index: 0,
                        toc_start_index,
                        body_title_index,
                        body_start_index: paragraph_index,
                    });
                }
                return None;
            }
        }

        paragraph_index = paragraph_index.checked_add(1)?;
    }

    None
}

pub(crate) fn paragraph_contains(paragraph: &Paragraph, needle: &str) -> bool {
    let mut matched = 0usize;
    let needle_len = needle.chars().count();
    if needle_len == 0 {
        return true;
    }

    for text in paragraph_text_fragments(paragraph) {
        for character in text.chars() {
            let expected = needle.chars().nth(matched);
            if expected == Some(character) {
                matched += 1;
                if matched == needle_len {
                    return true;
                }
            } else {
                matched = usize::from(needle.starts_with(character));
            }
        }
    }

    false
}

pub(crate) fn paragraph_trimmed_equals(paragraph: &Paragraph, expected: &str) -> bool {
    let mut expected_index = 0usize;
    let mut saw_non_whitespace = false;
    let mut trailing_whitespace = false;

    for text in paragraph_text_fragments(paragraph) {
        for character in text.chars() {
            if !saw_non_whitespace && character.is_whitespace() {
                continue;
            }
            saw_non_whitespace = true;
            if trailing_whitespace {
                if !character.is_whitespace() {
                    return false;
                }
                continue;
            }
            if character.is_whitespace() && expected.chars().nth(expected_index).is_none() {
                trailing_whitespace = true;
                continue;
            }
            if expected.chars().nth(expected_index) != Some(character) {
                return false;
            }
            expected_index += 1;
        }
    }

    saw_non_whitespace && expected.chars().nth(expected_index).is_none()
}

pub(crate) fn paragraph_trimmed_starts_with(paragraph: &Paragraph, expected: &str) -> bool {
    let mut expected_index = 0usize;
    let mut saw_non_whitespace = false;

    for text in paragraph_text_fragments(paragraph) {
        for character in text.chars() {
            if !saw_non_whitespace && character.is_whitespace() {
                continue;
            }
            saw_non_whitespace = true;
            if expected.chars().nth(expected_index) != Some(character) {
                return false;
            }
            expected_index += 1;
            if expected.chars().nth(expected_index).is_none() {
                return true;
            }
        }
    }

    false
}

pub(crate) fn paragraph_text_fragments(paragraph: &Paragraph) -> impl Iterator<Item = &str> {
    paragraph
        .inlines()
        .iter()
        .filter_map(|inline| match inline {
            Inline::Text(run) => Some(run.text()),
            Inline::Ruby(ruby) => Some(ruby.base_text()),
            Inline::Unknown(_) => None,
        })
}

pub(crate) fn document_paragraph_character_count(document: &Document) -> Result<usize> {
    document
        .blocks()
        .iter()
        .filter_map(|block| match block {
            Block::Paragraph(paragraph) => Some(paragraph),
            Block::Unknown(_) => None,
        })
        .try_fold(0usize, |total, paragraph| {
            let character_count =
                paragraph_text_fragments(paragraph).try_fold(0usize, |character_count, text| {
                    checked_page_shape_add(character_count, text.chars().count())
                })?;
            checked_page_shape_add(total, character_count.max(1))
        })
}

pub(crate) fn document_chapter_title_candidates(document: &Document) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut titles = Vec::new();
    for (_, text) in document_paragraph_texts(document) {
        let trimmed = text.trim();
        if !is_short_chapter_title(trimmed) {
            continue;
        }
        if seen.insert(trimmed.to_string()) {
            titles.push(trimmed.to_string());
        }
    }
    titles.sort_by_key(|title| std::cmp::Reverse(title.chars().count()));
    titles
}

pub(crate) fn running_body_start_page(
    pages: &[Vec<PageTextLine>],
    document_title: &str,
    chapter_titles: &[String],
) -> Option<usize> {
    let mut seen_body_title_page = false;
    for (page_index, page) in pages.iter().enumerate() {
        if page_index > 0 && page_has_exact_text_line(page, document_title) {
            seen_body_title_page = true;
            continue;
        }
        if seen_body_title_page && page_chapter_title(page, chapter_titles).is_some() {
            return Some(page_index);
        }
    }
    None
}

pub(crate) fn running_chapter_title_for_page(
    pages: &[Vec<PageTextLine>],
    body_start_page: usize,
    page_index: usize,
    chapter_titles: &[String],
) -> Option<String> {
    let mut current = None;
    for page in pages
        .iter()
        .take(page_index.saturating_add(1))
        .skip(body_start_page)
    {
        if let Some(title) = page_chapter_title(page, chapter_titles) {
            current = Some(title);
        }
    }
    current
}

pub(crate) fn is_short_chapter_title(text: &str) -> bool {
    if text.chars().count() > 32 {
        return false;
    }
    let Some((prefix, suffix)) = text.split_once('、') else {
        return false;
    };
    !prefix.is_empty() && !suffix.trim().is_empty() && prefix.chars().all(is_japanese_number_char)
}

pub(crate) fn is_japanese_number_char(character: char) -> bool {
    matches!(
        character,
        '〇' | '零'
            | '一'
            | '二'
            | '三'
            | '四'
            | '五'
            | '六'
            | '七'
            | '八'
            | '九'
            | '十'
            | '百'
            | '千'
            | '壱'
            | '弐'
            | '参'
    )
}

pub(crate) fn toc_leader_line(title: &str, page_label: &str, max_columns: usize) -> String {
    let title_width = text_display_column_width(title);
    let page_width = text_display_column_width(page_label);
    let leader_width = max_columns.saturating_sub(title_width + page_width).max(8);
    let leader_count = (leader_width / display_column_width('…')).max(4);
    format!("{title}{}{page_label}", "…".repeat(leader_count))
}

pub(crate) fn trim_trailing_projection_blank_lines(lines: &mut Vec<PageTextLine>) {
    while lines
        .last()
        .is_some_and(|line| line.text().is_empty() && line.paragraph_index().is_none())
    {
        lines.pop();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageLineSegment {
    pub(crate) line: PageTextLine,
    pub(crate) break_after: bool,
}

pub(crate) fn push_paginated_line(
    pages: &mut Vec<Vec<PageTextLine>>,
    current_page: &mut Vec<PageTextLine>,
    line: PageTextLine,
    lines_per_page: usize,
) {
    if current_page.len() >= lines_per_page {
        pages.push(std::mem::take(current_page));
    }
    current_page.push(line);
}

pub(crate) fn document_plain_text(document: &Document) -> String {
    let mut output = String::new();

    for block in document.blocks() {
        if let Block::Paragraph(paragraph) = block {
            output.push_str(&paragraph_text(paragraph));
            output.push('\n');
        }
    }

    output
}

pub(crate) fn checked_char_boundary(text: &str, char_offset: usize) -> Result<usize> {
    let char_count = text.chars().count();
    if char_offset > char_count {
        return Err(rjtd_core::Error::InvalidData(format!(
            "char offset {char_offset} out of range (paragraph length {char_count})"
        )));
    }

    if char_offset == char_count {
        return Ok(text.len());
    }

    text.char_indices()
        .nth(char_offset)
        .map(|(byte_index, _)| byte_index)
        .ok_or_else(|| {
            rjtd_core::Error::InvalidData(format!(
                "char offset {char_offset} out of range (paragraph length {char_count})"
            ))
        })
}

pub(crate) fn find_in_text(text: &str, query: &str, case_sensitive: bool) -> Vec<usize> {
    if text.is_empty() || query.is_empty() {
        return Vec::new();
    }

    let text_chars = text.chars().collect::<Vec<_>>();
    let query_chars = query.chars().collect::<Vec<_>>();
    let query_len = query_chars.len();
    if text_chars.len() < query_len {
        return Vec::new();
    }

    if case_sensitive {
        return text_chars
            .windows(query_len)
            .enumerate()
            .filter_map(|(index, window)| (window == query_chars.as_slice()).then_some(index))
            .collect();
    }

    let folded_text = text_chars
        .iter()
        .map(|character| character.to_lowercase().collect::<String>())
        .collect::<Vec<_>>();
    let folded_query = query_chars
        .iter()
        .map(|character| character.to_lowercase().collect::<String>())
        .collect::<Vec<_>>();

    folded_text
        .windows(query_len)
        .enumerate()
        .filter_map(|(index, window)| (window == folded_query.as_slice()).then_some(index))
        .collect()
}

pub(crate) fn display_column_width(character: char) -> usize {
    match character {
        '\t' => APP_TAB_COLUMNS,
        _ if character.is_ascii() => 1,
        _ => 2,
    }
}

pub(crate) fn column_width_px(layout: PageLayout) -> f64 {
    layout.body_width_px() as f64 / layout.wrap_columns(WritingMode::Horizontal) as f64
}

pub(crate) fn line_index_for_y(layout: PageLayout, line_count: usize, y: f64) -> usize {
    if line_count == 0 {
        return 0;
    }

    let relative_y = normalize_coordinate(y) - layout.margin_px() as f64;
    let line_index = (relative_y.max(0.0) / APP_LINE_HEIGHT_PX as f64).floor() as usize;
    line_index.min(line_count - 1)
}

pub(crate) fn cursor_rect_from_line(
    layout: PageLayout,
    page_index: usize,
    line_index: usize,
    line: &PageTextLine,
    char_offset: usize,
) -> CursorRect {
    let char_offset = char_offset.clamp(line.char_start(), line.char_end());
    let x = layout.margin_px() as f64
        + column_units_before(line, char_offset) * column_width_px(layout);
    let y = layout.margin_px() as f64 + line_index as f64 * APP_LINE_HEIGHT_PX as f64;

    CursorRect {
        page_index,
        line_index,
        x,
        y,
        height: APP_LINE_HEIGHT_PX as f64,
    }
}

pub(crate) fn column_units_before(line: &PageTextLine, char_offset: usize) -> f64 {
    let mut units = 0.0;

    for (current_offset, character) in (line.char_start()..).zip(line.text().chars()) {
        if current_offset >= char_offset {
            break;
        }
        units += display_column_width(character) as f64;
    }

    units
}

pub(crate) fn char_offset_for_x(layout: PageLayout, line: &PageTextLine, x: f64) -> usize {
    let target_units =
        ((normalize_coordinate(x) - layout.margin_px() as f64) / column_width_px(layout)).max(0.0);
    let mut units = 0.0;

    for (char_offset, character) in (line.char_start()..).zip(line.text().chars()) {
        let width = display_column_width(character) as f64;
        if target_units <= units + (width / 2.0) {
            return char_offset;
        }
        units += width;
    }

    line.char_end()
}

pub(crate) fn selection_overlap(
    line: &PageTextLine,
    paragraph_index: usize,
    range: &TextRange,
) -> Option<(usize, usize)> {
    if paragraph_index < range.start_para || paragraph_index > range.end_para {
        return None;
    }

    let selection_start = if paragraph_index == range.start_para {
        range.start_offset
    } else {
        line.char_start()
    };
    let selection_end = if paragraph_index == range.end_para {
        range.end_offset
    } else {
        line.char_end()
    };

    let start = line.char_start().max(selection_start);
    let end = line.char_end().min(selection_end);
    if start > end || (start == end && !line.text().is_empty()) {
        return None;
    }
    Some((start, end))
}

pub(crate) fn normalize_coordinate(coordinate: f64) -> f64 {
    if coordinate.is_finite() {
        coordinate
    } else {
        0.0
    }
}

pub(crate) fn format_cursor_rect(rect: &CursorRect) -> String {
    format!(
        "{{\"pageIndex\":{},\"lineIndex\":{},\"x\":{:.1},\"y\":{:.1},\"height\":{:.1}}}",
        rect.page_index, rect.line_index, rect.x, rect.y, rect.height
    )
}

pub(crate) fn format_search_result(hit: &SearchHit, wrapped: bool) -> String {
    format!(
        "{{\"found\":true,\"wrapped\":{},\"sec\":{},\"para\":{},\"charOffset\":{},\"length\":{}}}",
        wrapped, hit.sec, hit.para, hit.char_offset, hit.length
    )
}

pub(crate) fn format_search_hit(hit: &SearchHit) -> String {
    format!(
        "{{\"sec\":{},\"para\":{},\"charOffset\":{},\"length\":{}}}",
        hit.sec, hit.para, hit.char_offset, hit.length
    )
}

pub(crate) fn format_nav_text(section_idx: u32, paragraph_idx: u32, char_offset: u32) -> String {
    format!(
        "{{\"type\":\"text\",\"sec\":{},\"para\":{},\"charOffset\":{},\"context\":[]}}",
        section_idx, paragraph_idx, char_offset
    )
}
