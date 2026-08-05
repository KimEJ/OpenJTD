#[cfg(not(target_arch = "wasm32"))]
use rjtd_export::to_pdf_with_file_name;
use rjtd_export::{to_html, to_json, to_markdown, to_plain_text};
use rjtd_model::{DocumentCore, parse_document};

use crate::input::read_file;

use super::render_support::*;
use super::support::*;

pub(crate) fn run_page_layer_tree(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "page-layer-tree")?;
    let page_index = required_page_index(args.next(), "page-layer-tree")?;
    let bytes = read_file(&path)?;
    let mut core = DocumentCore::from_bytes(&bytes).map_err(|error| error.to_string())?;
    core.set_file_name(&path);
    write_stdout(
        &core
            .get_page_layer_tree(page_index)
            .map_err(|error| error.to_string())?,
    )
}

pub(crate) fn run_page_info(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "page-info")?;
    let page_index = required_page_index(args.next(), "page-info")?;
    let bytes = read_file(&path)?;
    let mut core = DocumentCore::from_bytes(&bytes).map_err(|error| error.to_string())?;
    core.set_file_name(&path);
    write_stdout(
        &core
            .get_page_info(page_index)
            .map_err(|error| error.to_string())?,
    )
}

pub(crate) fn run_document_info(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "document-info")?;
    let bytes = read_file(&path)?;
    let mut core = DocumentCore::from_bytes(&bytes).map_err(|error| error.to_string())?;
    core.set_file_name(&path);
    write_stdout(&core.get_document_info())
}

pub(crate) fn run_page_svg(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "page-svg")?;
    let page_index = required_page_index(args.next(), "page-svg")?;
    let bytes = read_file(&path)?;
    let mut core = DocumentCore::from_bytes(&bytes).map_err(|error| error.to_string())?;
    core.set_file_name(&path);
    write_stdout(
        &core
            .render_page_svg(page_index)
            .map_err(|error| error.to_string())?,
    )
}

pub(crate) fn run_export(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "export")?;
    let options = export_options(args)?;
    let bytes = read_file(&path)?;
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;

    match options.format.as_str() {
        "json" => write_stdout(&to_json(&document))?,
        "md" | "markdown" => write_stdout(&to_markdown(&document))?,
        "txt" | "text" => write_stdout(&to_plain_text(&document))?,
        "pdf" => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let Some(output_path) = options.output.as_deref() else {
                    return Err(
                        "PDF export requires `-o <output.pdf>` or `--output <output.pdf>`".into(),
                    );
                };
                let pdf = to_pdf_with_file_name(&document, &path)?;
                write_file(output_path, &pdf)?;
            }
            #[cfg(target_arch = "wasm32")]
            {
                return Err("PDF export is only available on native targets".into());
            }
        }
        "html" => {
            write_stdout(&to_html(&document))?;
        }
        other => return Err(format!("unsupported export format: {other}")),
    }
    Ok(())
}
