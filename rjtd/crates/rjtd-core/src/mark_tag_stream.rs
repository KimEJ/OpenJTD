//! Bounded names in `/MarkTag`; directory values do not decode text positions.

pub const MARK_TAG_PATH: &str = "/MarkTag";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkTagNameCandidate {
    entry_value: u32,
    name: String,
    byte_start: usize,
    byte_end: usize,
}

impl MarkTagNameCandidate {
    pub fn entry_value(&self) -> u32 {
        self.entry_value
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn byte_start(&self) -> usize {
        self.byte_start
    }
    pub fn byte_end(&self) -> usize {
        self.byte_end
    }
}

pub fn parse_mark_tag_name_candidates(data: &[u8]) -> Option<Vec<MarkTagNameCandidate>> {
    if data.len() > 64 * 1024 || data.get(..8) != Some(b"MarkV.01") {
        return None;
    }
    let count = usize::from(u16::from_be_bytes(data.get(8..10)?.try_into().ok()?));
    if count > 1024 {
        return None;
    }
    let mut cursor = 10;
    let mut names = Vec::new();
    for _ in 0..count {
        let start = cursor;
        let entry_value = u32::from_be_bytes(data.get(cursor..cursor + 4)?.try_into().ok()?);
        let length = usize::from(u16::from_be_bytes(
            data.get(cursor + 4..cursor + 6)?.try_into().ok()?,
        ));
        if !(1..=1024).contains(&length) {
            return None;
        }
        cursor += 6;
        let end = cursor.checked_add(length * 2)?;
        let words = data
            .get(cursor..end)?
            .as_chunks::<2>()
            .0
            .iter()
            .map(|word| u16::from_be_bytes(*word))
            .collect::<Vec<_>>();
        let name = String::from_utf16(&words).ok()?;
        if name.chars().any(char::is_control) {
            return None;
        }
        names.push(MarkTagNameCandidate {
            entry_value,
            name,
            byte_start: start,
            byte_end: end,
        });
        cursor = end;
    }
    (cursor == data.len()).then_some(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn directory() -> Vec<u8> {
        let mut bytes = b"MarkV.01".to_vec();
        bytes.extend(2_u16.to_be_bytes());
        for (value, name) in [(0_u32, "POINT"), (0x10000, "節😀")] {
            bytes.extend(value.to_be_bytes());
            bytes.extend((name.encode_utf16().count() as u16).to_be_bytes());
            bytes.extend(name.encode_utf16().flat_map(u16::to_be_bytes));
        }
        bytes
    }
    #[test]
    fn names_keep_directory_values_utf16_and_record_spans() {
        let data = directory();
        let names = parse_mark_tag_name_candidates(&data).unwrap();
        assert_eq!(
            names
                .iter()
                .map(MarkTagNameCandidate::name)
                .collect::<Vec<_>>(),
            ["POINT", "節😀"]
        );
        assert_eq!(names[1].entry_value(), 0x10000);
        assert_eq!(names[0].byte_start(), 10);
        assert_eq!(names[0].byte_end(), names[1].byte_start());
        assert_eq!(names[1].byte_end(), data.len());
    }
    #[test]
    fn malformed_unknown_and_excessive_directories_stay_unparsed() {
        let data = directory();
        for end in 0..data.len() {
            assert!(parse_mark_tag_name_candidates(&data[..end]).is_none());
        }
        let mut trailing = data.clone();
        trailing.push(0);
        assert!(parse_mark_tag_name_candidates(&trailing).is_none());
        let mut bad_utf16 = data.clone();
        bad_utf16[16..18].copy_from_slice(&0xd800_u16.to_be_bytes());
        assert!(parse_mark_tag_name_candidates(&bad_utf16).is_none());
        let mut too_many = data.clone();
        too_many[8..10].copy_from_slice(&1025_u16.to_be_bytes());
        assert!(parse_mark_tag_name_candidates(&too_many).is_none());
        assert!(parse_mark_tag_name_candidates(&vec![0; 65537]).is_none());
    }
}
