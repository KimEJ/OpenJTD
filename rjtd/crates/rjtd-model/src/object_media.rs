use super::*;

pub(super) const VISUAL_LIST_MAGIC_OFFSET: usize = 4;

pub(super) const VISUAL_LIST_MAGIC: &[u8; 4] = b"BMDV";

pub(super) const VISUAL_LIST_HEADER_BYTES: usize = 0x50;

pub(super) const VISUAL_LIST_VERSION_OFFSET: usize = 0x08;

pub(super) const VISUAL_LIST_FLAGS_OFFSET: usize = 0x0c;

pub(super) const VISUAL_LIST_WIDTH_OFFSET: usize = 0x1c;

pub(super) const VISUAL_LIST_HEIGHT_OFFSET: usize = 0x20;

pub(super) const VISUAL_LIST_ROW_STRIDE_OFFSET: usize = 0x24;

pub(super) const VISUAL_LIST_BIT_DEPTH_OFFSET: usize = 0x2c;

pub(super) const VISUAL_LIST_X_PPM_OFFSET: usize = 0x30;

pub(super) const VISUAL_LIST_Y_PPM_OFFSET: usize = 0x34;

pub(super) const VISUAL_LIST_RLE_LENGTH_OFFSET: usize = 0x4c;

pub(super) const VISUAL_LIST_MIN_HORIZONTAL_RUN_PERCENT: usize = 31;

pub(super) const JSFART2_CONTENTS_MAGIC_UTF16LE: &[u8; 22] = b"M\0S\0T\0U\0D\0I\0O\0.\0O\0C\0X\0";

pub(super) const JSFART2_ART_WIDTH_OFFSET: usize = 40;

pub(super) const JSFART2_ART_HEIGHT_OFFSET: usize = 44;

pub(super) const JSFART2_ART_FRAME_LEFT_OFFSET: usize = 68;

pub(super) const JSFART2_ART_FRAME_TOP_OFFSET: usize = 72;

pub(super) const JSFART2_ART_FRAME_RIGHT_OFFSET: usize = 76;

pub(super) const JSFART2_ART_FRAME_BOTTOM_OFFSET: usize = 80;

pub(super) const JSFART2_ART_STROKE_WIDTH_CANDIDATE_OFFSET: usize = 100;

pub(super) const JSFART2_ART_STYLE_WORD_1_OFFSET: usize = 0x6c;

pub(super) const JSFART2_ART_STYLE_WORD_2_OFFSET: usize = 0x70;

pub(super) const JSFART2_ART_PAINT_COLOR_CANDIDATE_OFFSET: usize = 0x88;

pub(super) const JSFART2_ART_PAINT_FLAG_CANDIDATE_OFFSET: usize = 0x94;

pub(super) const JSFART2_ART_EFFECT_WORD_CANDIDATE_OFFSET: usize = 0xa0;

pub(super) const JSEQ3_CONTENTS_MAGIC_UTF16LE: &[u8; 16] = b"M\0A\0T\0H\0.\0V\0A\0F\0";

pub(super) const JSEQ3_SO_TRAILER_BYTES: usize = 64;

pub(super) const JSEQ3_SO_FIELD_BYTES: usize = 4;

pub(super) const JSEQ3_SO_FIELD_COUNT: usize = 9;

pub(super) const JSEQ3_TEXT_MARKERS: &[&str] =
    &["Times New Roman", "JustUnitMark", "JustOubunMark"];

pub(super) const JSEQ3_TEXT_TOKEN_CHARS: &str = "０１２３４５６７８９＋－÷（）()";

pub(super) const JSEQ3_TEXT_RUN_SCAN_MIN_OFFSET: usize = 0x80;

pub(super) const JSEQ3_TEXT_RUN_CONTIGUOUS_STRIDE_BYTES: usize = 28;

pub(super) const JSEQ3_TEXT_RUN_CONTEXT_BEFORE_BYTES: usize = 64;

pub(super) const JSEQ3_TEXT_RUN_CONTEXT_FIELD_COUNT: usize = 40;

pub(super) const EMBEDDING_INFO_PATH: &str = "/EmbedItems/EmbeddingInfo";

pub(super) const EMBEDDING_INFO_HEADER_BYTES: usize = 16;

pub(super) const EMBEDDING_INFO_CLASS_LENGTH_OFFSET: usize = 42;

pub(super) const EMBEDDING_INFO_CLASS_START_OFFSET: usize = 46;

pub(super) const EMBEDDING_INFO_PRIMARY_WIDTH_OFFSET: usize = 14;

pub(super) const EMBEDDING_INFO_PRIMARY_HEIGHT_OFFSET: usize = 18;

pub(super) const EMBEDDING_INFO_EMBEDDING_INDEX_OFFSET: usize = 8;

pub(super) const EMBEDDING_INFO_TRAILING_BYTES: usize = 80;

pub(super) const EMBEDDING_INFO_FRAME_REF_TRAILING_OFFSET: usize = 0;

pub(super) const EMBEDDING_INFO_FRAME_WIDTH_TRAILING_OFFSET: usize = 4;

pub(super) const EMBEDDING_INFO_FRAME_HEIGHT_TRAILING_OFFSET: usize = 8;

pub(super) const APP_IMAGE_DIAGNOSTIC_THUMB_PX: f32 = 72.0;

pub(super) const APP_IMAGE_DIAGNOSTIC_GAP_PX: f32 = 8.0;

pub(super) const APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS: usize = 8;

#[derive(Debug, Clone)]
pub(super) struct DocumentSnapshot {
    pub(super) id: u32,
    pub(super) document: Document,
    pub(super) pages: Vec<Vec<PageTextLine>>,
    pub(super) file_name: String,
    pub(super) dpi: f64,
    pub(super) page_layout: PageLayout,
    pub(super) show_paragraph_marks: bool,
    pub(super) show_control_codes: bool,
    pub(super) show_transparent_borders: bool,
    pub(super) clip_enabled: bool,
    pub(super) writing_mode: WritingMode,
    pub(super) caret_section: u32,
    pub(super) caret_paragraph: u32,
    pub(super) caret_char_offset: u32,
    pub(super) clipboard_text: Option<String>,
}

