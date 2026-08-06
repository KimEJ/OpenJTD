use super::*;
use crate::*;
use std::fs;

/// Local `/PageMark` streams where admitting flags-high `0x0003` completes an
/// alternating record chain. Recorded as literal `(byteOffset, index, flags,
/// lineStart, lineEnd)` tuples, because the two files disagree about stride: only
/// `kibasen` is on core's 84-byte entry grid.
type PartitionRecord = (usize, u32, u32, u32, u32);

const KIBASEN_SAMPLE: &str =
    "justsystems-20090825102932-jp-school-academy-report-files-kibasen.jtd";

/// `kibasen` records sit at `12 + k * 84`, which is exactly where core's
/// `fixed84` entry walk puts its six entries.
const KIBASEN_RECORDS: &[PartitionRecord] = &[
    (12, 0, 0x0001_0000, 0, 9),
    (96, 0, 0x0003_0000, 10, 19),
    (180, 1, 0x0001_0000, 20, 29),
    (264, 1, 0x0003_0000, 30, 39),
    (348, 2, 0x0001_0000, 40, 41),
    (432, 2, 0x0003_0000, 42, 42),
];

/// `raihoumemo01` carries the same alternating chain on a variable stride
/// (80, 80, 48, 16, 16), so it is the counter-case to any fixed-grid reading.
const RAIHOUMEMO_RECORDS: &[PartitionRecord] = &[
    (12, 0, 0x0001_0000, 0, 32),
    (92, 0, 0x0003_0000, 33, 65),
    (172, 1, 0x0001_0000, 66, 100),
    (220, 1, 0x0003_0000, 101, 135),
    (236, 2, 0x0001_0000, 136, 137),
    (252, 2, 0x0003_0000, 138, 138),
];

const FLAGS_HIGH_THREE_PARTITION_SAMPLES: &[(&str, &[PartitionRecord])] = &[
    (KIBASEN_SAMPLE, KIBASEN_RECORDS),
    ("raihoumemo01.jtt", RAIHOUMEMO_RECORDS),
];

/// Observed `/LineMark` declared record count minus `/PageMark` raw line extent,
/// per sample. The survey behind these numbers read every local `.jtd`/`.jtt`
/// sample exposing both a normalized `/PageMark` record and a `/LineMark`
/// declared record count: 50 files, none of them `0`, 35 negative and 15 exactly
/// `1`. A single reusable offset between the two counts therefore does not exist.
const LINE_DOMAIN_SIZE_DELTAS: &[(&str, i64)] = &[
    // The 15 positive files are all exactly `1`. That near miss is why the raw
    // comparison is reported instead of being reduced to the refutation bool.
    ("justsystems-20120223023549-jp-just-finance-j200003.jtd", 1),
    ("justsystems-20120223023906-jp-just-finance-j200003c.jtd", 1),
    (
        "ichitaro-20030422210439-success-002-success_data-natsu.jtd",
        1,
    ),
    // Negative files are the refuting ones: the raw line field reaches values with
    // no matching record ordinal, by one record and by hundreds.
    (KIBASEN_SAMPLE, -1),
    ("b6.jtd", -14),
    ("46.jtd", -43),
    ("raihoumemo01.jtt", -106),
    (
        "ichitaro-20030228030923-success-002-success_data-test.jtd",
        -202,
    ),
];

/// `page01-grid` keeps one page setup and only moves the body: `DOWNTEST` and
/// `down_*Low` push it down by whole lines, `right_*Tick` pushes it sideways.
const CONTENT_ONLY_EDIT_SERIES: &[&str] = &[
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_DOWNTEST_1LINE.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_DOWNTEST_BASE.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_down_1Low.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_down_2Low.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_down_3Low.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_down_4Low.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_right_1Tick.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_right_2Tick.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_right_3Tick.jtd",
    "ichitaro-source-y-probe/corpus/page01-grid/PAGE 01_right_4Tick.jtd",
];

const CONTENT_ONLY_EDIT_LINE_RANGES: &[(u32, u32)] = &[(0, 39), (40, 79), (80, 80)];

