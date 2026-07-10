use rjtd_core::container::read_cfb_stream;

const LINE_MARK_HEADER_BYTES: usize = 18;
const LINE_MARK_COUNT_OFFSET: usize = 8;
const LINE_MARK_BASE_UNIT: usize = 16;
const LINE_MARK_RECORD_BYTES: usize = 4;

pub(super) struct LineMarkSignal {
    pub(super) signature: String,
    pub(super) len: String,
    pub(super) declared_count: String,
    pub(super) parsed_records: String,
    pub(super) deltas: String,
}

pub(super) fn line_mark_signal(bytes: &[u8]) -> LineMarkSignal {
    let Ok(stream) = read_cfb_stream(bytes, "/LineMark") else {
        return LineMarkSignal::missing();
    };
    let declared_count = read_be16_candidate(&stream, LINE_MARK_COUNT_OFFSET)
        .map(usize::from)
        .unwrap_or_default();
    let max_records = stream.len().saturating_sub(LINE_MARK_HEADER_BYTES) / LINE_MARK_RECORD_BYTES;
    let parsed_limit = declared_count.min(max_records);
    let mut parsed_records = 0usize;
    let mut unit_start = LINE_MARK_BASE_UNIT;
    let mut deltas = Vec::new();

    for record_index in 0..parsed_limit {
        let byte_offset = LINE_MARK_HEADER_BYTES + record_index * LINE_MARK_RECORD_BYTES;
        let Some(delta_word) = read_be16_candidate(&stream, byte_offset) else {
            break;
        };
        let Some(flag_word) = read_be16_candidate(&stream, byte_offset + 2) else {
            break;
        };
        let delta = delta_word as i16;
        if delta <= 0 {
            break;
        }
        let unit_end = unit_start.saturating_add(delta as usize);
        deltas.push(format!("{delta}:0x{flag_word:04x}:{unit_start}-{unit_end}"));
        unit_start = unit_end;
        parsed_records += 1;
    }

    let delta_signature = deltas.join(",");
    LineMarkSignal {
        signature: format!(
            "len={},declared={},parsed={},deltas={}",
            stream.len(),
            declared_count,
            parsed_records,
            delta_signature
        ),
        len: stream.len().to_string(),
        declared_count: declared_count.to_string(),
        parsed_records: parsed_records.to_string(),
        deltas: delta_signature,
    }
}

impl LineMarkSignal {
    fn missing() -> Self {
        Self {
            signature: "missing".to_string(),
            len: "missing".to_string(),
            declared_count: "missing".to_string(),
            parsed_records: "missing".to_string(),
            deltas: "missing".to_string(),
        }
    }
}

fn read_be16_candidate(bytes: &[u8], offset: usize) -> Option<u16> {
    let chunk = bytes.get(offset..offset + 2)?;
    Some(u16::from_be_bytes([chunk[0], chunk[1]]))
}
