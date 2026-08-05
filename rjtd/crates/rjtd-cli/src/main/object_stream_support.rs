use std::collections::{BTreeMap, BTreeSet};

use rjtd_model::{
    Document, ObjectFdmIndexBbox, ObjectFdmIndexEntryCandidate, ObjectFdmVectorCommandCandidate,
    ObjectFrameRecordCandidate, ObjectFrameReferenceRowCandidate, ObjectImagePayloadSpan,
    ObjectImageSignatureHit, ObjectStreamCandidate as ModelObjectStreamCandidate, parse_document,
};

use super::object_fdm_support::*;
use super::support::*;
use super::text_position_count_support::read_be16_fields;

pub(crate) const SO_RECORD_MARKER: &[u8] = b"SO\0\0";
pub(crate) const SO_RECORD_BYTES: usize = 36;
pub(crate) const SO_RECORD_DWORDS: usize = SO_RECORD_BYTES / 4;
pub(crate) const OBJECT_STREAM_PREFIX_PREVIEW_BYTES: usize = 16;
pub(crate) const OBJECT_STREAM_MAX_REPORTED_HITS: usize = 6;
pub(crate) const VISUAL_LIST_MAGIC_OFFSET: usize = 4;
pub(crate) const VISUAL_LIST_MAGIC: &[u8; 4] = b"BMDV";
pub(crate) const VISUAL_LIST_WIDTH_OFFSET: usize = 0x1c;
pub(crate) const VISUAL_LIST_HEIGHT_OFFSET: usize = 0x20;
pub(crate) const VISUAL_LIST_BIT_DEPTH_OFFSET: usize = 0x2c;
pub(crate) const VISUAL_LIST_RLE_LENGTH_OFFSET: usize = 0x4c;
pub(crate) const EMBEDDED_PRESS_SNAPSHOT_MAGIC: &[u8; 12] = b"JSSnapShot32";
pub(crate) const EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET: usize = 0x24;
pub(crate) const EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET: usize = 0x34;
pub(crate) const EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET: usize = 0x48;
pub(crate) const EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET: usize = 0x4c;
pub(crate) const JSFART2_CONTENTS_MAGIC_UTF16LE: &[u8; 22] = b"M\0S\0T\0U\0D\0I\0O\0.\0O\0C\0X\0";
pub(crate) const JSEQ3_CONTENTS_MAGIC_UTF16LE: &[u8; 16] = b"M\0A\0T\0H\0.\0V\0A\0F\0";
pub(crate) const JSEQ3_SO_TRAILER_BYTES: usize = 64;
pub(crate) const JSEQ3_SO_FIELD_BYTES: usize = 4;
pub(crate) const JSEQ3_SO_FIELD_COUNT: usize = 9;
pub(crate) const JSEQ3_TEXT_MARKERS: &[&str] =
    &["Times New Roman", "JustUnitMark", "JustOubunMark"];
pub(crate) const OBJECT_REFERENCE_CONTEXT_BEFORE_BYTES: usize = 8;
pub(crate) const OBJECT_REFERENCE_CONTEXT_AFTER_BYTES: usize = 8;
pub(crate) const OBJECT_REFERENCE_FIELD_STRIDES: &[usize] = &[
    4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 68, 72, 80, 84,
];
pub(crate) const OBJECT_FRAME_REFERENCE_RECORD_CANDIDATES:
    &[ObjectFrameReferenceRecordCandidate] = &[
    ObjectFrameReferenceRecordCandidate {
        encoding: "u16-le",
        stride: 12,
        field_offset: 5,
    },
    ObjectFrameReferenceRecordCandidate {
        encoding: "u16-be",
        stride: 12,
        field_offset: 7,
    },
    ObjectFrameReferenceRecordCandidate {
        encoding: "u16-be",
        stride: 20,
        field_offset: 15,
    },
];

pub(crate) struct ObjectStreamCandidate {
    pub(crate) path: String,
    pub(crate) size: usize,
    pub(crate) reasons: Vec<&'static str>,
    pub(crate) image_signature_hits: Vec<ObjectSignatureHit>,
    pub(crate) svg_offsets: Vec<usize>,
    pub(crate) so_offsets: Vec<usize>,
    pub(crate) visual_list: Option<CliVisualListCandidate>,
    pub(crate) embedded_press_snapshot: Option<CliEmbeddedPressSnapshotCandidate>,
    pub(crate) jseq3_formula: Option<CliJseq3FormulaCandidate>,
    pub(crate) jsfart_stream_profile: Option<CliJsfartStreamProfileCandidate>,
    pub(crate) prefix_hex: String,
}

pub(crate) struct CliVisualListCandidate {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) bit_depth: u32,
    pub(crate) rle_data_len: usize,
}

pub(crate) struct CliEmbeddedPressSnapshotCandidate {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) body_length_candidate: u32,
    pub(crate) object_count_candidate: u32,
}

pub(crate) struct CliJseq3FormulaCandidate {
    pub(crate) so_trailer_offset: Option<usize>,
    pub(crate) so_trailer_fields: Vec<u32>,
    pub(crate) text_markers: Vec<CliJseq3TextMarkerCandidate>,
}

pub(crate) struct CliJseq3TextMarkerCandidate {
    pub(crate) text: &'static str,
    pub(crate) offset: usize,
}

