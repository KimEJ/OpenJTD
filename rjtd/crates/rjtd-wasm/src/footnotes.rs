use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = insertTextInFootnote)]
    pub fn insert_text_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        fn_para_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String, JsValue> {
        self.core
            .insert_text_in_footnote(
                section_idx,
                paragraph_idx,
                control_idx,
                fn_para_idx,
                char_offset,
                text,
            )
            .map_err(js_error)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = deleteTextInFootnote)]
    pub fn delete_text_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        fn_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_text_in_footnote(
                section_idx,
                paragraph_idx,
                control_idx,
                fn_para_idx,
                char_offset,
                count,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = splitParagraphInFootnote)]
    pub fn split_paragraph_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        fn_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .split_paragraph_in_footnote(
                section_idx,
                paragraph_idx,
                control_idx,
                fn_para_idx,
                char_offset,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = mergeParagraphInFootnote)]
    pub fn merge_paragraph_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        fn_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .merge_paragraph_in_footnote(section_idx, paragraph_idx, control_idx, fn_para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getCursorRectInFootnote)]
    pub fn get_cursor_rect_in_footnote(
        &self,
        page_num: u32,
        footnote_index: u32,
        fn_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cursor_rect_in_footnote(page_num, footnote_index, fn_para_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getCursorRectInNote)]
    pub fn get_cursor_rect_in_note(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        note_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cursor_rect_in_note(
                section_idx,
                paragraph_idx,
                control_idx,
                note_para_idx,
                char_offset,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getParaPropertiesInFootnote)]
    pub fn get_para_properties_in_footnote(
        &self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        fn_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_para_properties_in_footnote(section_idx, paragraph_idx, control_idx, fn_para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = applyParaFormatInFootnote)]
    pub fn apply_para_format_in_footnote(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        control_idx: u32,
        fn_para_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .apply_para_format_in_footnote(
                section_idx,
                paragraph_idx,
                control_idx,
                fn_para_idx,
                props_json,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getSelectionRectsInFootnote)]
    pub fn get_selection_rects_in_footnote(
        &self,
        page_num: u32,
        footnote_index: u32,
        start_fn_para: u32,
        start_offset: u32,
        end_fn_para: u32,
        end_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_selection_rects_in_footnote(
                page_num,
                footnote_index,
                start_fn_para,
                start_offset,
                end_fn_para,
                end_offset,
            )
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = insertFootnote)]
    pub fn insert_footnote(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .insert_footnote(section_idx, para_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = insertEndnote)]
    pub fn insert_endnote(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .insert_endnote(section_idx, para_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getEndnoteShape)]
    pub fn get_endnote_shape(&self, section_idx: u32) -> Result<String, JsValue> {
        self.core.get_endnote_shape(section_idx).map_err(js_error)
    }
    #[wasm_bindgen(js_name = applyEndnoteShape)]
    pub fn apply_endnote_shape(
        &mut self,
        section_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .apply_endnote_shape(section_idx, props_json)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getFootnoteInfo)]
    pub fn get_footnote_info(
        &self,
        section_idx: u32,
        para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_footnote_info(section_idx, para_idx, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = deleteFootnote)]
    pub fn delete_footnote(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_footnote(section_idx, para_idx, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getPageFootnoteInfo)]
    pub fn get_page_footnote_info(
        &self,
        page_num: u32,
        footnote_index: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_page_footnote_info(page_num, footnote_index)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getNoteEditInfo)]
    pub fn get_note_edit_info(
        &self,
        section_idx: u32,
        para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_note_edit_info(section_idx, para_idx, control_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getNoteEquationProperties)]
    pub fn get_note_equation_properties(
        &self,
        section_idx: u32,
        para_idx: u32,
        control_idx: u32,
        note_para_idx: u32,
        equation_idx: u32,
    ) -> String {
        self.core.get_note_equation_properties(
            section_idx,
            para_idx,
            control_idx,
            note_para_idx,
            equation_idx,
        )
    }
    #[wasm_bindgen(js_name = setNoteEquationProperties)]
    pub fn set_note_equation_properties(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        control_idx: u32,
        note_para_idx: u32,
        equation_idx: u32,
        props_json: &str,
    ) -> String {
        self.core.set_note_equation_properties(
            section_idx,
            para_idx,
            control_idx,
            note_para_idx,
            equation_idx,
            props_json,
        )
    }
}
