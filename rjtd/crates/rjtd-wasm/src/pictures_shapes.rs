use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
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
}
