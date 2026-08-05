#![doc = include_str!("../README.md")]

mod html;
mod json;
mod pdf;
mod text;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

pub use html::to_html;
pub use json::to_json;
#[cfg(not(target_arch = "wasm32"))]
pub use pdf::{to_pdf, to_pdf_with_file_name};
pub use text::{to_markdown, to_plain_text};
