#![doc = include_str!("../README.md")]

pub mod auto_text_info;
pub mod compressed_document;
pub mod container;
pub mod document_text;
pub mod document_text_position;
pub mod error;
pub mod font_stream;
pub mod format;
pub mod layout_mark;
pub mod lha;
pub mod limits;
pub mod record;
pub mod stream;
pub mod style_stream;

pub use error::{Error, Result};
pub use limits::{DecompressionBudget, ParseLimits};
