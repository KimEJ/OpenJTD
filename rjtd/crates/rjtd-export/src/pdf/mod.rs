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
    let date = document
        .text_field_candidates()
        .iter()
        .any(|field| field.kind() == rjtd_model::DocumentTextFieldKind::PrintingDate)
        .then(local_print_date)
        .flatten();
    to_pdf_with_file_name_and_print_date(document, file_name, date.as_deref())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_pdf_with_file_name_and_print_date(
    document: &Document,
    file_name: &str,
    date: Option<&str>,
) -> Result<Vec<u8>, String> {
    let mut core = DocumentCore::from_document(document.clone());
    if !file_name.is_empty() {
        core.set_file_name(file_name);
    }
    if let Some(date) = date {
        core.set_print_date(date)
            .map_err(|error| error.to_string())?;
    }
    let mut svg_pages = Vec::new();

    for page in 0..core.page_count() {
        svg_pages.push(
            core.render_page_svg(page)
                .map_err(|error| error.to_string())?,
        );
    }

    svgs_to_pdf(&svg_pages, Some(&core))
}

#[cfg(all(not(target_arch = "wasm32"), unix))]
fn local_print_date() -> Option<String> {
    let output = std::process::Command::new("date")
        .arg("+%Y/%m/%d")
        .env("LC_ALL", "C")
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).ok())
        .flatten()
        .map(|date| date.trim().to_string())
}

#[cfg(all(not(target_arch = "wasm32"), not(unix)))]
fn local_print_date() -> Option<String> {
    None
}
