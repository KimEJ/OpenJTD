use std::fs;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static SAMPLE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn assert_json_brackets_balanced(json: &str) {
    let mut stack = Vec::new();
    let mut in_string = false;
    let mut escaped = false;

    for (offset, byte) in json.bytes().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            match byte {
                b'\\' => escaped = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => stack.push(byte),
            b'}' => assert_eq!(stack.pop(), Some(b'{'), "unmatched }} at byte {offset}"),
            b']' => assert_eq!(stack.pop(), Some(b'['), "unmatched ] at byte {offset}"),
            _ => {}
        }
    }

    assert!(!in_string, "unterminated JSON string");
    assert!(stack.is_empty(), "unclosed JSON delimiters: {stack:?}");
}

pub(crate) fn tiny_cfb_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/\u{4}JSRV_SegmentInformation")
        .unwrap()
        .write_all(b"segment")
        .unwrap();
    compound
        .create_stream("/DocInfo")
        .unwrap()
        .write_all(b"doc")
        .unwrap();
    compound.create_storage("/BodyText").unwrap();
    compound
        .create_stream("/BodyText/Section0")
        .unwrap()
        .write_all(b"hello")
        .unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn compressed_jttc_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/JSCompDocument")
        .unwrap()
        .write_all(b"\x26\0JustCompressedDocument\0-lh5-\0payload")
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn embedded_document_text_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut embedded = b"prefix SsmgV.01".to_vec();
    embedded.extend_from_slice(&[0x00, 0x1f]);
    for unit in "Note".encode_utf16() {
        embedded.extend_from_slice(&unit.to_be_bytes());
    }
    compound
        .create_stream("/JSSlipObject1")
        .unwrap()
        .write_all(&embedded)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn duplicate_stream_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/Needle")
        .unwrap()
        .write_all(b"needle")
        .unwrap();
    compound
        .create_stream("/Haystack")
        .unwrap()
        .write_all(b"xxneedleneedle")
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn so_record_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/Object")
        .unwrap()
        .write_all(&[
            b'x', b'x', b'S', b'O', 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        ])
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn object_stream_candidates_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound.create_storage("/EmbedItems").unwrap();
    compound.create_storage("/EmbedItems/Embedding 1").unwrap();
    let mut embedded_object = utf16le_fixture("JSFART.OBJECT");
    embedded_object.extend_from_slice(&[
        b'S', b'O', 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
    ]);
    compound
        .create_stream("/EmbedItems/Embedding 1/JSFart2Contents")
        .unwrap()
        .write_all(&embedded_object)
        .unwrap();
    let mut embedded_press = vec![0; 0x80];
    embedded_press[..12].copy_from_slice(b"JSSnapShot32");
    embedded_press[0x24..0x28].copy_from_slice(&3656u32.to_le_bytes());
    embedded_press[0x34..0x38].copy_from_slice(&17u32.to_le_bytes());
    embedded_press[0x48..0x4c].copy_from_slice(&2590u32.to_le_bytes());
    embedded_press[0x4c..0x50].copy_from_slice(&460u32.to_le_bytes());
    compound
        .create_stream("/EmbedItems/Embedding 1/\x03EmbeddedPress")
        .unwrap()
        .write_all(&embedded_press)
        .unwrap();
    compound
        .create_stream("/EmbedItems/Embedding 1/JSEQ3Contents")
        .unwrap()
        .write_all(&jseq3_contents_fixture())
        .unwrap();
    compound.create_storage("/EmbedItems/Embedding 2").unwrap();
    compound
        .create_stream("/EmbedItems/Embedding 2/Image.png")
        .unwrap()
        .write_all(minimal_jpeg_payload())
        .unwrap();
    compound
        .create_stream("/Figure")
        .unwrap()
        .write_all(&[
            b'S', b'O', 0x00, 0x00, 0xff, 0x09, 0x02, 0x00, 0xa0, 0x08, 0x00, 0x02,
        ])
        .unwrap();
    compound.create_storage("/Tables").unwrap();
    compound
        .create_stream("/Tables/Table1")
        .unwrap()
        .write_all(b"table payload")
        .unwrap();
    compound
        .create_stream("/Vector.svg")
        .unwrap()
        .write_all(br#"<?xml version="1.0"?><svg viewBox="0 0 10 10"></svg>"#)
        .unwrap();
    compound
        .create_stream("/VisualList")
        .unwrap()
        .write_all(b"\x00\x00\x08\xf8BMDV visual payload")
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn jseq3_contents_fixture() -> Vec<u8> {
    let mut stream = b"M\0A\0T\0H\0.\0V\0A\0F\0".to_vec();
    stream.extend(utf16le_fixture("Times New Roman"));
    stream.extend(utf16le_fixture("JustUnitMark"));
    stream.extend(utf16le_fixture("JustOubunMark"));
    stream.resize(116, 0);
    for field in [
        0x0000_4f53u32,
        0x200e_0a20,
        0x17ee_8d1a,
        0x4f7a_78ca,
        0,
        0,
        0x0000_8d1a,
        0x0000_1c7a,
        0,
    ] {
        stream.extend_from_slice(&field.to_le_bytes());
    }
    stream.resize(180, 0);
    stream
}

pub(crate) fn utf16le_fixture(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

pub(crate) fn object_frame_reference_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound.create_storage("/EmbedItems").unwrap();
    compound.create_storage("/EmbedItems/Embedding 2").unwrap();
    compound
        .create_stream("/EmbedItems/Embedding 2/Image.jpg")
        .unwrap()
        .write_all(minimal_jpeg_payload())
        .unwrap();
    compound
        .create_stream("/Frame")
        .unwrap()
        .write_all(&[
            0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01,
        ])
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn object_frame_row_link_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound.create_storage("/EmbedItems").unwrap();
    compound.create_storage("/EmbedItems/Embedding 2").unwrap();
    compound
        .create_stream("/EmbedItems/Embedding 2/Image.jpg")
        .unwrap()
        .write_all(minimal_jpeg_payload())
        .unwrap();

    let suffix_row = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    ];
    let mut frame = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00];
    frame.extend_from_slice(&suffix_row);
    frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    frame.extend_from_slice(&suffix_row);
    compound
        .create_stream("/Frame")
        .unwrap()
        .write_all(&frame)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn object_fdm_index_path() -> PathBuf {
    object_fdm_index_with_vector_tail(tiny_png_payload())
}

