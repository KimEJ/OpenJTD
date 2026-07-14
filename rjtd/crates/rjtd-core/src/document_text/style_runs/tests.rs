use super::{
    DocumentTextStyleDiagnosticKind, DocumentTextStyleEvent, DocumentTextStyleSection,
    DocumentTextStyleTypedValue, document_text_style_code_name, parse_document_text_style_section,
};

#[test]
fn parses_textv01_style_events_as_runs_and_one_unit_property_changes() {
    let bytes = synthetic_document_text_with_style_section(
        7,
        &[
            0x00, 0x00, 0x00, 0x00, 0x03, 0xfe, 0x01, 0x02, 0x12, 0x34, 0x04, 0x01, 0x80, 0xff,
            0x00, 0xfe, 0x0f, 0x04, 0x00, 0x00, 0x00, 0x09, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x02,
        ],
    );

    let section = parse_document_text_style_section(&bytes);

    assert_eq!(section.content_unit_count(), 7);
    assert_eq!(section.style_start(), 46);
    assert!(section.terminal_bytes().is_empty());
    assert!(section.trailing_bytes().is_empty());
    assert!(!section.truncated());
    assert_eq!(section.events().len(), 4);

    let run = match &section.events()[0] {
        DocumentTextStyleEvent::Run(run) => run,
        other => panic!("expected run event, got {other:?}"),
    };
    assert_eq!(run.length(), 3);
    assert_eq!(run.source_span().unit_start(), 16);
    assert_eq!(run.source_span().unit_end(), 19);

    let change = match &section.events()[1] {
        DocumentTextStyleEvent::PropertyChange(change) => change,
        other => panic!("expected property change event, got {other:?}"),
    };
    assert_eq!(change.consumed_units(), 1);
    assert_eq!(change.source_span().unit_start(), 19);
    assert_eq!(change.source_span().unit_end(), 20);
    assert_eq!(change.properties().len(), 2);
    assert_eq!(change.properties()[0].property_id(), 1);
    assert_eq!(change.properties()[0].raw_value(), &[0x12, 0x34]);
    assert_eq!(
        change.properties()[0].typed_value(),
        Some(DocumentTextStyleTypedValue::U16(0x1234))
    );
    assert_eq!(change.properties()[1].property_id(), 4);
    assert_eq!(
        change.properties()[1].typed_value(),
        Some(DocumentTextStyleTypedValue::U8(0x80))
    );

    let second_change = match &section.events()[2] {
        DocumentTextStyleEvent::PropertyChange(change) => change,
        other => panic!("expected property change event, got {other:?}"),
    };
    assert_eq!(second_change.source_span().unit_start(), 20);
    assert_eq!(second_change.source_span().unit_end(), 21);
    assert_eq!(second_change.properties().len(), 1);
    assert_eq!(second_change.properties()[0].property_id(), 15);
    assert_eq!(
        second_change.properties()[0].typed_value(),
        Some(DocumentTextStyleTypedValue::U32(9))
    );

    let tail_run = match &section.events()[3] {
        DocumentTextStyleEvent::Run(run) => run,
        other => panic!("expected run event, got {other:?}"),
    };
    assert_eq!(tail_run.length(), 2);
    assert_eq!(tail_run.source_span().unit_start(), 21);
    assert_eq!(tail_run.source_span().unit_end(), 23);
}

