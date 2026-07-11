use super::super::DocumentTextSourceSpan;
use super::types::*;

pub fn parse_document_text_row_headers(data: &[u8]) -> Vec<DocumentTextRowHeaderRecord> {
    let units = data
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<_>>();
    let mut records = Vec::new();
    let mut index = 0usize;

    while index + 13 <= units.len() {
        let Some(record) = parse_document_text_row_header_at(&units, index) else {
            index += 1;
            continue;
        };
        let next_index = record.source_span().unit_end();
        if record.fixed_fields().subtype() == 0x008f {
            records.push(record);
        }
        index = next_index.max(index + 1);
    }

    records
}

fn parse_document_text_row_header_at(
    units: &[u16],
    start: usize,
) -> Option<DocumentTextRowHeaderRecord> {
    if units.get(start).copied() != Some(0x001c) || units.get(start + 1).copied() != Some(0x0010) {
        return None;
    }

    let total_len_words = usize::from(*units.get(start + 2)?);
    if total_len_words < 13 {
        return None;
    }

    let end = start.checked_add(total_len_words)?;
    if end > units.len() {
        return None;
    }

    let footer_start = end.checked_sub(4)?;
    let footer = units.get(footer_start..end)?;
    if footer != [total_len_words as u16, 0x0000, 0x0010, 0x001f] {
        return None;
    }

    let fixed_words = units.get(start + 3..start + 9)?;
    let fixed_fields = DocumentTextRowHeaderFixedFields::new(fixed_words.try_into().ok()?);
    let raw_payload_words = units.get(start + 9..footer_start)?.to_vec();
    let (mut pairs, raw_tail_words, tail_truncated) =
        parse_document_text_row_header_pairs(&raw_payload_words, start + 9, fixed_fields.w8());
    let geometry_complete = row_header_geometry_complete(
        &pairs,
        &raw_tail_words,
        tail_truncated,
        fixed_fields.w8(),
        fixed_fields.grid_extent(),
    );
    for pair in &mut pairs {
        pair.geometry_complete = geometry_complete;
    }
    Some(DocumentTextRowHeaderRecord {
        source_span: DocumentTextSourceSpan::new(start, end),
        total_len_words: total_len_words as u16,
        fixed_fields,
        raw_payload_words,
        pairs,
        raw_tail_words,
        tail_truncated,
        geometry_complete,
    })
}

fn parse_document_text_row_header_pairs(
    payload_words: &[u16],
    payload_unit_start: usize,
    cursor_start: u16,
) -> (Vec<DocumentTextRowHeaderPair>, Vec<u16>, bool) {
    let mut pairs = Vec::new();
    let mut index = 0usize;
    let mut cursor = u32::from(cursor_start);

    while index + 1 < payload_words.len() {
        if payload_words[index] == 0xffff && payload_words[index + 1] == 0x0000 {
            break;
        }
        let state_code = payload_words[index];
        let run_length = payload_words[index + 1];
        let start_unit = cursor + 1;
        let end_unit = cursor + u32::from(run_length) + 1;
        pairs.push(DocumentTextRowHeaderPair::new(
            payload_unit_start + index,
            state_code,
            run_length,
            start_unit,
            end_unit,
            DocumentTextRowHeaderPairClassification::from_pair(state_code, run_length),
        ));
        cursor = end_unit;
        index += 2;
    }

    let raw_tail_words = payload_words[index..].to_vec();
    let tail_truncated = !raw_tail_words.len().is_multiple_of(2);
    (pairs, raw_tail_words, tail_truncated)
}

fn row_header_geometry_complete(
    pairs: &[DocumentTextRowHeaderPair],
    raw_tail_words: &[u16],
    tail_truncated: bool,
    cursor_start: u16,
    grid_extent: u16,
) -> bool {
    if tail_truncated {
        return false;
    }
    if !(raw_tail_words.is_empty() || raw_tail_words == [0xffff, 0x0000]) {
        return false;
    }

    let pair_consumed_units = pairs
        .iter()
        .map(|pair| u32::from(pair.run_length()) + 1)
        .sum::<u32>();
    u32::from(grid_extent) == u32::from(cursor_start) + pair_consumed_units + 1
}
