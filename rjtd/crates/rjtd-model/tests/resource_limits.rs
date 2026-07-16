use std::collections::HashSet;
use std::io::{Cursor, Write};

use rjtd_core::compressed_document::JUST_COMPRESSED_DOCUMENT_MAGIC;
use rjtd_core::{Error, ParseLimits};
use rjtd_model::{Document, DocumentCore, parse_document, parse_document_with_limits};

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
fn rejects_cumulative_cfb_streams_at_public_parse_entry() {
    // Given
    let bytes = document_with_streams(&[("/DocumentText", b"SsmgV.01"), ("/LineMark", b"line")]);
    let limits = ParseLimits::DEFAULT.with_max_streams(1);

    // When
    let result = parse_document_with_limits(&bytes, limits);

    // Then
    assert_eq!(
        result,
        Err(Error::ResourceLimit {
            resource: "document streams",
            limit: 1,
            actual: 2,
        })
    );
}

#[test]
fn accepts_exact_cumulative_cfb_stream_bytes_and_rejects_limit_plus_one() {
    let bytes = document_with_streams(&[("/DocumentText", b"SsmgV.01"), ("/LineMark", b"line")]);

    assert!(
        parse_document_with_limits(&bytes, ParseLimits::DEFAULT.with_max_stream_bytes(12),).is_ok()
    );
    assert_eq!(
        parse_document_with_limits(&bytes, ParseLimits::DEFAULT.with_max_stream_bytes(11)),
        Err(Error::ResourceLimit {
            resource: "document stream bytes",
            limit: 11,
            actual: 12,
        })
    );
}

#[test]
fn rejects_cumulative_frame_records_at_public_parse_entry() {
    // Given
    let mut frame = vec![0; 16 + (2 * 60)];
    frame[14..16].copy_from_slice(&2_u16.to_be_bytes());
    let bytes = document_with_streams(&[("/DocumentText", b"SsmgV.01"), ("/Frame", &frame)]);
    let limits = ParseLimits::DEFAULT.with_max_records(1);

    // When
    let result = parse_document_with_limits(&bytes, limits);

    // Then
    assert_eq!(
        result,
        Err(Error::ResourceLimit {
            resource: "document records",
            limit: 1,
            actual: 2,
        })
    );
}

#[test]
fn accepts_exact_frame_record_bytes_and_rejects_limit_plus_one() {
    let mut frame = vec![0; 16 + (2 * 60)];
    frame[14..16].copy_from_slice(&2_u16.to_be_bytes());
    let bytes = document_with_streams(&[("/DocumentText", b"SsmgV.01"), ("/Frame", &frame)]);

    assert!(
        parse_document_with_limits(
            &bytes,
            ParseLimits::DEFAULT
                .with_max_records(2)
                .with_max_record_bytes(120),
        )
        .is_ok()
    );
    assert_eq!(
        parse_document_with_limits(&bytes, ParseLimits::DEFAULT.with_max_record_bytes(119),),
        Err(Error::ResourceLimit {
            resource: "document record bytes",
            limit: 119,
            actual: 120,
        })
    );
}

#[test]
fn rejects_cumulative_embedding_frame_records_before_their_vector_grows() {
    let embedding_info = embedding_info_frames(2);
    let bytes = document_with_streams(&[
        ("/DocumentText", b"SsmgV.01"),
        ("/EmbedItems/EmbeddingInfo", &embedding_info),
    ]);

    assert!(
        parse_document_with_limits(
            &bytes,
            ParseLimits::DEFAULT
                .with_max_records(2)
                .with_max_record_bytes(260),
        )
        .is_ok()
    );
    assert_eq!(
        parse_document_with_limits(&bytes, ParseLimits::DEFAULT.with_max_records(1)),
        Err(Error::ResourceLimit {
            resource: "document records",
            limit: 1,
            actual: 2,
        })
    );
    assert_eq!(
        parse_document_with_limits(&bytes, ParseLimits::DEFAULT.with_max_record_bytes(259),),
        Err(Error::ResourceLimit {
            resource: "document record bytes",
            limit: 259,
            actual: 260,
        })
    );
}

#[test]
fn rejects_cumulative_embedded_images_at_public_parse_entry() {
    // Given
    let bytes = document_with_streams(&[
        ("/DocumentText", b"SsmgV.01"),
        (
            "/image",
            &[minimal_jpeg_payload(), minimal_jpeg_payload()].concat(),
        ),
    ]);
    let limits = ParseLimits::DEFAULT.with_max_images(1);

    // When
    let result = parse_document_with_limits(&bytes, limits);

    // Then
    assert_eq!(
        result,
        Err(Error::ResourceLimit {
            resource: "embedded images",
            limit: 1,
            actual: 2,
        })
    );
}

