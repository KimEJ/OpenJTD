use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = getParaPropertiesInHf)]
    pub fn get_para_properties_in_hf(
        &self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_para_properties_in_hf(section_idx, is_header, apply_to, hf_para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = applyParaFormatInHf)]
    pub fn apply_para_format_in_hf(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .apply_para_format_in_hf(section_idx, is_header, apply_to, hf_para_idx, props_json)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = insertFieldInHf)]
    pub fn insert_field_in_hf(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
        char_offset: u32,
        field_type: u32,
    ) -> Result<String, JsValue> {
        self.core
            .insert_field_in_hf(
                section_idx,
                is_header,
                apply_to,
                hf_para_idx,
                char_offset,
                field_type,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = applyHfTemplate)]
    pub fn apply_hf_template(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        template_id: u32,
    ) -> Result<String, JsValue> {
        self.core
            .apply_hf_template(section_idx, is_header, apply_to, template_id)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getHeaderFooter)]
    pub fn get_header_footer(
        &self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_header_footer(section_idx, is_header, apply_to)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = createHeaderFooter)]
    pub fn create_header_footer(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
    ) -> Result<String, JsValue> {
        self.core
            .create_header_footer(section_idx, is_header, apply_to)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = insertTextInHeaderFooter)]
    pub fn insert_text_in_header_footer(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String, JsValue> {
        self.core
            .insert_text_in_header_footer(
                section_idx,
                is_header,
                apply_to,
                hf_para_idx,
                char_offset,
                text,
            )
            .map_err(js_error)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = deleteTextInHeaderFooter)]
    pub fn delete_text_in_header_footer(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_text_in_header_footer(
                section_idx,
                is_header,
                apply_to,
                hf_para_idx,
                char_offset,
                count,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = splitParagraphInHeaderFooter)]
    pub fn split_paragraph_in_header_footer(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .split_paragraph_in_header_footer(
                section_idx,
                is_header,
                apply_to,
                hf_para_idx,
                char_offset,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = mergeParagraphInHeaderFooter)]
    pub fn merge_paragraph_in_header_footer(
        &mut self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .merge_paragraph_in_header_footer(section_idx, is_header, apply_to, hf_para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getHeaderFooterParaInfo)]
    pub fn get_header_footer_para_info(
        &self,
        section_idx: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_header_footer_para_info(section_idx, is_header, apply_to, hf_para_idx)
            .map_err(js_error)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = getCursorRectInHeaderFooter)]
    pub fn get_cursor_rect_in_header_footer(
        &self,
        page_num: u32,
        is_header: bool,
        apply_to: u32,
        hf_para_idx: u32,
        char_offset: u32,
        preferred_page: i32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cursor_rect_in_header_footer(
                page_num,
                is_header,
                apply_to,
                hf_para_idx,
                char_offset,
                preferred_page,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = deleteHeaderFooter)]
    pub fn delete_header_footer(&mut self, section_idx: u32, is_header: bool, apply_to: u32) {
        self.core
            .delete_header_footer(section_idx, is_header, apply_to);
    }
    #[wasm_bindgen(js_name = getHeaderFooterList)]
    pub fn get_header_footer_list(
        &self,
        current_section_idx: u32,
        current_is_header: bool,
        current_apply_to: u32,
    ) -> String {
        self.core
            .get_header_footer_list(current_section_idx, current_is_header, current_apply_to)
    }
    #[wasm_bindgen(js_name = toggleHideHeaderFooter)]
    pub fn toggle_hide_header_footer(
        &mut self,
        page_num: u32,
        is_header: bool,
    ) -> Result<String, JsValue> {
        self.core
            .toggle_hide_header_footer(page_num, is_header)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = navigateHeaderFooterByPage)]
    pub fn navigate_header_footer_by_page(
        &self,
        current_page: u32,
        is_header: bool,
        direction: i32,
    ) -> String {
        self.core
            .navigate_header_footer_by_page(current_page, is_header, direction)
    }
}
