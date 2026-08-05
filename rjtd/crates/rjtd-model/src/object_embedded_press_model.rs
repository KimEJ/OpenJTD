use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectStreamCandidateReason {
    ObjectPath,
    ImagePath,
    ShapePath,
    TablePath,
    VisualListPath,
    FigureLink,
    EmbeddedPressSnapshot,
    FdmText,
    JsfartArt,
    Jseq3Formula,
    SoMarker,
    ImageSignature,
    SvgSignature,
}

impl ObjectStreamCandidateReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ObjectPath => "object-path",
            Self::ImagePath => "image-path",
            Self::ShapePath => "shape-path",
            Self::TablePath => "table-path",
            Self::VisualListPath => "visual-list-path",
            Self::FigureLink => "figure-link",
            Self::EmbeddedPressSnapshot => "embedded-press-snapshot",
            Self::FdmText => "fdm-text",
            Self::JsfartArt => "jsfart-art",
            Self::Jseq3Formula => "jseq3-formula",
            Self::SoMarker => "so-marker",
            Self::ImageSignature => "image-signature",
            Self::SvgSignature => "svg-signature",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddedPressVectorSegmentCandidate {
    pub(crate) x1: u32,
    pub(crate) y1: u32,
    pub(crate) x2: u32,
    pub(crate) y2: u32,
}

impl ObjectEmbeddedPressVectorSegmentCandidate {
    pub(crate) fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn x1(&self) -> u32 {
        self.x1
    }

    pub fn y1(&self) -> u32 {
        self.y1
    }

    pub fn x2(&self) -> u32 {
        self.x2
    }

