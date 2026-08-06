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
