use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = insertNewNumber)]
    pub fn insert_new_number(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        start_num: u32,
    ) -> Result<String, JsValue> {
        self.core
            .insert_new_number(section_idx, paragraph_idx, char_offset, start_num)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = setNumberingRestart)]
    pub fn set_numbering_restart(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        mode: u32,
        start_num: u32,
    ) -> Result<String, JsValue> {
        self.core
            .set_numbering_restart(section_idx, paragraph_idx, mode, start_num)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = createStyle)]
    pub fn create_style(&mut self, json: &str) -> u32 {
        self.core.create_style(json)
    }
    #[wasm_bindgen(js_name = updateStyle)]
    pub fn update_style(&mut self, style_id: u32, json: &str) -> bool {
        self.core.update_style(style_id, json)
    }
    #[wasm_bindgen(js_name = updateStyleShapes)]
    pub fn update_style_shapes(
        &mut self,
        style_id: u32,
        char_mods_json: &str,
        para_mods_json: &str,
    ) -> bool {
        self.core
            .update_style_shapes(style_id, char_mods_json, para_mods_json)
    }
    #[wasm_bindgen(js_name = deleteStyle)]
    pub fn delete_style(&mut self, style_id: u32) -> bool {
        self.core.delete_style(style_id)
    }
    #[wasm_bindgen(js_name = createNumbering)]
    pub fn create_numbering(&mut self, json: &str) -> u32 {
        self.core.create_numbering(json)
    }
    #[wasm_bindgen(js_name = getCharPropertiesAt)]
    pub fn get_char_properties_at(
        &self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_char_properties_at(section_idx, para_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = applyCharFormat)]
    pub fn apply_char_format(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        start_offset: u32,
        end_offset: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .apply_char_format(section_idx, para_idx, start_offset, end_offset, props_json)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = findOrCreateFontId)]
    pub fn find_or_create_font_id(&self, name: &str) -> u32 {
        self.core.find_or_create_font_id(name)
    }
    #[wasm_bindgen(js_name = findOrCreateFontIdForLang)]
    pub fn find_or_create_font_id_for_lang(&self, lang: u32, name: &str) -> u32 {
        self.core.find_or_create_font_id_for_lang(lang, name)
    }
    #[wasm_bindgen(js_name = getParaPropertiesAt)]
    pub fn get_para_properties_at(
        &self,
        section_idx: u32,
        para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_para_properties_at(section_idx, para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = applyParaFormat)]
    pub fn apply_para_format(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .apply_para_format(section_idx, para_idx, props_json)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getStyleList)]
    pub fn get_style_list(&self) -> String {
        self.core.get_style_list()
    }
    #[wasm_bindgen(js_name = getStyleDetail)]
    pub fn get_style_detail(&self, style_id: u32) -> Result<String, JsValue> {
        self.core.get_style_detail(style_id).map_err(js_error)
    }
    #[wasm_bindgen(js_name = getStyleAt)]
    pub fn get_style_at(&self, section_idx: u32, para_idx: u32) -> Result<String, JsValue> {
        self.core
            .get_style_at(section_idx, para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = applyStyle)]
    pub fn apply_style(
        &mut self,
        section_idx: u32,
        para_idx: u32,
        style_id: u32,
    ) -> Result<String, JsValue> {
        self.core
            .apply_style(section_idx, para_idx, style_id)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getNumberingList)]
    pub fn get_numbering_list(&self) -> String {
        self.core.get_numbering_list()
    }
    #[wasm_bindgen(js_name = getBulletList)]
    pub fn get_bullet_list(&self) -> String {
        self.core.get_bullet_list()
    }
    #[wasm_bindgen(js_name = ensureDefaultNumbering)]
    pub fn ensure_default_numbering(&self) -> u32 {
        self.core.ensure_default_numbering()
    }
    #[wasm_bindgen(js_name = ensureDefaultBullet)]
    pub fn ensure_default_bullet(&self, bullet_char: &str) -> u32 {
        self.core.ensure_default_bullet(bullet_char)
    }
}
