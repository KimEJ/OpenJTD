#[cfg(feature = "rendering")]
mod candidate_json;
mod candidate_types;
#[cfg(feature = "rendering")]
mod connector_endpoint_owner;
#[cfg(feature = "rendering")]
mod connector_graph_diagnostics;
#[cfg(feature = "rendering")]
mod connector_line_rule;
#[cfg(feature = "rendering")]
mod connector_order_trace;
mod constants;
mod diagnostic_types;
#[cfg(feature = "rendering")]
mod frame_diagnostics;
mod index_text_parsing;
mod native_figures;
#[cfg(feature = "rendering")]
mod paint_coverage;
#[cfg(feature = "rendering")]
mod success_data_svg;
#[cfg(feature = "rendering")]
mod text_mask;
mod vector_parsing;

pub use candidate_types::{
    ObjectFdmConnectorCandidate, ObjectFdmIndexBbox, ObjectFdmIndexEntryCandidate,
    ObjectFdmTextCandidate, ObjectFdmTextIndexEntryCandidate, ObjectFdmVectorCommandCandidate,
    ObjectFdmVectorCommandSourceSegment, ObjectFdmVectorCurveSegment, ObjectFdmVectorEllipse,
    ObjectFdmVectorPoint, ObjectFdmVectorSegmentCandidate,
};

#[cfg(feature = "rendering")]
pub(crate) use candidate_json::*;
pub(crate) use candidate_types::*;
#[cfg(feature = "rendering")]
pub(crate) use connector_endpoint_owner::*;
#[cfg(feature = "rendering")]
pub(crate) use connector_graph_diagnostics::*;
#[cfg(feature = "rendering")]
pub(crate) use connector_line_rule::*;
#[cfg(feature = "rendering")]
pub(crate) use connector_order_trace::*;
pub(crate) use constants::*;
#[cfg(feature = "rendering")]
pub(crate) use diagnostic_types::*;
#[cfg(feature = "rendering")]
pub(crate) use frame_diagnostics::*;
pub(crate) use index_text_parsing::*;
pub use native_figures::NativeFigureShapeCandidate;
#[cfg(feature = "rendering")]
pub(crate) use native_figures::*;
#[cfg(feature = "rendering")]
pub(crate) use paint_coverage::*;
#[cfg(feature = "rendering")]
pub(crate) use success_data_svg::*;
#[cfg(feature = "rendering")]
pub(crate) use text_mask::*;
pub(crate) use vector_parsing::*;
