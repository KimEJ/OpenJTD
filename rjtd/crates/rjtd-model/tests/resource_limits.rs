use std::io::{Cursor, Write};

use rjtd_core::compressed_document::JUST_COMPRESSED_DOCUMENT_MAGIC;
use rjtd_core::{Error, ParseLimits};
use rjtd_model::{parse_document, parse_document_with_limits};

#[test]
fn rejects_document_input_over_default_limit() {
    // Given
    let actual = ParseLimits::DEFAULT.max_input_bytes() + 1;
    let bytes = vec![0; actual];

    // When
    let result = parse_document(&bytes);

    // Then
    assert!(matches!(
        result,
        Err(Error::ResourceLimit {
            resource: "input bytes",
            limit,
            actual: reported,
        }) if limit == ParseLimits::DEFAULT.max_input_bytes() && reported == actual
    ));
}

#[test]
fn applies_custom_input_limit_at_public_parse_entry() {
    // Given
    let limits = ParseLimits::DEFAULT.with_max_input_bytes(3);
    let bytes = [0; 4];

    // When
    let result = parse_document_with_limits(&bytes, limits);

    // Then
    assert_eq!(
        result,
        Err(Error::ResourceLimit {
            resource: "input bytes",
            limit: 3,
            actual: 4,
        })
    );
}

#[test]
fn propagates_resource_limit_from_optional_compressed_stream() {
    // Given
    let bytes = document_with_compressed_stream(&oversized_lh5_stream());
    let limits = ParseLimits::DEFAULT.with_max_decompressed_bytes(1);

    // When
    let result = parse_document_with_limits(&bytes, limits);

    // Then
    assert_eq!(
        result,
        Err(Error::ResourceLimit {
            resource: "LH5 decompressed bytes",
            limit: 1,
            actual: 2,
        })
    );
}

#[test]
fn tolerates_malformed_optional_compressed_stream() {
    // Given
    let bytes = document_with_compressed_stream(JUST_COMPRESSED_DOCUMENT_MAGIC);

    // When
    let result = parse_document_with_limits(&bytes, ParseLimits::DEFAULT);

    // Then
    assert!(result.is_ok());
}

fn document_with_compressed_stream(compressed: &[u8]) -> Vec<u8> {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    {
        let mut stream = compound.create_stream("/DocumentText").unwrap();
        stream.write_all(b"SsmgV.01").unwrap();
    }
    {
        let mut stream = compound.create_stream("/JSCompDocument").unwrap();
        stream.write_all(compressed).unwrap();
    }
    compound.into_inner().into_inner()
}

fn oversized_lh5_stream() -> Vec<u8> {
    let mut bytes = JUST_COMPRESSED_DOCUMENT_MAGIC.to_vec();
    bytes.extend_from_slice(&[22, 0, b'-', b'l', b'h', b'5', b'-']);
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&2_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&[0x20, 0, 0, 0, 0]);
    bytes
}
