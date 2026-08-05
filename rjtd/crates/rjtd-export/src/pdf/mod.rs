#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod convert;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod fonts;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod patch;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod safety;

#[cfg(not(target_arch = "wasm32"))]
use convert::svgs_to_pdf;

#[cfg(not(target_arch = "wasm32"))]
use rjtd_model::{Document, DocumentCore};

#[cfg(not(target_arch = "wasm32"))]
pub fn to_pdf(document: &Document) -> Result<Vec<u8>, String> {
    to_pdf_with_file_name(document, "")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_pdf_with_file_name(document: &Document, file_name: &str) -> Result<Vec<u8>, String> {
    let mut core = DocumentCore::from_document(document.clone());
    if !file_name.is_empty() {
        core.set_file_name(file_name);
    }
    let mut svg_pages = Vec::new();

    for page in 0..core.page_count() {
        svg_pages.push(
            core.render_page_svg(page)
                .map_err(|error| error.to_string())?,
        );
    }

    svgs_to_pdf(&svg_pages)
}
