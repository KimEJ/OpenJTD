use crate::*;

/// `/PageMark` u16 field preview of the `tsaiten` page record, kept as a literal
/// word vector so the identity search is exercised without a sample file.
const TSAITEN_PAGE_MARK_U16_FIELD_PREVIEW: &[u16] = &[
    0, 0, 1, 0, 0, 0, 0, 42, 0, 0, 564, 0, 0, 564, 194, 423, 223, 564, 564, 370, 255, 564, 0, 0,
];

fn page_mark_record_header_bytes(
    index: u32,
    flags: u32,
    line_start: u32,
    line_end: u32,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    for field in [index, flags, line_start, line_end] {
        bytes.extend_from_slice(&field.to_be_bytes());
    }
    bytes
}

/// A record header preceded by two pad bytes and followed by a zero word, which
/// is the smallest buffer where the two-byte-early subrecord window is complete.
fn page_mark_record_header_stream(
    index: u32,
    flags: u32,
    line_start: u32,
    line_end: u32,
) -> Vec<u8> {
    let mut bytes = vec![0u8, 0u8];
    bytes.extend_from_slice(&page_mark_record_header_bytes(
        index, flags, line_start, line_end,
    ));
    bytes.extend_from_slice(&[0u8; 4]);
    bytes
}

#[test]
fn page_mark_absolute_y_slot_field_is_the_owning_record_flags_low_u16() {
    let bytes = page_mark_record_header_stream(3, 0x0001_0400, 100, 200);
    let header_offset = 2usize;
    let subrecord_offset = header_offset + PAGE_MARK_SUBRECORD_RECORD_HEADER_BACK_SHIFT_BYTES;

    let subrecord = page_mark_raw_u16_subrecord_candidate_at(&bytes, subrecord_offset)
        .expect("shifted record header must satisfy the raw u16 subrecord shape");
    assert_eq!(
        subrecord.words[3], 0,
        "words[3] is the line-start high half"
    );
    assert_eq!(subrecord.words[5], 0, "words[5] is the line-end high half");
    assert_eq!(subrecord.words[4], 100);
    assert_eq!(subrecord.words[6], 200);

    let header = page_mark_subrecord_shifted_record_header(&bytes, subrecord_offset)
        .expect("record header two bytes before the subrecord must parse");
    assert_eq!(header.offset, header_offset);
    assert_eq!(header.index, 3);
    assert_eq!(header.flags, 0x0001_0400);
    assert_eq!(header.line_start, 100);
    assert_eq!(header.line_end, 200);

    let field_value = subrecord.words[PAGE_MARK_ABSOLUTE_Y_SLOT_FIELD_INDEX];
    assert_eq!(
        field_value, 1024,
        "0x0400 is read as 1024 px by the slot probe"
    );
    assert_eq!(
        u32::from(field_value),
        header.flags & 0xffff,
        "absolute-y-slot field 2 is the owning record flags low u16"
    );

    let row = PageMarkSubrecordRecordFlagAliasRow {
        subrecord_byte_offset: subrecord_offset,
        shifted_record_header_offset: header.offset,
        shifted_record_index: header.index,
        shifted_record_flags: header.flags,
        flags_high_u16: (header.flags >> 16) as u16,
        flags_low_u16: (header.flags & 0xffff) as u16,
        field_value,
        header_shape_valid: true,
        line_range_matches_subrecord: header.line_start == u32::from(subrecord.words[4])
            && header.line_end == u32::from(subrecord.words[6]),
    };
    assert!(row.field_equals_flags_low_u16());
    assert!(row.refutes_page_space_px());

    let high_five = page_mark_record_header_stream(2, 0x0005_0300, 85, 140);
    let header = page_mark_subrecord_shifted_record_header(&high_five, 4)
        .expect("observed high-five flags must retain the record-header alias");
    assert_eq!(header.flags, 0x0005_0300);
}

#[test]
fn page_mark_shifted_record_header_rejects_non_record_windows() {
    let flags_high_wrong = page_mark_record_header_stream(3, 0x0002_0400, 100, 200);
    assert!(page_mark_subrecord_shifted_record_header(&flags_high_wrong, 4).is_none());

    let index_out_of_range = page_mark_record_header_stream(4096, 0x0001_0400, 100, 200);
    assert!(page_mark_subrecord_shifted_record_header(&index_out_of_range, 4).is_none());

    let inverted_line_range = page_mark_record_header_stream(3, 0x0001_0400, 200, 100);
    assert!(page_mark_subrecord_shifted_record_header(&inverted_line_range, 4).is_none());

    let line_end_out_of_range = page_mark_record_header_stream(3, 0x0001_0400, 100, 10_000);
    assert!(page_mark_subrecord_shifted_record_header(&line_end_out_of_range, 4).is_none());

    // No room for a header two bytes before the subrecord start.
    assert!(page_mark_subrecord_shifted_record_header(&[0u8; 32], 1).is_none());
}

