use crate::*;

pub(crate) fn shanai_lan_line_mark_intervals(
    document: &Document,
) -> Vec<ShanaiLanLineMarkInterval> {
    let Some(bytes) = document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/LineMark")
        .map(RawStream::bytes)
    else {
        return Vec::new();
    };
    shanai_lan_line_mark_intervals_from_bytes(bytes)
}

#[cfg(feature = "rendering")]
pub(crate) fn shanai_lan_line_mark_profile(document: &Document) -> &'static str {
    document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/LineMark")
        .map(RawStream::bytes)
        .map(shanai_lan_line_mark_profile_from_bytes)
        .unwrap_or(SHANAI_LAN_LINE_MARK_PROFILE_ABSENT)
}

#[cfg(feature = "rendering")]
pub(crate) fn shanai_lan_line_mark_profile_from_bytes(bytes: &[u8]) -> &'static str {
    if !shanai_lan_line_mark_intervals_from_bytes(bytes).is_empty() {
        return SHANAI_LAN_LINE_MARK_PROFILE_BE_DELTA_V1;
    }
    if utf16le_ascii_contains(bytes, "MacrosStreamStyle") {
        return SHANAI_LAN_LINE_MARK_PROFILE_MACRO_STYLE;
    }
    SHANAI_LAN_LINE_MARK_PROFILE_UNPARSED
}

pub(crate) fn shanai_lan_line_mark_intervals_from_bytes(
    bytes: &[u8],
) -> Vec<ShanaiLanLineMarkInterval> {
    let Some(count) = read_be16_at(bytes, LINE_MARK_BE_DELTA_COUNT_OFFSET).map(usize::from) else {
        return Vec::new();
    };
    if count == 0
        || bytes.len()
            < LINE_MARK_BE_DELTA_HEADER_BYTES
                + count.saturating_mul(LINE_MARK_BE_DELTA_RECORD_BYTES)
    {
        return Vec::new();
    }

    let mut intervals = Vec::new();
    let mut unit_start = LINE_MARK_BE_DELTA_BASE_UNIT;
    for record_index in 0..count {
        let offset = line_mark_be_delta_record_byte_offset(record_index);
        let Some(delta_word) = read_be16_at(bytes, offset) else {
            break;
        };
        let Some(flag_word) = read_be16_at(bytes, offset + 2) else {
            break;
        };
        let delta = delta_word as i16;
        if delta <= 0 {
            break;
        }
        let unit_end = unit_start.saturating_add(delta as usize);
        intervals.push(ShanaiLanLineMarkInterval {
            record_index,
            unit_start,
            unit_end,
            flag_word,
        });
        unit_start = unit_end;
    }
    intervals
}

#[cfg(feature = "rendering")]
pub(crate) fn shanai_lan_line_mark_for_header(
    intervals: &[ShanaiLanLineMarkInterval],
    header: &ShanaiLanLineHeader,
) -> Option<ShanaiLanLineMarkInterval> {
    let unit = header.start / 2;
    intervals
        .iter()
        .copied()
        .find(|interval| interval.unit_start <= unit && unit < interval.unit_end)
}

#[cfg(feature = "rendering")]
pub(crate) fn shanai_lan_line_header_at(
    bytes: &[u8],
    offset: usize,
) -> Option<ShanaiLanLineHeader> {
    let raw = bytes.get(offset..offset.checked_add(24)?)?;
    if !raw.starts_with(&[0x00, 0x1c, 0x00, 0x30]) {
        return None;
    }
    let mut words = [0u16; 12];
    for (index, chunk) in raw.as_chunks::<2>().0.iter().enumerate() {
        words[index] = u16::from_be_bytes([chunk[0], chunk[1]]);
    }
    // Class 0x0030 carries a word count and an echoed count, not a font size.
    // Native cell-text records also carry zero at word 6. Preserve that opaque
    // field while requiring the same complete fixed twelve-word framing.
    if words[2] != 12
        || !matches!(words[6], 0 | 0x00ff)
        || words[7] != 0
        || words[8] != 12
        || words[9] != 0
        || words[10] != 0x0030
        || words[11] != 0x001f
    {
        return None;
    }
    Some(ShanaiLanLineHeader {
        offset_units: words[4],
        extent_units: words[5],
        font_size_units: words[2],
        raw_words: words,
        start: offset,
        end: offset + 24,
    })
}

pub(crate) fn line_mark_be_delta_record_byte_offset(record_index: usize) -> usize {
    LINE_MARK_BE_DELTA_HEADER_BYTES + record_index * LINE_MARK_BE_DELTA_RECORD_BYTES
}

#[cfg(feature = "rendering")]
pub(crate) fn line_mark_be_delta_record_word_index(record_index: usize) -> usize {
    line_mark_be_delta_record_byte_offset(record_index) / 2
}

pub(crate) const LINE_MARK_BE_DELTA_HEADER_BYTES: usize = 18;

pub(crate) const LINE_MARK_BE_DELTA_COUNT_OFFSET: usize = 8;

pub(crate) const LINE_MARK_BE_DELTA_BASE_UNIT: usize = 16;

pub(crate) const LINE_MARK_BE_DELTA_RECORD_BYTES: usize = 4;

#[cfg(feature = "rendering")]
pub(crate) const SHANAI_LAN_LINE_MARK_PROFILE_ABSENT: &str = "absent";

#[cfg(feature = "rendering")]
pub(crate) const SHANAI_LAN_LINE_MARK_PROFILE_BE_DELTA_V1: &str = "be16-delta-v1";

#[cfg(feature = "rendering")]
pub(crate) const SHANAI_LAN_LINE_MARK_PROFILE_MACRO_STYLE: &str = "macro-stream-style-reference";

#[cfg(feature = "rendering")]
pub(crate) const SHANAI_LAN_LINE_MARK_PROFILE_UNPARSED: &str = "unparsed";
