mod events;

pub use events::{
    DocumentTextStyleProperty, DocumentTextStylePropertyChangeEvent, DocumentTextStyleRunEvent,
    DocumentTextStyleTypedValue,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextStyleSection {
    content_unit_count: u32,
    style_start: usize,
    events: Vec<DocumentTextStyleEvent>,
    terminal_bytes: Vec<u8>,
    trailing_bytes: Vec<u8>,
    truncated: bool,
    diagnostics: Vec<DocumentTextStyleDiagnostic>,
}

impl DocumentTextStyleSection {
    pub(crate) fn new(
        content_unit_count: u32,
        style_start: usize,
        events: Vec<DocumentTextStyleEvent>,
        terminal_bytes: Vec<u8>,
        trailing_bytes: Vec<u8>,
        truncated: bool,
        diagnostics: Vec<DocumentTextStyleDiagnostic>,
    ) -> Self {
        Self {
            content_unit_count,
            style_start,
            events,
            terminal_bytes,
            trailing_bytes,
            truncated,
            diagnostics,
        }
    }

    pub fn content_unit_count(&self) -> u32 {
        self.content_unit_count
    }

    pub fn style_start(&self) -> usize {
        self.style_start
    }

    pub fn events(&self) -> &[DocumentTextStyleEvent] {
        &self.events
    }

    pub fn terminal_bytes(&self) -> &[u8] {
        &self.terminal_bytes
    }

    pub fn trailing_bytes(&self) -> &[u8] {
        &self.trailing_bytes
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn diagnostics(&self) -> &[DocumentTextStyleDiagnostic] {
        &self.diagnostics
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentTextStyleEvent {
    Run(DocumentTextStyleRunEvent),
    PropertyChange(DocumentTextStylePropertyChangeEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentTextStyleDiagnostic {
    offset: usize,
    kind: DocumentTextStyleDiagnosticKind,
}

impl DocumentTextStyleDiagnostic {
    pub(crate) fn new(offset: usize, kind: DocumentTextStyleDiagnosticKind) -> Self {
        Self { offset, kind }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn kind(&self) -> DocumentTextStyleDiagnosticKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentTextStyleDiagnosticKind {
    HeaderTooShort,
    StyleStartOverflow,
    StyleStartPastEnd,
    ZeroLengthRun,
    UnexpectedMarker,
    TruncatedRun,
    TruncatedProperty,
    TruncatedPropertyValue,
    TruncatedPropertyTerminator,
    CursorOverflow,
    CursorPastContentEnd,
}

pub fn document_text_style_code_name(code: u32) -> Option<&'static str> {
    match code {
        1 => Some("single"),
        2 => Some("double"),
        3 => Some("dotted"),
        4 => Some("dash"),
        5 => Some("long-dash"),
        6 => Some("dot-dash"),
        7 => Some("dot-dot-dash"),
        8 => Some("wave"),
        9 => Some("bold"),
        10 => Some("bold-dotted"),
        11 => Some("bold-dash"),
        12 => Some("bold-long-dash"),
        13 => Some("bold-dot-dash"),
        14 => Some("bold-dot-dot-dash"),
        15 => Some("bold-wave"),
        16 => Some("double-wave"),
        17 => Some("small-wave"),
        18 => Some("single-line"),
        19 => Some("double-line"),
        20 => Some("thick-line"),
        _ => None,
    }
}