#[test]
fn page_mark_line_pitch_identity_finds_font_plus_leading() {
    let identity = page_mark_line_pitch_identity_candidate(TSAITEN_PAGE_MARK_U16_FIELD_PREVIEW)
        .expect("repeated pitch word with one addend pair must be found");
    assert_eq!(identity.pitch_mm100, 564);
    assert_eq!(identity.pitch_word_indexes, vec![10, 13, 17, 18, 21]);
    assert_eq!(identity.font_mm100, 370);
    assert_eq!(identity.font_word_index, 19);
    assert_eq!(identity.leading_mm100, 194);
    assert_eq!(identity.leading_word_index, 14);
    assert_eq!(identity.addend_pair_count, 1);
    assert_eq!(
        u32::from(identity.font_mm100) + u32::from(identity.leading_mm100),
        u32::from(identity.pitch_mm100)
    );
}

#[test]
fn page_mark_line_pitch_identity_requires_a_repeated_pitch_word() {
    assert!(page_mark_line_pitch_identity_candidate(&[564, 370, 194]).is_none());
    // Repeated but with no addend pair inside the entry.
    assert!(page_mark_line_pitch_identity_candidate(&[564, 564, 564, 111]).is_none());
    assert!(page_mark_line_pitch_identity_candidate(&[]).is_none());
}

#[test]
fn page_layout_style_margin_quad_candidates_read_raw_be_u16_fields() {
    let mut bytes = Vec::new();
    for field in [3_000u16, 3_000, 2_500, 1_500, 1_000] {
        bytes.extend_from_slice(&field.to_be_bytes());
    }
    let candidates = page_layout_style_margin_quad_candidates(&bytes, 21_000, 29_700, 564, 42);
    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates[0].record_payload_offsets, vec![0, 2, 4, 6]);
    assert_eq!(candidates[0].values_mm100, vec![3_000, 3_000, 2_500, 1_500]);
    assert_eq!(candidates[0].best_page_fit_remainder_mm100, 12);
    assert_eq!(candidates[1].record_payload_offsets, vec![2, 4, 6, 8]);
    assert_eq!(candidates[1].best_page_fit_remainder_mm100, 512);
}

#[test]
fn page_layout_style_margin_quad_candidates_reject_non_mm100_words() {
    let mut bytes = Vec::new();
    for field in [3_001u16, 3_000, 2_500, 1_500] {
        bytes.extend_from_slice(&field.to_be_bytes());
    }
    assert!(page_layout_style_margin_quad_candidates(&bytes, 21_000, 29_700, 564, 42).is_empty());
}

#[test]
fn page_grid_y_anchor_vertical_pairings_cross_check_line_capacity() {
    let pairings = page_grid_y_anchor_vertical_pairings(
        29_700,
        &[3_000, 3_000, 2_500, 1_500],
        Some(564),
        0,
        &[7, 9, 11],
        3,
    );
    assert_eq!(pairings.len(), 2);

    let first = &pairings[0];
    assert_eq!(first.label, "margin-quad-fields-0-1-as-top-bottom");
    assert_eq!(first.body_height_mm100, 23_700);
    assert_eq!(first.line_capacity, 42);
    assert_eq!(first.line_capacity_remainder_mm100, 12);
    assert_eq!(first.page_line_row_tops_px.len(), 3);
    assert_eq!(first.own_row_top_y_px.len(), 3);
    assert!((first.own_row_top_y_px[0] - 262.602).abs() < 0.01);

    let second = &pairings[1];
    assert_eq!(second.label, "margin-quad-fields-2-3-as-top-bottom");
    assert_eq!(second.body_height_mm100, 25_700);
    assert_eq!(second.line_capacity, 45);
    assert_eq!(second.line_capacity_remainder_mm100, 320);

    // The tsaiten page record is lines 0..42 inclusive, so 43 lines / 42 gaps. Only
    // the 0/1 pairing fits, and only against the gap count.
    assert!(!first.matches_page_record_line_count(43));
    assert!(first.matches_page_record_line_gap_count(43));
    assert!(first.fits_page_record(43));
    assert!(!second.fits_page_record(43));
    assert!(first.matches_page_record_line_count(42));

    // Projected page-line tops advance by the mm100 pitch, not by a page-space guess.
    let pitch_px = second.page_line_row_tops_px[1] - second.page_line_row_tops_px[0];
    assert!((pitch_px - 21.317).abs() < 0.01, "pitch px was {pitch_px}");
}

#[test]
fn page_grid_y_anchor_vertical_pairings_tolerate_missing_fields() {
    assert!(
        page_grid_y_anchor_vertical_pairings(29_700, &[3_000, 3_000], Some(564), 0, &[7], 2,).len()
            == 1
    );
    assert!(page_grid_y_anchor_vertical_pairings(29_700, &[], Some(564), 0, &[7], 2).is_empty());
    let no_pitch = page_grid_y_anchor_vertical_pairings(29_700, &[3_000, 3_000], None, 0, &[7], 2);
    assert_eq!(no_pitch[0].line_capacity, 0);
    assert_eq!(
        no_pitch[0].page_line_row_tops_px[1],
        no_pitch[0].body_top_px
    );
}