/// `baseline-sweep` top-margin series: same body, five different top margins.
const PAGE_GEOMETRY_ONLY_EDIT_SERIES: &[(&str, &[(u32, u32)])] = &[
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/040a_top_margin_20mm.jtd",
        &[(0, 5), (6, 56), (57, 57)],
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/040b_top_margin_30mm_baseline.jtd",
        &[(0, 5), (6, 53), (54, 54)],
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/040c_top_margin_40mm.jtd",
        &[(0, 5), (6, 51), (52, 52)],
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/040d_top_margin_50mm.jtd",
        &[(0, 5), (6, 49), (50, 50)],
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/040e_top_margin_60mm.jtd",
        &[(0, 5), (6, 47), (48, 48)],
    ),
];

/// The first three top-margin files also share one byte-identical
/// `/PageLayoutStyle`, so that stream cannot carry the line domain either.
const PAGE_GEOMETRY_SHARED_PAGE_LAYOUT_STYLE_PREFIX: usize = 3;

struct SeriesSample {
    line_ranges: Vec<(u32, u32)>,
    document_text: Vec<u8>,
    line_mark: Vec<u8>,
    page_layout_style: Option<Vec<u8>>,
}

fn read_series_sample(relative_path: &str) -> Option<SeriesSample> {
    let sample_bytes = fs::read(local_samples_dir().join(relative_path)).ok()?;
    let document = parse_document(&sample_bytes).unwrap();
    let page_mark = raw_stream_bytes(&document, PAGE_MARK_PATH)
        .unwrap_or_else(|| panic!("{relative_path} must expose /PageMark"));
    let stream = |name: &str| {
        raw_stream_bytes(&document, name)
            .unwrap_or_else(|| panic!("{relative_path} must expose {name}"))
            .to_vec()
    };
    Some(SeriesSample {
        line_ranges: page_mark_normalized_record_headers(page_mark)
            .iter()
            .map(|header| (header.line_start, header.line_end))
            .collect(),
        document_text: stream("/DocumentText"),
        line_mark: stream(LINE_MARK_PATH),
        page_layout_style: document
            .unknown_styles()
            .iter()
            .find(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
            .map(|style| style.payload().to_vec()),
    })
}

fn distinct_count<T: Ord>(values: impl IntoIterator<Item = T>) -> usize {
    values.into_iter().collect::<BTreeSet<_>>().len()
}

/// Moving the body without touching the page setup rewrites `/DocumentText` and
/// `/LineMark` but leaves the raw `/PageMark` line tuples alone, so the raw line
/// fields cannot be an ordinal into either stream's units or records.
#[test]
fn local_page_mark_raw_line_domain_is_invariant_while_content_streams_change() {
    let samples = CONTENT_ONLY_EDIT_SERIES
        .iter()
        .filter_map(|relative_path| read_series_sample(relative_path))
        .collect::<Vec<_>>();
    if samples.len() < CONTENT_ONLY_EDIT_SERIES.len() {
        return;
    }

    for (relative_path, sample) in CONTENT_ONLY_EDIT_SERIES.iter().zip(&samples) {
        assert_eq!(
            sample.line_ranges,
            CONTENT_ONLY_EDIT_LINE_RANGES.to_vec(),
            "{relative_path} raw /PageMark line ranges"
        );
    }
    assert_eq!(
        distinct_count(samples.iter().map(|sample| &sample.document_text)),
        CONTENT_ONLY_EDIT_SERIES.len(),
        "every sample in the series must carry a different /DocumentText"
    );
    assert_eq!(
        distinct_count(samples.iter().map(|sample| &sample.line_mark)),
        8,
        "the series must carry eight distinct /LineMark streams"
    );
}

/// The mirror case: `/DocumentText` and `/LineMark` are byte-identical across the
/// top-margin series while every raw `/PageMark` line domain differs, and the
/// first three files share `/PageLayoutStyle` as well.
#[test]
fn local_page_mark_raw_line_domain_changes_while_content_streams_are_byte_identical() {
    let samples = PAGE_GEOMETRY_ONLY_EDIT_SERIES
        .iter()
        .filter_map(|(relative_path, _)| read_series_sample(relative_path))
        .collect::<Vec<_>>();
    if samples.len() < PAGE_GEOMETRY_ONLY_EDIT_SERIES.len() {
        return;
    }

    for ((relative_path, expected_ranges), sample) in
        PAGE_GEOMETRY_ONLY_EDIT_SERIES.iter().zip(&samples)
    {
        assert_eq!(
            sample.line_ranges,
            expected_ranges.to_vec(),
            "{relative_path} raw /PageMark line ranges"
        );
    }
    assert_eq!(
        distinct_count(samples.iter().map(|sample| &sample.line_ranges)),
        PAGE_GEOMETRY_ONLY_EDIT_SERIES.len(),
        "the raw line domain must differ in every file of the series"
    );
    assert_eq!(
        distinct_count(samples.iter().map(|sample| &sample.document_text)),
        1,
        "/DocumentText must stay byte-identical across the series"
    );
    assert_eq!(
        distinct_count(samples.iter().map(|sample| &sample.line_mark)),
        1,
        "/LineMark must stay byte-identical across the series"
    );
    assert_eq!(
        distinct_count(
            samples[..PAGE_GEOMETRY_SHARED_PAGE_LAYOUT_STYLE_PREFIX]
                .iter()
                .map(|sample| &sample.page_layout_style)
        ),
        1,
        "the first three files must share one /PageLayoutStyle while their line domains differ"
    );
    assert!(
        samples[0].page_layout_style.is_some(),
        "the shared /PageLayoutStyle must be present rather than absent in all three"
    );
}

#[test]
fn local_page_mark_raw_line_extent_never_equals_the_line_mark_declared_record_count() {
    for (sample_name, expected_delta) in LINE_DOMAIN_SIZE_DELTAS {
        let sample_path = local_samples_dir().join(sample_name);
        let Ok(sample_bytes) = fs::read(&sample_path) else {
            continue;
        };
        let document = parse_document(&sample_bytes).unwrap();
        let page_mark = raw_stream_bytes(&document, PAGE_MARK_PATH)
            .unwrap_or_else(|| panic!("{sample_name} must expose /PageMark"));
        let line_mark = raw_stream_bytes(&document, LINE_MARK_PATH)
            .unwrap_or_else(|| panic!("{sample_name} must expose /LineMark"));

        let headers = page_mark_normalized_record_headers(page_mark);
        let delta =
            line_mark_declared_record_count_minus_page_mark_line_extent(&headers, line_mark);
        assert_eq!(
            delta,
            Some(*expected_delta),
            "{sample_name} /LineMark declared record count minus /PageMark raw line extent"
        );
        assert_ne!(
            delta,
            Some(0),
            "{sample_name} would be the first local sample where the two domains are the same size"
        );
    }
}

/// `/DocumentEditStyles` byte offsets that differ anywhere across the top-margin
/// series, and the `be32` reading of the field containing them. The values are
/// not ordered by the controlled edit, so it is not a direct monotonic vertical
/// metric and cannot directly explain the line domain.
const PAGE_GEOMETRY_EDIT_STYLE_DIFFERING_OFFSETS: &[usize] = &[0x001f];
const PAGE_GEOMETRY_EDIT_STYLE_FIELD_OFFSET: usize = 0x001c;
const PAGE_GEOMETRY_EDIT_STYLE_FIELD_VALUES: &[u32] =
    &[265_912, 265_900, 265_895, 265_897, 265_926];

/// `/DocumentViewStyles` byte offsets that differ across the same series. Only
/// two 2-byte fields move: one inside record `0x1002` and one in the trailing
/// area past the end of the sequential record chain.
const PAGE_GEOMETRY_VIEW_STYLE_DIFFERING_OFFSETS: &[usize] = &[0x0011, 0x0012, 0x0401, 0x0402];

/// Record `0x1002` of `/DocumentViewStyles`: `be16` quadruple at payload offsets
/// 3, 5, 7 and 9. Only the first entry moves across the series, in steps of
/// `1000` per file, while the other three stay at `3000`. The entry is a
/// source-backed page field that tracks the controlled edit in steps of `1000`,
/// `1000`, `1000`, and `1001`; what it measures stays undecoded.
const PAGE_GEOMETRY_VIEW_STYLE_RECORD_CODE: u16 = 0x1002;
const PAGE_GEOMETRY_VIEW_STYLE_RECORD_OFFSET: usize = 10;
const PAGE_GEOMETRY_VIEW_STYLE_RECORD_PAYLOAD_LEN: usize = 33;
const PAGE_GEOMETRY_VIEW_STYLE_QUAD_PAYLOAD_OFFSETS: &[usize] = &[3, 5, 7, 9];
const PAGE_GEOMETRY_VIEW_STYLE_MOVING_QUAD_VALUES: &[u16] = &[2000, 3000, 4000, 5000, 6001];
const PAGE_GEOMETRY_VIEW_STYLE_FIXED_QUAD_VALUE: u16 = 3000;

/// The second moving `/DocumentViewStyles` field, read as `be32` at `0x03ff`.
/// It decreases monotonically, but its steps are not proportional to the line
/// domain: two series steps share the same line-extent delta and disagree on
/// this field, so no affine map from the line domain to it exists.
const PAGE_GEOMETRY_VIEW_STYLE_TRAILING_FIELD_OFFSET: usize = 0x03ff;
const PAGE_GEOMETRY_VIEW_STYLE_TRAILING_FIELD_VALUES: &[u32] =
    &[48_801, 47_812, 46_797, 45_808, 44_792];

/// Local files carrying a `/DocumentViewStyles` record `0x1002` payload that is
/// byte-identical to the `040b` baseline, with their `/PageMark` raw line
/// extents. One source record, six different line domains.
const SHARED_VIEW_STYLE_RECORD_1002_SAMPLES: &[(&str, u32)] = &[
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/040b_top_margin_30mm_baseline.jtd",
        55,
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/010a_table_after_1_paragraph.jtd",
        56,
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/054_font_size_paragraph_plus.jtd",
        53,
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/074c_table_crosses_page_boundary.jtd",
        90,
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/081_plain_paragraph_line_spacing_plus.jtd",
        63,
    ),
    (
        "ichitaro-source-y-probe/corpus/baseline-sweep/082_plain_paragraph_font_size_plus.jtd",
        64,
    ),
];

