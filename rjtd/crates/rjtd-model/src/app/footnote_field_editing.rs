use super::*;

impl DocumentCore {
    #[allow(clippy::too_many_arguments)]
    pub fn insert_text_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _fn_para_idx: u32,
        char_offset: u32,
        _text: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_text_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _fn_para_idx: u32,
        char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn split_paragraph_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        fn_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"fnParaIndex\":{fn_para_idx},\"charOffset\":{char_offset}}}"
        ))
    }

    pub fn merge_paragraph_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        fn_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"fnParaIndex\":{fn_para_idx},\"charOffset\":0}}"
        ))
    }

    pub fn get_cursor_rect_in_footnote(
        &self,
        page_num: u32,
        _footnote_index: u32,
        _fn_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.page_lines(page_num)?;
        Ok(default_cursor_rect_json(page_num))
    }

    pub fn get_cursor_rect_in_note(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _note_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(default_cursor_rect_json(0))
    }

    pub fn get_para_properties_in_footnote(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _fn_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(default_para_properties_json())
    }

    pub fn apply_para_format_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _fn_para_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_selection_rects_in_footnote(
        &self,
        page_num: u32,
        _footnote_index: u32,
        _start_fn_para: u32,
        _start_offset: u32,
        _end_fn_para: u32,
        _end_offset: u32,
    ) -> Result<String> {
        self.page_lines(page_num)?;
        Ok("[]".to_string())
    }

    pub fn get_para_properties_in_hf(
        &self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(default_para_properties_json())
    }

    pub fn apply_para_format_in_hf(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn insert_field_in_hf(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        char_offset: u32,
        _field_type: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn apply_hf_template(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _template_id: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn export_selection_html(
        &self,
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
        Ok(format!("<p>{}</p>", escape_xml(&text)))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn export_selection_in_cell_html(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _start_cell_para: u32,
        _start_offset: u32,
        _end_cell_para: u32,
        _end_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(String::new())
    }

    pub fn export_control_html(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _cell_path_json: &str,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(String::new())
    }

    pub fn paste_html(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _html: &str,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn paste_html_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        char_offset: u32,
        _html: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn paste_html_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
        _html: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn get_text_box_control_index(&self, section_idx: u32, paragraph_idx: u32) -> Result<i32> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok(-1)
    }

    pub fn get_text_box_control_index_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<i32> {
        self.get_text_box_control_index(section_idx, paragraph_idx)
    }

    pub fn get_char_properties_at(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(default_char_properties_json())
    }

    pub fn get_char_properties_at_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_char_properties_at(section_idx, paragraph_idx, char_offset)
    }

    pub fn apply_char_format(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        start_offset: u32,
        end_offset: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, start_offset)?;
        self.ensure_text_position(section_idx, paragraph_idx, end_offset)?;
        if start_offset > end_offset {
            return Err(rjtd_core::Error::InvalidData(format!(
                "start offset {start_offset} is after end offset {end_offset}"
            )));
        }
        Ok("{\"ok\":true}".to_string())
    }

    pub fn apply_char_format_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        start_offset: u32,
        end_offset: u32,
        props_json: &str,
    ) -> Result<String> {
        self.apply_char_format(
            section_idx,
            paragraph_idx,
            start_offset,
            end_offset,
            props_json,
        )
    }

    pub fn find_or_create_font_id(&self, _name: &str) -> u32 {
        0
    }

    pub fn find_or_create_font_id_for_lang(&self, _lang: u32, _name: &str) -> u32 {
        0
    }

    pub fn get_para_properties_at(&self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok(default_para_properties_json())
    }

    pub fn get_para_properties_at_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String> {
        self.get_para_properties_at(section_idx, paragraph_idx)
    }

    pub fn apply_para_format(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok("{\"ok\":true}".to_string())
    }

    pub fn apply_para_format_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.apply_para_format(section_idx, paragraph_idx, props_json)
    }

    pub fn get_style_list(&self) -> String {
        let candidates = text_style_candidates(self.document.unknown_styles());
        let mut output = format!(
            "[{{\"id\":0,\"name\":\"Normal\",\"englishName\":\"Normal\",\"type\":0,\"nextStyleId\":0,\"paraShapeId\":0,\"charShapeId\":0,\"decoded\":false,\"sourceStreamCount\":{},\"candidateCount\":{}}}",
            self.document.unknown_styles().len(),
            candidates.len()
        );
        for candidate in &candidates {
            output.push(',');
            push_style_candidate_json(&mut output, candidate);
        }
        output.push(']');
        output
    }

    pub fn get_style_detail(&self, style_id: u32) -> Result<String> {
        if style_id == 0 {
            Ok(format!(
                "{{\"charProps\":{},\"paraProps\":{},\"decoded\":false,\"sourceStreams\":{}}}",
                default_char_properties_json(),
                default_para_properties_json(),
                style_source_streams_json(self.document.unknown_styles())
            ))
        } else {
            let candidates = text_style_candidates(self.document.unknown_styles());
            match candidates.iter().find(|candidate| candidate.id == style_id) {
                Some(candidate) => Ok(style_candidate_detail_json(candidate)),
                None => Err(rjtd_core::Error::InvalidData(format!(
                    "style {style_id} out of range"
                ))),
            }
        }
    }

    pub fn get_style_at(&self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        let paragraph = self.paragraph(paragraph_idx as usize)?;
        Ok(
            match paragraph
                .style()
                .and_then(|style| style.id().parse::<u32>().ok())
            {
                Some(0) | None => "{\"id\":0,\"name\":\"Normal\"}".to_string(),
                Some(style_id) => {
                    let candidates = text_style_candidates(self.document.unknown_styles());
                    match candidates.iter().find(|candidate| candidate.id == style_id) {
                        Some(candidate) => style_at_candidate_json(candidate),
                        None => format!(
                            "{{\"id\":{},\"name\":\"Unknown\",\"decoded\":false,\"jtdCandidate\":true}}",
                            style_id
                        ),
                    }
                }
            },
        )
    }

    pub fn apply_style(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        style_id: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        if style_id == 0 {
            self.set_paragraph_style(paragraph_idx as usize, None)?;
            return Ok("{\"ok\":true}".to_string());
        }
        let candidates = text_style_candidates(self.document.unknown_styles());
        let Some(candidate) = candidates.iter().find(|candidate| candidate.id == style_id) else {
            return Err(rjtd_core::Error::InvalidData(format!(
                "style {style_id} out of range"
            )));
        };
        self.set_paragraph_style(
            paragraph_idx as usize,
            Some(StyleRef::new(candidate.id.to_string())),
        )?;
        Ok(format!(
            "{{\"ok\":true,\"decoded\":false,\"styleId\":{},\"name\":{}}}",
            candidate.id,
            json_string(&candidate.name)
        ))
    }

    pub fn get_numbering_list(&self) -> String {
        "[]".to_string()
    }

    pub fn get_bullet_list(&self) -> String {
        "[]".to_string()
    }

    pub fn ensure_default_numbering(&self) -> u32 {
        0
    }

    pub fn ensure_default_bullet(&self, _bullet_char: &str) -> u32 {
        0
    }

    pub fn get_paragraph_count(&self, section_idx: u32) -> Result<u32> {
        self.ensure_section(section_idx)?;
        Ok(self.paragraph_count() as u32)
    }

    pub fn get_paragraph_count_native(&self, section_idx: u32) -> Result<u32> {
        self.get_paragraph_count(section_idx)
    }

    pub fn get_paragraph_length(&self, section_idx: u32, paragraph_idx: u32) -> Result<u32> {
        self.ensure_section(section_idx)?;
        let paragraph = self.paragraph(paragraph_idx as usize)?;
        Ok(paragraph_text(paragraph).chars().count() as u32)
    }

    pub fn get_paragraph_length_native(&self, section_idx: u32, paragraph_idx: u32) -> Result<u32> {
        self.get_paragraph_length(section_idx, paragraph_idx)
    }

    pub fn get_text_range(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let text = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let start = checked_char_boundary(&text, char_offset as usize)?;
        let end_offset = (char_offset as usize)
            .saturating_add(count as usize)
            .min(text.chars().count());
        let end = checked_char_boundary(&text, end_offset)?;
        Ok(text[start..end].to_string())
    }

    pub fn get_text_range_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.get_text_range(section_idx, paragraph_idx, char_offset, count)
    }

    pub fn insert_text(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let current = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let insert_at = checked_char_boundary(&current, char_offset as usize)?;
        let mut next = current;
        next.insert_str(insert_at, text);
        self.set_paragraph_text(paragraph_idx as usize, next)?;

        let new_offset = char_offset + text.chars().count() as u32;
        self.set_caret(section_idx, paragraph_idx, new_offset);
        self.refresh_pages();
        Ok(json_ok_with(&format!("\"charOffset\":{new_offset}")))
    }

    pub fn insert_text_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String> {
        self.insert_text(section_idx, paragraph_idx, char_offset, text)
    }

    pub fn delete_text(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let current = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let start = checked_char_boundary(&current, char_offset as usize)?;
        let end_offset = (char_offset as usize)
            .saturating_add(count as usize)
            .min(current.chars().count());
        let end = checked_char_boundary(&current, end_offset)?;
        let mut next = current;
        next.replace_range(start..end, "");
        self.set_paragraph_text(paragraph_idx as usize, next)?;

        self.set_caret(section_idx, paragraph_idx, char_offset);
        self.refresh_pages();
        Ok(json_ok_with(&format!("\"charOffset\":{char_offset}")))
    }

    pub fn delete_text_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.delete_text(section_idx, paragraph_idx, char_offset, count)
    }

    pub fn split_paragraph(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let block_index = self.paragraph_block_index(paragraph_idx as usize)?;
        let current = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let original_style = self.paragraph(paragraph_idx as usize)?.style().cloned();
        let split_at = checked_char_boundary(&current, char_offset as usize)?;
        let left = current[..split_at].to_string();
        let right = current[split_at..].to_string();
        self.replace_paragraph_block(block_index, left)?;
        self.document.blocks.insert(
            block_index + 1,
            Block::Paragraph(Paragraph::new(
                vec![Inline::Text(TextRun::new(right, None))],
                original_style,
            )),
        );

        let new_paragraph_idx = paragraph_idx + 1;
        self.set_caret(section_idx, new_paragraph_idx, 0);
        self.refresh_pages();
        Ok(json_ok_with(&format!(
            "\"paraIdx\":{new_paragraph_idx},\"charOffset\":0"
        )))
    }

    pub fn split_paragraph_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.split_paragraph(section_idx, paragraph_idx, char_offset)
    }

    pub fn merge_paragraph(&mut self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        if paragraph_idx == 0 {
            return Err(rjtd_core::Error::InvalidData(
                "first paragraph cannot be merged".to_string(),
            ));
        }

        let previous_idx = paragraph_idx - 1;
        let previous_block_index = self.paragraph_block_index(previous_idx as usize)?;
        let current_block_index = self.paragraph_block_index(paragraph_idx as usize)?;
        let previous = paragraph_text(self.paragraph(previous_idx as usize)?);
        let current = paragraph_text(self.paragraph(paragraph_idx as usize)?);
        let merge_point = previous.chars().count() as u32;
        self.replace_paragraph_block(previous_block_index, format!("{previous}{current}"))?;
        self.document.blocks.remove(current_block_index);

        self.set_caret(section_idx, previous_idx, merge_point);
        self.refresh_pages();
        Ok(json_ok_with(&format!(
            "\"paraIdx\":{previous_idx},\"charOffset\":{merge_point}"
        )))
    }

    pub fn merge_paragraph_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String> {
        self.merge_paragraph(section_idx, paragraph_idx)
    }

    pub fn get_caret_position(&self) -> String {
        format!(
            "{{\"sectionIndex\":{},\"paragraphIndex\":{},\"charOffset\":{}}}",
            self.caret_section, self.caret_paragraph, self.caret_char_offset
        )
    }

    pub fn save_snapshot(&mut self) -> u32 {
        let id = self.next_snapshot_id;
        self.next_snapshot_id = next_snapshot_id(id);
        let snapshot = DocumentSnapshot::capture(id, self);
        self.snapshots.push(snapshot);
        id
    }

    pub fn save_snapshot_native(&mut self) -> u32 {
        self.save_snapshot()
    }

    pub fn restore_snapshot(&mut self, id: u32) -> Result<String> {
        let snapshot = self
            .snapshots
            .iter()
            .find(|snapshot| snapshot.id == id)
            .cloned()
            .ok_or_else(|| rjtd_core::Error::InvalidData(format!("snapshot {id} not found")))?;

        self.document = snapshot.document;
        self.pages = snapshot.pages;
        self.file_name = snapshot.file_name;
        self.dpi = snapshot.dpi;
        self.page_layout = snapshot.page_layout;
        self.show_paragraph_marks = snapshot.show_paragraph_marks;
        self.show_control_codes = snapshot.show_control_codes;
        self.show_transparent_borders = snapshot.show_transparent_borders;
        self.clip_enabled = snapshot.clip_enabled;
        self.writing_mode = snapshot.writing_mode;
        self.caret_section = snapshot.caret_section;
        self.caret_paragraph = snapshot.caret_paragraph;
        self.caret_char_offset = snapshot.caret_char_offset;
        self.clipboard_text = snapshot.clipboard_text;

        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn restore_snapshot_native(&mut self, id: u32) -> Result<String> {
        self.restore_snapshot(id)
    }

    pub fn discard_snapshot(&mut self, id: u32) {
        self.snapshots.retain(|snapshot| snapshot.id != id);
    }

    pub fn discard_snapshot_native(&mut self, id: u32) {
        self.discard_snapshot(id);
    }

    pub fn set_show_paragraph_marks(&mut self, enabled: bool) {
        self.show_paragraph_marks = enabled;
    }

    pub fn set_show_paragraph_marks_native(&mut self, enabled: bool) {
        self.set_show_paragraph_marks(enabled);
    }

    pub fn get_show_control_codes(&self) -> bool {
        self.show_control_codes
    }

    pub fn get_show_control_codes_native(&self) -> bool {
        self.get_show_control_codes()
    }

    pub fn set_show_control_codes(&mut self, enabled: bool) {
        self.show_control_codes = enabled;
    }

    pub fn set_show_control_codes_native(&mut self, enabled: bool) {
        self.set_show_control_codes(enabled);
    }

    pub fn get_show_transparent_borders(&self) -> bool {
        self.show_transparent_borders
    }

    pub fn get_show_transparent_borders_native(&self) -> bool {
        self.get_show_transparent_borders()
    }

    pub fn set_show_transparent_borders(&mut self, enabled: bool) {
        self.show_transparent_borders = enabled;
    }

    pub fn set_show_transparent_borders_native(&mut self, enabled: bool) {
        self.set_show_transparent_borders(enabled);
    }

    pub fn set_clip_enabled(&mut self, enabled: bool) {
        self.clip_enabled = enabled;
    }

    pub fn set_clip_enabled_native(&mut self, enabled: bool) {
        self.set_clip_enabled(enabled);
    }

    pub fn get_position_of_page(&self, global_page: u32) -> Result<String> {
        let lines = self.page_lines(global_page)?;
        let paragraph_index = lines
            .iter()
            .find_map(PageTextLine::paragraph_index)
            .unwrap_or(0);
        self.paragraph_block_index(paragraph_index)?;
        Ok(format!(
            "{{\"ok\":true,\"sec\":0,\"para\":{},\"charOffset\":0}}",
            paragraph_index
        ))
    }

    pub fn get_position_of_page_native(&self, global_page: u32) -> Result<String> {
        self.get_position_of_page(global_page)
    }

    pub fn get_page_of_position(&self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        for (page_index, page) in self.pages.iter().enumerate() {
            if page
                .iter()
                .any(|line| line.paragraph_index() == Some(paragraph_idx as usize))
            {
                return Ok(format!("{{\"ok\":true,\"page\":{page_index}}}"));
            }
        }
        Ok("{\"ok\":true,\"page\":0}".to_string())
    }

    pub fn get_page_of_position_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String> {
        self.get_page_of_position(section_idx, paragraph_idx)
    }

    pub fn find_next_editable_control(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: i32,
        delta: i32,
    ) -> String {
        if self.ensure_section(section_idx).is_err()
            || self.paragraph_block_index(paragraph_idx as usize).is_err()
        {
            return "{\"type\":\"none\"}".to_string();
        }

        let paragraph_count = self.paragraph_count() as u32;
        if delta > 0 && paragraph_idx + 1 < paragraph_count {
            return format!(
                "{{\"type\":\"body\",\"sec\":{},\"para\":{}}}",
                section_idx,
                paragraph_idx + 1
            );
        }
        if delta < 0 && paragraph_idx > 0 {
            return format!(
                "{{\"type\":\"body\",\"sec\":{},\"para\":{}}}",
                section_idx,
                paragraph_idx - 1
            );
        }

        "{\"type\":\"none\"}".to_string()
    }

    pub fn find_next_editable_control_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: i32,
        delta: i32,
    ) -> String {
        self.find_next_editable_control(section_idx, paragraph_idx, control_idx, delta)
    }

    pub fn find_nearest_control_backward(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        if self.ensure_section(section_idx).is_err()
            || self.paragraph_block_index(paragraph_idx as usize).is_err()
        {
            return "{\"type\":\"none\"}".to_string();
        }

        let controls = projected_text_controls(&self.document);
        if let Some(control) = controls.iter().rev().find(|control| {
            control.paragraph_index < paragraph_idx as usize
                || (control.paragraph_index == paragraph_idx as usize
                    && control.char_offset < char_offset as usize)
        }) {
            return projected_control_json(control);
        }

        "{\"type\":\"none\"}".to_string()
    }

    pub fn find_nearest_control_backward_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        self.find_nearest_control_backward(section_idx, paragraph_idx, char_offset)
    }

    pub fn find_nearest_control_forward(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        if self.ensure_section(section_idx).is_err()
            || self.paragraph_block_index(paragraph_idx as usize).is_err()
        {
            return "{\"type\":\"none\"}".to_string();
        }

        let controls = projected_text_controls(&self.document);
        if let Some(control) = controls.iter().find(|control| {
            control.paragraph_index > paragraph_idx as usize
                || (control.paragraph_index == paragraph_idx as usize
                    && control.char_offset > char_offset as usize)
        }) {
            return projected_control_json(control);
        }

        "{\"type\":\"none\"}".to_string()
    }

    pub fn find_nearest_control_forward_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        self.find_nearest_control_forward(section_idx, paragraph_idx, char_offset)
    }

    pub fn get_control_text_positions(&self, section_idx: u32, paragraph_idx: u32) -> String {
        if self.ensure_section(section_idx).is_err()
            || self.paragraph_block_index(paragraph_idx as usize).is_err()
        {
            return "[]".to_string();
        }

        let positions = projected_text_controls(&self.document)
            .into_iter()
            .filter(|control| control.paragraph_index == paragraph_idx as usize)
            .map(|control| control.char_offset.to_string())
            .collect::<Vec<_>>();
        format!("[{}]", positions.join(","))
    }

    pub fn get_control_text_positions_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> String {
        self.get_control_text_positions(section_idx, paragraph_idx)
    }

    pub fn navigate_next_editable(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        delta: i32,
        _context_json: &str,
    ) -> String {
        if self.ensure_section(section_idx).is_err() {
            return "{\"type\":\"boundary\"}".to_string();
        }
        let Ok(paragraph) = self.paragraph(paragraph_idx as usize) else {
            return "{\"type\":\"boundary\"}".to_string();
        };

        let paragraph_len = paragraph_text(paragraph).chars().count() as u32;
        if delta > 0 {
            if char_offset < paragraph_len {
                return format_nav_text(section_idx, paragraph_idx, char_offset + 1);
            }
            if paragraph_idx + 1 < self.paragraph_count() as u32 {
                return format_nav_text(section_idx, paragraph_idx + 1, 0);
            }
        } else if delta < 0 {
            if char_offset > 0 {
                return format_nav_text(section_idx, paragraph_idx, char_offset - 1);
            }
            if paragraph_idx > 0 {
                let previous = self
                    .paragraph(paragraph_idx.saturating_sub(1) as usize)
                    .map(paragraph_text)
                    .unwrap_or_default()
                    .chars()
                    .count() as u32;
                return format_nav_text(section_idx, paragraph_idx - 1, previous);
            }
        }

        "{\"type\":\"boundary\"}".to_string()
    }

    pub fn navigate_next_editable_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        delta: i32,
        context_json: &str,
    ) -> String {
        self.navigate_next_editable(section_idx, paragraph_idx, char_offset, delta, context_json)
    }

    pub fn get_field_list(&self) -> String {
        "[]".to_string()
    }

    pub fn get_field_list_native(&self) -> String {
        self.get_field_list()
    }

    pub fn get_field_value(&self, field_id: u32) -> String {
        format!("{{\"ok\":false,\"fieldId\":{field_id},\"value\":\"\"}}")
    }

    pub fn get_field_value_native(&self, field_id: u32) -> String {
        self.get_field_value(field_id)
    }

    pub fn get_field_value_by_name(&self, name: &str) -> String {
        format!(
            "{{\"ok\":false,\"fieldId\":0,\"name\":{},\"value\":\"\"}}",
            json_string(name)
        )
    }

    pub fn get_field_value_by_name_native(&self, name: &str) -> String {
        self.get_field_value_by_name(name)
    }

    pub fn set_field_value(&mut self, field_id: u32, value: &str) -> String {
        format!(
            "{{\"ok\":false,\"fieldId\":{},\"oldValue\":\"\",\"newValue\":{}}}",
            field_id,
            json_string(value)
        )
    }

    pub fn set_field_value_native(&mut self, field_id: u32, value: &str) -> String {
        self.set_field_value(field_id, value)
    }

    pub fn set_field_value_by_name(&mut self, name: &str, value: &str) -> String {
        format!(
            "{{\"ok\":false,\"fieldId\":0,\"name\":{},\"oldValue\":\"\",\"newValue\":{}}}",
            json_string(name),
            json_string(value)
        )
    }

    pub fn set_field_value_by_name_native(&mut self, name: &str, value: &str) -> String {
        self.set_field_value_by_name(name, value)
    }

    pub fn get_field_info_at(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        if self
            .ensure_text_position(section_idx, paragraph_idx, char_offset)
            .is_err()
        {
            return "{\"inField\":false}".to_string();
        }
        "{\"inField\":false}".to_string()
    }

    pub fn get_field_info_at_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        self.get_field_info_at(section_idx, paragraph_idx, char_offset)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_field_info_at_in_cell(
        &self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
        _is_textbox: bool,
    ) -> String {
        "{\"inField\":false}".to_string()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_field_info_at_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> String {
        self.get_field_info_at_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }

    pub fn get_field_info_at_by_path(
        &self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _path_json: &str,
        _char_offset: u32,
    ) -> String {
        "{\"inField\":false}".to_string()
    }

    pub fn get_field_info_at_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> String {
        self.get_field_info_at_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    pub fn remove_field_at(
        &mut self,
        _section_idx: u32,
        _paragraph_idx: u32,
        _char_offset: u32,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn remove_field_at_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> String {
        self.remove_field_at(section_idx, paragraph_idx, char_offset)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn remove_field_at_in_cell(
        &mut self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
        _is_textbox: bool,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn remove_field_at_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> String {
        self.remove_field_at_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }

    pub fn set_active_field(
        &mut self,
        _section_idx: u32,
        _paragraph_idx: u32,
        _char_offset: u32,
    ) -> bool {
        false
    }

    pub fn set_active_field_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> bool {
        self.set_active_field(section_idx, paragraph_idx, char_offset)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_active_field_in_cell(
        &mut self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
        _is_textbox: bool,
    ) -> bool {
        false
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_active_field_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> bool {
        self.set_active_field_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }

    pub fn set_active_field_by_path(
        &mut self,
        _section_idx: u32,
        _parent_para_idx: u32,
        _path_json: &str,
        _char_offset: u32,
    ) -> bool {
        false
    }

    pub fn set_active_field_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> bool {
        self.set_active_field_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    pub fn clear_active_field(&mut self) {}

    pub fn clear_active_field_native(&mut self) {
        self.clear_active_field();
    }

    pub fn get_click_here_props(&self, _field_id: u32) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn get_click_here_props_native(&self, field_id: u32) -> String {
        self.get_click_here_props(field_id)
    }

    pub fn update_click_here_props(
        &mut self,
        _field_id: u32,
        _guide: &str,
        _memo: &str,
        _name: &str,
        _editable: bool,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn update_click_here_props_native(
        &mut self,
        field_id: u32,
        guide: &str,
        memo: &str,
        name: &str,
        editable: bool,
    ) -> String {
        self.update_click_here_props(field_id, guide, memo, name, editable)
    }

    pub fn get_header_footer(
        &self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":true,\"exists\":false}".to_string())
    }

    pub fn get_header_footer_native(
        &self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
    ) -> Result<String> {
        self.get_header_footer(section_idx, is_header, apply_to)
    }

    pub fn create_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false,\"exists\":false}".to_string())
    }

    pub fn create_header_footer_native(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
    ) -> Result<String> {
        self.create_header_footer(section_idx, is_header, apply_to)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_text_in_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _char_offset: u32,
        _text: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_text_in_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn split_paragraph_in_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn merge_paragraph_in_header_footer(
        &mut self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_header_footer_para_info(
        &self,
        section_idx: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false,\"paraCount\":0,\"charCount\":0}".to_string())
    }

    pub fn get_cursor_rect_in_header_footer(
        &self,
        page_num: u32,
        _is_header: bool,
        _apply_to: u32,
        _hf_para_idx: u32,
        _char_offset: u32,
        preferred_page: i32,
    ) -> Result<String> {
        self.page_lines(page_num)?;
        let page_index = if preferred_page >= 0 {
            preferred_page as u32
        } else {
            page_num
        };
        Ok(format!(
            "{{\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1},\"height\":{:.1}}}",
            page_index, APP_PAGE_MARGIN_PX, APP_PAGE_MARGIN_PX, APP_LINE_HEIGHT_PX
        ))
    }

    pub fn delete_header_footer(&mut self, _section_idx: u32, _is_header: bool, _apply_to: u32) {}

    pub fn get_header_footer_list(
        &self,
        _current_section_idx: u32,
        _current_is_header: bool,
        _current_apply_to: u32,
    ) -> String {
        "{\"ok\":true,\"items\":[],\"currentIndex\":-1}".to_string()
    }

    pub fn toggle_hide_header_footer(&mut self, page_num: u32, _is_header: bool) -> Result<String> {
        self.page_lines(page_num)?;
        Ok("{\"ok\":false,\"hidden\":false}".to_string())
    }

    pub fn navigate_header_footer_by_page(
        &self,
        _current_page: u32,
        _is_header: bool,
        _direction: i32,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn insert_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn insert_endnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_endnote_shape(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(default_endnote_shape_json())
    }

    pub fn apply_endnote_shape(&mut self, section_idx: u32, _props_json: &str) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_footnote_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok(
            "{\"ok\":false,\"paraCount\":0,\"totalTextLen\":0,\"number\":0,\"texts\":[]}"
                .to_string(),
        )
    }

    pub fn delete_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok("{\"ok\":false,\"sectionIndex\":0,\"paragraphIndex\":0,\"controlIndex\":0,\"charOffset\":0,\"deletedNumber\":0}".to_string())
    }

    pub fn get_page_footnote_info(&self, page_num: u32, _footnote_index: u32) -> Result<String> {
        self.page_lines(page_num)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_note_edit_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_note_equation_properties(
        &self,
        _section_idx: u32,
        _paragraph_idx: u32,
        _control_idx: u32,
        _note_para_idx: u32,
        _equation_idx: u32,
    ) -> String {
        "{\"ok\":false}".to_string()
    }

    pub fn set_note_equation_properties(
        &mut self,
        _section_idx: u32,
        _paragraph_idx: u32,
        _control_idx: u32,
        _note_para_idx: u32,
        _equation_idx: u32,
        _props_json: &str,
    ) -> String {
        "{\"ok\":false}".to_string()
    }
}
