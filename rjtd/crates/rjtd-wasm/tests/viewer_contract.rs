const VIEWER_HTML: &str = include_str!("../../../../openjtd.github.io/index.html");

#[test]
fn viewer_sets_file_name_before_reading_page_count() {
    let constructor_offset = VIEWER_HTML
        .find("doc = new HwpDocument(bytes);")
        .expect("viewer must construct the WASM document from the selected file");
    let after_constructor = &VIEWER_HTML[constructor_offset..];
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
    assert!(
        VIEWER_HTML.contains("id=\"drop-error\"")
            && VIEWER_HTML.contains("class=\"drop-zone__error\"")
            && VIEWER_HTML.contains("role=\"alert\""),
        "viewer must provide an accessible error surface beside the file picker"
    );
    assert!(
        VIEWER_HTML.contains(
            "document.getElementById('drop-error').textContent = msg.startsWith('エラー:') ? msg : '';"
        ),
        "file-open failures must remain visible while the viewer itself is hidden"
    );
    assert!(
        VIEWER_HTML.contains("resetViewer(`エラー: ${e}`);")
            && VIEWER_HTML.contains("function resetViewer(status = '')")
            && VIEWER_HTML.contains("document.getElementById('drop-zone').style.display = 'flex';"),
        "failed replacement uploads must return to the drop zone before exposing the error"
    );
}