pub(crate) struct CliJsfartStreamProfileCandidate {
    pub(crate) magic_family: &'static str,
    pub(crate) magic_family_hex: String,
    pub(crate) magic_ascii_or_utf16_preview: String,
    pub(crate) structured_art_candidate_present: bool,
    pub(crate) render_promotion_blocked_reason: &'static str,
}

pub(crate) struct ObjectSignatureHit {
    pub(crate) kind: &'static str,
    pub(crate) offset: usize,
}

pub(crate) fn classify_object_stream_candidate(
    path: &str,
    stream: &[u8],
) -> Option<ObjectStreamCandidate> {
    let mut reasons = Vec::new();
    push_object_path_reasons(path, &mut reasons);

    let image_signature_hits = image_signature_hits(stream);
    if !image_signature_hits.is_empty() {
        push_unique_reason(&mut reasons, "image-signature");
    }

    let svg_offsets = svg_signature_offsets(stream);
    if !svg_offsets.is_empty() {
        push_unique_reason(&mut reasons, "svg-signature");
    }

    let so_offsets = find_subslice_offsets(stream, SO_RECORD_MARKER);
    if !so_offsets.is_empty() {
        push_unique_reason(&mut reasons, "so-marker");
    }
    let visual_list = visual_list_candidate_from_stream(path, stream);
    let embedded_press_snapshot = embedded_press_snapshot_candidate_from_stream(stream);
    let jseq3_formula = jseq3_formula_candidate_from_stream(path, stream);
    let jsfart_stream_profile = jsfart_stream_profile_candidate_from_stream(path, stream);
    if figure_link_candidate_from_stream(path, stream) {
        push_unique_reason(&mut reasons, "figure-link");
    }
    if embedded_press_snapshot.is_some() {
        push_unique_reason(&mut reasons, "embedded-press-snapshot");
    }
    if jseq3_formula.is_some() {
        push_unique_reason(&mut reasons, "jseq3-formula");
    }

    if reasons.is_empty() {
        return None;
    }

    Some(ObjectStreamCandidate {
        path: path.to_string(),
        size: stream.len(),
        reasons,
        image_signature_hits,
        svg_offsets,
        so_offsets,
        visual_list,
        embedded_press_snapshot,
        jseq3_formula,
        jsfart_stream_profile,
        prefix_hex: format_hex_preview(stream, OBJECT_STREAM_PREFIX_PREVIEW_BYTES),
    })
}

pub(crate) fn figure_link_candidate_from_stream(path: &str, stream: &[u8]) -> bool {
    let lower = path.to_ascii_lowercase();
    if !lower.contains("/figuredata/") || !lower.ends_with("/link") {
        return false;
    }
    let header_bytes = 8usize;
    let row_bytes = 14usize;
    if stream.len() < header_bytes + row_bytes {
        return false;
    }
    let row_payload_len = stream.len().saturating_sub(header_bytes);
    if !row_payload_len.is_multiple_of(row_bytes) {
        return false;
    }
    let row_count = row_payload_len / row_bytes;
    if row_count == 0 || read_be16_candidate(stream, 6).map(usize::from) != Some(row_count) {
        return false;
    }
    (0..row_count).all(|row_index| {
        let row_start = header_bytes + row_index * row_bytes;
        read_be16_candidate(stream, row_start + 8) == Some(0x0016)
    })
}

pub(crate) fn embedded_press_snapshot_candidate_from_stream(
    stream: &[u8],
) -> Option<CliEmbeddedPressSnapshotCandidate> {
    if stream.get(..EMBEDDED_PRESS_SNAPSHOT_MAGIC.len())? != EMBEDDED_PRESS_SNAPSHOT_MAGIC {
        return None;
    }
    let body_length_candidate =
        read_le32_candidate(stream, EMBEDDED_PRESS_SNAPSHOT_BODY_LENGTH_OFFSET)?;
    let object_count_candidate =
        read_le32_candidate(stream, EMBEDDED_PRESS_SNAPSHOT_OBJECT_COUNT_OFFSET)?;
    let width = read_le32_candidate(stream, EMBEDDED_PRESS_SNAPSHOT_WIDTH_OFFSET)?;
    let height = read_le32_candidate(stream, EMBEDDED_PRESS_SNAPSHOT_HEIGHT_OFFSET)?;
    if body_length_candidate == 0 || width == 0 || height == 0 {
        return None;
    }
    Some(CliEmbeddedPressSnapshotCandidate {
        width,
        height,
        body_length_candidate,
        object_count_candidate,
    })
}

pub(crate) fn jseq3_formula_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<CliJseq3FormulaCandidate> {
    if !path.ends_with("/JSEQ3Contents") {
        return None;
    }
    if stream.get(..JSEQ3_CONTENTS_MAGIC_UTF16LE.len())? != JSEQ3_CONTENTS_MAGIC_UTF16LE {
        return None;
    }
    let so_trailer_offset = find_subslice_offsets(stream, SO_RECORD_MARKER)
        .into_iter()
        .find(|offset| {
            offset.saturating_add(JSEQ3_SO_FIELD_COUNT * JSEQ3_SO_FIELD_BYTES) <= stream.len()
                && offset.saturating_add(JSEQ3_SO_TRAILER_BYTES) >= stream.len()
        });
    let so_trailer_fields = so_trailer_offset
        .and_then(|offset| stream.get(offset..))
        .map(jseq3_so_trailer_fields)
        .unwrap_or_default();
    Some(CliJseq3FormulaCandidate {
        so_trailer_offset,
        so_trailer_fields,
        text_markers: jseq3_text_marker_candidates(stream),
    })
}