pub(crate) fn object_fdm_signature_only_path() -> PathBuf {
    object_fdm_index_with_vector_tail(b"\x89PNG\r\n\x1a\ntruncated")
}

pub(crate) fn object_fdm_index_with_vector_tail(vector_tail: &[u8]) -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound.create_storage("/FigureData").unwrap();
    compound.create_storage("/FigureData/main_data").unwrap();

    let mut index = vec![
        0x03, 0x0b, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02,
    ];
    index.extend_from_slice(&0_u32.to_be_bytes());
    index.extend_from_slice(&0x0b00_u16.to_be_bytes());
    for value in [1_i32, 2, 3, 4] {
        index.extend_from_slice(&value.to_be_bytes());
    }
    index.extend_from_slice(&32_u32.to_be_bytes());
    index.extend_from_slice(&0x0b00_u16.to_be_bytes());
    for value in [-1_i32, -2, 10, 20] {
        index.extend_from_slice(&value.to_be_bytes());
    }

    let mut vector = vec![0x11; 32];
    vector.extend_from_slice(b"head");
    vector.extend_from_slice(vector_tail);
    compound
        .create_stream("/FigureData/main_data/FDMIndex")
        .unwrap()
        .write_all(&index)
        .unwrap();
    compound
        .create_stream("/FigureData/main_data/FDMVector")
        .unwrap()
        .write_all(&vector)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn object_fdm_frame_link_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound.create_storage("/FigureData").unwrap();
    compound.create_storage("/FigureData/main_data").unwrap();

    let mut index = vec![
        0x03, 0x0b, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02,
    ];
    index.extend_from_slice(&0_u32.to_be_bytes());
    index.extend_from_slice(&0x0b00_u16.to_be_bytes());
    for value in [1_i32, 2, 3, 4] {
        index.extend_from_slice(&value.to_be_bytes());
    }
    index.extend_from_slice(&32_u32.to_be_bytes());
    index.extend_from_slice(&0x0b00_u16.to_be_bytes());
    for value in [-1_i32, -2, 10, 20] {
        index.extend_from_slice(&value.to_be_bytes());
    }

    let mut vector = vec![0x11; 32];
    vector.extend_from_slice(b"head");
    vector.extend_from_slice(tiny_png_payload());
    compound
        .create_stream("/FigureData/main_data/FDMIndex")
        .unwrap()
        .write_all(&index)
        .unwrap();
    compound
        .create_stream("/FigureData/main_data/FDMVector")
        .unwrap()
        .write_all(&vector)
        .unwrap();

    let mut frame = vec![
        0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x01, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x02,
    ];
    frame.extend_from_slice(&fdm_frame_record_fixture(0, 0x0004, (11, 22, 33, 44)));
    frame.extend_from_slice(&fdm_frame_record_fixture(1, 0x0007, (100, 200, 300, 400)));
    compound
        .create_stream("/Frame")
        .unwrap()
        .write_all(&frame)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn fdm_frame_record_fixture(
    object_id: u16,
    object_type: u16,
    geometry: (u16, u16, u16, u16),
) -> Vec<u8> {
    let mut row = vec![0; 60];
    row[0..2].copy_from_slice(&0x0102_u16.to_be_bytes());
    row[2..4].copy_from_slice(&0x0038_u16.to_be_bytes());
    row[6..8].copy_from_slice(&object_id.to_be_bytes());
    row[12..14].copy_from_slice(&object_type.to_be_bytes());
    row[28..30].copy_from_slice(&geometry.0.to_be_bytes());
    row[32..34].copy_from_slice(&geometry.1.to_be_bytes());
    row[36..38].copy_from_slice(&geometry.2.to_be_bytes());
    row[40..42].copy_from_slice(&geometry.3.to_be_bytes());
    row
}

pub(crate) fn minimal_jpeg_payload() -> &'static [u8] {
    &[
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x10,
        0x00, 0x20, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff, 0xda, 0x00,
        0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x00, 0xff, 0xd9,
    ]
}

