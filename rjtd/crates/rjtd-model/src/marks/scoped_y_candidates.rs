use super::*;
use crate::*;

pub(crate) fn push_page_mark_scoped_y_slot_fit_json(
    output: &mut String,
    fit: &PageMarkScopedYSlotFit,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(fit.source));
    output.push_str(",\"interpretation\":");
    output.push_str(&json_string(fit.interpretation));
    output.push_str(",\"grouping\":\"fieldIndex+tailBlock16WordIndex\"");
    output.push_str(",\"fieldIndex\":");
    output.push_str(&fit.field_index.to_string());
    output.push_str(",\"tailBlock16WordIndex\":");
    output.push_str(&fit.tail_block16_word_index.to_string());
    output.push_str(",\"memberCount\":");
    output.push_str(&fit.members.len().to_string());
    output.push_str(",\"rawRecordIndexes\":");
    push_u32_array_json(
        output,
        &page_mark_scoped_y_members_raw_record_indexes(&fit.members),
    );
    output.push_str(",\"rawRecordScanIndexes\":");
    push_usize_array_json(
        output,
        &page_mark_scoped_y_members_raw_record_scan_indexes(&fit.members),
    );
    output.push_str(",\"byteOffsets\":");
    push_usize_array_json(
        output,
        &page_mark_scoped_y_members_byte_offsets(&fit.members),
    );
    output.push_str(",\"rowLineRangeCoverageCount\":");
    output.push_str(&fit.row_line_range_coverage_count.to_string());
    output.push_str(",\"tableTopResidualsPx\":");
    push_f32_array_json(output, &fit.table_top_residuals);
    output.push_str(",\"tableTopHitCount\":");
    output.push_str(&fit.table_top_hit_count.to_string());
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
    output.push_str(",\"orderedLineMarkRecordCoveragePolicy\":");
    output.push_str(&json_string(
        "one-ordered-subrecord-member-per-line-mark-record",
    ));
    output.push_str(",\"orderedLineMarkRecordCoverageCount\":");
    output.push_str(&fit.ordered_line_mark_record_coverage_count.to_string());
    output.push_str(",\"orderedLineMarkRecordCoverageComplete\":");
    output.push_str(if fit.ordered_line_mark_record_coverage_complete {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"orderedLineMarkRecordIndexesCovered\":");
    push_usize_array_json(output, &fit.ordered_line_mark_record_indexes_covered);
    output.push_str(",\"orderedLineMarkRecordMemberByteOffsets\":");
    push_usize_array_json(output, &fit.ordered_line_mark_record_member_byte_offsets);
    output.push_str(",\"sampleMembers\":[");
    for (index, member) in fit.members.iter().take(6).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_family_member_json(output, member);
    }
    output.push_str("]}");
}

