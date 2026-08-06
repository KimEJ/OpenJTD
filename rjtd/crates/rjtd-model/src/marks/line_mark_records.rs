use super::*;
use crate::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_line_mark_record_family_y_candidate_json(
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

pub(crate) fn line_mark_record_indexes_y_tops(
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

pub(crate) fn line_mark_record_indexes_y_residuals(
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

pub(crate) fn push_line_mark_delta_record_json(
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

pub(crate) fn push_line_mark_word_context_json(
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

pub(crate) fn push_line_mark_nearest_tag_json(
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

pub(crate) fn nearest_line_mark_tag(
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

pub(crate) fn push_line_mark_tag_family_counts_json(output: &mut String, counts: [usize; 3]) {
    output.push_str("{\"0x1000\":");
    output.push_str(&counts[0].to_string());
    output.push_str(",\"0x1001\":");
    output.push_str(&counts[1].to_string());
    output.push_str(",\"0x1002\":");
    output.push_str(&counts[2].to_string());
    output.push('}');
}

pub(crate) fn line_mark_tag_family_counts(words: &[u16]) -> [usize; 3] {
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

pub(crate) fn is_line_mark_tag_word(word: u16) -> bool {
    matches!(word, 0x1000..=0x1002)
}

/// `/LineMark` declares its be-delta record count in the stream header. The same
/// length guard the interval walk applies keeps malformed streams out, so the
/// source-declared count can be reported beside other literal stream bounds.
pub(crate) fn line_mark_declared_record_count(bytes: &[u8]) -> Option<usize> {
    let count = read_be16_at(bytes, LINE_MARK_BE_DELTA_COUNT_OFFSET).map(usize::from)?;
    (count > 0
        && bytes.len()
            >= LINE_MARK_BE_DELTA_HEADER_BYTES
                + count.saturating_mul(LINE_MARK_BE_DELTA_RECORD_BYTES))
    .then_some(count)
}

pub(crate) fn line_mark_interval_for_record(
    intervals: &[ShanaiLanLineMarkInterval],
    record_index: usize,
) -> Option<ShanaiLanLineMarkInterval> {
    intervals
        .iter()
        .copied()
        .find(|interval| interval.record_index == record_index)
}

pub(crate) fn line_mark_interval_span_units(interval: ShanaiLanLineMarkInterval) -> usize {
    interval.unit_end.saturating_sub(interval.unit_start)
}

pub(crate) fn page_mark_raw_subrecord_record_context(
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

pub(crate) fn page_mark_subrecord_line_range_contains_record(
    candidate: &PageMarkRawSubrecordLineSpanCandidate,
    record_index: usize,
) -> bool {
    usize::from(candidate.line_start_candidate) <= record_index
        && record_index <= usize::from(candidate.line_end_candidate)
}

pub(crate) fn page_mark_subrecord_line_range_record_distance(
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

pub(crate) fn push_page_mark_u16_geometry_hypotheses_json(
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
