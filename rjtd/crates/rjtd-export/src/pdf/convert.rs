use super::fonts::{add_font_fallbacks, create_fontdb};
use super::patch::ensure_pdf_form_xobject_form_types;
use super::safety::{scrub_embedded_pdf_eof_markers, validate_pdf_preview_safety};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn svgs_to_pdf(svg_pages: &[String]) -> Result<Vec<u8>, String> {
    if svg_pages.is_empty() {
        return Err("no pages to export".to_string());
    }

    let options = usvg::Options {
        fontdb: std::sync::Arc::new(create_fontdb()),
        ..Default::default()
    };

    use pdf_writer::{Finish, Pdf, Ref};
    use std::collections::HashMap;

    let mut alloc = Ref::new(1);
    let catalog_ref = alloc.bump();
    let page_tree_ref = alloc.bump();

    struct PageData {
        chunk: pdf_writer::Chunk,
        svg_ref: Ref,
        width: f32,
        height: f32,
    }

    let mut page_datas = Vec::new();

    for svg in svg_pages {
        let svg_with_fallback = add_font_fallbacks(svg);
        let tree = usvg::Tree::from_str(&svg_with_fallback, &options)
            .map_err(|error| format!("SVG parse failed: {error}"))?;
        let (chunk, svg_ref) = svg2pdf::to_chunk(&tree, svg2pdf::ConversionOptions::default())
            .map_err(|error| format!("SVG chunk conversion failed: {error:?}"))?;
        let dpi_ratio = 72.0 / 96.0;
        page_datas.push(PageData {
            chunk,
            svg_ref,
            width: tree.size().width() * dpi_ratio,
            height: tree.size().height() * dpi_ratio,
        });
    }

    let mut page_refs = Vec::new();
    let mut renumbered_chunks = Vec::new();
    let mut svg_refs_remapped = Vec::new();

    for page_data in &page_datas {
        let page_ref = alloc.bump();
        page_refs.push(page_ref);
        let mut map = HashMap::new();
        let renumbered = page_data
            .chunk
            .renumber(|old| *map.entry(old).or_insert_with(|| alloc.bump()));
        let remapped_svg_ref = map
            .get(&page_data.svg_ref)
            .copied()
            .unwrap_or(page_data.svg_ref);
        renumbered_chunks.push(renumbered);
        svg_refs_remapped.push(remapped_svg_ref);
    }

    let mut pdf = Pdf::new();
    pdf.set_version(1, 4);
    pdf.catalog(catalog_ref).pages(page_tree_ref);
    pdf.pages(page_tree_ref)
        .count(page_refs.len() as i32)
        .kids(page_refs.iter().copied());

    let svg_name = pdf_writer::Name(b"S1");
    for (index, page_data) in page_datas.iter().enumerate() {
        let page_ref = page_refs[index];
        let content_ref = alloc.bump();
        let svg_ref = svg_refs_remapped[index];

        let mut page = pdf.page(page_ref);
        page.media_box(pdf_writer::Rect::new(
            0.0,
            0.0,
            page_data.width,
            page_data.height,
        ));
        page.parent(page_tree_ref);
        page.contents(content_ref);

        let mut resources = page.resources();
        resources.proc_sets_all();
        resources.x_objects().pair(svg_name, svg_ref);
        resources.finish();
        page.finish();

        let mut content = pdf_writer::Content::new();
        content.save_state();
        content.set_fill_rgb(1.0, 1.0, 1.0);
        content.rect(0.0, 0.0, page_data.width, page_data.height);
        content.fill_nonzero();
        content.restore_state();
        content.save_state();
        content.transform([page_data.width, 0.0, 0.0, page_data.height, 0.0, 0.0]);
        content.x_object(svg_name);
        content.restore_state();
        pdf.stream(content_ref, &content.finish());
    }

    for chunk in &renumbered_chunks {
        pdf.extend(chunk);
    }

    let info_ref = alloc.bump();
    pdf.document_info(info_ref)
        .producer(pdf_writer::TextStr("rjtd"));

    let mut bytes = pdf.finish();
    scrub_embedded_pdf_eof_markers(&mut bytes);
    ensure_pdf_form_xobject_form_types(&mut bytes)?;
    validate_pdf_preview_safety(&bytes)?;
    Ok(bytes)
}
