mod parser;
mod types;

pub use parser::parse_document_text_row_headers;
pub use types::{
    DocumentTextRowHeaderFixedFields, DocumentTextRowHeaderPair,
    DocumentTextRowHeaderPairClassification, DocumentTextRowHeaderRecord,
};
