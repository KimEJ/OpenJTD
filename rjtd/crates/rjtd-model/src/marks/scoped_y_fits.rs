use super::*;
use crate::*;

pub(crate) fn page_mark_scoped_y_target_hit_counts(
    targets: &[f32],
    candidates: &[PageMarkScopedYValueCandidate],
    tolerance_px: f32,
) -> (usize, usize) {
    let mut target_hit_count = 0usize;
    let mut total_hit_count = 0usize;
    for target in targets {
        let hit_count = candidates
            .iter()
            .filter(|candidate| (candidate.value_px - *target).abs() <= tolerance_px)
            .count();
        if hit_count > 0 {
            target_hit_count += 1;
            total_hit_count += hit_count;
        }
    }
    (target_hit_count, total_hit_count)
}

pub(crate) fn page_mark_scoped_delta_target_hit_counts(
    targets: &[f32],
    candidates: &[PageMarkScopedYValueCandidate],
    tolerance_px: f32,
) -> (usize, usize) {
    let delta_candidates = page_mark_scoped_y_pairwise_delta_candidates(candidates);
    let mut target_hit_count = 0usize;
    let mut total_hit_count = 0usize;
    for target in targets {
        let hit_count = delta_candidates
            .iter()
            .filter(|candidate| (candidate.delta_px - *target).abs() <= tolerance_px)
            .count();
        if hit_count > 0 {
            target_hit_count += 1;
            total_hit_count += hit_count;
        }
    }
    (target_hit_count, total_hit_count)
}

pub(crate) fn page_mark_raw_header_indexes_for_line_mark_record_indexes(
    record_headers: &[PageMarkRecordHeader],
    line_mark_record_indexes: &[usize],
) -> Vec<usize> {
    line_mark_record_indexes
        .iter()
        .filter_map(|record_index| {
            record_headers.iter().position(|header| {
                header.line_start as usize <= *record_index
                    && *record_index <= header.line_end as usize
            })
        })
        .collect()
}

pub(crate) fn page_mark_subrecord_nearest_line_span_matches<'a>(
    targets: &[usize],
    candidates: &'a [PageMarkRawSubrecordLineSpanCandidate],
) -> Vec<PageMarkSubrecordLineSpanMatch<'a>> {
    targets
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(target_index, target_units)| {
            candidates
                .iter()
                .map(|candidate| PageMarkSubrecordLineSpanMatch {
                    target_index,
                    target_units,
                    residual_units: candidate.line_span_units as i32 - target_units as i32,
                    candidate,
                })
                .min_by(|left, right| {
                    left.residual_units
                        .abs()
                        .cmp(&right.residual_units.abs())
                        .then_with(|| {
                            left.candidate
                                .raw_record_scan_index
                                .cmp(&right.candidate.raw_record_scan_index)
                        })
                        .then_with(|| left.candidate.byte_offset.cmp(&right.candidate.byte_offset))
                })
        })
        .collect()
}

pub(crate) fn page_mark_raw_subrecord_line_span_candidates(
    bytes: &[u8],
    record_headers: &[PageMarkRecordHeader],
    max_line_end: Option<u32>,
) -> Vec<PageMarkRawSubrecordLineSpanCandidate> {
    let mut candidates = Vec::new();
    for (scan_index, header) in record_headers.iter().copied().enumerate() {
        let next_offset = record_headers
            .get(scan_index + 1)
            .map(|next| next.offset)
            .unwrap_or(bytes.len());
        let tail_start = header.offset.saturating_add(16);
        if tail_start >= next_offset || next_offset > bytes.len() {
            continue;
        }
        for byte_offset in (tail_start..next_offset.saturating_sub(15)).step_by(2) {
            let Some(subrecord) = page_mark_raw_u16_subrecord_candidate_at(bytes, byte_offset)
            else {
                continue;
            };
            let line_start = subrecord.words[4];
            let line_end = subrecord.words[6];
            if max_line_end.is_none_or(|max_line_end| u32::from(line_end) > max_line_end) {
                continue;
            }
            let record_relative_byte_offset = subrecord.byte_offset.saturating_sub(header.offset);
            let tail_block16_word_index = record_relative_byte_offset
                .checked_sub(16)
                .map(|offset| (offset / 2) % 16);
            candidates.push(PageMarkRawSubrecordLineSpanCandidate {
                byte_offset: subrecord.byte_offset,
                raw_record_index: header.index,
                raw_record_scan_index: scan_index,
                tail_block16_word_index,
                line_start_candidate: line_start,
                line_end_candidate: line_end,
                line_span_units: usize::from(line_end.saturating_sub(line_start)),
                field2_value: subrecord.words[2],
                words: subrecord.words,
            });
        }
    }
    candidates.sort_by(|left, right| {
        left.raw_record_scan_index
            .cmp(&right.raw_record_scan_index)
            .then_with(|| left.byte_offset.cmp(&right.byte_offset))
    });
    candidates
}

