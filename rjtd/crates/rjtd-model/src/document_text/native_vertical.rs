#[cfg(feature = "rendering")]
mod render;
mod source;

#[cfg(feature = "rendering")]
pub(crate) use render::*;
pub use source::{DocumentTatechuyokoCandidate, WritingMode};
pub(crate) use source::{modern_source_writing_mode, native_tatechuyoko_candidates};
#[cfg(feature = "rendering")]
pub(crate) use source::{modern_view_writing_mode, native_tatechuyoko_end};
