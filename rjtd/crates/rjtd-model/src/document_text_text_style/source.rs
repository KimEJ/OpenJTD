use crate::{Document, TextSourceSpan};
use rjtd_core::document_text::{DocumentTextStyleResolver, DocumentTextStyleTypedValue};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, StyleStreamRecordLayout, summarize_style_stream,
};

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

pub(crate) fn document_default_font_size_mm100(document: &Document) -> Option<u16> {
    let payload = document_view_style_payload(document, 0x1006)?;
    if !matches!(payload.len(), 20 | 21) {
        return None;
    }
    if payload.get(..3) != Some(&[0x1f, 0, 0]) {
        return None;
    }
    valid_font_size_mm100(u16::from_be_bytes([payload[3], payload[4]]))
}

fn valid_font_size_mm100(size_mm100: u16) -> Option<u16> {
    if matches!(size_mm100, 0 | 0xfffd..=0xffff) {
        return None;
    }
    Some(size_mm100)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceFontSize {
    Default,
    Mm100(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCharacterStyle<'a> {
    pub(super) flags: Option<u32>,
    pub(super) underline: bool,
    pub(super) script: Option<&'static str>,
    pub(super) script_basis: Option<&'static str>,
    pub(super) font: Option<(u16, &'a str)>,
}

pub(super) fn document_text_source_font_size(
    resolver: &DocumentTextStyleResolver,
    source_span: &TextSourceSpan,
) -> Option<SourceFontSize> {
    let value = resolver.uniform_optional_value_in_range(
        source_span.unit_start(),
        source_span.unit_end(),
        2,
    )?;
    match value {
        Some(DocumentTextStyleTypedValue::U16(0)) | None => Some(SourceFontSize::Default),
        Some(DocumentTextStyleTypedValue::U16(size_mm100)) => {
            Some(SourceFontSize::Mm100(valid_font_size_mm100(size_mm100)?))
        }
        _ => None,
    }
}

pub(super) fn document_text_source_character_style<'a>(
    document: &'a Document,
    resolver: &DocumentTextStyleResolver,
    span: &TextSourceSpan,
) -> SourceCharacterStyle<'a> {
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
    let script_basis = if script.is_none() && flags == Some(0x8000_0000) {
        crate::linked_footnote_marker_script_basis(document, resolver, span)
    } else {
        None
    };
    let script = script.or(script_basis.map(|_| "super"));
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
        Some((id, font.name()))
    });
    SourceCharacterStyle {
        flags,
        underline: flags == Some(0x8000_0010)
            && value(13) == Some(Some(DocumentTextStyleTypedValue::U16(1))),
        script,
        script_basis,
        font,
    }
}

pub(super) fn document_text_packed_bgr(
    resolver: &DocumentTextStyleResolver,
    source_span: &TextSourceSpan,
) -> Option<u32> {
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
    Some(bgr)
}

fn document_text_style_resolver_for_span(
    document: &Document,
    span: &TextSourceSpan,
) -> Option<DocumentTextStyleResolver> {
    if span.unit_start() >= span.unit_end()
        || span.unit_start().checked_mul(2)? != span.byte_start()
        || span.unit_end().checked_mul(2)? != span.byte_end()
    {
        return None;
    }
    let mut streams = document
        .raw_streams()
        .iter()
        .filter(|stream| stream.name() == "/DocumentText");
    let bytes = streams.next()?.bytes();
    if streams.next().is_some() {
        return None;
    }
    let count = usize::try_from(u32::from_be_bytes(bytes.get(28..32)?.try_into().ok()?)).ok()?;
    if span.unit_start() < 16
        || span.unit_end() > 16_usize.checked_add(count)?
        || 32_usize.checked_add(count.checked_mul(2)?)? > bytes.len()
    {
        return None;
    }
    document_text_style_resolver(document)
}

impl Document {
    pub fn default_font_size_mm100_candidate(&self) -> Option<u16> {
        document_default_font_size_mm100(self)
    }
    pub fn text_font_size_source_candidate(&self, span: &TextSourceSpan) -> Option<SourceFontSize> {
        document_text_source_font_size(&document_text_style_resolver_for_span(self, span)?, span)
    }
    /// Uniform bounded source flags and raw font identity; never a CSS family list.
    pub fn text_character_style_source_candidate(
        &self,
        span: &TextSourceSpan,
    ) -> Option<SourceCharacterStyle<'_>> {
        Some(document_text_source_character_style(
            self,
            &document_text_style_resolver_for_span(self, span)?,
            span,
        ))
    }
    pub fn text_foreground_bgr24_candidate(&self, span: &TextSourceSpan) -> Option<u32> {
        document_text_packed_bgr(&document_text_style_resolver_for_span(self, span)?, span)
    }
}
impl SourceCharacterStyle<'_> {
    pub fn flags(&self) -> Option<u32> {
        self.flags
    }
    pub fn underline_candidate(&self) -> bool {
        self.underline
    }
    pub fn script_candidate(&self) -> Option<&'static str> {
        self.script
    }
    pub fn script_basis(&self) -> Option<&'static str> {
        self.script_basis
    }
    pub fn font(&self) -> Option<(u16, &str)> {
        self.font
    }
}
