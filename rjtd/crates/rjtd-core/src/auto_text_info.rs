use crate::container::read_cfb_stream;
use crate::{Error, Result};

pub const AUTO_TEXT_INFO_PATH: &str = "/AutoTextInfo";

const MIN_AUTO_TEXT_CHARS: usize = 4;
const SSMG_HEADER_BYTES: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoTextInfo {
    name: String,
    entries: Vec<AutoTextEntry>,
}

impl AutoTextInfo {
    fn new(name: impl Into<String>, entries: Vec<AutoTextEntry>) -> Self {
        Self {
            name: name.into(),
            entries,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn entries(&self) -> &[AutoTextEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoTextEntry {
    offset: usize,
    text: String,
}

impl AutoTextEntry {
    fn new(offset: usize, text: String) -> Self {
        Self { offset, text }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

pub fn read_auto_text_info(data: &[u8]) -> Result<AutoTextInfo> {
    let stream = read_cfb_stream(data, AUTO_TEXT_INFO_PATH)?;
    parse_auto_text_info(&stream)
}

pub fn parse_auto_text_info(data: &[u8]) -> Result<AutoTextInfo> {
    let mut entries = Vec::new();
    let scan_start = if data.starts_with(b"SsmgV.01") {
        SSMG_HEADER_BYTES
    } else {
        0
    };
    collect_utf16be_runs(data, scan_start, &mut entries);
    entries.sort_by_key(|entry| entry.offset());
    entries.dedup_by(|left, right| left.offset == right.offset && left.text == right.text);

    if entries.is_empty() {
        return Err(Error::InvalidData(
            "AutoTextInfo has no plausible UTF-16BE text runs".into(),
        ));
    }

    Ok(AutoTextInfo::new(AUTO_TEXT_INFO_PATH, entries))
}

fn collect_utf16be_runs(data: &[u8], alignment: usize, entries: &mut Vec<AutoTextEntry>) {
    let mut offset = alignment;
    let mut start = None;
    let mut units = Vec::new();

    while offset + 1 < data.len() {
        let unit = u16::from_be_bytes([data[offset], data[offset + 1]]);
        if is_auto_text_unit(unit) {
            start.get_or_insert(offset);
            units.push(unit);
            offset += 2;
            continue;
        }

        push_run(entries, start.take(), &mut units);
        offset += 2;
    }

    push_run(entries, start, &mut units);
}

fn push_run(entries: &mut Vec<AutoTextEntry>, start: Option<usize>, units: &mut Vec<u16>) {
    if let Some(start) = start
        && units.len() >= MIN_AUTO_TEXT_CHARS
        && let Ok(text) = String::from_utf16(units)
        && looks_like_auto_text(&text)
    {
        entries.push(AutoTextEntry::new(start, text));
    }
    units.clear();
}

fn looks_like_auto_text(text: &str) -> bool {
    text.chars().any(|character| {
        matches!(
            character as u32,
            0x3040..=0x30ff | 0x3400..=0x9fff | 0xf900..=0xfaff | 0xff00..=0xffef
        )
    })
}

fn is_auto_text_unit(unit: u16) -> bool {
    matches!(unit, 0x0009 | 0x000a | 0x000d)
        || (unit >= 0x20
            && !matches!(unit, 0xfffe | 0xffff)
            && !(0xfdd0..=0xfdef).contains(&unit)
            && !(0xd800..=0xdfff).contains(&unit))
}

#[cfg(test)]
mod tests {
    use super::{AUTO_TEXT_INFO_PATH, parse_auto_text_info, read_auto_text_info};
    use std::io::{Cursor, Write};

    #[test]
    fn parses_utf16be_auto_text_candidates() {
        let mut bytes = b"SsmgV.01".to_vec();
        bytes.resize(20, 0);
        for unit in "銀河鉄道の夜".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        bytes.extend_from_slice(&[0, 0, 0xff, 0xff]);

        let info = parse_auto_text_info(&bytes).unwrap();

        assert_eq!(info.name(), AUTO_TEXT_INFO_PATH);
        assert_eq!(info.entries().len(), 1);
        assert_eq!(info.entries()[0].offset(), 20);
        assert_eq!(info.entries()[0].text(), "銀河鉄道の夜");
    }

    #[test]
    fn reads_auto_text_info_from_cfb() {
        let mut payload = Vec::new();
        for unit in "文書題名".encode_utf16() {
            payload.extend_from_slice(&unit.to_be_bytes());
        }
        let bytes = cfb_with_stream(AUTO_TEXT_INFO_PATH, &payload);

        let info = read_auto_text_info(&bytes).unwrap();

        assert_eq!(info.entries()[0].text(), "文書題名");
    }

    fn cfb_with_stream(path: &str, payload: &[u8]) -> Vec<u8> {
        let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        compound
            .create_stream(path)
            .unwrap()
            .write_all(payload)
            .unwrap();
        compound.into_inner().into_inner()
    }
}
