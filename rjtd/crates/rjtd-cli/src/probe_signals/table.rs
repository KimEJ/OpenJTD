use rjtd_model::parse_document;

use super::TableLineHeaderRowSignal;
use super::document_text_headers::document_text_table_line_header_rows;

pub(super) struct TableSignal {
    pub(super) signature: String,
    pub(super) candidate_count: usize,
    pub(super) sparse_candidate_count: usize,
    pub(super) non_empty_cell_count: usize,
    pub(super) line_header_signature: String,
    pub(super) line_header_rows: Vec<TableLineHeaderRowSignal>,
}

pub(super) fn table_signal(bytes: &[u8]) -> TableSignal {
    let Ok(document) = parse_document(bytes) else {
        return TableSignal {
            signature: "parse-error".to_string(),
            candidate_count: 0,
            sparse_candidate_count: 0,
            non_empty_cell_count: 0,
            line_header_signature: "parse-error".to_string(),
            line_header_rows: Vec::new(),
        };
    };
    let line_header_rows = document_text_table_line_header_rows(bytes, document.table_candidates());
    let line_header_signature = table_line_header_signature(&line_header_rows);
    let candidate_count = document.table_candidates().len();
    let sparse_candidate_count = document
        .table_candidates()
        .iter()
        .filter(|candidate| candidate.is_sparse_document_text_control_run_candidate())
        .count();
    let non_empty_cell_count = document
        .table_candidates()
        .iter()
        .map(|candidate| candidate.non_empty_cell_count_candidate())
        .sum::<usize>();
    let interval_signature = document
        .table_candidates()
        .iter()
        .map(|candidate| {
            format!(
                "{}:{}:{}:{}:{}",
                candidate.kind(),
                candidate.interval_count(),
                candidate.source_start(),
                candidate.source_end(),
                candidate.non_empty_cell_count_candidate()
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    TableSignal {
        signature: format!(
            "count={candidate_count},sparse={sparse_candidate_count},non-empty={non_empty_cell_count},intervals={interval_signature}"
        ),
        candidate_count,
        sparse_candidate_count,
        non_empty_cell_count,
        line_header_signature,
        line_header_rows,
    }
}

fn table_line_header_signature(rows: &[TableLineHeaderRowSignal]) -> String {
    if rows.is_empty() {
        return "missing".to_string();
    }
    rows.iter()
        .map(|row| {
            format!(
                "{}:{}:{}-{}:{}:{}",
                row.candidate_index,
                row.row_index,
                row.source_start,
                row.source_end,
                format_u16_values(&row.cell_offsets),
                format_u16_values(&row.cell_extents)
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn format_u16_values(values: &[u16]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }
    values
        .iter()
        .map(u16::to_string)
        .collect::<Vec<_>>()
        .join(",")
}