pub(crate) fn jseq3_so_trailer_fields(trailer: &[u8]) -> Vec<u32> {
    (0..JSEQ3_SO_FIELD_COUNT)
        .filter_map(|index| read_le32_candidate(trailer, index * JSEQ3_SO_FIELD_BYTES))
        .collect()
}

pub(crate) fn jseq3_text_marker_candidates(stream: &[u8]) -> Vec<CliJseq3TextMarkerCandidate> {
    let mut candidates = Vec::new();
    for marker in JSEQ3_TEXT_MARKERS {
        let encoded = utf16le_bytes(marker);
        for offset in find_subslice_offsets(stream, &encoded) {
            candidates.push(CliJseq3TextMarkerCandidate {
                text: marker,
                offset,
            });
        }
    }
    candidates.sort_by_key(|candidate| candidate.offset);
    candidates
}

pub(crate) fn jsfart_stream_profile_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<CliJsfartStreamProfileCandidate> {
    if !path.ends_with("/JSFart2Contents") {
        return None;
    }
    let header_prefix = &stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)];
    let preview = utf16le_printable_preview(header_prefix);
    let structured_art_candidate_present = stream.starts_with(JSFART2_CONTENTS_MAGIC_UTF16LE);
    let render_promotion_blocked_reason = if structured_art_candidate_present {
        "structured-jsfart-art-still-paint-authority-unproven"
    } else {
        "jsfart-variant-layout-undecoded"
    };
    Some(CliJsfartStreamProfileCandidate {
        magic_family: jsfart_stream_magic_family(stream, &preview),
        magic_family_hex: format_hex_preview(&stream[..stream.len().min(2)], 2),
        magic_ascii_or_utf16_preview: preview,
        structured_art_candidate_present,
        render_promotion_blocked_reason,
    })
}

pub(crate) fn jsfart_stream_magic_family(stream: &[u8], utf16le_preview: &str) -> &'static str {
    if stream.starts_with(JSFART2_CONTENTS_MAGIC_UTF16LE) {
        "mstudio-ocx-utf16le"
    } else if utf16le_preview.starts_with("JSFART.") {
        "jsfart-object-utf16le"
    } else if stream.get(..2).is_some_and(|prefix| prefix == [0x00, 0x00]) {
        "zero-prefix"
    } else if !utf16le_preview.is_empty() {
        "utf16le-text-prefix"
    } else {
        "binary-prefix"
    }
}

pub(crate) fn utf16le_printable_preview(bytes: &[u8]) -> String {
    let mut preview = String::new();
    for chunk in bytes.chunks_exact(2) {
        let value = u16::from_le_bytes([chunk[0], chunk[1]]);
        if value == 0 {
            break;
        }
        let Some(character) = char::from_u32(u32::from(value)) else {
            break;
        };
        if character.is_control() {
            break;
        }
        preview.push(character);
    }
    preview
}

pub(crate) fn utf16le_bytes(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for unit in text.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

pub(crate) fn visual_list_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<CliVisualListCandidate> {
    if !path.to_ascii_lowercase().contains("visuallist") {
        return None;
    }
    if stream.get(VISUAL_LIST_MAGIC_OFFSET..VISUAL_LIST_MAGIC_OFFSET + VISUAL_LIST_MAGIC.len())?
        != VISUAL_LIST_MAGIC
    {
        return None;
    }
    Some(CliVisualListCandidate {
        width: read_be32_at(stream, VISUAL_LIST_WIDTH_OFFSET)?,
        height: read_be32_at(stream, VISUAL_LIST_HEIGHT_OFFSET)?,
        bit_depth: read_be32_at(stream, VISUAL_LIST_BIT_DEPTH_OFFSET)?,
        rle_data_len: read_be32_at(stream, VISUAL_LIST_RLE_LENGTH_OFFSET)? as usize,
    })
}

pub(crate) fn format_embedded_press_snapshot_candidate(
    candidate: Option<&CliEmbeddedPressSnapshotCandidate>,
) -> String {
    candidate
        .map(|candidate| {
            format!(
                "JSSnapShot32,{}x{},body={},objects={}",
                candidate.width,
                candidate.height,
                candidate.body_length_candidate,
                candidate.object_count_candidate
            )
        })
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_jseq3_formula_candidate(
    candidate: Option<&CliJseq3FormulaCandidate>,
) -> String {
    let Some(candidate) = candidate else {
        return "-".to_string();
    };
    let fields = if candidate.so_trailer_fields.is_empty() {
        "-".to_string()
    } else {
        candidate
            .so_trailer_fields
            .iter()
            .map(|field| format!("0x{field:08x}"))
            .collect::<Vec<_>>()
            .join(",")
    };
    let markers = if candidate.text_markers.is_empty() {
        "-".to_string()
    } else {
        candidate
            .text_markers
            .iter()
            .map(|marker| format!("{}@{}", marker.text, marker.offset))
            .collect::<Vec<_>>()
            .join(",")
    };
    format!(
        "MATH.VAF,so={},fields={},markers={}",
        format_optional_usize(candidate.so_trailer_offset),
        fields,
        markers
    )
}

pub(crate) fn format_jsfart_stream_profile_candidate(
    candidate: Option<&CliJsfartStreamProfileCandidate>,
) -> String {
    let Some(candidate) = candidate else {
        return "-".to_string();
    };
    format!(
        "{},hex={},preview={},structured-art={},blocked={}",
        candidate.magic_family,
        candidate.magic_family_hex,
        escaped_text(&candidate.magic_ascii_or_utf16_preview),
        candidate.structured_art_candidate_present,
        candidate.render_promotion_blocked_reason
    )
}

pub(crate) fn format_visual_list_candidate(candidate: Option<&CliVisualListCandidate>) -> String {
    candidate
        .map(|candidate| {
            format!(
                "{}x{}x{}bpp,rle={}",
                candidate.width, candidate.height, candidate.bit_depth, candidate.rle_data_len
            )
        })
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn push_object_path_reasons(path: &str, reasons: &mut Vec<&'static str>) {
    let lower = path.to_ascii_lowercase();
    let segments = lower
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &[
                "embeditems",
                "embedding",
                "jsfart",
                "compobj",
                "ole",
                "object",
                "bin",
            ],
        )
    }) {
        push_unique_reason(reasons, "object-path");
    }

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &[
                "image", "picture", "graphic", "bitmap", "png", "jpg", "jpeg", "gif", "bmp", "tif",
                "tiff", "wmf", "emf",
            ],
        )
    }) {
        push_unique_reason(reasons, "image-path");
    }

    if segments.iter().any(|segment| {
        contains_any(
            segment,
            &["figure", "shape", "draw", "frame", "layoutbox", "svg"],
        )
    }) {
        push_unique_reason(reasons, "shape-path");
    }

    if segments.iter().any(|segment| {
        contains_any(segment, &["table", "cell", "tbl", "hyo"])
            && !contains_any(segment, &["positiontable", "style"])
    }) {
        push_unique_reason(reasons, "table-path");
    }

    if segments.contains(&"visuallist") {
        push_unique_reason(reasons, "visual-list-path");
    }
}

