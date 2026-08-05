use super::*;

pub(super) const LINE_MARK_PATH: &str = "/LineMark";

pub(super) const LINE_MARK_BE_DELTA_HEADER_BYTES: usize = 18;

pub(super) const LINE_MARK_BE_DELTA_COUNT_OFFSET: usize = 8;

pub(super) const LINE_MARK_BE_DELTA_BASE_UNIT: usize = 16;

pub(super) const LINE_MARK_BE_DELTA_RECORD_BYTES: usize = 4;

pub(super) const SHANAI_LAN_LINE_MARK_PROFILE_ABSENT: &str = "absent";

pub(super) const SHANAI_LAN_LINE_MARK_PROFILE_BE_DELTA_V1: &str = "be16-delta-v1";

pub(super) const SHANAI_LAN_LINE_MARK_PROFILE_MACRO_STYLE: &str = "macro-stream-style-reference";

pub(super) const SHANAI_LAN_LINE_MARK_PROFILE_UNPARSED: &str = "unparsed";

pub(super) const PAGE_MARK_CENTIPOINT_TO_CSS_PX: f32 = PDF_POINT_TO_CSS_PX / 100.0;

pub(super) const PAGE_MARK_SEPARATOR_MIN_Y_CENTIPOINTS: u16 = 10_000;

pub(super) const PAGE_MARK_SEPARATOR_MAX_Y_CENTIPOINTS: u16 = 60_000;

pub(super) const PAGE_MARK_SEPARATOR_STROKE_WIDTH_PX: f32 = 1.15;

pub(super) fn page_mark_u16_subrecord_candidates(
    fields: &[u16],
) -> Vec<PageMarkU16SubrecordCandidate> {
    fields
        .windows(8)
        .enumerate()
        .filter_map(|(word_index, window)| {
            let words = [
                window[0], window[1], window[2], window[3], window[4], window[5], window[6],
                window[7],
            ];
            page_mark_u16_subrecord_words_look_plausible(&words).then_some(
                PageMarkU16SubrecordCandidate {
                    word_index,
                    byte_offset: word_index * 2,
                    words,
                },
            )
        })
        .collect()
}

pub(super) fn page_mark_u16_subrecord_words_look_plausible(words: &[u16; 8]) -> bool {
    words[3] == 0 && words[5] == 0 && words[7] == 0 && words[4] <= words[6]
}

pub(super) fn page_mark_u16_subrecord_u32_fields(words: &[u16; 8]) -> [u32; 4] {
    [
        (u32::from(words[0]) << 16) | u32::from(words[1]),
        (u32::from(words[2]) << 16) | u32::from(words[3]),
        (u32::from(words[4]) << 16) | u32::from(words[5]),
        (u32::from(words[6]) << 16) | u32::from(words[7]),
    ]
}

pub(super) fn push_page_mark_u16_subrecord_scan_json(
    output: &mut String,
    fields: &[u16],
    entry_stream_byte_offset: usize,
) {
    let candidates = page_mark_u16_subrecord_candidates(fields);
    output.push_str("{\"source\":\"/PageMark raw u16 subrecord scan\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidates.len().to_string());
    output.push_str(",\"candidates\":[");
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let words = candidate.words();
        let u32_fields = candidate.u32_fields();
        output.push_str("{\"entryRelativeByteOffset\":");
        output.push_str(&candidate.byte_offset().to_string());
        output.push_str(",\"streamByteOffset\":");
        output.push_str(&(entry_stream_byte_offset + candidate.byte_offset()).to_string());
        output.push_str(",\"wordIndex\":");
        output.push_str(&candidate.word_index().to_string());
        output.push_str(",\"words\":");
        push_u16_array_json(output, &words);
        output.push_str(",\"wordsHex\":");
        push_u16_hex_array_json(output, &words);
        output.push_str(",\"u32Fields\":");
        push_u32_array_json(output, &u32_fields);
        output.push_str(",\"u32FieldsHex\":");
        push_u32_hex8_array_json(output, &u32_fields);
        output.push_str(",\"decoded\":false}");
    }
    output.push_str("]}");
}

pub fn page_mark_u16_geometry_profile(fields: &[u16]) -> PageMarkU16GeometryProfile {
    let field = |index: usize| fields.get(index).copied();
    let selected_field_indexes = [10usize, 13, 14, 17, 18, 19, 20, 21];
    let selected_fields_all_zero = selected_field_indexes
        .iter()
        .all(|word_index| field(*word_index) == Some(0));
    let non_zero_additive_unit_candidate =
        field(13)
            .zip(field(14))
            .zip(field(21))
            .is_some_and(|((primary, secondary), combined)| {
                primary > 0
                    && secondary > 0
                    && combined > 0
                    && primary
                        .checked_add(secondary)
                        .is_some_and(|sum| sum == combined)
            });

    PageMarkU16GeometryProfile {
        selected_fields_all_zero,
        non_zero_additive_unit_candidate,
        word20_is_00ff: field(20) == Some(0x00ff),
    }
}

pub(super) fn push_line_mark_record_stride_fields_json(
    output: &mut String,
    record_indexes: &[usize],
) {
    let stride = uniform_usize_stride(record_indexes);
    output.push_str(",\"lineMarkRecordIndexes\":");
    push_usize_array_json(output, record_indexes);
    output.push_str(",\"uniformLineMarkRecordStride\":");
    output.push_str(if stride.is_some() { "true" } else { "false" });
    output.push_str(",\"lineMarkRecordStride\":");
    push_optional_usize_json(output, stride);
    output.push_str(",\"interleavedLineMarkRecordCountBetweenRows\":");
    push_optional_usize_json(output, stride.map(|value| value.saturating_sub(1)));
}

