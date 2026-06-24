/* @ts-self-types="./rjtd_wasm.d.ts" */

export class HwpDocument {
    static __wrap(ptr) {
        const obj = Object.create(HwpDocument.prototype);
        obj.__wbg_ptr = ptr;
        HwpDocumentFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HwpDocumentFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hwpdocument_free(ptr, 0);
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @param {string} name
     * @returns {string}
     */
    addBookmark(section_idx, paragraph_idx, char_offset, name) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_addBookmark(this.__wbg_ptr, section_idx, paragraph_idx, char_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} style_id
     * @returns {string}
     */
    applyCellStyle(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, style_id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_applyCellStyle(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, style_id);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} start_offset
     * @param {number} end_offset
     * @param {string} props_json
     * @returns {string}
     */
    applyCharFormat(section_idx, para_idx, start_offset, end_offset, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_applyCharFormat(this.__wbg_ptr, section_idx, para_idx, start_offset, end_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} start_offset
     * @param {number} end_offset
     * @param {string} props_json
     * @returns {string}
     */
    applyCharFormatInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, start_offset, end_offset, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_applyCharFormatInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, start_offset, end_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {string} props_json
     * @returns {string}
     */
    applyEndnoteShape(section_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_applyEndnoteShape(this.__wbg_ptr, section_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} template_id
     * @returns {string}
     */
    applyHfTemplate(section_idx, is_header, apply_to, template_id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_applyHfTemplate(this.__wbg_ptr, section_idx, is_header, apply_to, template_id);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {string} props_json
     * @returns {string}
     */
    applyParaFormat(section_idx, para_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_applyParaFormat(this.__wbg_ptr, section_idx, para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {string} props_json
     * @returns {string}
     */
    applyParaFormatInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_applyParaFormatInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {number} fn_para_idx
     * @param {string} props_json
     * @returns {string}
     */
    applyParaFormatInFootnote(section_idx, paragraph_idx, control_idx, fn_para_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_applyParaFormatInFootnote(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, fn_para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @param {string} props_json
     * @returns {string}
     */
    applyParaFormatInHf(section_idx, is_header, apply_to, hf_para_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_applyParaFormatInHf(this.__wbg_ptr, section_idx, is_header, apply_to, hf_para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} style_id
     * @returns {string}
     */
    applyStyle(section_idx, para_idx, style_id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_applyStyle(this.__wbg_ptr, section_idx, para_idx, style_id);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {string} operation
     * @returns {string}
     */
    changeShapeZOrder(section_idx, parent_para_idx, control_idx, operation) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(operation, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_changeShapeZOrder(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    clearActiveField() {
        wasm.hwpdocument_clearActiveField(this.__wbg_ptr);
    }
    clearClipboard() {
        wasm.hwpdocument_clearClipboard(this.__wbg_ptr);
    }
    /**
     * @returns {boolean}
     */
    clipboardHasControl() {
        const ret = wasm.hwpdocument_clipboardHasControl(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {string}
     */
    convertToEditable() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_convertToEditable(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {string} cell_path_json
     * @param {number} control_idx
     * @returns {string}
     */
    copyControl(section_idx, paragraph_idx, cell_path_json, control_idx) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_copyControl(this.__wbg_ptr, section_idx, paragraph_idx, ptr0, len0, control_idx);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} start_para_idx
     * @param {number} start_char_offset
     * @param {number} end_para_idx
     * @param {number} end_char_offset
     * @returns {string}
     */
    copySelection(section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_copySelection(this.__wbg_ptr, section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} start_cell_para_idx
     * @param {number} start_char_offset
     * @param {number} end_cell_para_idx
     * @param {number} end_char_offset
     * @returns {string}
     */
    copySelectionInCell(section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para_idx, start_char_offset, end_cell_para_idx, end_char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_copySelectionInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para_idx, start_char_offset, end_cell_para_idx, end_char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    createBlankDocument() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_createBlankDocument(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {HwpDocument}
     */
    static createEmpty() {
        const ret = wasm.hwpdocument_createEmpty();
        return HwpDocument.__wrap(ret);
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @returns {string}
     */
    createHeaderFooter(section_idx, is_header, apply_to) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_createHeaderFooter(this.__wbg_ptr, section_idx, is_header, apply_to);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {string} json
     * @returns {number}
     */
    createNumbering(json) {
        const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_createNumbering(this.__wbg_ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * @param {string} params_json
     * @returns {string}
     */
    createShapeControl(params_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_createShapeControl(this.__wbg_ptr, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {string} json
     * @returns {number}
     */
    createStyle(json) {
        const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_createStyle(this.__wbg_ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @param {number} rows
     * @param {number} cols
     * @returns {string}
     */
    createTable(section_idx, paragraph_idx, char_offset, rows, cols) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_createTable(this.__wbg_ptr, section_idx, paragraph_idx, char_offset, rows, cols);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} start_para
     * @param {number} count
     * @returns {string}
     */
    debugDumpStableIds(section_idx, start_para, count) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_debugDumpStableIds(this.__wbg_ptr, section_idx, start_para, count);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @returns {string}
     */
    deleteBookmark(section_idx, paragraph_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteBookmark(this.__wbg_ptr, section_idx, paragraph_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} cell_path_json
     * @param {number} inner_control_idx
     * @returns {string}
     */
    deleteCellPictureControlByPath(section_idx, parent_para_idx, cell_path_json, inner_control_idx) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_deleteCellPictureControlByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, inner_control_idx);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    deleteEquationControl(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteEquationControl(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    deleteFootnote(section_idx, para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteFootnote(this.__wbg_ptr, section_idx, para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     */
    deleteHeaderFooter(section_idx, is_header, apply_to) {
        wasm.hwpdocument_deleteHeaderFooter(this.__wbg_ptr, section_idx, is_header, apply_to);
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    deletePictureControl(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deletePictureControl(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} start_para_idx
     * @param {number} start_char_offset
     * @param {number} end_para_idx
     * @param {number} end_char_offset
     * @returns {string}
     */
    deleteRange(section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteRange(this.__wbg_ptr, section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} start_cell_para_idx
     * @param {number} start_char_offset
     * @param {number} end_cell_para_idx
     * @param {number} end_char_offset
     * @returns {string}
     */
    deleteRangeInCell(section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para_idx, start_char_offset, end_cell_para_idx, end_char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteRangeInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para_idx, start_char_offset, end_cell_para_idx, end_char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    deleteShapeControl(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteShapeControl(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} style_id
     * @returns {boolean}
     */
    deleteStyle(style_id) {
        const ret = wasm.hwpdocument_deleteStyle(this.__wbg_ptr, style_id);
        return ret !== 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} col_idx
     * @returns {string}
     */
    deleteTableColumn(section_idx, parent_para_idx, control_idx, col_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteTableColumn(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, col_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    deleteTableControl(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteTableControl(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} row_idx
     * @returns {string}
     */
    deleteTableRow(section_idx, parent_para_idx, control_idx, row_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteTableRow(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, row_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @param {number} count
     * @returns {string}
     */
    deleteText(section_idx, para_idx, char_offset, count) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteText(this.__wbg_ptr, section_idx, para_idx, char_offset, count);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @param {number} count
     * @returns {string}
     */
    deleteTextInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, count) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteTextInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, count);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @param {number} count
     * @returns {string}
     */
    deleteTextInCellByPath(section_idx, parent_para_idx, path_json, char_offset, count) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_deleteTextInCellByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset, count);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {number} fn_para_idx
     * @param {number} char_offset
     * @param {number} count
     * @returns {string}
     */
    deleteTextInFootnote(section_idx, paragraph_idx, control_idx, fn_para_idx, char_offset, count) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteTextInFootnote(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, fn_para_idx, char_offset, count);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @param {number} char_offset
     * @param {number} count
     * @returns {string}
     */
    deleteTextInHeaderFooter(section_idx, is_header, apply_to, hf_para_idx, char_offset, count) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_deleteTextInHeaderFooter(this.__wbg_ptr, section_idx, is_header, apply_to, hf_para_idx, char_offset, count);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} id
     */
    discardSnapshot(id) {
        wasm.hwpdocument_discardSnapshot(this.__wbg_ptr, id);
    }
    /**
     * @param {string} bullet_char
     * @returns {number}
     */
    ensureDefaultBullet(bullet_char) {
        const ptr0 = passStringToWasm0(bullet_char, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_ensureDefaultBullet(this.__wbg_ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    ensureDefaultNumbering() {
        const ret = wasm.hwpdocument_ensureDefaultNumbering(this.__wbg_ptr);
        return ret >>> 0;
    }
    ensureParagraphStableIds() {
        wasm.hwpdocument_ensureParagraphStableIds(this.__wbg_ptr);
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} target_row
     * @param {number} target_col
     * @param {string} formula
     * @param {boolean} write_result
     * @returns {string}
     */
    evaluateTableFormula(section_idx, parent_para_idx, control_idx, target_row, target_col, formula, write_result) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(formula, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_evaluateTableFormula(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, target_row, target_col, ptr0, len0, write_result);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {string} cell_path_json
     * @param {number} control_idx
     * @returns {string}
     */
    exportControlHtml(section_idx, paragraph_idx, cell_path_json, control_idx) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_exportControlHtml(this.__wbg_ptr, section_idx, paragraph_idx, ptr0, len0, control_idx);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @returns {Uint8Array}
     */
    exportHwp() {
        const ret = wasm.hwpdocument_exportHwp(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    exportHwpVerify() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_exportHwpVerify(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {Uint8Array}
     */
    exportHwpx() {
        const ret = wasm.hwpdocument_exportHwpx(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {number} section_idx
     * @param {number} start_para_idx
     * @param {number} start_char_offset
     * @param {number} end_para_idx
     * @param {number} end_char_offset
     * @returns {string}
     */
    exportSelectionHtml(section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_exportSelectionHtml(this.__wbg_ptr, section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} start_cell_para
     * @param {number} start_offset
     * @param {number} end_cell_para
     * @param {number} end_offset
     * @returns {string}
     */
    exportSelectionInCellHtml(section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para, start_offset, end_cell_para, end_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_exportSelectionInCellHtml(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para, start_offset, end_cell_para, end_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    findNearestControlBackward(section_idx, para_idx, char_offset) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_findNearestControlBackward(this.__wbg_ptr, section_idx, para_idx, char_offset);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    findNearestControlForward(section_idx, para_idx, char_offset) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_findNearestControlForward(this.__wbg_ptr, section_idx, para_idx, char_offset);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} control_idx
     * @param {number} delta
     * @returns {string}
     */
    findNextEditableControl(section_idx, para_idx, control_idx, delta) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_findNextEditableControl(this.__wbg_ptr, section_idx, para_idx, control_idx, delta);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} name
     * @returns {number}
     */
    findOrCreateFontId(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_findOrCreateFontId(this.__wbg_ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * @param {number} lang
     * @param {string} name
     * @returns {number}
     */
    findOrCreateFontIdForLang(lang, name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_findOrCreateFontIdForLang(this.__wbg_ptr, lang, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    getBookmarks() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getBookmarks(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getBulletList() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getBulletList(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {string} mode
     * @returns {string}
     */
    getCanvasKitReplayPlan(page_num, mode) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getCanvasKitReplayPlan(this.__wbg_ptr, page_num, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getCaretPosition() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getCaretPosition(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getCellCharPropertiesAt(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCellCharPropertiesAt(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @returns {string}
     */
    getCellInfo(section_idx, parent_para_idx, control_idx, cell_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCellInfo(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @returns {string}
     */
    getCellInfoByPath(section_idx, parent_para_idx, path_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getCellInfoByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @returns {string}
     */
    getCellParaPropertiesAt(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCellParaPropertiesAt(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @returns {number}
     */
    getCellParagraphCount(section_idx, parent_para_idx, control_idx, cell_idx) {
        const ret = wasm.hwpdocument_getCellParagraphCount(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @returns {number}
     */
    getCellParagraphCountByPath(section_idx, parent_para_idx, path_json) {
        const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_getCellParagraphCountByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @returns {number}
     */
    getCellParagraphLength(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx) {
        const ret = wasm.hwpdocument_getCellParagraphLength(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @returns {number}
     */
    getCellParagraphLengthByPath(section_idx, parent_para_idx, path_json) {
        const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_getCellParagraphLengthByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} cell_path_json
     * @param {number} inner_control_idx
     * @returns {string}
     */
    getCellPicturePropertiesByPath(section_idx, parent_para_idx, cell_path_json, inner_control_idx) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getCellPicturePropertiesByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, inner_control_idx);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @returns {string}
     */
    getCellProperties(section_idx, parent_para_idx, control_idx, cell_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCellProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} cell_path_json
     * @param {number} inner_control_idx
     * @returns {string}
     */
    getCellShapePropertiesByPath(section_idx, parent_para_idx, cell_path_json, inner_control_idx) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getCellShapePropertiesByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, inner_control_idx);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @returns {string}
     */
    getCellStyleAt(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCellStyleAt(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @returns {number}
     */
    getCellTextDirection(section_idx, parent_para_idx, control_idx, cell_idx) {
        const ret = wasm.hwpdocument_getCellTextDirection(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getCharPropertiesAt(section_idx, para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCharPropertiesAt(this.__wbg_ptr, section_idx, para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} field_id
     * @returns {string}
     */
    getClickHereProps(field_id) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getClickHereProps(this.__wbg_ptr, field_id);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getClipboardText() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getClipboardText(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @returns {string}
     */
    getColumnDef(section_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getColumnDef(this.__wbg_ptr, section_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {string} cell_path_json
     * @param {number} control_idx
     * @returns {Uint8Array}
     */
    getControlImageData(section_idx, paragraph_idx, cell_path_json, control_idx) {
        const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_getControlImageData(this.__wbg_ptr, section_idx, paragraph_idx, ptr0, len0, control_idx);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {string} cell_path_json
     * @param {number} control_idx
     * @returns {string}
     */
    getControlImageMime(section_idx, paragraph_idx, cell_path_json, control_idx) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getControlImageMime(this.__wbg_ptr, section_idx, paragraph_idx, ptr0, len0, control_idx);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @returns {string}
     */
    getControlTextPositions(section_idx, para_idx) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getControlTextPositions(this.__wbg_ptr, section_idx, para_idx);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getCursorRect(section_idx, para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCursorRect(this.__wbg_ptr, section_idx, para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @returns {string}
     */
    getCursorRectByPath(section_idx, parent_para_idx, path_json, char_offset) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getCursorRectByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getCursorRectInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCursorRectInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {number} footnote_index
     * @param {number} fn_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getCursorRectInFootnote(page_num, footnote_index, fn_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCursorRectInFootnote(this.__wbg_ptr, page_num, footnote_index, fn_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @param {number} char_offset
     * @param {number} preferred_page
     * @returns {string}
     */
    getCursorRectInHeaderFooter(page_num, is_header, apply_to, hf_para_idx, char_offset, preferred_page) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCursorRectInHeaderFooter(this.__wbg_ptr, page_num, is_header, apply_to, hf_para_idx, char_offset, preferred_page);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {number} note_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getCursorRectInNote(section_idx, paragraph_idx, control_idx, note_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getCursorRectInNote(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, note_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getDocumentInfo() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getDocumentInfo(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    getDpi() {
        const ret = wasm.hwpdocument_getDpi(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} section_idx
     * @returns {string}
     */
    getEndnoteShape(section_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getEndnoteShape(this.__wbg_ptr, section_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @returns {string}
     */
    getEquationProperties(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getEquationProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getExternalImageBasenames() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getExternalImageBasenames(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getFieldInfoAt(section_idx, para_idx, char_offset) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getFieldInfoAt(this.__wbg_ptr, section_idx, para_idx, char_offset);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @returns {string}
     */
    getFieldInfoAtByPath(section_idx, parent_para_idx, path_json, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getFieldInfoAtByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @param {boolean} is_textbox
     * @returns {string}
     */
    getFieldInfoAtInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, is_textbox) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getFieldInfoAtInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, is_textbox);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getFieldList() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getFieldList(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} field_id
     * @returns {string}
     */
    getFieldValue(field_id) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getFieldValue(this.__wbg_ptr, field_id);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} name
     * @returns {string}
     */
    getFieldValueByName(name) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getFieldValueByName(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getFootnoteInfo(section_idx, para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getFootnoteInfo(this.__wbg_ptr, section_idx, para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {number} x
     * @param {number} y
     * @returns {string}
     */
    getFormObjectAt(page_num, x, y) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getFormObjectAt(this.__wbg_ptr, page_num, x, y);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getFormObjectInfo(section_idx, paragraph_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getFormObjectInfo(this.__wbg_ptr, section_idx, paragraph_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getFormValue(section_idx, paragraph_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getFormValue(this.__wbg_ptr, section_idx, paragraph_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @returns {string}
     */
    getHeaderFooter(section_idx, is_header, apply_to) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getHeaderFooter(this.__wbg_ptr, section_idx, is_header, apply_to);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} current_section_idx
     * @param {boolean} current_is_header
     * @param {number} current_apply_to
     * @returns {string}
     */
    getHeaderFooterList(current_section_idx, current_is_header, current_apply_to) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getHeaderFooterList(this.__wbg_ptr, current_section_idx, current_is_header, current_apply_to);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @returns {string}
     */
    getHeaderFooterParaInfo(section_idx, is_header, apply_to, hf_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getHeaderFooterParaInfo(this.__wbg_ptr, section_idx, is_header, apply_to, hf_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} outer_para_idx
     * @param {number} outer_control_idx
     * @param {number} inner_para_idx
     * @param {number} inner_control_idx
     * @returns {string}
     */
    getHeaderFooterPictureProperties(section_idx, outer_para_idx, outer_control_idx, inner_para_idx, inner_control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getHeaderFooterPictureProperties(this.__wbg_ptr, section_idx, outer_para_idx, outer_control_idx, inner_para_idx, inner_control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getLineInfo(section_idx, para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getLineInfo(this.__wbg_ptr, section_idx, para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    getLineInfoInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getLineInfoInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getNoteEditInfo(section_idx, para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getNoteEditInfo(this.__wbg_ptr, section_idx, para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} control_idx
     * @param {number} note_para_idx
     * @param {number} equation_idx
     * @returns {string}
     */
    getNoteEquationProperties(section_idx, para_idx, control_idx, note_para_idx, equation_idx) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getNoteEquationProperties(this.__wbg_ptr, section_idx, para_idx, control_idx, note_para_idx, equation_idx);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getNumberingList() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getNumberingList(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @returns {string}
     */
    getPageBorderFill(section_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPageBorderFill(this.__wbg_ptr, section_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @returns {string}
     */
    getPageControlLayout(page_num) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPageControlLayout(this.__wbg_ptr, page_num);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @returns {string}
     */
    getPageDef(section_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPageDef(this.__wbg_ptr, section_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {number} footnote_index
     * @returns {string}
     */
    getPageFootnoteInfo(page_num, footnote_index) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPageFootnoteInfo(this.__wbg_ptr, page_num, footnote_index);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @returns {string}
     */
    getPageInfo(page_num) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPageInfo(this.__wbg_ptr, page_num);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @returns {string}
     */
    getPageLayerTree(page_num) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPageLayerTree(this.__wbg_ptr, page_num);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {string} profile
     * @returns {string}
     */
    getPageLayerTreeWithProfile(page_num, profile) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(profile, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getPageLayerTreeWithProfile(this.__wbg_ptr, page_num, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @returns {string}
     */
    getPageOfPosition(section_idx, para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPageOfPosition(this.__wbg_ptr, section_idx, para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @returns {string}
     */
    getPageOverlayImages(page_num) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPageOverlayImages(this.__wbg_ptr, page_num);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @returns {string}
     */
    getParaPropertiesAt(section_idx, para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getParaPropertiesAt(this.__wbg_ptr, section_idx, para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {number} fn_para_idx
     * @returns {string}
     */
    getParaPropertiesInFootnote(section_idx, paragraph_idx, control_idx, fn_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getParaPropertiesInFootnote(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, fn_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @returns {string}
     */
    getParaPropertiesInHf(section_idx, is_header, apply_to, hf_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getParaPropertiesInHf(this.__wbg_ptr, section_idx, is_header, apply_to, hf_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @returns {number}
     */
    getParagraphCount(section_idx) {
        const ret = wasm.hwpdocument_getParagraphCount(this.__wbg_ptr, section_idx);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @returns {number}
     */
    getParagraphLength(section_idx, para_idx) {
        const ret = wasm.hwpdocument_getParagraphLength(this.__wbg_ptr, section_idx, para_idx);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @returns {string}
     */
    getParagraphStableId(section_idx, paragraph_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getParagraphStableId(this.__wbg_ptr, section_idx, paragraph_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getPictureProperties(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPictureProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} global_page
     * @returns {string}
     */
    getPositionOfPage(global_page) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getPositionOfPage(this.__wbg_ptr, global_page);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    getSectionCount() {
        const ret = wasm.hwpdocument_getSectionCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} section_idx
     * @returns {string}
     */
    getSectionDef(section_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getSectionDef(this.__wbg_ptr, section_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} start_para_idx
     * @param {number} start_char_offset
     * @param {number} end_para_idx
     * @param {number} end_char_offset
     * @returns {string}
     */
    getSelectionRects(section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getSelectionRects(this.__wbg_ptr, section_idx, start_para_idx, start_char_offset, end_para_idx, end_char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} start_cell_para_idx
     * @param {number} start_char_offset
     * @param {number} end_cell_para_idx
     * @param {number} end_char_offset
     * @returns {string}
     */
    getSelectionRectsInCell(section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para_idx, start_char_offset, end_cell_para_idx, end_char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getSelectionRectsInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, start_cell_para_idx, start_char_offset, end_cell_para_idx, end_char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {number} footnote_index
     * @param {number} start_fn_para
     * @param {number} start_offset
     * @param {number} end_fn_para
     * @param {number} end_offset
     * @returns {string}
     */
    getSelectionRectsInFootnote(page_num, footnote_index, start_fn_para, start_offset, end_fn_para, end_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getSelectionRectsInFootnote(this.__wbg_ptr, page_num, footnote_index, start_fn_para, start_offset, end_fn_para, end_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getShapeBBox(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getShapeBBox(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getShapeProperties(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getShapeProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getShapeText(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getShapeText(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {boolean}
     */
    getShowControlCodes() {
        const ret = wasm.hwpdocument_getShowControlCodes(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    getShowTransparentBorders() {
        const ret = wasm.hwpdocument_getShowTransparentBorders(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {string}
     */
    getSourceFormat() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getSourceFormat(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @returns {string}
     */
    getStyleAt(section_idx, para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getStyleAt(this.__wbg_ptr, section_idx, para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} style_id
     * @returns {string}
     */
    getStyleDetail(style_id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getStyleDetail(this.__wbg_ptr, style_id);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getStyleList() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getStyleList(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getTableBBox(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getTableBBox(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number | null} [page_hint]
     * @returns {string}
     */
    getTableCellBboxes(section_idx, parent_para_idx, control_idx, page_hint) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getTableCellBboxes(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, isLikeNone(page_hint) ? Number.MAX_SAFE_INTEGER : (page_hint) >>> 0);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @returns {string}
     */
    getTableCellBboxesByPath(section_idx, parent_para_idx, path_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getTableCellBboxesByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getTableDimensions(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getTableDimensions(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @returns {string}
     */
    getTableDimensionsByPath(section_idx, parent_para_idx, path_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getTableDimensionsByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getTableProperties(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getTableProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    getTableSignature(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getTableSignature(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @returns {number}
     */
    getTextBoxControlIndex(section_idx, para_idx) {
        const ret = wasm.hwpdocument_getTextBoxControlIndex(this.__wbg_ptr, section_idx, para_idx);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0];
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @param {number} count
     * @returns {string}
     */
    getTextInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, count) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getTextInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, count);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @param {number} count
     * @returns {string}
     */
    getTextInCellByPath(section_idx, parent_para_idx, path_json, char_offset, count) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_getTextInCellByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset, count);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @param {number} count
     * @returns {string}
     */
    getTextRange(section_idx, para_idx, char_offset, count) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_getTextRange(this.__wbg_ptr, section_idx, para_idx, char_offset, count);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    getValidationWarnings() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_getValidationWarnings(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} json
     * @returns {string}
     */
    groupShapes(json) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_groupShapes(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {boolean}
     */
    hasInternalClipboard() {
        const ret = wasm.hwpdocument_hasInternalClipboard(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {number} page_num
     * @param {number} x
     * @param {number} y
     * @returns {string}
     */
    hitTest(page_num, x, y) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_hitTest(this.__wbg_ptr, page_num, x, y);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {number} x
     * @param {number} y
     * @returns {string}
     */
    hitTestBodyFootnoteMarker(page_num, x, y) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_hitTestBodyFootnoteMarker(this.__wbg_ptr, page_num, x, y);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {number} x
     * @param {number} y
     * @returns {string}
     */
    hitTestFootnote(page_num, x, y) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_hitTestFootnote(this.__wbg_ptr, page_num, x, y);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {number} x
     * @param {number} y
     * @returns {string}
     */
    hitTestHeaderFooter(page_num, x, y) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_hitTestHeaderFooter(this.__wbg_ptr, page_num, x, y);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {number} x
     * @param {number} y
     * @returns {string}
     */
    hitTestInFootnote(page_num, x, y) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_hitTestInFootnote(this.__wbg_ptr, page_num, x, y);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {boolean} _is_header
     * @param {number} x
     * @param {number} y
     * @returns {string}
     */
    hitTestInHeaderFooter(page_num, _is_header, x, y) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_hitTestInHeaderFooter(this.__wbg_ptr, page_num, _is_header, x, y);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} name
     * @param {Uint8Array} bytes
     * @param {string} display_path
     * @returns {number}
     */
    injectExternalImage(name, bytes, display_path) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(display_path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_injectExternalImage(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
        return ret >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @returns {string}
     */
    insertColumnBreak(section_idx, paragraph_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_insertColumnBreak(this.__wbg_ptr, section_idx, paragraph_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    insertEndnote(section_idx, para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_insertEndnote(this.__wbg_ptr, section_idx, para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @param {string} script
     * @param {number} font_size
     * @param {number} color
     * @returns {string}
     */
    insertEquation(section_idx, paragraph_idx, char_offset, script, font_size, color) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(script, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_insertEquation(this.__wbg_ptr, section_idx, paragraph_idx, char_offset, ptr0, len0, font_size, color);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @param {number} char_offset
     * @param {number} field_type
     * @returns {string}
     */
    insertFieldInHf(section_idx, is_header, apply_to, hf_para_idx, char_offset, field_type) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_insertFieldInHf(this.__wbg_ptr, section_idx, is_header, apply_to, hf_para_idx, char_offset, field_type);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    insertFootnote(section_idx, para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_insertFootnote(this.__wbg_ptr, section_idx, para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @param {number} start_num
     * @returns {string}
     */
    insertNewNumber(section_idx, paragraph_idx, char_offset, start_num) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_insertNewNumber(this.__wbg_ptr, section_idx, paragraph_idx, char_offset, start_num);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @returns {string}
     */
    insertPageBreak(section_idx, paragraph_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_insertPageBreak(this.__wbg_ptr, section_idx, paragraph_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @param {string} cell_path_json
     * @param {Uint8Array} image_data
     * @param {number} width
     * @param {number} height
     * @param {number} natural_width_px
     * @param {number} natural_height_px
     * @param {string} extension
     * @param {string} description
     * @param {number | null} [paper_offset_x_hu]
     * @param {number | null} [paper_offset_y_hu]
     * @returns {string}
     */
    insertPicture(section_idx, paragraph_idx, char_offset, cell_path_json, image_data, width, height, natural_width_px, natural_height_px, extension, description, paper_offset_x_hu, paper_offset_y_hu) {
        let deferred6_0;
        let deferred6_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(image_data, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(extension, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len2 = WASM_VECTOR_LEN;
            const ptr3 = passStringToWasm0(description, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len3 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_insertPicture(this.__wbg_ptr, section_idx, paragraph_idx, char_offset, ptr0, len0, ptr1, len1, width, height, natural_width_px, natural_height_px, ptr2, len2, ptr3, len3, isLikeNone(paper_offset_x_hu) ? Number.MAX_SAFE_INTEGER : (paper_offset_x_hu) >> 0, isLikeNone(paper_offset_y_hu) ? Number.MAX_SAFE_INTEGER : (paper_offset_y_hu) >> 0);
            var ptr5 = ret[0];
            var len5 = ret[1];
            if (ret[3]) {
                ptr5 = 0; len5 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred6_0 = ptr5;
            deferred6_1 = len5;
            return getStringFromWasm0(ptr5, len5);
        } finally {
            wasm.__wbindgen_free(deferred6_0, deferred6_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} col_idx
     * @param {boolean} right
     * @returns {string}
     */
    insertTableColumn(section_idx, parent_para_idx, control_idx, col_idx, right) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_insertTableColumn(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, col_idx, right);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} row_idx
     * @param {boolean} below
     * @returns {string}
     */
    insertTableRow(section_idx, parent_para_idx, control_idx, row_idx, below) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_insertTableRow(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, row_idx, below);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @param {string} text
     * @returns {string}
     */
    insertText(section_idx, para_idx, char_offset, text) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_insertText(this.__wbg_ptr, section_idx, para_idx, char_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @param {string} text
     * @returns {string}
     */
    insertTextInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, text) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_insertTextInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @param {string} text
     * @returns {string}
     */
    insertTextInCellByPath(section_idx, parent_para_idx, path_json, char_offset, text) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_insertTextInCellByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset, ptr1, len1);
            var ptr3 = ret[0];
            var len3 = ret[1];
            if (ret[3]) {
                ptr3 = 0; len3 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred4_0 = ptr3;
            deferred4_1 = len3;
            return getStringFromWasm0(ptr3, len3);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {number} fn_para_idx
     * @param {number} char_offset
     * @param {string} text
     * @returns {string}
     */
    insertTextInFootnote(section_idx, paragraph_idx, control_idx, fn_para_idx, char_offset, text) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_insertTextInFootnote(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, fn_para_idx, char_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @param {number} char_offset
     * @param {string} text
     * @returns {string}
     */
    insertTextInHeaderFooter(section_idx, is_header, apply_to, hf_para_idx, char_offset, text) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_insertTextInHeaderFooter(this.__wbg_ptr, section_idx, is_header, apply_to, hf_para_idx, char_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @returns {string}
     */
    mergeParagraph(section_idx, para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_mergeParagraph(this.__wbg_ptr, section_idx, para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @returns {string}
     */
    mergeParagraphInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_mergeParagraphInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @returns {string}
     */
    mergeParagraphInCellByPath(section_idx, parent_para_idx, path_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_mergeParagraphInCellByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {number} fn_para_idx
     * @returns {string}
     */
    mergeParagraphInFootnote(section_idx, paragraph_idx, control_idx, fn_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_mergeParagraphInFootnote(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, fn_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @returns {string}
     */
    mergeParagraphInHeaderFooter(section_idx, is_header, apply_to, hf_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_mergeParagraphInHeaderFooter(this.__wbg_ptr, section_idx, is_header, apply_to, hf_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} start_row
     * @param {number} start_col
     * @param {number} end_row
     * @param {number} end_col
     * @returns {string}
     */
    mergeTableCells(section_idx, parent_para_idx, control_idx, start_row, start_col, end_row, end_col) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_mergeTableCells(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, start_row, start_col, end_row, end_col);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} sx
     * @param {number} sy
     * @param {number} ex
     * @param {number} ey
     * @returns {string}
     */
    moveLineEndpoint(section_idx, parent_para_idx, control_idx, sx, sy, ex, ey) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_moveLineEndpoint(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, sx, sy, ex, ey);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} delta_h
     * @param {number} delta_v
     * @returns {string}
     */
    moveTableOffset(section_idx, parent_para_idx, control_idx, delta_h, delta_v) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_moveTableOffset(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, delta_h, delta_v);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @param {number} delta
     * @param {number} preferred_x
     * @param {number} _parent_para_idx
     * @param {number} _control_idx
     * @param {number} _cell_idx
     * @param {number} _cell_para_idx
     * @returns {string}
     */
    moveVertical(section_idx, para_idx, char_offset, delta, preferred_x, _parent_para_idx, _control_idx, _cell_idx, _cell_para_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_moveVertical(this.__wbg_ptr, section_idx, para_idx, char_offset, delta, preferred_x, _parent_para_idx, _control_idx, _cell_idx, _cell_para_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @param {number} delta
     * @param {number} preferred_x
     * @returns {string}
     */
    moveVerticalByPath(section_idx, parent_para_idx, path_json, char_offset, delta, preferred_x) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_moveVerticalByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset, delta, preferred_x);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} current_page
     * @param {boolean} is_header
     * @param {number} direction
     * @returns {string}
     */
    navigateHeaderFooterByPage(current_page, is_header, direction) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_navigateHeaderFooterByPage(this.__wbg_ptr, current_page, is_header, direction);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @param {number} delta
     * @param {string} context_json
     * @returns {string}
     */
    navigateNextEditable(section_idx, para_idx, char_offset, delta, context_json) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(context_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_navigateNextEditable(this.__wbg_ptr, section_idx, para_idx, char_offset, delta, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {Uint8Array} data
     */
    constructor(data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_new(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0];
        HwpDocumentFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    pageCount() {
        const ret = wasm.hwpdocument_pageCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @returns {string}
     */
    pasteControl(section_idx, paragraph_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_pasteControl(this.__wbg_ptr, section_idx, paragraph_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} char_offset
     * @param {string} html
     * @returns {string}
     */
    pasteHtml(section_idx, paragraph_idx, char_offset, html) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_pasteHtml(this.__wbg_ptr, section_idx, paragraph_idx, char_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @param {string} html
     * @returns {string}
     */
    pasteHtmlInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, html) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_pasteHtmlInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @param {string} html
     * @returns {string}
     */
    pasteHtmlInCellByPath(section_idx, parent_para_idx, path_json, char_offset, html) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(html, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_pasteHtmlInCellByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset, ptr1, len1);
            var ptr3 = ret[0];
            var len3 = ret[1];
            if (ret[3]) {
                ptr3 = 0; len3 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred4_0 = ptr3;
            deferred4_1 = len3;
            return getStringFromWasm0(ptr3, len3);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    pasteInternal(section_idx, para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_pasteInternal(this.__wbg_ptr, section_idx, para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    pasteInternalInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_pasteInternalInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @returns {string}
     */
    pasteInternalInCellByPath(section_idx, parent_para_idx, path_json, char_offset) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_pasteInternalInCellByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    plainText() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_plainText(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    reflowLinesegs() {
        const ret = wasm.hwpdocument_reflowLinesegs(this.__wbg_ptr);
        return ret >>> 0;
    }
    refreshLayout() {
        wasm.hwpdocument_refreshLayout(this.__wbg_ptr);
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    removeFieldAt(section_idx, para_idx, char_offset) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_removeFieldAt(this.__wbg_ptr, section_idx, para_idx, char_offset);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @param {boolean} is_textbox
     * @returns {string}
     */
    removeFieldAtInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, is_textbox) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hwpdocument_removeFieldAtInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, is_textbox);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {string} new_name
     * @returns {string}
     */
    renameBookmark(section_idx, paragraph_idx, control_idx, new_name) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(new_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_renameBookmark(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {string} script
     * @param {number} font_size_hwpunit
     * @param {number} color
     * @returns {string}
     */
    renderEquationPreview(script, font_size_hwpunit, color) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(script, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_renderEquationPreview(this.__wbg_ptr, ptr0, len0, font_size_hwpunit, color);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @returns {string}
     */
    renderPageHtml(page_num) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_renderPageHtml(this.__wbg_ptr, page_num);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @returns {string}
     */
    renderPageSvg(page_num) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_renderPageSvg(this.__wbg_ptr, page_num);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {HTMLCanvasElement} canvas
     * @param {number} scale
     */
    renderPageToCanvas(page_num, canvas, scale) {
        const ret = wasm.hwpdocument_renderPageToCanvas(this.__wbg_ptr, page_num, canvas, scale);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} page_num
     * @param {HTMLCanvasElement} canvas
     * @param {number} scale
     * @param {string} _layer_kind
     */
    renderPageToCanvasFiltered(page_num, canvas, scale, _layer_kind) {
        const ptr0 = passStringToWasm0(_layer_kind, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_renderPageToCanvasFiltered(this.__wbg_ptr, page_num, canvas, scale, ptr0, len0);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {number} page_num
     * @param {HTMLCanvasElement} canvas
     * @param {number} scale
     */
    renderPageToCanvasLegacy(page_num, canvas, scale) {
        const ret = wasm.hwpdocument_renderPageToCanvasLegacy(this.__wbg_ptr, page_num, canvas, scale);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {string} query
     * @param {string} new_text
     * @param {boolean} case_sensitive
     * @returns {string}
     */
    replaceAll(query, new_text, case_sensitive) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(query, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(new_text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_replaceAll(this.__wbg_ptr, ptr0, len0, ptr1, len1, case_sensitive);
            var ptr3 = ret[0];
            var len3 = ret[1];
            if (ret[3]) {
                ptr3 = 0; len3 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred4_0 = ptr3;
            deferred4_1 = len3;
            return getStringFromWasm0(ptr3, len3);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * @param {string} query
     * @param {string} new_text
     * @param {boolean} case_sensitive
     * @returns {string}
     */
    replaceOne(query, new_text, case_sensitive) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(query, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(new_text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_replaceOne(this.__wbg_ptr, ptr0, len0, ptr1, len1, case_sensitive);
            var ptr3 = ret[0];
            var len3 = ret[1];
            if (ret[3]) {
                ptr3 = 0; len3 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred4_0 = ptr3;
            deferred4_1 = len3;
            return getStringFromWasm0(ptr3, len3);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @param {number} length
     * @param {string} new_text
     * @returns {string}
     */
    replaceText(section_idx, para_idx, char_offset, length, new_text) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(new_text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_replaceText(this.__wbg_ptr, section_idx, para_idx, char_offset, length, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {string} updates_json
     * @returns {string}
     */
    resizeTableCells(section_idx, parent_para_idx, control_idx, updates_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(updates_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_resizeTableCells(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} id
     * @returns {string}
     */
    restoreSnapshot(id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_restoreSnapshot(this.__wbg_ptr, id);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    saveSnapshot() {
        const ret = wasm.hwpdocument_saveSnapshot(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {string} query
     * @param {boolean} case_sensitive
     * @param {boolean} include_cells
     * @returns {string}
     */
    searchAllText(query, case_sensitive, include_cells) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(query, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_searchAllText(this.__wbg_ptr, ptr0, len0, case_sensitive, include_cells);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {string} query
     * @param {number} from_sec
     * @param {number} from_para
     * @param {number} from_char
     * @param {boolean} forward
     * @param {boolean} case_sensitive
     * @returns {string}
     */
    searchText(query, from_sec, from_para, from_char, forward, case_sensitive) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(query, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_searchText(this.__wbg_ptr, ptr0, len0, from_sec, from_para, from_char, forward, case_sensitive);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {boolean}
     */
    setActiveField(section_idx, para_idx, char_offset) {
        const ret = wasm.hwpdocument_setActiveField(this.__wbg_ptr, section_idx, para_idx, char_offset);
        return ret !== 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @returns {boolean}
     */
    setActiveFieldByPath(section_idx, parent_para_idx, path_json, char_offset) {
        const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_setActiveFieldByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset);
        return ret !== 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @param {boolean} is_textbox
     * @returns {boolean}
     */
    setActiveFieldInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, is_textbox) {
        const ret = wasm.hwpdocument_setActiveFieldInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset, is_textbox);
        return ret !== 0;
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} cell_path_json
     * @param {number} inner_control_idx
     * @param {string} props_json
     * @returns {string}
     */
    setCellPicturePropertiesByPath(section_idx, parent_para_idx, cell_path_json, inner_control_idx, props_json) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setCellPicturePropertiesByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, inner_control_idx, ptr1, len1);
            var ptr3 = ret[0];
            var len3 = ret[1];
            if (ret[3]) {
                ptr3 = 0; len3 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred4_0 = ptr3;
            deferred4_1 = len3;
            return getStringFromWasm0(ptr3, len3);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {string} props_json
     * @returns {string}
     */
    setCellProperties(section_idx, parent_para_idx, control_idx, cell_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setCellProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} cell_path_json
     * @param {number} inner_control_idx
     * @param {string} props_json
     * @returns {string}
     */
    setCellShapePropertiesByPath(section_idx, parent_para_idx, cell_path_json, inner_control_idx, props_json) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(cell_path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setCellShapePropertiesByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, inner_control_idx, ptr1, len1);
            var ptr3 = ret[0];
            var len3 = ret[1];
            if (ret[3]) {
                ptr3 = 0; len3 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred4_0 = ptr3;
            deferred4_1 = len3;
            return getStringFromWasm0(ptr3, len3);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * @param {boolean} enabled
     */
    setClipEnabled(enabled) {
        wasm.hwpdocument_setClipEnabled(this.__wbg_ptr, enabled);
    }
    /**
     * @param {number} section_idx
     * @param {number} column_count
     * @param {number} column_type
     * @param {number} same_width
     * @param {number} spacing_hu
     * @returns {string}
     */
    setColumnDef(section_idx, column_count, column_type, same_width, spacing_hu) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_setColumnDef(this.__wbg_ptr, section_idx, column_count, column_type, same_width, spacing_hu);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} dpi
     */
    setDpi(dpi) {
        wasm.hwpdocument_setDpi(this.__wbg_ptr, dpi);
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {string} props_json
     * @returns {string}
     */
    setEquationProperties(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setEquationProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} field_id
     * @param {string} value
     * @returns {string}
     */
    setFieldValue(field_id, value) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setFieldValue(this.__wbg_ptr, field_id, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {string} name
     * @param {string} value
     * @returns {string}
     */
    setFieldValueByName(name, value) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setFieldValueByName(this.__wbg_ptr, ptr0, len0, ptr1, len1);
            deferred3_0 = ret[0];
            deferred3_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {string} name
     */
    setFileName(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.hwpdocument_setFileName(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {string} value_json
     * @returns {string}
     */
    setFormValue(section_idx, paragraph_idx, control_idx, value_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(value_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setFormValue(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} table_para
     * @param {number} table_ci
     * @param {number} cell_idx
     * @param {number} cell_para
     * @param {number} form_ci
     * @param {string} value_json
     * @returns {string}
     */
    setFormValueInCell(section_idx, table_para, table_ci, cell_idx, cell_para, form_ci, value_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(value_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setFormValueInCell(this.__wbg_ptr, section_idx, table_para, table_ci, cell_idx, cell_para, form_ci, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} outer_para_idx
     * @param {number} outer_control_idx
     * @param {number} inner_para_idx
     * @param {number} inner_control_idx
     * @param {string} props_json
     * @returns {string}
     */
    setHeaderFooterPictureProperties(section_idx, outer_para_idx, outer_control_idx, inner_para_idx, inner_control_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setHeaderFooterPictureProperties(this.__wbg_ptr, section_idx, outer_para_idx, outer_control_idx, inner_para_idx, inner_control_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} control_idx
     * @param {number} note_para_idx
     * @param {number} equation_idx
     * @param {string} props_json
     * @returns {string}
     */
    setNoteEquationProperties(section_idx, para_idx, control_idx, note_para_idx, equation_idx, props_json) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setNoteEquationProperties(this.__wbg_ptr, section_idx, para_idx, control_idx, note_para_idx, equation_idx, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} mode
     * @param {number} start_num
     * @returns {string}
     */
    setNumberingRestart(section_idx, paragraph_idx, mode, start_num) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_setNumberingRestart(this.__wbg_ptr, section_idx, paragraph_idx, mode, start_num);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {string} settings_json
     * @returns {string}
     */
    setPageBorderFill(section_idx, settings_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(settings_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setPageBorderFill(this.__wbg_ptr, section_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {string} page_def_json
     * @returns {string}
     */
    setPageDef(section_idx, page_def_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(page_def_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setPageDef(this.__wbg_ptr, section_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {string} props_json
     * @returns {string}
     */
    setPictureProperties(section_idx, parent_para_idx, control_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setPictureProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {string} section_def_json
     * @returns {string}
     */
    setSectionDef(section_idx, section_def_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(section_def_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setSectionDef(this.__wbg_ptr, section_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {string} section_def_json
     * @returns {string}
     */
    setSectionDefAll(section_def_json) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(section_def_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setSectionDefAll(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {string} props_json
     * @returns {string}
     */
    setShapeProperties(section_idx, parent_para_idx, control_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setShapeProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {boolean} enabled
     */
    setShowControlCodes(enabled) {
        wasm.hwpdocument_setShowControlCodes(this.__wbg_ptr, enabled);
    }
    /**
     * @param {boolean} enabled
     */
    setShowParagraphMarks(enabled) {
        wasm.hwpdocument_setShowParagraphMarks(this.__wbg_ptr, enabled);
    }
    /**
     * @param {boolean} enabled
     */
    setShowTransparentBorders(enabled) {
        wasm.hwpdocument_setShowTransparentBorders(this.__wbg_ptr, enabled);
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {string} props_json
     * @returns {string}
     */
    setTableProperties(section_idx, parent_para_idx, control_idx, props_json) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(props_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_setTableProperties(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, ptr0, len0);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    splitParagraph(section_idx, para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_splitParagraph(this.__wbg_ptr, section_idx, para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} cell_idx
     * @param {number} cell_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    splitParagraphInCell(section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_splitParagraphInCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, cell_idx, cell_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {string} path_json
     * @param {number} char_offset
     * @returns {string}
     */
    splitParagraphInCellByPath(section_idx, parent_para_idx, path_json, char_offset) {
        let deferred3_0;
        let deferred3_1;
        try {
            const ptr0 = passStringToWasm0(path_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_splitParagraphInCellByPath(this.__wbg_ptr, section_idx, parent_para_idx, ptr0, len0, char_offset);
            var ptr2 = ret[0];
            var len2 = ret[1];
            if (ret[3]) {
                ptr2 = 0; len2 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred3_0 = ptr2;
            deferred3_1 = len2;
            return getStringFromWasm0(ptr2, len2);
        } finally {
            wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} paragraph_idx
     * @param {number} control_idx
     * @param {number} fn_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    splitParagraphInFootnote(section_idx, paragraph_idx, control_idx, fn_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_splitParagraphInFootnote(this.__wbg_ptr, section_idx, paragraph_idx, control_idx, fn_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {boolean} is_header
     * @param {number} apply_to
     * @param {number} hf_para_idx
     * @param {number} char_offset
     * @returns {string}
     */
    splitParagraphInHeaderFooter(section_idx, is_header, apply_to, hf_para_idx, char_offset) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_splitParagraphInHeaderFooter(this.__wbg_ptr, section_idx, is_header, apply_to, hf_para_idx, char_offset);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} row
     * @param {number} col
     * @returns {string}
     */
    splitTableCell(section_idx, parent_para_idx, control_idx, row, col) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_splitTableCell(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, row, col);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} row
     * @param {number} col
     * @param {number} n_rows
     * @param {number} m_cols
     * @param {boolean} equal_row_height
     * @param {boolean} merge_first
     * @returns {string}
     */
    splitTableCellInto(section_idx, parent_para_idx, control_idx, row, col, n_rows, m_cols, equal_row_height, merge_first) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_splitTableCellInto(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, row, col, n_rows, m_cols, equal_row_height, merge_first);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @param {number} start_row
     * @param {number} start_col
     * @param {number} end_row
     * @param {number} end_col
     * @param {number} n_rows
     * @param {number} m_cols
     * @param {boolean} equal_row_height
     * @returns {string}
     */
    splitTableCellsInRange(section_idx, parent_para_idx, control_idx, start_row, start_col, end_row, end_col, n_rows, m_cols, equal_row_height) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_splitTableCellsInRange(this.__wbg_ptr, section_idx, parent_para_idx, control_idx, start_row, start_col, end_row, end_col, n_rows, m_cols, equal_row_height);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} page_num
     * @param {boolean} is_header
     * @returns {string}
     */
    toggleHideHeaderFooter(page_num, is_header) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_toggleHideHeaderFooter(this.__wbg_ptr, page_num, is_header);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     * @param {number} parent_para_idx
     * @param {number} control_idx
     * @returns {string}
     */
    ungroupShape(section_idx, parent_para_idx, control_idx) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ret = wasm.hwpdocument_ungroupShape(this.__wbg_ptr, section_idx, parent_para_idx, control_idx);
            var ptr1 = ret[0];
            var len1 = ret[1];
            if (ret[3]) {
                ptr1 = 0; len1 = 0;
                throw takeFromExternrefTable0(ret[2]);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {number} field_id
     * @param {string} guide
     * @param {string} memo
     * @param {string} name
     * @param {boolean} editable
     * @returns {string}
     */
    updateClickHereProps(field_id, guide, memo, name, editable) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(guide, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(memo, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len2 = WASM_VECTOR_LEN;
            const ret = wasm.hwpdocument_updateClickHereProps(this.__wbg_ptr, field_id, ptr0, len0, ptr1, len1, ptr2, len2, editable);
            deferred4_0 = ret[0];
            deferred4_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * @param {number} section_idx
     */
    updateConnectorsInSection(section_idx) {
        wasm.hwpdocument_updateConnectorsInSection(this.__wbg_ptr, section_idx);
    }
    /**
     * @param {number} style_id
     * @param {string} json
     * @returns {boolean}
     */
    updateStyle(style_id, json) {
        const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_updateStyle(this.__wbg_ptr, style_id, ptr0, len0);
        return ret !== 0;
    }
    /**
     * @param {number} style_id
     * @param {string} char_mods_json
     * @param {string} para_mods_json
     * @returns {boolean}
     */
    updateStyleShapes(style_id, char_mods_json, para_mods_json) {
        const ptr0 = passStringToWasm0(char_mods_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(para_mods_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.hwpdocument_updateStyleShapes(this.__wbg_ptr, style_id, ptr0, len0, ptr1, len1);
        return ret !== 0;
    }
}
if (Symbol.dispose) HwpDocument.prototype[Symbol.dispose] = HwpDocument.prototype.free;
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_ea4887a5f8f9a9db: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_fillRect_3c420f5077df8d3b: function(arg0, arg1, arg2, arg3, arg4) {
            arg0.fillRect(arg1, arg2, arg3, arg4);
        },
        __wbg_fillText_cdea0ac33ff3d2d1: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
        }, arguments); },
        __wbg_getContext_486aab500e1c34c9: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_instanceof_CanvasRenderingContext2d_d0cab9e931424c52: function(arg0) {
            let result;
            try {
                result = arg0 instanceof CanvasRenderingContext2D;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_setTransform_49a6e126738858db: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            arg0.setTransform(arg1, arg2, arg3, arg4, arg5, arg6);
        }, arguments); },
        __wbg_set_fillStyle_35471aa9a10a6686: function(arg0, arg1, arg2) {
            arg0.fillStyle = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_font_e2bce6175ef42bc3: function(arg0, arg1, arg2) {
            arg0.font = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_height_ad5056ea051acd78: function(arg0, arg1) {
            arg0.height = arg1 >>> 0;
        },
        __wbg_set_width_031bdecd763c5855: function(arg0, arg1) {
            arg0.width = arg1 >>> 0;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./rjtd_wasm_bg.js": import0,
    };
}

const HwpDocumentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_hwpdocument_free(ptr, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('rjtd_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
