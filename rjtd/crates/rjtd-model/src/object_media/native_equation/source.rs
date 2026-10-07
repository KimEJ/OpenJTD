use crate::{
    Document, DocumentTextFlowKind, JSEQ3_CONTENTS_MAGIC_UTF16LE, TextSourceSpan,
    decode_utf16le_c_string, document_text_raw_stream, read_le32_at,
    shanai_lan_line_mark_intervals,
};

const CONTENTS: &str = "/EmbedItems/Embedding 1/JSEQ3Contents";
const SNAPSHOT: &str = "/EmbedItems/Embedding 1/\x03EmbeddedPress";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeEquationGlyphCandidate {
    pub(super) text: String,
    pub(super) x: u32,
    pub(super) y: u32,
    pub(super) font_size: u32,
    pub(super) family: String,
    pub(super) italic: bool,
    pub(super) byte_start: usize,
    pub(super) byte_end: usize,
    pub(super) contents_offset: usize,
}

impl NativeEquationGlyphCandidate {
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn position_mm100(&self) -> [u32; 2] {
        [self.x, self.y]
    }
    pub fn font_size_mm100(&self) -> u32 {
        self.font_size
    }
    pub fn family(&self) -> &str {
        &self.family
    }
    pub fn italic(&self) -> bool {
        self.italic
    }
    pub fn snapshot_range(&self) -> [usize; 2] {
        [self.byte_start, self.byte_end]
    }
    pub fn contents_offset(&self) -> usize {
        self.contents_offset
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeEquationCandidate {
    pub(super) frame: [u16; 4],
    pub(super) line: usize,
    pub(super) record_span: TextSourceSpan,
    pub(super) glyphs: Vec<NativeEquationGlyphCandidate>,
}

impl NativeEquationCandidate {
    pub fn frame_mm100(&self) -> [u16; 4] {
        self.frame
    }
    pub fn line_mark_index(&self) -> usize {
        self.line
    }
    pub fn record_span(&self) -> &TextSourceSpan {
        &self.record_span
    }
    pub fn glyphs(&self) -> &[NativeEquationGlyphCandidate] {
        &self.glyphs
    }
}

fn raw<'a>(document: &'a Document, name: &str) -> Option<&'a [u8]> {
    let mut streams = document.raw_streams().iter().filter(|s| s.name() == name);
    let bytes = streams.next()?.bytes();
    streams.next().is_none().then_some(bytes)
}

/// Font and one-character TextOut packets in the controlled GCI profile.
/// TextOut uses the saved top reference; font/vertical metrics remain candidates.
fn snapshot_glyphs(
    bytes: &[u8],
    width: u32,
    height: u32,
) -> Option<Vec<NativeEquationGlyphCandidate>> {
    if bytes.len() > 64 * 1024
        || bytes.get(..12)? != b"JSSnapShot32"
        || bytes.get(0x2c..0x30)? != b"GCI\0"
        || read_le32_at(bytes, 0x48)? != width
        || read_le32_at(bytes, 0x4c)? != height
    {
        return None;
    }
    let mut offset = 0x80;
    let mut font = None;
    let mut selected = false;
    let mut result = Vec::new();
    let mut stage = 0;
    while offset < bytes.len() {
        let n = usize::try_from(read_le32_at(bytes, offset)?).ok()?;
        if n < 8 || n % 4 != 0 {
            return None;
        }
        let end = offset.checked_add(n)?;
        let record = bytes.get(offset..end)?;
        let kind = read_le32_at(record, 4)?;
        let p = &record[8..];
        match (kind, n) {
            (0x3c, 12) if result.is_empty() && offset == 0x80 => {
                if read_le32_at(p, 0)? != 0xcc {
                    return None;
                }
            }
            (0x24, 20) if offset == 0x8c => {
                if p != [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0] {
                    return None;
                }
            }
            (0x94, 244) if stage == 0 => {
                if read_le32_at(p, 0)? != 228
                    || read_le32_at(p, 4)? != 16
                    || read_le32_at(p, 8)? != 0
                    || read_le32_at(p, 12)? != 0xffffff
                    || read_le32_at(p, 20)? != 36
                    || read_le32_at(p, 24)? != 140
                    || read_le32_at(p, 28)? != 172
                    || read_le32_at(p, 60)? != 400
                    || !matches!(p[64], 0 | 255)
                    || p.get(65..68)? != [0, 0, 0]
                    || read_le32_at(p, 180)? != 56
                    || read_le32_at(p, 184)? != 0
                    || read_le32_at(p, 192)? != 1024
                    || read_le32_at(p, 196)? != 1024
                {
                    return None;
                }
                let family = decode_utf16le_c_string(p.get(72..136)?)?;
                if family != "Times New Roman" {
                    return None;
                }
                let size = read_le32_at(p, 188)?;
                if !(100..=1000).contains(&size) {
                    return None;
                }
                font = Some((family, size, p[64] == 255));
                selected = false;
                stage = 1;
            }
            (0x60, 16) => {
                let id = read_le32_at(p, 0)?;
                if read_le32_at(p, 4)? != 0 || !matches!(id, 3 | 16) {
                    return None;
                }
                if !((stage == 1 && id == 16) || (stage == 4 && id == 3)) {
                    return None;
                }
                selected = id == 16;
                stage = if selected { 2 } else { 5 };
            }
            (0x40, 12) if stage == 2 => {
                if read_le32_at(p, 0)? != 1 {
                    return None;
                }
                stage = 3;
            }
            (0xc8, 48) if stage == 3 => {
                if !selected
                    || read_le32_at(p, 8)? != 0
                    || read_le32_at(p, 12)? != 0
                    || read_le32_at(p, 16)? != 4
                    || read_le32_at(p, 20)? != 1
                    || p.get(24..36)?.iter().any(|b| *b != 0)
                {
                    return None;
                }
                let (family, size, italic) = font.as_ref()?;
                let ch = char::from_u32(read_le32_at(p, 36)?)?;
                if !(ch.is_ascii_alphanumeric() || matches!(ch, '+' | '-' | '=' | '(' | ')')) {
                    return None;
                }
                let x = read_le32_at(p, 0)?;
                let y = read_le32_at(p, 4)?;
                if x >= width || y >= height {
                    return None;
                }
                result.push(NativeEquationGlyphCandidate {
                    text: ch.to_string(),
                    x,
                    y,
                    font_size: *size,
                    family: family.clone(),
                    italic: *italic,
                    byte_start: offset,
                    byte_end: end,
                    contents_offset: 0,
                });
                if result.len() > 32 {
                    return None;
                }
                stage = 4;
            }
            (0x65, 12) if stage == 5 => {
                if read_le32_at(p, 0)? != 16 {
                    return None;
                }
                font = None;
                selected = false;
                stage = 0;
            }
            _ => return None,
        }
        offset = end;
    }
    (!result.is_empty() && font.is_none() && !selected && stage == 0).then_some(result)
}

