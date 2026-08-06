use super::*;
use crate::*;

/// Observed high halves of `/PageMark` record-header `flags` be32 values. Only
/// the high half is used as a structural discriminator: the low half is what the
/// absolute-y-slot probe has been reading as a coordinate. Their semantics stay
/// undecoded.
pub(crate) const PAGE_MARK_RECORD_HEADER_FLAGS_HIGH_U16_VALUES: &[u16] = &[0x0001, 0x0005];

/// A `/PageMark` raw u16 subrecord is a record header read two bytes early.
///
/// Record header be32 layout at `O`: `index`, `flags`, `lineStart`, `lineEnd`.
/// Read as u16 words starting at `O - 2` that becomes
/// `[.., indexLow, flagsHigh, flagsLow, lineStartHigh, lineStartLow, lineEndHigh,
/// lineEndLow, ..]`, which is exactly the subrecord shape the scan accepts:
/// `words[3] == 0` is `lineStartHigh`, `words[5] == 0` is `lineEndHigh`, and
/// `words[4] <= words[6]` is `lineStart <= lineEnd`. The absolute-y-slot field
/// `words[2]` is therefore the low u16 of that record's `flags`, not a coordinate.
pub(crate) const PAGE_MARK_SUBRECORD_RECORD_HEADER_BACK_SHIFT_BYTES: usize = 2;

/// Same line-range sanity bound as `page_mark_record_headers`.
pub(crate) const PAGE_MARK_RECORD_HEADER_MAX_LINE_END: u32 = 10_000;

/// Same record-index sanity bound as `page_mark_record_headers`.
pub(crate) const PAGE_MARK_RECORD_HEADER_MAX_INDEX: u32 = 256;

/// One absolute-y-slot row resolved against the record header the subrecord
/// window actually overlaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageMarkSubrecordRecordFlagAliasRow {
    pub(crate) subrecord_byte_offset: usize,
    pub(crate) shifted_record_header_offset: usize,
    pub(crate) shifted_record_index: u32,
    pub(crate) shifted_record_flags: u32,
    pub(crate) flags_high_u16: u16,
    pub(crate) flags_low_u16: u16,
    pub(crate) field_value: u16,
    pub(crate) header_shape_valid: bool,
    pub(crate) line_range_matches_subrecord: bool,
}

impl PageMarkSubrecordRecordFlagAliasRow {
    pub(crate) fn field_equals_flags_low_u16(&self) -> bool {
        self.field_value == self.flags_low_u16
    }

    pub(crate) fn refutes_page_space_px(&self) -> bool {
        self.header_shape_valid
            && self.line_range_matches_subrecord
            && self.field_equals_flags_low_u16()
    }
}

/// Row-ordered structural read of the absolute-y-slot field as a record-header
/// `flags` half-word. Diagnostic-only: it refutes the direct-px reading, it does
/// not decode what the flags mean.
#[derive(Debug, Clone)]
pub(crate) struct TableGridSourceOnlyPageMarkAbsoluteYSlotRecordFlagAlias {
    pub(crate) back_shift_bytes: usize,
    pub(crate) rows: Vec<PageMarkSubrecordRecordFlagAliasRow>,
    pub(crate) page_space_px_plausible: bool,
}

impl TableGridSourceOnlyPageMarkAbsoluteYSlotRecordFlagAlias {
    pub(crate) fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn header_shape_valid_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.header_shape_valid)
            .count()
    }

    pub(crate) fn field_equals_flags_low_u16_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.field_equals_flags_low_u16())
            .count()
    }

    pub(crate) fn line_range_match_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.line_range_matches_subrecord)
            .count()
    }

    pub(crate) fn all_rows_refute_page_space_px(&self) -> bool {
        !self.rows.is_empty() && self.rows.iter().all(|row| row.refutes_page_space_px())
    }
}

/// Reads the record header the subrecord window overlaps, two bytes before the
/// subrecord start. Returns `None` when the shifted window is not a well-formed
/// `/PageMark` record header, so a coincidental subrecord cannot claim the alias.
pub(crate) fn page_mark_subrecord_shifted_record_header(
    bytes: &[u8],
    subrecord_byte_offset: usize,
) -> Option<PageMarkRecordHeader> {
    let offset =
        subrecord_byte_offset.checked_sub(PAGE_MARK_SUBRECORD_RECORD_HEADER_BACK_SHIFT_BYTES)?;
    let index = read_be32_at(bytes, offset)?;
    let flags = read_be32_at(bytes, offset + 4)?;
    let line_start = read_be32_at(bytes, offset + 8)?;
    let line_end = read_be32_at(bytes, offset + 12)?;
    if index >= PAGE_MARK_RECORD_HEADER_MAX_INDEX
        || !PAGE_MARK_RECORD_HEADER_FLAGS_HIGH_U16_VALUES.contains(&((flags >> 16) as u16))
        || line_start > line_end
        || line_end >= PAGE_MARK_RECORD_HEADER_MAX_LINE_END
    {
        return None;
    }
    Some(PageMarkRecordHeader {
        offset,
        index,
        flags,
        line_start,
        line_end,
    })
}