pub(crate) fn push_page_mark_subrecord_line_span_matches_json(
    output: &mut String,
    matches: &[PageMarkSubrecordLineSpanMatch<'_>],
) {
    output.push('[');
    for (index, match_) in matches.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"targetIndex\":");
        output.push_str(&match_.target_index.to_string());
        output.push_str(",\"targetUnits\":");
        output.push_str(&match_.target_units.to_string());
        output.push_str(",\"residualUnits\":");
        output.push_str(&match_.residual_units.to_string());
        output.push_str(",\"candidate\":");
        push_page_mark_raw_subrecord_line_span_candidate_json(output, match_.candidate);
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_page_mark_raw_subrecord_line_span_candidate_json(
    output: &mut String,
    candidate: &PageMarkRawSubrecordLineSpanCandidate,
) {
    output.push_str("{\"byteOffset\":");
    output.push_str(&candidate.byte_offset.to_string());
    output.push_str(",\"rawRecordIndex\":");
    output.push_str(&candidate.raw_record_index.to_string());
    output.push_str(",\"rawRecordScanIndex\":");
    output.push_str(&candidate.raw_record_scan_index.to_string());
    output.push_str(",\"tailBlock16WordIndex\":");
    push_option_usize_json(output, candidate.tail_block16_word_index);
    output.push_str(",\"lineStartCandidate\":");
    output.push_str(&candidate.line_start_candidate.to_string());
    output.push_str(",\"lineEndCandidate\":");
    output.push_str(&candidate.line_end_candidate.to_string());
    output.push_str(",\"lineSpanUnits\":");
    output.push_str(&candidate.line_span_units.to_string());
    output.push_str(",\"field2Value\":");
    output.push_str(&candidate.field2_value.to_string());
    output.push_str(",\"field2DirectPx\":");
    output.push_str(&format!("{:.3}", f32::from(candidate.field2_value)));
    output.push_str(",\"words\":");
    push_u16_array_json(output, &candidate.words);
    output.push_str(",\"wordsHex\":");
    push_u16_hex_array_json(output, &candidate.words);
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_page_mark_scoped_y_record_set_probe_json(
    output: &mut String,
    record_set: &str,
    layout: PageLayout,
    page_mark: &DocumentPageMark,
    page_mark_bytes: &[u8],
    record_headers: &[PageMarkRecordHeader],
    line_mark_record_indexes: &[usize],
    row_top_targets: &[f32],
    row_delta_targets: &[f32],
    tolerance_px: f32,
) {
    if line_mark_record_indexes.len() != row_top_targets.len()
        || line_mark_record_indexes.is_empty()
    {
        output.push_str("null");
        return;
    }

    let mut parsed_entry_indexes = Vec::new();
    let mut raw_header_indexes = Vec::new();
    for record_index in line_mark_record_indexes {
        if let Some(entry_index) = page_mark.entries().iter().position(|entry| {
            let Some(line_start) = entry.line_start().map(|value| value as usize) else {
                return false;
            };
            let Some(line_end) = entry.line_end().map(|value| value as usize) else {
                return false;
            };
            line_start <= *record_index && *record_index <= line_end
        }) {
            parsed_entry_indexes.push(entry_index);
        }
        if let Some(header_index) = record_headers.iter().position(|header| {
            header.line_start as usize <= *record_index && *record_index <= header.line_end as usize
        }) {
            raw_header_indexes.push(header_index);
        }
    }

    let single_parsed_entry_index = single_usize_value(&parsed_entry_indexes);
    let single_raw_header_index = single_usize_value(&raw_header_indexes);
    let mut value_candidates = Vec::new();
    if let Some(entry_index) = single_parsed_entry_index
        && let Some(entry) = page_mark.entries().get(entry_index)
    {
        collect_page_mark_entry_y_value_candidates(&mut value_candidates, entry);
    }
    if let Some(header_index) = single_raw_header_index
        && let Some(header) = record_headers.get(header_index)
    {
        let next_offset = record_headers
            .get(header_index + 1)
            .map(|next| next.offset)
            .unwrap_or(page_mark_bytes.len());
        collect_page_mark_raw_header_y_value_candidates(
            &mut value_candidates,
            page_mark_bytes,
            *header,
            next_offset,
        );
    }

    output.push_str(
        "{\"source\":\"/PageMark scoped raw fields+alternateLineMarkRecordSet+referenceTableBBox\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"recordSet\":");
    output.push_str(&json_string(record_set));
    output.push_str(",\"tolerancePx\":");
    output.push_str(&format!("{tolerance_px:.3}"));
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, line_mark_record_indexes);
    output.push_str(",\"parsedEntryMatchCount\":");
    output.push_str(&parsed_entry_indexes.len().to_string());
    output.push_str(",\"singleParsedPageMarkEntryMatched\":");
    output.push_str(if single_parsed_entry_index.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedParsedPageMarkEntryIndex\":");
    push_option_usize_json(output, single_parsed_entry_index);
    output.push_str(",\"rawHeaderMatchCount\":");
    output.push_str(&raw_header_indexes.len().to_string());
    output.push_str(",\"singleRawRecordHeaderMatched\":");
    output.push_str(if single_raw_header_index.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matchedRawRecordHeaderIndex\":");
    push_option_usize_json(output, single_raw_header_index);
    output.push_str(",\"valueCandidateCount\":");
    output.push_str(&value_candidates.len().to_string());
    output.push_str(",\"rowTopNearestCandidates\":");
    push_page_mark_scoped_nearest_y_candidates_json(output, row_top_targets, &value_candidates);
    output.push_str(",\"rowTopHitSummary\":");
    push_page_mark_scoped_y_hit_summary_json(
        output,
        row_top_targets,
        &value_candidates,
        tolerance_px,
    );
    output.push_str(",\"sharedFieldFamilyResiduals\":");
    push_page_mark_scoped_y_shared_field_family_residuals_json(
        output,
        page_mark.family(),
        page_mark,
        page_mark_bytes,
        record_headers,
        single_parsed_entry_index,
        line_mark_record_indexes,
        row_top_targets,
        row_delta_targets,
        tolerance_px,
    );
    output.push_str(",\"slotScopedSubrecordYSequenceComparison\":");
    push_page_mark_slot_scoped_subrecord_y_sequence_comparison_json(
        output,
        layout,
        page_mark,
        page_mark_bytes,
        record_headers,
        single_parsed_entry_index,
        single_raw_header_index,
        line_mark_record_indexes,
        row_top_targets,
        row_delta_targets,
        tolerance_px,
    );
    output
        .push_str(",\"renderPromotionContribution\":\"alternate-record-set-page-mark-probe-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "alternate-line-mark-record-set-page-mark-semantics-unproven",
    ));
    output.push('}');
}