pub(crate) fn tiny_png_payload() -> &'static [u8] {
    &[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f,
        0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0d, 0x0a, 0x2d, 0xb4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ]
}

pub(crate) fn object_fdm_index_shape_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound.create_storage("/FigureData").unwrap();
    compound.create_storage("/FigureData/main_data").unwrap();

    let mut index = vec![
        0x03, 0x0b, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01,
    ];
    index.extend_from_slice(&32_u32.to_be_bytes());
    index.extend_from_slice(&0x0b00_u16.to_be_bytes());
    for value in [1_i32, 2, 3, 4] {
        index.extend_from_slice(&value.to_be_bytes());
    }
    index.extend_from_slice(&0xffff_fff0_u32.to_be_bytes());
    index.extend_from_slice(&0xffff_u16.to_be_bytes());
    for value in [-1_i32, -2, -3, -4] {
        index.extend_from_slice(&value.to_be_bytes());
    }

    let mut vector = vec![0x11; 32];
    vector.extend_from_slice(b"head");
    vector.extend_from_slice(b"\xff\xd8\xffdata\xff\xd9");
    compound
        .create_stream("/FigureData/main_data/FDMIndex")
        .unwrap()
        .write_all(&index)
        .unwrap();
    compound
        .create_stream("/FigureData/main_data/FDMVector")
        .unwrap()
        .write_all(&vector)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn object_fdm_index_mixed_rows_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound.create_storage("/FigureData").unwrap();
    compound.create_storage("/FigureData/main_data").unwrap();

    let mut index = vec![
        0x03, 0x0b, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02,
    ];
    index.extend_from_slice(&32_u32.to_be_bytes());
    index.extend_from_slice(&0x0b00_u16.to_be_bytes());
    for value in [1_i32, 2, 3, 4] {
        index.extend_from_slice(&value.to_be_bytes());
    }
    index.extend_from_slice(&[
        0x06, 0x00, 0xff, 0xff, 0xd3, 0xc0, 0xff, 0xff, 0xd5, 0xbc, 0xff, 0xff, 0xc0, 0x28, 0xff,
        0xff, 0xc2, 0x21, 0x00, 0x00, 0x00, 0x40,
    ]);
    index.extend_from_slice(&[
        0x0a, 0x00, 0xff, 0xff, 0xd3, 0x48, 0xff, 0xff, 0xd5, 0x4b, 0xff, 0xff, 0xc0, 0x00, 0xff,
        0xff, 0xc2, 0x01, 0x00, 0x00, 0x00, 0x00,
    ]);

    let mut vector = vec![0x11; 32];
    vector.extend_from_slice(b"head");
    vector.extend_from_slice(b"\xff\xd8\xffdata\xff\xd9");
    compound
        .create_stream("/FigureData/main_data/FDMIndex")
        .unwrap()
        .write_all(&index)
        .unwrap();
    compound
        .create_stream("/FigureData/main_data/FDMVector")
        .unwrap()
        .write_all(&vector)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn so_record_cluster_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let record = [
        b'S', b'O', 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x64, 0x00, 0x00, 0x00,
    ];
    compound
        .create_stream("/First")
        .unwrap()
        .write_all(&record)
        .unwrap();
    let mut second = b"xx".to_vec();
    second.extend_from_slice(&record);
    compound
        .create_stream("/Second")
        .unwrap()
        .write_all(&second)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn so_record_geometry_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut record = Vec::new();
    for field in [
        0x00004f53, 0x000009ff, 0x000008a0, 0x0000139a, 0x000008a0, 0, 0, 0, 0,
    ] {
        record.extend_from_slice(&u32::to_le_bytes(field));
    }
    compound
        .create_stream("/Geometry")
        .unwrap()
        .write_all(&record)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn so_record_packed_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut record = Vec::new();
    for field in [
        0x00004f53, 0x200e0a20, 0x17ee8d1a, 0x4f7a78ca, 0, 0, 0x00008d1a, 0x00001c7a, 0,
    ] {
        record.extend_from_slice(&u32::to_le_bytes(field));
    }
    compound
        .create_stream("/Packed")
        .unwrap()
        .write_all(&record)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn skipped_inline_document_text_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_with_skipped_inline())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn control_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_with_repeated_controls())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn control_cluster_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_with_control_cluster())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn position_table_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&position_table_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_table_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn shifted_text_count_table_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut entry = [0; 29];
    entry[1..5].copy_from_slice(&0x0000_96cau32.to_be_bytes());
    entry[5..9].copy_from_slice(&0x0000_96cau32.to_be_bytes());
    entry[9..17].copy_from_slice(&[0x01, 0x01, 0x00, 0x41, 0x00, 0x4f, 0x01, 0x00]);
    entry[17..21].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);
    entry[25..29].copy_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_raw_entries(&[entry]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_delta_table_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut be0 = [0; 29];
    be0[0..4].copy_from_slice(&100u32.to_be_bytes());
    be0[4..8].copy_from_slice(&112u32.to_be_bytes());
    be0[8..28].copy_from_slice(&[
        0x01, 0x01, 0x00, 0x0a, 0x00, 0x16, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00,
    ]);

    let mut shifted = [0; 29];
    shifted[1..5].copy_from_slice(&0x0000_96cau32.to_be_bytes());
    shifted[5..9].copy_from_slice(&0x0000_96cau32.to_be_bytes());
    shifted[9..29].copy_from_slice(&[
        0x01, 0x01, 0x00, 0x41, 0x00, 0x4f, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00,
    ]);

    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_raw_entries(&[be0, shifted]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_tail_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut be0 = [0; 29];
    be0[0..4].copy_from_slice(&100u32.to_be_bytes());
    be0[4..8].copy_from_slice(&112u32.to_be_bytes());
    be0[8..28].copy_from_slice(&[
        0x01, 0x01, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00,
    ]);

    let mut shifted = [0; 29];
    shifted[1..5].copy_from_slice(&0x0000_96cau32.to_be_bytes());
    shifted[5..9].copy_from_slice(&0x0000_96cau32.to_be_bytes());
    shifted[9..29].copy_from_slice(&[
        0x01, 0x01, 0x00, 0x09, 0x00, 0x0b, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00,
    ]);

    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_raw_entries(&[be0, shifted]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_ranges(&[(10, 13), (5, 6)]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_boundary_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_ranges(&[(10, 16), (7, 8)]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_table_candidate_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_ranges(&[(0, 30)]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn sparse_table_candidate_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_with_sparse_table_rows())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_finance_table_candidate_path() -> PathBuf {
    let document_text = document_text_with_finance_table_rows();
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text)
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_ranges(&[(
            0,
            document_text.len() as u32,
        )]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_cluster_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_ranges(&[
            (10, 13),
            (10, 13),
            (20, 24),
        ]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_map_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&position_table_fixture_with_offsets(&[(1, 10), (2, 5)]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn mark_summary_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&position_table_fixture())
        .unwrap();
    compound
        .create_stream("/LineMark")
        .unwrap()
        .write_all(&[0x09, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00])
        .unwrap();

    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&2u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&1u32.to_be_bytes());
    for index in 0..=2u32 {
        let mut entry = [0; 84];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        page_mark.extend_from_slice(&entry);
    }
    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    let mut paper_mark = Vec::new();
    paper_mark.extend_from_slice(&2u32.to_be_bytes());
    paper_mark.extend_from_slice(&0x0cu32.to_be_bytes());
    paper_mark.extend_from_slice(&1u32.to_be_bytes());
    for index in 0..=2u32 {
        paper_mark.extend_from_slice(&index.to_be_bytes());
        paper_mark.extend_from_slice(&(0x0001_0000u32 + index).to_be_bytes());
    }
    compound
        .create_stream("/PaperMark")
        .unwrap()
        .write_all(&paper_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn line_mark_tags_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut bytes = Vec::new();
    for word in [
        0x0914, 0x0000, 0x0001, 0x1002, 0x0077, 0x0002, 0x1000, 0x0074, 0x1001, 0x000d,
    ] {
        bytes.extend_from_slice(&u16::to_be_bytes(word));
    }
    compound
        .create_stream("/LineMark")
        .unwrap()
        .write_all(&bytes)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn line_mark_intervals_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut bytes = vec![0; 18];
    bytes[8..10].copy_from_slice(&u16::to_be_bytes(3));
    for (delta, flag) in [(5u16, 0x0002u16), (8, 0x8002), (0, 0x0000)] {
        bytes.extend_from_slice(&u16::to_be_bytes(delta));
        bytes.extend_from_slice(&u16::to_be_bytes(flag));
    }
    compound
        .create_stream("/LineMark")
        .unwrap()
        .write_all(&bytes)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn line_mark_text_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut line_mark = Vec::new();
    for word in [
        0x0914, 0x0000, 0x0001, 0x1002, 0x0041, 0x0002, 0x1000, 0x0074, 0x1001, 0x000d,
    ] {
        line_mark.extend_from_slice(&u16::to_be_bytes(word));
    }
    compound
        .create_stream("/LineMark")
        .unwrap()
        .write_all(&line_mark)
        .unwrap();

    let mut document_text = Vec::new();
    for word in [0x001f, 0x0041, 0x0042, 0x0074, 0x0043] {
        document_text.extend_from_slice(&u16::to_be_bytes(word));
    }
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_position_line_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut line_mark = Vec::new();
    for word in [
        0x0914, 0x0000, 0x1002, 0x0041, 0x1000, 0x0074, 0x000d, 0x1001, 0x000a,
    ] {
        line_mark.extend_from_slice(&u16::to_be_bytes(word));
    }
    compound
        .create_stream("/LineMark")
        .unwrap()
        .write_all(&line_mark)
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&position_table_fixture_with_offsets(&[
            (1, 4),
            (2, 8),
            (3, 20),
        ]))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_count_layout_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut line_mark = Vec::new();
    for word in [0x0914, 0x0000, 0x1002, 0x0041, 0x1000, 0x0074] {
        line_mark.extend_from_slice(&u16::to_be_bytes(word));
    }
    compound
        .create_stream("/LineMark")
        .unwrap()
        .write_all(&line_mark)
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_ranges(&[(2, 12), (4, 5)]))
        .unwrap();

    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&2u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&1u32.to_be_bytes());
    for index in 0..3u32 {
        let mut entry = [0; 84];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        page_mark.extend_from_slice(&entry);
    }
    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    let mut paper_mark = Vec::new();
    paper_mark.extend_from_slice(&2u32.to_be_bytes());
    paper_mark.extend_from_slice(&0x0cu32.to_be_bytes());
    paper_mark.extend_from_slice(&1u32.to_be_bytes());
    for index in 0..3u32 {
        paper_mark.extend_from_slice(&index.to_be_bytes());
        paper_mark.extend_from_slice(&0x0001_0000u32.to_be_bytes());
    }
    compound
        .create_stream("/PaperMark")
        .unwrap()
        .write_all(&paper_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_boundary_layout_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_ranges(&[(9, 12)]))
        .unwrap();

    let mut line_mark = Vec::new();
    for index in 0..20u16 {
        let word = match index {
            8 => 0x1002,
            12 => 0x1000,
            _ => index,
        };
        line_mark.extend_from_slice(&u16::to_be_bytes(word));
    }
    compound
        .create_stream("/LineMark")
        .unwrap()
        .write_all(&line_mark)
        .unwrap();

    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&19u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&18u32.to_be_bytes());
    for index in 0..20u32 {
        let mut entry = [0; 84];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        page_mark.extend_from_slice(&entry);
    }
    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    let mut paper_mark = Vec::new();
    paper_mark.extend_from_slice(&19u32.to_be_bytes());
    paper_mark.extend_from_slice(&0x0cu32.to_be_bytes());
    paper_mark.extend_from_slice(&18u32.to_be_bytes());
    for index in 0..20u32 {
        paper_mark.extend_from_slice(&index.to_be_bytes());
        paper_mark.extend_from_slice(&0x0001_0000u32.to_be_bytes());
    }
    compound
        .create_stream("/PaperMark")
        .unwrap()
        .write_all(&paper_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_boundary_paragraph_like_style_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();

    let mut entry = [0; 29];
    entry[0..4].copy_from_slice(&9u32.to_be_bytes());
    entry[4..8].copy_from_slice(&13u32.to_be_bytes());
    entry[8..28].copy_from_slice(&[
        0x02, 0x02, 0x00, 0x01, 0x00, 0x2f, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00,
    ]);
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_raw_entries(&[entry]))
        .unwrap();

    let mut line_mark = Vec::new();
    for index in 0..20u16 {
        line_mark.extend_from_slice(&u16::to_be_bytes(index));
    }
    compound
        .create_stream("/LineMark")
        .unwrap()
        .write_all(&line_mark)
        .unwrap();

    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&19u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&18u32.to_be_bytes());
    for index in 0..20u32 {
        let mut entry = [0; 84];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        page_mark.extend_from_slice(&entry);
    }
    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    let mut paper_mark = Vec::new();
    paper_mark.extend_from_slice(&19u32.to_be_bytes());
    paper_mark.extend_from_slice(&0x0cu32.to_be_bytes());
    paper_mark.extend_from_slice(&18u32.to_be_bytes());
    for index in 0..20u32 {
        paper_mark.extend_from_slice(&index.to_be_bytes());
        paper_mark.extend_from_slice(&0x0001_0000u32.to_be_bytes());
    }
    compound
        .create_stream("/PaperMark")
        .unwrap()
        .write_all(&paper_mark)
        .unwrap();
    compound
        .create_stream("/TextLayoutStyle")
        .unwrap()
        .write_all(&ssmg_style_with_labeled_slots(0x5555, &["見出し", "本文"]))
        .unwrap();
    compound
        .create_stream("/PageLayoutStyle")
        .unwrap()
        .write_all(&ssmg_style_with_labeled_slots(0x4444, &["ページ"]))
        .unwrap();
    compound
        .create_stream("/DocumentViewStyles")
        .unwrap()
        .write_all(&document_view_style_group_fixture(1))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn raw_stream_path(bytes: &[u8]) -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/Raw")
        .unwrap()
        .write_all(bytes)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_probe_path() -> PathBuf {
    let mut bytes = b"\0Ver.2.3\0".to_vec();
    for unit in "Layout".encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.push(0);
    for unit in "Wide".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }

    raw_stream_path(&bytes)
}

pub(crate) fn style_stream_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/TextLayoutStyle")
        .unwrap()
        .write_all(&text_layout_style_with_label_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn page_layout_style_slot_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/PageLayoutStyle")
        .unwrap()
        .write_all(&page_layout_style_with_slot_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn a5_page_layer_tree_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_from_str(&a5_page_layer_tree_text()))
        .unwrap();
    compound
        .create_stream("/AutoTextInfo")
        .unwrap()
        .write_all(&auto_text_info_fixture("銀河鉄道の夜"))
        .unwrap();
    compound
        .create_stream("/PageLayoutStyle")
        .unwrap()
        .write_all(&page_layout_style_with_slot_fixture())
        .unwrap();

    write_named_sample("a5.jtd", compound.into_inner().into_inner())
}

pub(crate) fn a5_page_layer_tree_text() -> String {
    format!(
        "{}\n{}",
        "銀河鉄道の夜\t\t\t\t宮沢 賢治\n目次\n一、午后の授業\n銀河鉄道の夜\n一、午后の授業",
        "ではみなさんは、そういうふうに川だと云われたりしていました。".repeat(120)
    )
}

pub(crate) fn text_position_style_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut entry = [0; 29];
    entry[0..4].copy_from_slice(&10u32.to_be_bytes());
    entry[4..8].copy_from_slice(&16u32.to_be_bytes());
    entry[8..28].copy_from_slice(&[
        0x02, 0x02, 0x00, 0x01, 0x00, 0x2f, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00,
    ]);
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture_with_raw_entries(&[entry]))
        .unwrap();
    compound
        .create_stream("/TextLayoutStyle")
        .unwrap()
        .write_all(&ssmg_style_with_labeled_slots(0x5555, &["見出し", "本文"]))
        .unwrap();
    compound
        .create_stream("/PageLayoutStyle")
        .unwrap()
        .write_all(&ssmg_style_with_labeled_slots(0x4444, &["ページ"]))
        .unwrap();
    compound
        .create_stream("/DocumentViewStyles")
        .unwrap()
        .write_all(&document_view_style_group_fixture(1))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn text_layout_style_with_label_fixture() -> Vec<u8> {
    ssmg_style_with_labeled_slots(0x5555, &["\u{672c}\u{6587}"])
}

