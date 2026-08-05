use super::*;
use crate::*;

pub(crate) const VISUAL_LIST_MAGIC_OFFSET: usize = 4;

pub(crate) const VISUAL_LIST_MAGIC: &[u8; 4] = b"BMDV";

pub(crate) const VISUAL_LIST_HEADER_BYTES: usize = 0x50;

pub(crate) const VISUAL_LIST_VERSION_OFFSET: usize = 0x08;

pub(crate) const VISUAL_LIST_FLAGS_OFFSET: usize = 0x0c;

pub(crate) const VISUAL_LIST_WIDTH_OFFSET: usize = 0x1c;

pub(crate) const VISUAL_LIST_HEIGHT_OFFSET: usize = 0x20;

pub(crate) const VISUAL_LIST_ROW_STRIDE_OFFSET: usize = 0x24;

pub(crate) const VISUAL_LIST_BIT_DEPTH_OFFSET: usize = 0x2c;

pub(crate) const VISUAL_LIST_X_PPM_OFFSET: usize = 0x30;

pub(crate) const VISUAL_LIST_Y_PPM_OFFSET: usize = 0x34;

pub(crate) const VISUAL_LIST_RLE_LENGTH_OFFSET: usize = 0x4c;

pub(crate) const VISUAL_LIST_MIN_HORIZONTAL_RUN_PERCENT: usize = 31;

pub(crate) const JSFART2_CONTENTS_MAGIC_UTF16LE: &[u8; 22] = b"M\0S\0T\0U\0D\0I\0O\0.\0O\0C\0X\0";

pub(crate) const JSFART2_ART_WIDTH_OFFSET: usize = 40;

pub(crate) const JSFART2_ART_HEIGHT_OFFSET: usize = 44;

pub(crate) const JSFART2_ART_FRAME_LEFT_OFFSET: usize = 68;

pub(crate) const JSFART2_ART_FRAME_TOP_OFFSET: usize = 72;

pub(crate) const JSFART2_ART_FRAME_RIGHT_OFFSET: usize = 76;

pub(crate) const JSFART2_ART_FRAME_BOTTOM_OFFSET: usize = 80;

pub(crate) const JSFART2_ART_STROKE_WIDTH_CANDIDATE_OFFSET: usize = 100;

pub(crate) const JSFART2_ART_STYLE_WORD_1_OFFSET: usize = 0x6c;

pub(crate) const JSFART2_ART_STYLE_WORD_2_OFFSET: usize = 0x70;

pub(crate) const JSFART2_ART_PAINT_COLOR_CANDIDATE_OFFSET: usize = 0x88;

pub(crate) const JSFART2_ART_PAINT_FLAG_CANDIDATE_OFFSET: usize = 0x94;

pub(crate) const JSFART2_ART_EFFECT_WORD_CANDIDATE_OFFSET: usize = 0xa0;

pub(crate) const JSEQ3_CONTENTS_MAGIC_UTF16LE: &[u8; 16] = b"M\0A\0T\0H\0.\0V\0A\0F\0";

pub(crate) const JSEQ3_SO_TRAILER_BYTES: usize = 64;

pub(crate) const JSEQ3_SO_FIELD_BYTES: usize = 4;

pub(crate) const JSEQ3_SO_FIELD_COUNT: usize = 9;

pub(crate) const JSEQ3_TEXT_MARKERS: &[&str] =
    &["Times New Roman", "JustUnitMark", "JustOubunMark"];

pub(crate) const JSEQ3_TEXT_TOKEN_CHARS: &str = "０１２３４５６７８９＋－÷（）()";

pub(crate) const JSEQ3_TEXT_RUN_SCAN_MIN_OFFSET: usize = 0x80;

pub(crate) const JSEQ3_TEXT_RUN_CONTIGUOUS_STRIDE_BYTES: usize = 28;

pub(crate) const JSEQ3_TEXT_RUN_CONTEXT_BEFORE_BYTES: usize = 64;

pub(crate) const JSEQ3_TEXT_RUN_CONTEXT_FIELD_COUNT: usize = 40;

pub(crate) const EMBEDDING_INFO_PATH: &str = "/EmbedItems/EmbeddingInfo";

