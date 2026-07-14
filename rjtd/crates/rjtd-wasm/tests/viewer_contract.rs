use std::path::PathBuf;

fn viewer_html() -> Option<String> {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    if project_root.join("rjtd/Cargo.toml").is_file() {
        return Some(
            std::fs::read_to_string(project_root.join("openjtd.github.io/index.html"))
                .expect("repository viewer must be readable"),
        );
    }
    None
}

#[test]
fn viewer_sets_file_name_before_reading_page_count() {
    let Some(viewer_html) = viewer_html() else {
        return;
    };
    let constructor_offset = viewer_html
        .find("doc = new HwpDocument(bytes);")
        .expect("viewer must construct the WASM document from the selected file");
    let after_constructor = &viewer_html[constructor_offset..];
    let file_name_offset = after_constructor
        .find("doc.setFileName(file.name);")
        .expect("viewer must pass the selected file name to the WASM document");
    let page_count_offset = after_constructor
        .find("totalPages = doc.pageCount();")
        .expect("viewer must read the document page count");

    assert!(
        file_name_offset < page_count_offset,
        "viewer must apply filename-backed source hints before rendering pages"
    );
}

#[test]
fn viewer_exposes_open_errors_in_the_visible_drop_zone() {
    let Some(viewer_html) = viewer_html() else {
        return;
    };
    assert!(
        viewer_html.contains("id=\"drop-error\"")
            && viewer_html.contains("class=\"drop-zone__error\"")
            && viewer_html.contains("role=\"alert\""),
        "viewer must provide an accessible error surface beside the file picker"
    );
    assert!(
        viewer_html.contains(
            "document.getElementById('drop-error').textContent = msg.startsWith('エラー:') ? msg : '';"
        ),
        "file-open failures must remain visible while the viewer itself is hidden"
    );
    assert!(
        viewer_html.contains("resetViewer(`エラー: ${e}`);")
            && viewer_html.contains("function resetViewer(status = '')")
            && viewer_html.contains("document.getElementById('drop-zone').style.display = 'flex';"),
        "failed replacement uploads must return to the drop zone before exposing the error"
    );
}
