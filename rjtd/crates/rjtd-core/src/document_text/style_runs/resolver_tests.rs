use super::{DocumentTextStyleResolver, DocumentTextStyleTypedValue};

#[test]
fn resolves_persistent_property_state_at_document_text_units() {
    // Given: one explicit green text color and border style followed by an auto-color reset.
    let bytes = synthetic_document_text_with_style_section(
        5,
        &[
            0xfe, 0x01, 0x02, 0x00, 0x04, 0x0f, 0x04, 0x00, 0x00, 0x80, 0x00, 0xff, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0xfe, 0x0f, 0x04, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ],
    );

    // When: the style stream is resolved into source-unit state.
    let resolver = DocumentTextStyleResolver::from_document_text_bytes(&bytes);

    // Then: changed properties persist through runs and reset at the exact change unit.
    assert_eq!(
        resolver.style_at_unit(18).and_then(|style| style.value(1)),
        Some(DocumentTextStyleTypedValue::U16(4))
    );
    assert_eq!(
        resolver.style_at_unit(18).and_then(|style| style.value(15)),
        Some(DocumentTextStyleTypedValue::U32(0x0000_8000))
    );
    assert_eq!(
        resolver.style_at_unit(19).and_then(|style| style.value(15)),
        Some(DocumentTextStyleTypedValue::U32(u32::MAX))
    );
    assert_eq!(resolver.content_unit_count(), 5);
    assert!(!resolver.truncated(), "{:?}", resolver.diagnostics());
}

fn synthetic_document_text_with_style_section(
    content_unit_count: u32,
    style_bytes: &[u8],
) -> Vec<u8> {
    let style_start = 32 + usize::try_from(content_unit_count).ok().unwrap_or(0) * 2;
    let mut bytes = vec![0; style_start];
    bytes[28..32].copy_from_slice(&content_unit_count.to_be_bytes());
    bytes.extend_from_slice(style_bytes);
    bytes
}
