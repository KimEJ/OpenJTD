use crate::compressed_document::{
    decompress_just_compressed_document, is_just_compressed_document,
};
use crate::container::read_cfb_stream;
use crate::document_text::COMPRESSED_DOCUMENT_PATH;
use crate::{Error, Result};

pub const DOCUMENT_EDIT_STYLES_PATH: &str = "/DocumentEditStyles";
pub const DOCUMENT_VIEW_STYLES_PATH: &str = "/DocumentViewStyles";
pub const TEXT_LAYOUT_STYLE_PATH: &str = "/TextLayoutStyle";
pub const PAGE_LAYOUT_STYLE_PATH: &str = "/PageLayoutStyle";
pub const PAGE_LAYOUT_STYLE_HEADER_PATH: &str = "/PageLayoutStyleHeader";

const OBSERVED_STYLE_STREAM_PATHS: &[&str] = &[
    DOCUMENT_EDIT_STYLES_PATH,
    DOCUMENT_VIEW_STYLES_PATH,
    TEXT_LAYOUT_STYLE_PATH,
    PAGE_LAYOUT_STYLE_PATH,
    PAGE_LAYOUT_STYLE_HEADER_PATH,
];

const SSMG_RECORD_AREA_OFFSET: usize = 0x114;
const SSMG_SLOT_STRIDE: usize = 0x100;
const SEQUENTIAL_RECORD_SEARCH_LIMIT: usize = 0x200;
const MAX_SEQUENTIAL_RECORD_PAYLOAD_LEN: usize = 0x4000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleStream {
    name: String,
    bytes: Vec<u8>,
}

