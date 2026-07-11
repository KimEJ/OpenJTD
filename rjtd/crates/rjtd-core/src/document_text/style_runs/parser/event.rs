use super::super::super::DocumentTextSourceSpan;
use super::super::types::{
    DocumentTextStyleDiagnostic, DocumentTextStyleDiagnosticKind, DocumentTextStyleProperty,
    DocumentTextStylePropertyChangeEvent, DocumentTextStyleRunEvent,
};

const SOURCE_UNIT_BIAS: usize = 16;

pub(super) fn parse_run_event(
    data: &[u8],
    index: usize,
    cursor: usize,
    diagnostics: &mut Vec<DocumentTextStyleDiagnostic>,
) -> Result<(DocumentTextStyleRunEvent, usize, usize), DocumentTextStyleDiagnosticKind> {
    let bytes = data
        .get(index..index + 5)
        .ok_or(DocumentTextStyleDiagnosticKind::TruncatedRun)?;
    let length = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
    if length == 0 {
        return Err(DocumentTextStyleDiagnosticKind::ZeroLengthRun);
    }
    let consumed_units =
        usize::try_from(length).map_err(|_| DocumentTextStyleDiagnosticKind::CursorOverflow)?;
    let source_span = normalized_source_span(cursor, consumed_units, index, diagnostics)
        .ok_or(DocumentTextStyleDiagnosticKind::CursorOverflow)?;
    let raw_bytes = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]];
    Ok((
        DocumentTextStyleRunEvent::new(source_span, index, length, raw_bytes),
        index + 5,
        consumed_units,
    ))
}

pub(super) fn parse_property_change_event(
    data: &[u8],
    start: usize,
    cursor: usize,
    diagnostics: &mut Vec<DocumentTextStyleDiagnostic>,
) -> Result<(DocumentTextStylePropertyChangeEvent, usize), DocumentTextStyleDiagnosticKind> {
    let source_span = normalized_source_span(cursor, 1, start, diagnostics)
        .ok_or(DocumentTextStyleDiagnosticKind::CursorOverflow)?;
    let mut index = start + 1;
    let mut properties = Vec::new();

    loop {
        if let Some([0xff, 0x00]) = data.get(index..index + 2) {
            let end = index + 2;
            return Ok((
                DocumentTextStylePropertyChangeEvent::new(
                    source_span,
                    start,
                    end,
                    properties,
                    data[start..end].to_vec(),
                ),
                end,
            ));
        }

        let (&property_id, &value_len) = data
            .get(index)
            .zip(data.get(index + 1))
            .ok_or(DocumentTextStyleDiagnosticKind::TruncatedProperty)?;
        let value_len = usize::from(value_len);
        let value_start = index + 2;
        let value_end = value_start + value_len;
        let raw_value = data
            .get(value_start..value_end)
            .ok_or(DocumentTextStyleDiagnosticKind::TruncatedPropertyValue)?;
        properties.push(DocumentTextStyleProperty::new(
            index,
            property_id,
            raw_value.to_vec(),
        ));
        index = value_end;
        if index > data.len() {
            return Err(DocumentTextStyleDiagnosticKind::TruncatedPropertyTerminator);
        }
    }
}

fn normalized_source_span(
    cursor: usize,
    consumed_units: usize,
    offset: usize,
    diagnostics: &mut Vec<DocumentTextStyleDiagnostic>,
) -> Option<DocumentTextSourceSpan> {
    let Some(unit_start) = SOURCE_UNIT_BIAS.checked_add(cursor) else {
        diagnostics.push(DocumentTextStyleDiagnostic::new(
            offset,
            DocumentTextStyleDiagnosticKind::CursorOverflow,
        ));
        return None;
    };
    let Some(unit_end) = unit_start.checked_add(consumed_units) else {
        diagnostics.push(DocumentTextStyleDiagnostic::new(
            offset,
            DocumentTextStyleDiagnosticKind::CursorOverflow,
        ));
        return None;
    };
    Some(DocumentTextSourceSpan::new(unit_start, unit_end))
}
