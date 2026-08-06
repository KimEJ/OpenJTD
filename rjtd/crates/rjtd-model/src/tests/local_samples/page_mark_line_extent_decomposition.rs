use super::*;
use crate::*;
use std::fs;

fn read_be16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn distinct_count<T: Ord>(values: impl IntoIterator<Item = T>) -> usize {
    values.into_iter().collect::<BTreeSet<_>>().len()
}

/// Corpus files whose normalized `/PageMark` walk carries exactly one flags-high
/// `0x0005` record, with the literals the extent decomposition is built from:
/// `(relativePath, flagsHighFiveScanIndex, firstRecordLineEndPlusOne,
/// lineMarkDeclaredRecordCount, spanAfterFlagsHighFive, lineExtent)`.
///
/// The three equalities the decomposition reports hold in every row: the first
/// record boundary is the `/LineMark` header `be16` at offset `12`, the
/// `0x0005` record boundary is the declared record count minus two, and the
/// extent is the declared record count plus the following record's span minus
/// one. Within this controlled set, only the five `040a`–`040e` rows vary
/// `span_after_flags_high_five`; all other listed rows retain `48`. The file
/// labels are corpus identifiers, not semantic authority for the edited field.
type ExtentDecompositionRow = (&'static str, usize, u32, usize, u32, u32);

const EXTENT_DECOMPOSITION_ROWS: &[ExtentDecompositionRow] = &[
    ("010a_table_after_1_paragraph", 0, 7, 9, 48, 56),
    ("011a_table_after_4_paragraphs", 0, 10, 12, 48, 59),
    ("012a_table_after_8_paragraphs", 0, 14, 16, 48, 63),
    ("030_col1_width_plus", 0, 6, 8, 48, 55),
    ("031_col2_width_plus", 0, 6, 8, 48, 55),
    ("032_table_width_plus_both_cols", 0, 6, 8, 48, 55),
    ("040a_top_margin_20mm", 0, 6, 8, 51, 58),
    ("040b_top_margin_30mm_baseline", 0, 6, 8, 48, 55),
    ("040c_top_margin_40mm", 0, 6, 8, 46, 53),
    ("040d_top_margin_50mm", 0, 6, 8, 44, 51),
    ("040e_top_margin_60mm", 0, 6, 8, 42, 49),
    ("053_font_size_table_plus", 0, 6, 8, 48, 55),
    ("054_font_size_paragraph_plus", 0, 4, 6, 48, 53),
    ("055_table_cell_line_spacing_plus", 0, 6, 8, 48, 55),
    ("056_paragraph_line_spacing_plus", 0, 6, 8, 48, 55),
    ("060_table_3x3", 0, 6, 8, 48, 55),
    ("061_table_2x5", 0, 5, 7, 48, 54),
    ("062_table_1x3", 0, 4, 6, 48, 53),
    ("063_table_4x2", 0, 7, 9, 48, 56),
    ("064_merged_header", 0, 6, 8, 48, 55),
    ("066_empty_cells", 0, 6, 8, 48, 55),
    (
        "074a_many_paragraphs_then_small_table_page2",
        1,
        3,
        10,
        48,
        57,
    ),
    ("074b_table_near_page_bottom_no_split", 1, 3, 30, 48, 77),
    ("074c_table_crosses_page_boundary", 1, 3, 43, 48, 90),
    ("074d_many_row_table_2col_simple", 1, 3, 37, 48, 84),
    ("081_plain_paragraph_line_spacing_plus", 0, 14, 16, 48, 63),
    ("082_plain_paragraph_font_size_plus", 0, 15, 17, 48, 64),
];

/// The counter-direction: corpus files with no flags-high `0x0005` record, as
/// `(relativePath, lineMarkDeclaredRecordCount, secondToLastRecordSpan,
/// lineExtent)`. The gate returns nothing for all of them, and the extent sum
/// that is exact above misses in every one. The four `scratch/misc-editor-saves`
/// rows are included because an individual raw boundary coincides with a
/// `/LineMark` header value there, demonstrating why the gate rather than an
/// isolated equality carries the claim.
const NO_FLAGS_HIGH_FIVE_ROWS: &[(&str, usize, u32, u32)] = &[
    ("corpus/baseline-sweep/000_base_a", 9, 40, 81),
    ("corpus/baseline-sweep/001_base_b_resave", 9, 40, 81),
    (
        "corpus/baseline-sweep/010_table_moved_down_small",
        10,
        40,
        81,
    ),
    (
        "corpus/baseline-sweep/011_table_moved_down_large",
        17,
        40,
        81,
    ),
    ("corpus/baseline-sweep/013_table_moved_right", 9, 40, 81),
    ("corpus/baseline-sweep/020_row1_height_plus", 11, 40, 81),
    ("corpus/baseline-sweep/021_row2_height_plus", 11, 40, 81),
    ("corpus/baseline-sweep/022_row3_height_plus", 11, 40, 81),
    ("corpus/baseline-sweep/040_top_margin_plus", 9, 36, 73),
    ("corpus/baseline-sweep/050_wrapped_one_cell", 14, 40, 81),
    ("corpus/baseline-sweep/070_two_tables_vertical", 20, 40, 81),
    (
        "corpus/baseline-sweep/080_plain_paragraph_lines_only",
        13,
        40,
        81,
    ),
    ("corpus/page01-grid/PAGE 01", 8, 40, 81),
    ("corpus/page01-grid/PAGE 01_DOWNTEST_1LINE", 9, 40, 81),
    ("corpus/page01-grid/PAGE 01_DOWNTEST_BASE", 8, 40, 81),
    ("corpus/page01-grid/PAGE 01_down_1Low", 9, 40, 81),
    ("corpus/page01-grid/PAGE 01_down_2Low", 10, 40, 81),
    ("corpus/page01-grid/PAGE 01_down_3Low", 11, 40, 81),
    ("corpus/page01-grid/PAGE 01_down_4Low", 12, 40, 81),
    ("corpus/page01-grid/PAGE 01_right_1Tick", 8, 40, 81),
    ("corpus/page01-grid/PAGE 01_right_2Tick", 8, 40, 81),
    ("corpus/page01-grid/PAGE 01_right_3Tick", 8, 40, 81),
    ("corpus/page01-grid/PAGE 01_right_4Tick", 8, 40, 81),
    ("scratch/misc-editor-saves/asdf", 41, 40, 121),
    ("scratch/misc-editor-saves/zxcv", 41, 40, 121),
    ("scratch/misc-editor-saves/○○○", 15, 17, 33),
    ("scratch/misc-editor-saves/来訪メモ", 68, 2, 139),
];

fn read_page_mark_and_line_mark(relative_path: &str) -> Option<(Vec<u8>, Vec<u8>)> {
    let sample_bytes = fs::read(local_samples_dir().join(relative_path)).ok()?;
    let document = parse_document(&sample_bytes).unwrap();
    let page_mark = raw_stream_bytes(&document, PAGE_MARK_PATH)
        .unwrap_or_else(|| panic!("{relative_path} must expose /PageMark"))
        .to_vec();
    let line_mark = raw_stream_bytes(&document, LINE_MARK_PATH)
        .unwrap_or_else(|| panic!("{relative_path} must expose /LineMark"))
        .to_vec();
    Some((page_mark, line_mark))
}

/// The `/PageMark` raw line extent is not one opaque number: on every corpus
/// stream that carries exactly one flags-high `0x0005` record it is the
/// `/LineMark` declared record count plus the span of the record following that
/// one, minus one. Two literal boundary equalities against `/LineMark` header
/// fields come with it. The gate does the work: all 27 ungated corpus streams
/// reject the extent sum, even though individual boundary coincidences occur.
#[test]
fn local_page_mark_raw_line_extent_decomposes_against_line_mark_header_fields() {
    let mut checked = 0usize;
    for (
        sample_name,
        expected_scan_index,
        expected_first,
        expected_count,
        expected_span,
        expected_extent,
    ) in EXTENT_DECOMPOSITION_ROWS
    {
        let relative_path =
            format!("ichitaro-source-y-probe/corpus/baseline-sweep/{sample_name}.jtd");
        let Some((page_mark, line_mark)) = read_page_mark_and_line_mark(&relative_path) else {
            continue;
        };
        let headers = page_mark_normalized_record_headers(&page_mark);
        let decomposition = page_mark_raw_line_extent_line_mark_decomposition(&headers, &line_mark)
            .unwrap_or_else(|| panic!("{sample_name} must expose exactly one 0x0005 record"));
        assert_eq!(
            (
                decomposition.flags_high_five_scan_index,
                decomposition.first_record_line_end_plus_one,
                decomposition.line_mark_declared_record_count,
                decomposition.span_after_flags_high_five,
                decomposition.line_extent,
            ),
            (
                *expected_scan_index,
                *expected_first,
                *expected_count,
                *expected_span,
                *expected_extent
            ),
            "{sample_name} extent decomposition literals"
        );
        assert!(
            decomposition.all_equalities_hold(),
            "{sample_name} extent decomposition equalities: {decomposition:?}"
        );
        checked += 1;
    }

    for (sample_name, expected_count, expected_span, expected_extent) in NO_FLAGS_HIGH_FIVE_ROWS {
        let relative_path = format!("ichitaro-source-y-probe/{sample_name}.jtd");
        let Some((page_mark, line_mark)) = read_page_mark_and_line_mark(&relative_path) else {
            continue;
        };
        let headers = page_mark_normalized_record_headers(&page_mark);
        assert!(
            page_mark_raw_line_extent_line_mark_decomposition(&headers, &line_mark).is_none(),
            "{sample_name} must not pass the flags-high 0x0005 gate"
        );
        let second_to_last = headers[headers.len() - 2];
        let extent = headers
            .iter()
            .map(|header| header.line_end + 1)
            .max()
            .unwrap();
        assert_eq!(
            (
                line_mark_declared_record_count(&line_mark),
                second_to_last.line_end - second_to_last.line_start + 1,
                extent,
            ),
            (Some(*expected_count), *expected_span, *expected_extent),
            "{sample_name} ungated literals"
        );
        assert_ne!(
            i64::from(extent),
            *expected_count as i64 + i64::from(*expected_span) - 1,
            "{sample_name} would be the first ungated corpus stream where the gated extent sum \
             also holds"
        );
        checked += 1;
    }

    if checked > 0 {
        assert_eq!(
            checked,
            EXTENT_DECOMPOSITION_ROWS.len() + NO_FLAGS_HIGH_FIVE_ROWS.len(),
            "the corpus must expose every listed sample or none of them"
        );
    }
}

/// `/LineMark` be-delta record words for four controlled corpus rows, as
/// `(relativeSampleName, [word per record index])`.
const BE_DELTA_CONTROL_LINE_MARK_WORDS: &[(&str, &[u16])] = &[
    (
        "040b_top_margin_30mm_baseline",
        &[13, 51, 81, 81, 81, 13, 1, 65_214],
    ),
    (
        "053_font_size_table_plus",
        &[13, 51, 81, 81, 81, 13, 1, 65_214],
    ),
    (
        "055_table_cell_line_spacing_plus",
        &[13, 51, 87, 87, 87, 13, 1, 65_196],
    ),
    (
        "056_paragraph_line_spacing_plus",
        &[13, 57, 87, 87, 87, 13, 1, 65_190],
    ),
];

const BE_DELTA_CONTROL_LINE_RANGES: &[(u32, u32)] = &[(0, 5), (6, 53), (54, 54)];

fn line_mark_be_delta_words(line_mark: &[u8], count: usize) -> Vec<u16> {
    (0..count)
        .map(|record_index| {
            read_be16(
                line_mark,
                LINE_MARK_BE_DELTA_HEADER_BYTES + record_index * 4,
            )
        })
        .collect()
}

/// The changing `/LineMark` be-delta words do not drive the raw line domain on
/// their own. Relative to rows `040b` and `053`, row `055` changes record words
/// 2..=4 from `81` to `87`, while row `056` also changes record word 1 from `51`
/// to `57`. The declared count, the `be16` at offset `12`, and all three raw
/// `/PageMark` ranges remain identical. Rows `040b` and `053` have byte-identical
/// `/LineMark` streams. The corpus labels do not decode what caused the changes.
#[test]
fn local_line_mark_be_delta_words_vary_without_moving_the_page_mark_line_domain() {
    let samples = BE_DELTA_CONTROL_LINE_MARK_WORDS
        .iter()
        .filter_map(|(sample_name, expected_deltas)| {
            let relative_path =
                format!("ichitaro-source-y-probe/corpus/baseline-sweep/{sample_name}.jtd");
            read_page_mark_and_line_mark(&relative_path)
                .map(|streams| (sample_name, expected_deltas, streams))
        })
        .collect::<Vec<_>>();
    if samples.len() < BE_DELTA_CONTROL_LINE_MARK_WORDS.len() {
        return;
    }

    for (sample_name, expected_deltas, (page_mark, line_mark)) in &samples {
        let count = line_mark_declared_record_count(line_mark);
        assert_eq!(
            count,
            Some(expected_deltas.len()),
            "{sample_name} /LineMark declared record count"
        );
        assert_eq!(
            line_mark_be_delta_words(line_mark, expected_deltas.len()),
            expected_deltas.to_vec(),
            "{sample_name} /LineMark be-delta words"
        );
        assert_eq!(
            read_be16(line_mark, LINE_MARK_BE_DELTA_HEADER_U16_12_OFFSET),
            6,
            "{sample_name} /LineMark be16 at offset 12"
        );
        assert_eq!(
            page_mark_normalized_record_headers(page_mark)
                .iter()
                .map(|header| (header.line_start, header.line_end))
                .collect::<Vec<_>>(),
            BE_DELTA_CONTROL_LINE_RANGES.to_vec(),
            "{sample_name} raw /PageMark line ranges"
        );
    }

    assert_eq!(
        distinct_count(samples.iter().map(|(_, _, (_, line_mark))| line_mark)),
        3,
        "the four controls must expose three distinct /LineMark streams"
    );
    assert_eq!(
        samples[0].2.1, samples[1].2.1,
        "controlled rows 040b and 053 must leave /LineMark byte-identical"
    );
}