pub(crate) const EMBEDDING_INFO_HEADER_BYTES: usize = 16;

pub(crate) const EMBEDDING_INFO_CLASS_LENGTH_OFFSET: usize = 42;

pub(crate) const EMBEDDING_INFO_CLASS_START_OFFSET: usize = 46;

pub(crate) const EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET: usize = 14;

pub(crate) const EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET: usize = 18;

pub(crate) const EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET: usize = 8;

pub(crate) const EMBEDDING_INFO_TRAILING_BYTES: usize = 80;

pub(crate) const EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET: usize = 0;

pub(crate) const EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET: usize = 4;

pub(crate) const EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET: usize = 8;

pub(crate) const APP_IMAGE_DIAGNOSTIC_THUMB_PX: f32 = 72.0;

pub(crate) const APP_IMAGE_DIAGNOSTIC_GAP_PX: f32 = 8.0;

pub(crate) const APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS: usize = 8;

#[derive(Debug, Clone)]
pub(crate) struct DocumentSnapshot {
    pub(crate) id: u32,
    pub(crate) document: Document,
    pub(crate) pages: Vec<Vec<PageTextLine>>,
    pub(crate) file_name: String,
    pub(crate) dpi: f64,
    pub(crate) page_layout: PageLayout,
    pub(crate) show_paragraph_marks: bool,
    pub(crate) show_control_codes: bool,
    pub(crate) show_transparent_borders: bool,
    pub(crate) clip_enabled: bool,
    pub(crate) writing_mode: WritingMode,
    pub(crate) caret_section: u32,
    pub(crate) caret_paragraph: u32,
    pub(crate) caret_char_offset: u32,
    pub(crate) clipboard_text: Option<String>,
}

