mod parser;
mod resolver;
mod types;

pub use parser::parse_document_text_style_section;
pub use resolver::{DocumentTextResolvedStyle, DocumentTextStyleResolver};
pub use types::{
    DocumentTextStyleDiagnostic, DocumentTextStyleDiagnosticKind, DocumentTextStyleEvent,
    DocumentTextStyleProperty, DocumentTextStylePropertyChangeEvent, DocumentTextStyleRunEvent,
    DocumentTextStyleSection, DocumentTextStyleTypedValue, document_text_style_code_name,
};

#[cfg(test)]
mod fixture_tests;

#[cfg(test)]
mod resolver_tests;

#[cfg(test)]
mod tests;
