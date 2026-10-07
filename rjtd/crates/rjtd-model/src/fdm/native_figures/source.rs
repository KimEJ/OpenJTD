use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Document, DocumentTextFlowKind, PAGE_LAYOUT_STYLE_PATH, TextSourceSpan,
    document_text_raw_stream, read_be16_at, read_be32_at, read_i32_be_at,
    shanai_lan_line_mark_intervals,
};

/// Source-bound rectangle/ellipse/line in the controlled three-figure profile.
/// Units, paint and anchor roles remain candidates; raw streams are retained.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeFigureShapeCandidate {
    pub(super) object_id: u16,
    pub(super) slot: usize,
    pub(super) kind: &'static str,
    pub(super) frame: [u16; 4],
    pub(super) line: usize,
    pub(super) record_span: TextSourceSpan,
    pub(super) vector_offset: usize,
    pub(super) command_offset: usize,
    pub(super) bounds: [f64; 4],
    pub(super) points: Vec<[f64; 2]>,
    pub(super) fill: Option<String>,
    pub(super) stroke_units: u16,
}

impl NativeFigureShapeCandidate {
    pub fn object_id(&self) -> u16 {
        self.object_id
    }
    pub fn slot(&self) -> usize {
        self.slot
    }
    pub fn kind(&self) -> &'static str {
        self.kind
    }
    pub fn frame_mm100(&self) -> [u16; 4] {
        self.frame
    }
    pub fn line_mark_index(&self) -> usize {
        self.line
    }
    pub fn record_span(&self) -> &TextSourceSpan {
        &self.record_span
    }
    pub fn vector_offset(&self) -> usize {
        self.vector_offset
    }
    pub fn command_offset(&self) -> usize {
        self.command_offset
    }
    pub fn fill(&self) -> Option<&str> {
        self.fill.as_deref()
    }
    pub fn stroke_units(&self) -> u16 {
        self.stroke_units
    }
}

fn raw<'a>(document: &'a Document, path: &str) -> Option<&'a [u8]> {
    let mut streams = document.raw_streams().iter().filter(|s| s.name() == path);
    let bytes = streams.next()?.bytes();
    streams.next().is_none().then_some(bytes)
}

impl Document {
    pub fn figure_shape_candidates(&self) -> Vec<NativeFigureShapeCandidate> {
        native_figure_bindings(self).unwrap_or_default()
    }
}