impl Document {
    pub fn equation_candidates(&self) -> Vec<NativeEquationCandidate> {
        native_equation_binding(self).into_iter().collect()
    }
}

pub(super) fn native_equation_binding(document: &Document) -> Option<NativeEquationCandidate> {
    let [frame] = document.object_frame_records() else {
        return None;
    };
    let [embedding] = document.object_embedding_frames() else {
        return None;
    };
    if frame.source_path() != "/Frame"
        || frame.object_id() != 0
        || frame.object_type() != 1
        || embedding.class_name() != "JSEQ.Document.3"
        || embedding.embedding_index() != 1
        || embedding.frame_ref() != 0
        || embedding.frame_width() != u32::from(frame.width())
        || embedding.frame_height() != u32::from(frame.height())
        || frame.x() != 0
        || frame.y() != 0
    {
        return None;
    }
    let row = frame.raw_bytes();
    let profile = [
        0x102_u16, 56, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 1, 0, 0, 4,
        0x300, 0, 0,
    ];
    if row.len() != 60
        || row
            .as_chunks::<2>()
            .0
            .iter()
            .zip(profile)
            .enumerate()
            .any(|(i, (w, v))| !matches!(i, 14 | 16 | 18 | 20) && u16::from_be_bytes(*w) != v)
    {
        return None;
    }
    let contents = raw(document, CONTENTS)?;
    if contents.len() > 64 * 1024 || contents.get(..16)? != JSEQ3_CONTENTS_MAGIC_UTF16LE {
        return None;
    }
    let snapshot = raw(document, SNAPSHOT)?;
    let mut glyphs = snapshot_glyphs(
        snapshot,
        u32::from(frame.width()),
        u32::from(frame.height()),
    )?;
    // Same character/color packets in the editable formula corroborate cached glyphs.
    let mut cursor = 0x80;
    for glyph in &mut glyphs {
        let ch = glyph.text.chars().next()? as u32;
        let mut packet = ch.to_le_bytes().to_vec();
        packet.extend([0, 0, 0, 0, 255, 255, 255, 0]);
        let offset = contents
            .get(cursor..)?
            .windows(packet.len())
            .position(|w| w == packet)?
            + cursor;
        if offset % 2 != 0 {
            return None;
        }
        glyph.contents_offset = offset;
        cursor = offset + packet.len();
    }
    let flow = document.document_text_flow()?;
    let events = flow.events();
    let [before, record, start, prefix, value, suffix, after] = events else {
        return None;
    };
    if before.kind() != DocumentTextFlowKind::Text
        || after.kind() != DocumentTextFlowKind::Text
        || !before.text().ends_with('\n')
        || !after.text().starts_with('\n')
        || before.text().chars().filter(|c| *c == '\n').count() != 1
        || after.text().chars().filter(|c| *c == '\n').count() != 1
        || record.raw_words()
            != [
                0x1c, 0, 14, 0, 0x30, 0xffff, 0x507, 0x12, 0, 0, 14, 0, 0, 0x1f,
            ]
        || start.kind() != DocumentTextFlowKind::Control
        || start.code() != Some(0x1c)
        || prefix.raw_words() != [1, 7, 0, 0, 1]
        || value.kind() != DocumentTextFlowKind::Inline
        || value.selector() != Some(1)
        || !value.text().is_empty()
        || suffix.raw_words() != [5, 0, 1, 0x1f]
        || !events
            .windows(2)
            .all(|p| p[0].unit_end() == p[1].unit_start())
    {
        return None;
    }
    let source = document_text_raw_stream(document)?;
    if source.get(value.source_span().byte_start()..value.source_span().byte_end())?
        != [0, 0x1d, 0, 2, 0, 0x1e]
    {
        return None;
    }
    let intervals = shanai_lan_line_mark_intervals(document);
    let [first, middle, last] = intervals.as_slice() else {
        return None;
    };
    if first.unit_start != flow.source_span().unit_start()
        || middle.unit_start != record.unit_start()
        || middle.flag_word != 0x8002
        || last.unit_start != suffix.unit_end() + 1
        || last.unit_end != flow.source_span().unit_end() + 1
    {
        return None;
    }
    Some(NativeEquationCandidate {
        frame: [frame.x(), frame.y(), frame.width(), frame.height()],
        line: middle.record_index,
        record_span: record.source_span().clone(),
        glyphs,
    })
}
