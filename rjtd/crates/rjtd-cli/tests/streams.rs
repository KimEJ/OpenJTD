use std::fs;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static SAMPLE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn assert_json_brackets_balanced(json: &str) {
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

fn tiny_cfb_path() -> PathBuf {
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

fn compressed_jttc_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/JSCompDocument")
        .unwrap()
        .write_all(b"\x26\0JustCompressedDocument\0-lh5-\0payload")
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn embedded_document_text_path() -> PathBuf {
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

fn duplicate_stream_path() -> PathBuf {
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

fn so_record_path() -> PathBuf {
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

fn object_stream_candidates_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_fixture())
        .unwrap();
    compound.create_storage("/EmbedItems").unwrap();
    compound.create_storage("/EmbedItems/Embedding 1").unwrap();
    let mut embedded_object = b"object-prefix".to_vec();
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

fn jseq3_contents_fixture() -> Vec<u8> {
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

fn utf16le_fixture(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

fn object_frame_reference_path() -> PathBuf {
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

fn object_frame_row_link_path() -> PathBuf {
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

fn object_fdm_index_path() -> PathBuf {
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

    write_sample(compound.into_inner().into_inner())
}

fn object_fdm_frame_link_path() -> PathBuf {
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

fn fdm_frame_record_fixture(
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

fn minimal_jpeg_payload() -> &'static [u8] {
    &[
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x10,
        0x00, 0x20, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff, 0xda, 0x00,
        0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x00, 0xff, 0xd9,
    ]
}

fn tiny_png_payload() -> &'static [u8] {
    &[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f,
        0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0d, 0x0a, 0x2d, 0xb4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ]
}

fn object_fdm_index_shape_path() -> PathBuf {
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

fn object_fdm_index_mixed_rows_path() -> PathBuf {
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

fn so_record_cluster_path() -> PathBuf {
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

fn so_record_geometry_path() -> PathBuf {
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

fn so_record_packed_path() -> PathBuf {
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

fn skipped_inline_document_text_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_with_skipped_inline())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn control_context_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_with_repeated_controls())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn control_cluster_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_with_control_cluster())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn position_table_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&position_table_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn text_count_table_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentTextPositionTables")
        .unwrap()
        .write_all(&text_count_table_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn shifted_text_count_table_path() -> PathBuf {
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

fn text_count_delta_table_path() -> PathBuf {
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

fn text_count_tail_context_path() -> PathBuf {
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

fn text_count_context_path() -> PathBuf {
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

fn text_count_boundary_path() -> PathBuf {
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

fn text_count_table_candidate_path() -> PathBuf {
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

fn sparse_table_candidate_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/DocumentText")
        .unwrap()
        .write_all(&document_text_with_sparse_table_rows())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn text_count_finance_table_candidate_path() -> PathBuf {
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

fn text_count_cluster_path() -> PathBuf {
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

fn text_map_path() -> PathBuf {
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

fn mark_summary_path() -> PathBuf {
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

fn line_mark_tags_path() -> PathBuf {
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

fn line_mark_intervals_path() -> PathBuf {
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

fn line_mark_text_context_path() -> PathBuf {
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

fn text_position_line_context_path() -> PathBuf {
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

fn text_count_layout_context_path() -> PathBuf {
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

fn text_boundary_layout_context_path() -> PathBuf {
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

fn text_boundary_paragraph_like_style_context_path() -> PathBuf {
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

fn raw_stream_path(bytes: &[u8]) -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/Raw")
        .unwrap()
        .write_all(bytes)
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn text_probe_path() -> PathBuf {
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

fn style_stream_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/TextLayoutStyle")
        .unwrap()
        .write_all(&text_layout_style_with_label_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn page_layout_style_slot_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
    compound
        .create_stream("/PageLayoutStyle")
        .unwrap()
        .write_all(&page_layout_style_with_slot_fixture())
        .unwrap();

    write_sample(compound.into_inner().into_inner())
}

fn a5_page_layer_tree_path() -> PathBuf {
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

fn a5_page_layer_tree_text() -> String {
    format!(
        "{}\n{}",
        "銀河鉄道の夜\t\t\t\t宮沢 賢治\n目次\n一、午后の授業\n銀河鉄道の夜\n一、午后の授業",
        "ではみなさんは、そういうふうに川だと云われたりしていました。".repeat(120)
    )
}

fn text_position_style_context_path() -> PathBuf {
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

fn text_layout_style_with_label_fixture() -> Vec<u8> {
    ssmg_style_with_labeled_slots(0x5555, &["\u{672c}\u{6587}"])
}

fn page_layout_style_with_slot_fixture() -> Vec<u8> {
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

fn ssmg_style_with_labeled_slots(code: u16, labels: &[&str]) -> Vec<u8> {
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

fn document_view_style_group_fixture(group_id: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    for low in 0x04..=0x07u16 {
        let code = (0x30 + group_id) << 8 | low;
        bytes.extend_from_slice(&code.to_be_bytes());
        bytes.extend_from_slice(&1u16.to_be_bytes());
        bytes.push(low as u8);
    }
    bytes
}

fn document_view_style_ungrouped_path() -> PathBuf {
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

fn paper_mark_path() -> PathBuf {
    let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
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

fn page_mark_path() -> PathBuf {
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

fn set_page_mark_entry_u16(entry: &mut [u8; 84], word_index: usize, value: u16) {
    let offset = word_index * 2;
    entry[offset..offset + 2].copy_from_slice(&u16::to_be_bytes(value));
}

fn page_mark_u16_profile_path() -> PathBuf {
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

fn page_mark_variable_shape_path() -> PathBuf {
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

fn page_mark_count_variable_shape_path() -> PathBuf {
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

fn page_mark_fixed84_tail_shape_path() -> PathBuf {
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

fn write_sample(bytes: Vec<u8>) -> PathBuf {
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

fn write_named_sample(file_name: &str, bytes: Vec<u8>) -> PathBuf {
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

#[test]
fn streams_command_lists_cfb_entries() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("streams")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("stream\t7\t/\\x04JSRV_SegmentInformation"));
    assert!(stdout.contains("storage\t0\t/BodyText"));
    assert!(stdout.contains("stream\t5\t/BodyText/Section0"));
    assert!(stdout.contains("stream\t24\t/DocumentText"));
    assert!(stdout.contains("stream\t3\t/DocInfo"));
}

#[test]
fn info_command_reports_document_text_inventory() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("info")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("format\tcfb-document-text"));
    assert!(stdout.contains("streams\t4"));
    assert!(stdout.contains("storages\t1"));
    assert!(stdout.contains("document_text_bytes\t24"));
    assert!(stdout.contains("compressed_document_bytes\t-"));
}

#[test]
fn info_command_reports_compressed_jttc_inventory() {
    let path = compressed_jttc_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("info")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("format\tcfb-just-compressed-document"));
    assert!(stdout.contains("document_text_bytes\t-"));
    assert!(stdout.contains("compressed_document_bytes\t38"));
}

#[test]
fn info_command_reports_embedded_document_text_inventory() {
    let path = embedded_document_text_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("info")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("format\tcfb-embedded-document-text"));
    assert!(stdout.contains("document_text_bytes\t-"));
    assert!(stdout.contains("embedded_document_text\tpresent"));
}

#[test]
fn dump_stream_command_writes_raw_stream_to_stdout() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("dump-stream")
        .arg(&path)
        .arg("/BodyText/Section0")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"hello");
}

#[test]
fn style_records_command_reports_style_stream_record_summaries() {
    let path = style_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("style-records")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("style_streams\t1\n"));
    assert!(stdout.contains(
        "stream\t/TextLayoutStyle\tbytes=286\tfamily=ssmg\trecordLayout=ssmg-slots\trecordCount=1\theaderU32Be=0x00000000,0x00000000,0x00000000\theaderU16Be=0x0000,0x0000\n"
    ));
    assert!(
        stdout.contains(
            "record\t/TextLayoutStyle\t0\toffset=276\tcode=0x5555\tpayloadLength=6\tlabel="
        )
    );
    assert!(stdout.contains("\u{672c}\u{6587}\n"));
}

#[test]
fn page_layout_style_slots_command_reports_raw_slot_parts() {
    let path = page_layout_style_slot_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-layout-style-slots")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("summary\tstatus=ok\tstream=/PageLayoutStyle\tstream-bytes="));
    assert!(stdout.contains(
        "\trecords=1\tslots=3\tpaired-slot-pairs=0x32/0x33\tfacing-pages-candidate=true\tdecoded=false\n"
    ));
    assert!(stdout.contains(
        "record\t0\toffset=276\tcode=0x4444\tpayloadLength=43\tlabel=ページ\tsubrecords=6\n"
    ));
    assert!(stdout.contains(
        "slot\t0\t0x31\tpart05First=0x04\tpart05NonZero=true\tpart04=aa\tpart05=0400\tpart06=bb\tpart07=cc\n"
    ));
    assert!(stdout.contains(
        "slot\t0\t0x32\tpart05First=0x04\tpart05NonZero=true\tpart04=-\tpart05=0400\tpart06=-\tpart07=-\n"
    ));
    assert!(stdout.contains(
        "slot\t0\t0x33\tpart05First=0x04\tpart05NonZero=true\tpart04=-\tpart05=0400\tpart06=-\tpart07=-\n"
    ));
}

#[test]
fn page_layer_tree_command_reports_facing_page_decoration_evidence() {
    let path = a5_page_layer_tree_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-layer-tree")
        .arg(&path)
        .arg("5")
        .output()
        .unwrap();

    let parent = path.parent().map(PathBuf::from);
    fs::remove_file(&path).unwrap();
    if let Some(parent) = parent {
        fs::remove_dir(parent).unwrap();
    }

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_json_brackets_balanced(&stdout);
    assert!(stdout.contains("]},\"textSources\""));
    assert!(stdout.contains("\"type\":\"pageDecoration\""));
    assert!(stdout.contains("\"source\":\"autoTextInfo+pageLayoutStylePairedSlots+documentText\""));
    assert!(stdout.contains("\"projectionKind\":\"layoutStyleAutoTextProjection\""));
    assert!(stdout.contains("\"decoded\":false"));
    assert!(stdout.contains("\"sidePolicy\":\"facing-pages-odd-right-even-left\""));
    assert!(stdout.contains("\"sidePolicyDecoded\":false"));
    assert!(stdout.contains("\"facingPagesCandidate\":true"));
    assert!(stdout.contains("\"pairedSlotPairs\":[\"0x32/0x33\"]"));
    assert!(stdout.contains("\"slotEvidence\""));
    assert!(stdout.contains("\"side\":\"left\""));
    assert!(stdout.contains("\"pageNumber\":6"));
    assert!(stdout.contains("\"headerText\":\"一、午后の授業\""));
}

#[test]
fn page_info_command_reports_page_metrics_and_mark_context() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-info")
        .arg(&path)
        .arg("0")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_json_brackets_balanced(&stdout);
    assert!(stdout.contains("\"pageIndex\":0"));
    assert!(stdout.contains("\"pageNumber\":1"));
    assert!(stdout.contains("\"columns\":[{\"x\":72.0,\"width\":650.0}]"));
    assert!(stdout.contains("\"layoutMarkEvidence\":null"));
}

#[test]
fn page_svg_command_writes_rendered_svg_page() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-svg")
        .arg(&path)
        .arg("0")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with("<svg "));
    assert!(stdout.contains("class=\"rjtd-text\""));
    assert!(stdout.contains(">銀河</text>"));
}

#[test]
fn style_candidates_command_reports_labeled_text_layout_records() {
    let path = style_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("style-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "style_candidates\t1\n",
            "candidate\t1\t/TextLayoutStyle\t0\toffset=276\tcode=0x5555\tpayloadLength=6\tname=",
            "\u{672c}\u{6587}",
            "\n"
        )
    );
}

#[test]
fn text_layout_style_records_command_reports_payload_diagnostics() {
    let path = style_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-layout-style-records")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tstatus=ok\tstream=/TextLayoutStyle\tstream-bytes=286\trecords=1\tlabeled=1\n"
    ));
    assert!(stdout.contains(
        "record\t0\tcandidate=1\toffset=276\tcode=0x5555\tpayloadLength=6\tpayloadDigest=0x"
    ));
    assert!(
        stdout.contains("\tpayloadPrefix=0002672c6587\tpayloadBe16=0x0002,0x672c,0x6587\tlabel=")
    );
    assert!(stdout.contains("\u{672c}\u{6587}\n"));
}

#[test]
fn text_layout_style_records_command_reports_missing_stream() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-layout-style-records")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "summary\tstatus=missing\tstream=/TextLayoutStyle\tstream-bytes=0\trecords=0\tlabeled=0\n"
    );
}

#[test]
fn document_view_style_groups_command_reports_payload_diagnostics() {
    let path = text_position_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("document-view-style-groups")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "summary\tstatus=ok\tstream-bytes=20\trecords=4\tgroups=1\tgroup-records=4\n"
        )
    );
    assert!(stdout.contains(
        "group\t1\trecords=4\tcodes=0x3104,0x3105,0x3106,0x3107\tpayloadLengths=1,1,1,1\tpayloadDigest=0x"
    ));
    assert!(
        stdout.contains("record\t1\t0\toffset=0\tcode=0x3104\tpayloadLength=1\tpayloadDigest=0x")
    );
    assert!(stdout.contains("\tpayloadPrefix=04\tpayloadBe16=-\n"));
    assert!(
        stdout.contains("record\t1\t3\toffset=15\tcode=0x3107\tpayloadLength=1\tpayloadDigest=0x")
    );
    assert!(stdout.contains("\tpayloadPrefix=07\tpayloadBe16=-\n"));
}

#[test]
fn document_view_style_groups_command_ignores_ungrouped_records() {
    let path = document_view_style_ungrouped_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("document-view-style-groups")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "summary\tstatus=ok\tstream-bytes=20\trecords=4\tgroups=0\tgroup-records=0\n"
    );
}

#[test]
fn dump_stream_command_accepts_escaped_control_path() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("dump-stream")
        .arg(&path)
        .arg("/\\x04JSRV_SegmentInformation")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"segment");
}

#[test]
fn cfb_map_command_reports_special_chains() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cfb-map")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("sector_size\t"));
    assert!(stdout.contains("mini_stream_cutoff\t"));
    assert!(stdout.contains("fat_sectors\t"));
    assert!(stdout.contains("directory_chain\tcomplete\t"));
    assert!(stdout.contains("root_mini_stream\t"));
    assert!(stdout.contains("mini_stream_chain\tcomplete\t"));
}

#[test]
fn cfb_dir_command_reports_raw_directory_entries() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cfb-dir")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\troot\t"));
    assert!(stdout.contains("\tstream\t3\t"));
    assert!(stdout.contains("\t/DocInfo\tDocInfo\t7\n"));
}

#[test]
fn stream_meta_command_reports_storage_location() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-meta")
        .arg(&path)
        .arg("/DocInfo")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("path\t/DocInfo\n"));
    assert!(stdout.contains("size\t3\n"));
    assert!(stdout.contains("storage\tmini\n"));
    assert!(stdout.contains("mini_stream_cutoff\t"));
    assert!(stdout.contains("mini_stream_bytes\t"));
}

#[test]
fn stream_chain_command_reports_sector_chain() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-chain")
        .arg(&path)
        .arg("/DocInfo")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("path\t/DocInfo\n"));
    assert!(stdout.contains("storage\tmini\n"));
    assert!(stdout.contains("declared_size\t3\n"));
    assert!(stdout.contains("sector_size\t64\n"));
    assert!(stdout.contains("offset_basis\tmini-stream\n"));
    assert!(stdout.contains("status\tcomplete\n"));
    assert!(stdout.contains("sector\t0\t"));
}

#[test]
fn stream_words_command_reports_big_endian_words() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-words")
        .arg(&path)
        .arg("/DocInfo")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "0\t0\t0x646f\n");
}

#[test]
fn line_mark_tags_command_reports_tag_contexts() {
    let path = line_mark_tags_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("line-mark-tags")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "tag\t3\t6\t0x1002\tprev=0x0914,0x0000,0x0001\tnext=0x0077,0x0002,0x1000,0x0074,0x1001,0x000d\n"
    ));
    assert!(stdout.contains(
        "tag\t6\t12\t0x1000\tprev=0x0001,0x1002,0x0077,0x0002\tnext=0x0074,0x1001,0x000d\n"
    ));
    assert!(stdout.contains("tag\t8\t16\t0x1001\tprev=0x0077,0x0002,0x1000,0x0074\tnext=0x000d\n"));
}

#[test]
fn line_mark_intervals_command_reports_delta_records() {
    let path = line_mark_intervals_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("line-mark-intervals")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tlen=30\twords=15\tprofile=be16-delta-v1\tdeclared-count=3\tmax-records=3\tparsed-records=2\tbase-unit=16"
    ));
    assert!(stdout.contains(
        "line-mark-interval\trecord=0\tbyte=18\tword=9\tdelta=5\tflag=0x0002\tunit-start=16\tunit-end=21"
    ));
    assert!(stdout.contains(
        "line-mark-interval\trecord=1\tbyte=22\tword=11\tdelta=8\tflag=0x8002\tunit-start=21\tunit-end=29"
    ));
    assert!(stdout.contains(
        "line-mark-interval-stop\trecord=2\tbyte=26\tdelta=0\tflag=0x0000\treason=non-positive-delta"
    ));
}

#[test]
fn line_mark_text_context_command_compares_tag_words_to_document_text() {
    let path = line_mark_text_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("line-mark-text-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "tag\t3\t6\t0x1002\tline-byte=hit:text(-)@2-10/1-5:ABtC\tline-unit=hit:text(-)@2-10/1-5:ABtC\tnext0=0x0041\tdoc-word-hits=1\tfirst-doc-unit=1\tfirst-doc-context=hit:text(-)@2-10/1-5:ABtC"
    ));
    assert!(stdout.contains(
        "tag\t6\t12\t0x1000\tline-byte=between:text(-)@2-10/1-5:ABtC|-\tline-unit=between:text(-)@2-10/1-5:ABtC|-\tnext0=0x0074\tdoc-word-hits=1\tfirst-doc-unit=3\tfirst-doc-context=hit:text(-)@2-10/1-5:ABtC"
    ));
}

#[test]
fn text_position_line_context_command_compares_mark_offsets_to_line_mark() {
    let path = text_position_line_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-line-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tline-words=9\tline-tags=3\tmark-entries=3\tpage-entries=missing\tpaper-entries=missing\n"
    ));
    assert!(stdout.contains(
        "header\t30\t000000000002\tline-index=2\tword=0x1002\tprev-tag=-\tnext-tag=0x1000@4,d=2\tcontext=prev=0x0914,0x0000|next=0x0041,0x1000,0x0074,0x000d,0x1001,0x000a\n"
    ));
    assert!(stdout.contains(
        "entry\t1\t4\tline-index=4\tword=0x1000\tprev-tag=0x1002@2,d=-2\tnext-tag=0x1001@7,d=3\tcontext=prev=0x0914,0x0000,0x1002,0x0041|next=0x0074,0x000d,0x1001,0x000a\n"
    ));
    assert!(stdout.contains(
        "entry\t3\t20\tline-index=20\tword=out-of-range\tprev-tag=0x1001@7,d=-13\tnext-tag=-\tcontext=prev=0x0074,0x000d,0x1001,0x000a|next=-\n"
    ));
}

#[test]
fn stream_word_frequencies_command_reports_big_endian_counts() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-word-frequencies")
        .arg(&path)
        .arg("/BodyText/Section0")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "1\t0x6865\n1\t0x6c6c\n"
    );
}

#[test]
fn stream_dwords_command_reports_big_endian_dwords() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-dwords")
        .arg(&path)
        .arg("/BodyText/Section0")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "0\t0\t0x68656c6c\n"
    );
}