impl DocumentSnapshot {
    pub(crate) fn capture(id: u32, core: &DocumentCore) -> Self {
        Self {
            id,
            document: core.document.clone(),
            pages: core.pages.clone(),
            file_name: core.file_name.clone(),
            dpi: core.dpi,
            page_layout: core.page_layout,
            show_paragraph_marks: core.show_paragraph_marks,
            show_control_codes: core.show_control_codes,
            show_transparent_borders: core.show_transparent_borders,
            clip_enabled: core.clip_enabled,
            writing_mode: core.writing_mode,
            caret_section: core.caret_section,
            caret_paragraph: core.caret_paragraph,
            caret_char_offset: core.caret_char_offset,
            clipboard_text: core.clipboard_text.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJseq3TextMarkerCandidate {
    pub(crate) text: String,
    pub(crate) offset: usize,
    pub(crate) encoding: String,
}

impl ObjectJseq3TextMarkerCandidate {
    pub(crate) fn new(text: impl Into<String>, offset: usize, encoding: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            offset,
            encoding: encoding.into(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJseq3TextTokenCandidate {
    pub(crate) text: String,
    pub(crate) offset: usize,
    pub(crate) encoding: String,
}

impl ObjectJseq3TextTokenCandidate {
    pub(crate) fn new(text: impl Into<String>, offset: usize, encoding: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            offset,
            encoding: encoding.into(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJseq3TextRunCandidate {
    pub(crate) text: String,
    pub(crate) start_offset: usize,
    pub(crate) end_offset: usize,
    pub(crate) token_offsets: Vec<usize>,
    pub(crate) context_start_offset: usize,
    pub(crate) context_fields_le32: Vec<i32>,
}

impl ObjectJseq3TextRunCandidate {
    pub(crate) fn new(
        text: impl Into<String>,
        start_offset: usize,
        end_offset: usize,
        token_offsets: Vec<usize>,
        context_start_offset: usize,
        context_fields_le32: Vec<i32>,
    ) -> Self {
        Self {
            text: text.into(),
            start_offset,
            end_offset,
            token_offsets,
            context_start_offset,
            context_fields_le32,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn start_offset(&self) -> usize {
        self.start_offset
    }

    pub fn end_offset(&self) -> usize {
        self.end_offset
    }

    pub fn token_offsets(&self) -> &[usize] {
        &self.token_offsets
    }

    pub fn context_start_offset(&self) -> usize {
        self.context_start_offset
    }

    pub fn context_fields_le32(&self) -> &[i32] {
        &self.context_fields_le32
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJseq3FormulaCandidate {
    pub(crate) magic: String,
    pub(crate) magic_offset: usize,
    pub(crate) so_trailer_offset: Option<usize>,
    pub(crate) so_trailer_length: Option<usize>,
    pub(crate) so_trailer_fields: Vec<u32>,
    pub(crate) text_markers: Vec<ObjectJseq3TextMarkerCandidate>,
    pub(crate) text_tokens: Vec<ObjectJseq3TextTokenCandidate>,
    pub(crate) text_runs: Vec<ObjectJseq3TextRunCandidate>,
    pub(crate) header_prefix: Vec<u8>,
}

impl ObjectJseq3FormulaCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        magic_offset: usize,
        so_trailer_offset: Option<usize>,
        so_trailer_length: Option<usize>,
        so_trailer_fields: Vec<u32>,
        text_markers: Vec<ObjectJseq3TextMarkerCandidate>,
        text_tokens: Vec<ObjectJseq3TextTokenCandidate>,
        text_runs: Vec<ObjectJseq3TextRunCandidate>,
        header_prefix: Vec<u8>,
    ) -> Self {
        Self {
            magic: "MATH.VAF".to_string(),
            magic_offset,
            so_trailer_offset,
            so_trailer_length,
            so_trailer_fields,
            text_markers,
            text_tokens,
            text_runs,
            header_prefix,
        }
    }

    pub fn magic(&self) -> &str {
        &self.magic
    }

    pub fn magic_offset(&self) -> usize {
        self.magic_offset
    }

    pub fn so_trailer_offset(&self) -> Option<usize> {
        self.so_trailer_offset
    }

    pub fn so_trailer_length(&self) -> Option<usize> {
        self.so_trailer_length
    }

    pub fn so_trailer_fields(&self) -> &[u32] {
        &self.so_trailer_fields
    }

    pub fn text_markers(&self) -> &[ObjectJseq3TextMarkerCandidate] {
        &self.text_markers
    }

    pub fn text_tokens(&self) -> &[ObjectJseq3TextTokenCandidate] {
        &self.text_tokens
    }

    pub fn text_runs(&self) -> &[ObjectJseq3TextRunCandidate] {
        &self.text_runs
    }

    pub fn header_prefix(&self) -> &[u8] {
        &self.header_prefix
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJsfartArtFrameCandidate {
    pub(crate) left: u32,
    pub(crate) top: u32,
    pub(crate) right: u32,
    pub(crate) bottom: u32,
    pub(crate) content_left: u32,
    pub(crate) content_top: u32,
    pub(crate) content_right: u32,
    pub(crate) content_bottom: u32,
    pub(crate) corner_radius_x: u32,
    pub(crate) corner_radius_y: u32,
    pub(crate) stroke_width_candidate: Option<u32>,
}

impl ObjectJsfartArtFrameCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        left: u32,
        top: u32,
        right: u32,
        bottom: u32,
        content_left: u32,
        content_top: u32,
        content_right: u32,
        content_bottom: u32,
        corner_radius_x: u32,
        corner_radius_y: u32,
        stroke_width_candidate: Option<u32>,
    ) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
            content_left,
            content_top,
            content_right,
            content_bottom,
            corner_radius_x,
            corner_radius_y,
            stroke_width_candidate,
        }
    }

    pub fn left(&self) -> u32 {
        self.left
    }

    pub fn top(&self) -> u32 {
        self.top
    }

    pub fn right(&self) -> u32 {
        self.right
    }

    pub fn bottom(&self) -> u32 {
        self.bottom
    }

    pub fn content_left(&self) -> u32 {
        self.content_left
    }

    pub fn content_top(&self) -> u32 {
        self.content_top
    }

    pub fn content_right(&self) -> u32 {
        self.content_right
    }

    pub fn content_bottom(&self) -> u32 {
        self.content_bottom
    }

    pub fn corner_radius_x(&self) -> u32 {
        self.corner_radius_x
    }

    pub fn corner_radius_y(&self) -> u32 {
        self.corner_radius_y
    }

    pub fn stroke_width_candidate(&self) -> Option<u32> {
        self.stroke_width_candidate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJsfartStreamProfileCandidate {
    pub(crate) magic_family: String,
    pub(crate) magic_family_hex: String,
    pub(crate) magic_offset: usize,
    pub(crate) magic_ascii_or_utf16_preview: String,
    pub(crate) header_prefix: Vec<u8>,
    pub(crate) structured_art_candidate_present: bool,
    pub(crate) render_promotion_blocked_reason: String,
}

impl ObjectJsfartStreamProfileCandidate {
    pub(crate) fn new(
        magic_family: impl Into<String>,
        magic_family_hex: impl Into<String>,
        magic_offset: usize,
        magic_ascii_or_utf16_preview: impl Into<String>,
        header_prefix: Vec<u8>,
        structured_art_candidate_present: bool,
        render_promotion_blocked_reason: impl Into<String>,
    ) -> Self {
        Self {
            magic_family: magic_family.into(),
            magic_family_hex: magic_family_hex.into(),
            magic_offset,
            magic_ascii_or_utf16_preview: magic_ascii_or_utf16_preview.into(),
            header_prefix,
            structured_art_candidate_present,
            render_promotion_blocked_reason: render_promotion_blocked_reason.into(),
        }
    }

    pub fn magic_family(&self) -> &str {
        &self.magic_family
    }

    pub fn magic_family_hex(&self) -> &str {
        &self.magic_family_hex
    }

    pub fn magic_offset(&self) -> usize {
        self.magic_offset
    }

    pub fn magic_ascii_or_utf16_preview(&self) -> &str {
        &self.magic_ascii_or_utf16_preview
    }

    pub fn header_prefix(&self) -> &[u8] {
        &self.header_prefix
    }

    pub fn structured_art_candidate_present(&self) -> bool {
        self.structured_art_candidate_present
    }

    pub fn render_promotion_blocked_reason(&self) -> &str {
        &self.render_promotion_blocked_reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectJsfartArtCandidate {
    pub(crate) magic: String,
    pub(crate) magic_offset: usize,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) frame_candidate: Option<ObjectJsfartArtFrameCandidate>,
    pub(crate) paint_candidate: Option<ObjectJsfartArtPaintCandidate>,
    pub(crate) header_prefix: Vec<u8>,
}

impl ObjectJsfartArtCandidate {
    pub(crate) fn new(
        magic_offset: usize,
        width: u32,
        height: u32,
        frame_candidate: Option<ObjectJsfartArtFrameCandidate>,
        paint_candidate: Option<ObjectJsfartArtPaintCandidate>,
        header_prefix: Vec<u8>,
    ) -> Self {
        Self {
            magic: "MSTUDIO.OCX".to_string(),
            magic_offset,
            width,
            height,
            frame_candidate,
            paint_candidate,
            header_prefix,
        }
    }

    pub fn magic(&self) -> &str {
        &self.magic
    }

    pub fn magic_offset(&self) -> usize {
        self.magic_offset
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn frame_candidate(&self) -> Option<&ObjectJsfartArtFrameCandidate> {
        self.frame_candidate.as_ref()
    }

    pub fn paint_candidate(&self) -> Option<&ObjectJsfartArtPaintCandidate> {
        self.paint_candidate.as_ref()
    }

    pub fn header_prefix(&self) -> &[u8] {
        &self.header_prefix
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectJsfartArtPaintCandidate {
    pub(crate) style_word_1: u32,
    pub(crate) style_word_2: u32,
    pub(crate) paint_color_candidate: u32,
    pub(crate) paint_flag_candidate: u32,
    pub(crate) effect_word_candidate: u32,
}

impl ObjectJsfartArtPaintCandidate {
    pub(crate) fn new(
        style_word_1: u32,
        style_word_2: u32,
        paint_color_candidate: u32,
        paint_flag_candidate: u32,
        effect_word_candidate: u32,
    ) -> Self {
        Self {
            style_word_1,
            style_word_2,
            paint_color_candidate,
            paint_flag_candidate,
            effect_word_candidate,
        }
    }

    pub fn style_word_1(&self) -> u32 {
        self.style_word_1
    }

    pub fn style_word_2(&self) -> u32 {
        self.style_word_2
    }

    pub fn paint_color_candidate(&self) -> u32 {
        self.paint_color_candidate
    }

    pub fn paint_flag_candidate(&self) -> u32 {
        self.paint_flag_candidate
    }

    pub fn effect_word_candidate(&self) -> u32 {
        self.effect_word_candidate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddedPressSnapshotCandidate {
    pub(crate) magic: String,
    pub(crate) body_length_candidate: u32,
    pub(crate) format_marker: String,
    pub(crate) object_count_candidate: u32,
    pub(crate) object_table_offset_candidate: u32,
    pub(crate) payload_length_candidate: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) header_prefix: Vec<u8>,
    pub(crate) vector_segments: Vec<ObjectEmbeddedPressVectorSegmentCandidate>,
    pub(crate) vector_paths: Vec<ObjectEmbeddedPressVectorPathCandidate>,
}

impl ObjectEmbeddedPressSnapshotCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        body_length_candidate: u32,
        format_marker: impl Into<String>,
        object_count_candidate: u32,
        object_table_offset_candidate: u32,
        payload_length_candidate: u32,
        width: u32,
        height: u32,
        header_prefix: Vec<u8>,
        vector_segments: Vec<ObjectEmbeddedPressVectorSegmentCandidate>,
        vector_paths: Vec<ObjectEmbeddedPressVectorPathCandidate>,
    ) -> Self {
        Self {
            magic: "JSSnapShot32".to_string(),
            body_length_candidate,
            format_marker: format_marker.into(),
            object_count_candidate,
            object_table_offset_candidate,
            payload_length_candidate,
            width,
            height,
            header_prefix,
            vector_segments,
            vector_paths,
        }
    }

    pub fn magic(&self) -> &str {
        &self.magic
    }

    pub fn body_length_candidate(&self) -> u32 {
        self.body_length_candidate
    }

    pub fn format_marker(&self) -> &str {
        &self.format_marker
    }

    pub fn object_count_candidate(&self) -> u32 {
        self.object_count_candidate
    }

    pub fn object_table_offset_candidate(&self) -> u32 {
        self.object_table_offset_candidate
    }

    pub fn payload_length_candidate(&self) -> u32 {
        self.payload_length_candidate
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn header_prefix(&self) -> &[u8] {
        &self.header_prefix
    }

    pub fn vector_segments(&self) -> &[ObjectEmbeddedPressVectorSegmentCandidate] {
        &self.vector_segments
    }

    pub fn vector_paths(&self) -> &[ObjectEmbeddedPressVectorPathCandidate] {
        &self.vector_paths
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageSignatureHit {
    pub(crate) kind: String,
    pub(crate) offset: usize,
}

impl ObjectImageSignatureHit {
    pub fn new(kind: impl Into<String>, offset: usize) -> Self {
        Self {
            kind: kind.into(),
            offset,
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageNumericHeaderField {
    pub(crate) offset: usize,
    pub(crate) value: u64,
}

impl ObjectImageNumericHeaderField {
    pub fn new(offset: usize, value: u64) -> Self {
        Self { offset, value }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageSourcePathCandidate {
    pub(crate) length_offset: usize,
    pub(crate) declared_length: usize,
    pub(crate) bytes_start: usize,
    pub(crate) bytes_end: usize,
    pub(crate) nul_terminated: bool,
    pub(crate) bytes: Vec<u8>,
    pub(crate) text_lossy: String,
}

impl ObjectImageSourcePathCandidate {
    pub fn new(
        length_offset: usize,
        declared_length: usize,
        bytes_start: usize,
        bytes_end: usize,
        nul_terminated: bool,
        bytes: Vec<u8>,
    ) -> Self {
        let text_bytes = if nul_terminated && bytes.last() == Some(&0) {
            &bytes[..bytes.len().saturating_sub(1)]
        } else {
            &bytes
        };
        Self {
            length_offset,
            declared_length,
            bytes_start,
            bytes_end,
            nul_terminated,
            text_lossy: String::from_utf8_lossy(text_bytes).into_owned(),
            bytes,
        }
    }

    pub fn length_offset(&self) -> usize {
        self.length_offset
    }

    pub fn declared_length(&self) -> usize {
        self.declared_length
    }

    pub fn bytes_start(&self) -> usize {
        self.bytes_start
    }

    pub fn bytes_end(&self) -> usize {
        self.bytes_end
    }

    pub fn nul_terminated(&self) -> bool {
        self.nul_terminated
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn text_lossy(&self) -> &str {
        &self.text_lossy
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageHeaderFieldCandidates {
    pub(crate) u16_le_prefix: Vec<ObjectImageNumericHeaderField>,
    pub(crate) u32_le_prefix: Vec<ObjectImageNumericHeaderField>,
    pub(crate) source_path_candidate: Option<ObjectImageSourcePathCandidate>,
}

impl ObjectImageHeaderFieldCandidates {
    pub fn new(
        u16_le_prefix: Vec<ObjectImageNumericHeaderField>,
        u32_le_prefix: Vec<ObjectImageNumericHeaderField>,
        source_path_candidate: Option<ObjectImageSourcePathCandidate>,
    ) -> Self {
        Self {
            u16_le_prefix,
            u32_le_prefix,
            source_path_candidate,
        }
    }

    pub fn u16_le_prefix(&self) -> &[ObjectImageNumericHeaderField] {
        &self.u16_le_prefix
    }

    pub fn u32_le_prefix(&self) -> &[ObjectImageNumericHeaderField] {
        &self.u32_le_prefix
    }

    pub fn source_path_candidate(&self) -> Option<&ObjectImageSourcePathCandidate> {
        self.source_path_candidate.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImageDeclaredLengthCandidate {
    pub(crate) offset: usize,
    pub(crate) value: usize,
    pub(crate) endian: String,
}

impl ObjectImageDeclaredLengthCandidate {
    pub fn new(offset: usize, value: usize, endian: impl Into<String>) -> Self {
        Self {
            offset,
            value,
            endian: endian.into(),
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn endian(&self) -> &str {
        &self.endian
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImagePayloadEnvelope {
    pub(crate) header_start: usize,
    pub(crate) header_end: usize,
    pub(crate) trailer_start: usize,
    pub(crate) trailer_end: usize,
    pub(crate) declared_payload_length: Option<ObjectImageDeclaredLengthCandidate>,
    pub(crate) header_fields: ObjectImageHeaderFieldCandidates,
    pub(crate) header: Vec<u8>,
    pub(crate) trailer: Vec<u8>,
}

impl ObjectImagePayloadEnvelope {
    pub fn new(
        header_start: usize,
        header_end: usize,
        trailer_start: usize,
        trailer_end: usize,
        declared_payload_length: Option<ObjectImageDeclaredLengthCandidate>,
        header: Vec<u8>,
        trailer: Vec<u8>,
    ) -> Self {
        let header_fields = image_header_field_candidates(header_start, &header);
        Self {
            header_start,
            header_end,
            trailer_start,
            trailer_end,
            declared_payload_length,
            header_fields,
            header,
            trailer,
        }
    }

    pub fn header_start(&self) -> usize {
        self.header_start
    }

    pub fn header_end(&self) -> usize {
        self.header_end
    }

    pub fn header_len(&self) -> usize {
        self.header.len()
    }

    pub fn header(&self) -> &[u8] {
        &self.header
    }

    pub fn trailer_start(&self) -> usize {
        self.trailer_start
    }

    pub fn trailer_end(&self) -> usize {
        self.trailer_end
    }

    pub fn trailer_len(&self) -> usize {
        self.trailer.len()
    }

    pub fn trailer(&self) -> &[u8] {
        &self.trailer
    }

    pub fn declared_payload_length(&self) -> Option<&ObjectImageDeclaredLengthCandidate> {
        self.declared_payload_length.as_ref()
    }

    pub fn header_fields(&self) -> &ObjectImageHeaderFieldCandidates {
        &self.header_fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectImagePayloadLocation {
    pub(crate) signature_offset: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl ObjectImagePayloadLocation {
    pub fn new(signature_offset: usize, start: usize, end: usize) -> Self {
        Self {
            signature_offset,
            start,
            end,
        }
    }

    pub fn signature_offset(&self) -> usize {
        self.signature_offset
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectImagePayloadSpan {
    pub(crate) kind: String,
    pub(crate) mime: String,
    pub(crate) location: ObjectImagePayloadLocation,
    pub(crate) complete: bool,
    pub(crate) payload: Vec<u8>,
    pub(crate) dimensions: Option<ObjectImageDimensions>,
    pub(crate) envelope: ObjectImagePayloadEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectImageDimensions {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl ObjectImageDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(self) -> u32 {
        self.width
    }

    pub fn height(self) -> u32 {
        self.height
    }
}

impl ObjectImagePayloadSpan {
    pub fn new(
        kind: impl Into<String>,
        mime: impl Into<String>,
        location: ObjectImagePayloadLocation,
        complete: bool,
        payload: Vec<u8>,
        envelope: ObjectImagePayloadEnvelope,
    ) -> Self {
        let dimensions = image_payload_dimensions(&payload);
        Self::new_with_dimensions(
            kind, mime, location, complete, payload, dimensions, envelope,
        )
    }

    pub(crate) fn new_with_dimensions(
        kind: impl Into<String>,
        mime: impl Into<String>,
        location: ObjectImagePayloadLocation,
        complete: bool,
        payload: Vec<u8>,
        dimensions: Option<ObjectImageDimensions>,
        envelope: ObjectImagePayloadEnvelope,
    ) -> Self {
        Self {
            kind: kind.into(),
            mime: mime.into(),
            location,
            complete,
            payload,
            dimensions,
            envelope,
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn mime(&self) -> &str {
        &self.mime
    }

    pub fn signature_offset(&self) -> usize {
        self.location.signature_offset()
    }

    pub fn start(&self) -> usize {
        self.location.start()
    }

    pub fn end(&self) -> usize {
        self.location.end()
    }

    pub fn len(&self) -> usize {
        self.payload.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn complete(&self) -> bool {
        self.complete
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn dimensions(&self) -> Option<ObjectImageDimensions> {
        self.dimensions
    }

    pub fn envelope(&self) -> &ObjectImagePayloadEnvelope {
        &self.envelope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddingFrameCandidate {
    pub(crate) source_path: String,
    pub(crate) row_index: usize,
    pub(crate) row_start: usize,
    pub(crate) embedding_index: usize,
    pub(crate) class_name: String,
    pub(crate) primary_width: u16,
    pub(crate) primary_height: u16,
    pub(crate) frame_ref: u32,
    pub(crate) frame_width: u32,
    pub(crate) frame_height: u32,
    pub(crate) row_prefix: Vec<u8>,
}

impl ObjectEmbeddingFrameCandidate {
    pub(crate) fn new(
        source_path: impl Into<String>,
        row_index: usize,
        row_start: usize,
        row: &[u8],
        class_name: impl Into<String>,
        trailing: &[u8],
    ) -> Option<Self> {
        let class_name = class_name.into();
        let embedding_index = read_le32_at(row, EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET)? as usize;
        let primary_width = read_le16_at(row, EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET)?;
        let primary_height = read_le16_at(row, EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET)?;
        let frame_ref = read_le32_at(trailing, EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET)?;
        let frame_width = read_le32_at(trailing, EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET)?;
        let frame_height = read_le32_at(trailing, EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET)?;
        Some(Self {
            source_path: source_path.into(),
            row_index,
            row_start,
            embedding_index,
            class_name,
            primary_width,
            primary_height,
            frame_ref,
            frame_width,
            frame_height,
            row_prefix: row[..row.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
        })
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn row_start(&self) -> usize {
        self.row_start
    }

    pub fn embedding_index(&self) -> usize {
        self.embedding_index
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub fn primary_width(&self) -> u16 {
        self.primary_width
    }

    pub fn primary_height(&self) -> u16 {
        self.primary_height
    }

    pub fn frame_ref(&self) -> u32 {
        self.frame_ref
    }

    pub fn frame_width(&self) -> u32 {
        self.frame_width
    }

    pub fn frame_height(&self) -> u32 {
        self.frame_height
    }

    pub fn row_prefix(&self) -> &[u8] {
        &self.row_prefix
    }
}
