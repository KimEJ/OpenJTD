use super::super::DocumentTextSourceSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextRowHeaderFixedFields {
    raw_words: [u16; 6],
}

impl DocumentTextRowHeaderFixedFields {
    pub(super) fn new(raw_words: [u16; 6]) -> Self {
        Self { raw_words }
    }

    pub fn raw_words(&self) -> &[u16] {
        &self.raw_words
    }

    pub fn w3(&self) -> u16 {
        self.raw_words[0]
    }

    pub fn subtype(&self) -> u16 {
        self.raw_words[1]
    }

    pub fn w5(&self) -> u16 {
        self.raw_words[2]
    }

    pub fn grid_extent(&self) -> u16 {
        self.raw_words[3]
    }

    pub fn w7(&self) -> u16 {
        self.raw_words[4]
    }

    pub fn w8(&self) -> u16 {
        self.raw_words[5]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentTextRowHeaderPair {
    source_span: DocumentTextSourceSpan,
    state_code: u16,
    run_length: u16,
    start_unit: u32,
    end_unit: u32,
    classification: DocumentTextRowHeaderPairClassification,
    pub(super) geometry_complete: bool,
}

impl DocumentTextRowHeaderPair {
    pub(super) fn new(
        unit_start: usize,
        state_code: u16,
        run_length: u16,
        start_unit: u32,
        end_unit: u32,
        classification: DocumentTextRowHeaderPairClassification,
    ) -> Self {
        Self {
            source_span: DocumentTextSourceSpan::new(unit_start, unit_start + 2),
            state_code,
            run_length,
            start_unit,
            end_unit,
            classification,
            geometry_complete: false,
        }
    }

    pub fn source_span(&self) -> DocumentTextSourceSpan {
        self.source_span
    }

    pub fn state_code(&self) -> u16 {
        self.state_code
    }

    pub fn run_length(&self) -> u16 {
        self.run_length
    }

    pub fn start_unit(&self) -> u32 {
        self.start_unit
    }

    pub fn end_unit(&self) -> u32 {
        self.end_unit
    }

    pub fn classification(&self) -> DocumentTextRowHeaderPairClassification {
        self.classification
    }

    pub fn geometry_complete(&self) -> bool {
        self.geometry_complete
    }

    pub fn raw_words(&self) -> [u16; 2] {
        [self.state_code, self.run_length]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentTextRowHeaderPairClassification {
    Junction,
    BlankRun,
    NonBlankRun,
}

impl DocumentTextRowHeaderPairClassification {
    pub(super) fn from_pair(state_code: u16, run_length: u16) -> Self {
        if run_length == 0 {
            Self::Junction
        } else if state_code == 0 {
            Self::BlankRun
        } else {
            Self::NonBlankRun
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextRowHeaderRecord {
    pub(super) source_span: DocumentTextSourceSpan,
    pub(super) total_len_words: u16,
    pub(super) fixed_fields: DocumentTextRowHeaderFixedFields,
    pub(super) raw_payload_words: Vec<u16>,
    pub(super) pairs: Vec<DocumentTextRowHeaderPair>,
    pub(super) raw_tail_words: Vec<u16>,
    pub(super) tail_truncated: bool,
    pub(super) geometry_complete: bool,
}

impl DocumentTextRowHeaderRecord {
    pub fn source_span(&self) -> DocumentTextSourceSpan {
        self.source_span
    }

    pub fn total_len_words(&self) -> u16 {
        self.total_len_words
    }

    pub fn fixed_fields(&self) -> &DocumentTextRowHeaderFixedFields {
        &self.fixed_fields
    }

    pub fn raw_payload_words(&self) -> &[u16] {
        &self.raw_payload_words
    }

    pub fn pairs(&self) -> &[DocumentTextRowHeaderPair] {
        &self.pairs
    }

    pub fn raw_tail_words(&self) -> &[u16] {
        &self.raw_tail_words
    }

    pub fn tail_truncated(&self) -> bool {
        self.tail_truncated
    }

    pub fn geometry_complete(&self) -> bool {
        self.geometry_complete
    }
}