#[test]
fn stream_dword_frequencies_command_reports_big_endian_counts() {
    let path = raw_stream_path(b"AAAABBBBAAAAzz");
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-dword-frequencies")
        .arg(&path)
        .arg("/Raw")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "2\t0x41414141\n1\t0x42424242\n"
    );
}

#[test]
fn stream_text_probe_reports_ascii_and_utf16_candidates() {
    let path = text_probe_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-text-probe")
        .arg(&path)
        .arg("/Raw")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("ascii\t1\tVer.2.3\n"));
    assert!(stdout.contains("utf16le\t"));
    assert!(stdout.contains("Layout"));
    assert!(stdout.contains("utf16be\t"));
    assert!(stdout.contains("Wide"));
}

#[test]
fn stream_find_command_reports_exact_stream_matches() {
    let path = duplicate_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-find")
        .arg(&path)
        .arg("/Needle")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("needle\t/Needle\t6\n"));
    assert!(stdout.contains("match\t/Haystack\t2\t6\n"));
    assert!(stdout.contains("match\t/Haystack\t8\t6\n"));
    assert!(stdout.contains("match\t/Needle\t0\t6\n"));
}

#[test]
fn stream_find_bytes_command_reports_hex_matches() {
    let path = duplicate_stream_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("stream-find-bytes")
        .arg(&path)
        .arg("0x6e 65_65 64")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("needle\t6e656564\t4\n"));
    assert!(stdout.contains("match\t/Haystack\t2\t4\n"));
    assert!(stdout.contains("match\t/Haystack\t8\t4\n"));
    assert!(stdout.contains("match\t/Needle\t0\t4\n"));
}

#[test]
fn so_records_command_reports_marker_fields() {
    let path = so_record_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-records")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("record\t/Object\t2\t"));
    assert!(stdout.contains("0x00004f53,0x00000007,0x00000100"));
    assert!(stdout.contains("534f00000700000000010000"));
}