pub(crate) fn page_layout_style_with_slot_fixture() -> Vec<u8> {
    let mut bytes = vec![0; 0x114];
    bytes[0..8].copy_from_slice(b"SsmgV.01");

    let mut payload = Vec::new();
    payload.extend_from_slice(&3u16.to_be_bytes());
    for unit in "ページ".encode_utf16() {
        payload.extend_from_slice(&unit.to_be_bytes());
    }
    payload.extend_from_slice(&[0, 0]);
    payload.extend_from_slice(&[0x31, 0x04, 0, 1, 0xaa]);
    payload.extend_from_slice(&[0x31, 0x05, 0, 2, 0x04, 0x00]);
    payload.extend_from_slice(&[0x31, 0x06, 0, 1, 0xbb]);
    payload.extend_from_slice(&[0x31, 0x07, 0, 1, 0xcc]);
    payload.extend_from_slice(&[0x32, 0x05, 0, 2, 0x04, 0x00]);
    payload.extend_from_slice(&[0x33, 0x05, 0, 2, 0x04, 0x00]);

    bytes.extend_from_slice(&0x4444u16.to_be_bytes());
    bytes.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    bytes.extend_from_slice(&payload);
    bytes
}

pub(crate) fn ssmg_style_with_labeled_slots(code: u16, labels: &[&str]) -> Vec<u8> {
    let mut bytes = vec![0; 0x114];
    bytes[0..8].copy_from_slice(b"SsmgV.01");

    for label in labels {
        let aligned_len = if bytes.len() <= 0x114 {
            0x114
        } else {
            0x114 + (bytes.len() - 0x114).div_ceil(0x100) * 0x100
        };
        bytes.resize(aligned_len, 0);

        let mut payload = Vec::new();
        payload.extend_from_slice(&(label.encode_utf16().count() as u16).to_be_bytes());
        for unit in label.encode_utf16() {
            payload.extend_from_slice(&unit.to_be_bytes());
        }

        bytes.extend_from_slice(&code.to_be_bytes());
        bytes.extend_from_slice(&(payload.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&payload);
    }
    bytes
}

pub(crate) fn document_view_style_group_fixture(group_id: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    for low in 0x04..=0x07u16 {
        let code = (0x30 + group_id) << 8 | low;
        bytes.extend_from_slice(&code.to_be_bytes());
        bytes.extend_from_slice(&1u16.to_be_bytes());
        bytes.push(low as u8);
    }
    bytes
}

pub(crate) fn document_view_styles_sequential_fixture(first_code: u16) -> Vec<u8> {
    let mut bytes = vec![0u8; 10];
    for offset in 0..4u16 {
        let code = first_code + offset;
        bytes.extend_from_slice(&code.to_be_bytes());
        bytes.extend_from_slice(&1u16.to_be_bytes());
        bytes.push(0);
    }
    bytes
}

pub(crate) fn document_info_path_with_document_view_styles(first_code: u16) -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound
        .create_stream("/DocumentViewStyles")
        .unwrap()
        .write_all(&document_view_styles_sequential_fixture(first_code))
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn document_view_style_ungrouped_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut bytes = Vec::new();
    for low in 0x04..=0x07u16 {
        let code = 0x1000 | low;
        bytes.extend_from_slice(&code.to_be_bytes());
        bytes.extend_from_slice(&1u16.to_be_bytes());
        bytes.push(low as u8);
    }
    compound
        .create_stream("/DocumentViewStyles")
        .unwrap()
        .write_all(&bytes)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn paper_mark_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    let mut paper_mark = Vec::new();
    paper_mark.extend_from_slice(&2u32.to_be_bytes());
    paper_mark.extend_from_slice(&0x0cu32.to_be_bytes());
    paper_mark.extend_from_slice(&1u32.to_be_bytes());
    paper_mark.extend_from_slice(&0u32.to_be_bytes());
    paper_mark.extend_from_slice(&0x0001_0010u32.to_be_bytes());
    paper_mark.extend_from_slice(&1u32.to_be_bytes());
    paper_mark.extend_from_slice(&0x0001_0011u32.to_be_bytes());
    paper_mark.extend_from_slice(&2u32.to_be_bytes());
    paper_mark.extend_from_slice(&0x0001_0000u32.to_be_bytes());
    compound
        .create_stream("/PaperMark")
        .unwrap()
        .write_all(&paper_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn page_mark_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&2u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&1u32.to_be_bytes());
    for index in 0..=2u32 {
        let mut entry = [0; 84];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        entry[4..8].copy_from_slice(&(0x0001_0000u32 + index).to_be_bytes());
        page_mark.extend_from_slice(&entry);
    }
    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn set_page_mark_entry_u16(entry: &mut [u8; 84], word_index: usize, value: u16) {
    let offset = word_index * 2;
    entry[offset..offset + 2].copy_from_slice(&u16::to_be_bytes(value));
}

pub(crate) fn page_mark_u16_profile_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&3u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&2u32.to_be_bytes());

    let mut zero = [0; 84];
    zero[0..4].copy_from_slice(&0u32.to_be_bytes());
    zero[4..8].copy_from_slice(&0x0001_0000u32.to_be_bytes());
    page_mark.extend_from_slice(&zero);

    let mut additive_row = [0; 84];
    additive_row[0..4].copy_from_slice(&1u32.to_be_bytes());
    additive_row[4..8].copy_from_slice(&0x0001_0000u32.to_be_bytes());
    set_page_mark_entry_u16(&mut additive_row, 10, 353);
    set_page_mark_entry_u16(&mut additive_row, 13, 353);
    set_page_mark_entry_u16(&mut additive_row, 14, 246);
    set_page_mark_entry_u16(&mut additive_row, 17, 353);
    set_page_mark_entry_u16(&mut additive_row, 18, 353);
    set_page_mark_entry_u16(&mut additive_row, 19, 353);
    set_page_mark_entry_u16(&mut additive_row, 20, 8);
    set_page_mark_entry_u16(&mut additive_row, 21, 599);
    page_mark.extend_from_slice(&additive_row);

    let mut additive_boundary = [0; 84];
    additive_boundary[0..4].copy_from_slice(&2u32.to_be_bytes());
    additive_boundary[4..8].copy_from_slice(&0x0001_0000u32.to_be_bytes());
    set_page_mark_entry_u16(&mut additive_boundary, 10, 370);
    set_page_mark_entry_u16(&mut additive_boundary, 13, 370);
    set_page_mark_entry_u16(&mut additive_boundary, 14, 185);
    set_page_mark_entry_u16(&mut additive_boundary, 17, 370);
    set_page_mark_entry_u16(&mut additive_boundary, 18, 370);
    set_page_mark_entry_u16(&mut additive_boundary, 19, 370);
    set_page_mark_entry_u16(&mut additive_boundary, 20, 255);
    set_page_mark_entry_u16(&mut additive_boundary, 21, 555);
    page_mark.extend_from_slice(&additive_boundary);

    let mut mixed = [0; 84];
    mixed[0..4].copy_from_slice(&3u32.to_be_bytes());
    mixed[4..8].copy_from_slice(&0x0001_0000u32.to_be_bytes());
    set_page_mark_entry_u16(&mut mixed, 13, 1);
    set_page_mark_entry_u16(&mut mixed, 14, 2);
    set_page_mark_entry_u16(&mut mixed, 21, 4);
    page_mark.extend_from_slice(&mixed);

    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn page_mark_variable_shape_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&3u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&2u32.to_be_bytes());
    for index in 0..4u32 {
        let mut entry = [0; 20];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        entry[4..8].copy_from_slice(&(0x0100_0000u32 + index).to_be_bytes());
        page_mark.extend_from_slice(&entry);
    }
    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn page_mark_count_variable_shape_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&5u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&4u32.to_be_bytes());
    for index in 0..5u32 {
        let mut entry = [0; 20];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        entry[4..8].copy_from_slice(&(0x0200_0000u32 + index).to_be_bytes());
        page_mark.extend_from_slice(&entry);
    }
    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn page_mark_fixed84_tail_shape_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    let mut page_mark = Vec::new();
    page_mark.extend_from_slice(&6u32.to_be_bytes());
    page_mark.extend_from_slice(&0x10u32.to_be_bytes());
    page_mark.extend_from_slice(&4u32.to_be_bytes());
    for index in 0..2u32 {
        let mut entry = [0; 84];
        entry[0..4].copy_from_slice(&index.to_be_bytes());
        entry[4..8].copy_from_slice(&(0x0300_0000u32 + index).to_be_bytes());
        page_mark.extend_from_slice(&entry);
    }
    page_mark.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
    compound
        .create_stream("/PageMark")
        .unwrap()
        .write_all(&page_mark)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