pub(super) fn page_marks_json(page_marks: &[DocumentPageMark]) -> String {
    let mut output = String::from("[");
    for (index, page_mark) in page_marks.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"sourceStream\":");
        output.push_str(&json_string(page_mark.source_stream()));
        output.push_str(",\"family\":");
        output.push_str(&json_string(page_mark.family()));
        output.push_str(",\"headerCount\":");
        output.push_str(&page_mark.header_count().to_string());
        output.push_str(",\"headerStride\":");
        output.push_str(&page_mark.header_stride().to_string());
        output.push_str(",\"headerLastIndex\":");
        output.push_str(&page_mark.header_last_index().to_string());
        output.push_str(",\"entryCount\":");
        output.push_str(&page_mark.entries().len().to_string());
        output.push_str(",\"trailingByteLength\":");
        output.push_str(&page_mark.trailing_byte_len().to_string());
        output.push_str(",\"entries\":[");
        for (entry_index, entry) in page_mark.entries().iter().enumerate() {
            if entry_index > 0 {
                output.push(',');
            }
            output.push_str("{\"rowIndex\":");
            output.push_str(&entry.row_index().to_string());
            output.push_str(",\"index\":");
            push_option_u32_json(&mut output, entry.index());
            output.push_str(",\"flags\":");
            push_option_u32_json(&mut output, entry.flags());
            output.push_str(",\"flagsHex\":");
            if let Some(flags) = entry.flags() {
                output.push_str(&json_string(&format!("0x{flags:08x}")));
            } else {
                output.push_str("null");
            }
            output.push_str(",\"lineStart\":");
            push_option_u32_json(&mut output, entry.line_start());
            output.push_str(",\"lineEnd\":");
            push_option_u32_json(&mut output, entry.line_end());
            output.push_str(",\"rawLength\":");
            output.push_str(&entry.raw_len().to_string());
            output.push_str(",\"rawHex\":");
            output.push_str(&json_string(&hex_bytes(entry.raw())));
            output.push_str(",\"u16Fields\":");
            push_u16_array_json(&mut output, entry.u16_fields());
            output.push_str(",\"u16FieldsHex\":");
            push_u16_hex_array_json(&mut output, entry.u16_fields());
            output.push_str(",\"u16GeometryClass\":");
            output.push_str(&json_string(entry.u16_geometry_profile().class_name()));
            output.push_str(",\"u16SubrecordScan\":");
            push_page_mark_u16_subrecord_scan_json(
                &mut output,
                entry.u16_fields(),
                page_mark_entry_stream_byte_offset(page_mark, entry_index),
            );
            output.push_str(",\"u32Fields\":");
            push_u32_array_json(&mut output, entry.u32_fields());
            output.push_str(",\"u32FieldsHex\":");
            push_u32_hex8_array_json(&mut output, entry.u32_fields());
            output.push_str(",\"u16GeometryHypotheses\":");
            push_page_mark_u16_geometry_hypotheses_json(&mut output, entry.u16_fields(), None);
            output.push_str(",\"decoded\":false}");
        }
        output.push_str("],\"decoded\":false}");
    }
    output.push(']');
    output
}

pub(super) fn page_mark_entry_stream_byte_offset(
    page_mark: &DocumentPageMark,
    entry_index: usize,
) -> usize {
    12 + page_mark
        .entries()
        .iter()
        .take(entry_index)
        .map(DocumentPageMarkEntry::raw_len)
        .sum::<usize>()
}

pub(super) fn paper_marks_json(paper_marks: &[DocumentPaperMark]) -> String {
    let mut output = String::from("[");
    for (index, paper_mark) in paper_marks.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"sourceStream\":");
        output.push_str(&json_string(paper_mark.source_stream()));
        output.push_str(",\"headerCount\":");
        output.push_str(&paper_mark.header_count().to_string());
        output.push_str(",\"headerStride\":");
        output.push_str(&paper_mark.header_stride().to_string());
        output.push_str(",\"headerLastIndex\":");
        output.push_str(&paper_mark.header_last_index().to_string());
        output.push_str(",\"entryCount\":");
        output.push_str(&paper_mark.entries().len().to_string());
        output.push_str(",\"entries\":[");
        for (entry_index, entry) in paper_mark.entries().iter().enumerate() {
            if entry_index > 0 {
                output.push(',');
            }
            output.push_str("{\"rowIndex\":");
            output.push_str(&entry.row_index().to_string());
            output.push_str(",\"index\":");
            output.push_str(&entry.index().to_string());
            output.push_str(",\"flags\":");
            output.push_str(&entry.flags().to_string());
            output.push_str(",\"flagsHex\":");
            output.push_str(&json_string(&format!("0x{:08x}", entry.flags())));
            output.push_str(",\"rawLength\":");
            output.push_str(&entry.raw_len().to_string());
            output.push_str(",\"decoded\":false}");
        }
        output.push_str("],\"decoded\":false}");
    }
    output.push(']');
    output
}

pub(super) fn paper_mark_writing_mode_diagnostics(
    paper_marks: &[DocumentPaperMark],
) -> PaperMarkWritingModeDiagnostics {
    let flag_bit0_vertical_candidate = paper_marks.iter().any(|mark| {
        mark.entries()
            .iter()
            .any(|entry| entry.flags() & 0x0000_0001 != 0)
    });
    let flag_bit17_index_step_candidate = paper_marks.iter().any(|mark| {
        mark.entries()
            .iter()
            .any(|entry| entry.flags() & 0x0002_0000 != 0)
    });

    let mut evidence = Vec::new();
    if flag_bit0_vertical_candidate {
        evidence.push("paper-mark-flag-bit0-vertical-corpus-consistent");
    }
    if flag_bit17_index_step_candidate {
        evidence.push("paper-mark-flag-bit17-index-step-corpus-consistent");
        evidence.push("paper-mark-flag-bit17-landscape-negative-dousoukai-counterexample");
    }

    let blockers = if flag_bit0_vertical_candidate {
        vec!["paper-mark-writing-mode-flag-semantics-unproven"]
    } else {
        Vec::new()
    };

    PaperMarkWritingModeDiagnostics {
        candidate: flag_bit0_vertical_candidate.then_some(WritingMode::VerticalRl),
        flag_bit0_vertical_candidate,
        flag_bit17_index_step_candidate,
        evidence,
        blockers,
    }
}

