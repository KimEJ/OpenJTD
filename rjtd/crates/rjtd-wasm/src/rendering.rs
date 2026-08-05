use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
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
}

#[cfg(target_arch = "wasm32")]
fn render_core_page_to_canvas(
    core: &rjtd_model::DocumentCore,
    page_num: u32,
    canvas: &web_sys::HtmlCanvasElement,
    scale: f64,
) -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;

    let lines = core.page_text_lines(page_num).map_err(js_error)?;
    let layout = crate::canvas::canvas_layout(core.page_width_px(), core.page_height_px(), scale)
        .map_err(js_error)?;
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