pub(crate) fn write_sample(bytes: Vec<u8>) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = SAMPLE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "rjtd-streams-{}-{nonce}-{counter}.jtd",
        std::process::id()
    ));
    fs::write(&path, bytes).unwrap();
    path
}

pub(crate) fn write_named_sample(file_name: &str, bytes: Vec<u8>) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = SAMPLE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "rjtd-streams-{}-{nonce}-{counter}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(file_name);
    fs::write(&path, bytes).unwrap();
    path
}

pub(crate) fn document_text_fixture() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x1f]);
    for unit in "銀河".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes.extend_from_slice(&[0x00, 0x1c]);
    bytes.extend_from_slice(&[0x00, 0x1f]);
    for unit in "鉄道\n".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

pub(crate) fn document_text_from_str(text: &str) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x1f]);
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

pub(crate) fn auto_text_info_fixture(text: &str) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.resize(84, 0);
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

pub(crate) fn document_text_with_finance_table_rows() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    for unit in "　　売掛金2,441,9973,983,602△1,541,6042,766,830".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(&mut bytes, &[0x001c, 0x001f]);
    for unit in "流動資産合計4,249,16115.54,988,33217,327".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

pub(crate) fn document_text_with_sparse_table_rows() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    append_sparse_table_row(&mut bytes, &["", "", "(1)表面積", ""]);
    append_sparse_table_row(&mut bytes, &["", "１", "", ""]);
    append_sparse_table_row(&mut bytes, &["", "ＡＢ　＝　ｃｍ", ""]);
    append_sparse_table_row(&mut bytes, &["", "ＡＣ　＝　ｃｍ", ""]);
    bytes
}