#[test]
fn decodes_typed_property_widths_and_style_code_names_conservatively() {
    let bytes = synthetic_document_text_with_style_section(
        3,
        &[
            0xfe, 0x04, 0x02, 0x00, 0x09, 0x0f, 0x04, 0x00, 0xff, 0xcc, 0x99, 0x10, 0x04, 0x00,
            0x33, 0xff, 0xff, 0x04, 0x02, 0x12, 0x34, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ],
    );

    let section = parse_document_text_style_section(&bytes);
    let change = match &section.events()[0] {
        DocumentTextStyleEvent::PropertyChange(change) => change,
        other => panic!("expected property change event, got {other:?}"),
    };

    assert_eq!(change.properties()[0].property_id(), 4);
    assert_eq!(change.properties()[0].expected_width(), Some(1));
    assert_eq!(change.properties()[0].raw_value(), &[0x00, 0x09]);
    assert_eq!(change.properties()[0].typed_value(), None);

    assert_eq!(change.properties()[1].property_id(), 15);
    assert_eq!(
        change.properties()[1].typed_value(),
        Some(DocumentTextStyleTypedValue::U32(0x00ff_cc99))
    );

    assert_eq!(change.properties()[2].property_id(), 16);
    assert_eq!(
        change.properties()[2].typed_value(),
        Some(DocumentTextStyleTypedValue::U32(0x0033_ffff))
    );

    assert_eq!(document_text_style_code_name(1), Some("single"));
    assert_eq!(document_text_style_code_name(8), Some("wave"));
    assert_eq!(document_text_style_code_name(15), Some("bold-wave"));
    assert_eq!(document_text_style_code_name(20), Some("thick-line"));
    assert_eq!(document_text_style_code_name(21), None);
}

#[test]
fn preserves_terminal_bytes_and_truncated_tails_without_panicking() {
    let terminal = parse_document_text_style_section(&synthetic_document_text_with_style_section(
        1,
        &[0xff, 0xaa, 0xbb],
    ));
    assert!(terminal.events().is_empty());
    assert_eq!(terminal.terminal_bytes(), &[0xff]);
    assert_eq!(terminal.trailing_bytes(), &[0xaa, 0xbb]);
    assert!(!terminal.truncated());

    let truncated = parse_document_text_style_section(&synthetic_document_text_with_style_section(
        2,
        &[0x00, 0x00, 0x00, 0x00, 0x01, 0xfe, 0x01, 0x04, 0x00, 0x11],
    ));
    assert_eq!(truncated.events().len(), 1);
    assert_eq!(truncated.terminal_bytes(), &[]);
    assert_eq!(truncated.trailing_bytes(), &[0xfe, 0x01, 0x04, 0x00, 0x11]);
    assert!(truncated.truncated());
    assert!(!truncated.diagnostics().is_empty());
}

#[test]
fn preserves_zero_padding_after_exact_style_coverage() {
    let padding = [0x00, 0x00, 0x00, 0x00, 0x00];
    let section = parse_document_text_style_section(&synthetic_document_text_with_style_section(
        3,
        &[
            0x00, 0x00, 0x00, 0x00, 0x03, padding[0], padding[1], padding[2], padding[3],
            padding[4],
        ],
    ));

    assert_eq!(section.events().len(), 1);
    assert_eq!(style_coverage_units(&section), 3);
    assert_eq!(section.terminal_bytes(), &[]);
    assert_eq!(section.trailing_bytes(), &padding);
    assert!(!section.truncated());
    assert!(section.diagnostics().is_empty());
}

#[test]
fn truncates_and_preserves_overrun_run_event() {
    let style_bytes = [0x00, 0x00, 0x00, 0x00, 0x04, 0xaa];
    let section = parse_document_text_style_section(&synthetic_document_text_with_style_section(
        3,
        &style_bytes,
    ));

    assert!(section.events().is_empty());
    assert_eq!(section.terminal_bytes(), &[]);
    assert_eq!(section.trailing_bytes(), &style_bytes);
    assert!(section.truncated());
    assert_eq!(
        section
            .diagnostics()
            .last()
            .map(|diagnostic| diagnostic.kind()),
        Some(DocumentTextStyleDiagnosticKind::CursorPastContentEnd)
    );
}

fn style_coverage_units(section: &DocumentTextStyleSection) -> usize {
    section
        .events()
        .iter()
        .map(|event| match event {
            DocumentTextStyleEvent::Run(run) => usize::try_from(run.length()).ok().unwrap_or(0),
            DocumentTextStyleEvent::PropertyChange(change) => {
                usize::try_from(change.consumed_units()).ok().unwrap_or(0)
            }
        })
        .sum()
}

fn synthetic_document_text_with_style_section(
    content_unit_count: u32,
    style_bytes: &[u8],
) -> Vec<u8> {
    let style_start = 32 + (content_unit_count as usize * 2);
    let mut bytes = vec![0; style_start];
    bytes[28..32].copy_from_slice(&content_unit_count.to_be_bytes());
    bytes.extend_from_slice(style_bytes);
    bytes
}
