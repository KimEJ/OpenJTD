use crate::{
    DocumentTextFieldCandidate, DocumentTextFieldKind, PageLayerTextFragment, TextSourceSpan,
    field_style_span,
};

pub(crate) fn apply_print_date(
    fragment: &mut PageLayerTextFragment,
    fields: &[DocumentTextFieldCandidate],
    date: Option<&str>,
) {
    if let Some(date) = date
        && fields.iter().any(|field| {
            field.kind() == DocumentTextFieldKind::PrintingDate
                && fragment.source_span.as_ref() == Some(field.value_span())
                && fragment.text == field.cached_value()
        })
    {
        fragment.text = date.to_string();
    }
}

pub(crate) fn field_for_span<'a>(
    fields: &'a [DocumentTextFieldCandidate],
    span: &TextSourceSpan,
) -> Option<&'a DocumentTextFieldCandidate> {
    fields.iter().find(|field| {
        field.value_span().unit_start() <= span.unit_start()
            && span.unit_end() <= field.value_span().unit_end()
    })
}

pub(crate) fn field_paint_span(
    field: Option<&DocumentTextFieldCandidate>,
    span: Option<&TextSourceSpan>,
) -> Option<TextSourceSpan> {
    field
        .filter(|field| field.kind() == DocumentTextFieldKind::Hyperlink)
        .map(field_style_span)
        .or_else(|| span.cloned())
}