pub(crate) fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

pub(crate) fn push_unique_reason(reasons: &mut Vec<&'static str>, reason: &'static str) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

pub(crate) fn image_signature_hits(stream: &[u8]) -> Vec<ObjectSignatureHit> {
    let mut hits = Vec::new();
    push_signature_hits(&mut hits, stream, "png", b"\x89PNG\r\n\x1a\n", true);
    push_signature_hits(&mut hits, stream, "jpeg", b"\xff\xd8\xff", true);
    push_signature_hits(&mut hits, stream, "gif87a", b"GIF87a", true);
    push_signature_hits(&mut hits, stream, "gif89a", b"GIF89a", true);
    push_signature_hits(&mut hits, stream, "tiff-le", b"II\x2a\0", true);
    push_signature_hits(&mut hits, stream, "tiff-be", b"MM\0\x2a", true);
    push_signature_hits(
        &mut hits,
        stream,
        "wmf-placeable",
        b"\xd7\xcd\xc6\x9a",
        true,
    );
    push_signature_hits(&mut hits, stream, "bmp", b"BM", false);

    hits.sort_by(|left, right| {
        left.offset
            .cmp(&right.offset)
            .then_with(|| left.kind.cmp(right.kind))
    });
    hits
}

pub(crate) fn push_signature_hits(
    hits: &mut Vec<ObjectSignatureHit>,
    stream: &[u8],
    kind: &'static str,
    signature: &[u8],
    scan_anywhere: bool,
) {
    let offsets = if scan_anywhere {
        find_subslice_offsets(stream, signature)
    } else if stream.starts_with(signature) {
        vec![0]
    } else {
        Vec::new()
    };

    for offset in offsets {
        hits.push(ObjectSignatureHit { kind, offset });
    }
}

pub(crate) fn svg_signature_offsets(stream: &[u8]) -> Vec<usize> {
    let ascii_lower = stream
        .iter()
        .map(|byte| byte.to_ascii_lowercase())
        .collect::<Vec<_>>();
    find_subslice_offsets(&ascii_lower, b"<svg")
}

pub(crate) fn object_stream_reason_count(
    reason_counts: &BTreeMap<&'static str, usize>,
    reason: &'static str,
) -> usize {
    reason_counts.get(reason).copied().unwrap_or_default()
}

pub(crate) fn format_object_signature_hits(hits: &[ObjectSignatureHit]) -> String {
    if hits.is_empty() {
        return "-".to_string();
    }

    let mut values = hits
        .iter()
        .take(OBJECT_STREAM_MAX_REPORTED_HITS)
        .map(|hit| format!("{}@{}", hit.kind, hit.offset))
        .collect::<Vec<_>>();
    if hits.len() > OBJECT_STREAM_MAX_REPORTED_HITS {
        values.push(format!("+{}", hits.len() - OBJECT_STREAM_MAX_REPORTED_HITS));
    }
    values.join(",")
}

pub(crate) fn format_usize_hit_list(offsets: &[usize]) -> String {
    if offsets.is_empty() {
        return "-".to_string();
    }

    let mut values = offsets
        .iter()
        .take(OBJECT_STREAM_MAX_REPORTED_HITS)
        .map(usize::to_string)
        .collect::<Vec<_>>();
    if offsets.len() > OBJECT_STREAM_MAX_REPORTED_HITS {
        values.push(format!(
            "+{}",
            offsets.len() - OBJECT_STREAM_MAX_REPORTED_HITS
        ));
    }
    values.join(",")
}

pub(crate) fn fdm_raw_vector_commands_for_path<'a>(
    document: &'a Document,
    vector_path: &str,
) -> Option<&'a [ObjectFdmVectorCommandCandidate]> {
    document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == vector_path)
        .map(ModelObjectStreamCandidate::fdm_raw_vector_commands)
}

