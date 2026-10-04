use crate::{Document, TextSourceSpan, hundredth_millimeters_to_css_px};
use rjtd_core::document_text::{DocumentTextStyleResolver, DocumentTextStyleTypedValue};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, StyleStreamRecordLayout, summarize_style_stream,
};

pub(crate) const DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS: &str =
    "document-text-style-property-15-text-run-candidate";

pub(crate) fn document_text_style_resolver(
    document: &Document,
) -> Option<DocumentTextStyleResolver> {
    let bytes = document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/DocumentText")?
        .bytes();
    if !bytes.starts_with(b"SsmgV.01") || bytes.get(20..28) != Some(b"TextV.01".as_slice()) {
        return None;
    }
    Some(DocumentTextStyleResolver::from_document_text_bytes(bytes))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DocumentTextFontSize {
    pub(crate) px: f32,
    pub(crate) basis: &'static str,
}

pub(crate) fn document_default_font_size_px(document: &Document) -> Option<f32> {
    let bytes = document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))?
        .payload();
    let summary = summarize_style_stream(bytes);
    if summary.record_layout() != StyleStreamRecordLayout::Sequential {
        return None;
    }
    let record = summary
        .records()
        .iter()
        .find(|record| record.code() == 0x1006)?;
    if !matches!(record.payload_len(), 20 | 21) {
        return None;
    }
    let start = record.offset().checked_add(4)?;
    let payload = bytes.get(start..start.checked_add(record.payload_len())?)?;
    if payload.get(..3) != Some(&[0x1f, 0, 0]) {
        return None;
    }
    font_size_mm100_to_px(u16::from_be_bytes([payload[3], payload[4]]))
}

pub(crate) fn document_text_font_size(
    resolver: &DocumentTextStyleResolver,
    source_span: &TextSourceSpan,
    default_font_size_px: Option<f32>,
) -> Option<DocumentTextFontSize> {
    let value = resolver.uniform_optional_value_in_range(
        source_span.unit_start(),
        source_span.unit_end(),
        2,
    )?;
    match value {
        Some(DocumentTextStyleTypedValue::U16(0)) | None => Some(DocumentTextFontSize {
            px: default_font_size_px?,
            basis: "document-view-style-1006-default",
        }),
        Some(DocumentTextStyleTypedValue::U16(size_mm100)) => Some(DocumentTextFontSize {
            px: font_size_mm100_to_px(size_mm100)?,
            basis: "document-text-style-property-2",
        }),
        _ => None,
    }
}

fn font_size_mm100_to_px(size_mm100: u16) -> Option<f32> {
    if matches!(size_mm100, 0 | 0xfffd..=0xffff) {
        return None;
    };
    // Controlled 10.5pt/14pt text uses 370/494 hundredths of a millimeter.
    // Zero resets to the document style; it is not a zero-size font.
    Some(hundredth_millimeters_to_css_px(u32::from(size_mm100)))
}

