use super::*;

impl DocumentCore {
    #[allow(clippy::too_many_arguments)]
    pub fn insert_text_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        char_offset: u32,
        _text: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_text_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String> {
        self.insert_text_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            text,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_text_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_text_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.delete_text_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            count,
        )
    }

    pub fn insert_text_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
        _text: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn insert_text_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        text: &str,
    ) -> Result<String> {
        self.insert_text_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset, text)
    }

    pub fn delete_text_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn delete_text_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.delete_text_in_cell_by_path(
            section_idx,
            parent_para_idx,
            path_json,
            char_offset,
            count,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_paragraph_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"cellParaIndex\":{cell_para_idx},\"charOffset\":{char_offset}}}"
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_paragraph_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.split_paragraph_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn split_paragraph_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"cellParaIndex\":0,\"charOffset\":{char_offset}}}"
        ))
    }

    pub fn split_paragraph_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.split_paragraph_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    pub fn merge_paragraph_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"cellParaIndex\":{cell_para_idx},\"charOffset\":0}}"
        ))
    }

    pub fn merge_paragraph_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String> {
        self.merge_paragraph_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    pub fn merge_paragraph_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false,\"cellParaIndex\":0,\"charOffset\":0}".to_string())
    }

    pub fn merge_paragraph_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String> {
        self.merge_paragraph_in_cell_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn paste_internal_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn paste_internal_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.paste_internal_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn get_cell_paragraph_count(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(|_| 1)
            .unwrap_or(0))
    }

    pub fn get_cell_paragraph_count_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<u32> {
        self.get_cell_paragraph_count(section_idx, parent_para_idx, control_idx, cell_idx)
    }

    pub fn get_cell_paragraph_length(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        if cell_para_idx != 0 {
            return Ok(0);
        }
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(|cell| cell.text_preview().chars().count() as u32)
            .unwrap_or(0))
    }

    pub fn get_cell_paragraph_length_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<u32> {
        self.get_cell_paragraph_length(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    pub fn get_cell_paragraph_count_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(0)
    }

    pub fn get_cell_paragraph_count_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<u32> {
        self.get_cell_paragraph_count_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_cell_paragraph_length_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(0)
    }

    pub fn get_cell_paragraph_length_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<u32> {
        self.get_cell_paragraph_length_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_cell_text_direction(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
    ) -> Result<u32> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(0)
    }

    pub fn get_cell_text_direction_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<u32> {
        self.get_cell_text_direction(section_idx, parent_para_idx, control_idx, cell_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_text_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        if cell_para_idx != 0 {
            return Ok(String::new());
        }
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(|cell| char_slice(cell.text_preview(), char_offset, count))
            .unwrap_or_default())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_text_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.get_text_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            count,
        )
    }

    pub fn get_text_in_cell_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        _char_offset: u32,
        _count: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(String::new())
    }

    pub fn get_text_in_cell_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        count: u32,
    ) -> Result<String> {
        self.get_text_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset, count)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_cursor_rect_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cursor_rect_json(0))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_cursor_rect_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_cursor_rect_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn get_cursor_rect_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cursor_rect_json(0))
    }

    pub fn get_cursor_rect_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.get_cursor_rect_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_line_info_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        if cell_para_idx != 0 {
            return Ok(default_line_info_json());
        }
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(observed_cell_line_info_json)
            .unwrap_or_else(default_line_info_json))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_line_info_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_line_info_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn get_table_dimensions(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(self
            .observed_table_candidate(control_idx)
            .map(observed_table_dimensions_json)
            .unwrap_or_else(default_table_dimensions_json))
    }

    pub fn get_table_dimensions_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_table_dimensions(section_idx, parent_para_idx, control_idx)
    }

    pub fn get_table_dimensions_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_dimensions_json())
    }

    pub fn get_table_dimensions_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String> {
        self.get_table_dimensions_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_cell_info(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(self
            .observed_table_cell(control_idx, cell_idx)
            .map(|cell| observed_cell_info_json(cell_idx, cell))
            .unwrap_or_else(default_cell_info_json))
    }

    pub fn get_cell_info_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<String> {
        self.get_cell_info(section_idx, parent_para_idx, control_idx, cell_idx)
    }

    pub fn get_cell_info_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_info_json())
    }

    pub fn get_cell_info_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String> {
        self.get_cell_info_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_cell_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_properties_json())
    }

    pub fn get_cell_properties_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<String> {
        self.get_cell_properties(section_idx, parent_para_idx, control_idx, cell_idx)
    }

    pub fn set_cell_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_cell_properties_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.set_cell_properties(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            props_json,
        )
    }

    pub fn resize_table_cells(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _updates_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn resize_table_cells_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        updates_json: &str,
    ) -> Result<String> {
        self.resize_table_cells(section_idx, parent_para_idx, control_idx, updates_json)
    }

    pub fn move_table_offset(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        _delta_h: i32,
        _delta_v: i32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"ppi\":{},\"ci\":{}}}",
            parent_para_idx, control_idx
        ))
    }

    pub fn move_table_offset_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        delta_h: i32,
        delta_v: i32,
    ) -> Result<String> {
        self.move_table_offset(section_idx, parent_para_idx, control_idx, delta_h, delta_v)
    }

    pub fn get_table_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_properties_json())
    }

    pub fn get_table_properties_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_table_properties(section_idx, parent_para_idx, control_idx)
    }

    pub fn set_table_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_table_properties_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.set_table_properties(section_idx, parent_para_idx, control_idx, props_json)
    }

    pub fn get_table_cell_bboxes(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _page_hint: Option<u32>,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("[]".to_string())
    }

    pub fn get_table_cell_bboxes_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        page_hint: Option<u32>,
    ) -> Result<String> {
        self.get_table_cell_bboxes(section_idx, parent_para_idx, control_idx, page_hint)
    }

    pub fn get_table_cell_bboxes_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("[]".to_string())
    }

    pub fn get_table_cell_bboxes_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String> {
        self.get_table_cell_bboxes_by_path(section_idx, parent_para_idx, path_json)
    }

    pub fn get_table_bbox(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"pageIndex\":0,\"x\":0.0,\"y\":0.0,\"width\":0.0,\"height\":0.0}".to_string())
    }

    pub fn get_table_bbox_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_table_bbox(section_idx, parent_para_idx, control_idx)
    }

    pub fn create_table(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _rows: u32,
        _cols: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{},\"controlIdx\":-1}}",
            paragraph_idx
        ))
    }

    pub fn create_table_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        rows: u32,
        cols: u32,
    ) -> Result<String> {
        self.create_table(section_idx, paragraph_idx, char_offset, rows, cols)
    }

    pub fn delete_table_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn delete_table_control_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.delete_table_control(section_idx, parent_para_idx, control_idx)
    }

    pub fn insert_table_row(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _row_idx: u32,
        _below: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_edit_result_json())
    }

    pub fn insert_table_row_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row_idx: u32,
        below: bool,
    ) -> Result<String> {
        self.insert_table_row(section_idx, parent_para_idx, control_idx, row_idx, below)
    }

    pub fn insert_table_column(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _col_idx: u32,
        _right: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_edit_result_json())
    }

    pub fn insert_table_column_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        col_idx: u32,
        right: bool,
    ) -> Result<String> {
        self.insert_table_column(section_idx, parent_para_idx, control_idx, col_idx, right)
    }

    pub fn delete_table_row(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _row_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_edit_result_json())
    }

    pub fn delete_table_row_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row_idx: u32,
    ) -> Result<String> {
        self.delete_table_row(section_idx, parent_para_idx, control_idx, row_idx)
    }

    pub fn delete_table_column(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _col_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_table_edit_result_json())
    }

    pub fn delete_table_column_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        col_idx: u32,
    ) -> Result<String> {
        self.delete_table_column(section_idx, parent_para_idx, control_idx, col_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn merge_table_cells(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _start_row: u32,
        _start_col: u32,
        _end_row: u32,
        _end_col: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_count_result_json())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn merge_table_cells_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
    ) -> Result<String> {
        self.merge_table_cells(
            section_idx,
            parent_para_idx,
            control_idx,
            start_row,
            start_col,
            end_row,
            end_col,
        )
    }

    pub fn split_table_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _row: u32,
        _col: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_count_result_json())
    }

    pub fn split_table_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row: u32,
        col: u32,
    ) -> Result<String> {
        self.split_table_cell(section_idx, parent_para_idx, control_idx, row, col)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_table_cell_into(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _row: u32,
        _col: u32,
        _n_rows: u32,
        _m_cols: u32,
        _equal_row_height: bool,
        _merge_first: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_count_result_json())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_table_cell_into_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row: u32,
        col: u32,
        n_rows: u32,
        m_cols: u32,
        equal_row_height: bool,
        merge_first: bool,
    ) -> Result<String> {
        self.split_table_cell_into(
            section_idx,
            parent_para_idx,
            control_idx,
            row,
            col,
            n_rows,
            m_cols,
            equal_row_height,
            merge_first,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_table_cells_in_range(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _start_row: u32,
        _start_col: u32,
        _end_row: u32,
        _end_col: u32,
        _n_rows: u32,
        _m_cols: u32,
        _equal_row_height: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_cell_count_result_json())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_table_cells_in_range_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
        n_rows: u32,
        m_cols: u32,
        equal_row_height: bool,
    ) -> Result<String> {
        self.split_table_cells_in_range(
            section_idx,
            parent_para_idx,
            control_idx,
            start_row,
            start_col,
            end_row,
            end_col,
            n_rows,
            m_cols,
            equal_row_height,
        )
    }

    pub fn get_column_def(&self, section_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"columnCount\":1,\"columnType\":0,\"sameWidth\":true,\"spacing\":0}".to_string())
    }

    pub fn get_column_def_native(&self, section_idx: u32) -> Result<String> {
        self.get_column_def(section_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_selection_rects_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _start_cell_para_idx: u32,
        _start_char_offset: u32,
        _end_cell_para_idx: u32,
        _end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("[]".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_selection_rects_in_cell_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.get_selection_rects_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            start_cell_para_idx,
            start_char_offset,
            end_cell_para_idx,
            end_char_offset,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn copy_selection_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _start_cell_para_idx: u32,
        _start_char_offset: u32,
        _end_cell_para_idx: u32,
        _end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false,\"text\":\"\"}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn copy_selection_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.copy_selection_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            start_cell_para_idx,
            start_char_offset,
            end_cell_para_idx,
            end_char_offset,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_range_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        _end_cell_para_idx: u32,
        _end_char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{start_cell_para_idx},\"charOffset\":{start_char_offset}}}"
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn delete_range_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String> {
        self.delete_range_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            start_cell_para_idx,
            start_char_offset,
            end_cell_para_idx,
            end_char_offset,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_cell_char_properties_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_char_properties_json())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_cell_char_properties_at_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.get_cell_char_properties_at(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
        )
    }

    pub fn get_cell_para_properties_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_para_properties_json())
    }

    pub fn get_cell_para_properties_at_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String> {
        self.get_cell_para_properties_at(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_char_format_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        start_offset: u32,
        end_offset: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        if start_offset > end_offset {
            return Err(rjtd_core::Error::InvalidData(format!(
                "start offset {start_offset} is after end offset {end_offset}"
            )));
        }
        Ok("{\"ok\":false}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_char_format_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        start_offset: u32,
        end_offset: u32,
        props_json: &str,
    ) -> Result<String> {
        self.apply_char_format_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            start_offset,
            end_offset,
            props_json,
        )
    }

    pub fn apply_para_format_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn apply_para_format_in_cell_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.apply_para_format_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            props_json,
        )
    }

    pub fn get_cell_style_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"id\":0,\"name\":\"Normal\"}".to_string())
    }

    pub fn get_cell_style_at_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String> {
        self.get_cell_style_at(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
        )
    }

    pub fn apply_cell_style(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
        _style_id: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn apply_cell_style_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        style_id: u32,
    ) -> Result<String> {
        self.apply_cell_style(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            style_id,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_table_formula(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _target_row: u32,
        _target_col: u32,
        formula: &str,
        _write_result: bool,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"value\":\"\",\"formula\":{}}}",
            json_string(formula)
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_table_formula_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        target_row: u32,
        target_col: u32,
        formula: &str,
        write_result: bool,
    ) -> Result<String> {
        self.evaluate_table_formula(
            section_idx,
            parent_para_idx,
            control_idx,
            target_row,
            target_col,
            formula,
            write_result,
        )
    }

    pub fn paste_internal_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(format!(
            "{{\"ok\":false,\"cellParaIdx\":0,\"charOffset\":{char_offset}}}"
        ))
    }

    pub fn paste_internal_in_cell_by_path_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String> {
        self.paste_internal_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }

    pub fn move_vertical_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _path_json: &str,
        char_offset: u32,
        _delta: i32,
        preferred_x: f64,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        let x = if preferred_x.is_finite() && preferred_x >= 0.0 {
            preferred_x
        } else {
            APP_PAGE_MARGIN_PX as f64
        };
        Ok(format!(
            "{{\"sectionIndex\":{},\"paragraphIndex\":{},\"charOffset\":{},\"pageIndex\":0,\"x\":{:.1},\"y\":{:.1},\"height\":{:.1},\"preferredX\":{:.1},\"rectValid\":false}}",
            section_idx, parent_para_idx, char_offset, x, APP_PAGE_MARGIN_PX, APP_LINE_HEIGHT_PX, x
        ))
    }

    pub fn move_vertical_by_path_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        delta: i32,
        preferred_x: f64,
    ) -> Result<String> {
        self.move_vertical_by_path(
            section_idx,
            parent_para_idx,
            path_json,
            char_offset,
            delta,
            preferred_x,
        )
    }

    pub fn get_table_signature(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(self
            .observed_table_candidate(control_idx)
            .map(observed_table_signature)
            .unwrap_or_default())
    }

    pub fn get_table_signature_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_table_signature(section_idx, parent_para_idx, control_idx)
    }

    pub fn get_paragraph_stable_id(&self, section_idx: u32, paragraph_idx: u32) -> Result<String> {
        self.ensure_section(section_idx)?;
        self.paragraph_block_index(paragraph_idx as usize)?;
        Ok(format!("rjtd-p{paragraph_idx}"))
    }

    pub fn get_paragraph_stable_id_native(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String> {
        self.get_paragraph_stable_id(section_idx, paragraph_idx)
    }

    pub fn ensure_paragraph_stable_ids(&mut self) {}

    pub fn ensure_paragraph_stable_ids_native(&mut self) {
        self.ensure_paragraph_stable_ids();
    }

    pub fn debug_dump_stable_ids(
        &self,
        section_idx: u32,
        start_para: u32,
        count: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        let end = start_para.saturating_add(count);
        let mut items = Vec::new();
        for para_idx in start_para..end {
            if self.paragraph_block_index(para_idx as usize).is_ok() {
                items.push(format!(
                    "{{\"sec\":{},\"para\":{},\"stableId\":\"rjtd-p{}\"}}",
                    section_idx, para_idx, para_idx
                ));
            }
        }
        Ok(format!("[{}]", items.join(",")))
    }

    pub fn debug_dump_stable_ids_native(
        &self,
        section_idx: u32,
        start_para: u32,
        count: u32,
    ) -> Result<String> {
        self.debug_dump_stable_ids(section_idx, start_para, count)
    }
}
