use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridSourceTopTextPlacementReadiness {
    pub(crate) ready: bool,
    pub(crate) blocked_reasons: Vec<&'static str>,
}

pub(crate) struct TableGridSourceTablePlacementCoherenceInput<'a> {
    pub(crate) layout: PageLayout,
    pub(crate) document: &'a Document,
    pub(crate) candidate: &'a TableCandidate,
    pub(crate) rows: &'a [TableCandidateLineHeaderRow],
    pub(crate) anchor_span: &'a TextSourceSpan,
    pub(crate) anchor_header: ShanaiLanLineHeader,
    pub(crate) table_min_offset_units: Option<u16>,
    pub(crate) table_max_extent_units: Option<u16>,
    pub(crate) table_font_size_units: Option<u16>,
    pub(crate) source_gap_after_anchor_text_units: usize,
}

impl TableGridSourceTopTextPlacementReadiness {
    pub(crate) fn blocked_reason(&self) -> Option<&'static str> {
        self.blocked_reasons.first().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridRelatedHorizontalSourceLayoutSummary {
    pub(crate) table_candidate_index: usize,
    pub(crate) row_count: usize,
    pub(crate) column_count: usize,
    pub(crate) x_unit_start: u16,
    pub(crate) x_unit_end: u16,
    pub(crate) x_unit_full_extent_units: u16,
    pub(crate) x_unit_all_rows_agree: bool,
    pub(crate) first_column_slot_units: Option<u16>,
    pub(crate) first_matched_cell_span_units: Option<u16>,
    pub(crate) first_intercell_gap_units: Option<u16>,
    pub(crate) matched_cell_span_units: Vec<u16>,
    pub(crate) matched_cell_gap_units: Vec<u16>,
    pub(crate) x_unit_column_slot_width_units: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridSparseSiblingPostRowGap {
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) sparse_row_indexes: Vec<usize>,
    pub(crate) sparse_source_interval_indexes: Vec<usize>,
    pub(crate) kind: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridPageMarkRawRecordSourceRangeCoverageSummary {
    pub(crate) record_header_count: usize,
    pub(crate) candidate_row_count: usize,
    pub(crate) row_source_coverage_count: usize,
    pub(crate) all_rows_have_header_coverage: bool,
    pub(crate) total_overlapping_header_count: usize,
    pub(crate) matched_scan_indexes: Vec<usize>,
    pub(crate) matched_scan_indexes_monotonic: bool,
    pub(crate) rows: Vec<TableGridPageMarkRawRecordSourceRangeCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridPageMarkRawRecordSourceRangeCoverageRow {
    pub(crate) row_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) matches: Vec<TableGridPageMarkRawRecordSourceRangeCoverageMatch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TableGridPageMarkRawRecordSourceRangeCoverageMatch {
    pub(crate) scan_index: usize,
    pub(crate) header: PageMarkRecordHeader,
    pub(crate) overlap_start: usize,
    pub(crate) overlap_end: usize,
    pub(crate) overlap_units: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PageMarkRawNumericHit {
    pub(crate) kind: &'static str,
    pub(crate) byte_offset: usize,
    pub(crate) value_index: usize,
    pub(crate) value: u32,
    pub(crate) residual_px: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageMarkRawNumericHitRecordContext {
    pub(crate) scan_index: usize,
    pub(crate) record_byte_offset: usize,
    pub(crate) record_next_byte_offset: usize,
    pub(crate) record_index: u32,
    pub(crate) record_line_start: u32,
    pub(crate) record_line_end: u32,
    pub(crate) record_relative_byte_offset: usize,
    pub(crate) record_tail_relative_byte_offset: Option<usize>,
    pub(crate) record_tail_word_index: Option<usize>,
    pub(crate) record_tail_block16_index: Option<usize>,
    pub(crate) record_tail_block16_word_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageMarkScopedYValueCandidate {
    pub(crate) source: &'static str,
    pub(crate) interpretation: &'static str,
    pub(crate) word_index: Option<usize>,
    pub(crate) byte_offset: Option<usize>,
    pub(crate) value: u32,
    pub(crate) value_px: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageMarkScopedYDeltaCandidate {
    pub(crate) source: &'static str,
    pub(crate) interpretation: &'static str,
    pub(crate) left_word_index: Option<usize>,
    pub(crate) right_word_index: Option<usize>,
    pub(crate) left_byte_offset: Option<usize>,
    pub(crate) right_byte_offset: Option<usize>,
    pub(crate) left_value: u32,
    pub(crate) right_value: u32,
    pub(crate) left_value_px: f32,
    pub(crate) right_value_px: f32,
    pub(crate) delta_px: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageMarkScopedYFamilyMember {
    pub(crate) source: &'static str,
    pub(crate) interpretation: &'static str,
    pub(crate) family_kind: &'static str,
    pub(crate) field_index: usize,
    pub(crate) word_index: Option<usize>,
    pub(crate) byte_offset: Option<usize>,
    pub(crate) raw_record_index: Option<u32>,
    pub(crate) raw_record_scan_index: Option<usize>,
    pub(crate) tail_block16_word_index: Option<usize>,
    pub(crate) subrecord_line_start_candidate: Option<u32>,
    pub(crate) subrecord_line_end_candidate: Option<u32>,
    pub(crate) value: u32,
    pub(crate) value_px: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageMarkScopedYFamilyFit {
    pub(crate) source: &'static str,
    pub(crate) interpretation: &'static str,
    pub(crate) family_kind: &'static str,
    pub(crate) field_index: usize,
    pub(crate) members: Vec<PageMarkScopedYFamilyMember>,
    pub(crate) table_top_residuals: Vec<f32>,
    pub(crate) table_top_hit_members: Vec<PageMarkScopedYFamilyMember>,
    pub(crate) row_top_residuals: Vec<f32>,
    pub(crate) row_delta_residuals: Vec<f32>,
    pub(crate) table_top_hit_count: usize,
    pub(crate) row_top_coverage_count: usize,
    pub(crate) row_delta_coverage_count: usize,
    pub(crate) row_line_range_coverage_count: usize,
    pub(crate) table_top_hit_line_range_coverage_count: usize,
    pub(crate) row_top_mean_abs_residual: Option<f32>,
    pub(crate) row_top_max_abs_residual: Option<f32>,
    pub(crate) row_delta_mean_abs_residual: Option<f32>,
    pub(crate) row_delta_max_abs_residual: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PageMarkScopedYSlotFit {
    pub(crate) source: &'static str,
    pub(crate) interpretation: &'static str,
    pub(crate) field_index: usize,
    pub(crate) tail_block16_word_index: usize,
    pub(crate) members: Vec<PageMarkScopedYFamilyMember>,
    pub(crate) table_top_residuals: Vec<f32>,
    pub(crate) row_top_residuals: Vec<f32>,
    pub(crate) row_delta_residuals: Vec<f32>,
    pub(crate) table_top_hit_count: usize,
    pub(crate) row_top_coverage_count: usize,
    pub(crate) row_delta_coverage_count: usize,
    pub(crate) row_line_range_coverage_count: usize,
    pub(crate) ordered_line_mark_record_coverage_count: usize,
    pub(crate) ordered_line_mark_record_coverage_complete: bool,
    pub(crate) ordered_line_mark_record_indexes_covered: Vec<usize>,
    pub(crate) ordered_line_mark_record_member_byte_offsets: Vec<usize>,
    pub(crate) row_top_mean_abs_residual: Option<f32>,
    pub(crate) row_top_max_abs_residual: Option<f32>,
    pub(crate) row_delta_mean_abs_residual: Option<f32>,
    pub(crate) row_delta_max_abs_residual: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageMarkScopedYOrderedLineRangeCoverage {
    pub(crate) record_indexes_covered: Vec<usize>,
    pub(crate) member_byte_offsets: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageMarkRawSubrecordLineSpanCandidate {
    pub(crate) byte_offset: usize,
    pub(crate) raw_record_index: u32,
    pub(crate) raw_record_scan_index: usize,
    pub(crate) tail_block16_word_index: Option<usize>,
    pub(crate) line_start_candidate: u16,
    pub(crate) line_end_candidate: u16,
    pub(crate) line_span_units: usize,
    pub(crate) field2_value: u16,
    pub(crate) words: [u16; 8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridPageMarkSubrecordLineSpanReadiness {
    pub(crate) selected_record_indexes: Vec<usize>,
    pub(crate) previous_record_indexes: Vec<usize>,
    pub(crate) selected_post_row_gap_span_targets: Vec<usize>,
    pub(crate) post_row_gap_span_targets: Vec<usize>,
    pub(crate) previous_row_span_targets: Vec<usize>,
    pub(crate) compact_row_span_targets: Vec<usize>,
    pub(crate) candidate_count: usize,
    pub(crate) selected_post_row_gap_span_hit_count: usize,
    pub(crate) previous_row_span_hit_count: usize,
    pub(crate) compact_row_span_hit_count: usize,
    pub(crate) selected_post_row_gap_span_max_abs_residual_units: Option<i32>,
    pub(crate) previous_row_span_max_abs_residual_units: Option<i32>,
    pub(crate) compact_row_span_max_abs_residual_units: Option<i32>,
    pub(crate) selected_post_row_gap_span_coverage: TableGridPageMarkSubrecordLineSpanCoverage,
    pub(crate) previous_row_span_coverage: TableGridPageMarkSubrecordLineSpanCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageMarkSubrecordLineSpanMatch<'a> {
    pub(crate) target_index: usize,
    pub(crate) target_units: usize,
    pub(crate) residual_units: i32,
    pub(crate) candidate: &'a PageMarkRawSubrecordLineSpanCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridPageMarkSubrecordLineSpanCoverage {
    pub(crate) matched_record_indexes: Vec<usize>,
    pub(crate) matched_candidate_byte_offsets: Vec<usize>,
    pub(crate) unique_candidate_byte_offsets: Vec<usize>,
    pub(crate) duplicate_candidate_byte_offsets: Vec<usize>,
    pub(crate) ordered_unique_coverage_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridCrossTableSubrecordOrderingProbe {
    pub(crate) current_table_candidate_index: usize,
    pub(crate) related_table_candidate_indexes: Vec<usize>,
    pub(crate) combined_line_mark_record_indexes: Vec<usize>,
    pub(crate) combined_matched_byte_offsets: Vec<usize>,
    pub(crate) combined_raw_record_scan_indexes: Vec<usize>,
    pub(crate) combined_tail_block16_word_indexes: Vec<Option<usize>>,
    pub(crate) combined_line_start_candidates: Vec<u16>,
    pub(crate) combined_line_end_candidates: Vec<u16>,
    pub(crate) combined_field2_values: Vec<u16>,
    pub(crate) monotonic_raw_record_scan_index: bool,
    pub(crate) monotonic_line_start_candidate: bool,
    pub(crate) family_reused_after_later_family: bool,
    pub(crate) cross_table_ordering_consistent: bool,
    pub(crate) tables: Vec<TableGridCrossTableSubrecordOrderingTable>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridCrossTableSubrecordOrderingTable {
    pub(crate) table_candidate_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) row_count: usize,
    pub(crate) matched_rows: Vec<TableGridCrossTableSubrecordOrderingMatch>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridCrossTableSubrecordOrderingMatch {
    pub(crate) row_index: usize,
    pub(crate) line_mark_record_index: usize,
    pub(crate) target_units: usize,
    pub(crate) residual_units: i32,
    pub(crate) byte_offset: usize,
    pub(crate) raw_record_index: u32,
    pub(crate) raw_record_scan_index: usize,
    pub(crate) tail_block16_word_index: Option<usize>,
    pub(crate) line_start_candidate: u16,
    pub(crate) line_end_candidate: u16,
    pub(crate) field2_value: u16,
}

pub(crate) fn usize_values_are_monotonic_non_decreasing(values: &[usize]) -> bool {
    values.windows(2).all(|pair| pair[0] <= pair[1])
}

pub(crate) fn u16_values_are_monotonic_non_decreasing(values: &[u16]) -> bool {
    values.windows(2).all(|pair| pair[0] <= pair[1])
}

pub(crate) fn values_reused_after_different_value(values: &[usize]) -> bool {
    let mut last_seen = BTreeMap::<usize, usize>::new();
    for (index, value) in values.iter().copied().enumerate() {
        if let Some(previous_index) = last_seen.insert(value, index)
            && values[previous_index + 1..index]
                .iter()
                .any(|between| *between != value)
        {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TableGridCrossTableRowBoundaryOffsetProbe {
    pub(crate) current_table_candidate_index: usize,
    pub(crate) sparse_table_candidate_index: usize,
    pub(crate) related_table_candidate_indexes: Vec<usize>,
    pub(crate) related_table_count: usize,
    pub(crate) table_count_with_previous_row_span_alignment: usize,
    pub(crate) row_boundary_offset_candidate_units: Vec<i32>,
    pub(crate) stable_row_boundary_offset_candidate_units: Option<i32>,
    pub(crate) all_related_tables_have_offset_candidate: bool,
    pub(crate) all_offsets_stable: bool,
    pub(crate) all_offsets_require_transform: bool,
    pub(crate) all_offset_normalized_boundaries_exact: bool,
    pub(crate) combined_line_mark_record_indexes: Vec<usize>,
    pub(crate) page_mark_entry_index: Option<usize>,
    pub(crate) page_index_candidate: Option<usize>,
    pub(crate) page_line_start: Option<usize>,
    pub(crate) page_line_end: Option<usize>,
    pub(crate) page_mark_u16_field_count: usize,
    pub(crate) page_mark_u16_field_preview: Vec<u16>,
    pub(crate) combined_line_offsets_from_page_start: Vec<usize>,
    pub(crate) combined_line_offsets_monotonic: bool,
    pub(crate) combined_line_mark_record_y_pitch_px: Option<f32>,
    pub(crate) combined_line_mark_record_y_pitch_basis: Option<&'static str>,
    pub(crate) combined_line_mark_record_y_tops_px: Vec<f32>,
    pub(crate) combined_line_mark_record_y_span_px: Option<f32>,
    pub(crate) source_unit_to_page_line_index_source_units: Vec<usize>,
    pub(crate) source_unit_to_page_line_index_slope: Option<f32>,
    pub(crate) source_unit_to_page_line_index_intercept: Option<f32>,
    pub(crate) source_unit_to_page_line_index_fitted_indexes: Vec<f32>,
    pub(crate) source_unit_to_page_line_index_residual_indexes: Vec<f32>,
    pub(crate) source_unit_to_page_line_index_max_abs_residual: Option<f32>,
    pub(crate) source_unit_to_page_line_index_exact: bool,
    pub(crate) source_unit_to_page_line_index_rows: Vec<TableGridSourceUnitToPageLineIndexFitRow>,
    pub(crate) source_unit_to_page_line_index_piecewise_max_abs_residual: Option<f32>,
    pub(crate) source_unit_to_page_line_index_piecewise_all_tables_exact: bool,
    pub(crate) source_unit_to_page_line_index_piecewise_tables:
        Vec<TableGridSourceUnitToPageLineIndexPiecewiseTable>,
    pub(crate) source_unit_to_page_line_index_piecewise_transitions:
        Vec<TableGridSourceUnitToPageLineIndexPiecewiseTransition>,
    pub(crate) all_records_within_single_page_mark_entry: bool,
    pub(crate) tables: Vec<TableGridCrossTableRowBoundaryOffsetTable>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TableGridCrossTableRowBoundaryOffsetTable {
    pub(crate) table_candidate_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) row_count: usize,
    pub(crate) line_mark_record_indexes: Vec<usize>,
    pub(crate) page_mark_line_offsets_from_entry_start: Vec<usize>,
    pub(crate) page_mark_records_within_single_entry: bool,
    pub(crate) line_mark_record_y_tops_px: Vec<f32>,
    pub(crate) selected_spacing_record_indexes: Vec<usize>,
    pub(crate) selected_spacing_page_mark_line_offsets_from_entry_start: Vec<usize>,
    pub(crate) selected_spacing_records_within_single_entry: bool,
    pub(crate) selected_spacing_record_y_tops_px: Vec<f32>,
    pub(crate) selected_spacing_line_mark_start_units: Vec<usize>,
    pub(crate) selected_spacing_line_mark_end_units: Vec<usize>,
    pub(crate) selected_spacing_start_residual_units: Vec<i32>,
    pub(crate) selected_spacing_end_residual_units: Vec<i32>,
    pub(crate) selected_spacing_span_residual_units: Vec<i32>,
    pub(crate) selected_minus_previous_record_index_gaps: Vec<i32>,
    pub(crate) selected_minus_previous_record_y_delta_px: Vec<f32>,
    pub(crate) row_source_start_units: Vec<usize>,
    pub(crate) row_source_end_units: Vec<usize>,
    pub(crate) line_mark_start_units: Vec<usize>,
    pub(crate) line_mark_end_units: Vec<usize>,
    pub(crate) start_residual_units: Vec<i32>,
    pub(crate) end_residual_units: Vec<i32>,
    pub(crate) span_residual_units: Vec<i32>,
    pub(crate) row_boundary_offset_candidate_units: Option<i32>,
    pub(crate) offset_normalized_start_residual_units: Vec<i32>,
    pub(crate) offset_normalized_end_residual_units: Vec<i32>,
    pub(crate) offset_normalized_exact_boundary_aligned: bool,
    pub(crate) exact_boundary_aligned: bool,
    pub(crate) span_only_match: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TableGridSourceUnitToPageLineIndexFitRow {
    pub(crate) table_candidate_index: usize,
    pub(crate) row_index: usize,
    pub(crate) row_source_start_units: usize,
    pub(crate) line_mark_record_index: usize,
    pub(crate) fitted_record_index: f32,
    pub(crate) residual_record_index: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TableGridSourceUnitToPageLineIndexPiecewiseTable {
    pub(crate) table_candidate_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) row_count: usize,
    pub(crate) row_source_start_units: Vec<usize>,
    pub(crate) line_mark_record_indexes: Vec<usize>,
    pub(crate) slope_record_indexes_per_source_unit: Option<f32>,
    pub(crate) intercept_record_index: Option<f32>,
    pub(crate) fitted_record_indexes: Vec<f32>,
    pub(crate) residual_record_indexes: Vec<f32>,
    pub(crate) max_abs_residual_record_indexes: Option<f32>,
    pub(crate) exact_fit: bool,
    pub(crate) page_mark_records_within_single_entry: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TableGridSourceUnitToPageLineIndexPiecewiseTransition {
    pub(crate) from_table_candidate_index: usize,
    pub(crate) to_table_candidate_index: usize,
    pub(crate) previous_last_source_unit: usize,
    pub(crate) next_first_source_unit: usize,
    pub(crate) source_range_gap_units: usize,
    pub(crate) row_source_start_gap_units: i32,
    pub(crate) previous_last_record_index: usize,
    pub(crate) next_first_record_index: usize,
    pub(crate) line_mark_record_gap: i32,
    pub(crate) same_page_mark_entry: bool,
}

pub(crate) fn adjacent_f32_deltas(values: &[f32]) -> Vec<f32> {
    values.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

pub(crate) fn max_abs_i32(values: &[i32]) -> Option<i32> {
    values.iter().map(|value| value.saturating_abs()).max()
}

pub(crate) fn row_source_start_gap_minus_source_range_gap_units(
    row_source_start_gap_units: i32,
    source_range_gap_units: usize,
) -> i32 {
    let Ok(source_range_gap_units) = i32::try_from(source_range_gap_units) else {
        return i32::MIN;
    };
    row_source_start_gap_units.saturating_sub(source_range_gap_units)
}

pub(crate) fn single_i32_value(values: &[i32]) -> Option<i32> {
    let first = *values.first()?;
    values.iter().all(|value| *value == first).then_some(first)
}

pub(crate) fn single_u16_value(values: &[u16]) -> Option<u16> {
    let first = *values.first()?;
    values.iter().all(|value| *value == first).then_some(first)
}

pub(crate) fn ratio_usize_by_i32(numerators: &[usize], denominators: &[i32]) -> Vec<f32> {
    numerators
        .iter()
        .copied()
        .zip(denominators.iter().copied())
        .filter_map(|(numerator, denominator)| {
            (denominator != 0).then_some(numerator as f32 / denominator as f32)
        })
        .filter(|ratio| ratio.is_finite())
        .collect()
}

pub(crate) fn ratio_i32_by_i32(numerators: &[i32], denominators: &[i32]) -> Vec<f32> {
    numerators
        .iter()
        .copied()
        .zip(denominators.iter().copied())
        .filter_map(|(numerator, denominator)| {
            (denominator != 0).then_some(numerator as f32 / denominator as f32)
        })
        .filter(|ratio| ratio.is_finite())
        .collect()
}

pub(crate) fn ratio_f32_by_i32(numerators: &[f32], denominators: &[i32]) -> Vec<f32> {
    numerators
        .iter()
        .copied()
        .zip(denominators.iter().copied())
        .filter_map(|(numerator, denominator)| {
            (denominator != 0).then_some(numerator / denominator as f32)
        })
        .filter(|ratio| ratio.is_finite())
        .collect()
}

pub(crate) fn rounded_f32_values_all_same(values: &[f32]) -> bool {
    let Some(first) = values.first().map(|value| rounded_milli(*value)) else {
        return false;
    };
    values
        .iter()
        .map(|value| rounded_milli(*value))
        .all(|value| value == first)
}

pub(crate) fn f32_value_spread(values: &[f32]) -> Option<f32> {
    let mut iter = values.iter().copied().filter(|value| value.is_finite());
    let first = iter.next()?;
    let (min, max) = iter.fold((first, first), |(min, max), value| {
        (min.min(value), max.max(value))
    });
    Some(max - min)
}

pub(crate) fn signed_usize_delta_i32(left: usize, right: usize) -> i32 {
    if left >= right {
        i32::try_from(left - right).unwrap_or(i32::MAX)
    } else {
        i32::try_from(right - left).map_or(i32::MIN, |delta| -delta)
    }
}

pub(crate) fn push_optional_i32_json(output: &mut String, value: Option<i32>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

pub(crate) fn single_usize_value(values: &[usize]) -> Option<usize> {
    let first = *values.first()?;
    values.iter().all(|value| *value == first).then_some(first)
}

pub(crate) fn option_f32_order(left: Option<f32>, right: Option<f32>) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.partial_cmp(&right).unwrap_or(Ordering::Equal),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

pub(crate) fn mean_and_max_abs_residual(residuals: &[f32]) -> (Option<f32>, Option<f32>) {
    if residuals.is_empty() {
        return (None, None);
    }
    let mean =
        residuals.iter().map(|residual| residual.abs()).sum::<f32>() / residuals.len() as f32;
    let max = residuals
        .iter()
        .map(|residual| residual.abs())
        .fold(0.0f32, f32::max);
    (Some(mean), Some(max))
}

pub(crate) fn push_option_f32_json(output: &mut String, value: Option<f32>) {
    match value {
        Some(value) if value.is_finite() => output.push_str(&format!("{value:.3}")),
        _ => output.push_str("null"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageMarkRawU16SubrecordCandidate {
    pub(crate) byte_offset: usize,
    pub(crate) field_index: usize,
    pub(crate) words: [u16; 8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridLineMarkRowGapSequenceRow {
    pub(crate) compact_row_index: usize,
    pub(crate) sparse_row_index: usize,
    pub(crate) source_interval_index: usize,
    pub(crate) row_source_start: usize,
    pub(crate) row_source_end: usize,
    pub(crate) row_source_start_units: usize,
    pub(crate) row_source_end_units: usize,
    pub(crate) selected_line_mark: ShanaiLanLineMarkInterval,
    pub(crate) previous_line_mark: Option<ShanaiLanLineMarkInterval>,
    pub(crate) next_line_mark: Option<ShanaiLanLineMarkInterval>,
    pub(crate) post_row_gap: Option<TableGridSparseSiblingPostRowGap>,
    pub(crate) next_row_span_units: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridPageMarkLineContext {
    pub(crate) page_mark_entry_index: usize,
    pub(crate) page_index_candidate: Option<usize>,
    pub(crate) page_line_start: usize,
    pub(crate) page_line_end: usize,
    pub(crate) page_mark_u16_fields: Vec<u16>,
}

pub(crate) fn residuals_f32(candidates: &[f32], references: &[f32]) -> Vec<f32> {
    candidates
        .iter()
        .zip(references)
        .map(|(candidate, reference)| candidate - reference)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TableGridResolvedLineMarkRow {
    pub(crate) interval: ShanaiLanLineMarkInterval,
    pub(crate) role: TableGridLineMarkRowRecordRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableGridLineMarkRowRecordRole {
    SelectedOverlap,
    PreviousCompactRowSpan,
}

impl TableGridLineMarkRowRecordRole {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::SelectedOverlap => "selected-overlap-record",
            Self::PreviousCompactRowSpan => "previous-compact-row-span-record",
        }
    }
}

pub(crate) fn push_f32_array_json(output: &mut String, values: &[f32]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!("{value:.3}"));
    }
    output.push(']');
}

pub(crate) fn max_abs_f32(values: &[f32]) -> Option<f32> {
    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .map(f32::abs)
        .reduce(f32::max)
}

pub(crate) fn mean_abs_f32(values: &[f32]) -> Option<f32> {
    let mut sum = 0.0f32;
    let mut count = 0usize;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        sum += value.abs();
        count += 1;
    }
    (count > 0).then(|| sum / count as f32)
}

pub(crate) fn mean_f32(values: &[f32]) -> Option<f32> {
    let mut sum = 0.0f32;
    let mut count = 0usize;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        sum += value;
        count += 1;
    }
    (count > 0).then(|| sum / count as f32)
}

pub(crate) fn slope_from_indexed_tops(indexes: &[f32], tops: &[f32]) -> Option<f32> {
    let first_index = *indexes.first()?;
    let last_index = *indexes.last()?;
    let first_top = *tops.first()?;
    let last_top = *tops.last()?;
    let index_span = last_index - first_index;
    (index_span.is_finite() && index_span.abs() > f32::EPSILON)
        .then_some((last_top - first_top) / index_span)
}

pub(crate) fn affine_fit_f32(xs: &[f32], ys: &[f32]) -> Option<(f32, f32)> {
    if xs.len() != ys.len() || xs.len() < 2 {
        return None;
    }
    let mean_x = mean_f32(xs)?;
    let mean_y = mean_f32(ys)?;
    let mut numerator = 0.0f32;
    let mut denominator = 0.0f32;
    for (x, y) in xs.iter().copied().zip(ys.iter().copied()) {
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        let dx = x - mean_x;
        numerator += dx * (y - mean_y);
        denominator += dx * dx;
    }
    if !denominator.is_finite() || denominator.abs() <= f32::EPSILON {
        return None;
    }
    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;
    (slope.is_finite() && intercept.is_finite()).then_some((slope, intercept))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridSparseSiblingSegmentMatch {
    pub(crate) compact_column_index: usize,
    pub(crate) sparse_column_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) text_matches: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridSparseSiblingRowMatch {
    pub(crate) compact_row_index: usize,
    pub(crate) sparse_row_index: usize,
    pub(crate) source_interval_index: usize,
    pub(crate) source_start: usize,
    pub(crate) source_end: usize,
    pub(crate) compact_cell_count: usize,
    pub(crate) sparse_cell_count: usize,
    pub(crate) sparse_empty_cell_count: usize,
    pub(crate) sparse_non_empty_cell_count: usize,
    pub(crate) first_non_empty_sparse_column_index: Option<usize>,
    pub(crate) last_non_empty_sparse_column_index: Option<usize>,
    pub(crate) compact_to_sparse_column_offset: Option<usize>,
    pub(crate) segments: Vec<TableGridSparseSiblingSegmentMatch>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridSparseSiblingEvidence<'a> {
    pub(crate) sparse_candidate: &'a TableCandidate,
    pub(crate) rows: Vec<TableGridSparseSiblingRowMatch>,
    pub(crate) compact_to_sparse_column_offset: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableGridUnitBBoxBasis {
    MatchedCells,
    MatchedCellsPlusFirstTrailingHeader,
    FullLineHeaderExtent,
}

impl TableGridUnitBBoxBasis {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            TableGridUnitBBoxBasis::MatchedCells => "matched-cells",
            TableGridUnitBBoxBasis::MatchedCellsPlusFirstTrailingHeader => {
                "matched-cells-plus-first-trailing-header"
            }
            TableGridUnitBBoxBasis::FullLineHeaderExtent => "full-line-header-extent",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridHorizontalFrameCandidateSupport {
    pub(crate) frame_basis: &'static str,
    pub(crate) selected_x: f32,
    pub(crate) selected_width: f32,
    pub(crate) contribution: &'static str,
    pub(crate) blocked_reason: &'static str,
}

pub(crate) fn rounded_milli(value: f32) -> i32 {
    (value * 1000.0).round() as i32
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate {
    pub(crate) source: &'static str,
    pub(crate) interpretation: &'static str,
    pub(crate) field_index: usize,
    pub(crate) tail_block16_word_index: Option<usize>,
    pub(crate) raw_record_scan_index: Option<usize>,
    pub(crate) raw_record_index: Option<u32>,
    pub(crate) byte_offset: usize,
    pub(crate) subrecord_byte_offset: usize,
    pub(crate) subrecord_line_start_candidate: u16,
    pub(crate) subrecord_line_end_candidate: u16,
    pub(crate) value: u16,
    pub(crate) value_px: f32,
}

/// Row-ordered value scan of the absolute-y-slot field across the subrecords the
/// selected post-row-gap coverage matched. Diagnostic-only: it tests whether the
/// field can be direct page-space px at all, it does not decode the field.
#[derive(Debug, Clone)]
pub(crate) struct TableGridSourceOnlyPageMarkFieldQuantization {
    pub(crate) field_index: usize,
    pub(crate) tail_block16_word_index: Option<usize>,
    pub(crate) quantum_units: u16,
    pub(crate) value_count: usize,
    pub(crate) row_values: Vec<u16>,
    pub(crate) distinct_values: Vec<u16>,
    pub(crate) all_values_multiple_of_quantum: bool,
    pub(crate) low_byte_all_zero: bool,
    pub(crate) high_byte_values: Vec<u16>,
    pub(crate) raw_record_scan_indexes: Vec<usize>,
    pub(crate) values_constant_per_raw_record_scan_index: bool,
    pub(crate) value_row_distinct: bool,
    pub(crate) page_space_px_plausible: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
    pub(crate) line_domain_y: Option<f32>,
    pub(crate) selected_span_units: Option<usize>,
    pub(crate) line_domain_projected_y: Option<f32>,
    pub(crate) candidates: Vec<TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate>,
    pub(crate) best_absolute_y_slot: Option<TableGridSourceOnlyPageMarkAbsoluteYSlotCandidate>,
    pub(crate) residual_px: Option<f32>,
    pub(crate) agrees: bool,
    pub(crate) field_quantization: Option<TableGridSourceOnlyPageMarkFieldQuantization>,
}

impl TableGridSourceOnlyPageMarkAbsoluteYSlotAgreement {
    pub(crate) fn semantics_ready(&self) -> bool {
        self.line_domain_projected_y.is_some()
            && self.best_absolute_y_slot.is_some()
            && self.agrees
            && !self.field_quantization_refutes_page_space_px()
    }

    pub(crate) fn field_quantization_refutes_page_space_px(&self) -> bool {
        self.field_quantization
            .as_ref()
            .is_some_and(|quantization| !quantization.page_space_px_plausible)
    }

    pub(crate) fn field_quantization_blocked_reasons(&self) -> Vec<&'static str> {
        let mut reasons = Vec::new();
        let Some(quantization) = self.field_quantization.as_ref() else {
            return reasons;
        };
        if quantization.all_values_multiple_of_quantum && quantization.low_byte_all_zero {
            reasons.push("page-mark-absolute-y-slot-field-quantized-not-page-space-px");
        }
        if quantization.values_constant_per_raw_record_scan_index
            && !quantization.value_row_distinct
        {
            reasons
                .push("page-mark-absolute-y-slot-field-constant-per-raw-record-not-row-distinct");
        }
        reasons
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridSourceGapToPageLineGapReadinessHints {
    pub(crate) transition_count: usize,
    pub(crate) same_page_mark_entry_transition_count: usize,
    pub(crate) all_transitions_same_page_mark_entry: bool,
    pub(crate) source_range_gap_to_page_line_gap_max_abs_delta_units: Option<i32>,
    pub(crate) row_source_start_gap_to_page_line_gap_max_abs_delta_units: Option<i32>,
    pub(crate) segment_offset_gap_to_page_line_gap_max_abs_delta_units: Option<i32>,
    pub(crate) best_candidate_transform_kind: Option<&'static str>,
    pub(crate) best_candidate_max_abs_delta_units: Option<i32>,
    pub(crate) source_range_units_per_page_line_gap_spread: Option<f32>,
    pub(crate) row_source_start_units_per_page_line_gap_spread: Option<f32>,
    pub(crate) segment_offset_units_per_page_line_gap_spread: Option<f32>,
    pub(crate) affine_row_source_start_gap_fit: Option<TableGridAffineRowSourceStartGapFit>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TableGridSourceGapToPageLineGapTransformCandidateSummary {
    pub(crate) kind: &'static str,
    pub(crate) max_abs_delta_units: Option<i32>,
    pub(crate) units_per_page_line_gap_spread: Option<f32>,
    pub(crate) affine_row_source_start_gap_fit: Option<TableGridAffineRowSourceStartGapFit>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TableGridAffineRowSourceStartGapFit {
    pub(crate) numerator_slope: i64,
    pub(crate) denominator_slope: i64,
    pub(crate) numerator_intercept: i64,
    pub(crate) denominator_intercept: i64,
    pub(crate) max_abs_residual: f64,
    pub(crate) max_abs_residual_ceiling_units: i32,
    pub(crate) sample_count: usize,
    pub(crate) family_scoped: bool,
    pub(crate) fit_stable: bool,
}

impl TableGridAffineRowSourceStartGapFit {
    pub(crate) fn blocked_reason(&self) -> &'static str {
        "affine-row-source-start-gap-family-transform-authority-unproven"
    }

    pub(crate) fn max_abs_residual_ceiling_units(&self) -> i32 {
        self.max_abs_residual_ceiling_units
    }
}

impl TableGridSourceGapToPageLineGapReadinessHints {
    pub(crate) fn source_gap_to_page_line_gap_transform_stable(&self) -> bool {
        self.transition_count > 0 && self.best_candidate_max_abs_delta_units == Some(0)
    }

    pub(crate) fn table_family_source_gap_to_page_line_gap_transform_stable(&self) -> bool {
        self.source_gap_to_page_line_gap_transform_stable()
            && self.all_transitions_same_page_mark_entry
    }

    pub(crate) fn transform_blocked_reason(&self) -> Option<&'static str> {
        (!self.source_gap_to_page_line_gap_transform_stable())
            .then_some("source-gap-to-page-line-gap-transform-not-stable")
    }

    pub(crate) fn table_family_transform_blocked_reason(&self) -> Option<&'static str> {
        if self.transition_count == 0 {
            Some("source-gap-to-page-line-gap-transform-evidence-absent")
        } else if !self.all_transitions_same_page_mark_entry {
            Some("source-gap-to-page-line-gap-transform-crosses-page-mark-entries")
        } else if !self.source_gap_to_page_line_gap_transform_stable() {
            Some("source-gap-to-page-line-gap-transform-unstable-across-table-family")
        } else {
            None
        }
    }

    pub(crate) fn transform_candidate_summaries(
        &self,
    ) -> Vec<TableGridSourceGapToPageLineGapTransformCandidateSummary> {
        let mut summaries = vec![
            TableGridSourceGapToPageLineGapTransformCandidateSummary {
                kind: "direct-source-range-gap",
                max_abs_delta_units: self.source_range_gap_to_page_line_gap_max_abs_delta_units,
                units_per_page_line_gap_spread: self.source_range_units_per_page_line_gap_spread,
                affine_row_source_start_gap_fit: None,
            },
            TableGridSourceGapToPageLineGapTransformCandidateSummary {
                kind: "direct-row-source-start-gap",
                max_abs_delta_units: self.row_source_start_gap_to_page_line_gap_max_abs_delta_units,
                units_per_page_line_gap_spread: self
                    .row_source_start_units_per_page_line_gap_spread,
                affine_row_source_start_gap_fit: None,
            },
            TableGridSourceGapToPageLineGapTransformCandidateSummary {
                kind: "segment-offset-gap",
                max_abs_delta_units: self.segment_offset_gap_to_page_line_gap_max_abs_delta_units,
                units_per_page_line_gap_spread: self.segment_offset_units_per_page_line_gap_spread,
                affine_row_source_start_gap_fit: None,
            },
        ];
        if let Some(fit) = self.affine_row_source_start_gap_fit {
            summaries.push(TableGridSourceGapToPageLineGapTransformCandidateSummary {
                kind: "affine-row-source-start-gap",
                max_abs_delta_units: Some(fit.max_abs_residual_ceiling_units()),
                units_per_page_line_gap_spread: None,
                affine_row_source_start_gap_fit: Some(fit),
            });
        }
        summaries
    }

    pub(crate) fn transform_candidate_count(&self) -> usize {
        self.transform_candidate_summaries()
            .iter()
            .filter(|candidate| {
                candidate.max_abs_delta_units.is_some()
                    || candidate.affine_row_source_start_gap_fit.is_some()
            })
            .count()
    }

    pub(crate) fn exact_transform_candidate_count(&self) -> usize {
        self.transform_candidate_summaries()
            .iter()
            .filter(|candidate| {
                candidate.affine_row_source_start_gap_fit.is_none()
                    && candidate.max_abs_delta_units == Some(0)
            })
            .count()
    }

    pub(crate) fn best_candidate_transition_coverage_count(&self) -> usize {
        if self.best_candidate_transform_kind.is_some() {
            self.transition_count
        } else {
            0
        }
    }

    pub(crate) fn best_candidate_units_per_page_line_gap_spread(&self) -> Option<f32> {
        let best_kind = self.best_candidate_transform_kind?;
        self.transform_candidate_summaries()
            .iter()
            .find(|candidate| candidate.kind == best_kind)
            .and_then(|candidate| candidate.units_per_page_line_gap_spread)
    }

    pub(crate) fn lowest_spread_candidate(&self) -> Option<(&'static str, f32)> {
        let summaries = self.transform_candidate_summaries();
        summaries
            .iter()
            .filter_map(|candidate| {
                candidate
                    .units_per_page_line_gap_spread
                    .map(|spread| (candidate.kind, spread))
            })
            .min_by(|left, right| left.1.total_cmp(&right.1))
    }
}

pub(crate) fn affine_row_source_start_gap_fit(
    page_line_gaps: &[i32],
    row_source_start_gap_units: &[i32],
    family_scoped: bool,
) -> Option<TableGridAffineRowSourceStartGapFit> {
    if !family_scoped {
        return None;
    }
    if page_line_gaps.len() != row_source_start_gap_units.len() {
        return None;
    }
    let sample_count = page_line_gaps.len();
    if sample_count < 3 {
        return None;
    }
    let n = i64::try_from(sample_count).ok()?;
    let page_line_gaps = &page_line_gaps[..sample_count];
    let row_source_start_gap_units = &row_source_start_gap_units[..sample_count];
    let first_page_line_gap = *page_line_gaps.first()?;
    if page_line_gaps
        .iter()
        .all(|page_line_gap| *page_line_gap == first_page_line_gap)
    {
        return None;
    }

    let sum_y = page_line_gaps
        .iter()
        .copied()
        .map(i64::from)
        .try_fold(0_i64, |accumulator, page_line_gap| {
            accumulator.checked_add(page_line_gap)
        })?;
    let sum_x = row_source_start_gap_units
        .iter()
        .copied()
        .map(i64::from)
        .try_fold(0_i64, |accumulator, row_source_start_gap| {
            accumulator.checked_add(row_source_start_gap)
        })?;
    let sum_xy = page_line_gaps
        .iter()
        .copied()
        .zip(row_source_start_gap_units.iter().copied())
        .try_fold(
            0_i64,
            |accumulator, (page_line_gap, row_source_start_gap)| {
                let product =
                    i64::from(page_line_gap).checked_mul(i64::from(row_source_start_gap))?;
                accumulator.checked_add(product)
            },
        )?;
    let sum_y_squared =
        page_line_gaps
            .iter()
            .copied()
            .try_fold(0_i64, |accumulator, page_line_gap| {
                let page_line_gap = i64::from(page_line_gap);
                let squared = page_line_gap.checked_mul(page_line_gap)?;
                accumulator.checked_add(squared)
            })?;

    let slope_numerator = n
        .checked_mul(sum_xy)?
        .checked_sub(sum_y.checked_mul(sum_x)?)?;
    let slope_denominator = n
        .checked_mul(sum_y_squared)?
        .checked_sub(sum_y.checked_mul(sum_y)?)?;
    if slope_denominator == 0 {
        return None;
    }
    let intercept_numerator = sum_x
        .checked_mul(slope_denominator)?
        .checked_sub(slope_numerator.checked_mul(sum_y)?)?;
    let intercept_denominator = n.checked_mul(slope_denominator)?;
    let common_denominator = intercept_denominator.checked_abs()?;
    if common_denominator == 0 {
        return None;
    }

    let max_abs_residual_numerator = page_line_gaps
        .iter()
        .copied()
        .zip(row_source_start_gap_units.iter().copied())
        .try_fold(
            0_i64,
            |max_residual, (page_line_gap, row_source_start_gap)| {
                let predicted_numerator = slope_numerator
                    .checked_mul(i64::from(page_line_gap))?
                    .checked_mul(n)?
                    .checked_add(intercept_numerator)?;
                let observed_numerator =
                    i64::from(row_source_start_gap).checked_mul(intercept_denominator)?;
                let residual = observed_numerator
                    .checked_sub(predicted_numerator)?
                    .checked_abs()?;
                Some(max_residual.max(residual))
            },
        )?;

    if slope_numerator == i64::MIN
        || slope_denominator == i64::MIN
        || intercept_numerator == i64::MIN
        || intercept_denominator == i64::MIN
    {
        return None;
    }

    let (numerator_slope, denominator_slope) =
        reduce_i64_fraction(slope_numerator, slope_denominator);
    let (numerator_intercept, denominator_intercept) =
        reduce_i64_fraction(intercept_numerator, intercept_denominator);
    let max_abs_residual = ratio_i64_to_f64(max_abs_residual_numerator, common_denominator)?;
    let max_abs_residual_ceiling_units = i32::try_from(
        max_abs_residual_numerator.checked_add(common_denominator.checked_sub(1)?)?
            / common_denominator,
    )
    .ok()?;

    Some(TableGridAffineRowSourceStartGapFit {
        numerator_slope,
        denominator_slope,
        numerator_intercept,
        denominator_intercept,
        max_abs_residual,
        max_abs_residual_ceiling_units,
        sample_count,
        family_scoped,
        fit_stable: max_abs_residual_numerator <= common_denominator,
    })
}

pub(crate) fn reduce_i64_fraction(numerator: i64, denominator: i64) -> (i64, i64) {
    if denominator == 0 {
        return (numerator, denominator);
    }
    let mut numerator = numerator;
    let mut denominator = denominator;
    if denominator < 0 {
        numerator = -numerator;
        denominator = -denominator;
    }
    let divisor = gcd_i64(numerator, denominator);
    (numerator / divisor, denominator / divisor)
}

pub(crate) fn gcd_i64(left: i64, right: i64) -> i64 {
    let mut left = left.abs();
    let mut right = right.abs();
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

pub(crate) fn ratio_i64_to_f64(numerator: i64, denominator: i64) -> Option<f64> {
    if denominator == 0 {
        return None;
    }
    let numerator = i32::try_from(numerator).ok().map(f64::from)?;
    let denominator = i32::try_from(denominator).ok().map(f64::from)?;
    Some(numerator / denominator)
}

pub(crate) fn push_affine_row_source_start_gap_fit_json(
    output: &mut String,
    fit: Option<TableGridAffineRowSourceStartGapFit>,
) {
    let Some(fit) = fit else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"sourceOnlyPageYOriginDomainGate.transitionSemanticsReadiness.affineRowSourceStartGapFit\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"numeratorSlope\":");
    output.push_str(&fit.numerator_slope.to_string());
    output.push_str(",\"denominatorSlope\":");
    output.push_str(&fit.denominator_slope.to_string());
    output.push_str(",\"numeratorIntercept\":");
    output.push_str(&fit.numerator_intercept.to_string());
    output.push_str(",\"denominatorIntercept\":");
    output.push_str(&fit.denominator_intercept.to_string());
    output.push_str(",\"maxAbsResidual\":");
    output.push_str(&format!("{:.3}", fit.max_abs_residual));
    output.push_str(",\"sampleCount\":");
    output.push_str(&fit.sample_count.to_string());
    output.push_str(",\"familyScoped\":");
    output.push_str(if fit.family_scoped { "true" } else { "false" });
    output.push_str(",\"fitStable\":");
    output.push_str(if fit.fit_stable { "true" } else { "false" });
    output.push_str(",\"blockedReason\":");
    output.push_str(&json_string(fit.blocked_reason()));
    output.push_str(
        ",\"renderPromotionContribution\":\"affine-row-source-start-gap-fit-diagnostic-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(fit.blocked_reason()));
    output.push('}');
}

#[derive(Debug, Clone)]
pub(crate) struct TableGridSourceOnlyPageYOriginCandidateSupport {
    pub(crate) origin_basis: &'static str,
    pub(crate) selected_y: f32,
    pub(crate) row_height: Option<f32>,
    pub(crate) table_candidate_index: Option<usize>,
    pub(crate) contribution: &'static str,
    pub(crate) blocked_reason: &'static str,
    pub(crate) extra_blocked_reasons: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridSourceOnlyStrideRowCoverageSummary {
    pub(crate) candidate_row_count: usize,
    pub(crate) matched_row_count: usize,
    pub(crate) all_rows_covered: bool,
    pub(crate) line_mark_record_selection: &'static str,
    pub(crate) line_mark_record_indexes: Vec<usize>,
    pub(crate) uniform_line_mark_record_stride: bool,
    pub(crate) line_mark_record_stride: Option<usize>,
    pub(crate) matches_stride_candidate_record_indexes: bool,
    pub(crate) row_span_units: Vec<usize>,
    pub(crate) line_mark_span_units: Vec<usize>,
    pub(crate) row_span_residual_units: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridLineMarkRowBoundaryAlignmentSummary {
    pub(crate) candidate_row_count: usize,
    pub(crate) selected_spacing_record_alignment:
        Option<TableGridLineMarkRowBoundaryAlignmentFamily>,
    pub(crate) previous_row_span_record_alignment:
        Option<TableGridLineMarkRowBoundaryAlignmentFamily>,
    pub(crate) next_record_alignment: Option<TableGridLineMarkRowBoundaryAlignmentFamily>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridLineMarkRowBoundaryAlignmentFamily {
    pub(crate) family: &'static str,
    pub(crate) span_interpretation: &'static str,
    pub(crate) row_count: usize,
    pub(crate) record_indexes: Vec<usize>,
    pub(crate) uniform_line_mark_record_stride: bool,
    pub(crate) line_mark_record_stride: Option<usize>,
    pub(crate) matches_stride_candidate_record_indexes: bool,
    pub(crate) row_source_start_units: Vec<usize>,
    pub(crate) row_source_end_units: Vec<usize>,
    pub(crate) line_mark_start_units: Vec<usize>,
    pub(crate) line_mark_end_units: Vec<usize>,
    pub(crate) start_residual_units: Vec<i32>,
    pub(crate) end_residual_units: Vec<i32>,
    pub(crate) span_residual_units: Vec<i32>,
    pub(crate) exact_boundary_match_count: usize,
    pub(crate) exact_boundary_aligned: bool,
    pub(crate) start_residual_stable: bool,
    pub(crate) end_residual_stable: bool,
    pub(crate) span_residual_stable: bool,
    pub(crate) stable_start_residual_units: Option<i32>,
    pub(crate) stable_end_residual_units: Option<i32>,
    pub(crate) stable_span_residual_units: Option<i32>,
    pub(crate) row_boundary_offset_candidate_units: Option<i32>,
    pub(crate) offset_normalized_start_residual_units: Vec<i32>,
    pub(crate) offset_normalized_end_residual_units: Vec<i32>,
    pub(crate) offset_normalized_exact_boundary_match_count: usize,
    pub(crate) offset_normalized_exact_boundary_aligned: bool,
    pub(crate) span_only_match: bool,
    pub(crate) rows: Vec<TableGridLineMarkRowBoundaryAlignmentRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridLineMarkRowBoundaryAlignmentRow {
    pub(crate) compact_row_index: usize,
    pub(crate) sparse_row_index: usize,
    pub(crate) source_interval_index: usize,
    pub(crate) line_mark_record_index: usize,
    pub(crate) row_source_start_units: usize,
    pub(crate) row_source_end_units: usize,
    pub(crate) line_mark_start_units: usize,
    pub(crate) line_mark_end_units: usize,
    pub(crate) start_residual_units: i32,
    pub(crate) end_residual_units: i32,
    pub(crate) span_residual_units: i32,
    pub(crate) exact_boundary_aligned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridStridePageMarkEntryLineBoundsCoverageSummary {
    pub(crate) candidate_row_count: usize,
    pub(crate) line_mark_record_indexes: Vec<usize>,
    pub(crate) record_stride: usize,
    pub(crate) page_mark_entry_index: usize,
    pub(crate) page_index_candidate: Option<usize>,
    pub(crate) page_line_start: usize,
    pub(crate) page_line_end: usize,
    pub(crate) line_offsets_from_page_start: Vec<usize>,
    pub(crate) row_count_matches_stride_candidate: bool,
    pub(crate) all_line_mark_records_within_page_mark_entry: bool,
    pub(crate) coverage_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGridPageMarkSubrecordLineRangeRecordCoverageSummary {
    pub(crate) candidate_count: usize,
    pub(crate) selected_record_indexes: Vec<usize>,
    pub(crate) previous_record_indexes: Vec<usize>,
    pub(crate) selected_covered_record_indexes: Vec<usize>,
    pub(crate) previous_covered_record_indexes: Vec<usize>,
    pub(crate) selected_containing_candidate_byte_offsets: Vec<usize>,
    pub(crate) previous_containing_candidate_byte_offsets: Vec<usize>,
    pub(crate) selected_coverage_complete: bool,
    pub(crate) previous_coverage_complete: bool,
    pub(crate) selected_nearest_matches: Vec<TableGridPageMarkSubrecordLineRangeRecordMatch>,
    pub(crate) previous_nearest_matches: Vec<TableGridPageMarkSubrecordLineRangeRecordMatch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TableGridPageMarkSubrecordLineRangeRecordMatch {
    pub(crate) record_index: usize,
    pub(crate) distance_units: usize,
    pub(crate) candidate: PageMarkRawSubrecordLineSpanCandidate,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PageMarkU16LayoutComparison {
    pub(crate) page_width_px: f32,
    pub(crate) page_height_px: f32,
    pub(crate) page_margin_px: f32,
    pub(crate) page_body_width_px: f32,
}

pub(crate) fn table_source_offset_to_units(
    basis: TextCountRangeOverlapBasis,
    offset: usize,
) -> usize {
    match basis {
        TextCountRangeOverlapBasis::Byte => offset / 2,
        TextCountRangeOverlapBasis::Unit => offset,
    }
}

pub(crate) fn push_optional_usize_json(output: &mut String, value: Option<usize>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

pub(crate) fn push_optional_u16_json(output: &mut String, value: Option<u16>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

pub(crate) fn ranges_overlap_half_open(
    start: usize,
    end: usize,
    other_start: usize,
    other_end: usize,
) -> bool {
    start < other_end && other_start < end
}
