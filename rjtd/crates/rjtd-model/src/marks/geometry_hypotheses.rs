use super::*;
use crate::*;

pub(crate) fn table_candidate_direct_page_mark_line_hit_count(
    page_mark: Option<&DocumentPageMark>,
    candidate: &TableCandidate,
) -> usize {
    let Some(page_mark) = page_mark else {
        return 0;
    };
    page_mark
        .entries()
        .iter()
        .filter(|entry| {
            let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                return false;
            };
            let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                return false;
            };
            ranges_overlap_half_open(
                candidate.source_start(),
                candidate.source_end(),
                line_start,
                line_end.saturating_add(1),
            )
        })
        .count()
}

pub(crate) fn push_answer_sheet_section_line_mark_geometry_candidate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let intervals = shanai_lan_line_mark_intervals(document);
    let anchors = success_data_test_answer_sheet_section_anchors(candidate);
    output.push_str("{\"source\":\"sparseTableCandidateTopology+/LineMark+/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"tableCandidateIndex\":");
    output.push_str(&candidate.index().to_string());
    output.push_str(",\"sectionAnchorCount\":");
    output.push_str(&anchors.len().to_string());
    output.push_str(",\"lineMarkIntervalCount\":");
    output.push_str(&intervals.len().to_string());
    let matched_count = anchors
        .iter()
        .filter(|anchor| {
            best_line_mark_interval_for_unit_range(
                &intervals,
                table_source_offset_to_units(candidate.basis(), anchor.row_source_start),
                table_source_offset_to_units(candidate.basis(), anchor.row_source_end),
            )
            .is_some()
        })
        .count();
    output.push_str(",\"matchedSectionAnchorCount\":");
    output.push_str(&matched_count.to_string());
    output.push_str(",\"rows\":[");
    for (index, anchor) in anchors.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let row_unit_start =
            table_source_offset_to_units(candidate.basis(), anchor.row_source_start);
        let row_unit_end = table_source_offset_to_units(candidate.basis(), anchor.row_source_end);
        let line_mark =
            best_line_mark_interval_for_unit_range(&intervals, row_unit_start, row_unit_end);
        output.push_str("{\"sectionLabel\":");
        output.push_str(&json_string(&anchor.section_label));
        output.push_str(",\"rowIndex\":");
        output.push_str(&anchor.row_index.to_string());
        output.push_str(",\"sourceIntervalIndex\":");
        output.push_str(&anchor.source_interval_index.to_string());
        output.push_str(",\"rowSourceUnitRange\":");
        output.push_str(&source_range_json(row_unit_start, row_unit_end));
        output.push_str(",\"cellIndex\":");
        output.push_str(&anchor.cell_index.to_string());
        output.push_str(",\"lineMarkRecordIndex\":");
        match line_mark {
            Some(interval) => output.push_str(&interval.record_index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"lineMarkUnitRange\":");
        match line_mark {
            Some(interval) => {
                output.push_str(&source_range_json(interval.unit_start, interval.unit_end));
            }
            None => output.push_str("null"),
        }
        output.push_str(",\"sourceGridCandidate\":");
        match line_mark.and_then(|interval| {
            success_data_test_line_mark_page_grid_candidate(document, layout, interval.record_index)
        }) {
            Some(candidate) => push_success_data_test_line_mark_page_grid_candidate_json(
                output, document, layout, &candidate, None, None,
            ),
            None => output.push_str("null"),
        }
        output.push('}');
    }
    output.push_str("],\"renderPromotionContribution\":\"section-anchor-row-track-candidate-only\",\"renderPromotionBlockedReason\":\"sparse-section-row-to-merged-cell-geometry-semantics-unproven\"}");
}

pub(crate) fn best_line_mark_interval_for_unit_range(
    intervals: &[ShanaiLanLineMarkInterval],
    unit_start: usize,
    unit_end: usize,
) -> Option<ShanaiLanLineMarkInterval> {
    intervals
        .iter()
        .copied()
        .filter_map(|interval| {
            line_mark_interval_match_key(interval, unit_start, unit_end).map(|key| (interval, key))
        })
        .max_by_key(|(_, key)| *key)
        .map(|(interval, _)| interval)
}

pub(crate) fn line_mark_interval_match_key(
    interval: ShanaiLanLineMarkInterval,
    unit_start: usize,
    unit_end: usize,
) -> Option<(bool, bool, usize, usize, usize)> {
    if interval.unit_start >= unit_end || unit_start >= interval.unit_end {
        return None;
    }
    let exact_match = interval.unit_start == unit_start && interval.unit_end == unit_end;
    let contains_source = interval.unit_start <= unit_start && unit_end <= interval.unit_end;
    let overlap_start = interval.unit_start.max(unit_start);
    let overlap_end = interval.unit_end.min(unit_end);
    let overlap_len = overlap_end.saturating_sub(overlap_start);
    let interval_len = interval.unit_end.saturating_sub(interval.unit_start);
    Some((
        exact_match,
        contains_source,
        overlap_len,
        usize::MAX.saturating_sub(interval_len),
        usize::MAX.saturating_sub(interval.record_index),
    ))
}

pub(crate) fn push_page_mark_selected_fields_from_parts_json(
    output: &mut String,
    entry_index: Option<usize>,
    line_start: Option<u32>,
    line_end: Option<u32>,
    u16_fields: &[u16],
) {
    let line_count = line_start
        .zip(line_end)
        .map(|(start, end)| end.saturating_sub(start).saturating_add(1));
    let line_gap_count = line_start
        .zip(line_end)
        .map(|(start, end)| end.saturating_sub(start));
    output.push_str("{\"source\":\"/PageMark\",\"entryIndex\":");
    push_optional_usize_json(output, entry_index);
    output.push_str(",\"lineStart\":");
    push_option_u32_json(output, line_start);
    output.push_str(",\"lineEnd\":");
    push_option_u32_json(output, line_end);
    output.push_str(",\"lineCount\":");
    push_option_u32_json(output, line_count);
    output.push_str(",\"lineGapCount\":");
    push_option_u32_json(output, line_gap_count);
    output.push_str(",\"u16GeometryClass\":");
    output.push_str(&json_string(
        page_mark_u16_geometry_profile(u16_fields).class_name(),
    ));
    output.push_str(",\"selectedFields\":[");
    for (index, word_index) in PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES
        .iter()
        .enumerate()
    {
        if index > 0 {
            output.push(',');
        }
        let value = u16_fields.get(*word_index).copied();
        output.push_str("{\"wordIndex\":");
        output.push_str(&word_index.to_string());
        output.push_str(",\"value\":");
        push_optional_u16_json(output, value);
        output.push_str(",\"hex\":");
        push_option_u16_hex_json(output, value);
        output.push_str(",\"perLineCount\":");
        push_optional_field_ratio_json(output, value, line_count);
        output.push_str(",\"perLineGapCount\":");
        push_optional_field_ratio_json(output, value, line_gap_count);
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn line_mark_be_delta_record_byte_offset(record_index: usize) -> usize {
    LINE_MARK_BE_DELTA_HEADER_BYTES + record_index * LINE_MARK_BE_DELTA_RECORD_BYTES
}

pub(crate) fn line_mark_be_delta_record_word_index(record_index: usize) -> usize {
    line_mark_be_delta_record_byte_offset(record_index) / 2
}