pub(crate) fn collect_page_mark_entry_y_value_candidates(
    output: &mut Vec<PageMarkScopedYValueCandidate>,
    entry: &DocumentPageMarkEntry,
) {
    for (word_index, value) in entry.u16_fields().iter().copied().enumerate() {
        push_page_mark_scoped_y_value_candidate(
            output,
            "parsedEntryU16",
            "direct-u16-px",
            Some(word_index),
            Some(word_index * 2),
            u32::from(value),
            f32::from(value),
        );
        push_page_mark_scoped_y_value_candidate(
            output,
            "parsedEntryU16",
            "centipoint-to-css-px",
            Some(word_index),
            Some(word_index * 2),
            u32::from(value),
            page_mark_centipoints_to_css_px(u32::from(value)),
        );
    }
    for (word_index, value) in entry.u32_fields().iter().copied().enumerate() {
        if value <= 10_000 {
            push_page_mark_scoped_y_value_candidate(
                output,
                "parsedEntryU32",
                "direct-u32-px",
                Some(word_index),
                Some(word_index * 4),
                value,
                value as f32,
            );
        }
    }
    for subrecord in page_mark_u16_subrecord_candidates(entry.u16_fields()) {
        let words = subrecord.words();
        for (field_index, value) in words.iter().copied().enumerate() {
            let word_index = subrecord.word_index() + field_index;
            push_page_mark_scoped_y_value_candidate(
                output,
                "parsedEntryU16Subrecord",
                "direct-u16-px",
                Some(word_index),
                Some(word_index * 2),
                u32::from(value),
                f32::from(value),
            );
            push_page_mark_scoped_y_value_candidate(
                output,
                "parsedEntryU16Subrecord",
                "centipoint-to-css-px",
                Some(word_index),
                Some(word_index * 2),
                u32::from(value),
                page_mark_centipoints_to_css_px(u32::from(value)),
            );
        }
    }
}

pub(crate) fn collect_page_mark_raw_header_y_value_candidates(
    output: &mut Vec<PageMarkScopedYValueCandidate>,
    bytes: &[u8],
    header: PageMarkRecordHeader,
    next_offset: usize,
) {
    let tail_start = header.offset.saturating_add(16);
    if tail_start >= next_offset || next_offset > bytes.len() {
        return;
    }
    let tail = &bytes[tail_start..next_offset];
    for (word_index, chunk) in tail.chunks_exact(2).enumerate() {
        let value = u16::from_be_bytes([chunk[0], chunk[1]]);
        let stream_word_index = (tail_start / 2).saturating_add(word_index);
        let stream_byte_offset = tail_start.saturating_add(word_index * 2);
        push_page_mark_scoped_y_value_candidate(
            output,
            "rawRecordHeaderTailU16",
            "direct-u16-px",
            Some(stream_word_index),
            Some(stream_byte_offset),
            u32::from(value),
            f32::from(value),
        );
        push_page_mark_scoped_y_value_candidate(
            output,
            "rawRecordHeaderTailU16",
            "centipoint-to-css-px",
            Some(stream_word_index),
            Some(stream_byte_offset),
            u32::from(value),
            page_mark_centipoints_to_css_px(u32::from(value)),
        );
    }
    for (word_index, chunk) in tail.chunks_exact(4).enumerate() {
        let value = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        if value <= 10_000 {
            push_page_mark_scoped_y_value_candidate(
                output,
                "rawRecordHeaderTailU32",
                "direct-u32-px",
                Some((tail_start / 4).saturating_add(word_index)),
                Some(tail_start.saturating_add(word_index * 4)),
                value,
                value as f32,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_page_mark_scoped_y_shared_field_family_residuals_json(
    output: &mut String,
    parsed_page_mark_family: &str,
    page_mark: &DocumentPageMark,
    bytes: &[u8],
    record_headers: &[PageMarkRecordHeader],
    parsed_entry_index: Option<usize>,
    line_mark_record_indexes: &[usize],
    row_top_targets: &[f32],
    row_delta_targets: &[f32],
    tolerance_px: f32,
) {
    let mut members = Vec::new();
    collect_page_mark_scoped_y_family_members(
        &mut members,
        page_mark,
        parsed_entry_index,
        bytes,
        record_headers,
    );
    let mut fits = page_mark_scoped_y_family_fits(
        members,
        line_mark_record_indexes,
        row_top_targets,
        row_delta_targets,
        tolerance_px,
    );
    fits.sort_by(page_mark_scoped_y_family_fit_ordering);

    output.push_str("{\"source\":\"/PageMark scoped field families+/LineMark+referenceTableBBox\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"parsedPageMarkFamily\":");
    output.push_str(&json_string(parsed_page_mark_family));
    output.push_str(",\"familyKind\":\"u16-subrecord-field\"");
    output.push_str(",\"familyCount\":");
    output.push_str(&fits.len().to_string());
    output.push_str(",\"bestTableTopFamily\":");
    match fits.iter().find(|fit| fit.table_top_hit_count > 0) {
        Some(fit) => push_page_mark_scoped_y_family_fit_json(output, fit),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestRowTopFamily\":");
    match fits.iter().find(|fit| fit.row_top_coverage_count > 0) {
        Some(fit) => push_page_mark_scoped_y_family_fit_json(output, fit),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestRowDeltaFamily\":");
    match fits.iter().find(|fit| fit.row_delta_coverage_count > 0) {
        Some(fit) => push_page_mark_scoped_y_family_fit_json(output, fit),
        None => output.push_str("null"),
    }
    output.push_str(",\"families\":[");
    for (index, fit) in fits.iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_family_fit_json(output, fit);
    }
    output.push_str("],\"renderPromotionContribution\":\"field-family-residual-diagnostic-only\",\"renderPromotionBlockedReason\":\"page-mark-subrecord-field-family-semantics-unproven\"}");
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_page_mark_slot_scoped_subrecord_y_sequence_comparison_json(
    output: &mut String,
    layout: PageLayout,
    page_mark: &DocumentPageMark,
    bytes: &[u8],
    record_headers: &[PageMarkRecordHeader],
    parsed_entry_index: Option<usize>,
    matched_raw_header_index: Option<usize>,
    line_mark_record_indexes: &[usize],
    row_top_targets: &[f32],
    row_delta_targets: &[f32],
    tolerance_px: f32,
) {
    let mut members = Vec::new();
    collect_page_mark_scoped_y_family_members(
        &mut members,
        page_mark,
        parsed_entry_index,
        bytes,
        record_headers,
    );
    let subrecord_line_range_max =
        page_mark_subrecord_line_range_max_candidate(page_mark, record_headers);
    let same_header_members = matched_raw_header_index
        .map(|matched_raw_header_index| {
            members
                .iter()
                .filter(|member| member.raw_record_scan_index == Some(matched_raw_header_index))
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let foreign_header_members = matched_raw_header_index
        .map(|matched_raw_header_index| {
            members
                .iter()
                .filter(|member| {
                    member
                        .raw_record_scan_index
                        .is_some_and(|scan_index| scan_index != matched_raw_header_index)
                })
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut slots = page_mark_scoped_y_slot_fits(
        members,
        line_mark_record_indexes,
        row_top_targets,
        row_delta_targets,
        tolerance_px,
    );
    slots.sort_by(page_mark_scoped_y_slot_fit_ordering);
    let mut same_header_slots = page_mark_scoped_y_slot_fits(
        same_header_members,
        line_mark_record_indexes,
        row_top_targets,
        row_delta_targets,
        tolerance_px,
    );
    same_header_slots.sort_by(page_mark_scoped_y_slot_fit_ordering);
    let mut foreign_header_slots = page_mark_scoped_y_slot_fits(
        foreign_header_members,
        line_mark_record_indexes,
        row_top_targets,
        row_delta_targets,
        tolerance_px,
    );
    foreign_header_slots.sort_by(page_mark_scoped_y_slot_fit_ordering);

    output.push_str(
        "{\"source\":\"/PageMark raw u16 subrecord scan+/LineMark+/PageMark+referenceTableBBox\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"grouping\":\"fieldIndex+tailBlock16WordIndex\"");
    output.push_str(",\"matchedRawRecordHeaderIndex\":");
    push_option_usize_json(output, matched_raw_header_index);
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, line_mark_record_indexes);
    output.push_str(",\"referenceRowTops\":");
    push_f32_array_json(output, row_top_targets);
    output.push_str(",\"referenceRowDeltas\":");
    push_f32_array_json(output, row_delta_targets);
    output.push_str(
        ",\"subrecordLineRangeCandidatePolicy\":\"words4To6WithinScannedRecordHeaderMaxLineEnd\"",
    );
    output.push_str(",\"subrecordLineRangeMaxCandidate\":");
    push_option_u32_json(output, subrecord_line_range_max);
    output.push_str(",\"pageScaleCandidates\":");
    push_page_mark_slot_scoped_page_scale_candidates_json(
        output,
        layout,
        parsed_entry_index.and_then(|index| page_mark.entries().get(index)),
    );
    output.push_str(",\"slotCount\":");
    output.push_str(&slots.len().to_string());
    output.push_str(",\"sameHeaderSlotCount\":");
    output.push_str(&same_header_slots.len().to_string());
    output.push_str(",\"sameHeaderBestTableTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &same_header_slots, |slot| {
        slot.table_top_hit_count > 0
    });
    output.push_str(",\"sameHeaderBestRowTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &same_header_slots, |slot| {
        slot.row_top_coverage_count > 0
    });
    output.push_str(",\"sameHeaderBestRowDeltaSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &same_header_slots, |slot| {
        slot.row_delta_coverage_count > 0
    });
    output.push_str(",\"foreignHeaderSlotCount\":");
    output.push_str(&foreign_header_slots.len().to_string());
    output.push_str(",\"foreignHeaderBestTableTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &foreign_header_slots, |slot| {
        slot.table_top_hit_count > 0
    });
    output.push_str(",\"bestTableTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &slots, |slot| slot.table_top_hit_count > 0);
    output.push_str(",\"bestRowTopSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &slots, |slot| {
        slot.row_top_coverage_count > 0
    });
    output.push_str(",\"bestRowDeltaSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &slots, |slot| {
        slot.row_delta_coverage_count > 0
    });
    output.push_str(",\"orderedLineMarkRecordCoveragePolicy\":");
    output.push_str(&json_string(
        "one-ordered-subrecord-member-per-line-mark-record",
    ));
    output.push_str(",\"bestOrderedLineMarkRecordCoverageSlot\":");
    push_page_mark_scoped_y_best_slot_fit_json(output, &slots, |slot| {
        slot.ordered_line_mark_record_coverage_count > 0
    });
    output.push_str(",\"sameHeaderSlots\":[");
    for (index, slot) in same_header_slots.iter().take(8).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_slot_fit_json(output, slot);
    }
    output.push(']');
    output.push_str(",\"slots\":[");
    for (index, slot) in slots.iter().take(12).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_slot_fit_json(output, slot);
    }
    output.push_str("],\"renderPromotionContribution\":\"slot-scoped-subrecord-sequence-diagnostic-only\",\"renderPromotionBlockedReason\":\"page-mark-subrecord-slot-y-semantics-unproven\"}");
}