fn read_style_stream(relative_path: &str, name: &str) -> Option<Vec<u8>> {
    let sample_bytes = fs::read(local_samples_dir().join(relative_path)).ok()?;
    let document = parse_document(&sample_bytes).unwrap();
    document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(name))
        .map(|style| style.payload().to_vec())
}

fn read_page_mark_line_extent(relative_path: &str) -> Option<u32> {
    let sample_bytes = fs::read(local_samples_dir().join(relative_path)).ok()?;
    let document = parse_document(&sample_bytes).unwrap();
    let page_mark = raw_stream_bytes(&document, PAGE_MARK_PATH)
        .unwrap_or_else(|| panic!("{relative_path} must expose /PageMark"));
    page_mark_normalized_record_headers(page_mark)
        .iter()
        .map(|header| header.line_end + 1)
        .max()
}

fn document_view_styles_record_1002_payload(bytes: &[u8]) -> Vec<u8> {
    let summary = summarize_style_stream(bytes);
    let record = summary
        .records()
        .iter()
        .find(|record| record.code() == PAGE_GEOMETRY_VIEW_STYLE_RECORD_CODE)
        .expect("/DocumentViewStyles must expose record 0x1002");
    assert_eq!(record.offset(), PAGE_GEOMETRY_VIEW_STYLE_RECORD_OFFSET);
    assert_eq!(
        record.payload_len(),
        PAGE_GEOMETRY_VIEW_STYLE_RECORD_PAYLOAD_LEN
    );
    let payload_start = record.offset() + 4;
    bytes[payload_start..payload_start + record.payload_len()].to_vec()
}

