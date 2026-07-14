use super::super::super::DocumentTextSourceSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextStyleRunEvent {
    source_span: DocumentTextSourceSpan,
    byte_start: usize,
    byte_end: usize,
    length: u32,
    raw_bytes: [u8; 5],
}

impl DocumentTextStyleRunEvent {
    pub(crate) fn new(
        source_span: DocumentTextSourceSpan,
        byte_start: usize,
        length: u32,
        raw_bytes: [u8; 5],
    ) -> Self {
        Self {
            source_span,
            byte_start,
            byte_end: byte_start + raw_bytes.len(),
            length,
            raw_bytes,
        }
    }

    pub fn source_span(&self) -> DocumentTextSourceSpan {
        self.source_span
    }

    pub fn byte_start(&self) -> usize {
        self.byte_start
    }

    pub fn byte_end(&self) -> usize {
        self.byte_end
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn raw_bytes(&self) -> &[u8; 5] {
        &self.raw_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextStylePropertyChangeEvent {
    source_span: DocumentTextSourceSpan,
    byte_start: usize,
    byte_end: usize,
    consumed_units: u32,
    properties: Vec<DocumentTextStyleProperty>,
    raw_bytes: Vec<u8>,
}

impl DocumentTextStylePropertyChangeEvent {
    pub(crate) fn new(
        source_span: DocumentTextSourceSpan,
        byte_start: usize,
        byte_end: usize,
        properties: Vec<DocumentTextStyleProperty>,
        raw_bytes: Vec<u8>,
    ) -> Self {
        Self {
            source_span,
            byte_start,
            byte_end,
            consumed_units: 1,
            properties,
            raw_bytes,
        }
    }

    pub fn source_span(&self) -> DocumentTextSourceSpan {
        self.source_span
    }

    pub fn byte_start(&self) -> usize {
        self.byte_start
    }

    pub fn byte_end(&self) -> usize {
        self.byte_end
    }

    pub fn consumed_units(&self) -> u32 {
        self.consumed_units
    }

    pub fn properties(&self) -> &[DocumentTextStyleProperty] {
        &self.properties
    }

    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextStyleProperty {
    byte_start: usize,
    byte_end: usize,
    property_id: u8,
    expected_width: Option<usize>,
    raw_value: Vec<u8>,
    typed_value: Option<DocumentTextStyleTypedValue>,
}

impl DocumentTextStyleProperty {
    pub(crate) fn new(byte_start: usize, property_id: u8, raw_value: Vec<u8>) -> Self {
        let expected_width = expected_property_width(property_id);
        let typed_value = decode_typed_value(expected_width, &raw_value);
        Self {
            byte_start,
            byte_end: byte_start + 2 + raw_value.len(),
            property_id,
            expected_width,
            raw_value,
            typed_value,
        }
    }

    pub fn byte_start(&self) -> usize {
        self.byte_start
    }

    pub fn byte_end(&self) -> usize {
        self.byte_end
    }

    pub fn property_id(&self) -> u8 {
        self.property_id
    }

    pub fn expected_width(&self) -> Option<usize> {
        self.expected_width
    }

    pub fn raw_value(&self) -> &[u8] {
        &self.raw_value
    }

    pub fn typed_value(&self) -> Option<DocumentTextStyleTypedValue> {
        self.typed_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentTextStyleTypedValue {
    U8(u8),
    U16(u16),
    U32(u32),
}

fn expected_property_width(property_id: u8) -> Option<usize> {
    match property_id {
        1 | 2 | 3 | 8 | 13 | 14 | 18 | 19 => Some(2),
        4 | 5 | 6 | 7 | 9 | 10 | 11 | 12 => Some(1),
        15 | 16 | 17 | 20 => Some(4),
        _ => None,
    }
}

fn decode_typed_value(
    expected_width: Option<usize>,
    raw_value: &[u8],
) -> Option<DocumentTextStyleTypedValue> {
    match (expected_width, raw_value) {
        (Some(1), [value]) => Some(DocumentTextStyleTypedValue::U8(*value)),
        (Some(2), [a, b]) => Some(DocumentTextStyleTypedValue::U16(u16::from_be_bytes([
            *a, *b,
        ]))),
        (Some(4), [a, b, c, d]) => Some(DocumentTextStyleTypedValue::U32(u32::from_be_bytes([
            *a, *b, *c, *d,
        ]))),
        _ => None,
    }
}
