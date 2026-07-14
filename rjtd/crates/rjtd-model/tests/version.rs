use rjtd_model::{Document, DocumentCore};

#[test]
fn document_info_reports_package_version() {
    // Given
    let core = DocumentCore::from_document(Document::from_plain_text("preview"));

    // When
    let info = core.get_document_info();

    // Then
    assert!(info.contains(&format!("\"version\":\"{}\"", env!("CARGO_PKG_VERSION"))));
}
