#![doc = include_str!("../README.md")]

use rjtd_core::{ParseLimits, ResourceBudget};
use rjtd_model::{Document, DocumentCore};
use wasm_bindgen::prelude::*;

#[cfg(any(test, target_arch = "wasm32"))]
mod canvas;

mod equations;
mod footnotes;
mod forms_fields;
mod header_footer;
mod html_io;
mod lifecycle;
mod navigation;
mod pictures_shapes;
mod rendering;
mod search_selection;
mod structure;
mod styles;
mod tables;
mod text_editing;

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
        Self::from_bytes_with_limits(data, ParseLimits::DEFAULT)
    }

    pub fn from_bytes_with_limits(data: &[u8], limits: ParseLimits) -> rjtd_core::Result<Self> {
        let mut budget = limits.resource_budget();
        Self::from_bytes_with_budget(data, &mut budget)
    }

    pub fn from_bytes_with_budget(
        data: &[u8],
        budget: &mut ResourceBudget,
    ) -> rjtd_core::Result<Self> {
        DocumentCore::from_bytes_with_budget(data, budget).map(|core| Self { core })
    }

    pub fn from_document(document: Document) -> Self {
        Self {
            core: DocumentCore::from_document(document),
        }
    }

    pub fn from_document_with_limits(
        document: Document,
        limits: ParseLimits,
    ) -> rjtd_core::Result<Self> {
        DocumentCore::from_document_with_limits(document, limits).map(|core| Self { core })
    }
}

fn js_error(error: rjtd_core::Error) -> JsValue {
    JsValue::from_str(&js_error_message(&error))
}

fn js_error_message(error: &rjtd_core::Error) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_aware_byte_loading_preserves_resource_limit_error_message() {
        let error = match HwpDocument::from_bytes_with_limits(
            &[0],
            ParseLimits::DEFAULT.with_max_input_bytes(0),
        ) {
            Err(error) => error,
            Ok(_) => panic!("expected resource-limit error"),
        };

        assert!(matches!(
            &error,
            rjtd_core::Error::ResourceLimit {
                resource: "input bytes",
                limit: 0,
                actual: 1,
            }
        ));
        assert_eq!(
            js_error_message(&error),
            "resource limit exceeded: input bytes is 1, limit is 0"
        );
    }

    #[test]
    fn limits_aware_document_entry_point_rejects_page_allocation() {
        let result = HwpDocument::from_document_with_limits(
            Document::from_plain_text("page"),
            ParseLimits::DEFAULT.with_max_pages(0),
        );

        assert!(matches!(
            result,
            Err(rjtd_core::Error::ResourceLimit {
                resource: "document pages",
                limit: 0,
                actual: 1,
            })
        ));
    }

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
