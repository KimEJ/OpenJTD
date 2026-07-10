use rjtd_core::container::read_cfb_stream;

use crate::probe_signals::{JtdSignal, TableLineHeaderRowSignal};

const LINE_MARK_HEADER_BYTES: usize = 18;
const LINE_MARK_COUNT_OFFSET: usize = 8;
const LINE_MARK_BASE_UNIT: usize = 16;
const LINE_MARK_RECORD_BYTES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
struct LineRecord {
    delta: i16,
    flag: u16,
    unit_start: usize,
    unit_end: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct LineDetail {
    declared_count: String,
    parsed_records: String,
    records: Vec<LineRecord>,
}

impl LineDetail {
    pub(crate) fn record_index_containing(
        &self,
        source_start: usize,
        source_end: usize,
    ) -> Option<usize> {
        if source_start >= source_end {
            return None;
        }
        self.records
            .iter()
            .position(|record| record.unit_start <= source_start && source_end <= record.unit_end)
    }

    pub(crate) fn record_index_matching(
        &self,
        source_start: usize,
        source_end: usize,
    ) -> Option<usize> {
        if source_start >= source_end {
            return None;
        }
        self.records
            .iter()
            .position(|record| record.unit_start == source_start && record.unit_end == source_end)
    }

    pub(crate) fn record_stride_containing(
        &self,
        rows: &[TableLineHeaderRowSignal],
    ) -> Option<isize> {
        if rows.len() < 2 {
            return None;
        }
        let records = rows
            .iter()
            .map(|row| self.record_index_containing(row.source_start, row.source_end))
            .collect::<Option<Vec<_>>>()?;
        uniform_adjacent_delta(&records)
    }

    pub(crate) fn exact_source_range_match_count(
        &self,
        rows: &[TableLineHeaderRowSignal],
    ) -> usize {
        rows.iter()
            .filter(|row| {
                self.record_index_matching(row.source_start, row.source_end)
                    .is_some()
            })
            .count()
    }

    pub(crate) fn rows_exact_and_contiguous(&self, rows: &[TableLineHeaderRowSignal]) -> bool {
        if rows.is_empty() {
            return false;
        }
        let Some(records) = rows
            .iter()
            .map(|row| self.record_index_matching(row.source_start, row.source_end))
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let contiguous_records = records
            .windows(2)
            .all(|records| records[0].checked_add(1) == Some(records[1]));
        let contiguous_sources = rows
            .windows(2)
            .all(|rows| rows[0].source_end == rows[1].source_start);
        contiguous_records && contiguous_sources
    }
}

fn uniform_adjacent_delta(values: &[usize]) -> Option<isize> {
    let mut deltas = values
        .windows(2)
        .map(|values| optional_usize_delta(values[0], values[1]));
    let first_delta = deltas.next()??;
    if deltas.all(|delta| delta == Some(first_delta)) {
        Some(first_delta)
    } else {
        None
    }
}

fn optional_usize_delta(base: usize, candidate: usize) -> Option<isize> {
    let base = isize::try_from(base).ok()?;
    let candidate = isize::try_from(candidate).ok()?;
    Some(candidate.saturating_sub(base))
}

pub(crate) fn line_detail(bytes: &[u8]) -> LineDetail {
    let Ok(stream) = read_cfb_stream(bytes, "/LineMark") else {
        return LineDetail {
            declared_count: "missing".to_string(),
            parsed_records: "missing".to_string(),
            records: Vec::new(),
        };
    };
    let declared_count = read_be16(&stream, LINE_MARK_COUNT_OFFSET)
        .map(usize::from)
        .unwrap_or_default();
    let max_records = stream.len().saturating_sub(LINE_MARK_HEADER_BYTES) / LINE_MARK_RECORD_BYTES;
    let parsed_limit = declared_count.min(max_records);
    let mut records = Vec::new();
    let mut unit_start = LINE_MARK_BASE_UNIT;

    for record_index in 0..parsed_limit {
        let byte_offset = LINE_MARK_HEADER_BYTES + record_index * LINE_MARK_RECORD_BYTES;
        let Some(delta_word) = read_be16(&stream, byte_offset) else {
            break;
        };
        let Some(flag) = read_be16(&stream, byte_offset + 2) else {
            break;
        };
        let delta = delta_word as i16;
        if delta <= 0 {
            break;
        }
        let unit_end = unit_start.saturating_add(delta as usize);
        records.push(LineRecord {
            delta,
            flag,
            unit_start,
            unit_end,
        });
        unit_start = unit_end;
    }

    LineDetail {
        declared_count: declared_count.to_string(),
        parsed_records: records.len().to_string(),
        records,
    }
}

pub(crate) fn line_diff_lines(
    base: &LineDetail,
    candidate: &LineDetail,
    base_signal: &JtdSignal,
    candidate_signal: &JtdSignal,
) -> Vec<String> {
    let mut lines = vec![format!(
        "line-summary\tbaseDeclared={}\tcandidateDeclared={}\tbaseParsed={}\tcandidateParsed={}\tlineSignatureSame={}",
        base.declared_count,
        candidate.declared_count,
        base.parsed_records,
        candidate.parsed_records,
        base_signal.line_signature == candidate_signal.line_signature
    )];
    for record_index in 0..base.records.len().max(candidate.records.len()) {
        match (
            base.records.get(record_index),
            candidate.records.get(record_index),
        ) {
            (Some(left), Some(right)) if left == right => {}
            (Some(left), Some(right)) => lines.push(line_record_diff(
                record_index,
                "changed",
                Some(left),
                Some(right),
            )),
            (Some(left), None) => {
                lines.push(line_record_diff(record_index, "removed", Some(left), None));
            }
            (None, Some(right)) => {
                lines.push(line_record_diff(record_index, "added", None, Some(right)));
            }
            (None, None) => {}
        }
    }
    if lines.len() == 1 {
        lines.push("line-delta-diff\tstatus=none".to_string());
    }
    lines
}

fn line_record_diff(
    record_index: usize,
    status: &str,
    base: Option<&LineRecord>,
    candidate: Option<&LineRecord>,
) -> String {
    format!(
        "line-delta-diff\trecord={record_index}\tstatus={status}\tbaseDelta={}\tcandidateDelta={}\tbaseFlag={}\tcandidateFlag={}\tbaseSpan={}\tcandidateSpan={}",
        base.map_or("-".to_string(), |record| record.delta.to_string()),
        candidate.map_or("-".to_string(), |record| record.delta.to_string()),
        base.map_or("-".to_string(), |record| format!("0x{:04x}", record.flag)),
        candidate.map_or("-".to_string(), |record| format!("0x{:04x}", record.flag)),
        base.map_or("-".to_string(), span),
        candidate.map_or("-".to_string(), span)
    )
}

fn span(record: &LineRecord) -> String {
    format!("{}-{}", record.unit_start, record.unit_end)
}

fn read_be16(bytes: &[u8], offset: usize) -> Option<u16> {
    let chunk = bytes.get(offset..offset + 2)?;
    Some(u16::from_be_bytes([chunk[0], chunk[1]]))
}