fn differing_byte_offsets(blobs: &[Vec<u8>]) -> Vec<usize> {
    let len = blobs.first().map(Vec::len).unwrap_or(0);
    assert!(
        blobs.iter().all(|blob| blob.len() == len),
        "the series must expose one stream length"
    );
    (0..len)
        .filter(|offset| blobs.iter().any(|blob| blob[*offset] != blobs[0][*offset]))
        .collect()
}

fn read_be16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn read_be32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

/// Of the two style streams that survived the earlier five-distinct-contents
/// survey, only `/DocumentViewStyles` carries a field that moves monotonically
/// with the controlled page-geometry edit. The single `/DocumentEditStyles` byte
/// that changes is not ordered by the edit, and the second `/DocumentViewStyles`
/// field admits no affine map from the line domain. The monotonic field is then
/// tested below against documents that share it but have different line domains.
#[test]
fn local_page_geometry_series_style_streams_expose_one_monotonic_source_field() {
    let Some(edit_styles) = PAGE_GEOMETRY_ONLY_EDIT_SERIES
        .iter()
        .map(|(relative_path, _)| {
            read_style_stream(
                relative_path,
                rjtd_core::style_stream::DOCUMENT_EDIT_STYLES_PATH,
            )
        })
        .collect::<Option<Vec<_>>>()
    else {
        return;
    };
    let view_styles = PAGE_GEOMETRY_ONLY_EDIT_SERIES
        .iter()
        .map(|(relative_path, _)| read_style_stream(relative_path, DOCUMENT_VIEW_STYLES_PATH))
        .collect::<Option<Vec<_>>>()
        .expect("every file with /DocumentEditStyles must expose /DocumentViewStyles");

    assert_eq!(
        differing_byte_offsets(&edit_styles),
        PAGE_GEOMETRY_EDIT_STYLE_DIFFERING_OFFSETS.to_vec(),
        "/DocumentEditStyles differing byte offsets across the series"
    );
    let edit_field_values = edit_styles
        .iter()
        .map(|blob| read_be32(blob, PAGE_GEOMETRY_EDIT_STYLE_FIELD_OFFSET))
        .collect::<Vec<_>>();
    assert_eq!(
        edit_field_values,
        PAGE_GEOMETRY_EDIT_STYLE_FIELD_VALUES.to_vec(),
        "/DocumentEditStyles be32 field containing the only changing byte"
    );
    assert!(
        !edit_field_values.is_sorted() && !edit_field_values.iter().rev().is_sorted(),
        "the /DocumentEditStyles field must not be ordered by the controlled edit"
    );

    assert_eq!(
        differing_byte_offsets(&view_styles),
        PAGE_GEOMETRY_VIEW_STYLE_DIFFERING_OFFSETS.to_vec(),
        "/DocumentViewStyles differing byte offsets across the series"
    );
    let payloads = view_styles
        .iter()
        .map(|blob| document_view_styles_record_1002_payload(blob))
        .collect::<Vec<_>>();
    assert_eq!(
        differing_byte_offsets(&payloads),
        vec![
            PAGE_GEOMETRY_VIEW_STYLE_QUAD_PAYLOAD_OFFSETS[0],
            PAGE_GEOMETRY_VIEW_STYLE_QUAD_PAYLOAD_OFFSETS[0] + 1
        ],
        "only the first record 0x1002 quadruple entry may move"
    );
    assert_eq!(
        payloads
            .iter()
            .map(|payload| read_be16(payload, PAGE_GEOMETRY_VIEW_STYLE_QUAD_PAYLOAD_OFFSETS[0]))
            .collect::<Vec<_>>(),
        PAGE_GEOMETRY_VIEW_STYLE_MOVING_QUAD_VALUES.to_vec(),
        "record 0x1002 moving quadruple entry"
    );
    for payload in &payloads {
        for offset in &PAGE_GEOMETRY_VIEW_STYLE_QUAD_PAYLOAD_OFFSETS[1..] {
            assert_eq!(
                read_be16(payload, *offset),
                PAGE_GEOMETRY_VIEW_STYLE_FIXED_QUAD_VALUE,
                "record 0x1002 quadruple entry at payload offset {offset}"
            );
        }
    }

    let trailing_values = view_styles
        .iter()
        .map(|blob| read_be32(blob, PAGE_GEOMETRY_VIEW_STYLE_TRAILING_FIELD_OFFSET))
        .collect::<Vec<_>>();
    assert_eq!(
        trailing_values,
        PAGE_GEOMETRY_VIEW_STYLE_TRAILING_FIELD_VALUES.to_vec(),
        "/DocumentViewStyles trailing moving field"
    );
    let line_extents = PAGE_GEOMETRY_ONLY_EDIT_SERIES
        .iter()
        .map(|(_, ranges)| ranges.iter().map(|(_, end)| end + 1).max().unwrap())
        .collect::<Vec<_>>();
    let steps = (1..line_extents.len())
        .map(|index| {
            (
                i64::from(line_extents[index]) - i64::from(line_extents[index - 1]),
                i64::from(trailing_values[index]) - i64::from(trailing_values[index - 1]),
            )
        })
        .collect::<Vec<_>>();
    assert!(
        steps.iter().any(|left| steps
            .iter()
            .any(|right| left.0 == right.0 && left.1 != right.1)),
        "two series steps must share a line-extent delta and disagree on the trailing field, \
         which rules out any affine map from the line domain: {steps:?}"
    );
}

