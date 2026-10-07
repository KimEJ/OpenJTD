#[cfg(feature = "rendering")]
pub(crate) fn native_rule_text_supported(text: &str) -> bool {
    text.chars().all(native_rule_character_supported)
}

#[cfg(feature = "rendering")]
pub(crate) fn native_rule_character_supported(character: char) -> bool {
    character.is_ascii_graphic()
        || character == ' '
        || matches!(character, '\u{3000}'..='\u{30ff}' | '\u{3400}'..='\u{9fff}' | '\u{ff01}'..='\u{ffef}')
}

#[cfg(feature = "rendering")]
use crate::*;

#[cfg(feature = "rendering")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct NativeControlTableRowHeader {
    pub(crate) start_unit: usize,
    pub(crate) grid_extent: u16,
    pub(crate) left_edge_units: u16,
}

#[cfg(feature = "rendering")]
#[derive(Debug)]
pub(crate) struct NativeControlFlowSourceLine {
    pub(crate) text: String,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) leading_ascii_space_count: usize,
    pub(crate) line_mark_record_index: usize,
}

#[cfg(feature = "rendering")]
pub(crate) fn native_flow_preceding_header(
    bytes: &[u8],
    range_start: usize,
    entry_byte_start: usize,
) -> Option<ShanaiLanLineHeader> {
    let start = range_start.min(entry_byte_start) & !1;
    let headers = (start..entry_byte_start)
        .step_by(2)
        .filter_map(|offset| shanai_lan_line_header_at(bytes, offset))
        .filter(|header| header.end <= entry_byte_start)
        .filter(|header| header.offset_units == 0 && header.extent_units != 0)
        .collect::<Vec<_>>();
    let [header] = headers.as_slice() else {
        return None;
    };
    Some(*header)
}

#[cfg(feature = "rendering")]
pub(crate) fn native_flow_source_lines(
    entry: &DocumentTextMapEntry,
) -> Option<Vec<NativeControlFlowSourceLine>> {
    if entry.text().contains('\r') || !entry.text().is_ascii() {
        return None;
    }
    let mut offset_units = entry.unit_start();
    let mut lines = Vec::new();
    for chunk in entry.text().split_inclusive('\n') {
        let text = chunk.strip_suffix('\n').unwrap_or(chunk);
        let chunk_units = chunk.encode_utf16().count();
        let text_units = text.encode_utf16().count();
        if !text.is_empty() {
            if !text
                .chars()
                .all(|character| character.is_ascii_graphic() || character == ' ')
            {
                return None;
            }
            let leading_ascii_space_count = text
                .chars()
                .take_while(|character| *character == ' ')
                .count();
            if leading_ascii_space_count == text.chars().count() {
                return None;
            }
            lines.push(NativeControlFlowSourceLine {
                text: text.to_string(),
                source_span: TextSourceSpan::new(
                    offset_units * 2,
                    (offset_units + text_units) * 2,
                    offset_units,
                    offset_units + text_units,
                ),
                leading_ascii_space_count,
                line_mark_record_index: 0,
            });
        }
        offset_units = offset_units.checked_add(chunk_units)?;
    }
    (offset_units == entry.unit_end()).then_some(lines)
}

#[cfg(feature = "rendering")]
pub(crate) fn native_control_table_trailing_empty_row_record(
    bytes: &[u8],
    start_unit: usize,
    grid_extent: u16,
    left_edge_units: u16,
    rows: &[DocumentTextControlTableRow],
    intervals: &[ShanaiLanLineMarkInterval],
) -> Option<usize> {
    let start = rows.iter().position(|row| row.source_start == start_unit)?;
    let mut final_record = None;
    let mut next_unit = start_unit;
    for row in rows.iter().skip(start).take(64) {
        if row.source_start != next_unit || row.cells.iter().any(|cell| !cell.text.is_empty()) {
            break;
        }
        let Some(parent) = native_parent_row_header(bytes, row.source_start) else {
            break;
        };
        if parent.start_unit != row.source_start
            || parent.grid_extent != grid_extent
            || parent.left_edge_units != left_edge_units
        {
            break;
        }
        let Some(interval) = intervals.iter().find(|interval| {
            interval.unit_start == row.source_start && interval.unit_end == row.source_end
        }) else {
            break;
        };
        final_record = Some(interval.record_index);
        next_unit = row.source_end;
    }
    final_record
}

#[cfg(feature = "rendering")]
pub(crate) fn native_matched_cell_headers<'a>(
    row: &'a TableCandidateLineHeaderRow,
    interval: &TableCandidateInterval,
) -> Option<Vec<&'a ShanaiLanLineHeader>> {
    let mut headers: Vec<&ShanaiLanLineHeader> = Vec::new();
    for segment in interval.column_segments() {
        let start = segment.source_start()?;
        let header = row
            .headers
            .iter()
            .filter(|header| header.end / 2 == start)
            .collect::<Vec<_>>();
        if header.len() != 1
            || headers
                .iter()
                .any(|existing| existing.start == header[0].start)
        {
            return None;
        }
        headers.push(header[0]);
    }
    Some(headers)
}

#[cfg(feature = "rendering")]
pub(crate) fn native_parent_row_header(
    bytes: &[u8],
    row_start_unit: usize,
) -> Option<NativeControlTableRowHeader> {
    let units = bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<_>>();
    let mut matches = Vec::new();
    for start in row_start_unit.saturating_sub(512)..=row_start_unit {
        if units.get(start..start + 4)? != [0x001c, 0x0010, *units.get(start + 2)?, 0] {
            continue;
        }
        let length = usize::from(*units.get(start + 2)?);
        let end = start.checked_add(length)?;
        if length < 13 || (start != row_start_unit && end != row_start_unit) {
            continue;
        }
        if units.get(end.checked_sub(4)?..end)? != [length as u16, 0, 0x0010, 0x001f] {
            continue;
        }
        matches.push(NativeControlTableRowHeader {
            start_unit: start,
            grid_extent: *units.get(start + 6)?,
            left_edge_units: *units.get(start + 8)?,
        });
    }
    (matches.len() == 1).then(|| matches[0])
}