impl DocumentSnapshot {
    pub(super) fn capture(id: u32, core: &DocumentCore) -> Self {
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
    pub(super) text: String,
    pub(super) offset: usize,
    pub(super) encoding: String,
}

impl ObjectJseq3TextMarkerCandidate {
    pub(super) fn new(text: impl Into<String>, offset: usize, encoding: impl Into<String>) -> Self {
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
    pub(super) text: String,
    pub(super) offset: usize,
    pub(super) encoding: String,
}

impl ObjectJseq3TextTokenCandidate {
    pub(super) fn new(text: impl Into<String>, offset: usize, encoding: impl Into<String>) -> Self {
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
    pub(super) text: String,
    pub(super) start_offset: usize,
    pub(super) end_offset: usize,
    pub(super) token_offsets: Vec<usize>,
    pub(super) context_start_offset: usize,
    pub(super) context_fields_le32: Vec<i32>,
}

impl ObjectJseq3TextRunCandidate {
    pub(super) fn new(
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
    pub(super) magic: String,
    pub(super) magic_offset: usize,
    pub(super) so_trailer_offset: Option<usize>,
    pub(super) so_trailer_length: Option<usize>,
    pub(super) so_trailer_fields: Vec<u32>,
    pub(super) text_markers: Vec<ObjectJseq3TextMarkerCandidate>,
    pub(super) text_tokens: Vec<ObjectJseq3TextTokenCandidate>,
    pub(super) text_runs: Vec<ObjectJseq3TextRunCandidate>,
    pub(super) header_prefix: Vec<u8>,
}

impl ObjectJseq3FormulaCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
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
    pub(super) left: u32,
    pub(super) top: u32,
    pub(super) right: u32,
    pub(super) bottom: u32,
    pub(super) content_left: u32,
    pub(super) content_top: u32,
    pub(super) content_right: u32,
    pub(super) content_bottom: u32,
    pub(super) corner_radius_x: u32,
    pub(super) corner_radius_y: u32,
    pub(super) stroke_width_candidate: Option<u32>,
}

impl ObjectJsfartArtFrameCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
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
    pub(super) magic_family: String,
    pub(super) magic_family_hex: String,
    pub(super) magic_offset: usize,
    pub(super) magic_ascii_or_utf16_preview: String,
    pub(super) header_prefix: Vec<u8>,
    pub(super) structured_art_candidate_present: bool,
    pub(super) render_promotion_blocked_reason: String,
}

impl ObjectJsfartStreamProfileCandidate {
    pub(super) fn new(
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
    pub(super) magic: String,
    pub(super) magic_offset: usize,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) frame_candidate: Option<ObjectJsfartArtFrameCandidate>,
    pub(super) paint_candidate: Option<ObjectJsfartArtPaintCandidate>,
    pub(super) header_prefix: Vec<u8>,
}

impl ObjectJsfartArtCandidate {
    pub(super) fn new(
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
    pub(super) style_word_1: u32,
    pub(super) style_word_2: u32,
    pub(super) paint_color_candidate: u32,
    pub(super) paint_flag_candidate: u32,
    pub(super) effect_word_candidate: u32,
}

impl ObjectJsfartArtPaintCandidate {
    pub(super) fn new(
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
    pub(super) magic: String,
    pub(super) body_length_candidate: u32,
    pub(super) format_marker: String,
    pub(super) object_count_candidate: u32,
    pub(super) object_table_offset_candidate: u32,
    pub(super) payload_length_candidate: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) header_prefix: Vec<u8>,
    pub(super) vector_segments: Vec<ObjectEmbeddedPressVectorSegmentCandidate>,
    pub(super) vector_paths: Vec<ObjectEmbeddedPressVectorPathCandidate>,
}

impl ObjectEmbeddedPressSnapshotCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
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
    pub(super) kind: String,
    pub(super) offset: usize,
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
    pub(super) offset: usize,
    pub(super) value: u64,
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
    pub(super) length_offset: usize,
    pub(super) declared_length: usize,
    pub(super) bytes_start: usize,
    pub(super) bytes_end: usize,
    pub(super) nul_terminated: bool,
    pub(super) bytes: Vec<u8>,
    pub(super) text_lossy: String,
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
    pub(super) u16_le_prefix: Vec<ObjectImageNumericHeaderField>,
    pub(super) u32_le_prefix: Vec<ObjectImageNumericHeaderField>,
    pub(super) source_path_candidate: Option<ObjectImageSourcePathCandidate>,
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
    pub(super) offset: usize,
    pub(super) value: usize,
    pub(super) endian: String,
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
    pub(super) header_start: usize,
    pub(super) header_end: usize,
    pub(super) trailer_start: usize,
    pub(super) trailer_end: usize,
    pub(super) declared_payload_length: Option<ObjectImageDeclaredLengthCandidate>,
    pub(super) header_fields: ObjectImageHeaderFieldCandidates,
    pub(super) header: Vec<u8>,
    pub(super) trailer: Vec<u8>,
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
    pub(super) signature_offset: usize,
    pub(super) start: usize,
    pub(super) end: usize,
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
    pub(super) kind: String,
    pub(super) mime: String,
    pub(super) location: ObjectImagePayloadLocation,
    pub(super) complete: bool,
    pub(super) payload: Vec<u8>,
    pub(super) dimensions: Option<ObjectImageDimensions>,
    pub(super) envelope: ObjectImagePayloadEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectImageDimensions {
    pub(super) width: u32,
    pub(super) height: u32,
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

    pub(super) fn new_with_dimensions(
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
    pub(super) source_path: String,
    pub(super) row_index: usize,
    pub(super) row_start: usize,
    pub(super) embedding_index: usize,
    pub(super) class_name: String,
    pub(super) primary_width: u16,
    pub(super) primary_height: u16,
    pub(super) frame_ref: u32,
    pub(super) frame_width: u32,
    pub(super) frame_height: u32,
    pub(super) row_prefix: Vec<u8>,
}

impl ObjectEmbeddingFrameCandidate {
    pub(super) fn new(
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

pub(super) fn object_embedding_frames_from_cfb(
    data: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectEmbeddingFrameCandidate>> {
    let Ok(stream) = read_cfb_stream(data, EMBEDDING_INFO_PATH) else {
        return Ok(Vec::new());
    };

    object_embedding_frames_from_stream(EMBEDDING_INFO_PATH, &stream, budget)
}

pub(super) fn object_embedding_frames_from_stream(
    path: &str,
    stream: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectEmbeddingFrameCandidate>> {
    let Some(declared_count) = read_le32_at(stream, 0).map(|value| value as usize) else {
        return Ok(Vec::new());
    };

    let mut frames = Vec::new();
    let mut cursor = EMBEDDING_INFO_HEADER_BYTES;
    for row_index in 0..declared_count {
        let Some(class_len_offset) = cursor.checked_add(EMBEDDING_INFO_CLASS_LENGTH_OFFSET) else {
            break;
        };
        let Some(class_len) = read_le32_at(stream, class_len_offset).map(|value| value as usize)
        else {
            break;
        };
        let Some(class_start) = cursor.checked_add(EMBEDDING_INFO_CLASS_START_OFFSET) else {
            break;
        };
        let Some(class_end) = class_start.checked_add(class_len) else {
            break;
        };
        let Some(row_end) = class_end.checked_add(EMBEDDING_INFO_TRAILING_BYTES) else {
            break;
        };
        let Some(row) = stream.get(cursor..row_end) else {
            break;
        };
        let Some(class_bytes) = stream.get(class_start..class_end) else {
            break;
        };
        let trailing = &stream[class_end..row_end];
        budget.reserve_record(row.len())?;
        let Some(class_name) = decode_utf16le_c_string(class_bytes) else {
            break;
        };
        if class_name.is_empty() || class_len == 0 || class_len % 2 != 0 {
            break;
        }
        let Some(frame) =
            ObjectEmbeddingFrameCandidate::new(path, row_index, cursor, row, class_name, trailing)
        else {
            break;
        };
        if embedding_frame_candidate_is_plausible(&frame) {
            frames.push(frame);
        }
        cursor = row_end;
    }

    Ok(frames)
}

pub(super) fn embedding_frame_candidate_is_plausible(
    frame: &ObjectEmbeddingFrameCandidate,
) -> bool {
    frame.embedding_index() > 0
        && frame.frame_ref() > 0
        && frame.frame_width() > 0
        && frame.frame_height() > 0
        && frame.frame_width() <= 200_000
        && frame.frame_height() <= 200_000
        && frame.class_name().chars().all(|character| {
            character == '.'
                || character == '_'
                || character == '-'
                || character.is_ascii_alphanumeric()
        })
}

pub(super) fn object_stream_embedding_reference_patterns(
    embedding_index: usize,
) -> Vec<(&'static str, Vec<u8>)> {
    let mut patterns = Vec::new();
    if let Ok(index) = u16::try_from(embedding_index) {
        patterns.push(("u16-le", index.to_le_bytes().to_vec()));
        patterns.push(("u16-be", index.to_be_bytes().to_vec()));
    }
    if let Ok(index) = u32::try_from(embedding_index) {
        patterns.push(("u32-le", index.to_le_bytes().to_vec()));
        patterns.push(("u32-be", index.to_be_bytes().to_vec()));
    }
    patterns
}

pub(super) fn jsfart_stream_profile_candidate_from_stream(
    path: &str,
    stream: &[u8],
    structured_art_candidate_present: bool,
) -> Option<ObjectJsfartStreamProfileCandidate> {
    if !path.ends_with("/JSFart2Contents") {
        return None;
    }

    let header_prefix = stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec();
    let magic_family_hex = hex_bytes(&stream[..stream.len().min(2)]);
    let preview = utf16le_printable_preview(&header_prefix);
    let magic_family = jsfart_stream_magic_family(stream, &preview);
    let render_promotion_blocked_reason = if structured_art_candidate_present {
        "structured-jsfart-art-still-paint-authority-unproven"
    } else {
        "jsfart-variant-layout-undecoded"
    };

    Some(ObjectJsfartStreamProfileCandidate::new(
        magic_family,
        magic_family_hex,
        0,
        preview,
        header_prefix,
        structured_art_candidate_present,
        render_promotion_blocked_reason,
    ))
}

pub(super) fn jsfart_stream_magic_family(stream: &[u8], utf16le_preview: &str) -> &'static str {
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

pub(super) fn jsfart_art_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectJsfartArtCandidate> {
    if !path.ends_with("/JSFart2Contents") {
        return None;
    }
    if stream.get(..JSFART2_CONTENTS_MAGIC_UTF16LE.len())? != JSFART2_CONTENTS_MAGIC_UTF16LE {
        return None;
    }

    let width = read_le32_at(stream, JSFART2_ART_WIDTH_OFFSET)?;
    let height = read_le32_at(stream, JSFART2_ART_HEIGHT_OFFSET)?;
    let frame_candidate = jsfart_art_frame_candidate_from_stream(stream, width, height);
    let paint_candidate = jsfart_art_paint_candidate_from_stream(stream);
    Some(ObjectJsfartArtCandidate::new(
        0,
        width,
        height,
        frame_candidate,
        paint_candidate,
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
    ))
}

pub(super) fn jsfart_art_paint_candidate_from_stream(
    stream: &[u8],
) -> Option<ObjectJsfartArtPaintCandidate> {
    Some(ObjectJsfartArtPaintCandidate::new(
        read_le32_at(stream, JSFART2_ART_STYLE_WORD_1_OFFSET)?,
        read_le32_at(stream, JSFART2_ART_STYLE_WORD_2_OFFSET)?,
        read_le32_at(stream, JSFART2_ART_PAINT_COLOR_CANDIDATE_OFFSET)?,
        read_le32_at(stream, JSFART2_ART_PAINT_FLAG_CANDIDATE_OFFSET)?,
        read_le32_at(stream, JSFART2_ART_EFFECT_WORD_CANDIDATE_OFFSET)?,
    ))
}

pub(super) fn jsfart_art_frame_candidate_from_stream(
    stream: &[u8],
    width: u32,
    height: u32,
) -> Option<ObjectJsfartArtFrameCandidate> {
    if width == 0 || height == 0 {
        return None;
    }

    let content_left = read_le32_at(stream, JSFART2_ART_FRAME_LEFT_OFFSET)?;
    let content_top = read_le32_at(stream, JSFART2_ART_FRAME_TOP_OFFSET)?;
    let content_right = read_le32_at(stream, JSFART2_ART_FRAME_RIGHT_OFFSET)?;
    let content_bottom = read_le32_at(stream, JSFART2_ART_FRAME_BOTTOM_OFFSET)?;
    if !(content_left < content_right
        && content_top < content_bottom
        && content_right <= width
        && content_bottom <= height)
    {
        return None;
    }

    let corner_radius_x = content_left;
    let corner_radius_y = content_top;
    let stroke_width_candidate = read_le32_at(stream, JSFART2_ART_STROKE_WIDTH_CANDIDATE_OFFSET)
        .filter(|value| *value > 0 && *value <= height);

    Some(ObjectJsfartArtFrameCandidate::new(
        0,
        0,
        width,
        height,
        content_left,
        content_top,
        content_right,
        content_bottom,
        corner_radius_x,
        corner_radius_y,
        stroke_width_candidate,
    ))
}

pub(super) fn jseq3_formula_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectJseq3FormulaCandidate> {
    if !path.ends_with("/JSEQ3Contents") {
        return None;
    }
    if stream.get(..JSEQ3_CONTENTS_MAGIC_UTF16LE.len())? != JSEQ3_CONTENTS_MAGIC_UTF16LE {
        return None;
    }

    let so_trailer_offset = jseq3_so_trailer_offset(stream);
    let so_trailer_length = so_trailer_offset.map(|offset| stream.len().saturating_sub(offset));
    let so_trailer_fields = so_trailer_offset
        .and_then(|offset| stream.get(offset..))
        .map(jseq3_so_trailer_fields)
        .unwrap_or_default();
    let text_markers = jseq3_text_marker_candidates(stream);
    let text_tokens = jseq3_text_token_candidates(stream);
    let text_runs = jseq3_text_run_candidates(stream, &text_tokens);
    Some(ObjectJseq3FormulaCandidate::new(
        0,
        so_trailer_offset,
        so_trailer_length,
        so_trailer_fields,
        text_markers,
        text_tokens,
        text_runs,
        stream[..stream.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
    ))
}

pub(super) fn jseq3_so_trailer_offset(stream: &[u8]) -> Option<usize> {
    find_subslice_offsets(stream, SO_RECORD_MARKER)
        .into_iter()
        .find(|offset| {
            offset.saturating_add(JSEQ3_SO_FIELD_COUNT * JSEQ3_SO_FIELD_BYTES) <= stream.len()
                && offset.saturating_add(JSEQ3_SO_TRAILER_BYTES) >= stream.len()
        })
}

pub(super) fn jseq3_so_trailer_fields(trailer: &[u8]) -> Vec<u32> {
    (0..JSEQ3_SO_FIELD_COUNT)
        .filter_map(|index| read_le32_at(trailer, index * JSEQ3_SO_FIELD_BYTES))
        .collect()
}

pub(super) fn jseq3_text_marker_candidates(stream: &[u8]) -> Vec<ObjectJseq3TextMarkerCandidate> {
    let mut candidates = Vec::new();
    for marker in JSEQ3_TEXT_MARKERS {
        let encoded = utf16le_bytes(marker);
        for offset in find_subslice_offsets(stream, &encoded) {
            candidates.push(ObjectJseq3TextMarkerCandidate::new(
                *marker, offset, "utf-16le",
            ));
        }
    }
    candidates.sort_by_key(|candidate| candidate.offset());
    candidates
}

pub(super) fn jseq3_text_token_candidates(stream: &[u8]) -> Vec<ObjectJseq3TextTokenCandidate> {
    let mut tokens = Vec::new();
    let mut offset = 0usize;
    while offset + 2 <= stream.len() {
        let Some(unit) = read_le16_at(stream, offset) else {
            break;
        };
        if let Some(character) = char::from_u32(u32::from(unit))
            && JSEQ3_TEXT_TOKEN_CHARS.contains(character)
        {
            tokens.push(ObjectJseq3TextTokenCandidate::new(
                character.to_string(),
                offset,
                "utf-16le",
            ));
        }
        offset += 2;
    }
    tokens
}

pub(super) fn jseq3_text_run_candidates(
    stream: &[u8],
    tokens: &[ObjectJseq3TextTokenCandidate],
) -> Vec<ObjectJseq3TextRunCandidate> {
    let layout_tokens = tokens
        .iter()
        .filter(|token| token.offset() >= JSEQ3_TEXT_RUN_SCAN_MIN_OFFSET)
        .collect::<Vec<_>>();
    let mut runs = Vec::new();
    let mut index = 0usize;
    while index < layout_tokens.len() {
        let mut end_index = index + 1;
        while end_index < layout_tokens.len()
            && layout_tokens[end_index].offset()
                == layout_tokens[end_index - 1]
                    .offset()
                    .saturating_add(JSEQ3_TEXT_RUN_CONTIGUOUS_STRIDE_BYTES)
        {
            end_index += 1;
        }
        let slice = &layout_tokens[index..end_index];
        let Some(first) = slice.first() else {
            break;
        };
        let Some(last) = slice.last() else {
            break;
        };
        let text = slice
            .iter()
            .map(|token| token.text())
            .collect::<Vec<_>>()
            .join("");
        let token_offsets = slice.iter().map(|token| token.offset()).collect::<Vec<_>>();
        let context_start = first
            .offset()
            .saturating_sub(JSEQ3_TEXT_RUN_CONTEXT_BEFORE_BYTES);
        let context_fields = (0..JSEQ3_TEXT_RUN_CONTEXT_FIELD_COUNT)
            .filter_map(|field_index| {
                let offset = context_start.saturating_add(field_index * 4);
                stream
                    .get(offset..offset.saturating_add(4))
                    .and_then(|bytes| bytes.try_into().ok())
                    .map(i32::from_le_bytes)
            })
            .collect::<Vec<_>>();
        runs.push(ObjectJseq3TextRunCandidate::new(
            text,
            first.offset(),
            last.offset().saturating_add(2),
            token_offsets,
            context_start,
            context_fields,
        ));
        index = end_index;
    }
    runs
}

pub(super) fn visual_list_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectVisualListCandidate> {
    if !path.to_ascii_lowercase().contains("visuallist") {
        return None;
    }
    if stream.get(VISUAL_LIST_MAGIC_OFFSET..VISUAL_LIST_MAGIC_OFFSET + VISUAL_LIST_MAGIC.len())?
        != VISUAL_LIST_MAGIC
    {
        return None;
    }
    let declared_size = read_be32_at(stream, 0)? as usize;
    let version = read_be32_at(stream, VISUAL_LIST_VERSION_OFFSET)?;
    let flags = read_be32_at(stream, VISUAL_LIST_FLAGS_OFFSET)?;
    let width = read_be32_at(stream, VISUAL_LIST_WIDTH_OFFSET)?;
    let height = read_be32_at(stream, VISUAL_LIST_HEIGHT_OFFSET)?;
    let row_stride = read_be32_at(stream, VISUAL_LIST_ROW_STRIDE_OFFSET)?;
    let bit_depth = read_be32_at(stream, VISUAL_LIST_BIT_DEPTH_OFFSET)?;
    let x_pixels_per_meter = read_be32_at(stream, VISUAL_LIST_X_PPM_OFFSET)?;
    let y_pixels_per_meter = read_be32_at(stream, VISUAL_LIST_Y_PPM_OFFSET)?;
    let rle_data_len = read_be32_at(stream, VISUAL_LIST_RLE_LENGTH_OFFSET)? as usize;
    let rle_data_end = VISUAL_LIST_HEADER_BYTES.checked_add(rle_data_len)?;
    let rle_data = stream.get(VISUAL_LIST_HEADER_BYTES..rle_data_end)?;
    let pixels = decode_visual_list_rle8(width, height, rle_data)?;
    Some(ObjectVisualListCandidate::new(
        declared_size,
        version,
        flags,
        width,
        height,
        row_stride,
        bit_depth,
        x_pixels_per_meter,
        y_pixels_per_meter,
        VISUAL_LIST_HEADER_BYTES,
        rle_data_len,
        pixels,
    ))
}

pub(super) fn decode_visual_list_rle8(width: u32, height: u32, data: &[u8]) -> Option<Vec<u8>> {
    if width == 0 || height == 0 || width > 10_000 || height > 10_000 {
        return None;
    }
    let width = usize::try_from(width).ok()?;
    let height = usize::try_from(height).ok()?;
    let total_pixels = width.checked_mul(height)?;
    if total_pixels > 16_000_000 {
        return None;
    }

    let fill = visual_list_default_pixel(data);
    let mut pixels = Vec::with_capacity(total_pixels);
    let mut row = Vec::with_capacity(width);
    let mut offset = 0usize;
    while offset + 1 < data.len() && pixels.len() < total_pixels {
        let count = data[offset];
        let value = data[offset + 1];
        offset += 2;
        if count != 0 {
            row.extend(std::iter::repeat_n(value, count as usize));
            continue;
        }

        match value {
            0 => flush_visual_list_row(&mut pixels, &mut row, width, height, fill),
            1 => break,
            2 => {
                if offset + 1 >= data.len() {
                    return None;
                }
                let dx = data[offset] as usize;
                let dy = data[offset + 1] as usize;
                offset += 2;
                row.extend(std::iter::repeat_n(fill, dx));
                for _ in 0..dy {
                    flush_visual_list_row(&mut pixels, &mut row, width, height, fill);
                }
            }
            literal_len => {
                let literal_len = literal_len as usize;
                let literal_end = offset.checked_add(literal_len)?;
                row.extend_from_slice(data.get(offset..literal_end)?);
                offset = literal_end;
                if literal_len % 2 == 1 {
                    offset = offset.checked_add(1)?;
                    if offset > data.len() {
                        return None;
                    }
                }
            }
        }
    }

    if !row.is_empty() && pixels.len() < total_pixels {
        flush_visual_list_row(&mut pixels, &mut row, width, height, fill);
    }
    while pixels.len() < total_pixels {
        pixels.extend(std::iter::repeat_n(fill, width));
    }
    pixels.truncate(total_pixels);
    Some(pixels)
}

pub(super) fn visual_list_default_pixel(data: &[u8]) -> u8 {
    if data.len() >= 2 && data[0] != 0 {
        data[1]
    } else {
        0xff
    }
}

pub(super) fn flush_visual_list_row(
    pixels: &mut Vec<u8>,
    row: &mut Vec<u8>,
    width: usize,
    height: usize,
    fill: u8,
) {
    if pixels.len() >= width.saturating_mul(height) {
        row.clear();
        return;
    }
    if row.len() < width {
        row.extend(std::iter::repeat_n(fill, width - row.len()));
    }
    pixels.extend(row.iter().copied().take(width));
    row.clear();
}

pub(super) fn image_signature_hits(
    stream: &[u8],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectImageSignatureHit>> {
    let mut hits = Vec::new();
    push_signature_hits(&mut hits, stream, "png", b"\x89PNG\r\n\x1a\n", true, budget)?;
    push_signature_hits(&mut hits, stream, "jpeg", b"\xff\xd8\xff", true, budget)?;
    push_signature_hits(&mut hits, stream, "gif87a", b"GIF87a", true, budget)?;
    push_signature_hits(&mut hits, stream, "gif89a", b"GIF89a", true, budget)?;
    push_signature_hits(&mut hits, stream, "tiff-le", b"II\x2a\0", true, budget)?;
    push_signature_hits(&mut hits, stream, "tiff-be", b"MM\0\x2a", true, budget)?;
    push_signature_hits(
        &mut hits,
        stream,
        "wmf-placeable",
        b"\xd7\xcd\xc6\x9a",
        true,
        budget,
    )?;
    push_signature_hits(&mut hits, stream, "bmp", b"BM", false, budget)?;

    hits.sort_by(|left, right| {
        left.offset()
            .cmp(&right.offset())
            .then_with(|| left.kind().cmp(right.kind()))
    });
    Ok(hits)
}

pub(super) fn image_payload_spans(
    stream: &[u8],
    hits: &[ObjectImageSignatureHit],
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectImagePayloadSpan>> {
    let mut candidates = hits
        .iter()
        .filter_map(|hit| image_payload_candidate(stream, hit));
    let mut previous_end = None;
    let mut candidate = candidates.next();
    let mut spans = Vec::new();

    while let Some(current) = candidate {
        candidate = candidates.next();
        let next_start = candidate.as_ref().map(|next| next.start);
        let header_start = previous_end
            .filter(|end| *end <= current.start)
            .unwrap_or(0);
        let trailer_end = next_start
            .filter(|start| *start >= current.end)
            .unwrap_or(stream.len());
        let Some(payload) = stream.get(current.start..current.end) else {
            previous_end = Some(current.end);
            continue;
        };
        let dimensions = image_payload_dimensions(payload);
        if let Some(dimensions) = dimensions {
            budget.check_image_dimensions(dimensions.width(), dimensions.height())?;
        }
        let retained_bytes = image_payload_retained_bytes(
            payload.len(),
            header_start,
            current.start,
            current.end,
            trailer_end,
        )?;
        budget.reserve_image(retained_bytes)?;
        let envelope = image_payload_envelope(
            stream,
            header_start,
            current.start,
            current.end,
            trailer_end,
        );
        spans.push(ObjectImagePayloadSpan::new_with_dimensions(
            current.kind,
            current.mime,
            ObjectImagePayloadLocation::new(current.signature_offset, current.start, current.end),
            true,
            payload.to_vec(),
            dimensions,
            envelope,
        ));
        previous_end = Some(current.end);
    }
    Ok(spans)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ImagePayloadCandidate<'a> {
    pub(super) kind: &'a str,
    pub(super) mime: &'static str,
    pub(super) signature_offset: usize,
    pub(super) start: usize,
    pub(super) end: usize,
}

pub(super) fn image_payload_candidate<'a>(
    stream: &[u8],
    hit: &'a ObjectImageSignatureHit,
) -> Option<ImagePayloadCandidate<'a>> {
    let end = match hit.kind() {
        "jpeg" => jpeg_payload_end(stream, hit.offset())?,
        "png" => png_payload_end(stream, hit.offset())?,
        "gif87a" | "gif89a" => gif_payload_end(stream, hit.offset())?,
        "bmp" => bmp_payload_end(stream, hit.offset())?,
        _ => return None,
    };

    Some(ImagePayloadCandidate {
        kind: hit.kind(),
        mime: image_mime_for_kind(hit.kind()),
        signature_offset: hit.offset(),
        start: hit.offset(),
        end,
    })
}

pub(super) fn image_payload_dimensions(payload: &[u8]) -> Option<ObjectImageDimensions> {
    png_payload_dimensions(payload)
        .or_else(|| gif_payload_dimensions(payload))
        .or_else(|| bmp_payload_dimensions(payload))
        .or_else(|| jpeg_payload_dimensions(payload))
}

pub(super) fn image_payload_envelope(
    stream: &[u8],
    header_start: usize,
    header_end: usize,
    trailer_start: usize,
    trailer_end: usize,
) -> ObjectImagePayloadEnvelope {
    let header_start = header_start.min(header_end).min(stream.len());
    let header_end = header_end.min(stream.len());
    let trailer_start = trailer_start.min(stream.len());
    let trailer_end = trailer_end.max(trailer_start).min(stream.len());
    let header = stream[header_start..header_end].to_vec();
    let trailer = stream[trailer_start..trailer_end].to_vec();
    let declared_payload_length =
        image_declared_payload_length(&header, header_start, trailer_start - header_end);

    ObjectImagePayloadEnvelope::new(
        header_start,
        header_end,
        trailer_start,
        trailer_end,
        declared_payload_length,
        header,
        trailer,
    )
}

pub(super) fn image_header_field_candidates(
    header_start: usize,
    header: &[u8],
) -> ObjectImageHeaderFieldCandidates {
    let prefix_len = header.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES);
    let prefix = &header[..prefix_len];
    let mut u16_le_prefix = Vec::new();
    for relative_offset in (0..prefix.len()).step_by(2) {
        if relative_offset + 2 <= prefix.len() {
            u16_le_prefix.push(ObjectImageNumericHeaderField::new(
                header_start + relative_offset,
                u16::from_le_bytes([prefix[relative_offset], prefix[relative_offset + 1]]) as u64,
            ));
        }
    }

    let mut u32_le_prefix = Vec::new();
    for relative_offset in (0..prefix.len()).step_by(4) {
        if relative_offset + 4 <= prefix.len() {
            u32_le_prefix.push(ObjectImageNumericHeaderField::new(
                header_start + relative_offset,
                u32::from_le_bytes([
                    prefix[relative_offset],
                    prefix[relative_offset + 1],
                    prefix[relative_offset + 2],
                    prefix[relative_offset + 3],
                ]) as u64,
            ));
        }
    }

    ObjectImageHeaderFieldCandidates::new(
        u16_le_prefix,
        u32_le_prefix,
        image_source_path_candidate(header_start, header),
    )
}

pub(super) fn image_source_path_candidate(
    header_start: usize,
    header: &[u8],
) -> Option<ObjectImageSourcePathCandidate> {
    let length_offset = 16;
    let declared_length = *header.get(length_offset)? as usize;
    if declared_length < 3 {
        return None;
    }
    let bytes_start = length_offset + 1;
    let declared_end = bytes_start.checked_add(declared_length)?;
    let text_bytes = header.get(bytes_start..declared_end)?;
    let raw_end = if header.get(declared_end) == Some(&0) {
        declared_end + 1
    } else if text_bytes.last() == Some(&0) {
        declared_end
    } else {
        return None;
    };
    let bytes = header.get(bytes_start..raw_end)?;
    let text_bytes = if text_bytes.last() == Some(&0) {
        &text_bytes[..text_bytes.len().saturating_sub(1)]
    } else {
        text_bytes
    };
    if !looks_like_embedded_source_path(text_bytes) {
        return None;
    }

    Some(ObjectImageSourcePathCandidate::new(
        header_start + length_offset,
        declared_length,
        header_start + bytes_start,
        header_start + raw_end,
        true,
        bytes.to_vec(),
    ))
}

pub(super) fn image_declared_payload_length(
    header: &[u8],
    header_start: usize,
    payload_len: usize,
) -> Option<ObjectImageDeclaredLengthCandidate> {
    let offset_in_header = header.len().checked_sub(4)?;
    let value = u32::from_le_bytes([
        header[offset_in_header],
        header[offset_in_header + 1],
        header[offset_in_header + 2],
        header[offset_in_header + 3],
    ]) as usize;
    (value == payload_len).then(|| {
        ObjectImageDeclaredLengthCandidate::new(header_start + offset_in_header, value, "le32")
    })
}

pub(super) fn image_mime_for_kind(kind: &str) -> &'static str {
    match kind {
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif87a" | "gif89a" => "image/gif",
        "bmp" => "image/bmp",
        "tiff-le" | "tiff-be" => "image/tiff",
        "wmf-placeable" => "image/wmf",
        _ => "application/octet-stream",
    }
}

pub(super) fn next_snapshot_id(current: u32) -> u32 {
    current.checked_add(1).filter(|id| *id > 0).unwrap_or(1)
}

pub(super) fn object_embedding_frames_json(frames: &[ObjectEmbeddingFrameCandidate]) -> String {
    let mut output = String::from("[");
    for (index, frame) in frames.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_embedding_frame_candidate_json(&mut output, frame);
    }
    output.push(']');
    output
}

pub(super) fn push_object_embedding_frame_candidate_json(
    output: &mut String,
    frame: &ObjectEmbeddingFrameCandidate,
) {
    output.push_str("{\"sourcePath\":");
    output.push_str(&json_string(frame.source_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&frame.row_index().to_string());
    output.push_str(",\"rowStart\":");
    output.push_str(&frame.row_start().to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    output.push_str(&json_string(frame.class_name()));
    output.push_str(",\"primarySize\":{\"width\":");
    output.push_str(&frame.primary_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&frame.primary_height().to_string());
    output.push_str("},\"frameRef\":");
    output.push_str(&frame.frame_ref().to_string());
    output.push_str(",\"frameSize\":{\"width\":");
    output.push_str(&frame.frame_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&frame.frame_height().to_string());
    output.push_str("},\"rowPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(frame.row_prefix())));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_jsfart_stream_profile_candidate_json(
    output: &mut String,
    profile: &ObjectJsfartStreamProfileCandidate,
) {
    output.push_str("{\"format\":\"JSFart2Contents\",\"source\":\"stream-prefix\",\"sourceCandidateType\":\"objectStream\",\"magicFamily\":");
    output.push_str(&json_string(profile.magic_family()));
    output.push_str(",\"magicFamilyHex\":");
    output.push_str(&json_string(profile.magic_family_hex()));
    output.push_str(",\"magicOffset\":");
    output.push_str(&profile.magic_offset().to_string());
    output.push_str(",\"magicAsciiOrUtf16Preview\":");
    output.push_str(&json_string(profile.magic_ascii_or_utf16_preview()));
    output.push_str(",\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(profile.header_prefix())));
    output.push_str(",\"structuredArtCandidatePresent\":");
    output.push_str(if profile.structured_art_candidate_present() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"renderable\":false,\"decoded\":false,\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(profile.render_promotion_blocked_reason()));
    output.push('}');
}

pub(super) fn push_object_jsfart_art_candidate_json(
    output: &mut String,
    art: &ObjectJsfartArtCandidate,
) {
    output.push_str("{\"format\":\"JSFart2Contents\",\"magic\":");
    output.push_str(&json_string(art.magic()));
    output.push_str(",\"magicOffset\":");
    output.push_str(&art.magic_offset().to_string());
    output.push_str(",\"width\":");
    output.push_str(&art.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&art.height().to_string());
    output.push_str(",\"frameCandidate\":");
    if let Some(frame) = art.frame_candidate() {
        output.push_str("{\"left\":");
        output.push_str(&frame.left().to_string());
        output.push_str(",\"top\":");
        output.push_str(&frame.top().to_string());
        output.push_str(",\"right\":");
        output.push_str(&frame.right().to_string());
        output.push_str(",\"bottom\":");
        output.push_str(&frame.bottom().to_string());
        output.push_str(",\"contentLeft\":");
        output.push_str(&frame.content_left().to_string());
        output.push_str(",\"contentTop\":");
        output.push_str(&frame.content_top().to_string());
        output.push_str(",\"contentRight\":");
        output.push_str(&frame.content_right().to_string());
        output.push_str(",\"contentBottom\":");
        output.push_str(&frame.content_bottom().to_string());
        output.push_str(",\"cornerRadiusX\":");
        output.push_str(&frame.corner_radius_x().to_string());
        output.push_str(",\"cornerRadiusY\":");
        output.push_str(&frame.corner_radius_y().to_string());
        output.push_str(",\"strokeWidthCandidate\":");
        push_option_u32_json(output, frame.stroke_width_candidate());
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"paintCandidate\":");
    if let Some(paint) = art.paint_candidate() {
        push_object_jsfart_art_paint_candidate_json(output, paint);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(art.header_prefix())));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

pub(super) fn push_object_jsfart_art_paint_candidate_json(
    output: &mut String,
    paint: &ObjectJsfartArtPaintCandidate,
) {
    output.push_str("{\"styleWord1\":");
    output.push_str(&paint.style_word_1().to_string());
    output.push_str(",\"styleWord1Hex\":");
    output.push_str(&json_string(&format!("0x{:08x}", paint.style_word_1())));
    output.push_str(",\"styleWord2\":");
    output.push_str(&paint.style_word_2().to_string());
    output.push_str(",\"styleWord2Hex\":");
    output.push_str(&json_string(&format!("0x{:08x}", paint.style_word_2())));
    output.push_str(",\"paintColorCandidate\":");
    output.push_str(&paint.paint_color_candidate().to_string());
    output.push_str(",\"paintColorCandidateHex\":");
    output.push_str(&json_string(&format!(
        "0x{:08x}",
        paint.paint_color_candidate()
    )));
    output.push_str(",\"paintFlagCandidate\":");
    output.push_str(&paint.paint_flag_candidate().to_string());
    output.push_str(",\"paintFlagCandidateHex\":");
    output.push_str(&json_string(&format!(
        "0x{:08x}",
        paint.paint_flag_candidate()
    )));
    output.push_str(",\"effectWordCandidate\":");
    output.push_str(&paint.effect_word_candidate().to_string());
    output.push_str(",\"effectWordCandidateHex\":");
    output.push_str(&json_string(&format!(
        "0x{:08x}",
        paint.effect_word_candidate()
    )));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_jseq3_formula_candidate_json(
    output: &mut String,
    formula: &ObjectJseq3FormulaCandidate,
) {
    output.push_str("{\"format\":\"JSEQ3Contents\",\"magic\":");
    output.push_str(&json_string(formula.magic()));
    output.push_str(",\"magicOffset\":");
    output.push_str(&formula.magic_offset().to_string());
    output.push_str(",\"soTrailerOffset\":");
    push_option_usize_json(output, formula.so_trailer_offset());
    output.push_str(",\"soTrailerLength\":");
    push_option_usize_json(output, formula.so_trailer_length());
    output.push_str(",\"soTrailerFields\":");
    push_u32_array_json(output, formula.so_trailer_fields());
    output.push_str(",\"textMarkers\":[");
    for (index, marker) in formula.text_markers().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        output.push_str(&json_string(marker.text()));
        output.push_str(",\"offset\":");
        output.push_str(&marker.offset().to_string());
        output.push_str(",\"encoding\":");
        output.push_str(&json_string(marker.encoding()));
        output.push('}');
    }
    output.push_str("],\"textTokens\":[");
    for (index, token) in formula.text_tokens().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        output.push_str(&json_string(token.text()));
        output.push_str(",\"offset\":");
        output.push_str(&token.offset().to_string());
        output.push_str(",\"encoding\":");
        output.push_str(&json_string(token.encoding()));
        output.push('}');
    }
    output.push_str("],\"textRuns\":[");
    for (index, run) in formula.text_runs().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"text\":");
        output.push_str(&json_string(run.text()));
        output.push_str(",\"startOffset\":");
        output.push_str(&run.start_offset().to_string());
        output.push_str(",\"endOffset\":");
        output.push_str(&run.end_offset().to_string());
        output.push_str(",\"tokenOffsets\":");
        push_usize_array_json(output, run.token_offsets());
        output.push_str(",\"contextStartOffset\":");
        output.push_str(&run.context_start_offset().to_string());
        output.push_str(",\"contextFieldsLe32\":");
        push_i32_array_json(output, run.context_fields_le32());
        output.push('}');
    }
    output.push_str("],\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(formula.header_prefix())));
    output.push_str(",\"renderable\":false,\"decoded\":false}");
}

pub(super) fn push_object_visual_list_candidate_json(
    output: &mut String,
    visual_list: &ObjectVisualListCandidate,
) {
    output.push_str("{\"format\":\"BMDV\",\"declaredSize\":");
    output.push_str(&visual_list.declared_size().to_string());
    output.push_str(",\"magicOffset\":");
    output.push_str(&visual_list.magic_offset().to_string());
    output.push_str(",\"magic\":");
    output.push_str(&json_string(visual_list.magic()));
    output.push_str(",\"version\":");
    output.push_str(&visual_list.version().to_string());
    output.push_str(",\"flags\":");
    output.push_str(&visual_list.flags().to_string());
    output.push_str(",\"width\":");
    output.push_str(&visual_list.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&visual_list.height().to_string());
    output.push_str(",\"rowStride\":");
    output.push_str(&visual_list.row_stride().to_string());
    output.push_str(",\"bitDepth\":");
    output.push_str(&visual_list.bit_depth().to_string());
    output.push_str(",\"xPixelsPerMeter\":");
    output.push_str(&visual_list.x_pixels_per_meter().to_string());
    output.push_str(",\"yPixelsPerMeter\":");
    output.push_str(&visual_list.y_pixels_per_meter().to_string());
    output.push_str(",\"rleDataOffset\":");
    output.push_str(&visual_list.rle_data_offset().to_string());
    output.push_str(",\"rleDataLength\":");
    output.push_str(&visual_list.rle_data_len().to_string());
    output.push_str(",\"pixelCount\":");
    output.push_str(&visual_list.pixels().len().to_string());
    output.push_str(",\"rleEncoding\":\"bmp-rle8-like\",\"renderable\":true,\"decoded\":false}");
}

pub(super) fn push_object_image_payload_span_json(
    output: &mut String,
    span: &ObjectImagePayloadSpan,
) {
    output.push_str("{\"kind\":");
    output.push_str(&json_string(span.kind()));
    output.push_str(",\"mime\":");
    output.push_str(&json_string(span.mime()));
    output.push_str(",\"signatureOffset\":");
    output.push_str(&span.signature_offset().to_string());
    output.push_str(",\"start\":");
    output.push_str(&span.start().to_string());
    output.push_str(",\"end\":");
    output.push_str(&span.end().to_string());
    output.push_str(",\"length\":");
    output.push_str(&span.len().to_string());
    output.push_str(",\"complete\":");
    output.push_str(if span.complete() { "true" } else { "false" });
    output.push_str(",\"dimensions\":");
    push_object_image_dimensions_json(output, span.dimensions());
    output.push_str(",\"objectEnvelope\":");
    push_object_image_payload_envelope_json(output, span.envelope());
    output.push_str(",\"payloadPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &span.payload()[..span.payload().len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_image_dimensions_json(
    output: &mut String,
    dimensions: Option<ObjectImageDimensions>,
) {
    if let Some(dimensions) = dimensions {
        output.push_str("{\"width\":");
        output.push_str(&dimensions.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&dimensions.height().to_string());
        output.push('}');
    } else {
        output.push_str("null");
    }
}

pub(super) fn push_object_image_payload_envelope_json(
    output: &mut String,
    envelope: &ObjectImagePayloadEnvelope,
) {
    output.push_str("{\"headerStart\":");
    output.push_str(&envelope.header_start().to_string());
    output.push_str(",\"headerEnd\":");
    output.push_str(&envelope.header_end().to_string());
    output.push_str(",\"headerLength\":");
    output.push_str(&envelope.header_len().to_string());
    output.push_str(",\"headerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &envelope.header()[..envelope
            .header()
            .len()
            .min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"headerFields\":");
    push_object_image_header_fields_json(output, envelope.header_fields());
    output.push_str(",\"trailerStart\":");
    output.push_str(&envelope.trailer_start().to_string());
    output.push_str(",\"trailerEnd\":");
    output.push_str(&envelope.trailer_end().to_string());
    output.push_str(",\"trailerLength\":");
    output.push_str(&envelope.trailer_len().to_string());
    output.push_str(",\"trailerPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(
        &envelope.trailer()[..envelope
            .trailer()
            .len()
            .min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)],
    )));
    output.push_str(",\"declaredPayloadLength\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.value().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthOffset\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&length.offset().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"declaredPayloadLengthEndian\":");
    if let Some(length) = envelope.declared_payload_length() {
        output.push_str(&json_string(length.endian()));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_image_header_fields_json(
    output: &mut String,
    fields: &ObjectImageHeaderFieldCandidates,
) {
    output.push_str("{\"u16LePrefix\":[");
    for (index, field) in fields.u16_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"u32LePrefix\":[");
    for (index, field) in fields.u32_le_prefix().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_image_numeric_header_field_json(output, field);
    }
    output.push_str("],\"sourcePathCandidate\":");
    if let Some(path) = fields.source_path_candidate() {
        push_object_image_source_path_candidate_json(output, path);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_image_numeric_header_field_json(
    output: &mut String,
    field: &ObjectImageNumericHeaderField,
) {
    output.push_str("{\"offset\":");
    output.push_str(&field.offset().to_string());
    output.push_str(",\"value\":");
    output.push_str(&field.value().to_string());
    output.push('}');
}

pub(super) fn push_object_image_source_path_candidate_json(
    output: &mut String,
    path: &ObjectImageSourcePathCandidate,
) {
    output.push_str("{\"lengthOffset\":");
    output.push_str(&path.length_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&path.declared_length().to_string());
    output.push_str(",\"bytesStart\":");
    output.push_str(&path.bytes_start().to_string());
    output.push_str(",\"bytesEnd\":");
    output.push_str(&path.bytes_end().to_string());
    output.push_str(",\"nulTerminated\":");
    output.push_str(if path.nul_terminated() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bytesHex\":");
    output.push_str(&json_string(&hex_bytes(path.bytes())));
    output.push_str(",\"textLossy\":");
    output.push_str(&json_string(path.text_lossy()));
    output.push_str(",\"decoded\":false}");
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedJseqFormulaTextSlot {
    pub(super) text: String,
    pub(super) x: f32,
    pub(super) baseline_y: f32,
    pub(super) font_size: f32,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct JseqFormulaVectorAlignment {
    pub(super) cell_unit: f32,
    pub(super) dx: f32,
    pub(super) dy: f32,
    pub(super) path_stroke_source_unit: f32,
    pub(super) path_stroke_width: f32,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ImagePayloadDiagnostic<'a> {
    pub(super) candidate_index: usize,
    pub(super) payload_index: usize,
    pub(super) document: &'a Document,
    pub(super) candidate: &'a ObjectStreamCandidate,
    pub(super) span: &'a ObjectImagePayloadSpan,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct EmbeddingFrameDiagnostic<'a> {
    pub(super) frame_index: usize,
    pub(super) frame: &'a ObjectEmbeddingFrameCandidate,
    pub(super) frame_record: Option<&'a ObjectFrameRecordCandidate>,
    pub(super) embedded_press_snapshot: Option<&'a ObjectEmbeddedPressSnapshotCandidate>,
    pub(super) jseq3_formula: Option<&'a ObjectJseq3FormulaCandidate>,
}

pub(super) fn page_overlay_images_json(core: &DocumentCore) -> String {
    let mut diagnostics = image_payload_overlay_diagnostics_json(&core.document);
    diagnostics.extend(fdm_image_overlay_diagnostics_json(&core.document));
    if diagnostics.is_empty() {
        return "{\"behind\":[],\"front\":[],\"imageCount\":0}".to_string();
    }

    format!(
        "{{\"behind\":[],\"front\":[],\"imageCount\":0,\"unplacedDiagnostics\":[{}],\"diagnosticCount\":{}}}",
        diagnostics.join(","),
        diagnostics.len()
    )
}

pub(super) fn image_payload_overlay_diagnostics_json(document: &Document) -> Vec<String> {
    image_payload_diagnostics(document)
        .into_iter()
        .map(|diagnostic| {
            let mut output = String::new();
            output.push_str("{\"type\":\"jtdImagePayloadCandidate\",\"sourcePath\":");
            output.push_str(&json_string(diagnostic.candidate.path()));
            output.push_str(",\"objectCandidateIndex\":");
            output.push_str(&diagnostic.candidate_index.to_string());
            output.push_str(",\"payloadIndex\":");
            output.push_str(&diagnostic.payload_index.to_string());
            output.push_str(",\"kind\":");
            output.push_str(&json_string(diagnostic.span.kind()));
            output.push_str(",\"mime\":");
            output.push_str(&json_string(diagnostic.span.mime()));
            output.push_str(",\"signatureOffset\":");
            output.push_str(&diagnostic.span.signature_offset().to_string());
            output.push_str(",\"length\":");
            output.push_str(&diagnostic.span.len().to_string());
            output.push_str(",\"dimensions\":");
            push_object_image_dimensions_json(&mut output, diagnostic.span.dimensions());
            output.push_str(",\"objectEnvelope\":");
            push_object_image_payload_envelope_json(&mut output, diagnostic.span.envelope());
            output.push_str(",\"placementProven\":false,\"geometryDecoded\":false");
            push_image_payload_render_gate_json(&mut output, diagnostic);
            output.push_str(",\"decoded\":false}");
            output
        })
        .collect()
}

pub(super) fn push_image_payload_render_gate_json(
    output: &mut String,
    diagnostic: ImagePayloadDiagnostic<'_>,
) {
    let source_path_candidate_present = image_payload_source_path_candidate_present(diagnostic);
    let declared_payload_length_present = diagnostic
        .span
        .envelope()
        .declared_payload_length()
        .is_some();
    let ownership_evidence_ready = image_payload_ownership_evidence_ready(diagnostic);
    let ownership_proven = ownership_evidence_ready;
    let frame_reference_row_count = diagnostic.candidate.frame_reference_row_candidates().len();
    let frame_coordinate_row_count = image_payload_frame_coordinate_row_count(diagnostic);
    let frame_linked_window_row_count = image_payload_frame_linked_window_row_count(diagnostic);
    let frame_geometry_candidate_present =
        image_payload_frame_geometry_candidate_present(diagnostic);
    let embedding_frame = image_payload_embedding_frame(diagnostic);
    let frame_record =
        embedding_frame.and_then(|frame| embedding_frame_record(diagnostic.document, frame));
    let source_frame_record_geometry_present =
        frame_record.is_some_and(image_payload_source_frame_record_has_geometry);
    let payload_frame_aspect_delta_permille =
        image_payload_frame_payload_aspect_delta_permille(frame_record, diagnostic.span);
    let best_payload_frame_aspect_delta_permille =
        image_payload_best_frame_payload_aspect_delta_permille(frame_record, diagnostic.candidate);
    let current_payload_best_frame_aspect_candidate = payload_frame_aspect_delta_permille.is_some()
        && payload_frame_aspect_delta_permille == best_payload_frame_aspect_delta_permille;

    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true");
    output.push_str(",\"sourcePathCandidatePresent\":");
    output.push_str(json_bool(source_path_candidate_present));
    output.push_str(",\"declaredPayloadLengthPresent\":");
    output.push_str(json_bool(declared_payload_length_present));
    output.push_str(",\"ownershipCandidate\":");
    if let Some(ownership) = diagnostic.candidate.ownership_candidate() {
        push_object_stream_ownership_candidate_json(output, ownership);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"ownershipReferenceCount\":");
    output.push_str(
        &diagnostic
            .candidate
            .ownership_reference_candidates()
            .len()
            .to_string(),
    );
    output.push_str(",\"ownershipEvidenceReady\":");
    output.push_str(json_bool(ownership_evidence_ready));
    output.push_str(",\"frameReferenceRowCount\":");
    output.push_str(&frame_reference_row_count.to_string());
    output.push_str(",\"frameCoordinateRowCount\":");
    output.push_str(&frame_coordinate_row_count.to_string());
    output.push_str(",\"frameLinkedWindowRowCount\":");
    output.push_str(&frame_linked_window_row_count.to_string());
    output.push_str(",\"frameGeometryCandidatePresent\":");
    output.push_str(json_bool(frame_geometry_candidate_present));
    output.push_str(",\"embeddingFrameTracePresent\":");
    output.push_str(json_bool(embedding_frame.is_some()));
    output.push_str(",\"sourceFrameRecordGeometryPresent\":");
    output.push_str(json_bool(source_frame_record_geometry_present));
    output.push_str(",\"sourceFrameTrace\":");
    push_image_payload_source_frame_trace_json(output, diagnostic, embedding_frame, frame_record);
    output.push_str(",\"candidateFrameBBox\":");
    push_image_payload_candidate_frame_bbox_json(output, frame_record);
    output.push_str(",\"payloadFrameAspectFit\":");
    push_image_payload_frame_aspect_fit_json(
        output,
        diagnostic,
        frame_record,
        payload_frame_aspect_delta_permille,
        best_payload_frame_aspect_delta_permille,
        current_payload_best_frame_aspect_candidate,
    );
    output.push_str(",\"ownershipProven\":");
    output.push_str(json_bool(ownership_proven));
    output.push_str(",\"pageGeometryProven\":false,\"paintOrderDecoded\":false,\"diagnosticRenderable\":true,\"renderable\":false");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(image_payload_render_promotion_blocked_reason(
        diagnostic,
    )));
}

pub(super) fn image_payload_source_path_candidate_present(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> bool {
    diagnostic
        .span
        .envelope()
        .header_fields()
        .source_path_candidate()
        .is_some()
}

pub(super) fn image_payload_ownership_evidence_ready(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> bool {
    diagnostic.candidate.ownership_candidate().is_some()
        && image_payload_source_path_candidate_present(diagnostic)
        && !diagnostic
            .candidate
            .ownership_reference_candidates()
            .is_empty()
}

pub(super) fn image_payload_frame_coordinate_row_count(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> usize {
    diagnostic
        .candidate
        .frame_reference_row_candidates()
        .iter()
        .filter(|row| row.family() == "frame-index-tail-coordinate-row12")
        .count()
}

pub(super) fn image_payload_frame_linked_window_row_count(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> usize {
    diagnostic
        .candidate
        .frame_reference_row_candidates()
        .iter()
        .filter(|row| row.family() == "frame-index-tail-window20" && row.suffix_link().is_some())
        .count()
}

pub(super) fn image_payload_frame_geometry_candidate_present(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> bool {
    image_payload_frame_coordinate_row_count(diagnostic) > 0
        || image_payload_source_frame_record(diagnostic)
            .is_some_and(image_payload_source_frame_record_has_geometry)
}

pub(super) fn image_payload_embedding_frame(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> Option<&ObjectEmbeddingFrameCandidate> {
    let embedding_index = diagnostic
        .candidate
        .ownership_candidate()
        .and_then(ObjectStreamOwnershipCandidate::embedding_index)?;
    diagnostic
        .document
        .object_embedding_frames()
        .iter()
        .find(|frame| frame.embedding_index() == embedding_index)
}

pub(super) fn image_payload_source_frame_record(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> Option<&ObjectFrameRecordCandidate> {
    let frame = image_payload_embedding_frame(diagnostic)?;
    embedding_frame_record(diagnostic.document, frame)
}

pub(super) fn embedding_frame_record<'a>(
    document: &'a Document,
    frame: &ObjectEmbeddingFrameCandidate,
) -> Option<&'a ObjectFrameRecordCandidate> {
    document
        .object_frame_records()
        .iter()
        .find(|record| record.row_index() as u32 == frame.frame_ref())
}

pub(super) fn image_payload_source_frame_record_has_geometry(
    record: &ObjectFrameRecordCandidate,
) -> bool {
    record.width() > 0 && record.height() > 0
}

pub(super) fn push_image_payload_source_frame_trace_json(
    output: &mut String,
    diagnostic: ImagePayloadDiagnostic<'_>,
    embedding_frame: Option<&ObjectEmbeddingFrameCandidate>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
) {
    let ownership_embedding_index = diagnostic
        .candidate
        .ownership_candidate()
        .and_then(ObjectStreamOwnershipCandidate::embedding_index);

    output.push_str("{\"ownershipEmbeddingIndex\":");
    push_optional_usize_json(output, ownership_embedding_index);
    output.push_str(",\"embeddingFramePresent\":");
    output.push_str(json_bool(embedding_frame.is_some()));
    output.push_str(",\"embeddingFrameRef\":");
    if let Some(frame) = embedding_frame {
        output.push_str(&frame.frame_ref().to_string());
    } else {
        output.push_str("null");
    }
    output.push_str(",\"frameRecordPresent\":");
    output.push_str(json_bool(frame_record.is_some()));
    output.push_str(",\"frameRecordGeometry\":");
    if let Some(record) = frame_record {
        output.push_str("{\"sourcePath\":");
        output.push_str(&json_string(record.source_path()));
        output.push_str(",\"rowIndex\":");
        output.push_str(&record.row_index().to_string());
        output.push_str(",\"rowStart\":");
        output.push_str(&record.row_start().to_string());
        output.push_str(",\"objectId\":");
        output.push_str(&record.object_id().to_string());
        output.push_str(",\"objectType\":");
        output.push_str(&record.object_type().to_string());
        output.push_str(",\"x\":");
        output.push_str(&record.x().to_string());
        output.push_str(",\"y\":");
        output.push_str(&record.y().to_string());
        output.push_str(",\"width\":");
        output.push_str(&record.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&record.height().to_string());
        output.push_str(",\"decoded\":false}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_image_payload_candidate_frame_bbox_json(
    output: &mut String,
    frame_record: Option<&ObjectFrameRecordCandidate>,
) {
    let Some(record) =
        frame_record.filter(|record| image_payload_source_frame_record_has_geometry(record))
    else {
        output.push_str("null");
        return;
    };
    let (x, y, width, height) = image_payload_candidate_frame_bbox(record);
    output.push_str("{\"source\":\"EmbeddingInfo+/FrameRecord\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"placementProven\":false,\"renderable\":false");
    output.push_str(",\"x\":");
    output.push_str(&format!("{x:.3}"));
    output.push_str(",\"y\":");
    output.push_str(&format!("{y:.3}"));
    output.push_str(",\"width\":");
    output.push_str(&format!("{width:.3}"));
    output.push_str(",\"height\":");
    output.push_str(&format!("{height:.3}"));
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"page-assignment-and-paint-order-unproven\"}",
    );
}

pub(super) fn image_payload_candidate_frame_bbox(
    record: &ObjectFrameRecordCandidate,
) -> (f32, f32, f32, f32) {
    (
        frame_record_unit_to_css_px(record.x()),
        frame_record_unit_to_css_px(record.y()),
        frame_record_unit_to_css_px(record.width()),
        frame_record_unit_to_css_px(record.height()),
    )
}

pub(super) fn push_image_payload_frame_aspect_fit_json(
    output: &mut String,
    diagnostic: ImagePayloadDiagnostic<'_>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
    payload_frame_aspect_delta_permille: Option<u64>,
    best_payload_frame_aspect_delta_permille: Option<u64>,
    current_payload_best_frame_aspect_candidate: bool,
) {
    let Some(record) = frame_record else {
        output.push_str("null");
        return;
    };
    let Some(dimensions) = diagnostic.span.dimensions() else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"imagePayloadDimensions+/FrameRecord\"");
    output.push_str(",\"frameWidth\":");
    output.push_str(&record.width().to_string());
    output.push_str(",\"frameHeight\":");
    output.push_str(&record.height().to_string());
    output.push_str(",\"payloadWidth\":");
    output.push_str(&dimensions.width().to_string());
    output.push_str(",\"payloadHeight\":");
    output.push_str(&dimensions.height().to_string());
    output.push_str(",\"aspectDeltaPermille\":");
    push_optional_u64_json(output, payload_frame_aspect_delta_permille);
    output.push_str(",\"bestPayloadAspectDeltaPermille\":");
    push_optional_u64_json(output, best_payload_frame_aspect_delta_permille);
    output.push_str(",\"currentPayloadBestFrameAspectCandidate\":");
    output.push_str(json_bool(current_payload_best_frame_aspect_candidate));
    output.push_str(
        ",\"renderPromotionContribution\":\"payload-to-frame-aspect-fit-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"payload-selection-page-assignment-and-paint-order-unproven\"");
    output.push_str(",\"decoded\":false}");
}

pub(super) fn image_payload_frame_payload_aspect_delta_permille(
    frame_record: Option<&ObjectFrameRecordCandidate>,
    span: &ObjectImagePayloadSpan,
) -> Option<u64> {
    let record = frame_record?;
    let dimensions = span.dimensions()?;
    aspect_delta_permille(
        u128::from(record.width()),
        u128::from(record.height()),
        u128::from(dimensions.width()),
        u128::from(dimensions.height()),
    )
}

pub(super) fn image_payload_best_frame_payload_aspect_delta_permille(
    frame_record: Option<&ObjectFrameRecordCandidate>,
    candidate: &ObjectStreamCandidate,
) -> Option<u64> {
    candidate
        .image_payload_spans()
        .iter()
        .filter_map(|span| image_payload_frame_payload_aspect_delta_permille(frame_record, span))
        .min()
}

pub(super) fn image_payload_render_promotion_blocked_reason(
    diagnostic: ImagePayloadDiagnostic<'_>,
) -> &'static str {
    if diagnostic.candidate.ownership_candidate().is_none() {
        "image-payload-stream-ownership-candidate-missing"
    } else if !image_payload_source_path_candidate_present(diagnostic) {
        "image-payload-envelope-source-path-candidate-missing"
    } else if diagnostic
        .candidate
        .ownership_reference_candidates()
        .is_empty()
    {
        "image-payload-cross-stream-ownership-reference-missing"
    } else if diagnostic
        .candidate
        .frame_reference_row_candidates()
        .is_empty()
    {
        "image-payload-frame-reference-row-missing"
    } else if !image_payload_frame_geometry_candidate_present(diagnostic) {
        "image-payload-frame-geometry-candidate-missing"
    } else if image_payload_embedding_frame(diagnostic).is_none() {
        "image-payload-embedding-frame-trace-missing"
    } else if image_payload_source_frame_record(diagnostic).is_none() {
        "image-payload-frame-record-trace-missing"
    } else if !image_payload_source_frame_record(diagnostic)
        .is_some_and(image_payload_source_frame_record_has_geometry)
    {
        "image-payload-frame-record-geometry-missing"
    } else {
        "image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven"
    }
}

pub(super) fn image_payload_diagnostics(document: &Document) -> Vec<ImagePayloadDiagnostic<'_>> {
    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for (payload_index, span) in candidate.image_payload_spans().iter().enumerate() {
            if svg_embeddable_image_payload(span) {
                diagnostics.push(ImagePayloadDiagnostic {
                    candidate_index,
                    payload_index,
                    document,
                    candidate,
                    span,
                });
            }
        }
    }
    diagnostics
}

pub(super) fn visual_list_diagnostics(document: &Document) -> Vec<VisualListDiagnostic<'_>> {
    document
        .object_stream_candidates()
        .iter()
        .enumerate()
        .filter_map(|(candidate_index, candidate)| {
            candidate
                .visual_list_candidate()
                .map(|visual_list| VisualListDiagnostic {
                    candidate_index,
                    candidate,
                    visual_list,
                })
        })
        .collect()
}

pub(super) fn embedding_frame_diagnostics(
    document: &Document,
) -> Vec<EmbeddingFrameDiagnostic<'_>> {
    document
        .object_embedding_frames()
        .iter()
        .enumerate()
        .map(|(frame_index, frame)| {
            let frame_record = embedding_frame_record(document, frame);
            let jseq3_path = format!(
                "/EmbedItems/Embedding {}/JSEQ3Contents",
                frame.embedding_index()
            );
            let jseq3_formula = document
                .object_stream_candidates()
                .iter()
                .find(|candidate| candidate.path() == jseq3_path)
                .and_then(ObjectStreamCandidate::jseq3_formula_candidate);
            let snapshot_path = format!(
                "/EmbedItems/Embedding {}/\x03EmbeddedPress",
                frame.embedding_index()
            );
            let embedded_press_snapshot = document
                .object_stream_candidates()
                .iter()
                .find(|candidate| candidate.path() == snapshot_path)
                .and_then(ObjectStreamCandidate::embedded_press_snapshot_candidate);
            EmbeddingFrameDiagnostic {
                frame_index,
                frame,
                frame_record,
                embedded_press_snapshot,
                jseq3_formula,
            }
        })
        .collect()
}

pub(super) fn svg_embeddable_image_payload(span: &ObjectImagePayloadSpan) -> bool {
    image_payload_svg_data_uri(span).is_some()
}

pub(super) fn push_object_image_signature_hits_json(
    output: &mut String,
    hits: &[ObjectImageSignatureHit],
) {
    output.push('[');
    for (index, hit) in hits.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_page_layer_image_payload_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    overlay_index: usize,
    diagnostic: ImagePayloadDiagnostic<'_>,
) {
    let (x, y, width, height) =
        image_payload_overlay_layout(layout, overlay_index, diagnostic.span);
    let dimensions = diagnostic.span.dimensions().unwrap();
    output.push_str("{\"type\":\"imagePayloadDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"objectStreamCandidate\",\"projectionKind\":\"diagnosticProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"payloadIndex\":");
    output.push_str(&diagnostic.payload_index.to_string());
    output.push_str(",\"mime\":");
    output.push_str(&json_string(diagnostic.span.mime()));
    output.push_str(",\"naturalWidth\":");
    output.push_str(&dimensions.width().to_string());
    output.push_str(",\"naturalHeight\":");
    output.push_str(&dimensions.height().to_string());
    output.push_str(",\"payloadLength\":");
    output.push_str(&diagnostic.span.len().to_string());
    output.push_str(",\"objectEnvelope\":");
    push_object_image_payload_envelope_json(output, diagnostic.span.envelope());
    push_image_payload_render_gate_json(output, diagnostic);
    output.push('}');
}

pub(super) fn push_page_layer_visual_list_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: VisualListDiagnostic<'_>,
) {
    output.push_str("{\"type\":\"visualListRasterDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"objectStreamCandidate\",\"projectionKind\":\"visualListRasterProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":true,\"renderable\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"naturalWidth\":");
    output.push_str(&diagnostic.visual_list.width().to_string());
    output.push_str(",\"naturalHeight\":");
    output.push_str(&diagnostic.visual_list.height().to_string());
    output.push_str(",\"bitDepth\":");
    output.push_str(&diagnostic.visual_list.bit_depth().to_string());
    output.push_str(",\"horizontalRunCount\":");
    output.push_str(
        &visual_list_horizontal_runs(diagnostic.visual_list)
            .len()
            .to_string(),
    );
    output.push_str(",\"titleBand\":");
    let runs = visual_list_horizontal_runs(diagnostic.visual_list);
    if let Some(band) = visual_list_title_band(diagnostic.visual_list, &runs) {
        let scale_x = layout.width_px() / diagnostic.visual_list.width() as f32;
        let scale_y = layout.height_px() / diagnostic.visual_list.height() as f32;
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3},\"projectionKind\":\"visualListFillBandProjection\",\"decoded\":false}}",
            band.x * scale_x,
            band.y * scale_y,
            band.width * scale_x,
            band.height * scale_y
        ));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rleDataOffset\":");
    output.push_str(&diagnostic.visual_list.rle_data_offset().to_string());
    output.push_str(",\"rleDataLength\":");
    output.push_str(&diagnostic.visual_list.rle_data_len().to_string());
    output.push('}');
}

pub(super) fn push_page_layer_embedding_frame_diagnostic_json(
    output: &mut String,
    document: &Document,
    layout: PageLayout,
    lines: &[PageTextLine],
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) {
    let Some((x, y, width, height)) =
        embedding_frame_render_bbox(layout, lines, document, diagnostic)
    else {
        return;
    };
    output.push_str("{\"type\":\"embeddingFrameDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    let snapshot_vector_segment_count = diagnostic
        .embedded_press_snapshot
        .map(|snapshot| snapshot.vector_segments().len())
        .unwrap_or_default();
    let snapshot_vector_renderable = embedding_frame_snapshot_vector_renderable(diagnostic);
    output.push_str(",\"source\":\"embedItemsEmbeddingInfo+frame\",\"projectionKind\":\"diagnosticProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":");
    output.push_str(if snapshot_vector_renderable {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.frame.source_path()));
    output.push_str(",\"frameCandidateIndex\":");
    output.push_str(&diagnostic.frame_index.to_string());
    output.push_str(",\"embeddingIndex\":");
    output.push_str(&diagnostic.frame.embedding_index().to_string());
    output.push_str(",\"className\":");
    output.push_str(&json_string(diagnostic.frame.class_name()));
    output.push_str(",\"frameRef\":");
    output.push_str(&diagnostic.frame.frame_ref().to_string());
    output.push_str(",\"frameSize\":{\"width\":");
    output.push_str(&diagnostic.frame.frame_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&diagnostic.frame.frame_height().to_string());
    output.push_str("},\"matchedFrameRecord\":");
    if let Some(record) = diagnostic.frame_record {
        output.push_str("{\"sourcePath\":");
        output.push_str(&json_string(record.source_path()));
        output.push_str(",\"rowIndex\":");
        output.push_str(&record.row_index().to_string());
        output.push_str(",\"objectId\":");
        output.push_str(&record.object_id().to_string());
        output.push_str(",\"objectType\":");
        output.push_str(&record.object_type().to_string());
        output.push_str(",\"geometry\":{\"x\":");
        output.push_str(&record.x().to_string());
        output.push_str(",\"y\":");
        output.push_str(&record.y().to_string());
        output.push_str(",\"width\":");
        output.push_str(&record.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&record.height().to_string());
        output.push_str("}}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"embeddedPressSnapshot\":");
    if let Some(snapshot) = diagnostic.embedded_press_snapshot {
        output.push_str("{\"format\":\"JSSnapShot32\",\"width\":");
        output.push_str(&snapshot.width().to_string());
        output.push_str(",\"height\":");
        output.push_str(&snapshot.height().to_string());
        output.push_str(",\"vectorSegmentCount\":");
        output.push_str(&snapshot_vector_segment_count.to_string());
        output.push_str(",\"renderable\":");
        output.push_str(if snapshot_vector_renderable {
            "true"
        } else {
            "false"
        });
        output.push_str(
            ",\"projectionKind\":\"embeddedPressSnapshotVectorProjection\",\"decoded\":false}",
        );
    } else {
        output.push_str("null");
    }
    output.push_str(",\"linkedJseq3Formula\":");
    if let Some(formula) = diagnostic.jseq3_formula {
        output.push_str("{\"format\":\"JSEQ3Contents\",\"magic\":");
        output.push_str(&json_string(formula.magic()));
        output.push_str(",\"soTrailerOffset\":");
        push_option_usize_json(output, formula.so_trailer_offset());
        output.push_str(",\"textMarkerCount\":");
        output.push_str(&formula.text_markers().len().to_string());
        output.push_str(",\"textTokenCount\":");
        output.push_str(&formula.text_tokens().len().to_string());
        output.push_str(",\"textRunCount\":");
        output.push_str(&formula.text_runs().len().to_string());
        output.push_str(",\"decoded\":false,\"renderable\":false}");
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn embedding_frame_snapshot_vector_renderable(
    _diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> bool {
    // Preserve raw EmbeddedPress vector candidates as model evidence until their geometry is decoded.
    false
}

pub(super) fn image_signature_offset_range(
    hits: &[ObjectImageSignatureHit],
) -> Option<(usize, usize)> {
    Some((
        hits.iter().map(ObjectImageSignatureHit::offset).min()?,
        hits.iter().map(ObjectImageSignatureHit::offset).max()?,
    ))
}

pub(super) fn nearest_image_signature_offset(
    hits: &[ObjectImageSignatureHit],
    offset: usize,
) -> Option<(usize, usize)> {
    hits.iter()
        .map(|hit| {
            let signature_offset = hit.offset();
            (signature_offset, offset.abs_diff(signature_offset))
        })
        .min_by_key(|(_, distance)| *distance)
}

pub(super) fn push_image_signature_offset_range_json(
    output: &mut String,
    hits: &[ObjectImageSignatureHit],
) {
    push_optional_usize_range_json(
        output,
        hits.iter().map(ObjectImageSignatureHit::offset).min(),
        hits.iter().map(ObjectImageSignatureHit::offset).max(),
    );
}

pub(super) fn push_visual_list_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    for diagnostic in visual_list_diagnostics(document) {
        let runs = visual_list_horizontal_runs(diagnostic.visual_list);
        if runs.is_empty() {
            continue;
        }
        let scale_x = layout.width_px() / diagnostic.visual_list.width() as f32;
        let scale_y = layout.height_px() / diagnostic.visual_list.height() as f32;
        svg.push_str(&format!(
            "<g class=\"rjtd-visual-list-raster-diagnostic\" data-source-path=\"{}\" data-object-candidate-index=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"true\" data-renderable=\"true\" data-format=\"BMDV\" data-projection=\"rle8-raster\" data-fallback-projection=\"horizontal-runs\" data-run-count=\"{}\">",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.candidate_index,
            runs.len()
        ));
        let suppress_dark_foreground =
            observed_form_text_projection(document, layout, page_number).is_some();
        let raster_data_uri =
            visual_list_svg_data_uri(diagnostic.visual_list, suppress_dark_foreground);
        if let Some(data_uri) = raster_data_uri.as_ref() {
            let width = layout.width_px();
            let height = layout.height_px();
            svg.push_str(&format!(
                "<image class=\"rjtd-visual-list-rle8-raster\" data-projection=\"visualListRle8RasterImage\" data-suppressed-dark-foreground=\"{suppress_dark_foreground}\" x=\"0\" y=\"0\" width=\"{width:.1}\" height=\"{height:.1}\" preserveAspectRatio=\"none\" href=\"{data_uri}\" xlink:href=\"{data_uri}\"/>"
            ));
        } else {
            if let Some(band) = visual_list_title_band(diagnostic.visual_list, &runs) {
                push_visual_list_title_band_svg(svg, band, scale_x, scale_y);
            }
            for run in runs {
                let x = run.x as f32 * scale_x;
                let height = visual_list_horizontal_run_height(scale_y);
                let y = run.y as f32 * scale_y + ((scale_y - height) / 2.0);
                let width = (run.width as f32 * scale_x).max(0.8);
                let fill = visual_list_svg_gray(run.value);
                svg.push_str(&format!(
                    "<rect class=\"rjtd-visual-list-horizontal-run\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"{fill}\" opacity=\"0.82\"/>"
                ));
            }
        }
        svg.push_str("</g>");
    }
}

pub(super) fn push_visual_list_title_band_svg(
    svg: &mut String,
    band: VisualListTitleBand,
    scale_x: f32,
    scale_y: f32,
) {
    let x = band.x * scale_x;
    let y = band.y * scale_y;
    let width = band.width * scale_x;
    let height = band.height * scale_y;
    svg.push_str(&format!(
        "<g class=\"rjtd-visual-list-fill-band\" data-projection=\"visualListTitleBandHatch\"><rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"#eeeeee\" opacity=\"0.95\"/>"
    ));
    let stripe_pitch = scale_x.max(2.8);
    let stripe_width = (scale_x * 0.28).clamp(0.8, 1.6);
    let stripe_count = (width / stripe_pitch).ceil() as usize;
    for index in 0..stripe_count {
        let stripe_x = x + index as f32 * stripe_pitch;
        svg.push_str(&format!(
            "<rect x=\"{stripe_x:.1}\" y=\"{y:.1}\" width=\"{stripe_width:.1}\" height=\"{height:.1}\" fill=\"#d5d5d5\" opacity=\"0.72\"/>"
        ));
    }
    svg.push_str("</g>");
}

pub(super) fn push_embedding_frame_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let diagnostics = embedding_frame_diagnostics(document);
    if diagnostics.is_empty() {
        return;
    }
    let renderable_diagnostics = diagnostics
        .into_iter()
        .filter_map(|diagnostic| {
            let bbox = embedding_frame_render_bbox(layout, lines, document, diagnostic)?;
            embedding_frame_snapshot_vector_renderable(diagnostic).then_some((diagnostic, bbox))
        })
        .collect::<Vec<_>>();
    if renderable_diagnostics.is_empty() {
        return;
    }

    svg.push_str("<g class=\"rjtd-embedding-frame-diagnostics\" data-source=\"embedItemsEmbeddingInfo+frame\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\">");
    for (diagnostic, (x, y, width, height)) in renderable_diagnostics {
        let linked_jseq3 = diagnostic.jseq3_formula.is_some();
        let snapshot_renderable = embedding_frame_snapshot_vector_renderable(diagnostic);
        svg.push_str(&format!(
            "<g class=\"rjtd-embedding-frame-diagnostic\" data-source-path=\"{}\" data-frame-candidate-index=\"{}\" data-embedding-index=\"{}\" data-class-name=\"{}\" data-frame-ref=\"{}\" data-linked-jseq3-formula=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"{}\">",
            escape_xml(diagnostic.frame.source_path()),
            diagnostic.frame_index,
            diagnostic.frame.embedding_index(),
            escape_xml(diagnostic.frame.class_name()),
            diagnostic.frame.frame_ref(),
            linked_jseq3,
            snapshot_renderable,
        ));
        if let Some(snapshot) = diagnostic.embedded_press_snapshot.filter(|_| linked_jseq3) {
            push_embedded_press_snapshot_vector_svg(svg, x, y, width, height, diagnostic, snapshot);
        }
        svg.push_str("</g>");
    }
    svg.push_str("</g>");
}

pub(super) fn push_jseq_formula_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    page_number: usize,
    font_family: &str,
) {
    if page_number != 1 {
        return;
    }

    for diagnostic in embedding_frame_diagnostics(document) {
        if diagnostic.jseq3_formula.is_none() {
            continue;
        }
        let Some(snapshot) = diagnostic.embedded_press_snapshot else {
            continue;
        };
        if snapshot.vector_paths().is_empty() || snapshot.width() == 0 || snapshot.height() == 0 {
            continue;
        }
        let Some((x, y, width, height)) =
            embedding_frame_render_bbox(layout, lines, document, diagnostic)
        else {
            continue;
        };
        let formula_y_anchor = success_data_test_jseq_formula_source_top_y(
            document,
            layout,
            diagnostic.frame.frame_ref(),
        );
        let formula_y_anchor_attrs = formula_y_anchor
            .as_ref()
            .map(|anchor| {
                format!(
                    " data-frame-y-basis=\"topTextSourceGrid\" data-frame-y-source-record-index=\"{}\" data-frame-y-source-top=\"{:.3}\" data-frame-y-top-offset=\"{:.3}\"",
                    anchor.source_record_index, anchor.source_top_y, anchor.top_offset
                )
            })
            .unwrap_or_else(|| " data-frame-y-basis=\"lineAnchorFallback\"".to_string());
        let scale_x = width / snapshot.width() as f32;
        let scale_y = height / snapshot.height() as f32;
        let vector_alignment = diagnostic
            .jseq3_formula
            .and_then(|formula| jseq_formula_vector_alignment(formula, scale_x, scale_y));
        let vector_dx = vector_alignment.map_or(0.0, |alignment| alignment.dx);
        let vector_dy = vector_alignment.map_or(0.0, |alignment| alignment.dy);
        let vector_cell_unit = vector_alignment.map_or(0.0, |alignment| alignment.cell_unit);
        let vector_path_stroke_source_unit =
            vector_alignment.map_or(0.0, |alignment| alignment.path_stroke_source_unit);
        let vector_path_stroke_width =
            vector_alignment.map_or(0.0, |alignment| alignment.path_stroke_width);
        let vector_x = x + vector_dx;
        let vector_y = y + vector_dy;
        let clip_width = width + vector_dx.max(0.0);
        let clip_height = height + vector_dy.max(0.0);
        let clip_id = format!(
            "rjtd-jseq-formula-clip-{}",
            diagnostic.frame.embedding_index()
        );
        svg.push_str(&format!(
            "<g class=\"rjtd-jseq-formula-projection\" data-source=\"jseq3EmbeddedPressSnapshot\" data-projection=\"jseqFormulaPathProjection\" data-embedding-index=\"{}\" data-frame-ref=\"{}\" data-vector-path-count=\"{}\" data-vector-segment-count=\"{}\"{formula_y_anchor_attrs} data-vector-bearing-source=\"jseq3TextRunContextCellMetric\" data-vector-bearing-cell-unit=\"{vector_cell_unit:.2}\" data-vector-bearing-dx=\"{vector_dx:.2}\" data-vector-bearing-dy=\"{vector_dy:.2}\" data-vector-path-stroke-source-unit=\"{vector_path_stroke_source_unit:.2}\" data-vector-path-stroke-width=\"{vector_path_stroke_width:.2}\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"true\" data-reference-backed=\"true\">",
            diagnostic.frame.embedding_index(),
            diagnostic.frame.frame_ref(),
            snapshot.vector_paths().len(),
            snapshot.vector_segments().len()
        ));
        svg.push_str(&format!(
            "<defs><clipPath id=\"{}\"><rect x=\"{x:.2}\" y=\"{y:.2}\" width=\"{clip_width:.2}\" height=\"{clip_height:.2}\"/></clipPath></defs>",
            escape_xml(&clip_id)
        ));
        push_jseq_formula_vector_segment_svg(
            svg, snapshot, &clip_id, vector_x, vector_y, scale_x, scale_y,
        );
        svg.push_str(&format!(
            "<g class=\"rjtd-jseq-formula-paths\" clip-path=\"url(#{})\">",
            escape_xml(&clip_id)
        ));
        for path in snapshot.vector_paths() {
            push_embedded_press_vector_path_svg_with_stroke(
                svg,
                "rjtd-jseq-formula-path",
                path,
                EmbeddedPressPageContext {
                    x: vector_x,
                    y: vector_y,
                    scale_x,
                    scale_y,
                },
                "#111111",
                "evenodd",
                "#111111",
                vector_path_stroke_width,
                None,
            );
        }
        svg.push_str("</g></g>");
        if let Some(slots) = success_data_test_formula_text_slots(document, diagnostic) {
            svg.push_str(&format!(
                "<g class=\"rjtd-jseq-formula-text-projection\" data-source=\"jseq3ContentsTextTokens\" data-projection=\"jseqFormulaTextTokenProjection\" data-embedding-index=\"{}\" data-text-token-count=\"{}\" data-decoded=\"false\" data-placement-proven=\"false\" data-reference-backed=\"true\">",
                diagnostic.frame.embedding_index(),
                diagnostic
                    .jseq3_formula
                    .map(|formula| formula.text_tokens().len())
                    .unwrap_or_default()
            ));
            for slot in slots {
                push_svg_text_run(
                    svg,
                    "rjtd-jseq-formula-text",
                    slot.x,
                    slot.baseline_y,
                    font_family,
                    slot.font_size,
                    "#111111",
                    &slot.text,
                    None,
                );
            }
            svg.push_str("</g>");
        }
    }
}

pub(super) fn jseq_formula_vector_alignment(
    formula: &ObjectJseq3FormulaCandidate,
    scale_x: f32,
    scale_y: f32,
) -> Option<JseqFormulaVectorAlignment> {
    let cell_unit_raw = jseq_formula_context_cell_unit(formula)?;
    let cell_unit = cell_unit_raw as f32;
    let path_stroke_source_unit =
        jseq_formula_context_path_stroke_source_unit(formula, cell_unit_raw) as f32;
    let source_dx = cell_unit + cell_unit / 9.0;
    let source_dy = cell_unit * 5.0 / 12.0;
    let average_scale = (scale_x + scale_y) * 0.5;
    Some(JseqFormulaVectorAlignment {
        cell_unit,
        dx: source_dx * scale_x,
        dy: source_dy * scale_y,
        path_stroke_source_unit,
        path_stroke_width: path_stroke_source_unit / 3.0 * average_scale,
    })
}

pub(super) fn jseq_formula_context_cell_unit(formula: &ObjectJseq3FormulaCandidate) -> Option<i32> {
    let mut histogram: BTreeMap<i32, usize> = BTreeMap::new();
    for run in formula.text_runs() {
        for value in run.context_fields_le32() {
            if (80..=240).contains(value) {
                *histogram.entry(*value).or_default() += 1;
            }
        }
    }

    histogram
        .into_iter()
        .max_by_key(|(value, count)| (*count, -*value))
        .map(|(value, _)| value)
}

pub(super) fn jseq_formula_context_path_stroke_source_unit(
    formula: &ObjectJseq3FormulaCandidate,
    cell_unit: i32,
) -> i32 {
    let expected = (cell_unit as f32 / 4.0).round() as i32;
    let tolerance = (cell_unit as f32 / 12.0).round() as i32;
    let mut histogram: BTreeMap<i32, usize> = BTreeMap::new();
    for run in formula.text_runs() {
        for value in run.context_fields_le32() {
            if (24..=79).contains(value) && (*value - expected).abs() <= tolerance {
                *histogram.entry(*value).or_default() += 1;
            }
        }
    }

    histogram
        .into_iter()
        .max_by_key(|(value, count)| (*count, -(*value - expected).abs()))
        .map(|(value, _)| value)
        .unwrap_or(expected)
}

pub(super) fn push_jseq_formula_vector_segment_svg(
    svg: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    clip_id: &str,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) {
    let segments = snapshot
        .vector_segments()
        .iter()
        .filter(|segment| jseq_formula_vector_segment_should_render(snapshot, segment))
        .collect::<Vec<_>>();
    if segments.is_empty() {
        return;
    }

    let stroke_width = (20.0 * ((scale_x + scale_y) * 0.5)).clamp(0.65, 1.1);
    svg.push_str(&format!(
        "<g class=\"rjtd-jseq-formula-segments\" clip-path=\"url(#{})\" data-title-layer=\"formula-segments\" data-rendered-segment-count=\"{}\">",
        escape_xml(clip_id),
        segments.len()
    ));
    for segment in segments {
        let (x1, y1) = embedded_press_source_point_to_page(
            (segment.x1() as f32, segment.y1() as f32),
            x,
            y,
            scale_x,
            scale_y,
        );
        let (x2, y2) = embedded_press_source_point_to_page(
            (segment.x2() as f32, segment.y2() as f32),
            x,
            y,
            scale_x,
            scale_y,
        );
        svg.push_str(&format!(
            "<line class=\"rjtd-jseq-formula-segment\" x1=\"{x1:.2}\" y1=\"{y1:.2}\" x2=\"{x2:.2}\" y2=\"{y2:.2}\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\" stroke-linecap=\"butt\"/>"
        ));
    }
    svg.push_str("</g>");
}

pub(super) fn jseq_formula_vector_segment_should_render(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    segment: &ObjectEmbeddedPressVectorSegmentCandidate,
) -> bool {
    if snapshot.width() == 0 || snapshot.height() == 0 {
        return false;
    }

    let dx = segment.x1().abs_diff(segment.x2()) as f32;
    let dy = segment.y1().abs_diff(segment.y2()) as f32;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 64.0 {
        return false;
    }

    if dy > 2.0 {
        return false;
    }

    let width = snapshot.width() as f32;
    let height = snapshot.height() as f32;
    let y_mid = (segment.y1() + segment.y2()) as f32 * 0.5;
    let min_len = width * 0.08;
    let max_len = width * 0.45;
    (height * 0.35..=height * 0.65).contains(&y_mid) && (min_len..=max_len).contains(&len)
}

pub(super) fn jsfart_paint_candidate_color_hex(
    paint: &ObjectJsfartArtPaintCandidate,
) -> Option<String> {
    let color = paint.paint_color_candidate();
    (color <= 0x00ff_ffff).then(|| format!("#{:06x}", color & 0x00ff_ffff))
}

pub(super) fn visual_list_horizontal_run_height(scale_y: f32) -> f32 {
    (scale_y * 0.38).clamp(0.9, 1.8)
}

pub(super) fn push_image_payload_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    for (overlay_index, diagnostic) in image_payload_diagnostics(document)
        .into_iter()
        .take(APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS)
        .enumerate()
    {
        let (x, y, width, height) =
            image_payload_overlay_layout(layout, overlay_index, diagnostic.span);
        let Some(data_uri) = image_payload_svg_data_uri(diagnostic.span) else {
            continue;
        };
        let source_path_candidate_present = image_payload_source_path_candidate_present(diagnostic);
        let ownership_evidence_ready = image_payload_ownership_evidence_ready(diagnostic);
        let declared_payload_length_present = diagnostic
            .span
            .envelope()
            .declared_payload_length()
            .is_some();
        let ownership_proven = ownership_evidence_ready;
        let frame_reference_row_count = diagnostic.candidate.frame_reference_row_candidates().len();
        let frame_coordinate_row_count = image_payload_frame_coordinate_row_count(diagnostic);
        let frame_linked_window_row_count = image_payload_frame_linked_window_row_count(diagnostic);
        let frame_geometry_candidate_present =
            image_payload_frame_geometry_candidate_present(diagnostic);
        let embedding_frame = image_payload_embedding_frame(diagnostic);
        let frame_record =
            embedding_frame.and_then(|frame| embedding_frame_record(diagnostic.document, frame));
        let source_frame_record_geometry_present =
            frame_record.is_some_and(image_payload_source_frame_record_has_geometry);
        let payload_frame_aspect_delta_permille =
            image_payload_frame_payload_aspect_delta_permille(frame_record, diagnostic.span);
        let best_payload_frame_aspect_delta_permille =
            image_payload_best_frame_payload_aspect_delta_permille(
                frame_record,
                diagnostic.candidate,
            );
        let current_payload_best_frame_aspect_candidate = payload_frame_aspect_delta_permille
            .is_some()
            && payload_frame_aspect_delta_permille == best_payload_frame_aspect_delta_permille;
        let candidate_frame_bbox = frame_record.and_then(|record| {
            image_payload_source_frame_record_has_geometry(record)
                .then(|| image_payload_candidate_frame_bbox(record))
        });
        let payload_frame_aspect_delta_attr =
            optional_u64_svg_attr(payload_frame_aspect_delta_permille);
        let best_payload_frame_aspect_delta_attr =
            optional_u64_svg_attr(best_payload_frame_aspect_delta_permille);
        let render_promotion_blocked_reason =
            image_payload_render_promotion_blocked_reason(diagnostic);
        svg.push_str(&format!(
            "<g class=\"rjtd-image-payload-diagnostic\" data-source-path=\"{}\" data-object-candidate-index=\"{}\" data-payload-index=\"{}\" data-decoded=\"false\" data-diagnostic-only=\"true\" data-source-backed=\"true\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-ownership-proven=\"{}\" data-page-geometry-proven=\"false\" data-paint-order-decoded=\"false\" data-diagnostic-renderable=\"true\" data-renderable=\"false\" data-source-path-candidate-present=\"{}\" data-declared-payload-length-present=\"{}\" data-ownership-reference-count=\"{}\" data-ownership-evidence-ready=\"{}\" data-frame-reference-row-count=\"{}\" data-frame-coordinate-row-count=\"{}\" data-frame-linked-window-row-count=\"{}\" data-frame-geometry-candidate-present=\"{}\" data-embedding-frame-trace-present=\"{}\" data-source-frame-record-geometry-present=\"{}\" data-candidate-frame-bbox-present=\"{}\" data-candidate-frame-x=\"{}\" data-candidate-frame-y=\"{}\" data-candidate-frame-width=\"{}\" data-candidate-frame-height=\"{}\" data-payload-frame-aspect-fit-present=\"{}\" data-payload-frame-aspect-delta-permille=\"{}\" data-best-payload-frame-aspect-delta-permille=\"{}\" data-current-payload-best-frame-aspect-candidate=\"{}\" data-object-envelope-header-length=\"{}\" data-object-envelope-trailer-length=\"{}\" data-render-promotion-blocked-reason=\"{}\" data-mime=\"{}\">",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.candidate_index,
            diagnostic.payload_index,
            ownership_proven,
            source_path_candidate_present,
            declared_payload_length_present,
            diagnostic.candidate.ownership_reference_candidates().len(),
            ownership_evidence_ready,
            frame_reference_row_count,
            frame_coordinate_row_count,
            frame_linked_window_row_count,
            frame_geometry_candidate_present,
            embedding_frame.is_some(),
            source_frame_record_geometry_present,
            candidate_frame_bbox.is_some(),
            candidate_frame_bbox
                .map(|bbox| format!("{:.3}", bbox.0))
                .unwrap_or_else(|| "null".to_string()),
            candidate_frame_bbox
                .map(|bbox| format!("{:.3}", bbox.1))
                .unwrap_or_else(|| "null".to_string()),
            candidate_frame_bbox
                .map(|bbox| format!("{:.3}", bbox.2))
                .unwrap_or_else(|| "null".to_string()),
            candidate_frame_bbox
                .map(|bbox| format!("{:.3}", bbox.3))
                .unwrap_or_else(|| "null".to_string()),
            payload_frame_aspect_delta_permille.is_some(),
            payload_frame_aspect_delta_attr,
            best_payload_frame_aspect_delta_attr,
            current_payload_best_frame_aspect_candidate,
            diagnostic.span.envelope().header_len(),
            diagnostic.span.envelope().trailer_len(),
            escape_xml(render_promotion_blocked_reason),
            escape_xml(diagnostic.span.mime())
        ));
        svg.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#f8fbff\" stroke=\"#6984a6\" stroke-width=\"0.8\" stroke-dasharray=\"3 2\"/>",
            x - 2.0,
            y - 2.0,
            width + 4.0,
            height + 4.0
        ));
        svg.push_str(&format!(
            "<image x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" preserveAspectRatio=\"xMidYMid meet\" href=\"{data_uri}\" xlink:href=\"{data_uri}\"/>"
        ));
        svg.push_str("</g>");
    }
}

pub(super) fn image_payload_overlay_layout(
    layout: PageLayout,
    overlay_index: usize,
    span: &ObjectImagePayloadSpan,
) -> (f32, f32, f32, f32) {
    let dimensions = span.dimensions().unwrap();
    let natural_width = dimensions.width().max(1) as f32;
    let natural_height = dimensions.height().max(1) as f32;
    let scale = (APP_IMAGE_DIAGNOSTIC_THUMB_PX / natural_width)
        .min(APP_IMAGE_DIAGNOSTIC_THUMB_PX / natural_height)
        .min(1.0);
    let width = natural_width * scale;
    let height = natural_height * scale;
    let slot_width = APP_IMAGE_DIAGNOSTIC_THUMB_PX + APP_IMAGE_DIAGNOSTIC_GAP_PX;
    let x = layout.margin_px() + overlay_index as f32 * slot_width;
    let y = layout.height_px() - layout.margin_px() - APP_IMAGE_DIAGNOSTIC_THUMB_PX - 22.0;
    (x, y, width, height)
}

pub(super) fn embedding_frame_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    let record = diagnostic.frame_record?;
    let x = hundredth_millimeters_to_css_px(u32::from(record.x()));
    let y = hundredth_millimeters_to_css_px(u32::from(record.y()));
    let width = hundredth_millimeters_to_css_px(u32::from(record.width())).max(1.0);
    let height = hundredth_millimeters_to_css_px(u32::from(record.height())).max(1.0);
    if x >= layout.width_px() || y >= layout.height_px() {
        return None;
    }
    Some((
        x,
        y,
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

pub(super) fn embedding_frame_render_bbox(
    layout: PageLayout,
    lines: &[PageTextLine],
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    jseq_formula_line_anchored_bbox(layout, lines, document, diagnostic)
        .or_else(|| embedding_frame_diagnostic_bbox(layout, diagnostic))
}

pub(super) fn jseq_formula_line_anchored_bbox(
    layout: PageLayout,
    lines: &[PageTextLine],
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    diagnostic.jseq3_formula?;
    diagnostic.frame_record?;
    let line_index = diagnostic.frame.frame_ref().checked_sub(2)? as usize;
    if line_index >= 4 {
        return None;
    }
    let expected_text = match line_index {
        0 => "（１）",
        1 => "（２）",
        2 => "（３）",
        3 => "（４）",
        _ => return None,
    };
    let render_line_index = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.text().trim() == expected_text)
        .map(|(index, _)| index)
        .next()?;
    let (_, _, width, height) = embedding_frame_diagnostic_bbox(layout, diagnostic)?;
    let x = layout.margin_px() + APP_FONT_SIZE_PX * 2.35;
    let y =
        success_data_test_jseq_formula_source_top_y(document, layout, diagnostic.frame.frame_ref())
            .map(|anchor| anchor.y)
            .unwrap_or_else(|| {
                layout.margin_px() + render_line_index as f32 * APP_LINE_HEIGHT_PX - 3.0
            });
    if x >= layout.width_px() || y >= layout.height_px() {
        return None;
    }
    Some((
        x,
        y.max(0.0),
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

pub(super) fn image_payload_svg_data_uri(span: &ObjectImagePayloadSpan) -> Option<String> {
    #[cfg(not(feature = "bitmap-images"))]
    {
        let _ = span;
        None
    }
    #[cfg(feature = "bitmap-images")]
    {
        if !span.complete()
            || span.dimensions().is_none()
            || !matches!(span.mime(), "image/jpeg" | "image/png")
        {
            return None;
        }

        let image = image::load_from_memory(span.payload()).ok()?;
        let mut cursor = std::io::Cursor::new(Vec::new());
        image.write_to(&mut cursor, image::ImageFormat::Png).ok()?;
        let encoded = BASE64_STANDARD.encode(cursor.into_inner());
        Some(format!("data:image/png;base64,{encoded}"))
    }
}

pub(super) fn visual_list_svg_data_uri(
    visual_list: &ObjectVisualListCandidate,
    suppress_dark_foreground: bool,
) -> Option<String> {
    #[cfg(not(feature = "bitmap-images"))]
    {
        let _ = (visual_list, suppress_dark_foreground);
        None
    }
    #[cfg(feature = "bitmap-images")]
    {
        let width = visual_list.width();
        let height = visual_list.height();
        if width == 0 || height == 0 {
            return None;
        }
        let expected_len = usize::try_from(width)
            .ok()?
            .checked_mul(usize::try_from(height).ok()?)?;
        if visual_list.pixels().len() != expected_len {
            return None;
        }

        let background = visual_list_background_pixel(visual_list.pixels());
        let dark_foreground = suppress_dark_foreground
            .then(|| visual_list_dark_foreground_pixel(visual_list.pixels(), background))
            .flatten();
        let mut rgba = Vec::with_capacity(expected_len.checked_mul(4)?);
        for pixel in visual_list.pixels() {
            if *pixel == background || dark_foreground.is_some_and(|dark| *pixel == dark) {
                rgba.extend_from_slice(&[0xff, 0xff, 0xff, 0x00]);
            } else {
                rgba.extend_from_slice(&[*pixel, *pixel, *pixel, 0xff]);
            }
        }
        let image = image::RgbaImage::from_vec(width, height, rgba)?;
        let mut cursor = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(image)
            .write_to(&mut cursor, image::ImageFormat::Png)
            .ok()?;
        let encoded = BASE64_STANDARD.encode(cursor.into_inner());
        Some(format!("data:image/png;base64,{encoded}"))
    }
}

pub(super) fn visual_list_horizontal_runs(
    visual_list: &ObjectVisualListCandidate,
) -> Vec<VisualListHorizontalRun> {
    let Ok(width) = usize::try_from(visual_list.width()) else {
        return Vec::new();
    };
    let Ok(height) = usize::try_from(visual_list.height()) else {
        return Vec::new();
    };
    if width == 0 || height == 0 {
        return Vec::new();
    }

    let background = visual_list_background_pixel(visual_list.pixels());
    let min_run = ((width * VISUAL_LIST_MIN_HORIZONTAL_RUN_PERCENT) / 100).max(8);
    let mut runs = Vec::new();
    for y in 0..height {
        let row_start = y * width;
        let Some(row) = visual_list.pixels().get(row_start..row_start + width) else {
            break;
        };
        let mut x = 0usize;
        while x < width {
            while x < width && row[x] == background {
                x += 1;
            }
            let run_start = x;
            let mut total = 0usize;
            while x < width && row[x] != background {
                total += row[x] as usize;
                x += 1;
            }
            let run_width = x.saturating_sub(run_start);
            if run_width >= min_run {
                runs.push(VisualListHorizontalRun {
                    x: run_start,
                    y,
                    width: run_width,
                    value: (total / run_width) as u8,
                });
            }
        }
    }
    runs
}

pub(super) fn visual_list_title_band(
    visual_list: &ObjectVisualListCandidate,
    runs: &[VisualListHorizontalRun],
) -> Option<VisualListTitleBand> {
    let width = usize::try_from(visual_list.width()).ok()?;
    let min_width = (width * 60) / 100;
    for (index, top) in runs.iter().enumerate() {
        if top.y > usize::try_from(visual_list.height()).ok()? / 4 || top.width < min_width {
            continue;
        }
        for bottom in runs.iter().skip(index + 1) {
            if bottom.y <= top.y || bottom.y - top.y > 12 {
                continue;
            }
            let left_delta = top.x.abs_diff(bottom.x);
            let width_delta = top.width.abs_diff(bottom.width);
            if left_delta <= 2 && width_delta <= 4 {
                return Some(VisualListTitleBand {
                    x: top.x.min(bottom.x) as f32,
                    y: top.y as f32,
                    width: top.width.max(bottom.width) as f32,
                    height: (bottom.y - top.y + 1) as f32,
                });
            }
        }
    }
    None
}

pub(super) const VISUAL_LIST_GOTHIC_FONT_FAMILY: &str =
    "'ＭＳ ゴシック', 'MS Gothic', 'Hiragino Kaku Gothic ProN', 'Yu Gothic', Meiryo, sans-serif";

#[derive(Debug, Clone, Copy)]
pub(super) struct SuccessDataTestJseqFormulaTopAnchor {
    pub(super) y: f32,
    pub(super) source_record_index: usize,
    pub(super) source_top_y: f32,
    pub(super) top_offset: f32,
}

pub(super) fn resolve_jseq_formula_text_slot(
    formula: &ObjectJseq3FormulaCandidate,
    slot: SuccessDataTestFormulaTextSlot,
) -> Option<ResolvedJseqFormulaTextSlot> {
    let text = if let Some(text) = formula
        .text_runs()
        .iter()
        .map(ObjectJseq3TextRunCandidate::text)
        .find_map(|text| {
            if slot.text == text {
                return Some(text.to_string());
            }
            let suffix = slot.text.strip_prefix(text)?;
            suffix
                .chars()
                .all(jseq_formula_compat_delimiter)
                .then(|| format!("{text}{}", normalize_jseq_formula_delimiters(suffix)))
        }) {
        text
    } else if slot.text.chars().all(jseq_formula_compat_delimiter) {
        normalize_jseq_formula_delimiters(slot.text)
    } else {
        return None;
    };
    Some(ResolvedJseqFormulaTextSlot {
        text,
        x: slot.x,
        baseline_y: slot.baseline_y,
        font_size: slot.font_size,
    })
}

pub(super) fn jseq_formula_compat_delimiter(character: char) -> bool {
    matches!(character, '（' | '）' | '(' | ')')
}

pub(super) fn normalize_jseq_formula_delimiters(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            '（' => '(',
            '）' => ')',
            other => other,
        })
        .collect()
}

pub(super) fn document_has_fax02_visual_list(document: &Document) -> bool {
    document.object_stream_candidates().iter().any(|candidate| {
        candidate
            .visual_list_candidate()
            .is_some_and(|visual_list| visual_list.width() == 120 && visual_list.height() == 169)
    })
}

pub(super) fn visual_list_background_pixel(pixels: &[u8]) -> u8 {
    let mut counts = [0usize; 256];
    for pixel in pixels {
        counts[*pixel as usize] += 1;
    }
    counts
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .map(|(pixel, _)| pixel as u8)
        .unwrap_or(0xff)
}

#[cfg(feature = "bitmap-images")]
pub(super) fn visual_list_dark_foreground_pixel(pixels: &[u8], background: u8) -> Option<u8> {
    pixels
        .iter()
        .copied()
        .filter(|pixel| *pixel != background)
        .min()
}

pub(super) fn visual_list_svg_gray(value: u8) -> String {
    format!("#{value:02x}{value:02x}{value:02x}")
}