/// The counter-direction for the one moving source field: the whole record
/// `0x1002` payload is byte-identical in six local files whose `/PageMark` raw
/// line extents are all different. A source field that does not change while
/// the line domain does cannot produce that domain on its own.
#[test]
fn local_page_mark_line_domain_is_not_a_function_of_view_style_record_1002() {
    let Some(payloads) = SHARED_VIEW_STYLE_RECORD_1002_SAMPLES
        .iter()
        .map(|(relative_path, _)| {
            read_style_stream(relative_path, DOCUMENT_VIEW_STYLES_PATH)
                .map(|blob| document_view_styles_record_1002_payload(&blob))
        })
        .collect::<Option<Vec<_>>>()
    else {
        return;
    };
    assert_eq!(
        distinct_count(&payloads),
        1,
        "every sample must share one byte-identical record 0x1002 payload"
    );

    let extents = SHARED_VIEW_STYLE_RECORD_1002_SAMPLES
        .iter()
        .map(|(relative_path, _)| {
            read_page_mark_line_extent(relative_path)
                .unwrap_or_else(|| panic!("{relative_path} must expose a normalized /PageMark"))
        })
        .collect::<Vec<_>>();
    assert_eq!(
        extents,
        SHARED_VIEW_STYLE_RECORD_1002_SAMPLES
            .iter()
            .map(|(_, extent)| *extent)
            .collect::<Vec<_>>(),
        "raw /PageMark line extents behind one shared source record"
    );
    assert_eq!(
        distinct_count(&extents),
        SHARED_VIEW_STYLE_RECORD_1002_SAMPLES.len(),
        "the shared source record must stand behind one distinct line extent per sample"
    );
}

