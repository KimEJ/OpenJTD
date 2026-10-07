#[cfg(feature = "rendering")]
mod render;
mod source;

#[cfg(feature = "rendering")]
pub(crate) use render::linked_footnote_body_line_top;
pub use source::DocumentFootnoteTextCandidate;
pub(crate) use source::linked_footnote_marker_script_basis;
