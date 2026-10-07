use crate::{
    Document, DocumentTextFlowKind, ObjectImageDeclaredLengthCandidate, TextSourceSpan,
    document_text_raw_stream, read_be16_at,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeImageMode {
    Inline,
    Wrap,
    Front,
}

impl NativeImageMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Wrap => "wrap",
            Self::Front => "front",
        }
    }
}

/// Single-slot PNG/frame association in the controlled horizontal profile.
/// Other associations and alignment fields retain raw/candidate status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentImageFrameCandidate {
    pub(super) mode: NativeImageMode,
    pub(super) record_span: TextSourceSpan,
    pub(super) cache_end: usize,
    pub(super) image_index: usize,
    pub(super) payload_index: usize,
    pub(super) x: u16,
    pub(super) y: u16,
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) wrap_margin: u16,
}

impl DocumentImageFrameCandidate {
    pub fn mode(&self) -> NativeImageMode {
        self.mode
    }
    pub fn record_span(&self) -> &TextSourceSpan {
        &self.record_span
    }
    pub fn image_index(&self) -> usize {
        self.image_index
    }
    pub fn payload_index(&self) -> usize {
        self.payload_index
    }
    pub fn geometry_mm100(&self) -> [u16; 4] {
        [self.x, self.y, self.width, self.height]
    }
    pub fn wrap_margin_mm100(&self) -> u16 {
        self.wrap_margin
    }
}

impl Document {
    pub fn image_frame_candidates(&self) -> Vec<DocumentImageFrameCandidate> {
        native_image_binding(self).into_iter().collect()
    }
}

pub(super) fn native_image_binding(document: &Document) -> Option<DocumentImageFrameCandidate> {
    if !document.object_embedding_frames().is_empty()
        || document.object_stream_candidates().iter().any(|c| {
            !c.fdm_raw_vector_commands().is_empty()
                || c.jseq3_formula_candidate().is_some()
                || c.embedded_press_snapshot_candidate().is_some()
        })
    {
        return None;
    }
    let frames = document
        .object_stream_candidates()
        .iter()
        .filter(|c| c.path() == "/Frame")
        .collect::<Vec<_>>();
    let [source] = frames.as_slice() else {
        return None;
    };
    if source.size() != 76
        || source.payload_prefix() != [0, 1, 0, 4, 0, 2, 0, 1, 1, 1, 0, 4, 0, 0, 0, 1]
    {
        return None;
    }
    let [frame] = document.object_frame_records() else {
        return None;
    };
    if frame.source_path() != "/Frame"
        || frame.row_index() != 0
        || frame.row_start() != 16
        || frame.object_id() != 0
        || frame.object_type() != 1
    {
        return None;
    }
    let bytes = frame.raw_bytes();
    let profile = [
        0x102_u16, 56, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 1, 0, 0, 4,
        0x304, 0, 0,
    ];
    if bytes.len() != 60
        || bytes
            .as_chunks::<2>()
            .0
            .iter()
            .zip(profile)
            .enumerate()
            .any(|(i, (word, expected))| {
                !matches!(i, 8 | 14 | 16 | 18 | 20 | 23) && u16::from_be_bytes(*word) != expected
            })
    {
        return None;
    }
    let mode = match (read_be16_at(bytes, 16)?, read_be16_at(bytes, 46)?) {
        (0, 1) => NativeImageMode::Inline,
        (1, 1) => NativeImageMode::Wrap,
        (1, 4) => NativeImageMode::Front,
        _ => return None,
    };
    let images = document
        .object_stream_candidates()
        .iter()
        .enumerate()
        .flat_map(|(index, c)| {
            c.image_payload_spans()
                .iter()
                .enumerate()
                .map(move |(payload, span)| (index, payload, c, span))
        })
        .collect::<Vec<_>>();
    let [(image_index, payload_index, image, span)] = images.as_slice() else {
        return None;
    };
    if image.path() != "/EmbedItems/Embedding 1/Contents"
        || !span.complete()
        || span.mime() != "image/png"
        || span
            .envelope()
            .declared_payload_length()
            .map(ObjectImageDeclaredLengthCandidate::value)
            != Some(span.len())
    {
        return None;
    }
    let size = span.dimensions()?;
    if size.width() == 0 || size.height() == 0 {
        return None;
    }
    if frame.width() == 0
        || frame.height() == 0
        || frame.y() != 0
        || u64::from(frame.width()) * u64::from(size.height())
            != u64::from(frame.height()) * u64::from(size.width())
    {
        return None;
    }
    let flow = document.document_text_flow()?;
    let records = flow
        .events()
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind() == DocumentTextFlowKind::Record)
        .collect::<Vec<_>>();
    let [(index, record)] = records.as_slice() else {
        return None;
    };
    let context = if mode == NativeImageMode::Inline {
        (0x107, 0x10)
    } else {
        (0x507, 0x12)
    };
    if record.raw_words()
        != [
            0x1c, 0, 14, 0, 0x30, 0xffff, context.0, context.1, 0, 0, 14, 0, 0, 0x1f,
        ]
    {
        return None;
    }
    let group = flow.events().get(index + 1..index + 5)?;
    let [start, prefix, value, suffix] = group else {
        return None;
    };
    if start.kind() != DocumentTextFlowKind::Control
        || start.code() != Some(0x1c)
        || prefix.kind() != DocumentTextFlowKind::Opaque
        || prefix.raw_words() != [1, 7, 0, 0, 1]
        || value.kind() != DocumentTextFlowKind::Inline
        || value.selector() != Some(1)
        || !value.text().is_empty()
        || value.unit_end().checked_sub(value.unit_start())? != 3
        || suffix.kind() != DocumentTextFlowKind::Opaque
        || suffix.raw_words() != [5, 0, 1, 0x1f]
        || record.unit_end() != start.unit_start()
        || !group
            .windows(2)
            .all(|pair| pair[0].unit_end() == pair[1].unit_start())
    {
        return None;
    }
    let raw = document_text_raw_stream(document)?;
    if raw.get(value.source_span().byte_start()..value.source_span().byte_end())?
        != [0, 0x1d, 0, 2, 0, 0x1e]
    {
        return None;
    }
    let text_events = flow
        .events()
        .iter()
        .filter(|e| e.kind() == DocumentTextFlowKind::Text)
        .collect::<Vec<_>>();
    if flow.events().len()
        != if mode == NativeImageMode::Inline {
            7
        } else {
            6
        }
        || text_events.len()
            != if mode == NativeImageMode::Inline {
                2
            } else {
                1
            }
        || text_events.iter().any(|e| {
            e.text().is_empty()
                || e.text().len() > 256
                || !e.text().chars().all(|c| c.is_ascii_graphic() || c == ' ')
        })
    {
        return None;
    }
    if mode == NativeImageMode::Inline {
        if text_events[0].unit_start() != flow.source_span().unit_start()
            || text_events[0].unit_end() != record.unit_start()
            || text_events[1].unit_start() != suffix.unit_end()
        {
            return None;
        }
    } else if record.unit_start() != flow.source_span().unit_start()
        || text_events[0].unit_start() != suffix.unit_end()
    {
        return None;
    }
    Some(DocumentImageFrameCandidate {
        mode,
        record_span: record.source_span().clone(),
        cache_end: suffix.unit_end(),
        image_index: *image_index,
        payload_index: *payload_index,
        x: frame.x(),
        y: frame.y(),
        width: frame.width(),
        height: frame.height(),
        wrap_margin: 200,
    })
}
