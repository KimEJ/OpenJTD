use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(js_name = getPositionOfPage)]
    pub fn get_position_of_page(&self, global_page: u32) -> Result<String, JsValue> {
        self.core
            .get_position_of_page(global_page)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getPageOfPosition)]
    pub fn get_page_of_position(&self, section_idx: u32, para_idx: u32) -> Result<String, JsValue> {
        self.core
            .get_page_of_position(section_idx, para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getPageControlLayout)]
    pub fn get_page_control_layout(&self, page_num: u32) -> Result<String, JsValue> {
        self.core
            .get_page_control_layout(page_num)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = getTextBoxControlIndex)]
    pub fn get_text_box_control_index(
        &self,
        section_idx: u32,
        para_idx: u32,
    ) -> Result<i32, JsValue> {
        self.core
            .get_text_box_control_index(section_idx, para_idx)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = findNextEditableControl)]
    pub fn find_next_editable_control(
        &self,
        section_idx: u32,
        para_idx: u32,
        control_idx: i32,
        delta: i32,
    ) -> String {
        self.core
            .find_next_editable_control(section_idx, para_idx, control_idx, delta)
    }
    #[wasm_bindgen(js_name = findNearestControlBackward)]
    pub fn find_nearest_control_backward(
        &self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> String {
        self.core
            .find_nearest_control_backward(section_idx, para_idx, char_offset)
    }
    #[wasm_bindgen(js_name = findNearestControlForward)]
    pub fn find_nearest_control_forward(
        &self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> String {
        self.core
            .find_nearest_control_forward(section_idx, para_idx, char_offset)
    }
    #[wasm_bindgen(js_name = getControlTextPositions)]
    pub fn get_control_text_positions(&self, section_idx: u32, para_idx: u32) -> String {
        self.core.get_control_text_positions(section_idx, para_idx)
    }
    #[wasm_bindgen(js_name = navigateNextEditable)]
    pub fn navigate_next_editable(
        &self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
        delta: i32,
        context_json: &str,
    ) -> String {
        self.core
            .navigate_next_editable(section_idx, para_idx, char_offset, delta, context_json)
    }
    #[wasm_bindgen(js_name = getCursorRect)]
    pub fn get_cursor_rect(
        &self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cursor_rect(section_idx, para_idx, char_offset)
            .map_err(js_error)
    }
    #[wasm_bindgen(js_name = hitTest)]
    pub fn hit_test(&self, page_num: u32, x: f64, y: f64) -> Result<String, JsValue> {
        self.core.hit_test(page_num, x, y).map_err(js_error)
    }
    #[wasm_bindgen(js_name = hitTestBodyFootnoteMarker)]
    pub fn hit_test_body_footnote_marker(&self, page_num: u32, x: f64, y: f64) -> String {
        hit_false_json(page_num, x, y)
    }
    #[wasm_bindgen(js_name = hitTestFootnote)]
    pub fn hit_test_footnote(&self, page_num: u32, x: f64, y: f64) -> String {
        hit_false_json(page_num, x, y)
    }
    #[wasm_bindgen(js_name = hitTestHeaderFooter)]
    pub fn hit_test_header_footer(&self, page_num: u32, x: f64, y: f64) -> String {
        hit_false_json(page_num, x, y)
    }
    #[wasm_bindgen(js_name = hitTestInFootnote)]
    pub fn hit_test_in_footnote(&self, page_num: u32, x: f64, y: f64) -> String {
        hit_false_json(page_num, x, y)
    }
    #[wasm_bindgen(js_name = hitTestInHeaderFooter)]
    pub fn hit_test_in_header_footer(
        &self,
        page_num: u32,
        _is_header: bool,
        x: f64,
        y: f64,
    ) -> String {
        hit_false_json(page_num, x, y)
    }
    #[wasm_bindgen(js_name = getLineInfo)]
    pub fn get_line_info(
        &self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_line_info(section_idx, para_idx, char_offset)
            .map_err(js_error)
    }
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = moveVertical)]
    pub fn move_vertical(
        &self,
        section_idx: u32,
        para_idx: u32,
        char_offset: u32,
        delta: i32,
        preferred_x: f64,
        _parent_para_idx: u32,
        _control_idx: u32,
        _cell_idx: u32,
        _cell_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .move_vertical(section_idx, para_idx, char_offset, delta, preferred_x)
            .map_err(js_error)
    }
}

fn hit_false_json(page_num: u32, x: f64, y: f64) -> String {
    format!(
        "{{\"hit\":false,\"pageIndex\":{},\"x\":{:.1},\"y\":{:.1}}}",
        page_num,
        normalize_coordinate(x),
        normalize_coordinate(y)
    )
}

fn normalize_coordinate(coordinate: f64) -> f64 {
    if coordinate.is_finite() {
        coordinate
    } else {
        0.0
    }
}
