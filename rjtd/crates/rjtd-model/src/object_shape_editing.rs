use super::*;

impl DocumentCore {
    pub fn get_shape_bbox(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_object_bbox_json())
    }

    pub fn get_shape_bbox_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_shape_bbox(section_idx, parent_para_idx, control_idx)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_picture(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        cell_path_json: &str,
        _image_data: &[u8],
        _width: u32,
        _height: u32,
        _natural_width_px: u32,
        _natural_height_px: u32,
        _extension: &str,
        _description: &str,
        _paper_offset_x_hu: Option<i32>,
        _paper_offset_y_hu: Option<i32>,
    ) -> Result<String> {
        if cell_path_json.is_empty() || cell_path_json == "[]" {
            self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        } else {
            self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        }
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{},\"controlIdx\":-1}}",
            paragraph_idx
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_picture_native(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        cell_path_json: &str,
        image_data: &[u8],
        width: u32,
        height: u32,
        natural_width_px: u32,
        natural_height_px: u32,
        extension: &str,
        description: &str,
        paper_offset_x_hu: Option<i32>,
        paper_offset_y_hu: Option<i32>,
    ) -> Result<String> {
        self.insert_picture(
            section_idx,
            paragraph_idx,
            char_offset,
            cell_path_json,
            image_data,
            width,
            height,
            natural_width_px,
            natural_height_px,
            extension,
            description,
            paper_offset_x_hu,
            paper_offset_y_hu,
        )
    }

    pub fn get_picture_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_picture_properties_json())
    }

    pub fn get_picture_properties_native(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.get_picture_properties(section_idx, parent_para_idx, control_idx)
    }

    pub fn get_header_footer_picture_properties(
        &self,
        section_idx: u32,
        _outer_para_idx: u32,
        _outer_control_idx: u32,
        _inner_para_idx: u32,
        _inner_control_idx: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(default_picture_properties_json())
    }

    pub fn get_header_footer_picture_properties_native(
        &self,
        section_idx: u32,
        outer_para_idx: u32,
        outer_control_idx: u32,
        inner_para_idx: u32,
        inner_control_idx: u32,
    ) -> Result<String> {
        self.get_header_footer_picture_properties(
            section_idx,
            outer_para_idx,
            outer_control_idx,
            inner_para_idx,
            inner_control_idx,
        )
    }

    pub fn set_picture_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_picture_properties_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.set_picture_properties(section_idx, parent_para_idx, control_idx, props_json)
    }

    pub fn set_header_footer_picture_properties(
        &mut self,
        section_idx: u32,
        _outer_para_idx: u32,
        _outer_control_idx: u32,
        _inner_para_idx: u32,
        _inner_control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_header_footer_picture_properties_native(
        &mut self,
        section_idx: u32,
        outer_para_idx: u32,
        outer_control_idx: u32,
        inner_para_idx: u32,
        inner_control_idx: u32,
        props_json: &str,
    ) -> Result<String> {
        self.set_header_footer_picture_properties(
            section_idx,
            outer_para_idx,
            outer_control_idx,
            inner_para_idx,
            inner_control_idx,
            props_json,
        )
    }

    pub fn delete_picture_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn delete_picture_control_native(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String> {
        self.delete_picture_control(section_idx, parent_para_idx, control_idx)
    }

    pub fn delete_cell_picture_control_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_cell_shape_properties_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_shape_properties_json())
    }

    pub fn get_cell_picture_properties_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_picture_properties_json())
    }

    pub fn set_cell_shape_properties_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_cell_picture_properties_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _cell_path_json: &str,
        _inner_control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn delete_equation_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_equation_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: i32,
        _cell_para_idx: i32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_equation_properties_json())
    }

    pub fn set_equation_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: i32,
        _cell_para_idx: i32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn render_equation_preview(
        &self,
        script: &str,
        font_size_hwpunit: u32,
        color: u32,
    ) -> String {
        let font_size = (font_size_hwpunit as f64 / 100.0).clamp(8.0, 96.0);
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"320\" height=\"80\" viewBox=\"0 0 320 80\"><rect width=\"320\" height=\"80\" fill=\"#ffffff\"/><text x=\"12\" y=\"46\" font-family=\"serif\" font-size=\"{font_size:.1}\" fill=\"#{color:06x}\">{}</text></svg>",
            escape_xml(script)
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_shape_control(&mut self, _params_json: &str) -> Result<String> {
        Ok("{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}".to_string())
    }

    pub fn get_shape_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok(default_shape_properties_json())
    }

    pub fn get_shape_text(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false,\"text\":\"\"}".to_string())
    }

    pub fn set_shape_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _props_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn delete_shape_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn change_shape_z_order(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _operation: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false,\"zOrder\":0}".to_string())
    }

    pub fn group_shapes(&mut self, _json: &str) -> String {
        "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}".to_string()
    }

    pub fn ungroup_shape(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn move_line_endpoint(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        _control_idx: u32,
        _sx: i32,
        _sy: i32,
        _ex: i32,
        _ey: i32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, parent_para_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn update_connectors_in_section(&mut self, _section_idx: u32) {}

    pub fn insert_equation(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _script: &str,
        _font_size: u32,
        _color: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{},\"controlIdx\":-1}}",
            paragraph_idx
        ))
    }

    pub fn get_form_object_at(&self, page_num: u32, _x: f64, _y: f64) -> Result<String> {
        self.page_lines(page_num)?;
        Ok("{\"found\":false}".to_string())
    }

    pub fn get_form_value(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn set_form_value(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _value_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_form_value_in_cell(
        &mut self,
        section_idx: u32,
        table_para: u32,
        _table_ci: u32,
        _cell_idx: u32,
        _cell_para: u32,
        _form_ci: u32,
        _value_json: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, table_para)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn get_form_object_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn copy_control(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _cell_path_json: &str,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn paste_control(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!(
            "{{\"ok\":false,\"paraIdx\":{},\"controlIdx\":-1}}",
            paragraph_idx
        ))
    }

    pub fn get_control_image_data(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _cell_path_json: &str,
        _control_idx: u32,
    ) -> Result<Vec<u8>> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(Vec::new())
    }

    pub fn get_control_image_mime(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        _cell_path_json: &str,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok(String::new())
    }

    pub fn get_bookmarks(&self) -> String {
        "[]".to_string()
    }

    pub fn add_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _name: &str,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok("{\"ok\":false,\"error\":\"bookmarks are not decoded\"}".to_string())
    }

    pub fn delete_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false,\"error\":\"bookmarks are not decoded\"}".to_string())
    }

    pub fn rename_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _control_idx: u32,
        _new_name: &str,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false,\"error\":\"bookmarks are not decoded\"}".to_string())
    }

    pub fn export_hwp(&self) -> Vec<u8> {
        Vec::new()
    }

    pub fn export_hwpx(&self) -> Vec<u8> {
        Vec::new()
    }

    pub fn export_hwp_verify(&self) -> String {
        "{\"ok\":false,\"errors\":[\"JTD to HWP/HWPX export is not implemented\"],\"warnings\":[]}"
            .to_string()
    }

    pub fn insert_page_break(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn insert_column_break(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn insert_new_number(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        _start_num: u32,
    ) -> Result<String> {
        self.ensure_text_position(section_idx, paragraph_idx, char_offset)?;
        Ok(format!("{{\"ok\":false,\"charOffset\":{char_offset}}}"))
    }

    pub fn set_column_def(
        &mut self,
        section_idx: u32,
        _column_count: u32,
        _column_type: u32,
        _same_width: u32,
        _spacing_hu: u32,
    ) -> Result<String> {
        self.ensure_section(section_idx)?;
        Ok(ok_page_count_json(self.page_count()))
    }

    pub fn set_numbering_restart(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        _mode: u32,
        _start_num: u32,
    ) -> Result<String> {
        self.ensure_parent_paragraph(section_idx, paragraph_idx)?;
        Ok("{\"ok\":false}".to_string())
    }

    pub fn create_style(&mut self, _json: &str) -> u32 {
        0
    }

    pub fn update_style(&mut self, style_id: u32, _json: &str) -> bool {
        style_id == 0
    }

    pub fn update_style_shapes(
        &mut self,
        style_id: u32,
        _char_mods_json: &str,
        _para_mods_json: &str,
    ) -> bool {
        style_id == 0
    }

    pub fn delete_style(&mut self, _style_id: u32) -> bool {
        false
    }

    pub fn create_numbering(&mut self, _json: &str) -> u32 {
        0
    }
}
