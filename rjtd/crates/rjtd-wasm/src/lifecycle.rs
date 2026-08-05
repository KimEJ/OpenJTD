use crate::{HwpDocument, js_error};
use rjtd_model::{Document, DocumentCore};
use wasm_bindgen::prelude::*;

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
    #[wasm_bindgen(js_name = getColumnDef)]
    pub fn get_column_def(&self, section_idx: u32) -> Result<String, JsValue> {
        self.core.get_column_def(section_idx).map_err(js_error)
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
}

fn blank_document() -> Document {
    Document::new(
        Default::default(),
        vec![rjtd_model::Block::Paragraph(
            rjtd_model::Paragraph::from_text(""),
        )],
    )
}