/// Builds one alias row per subrecord the selected post-row-gap coverage matched,
/// in the same row order the quantization gate uses.
pub(crate) fn table_grid_source_only_page_mark_absolute_y_slot_record_flag_alias(
    document: &Document,
    subrecord_span_readiness: Option<&TableGridPageMarkSubrecordLineSpanReadiness>,
) -> Option<TableGridSourceOnlyPageMarkAbsoluteYSlotRecordFlagAlias> {
    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let readiness = subrecord_span_readiness?;

    let mut rows = Vec::new();
    for subrecord_byte_offset in readiness
        .selected_post_row_gap_span_coverage
        .matched_candidate_byte_offsets
        .iter()
        .copied()
    {
        let Some(subrecord) =
            page_mark_raw_u16_subrecord_candidate_at(page_mark_bytes, subrecord_byte_offset)
        else {
            continue;
        };
        let field_value = subrecord.words[PAGE_MARK_ABSOLUTE_Y_SLOT_FIELD_INDEX];
        let shifted =
            page_mark_subrecord_shifted_record_header(page_mark_bytes, subrecord_byte_offset);
        let header_shape_valid = shifted.is_some();
        let header = shifted.unwrap_or(PageMarkRecordHeader {
            offset: subrecord_byte_offset
                .saturating_sub(PAGE_MARK_SUBRECORD_RECORD_HEADER_BACK_SHIFT_BYTES),
            index: 0,
            flags: 0,
            line_start: 0,
            line_end: 0,
        });
        rows.push(PageMarkSubrecordRecordFlagAliasRow {
            subrecord_byte_offset,
            shifted_record_header_offset: header.offset,
            shifted_record_index: header.index,
            shifted_record_flags: header.flags,
            flags_high_u16: (header.flags >> 16) as u16,
            flags_low_u16: (header.flags & 0xffff) as u16,
            field_value,
            header_shape_valid,
            line_range_matches_subrecord: header_shape_valid
                && header.line_start == u32::from(subrecord.words[4])
                && header.line_end == u32::from(subrecord.words[6]),
        });
    }
    if rows.is_empty() {
        return None;
    }

    // Structure refutes the direct-px interpretation only when every row is a
    // well-formed shifted record header whose flags low half is the read field.
    let page_space_px_plausible = !rows.iter().all(|row| row.refutes_page_space_px());
    Some(TableGridSourceOnlyPageMarkAbsoluteYSlotRecordFlagAlias {
        back_shift_bytes: PAGE_MARK_SUBRECORD_RECORD_HEADER_BACK_SHIFT_BYTES,
        rows,
        page_space_px_plausible,
    })
}

pub(crate) fn push_table_grid_source_only_page_mark_record_flag_alias_gate_json(
    output: &mut String,
    alias: Option<&TableGridSourceOnlyPageMarkAbsoluteYSlotRecordFlagAlias>,
    blocked_reasons: &[&str],
) {
    let Some(alias) = alias else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/PageMark raw u16 subrecord scan+record header grammar\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"referenceBBoxUsed\":false,\"selectionReady\":false");
    output.push_str(",\"fieldIndex\":");
    output.push_str(&PAGE_MARK_ABSOLUTE_Y_SLOT_FIELD_INDEX.to_string());
    output.push_str(",\"recordHeaderBackShiftBytes\":");
    output.push_str(&alias.back_shift_bytes.to_string());
    output.push_str(",\"aliasRelation\":\"absoluteYSlotField2 == owningRawRecordFlags & 0xffff\"");
    output.push_str(",\"rowCount\":");
    output.push_str(&alias.row_count().to_string());
    output.push_str(",\"subrecordByteOffsets\":");
    push_usize_array_json(
        output,
        &alias
            .rows
            .iter()
            .map(|row| row.subrecord_byte_offset)
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"shiftedRecordHeaderOffsets\":");
    push_usize_array_json(
        output,
        &alias
            .rows
            .iter()
            .map(|row| row.shifted_record_header_offset)
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"shiftedRecordIndexes\":");
    push_u32_array_json(
        output,
        &alias
            .rows
            .iter()
            .map(|row| row.shifted_record_index)
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"shiftedRecordFlagsHex\":");
    push_u32_hex8_array_json(
        output,
        &alias
            .rows
            .iter()
            .map(|row| row.shifted_record_flags)
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"flagsHighU16Values\":");
    push_u16_array_json(
        output,
        &alias
            .rows
            .iter()
            .map(|row| row.flags_high_u16)
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"flagsLowU16Values\":");
    push_u16_array_json(
        output,
        &alias
            .rows
            .iter()
            .map(|row| row.flags_low_u16)
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"fieldValues\":");
    push_u16_array_json(
        output,
        &alias
            .rows
            .iter()
            .map(|row| row.field_value)
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"headerShapeValidCount\":");
    output.push_str(&alias.header_shape_valid_count().to_string());
    output.push_str(",\"allHeaderShapesValid\":");
    output.push_str(json_bool(
        alias.header_shape_valid_count() == alias.row_count(),
    ));
    output.push_str(",\"lineRangeMatchesSubrecordCount\":");
    output.push_str(&alias.line_range_match_count().to_string());
    output.push_str(",\"allLineRangesMatchSubrecord\":");
    output.push_str(json_bool(
        alias.line_range_match_count() == alias.row_count(),
    ));
    output.push_str(",\"fieldEqualsFlagsLowU16Count\":");
    output.push_str(&alias.field_equals_flags_low_u16_count().to_string());
    output.push_str(",\"allFieldsEqualFlagsLowU16\":");
    output.push_str(json_bool(
        alias.field_equals_flags_low_u16_count() == alias.row_count(),
    ));
    output.push_str(",\"structurallyRefutesPageSpacePx\":");
    output.push_str(json_bool(alias.all_rows_refute_page_space_px()));
    output.push_str(",\"pageSpacePxPlausible\":");
    output.push_str(json_bool(alias.page_space_px_plausible));
    output.push_str(",\"flagsSemantics\":\"undecoded-record-flag-half-word\"");
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-only-page-mark-record-flag-alias-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    match blocked_reasons.first() {
        Some(reason) => output.push_str(&json_string(reason)),
        None => output.push_str("null"),
    }
    output.push('}');
}
