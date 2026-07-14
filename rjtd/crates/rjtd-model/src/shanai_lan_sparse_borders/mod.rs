use super::*;
use rjtd_core::document_text::{
    DocumentTextRowHeaderPairClassification, DocumentTextStyleDiagnostic,
    DocumentTextStyleDiagnosticKind, DocumentTextStyleEvent, DocumentTextStyleSection,
    DocumentTextStyleTypedValue, parse_document_text_style_section,
};

const DOCUMENT_VIEW_STYLES_X_ORIGIN_LEFT_OFFSET: usize = 152;
const DOCUMENT_VIEW_STYLES_Y_ORIGIN_OFFSET: usize = 176;
const DOCUMENT_VIEW_STYLES_X_ORIGIN_RIGHT_OFFSET: usize = 180;
const SOURCE_PAGE_TRANSFORM_BOTTOM_OFFSET_DENOMINATOR: u32 = 2;
const SPARSE_TABLE_BORDER_DASH_LENGTH_PX: f32 = 3.2;
const SPARSE_TABLE_BORDER_THIN_STROKE_WIDTH_PX: f32 = 0.8;
const SPARSE_TABLE_BORDER_THICK_STROKE_WIDTH_PX: f32 = 2.56;

mod candidate_json;
mod candidates;
mod geometry;
mod geometry_json;
mod layer_json;
mod render;
mod row_json;
mod style_json;
mod topology;
mod transform;
mod types;

pub(super) use layer_json::push_page_layer_shanai_lan_sparse_table_border_topology_diagnostic_json;
pub(super) use render::push_shanai_lan_sparse_table_borders_svg;
pub(super) use topology::shanai_lan_sparse_table_border_topology_diagnostic;
#[cfg(test)]
pub(super) use transform::shanai_lan_source_page_transform_candidate_from_raw_fields;
