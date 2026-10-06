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

impl Document {
    /// Controlled 60% spacing association; a general binary-unit formula is unproven.
    pub fn character_spacing_percent_candidate(&self) -> Option<u16> {
        let font = document_view_style_payload(self, 0x1006)?;
        (matches!(font.len(), 20 | 21)
            && font.get(..3) == Some(&[0x1f, 0, 0])
            && font.get(16..18) == Some(&[2, 0x66])
            && document_view_style_payload(self, 0x100b)? == [2, 0, 13, 0, 4, 0, 0, 0, 8])
        .then_some(60)
    }
    /// Bounded western-style candidate; unknown records remain in unknown_styles.
    /// Absence of the optional off value is enabled only in this observed profile.
    pub fn english_justification_candidate(&self) -> Option<bool> {
        let payload = document_view_style_payload(self, 0x100b)?;
        match payload {
            [2, 2, 0x58, 0, 4, 0, 0, 0, 8] => Some(true),
            [2, 2, 0x58, 0x40, 0, 4, 0, 0, 0, 8] => Some(false),
            _ => None,
        }
    }
}

fn document_view_style_payload(document: &Document, code: u16) -> Option<&[u8]> {
    let styles = document
        .unknown_styles()
        .iter()
        .filter(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))
        .collect::<Vec<_>>();
    let [style] = styles.as_slice() else {
        return None;
    };
    let bytes = style.payload();
    let summary = summarize_style_stream(bytes);
    if summary.record_layout() != StyleStreamRecordLayout::Sequential {
        return None;
    }
    let records = summary
        .records()
        .iter()
        .filter(|record| record.code() == code)
        .collect::<Vec<_>>();
    let [record] = records.as_slice() else {
        return None;
    };
    let start = record.offset().checked_add(4)?;
    bytes.get(start..start.checked_add(record.payload_len())?)
}

pub(crate) fn document_default_font_size_px(document: &Document) -> Option<f32> {
    let payload = document_view_style_payload(document, 0x1006)?;
    if !matches!(payload.len(), 20 | 21) {
        return None;
    }
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

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct DocumentTextCharacterStyle {
    pub(crate) source_unit_start: Option<usize>,
    pub(crate) flags: Option<u32>,
    pub(crate) bold: bool,
    pub(crate) italic: bool,
    pub(crate) underline: bool,
    pub(crate) script: Option<&'static str>,
    pub(crate) font: Option<(u16, String)>,
}

impl DocumentTextCharacterStyle {
    pub(crate) fn font_scale(&self) -> f32 {
        if self.script.is_some() { 0.5 } else { 1.0 }
    }

    pub(crate) fn baseline_shift(&self, unscaled_font_size: f32) -> f32 {
        // Quarter-area scripts use half width/height. Align the upper half to
        // the fallback em top and the lower half to the ordinary baseline.
        // Exact glyph ascent and device quantization remain font dependent.
        if self.script == Some("super") {
            -unscaled_font_size * 0.5
        } else {
            0.0
        }
    }
}

pub(crate) fn document_text_character_style(
    document: &Document,
    resolver: &DocumentTextStyleResolver,
    span: &TextSourceSpan,
) -> DocumentTextCharacterStyle {
    let value = |property| {
        resolver.uniform_optional_value_in_range(span.unit_start(), span.unit_end(), property)
    };
    let flags = match value(20) {
        Some(Some(DocumentTextStyleTypedValue::U32(flags)))
            if matches!(
                flags,
                0 | 0x8000_0000
                    | 0x8400_0000
                    | 0x9000_0000
                    | 0x8000_0010
                    | 0x8000_0c00
                    | 0x8000_0400
            ) =>
        {
            Some(flags)
        }
        _ => None,
    };
    let half_size = value(4) == Some(Some(DocumentTextStyleTypedValue::U8(50)))
        && value(5) == Some(Some(DocumentTextStyleTypedValue::U8(50)));
    let script = match (flags, half_size) {
        (Some(0x8000_0c00), true) => Some("super"),
        (Some(0x8000_0400), true) => Some("sub"),
        _ => None,
    };
    let default_font = || {
        let payload = document_view_style_payload(document, 0x1006)?;
        if !matches!(payload.len(), 20 | 21) || payload.get(..3) != Some(&[0x1f, 0, 0]) {
            return None;
        }
        Some(u16::from_be_bytes([payload[5], payload[6]]))
    };
    let font_id = match value(3) {
        Some(Some(DocumentTextStyleTypedValue::U16(0xffff))) | Some(None) => default_font(),
        Some(Some(DocumentTextStyleTypedValue::U16(id))) if id < 0xfffd => Some(id),
        _ => None,
    };
    let font = font_id.and_then(|id| {
        let mut fonts = document
            .fonts()
            .iter()
            .filter(|font| font.source_stream() == "/Font" && font.id() == id);
        let font = fonts.next()?;
        if fonts.next().is_some() || font.name().trim().is_empty() {
            return None;
        }
        let mut names = Vec::new();
        crate::push_font_family_with_aliases(&mut names, font.name());
        let family = names
            .iter()
            .map(|name| crate::css_font_family_name(name))
            .collect::<Vec<_>>()
            .join(", ");
        Some((
            id,
            format!("{family}, {}", crate::document_font_family_css(document)),
        ))
    });
    DocumentTextCharacterStyle {
        source_unit_start: Some(span.unit_start()),
        flags,
        bold: flags == Some(0x8400_0000),
        italic: flags == Some(0x9000_0000),
        underline: flags == Some(0x8000_0010)
            && value(13) == Some(Some(DocumentTextStyleTypedValue::U16(1))),
        script,
        font,
    }
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
    fn distinguishes_supported_english_justification_profiles_without_guessing_other_flags() {
        use crate::{Document, UnknownStyle};
        let candidate = |payload: &[u8], duplicate: bool| {
            let mut bytes = vec![0; 32];
            let records = [
                (0x100b_u16, payload),
                (0x1007, &[1][..]),
                (0x1008, &[2][..]),
                (0x1009, &[3][..]),
            ];
            for (code, value) in records {
                bytes.extend(code.to_be_bytes());
                bytes.extend((value.len() as u16).to_be_bytes());
                bytes.extend(value);
            }
            if duplicate {
                bytes.extend(0x100b_u16.to_be_bytes());
                bytes.extend((payload.len() as u16).to_be_bytes());
                bytes.extend(payload);
            }
            let original = bytes.clone();
            let mut doc = Document::from_plain_text("A B");
            doc.push_unknown_style(UnknownStyle::from_stream("/DocumentViewStyles", bytes));
            let value = doc.english_justification_candidate();
            assert_eq!(doc.unknown_styles()[0].payload(), original);
            value
        };
        assert_eq!(
            candidate(&[2, 2, 0x58, 0, 4, 0, 0, 0, 8], false),
            Some(true)
        );
        assert_eq!(
            candidate(&[2, 2, 0x58, 0x40, 0, 4, 0, 0, 0, 8], false),
            Some(false)
        );
        assert_eq!(
            candidate(&[2, 2, 0x58, 0x40, 1, 4, 0, 0, 0, 8], false),
            None
        );
        assert_eq!(candidate(&[2, 2, 0x58, 0, 4, 0, 0, 0, 8], true), None);
        assert_eq!(
            Document::from_plain_text("A B").english_justification_candidate(),
            None
        );
    }

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
