use crate::TextSourceSpan;
use rjtd_core::document_text::{DocumentTextStyleResolver, DocumentTextStyleTypedValue};

pub(crate) const DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS: &str =
    "document-text-style-property-15-text-run-candidate";

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
    use super::document_text_property_15_color_candidate;
    use crate::TextSourceSpan;
    use rjtd_core::document_text::DocumentTextStyleResolver;

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
