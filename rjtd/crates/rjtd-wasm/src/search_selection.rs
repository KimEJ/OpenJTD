use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = searchText)]
    pub fn search_text(
        &self,
        query: &str,
        from_sec: u32,
        from_para: u32,
        from_char: u32,
        forward: bool,
        case_sensitive: bool,
    ) -> Result<String, JsValue> {
        self.core
            .search_text(
                query,
                from_sec,
                from_para,
                from_char,
                forward,
                case_sensitive,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = searchAllText)]
    pub fn search_all_text(
        &self,
        query: &str,
        case_sensitive: bool,
        include_cells: bool,
    ) -> String {
        self.core
            .search_all_text(query, case_sensitive, include_cells)
    }
    #[wasm_bindgen(js_name = replaceText)]
    pub fn replace_text(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
        length: u32,
        new_text: &str,
    ) -> Result<String, JsValue> {
        self.core
            .replace_text(section_idx, para_idx, char_offset, length, new_text)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = replaceOne)]
    pub fn replace_one(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String, JsValue> {
        self.core
            .replace_one(query, new_text, case_sensitive)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = replaceAll)]
    pub fn replace_all(
        &mut self,
        query: &str,
        new_text: &str,
        case_sensitive: bool,
    ) -> Result<String, JsValue> {
        self.core
            .replace_all(query, new_text, case_sensitive)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getSelectionRects)]
    pub fn get_selection_rects(
        &self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_selection_rects(
                section_idx,
                start_para_idx,
                start_char_offset,
                end_para_idx,
                end_char_offset,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = deleteRange)]
    pub fn delete_range(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_range(
                section_idx,
                start_para_idx,
                start_char_offset,
                end_para_idx,
                end_char_offset,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = copySelection)]
    pub fn copy_selection(
        &mut self,
        section_idx: u32,
        start_para_idx: u32,
        start_char_offset: u32,
        end_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .copy_selection(
                section_idx,
                start_para_idx,
                start_char_offset,
                end_para_idx,
                end_char_offset,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = pasteInternal)]
    pub fn paste_internal(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .paste_internal(section_idx, para_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = hasInternalClipboard)]
    pub fn has_internal_clipboard(&self) -> bool {
        self.core.has_internal_clipboard()
    }
    #[wasm_bindgen(js_name = getClipboardText)]
    pub fn get_clipboard_text(&self) -> String {
        self.core.get_clipboard_text()
    }
    #[wasm_bindgen(js_name = clearClipboard)]
    pub fn clear_clipboard(&mut self) {
        self.core.clear_clipboard();
    }
    #[wasm_bindgen(js_name = clipboardHasControl)]
    pub fn clipboard_has_control(&self) -> bool {
        self.core.clipboard_has_control()
    }
}
