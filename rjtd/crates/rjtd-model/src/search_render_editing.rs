use super::*;

impl DocumentCore {
    pub fn search_text(
        &self,
        query: &str,
        from_sec: u32,
        from_para: u32,
        from_char: u32,
        forward: bool,
        case_sensitive: bool,
    ) -> Result<String> {
        self.ensure_section(from_sec)?;
        if query.is_empty() {
            return Ok("{\"found\":false}".to_string());
        }

        let hits = self.search_hits(query, case_sensitive);
        if hits.is_empty() {
            return Ok("{\"found\":false}".to_string());
        }

        if forward {
            let after = hits.iter().find(|hit| {
                hit.sec > from_sec
                    || (hit.sec == from_sec && hit.para > from_para)
                    || (hit.sec == from_sec && hit.para == from_para && hit.char_offset > from_char)
            });
            Ok(match after {
                Some(hit) => format_search_result(hit, false),
                None => format_search_result(&hits[0], true),
            })
        } else {
            let before = hits.iter().rev().find(|hit| {
                hit.sec < from_sec
                    || (hit.sec == from_sec && hit.para < from_para)
                    || (hit.sec == from_sec && hit.para == from_para && hit.char_offset < from_char)
            });
            Ok(match before {
                Some(hit) => format_search_result(hit, false),
                None => format_search_result(&hits[hits.len() - 1], true),
            })
        }
    }

    pub fn search_text_native(
        &self,
        query: &str,
        from_sec: u32,
        from_para: u32,
        from_char: u32,
        forward: bool,
        case_sensitive: bool,
    ) -> Result<String> {
        self.search_text(
            query,
            from_sec,
            from_para,
            from_char,
            forward,
            case_sensitive,
        )
    }

    pub fn search_all_text(
        &self,
        query: &str,
        case_sensitive: bool,
        _include_cells: bool,
    ) -> String {
        if query.is_empty() {
            return "[]".to_string();
        }

        let hits = self.search_hits(query, case_sensitive);
        let json_hits = hits
            .iter()
            .map(format_search_hit)
            .collect::<Vec<_>>()
            .join(",");
        format!("[{json_hits}]")
    }

    pub fn search_all_text_native(
        &self,
        query: &str,
        case_sensitive: bool,
        include_cells: bool,
    ) -> String {
        self.search_all_text(query, case_sensitive, include_cells)
    }

    pub fn replace_text(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        length: u32,
        new_text: &str,
    ) -> Result<String> {
        self.delete_text(section_idx, paragraph_idx, char_offset, length)?;
        self.insert_text(section_idx, paragraph_idx, char_offset, new_text)?;
        Ok(format!(
            "{{\"ok\":true,\"charOffset\":{},\"newLength\":{}}}",
            char_offset,
            new_text.chars().count()
        ))
    }

