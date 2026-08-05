use crate::{HwpDocument, js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl HwpDocument {
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
}
