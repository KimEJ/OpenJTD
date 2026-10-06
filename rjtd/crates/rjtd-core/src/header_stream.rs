//! Bounded plain-text slots in the observed `/Header` Ssmg profile.
//! Slot roles and layout are interpreted separately; other profiles stay raw.

pub const HEADER_PATH: &str = "/Header";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderTextCandidate {
    slot_id: u32,
    text: String,
    byte_start: usize,
    byte_end: usize,
}

impl HeaderTextCandidate {
    pub fn slot_id(&self) -> u32 {
        self.slot_id
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn byte_start(&self) -> usize {
        self.byte_start
    }
    pub fn byte_end(&self) -> usize {
        self.byte_end
    }
}

pub fn parse_header_text_candidates(data: &[u8]) -> Option<Vec<HeaderTextCandidate>> {
    let be32 = |offset| {
        Some(u32::from_be_bytes(
            data.get(offset..offset + 4)?.try_into().ok()?,
        ))
    };
    if data.get(..8) != Some(b"SsmgV.01") || be32(12)? != 0x100 {
        return None;
    }
    let count = usize::try_from(be32(8)?).ok()?;
    if !(1..=9).contains(&count) || data.len() != 16 + count * (512 + 24) + 8 {
        return None;
    }
    let table = 16 + count * 512;
    if be32(table)? != 0 || be32(data.len() - 4)? != 0 {
        return None;
    }
    let mut entries: Vec<HeaderTextCandidate> = Vec::with_capacity(count);
    for index in 0..count {
        let slot = 16 + index * 512;
        let units = usize::try_from(be32(slot + 12)?).ok()?;
        // A text slot occupies the first 256 bytes; its paired TCnt slot is empty.
        if units == 0 || units > 117 {
            return None;
        }
        let start = slot + 16;
        let end = start + units * 2;
        if be32(slot)? != if index == 0 { (count * 2) as u32 } else { 0 }
            || data.get(slot + 4..slot + 12) != Some(b"TextV.01")
            || data.get(end..end + 1) != Some(&[0])
            || be32(end + 1)? != units as u32
            || data.get(end + 5..slot + 260)?.iter().any(|byte| *byte != 0)
            || data.get(slot + 260..slot + 268) != Some(b"TCntV.01")
            || data
                .get(slot + 268..slot + 512)?
                .iter()
                .any(|byte| *byte != 0)
        {
            return None;
        }
        let record = table + 4 + index * 24;
        let id = be32(record)?;
        if entries.iter().any(|entry| entry.slot_id == id)
            || be32(record + 4)? != 0
            || be32(record + 8)? != (17 + 2 * units) as u32
            || be32(record + 12)? != 1
            || be32(record + 16)? != 1
            || be32(record + 20)? != (index * 2) as u32
        {
            return None;
        }
        let words = data[start..end]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|word| u16::from_be_bytes(*word))
            .collect::<Vec<_>>();
        let text = String::from_utf16(&words).ok()?;
        if text
            .chars()
            .any(|ch| ch.is_control() || matches!(ch, '\u{fffe}' | '\u{ffff}'))
        {
            return None;
        }
        entries.push(HeaderTextCandidate {
            slot_id: id,
            text,
            byte_start: start,
            byte_end: end,
        });
    }
    Some(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        let values = [(0_u32, "- ? -"), (1, "A日😀")];
        let mut bytes = vec![0; 16 + values.len() * 536 + 8];
        bytes[..8].copy_from_slice(b"SsmgV.01");
        bytes[8..12].copy_from_slice(&(values.len() as u32).to_be_bytes());
        bytes[12..16].copy_from_slice(&256_u32.to_be_bytes());
        bytes[16..20].copy_from_slice(&(values.len() as u32 * 2).to_be_bytes());
        let table = 16 + values.len() * 512;
        for (index, (id, text)) in values.into_iter().enumerate() {
            let slot = 16 + index * 512;
            let words = text.encode_utf16().collect::<Vec<_>>();
            bytes[slot + 4..slot + 12].copy_from_slice(b"TextV.01");
            bytes[slot + 12..slot + 16].copy_from_slice(&(words.len() as u32).to_be_bytes());
            for (i, word) in words.iter().enumerate() {
                bytes[slot + 16 + i * 2..slot + 18 + i * 2].copy_from_slice(&word.to_be_bytes());
            }
            let end = slot + 16 + words.len() * 2;
            bytes[end + 1..end + 5].copy_from_slice(&(words.len() as u32).to_be_bytes());
            bytes[slot + 260..slot + 268].copy_from_slice(b"TCntV.01");
            for (i, value) in [id, 0, 17 + words.len() as u32 * 2, 1, 1, index as u32 * 2]
                .into_iter()
                .enumerate()
            {
                let at = table + 4 + index * 24 + i * 4;
                bytes[at..at + 4].copy_from_slice(&value.to_be_bytes());
            }
        }
        bytes
    }

    #[test]
    fn reads_plain_slots_using_tail_ids_and_utf16_ranges() {
        let bytes = fixture();
        let entries = parse_header_text_candidates(&bytes).unwrap();
        assert_eq!(entries[1].slot_id(), 1);
        assert_eq!(entries[1].text(), "A日😀");
        assert_eq!((entries[1].byte_start(), entries[1].byte_end()), (544, 552));
    }

    #[test]
    fn rejects_truncation_duplicate_ids_styled_slots_and_invalid_utf16() {
        let original = fixture();
        for cut in [0, 12, 16, 100, original.len() - 1] {
            assert!(parse_header_text_candidates(&original[..cut]).is_none());
        }
        for (offset, value) in [
            (8, 255),
            (16 + 2 * 512 + 4 + 24 + 3, 0),
            (16 + 268, 1),
            (544, 0xdc),
        ] {
            let mut bytes = original.clone();
            bytes[offset] = value;
            assert!(
                parse_header_text_candidates(&bytes).is_none(),
                "offset {offset}"
            );
        }
    }
}