pub(crate) fn page_mark_scoped_y_members_raw_record_indexes(
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<u32> {
    members
        .iter()
        .filter_map(|member| member.raw_record_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_members_raw_record_scan_indexes(
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<usize> {
    members
        .iter()
        .filter_map(|member| member.raw_record_scan_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_members_byte_offsets(
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<usize> {
    members
        .iter()
        .filter_map(|member| member.byte_offset)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_raw_record_indexes(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<u32> {
    fit.members
        .iter()
        .filter_map(|member| member.raw_record_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_raw_record_scan_indexes(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<usize> {
    fit.members
        .iter()
        .filter_map(|member| member.raw_record_scan_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_word_indexes(fit: &PageMarkScopedYFamilyFit) -> Vec<usize> {
    fit.members
        .iter()
        .filter_map(|member| member.word_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_byte_offsets(fit: &PageMarkScopedYFamilyFit) -> Vec<usize> {
    fit.members
        .iter()
        .filter_map(|member| member.byte_offset)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_tail_block16_word_indexes(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<usize> {
    fit.members
        .iter()
        .filter_map(|member| member.tail_block16_word_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_table_top_hit_raw_record_indexes(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<u32> {
    fit.table_top_hit_members
        .iter()
        .filter_map(|member| member.raw_record_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn page_mark_scoped_y_family_table_top_hit_byte_offsets(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<usize> {
    fit.table_top_hit_members
        .iter()
        .filter_map(|member| member.byte_offset)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn push_page_mark_scoped_y_family_member_json(
    output: &mut String,
    member: &PageMarkScopedYFamilyMember,
) {
    output.push_str("{\"wordIndex\":");
    push_option_usize_json(output, member.word_index);
    output.push_str(",\"byteOffset\":");
    push_option_usize_json(output, member.byte_offset);
    output.push_str(",\"rawRecordIndex\":");
    match member.raw_record_index {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"rawRecordScanIndex\":");
    push_option_usize_json(output, member.raw_record_scan_index);
    output.push_str(",\"tailBlock16WordIndex\":");
    push_option_usize_json(output, member.tail_block16_word_index);
    output.push_str(",\"subrecordLineStartCandidate\":");
    match member.subrecord_line_start_candidate {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"subrecordLineEndCandidate\":");
    match member.subrecord_line_end_candidate {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"value\":");
    output.push_str(&member.value.to_string());
    output.push_str(",\"valuePx\":");
    output.push_str(&format!("{:.3}", member.value_px));
    output.push('}');
}

pub(crate) fn push_page_mark_scoped_y_value_candidate(
    output: &mut Vec<PageMarkScopedYValueCandidate>,
    source: &'static str,
    interpretation: &'static str,
    word_index: Option<usize>,
    byte_offset: Option<usize>,
    value: u32,
    value_px: f32,
) {
    if value_px.is_finite() {
        output.push(PageMarkScopedYValueCandidate {
            source,
            interpretation,
            word_index,
            byte_offset,
            value,
            value_px,
        });
    }
}

pub(crate) fn push_page_mark_scoped_nearest_y_candidates_json(
    output: &mut String,
    targets: &[f32],
    candidates: &[PageMarkScopedYValueCandidate],
) {
    output.push('[');
    for (target_index, target) in targets.iter().copied().enumerate() {
        if target_index > 0 {
            output.push(',');
        }
        output.push_str("{\"targetIndex\":");
        output.push_str(&target_index.to_string());
        output.push_str(",\"targetPx\":");
        output.push_str(&format!("{target:.3}"));
        output.push_str(",\"nearestCandidate\":");
        match nearest_page_mark_scoped_y_candidate(target, candidates) {
            Some((candidate, residual)) => {
                push_page_mark_scoped_y_candidate_json(output, candidate, residual);
            }
            None => output.push_str("null"),
        }
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_page_mark_scoped_y_hit_summary_json(
    output: &mut String,
    targets: &[f32],
    candidates: &[PageMarkScopedYValueCandidate],
    tolerance_px: f32,
) {
    let mut hit_count = 0usize;
    let mut target_hit = vec![false; targets.len()];
    let mut hits: Vec<(usize, f32, &PageMarkScopedYValueCandidate, f32)> = Vec::new();
    for (target_index, target) in targets.iter().copied().enumerate() {
        for candidate in candidates {
            let residual = candidate.value_px - target;
            if residual.abs() <= tolerance_px {
                hit_count += 1;
                target_hit[target_index] = true;
                if hits.len() < 16 {
                    hits.push((target_index, target, candidate, residual));
                }
            }
        }
    }
    output.push_str("{\"targetCount\":");
    output.push_str(&targets.len().to_string());
    output.push_str(",\"targetHitCount\":");
    output.push_str(&target_hit.iter().filter(|hit| **hit).count().to_string());
    output.push_str(",\"hitCount\":");
    output.push_str(&hit_count.to_string());
    output.push_str(",\"hits\":[");
    for (index, (target_index, target, candidate, residual)) in hits.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"targetIndex\":");
        output.push_str(&target_index.to_string());
        output.push_str(",\"targetPx\":");
        output.push_str(&format!("{target:.3}"));
        output.push_str(",\"candidate\":");
        push_page_mark_scoped_y_candidate_json(output, candidate, *residual);
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn nearest_page_mark_scoped_y_candidate(
    target: f32,
    candidates: &[PageMarkScopedYValueCandidate],
) -> Option<(&PageMarkScopedYValueCandidate, f32)> {
    candidates
        .iter()
        .map(|candidate| (candidate, candidate.value_px - target))
        .filter(|(_, residual)| residual.is_finite())
        .min_by(
            |(left_candidate, left_residual), (right_candidate, right_residual)| {
                left_residual
                    .abs()
                    .partial_cmp(&right_residual.abs())
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| left_candidate.source.cmp(right_candidate.source))
                    .then_with(|| {
                        left_candidate
                            .interpretation
                            .cmp(right_candidate.interpretation)
                    })
                    .then_with(|| left_candidate.word_index.cmp(&right_candidate.word_index))
            },
        )
}

pub(crate) fn push_page_mark_scoped_y_candidate_json(
    output: &mut String,
    candidate: &PageMarkScopedYValueCandidate,
    residual: f32,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(candidate.source));
    output.push_str(",\"interpretation\":");
    output.push_str(&json_string(candidate.interpretation));
    output.push_str(",\"wordIndex\":");
    push_option_usize_json(output, candidate.word_index);
    output.push_str(",\"byteOffset\":");
    push_option_usize_json(output, candidate.byte_offset);
    output.push_str(",\"value\":");
    output.push_str(&candidate.value.to_string());
    output.push_str(",\"valuePx\":");
    output.push_str(&format!("{:.3}", candidate.value_px));
    output.push_str(",\"residualPx\":");
    output.push_str(&format!("{residual:.3}"));
    output.push('}');
}

pub(crate) fn push_page_mark_scoped_nearest_delta_candidates_json(
    output: &mut String,
    targets: &[f32],
    candidates: &[PageMarkScopedYValueCandidate],
) {
    let delta_candidates = page_mark_scoped_y_pairwise_delta_candidates(candidates);
    output.push('[');
    for (target_index, target) in targets.iter().copied().enumerate() {
        if target_index > 0 {
            output.push(',');
        }
        output.push_str("{\"targetIndex\":");
        output.push_str(&target_index.to_string());
        output.push_str(",\"targetPx\":");
        output.push_str(&format!("{target:.3}"));
        output.push_str(",\"nearestCandidate\":");
        match nearest_page_mark_scoped_delta_candidate(target, &delta_candidates) {
            Some((candidate, residual)) => {
                push_page_mark_scoped_delta_candidate_json(output, candidate, residual);
            }
            None => output.push_str("null"),
        }
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_page_mark_scoped_delta_hit_summary_json(
    output: &mut String,
    targets: &[f32],
    candidates: &[PageMarkScopedYValueCandidate],
    tolerance_px: f32,
) {
    let delta_candidates = page_mark_scoped_y_pairwise_delta_candidates(candidates);
    let mut hit_count = 0usize;
    let mut target_hit = vec![false; targets.len()];
    let mut hits: Vec<(usize, f32, &PageMarkScopedYDeltaCandidate, f32)> = Vec::new();
    for (target_index, target) in targets.iter().copied().enumerate() {
        for candidate in &delta_candidates {
            let residual = candidate.delta_px - target;
            if residual.abs() <= tolerance_px {
                hit_count += 1;
                target_hit[target_index] = true;
                if hits.len() < 16 {
                    hits.push((target_index, target, candidate, residual));
                }
            }
        }
    }

    output.push_str("{\"targetCount\":");
    output.push_str(&targets.len().to_string());
    output.push_str(",\"targetHitCount\":");
    output.push_str(&target_hit.iter().filter(|hit| **hit).count().to_string());
    output.push_str(",\"hitCount\":");
    output.push_str(&hit_count.to_string());
    output.push_str(",\"candidateDeltaCount\":");
    output.push_str(&delta_candidates.len().to_string());
    output.push_str(",\"hits\":[");
    for (index, (target_index, target, candidate, residual)) in hits.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"targetIndex\":");
        output.push_str(&target_index.to_string());
        output.push_str(",\"targetPx\":");
        output.push_str(&format!("{target:.3}"));
        output.push_str(",\"candidate\":");
        push_page_mark_scoped_delta_candidate_json(output, candidate, *residual);
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn page_mark_scoped_y_pairwise_delta_candidates(
    candidates: &[PageMarkScopedYValueCandidate],
) -> Vec<PageMarkScopedYDeltaCandidate> {
    let mut grouped: BTreeMap<(&'static str, &'static str), Vec<&PageMarkScopedYValueCandidate>> =
        BTreeMap::new();
    for candidate in candidates {
        if candidate.value_px.is_finite() {
            grouped
                .entry((candidate.source, candidate.interpretation))
                .or_default()
                .push(candidate);
        }
    }

    let mut delta_candidates = Vec::new();
    for ((source, interpretation), mut members) in grouped {
        members.sort_by(|left, right| {
            left.word_index
                .cmp(&right.word_index)
                .then_with(|| left.byte_offset.cmp(&right.byte_offset))
                .then_with(|| left.value.cmp(&right.value))
        });
        for pair in members.windows(2) {
            let left = pair[0];
            let right = pair[1];
            let delta_px = right.value_px - left.value_px;
            if !delta_px.is_finite() {
                continue;
            }
            delta_candidates.push(PageMarkScopedYDeltaCandidate {
                source,
                interpretation,
                left_word_index: left.word_index,
                right_word_index: right.word_index,
                left_byte_offset: left.byte_offset,
                right_byte_offset: right.byte_offset,
                left_value: left.value,
                right_value: right.value,
                left_value_px: left.value_px,
                right_value_px: right.value_px,
                delta_px,
            });
        }
    }
    delta_candidates
}

pub(crate) fn nearest_page_mark_scoped_delta_candidate(
    target: f32,
    candidates: &[PageMarkScopedYDeltaCandidate],
) -> Option<(&PageMarkScopedYDeltaCandidate, f32)> {
    candidates
        .iter()
        .map(|candidate| (candidate, candidate.delta_px - target))
        .filter(|(_, residual)| residual.is_finite())
        .min_by(
            |(left_candidate, left_residual), (right_candidate, right_residual)| {
                left_residual
                    .abs()
                    .partial_cmp(&right_residual.abs())
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| left_candidate.source.cmp(right_candidate.source))
                    .then_with(|| {
                        left_candidate
                            .interpretation
                            .cmp(right_candidate.interpretation)
                    })
                    .then_with(|| {
                        left_candidate
                            .left_word_index
                            .cmp(&right_candidate.left_word_index)
                    })
                    .then_with(|| {
                        left_candidate
                            .right_word_index
                            .cmp(&right_candidate.right_word_index)
                    })
            },
        )
}

pub(crate) fn push_page_mark_scoped_delta_candidate_json(
    output: &mut String,
    candidate: &PageMarkScopedYDeltaCandidate,
    residual: f32,
) {
    output.push_str("{\"source\":");
    output.push_str(&json_string(candidate.source));
    output.push_str(",\"interpretation\":");
    output.push_str(&json_string(candidate.interpretation));
    output.push_str(",\"leftWordIndex\":");
    push_option_usize_json(output, candidate.left_word_index);
    output.push_str(",\"rightWordIndex\":");
    push_option_usize_json(output, candidate.right_word_index);
    output.push_str(",\"leftByteOffset\":");
    push_option_usize_json(output, candidate.left_byte_offset);
    output.push_str(",\"rightByteOffset\":");
    push_option_usize_json(output, candidate.right_byte_offset);
    output.push_str(",\"leftValue\":");
    output.push_str(&candidate.left_value.to_string());
    output.push_str(",\"rightValue\":");
    output.push_str(&candidate.right_value.to_string());
    output.push_str(",\"leftValuePx\":");
    output.push_str(&format!("{:.3}", candidate.left_value_px));
    output.push_str(",\"rightValuePx\":");
    output.push_str(&format!("{:.3}", candidate.right_value_px));
    output.push_str(",\"deltaPx\":");
    output.push_str(&format!("{:.3}", candidate.delta_px));
    output.push_str(",\"residualPx\":");
    output.push_str(&format!("{residual:.3}"));
    output.push('}');
}

pub(crate) fn page_mark_raw_numeric_hits_near(
    bytes: &[u8],
    target: f32,
    tolerance: f32,
) -> Vec<PageMarkRawNumericHit> {
    let mut hits = Vec::new();
    let max_u16_offset = bytes.len().saturating_sub(2);
    for byte_offset in (0..=max_u16_offset).step_by(2) {
        let Some(value) = read_be16_at(bytes, byte_offset) else {
            continue;
        };
        let residual_px = f32::from(value) - target;
        if residual_px.abs() <= tolerance {
            hits.push(PageMarkRawNumericHit {
                kind: "u16be",
                byte_offset,
                value_index: byte_offset / 2,
                value: u32::from(value),
                residual_px,
            });
        }
    }
    let max_u32_offset = bytes.len().saturating_sub(4);
    for byte_offset in (0..=max_u32_offset).step_by(4) {
        let Some(value) = read_be32_at(bytes, byte_offset) else {
            continue;
        };
        let residual_px = value as f32 - target;
        if residual_px.abs() <= tolerance {
            hits.push(PageMarkRawNumericHit {
                kind: "u32be",
                byte_offset,
                value_index: byte_offset / 4,
                value,
                residual_px,
            });
        }
    }
    hits.sort_by(|left, right| {
        left.residual_px
            .abs()
            .partial_cmp(&right.residual_px.abs())
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.byte_offset.cmp(&right.byte_offset))
            .then_with(|| left.kind.cmp(right.kind))
    });
    hits
}

pub(crate) fn push_page_mark_raw_numeric_hit_json(
    output: &mut String,
    hit: &PageMarkRawNumericHit,
    bytes: &[u8],
    record_headers: &[PageMarkRecordHeader],
) {
    output.push_str("{\"kind\":");
    output.push_str(&json_string(hit.kind));
    output.push_str(",\"byteOffset\":");
    output.push_str(&hit.byte_offset.to_string());
    output.push_str(",\"valueIndex\":");
    output.push_str(&hit.value_index.to_string());
    output.push_str(",\"value\":");
    output.push_str(&hit.value.to_string());
    output.push_str(",\"hex\":");
    if hit.kind == "u16be" {
        output.push_str(&json_string(&format!("0x{:04x}", hit.value)));
    } else {
        output.push_str(&json_string(&format!("0x{:08x}", hit.value)));
    }
    output.push_str(",\"residualPx\":");
    output.push_str(&format!("{:.3}", hit.residual_px));
    output.push_str(",\"recordContext\":");
    match page_mark_raw_numeric_hit_record_context(record_headers, bytes.len(), hit.byte_offset) {
        Some(context) => push_page_mark_raw_numeric_hit_record_context_json(output, context),
        None => output.push_str("null"),
    }
    output.push_str(",\"enclosingSubrecord\":");
    match page_mark_raw_numeric_hit_enclosing_subrecord(bytes, hit) {
        Some(candidate) => push_page_mark_raw_u16_subrecord_candidate_json(output, candidate),
        None => output.push_str("null"),
    }
    output.push_str(",\"contextU16BE\":");
    let (context_start, context_words) = page_mark_raw_numeric_hit_context_words(bytes, hit);
    output.push_str("{\"source\":\"/PageMark raw u16 window\",\"wordWindowStartByteOffset\":");
    output.push_str(&context_start.to_string());
    output.push_str(",\"wordWindowCenterByteOffset\":");
    output.push_str(&hit.byte_offset.to_string());
    output.push_str(",\"words\":");
    push_u16_array_json(output, &context_words);
    output.push_str(",\"wordsHex\":");
    push_u16_hex_array_json(output, &context_words);
    output.push('}');
    output.push('}');
}

pub(crate) fn page_mark_raw_numeric_hit_enclosing_subrecord(
    bytes: &[u8],
    hit: &PageMarkRawNumericHit,
) -> Option<PageMarkRawU16SubrecordCandidate> {
    if hit.kind != "u16be" {
        return None;
    }
    let hit_offset = hit.byte_offset - (hit.byte_offset % 2);
    let first_offset = hit_offset.saturating_sub(14);
    (first_offset..=hit_offset).step_by(2).find_map(|offset| {
        let mut candidate = page_mark_raw_u16_subrecord_candidate_at(bytes, offset)?;
        let candidate_start = candidate.byte_offset;
        let candidate_end = candidate_start + 16;
        if candidate_start <= hit_offset && hit_offset < candidate_end {
            candidate.field_index = (hit_offset - candidate_start) / 2;
            (candidate.words[candidate.field_index] as u32 == hit.value).then_some(candidate)
        } else {
            None
        }
    })
}

pub(crate) fn page_mark_raw_u16_subrecord_candidate_at(
    bytes: &[u8],
    byte_offset: usize,
) -> Option<PageMarkRawU16SubrecordCandidate> {
    let raw = bytes.get(byte_offset..byte_offset + 16)?;
    let mut words = [0u16; 8];
    for (index, chunk) in raw.chunks_exact(2).enumerate() {
        words[index] = u16::from_be_bytes([chunk[0], chunk[1]]);
    }
    if words[3] != 0 || words[5] != 0 || words[7] != 0 || words[4] > words[6] {
        return None;
    }
    Some(PageMarkRawU16SubrecordCandidate {
        byte_offset,
        field_index: 0,
        words,
    })
}

pub(crate) fn push_page_mark_raw_u16_subrecord_candidate_json(
    output: &mut String,
    candidate: PageMarkRawU16SubrecordCandidate,
) {
    output.push_str("{\"source\":\"/PageMark raw u16 subrecord scan\"");
    output.push_str(",\"byteOffset\":");
    output.push_str(&candidate.byte_offset.to_string());
    output.push_str(",\"fieldIndex\":");
    output.push_str(&candidate.field_index.to_string());
    output.push_str(",\"fieldRole\":");
    output.push_str(&json_string(match candidate.field_index {
        2 => "unknown-u16-field-2",
        _ => "unknown-u16-field",
    }));
    output.push_str(",\"words\":");
    push_u16_array_json(output, &candidate.words);
    output.push_str(",\"wordsHex\":");
    push_u16_hex_array_json(output, &candidate.words);
    let u32_fields = page_mark_raw_u16_subrecord_u32_fields(&candidate.words);
    output.push_str(",\"u32Fields\":");
    push_u32_array_json(output, &u32_fields);
    output.push_str(",\"u32FieldsHex\":");
    push_u32_hex8_array_json(output, &u32_fields);
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false}");
}

pub(crate) fn page_mark_raw_u16_subrecord_u32_fields(words: &[u16; 8]) -> [u32; 4] {
    [
        (u32::from(words[0]) << 16) | u32::from(words[1]),
        (u32::from(words[2]) << 16) | u32::from(words[3]),
        (u32::from(words[4]) << 16) | u32::from(words[5]),
        (u32::from(words[6]) << 16) | u32::from(words[7]),
    ]
}

pub(crate) fn page_mark_raw_numeric_hit_record_context(
    record_headers: &[PageMarkRecordHeader],
    stream_len: usize,
    byte_offset: usize,
) -> Option<PageMarkRawNumericHitRecordContext> {
    record_headers
        .iter()
        .enumerate()
        .find_map(|(scan_index, header)| {
            let record_next_byte_offset = record_headers
                .get(scan_index + 1)
                .map(|next| next.offset)
                .unwrap_or(stream_len);
            (header.offset <= byte_offset && byte_offset < record_next_byte_offset).then(|| {
                let record_relative_byte_offset = byte_offset.saturating_sub(header.offset);
                let record_tail_relative_byte_offset = record_relative_byte_offset.checked_sub(16);
                let record_tail_word_index =
                    record_tail_relative_byte_offset.map(|offset| offset / 2);
                let record_tail_block16_index =
                    record_tail_word_index.map(|word_index| word_index / 16);
                let record_tail_block16_word_index =
                    record_tail_word_index.map(|word_index| word_index % 16);
                PageMarkRawNumericHitRecordContext {
                    scan_index,
                    record_byte_offset: header.offset,
                    record_next_byte_offset,
                    record_index: header.index,
                    record_line_start: header.line_start,
                    record_line_end: header.line_end,
                    record_relative_byte_offset,
                    record_tail_relative_byte_offset,
                    record_tail_word_index,
                    record_tail_block16_index,
                    record_tail_block16_word_index,
                }
            })
        })
}

pub(crate) fn push_page_mark_raw_numeric_hit_record_context_json(
    output: &mut String,
    context: PageMarkRawNumericHitRecordContext,
) {
    output.push_str("{\"source\":\"/PageMark raw record scan\"");
    output.push_str(",\"scanIndex\":");
    output.push_str(&context.scan_index.to_string());
    output.push_str(",\"recordByteOffset\":");
    output.push_str(&context.record_byte_offset.to_string());
    output.push_str(",\"recordNextByteOffset\":");
    output.push_str(&context.record_next_byte_offset.to_string());
    output.push_str(",\"recordIndex\":");
    output.push_str(&context.record_index.to_string());
    output.push_str(",\"recordLineStart\":");
    output.push_str(&context.record_line_start.to_string());
    output.push_str(",\"recordLineEnd\":");
    output.push_str(&context.record_line_end.to_string());
    output.push_str(",\"recordRelativeByteOffset\":");
    output.push_str(&context.record_relative_byte_offset.to_string());
    output.push_str(",\"recordTailRelativeByteOffset\":");
    push_option_usize_json(output, context.record_tail_relative_byte_offset);
    output.push_str(",\"recordTailWordIndex\":");
    push_option_usize_json(output, context.record_tail_word_index);
    output.push_str(",\"recordTailBlock16Index\":");
    push_option_usize_json(output, context.record_tail_block16_index);
    output.push_str(",\"recordTailBlock16WordIndex\":");
    push_option_usize_json(output, context.record_tail_block16_word_index);
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false}");
}

pub(crate) fn push_page_mark_raw_numeric_hit_record_context_summary_json(
    output: &mut String,
    contexts: &[PageMarkRawNumericHitRecordContext],
) {
    let mut distinct_record_indexes = contexts
        .iter()
        .map(|context| context.record_index as usize)
        .collect::<Vec<_>>();
    distinct_record_indexes.sort_unstable();
    distinct_record_indexes.dedup();
    let mut distinct_tail_block16_word_indexes = contexts
        .iter()
        .filter_map(|context| context.record_tail_block16_word_index)
        .collect::<Vec<_>>();
    distinct_tail_block16_word_indexes.sort_unstable();
    distinct_tail_block16_word_indexes.dedup();
    let all_hits_in_single_record_header =
        !contexts.is_empty() && distinct_record_indexes.len() == 1;
    let all_hits_share_tail_block16_word_index =
        !contexts.is_empty() && distinct_tail_block16_word_indexes.len() == 1;

    output.push_str("{\"source\":\"/PageMark raw numeric scan context\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"hitContextCount\":");
    output.push_str(&contexts.len().to_string());
    output.push_str(",\"distinctRecordIndexes\":");
    push_usize_array_json(output, &distinct_record_indexes);
    output.push_str(",\"allHitsInSingleRecordHeader\":");
    output.push_str(if all_hits_in_single_record_header {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"distinctTailBlock16WordIndexes\":");
    push_usize_array_json(output, &distinct_tail_block16_word_indexes);
    output.push_str(",\"allHitsShareTailBlock16WordIndex\":");
    output.push_str(if all_hits_share_tail_block16_word_index {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"raw-hit-context-diagnostic-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "numeric-hit-context-does-not-decode-field-semantics",
    ));
    output.push('}');
}

pub(crate) fn page_mark_raw_numeric_hit_context_words(
    bytes: &[u8],
    hit: &PageMarkRawNumericHit,
) -> (usize, Vec<u16>) {
    let aligned_hit_offset = hit.byte_offset - (hit.byte_offset % 2);
    let start = aligned_hit_offset.saturating_sub(12);
    let end = (aligned_hit_offset + 14).min(bytes.len());
    let mut words = Vec::new();
    for offset in (start..end.saturating_sub(1)).step_by(2) {
        if let Some(value) = read_be16_at(bytes, offset) {
            words.push(value);
        }
    }
    (start, words)
}
