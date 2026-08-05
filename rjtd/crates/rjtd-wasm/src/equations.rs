use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = deleteEquationControl)]
    pub fn delete_equation_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_equation_control(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getEquationProperties)]
    pub fn get_equation_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: i32,
        cell_para_idx: i32,
    ) -> Result<String, JsValue> {
        self.core
            .get_equation_properties(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = setEquationProperties)]
    pub fn set_equation_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: i32,
        cell_para_idx: i32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_equation_properties(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                props_json,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = renderEquationPreview)]
    pub fn render_equation_preview(
        &self,
        script: &str,
        font_size_hwpunit: u32,
        color: u32,
    ) -> String {
        self.core
            .render_equation_preview(script, font_size_hwpunit, color)
    }
    #[wasm_bindgen(js_name = insertEquation)]
    pub fn insert_equation(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        script: &str,
        font_size: u32,
        color: u32,
    ) -> Result<String, JsValue> {
        self.core
            .insert_equation(
                section_idx,
                paragraph_idx,
                char_offset,
                script,
                font_size,
                color,
            )
            .map_err(js_error)
    }
}
