use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = getParagraphStableId)]
    pub fn get_paragraph_stable_id(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_paragraph_stable_id(section_idx, paragraph_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = ensureParagraphStableIds)]
    pub fn ensure_paragraph_stable_ids(&mut self) {
        self.core.ensure_paragraph_stable_ids();
    }
    #[wasm_bindgen(js_name = debugDumpStableIds)]
    pub fn debug_dump_stable_ids(
        &self,
        section_idx: u32,
        start_para: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .debug_dump_stable_ids(section_idx, start_para, count)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = copyControl)]
    pub fn copy_control(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        cell_path_json: &str,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .copy_control(section_idx, paragraph_idx, cell_path_json, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = pasteControl)]
    pub fn paste_control(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .paste_control(section_idx, paragraph_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getControlImageData)]
    pub fn get_control_image_data(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        cell_path_json: &str,
        control_idx: u32,
    ) -> Result<Vec<u8>, JsValue> {
        self.core
            .get_control_image_data(section_idx, paragraph_idx, cell_path_json, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getControlImageMime)]
    pub fn get_control_image_mime(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        cell_path_json: &str,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_control_image_mime(section_idx, paragraph_idx, cell_path_json, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getBookmarks)]
    pub fn get_bookmarks(&self) -> String {
        self.core.get_bookmarks()
    }
    #[wasm_bindgen(js_name = addBookmark)]
    pub fn add_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        name: &str,
    ) -> Result<String, JsValue> {
        self.core
            .add_bookmark(section_idx, paragraph_idx, char_offset, name)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = deleteBookmark)]
    pub fn delete_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_bookmark(section_idx, paragraph_idx, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = renameBookmark)]
    pub fn rename_bookmark(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        new_name: &str,
    ) -> Result<String, JsValue> {
        self.core
            .rename_bookmark(section_idx, paragraph_idx, control_idx, new_name)
            .map_err(js_error)
    }
}