#[test]
fn object_stream_candidates_command_reports_visual_object_inventory() {
    let path = object_stream_candidates_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-stream-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let summary = stdout.lines().next().unwrap_or_default();
    assert!(summary.starts_with("summary\t"));
    for field in [
        "streams=9",
        "candidates=8",
        "unreadable=0",
        "object-path=4",
        "image-path=1",
        "shape-path=2",
        "table-path=1",
        "visual-list-path=1",
        "visual-list-raster=0",
        "embedded-press-snapshot=1",
        "jseq3-formula=1",
        "so-marker=3",
        "image-signature=1",
        "svg-signature=1",
        "decoded=false",
    ] {
        assert!(
            summary.contains(field),
            "summary did not contain {field}: {summary}"
        );
    }
    assert!(stdout.contains(
        "stream=/EmbedItems/Embedding 1/JSFart2Contents\tsize=25\treasons=object-path,so-marker\timage-signatures=-\tsvg-offsets=-\tso-offsets=13\t"
    ));
    assert!(stdout.contains(
        "stream=/EmbedItems/Embedding 1/\\x03EmbeddedPress\tsize=128\treasons=object-path,embedded-press-snapshot\timage-signatures=-\tsvg-offsets=-\tso-offsets=-\tvisual-list=-\tembedded-press-snapshot=JSSnapShot32,2590x460,body=3656,objects=17\t"
    ));
    assert!(stdout.contains("stream=/EmbedItems/Embedding 1/JSEQ3Contents"));
    assert!(stdout.contains("jseq3-formula"));
    assert!(stdout.contains("jseq3-formula=MATH.VAF,so=116,fields=0x00004f53,0x200e0a20"));
    assert!(stdout.contains("markers=Times New Roman@16,JustUnitMark@46,JustOubunMark@70"));
    assert!(stdout.contains(
        "stream=/EmbedItems/Embedding 2/Image.png\tsize=44\treasons=object-path,image-path,image-signature\timage-signatures=jpeg@0\t"
    ));
    assert!(stdout.contains(
        "stream=/Figure\tsize=12\treasons=shape-path,so-marker\timage-signatures=-\tsvg-offsets=-\tso-offsets=0\t"
    ));
    assert!(stdout.contains(
        "stream=/Tables/Table1\tsize=13\treasons=table-path\timage-signatures=-\tsvg-offsets=-\tso-offsets=-\t"
    ));
    assert!(stdout.contains(
        "stream=/Vector.svg\tsize=52\treasons=shape-path,svg-signature\timage-signatures=-\tsvg-offsets=21\tso-offsets=-\t"
    ));
    assert!(stdout.contains(
        "stream=/VisualList\tsize=23\treasons=visual-list-path\timage-signatures=-\tsvg-offsets=-\tso-offsets=-\tvisual-list=-\t"
    ));
}

#[test]
fn object_ownership_references_command_reports_reference_contexts() {
    let path = object_stream_candidates_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-ownership-references")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "object-ownership-reference\tsource=/EmbedItems/Embedding 2/Image.png\ttarget=/Figure\tencoding=u16-le\toffset=6\ttotal=1\tmod2=0\tmod4=2\t"
    ));
    assert!(stdout.contains("window-start=0\twindow-hex=534f0000ff090200a0080002\t"));
    assert!(stdout.contains("\tle16=2\tbe16=512\tle32=144703490\tbe32=33595400\t"));
    assert!(stdout.contains(
        "summary\tsources=1\treferences=2\treported-offsets=2\ttarget-missing=0\tdecoded=false\n"
    ));
}

#[test]
fn object_ownership_reference_fields_command_groups_stride_candidates() {
    let path = object_stream_candidates_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-ownership-reference-fields")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("summary\tsources=1\treferences=2\treported-offsets=2\tfield-groups=40\t")
    );
    assert!(stdout.contains(
        "strides=4,8,12,16,20,24,28,32,36,40,44,48,52,56,60,64,68,72,80,84\tdecoded=false"
    ));
    assert!(stdout.contains(
        "object-ownership-reference-field\ttarget=/Figure\tencoding=u16-le\tstride=12\tfield-offset=6\tmatches=1\tsource-count=1\tembedding-indexes=2\trow-indexes=0\tcross-row=0\tdecoded=false"
    ));
    assert!(stdout.contains(
        "object-ownership-reference-field\ttarget=/Figure\tencoding=u16-be\tstride=12\tfield-offset=10\tmatches=1\tsource-count=1\tembedding-indexes=2\trow-indexes=0\tcross-row=0\tdecoded=false"
    ));
}

#[test]
fn object_frame_reference_records_command_expands_candidate_rows() {
    let path = object_frame_reference_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-frame-reference-records")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "object-frame-reference-record\tsource=/EmbedItems/Embedding 2/Image.jpg\tembedding=2\ttarget=/Frame\tencoding=u16-le\tstride=12\tfield-offset=5\toffset=5\trow-index=0\trow-start=0\tcandidate=u16-le/12/5\t"
    ));
    assert!(stdout.contains("row-hex=000100000002000000010001\trow-be16=0x0001,0x0000,0x0002,0x0000,0x0001,0x0001\trow-le16=256,0,512,0,256,256\t"));
    assert!(stdout.contains(
        "row-be32=0x00010000,0x00020000,0x00010001\trow-le32=0x00000100,0x00000200,0x01000100\tdecoded=false"
    ));
    assert!(
        stdout.contains(
            "summary\tsources=1\tframe-references=4\trecords=1\tskipped=0\tcandidates=u16-le/12/5,u16-be/12/7,u16-be/20/15\tdecoded=false\n"
        ),
        "stdout: {stdout}"
    );
}

#[test]
fn object_frame_record_families_command_groups_candidate_rows() {
    let path = object_frame_reference_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-frame-record-families")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "object-frame-record-family\tfamily=frame-index-flag-row12\trows=1\tcandidates=u16-le/12/5\tembeddings=2\texamples=000100000002000000010001\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tfamilies=1\trecords=1\tskipped=0\tcandidates=u16-le/12/5,u16-be/12/7,u16-be/20/15\tdecoded=false\n"
    ));
}

#[test]
fn object_frame_row_links_command_connects_window_suffix_rows() {
    let path = object_frame_row_link_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-frame-row-links")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "object-frame-row-link\tsource=/EmbedItems/Embedding 2/Image.jpg\tembedding=2\trow20-family=frame-index-tail-window20\trow20-start=0\trow20-index=0\tprefix-hex=0000000000200000\tsuffix-hex=000000000102000002000000\trelation=same-source\tsuffix-family=frame-index-tail-coordinate-row12\tmatched-source=/EmbedItems/Embedding 2/Image.jpg\tmatched-row-start=24\tmatched-row-index=2\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\trow20=1\tlinked=1\tunlinked=0\trelations=same-source:1\tfamily-pairs=frame-index-tail-window20->frame-index-tail-coordinate-row12:1\tdecoded=false"
    ));
}

#[test]
fn object_image_frame_candidates_command_prioritizes_row12_tail_coordinates() {
    let path = object_frame_row_link_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-image-frame-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "object-image-frame-candidate\tsource=/EmbedItems/Embedding 2/Image.jpg\tembedding=2\tpayloads=1\tpayload-kinds=jpeg\tpayload-dimensions=jpeg@0:32x16\tdimensioned-payloads=1\tframe-rows=3\t"
        ),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(
        "object-image-frame-candidate\tsource=/EmbedItems/Embedding 2/Image.jpg\tembedding=2\tpayloads=1\tpayload-kinds=jpeg\tpayload-dimensions=jpeg@0:32x16\tdimensioned-payloads=1\tframe-rows=3\t"
    ));
    assert!(stdout.contains(
        "row-families=frame-index-mixed-row12:1,frame-index-tail-coordinate-row12:1,frame-index-tail-window20:1\trow12-tail-coordinate=1\trow12-tail-zero=0\trow20-tail-window=1\trow20-linked=1\tle-row12=1\tpreferred=row12-tail-coordinate\tcoordinate-pairs=24:258x512\tbest-coordinate-aspect-delta-permille=748\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tsources=1\tframe-linked=1\tmissing-frame=0\tframe-rows=3\tdimensioned-payloads=1\taspect-candidates=1\tpreferred=row12-tail-coordinate:1\tdecoded=false"
    ));
}

#[test]
fn object_fdm_index_command_links_index_rows_to_vector_image_hits() {
    let path = object_fdm_index_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-index")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "object-fdm-index-summary\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\tindex-bytes=64\tvector-bytes=103\tdeclared-count=2\tparsed-entries=2\ttrailing-bytes=0\tentries-with-image=1\timage-hits=1\tvector-missing=false\tdecoded=false"
        ),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(
        "object-fdm-index-entry\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=0\tindex-offset=20\tvector-offset=0\tnext-vector-offset=32\tvector-length=32\tkind=0x0b00\tbbox=1,2,3,4\tvalid-vector-offset=true\t"
    ));
    assert!(stdout.contains(
        "object-fdm-index-entry\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=1\tindex-offset=42\tvector-offset=32\tnext-vector-offset=103\tvector-length=71\tkind=0x0b00\tbbox=-1,-2,10,20\tvalid-vector-offset=true\t"
    ));
    assert!(
        stdout.contains("image-signatures=png@36\tsegment-image-signatures=png@4\tdecoded=false")
    );
    assert!(stdout.contains(
        "summary\tindexes=1\tentries=2\tentries-with-image=1\timage-hits=1\tmissing-vectors=0\tdecoded=false"
    ));
}

#[test]
fn object_fdm_image_candidates_command_reports_unplaced_image_segments() {
    let path = object_fdm_index_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-image-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "object-fdm-image-candidate\tsource=/FigureData/main_data/FDMVector\tindex=/FigureData/main_data/FDMIndex\trow=1\tvector-offset=32\tnext-vector-offset=103\tvector-length=71\tkind=0x0b00\tbbox=-1,-2,10,20\tnormalized-bbox=-1,-2,10,20\tbbox-size=11x22\tbbox-order=forward\tbbox-plausible=true\timage-hits=1\tcomplete-payloads=1"
    ));
    assert!(stdout.contains(
        "image-signatures=png@36\tsegment-image-signatures=png@4\trenderable=false\treason=page-placement-unproven\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tsources=1\tcandidates=1\timage-hits=1\tcomplete-payloads=1\tbbox-plausible=1\trenderable=0\tdecoded=false"
    ));
}

#[test]
fn object_fdm_frame_links_command_connects_fdm_rows_to_frame_records() {
    let path = object_fdm_frame_link_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-frame-links")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "object-fdm-frame-link\tsource=/FigureData/main_data/FDMVector\tindex=/FigureData/main_data/FDMIndex\trow=1\timage-hits=1\tcomplete-payloads=1\tframe-linked=true\tframe-source=/Frame\tframe-row=1\tframe-start=76\tframe-object-id=1\tframe-kind=0x0102\tframe-type=0x0007\tframe-geometry=100,200,300,400\tframe-size=300x400\tpayload-dimensions=png@36:1x1\tdimensioned-payloads=1\tbest-aspect-delta-permille=250\tlink-basis=fdm-row-index-to-frame-object-id\trenderable=false\treason=page-placement-unproven\tdecoded=false"
        ),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(
        "summary\tsources=1\tcandidates=1\tframe-linked=1\tmissing-frame=0\tcomplete-payloads=1\trenderable=0\tdecoded=false"
    ));
}