#[test]
fn applies_embedded_image_payload_and_dimension_limits_before_retention() {
    let image = minimal_jpeg_payload();
    let bytes = document_with_streams(&[("/DocumentText", b"SsmgV.01"), ("/image", image)]);

    assert!(
        parse_document_with_limits(
            &bytes,
            ParseLimits::DEFAULT
                .with_max_image_bytes(image.len())
                .with_max_image_width(32)
                .with_max_image_height(16)
                .with_max_image_pixels(512),
        )
        .is_ok()
    );
    assert_eq!(
        parse_document_with_limits(
            &bytes,
            ParseLimits::DEFAULT.with_max_image_bytes(image.len() - 1),
        ),
        Err(Error::ResourceLimit {
            resource: "embedded image bytes",
            limit: image.len() - 1,
            actual: image.len(),
        })
    );
    assert_eq!(
        parse_document_with_limits(&bytes, ParseLimits::DEFAULT.with_max_image_width(31),),
        Err(Error::ResourceLimit {
            resource: "embedded image width",
            limit: 31,
            actual: 32,
        })
    );
    assert_eq!(
        parse_document_with_limits(&bytes, ParseLimits::DEFAULT.with_max_image_pixels(511),),
        Err(Error::ResourceLimit {
            resource: "embedded image pixels",
            limit: 511,
            actual: 512,
        })
    );
}

#[test]
fn rejects_cumulative_embedded_image_payload_bytes() {
    let image = [minimal_jpeg_payload(), minimal_jpeg_payload()].concat();
    let bytes = document_with_streams(&[("/DocumentText", b"SsmgV.01"), ("/image", &image)]);

    assert_eq!(
        parse_document_with_limits(
            &bytes,
            ParseLimits::DEFAULT.with_max_image_bytes(image.len() - 1),
        ),
        Err(Error::ResourceLimit {
            resource: "embedded image bytes",
            limit: image.len() - 1,
            actual: image.len(),
        })
    );
}

#[test]
fn preserves_one_budget_from_model_construction_through_page_construction() {
    let mut budget = ParseLimits::DEFAULT.with_max_pages(1).resource_budget();
    budget.reserve_page().unwrap();

    assert_resource_limit(
        DocumentCore::from_document_with_budget(Document::from_plain_text("page"), &mut budget),
        Error::ResourceLimit {
            resource: "document pages",
            limit: 1,
            actual: 2,
        },
    );
}

#[test]
fn applies_page_limits_at_default_and_custom_core_byte_entry_points() {
    let bytes = document_with_streams(&[("/DocumentText", b"SsmgV.01")]);

    assert!(DocumentCore::from_bytes(&bytes).is_ok());
    assert_resource_limit(
        DocumentCore::from_bytes_with_limits(&bytes, ParseLimits::DEFAULT.with_max_pages(0)),
        Error::ResourceLimit {
            resource: "document pages",
            limit: 0,
            actual: 1,
        },
    );
    assert_resource_limit(
        DocumentCore::from_document_with_limits(
            Document::from_plain_text("page"),
            ParseLimits::DEFAULT.with_max_page_lines(0),
        ),
        Error::ResourceLimit {
            resource: "document page lines",
            limit: 0,
            actual: 1,
        },
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
    document_with_streams(&[
        ("/DocumentText", b"SsmgV.01"),
        ("/JSCompDocument", compressed),
    ])
}

fn document_with_streams(streams: &[(&str, &[u8])]) -> Vec<u8> {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut storages = HashSet::new();
    for (path, payload) in streams {
        create_parent_storages(&mut compound, path, &mut storages);
        let mut stream = compound.create_stream(path).unwrap();
        stream.write_all(payload).unwrap();
    }
    compound.into_inner().into_inner()
}

fn create_parent_storages(
    compound: &mut cfb::CompoundFile<Cursor<Vec<u8>>>,
    path: &str,
    storages: &mut HashSet<String>,
) {
    let segments = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let mut current = String::new();
    for segment in &segments[..segments.len().saturating_sub(1)] {
        current.push('/');
        current.push_str(segment);
        if storages.insert(current.clone()) {
            compound.create_storage(&current).unwrap();
        }
    }
}

fn embedding_info_frames(count: usize) -> Vec<u8> {
    let mut bytes = vec![0; 16];
    bytes[..4].copy_from_slice(&(count as u32).to_le_bytes());
    for index in 0..count {
        let class_bytes = [b'A', 0, 0, 0];
        let row_start = bytes.len();
        bytes.resize(row_start + 46, 0);
        bytes[row_start + 8..row_start + 12]
            .copy_from_slice(&u32::try_from(index + 1).unwrap().to_le_bytes());
        bytes[row_start + 14..row_start + 16].copy_from_slice(&1_u16.to_le_bytes());
        bytes[row_start + 18..row_start + 20].copy_from_slice(&1_u16.to_le_bytes());
        bytes[row_start + 42..row_start + 46]
            .copy_from_slice(&(class_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&class_bytes);
        let trailing_start = bytes.len();
        bytes.resize(trailing_start + 80, 0);
        bytes[trailing_start..trailing_start + 4].copy_from_slice(&1_u32.to_le_bytes());
        bytes[trailing_start + 4..trailing_start + 8].copy_from_slice(&1_u32.to_le_bytes());
        bytes[trailing_start + 8..trailing_start + 12].copy_from_slice(&1_u32.to_le_bytes());
    }
    bytes
}

fn assert_resource_limit<T>(result: Result<T, Error>, expected: Error) {
    match result {
        Err(error) => assert_eq!(error, expected),
        Ok(_) => panic!("expected resource-limit error"),
    }
}

fn minimal_jpeg_payload() -> &'static [u8] {
    &[
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x10,
        0x00, 0x20, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff, 0xda, 0x00,
        0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x00, 0xff, 0xd9,
    ]
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
