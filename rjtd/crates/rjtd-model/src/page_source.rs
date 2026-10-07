use crate::{Document, WritingMode, modern_source_writing_mode, read_be16_at, read_be32_at};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, summarize_style_stream,
};

pub(crate) const LAYOUT_BOX_PATH: &str = "/LayoutBox";

pub(crate) const DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET: usize = 16;

pub(crate) const DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET: usize = 20;

pub(crate) const PAGE_LAYOUT_STYLE_RECORD_CODE: u16 = 0x4444;

pub(crate) const PAGE_LAYOUT_STYLE_PAGE_SIZE_SUBRECORD_CODE: u16 = 0x4001;

pub(crate) const PAGE_LAYOUT_STYLE_PAGE_SIZE_WIDTH_OFFSET: usize = 4;

pub(crate) const PAGE_LAYOUT_STYLE_PAGE_SIZE_HEIGHT_OFFSET: usize = 8;

pub(crate) const PAGE_LAYOUT_STYLE_PAYLOAD_WIDTH_OFFSET: usize = 24;

pub(crate) const PAGE_LAYOUT_STYLE_PAYLOAD_HEIGHT_OFFSET: usize = 28;

const MIN_PAPER_SIZE_MM100: u32 = 5_000;
const MAX_PAPER_SIZE_MM100: u32 = 50_000;

pub(crate) fn paper_size_mm100_is_plausible(value: u32) -> bool {
    (MIN_PAPER_SIZE_MM100..=MAX_PAPER_SIZE_MM100).contains(&value)
}

pub(crate) fn page_margins_mm100_at(bytes: &[u8], offset: usize) -> Option<[u16; 4]> {
    let top = read_be16_at(bytes, offset)?;
    let bottom = read_be16_at(bytes, offset + 2)?;
    let left = read_be16_at(bytes, offset + 4)?;
    let right = read_be16_at(bytes, offset + 6)?;
    if [top, bottom, left, right]
        .iter()
        .any(|value| *value >= 0xfffd)
    {
        return None;
    }
    Some([left, right, top, bottom])
}

pub(crate) fn page_size_mm100_from_document_view_styles(bytes: &[u8]) -> Option<(u32, u32)> {
    modern_document_view_size_mm100(bytes).or_else(|| {
        page_size_mm100_from_encoded_shift8(
            read_be32_at(bytes, DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET)?,
            read_be32_at(bytes, DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET)?,
        )
    })
}

pub(crate) fn modern_document_view_size_mm100(bytes: &[u8]) -> Option<(u32, u32)> {
    let summary = summarize_style_stream(bytes);
    if summary.record_layout() != rjtd_core::style_stream::StyleStreamRecordLayout::Sequential {
        return None;
    }
    let records = summary
        .records()
        .iter()
        .filter(|record| record.code() == 0x1001)
        .collect::<Vec<_>>();
    if let [record] = records.as_slice() {
        let start = record.offset().checked_add(4)?;
        let payload = bytes.get(start..start.checked_add(record.payload_len())?)?;
        let extra = match payload.len() {
            258 => 0,
            267 if payload.first() == Some(&14) => 9,
            _ => usize::MAX,
        };
        let prefix = if extra == 9 {
            [1, 4, 1, 0, 0, 0]
        } else {
            [0, 4, 1, 0, 0, 0]
        };
        if extra != usize::MAX && payload.get(extra..extra + 6) == Some(prefix.as_slice()) {
            let le = |offset: usize| {
                payload
                    .get(offset..offset + 4)
                    .map(|value| u32::from_le_bytes(value.try_into().unwrap()))
            };
            let stock = (le(126 + extra)?, le(130 + extra)?);
            if stock == (le(154 + extra)?, le(158 + extra)?) {
                let (width, height) = if extra == 9 {
                    (read_be32_at(payload, 1)?, read_be32_at(payload, 5)?)
                } else {
                    stock
                };
                if let Some(layout) = page_size_mm100_from_encoded_shift8(
                    width.checked_mul(256)?,
                    height.checked_mul(256)?,
                ) {
                    return Some(layout);
                }
            }
        }
    }
    None
}

pub(crate) fn page_size_mm100_from_page_layout_style(bytes: &[u8]) -> Option<(u32, u32)> {
    summarize_style_stream(bytes)
        .records()
        .iter()
        .filter(|record| record.code() == PAGE_LAYOUT_STYLE_RECORD_CODE)
        .find_map(|record| {
            if let Some(layout) = record
                .subrecords()
                .iter()
                .find(|subrecord| subrecord.code() == PAGE_LAYOUT_STYLE_PAGE_SIZE_SUBRECORD_CODE)
                .and_then(|subrecord| {
                    page_size_mm100_from_encoded_shift8(
                        read_be32_at(
                            subrecord.payload(),
                            PAGE_LAYOUT_STYLE_PAGE_SIZE_WIDTH_OFFSET,
                        )?,
                        read_be32_at(
                            subrecord.payload(),
                            PAGE_LAYOUT_STYLE_PAGE_SIZE_HEIGHT_OFFSET,
                        )?,
                    )
                })
            {
                return Some(layout);
            }

            let payload_start = record.offset().checked_add(4)?;
            page_size_mm100_from_encoded_shift8(
                read_be32_at(
                    bytes,
                    payload_start.checked_add(PAGE_LAYOUT_STYLE_PAYLOAD_WIDTH_OFFSET)?,
                )?,
                read_be32_at(
                    bytes,
                    payload_start.checked_add(PAGE_LAYOUT_STYLE_PAYLOAD_HEIGHT_OFFSET)?,
                )?,
            )
        })
}

