use super::{
    DocumentTextStyleDiagnostic, DocumentTextStyleEvent, DocumentTextStyleSection,
    DocumentTextStyleTypedValue, parse_document_text_style_section,
};

const PROPERTY_SLOT_COUNT: usize = 21;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentTextResolvedStyle {
    values: [Option<DocumentTextStyleTypedValue>; PROPERTY_SLOT_COUNT],
}

impl Default for DocumentTextResolvedStyle {
    fn default() -> Self {
        Self {
            values: [None; PROPERTY_SLOT_COUNT],
        }
    }
}

impl DocumentTextResolvedStyle {
    pub fn value(self, property_id: u8) -> Option<DocumentTextStyleTypedValue> {
        self.values.get(usize::from(property_id)).copied().flatten()
    }

    fn apply(&mut self, property_id: u8, value: Option<DocumentTextStyleTypedValue>) {
        if let Some(slot) = self.values.get_mut(usize::from(property_id)) {
            *slot = value;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DocumentTextResolvedStyleSpan {
    source_unit_start: usize,
    source_unit_end: usize,
    style: DocumentTextResolvedStyle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextStyleResolver {
    section_present: bool,
    content_unit_count: u32,
    style_start: usize,
    event_count: usize,
    truncated: bool,
    diagnostics: Vec<DocumentTextStyleDiagnostic>,
    spans: Vec<DocumentTextResolvedStyleSpan>,
}

impl DocumentTextStyleResolver {
    pub fn from_document_text_bytes(bytes: &[u8]) -> Self {
        let section = parse_document_text_style_section(bytes);
        Self::from_style_section(bytes.len(), &section)
    }

    fn from_style_section(bytes_len: usize, section: &DocumentTextStyleSection) -> Self {
        let mut spans = Vec::with_capacity(section.events().len());
        let mut style = DocumentTextResolvedStyle::default();
        for event in section.events() {
            match event {
                DocumentTextStyleEvent::Run(run) => {
                    spans.push(DocumentTextResolvedStyleSpan {
                        source_unit_start: run.source_span().unit_start(),
                        source_unit_end: run.source_span().unit_end(),
                        style,
                    });
                }
                DocumentTextStyleEvent::PropertyChange(change) => {
                    for property in change.properties() {
                        style.apply(property.property_id(), property.typed_value());
                    }
                    spans.push(DocumentTextResolvedStyleSpan {
                        source_unit_start: change.source_span().unit_start(),
                        source_unit_end: change.source_span().unit_end(),
                        style,
                    });
                }
            }
        }
        Self {
            section_present: section.style_start() < bytes_len && !section.events().is_empty(),
            content_unit_count: section.content_unit_count(),
            style_start: section.style_start(),
            event_count: section.events().len(),
            truncated: section.truncated(),
            diagnostics: section.diagnostics().to_vec(),
            spans,
        }
    }

    pub fn style_at_unit(&self, source_unit: usize) -> Option<DocumentTextResolvedStyle> {
        self.spans
            .iter()
            .find(|span| {
                span.source_unit_start <= source_unit && source_unit < span.source_unit_end
            })
            .map(|span| span.style)
    }

    pub fn uniform_value_in_range(
        &self,
        source_unit_start: usize,
        source_unit_end: usize,
        property_id: u8,
    ) -> Option<DocumentTextStyleTypedValue> {
        if source_unit_start >= source_unit_end {
            return None;
        }
        let expected = self.style_at_unit(source_unit_start)?.value(property_id)?;
        let mut covered_until = source_unit_start;
        for span in self.spans.iter().filter(|span| {
            span.source_unit_end > source_unit_start && span.source_unit_start < source_unit_end
        }) {
            if span.source_unit_start > covered_until
                || span.style.value(property_id) != Some(expected)
            {
                return None;
            }
            covered_until = covered_until.max(span.source_unit_end.min(source_unit_end));
            if covered_until == source_unit_end {
                return Some(expected);
            }
        }
        None
    }

    pub fn section_present(&self) -> bool {
        self.section_present
    }

    pub fn content_unit_count(&self) -> u32 {
        self.content_unit_count
    }

    pub fn style_start(&self) -> usize {
        self.style_start
    }

    pub fn event_count(&self) -> usize {
        self.event_count
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn diagnostics(&self) -> &[DocumentTextStyleDiagnostic] {
        &self.diagnostics
    }
}
