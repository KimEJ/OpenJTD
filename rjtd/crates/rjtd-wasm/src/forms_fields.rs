use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = getFormObjectAt)]
    pub fn get_form_object_at(&self, page_num: u32, x: f64, y: f64) -> Result<String, JsValue> {
        self.core
            .get_form_object_at(page_num, x, y)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getFormValue)]
    pub fn get_form_value(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_form_value(section_idx, paragraph_idx, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = setFormValue)]
    pub fn set_form_value(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        value_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_form_value(section_idx, paragraph_idx, control_idx, value_json)
            .map_err(js_error)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = setFormValueInCell)]
    pub fn set_form_value_in_cell(
        &mut self,
        section_idx: u32,
        table_para: u32,
        table_ci: u32,
        cell_idx: u32,
        cell_para: u32,
        form_ci: u32,
        value_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_form_value_in_cell(
                section_idx,
                table_para,
                table_ci,
                cell_idx,
                cell_para,
                form_ci,
                value_json,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getFormObjectInfo)]
    pub fn get_form_object_info(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_form_object_info(section_idx, paragraph_idx, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getFieldList)]
    pub fn get_field_list(&self) -> String {
        self.core.get_field_list()
    }
    #[wasm_bindgen(js_name = getFieldValue)]
    pub fn get_field_value(&self, field_id: u32) -> String {
        self.core.get_field_value(field_id)
    }
    #[wasm_bindgen(js_name = getFieldValueByName)]
    pub fn get_field_value_by_name(&self, name: &str) -> String {
        self.core.get_field_value_by_name(name)
    }
    #[wasm_bindgen(js_name = setFieldValue)]
    pub fn set_field_value(&mut self, field_id: u32, value: &str) -> String {
        self.core.set_field_value(field_id, value)
    }
    #[wasm_bindgen(js_name = setFieldValueByName)]
    pub fn set_field_value_by_name(&mut self, name: &str, value: &str) -> String {
        self.core.set_field_value_by_name(name, value)
    }
    #[wasm_bindgen(js_name = getFieldInfoAt)]
    pub fn get_field_info_at(&self, section_idx: u32, para_idx: u32, char_offset: u32) -> String {
        self.core
            .get_field_info_at(section_idx, para_idx, char_offset)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = getFieldInfoAtInCell)]
    pub fn get_field_info_at_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> String {
        self.core.get_field_info_at_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }
    #[wasm_bindgen(js_name = getFieldInfoAtByPath)]
    pub fn get_field_info_at_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> String {
        self.core
            .get_field_info_at_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }
    #[wasm_bindgen(js_name = removeFieldAt)]
    pub fn remove_field_at(&mut self, section_idx: u32, para_idx: u32, char_offset: u32) -> String {
        self.core
            .remove_field_at(section_idx, para_idx, char_offset)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = removeFieldAtInCell)]
    pub fn remove_field_at_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> String {
        self.core.remove_field_at_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }
    #[wasm_bindgen(js_name = setActiveField)]
    pub fn set_active_field(&mut self, section_idx: u32, para_idx: u32, char_offset: u32) -> bool {
        self.core
            .set_active_field(section_idx, para_idx, char_offset)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = setActiveFieldInCell)]
    pub fn set_active_field_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        is_textbox: bool,
    ) -> bool {
        self.core.set_active_field_in_cell(
            section_idx,
            parent_para_idx,
            control_idx,
            cell_idx,
            cell_para_idx,
            char_offset,
            is_textbox,
        )
    }
    #[wasm_bindgen(js_name = setActiveFieldByPath)]
    pub fn set_active_field_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> bool {
        self.core
            .set_active_field_by_path(section_idx, parent_para_idx, path_json, char_offset)
    }
    #[wasm_bindgen(js_name = clearActiveField)]
    pub fn clear_active_field(&mut self) {
        self.core.clear_active_field();
    }
    #[wasm_bindgen(js_name = getClickHereProps)]
    pub fn get_click_here_props(&self, field_id: u32) -> String {
        self.core.get_click_here_props(field_id)
    }
    #[wasm_bindgen(js_name = updateClickHereProps)]
    pub fn update_click_here_props(
        &mut self,
        field_id: u32,
        guide: &str,
        memo: &str,
        name: &str,
        editable: bool,
    ) -> String {
        self.core
            .update_click_here_props(field_id, guide, memo, name, editable)
    }
}