pub(crate) fn push_page_mark_scoped_y_best_slot_fit_json(
    output: &mut String,
    slots: &[PageMarkScopedYSlotFit],
    predicate: impl Fn(&PageMarkScopedYSlotFit) -> bool,
) {
    match slots.iter().find(|slot| predicate(slot)) {
        Some(slot) => push_page_mark_scoped_y_slot_fit_json(output, slot),
        None => output.push_str("null"),
    }
}

pub(crate) fn push_page_mark_slot_scoped_page_scale_candidates_json(
    output: &mut String,
    layout: PageLayout,
    entry: Option<&DocumentPageMarkEntry>,
) {
    let field = |index: usize| entry.and_then(|entry| entry.u16_fields().get(index).copied());
    let word_13_plus_14 = field(13)
        .zip(field(14))
        .and_then(|(left, right)| left.checked_add(right));
    output.push_str("{\"source\":\"/PageMark selected fields+layout\"");
    output.push_str(",\"pageWidthPx\":");
    output.push_str(&format!("{:.3}", layout.width_px()));
    output.push_str(",\"pageHeightPx\":");
    output.push_str(&format!("{:.3}", layout.height_px()));
    output.push_str(",\"pageHeightPxPerWord21Unit\":");
    push_optional_f32_json(
        output,
        field(21)
            .filter(|value| *value > 0)
            .map(|value| layout.height_px() / f32::from(value)),
    );
    output.push_str(",\"pageHeightPxPerWord13Plus14Unit\":");
    push_optional_f32_json(
        output,
        word_13_plus_14
            .filter(|value| *value > 0)
            .map(|value| layout.height_px() / f32::from(value)),
    );
    output.push_str(",\"word21\":");
    push_optional_u16_json(output, field(21));
    output.push_str(",\"word13Plus14\":");
    push_optional_u16_json(output, word_13_plus_14);
    output.push('}');
}