pub(crate) fn page_size_mm100_from_encoded_shift8(
    width_field: u32,
    height_field: u32,
) -> Option<(u32, u32)> {
    let width_mm100 = width_field >> 8;
    let height_mm100 = height_field >> 8;
    if !paper_size_mm100_is_plausible(width_mm100) || !paper_size_mm100_is_plausible(height_mm100) {
        return None;
    }
    Some((width_mm100, height_mm100))
}

pub(crate) fn document_source_margins_mm100(document: &Document) -> Option<[u16; 4]> {
    let page_style = document
        .unknown_styles()
        .iter()
        .find(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH));
    page_style
        .and_then(|style| {
            let summary = summarize_style_stream(style.payload());
            let mut records = summary
                .records()
                .iter()
                .filter(|r| r.code() == PAGE_LAYOUT_STYLE_RECORD_CODE);
            let record = records.next()?;
            if records.next().is_some() {
                return None;
            }
            let payload = record
                .subrecords()
                .iter()
                .find(|s| s.code() == 0x4002)?
                .payload();
            if !matches!(payload.len(), 39 | 40) || payload.get(..3) != Some(&[0xfe, 0x80, 0]) {
                return None;
            }
            page_margins_mm100_at(payload, 3)
        })
        .or_else(|| {
            // An explicit page-layout margin record has authority over the view defaults.
            if page_style.is_some_and(|style| {
                summarize_style_stream(style.payload())
                    .records()
                    .iter()
                    .any(|r| r.subrecords().iter().any(|s| s.code() == 0x4002))
            }) {
                return None;
            }
            let bytes = document
                .unknown_styles()
                .iter()
                .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))?
                .payload();
            let summary = summarize_style_stream(bytes);
            let record = summary.records().iter().find(|r| r.code() == 0x1002)?;
            let start = record.offset().checked_add(4)?;
            let payload = bytes.get(start..start.checked_add(record.payload_len())?)?;
            let offset = match (payload.len(), payload.get(..2)) {
                (32, Some([0, 0xd8])) => 2,
                (33, Some([0, 0xd8]))
                    if modern_source_writing_mode(document) == Some(WritingMode::VerticalRl) =>
                {
                    2
                }
                (33, Some([0, 0xd9])) if payload.get(2) == Some(&1) => 3,
                _ => return None,
            };
            page_margins_mm100_at(payload, offset)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retains_source_page_units_and_shifted_field_low_bits_without_display_conversion() {
        let width = (21_000 << 8) | 0xa5;
        let height = (29_700 << 8) | 0x5a;
        assert_eq!(
            page_size_mm100_from_encoded_shift8(width, height),
            Some((21_000, 29_700))
        );
        let mut bytes = vec![0; 24];
        bytes[16..20].copy_from_slice(&width.to_be_bytes());
        bytes[20..24].copy_from_slice(&height.to_be_bytes());
        assert_eq!(
            page_size_mm100_from_document_view_styles(&bytes),
            Some((21_000, 29_700))
        );
        assert_eq!(
            page_size_mm100_from_document_view_styles(&bytes[..23]),
            None
        );
        for invalid in [0, 4_999, 50_001, 0x00ff_ffff] {
            assert_eq!(
                page_size_mm100_from_encoded_shift8(invalid << 8, height),
                None
            );
        }
        assert_eq!(
            page_size_mm100_from_encoded_shift8(5_000 << 8, 50_000 << 8),
            Some((5_000, 50_000))
        );
    }

    #[test]
    fn retains_raw_margin_order_and_rejects_sentinels_and_truncation() {
        let mut bytes = vec![0xaa];
        for value in [2_000_u16, 3_000, 1_000, 4_000] {
            bytes.extend(value.to_be_bytes());
        }
        assert_eq!(
            page_margins_mm100_at(&bytes, 1),
            Some([1_000, 4_000, 2_000, 3_000])
        );
        assert_eq!(page_margins_mm100_at(&bytes[..8], 1), None);
        bytes[1..3].copy_from_slice(&0xfffd_u16.to_be_bytes());
        assert_eq!(page_margins_mm100_at(&bytes, 1), None);
        assert_eq!(page_margins_mm100_at(&[0; 8], 0), Some([0; 4]));
    }
}