#[test]
fn local_page_mark_flags_high_three_records_complete_an_alternating_chain() {
    for (sample_name, expected_records) in FLAGS_HIGH_THREE_PARTITION_SAMPLES {
        let sample_path = local_samples_dir().join(sample_name);
        let Ok(sample_bytes) = fs::read(&sample_path) else {
            continue;
        };
        let document = parse_document(&sample_bytes).unwrap();
        let bytes = raw_stream_bytes(&document, PAGE_MARK_PATH)
            .unwrap_or_else(|| panic!("{sample_name} must expose /PageMark"));

        let headers = page_mark_normalized_record_headers(bytes);
        assert_eq!(
            headers
                .iter()
                .map(|header| (
                    header.offset,
                    header.index,
                    header.flags,
                    header.line_start,
                    header.line_end
                ))
                .collect::<Vec<_>>(),
            expected_records.to_vec(),
            "{sample_name} normalized record offsets and tuples"
        );
        assert!(
            headers
                .windows(2)
                .all(|pair| pair[1].line_start == pair[0].line_end + 1),
            "{sample_name} raw line ranges must chain without a gap or an overlap"
        );

        // The legacy exact-flag scan that separators and rendering read is unchanged.
        let legacy = page_mark_record_headers(bytes);
        assert_eq!(legacy.len(), 3, "{sample_name} legacy record header count");
        assert!(legacy.iter().all(|header| header.flags == 0x0001_0000));

        // Only `kibasen` coincides with core's parsed entry grid, so only there can
        // the admitted records be cross-checked against the entry walk.
        if *sample_name == KIBASEN_SAMPLE {
            let page_mark = document
                .page_marks()
                .first()
                .expect("kibasen must expose a parsed /PageMark");
            assert_eq!(page_mark.family(), "fixed84");
            assert_eq!(
                page_mark
                    .entries()
                    .iter()
                    .map(|entry| (
                        entry.index(),
                        entry.flags(),
                        entry.line_start(),
                        entry.line_end()
                    ))
                    .collect::<Vec<_>>(),
                headers
                    .iter()
                    .map(|header| (
                        Some(header.index),
                        Some(header.flags),
                        Some(header.line_start),
                        Some(header.line_end)
                    ))
                    .collect::<Vec<_>>(),
                "kibasen admitted records are core's parsed fixed84 entries"
            );
        }
    }
}