pub(crate) fn page_mark_scoped_y_slot_fits(
    members: Vec<PageMarkScopedYFamilyMember>,
    line_mark_record_indexes: &[usize],
    row_top_targets: &[f32],
    row_delta_targets: &[f32],
    tolerance_px: f32,
) -> Vec<PageMarkScopedYSlotFit> {
    let mut grouped: BTreeMap<(&'static str, &'static str, usize, usize), Vec<_>> = BTreeMap::new();
    for member in members {
        if member.source != "rawRecordHeaderTailU16Subrecord" {
            continue;
        }
        let Some(tail_block16_word_index) = member.tail_block16_word_index else {
            continue;
        };
        grouped
            .entry((
                member.source,
                member.interpretation,
                member.field_index,
                tail_block16_word_index,
            ))
            .or_default()
            .push(member);
    }

    grouped
        .into_iter()
        .filter_map(
            |((source, interpretation, field_index, tail_block16_word_index), mut members)| {
                members.sort_by(|left, right| {
                    left.raw_record_scan_index
                        .cmp(&right.raw_record_scan_index)
                        .then_with(|| left.byte_offset.cmp(&right.byte_offset))
                        .then_with(|| left.value.cmp(&right.value))
                });
                let table_top_target = row_top_targets.first().copied()?;
                let table_top_residuals = members
                    .iter()
                    .map(|member| member.value_px - table_top_target)
                    .filter(|residual| residual.abs() <= tolerance_px)
                    .collect::<Vec<_>>();
                let table_top_hit_count = table_top_residuals.len();
                let row_top_residuals =
                    page_mark_scoped_y_family_nearest_residuals(row_top_targets, &members);
                let row_delta_residuals =
                    page_mark_scoped_y_family_nearest_delta_residuals(row_delta_targets, &members);
                let row_top_coverage_count = row_top_residuals
                    .iter()
                    .filter(|residual| residual.abs() <= tolerance_px)
                    .count();
                let row_delta_coverage_count = row_delta_residuals
                    .iter()
                    .filter(|residual| residual.abs() <= tolerance_px)
                    .count();
                let row_line_range_coverage_count =
                    page_mark_scoped_y_family_line_range_coverage_count(
                        line_mark_record_indexes,
                        &members,
                    );
                let ordered_line_range_coverage = page_mark_scoped_y_ordered_line_range_coverage(
                    line_mark_record_indexes,
                    &members,
                );
                let ordered_line_mark_record_coverage_count =
                    ordered_line_range_coverage.record_indexes_covered.len();
                let ordered_line_mark_record_coverage_complete = !line_mark_record_indexes
                    .is_empty()
                    && line_mark_record_indexes
                        .windows(2)
                        .all(|pair| pair[0] < pair[1])
                    && ordered_line_mark_record_coverage_count == line_mark_record_indexes.len();
                let (row_top_mean_abs_residual, row_top_max_abs_residual) =
                    mean_and_max_abs_residual(&row_top_residuals);
                let (row_delta_mean_abs_residual, row_delta_max_abs_residual) =
                    mean_and_max_abs_residual(&row_delta_residuals);
                Some(PageMarkScopedYSlotFit {
                    source,
                    interpretation,
                    field_index,
                    tail_block16_word_index,
                    members,
                    table_top_residuals,
                    row_top_residuals,
                    row_delta_residuals,
                    table_top_hit_count,
                    row_top_coverage_count,
                    row_delta_coverage_count,
                    row_line_range_coverage_count,
                    ordered_line_mark_record_coverage_count,
                    ordered_line_mark_record_coverage_complete,
                    ordered_line_mark_record_indexes_covered: ordered_line_range_coverage
                        .record_indexes_covered,
                    ordered_line_mark_record_member_byte_offsets: ordered_line_range_coverage
                        .member_byte_offsets,
                    row_top_mean_abs_residual,
                    row_top_max_abs_residual,
                    row_delta_mean_abs_residual,
                    row_delta_max_abs_residual,
                })
            },
        )
        .collect()
}

pub(crate) fn page_mark_scoped_y_slot_fit_ordering(
    left: &PageMarkScopedYSlotFit,
    right: &PageMarkScopedYSlotFit,
) -> Ordering {
    right
        .ordered_line_mark_record_coverage_count
        .cmp(&left.ordered_line_mark_record_coverage_count)
        .then_with(|| {
            right
                .ordered_line_mark_record_coverage_complete
                .cmp(&left.ordered_line_mark_record_coverage_complete)
        })
        .then_with(|| {
            right
                .row_line_range_coverage_count
                .cmp(&left.row_line_range_coverage_count)
        })
        .then_with(|| right.table_top_hit_count.cmp(&left.table_top_hit_count))
        .then_with(|| {
            right
                .row_top_coverage_count
                .cmp(&left.row_top_coverage_count)
        })
        .then_with(|| {
            right
                .row_delta_coverage_count
                .cmp(&left.row_delta_coverage_count)
        })
        .then_with(|| {
            option_f32_order(
                left.row_top_max_abs_residual,
                right.row_top_max_abs_residual,
            )
        })
        .then_with(|| right.members.len().cmp(&left.members.len()))
        .then_with(|| left.source.cmp(right.source))
        .then_with(|| left.interpretation.cmp(right.interpretation))
        .then_with(|| left.field_index.cmp(&right.field_index))
        .then_with(|| {
            left.tail_block16_word_index
                .cmp(&right.tail_block16_word_index)
        })
}

