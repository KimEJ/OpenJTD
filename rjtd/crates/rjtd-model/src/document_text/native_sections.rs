#[cfg(feature = "rendering")]
mod render;
mod source;

#[cfg(feature = "rendering")]
pub(crate) use render::*;
#[cfg(any(test, feature = "rendering"))]
pub(crate) use source::{native_section_marker, native_section_setting_line};

pub use source::NativeSectionSource as DocumentSectionSourceCandidate;