pub(super) fn push_page_layer_page_mark_separator_json(
    output: &mut String,
    separator: &PageMarkSeparatorProjection,
) {
    output.push_str("{\"type\":\"pageMarkSeparator\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        separator.x, separator.y, separator.width, separator.stroke_width
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(separator.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(separator.projection_kind));
    output.push_str(",\"role\":");
    output.push_str(&json_string(separator.role));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"styleDecoded\":false,\"placementProven\":true,\"pageAssignmentDecoded\":");
    output.push_str(if separator.page_assignment_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceRecordOffset\":");
    output.push_str(&separator.source_record_offset.to_string());
    output.push_str(",\"sourceRecordIndex\":");
    output.push_str(&separator.source_record_index.to_string());
    output.push_str(",\"sourceLineRange\":{\"start\":");
    output.push_str(&separator.source_line_start.to_string());
    output.push_str(",\"end\":");
    output.push_str(&separator.source_line_end.to_string());
    output.push_str("},\"sourceYCentipoints\":");
    output.push_str(&separator.source_y_centipoints.to_string());
    output.push_str(",\"sourceAdvanceCentipoints\":");
    output.push_str(&separator.source_advance_centipoints.to_string());
    output.push_str(",\"placementBasis\":");
    output.push_str(&json_string(separator.placement_basis));
    output.push_str(",\"styleBasis\":");
    output.push_str(&json_string(separator.style_basis));
    output.push('}');
}