    pub fn y2(&self) -> u32 {
        self.y2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectEmbeddedPressVectorPathKind {
    Outline,
    Texture,
}

impl ObjectEmbeddedPressVectorPathKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Outline => "outline",
            Self::Texture => "texture",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectEmbeddedPressTextureBezierHeaderCandidate {
    pub(crate) point_count: u32,
    pub(crate) byte_count: u32,
    pub(crate) flags: u32,
}

impl ObjectEmbeddedPressTextureBezierHeaderCandidate {
    pub(crate) fn new(point_count: u32, byte_count: u32, flags: u32) -> Self {
        Self {
            point_count,
            byte_count,
            flags,
        }
    }

    pub fn point_count(&self) -> u32 {
        self.point_count
    }

    pub fn byte_count(&self) -> u32 {
        self.byte_count
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectEmbeddedPressVectorPathCommandCandidate {
    MoveTo {
        x: u32,
        y: u32,
    },
    CubicTo {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        x3: u32,
        y3: u32,
    },
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddedPressStateRecordCandidate {
    pub(crate) record_type: u32,
    pub(crate) offset: usize,
    pub(crate) payload: Vec<u8>,
}

impl ObjectEmbeddedPressStateRecordCandidate {
    pub(crate) fn new(record_type: u32, offset: usize, payload: Vec<u8>) -> Self {
        Self {
            record_type,
            offset,
            payload,
        }
    }

    pub fn record_type(&self) -> u32 {
        self.record_type
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn payload_le32_words(&self) -> Vec<u32> {
        self.payload
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("chunk size is exact")))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEmbeddedPressVectorPathCandidate {
    pub(crate) kind: ObjectEmbeddedPressVectorPathKind,
    pub(crate) texture_bezier_header: Option<ObjectEmbeddedPressTextureBezierHeaderCandidate>,
    pub(crate) state_records: Vec<ObjectEmbeddedPressStateRecordCandidate>,
    pub(crate) commands: Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
}

impl ObjectEmbeddedPressVectorPathCandidate {
    pub(crate) fn new(
        kind: ObjectEmbeddedPressVectorPathKind,
        texture_bezier_header: Option<ObjectEmbeddedPressTextureBezierHeaderCandidate>,
        state_records: Vec<ObjectEmbeddedPressStateRecordCandidate>,
        commands: Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
    ) -> Self {
        Self {
            kind,
            texture_bezier_header,
            state_records,
            commands,
        }
    }

    pub fn kind(&self) -> ObjectEmbeddedPressVectorPathKind {
        self.kind
    }

    pub fn texture_bezier_header(&self) -> Option<ObjectEmbeddedPressTextureBezierHeaderCandidate> {
        self.texture_bezier_header
    }

    pub fn state_records(&self) -> &[ObjectEmbeddedPressStateRecordCandidate] {
        &self.state_records
    }

    pub fn commands(&self) -> &[ObjectEmbeddedPressVectorPathCommandCandidate] {
        &self.commands
    }
}

pub(crate) struct ObjectEmbeddedPressVectorPathBuilder {
    pub(crate) kind: ObjectEmbeddedPressVectorPathKind,
    pub(crate) texture_bezier_header: Option<ObjectEmbeddedPressTextureBezierHeaderCandidate>,
    pub(crate) state_records: Vec<ObjectEmbeddedPressStateRecordCandidate>,
    pub(crate) commands: Vec<ObjectEmbeddedPressVectorPathCommandCandidate>,
}

impl ObjectEmbeddedPressVectorPathBuilder {
    pub(crate) fn new(state_records: Vec<ObjectEmbeddedPressStateRecordCandidate>) -> Self {
        Self {
            kind: ObjectEmbeddedPressVectorPathKind::Outline,
            texture_bezier_header: None,
            state_records,
            commands: Vec::new(),
        }
    }

    pub(crate) fn mark_texture(&mut self, header: ObjectEmbeddedPressTextureBezierHeaderCandidate) {
        self.kind = ObjectEmbeddedPressVectorPathKind::Texture;
        if self.texture_bezier_header.is_none() {
            self.texture_bezier_header = Some(header);
        }
    }

    pub(crate) fn push(&mut self, command: ObjectEmbeddedPressVectorPathCommandCandidate) {
        self.commands.push(command);
    }

    pub(crate) fn finish(self) -> Option<ObjectEmbeddedPressVectorPathCandidate> {
        (!self.commands.is_empty()).then(|| {
            ObjectEmbeddedPressVectorPathCandidate::new(
                self.kind,
                self.texture_bezier_header,
                self.state_records,
                self.commands,
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectVisualListCandidate {
    pub(crate) declared_size: usize,
    pub(crate) magic_offset: usize,
    pub(crate) magic: String,
    pub(crate) version: u32,
    pub(crate) flags: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) row_stride: u32,
    pub(crate) bit_depth: u32,
    pub(crate) x_pixels_per_meter: u32,
    pub(crate) y_pixels_per_meter: u32,
    pub(crate) rle_data_offset: usize,
    pub(crate) rle_data_len: usize,
    pub(crate) pixels: Vec<u8>,
}

impl ObjectVisualListCandidate {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        declared_size: usize,
        version: u32,
        flags: u32,
        width: u32,
        height: u32,
        row_stride: u32,
        bit_depth: u32,
        x_pixels_per_meter: u32,
        y_pixels_per_meter: u32,
        rle_data_offset: usize,
        rle_data_len: usize,
        pixels: Vec<u8>,
    ) -> Self {
        Self {
            declared_size,
            magic_offset: VISUAL_LIST_MAGIC_OFFSET,
            magic: "BMDV".to_string(),
            version,
            flags,
            width,
            height,
            row_stride,
            bit_depth,
            x_pixels_per_meter,
            y_pixels_per_meter,
            rle_data_offset,
            rle_data_len,
            pixels,
        }
    }

    pub fn declared_size(&self) -> usize {
        self.declared_size
    }

    pub fn magic_offset(&self) -> usize {
        self.magic_offset
    }

    pub fn magic(&self) -> &str {
        &self.magic
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn row_stride(&self) -> u32 {
        self.row_stride
    }

    pub fn bit_depth(&self) -> u32 {
        self.bit_depth
    }

    pub fn x_pixels_per_meter(&self) -> u32 {
        self.x_pixels_per_meter
    }

    pub fn y_pixels_per_meter(&self) -> u32 {
        self.y_pixels_per_meter
    }

    pub fn rle_data_offset(&self) -> usize {
        self.rle_data_offset
    }

    pub fn rle_data_len(&self) -> usize {
        self.rle_data_len
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamCandidate {
    pub(crate) path: String,
    pub(crate) size: usize,
    pub(crate) reasons: Vec<ObjectStreamCandidateReason>,
    pub(crate) ownership_candidate: Option<ObjectStreamOwnershipCandidate>,
    pub(crate) ownership_reference_candidates: Vec<ObjectStreamOwnershipReferenceCandidate>,
    pub(crate) frame_reference_row_candidates: Vec<ObjectFrameReferenceRowCandidate>,
    pub(crate) fdm_index_entry_candidates: Vec<ObjectFdmIndexEntryCandidate>,
    pub(crate) fdm_text_index_entry_candidates: Vec<ObjectFdmTextIndexEntryCandidate>,
    pub(crate) fdm_raw_vector_segments: Vec<ObjectFdmVectorSegmentCandidate>,
    pub(crate) fdm_raw_vector_commands: Vec<ObjectFdmVectorCommandCandidate>,
    pub(crate) image_signature_hits: Vec<ObjectImageSignatureHit>,
    pub(crate) image_payload_spans: Vec<ObjectImagePayloadSpan>,
    pub(crate) visual_list_candidate: Option<ObjectVisualListCandidate>,
    pub(crate) figure_link_candidate: Option<ObjectFigureLinkCandidate>,
    pub(crate) embedded_press_snapshot_candidate: Option<ObjectEmbeddedPressSnapshotCandidate>,
    pub(crate) fdm_text_candidates: Vec<ObjectFdmTextCandidate>,
    pub(crate) jsfart_stream_profile_candidate: Option<ObjectJsfartStreamProfileCandidate>,
    pub(crate) jsfart_art_candidate: Option<ObjectJsfartArtCandidate>,
    pub(crate) jseq3_formula_candidate: Option<ObjectJseq3FormulaCandidate>,
    pub(crate) svg_offsets: Vec<usize>,
    pub(crate) so_offsets: Vec<usize>,
    pub(crate) payload_prefix: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ObjectFrameReferenceRowProjection {
    pub(crate) encoding: &'static str,
    pub(crate) stride: usize,
    pub(crate) field_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamOwnershipReferenceCandidate {
    pub(crate) target_path: String,
    pub(crate) encoding: String,
    pub(crate) total_matches: usize,
    pub(crate) offsets: Vec<usize>,
}

impl ObjectStreamOwnershipReferenceCandidate {
    pub fn new(
        target_path: impl Into<String>,
        encoding: impl Into<String>,
        total_matches: usize,
        offsets: Vec<usize>,
    ) -> Self {
        Self {
            target_path: target_path.into(),
            encoding: encoding.into(),
            total_matches,
            offsets,
        }
    }

    pub fn target_path(&self) -> &str {
        &self.target_path
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    pub fn total_matches(&self) -> usize {
        self.total_matches
    }

    pub fn offsets(&self) -> &[usize] {
        &self.offsets
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFrameReferenceRowCandidate {
    pub(crate) target_path: String,
    pub(crate) encoding: String,
    pub(crate) stride: usize,
    pub(crate) field_offset: usize,
    pub(crate) offset: usize,
    pub(crate) row_index: usize,
    pub(crate) row_start: usize,
    pub(crate) family: String,
    pub(crate) row: Vec<u8>,
    pub(crate) suffix_link: Option<ObjectFrameReferenceRowLink>,
}

impl ObjectFrameReferenceRowCandidate {
    pub(crate) fn new(
        target_path: impl Into<String>,
        encoding: impl Into<String>,
        stride: usize,
        field_offset: usize,
        location: ObjectFrameReferenceRowLocation,
        row: Vec<u8>,
    ) -> Self {
        let encoding = encoding.into();
        let family =
            classify_object_frame_reference_row(&row, encoding.as_str(), stride, field_offset);
        Self {
            target_path: target_path.into(),
            encoding,
            stride,
            field_offset,
            offset: location.offset,
            row_index: location.row_index,
            row_start: location.row_start,
            family: family.to_string(),
            row,
            suffix_link: None,
        }
    }

    pub fn target_path(&self) -> &str {
        &self.target_path
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn field_offset(&self) -> usize {
        self.field_offset
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn row_start(&self) -> usize {
        self.row_start
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn row(&self) -> &[u8] {
        &self.row
    }

    pub fn suffix_link(&self) -> Option<&ObjectFrameReferenceRowLink> {
        self.suffix_link.as_ref()
    }

    pub(crate) fn set_suffix_link(&mut self, suffix_link: ObjectFrameReferenceRowLink) {
        self.suffix_link = Some(suffix_link);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ObjectFrameReferenceRowLocation {
    pub(crate) offset: usize,
    pub(crate) row_index: usize,
    pub(crate) row_start: usize,
}

impl ObjectFrameReferenceRowLocation {
    pub(crate) fn new(offset: usize, row_index: usize, row_start: usize) -> Self {
        Self {
            offset,
            row_index,
            row_start,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFrameReferenceRowLink {
    pub(crate) relation: String,
    pub(crate) suffix_family: String,
    pub(crate) matched_row_start: usize,
    pub(crate) matched_row_index: usize,
}

impl ObjectFrameReferenceRowLink {
    pub(crate) fn new(
        relation: impl Into<String>,
        suffix_family: impl Into<String>,
        matched_row_start: usize,
        matched_row_index: usize,
    ) -> Self {
        Self {
            relation: relation.into(),
            suffix_family: suffix_family.into(),
            matched_row_start,
            matched_row_index,
        }
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn suffix_family(&self) -> &str {
        &self.suffix_family
    }

    pub fn matched_row_start(&self) -> usize {
        self.matched_row_start
    }

    pub fn matched_row_index(&self) -> usize {
        self.matched_row_index
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFrameRecordCandidate {
    pub(crate) source_path: String,
    pub(crate) row_index: usize,
    pub(crate) row_start: usize,
    pub(crate) record_len: usize,
    pub(crate) record_kind: u16,
    pub(crate) declared_record_bytes: u16,
    pub(crate) object_id: u16,
    pub(crate) object_type: u16,
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) corner_radius: u16,
    pub(crate) style_id: u16,
    pub(crate) row_prefix: Vec<u8>,
}

impl ObjectFrameRecordCandidate {
    pub(crate) fn new(
        source_path: impl Into<String>,
        row_index: usize,
        row_start: usize,
        row: &[u8],
    ) -> Self {
        Self {
            source_path: source_path.into(),
            row_index,
            row_start,
            record_len: row.len(),
            record_kind: read_be16_at(row, 0).unwrap_or_default(),
            declared_record_bytes: read_be16_at(row, 2).unwrap_or_default(),
            object_id: read_be16_at(row, FRAME_RECORD_ID_OFFSET).unwrap_or_default(),
            object_type: read_be16_at(row, FRAME_RECORD_TYPE_OFFSET).unwrap_or_default(),
            x: read_be16_at(row, FRAME_RECORD_X_OFFSET).unwrap_or_default(),
            y: read_be16_at(row, FRAME_RECORD_Y_OFFSET).unwrap_or_default(),
            width: read_be16_at(row, FRAME_RECORD_WIDTH_OFFSET).unwrap_or_default(),
            height: read_be16_at(row, FRAME_RECORD_HEIGHT_OFFSET).unwrap_or_default(),
            corner_radius: read_be16_at(row, FRAME_RECORD_CORNER_RADIUS_OFFSET).unwrap_or_default(),
            style_id: read_be16_at(row, FRAME_RECORD_STYLE_ID_OFFSET).unwrap_or_default(),
            row_prefix: row[..row.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)].to_vec(),
        }
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

    pub fn record_len(&self) -> usize {
        self.record_len
    }

    pub fn record_kind(&self) -> u16 {
        self.record_kind
    }

    pub fn declared_record_bytes(&self) -> u16 {
        self.declared_record_bytes
    }

    pub fn object_id(&self) -> u16 {
        self.object_id
    }

    pub fn object_type(&self) -> u16 {
        self.object_type
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn corner_radius(&self) -> u16 {
        self.corner_radius
    }

    pub fn style_id(&self) -> u16 {
        self.style_id
    }

    pub fn row_prefix(&self) -> &[u8] {
        &self.row_prefix
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFigureLinkCandidate {
    pub(crate) header_words_be: Vec<u16>,
    pub(crate) declared_row_count_candidate: Option<u16>,
    pub(crate) row_stride: usize,
    pub(crate) rows: Vec<ObjectFigureLinkRowCandidate>,
}

impl ObjectFigureLinkCandidate {
    pub(crate) fn new(
        header_words_be: Vec<u16>,
        declared_row_count_candidate: Option<u16>,
        row_stride: usize,
        rows: Vec<ObjectFigureLinkRowCandidate>,
    ) -> Self {
        Self {
            header_words_be,
            declared_row_count_candidate,
            row_stride,
            rows,
        }
    }

    pub fn header_words_be(&self) -> &[u16] {
        &self.header_words_be
    }

    pub fn declared_row_count_candidate(&self) -> Option<u16> {
        self.declared_row_count_candidate
    }

    pub fn row_stride(&self) -> usize {
        self.row_stride
    }

    pub fn rows(&self) -> &[ObjectFigureLinkRowCandidate] {
        &self.rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFigureLinkRowCandidate {
    pub(crate) row_index: usize,
    pub(crate) row_start: usize,
    pub(crate) words_be: Vec<u16>,
    pub(crate) row: Vec<u8>,
}

impl ObjectFigureLinkRowCandidate {
    pub(crate) fn new(row_index: usize, row_start: usize, row: &[u8]) -> Self {
        Self {
            row_index,
            row_start,
            words_be: read_be16_fields(row),
            row: row.to_vec(),
        }
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn row_start(&self) -> usize {
        self.row_start
    }

    pub fn words_be(&self) -> &[u16] {
        &self.words_be
    }

    pub fn group_index_candidate(&self) -> Option<u16> {
        self.words_be.get(1).copied()
    }

    pub fn source_id_candidate(&self) -> Option<u16> {
        self.words_be.get(3).copied()
    }

    pub fn relation_kind_candidate(&self) -> Option<u16> {
        self.words_be.get(4).copied()
    }

    pub fn target_row_index_candidate(&self) -> Option<u16> {
        self.words_be.get(6).copied()
    }

    pub fn row(&self) -> &[u8] {
        &self.row
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamOwnershipCandidate {
    pub(crate) basis: String,
    pub(crate) family: String,
    pub(crate) storage_path: Option<String>,
    pub(crate) embedding_index: Option<usize>,
    pub(crate) stream_role: String,
}

impl ObjectStreamOwnershipCandidate {
    pub fn new(
        basis: impl Into<String>,
        family: impl Into<String>,
        storage_path: Option<String>,
        embedding_index: Option<usize>,
        stream_role: impl Into<String>,
    ) -> Self {
        Self {
            basis: basis.into(),
            family: family.into(),
            storage_path,
            embedding_index,
            stream_role: stream_role.into(),
        }
    }

    pub fn basis(&self) -> &str {
        &self.basis
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn storage_path(&self) -> Option<&str> {
        self.storage_path.as_deref()
    }

    pub fn embedding_index(&self) -> Option<usize> {
        self.embedding_index
    }

    pub fn stream_role(&self) -> &str {
        &self.stream_role
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamCandidateEvidence {
    pub(crate) reasons: Vec<ObjectStreamCandidateReason>,
    pub(crate) image_signature_hits: Vec<ObjectImageSignatureHit>,
    pub(crate) image_payload_spans: Vec<ObjectImagePayloadSpan>,
    pub(crate) visual_list_candidate: Option<ObjectVisualListCandidate>,
    pub(crate) figure_link_candidate: Option<ObjectFigureLinkCandidate>,
    pub(crate) embedded_press_snapshot_candidate: Option<ObjectEmbeddedPressSnapshotCandidate>,
    pub(crate) fdm_text_candidates: Vec<ObjectFdmTextCandidate>,
    pub(crate) jsfart_stream_profile_candidate: Option<ObjectJsfartStreamProfileCandidate>,
    pub(crate) jsfart_art_candidate: Option<ObjectJsfartArtCandidate>,
    pub(crate) jseq3_formula_candidate: Option<ObjectJseq3FormulaCandidate>,
    pub(crate) svg_offsets: Vec<usize>,
    pub(crate) so_offsets: Vec<usize>,
}

impl ObjectStreamCandidateEvidence {
    pub fn new(
        reasons: Vec<ObjectStreamCandidateReason>,
        image_signature_hits: Vec<ObjectImageSignatureHit>,
        image_payload_spans: Vec<ObjectImagePayloadSpan>,
        visual_list_candidate: Option<ObjectVisualListCandidate>,
        svg_offsets: Vec<usize>,
        so_offsets: Vec<usize>,
    ) -> Self {
        Self {
            reasons,
            image_signature_hits,
            image_payload_spans,
            visual_list_candidate,
            figure_link_candidate: None,
            embedded_press_snapshot_candidate: None,
            fdm_text_candidates: Vec::new(),
            jsfart_stream_profile_candidate: None,
            jsfart_art_candidate: None,
            jseq3_formula_candidate: None,
            svg_offsets,
            so_offsets,
        }
    }

    pub fn reasons(&self) -> &[ObjectStreamCandidateReason] {
        &self.reasons
    }

    pub fn image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.image_signature_hits
    }

    pub fn image_payload_spans(&self) -> &[ObjectImagePayloadSpan] {
        &self.image_payload_spans
    }

    pub fn visual_list_candidate(&self) -> Option<&ObjectVisualListCandidate> {
        self.visual_list_candidate.as_ref()
    }

    pub(crate) fn with_figure_link_candidate(
        mut self,
        link: Option<ObjectFigureLinkCandidate>,
    ) -> Self {
        self.figure_link_candidate = link;
        self
    }

    pub(crate) fn with_embedded_press_snapshot_candidate(
        mut self,
        snapshot: Option<ObjectEmbeddedPressSnapshotCandidate>,
    ) -> Self {
        self.embedded_press_snapshot_candidate = snapshot;
        self
    }

    pub(crate) fn with_fdm_text_candidates(
        mut self,
        candidates: Vec<ObjectFdmTextCandidate>,
    ) -> Self {
        self.fdm_text_candidates = candidates;
        self
    }

    pub(crate) fn with_jseq3_formula_candidate(
        mut self,
        formula: Option<ObjectJseq3FormulaCandidate>,
    ) -> Self {
        self.jseq3_formula_candidate = formula;
        self
    }

    pub(crate) fn with_jsfart_stream_profile_candidate(
        mut self,
        profile: Option<ObjectJsfartStreamProfileCandidate>,
    ) -> Self {
        self.jsfart_stream_profile_candidate = profile;
        self
    }

    pub(crate) fn with_jsfart_art_candidate(
        mut self,
        art: Option<ObjectJsfartArtCandidate>,
    ) -> Self {
        self.jsfart_art_candidate = art;
        self
    }

    pub fn svg_offsets(&self) -> &[usize] {
        &self.svg_offsets
    }

    pub fn so_offsets(&self) -> &[usize] {
        &self.so_offsets
    }
}

impl ObjectStreamCandidate {
    pub fn new(
        path: impl Into<String>,
        size: usize,
        evidence: ObjectStreamCandidateEvidence,
        payload_prefix: Vec<u8>,
    ) -> Self {
        let path = path.into();
        let ownership_candidate = object_stream_ownership_candidate(&path);
        Self {
            path,
            size,
            reasons: evidence.reasons,
            ownership_candidate,
            ownership_reference_candidates: Vec::new(),
            frame_reference_row_candidates: Vec::new(),
            fdm_index_entry_candidates: Vec::new(),
            fdm_text_index_entry_candidates: Vec::new(),
            fdm_raw_vector_segments: Vec::new(),
            fdm_raw_vector_commands: Vec::new(),
            image_signature_hits: evidence.image_signature_hits,
            image_payload_spans: evidence.image_payload_spans,
            visual_list_candidate: evidence.visual_list_candidate,
            figure_link_candidate: evidence.figure_link_candidate,
            embedded_press_snapshot_candidate: evidence.embedded_press_snapshot_candidate,
            fdm_text_candidates: evidence.fdm_text_candidates,
            jsfart_stream_profile_candidate: evidence.jsfart_stream_profile_candidate,
            jsfart_art_candidate: evidence.jsfart_art_candidate,
            jseq3_formula_candidate: evidence.jseq3_formula_candidate,
            svg_offsets: evidence.svg_offsets,
            so_offsets: evidence.so_offsets,
            payload_prefix,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn reasons(&self) -> &[ObjectStreamCandidateReason] {
        &self.reasons
    }

    pub fn ownership_candidate(&self) -> Option<&ObjectStreamOwnershipCandidate> {
        self.ownership_candidate.as_ref()
    }

    pub fn ownership_reference_candidates(&self) -> &[ObjectStreamOwnershipReferenceCandidate] {
        &self.ownership_reference_candidates
    }

    pub fn frame_reference_row_candidates(&self) -> &[ObjectFrameReferenceRowCandidate] {
        &self.frame_reference_row_candidates
    }

    pub fn fdm_index_entry_candidates(&self) -> &[ObjectFdmIndexEntryCandidate] {
        &self.fdm_index_entry_candidates
    }

    pub fn fdm_text_index_entry_candidates(&self) -> &[ObjectFdmTextIndexEntryCandidate] {
        &self.fdm_text_index_entry_candidates
    }

    pub fn fdm_raw_vector_segments(&self) -> &[ObjectFdmVectorSegmentCandidate] {
        &self.fdm_raw_vector_segments
    }

    pub fn fdm_raw_vector_commands(&self) -> &[ObjectFdmVectorCommandCandidate] {
        &self.fdm_raw_vector_commands
    }

    pub(crate) fn set_ownership_reference_candidates(
        &mut self,
        ownership_reference_candidates: Vec<ObjectStreamOwnershipReferenceCandidate>,
    ) {
        self.ownership_reference_candidates = ownership_reference_candidates;
    }

    pub(crate) fn set_frame_reference_row_candidates(
        &mut self,
        frame_reference_row_candidates: Vec<ObjectFrameReferenceRowCandidate>,
    ) {
        self.frame_reference_row_candidates = frame_reference_row_candidates;
    }

    pub(crate) fn set_fdm_index_entry_candidates(
        &mut self,
        fdm_index_entry_candidates: Vec<ObjectFdmIndexEntryCandidate>,
    ) {
        self.fdm_index_entry_candidates = fdm_index_entry_candidates;
    }

    pub(crate) fn set_fdm_text_index_entry_candidates(
        &mut self,
        fdm_text_index_entry_candidates: Vec<ObjectFdmTextIndexEntryCandidate>,
    ) {
        self.fdm_text_index_entry_candidates = fdm_text_index_entry_candidates;
    }

    pub(crate) fn set_fdm_raw_vector_segments(
        &mut self,
        fdm_raw_vector_segments: Vec<ObjectFdmVectorSegmentCandidate>,
    ) {
        self.fdm_raw_vector_segments = fdm_raw_vector_segments;
    }

    pub(crate) fn set_fdm_raw_vector_commands(
        &mut self,
        fdm_raw_vector_commands: Vec<ObjectFdmVectorCommandCandidate>,
    ) {
        self.fdm_raw_vector_commands = fdm_raw_vector_commands;
    }

    pub fn image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.image_signature_hits
    }

    pub fn image_payload_spans(&self) -> &[ObjectImagePayloadSpan] {
        &self.image_payload_spans
    }

    pub fn visual_list_candidate(&self) -> Option<&ObjectVisualListCandidate> {
        self.visual_list_candidate.as_ref()
    }

    pub fn figure_link_candidate(&self) -> Option<&ObjectFigureLinkCandidate> {
        self.figure_link_candidate.as_ref()
    }

    pub fn embedded_press_snapshot_candidate(
        &self,
    ) -> Option<&ObjectEmbeddedPressSnapshotCandidate> {
        self.embedded_press_snapshot_candidate.as_ref()
    }

    pub fn fdm_text_candidates(&self) -> &[ObjectFdmTextCandidate] {
        &self.fdm_text_candidates
    }

    pub fn jsfart_stream_profile_candidate(&self) -> Option<&ObjectJsfartStreamProfileCandidate> {
        self.jsfart_stream_profile_candidate.as_ref()
    }

    pub fn jsfart_art_candidate(&self) -> Option<&ObjectJsfartArtCandidate> {
        self.jsfart_art_candidate.as_ref()
    }

    pub fn jseq3_formula_candidate(&self) -> Option<&ObjectJseq3FormulaCandidate> {
        self.jseq3_formula_candidate.as_ref()
    }

    pub fn svg_offsets(&self) -> &[usize] {
        &self.svg_offsets
    }

    pub fn so_offsets(&self) -> &[usize] {
        &self.so_offsets
    }

    pub fn payload_prefix(&self) -> &[u8] {
        &self.payload_prefix
    }
}
