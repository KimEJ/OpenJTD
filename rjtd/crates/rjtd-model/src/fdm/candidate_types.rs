use super::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFdmTextCandidate {
    pub(crate) text: String,
    pub(crate) text_offset: usize,
    pub(crate) marker_offset: usize,
    pub(crate) raw_text: Vec<u8>,
    pub(crate) bbox: Option<ObjectFdmIndexBbox>,
}

impl ObjectFdmTextCandidate {
    pub(crate) fn new(
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
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
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
    pub(crate) index_path: String,
    pub(crate) vector_path: String,
    pub(crate) row_index: usize,
    pub(crate) index_offset: usize,
    pub(crate) vector_offset: usize,
    pub(crate) next_vector_offset: usize,
    pub(crate) vector_len: usize,
    pub(crate) kind: u16,
    pub(crate) bbox: ObjectFdmIndexBbox,
    pub(crate) valid_vector_offset: bool,
    pub(crate) vector_prefix: Vec<u8>,
    pub(crate) image_signature_hits: Vec<ObjectImageSignatureHit>,
    pub(crate) segment_image_signature_hits: Vec<ObjectImageSignatureHit>,
    pub(crate) vector_commands: Vec<ObjectFdmVectorCommandCandidate>,
    pub(crate) connector_candidates: Vec<ObjectFdmConnectorCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFdmTextIndexEntryCandidate {
    pub(crate) index_path: String,
    pub(crate) text_path: String,
    pub(crate) row_index: usize,
    pub(crate) index_offset: usize,
    pub(crate) text_record_offset: usize,
    pub(crate) kind: u16,
    pub(crate) bbox: ObjectFdmIndexBbox,
    pub(crate) text_record_bbox: Option<ObjectFdmIndexBbox>,
    pub(crate) valid_text_record_offset: bool,
    pub(crate) text_record_prefix: Vec<u8>,
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
pub(crate) struct FdmIndexSegmentBboxAxisPairGate {
    pub(crate) valid_index_row_count: usize,
    pub(crate) linked_row_count: usize,
    pub(crate) axis_pair_order_agreement_row_count: usize,
}

impl FdmIndexSegmentBboxAxisPairGate {
    pub(crate) fn new(
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

    pub(crate) fn valid_index_row_count(self) -> usize {
        self.valid_index_row_count
    }

    pub(crate) fn linked_row_count(self) -> usize {
        self.linked_row_count
    }

    pub(crate) fn axis_pair_order_agreement_row_count(self) -> usize {
        self.axis_pair_order_agreement_row_count
    }

    pub(crate) fn axis_pair_order_agreement_complete(self) -> bool {
        self.valid_index_row_count > 0
            && self.valid_index_row_count == self.linked_row_count
            && self.linked_row_count == self.axis_pair_order_agreement_row_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FdmTextMirrorAnchorAgreement {
    pub(crate) indexed_text_path: String,
    pub(crate) mirrored_text_path: String,
    pub(crate) text_record_count: usize,
    pub(crate) ordered_text_agreement: bool,
    pub(crate) ordered_record_bbox_agreement: bool,
    pub(crate) indexed_record_offset_agreement: bool,
    pub(crate) indexed_record_bbox_agreement: bool,
}

impl FdmTextMirrorAnchorAgreement {
    pub(crate) fn new(
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

    pub(crate) fn indexed_text_path(&self) -> &str {
        &self.indexed_text_path
    }

    pub(crate) fn mirrored_text_path(&self) -> &str {
        &self.mirrored_text_path
    }

    pub(crate) fn text_record_count(&self) -> usize {
        self.text_record_count
    }

    pub(crate) fn ordered_text_agreement(&self) -> bool {
        self.ordered_text_agreement
    }

    pub(crate) fn ordered_record_bbox_agreement(&self) -> bool {
        self.ordered_record_bbox_agreement
    }

    pub(crate) fn indexed_record_offset_agreement(&self) -> bool {
        self.indexed_record_offset_agreement
    }

    pub(crate) fn indexed_record_bbox_agreement(&self) -> bool {
        self.indexed_record_bbox_agreement
    }

    pub(crate) fn source_anchor_trace_ready(&self) -> bool {
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
    pub(crate) command_index: usize,
    pub(crate) relative_offset: usize,
    pub(crate) marker: [u8; 4],
    pub(crate) style_word: u16,
    pub(crate) primitive_kind: &'static str,
    pub(crate) fill_color: Option<u32>,
    pub(crate) stroke_color: Option<u32>,
    pub(crate) source_start: ObjectFdmVectorPoint,
    pub(crate) source_end: ObjectFdmVectorPoint,
    pub(crate) source_bbox: ObjectFdmIndexBbox,
    pub(crate) source_span: i32,
    pub(crate) endpoint_dx: i32,
    pub(crate) endpoint_dy: i32,
    pub(crate) endpoint_distance_squared: u64,
    pub(crate) path_point_count: usize,
    pub(crate) path_segment_count: usize,
    pub(crate) orthogonal_segment_count: usize,
    pub(crate) diagonal_segment_count: usize,
    pub(crate) curve_segment_count: usize,
    pub(crate) compound_child_offset_count: usize,
    pub(crate) axis_aligned: bool,
    pub(crate) orientation: &'static str,
    pub(crate) basis: &'static str,
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
    pub(crate) relative_offset: usize,
    pub(crate) declared_len: u16,
    pub(crate) command_count: u16,
    pub(crate) command_offsets: Vec<u16>,
    pub(crate) bbox: Option<ObjectFdmIndexBbox>,
    pub(crate) source_width: i32,
    pub(crate) source_height: i32,
}

impl ObjectFdmVectorSegmentCandidate {
    pub(crate) fn new(relative_offset: usize, header: FdmVectorSegmentHeader) -> Self {
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
    pub(crate) command_index: usize,
    pub(crate) relative_offset: usize,
    pub(crate) source_vector_relative_offset: Option<usize>,
    pub(crate) source_segment: Option<ObjectFdmVectorCommandSourceSegment>,
    pub(crate) record_len: usize,
    pub(crate) declared_record_len: u16,
    pub(crate) style_word: u16,
    pub(crate) marker: [u8; 4],
    pub(crate) bbox: Option<ObjectFdmIndexBbox>,
    pub(crate) path_points: Vec<ObjectFdmVectorPoint>,
    pub(crate) curve_segments: Vec<ObjectFdmVectorCurveSegment>,
    pub(crate) ellipse: Option<ObjectFdmVectorEllipse>,
    pub(crate) compound_child_offsets: Vec<u16>,
    pub(crate) compound_child_layout: Option<FdmCompoundChildLayout>,
    pub(crate) gradient_colors: Option<FdmVectorGradientContext>,
    pub(crate) fill_color: Option<u32>,
    pub(crate) stroke_color: Option<u32>,
}

impl ObjectFdmVectorCommandCandidate {
    pub(crate) fn new(
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

    pub(crate) fn with_source_vector_relative_offset(mut self, relative_offset: usize) -> Self {
        self.source_vector_relative_offset = Some(relative_offset);
        self
    }

    pub(crate) fn with_source_segment(
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

    pub(crate) fn compound_child_layout(&self) -> Option<&FdmCompoundChildLayout> {
        self.compound_child_layout.as_ref()
    }

    pub(crate) fn gradient_colors(&self) -> Option<FdmVectorGradientContext> {
        self.gradient_colors
    }

    pub fn fill_color(&self) -> Option<u32> {
        self.fill_color
    }

    pub fn stroke_color(&self) -> Option<u32> {
        self.stroke_color
    }

    pub(crate) fn has_renderable_geometry(&self) -> bool {
        self.path_points.len() >= 2 || self.ellipse.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmVectorCommandSourceSegment {
    pub(crate) relative_offset: usize,
    pub(crate) local_offset: usize,
    pub(crate) declared_len: u16,
    pub(crate) command_count: u16,
    pub(crate) command_index: usize,
    pub(crate) command_offset: u16,
}

impl ObjectFdmVectorCommandSourceSegment {
    pub(crate) fn new(
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
pub(crate) struct FdmVectorStyleContext {
    pub(crate) fill_color: Option<u32>,
    pub(crate) stroke_color: Option<u32>,
    pub(crate) gradient_colors: Option<FdmVectorGradientContext>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FdmVectorGradientContext {
    pub(crate) from_color: u32,
    pub(crate) to_color: u32,
}

impl FdmVectorGradientContext {
    pub(crate) fn new(from_color: u32, to_color: u32) -> Self {
        Self {
            from_color,
            to_color,
        }
    }

    pub(crate) fn start_color(self) -> u32 {
        self.from_color
    }

    pub(crate) fn end_color(self) -> u32 {
        self.to_color
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectFdmVectorPoint {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl ObjectFdmVectorPoint {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub(crate) fn offset(self, dx: i32, dy: i32) -> Self {
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
    pub(crate) control_1: ObjectFdmVectorPoint,
    pub(crate) control_2: ObjectFdmVectorPoint,
}

impl ObjectFdmVectorCurveSegment {
    pub(crate) fn new(control_1: ObjectFdmVectorPoint, control_2: ObjectFdmVectorPoint) -> Self {
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
    pub(crate) center: ObjectFdmVectorPoint,
    pub(crate) radius_x: i32,
    pub(crate) radius_y: i32,
    pub(crate) color: Option<u32>,
}

impl ObjectFdmVectorEllipse {
    pub(crate) fn new(
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
