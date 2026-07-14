mod event;

use super::types::{
    DocumentTextStyleDiagnostic, DocumentTextStyleDiagnosticKind, DocumentTextStyleEvent,
    DocumentTextStyleSection,
};
use event::{parse_property_change_event, parse_run_event};

const CONTENT_UNIT_COUNT_OFFSET: usize = 28;
const STYLE_SECTION_BASE_OFFSET: usize = 32;

pub fn parse_document_text_style_section(data: &[u8]) -> DocumentTextStyleSection {
    let mut diagnostics = Vec::new();
    let content_unit_count = match read_be_u32(data, CONTENT_UNIT_COUNT_OFFSET) {
        Some(count) => count,
        None => {
            diagnostics.push(DocumentTextStyleDiagnostic::new(
                CONTENT_UNIT_COUNT_OFFSET,
                DocumentTextStyleDiagnosticKind::HeaderTooShort,
            ));
            0
        }
    };
    let style_start = match usize::try_from(content_unit_count)
        .ok()
        .and_then(|count| count.checked_mul(2))
        .and_then(|count| STYLE_SECTION_BASE_OFFSET.checked_add(count))
    {
        Some(offset) => offset,
        None => {
            diagnostics.push(DocumentTextStyleDiagnostic::new(
                CONTENT_UNIT_COUNT_OFFSET,
                DocumentTextStyleDiagnosticKind::StyleStartOverflow,
            ));
            data.len()
        }
    };
    if style_start > data.len() {
        diagnostics.push(DocumentTextStyleDiagnostic::new(
            style_start,
            DocumentTextStyleDiagnosticKind::StyleStartPastEnd,
        ));
        return DocumentTextStyleSection::new(
            content_unit_count,
            style_start,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            true,
            diagnostics,
        );
    }

    let mut events = Vec::new();
    let mut terminal_bytes = Vec::new();
    let mut trailing_bytes = Vec::new();
    let mut truncated = false;
    let mut cursor = 0usize;
    let mut index = style_start;
    let content_unit_limit = usize::try_from(content_unit_count)
        .ok()
        .unwrap_or(usize::MAX);

    while index < data.len() && events.is_empty() && cursor < content_unit_limit {
        match data[index] {
            0x00 | 0xfe => break,
            0xff => {
                terminal_bytes.push(0xff);
                trailing_bytes.extend_from_slice(&data[index + 1..]);
                return DocumentTextStyleSection::new(
                    content_unit_count,
                    style_start,
                    events,
                    terminal_bytes,
                    trailing_bytes,
                    truncated,
                    diagnostics,
                );
            }
            byte => {
                terminal_bytes.push(byte);
                index += 1;
            }
        }
    }

    while index < data.len() && cursor < content_unit_limit {
        match data[index] {
            0x00 => match parse_run_event(data, index, cursor, &mut diagnostics) {
                Ok((event, next_index, consumed_units)) => {
                    let next_cursor =
                        match checked_cursor_advance(cursor, consumed_units, content_unit_limit) {
                            Ok(next_cursor) => next_cursor,
                            Err(kind) => {
                                push_truncated(
                                    &mut diagnostics,
                                    &mut trailing_bytes,
                                    &mut truncated,
                                    index,
                                    data,
                                    kind,
                                );
                                break;
                            }
                        };
                    events.push(DocumentTextStyleEvent::Run(event));
                    cursor = next_cursor;
                    index = next_index;
                }
                Err(kind) => {
                    push_truncated(
                        &mut diagnostics,
                        &mut trailing_bytes,
                        &mut truncated,
                        index,
                        data,
                        kind,
                    );
                    break;
                }
            },
            0xfe => match parse_property_change_event(data, index, cursor, &mut diagnostics) {
                Ok((event, next_index)) => {
                    let next_cursor = match checked_cursor_advance(cursor, 1, content_unit_limit) {
                        Ok(next_cursor) => next_cursor,
                        Err(kind) => {
                            push_truncated(
                                &mut diagnostics,
                                &mut trailing_bytes,
                                &mut truncated,
                                index,
                                data,
                                kind,
                            );
                            break;
                        }
                    };
                    events.push(DocumentTextStyleEvent::PropertyChange(event));
                    cursor = next_cursor;
                    index = next_index;
                }
                Err(kind) => {
                    push_truncated(
                        &mut diagnostics,
                        &mut trailing_bytes,
                        &mut truncated,
                        index,
                        data,
                        kind,
                    );
                    break;
                }
            },
            0xff => {
                terminal_bytes.push(0xff);
                trailing_bytes.extend_from_slice(&data[index + 1..]);
                break;
            }
            _ => {
                diagnostics.push(DocumentTextStyleDiagnostic::new(
                    index,
                    DocumentTextStyleDiagnosticKind::UnexpectedMarker,
                ));
                trailing_bytes.extend_from_slice(&data[index..]);
                truncated = true;
                break;
            }
        }
    }

    if cursor == content_unit_limit {
        trailing_bytes.extend_from_slice(&data[index..]);
    }

    DocumentTextStyleSection::new(
        content_unit_count,
        style_start,
        events,
        terminal_bytes,
        trailing_bytes,
        truncated,
        diagnostics,
    )
}

fn checked_cursor_advance(
    cursor: usize,
    consumed_units: usize,
    content_unit_limit: usize,
) -> Result<usize, DocumentTextStyleDiagnosticKind> {
    let next_cursor = cursor
        .checked_add(consumed_units)
        .ok_or(DocumentTextStyleDiagnosticKind::CursorOverflow)?;
    if next_cursor > content_unit_limit {
        return Err(DocumentTextStyleDiagnosticKind::CursorPastContentEnd);
    }
    Ok(next_cursor)
}

fn push_truncated(
    diagnostics: &mut Vec<DocumentTextStyleDiagnostic>,
    trailing_bytes: &mut Vec<u8>,
    truncated: &mut bool,
    offset: usize,
    data: &[u8],
    kind: DocumentTextStyleDiagnosticKind,
) {
    diagnostics.push(DocumentTextStyleDiagnostic::new(offset, kind));
    trailing_bytes.extend_from_slice(&data[offset..]);
    *truncated = true;
}

fn read_be_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset + 4)?;
    Some(u32::from_be_bytes(bytes.try_into().ok()?))
}