pub(crate) fn append_sparse_table_row(bytes: &mut Vec<u8>, cells: &[&str]) {
    for (cell_index, cell) in cells.iter().enumerate() {
        if cell_index > 0 {
            extend_units(bytes, &[0x001c, 0x001f]);
        } else if !cell.is_empty() {
            extend_units(bytes, &[0x001f]);
        }
        if !cell.is_empty() {
            for unit in cell.encode_utf16() {
                bytes.extend_from_slice(&unit.to_be_bytes());
            }
        }
    }
    extend_units(bytes, &[0x000e]);
}

pub(crate) fn document_text_with_skipped_inline() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x1f]);
    for unit in "本文".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(
        &mut bytes,
        &[0x001c, 0x0001, 0x0007, 0x0000, 0x0001, 0x0082, 0x001d],
    );
    for unit in "ふりがな".encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    extend_units(&mut bytes, &[0x001e]);
    bytes
}

pub(crate) fn document_text_with_repeated_controls() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(
        &mut bytes,
        &[
            0x001f, 0x0041, 0x001c, 0x001f, 0x0042, 0x000e, 0x001f, 0x0043,
        ],
    );
    bytes
}

pub(crate) fn document_text_with_control_cluster() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(
        &mut bytes,
        &[
            0x001f, 0x0041, 0x000e, 0x001d, 0x001f, 0x0042, 0x001c, 0x001f, 0x0043,
        ],
    );
    bytes
}