pub(crate) fn fdm_index_offset_field_reference_summaries(
    entry: &FdmIndexEntry,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) -> Vec<String> {
    let fields = [
        Some(("vectorOffset", entry.vector_offset)),
        non_negative_fdm_index_offset_field("bbox.left", entry.left),
        non_negative_fdm_index_offset_field("bbox.top", entry.top),
        non_negative_fdm_index_offset_field("bbox.right", entry.right),
        non_negative_fdm_index_offset_field("bbox.bottom", entry.bottom),
    ];

    let mut references = Vec::new();
    for (field_name, field_value) in fields.into_iter().flatten() {
        let command_matches = raw_commands
            .iter()
            .filter(|command| command.relative_offset() == field_value)
            .map(ObjectFdmVectorCommandCandidate::relative_offset)
            .collect::<Vec<_>>();
        if !command_matches.is_empty() {
            references.push(format!(
                "{}:command:{}{}",
                field_name,
                field_value,
                format_offset_field_ref_match_suffix(field_value, &command_matches)
            ));
        }

        let segment_matches = raw_commands
            .iter()
            .filter(|command| {
                command
                    .source_segment()
                    .is_some_and(|segment| segment.relative_offset() == field_value)
            })
            .map(ObjectFdmVectorCommandCandidate::relative_offset)
            .collect::<Vec<_>>();
        if !segment_matches.is_empty() {
            references.push(format!(
                "{}:segment:{}->[{}]",
                field_name,
                field_value,
                format_usize_hit_list(&segment_matches)
            ));
        }
    }
    references
}

pub(crate) fn non_negative_fdm_index_offset_field(
    field_name: &'static str,
    value: i32,
) -> Option<(&'static str, usize)> {
    (value >= 0).then_some((field_name, value as usize))
}

pub(crate) fn format_offset_field_ref_match_suffix(
    field_value: usize,
    matches: &[usize],
) -> String {
    if matches.len() == 1 && matches[0] == field_value {
        String::new()
    } else {
        format!("->[{}]", format_usize_hit_list(matches))
    }
}

pub(crate) fn format_offset_field_refs(references: &[String]) -> String {
    if references.is_empty() {
        "-".to_string()
    } else {
        references.join(",")
    }
}

pub(crate) struct ObjectReferenceContext {
    pub(crate) start: usize,
    pub(crate) hex: String,
}

pub(crate) struct ObjectFrameReferenceRecordCandidate {
    pub(crate) encoding: &'static str,
    pub(crate) stride: usize,
    pub(crate) field_offset: usize,
}

