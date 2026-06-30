use crate::compressed_document::{
    decompress_just_compressed_document, is_just_compressed_document,
};
use crate::container::read_cfb_stream;
use crate::document_text::COMPRESSED_DOCUMENT_PATH;
use crate::{Error, Result};

pub const FONT_STREAM_PATH: &str = "/Font";

const FONT_STREAM_MAGIC: &[u8; 8] = b"FontV.01";
const FONT_STREAM_HEADER_LEN: usize = 10;
const FONT_ENTRY_NAME_OFFSET: usize = 30;
const FONT_NEXT_ENTRY_SEARCH_BYTES: usize = 64;
const FONT_NAME_MAX_UNITS: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontStream {
    name: String,
    bytes: Vec<u8>,
    declared_count: u16,
    entries: Vec<FontEntry>,
}

impl FontStream {
    fn new(name: impl Into<String>, bytes: Vec<u8>) -> Self {
        let declared_count = declared_font_count(&bytes).unwrap_or(0);
        let entries = parse_font_entries(&bytes);
        Self {
            name: name.into(),
            bytes,
            declared_count,
            entries,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn declared_count(&self) -> u16 {
        self.declared_count
    }

    pub fn entries(&self) -> &[FontEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontEntry {
    id: u16,
    offset: usize,
    name: String,
    raw: Vec<u8>,
}

impl FontEntry {
    fn new(id: u16, offset: usize, name: String, raw: Vec<u8>) -> Self {
        Self {
            id,
            offset,
            name,
            raw,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
}

pub fn read_font_stream(data: &[u8]) -> Result<FontStream> {
    if let Some(inner_document) = maybe_decompressed_inner_document(data)? {
        return read_font_stream_from_cfb(&inner_document);
    }

    read_font_stream_from_cfb(data)
}

pub fn summarize_font_stream(data: &[u8]) -> FontStream {
    FontStream::new(FONT_STREAM_PATH, data.to_vec())
}

fn read_font_stream_from_cfb(data: &[u8]) -> Result<FontStream> {
    let bytes = read_cfb_stream(data, FONT_STREAM_PATH)?;
    Ok(FontStream::new(FONT_STREAM_PATH, bytes))
}

fn maybe_decompressed_inner_document(data: &[u8]) -> Result<Option<Vec<u8>>> {
    let compressed = match read_cfb_stream(data, COMPRESSED_DOCUMENT_PATH) {
        Ok(stream) => stream,
        Err(Error::NotFound(_)) => return Ok(None),
        Err(error) => return Err(error),
    };

    if is_just_compressed_document(&compressed) {
        Ok(Some(decompress_just_compressed_document(&compressed)?))
    } else {
        Ok(None)
    }
}

fn declared_font_count(data: &[u8]) -> Option<u16> {
    if !data.starts_with(FONT_STREAM_MAGIC) {
        return None;
    }
    let bytes = data.get(8..10)?;
    Some(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn parse_font_entries(data: &[u8]) -> Vec<FontEntry> {
    let Some(declared_count) = declared_font_count(data) else {
        return Vec::new();
    };

    let mut entries = Vec::new();
    let mut offset = FONT_STREAM_HEADER_LEN;

    for index in 0..declared_count as usize {
        if offset + FONT_ENTRY_NAME_OFFSET >= data.len() {
            break;
        }

        let id = read_u16_be(data, offset).unwrap_or(index as u16);
        let name_start = offset + FONT_ENTRY_NAME_OFFSET;
        let Some((name, name_end)) = read_utf16be_null_string(data, name_start) else {
            break;
        };
        if !looks_like_font_name(&name) {
            break;
        }

        let next_offset = if index + 1 == declared_count as usize {
            data.len()
        } else {
            find_next_font_entry_offset(data, name_end).unwrap_or(name_end)
        };
        if next_offset <= offset || next_offset > data.len() {
            break;
        }

        entries.push(FontEntry::new(
            id,
            offset,
            name,
            data[offset..next_offset].to_vec(),
        ));
        offset = next_offset;
    }

    entries
}

fn find_next_font_entry_offset(data: &[u8], after_name: usize) -> Option<usize> {
    let search_end = data
        .len()
        .min(after_name.saturating_add(FONT_NEXT_ENTRY_SEARCH_BYTES));
    for candidate in after_name..search_end {
        if !looks_like_font_entry_header(data, candidate) {
            continue;
        }
        let name_start = candidate.checked_add(FONT_ENTRY_NAME_OFFSET)?;
        if name_start >= data.len() {
            break;
        }
        let Some((name, _)) = read_utf16be_null_string(data, name_start) else {
            continue;
        };
        if looks_like_font_name(&name) {
            return Some(candidate);
        }
    }
    None
}

fn looks_like_font_entry_header(data: &[u8], offset: usize) -> bool {
    let Some(zero_area) = data.get(offset + 2..offset + 20) else {
        return false;
    };
    zero_area.iter().all(|byte| *byte == 0)
}

fn read_utf16be_null_string(data: &[u8], start: usize) -> Option<(String, usize)> {
    let mut units = Vec::new();
    let mut offset = start;

    while offset + 1 < data.len() && units.len() <= FONT_NAME_MAX_UNITS {
        let unit = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        if unit == 0 {
            let value = String::from_utf16(&units).ok()?;
            return Some((value, offset));
        }
        if unit < 0x20 || matches!(unit, 0xfffe | 0xffff) || (0xd800..=0xdfff).contains(&unit) {
            return None;
        }
        units.push(unit);
    }

    None
}

fn looks_like_font_name(value: &str) -> bool {
    let trimmed = value.trim();
    let char_count = trimmed.chars().count();
    if !(2..=FONT_NAME_MAX_UNITS).contains(&char_count) {
        return false;
    }

    trimmed.chars().all(is_font_name_character)
        && trimmed
            .chars()
            .any(|character| character.is_alphanumeric() || is_cjk_or_kana(character))
}

fn is_font_name_character(character: char) -> bool {
    character.is_ascii_graphic()
        || character == ' '
        || matches!(
            character as u32,
            0x3000..=0x30ff
                | 0x3400..=0x4dbf
                | 0x4e00..=0x9fff
                | 0xac00..=0xd7af
                | 0xff00..=0xffef
        )
}

fn is_cjk_or_kana(character: char) -> bool {
    matches!(
        character as u32,
        0x3040..=0x30ff | 0x3400..=0x4dbf | 0x4e00..=0x9fff | 0xac00..=0xd7af | 0xff00..=0xffef
    )
}

fn read_u16_be(data: &[u8], offset: usize) -> Option<u16> {
    let bytes = data.get(offset..offset + 2)?;
    Some(u16::from_be_bytes([bytes[0], bytes[1]]))
}

#[cfg(test)]
mod tests {
    use super::{FONT_STREAM_PATH, summarize_font_stream};
    use std::io::{Cursor, Write};

    #[test]
    fn summarizes_font_stream_entries() {
        let bytes = font_stream_fixture(&[
            (0, "ＭＳ 明朝", 18),
            (2, "ＭＳ ゴシック", 18),
            (3, "Times New Roman", 18),
        ]);

        let stream = summarize_font_stream(&bytes);

        assert_eq!(stream.name(), FONT_STREAM_PATH);
        assert_eq!(stream.declared_count(), 3);
        assert_eq!(stream.entries().len(), 3);
        assert_eq!(stream.entries()[0].id(), 0);
        assert_eq!(stream.entries()[0].offset(), 10);
        assert_eq!(stream.entries()[0].name(), "ＭＳ 明朝");
        assert_eq!(stream.entries()[1].name(), "ＭＳ ゴシック");
        assert_eq!(stream.entries()[2].name(), "Times New Roman");
        assert!(!stream.entries()[0].raw().is_empty());
    }

    #[test]
    fn reads_font_stream_from_cfb() {
        let bytes = cfb_with_font_stream(&font_stream_fixture(&[(1, "Arial", 18)]));

        let stream = super::read_font_stream(&bytes).unwrap();

        assert_eq!(stream.entries().len(), 1);
        assert_eq!(stream.entries()[0].name(), "Arial");
    }

    fn font_stream_fixture(entries: &[(u16, &str, usize)]) -> Vec<u8> {
        let mut bytes = b"FontV.01".to_vec();
        bytes.extend_from_slice(&(entries.len() as u16).to_be_bytes());
        for (id, name, suffix_len) in entries {
            bytes.extend_from_slice(&font_entry_fixture(*id, name, *suffix_len));
        }
        bytes
    }

    fn font_entry_fixture(id: u16, name: &str, suffix_len: usize) -> Vec<u8> {
        let mut entry = vec![0; 30];
        entry[0..2].copy_from_slice(&id.to_be_bytes());
        entry[20..22].copy_from_slice(&0x0190u16.to_be_bytes());
        for unit in name.encode_utf16() {
            entry.extend_from_slice(&unit.to_be_bytes());
        }
        entry.extend_from_slice(&[0, 0]);
        entry.extend(std::iter::repeat_n(0, suffix_len));
        entry
    }

    fn cfb_with_font_stream(font: &[u8]) -> Vec<u8> {
        let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        compound
            .create_stream(FONT_STREAM_PATH)
            .unwrap()
            .write_all(font)
            .unwrap();
        compound.into_inner().into_inner()
    }
}
