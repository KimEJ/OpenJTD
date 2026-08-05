use std::collections::BTreeMap;

use rjtd_core::layout_mark::{PageMark, read_page_mark, read_paper_mark};
use rjtd_model::{DocumentCore, page_mark_u16_geometry_profile};

use super::support::*;
use super::text_position_count_support::format_optional_f32_3;

pub(crate) fn page_mark_summary(bytes: &[u8]) -> String {
    let Ok(page_mark) = read_page_mark(bytes) else {
        return "missing".to_string();
    };
    let header = page_mark.header();
    format!(
        "count={},stride={},last={},entries={},family={}",
        header.count_value(),
        header.stride_value(),
        header.last_index_value(),
        page_mark.entries().len(),
        page_mark.family().as_str()
    )
}

pub(crate) fn page_mark_entries_summary(bytes: &[u8]) -> String {
    read_page_mark(bytes)
        .map(|page_mark| page_mark.entries().len().to_string())
        .unwrap_or_else(|_| "missing".to_string())
}

pub(crate) fn paper_mark_summary(bytes: &[u8]) -> String {
    let Ok(paper_mark) = read_paper_mark(bytes) else {
        return "missing".to_string();
    };
    let header = paper_mark.header();
    format!(
        "count={},stride={},last={},entries={}",
        header.count_value(),
        header.stride_value(),
        header.last_index_value(),
        paper_mark.entries().len()
    )
}

pub(crate) fn paper_mark_entries_summary(bytes: &[u8]) -> String {
    read_paper_mark(bytes)
        .map(|paper_mark| paper_mark.entries().len().to_string())
        .unwrap_or_else(|_| "missing".to_string())
}

pub(crate) const PAGE_MARK_U16_PROFILE_WORD_INDEXES: [usize; 8] = [10, 13, 14, 17, 18, 19, 20, 21];
pub(crate) const PAGE_MARK_U16_PROFILE_CLASSES: [&str; 4] = [
    "zero-sentinel",
    "additive-row",
    "additive-boundary",
    "mixed-payload",
];