impl StyleStream {
    fn new(name: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            bytes,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn summary(&self) -> StyleStreamSummary {
        summarize_style_stream(&self.bytes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleStreamSummary {
    family: StyleStreamFamily,
    header_u32_be: Vec<u32>,
    header_u16_be: Vec<u16>,
    record_layout: StyleStreamRecordLayout,
    records: Vec<StyleStreamRecordSummary>,
}

impl StyleStreamSummary {
    fn new(
        family: StyleStreamFamily,
        header_u32_be: Vec<u32>,
        header_u16_be: Vec<u16>,
        record_layout: StyleStreamRecordLayout,
        records: Vec<StyleStreamRecordSummary>,
    ) -> Self {
        Self {
            family,
            header_u32_be,
            header_u16_be,
            record_layout,
            records,
        }
    }

    pub fn family(&self) -> StyleStreamFamily {
        self.family
    }

    pub fn header_u32_be(&self) -> &[u32] {
        &self.header_u32_be
    }

    pub fn header_u16_be(&self) -> &[u16] {
        &self.header_u16_be
    }

    pub fn record_layout(&self) -> StyleStreamRecordLayout {
        self.record_layout
    }

    pub fn records(&self) -> &[StyleStreamRecordSummary] {
        &self.records
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleStreamFamily {
    Ssmg,
    Table,
    Unknown,
}

impl StyleStreamFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ssmg => "ssmg",
            Self::Table => "table",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleStreamRecordLayout {
    None,
    SsmgSlots,
    Sequential,
}

impl StyleStreamRecordLayout {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SsmgSlots => "ssmg-slots",
            Self::Sequential => "sequential",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleStreamRecordSummary {
    offset: usize,
    code: u16,
    payload_len: usize,
    label: Option<String>,
    subrecords: Vec<StyleStreamSubrecordSummary>,
}

impl StyleStreamRecordSummary {
    fn new(
        offset: usize,
        code: u16,
        payload_len: usize,
        label: Option<String>,
        subrecords: Vec<StyleStreamSubrecordSummary>,
    ) -> Self {
        Self {
            offset,
            code,
            payload_len,
            label,
            subrecords,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn payload_len(&self) -> usize {
        self.payload_len
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn subrecords(&self) -> &[StyleStreamSubrecordSummary] {
        &self.subrecords
    }

    pub fn total_len(&self) -> usize {
        4 + self.payload_len
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleStreamSubrecordSummary {
    offset: usize,
    code: u16,
    payload_len: usize,
    payload: Vec<u8>,
}

impl StyleStreamSubrecordSummary {
    fn new(offset: usize, code: u16, payload_len: usize, payload: Vec<u8>) -> Self {
        Self {
            offset,
            code,
            payload_len,
            payload,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn payload_len(&self) -> usize {
        self.payload_len
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn total_len(&self) -> usize {
        4 + self.payload_len
    }
}

pub fn summarize_style_stream(data: &[u8]) -> StyleStreamSummary {
    if data.starts_with(b"SsmgV.01") {
        let records = parse_ssmg_slot_records(data);
        let record_layout = if records.is_empty() {
            StyleStreamRecordLayout::None
        } else {
            StyleStreamRecordLayout::SsmgSlots
        };
        return StyleStreamSummary::new(
            StyleStreamFamily::Ssmg,
            read_be_u32_fields(data, 8, 3),
            read_be_u16_fields(data, 20, 2),
            record_layout,
            records,
        );
    }

    if data.len() >= 8 {
        let records = parse_sequential_style_records(data);
        let record_layout = if records.is_empty() {
            StyleStreamRecordLayout::None
        } else {
            StyleStreamRecordLayout::Sequential
        };
        return StyleStreamSummary::new(
            StyleStreamFamily::Table,
            read_be_u32_fields(data, 0, 4),
            read_be_u16_fields(data, 0, 8),
            record_layout,
            records,
        );
    }

    StyleStreamSummary::new(
        StyleStreamFamily::Unknown,
        Vec::new(),
        read_be_u16_fields(data, 0, 4),
        StyleStreamRecordLayout::None,
        Vec::new(),
    )
}

pub fn read_style_streams(data: &[u8]) -> Result<Vec<StyleStream>> {
    if let Some(inner_document) = maybe_decompressed_inner_document(data)? {
        return read_style_streams_from_cfb(&inner_document);
    }

    read_style_streams_from_cfb(data)
}

fn maybe_decompressed_inner_document(data: &[u8]) -> Result<Option<Vec<u8>>> {
    let compressed = match read_cfb_stream(data, COMPRESSED_DOCUMENT_PATH) {
        Ok(stream) => stream,
        Err(Error::NotFound(_)) => return Ok(None),
        Err(error) => return Err(error),
    };

    if is_just_compressed_document(&compressed) {
        Ok(Some(decompress_just_compressed_document(&compressed)?))
    } else {
        Ok(None)
    }
}

fn read_style_streams_from_cfb(data: &[u8]) -> Result<Vec<StyleStream>> {
    let mut streams = Vec::new();

    for path in OBSERVED_STYLE_STREAM_PATHS {
        match read_cfb_stream(data, path) {
            Ok(bytes) => streams.push(StyleStream::new(*path, bytes)),
            Err(Error::NotFound(_)) => {}
            Err(error) => return Err(error),
        }
    }

    Ok(streams)
}

fn read_be_u32_fields(data: &[u8], start: usize, count: usize) -> Vec<u32> {
    let mut fields = Vec::new();
    let mut offset = start;

    for _ in 0..count {
        let Some(bytes) = data.get(offset..offset + 4) else {
            break;
        };
        fields.push(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
        offset += 4;
    }

    fields
}

fn read_be_u16_fields(data: &[u8], start: usize, count: usize) -> Vec<u16> {
    let mut fields = Vec::new();
    let mut offset = start;

    for _ in 0..count {
        let Some(bytes) = data.get(offset..offset + 2) else {
            break;
        };
        fields.push(u16::from_be_bytes([bytes[0], bytes[1]]));
        offset += 2;
    }

    fields
}

fn parse_ssmg_slot_records(data: &[u8]) -> Vec<StyleStreamRecordSummary> {
    let mut records = Vec::new();
    let mut offset = SSMG_RECORD_AREA_OFFSET;

    while offset + 4 <= data.len() {
        let code = read_be_u16_at(data, offset);
        let payload_len = read_be_u16_at(data, offset + 2) as usize;

        if code == 0 && payload_len == 0 {
            offset += SSMG_SLOT_STRIDE;
            continue;
        }

        if !looks_like_ssmg_slot_record_code(code) {
            offset += SSMG_SLOT_STRIDE;
            continue;
        }

        let payload_start = offset + 4;
        let Some(payload_end) = payload_start.checked_add(payload_len) else {
            break;
        };
        if payload_end > data.len() {
            break;
        }

        let label = read_ssmg_slot_record_label(data, payload_start, payload_len);
        let subrecords = parse_style_record_subrecords(&data[payload_start..payload_end]);
        records.push(StyleStreamRecordSummary::new(
            offset,
            code,
            payload_len,
            label,
            subrecords,
        ));
        offset = align_up_relative(payload_end, SSMG_RECORD_AREA_OFFSET, SSMG_SLOT_STRIDE);
    }

    records
}

fn looks_like_ssmg_slot_record_code(code: u16) -> bool {
    matches!(code, 0x4444 | 0x5555)
}

fn read_ssmg_slot_record_label(
    data: &[u8],
    payload_start: usize,
    payload_len: usize,
) -> Option<String> {
    if payload_len < 2 {
        return None;
    }

    let payload_end = payload_start.checked_add(payload_len)?;
    if payload_end > data.len() {
        return None;
    }

    let unit_len = read_be_u16_at(data, payload_start) as usize;
    if unit_len == 0 {
        return None;
    }

    let label_start = payload_start + 2;
    let label_byte_len = unit_len.checked_mul(2)?;
    let label_end = label_start.checked_add(label_byte_len)?;
    if label_end > payload_end {
        return None;
    }

    let mut units = Vec::with_capacity(unit_len);
    for chunk in data[label_start..label_end].chunks_exact(2) {
        units.push(u16::from_be_bytes([chunk[0], chunk[1]]));
    }
    while units.last().copied() == Some(0) {
        units.pop();
    }
    if units.is_empty() || !looks_like_label_units(&units) {
        return None;
    }

    String::from_utf16(&units).ok()
}

fn looks_like_label_units(units: &[u16]) -> bool {
    units.iter().copied().all(|unit| {
        unit >= 0x20
            && !matches!(unit, 0xfffe | 0xffff)
            && !(0xfdd0..=0xfdef).contains(&unit)
            && !(0xd800..=0xdfff).contains(&unit)
    })
}

fn parse_style_record_subrecords(payload: &[u8]) -> Vec<StyleStreamSubrecordSummary> {
    let mut best = Vec::new();
    let search_limit = payload.len().min(0x80);

    for start in 0..search_limit {
        let records = parse_style_record_subrecords_from(payload, start);
        if records.len() > best.len() {
            best = records;
        }
    }

    if best.len() < 3 { Vec::new() } else { best }
}

fn parse_style_record_subrecords_from(
    payload: &[u8],
    start: usize,
) -> Vec<StyleStreamSubrecordSummary> {
    let mut records = Vec::new();
    let mut offset = start;

    while offset + 4 <= payload.len() {
        let code = read_be_u16_at(payload, offset);
        let payload_len = read_be_u16_at(payload, offset + 2) as usize;
        if !looks_like_style_subrecord_code(code)
            || payload_len == 0
            || payload_len > 0x200
            || offset + 4 + payload_len > payload.len()
        {
            break;
        }

        let payload_start = offset + 4;
        let payload_end = payload_start + payload_len;
        records.push(StyleStreamSubrecordSummary::new(
            offset,
            code,
            payload_len,
            payload[payload_start..payload_end].to_vec(),
        ));
        offset = payload_end;
    }

    records
}

fn looks_like_style_subrecord_code(code: u16) -> bool {
    (0x1000..=0x7fff).contains(&code)
}

fn parse_sequential_style_records(data: &[u8]) -> Vec<StyleStreamRecordSummary> {
    let mut best = Vec::new();
    let search_limit = data.len().min(SEQUENTIAL_RECORD_SEARCH_LIMIT);

    for start in 0..search_limit {
        let records = parse_sequential_style_records_from(data, start);
        if records.len() < 4 {
            continue;
        }
        if records_coverage(&records) > records_coverage(&best) {
            best = records;
        }
    }

    best
}

fn parse_sequential_style_records_from(data: &[u8], start: usize) -> Vec<StyleStreamRecordSummary> {
    let mut records = Vec::new();
    let mut offset = start;

    while offset + 4 <= data.len() {
        let code = read_be_u16_at(data, offset);
        let payload_len = read_be_u16_at(data, offset + 2) as usize;

        if !looks_like_sequential_record_code(code)
            || payload_len == 0
            || payload_len > MAX_SEQUENTIAL_RECORD_PAYLOAD_LEN
        {
            break;
        }

        let payload_start = offset + 4;
        let Some(payload_end) = payload_start.checked_add(payload_len) else {
            break;
        };
        if payload_end > data.len() {
            break;
        }

        records.push(StyleStreamRecordSummary::new(
            offset,
            code,
            payload_len,
            None,
            Vec::new(),
        ));
        offset = payload_end;
    }

    records
}

fn looks_like_sequential_record_code(code: u16) -> bool {
    (0x1000..=0x7fff).contains(&code)
}

fn records_coverage(records: &[StyleStreamRecordSummary]) -> usize {
    match (records.first(), records.last()) {
        (Some(first), Some(last)) => last.offset + last.total_len() - first.offset,
        _ => 0,
    }
}

fn align_up_relative(value: usize, start: usize, stride: usize) -> usize {
    if value <= start {
        return start;
    }
    let relative = value - start;
    start + relative.div_ceil(stride) * stride
}

fn read_be_u16_at(data: &[u8], offset: usize) -> u16 {
    let bytes = &data[offset..offset + 2];
    u16::from_be_bytes([bytes[0], bytes[1]])
}

#[cfg(test)]
mod tests {
    use super::{
        DOCUMENT_EDIT_STYLES_PATH, StyleStreamFamily, StyleStreamRecordLayout,
        TEXT_LAYOUT_STYLE_PATH, read_style_streams, summarize_style_stream,
    };
    use std::io::{Cursor, Write};

    #[test]
    fn reads_observed_style_streams_from_cfb() {
        let bytes = cfb_with_streams(&[
            (DOCUMENT_EDIT_STYLES_PATH, &[1, 2][..]),
            (TEXT_LAYOUT_STYLE_PATH, &[3, 4, 5][..]),
            ("/DocumentText", b"SsmgV.01"),
        ]);

        let streams = read_style_streams(&bytes).unwrap();

        assert_eq!(streams.len(), 2);
        assert_eq!(streams[0].name(), DOCUMENT_EDIT_STYLES_PATH);
        assert_eq!(streams[0].bytes(), &[1, 2]);
        assert_eq!(streams[1].name(), TEXT_LAYOUT_STYLE_PATH);
        assert_eq!(streams[1].bytes(), &[3, 4, 5]);
    }

    #[test]
    fn ignores_absent_style_streams() {
        let bytes = cfb_with_streams(&[("/DocumentText", b"SsmgV.01")]);

        let streams = read_style_streams(&bytes).unwrap();

        assert!(streams.is_empty());
    }

    #[test]
    fn summarizes_ssmg_style_stream_header_fields() {
        let summary = summarize_style_stream(&[
            b'S', b's', b'm', b'g', b'V', b'.', b'0', b'1', 0, 0, 0, 0x1c, 0, 0, 1, 0, 0, 0, 0,
            0x20, 0, 1, 0, 2,
        ]);

        assert_eq!(summary.family(), StyleStreamFamily::Ssmg);
        assert_eq!(summary.header_u32_be(), &[28, 256, 32]);
        assert_eq!(summary.header_u16_be(), &[1, 2]);
        assert_eq!(summary.record_layout(), StyleStreamRecordLayout::None);
        assert!(summary.records().is_empty());
    }

    #[test]
    fn summarizes_table_style_stream_prefix_fields() {
        let summary =
            summarize_style_stream(&[0, 1, 0, 1, 0x20, 0, 0, 0, 0, 0x1c, 0, 0, 0, 0x14, 0, 0]);

        assert_eq!(summary.family(), StyleStreamFamily::Table);
        assert_eq!(
            summary.header_u32_be(),
            &[0x0001_0001, 0x2000_0000, 0x001c_0000, 0x0014_0000]
        );
        assert_eq!(
            summary.header_u16_be(),
            &[
                0x0001, 0x0001, 0x2000, 0x0000, 0x001c, 0x0000, 0x0014, 0x0000
            ]
        );
        assert_eq!(summary.record_layout(), StyleStreamRecordLayout::None);
        assert!(summary.records().is_empty());
    }

    #[test]
    fn summarizes_ssmg_slot_record_boundaries() {
        let mut bytes = vec![
            b'S', b's', b'm', b'g', b'V', b'.', b'0', b'1', 0, 0, 0, 0x1c, 0, 0, 1, 0, 0, 0, 0,
            0x20, 0, 1, 0, 2,
        ];
        bytes.resize(0x114, 0);
        bytes.extend_from_slice(&[0x55, 0x55, 0x00, 0x08, 0x00, 0x03, 0, b'A', 0, b'B', 0, 0]);
        bytes.resize(0x214, 0);
        bytes.extend_from_slice(&[0x55, 0x55, 0x00, 0x02, 4, 5]);
        bytes.resize(0x314, 0);
        bytes.extend_from_slice(&[0x00, 0x19, 0x00, 0x07, 6, 7, 8, 9, 10, 11, 12]);

        let summary = summarize_style_stream(&bytes);

        assert_eq!(summary.record_layout(), StyleStreamRecordLayout::SsmgSlots);
        assert_eq!(summary.records().len(), 2);
        assert_eq!(summary.records()[0].offset(), 0x114);
        assert_eq!(summary.records()[0].code(), 0x5555);
        assert_eq!(summary.records()[0].payload_len(), 8);
        assert_eq!(summary.records()[0].label(), Some("AB"));
        assert!(summary.records()[0].subrecords().is_empty());
        assert_eq!(summary.records()[1].offset(), 0x214);
        assert_eq!(summary.records()[1].code(), 0x5555);
        assert_eq!(summary.records()[1].payload_len(), 2);
        assert_eq!(summary.records()[1].label(), None);
    }

    #[test]
    fn summarizes_nested_ssmg_style_subrecords() {
        let mut bytes = vec![
            b'S', b's', b'm', b'g', b'V', b'.', b'0', b'1', 0, 0, 0, 0x1c, 0, 0, 1, 0, 0, 0, 0,
            0x20, 0, 1, 0, 2,
        ];
        let mut payload = vec![0, 1, 0, b'A'];
        payload.extend_from_slice(&[0, 0]);
        payload.extend_from_slice(&[0x40, 0x01, 0, 1, 0xaa]);
        payload.extend_from_slice(&[0x31, 0x05, 0, 2, 0xbb, 0xcc]);
        payload.extend_from_slice(&[0x39, 0x07, 0, 1, 0xdd]);
        bytes.resize(0x114, 0);
        bytes.extend_from_slice(&[0x44, 0x44]);
        bytes.extend_from_slice(&(payload.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&payload);

        let summary = summarize_style_stream(&bytes);

        assert_eq!(summary.records().len(), 1);
        let record = &summary.records()[0];
        assert_eq!(record.label(), Some("A"));
        assert_eq!(record.subrecords().len(), 3);
        assert_eq!(record.subrecords()[0].offset(), 6);
        assert_eq!(record.subrecords()[0].code(), 0x4001);
        assert_eq!(record.subrecords()[1].code(), 0x3105);
        assert_eq!(record.subrecords()[1].payload(), &[0xbb, 0xcc]);
        assert_eq!(record.subrecords()[2].total_len(), 5);
    }

    #[test]
    fn summarizes_sequential_table_record_boundaries() {
        let mut bytes = vec![0; 0x20];
        bytes.extend_from_slice(&[0x10, 0x02, 0x00, 0x02, 1, 2]);
        bytes.extend_from_slice(&[0x10, 0x03, 0x00, 0x01, 3]);
        bytes.extend_from_slice(&[0x31, 0x04, 0x00, 0x03, 4, 5, 6]);
        bytes.extend_from_slice(&[0x31, 0x05, 0x00, 0x01, 7]);

        let summary = summarize_style_stream(&bytes);

        assert_eq!(summary.family(), StyleStreamFamily::Table);
        assert_eq!(summary.record_layout(), StyleStreamRecordLayout::Sequential);
        assert_eq!(summary.records().len(), 4);
        assert_eq!(summary.records()[0].offset(), 0x20);
        assert_eq!(summary.records()[0].code(), 0x1002);
        assert_eq!(summary.records()[0].payload_len(), 2);
        assert_eq!(summary.records()[3].offset(), 0x32);
        assert_eq!(summary.records()[3].code(), 0x3105);
        assert_eq!(summary.records()[3].payload_len(), 1);
    }

    fn cfb_with_streams(streams: &[(&str, &[u8])]) -> Vec<u8> {
        let mut compound = cfb::CompoundFile::create(Cursor::new(Vec::new())).unwrap();
        for (path, payload) in streams {
            compound
                .create_stream(path)
                .unwrap()
                .write_all(payload)
                .unwrap();
        }
        compound.into_inner().into_inner()
    }
}