pub(crate) fn position_table_fixture() -> Vec<u8> {
    position_table_fixture_with_offsets(&[(1, 0x1234), (2, 0x5678)])
}

pub(crate) fn position_table_fixture_with_offsets(entries: &[(u16, u32)]) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    bytes.extend_from_slice(&[0x00, 0x00, 0x01, 0x00]);
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    bytes.extend_from_slice(b"TCntV.01");
    bytes.extend_from_slice(&[0x00, 0x00]);
    bytes.extend_from_slice(b"MarkV.01");
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
    for (id, offset) in entries {
        bytes.extend_from_slice(&id.to_be_bytes());
        bytes.extend_from_slice(&offset.to_be_bytes());
    }
    bytes.extend_from_slice(&[0xff, 0xff, 0xff, 0xff]);
    bytes
}

pub(crate) fn text_count_table_fixture() -> Vec<u8> {
    text_count_table_fixture_with_ranges(&[(0x1234, 0x1250), (0x2000, 0x2400)])
}

pub(crate) fn text_count_table_fixture_with_ranges(entries: &[(u32, u32)]) -> Vec<u8> {
    let mut raw_entries = Vec::new();
    for (index, (start, end)) in entries.iter().enumerate() {
        let mut entry = [0; 29];
        entry[0..4].copy_from_slice(&start.to_be_bytes());
        entry[4..8].copy_from_slice(&end.to_be_bytes());
        entry[8..12].copy_from_slice(&[0x01, 0x01, 0x00, 0x05 + index as u8]);
        entry[20..24].copy_from_slice(&[0x00, 0x00, 0x00, 0x01]);
        raw_entries.push(entry);
    }
    text_count_table_fixture_with_raw_entries(&raw_entries)
}

pub(crate) fn text_count_table_fixture_with_raw_entries(entries: &[[u8; 29]]) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    bytes.extend_from_slice(&[0x00, 0x00, 0x01, 0x00]);
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    bytes.extend_from_slice(b"TCntV.01");
    bytes.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]);
    bytes.extend_from_slice(&(entries.len() as u16).to_be_bytes());
    bytes.extend_from_slice(&[0x00, 0x24]);
    for entry in entries {
        bytes.extend_from_slice(entry);
    }
    bytes
}

pub(crate) fn extend_units(bytes: &mut Vec<u8>, units: &[u16]) {
    for unit in units {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
}
