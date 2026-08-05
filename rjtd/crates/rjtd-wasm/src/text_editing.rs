use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = plainText)]
    pub fn plain_text(&self) -> String {
        self.core.plain_text()
    }
    #[wasm_bindgen(js_name = insertText)]
    pub fn insert_text(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String, JsValue> {
        self.core
            .insert_text(section_idx, para_idx, char_offset, text)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = deleteText)]
    pub fn delete_text(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_text(section_idx, para_idx, char_offset, count)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = splitParagraph)]
    pub fn split_paragraph(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .split_paragraph(section_idx, para_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = mergeParagraph)]
    pub fn merge_paragraph(&mut self, section_idx: u32, para_idx: u32) -> Result<String, JsValue> {
        self.core
            .merge_paragraph(section_idx, para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getTextRange)]
    pub fn get_text_range(
        &self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_text_range(section_idx, para_idx, char_offset, count)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getParagraphLength)]
    pub fn get_paragraph_length(&self, section_idx: u32, para_idx: u32) -> Result<u32, JsValue> {
        self.core
            .get_paragraph_length(section_idx, para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getParagraphCount)]
    pub fn get_paragraph_count(&self, section_idx: u32) -> Result<u32, JsValue> {
        self.core.get_paragraph_count(section_idx).map_err(js_error)
    }
    #[wasm_bindgen(js_name = getCaretPosition)]
    pub fn get_caret_position(&self) -> String {
        self.core.get_caret_position()
    }
    #[wasm_bindgen(js_name = insertPageBreak)]
    pub fn insert_page_break(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .insert_page_break(section_idx, paragraph_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = insertColumnBreak)]
    pub fn insert_column_break(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .insert_column_break(section_idx, paragraph_idx, char_offset)
            .map_err(js_error)
    }
}