#[test]
fn object_fdm_index_shape_command_classifies_declared_prefix_rows() {
    let path = object_fdm_index_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-index-shape")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "object-fdm-index-shape\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\tindex-bytes=64\tvector-bytes=45\theader-family=fdm-index-v1"
    ));
    assert!(stdout.contains(
        "declared-count=1\tdeclared-plausible=true\trow22-stream-rows=2\trow22-trailing-bytes=0\tdeclared-row22=1\tpost-declared-bytes=22"
    ));
    assert!(stdout.contains(
        "all-valid=1\tall-invalid=1\tall-image-rows=1\tall-image-hits=1\tdeclared-valid=1\tdeclared-invalid=0\tdeclared-image-rows=1\tdeclared-image-hits=1"
    ));
    assert!(stdout.contains(
        "first-invalid-row=1\tfirst-invalid-offset=4294967280\tshape=row22-count-prefix\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tindexes=1\theader-v1=1\tunknown-header=0\tdeclared-plausible=1\tstream-rows=2\tstream-invalid=1\tdeclared-rows=1\tdeclared-invalid=0\tdeclared-image-hits=1\tshapes=row22-count-prefix:1\tdecoded=false"
    ));
}

#[test]
fn object_fdm_index_rows_command_classifies_coordinate_like_invalid_rows() {
    let path = object_fdm_index_mixed_rows_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("object-fdm-index-rows")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "object-fdm-index-row\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=0\tscope=declared\trole=vector-segment\tindex-offset=20\tvector-offset=32"
    ));
    assert!(stdout.contains(
        "object-fdm-index-row\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=1\tscope=declared\trole=coordinate-like-invalid\tindex-offset=42\tvector-offset=100728831"
    ));
    assert!(stdout.contains(
        "be16=0x0600,0xffff,0xd3c0,0xffff,0xd5bc,0xffff,0xc028,0xffff,0xc221,0x0000,0x0040\ti16=1536,-1,-11328,-1,-10820,-1,-16344,-1,-15839,0,64"
    ));
    assert!(stdout.contains(
        "object-fdm-index-row\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\trow=2\tscope=post-declared\trole=coordinate-like-invalid"
    ));
    assert!(stdout.contains(
        "object-fdm-index-rows-summary\tindex=/FigureData/main_data/FDMIndex\tvector=/FigureData/main_data/FDMVector\tindex-bytes=86\tvector-bytes=45\theader-family=fdm-index-v1\tdeclared-count=2\trows=3\tdeclared-rows=2\tpost-declared-rows=1\traw-rows=0\tvalid-rows=1\tinvalid-rows=2\timage-hits=1\troles=coordinate-like-invalid:2,vector-segment:1\tvector-missing=false\tdecoded=false"
    ));
    assert!(stdout.contains(
        "summary\tindexes=1\trows=3\tdeclared-rows=2\tpost-declared-rows=1\traw-rows=0\tvalid-rows=1\tinvalid-rows=2\timage-hits=1\tmissing-vectors=0\troles=coordinate-like-invalid:2,vector-segment:1\tdecoded=false"
    ));
}

#[test]
fn so_record_clusters_command_groups_raw_records() {
    let path = so_record_cluster_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-record-clusters")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("cluster\t2\t"));
    assert!(stdout.contains("0x00004f53,0x00000007,0x00000100,0x00000000,0x00000064"));
    assert!(stdout.contains("/First@0,/Second@2"));
}

#[test]
fn so_record_fields_command_reports_le_breakdown() {
    let path = so_record_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-record-fields")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout
            .contains("field\t/Object\t2\t0\t0x00004f53\t20307\t20307\t0x4f53\t20307\t0x0000\t0\n")
    );
    assert!(stdout.contains("field\t/Object\t2\t1\t0x00000007\t7\t7\t0x0007\t7\t0x0000\t0\n"));
    assert!(
        stdout.contains("field\t/Object\t2\t2\t0x00000100\t256\t256\t0x0100\t256\t0x0000\t0\n")
    );
}

#[test]
fn so_record_geometry_command_reports_coordinate_candidates() {
    let path = so_record_geometry_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-record-geometry")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "candidate\t/Geometry\t0\tgeometry-like\t2559\t2208\t5018\t2208\t2459\t0\t7577\t4416\t"
    ));
    assert!(
        stdout.contains("534f0000ff090000a00800009a130000a008000000000000000000000000000000000000")
    );
}

#[test]
fn so_record_halves_command_reports_packed_pairs() {
    let path = so_record_packed_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("so-record-halves")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "halves\t/Packed\t0\tpacked-jseq3-like\tlo_u16=2592,36122,30922,0,0,36122,7290,0\t"
    ));
    assert!(stdout.contains("hi_u16=8206,6126,20346,0,0,0,0,0\t"));
    assert!(stdout.contains("lo_i16=2592,-29414,30922,0,0,-29414,7290,0\t"));
    assert!(stdout.contains("hi_i16=8206,6126,20346,0,0,0,0,0\t"));
    assert!(
        stdout.contains("534f0000200a0e201a8dee17ca787a4f00000000000000001a8d00007a1c000000000000")
    );
}

#[test]
fn cat_command_extracts_document_text_runs() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cat")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "銀河鉄道\n");
}

#[test]
fn text_tokens_command_reports_structured_document_text() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-tokens")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "text\t銀河\ncontrol\t0x001c\ntext\t鉄道\\n\n"
    );
}

#[test]
fn text_tokens_command_preserves_skipped_inline_text() {
    let path = skipped_inline_document_text_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-tokens")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("text\t本文\n"));
    assert!(stdout.contains("skipped-inline\t0x0082\t24\tふりがな\n"));
}

#[test]
fn text_control_context_command_reports_neighboring_controls() {
    let path = control_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-context")
        .arg(&path)
        .output()
        .unwrap();
    let filtered = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-context")
        .arg(&path)
        .arg("0x000e")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "control-context\t1\t0x001c\tbyte=12-14\tunit=6-7\tprev=text(-)@10-12/5-6:A\tnext=text(-)@16-18/8-9:B\tprev-control=-\tnext-control=0x000e@3,d=2,byte=18,unit=9\n"
    ));
    assert!(stdout.contains(
        "control-context\t3\t0x000e\tbyte=18-20\tunit=9-10\tprev=text(-)@16-18/8-9:B\tnext=text(-)@22-24/11-12:C\tprev-control=0x001c@1,d=-2,byte=12,unit=6\tnext-control=-\n"
    ));

    assert!(
        filtered.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&filtered.stderr)
    );
    assert_eq!(
        String::from_utf8(filtered.stdout).unwrap(),
        "control-context\t3\t0x000e\tbyte=18-20\tunit=9-10\tprev=text(-)@16-18/8-9:B\tnext=text(-)@22-24/11-12:C\tprev-control=0x001c@1,d=-2,byte=12,unit=6\tnext-control=-\n"
    );
}

#[test]
fn text_control_clusters_command_groups_adjacent_controls() {
    let path = control_cluster_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-clusters")
        .arg(&path)
        .output()
        .unwrap();
    let filtered = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-clusters")
        .arg(&path)
        .arg("0x000e")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "control-cluster\t1-2\tlen=2\tcodes=0x000e,0x001d\tbyte=12-16\tunit=6-8\tprev=text(-)@10-12/5-6:A\tnext=text(-)@18-20/9-10:B\n",
            "control-cluster\t4-4\tlen=1\tcodes=0x001c\tbyte=20-22\tunit=10-11\tprev=text(-)@18-20/9-10:B\tnext=text(-)@24-26/12-13:C\n",
        )
    );

    assert!(
        filtered.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&filtered.stderr)
    );
    assert_eq!(
        String::from_utf8(filtered.stdout).unwrap(),
        "control-cluster\t1-2\tlen=2\tcodes=0x000e,0x001d\tbyte=12-16\tunit=6-8\tprev=text(-)@10-12/5-6:A\tnext=text(-)@18-20/9-10:B\n"
    );
}

#[test]
fn text_control_ranges_command_summarizes_delimited_intervals() {
    let path = control_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-ranges")
        .arg(&path)
        .output()
        .unwrap();
    let filtered = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-control-ranges")
        .arg(&path)
        .arg("0x001c")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "control-range\t0\tdelimiter=all\tprev=start\tnext=0x001c@1,byte=12,unit=6\tentries=0-0\tbyte=10-12\tunit=5-6\tentries=1,text=1,inline=0,skipped=0,control=0,controls=-,preview=A\n",
            "control-range\t1\tdelimiter=all\tprev=0x001c@1,byte=12,unit=6\tnext=0x000e@3,byte=18,unit=9\tentries=2-2\tbyte=14-18\tunit=7-9\tentries=1,text=1,inline=0,skipped=0,control=0,controls=-,preview=B\n",
            "control-range\t2\tdelimiter=all\tprev=0x000e@3,byte=18,unit=9\tnext=end\tentries=4-4\tbyte=20-24\tunit=10-12\tentries=1,text=1,inline=0,skipped=0,control=0,controls=-,preview=C\n",
        )
    );

    assert!(
        filtered.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&filtered.stderr)
    );
    assert_eq!(
        String::from_utf8(filtered.stdout).unwrap(),
        concat!(
            "control-range\t0\tdelimiter=0x001c\tprev=start\tnext=0x001c@1,byte=12,unit=6\tentries=0-0\tbyte=10-12\tunit=5-6\tentries=1,text=1,inline=0,skipped=0,control=0,controls=-,preview=A\n",
            "control-range\t1\tdelimiter=0x001c\tprev=0x001c@1,byte=12,unit=6\tnext=end\tentries=2-4\tbyte=14-24\tunit=7-12\tentries=3,text=2,inline=0,skipped=0,control=1,controls=0x000e:1,preview=BC\n",
        )
    );
}

#[test]
fn text_positions_command_reports_mark_offsets() {
    let path = position_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-positions")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "1\t4660\n2\t22136\n"
    );
}

#[test]
fn text_position_mark_header_command_reports_raw_header_and_entries() {
    let path = position_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-mark-header")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("header\t30\t000000000002\tbe16=0,0,2\tle16=0,0,512\tbe32@0=0\tbe32@2=2\n")
    );
    assert!(stdout.contains("entry\t30\t0\t44\t1\t4660\t000100001234\n"));
    assert!(stdout.contains("entry\t30\t1\t50\t2\t22136\t000200005678\n"));
}

#[test]
fn text_position_mark_summary_command_reports_related_stream_metrics() {
    let path = mark_summary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-mark-summary")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with("summary\t30\t000000000002\t2\t2\t22136\t24\t12\t"));
    assert!(stdout.contains("len=8,words=0x0914,0x0000,0x0001,0x0000\t"));
    assert!(stdout.contains("count=2,stride=16,last=1,entries=3,family=fixed84\t"));
    assert!(stdout.contains("count=2,stride=12,last=1,entries=3\t264\t36\n"));
}

