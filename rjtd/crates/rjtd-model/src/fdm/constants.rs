pub(crate) const FDM_INDEX_HEADER_BYTES: usize = 20;

pub(crate) const FDM_INDEX_ENTRY_BYTES: usize = 22;

pub(crate) const FDM_INDEX_DECLARED_COUNT_OFFSET: usize = 18;

pub(crate) const FDM_VECTOR_SEGMENT_MAGIC: &[u8; 4] = b"\x01\x00\x0b\x60";

pub(crate) const FDM_VECTOR_SEGMENT_HEADER_BYTES: usize = 52;

pub(crate) const FDM_VECTOR_COMMAND_OFFSET_BYTES: usize = 2;

pub(crate) const FDM_VECTOR_COMMAND_DECLARED_LEN_OFFSET: usize = 4;

pub(crate) const FDM_VECTOR_COMMAND_BBOX_OFFSET: usize = 20;

pub(crate) const FDM_VECTOR_COMMAND_BBOX_MARKER: &[u8; 4] = b"\xff\x00\x0a\x60";

pub(crate) const FDM_VECTOR_COMMAND_LINE_MARKER: &[u8; 4] = b"\xff\x00\x01\x60";

pub(crate) const FDM_VECTOR_COMMAND_NESTED_LINE_MARKER: &[u8; 4] = b"\x00\x00\x01\x60";

pub(crate) const FDM_VECTOR_COMMAND_INDEXED_LINE_MARKER: &[u8; 4] = b"\x01\x00\x01\x60";

pub(crate) const FDM_VECTOR_COMMAND_LINE_POINTS_OFFSET: usize = 16;

pub(crate) const FDM_VECTOR_COMMAND_ELLIPSE_COLOR_OFFSET: usize = 12;

pub(crate) const FDM_VECTOR_COMMAND_ELLIPSE_CENTER_OFFSET: usize = 16;

pub(crate) const FDM_VECTOR_COMMAND_ELLIPSE_RADIUS_OFFSET: usize = 24;

pub(crate) const FDM_VECTOR_COMMAND_PATH_POINT_COUNT_OFFSET: usize = 16;

pub(crate) const FDM_VECTOR_COMMAND_PATH_POINTS_OFFSET: usize = 18;

pub(crate) const FDM_TEXT_RECORD_MARKER: &[u8; 4] = b"\x01\x00\x14\x60";

pub(crate) const FDM_TEXT_EXPANDED_RECORD_MARKER: &[u8; 4] = b"\x01\x00\x16\x60";

pub(crate) const FDM_TEXT_RECORD_DECLARED_LENGTH_OFFSET: usize = 4;

pub(crate) const FDM_TEXT_RECORD_TRAILER: &[u8; 4] = b"\x00\x0d\x00\x0d";

pub(crate) const FDM_TEXT_RECORD_TEXT_DELIMITER: &[u8; 2] = b"\x00\x0d";

pub(crate) const FDM_TEXT_RECORD_BBOX_OFFSET_FROM_MARKER: usize = 8;

pub(crate) const FDM_TEXT_RECORD_BACKSCAN_BYTES: usize = 96;

pub(crate) const FDM_TEXT_EXPANDED_COUNT_OFFSET_FROM_MARKER: usize = 0x22;

pub(crate) const FDM_TEXT_EXPANDED_INDEX_KIND: u16 = 0x1600;

pub(crate) const FDM_VECTOR_COMMAND_ELLIPSE_MARKERS: [[u8; 4]; 3] = [
    *b"\xff\x00\x04\x60",
    *b"\x00\x00\x04\x60",
    *b"\x01\x00\x04\x60",
];

pub(crate) const FDM_VECTOR_COMMAND_PATH_MARKERS: [[u8; 4]; 6] = [
    *b"\xff\x00\x06\x60",
    *b"\xff\x00\x09\x60",
    *b"\x00\x00\x06\x60",
    *b"\x00\x00\x09\x60",
    *b"\x01\x00\x06\x60",
    *b"\x01\x00\x09\x60",
];

pub(crate) const FDM_VECTOR_NESTED_PRIMITIVE_MARKERS: [[u8; 4]; 12] = [
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

pub(crate) const FDM_VECTOR_TEXT_MASK_MIN_INNER_AREA_RATIO: f64 = 0.30;

pub(crate) const FDM_VECTOR_TEXT_MASK_MAX_INNER_AREA_RATIO: f64 = 0.85;

pub(crate) const FDM_VECTOR_RENDERED_PRIMITIVE_MARKERS: [[u8; 4]; 12] = [
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

pub(crate) const FDM_VECTOR_PATH_DIAGNOSTIC_MAX_SPAN_RATIO: f32 = 0.28;

pub(crate) const FDM_TEXT_MASK_COHORT_MIN_PRIMITIVES: usize = 3;

pub(crate) const FDM_TEXT_MASK_COHORT_LIMIT: usize = 24;

pub(crate) const FDM_TEXT_MASK_RIGHT_NEIGHBOR_MAX_GAP_FACTOR: f32 = 3.0;

pub(crate) const FDM_TEXT_MASK_COMPONENT_MIN_PRIMITIVES: usize = 3;

pub(crate) const FDM_TEXT_MASK_COMPONENT_MAX_HEIGHT_LINE_FACTOR: f32 = 0.85;

pub(crate) const FDM_CONNECTOR_CANDIDATE_MIN_SOURCE_SPAN_UNITS: i32 = 500;

pub(crate) const FDM_CONNECTOR_CANDIDATE_MIN_PROJECTED_SPAN_PX: f32 = 48.0;

pub(crate) const FDM_OPEN_STROKE_AXIS_RULE_MIN_PROJECTED_SPAN_PX: f32 = 5.0;

pub(crate) const FDM_OPEN_STROKE_AXIS_RULE_ROW_COHORT_LIMIT: usize = 16;

pub(crate) const FDM_OPEN_STROKE_ROW_COHORT_LIMIT: usize = 16;

pub(crate) const FIGURE_LINK_HEADER_BYTES: usize = 8;

pub(crate) const FIGURE_LINK_ROW_BYTES: usize = 14;

pub(crate) const FIGURE_LINK_RELATION_KIND_CANDIDATE_OFFSET: usize = 8;

pub(crate) const FIGURE_LINK_RELATION_KIND_CANDIDATE: u16 = 0x0016;

pub(crate) const SHANAI_LAN_FDM_FRAME_X_DIVISOR: f32 = 24.0;

pub(crate) const SHANAI_LAN_FDM_FRAME_Y_DIVISOR: f32 = 1.0;

pub(crate) const SHANAI_LAN_FDM_FRAME_SIZE_DIVISOR: f32 = 24.0;

pub(crate) const FDM_CONNECTOR_LINE_RULE_SPAN_OVERFLOW_PROBE_UNITS: f32 = 2.0;

pub(crate) const FDM_CONNECTOR_LINE_RULE_TIGHT_PERPENDICULAR_PROBE_UNITS: f32 = 1.0;

pub(crate) const FDM_CONNECTOR_LINE_RULE_NEARBY_PERPENDICULAR_PROBE_UNITS: f32 = 2.0;

pub(crate) const FDM_CONNECTOR_ENDPOINT_OWNER_PROBE_RADIUS_PX: f32 = 18.0;

pub(crate) const FDM_CONNECTOR_ENDPOINT_OWNER_CANDIDATE_LIMIT: usize = 3;
