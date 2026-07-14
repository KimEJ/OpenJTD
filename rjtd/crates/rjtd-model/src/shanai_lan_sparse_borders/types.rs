use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderTopologyDiagnostic {
    pub(crate) stable_grid_extent_units: u16,
    pub(crate) rows: Vec<ShanaiLanSparseTableBorderRow>,
    pub(crate) horizontal_candidates: Vec<ShanaiLanSparseTableBorderHorizontalCandidate>,
    pub(crate) junction_candidates: Vec<ShanaiLanSparseTableBorderJunctionCandidate>,
    pub(crate) vertical_candidates: Vec<ShanaiLanSparseTableBorderVerticalCandidate>,
    pub(crate) cell_gap_midpoints: Vec<ShanaiLanSparseTableBorderCellGapMidpoint>,
    pub(crate) style_coverage: ShanaiLanSparseTableBorderStyleCoverage,
    pub(crate) source_page_transform_candidate: Option<ShanaiLanSourcePageTransformCandidate>,
    pub(crate) renderable: bool,
    pub(crate) blockers: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderRow {
    pub(crate) row_index: usize,
    pub(crate) group_index: usize,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) grid_extent_units: u16,
    pub(crate) w8_units: u16,
    pub(crate) line_mark_record_index: Option<usize>,
    pub(crate) line_mark_record_index_delta: Option<i32>,
    pub(crate) pairs: Vec<ShanaiLanSparseTableBorderPair>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderPair {
    pub(crate) pair_index: usize,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) state_code: u16,
    pub(crate) run_length: u16,
    pub(crate) start_unit: u32,
    pub(crate) end_unit: u32,
    pub(crate) blank_run: bool,
    pub(crate) upper_vertical_candidate: bool,
    pub(crate) lower_vertical_candidate: bool,
    pub(crate) top_horizontal_candidate: bool,
    pub(crate) bottom_horizontal_candidate: bool,
    pub(crate) style_source_covered: bool,
    pub(crate) upper_vertical_style_code: Option<u16>,
    pub(crate) lower_vertical_style_code: Option<u16>,
    pub(crate) top_horizontal_style_code: Option<u16>,
    pub(crate) bottom_horizontal_style_code: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderHorizontalCandidate {
    pub(crate) row_index: usize,
    pub(crate) group_index: usize,
    pub(crate) pair_index: usize,
    pub(crate) state_code: u16,
    pub(crate) start_unit: u32,
    pub(crate) end_unit: u32,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) edge_kind: ShanaiLanSparseTableBorderHorizontalEdgeKind,
    pub(crate) edge_style_code: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShanaiLanSparseTableBorderHorizontalEdgeKind {
    Top,
    Bottom,
}

impl ShanaiLanSparseTableBorderHorizontalEdgeKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderJunctionCandidate {
    pub(crate) row_index: usize,
    pub(crate) group_index: usize,
    pub(crate) pair_index: usize,
    pub(crate) state_code: u16,
    pub(crate) x_unit: u32,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) upper_vertical_candidate: bool,
    pub(crate) lower_vertical_candidate: bool,
    pub(crate) top_horizontal_candidate: bool,
    pub(crate) bottom_horizontal_candidate: bool,
    pub(crate) upper_vertical_style_code: Option<u16>,
    pub(crate) lower_vertical_style_code: Option<u16>,
    pub(crate) top_horizontal_style_code: Option<u16>,
    pub(crate) bottom_horizontal_style_code: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderVerticalCandidate {
    pub(crate) x_unit: u32,
    pub(crate) start_group_index: usize,
    pub(crate) end_group_index: usize,
    pub(crate) contributing_row_indexes: Vec<usize>,
    pub(crate) contributing_group_indexes: Vec<usize>,
    pub(crate) contributing_pair_indexes: Vec<usize>,
    pub(crate) contributing_source_spans: Vec<TextSourceSpan>,
    pub(crate) matching_gap_midpoint_units: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderCellGapMidpoint {
    pub(crate) group_index: usize,
    pub(crate) midpoint_unit: u32,
    pub(crate) left_extent_unit: u16,
    pub(crate) right_offset_unit: u16,
    pub(crate) left_source_span: TextSourceSpan,
    pub(crate) right_source_span: TextSourceSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderStyleState {
    pub(crate) upper_vertical_style_code: u16,
    pub(crate) lower_vertical_style_code: u16,
    pub(crate) top_horizontal_style_code: u16,
    pub(crate) bottom_horizontal_style_code: u16,
}

impl Default for ShanaiLanSparseTableBorderStyleState {
    fn default() -> Self {
        Self {
            upper_vertical_style_code: 0xffff,
            lower_vertical_style_code: 0x0000,
            top_horizontal_style_code: 0xffff,
            bottom_horizontal_style_code: 0xffff,
        }
    }
}

impl ShanaiLanSparseTableBorderStyleState {
    fn apply_property(
        &mut self,
        property_id: u8,
        typed_value: Option<DocumentTextStyleTypedValue>,
    ) {
        let Some(DocumentTextStyleTypedValue::U16(value)) = typed_value else {
            return;
        };
        match property_id {
            1 => self.upper_vertical_style_code = value,
            2 => self.lower_vertical_style_code = value,
            3 => self.top_horizontal_style_code = value,
            8 => self.bottom_horizontal_style_code = value,
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderStyleSpan {
    pub(crate) source_unit_start: usize,
    pub(crate) source_unit_end: usize,
    pub(crate) state: ShanaiLanSparseTableBorderStyleState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderStyleResolver {
    pub(crate) section_present: bool,
    pub(crate) content_unit_count: u32,
    pub(crate) style_start: usize,
    pub(crate) event_count: usize,
    pub(crate) truncated: bool,
    pub(crate) diagnostics: Vec<DocumentTextStyleDiagnostic>,
    pub(crate) spans: Vec<ShanaiLanSparseTableBorderStyleSpan>,
}

impl ShanaiLanSparseTableBorderStyleResolver {
    pub(crate) fn from_document_text_bytes(bytes: &[u8]) -> Self {
        let section = parse_document_text_style_section(bytes);
        Self::from_style_section(bytes.len(), &section)
    }

    fn from_style_section(bytes_len: usize, section: &DocumentTextStyleSection) -> Self {
        let mut spans = Vec::new();
        let mut state = ShanaiLanSparseTableBorderStyleState::default();
        for event in section.events() {
            match event {
                DocumentTextStyleEvent::Run(run) => {
                    spans.push(ShanaiLanSparseTableBorderStyleSpan {
                        source_unit_start: run.source_span().unit_start(),
                        source_unit_end: run.source_span().unit_end(),
                        state,
                    })
                }
                DocumentTextStyleEvent::PropertyChange(change) => {
                    for property in change.properties() {
                        state.apply_property(property.property_id(), property.typed_value());
                    }
                    spans.push(ShanaiLanSparseTableBorderStyleSpan {
                        source_unit_start: change.source_span().unit_start(),
                        source_unit_end: change.source_span().unit_end(),
                        state,
                    });
                }
            }
        }
        Self {
            section_present: section.style_start() < bytes_len && !section.events().is_empty(),
            content_unit_count: section.content_unit_count(),
            style_start: section.style_start(),
            event_count: section.events().len(),
            truncated: section.truncated(),
            diagnostics: section.diagnostics().to_vec(),
            spans,
        }
    }

    pub(crate) fn state_at_unit(
        &self,
        source_unit: usize,
    ) -> Option<ShanaiLanSparseTableBorderStyleState> {
        self.spans
            .iter()
            .find(|span| {
                span.source_unit_start <= source_unit && source_unit < span.source_unit_end
            })
            .map(|span| span.state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSparseTableBorderStyleCoverage {
    pub(crate) section_present: bool,
    pub(crate) content_unit_count: u32,
    pub(crate) style_start: usize,
    pub(crate) event_count: usize,
    pub(crate) truncated: bool,
    pub(crate) diagnostics: Vec<DocumentTextStyleDiagnostic>,
    pub(crate) relevant_source_units: Vec<usize>,
    pub(crate) covered_source_units: Vec<usize>,
    pub(crate) uncovered_source_units: Vec<usize>,
    pub(crate) horizontal_renderable_count: usize,
    pub(crate) vertical_renderable_half_count: usize,
}

impl ShanaiLanSparseTableBorderStyleCoverage {
    pub(crate) fn relevant_source_units_covered(&self) -> bool {
        self.section_present
            && !self.relevant_source_units.is_empty()
            && self.uncovered_source_units.is_empty()
    }

    pub(crate) fn admitted_render_segment_count(&self) -> usize {
        self.horizontal_renderable_count + self.vertical_renderable_half_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShanaiLanSourcePageTransformCandidate {
    pub(crate) page_mark_entry_index: usize,
    pub(crate) page_width_mm100: u32,
    pub(crate) page_height_mm100: u32,
    pub(crate) x_origin_left_mm100: u32,
    pub(crate) x_origin_right_mm100: u32,
    pub(crate) y_origin_mm100: u32,
    pub(crate) row_pitch_addend_a_mm100: u16,
    pub(crate) row_pitch_addend_b_mm100: u16,
    pub(crate) row_pitch_mm100: u32,
    pub(crate) page_mark_w21_mm100: Option<u16>,
}