#[test]
fn text_positions_command_rejects_count_only_table() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-positions")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("missing MarkV.01 table"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn text_position_counts_command_reports_tcnt_entries() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-counts")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("header\t1\t0\t2\t36\t2\n"));
    assert!(stdout.contains(
        "entry\t0\t4660\t4688\t0000123400001250010100050000000000000000000000010000000000\n"
    ));
    assert!(stdout.contains(
        "entry\t1\t8192\t9216\t0000200000002400010100060000000000000000000000010000000000\n"
    ));
}

#[test]
fn text_position_count_context_command_compares_tcnt_fields() {
    let path = text_count_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("0\t10\t13\thit:text(-)@10-14/5-7:銀河\thit:text(-)@10-14/5-7:銀河\t"));
    assert!(stdout.contains(
        "1\t5\t6\tbetween:-|text(-)@10-14/5-7:銀河\tbetween:-|text(-)@10-14/5-7:銀河\thit:text(-)@10-14/5-7:銀河\thit:text(-)@10-14/5-7:銀河"
    ));
}

#[test]
fn text_position_count_tail_context_command_compares_tail_fields() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("tail-context\t0\tbe0\t100\t112\tt1=5\tt2=6\ttspan=1\t"));
    assert!(
        stdout.contains("t1-unit=hit:text(-)@10-14/5-7:銀河\tt2-unit=hit:text(-)@10-14/5-7:銀河")
    );
    assert!(stdout.contains("tail-context\t1\tbe1-shifted\t38602\t38602\tt1=9\tt2=11\ttspan=2\t"));
    assert!(stdout.contains(
        "t1-unit=hit:text(-)@18-24/9-12:鉄道\\n\tt2-unit=hit:text(-)@18-24/9-12:鉄道\\n"
    ));
}

#[test]
fn text_position_count_clusters_command_groups_duplicate_ranges() {
    let path = text_count_cluster_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-clusters")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("10\t13\t2\t0,1\t2\t"));
    assert!(stdout.contains("010100050000000000000000000000010000000000"));
    assert!(stdout.contains("010100060000000000000000000000010000000000"));
    assert!(stdout.contains("20\t24\t1\t2\t1\t"));
}

#[test]
fn text_position_count_candidates_command_reports_shifted_fields() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("0\t4660\t4688\t1192960\t1200129\t"));
    assert!(stdout.contains("1\t8192\t9216\t2097152\t2359297\t"));
}

#[test]
fn text_position_count_family_command_classifies_be0_entries() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-family")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("family\t0\tbe0\t4660\t4688\t4660\t4688\t1192960\t1200129\tlead=0x00\t")
    );
    assert!(stdout.contains("tail=010100050000000000000000000000010000000000\n"));
}

#[test]
fn text_position_count_family_command_classifies_shifted_entries() {
    let path = shifted_text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-family")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "family\t0\tbe1-shifted\t38602\t38602\t150\t3388997782\t38602\t38602\tlead=0x00\t"
    ));
    assert!(stdout.contains("tail=01010041004f0100000100000000000001000000\n"));
}

#[test]
fn text_position_count_fields_command_expands_tail_words() {
    let path = text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-fields")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "fields\t0\tbe0\t4660\t4688\t28\tlead=0x00\ttail-offset=8\ttail-be16=0x0101,0x0005,0x0000,0x0000,0x0000,0x0000,0x0000,0x0001,0x0000,0x0000\ttail-extra=00"
    ));
}

#[test]
fn text_position_count_fields_command_expands_shifted_tail_words() {
    let path = shifted_text_count_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-fields")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "fields\t0\tbe1-shifted\t38602\t38602\t0\tlead=0x00\ttail-offset=9\ttail-be16=0x0101,0x0041,0x004f,0x0100,0x0001,0x0000,0x0000,0x0000,0x0100,0x0000\ttail-extra=-"
    ));
}

#[test]
fn text_position_count_field_deltas_command_compares_tail_range_to_chosen_range() {
    let path = text_count_delta_table_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-field-deltas")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "delta\t0\tbe0\t100\t112\t12\ttail-offset=8\tt1=10\tt2=22\ttspan=12\tspan-relation=eq\tstart-minus-t1=90\tend-minus-t2=90\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001"
    ));
    assert!(stdout.contains(
        "delta\t1\tbe1-shifted\t38602\t38602\t0\ttail-offset=9\tt1=65\tt2=79\ttspan=14\tspan-relation=gt\tstart-minus-t1=38537\tend-minus-t2=38523\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001"
    ));
}

#[test]
fn text_position_count_tail_delta_scan_command_scores_unit_offsets() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-delta-scan")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("delta\t0\t2\t4\t4\t4\t2\t2\n"));
    assert!(stdout.contains("delta\t29\t2\t4\t0\t0\t0\t0\n"));
    assert!(stdout.contains("delta\t64\t2\t4\t0\t0\t0\t0\n"));
}

#[test]
fn text_position_count_tail_delta_groups_command_summarizes_pattern_scores() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-delta-groups")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "group\tbe0\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\trows=1\tendpoints=2\tbest-unit=0:2:1\tbest-text=0:2:1\td0=2:2:1:1\td29=0:0:0:0\td30=0:0:0:0\n"
    ));
    assert!(stdout.contains(
        "group\tbe1-shifted\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\trows=1\tendpoints=2\tbest-unit=0:2:1\tbest-text=0:2:1\td0=2:2:1:1\td29=0:0:0:0\td30=0:0:0:0\n"
    ));
}

#[test]
fn text_position_count_tail_row_deltas_command_reports_per_row_scores() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-row-deltas")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("summary\tentries=2\tdoc-bytes=24\tdoc-units=12\n"));
    assert!(stdout.contains(
        "row\t0\tbe0\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\tstart=100\tend=112\tspan=12\tt1=5\tt2=6\ttspan=1\tbest-unit=0:2:1\tbest-text=0:2:1\td0=2:2:1:1\td29=0:0:0:0\td30=0:0:0:0\n"
    ));
    assert!(stdout.contains(
        "row\t1\tbe1-shifted\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\tstart=38602\tend=38602\tspan=0\tt1=9\tt2=11\ttspan=2\tbest-unit=0:2:1\tbest-text=0:2:1\td0=2:2:1:1\td29=0:0:0:0\td30=0:0:0:0\n"
    ));
}

#[test]
fn text_position_count_tail_row_context_command_reports_chosen_and_tail_contexts() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-row-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "row-context\t0\tbe0\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\tstart=100\tend=112\tt1=5\tt2=6\tbest-unit=0:2:1\tbest-text=0:2:1"
    ));
    assert!(stdout.contains("start-byte=between:text(-)@18-24/9-12:鉄道\\n|-\t"));
    assert!(stdout.contains(
        "t1-unit-best=hit:text(-)@10-14/5-7:銀河\tt2-unit-best=hit:text(-)@10-14/5-7:銀河"
    ));
    assert!(stdout.contains(
        "row-context\t1\tbe1-shifted\tt0=0x0101\tt3=0x0100\tt4=0x0001\tt7=0x0001\tstart=38602\tend=38602\tt1=9\tt2=11\tbest-unit=0:2:1\tbest-text=0:2:1"
    ));
    assert!(stdout.contains(
        "t1-unit-best=hit:text(-)@18-24/9-12:鉄道\\n\tt2-unit-best=hit:text(-)@18-24/9-12:鉄道\\n"
    ));
}

#[test]
fn text_position_count_tail_field_roles_command_summarizes_field_and_pair_hits() {
    let path = text_count_tail_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-tail-field-roles")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("summary\tposition-status=ok\tentries=2\tdoc-bytes=24\tdoc-units=12\n")
    );
    assert!(stdout.contains(
        "field\tf1\tnonzero=2\tdistinct=2\tvalues=0x0005:1,0x0009:1\tunit-d0=2\ttext-d0=2"
    ));
    assert!(stdout.contains(
        "field\tf2\tnonzero=2\tdistinct=2\tvalues=0x0006:1,0x000b:1\tunit-d0=2\ttext-d0=2"
    ));
    assert!(stdout.contains(
        "pair\tf1-f2\tpairs=2\tendpoints=4\tspan-eq=0\tspan-lt=1\tspan-gt=1\tbest-unit=0:4:2\tbest-text=0:4:2\td0=4:4:2:2"
    ));
}

#[test]
fn text_position_count_range_preview_command_reports_overlapping_text() {
    let path = text_count_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-range-preview")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "range-preview\t0\tbe0\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=10\tend=13\tspan=3\tbyte-range=entries=1,text=1,inline=0,skipped=0,control=0,preview=銀河\tunit-range=entries=1,text=1,inline=0,skipped=0,control=0,preview=鉄道\\n\n"
    ));
    assert!(stdout.contains(
        "range-preview\t1\tbe0\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=5\tend=6\tspan=1\tbyte-range=entries=0,text=0,inline=0,skipped=0,control=0,preview=-\tunit-range=entries=1,text=1,inline=0,skipped=0,control=0,preview=銀河\n"
    ));
}

#[test]
fn text_position_style_context_command_reports_tail_field_style_hits() {
    let path = text_position_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-style-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tposition-status=ok\tentries=1\ttext-style-candidates=2\tpage-style-candidates=1\tview-style-records=4\n"
    ));
    assert!(stdout.contains(
        "entry\t0\tbe0\tstart=10\tend=16\tspan=6\ttail-fields=f0=0x0202,f1=0x0001,f2=0x002f,f3=0x0100,f4=0x0000,f5=0x0000,f6=0x0000,f7=0x0001,f8=0x0000,f9=0x0000"
    ));
    assert!(stdout.contains(
        "text-style-id-hits=f1=0x0001:id1:offset276:見出し,f7=0x0001:id1:offset276:見出し"
    ));
    assert!(stdout.contains(
        "text-style-index-hits=f1=0x0001:idx1:id2:offset532:本文,f7=0x0001:idx1:id2:offset532:本文"
    ));
    assert!(stdout.contains(
        "page-style-id-hits=f1=0x0001:id1:offset276:ページ,f7=0x0001:id1:offset276:ページ"
    ));
    assert!(stdout.contains(
        "view-style-group-hits=f1=0x0001:group1:records4:codes0x3104,0x3105,0x3106,0x3107,f7=0x0001:group1:records4:codes0x3104,0x3105,0x3106,0x3107"
    ));
    assert!(stdout.contains("byte-range=entries=2,text=1,inline=0,skipped=0,control=1"));
}

