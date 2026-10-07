use crate::DocumentTextFlowEvent;
#[cfg(feature = "rendering")]
use crate::TextSourceSpan;

/// Observed repeated fixed-pitch attributes, kept as candidates in source flow.
pub(crate) fn native_rule_fixed_pitch(event: &DocumentTextFlowEvent) -> Option<u16> {
    let words = event.raw_words();
    (event.record_class() == Some(0x0010)
        && words.len() >= 16
        && words[3..7] == [0, 0x20, 4, 8]
        && words[8] == 8
        && words[7] == words[9]
        && (1..=5_000).contains(&words[7]))
    .then(|| words[7])
    .or_else(|| native_plain_paragraph_fixed_pitch(event))
}

/// Bounded standalone paragraph profile with one saved fixed-pitch field.
/// The zero pair is absent here, not a conflicting repeated table attribute.
pub(crate) fn native_plain_paragraph_fixed_pitch(event: &DocumentTextFlowEvent) -> Option<u16> {
    let words = event.raw_words();
    (event.record_class() == Some(0x0010)
        && words.len() == 16
        && words[..7] == [0x1c, 0x10, 16, 0, 0x20, 4, 8]
        && words[8..] == [0, 0, 0xffff, 0, 16, 0, 0x10, 0x1f]
        && (1..=5_000).contains(&words[7]))
    .then(|| words[7])
}

pub(crate) fn native_rule_parent_offset(event: &DocumentTextFlowEvent) -> Option<usize> {
    if event.record_class() != Some(0x0010) || event.raw_words().get(3) != Some(&0) {
        return None;
    }
    match event.raw_words().get(4) {
        Some(0x008f) => Some(0),
        Some(0x20)
            if native_rule_fixed_pitch(event).is_some()
                && event.raw_words().get(10) == Some(&0x008f) =>
        {
            Some(6)
        }
        _ => None,
    }
}

#[cfg(feature = "rendering")]
pub(crate) fn native_rule_visible_span(text: &str, span: &TextSourceSpan) -> TextSourceSpan {
    let leading = text.chars().take_while(|c| *c == ' ').count();
    if leading == text.chars().count() {
        span.clone()
    } else {
        span.subspan_by_units(leading, text.encode_utf16().count())
    }
}

impl DocumentTextFlowEvent {
    /// Saved fixed-pitch mm100 field in the observed paragraph/table profiles.
    pub fn fixed_pitch_mm100_candidate(&self) -> Option<u16> {
        native_rule_fixed_pitch(self)
    }
    /// Candidate offset of the repeated native rule-parent field, not page placement.
    pub fn rule_parent_offset_candidate(&self) -> Option<usize> {
        native_rule_parent_offset(self)
    }
}