pub(crate) fn write_page_mark_u16_profile(page_mark: &PageMark) -> Result<(), String> {
    let mut class_counts = BTreeMap::<&'static str, usize>::new();
    let mut tuple_counts = BTreeMap::<(&'static str, [Option<u16>; 8]), usize>::new();

    for entry in page_mark.entries() {
        let fields = be16_words(entry.raw()).collect::<Vec<_>>();
        let profile = page_mark_u16_geometry_profile(&fields);
        let class_name = profile.class_name();
        *class_counts.entry(class_name).or_insert(0) += 1;
        let tuple = PAGE_MARK_U16_PROFILE_WORD_INDEXES.map(|index| fields.get(index).copied());
        *tuple_counts.entry((class_name, tuple)).or_insert(0) += 1;
    }

    write_stdout_line(&format!(
        "summary\tentries={}\tzero-sentinel={}\tadditive-row={}\tadditive-boundary={}\tmixed-payload={}\tdecoded=false",
        page_mark.entries().len(),
        class_counts.get("zero-sentinel").copied().unwrap_or(0),
        class_counts.get("additive-row").copied().unwrap_or(0),
        class_counts.get("additive-boundary").copied().unwrap_or(0),
        class_counts.get("mixed-payload").copied().unwrap_or(0)
    ))?;

    for class_name in PAGE_MARK_U16_PROFILE_CLASSES {
        write_stdout_line(&format!(
            "profile\t{}\t{}",
            class_name,
            class_counts.get(class_name).copied().unwrap_or(0)
        ))?;
    }

    for ((class_name, tuple), count) in tuple_counts {
        write_stdout_line(&format!(
            "tuple\t{}\t{}\t{}",
            class_name,
            count,
            format_page_mark_u16_profile_tuple(&tuple)
        ))?;
    }

    Ok(())
}

pub(crate) fn write_page_mark_pitch_profile(
    path: &str,
    bytes: &[u8],
    page_mark: &PageMark,
) -> Result<(), String> {
    let mut core = DocumentCore::from_bytes(bytes).map_err(|error| error.to_string())?;
    core.set_file_name(path);
    let layout = core.page_layout();
    let mut class_counts = BTreeMap::<&'static str, usize>::new();

    for entry in page_mark.entries() {
        let fields = be16_words(entry.raw()).collect::<Vec<_>>();
        let class_name = page_mark_u16_geometry_profile(&fields).class_name();
        *class_counts.entry(class_name).or_insert(0) += 1;
    }

    write_stdout_line(&format!(
        "summary\tentries={}\tpageWidthPx={:.3}\tpageHeightPx={:.3}\tbodyWidthPx={:.3}\tbodyHeightPx={:.3}\tmarginPx={:.3}\tzero-sentinel={}\tadditive-row={}\tadditive-boundary={}\tmixed-payload={}\tdecoded=false",
        page_mark.entries().len(),
        layout.width_px(),
        layout.height_px(),
        layout.body_width_px(),
        layout.body_height_px(),
        layout.margin_px(),
        class_counts.get("zero-sentinel").copied().unwrap_or(0),
        class_counts.get("additive-row").copied().unwrap_or(0),
        class_counts.get("additive-boundary").copied().unwrap_or(0),
        class_counts.get("mixed-payload").copied().unwrap_or(0)
    ))?;

    for (row, entry) in page_mark.entries().iter().enumerate() {
        let fields = be16_words(entry.raw()).collect::<Vec<_>>();
        let profile = page_mark_u16_geometry_profile(&fields);
        let line_count = page_mark_entry_line_count(entry.line_start(), entry.line_end());
        let line_gap_count = line_count.and_then(|count| count.checked_sub(1));
        let tuple = PAGE_MARK_U16_PROFILE_WORD_INDEXES.map(|index| fields.get(index).copied());
        write_stdout_line(&format!(
            "entry\t{}\tclass={}\tpageIndex={}\tlineStart={}\tlineEnd={}\tlineCount={}\tlineGapCount={}\tpageHeightPxPerLineCount={}\tpageHeightPxPerLineGap={}\tbodyHeightPxPerLineCount={}\tbodyHeightPxPerLineGap={}\t{}\tdecoded=false",
            row,
            profile.class_name(),
            format_optional_u32(entry.index()),
            format_optional_u32(entry.line_start()),
            format_optional_u32(entry.line_end()),
            format_optional_u32(line_count),
            format_optional_u32(line_gap_count),
            format_optional_f32_3(page_mark_pitch(layout.height_px(), line_count)),
            format_optional_f32_3(page_mark_pitch(layout.height_px(), line_gap_count)),
            format_optional_f32_3(page_mark_pitch(layout.body_height_px(), line_count)),
            format_optional_f32_3(page_mark_pitch(layout.body_height_px(), line_gap_count)),
            format_page_mark_u16_profile_tuple(&tuple)
        ))?;
    }

    Ok(())
}

pub(crate) fn page_mark_entry_line_count(
    line_start: Option<u32>,
    line_end: Option<u32>,
) -> Option<u32> {
    let line_start = line_start?;
    let line_end = line_end?;
    if line_end < line_start {
        return None;
    }
    Some(line_end - line_start + 1)
}

pub(crate) fn page_mark_pitch(size_px: f32, count: Option<u32>) -> Option<f32> {
    let count = count?;
    if count == 0 {
        return None;
    }
    Some(size_px / count as f32)
}

pub(crate) fn format_page_mark_u16_profile_tuple(tuple: &[Option<u16>; 8]) -> String {
    PAGE_MARK_U16_PROFILE_WORD_INDEXES
        .iter()
        .zip(tuple.iter())
        .map(|(word_index, value)| {
            format!(
                "w{}={}",
                word_index,
                value
                    .map(|value| format!("{value}/0x{value:04x}"))
                    .unwrap_or_else(|| "-".to_string())
            )
        })
        .collect::<Vec<_>>()
        .join("\t")
}

pub(crate) struct PageMarkShapeClassification {
    pub(crate) name: &'static str,
    pub(crate) rows: Option<usize>,
    pub(crate) row_bytes: Option<usize>,
    pub(crate) trim_bytes: usize,
}

impl PageMarkShapeClassification {
    pub(crate) fn new(
        name: &'static str,
        rows: Option<usize>,
        row_bytes: Option<usize>,
        trim_bytes: usize,
    ) -> Self {
        Self {
            name,
            rows,
            row_bytes,
            trim_bytes,
        }
    }
}

pub(crate) fn classify_page_mark_shape(
    tail_bytes: usize,
    header_count: u32,
    header_stride: u32,
    header_last: u32,
) -> PageMarkShapeClassification {
    if header_stride != 0x10 || header_count > 10_000 || header_last > 10_000 {
        return PageMarkShapeClassification::new("non-page-header", None, None, 0);
    }

    let count_plus_one = header_count.saturating_add(1) as usize;
    if count_plus_one > 0 && tail_bytes.is_multiple_of(84) {
        let rows = tail_bytes / 84;
        if rows == count_plus_one {
            return PageMarkShapeClassification::new(
                "fixed84-count-plus-one",
                Some(rows),
                Some(84),
                0,
            );
        }
        return PageMarkShapeClassification::new("fixed84", Some(rows), Some(84), 0);
    }

    if count_plus_one > 0 && tail_bytes.is_multiple_of(count_plus_one) {
        return PageMarkShapeClassification::new(
            "count-plus-one-variable",
            Some(count_plus_one),
            Some(tail_bytes / count_plus_one),
            0,
        );
    }

    if tail_bytes >= 2 {
        let trimmed = tail_bytes - 2;
        if count_plus_one > 0 && trimmed.is_multiple_of(count_plus_one) {
            return PageMarkShapeClassification::new(
                "count-plus-one-trim2",
                Some(count_plus_one),
                Some(trimmed / count_plus_one),
                2,
            );
        }
    }

    let count = header_count as usize;
    if count > 0 && tail_bytes.is_multiple_of(count) {
        return PageMarkShapeClassification::new(
            "count-variable",
            Some(count),
            Some(tail_bytes / count),
            0,
        );
    }

    if tail_bytes >= 84 {
        return PageMarkShapeClassification::new(
            "fixed84-tail",
            Some(tail_bytes / 84),
            Some(84),
            tail_bytes % 84,
        );
    }

    PageMarkShapeClassification::new("unclassified", None, None, 0)
}

pub(crate) fn classify_paper_mark_shape(
    tail_bytes: usize,
    header_count: u32,
    header_stride: u32,
    header_last: u32,
) -> PageMarkShapeClassification {
    if header_stride != 0x0c || header_count > 10_000 || header_last > 10_000 {
        return PageMarkShapeClassification::new("non-paper-header", None, None, 0);
    }

    if tail_bytes.is_multiple_of(8) {
        return PageMarkShapeClassification::new("fixed8", Some(tail_bytes / 8), Some(8), 0);
    }

    PageMarkShapeClassification::new("unclassified", None, None, 0)
}

pub(crate) fn write_fixed_row_candidate(
    label: &str,
    tail_bytes: usize,
    row_bytes: usize,
) -> Result<(), String> {
    write_stdout_line(&format!(
        "candidate\t{}\t{}\t{}\t{}",
        label,
        tail_bytes / row_bytes,
        row_bytes,
        tail_bytes % row_bytes
    ))
}

pub(crate) fn write_header_row_candidate(
    label: &str,
    tail_bytes: usize,
    row_count: u32,
) -> Result<(), String> {
    if row_count == 0 {
        return write_stdout_line(&format!("candidate\t{label}\t-\t-\t-"));
    }
    let row_count = row_count as usize;
    write_stdout_line(&format!(
        "candidate\t{}\t{}\t{}\t{}",
        label,
        row_count,
        tail_bytes / row_count,
        tail_bytes % row_count
    ))
}