pub(super) fn native_figure_bindings(
    document: &Document,
) -> Option<Vec<NativeFigureShapeCandidate>> {
    if !document.blocks().is_empty()
        || !document.object_embedding_frames().is_empty()
        || document
            .unknown_styles()
            .iter()
            .any(|s| s.name() == Some(PAGE_LAYOUT_STYLE_PATH))
    {
        return None;
    }
    let frames = document.object_frame_records();
    if frames.len() != 3 {
        return None;
    }
    let frame_bytes = raw(document, "/Frame")?;
    if frame_bytes.len() != 196
        || frame_bytes.get(..16)? != [0, 1, 0, 4, 0, 2, 0, 1, 1, 1, 0, 4, 0, 0, 0, 3]
    {
        return None;
    }
    let figure = raw(document, "/Figure")?;
    if figure.len() != 56 || figure.get(..8)? != [0, 1, 0, 4, 0, 2, 0, 1] {
        return None;
    }
    let mut figure_ids = Vec::new();
    for i in 0..3 {
        let row = &figure[8 + i * 16..24 + i * 16];
        let id = read_be16_at(row, 12)?;
        let expected = [0x601_u16, 12, 0, 0, 0, 1, id, 0x100];
        if row
            .as_chunks::<2>()
            .0
            .iter()
            .zip(expected)
            .any(|(w, v)| u16::from_be_bytes(*w) != v)
        {
            return None;
        }
        if id > 2 || figure_ids.contains(&id) {
            return None;
        }
        figure_ids.push(id);
    }
    let vectors = document
        .object_stream_candidates()
        .iter()
        .filter(|s| s.path() == "/FigureData/main_data/FDMVector")
        .collect::<Vec<_>>();
    let [vector] = vectors.as_slice() else {
        return None;
    };
    let bytes = raw(document, vector.path())?;
    let indices = vector.fdm_index_entry_candidates();
    let commands = vector.fdm_raw_vector_commands();
    if indices.len() != 3 || commands.len() != 3 || !vector.image_payload_spans().is_empty() {
        return None;
    }
    let alpha = raw(document, "/FigureData/main_data/AlphaBlend")?;
    let alpha_words = alpha.as_chunks::<4>();
    if !alpha_words.1.is_empty() {
        return None;
    }
    let alpha_values = alpha_words
        .0
        .iter()
        .map(|w| u32::from_be_bytes(*w))
        .collect::<Vec<_>>();
    let fills = match alpha_values.as_slice() {
        [16, 1, 0, 0] => 0,
        [
            64,
            1,
            2,
            0,
            1,
            0,
            0xffff_ffff,
            0,
            0,
            0,
            1,
            1,
            0xffff_ffff,
            0,
            0,
            0,
        ] => 2,
        _ => return None,
    };
    let flow = document.document_text_flow()?;
    let intervals = shanai_lan_line_mark_intervals(document);
    if intervals.len() != 5
        || intervals.first()?.unit_start != flow.source_span().unit_start()
        || intervals.last()?.unit_end != flow.source_span().unit_end().checked_add(1)?
    {
        return None;
    }
    let records = flow
        .events()
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind() == DocumentTextFlowKind::Record)
        .collect::<Vec<_>>();
    if records.len() != 3
        || flow
            .events()
            .iter()
            .filter(|e| e.kind() == DocumentTextFlowKind::Text)
            .any(|e| e.text().chars().any(|c| c != '\n'))
    {
        return None;
    }
    let mut anchors = BTreeMap::new();
    let source_text = document_text_raw_stream(document)?;
    for (at, record) in records {
        let words = record.raw_words();
        if words.len() != 14
            || words[..8] != [0x1c, 0, 14, 0, 0x30, 0xffff, 0x507, 0x12]
            || words[9..] != [0, 14, 0, 0, 0x1f]
        {
            return None;
        }
        let id = words[8];
        if id > 2 {
            return None;
        }
        let group = flow.events().get(at + 1..at + 5)?;
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
            || suffix.kind() != DocumentTextFlowKind::Opaque
            || suffix.raw_words() != [5, 0, 1, 0x1f]
            || record.unit_end() != start.unit_start()
            || !group
                .windows(2)
                .all(|p| p[0].unit_end() == p[1].unit_start())
            || source_text.get(value.source_span().byte_start()..value.source_span().byte_end())?
                != [0, 0x1d, 0, 2, 0, 0x1e]
        {
            return None;
        }
        let interval = intervals
            .iter()
            .find(|l| l.unit_start == record.unit_start())?;
        if interval.flag_word != 0x8002
            || suffix.unit_end() > interval.unit_end
            || anchors
                .insert(id, (interval.record_index, record.source_span().clone()))
                .is_some()
        {
            return None;
        }
    }
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();
    let mut filled = 0;
    for (frame_index, frame) in frames.iter().enumerate() {
        let row = frame.raw_bytes();
        let profile = [
            0x102_u16, 56, 0, 0, 4, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            1, 0x4000, 0, 0,
        ];
        if row.len() != 60
            || frame.source_path() != "/Frame"
            || frame.row_index() != frame_index
            || frame.row_start() != 16 + frame_index * 60
            || row
                .as_chunks::<2>()
                .0
                .iter()
                .zip(profile)
                .enumerate()
                .any(|(i, (w, v))| {
                    !matches!(i, 3 | 6 | 14 | 16 | 18 | 20) && u16::from_be_bytes(*w) != v
                })
        {
            return None;
        }
        let id = frame.object_id();
        if id > 2 || !seen.insert(id) {
            return None;
        }
        let slot = usize::from(frame.object_type()).checked_sub(1)?;
        if figure_ids.get(slot) != Some(&id) {
            return None;
        }
        let index = indices.get(slot)?;
        if index.row_index() != slot
            || !index.valid_vector_offset()
            || index.vector_path() != vector.path()
            || index.index_path() != "/FigureData/main_data/FDMIndex"
        {
            return None;
        }
        let from = index.vector_offset();
        let to = index.next_vector_offset();
        let segment = bytes.get(from..to)?;
        if index.kind() != u16::from(*segment.get(2)?) << 8 {
            return None;
        }
        if segment.len() != index.vector_len()
            || read_be16_at(segment, 4)? as usize != segment.len()
        {
            return None;
        }
        let members = commands
            .iter()
            .filter(|c| {
                c.source_vector_relative_offset()
                    .is_some_and(|o| from <= o && o < to)
            })
            .collect::<Vec<_>>();
        let [command] = members.as_slice() else {
            return None;
        };
        let offset = command.source_vector_relative_offset()?;
        let child = bytes.get(offset..offset.checked_add(command.record_len())?)?;
        if offset + child.len() != to
            || read_be16_at(child, 4)? as usize != child.len()
            || read_be16_at(child, 8)? != 8
            || read_be32_at(child, 10)? != 0
        {
            return None;
        }
        let fill = if offset == from {
            if child.get(..2)? != [1, 0] {
                return None;
            }
            None
        } else {
            if offset - from != 46
                || segment.get(..4)? != [1, 0, 0x0a, 0x60]
                || read_be16_at(segment, 6)? != 1
                || segment.get(8..16)?.iter().any(|b| *b != 0)
                || read_be16_at(segment, 16)? != 0x0800
                || read_be16_at(segment, 18)? != 0
                || read_be32_at(segment, 40)? != 0x00ff_ffff
                || read_be16_at(segment, 44)? != 46
                || child.get(..2)? != [0xff, 0]
            {
                return None;
            }
            let bgr = read_be32_at(segment, 36)?;
            if bgr > 0xff_ffff {
                return None;
            }
            filled += 1;
            Some(format!(
                "#{:02x}{:02x}{:02x}",
                bgr & 255,
                (bgr >> 8) & 255,
                (bgr >> 16) & 255
            ))
        };
        let b = index.bbox();
        // Index coordinates are stored as axis pairs x1,x2,y1,y2 in this profile.
        let bounds = [
            f64::from(b.left()),
            f64::from(b.right()),
            f64::from(b.top()),
            f64::from(b.bottom()),
        ];
        if bounds[2] <= bounds[0]
            || bounds[3] <= bounds[1]
            || frame.width() == 0
            || frame.height() == 0
        {
            return None;
        }
        if fill.is_some()
            && [20, 24, 28, 32]
                .into_iter()
                .zip(bounds)
                .any(|(offset, value)| {
                    read_i32_be_at(segment, offset).map(f64::from) != Some(value)
                })
        {
            return None;
        }
        let kind = match command.marker()[2] {
            6 if command.path_points().len() == 5
                && command.path_points().first() == command.path_points().last() =>
            {
                "rectangle"
            }
            4 if command.ellipse().is_some() => "ellipse",
            1 if command.path_points().len() == 2 && fill.is_none() => "line",
            _ => return None,
        };
        let points = if let Some(e) = command.ellipse() {
            vec![
                [f64::from(e.center().x()), f64::from(e.center().y())],
                [f64::from(e.radius_x()), f64::from(e.radius_y())],
            ]
        } else {
            command
                .path_points()
                .iter()
                .map(|p| [f64::from(p.x()), f64::from(p.y())])
                .collect()
        };
        if !command.curve_segments().is_empty() || points.iter().flatten().any(|n| !n.is_finite()) {
            return None;
        }
        if kind == "ellipse" {
            if points[1][0] <= 0.0
                || points[1][1] <= 0.0
                || points[0][0] - points[1][0] < bounds[0]
                || points[0][0] + points[1][0] > bounds[2]
                || points[0][1] - points[1][1] < bounds[1]
                || points[0][1] + points[1][1] > bounds[3]
            {
                return None;
            }
        } else {
            if points.iter().any(|p| {
                p[0] < bounds[0] || p[0] > bounds[2] || p[1] < bounds[1] || p[1] > bounds[3]
            }) {
                return None;
            }
            if kind == "rectangle"
                && !(points[0][1] == points[1][1]
                    && points[1][0] == points[2][0]
                    && points[2][1] == points[3][1]
                    && points[3][0] == points[0][0]
                    && points[0][0] < points[1][0]
                    && points[0][1] < points[3][1])
            {
                return None;
            }
        }
        let (line, span) = anchors.get(&id)?;
        result.push(NativeFigureShapeCandidate {
            object_id: id,
            slot,
            kind,
            frame: [frame.x(), frame.y(), frame.width(), frame.height()],
            line: *line,
            record_span: span.clone(),
            vector_offset: from,
            command_offset: offset,
            bounds,
            points,
            fill,
            stroke_units: 8,
        });
    }
    result.sort_by_key(|s| s.slot);
    if filled != fills
        || result.iter().map(|s| s.slot).collect::<Vec<_>>() != [0, 1, 2]
        || result.iter().map(|s| s.kind).collect::<BTreeSet<_>>()
            != BTreeSet::from(["rectangle", "ellipse", "line"])
    {
        return None;
    }
    Some(result)
}
