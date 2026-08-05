use super::*;

pub(super) const FDM_INDEX_HEADER_BYTES: usize = 20;

pub(super) const FDM_INDEX_ENTRY_BYTES: usize = 22;

pub(super) const FDM_INDEX_DECLARED_COUNT_OFFSET: usize = 18;

pub(super) const FDM_VECTOR_SEGMENT_MAGIC: &[u8; 4] = b"\x01\x00\x0b\x60";

pub(super) const FDM_VECTOR_SEGMENT_HEADER_BYTES: usize = 52;

pub(super) const FDM_VECTOR_COMMAND_OFFSET_BYTES: usize = 2;

pub(super) const FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET: usize = 4;

pub(super) const FDM_VECTOR_COMMAND_BBOX_OFFSET: usize = 20;

pub(super) const FDM_VECTOR_COMMAND_BBOX_MARKER: &[u8; 4] = b"\xff\x00\x0a\x60";

pub(super) const FDM_VECTOR_COMMAND_LINE_MARKER: &[u8; 4] = b"\xff\x00\x01\x60";

pub(super) const FDM_VECTOR_COMMAND_NESTED_LINE_MARKER: &[u8; 4] = b"\x00\x00\x01\x60";

pub(super) const FDM_VECTOR_COMMAND_INDEXED_LINE_MARKER: &[u8; 4] = b"\x01\x00\x01\x60";

pub(super) const FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET: usize = 16;

pub(super) const FDM_VECTOR_COMMAND_ELLIPSE_COLOR_OFFSET: usize = 12;

pub(super) const FDM_VECTOR_COMMAND_ELLIPSE_CENTER_OFFSET: usize = 16;

pub(super) const FDM_VECTOR_COMMAND_ELLIPSE_RADIUS_OFFSET: usize = 24;

pub(super) const FDM_VECTOR_COMMAND_PATH_POINT_COUNT_OFFSET: usize = 16;

pub(super) const FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET: usize = 18;

pub(super) const FDM_TEXT_RECORD_MARKER: &[u8; 4] = b"\x01\x00\x14\x60";

pub(super) const FDM_TEXT_EXPANDED_RECORD_MARKER: &[u8; 4] = b"\x01\x00\x16\x60";

pub(super) const FDM_TEXT_RECORD_DECLARED_LENGTH_OFFSET: usize = 4;

pub(super) const FDM_TEXT_RECORD_TRAILER: &[u8; 4] = b"\x00\x0d\x00\x0d";

pub(super) const FDM_TEXT_RECORD_TEXT_DELIMITER: &[u8; 2] = b"\x00\x0d";

pub(super) const FDM_TEXT_RECORD_BBOX_OFFSET_FROM_MARKER: usize = 8;

pub(super) const FDM_TEXT_RECORD_BACKSCAN_BYTES: usize = 96;

pub(super) const FDM_TEXT_EXPANDED_COUNT_OFFSET_FROM_MARKER: usize = 0x22;

pub(super) const FDM_TEXT_EXPANDED_INDEX_KIND: u16 = 0x1600;

pub(super) const FDM_VECTOR_COMMAND_ELLIPSE_MARKERS: [[u8; 4]; 3] = [
    *b"\xff\x00\x04\x60",
    *b"\x00\x00\x04\x60",
    *b"\x01\x00\x04\x60",
];

pub(super) const FDM_VECTOR_COMMAND_PATH_MARKERS: [[u8; 4]; 6] = [
    *b"\xff\x00\x06\x60",
    *b"\xff\x00\x09\x60",
    *b"\x00\x00\x06\x60",
    *b"\x00\x00\x09\x60",
    *b"\x01\x00\x06\x60",
    *b"\x01\x00\x09\x60",
];

pub(super) const FDM_VECTOR_NESTED_PRIMITIVE_MARKERS: [[u8; 4]; 12] = [
    *b"\x00\x00\x01\x60",
    *b"\x00\x00\x04\x60",
    *b"\x00\x00\x06\x60",
    *b"\x00\x00\x09\x60",
    *b"\x01\x00\x01\x60",
    *b"\x01\x00\x04\x60",
    *b"\x01\x00\x06\x60",
    *b"\x01\x00\x09\x60",
    *b"\xff\x00\x01\x60",
    *b"\xff\x00\x04\x60",
    *b"\xff\x00\x06\x60",
    *b"\xff\x00\x09\x60",
];

pub(super) const FDM_VECTOR_TEXT_MASK_MIN_INNER_AREA_RATIO: f64 = 0.30;

pub(super) const FDM_VECTOR_TEXT_MASK_MAX_INNER_AREA_RATIO: f64 = 0.85;

pub(super) const FDM_VECTOR_RENDERED_PRIMITIVE_MARKERS: [[u8; 4]; 12] = [
    *b"\xff\x00\x01\x60",
    *b"\xff\x00\x04\x60",
    *b"\xff\x00\x06\x60",
    *b"\xff\x00\x09\x60",
    *b"\x00\x00\x01\x60",
    *b"\x00\x00\x04\x60",
    *b"\x00\x00\x06\x60",
    *b"\x00\x00\x09\x60",
    *b"\x01\x00\x01\x60",
    *b"\x01\x00\x04\x60",
    *b"\x01\x00\x06\x60",
    *b"\x01\x00\x09\x60",
];

pub(super) const FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO: f32 = 0.28;

pub(super) const FDM_TEXT_MASK_COHORT_MIN_PRIMITIVES: usize = 3;

pub(super) const FDM_TEXT_MASK_COHORT_LIMIT: usize = 24;

pub(super) const FDM_TEXT_MASK_RIGHT_NEIGHBOR_MAX_GAP_FACTOR: f32 = 3.0;

pub(super) const FDM_TEXT_MASK_COMPONENT_MIN_PRIMITIVES: usize = 3;

pub(super) const FDM_TEXT_MASK_COMPONENT_MAX_HEIGHT_LINE_FACTOR: f32 = 0.85;

pub(super) const FDM_CONNECTOR_CANDIDATE_MIN_SOURCE_SPAN_UNITS: i32 = 500;

pub(super) const FDM_CONNECTOR_CANDIDATE_MIN_PROJECTED_SPAN_PX: f32 = 48.0;

pub(super) const FDM_OPEN_STROKE_AXIS_RULE_MIN_PROJECTED_SPAN_PX: f32 = 5.0;

pub(super) const FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT: usize = 16;

pub(super) const FDM_OPEN_STROKE_ROW_COHORT_LIMIT: usize = 16;

pub(super) const FIGURE_LINK_HEADER_BYTES: usize = 8;

pub(super) const FIGURE_LINK_ROW_BYTES: usize = 14;

pub(super) const FIGURE_LINK_RELATION_KIND_CANDIDATE_OFFSET: usize = 8;

pub(super) const FIGURE_LINK_RELATION_KIND_CANDIDATE: u16 = 0x0016;

pub(super) const SHANAI_LAN_FDM_FRAME_X_DIVISOR: f32 = 24.0;

pub(super) const SHANAI_LAN_FDM_FRAME_Y_DIVISOR: f32 = 1.0;

pub(super) const SHANAI_LAN_FDM_FRAME_SIZE_DIVISOR: f32 = 24.0;

pub(super) const FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS: f32 = 2.0;

pub(super) const FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS: f32 = 1.0;

pub(super) const FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS: f32 = 2.0;

pub(super) const FDM_CONNECTOR_ENDPOINT_OWNER_PROBE_RADIUS_PX: f32 = 18.0;

pub(super) const FDM_CONNECTOR_ENDPOINT_OWNER_CANDIDATE_LIMIT: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFdmTextCandidate {
    pub(super) text: String,
    pub(super) text_offset: usize,
    pub(super) marker_offset: usize,
    pub(super) raw_text: Vec<u8>,
    pub(super) bbox: Option<ObjectFdmIndexBbox>,
}

impl ObjectFdmTextCandidate {
    pub(super) fn new(
        text: impl Into<String>,
        text_offset: usize,
        marker_offset: usize,
        raw_text: Vec<u8>,
        bbox: Option<ObjectFdmIndexBbox>,
    ) -> Self {
        Self {
            text: text.into(),
            text_offset,
            marker_offset,
            raw_text,
            bbox,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_offset(&self) -> usize {
        self.text_offset
    }

    pub fn marker_offset(&self) -> usize {
        self.marker_offset
    }

    pub fn raw_text(&self) -> &[u8] {
        &self.raw_text
    }

    pub fn bbox(&self) -> Option<ObjectFdmIndexBbox> {
        self.bbox
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmIndexBbox {
    pub(super) left: i32,
    pub(super) top: i32,
    pub(super) right: i32,
    pub(super) bottom: i32,
}

impl ObjectFdmIndexBbox {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn left(self) -> i32 {
        self.left
    }

    pub fn top(self) -> i32 {
        self.top
    }

    pub fn right(self) -> i32 {
        self.right
    }

    pub fn bottom(self) -> i32 {
        self.bottom
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFdmIndexEntryCandidate {
    pub(super) index_path: String,
    pub(super) vector_path: String,
    pub(super) row_index: usize,
    pub(super) index_offset: usize,
    pub(super) vector_offset: usize,
    pub(super) next_vector_offset: usize,
    pub(super) vector_len: usize,
    pub(super) kind: u16,
    pub(super) bbox: ObjectFdmIndexBbox,
    pub(super) valid_vector_offset: bool,
    pub(super) vector_prefix: Vec<u8>,
    pub(super) image_signature_hits: Vec<ObjectImageSignatureHit>,
    pub(super) segment_image_signature_hits: Vec<ObjectImageSignatureHit>,
    pub(super) vector_commands: Vec<ObjectFdmVectorCommandCandidate>,
    pub(super) connector_candidates: Vec<ObjectFdmConnectorCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFdmTextIndexEntryCandidate {
    pub(super) index_path: String,
    pub(super) text_path: String,
    pub(super) row_index: usize,
    pub(super) index_offset: usize,
    pub(super) text_record_offset: usize,
    pub(super) kind: u16,
    pub(super) bbox: ObjectFdmIndexBbox,
    pub(super) text_record_bbox: Option<ObjectFdmIndexBbox>,
    pub(super) valid_text_record_offset: bool,
    pub(super) text_record_prefix: Vec<u8>,
}

impl ObjectFdmTextIndexEntryCandidate {
    pub fn index_path(&self) -> &str {
        &self.index_path
    }

    pub fn text_path(&self) -> &str {
        &self.text_path
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn index_offset(&self) -> usize {
        self.index_offset
    }

    pub fn text_record_offset(&self) -> usize {
        self.text_record_offset
    }

    pub fn kind(&self) -> u16 {
        self.kind
    }

    pub fn bbox(&self) -> ObjectFdmIndexBbox {
        self.bbox
    }

    pub fn text_record_bbox(&self) -> Option<ObjectFdmIndexBbox> {
        self.text_record_bbox
    }

    pub fn valid_text_record_offset(&self) -> bool {
        self.valid_text_record_offset
    }

    pub fn text_record_prefix(&self) -> &[u8] {
        &self.text_record_prefix
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmIndexSegmentBboxAxisPairGate {
    pub(super) valid_index_row_count: usize,
    pub(super) linked_row_count: usize,
    pub(super) axis_pair_order_agreement_row_count: usize,
}

impl FdmIndexSegmentBboxAxisPairGate {
    pub(super) fn new(
        valid_index_row_count: usize,
        linked_row_count: usize,
        axis_pair_order_agreement_row_count: usize,
    ) -> Self {
        Self {
            valid_index_row_count,
            linked_row_count,
            axis_pair_order_agreement_row_count,
        }
    }

    pub(super) fn valid_index_row_count(self) -> usize {
        self.valid_index_row_count
    }

    pub(super) fn linked_row_count(self) -> usize {
        self.linked_row_count
    }

    pub(super) fn axis_pair_order_agreement_row_count(self) -> usize {
        self.axis_pair_order_agreement_row_count
    }

    pub(super) fn axis_pair_order_agreement_complete(self) -> bool {
        self.valid_index_row_count > 0
            && self.valid_index_row_count == self.linked_row_count
            && self.linked_row_count == self.axis_pair_order_agreement_row_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FdmTextMirrorAnchorAgreement {
    pub(super) indexed_text_path: String,
    pub(super) mirrored_text_path: String,
    pub(super) text_record_count: usize,
    pub(super) ordered_text_agreement: bool,
    pub(super) ordered_record_bbox_agreement: bool,
    pub(super) indexed_record_offset_agreement: bool,
    pub(super) indexed_record_bbox_agreement: bool,
}

impl FdmTextMirrorAnchorAgreement {
    pub(super) fn new(
        indexed_text_path: impl Into<String>,
        mirrored_text_path: impl Into<String>,
        text_record_count: usize,
        ordered_text_agreement: bool,
        ordered_record_bbox_agreement: bool,
        indexed_record_offset_agreement: bool,
        indexed_record_bbox_agreement: bool,
    ) -> Self {
        Self {
            indexed_text_path: indexed_text_path.into(),
            mirrored_text_path: mirrored_text_path.into(),
            text_record_count,
            ordered_text_agreement,
            ordered_record_bbox_agreement,
            indexed_record_offset_agreement,
            indexed_record_bbox_agreement,
        }
    }

    pub(super) fn indexed_text_path(&self) -> &str {
        &self.indexed_text_path
    }

    pub(super) fn mirrored_text_path(&self) -> &str {
        &self.mirrored_text_path
    }

    pub(super) fn text_record_count(&self) -> usize {
        self.text_record_count
    }

    pub(super) fn ordered_text_agreement(&self) -> bool {
        self.ordered_text_agreement
    }

    pub(super) fn ordered_record_bbox_agreement(&self) -> bool {
        self.ordered_record_bbox_agreement
    }

    pub(super) fn indexed_record_offset_agreement(&self) -> bool {
        self.indexed_record_offset_agreement
    }

    pub(super) fn indexed_record_bbox_agreement(&self) -> bool {
        self.indexed_record_bbox_agreement
    }

    pub(super) fn source_anchor_trace_ready(&self) -> bool {
        self.ordered_text_agreement
            && self.ordered_record_bbox_agreement
            && self.indexed_record_offset_agreement
            && self.indexed_record_bbox_agreement
    }
}

impl ObjectFdmIndexEntryCandidate {
    pub fn index_path(&self) -> &str {
        &self.index_path
    }

    pub fn vector_path(&self) -> &str {
        &self.vector_path
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn index_offset(&self) -> usize {
        self.index_offset
    }

    pub fn vector_offset(&self) -> usize {
        self.vector_offset
    }

    pub fn next_vector_offset(&self) -> usize {
        self.next_vector_offset
    }

    pub fn vector_len(&self) -> usize {
        self.vector_len
    }

    pub fn kind(&self) -> u16 {
        self.kind
    }

    pub fn bbox(&self) -> ObjectFdmIndexBbox {
        self.bbox
    }

    pub fn valid_vector_offset(&self) -> bool {
        self.valid_vector_offset
    }

    pub fn vector_prefix(&self) -> &[u8] {
        &self.vector_prefix
    }

    pub fn image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.image_signature_hits
    }

    pub fn segment_image_signature_hits(&self) -> &[ObjectImageSignatureHit] {
        &self.segment_image_signature_hits
    }

    pub fn vector_commands(&self) -> &[ObjectFdmVectorCommandCandidate] {
        &self.vector_commands
    }

    pub fn connector_candidates(&self) -> &[ObjectFdmConnectorCandidate] {
        &self.connector_candidates
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmConnectorCandidate {
    pub(super) command_index: usize,
    pub(super) relative_offset: usize,
    pub(super) marker: [u8; 4],
    pub(super) style_word: u16,
    pub(super) primitive_kind: &'static str,
    pub(super) fill_color: Option<u32>,
    pub(super) stroke_color: Option<u32>,
    pub(super) source_start: ObjectFdmVectorPoint,
    pub(super) source_end: ObjectFdmVectorPoint,
    pub(super) source_bbox: ObjectFdmIndexBbox,
    pub(super) source_span: i32,
    pub(super) endpoint_dx: i32,
    pub(super) endpoint_dy: i32,
    pub(super) endpoint_distance_squared: u64,
    pub(super) path_point_count: usize,
    pub(super) path_segment_count: usize,
    pub(super) orthogonal_segment_count: usize,
    pub(super) diagonal_segment_count: usize,
    pub(super) curve_segment_count: usize,
    pub(super) compound_child_offset_count: usize,
    pub(super) axis_aligned: bool,
    pub(super) orientation: &'static str,
    pub(super) basis: &'static str,
}

impl ObjectFdmConnectorCandidate {
    pub fn command_index(self) -> usize {
        self.command_index
    }

    pub fn relative_offset(self) -> usize {
        self.relative_offset
    }

    pub fn marker(self) -> [u8; 4] {
        self.marker
    }

    pub fn style_word(self) -> u16 {
        self.style_word
    }

    pub fn primitive_kind(self) -> &'static str {
        self.primitive_kind
    }

    pub fn fill_color(self) -> Option<u32> {
        self.fill_color
    }

    pub fn stroke_color(self) -> Option<u32> {
        self.stroke_color
    }

    pub fn source_start(self) -> ObjectFdmVectorPoint {
        self.source_start
    }

    pub fn source_end(self) -> ObjectFdmVectorPoint {
        self.source_end
    }

    pub fn source_bbox(self) -> ObjectFdmIndexBbox {
        self.source_bbox
    }

    pub fn source_span(self) -> i32 {
        self.source_span
    }

    pub fn endpoint_dx(self) -> i32 {
        self.endpoint_dx
    }

    pub fn endpoint_dy(self) -> i32 {
        self.endpoint_dy
    }

    pub fn endpoint_distance_squared(self) -> u64 {
        self.endpoint_distance_squared
    }

    pub fn path_point_count(self) -> usize {
        self.path_point_count
    }

    pub fn path_segment_count(self) -> usize {
        self.path_segment_count
    }

    pub fn orthogonal_segment_count(self) -> usize {
        self.orthogonal_segment_count
    }

    pub fn diagonal_segment_count(self) -> usize {
        self.diagonal_segment_count
    }

    pub fn curve_segment_count(self) -> usize {
        self.curve_segment_count
    }

    pub fn compound_child_offset_count(self) -> usize {
        self.compound_child_offset_count
    }

    pub fn axis_aligned(self) -> bool {
        self.axis_aligned
    }

    pub fn orientation(self) -> &'static str {
        self.orientation
    }

    pub fn basis(self) -> &'static str {
        self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFdmVectorSegmentCandidate {
    pub(super) relative_offset: usize,
    pub(super) declared_len: u16,
    pub(super) command_count: u16,
    pub(super) command_offsets: Vec<u16>,
    pub(super) bbox: Option<ObjectFdmIndexBbox>,
    pub(super) source_width: i32,
    pub(super) source_height: i32,
}

impl ObjectFdmVectorSegmentCandidate {
    pub(super) fn new(relative_offset: usize, header: FdmVectorSegmentHeader) -> Self {
        Self {
            relative_offset,
            declared_len: header.declared_len,
            command_count: header.command_count,
            command_offsets: header.command_offsets,
            bbox: header.bbox,
            source_width: header.source_width,
            source_height: header.source_height,
        }
    }

    pub fn relative_offset(&self) -> usize {
        self.relative_offset
    }

    pub fn declared_len(&self) -> u16 {
        self.declared_len
    }

    pub fn command_count(&self) -> u16 {
        self.command_count
    }

    pub fn command_offsets(&self) -> &[u16] {
        &self.command_offsets
    }

    pub fn bbox(&self) -> Option<ObjectFdmIndexBbox> {
        self.bbox
    }

    pub fn source_width(&self) -> i32 {
        self.source_width
    }

    pub fn source_height(&self) -> i32 {
        self.source_height
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFdmVectorCommandCandidate {
    pub(super) command_index: usize,
    pub(super) relative_offset: usize,
    pub(super) source_vector_relative_offset: Option<usize>,
    pub(super) source_segment: Option<ObjectFdmVectorCommandSourceSegment>,
    pub(super) record_len: usize,
    pub(super) declared_record_len: u16,
    pub(super) style_word: u16,
    pub(super) marker: [u8; 4],
    pub(super) bbox: Option<ObjectFdmIndexBbox>,
    pub(super) path_points: Vec<ObjectFdmVectorPoint>,
    pub(super) curve_segments: Vec<ObjectFdmVectorCurveSegment>,
    pub(super) ellipse: Option<ObjectFdmVectorEllipse>,
    pub(super) compound_child_offsets: Vec<u16>,
    pub(super) compound_child_layout: Option<FdmCompoundChildLayout>,
    pub(super) gradient_colors: Option<FdmVectorGradientContext>,
    pub(super) fill_color: Option<u32>,
    pub(super) stroke_color: Option<u32>,
}

impl ObjectFdmVectorCommandCandidate {
    pub(super) fn new(
        command_index: usize,
        relative_offset: usize,
        record: &[u8],
        next_offset: usize,
        style_context: Option<FdmVectorStyleContext>,
    ) -> Option<Self> {
        if record.len() < FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2 {
            return None;
        }
        let marker = [record[0], record[1], record[2], record[3]];
        let declared_record_len = read_be16_at(record, FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET)?;
        let style_word = read_be16_at(record, 6).unwrap_or_default();
        let bbox = fdm_vector_command_bbox(record);
        let path_points = fdm_vector_command_path_points(record, marker);
        let curve_segments = fdm_vector_command_curve_segments(record, marker, &path_points);
        let ellipse = fdm_vector_command_ellipse(record, marker);
        let compound_child_layout = fdm_vector_compound_child_layout(record);
        let compound_child_offsets = fdm_vector_compound_child_offsets(record);
        let gradient_colors = style_context.and_then(|style| style.gradient_colors);
        Some(Self {
            command_index,
            relative_offset,
            source_vector_relative_offset: None,
            source_segment: None,
            record_len: next_offset.saturating_sub(relative_offset),
            declared_record_len,
            style_word,
            marker,
            bbox,
            path_points,
            curve_segments,
            ellipse,
            compound_child_offsets,
            compound_child_layout,
            gradient_colors,
            fill_color: style_context.and_then(|style| style.fill_color),
            stroke_color: style_context.and_then(|style| style.stroke_color),
        })
    }

    pub(super) fn with_source_vector_relative_offset(mut self, relative_offset: usize) -> Self {
        self.source_vector_relative_offset = Some(relative_offset);
        self
    }

    pub(super) fn with_source_segment(
        mut self,
        source_segment: ObjectFdmVectorCommandSourceSegment,
    ) -> Self {
        self.source_segment = Some(source_segment);
        self
    }

    pub fn command_index(&self) -> usize {
        self.command_index
    }

    pub fn relative_offset(&self) -> usize {
        self.relative_offset
    }

    pub fn source_vector_relative_offset(&self) -> Option<usize> {
        self.source_vector_relative_offset
    }

    pub fn source_segment(&self) -> Option<ObjectFdmVectorCommandSourceSegment> {
        self.source_segment
    }

    pub fn record_len(&self) -> usize {
        self.record_len
    }

    pub fn declared_record_len(&self) -> u16 {
        self.declared_record_len
    }

    pub fn style_word(&self) -> u16 {
        self.style_word
    }

    pub fn marker(&self) -> &[u8; 4] {
        &self.marker
    }

    pub fn bbox(&self) -> Option<ObjectFdmIndexBbox> {
        self.bbox
    }

    pub fn path_points(&self) -> &[ObjectFdmVectorPoint] {
        &self.path_points
    }

    pub fn curve_segments(&self) -> &[ObjectFdmVectorCurveSegment] {
        &self.curve_segments
    }

    pub fn ellipse(&self) -> Option<ObjectFdmVectorEllipse> {
        self.ellipse
    }

    pub fn compound_child_offsets(&self) -> &[u16] {
        &self.compound_child_offsets
    }

    pub(super) fn compound_child_layout(&self) -> Option<&FdmCompoundChildLayout> {
        self.compound_child_layout.as_ref()
    }

    pub(super) fn gradient_colors(&self) -> Option<FdmVectorGradientContext> {
        self.gradient_colors
    }

    pub fn fill_color(&self) -> Option<u32> {
        self.fill_color
    }

    pub fn stroke_color(&self) -> Option<u32> {
        self.stroke_color
    }

    pub(super) fn has_renderable_geometry(&self) -> bool {
        self.path_points.len() >= 2 || self.ellipse.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmVectorCommandSourceSegment {
    pub(super) relative_offset: usize,
    pub(super) local_offset: usize,
    pub(super) declared_len: u16,
    pub(super) command_count: u16,
    pub(super) command_index: usize,
    pub(super) command_offset: u16,
}

impl ObjectFdmVectorCommandSourceSegment {
    pub(super) fn new(
        relative_offset: usize,
        local_offset: usize,
        header: &FdmVectorSegmentHeader,
        command_index: usize,
        command_offset: u16,
    ) -> Self {
        Self {
            relative_offset,
            local_offset,
            declared_len: header.declared_len,
            command_count: header.command_count,
            command_index,
            command_offset,
        }
    }

    pub fn relative_offset(self) -> usize {
        self.relative_offset
    }

    pub fn local_offset(self) -> usize {
        self.local_offset
    }

    pub fn declared_len(self) -> u16 {
        self.declared_len
    }

    pub fn command_count(self) -> u16 {
        self.command_count
    }

    pub fn command_index(self) -> usize {
        self.command_index
    }

    pub fn command_offset(self) -> u16 {
        self.command_offset
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmVectorStyleContext {
    pub(super) fill_color: Option<u32>,
    pub(super) stroke_color: Option<u32>,
    pub(super) gradient_colors: Option<FdmVectorGradientContext>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmVectorGradientContext {
    pub(super) from_color: u32,
    pub(super) to_color: u32,
}

impl FdmVectorGradientContext {
    pub(super) fn new(from_color: u32, to_color: u32) -> Self {
        Self {
            from_color,
            to_color,
        }
    }

    pub(super) fn start_color(self) -> u32 {
        self.from_color
    }

    pub(super) fn end_color(self) -> u32 {
        self.to_color
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmVectorPoint {
    pub(super) x: i32,
    pub(super) y: i32,
}

impl ObjectFdmVectorPoint {
    pub(super) fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub(super) fn offset(self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x.saturating_add(dx),
            y: self.y.saturating_add(dy),
        }
    }

    pub fn x(self) -> i32 {
        self.x
    }

    pub fn y(self) -> i32 {
        self.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmVectorCurveSegment {
    pub(super) control_1: ObjectFdmVectorPoint,
    pub(super) control_2: ObjectFdmVectorPoint,
}

impl ObjectFdmVectorCurveSegment {
    pub(super) fn new(control_1: ObjectFdmVectorPoint, control_2: ObjectFdmVectorPoint) -> Self {
        Self {
            control_1,
            control_2,
        }
    }

    pub fn control_1(self) -> ObjectFdmVectorPoint {
        self.control_1
    }

    pub fn control_2(self) -> ObjectFdmVectorPoint {
        self.control_2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmVectorEllipse {
    pub(super) center: ObjectFdmVectorPoint,
    pub(super) radius_x: i32,
    pub(super) radius_y: i32,
    pub(super) color: Option<u32>,
}

impl ObjectFdmVectorEllipse {
    pub(super) fn new(
        center: ObjectFdmVectorPoint,
        radius_x: i32,
        radius_y: i32,
        color: Option<u32>,
    ) -> Self {
        Self {
            center,
            radius_x,
            radius_y,
            color,
        }
    }

    pub fn center(self) -> ObjectFdmVectorPoint {
        self.center
    }

    pub fn radius_x(self) -> i32 {
        self.radius_x
    }

    pub fn radius_y(self) -> i32 {
        self.radius_y
    }

    pub fn color(self) -> Option<u32> {
        self.color
    }
}

pub(super) fn attach_object_stream_fdm_index_entries(
    candidates: &mut [ObjectStreamCandidate],
    streams: &[(String, Vec<u8>)],
    budget: &mut ResourceBudget,
) -> Result<()> {
    for candidate in candidates {
        if fdm_index_path_for_vector(candidate.path()).is_none() {
            continue;
        }
        let Some((_, vector_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(candidate.path()))
        else {
            continue;
        };
        candidate.set_fdm_raw_vector_segments(fdm_raw_vector_segment_candidates(vector_stream));
        candidate.set_fdm_raw_vector_commands(fdm_raw_vector_command_candidates(vector_stream));
        let Some((actual_index_path, index_stream)) =
            fdm_index_stream_for_vector(candidate.path(), vector_stream.len(), streams)
        else {
            continue;
        };

        let all_entries = parse_fdm_index_entries(index_stream, vector_stream.len());
        let entries = fdm_index_declared_entries(index_stream, &all_entries);
        if entries.is_empty() {
            continue;
        }
        let vector_hits = image_signature_hits(vector_stream, budget)?;
        let mut fdm_entries = Vec::new();
        for entry in entries {
            let segment = fdm_vector_segment(entry.vector_offset, entries, vector_stream);
            let segment_hits =
                fdm_segment_signature_hits(&vector_hits, segment.start, segment.end, budget)?;
            let relative_hits = fdm_relative_signature_hits(&segment_hits, segment.start, budget)?;
            let vector_prefix = vector_stream
                .get(segment.start..segment.end)
                .unwrap_or_default();
            let vector_commands = fdm_vector_command_candidates(vector_prefix, segment.start);
            let connector_candidates = fdm_connector_candidates(&vector_commands);

            fdm_entries.push(ObjectFdmIndexEntryCandidate {
                index_path: actual_index_path.clone(),
                vector_path: candidate.path().to_string(),
                row_index: entry.row_index,
                index_offset: entry.index_offset,
                vector_offset: entry.vector_offset,
                next_vector_offset: segment.end,
                vector_len: segment.end.saturating_sub(segment.start),
                kind: entry.kind,
                bbox: ObjectFdmIndexBbox::new(entry.left, entry.top, entry.right, entry.bottom),
                valid_vector_offset: entry.valid_vector_offset,
                vector_prefix: vector_prefix
                    [..vector_prefix.len().min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
                    .to_vec(),
                image_signature_hits: segment_hits,
                segment_image_signature_hits: relative_hits,
                vector_commands,
                connector_candidates,
            });
        }
        candidate.set_fdm_index_entry_candidates(fdm_entries);
    }
    Ok(())
}

pub(super) fn attach_object_stream_fdm_text_index_entries(
    candidates: &mut [ObjectStreamCandidate],
    streams: &[(String, Vec<u8>)],
) {
    for candidate in candidates {
        let Some(index_path) = fdm_index_path_for_text(candidate.path()) else {
            continue;
        };
        let Some((_, text_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(candidate.path()))
        else {
            continue;
        };
        let Some((actual_index_path, index_stream)) = streams
            .iter()
            .find(|(path, _)| path.eq_ignore_ascii_case(&index_path))
        else {
            continue;
        };

        let entries = parse_fdm_text_index_entries(index_stream, text_stream);
        let fdm_text_entries = entries
            .iter()
            .map(|entry| {
                let text_record_prefix = text_stream
                    .get(entry.text_record_offset..)
                    .unwrap_or_default()[..text_stream
                    .len()
                    .saturating_sub(entry.text_record_offset)
                    .min(OBJECT_STREAM_PREFIX_PREVIEW_BYTES)]
                    .to_vec();
                ObjectFdmTextIndexEntryCandidate {
                    index_path: actual_index_path.clone(),
                    text_path: candidate.path().to_string(),
                    row_index: entry.row_index,
                    index_offset: entry.index_offset,
                    text_record_offset: entry.text_record_offset,
                    kind: entry.kind,
                    bbox: entry.bbox,
                    text_record_bbox: fdm_text_candidate_bbox(
                        text_stream,
                        entry.text_record_offset,
                    ),
                    valid_text_record_offset: true,
                    text_record_prefix,
                }
            })
            .collect::<Vec<_>>();
        if !fdm_text_entries.is_empty() {
            candidate.set_fdm_text_index_entry_candidates(fdm_text_entries);
        }
    }
}

pub(super) fn fdm_text_record_marker_at(stream: &[u8], offset: usize) -> Option<[u8; 4]> {
    let marker: [u8; 4] = stream
        .get(offset..offset.saturating_add(4))?
        .try_into()
        .ok()?;
    (marker[0] == 0x01 && marker[1] == 0x00 && marker[3] == 0x60).then_some(marker)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FdmVectorSegmentHeader {
    pub(super) declared_len: u16,
    pub(super) command_count: u16,
    pub(super) command_offsets: Vec<u16>,
    pub(super) bbox: Option<ObjectFdmIndexBbox>,
    pub(super) source_width: i32,
    pub(super) source_height: i32,
}

pub(super) fn fdm_raw_vector_segment_candidates(
    vector_stream: &[u8],
) -> Vec<ObjectFdmVectorSegmentCandidate> {
    let mut segments = Vec::new();
    let mut offset = 0usize;
    while offset + FDM_VECTOR_SEGMENT_HEADER_BYTES <= vector_stream.len() {
        if !vector_stream[offset..].starts_with(FDM_VECTOR_SEGMENT_MAGIC) {
            offset += 1;
            continue;
        }
        let Some(header) = fdm_vector_segment_header(&vector_stream[offset..]) else {
            offset += 1;
            continue;
        };
        let declared_len = usize::from(header.declared_len).max(1);
        segments.push(ObjectFdmVectorSegmentCandidate::new(offset, header));
        offset += declared_len;
    }
    segments
}

pub(super) fn fdm_vector_segment_header(segment: &[u8]) -> Option<FdmVectorSegmentHeader> {
    if segment.len() < FDM_VECTOR_SEGMENT_HEADER_BYTES
        || !segment.starts_with(FDM_VECTOR_SEGMENT_MAGIC)
    {
        return None;
    }

    let declared_len = read_be16_at(segment, 4)?;
    let command_count = read_be16_at(segment, 6)?;
    let declared_len_usize = usize::from(declared_len);
    let command_count_usize = usize::from(command_count);
    if declared_len_usize < FDM_VECTOR_SEGMENT_HEADER_BYTES || declared_len_usize > segment.len() {
        return None;
    }

    let offset_table_end =
        FDM_VECTOR_SEGMENT_HEADER_BYTES + command_count_usize * FDM_VECTOR_COMMAND_OFFSET_BYTES;
    if offset_table_end > declared_len_usize {
        return None;
    }

    let mut command_offsets = Vec::with_capacity(command_count_usize);
    for command_index in 0..command_count_usize {
        let offset_start =
            FDM_VECTOR_SEGMENT_HEADER_BYTES + command_index * FDM_VECTOR_COMMAND_OFFSET_BYTES;
        let offset = read_be16_at(segment, offset_start)?;
        let offset_usize = usize::from(offset);
        if offset_usize < offset_table_end || offset_usize >= declared_len_usize {
            return None;
        }
        command_offsets.push(offset);
    }

    let bbox = Some(ObjectFdmIndexBbox::new(
        read_i32_be_at(segment, 20)?,
        read_i32_be_at(segment, 24)?,
        read_i32_be_at(segment, 28)?,
        read_i32_be_at(segment, 32)?,
    ));
    let source_width = read_i32_be_at(segment, 36).unwrap_or_default();
    let source_height = read_i32_be_at(segment, 40).unwrap_or_default();
    Some(FdmVectorSegmentHeader {
        declared_len,
        command_count,
        command_offsets,
        bbox,
        source_width,
        source_height,
    })
}

pub(super) fn fdm_vector_command_source_segment_for_vector_offset(
    segments: &[ObjectFdmVectorSegmentCandidate],
    vector_offset: usize,
) -> Option<ObjectFdmVectorCommandSourceSegment> {
    segments.iter().find_map(|segment| {
        let segment_start = segment.relative_offset();
        let segment_end = segment_start.saturating_add(usize::from(segment.declared_len()));
        if vector_offset < segment_start || vector_offset >= segment_end {
            return None;
        }
        let header = FdmVectorSegmentHeader {
            declared_len: segment.declared_len(),
            command_count: segment.command_count(),
            command_offsets: segment.command_offsets().to_vec(),
            bbox: segment.bbox(),
            source_width: segment.source_width(),
            source_height: segment.source_height(),
        };
        fdm_vector_command_source_segment_for_local_offset(
            segment_start,
            &header,
            vector_offset.saturating_sub(segment_start),
        )
    })
}

pub(super) fn fdm_vector_command_source_segment_for_local_offset(
    segment_relative_offset: usize,
    header: &FdmVectorSegmentHeader,
    local_offset: usize,
) -> Option<ObjectFdmVectorCommandSourceSegment> {
    let declared_len = usize::from(header.declared_len);
    if local_offset >= declared_len {
        return None;
    }
    header.command_offsets.iter().copied().enumerate().find_map(
        |(command_index, command_offset)| {
            let command_offset_usize = usize::from(command_offset);
            let next_offset = header
                .command_offsets
                .get(command_index + 1)
                .copied()
                .map(usize::from)
                .unwrap_or(declared_len);
            (command_offset_usize <= local_offset && local_offset < next_offset).then(|| {
                ObjectFdmVectorCommandSourceSegment::new(
                    segment_relative_offset,
                    local_offset,
                    header,
                    command_index,
                    command_offset,
                )
            })
        },
    )
}

pub(super) fn fdm_raw_vector_command_candidates(
    vector_stream: &[u8],
) -> Vec<ObjectFdmVectorCommandCandidate> {
    let segments = fdm_raw_vector_segment_candidates(vector_stream);
    let mut commands = Vec::new();
    let mut offset = 0usize;
    while offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2 <= vector_stream.len() {
        let marker = [
            vector_stream[offset],
            vector_stream[offset + 1],
            vector_stream[offset + 2],
            vector_stream[offset + 3],
        ];
        if !FDM_VECTOR_RENDERED_PRIMITIVE_MARKERS.contains(&marker) {
            offset += 1;
            continue;
        }
        let Some(record_len) = read_be16_at(
            vector_stream,
            offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET,
        )
        .map(usize::from) else {
            offset += 1;
            continue;
        };
        if record_len < FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2
            || offset + record_len > vector_stream.len()
        {
            offset += 1;
            continue;
        }
        let record = &vector_stream[offset..offset + record_len];
        if let Some(command) = ObjectFdmVectorCommandCandidate::new(
            commands.len(),
            offset,
            record,
            offset + record_len,
            None,
        ) && command.has_renderable_geometry()
        {
            let command = command.with_source_vector_relative_offset(offset);
            let command = if let Some(source_segment) =
                fdm_vector_command_source_segment_for_vector_offset(&segments, offset)
            {
                command.with_source_segment(source_segment)
            } else {
                command
            };
            commands.push(command);
        }
        offset += record_len.max(1);
    }
    commands
}

pub(super) fn fdm_vector_command_candidates(
    segment: &[u8],
    segment_relative_offset: usize,
) -> Vec<ObjectFdmVectorCommandCandidate> {
    let Some(header) = fdm_vector_segment_header(segment) else {
        return Vec::new();
    };
    let segment_len = usize::from(header.declared_len);
    let offsets = header
        .command_offsets
        .iter()
        .map(|offset| usize::from(*offset))
        .collect::<Vec<_>>();

    let mut commands = Vec::new();
    for (command_index, relative_offset) in offsets.iter().enumerate() {
        let next_offset = offsets
            .get(command_index + 1)
            .copied()
            .unwrap_or(segment_len);
        if next_offset <= *relative_offset || next_offset > segment_len {
            continue;
        }
        let Some(record) = segment.get(*relative_offset..next_offset) else {
            continue;
        };
        let Some(command) = ObjectFdmVectorCommandCandidate::new(
            command_index,
            *relative_offset,
            record,
            next_offset,
            None,
        ) else {
            continue;
        };
        let command =
            command.with_source_vector_relative_offset(segment_relative_offset + *relative_offset);
        let command = if let Some(source_segment) =
            fdm_vector_command_source_segment_for_local_offset(
                segment_relative_offset,
                &header,
                *relative_offset,
            ) {
            command.with_source_segment(source_segment)
        } else {
            command
        };
        commands.push(command);
        commands.extend(fdm_vector_nested_primitive_command_candidates(
            command_index,
            *relative_offset,
            segment_relative_offset,
            &header,
            record,
        ));
    }
    commands
}

pub(super) fn fdm_vector_nested_primitive_command_candidates(
    parent_command_index: usize,
    parent_relative_offset: usize,
    segment_relative_offset: usize,
    header: &FdmVectorSegmentHeader,
    record: &[u8],
) -> Vec<ObjectFdmVectorCommandCandidate> {
    if !record.starts_with(FDM_VECTOR_COMMAND_BBOX_MARKER) {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    let mut scan_offset = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16;
    let style_context = fdm_vector_compound_style_context(record);
    let table_offsets = fdm_vector_compound_child_offsets(record);
    for (nested_index, nested_offset) in table_offsets.iter().copied().enumerate() {
        let nested_offset = usize::from(nested_offset);
        if let Some(candidate) = fdm_vector_nested_primitive_command_candidate_at(
            parent_command_index,
            nested_index,
            parent_relative_offset,
            segment_relative_offset,
            header,
            record,
            nested_offset,
            style_context,
        ) {
            candidates.push(candidate);
        }
    }
    if !candidates.is_empty() {
        return candidates;
    }

    let mut nested_index = 0usize;
    while scan_offset + 8 <= record.len() {
        let Some((nested_offset, _marker)) =
            find_fdm_vector_nested_primitive_marker(record, scan_offset)
        else {
            break;
        };
        if let Some(candidate) = fdm_vector_nested_primitive_command_candidate_at(
            parent_command_index,
            nested_index,
            parent_relative_offset,
            segment_relative_offset,
            header,
            record,
            nested_offset,
            style_context,
        ) {
            scan_offset = nested_offset + candidate.declared_record_len() as usize;
            candidates.push(candidate);
        } else {
            scan_offset = nested_offset + 1;
        };
        nested_index += 1;
    }
    candidates
}

#[allow(clippy::too_many_arguments)]
pub(super) fn fdm_vector_nested_primitive_command_candidate_at(
    parent_command_index: usize,
    nested_index: usize,
    parent_relative_offset: usize,
    segment_relative_offset: usize,
    header: &FdmVectorSegmentHeader,
    record: &[u8],
    nested_offset: usize,
    style_context: Option<FdmVectorStyleContext>,
) -> Option<ObjectFdmVectorCommandCandidate> {
    if nested_offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2 > record.len()
        || !FDM_VECTOR_NESTED_PRIMITIVE_MARKERS
            .iter()
            .any(|marker| record[nested_offset..].starts_with(marker))
    {
        return None;
    }
    let nested_len = read_be16_at(
        record,
        nested_offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET,
    )
    .map(usize::from)?;
    if nested_len < FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2
        || nested_offset + nested_len > record.len()
    {
        return None;
    }

    let child_relative_offset = parent_relative_offset + nested_offset;
    let child_next_offset = child_relative_offset + nested_len;
    let synthetic_command_index = parent_command_index * 1000 + nested_index + 1;
    ObjectFdmVectorCommandCandidate::new(
        synthetic_command_index,
        child_relative_offset,
        &record[nested_offset..nested_offset + nested_len],
        child_next_offset,
        style_context,
    )
    .map(|command| {
        let command = command
            .with_source_vector_relative_offset(segment_relative_offset + child_relative_offset);
        if let Some(source_segment) = fdm_vector_command_source_segment_for_local_offset(
            segment_relative_offset,
            header,
            child_relative_offset,
        ) {
            command.with_source_segment(source_segment)
        } else {
            command
        }
    })
    .filter(ObjectFdmVectorCommandCandidate::has_renderable_geometry)
}

pub(super) fn fdm_vector_compound_style_context(record: &[u8]) -> Option<FdmVectorStyleContext> {
    if !record.starts_with(FDM_VECTOR_COMMAND_BBOX_MARKER) {
        return None;
    }

    let prefix_start = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16;
    let (first_child_offset, _) = find_fdm_vector_nested_primitive_marker(record, prefix_start)?;
    if first_child_offset <= prefix_start {
        return None;
    }
    let prefix = record.get(prefix_start..first_child_offset)?;
    let fill_color = fdm_vector_prefix_color(prefix, 0);
    let stroke_color = fdm_vector_prefix_color(prefix, 4);
    let gradient_colors = fdm_vector_compound_gradient_context(record, fill_color, stroke_color);
    if fill_color.is_none() && stroke_color.is_none() && gradient_colors.is_none() {
        None
    } else {
        Some(FdmVectorStyleContext {
            fill_color,
            stroke_color,
            gradient_colors,
        })
    }
}

pub(super) fn fdm_vector_compound_gradient_context(
    record: &[u8],
    fill_color: Option<u32>,
    stroke_color: Option<u32>,
) -> Option<FdmVectorGradientContext> {
    if read_be16_at(record, 6)? != 0x0001 {
        return None;
    }
    let fill_color = fill_color?;
    let stroke_color = stroke_color?;
    if fill_color == stroke_color {
        return None;
    }
    if fdm_vector_compound_child_offsets(record).len() != 1 {
        return None;
    }
    Some(FdmVectorGradientContext::new(stroke_color, fill_color))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FdmCompoundChildLayout {
    pub(super) child_offsets: Vec<u16>,
    pub(super) first_child_matches_prefix_end: bool,
    pub(super) child_offsets_strictly_increasing: bool,
    pub(super) child_records_fit_parent: bool,
    pub(super) child_records_do_not_overlap: bool,
}

impl FdmCompoundChildLayout {
    pub(super) fn child_offsets(&self) -> &[u16] {
        &self.child_offsets
    }

    pub(super) fn first_child_matches_prefix_end(&self) -> bool {
        self.first_child_matches_prefix_end
    }

    pub(super) fn child_offsets_strictly_increasing(&self) -> bool {
        self.child_offsets_strictly_increasing
    }

    pub(super) fn child_records_fit_parent(&self) -> bool {
        self.child_records_fit_parent
    }

    pub(super) fn child_records_do_not_overlap(&self) -> bool {
        self.child_records_do_not_overlap
    }

    pub(super) fn is_valid_for_nested_projection(&self) -> bool {
        self.first_child_matches_prefix_end
            && self.child_offsets_strictly_increasing
            && self.child_records_fit_parent
            && self.child_records_do_not_overlap
    }
}

pub(super) fn fdm_vector_compound_child_layout(record: &[u8]) -> Option<FdmCompoundChildLayout> {
    let prefix = fdm_vector_compound_prefix(record)?;
    if prefix.len() < 10 || prefix.len() % 2 != 0 {
        return None;
    }
    let child_offsets = prefix[8..]
        .chunks_exact(2)
        .filter_map(|chunk| read_be16_at(chunk, 0))
        .collect::<Vec<_>>();
    if child_offsets.is_empty() {
        return None;
    }
    let first_child_offset = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16 + prefix.len();
    let first_child_matches_prefix_end = child_offsets
        .first()
        .is_some_and(|offset| usize::from(*offset) == first_child_offset);
    let child_offsets_strictly_increasing = child_offsets.windows(2).all(|pair| pair[0] < pair[1]);
    let child_records = child_offsets
        .iter()
        .map(|offset| {
            let offset = usize::from(*offset);
            let marker_valid = offset < record.len()
                && FDM_VECTOR_NESTED_PRIMITIVE_MARKERS
                    .iter()
                    .any(|marker| record[offset..].starts_with(marker));
            let declared_end = marker_valid
                .then(|| {
                    read_be16_at(record, offset + FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET)
                        .map(usize::from)
                        .and_then(|length| {
                            (length >= FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET + 2)
                                .then_some(offset.saturating_add(length))
                        })
                })
                .flatten();
            (offset, declared_end)
        })
        .collect::<Vec<_>>();
    let child_records_fit_parent = child_records
        .iter()
        .all(|(_, end)| end.is_some_and(|end| end <= record.len()));
    let child_records_do_not_overlap = child_records
        .windows(2)
        .all(|pair| pair[0].1.is_some_and(|first_end| first_end <= pair[1].0));

    Some(FdmCompoundChildLayout {
        child_offsets,
        first_child_matches_prefix_end,
        child_offsets_strictly_increasing,
        child_records_fit_parent,
        child_records_do_not_overlap,
    })
}

pub(super) fn fdm_vector_compound_child_offsets(record: &[u8]) -> Vec<u16> {
    let Some(prefix) = fdm_vector_compound_prefix(record) else {
        return Vec::new();
    };
    if prefix.len() < 10 || prefix.len() % 2 != 0 {
        return Vec::new();
    }
    let offsets = prefix[8..]
        .chunks_exact(2)
        .filter_map(|chunk| read_be16_at(chunk, 0))
        .collect::<Vec<_>>();
    if offsets.is_empty() {
        return Vec::new();
    }
    let first_child_offset = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16 + prefix.len();
    if offsets
        .first()
        .is_some_and(|offset| usize::from(*offset) == first_child_offset)
        && offsets.iter().all(|offset| {
            usize::from(*offset) >= first_child_offset && usize::from(*offset) <= record.len()
        })
    {
        offsets
            .into_iter()
            .filter(|offset| {
                let offset = usize::from(*offset);
                offset < record.len()
                    && FDM_VECTOR_NESTED_PRIMITIVE_MARKERS
                        .iter()
                        .any(|marker| record[offset..].starts_with(marker))
            })
            .collect()
    } else {
        Vec::new()
    }
}

pub(super) fn fdm_vector_compound_prefix(record: &[u8]) -> Option<&[u8]> {
    if !record.starts_with(FDM_VECTOR_COMMAND_BBOX_MARKER) {
        return None;
    }
    let prefix_start = FDM_VECTOR_COMMAND_BBOX_OFFSET + 16;
    let (first_child_offset, _) = find_fdm_vector_nested_primitive_marker(record, prefix_start)?;
    if first_child_offset <= prefix_start {
        return None;
    }
    record.get(prefix_start..first_child_offset)
}

pub(super) fn fdm_vector_prefix_color(prefix: &[u8], offset: usize) -> Option<u32> {
    let color = read_be32_at(prefix, offset)?;
    if color > 0x00ff_ffff {
        return None;
    }
    if color == 0
        || color == 0x00ff_ffff
        || color >= 0x0001_0000
        || fdm_vector_is_grayscale_color(color)
    {
        Some(color)
    } else {
        None
    }
}

pub(super) fn fdm_vector_is_grayscale_color(color: u32) -> bool {
    let red = (color >> 16) & 0xff;
    let green = (color >> 8) & 0xff;
    let blue = color & 0xff;
    red == green && green == blue
}

pub(super) fn find_fdm_vector_nested_primitive_marker(
    record: &[u8],
    start_offset: usize,
) -> Option<(usize, [u8; 4])> {
    let mut best: Option<(usize, [u8; 4])> = None;
    for marker in FDM_VECTOR_NESTED_PRIMITIVE_MARKERS {
        let Some(position) = find_subslice_offsets(&record[start_offset..], &marker)
            .into_iter()
            .next()
        else {
            continue;
        };
        let offset = start_offset + position;
        if best.is_none_or(|(best_offset, _)| offset < best_offset) {
            best = Some((offset, marker));
        }
    }
    best
}

pub(super) fn fdm_vector_command_bbox(record: &[u8]) -> Option<ObjectFdmIndexBbox> {
    if !record.starts_with(FDM_VECTOR_COMMAND_BBOX_MARKER)
        || record.len() < FDM_VECTOR_COMMAND_BBOX_OFFSET + 16
    {
        return None;
    }
    let left = read_i32_be_at(record, FDM_VECTOR_COMMAND_BBOX_OFFSET)?;
    let top = read_i32_be_at(record, FDM_VECTOR_COMMAND_BBOX_OFFSET + 4)?;
    let right = read_i32_be_at(record, FDM_VECTOR_COMMAND_BBOX_OFFSET + 8)?;
    let bottom = read_i32_be_at(record, FDM_VECTOR_COMMAND_BBOX_OFFSET + 12)?;
    Some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

pub(super) fn fdm_vector_command_ellipse(
    record: &[u8],
    marker: [u8; 4],
) -> Option<ObjectFdmVectorEllipse> {
    if !FDM_VECTOR_COMMAND_ELLIPSE_MARKERS.contains(&marker)
        || record.len() < FDM_VECTOR_COMMAND_ELLIPSE_RADIUS_OFFSET + 4
    {
        return None;
    }

    let center_x = read_i32_be_at(record, FDM_VECTOR_COMMAND_ELLIPSE_CENTER_OFFSET)?;
    let center_y = read_i32_be_at(record, FDM_VECTOR_COMMAND_ELLIPSE_CENTER_OFFSET + 4)?;
    let radius_x = read_be16_at(record, FDM_VECTOR_COMMAND_ELLIPSE_RADIUS_OFFSET).map(i32::from)?;
    let radius_y =
        read_be16_at(record, FDM_VECTOR_COMMAND_ELLIPSE_RADIUS_OFFSET + 2).map(i32::from)?;
    if radius_x <= 0 || radius_y <= 0 {
        return None;
    }
    let color = read_be32_at(record, FDM_VECTOR_COMMAND_ELLIPSE_COLOR_OFFSET);
    Some(ObjectFdmVectorEllipse::new(
        ObjectFdmVectorPoint::new(center_x, center_y),
        radius_x,
        radius_y,
        color,
    ))
}

pub(super) fn fdm_vector_command_curve_segments(
    record: &[u8],
    marker: [u8; 4],
    points: &[ObjectFdmVectorPoint],
) -> Vec<ObjectFdmVectorCurveSegment> {
    if !fdm_vector_marker_is_bezier_curve(&marker) || points.len() < 2 {
        return Vec::new();
    }

    let controls_start = FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET + points.len() * 8;
    let segment_count = points.len().saturating_sub(1);
    let mut segments = Vec::with_capacity(segment_count);
    for segment_index in 0..segment_count {
        let offset = controls_start + segment_index * 16;
        if offset + 16 > record.len() {
            break;
        }
        let Some(control_1_dx) = read_i32_be_at(record, offset) else {
            break;
        };
        let Some(control_1_dy) = read_i32_be_at(record, offset + 4) else {
            break;
        };
        let Some(control_2_dx) = read_i32_be_at(record, offset + 8) else {
            break;
        };
        let Some(control_2_dy) = read_i32_be_at(record, offset + 12) else {
            break;
        };
        let control_1 = points[segment_index].offset(control_1_dx, control_1_dy);
        let control_2 = points[segment_index + 1].offset(control_2_dx, control_2_dy);
        segments.push(ObjectFdmVectorCurveSegment::new(control_1, control_2));
    }
    segments
}

pub(super) fn fdm_vector_command_path_points(
    record: &[u8],
    marker: [u8; 4],
) -> Vec<ObjectFdmVectorPoint> {
    if fdm_vector_marker_is_line(&marker) {
        if record.len() < FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET + 16 {
            return Vec::new();
        }
        let Some(x1) = read_i32_be_at(record, FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET) else {
            return Vec::new();
        };
        let Some(y1) = read_i32_be_at(record, FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET + 4) else {
            return Vec::new();
        };
        let Some(x2) = read_i32_be_at(record, FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET + 8) else {
            return Vec::new();
        };
        let Some(y2) = read_i32_be_at(record, FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET + 12) else {
            return Vec::new();
        };
        if x1 == x2 && y1 == y2 {
            return Vec::new();
        }
        return vec![
            ObjectFdmVectorPoint::new(x1, y1),
            ObjectFdmVectorPoint::new(x2, y2),
        ];
    }

    if !FDM_VECTOR_COMMAND_PATH_MARKERS.contains(&marker)
        || record.len() < FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET
    {
        return Vec::new();
    }
    let Some(point_count) =
        read_be16_at(record, FDM_VECTOR_COMMAND_PATH_POINT_COUNT_OFFSET).map(usize::from)
    else {
        return Vec::new();
    };
    let points_end = FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET + point_count * 8;
    if point_count < 2 || points_end > record.len() {
        return Vec::new();
    }

    let mut points = Vec::with_capacity(point_count);
    for index in 0..point_count {
        let offset = FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET + index * 8;
        let Some(x) = read_i32_be_at(record, offset) else {
            return Vec::new();
        };
        let Some(y) = read_i32_be_at(record, offset + 4) else {
            return Vec::new();
        };
        points.push(ObjectFdmVectorPoint::new(x, y));
    }
    points
}

pub(super) fn fdm_vector_path_points_bbox(
    points: &[ObjectFdmVectorPoint],
) -> Option<ObjectFdmIndexBbox> {
    let mut iter = points.iter();
    let first = *iter.next()?;
    let mut left = first.x();
    let mut top = first.y();
    let mut right = first.x();
    let mut bottom = first.y();
    for point in iter {
        left = left.min(point.x());
        top = top.min(point.y());
        right = right.max(point.x());
        bottom = bottom.max(point.y());
    }
    Some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

pub(super) fn fdm_vector_ellipse_bbox(ellipse: ObjectFdmVectorEllipse) -> ObjectFdmIndexBbox {
    let center = ellipse.center();
    ObjectFdmIndexBbox::new(
        center.x().saturating_sub(ellipse.radius_x()),
        center.y().saturating_sub(ellipse.radius_y()),
        center.x().saturating_add(ellipse.radius_x()),
        center.y().saturating_add(ellipse.radius_y()),
    )
}

pub(super) fn fdm_vector_command_source_bbox(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmIndexBbox> {
    if !command.path_points().is_empty() {
        let mut points =
            Vec::with_capacity(command.path_points().len() + command.curve_segments().len() * 2);
        points.extend_from_slice(command.path_points());
        for segment in command.curve_segments() {
            points.push(segment.control_1());
            points.push(segment.control_2());
        }
        let bbox = fdm_vector_path_points_bbox(&points)?;
        return Some(bbox);
    }
    command.ellipse().map(fdm_vector_ellipse_bbox)
}

pub(super) fn fdm_connector_candidates(
    commands: &[ObjectFdmVectorCommandCandidate],
) -> Vec<ObjectFdmConnectorCandidate> {
    commands
        .iter()
        .filter_map(fdm_connector_candidate_from_command)
        .collect()
}

pub(super) fn fdm_connector_candidate_from_command(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmConnectorCandidate> {
    if command.ellipse().is_some() || fdm_vector_path_is_closed(command.path_points()) {
        return None;
    }
    let source_start = *command.path_points().first()?;
    let source_end = *command.path_points().last()?;
    let source_bbox = fdm_vector_command_source_bbox(command)?;
    let normalized = normalize_fdm_bbox(source_bbox);
    let source_width = normalized.2.saturating_sub(normalized.0);
    let source_height = normalized.3.saturating_sub(normalized.1);
    let source_span = source_width.max(source_height);
    if source_span < FDM_CONNECTOR_CANDIDATE_MIN_SOURCE_SPAN_UNITS {
        return None;
    }
    let endpoint_dx = source_end.x().saturating_sub(source_start.x());
    let endpoint_dy = source_end.y().saturating_sub(source_start.y());
    let dx = i64::from(endpoint_dx);
    let dy = i64::from(endpoint_dy);
    let endpoint_distance_squared = (dx.saturating_mul(dx) + dy.saturating_mul(dy)) as u64;
    let (path_segment_count, orthogonal_segment_count, diagonal_segment_count) =
        fdm_connector_path_segment_counts(command.path_points());

    Some(ObjectFdmConnectorCandidate {
        command_index: command.command_index(),
        relative_offset: command.relative_offset(),
        marker: *command.marker(),
        style_word: command.style_word(),
        primitive_kind: fdm_vector_primitive_kind(command),
        fill_color: command.fill_color(),
        stroke_color: command.stroke_color(),
        source_start,
        source_end,
        source_bbox,
        source_span,
        endpoint_dx,
        endpoint_dy,
        endpoint_distance_squared,
        path_point_count: command.path_points().len(),
        path_segment_count,
        orthogonal_segment_count,
        diagonal_segment_count,
        curve_segment_count: command.curve_segments().len(),
        compound_child_offset_count: command.compound_child_offsets().len(),
        axis_aligned: endpoint_dx == 0 || endpoint_dy == 0,
        orientation: fdm_connector_orientation(endpoint_dx as f32, endpoint_dy as f32),
        basis: "long-open-source-path",
    })
}

pub(super) fn fdm_connector_path_segment_counts(
    points: &[ObjectFdmVectorPoint],
) -> (usize, usize, usize) {
    let mut path_segment_count = 0usize;
    let mut orthogonal_segment_count = 0usize;
    let mut diagonal_segment_count = 0usize;
    for pair in points.windows(2) {
        let start = pair[0];
        let end = pair[1];
        path_segment_count += 1;
        if start.x() == end.x() || start.y() == end.y() {
            orthogonal_segment_count += 1;
        } else {
            diagonal_segment_count += 1;
        }
    }
    (
        path_segment_count,
        orthogonal_segment_count,
        diagonal_segment_count,
    )
}

pub(super) fn fdm_vector_path_is_closed(points: &[ObjectFdmVectorPoint]) -> bool {
    points.len() >= 3 && points.first() == points.last()
}

pub(super) fn fdm_vector_primitive_is_closed(command: &ObjectFdmVectorCommandCandidate) -> bool {
    command.ellipse().is_some() || fdm_vector_path_is_closed(command.path_points())
}

pub(super) fn fdm_vector_marker_is_bezier_curve(marker: &[u8; 4]) -> bool {
    marker == b"\xff\x00\x09\x60" || marker == b"\x00\x00\x09\x60" || marker == b"\x01\x00\x09\x60"
}

pub(super) fn fdm_vector_marker_is_line(marker: &[u8; 4]) -> bool {
    marker == FDM_VECTOR_COMMAND_LINE_MARKER
        || marker == FDM_VECTOR_COMMAND_NESTED_LINE_MARKER
        || marker == FDM_VECTOR_COMMAND_INDEXED_LINE_MARKER
}

pub(super) fn fdm_vector_primitive_kind(command: &ObjectFdmVectorCommandCandidate) -> &'static str {
    if command.ellipse().is_some() {
        "ellipse"
    } else if !command.curve_segments().is_empty() {
        "cubicBezier"
    } else if fdm_vector_marker_is_bezier_curve(command.marker()) {
        "quadraticBezier"
    } else {
        "polyline"
    }
}

pub(super) fn fdm_vector_stroke_width(command: &ObjectFdmVectorCommandCandidate) -> f32 {
    if command.ellipse().is_some() {
        return if command.style_word() == 0x0010 {
            2.250
        } else {
            0.720
        };
    }
    if fdm_vector_marker_is_bezier_curve(command.marker()) && command.style_word() == 0x0010 {
        return 2.250;
    }
    if fdm_vector_path_is_closed(command.path_points()) && command.fill_color().is_some() {
        return 0.139;
    }
    if fdm_vector_marker_is_line(command.marker()) {
        return 0.500;
    }
    match command.style_word() & 0x000f {
        0x0004 => 0.410,
        0x0005 => 0.480,
        0x0008 => 0.410,
        _ => 0.500,
    }
}

pub(super) fn fdm_vector_render_stroke_color(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> String {
    diagnostic
        .command
        .stroke_color()
        .and_then(fdm_vector_css_color)
        .unwrap_or_else(|| {
            if fdm_vector_uncolored_path_uses_light_stroke(diagnostic, diagnostics) {
                "#ffffff".to_string()
            } else {
                "#111111".to_string()
            }
        })
}

pub(super) fn fdm_vector_render_fill_color(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> String {
    if !fdm_vector_primitive_is_closed(diagnostic.command) {
        return "none".to_string();
    }
    let Some(fill_color) = diagnostic.command.fill_color() else {
        return "none".to_string();
    };
    if fdm_vector_filled_path_is_text_mask_outer(diagnostic, diagnostics) {
        return "#ffffff".to_string();
    }
    if fdm_vector_filled_path_is_text_mask_inner(diagnostic, diagnostics) {
        return "#000000".to_string();
    }
    if fdm_vector_filled_path_is_compound_hole(diagnostic, diagnostics) {
        return fdm_vector_containing_fill_color(diagnostic, diagnostics)
            .and_then(fdm_vector_css_color)
            .unwrap_or_else(|| "#111111".to_string());
    }
    fdm_vector_css_color(fill_color).unwrap_or_else(|| "none".to_string())
}

pub(super) fn fdm_vector_linear_gradient_colors(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<(String, String)> {
    if !fdm_vector_primitive_is_closed(command) {
        return None;
    }
    let gradient = command.gradient_colors()?;
    let from = fdm_vector_css_color(gradient.start_color())?;
    let to = fdm_vector_css_color(gradient.end_color())?;
    (from != to).then_some((from, to))
}

pub(super) fn fdm_vector_filled_path_is_text_mask_outer(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    let command = diagnostic.command;
    if !fdm_vector_text_mask_candidate(command)
        || !command.fill_color().is_some_and(fdm_vector_color_is_black)
    {
        return false;
    }
    let Some(outer_bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let outer_area = fdm_bbox_area(outer_bbox);
    if outer_area == 0 {
        return false;
    }

    diagnostics.iter().any(|other| {
        if other.candidate_index != diagnostic.candidate_index
            || other.entry.row_index() != diagnostic.entry.row_index()
            || other.command.command_index() == command.command_index()
            || !fdm_vector_text_mask_candidate(other.command)
            || !other
                .command
                .fill_color()
                .is_some_and(fdm_vector_color_is_white)
        {
            return false;
        }
        let Some(inner_bbox) =
            fdm_vector_command_source_bbox(other.command).map(normalize_fdm_bbox)
        else {
            return false;
        };
        fdm_vector_text_mask_area_ratio(outer_bbox, inner_bbox).is_some_and(|ratio| {
            fdm_bbox_contains(outer_bbox, inner_bbox)
                && ratio >= FDM_VECTOR_TEXT_MASK_MIN_INNER_AREA_RATIO
        })
    })
}

pub(super) fn fdm_vector_filled_path_is_text_mask_inner(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    let command = diagnostic.command;
    if !fdm_vector_text_mask_candidate(command)
        || !command.fill_color().is_some_and(fdm_vector_color_is_white)
    {
        return false;
    }
    let Some(inner_bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };

    diagnostics.iter().any(|other| {
        if other.candidate_index != diagnostic.candidate_index
            || other.entry.row_index() != diagnostic.entry.row_index()
            || other.command.command_index() == command.command_index()
            || !fdm_vector_text_mask_candidate(other.command)
            || !other
                .command
                .fill_color()
                .is_some_and(fdm_vector_color_is_black)
        {
            return false;
        }
        let Some(outer_bbox) =
            fdm_vector_command_source_bbox(other.command).map(normalize_fdm_bbox)
        else {
            return false;
        };
        fdm_vector_text_mask_area_ratio(outer_bbox, inner_bbox).is_some_and(|ratio| {
            fdm_bbox_contains(outer_bbox, inner_bbox)
                && ratio >= FDM_VECTOR_TEXT_MASK_MIN_INNER_AREA_RATIO
        })
    })
}

pub(super) fn fdm_vector_text_mask_candidate(command: &ObjectFdmVectorCommandCandidate) -> bool {
    command.marker() == b"\x00\x00\x06\x60"
        && command.style_word() == 0x0008
        && command.stroke_color().is_none()
        && command.ellipse().is_none()
        && command.fill_color().is_some()
        && fdm_vector_path_is_closed(command.path_points())
}

pub(super) fn fdm_vector_text_mask_area_ratio(
    outer_bbox: (i32, i32, i32, i32),
    inner_bbox: (i32, i32, i32, i32),
) -> Option<f64> {
    let outer_area = fdm_bbox_area(outer_bbox);
    let inner_area = fdm_bbox_area(inner_bbox);
    if outer_area == 0 || inner_area == 0 {
        return None;
    }
    let ratio = inner_area as f64 / outer_area as f64;
    (FDM_VECTOR_TEXT_MASK_MIN_INNER_AREA_RATIO..=FDM_VECTOR_TEXT_MASK_MAX_INNER_AREA_RATIO)
        .contains(&ratio)
        .then_some(ratio)
}

pub(super) fn fdm_vector_filled_path_is_counter_overlay(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    fdm_vector_filled_path_is_compound_hole(diagnostic, diagnostics)
        || fdm_vector_filled_path_is_text_mask_inner(diagnostic, diagnostics)
}

pub(super) fn fdm_vector_filled_path_is_compound_hole(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    let command = diagnostic.command;
    if command.ellipse().is_some()
        || command.command_index() < 1000
        || command.fill_color().is_none()
        || !fdm_vector_path_is_closed(command.path_points())
    {
        return false;
    }
    let Some(inner_bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let inner_area = fdm_bbox_area(inner_bbox);
    if inner_area == 0 {
        return false;
    }
    let parent_command_index = command.command_index() / 1000;
    diagnostics.iter().any(|other| {
        other.candidate_index == diagnostic.candidate_index
            && other.entry.row_index() == diagnostic.entry.row_index()
            && other.command.command_index() / 1000 == parent_command_index
            && other.command.command_index() != command.command_index()
            && other.command.command_index() >= 1000
            && other.command.ellipse().is_none()
            && other.command.fill_color() == command.fill_color()
            && fdm_vector_path_is_closed(other.command.path_points())
            && fdm_vector_command_source_bbox(other.command)
                .map(normalize_fdm_bbox)
                .is_some_and(|outer_bbox| {
                    fdm_bbox_contains(outer_bbox, inner_bbox)
                        && fdm_bbox_area(outer_bbox) > inner_area
                })
    })
}

pub(super) fn fdm_vector_containing_fill_color(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> Option<u32> {
    let inner_bbox = fdm_vector_command_source_bbox(diagnostic.command).map(normalize_fdm_bbox)?;
    diagnostics
        .iter()
        .filter(|other| {
            other.candidate_index == diagnostic.candidate_index
                && other.entry.row_index() == diagnostic.entry.row_index()
                && other.command.command_index() != diagnostic.command.command_index()
                && other.command.fill_color() != diagnostic.command.fill_color()
                && other.command.fill_color().is_some()
        })
        .filter_map(|other| {
            let outer_bbox =
                fdm_vector_command_source_bbox(other.command).map(normalize_fdm_bbox)?;
            fdm_bbox_contains(outer_bbox, inner_bbox)
                .then_some((fdm_bbox_area(outer_bbox), other.command.fill_color()?))
        })
        .min_by_key(|(area, _)| *area)
        .map(|(_, color)| color)
}

pub(super) fn fdm_vector_uncolored_path_uses_light_stroke(
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> bool {
    let command = diagnostic.command;
    if command.marker() != b"\xff\x00\x06\x60"
        || command.style_word() != 0x0004
        || command.fill_color().is_some()
        || command.stroke_color().is_some()
        || !fdm_vector_path_is_closed(command.path_points())
    {
        return false;
    }
    let Some(inner_bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };

    diagnostics.iter().any(|other| {
        other.candidate_index == diagnostic.candidate_index
            && other.entry.row_index() == diagnostic.entry.row_index()
            && other
                .command
                .fill_color()
                .is_some_and(fdm_vector_color_is_black)
            && fdm_vector_command_source_bbox(other.command)
                .map(normalize_fdm_bbox)
                .is_some_and(|outer_bbox| fdm_bbox_contains(outer_bbox, inner_bbox))
    })
}

pub(super) fn fdm_vector_color_is_black(color: u32) -> bool {
    color & 0x00ff_ffff == 0
}

pub(super) fn fdm_vector_color_is_white(color: u32) -> bool {
    color & 0x00ff_ffff == 0x00ff_ffff
}

pub(super) fn fdm_bbox_contains(outer: (i32, i32, i32, i32), inner: (i32, i32, i32, i32)) -> bool {
    outer.0 <= inner.0 && outer.1 <= inner.1 && outer.2 >= inner.2 && outer.3 >= inner.3
}

pub(super) fn fdm_bbox_intersects(left: (i32, i32, i32, i32), right: (i32, i32, i32, i32)) -> bool {
    left.0 < right.2 && right.0 < left.2 && left.1 < right.3 && right.1 < left.3
}

pub(super) fn fdm_bbox_area(bbox: (i32, i32, i32, i32)) -> i64 {
    let width = i64::from(bbox.2.saturating_sub(bbox.0).max(0));
    let height = i64::from(bbox.3.saturating_sub(bbox.1).max(0));
    width.saturating_mul(height)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FdmIndexEntry {
    pub(super) row_index: usize,
    pub(super) index_offset: usize,
    pub(super) vector_offset: usize,
    pub(super) kind: u16,
    pub(super) left: i32,
    pub(super) top: i32,
    pub(super) right: i32,
    pub(super) bottom: i32,
    pub(super) valid_vector_offset: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FdmTextIndexEntry {
    pub(super) row_index: usize,
    pub(super) index_offset: usize,
    pub(super) text_record_offset: usize,
    pub(super) kind: u16,
    pub(super) bbox: ObjectFdmIndexBbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmVectorSegment {
    pub(super) start: usize,
    pub(super) end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct FdmProjectionViewport {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) width: f32,
    pub(super) height: f32,
}

pub(super) fn fdm_index_path_for_vector(vector_path: &str) -> Option<String> {
    if !vector_path
        .get(vector_path.len().saturating_sub("/FDMVector".len())..)?
        .eq_ignore_ascii_case("/FDMVector")
    {
        return None;
    }
    vector_path
        .get(..vector_path.len().saturating_sub("/FDMVector".len()))
        .map(|prefix| format!("{prefix}/FDMIndex"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmIndexVectorPairScore {
    pub(super) valid_declared_rows: usize,
    pub(super) declared_rows: usize,
    pub(super) invalid_declared_rows: usize,
}

impl FdmIndexVectorPairScore {
    pub(super) fn compare(self, other: Self) -> Ordering {
        self.valid_declared_rows
            .cmp(&other.valid_declared_rows)
            .then_with(|| other.invalid_declared_rows.cmp(&self.invalid_declared_rows))
            .then_with(|| self.declared_rows.cmp(&other.declared_rows))
    }
}

pub(super) fn fdm_index_stream_for_vector<'a>(
    vector_path: &str,
    vector_len: usize,
    streams: &'a [(String, Vec<u8>)],
) -> Option<(&'a String, &'a Vec<u8>)> {
    let exact_index_path = fdm_index_path_for_vector(vector_path)?;
    if let Some((path, stream)) = streams
        .iter()
        .find(|(path, _)| path.eq_ignore_ascii_case(&exact_index_path))
        && fdm_index_vector_pair_score(stream, vector_len).is_some()
    {
        return Some((path, stream));
    }

    let mut best: Option<(&String, &Vec<u8>, FdmIndexVectorPairScore)> = None;
    let mut tied_best = false;
    for (path, stream) in streams
        .iter()
        .filter(|(path, _)| stream_path_ends_with(path, "/FDMIndex"))
    {
        let Some(score) = fdm_index_vector_pair_score(stream, vector_len) else {
            continue;
        };
        match best {
            Some((_, _, best_score)) => match score.compare(best_score) {
                Ordering::Greater => {
                    best = Some((path, stream, score));
                    tied_best = false;
                }
                Ordering::Equal => tied_best = true,
                Ordering::Less => {}
            },
            None => {
                best = Some((path, stream, score));
                tied_best = false;
            }
        }
    }

    if tied_best {
        return None;
    }
    best.map(|(path, stream, _)| (path, stream))
}

pub(super) fn fdm_index_vector_pair_score(
    index_stream: &[u8],
    vector_len: usize,
) -> Option<FdmIndexVectorPairScore> {
    let all_entries = parse_fdm_index_entries(index_stream, vector_len);
    let entries = fdm_index_declared_entries(index_stream, &all_entries);
    if entries.is_empty() {
        return None;
    }
    let valid_declared_rows = entries
        .iter()
        .filter(|entry| entry.valid_vector_offset)
        .count();
    if valid_declared_rows == 0 {
        return None;
    }
    Some(FdmIndexVectorPairScore {
        valid_declared_rows,
        declared_rows: entries.len(),
        invalid_declared_rows: entries.len().saturating_sub(valid_declared_rows),
    })
}

pub(super) fn fdm_index_path_for_text(text_path: &str) -> Option<String> {
    if !text_path
        .get(text_path.len().saturating_sub("/FDMText".len())..)?
        .eq_ignore_ascii_case("/FDMText")
    {
        return None;
    }
    text_path
        .get(..text_path.len().saturating_sub("/FDMText".len()))
        .map(|prefix| format!("{prefix}/FDMIndex"))
}

pub(super) fn fdm_index_declared_entries<'a>(
    index_stream: &[u8],
    entries: &'a [FdmIndexEntry],
) -> &'a [FdmIndexEntry] {
    if !index_stream.starts_with(&[0x03, 0x0b, 0x00, 0x01]) {
        return &[];
    }

    let Some(count) = read_be16_at(index_stream, FDM_INDEX_DECLARED_COUNT_OFFSET).map(usize::from)
    else {
        return &[];
    };
    if count > entries.len() {
        return &[];
    }

    &entries[..count]
}

pub(super) fn parse_fdm_index_entries(
    index_stream: &[u8],
    vector_len: usize,
) -> Vec<FdmIndexEntry> {
    if index_stream.len() < FDM_INDEX_HEADER_BYTES {
        return Vec::new();
    }

    let entry_bytes = index_stream.len() - FDM_INDEX_HEADER_BYTES;
    let entry_count = entry_bytes / FDM_INDEX_ENTRY_BYTES;
    let mut entries = Vec::with_capacity(entry_count);
    for row_index in 0..entry_count {
        let index_offset = FDM_INDEX_HEADER_BYTES + row_index * FDM_INDEX_ENTRY_BYTES;
        let Some(vector_offset) = read_be32_at(index_stream, index_offset) else {
            continue;
        };
        let Some(kind) = read_be16_at(index_stream, index_offset + 4) else {
            continue;
        };
        let Some(left) = read_i32_be_at(index_stream, index_offset + 6) else {
            continue;
        };
        let Some(top) = read_i32_be_at(index_stream, index_offset + 10) else {
            continue;
        };
        let Some(right) = read_i32_be_at(index_stream, index_offset + 14) else {
            continue;
        };
        let Some(bottom) = read_i32_be_at(index_stream, index_offset + 18) else {
            continue;
        };
        let vector_offset = vector_offset as usize;
        entries.push(FdmIndexEntry {
            row_index,
            index_offset,
            vector_offset,
            kind,
            left,
            top,
            right,
            bottom,
            valid_vector_offset: vector_offset < vector_len,
        });
    }
    entries
}

pub(super) fn parse_fdm_text_index_entries(
    index_stream: &[u8],
    text_stream: &[u8],
) -> Vec<FdmTextIndexEntry> {
    let mut entries = Vec::new();
    let mut offset = 0usize;
    while offset + FDM_INDEX_ENTRY_BYTES <= index_stream.len() {
        let Some(text_record_offset) =
            read_be32_at(index_stream, offset).map(|value| value as usize)
        else {
            offset += 1;
            continue;
        };
        let Some(kind) = read_be16_at(index_stream, offset + 4) else {
            offset += 1;
            continue;
        };
        if kind != FDM_TEXT_EXPANDED_INDEX_KIND
            || fdm_text_record_marker_at(text_stream, text_record_offset).is_none()
        {
            offset += 1;
            continue;
        }
        let Some(left) = read_i32_be_at(index_stream, offset + 6) else {
            offset += 1;
            continue;
        };
        let Some(top) = read_i32_be_at(index_stream, offset + 10) else {
            offset += 1;
            continue;
        };
        let Some(right) = read_i32_be_at(index_stream, offset + 14) else {
            offset += 1;
            continue;
        };
        let Some(bottom) = read_i32_be_at(index_stream, offset + 18) else {
            offset += 1;
            continue;
        };
        entries.push(FdmTextIndexEntry {
            row_index: entries.len(),
            index_offset: offset,
            text_record_offset,
            kind,
            bbox: ObjectFdmIndexBbox::new(left, top, right, bottom),
        });
        offset += FDM_INDEX_ENTRY_BYTES;
    }
    entries
}

pub(super) fn fdm_vector_segment(
    vector_offset: usize,
    entries: &[FdmIndexEntry],
    vector_stream: &[u8],
) -> FdmVectorSegment {
    let start = vector_offset.min(vector_stream.len());
    let end = entries
        .iter()
        .filter_map(|entry| {
            (entry.vector_offset > vector_offset && entry.vector_offset <= vector_stream.len())
                .then_some(entry.vector_offset)
        })
        .min()
        .unwrap_or(vector_stream.len());
    FdmVectorSegment { start, end }
}

pub(super) fn fdm_segment_signature_hits(
    vector_hits: &[ObjectImageSignatureHit],
    start: usize,
    end: usize,
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectImageSignatureHit>> {
    let mut hits = Vec::new();
    for hit in vector_hits
        .iter()
        .filter(|hit| hit.offset() >= start && hit.offset() < end)
    {
        reserve_image_signature_candidate(budget, hit.kind())?;
        hits.push(hit.clone());
    }
    Ok(hits)
}

pub(super) fn fdm_relative_signature_hits(
    segment_hits: &[ObjectImageSignatureHit],
    segment_start: usize,
    budget: &mut ResourceBudget,
) -> Result<Vec<ObjectImageSignatureHit>> {
    let mut hits = Vec::new();
    for hit in segment_hits {
        reserve_image_signature_candidate(budget, hit.kind())?;
        hits.push(ObjectImageSignatureHit::new(
            hit.kind(),
            hit.offset().saturating_sub(segment_start),
        ));
    }
    Ok(hits)
}

pub(super) fn fdm_text_candidates_from_stream(
    path: &str,
    stream: &[u8],
) -> Vec<ObjectFdmTextCandidate> {
    if !path.ends_with("/FDMText") {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    let marker_offsets = fdm_text_record_marker_offsets(stream);
    for (index, marker_offset) in marker_offsets.iter().copied().enumerate() {
        let next_record_offset = marker_offsets
            .get(index + 1)
            .copied()
            .unwrap_or(stream.len());
        let Some(marker) = stream.get(marker_offset..marker_offset.saturating_add(4)) else {
            continue;
        };
        let decoded = if marker == FDM_TEXT_RECORD_MARKER {
            fdm_text_candidate_legacy_text(stream, marker_offset, next_record_offset)
        } else if marker == FDM_TEXT_EXPANDED_RECORD_MARKER {
            fdm_text_candidate_expanded_text(stream, marker_offset, next_record_offset)
        } else {
            continue;
        };
        let Some((text, text_offset, raw_text)) = decoded else {
            continue;
        };
        if text.trim().is_empty() {
            continue;
        }

        candidates.push(ObjectFdmTextCandidate::new(
            text,
            text_offset,
            marker_offset,
            raw_text,
            fdm_text_candidate_bbox(stream, marker_offset),
        ));
    }
    candidates
}

pub(super) fn fdm_text_mirror_anchor_agreements(
    candidates: &[ObjectStreamCandidate],
) -> Vec<FdmTextMirrorAnchorAgreement> {
    let mut agreements = Vec::new();
    for indexed in candidates {
        let indexed_texts = indexed.fdm_text_candidates();
        if indexed_texts.is_empty()
            || indexed.fdm_text_index_entry_candidates().len() != indexed_texts.len()
            || indexed_texts.iter().any(|text| text.bbox().is_none())
        {
            continue;
        }

        let indexed_record_offset_agreement = indexed
            .fdm_text_index_entry_candidates()
            .iter()
            .zip(indexed_texts)
            .all(|(entry, text)| {
                entry.valid_text_record_offset()
                    && entry.text_path() == indexed.path()
                    && entry.text_record_offset() == text.marker_offset()
            });
        let indexed_record_bbox_agreement = indexed
            .fdm_text_index_entry_candidates()
            .iter()
            .zip(indexed_texts)
            .all(|(entry, text)| text.bbox().is_some() && entry.text_record_bbox() == text.bbox());
        if !indexed_record_offset_agreement || !indexed_record_bbox_agreement {
            continue;
        }

        for mirrored in candidates {
            if indexed.path() == mirrored.path() {
                continue;
            }
            let mirrored_texts = mirrored.fdm_text_candidates();
            let ordered_text_agreement = indexed_texts.len() == mirrored_texts.len()
                && indexed_texts
                    .iter()
                    .zip(mirrored_texts)
                    .all(|(left, right)| left.text() == right.text());
            let ordered_record_bbox_agreement = indexed_texts.len() == mirrored_texts.len()
                && indexed_texts
                    .iter()
                    .zip(mirrored_texts)
                    .all(|(left, right)| {
                        left.bbox().is_some()
                            && right.bbox().is_some()
                            && left.bbox() == right.bbox()
                    });
            if !ordered_text_agreement || !ordered_record_bbox_agreement {
                continue;
            }

            agreements.push(FdmTextMirrorAnchorAgreement::new(
                indexed.path(),
                mirrored.path(),
                indexed_texts.len(),
                ordered_text_agreement,
                ordered_record_bbox_agreement,
                indexed_record_offset_agreement,
                indexed_record_bbox_agreement,
            ));
        }
    }
    agreements
}

pub(super) fn fdm_text_record_marker_offsets(stream: &[u8]) -> Vec<usize> {
    let mut offsets = find_subslice_offsets(stream, FDM_TEXT_RECORD_MARKER);
    offsets.extend(find_subslice_offsets(
        stream,
        FDM_TEXT_EXPANDED_RECORD_MARKER,
    ));
    offsets.sort_unstable();
    offsets.dedup();
    offsets
}

pub(super) fn figure_link_candidate_from_stream(
    path: &str,
    stream: &[u8],
) -> Option<ObjectFigureLinkCandidate> {
    let lower = path.to_ascii_lowercase();
    if !lower.contains("/figuredata/") || !lower.ends_with("/link") {
        return None;
    }
    if stream.len() < FIGURE_LINK_HEADER_BYTES + FIGURE_LINK_ROW_BYTES {
        return None;
    }
    let row_payload_len = stream.len().checked_sub(FIGURE_LINK_HEADER_BYTES)?;
    if row_payload_len % FIGURE_LINK_ROW_BYTES != 0 {
        return None;
    }

    let declared_row_count_candidate = read_be16_at(stream, 6);
    let row_count = row_payload_len / FIGURE_LINK_ROW_BYTES;
    if row_count == 0 || declared_row_count_candidate.map(usize::from) != Some(row_count) {
        return None;
    }

    let mut rows = Vec::with_capacity(row_count);
    for row_index in 0..row_count {
        let row_start = FIGURE_LINK_HEADER_BYTES + row_index * FIGURE_LINK_ROW_BYTES;
        let row_end = row_start + FIGURE_LINK_ROW_BYTES;
        let row = stream.get(row_start..row_end)?;
        let relation_kind = read_be16_at(row, FIGURE_LINK_RELATION_KIND_CANDIDATE_OFFSET)?;
        if relation_kind != FIGURE_LINK_RELATION_KIND_CANDIDATE {
            return None;
        }
        rows.push(ObjectFigureLinkRowCandidate::new(row_index, row_start, row));
    }

    Some(ObjectFigureLinkCandidate::new(
        read_be16_fields(&stream[..FIGURE_LINK_HEADER_BYTES]),
        declared_row_count_candidate,
        FIGURE_LINK_ROW_BYTES,
        rows,
    ))
}

pub(super) fn fdm_text_candidate_legacy_text(
    stream: &[u8],
    marker_offset: usize,
    next_record_offset: usize,
) -> Option<(String, usize, Vec<u8>)> {
    if next_record_offset <= marker_offset + FDM_TEXT_RECORD_MARKER.len() {
        return None;
    };
    let text_end = next_record_offset.checked_sub(FDM_TEXT_RECORD_TRAILER.len())?;
    if stream.get(text_end..next_record_offset) != Some(FDM_TEXT_RECORD_TRAILER.as_slice()) {
        return None;
    }
    let search_start = marker_offset.max(text_end.saturating_sub(FDM_TEXT_RECORD_BACKSCAN_BYTES));
    let delimiter_offset = (search_start..text_end).rev().find(|offset| {
        stream.get(*offset..offset.saturating_add(FDM_TEXT_RECORD_TEXT_DELIMITER.len()))
            == Some(FDM_TEXT_RECORD_TEXT_DELIMITER.as_slice())
    })?;
    let text_offset = delimiter_offset + FDM_TEXT_RECORD_TEXT_DELIMITER.len();
    let raw_text = stream.get(text_offset..text_end)?;
    let text = decode_fdm_text_bytes(raw_text)?;
    Some((text, text_offset, raw_text.to_vec()))
}

pub(super) fn fdm_text_candidate_expanded_text(
    stream: &[u8],
    marker_offset: usize,
    next_marker_offset: usize,
) -> Option<(String, usize, Vec<u8>)> {
    let declared_len = read_be32_at(
        stream,
        marker_offset + FDM_TEXT_RECORD_DECLARED_LENGTH_OFFSET,
    )
    .map(|value| value as usize)?;
    let declared_end = marker_offset.checked_add(declared_len)?;
    let record_end = if declared_len > FDM_TEXT_EXPANDED_COUNT_OFFSET_FROM_MARKER
        && declared_end <= stream.len()
    {
        declared_end
    } else {
        next_marker_offset
    };
    let count = read_be16_at(
        stream,
        marker_offset + FDM_TEXT_EXPANDED_COUNT_OFFSET_FROM_MARKER,
    )
    .map(usize::from)?;
    let text_len = count.checked_sub(1)?.checked_mul(2)?;
    let mut matches = Vec::new();
    for delimiter_offset in marker_offset..record_end.saturating_sub(FDM_TEXT_RECORD_TRAILER.len())
    {
        if stream.get(
            delimiter_offset..delimiter_offset.saturating_add(FDM_TEXT_RECORD_TEXT_DELIMITER.len()),
        ) != Some(FDM_TEXT_RECORD_TEXT_DELIMITER.as_slice())
        {
            continue;
        }
        let text_offset = delimiter_offset + FDM_TEXT_RECORD_TEXT_DELIMITER.len();
        let Some(text_end) = text_offset.checked_add(text_len) else {
            continue;
        };
        let Some(trailer_end) = text_end.checked_add(FDM_TEXT_RECORD_TRAILER.len()) else {
            continue;
        };
        if trailer_end > record_end {
            continue;
        }
        if stream.get(text_end..trailer_end) == Some(FDM_TEXT_RECORD_TRAILER.as_slice()) {
            matches.push((text_offset, text_end));
        }
    }
    if matches.len() != 1 {
        return None;
    }
    let (text_offset, text_end) = matches[0];
    let raw_text = stream.get(text_offset..text_end)?;
    let text = decode_fdm_text_utf16be(raw_text)?;
    Some((text, text_offset, raw_text.to_vec()))
}

pub(super) fn fdm_text_candidate_bbox(
    stream: &[u8],
    marker_offset: usize,
) -> Option<ObjectFdmIndexBbox> {
    let offset = marker_offset.saturating_add(FDM_TEXT_RECORD_BBOX_OFFSET_FROM_MARKER);
    let left = read_i32_be_at(stream, offset)?;
    let top = read_i32_be_at(stream, offset + 4)?;
    let right = read_i32_be_at(stream, offset + 8)?;
    let bottom = read_i32_be_at(stream, offset + 12)?;
    (left != right && top != bottom).then_some(ObjectFdmIndexBbox::new(left, top, right, bottom))
}

pub(super) fn decode_fdm_text_bytes(bytes: &[u8]) -> Option<String> {
    let mut output = String::new();
    let mut offset = 0usize;
    while offset < bytes.len() {
        if bytes[offset] == 0 {
            let unit = *bytes.get(offset + 1)?;
            if unit >= 0x20 || unit == b'\r' {
                if unit != b'\r' {
                    output.push(char::from(unit));
                }
                offset += 2;
                continue;
            }
        }
        let first = bytes[offset];
        let second = *bytes.get(offset + 1)?;
        output.push(decode_fdm_text_shift_jis_pair(first, second)?);
        offset += 2;
    }
    Some(output)
}

pub(super) fn decode_fdm_text_utf16be(bytes: &[u8]) -> Option<String> {
    let mut output = String::new();
    let mut chunks = bytes.chunks_exact(2);
    for chunk in &mut chunks {
        let code_unit = u16::from_be_bytes([chunk[0], chunk[1]]);
        if code_unit == u16::from(b'\r') {
            continue;
        }
        output.push(char::from_u32(u32::from(code_unit))?);
    }
    chunks.remainder().is_empty().then_some(output)
}

pub(super) fn decode_fdm_text_shift_jis_pair(first: u8, second: u8) -> Option<char> {
    match (first, second) {
        (0x81, 0x8b) => Some('°'),
        (0x82, 0x60..=0x62) => char::from_u32('Ａ' as u32 + u32::from(second - 0x60)),
        (0x82, 0x4f..=0x58) => char::from_u32('０' as u32 + u32::from(second - 0x4f)),
        (0x82, 0x6e) => Some('Ｏ'),
        (0x82, 0x98) => Some('ｘ'),
        (0x87, 0x70) => Some('㎝'),
        _ => None,
    }
}

pub(super) fn project_fdm_single_page_diagram(
    document: &Document,
    pages: &mut Vec<Vec<PageTextLine>>,
) -> bool {
    if !document_has_shanai_lan_fdm_command_evidence(document) {
        return false;
    }

    if pages.is_empty() {
        pages.push(Vec::new());
    } else {
        pages.truncate(1);
    }
    true
}

pub(super) fn fdm_text_mirror_anchor_agreements_json(
    agreements: &[FdmTextMirrorAnchorAgreement],
) -> String {
    let mut output = String::from("[");
    for (index, agreement) in agreements.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_text_mirror_anchor_agreement_json(&mut output, agreement);
    }
    output.push(']');
    output
}

pub(super) fn push_fdm_text_mirror_anchor_agreement_json(
    output: &mut String,
    agreement: &FdmTextMirrorAnchorAgreement,
) {
    output.push_str(
        "{\"source\":\"FDMText mirrored record sequence+FDMTextIndex row-to-record links\"",
    );
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false");
    output.push_str(",\"diagnosticOnly\":true,\"renderable\":false,\"placementProven\":false");
    output.push_str(",\"indexedTextPath\":");
    output.push_str(&json_string(agreement.indexed_text_path()));
    output.push_str(",\"mirroredTextPath\":");
    output.push_str(&json_string(agreement.mirrored_text_path()));
    output.push_str(",\"textRecordCount\":");
    output.push_str(&agreement.text_record_count().to_string());
    output.push_str(",\"orderedTextAgreement\":");
    output.push_str(&agreement.ordered_text_agreement().to_string());
    output.push_str(",\"orderedRecordBboxAgreement\":");
    output.push_str(&agreement.ordered_record_bbox_agreement().to_string());
    output.push_str(",\"indexedRecordOffsetAgreement\":");
    output.push_str(&agreement.indexed_record_offset_agreement().to_string());
    output.push_str(",\"indexedRecordBboxAgreement\":");
    output.push_str(&agreement.indexed_record_bbox_agreement().to_string());
    output.push_str(",\"sourceAnchorTraceReady\":");
    output.push_str(&agreement.source_anchor_trace_ready().to_string());
    output.push_str(
        ",\"sourceToPageTransformDecoded\":false,\"roleDecoded\":false,\"paintOrderDecoded\":false",
    );
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"fdmtext-source-to-page-transform-undecoded\"}",
    );
}

pub(super) fn push_fdm_index_segment_bbox_axis_pair_gate_json(
    output: &mut String,
    gate: FdmIndexSegmentBboxAxisPairGate,
) {
    output.push_str("{\"source\":\"FDMIndex raw bbox fields+FDMVector segment header bbox\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false");
    output.push_str(",\"diagnosticOnly\":true,\"renderable\":false,\"placementProven\":false");
    output.push_str(",\"validIndexRowCount\":");
    output.push_str(&gate.valid_index_row_count().to_string());
    output.push_str(",\"linkedRowCount\":");
    output.push_str(&gate.linked_row_count().to_string());
    output.push_str(",\"axisPairOrderAgreementRowCount\":");
    output.push_str(&gate.axis_pair_order_agreement_row_count().to_string());
    output.push_str(",\"axisPairOrderAgreementComplete\":");
    output.push_str(&gate.axis_pair_order_agreement_complete().to_string());
    output.push_str(",\"normalizationInputSourceBacked\":");
    output.push_str(&gate.axis_pair_order_agreement_complete().to_string());
    output.push_str(
        ",\"fieldOrderDecoded\":false,\"pageTransformDecoded\":false,\"objectRoleDecoded\":false",
    );
    let blocked_reason = if gate.axis_pair_order_agreement_complete() {
        "fdm-index-axis-pair-does-not-decode-page-transform-or-object-role"
    } else {
        "fdm-index-axis-pair-order-incomplete"
    };
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(blocked_reason));
    output.push('}');
}

pub(super) fn push_object_fdm_text_candidate_json(
    output: &mut String,
    candidate: &ObjectFdmTextCandidate,
) {
    output.push_str("{\"text\":");
    output.push_str(&json_string(candidate.text()));
    output.push_str(",\"textOffset\":");
    output.push_str(&candidate.text_offset().to_string());
    output.push_str(",\"markerOffset\":");
    output.push_str(&candidate.marker_offset().to_string());
    output.push_str(",\"rawTextHex\":");
    output.push_str(&json_string(&hex_bytes(candidate.raw_text())));
    output.push_str(",\"bbox\":");
    if let Some(bbox) = candidate.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_fdm_text_index_entry_candidate_json(
    output: &mut String,
    candidate: &ObjectFdmTextIndexEntryCandidate,
) {
    output.push_str("{\"indexPath\":");
    output.push_str(&json_string(candidate.index_path()));
    output.push_str(",\"textPath\":");
    output.push_str(&json_string(candidate.text_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&candidate.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&candidate.index_offset().to_string());
    output.push_str(",\"textRecordOffset\":");
    output.push_str(&candidate.text_record_offset().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&candidate.kind().to_string());
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", candidate.kind())));
    output.push_str(",\"validTextRecordOffset\":");
    output.push_str(if candidate.valid_text_record_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"bbox\":");
    push_object_fdm_index_bbox_json(output, candidate.bbox());
    output.push_str(",\"textRecordBbox\":");
    if let Some(bbox) = candidate.text_record_bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"textRecordPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(candidate.text_record_prefix())));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_object_fdm_index_entry_candidate_json(
    output: &mut String,
    entry: &ObjectFdmIndexEntryCandidate,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) {
    output.push_str("{\"indexPath\":");
    output.push_str(&json_string(entry.index_path()));
    output.push_str(",\"vectorPath\":");
    output.push_str(&json_string(entry.vector_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&entry.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&entry.index_offset().to_string());
    output.push_str(",\"vectorOffset\":");
    output.push_str(&entry.vector_offset().to_string());
    output.push_str(",\"nextVectorOffset\":");
    output.push_str(&entry.next_vector_offset().to_string());
    output.push_str(",\"vectorLength\":");
    output.push_str(&entry.vector_len().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&entry.kind().to_string());
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", entry.kind())));
    output.push_str(",\"bbox\":");
    push_object_fdm_index_bbox_json(output, entry.bbox());
    output.push_str(",\"validVectorOffset\":");
    output.push_str(if entry.valid_vector_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"offsetFieldReferenceCandidates\":");
    push_object_fdm_index_offset_field_reference_candidates_json(output, entry, raw_commands);
    output.push_str(",\"vectorPrefixHex\":");
    output.push_str(&json_string(&hex_bytes(entry.vector_prefix())));
    output.push_str(",\"vectorCommandCount\":");
    output.push_str(&entry.vector_commands().len().to_string());
    output.push_str(",\"vectorCommandBboxCount\":");
    output.push_str(
        &entry
            .vector_commands()
            .iter()
            .filter(|command| command.bbox().is_some())
            .count()
            .to_string(),
    );
    output.push_str(",\"vectorCommands\":[");
    for (index, command) in entry.vector_commands().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_vector_command_candidate_json(output, command);
    }
    output.push(']');
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&entry.connector_candidates().len().to_string());
    output.push_str(",\"connectorCandidates\":[");
    for (index, candidate) in entry.connector_candidates().iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_object_fdm_connector_candidate_json(output, candidate);
    }
    output.push(']');
    output.push_str(",\"imageSignatures\":[");
    for (index, hit) in entry.image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"segmentImageSignatures\":[");
    for (index, hit) in entry.segment_image_signature_hits().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push('}');
    }
    output.push_str("],\"decoded\":false}");
}

pub(super) fn push_object_fdm_index_offset_field_reference_candidates_json(
    output: &mut String,
    entry: &ObjectFdmIndexEntryCandidate,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) {
    let bbox = entry.bbox();
    let fields = [
        Some(("vectorOffset", entry.vector_offset())),
        non_negative_i32_offset("bbox.left", bbox.left()),
        non_negative_i32_offset("bbox.top", bbox.top()),
        non_negative_i32_offset("bbox.right", bbox.right()),
        non_negative_i32_offset("bbox.bottom", bbox.bottom()),
    ];
    output.push('[');
    let mut emitted = 0usize;
    for field in fields.into_iter().flatten() {
        emitted += push_object_fdm_index_offset_field_reference_candidate_json(
            output,
            emitted,
            field.0,
            field.1,
            raw_commands,
        );
    }
    output.push(']');
}

pub(super) fn push_object_fdm_index_offset_field_reference_candidate_json(
    output: &mut String,
    emitted: usize,
    field_name: &str,
    field_value: usize,
    raw_commands: &[ObjectFdmVectorCommandCandidate],
) -> usize {
    let command_matches = raw_commands
        .iter()
        .filter(|command| command.relative_offset() == field_value)
        .map(ObjectFdmVectorCommandCandidate::relative_offset)
        .collect::<Vec<_>>();
    let segment_matches = raw_commands
        .iter()
        .filter(|command| {
            command
                .source_segment()
                .is_some_and(|segment| segment.relative_offset() == field_value)
        })
        .map(ObjectFdmVectorCommandCandidate::relative_offset)
        .collect::<Vec<_>>();

    let mut local_emitted = 0usize;
    if !command_matches.is_empty() {
        if emitted + local_emitted > 0 {
            output.push(',');
        }
        output.push_str("{\"offsetField\":");
        output.push_str(&json_string(field_name));
        output.push_str(",\"offsetValue\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"matchKind\":\"command-relative-offset-field\"");
        output.push_str(",\"referenceSource\":\"fdmRawVectorCommands.relativeOffset\"");
        output.push_str(",\"matchedCommandRelativeOffsets\":");
        push_usize_array_json(output, &command_matches);
        output.push_str(",\"decoded\":false}");
        local_emitted += 1;
    }
    if !segment_matches.is_empty() {
        if emitted + local_emitted > 0 {
            output.push(',');
        }
        output.push_str("{\"offsetField\":");
        output.push_str(&json_string(field_name));
        output.push_str(",\"offsetValue\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"matchKind\":\"source-segment-relative-offset-field\"");
        output
            .push_str(",\"referenceSource\":\"fdmRawVectorCommands.sourceSegment.relativeOffset\"");
        output.push_str(",\"sourceSegmentRelativeOffset\":");
        output.push_str(&field_value.to_string());
        output.push_str(",\"sourceSegmentBackedCommandCount\":");
        output.push_str(&segment_matches.len().to_string());
        output.push_str(",\"matchedCommandRelativeOffsets\":");
        push_usize_array_json(output, &segment_matches);
        output.push_str(",\"decoded\":false}");
        local_emitted += 1;
    }
    local_emitted
}

pub(super) fn push_object_fdm_connector_candidate_json(
    output: &mut String,
    candidate: ObjectFdmConnectorCandidate,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&candidate.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&candidate.relative_offset().to_string());
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(&candidate.marker())));
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(candidate.primitive_kind()));
    output.push_str(",\"styleWord\":");
    output.push_str(&candidate.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", candidate.style_word())));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, candidate.fill_color());
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, candidate.stroke_color());
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(candidate.basis()));
    output.push_str(",\"sourceEndpoints\":");
    push_fdm_connector_candidate_source_endpoints_json(output, candidate);
    output.push_str(",\"sourceBbox\":");
    push_object_fdm_index_bbox_json(output, candidate.source_bbox());
    output.push_str(",\"sourceSpan\":");
    output.push_str(&candidate.source_span().to_string());
    output.push_str(",\"endpointDelta\":{\"x\":");
    output.push_str(&candidate.endpoint_dx().to_string());
    output.push_str(",\"y\":");
    output.push_str(&candidate.endpoint_dy().to_string());
    output.push('}');
    output.push_str(",\"endpointDistanceSquared\":");
    output.push_str(&candidate.endpoint_distance_squared().to_string());
    output.push_str(",\"pathPointCount\":");
    output.push_str(&candidate.path_point_count().to_string());
    output.push_str(",\"pathSegmentCount\":");
    output.push_str(&candidate.path_segment_count().to_string());
    output.push_str(",\"orthogonalSegmentCount\":");
    output.push_str(&candidate.orthogonal_segment_count().to_string());
    output.push_str(",\"diagonalSegmentCount\":");
    output.push_str(&candidate.diagonal_segment_count().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&candidate.curve_segment_count().to_string());
    output.push_str(",\"compoundChildOffsetCount\":");
    output.push_str(&candidate.compound_child_offset_count().to_string());
    output.push_str(",\"axisAligned\":");
    output.push_str(if candidate.axis_aligned() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"orientation\":");
    output.push_str(&json_string(candidate.orientation()));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_fdm_connector_candidate_source_endpoints_json(
    output: &mut String,
    candidate: ObjectFdmConnectorCandidate,
) {
    output.push_str("{\"start\":");
    push_fdm_vector_point_json(output, candidate.source_start());
    output.push_str(",\"end\":");
    push_fdm_vector_point_json(output, candidate.source_end());
    output.push('}');
}

pub(super) fn push_fdm_vector_point_json(output: &mut String, point: ObjectFdmVectorPoint) {
    output.push_str("{\"x\":");
    output.push_str(&point.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&point.y().to_string());
    output.push('}');
}

pub(super) fn push_object_fdm_vector_command_candidate_json(
    output: &mut String,
    command: &ObjectFdmVectorCommandCandidate,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&command.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&command.relative_offset().to_string());
    output.push_str(",\"sourceVectorRelativeOffset\":");
    push_optional_usize_json(output, command.source_vector_relative_offset());
    output.push_str(",\"sourceSegment\":");
    if let Some(source_segment) = command.source_segment() {
        push_object_fdm_vector_command_source_segment_json(output, source_segment);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"recordLength\":");
    output.push_str(&command.record_len().to_string());
    output.push_str(",\"declaredRecordLength\":");
    output.push_str(&command.declared_record_len().to_string());
    output.push_str(",\"styleWord\":");
    output.push_str(&command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", command.style_word())));
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(command.marker())));
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(command)));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, command.fill_color());
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, command.stroke_color());
    output.push_str(",\"bbox\":");
    if let Some(bbox) = command.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pathPointCount\":");
    output.push_str(&command.path_points().len().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&command.curve_segments().len().to_string());
    output.push_str(",\"ellipse\":");
    if let Some(ellipse) = command.ellipse() {
        push_fdm_vector_ellipse_json(output, ellipse);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(command) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"compoundChildLayoutGate\":");
    push_fdm_compound_child_layout_gate_json(output, command);
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_fdm_compound_child_layout_gate_json(
    output: &mut String,
    command: &ObjectFdmVectorCommandCandidate,
) {
    let Some(layout) = command.compound_child_layout() else {
        output.push_str("null");
        return;
    };
    output.push_str(
        "{\"source\":\"FDMVector compound prefix child-offset table+child declared lengths\"",
    );
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"childOffsets\":");
    push_u16_array_json(output, layout.child_offsets());
    output.push_str(",\"firstChildMatchesPrefixEnd\":");
    output.push_str(&layout.first_child_matches_prefix_end().to_string());
    output.push_str(",\"childOffsetsStrictlyIncreasing\":");
    output.push_str(&layout.child_offsets_strictly_increasing().to_string());
    output.push_str(",\"childRecordsFitParent\":");
    output.push_str(&layout.child_records_fit_parent().to_string());
    output.push_str(",\"childRecordsDoNotOverlap\":");
    output.push_str(&layout.child_records_do_not_overlap().to_string());
    output.push_str(",\"nestedProjectionInputValid\":");
    output.push_str(&layout.is_valid_for_nested_projection().to_string());
    output.push_str(",\"renderPromotionBlockedReason\":\"compound-child-boundaries-do-not-prove-connector-ownership-or-paint-order\"}");
}

pub(super) fn push_object_fdm_vector_command_source_segment_json(
    output: &mut String,
    source_segment: ObjectFdmVectorCommandSourceSegment,
) {
    output.push_str("{\"relativeOffset\":");
    output.push_str(&source_segment.relative_offset().to_string());
    output.push_str(",\"localOffset\":");
    output.push_str(&source_segment.local_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&source_segment.declared_len().to_string());
    output.push_str(",\"commandCount\":");
    output.push_str(&source_segment.command_count().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&source_segment.command_index().to_string());
    output.push_str(",\"commandOffset\":");
    output.push_str(&source_segment.command_offset().to_string());
    output.push('}');
}

pub(super) fn push_object_fdm_vector_segment_candidate_json(
    output: &mut String,
    segment: &ObjectFdmVectorSegmentCandidate,
) {
    output.push_str("{\"relativeOffset\":");
    output.push_str(&segment.relative_offset().to_string());
    output.push_str(",\"declaredLength\":");
    output.push_str(&segment.declared_len().to_string());
    output.push_str(",\"commandCount\":");
    output.push_str(&segment.command_count().to_string());
    output.push_str(",\"commandOffsets\":");
    push_u16_array_json(output, segment.command_offsets());
    output.push_str(",\"bbox\":");
    if let Some(bbox) = segment.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceSpanCandidate\":{\"width\":");
    output.push_str(&segment.source_width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&segment.source_height().to_string());
    output.push_str("},\"decoded\":false}");
}

pub(super) fn push_fdm_vector_ellipse_json(output: &mut String, ellipse: ObjectFdmVectorEllipse) {
    let center = ellipse.center();
    output.push_str("{\"center\":{\"x\":");
    output.push_str(&center.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&center.y().to_string());
    output.push_str("},\"radiusX\":");
    output.push_str(&ellipse.radius_x().to_string());
    output.push_str(",\"radiusY\":");
    output.push_str(&ellipse.radius_y().to_string());
    output.push_str(",\"color\":");
    if let Some(color) = ellipse.color().and_then(fdm_vector_primitive_css_color) {
        output.push_str(&json_string(&color));
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn push_fdm_vector_optional_color_json(output: &mut String, color: Option<u32>) {
    if let Some(color) = color.and_then(fdm_vector_css_color) {
        output.push_str(&json_string(&color));
    } else {
        output.push_str("null");
    }
}

pub(super) fn push_object_fdm_index_bbox_json(output: &mut String, bbox: ObjectFdmIndexBbox) {
    output.push_str("{\"left\":");
    output.push_str(&bbox.left().to_string());
    output.push_str(",\"top\":");
    output.push_str(&bbox.top().to_string());
    output.push_str(",\"right\":");
    output.push_str(&bbox.right().to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&bbox.bottom().to_string());
    output.push('}');
}

pub(super) fn fdm_vector_css_color(color: u32) -> Option<String> {
    if color > 0x00ff_ffff {
        return None;
    }
    let blue = (color >> 16) & 0xff;
    let green = (color >> 8) & 0xff;
    let red = color & 0xff;
    Some(format!("#{red:02x}{green:02x}{blue:02x}"))
}

pub(super) fn fdm_vector_primitive_css_color(color: u32) -> Option<String> {
    if color <= 0x00ff_ffff {
        return fdm_vector_css_color(color);
    }
    if color & 0xff00_0000 == 0xff00_0000 {
        return fdm_vector_css_color(color & 0x00ff_ffff);
    }
    None
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FdmFrameDiagnostic<'a> {
    pub(super) candidate_index: usize,
    pub(super) candidate: &'a ObjectStreamCandidate,
    pub(super) entry: &'a ObjectFdmIndexEntryCandidate,
    pub(super) frame_record: &'a ObjectFrameRecordCandidate,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FdmCommandDiagnostic<'a> {
    pub(super) candidate_index: usize,
    pub(super) candidate: &'a ObjectStreamCandidate,
    pub(super) entry: &'a ObjectFdmIndexEntryCandidate,
    pub(super) command: &'a ObjectFdmVectorCommandCandidate,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FdmConnectorParentCompoundProvenance<'a> {
    pub(super) parent: &'a ObjectFdmVectorCommandCandidate,
    pub(super) child_offset_in_parent: usize,
    pub(super) child_offset_table_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmCommandProjectionExtent {
    pub(super) left: i32,
    pub(super) top: i32,
    pub(super) right: i32,
    pub(super) bottom: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct FdmConnectorCandidateMetric {
    pub(super) source_start: ObjectFdmVectorPoint,
    pub(super) source_end: ObjectFdmVectorPoint,
    pub(super) projected_start: (f32, f32),
    pub(super) projected_end: (f32, f32),
    pub(super) projected_bbox: (f32, f32, f32, f32),
    pub(super) source_endpoint_distance: f32,
    pub(super) projected_endpoint_distance: f32,
    pub(super) projected_span: f32,
    pub(super) orientation: &'static str,
    pub(super) basis: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FdmConnectorOrderTraceNodeJson {
    pub(super) parent_relative_offset: Option<usize>,
    pub(super) relative_offset: Option<usize>,
    pub(super) rank: usize,
    pub(super) json: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmConnectorOrderTraceSummary {
    pub(super) trace_count: usize,
    pub(super) source_segment_matches_index_entry_count: usize,
    pub(super) entry_connector_candidate_count: usize,
    pub(super) image_bearing_segment_count: usize,
    pub(super) image_bearing_complete_payload_segment_count: usize,
    pub(super) image_bearing_signature_without_payload_segment_count: usize,
    pub(super) parent_normalized_ordered_same_row_same_connector_count: usize,
    pub(super) bbox_contained_count: usize,
    pub(super) bbox_overlaps_count: usize,
    pub(super) bbox_disjoint_count: usize,
    pub(super) bbox_missing_count: usize,
    pub(super) image_bearing_bbox_contained_count: usize,
    pub(super) image_bearing_bbox_overlaps_count: usize,
    pub(super) image_bearing_bbox_disjoint_count: usize,
    pub(super) image_bearing_bbox_missing_count: usize,
    pub(super) connector_before_axis_rule_parent_span_count: usize,
    pub(super) connector_between_axis_rule_parent_span_count: usize,
    pub(super) connector_after_axis_rule_parent_span_count: usize,
    pub(super) connector_axis_rule_parent_span_missing_count: usize,
    pub(super) image_bearing_connector_before_axis_rule_parent_span_count: usize,
    pub(super) image_bearing_connector_between_axis_rule_parent_span_count: usize,
    pub(super) image_bearing_connector_after_axis_rule_parent_span_count: usize,
    pub(super) image_bearing_connector_axis_rule_parent_span_missing_count: usize,
    pub(super) image_bearing_connector_before_segment_signature_range_count: usize,
    pub(super) image_bearing_connector_inside_segment_signature_range_count: usize,
    pub(super) image_bearing_connector_after_segment_signature_range_count: usize,
    pub(super) image_bearing_connector_segment_signature_range_missing_count: usize,
    pub(super) owner_parent_span_before_axis_rule_parent_span_count: usize,
    pub(super) owner_parent_span_after_axis_rule_parent_span_count: usize,
    pub(super) owner_parent_span_inside_axis_rule_parent_span_count: usize,
    pub(super) axis_rule_parent_span_inside_owner_parent_span_count: usize,
    pub(super) owner_parent_span_overlaps_axis_rule_parent_span_count: usize,
    pub(super) owner_axis_rule_parent_span_missing_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct FdmConnectorTextGridPoint {
    pub(super) x_units: f32,
    pub(super) group_index_float: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct FdmConnectorLineRuleDistance {
    pub(super) axis_delta: f32,
    pub(super) inline_delta: f32,
    pub(super) distance_grid: f32,
    pub(super) closest_x_units: f32,
    pub(super) closest_group_index: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmConnectorLineRuleEndpointMatchSummary {
    pub(super) start_match_count: usize,
    pub(super) end_match_count: usize,
    pub(super) total_match_count: usize,
    pub(super) tight_match_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail {
    pub(super) summary: FdmConnectorLineRuleEndpointMatchSummary,
    pub(super) start_tight_match_count: usize,
    pub(super) end_tight_match_count: usize,
    pub(super) axis_rule_endpoint_match_marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
    pub(super) axis_rule_match_parent_relative_offset_min: Option<usize>,
    pub(super) axis_rule_match_parent_relative_offset_max: Option<usize>,
}

impl FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail {
    pub(super) fn tight_dual_endpoint_match(self) -> bool {
        self.start_tight_match_count > 0 && self.end_tight_match_count > 0
    }
}

impl FdmConnectorOrderTraceSummary {
    pub(super) fn readiness_blocked_reason(self) -> &'static str {
        if self.trace_count == 0 {
            "no-tight-non-diagonal-dual-endpoint-axis-rule-connectors"
        } else if self.source_segment_matches_index_entry_count < self.trace_count {
            "connector-source-segment-membership-incomplete"
        } else if self.entry_connector_candidate_count < self.trace_count {
            "fdm-index-entry-connector-membership-incomplete"
        } else if self.image_bearing_segment_count > 0 {
            if self.image_bearing_complete_payload_segment_count == 0 {
                "image-signature-fragment-role-unproven"
            } else {
                "image-bearing-segment-paint-order-unproven"
            }
        } else if self.parent_normalized_ordered_same_row_same_connector_count == 0 {
            "no-parent-normalized-ordered-same-row-same-connector"
        } else if self.bbox_missing_count > 0 {
            "connector-fdm-index-bbox-relation-missing"
        } else if self.connector_axis_rule_parent_span_missing_count > 0 {
            "connector-axis-rule-parent-span-relation-missing"
        } else if self.owner_axis_rule_parent_span_missing_count > 0 {
            "owner-axis-rule-parent-span-relation-missing"
        } else {
            "connector-ownership-and-paint-order-unproven"
        }
    }

    pub(super) fn promotion_ready(self) -> bool {
        self.readiness_blocked_reason() == "connector-ownership-and-paint-order-unproven"
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmConnectorEndpointOwnerMatchSummary {
    pub(super) start_candidate_count: usize,
    pub(super) end_candidate_count: usize,
    pub(super) total_candidate_count: usize,
    pub(super) start_within_probe_count: usize,
    pub(super) end_within_probe_count: usize,
    pub(super) within_probe_candidate_count: usize,
    pub(super) fdm_primitive_candidate_count: usize,
    pub(super) document_text_slot_candidate_count: usize,
    pub(super) connector_command_index: usize,
    pub(super) connector_parent_command_index: usize,
    pub(super) connector_synthetic_nested_command: bool,
    pub(super) connector_relative_offset: usize,
    pub(super) connector_parent_relative_offset: Option<usize>,
    pub(super) start_nearest_fdm_owner_row_index: Option<usize>,
    pub(super) start_nearest_fdm_owner_command_index: Option<usize>,
    pub(super) start_nearest_fdm_owner_parent_command_index: Option<usize>,
    pub(super) start_nearest_fdm_owner_synthetic_nested_command: bool,
    pub(super) start_nearest_fdm_owner_relative_offset: Option<usize>,
    pub(super) start_nearest_fdm_owner_parent_relative_offset: Option<usize>,
    pub(super) end_nearest_fdm_owner_row_index: Option<usize>,
    pub(super) end_nearest_fdm_owner_command_index: Option<usize>,
    pub(super) end_nearest_fdm_owner_parent_command_index: Option<usize>,
    pub(super) end_nearest_fdm_owner_synthetic_nested_command: bool,
    pub(super) end_nearest_fdm_owner_relative_offset: Option<usize>,
    pub(super) end_nearest_fdm_owner_parent_relative_offset: Option<usize>,
    pub(super) nearest_fdm_owner_rows_match: bool,
    pub(super) nearest_fdm_owner_row_matches_connector_row: bool,
    pub(super) mixed_top_level_vs_nested_order_namespace: bool,
    pub(super) connector_command_between_nearest_fdm_owner_commands: bool,
    pub(super) connector_command_before_nearest_fdm_owner_commands: bool,
    pub(super) connector_command_after_nearest_fdm_owner_commands: bool,
    pub(super) connector_relative_offset_between_nearest_fdm_owner_offsets: bool,
    pub(super) connector_relative_offset_before_nearest_fdm_owner_offsets: bool,
    pub(super) connector_relative_offset_after_nearest_fdm_owner_offsets: bool,
    pub(super) connector_parent_command_between_nearest_fdm_owner_parent_commands: bool,
    pub(super) connector_parent_command_before_nearest_fdm_owner_parent_commands: bool,
    pub(super) connector_parent_command_after_nearest_fdm_owner_parent_commands: bool,
    pub(super) connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets: bool,
    pub(super) connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets: bool,
    pub(super) connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmConnectorGraphDiagnosticSummary {
    pub(super) page_paint_coverage_summary: FdmPagePaintCoverageSummary,
    pub(super) connector_candidate_count: usize,
    pub(super) line_rule_projection_count: usize,
    pub(super) fdm_open_stroke_axis_rule_projection_count: usize,
    pub(super) connector_endpoint_probe_count: usize,
    pub(super) total_thresholded_endpoint_match_count: usize,
    pub(super) matched_connector_count: usize,
    pub(super) dual_endpoint_match_connector_count: usize,
    pub(super) start_endpoint_line_rule_match_connector_count: usize,
    pub(super) end_endpoint_line_rule_match_connector_count: usize,
    pub(super) start_only_line_rule_match_connector_count: usize,
    pub(super) end_only_line_rule_match_connector_count: usize,
    pub(super) tight_endpoint_match_count: usize,
    pub(super) nearby_endpoint_match_count: usize,
    pub(super) no_thresholded_line_rule_endpoint_match_connector_count: usize,
    pub(super) single_or_missing_endpoint_line_rule_match_connector_count: usize,
    pub(super) connector_ownership_and_paint_order_unproven_connector_count: usize,
    pub(super) endpoint_owner_candidate_connector_count: usize,
    pub(super) endpoint_owner_probe_count: usize,
    pub(super) total_endpoint_owner_candidate_count: usize,
    pub(super) within_probe_endpoint_owner_candidate_count: usize,
    pub(super) fdm_primitive_endpoint_owner_candidate_count: usize,
    pub(super) document_text_slot_endpoint_owner_candidate_count: usize,
    pub(super) start_endpoint_owner_within_probe_connector_count: usize,
    pub(super) end_endpoint_owner_within_probe_connector_count: usize,
    pub(super) dual_endpoint_owner_within_probe_connector_count: usize,
    pub(super) owner_proven_connector_count: usize,
    pub(super) dual_endpoint_nearest_fdm_owner_same_row_connector_count: usize,
    pub(super) dual_endpoint_nearest_fdm_owner_row_mismatch_connector_count: usize,
    pub(super) dual_endpoint_nearest_fdm_owner_same_connector_row_count: usize,
    pub(super) connector_command_between_nearest_fdm_owner_commands_count: usize,
    pub(super) connector_command_before_nearest_fdm_owner_commands_count: usize,
    pub(super) connector_command_after_nearest_fdm_owner_commands_count: usize,
    pub(super) ordered_same_row_same_connector_count: usize,
    pub(super) parent_normalized_ordered_same_row_same_connector_count: usize,
    pub(super) missing_endpoint_owner_candidate_connector_count: usize,
    pub(super) nearest_owner_row_mismatch_connector_count: usize,
    pub(super) owner_row_candidate_unproven_connector_count: usize,
    pub(super) owner_grouping_proven_connector_count: usize,
    pub(super) skipped_inline_line_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary,
    pub(super) vertical_anchor_line_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary,
    pub(super) fdm_open_stroke_axis_rule_match_summary: FdmConnectorRuleSetMatchDiagnosticSummary,
    pub(super) ordered_owner_row_match_summary: FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) non_ordered_owner_row_match_summary: FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) parent_normalized_ordered_owner_row_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) parent_normalized_non_ordered_owner_row_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) missing_endpoint_owner_relation_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) nearest_owner_row_mismatch_relation_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) nearest_owner_row_not_connector_row_relation_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_mixed_command_namespace_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_before_owner_command_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_between_owner_command_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_after_owner_command_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_owner_command_relation_unclassified_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) missing_endpoint_owner_source_order_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) nearest_owner_row_mismatch_source_order_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) nearest_owner_row_not_connector_row_source_order_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_before_owner_relative_offset_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_between_owner_relative_offset_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_after_owner_relative_offset_span_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) same_row_relative_offset_relation_unclassified_match_summary:
        FdmConnectorOwnerRowCohortDiagnosticSummary,
    pub(super) fdm_open_stroke_axis_rule_row_cohort_count: usize,
    pub(super) fdm_open_stroke_axis_rule_row_cohorts:
        [FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary;
            FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT],
    pub(super) fdm_open_stroke_axis_rule_owner_promotion_gate_summary:
        FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary,
    pub(super) same_row_axis_rule_connector_order_trace_summary: FdmConnectorOrderTraceSummary,
    pub(super) dominant_matched_connector_row_index: Option<usize>,
    pub(super) dominant_matched_connector_row_connector_candidate_count: usize,
    pub(super) dominant_matched_connector_row_total_thresholded_endpoint_match_count: usize,
    pub(super) dominant_matched_connector_row_matched_connector_count: usize,
    pub(super) dominant_matched_connector_row_dual_endpoint_match_connector_count: usize,
    pub(super) dominant_matched_connector_row_start_only_match_connector_count: usize,
    pub(super) dominant_matched_connector_row_end_only_match_connector_count: usize,
    pub(super) dominant_matched_connector_row_tight_endpoint_match_count: usize,
    pub(super) dominant_matched_connector_row_nearby_endpoint_match_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmPagePaintCoverageSummary {
    pub(super) inspected_primitive_count: usize,
    pub(super) rendered_primitive_count: usize,
    pub(super) large_span_filtered_primitive_count: usize,
    pub(super) closed_fill_primitive_count: usize,
    pub(super) page_fill_candidate_count: usize,
    pub(super) max_page_coverage_ratio_ppm: u32,
    pub(super) max_viewport_coverage_ratio_ppm: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct FdmTextMaskCohortDiagnosticSummary {
    pub(super) row_index: usize,
    pub(super) primitive_count: usize,
    pub(super) black_fill_primitive_count: usize,
    pub(super) white_fill_primitive_count: usize,
    pub(super) counter_overlay_count: usize,
    pub(super) command_index_min: Option<usize>,
    pub(super) command_index_max: Option<usize>,
    pub(super) relative_offset_min: Option<usize>,
    pub(super) relative_offset_max: Option<usize>,
    pub(super) source_bbox: Option<(i32, i32, i32, i32)>,
    pub(super) projected_bbox: Option<(f32, f32, f32, f32)>,
    pub(super) component_count: usize,
    pub(super) top_text_like_component: Option<FdmTextMaskComponentDiagnosticSummary>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct FdmTextMaskComponentDiagnosticSummary {
    pub(super) component_index: usize,
    pub(super) primitive_count: usize,
    pub(super) black_fill_primitive_count: usize,
    pub(super) white_fill_primitive_count: usize,
    pub(super) counter_overlay_count: usize,
    pub(super) command_index_min: Option<usize>,
    pub(super) command_index_max: Option<usize>,
    pub(super) relative_offset_min: Option<usize>,
    pub(super) relative_offset_max: Option<usize>,
    pub(super) source_bbox: Option<(i32, i32, i32, i32)>,
    pub(super) projected_bbox: Option<(f32, f32, f32, f32)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct FdmTextMaskPrimitiveDiagnosticSummary {
    pub(super) command_index: usize,
    pub(super) relative_offset: usize,
    pub(super) source_bbox: Option<(i32, i32, i32, i32)>,
    pub(super) projected_bbox: (f32, f32, f32, f32),
    pub(super) black_fill: bool,
    pub(super) white_fill: bool,
    pub(super) counter_overlay: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmConnectorRuleSetMatchDiagnosticSummary {
    pub(super) line_rule_projection_count: usize,
    pub(super) connector_candidate_count: usize,
    pub(super) connector_endpoint_probe_count: usize,
    pub(super) total_thresholded_endpoint_match_count: usize,
    pub(super) matched_connector_count: usize,
    pub(super) dual_endpoint_match_connector_count: usize,
    pub(super) tight_endpoint_match_count: usize,
    pub(super) nearby_endpoint_match_count: usize,
    pub(super) no_thresholded_line_rule_endpoint_match_connector_count: usize,
    pub(super) single_or_missing_endpoint_line_rule_match_connector_count: usize,
    pub(super) connector_ownership_and_paint_order_unproven_connector_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
    pub(super) row_index: usize,
    pub(super) connector_candidate_count: usize,
    pub(super) total_thresholded_endpoint_match_count: usize,
    pub(super) matched_connector_count: usize,
    pub(super) dual_endpoint_match_connector_count: usize,
    pub(super) tight_endpoint_match_count: usize,
    pub(super) nearby_endpoint_match_count: usize,
    pub(super) tight_dual_endpoint_match_connector_count: usize,
    pub(super) horizontal_dual_endpoint_match_connector_count: usize,
    pub(super) vertical_dual_endpoint_match_connector_count: usize,
    pub(super) diagonal_dual_endpoint_match_connector_count: usize,
    pub(super) horizontal_tight_dual_endpoint_match_connector_count: usize,
    pub(super) vertical_tight_dual_endpoint_match_connector_count: usize,
    pub(super) diagonal_tight_dual_endpoint_match_connector_count: usize,
    pub(super) tight_non_diagonal_dual_projected_bbox_x_min_milli: Option<i32>,
    pub(super) tight_non_diagonal_dual_projected_bbox_y_min_milli: Option<i32>,
    pub(super) tight_non_diagonal_dual_projected_bbox_x_max_milli: Option<i32>,
    pub(super) tight_non_diagonal_dual_projected_bbox_y_max_milli: Option<i32>,
    pub(super) matched_projected_bbox_x_min_milli: Option<i32>,
    pub(super) matched_projected_bbox_y_min_milli: Option<i32>,
    pub(super) matched_projected_bbox_x_max_milli: Option<i32>,
    pub(super) matched_projected_bbox_y_max_milli: Option<i32>,
    pub(super) dual_projected_bbox_x_min_milli: Option<i32>,
    pub(super) dual_projected_bbox_y_min_milli: Option<i32>,
    pub(super) dual_projected_bbox_x_max_milli: Option<i32>,
    pub(super) dual_projected_bbox_y_max_milli: Option<i32>,
    pub(super) owner_promotion_gate_summary:
        FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary,
    pub(super) matched_connector_marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
    pub(super) dual_connector_marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
    pub(super) tight_non_diagonal_dual_connector_marker_style_profile:
        FdmOpenStrokeMarkerStyleProfile,
    pub(super) axis_rule_endpoint_match_marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
    pub(super) fdm_index_vector_offset: Option<usize>,
    pub(super) fdm_index_vector_len: Option<usize>,
    pub(super) fdm_index_valid_vector_offset: bool,
    pub(super) fdm_index_image_signature_count: usize,
    pub(super) fdm_index_segment_image_signature_count: usize,
    pub(super) fdm_index_vector_command_count: Option<usize>,
    pub(super) fdm_index_connector_candidate_count: Option<usize>,
    pub(super) fdm_index_non_connector_command_count: Option<usize>,
    pub(super) fdm_index_source_segment_relative_offset: Option<usize>,
    pub(super) fdm_index_source_segment_command_count: Option<usize>,
    pub(super) fdm_index_bbox_left: Option<i32>,
    pub(super) fdm_index_bbox_top: Option<i32>,
    pub(super) fdm_index_bbox_right: Option<i32>,
    pub(super) fdm_index_bbox_bottom: Option<i32>,
    pub(super) source_segment_backed_connector_count: usize,
    pub(super) source_segment_matches_index_entry_connector_count: usize,
    pub(super) source_segment_missing_connector_count: usize,
    pub(super) dual_endpoint_source_segment_backed_connector_count: usize,
    pub(super) dual_endpoint_source_segment_matches_index_entry_connector_count: usize,
    pub(super) dual_endpoint_image_bearing_segment_connector_count: usize,
    pub(super) fdm_index_bbox_contains_connector_count: usize,
    pub(super) fdm_index_bbox_overlaps_connector_count: usize,
    pub(super) fdm_index_bbox_disjoint_connector_count: usize,
    pub(super) fdm_index_bbox_source_bbox_missing_connector_count: usize,
    pub(super) dual_endpoint_fdm_index_bbox_contains_connector_count: usize,
    pub(super) dual_endpoint_fdm_index_bbox_overlaps_connector_count: usize,
    pub(super) dual_endpoint_fdm_index_bbox_disjoint_connector_count: usize,
    pub(super) dual_endpoint_fdm_index_bbox_source_bbox_missing_connector_count: usize,
    pub(super) dual_endpoint_axis_rule_source_order_backed_connector_count: usize,
    pub(super) dual_endpoint_connector_parent_relative_offset_min: Option<usize>,
    pub(super) dual_endpoint_connector_parent_relative_offset_max: Option<usize>,
    pub(super) dual_endpoint_axis_rule_parent_relative_offset_min: Option<usize>,
    pub(super) dual_endpoint_axis_rule_parent_relative_offset_max: Option<usize>,
    pub(super) dual_endpoint_connector_before_axis_rule_parent_span_count: usize,
    pub(super) dual_endpoint_connector_between_axis_rule_parent_span_count: usize,
    pub(super) dual_endpoint_connector_after_axis_rule_parent_span_count: usize,
    pub(super) dual_endpoint_connector_axis_rule_parent_span_unclassified_count: usize,
}

impl FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
    pub(super) fn non_diagonal_dual_endpoint_match_connector_count(self) -> usize {
        self.horizontal_dual_endpoint_match_connector_count
            + self.vertical_dual_endpoint_match_connector_count
    }

    pub(super) fn non_diagonal_tight_dual_endpoint_match_connector_count(self) -> usize {
        self.horizontal_tight_dual_endpoint_match_connector_count
            + self.vertical_tight_dual_endpoint_match_connector_count
    }

    pub(super) fn image_bearing_segment_candidate(self) -> bool {
        self.fdm_index_image_signature_count > 0 || self.fdm_index_segment_image_signature_count > 0
    }

    pub(super) fn fdm_index_segment_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_source_segment_backed_connector_count
            < self.dual_endpoint_match_connector_count
        {
            "connector-source-segment-membership-missing"
        } else if self.dual_endpoint_source_segment_matches_index_entry_connector_count
            < self.dual_endpoint_match_connector_count
        {
            "connector-source-segment-does-not-match-fdm-index-row"
        } else if self.dual_endpoint_image_bearing_segment_connector_count > 0 {
            "fdm-index-image-bearing-segment-role-unproven"
        } else {
            "fdm-index-segment-ownership-and-paint-order-unproven"
        }
    }

    pub(super) fn fdm_index_connector_composition_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_image_bearing_segment_connector_count > 0 {
            "fdm-index-image-bearing-segment-role-unproven"
        } else if self.fdm_index_connector_candidate_count.unwrap_or_default() == 0 {
            "fdm-index-connector-candidate-composition-missing"
        } else if self.dual_endpoint_fdm_index_bbox_contains_connector_count
            == self.dual_endpoint_match_connector_count
        {
            "fdm-index-contained-composition-internal-stroke-role-unproven"
        } else if self.dual_endpoint_fdm_index_bbox_disjoint_connector_count
            == self.dual_endpoint_match_connector_count
        {
            "fdm-index-disjoint-connector-composition-ownership-and-paint-order-unproven"
        } else {
            "fdm-index-mixed-connector-composition-ownership-and-paint-order-unproven"
        }
    }

    pub(super) fn fdm_index_bbox_relation_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_fdm_index_bbox_source_bbox_missing_connector_count > 0 {
            "connector-source-bbox-missing"
        } else if self.dual_endpoint_image_bearing_segment_connector_count > 0 {
            "fdm-index-image-bearing-segment-role-unproven"
        } else if self.dual_endpoint_fdm_index_bbox_contains_connector_count
            == self.dual_endpoint_match_connector_count
        {
            "fdm-index-bbox-contained-internal-stroke-role-unproven"
        } else if self.dual_endpoint_fdm_index_bbox_disjoint_connector_count
            == self.dual_endpoint_match_connector_count
        {
            "fdm-index-bbox-disjoint-connector-ownership-unproven"
        } else {
            "fdm-index-bbox-mixed-connector-relation-unproven"
        }
    }

    pub(super) fn axis_rule_source_order_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_axis_rule_source_order_backed_connector_count
            < self.dual_endpoint_match_connector_count
        {
            "axis-rule-source-order-evidence-missing"
        } else if self.dual_endpoint_connector_axis_rule_parent_span_unclassified_count > 0 {
            "axis-rule-source-order-relation-unclassified"
        } else if self.dual_endpoint_connector_before_axis_rule_parent_span_count
            == self.dual_endpoint_match_connector_count
        {
            "connector-before-axis-rule-parent-span-paint-order-unproven"
        } else if self.dual_endpoint_connector_between_axis_rule_parent_span_count
            == self.dual_endpoint_match_connector_count
        {
            "connector-between-axis-rule-parent-span-paint-order-unproven"
        } else if self.dual_endpoint_connector_after_axis_rule_parent_span_count
            == self.dual_endpoint_match_connector_count
        {
            "connector-after-axis-rule-parent-span-paint-order-unproven"
        } else {
            "mixed-connector-axis-rule-parent-span-paint-order-unproven"
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
    pub(super) dual_endpoint_match_connector_count: usize,
    pub(super) dual_endpoint_owner_candidate_count: usize,
    pub(super) nearest_fdm_owner_rows_match_count: usize,
    pub(super) nearest_fdm_owner_row_matches_connector_row_count: usize,
    pub(super) mixed_top_level_vs_nested_order_namespace_count: usize,
    pub(super) parent_normalized_ordered_same_row_same_connector_count: usize,
    pub(super) missing_endpoint_owner_candidate_count: usize,
    pub(super) nearest_owner_row_mismatch_count: usize,
    pub(super) nearest_owner_row_not_connector_row_count: usize,
    pub(super) before_owner_parent_command_span_count: usize,
    pub(super) between_owner_parent_command_span_count: usize,
    pub(super) after_owner_parent_command_span_count: usize,
    pub(super) parent_command_relation_unclassified_count: usize,
    pub(super) before_owner_parent_relative_offset_span_count: usize,
    pub(super) between_owner_parent_relative_offset_span_count: usize,
    pub(super) after_owner_parent_relative_offset_span_count: usize,
    pub(super) parent_relative_offset_relation_unclassified_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmOpenStrokeMarkerStyleProfile {
    pub(super) command_count: usize,
    pub(super) line_marker_count: usize,
    pub(super) path_marker_count: usize,
    pub(super) bezier_marker_count: usize,
    pub(super) ellipse_marker_count: usize,
    pub(super) other_marker_count: usize,
    pub(super) style_0000_count: usize,
    pub(super) style_0005_count: usize,
    pub(super) style_0080_count: usize,
    pub(super) style_00a0_count: usize,
    pub(super) other_style_count: usize,
}

impl FdmOpenStrokeMarkerStyleProfile {
    pub(super) fn marker_family_diversity_count(self) -> usize {
        [
            self.line_marker_count,
            self.path_marker_count,
            self.bezier_marker_count,
            self.ellipse_marker_count,
            self.other_marker_count,
        ]
        .into_iter()
        .filter(|count| *count > 0)
        .count()
    }

    pub(super) fn style_word_diversity_count(self) -> usize {
        [
            self.style_0000_count,
            self.style_0005_count,
            self.style_0080_count,
            self.style_00a0_count,
            self.other_style_count,
        ]
        .into_iter()
        .filter(|count| *count > 0)
        .count()
    }

    pub(super) fn dominant_marker_family(self) -> (&'static str, usize) {
        [
            ("line-marker", self.line_marker_count),
            ("path-marker", self.path_marker_count),
            ("bezier-marker", self.bezier_marker_count),
            ("ellipse-marker", self.ellipse_marker_count),
            ("other-marker", self.other_marker_count),
        ]
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap_or(("none", 0))
    }

    pub(super) fn dominant_style_word(self) -> (&'static str, usize) {
        [
            ("0x0000", self.style_0000_count),
            ("0x0005", self.style_0005_count),
            ("0x0080", self.style_0080_count),
            ("0x00a0", self.style_00a0_count),
            ("other-style", self.other_style_count),
        ]
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap_or(("none", 0))
    }

    pub(super) fn marker_family_homogeneous(self) -> bool {
        self.command_count > 0 && self.marker_family_diversity_count() == 1
    }

    pub(super) fn style_word_homogeneous(self) -> bool {
        self.command_count > 0 && self.style_word_diversity_count() == 1
    }

    pub(super) fn homogeneous_marker_style_candidate(self) -> bool {
        self.marker_family_homogeneous() && self.style_word_homogeneous()
    }

    pub(super) fn marker_style_role_promotion_blocked_reason(self) -> &'static str {
        if self.command_count == 0 {
            "marker-style-profile-empty"
        } else if !self.marker_family_homogeneous() && !self.style_word_homogeneous() {
            "mixed-marker-family-and-style-word-role-unproven"
        } else if !self.marker_family_homogeneous() {
            "mixed-marker-family-role-unproven"
        } else if !self.style_word_homogeneous() {
            "mixed-style-word-role-unproven"
        } else {
            "homogeneous-marker-style-still-needs-owner-and-paint-order"
        }
    }
}

impl FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
    pub(super) fn parent_normalized_order_gate_blocked_reason(self) -> &'static str {
        if self.dual_endpoint_match_connector_count == 0 {
            "no-same-row-axis-rule-dual-endpoint-match"
        } else if self.dual_endpoint_owner_candidate_count
            < self.dual_endpoint_match_connector_count
        {
            "missing-axis-rule-dual-endpoint-owner-candidate"
        } else if self.nearest_fdm_owner_rows_match_count < self.dual_endpoint_match_connector_count
        {
            "nearest-owner-row-mismatch"
        } else if self.nearest_fdm_owner_row_matches_connector_row_count
            < self.dual_endpoint_match_connector_count
        {
            "nearest-owner-row-not-connector-row"
        } else if self.parent_command_relation_unclassified_count > 0 {
            "connector-parent-command-relation-unclassified"
        } else if self.parent_relative_offset_relation_unclassified_count > 0 {
            "connector-parent-relative-offset-relation-unclassified"
        } else if self.between_owner_parent_command_span_count
            < self.dual_endpoint_match_connector_count
        {
            "connector-parent-command-outside-nearest-owner-parent-command-span"
        } else if self.between_owner_parent_relative_offset_span_count
            < self.dual_endpoint_match_connector_count
        {
            "connector-parent-relative-offset-outside-nearest-owner-parent-relative-offset-span"
        } else if self.parent_normalized_ordered_same_row_same_connector_count
            < self.dual_endpoint_match_connector_count
        {
            "parent-command-source-order-axis-disagreement"
        } else {
            "none"
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmConnectorOwnerRowCohortDiagnosticSummary {
    pub(super) connector_candidate_count: usize,
    pub(super) total_thresholded_endpoint_match_count: usize,
    pub(super) matched_connector_count: usize,
    pub(super) dual_endpoint_match_connector_count: usize,
    pub(super) tight_endpoint_match_count: usize,
    pub(super) nearby_endpoint_match_count: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct FdmConnectorMatchedRowDiagnosticSummary {
    pub(super) connector_candidate_count: usize,
    pub(super) total_thresholded_endpoint_match_count: usize,
    pub(super) matched_connector_count: usize,
    pub(super) dual_endpoint_match_connector_count: usize,
    pub(super) start_only_match_connector_count: usize,
    pub(super) end_only_match_connector_count: usize,
    pub(super) tight_endpoint_match_count: usize,
    pub(super) nearby_endpoint_match_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct FdmOpenStrokeCohortSummary {
    pub(super) primitive_count: usize,
    pub(super) open_stroke_count: usize,
    pub(super) connector_candidate_count: usize,
    pub(super) horizontal_count: usize,
    pub(super) vertical_count: usize,
    pub(super) diagonal_count: usize,
    pub(super) line_marker_count: usize,
    pub(super) non_line_marker_count: usize,
    pub(super) row_count: usize,
    pub(super) dominant_connector_row_index: Option<usize>,
    pub(super) dominant_connector_row_connector_candidate_count: usize,
    pub(super) dominant_connector_row_open_stroke_count: usize,
    pub(super) dominant_connector_row_horizontal_count: usize,
    pub(super) dominant_connector_row_vertical_count: usize,
    pub(super) row_cohorts: Vec<FdmOpenStrokeRowCohortSummary>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct FdmOpenStrokeRowCohortSummary {
    pub(super) row_index: usize,
    pub(super) open_stroke_count: usize,
    pub(super) connector_candidate_count: usize,
    pub(super) horizontal_count: usize,
    pub(super) vertical_count: usize,
    pub(super) diagonal_count: usize,
    pub(super) line_marker_count: usize,
    pub(super) non_line_marker_count: usize,
    pub(super) command_index_min: Option<usize>,
    pub(super) command_index_max: Option<usize>,
    pub(super) relative_offset_min: Option<usize>,
    pub(super) relative_offset_max: Option<usize>,
    pub(super) source_bbox_union: Option<(i32, i32, i32, i32)>,
    pub(super) projected_bbox_union: Option<(f32, f32, f32, f32)>,
    pub(super) marker_style_profile: FdmOpenStrokeMarkerStyleProfile,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FdmOpenStrokeAxisRule<'a> {
    pub(super) diagnostic: FdmCommandDiagnostic<'a>,
    pub(super) orientation: &'static str,
    pub(super) line_offset_units: f32,
    pub(super) line_extent_units: f32,
    pub(super) group_index: f32,
    pub(super) end_group_index: f32,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum FdmConnectorEndpointOwnerCandidate<'a> {
    Primitive {
        diagnostic: FdmCommandDiagnostic<'a>,
        bbox: (f32, f32, f32, f32),
        distance_px: f32,
    },
    TextSlot {
        slot: &'a ShanaiLanTextSlot,
        bbox: (f32, f32, f32, f32),
        distance_px: f32,
    },
}

impl FdmConnectorEndpointOwnerCandidate<'_> {
    pub(super) fn distance_px(self) -> f32 {
        match self {
            FdmConnectorEndpointOwnerCandidate::Primitive { distance_px, .. }
            | FdmConnectorEndpointOwnerCandidate::TextSlot { distance_px, .. } => distance_px,
        }
    }

    pub(super) fn rank(self) -> usize {
        match self {
            FdmConnectorEndpointOwnerCandidate::Primitive { .. } => 0,
            FdmConnectorEndpointOwnerCandidate::TextSlot { .. } => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SuccessDataTestFdmProjection {
    pub(super) role: &'static str,
    pub(super) source_left: i32,
    pub(super) source_top: i32,
    pub(super) source_right: i32,
    pub(super) source_bottom: i32,
    pub(super) target_x_px: f32,
    pub(super) target_y_px: f32,
    pub(super) target_width_px: f32,
    pub(super) target_height_px: f32,
    pub(super) scale_mode: SuccessDataTestFdmScaleMode,
    pub(super) text_corroboration_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SuccessDataTestFdmScaleMode {
    IndependentReferenceBox,
    UniformUnitsFromHorizontalSpan,
}

impl SuccessDataTestFdmScaleMode {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            SuccessDataTestFdmScaleMode::IndependentReferenceBox => "independent-reference-box",
            SuccessDataTestFdmScaleMode::UniformUnitsFromHorizontalSpan => {
                "uniform-units-from-horizontal-span"
            }
        }
    }
}

pub(super) type FdmTextMaskRightNeighborMatch<'a> =
    (&'a ShanaiLanTextSlot, (f32, f32, f32, f32), f32, f32, f32);

#[derive(Debug, Clone, Copy)]
pub(super) struct FdmTextMaskRightNeighborCandidate<'a> {
    pub(super) slot_index: usize,
    pub(super) slot: &'a ShanaiLanTextSlot,
    pub(super) bbox: (f32, f32, f32, f32),
    pub(super) gap_px: f32,
    pub(super) vertical_overlap_px: f32,
    pub(super) center_delta_y_px: f32,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FdmTextMaskPreFragmentBridgeMetrics {
    pub(super) pre_fragment_unit_count: usize,
    pub(super) pre_fragment_grid_units: usize,
    pub(super) pre_fragment_projected_width_px: f32,
    pub(super) line_start_x: f32,
    pub(super) text_start_x: f32,
    pub(super) source_begins_after_line_start: bool,
    pub(super) source_ends_before_text_start: bool,
    pub(super) source_bbox_within_pre_fragment_projection: bool,
    pub(super) source_bbox_right_to_text_start_px: f32,
    pub(super) text_baseline_minus_source_bottom_px: f32,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FdmTextMaskSourceTransformCandidate<'a> {
    pub(super) row_index: usize,
    pub(super) candidate_class: &'static str,
    pub(super) component_index: Option<usize>,
    pub(super) slot_index: usize,
    pub(super) slot: &'a ShanaiLanTextSlot,
    pub(super) source_bbox: (i32, i32, i32, i32),
    pub(super) projected_bbox: (f32, f32, f32, f32),
    pub(super) metrics: FdmTextMaskPreFragmentBridgeMetrics,
    pub(super) cohort_component_agreement: bool,
    pub(super) current_projection_grid_start: f32,
    pub(super) current_projection_grid_end: f32,
    pub(super) current_projection_grid_span: f32,
    pub(super) source_units_per_text_grid_unit_x: f32,
    pub(super) line_start_source_x: f32,
    pub(super) text_start_source_x: f32,
    pub(super) source_gap_to_text_start_x: f32,
}

pub(super) fn fdm_frame_diagnostics(document: &Document) -> Vec<FdmFrameDiagnostic<'_>> {
    if !document_has_shanai_lan_fdm_frame_evidence(document) {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for entry in candidate
            .fdm_index_entry_candidates()
            .iter()
            .filter(|entry| !entry.segment_image_signature_hits().is_empty())
        {
            if let Some(frame_record) = fdm_frame_record_for_entry(document, entry) {
                diagnostics.push(FdmFrameDiagnostic {
                    candidate_index,
                    candidate,
                    entry,
                    frame_record,
                });
            }
        }
    }
    diagnostics
}

pub(super) fn fdm_frame_record_for_entry<'a>(
    document: &'a Document,
    entry: &ObjectFdmIndexEntryCandidate,
) -> Option<&'a ObjectFrameRecordCandidate> {
    document.object_frame_records().iter().find(|record| {
        usize::from(record.object_id()) == entry.row_index()
            || record.row_index() == entry.row_index()
    })
}

pub(super) fn fdm_command_diagnostics(document: &Document) -> Vec<FdmCommandDiagnostic<'_>> {
    if !document_has_shanai_lan_fdm_command_evidence(document) {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for entry in candidate.fdm_index_entry_candidates() {
            for command in entry
                .vector_commands()
                .iter()
                .filter(|command| command.bbox().is_some())
            {
                diagnostics.push(FdmCommandDiagnostic {
                    candidate_index,
                    candidate,
                    entry,
                    command,
                });
            }
        }
    }
    diagnostics
}

pub(super) fn fdm_command_projection_extent(
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> Option<FdmCommandProjectionExtent> {
    let mut iter = diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.command.bbox())
        .map(normalize_fdm_bbox);
    let first = iter.next()?;
    let mut extent = FdmCommandProjectionExtent {
        left: first.0,
        top: first.1,
        right: first.2,
        bottom: first.3,
    };
    for bbox in iter {
        extent.left = extent.left.min(bbox.0);
        extent.top = extent.top.min(bbox.1);
        extent.right = extent.right.max(bbox.2);
        extent.bottom = extent.bottom.max(bbox.3);
    }
    if extent.left >= extent.right || extent.top >= extent.bottom {
        return None;
    }
    Some(extent)
}

pub(super) fn fdm_vector_primitive_diagnostics(
    document: &Document,
) -> Vec<FdmCommandDiagnostic<'_>> {
    if !document_has_shanai_lan_fdm_command_evidence(document) {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    for (candidate_index, candidate) in document.object_stream_candidates().iter().enumerate() {
        for entry in candidate.fdm_index_entry_candidates() {
            for command in entry.vector_commands().iter().filter(|command| {
                FDM_VECTOR_RENDERED_PRIMITIVE_MARKERS.contains(command.marker())
                    && command.has_renderable_geometry()
            }) {
                diagnostics.push(FdmCommandDiagnostic {
                    candidate_index,
                    candidate,
                    entry,
                    command,
                });
            }
        }
    }
    diagnostics
}

pub(super) fn fdm_image_overlay_diagnostics_json(document: &Document) -> Vec<String> {
    let mut diagnostics = Vec::new();
    for candidate in document.object_stream_candidates() {
        for entry in candidate
            .fdm_index_entry_candidates()
            .iter()
            .filter(|entry| !entry.segment_image_signature_hits().is_empty())
        {
            let bbox = entry.bbox();
            let normalized = normalize_fdm_bbox(bbox);
            let bbox_width = normalized.2.saturating_sub(normalized.0);
            let bbox_height = normalized.3.saturating_sub(normalized.1);
            let mut output = String::new();
            output.push_str("{\"type\":\"jtdFdmVectorImageCandidate\",\"sourcePath\":");
            output.push_str(&json_string(candidate.path()));
            output.push_str(",\"indexPath\":");
            output.push_str(&json_string(entry.index_path()));
            output.push_str(",\"vectorPath\":");
            output.push_str(&json_string(entry.vector_path()));
            output.push_str(",\"rowIndex\":");
            output.push_str(&entry.row_index().to_string());
            output.push_str(",\"vectorOffset\":");
            output.push_str(&entry.vector_offset().to_string());
            output.push_str(",\"nextVectorOffset\":");
            output.push_str(&entry.next_vector_offset().to_string());
            output.push_str(",\"vectorLength\":");
            output.push_str(&entry.vector_len().to_string());
            output.push_str(",\"kind\":");
            output.push_str(&entry.kind().to_string());
            output.push_str(",\"kindHex\":");
            output.push_str(&json_string(&format!("0x{:04x}", entry.kind())));
            output.push_str(",\"bbox\":");
            push_object_fdm_index_bbox_json(&mut output, bbox);
            output.push_str(",\"normalizedBbox\":");
            push_fdm_normalized_bbox_json(&mut output, normalized);
            output.push_str(",\"bboxWidth\":");
            output.push_str(&bbox_width.to_string());
            output.push_str(",\"bboxHeight\":");
            output.push_str(&bbox_height.to_string());
            output.push_str(",\"bboxOrder\":");
            output.push_str(&json_string(fdm_bbox_order(bbox)));
            output.push_str(",\"bboxPlausible\":");
            output.push_str(if fdm_bbox_is_plausible(bbox) {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"imageSignatures\":");
            push_object_image_signature_hits_json(&mut output, entry.image_signature_hits());
            output.push_str(",\"segmentImageSignatures\":");
            push_object_image_signature_hits_json(
                &mut output,
                entry.segment_image_signature_hits(),
            );
            output.push_str(",\"completePayloads\":");
            output.push_str(&fdm_entry_complete_payload_count(candidate, entry).to_string());
            output.push_str(",\"placementProven\":false,\"renderable\":false,\"reason\":\"page-placement-unproven\",\"decoded\":false}");
            diagnostics.push(output);
        }
    }
    diagnostics
}

pub(super) fn fdm_entry_complete_payload_count(
    candidate: &ObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> usize {
    candidate
        .image_payload_spans()
        .iter()
        .filter(|span| {
            span.complete()
                && span.signature_offset() >= entry.vector_offset()
                && span.signature_offset() < entry.next_vector_offset()
        })
        .count()
}

pub(super) fn fdm_entry_image_payload_extraction_status(
    candidate: &ObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> &'static str {
    if entry.image_signature_hits().is_empty() && entry.segment_image_signature_hits().is_empty() {
        "no-image-signature"
    } else if fdm_entry_complete_payload_count(candidate, entry) > 0 {
        "complete-payload-in-fdm-index-segment"
    } else if candidate
        .image_payload_spans()
        .iter()
        .any(|span| span.complete())
    {
        "complete-payload-elsewhere-in-vector-stream"
    } else {
        "signature-without-complete-payload"
    }
}

pub(super) fn fdm_entry_frame_render_blocked_reason(
    candidate: &ObjectStreamCandidate,
    entry: &ObjectFdmIndexEntryCandidate,
) -> &'static str {
    match fdm_entry_image_payload_extraction_status(candidate, entry) {
        "signature-without-complete-payload" => {
            "image-signature-without-complete-payload-role-unproven"
        }
        "no-image-signature" => "fdm-frame-image-payload-absent",
        "complete-payload-in-fdm-index-segment" => {
            "fdm-frame-linked-image-payload-placement-and-paint-order-unproven"
        }
        _ => "fdm-frame-image-placement-and-paint-order-unproven",
    }
}

pub(super) fn fdm_index_segment_bbox_axis_pair_gate(
    candidate: &ObjectStreamCandidate,
) -> Option<FdmIndexSegmentBboxAxisPairGate> {
    let valid_index_row_count = candidate
        .fdm_index_entry_candidates()
        .iter()
        .filter(|entry| entry.valid_vector_offset())
        .count();
    if valid_index_row_count == 0 {
        return None;
    }

    let mut linked_row_count = 0usize;
    let mut axis_pair_order_agreement_row_count = 0usize;
    for entry in candidate
        .fdm_index_entry_candidates()
        .iter()
        .filter(|entry| entry.valid_vector_offset())
    {
        let Some(segment_bbox) = candidate
            .fdm_raw_vector_segments()
            .iter()
            .find(|segment| segment.relative_offset() == entry.vector_offset())
            .and_then(ObjectFdmVectorSegmentCandidate::bbox)
        else {
            continue;
        };
        linked_row_count += 1;
        let index_bbox = entry.bbox();
        if index_bbox.left() == segment_bbox.left()
            && index_bbox.top() == segment_bbox.right()
            && index_bbox.right() == segment_bbox.top()
            && index_bbox.bottom() == segment_bbox.bottom()
        {
            axis_pair_order_agreement_row_count += 1;
        }
    }

    (linked_row_count > 0).then_some(FdmIndexSegmentBboxAxisPairGate::new(
        valid_index_row_count,
        linked_row_count,
        axis_pair_order_agreement_row_count,
    ))
}

pub(super) fn normalize_fdm_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.right()),
        bbox.top().min(bbox.bottom()),
        bbox.left().max(bbox.right()),
        bbox.top().max(bbox.bottom()),
    )
}

pub(super) fn normalize_fdm_index_entry_bbox(bbox: ObjectFdmIndexBbox) -> (i32, i32, i32, i32) {
    (
        bbox.left().min(bbox.top()),
        bbox.right().min(bbox.bottom()),
        bbox.left().max(bbox.top()),
        bbox.right().max(bbox.bottom()),
    )
}

pub(super) fn fdm_bbox_center(bbox: (i32, i32, i32, i32)) -> (i32, i32) {
    let center_x = i64::from(bbox.0) + (i64::from(bbox.2) - i64::from(bbox.0)) / 2;
    let center_y = i64::from(bbox.1) + (i64::from(bbox.3) - i64::from(bbox.1)) / 2;
    (center_x as i32, center_y as i32)
}

pub(super) fn fdm_bbox_extent_union(
    current: Option<(i32, i32, i32, i32)>,
    next: (i32, i32, i32, i32),
) -> Option<(i32, i32, i32, i32)> {
    match current {
        Some((left, top, right, bottom)) => Some((
            left.min(next.0),
            top.min(next.1),
            right.max(next.2),
            bottom.max(next.3),
        )),
        None => Some(next),
    }
}

pub(super) fn push_fdm_normalized_bbox_json(output: &mut String, bbox: (i32, i32, i32, i32)) {
    output.push_str("{\"left\":");
    output.push_str(&bbox.0.to_string());
    output.push_str(",\"top\":");
    output.push_str(&bbox.1.to_string());
    output.push_str(",\"right\":");
    output.push_str(&bbox.2.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&bbox.3.to_string());
    output.push('}');
}

pub(super) fn fdm_bbox_order(bbox: ObjectFdmIndexBbox) -> &'static str {
    match (bbox.left() <= bbox.right(), bbox.top() <= bbox.bottom()) {
        (true, true) => "forward",
        (false, true) => "inverted-x",
        (true, false) => "inverted-y",
        (false, false) => "inverted-xy",
    }
}

pub(super) fn fdm_bbox_is_plausible(bbox: ObjectFdmIndexBbox) -> bool {
    let normalized = normalize_fdm_bbox(bbox);
    let width = normalized.2.saturating_sub(normalized.0);
    let height = normalized.3.saturating_sub(normalized.1);
    width > 0 && height > 0 && width <= 200_000 && height <= 200_000
}

pub(super) fn push_answer_sheet_fdm_text_geometry_evidence_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    let text_candidates = candidate.fdm_text_candidates();
    let index_entries = candidate.fdm_text_index_entry_candidates();
    let indexed_text_count = index_entries.len();
    let bbox_extent = text_candidates
        .iter()
        .filter_map(|candidate| candidate.bbox().map(normalize_fdm_bbox))
        .fold(None, fdm_bbox_extent_union);
    let index_bbox_extent = index_entries
        .iter()
        .map(|entry| normalize_fdm_bbox(entry.bbox()))
        .fold(None, fdm_bbox_extent_union);

    output.push_str("{\"source\":\"FDMText\",\"sourcePath\":");
    output.push_str(&json_string(candidate.path()));
    output.push_str(",\"textCount\":");
    output.push_str(&text_candidates.len().to_string());
    output.push_str(",\"indexedTextCount\":");
    output.push_str(&indexed_text_count.to_string());
    output.push_str(",\"bboxExtent\":");
    match bbox_extent {
        Some((left, top, right, bottom)) => {
            output.push_str(&format!(
                "{{\"left\":{left},\"top\":{top},\"right\":{right},\"bottom\":{bottom},\"width\":{},\"height\":{}}}",
                right - left,
                bottom - top
            ));
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"labels\":[");
    for (index, text) in text_candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"index\":");
        output.push_str(&index.to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(text.text()));
        output.push_str(",\"markerOffset\":");
        output.push_str(&text.marker_offset().to_string());
        output.push_str(",\"textOffset\":");
        output.push_str(&text.text_offset().to_string());
        output.push_str(",\"bbox\":");
        match text.bbox().map(normalize_fdm_bbox) {
            Some((left, top, right, bottom)) => {
                output.push_str(&format!(
                    "{{\"left\":{left},\"top\":{top},\"right\":{right},\"bottom\":{bottom},\"width\":{},\"height\":{}}}",
                    right - left,
                    bottom - top
                ));
            }
            None => output.push_str("null"),
        }
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"textIndexEntries\":[");
    for (index, entry) in index_entries.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_answer_sheet_fdm_text_index_entry_json(output, entry);
    }
    output.push_str("],\"indexBboxExtent\":");
    match index_bbox_extent {
        Some((left, top, right, bottom)) => {
            output.push_str(&format!(
                "{{\"left\":{left},\"top\":{top},\"right\":{right},\"bottom\":{bottom},\"width\":{},\"height\":{}}}",
                right - left,
                bottom - top
            ));
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"triangleSourceBboxCandidate\":");
    if let Some(bbox) = success_data_test_answer_sheet_triangle_source_bbox(candidate) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"geometryDecoded\":true,\"placementDecoded\":false,\"decoded\":false}");
}

pub(super) fn push_answer_sheet_fdm_text_index_entry_json(
    output: &mut String,
    entry: &ObjectFdmTextIndexEntryCandidate,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&entry.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&entry.index_offset().to_string());
    output.push_str(",\"textRecordOffset\":");
    output.push_str(&entry.text_record_offset().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&entry.kind().to_string());
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", entry.kind())));
    output.push_str(",\"bbox\":");
    push_object_fdm_index_bbox_json(output, entry.bbox());
    output.push_str(",\"textRecordBbox\":");
    if let Some(bbox) = entry.text_record_bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_answer_sheet_figure_link_evidence_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    let Some(link) = candidate.figure_link_candidate() else {
        output.push_str("null");
        return;
    };

    let mut relation_kinds = BTreeSet::new();
    for row in link.rows() {
        if let Some(kind) = row.relation_kind_candidate() {
            relation_kinds.insert(kind);
        }
    }

    output.push_str("{\"source\":\"figureLink\",\"sourcePath\":");
    output.push_str(&json_string(candidate.path()));
    output.push_str(",\"declaredRowCountCandidate\":");
    push_option_u16_json(output, link.declared_row_count_candidate());
    output.push_str(",\"rowStride\":");
    output.push_str(&link.row_stride().to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&link.rows().len().to_string());
    output.push_str(",\"relationKinds\":[");
    for (index, kind) in relation_kinds.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"kind\":");
        output.push_str(&kind.to_string());
        output.push_str(",\"kindHex\":");
        output.push_str(&json_string(&format!("0x{kind:04x}")));
        output.push('}');
    }
    output.push_str("],\"rows\":[");
    for (index, row) in link.rows().iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&row.row_index().to_string());
        output.push_str(",\"rowStart\":");
        output.push_str(&row.row_start().to_string());
        output.push_str(",\"sourceIdCandidate\":");
        push_option_u16_json(output, row.source_id_candidate());
        output.push_str(",\"relationKindCandidate\":");
        push_option_u16_json(output, row.relation_kind_candidate());
        output.push_str(",\"relationKindCandidateHex\":");
        push_option_u16_hex_json(output, row.relation_kind_candidate());
        output.push_str(",\"targetRowIndexCandidate\":");
        push_option_u16_json(output, row.target_row_index_candidate());
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("],\"geometryDecoded\":false,\"decoded\":false}");
}

pub(super) fn push_page_layer_fdm_frame_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmFrameDiagnostic<'_>,
) {
    let Some((x, y, width, height)) = fdm_frame_diagnostic_bbox(layout, diagnostic) else {
        return;
    };
    output.push_str("{\"type\":\"fdmFrameDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"fdmIndex+frame\",\"projectionKind\":\"fdmFrameDiagnosticProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(fdm_entry_frame_render_blocked_reason(
        diagnostic.candidate,
        diagnostic.entry,
    )));
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"indexPath\":");
    output.push_str(&json_string(diagnostic.entry.index_path()));
    output.push_str(",\"vectorPath\":");
    output.push_str(&json_string(diagnostic.entry.vector_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"kind\":");
    output.push_str(&diagnostic.entry.kind().to_string());
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", diagnostic.entry.kind())));
    output.push_str(",\"imageSignatures\":");
    push_object_image_signature_hits_json(output, diagnostic.entry.image_signature_hits());
    output.push_str(",\"segmentImageSignatures\":");
    push_object_image_signature_hits_json(output, diagnostic.entry.segment_image_signature_hits());
    output.push_str(",\"completePayloads\":");
    output.push_str(
        &fdm_entry_complete_payload_count(diagnostic.candidate, diagnostic.entry).to_string(),
    );
    output.push_str(",\"imagePayloadExtractionStatus\":");
    output.push_str(&json_string(fdm_entry_image_payload_extraction_status(
        diagnostic.candidate,
        diagnostic.entry,
    )));
    output.push_str(",\"matchedFrameRecord\":{\"sourcePath\":");
    output.push_str(&json_string(diagnostic.frame_record.source_path()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.frame_record.row_index().to_string());
    output.push_str(",\"objectId\":");
    output.push_str(&diagnostic.frame_record.object_id().to_string());
    output.push_str(",\"recordKind\":");
    output.push_str(&diagnostic.frame_record.record_kind().to_string());
    output.push_str(",\"recordKindHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.frame_record.record_kind()
    )));
    output.push_str(",\"objectType\":");
    output.push_str(&diagnostic.frame_record.object_type().to_string());
    output.push_str(",\"objectTypeHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.frame_record.object_type()
    )));
    output.push_str(",\"geometry\":{\"x\":");
    output.push_str(&diagnostic.frame_record.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&diagnostic.frame_record.y().to_string());
    output.push_str(",\"width\":");
    output.push_str(&diagnostic.frame_record.width().to_string());
    output.push_str(",\"height\":");
    output.push_str(&diagnostic.frame_record.height().to_string());
    output.push_str("}}");
    output.push('}');
}

pub(super) fn push_page_layer_fdm_command_diagnostic_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) {
    let Some((x, y, width, height)) = fdm_command_diagnostic_bbox(layout, diagnostic, extent)
    else {
        return;
    };
    output.push_str("{\"type\":\"fdmVectorCommandDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"fdmVectorCommand\",\"projectionKind\":\"fdmCommandBBoxReferenceProjection\",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"recordLength\":");
    output.push_str(&diagnostic.command.record_len().to_string());
    output.push_str(",\"declaredRecordLength\":");
    output.push_str(&diagnostic.command.declared_record_len().to_string());
    output.push_str(",\"compoundChildOffsets\":");
    push_u16_array_json(output, diagnostic.command.compound_child_offsets());
    output.push_str(",\"compoundChildLayoutGate\":");
    push_fdm_compound_child_layout_gate_json(output, diagnostic.command);
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"sourceBbox\":");
    if let Some(bbox) = diagnostic.command.bbox() {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"projectionExtent\":{\"left\":");
    output.push_str(&extent.left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&extent.top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&extent.right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&extent.bottom.to_string());
    output.push('}');
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(super) fn push_page_layer_fdm_projection_extent_summary_json(
    output: &mut String,
    layout: PageLayout,
    command_diagnostics: &[FdmCommandDiagnostic<'_>],
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    active_extent: FdmCommandProjectionExtent,
) {
    let primitive_extent = fdm_vector_primitive_source_projection_extent(primitive_diagnostics);
    let index_entry_extent = fdm_index_entry_projection_extent(command_diagnostics);
    output.push_str("{\"type\":\"fdmProjectionExtentSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"FDMVector command bboxes+FDMIndex entry bboxes\"");
    output.push_str(",\"projectionKind\":\"fdmProjectionExtentSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true,\"sourceBacked\":true");
    output.push_str(",\"activeRenderExtentBasis\":\"fdmVectorCommandBboxExtent\"");
    output.push_str(",\"renderPromotionBlockedReason\":\"fdm-vector-page-placement-transform-source-fields-unproven\"");
    output.push_str(",\"commandDiagnosticCount\":");
    output.push_str(&command_diagnostics.len().to_string());
    output.push_str(",\"renderedPrimitiveDiagnosticCount\":");
    output.push_str(&primitive_diagnostics.len().to_string());
    output.push_str(",\"fdmIndexEntryCount\":");
    output.push_str(&fdm_index_entry_count(command_diagnostics).to_string());
    output.push_str(",\"activeCommandExtent\":");
    push_fdm_command_projection_extent_json(output, active_extent);
    output.push_str(",\"renderedPrimitiveExtent\":");
    push_optional_fdm_command_projection_extent_json(output, primitive_extent);
    output.push_str(",\"fdmIndexEntryExtent\":");
    push_optional_fdm_command_projection_extent_json(output, index_entry_extent);
    output.push_str(",\"extentAgreement\":{\"commandMatchesRenderedPrimitives\":");
    output.push_str(&(primitive_extent == Some(active_extent)).to_string());
    output.push_str(",\"commandMatchesFdmIndexEntries\":");
    output.push_str(&(index_entry_extent == Some(active_extent)).to_string());
    output.push_str(",\"renderedPrimitivesMatchFdmIndexEntries\":");
    output.push_str(
        &(primitive_extent.is_some() && primitive_extent == index_entry_extent).to_string(),
    );
    output.push('}');
    output.push_str(",\"extentResiduals\":{\"commandVsRenderedPrimitives\":");
    push_fdm_command_projection_extent_residual_json(output, Some(active_extent), primitive_extent);
    output.push_str(",\"commandVsFdmIndexEntries\":");
    push_fdm_command_projection_extent_residual_json(
        output,
        Some(active_extent),
        index_entry_extent,
    );
    output.push_str(",\"renderedPrimitivesVsFdmIndexEntries\":");
    push_fdm_command_projection_extent_residual_json(output, primitive_extent, index_entry_extent);
    output.push('}');
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(super) fn fdm_index_entry_count(diagnostics: &[FdmCommandDiagnostic<'_>]) -> usize {
    diagnostics
        .iter()
        .map(|diagnostic| (diagnostic.candidate_index, diagnostic.entry.row_index()))
        .collect::<BTreeSet<_>>()
        .len()
}

pub(super) fn fdm_index_entry_projection_extent(
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> Option<FdmCommandProjectionExtent> {
    let mut seen = BTreeSet::<(usize, usize)>::new();
    let mut extent = None;
    for diagnostic in diagnostics {
        if !seen.insert((diagnostic.candidate_index, diagnostic.entry.row_index())) {
            continue;
        }
        extent = fdm_bbox_extent_union(
            extent,
            normalize_fdm_index_entry_bbox(diagnostic.entry.bbox()),
        );
    }
    extent.map(|(left, top, right, bottom)| FdmCommandProjectionExtent {
        left,
        top,
        right,
        bottom,
    })
}

pub(super) fn fdm_vector_primitive_source_projection_extent(
    diagnostics: &[FdmCommandDiagnostic<'_>],
) -> Option<FdmCommandProjectionExtent> {
    let extent = diagnostics
        .iter()
        .filter_map(|diagnostic| fdm_vector_command_source_bbox(diagnostic.command))
        .map(normalize_fdm_bbox)
        .fold(None, fdm_bbox_extent_union)?;
    Some(FdmCommandProjectionExtent {
        left: extent.0,
        top: extent.1,
        right: extent.2,
        bottom: extent.3,
    })
}

pub(super) fn push_optional_fdm_command_projection_extent_json(
    output: &mut String,
    extent: Option<FdmCommandProjectionExtent>,
) {
    if let Some(extent) = extent {
        push_fdm_command_projection_extent_json(output, extent);
    } else {
        output.push_str("null");
    }
}

pub(super) fn push_fdm_command_projection_extent_json(
    output: &mut String,
    extent: FdmCommandProjectionExtent,
) {
    output.push_str("{\"left\":");
    output.push_str(&extent.left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&extent.top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&extent.right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&extent.bottom.to_string());
    output.push_str(",\"spanX\":");
    output.push_str(&(extent.right - extent.left).to_string());
    output.push_str(",\"spanY\":");
    output.push_str(&(extent.bottom - extent.top).to_string());
    output.push('}');
}

pub(super) fn push_fdm_command_projection_extent_residual_json(
    output: &mut String,
    left: Option<FdmCommandProjectionExtent>,
    right: Option<FdmCommandProjectionExtent>,
) {
    let (Some(left), Some(right)) = (left, right) else {
        output.push_str("null");
        return;
    };
    let left_delta = right.left - left.left;
    let top_delta = right.top - left.top;
    let right_delta = right.right - left.right;
    let bottom_delta = right.bottom - left.bottom;
    let max_abs_delta = left_delta
        .abs()
        .max(top_delta.abs())
        .max(right_delta.abs())
        .max(bottom_delta.abs());
    output.push_str("{\"leftDelta\":");
    output.push_str(&left_delta.to_string());
    output.push_str(",\"topDelta\":");
    output.push_str(&top_delta.to_string());
    output.push_str(",\"rightDelta\":");
    output.push_str(&right_delta.to_string());
    output.push_str(",\"bottomDelta\":");
    output.push_str(&bottom_delta.to_string());
    output.push_str(",\"maxAbsDelta\":");
    output.push_str(&max_abs_delta.to_string());
    output.push('}');
}

pub(super) fn push_page_layer_fdm_vector_primitive_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    diagnostics: &[FdmCommandDiagnostic<'_>],
) {
    let Some((x, y, width, height)) = fdm_path_diagnostic_bbox(layout, diagnostic, extent) else {
        return;
    };
    output.push_str("{\"type\":\"fdmVectorPrimitiveProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\",\"projectionKind\":\"fdmVectorPrimitiveReferenceProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":true,\"referenceBacked\":true");
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
    output.push_str(",\"styleWord\":");
    output.push_str(&diagnostic.command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    output.push_str(",\"fillColor\":");
    push_fdm_vector_optional_color_json(output, diagnostic.command.fill_color());
    let render_fill_color = fdm_vector_render_fill_color(diagnostic, diagnostics);
    let render_gradient = fdm_vector_linear_gradient_colors(diagnostic.command);
    output.push_str(",\"renderFillKind\":");
    output.push_str(&json_string(if render_gradient.is_some() {
        "linearGradient"
    } else if render_fill_color == "none" {
        "none"
    } else {
        "solid"
    }));
    output.push_str(",\"renderFillColor\":");
    output.push_str(&json_string(&render_fill_color));
    output.push_str(",\"renderGradient\":");
    if let Some((from, to)) = render_gradient.as_ref() {
        output.push_str("{\"from\":");
        output.push_str(&json_string(from));
        output.push_str(",\"to\":");
        output.push_str(&json_string(to));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderCounterOverlay\":");
    output.push_str(
        if fdm_vector_filled_path_is_counter_overlay(diagnostic, diagnostics) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"strokeColor\":");
    push_fdm_vector_optional_color_json(output, diagnostic.command.stroke_color());
    output.push_str(",\"renderStrokeColor\":");
    output.push_str(&json_string(&fdm_vector_render_stroke_color(
        diagnostic,
        diagnostics,
    )));
    output.push_str(",\"pathClosed\":");
    output.push_str(if fdm_vector_primitive_is_closed(diagnostic.command) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!(
        "{:.3}",
        fdm_vector_stroke_width(diagnostic.command)
    ));
    output.push_str(",\"pathPointCount\":");
    output.push_str(&diagnostic.command.path_points().len().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&diagnostic.command.curve_segments().len().to_string());
    output.push_str(",\"ellipse\":");
    if let Some(ellipse) = diagnostic.command.ellipse() {
        push_fdm_vector_ellipse_json(output, ellipse);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourcePathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"paintCoverage\":");
    push_fdm_paint_coverage_json(
        output,
        fdm_vector_paint_coverage(layout, diagnostic, diagnostics, (x, y, width, height)),
    );
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(super) fn push_page_layer_fdm_vector_primitive_large_span_blocked_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
) {
    let Some((x, y, width, height)) = fdm_path_span_filter_blocked(layout, diagnostic, extent)
    else {
        return;
    };
    output.push_str("{\"type\":\"fdmVectorPrimitiveLargeSpanBlockedDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    let paint_coverage = fdm_vector_paint_coverage(
        layout,
        diagnostic,
        primitive_diagnostics,
        (x, y, width, height),
    );
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\",\"projectionKind\":\"fdmVectorPrimitiveLargeSpanFilteredProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"diagnosticOnly\":true,\"referenceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(if paint_coverage.page_fill_candidate {
        "fdm-page-fill-source-evidence-unproven"
    } else {
        "fdm-vector-large-span-filter-unproven"
    }));
    output.push_str(",\"filterBasis\":\"projected-page-span-ratio\"");
    output.push_str(",\"largeSpanFilterMaxPageRatio\":");
    output.push_str(&format!("{FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO:.6}"));
    output.push_str(",\"pageWidthRatio\":");
    output.push_str(&format!("{:.6}", width / layout.width_px()));
    output.push_str(",\"pageHeightRatio\":");
    output.push_str(&format!("{:.6}", height / layout.height_px()));
    let viewport = fdm_projection_viewport(layout);
    output.push_str(",\"viewportWidthRatio\":");
    output.push_str(&format!("{:.6}", width / viewport.width));
    output.push_str(",\"viewportHeightRatio\":");
    output.push_str(&format!("{:.6}", height / viewport.height));
    output.push_str(",\"paintCoverage\":");
    push_fdm_paint_coverage_json(output, paint_coverage);
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
    output.push_str(",\"styleWord\":");
    output.push_str(&diagnostic.command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    output.push_str(",\"pathClosed\":");
    output.push_str(if fdm_vector_primitive_is_closed(diagnostic.command) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!(
        "{:.3}",
        fdm_vector_stroke_width(diagnostic.command)
    ));
    output.push_str(",\"pathPointCount\":");
    output.push_str(&diagnostic.command.path_points().len().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&diagnostic.command.curve_segments().len().to_string());
    output.push_str(",\"sourcePathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
        push_object_fdm_index_bbox_json(output, bbox);
        output.push_str(",\"normalizedSourcePathBbox\":");
        push_fdm_normalized_bbox_json(output, normalize_fdm_bbox(bbox));
    } else {
        output.push_str("null,\"normalizedSourcePathBbox\":null");
    }
    output.push_str(",\"projectionExtent\":{\"left\":");
    output.push_str(&extent.left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&extent.top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&extent.right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&extent.bottom.to_string());
    output.push('}');
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(super) fn push_page_layer_fdm_text_mask_cohort_summary_json(
    output: &mut String,
    layout: PageLayout,
    cohorts: &[FdmTextMaskCohortDiagnosticSummary],
    text_projection: &ShanaiLanTextProjection,
) {
    output.push_str("{\"type\":\"fdmTextMaskCohortSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive+documentTextGroupLineProjection\"");
    output.push_str(",\"projectionKind\":\"fdmTextMaskCohortSummary\"");
    output.push_str(",\"basis\":\"fdmVectorClosedFillCohort+documentTextRightNeighbor\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"fdm-text-mask-document-text-alignment-unproven\"",
    );
    output.push_str(",\"candidatePredicate\":{\"minPrimitiveCount\":");
    output.push_str(&FDM_TEXT_MASK_COHORT_MIN_PRIMITIVES.to_string());
    output.push_str(",\"maxCohorts\":");
    output.push_str(&FDM_TEXT_MASK_COHORT_LIMIT.to_string());
    output.push_str(",\"rightNeighborMaxGapFactor\":");
    output.push_str(&format!("{FDM_TEXT_MASK_RIGHT_NEIGHBOR_MAX_GAP_FACTOR:.3}"));
    output.push_str(",\"requiresClosedFillPrimitive\":true,\"requiresBlackOrWhiteFill\":true}");
    output.push_str(",\"cohortCount\":");
    output.push_str(&cohorts.len().to_string());
    output.push_str(",\"rightNeighborCandidateCount\":");
    output.push_str(
        &cohorts
            .iter()
            .filter(|cohort| {
                fdm_text_mask_cohort_right_neighbor_text_slot(cohort, text_projection).is_some()
            })
            .count()
            .to_string(),
    );
    output.push_str(",\"topTextLikeComponentCandidateCount\":");
    output.push_str(
        &cohorts
            .iter()
            .filter(|cohort| cohort.top_text_like_component.is_some())
            .count()
            .to_string(),
    );
    output.push_str(",\"componentRightNeighborCandidateCount\":");
    output.push_str(
        &cohorts
            .iter()
            .filter(|cohort| {
                cohort
                    .top_text_like_component
                    .and_then(|component| component.projected_bbox)
                    .and_then(|bbox| {
                        fdm_text_mask_bbox_right_neighbor_text_slot(bbox, text_projection)
                    })
                    .is_some()
            })
            .count()
            .to_string(),
    );
    output.push_str(",\"cohorts\":[");
    for (index, cohort) in cohorts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_text_mask_cohort_json(output, cohort, text_projection);
    }
    output.push_str("]}");
}

pub(super) fn push_page_layer_fdm_text_mask_source_transform_candidate_summary_json(
    output: &mut String,
    layout: PageLayout,
    cohorts: &[FdmTextMaskCohortDiagnosticSummary],
    text_projection: &ShanaiLanTextProjection,
) {
    let candidates = fdm_text_mask_source_transform_candidates(cohorts, text_projection);
    let bridge_candidate_count = candidates
        .iter()
        .filter(|candidate| candidate.metrics.source_bbox_within_pre_fragment_projection)
        .count();
    let row_anchor_ambiguous_count = candidates
        .iter()
        .filter(|candidate| {
            candidate
                .slot
                .line_header_same_segment_group_run_distinct_text_group_count
                .is_some_and(|count| count > 1)
        })
        .count();
    let slot_not_split_count = candidates
        .iter()
        .filter(|candidate| !candidate.slot.split_from_text_run)
        .count();
    let cohort_component_agreement_count = candidates
        .iter()
        .filter(|candidate| candidate.cohort_component_agreement)
        .count();

    output.push_str("{\"type\":\"fdmTextMaskSourceTransformCandidateSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"fdmVectorClosedFillComponent+/DocumentText pre-fragment span\"");
    output.push_str(",\"projectionKind\":\"fdmTextMaskSourceTransformCandidateSummary\"");
    output.push_str(",\"basis\":\"topTextLikeComponent+preFragmentGridOffset\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"renderPromotionBlockedReason\":\"fdm-source-to-document-text-transform-reference-backed-and-row-anchor-unproven\"");
    output.push_str(",\"candidatePredicate\":{\"requiresTopTextLikeComponent\":true,\"requiresComponentSourceBbox\":true,\"requiresRightNeighborDocumentTextSlot\":true,\"requiresPreFragmentSpan\":true}");
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidates.len().to_string());
    output.push_str(",\"preFragmentBridgeCandidateCount\":");
    output.push_str(&bridge_candidate_count.to_string());
    output.push_str(",\"cohortComponentAgreementCount\":");
    output.push_str(&cohort_component_agreement_count.to_string());
    output.push_str(",\"rowAnchorAmbiguousCandidateCount\":");
    output.push_str(&row_anchor_ambiguous_count.to_string());
    output.push_str(",\"slotNotSplitCandidateCount\":");
    output.push_str(&slot_not_split_count.to_string());
    output.push_str(",\"sourceUnitsPerTextGridUnitXRange\":");
    push_fdm_text_mask_source_transform_ratio_range_json(output, &candidates);
    output.push_str(",\"promotionGate\":{\"sourceBacked\":true,\"referenceBacked\":true,\"promotionReady\":false,\"blockedReasons\":[\"document-text-grid-origin-reference-backed\",\"line-header-y-run-placement-semantics-unproven\",\"document-text-pre-fragment-fdm-mask-role-unproven\",\"fdm-text-mask-to-document-text-baseline-transform-unproven\",\"fdm-source-transform-cross-sample-support-missing\"],\"renderPromotionBlockedReason\":\"fdm-source-to-document-text-transform-reference-backed-and-row-anchor-unproven\"}");
    output.push_str(",\"candidates\":[");
    for (index, candidate) in candidates.iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_text_mask_source_transform_candidate_json(output, *candidate);
    }
    output.push_str("]}");
}

pub(super) fn push_fdm_text_mask_source_transform_ratio_range_json(
    output: &mut String,
    candidates: &[FdmTextMaskSourceTransformCandidate<'_>],
) {
    let mut ratios = candidates
        .iter()
        .map(|candidate| candidate.source_units_per_text_grid_unit_x)
        .filter(|ratio| ratio.is_finite());
    let Some(first) = ratios.next() else {
        output.push_str("null");
        return;
    };
    let (mut min_ratio, mut max_ratio) = (first, first);
    for ratio in ratios {
        min_ratio = min_ratio.min(ratio);
        max_ratio = max_ratio.max(ratio);
    }
    output.push_str("{\"min\":");
    output.push_str(&format!("{min_ratio:.3}"));
    output.push_str(",\"max\":");
    output.push_str(&format!("{max_ratio:.3}"));
    output.push('}');
}

pub(super) fn push_fdm_text_mask_source_transform_candidate_json(
    output: &mut String,
    candidate: FdmTextMaskSourceTransformCandidate<'_>,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&candidate.row_index.to_string());
    output.push_str(",\"candidateClass\":");
    output.push_str(&json_string(candidate.candidate_class));
    output.push_str(",\"componentIndex\":");
    push_option_usize_json(output, candidate.component_index);
    output.push_str(",\"slotIndex\":");
    output.push_str(&candidate.slot_index.to_string());
    output.push_str(",\"slotText\":");
    output.push_str(&json_string(&candidate.slot.text));
    output.push_str(",\"slotSourceUnitRange\":");
    output.push_str(&source_range_json(
        candidate.slot.source_span.unit_start(),
        candidate.slot.source_span.unit_end(),
    ));
    output.push_str(",\"slotGroupIndex\":");
    match candidate.slot.group_index {
        Some(group_index) => output.push_str(&group_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceBbox\":");
    push_fdm_normalized_bbox_json(output, candidate.source_bbox);
    output.push_str(",\"currentProjectedBbox\":");
    push_bbox_tuple_json(output, candidate.projected_bbox);
    output.push_str(",\"currentProjectionGridOffsetRange\":{\"start\":");
    output.push_str(&format!("{:.3}", candidate.current_projection_grid_start));
    output.push_str(",\"end\":");
    output.push_str(&format!("{:.3}", candidate.current_projection_grid_end));
    output.push_str(",\"span\":");
    output.push_str(&format!("{:.3}", candidate.current_projection_grid_span));
    output.push('}');
    output.push_str(",\"sourceXTransformCandidate\":{\"sourceUnitsPerTextGridUnit\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.source_units_per_text_grid_unit_x
    ));
    output.push_str(",\"lineStartSourceX\":");
    output.push_str(&format!("{:.3}", candidate.line_start_source_x));
    output.push_str(",\"textStartSourceX\":");
    output.push_str(&format!("{:.3}", candidate.text_start_source_x));
    output.push_str(",\"sourceGapToTextStartX\":");
    output.push_str(&format!("{:.3}", candidate.source_gap_to_text_start_x));
    output.push_str(",\"transformAuthorityProven\":false}");
    output.push_str(",\"preFragmentBridge\":{\"preFragmentUnitCount\":");
    output.push_str(&candidate.metrics.pre_fragment_unit_count.to_string());
    output.push_str(",\"preFragmentGridUnits\":");
    output.push_str(&candidate.metrics.pre_fragment_grid_units.to_string());
    output.push_str(",\"sourceBboxWithinPreFragmentProjection\":");
    output.push_str(
        &candidate
            .metrics
            .source_bbox_within_pre_fragment_projection
            .to_string(),
    );
    output.push_str(",\"sourceBboxRightToTextStartGapPx\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.metrics.source_bbox_right_to_text_start_px
    ));
    output.push_str(",\"baselineResidualPx\":");
    output.push_str(&format!(
        "{:.3}",
        candidate.metrics.text_baseline_minus_source_bottom_px
    ));
    output.push('}');
    output.push_str(",\"cohortComponentAgreement\":");
    output.push_str(&candidate.cohort_component_agreement.to_string());
    output.push_str(",\"rowAnchorAmbiguous\":");
    output.push_str(
        &candidate
            .slot
            .line_header_same_segment_group_run_distinct_text_group_count
            .is_some_and(|count| count > 1)
            .to_string(),
    );
    output.push_str(",\"splitFromTextRun\":");
    output.push_str(&candidate.slot.split_from_text_run.to_string());
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"fdm-source-transform-candidate-diagnostic-only\"}",
    );
}

pub(super) fn push_fdm_text_mask_cohort_json(
    output: &mut String,
    cohort: &FdmTextMaskCohortDiagnosticSummary,
    text_projection: &ShanaiLanTextProjection,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&cohort.row_index.to_string());
    output.push_str(",\"primitiveCount\":");
    output.push_str(&cohort.primitive_count.to_string());
    output.push_str(",\"blackFillPrimitiveCount\":");
    output.push_str(&cohort.black_fill_primitive_count.to_string());
    output.push_str(",\"whiteFillPrimitiveCount\":");
    output.push_str(&cohort.white_fill_primitive_count.to_string());
    output.push_str(",\"counterOverlayCount\":");
    output.push_str(&cohort.counter_overlay_count.to_string());
    output.push_str(",\"commandIndexMin\":");
    push_option_usize_json(output, cohort.command_index_min);
    output.push_str(",\"commandIndexMax\":");
    push_option_usize_json(output, cohort.command_index_max);
    output.push_str(",\"relativeOffsetMin\":");
    push_option_usize_json(output, cohort.relative_offset_min);
    output.push_str(",\"relativeOffsetMax\":");
    push_option_usize_json(output, cohort.relative_offset_max);
    output.push_str(",\"projectedBbox\":");
    if let Some(bbox) = cohort.projected_bbox {
        push_bbox_tuple_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceBbox\":");
    if let Some(bbox) = cohort.source_bbox {
        push_fdm_normalized_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rightNeighborTextSlotCandidate\":");
    push_fdm_text_mask_right_neighbor_text_slot_json(
        output,
        cohort.projected_bbox,
        text_projection,
        "right-neighbor-overlapping-y",
    );
    output.push_str(",\"componentCount\":");
    output.push_str(&cohort.component_count.to_string());
    output.push_str(",\"topTextLikeComponentCandidate\":");
    if let Some(component) = cohort.top_text_like_component {
        push_fdm_text_mask_component_json(output, component, text_projection);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rightNeighborPromotionReadiness\":");
    push_fdm_text_mask_right_neighbor_promotion_readiness_json(output, cohort, text_projection);
    output.push('}');
}

pub(super) fn push_fdm_text_mask_component_json(
    output: &mut String,
    component: FdmTextMaskComponentDiagnosticSummary,
    text_projection: &ShanaiLanTextProjection,
) {
    output.push_str("{\"source\":\"fdmVectorClosedFillComponent\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"fdm-text-mask-component-to-document-text-alignment-unproven\"",
    );
    output.push_str(",\"componentIndex\":");
    output.push_str(&component.component_index.to_string());
    output.push_str(",\"primitiveCount\":");
    output.push_str(&component.primitive_count.to_string());
    output.push_str(",\"blackFillPrimitiveCount\":");
    output.push_str(&component.black_fill_primitive_count.to_string());
    output.push_str(",\"whiteFillPrimitiveCount\":");
    output.push_str(&component.white_fill_primitive_count.to_string());
    output.push_str(",\"counterOverlayCount\":");
    output.push_str(&component.counter_overlay_count.to_string());
    output.push_str(",\"commandIndexMin\":");
    push_option_usize_json(output, component.command_index_min);
    output.push_str(",\"commandIndexMax\":");
    push_option_usize_json(output, component.command_index_max);
    output.push_str(",\"relativeOffsetMin\":");
    push_option_usize_json(output, component.relative_offset_min);
    output.push_str(",\"relativeOffsetMax\":");
    push_option_usize_json(output, component.relative_offset_max);
    output.push_str(",\"projectedBbox\":");
    if let Some(bbox) = component.projected_bbox {
        push_bbox_tuple_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"sourceBbox\":");
    if let Some(bbox) = component.source_bbox {
        push_fdm_normalized_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"rightNeighborTextSlotCandidate\":");
    push_fdm_text_mask_right_neighbor_text_slot_json(
        output,
        component.projected_bbox,
        text_projection,
        "component-right-neighbor-overlapping-y",
    );
    output.push('}');
}

pub(super) fn push_fdm_text_mask_right_neighbor_promotion_readiness_json(
    output: &mut String,
    cohort: &FdmTextMaskCohortDiagnosticSummary,
    text_projection: &ShanaiLanTextProjection,
) {
    let cohort_candidates = cohort
        .projected_bbox
        .map(|bbox| fdm_text_mask_bbox_right_neighbor_text_slot_candidates(bbox, text_projection))
        .unwrap_or_default();
    let component_bbox = cohort
        .top_text_like_component
        .and_then(|component| component.projected_bbox);
    let component_candidates = component_bbox
        .map(|bbox| fdm_text_mask_bbox_right_neighbor_text_slot_candidates(bbox, text_projection))
        .unwrap_or_default();
    let cohort_best = cohort_candidates.first().copied();
    let component_best = component_candidates.first().copied();
    let Some((selected, selected_bbox, selected_candidates)) = component_best
        .and_then(|candidate| Some((candidate, component_bbox?, &component_candidates)))
        .or_else(|| {
            cohort_best
                .and_then(|candidate| Some((candidate, cohort.projected_bbox?, &cohort_candidates)))
        })
    else {
        output.push_str("null");
        return;
    };
    let metrics =
        fdm_text_mask_pre_fragment_bridge_metrics(selected_bbox, text_projection, selected.slot);
    let cohort_component_agreement =
        cohort_best
            .zip(component_best)
            .is_some_and(|(cohort, component)| {
                fdm_text_mask_right_neighbor_candidates_same_slot(cohort, component)
            });
    let second_best = selected_candidates.get(1).copied();
    let gap_margin_px = second_best.map(|candidate| candidate.gap_px.abs() - selected.gap_px.abs());
    let row_anchor_ambiguous = selected
        .slot
        .line_header_same_segment_group_run_distinct_text_group_count
        .is_some_and(|count| count > 1);

    output.push_str("{\"type\":\"fdmTextMaskRightNeighborPromotionReadiness\"");
    output.push_str(",\"source\":\"fdmVectorClosedFillCohort+/DocumentText\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"cohortSlot\":");
    push_fdm_text_mask_right_neighbor_readiness_slot_json(output, cohort_best);
    output.push_str(",\"componentSlot\":");
    push_fdm_text_mask_right_neighbor_readiness_slot_json(output, component_best);
    output.push_str(",\"cohortComponentAgreement\":");
    output.push_str(&cohort_component_agreement.to_string());
    output.push_str(",\"bestGapPx\":");
    output.push_str(&format!("{:.3}", selected.gap_px));
    output.push_str(",\"secondBestGapPx\":");
    if let Some(second_best) = second_best {
        output.push_str(&format!("{:.3}", second_best.gap_px));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"gapMarginPx\":");
    if let Some(gap_margin_px) = gap_margin_px {
        output.push_str(&format!("{gap_margin_px:.3}"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"splitFromTextRun\":");
    output.push_str(&selected.slot.split_from_text_run.to_string());
    output.push_str(",\"fragmentCount\":");
    output.push_str(&selected.slot.fragment_context.fragment_count.to_string());
    output.push_str(",\"preFragmentUnitCount\":");
    output.push_str(&metrics.pre_fragment_unit_count.to_string());
    output.push_str(",\"sourceBboxWithinPreFragmentProjection\":");
    output.push_str(
        &metrics
            .source_bbox_within_pre_fragment_projection
            .to_string(),
    );
    output.push_str(",\"sourceBboxBeginsAfterLineStart\":");
    output.push_str(&metrics.source_begins_after_line_start.to_string());
    output.push_str(",\"sourceBboxEndsBeforeTextStart\":");
    output.push_str(&metrics.source_ends_before_text_start.to_string());
    output.push_str(",\"sameSegmentGroupRunDistinctTextGroupCount\":");
    push_option_usize_json(
        output,
        selected
            .slot
            .line_header_same_segment_group_run_distinct_text_group_count,
    );
    output.push_str(",\"rowAnchorAmbiguous\":");
    output.push_str(&row_anchor_ambiguous.to_string());
    output.push_str(",\"baselineResidualPx\":");
    output.push_str(&format!(
        "{:.3}",
        metrics.text_baseline_minus_source_bottom_px
    ));
    output.push_str(",\"promotionReady\":false,\"blockedReasons\":");
    push_fdm_text_mask_promotion_blocked_reasons_json(
        output,
        cohort_best,
        component_best,
        cohort_component_agreement,
        selected.slot,
        metrics,
        row_anchor_ambiguous,
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"fdm-text-mask-right-neighbor-promotion-readiness-blocked\"}");
}

pub(super) fn push_fdm_text_mask_right_neighbor_readiness_slot_json(
    output: &mut String,
    candidate: Option<FdmTextMaskRightNeighborCandidate<'_>>,
) {
    let Some(candidate) = candidate else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"slotIndex\":");
    output.push_str(&candidate.slot_index.to_string());
    output.push_str(",\"text\":");
    output.push_str(&json_string(&candidate.slot.text));
    output.push_str(",\"bbox\":");
    push_bbox_tuple_json(output, candidate.bbox);
    output.push_str(",\"horizontalGapPx\":");
    output.push_str(&format!("{:.3}", candidate.gap_px));
    output.push_str(",\"verticalOverlapPx\":");
    output.push_str(&format!("{:.3}", candidate.vertical_overlap_px));
    output.push_str(",\"centerDeltaYPx\":");
    output.push_str(&format!("{:.3}", candidate.center_delta_y_px));
    output.push_str(",\"groupIndex\":");
    match candidate.slot.group_index {
        Some(group_index) => output.push_str(&group_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(
        candidate.slot.source_span.unit_start(),
        candidate.slot.source_span.unit_end(),
    ));
    output.push_str(",\"splitFromTextRun\":");
    output.push_str(&candidate.slot.split_from_text_run.to_string());
    output.push('}');
}

pub(super) fn push_fdm_text_mask_promotion_blocked_reasons_json(
    output: &mut String,
    cohort_best: Option<FdmTextMaskRightNeighborCandidate<'_>>,
    component_best: Option<FdmTextMaskRightNeighborCandidate<'_>>,
    cohort_component_agreement: bool,
    selected_slot: &ShanaiLanTextSlot,
    metrics: FdmTextMaskPreFragmentBridgeMetrics,
    row_anchor_ambiguous: bool,
) {
    let mut reasons = Vec::<&str>::new();
    if cohort_best.is_none() {
        reasons.push("fdm-text-mask-cohort-right-neighbor-missing");
    }
    if component_best.is_none() {
        reasons.push("fdm-text-mask-component-right-neighbor-missing");
    }
    if !cohort_component_agreement {
        reasons.push("fdm-text-mask-cohort-component-slot-disagreement");
    }
    if !selected_slot.split_from_text_run {
        reasons.push("document-text-slot-not-split-from-text-run");
    }
    if metrics.pre_fragment_unit_count == 0 {
        reasons.push("document-text-pre-fragment-empty");
    }
    if !metrics.source_bbox_within_pre_fragment_projection {
        reasons.push("fdm-bbox-outside-document-text-pre-fragment-projection");
    }
    if row_anchor_ambiguous {
        reasons.push("line-header-y-run-placement-semantics-unproven");
    }
    reasons.push("document-text-pre-fragment-fdm-mask-role-unproven");
    reasons.push("fdm-text-mask-to-document-text-baseline-transform-unproven");
    reasons.push("fdm-text-mask-promotion-cross-sample-support-missing");

    output.push('[');
    for (index, reason) in reasons.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(reason));
    }
    output.push(']');
}

pub(super) fn fdm_text_mask_right_neighbor_candidates_same_slot(
    left: FdmTextMaskRightNeighborCandidate<'_>,
    right: FdmTextMaskRightNeighborCandidate<'_>,
) -> bool {
    left.slot_index == right.slot_index
}

pub(super) fn push_fdm_text_mask_right_neighbor_text_slot_json(
    output: &mut String,
    source_bbox: Option<(f32, f32, f32, f32)>,
    text_projection: &ShanaiLanTextProjection,
    candidate_relation: &'static str,
) {
    let Some(source_bbox) = source_bbox else {
        output.push_str("null");
        return;
    };
    let Some((slot, bbox, gap_px, vertical_overlap_px, center_delta_y_px)) =
        fdm_text_mask_bbox_right_neighbor_text_slot(source_bbox, text_projection)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/DocumentText\",\"sourceBacked\":true,\"decoded\":false,\"candidateRelation\":");
    output.push_str(&json_string(candidate_relation));
    output.push_str(",\"text\":");
    output.push_str(&json_string(&slot.text));
    output.push_str(",\"bbox\":");
    push_bbox_tuple_json(output, bbox);
    output.push_str(",\"horizontalGapPx\":");
    output.push_str(&format!("{gap_px:.3}"));
    output.push_str(",\"verticalOverlapPx\":");
    output.push_str(&format!("{vertical_overlap_px:.3}"));
    output.push_str(",\"centerDeltaYPx\":");
    output.push_str(&format!("{center_delta_y_px:.3}"));
    output.push_str(",\"groupIndex\":");
    match slot.group_index {
        Some(group_index) => output.push_str(&group_index.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineOffsetUnits\":");
    output.push_str(&slot.line_offset_units.to_string());
    output.push_str(",\"leadingUnits\":");
    output.push_str(&slot.leading_units.to_string());
    output.push_str(",\"fragmentStartUnits\":");
    output.push_str(&slot.fragment_start_units.to_string());
    output.push_str(",\"sourceByteRange\":");
    output.push_str(&source_range_json(
        slot.source_span.byte_start(),
        slot.source_span.byte_end(),
    ));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(
        slot.source_span.unit_start(),
        slot.source_span.unit_end(),
    ));
    output.push_str(",\"leadingWhitespaceBridgeCandidate\":");
    push_fdm_text_mask_leading_whitespace_bridge_candidate_json(
        output,
        source_bbox,
        text_projection,
        slot,
    );
    output.push_str(",\"lineHeaderYPlacementCandidate\":");
    if slot.line_header_same_segment_group_run.is_some() {
        output.push_str("{\"renderPromotionBlockedReason\":\"line-header-y-run-placement-semantics-unproven\",\"renderPromotionBlockedDetail\":");
        output.push_str(&json_string(
            if slot
                .line_header_same_segment_group_run_distinct_text_group_count
                .is_some_and(|count| count > 1)
            {
                "same-segment-run-spans-multiple-visible-text-rows"
            } else {
                "line-header-y-run-transform-undecoded"
            },
        ));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"alignmentPromotionBlockedReason\":\"fdm-text-mask-to-document-text-baseline-transform-unproven\"}");
}

pub(super) fn fdm_text_mask_pre_fragment_bridge_metrics(
    source_bbox: (f32, f32, f32, f32),
    text_projection: &ShanaiLanTextProjection,
    slot: &ShanaiLanTextSlot,
) -> FdmTextMaskPreFragmentBridgeMetrics {
    let parent_span = &slot.fragment_context.parent_source_span;
    let pre_fragment_unit_count = slot
        .source_span
        .unit_start()
        .saturating_sub(parent_span.unit_start());
    let pre_fragment_grid_units = (slot.leading_units + slot.fragment_start_units) * 2;
    let pre_fragment_projected_width_px =
        pre_fragment_grid_units as f32 * text_projection.grid_unit_px;
    let line_start_x = slot.x - pre_fragment_projected_width_px;
    let text_start_x = slot.x;
    let source_right = source_bbox.0 + source_bbox.2;
    let source_bottom = source_bbox.1 + source_bbox.3;
    let source_begins_after_line_start = source_bbox.0 >= line_start_x - 0.5;
    let source_ends_before_text_start = source_right <= text_start_x + 0.5;
    let source_bbox_within_pre_fragment_projection =
        source_begins_after_line_start && source_ends_before_text_start;
    let text_baseline_y = shanai_lan_text_baseline_y(slot);

    FdmTextMaskPreFragmentBridgeMetrics {
        pre_fragment_unit_count,
        pre_fragment_grid_units,
        pre_fragment_projected_width_px,
        line_start_x,
        text_start_x,
        source_begins_after_line_start,
        source_ends_before_text_start,
        source_bbox_within_pre_fragment_projection,
        source_bbox_right_to_text_start_px: text_start_x - source_right,
        text_baseline_minus_source_bottom_px: text_baseline_y - source_bottom,
    }
}

pub(super) fn push_fdm_text_mask_leading_whitespace_bridge_candidate_json(
    output: &mut String,
    source_bbox: (f32, f32, f32, f32),
    text_projection: &ShanaiLanTextProjection,
    slot: &ShanaiLanTextSlot,
) {
    let parent_span = &slot.fragment_context.parent_source_span;
    let metrics = fdm_text_mask_pre_fragment_bridge_metrics(source_bbox, text_projection, slot);
    if metrics.pre_fragment_unit_count == 0
        && slot.leading_units == 0
        && slot.fragment_start_units == 0
    {
        output.push_str("null");
        return;
    }

    output.push_str("{\"source\":\"fdmTextMaskBbox+/DocumentText pre-fragment span\",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output
        .push_str(",\"candidateClass\":\"fdm-bbox-inside-document-text-pre-fragment-projection\"");
    output.push_str(",\"bridgeCandidate\":");
    output.push_str(
        &metrics
            .source_bbox_within_pre_fragment_projection
            .to_string(),
    );
    output.push_str(",\"parentTextRunSourceByteRange\":");
    output.push_str(&source_range_json(
        parent_span.byte_start(),
        parent_span.byte_end(),
    ));
    output.push_str(",\"parentTextRunSourceUnitRange\":");
    output.push_str(&source_range_json(
        parent_span.unit_start(),
        parent_span.unit_end(),
    ));
    output.push_str(",\"preFragmentSourceByteRange\":");
    output.push_str(&source_range_json(
        parent_span.byte_start(),
        slot.source_span.byte_start(),
    ));
    output.push_str(",\"preFragmentSourceUnitRange\":");
    output.push_str(&source_range_json(
        parent_span.unit_start(),
        slot.source_span.unit_start(),
    ));
    output.push_str(",\"preFragmentUnitCount\":");
    output.push_str(&metrics.pre_fragment_unit_count.to_string());
    output.push_str(",\"leadingDisplayUnits\":");
    output.push_str(&slot.leading_units.to_string());
    output.push_str(",\"fragmentStartUnits\":");
    output.push_str(&slot.fragment_start_units.to_string());
    output.push_str(",\"preFragmentProjectionGridUnits\":");
    output.push_str(&metrics.pre_fragment_grid_units.to_string());
    output.push_str(",\"preFragmentProjectedWidthPx\":");
    output.push_str(&format!("{:.3}", metrics.pre_fragment_projected_width_px));
    output.push_str(",\"lineStartX\":");
    output.push_str(&format!("{:.3}", metrics.line_start_x));
    output.push_str(",\"textStartX\":");
    output.push_str(&format!("{:.3}", metrics.text_start_x));
    output.push_str(",\"sourceBboxOffsetFromLineStartPx\":");
    output.push_str(&format!("{:.3}", source_bbox.0 - metrics.line_start_x));
    output.push_str(",\"sourceBboxEndOffsetFromLineStartPx\":");
    output.push_str(&format!(
        "{:.3}",
        source_bbox.0 + source_bbox.2 - metrics.line_start_x
    ));
    output.push_str(",\"sourceBboxWithinPreFragmentProjection\":");
    output.push_str(
        &metrics
            .source_bbox_within_pre_fragment_projection
            .to_string(),
    );
    output.push_str(",\"sourceBboxBeginsAfterLineStart\":");
    output.push_str(&metrics.source_begins_after_line_start.to_string());
    output.push_str(",\"sourceBboxEndsBeforeTextStart\":");
    output.push_str(&metrics.source_ends_before_text_start.to_string());
    output.push_str(",\"sourceBboxRightToTextStartGapPx\":");
    output.push_str(&format!(
        "{:.3}",
        metrics.source_bbox_right_to_text_start_px
    ));
    output.push_str(",\"textBaselineMinusSourceBottomPx\":");
    output.push_str(&format!(
        "{:.3}",
        metrics.text_baseline_minus_source_bottom_px
    ));
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"document-text-pre-fragment-fdm-mask-role-unproven\"}",
    );
}

pub(super) fn fdm_text_mask_source_transform_candidates<'a>(
    cohorts: &[FdmTextMaskCohortDiagnosticSummary],
    text_projection: &'a ShanaiLanTextProjection,
) -> Vec<FdmTextMaskSourceTransformCandidate<'a>> {
    if text_projection.grid_unit_px <= 0.0 {
        return Vec::new();
    }

    cohorts
        .iter()
        .filter_map(|cohort| {
            let component = cohort.top_text_like_component?;
            let projected_bbox = component.projected_bbox?;
            let source_bbox = component.source_bbox?;
            if projected_bbox.2 <= 0.0 {
                return None;
            }

            let component_candidates = fdm_text_mask_bbox_right_neighbor_text_slot_candidates(
                projected_bbox,
                text_projection,
            );
            let selected = component_candidates.first().copied()?;
            let cohort_component_agreement = cohort
                .projected_bbox
                .map(|bbox| {
                    fdm_text_mask_bbox_right_neighbor_text_slot_candidates(bbox, text_projection)
                })
                .and_then(|candidates| candidates.first().copied())
                .is_some_and(|cohort_candidate| {
                    fdm_text_mask_right_neighbor_candidates_same_slot(cohort_candidate, selected)
                });
            let metrics = fdm_text_mask_pre_fragment_bridge_metrics(
                projected_bbox,
                text_projection,
                selected.slot,
            );
            if metrics.pre_fragment_unit_count == 0 {
                return None;
            }

            let current_projection_grid_start =
                (projected_bbox.0 - metrics.line_start_x) / text_projection.grid_unit_px;
            let current_projection_grid_end = (projected_bbox.0 + projected_bbox.2
                - metrics.line_start_x)
                / text_projection.grid_unit_px;
            let current_projection_grid_span =
                current_projection_grid_end - current_projection_grid_start;
            if current_projection_grid_span <= 0.0 {
                return None;
            }

            let source_span_x = (source_bbox.2 - source_bbox.0).max(1) as f32;
            let source_units_per_text_grid_unit_x = source_span_x / current_projection_grid_span;
            let line_start_source_x = source_bbox.0 as f32
                - current_projection_grid_start * source_units_per_text_grid_unit_x;
            let text_start_source_x = line_start_source_x
                + metrics.pre_fragment_grid_units as f32 * source_units_per_text_grid_unit_x;
            let source_gap_to_text_start_x = text_start_source_x - source_bbox.2 as f32;

            [
                current_projection_grid_start,
                current_projection_grid_end,
                current_projection_grid_span,
                source_units_per_text_grid_unit_x,
                line_start_source_x,
                text_start_source_x,
                source_gap_to_text_start_x,
            ]
            .into_iter()
            .all(f32::is_finite)
            .then_some(FdmTextMaskSourceTransformCandidate {
                row_index: cohort.row_index,
                candidate_class: "top-text-like-component-to-document-text-pre-fragment",
                component_index: Some(component.component_index),
                slot_index: selected.slot_index,
                slot: selected.slot,
                source_bbox,
                projected_bbox,
                metrics,
                cohort_component_agreement,
                current_projection_grid_start,
                current_projection_grid_end,
                current_projection_grid_span,
                source_units_per_text_grid_unit_x,
                line_start_source_x,
                text_start_source_x,
                source_gap_to_text_start_x,
            })
        })
        .collect()
}

pub(super) fn fdm_text_mask_cohort_summaries(
    layout: PageLayout,
    diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
) -> Vec<FdmTextMaskCohortDiagnosticSummary> {
    let mut by_row = BTreeMap::<usize, FdmTextMaskCohortDiagnosticSummary>::new();
    let mut primitives_by_row =
        BTreeMap::<usize, Vec<FdmTextMaskPrimitiveDiagnosticSummary>>::new();
    for diagnostic in diagnostics.iter().copied() {
        if !fdm_text_mask_cohort_primitive_candidate(diagnostic.command) {
            continue;
        }
        let Some(bbox) = fdm_path_diagnostic_bbox(layout, diagnostic, extent) else {
            continue;
        };
        if bbox.2 <= 0.0 || bbox.3 <= 0.0 {
            continue;
        }
        let source_bbox =
            fdm_vector_command_source_bbox(diagnostic.command).map(normalize_fdm_bbox);
        let row = by_row
            .entry(diagnostic.entry.row_index())
            .or_insert_with(|| FdmTextMaskCohortDiagnosticSummary {
                row_index: diagnostic.entry.row_index(),
                ..Default::default()
            });
        row.primitive_count += 1;
        let black_fill = diagnostic
            .command
            .fill_color()
            .is_some_and(fdm_vector_color_is_black);
        let white_fill = diagnostic
            .command
            .fill_color()
            .is_some_and(fdm_vector_color_is_white);
        let counter_overlay = fdm_vector_filled_path_is_counter_overlay(diagnostic, diagnostics);
        if black_fill {
            row.black_fill_primitive_count += 1;
        }
        if white_fill {
            row.white_fill_primitive_count += 1;
        }
        if counter_overlay {
            row.counter_overlay_count += 1;
        }
        update_optional_usize_min_max(
            &mut row.command_index_min,
            &mut row.command_index_max,
            diagnostic.command.command_index(),
        );
        update_optional_usize_min_max(
            &mut row.relative_offset_min,
            &mut row.relative_offset_max,
            diagnostic.command.relative_offset(),
        );
        if let Some(source_bbox) = source_bbox {
            row.source_bbox = fdm_bbox_extent_union(row.source_bbox, source_bbox);
        }
        row.projected_bbox = bbox_tuple_union(row.projected_bbox, bbox);
        primitives_by_row
            .entry(diagnostic.entry.row_index())
            .or_default()
            .push(FdmTextMaskPrimitiveDiagnosticSummary {
                command_index: diagnostic.command.command_index(),
                relative_offset: diagnostic.command.relative_offset(),
                source_bbox,
                projected_bbox: bbox,
                black_fill,
                white_fill,
                counter_overlay,
            });
    }

    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let text_line_height_px = 12.0 * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR * scale_y;
    let mut cohorts = by_row
        .into_values()
        .filter_map(|mut cohort| {
            let components = primitives_by_row
                .get(&cohort.row_index)
                .map(|primitives| fdm_text_mask_component_summaries(primitives))
                .unwrap_or_default();
            cohort.component_count = components.len();
            cohort.top_text_like_component =
                fdm_text_mask_top_text_like_component(&components, text_line_height_px);
            (cohort.primitive_count >= FDM_TEXT_MASK_COHORT_MIN_PRIMITIVES
                && cohort.projected_bbox.is_some())
            .then_some(cohort)
        })
        .collect::<Vec<_>>();
    cohorts.sort_by(|left, right| {
        let left_bbox = left.projected_bbox.unwrap_or_default();
        let right_bbox = right.projected_bbox.unwrap_or_default();
        left_bbox
            .1
            .partial_cmp(&right_bbox.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left_bbox
                    .0
                    .partial_cmp(&right_bbox.0)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.row_index.cmp(&right.row_index))
    });
    cohorts.truncate(FDM_TEXT_MASK_COHORT_LIMIT);
    cohorts
}

pub(super) fn fdm_text_mask_component_summaries(
    primitives: &[FdmTextMaskPrimitiveDiagnosticSummary],
) -> Vec<FdmTextMaskComponentDiagnosticSummary> {
    let mut sorted = primitives.to_vec();
    sorted.sort_by(|left, right| {
        left.projected_bbox
            .1
            .partial_cmp(&right.projected_bbox.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left.projected_bbox
                    .0
                    .partial_cmp(&right.projected_bbox.0)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.relative_offset.cmp(&right.relative_offset))
    });

    let mut components = Vec::<FdmTextMaskComponentDiagnosticSummary>::new();
    for primitive in sorted {
        let component_index = components.iter().position(|component| {
            component.projected_bbox.is_some_and(|bbox| {
                fdm_text_mask_component_bboxes_touch(bbox, primitive.projected_bbox)
            })
        });
        match component_index {
            Some(index) => {
                update_fdm_text_mask_component_summary(&mut components[index], primitive);
                merge_touching_fdm_text_mask_components(&mut components);
            }
            None => {
                let mut component = FdmTextMaskComponentDiagnosticSummary::default();
                update_fdm_text_mask_component_summary(&mut component, primitive);
                components.push(component);
            }
        }
    }

    components.sort_by(|left, right| {
        let left_bbox = left.projected_bbox.unwrap_or_default();
        let right_bbox = right.projected_bbox.unwrap_or_default();
        left_bbox
            .1
            .partial_cmp(&right_bbox.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left_bbox
                    .0
                    .partial_cmp(&right_bbox.0)
                    .unwrap_or(Ordering::Equal)
            })
    });
    for (index, component) in components.iter_mut().enumerate() {
        component.component_index = index;
    }
    components
}

pub(super) fn update_fdm_text_mask_component_summary(
    component: &mut FdmTextMaskComponentDiagnosticSummary,
    primitive: FdmTextMaskPrimitiveDiagnosticSummary,
) {
    component.primitive_count += 1;
    if primitive.black_fill {
        component.black_fill_primitive_count += 1;
    }
    if primitive.white_fill {
        component.white_fill_primitive_count += 1;
    }
    if primitive.counter_overlay {
        component.counter_overlay_count += 1;
    }
    update_optional_usize_min_max(
        &mut component.command_index_min,
        &mut component.command_index_max,
        primitive.command_index,
    );
    update_optional_usize_min_max(
        &mut component.relative_offset_min,
        &mut component.relative_offset_max,
        primitive.relative_offset,
    );
    if let Some(source_bbox) = primitive.source_bbox {
        component.source_bbox = fdm_bbox_extent_union(component.source_bbox, source_bbox);
    }
    component.projected_bbox = bbox_tuple_union(component.projected_bbox, primitive.projected_bbox);
}

pub(super) fn merge_touching_fdm_text_mask_components(
    components: &mut Vec<FdmTextMaskComponentDiagnosticSummary>,
) {
    let mut index = 0usize;
    while index < components.len() {
        let mut merge_index = index + 1;
        while merge_index < components.len() {
            let Some(left_bbox) = components[index].projected_bbox else {
                break;
            };
            let Some(right_bbox) = components[merge_index].projected_bbox else {
                merge_index += 1;
                continue;
            };
            if !fdm_text_mask_component_bboxes_touch(left_bbox, right_bbox) {
                merge_index += 1;
                continue;
            }
            let right = components.remove(merge_index);
            merge_fdm_text_mask_component_summary(&mut components[index], right);
        }
        index += 1;
    }
}

pub(super) fn merge_fdm_text_mask_component_summary(
    target: &mut FdmTextMaskComponentDiagnosticSummary,
    source: FdmTextMaskComponentDiagnosticSummary,
) {
    target.primitive_count += source.primitive_count;
    target.black_fill_primitive_count += source.black_fill_primitive_count;
    target.white_fill_primitive_count += source.white_fill_primitive_count;
    target.counter_overlay_count += source.counter_overlay_count;
    if let Some(value) = source.command_index_min {
        update_optional_usize_min_max(
            &mut target.command_index_min,
            &mut target.command_index_max,
            value,
        );
    }
    if let Some(value) = source.command_index_max {
        update_optional_usize_min_max(
            &mut target.command_index_min,
            &mut target.command_index_max,
            value,
        );
    }
    if let Some(value) = source.relative_offset_min {
        update_optional_usize_min_max(
            &mut target.relative_offset_min,
            &mut target.relative_offset_max,
            value,
        );
    }
    if let Some(value) = source.relative_offset_max {
        update_optional_usize_min_max(
            &mut target.relative_offset_min,
            &mut target.relative_offset_max,
            value,
        );
    }
    if let Some(source_bbox) = source.projected_bbox {
        target.projected_bbox = bbox_tuple_union(target.projected_bbox, source_bbox);
    }
    if let Some(source_bbox) = source.source_bbox {
        target.source_bbox = fdm_bbox_extent_union(target.source_bbox, source_bbox);
    }
}

pub(super) fn fdm_text_mask_component_bboxes_touch(
    left: (f32, f32, f32, f32),
    right: (f32, f32, f32, f32),
) -> bool {
    let horizontal_gap = bbox_axis_gap(left.0, left.0 + left.2, right.0, right.0 + right.2);
    let vertical_overlap = (left.1 + left.3).min(right.1 + right.3) - left.1.max(right.1);
    let max_horizontal_gap = left.3.max(right.3) * 0.85;
    vertical_overlap > 0.0 && horizontal_gap <= max_horizontal_gap
}

pub(super) fn fdm_text_mask_top_text_like_component(
    components: &[FdmTextMaskComponentDiagnosticSummary],
    text_line_height_px: f32,
) -> Option<FdmTextMaskComponentDiagnosticSummary> {
    components
        .iter()
        .copied()
        .filter(|component| {
            let Some(bbox) = component.projected_bbox else {
                return false;
            };
            component.primitive_count >= FDM_TEXT_MASK_COMPONENT_MIN_PRIMITIVES
                && component.black_fill_primitive_count > 0
                && bbox.3 <= text_line_height_px * FDM_TEXT_MASK_COMPONENT_MAX_HEIGHT_LINE_FACTOR
        })
        .min_by(|left, right| {
            let left_bbox = left.projected_bbox.unwrap_or_default();
            let right_bbox = right.projected_bbox.unwrap_or_default();
            left_bbox
                .1
                .partial_cmp(&right_bbox.1)
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    left_bbox
                        .0
                        .partial_cmp(&right_bbox.0)
                        .unwrap_or(Ordering::Equal)
                })
        })
}

pub(super) fn fdm_text_mask_cohort_primitive_candidate(
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    command.ellipse().is_none()
        && fdm_vector_primitive_is_closed(command)
        && command.fill_color().is_some_and(|color| {
            fdm_vector_color_is_black(color) || fdm_vector_color_is_white(color)
        })
}

pub(super) fn fdm_text_mask_cohort_right_neighbor_text_slot<'a>(
    cohort: &FdmTextMaskCohortDiagnosticSummary,
    text_projection: &'a ShanaiLanTextProjection,
) -> Option<FdmTextMaskRightNeighborMatch<'a>> {
    fdm_text_mask_bbox_right_neighbor_text_slot(cohort.projected_bbox?, text_projection)
}

pub(super) fn fdm_text_mask_bbox_right_neighbor_text_slot(
    source_bbox: (f32, f32, f32, f32),
    text_projection: &ShanaiLanTextProjection,
) -> Option<FdmTextMaskRightNeighborMatch<'_>> {
    fdm_text_mask_bbox_right_neighbor_text_slot_candidates(source_bbox, text_projection)
        .into_iter()
        .next()
        .map(|candidate| {
            (
                candidate.slot,
                candidate.bbox,
                candidate.gap_px,
                candidate.vertical_overlap_px,
                candidate.center_delta_y_px,
            )
        })
}

pub(super) fn fdm_text_mask_bbox_right_neighbor_text_slot_candidates<'a>(
    source_bbox: (f32, f32, f32, f32),
    text_projection: &'a ShanaiLanTextProjection,
) -> Vec<FdmTextMaskRightNeighborCandidate<'a>> {
    let source_right = source_bbox.0 + source_bbox.2;
    let source_bottom = source_bbox.1 + source_bbox.3;
    let source_center_y = source_bbox.1 + source_bbox.3 * 0.5;
    let max_gap_px = text_projection.line_height_px * FDM_TEXT_MASK_RIGHT_NEIGHBOR_MAX_GAP_FACTOR;
    let mut candidates = text_projection
        .slots
        .iter()
        .enumerate()
        .filter_map(|(slot_index, slot)| {
            let bbox = shanai_lan_text_slot_bbox(slot);
            let gap_px = bbox.0 - source_right;
            if gap_px < -text_projection.line_height_px || gap_px > max_gap_px {
                return None;
            }
            let vertical_overlap_px =
                source_bottom.min(bbox.1 + bbox.3) - source_bbox.1.max(bbox.1);
            let text_center_y = bbox.1 + bbox.3 * 0.5;
            let center_delta_y_px = text_center_y - source_center_y;
            if vertical_overlap_px <= 0.0
                && center_delta_y_px.abs() > text_projection.line_height_px
            {
                return None;
            }
            Some(FdmTextMaskRightNeighborCandidate {
                slot_index,
                slot,
                bbox,
                gap_px,
                vertical_overlap_px: vertical_overlap_px.max(0.0),
                center_delta_y_px,
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.gap_px
            .abs()
            .partial_cmp(&right.gap_px.abs())
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left.center_delta_y_px
                    .abs()
                    .partial_cmp(&right.center_delta_y_px.abs())
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.slot_index.cmp(&right.slot_index))
    });
    candidates
}

pub(super) fn push_fdm_vector_command_provenance_json(
    output: &mut String,
    command: &ObjectFdmVectorCommandCandidate,
) {
    output.push_str(",\"sourceVectorRelativeOffset\":");
    push_optional_usize_json(output, command.source_vector_relative_offset());
    output.push_str(",\"sourceSegment\":");
    if let Some(source_segment) = command.source_segment() {
        push_object_fdm_vector_command_source_segment_json(output, source_segment);
    } else {
        output.push_str("null");
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct FdmPaintCoverage {
    pub(super) bbox_area_px: f32,
    pub(super) page_coverage_ratio: f32,
    pub(super) viewport_coverage_ratio: f32,
    pub(super) closed_primitive: bool,
    pub(super) fill_paint_present: bool,
    pub(super) page_fill_candidate: bool,
    pub(super) page_fill_candidate_basis: &'static str,
    pub(super) page_fill_candidate_reason: &'static str,
    pub(super) page_paint_source_evidence_proven: bool,
    pub(super) render_promotion_blocked_reason: &'static str,
}

pub(super) fn fdm_vector_paint_coverage(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    diagnostics: &[FdmCommandDiagnostic<'_>],
    bbox: (f32, f32, f32, f32),
) -> FdmPaintCoverage {
    let (_, _, width, height) = bbox;
    let page_coverage_ratio = projected_bbox_page_coverage_ratio(layout, width, height);
    let viewport_coverage_ratio = projected_bbox_viewport_coverage_ratio(layout, width, height);
    let closed_primitive = fdm_vector_primitive_is_closed(diagnostic.command);
    let fill_paint_present = if let Some(ellipse) = diagnostic.command.ellipse() {
        ellipse.color().is_some() && fdm_vector_ellipse_should_fill(ellipse)
    } else {
        fdm_vector_linear_gradient_colors(diagnostic.command).is_some()
            || fdm_vector_render_fill_color(diagnostic, diagnostics) != "none"
    };
    let large_span_filter_met = fdm_path_span_filter_blocks(layout, diagnostic.command, bbox);
    let page_fill_candidate = closed_primitive && fill_paint_present && large_span_filter_met;
    let page_fill_candidate_reason = if !closed_primitive {
        "open-primitive-not-page-fill"
    } else if !fill_paint_present {
        "no-fill-paint"
    } else if !large_span_filter_met {
        "large-span-filter-not-met"
    } else {
        "closed-fill-large-span-filter-met"
    };
    let page_paint_source_evidence_proven = false;
    let render_promotion_blocked_reason = if page_fill_candidate {
        "fdm-page-fill-source-evidence-unproven"
    } else {
        "not-page-fill-candidate"
    };
    FdmPaintCoverage {
        bbox_area_px: width.max(0.0) * height.max(0.0),
        page_coverage_ratio,
        viewport_coverage_ratio,
        closed_primitive,
        fill_paint_present,
        page_fill_candidate,
        page_fill_candidate_basis: "closed-fill-and-large-span-filter",
        page_fill_candidate_reason,
        page_paint_source_evidence_proven,
        render_promotion_blocked_reason,
    }
}

pub(super) fn fdm_page_paint_coverage_summary(
    layout: PageLayout,
    diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
) -> FdmPagePaintCoverageSummary {
    let mut summary = FdmPagePaintCoverageSummary::default();
    for diagnostic in diagnostics.iter().copied() {
        let Some(bbox) = fdm_path_unfiltered_bbox(layout, diagnostic, extent) else {
            continue;
        };
        let coverage = fdm_vector_paint_coverage(layout, diagnostic, diagnostics, bbox);
        summary.inspected_primitive_count += 1;
        if fdm_path_span_filter_blocks(layout, diagnostic.command, bbox) {
            summary.large_span_filtered_primitive_count += 1;
        } else {
            summary.rendered_primitive_count += 1;
        }
        if coverage.closed_primitive && coverage.fill_paint_present {
            summary.closed_fill_primitive_count += 1;
        }
        if coverage.page_fill_candidate {
            summary.page_fill_candidate_count += 1;
        }
        summary.max_page_coverage_ratio_ppm = summary
            .max_page_coverage_ratio_ppm
            .max(ratio_to_ppm(coverage.page_coverage_ratio));
        summary.max_viewport_coverage_ratio_ppm = summary
            .max_viewport_coverage_ratio_ppm
            .max(ratio_to_ppm(coverage.viewport_coverage_ratio));
    }
    summary
}

pub(super) fn push_fdm_paint_coverage_json(output: &mut String, coverage: FdmPaintCoverage) {
    output.push_str("{\"bboxAreaPx\":");
    output.push_str(&format!("{:.3}", coverage.bbox_area_px));
    output.push_str(",\"pageCoverageRatio\":");
    output.push_str(&format!("{:.6}", coverage.page_coverage_ratio));
    output.push_str(",\"viewportCoverageRatio\":");
    output.push_str(&format!("{:.6}", coverage.viewport_coverage_ratio));
    output.push_str(",\"pageFillSpanFilterMaxPageRatio\":");
    output.push_str(&format!("{FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO:.6}"));
    output.push_str(",\"closedPrimitive\":");
    output.push_str(if coverage.closed_primitive {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fillPaintPresent\":");
    output.push_str(if coverage.fill_paint_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageFillCandidate\":");
    output.push_str(if coverage.page_fill_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pageFillCandidateBasis\":");
    output.push_str(&json_string(coverage.page_fill_candidate_basis));
    output.push_str(",\"pageFillCandidateReason\":");
    output.push_str(&json_string(coverage.page_fill_candidate_reason));
    output.push_str(",\"paintPromotionGate\":{\"pagePaintSourceEvidenceProven\":");
    output.push_str(if coverage.page_paint_source_evidence_proven {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderable\":false,\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(coverage.render_promotion_blocked_reason));
    output.push('}');
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(coverage.render_promotion_blocked_reason));
    output.push_str(",\"decoded\":false}");
}

pub(super) fn push_fdm_page_paint_coverage_summary_json(
    output: &mut String,
    summary: FdmPagePaintCoverageSummary,
) {
    output.push_str("{\"basis\":\"fdmVectorPrimitivePaintCoverage\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(if summary.page_fill_candidate_count == 0 {
        "no-page-fill-candidates"
    } else {
        "page-background-paint-order-and-extent-unproven"
    }));
    output.push_str(",\"inspectedPrimitiveCount\":");
    output.push_str(&summary.inspected_primitive_count.to_string());
    output.push_str(",\"renderedPrimitiveCount\":");
    output.push_str(&summary.rendered_primitive_count.to_string());
    output.push_str(",\"largeSpanFilteredPrimitiveCount\":");
    output.push_str(&summary.large_span_filtered_primitive_count.to_string());
    output.push_str(",\"closedFillPrimitiveCount\":");
    output.push_str(&summary.closed_fill_primitive_count.to_string());
    output.push_str(",\"pageFillCandidateCount\":");
    output.push_str(&summary.page_fill_candidate_count.to_string());
    output.push_str(",\"maxPageCoverageRatio\":");
    push_ratio_ppm_json(output, summary.max_page_coverage_ratio_ppm);
    output.push_str(",\"maxViewportCoverageRatio\":");
    push_ratio_ppm_json(output, summary.max_viewport_coverage_ratio_ppm);
    output.push_str(",\"pageFillCandidateBasis\":\"closed-fill-and-large-span-filter\"");
    output.push_str(",\"pageFillSpanFilterMaxPageRatio\":");
    output.push_str(&format!("{FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO:.6}"));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_page_layer_fdm_connector_candidate_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    fdm_open_stroke_axis_rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let (x, y, width, height) = metric.projected_bbox;
    output.push_str("{\"type\":\"fdmConnectorCandidateDiagnostic\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":\"fdmVectorCommandConnectorCandidate\",\"projectionKind\":\"fdmOpenPathConnectorCandidateProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":\"connector-ownership-grouping-and-paint-order-unproven\"");
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(metric.basis));
    output.push_str(",\"sourcePath\":");
    output.push_str(&json_string(diagnostic.candidate.path()));
    output.push_str(",\"objectCandidateIndex\":");
    output.push_str(&diagnostic.candidate_index.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"parentCompoundCommand\":");
    push_fdm_connector_parent_compound_provenance_json(output, diagnostic);
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
    output.push_str(",\"styleWord\":");
    output.push_str(&diagnostic.command.style_word().to_string());
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!(
        "{:.3}",
        fdm_vector_stroke_width(diagnostic.command)
    ));
    output.push_str(",\"sourceEndpoints\":");
    push_fdm_connector_source_endpoints_json(output, metric);
    output.push_str(",\"projectedEndpoints\":");
    push_fdm_connector_projected_endpoints_json(output, metric);
    output.push_str(",\"projectedTextGrid\":");
    push_fdm_connector_projected_text_grid_json(output, layout, metric, line_rule_projection);
    output.push_str(",\"lineRuleAttachmentCandidates\":");
    push_fdm_connector_line_rule_attachment_candidates_json(
        output,
        layout,
        metric,
        line_rule_projection,
    );
    output.push_str(",\"lineRuleEndpointMatches\":");
    push_fdm_connector_line_rule_endpoint_matches_json(
        output,
        layout,
        metric,
        line_rule_projection,
    );
    output.push_str(",\"lineRuleEndpointMatchSummary\":");
    push_fdm_connector_line_rule_endpoint_match_summary_json(
        output,
        layout,
        metric,
        line_rule_projection,
    );
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleEndpointMatches\":");
    push_fdm_connector_open_stroke_axis_rule_endpoint_matches_json(
        output,
        layout,
        diagnostic,
        extent,
        metric,
        line_rule_projection,
        fdm_open_stroke_axis_rules,
    );
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleEndpointMatchSummary\":");
    push_fdm_connector_open_stroke_axis_rule_endpoint_match_summary_json(
        output,
        layout,
        diagnostic,
        metric,
        line_rule_projection,
        fdm_open_stroke_axis_rules,
    );
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleOwnerPromotionGate\":");
    push_fdm_connector_open_stroke_axis_rule_owner_promotion_gate_json(
        output,
        layout,
        diagnostic,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
        line_rule_projection,
        fdm_open_stroke_axis_rules,
    );
    output.push_str(",\"endpointOwnerCandidates\":");
    push_fdm_connector_endpoint_owner_candidates_json(
        output,
        layout,
        diagnostic,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );
    output.push_str(",\"endpointOwnerMatchSummary\":");
    push_fdm_connector_endpoint_owner_match_summary_json(
        output,
        layout,
        diagnostic,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );
    output.push_str(",\"sourceEndpointDistance\":");
    output.push_str(&format!("{:.3}", metric.source_endpoint_distance));
    output.push_str(",\"projectedEndpointDistance\":");
    output.push_str(&format!("{:.3}", metric.projected_endpoint_distance));
    output.push_str(",\"projectedSpan\":");
    output.push_str(&format!("{:.3}", metric.projected_span));
    output.push_str(",\"orientation\":");
    output.push_str(&json_string(metric.orientation));
    output.push_str(",\"pathPointCount\":");
    output.push_str(&diagnostic.command.path_points().len().to_string());
    output.push_str(",\"curveSegmentCount\":");
    output.push_str(&diagnostic.command.curve_segments().len().to_string());
    output.push_str(",\"sourcePathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
        push_object_fdm_index_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"projectionExtent\":{\"left\":");
    output.push_str(&extent.left.to_string());
    output.push_str(",\"top\":");
    output.push_str(&extent.top.to_string());
    output.push_str(",\"right\":");
    output.push_str(&extent.right.to_string());
    output.push_str(",\"bottom\":");
    output.push_str(&extent.bottom.to_string());
    output.push('}');
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(super) fn fdm_connector_order_trace_json(
    layout: PageLayout,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
    text_projection: Option<&ShanaiLanTextProjection>,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    fdm_open_stroke_axis_rules: &[FdmOpenStrokeAxisRule<'_>],
) -> Option<String> {
    let projection = line_rule_projection?;
    let mut selected = Vec::new();
    for diagnostic in primitive_diagnostics.iter().copied() {
        let Some(metric) = fdm_connector_candidate_metric(layout, diagnostic, extent) else {
            continue;
        };
        let Some(detail) = fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
            layout,
            diagnostic,
            metric,
            projection,
            fdm_open_stroke_axis_rules,
        ) else {
            continue;
        };
        if !detail.tight_dual_endpoint_match() || metric.orientation == "diagonal" {
            continue;
        }
        selected.push((diagnostic, metric, detail));
    }

    if selected.is_empty() {
        return None;
    }

    let mut output = format!(
        "{{\"type\":\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTrace\",\"bbox\":{{\"x\":0.0,\"y\":0.0,\"width\":{:.1},\"height\":{:.1}}}",
        layout.width_px(),
        layout.height_px()
    );
    output.push_str(",\"projectionKind\":\"fdmConnectorSourceOrderTrace\"");
    output.push_str(
        ",\"source\":\"FDMIndex+FDMVector+sameRowFdmOpenStrokeAxisRule+endpointOwnerMatch\"",
    );
    output.push_str(",\"basis\":\"sameRowFdmOpenStrokeAxisRule+tightDualEndpoint+nonDiagonalConnector+sourceOrderTrace\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true,\"sourceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":\"projected-endpoint-straight-line-paint-order-and-ownership-unproven\"");
    output.push_str(",\"selectionPredicate\":{\"requiresTightDualEndpointAxisRuleMatch\":true,\"excludesDiagonalConnectors\":true,\"rowHardcoded\":false}");
    output.push_str(",\"traceCount\":");
    output.push_str(&selected.len().to_string());
    output.push_str(",\"summary\":");
    push_fdm_connector_order_trace_summary_json(
        &mut output,
        fdm_connector_order_trace_summary(
            layout,
            &selected,
            primitive_diagnostics,
            extent,
            text_projection,
        ),
    );
    output.push_str(",\"traces\":[");
    for (index, (diagnostic, metric, detail)) in selected.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_connector_order_trace_connector_json(
            &mut output,
            layout,
            diagnostic,
            extent,
            metric,
            detail,
            primitive_diagnostics,
            text_projection,
            projection,
            fdm_open_stroke_axis_rules,
        );
    }
    output.push_str("]}");
    Some(output)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_fdm_connector_order_trace_connector_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
    projection: &ShanaiLanLineRuleProjection,
    fdm_open_stroke_axis_rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let owner_summary = fdm_connector_endpoint_owner_match_summary(
        layout,
        diagnostic,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );
    let viewport = fdm_projection_viewport(layout);
    let start_matches =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
            .map(|point| {
                fdm_connector_open_stroke_axis_rule_endpoint_matches(
                    diagnostic,
                    fdm_open_stroke_axis_rules,
                    point,
                )
            })
            .unwrap_or_default();
    let end_matches =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
            .map(|point| {
                fdm_connector_open_stroke_axis_rule_endpoint_matches(
                    diagnostic,
                    fdm_open_stroke_axis_rules,
                    point,
                )
            })
            .unwrap_or_default();

    output.push_str("{\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"fdmIndexRow\":");
    push_fdm_connector_order_trace_index_row_json(output, diagnostic);
    output.push_str(",\"imageBearingSegmentGate\":");
    push_fdm_connector_order_trace_image_bearing_gate_json(
        output,
        diagnostic,
        detail,
        owner_summary,
    );
    output.push_str(",\"connector\":");
    push_fdm_connector_order_trace_connector_command_json(output, diagnostic);
    output.push_str(",\"axisRuleMatchSummary\":{\"startMatchCount\":");
    output.push_str(&detail.summary.start_match_count.to_string());
    output.push_str(",\"endMatchCount\":");
    output.push_str(&detail.summary.end_match_count.to_string());
    output.push_str(",\"totalMatchCount\":");
    output.push_str(&detail.summary.total_match_count.to_string());
    output.push_str(",\"tightMatchCount\":");
    output.push_str(&detail.summary.tight_match_count.to_string());
    output.push_str(",\"tightDualEndpointMatch\":");
    output.push_str(if detail.tight_dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"axisRuleParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    );
    output.push('}');
    output.push_str(",\"endpointOwners\":");
    push_fdm_connector_order_trace_endpoint_owners_json(output, owner_summary);
    output.push_str(",\"axisRuleMatches\":{\"start\":");
    push_fdm_connector_order_trace_axis_match_array_json(output, &start_matches);
    output.push_str(",\"end\":");
    push_fdm_connector_order_trace_axis_match_array_json(output, &end_matches);
    output.push('}');
    output.push_str(",\"relations\":");
    push_fdm_connector_order_trace_relations_json(output, diagnostic, detail, owner_summary);
    output.push_str(",\"sourceOrderNodes\":");
    push_fdm_connector_order_trace_source_order_nodes_json(
        output,
        diagnostic,
        owner_summary,
        &start_matches,
        &end_matches,
    );
    output.push('}');
}

pub(super) fn fdm_connector_order_trace_summary(
    layout: PageLayout,
    selected: &[(
        FdmCommandDiagnostic<'_>,
        FdmConnectorCandidateMetric,
        FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
    )],
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
    text_projection: Option<&ShanaiLanTextProjection>,
) -> FdmConnectorOrderTraceSummary {
    let mut summary = FdmConnectorOrderTraceSummary {
        trace_count: selected.len(),
        ..Default::default()
    };

    for (diagnostic, metric, detail) in selected.iter().copied() {
        let owner_summary = fdm_connector_endpoint_owner_match_summary(
            layout,
            diagnostic,
            extent,
            metric,
            primitive_diagnostics,
            text_projection,
        );
        if diagnostic
            .command
            .source_segment()
            .map(|segment| segment.relative_offset() == diagnostic.entry.vector_offset())
            .unwrap_or(false)
        {
            summary.source_segment_matches_index_entry_count += 1;
        }
        if fdm_connector_command_matches_entry_connector_candidate(diagnostic) {
            summary.entry_connector_candidate_count += 1;
        }
        let image_bearing_segment = fdm_connector_image_bearing_segment_candidate(diagnostic);
        if image_bearing_segment {
            summary.image_bearing_segment_count += 1;
            if fdm_connector_segment_complete_image_payload_span_count(diagnostic) > 0 {
                summary.image_bearing_complete_payload_segment_count += 1;
            } else {
                summary.image_bearing_signature_without_payload_segment_count += 1;
            }
        }
        if owner_summary.parent_normalized_ordered_same_row_same_connector() {
            summary.parent_normalized_ordered_same_row_same_connector_count += 1;
        }

        let bbox_relation = fdm_connector_fdm_index_bbox_relation(diagnostic);
        match bbox_relation {
            "contained-in-fdm-index-bbox" => {
                summary.bbox_contained_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_bbox_contained_count += 1;
                }
            }
            "overlaps-fdm-index-bbox" => {
                summary.bbox_overlaps_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_bbox_overlaps_count += 1;
                }
            }
            "disjoint-from-fdm-index-bbox" => {
                summary.bbox_disjoint_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_bbox_disjoint_count += 1;
                }
            }
            _ => {
                summary.bbox_missing_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_bbox_missing_count += 1;
                }
            }
        }

        let connector_axis_rule_relation = fdm_connector_axis_rule_parent_span_relation(
            owner_summary.connector_parent_relative_offset,
            detail.axis_rule_match_parent_relative_offset_min,
            detail.axis_rule_match_parent_relative_offset_max,
        );
        match connector_axis_rule_relation {
            "connector-before-axis-rule-parent-span" => {
                summary.connector_before_axis_rule_parent_span_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_connector_before_axis_rule_parent_span_count += 1;
                }
            }
            "connector-between-axis-rule-parent-span" => {
                summary.connector_between_axis_rule_parent_span_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_connector_between_axis_rule_parent_span_count += 1;
                }
            }
            "connector-after-axis-rule-parent-span" => {
                summary.connector_after_axis_rule_parent_span_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_connector_after_axis_rule_parent_span_count += 1;
                }
            }
            _ => {
                summary.connector_axis_rule_parent_span_missing_count += 1;
                if image_bearing_segment {
                    summary.image_bearing_connector_axis_rule_parent_span_missing_count += 1;
                }
            }
        }
        if image_bearing_segment {
            match fdm_connector_relation_to_segment_image_signature_range(diagnostic) {
                "connector-before-segment-image-signature-range" => {
                    summary.image_bearing_connector_before_segment_signature_range_count += 1;
                }
                "connector-inside-segment-image-signature-range" => {
                    summary.image_bearing_connector_inside_segment_signature_range_count += 1;
                }
                "connector-after-segment-image-signature-range" => {
                    summary.image_bearing_connector_after_segment_signature_range_count += 1;
                }
                _ => {
                    summary.image_bearing_connector_segment_signature_range_missing_count += 1;
                }
            }
        }

        match fdm_owner_axis_rule_parent_span_relation(owner_summary, detail) {
            "owner-parent-span-before-axis-rule-parent-span" => {
                summary.owner_parent_span_before_axis_rule_parent_span_count += 1;
            }
            "owner-parent-span-after-axis-rule-parent-span" => {
                summary.owner_parent_span_after_axis_rule_parent_span_count += 1;
            }
            "owner-parent-span-inside-axis-rule-parent-span" => {
                summary.owner_parent_span_inside_axis_rule_parent_span_count += 1;
            }
            "axis-rule-parent-span-inside-owner-parent-span" => {
                summary.axis_rule_parent_span_inside_owner_parent_span_count += 1;
            }
            "owner-parent-span-overlaps-axis-rule-parent-span" => {
                summary.owner_parent_span_overlaps_axis_rule_parent_span_count += 1;
            }
            _ => summary.owner_axis_rule_parent_span_missing_count += 1,
        }
    }

    summary
}

pub(super) fn push_fdm_connector_order_trace_summary_json(
    output: &mut String,
    summary: FdmConnectorOrderTraceSummary,
) {
    output
        .push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTrace+relationCounts\"");
    output.push_str(",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true");
    output.push_str(",\"promotionReady\":");
    output.push_str(if summary.promotion_ready() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"readinessBlockedReason\":");
    output.push_str(&json_string(summary.readiness_blocked_reason()));
    output.push_str(",\"traceCount\":");
    output.push_str(&summary.trace_count.to_string());
    output.push_str(",\"sourceSegmentMatchesIndexEntryCount\":");
    output.push_str(&summary.source_segment_matches_index_entry_count.to_string());
    output.push_str(",\"entryConnectorCandidateCount\":");
    output.push_str(&summary.entry_connector_candidate_count.to_string());
    output.push_str(",\"imageBearingSegmentCount\":");
    output.push_str(&summary.image_bearing_segment_count.to_string());
    output.push_str(",\"imageBearingCompletePayloadSegmentCount\":");
    output.push_str(
        &summary
            .image_bearing_complete_payload_segment_count
            .to_string(),
    );
    output.push_str(",\"imageBearingSignatureWithoutPayloadSegmentCount\":");
    output.push_str(
        &summary
            .image_bearing_signature_without_payload_segment_count
            .to_string(),
    );
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnectorCount\":");
    output.push_str(
        &summary
            .parent_normalized_ordered_same_row_same_connector_count
            .to_string(),
    );
    output.push_str(",\"bboxRelationCounts\":{");
    output.push_str("\"contained\":");
    output.push_str(&summary.bbox_contained_count.to_string());
    output.push_str(",\"overlaps\":");
    output.push_str(&summary.bbox_overlaps_count.to_string());
    output.push_str(",\"disjoint\":");
    output.push_str(&summary.bbox_disjoint_count.to_string());
    output.push_str(",\"missing\":");
    output.push_str(&summary.bbox_missing_count.to_string());
    output.push('}');
    output.push_str(",\"imageBearingBboxRelationCounts\":{");
    output.push_str("\"contained\":");
    output.push_str(&summary.image_bearing_bbox_contained_count.to_string());
    output.push_str(",\"overlaps\":");
    output.push_str(&summary.image_bearing_bbox_overlaps_count.to_string());
    output.push_str(",\"disjoint\":");
    output.push_str(&summary.image_bearing_bbox_disjoint_count.to_string());
    output.push_str(",\"missing\":");
    output.push_str(&summary.image_bearing_bbox_missing_count.to_string());
    output.push('}');
    output.push_str(",\"connectorVsAxisRuleParentSpanCounts\":{");
    output.push_str("\"before\":");
    output.push_str(
        &summary
            .connector_before_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"between\":");
    output.push_str(
        &summary
            .connector_between_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"after\":");
    output.push_str(
        &summary
            .connector_after_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"missing\":");
    output.push_str(
        &summary
            .connector_axis_rule_parent_span_missing_count
            .to_string(),
    );
    output.push('}');
    output.push_str(",\"imageBearingConnectorVsAxisRuleParentSpanCounts\":{");
    output.push_str("\"before\":");
    output.push_str(
        &summary
            .image_bearing_connector_before_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"between\":");
    output.push_str(
        &summary
            .image_bearing_connector_between_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"after\":");
    output.push_str(
        &summary
            .image_bearing_connector_after_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"missing\":");
    output.push_str(
        &summary
            .image_bearing_connector_axis_rule_parent_span_missing_count
            .to_string(),
    );
    output.push('}');
    output.push_str(",\"imageBearingConnectorVsSegmentImageSignatureRangeCounts\":{");
    output.push_str("\"before\":");
    output.push_str(
        &summary
            .image_bearing_connector_before_segment_signature_range_count
            .to_string(),
    );
    output.push_str(",\"inside\":");
    output.push_str(
        &summary
            .image_bearing_connector_inside_segment_signature_range_count
            .to_string(),
    );
    output.push_str(",\"after\":");
    output.push_str(
        &summary
            .image_bearing_connector_after_segment_signature_range_count
            .to_string(),
    );
    output.push_str(",\"missing\":");
    output.push_str(
        &summary
            .image_bearing_connector_segment_signature_range_missing_count
            .to_string(),
    );
    output.push('}');
    output.push_str(",\"ownerVsAxisRuleParentSpanCounts\":{");
    output.push_str("\"before\":");
    output.push_str(
        &summary
            .owner_parent_span_before_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"after\":");
    output.push_str(
        &summary
            .owner_parent_span_after_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"ownerInsideAxis\":");
    output.push_str(
        &summary
            .owner_parent_span_inside_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"axisInsideOwner\":");
    output.push_str(
        &summary
            .axis_rule_parent_span_inside_owner_parent_span_count
            .to_string(),
    );
    output.push_str(",\"overlaps\":");
    output.push_str(
        &summary
            .owner_parent_span_overlaps_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"missing\":");
    output.push_str(
        &summary
            .owner_axis_rule_parent_span_missing_count
            .to_string(),
    );
    output.push_str("}}");
}

pub(super) fn push_fdm_connector_order_trace_index_row_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&diagnostic.entry.row_index().to_string());
    output.push_str(",\"indexOffset\":");
    output.push_str(&diagnostic.entry.index_offset().to_string());
    output.push_str(",\"vectorOffset\":");
    output.push_str(&diagnostic.entry.vector_offset().to_string());
    output.push_str(",\"vectorLength\":");
    output.push_str(&diagnostic.entry.vector_len().to_string());
    output.push_str(",\"validVectorOffset\":");
    output.push_str(if diagnostic.entry.valid_vector_offset() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"kindHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", diagnostic.entry.kind())));
    output.push_str(",\"normalizedBbox\":");
    push_fdm_normalized_bbox_json(output, normalize_fdm_bbox(diagnostic.entry.bbox()));
    output.push_str(",\"axisPairBbox\":");
    push_fdm_normalized_bbox_json(
        output,
        normalize_fdm_index_entry_bbox(diagnostic.entry.bbox()),
    );
    output.push_str(",\"imageSignatureCount\":");
    output.push_str(&diagnostic.entry.image_signature_hits().len().to_string());
    output.push_str(",\"segmentImageSignatureCount\":");
    output.push_str(
        &diagnostic
            .entry
            .segment_image_signature_hits()
            .len()
            .to_string(),
    );
    output.push_str(",\"imageBearingSegmentCandidate\":");
    output.push_str(
        if !diagnostic.entry.image_signature_hits().is_empty()
            || !diagnostic.entry.segment_image_signature_hits().is_empty()
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"vectorCommandCount\":");
    output.push_str(&diagnostic.entry.vector_commands().len().to_string());
    output.push_str(",\"entryConnectorCandidateCount\":");
    output.push_str(&diagnostic.entry.connector_candidates().len().to_string());
    output.push('}');
}

pub(super) fn push_fdm_connector_order_trace_image_bearing_gate_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
) {
    let image_bearing = fdm_connector_image_bearing_segment_candidate(diagnostic);
    output.push_str(
        "{\"basis\":\"FDMIndex.imageSignature+FDMVector.connectorBbox+sameRowAxisRuleParentSpan\"",
    );
    output.push_str(",\"source\":\"FDMIndex.segmentImageSignatures+FDMVector.commandSourceBbox\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        fdm_connector_image_bearing_gate_blocked_reason(diagnostic),
    ));
    output.push_str(",\"imageBearingSegmentCandidate\":");
    output.push_str(if image_bearing { "true" } else { "false" });
    output.push_str(",\"connectorParent\":{\"commandIndex\":");
    output.push_str(
        &fdm_command_parent_command_index(diagnostic.command.command_index()).to_string(),
    );
    output.push_str(",\"relativeOffset\":");
    push_option_usize_json(
        output,
        fdm_command_normalized_parent_relative_offset(diagnostic),
    );
    output.push('}');
    output.push_str(",\"axisRuleParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    );
    output.push_str(",\"endpointOwnerParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        owner_summary
            .start_nearest_fdm_owner_parent_relative_offset
            .min(owner_summary.end_nearest_fdm_owner_parent_relative_offset),
        owner_summary
            .start_nearest_fdm_owner_parent_relative_offset
            .max(owner_summary.end_nearest_fdm_owner_parent_relative_offset),
    );
    output.push_str(",\"endpointOwnerParentRelations\":{\"connectorVsOwnerParentSpan\":");
    output.push_str(&json_string(
        owner_summary.owner_parent_source_order_relation(),
    ));
    output.push_str(",\"ownerParentSpanVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_owner_axis_rule_parent_span_relation(
        owner_summary,
        detail,
    )));
    output.push('}');
    output.push_str(",\"endpointOwnerParentRelationToAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_owner_axis_rule_parent_span_relation(
        owner_summary,
        detail,
    )));
    output.push_str(",\"imageSignatures\":");
    push_object_image_signature_hits_json(output, diagnostic.entry.image_signature_hits());
    output.push_str(",\"segmentImageSignatures\":");
    push_object_image_signature_hits_json(output, diagnostic.entry.segment_image_signature_hits());
    output.push_str(",\"segmentImageSignatureCommandContexts\":");
    push_fdm_connector_segment_image_signature_command_contexts_json(output, diagnostic);
    output.push_str(",\"imageSignatureOffsetRange\":");
    push_image_signature_offset_range_json(output, diagnostic.entry.image_signature_hits());
    output.push_str(",\"segmentImageSignatureOffsetRange\":");
    push_image_signature_offset_range_json(output, diagnostic.entry.segment_image_signature_hits());
    output.push_str(",\"completeImagePayloadSpanCount\":");
    output.push_str(&fdm_connector_complete_image_payload_span_count(diagnostic).to_string());
    output.push_str(",\"segmentCompleteImagePayloadSpanCount\":");
    output
        .push_str(&fdm_connector_segment_complete_image_payload_span_count(diagnostic).to_string());
    output.push_str(",\"payloadExtractionStatus\":");
    output.push_str(&json_string(fdm_connector_image_payload_extraction_status(
        diagnostic,
    )));
    output.push_str(",\"connectorVsSegmentImageSignatureRange\":");
    output.push_str(&json_string(
        fdm_connector_relation_to_segment_image_signature_range(diagnostic),
    ));
    output.push_str(",\"connectorVsImageSignatureRange\":");
    output.push_str(&json_string(
        fdm_connector_relation_to_image_signature_range(diagnostic),
    ));
    let nearest_segment_signature = fdm_connector_nearest_segment_image_signature(diagnostic);
    output.push_str(",\"nearestSegmentImageSignatureOffset\":");
    push_option_usize_json(output, nearest_segment_signature.map(|(offset, _)| offset));
    output.push_str(",\"nearestSegmentImageSignatureDistance\":");
    push_option_usize_json(
        output,
        nearest_segment_signature.map(|(_, distance)| distance),
    );
    let nearest_signature = fdm_connector_nearest_image_signature(diagnostic);
    output.push_str(",\"nearestImageSignatureOffset\":");
    push_option_usize_json(output, nearest_signature.map(|(offset, _)| offset));
    output.push_str(",\"nearestImageSignatureDistance\":");
    push_option_usize_json(output, nearest_signature.map(|(_, distance)| distance));
    output.push_str(",\"bboxRelationToFdmIndex\":");
    output.push_str(&json_string(fdm_connector_fdm_index_bbox_relation(
        diagnostic,
    )));
    output.push_str(",\"connectorVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_connector_axis_rule_parent_span_relation(
        owner_summary.connector_parent_relative_offset,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    )));
    output.push_str(",\"ownerParentSpanVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_owner_axis_rule_parent_span_relation(
        owner_summary,
        detail,
    )));
    output.push('}');
}

pub(super) fn fdm_connector_image_bearing_segment_candidate(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> bool {
    !diagnostic.entry.image_signature_hits().is_empty()
        || !diagnostic.entry.segment_image_signature_hits().is_empty()
}

pub(super) fn push_fdm_connector_segment_image_signature_command_contexts_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
) {
    output.push('[');
    for (index, hit) in diagnostic
        .entry
        .segment_image_signature_hits()
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        let containing_commands = diagnostic
            .entry
            .vector_commands()
            .iter()
            .filter(|command| {
                fdm_offset_inside_command_record(
                    hit.offset(),
                    command.relative_offset(),
                    command.record_len(),
                )
            })
            .collect::<Vec<_>>();
        output.push_str("{\"kind\":");
        output.push_str(&json_string(hit.kind()));
        output.push_str(",\"offset\":");
        output.push_str(&hit.offset().to_string());
        output.push_str(",\"relationToTraceConnectorCommand\":");
        output.push_str(&json_string(fdm_offset_relation_to_command_record(
            hit.offset(),
            diagnostic.command.relative_offset(),
            diagnostic.command.record_len(),
        )));
        output.push_str(",\"containingCommandCount\":");
        output.push_str(&containing_commands.len().to_string());
        output.push_str(",\"containingCommands\":[");
        for (command_index, command) in containing_commands.iter().enumerate() {
            if command_index > 0 {
                output.push(',');
            }
            output.push_str("{\"commandIndex\":");
            output.push_str(&command.command_index().to_string());
            output.push_str(",\"relativeOffset\":");
            output.push_str(&command.relative_offset().to_string());
            output.push_str(",\"recordEnd\":");
            output.push_str(
                &command
                    .relative_offset()
                    .saturating_add(command.record_len())
                    .to_string(),
            );
            output.push_str(",\"recordLength\":");
            output.push_str(&command.record_len().to_string());
            output.push_str(",\"declaredRecordLength\":");
            output.push_str(&command.declared_record_len().to_string());
            output.push_str(",\"offsetInCommand\":");
            output.push_str(
                &hit.offset()
                    .saturating_sub(command.relative_offset())
                    .to_string(),
            );
            output.push_str(",\"markerHex\":");
            output.push_str(&json_string(&hex_bytes(command.marker())));
            output.push_str(",\"primitiveKind\":");
            output.push_str(&json_string(fdm_vector_primitive_kind(command)));
            output.push_str(",\"styleWordHex\":");
            output.push_str(&json_string(&format!("0x{:04x}", command.style_word())));
            output.push_str(",\"syntheticNestedCommand\":");
            output.push_str(
                if fdm_command_index_is_synthetic_nested(command.command_index()) {
                    "true"
                } else {
                    "false"
                },
            );
            output.push_str(",\"sameAsTraceConnector\":");
            output.push_str(
                if command.command_index() == diagnostic.command.command_index()
                    && command.relative_offset() == diagnostic.command.relative_offset()
                {
                    "true"
                } else {
                    "false"
                },
            );
            output.push('}');
        }
        output.push_str("]}");
    }
    output.push(']');
}

pub(super) fn fdm_offset_inside_command_record(offset: usize, start: usize, len: usize) -> bool {
    start <= offset && offset < start.saturating_add(len)
}

pub(super) fn fdm_offset_relation_to_command_record(
    offset: usize,
    start: usize,
    len: usize,
) -> &'static str {
    if offset < start {
        "before-command-record"
    } else if offset >= start.saturating_add(len) {
        "after-command-record"
    } else {
        "inside-command-record"
    }
}

pub(super) fn fdm_connector_complete_image_payload_span_count(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> usize {
    diagnostic
        .candidate
        .image_payload_spans()
        .iter()
        .filter(|span| span.complete())
        .count()
}

pub(super) fn fdm_connector_segment_complete_image_payload_span_count(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> usize {
    diagnostic
        .candidate
        .image_payload_spans()
        .iter()
        .filter(|span| {
            span.complete()
                && span.signature_offset() >= diagnostic.entry.vector_offset()
                && span.signature_offset() < diagnostic.entry.next_vector_offset()
        })
        .count()
}

pub(super) fn fdm_connector_image_payload_extraction_status(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    if !fdm_connector_image_bearing_segment_candidate(diagnostic) {
        "no-image-signature"
    } else if fdm_connector_segment_complete_image_payload_span_count(diagnostic) > 0 {
        "complete-payload-in-fdm-index-segment"
    } else if fdm_connector_complete_image_payload_span_count(diagnostic) > 0 {
        "complete-payload-elsewhere-in-vector-stream"
    } else {
        "signature-without-complete-payload"
    }
}

pub(super) fn fdm_connector_relation_to_segment_image_signature_range(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    let Some((min_offset, max_offset)) =
        image_signature_offset_range(diagnostic.entry.segment_image_signature_hits())
    else {
        return "no-segment-image-signature-range";
    };
    let offset = diagnostic.command.relative_offset();
    if offset < min_offset {
        "connector-before-segment-image-signature-range"
    } else if offset > max_offset {
        "connector-after-segment-image-signature-range"
    } else {
        "connector-inside-segment-image-signature-range"
    }
}

pub(super) fn fdm_connector_relation_to_image_signature_range(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    let Some((min_offset, max_offset)) =
        image_signature_offset_range(diagnostic.entry.image_signature_hits())
    else {
        return "no-image-signature-range";
    };
    let Some(offset) = diagnostic.command.source_vector_relative_offset() else {
        return "connector-source-vector-offset-missing";
    };
    if offset < min_offset {
        "connector-before-image-signature-range"
    } else if offset > max_offset {
        "connector-after-image-signature-range"
    } else {
        "connector-inside-image-signature-range"
    }
}

pub(super) fn fdm_connector_nearest_segment_image_signature(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> Option<(usize, usize)> {
    nearest_image_signature_offset(
        diagnostic.entry.segment_image_signature_hits(),
        diagnostic.command.relative_offset(),
    )
}

pub(super) fn fdm_connector_nearest_image_signature(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> Option<(usize, usize)> {
    nearest_image_signature_offset(
        diagnostic.entry.image_signature_hits(),
        diagnostic.command.source_vector_relative_offset()?,
    )
}

pub(super) fn fdm_connector_image_bearing_gate_blocked_reason(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    if !fdm_connector_image_bearing_segment_candidate(diagnostic) {
        return "not-image-bearing-segment";
    }
    if fdm_connector_segment_complete_image_payload_span_count(diagnostic) == 0 {
        return "image-signature-without-complete-payload-role-unproven";
    }
    match fdm_connector_fdm_index_bbox_relation(diagnostic) {
        "contained-in-fdm-index-bbox" => "image-bearing-contained-internal-stroke-role-unproven",
        "overlaps-fdm-index-bbox" => "image-bearing-overlapping-object-boundary-role-unproven",
        "disjoint-from-fdm-index-bbox" => "image-bearing-disjoint-external-connector-role-unproven",
        _ => "image-bearing-connector-source-bbox-missing",
    }
}

pub(super) fn push_fdm_connector_order_trace_connector_command_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
) {
    output.push_str("{\"commandIndex\":");
    output.push_str(&diagnostic.command.command_index().to_string());
    output.push_str(",\"parentCommandIndex\":");
    output.push_str(
        &fdm_command_parent_command_index(diagnostic.command.command_index()).to_string(),
    );
    output.push_str(",\"syntheticNestedCommand\":");
    output.push_str(
        if fdm_command_index_is_synthetic_nested(diagnostic.command.command_index()) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"relativeOffset\":");
    output.push_str(&diagnostic.command.relative_offset().to_string());
    output.push_str(",\"parentRelativeOffset\":");
    push_option_usize_json(
        output,
        fdm_command_normalized_parent_relative_offset(diagnostic),
    );
    push_fdm_vector_command_provenance_json(output, diagnostic.command);
    output.push_str(",\"sourceSegmentMatchesIndexEntry\":");
    match diagnostic.command.source_segment() {
        Some(source_segment) => output.push_str(
            if source_segment.relative_offset() == diagnostic.entry.vector_offset() {
                "true"
            } else {
                "false"
            },
        ),
        None => output.push_str("null"),
    }
    output.push_str(",\"entryConnectorCandidate\":");
    output.push_str(
        if fdm_connector_command_matches_entry_connector_candidate(diagnostic) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"markerHex\":");
    output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    output.push_str(",\"primitiveKind\":");
    output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
    output.push_str(",\"styleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    output.push_str(",\"parentCompoundCommand\":");
    push_fdm_connector_parent_compound_provenance_json(output, diagnostic);
    output.push_str(",\"sourcePathBbox\":");
    if let Some(bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
        push_object_fdm_index_bbox_json(output, bbox);
        output.push_str(",\"normalizedSourcePathBbox\":");
        push_fdm_normalized_bbox_json(output, normalize_fdm_bbox(bbox));
    } else {
        output.push_str("null,\"normalizedSourcePathBbox\":null");
    }
    output.push('}');
}

pub(super) fn fdm_connector_command_matches_entry_connector_candidate(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> bool {
    diagnostic
        .entry
        .connector_candidates()
        .iter()
        .any(|candidate| {
            candidate.command_index() == diagnostic.command.command_index()
                && candidate.relative_offset() == diagnostic.command.relative_offset()
        })
}

pub(super) fn push_fdm_connector_order_trace_endpoint_owners_json(
    output: &mut String,
    summary: FdmConnectorEndpointOwnerMatchSummary,
) {
    output.push_str("{\"start\":");
    push_fdm_connector_order_trace_owner_json(
        output,
        summary.start_nearest_fdm_owner_row_index,
        summary.start_nearest_fdm_owner_command_index,
        summary.start_nearest_fdm_owner_parent_command_index,
        summary.start_nearest_fdm_owner_synthetic_nested_command,
        summary.start_nearest_fdm_owner_relative_offset,
        summary.start_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"end\":");
    push_fdm_connector_order_trace_owner_json(
        output,
        summary.end_nearest_fdm_owner_row_index,
        summary.end_nearest_fdm_owner_command_index,
        summary.end_nearest_fdm_owner_parent_command_index,
        summary.end_nearest_fdm_owner_synthetic_nested_command,
        summary.end_nearest_fdm_owner_relative_offset,
        summary.end_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"dualEndpointOwnerCandidate\":");
    output.push_str(if summary.dual_endpoint_owner_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nearestFdmOwnerRowsMatch\":");
    output.push_str(if summary.nearest_fdm_owner_rows_match {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nearestFdmOwnerRowMatchesConnectorRow\":");
    output.push_str(if summary.nearest_fdm_owner_row_matches_connector_row {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(super) fn push_fdm_connector_order_trace_owner_json(
    output: &mut String,
    row_index: Option<usize>,
    command_index: Option<usize>,
    parent_command_index: Option<usize>,
    synthetic_nested_command: bool,
    relative_offset: Option<usize>,
    parent_relative_offset: Option<usize>,
) {
    let (Some(row_index), Some(command_index), Some(parent_command_index)) =
        (row_index, command_index, parent_command_index)
    else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"rowIndex\":");
    output.push_str(&row_index.to_string());
    output.push_str(",\"commandIndex\":");
    output.push_str(&command_index.to_string());
    output.push_str(",\"parentCommandIndex\":");
    output.push_str(&parent_command_index.to_string());
    output.push_str(",\"syntheticNestedCommand\":");
    output.push_str(if synthetic_nested_command {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"relativeOffset\":");
    push_option_usize_json(output, relative_offset);
    output.push_str(",\"parentRelativeOffset\":");
    push_option_usize_json(output, parent_relative_offset);
    output.push('}');
}

pub(super) fn push_fdm_connector_order_trace_axis_match_array_json(
    output: &mut String,
    matches: &[(
        usize,
        &FdmOpenStrokeAxisRule<'_>,
        FdmConnectorLineRuleDistance,
        &'static str,
    )],
) {
    output.push('[');
    for (index, (axis_rule_index, rule, distance, tier)) in matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"axisRuleIndex\":");
        output.push_str(&axis_rule_index.to_string());
        output.push_str(",\"ruleCommandIndex\":");
        output.push_str(&rule.diagnostic.command.command_index().to_string());
        output.push_str(",\"ruleParentCommandIndex\":");
        output.push_str(
            &fdm_command_parent_command_index(rule.diagnostic.command.command_index()).to_string(),
        );
        output.push_str(",\"ruleRelativeOffset\":");
        output.push_str(&rule.diagnostic.command.relative_offset().to_string());
        output.push_str(",\"ruleParentRelativeOffset\":");
        push_option_usize_json(
            output,
            fdm_command_normalized_parent_relative_offset(rule.diagnostic),
        );
        output.push_str(",\"ruleMarkerHex\":");
        output.push_str(&json_string(&hex_bytes(rule.diagnostic.command.marker())));
        output.push_str(",\"ruleStyleWordHex\":");
        output.push_str(&json_string(&format!(
            "0x{:04x}",
            rule.diagnostic.command.style_word()
        )));
        output.push_str(",\"orientation\":");
        output.push_str(&json_string(rule.orientation));
        output.push_str(",\"tier\":");
        output.push_str(&json_string(tier));
        output.push_str(",\"distanceGrid\":");
        output.push_str(&format!("{:.3}", distance.distance_grid));
        output.push_str(",\"axisDelta\":");
        output.push_str(&format!("{:.3}", distance.axis_delta));
        output.push_str(",\"inlineDelta\":");
        output.push_str(&format!("{:.3}", distance.inline_delta));
        output.push('}');
    }
    output.push(']');
}

pub(super) fn push_fdm_connector_order_trace_relations_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
) {
    output.push_str("{\"connectorVsOwnerParentSpan\":");
    output.push_str(&json_string(
        owner_summary.owner_parent_source_order_relation(),
    ));
    output.push_str(",\"connectorVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_connector_axis_rule_parent_span_relation(
        owner_summary.connector_parent_relative_offset,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    )));
    output.push_str(",\"ownerParentSpanVsAxisRuleParentSpan\":");
    output.push_str(&json_string(fdm_owner_axis_rule_parent_span_relation(
        owner_summary,
        detail,
    )));
    output.push_str(",\"bboxRelationToFdmIndex\":");
    output.push_str(&json_string(fdm_connector_fdm_index_bbox_relation(
        diagnostic,
    )));
    output.push_str(",\"entryConnectorCandidate\":");
    output.push_str(
        if fdm_connector_command_matches_entry_connector_candidate(diagnostic) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnector\":");
    output.push_str(
        if owner_summary.parent_normalized_ordered_same_row_same_connector() {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(super) fn fdm_connector_axis_rule_parent_span_relation(
    connector_parent_relative_offset: Option<usize>,
    axis_rule_min: Option<usize>,
    axis_rule_max: Option<usize>,
) -> &'static str {
    let (Some(connector), Some(axis_min), Some(axis_max)) = (
        connector_parent_relative_offset,
        axis_rule_min,
        axis_rule_max,
    ) else {
        return "connector-or-axis-rule-parent-offset-missing";
    };
    if connector < axis_min {
        "connector-before-axis-rule-parent-span"
    } else if connector > axis_max {
        "connector-after-axis-rule-parent-span"
    } else {
        "connector-between-axis-rule-parent-span"
    }
}

pub(super) fn fdm_owner_axis_rule_parent_span_relation(
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
) -> &'static str {
    let (Some(start_owner), Some(end_owner), Some(axis_min), Some(axis_max)) = (
        owner_summary.start_nearest_fdm_owner_parent_relative_offset,
        owner_summary.end_nearest_fdm_owner_parent_relative_offset,
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    ) else {
        return "owner-or-axis-rule-parent-span-missing";
    };
    let owner_min = start_owner.min(end_owner);
    let owner_max = start_owner.max(end_owner);
    if owner_max < axis_min {
        "owner-parent-span-before-axis-rule-parent-span"
    } else if owner_min > axis_max {
        "owner-parent-span-after-axis-rule-parent-span"
    } else if axis_min <= owner_min && owner_max <= axis_max {
        "owner-parent-span-inside-axis-rule-parent-span"
    } else if owner_min <= axis_min && axis_max <= owner_max {
        "axis-rule-parent-span-inside-owner-parent-span"
    } else {
        "owner-parent-span-overlaps-axis-rule-parent-span"
    }
}

pub(super) fn fdm_connector_fdm_index_bbox_relation(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> &'static str {
    let Some(connector_bbox) =
        fdm_vector_command_source_bbox(diagnostic.command).map(normalize_fdm_bbox)
    else {
        return "connector-source-bbox-missing";
    };
    let index_bbox = normalize_fdm_index_entry_bbox(diagnostic.entry.bbox());
    if fdm_bbox_contains(index_bbox, connector_bbox) {
        "contained-in-fdm-index-bbox"
    } else if fdm_bbox_intersects(index_bbox, connector_bbox) {
        "overlaps-fdm-index-bbox"
    } else {
        "disjoint-from-fdm-index-bbox"
    }
}

pub(super) fn push_fdm_connector_order_trace_source_order_nodes_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
    start_matches: &[(
        usize,
        &FdmOpenStrokeAxisRule<'_>,
        FdmConnectorLineRuleDistance,
        &'static str,
    )],
    end_matches: &[(
        usize,
        &FdmOpenStrokeAxisRule<'_>,
        FdmConnectorLineRuleDistance,
        &'static str,
    )],
) {
    let mut nodes = Vec::new();
    nodes.push(fdm_connector_order_trace_node_from_diagnostic(
        "connector",
        None,
        diagnostic,
        10,
    ));
    if let Some(node) = fdm_connector_order_trace_node_from_owner_summary(
        "nearestFdmOwner",
        "start",
        owner_summary.start_nearest_fdm_owner_row_index,
        owner_summary.start_nearest_fdm_owner_command_index,
        owner_summary.start_nearest_fdm_owner_parent_command_index,
        owner_summary.start_nearest_fdm_owner_synthetic_nested_command,
        owner_summary.start_nearest_fdm_owner_relative_offset,
        owner_summary.start_nearest_fdm_owner_parent_relative_offset,
        0,
    ) {
        nodes.push(node);
    }
    if let Some(node) = fdm_connector_order_trace_node_from_owner_summary(
        "nearestFdmOwner",
        "end",
        owner_summary.end_nearest_fdm_owner_row_index,
        owner_summary.end_nearest_fdm_owner_command_index,
        owner_summary.end_nearest_fdm_owner_parent_command_index,
        owner_summary.end_nearest_fdm_owner_synthetic_nested_command,
        owner_summary.end_nearest_fdm_owner_relative_offset,
        owner_summary.end_nearest_fdm_owner_parent_relative_offset,
        1,
    ) {
        nodes.push(node);
    }
    for (_, rule, _, _) in start_matches {
        nodes.push(fdm_connector_order_trace_node_from_diagnostic(
            "axisRule",
            Some("start"),
            rule.diagnostic,
            20,
        ));
    }
    for (_, rule, _, _) in end_matches {
        nodes.push(fdm_connector_order_trace_node_from_diagnostic(
            "axisRule",
            Some("end"),
            rule.diagnostic,
            21,
        ));
    }
    nodes.sort_by(|left, right| {
        left.parent_relative_offset
            .unwrap_or(usize::MAX)
            .cmp(&right.parent_relative_offset.unwrap_or(usize::MAX))
            .then_with(|| {
                left.relative_offset
                    .unwrap_or(usize::MAX)
                    .cmp(&right.relative_offset.unwrap_or(usize::MAX))
            })
            .then_with(|| left.rank.cmp(&right.rank))
    });
    output.push('[');
    for (index, node) in nodes.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&node.json);
    }
    output.push(']');
}

pub(super) fn fdm_connector_order_trace_node_from_diagnostic(
    role: &'static str,
    endpoint: Option<&'static str>,
    diagnostic: FdmCommandDiagnostic<'_>,
    rank: usize,
) -> FdmConnectorOrderTraceNodeJson {
    let parent_relative_offset = fdm_command_normalized_parent_relative_offset(diagnostic);
    let relative_offset = Some(diagnostic.command.relative_offset());
    let mut json = String::new();
    json.push_str("{\"role\":");
    json.push_str(&json_string(role));
    json.push_str(",\"endpoint\":");
    match endpoint {
        Some(endpoint) => json.push_str(&json_string(endpoint)),
        None => json.push_str("null"),
    }
    json.push_str(",\"rowIndex\":");
    json.push_str(&diagnostic.entry.row_index().to_string());
    json.push_str(",\"commandIndex\":");
    json.push_str(&diagnostic.command.command_index().to_string());
    json.push_str(",\"parentCommandIndex\":");
    json.push_str(
        &fdm_command_parent_command_index(diagnostic.command.command_index()).to_string(),
    );
    json.push_str(",\"syntheticNestedCommand\":");
    json.push_str(
        if fdm_command_index_is_synthetic_nested(diagnostic.command.command_index()) {
            "true"
        } else {
            "false"
        },
    );
    json.push_str(",\"relativeOffset\":");
    json.push_str(&diagnostic.command.relative_offset().to_string());
    json.push_str(",\"parentRelativeOffset\":");
    push_option_usize_json(&mut json, parent_relative_offset);
    json.push_str(",\"markerHex\":");
    json.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
    json.push_str(",\"styleWordHex\":");
    json.push_str(&json_string(&format!(
        "0x{:04x}",
        diagnostic.command.style_word()
    )));
    json.push('}');
    FdmConnectorOrderTraceNodeJson {
        parent_relative_offset,
        relative_offset,
        rank,
        json,
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn fdm_connector_order_trace_node_from_owner_summary(
    role: &'static str,
    endpoint: &'static str,
    row_index: Option<usize>,
    command_index: Option<usize>,
    parent_command_index: Option<usize>,
    synthetic_nested_command: bool,
    relative_offset: Option<usize>,
    parent_relative_offset: Option<usize>,
    rank: usize,
) -> Option<FdmConnectorOrderTraceNodeJson> {
    let (Some(row_index), Some(command_index), Some(parent_command_index)) =
        (row_index, command_index, parent_command_index)
    else {
        return None;
    };
    let mut json = String::new();
    json.push_str("{\"role\":");
    json.push_str(&json_string(role));
    json.push_str(",\"endpoint\":");
    json.push_str(&json_string(endpoint));
    json.push_str(",\"rowIndex\":");
    json.push_str(&row_index.to_string());
    json.push_str(",\"commandIndex\":");
    json.push_str(&command_index.to_string());
    json.push_str(",\"parentCommandIndex\":");
    json.push_str(&parent_command_index.to_string());
    json.push_str(",\"syntheticNestedCommand\":");
    json.push_str(if synthetic_nested_command {
        "true"
    } else {
        "false"
    });
    json.push_str(",\"relativeOffset\":");
    push_option_usize_json(&mut json, relative_offset);
    json.push_str(",\"parentRelativeOffset\":");
    push_option_usize_json(&mut json, parent_relative_offset);
    json.push('}');
    Some(FdmConnectorOrderTraceNodeJson {
        parent_relative_offset,
        relative_offset,
        rank,
        json,
    })
}

pub(super) fn push_fdm_connector_parent_compound_provenance_json(
    output: &mut String,
    diagnostic: FdmCommandDiagnostic<'_>,
) {
    let Some(provenance) = fdm_connector_parent_compound_provenance(diagnostic) else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"basis\":\"synthetic-nested-command-index+relative-offset\"");
    output.push_str(",\"sourceBacked\":true,\"decoded\":false");
    output.push_str(",\"parentCommandIndex\":");
    output.push_str(&provenance.parent.command_index().to_string());
    output.push_str(",\"parentRelativeOffset\":");
    output.push_str(&provenance.parent.relative_offset().to_string());
    output.push_str(",\"parentRecordLength\":");
    output.push_str(&provenance.parent.record_len().to_string());
    output.push_str(",\"parentDeclaredRecordLength\":");
    output.push_str(&provenance.parent.declared_record_len().to_string());
    output.push_str(",\"parentMarkerHex\":");
    output.push_str(&json_string(&hex_bytes(provenance.parent.marker())));
    output.push_str(",\"parentStyleWord\":");
    output.push_str(&provenance.parent.style_word().to_string());
    output.push_str(",\"parentStyleWordHex\":");
    output.push_str(&json_string(&format!(
        "0x{:04x}",
        provenance.parent.style_word()
    )));
    output.push_str(",\"parentCompoundChildOffsets\":");
    push_u16_array_json(output, provenance.parent.compound_child_offsets());
    output.push_str(",\"childOffsetInParent\":");
    output.push_str(&provenance.child_offset_in_parent.to_string());
    output.push_str(",\"childOffsetTableIndex\":");
    push_option_usize_json(output, provenance.child_offset_table_index);
    output.push_str(",\"childOffsetTableMatched\":");
    output.push_str(if provenance.child_offset_table_index.is_some() {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(super) fn push_page_layer_fdm_connector_graph_diagnostic_summary_json(
    output: &mut String,
    layout: PageLayout,
    summary: FdmConnectorGraphDiagnosticSummary,
) {
    output.push_str("{\"type\":\"fdmConnectorGraphDiagnosticSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(
        ",\"source\":\"fdmVectorCommandConnectorCandidate+documentTextLineRuleProjection\"",
    );
    output.push_str(",\"projectionKind\":\"fdmConnectorGraphDiagnosticSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(summary.render_promotion_blocked_reason()));
    output.push_str(",\"pagePaintCoverageSummary\":");
    push_fdm_page_paint_coverage_summary_json(output, summary.page_paint_coverage_summary);
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleConnectorOrderTraceSummary\":");
    push_fdm_connector_order_trace_summary_json(
        output,
        summary.same_row_axis_rule_connector_order_trace_summary,
    );
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"lineRuleProjectionCount\":");
    output.push_str(&summary.line_rule_projection_count.to_string());
    output.push_str(",\"fdmOpenStrokeAxisRuleProjectionCount\":");
    output.push_str(
        &summary
            .fdm_open_stroke_axis_rule_projection_count
            .to_string(),
    );
    output.push_str(",\"connectorEndpointProbeCount\":");
    output.push_str(&summary.connector_endpoint_probe_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&summary.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&summary.matched_connector_count.to_string());
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"startEndpointLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .start_endpoint_line_rule_match_connector_count
            .to_string(),
    );
    output.push_str(",\"endEndpointLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .end_endpoint_line_rule_match_connector_count
            .to_string(),
    );
    output.push_str(",\"startOnlyLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .start_only_line_rule_match_connector_count
            .to_string(),
    );
    output.push_str(",\"endOnlyLineRuleMatchConnectorCount\":");
    output.push_str(&summary.end_only_line_rule_match_connector_count.to_string());
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&summary.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&summary.nearby_endpoint_match_count.to_string());
    output.push_str(",\"noThresholdedLineRuleEndpointMatchConnectorCount\":");
    output.push_str(
        &summary
            .no_thresholded_line_rule_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"singleOrMissingEndpointLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .single_or_missing_endpoint_line_rule_match_connector_count
            .to_string(),
    );
    output.push_str(",\"connectorOwnershipAndPaintOrderUnprovenConnectorCount\":");
    output.push_str(
        &summary
            .connector_ownership_and_paint_order_unproven_connector_count
            .to_string(),
    );
    output.push_str(",\"endpointOwnerCandidateConnectorCount\":");
    output.push_str(&summary.endpoint_owner_candidate_connector_count.to_string());
    output.push_str(",\"endpointOwnerProbeCount\":");
    output.push_str(&summary.endpoint_owner_probe_count.to_string());
    output.push_str(",\"totalEndpointOwnerCandidateCount\":");
    output.push_str(&summary.total_endpoint_owner_candidate_count.to_string());
    output.push_str(",\"withinProbeEndpointOwnerCandidateCount\":");
    output.push_str(
        &summary
            .within_probe_endpoint_owner_candidate_count
            .to_string(),
    );
    output.push_str(",\"fdmPrimitiveEndpointOwnerCandidateCount\":");
    output.push_str(
        &summary
            .fdm_primitive_endpoint_owner_candidate_count
            .to_string(),
    );
    output.push_str(",\"documentTextSlotEndpointOwnerCandidateCount\":");
    output.push_str(
        &summary
            .document_text_slot_endpoint_owner_candidate_count
            .to_string(),
    );
    output.push_str(",\"startEndpointOwnerWithinProbeConnectorCount\":");
    output.push_str(
        &summary
            .start_endpoint_owner_within_probe_connector_count
            .to_string(),
    );
    output.push_str(",\"endEndpointOwnerWithinProbeConnectorCount\":");
    output.push_str(
        &summary
            .end_endpoint_owner_within_probe_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointOwnerWithinProbeConnectorCount\":");
    output.push_str(
        &summary
            .dual_endpoint_owner_within_probe_connector_count
            .to_string(),
    );
    output.push_str(",\"ownerProvenConnectorCount\":");
    output.push_str(&summary.owner_proven_connector_count.to_string());
    output.push_str(",\"dualEndpointNearestFdmOwnerSameRowConnectorCount\":");
    output.push_str(
        &summary
            .dual_endpoint_nearest_fdm_owner_same_row_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointNearestFdmOwnerRowMismatchConnectorCount\":");
    output.push_str(
        &summary
            .dual_endpoint_nearest_fdm_owner_row_mismatch_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointNearestFdmOwnerSameConnectorRowCount\":");
    output.push_str(
        &summary
            .dual_endpoint_nearest_fdm_owner_same_connector_row_count
            .to_string(),
    );
    output.push_str(",\"connectorCommandBetweenNearestFdmOwnerCommandsCount\":");
    output.push_str(
        &summary
            .connector_command_between_nearest_fdm_owner_commands_count
            .to_string(),
    );
    output.push_str(",\"connectorCommandBeforeNearestFdmOwnerCommandsCount\":");
    output.push_str(
        &summary
            .connector_command_before_nearest_fdm_owner_commands_count
            .to_string(),
    );
    output.push_str(",\"connectorCommandAfterNearestFdmOwnerCommandsCount\":");
    output.push_str(
        &summary
            .connector_command_after_nearest_fdm_owner_commands_count
            .to_string(),
    );
    output.push_str(",\"orderedSameRowSameConnectorCount\":");
    output.push_str(&summary.ordered_same_row_same_connector_count.to_string());
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnectorCount\":");
    output.push_str(
        &summary
            .parent_normalized_ordered_same_row_same_connector_count
            .to_string(),
    );
    output.push_str(",\"missingEndpointOwnerCandidateConnectorCount\":");
    output.push_str(
        &summary
            .missing_endpoint_owner_candidate_connector_count
            .to_string(),
    );
    output.push_str(",\"nearestOwnerRowMismatchConnectorCount\":");
    output.push_str(
        &summary
            .nearest_owner_row_mismatch_connector_count
            .to_string(),
    );
    output.push_str(",\"ownerRowCandidateUnprovenConnectorCount\":");
    output.push_str(
        &summary
            .owner_row_candidate_unproven_connector_count
            .to_string(),
    );
    output.push_str(",\"ownerGroupingProvenConnectorCount\":");
    output.push_str(&summary.owner_grouping_proven_connector_count.to_string());
    output.push_str(",\"lineRuleEndpointMatchProvenanceSummaries\":[");
    push_fdm_connector_rule_set_match_summary_json(
        output,
        "allDocumentTextLineRules",
        None,
        summary.all_line_rule_match_summary(),
    );
    output.push(',');
    push_fdm_connector_rule_set_match_summary_json(
        output,
        "skippedInlineLineHeaderOnly",
        Some("skippedInlineLineHeader"),
        summary.skipped_inline_line_rule_match_summary,
    );
    output.push(',');
    push_fdm_connector_rule_set_match_summary_json(
        output,
        "verticalAnchorRunFromLineHeadersOnly",
        Some("verticalAnchorRunFromLineHeaders"),
        summary.vertical_anchor_line_rule_match_summary,
    );
    output.push(',');
    push_fdm_connector_rule_set_match_summary_json(
        output,
        "sameRowFdmOpenStrokeAxisRules",
        Some("fdmOpenStrokeAxisRule"),
        summary.fdm_open_stroke_axis_rule_match_summary,
    );
    output.push(']');
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleRowCohorts\":");
    push_fdm_open_stroke_axis_rule_row_cohorts_json(output, summary);
    output.push_str(",\"sameRowFdmOpenStrokeAxisRuleOwnerPromotionGateSummary\":");
    push_fdm_open_stroke_axis_rule_owner_promotion_gate_summary_json(
        output,
        summary.fdm_open_stroke_axis_rule_owner_promotion_gate_summary,
    );
    output.push_str(",\"ownerRowCohortEndpointMatchSummaries\":[");
    push_fdm_connector_owner_row_cohort_summary_json(
        output,
        "orderedSameRowSameConnector",
        summary.ordered_owner_row_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_row_cohort_summary_json(
        output,
        "notOrderedSameRowSameConnector",
        summary.non_ordered_owner_row_match_summary,
    );
    output.push(']');
    output.push_str(",\"parentNormalizedOwnerRowCohortEndpointMatchSummaries\":[");
    push_fdm_connector_owner_row_cohort_summary_json(
        output,
        "parentNormalizedOrderedSameRowSameConnector",
        summary.parent_normalized_ordered_owner_row_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_row_cohort_summary_json(
        output,
        "notParentNormalizedOrderedSameRowSameConnector",
        summary.parent_normalized_non_ordered_owner_row_match_summary,
    );
    output.push(']');
    output.push_str(",\"ownerCommandRelationEndpointMatchSummaries\":[");
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_relation_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_relation_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "nearest-owner-row-not-connector-row",
        summary.nearest_owner_row_not_connector_row_relation_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-mixed-command-namespace",
        summary.same_row_mixed_command_namespace_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-before-owner-command-span",
        summary.same_row_before_owner_command_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-between-owner-command-span",
        summary.same_row_between_owner_command_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-after-owner-command-span",
        summary.same_row_after_owner_command_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-command-relation-unclassified",
        summary.same_row_owner_command_relation_unclassified_match_summary,
    );
    output.push(']');
    output.push_str(",\"ownerSourceOrderRelationEndpointMatchSummaries\":[");
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_source_order_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_source_order_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "nearest-owner-row-not-connector-row",
        summary.nearest_owner_row_not_connector_row_source_order_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-before-owner-relative-offset-span",
        summary.same_row_before_owner_relative_offset_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-between-owner-relative-offset-span",
        summary.same_row_between_owner_relative_offset_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-after-owner-relative-offset-span",
        summary.same_row_after_owner_relative_offset_span_match_summary,
    );
    output.push(',');
    push_fdm_connector_owner_command_relation_summary_json(
        output,
        "same-row-relative-offset-relation-unclassified",
        summary.same_row_relative_offset_relation_unclassified_match_summary,
    );
    output.push(']');
    output.push_str(",\"ownerGroupingPromotionBlockedReasonCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_candidate_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "owner-row-candidate-unproven",
        summary.owner_row_candidate_unproven_connector_count,
    );
    output.push(']');
    output.push_str(",\"graphPromotionBlockedReasonCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "no-thresholded-line-rule-endpoint-match",
        summary.no_thresholded_line_rule_endpoint_match_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "single-or-missing-endpoint-line-rule-match",
        summary.single_or_missing_endpoint_line_rule_match_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "connector-ownership-and-paint-order-unproven",
        summary.connector_ownership_and_paint_order_unproven_connector_count,
    );
    output.push_str("],\"dominantMatchedConnectorRow\":");
    push_fdm_connector_dominant_matched_connector_row_json(output, summary);
    output.push_str(",\"endpointMatchThresholds\":{\"basis\":\"documentTextLineHeaderGrid\"");
    output.push_str(",\"spanOverflowProbeUnits\":");
    output.push_str(&format!(
        "{:.3}",
        FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS
    ));
    output.push_str(",\"tightPerpendicularProbeUnits\":");
    output.push_str(&format!(
        "{:.3}",
        FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS
    ));
    output.push_str(",\"nearbyPerpendicularProbeUnits\":");
    output.push_str(&format!(
        "{:.3}",
        FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS
    ));
    output.push_str(",\"attachmentProven\":false}");
    output.push_str(",\"projectionViewport\":");
    push_fdm_projection_viewport_json(output, layout);
    output.push('}');
}

pub(super) fn push_fdm_connector_rule_set_match_summary_json(
    output: &mut String,
    rule_set: &str,
    candidate_source: Option<&str>,
    summary: FdmConnectorRuleSetMatchDiagnosticSummary,
) {
    output.push_str("{\"ruleSet\":");
    output.push_str(&json_string(rule_set));
    output.push_str(",\"candidateSource\":");
    match candidate_source {
        Some(candidate_source) => output.push_str(&json_string(candidate_source)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineRuleProjectionCount\":");
    output.push_str(&summary.line_rule_projection_count.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"connectorEndpointProbeCount\":");
    output.push_str(&summary.connector_endpoint_probe_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&summary.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&summary.matched_connector_count.to_string());
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&summary.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&summary.nearby_endpoint_match_count.to_string());
    output.push_str(",\"graphPromotionBlockedReasonCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "no-thresholded-line-rule-endpoint-match",
        summary.no_thresholded_line_rule_endpoint_match_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "single-or-missing-endpoint-line-rule-match",
        summary.single_or_missing_endpoint_line_rule_match_connector_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "connector-ownership-and-paint-order-unproven",
        summary.connector_ownership_and_paint_order_unproven_connector_count,
    );
    output.push_str("]}");
}

pub(super) fn push_fdm_open_stroke_axis_rule_row_cohorts_json(
    output: &mut String,
    summary: FdmConnectorGraphDiagnosticSummary,
) {
    let row_cohorts = summary
        .fdm_open_stroke_axis_rule_row_cohorts
        .iter()
        .take(summary.fdm_open_stroke_axis_rule_row_cohort_count)
        .copied()
        .collect::<Vec<_>>();
    let tight_non_diagonal_dual_candidate_count = row_cohorts
        .iter()
        .map(|row| row.non_diagonal_tight_dual_endpoint_match_connector_count())
        .sum::<usize>();
    let tight_non_diagonal_dual_row_cohort_count = row_cohorts
        .iter()
        .filter(|row| row.non_diagonal_tight_dual_endpoint_match_connector_count() > 0)
        .count();
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+connectorRowIndex\"");
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"",
    );
    output.push_str(",\"rowCohortLimit\":");
    output.push_str(&FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT.to_string());
    output.push_str(",\"rowCohortCount\":");
    output.push_str(
        &summary
            .fdm_open_stroke_axis_rule_row_cohort_count
            .to_string(),
    );
    output.push_str(",\"renderReadinessPredicate\":");
    push_fdm_open_stroke_axis_rule_render_readiness_predicate_json(
        output,
        tight_non_diagonal_dual_candidate_count,
        tight_non_diagonal_dual_row_cohort_count,
    );
    output.push_str(",\"rowCohorts\":[");
    for (index, row) in row_cohorts.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_open_stroke_axis_rule_row_cohort_json(output, row);
    }
    output.push_str("]}");
}

pub(super) fn push_fdm_open_stroke_axis_rule_render_readiness_predicate_json(
    output: &mut String,
    candidate_count: usize,
    row_cohort_count: usize,
) {
    output.push_str(
        "{\"basis\":\"sameRowFdmOpenStrokeAxisRule+tightDualEndpoint+nonDiagonalConnector\"",
    );
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive+fdmConnectorEndpointOwnerMatch\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"requiresTightDualEndpointMatch\":true");
    output.push_str(",\"excludesDiagonalConnectors\":true");
    output.push_str(",\"requiresDualEndpointOwnerCandidate\":true");
    output.push_str(",\"requiresNearestFdmOwnerRowsMatch\":true");
    output.push_str(",\"requiresNearestFdmOwnerRowMatchesConnectorRow\":true");
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidate_count.to_string());
    output.push_str(",\"rowCohortCount\":");
    output.push_str(&row_cohort_count.to_string());
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"}",
    );
}

pub(super) fn push_fdm_open_stroke_axis_rule_row_cohort_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&row.connector_candidate_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&row.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&row.matched_connector_count.to_string());
    output.push_str(",\"fdmIndexSegmentGate\":");
    push_fdm_open_stroke_axis_rule_index_segment_gate_json(output, row);
    output.push_str(",\"fdmIndexConnectorCompositionGate\":");
    push_fdm_open_stroke_axis_rule_index_connector_composition_gate_json(output, row);
    output.push_str(",\"fdmIndexBboxRelationGate\":");
    push_fdm_open_stroke_axis_rule_index_bbox_relation_gate_json(output, row);
    output.push_str(",\"axisRuleSourceOrderGate\":");
    push_fdm_open_stroke_axis_rule_source_order_gate_json(output, row);
    output.push_str(",\"matchedProjectedBboxUnion\":");
    push_optional_bbox_milli_json(
        output,
        row.matched_projected_bbox_x_min_milli,
        row.matched_projected_bbox_y_min_milli,
        row.matched_projected_bbox_x_max_milli,
        row.matched_projected_bbox_y_max_milli,
    );
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&row.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"dualEndpointProjectedBboxUnion\":");
    push_optional_bbox_milli_json(
        output,
        row.dual_projected_bbox_x_min_milli,
        row.dual_projected_bbox_y_min_milli,
        row.dual_projected_bbox_x_max_milli,
        row.dual_projected_bbox_y_max_milli,
    );
    output.push_str(",\"nonDiagonalDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.non_diagonal_dual_endpoint_match_connector_count()
            .to_string(),
    );
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&row.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&row.nearby_endpoint_match_count.to_string());
    output.push_str(",\"tightDualEndpointMatchConnectorCount\":");
    output.push_str(&row.tight_dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"nonDiagonalTightDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.non_diagonal_tight_dual_endpoint_match_connector_count()
            .to_string(),
    );
    output.push_str(",\"tightNonDiagonalDualEndpointProjectedBboxUnion\":");
    push_optional_bbox_milli_json(
        output,
        row.tight_non_diagonal_dual_projected_bbox_x_min_milli,
        row.tight_non_diagonal_dual_projected_bbox_y_min_milli,
        row.tight_non_diagonal_dual_projected_bbox_x_max_milli,
        row.tight_non_diagonal_dual_projected_bbox_y_max_milli,
    );
    output.push_str(",\"horizontalDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.horizontal_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"verticalDualEndpointMatchConnectorCount\":");
    output.push_str(&row.vertical_dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"diagonalDualEndpointMatchConnectorCount\":");
    output.push_str(&row.diagonal_dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"horizontalTightDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.horizontal_tight_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"verticalTightDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.vertical_tight_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"diagonalTightDualEndpointMatchConnectorCount\":");
    output.push_str(
        &row.diagonal_tight_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"matchedConnectorMarkerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(
        output,
        row.matched_connector_marker_style_profile,
    );
    output.push_str(",\"dualConnectorMarkerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(output, row.dual_connector_marker_style_profile);
    output.push_str(",\"tightNonDiagonalDualConnectorMarkerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(
        output,
        row.tight_non_diagonal_dual_connector_marker_style_profile,
    );
    output.push_str(",\"axisRuleEndpointMatchMarkerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(
        output,
        row.axis_rule_endpoint_match_marker_style_profile,
    );
    output.push_str(",\"markerStyleAgreementGate\":");
    push_fdm_open_stroke_axis_rule_marker_style_agreement_gate_json(output, row);
    output.push_str(",\"ownerPromotionGate\":");
    if row
        .owner_promotion_gate_summary
        .dual_endpoint_match_connector_count
        > 0
    {
        push_fdm_open_stroke_axis_rule_owner_promotion_gate_summary_json(
            output,
            row.owner_promotion_gate_summary,
        );
    } else {
        output.push_str("null");
    }
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"}",
    );
}

pub(super) fn push_fdm_open_stroke_axis_rule_marker_style_agreement_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    let connector_profile = row.tight_non_diagonal_dual_connector_marker_style_profile;
    let axis_rule_profile = row.axis_rule_endpoint_match_marker_style_profile;
    let (connector_marker_family, connector_marker_family_count) =
        connector_profile.dominant_marker_family();
    let (axis_rule_marker_family, axis_rule_marker_family_count) =
        axis_rule_profile.dominant_marker_family();
    let (connector_style_word, connector_style_word_count) =
        connector_profile.dominant_style_word();
    let (axis_rule_style_word, axis_rule_style_word_count) =
        axis_rule_profile.dominant_style_word();
    let dominant_marker_family_matches = connector_profile.command_count > 0
        && axis_rule_profile.command_count > 0
        && connector_marker_family == axis_rule_marker_family;
    let dominant_style_word_matches = connector_profile.command_count > 0
        && axis_rule_profile.command_count > 0
        && connector_style_word == axis_rule_style_word;
    let marker_style_agreement_candidate =
        dominant_marker_family_matches && dominant_style_word_matches;

    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+markerStyleAgreement\"");
    output.push_str(",\"source\":\"FDMVector.marker+styleWord\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"connectorProfile\":\"tightNonDiagonalDualConnectorMarkerStyleProfile\"");
    output.push_str(",\"axisRuleProfile\":\"axisRuleEndpointMatchMarkerStyleProfile\"");
    output.push_str(",\"connectorCommandCount\":");
    output.push_str(&connector_profile.command_count.to_string());
    output.push_str(",\"axisRuleCommandCount\":");
    output.push_str(&axis_rule_profile.command_count.to_string());
    output.push_str(",\"connectorDominantMarkerFamily\":");
    output.push_str(&json_string(connector_marker_family));
    output.push_str(",\"connectorDominantMarkerFamilyCount\":");
    output.push_str(&connector_marker_family_count.to_string());
    output.push_str(",\"axisRuleDominantMarkerFamily\":");
    output.push_str(&json_string(axis_rule_marker_family));
    output.push_str(",\"axisRuleDominantMarkerFamilyCount\":");
    output.push_str(&axis_rule_marker_family_count.to_string());
    output.push_str(",\"dominantMarkerFamilyMatches\":");
    output.push_str(if dominant_marker_family_matches {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorDominantStyleWord\":");
    output.push_str(&json_string(connector_style_word));
    output.push_str(",\"connectorDominantStyleWordCount\":");
    output.push_str(&connector_style_word_count.to_string());
    output.push_str(",\"axisRuleDominantStyleWord\":");
    output.push_str(&json_string(axis_rule_style_word));
    output.push_str(",\"axisRuleDominantStyleWordCount\":");
    output.push_str(&axis_rule_style_word_count.to_string());
    output.push_str(",\"dominantStyleWordMatches\":");
    output.push_str(if dominant_style_word_matches {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorMarkerFamilyHomogeneous\":");
    output.push_str(if connector_profile.marker_family_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"axisRuleMarkerFamilyHomogeneous\":");
    output.push_str(if axis_rule_profile.marker_family_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorStyleWordHomogeneous\":");
    output.push_str(if connector_profile.style_word_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"axisRuleStyleWordHomogeneous\":");
    output.push_str(if axis_rule_profile.style_word_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"markerStyleAgreementCandidate\":");
    output.push_str(if marker_style_agreement_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        fdm_open_stroke_axis_rule_marker_style_agreement_blocked_reason(
            connector_profile,
            axis_rule_profile,
        ),
    ));
    output.push('}');
}

pub(super) fn fdm_open_stroke_axis_rule_marker_style_agreement_blocked_reason(
    connector_profile: FdmOpenStrokeMarkerStyleProfile,
    axis_rule_profile: FdmOpenStrokeMarkerStyleProfile,
) -> &'static str {
    if connector_profile.command_count == 0 {
        "connector-marker-style-profile-empty"
    } else if axis_rule_profile.command_count == 0 {
        "axis-rule-marker-style-profile-empty"
    } else if connector_profile.dominant_marker_family().0
        != axis_rule_profile.dominant_marker_family().0
        && connector_profile.dominant_style_word().0 != axis_rule_profile.dominant_style_word().0
    {
        "connector-axis-rule-marker-and-style-dominance-mismatch"
    } else if connector_profile.dominant_marker_family().0
        != axis_rule_profile.dominant_marker_family().0
    {
        "connector-axis-rule-marker-family-dominance-mismatch"
    } else if connector_profile.dominant_style_word().0 != axis_rule_profile.dominant_style_word().0
    {
        "connector-axis-rule-style-word-dominance-mismatch"
    } else if !connector_profile.homogeneous_marker_style_candidate()
        || !axis_rule_profile.homogeneous_marker_style_candidate()
    {
        "matched-dominant-marker-style-still-mixed"
    } else {
        "marker-style-agreement-still-needs-owner-and-paint-order"
    }
}

pub(super) fn push_fdm_open_stroke_axis_rule_index_segment_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexSegmentMembership\"");
    output.push_str(",\"source\":\"FDMIndex.vectorOffset+FDMVector.sourceSegment\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(row.fdm_index_segment_gate_blocked_reason()));
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"vectorOffset\":");
    push_option_usize_json(output, row.fdm_index_vector_offset);
    output.push_str(",\"vectorLength\":");
    push_option_usize_json(output, row.fdm_index_vector_len);
    output.push_str(",\"validVectorOffset\":");
    output.push_str(if row.fdm_index_valid_vector_offset {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceSegmentRelativeOffset\":");
    push_option_usize_json(output, row.fdm_index_source_segment_relative_offset);
    output.push_str(",\"sourceSegmentCommandCount\":");
    push_option_usize_json(output, row.fdm_index_source_segment_command_count);
    output.push_str(",\"imageSignatureCount\":");
    output.push_str(&row.fdm_index_image_signature_count.to_string());
    output.push_str(",\"segmentImageSignatureCount\":");
    output.push_str(&row.fdm_index_segment_image_signature_count.to_string());
    output.push_str(",\"imageBearingSegmentCandidate\":");
    output.push_str(if row.image_bearing_segment_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceSegmentBackedConnectorCount\":");
    output.push_str(&row.source_segment_backed_connector_count.to_string());
    output.push_str(",\"sourceSegmentMatchesIndexEntryConnectorCount\":");
    output.push_str(
        &row.source_segment_matches_index_entry_connector_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentMissingConnectorCount\":");
    output.push_str(&row.source_segment_missing_connector_count.to_string());
    output.push_str(",\"dualEndpointSourceSegmentBackedConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_source_segment_backed_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointSourceSegmentMatchesIndexEntryConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_source_segment_matches_index_entry_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointImageBearingSegmentConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_image_bearing_segment_connector_count
            .to_string(),
    );
    output.push('}');
}

pub(super) fn push_fdm_open_stroke_axis_rule_source_order_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+parentRelativeSourceOrder\"");
    output.push_str(",\"source\":\"FDMVector.commandRelativeOffset+compoundParentRelativeOffset\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        row.axis_rule_source_order_gate_blocked_reason(),
    ));
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"dualEndpointConnectorCount\":");
    output.push_str(&row.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"sourceOrderBackedDualEndpointConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_axis_rule_source_order_backed_connector_count
            .to_string(),
    );
    output.push_str(",\"connectorParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        row.dual_endpoint_connector_parent_relative_offset_min,
        row.dual_endpoint_connector_parent_relative_offset_max,
    );
    output.push_str(",\"axisRuleParentRelativeOffsetRange\":");
    push_optional_usize_range_json(
        output,
        row.dual_endpoint_axis_rule_parent_relative_offset_min,
        row.dual_endpoint_axis_rule_parent_relative_offset_max,
    );
    output.push_str(",\"connectorBeforeAxisRuleParentSpanCount\":");
    output.push_str(
        &row.dual_endpoint_connector_before_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"connectorBetweenAxisRuleParentSpanCount\":");
    output.push_str(
        &row.dual_endpoint_connector_between_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"connectorAfterAxisRuleParentSpanCount\":");
    output.push_str(
        &row.dual_endpoint_connector_after_axis_rule_parent_span_count
            .to_string(),
    );
    output.push_str(",\"connectorAxisRuleParentSpanUnclassifiedCount\":");
    output.push_str(
        &row.dual_endpoint_connector_axis_rule_parent_span_unclassified_count
            .to_string(),
    );
    output.push('}');
}

pub(super) fn push_fdm_open_stroke_axis_rule_index_connector_composition_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    let vector_command_count = row.fdm_index_vector_command_count.unwrap_or_default();
    let connector_candidate_count = row.fdm_index_connector_candidate_count.unwrap_or_default();
    let non_connector_command_count = row
        .fdm_index_non_connector_command_count
        .unwrap_or_default();
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexConnectorComposition\"");
    output.push_str(",\"source\":\"FDMIndex.vectorCommands+FDMIndex.connectorCandidates\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        row.fdm_index_connector_composition_gate_blocked_reason(),
    ));
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"vectorCommandCount\":");
    push_option_usize_json(output, row.fdm_index_vector_command_count);
    output.push_str(",\"connectorCandidateCount\":");
    push_option_usize_json(output, row.fdm_index_connector_candidate_count);
    output.push_str(",\"nonConnectorCommandCount\":");
    push_option_usize_json(output, row.fdm_index_non_connector_command_count);
    output.push_str(",\"rowCohortConnectorCandidateCount\":");
    output.push_str(&row.connector_candidate_count.to_string());
    output.push_str(",\"connectorOnlySegmentCandidate\":");
    output.push_str(
        if vector_command_count > 0 && non_connector_command_count == 0 {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorDominantSegmentCandidate\":");
    output.push_str(if connector_candidate_count > non_connector_command_count {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorCandidateDensityPermille\":");
    push_option_usize_json(
        output,
        permille(connector_candidate_count, vector_command_count),
    );
    output.push_str(",\"matchedConnectorCoveragePermille\":");
    push_option_usize_json(
        output,
        permille(row.matched_connector_count, row.connector_candidate_count),
    );
    output.push_str(",\"dualEndpointMatchedConnectorCoveragePermille\":");
    push_option_usize_json(
        output,
        permille(
            row.dual_endpoint_match_connector_count,
            row.connector_candidate_count,
        ),
    );
    output.push('}');
}

pub(super) fn push_fdm_open_stroke_axis_rule_index_bbox_relation_gate_json(
    output: &mut String,
    row: FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
) {
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+fdmIndexBboxRelation\"");
    output.push_str(",\"source\":\"FDMIndex.bbox+FDMVector.commandSourceBbox\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        row.fdm_index_bbox_relation_gate_blocked_reason(),
    ));
    output.push_str(",\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"indexBbox\":");
    if let (Some(left), Some(top), Some(right), Some(bottom)) = (
        row.fdm_index_bbox_left,
        row.fdm_index_bbox_top,
        row.fdm_index_bbox_right,
        row.fdm_index_bbox_bottom,
    ) {
        push_fdm_normalized_bbox_json(output, (left, top, right, bottom));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"containsConnectorCount\":");
    output.push_str(&row.fdm_index_bbox_contains_connector_count.to_string());
    output.push_str(",\"overlapsConnectorCount\":");
    output.push_str(&row.fdm_index_bbox_overlaps_connector_count.to_string());
    output.push_str(",\"disjointConnectorCount\":");
    output.push_str(&row.fdm_index_bbox_disjoint_connector_count.to_string());
    output.push_str(",\"sourceBboxMissingConnectorCount\":");
    output.push_str(
        &row.fdm_index_bbox_source_bbox_missing_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointContainsConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_fdm_index_bbox_contains_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointOverlapsConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_fdm_index_bbox_overlaps_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointDisjointConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_fdm_index_bbox_disjoint_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointSourceBboxMissingConnectorCount\":");
    output.push_str(
        &row.dual_endpoint_fdm_index_bbox_source_bbox_missing_connector_count
            .to_string(),
    );
    output.push('}');
}

pub(super) fn push_fdm_open_stroke_axis_rule_owner_promotion_gate_summary_json(
    output: &mut String,
    summary: FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary,
) {
    let parent_normalized_order_gate_blocked_reason =
        summary.parent_normalized_order_gate_blocked_reason();
    let render_promotion_blocked_reason = if parent_normalized_order_gate_blocked_reason == "none" {
        "connector-ownership-and-paint-order-unproven"
    } else {
        parent_normalized_order_gate_blocked_reason
    };

    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+endpointOwnerMatchSummary+parentNormalizedOrderGate\"");
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive+fdmConnectorEndpointOwnerMatch\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"axisRuleDualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"dualEndpointOwnerCandidateCount\":");
    output.push_str(&summary.dual_endpoint_owner_candidate_count.to_string());
    output.push_str(",\"nearestFdmOwnerRowsMatchCount\":");
    output.push_str(&summary.nearest_fdm_owner_rows_match_count.to_string());
    output.push_str(",\"nearestFdmOwnerRowMatchesConnectorRowCount\":");
    output.push_str(
        &summary
            .nearest_fdm_owner_row_matches_connector_row_count
            .to_string(),
    );
    output.push_str(",\"mixedTopLevelVsNestedOrderNamespaceCount\":");
    output.push_str(
        &summary
            .mixed_top_level_vs_nested_order_namespace_count
            .to_string(),
    );
    output.push_str(",\"parentNormalizedOrderGateBlockedReason\":");
    output.push_str(&json_string(parent_normalized_order_gate_blocked_reason));
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnectorCount\":");
    output.push_str(
        &summary
            .parent_normalized_ordered_same_row_same_connector_count
            .to_string(),
    );
    output.push_str(",\"ownerParentCommandRelationCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_candidate_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-not-connector-row",
        summary.nearest_owner_row_not_connector_row_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-before-owner-parent-command-span",
        summary.before_owner_parent_command_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-between-owner-parent-command-span",
        summary.between_owner_parent_command_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-after-owner-parent-command-span",
        summary.after_owner_parent_command_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-parent-command-relation-unclassified",
        summary.parent_command_relation_unclassified_count,
    );
    output.push(']');
    output.push_str(",\"ownerParentSourceOrderRelationCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "missing-endpoint-owner-candidate",
        summary.missing_endpoint_owner_candidate_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-mismatch",
        summary.nearest_owner_row_mismatch_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "nearest-owner-row-not-connector-row",
        summary.nearest_owner_row_not_connector_row_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-before-owner-parent-relative-offset-span",
        summary.before_owner_parent_relative_offset_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-between-owner-parent-relative-offset-span",
        summary.between_owner_parent_relative_offset_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-after-owner-parent-relative-offset-span",
        summary.after_owner_parent_relative_offset_span_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "same-row-parent-relative-offset-relation-unclassified",
        summary.parent_relative_offset_relation_unclassified_count,
    );
    output.push_str("]}");
}

pub(super) fn push_fdm_open_stroke_marker_style_profile_json(
    output: &mut String,
    profile: FdmOpenStrokeMarkerStyleProfile,
) {
    output.push_str("{\"basis\":\"fdm-vector-marker+style-word\"");
    output.push_str(",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true");
    output.push_str(",\"commandCount\":");
    output.push_str(&profile.command_count.to_string());
    output.push_str(",\"markerFamilyCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "line-marker",
        profile.line_marker_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "path-marker",
        profile.path_marker_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "bezier-marker",
        profile.bezier_marker_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "ellipse-marker",
        profile.ellipse_marker_count,
    );
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "other-marker",
        profile.other_marker_count,
    );
    output.push_str("],\"styleWordCounts\":[");
    push_fdm_connector_graph_blocked_reason_count_json(output, "0x0000", profile.style_0000_count);
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(output, "0x0005", profile.style_0005_count);
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(output, "0x0080", profile.style_0080_count);
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(output, "0x00a0", profile.style_00a0_count);
    output.push(',');
    push_fdm_connector_graph_blocked_reason_count_json(
        output,
        "other-style",
        profile.other_style_count,
    );
    output.push_str("],\"roleGate\":");
    push_fdm_open_stroke_marker_style_role_gate_json(output, profile);
    output.push('}');
}

pub(super) fn push_fdm_open_stroke_marker_style_role_gate_json(
    output: &mut String,
    profile: FdmOpenStrokeMarkerStyleProfile,
) {
    output.push_str("{\"basis\":\"fdm-vector-marker-style-profile\"");
    output.push_str(
        ",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false",
    );
    output.push_str(",\"markerFamilyDiversityCount\":");
    output.push_str(&profile.marker_family_diversity_count().to_string());
    output.push_str(",\"styleWordDiversityCount\":");
    output.push_str(&profile.style_word_diversity_count().to_string());
    let (dominant_marker_family, dominant_marker_family_count) = profile.dominant_marker_family();
    let (dominant_style_word, dominant_style_word_count) = profile.dominant_style_word();
    output.push_str(",\"dominantMarkerFamily\":");
    output.push_str(&json_string(dominant_marker_family));
    output.push_str(",\"dominantMarkerFamilyCount\":");
    output.push_str(&dominant_marker_family_count.to_string());
    output.push_str(",\"dominantStyleWord\":");
    output.push_str(&json_string(dominant_style_word));
    output.push_str(",\"dominantStyleWordCount\":");
    output.push_str(&dominant_style_word_count.to_string());
    output.push_str(",\"markerFamilyHomogeneous\":");
    output.push_str(if profile.marker_family_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"styleWordHomogeneous\":");
    output.push_str(if profile.style_word_homogeneous() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"homogeneousMarkerStyleCandidate\":");
    output.push_str(if profile.homogeneous_marker_style_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        profile.marker_style_role_promotion_blocked_reason(),
    ));
    output.push('}');
}

pub(super) fn push_fdm_connector_owner_row_cohort_summary_json(
    output: &mut String,
    cohort: &str,
    summary: FdmConnectorOwnerRowCohortDiagnosticSummary,
) {
    output.push_str("{\"cohort\":");
    output.push_str(&json_string(cohort));
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&summary.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&summary.matched_connector_count.to_string());
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&summary.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&summary.nearby_endpoint_match_count.to_string());
    output.push('}');
}

pub(super) fn push_fdm_connector_owner_command_relation_summary_json(
    output: &mut String,
    relation: &str,
    summary: FdmConnectorOwnerRowCohortDiagnosticSummary,
) {
    output.push_str("{\"relation\":");
    output.push_str(&json_string(relation));
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(&summary.total_thresholded_endpoint_match_count.to_string());
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(&summary.matched_connector_count.to_string());
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(&summary.dual_endpoint_match_connector_count.to_string());
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(&summary.tight_endpoint_match_count.to_string());
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(&summary.nearby_endpoint_match_count.to_string());
    output.push('}');
}

pub(super) fn push_fdm_connector_graph_blocked_reason_count_json(
    output: &mut String,
    reason: &str,
    count: usize,
) {
    output.push_str("{\"reason\":");
    output.push_str(&json_string(reason));
    output.push_str(",\"count\":");
    output.push_str(&count.to_string());
    output.push('}');
}

pub(super) fn push_fdm_connector_dominant_matched_connector_row_json(
    output: &mut String,
    summary: FdmConnectorGraphDiagnosticSummary,
) {
    let Some(row_index) = summary.dominant_matched_connector_row_index else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"basis\":\"fdmConnectorCandidateRowIndex+lineRuleEndpointMatchSummary\"");
    output.push_str(",\"rowIndex\":");
    output.push_str(&row_index.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_connector_candidate_count
            .to_string(),
    );
    output.push_str(",\"totalThresholdedEndpointMatchCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_total_thresholded_endpoint_match_count
            .to_string(),
    );
    output.push_str(",\"matchedConnectorCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_matched_connector_count
            .to_string(),
    );
    output.push_str(",\"dualEndpointMatchConnectorCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_dual_endpoint_match_connector_count
            .to_string(),
    );
    output.push_str(",\"startOnlyLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_start_only_match_connector_count
            .to_string(),
    );
    output.push_str(",\"endOnlyLineRuleMatchConnectorCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_end_only_match_connector_count
            .to_string(),
    );
    output.push_str(",\"tightEndpointMatchCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_tight_endpoint_match_count
            .to_string(),
    );
    output.push_str(",\"nearbyEndpointMatchCount\":");
    output.push_str(
        &summary
            .dominant_matched_connector_row_nearby_endpoint_match_count
            .to_string(),
    );
    output.push_str(",\"renderPromotionBlockedReason\":\"dominant-row-still-lacks-dual-endpoint-line-rule-match\"}");
}

pub(super) fn push_page_layer_fdm_open_stroke_cohort_summary_json(
    output: &mut String,
    layout: PageLayout,
    summary: &FdmOpenStrokeCohortSummary,
) {
    output.push_str("{\"type\":\"fdmOpenStrokeCohortSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"projectionKind\":\"fdmOpenStrokeCohortSummary\"");
    output.push_str(",\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"placementProven\":false,\"decoded\":false");
    push_fdm_open_stroke_cohort_summary_fields_json(output, summary);
    output.push('}');
}

pub(super) fn fdm_open_stroke_cohort_summary_json(
    layout: PageLayout,
    document: &Document,
) -> String {
    let command_diagnostics = fdm_command_diagnostics(document);
    let Some(extent) = fdm_command_projection_extent(&command_diagnostics) else {
        return "null".to_string();
    };
    let primitive_diagnostics = fdm_vector_primitive_diagnostics(document);
    let Some(summary) = fdm_open_stroke_cohort_summary(layout, &primitive_diagnostics, extent)
    else {
        return "null".to_string();
    };
    let mut output = String::from("{");
    output.push_str("\"projectionKind\":\"fdmOpenStrokeCohortSummary\"");
    output.push_str(",\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"placementProven\":false,\"decoded\":false");
    push_fdm_open_stroke_cohort_summary_fields_json(&mut output, &summary);
    output.push('}');
    output
}

pub(super) fn push_fdm_open_stroke_cohort_summary_fields_json(
    output: &mut String,
    summary: &FdmOpenStrokeCohortSummary,
) {
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\"");
    output.push_str(",\"basis\":\"open-stroke-row-source-cohorts\"");
    output.push_str(",\"sourceBacked\":true");
    output.push_str(",\"geometryDecoded\":true");
    output.push_str(",\"ownershipProven\":false");
    output.push_str(",\"paintOrderDecoded\":false");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"open-stroke-role-and-paint-order-unproven\"",
    );
    output.push_str(",\"primitiveCount\":");
    output.push_str(&summary.primitive_count.to_string());
    output.push_str(",\"openStrokeCount\":");
    output.push_str(&summary.open_stroke_count.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&summary.connector_candidate_count.to_string());
    output.push_str(",\"horizontalCount\":");
    output.push_str(&summary.horizontal_count.to_string());
    output.push_str(",\"verticalCount\":");
    output.push_str(&summary.vertical_count.to_string());
    output.push_str(",\"diagonalCount\":");
    output.push_str(&summary.diagonal_count.to_string());
    output.push_str(",\"lineMarkerCount\":");
    output.push_str(&summary.line_marker_count.to_string());
    output.push_str(",\"nonLineMarkerCount\":");
    output.push_str(&summary.non_line_marker_count.to_string());
    output.push_str(",\"rowCount\":");
    output.push_str(&summary.row_count.to_string());
    output.push_str(",\"rowCohortLimit\":");
    output.push_str(&FDM_OPEN_STROKE_ROW_COHORT_LIMIT.to_string());
    output.push_str(",\"rowCohortCount\":");
    output.push_str(&summary.row_cohorts.len().to_string());
    output.push_str(",\"dominantConnectorRow\":");
    push_fdm_open_stroke_dominant_connector_row_json(output, summary);
    output.push_str(",\"rowCohorts\":[");
    for (index, row) in summary.row_cohorts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_open_stroke_row_cohort_json(output, row);
    }
    output.push(']');
}

pub(super) fn push_fdm_open_stroke_dominant_connector_row_json(
    output: &mut String,
    summary: &FdmOpenStrokeCohortSummary,
) {
    let Some(row_index) = summary.dominant_connector_row_index else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"basis\":\"fdmOpenStrokeRowConnectorCandidateCount\"");
    output.push_str(",\"rowIndex\":");
    output.push_str(&row_index.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(
        &summary
            .dominant_connector_row_connector_candidate_count
            .to_string(),
    );
    output.push_str(",\"openStrokeCount\":");
    output.push_str(&summary.dominant_connector_row_open_stroke_count.to_string());
    output.push_str(",\"horizontalCount\":");
    output.push_str(&summary.dominant_connector_row_horizontal_count.to_string());
    output.push_str(",\"verticalCount\":");
    output.push_str(&summary.dominant_connector_row_vertical_count.to_string());
    output
        .push_str(",\"renderPromotionBlockedReason\":\"dominant-open-stroke-row-role-unproven\"}");
}

pub(super) fn push_fdm_open_stroke_row_cohort_json(
    output: &mut String,
    row: &FdmOpenStrokeRowCohortSummary,
) {
    output.push_str("{\"rowIndex\":");
    output.push_str(&row.row_index.to_string());
    output.push_str(",\"openStrokeCount\":");
    output.push_str(&row.open_stroke_count.to_string());
    output.push_str(",\"connectorCandidateCount\":");
    output.push_str(&row.connector_candidate_count.to_string());
    output.push_str(",\"horizontalCount\":");
    output.push_str(&row.horizontal_count.to_string());
    output.push_str(",\"verticalCount\":");
    output.push_str(&row.vertical_count.to_string());
    output.push_str(",\"diagonalCount\":");
    output.push_str(&row.diagonal_count.to_string());
    output.push_str(",\"lineMarkerCount\":");
    output.push_str(&row.line_marker_count.to_string());
    output.push_str(",\"nonLineMarkerCount\":");
    output.push_str(&row.non_line_marker_count.to_string());
    output.push_str(",\"markerStyleProfile\":");
    push_fdm_open_stroke_marker_style_profile_json(output, row.marker_style_profile);
    output.push_str(",\"commandIndexMin\":");
    push_option_usize_json(output, row.command_index_min);
    output.push_str(",\"commandIndexMax\":");
    push_option_usize_json(output, row.command_index_max);
    output.push_str(",\"relativeOffsetMin\":");
    push_option_usize_json(output, row.relative_offset_min);
    output.push_str(",\"relativeOffsetMax\":");
    push_option_usize_json(output, row.relative_offset_max);
    output.push_str(",\"sourceBboxUnion\":");
    if let Some(bbox) = row.source_bbox_union {
        push_fdm_normalized_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"projectedBboxUnion\":");
    if let Some(bbox) = row.projected_bbox_union {
        push_bbox_tuple_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn push_fdm_connector_endpoint_owner_candidates_json(
    output: &mut String,
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    output.push_str("{\"basis\":\"fdmPrimitiveProjection+documentTextGroupLineProjection\",\"ownershipProven\":false,\"sourceBacked\":true");
    let probe_radius_px = fdm_connector_endpoint_owner_probe_radius_px(text_projection);
    output.push_str(",\"probeRadiusPx\":");
    output.push_str(&format!("{probe_radius_px:.3}"));
    output.push_str(",\"candidateLimit\":");
    output.push_str(&FDM_CONNECTOR_ENDPOINT_OWNER_CANDIDATE_LIMIT.to_string());
    output.push_str(",\"start\":");
    push_fdm_connector_endpoint_owner_candidate_array_json(
        output,
        metric.projected_start,
        layout,
        diagnostic,
        extent,
        primitive_diagnostics,
        text_projection,
        probe_radius_px,
    );
    output.push_str(",\"end\":");
    push_fdm_connector_endpoint_owner_candidate_array_json(
        output,
        metric.projected_end,
        layout,
        diagnostic,
        extent,
        primitive_diagnostics,
        text_projection,
        probe_radius_px,
    );
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_fdm_connector_endpoint_owner_candidate_array_json(
    output: &mut String,
    point: (f32, f32),
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
    probe_radius_px: f32,
) {
    let candidates = fdm_connector_endpoint_owner_candidates(
        point,
        layout,
        connector,
        extent,
        primitive_diagnostics,
        text_projection,
    );
    output.push('[');
    for (index, candidate) in candidates.iter().copied().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_fdm_connector_endpoint_owner_candidate_json(output, candidate, probe_radius_px);
    }
    output.push(']');
}

pub(super) fn push_fdm_connector_endpoint_owner_candidate_json(
    output: &mut String,
    candidate: FdmConnectorEndpointOwnerCandidate<'_>,
    probe_radius_px: f32,
) {
    match candidate {
        FdmConnectorEndpointOwnerCandidate::Primitive {
            diagnostic,
            bbox,
            distance_px,
        } => {
            output.push_str("{\"kind\":\"fdmPrimitive\",\"source\":\"fdmVectorCommandPrimitive\"");
            output.push_str(",\"ownerProven\":false,\"sourceBacked\":true");
            output.push_str(",\"distancePx\":");
            output.push_str(&format!("{distance_px:.3}"));
            output.push_str(",\"withinProbeRadius\":");
            output.push_str(if distance_px <= probe_radius_px {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"bbox\":");
            push_bbox_tuple_json(output, bbox);
            output.push_str(",\"sourcePath\":");
            output.push_str(&json_string(diagnostic.candidate.path()));
            output.push_str(",\"objectCandidateIndex\":");
            output.push_str(&diagnostic.candidate_index.to_string());
            output.push_str(",\"rowIndex\":");
            output.push_str(&diagnostic.entry.row_index().to_string());
            output.push_str(",\"commandIndex\":");
            output.push_str(&diagnostic.command.command_index().to_string());
            output.push_str(",\"relativeOffset\":");
            output.push_str(&diagnostic.command.relative_offset().to_string());
            output.push_str(",\"markerHex\":");
            output.push_str(&json_string(&hex_bytes(diagnostic.command.marker())));
            output.push_str(",\"primitiveKind\":");
            output.push_str(&json_string(fdm_vector_primitive_kind(diagnostic.command)));
            output.push_str(",\"sourcePathBbox\":");
            if let Some(source_bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
                push_object_fdm_index_bbox_json(output, source_bbox);
            } else {
                output.push_str("null");
            }
            output.push('}');
        }
        FdmConnectorEndpointOwnerCandidate::TextSlot {
            slot,
            bbox,
            distance_px,
        } => {
            output.push_str("{\"kind\":\"documentTextSlot\",\"source\":\"/DocumentText\"");
            output.push_str(",\"ownerProven\":false,\"sourceBacked\":true");
            output.push_str(",\"distancePx\":");
            output.push_str(&format!("{distance_px:.3}"));
            output.push_str(",\"withinProbeRadius\":");
            output.push_str(if distance_px <= probe_radius_px {
                "true"
            } else {
                "false"
            });
            output.push_str(",\"bbox\":");
            push_bbox_tuple_json(output, bbox);
            output.push_str(",\"text\":");
            output.push_str(&json_string(&slot.text));
            output.push_str(",\"groupIndex\":");
            match slot.group_index {
                Some(group_index) => output.push_str(&group_index.to_string()),
                None => output.push_str("null"),
            }
            output.push_str(",\"lineOffsetUnits\":");
            output.push_str(&slot.line_offset_units.to_string());
            output.push_str(",\"fragmentStartUnits\":");
            output.push_str(&slot.fragment_start_units.to_string());
            output.push_str(",\"sourceByteRange\":");
            output.push_str(&source_range_json(
                slot.source_span.byte_start(),
                slot.source_span.byte_end(),
            ));
            output.push_str(",\"sourceUnitRange\":");
            output.push_str(&source_range_json(
                slot.source_span.unit_start(),
                slot.source_span.unit_end(),
            ));
            output.push('}');
        }
    }
}

pub(super) fn push_fdm_connector_endpoint_owner_match_summary_json(
    output: &mut String,
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    let summary = fdm_connector_endpoint_owner_match_summary(
        layout,
        connector,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );
    output.push_str("{\"startCandidateCount\":");
    output.push_str(&summary.start_candidate_count.to_string());
    output.push_str(",\"endCandidateCount\":");
    output.push_str(&summary.end_candidate_count.to_string());
    output.push_str(",\"totalCandidateCount\":");
    output.push_str(&summary.total_candidate_count.to_string());
    output.push_str(",\"startWithinProbeCount\":");
    output.push_str(&summary.start_within_probe_count.to_string());
    output.push_str(",\"endWithinProbeCount\":");
    output.push_str(&summary.end_within_probe_count.to_string());
    output.push_str(",\"withinProbeCandidateCount\":");
    output.push_str(&summary.within_probe_candidate_count.to_string());
    output.push_str(",\"fdmPrimitiveCandidateCount\":");
    output.push_str(&summary.fdm_primitive_candidate_count.to_string());
    output.push_str(",\"documentTextSlotCandidateCount\":");
    output.push_str(&summary.document_text_slot_candidate_count.to_string());
    output.push_str(",\"dualEndpointOwnerCandidate\":");
    output.push_str(if summary.dual_endpoint_owner_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorCommandIndex\":");
    output.push_str(&summary.connector_command_index.to_string());
    output.push_str(",\"connectorParentCommandIndex\":");
    output.push_str(&summary.connector_parent_command_index.to_string());
    output.push_str(",\"connectorSyntheticNestedCommand\":");
    output.push_str(if summary.connector_synthetic_nested_command {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorRelativeOffset\":");
    output.push_str(&summary.connector_relative_offset.to_string());
    output.push_str(",\"connectorParentRelativeOffset\":");
    push_option_usize_json(output, summary.connector_parent_relative_offset);
    output.push_str(",\"startNearestFdmOwner\":");
    push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
        output,
        summary.start_nearest_fdm_owner_row_index,
        summary.start_nearest_fdm_owner_command_index,
        summary.start_nearest_fdm_owner_parent_command_index,
        summary.start_nearest_fdm_owner_synthetic_nested_command,
        summary.start_nearest_fdm_owner_relative_offset,
    );
    output.push_str(",\"endNearestFdmOwner\":");
    push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
        output,
        summary.end_nearest_fdm_owner_row_index,
        summary.end_nearest_fdm_owner_command_index,
        summary.end_nearest_fdm_owner_parent_command_index,
        summary.end_nearest_fdm_owner_synthetic_nested_command,
        summary.end_nearest_fdm_owner_relative_offset,
    );
    output.push_str(",\"nearestFdmOwnerRowsMatch\":");
    output.push_str(if summary.nearest_fdm_owner_rows_match {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nearestFdmOwnerRowMatchesConnectorRow\":");
    output.push_str(if summary.nearest_fdm_owner_row_matches_connector_row {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedTopLevelVsNestedOrderNamespace\":");
    output.push_str(if summary.mixed_top_level_vs_nested_order_namespace {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"connectorCommandBetweenNearestFdmOwnerCommands\":");
    output.push_str(
        if summary.connector_command_between_nearest_fdm_owner_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorCommandBeforeNearestFdmOwnerCommands\":");
    output.push_str(
        if summary.connector_command_before_nearest_fdm_owner_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorCommandAfterNearestFdmOwnerCommands\":");
    output.push_str(
        if summary.connector_command_after_nearest_fdm_owner_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorRelativeOffsetBetweenNearestFdmOwnerOffsets\":");
    output.push_str(
        if summary.connector_relative_offset_between_nearest_fdm_owner_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorRelativeOffsetBeforeNearestFdmOwnerOffsets\":");
    output.push_str(
        if summary.connector_relative_offset_before_nearest_fdm_owner_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorRelativeOffsetAfterNearestFdmOwnerOffsets\":");
    output.push_str(
        if summary.connector_relative_offset_after_nearest_fdm_owner_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"startNearestFdmOwnerParentRelativeOffset\":");
    push_option_usize_json(
        output,
        summary.start_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"endNearestFdmOwnerParentRelativeOffset\":");
    push_option_usize_json(output, summary.end_nearest_fdm_owner_parent_relative_offset);
    output.push_str(",\"connectorParentCommandBetweenNearestFdmOwnerParentCommands\":");
    output.push_str(
        if summary.connector_parent_command_between_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandBeforeNearestFdmOwnerParentCommands\":");
    output.push_str(
        if summary.connector_parent_command_before_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandAfterNearestFdmOwnerParentCommands\":");
    output.push_str(
        if summary.connector_parent_command_after_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetBetweenNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if summary.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetBeforeNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if summary.connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetAfterNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if summary.connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"ownerCommandRelation\":");
    output.push_str(&json_string(summary.owner_command_relation()));
    output.push_str(",\"ownerSourceOrderRelation\":");
    output.push_str(&json_string(summary.owner_source_order_relation()));
    output.push_str(",\"ownerParentCommandRelation\":");
    output.push_str(&json_string(summary.owner_parent_command_relation()));
    output.push_str(",\"ownerParentSourceOrderRelation\":");
    output.push_str(&json_string(summary.owner_parent_source_order_relation()));
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnector\":");
    output.push_str(
        if summary.parent_normalized_ordered_same_row_same_connector() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"ownerProven\":false");
    output.push_str(",\"ownerGroupingProven\":false");
    output.push_str(",\"ownerGroupingPromotionBlockedReason\":");
    output.push_str(&json_string(
        summary.owner_grouping_promotion_blocked_reason(),
    ));
    output.push_str(",\"ownershipPromotionBlockedReason\":");
    output.push_str(&json_string(summary.ownership_promotion_blocked_reason()));
    output.push('}');
}

pub(super) fn push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
    output: &mut String,
    row_index: Option<usize>,
    command_index: Option<usize>,
    parent_command_index: Option<usize>,
    synthetic_nested_command: bool,
    relative_offset: Option<usize>,
) {
    if let (Some(row_index), Some(command_index), Some(parent_command_index)) =
        (row_index, command_index, parent_command_index)
    {
        output.push_str("{\"rowIndex\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"commandIndex\":");
        output.push_str(&command_index.to_string());
        output.push_str(",\"parentCommandIndex\":");
        output.push_str(&parent_command_index.to_string());
        output.push_str(",\"syntheticNestedCommand\":");
        output.push_str(if synthetic_nested_command {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"relativeOffset\":");
        match relative_offset {
            Some(relative_offset) => output.push_str(&relative_offset.to_string()),
            None => output.push_str("null"),
        }
        output.push('}');
    } else {
        output.push_str("null");
    }
}

pub(super) fn fdm_connector_endpoint_owner_probe_radius_px(
    text_projection: Option<&ShanaiLanTextProjection>,
) -> f32 {
    text_projection
        .map(|projection| projection.line_height_px)
        .filter(|value| *value > 0.0)
        .unwrap_or(FDM_CONNECTOR_ENDPOINT_OWNER_PROBE_RADIUS_PX)
}

pub(super) fn fdm_connector_endpoint_owner_candidates<'a>(
    point: (f32, f32),
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'a>,
    extent: FdmCommandProjectionExtent,
    primitive_diagnostics: &'a [FdmCommandDiagnostic<'a>],
    text_projection: Option<&'a ShanaiLanTextProjection>,
) -> Vec<FdmConnectorEndpointOwnerCandidate<'a>> {
    let mut candidates = Vec::new();
    for diagnostic in primitive_diagnostics.iter().copied() {
        if fdm_command_diagnostic_same_command(connector, diagnostic)
            || fdm_connector_candidate_metric(layout, diagnostic, extent).is_some()
        {
            continue;
        }
        let Some(bbox) = fdm_path_diagnostic_bbox(layout, diagnostic, extent) else {
            continue;
        };
        let distance_px = distance_from_point_to_bbox(point.0, point.1, bbox);
        candidates.push(FdmConnectorEndpointOwnerCandidate::Primitive {
            diagnostic,
            bbox,
            distance_px,
        });
    }

    if let Some(projection) = text_projection {
        for slot in &projection.slots {
            let bbox = shanai_lan_text_slot_bbox(slot);
            let distance_px = distance_from_point_to_bbox(point.0, point.1, bbox);
            candidates.push(FdmConnectorEndpointOwnerCandidate::TextSlot {
                slot,
                bbox,
                distance_px,
            });
        }
    }

    candidates.sort_by(|left, right| {
        left.distance_px()
            .partial_cmp(&right.distance_px())
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.rank().cmp(&right.rank()))
    });
    candidates.truncate(FDM_CONNECTOR_ENDPOINT_OWNER_CANDIDATE_LIMIT);
    candidates
}

pub(super) fn fdm_connector_endpoint_owner_match_summary(
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
) -> FdmConnectorEndpointOwnerMatchSummary {
    let probe_radius_px = fdm_connector_endpoint_owner_probe_radius_px(text_projection);
    let start = fdm_connector_endpoint_owner_candidates(
        metric.projected_start,
        layout,
        connector,
        extent,
        primitive_diagnostics,
        text_projection,
    );
    let end = fdm_connector_endpoint_owner_candidates(
        metric.projected_end,
        layout,
        connector,
        extent,
        primitive_diagnostics,
        text_projection,
    );
    let connector_command_index = connector.command.command_index();
    let connector_parent_relative_offset = fdm_command_normalized_parent_relative_offset(connector);
    let mut summary = FdmConnectorEndpointOwnerMatchSummary {
        start_candidate_count: start.len(),
        end_candidate_count: end.len(),
        total_candidate_count: start.len() + end.len(),
        connector_command_index,
        connector_parent_command_index: fdm_command_parent_command_index(connector_command_index),
        connector_synthetic_nested_command: fdm_command_index_is_synthetic_nested(
            connector_command_index,
        ),
        connector_relative_offset: connector.command.relative_offset(),
        connector_parent_relative_offset,
        ..Default::default()
    };
    accumulate_fdm_connector_endpoint_owner_candidates(&mut summary, &start, probe_radius_px, true);
    accumulate_fdm_connector_endpoint_owner_candidates(&mut summary, &end, probe_radius_px, false);
    if let Some(start_owner) = fdm_connector_nearest_within_probe_fdm_owner(&start, probe_radius_px)
    {
        let command_index = start_owner.command.command_index();
        summary.start_nearest_fdm_owner_row_index = Some(start_owner.entry.row_index());
        summary.start_nearest_fdm_owner_command_index = Some(command_index);
        summary.start_nearest_fdm_owner_parent_command_index =
            Some(fdm_command_parent_command_index(command_index));
        summary.start_nearest_fdm_owner_synthetic_nested_command =
            fdm_command_index_is_synthetic_nested(command_index);
        summary.start_nearest_fdm_owner_relative_offset =
            Some(start_owner.command.relative_offset());
        summary.start_nearest_fdm_owner_parent_relative_offset =
            fdm_command_normalized_parent_relative_offset(start_owner);
    }
    if let Some(end_owner) = fdm_connector_nearest_within_probe_fdm_owner(&end, probe_radius_px) {
        let command_index = end_owner.command.command_index();
        summary.end_nearest_fdm_owner_row_index = Some(end_owner.entry.row_index());
        summary.end_nearest_fdm_owner_command_index = Some(command_index);
        summary.end_nearest_fdm_owner_parent_command_index =
            Some(fdm_command_parent_command_index(command_index));
        summary.end_nearest_fdm_owner_synthetic_nested_command =
            fdm_command_index_is_synthetic_nested(command_index);
        summary.end_nearest_fdm_owner_relative_offset = Some(end_owner.command.relative_offset());
        summary.end_nearest_fdm_owner_parent_relative_offset =
            fdm_command_normalized_parent_relative_offset(end_owner);
    }
    if let (Some(start_row), Some(end_row)) = (
        summary.start_nearest_fdm_owner_row_index,
        summary.end_nearest_fdm_owner_row_index,
    ) {
        summary.nearest_fdm_owner_rows_match = start_row == end_row;
        summary.nearest_fdm_owner_row_matches_connector_row =
            start_row == connector.entry.row_index() && end_row == connector.entry.row_index();
    }
    if let (Some(start_command), Some(end_command)) = (
        summary.start_nearest_fdm_owner_command_index,
        summary.end_nearest_fdm_owner_command_index,
    ) {
        summary.mixed_top_level_vs_nested_order_namespace = summary
            .connector_synthetic_nested_command
            != summary.start_nearest_fdm_owner_synthetic_nested_command
            || summary.connector_synthetic_nested_command
                != summary.end_nearest_fdm_owner_synthetic_nested_command
            || summary.start_nearest_fdm_owner_synthetic_nested_command
                != summary.end_nearest_fdm_owner_synthetic_nested_command;
        let low = start_command.min(end_command);
        let high = start_command.max(end_command);
        let connector_command = summary.connector_command_index;
        summary.connector_command_between_nearest_fdm_owner_commands =
            low <= connector_command && connector_command <= high;
        summary.connector_command_before_nearest_fdm_owner_commands = connector_command < low;
        summary.connector_command_after_nearest_fdm_owner_commands = connector_command > high;

        let start_parent = fdm_command_parent_command_index(start_command);
        let end_parent = fdm_command_parent_command_index(end_command);
        let low_parent = start_parent.min(end_parent);
        let high_parent = start_parent.max(end_parent);
        let connector_parent = summary.connector_parent_command_index;
        summary.connector_parent_command_between_nearest_fdm_owner_parent_commands =
            low_parent <= connector_parent && connector_parent <= high_parent;
        summary.connector_parent_command_before_nearest_fdm_owner_parent_commands =
            connector_parent < low_parent;
        summary.connector_parent_command_after_nearest_fdm_owner_parent_commands =
            connector_parent > high_parent;
    }
    if let (Some(start_relative_offset), Some(end_relative_offset)) = (
        summary.start_nearest_fdm_owner_relative_offset,
        summary.end_nearest_fdm_owner_relative_offset,
    ) {
        let low = start_relative_offset.min(end_relative_offset);
        let high = start_relative_offset.max(end_relative_offset);
        let connector_relative_offset = summary.connector_relative_offset;
        summary.connector_relative_offset_between_nearest_fdm_owner_offsets =
            low <= connector_relative_offset && connector_relative_offset <= high;
        summary.connector_relative_offset_before_nearest_fdm_owner_offsets =
            connector_relative_offset < low;
        summary.connector_relative_offset_after_nearest_fdm_owner_offsets =
            connector_relative_offset > high;
    }
    if let (
        Some(connector_parent_relative_offset),
        Some(start_parent_relative_offset),
        Some(end_parent_relative_offset),
    ) = (
        summary.connector_parent_relative_offset,
        summary.start_nearest_fdm_owner_parent_relative_offset,
        summary.end_nearest_fdm_owner_parent_relative_offset,
    ) {
        let low = start_parent_relative_offset.min(end_parent_relative_offset);
        let high = start_parent_relative_offset.max(end_parent_relative_offset);
        summary.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets =
            low <= connector_parent_relative_offset && connector_parent_relative_offset <= high;
        summary.connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets =
            connector_parent_relative_offset < low;
        summary.connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets =
            connector_parent_relative_offset > high;
    }
    summary
}

pub(super) fn fdm_connector_nearest_within_probe_fdm_owner<'a>(
    candidates: &[FdmConnectorEndpointOwnerCandidate<'a>],
    probe_radius_px: f32,
) -> Option<FdmCommandDiagnostic<'a>> {
    candidates.iter().find_map(|candidate| match candidate {
        FdmConnectorEndpointOwnerCandidate::Primitive {
            diagnostic,
            distance_px,
            ..
        } if *distance_px <= probe_radius_px => Some(*diagnostic),
        _ => None,
    })
}

pub(super) fn accumulate_fdm_connector_endpoint_owner_candidates(
    summary: &mut FdmConnectorEndpointOwnerMatchSummary,
    candidates: &[FdmConnectorEndpointOwnerCandidate<'_>],
    probe_radius_px: f32,
    start: bool,
) {
    for candidate in candidates {
        match candidate {
            FdmConnectorEndpointOwnerCandidate::Primitive { .. } => {
                summary.fdm_primitive_candidate_count += 1;
            }
            FdmConnectorEndpointOwnerCandidate::TextSlot { .. } => {
                summary.document_text_slot_candidate_count += 1;
            }
        }
        if candidate.distance_px() <= probe_radius_px {
            summary.within_probe_candidate_count += 1;
            if start {
                summary.start_within_probe_count += 1;
            } else {
                summary.end_within_probe_count += 1;
            }
        }
    }
}

impl FdmConnectorEndpointOwnerMatchSummary {
    pub(super) fn dual_endpoint_owner_candidate(self) -> bool {
        self.start_within_probe_count > 0 && self.end_within_probe_count > 0
    }

    pub(super) fn ownership_promotion_blocked_reason(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else {
            "endpoint-owner-candidate-unproven"
        }
    }

    pub(super) fn owner_grouping_promotion_blocked_reason(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else {
            "owner-row-candidate-unproven"
        }
    }

    pub(super) fn owner_command_relation(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else if self.mixed_top_level_vs_nested_order_namespace {
            "same-row-mixed-command-namespace"
        } else if self.connector_command_before_nearest_fdm_owner_commands {
            "same-row-before-owner-command-span"
        } else if self.connector_command_between_nearest_fdm_owner_commands {
            "same-row-between-owner-command-span"
        } else if self.connector_command_after_nearest_fdm_owner_commands {
            "same-row-after-owner-command-span"
        } else {
            "same-row-command-relation-unclassified"
        }
    }

    pub(super) fn owner_source_order_relation(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else if self.connector_relative_offset_before_nearest_fdm_owner_offsets {
            "same-row-before-owner-relative-offset-span"
        } else if self.connector_relative_offset_between_nearest_fdm_owner_offsets {
            "same-row-between-owner-relative-offset-span"
        } else if self.connector_relative_offset_after_nearest_fdm_owner_offsets {
            "same-row-after-owner-relative-offset-span"
        } else {
            "same-row-relative-offset-relation-unclassified"
        }
    }

    pub(super) fn owner_parent_command_relation(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else if self.connector_parent_command_before_nearest_fdm_owner_parent_commands {
            "same-row-before-owner-parent-command-span"
        } else if self.connector_parent_command_between_nearest_fdm_owner_parent_commands {
            "same-row-between-owner-parent-command-span"
        } else if self.connector_parent_command_after_nearest_fdm_owner_parent_commands {
            "same-row-after-owner-parent-command-span"
        } else {
            "same-row-parent-command-relation-unclassified"
        }
    }

    pub(super) fn owner_parent_source_order_relation(self) -> &'static str {
        if !self.dual_endpoint_owner_candidate() {
            "missing-endpoint-owner-candidate"
        } else if !self.nearest_fdm_owner_rows_match {
            "nearest-owner-row-mismatch"
        } else if !self.nearest_fdm_owner_row_matches_connector_row {
            "nearest-owner-row-not-connector-row"
        } else if self.connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets {
            "same-row-before-owner-parent-relative-offset-span"
        } else if self.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets {
            "same-row-between-owner-parent-relative-offset-span"
        } else if self.connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets {
            "same-row-after-owner-parent-relative-offset-span"
        } else {
            "same-row-parent-relative-offset-relation-unclassified"
        }
    }

    pub(super) fn parent_normalized_ordered_same_row_same_connector(self) -> bool {
        self.nearest_fdm_owner_rows_match
            && self.nearest_fdm_owner_row_matches_connector_row
            && self.connector_parent_command_between_nearest_fdm_owner_parent_commands
            && self.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets
    }
}

pub(super) fn fdm_command_diagnostic_same_command(
    left: FdmCommandDiagnostic<'_>,
    right: FdmCommandDiagnostic<'_>,
) -> bool {
    left.candidate_index == right.candidate_index
        && left.entry.row_index() == right.entry.row_index()
        && left.command.command_index() == right.command.command_index()
}

pub(super) fn fdm_command_index_is_synthetic_nested(command_index: usize) -> bool {
    command_index >= 1000
}

pub(super) fn fdm_command_parent_command_index(command_index: usize) -> usize {
    if fdm_command_index_is_synthetic_nested(command_index) {
        command_index / 1000
    } else {
        command_index
    }
}

pub(super) fn fdm_command_normalized_parent_relative_offset(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> Option<usize> {
    if !fdm_command_index_is_synthetic_nested(diagnostic.command.command_index()) {
        return Some(diagnostic.command.relative_offset());
    }
    fdm_connector_parent_compound_provenance(diagnostic)
        .map(|provenance| provenance.parent.relative_offset())
}

pub(super) fn fdm_connector_parent_compound_provenance(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> Option<FdmConnectorParentCompoundProvenance<'_>> {
    let command_index = diagnostic.command.command_index();
    if !fdm_command_index_is_synthetic_nested(command_index) {
        return None;
    }
    let parent_command_index = fdm_command_parent_command_index(command_index);
    let parent = diagnostic.entry.vector_commands().iter().find(|command| {
        command.command_index() == parent_command_index
            && command.marker() == FDM_VECTOR_COMMAND_BBOX_MARKER
    })?;
    let child_offset_in_parent = diagnostic
        .command
        .relative_offset()
        .checked_sub(parent.relative_offset())?;
    let child_offset_table_index = parent
        .compound_child_offsets()
        .iter()
        .position(|offset| usize::from(*offset) == child_offset_in_parent);
    Some(FdmConnectorParentCompoundProvenance {
        parent,
        child_offset_in_parent,
        child_offset_table_index,
    })
}

pub(super) fn fdm_connector_graph_diagnostic_summary(
    layout: PageLayout,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
    text_projection: Option<&ShanaiLanTextProjection>,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) -> Option<FdmConnectorGraphDiagnosticSummary> {
    let projection = line_rule_projection?;
    let open_stroke_axis_rules =
        fdm_open_stroke_axis_rules(layout, primitive_diagnostics, extent, projection);
    let mut summary = FdmConnectorGraphDiagnosticSummary {
        page_paint_coverage_summary: fdm_page_paint_coverage_summary(
            layout,
            primitive_diagnostics,
            extent,
        ),
        line_rule_projection_count: projection.rules.len(),
        fdm_open_stroke_axis_rule_projection_count: open_stroke_axis_rules.len(),
        ..Default::default()
    };
    let mut row_summaries: BTreeMap<usize, FdmConnectorMatchedRowDiagnosticSummary> =
        BTreeMap::new();
    let mut open_stroke_axis_rule_row_summaries: BTreeMap<
        usize,
        FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    > = BTreeMap::new();
    let mut strict_order_trace_candidates = Vec::new();
    summary
        .skipped_inline_line_rule_match_summary
        .line_rule_projection_count = projection
        .rules
        .iter()
        .filter(|rule| rule.candidate_source == "skippedInlineLineHeader")
        .count();
    summary
        .vertical_anchor_line_rule_match_summary
        .line_rule_projection_count = projection
        .rules
        .iter()
        .filter(|rule| rule.candidate_source == "verticalAnchorRunFromLineHeaders")
        .count();
    summary
        .fdm_open_stroke_axis_rule_match_summary
        .line_rule_projection_count = open_stroke_axis_rules.len();

    for diagnostic in primitive_diagnostics.iter().copied() {
        let Some(metric) = fdm_connector_candidate_metric(layout, diagnostic, extent) else {
            continue;
        };
        let Some(endpoint_summary) =
            fdm_connector_line_rule_endpoint_match_summary(layout, metric, Some(projection))
        else {
            continue;
        };
        summary.connector_candidate_count += 1;
        summary.connector_endpoint_probe_count += 2;
        summary.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
        summary.tight_endpoint_match_count += endpoint_summary.tight_match_count;
        summary.nearby_endpoint_match_count +=
            endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
        if endpoint_summary.total_match_count > 0 {
            summary.matched_connector_count += 1;
        }
        if endpoint_summary.dual_endpoint_match() {
            summary.dual_endpoint_match_connector_count += 1;
        }
        if endpoint_summary.start_match_count > 0 {
            summary.start_endpoint_line_rule_match_connector_count += 1;
        }
        if endpoint_summary.end_match_count > 0 {
            summary.end_endpoint_line_rule_match_connector_count += 1;
        }
        if endpoint_summary.start_match_count > 0 && endpoint_summary.end_match_count == 0 {
            summary.start_only_line_rule_match_connector_count += 1;
        }
        if endpoint_summary.end_match_count > 0 && endpoint_summary.start_match_count == 0 {
            summary.end_only_line_rule_match_connector_count += 1;
        }
        let row_summary = row_summaries
            .entry(diagnostic.entry.row_index())
            .or_default();
        row_summary.connector_candidate_count += 1;
        row_summary.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
        row_summary.tight_endpoint_match_count += endpoint_summary.tight_match_count;
        row_summary.nearby_endpoint_match_count +=
            endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
        if endpoint_summary.total_match_count > 0 {
            row_summary.matched_connector_count += 1;
        }
        if endpoint_summary.dual_endpoint_match() {
            row_summary.dual_endpoint_match_connector_count += 1;
        }
        if endpoint_summary.start_match_count > 0 && endpoint_summary.end_match_count == 0 {
            row_summary.start_only_match_connector_count += 1;
        }
        if endpoint_summary.end_match_count > 0 && endpoint_summary.start_match_count == 0 {
            row_summary.end_only_match_connector_count += 1;
        }
        if let Some(skipped_inline_endpoint_summary) =
            fdm_connector_line_rule_endpoint_match_summary_for_candidate_source(
                layout,
                metric,
                projection,
                "skippedInlineLineHeader",
            )
        {
            accumulate_fdm_connector_rule_set_match_summary(
                &mut summary.skipped_inline_line_rule_match_summary,
                skipped_inline_endpoint_summary,
            );
        }
        if let Some(vertical_anchor_endpoint_summary) =
            fdm_connector_line_rule_endpoint_match_summary_for_candidate_source(
                layout,
                metric,
                projection,
                "verticalAnchorRunFromLineHeaders",
            )
        {
            accumulate_fdm_connector_rule_set_match_summary(
                &mut summary.vertical_anchor_line_rule_match_summary,
                vertical_anchor_endpoint_summary,
            );
        }
        let open_stroke_endpoint_detail = fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
            layout,
            diagnostic,
            metric,
            projection,
            &open_stroke_axis_rules,
        );
        if let Some(open_stroke_endpoint_detail) = open_stroke_endpoint_detail {
            accumulate_fdm_connector_rule_set_match_summary(
                &mut summary.fdm_open_stroke_axis_rule_match_summary,
                open_stroke_endpoint_detail.summary,
            );
            if open_stroke_endpoint_detail.tight_dual_endpoint_match()
                && metric.orientation != "diagonal"
            {
                strict_order_trace_candidates.push((
                    diagnostic,
                    metric,
                    open_stroke_endpoint_detail,
                ));
            }
            let open_stroke_row_summary = open_stroke_axis_rule_row_summaries
                .entry(diagnostic.entry.row_index())
                .or_insert_with(|| FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
                    row_index: diagnostic.entry.row_index(),
                    ..Default::default()
                });
            accumulate_fdm_open_stroke_axis_rule_row_cohort_summary(
                open_stroke_row_summary,
                diagnostic,
                metric,
                open_stroke_endpoint_detail,
            );
        }
        let owner_summary = fdm_connector_endpoint_owner_match_summary(
            layout,
            diagnostic,
            extent,
            metric,
            primitive_diagnostics,
            text_projection,
        );
        if owner_summary.total_candidate_count > 0 {
            summary.endpoint_owner_candidate_connector_count += 1;
        }
        summary.endpoint_owner_probe_count += 2;
        summary.total_endpoint_owner_candidate_count += owner_summary.total_candidate_count;
        summary.within_probe_endpoint_owner_candidate_count +=
            owner_summary.within_probe_candidate_count;
        summary.fdm_primitive_endpoint_owner_candidate_count +=
            owner_summary.fdm_primitive_candidate_count;
        summary.document_text_slot_endpoint_owner_candidate_count +=
            owner_summary.document_text_slot_candidate_count;
        if owner_summary.start_within_probe_count > 0 {
            summary.start_endpoint_owner_within_probe_connector_count += 1;
        }
        if owner_summary.end_within_probe_count > 0 {
            summary.end_endpoint_owner_within_probe_connector_count += 1;
        }
        if owner_summary.dual_endpoint_owner_candidate() {
            summary.dual_endpoint_owner_within_probe_connector_count += 1;
        }
        if owner_summary.nearest_fdm_owner_rows_match {
            summary.dual_endpoint_nearest_fdm_owner_same_row_connector_count += 1;
        } else if owner_summary.dual_endpoint_owner_candidate() {
            summary.dual_endpoint_nearest_fdm_owner_row_mismatch_connector_count += 1;
        }
        if owner_summary.nearest_fdm_owner_row_matches_connector_row {
            summary.dual_endpoint_nearest_fdm_owner_same_connector_row_count += 1;
        }
        if owner_summary.connector_command_between_nearest_fdm_owner_commands {
            summary.connector_command_between_nearest_fdm_owner_commands_count += 1;
        }
        if owner_summary.connector_command_before_nearest_fdm_owner_commands {
            summary.connector_command_before_nearest_fdm_owner_commands_count += 1;
        }
        if owner_summary.connector_command_after_nearest_fdm_owner_commands {
            summary.connector_command_after_nearest_fdm_owner_commands_count += 1;
        }
        if open_stroke_endpoint_detail
            .map(|detail| detail.summary.dual_endpoint_match())
            .unwrap_or(false)
        {
            accumulate_fdm_open_stroke_axis_rule_owner_promotion_gate_summary(
                &mut summary.fdm_open_stroke_axis_rule_owner_promotion_gate_summary,
                owner_summary,
            );
            let open_stroke_row_summary = open_stroke_axis_rule_row_summaries
                .entry(diagnostic.entry.row_index())
                .or_insert_with(|| FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary {
                    row_index: diagnostic.entry.row_index(),
                    ..Default::default()
                });
            accumulate_fdm_open_stroke_axis_rule_owner_promotion_gate_summary(
                &mut open_stroke_row_summary.owner_promotion_gate_summary,
                owner_summary,
            );
        }
        if owner_summary.parent_normalized_ordered_same_row_same_connector() {
            summary.parent_normalized_ordered_same_row_same_connector_count += 1;
            accumulate_fdm_connector_owner_row_cohort_match_summary(
                &mut summary.parent_normalized_ordered_owner_row_match_summary,
                endpoint_summary,
            );
        } else {
            accumulate_fdm_connector_owner_row_cohort_match_summary(
                &mut summary.parent_normalized_non_ordered_owner_row_match_summary,
                endpoint_summary,
            );
        }
        accumulate_fdm_connector_owner_command_relation_match_summary(
            &mut summary,
            owner_summary.owner_command_relation(),
            endpoint_summary,
        );
        accumulate_fdm_connector_owner_source_order_relation_match_summary(
            &mut summary,
            owner_summary.owner_source_order_relation(),
            endpoint_summary,
        );
        if owner_summary.nearest_fdm_owner_rows_match
            && owner_summary.nearest_fdm_owner_row_matches_connector_row
            && !owner_summary.mixed_top_level_vs_nested_order_namespace
            && owner_summary.connector_command_between_nearest_fdm_owner_commands
        {
            summary.ordered_same_row_same_connector_count += 1;
            accumulate_fdm_connector_owner_row_cohort_match_summary(
                &mut summary.ordered_owner_row_match_summary,
                endpoint_summary,
            );
        } else {
            accumulate_fdm_connector_owner_row_cohort_match_summary(
                &mut summary.non_ordered_owner_row_match_summary,
                endpoint_summary,
            );
        }
        match owner_summary.owner_grouping_promotion_blocked_reason() {
            "missing-endpoint-owner-candidate" => {
                summary.missing_endpoint_owner_candidate_connector_count += 1;
            }
            "nearest-owner-row-mismatch" => {
                summary.nearest_owner_row_mismatch_connector_count += 1;
            }
            "owner-row-candidate-unproven" => {
                summary.owner_row_candidate_unproven_connector_count += 1;
            }
            _ => {}
        }
        match endpoint_summary.graph_promotion_blocked_reason() {
            "no-thresholded-line-rule-endpoint-match" => {
                summary.no_thresholded_line_rule_endpoint_match_connector_count += 1;
            }
            "single-or-missing-endpoint-line-rule-match" => {
                summary.single_or_missing_endpoint_line_rule_match_connector_count += 1;
            }
            "connector-ownership-and-paint-order-unproven" => {
                summary.connector_ownership_and_paint_order_unproven_connector_count += 1;
            }
            _ => {}
        }
    }

    if let Some((row_index, row_summary)) = row_summaries
        .iter()
        .filter(|(_, row_summary)| row_summary.matched_connector_count > 0)
        .max_by(|(left_row_index, left), (right_row_index, right)| {
            left.matched_connector_count
                .cmp(&right.matched_connector_count)
                .then(
                    left.total_thresholded_endpoint_match_count
                        .cmp(&right.total_thresholded_endpoint_match_count),
                )
                .then(
                    left.dual_endpoint_match_connector_count
                        .cmp(&right.dual_endpoint_match_connector_count),
                )
                .then(
                    left.tight_endpoint_match_count
                        .cmp(&right.tight_endpoint_match_count),
                )
                .then_with(|| right_row_index.cmp(left_row_index))
        })
    {
        summary.dominant_matched_connector_row_index = Some(*row_index);
        summary.dominant_matched_connector_row_connector_candidate_count =
            row_summary.connector_candidate_count;
        summary.dominant_matched_connector_row_total_thresholded_endpoint_match_count =
            row_summary.total_thresholded_endpoint_match_count;
        summary.dominant_matched_connector_row_matched_connector_count =
            row_summary.matched_connector_count;
        summary.dominant_matched_connector_row_dual_endpoint_match_connector_count =
            row_summary.dual_endpoint_match_connector_count;
        summary.dominant_matched_connector_row_start_only_match_connector_count =
            row_summary.start_only_match_connector_count;
        summary.dominant_matched_connector_row_end_only_match_connector_count =
            row_summary.end_only_match_connector_count;
        summary.dominant_matched_connector_row_tight_endpoint_match_count =
            row_summary.tight_endpoint_match_count;
        summary.dominant_matched_connector_row_nearby_endpoint_match_count =
            row_summary.nearby_endpoint_match_count;
    }

    let mut open_stroke_axis_rule_row_cohorts = open_stroke_axis_rule_row_summaries
        .into_values()
        .collect::<Vec<_>>();
    open_stroke_axis_rule_row_cohorts.sort_by(|left, right| {
        right
            .dual_endpoint_match_connector_count
            .cmp(&left.dual_endpoint_match_connector_count)
            .then(
                right
                    .non_diagonal_dual_endpoint_match_connector_count()
                    .cmp(&left.non_diagonal_dual_endpoint_match_connector_count()),
            )
            .then(
                right
                    .tight_dual_endpoint_match_connector_count
                    .cmp(&left.tight_dual_endpoint_match_connector_count),
            )
            .then(
                right
                    .total_thresholded_endpoint_match_count
                    .cmp(&left.total_thresholded_endpoint_match_count),
            )
            .then_with(|| left.row_index.cmp(&right.row_index))
    });
    summary.fdm_open_stroke_axis_rule_row_cohort_count = open_stroke_axis_rule_row_cohorts
        .len()
        .min(FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT);
    for (index, row_summary) in open_stroke_axis_rule_row_cohorts
        .into_iter()
        .take(FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT)
        .enumerate()
    {
        summary.fdm_open_stroke_axis_rule_row_cohorts[index] = row_summary;
    }
    summary.same_row_axis_rule_connector_order_trace_summary = fdm_connector_order_trace_summary(
        layout,
        &strict_order_trace_candidates,
        primitive_diagnostics,
        extent,
        text_projection,
    );

    if summary.connector_candidate_count == 0 {
        None
    } else {
        Some(summary)
    }
}

pub(super) fn accumulate_fdm_connector_rule_set_match_summary(
    target: &mut FdmConnectorRuleSetMatchDiagnosticSummary,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    target.connector_candidate_count += 1;
    target.connector_endpoint_probe_count += 2;
    target.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
    target.tight_endpoint_match_count += endpoint_summary.tight_match_count;
    target.nearby_endpoint_match_count +=
        endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
    if endpoint_summary.total_match_count > 0 {
        target.matched_connector_count += 1;
    }
    if endpoint_summary.dual_endpoint_match() {
        target.dual_endpoint_match_connector_count += 1;
    }
    match endpoint_summary.graph_promotion_blocked_reason() {
        "no-thresholded-line-rule-endpoint-match" => {
            target.no_thresholded_line_rule_endpoint_match_connector_count += 1;
        }
        "single-or-missing-endpoint-line-rule-match" => {
            target.single_or_missing_endpoint_line_rule_match_connector_count += 1;
        }
        "connector-ownership-and-paint-order-unproven" => {
            target.connector_ownership_and_paint_order_unproven_connector_count += 1;
        }
        _ => {}
    }
}

pub(super) fn accumulate_fdm_open_stroke_axis_rule_row_cohort_summary(
    target: &mut FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    connector: FdmCommandDiagnostic<'_>,
    metric: FdmConnectorCandidateMetric,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
) {
    let endpoint_summary = detail.summary;
    accumulate_fdm_open_stroke_axis_rule_row_cohort_segment_gate(
        target,
        connector,
        endpoint_summary,
    );
    target.connector_candidate_count += 1;
    target.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
    target.tight_endpoint_match_count += endpoint_summary.tight_match_count;
    target.nearby_endpoint_match_count +=
        endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
    if endpoint_summary.total_match_count > 0 {
        target.matched_connector_count += 1;
        accumulate_fdm_open_stroke_marker_style_profile(
            &mut target.matched_connector_marker_style_profile,
            connector.command,
        );
        accumulate_fdm_open_stroke_marker_style_profile_from_profile(
            &mut target.axis_rule_endpoint_match_marker_style_profile,
            detail.axis_rule_endpoint_match_marker_style_profile,
        );
        accumulate_projected_bbox_union_milli(
            &mut target.matched_projected_bbox_x_min_milli,
            &mut target.matched_projected_bbox_y_min_milli,
            &mut target.matched_projected_bbox_x_max_milli,
            &mut target.matched_projected_bbox_y_max_milli,
            metric.projected_bbox,
        );
    }
    if !endpoint_summary.dual_endpoint_match() {
        return;
    }

    target.dual_endpoint_match_connector_count += 1;
    accumulate_fdm_open_stroke_axis_rule_source_order_gate(target, connector, detail);
    accumulate_fdm_open_stroke_marker_style_profile(
        &mut target.dual_connector_marker_style_profile,
        connector.command,
    );
    accumulate_projected_bbox_union_milli(
        &mut target.dual_projected_bbox_x_min_milli,
        &mut target.dual_projected_bbox_y_min_milli,
        &mut target.dual_projected_bbox_x_max_milli,
        &mut target.dual_projected_bbox_y_max_milli,
        metric.projected_bbox,
    );
    match metric.orientation {
        "horizontal" => {
            target.horizontal_dual_endpoint_match_connector_count += 1;
            if detail.tight_dual_endpoint_match() {
                target.horizontal_tight_dual_endpoint_match_connector_count += 1;
            }
        }
        "vertical" => {
            target.vertical_dual_endpoint_match_connector_count += 1;
            if detail.tight_dual_endpoint_match() {
                target.vertical_tight_dual_endpoint_match_connector_count += 1;
            }
        }
        _ => {
            target.diagonal_dual_endpoint_match_connector_count += 1;
            if detail.tight_dual_endpoint_match() {
                target.diagonal_tight_dual_endpoint_match_connector_count += 1;
            }
        }
    }
    if detail.tight_dual_endpoint_match() {
        target.tight_dual_endpoint_match_connector_count += 1;
        if metric.orientation != "diagonal" {
            accumulate_fdm_open_stroke_marker_style_profile(
                &mut target.tight_non_diagonal_dual_connector_marker_style_profile,
                connector.command,
            );
            accumulate_projected_bbox_union_milli(
                &mut target.tight_non_diagonal_dual_projected_bbox_x_min_milli,
                &mut target.tight_non_diagonal_dual_projected_bbox_y_min_milli,
                &mut target.tight_non_diagonal_dual_projected_bbox_x_max_milli,
                &mut target.tight_non_diagonal_dual_projected_bbox_y_max_milli,
                metric.projected_bbox,
            );
        }
    }
}

pub(super) fn accumulate_fdm_open_stroke_axis_rule_source_order_gate(
    target: &mut FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    connector: FdmCommandDiagnostic<'_>,
    detail: FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail,
) {
    let Some(connector_parent_relative_offset) =
        fdm_command_normalized_parent_relative_offset(connector)
    else {
        target.dual_endpoint_connector_axis_rule_parent_span_unclassified_count += 1;
        return;
    };
    let (Some(axis_rule_min), Some(axis_rule_max)) = (
        detail.axis_rule_match_parent_relative_offset_min,
        detail.axis_rule_match_parent_relative_offset_max,
    ) else {
        target.dual_endpoint_connector_axis_rule_parent_span_unclassified_count += 1;
        return;
    };

    target.dual_endpoint_axis_rule_source_order_backed_connector_count += 1;
    accumulate_usize_range(
        &mut target.dual_endpoint_connector_parent_relative_offset_min,
        &mut target.dual_endpoint_connector_parent_relative_offset_max,
        connector_parent_relative_offset,
    );
    accumulate_usize_range(
        &mut target.dual_endpoint_axis_rule_parent_relative_offset_min,
        &mut target.dual_endpoint_axis_rule_parent_relative_offset_max,
        axis_rule_min,
    );
    accumulate_usize_range(
        &mut target.dual_endpoint_axis_rule_parent_relative_offset_min,
        &mut target.dual_endpoint_axis_rule_parent_relative_offset_max,
        axis_rule_max,
    );

    if connector_parent_relative_offset < axis_rule_min {
        target.dual_endpoint_connector_before_axis_rule_parent_span_count += 1;
    } else if connector_parent_relative_offset > axis_rule_max {
        target.dual_endpoint_connector_after_axis_rule_parent_span_count += 1;
    } else {
        target.dual_endpoint_connector_between_axis_rule_parent_span_count += 1;
    }
}

pub(super) fn accumulate_fdm_open_stroke_axis_rule_row_cohort_segment_gate(
    target: &mut FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    connector: FdmCommandDiagnostic<'_>,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    accumulate_fdm_open_stroke_axis_rule_row_cohort_bbox_relation_gate(
        target,
        connector,
        endpoint_summary,
    );

    if target.fdm_index_vector_offset.is_none() {
        target.fdm_index_vector_offset = Some(connector.entry.vector_offset());
    }
    if target.fdm_index_vector_len.is_none() {
        target.fdm_index_vector_len = Some(connector.entry.vector_len());
    }
    if target.fdm_index_vector_command_count.is_none() {
        let vector_command_count = connector.entry.vector_commands().len();
        let connector_candidate_count = connector.entry.connector_candidates().len();
        target.fdm_index_vector_command_count = Some(vector_command_count);
        target.fdm_index_connector_candidate_count = Some(connector_candidate_count);
        target.fdm_index_non_connector_command_count =
            Some(vector_command_count.saturating_sub(connector_candidate_count));
    }
    target.fdm_index_valid_vector_offset |= connector.entry.valid_vector_offset();
    target.fdm_index_image_signature_count = target
        .fdm_index_image_signature_count
        .max(connector.entry.image_signature_hits().len());
    target.fdm_index_segment_image_signature_count = target
        .fdm_index_segment_image_signature_count
        .max(connector.entry.segment_image_signature_hits().len());

    let source_segment = connector.command.source_segment();
    if let Some(source_segment) = source_segment {
        if target.fdm_index_source_segment_relative_offset.is_none() {
            target.fdm_index_source_segment_relative_offset =
                Some(source_segment.relative_offset());
        }
        if target.fdm_index_source_segment_command_count.is_none() {
            target.fdm_index_source_segment_command_count =
                Some(usize::from(source_segment.command_count()));
        }
        target.source_segment_backed_connector_count += 1;
        if source_segment.relative_offset() == connector.entry.vector_offset() {
            target.source_segment_matches_index_entry_connector_count += 1;
        }
    } else {
        target.source_segment_missing_connector_count += 1;
    }

    if endpoint_summary.dual_endpoint_match() {
        if let Some(source_segment) = source_segment {
            target.dual_endpoint_source_segment_backed_connector_count += 1;
            if source_segment.relative_offset() == connector.entry.vector_offset() {
                target.dual_endpoint_source_segment_matches_index_entry_connector_count += 1;
            }
        }
        if target.image_bearing_segment_candidate() {
            target.dual_endpoint_image_bearing_segment_connector_count += 1;
        }
    }
}

pub(super) fn accumulate_fdm_open_stroke_axis_rule_row_cohort_bbox_relation_gate(
    target: &mut FdmOpenStrokeAxisRuleRowCohortDiagnosticSummary,
    connector: FdmCommandDiagnostic<'_>,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    let index_bbox = normalize_fdm_index_entry_bbox(connector.entry.bbox());
    if target.fdm_index_bbox_left.is_none() {
        target.fdm_index_bbox_left = Some(index_bbox.0);
        target.fdm_index_bbox_top = Some(index_bbox.1);
        target.fdm_index_bbox_right = Some(index_bbox.2);
        target.fdm_index_bbox_bottom = Some(index_bbox.3);
    }

    let Some(connector_bbox) =
        fdm_vector_command_source_bbox(connector.command).map(normalize_fdm_bbox)
    else {
        target.fdm_index_bbox_source_bbox_missing_connector_count += 1;
        if endpoint_summary.dual_endpoint_match() {
            target.dual_endpoint_fdm_index_bbox_source_bbox_missing_connector_count += 1;
        }
        return;
    };

    let relation = if fdm_bbox_contains(index_bbox, connector_bbox) {
        "contains"
    } else if fdm_bbox_intersects(index_bbox, connector_bbox) {
        "overlaps"
    } else {
        "disjoint"
    };

    match relation {
        "contains" => {
            target.fdm_index_bbox_contains_connector_count += 1;
            if endpoint_summary.dual_endpoint_match() {
                target.dual_endpoint_fdm_index_bbox_contains_connector_count += 1;
            }
        }
        "overlaps" => {
            target.fdm_index_bbox_overlaps_connector_count += 1;
            if endpoint_summary.dual_endpoint_match() {
                target.dual_endpoint_fdm_index_bbox_overlaps_connector_count += 1;
            }
        }
        _ => {
            target.fdm_index_bbox_disjoint_connector_count += 1;
            if endpoint_summary.dual_endpoint_match() {
                target.dual_endpoint_fdm_index_bbox_disjoint_connector_count += 1;
            }
        }
    }
}

pub(super) fn accumulate_fdm_open_stroke_marker_style_profile_from_profile(
    target: &mut FdmOpenStrokeMarkerStyleProfile,
    source: FdmOpenStrokeMarkerStyleProfile,
) {
    target.command_count += source.command_count;
    target.line_marker_count += source.line_marker_count;
    target.path_marker_count += source.path_marker_count;
    target.bezier_marker_count += source.bezier_marker_count;
    target.ellipse_marker_count += source.ellipse_marker_count;
    target.other_marker_count += source.other_marker_count;
    target.style_0000_count += source.style_0000_count;
    target.style_0005_count += source.style_0005_count;
    target.style_0080_count += source.style_0080_count;
    target.style_00a0_count += source.style_00a0_count;
    target.other_style_count += source.other_style_count;
}

pub(super) fn accumulate_fdm_open_stroke_marker_style_profile(
    target: &mut FdmOpenStrokeMarkerStyleProfile,
    command: &ObjectFdmVectorCommandCandidate,
) {
    target.command_count += 1;
    if fdm_vector_marker_is_line(command.marker()) {
        target.line_marker_count += 1;
    } else if FDM_VECTOR_COMMAND_PATH_MARKERS.contains(command.marker()) {
        target.path_marker_count += 1;
    } else if fdm_vector_marker_is_bezier_curve(command.marker()) {
        target.bezier_marker_count += 1;
    } else if FDM_VECTOR_COMMAND_ELLIPSE_MARKERS.contains(command.marker()) {
        target.ellipse_marker_count += 1;
    } else {
        target.other_marker_count += 1;
    }

    match command.style_word() {
        0x0000 => target.style_0000_count += 1,
        0x0005 => target.style_0005_count += 1,
        0x0080 => target.style_0080_count += 1,
        0x00a0 => target.style_00a0_count += 1,
        _ => target.other_style_count += 1,
    }
}

pub(super) fn accumulate_fdm_open_stroke_axis_rule_owner_promotion_gate_summary(
    target: &mut FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary,
    owner_summary: FdmConnectorEndpointOwnerMatchSummary,
) {
    target.dual_endpoint_match_connector_count += 1;
    if owner_summary.dual_endpoint_owner_candidate() {
        target.dual_endpoint_owner_candidate_count += 1;
    }
    if owner_summary.nearest_fdm_owner_rows_match {
        target.nearest_fdm_owner_rows_match_count += 1;
    }
    if owner_summary.nearest_fdm_owner_row_matches_connector_row {
        target.nearest_fdm_owner_row_matches_connector_row_count += 1;
    }
    if owner_summary.mixed_top_level_vs_nested_order_namespace {
        target.mixed_top_level_vs_nested_order_namespace_count += 1;
    }
    if owner_summary.parent_normalized_ordered_same_row_same_connector() {
        target.parent_normalized_ordered_same_row_same_connector_count += 1;
    }

    match owner_summary.owner_parent_command_relation() {
        "missing-endpoint-owner-candidate" => {
            target.missing_endpoint_owner_candidate_count += 1;
        }
        "nearest-owner-row-mismatch" => {
            target.nearest_owner_row_mismatch_count += 1;
        }
        "nearest-owner-row-not-connector-row" => {
            target.nearest_owner_row_not_connector_row_count += 1;
        }
        "same-row-before-owner-parent-command-span" => {
            target.before_owner_parent_command_span_count += 1;
        }
        "same-row-between-owner-parent-command-span" => {
            target.between_owner_parent_command_span_count += 1;
        }
        "same-row-after-owner-parent-command-span" => {
            target.after_owner_parent_command_span_count += 1;
        }
        "same-row-parent-command-relation-unclassified" => {
            target.parent_command_relation_unclassified_count += 1;
        }
        _ => {}
    }
    match owner_summary.owner_parent_source_order_relation() {
        "same-row-before-owner-parent-relative-offset-span" => {
            target.before_owner_parent_relative_offset_span_count += 1;
        }
        "same-row-between-owner-parent-relative-offset-span" => {
            target.between_owner_parent_relative_offset_span_count += 1;
        }
        "same-row-after-owner-parent-relative-offset-span" => {
            target.after_owner_parent_relative_offset_span_count += 1;
        }
        "same-row-parent-relative-offset-relation-unclassified" => {
            target.parent_relative_offset_relation_unclassified_count += 1;
        }
        _ => {}
    }
}

pub(super) fn accumulate_fdm_connector_owner_row_cohort_match_summary(
    target: &mut FdmConnectorOwnerRowCohortDiagnosticSummary,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    target.connector_candidate_count += 1;
    target.total_thresholded_endpoint_match_count += endpoint_summary.total_match_count;
    target.tight_endpoint_match_count += endpoint_summary.tight_match_count;
    target.nearby_endpoint_match_count +=
        endpoint_summary.total_match_count - endpoint_summary.tight_match_count;
    if endpoint_summary.total_match_count > 0 {
        target.matched_connector_count += 1;
    }
    if endpoint_summary.dual_endpoint_match() {
        target.dual_endpoint_match_connector_count += 1;
    }
}

pub(super) fn accumulate_fdm_connector_owner_command_relation_match_summary(
    target: &mut FdmConnectorGraphDiagnosticSummary,
    relation: &'static str,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    let summary = match relation {
        "missing-endpoint-owner-candidate" => {
            &mut target.missing_endpoint_owner_relation_match_summary
        }
        "nearest-owner-row-mismatch" => {
            &mut target.nearest_owner_row_mismatch_relation_match_summary
        }
        "nearest-owner-row-not-connector-row" => {
            &mut target.nearest_owner_row_not_connector_row_relation_match_summary
        }
        "same-row-mixed-command-namespace" => {
            &mut target.same_row_mixed_command_namespace_match_summary
        }
        "same-row-before-owner-command-span" => {
            &mut target.same_row_before_owner_command_span_match_summary
        }
        "same-row-between-owner-command-span" => {
            &mut target.same_row_between_owner_command_span_match_summary
        }
        "same-row-after-owner-command-span" => {
            &mut target.same_row_after_owner_command_span_match_summary
        }
        _ => &mut target.same_row_owner_command_relation_unclassified_match_summary,
    };
    accumulate_fdm_connector_owner_row_cohort_match_summary(summary, endpoint_summary);
}

pub(super) fn accumulate_fdm_connector_owner_source_order_relation_match_summary(
    target: &mut FdmConnectorGraphDiagnosticSummary,
    relation: &'static str,
    endpoint_summary: FdmConnectorLineRuleEndpointMatchSummary,
) {
    let summary = match relation {
        "missing-endpoint-owner-candidate" => {
            &mut target.missing_endpoint_owner_source_order_match_summary
        }
        "nearest-owner-row-mismatch" => {
            &mut target.nearest_owner_row_mismatch_source_order_match_summary
        }
        "nearest-owner-row-not-connector-row" => {
            &mut target.nearest_owner_row_not_connector_row_source_order_match_summary
        }
        "same-row-before-owner-relative-offset-span" => {
            &mut target.same_row_before_owner_relative_offset_span_match_summary
        }
        "same-row-between-owner-relative-offset-span" => {
            &mut target.same_row_between_owner_relative_offset_span_match_summary
        }
        "same-row-after-owner-relative-offset-span" => {
            &mut target.same_row_after_owner_relative_offset_span_match_summary
        }
        _ => &mut target.same_row_relative_offset_relation_unclassified_match_summary,
    };
    accumulate_fdm_connector_owner_row_cohort_match_summary(summary, endpoint_summary);
}

impl FdmConnectorGraphDiagnosticSummary {
    pub(super) fn all_line_rule_match_summary(self) -> FdmConnectorRuleSetMatchDiagnosticSummary {
        FdmConnectorRuleSetMatchDiagnosticSummary {
            line_rule_projection_count: self.line_rule_projection_count,
            connector_candidate_count: self.connector_candidate_count,
            connector_endpoint_probe_count: self.connector_endpoint_probe_count,
            total_thresholded_endpoint_match_count: self.total_thresholded_endpoint_match_count,
            matched_connector_count: self.matched_connector_count,
            dual_endpoint_match_connector_count: self.dual_endpoint_match_connector_count,
            tight_endpoint_match_count: self.tight_endpoint_match_count,
            nearby_endpoint_match_count: self.nearby_endpoint_match_count,
            no_thresholded_line_rule_endpoint_match_connector_count: self
                .no_thresholded_line_rule_endpoint_match_connector_count,
            single_or_missing_endpoint_line_rule_match_connector_count: self
                .single_or_missing_endpoint_line_rule_match_connector_count,
            connector_ownership_and_paint_order_unproven_connector_count: self
                .connector_ownership_and_paint_order_unproven_connector_count,
        }
    }

    pub(super) fn render_promotion_blocked_reason(self) -> &'static str {
        let axis_rule_owner_gate_summary =
            self.fdm_open_stroke_axis_rule_owner_promotion_gate_summary;
        let axis_rule_owner_gate_blocked_reason =
            axis_rule_owner_gate_summary.parent_normalized_order_gate_blocked_reason();
        if self.connector_candidate_count == 0 {
            "no-connector-candidates"
        } else if self.dual_endpoint_match_connector_count == 0
            && self
                .fdm_open_stroke_axis_rule_match_summary
                .dual_endpoint_match_connector_count
                > 0
            && axis_rule_owner_gate_summary.dual_endpoint_match_connector_count > 0
            && axis_rule_owner_gate_blocked_reason != "none"
        {
            axis_rule_owner_gate_blocked_reason
        } else if self.dual_endpoint_match_connector_count == 0
            && self
                .fdm_open_stroke_axis_rule_match_summary
                .dual_endpoint_match_connector_count
                > 0
            && self.parent_normalized_ordered_same_row_same_connector_count == 0
        {
            "same-row-axis-rule-parent-normalized-order-unproven"
        } else if self.dual_endpoint_match_connector_count == 0 {
            "no-dual-endpoint-line-rule-match"
        } else {
            "connector-ownership-grouping-and-paint-order-unproven"
        }
    }
}

pub(super) fn fdm_open_stroke_axis_rules<'a>(
    layout: PageLayout,
    diagnostics: &'a [FdmCommandDiagnostic<'a>],
    extent: FdmCommandProjectionExtent,
    projection: &ShanaiLanLineRuleProjection,
) -> Vec<FdmOpenStrokeAxisRule<'a>> {
    let viewport = fdm_projection_viewport(layout);
    diagnostics
        .iter()
        .copied()
        .filter_map(|diagnostic| {
            if fdm_vector_primitive_is_closed(diagnostic.command)
                || fdm_connector_candidate_metric(layout, diagnostic, extent).is_some()
            {
                return None;
            }

            let (x, y, width, height) = fdm_path_unfiltered_bbox(layout, diagnostic, extent)?;
            if width.max(height) < FDM_OPEN_STROKE_AXIS_RULE_MIN_PROJECTED_SPAN_PX {
                return None;
            }

            if width >= height * 2.0 {
                let center_y = y + height * 0.5;
                Some(FdmOpenStrokeAxisRule {
                    diagnostic,
                    orientation: "horizontal",
                    line_offset_units: (x - viewport.x) / projection.grid_unit_px,
                    line_extent_units: (x + width - viewport.x) / projection.grid_unit_px,
                    group_index: ((center_y - viewport.y) / projection.line_height_px) - 1.0,
                    end_group_index: ((center_y - viewport.y) / projection.line_height_px) - 1.0,
                })
            } else if height >= width * 2.0 {
                let center_x = x + width * 0.5;
                Some(FdmOpenStrokeAxisRule {
                    diagnostic,
                    orientation: "vertical",
                    line_offset_units: (center_x - viewport.x) / projection.grid_unit_px,
                    line_extent_units: (center_x - viewport.x) / projection.grid_unit_px,
                    group_index: ((y - viewport.y) / projection.line_height_px) - 1.0,
                    end_group_index: ((y + height - viewport.y) / projection.line_height_px) - 1.0,
                })
            } else {
                None
            }
        })
        .filter(|rule| {
            rule.line_offset_units.is_finite()
                && rule.line_extent_units.is_finite()
                && rule.group_index.is_finite()
                && rule.end_group_index.is_finite()
        })
        .collect()
}

pub(super) fn fdm_connector_open_stroke_axis_rule_endpoint_match_summary(
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    metric: FdmConnectorCandidateMetric,
    projection: &ShanaiLanLineRuleProjection,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) -> Option<FdmConnectorLineRuleEndpointMatchSummary> {
    fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
        layout, connector, metric, projection, rules,
    )
    .map(|detail| detail.summary)
}

pub(super) fn fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    metric: FdmConnectorCandidateMetric,
    projection: &ShanaiLanLineRuleProjection,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) -> Option<FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail> {
    let viewport = fdm_projection_viewport(layout);
    let start =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)?;
    let end = fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)?;
    let start_matches =
        fdm_connector_open_stroke_axis_rule_endpoint_matches(connector, rules, start);
    let end_matches = fdm_connector_open_stroke_axis_rule_endpoint_matches(connector, rules, end);
    let mut axis_rule_endpoint_match_marker_style_profile =
        FdmOpenStrokeMarkerStyleProfile::default();
    let mut axis_rule_match_parent_relative_offset_min = None;
    let mut axis_rule_match_parent_relative_offset_max = None;
    for (_, rule, _, _) in start_matches.iter().chain(end_matches.iter()) {
        accumulate_fdm_open_stroke_marker_style_profile(
            &mut axis_rule_endpoint_match_marker_style_profile,
            rule.diagnostic.command,
        );
        if let Some(relative_offset) =
            fdm_command_normalized_parent_relative_offset(rule.diagnostic)
        {
            accumulate_usize_range(
                &mut axis_rule_match_parent_relative_offset_min,
                &mut axis_rule_match_parent_relative_offset_max,
                relative_offset,
            );
        }
    }
    let tight_match_count = start_matches
        .iter()
        .chain(end_matches.iter())
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    let start_tight_match_count = start_matches
        .iter()
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    let end_tight_match_count = end_matches
        .iter()
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    Some(FdmConnectorOpenStrokeAxisRuleEndpointMatchDetail {
        summary: FdmConnectorLineRuleEndpointMatchSummary {
            start_match_count: start_matches.len(),
            end_match_count: end_matches.len(),
            total_match_count: start_matches.len() + end_matches.len(),
            tight_match_count,
        },
        start_tight_match_count,
        end_tight_match_count,
        axis_rule_endpoint_match_marker_style_profile,
        axis_rule_match_parent_relative_offset_min,
        axis_rule_match_parent_relative_offset_max,
    })
}

pub(super) fn fdm_connector_open_stroke_axis_rule_endpoint_matches<'a>(
    connector: FdmCommandDiagnostic<'_>,
    rules: &'a [FdmOpenStrokeAxisRule<'a>],
    point: FdmConnectorTextGridPoint,
) -> Vec<(
    usize,
    &'a FdmOpenStrokeAxisRule<'a>,
    FdmConnectorLineRuleDistance,
    &'static str,
)> {
    rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.diagnostic.entry.row_index() == connector.entry.row_index())
        .filter_map(|(rule_index, rule)| {
            let distance = fdm_open_stroke_axis_rule_distance(point, rule);
            if distance.inline_delta > FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS {
                return None;
            }
            let tier =
                if distance.axis_delta <= FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS {
                    "tight"
                } else if distance.axis_delta
                    <= FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS
                {
                    "nearby"
                } else {
                    return None;
                };
            Some((rule_index, rule, distance, tier))
        })
        .collect()
}

pub(super) fn fdm_open_stroke_axis_rule_distance(
    point: FdmConnectorTextGridPoint,
    rule: &FdmOpenStrokeAxisRule<'_>,
) -> FdmConnectorLineRuleDistance {
    let (axis_delta, inline_delta, closest_x_units, closest_group_index) = match rule.orientation {
        "horizontal" => {
            let start = rule.line_offset_units.min(rule.line_extent_units);
            let end = rule.line_offset_units.max(rule.line_extent_units);
            let closest_x = point.x_units.clamp(start, end);
            let inline_delta = if point.x_units < start {
                start - point.x_units
            } else if point.x_units > end {
                point.x_units - end
            } else {
                0.0
            };
            (
                (point.group_index_float - rule.group_index).abs(),
                inline_delta,
                closest_x,
                rule.group_index,
            )
        }
        "vertical" => {
            let start = rule.group_index.min(rule.end_group_index);
            let end = rule.group_index.max(rule.end_group_index);
            let closest_group = point.group_index_float.clamp(start, end);
            let inline_delta = if point.group_index_float < start {
                start - point.group_index_float
            } else if point.group_index_float > end {
                point.group_index_float - end
            } else {
                0.0
            };
            (
                (point.x_units - rule.line_offset_units).abs(),
                inline_delta,
                rule.line_offset_units,
                closest_group,
            )
        }
        _ => (
            (point.x_units - rule.line_offset_units).abs(),
            (point.group_index_float - rule.group_index).abs(),
            rule.line_offset_units,
            rule.group_index,
        ),
    };
    FdmConnectorLineRuleDistance {
        axis_delta,
        inline_delta,
        distance_grid: axis_delta.hypot(inline_delta),
        closest_x_units,
        closest_group_index,
    }
}

pub(super) fn fdm_open_stroke_cohort_summary(
    layout: PageLayout,
    diagnostics: &[FdmCommandDiagnostic<'_>],
    extent: FdmCommandProjectionExtent,
) -> Option<FdmOpenStrokeCohortSummary> {
    let mut summary = FdmOpenStrokeCohortSummary {
        primitive_count: diagnostics.len(),
        ..Default::default()
    };
    let mut rows: BTreeMap<usize, FdmOpenStrokeRowCohortSummary> = BTreeMap::new();

    for diagnostic in diagnostics.iter().copied() {
        if fdm_vector_primitive_is_closed(diagnostic.command) {
            continue;
        }

        let row = rows.entry(diagnostic.entry.row_index()).or_insert_with(|| {
            FdmOpenStrokeRowCohortSummary {
                row_index: diagnostic.entry.row_index(),
                ..Default::default()
            }
        });
        let metric = fdm_connector_candidate_metric(layout, diagnostic, extent);
        let orientation = metric
            .map(|metric| metric.orientation)
            .unwrap_or_else(|| fdm_open_stroke_source_orientation(diagnostic.command));

        summary.open_stroke_count += 1;
        row.open_stroke_count += 1;
        match orientation {
            "horizontal" => {
                summary.horizontal_count += 1;
                row.horizontal_count += 1;
            }
            "vertical" => {
                summary.vertical_count += 1;
                row.vertical_count += 1;
            }
            _ => {
                summary.diagonal_count += 1;
                row.diagonal_count += 1;
            }
        }
        if fdm_vector_marker_is_line(diagnostic.command.marker()) {
            summary.line_marker_count += 1;
            row.line_marker_count += 1;
        } else {
            summary.non_line_marker_count += 1;
            row.non_line_marker_count += 1;
        }
        accumulate_fdm_open_stroke_marker_style_profile(
            &mut row.marker_style_profile,
            diagnostic.command,
        );
        if metric.is_some() {
            summary.connector_candidate_count += 1;
            row.connector_candidate_count += 1;
        }

        update_optional_usize_min_max(
            &mut row.command_index_min,
            &mut row.command_index_max,
            diagnostic.command.command_index(),
        );
        update_optional_usize_min_max(
            &mut row.relative_offset_min,
            &mut row.relative_offset_max,
            diagnostic.command.relative_offset(),
        );
        if let Some(source_bbox) = fdm_vector_command_source_bbox(diagnostic.command) {
            row.source_bbox_union =
                fdm_bbox_extent_union(row.source_bbox_union, normalize_fdm_bbox(source_bbox));
        }
        if let Some(projected_bbox) = fdm_path_unfiltered_bbox(layout, diagnostic, extent) {
            row.projected_bbox_union = bbox_tuple_union(row.projected_bbox_union, projected_bbox);
        }
    }

    if summary.open_stroke_count == 0 {
        return None;
    }

    let mut row_cohorts = rows.into_values().collect::<Vec<_>>();
    summary.row_count = row_cohorts.len();
    row_cohorts.sort_by(|left, right| {
        right
            .connector_candidate_count
            .cmp(&left.connector_candidate_count)
            .then(right.open_stroke_count.cmp(&left.open_stroke_count))
            .then(right.horizontal_count.cmp(&left.horizontal_count))
            .then(right.vertical_count.cmp(&left.vertical_count))
            .then_with(|| left.row_index.cmp(&right.row_index))
    });

    if let Some(row) = row_cohorts
        .iter()
        .find(|row| row.connector_candidate_count > 0)
    {
        summary.dominant_connector_row_index = Some(row.row_index);
        summary.dominant_connector_row_connector_candidate_count = row.connector_candidate_count;
        summary.dominant_connector_row_open_stroke_count = row.open_stroke_count;
        summary.dominant_connector_row_horizontal_count = row.horizontal_count;
        summary.dominant_connector_row_vertical_count = row.vertical_count;
    }

    row_cohorts.truncate(FDM_OPEN_STROKE_ROW_COHORT_LIMIT);
    summary.row_cohorts = row_cohorts;
    Some(summary)
}

pub(super) fn fdm_open_stroke_source_orientation(
    command: &ObjectFdmVectorCommandCandidate,
) -> &'static str {
    let Some(start) = command.path_points().first() else {
        return "diagonal";
    };
    let Some(end) = command.path_points().last() else {
        return "diagonal";
    };
    let dx = end.x().saturating_sub(start.x()) as f32;
    let dy = end.y().saturating_sub(start.y()) as f32;
    fdm_connector_orientation(dx, dy)
}

pub(super) fn push_fdm_connector_source_endpoints_json(
    output: &mut String,
    metric: FdmConnectorCandidateMetric,
) {
    output.push_str("{\"start\":{\"x\":");
    output.push_str(&metric.source_start.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&metric.source_start.y().to_string());
    output.push_str("},\"end\":{\"x\":");
    output.push_str(&metric.source_end.x().to_string());
    output.push_str(",\"y\":");
    output.push_str(&metric.source_end.y().to_string());
    output.push_str("}}");
}

pub(super) fn push_fdm_connector_projected_endpoints_json(
    output: &mut String,
    metric: FdmConnectorCandidateMetric,
) {
    output.push_str(&format!(
        "{{\"start\":{{\"x\":{:.3},\"y\":{:.3}}},\"end\":{{\"x\":{:.3},\"y\":{:.3}}}}}",
        metric.projected_start.0,
        metric.projected_start.1,
        metric.projected_end.0,
        metric.projected_end.1
    ));
}

pub(super) fn push_fdm_connector_projected_text_grid_json(
    output: &mut String,
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    if projection.grid_unit_px <= 0.0 || projection.line_height_px <= 0.0 {
        output.push_str("null");
        return;
    }

    let viewport = fdm_projection_viewport(layout);
    output.push_str("{\"basis\":\"documentTextLineHeaderGrid\",\"start\":");
    if let Some(point) =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
    {
        push_fdm_connector_text_grid_point_json(output, point);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"end\":");
    if let Some(point) =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
    {
        push_fdm_connector_text_grid_point_json(output, point);
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn fdm_connector_projected_text_grid_point(
    point: (f32, f32),
    projection: &ShanaiLanLineRuleProjection,
    viewport: FdmProjectionViewport,
) -> Option<FdmConnectorTextGridPoint> {
    if projection.grid_unit_px <= 0.0 || projection.line_height_px <= 0.0 {
        return None;
    }
    let x_units = (point.0 - viewport.x) / projection.grid_unit_px;
    let group_index_float = ((point.1 - viewport.y) / projection.line_height_px) - 1.0;
    Some(FdmConnectorTextGridPoint {
        x_units,
        group_index_float,
    })
}

pub(super) fn push_fdm_connector_text_grid_point_json(
    output: &mut String,
    point: FdmConnectorTextGridPoint,
) {
    output.push_str(&format!(
        "{{\"xUnits\":{:.3},\"groupIndexFloat\":{:.3}}}",
        point.x_units, point.group_index_float
    ));
}

pub(super) fn push_fdm_connector_line_rule_attachment_candidates_json(
    output: &mut String,
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let viewport = fdm_projection_viewport(layout);
    let Some(start) =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
    else {
        output.push_str("null");
        return;
    };
    let Some(end) =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"basis\":\"documentTextLineHeaderGrid\",\"attachmentProven\":false");
    output.push_str(",\"start\":");
    push_fdm_connector_line_rule_attachment_candidate_json(output, projection, start);
    output.push_str(",\"end\":");
    push_fdm_connector_line_rule_attachment_candidate_json(output, projection, end);
    output.push('}');
}

pub(super) fn push_fdm_connector_line_rule_attachment_candidate_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
) {
    output.push_str("{\"point\":");
    push_fdm_connector_text_grid_point_json(output, point);
    output.push_str(",\"nearestLineRule\":");
    if let Some((rule_index, rule, distance)) =
        fdm_connector_nearest_line_rule_match(projection, point)
    {
        output.push_str("{\"ruleIndex\":");
        output.push_str(&rule_index.to_string());
        output.push_str(",\"distanceGrid\":");
        output.push_str(&format!("{:.3}", distance.distance_grid));
        output.push_str(",\"axisDelta\":");
        output.push_str(&format!("{:.3}", distance.axis_delta));
        output.push_str(",\"inlineDelta\":");
        output.push_str(&format!("{:.3}", distance.inline_delta));
        output.push_str(",\"closestPoint\":");
        push_fdm_connector_text_grid_point_json(
            output,
            FdmConnectorTextGridPoint {
                x_units: distance.closest_x_units,
                group_index_float: distance.closest_group_index,
            },
        );
        output.push_str(",\"orientation\":");
        output.push_str(&json_string(rule.orientation));
        output.push_str(",\"candidateSource\":");
        output.push_str(&json_string(rule.candidate_source));
        output.push_str(",\"groupIndex\":");
        output.push_str(&rule.group_index.to_string());
        output.push_str(",\"endGroupIndex\":");
        output.push_str(&rule.end_group_index.to_string());
        output.push_str(",\"lineOffsetUnits\":");
        output.push_str(&rule.line_offset_units.to_string());
        output.push_str(",\"lineExtentUnits\":");
        output.push_str(&rule.line_extent_units.to_string());
        output.push_str(",\"attachmentProven\":false}");
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(super) fn push_fdm_connector_line_rule_endpoint_matches_json(
    output: &mut String,
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let viewport = fdm_projection_viewport(layout);
    let Some(start) =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
    else {
        output.push_str("null");
        return;
    };
    let Some(end) =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"basis\":\"documentTextLineHeaderGrid\",\"attachmentProven\":false");
    output.push_str(",\"start\":");
    push_fdm_connector_line_rule_endpoint_match_array_json(output, projection, start);
    output.push_str(",\"end\":");
    push_fdm_connector_line_rule_endpoint_match_array_json(output, projection, end);
    output.push('}');
}

pub(super) fn push_fdm_connector_line_rule_endpoint_match_summary_json(
    output: &mut String,
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let Some(summary) =
        fdm_connector_line_rule_endpoint_match_summary(layout, metric, line_rule_projection)
    else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"startMatchCount\":");
    output.push_str(&summary.start_match_count.to_string());
    output.push_str(",\"endMatchCount\":");
    output.push_str(&summary.end_match_count.to_string());
    output.push_str(",\"totalMatchCount\":");
    output.push_str(&summary.total_match_count.to_string());
    output.push_str(",\"tightMatchCount\":");
    output.push_str(&summary.tight_match_count.to_string());
    output.push_str(",\"matchedEndpointCount\":");
    output.push_str(&summary.matched_endpoint_count().to_string());
    output.push_str(",\"dualEndpointMatch\":");
    output.push_str(if summary.dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"graphPromotionBlockedReason\":");
    output.push_str(&json_string(summary.graph_promotion_blocked_reason()));
    output.push('}');
}

pub(super) fn push_fdm_connector_open_stroke_axis_rule_endpoint_matches_json(
    output: &mut String,
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let viewport = fdm_projection_viewport(layout);
    let Some(start) =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)
    else {
        output.push_str("null");
        return;
    };
    let Some(end) =
        fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)
    else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+documentTextLineHeaderGrid\"");
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\"");
    output.push_str(",\"rowScoped\":true,\"attachmentProven\":false");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"",
    );
    output.push_str(",\"start\":");
    push_fdm_connector_open_stroke_axis_rule_endpoint_match_array_json(
        output, layout, extent, connector, rules, start,
    );
    output.push_str(",\"end\":");
    push_fdm_connector_open_stroke_axis_rule_endpoint_match_array_json(
        output, layout, extent, connector, rules, end,
    );
    output.push('}');
}

pub(super) fn push_fdm_connector_open_stroke_axis_rule_endpoint_match_summary_json(
    output: &mut String,
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let Some(detail) = fdm_connector_open_stroke_axis_rule_endpoint_match_detail(
        layout, connector, metric, projection, rules,
    ) else {
        output.push_str("null");
        return;
    };
    let summary = detail.summary;
    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule\"");
    output.push_str(",\"source\":\"fdmVectorCommandPrimitive\"");
    output.push_str(",\"startMatchCount\":");
    output.push_str(&summary.start_match_count.to_string());
    output.push_str(",\"endMatchCount\":");
    output.push_str(&summary.end_match_count.to_string());
    output.push_str(",\"totalMatchCount\":");
    output.push_str(&summary.total_match_count.to_string());
    output.push_str(",\"tightMatchCount\":");
    output.push_str(&summary.tight_match_count.to_string());
    output.push_str(",\"startTightMatchCount\":");
    output.push_str(&detail.start_tight_match_count.to_string());
    output.push_str(",\"endTightMatchCount\":");
    output.push_str(&detail.end_tight_match_count.to_string());
    output.push_str(",\"tightDualEndpointMatch\":");
    output.push_str(if detail.tight_dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedEndpointCount\":");
    output.push_str(&summary.matched_endpoint_count().to_string());
    output.push_str(",\"dualEndpointMatch\":");
    output.push_str(if summary.dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"graphPromotionBlockedReason\":");
    output.push_str(&json_string(summary.graph_promotion_blocked_reason()));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_fdm_connector_open_stroke_axis_rule_owner_promotion_gate_json(
    output: &mut String,
    layout: PageLayout,
    connector: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
    metric: FdmConnectorCandidateMetric,
    primitive_diagnostics: &[FdmCommandDiagnostic<'_>],
    text_projection: Option<&ShanaiLanTextProjection>,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
    rules: &[FdmOpenStrokeAxisRule<'_>],
) {
    let Some(projection) = line_rule_projection else {
        output.push_str("null");
        return;
    };
    let Some(axis_summary) = fdm_connector_open_stroke_axis_rule_endpoint_match_summary(
        layout, connector, metric, projection, rules,
    ) else {
        output.push_str("null");
        return;
    };
    let owner_summary = fdm_connector_endpoint_owner_match_summary(
        layout,
        connector,
        extent,
        metric,
        primitive_diagnostics,
        text_projection,
    );

    output.push_str("{\"basis\":\"sameRowFdmOpenStrokeAxisRule+endpointOwnerMatchSummary\"");
    output.push_str(",\"decoded\":false,\"sourceBacked\":true,\"renderable\":false");
    output.push_str(",\"axisRuleDualEndpointMatch\":");
    output.push_str(if axis_summary.dual_endpoint_match() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"axisRuleMatchedEndpointCount\":");
    output.push_str(&axis_summary.matched_endpoint_count().to_string());
    output.push_str(",\"axisRuleTotalMatchCount\":");
    output.push_str(&axis_summary.total_match_count.to_string());
    output.push_str(",\"axisRuleTightMatchCount\":");
    output.push_str(&axis_summary.tight_match_count.to_string());
    output.push_str(",\"parentNormalizedOrderGateBlockedReason\":");
    output.push_str(&json_string(
        FdmOpenStrokeAxisRuleOwnerPromotionGateDiagnosticSummary {
            dual_endpoint_match_connector_count: if axis_summary.dual_endpoint_match() {
                1
            } else {
                0
            },
            dual_endpoint_owner_candidate_count: if owner_summary.dual_endpoint_owner_candidate() {
                1
            } else {
                0
            },
            nearest_fdm_owner_rows_match_count: if owner_summary.nearest_fdm_owner_rows_match {
                1
            } else {
                0
            },
            nearest_fdm_owner_row_matches_connector_row_count: if owner_summary
                .nearest_fdm_owner_row_matches_connector_row
            {
                1
            } else {
                0
            },
            parent_normalized_ordered_same_row_same_connector_count: if owner_summary
                .parent_normalized_ordered_same_row_same_connector()
            {
                1
            } else {
                0
            },
            between_owner_parent_command_span_count: if owner_summary
                .connector_parent_command_between_nearest_fdm_owner_parent_commands
            {
                1
            } else {
                0
            },
            between_owner_parent_relative_offset_span_count: if owner_summary
                .connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets
            {
                1
            } else {
                0
            },
            ..Default::default()
        }
        .parent_normalized_order_gate_blocked_reason(),
    ));
    output.push_str(",\"dualEndpointOwnerCandidate\":");
    output.push_str(if owner_summary.dual_endpoint_owner_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nearestFdmOwnerRowsMatch\":");
    output.push_str(if owner_summary.nearest_fdm_owner_rows_match {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nearestFdmOwnerRowMatchesConnectorRow\":");
    output.push_str(
        if owner_summary.nearest_fdm_owner_row_matches_connector_row {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"mixedTopLevelVsNestedOrderNamespace\":");
    output.push_str(if owner_summary.mixed_top_level_vs_nested_order_namespace {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"parentNormalizedOrderedSameRowSameConnector\":");
    output.push_str(
        if owner_summary.parent_normalized_ordered_same_row_same_connector() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"ownerCommandRelation\":");
    output.push_str(&json_string(owner_summary.owner_command_relation()));
    output.push_str(",\"ownerSourceOrderRelation\":");
    output.push_str(&json_string(owner_summary.owner_source_order_relation()));
    output.push_str(",\"ownerParentCommandRelation\":");
    output.push_str(&json_string(owner_summary.owner_parent_command_relation()));
    output.push_str(",\"ownerParentSourceOrderRelation\":");
    output.push_str(&json_string(
        owner_summary.owner_parent_source_order_relation(),
    ));
    output.push_str(",\"connectorParentCommandIndex\":");
    output.push_str(&owner_summary.connector_parent_command_index.to_string());
    output.push_str(",\"connectorParentRelativeOffset\":");
    push_option_usize_json(output, owner_summary.connector_parent_relative_offset);
    output.push_str(",\"startNearestFdmOwnerParentRelativeOffset\":");
    push_option_usize_json(
        output,
        owner_summary.start_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"endNearestFdmOwnerParentRelativeOffset\":");
    push_option_usize_json(
        output,
        owner_summary.end_nearest_fdm_owner_parent_relative_offset,
    );
    output.push_str(",\"startNearestFdmOwner\":");
    push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
        output,
        owner_summary.start_nearest_fdm_owner_row_index,
        owner_summary.start_nearest_fdm_owner_command_index,
        owner_summary.start_nearest_fdm_owner_parent_command_index,
        owner_summary.start_nearest_fdm_owner_synthetic_nested_command,
        owner_summary.start_nearest_fdm_owner_relative_offset,
    );
    output.push_str(",\"endNearestFdmOwner\":");
    push_fdm_connector_endpoint_owner_nearest_fdm_owner_json(
        output,
        owner_summary.end_nearest_fdm_owner_row_index,
        owner_summary.end_nearest_fdm_owner_command_index,
        owner_summary.end_nearest_fdm_owner_parent_command_index,
        owner_summary.end_nearest_fdm_owner_synthetic_nested_command,
        owner_summary.end_nearest_fdm_owner_relative_offset,
    );
    output.push_str(",\"connectorCommandBetweenNearestFdmOwnerCommands\":");
    output.push_str(
        if owner_summary.connector_command_between_nearest_fdm_owner_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandBeforeNearestFdmOwnerParentCommands\":");
    output.push_str(
        if owner_summary.connector_parent_command_before_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandBetweenNearestFdmOwnerParentCommands\":");
    output.push_str(
        if owner_summary.connector_parent_command_between_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentCommandAfterNearestFdmOwnerParentCommands\":");
    output.push_str(
        if owner_summary.connector_parent_command_after_nearest_fdm_owner_parent_commands {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorRelativeOffsetBetweenNearestFdmOwnerOffsets\":");
    output.push_str(
        if owner_summary.connector_relative_offset_between_nearest_fdm_owner_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetBeforeNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if owner_summary.connector_parent_relative_offset_before_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetBetweenNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if owner_summary.connector_parent_relative_offset_between_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"connectorParentRelativeOffsetAfterNearestFdmOwnerParentOffsets\":");
    output.push_str(
        if owner_summary.connector_parent_relative_offset_after_nearest_fdm_owner_parent_offsets {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"ownerGroupingProven\":false");
    output.push_str(",\"ownerGroupingPromotionBlockedReason\":");
    output.push_str(&json_string(
        owner_summary.owner_grouping_promotion_blocked_reason(),
    ));
    output.push_str(",\"ownershipPromotionBlockedReason\":");
    output.push_str(&json_string(
        owner_summary.ownership_promotion_blocked_reason(),
    ));
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"connector-ownership-and-paint-order-unproven\"}",
    );
}

pub(super) fn fdm_connector_line_rule_endpoint_match_summary(
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    line_rule_projection: Option<&ShanaiLanLineRuleProjection>,
) -> Option<FdmConnectorLineRuleEndpointMatchSummary> {
    let projection = line_rule_projection?;
    let viewport = fdm_projection_viewport(layout);
    let start =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)?;
    let end = fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)?;
    let start_matches = fdm_connector_line_rule_endpoint_matches(projection, start);
    let end_matches = fdm_connector_line_rule_endpoint_matches(projection, end);
    let tight_match_count = start_matches
        .iter()
        .chain(end_matches.iter())
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    Some(FdmConnectorLineRuleEndpointMatchSummary {
        start_match_count: start_matches.len(),
        end_match_count: end_matches.len(),
        total_match_count: start_matches.len() + end_matches.len(),
        tight_match_count,
    })
}

pub(super) fn fdm_connector_line_rule_endpoint_match_summary_for_candidate_source(
    layout: PageLayout,
    metric: FdmConnectorCandidateMetric,
    projection: &ShanaiLanLineRuleProjection,
    candidate_source: &'static str,
) -> Option<FdmConnectorLineRuleEndpointMatchSummary> {
    let viewport = fdm_projection_viewport(layout);
    let start =
        fdm_connector_projected_text_grid_point(metric.projected_start, projection, viewport)?;
    let end = fdm_connector_projected_text_grid_point(metric.projected_end, projection, viewport)?;
    let start_matches = fdm_connector_line_rule_endpoint_matches_for_candidate_source(
        projection,
        start,
        candidate_source,
    );
    let end_matches = fdm_connector_line_rule_endpoint_matches_for_candidate_source(
        projection,
        end,
        candidate_source,
    );
    let tight_match_count = start_matches
        .iter()
        .chain(end_matches.iter())
        .filter(|(_, _, _, tier)| *tier == "tight")
        .count();
    Some(FdmConnectorLineRuleEndpointMatchSummary {
        start_match_count: start_matches.len(),
        end_match_count: end_matches.len(),
        total_match_count: start_matches.len() + end_matches.len(),
        tight_match_count,
    })
}

impl FdmConnectorLineRuleEndpointMatchSummary {
    pub(super) fn matched_endpoint_count(self) -> usize {
        usize::from(self.start_match_count > 0) + usize::from(self.end_match_count > 0)
    }

    pub(super) fn dual_endpoint_match(self) -> bool {
        self.start_match_count > 0 && self.end_match_count > 0
    }

    pub(super) fn graph_promotion_blocked_reason(self) -> &'static str {
        if self.total_match_count == 0 {
            "no-thresholded-line-rule-endpoint-match"
        } else if !self.dual_endpoint_match() {
            "single-or-missing-endpoint-line-rule-match"
        } else {
            "connector-ownership-and-paint-order-unproven"
        }
    }
}

pub(super) fn push_fdm_connector_line_rule_endpoint_match_array_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
) {
    let mut matches = fdm_connector_line_rule_endpoint_matches(projection, point);
    matches.sort_by(|left, right| {
        fdm_connector_line_rule_tier_rank(left.3)
            .cmp(&fdm_connector_line_rule_tier_rank(right.3))
            .then_with(|| {
                left.2
                    .axis_delta
                    .partial_cmp(&right.2.axis_delta)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| {
                left.2
                    .inline_delta
                    .partial_cmp(&right.2.inline_delta)
                    .unwrap_or(Ordering::Equal)
            })
    });

    output.push('[');
    for (match_index, (rule_index, rule, distance, tier)) in matches.iter().enumerate() {
        if match_index > 0 {
            output.push(',');
        }
        output.push_str("{\"ruleIndex\":");
        output.push_str(&rule_index.to_string());
        output.push_str(",\"tier\":");
        output.push_str(&json_string(tier));
        output.push_str(",\"perpendicularGroupDelta\":");
        output.push_str(&format!("{:.3}", distance.axis_delta));
        output.push_str(",\"spanOverflowUnits\":");
        output.push_str(&format!("{:.3}", distance.inline_delta));
        output.push_str(",\"inSpanAxis\":");
        output.push_str(if distance.inline_delta <= f32::EPSILON {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"closestPoint\":");
        push_fdm_connector_text_grid_point_json(
            output,
            FdmConnectorTextGridPoint {
                x_units: distance.closest_x_units,
                group_index_float: distance.closest_group_index,
            },
        );
        output.push_str(",\"orientation\":");
        output.push_str(&json_string(rule.orientation));
        output.push_str(",\"candidateSource\":");
        output.push_str(&json_string(rule.candidate_source));
        output.push_str(",\"attachmentProven\":false}");
    }
    output.push(']');
}

pub(super) fn push_fdm_connector_open_stroke_axis_rule_endpoint_match_array_json(
    output: &mut String,
    layout: PageLayout,
    extent: FdmCommandProjectionExtent,
    connector: FdmCommandDiagnostic<'_>,
    rules: &[FdmOpenStrokeAxisRule<'_>],
    point: FdmConnectorTextGridPoint,
) {
    let mut matches = fdm_connector_open_stroke_axis_rule_endpoint_matches(connector, rules, point);
    matches.sort_by(|left, right| {
        fdm_connector_line_rule_tier_rank(left.3)
            .cmp(&fdm_connector_line_rule_tier_rank(right.3))
            .then_with(|| {
                left.2
                    .axis_delta
                    .partial_cmp(&right.2.axis_delta)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| {
                left.2
                    .inline_delta
                    .partial_cmp(&right.2.inline_delta)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.0.cmp(&right.0))
    });

    output.push('[');
    for (match_index, (rule_index, rule, distance, tier)) in matches.iter().enumerate() {
        if match_index > 0 {
            output.push(',');
        }
        output.push_str("{\"axisRuleIndex\":");
        output.push_str(&rule_index.to_string());
        output.push_str(",\"tier\":");
        output.push_str(&json_string(tier));
        output.push_str(",\"perpendicularGroupDelta\":");
        output.push_str(&format!("{:.3}", distance.axis_delta));
        output.push_str(",\"spanOverflowUnits\":");
        output.push_str(&format!("{:.3}", distance.inline_delta));
        output.push_str(",\"distanceGrid\":");
        output.push_str(&format!("{:.3}", distance.distance_grid));
        output.push_str(",\"inSpanAxis\":");
        output.push_str(if distance.inline_delta <= f32::EPSILON {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"closestPoint\":");
        push_fdm_connector_text_grid_point_json(
            output,
            FdmConnectorTextGridPoint {
                x_units: distance.closest_x_units,
                group_index_float: distance.closest_group_index,
            },
        );
        output.push_str(",\"orientation\":");
        output.push_str(&json_string(rule.orientation));
        output.push_str(",\"ruleRowIndex\":");
        output.push_str(&rule.diagnostic.entry.row_index().to_string());
        output.push_str(",\"ruleCommandIndex\":");
        output.push_str(&rule.diagnostic.command.command_index().to_string());
        output.push_str(",\"ruleRelativeOffset\":");
        output.push_str(&rule.diagnostic.command.relative_offset().to_string());
        push_fdm_vector_command_provenance_json(output, rule.diagnostic.command);
        output.push_str(",\"ruleMarkerHex\":");
        output.push_str(&json_string(&hex_bytes(rule.diagnostic.command.marker())));
        output.push_str(",\"ruleStyleWord\":");
        output.push_str(&rule.diagnostic.command.style_word().to_string());
        output.push_str(",\"ruleStyleWordHex\":");
        output.push_str(&json_string(&format!(
            "0x{:04x}",
            rule.diagnostic.command.style_word()
        )));
        output.push_str(",\"groupIndex\":");
        output.push_str(&format!("{:.3}", rule.group_index));
        output.push_str(",\"endGroupIndex\":");
        output.push_str(&format!("{:.3}", rule.end_group_index));
        output.push_str(",\"lineOffsetUnits\":");
        output.push_str(&format!("{:.3}", rule.line_offset_units));
        output.push_str(",\"lineExtentUnits\":");
        output.push_str(&format!("{:.3}", rule.line_extent_units));
        output.push_str(",\"projectedBbox\":");
        if let Some(bbox) = fdm_path_unfiltered_bbox(layout, rule.diagnostic, extent) {
            push_bbox_tuple_json(output, bbox);
        } else {
            output.push_str("null");
        }
        output.push_str(",\"sourcePathBbox\":");
        if let Some(bbox) = fdm_vector_command_source_bbox(rule.diagnostic.command) {
            push_object_fdm_index_bbox_json(output, bbox);
        } else {
            output.push_str("null");
        }
        output.push_str(",\"sameRowAsConnector\":true,\"attachmentProven\":false}");
    }
    output.push(']');
}

pub(super) fn fdm_connector_line_rule_endpoint_matches(
    projection: &ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
) -> Vec<(
    usize,
    &ShanaiLanLineRule,
    FdmConnectorLineRuleDistance,
    &'static str,
)> {
    projection
        .rules
        .iter()
        .enumerate()
        .filter_map(|(rule_index, rule)| {
            let distance = fdm_connector_line_rule_distance(point, rule);
            if distance.inline_delta > FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS {
                return None;
            }
            let tier =
                if distance.axis_delta <= FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS {
                    "tight"
                } else if distance.axis_delta
                    <= FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS
                {
                    "nearby"
                } else {
                    return None;
                };
            Some((rule_index, rule, distance, tier))
        })
        .collect()
}

pub(super) fn fdm_connector_line_rule_endpoint_matches_for_candidate_source<'a>(
    projection: &'a ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
    candidate_source: &'static str,
) -> Vec<(
    usize,
    &'a ShanaiLanLineRule,
    FdmConnectorLineRuleDistance,
    &'static str,
)> {
    projection
        .rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.candidate_source == candidate_source)
        .filter_map(|(rule_index, rule)| {
            let distance = fdm_connector_line_rule_distance(point, rule);
            if distance.inline_delta > FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS {
                return None;
            }
            let tier =
                if distance.axis_delta <= FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS {
                    "tight"
                } else if distance.axis_delta
                    <= FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS
                {
                    "nearby"
                } else {
                    return None;
                };
            Some((rule_index, rule, distance, tier))
        })
        .collect()
}

pub(super) fn fdm_connector_line_rule_tier_rank(tier: &str) -> usize {
    match tier {
        "tight" => 0,
        "nearby" => 1,
        _ => 2,
    }
}

pub(super) fn fdm_connector_nearest_line_rule_match(
    projection: &ShanaiLanLineRuleProjection,
    point: FdmConnectorTextGridPoint,
) -> Option<(usize, &ShanaiLanLineRule, FdmConnectorLineRuleDistance)> {
    projection
        .rules
        .iter()
        .enumerate()
        .map(|(rule_index, rule)| {
            (
                rule_index,
                rule,
                fdm_connector_line_rule_distance(point, rule),
            )
        })
        .min_by(|left, right| {
            left.2
                .distance_grid
                .partial_cmp(&right.2.distance_grid)
                .unwrap_or(Ordering::Equal)
        })
}

pub(super) fn fdm_connector_line_rule_distance(
    point: FdmConnectorTextGridPoint,
    rule: &ShanaiLanLineRule,
) -> FdmConnectorLineRuleDistance {
    let (axis_delta, inline_delta, closest_x_units, closest_group_index) = match rule.orientation {
        "horizontal" => {
            let line_group = rule.group_index as f32;
            let start = f32::from(rule.line_offset_units.min(rule.line_extent_units));
            let end = f32::from(rule.line_offset_units.max(rule.line_extent_units));
            let closest_x = point.x_units.clamp(start, end);
            let inline_delta = if point.x_units < start {
                start - point.x_units
            } else if point.x_units > end {
                point.x_units - end
            } else {
                0.0
            };
            (
                (point.group_index_float - line_group).abs(),
                inline_delta,
                closest_x,
                line_group,
            )
        }
        "vertical" => {
            let line_x = f32::from(rule.line_offset_units);
            let start = rule.group_index.min(rule.end_group_index) as f32;
            let end = rule.group_index.max(rule.end_group_index) as f32;
            let closest_group = point.group_index_float.clamp(start, end);
            let inline_delta = if point.group_index_float < start {
                start - point.group_index_float
            } else if point.group_index_float > end {
                point.group_index_float - end
            } else {
                0.0
            };
            (
                (point.x_units - line_x).abs(),
                inline_delta,
                line_x,
                closest_group,
            )
        }
        _ => (
            (point.x_units - f32::from(rule.line_offset_units)).abs(),
            (point.group_index_float - rule.group_index as f32).abs(),
            f32::from(rule.line_offset_units),
            rule.group_index as f32,
        ),
    };
    FdmConnectorLineRuleDistance {
        axis_delta,
        inline_delta,
        distance_grid: axis_delta.hypot(inline_delta),
        closest_x_units,
        closest_group_index,
    }
}

pub(super) fn push_fdm_projection_viewport_json(output: &mut String, layout: PageLayout) {
    let viewport = fdm_projection_viewport(layout);
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        viewport.x, viewport.y, viewport.width, viewport.height
    ));
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmSourceCohort {
    pub(super) command_relative_offsets: Vec<usize>,
    pub(super) source_vector_offset_start: Option<usize>,
    pub(super) source_vector_offset_end: Option<usize>,
    pub(super) source_vector_offset_count: usize,
    pub(super) segment_backed_count: usize,
    pub(super) raw_span_count: usize,
    pub(super) segment_offsets: Vec<usize>,
}

impl SuccessDataTestFdmSourceCohort {
    pub(super) fn blocked_reason(&self) -> &'static str {
        if self.raw_span_count > 0 && self.segment_backed_count > 0 {
            "mixed-raw-and-segment-cohorts"
        } else if self.segment_offsets.len() > 1 {
            "multiple-source-segment-cohorts"
        } else {
            "source-owner-candidate-unproven"
        }
    }
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmPrimitiveOwnershipClassification<'a> {
    pub(super) command: &'a ObjectFdmVectorCommandCandidate,
    pub(super) role_candidates: Vec<&'static str>,
    pub(super) classification_basis: Vec<&'static str>,
    pub(super) index_row_references: Vec<SuccessDataTestFdmIndexRowReference>,
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmIndexRowReference {
    pub(super) row_index: usize,
    pub(super) index_offset: usize,
    pub(super) vector_offset: usize,
    pub(super) valid_vector_offset: bool,
    pub(super) offset_field: &'static str,
    pub(super) offset_value: usize,
    pub(super) match_kind: &'static str,
}

#[derive(Debug, Default)]
pub(super) struct SuccessDataTestFdmIndexRowOrderPromotionGate {
    pub(super) command_count: usize,
    pub(super) referenced_command_relative_offsets: BTreeSet<usize>,
    pub(super) referenced_row_indexes: BTreeSet<usize>,
    pub(super) row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
    pub(super) row_to_command_relative_offsets: BTreeMap<usize, BTreeSet<usize>>,
    pub(super) reference_count: usize,
    pub(super) valid_vector_offset_reference_count: usize,
    pub(super) command_relative_offset_field_reference_count: usize,
    pub(super) source_segment_relative_offset_field_reference_count: usize,
}

impl SuccessDataTestFdmIndexRowOrderPromotionGate {
    pub(super) fn referenced_command_count(&self) -> usize {
        self.referenced_command_relative_offsets.len()
    }

    pub(super) fn unreferenced_command_count(&self) -> usize {
        self.command_count
            .saturating_sub(self.referenced_command_count())
    }

    pub(super) fn unique_row_index_count(&self) -> usize {
        self.referenced_row_indexes.len()
    }

    pub(super) fn all_commands_referenced_by_index_rows_candidate(&self) -> bool {
        self.command_count > 0 && self.unreferenced_command_count() == 0
    }

    pub(super) fn one_to_one_row_command_reference_candidate(&self) -> bool {
        self.reference_count == self.referenced_command_count()
            && self.reference_count == self.unique_row_index_count()
    }

    pub(super) fn single_row_backs_multiple_commands_candidate(&self) -> bool {
        self.row_to_command_relative_offsets
            .values()
            .any(|offsets| offsets.len() > 1)
    }

    pub(super) fn row_order_matches_command_order_candidate(&self) -> bool {
        success_data_test_fdm_row_command_pairs_are_monotonic(&self.row_command_pairs)
    }
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmOffsetFieldAuthorityGate {
    pub(super) command_count: usize,
    pub(super) reference_count: usize,
    pub(super) valid_vector_offset_reference_count: usize,
    pub(super) command_relative_offset_field_reference_count: usize,
    pub(super) source_segment_relative_offset_field_reference_count: usize,
    pub(super) unclassified_offset_field_reference_count: usize,
    pub(super) raw_span_command_count: usize,
    pub(super) segment_backed_command_count: usize,
    pub(super) mixed_offset_field_namespaces: bool,
    pub(super) mixed_command_provenance_cohorts: bool,
    pub(super) all_references_use_command_relative_offset_field: bool,
    pub(super) all_references_use_source_segment_relative_offset_field: bool,
    pub(super) render_promotion_blocked_reason: &'static str,
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmRowFanoutSegmentOwnerGate {
    pub(super) command_count: usize,
    pub(super) reference_count: usize,
    pub(super) unique_row_index_count: usize,
    pub(super) command_relative_offset_field_reference_count: usize,
    pub(super) source_segment_relative_offset_field_reference_count: usize,
    pub(super) fanout_row_count: usize,
    pub(super) fanout_reference_count: usize,
    pub(super) fanout_command_relative_offset_field_reference_count: usize,
    pub(super) fanout_source_segment_relative_offset_field_reference_count: usize,
    pub(super) max_row_fanout: usize,
    pub(super) multi_command_row_indexes: Vec<usize>,
    pub(super) rows_with_multiple_command_refs: Vec<SuccessDataTestFdmRowFanoutSegmentOwnerRow>,
    pub(super) one_to_one_row_command_reference_candidate: bool,
    pub(super) single_row_backs_multiple_commands_candidate: bool,
    pub(super) mixed_offset_field_namespaces: bool,
    pub(super) mixed_command_provenance_cohorts: bool,
    pub(super) fanout_rows_use_command_relative_offset_fields: bool,
    pub(super) fanout_rows_use_source_segment_offset_fields: bool,
    pub(super) raw_span_command_count: usize,
    pub(super) segment_backed_command_count: usize,
    pub(super) render_promotion_blocked_reason: &'static str,
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmRowFanoutSegmentOwnerRow {
    pub(super) row_index: usize,
    pub(super) command_reference_count: usize,
    pub(super) command_relative_offsets: Vec<usize>,
    pub(super) match_kinds: Vec<&'static str>,
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmPrimitiveOwnershipGate {
    pub(super) row_command_gap_p95: Option<f32>,
    pub(super) row_direction_mismatch: bool,
    pub(super) multi_command_single_row: bool,
    pub(super) all_commands_referenced_by_index_rows_candidate: bool,
    pub(super) one_to_one_row_command_reference_candidate: bool,
    pub(super) mixed_raw_and_segment_cohorts: bool,
    pub(super) raw_span_command_count: usize,
    pub(super) segment_backed_command_count: usize,
    pub(super) ownership_proven: bool,
    pub(super) render_ownership_blocked_reason: &'static str,
    pub(super) render_ownership_blocked_reasons: Vec<&'static str>,
}

#[derive(Debug, Default)]
pub(super) struct SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
    pub(super) role_candidate: &'static str,
    pub(super) reference_count: usize,
    pub(super) valid_vector_offset_reference_count: usize,
    pub(super) valid_command_relative_offset_field_reference_count: usize,
    pub(super) valid_source_segment_relative_offset_field_reference_count: usize,
    pub(super) command_relative_offset_field_reference_count: usize,
    pub(super) source_segment_relative_offset_field_reference_count: usize,
    pub(super) command_relative_offsets: BTreeSet<usize>,
    pub(super) row_indexes: BTreeSet<usize>,
    pub(super) row_command_pairs: BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmRolePaintOrderContinuityProfile {
    pub(super) span_min: Option<usize>,
    pub(super) span_max: Option<usize>,
    pub(super) role_command_count: usize,
    pub(super) command_count_in_span: usize,
    pub(super) interleaved_non_role_command_count: usize,
    pub(super) max_command_offset_gap: usize,
    pub(super) continuity_score: f32,
}

impl SuccessDataTestFdmRolePaintOrderContinuityProfile {
    pub(super) fn span_contiguous_candidate(&self) -> bool {
        self.role_command_count > 0
            && self.command_count_in_span == self.role_command_count
            && self.interleaved_non_role_command_count == 0
    }

    pub(super) fn continuity_blocked(&self) -> bool {
        !self.span_contiguous_candidate()
    }

    pub(super) fn paint_order_authority_pending(&self) -> bool {
        self.span_contiguous_candidate()
    }

    pub(super) fn render_promotion_blocked_reason(&self) -> &'static str {
        if self.continuity_blocked() {
            "role-span-interleaved-non-role-commands"
        } else {
            "role-paint-order-authority-unproven"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct SuccessDataTestFdmIndexRowCommandPair {
    pub(super) row_index: usize,
    pub(super) command_relative_offset: usize,
    pub(super) match_kind: &'static str,
}

#[derive(Debug)]
pub(super) struct SuccessDataTestFdmSubdiagram<'a> {
    pub(super) index: usize,
    pub(super) anchor_relative_offset: usize,
    pub(super) center: ObjectFdmVectorPoint,
    pub(super) anchor_radius: i32,
    pub(super) commands: Vec<&'a ObjectFdmVectorCommandCandidate>,
}

pub(super) fn fdm_point_distance(a: ObjectFdmVectorPoint, b: ObjectFdmVectorPoint) -> f32 {
    let dx = (a.x() - b.x()) as f32;
    let dy = (a.y() - b.y()) as f32;
    (dx * dx + dy * dy).sqrt()
}

pub(super) fn fdm_point_distance_squared(a: ObjectFdmVectorPoint, b: ObjectFdmVectorPoint) -> i64 {
    let dx = i64::from(a.x() - b.x());
    let dy = i64::from(a.y() - b.y());
    dx * dx + dy * dy
}

pub(super) fn fdm_point_segment_distance(
    point: ObjectFdmVectorPoint,
    start: ObjectFdmVectorPoint,
    end: ObjectFdmVectorPoint,
) -> f32 {
    let px = point.x() as f32;
    let py = point.y() as f32;
    let sx = start.x() as f32;
    let sy = start.y() as f32;
    let ex = end.x() as f32;
    let ey = end.y() as f32;
    let dx = ex - sx;
    let dy = ey - sy;
    let length_squared = dx * dx + dy * dy;
    if length_squared <= f32::EPSILON {
        return ((px - sx) * (px - sx) + (py - sy) * (py - sy)).sqrt();
    }
    let t = (((px - sx) * dx + (py - sy) * dy) / length_squared).clamp(0.0, 1.0);
    let x = sx + t * dx;
    let y = sy + t * dy;
    ((px - x) * (px - x) + (py - y) * (py - y)).sqrt()
}

pub(super) fn push_fdm_frame_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let diagnostics = fdm_frame_diagnostics(document);
    if diagnostics.is_empty() {
        return;
    }

    svg.push_str("<g class=\"rjtd-fdm-frame-diagnostics\" data-source=\"fdmIndex+frame\" data-projection=\"fdmFrameDiagnosticProjection\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"false\">");
    for diagnostic in diagnostics {
        let Some((x, y, width, height)) = fdm_frame_diagnostic_bbox(layout, diagnostic) else {
            continue;
        };
        svg.push_str(&format!(
            "<g class=\"rjtd-fdm-frame-diagnostic\" data-source-path=\"{}\" data-object-candidate-index=\"{}\" data-row-index=\"{}\" data-frame-object-id=\"{}\" data-frame-type=\"0x{:04x}\" data-image-payload-extraction-status=\"{}\" data-render-promotion-blocked-reason=\"{}\" data-projection-kind=\"fdmFrameDiagnosticProjection\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"false\">",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.candidate_index,
            diagnostic.entry.row_index(),
            diagnostic.frame_record.object_id(),
            diagnostic.frame_record.object_type(),
            escape_xml(fdm_entry_image_payload_extraction_status(
                diagnostic.candidate,
                diagnostic.entry,
            )),
            escape_xml(fdm_entry_frame_render_blocked_reason(
                diagnostic.candidate,
                diagnostic.entry,
            ))
        ));
        svg.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"#eaf5ff\" fill-opacity=\"0.18\" stroke=\"#0a66b7\" stroke-width=\"1.2\" stroke-dasharray=\"5 3\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Hiragino Sans, Hiragino Kaku Gothic ProN, Yu Gothic, Meiryo, sans-serif\" font-size=\"9.0\" fill=\"#0a66b7\" letter-spacing=\"0\">FDM row {}</text>",
            x + 3.0,
            (y - 4.0).max(10.0),
            diagnostic.entry.row_index()
        ));
        svg.push_str("</g>");
    }
    svg.push_str("</g>");
}

pub(super) fn push_fdm_command_diagnostic_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    if page_number != 1 {
        return;
    }

    let diagnostics = fdm_command_diagnostics(document);
    let Some(extent) = fdm_command_projection_extent(&diagnostics) else {
        return;
    };

    svg.push_str("<g class=\"rjtd-fdm-command-diagnostics\" data-source=\"fdmVectorCommand\" data-projection=\"fdmCommandBBoxReferenceProjection\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"false\">");
    for diagnostic in diagnostics {
        let Some((x, y, width, height)) = fdm_command_diagnostic_bbox(layout, diagnostic, extent)
        else {
            continue;
        };
        let style = fdm_command_diagnostic_svg_style(diagnostic);
        svg.push_str(&format!(
            "<rect class=\"rjtd-fdm-command-diagnostic\" data-source-path=\"{}\" data-row-index=\"{}\" data-command-index=\"{}\" data-marker-hex=\"{}\" data-diagnostic-style-basis=\"{}\" data-image-signature-count=\"{}\" data-segment-image-signature-count=\"{}\" data-valid-vector-offset=\"{}\" data-projection-kind=\"fdmCommandBBoxReferenceProjection\" data-decoded=\"false\" data-geometry-decoded=\"false\" data-placement-proven=\"false\" data-renderable=\"false\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.65\" stroke-opacity=\"{}\"/>",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.entry.row_index(),
            diagnostic.command.command_index(),
            hex_bytes(diagnostic.command.marker()),
            style.basis,
            diagnostic.entry.image_signature_hits().len(),
            diagnostic.entry.segment_image_signature_hits().len(),
            diagnostic.entry.valid_vector_offset(),
            style.stroke,
            style.opacity
        ));
    }
    svg.push_str("</g>");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FdmCommandDiagnosticSvgStyle {
    pub(super) stroke: &'static str,
    pub(super) opacity: &'static str,
    pub(super) basis: &'static str,
}

pub(super) fn fdm_command_diagnostic_svg_style(
    diagnostic: FdmCommandDiagnostic<'_>,
) -> FdmCommandDiagnosticSvgStyle {
    if !diagnostic.entry.segment_image_signature_hits().is_empty() {
        FdmCommandDiagnosticSvgStyle {
            stroke: "#d9432f",
            opacity: "0.82",
            basis: "fdm-index-segment-image-signature",
        }
    } else if !diagnostic.entry.image_signature_hits().is_empty() {
        FdmCommandDiagnosticSvgStyle {
            stroke: "#d9432f",
            opacity: "0.82",
            basis: "fdm-index-image-signature",
        }
    } else {
        FdmCommandDiagnosticSvgStyle {
            stroke: "#4d95ff",
            opacity: "0.44",
            basis: "fdm-index-command-diagnostic-default",
        }
    }
}

pub(super) fn push_fdm_vector_primitive_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) -> bool {
    if page_number != 1 {
        return false;
    }

    let command_diagnostics = fdm_command_diagnostics(document);
    let Some(extent) = fdm_command_projection_extent(&command_diagnostics) else {
        return false;
    };
    let diagnostics = fdm_vector_primitive_diagnostics(document);
    if diagnostics.is_empty() {
        return false;
    }

    let group_start = svg.len();
    let mut rendered = false;
    let mut counter_overlays = String::new();
    svg.push_str("<g class=\"rjtd-fdm-vector-primitives\" data-source=\"fdmVectorCommandPrimitive\" data-projection=\"fdmVectorPrimitiveReferenceProjection\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\">");
    for diagnostic in diagnostics.iter().copied() {
        let Some((path_x, path_y, path_width, path_height)) =
            fdm_path_diagnostic_bbox(layout, diagnostic, extent)
        else {
            continue;
        };

        let path_closed = fdm_vector_primitive_is_closed(diagnostic.command);
        let fill = fdm_vector_render_fill_color(diagnostic, &diagnostics);
        let gradient = fdm_vector_linear_gradient_colors(diagnostic.command);
        let stroke = fdm_vector_render_stroke_color(diagnostic, &diagnostics);
        let data_fill = diagnostic
            .command
            .fill_color()
            .and_then(fdm_vector_css_color)
            .unwrap_or_else(|| "none".to_string());
        let data_stroke = diagnostic
            .command
            .stroke_color()
            .and_then(fdm_vector_css_color)
            .unwrap_or_else(|| "none".to_string());
        let stroke_width = fdm_vector_stroke_width(diagnostic.command);
        let primitive_kind = fdm_vector_primitive_kind(diagnostic.command);
        let paint_coverage = fdm_vector_paint_coverage(
            layout,
            diagnostic,
            &diagnostics,
            (path_x, path_y, path_width, path_height),
        );
        let page_coverage_ratio = paint_coverage.page_coverage_ratio;
        let viewport_coverage_ratio = paint_coverage.viewport_coverage_ratio;

        if let Some(ellipse) = diagnostic.command.ellipse() {
            let Some((cx, cy, rx, ry)) = fdm_projected_ellipse(layout, extent, ellipse) else {
                continue;
            };
            let ellipse_color = ellipse
                .color()
                .and_then(fdm_vector_primitive_css_color)
                .unwrap_or_else(|| "#111111".to_string());
            let fill = if fdm_vector_ellipse_should_fill(ellipse) {
                ellipse_color.as_str()
            } else {
                "none"
            };
            let stroke = if fdm_vector_ellipse_should_fill(ellipse) {
                "none"
            } else {
                ellipse_color.as_str()
            };
            svg.push_str(&format!(
                "<ellipse class=\"rjtd-fdm-vector-primitive\" data-source-path=\"{}\" data-row-index=\"{}\" data-command-index=\"{}\" data-marker-hex=\"{}\" data-primitive-kind=\"{}\" data-style-word=\"0x{:04x}\" data-fill-color=\"{}\" data-stroke-color=\"{}\" data-stroke-width=\"{:.3}\" data-path-closed=\"{}\" data-point-count=\"{}\" data-page-coverage-ratio=\"{page_coverage_ratio:.6}\" data-viewport-coverage-ratio=\"{viewport_coverage_ratio:.6}\" data-page-fill-candidate=\"{}\" data-page-fill-candidate-basis=\"{}\" data-page-fill-candidate-reason=\"{}\" data-page-fill-render-promotion-blocked-reason=\"{}\" data-projection-kind=\"fdmVectorPrimitiveReferenceProjection\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" rx=\"{rx:.1}\" ry=\"{ry:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.3}\" stroke-opacity=\"0.92\"/>",
                escape_xml(diagnostic.candidate.path()),
                diagnostic.entry.row_index(),
                diagnostic.command.command_index(),
                hex_bytes(diagnostic.command.marker()),
                primitive_kind,
                diagnostic.command.style_word(),
                data_fill,
                data_stroke,
                stroke_width,
                path_closed,
                diagnostic.command.path_points().len(),
                paint_coverage.page_fill_candidate,
                paint_coverage.page_fill_candidate_basis,
                paint_coverage.page_fill_candidate_reason,
                paint_coverage.render_promotion_blocked_reason,
                fill,
                stroke,
                stroke_width
            ));
            rendered = true;
            continue;
        }

        let Some(path_data) = fdm_projected_path_data(layout, extent, diagnostic.command) else {
            continue;
        };
        let fill_paint = if let Some((gradient_from, gradient_to)) = gradient.as_ref() {
            let gradient_id = format!(
                "rjtd-fdm-gradient-{}-{}",
                diagnostic.entry.row_index(),
                diagnostic.command.command_index()
            );
            svg.push_str(&format!(
                "<defs><linearGradient id=\"{gradient_id}\" gradientUnits=\"userSpaceOnUse\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"><stop offset=\"0%\" stop-color=\"{gradient_from}\"/><stop offset=\"100%\" stop-color=\"{gradient_to}\"/></linearGradient></defs>",
                path_x,
                path_y + path_height,
                path_x + path_width,
                path_y
            ));
            format!("url(#{gradient_id})")
        } else {
            fill.clone()
        };
        svg.push_str(&format!(
            "<path class=\"rjtd-fdm-vector-primitive\" data-source-path=\"{}\" data-row-index=\"{}\" data-command-index=\"{}\" data-marker-hex=\"{}\" data-primitive-kind=\"{}\" data-style-word=\"0x{:04x}\" data-fill-color=\"{}\" data-render-fill-kind=\"{}\" data-render-fill-color=\"{}\" data-stroke-color=\"{}\" data-render-stroke-color=\"{}\" data-stroke-width=\"{:.3}\" data-path-closed=\"{}\" data-point-count=\"{}\" data-page-coverage-ratio=\"{page_coverage_ratio:.6}\" data-viewport-coverage-ratio=\"{viewport_coverage_ratio:.6}\" data-page-fill-candidate=\"{}\" data-page-fill-candidate-basis=\"{}\" data-page-fill-candidate-reason=\"{}\" data-page-fill-render-promotion-blocked-reason=\"{}\" data-projection-kind=\"fdmVectorPrimitiveReferenceProjection\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\" d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.3}\" stroke-opacity=\"0.92\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            escape_xml(diagnostic.candidate.path()),
            diagnostic.entry.row_index(),
            diagnostic.command.command_index(),
            hex_bytes(diagnostic.command.marker()),
            primitive_kind,
            diagnostic.command.style_word(),
            data_fill,
            if gradient.is_some() {
                "linearGradient"
            } else if fill == "none" {
                "none"
            } else {
                "solid"
            },
            fill,
            data_stroke,
            stroke,
            stroke_width,
            path_closed,
            diagnostic.command.path_points().len(),
            paint_coverage.page_fill_candidate,
            paint_coverage.page_fill_candidate_basis,
            paint_coverage.page_fill_candidate_reason,
            paint_coverage.render_promotion_blocked_reason,
            path_data,
            fill_paint,
            stroke,
            stroke_width
        ));
        if fdm_vector_filled_path_is_counter_overlay(diagnostic, &diagnostics) {
            counter_overlays.push_str(&format!(
                "<path class=\"rjtd-fdm-vector-counter-overlay\" data-source-path=\"{}\" data-row-index=\"{}\" data-command-index=\"{}\" data-marker-hex=\"{}\" data-render-counter-overlay=\"true\" data-render-fill-color=\"{}\" data-projection-kind=\"fdmVectorPrimitiveReferenceProjection\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\" data-renderable=\"true\" d=\"{}\" fill=\"{}\" stroke=\"none\"/>",
                escape_xml(diagnostic.candidate.path()),
                diagnostic.entry.row_index(),
                diagnostic.command.command_index(),
                hex_bytes(diagnostic.command.marker()),
                fill,
                path_data,
                fill
            ));
        }
        rendered = true;
    }
    svg.push_str(&counter_overlays);
    svg.push_str("</g>");
    if !rendered {
        svg.truncate(group_start);
    }
    rendered
}

pub(super) fn fdm_frame_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: FdmFrameDiagnostic<'_>,
) -> Option<(f32, f32, f32, f32)> {
    let scale_x = layout.width_px() / SHANAI_LAN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let x = diagnostic.frame_record.x() as f32 / SHANAI_LAN_FDM_FRAME_X_DIVISOR * scale_x;
    let y = diagnostic.frame_record.y() as f32 / SHANAI_LAN_FDM_FRAME_Y_DIVISOR * scale_y;
    let width =
        diagnostic.frame_record.width() as f32 / SHANAI_LAN_FDM_FRAME_SIZE_DIVISOR * scale_x;
    let height =
        diagnostic.frame_record.height() as f32 / SHANAI_LAN_FDM_FRAME_SIZE_DIVISOR * scale_y;

    if x >= layout.width_px() || y >= layout.height_px() || width <= 0.0 || height <= 0.0 {
        return None;
    }
    Some((
        x,
        y,
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

pub(super) fn fdm_command_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<(f32, f32, f32, f32)> {
    let bbox = normalize_fdm_bbox(diagnostic.command.bbox()?);
    let span_x = (extent.right - extent.left) as f32;
    let span_y = (extent.bottom - extent.top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let viewport = fdm_projection_viewport(layout);
    let x = viewport.x + (bbox.0 - extent.left) as f32 / span_x * viewport.width;
    let y = viewport.y + (bbox.1 - extent.top) as f32 / span_y * viewport.height;
    let width = (bbox.2 - bbox.0).max(1) as f32 / span_x * viewport.width;
    let height = (bbox.3 - bbox.1).max(1) as f32 / span_y * viewport.height;
    if x >= layout.width_px() || y >= layout.height_px() || width <= 0.0 || height <= 0.0 {
        return None;
    }
    Some((
        x,
        y,
        width.min((layout.width_px() - x).max(1.0)),
        height.min((layout.height_px() - y).max(1.0)),
    ))
}

pub(super) fn fdm_path_diagnostic_bbox(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<(f32, f32, f32, f32)> {
    let bbox = fdm_path_unfiltered_bbox(layout, diagnostic, extent)?;
    if fdm_path_span_filter_blocks(layout, diagnostic.command, bbox) {
        return None;
    }
    Some(bbox)
}

pub(super) fn fdm_path_span_filter_blocked(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<(f32, f32, f32, f32)> {
    let bbox = fdm_path_unfiltered_bbox(layout, diagnostic, extent)?;
    if fdm_path_span_filter_blocks(layout, diagnostic.command, bbox) {
        Some(bbox)
    } else {
        None
    }
}

pub(super) fn fdm_path_span_filter_blocks(
    layout: PageLayout,
    command: &ObjectFdmVectorCommandCandidate,
    bbox: (f32, f32, f32, f32),
) -> bool {
    fdm_vector_path_span_filter_applies(command)
        && (bbox.2 / layout.width_px() > FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO
            || bbox.3 / layout.height_px() > FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO)
}

pub(super) fn fdm_path_unfiltered_bbox(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<(f32, f32, f32, f32)> {
    let source_bbox = fdm_vector_command_source_bbox(diagnostic.command)?;
    let bbox = normalize_fdm_bbox(source_bbox);
    let (x1, y1) = fdm_project_source_point(layout, extent, bbox.0, bbox.1)?;
    let (x2, y2) = fdm_project_source_point(layout, extent, bbox.2, bbox.3)?;
    let width = (x2 - x1).abs().max(0.5);
    let height = (y2 - y1).abs().max(0.5);
    Some((x1.min(x2), y1.min(y2), width, height))
}

pub(super) fn fdm_connector_candidate_metric(
    layout: PageLayout,
    diagnostic: FdmCommandDiagnostic<'_>,
    extent: FdmCommandProjectionExtent,
) -> Option<FdmConnectorCandidateMetric> {
    if diagnostic.command.ellipse().is_some()
        || fdm_vector_path_is_closed(diagnostic.command.path_points())
    {
        return None;
    }
    let source_start = *diagnostic.command.path_points().first()?;
    let source_end = *diagnostic.command.path_points().last()?;
    let projected_start =
        fdm_project_source_point(layout, extent, source_start.x(), source_start.y())?;
    let projected_end = fdm_project_source_point(layout, extent, source_end.x(), source_end.y())?;
    let projected_bbox = fdm_path_diagnostic_bbox(layout, diagnostic, extent)?;
    let source_dx = source_end.x().saturating_sub(source_start.x()) as f32;
    let source_dy = source_end.y().saturating_sub(source_start.y()) as f32;
    let projected_dx = projected_end.0 - projected_start.0;
    let projected_dy = projected_end.1 - projected_start.1;
    let source_endpoint_distance = source_dx.hypot(source_dy);
    let projected_endpoint_distance = projected_dx.hypot(projected_dy);
    let projected_span =
        projected_endpoint_distance.max(projected_bbox.2.abs().max(projected_bbox.3.abs()));
    if projected_span < FDM_CONNECTOR_CANDIDATE_MIN_PROJECTED_SPAN_PX {
        return None;
    }

    Some(FdmConnectorCandidateMetric {
        source_start,
        source_end,
        projected_start,
        projected_end,
        projected_bbox,
        source_endpoint_distance,
        projected_endpoint_distance,
        projected_span,
        orientation: fdm_connector_orientation(projected_dx, projected_dy),
        basis: if projected_endpoint_distance >= FDM_CONNECTOR_CANDIDATE_MIN_PROJECTED_SPAN_PX {
            "long-open-endpoint-path"
        } else {
            "long-open-bbox-path"
        },
    })
}

pub(super) fn fdm_connector_orientation(dx: f32, dy: f32) -> &'static str {
    let abs_x = dx.abs();
    let abs_y = dy.abs();
    if abs_x >= abs_y * 2.0 {
        "horizontal"
    } else if abs_y >= abs_x * 2.0 {
        "vertical"
    } else {
        "diagonal"
    }
}

pub(super) fn fdm_vector_path_span_filter_applies(
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    !fdm_vector_marker_is_line(command.marker()) || fdm_vector_path_is_closed(command.path_points())
}

pub(super) fn fdm_projected_path_data(
    layout: PageLayout,
    extent: FdmCommandProjectionExtent,
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<String> {
    let mut points = Vec::with_capacity(command.path_points().len());
    for point in command.path_points() {
        points.push(fdm_project_source_point(
            layout,
            extent,
            point.x(),
            point.y(),
        )?);
    }
    if points.len() < 2 {
        return None;
    }

    let mut path_data = format!("M {:.1} {:.1}", points[0].0, points[0].1);
    if command.curve_segments().len() == points.len().saturating_sub(1) {
        for (index, segment) in command.curve_segments().iter().enumerate() {
            let control_1 = segment.control_1();
            let control_2 = segment.control_2();
            let end = command.path_points()[index + 1];
            let (control_1_x, control_1_y) =
                fdm_project_source_point(layout, extent, control_1.x(), control_1.y())?;
            let (control_2_x, control_2_y) =
                fdm_project_source_point(layout, extent, control_2.x(), control_2.y())?;
            let (end_x, end_y) = fdm_project_source_point(layout, extent, end.x(), end.y())?;
            path_data.push_str(&format!(
                " C {control_1_x:.1} {control_1_y:.1} {control_2_x:.1} {control_2_y:.1} {end_x:.1} {end_y:.1}"
            ));
        }
    } else if fdm_vector_marker_is_bezier_curve(command.marker()) && points.len() >= 3 {
        let mut index = 1usize;
        while index + 1 < points.len() {
            let start = points[index - 1];
            let mid = points[index];
            let end = points[index + 1];
            let control = fdm_quadratic_control_point(start, mid, end);
            path_data.push_str(&format!(
                " Q {:.1} {:.1} {:.1} {:.1}",
                control.0, control.1, end.0, end.1
            ));
            index += 2;
        }
        while index < points.len() {
            let point = points[index];
            path_data.push_str(&format!(" L {:.1} {:.1}", point.0, point.1));
            index += 1;
        }
    } else {
        for point in points.iter().skip(1) {
            path_data.push_str(&format!(" L {:.1} {:.1}", point.0, point.1));
        }
    }

    if fdm_vector_path_is_closed(command.path_points()) {
        path_data.push_str(" Z");
    }
    Some(path_data)
}

pub(super) fn fdm_quadratic_control_point(
    start: (f32, f32),
    mid: (f32, f32),
    end: (f32, f32),
) -> (f32, f32) {
    (
        2.0 * mid.0 - (start.0 + end.0) * 0.5,
        2.0 * mid.1 - (start.1 + end.1) * 0.5,
    )
}

pub(super) fn fdm_projected_ellipse(
    layout: PageLayout,
    extent: FdmCommandProjectionExtent,
    ellipse: ObjectFdmVectorEllipse,
) -> Option<(f32, f32, f32, f32)> {
    let center = ellipse.center();
    let (cx, cy) = fdm_project_source_point(layout, extent, center.x(), center.y())?;
    let span_x = (extent.right - extent.left) as f32;
    let span_y = (extent.bottom - extent.top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let viewport = fdm_projection_viewport(layout);
    let rx = ellipse.radius_x() as f32 / span_x * viewport.width;
    let ry = ellipse.radius_y() as f32 / span_y * viewport.height;
    if rx <= 0.0 || ry <= 0.0 {
        return None;
    }
    Some((cx, cy, rx, ry))
}

pub(super) fn fdm_vector_ellipse_should_fill(ellipse: ObjectFdmVectorEllipse) -> bool {
    ellipse.radius_x().max(ellipse.radius_y()) <= 80
}

pub(super) fn fdm_project_source_point(
    layout: PageLayout,
    extent: FdmCommandProjectionExtent,
    x: i32,
    y: i32,
) -> Option<(f32, f32)> {
    let span_x = (extent.right - extent.left) as f32;
    let span_y = (extent.bottom - extent.top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let viewport = fdm_projection_viewport(layout);
    Some((
        viewport.x + (x - extent.left) as f32 / span_x * viewport.width,
        viewport.y + (y - extent.top) as f32 / span_y * viewport.height,
    ))
}

pub(super) fn fdm_projection_viewport(layout: PageLayout) -> FdmProjectionViewport {
    let scale_x = layout.width_px() / SHANAI_LAN_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    FdmProjectionViewport {
        x: SHANAI_LAN_REFERENCE_CONTENT_LEFT_PX * scale_x,
        y: SHANAI_LAN_REFERENCE_CONTENT_TOP_PX * scale_y,
        width: SHANAI_LAN_REFERENCE_CONTENT_WIDTH_PX * scale_x,
        height: SHANAI_LAN_REFERENCE_CONTENT_HEIGHT_PX * scale_y,
    }
}

pub(super) struct SuccessDataTestAnswerSheetFdmTextSlot {
    pub(super) text: String,
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) font_size: f32,
    pub(super) text_offset: usize,
    pub(super) marker_offset: usize,
    pub(super) index_offset: usize,
    pub(super) source_bbox: ObjectFdmIndexBbox,
    pub(super) text_bbox: ObjectFdmIndexBbox,
}

pub(super) struct SuccessDataTestAnswerSheetIndexedFdmLabel<'a> {
    pub(super) text: &'a ObjectFdmTextCandidate,
    pub(super) index: &'a ObjectFdmTextIndexEntryCandidate,
    pub(super) text_bbox: ObjectFdmIndexBbox,
}