pub(super) fn push_page_mark_line_pitch_agreement_gate_json(
    output: &mut String,
    layout: PageLayout,
    line_start: Option<u32>,
    line_end: Option<u32>,
    row_height_px: Option<f32>,
    row_height_basis: Option<&str>,
    u16_fields: &[u16],
) {
    const PITCH_AGREEMENT_TOLERANCE_PX: f32 = 0.5;

    let line_gap_count = line_start
        .zip(line_end)
        .map(|(start, end)| end.saturating_sub(start));
    let body_height_px_per_line_gap = line_gap_count
        .filter(|count| *count > 0)
        .map(|count| layout.body_height_px() / count as f32);
    let row_height_residual_px = row_height_px
        .zip(body_height_px_per_line_gap)
        .map(|(row, pitch)| row - pitch);
    let abs_row_height_residual_px = row_height_residual_px.map(f32::abs);
    let pitch_agreement_ready =
        abs_row_height_residual_px.is_some_and(|residual| residual <= PITCH_AGREEMENT_TOLERANCE_PX);
    let blocked_reason = if row_height_px.is_none() {
        Some("source-row-height-candidate-absent")
    } else if body_height_px_per_line_gap.is_none() {
        Some("page-mark-line-gap-pitch-absent")
    } else if !pitch_agreement_ready {
        Some("source-row-height-and-page-mark-line-gap-disagree")
    } else {
        None
    };

    output.push_str("{\"source\":\"/PageMark body line-gap pitch+source row height\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"rowHeightCandidatePresent\":");
    output.push_str(if row_height_px.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowHeightPx\":");
    push_optional_f32_json(output, row_height_px);
    output.push_str(",\"rowHeightBasis\":");
    match row_height_basis {
        Some(basis) => output.push_str(&json_string(basis)),
        None => output.push_str("null"),
    }
    output.push_str(",\"lineStart\":");
    push_option_u32_json(output, line_start);
    output.push_str(",\"lineEnd\":");
    push_option_u32_json(output, line_end);
    output.push_str(",\"lineGapCount\":");
    push_option_u32_json(output, line_gap_count);
    output.push_str(",\"bodyHeightPxPerLineGap\":");
    push_optional_f32_json(output, body_height_px_per_line_gap);
    output.push_str(",\"rowHeightResidualPx\":");
    push_optional_f32_json(output, row_height_residual_px);
    output.push_str(",\"absRowHeightResidualPx\":");
    push_optional_f32_json(output, abs_row_height_residual_px);
    output.push_str(",\"tolerancePx\":");
    output.push_str(&format!("{PITCH_AGREEMENT_TOLERANCE_PX:.3}"));
    output.push_str(",\"pageMarkU16GeometryClass\":");
    output.push_str(&json_string(
        page_mark_u16_geometry_profile(u16_fields).class_name(),
    ));
    output.push_str(",\"pitchAgreementReady\":");
    output.push_str(if pitch_agreement_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"page-mark-line-pitch-agreement-candidate\",\"renderPromotionBlockedReason\":");
    match blocked_reason {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push('}');
}

pub(super) const PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES: [usize; 10] =
    [10, 13, 14, 15, 16, 17, 18, 19, 20, 21];

pub(super) fn page_mark_scoped_y_target_hit_counts(
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

pub(super) fn page_mark_scoped_delta_target_hit_counts(
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

pub(super) fn page_mark_raw_header_indexes_for_line_mark_record_indexes(
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

pub(super) fn page_mark_subrecord_nearest_line_span_matches<'a>(
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

pub(super) fn page_mark_raw_subrecord_line_span_candidates(
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

pub(super) fn push_page_mark_subrecord_line_span_matches_json(
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

pub(super) fn push_page_mark_raw_subrecord_line_span_candidate_json(
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
pub(super) fn push_page_mark_scoped_y_record_set_probe_json(
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

pub(super) fn collect_page_mark_entry_y_value_candidates(
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

pub(super) fn collect_page_mark_raw_header_y_value_candidates(
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
pub(super) fn push_page_mark_scoped_y_shared_field_family_residuals_json(
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
pub(super) fn push_page_mark_slot_scoped_subrecord_y_sequence_comparison_json(
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

pub(super) fn push_page_mark_scoped_y_best_slot_fit_json(
    output: &mut String,
    slots: &[PageMarkScopedYSlotFit],
    predicate: impl Fn(&PageMarkScopedYSlotFit) -> bool,
) {
    match slots.iter().find(|slot| predicate(slot)) {
        Some(slot) => push_page_mark_scoped_y_slot_fit_json(output, slot),
        None => output.push_str("null"),
    }
}

pub(super) fn push_page_mark_slot_scoped_page_scale_candidates_json(
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

pub(super) fn page_mark_scoped_y_slot_fits(
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

pub(super) fn page_mark_scoped_y_slot_fit_ordering(
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

pub(super) fn page_mark_scoped_y_ordered_line_range_coverage(
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

pub(super) fn page_mark_scoped_y_member_covers_line_mark_record(
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

pub(super) fn collect_page_mark_scoped_y_family_members(
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

pub(super) fn page_mark_subrecord_line_range_max_candidate(
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

pub(super) fn page_mark_subrecord_line_range_candidate(
    words: &[u16; 8],
    max_line_end: Option<u32>,
) -> Option<(u32, u32)> {
    let start = u32::from(words[4]);
    let end = u32::from(words[6]);
    (start <= end && max_line_end.is_some_and(|max_line_end| end <= max_line_end))
        .then_some((start, end))
}

#[allow(clippy::too_many_arguments)]
pub(super) fn push_page_mark_scoped_y_family_member(
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

pub(super) fn page_mark_scoped_y_family_fits(
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

pub(super) fn page_mark_scoped_y_family_fit_ordering(
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

pub(super) fn page_mark_scoped_y_family_nearest_residuals(
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

pub(super) fn page_mark_scoped_y_family_nearest_delta_residuals(
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

pub(super) fn page_mark_scoped_y_family_adjacent_value_deltas(
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

pub(super) fn page_mark_scoped_y_family_line_range_coverage_count(
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

pub(super) fn push_page_mark_scoped_y_family_fit_json(
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

pub(super) fn push_page_mark_scoped_y_slot_fit_json(
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

pub(super) fn page_mark_scoped_y_members_raw_record_indexes(
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<u32> {
    members
        .iter()
        .filter_map(|member| member.raw_record_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_members_raw_record_scan_indexes(
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<usize> {
    members
        .iter()
        .filter_map(|member| member.raw_record_scan_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_members_byte_offsets(
    members: &[PageMarkScopedYFamilyMember],
) -> Vec<usize> {
    members
        .iter()
        .filter_map(|member| member.byte_offset)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_family_raw_record_indexes(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<u32> {
    fit.members
        .iter()
        .filter_map(|member| member.raw_record_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_family_raw_record_scan_indexes(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<usize> {
    fit.members
        .iter()
        .filter_map(|member| member.raw_record_scan_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_family_word_indexes(fit: &PageMarkScopedYFamilyFit) -> Vec<usize> {
    fit.members
        .iter()
        .filter_map(|member| member.word_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_family_byte_offsets(fit: &PageMarkScopedYFamilyFit) -> Vec<usize> {
    fit.members
        .iter()
        .filter_map(|member| member.byte_offset)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_family_tail_block16_word_indexes(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<usize> {
    fit.members
        .iter()
        .filter_map(|member| member.tail_block16_word_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_family_table_top_hit_raw_record_indexes(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<u32> {
    fit.table_top_hit_members
        .iter()
        .filter_map(|member| member.raw_record_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn page_mark_scoped_y_family_table_top_hit_byte_offsets(
    fit: &PageMarkScopedYFamilyFit,
) -> Vec<usize> {
    fit.table_top_hit_members
        .iter()
        .filter_map(|member| member.byte_offset)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn push_page_mark_scoped_y_family_member_json(
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

pub(super) fn push_page_mark_scoped_y_value_candidate(
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

pub(super) fn push_page_mark_scoped_nearest_y_candidates_json(
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

pub(super) fn push_page_mark_scoped_y_hit_summary_json(
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

pub(super) fn nearest_page_mark_scoped_y_candidate(
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

pub(super) fn push_page_mark_scoped_y_candidate_json(
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

pub(super) fn push_page_mark_scoped_nearest_delta_candidates_json(
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

pub(super) fn push_page_mark_scoped_delta_hit_summary_json(
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

pub(super) fn page_mark_scoped_y_pairwise_delta_candidates(
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

pub(super) fn nearest_page_mark_scoped_delta_candidate(
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

pub(super) fn push_page_mark_scoped_delta_candidate_json(
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

pub(super) fn page_mark_raw_numeric_hits_near(
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

pub(super) fn push_page_mark_raw_numeric_hit_json(
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

pub(super) fn page_mark_raw_numeric_hit_enclosing_subrecord(
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

pub(super) fn page_mark_raw_u16_subrecord_candidate_at(
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

pub(super) fn push_page_mark_raw_u16_subrecord_candidate_json(
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

pub(super) fn page_mark_raw_u16_subrecord_u32_fields(words: &[u16; 8]) -> [u32; 4] {
    [
        (u32::from(words[0]) << 16) | u32::from(words[1]),
        (u32::from(words[2]) << 16) | u32::from(words[3]),
        (u32::from(words[4]) << 16) | u32::from(words[5]),
        (u32::from(words[6]) << 16) | u32::from(words[7]),
    ]
}

pub(super) fn page_mark_raw_numeric_hit_record_context(
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

pub(super) fn push_page_mark_raw_numeric_hit_record_context_json(
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

pub(super) fn push_page_mark_raw_numeric_hit_record_context_summary_json(
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

pub(super) fn page_mark_raw_numeric_hit_context_words(
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

#[allow(clippy::too_many_arguments)]
pub(super) fn push_line_mark_record_family_y_candidate_json(
    output: &mut String,
    layout: PageLayout,
    family: &str,
    span_interpretation: &str,
    record_indexes: &[usize],
    context: &TableGridPageMarkLineContext,
    row_height: f32,
    page_line_pitch: Option<f32>,
    reference_row_tops: &[f32],
) {
    let row_height_row_tops = line_mark_record_indexes_y_tops(
        layout,
        record_indexes,
        context.page_line_start,
        row_height,
    );
    let row_height_residuals = residuals_f32(&row_height_row_tops, reference_row_tops);
    let page_line_row_tops = page_line_pitch.map(|pitch| {
        line_mark_record_indexes_y_tops(layout, record_indexes, context.page_line_start, pitch)
    });
    let page_line_residuals = page_line_row_tops
        .as_ref()
        .map(|row_tops| residuals_f32(row_tops, reference_row_tops));

    output.push_str("{\"family\":");
    output.push_str(&json_string(family));
    output.push_str(",\"spanInterpretation\":");
    output.push_str(&json_string(span_interpretation));
    output.push_str(",\"recordIndexes\":");
    push_usize_array_json(output, record_indexes);
    output.push_str(",\"uniformRecordStride\":");
    output.push_str(if uniform_usize_stride(record_indexes).is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"recordStride\":");
    push_optional_usize_json(output, uniform_usize_stride(record_indexes));
    output.push_str(",\"pageMarkEntryIndex\":");
    output.push_str(&context.page_mark_entry_index.to_string());
    output.push_str(",\"pageIndexCandidate\":");
    push_option_usize_json(output, context.page_index_candidate);
    output.push_str(",\"pageLineStart\":");
    output.push_str(&context.page_line_start.to_string());
    output.push_str(",\"pageLineEnd\":");
    output.push_str(&context.page_line_end.to_string());
    output.push_str(",\"pageMarkU16FieldPreview\":");
    let preview_len = context.page_mark_u16_fields.len().min(24);
    push_u16_array_json(output, &context.page_mark_u16_fields[..preview_len]);
    output.push_str(",\"rowHeightPitchPx\":");
    output.push_str(&format!("{row_height:.3}"));
    output.push_str(",\"rowHeightPitchRowTops\":");
    push_f32_array_json(output, &row_height_row_tops);
    output.push_str(",\"rowHeightPitchResidualsPx\":");
    push_f32_array_json(output, &row_height_residuals);
    output.push_str(",\"rowHeightPitchMeanAbsResidualPx\":");
    push_optional_f32_json(output, mean_abs_f32(&row_height_residuals));
    output.push_str(",\"rowHeightPitchMaxAbsResidualPx\":");
    push_optional_f32_json(output, max_abs_f32(&row_height_residuals));
    output.push_str(",\"pageLinePitchPx\":");
    push_optional_f32_json(output, page_line_pitch);
    output.push_str(",\"pageLinePitchRowTops\":");
    match page_line_row_tops.as_ref() {
        Some(row_tops) => push_f32_array_json(output, row_tops),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageLinePitchResidualsPx\":");
    match page_line_residuals.as_ref() {
        Some(residuals) => push_f32_array_json(output, residuals),
        None => output.push_str("null"),
    }
    output.push_str(",\"pageLinePitchMeanAbsResidualPx\":");
    push_optional_f32_json(
        output,
        page_line_residuals
            .as_ref()
            .and_then(|residuals| mean_abs_f32(residuals)),
    );
    output.push_str(",\"pageLinePitchMaxAbsResidualPx\":");
    push_optional_f32_json(
        output,
        page_line_residuals
            .as_ref()
            .and_then(|residuals| max_abs_f32(residuals)),
    );
    output.push('}');
}

pub(super) fn line_mark_record_indexes_y_tops(
    layout: PageLayout,
    record_indexes: &[usize],
    page_line_start: usize,
    pitch: f32,
) -> Vec<f32> {
    record_indexes
        .iter()
        .map(|record_index| {
            layout.margin_px() + record_index.saturating_sub(page_line_start) as f32 * pitch
        })
        .collect()
}

pub(super) fn line_mark_record_indexes_y_residuals(
    layout: PageLayout,
    record_indexes: &[usize],
    page_line_start: usize,
    pitch: f32,
    reference_row_tops: &[f32],
) -> Vec<f32> {
    residuals_f32(
        &line_mark_record_indexes_y_tops(layout, record_indexes, page_line_start, pitch),
        reference_row_tops,
    )
}

pub(super) fn push_line_mark_delta_record_json(
    output: &mut String,
    words: &[u16],
    interval: Option<ShanaiLanLineMarkInterval>,
) {
    let Some(interval) = interval else {
        output.push_str("null");
        return;
    };
    let byte_offset = line_mark_be_delta_record_byte_offset(interval.record_index);
    let word_index = line_mark_be_delta_record_word_index(interval.record_index);
    output.push_str("{\"recordIndex\":");
    output.push_str(&interval.record_index.to_string());
    output.push_str(",\"byteOffset\":");
    output.push_str(&byte_offset.to_string());
    output.push_str(",\"wordIndex\":");
    output.push_str(&word_index.to_string());
    output.push_str(",\"delta\":");
    output.push_str(&line_mark_interval_span_units(interval).to_string());
    output.push_str(",\"unitRange\":");
    output.push_str(&source_range_json(interval.unit_start, interval.unit_end));
    output.push_str(",\"flagWord\":");
    output.push_str(&interval.flag_word.to_string());
    output.push_str(",\"flagWordHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", interval.flag_word)));
    output.push_str(",\"context\":");
    push_line_mark_word_context_json(output, words, word_index);
    output.push_str(",\"nearestTagBefore\":");
    push_line_mark_nearest_tag_json(output, words, word_index, true);
    output.push_str(",\"nearestTagAfter\":");
    push_line_mark_nearest_tag_json(output, words, word_index, false);
    output.push('}');
}

pub(super) fn push_line_mark_word_context_json(
    output: &mut String,
    words: &[u16],
    word_index: usize,
) {
    let start = word_index.saturating_sub(4);
    let end = word_index.saturating_add(8).min(words.len());
    let context = words.get(start..end).unwrap_or(&[]);
    output.push_str("{\"wordWindowStart\":");
    output.push_str(&start.to_string());
    output.push_str(",\"wordWindowEnd\":");
    output.push_str(&end.to_string());
    output.push_str(",\"words\":");
    push_u16_array_json(output, context);
    output.push_str(",\"wordsHex\":");
    push_u16_hex_array_json(output, context);
    output.push('}');
}

pub(super) fn push_line_mark_nearest_tag_json(
    output: &mut String,
    words: &[u16],
    word_index: usize,
    before: bool,
) {
    match nearest_line_mark_tag(words, word_index, before) {
        Some((tag_word_index, tag)) => {
            output.push_str("{\"wordIndex\":");
            output.push_str(&tag_word_index.to_string());
            output.push_str(",\"byteOffset\":");
            output.push_str(&(tag_word_index * 2).to_string());
            output.push_str(",\"deltaWords\":");
            output.push_str(&(tag_word_index as isize - word_index as isize).to_string());
            output.push_str(",\"tag\":");
            output.push_str(&tag.to_string());
            output.push_str(",\"tagHex\":");
            output.push_str(&json_string(&format!("0x{tag:04x}")));
            output.push('}');
        }
        None => output.push_str("null"),
    }
}

pub(super) fn nearest_line_mark_tag(
    words: &[u16],
    word_index: usize,
    before: bool,
) -> Option<(usize, u16)> {
    if before {
        let end = word_index.min(words.len());
        words[..end]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, word)| is_line_mark_tag_word(**word))
            .map(|(index, word)| (index, *word))
    } else {
        let start = word_index.saturating_add(1).min(words.len());
        words[start..]
            .iter()
            .enumerate()
            .find(|(_, word)| is_line_mark_tag_word(**word))
            .map(|(offset, word)| (start + offset, *word))
    }
}

pub(super) fn push_line_mark_tag_family_counts_json(output: &mut String, counts: [usize; 3]) {
    output.push_str("{\"0x1000\":");
    output.push_str(&counts[0].to_string());
    output.push_str(",\"0x1001\":");
    output.push_str(&counts[1].to_string());
    output.push_str(",\"0x1002\":");
    output.push_str(&counts[2].to_string());
    output.push('}');
}

pub(super) fn line_mark_tag_family_counts(words: &[u16]) -> [usize; 3] {
    let mut counts = [0usize; 3];
    for word in words {
        match *word {
            0x1000 => counts[0] += 1,
            0x1001 => counts[1] += 1,
            0x1002 => counts[2] += 1,
            _ => {}
        }
    }
    counts
}

pub(super) fn is_line_mark_tag_word(word: u16) -> bool {
    matches!(word, 0x1000..=0x1002)
}

pub(super) fn line_mark_interval_for_record(
    intervals: &[ShanaiLanLineMarkInterval],
    record_index: usize,
) -> Option<ShanaiLanLineMarkInterval> {
    intervals
        .iter()
        .copied()
        .find(|interval| interval.record_index == record_index)
}

pub(super) fn line_mark_interval_span_units(interval: ShanaiLanLineMarkInterval) -> usize {
    interval.unit_end.saturating_sub(interval.unit_start)
}

pub(super) fn page_mark_raw_subrecord_record_context(
    record_headers: &[PageMarkRecordHeader],
    subrecord_byte_offset: usize,
) -> Option<(usize, u32, usize)> {
    let (scan_index, header) =
        record_headers
            .iter()
            .copied()
            .enumerate()
            .find(|(scan_index, header)| {
                let next_offset = record_headers
                    .get(scan_index + 1)
                    .map(|next| next.offset)
                    .unwrap_or(usize::MAX);
                header.offset <= subrecord_byte_offset && subrecord_byte_offset < next_offset
            })?;
    let tail_block16_word_index = subrecord_byte_offset
        .saturating_sub(header.offset)
        .checked_sub(16)
        .map(|offset| (offset / 2) % 16)?;
    Some((scan_index, header.index, tail_block16_word_index))
}

pub(super) fn page_mark_subrecord_line_range_contains_record(
    candidate: &PageMarkRawSubrecordLineSpanCandidate,
    record_index: usize,
) -> bool {
    usize::from(candidate.line_start_candidate) <= record_index
        && record_index <= usize::from(candidate.line_end_candidate)
}

pub(super) fn page_mark_subrecord_line_range_record_distance(
    candidate: &PageMarkRawSubrecordLineSpanCandidate,
    record_index: usize,
) -> usize {
    if record_index < usize::from(candidate.line_start_candidate) {
        usize::from(candidate.line_start_candidate).saturating_sub(record_index)
    } else if usize::from(candidate.line_end_candidate) < record_index {
        record_index.saturating_sub(usize::from(candidate.line_end_candidate))
    } else {
        0
    }
}

pub(super) fn push_page_mark_u16_geometry_hypotheses_json(
    output: &mut String,
    fields: &[u16],
    layout: Option<PageMarkU16LayoutComparison>,
) {
    let field = |index: usize| fields.get(index).copied();
    let word_10 = field(10);
    let word_13 = field(13);
    let word_14 = field(14);
    let word_17 = field(17);
    let word_18 = field(18);
    let word_19 = field(19);
    let word_21 = field(21);
    let profile = page_mark_u16_geometry_profile(fields);
    let word_13_plus_14 = word_13
        .zip(word_14)
        .and_then(|(left, right)| left.checked_add(right));
    let word_21_minus_13 = word_21
        .zip(word_13)
        .and_then(|(full, primary)| full.checked_sub(primary));
    let selected_field_indexes = [10usize, 13, 14, 17, 18, 19, 20, 21];

    output.push_str("{\"source\":\"/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"profile\":");
    output.push_str(&json_string(profile.class_name()));
    output.push_str(",\"selectedFields\":[");
    for (index, word_index) in selected_field_indexes.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"wordIndex\":");
        output.push_str(&word_index.to_string());
        output.push_str(",\"value\":");
        push_optional_u16_json(output, field(*word_index));
        output.push_str(",\"hex\":");
        push_option_u16_hex_json(output, field(*word_index));
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"word10EqualsWord13\":");
    output.push_str(if word_10.zip(word_13).is_some_and(|(a, b)| a == b) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"word17EqualsWord18\":");
    output.push_str(if word_17.zip(word_18).is_some_and(|(a, b)| a == b) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"word18EqualsWord19\":");
    output.push_str(if word_18.zip(word_19).is_some_and(|(a, b)| a == b) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"word20Is0x00ff\":");
    output.push_str(if profile.word20_is_00ff() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"word13PlusWord14\":");
    push_optional_u16_json(output, word_13_plus_14);
    output.push_str(",\"word13PlusWord14EqualsWord21\":");
    output.push_str(
        if word_13_plus_14
            .zip(word_21)
            .is_some_and(|(sum, word_21)| sum == word_21)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"word21MinusWord13\":");
    push_optional_u16_json(output, word_21_minus_13);
    output.push_str(",\"word21MinusWord13EqualsWord14\":");
    output.push_str(
        if word_21_minus_13
            .zip(word_14)
            .is_some_and(|(difference, word_14)| difference == word_14)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"word19EqualsWord13\":");
    output.push_str(if word_19.zip(word_13).is_some_and(|(a, b)| a == b) {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"selectedFieldsAllZero\":");
    output.push_str(if profile.selected_fields_all_zero() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"nonZeroAdditiveUnitCandidate\":");
    output.push_str(if profile.non_zero_additive_unit_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"layoutComparisons\":");
    if let Some(layout) = layout {
        output.push_str("{\"pageWidthPx\":");
        output.push_str(&format!("{:.3}", layout.page_width_px));
        output.push_str(",\"pageHeightPx\":");
        output.push_str(&format!("{:.3}", layout.page_height_px));
        output.push_str(",\"pageMarginPx\":");
        output.push_str(&format!("{:.3}", layout.page_margin_px));
        output.push_str(",\"bodyWidthPx\":");
        output.push_str(&format!("{:.3}", layout.page_body_width_px));
        output.push_str(",\"pageWidthPxPerWord21Unit\":");
        push_optional_f32_json(
            output,
            word_21.map(|value| layout.page_width_px / f32::from(value)),
        );
        output.push_str(",\"pageHeightPxPerWord21Unit\":");
        push_optional_f32_json(
            output,
            word_21.map(|value| layout.page_height_px / f32::from(value)),
        );
        output.push_str(",\"bodyWidthPxPerWord21Unit\":");
        push_optional_f32_json(
            output,
            word_21.map(|value| layout.page_body_width_px / f32::from(value)),
        );
        output.push_str(",\"bodyWidthPxPerWord13Unit\":");
        push_optional_f32_json(
            output,
            word_13.map(|value| layout.page_body_width_px / f32::from(value)),
        );
        output.push_str(",\"marginPxPerWord14Unit\":");
        push_optional_f32_json(
            output,
            word_14.map(|value| layout.page_margin_px / f32::from(value)),
        );
        output.push_str(",\"pageWidthPxPerWord13Plus14Unit\":");
        push_optional_f32_json(
            output,
            word_13_plus_14.map(|value| layout.page_width_px / f32::from(value)),
        );
        output.push_str(",\"pageHeightPxPerWord13Plus14Unit\":");
        push_optional_f32_json(
            output,
            word_13_plus_14.map(|value| layout.page_height_px / f32::from(value)),
        );
        output.push_str(",\"bodyWidthPxPerWord13Plus14Unit\":");
        push_optional_f32_json(
            output,
            word_13_plus_14.map(|value| layout.page_body_width_px / f32::from(value)),
        );
        output.push_str(",\"marginPxPerWord21MinusWord13Unit\":");
        push_optional_f32_json(
            output,
            word_21_minus_13.map(|value| layout.page_margin_px / f32::from(value)),
        );
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(
        ",\"renderPromotionContribution\":\"page-mark-u16-horizontal-geometry-candidate-only\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("page-mark-u16-geometry-semantics-unproven"));
    output.push('}');
}

pub(super) fn table_candidate_direct_page_mark_line_hit_count(
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

pub(super) fn push_answer_sheet_section_line_mark_geometry_candidate_json(
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

pub(super) fn best_line_mark_interval_for_unit_range(
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

pub(super) fn line_mark_interval_match_key(
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

pub(super) fn push_page_mark_selected_fields_from_parts_json(
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

pub(super) fn line_mark_be_delta_record_byte_offset(record_index: usize) -> usize {
    LINE_MARK_BE_DELTA_HEADER_BYTES + record_index * LINE_MARK_BE_DELTA_RECORD_BYTES
}

pub(super) fn line_mark_be_delta_record_word_index(record_index: usize) -> usize {
    line_mark_be_delta_record_byte_offset(record_index) / 2
}

pub(super) fn page_mark_section_separator_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<PageMarkSeparatorProjection> {
    if page_number != 1 {
        return None;
    }

    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let text_projection = layout_box_text_projection(document, layout, page_number)?;
    let caption = text_projection
        .slots
        .iter()
        .rev()
        .find(|slot| slot.role == "caption")?;
    let body = text_projection
        .slots
        .iter()
        .find(|slot| slot.role == "body")?;
    let caption_bottom = caption.y + caption.line_height;
    let body_top = body.y;
    if body_top <= caption_bottom {
        return None;
    }

    let frame_projection = page_frame_projection(document, layout, page_number)?;
    let bar = frame_projection
        .shapes
        .iter()
        .filter(|shape| shape.role == "horizontalPatternBar")
        .max_by(|left, right| {
            left.width
                .partial_cmp(&right.width)
                .unwrap_or(Ordering::Equal)
        })?;

    let candidate = page_mark_separator_candidate(page_mark_bytes)?;
    let y = page_mark_centipoints_to_css_px(
        u32::from(candidate.y_centipoints) + u32::from(candidate.advance_centipoints),
    );
    if y < caption_bottom || y > body_top {
        return None;
    }

    Some(PageMarkSeparatorProjection {
        source: PAGE_MARK_PATH,
        projection_kind: "pageMarkSectionSeparatorProjection",
        role: "sectionSeparator",
        x: bar.x,
        y,
        width: bar.width,
        stroke_width: PAGE_MARK_SEPARATOR_STROKE_WIDTH_PX,
        source_record_offset: candidate.record_offset,
        source_record_index: candidate.record_index,
        source_line_start: candidate.line_start,
        source_line_end: candidate.line_end,
        source_y_centipoints: candidate.y_centipoints,
        source_advance_centipoints: candidate.advance_centipoints,
        placement_basis: "pageMarkCentipointInsideLayoutBoxCaptionBodyGap",
        style_basis: "pageMarkSeparatorTailAndRecurringMarkAdvance",
        page_assignment_decoded: false,
    })
}

pub(super) fn page_mark_separator_candidate(bytes: &[u8]) -> Option<PageMarkSeparatorCandidate> {
    let advance_centipoints = page_mark_recurring_advance_centipoints(bytes).unwrap_or(0);
    let headers = page_mark_record_headers(bytes);
    for (header_index, header) in headers.iter().enumerate() {
        let next_offset = headers
            .get(header_index + 1)
            .map(|next| next.offset)
            .unwrap_or(bytes.len());
        let tail_start = header.offset.checked_add(16)?;
        if tail_start > next_offset || next_offset > bytes.len() {
            continue;
        }
        let tail = &bytes[tail_start..next_offset];
        let Some(y_centipoints) = page_mark_separator_tail_y_centipoints(tail) else {
            continue;
        };
        return Some(PageMarkSeparatorCandidate {
            record_offset: header.offset,
            record_index: header.index,
            line_start: header.line_start,
            line_end: header.line_end,
            y_centipoints,
            advance_centipoints,
        });
    }
    None
}

pub(super) fn page_mark_record_headers(bytes: &[u8]) -> Vec<PageMarkRecordHeader> {
    let mut headers = Vec::new();
    let mut offset = 12usize;
    while offset + 16 <= bytes.len() {
        let Some(index) = read_be32_at(bytes, offset) else {
            break;
        };
        let Some(flags) = read_be32_at(bytes, offset + 4) else {
            break;
        };
        let Some(line_start) = read_be32_at(bytes, offset + 8) else {
            break;
        };
        let Some(line_end) = read_be32_at(bytes, offset + 12) else {
            break;
        };
        if flags == 0x0001_0000 && index < 256 && line_start <= line_end && line_end < 10_000 {
            headers.push(PageMarkRecordHeader {
                offset,
                index,
                flags,
                line_start,
                line_end,
            });
        }
        offset += 1;
    }
    headers
}

pub(super) fn page_mark_separator_tail_y_centipoints(tail: &[u8]) -> Option<u16> {
    if !tail
        .windows(4)
        .any(|window| window == [0xff, 0xff, 0x00, 0x00])
    {
        return None;
    }
    tail.chunks_exact(2)
        .filter_map(|chunk| {
            let value = u16::from_be_bytes([chunk[0], chunk[1]]);
            (PAGE_MARK_SEPARATOR_MIN_Y_CENTIPOINTS..=PAGE_MARK_SEPARATOR_MAX_Y_CENTIPOINTS)
                .contains(&value)
                .then_some(value)
        })
        .next_back()
}

pub(super) fn page_mark_recurring_advance_centipoints(bytes: &[u8]) -> Option<u16> {
    let mut candidates = Vec::new();
    for offset in 0..bytes.len().saturating_sub(3) {
        if bytes[offset] == 0x00 && bytes[offset + 1] == 0xff {
            let value = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
            if (1..=2_000).contains(&value) {
                candidates.push(value);
            }
        }
    }
    candidates.sort_unstable();
    let mut best = None;
    let mut best_count = 0usize;
    let mut index = 0usize;
    while index < candidates.len() {
        let value = candidates[index];
        let count = candidates[index..]
            .iter()
            .take_while(|candidate| **candidate == value)
            .count();
        if count > best_count {
            best = Some(value);
            best_count = count;
        }
        index += count;
    }
    best
}

pub(super) fn page_mark_centipoints_to_css_px(value: u32) -> f32 {
    value as f32 * PAGE_MARK_CENTIPOINT_TO_CSS_PX
}

pub(super) fn push_page_mark_section_separator_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    let Some(separator) = page_mark_section_separator_projection(document, layout, page_number)
    else {
        return;
    };
    svg.push_str(&format!(
        "<line class=\"rjtd-page-mark-separator\" data-source=\"{}\" data-projection-kind=\"{}\" data-role=\"{}\" data-source-record-offset=\"{}\" data-source-record-index=\"{}\" data-source-line-start=\"{}\" data-source-line-end=\"{}\" data-source-y-centipoints=\"{}\" data-source-advance-centipoints=\"{}\" data-placement-basis=\"{}\" data-style-basis=\"{}\" x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#555555\" stroke-width=\"{:.2}\" stroke-dasharray=\"2.2 2.2\" stroke-linecap=\"butt\"/>",
        escape_xml(separator.source),
        escape_xml(separator.projection_kind),
        escape_xml(separator.role),
        separator.source_record_offset,
        separator.source_record_index,
        separator.source_line_start,
        separator.source_line_end,
        separator.source_y_centipoints,
        separator.source_advance_centipoints,
        escape_xml(separator.placement_basis),
        escape_xml(separator.style_basis),
        separator.x,
        separator.y,
        separator.x + separator.width,
        separator.y,
        separator.stroke_width
    ));
}