impl ObjectFrameReferenceRecordCandidate {
    pub(crate) fn name(&self) -> String {
        format!("{}/{}/{}", self.encoding, self.stride, self.field_offset)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ObjectFrameReferenceRecord {
    pub(crate) source_path: String,
    pub(crate) embedding_index: Option<usize>,
    pub(crate) target_path: String,
    pub(crate) encoding: String,
    pub(crate) stride: usize,
    pub(crate) field_offset: usize,
    pub(crate) offset: usize,
    pub(crate) row_index: usize,
    pub(crate) row_start: usize,
    pub(crate) candidate: String,
    pub(crate) row: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ObjectFrameReferenceRecordCollection {
    pub(crate) source_count: usize,
    pub(crate) reference_count: usize,
    pub(crate) skipped_count: usize,
    pub(crate) records: Vec<ObjectFrameReferenceRecord>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ObjectFrameRecordFamilySummary {
    pub(crate) rows: usize,
    pub(crate) candidates: BTreeSet<String>,
    pub(crate) embedding_indexes: BTreeSet<usize>,
    pub(crate) examples: BTreeSet<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ObjectImageFrameCandidateSummary {
    pub(crate) embedding_index: Option<usize>,
    pub(crate) payload_kinds: BTreeSet<String>,
    pub(crate) frame_rows: usize,
    pub(crate) family_counts: BTreeMap<String, usize>,
    pub(crate) row12_tail_coordinate: usize,
    pub(crate) row12_tail_zero: usize,
    pub(crate) row20_tail_window: usize,
    pub(crate) row20_linked: usize,
    pub(crate) le_row12: usize,
    pub(crate) coordinate_pairs: Vec<ObjectFrameCoordinatePair>,
    pub(crate) preferred: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ObjectFrameCoordinatePair {
    pub(crate) row_start: usize,
    pub(crate) x: u16,
    pub(crate) y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FdmIndexEntry {
    pub(crate) row_index: usize,
    pub(crate) index_offset: usize,
    pub(crate) row: Vec<u8>,
    pub(crate) vector_offset: usize,
    pub(crate) kind: u16,
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
    pub(crate) valid_vector_offset: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FdmVectorSegment {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ObjectReferenceFieldKey {
    pub(crate) target_path: String,
    pub(crate) encoding: String,
    pub(crate) stride: usize,
    pub(crate) field_offset: usize,
}

impl ObjectReferenceFieldKey {
    pub(crate) fn new(
        target_path: &str,
        encoding: &str,
        stride: usize,
        field_offset: usize,
    ) -> Self {
        Self {
            target_path: target_path.to_string(),
            encoding: encoding.to_string(),
            stride,
            field_offset,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ObjectReferenceFieldSummary {
    pub(crate) matches: usize,
    pub(crate) cross_row_matches: usize,
    pub(crate) source_streams: BTreeSet<String>,
    pub(crate) embedding_indexes: BTreeSet<usize>,
    pub(crate) row_indexes: BTreeSet<usize>,
}

pub(crate) fn collect_object_frame_reference_records(
    data: &[u8],
) -> Result<ObjectFrameReferenceRecordCollection, String> {
    let document = parse_document(data).map_err(|error| error.to_string())?;
    let streams = readable_cfb_streams(data)?;
    let mut collection = ObjectFrameReferenceRecordCollection::default();

    for candidate in document.object_stream_candidates() {
        let embedding_index = candidate
            .ownership_candidate()
            .and_then(|ownership| ownership.embedding_index());
        let mut source_reported = false;

        for reference in candidate
            .ownership_reference_candidates()
            .iter()
            .filter(|reference| reference.target_path().eq_ignore_ascii_case("/Frame"))
        {
            collection.reference_count += 1;
            let Some(target_stream) = streams.get(reference.target_path()) else {
                collection.skipped_count += reference.offsets().len();
                continue;
            };

            for offset in reference.offsets() {
                let offset = *offset;
                for projection in
                    OBJECT_FRAME_REFERENCE_RECORD_CANDIDATES
                        .iter()
                        .filter(|projection| {
                            projection.encoding == reference.encoding()
                                && offset % projection.stride == projection.field_offset
                        })
                {
                    let pattern_len = object_reference_pattern_len(reference.encoding());
                    if projection.field_offset + pattern_len > projection.stride {
                        collection.skipped_count += 1;
                        continue;
                    }
                    let row_start = offset - projection.field_offset;
                    let Some(row_end) = row_start.checked_add(projection.stride) else {
                        collection.skipped_count += 1;
                        continue;
                    };
                    let Some(row) = target_stream.get(row_start..row_end) else {
                        collection.skipped_count += 1;
                        continue;
                    };

                    if !source_reported {
                        collection.source_count += 1;
                        source_reported = true;
                    }
                    collection.records.push(ObjectFrameReferenceRecord {
                        source_path: candidate.path().to_string(),
                        embedding_index,
                        target_path: reference.target_path().to_string(),
                        encoding: reference.encoding().to_string(),
                        stride: projection.stride,
                        field_offset: projection.field_offset,
                        offset,
                        row_index: offset / projection.stride,
                        row_start,
                        candidate: projection.name(),
                        row: row.to_vec(),
                    });
                }
            }
        }
    }

    Ok(collection)
}

pub(crate) fn summarize_object_image_frame_candidate(
    candidate: &ModelObjectStreamCandidate,
) -> ObjectImageFrameCandidateSummary {
    let mut summary = ObjectImageFrameCandidateSummary {
        embedding_index: candidate
            .ownership_candidate()
            .and_then(|ownership| ownership.embedding_index()),
        ..ObjectImageFrameCandidateSummary::default()
    };

    for span in candidate.image_payload_spans() {
        summary.payload_kinds.insert(span.kind().to_string());
    }

    for row in candidate.frame_reference_row_candidates() {
        summary.frame_rows += 1;
        *summary
            .family_counts
            .entry(row.family().to_string())
            .or_default() += 1;

        if row.encoding() == "u16-be"
            && row.stride() == 12
            && row.field_offset() == 7
            && row.family() == "frame-index-tail-coordinate-row12"
        {
            summary.row12_tail_coordinate += 1;
            if let Some(pair) = object_frame_coordinate_pair(row) {
                summary.coordinate_pairs.push(pair);
            }
        } else if row.encoding() == "u16-be"
            && row.stride() == 12
            && row.field_offset() == 7
            && row.family() == "frame-index-tail-zero-row12"
        {
            summary.row12_tail_zero += 1;
        } else if row.encoding() == "u16-be"
            && row.stride() == 20
            && row.field_offset() == 15
            && row.family() == "frame-index-tail-window20"
        {
            summary.row20_tail_window += 1;
            if row.suffix_link().is_some() {
                summary.row20_linked += 1;
            }
        } else if row.encoding() == "u16-le" && row.stride() == 12 && row.field_offset() == 5 {
            summary.le_row12 += 1;
        }
    }

    summary.preferred = preferred_object_image_frame_candidate(&summary);
    summary
}

pub(crate) fn preferred_object_image_frame_candidate(
    summary: &ObjectImageFrameCandidateSummary,
) -> &'static str {
    if summary.row12_tail_coordinate > 0 {
        "row12-tail-coordinate"
    } else if summary.row12_tail_zero > 0 {
        "row12-tail-zero"
    } else if summary.row20_tail_window > 0 {
        "row20-tail-window"
    } else if summary.le_row12 > 0 {
        "u16-le-row12"
    } else {
        "none"
    }
}

pub(crate) fn fdm_entry_complete_payload_count(
    candidate: &ModelObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> usize {
    fdm_entry_complete_payload_spans(candidate, entry).len()
}

pub(crate) fn fdm_entry_complete_payload_spans<'a>(
    candidate: &'a ModelObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> Vec<&'a ObjectImagePayloadSpan> {
    candidate
        .image_payload_spans()
        .iter()
        .filter(|span| {
            span.complete()
                && span.signature_offset() >= entry.vector_offset()
                && span.signature_offset() < entry.next_vector_offset()
        })
        .collect()
}

pub(crate) fn fdm_image_candidate_render_blocked_reason(
    entry: &ObjectFdmIndexEntryCandidate,
    complete_payload_count: usize,
) -> &'static str {
    if complete_payload_count > 0 {
        "page-placement-unproven"
    } else if entry.segment_image_signature_hits().is_empty() {
        "fdm-frame-image-payload-absent"
    } else {
        "image-signature-without-complete-payload-role-unproven"
    }
}

pub(crate) fn fdm_frame_record_for_entry(
    records: &[ObjectFrameRecordCandidate],
    row_index: usize,
) -> Option<&ObjectFrameRecordCandidate> {
    let object_id = u16::try_from(row_index).ok()?;
    records
        .iter()
        .find(|record| record.object_id() == object_id)
}

pub(crate) fn fdm_frame_link_render_blocked_reason(
    frame_record: Option<&ObjectFrameRecordCandidate>,
    entry: &ObjectFdmIndexEntryCandidate,
    complete_payload_spans: &[&ObjectImagePayloadSpan],
) -> &'static str {
    if frame_record.is_none() {
        "fdm-frame-record-missing"
    } else if complete_payload_spans.is_empty() {
        if entry.segment_image_signature_hits().is_empty() {
            "fdm-frame-image-payload-absent"
        } else {
            "image-signature-without-complete-payload-role-unproven"
        }
    } else {
        "fdm-frame-linked-image-payload-placement-and-paint-order-unproven"
    }
}

pub(crate) fn normalize_fdm_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.right()),
        bbox.top().min(bbox.bottom()),
        bbox.left().max(bbox.right()),
        bbox.top().max(bbox.bottom()),
    )
}

pub(crate) fn fdm_bbox_order(bbox: ObjectFdmIndexBbox) -> &'static str {
    match (bbox.left() <= bbox.right(), bbox.top() <= bbox.bottom()) {
        (true, true) => "forward",
        (false, true) => "inverted-x",
        (true, false) => "inverted-y",
        (false, false) => "inverted-xy",
    }
}

pub(crate) fn fdm_bbox_is_plausible(bbox: ObjectFdmIndexBbox) -> bool {
    let normalized = normalize_fdm_bbox(bbox);
    let width = normalized.2.saturating_sub(normalized.0);
    let height = normalized.3.saturating_sub(normalized.1);
    width > 0 && height > 0 && width <= 200_000 && height <= 200_000
}

pub(crate) fn format_model_object_signature_hits(hits: &[ObjectImageSignatureHit]) -> String {
    let hits = hits
        .iter()
        .map(|hit| format!("{}@{}", hit.kind(), hit.offset()))
        .collect::<Vec<_>>();
    if hits.is_empty() {
        "-".to_string()
    } else {
        hits.join(",")
    }
}

pub(crate) fn format_optional_frame_geometry(
    record: Option<&ObjectFrameRecordCandidate>,
) -> String {
    record
        .map(|record| {
            format!(
                "{},{},{},{}",
                record.x(),
                record.y(),
                record.width(),
                record.height()
            )
        })
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_frame_size(record: Option<&ObjectFrameRecordCandidate>) -> String {
    record
        .map(|record| format!("{}x{}", record.width(), record.height()))
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_fdm_payload_dimensions(spans: &[&ObjectImagePayloadSpan]) -> String {
    let dimensions = spans
        .iter()
        .filter_map(|span| {
            let dimensions = span.dimensions()?;
            Some(format!(
                "{}@{}:{}x{}",
                span.kind(),
                span.signature_offset(),
                dimensions.width(),
                dimensions.height()
            ))
        })
        .collect::<Vec<_>>();
    if dimensions.is_empty() {
        "-".to_string()
    } else {
        dimensions.join(",")
    }
}

pub(crate) fn fdm_payload_dimension_count(spans: &[&ObjectImagePayloadSpan]) -> usize {
    spans
        .iter()
        .filter(|span| span.dimensions().is_some())
        .count()
}

pub(crate) fn best_frame_payload_aspect_delta_permille(
    frame_record: Option<&ObjectFrameRecordCandidate>,
    spans: &[&ObjectImagePayloadSpan],
) -> Option<u64> {
    let frame_record = frame_record?;
    let frame_width = u128::from(frame_record.width());
    let frame_height = u128::from(frame_record.height());
    if frame_width == 0 || frame_height == 0 {
        return None;
    }

    spans
        .iter()
        .filter_map(|span| {
            let dimensions = span.dimensions()?;
            aspect_delta_permille(
                frame_width,
                frame_height,
                u128::from(dimensions.width()),
                u128::from(dimensions.height()),
            )
        })
        .min()
}

pub(crate) fn object_frame_coordinate_pair(
    row: &ObjectFrameReferenceRowCandidate,
) -> Option<ObjectFrameCoordinatePair> {
    let be16 = read_be16_fields(row.row());
    Some(ObjectFrameCoordinatePair {
        row_start: row.row_start(),
        x: *be16.get(2)?,
        y: *be16.get(4)?,
    })
}

pub(crate) fn format_object_frame_coordinate_pairs(pairs: &[ObjectFrameCoordinatePair]) -> String {
    if pairs.is_empty() {
        return "-".to_string();
    }

    pairs
        .iter()
        .take(OBJECT_STREAM_MAX_REPORTED_HITS)
        .map(|pair| format!("{}:{}x{}", pair.row_start, pair.x, pair.y))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_object_payload_dimensions(spans: &[ObjectImagePayloadSpan]) -> String {
    let dimensions = spans
        .iter()
        .filter_map(|span| {
            let dimensions = span.dimensions()?;
            Some(format!(
                "{}@{}:{}x{}",
                span.kind(),
                span.signature_offset(),
                dimensions.width(),
                dimensions.height()
            ))
        })
        .collect::<Vec<_>>();
    if dimensions.is_empty() {
        "-".to_string()
    } else {
        dimensions.join(",")
    }
}

pub(crate) fn object_payload_dimension_count(spans: &[ObjectImagePayloadSpan]) -> usize {
    spans
        .iter()
        .filter(|span| span.dimensions().is_some())
        .count()
}

pub(crate) fn coordinate_payload_aspect_candidate_count(
    pairs: &[ObjectFrameCoordinatePair],
    spans: &[ObjectImagePayloadSpan],
) -> usize {
    let dimensioned_payloads = object_payload_dimension_count(spans);
    let nonzero_pairs = pairs
        .iter()
        .filter(|pair| pair.x != 0 && pair.y != 0)
        .count();
    dimensioned_payloads.saturating_mul(nonzero_pairs)
}

pub(crate) fn best_coordinate_payload_aspect_delta_permille(
    pairs: &[ObjectFrameCoordinatePair],
    spans: &[ObjectImagePayloadSpan],
) -> Option<u64> {
    pairs
        .iter()
        .filter(|pair| pair.x != 0 && pair.y != 0)
        .flat_map(|pair| {
            spans.iter().filter_map(move |span| {
                let dimensions = span.dimensions()?;
                aspect_delta_permille(
                    u128::from(pair.x),
                    u128::from(pair.y),
                    u128::from(dimensions.width()),
                    u128::from(dimensions.height()),
                )
            })
        })
        .min()
}

pub(crate) fn aspect_delta_permille(
    frame_width: u128,
    frame_height: u128,
    image_width: u128,
    image_height: u128,
) -> Option<u64> {
    if frame_width == 0 || frame_height == 0 || image_width == 0 || image_height == 0 {
        return None;
    }
    let left = frame_width.saturating_mul(image_height);
    let right = image_width.saturating_mul(frame_height);
    let denominator = left.max(right);
    if denominator == 0 {
        return None;
    }
    let delta = left.abs_diff(right);
    Some(((delta.saturating_mul(1000)) / denominator) as u64)
}

pub(crate) fn format_le32_fields(bytes: &[u8], max_fields: usize) -> String {
    let fields = le32_dwords(bytes)
        .take(max_fields)
        .map(|value| format!("0x{value:08x}"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        "-".to_string()
    } else {
        fields.join(",")
    }
}

pub(crate) fn format_be32_fields(bytes: &[u8]) -> String {
    let fields = be32_dwords(bytes)
        .map(|value| format!("0x{value:08x}"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        "-".to_string()
    } else {
        fields.join(",")
    }
}

pub(crate) fn classify_so_geometry_fields(fields: &[u32]) -> &'static str {
    if fields.len() < 5 {
        return "truncated";
    }

    let values = &fields[1..5];
    if is_jseq3_like_packed_so(fields) {
        return "packed-jseq3-like";
    }

    if is_ffff_preamble_so(fields) {
        return "packed-ffff-preamble";
    }

    if values.iter().any(|value| value >> 16 != 0) {
        return "packed";
    }

    if values == [7, 0x100, 0, 0x64] || values == [0x100, 0, 0x64, 0] {
        return "default-control";
    }

    if values.iter().any(|value| *value > 0x100) {
        return "geometry-like";
    }

    "unknown"
}

pub(crate) fn is_jseq3_like_packed_so(fields: &[u32]) -> bool {
    fields.len() >= 8
        && fields[4] == 0
        && fields[5] == 0
        && fields[6] == (fields[2] & 0xffff)
        && fields[7] != 0
        && fields[1] >> 16 != 0
        && fields[2] >> 16 != 0
        && fields[3] >> 16 != 0
}

pub(crate) fn is_ffff_preamble_so(fields: &[u32]) -> bool {
    fields.len() >= SO_RECORD_DWORDS
        && fields[1] >> 16 != 0
        && fields[2] >> 16 != 0
        && fields[3] == 0x0000ffff
        && fields[4..SO_RECORD_DWORDS].iter().all(|field| *field == 0)
}

pub(crate) fn format_so_geometry_candidate(
    fields: &[u32],
) -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
) {
    if fields.len() < 5 {
        let empty = "-".to_string();
        return (
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty,
        );
    }

    let f1 = fields[1];
    let f2 = fields[2];
    let f3 = fields[3];
    let f4 = fields[4];
    (
        f1.to_string(),
        f2.to_string(),
        f3.to_string(),
        f4.to_string(),
        (f3 as i64 - f1 as i64).to_string(),
        (f4 as i64 - f2 as i64).to_string(),
        (f1 as u64 + f3 as u64).to_string(),
        (f2 as u64 + f4 as u64).to_string(),
    )
}

pub(crate) fn format_so_u16_halves(fields: &[u32], high: bool) -> String {
    format_so_halves(fields, high, |value| value.to_string())
}

pub(crate) fn format_so_i16_halves(fields: &[u32], high: bool) -> String {
    format_so_halves(fields, high, |value| (value as i16).to_string())
}

pub(crate) fn format_so_halves(
    fields: &[u32],
    high: bool,
    formatter: impl Fn(u16) -> String,
) -> String {
    let halves = fields
        .iter()
        .skip(1)
        .map(|field| {
            if high {
                (field >> 16) as u16
            } else {
                *field as u16
            }
        })
        .map(formatter)
        .collect::<Vec<_>>();
    if halves.is_empty() {
        "-".to_string()
    } else {
        halves.join(",")
    }
}
