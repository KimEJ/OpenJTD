use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = exportSelectionHtml)]
    pub fn export_selection_html(
        &self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .export_selection_html(
                section_idx,
                start_para_idx,
                start_char_offset,
                end_para_idx,
                end_char_offset,
            )
            .map_err(js_error)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = exportSelectionInCellHtml)]
    pub fn export_selection_in_cell_html(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para: u32,
        start_offset: u32,
        end_cell_para: u32,
        end_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .export_selection_in_cell_html(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                start_cell_para,
                start_offset,
                end_cell_para,
                end_offset,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = exportControlHtml)]
    pub fn export_control_html(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        cell_path_json: &str,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .export_control_html(section_idx, paragraph_idx, cell_path_json, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = pasteHtml)]
    pub fn paste_html(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        html: &str,
    ) -> Result<String, JsValue> {
        self.core
            .paste_html(section_idx, paragraph_idx, char_offset, html)
            .map_err(js_error)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = pasteHtmlInCell)]
    pub fn paste_html_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        html: &str,
    ) -> Result<String, JsValue> {
        self.core
            .paste_html_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
                html,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = pasteHtmlInCellByPath)]
    pub fn paste_html_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        html: &str,
    ) -> Result<String, JsValue> {
        self.core
            .paste_html_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset, html)
            .map_err(js_error)
    }
}