#[test]
fn text_position_style_summary_command_reports_field_hit_distribution() {
    let path = text_position_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-style-summary")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tposition-status=ok\tentries=1\ttext-style-candidates=2\tpage-style-candidates=1\tview-style-records=4\n"
    ));
    assert!(stdout.contains(
        "field\tf1\tnonzero=1\tdistinct=1\tvalues=0x0001:1\ttext-style-id-hits=id1:1:offset276:見出し\ttext-style-index-hits=idx1:1:id2:offset532:本文\tpage-style-id-hits=id1:1:offset276:ページ\tpage-style-index-hits=-\tview-style-group-hits=group1:1:records4:codes0x3104,0x3105,0x3106,0x3107\n"
    ));
    assert!(stdout.contains(
        "field\tf7\tnonzero=1\tdistinct=1\tvalues=0x0001:1\ttext-style-id-hits=id1:1:offset276:見出し\ttext-style-index-hits=idx1:1:id2:offset532:本文\tpage-style-id-hits=id1:1:offset276:ページ\tpage-style-index-hits=-\tview-style-group-hits=group1:1:records4:codes0x3104,0x3105,0x3106,0x3107\n"
    ));
}

#[test]
fn text_position_count_range_boundaries_command_reports_edges_and_controls() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-range-boundaries")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "range-boundary\t0\tbe0\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=10\tend=16\tspan=6\t"
    ));
    assert!(stdout.contains(
        "byte-boundary=inside=2,full=2,partial=0,start-edge=aligned:text(-)@10-14/5-7:銀河,end-edge=aligned:control(0x001c)@14-16/7-8:,first=text(-)@10-14/5-7:銀河,last=control(0x001c)@14-16/7-8:,prev=-,next=text(-)@18-24/9-12:鉄道\\n,controls=0x001c:1"
    ));
    assert!(stdout.contains(
        "unit-boundary=inside=1,full=0,partial=1,start-edge=inside:text(-)@18-24/9-12:鉄道\\n,end-edge=gap:text(-)@18-24/9-12:鉄道\\n|-,first=text(-)@18-24/9-12:鉄道\\n,last=text(-)@18-24/9-12:鉄道\\n,prev=control(0x001c)@14-16/7-8:,next=-,controls=-"
    ));
    assert!(stdout.contains(
        "range-boundary\t1\tbe0\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=7\tend=8\tspan=1\t"
    ));
    assert!(stdout.contains(
        "unit-boundary=inside=1,full=1,partial=0,start-edge=aligned:control(0x001c)@14-16/7-8:,end-edge=aligned:control(0x001c)@14-16/7-8:,first=control(0x001c)@14-16/7-8:,last=control(0x001c)@14-16/7-8:,prev=text(-)@10-14/5-7:銀河,next=text(-)@18-24/9-12:鉄道\\n,controls=0x001c:1"
    ));
}

#[test]
fn text_position_count_control_ranges_command_compares_tcnt_to_control_intervals() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-control-ranges")
        .arg(&path)
        .arg("0x001c")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "count-control-range\t0\tbe0\tdelimiter=0x001c\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=10\tend=16\tspan=6\tbyte-ranges=count=1,first=0,last=0,byte=10-14,unit=5-7,entry-ranges=0-0,controls=-,preview=銀河\tunit-ranges=count=1,first=1,last=1,byte=16-24,unit=8-12,entry-ranges=2-2,controls=-,preview=鉄道\\n"
    ));
    assert!(stdout.contains(
        "count-control-range\t1\tbe0\tdelimiter=0x001c\tt0=0x0101\tt3=0x0000\tt4=0x0000\tt7=0x0001\tstart=7\tend=8\tspan=1\tbyte-ranges=count=0,first=-,last=-,byte=-,unit=-,entry-ranges=-,controls=-,preview=-\tunit-ranges=count=0,first=-,last=-,byte=-,unit=-,entry-ranges=-,controls=-,preview=-"
    ));
}

#[test]
fn text_boundary_candidates_command_reports_model_candidates() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "text-boundary-candidate\t0\tkind=controlDelimitedTextCountRange\trange=0\tbasis=byte\tdelimiter=0x001c\tintervals=1\tinterval-kind=single\tfirst=0\tlast=0\tsource=10-14\tdecoded=false\n",
            "text-boundary-candidate\t1\tkind=controlDelimitedTextCountRange\trange=0\tbasis=unit\tdelimiter=0x001c\tintervals=1\tinterval-kind=single\tfirst=1\tlast=1\tsource=8-11\tdecoded=false\n",
        )
    );
}

#[test]
fn table_candidates_command_reports_model_interval_evidence() {
    let path = text_count_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "table-candidate\t0\tkind=multiIntervalControlRangeTableCandidate\trange=0\tboundary=0\tbasis=byte\tdelimiter=0x001c\tintervals=2\tfirst=0\tlast=1\tsource=10-22\tsparse=false\tcells=0/0/0\tmax-columns=0\tinterval-details=0:source-interval=0,source=10-14,line-breaks=0,text=銀河|1:source-interval=1,source=16-22,line-breaks=0,text=鉄道\tdecoded=false\n",
            "table-candidate\t1\tkind=multiIntervalControlRangeTableCandidate\trange=0\tboundary=1\tbasis=unit\tdelimiter=0x001c\tintervals=2\tfirst=0\tlast=1\tsource=5-11\tsparse=false\tcells=0/0/0\tmax-columns=0\tinterval-details=0:source-interval=0,source=5-7,line-breaks=0,text=銀河|1:source-interval=1,source=8-11,line-breaks=0,text=鉄道\tdecoded=false\n",
        )
    );
}

#[test]
fn table_candidates_command_reports_sparse_document_text_table_evidence() {
    let path = sparse_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "kind=sparseDocumentTextControlRunTableCandidate\trange=-\tboundary=-\tbasis=unit"
    ));
    assert!(stdout.contains("\tsparse=true\tcells=4/10/14\tmax-columns=4\t"));
    assert!(stdout.contains("text=\\t\\t(1)表面積\\t"));
    assert!(stdout.contains("text=\\tＡＢ ＝ ｃｍ\\t"));
}

#[test]
fn table_candidate_context_command_reports_cell_like_shape() {
    let path = text_count_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-candidate-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "table-candidate-context\t0\trange=0\tboundary=0\tbasis=byte\tdelimiter=0x001c\tintervals=2\tsource=10-22\tshape=non-empty=2,empty=0,min-chars=2,max-chars=2,total-chars=4,line-breaks=0,cell-like=true"
    ));
    assert!(stdout.contains("0:source-interval=0,source=10-14,chars=2,line-breaks=0,text=銀河"));
    assert!(stdout.contains("1:source-interval=1,source=16-22,chars=2,line-breaks=0,text=鉄道"));
    assert!(stdout.contains(
        "table-candidate-context\t1\trange=0\tboundary=1\tbasis=unit\tdelimiter=0x001c\tintervals=2\tsource=5-11\tshape=non-empty=2,empty=0,min-chars=2,max-chars=2,total-chars=4,line-breaks=0,cell-like=true"
    ));
}

#[test]
fn table_cell_like_candidates_command_filters_strict_candidates() {
    let path = text_count_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-cell-like-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        concat!(
            "table-cell-like-candidate\t0\trange=0\tboundary=0\tbasis=byte\tdelimiter=0x001c\tintervals=2\tsource=10-22\tshape=non-empty=2,empty=0,min-chars=2,max-chars=2,total-chars=4,line-breaks=0,cell-like=true\ttexts=0:source-interval=0,source=10-14,chars=2,text=銀河|1:source-interval=1,source=16-22,chars=2,text=鉄道\tcolumn-split-candidate-rows=0\tmax-column-segment-count=0\tcolumn-segment-pattern-consistent=false\tcolumn-segment-pattern-mismatch-rows=0\tcolumn-grid-candidate=false\tcolumn-grid-shape=-\tcolumn-grid-pattern=-\tinterval-column-segments=-\tdecoded=false\n",
            "table-cell-like-candidate\t1\trange=0\tboundary=1\tbasis=unit\tdelimiter=0x001c\tintervals=2\tsource=5-11\tshape=non-empty=2,empty=0,min-chars=2,max-chars=2,total-chars=4,line-breaks=0,cell-like=true\ttexts=0:source-interval=0,source=5-7,chars=2,text=銀河|1:source-interval=1,source=8-11,chars=2,text=鉄道\tcolumn-split-candidate-rows=0\tmax-column-segment-count=0\tcolumn-segment-pattern-consistent=false\tcolumn-segment-pattern-mismatch-rows=0\tcolumn-grid-candidate=false\tcolumn-grid-shape=-\tcolumn-grid-pattern=-\tinterval-column-segments=-\tdecoded=false\n",
        )
    );
}

#[test]
fn table_cell_like_candidates_command_reports_column_segment_diagnostics() {
    let path = text_count_finance_table_candidate_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("table-cell-like-candidates")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("table-cell-like-candidate\t0\t"));
    assert!(stdout.contains("\tshape=non-empty=2,empty=0,"));
    assert!(stdout.contains("\tcolumn-split-candidate-rows=2\t"));
    assert!(stdout.contains("\tmax-column-segment-count=5\t"));
    assert!(stdout.contains("\tcolumn-segment-pattern-consistent=true\t"));
    assert!(stdout.contains("\tcolumn-segment-pattern-mismatch-rows=0\t"));
    assert!(stdout.contains("\tcolumn-grid-candidate=true\t"));
    assert!(stdout.contains("\tcolumn-grid-shape=2x5\t"));
    assert!(stdout.contains("\tcolumn-grid-pattern=label|value|value|value|value\t"));
    assert!(stdout.contains("interval-column-segments=0=0:label:2-5:売掛金"));
    assert!(stdout.contains("1:value:5-14:2,441,997"));
    assert!(stdout.contains("3:value:23-33:△1,541,604"));
    assert!(stdout.contains(";1=0:label:0-6:流動資産合計"));
    assert!(stdout.contains("4:value:28-34:17,327"));
}

#[test]
fn text_boundary_candidate_context_command_reports_text_context() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-candidate-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "text-boundary-candidate-context\t0\trange=0\tbasis=byte\tdelimiter=0x001c\tintervals=1\tinterval-kind=single\tsource=10-14\tline-breaks=0\ttext=entries=1,text=1,inline=0,skipped=0,control=0,preview=銀河"
    ));
    assert!(stdout.contains(
        "edges=inside=1,full=1,partial=0,start-edge=aligned:text(-)@10-14/5-7:銀河,end-edge=aligned:text(-)@10-14/5-7:銀河"
    ));
    assert!(stdout.contains(
        "text-boundary-candidate-context\t1\trange=0\tbasis=unit\tdelimiter=0x001c\tintervals=1\tinterval-kind=single\tsource=8-11\tline-breaks=0\ttext=entries=1,text=1,inline=0,skipped=0,control=0,preview=鉄道\\n"
    ));
    assert!(stdout.contains(
        "edges=inside=1,full=0,partial=1,start-edge=gap:control(0x001c)@14-16/7-8:|text(-)@18-24/9-12:鉄道\\n"
    ));
}