pub(crate) fn page_mark_scoped_y_ordered_line_range_coverage(
    line_mark_record_indexes: &[usize],
    members: &[PageMarkScopedYFamilyMember],
) -> PageMarkScopedYOrderedLineRangeCoverage {
    if line_mark_record_indexes.is_empty() || members.is_empty() {
        return PageMarkScopedYOrderedLineRangeCoverage {
            record_indexes_covered: Vec::new(),
            member_byte_offsets: Vec::new(),
        };
    }

    let row_count = line_mark_record_indexes.len();
    let member_count = members.len();
    let mut best = vec![vec![0usize; member_count + 1]; row_count + 1];

    for row_index in 1..=row_count {
        for member_index in 1..=member_count {
            let mut value =
                best[row_index - 1][member_index].max(best[row_index][member_index - 1]);
            if page_mark_scoped_y_member_covers_line_mark_record(
                &members[member_index - 1],
                line_mark_record_indexes[row_index - 1],
            ) {
                value = value.max(best[row_index - 1][member_index - 1] + 1);
            }
            best[row_index][member_index] = value;
        }
    }

    let mut matches = Vec::new();
    let mut row_index = row_count;
    let mut member_index = member_count;
    while row_index > 0 && member_index > 0 {
        if page_mark_scoped_y_member_covers_line_mark_record(
            &members[member_index - 1],
            line_mark_record_indexes[row_index - 1],
        ) && best[row_index][member_index] == best[row_index - 1][member_index - 1] + 1
        {
            matches.push((row_index - 1, member_index - 1));
            row_index -= 1;
            member_index -= 1;
        } else if best[row_index - 1][member_index] >= best[row_index][member_index - 1] {
            row_index -= 1;
        } else {
            member_index -= 1;
        }
    }
    matches.reverse();

    let mut record_indexes_covered = Vec::new();
    let mut member_byte_offsets = Vec::new();
    for (row_index, member_index) in matches {
        record_indexes_covered.push(line_mark_record_indexes[row_index]);
        if let Some(byte_offset) = members[member_index].byte_offset {
            member_byte_offsets.push(byte_offset);
        }
    }

    PageMarkScopedYOrderedLineRangeCoverage {
        record_indexes_covered,
        member_byte_offsets,
    }
}

pub(crate) fn page_mark_scoped_y_member_covers_line_mark_record(
    member: &PageMarkScopedYFamilyMember,
    record_index: usize,
) -> bool {
    if record_index > u32::MAX as usize {
        return false;
    }
    let record_index = record_index as u32;
    member
        .subrecord_line_start_candidate
        .zip(member.subrecord_line_end_candidate)
        .is_some_and(|(start, end)| start <= record_index && record_index <= end)
}

pub(crate) fn collect_page_mark_scoped_y_family_members(
    output: &mut Vec<PageMarkScopedYFamilyMember>,
    page_mark: &DocumentPageMark,
    parsed_entry_index: Option<usize>,
    bytes: &[u8],
    record_headers: &[PageMarkRecordHeader],
) {
    let subrecord_line_range_max =
        page_mark_subrecord_line_range_max_candidate(page_mark, record_headers);
    if let Some(entry_index) = parsed_entry_index
        && let Some(entry) = page_mark.entries().get(entry_index)
    {
        for subrecord in page_mark_u16_subrecord_candidates(entry.u16_fields()) {
            let words = subrecord.words();
            let line_range =
                page_mark_subrecord_line_range_candidate(&words, subrecord_line_range_max);
            for (field_index, value) in words.iter().copied().enumerate() {
                let word_index = subrecord.word_index() + field_index;
                push_page_mark_scoped_y_family_member(
                    output,
                    "parsedEntryU16Subrecord",
                    "direct-u16-px",
                    field_index,
                    Some(word_index),
                    Some(word_index * 2),
                    None,
                    None,
                    None,
                    line_range,
                    u32::from(value),
                    f32::from(value),
                );
                push_page_mark_scoped_y_family_member(
                    output,
                    "parsedEntryU16Subrecord",
                    "centipoint-to-css-px",
                    field_index,
                    Some(word_index),
                    Some(word_index * 2),
                    None,
                    None,
                    None,
                    line_range,
                    u32::from(value),
                    page_mark_centipoints_to_css_px(u32::from(value)),
                );
            }
        }
    }

    for (scan_index, header) in record_headers.iter().copied().enumerate() {
        let next_offset = record_headers
            .get(scan_index + 1)
            .map(|next| next.offset)
            .unwrap_or(bytes.len());
        let tail_start = header.offset.saturating_add(16);
        if tail_start >= next_offset || next_offset > bytes.len() {
            continue;
        }
        for byte_offset in (tail_start..next_offset.saturating_sub(15)).step_by(2) {
            let Some(subrecord) = page_mark_raw_u16_subrecord_candidate_at(bytes, byte_offset)
            else {
                continue;
            };
            let line_range = page_mark_subrecord_line_range_candidate(
                &subrecord.words,
                subrecord_line_range_max,
            );
            for (field_index, value) in subrecord.words.iter().copied().enumerate() {
                let field_byte_offset = subrecord.byte_offset + field_index * 2;
                if field_byte_offset >= next_offset {
                    continue;
                }
                let record_relative_byte_offset = field_byte_offset.saturating_sub(header.offset);
                let record_tail_word_index = record_relative_byte_offset
                    .checked_sub(16)
                    .map(|offset| offset / 2);
                let tail_block16_word_index = record_tail_word_index.map(|index| index % 16);
                let word_index = Some(field_byte_offset / 2);
                push_page_mark_scoped_y_family_member(
                    output,
                    "rawRecordHeaderTailU16Subrecord",
                    "direct-u16-px",
                    field_index,
                    word_index,
                    Some(field_byte_offset),
                    Some(header.index),
                    Some(scan_index),
                    tail_block16_word_index,
                    line_range,
                    u32::from(value),
                    f32::from(value),
                );
                push_page_mark_scoped_y_family_member(
                    output,
                    "rawRecordHeaderTailU16Subrecord",
                    "centipoint-to-css-px",
                    field_index,
                    word_index,
                    Some(field_byte_offset),
                    Some(header.index),
                    Some(scan_index),
                    tail_block16_word_index,
                    line_range,
                    u32::from(value),
                    page_mark_centipoints_to_css_px(u32::from(value)),
                );
            }
        }
    }
}

