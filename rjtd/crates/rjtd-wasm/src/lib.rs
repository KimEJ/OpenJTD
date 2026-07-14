#![doc = include_str!("../README.md")]

use rjtd_model::{Document, DocumentCore};
use wasm_bindgen::prelude::*;

#[cfg(any(test, target_arch = "wasm32"))]
mod canvas;

#[cfg(target_arch = "wasm32")]
use canvas::canvas_layout;

pub fn engine_name() -> &'static str {
    "rjtd"
}

#[wasm_bindgen]
pub struct HwpDocument {
    core: DocumentCore,
}

impl std::ops::Deref for HwpDocument {
    type Target = DocumentCore;

    fn deref(&self) -> &DocumentCore {
        &self.core
    }
}

impl std::ops::DerefMut for HwpDocument {
    fn deref_mut(&mut self) -> &mut DocumentCore {
        &mut self.core
    }
}

impl HwpDocument {
    pub fn from_bytes(data: &[u8]) -> rjtd_core::Result<Self> {
        DocumentCore::from_bytes(data).map(|core| Self { core })
    }

    pub fn from_document(document: Document) -> Self {
        Self {
            core: DocumentCore::from_document(document),
        }
    }
}

#[wasm_bindgen]
impl HwpDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Result<HwpDocument, JsValue> {
        Self::from_bytes(data).map_err(js_error)
    }

    #[wasm_bindgen(js_name = createEmpty)]
    pub fn create_empty() -> HwpDocument {
        HwpDocument::from_document(blank_document())
    }

    #[wasm_bindgen(js_name = createBlankDocument)]
    pub fn create_blank_document(&mut self) -> String {
        self.core = DocumentCore::from_document(blank_document());
        self.core.get_document_info()
    }

    #[wasm_bindgen(js_name = pageCount)]
    pub fn page_count(&self) -> u32 {
        self.core.page_count()
    }

    #[wasm_bindgen(js_name = getSectionCount)]
    pub fn get_section_count(&self) -> u32 {
        self.core.get_section_count()
    }

    #[wasm_bindgen(js_name = getDocumentInfo)]
    pub fn get_document_info(&self) -> String {
        self.core.get_document_info()
    }

    #[wasm_bindgen(js_name = getPageInfo)]
    pub fn get_page_info(&self, page_num: u32) -> Result<String, JsValue> {
        self.core.get_page_info(page_num).map_err(js_error)
    }

    #[wasm_bindgen(js_name = getPageDef)]
    pub fn get_page_def(&self, section_idx: u32) -> Result<String, JsValue> {
        self.core.get_page_def(section_idx).map_err(js_error)
    }

    #[wasm_bindgen(js_name = setPageDef)]
    pub fn set_page_def(
        &mut self,
        section_idx: u32,
        page_def_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_page_def(section_idx, page_def_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getSectionDef)]
    pub fn get_section_def(&self, section_idx: u32) -> Result<String, JsValue> {
        self.core.get_section_def(section_idx).map_err(js_error)
    }

    #[wasm_bindgen(js_name = setSectionDef)]
    pub fn set_section_def(
        &mut self,
        section_idx: u32,
        section_def_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_section_def(section_idx, section_def_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setSectionDefAll)]
    pub fn set_section_def_all(&mut self, section_def_json: &str) -> String {
        self.core.set_section_def_all(section_def_json)
    }

    #[wasm_bindgen(js_name = getPageBorderFill)]
    pub fn get_page_border_fill(&self, section_idx: u32) -> Result<String, JsValue> {
        self.core
            .get_page_border_fill(section_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setPageBorderFill)]
    pub fn set_page_border_fill(
        &mut self,
        section_idx: u32,
        settings_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_page_border_fill(section_idx, settings_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = renderPageSvg)]
    pub fn render_page_svg(&self, page_num: u32) -> Result<String, JsValue> {
        self.core.render_page_svg(page_num).map_err(js_error)
    }

    #[wasm_bindgen(js_name = renderPageHtml)]
    pub fn render_page_html(&self, page_num: u32) -> Result<String, JsValue> {
        self.core.render_page_html(page_num).map_err(js_error)
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = renderPageToCanvas)]
    pub fn render_page_to_canvas(
        &self,
        page_num: u32,
        canvas: &web_sys::HtmlCanvasElement,
        scale: f64,
    ) -> Result<(), JsValue> {
        render_core_page_to_canvas(&self.core, page_num, canvas, scale)
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = renderPageToCanvasFiltered)]
    pub fn render_page_to_canvas_filtered(
        &self,
        page_num: u32,
        canvas: &web_sys::HtmlCanvasElement,
        scale: f64,
        _layer_kind: &str,
    ) -> Result<(), JsValue> {
        render_core_page_to_canvas(&self.core, page_num, canvas, scale)
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = renderPageToCanvasLegacy)]
    pub fn render_page_to_canvas_legacy(
        &self,
        page_num: u32,
        canvas: &web_sys::HtmlCanvasElement,
        scale: f64,
    ) -> Result<(), JsValue> {
        render_core_page_to_canvas(&self.core, page_num, canvas, scale)
    }

    #[wasm_bindgen(js_name = getPageLayerTree)]
    pub fn get_page_layer_tree(&self, page_num: u32) -> Result<String, JsValue> {
        self.core.get_page_layer_tree(page_num).map_err(js_error)
    }

    #[wasm_bindgen(js_name = getPageLayerTreeWithProfile)]
    pub fn get_page_layer_tree_with_profile(
        &self,
        page_num: u32,
        profile: &str,
    ) -> Result<String, JsValue> {
        self.core
            .get_page_layer_tree_with_profile(page_num, profile)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getPageOverlayImages)]
    pub fn get_page_overlay_images(&self, page_num: u32) -> Result<String, JsValue> {
        self.core
            .get_page_overlay_images(page_num)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCanvasKitReplayPlan)]
    pub fn get_canvaskit_replay_plan(&self, page_num: u32, mode: &str) -> Result<String, JsValue> {
        self.core
            .get_canvaskit_replay_plan(page_num, mode)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setFileName)]
    pub fn set_file_name(&mut self, name: &str) {
        self.core.set_file_name(name);
    }

    #[wasm_bindgen(js_name = getDpi)]
    pub fn get_dpi(&self) -> f64 {
        self.core.get_dpi()
    }

    #[wasm_bindgen(js_name = setDpi)]
    pub fn set_dpi(&mut self, dpi: f64) {
        self.core.set_dpi(dpi);
    }

    #[wasm_bindgen(js_name = getSourceFormat)]
    pub fn get_source_format(&self) -> String {
        self.core.get_source_format().to_string()
    }

    #[wasm_bindgen(js_name = convertToEditable)]
    pub fn convert_to_editable(&mut self) -> String {
        self.core.convert_to_editable()
    }

    #[wasm_bindgen(js_name = refreshLayout)]
    pub fn refresh_layout(&mut self) {
        self.core.refresh_layout();
    }

    #[wasm_bindgen(js_name = getValidationWarnings)]
    pub fn get_validation_warnings(&self) -> String {
        self.core.get_validation_warnings()
    }

    #[wasm_bindgen(js_name = reflowLinesegs)]
    pub fn reflow_linesegs(&mut self) -> u32 {
        self.core.reflow_linesegs()
    }

    #[wasm_bindgen(js_name = getExternalImageBasenames)]
    pub fn get_external_image_basenames(&self) -> String {
        self.core.get_external_image_basenames()
    }

    #[wasm_bindgen(js_name = injectExternalImage)]
    pub fn inject_external_image(&mut self, name: &str, bytes: &[u8], display_path: &str) -> u32 {
        self.core.inject_external_image(name, bytes, display_path)
    }

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

    #[wasm_bindgen(js_name = setShowParagraphMarks)]
    pub fn set_show_paragraph_marks(&mut self, enabled: bool) {
        self.core.set_show_paragraph_marks(enabled);
    }

    #[wasm_bindgen(js_name = getShowControlCodes)]
    pub fn get_show_control_codes(&self) -> bool {
        self.core.get_show_control_codes()
    }

    #[wasm_bindgen(js_name = setShowControlCodes)]
    pub fn set_show_control_codes(&mut self, enabled: bool) {
        self.core.set_show_control_codes(enabled);
    }

    #[wasm_bindgen(js_name = getShowTransparentBorders)]
    pub fn get_show_transparent_borders(&self) -> bool {
        self.core.get_show_transparent_borders()
    }

    #[wasm_bindgen(js_name = setShowTransparentBorders)]
    pub fn set_show_transparent_borders(&mut self, enabled: bool) {
        self.core.set_show_transparent_borders(enabled);
    }

    #[wasm_bindgen(js_name = setClipEnabled)]
    pub fn set_clip_enabled(&mut self, enabled: bool) {
        self.core.set_clip_enabled(enabled);
    }

    #[wasm_bindgen(js_name = saveSnapshot)]
    pub fn save_snapshot(&mut self) -> u32 {
        self.core.save_snapshot()
    }

    #[wasm_bindgen(js_name = restoreSnapshot)]
    pub fn restore_snapshot(&mut self, id: u32) -> Result<String, JsValue> {
        self.core.restore_snapshot(id).map_err(js_error)
    }

    #[wasm_bindgen(js_name = discardSnapshot)]
    pub fn discard_snapshot(&mut self, id: u32) {
        self.core.discard_snapshot(id);
    }

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

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = insertTextInCell)]
    pub fn insert_text_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        text: &str,
    ) -> Result<String, JsValue> {
        self.core
            .insert_text_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
                text,
            )
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = deleteTextInCell)]
    pub fn delete_text_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_text_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
                count,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = insertTextInCellByPath)]
    pub fn insert_text_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        text: &str,
    ) -> Result<String, JsValue> {
        self.core
            .insert_text_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset, text)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = deleteTextInCellByPath)]
    pub fn delete_text_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_text_in_cell_by_path(
                section_idx,
                parent_para_idx,
                path_json,
                char_offset,
                count,
            )
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = splitParagraphInCell)]
    pub fn split_paragraph_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .split_paragraph_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = splitParagraphInCellByPath)]
    pub fn split_paragraph_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .split_paragraph_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = mergeParagraphInCell)]
    pub fn merge_paragraph_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .merge_paragraph_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = mergeParagraphInCellByPath)]
    pub fn merge_paragraph_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .merge_paragraph_in_cell_by_path(section_idx, parent_para_idx, path_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = pasteInternalInCell)]
    pub fn paste_internal_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .paste_internal_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellParagraphCount)]
    pub fn get_cell_paragraph_count(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<u32, JsValue> {
        self.core
            .get_cell_paragraph_count(section_idx, parent_para_idx, control_idx, cell_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellParagraphLength)]
    pub fn get_cell_paragraph_length(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<u32, JsValue> {
        self.core
            .get_cell_paragraph_length(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellParagraphCountByPath)]
    pub fn get_cell_paragraph_count_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<u32, JsValue> {
        self.core
            .get_cell_paragraph_count_by_path(section_idx, parent_para_idx, path_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellParagraphLengthByPath)]
    pub fn get_cell_paragraph_length_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<u32, JsValue> {
        self.core
            .get_cell_paragraph_length_by_path(section_idx, parent_para_idx, path_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellTextDirection)]
    pub fn get_cell_text_direction(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<u32, JsValue> {
        self.core
            .get_cell_text_direction(section_idx, parent_para_idx, control_idx, cell_idx)
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = getTextInCell)]
    pub fn get_text_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_text_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
                count,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getTextInCellByPath)]
    pub fn get_text_in_cell_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        count: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_text_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset, count)
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = getCursorRectInCell)]
    pub fn get_cursor_rect_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cursor_rect_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCursorRectByPath)]
    pub fn get_cursor_rect_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cursor_rect_by_path(section_idx, parent_para_idx, path_json, char_offset)
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = getLineInfoInCell)]
    pub fn get_line_info_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_line_info_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getTableDimensions)]
    pub fn get_table_dimensions(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_table_dimensions(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getTableDimensionsByPath)]
    pub fn get_table_dimensions_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .get_table_dimensions_by_path(section_idx, parent_para_idx, path_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellInfo)]
    pub fn get_cell_info(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cell_info(section_idx, parent_para_idx, control_idx, cell_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellInfoByPath)]
    pub fn get_cell_info_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .get_cell_info_by_path(section_idx, parent_para_idx, path_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellProperties)]
    pub fn get_cell_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cell_properties(section_idx, parent_para_idx, control_idx, cell_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setCellProperties)]
    pub fn set_cell_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_cell_properties(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                props_json,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = resizeTableCells)]
    pub fn resize_table_cells(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        updates_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .resize_table_cells(section_idx, parent_para_idx, control_idx, updates_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = moveTableOffset)]
    pub fn move_table_offset(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        delta_h: i32,
        delta_v: i32,
    ) -> Result<String, JsValue> {
        self.core
            .move_table_offset(section_idx, parent_para_idx, control_idx, delta_h, delta_v)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getTableProperties)]
    pub fn get_table_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_table_properties(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setTableProperties)]
    pub fn set_table_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_table_properties(section_idx, parent_para_idx, control_idx, props_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getTableCellBboxes)]
    pub fn get_table_cell_bboxes(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        page_hint: Option<u32>,
    ) -> Result<String, JsValue> {
        self.core
            .get_table_cell_bboxes(section_idx, parent_para_idx, control_idx, page_hint)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getTableCellBboxesByPath)]
    pub fn get_table_cell_bboxes_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .get_table_cell_bboxes_by_path(section_idx, parent_para_idx, path_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getTableBBox)]
    pub fn get_table_bbox(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_table_bbox(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = createTable)]
    pub fn create_table(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        rows: u32,
        cols: u32,
    ) -> Result<String, JsValue> {
        self.core
            .create_table(section_idx, paragraph_idx, char_offset, rows, cols)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = deleteTableControl)]
    pub fn delete_table_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_table_control(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = insertTableRow)]
    pub fn insert_table_row(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row_idx: u32,
        below: bool,
    ) -> Result<String, JsValue> {
        self.core
            .insert_table_row(section_idx, parent_para_idx, control_idx, row_idx, below)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = insertTableColumn)]
    pub fn insert_table_column(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        col_idx: u32,
        right: bool,
    ) -> Result<String, JsValue> {
        self.core
            .insert_table_column(section_idx, parent_para_idx, control_idx, col_idx, right)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = deleteTableRow)]
    pub fn delete_table_row(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_table_row(section_idx, parent_para_idx, control_idx, row_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = deleteTableColumn)]
    pub fn delete_table_column(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        col_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_table_column(section_idx, parent_para_idx, control_idx, col_idx)
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = mergeTableCells)]
    pub fn merge_table_cells(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
    ) -> Result<String, JsValue> {
        self.core
            .merge_table_cells(
                section_idx,
                parent_para_idx,
                control_idx,
                start_row,
                start_col,
                end_row,
                end_col,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = splitTableCell)]
    pub fn split_table_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row: u32,
        col: u32,
    ) -> Result<String, JsValue> {
        self.core
            .split_table_cell(section_idx, parent_para_idx, control_idx, row, col)
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = splitTableCellInto)]
    pub fn split_table_cell_into(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        row: u32,
        col: u32,
        n_rows: u32,
        m_cols: u32,
        equal_row_height: bool,
        merge_first: bool,
    ) -> Result<String, JsValue> {
        self.core
            .split_table_cell_into(
                section_idx,
                parent_para_idx,
                control_idx,
                row,
                col,
                n_rows,
                m_cols,
                equal_row_height,
                merge_first,
            )
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = splitTableCellsInRange)]
    pub fn split_table_cells_in_range(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        start_row: u32,
        start_col: u32,
        end_row: u32,
        end_col: u32,
        n_rows: u32,
        m_cols: u32,
        equal_row_height: bool,
    ) -> Result<String, JsValue> {
        self.core
            .split_table_cells_in_range(
                section_idx,
                parent_para_idx,
                control_idx,
                start_row,
                start_col,
                end_row,
                end_col,
                n_rows,
                m_cols,
                equal_row_height,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getColumnDef)]
    pub fn get_column_def(&self, section_idx: u32) -> Result<String, JsValue> {
        self.core.get_column_def(section_idx).map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = getSelectionRectsInCell)]
    pub fn get_selection_rects_in_cell(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_selection_rects_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                start_cell_para_idx,
                start_char_offset,
                end_cell_para_idx,
                end_char_offset,
            )
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = copySelectionInCell)]
    pub fn copy_selection_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .copy_selection_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                start_cell_para_idx,
                start_char_offset,
                end_cell_para_idx,
                end_char_offset,
            )
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = deleteRangeInCell)]
    pub fn delete_range_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        start_cell_para_idx: u32,
        start_char_offset: u32,
        end_cell_para_idx: u32,
        end_char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_range_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                start_cell_para_idx,
                start_char_offset,
                end_cell_para_idx,
                end_char_offset,
            )
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = getCellCharPropertiesAt)]
    pub fn get_cell_char_properties_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cell_char_properties_at(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                char_offset,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellParaPropertiesAt)]
    pub fn get_cell_para_properties_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cell_para_properties_at(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
            )
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = applyCharFormatInCell)]
    pub fn apply_char_format_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        start_offset: u32,
        end_offset: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .apply_char_format_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                start_offset,
                end_offset,
                props_json,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = applyParaFormatInCell)]
    pub fn apply_para_format_in_cell(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .apply_para_format_in_cell(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                props_json,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellStyleAt)]
    pub fn get_cell_style_at(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cell_style_at(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = applyCellStyle)]
    pub fn apply_cell_style(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        cell_idx: u32,
        cell_para_idx: u32,
        style_id: u32,
    ) -> Result<String, JsValue> {
        self.core
            .apply_cell_style(
                section_idx,
                parent_para_idx,
                control_idx,
                cell_idx,
                cell_para_idx,
                style_id,
            )
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = evaluateTableFormula)]
    pub fn evaluate_table_formula(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        target_row: u32,
        target_col: u32,
        formula: &str,
        write_result: bool,
    ) -> Result<String, JsValue> {
        self.core
            .evaluate_table_formula(
                section_idx,
                parent_para_idx,
                control_idx,
                target_row,
                target_col,
                formula,
                write_result,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = pasteInternalInCellByPath)]
    pub fn paste_internal_in_cell_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
    ) -> Result<String, JsValue> {
        self.core
            .paste_internal_in_cell_by_path(section_idx, parent_para_idx, path_json, char_offset)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = moveVerticalByPath)]
    pub fn move_vertical_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        path_json: &str,
        char_offset: u32,
        delta: i32,
        preferred_x: f64,
    ) -> Result<String, JsValue> {
        self.core
            .move_vertical_by_path(
                section_idx,
                parent_para_idx,
                path_json,
                char_offset,
                delta,
                preferred_x,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getTableSignature)]
    pub fn get_table_signature(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_table_signature(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

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

    #[wasm_bindgen(js_name = getShapeBBox)]
    pub fn get_shape_bbox(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_shape_bbox(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = insertPicture)]
    pub fn insert_picture(
        &mut self,
        section_idx: u32,
        paragraph_idx: u32,
        char_offset: u32,
        cell_path_json: &str,
        image_data: &[u8],
        width: u32,
        height: u32,
        natural_width_px: u32,
        natural_height_px: u32,
        extension: &str,
        description: &str,
        paper_offset_x_hu: Option<i32>,
        paper_offset_y_hu: Option<i32>,
    ) -> Result<String, JsValue> {
        self.core
            .insert_picture(
                section_idx,
                paragraph_idx,
                char_offset,
                cell_path_json,
                image_data,
                width,
                height,
                natural_width_px,
                natural_height_px,
                extension,
                description,
                paper_offset_x_hu,
                paper_offset_y_hu,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getPictureProperties)]
    pub fn get_picture_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_picture_properties(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getHeaderFooterPictureProperties)]
    pub fn get_header_footer_picture_properties(
        &self,
        section_idx: u32,
        outer_para_idx: u32,
        outer_control_idx: u32,
        inner_para_idx: u32,
        inner_control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_header_footer_picture_properties(
                section_idx,
                outer_para_idx,
                outer_control_idx,
                inner_para_idx,
                inner_control_idx,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setPictureProperties)]
    pub fn set_picture_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_picture_properties(section_idx, parent_para_idx, control_idx, props_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setHeaderFooterPictureProperties)]
    pub fn set_header_footer_picture_properties(
        &mut self,
        section_idx: u32,
        outer_para_idx: u32,
        outer_control_idx: u32,
        inner_para_idx: u32,
        inner_control_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_header_footer_picture_properties(
                section_idx,
                outer_para_idx,
                outer_control_idx,
                inner_para_idx,
                inner_control_idx,
                props_json,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = deletePictureControl)]
    pub fn delete_picture_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_picture_control(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = deleteCellPictureControlByPath)]
    pub fn delete_cell_picture_control_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        cell_path_json: &str,
        inner_control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_cell_picture_control_by_path(
                section_idx,
                parent_para_idx,
                cell_path_json,
                inner_control_idx,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellShapePropertiesByPath)]
    pub fn get_cell_shape_properties_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        cell_path_json: &str,
        inner_control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cell_shape_properties_by_path(
                section_idx,
                parent_para_idx,
                cell_path_json,
                inner_control_idx,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getCellPicturePropertiesByPath)]
    pub fn get_cell_picture_properties_by_path(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        cell_path_json: &str,
        inner_control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_cell_picture_properties_by_path(
                section_idx,
                parent_para_idx,
                cell_path_json,
                inner_control_idx,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setCellShapePropertiesByPath)]
    pub fn set_cell_shape_properties_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        cell_path_json: &str,
        inner_control_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_cell_shape_properties_by_path(
                section_idx,
                parent_para_idx,
                cell_path_json,
                inner_control_idx,
                props_json,
            )
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setCellPicturePropertiesByPath)]
    pub fn set_cell_picture_properties_by_path(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        cell_path_json: &str,
        inner_control_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_cell_picture_properties_by_path(
                section_idx,
                parent_para_idx,
                cell_path_json,
                inner_control_idx,
                props_json,
            )
            .map_err(js_error)
    }

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

    #[wasm_bindgen(js_name = createShapeControl)]
    pub fn create_shape_control(&mut self, params_json: &str) -> Result<String, JsValue> {
        self.core
            .create_shape_control(params_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getShapeProperties)]
    pub fn get_shape_properties(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_shape_properties(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = getShapeText)]
    pub fn get_shape_text(
        &self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .get_shape_text(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = setShapeProperties)]
    pub fn set_shape_properties(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        props_json: &str,
    ) -> Result<String, JsValue> {
        self.core
            .set_shape_properties(section_idx, parent_para_idx, control_idx, props_json)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = deleteShapeControl)]
    pub fn delete_shape_control(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .delete_shape_control(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = changeShapeZOrder)]
    pub fn change_shape_z_order(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        operation: &str,
    ) -> Result<String, JsValue> {
        self.core
            .change_shape_z_order(section_idx, parent_para_idx, control_idx, operation)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = groupShapes)]
    pub fn group_shapes(&mut self, json: &str) -> String {
        self.core.group_shapes(json)
    }

    #[wasm_bindgen(js_name = ungroupShape)]
    pub fn ungroup_shape(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
    ) -> Result<String, JsValue> {
        self.core
            .ungroup_shape(section_idx, parent_para_idx, control_idx)
            .map_err(js_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = moveLineEndpoint)]
    pub fn move_line_endpoint(
        &mut self,
        section_idx: u32,
        parent_para_idx: u32,
        control_idx: u32,
        sx: i32,
        sy: i32,
        ex: i32,
        ey: i32,
    ) -> Result<String, JsValue> {
        self.core
            .move_line_endpoint(section_idx, parent_para_idx, control_idx, sx, sy, ex, ey)
            .map_err(js_error)
    }

    #[wasm_bindgen(js_name = updateConnectorsInSection)]
    pub fn update_connectors_in_section(&mut self, section_idx: u32) {
        self.core.update_connectors_in_section(section_idx);
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

    #[wasm_bindgen(js_name = exportHwp)]
    pub fn export_hwp(&self) -> Vec<u8> {
        self.core.export_hwp()
    }

    #[wasm_bindgen(js_name = exportHwpx)]
    pub fn export_hwpx(&self) -> Vec<u8> {
        self.core.export_hwpx()
    }

    #[wasm_bindgen(js_name = exportHwpVerify)]
    pub fn export_hwp_verify(&self) -> String {
        self.core.export_hwp_verify()
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

    #[wasm_bindgen(js_name = setColumnDef)]
    pub fn set_column_def(
        &mut self,
        section_idx: u32,
        column_count: u32,
        column_type: u32,
        same_width: u32,
        spacing_hu: u32,
    ) -> Result<String, JsValue> {
        self.core
            .set_column_def(
                section_idx,
                column_count,
                column_type,
                same_width,
                spacing_hu,
            )
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

#[cfg(target_arch = "wasm32")]
fn render_core_page_to_canvas(
    core: &DocumentCore,
    page_num: u32,
    canvas: &web_sys::HtmlCanvasElement,
    scale: f64,
) -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;

    let lines = core.page_text_lines(page_num).map_err(js_error)?;
    let layout =
        canvas_layout(core.page_width_px(), core.page_height_px(), scale).map_err(js_error)?;
    canvas.set_width(layout.width());
    canvas.set_height(layout.height());

    let context = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("2d canvas context is unavailable"))?
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.set_transform(layout.scale(), 0.0, 0.0, layout.scale(), 0.0, 0.0)?;
    context.set_fill_style_str("#ffffff");
    context.fill_rect(0.0, 0.0, core.page_width_px(), core.page_height_px());

    context.set_fill_style_str("#111111");
    context.set_font(&format!(
        "{}px \"Hiragino Sans\", \"Yu Gothic\", Meiryo, sans-serif",
        core.font_size_px()
    ));

    for (index, line) in lines.iter().enumerate() {
        let y = core.page_margin_px() + core.font_size_px() + index as f64 * core.line_height_px();
        context.fill_text(line.text(), core.page_margin_px(), y)?;
    }

    Ok(())
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

fn blank_document() -> Document {
    Document::new(
        Default::default(),
        vec![rjtd_model::Block::Paragraph(
            rjtd_model::Paragraph::from_text(""),
        )],
    )
}

fn js_error(error: rjtd_core::Error) -> JsValue {
    JsValue::from_str(&error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hwp_document_wrapper_exposes_rhwp_shaped_surface() {
        let mut document = HwpDocument::from_document(Document::from_plain_text("銀河鉄道"));
        document.set_file_name("sample.jtd");

        assert_eq!(
            HwpDocument::create_empty().get_validation_warnings(),
            "{\"count\":0,\"summary\":{},\"warnings\":[]}"
        );
        assert_eq!(document.page_count(), 1);
        assert!(
            document
                .get_document_info()
                .contains("\"fileName\":\"sample.jtd\"")
        );
        assert!(
            document
                .get_page_info(0)
                .unwrap()
                .contains("\"pageIndex\":0")
        );
        assert!(
            document
                .get_page_def(0)
                .unwrap()
                .contains("\"width\":794.0")
        );
        assert!(
            document
                .get_section_def(0)
                .unwrap()
                .contains("\"pageNum\":1")
        );
        assert!(
            document
                .get_page_border_fill(0)
                .unwrap()
                .contains("\"basis\":\"paper\"")
        );
        assert!(document.render_page_svg(0).unwrap().contains("銀河鉄道"));
        let layer_tree = document.get_page_layer_tree(0).unwrap();
        assert!(layer_tree.contains("\"schemaVersion\":1"));
        assert!(layer_tree.contains("\"outputOptions\":{"));
        assert!(layer_tree.contains("\"root\":{\"kind\":\"leaf\""));
        assert!(layer_tree.contains("\"type\":\"pageBackground\""));
        assert!(layer_tree.contains("\"backgroundColor\":\"#ffffff\""));
        assert!(layer_tree.contains("\"type\":\"textRun\""));
        assert!(layer_tree.contains("\"textSources\":["));
        assert!(layer_tree.contains("\"fontResources\":{\"blobs\":[],\"faces\":[]}"));
        assert_eq!(
            document.get_page_overlay_images(0).unwrap(),
            "{\"behind\":[],\"front\":[],\"imageCount\":0}"
        );
        let replay_plan = document.get_canvaskit_replay_plan(0, "default").unwrap();
        assert!(replay_plan.contains("\"mode\":\"default\""));
        assert!(replay_plan.contains("\"totalItems\":2"));
        assert!(replay_plan.contains("\"opType\":\"pageBackground\""));
        assert!(replay_plan.contains("\"replayPlane\":\"background\""));
        assert!(replay_plan.contains("\"opType\":\"textRun\""));
        assert!(replay_plan.contains("\"replayPlane\":\"flow\""));
        assert!(replay_plan.contains("\"status\":\"direct\""));
        document.set_dpi(120.0);
        assert_eq!(document.get_dpi(), 120.0);
        assert_eq!(document.get_source_format(), "jtd");
        let warnings = document.get_validation_warnings();
        assert!(warnings.contains("\"count\":1"));
        assert!(warnings.contains("\"kind\":\"JtdFallbackTextPagination\""));
        assert_eq!(document.reflow_linesegs(), 0);
        assert_eq!(
            document.convert_to_editable(),
            "{\"ok\":true,\"converted\":false}"
        );
        assert!(
            document
                .get_cursor_rect(0, 0, 0)
                .unwrap()
                .contains("\"height\":23.0")
        );
        assert!(
            document
                .hit_test(0, 72.0, 72.0)
                .unwrap()
                .contains("\"paragraphIndex\":0")
        );
        assert!(
            document
                .get_line_info(0, 0, 0)
                .unwrap()
                .contains("\"lineCount\":1")
        );
        assert!(
            document
                .move_vertical(0, 0, 0, 1, -1.0, u32::MAX, u32::MAX, u32::MAX, u32::MAX)
                .unwrap()
                .contains("\"rectValid\":true")
        );
        assert!(
            document
                .hit_test_body_footnote_marker(0, 72.0, 72.0)
                .contains("\"hit\":false")
        );
        assert_eq!(document.get_section_count(), 1);
        assert_eq!(document.get_paragraph_count(0).unwrap(), 1);
        assert_eq!(document.get_paragraph_length(0, 0).unwrap(), 4);
        assert_eq!(document.get_text_range(0, 0, 1, 2).unwrap(), "河鉄");
        assert_eq!(
            document.insert_text(0, 0, 4, "の夜").unwrap(),
            "{\"ok\":true,\"charOffset\":6}"
        );
        assert_eq!(
            document.split_paragraph(0, 0, 2).unwrap(),
            "{\"ok\":true,\"paraIdx\":1,\"charOffset\":0}"
        );
        assert_eq!(document.get_paragraph_count(0).unwrap(), 2);
        document.set_show_paragraph_marks(true);
        assert!(!document.get_show_control_codes());
        document.set_show_control_codes(true);
        document.set_show_transparent_borders(true);
        document.set_clip_enabled(false);
        assert!(document.get_show_control_codes());
        assert!(document.get_show_transparent_borders());
        assert_eq!(
            document.get_position_of_page(0).unwrap(),
            "{\"ok\":true,\"sec\":0,\"para\":0,\"charOffset\":0}"
        );
        assert_eq!(
            document.get_page_of_position(0, 1).unwrap(),
            "{\"ok\":true,\"page\":0}"
        );
        assert_eq!(document.get_control_text_positions(0, 0), "[]");
        assert_eq!(
            document.find_next_editable_control(0, 0, -1, 1),
            "{\"type\":\"body\",\"sec\":0,\"para\":1}"
        );
        assert_eq!(
            document.find_nearest_control_backward(0, 0, 2),
            "{\"type\":\"none\"}"
        );
        assert_eq!(
            document.find_nearest_control_forward(0, 0, 2),
            "{\"type\":\"none\"}"
        );
        assert_eq!(
            document.navigate_next_editable(0, 0, 0, 1, "[]"),
            "{\"type\":\"text\",\"sec\":0,\"para\":0,\"charOffset\":1,\"context\":[]}"
        );
        assert_eq!(document.get_field_list(), "[]");
        assert_eq!(document.get_field_info_at(0, 0, 0), "{\"inField\":false}");
        assert_eq!(document.remove_field_at(0, 0, 0), "{\"ok\":false}");
        assert!(!document.set_active_field(0, 0, 0));
        document.clear_active_field();
        assert_eq!(document.get_click_here_props(1), "{\"ok\":false}");
        assert_eq!(
            document.get_header_footer(0, true, 0).unwrap(),
            "{\"ok\":true,\"exists\":false}"
        );
        assert_eq!(
            document.create_header_footer(0, true, 0).unwrap(),
            "{\"ok\":false,\"exists\":false}"
        );
        assert_eq!(
            document.get_header_footer_list(0, true, 0),
            "{\"ok\":true,\"items\":[],\"currentIndex\":-1}"
        );
        assert_eq!(
            document.toggle_hide_header_footer(0, true).unwrap(),
            "{\"ok\":false,\"hidden\":false}"
        );
        assert_eq!(
            document.navigate_header_footer_by_page(0, true, 1),
            "{\"ok\":false}"
        );
        assert_eq!(document.insert_footnote(0, 0, 0).unwrap(), "{\"ok\":false}");
        assert!(
            document
                .get_endnote_shape(0)
                .unwrap()
                .contains("\"ok\":false")
        );
        assert_eq!(
            document.get_note_edit_info(0, 0, 0).unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(
            document.merge_paragraph(0, 1).unwrap(),
            "{\"ok\":true,\"paraIdx\":0,\"charOffset\":2}"
        );
        assert!(
            document
                .get_page_control_layout(0)
                .unwrap()
                .contains("\"controls\":[]")
        );
        assert_eq!(
            document.get_column_def(0).unwrap(),
            "{\"columnCount\":1,\"columnType\":0,\"sameWidth\":true,\"spacing\":0}"
        );
        assert_eq!(
            document.get_table_dimensions(0, 0, 0).unwrap(),
            "{\"rowCount\":0,\"colCount\":0,\"cellCount\":0}"
        );
        assert_eq!(
            document.get_cell_info(0, 0, 0, 0).unwrap(),
            "{\"row\":0,\"col\":0,\"rowSpan\":1,\"colSpan\":1}"
        );
        assert_eq!(document.get_table_cell_bboxes(0, 0, 0, None).unwrap(), "[]");
        assert!(
            document
                .get_table_properties(0, 0, 0)
                .unwrap()
                .contains("\"repeatHeader\":false")
        );
        assert!(
            document
                .get_cell_properties(0, 0, 0, 0)
                .unwrap()
                .contains("\"isHeader\":false")
        );
        assert_eq!(
            document.insert_text_in_cell(0, 0, 0, 0, 0, 0, "x").unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(document.get_cell_paragraph_count(0, 0, 0, 0).unwrap(), 0);
        assert_eq!(
            document.get_cell_paragraph_length(0, 0, 0, 0, 0).unwrap(),
            0
        );
        assert_eq!(document.get_text_in_cell(0, 0, 0, 0, 0, 0, 10).unwrap(), "");
        assert!(
            document
                .get_cursor_rect_in_cell(0, 0, 0, 0, 0, 0)
                .unwrap()
                .contains("\"height\":23.0")
        );
        assert_eq!(
            document.insert_table_row(0, 0, 0, 0, true).unwrap(),
            "{\"ok\":false,\"rowCount\":0,\"colCount\":0}"
        );
        assert_eq!(
            document.merge_table_cells(0, 0, 0, 0, 0, 0, 1).unwrap(),
            "{\"ok\":false,\"cellCount\":0}"
        );
        assert_eq!(
            document.create_table(0, 0, 0, 2, 2).unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert_eq!(
            document
                .get_selection_rects_in_cell(0, 0, 0, 0, 0, 0, 0, 0)
                .unwrap(),
            "[]"
        );
        assert_eq!(
            document
                .copy_selection_in_cell(0, 0, 0, 0, 0, 0, 0, 0)
                .unwrap(),
            "{\"ok\":false,\"text\":\"\"}"
        );
        assert_eq!(
            document
                .apply_char_format_in_cell(0, 0, 0, 0, 0, 0, 0, "{}")
                .unwrap(),
            "{\"ok\":false}"
        );
        assert_eq!(
            document.apply_cell_style(0, 0, 0, 0, 0, 0).unwrap(),
            "{\"ok\":false}"
        );
        assert!(
            document
                .evaluate_table_formula(0, 0, 0, 0, 0, "=A1", false)
                .unwrap()
                .contains("\"ok\":false")
        );
        assert_eq!(document.get_paragraph_stable_id(0, 0).unwrap(), "rjtd-p0");
        document.ensure_paragraph_stable_ids();
        assert!(
            document
                .debug_dump_stable_ids(0, 0, 1)
                .unwrap()
                .contains("\"stableId\":\"rjtd-p0\"")
        );
        assert_eq!(document.get_table_signature(0, 0, 0).unwrap(), "");
        assert!(
            document
                .get_shape_bbox(0, 0, 0)
                .unwrap()
                .contains("\"width\":0.0")
        );
        assert_eq!(
            document
                .insert_picture(0, 0, 0, "", &[], 1, 1, 1, 1, "png", "", None, None)
                .unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert!(
            document
                .get_picture_properties(0, 0, 0)
                .unwrap()
                .contains("\"effect\":\"none\"")
        );
        assert_eq!(
            document.delete_picture_control(0, 0, 0).unwrap(),
            "{\"ok\":false}"
        );
        assert!(
            document
                .get_equation_properties(0, 0, 0, -1, -1)
                .unwrap()
                .contains("\"script\":\"\"")
        );
        assert!(
            document
                .render_equation_preview("x", 1000, 0)
                .contains(">x<")
        );
        assert_eq!(
            document.create_shape_control("{}").unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert_eq!(
            document.change_shape_z_order(0, 0, 0, "front").unwrap(),
            "{\"ok\":false,\"zOrder\":0}"
        );
        assert_eq!(
            document.insert_equation(0, 0, 0, "x", 1000, 0).unwrap(),
            "{\"ok\":false,\"paraIdx\":0,\"controlIdx\":-1}"
        );
        assert_eq!(
            document.get_form_object_at(0, 0.0, 0.0).unwrap(),
            "{\"found\":false}"
        );
        assert_eq!(document.get_form_value(0, 0, 0).unwrap(), "{\"ok\":false}");
        assert_eq!(
            document.copy_control(0, 0, "", 0).unwrap(),
            "{\"ok\":false}"
        );
        assert!(
            document
                .get_control_image_data(0, 0, "", 0)
                .unwrap()
                .is_empty()
        );
        assert_eq!(document.get_control_image_mime(0, 0, "", 0).unwrap(), "");
        assert_eq!(document.get_bookmarks(), "[]");
        assert!(document.export_hwp().is_empty());
        assert!(document.export_hwpx().is_empty());
        assert!(document.export_hwp_verify().contains("\"ok\":false"));
        assert_eq!(
            document.insert_page_break(0, 0, 0).unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            document.set_column_def(0, 1, 0, 1, 0).unwrap(),
            "{\"ok\":true,\"pageCount\":1}"
        );
        assert_eq!(document.create_style("{}"), 0);
        assert!(document.update_style(0, "{}"));
        assert_eq!(document.create_numbering("{}"), 0);
        assert_eq!(
            document
                .insert_text_in_footnote(0, 0, 0, 0, 0, "x")
                .unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            document
                .get_selection_rects_in_footnote(0, 0, 0, 0, 0, 0)
                .unwrap(),
            "[]"
        );
        assert!(
            document
                .get_para_properties_in_hf(0, true, 0, 0)
                .unwrap()
                .contains("\"alignment\":\"left\"")
        );
        assert_eq!(
            document.insert_field_in_hf(0, true, 0, 0, 0, 0).unwrap(),
            "{\"ok\":false,\"charOffset\":0}"
        );
        assert_eq!(
            document.export_selection_html(0, 0, 0, 0, 2).unwrap(),
            "<p>銀河</p>"
        );
        assert!(
            document
                .get_char_properties_at(0, 0, 0)
                .unwrap()
                .contains("\"fontFamily\":\"Hiragino Sans\"")
        );
        assert!(
            document
                .get_para_properties_at(0, 0)
                .unwrap()
                .contains("\"alignment\":\"left\"")
        );
        assert_eq!(
            document
                .apply_char_format(0, 0, 0, 1, "{\"bold\":true}")
                .unwrap(),
            "{\"ok\":true}"
        );
        assert_eq!(
            document
                .apply_para_format(0, 0, "{\"alignment\":\"center\"}")
                .unwrap(),
            "{\"ok\":true}"
        );
        assert_eq!(document.find_or_create_font_id("Hiragino Sans"), 0);
        assert!(document.get_style_list().contains("\"name\":\"Normal\""));
        assert!(
            document
                .get_style_detail(0)
                .unwrap()
                .contains("\"paraProps\"")
        );
        assert_eq!(
            document.get_style_at(0, 0).unwrap(),
            "{\"id\":0,\"name\":\"Normal\"}"
        );
        assert_eq!(document.apply_style(0, 0, 0).unwrap(), "{\"ok\":true}");
        assert_eq!(document.get_numbering_list(), "[]");
        assert_eq!(document.get_bullet_list(), "[]");
        assert_eq!(document.ensure_default_numbering(), 0);
        assert_eq!(document.ensure_default_bullet("*"), 0);
        assert!(
            document
                .get_selection_rects(0, 0, 0, 0, 2)
                .unwrap()
                .contains("\"width\"")
        );
        assert_eq!(
            document.copy_selection(0, 0, 0, 0, 2).unwrap(),
            "{\"ok\":true,\"text\":\"銀河\"}"
        );
        assert!(document.has_internal_clipboard());
        assert_eq!(document.get_clipboard_text(), "銀河");
        assert_eq!(
            document.paste_internal(0, 0, 0).unwrap(),
            "{\"ok\":true,\"charOffset\":2}"
        );
        assert!(
            document
                .get_text_range(0, 0, 0, 4)
                .unwrap()
                .starts_with("銀河")
        );
        assert_eq!(
            document.delete_range(0, 0, 0, 0, 2).unwrap(),
            "{\"ok\":true,\"charOffset\":0}"
        );
        document.clear_clipboard();
        assert!(!document.has_internal_clipboard());
        assert!(!document.clipboard_has_control());
        let snapshot_id = document.save_snapshot();
        assert_eq!(snapshot_id, 1);
        document.insert_text(0, 0, 0, "夜").unwrap();
        assert!(
            document
                .get_text_range(0, 0, 0, 3)
                .unwrap()
                .starts_with("夜")
        );
        assert_eq!(
            document.restore_snapshot(snapshot_id).unwrap(),
            "{\"ok\":true,\"pageCount\":1}"
        );
        document.discard_snapshot(snapshot_id);
        assert!(
            document
                .search_all_text("銀河", true, false)
                .contains("\"charOffset\":0")
        );
        assert!(
            document
                .search_text("銀河", 0, 0, 0, true, true)
                .unwrap()
                .contains("\"found\":true")
        );
        assert_eq!(
            document.replace_text(0, 0, 0, 2, "銀河").unwrap(),
            "{\"ok\":true,\"charOffset\":0,\"newLength\":2}"
        );
        assert_eq!(
            document.replace_one("銀河", "星", true).unwrap(),
            "{\"ok\":true,\"sec\":0,\"para\":0,\"charOffset\":0,\"newLength\":1}"
        );
        assert!(
            document
                .replace_all("星", "銀河", true)
                .unwrap()
                .contains("\"count\":")
        );

        let info = document.create_blank_document();
        assert!(info.contains("\"pageCount\":1"));
        assert_eq!(document.get_paragraph_count(0).unwrap(), 1);
    }

    #[test]
    fn hwp_document_wrapper_delegates_jtd_control_navigation_projection() {
        use rjtd_model::{
            Block, Inline, Metadata, Paragraph, TextControlBoundary, TextRun, TextSourceSpan,
        };

        let paragraph = Paragraph::new(
            vec![Inline::Text(TextRun::with_source_span(
                "銀河",
                None,
                Some(TextSourceSpan::new(0, 4, 0, 2)),
            ))],
            None,
        );
        let mut model = Document::new(Metadata::default(), vec![Block::Paragraph(paragraph)]);
        model.push_text_control_boundary(TextControlBoundary::new(
            0,
            0x001c,
            Some(TextSourceSpan::new(4, 6, 2, 3)),
        ));
        let document = HwpDocument::from_document(model);

        assert_eq!(document.get_control_text_positions(0, 0), "[2]");
        assert_eq!(
            document.find_nearest_control_backward(0, 0, 3),
            "{\"type\":\"jtdControl\",\"sec\":0,\"para\":0,\"ci\":0,\"charPos\":2,\"code\":28,\"codeHex\":\"0x001c\",\"decoded\":false}"
        );
        assert_eq!(
            document.find_nearest_control_forward(0, 0, 0),
            "{\"type\":\"jtdControl\",\"sec\":0,\"para\":0,\"ci\":0,\"charPos\":2,\"code\":28,\"codeHex\":\"0x001c\",\"decoded\":false}"
        );
        let layout = document.get_page_control_layout(0).unwrap();
        assert!(layout.contains("\"type\":\"jtdControl\""));
        assert!(layout.contains("\"source\":\"textControlBoundary\""));
    }

    #[test]
    fn hwp_document_wrapper_applies_jtd_style_candidates() {
        let mut model = Document::from_plain_text("銀河鉄道");
        model.push_unknown_style(rjtd_model::UnknownStyle::from_stream(
            rjtd_core::style_stream::TEXT_LAYOUT_STYLE_PATH,
            ssmg_style_with_label_fixture("本文"),
        ));
        let mut document = HwpDocument::from_document(model);

        let style_list = document.get_style_list();
        assert!(style_list.contains("\"name\":\"本文\""));
        assert!(style_list.contains("\"jtdCandidate\":true"));
        assert_eq!(
            document.get_style_at(0, 0).unwrap(),
            "{\"id\":0,\"name\":\"Normal\"}"
        );

        let applied = document.apply_style(0, 0, 1).unwrap();
        assert!(applied.contains("\"styleId\":1"));
        let style_at = document.get_style_at(0, 0).unwrap();
        assert!(style_at.contains("\"id\":1"));
        assert!(style_at.contains("\"name\":\"本文\""));
    }

    fn ssmg_style_with_label_fixture(label: &str) -> Vec<u8> {
        let mut bytes = vec![
            b'S', b's', b'm', b'g', b'V', b'.', b'0', b'1', 0, 0, 0, 0x1c, 0, 0, 1, 0, 0, 0, 0,
            0x20, 0, 1, 0, 2,
        ];
        bytes.resize(0x114, 0);
        let label_units = label.encode_utf16().collect::<Vec<_>>();
        let payload_len = 2 + label_units.len() * 2;
        bytes.extend_from_slice(&0x5555u16.to_be_bytes());
        bytes.extend_from_slice(&(payload_len as u16).to_be_bytes());
        bytes.extend_from_slice(&(label_units.len() as u16).to_be_bytes());
        for unit in label_units {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes
    }
}
