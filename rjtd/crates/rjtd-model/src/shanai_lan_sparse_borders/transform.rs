use super::{types::*, *};

pub(super) fn shanai_lan_source_page_transform_candidate(
    document: &Document,
    line_mark_record_indexes: &[usize],
) -> Option<ShanaiLanSourcePageTransformCandidate> {
    let style_payload = document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))?
        .payload();
    page_layout_from_document_view_styles(style_payload)?;
    let page_width_mm100 =
        read_be32_at(style_payload, DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET)? >> 8;
    let page_height_mm100 =
        read_be32_at(style_payload, DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET)? >> 8;
    let x_origin_left_raw = read_be32_at(style_payload, DOCUMENT_VIEW_STYLES_X_ORIGIN_LEFT_OFFSET)?;
    let y_origin_raw = read_be32_at(style_payload, DOCUMENT_VIEW_STYLES_Y_ORIGIN_OFFSET)?;
    let x_origin_right_raw =
        read_be32_at(style_payload, DOCUMENT_VIEW_STYLES_X_ORIGIN_RIGHT_OFFSET)?;
    if x_origin_left_raw & 0xff != 0 || y_origin_raw & 0xff != 0 || x_origin_right_raw & 0xff != 0 {
        return None;
    }

    let page_mark_entry =
        shanai_lan_page_mark_entry_covering_line_mark_records(document, line_mark_record_indexes)?;
    shanai_lan_source_page_transform_candidate_from_raw_fields(
        page_mark_entry.row_index(),
        page_width_mm100,
        page_height_mm100,
        x_origin_left_raw,
        y_origin_raw,
        x_origin_right_raw,
        page_mark_entry.u16_fields(),
    )
}

pub(crate) fn shanai_lan_source_page_transform_candidate_from_raw_fields(
    page_mark_entry_index: usize,
    page_width_mm100: u32,
    page_height_mm100: u32,
    x_origin_left_raw: u32,
    y_origin_raw: u32,
    x_origin_right_raw: u32,
    page_mark_u16_fields: &[u16],
) -> Option<ShanaiLanSourcePageTransformCandidate> {
    if x_origin_left_raw & 0xff != 0 || y_origin_raw & 0xff != 0 || x_origin_right_raw & 0xff != 0 {
        return None;
    }

    let x_origin_left_mm100 = x_origin_left_raw >> 8;
    let y_origin_mm100 = y_origin_raw >> 8;
    let x_origin_right_mm100 = x_origin_right_raw >> 8;
    if x_origin_left_mm100 == 0
        || y_origin_mm100 == 0
        || x_origin_left_mm100 != x_origin_right_mm100
        || x_origin_left_mm100 > page_width_mm100
        || y_origin_mm100 > page_height_mm100
        || page_mark_u16_fields.len() <= 14
    {
        return None;
    }

    let row_pitch_addend_a_mm100 = page_mark_u16_fields[13];
    let row_pitch_addend_b_mm100 = page_mark_u16_fields[14];
    let row_pitch_mm100 = u32::from(row_pitch_addend_a_mm100) + u32::from(row_pitch_addend_b_mm100);
    if row_pitch_mm100 == 0 || row_pitch_mm100 > page_height_mm100 || row_pitch_mm100 > 5_000 {
        return None;
    }

    Some(ShanaiLanSourcePageTransformCandidate {
        page_mark_entry_index,
        page_width_mm100,
        page_height_mm100,
        x_origin_left_mm100,
        x_origin_right_mm100,
        y_origin_mm100,
        row_pitch_addend_a_mm100,
        row_pitch_addend_b_mm100,
        row_pitch_mm100,
        page_mark_w21_mm100: page_mark_u16_fields.get(21).copied(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_transform_requires_a_page_mark_entry_covering_line_records() {
        let mut view_styles = vec![0; DOCUMENT_VIEW_STYLES_X_ORIGIN_RIGHT_OFFSET + 4];
        view_styles
            [DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET..DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET + 4]
            .copy_from_slice(&(29_700_u32 << 8).to_be_bytes());
        view_styles
            [DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET..DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET + 4]
            .copy_from_slice(&(21_000_u32 << 8).to_be_bytes());
        view_styles[DOCUMENT_VIEW_STYLES_X_ORIGIN_LEFT_OFFSET
            ..DOCUMENT_VIEW_STYLES_X_ORIGIN_LEFT_OFFSET + 4]
            .copy_from_slice(&(1_140_u32 << 8).to_be_bytes());
        view_styles[DOCUMENT_VIEW_STYLES_Y_ORIGIN_OFFSET..DOCUMENT_VIEW_STYLES_Y_ORIGIN_OFFSET + 4]
            .copy_from_slice(&(2_130_u32 << 8).to_be_bytes());
        view_styles[DOCUMENT_VIEW_STYLES_X_ORIGIN_RIGHT_OFFSET
            ..DOCUMENT_VIEW_STYLES_X_ORIGIN_RIGHT_OFFSET + 4]
            .copy_from_slice(&(1_140_u32 << 8).to_be_bytes());

        let mut page_mark_bytes = Vec::new();
        page_mark_bytes.extend_from_slice(&19_u32.to_be_bytes());
        page_mark_bytes.extend_from_slice(&0x10_u32.to_be_bytes());
        page_mark_bytes.extend_from_slice(&18_u32.to_be_bytes());
        for index in 0..20_u32 {
            let mut entry = [0; 84];
            entry[0..4].copy_from_slice(&index.to_be_bytes());
            if index == 0 {
                entry[26..28].copy_from_slice(&370_u16.to_be_bytes());
                entry[28..30].copy_from_slice(&105_u16.to_be_bytes());
            }
            page_mark_bytes.extend_from_slice(&entry);
        }
        let page_mark = rjtd_core::layout_mark::parse_page_mark(&page_mark_bytes).unwrap();

        let mut document = Document::default();
        document.push_unknown_style(UnknownStyle::from_stream(
            DOCUMENT_VIEW_STYLES_PATH,
            view_styles,
        ));
        document.push_page_mark(DocumentPageMark::from_page_mark(PAGE_MARK_PATH, &page_mark));

        assert!(
            shanai_lan_source_page_transform_candidate(&document, &[]).is_none(),
            "a plausible first PageMark entry must not replace explicit line-record coverage"
        );
    }
}
