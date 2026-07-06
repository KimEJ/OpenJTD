use std::path::Path;

use crate::probe_signals::TableLineHeaderRowSignal;

pub(crate) fn format_row_cell_offsets(rows: &[TableLineHeaderRowSignal]) -> String {
    if rows.is_empty() {
        return "-".to_string();
    }
    rows.iter()
        .map(|row| {
            row.cell_offsets
                .iter()
                .map(u16::to_string)
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join("|")
}

pub(crate) fn format_optional_u16(value: Option<u16>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_usize(value: Option<usize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_i32(value: Option<i32>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_isize(value: Option<isize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn format_optional_usize_list(values: &[Option<usize>]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }
    values
        .iter()
        .map(|value| format_optional_usize(*value))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn format_uniform_delta(
    record_delta: Option<isize>,
    base_records: &[Option<usize>],
    candidate_records: &[Option<usize>],
) -> String {
    if record_delta.is_some() {
        return "true".to_string();
    }
    if base_records.is_empty() || base_records.len() != candidate_records.len() {
        return "-".to_string();
    }
    "false".to_string()
}

pub(crate) fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("-")
        .to_string()
}