#[test]
fn text_boundary_candidate_agreement_command_compares_byte_and_unit_candidates() {
    let path = text_count_boundary_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-candidate-agreement")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "text-boundary-candidate-agreement\t0\trange=0\tdelimiter=0x001c\tbyte-index=0\tunit-index=1\tbyte-intervals=1\tunit-intervals=1\tbyte-interval-kind=single\tunit-interval-kind=single\tbyte-edge-good=false\tunit-edge-good=false\tbyte-line-breaks=0\tunit-line-breaks=0\ttext-match=false\tline-break-match=true\tbyte-text=銀河\tunit-text=鉄道\tdecoded=false\n"
    );
}

#[test]
fn text_boundary_candidate_layout_context_command_reports_rule_selected_context() {
    let path = text_boundary_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-candidate-layout-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "summary\tunit-001c-single-candidates=1\trule-selected=0\tline-bytes=40\tline-words=20\tpage-rows=20\tpage-bytes=1692\tpaper-rows=20\tpaper-bytes=172"
        ),
        "{stdout}"
    );
    assert!(stdout.contains(
        "candidate\t1\trange=0\tselected=false\tedge-good=false\tnon-empty=true\tline-breaks=0\tsource=8-11\ttext=鉄道"
    ));
    assert!(stdout.contains(
        "line-word-start=hit:8:0x1002\tline-word-end=hit:11:0x000b\tline-byte-start=hit:4:0x0004\tline-byte-end=unaligned:11"
    ));
    assert!(stdout.contains(
        "page-row-start=hit:8\tpage-row-end=hit:11\tpage-byte-start=hit:8\tpage-byte-end=hit:11\tpaper-row-start=hit:8\tpaper-row-end=hit:11"
    ));
}

#[test]
fn text_boundary_layout_map_command_scores_candidate_offset_transforms() {
    let path = text_boundary_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-layout-map")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tunit-001c-single-candidates=1\trule-selected=0\ttarget-sets=8\tbases=4\tdelta-range=-4096..4096"
    ));
    assert!(stdout.contains(
        "best\tscope=all\ttarget=line-tag-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\tpoints=2\tcandidates=1\tendpoints=2\tvalid=2\tinvalid=0\texact=1\ttotal-distance=1\tmax-distance=1\tdecoded=false"
    ));
    assert!(stdout.contains(
        "best\tscope=all\ttarget=page-entry-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\tpoints=20\tcandidates=1\tendpoints=2\tvalid=2\tinvalid=0\texact=2\ttotal-distance=0\tmax-distance=0\tdecoded=false"
    ));
    assert!(stdout.contains(
        "best\tscope=selected\ttarget=line-tag-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\tpoints=2\tcandidates=0\tendpoints=0\tvalid=0\tinvalid=0\texact=0\ttotal-distance=-\tmax-distance=-\tdecoded=false"
    ));
}

#[test]
fn text_boundary_layout_map_rows_command_reports_candidate_local_deltas() {
    let path = text_boundary_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-layout-map-rows")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tunit-001c-single-candidates=1\trule-selected=0\ttarget-sets=8\tbases=4\tlocal-rows=32"
    ));
    assert!(stdout.contains(
        "local\tcandidate=1\trange=0\tselected=false\ttarget=line-tag-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\texact=1\ttotal-distance=1\tmax-distance=1\tstart-nearest=8:8->8:d=0\tend-nearest=11:11->12:d=1\tsource=8-11\ttext=鉄道"
    ));
    assert!(stdout.contains(
        "local\tcandidate=1\trange=0\tselected=false\ttarget=page-entry-index\tbase=unit\tdelta=0\tdelta-at-boundary=false\texact=2\ttotal-distance=0\tmax-distance=0\tstart-nearest=8:8->8:d=0\tend-nearest=11:11->11:d=0"
    ));
    assert!(stdout.contains(
        "tcnt=index=0,family=be0,start=9,end=12,span=3,declared-start=9,declared-end=12,tail=257,5"
    ));
}

#[test]
fn text_boundary_paragraph_like_command_reports_diagnostic_classifier() {
    let path = text_boundary_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-paragraph-like")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tunit-001c-single-candidates=1\tstrict-selected=0\tparagraph-like=0\tselected-non-paragraph-like=0\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false"
    ));
    assert!(stdout.contains("candidate\t1\trange=0\tstrict-selected=false\tparagraph-like=false"));
    assert!(stdout.contains("line-word-evidence="));
    assert!(stdout.contains("page-field-evidence=page-be32-field:unit:0:8:8->8:d=0|11:11->11:d=0"));
    assert!(stdout.contains(
        "tcnt=index=0,family=be0,start=9,end=12,span=3,declared-start=9,declared-end=12,tail=257,5"
    ));
}

#[test]
fn text_boundary_paragraph_like_style_context_command_links_layout_and_style_evidence() {
    let path = text_boundary_paragraph_like_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-paragraph-like-style-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "summary\tunit-001c-single-candidates=1\tstrict-selected=0\tparagraph-like=0\tselected-non-paragraph-like=0\ttext-style-candidates=2\tpage-style-candidates=1\tview-style-records=4\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false"
        ),
        "{stdout}"
    );
    assert!(stdout.contains(
        "candidate\t1\trange=0\tstrict-selected=false\tparagraph-like=false\tline-word-evidence=line-word-value:unit:0:8:8->8:d=0|11:11->11:d=0\tpage-field-evidence=page-be32-field:unit:0:8:8->8:d=0|11:11->11:d=0"
    ));
    assert!(stdout.contains(
        "tail-fields=f0=0x0202,f1=0x0001,f2=0x002f,f3=0x0100,f4=0x0000,f5=0x0000,f6=0x0000,f7=0x0001,f8=0x0000,f9=0x0000"
    ));
    assert!(stdout.contains(
        "text-style-id-hits=f1=0x0001:id1:offset276:見出し,f7=0x0001:id1:offset276:見出し"
    ));
    assert!(stdout.contains(
        "text-style-index-hits=f1=0x0001:idx1:id2:offset532:本文,f7=0x0001:idx1:id2:offset532:本文"
    ));
    assert!(stdout.contains("page-style-id-hits=f1=0x0001:id1:offset276:ページ"));
    assert!(stdout.contains(
        "view-style-group-hits=f1=0x0001:group1:records4:codes0x3104,0x3105,0x3106,0x3107"
    ));
    assert!(stdout.contains(
        "tcnt=index=0,family=be0,start=9,end=13,span=4,declared-start=9,declared-end=13"
    ));
}

#[test]
fn text_boundary_paragraph_like_discriminators_command_summarizes_candidate_buckets() {
    let path = text_boundary_paragraph_like_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-boundary-paragraph-like-discriminators")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tunit-001c-single-candidates=1\tstrict-selected=0\tparagraph-like=0\tselected-non-paragraph-like=0\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false\n"
    ));
    assert!(stdout.contains(
        "bucket\tparagraph-like\trows=0\tstrict-selected=0\tline-word-exact2=0\tpage-field-exact2=0\tdual-exact2=0"
    ));
    assert!(stdout.contains(
        "bucket\tstrict-non-paragraph\trows=0\tstrict-selected=0\tline-word-exact2=0\tpage-field-exact2=0\tdual-exact2=0"
    ));
    assert!(stdout.contains(
        "bucket\tnon-strict\trows=1\tstrict-selected=0\tline-word-exact2=1\tpage-field-exact2=1\tdual-exact2=1\ttext-style-hit=1\tpage-style-hit=1\tview-style-group-hit=1"
    ));
    assert!(stdout.contains(
        "source-spans=3..3\trange-spans=4..4\tfamilies=be0:1\tf0=0x0202:1\tf4=0x0000:1\tf7=0x0001:1"
    ));
    assert!(stdout.contains(
        "line-evidence=line-word-value/unit/0:1\tpage-evidence=page-be32-field/unit/0:1"
    ));
}

#[test]
fn text_paragraph_boundary_targets_command_reports_layout_hit_provenance() {
    let path = text_boundary_paragraph_like_style_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-paragraph-boundary-targets")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout
            .contains("summary\ttext-paragraph-boundary-candidates=1\tline-words=20\tpage-rows=20")
    );
    assert!(
        stdout.contains(
            "text-paragraph-boundary-target\t0\tboundary=1\trange=0\tsource=8-11\tspan=4"
        )
    );
    assert!(stdout.contains(
        "line-word-evidence=line-word-value:unit:0\tline-start=value=8,hits=1,refs=word8:0x0008\tline-end=value=11,hits=1,refs=word11:0x000b"
    ));
    assert!(stdout.contains(
        "page-field-evidence=page-be32-field:unit:0\tpage-start=value=8,hits=1,refs=row8:f0:0x00000008\tpage-end=value=11,hits=1,refs=row11:f0:0x0000000b"
    ));
    assert!(stdout.contains("text=鉄道"));
    assert!(stdout.contains(
        "tcnt=index=0,family=be0,start=9,end=13,span=4,declared-start=9,declared-end=13"
    ));
}

#[test]
fn text_position_count_layout_context_command_compares_tcnt_offsets_to_layout_streams() {
    let path = text_count_layout_context_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-count-layout-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(
        "summary\tentries=2\tline-bytes=12\tline-words=6\tpage-rows=3\tpage-bytes=264\tpaper-rows=3\tpaper-bytes=36\n"
    ));
    assert!(stdout.contains(
        "entry\t0\tbe0\t2\t12\tline-word-start=hit:2:0x1002\tline-word-end=out-of-range:6\tline-byte-start=hit:1:0x0000\tline-byte-end=out-of-range:12\tpage-row-start=hit:2\tpage-row-end=out-of-range:3\tpage-byte-start=hit:2\tpage-byte-end=hit:12\tpaper-row-start=hit:2\tpaper-row-end=out-of-range:3\tpaper-byte-start=hit:2\tpaper-byte-end=hit:12\n"
    ));
}

#[test]
fn paper_marks_command_reports_header_and_entries() {
    let path = paper_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("paper-marks")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "header\t2\t12\t1\t3\nentry\t0\t0x00010010\nentry\t1\t0x00010011\nentry\t2\t0x00010000\n"
    );
}

#[test]
fn paper_mark_shape_command_reports_row_candidates() {
    let path = paper_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("paper-mark-shape")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("stream\t36\t36\tmini\n"));
    assert!(stdout.contains("alignment\tu32\ttrue\n"));
    assert!(stdout.contains("header\t2\t12\t1\n"));
    assert!(stdout.contains("classification\tfixed8\t3\t8\t0\n"));
    assert!(stdout.contains("candidate\tfixed8\t3\t8\t0\n"));
}

#[test]
fn page_marks_command_reports_header_and_raw_entries() {
    let path = page_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-marks")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.starts_with(
            "header\t2\t16\t1\t3\nfamily\tfixed84\t84\t0\nentry\t0\t0\t0000000000010000"
        )
    );
    assert!(stdout.contains("\tu16Class=zero-sentinel\n"));
    assert!(stdout.contains("\nentry\t2\t2\t0000000200010002"));
}

