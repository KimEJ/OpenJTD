use super::source::*;
use crate::{Document, TextSourceSpan, hundredth_millimeters_to_css_px};
use rjtd_core::document_text::DocumentTextStyleResolver;

pub(crate) const DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS: &str =
    "document-text-style-property-15-text-run-candidate";

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DocumentTextFontSize {
    pub(crate) px: f32,
    pub(crate) basis: &'static str,
}

pub(crate) fn document_default_font_size_px(document: &Document) -> Option<f32> {
    Some(hundredth_millimeters_to_css_px(u32::from(
        document_default_font_size_mm100(document)?,
    )))
}

pub(crate) fn document_text_font_size(
    resolver: &DocumentTextStyleResolver,
    source_span: &TextSourceSpan,
    default_font_size_px: Option<f32>,
) -> Option<DocumentTextFontSize> {
    match document_text_source_font_size(resolver, source_span)? {
        SourceFontSize::Default => Some(DocumentTextFontSize {
            px: default_font_size_px?,
            basis: "document-view-style-1006-default",
        }),
        SourceFontSize::Mm100(size) => Some(DocumentTextFontSize {
            px: hundredth_millimeters_to_css_px(u32::from(size)),
            basis: "document-text-style-property-2",
        }),
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct DocumentTextCharacterStyle {
    pub(crate) source_unit_start: Option<usize>,
    pub(crate) flags: Option<u32>,
    pub(crate) bold: bool,
    pub(crate) italic: bool,
    pub(crate) underline: bool,
    pub(crate) script: Option<&'static str>,
    pub(crate) script_basis: Option<&'static str>,
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
    let source = document_text_source_character_style(document, resolver, span);
    let font = source.font.map(|(id, name)| {
        let mut names = Vec::new();
        crate::push_font_family_with_aliases(&mut names, name);
        let family = names
            .iter()
            .map(|name| crate::css_font_family_name(name))
            .collect::<Vec<_>>()
            .join(", ");
        (
            id,
            format!("{family}, {}", crate::document_font_family_css(document)),
        )
    });
    DocumentTextCharacterStyle {
        source_unit_start: Some(span.unit_start()),
        flags: source.flags,
        bold: source.flags == Some(0x8400_0000),
        italic: source.flags == Some(0x9000_0000),
        underline: source.underline,
        script: source.script,
        script_basis: source.script_basis,
        font,
    }
}

pub(crate) fn document_text_foreground_color(
    resolver: &DocumentTextStyleResolver,
    source_span: &TextSourceSpan,
) -> Option<String> {
    let bgr = document_text_packed_bgr(resolver, source_span)?;
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
    let packed_bgr = document_text_packed_bgr(resolver, source_span)?;
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
