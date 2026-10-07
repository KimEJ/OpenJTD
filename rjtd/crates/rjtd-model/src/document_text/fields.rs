use crate::{
    Document, DocumentTextFlowEvent, DocumentTextFlowKind, TextSourceSpan, native_visible_text_span,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentTextFieldKind {
    PrintingDate,
    PageNumber,
    Hyperlink,
}

impl DocumentTextFieldKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PrintingDate => "printingDate",
            Self::PageNumber => "pageNumber",
            Self::Hyperlink => "hyperlink",
        }
    }
}

/// Bounded source field/cache binding; other field profiles remain raw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentTextFieldCandidate {
    kind: DocumentTextFieldKind,
    record_span: TextSourceSpan,
    value_span: TextSourceSpan,
    value: String,
    argument: String,
}

impl DocumentTextFieldCandidate {
    pub fn kind(&self) -> DocumentTextFieldKind {
        self.kind
    }
    pub fn record_span(&self) -> &TextSourceSpan {
        &self.record_span
    }
    pub fn value_span(&self) -> &TextSourceSpan {
        &self.value_span
    }
    pub fn cached_value(&self) -> &str {
        &self.value
    }
    pub fn argument(&self) -> &str {
        &self.argument
    }
}

fn field_group(events: &[DocumentTextFlowEvent], index: u16) -> Option<&DocumentTextFlowEvent> {
    let [start, prefix, value, suffix] = events else {
        return None;
    };
    let visible = index == 0;
    if start.code() != Some(0x1c)
        || start.kind() != DocumentTextFlowKind::Control
        || prefix.kind() != DocumentTextFlowKind::Opaque
        || prefix.raw_words() != [1, 7, 0, index, u16::from(visible)]
        || value.kind()
            != if visible {
                DocumentTextFlowKind::Inline
            } else {
                DocumentTextFlowKind::SkippedInline
            }
        || value.selector() != Some(u16::from(visible))
        || suffix.kind() != DocumentTextFlowKind::Opaque
        || suffix.raw_words() != [5, 0, 1, 0x1f]
        || !events
            .windows(2)
            .all(|pair| pair[0].unit_end() == pair[1].unit_start())
    {
        return None;
    }
    Some(value)
}

fn field_kind(event: &DocumentTextFlowEvent) -> Option<DocumentTextFieldKind> {
    let words = event.raw_words();
    if event.record_class() != Some(0) {
        return None;
    }
    match words.len() {
        28 if words[..10] == [0x1c, 0, 28, 0, 0x33, 110, 1039, 0x98, 6, 0xffff]
            && words[24..] == [28, 0, 0, 0x1f] =>
        {
            Some(DocumentTextFieldKind::PrintingDate)
        }
        15 if words
            == [
                0x1c, 0, 15, 0, 0x35, 50, 1165, 0x90, 0, 0, 0, 15, 0, 0, 0x1f,
            ] =>
        {
            Some(DocumentTextFieldKind::PageNumber)
        }
        12 if words == [0x1c, 0, 12, 0, 0x48, 0, 805, 1, 12, 0, 0, 0x1f] => {
            Some(DocumentTextFieldKind::Hyperlink)
        }
        _ => None,
    }
}

impl Document {
    pub fn text_field_candidates(&self) -> Vec<DocumentTextFieldCandidate> {
        let Some(flow) = self.document_text_flow() else {
            return Vec::new();
        };
        let events = flow.events();
        events
            .iter()
            .enumerate()
            .filter_map(|(index, record)| {
                let kind = field_kind(record)?;
                let value_group = events.get(index + 1..index + 5)?;
                let value = field_group(value_group, 0)?;
                let argument_group = events.get(index + 5..index + 9)?;
                let argument = field_group(argument_group, 1)?;
                if record.unit_end() != value_group[0].unit_start()
                    || value_group[3].unit_end() != argument_group[0].unit_start()
                {
                    return None;
                }
                match kind {
                    DocumentTextFieldKind::PrintingDate
                        if argument.text() == "DATE"
                            && (value.text() == "0000/00/00" || valid_print_date(value.text())) => {
                    }
                    DocumentTextFieldKind::PageNumber
                        if argument.text() == "PAGENUMBER" && valid_page_cache(value.text()) => {}
                    DocumentTextFieldKind::Hyperlink if valid_http_target(argument.text()) => {
                        let tail = events.get(index + 9..index + 13)?;
                        if argument_group[3].unit_end() != tail[0].unit_start()
                            || !field_group(tail, 2)?.text().is_empty()
                        {
                            return None;
                        }
                    }
                    _ => return None,
                }
                if value.text().is_empty() {
                    return None;
                }
                Some(DocumentTextFieldCandidate {
                    kind,
                    record_span: record.source_span().clone(),
                    value_span: native_visible_text_span(self, value.text(), value.source_span())?,
                    value: value.text().to_string(),
                    argument: argument.text().to_string(),
                })
            })
            .collect()
    }
}

fn valid_page_cache(value: &str) -> bool {
    value
        .strip_prefix("- ")
        .and_then(|value| value.strip_suffix(" -"))
        .is_some_and(|value| !value.is_empty() && value.chars().all(|c| c.is_ascii_digit()))
}

fn valid_http_target(value: &str) -> bool {
    value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"))
        .is_some_and(|rest| {
            !rest
                .split(['/', '?', '#'])
                .next()
                .unwrap_or_default()
                .is_empty()
        })
        && !value.chars().any(|c| c.is_control() || c.is_whitespace())
}

pub(crate) fn valid_print_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'/'
        || bytes[7] != b'/'
        || bytes
            .iter()
            .enumerate()
            .any(|(i, b)| i != 4 && i != 7 && !b.is_ascii_digit())
    {
        return false;
    }
    let year = value[..4].parse::<u32>().unwrap_or(0);
    let month = value[5..7].parse::<u32>().unwrap_or(0);
    let day = value[8..].parse::<u32>().unwrap_or(0);
    let limit = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            29
        }
        2 => 28,
        _ => 0,
    };
    year > 0 && day > 0 && day <= limit
}

pub(crate) fn field_style_span(field: &DocumentTextFieldCandidate) -> TextSourceSpan {
    field.record_span.subspan_by_units(0, 1)
}

pub(crate) fn native_field_end(
    fields: &[DocumentTextFieldCandidate],
    events: &[DocumentTextFlowEvent],
    start: usize,
) -> Option<usize> {
    let field = fields
        .iter()
        .find(|field| field.record_span.unit_start() == start)?;
    let count = if field.kind == DocumentTextFieldKind::Hyperlink {
        13
    } else {
        9
    };
    let index = events
        .iter()
        .position(|event| event.unit_start() == start)?;
    Some(events.get(index + count - 1)?.unit_end())
}
