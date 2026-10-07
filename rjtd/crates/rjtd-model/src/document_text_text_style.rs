mod render;
mod source;

pub(crate) use render::*;
pub(crate) use source::document_text_style_resolver;

#[cfg(test)]
mod tests {
    use super::source::{
        SourceFontSize, document_text_source_character_style, document_text_source_font_size,
    };
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
        assert_eq!(
            document_text_source_font_size(&resolver, &span(16, 18)),
            Some(SourceFontSize::Mm100(370))
        );
        assert_eq!(
            document_text_source_font_size(&resolver, &span(18, 20)),
            Some(SourceFontSize::Mm100(494))
        );
        assert_eq!(
            document_text_source_font_size(&resolver, &span(20, 22)),
            Some(SourceFontSize::Default)
        );
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

    #[test]
    fn keeps_the_source_font_name_without_backend_aliases_and_rejects_duplicate_ids() {
        use crate::{Document, DocumentFont};
        let bytes = synthetic_document_text_with_style_section(
            4,
            &[
                0xfe, 3, 2, 0, 2, 0xff, 0, 0, 0, 0, 0, 1, 0xfe, 3, 2, 0xff, 0xff, 0xff, 0, 0, 0, 0,
                0, 1,
            ],
        );
        let resolver = DocumentTextStyleResolver::from_document_text_bytes(&bytes);
        let span = TextSourceSpan::new(32, 36, 16, 18);
        let mut document = Document::from_plain_text("ABCD");
        document.push_font(DocumentFont::new("/Font", 2, 0, "MS Mincho", Vec::new()));
        assert_eq!(
            document_text_source_character_style(&document, &resolver, &span).font,
            Some((2, "MS Mincho"))
        );
        document.push_font(DocumentFont::new("/Font", 2, 10, "duplicate", Vec::new()));
        assert_eq!(
            document_text_source_character_style(&document, &resolver, &span).font,
            None
        );
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
