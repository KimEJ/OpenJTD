#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleStreamSpan {
    byte_start: usize,
    byte_end: usize,
}

impl StyleStreamSpan {
    fn new(byte_start: usize, byte_end: usize) -> Self {
        Self {
            byte_start,
            byte_end,
        }
    }

    pub fn byte_start(&self) -> usize {
        self.byte_start
    }

    pub fn byte_end(&self) -> usize {
        self.byte_end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentEditStyleSectionInventory {
    header: [u8; 4],
    sections: Vec<DocumentEditStyleSection>,
    trailing_bytes: Vec<u8>,
}

impl DocumentEditStyleSectionInventory {
    fn new(
        header: [u8; 4],
        sections: Vec<DocumentEditStyleSection>,
        trailing_bytes: Vec<u8>,
    ) -> Self {
        Self {
            header,
            sections,
            trailing_bytes,
        }
    }

    pub fn header(&self) -> &[u8; 4] {
        &self.header
    }

    pub fn sections(&self) -> &[DocumentEditStyleSection] {
        &self.sections
    }

    pub fn trailing_bytes(&self) -> &[u8] {
        &self.trailing_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentEditStyleSection {
    source_span: StyleStreamSpan,
    section_code: u16,
    payload_len_bytes: u32,
    payload: Vec<u8>,
    nested_records: Vec<DocumentEditStyleNestedRecord>,
    nested_trailing_bytes: Vec<u8>,
}

impl DocumentEditStyleSection {
    fn new(
        source_span: StyleStreamSpan,
        section_code: u16,
        payload_len_bytes: u32,
        payload: Vec<u8>,
        nested_records: Vec<DocumentEditStyleNestedRecord>,
        nested_trailing_bytes: Vec<u8>,
    ) -> Self {
        Self {
            source_span,
            section_code,
            payload_len_bytes,
            payload,
            nested_records,
            nested_trailing_bytes,
        }
    }

    pub fn source_span(&self) -> StyleStreamSpan {
        self.source_span
    }

    pub fn section_code(&self) -> u16 {
        self.section_code
    }

    pub fn payload_len_bytes(&self) -> u32 {
        self.payload_len_bytes
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn nested_records(&self) -> &[DocumentEditStyleNestedRecord] {
        &self.nested_records
    }

    pub fn nested_trailing_bytes(&self) -> &[u8] {
        &self.nested_trailing_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentEditStyleNestedRecord {
    source_span: StyleStreamSpan,
    record_type: u16,
    length_bytes: u16,
    payload: Vec<u8>,
}

impl DocumentEditStyleNestedRecord {
    fn new(
        source_span: StyleStreamSpan,
        record_type: u16,
        length_bytes: u16,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            source_span,
            record_type,
            length_bytes,
            payload,
        }
    }

    pub fn source_span(&self) -> StyleStreamSpan {
        self.source_span
    }

    pub fn record_type(&self) -> u16 {
        self.record_type
    }

    pub fn length_bytes(&self) -> u16 {
        self.length_bytes
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

pub fn parse_document_edit_style_sections(
    data: &[u8],
) -> Option<DocumentEditStyleSectionInventory> {
    let header: [u8; 4] = data.get(..4)?.try_into().ok()?;
    let mut sections = Vec::new();
    let mut trailing_bytes = Vec::new();
    let mut offset = 4usize;

    while offset < data.len() {
        let Some(section_header) = data.get(offset..offset + 6) else {
            trailing_bytes = data[offset..].to_vec();
            break;
        };
        let section_code = u16::from_be_bytes([section_header[0], section_header[1]]);
        let payload_len_bytes = u32::from_be_bytes([
            section_header[2],
            section_header[3],
            section_header[4],
            section_header[5],
        ]);
        let payload_start = offset + 6;
        let payload_len = usize::try_from(payload_len_bytes).ok()?;
        let Some(payload_end) = payload_start.checked_add(payload_len) else {
            trailing_bytes = data[offset..].to_vec();
            break;
        };
        if payload_end > data.len() {
            trailing_bytes = data[offset..].to_vec();
            break;
        }

        let payload = data[payload_start..payload_end].to_vec();
        let (nested_records, nested_trailing_bytes) = if section_code == 0x2001 {
            parse_document_edit_style_nested_records(&payload, payload_start)
        } else {
            (Vec::new(), Vec::new())
        };
        sections.push(DocumentEditStyleSection::new(
            StyleStreamSpan::new(offset, payload_end),
            section_code,
            payload_len_bytes,
            payload,
            nested_records,
            nested_trailing_bytes,
        ));
        offset = payload_end;
    }

    Some(DocumentEditStyleSectionInventory::new(
        header,
        sections,
        trailing_bytes,
    ))
}

fn parse_document_edit_style_nested_records(
    payload: &[u8],
    payload_start: usize,
) -> (Vec<DocumentEditStyleNestedRecord>, Vec<u8>) {
    let mut records = Vec::new();
    let mut offset = 0usize;

    while offset < payload.len() {
        let Some(record_header) = payload.get(offset..offset + 4) else {
            return (records, payload[offset..].to_vec());
        };
        let record_type = u16::from_be_bytes([record_header[0], record_header[1]]);
        let length_bytes = u16::from_be_bytes([record_header[2], record_header[3]]);
        let payload_offset = offset + 4;
        let payload_len = usize::from(length_bytes);
        let Some(record_end) = payload_offset.checked_add(payload_len) else {
            return (records, payload[offset..].to_vec());
        };
        if record_end > payload.len() {
            return (records, payload[offset..].to_vec());
        }

        records.push(DocumentEditStyleNestedRecord::new(
            StyleStreamSpan::new(payload_start + offset, payload_start + record_end),
            record_type,
            length_bytes,
            payload[payload_offset..record_end].to_vec(),
        ));
        offset = record_end;
    }

    (records, Vec::new())
}