#[test]
fn page_mark_u16_profile_command_reports_class_counts_and_tuples() {
    let path = page_mark_u16_profile_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-mark-u16-profile")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with(
        "summary\tentries=4\tzero-sentinel=1\tadditive-row=1\tadditive-boundary=1\tmixed-payload=1\tdecoded=false\n"
    ));
    assert!(stdout.contains("profile\tzero-sentinel\t1\n"));
    assert!(stdout.contains("profile\tadditive-row\t1\n"));
    assert!(stdout.contains("profile\tadditive-boundary\t1\n"));
    assert!(stdout.contains("profile\tmixed-payload\t1\n"));
    assert!(
        stdout.contains("tuple\tadditive-row\t1\tw10=353/0x0161\tw13=353/0x0161\tw14=246/0x00f6")
    );
    assert!(
        stdout.contains(
            "tuple\tadditive-boundary\t1\tw10=370/0x0172\tw13=370/0x0172\tw14=185/0x00b9"
        )
    );
    assert!(stdout.contains("tuple\tmixed-payload\t1\tw10=0/0x0000\tw13=1/0x0001\tw14=2/0x0002"));
}

#[test]
fn local_pdf_backed_page_mark_u16_profiles_stay_stable_when_available() {
    let sample_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("rjtd-testdata/local-samples");
    if !sample_dir.exists() {
        return;
    }

    let cases = [
        (
            "a5.jtd",
            "summary\tentries=75\tzero-sentinel=2\tadditive-row=68\tadditive-boundary=4\tmixed-payload=1\tdecoded=false",
            "tuple\tadditive-row\t68\tw10=353/0x0161\tw13=353/0x0161\tw14=246/0x00f6",
        ),
        (
            "46.jtd",
            "summary\tentries=97\tzero-sentinel=2\tadditive-row=90\tadditive-boundary=4\tmixed-payload=1\tdecoded=false",
            "tuple\tadditive-row\t90\tw10=353/0x0161\tw13=353/0x0161\tw14=246/0x00f6",
        ),
        (
            "ichitaro-20030228030923-success-002-success_data-test.jtd",
            "summary\tentries=2\tzero-sentinel=0\tadditive-row=0\tadditive-boundary=2\tmixed-payload=0\tdecoded=false",
            "tuple\tadditive-boundary\t2\tw10=370/0x0172\tw13=370/0x0172\tw14=185/0x00b9",
        ),
    ];

    let mut checked = 0usize;
    for (file_name, expected_summary, expected_tuple_prefix) in cases {
        let sample_path = sample_dir.join(file_name);
        if !sample_path.exists() || !sample_path.with_extension("pdf").exists() {
            continue;
        }

        let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
            .arg("page-mark-u16-profile")
            .arg(&sample_path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{} stderr: {}",
            file_name,
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.contains(expected_summary),
            "{} stdout: {}",
            file_name,
            stdout
        );
        assert!(
            stdout.contains(expected_tuple_prefix),
            "{} stdout: {}",
            file_name,
            stdout
        );
        checked += 1;
    }

    if sample_dir.join("a5.jtd").exists() {
        assert!(checked >= 1);
    }
}

#[test]
fn page_marks_command_reports_variable_family_rows() {
    let path = page_mark_variable_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-marks")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with(
        "header\t3\t16\t2\t4\nfamily\tcount-plus-one-variable\t20\t0\nentry\t0\t0\t0000000001000000"
    ));
    assert!(stdout.contains("\nentry\t3\t3\t0000000301000003"));
}

#[test]
fn page_marks_command_reports_count_variable_family_rows() {
    let path = page_mark_count_variable_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-marks")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with(
        "header\t5\t16\t4\t5\nfamily\tcount-variable\t20\t0\nentry\t0\t0\t0000000002000000"
    ));
    assert!(stdout.contains("\nentry\t4\t4\t0000000402000004"));
}

#[test]
fn page_marks_command_reports_fixed84_tail_family_rows() {
    let path = page_mark_fixed84_tail_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-marks")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with(
        "header\t6\t16\t4\t2\nfamily\tfixed84-tail\t84\t4\nentry\t0\t0\t0000000003000000"
    ));
    assert!(stdout.contains("\nentry\t1\t1\t0000000103000001"));
    assert!(stdout.contains("\ntrailing\tdeadbeef\n"));
}

#[test]
fn page_mark_shape_command_reports_row_candidates() {
    let path = page_mark_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-mark-shape")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("stream\t264\t264\tmini\n"));
    assert!(stdout.contains("alignment\tu32\ttrue\n"));
    assert!(stdout.contains("header\t2\t16\t1\n"));
    assert!(stdout.contains("classification\tfixed84-count-plus-one\t3\t84\t0\n"));
    assert!(stdout.contains("candidate\tfixed84\t3\t84\t0\n"));
    assert!(stdout.contains("candidate\tcount-plus-one\t3\t84\t0\n"));
}

#[test]
fn page_mark_shape_command_classifies_variable_count_rows() {
    let path = page_mark_variable_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-mark-shape")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("stream\t92\t92\tmini\n"));
    assert!(stdout.contains("header\t3\t16\t2\n"));
    assert!(stdout.contains("classification\tcount-plus-one-variable\t4\t20\t0\n"));
    assert!(stdout.contains("candidate\tcount-plus-one\t4\t20\t0\n"));
}

#[test]
fn page_mark_shape_command_classifies_fixed84_tail_rows() {
    let path = page_mark_fixed84_tail_shape_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("page-mark-shape")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("header\t6\t16\t4\n"));
    assert!(stdout.contains("classification\tfixed84-tail\t2\t84\t4\n"));
    assert!(stdout.contains("candidate\tfixed84\t2\t84\t4\n"));
}

#[test]
fn text_map_command_reports_token_ranges_and_position_hits() {
    let path = text_map_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-map")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("10\t14\t5\t7\ttext\t-\t1\t2\t銀河\n"));
    assert!(stdout.contains("14\t16\t7\t8\tcontrol\t0x001c\t-\t-\t\n"));
}

#[test]
fn text_position_context_command_compares_byte_and_unit_offsets() {
    let path = text_map_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-context")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains(
            "1\t10\thit:text(-)@10-14/5-7:銀河\thit:text(-)@18-24/9-12:鉄道\\n\tbetween:"
        )
    );
    assert!(
        stdout.contains(
            "2\t5\tbetween:-|text(-)@10-14/5-7:銀河\thit:text(-)@10-14/5-7:銀河\tbetween:"
        )
    );
}

#[test]
fn text_position_delta_scan_command_scores_unit_offsets() {
    let path = text_map_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("text-position-delta-scan")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("delta\t0\t2\t2\t2\n"));
    assert!(stdout.contains("delta\t29\t2\t0\t0\n"));
    assert!(stdout.contains("delta\t64\t2\t0\t0\n"));
}

#[test]
fn cat_command_reports_invalid_compressed_jttc_payload() {
    let path = compressed_jttc_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cat")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("invalid data"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn cat_command_extracts_embedded_document_text() {
    let path = embedded_document_text_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("cat")
        .arg(&path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Note");
}

#[test]
fn export_command_marks_embedded_document_text_source() {
    let path = embedded_document_text_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"text\":\"Note\""));
    assert!(stdout.contains("\"rawStreams\":[{\"name\":\"/EmbeddedDocumentText\""));
}

#[test]
fn export_command_writes_json_from_document_model() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"type\":\"paragraph\""));
    assert!(stdout.contains("\"text\":\"銀河\""));
    assert!(stdout.contains("\"text\":\"鉄道\""));
    assert!(stdout.contains("\"sourceSpan\":{\"byteStart\":10,\"byteEnd\":14"));
    assert!(stdout.contains("\"rawStreams\":[{\"name\":\"/DocumentText\",\"size\":24}]"));
}

#[test]
fn export_command_writes_markdown_from_document_model() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("md")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "銀河鉄道\n\n");
}

#[test]
fn export_command_writes_text_from_document_model() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("text")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "銀河鉄道\n");
}

#[test]
fn export_command_accepts_txt_alias() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("txt")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "銀河鉄道\n");
}

#[test]
fn export_command_writes_pdf_from_document_model() {
    let path = tiny_cfb_path();
    let output_path = path.with_extension("pdf");
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("pdf")
        .arg("-o")
        .arg(&output_path)
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let pdf = fs::read(&output_path).unwrap();
    fs::remove_file(&output_path).unwrap();

    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.ends_with(b"%%EOF"));
}

#[test]
fn export_command_rejects_pdf_without_output_path() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("pdf")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("PDF export requires"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn export_command_rejects_unknown_format() {
    let path = tiny_cfb_path();
    let output = Command::new(env!("CARGO_BIN_EXE_rjtd"))
        .arg("export")
        .arg(&path)
        .arg("--format")
        .arg("docx")
        .output()
        .unwrap();

    fs::remove_file(&path).unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("unsupported export format: docx"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn document_text_fixture() -> Vec<u8> {
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

fn document_text_from_str(text: &str) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.extend_from_slice(&[0x00, 0x1f]);
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn auto_text_info_fixture(text: &str) -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    bytes.resize(84, 0);
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn document_text_with_finance_table_rows() -> Vec<u8> {
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

fn document_text_with_sparse_table_rows() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(&mut bytes, &[0x001f]);
    append_sparse_table_row(&mut bytes, &["", "", "(1)表面積", ""]);
    append_sparse_table_row(&mut bytes, &["", "１", "", ""]);
    append_sparse_table_row(&mut bytes, &["", "ＡＢ　＝　ｃｍ", ""]);
    append_sparse_table_row(&mut bytes, &["", "ＡＣ　＝　ｃｍ", ""]);
    bytes
}

fn append_sparse_table_row(bytes: &mut Vec<u8>, cells: &[&str]) {
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

fn document_text_with_skipped_inline() -> Vec<u8> {
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

fn document_text_with_repeated_controls() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(
        &mut bytes,
        &[
            0x001f, 0x0041, 0x001c, 0x001f, 0x0042, 0x000e, 0x001f, 0x0043,
        ],
    );
    bytes
}

fn document_text_with_control_cluster() -> Vec<u8> {
    let mut bytes = b"SsmgV.01".to_vec();
    extend_units(
        &mut bytes,
        &[
            0x001f, 0x0041, 0x000e, 0x001d, 0x001f, 0x0042, 0x001c, 0x001f, 0x0043,
        ],
    );
    bytes
}

fn position_table_fixture() -> Vec<u8> {
    position_table_fixture_with_offsets(&[(1, 0x1234), (2, 0x5678)])
}

fn position_table_fixture_with_offsets(entries: &[(u16, u32)]) -> Vec<u8> {
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

fn text_count_table_fixture() -> Vec<u8> {
    text_count_table_fixture_with_ranges(&[(0x1234, 0x1250), (0x2000, 0x2400)])
}

fn text_count_table_fixture_with_ranges(entries: &[(u32, u32)]) -> Vec<u8> {
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

fn text_count_table_fixture_with_raw_entries(entries: &[[u8; 29]]) -> Vec<u8> {
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

fn extend_units(bytes: &mut Vec<u8>, units: &[u16]) {
    for unit in units {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
}