    pub fn replace_text_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        length: u32,
        new_text: &str,
    ) -> Result<String> {
        self.replace_text(section_idx, paragraph_idx, char_offset, length, new_text)
    }

    pub fn replace_one(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String> {
        if query.is_empty() {
            return Ok("{\"ok\":false}".to_string());
        }

        let Some(hit) = self.search_hits(query, case_sensitive).first().copied() else {
            return Ok("{\"ok\":false}".to_string());
        };

        self.replace_text(hit.sec, hit.para, hit.char_offset, hit.length, new_text)?;
        Ok(format!(
            "{{\"ok\":true,\"sec\":{},\"para\":{},\"charOffset\":{},\"newLength\":{}}}",
            hit.sec,
            hit.para,
            hit.char_offset,
            new_text.chars().count()
        ))
    }

    pub fn replace_one_native(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String> {
        self.replace_one(query, new_text, case_sensitive)
    }

    pub fn replace_all(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String> {
        if query.is_empty() {
            return Ok("{\"ok\":true,\"count\":0}".to_string());
        }

        let mut hits = self.search_hits(query, case_sensitive);
        let count = hits.len();
        hits.reverse();

        for hit in hits {
            self.replace_text(hit.sec, hit.para, hit.char_offset, hit.length, new_text)?;
        }

        Ok(format!("{{\"ok\":true,\"count\":{count}}}"))
    }

    pub fn replace_all_native(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String> {
        self.replace_all(query, new_text, case_sensitive)
    }

    pub fn get_selection_rects(
        &self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let range = self.normalized_text_range(
            start_para_idx as usize,
            start_char_offset as usize,
            end_para_idx as usize,
            end_char_offset as usize,
        )?;
        if range.is_collapsed() {
            return Ok("[]".to_string());
        }

        let mut rects = Vec::new();
        for (page_index, page) in self.pages.iter().enumerate() {
            for (line_index, line) in page.iter().enumerate() {
                let Some(paragraph_index) = line.paragraph_index() else {
                    continue;
                };
                let Some((start, end)) = selection_overlap(line, paragraph_index, &range) else {
                    continue;
                };
                let start_rect =
                    cursor_rect_from_line(self.page_layout, page_index, line_index, line, start);
                let end_rect =
                    cursor_rect_from_line(self.page_layout, page_index, line_index, line, end);
                let width = (end_rect.x - start_rect.x).max(2.0);
                rects.push(format!(
                    "{{\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1},\"width\":{:.1},\"height\":{:.1}}}",
                    page_index, start_rect.x, start_rect.y, width, start_rect.height
                ));
            }
        }

        Ok(format!("[{}]", rects.join(",")))
    }

    pub fn get_selection_rects_native(
        &self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.get_selection_rects(
            section_idx,
            start_para_idx,
            start_char_offset,
            end_para_idx,
            end_char_offset,
        )
    }

    pub fn delete_range(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let range = self.normalized_text_range(
            start_para_idx as usize,
            start_char_offset as usize,
            end_para_idx as usize,
            end_char_offset as usize,
        )?;
        if range.is_collapsed() {
            self.set_caret(
                section_idx,
                range.start_para as u32,
                range.start_offset as u32,
            );
            return Ok(json_ok_with(&format!(
                "\"paraIdx\":{},\"charOffset\":{}",
                range.start_para, range.start_offset
            )));
        }

        if range.start_para == range.end_para {
            return self.delete_text(
                section_idx,
                range.start_para as u32,
                range.start_offset as u32,
                (range.end_offset - range.start_offset) as u32,
            );
        }

        let start_text = paragraph_text(self.paragraph(range.start_para)?);
        let end_text = paragraph_text(self.paragraph(range.end_para)?);
        let start_byte = checked_char_boundary(&start_text, range.start_offset)?;
        let end_byte = checked_char_boundary(&end_text, range.end_offset)?;
        let merged = format!("{}{}", &start_text[..start_byte], &end_text[end_byte..]);
        let start_block = self.paragraph_block_index(range.start_para)?;

        for paragraph_index in (range.start_para + 1..=range.end_para).rev() {
            let block_index = self.paragraph_block_index(paragraph_index)?;
            self.document.blocks.remove(block_index);
        }
        self.replace_paragraph_block(start_block, merged)?;

        self.set_caret(
            section_idx,
            range.start_para as u32,
            range.start_offset as u32,
        );
        self.refresh_pages();
        Ok(json_ok_with(&format!(
            "\"paraIdx\":{},\"charOffset\":{}",
            range.start_para, range.start_offset
        )))
    }

    pub fn delete_range_native(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.delete_range(
            section_idx,
            start_para_idx,
            start_char_offset,
            end_para_idx,
            end_char_offset,
        )
    }

    pub fn copy_selection(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let text = self.selected_text(
            start_para_idx as usize,
            start_char_offset as usize,
            end_para_idx as usize,
            end_char_offset as usize,
        )?;
        self.clipboard_text = Some(text.clone());
        Ok(json_ok_with(&format!("\"text\":{}", json_string(&text))))
    }

    pub fn copy_selection_native(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.copy_selection(
            section_idx,
            start_para_idx,
            start_char_offset,
            end_para_idx,
            end_char_offset,
        )
    }

    pub fn paste_internal(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let Some(text) = self.clipboard_text.clone() else {
            return Ok("{\"ok\":false,\"error\":\"clipboard empty\"}".to_string());
        };
        let mut parts = text.split('\n');
        let first = parts.next().unwrap_or_default();
        let result = self.insert_text(section_idx, paragraph_idx, char_offset, first)?;
        let mut current_para = paragraph_idx;
        let mut current_offset = char_offset + first.chars().count() as u32;

        for part in parts {
            self.split_paragraph(section_idx, current_para, current_offset)?;
            current_para += 1;
            self.insert_text(section_idx, current_para, 0, part)?;
            current_offset = part.chars().count() as u32;
        }

        if text.contains('\n') {
            Ok(json_ok_with(&format!(
                "\"paraIdx\":{},\"charOffset\":{}",
                current_para, current_offset
            )))
        } else {
            Ok(result)
        }
    }

    pub fn paste_internal_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.paste_internal(section_idx, paragraph_idx, char_offset)
    }

    pub fn has_internal_clipboard(&self) -> bool {
        self.clipboard_text
            .as_ref()
            .is_some_and(|text| !text.is_empty())
    }

    pub fn get_clipboard_text(&self) -> String {
        self.clipboard_text.clone().unwrap_or_default()
    }

    pub fn clear_clipboard(&mut self) {
        self.clipboard_text = None;
    }

    pub fn clipboard_has_control(&self) -> bool {
        false
    }

    pub fn render_page_svg(&self, page_num: u32) -> Result<String> {
        let index = page_num as usize;
        let lines = self.page_lines(page_num)?;
        let decoration = self.page_decoration(index);

        Ok(render_text_page_svg(
            lines,
            index + 1,
            self.page_count() as usize,
            self.page_layout,
            self.writing_mode,
            &self.document,
            decoration.as_ref(),
        ))
    }

    pub fn render_page_svg_native(&self, page_num: u32) -> Result<String> {
        self.render_page_svg(page_num)
    }

    pub fn render_page_html(&self, page_num: u32) -> Result<String> {
        let svg = self.render_page_svg(page_num)?;
        Ok(format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>rjtd page {}</title></head><body>{}</body></html>",
            page_num + 1,
            svg
        ))
    }

    pub fn render_page_html_native(&self, page_num: u32) -> Result<String> {
        self.render_page_html(page_num)
    }

    pub fn get_cursor_rect(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let rect = self.cursor_rect_for(paragraph_idx as usize, char_offset as usize)?;
        Ok(format_cursor_rect(&rect))
    }

    pub fn get_cursor_rect_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_cursor_rect(section_idx, paragraph_idx, char_offset)
    }

    pub fn hit_test(&self, page_num: u32, x: f64, y: f64) -> Result<String> {
        let lines = self.page_lines(page_num)?;
        let Some((line_index, line)) =
            nearest_text_line(lines, line_index_for_y(self.page_layout, lines.len(), y))
        else {
            return Ok(format!(
                "{{\"hit\":false,\"sectionIndex\":0,\"paragraphIndex\":0,\"charOffset\":0,\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1}}}",
                page_num,
                normalize_coordinate(x),
                normalize_coordinate(y)
            ));
        };
        let paragraph_index = line.paragraph_index().unwrap_or_default();
        let char_offset = char_offset_for_x(self.page_layout, line, x);
        Ok(format!(
            "{{\"hit\":true,\"sectionIndex\":0,\"paragraphIndex\":{},\"charOffset\":{},\"pageIndex\":{},\"lineIndex\":{},\"x\":{:.1},\"y\":{:.1}}}",
            paragraph_index,
            char_offset,
            page_num,
            line_index,
            normalize_coordinate(x),
            normalize_coordinate(y)
        ))
    }

    pub fn hit_test_native(&self, page_num: u32, x: f64, y: f64) -> Result<String> {
        self.hit_test(page_num, x, y)
    }

    pub fn get_line_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let lines = self.paragraph_lines(paragraph_idx as usize);
        if lines.is_empty() {
            return Err(rjtd_core::Error::InvalidData(format!(
                "paragraph {paragraph_idx} out of range"
            )));
        }

        let selected_index = paragraph_line_index(&lines, char_offset as usize);
        let (page_index, page_line_index, line) = lines[selected_index];
        Ok(format!(
            "{{\"sectionIndex\":0,\"paragraphIndex\":{},\"lineIndex\":{},\"lineCount\":{},\"charStart\":{},\"charEnd\":{},\"pageIndex\":{},\"pageLineIndex\":{}}}",
            paragraph_idx,
            selected_index,
            lines.len(),
            line.char_start(),
            line.char_end(),
            page_index,
            page_line_index
        ))
    }

    pub fn get_line_info_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_line_info(section_idx, paragraph_idx, char_offset)
    }

    pub fn move_vertical(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        delta: i32,
        preferred_x: f64,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let locations = self.text_line_locations();
        if locations.is_empty() {
            return Err(rjtd_core::Error::InvalidData(
                "document has no text lines".to_string(),
            ));
        }

        let current_index =
            text_location_index(&locations, paragraph_idx as usize, char_offset as usize)?;
        let target_index = (current_index as i64 + i64::from(delta))
            .clamp(0, locations.len().saturating_sub(1) as i64) as usize;
        let (page_index, page_line_index, target_line) = locations[target_index];
        let current_rect = self.cursor_rect_for(paragraph_idx as usize, char_offset as usize)?;
        let target_x = if preferred_x.is_finite() && preferred_x >= 0.0 {
            preferred_x
        } else {
            current_rect.x
        };
        let new_char_offset = char_offset_for_x(self.page_layout, target_line, target_x);
        let rect = cursor_rect_from_line(
            self.page_layout,
            page_index,
            page_line_index,
            target_line,
            new_char_offset,
        );
        Ok(format!(
            "{{\"sectionIndex\":0,\"paragraphIndex\":{},\"charOffset\":{},\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1},\"height\":{:.1},\"preferredX\":{:.1},\"rectValid\":true}}",
            target_line.paragraph_index().unwrap_or_default(),
            new_char_offset,
            rect.page_index,
            rect.x,
            rect.y,
            rect.height,
            target_x
        ))
    }

    pub fn move_vertical_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        delta: i32,
        preferred_x: f64,
    ) -> Result<String> {
        self.move_vertical(section_idx, paragraph_idx, char_offset, delta, preferred_x)
    }

    pub(crate) fn page_lines(&self, page_num: u32) -> Result<&[PageTextLine]> {
        self.pages
            .get(page_num as usize)
            .map(Vec::as_slice)
            .ok_or_else(|| rjtd_core::Error::InvalidData(format!("page {page_num} out of range")))
    }

    pub(crate) fn cursor_rect_for(
        &self,
        paragraph_index: usize,
        char_offset: usize,
    ) -> Result<CursorRect> {
        let mut last_line = None;

        for (page_index, page) in self.pages.iter().enumerate() {
            for (line_index, line) in page.iter().enumerate() {
                if line.paragraph_index() != Some(paragraph_index) {
                    continue;
                }

                last_line = Some((page_index, line_index, line));
                if char_offset <= line.char_end() {
                    return Ok(cursor_rect_from_line(
                        self.page_layout,
                        page_index,
                        line_index,
                        line,
                        char_offset,
                    ));
                }
            }
        }

        if let Some((page_index, line_index, line)) = last_line {
            return Ok(cursor_rect_from_line(
                self.page_layout,
                page_index,
                line_index,
                line,
                line.char_end(),
            ));
        }

        Err(rjtd_core::Error::InvalidData(format!(
            "paragraph {paragraph_index} out of range"
        )))
    }

    pub(crate) fn paragraph_lines(
        &self,
        paragraph_index: usize,
    ) -> Vec<(usize, usize, &PageTextLine)> {
        self.text_line_locations()
            .into_iter()
            .filter(|(_, _, line)| line.paragraph_index() == Some(paragraph_index))
            .collect()
    }

    pub(crate) fn text_line_locations(&self) -> Vec<(usize, usize, &PageTextLine)> {
        let mut locations = Vec::new();

        for (page_index, page) in self.pages.iter().enumerate() {
            for (line_index, line) in page.iter().enumerate() {
                if line.paragraph_index().is_some() {
                    locations.push((page_index, line_index, line));
                }
            }
        }

        locations
    }

    pub(crate) fn normalized_text_range(
        &self,
        start_para: usize,
        start_offset: usize,
        end_para: usize,
        end_offset: usize,
    ) -> Result<TextRange> {
        let (start_para, start_offset, end_para, end_offset) =
            if (start_para, start_offset) <= (end_para, end_offset) {
                (start_para, start_offset, end_para, end_offset)
            } else {
                (end_para, end_offset, start_para, start_offset)
            };

        let start_text = paragraph_text(self.paragraph(start_para)?);
        let end_text = paragraph_text(self.paragraph(end_para)?);
        checked_char_boundary(&start_text, start_offset)?;
        checked_char_boundary(&end_text, end_offset)?;

        Ok(TextRange {
            start_para,
            start_offset,
            end_para,
            end_offset,
        })
    }

    pub(crate) fn selected_text(
        &self,
        start_para: usize,
        start_offset: usize,
        end_para: usize,
        end_offset: usize,
    ) -> Result<String> {
        let range = self.normalized_text_range(start_para, start_offset, end_para, end_offset)?;
        if range.is_collapsed() {
            return Ok(String::new());
        }

        if range.start_para == range.end_para {
            let text = paragraph_text(self.paragraph(range.start_para)?);
            let start = checked_char_boundary(&text, range.start_offset)?;
            let end = checked_char_boundary(&text, range.end_offset)?;
            return Ok(text[start..end].to_string());
        }

        let mut chunks = Vec::new();
        let first_text = paragraph_text(self.paragraph(range.start_para)?);
        let first_start = checked_char_boundary(&first_text, range.start_offset)?;
        chunks.push(first_text[first_start..].to_string());

        for paragraph_index in range.start_para + 1..range.end_para {
            chunks.push(paragraph_text(self.paragraph(paragraph_index)?));
        }

        let last_text = paragraph_text(self.paragraph(range.end_para)?);
        let last_end = checked_char_boundary(&last_text, range.end_offset)?;
        chunks.push(last_text[..last_end].to_string());

        Ok(chunks.join("\n"))
    }

    pub(crate) fn search_hits(&self, query: &str, case_sensitive: bool) -> Vec<SearchHit> {
        let mut hits = Vec::new();
        let mut paragraph_index = 0u32;
        let length = query.chars().count() as u32;

        for block in self.document.blocks() {
            if let Block::Paragraph(paragraph) = block {
                let text = paragraph_text(paragraph);
                for offset in find_in_text(&text, query, case_sensitive) {
                    hits.push(SearchHit {
                        sec: 0,
                        para: paragraph_index,
                        char_offset: offset as u32,
                        length,
                    });
                }
                paragraph_index += 1;
            }
        }

        hits
    }

    pub(crate) fn paragraph_count(&self) -> usize {
        self.document
            .blocks()
            .iter()
            .filter(|block| matches!(block, Block::Paragraph(_)))
            .count()
    }

    pub(crate) fn paragraph(&self, paragraph_index: usize) -> Result<&Paragraph> {
        let block_index = self.paragraph_block_index(paragraph_index)?;
        match &self.document.blocks[block_index] {
            Block::Paragraph(paragraph) => Ok(paragraph),
            Block::Unknown(_) => unreachable!("paragraph_block_index returned an unknown block"),
        }
    }

    pub(crate) fn paragraph_mut(&mut self, paragraph_index: usize) -> Result<&mut Paragraph> {
        let block_index = self.paragraph_block_index(paragraph_index)?;
        match &mut self.document.blocks[block_index] {
            Block::Paragraph(paragraph) => Ok(paragraph),
            Block::Unknown(_) => unreachable!("paragraph_block_index returned an unknown block"),
        }
    }

    pub(crate) fn paragraph_block_index(&self, paragraph_index: usize) -> Result<usize> {
        let mut current_index = 0usize;

        for (block_index, block) in self.document.blocks().iter().enumerate() {
            if matches!(block, Block::Paragraph(_)) {
                if current_index == paragraph_index {
                    return Ok(block_index);
                }
                current_index += 1;
            }
        }

        Err(rjtd_core::Error::InvalidData(format!(
            "paragraph {paragraph_index} out of range"
        )))
    }

    pub(crate) fn ensure_text_position(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<()> {
        self.ensure_section(section_idx)?;
        let text = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        checked_char_boundary(&text, char_offset as usize)?;
        Ok(())
    }

    pub(crate) fn ensure_parent_paragraph(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
    ) -> Result<()> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(parent_para_idx as usize)?;
        Ok(())
    }

    pub(crate) fn replace_paragraph_block(
        &mut self,
        block_index: usize,
        text: String,
    ) -> Result<()> {
        match self.document.blocks.get_mut(block_index) {
            Some(Block::Paragraph(paragraph)) => {
                paragraph.set_text(text);
                Ok(())
            }
            Some(Block::Unknown(_)) => Err(rjtd_core::Error::InvalidData(format!(
                "block {block_index} is not a paragraph"
            ))),
            None => Err(rjtd_core::Error::InvalidData(format!(
                "block {block_index} out of range"
            ))),
        }
    }

    pub(crate) fn set_paragraph_text(
        &mut self,
        paragraph_index: usize,
        text: String,
    ) -> Result<()> {
        let block_index = self.paragraph_block_index(paragraph_index)?;
        self.replace_paragraph_block(block_index, text)
    }

    pub(crate) fn set_paragraph_style(
        &mut self,
        paragraph_index: usize,
        style: Option<StyleRef>,
    ) -> Result<()> {
        self.paragraph_mut(paragraph_index)?.set_style(style);
        Ok(())
    }

    pub(crate) fn set_caret(&mut self, section_idx: u32, paragraph_idx: u32, char_offset: u32) {
        self.caret_section = section_idx;
        self.caret_paragraph = paragraph_idx;
        self.caret_char_offset = char_offset;
    }

    pub(crate) fn refresh_pages(&mut self) {
        self.pages = paginate_document_text(&self.document, self.page_layout, self.writing_mode);
        if project_fdm_single_page_diagram(&self.document, &mut self.pages) {
            return;
        }
        if let Some(pages) = project_sample_front_matter_pages(
            &self.document,
            &self.file_name,
            self.page_layout,
            self.writing_mode,
        ) {
            self.pages = pages;
        }
    }

    pub(crate) fn refresh_pages_with_budget(&mut self, budget: &mut ResourceBudget) -> Result<()> {
        let shape = page_construction_shape(&self.document, self.page_layout, self.writing_mode)?;
        budget.reserve_page_output(shape.pages, shape.lines)?;
        self.refresh_pages();
        Ok(())
    }

    pub(crate) fn ensure_section(&self, section_idx: u32) -> Result<()> {
        if section_idx == 0 {
            Ok(())
        } else {
            Err(rjtd_core::Error::InvalidData(format!(
                "section {section_idx} out of range"
            )))
        }
    }
}
