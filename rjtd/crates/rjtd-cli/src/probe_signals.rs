mod document_text_headers;
mod line_mark;
mod page_mark;
mod table;

use line_mark::line_mark_signal;
use page_mark::page_mark_signal;
use table::table_signal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JtdSignal {
    pub line_signature: String,
    pub page_signature: String,
    pub table_signature: String,
    pub source_signature_hash: u64,
    pub line_len: String,
    pub line_declared_count: String,
    pub line_parsed_records: String,
    pub line_deltas: String,
    pub page_family: String,
    pub page_entries: String,
    pub page_tuple_signature: String,
    pub table_candidate_count: usize,
    pub sparse_table_candidate_count: usize,
    pub table_non_empty_cell_count: usize,
    pub table_line_header_signature: String,
    pub table_line_header_rows: Vec<TableLineHeaderRowSignal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableLineHeaderRowSignal {
    pub candidate_index: usize,
    pub row_index: usize,
    pub source_start: usize,
    pub source_end: usize,
    pub raw_offsets: Vec<u16>,
    pub raw_extents: Vec<u16>,
    pub cell_offsets: Vec<u16>,
    pub cell_extents: Vec<u16>,
    pub font_size_units: Vec<u16>,
}

pub fn analyze_jtd(bytes: &[u8]) -> JtdSignal {
    let line = line_mark_signal(bytes);
    let page = page_mark_signal(bytes);
    let table = table_signal(bytes);
    let source_signature = format!(
        "line={};page={};table={}",
        line.signature, page.signature, table.signature
    );

    JtdSignal {
        source_signature_hash: fnv1a64(source_signature.as_bytes()),
        line_signature: line.signature,
        page_signature: page.signature,
        table_signature: table.signature,
        line_len: line.len,
        line_declared_count: line.declared_count,
        line_parsed_records: line.parsed_records,
        line_deltas: line.deltas,
        page_family: page.family,
        page_entries: page.entries,
        page_tuple_signature: page.tuple_signature,
        table_candidate_count: table.candidate_count,
        sparse_table_candidate_count: table.sparse_candidate_count,
        table_non_empty_cell_count: table.non_empty_cell_count,
        table_line_header_signature: table.line_header_signature,
        table_line_header_rows: table.line_header_rows,
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