pub(crate) fn page_mark_subrecord_line_range_max_candidate(
    page_mark: &DocumentPageMark,
    record_headers: &[PageMarkRecordHeader],
) -> Option<u32> {
    record_headers
        .iter()
        .map(|header| header.line_end)
        .filter(|line_end| *line_end <= 10_000)
        .max()
        .or_else(|| {
            page_mark
                .entries()
                .iter()
                .filter_map(DocumentPageMarkEntry::line_end)
                .filter(|line_end| *line_end <= 10_000)
                .max()
        })
}

pub(crate) fn page_mark_subrecord_line_range_candidate(
    words: &[u16; 8],
    max_line_end: Option<u32>,
) -> Option<(u32, u32)> {
    let start = u32::from(words[4]);
    let end = u32::from(words[6]);
    (start <= end && max_line_end.is_some_and(|max_line_end| end <= max_line_end))
        .then_some((start, end))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_page_mark_scoped_y_family_member(
    output: &mut Vec<PageMarkScopedYFamilyMember>,
    source: &'static str,
    interpretation: &'static str,
    field_index: usize,
    word_index: Option<usize>,
    byte_offset: Option<usize>,
    raw_record_index: Option<u32>,
    raw_record_scan_index: Option<usize>,
    tail_block16_word_index: Option<usize>,
    subrecord_line_range_candidate: Option<(u32, u32)>,
    value: u32,
    value_px: f32,
) {
    if value_px.is_finite() {
        output.push(PageMarkScopedYFamilyMember {
            source,
            interpretation,
            family_kind: "u16-subrecord-field",
            field_index,
            word_index,
            byte_offset,
            raw_record_index,
            raw_record_scan_index,
            tail_block16_word_index,
            subrecord_line_start_candidate: subrecord_line_range_candidate.map(|range| range.0),
            subrecord_line_end_candidate: subrecord_line_range_candidate.map(|range| range.1),
            value,
            value_px,
        });
    }
}

pub(crate) fn page_mark_scoped_y_family_fits(
    members: Vec<PageMarkScopedYFamilyMember>,
    line_mark_record_indexes: &[usize],
    row_top_targets: &[f32],
    row_delta_targets: &[f32],
    tolerance_px: f32,
) -> Vec<PageMarkScopedYFamilyFit> {
    let mut grouped: BTreeMap<(&'static str, &'static str, &'static str, usize), Vec<_>> =
        BTreeMap::new();
    for member in members {
        grouped
            .entry((
                member.source,
                member.interpretation,
                member.family_kind,
                member.field_index,
            ))
            .or_default()
            .push(member);
    }

    grouped
        .into_iter()
        .filter_map(
            |((source, interpretation, family_kind, field_index), mut members)| {
                members.sort_by(|left, right| {
                    left.raw_record_scan_index
                        .cmp(&right.raw_record_scan_index)
                        .then_with(|| left.byte_offset.cmp(&right.byte_offset))
                        .then_with(|| left.word_index.cmp(&right.word_index))
                        .then_with(|| left.value.cmp(&right.value))
                });
                let table_top_target = row_top_targets.first().copied()?;
                let table_top_residuals = members
                    .iter()
                    .map(|member| member.value_px - table_top_target)
                    .filter(|residual| residual.abs() <= tolerance_px)
                    .collect::<Vec<_>>();
                let table_top_hit_members = members
                    .iter()
                    .filter(|member| (member.value_px - table_top_target).abs() <= tolerance_px)
                    .cloned()
                    .collect::<Vec<_>>();
                let table_top_hit_count = table_top_residuals.len();
                let row_line_range_coverage_count =
                    page_mark_scoped_y_family_line_range_coverage_count(
                        line_mark_record_indexes,
                        &members,
                    );
                let table_top_hit_line_range_coverage_count =
                    page_mark_scoped_y_family_line_range_coverage_count(
                        line_mark_record_indexes,
                        &table_top_hit_members,
                    );
                let row_top_residuals =
                    page_mark_scoped_y_family_nearest_residuals(row_top_targets, &members);
                let row_delta_residuals =
                    page_mark_scoped_y_family_nearest_delta_residuals(row_delta_targets, &members);
                let row_top_coverage_count = row_top_residuals
                    .iter()
                    .filter(|residual| residual.abs() <= tolerance_px)
                    .count();
                let row_delta_coverage_count = row_delta_residuals
                    .iter()
                    .filter(|residual| residual.abs() <= tolerance_px)
                    .count();
                let (row_top_mean_abs_residual, row_top_max_abs_residual) =
                    mean_and_max_abs_residual(&row_top_residuals);
                let (row_delta_mean_abs_residual, row_delta_max_abs_residual) =
                    mean_and_max_abs_residual(&row_delta_residuals);
                Some(PageMarkScopedYFamilyFit {
                    source,
                    interpretation,
                    family_kind,
                    field_index,
                    members,
                    table_top_residuals,
                    table_top_hit_members,
                    row_top_residuals,
                    row_delta_residuals,
                    table_top_hit_count,
                    row_top_coverage_count,
                    row_delta_coverage_count,
                    row_line_range_coverage_count,
                    table_top_hit_line_range_coverage_count,
                    row_top_mean_abs_residual,
                    row_top_max_abs_residual,
                    row_delta_mean_abs_residual,
                    row_delta_max_abs_residual,
                })
            },
        )
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_fit_ordering(
    left: &PageMarkScopedYFamilyFit,
    right: &PageMarkScopedYFamilyFit,
) -> Ordering {
    right
        .table_top_hit_count
        .cmp(&left.table_top_hit_count)
        .then_with(|| {
            right
                .row_top_coverage_count
                .cmp(&left.row_top_coverage_count)
        })
        .then_with(|| {
            option_f32_order(
                left.row_top_max_abs_residual,
                right.row_top_max_abs_residual,
            )
        })
        .then_with(|| {
            option_f32_order(
                left.row_top_mean_abs_residual,
                right.row_top_mean_abs_residual,
            )
        })
        .then_with(|| right.members.len().cmp(&left.members.len()))
        .then_with(|| left.source.cmp(right.source))
        .then_with(|| left.interpretation.cmp(right.interpretation))
        .then_with(|| left.field_index.cmp(&right.field_index))
}

pub(crate) fn page_mark_scoped_y_family_nearest_residuals(
    targets: &[f32],
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<f32> {
    targets
        .iter()
        .filter_map(|target| {
            members
                .iter()
                .map(|member| member.value_px - *target)
                .filter(|residual| residual.is_finite())
                .min_by(|left, right| {
                    left.abs()
                        .partial_cmp(&right.abs())
                        .unwrap_or(Ordering::Equal)
                })
        })
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_nearest_delta_residuals(
    targets: &[f32],
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<f32> {
    let deltas = page_mark_scoped_y_family_adjacent_value_deltas(members);
    targets
        .iter()
        .filter_map(|target| {
            deltas
                .iter()
                .map(|delta| *delta - *target)
                .filter(|residual| residual.is_finite())
                .min_by(|left, right| {
                    left.abs()
                        .partial_cmp(&right.abs())
                        .unwrap_or(Ordering::Equal)
                })
        })
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_adjacent_value_deltas(
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<f32> {
    members
        .windows(2)
        .filter_map(|pair| {
            let delta = pair[1].value_px - pair[0].value_px;
            delta.is_finite().then_some(delta)
        })
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_line_range_coverage_count(
    line_mark_record_indexes: &[usize],
    members: &[PageMarkScopedYFamilyMember],
) -> usize {
    line_mark_record_indexes
        .iter()
        .filter(|record_index| {
            members.iter().any(|member| {
                member
                    .subrecord_line_start_candidate
                    .zip(member.subrecord_line_end_candidate)
                    .is_some_and(|(start, end)| {
                        let record_index = **record_index as u32;
                        start <= record_index && record_index <= end
                    })
            })
        })
        .count()
}

pub(crate) fn push_page_mark_scoped_y_family_fit_json(
    output: &mut String,
    fit: &PageMarkScopedYFamilyFit,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(fit.source));
    output.push_str(",\"interpretation\":");
    output.push_str(&json_string(fit.interpretation));
    output.push_str(",\"familyKind\":");
    output.push_str(&json_string(fit.family_kind));
    output.push_str(",\"fieldIndex\":");
    output.push_str(&fit.field_index.to_string());
    output.push_str(",\"memberCount\":");
    output.push_str(&fit.members.len().to_string());
    output.push_str(",\"rawRecordIndexes\":");
    push_u32_array_json(output, &page_mark_scoped_y_family_raw_record_indexes(fit));
    output.push_str(",\"rawRecordScanIndexes\":");
    push_usize_array_json(
        output,
        &page_mark_scoped_y_family_raw_record_scan_indexes(fit),
    );
    output.push_str(",\"wordIndexes\":");
    push_usize_array_json(output, &page_mark_scoped_y_family_word_indexes(fit));
    output.push_str(",\"byteOffsets\":");
    push_usize_array_json(output, &page_mark_scoped_y_family_byte_offsets(fit));
    output.push_str(",\"tailBlock16WordIndexes\":");
    push_usize_array_json(
        output,
        &page_mark_scoped_y_family_tail_block16_word_indexes(fit),
    );
    output.push_str(",\"tableTopHitRawRecordIndexes\":");
    push_u32_array_json(
        output,
        &page_mark_scoped_y_family_table_top_hit_raw_record_indexes(fit),
    );
    output.push_str(",\"tableTopHitByteOffsets\":");
    push_usize_array_json(
        output,
        &page_mark_scoped_y_family_table_top_hit_byte_offsets(fit),
    );
    output.push_str(",\"rowLineRangeCoverageCount\":");
    output.push_str(&fit.row_line_range_coverage_count.to_string());
    output.push_str(",\"tableTopHitLineRangeCoverageCount\":");
    output.push_str(&fit.table_top_hit_line_range_coverage_count.to_string());
    output.push_str(",\"tableTopResidualsPx\":");
    push_f32_array_json(output, &fit.table_top_residuals);
    output.push_str(",\"tableTopHitCount\":");
    output.push_str(&fit.table_top_hit_count.to_string());
    output.push_str(",\"tableTopMeanAbsResidualPx\":");
    push_option_f32_json(
        output,
        mean_and_max_abs_residual(&fit.table_top_residuals).0,
    );
    output.push_str(",\"tableTopMaxAbsResidualPx\":");
    push_option_f32_json(
        output,
        mean_and_max_abs_residual(&fit.table_top_residuals).1,
    );
    output.push_str(",\"rowTopResidualsPx\":");
    push_f32_array_json(output, &fit.row_top_residuals);
    output.push_str(",\"rowTopCoverageCount\":");
    output.push_str(&fit.row_top_coverage_count.to_string());
    output.push_str(",\"rowTopMeanAbsResidualPx\":");
    push_option_f32_json(output, fit.row_top_mean_abs_residual);
    output.push_str(",\"rowTopMaxAbsResidualPx\":");
    push_option_f32_json(output, fit.row_top_max_abs_residual);
    output.push_str(",\"rowDeltaResidualBasis\":");
    output.push_str(&json_string("adjacent-ordered-member-value-delta"));
    output.push_str(",\"rowDeltaCandidateDeltasPx\":");
    push_f32_array_json(
        output,
        &page_mark_scoped_y_family_adjacent_value_deltas(&fit.members),
    );
    output.push_str(",\"rowDeltaResidualsPx\":");
    push_f32_array_json(output, &fit.row_delta_residuals);
    output.push_str(",\"rowDeltaCoverageCount\":");
    output.push_str(&fit.row_delta_coverage_count.to_string());
    output.push_str(",\"rowDeltaMeanAbsResidualPx\":");
    push_option_f32_json(output, fit.row_delta_mean_abs_residual);
    output.push_str(",\"rowDeltaMaxAbsResidualPx\":");
    push_option_f32_json(output, fit.row_delta_max_abs_residual);
    output.push_str(",\"sampleMembers\":[");
    for (index, member) in fit.members.iter().take(6).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_family_member_json(output, member);
    }
    output.push_str("]}");
}
