use super::*;
use crate::*;

pub(crate) const LINE_MARK_PATH: &str = "/LineMark";

pub(crate) const LINE_MARK_BE_DELTA_HEADER_BYTES: usize = 18;

pub(crate) const LINE_MARK_BE_DELTA_COUNT_OFFSET: usize = 8;

/// `be16` at byte offset `12` of the `/LineMark` be-delta header, named by
/// position only. What it counts is not decoded here.
pub(crate) const LINE_MARK_BE_DELTA_HEADER_U16_12_OFFSET: usize = 12;

pub(crate) const LINE_MARK_BE_DELTA_BASE_UNIT: usize = 16;

pub(crate) const LINE_MARK_BE_DELTA_RECORD_BYTES: usize = 4;

pub(crate) const SHANAI_LAN_LINE_MARK_PROFILE_ABSENT: &str = "absent";

pub(crate) const SHANAI_LAN_LINE_MARK_PROFILE_BE_DELTA_V1: &str = "be16-delta-v1";

pub(crate) const SHANAI_LAN_LINE_MARK_PROFILE_MACRO_STYLE: &str = "macro-stream-style-reference";

pub(crate) const SHANAI_LAN_LINE_MARK_PROFILE_UNPARSED: &str = "unparsed";

pub(crate) const PAGE_MARK_CENTIPOINT_TO_CSS_PX: f32 = PDF_POINT_TO_CSS_PX / 100.0;

pub(crate) const PAGE_MARK_SEPARATOR_MIN_Y_CENTIPOINTS: u16 = 10_000;

pub(crate) const PAGE_MARK_SEPARATOR_MAX_Y_CENTIPOINTS: u16 = 60_000;

pub(crate) const PAGE_MARK_SEPARATOR_STROKE_WIDTH_PX: f32 = 1.15;

pub(crate) fn page_mark_u16_subrecord_candidates(
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

pub(crate) fn page_mark_u16_subrecord_words_look_plausible(words: &[u16; 8]) -> bool {
    words[3] == 0 && words[5] == 0 && words[7] == 0 && words[4] <= words[6]
}

pub(crate) fn page_mark_u16_subrecord_u32_fields(words: &[u16; 8]) -> [u32; 4] {
    [
        (u32::from(words[0]) << 16) | u32::from(words[1]),
        (u32::from(words[2]) << 16) | u32::from(words[3]),
        (u32::from(words[4]) << 16) | u32::from(words[5]),
        (u32::from(words[6]) << 16) | u32::from(words[7]),
    ]
}

pub(crate) fn push_page_mark_u16_subrecord_scan_json(
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

pub(crate) fn push_line_mark_record_stride_fields_json(
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

pub(crate) fn page_marks_json(page_marks: &[DocumentPageMark]) -> String {
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

pub(crate) fn page_mark_entry_stream_byte_offset(
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

pub(crate) fn paper_marks_json(paper_marks: &[DocumentPaperMark]) -> String {
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

pub(crate) fn paper_mark_writing_mode_diagnostics(
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

pub(crate) fn push_page_layer_page_mark_separator_json(
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

pub(crate) fn push_page_mark_line_pitch_agreement_gate_json(
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

pub(crate) const PAGE_MARK_HORIZONTAL_REFERENCE_WORD_INDEXES: [usize; 10] =
    [10, 13, 14, 15, 16, 17, 18, 19, 20, 21];