pub(crate) fn document_text_foreground_color(
    resolver: &DocumentTextStyleResolver,
    source_span: &TextSourceSpan,
) -> Option<String> {
    let DocumentTextStyleTypedValue::U32(bgr) =
        resolver.uniform_value_in_range(source_span.unit_start(), source_span.unit_end(), 15)?
    else {
        return None;
    };
    // Native black/red/blue and existing green/navy probes agree on BGR24.
    // Automatic-color and unknown high-byte states keep the renderer fallback.
    if bgr & 0xff00_0000 != 0 {
        return None;
    }
    Some(format!(
        "#{:02x}{:02x}{:02x}",
        bgr & 0xff,
        (bgr >> 8) & 0xff,
        (bgr >> 16) & 0xff
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DocumentTextProperty15ColorCandidate {
    pub(crate) packed_bgr: u32,
    pub(crate) css_color: &'static str,
}

pub(crate) fn document_text_property_15_color_candidate(
    resolver: &DocumentTextStyleResolver,
    source_span: &TextSourceSpan,
) -> Option<DocumentTextProperty15ColorCandidate> {
    let DocumentTextStyleTypedValue::U32(packed_bgr) =
        resolver.uniform_value_in_range(source_span.unit_start(), source_span.unit_end(), 15)?
    else {
        return None;
    };
    let css_color = packed_bgr_css_color(packed_bgr)?;
    Some(DocumentTextProperty15ColorCandidate {
        packed_bgr,
        css_color,
    })
}

fn packed_bgr_css_color(packed_bgr: u32) -> Option<&'static str> {
    match packed_bgr {
        0x0000_8000 => Some("#008000"),
        0x0066_0000 => Some("#000066"),
        0x0080_0000 => Some("#000080"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{document_text_font_size, document_text_property_15_color_candidate};
    use crate::TextSourceSpan;
    use rjtd_core::document_text::DocumentTextStyleResolver;

    #[test]
    fn resolves_explicit_font_size_without_treating_resets_or_mixed_ranges_as_sizes() {
        let bytes = synthetic_document_text_with_style_section(
            6,
            &[
                0xfe, 2, 2, 0x01, 0x72, 0xff, 0, 0, 0, 0, 0, 1, 0xfe, 2, 2, 0x01, 0xee, 0xff, 0, 0,
                0, 0, 0, 1, 0xfe, 2, 2, 0, 0, 0xff, 0, 0, 0, 0, 0, 1,
            ],
        );
        let resolver = DocumentTextStyleResolver::from_document_text_bytes(&bytes);
        let span = |start, end| TextSourceSpan::new(start * 2, end * 2, start, end);
        let small = document_text_font_size(&resolver, &span(16, 18), None).unwrap();
        let large = document_text_font_size(&resolver, &span(18, 20), None).unwrap();
        assert!((small.px - 13.984_252).abs() < 0.0001);
        assert!((large.px - 18.670_866).abs() < 0.0001);
        assert_eq!(
            document_text_font_size(&resolver, &span(16, 20), Some(14.0)),
            None
        );
        assert_eq!(
            document_text_font_size(&resolver, &span(20, 22), None),
            None
        );
        assert_eq!(
            document_text_font_size(&resolver, &span(22, 23), Some(14.0)),
            None
        );
        let reset = document_text_font_size(&resolver, &span(20, 22), Some(14.0)).unwrap();
        assert_eq!(reset.px, 14.0);
        assert_eq!(reset.basis, "document-view-style-1006-default");
    }

    #[test]
    fn resolves_property_15_color_only_for_uniform_text_ranges() {
        // Given: a green property-15 state followed by an automatic-color reset.
        let bytes = synthetic_document_text_with_style_section(
            4,
            &[
                0xfe, 0x0f, 0x04, 0x00, 0x00, 0x80, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                0xfe, 0x0f, 0x04, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            ],
        );
        let resolver = DocumentTextStyleResolver::from_document_text_bytes(&bytes);

        // When: exact and cross-boundary text ranges request a color candidate.
        let exact = document_text_property_15_color_candidate(
            &resolver,
            &TextSourceSpan::new(32, 36, 16, 18),
        );
        let crossed = document_text_property_15_color_candidate(
            &resolver,
            &TextSourceSpan::new(32, 40, 16, 20),
        );
        let automatic = document_text_property_15_color_candidate(
            &resolver,
            &TextSourceSpan::new(38, 40, 19, 20),
        );

        // Then: only the uniformly explicit BGR value becomes a CSS color.
        assert_eq!(exact.map(|candidate| candidate.css_color), Some("#008000"));
        assert_eq!(crossed, None);
        assert_eq!(automatic, None);
    }

    fn synthetic_document_text_with_style_section(
        content_unit_count: u32,
        style_bytes: &[u8],
    ) -> Vec<u8> {
        let style_start = 32 + usize::try_from(content_unit_count).ok().unwrap_or(0) * 2;
        let mut bytes = vec![0; style_start];
        bytes[28..32].copy_from_slice(&content_unit_count.to_be_bytes());
        bytes.extend_from_slice(style_bytes);
        bytes
    }
}
