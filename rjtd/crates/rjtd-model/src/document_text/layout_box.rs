use super::*;
use crate::*;

pub(crate) fn layout_box_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<LayoutBoxTextProjection> {
    if page_number != 1 {
        return None;
    }

    let bytes = raw_stream_bytes(document, LAYOUT_BOX_TEXT_PATH)?;
    let blocks = layout_box_text_blocks(bytes);
    if blocks.is_empty() {
        return None;
    }
    let records = raw_stream_bytes(document, LAYOUT_BOX_PATH)
        .map(layout_box_record_candidates)
        .unwrap_or_default();
    let body_anchor = layout_box_body_anchor(&blocks, &records, layout);
    let title_frame_shape = document
        .object_frame_records()
        .iter()
        .find_map(|record| page_frame_title_shape(record, layout));
    let document_text = document_visible_text(document);
    let mut slots = Vec::new();

    for block in &blocks {
        for fragment in &block.fragments {
            let trimmed = fragment.text.trim();
            if trimmed.is_empty() || document_text.contains(trimmed) {
                continue;
            }
            let role = layout_box_text_role(block, &fragment.text);
            match role {
                "body" => {
                    let Some((record, x, y, width, origin_pt)) =
                        layout_box_record_text_box(block.index, &records, layout)
                    else {
                        continue;
                    };
                    let line_height =
                        LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX * LAYOUT_BOX_TEXT_LINE_HEIGHT_FACTOR;
                    for (line_index, line) in layout_box_wrapped_text_lines(
                        &fragment.text,
                        width,
                        LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX,
                    )
                    .into_iter()
                    .enumerate()
                    {
                        if line.trim().is_empty() {
                            continue;
                        }
                        slots.push(LayoutBoxTextSlot {
                            role,
                            text: line,
                            x,
                            y: y + line_index as f32 * line_height,
                            font_size: LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX,
                            line_height,
                            source_span: fragment.source_span.clone(),
                            block_index: block.index,
                            layout_record_index: Some(record.index),
                            layout_record_byte_range: Some((record.byte_start, record.byte_end)),
                            layout_x_pt: record.x_field,
                            layout_y_pt: record.y_field,
                            layout_width_pt: record.width_field,
                            inferred_origin_pt: Some(origin_pt),
                            placement_basis: "layoutBoxRecordFields",
                        });
                    }
                }
                "title" => {
                    let font_size = LAYOUT_BOX_TEXT_TITLE_FONT_SIZE_PX;
                    let text_width = text_width_px_for_font_size(font_size, trimmed) as f32;
                    let (x, y, placement_basis) = if let Some(frame) = &title_frame_shape {
                        (
                            frame.x + ((frame.width - text_width) / 2.0).max(0.0),
                            frame.y + ((frame.height - font_size) / 2.0).max(0.0),
                            "pageFrameTitleCenter",
                        )
                    } else {
                        (
                            ((layout.width_px() - text_width) / 2.0).max(layout.margin_px() * 0.5),
                            layout.margin_px() * 0.56,
                            "shortLeadingLayoutBoxText",
                        )
                    };
                    slots.push(LayoutBoxTextSlot {
                        role,
                        text: trimmed.to_string(),
                        x,
                        y,
                        font_size,
                        line_height: font_size * 1.2,
                        source_span: fragment.source_span.clone(),
                        block_index: block.index,
                        layout_record_index: records.get(block.index).map(|record| record.index),
                        layout_record_byte_range: records
                            .get(block.index)
                            .map(|record| (record.byte_start, record.byte_end)),
                        layout_x_pt: records.get(block.index).and_then(|record| record.x_field),
                        layout_y_pt: records.get(block.index).and_then(|record| record.y_field),
                        layout_width_pt: records
                            .get(block.index)
                            .and_then(|record| record.width_field),
                        inferred_origin_pt: records
                            .get(block.index)
                            .and_then(layout_box_record_origin_pt),
                        placement_basis,
                    });
                }
                "caption" => {
                    let font_size = LAYOUT_BOX_TEXT_CAPTION_FONT_SIZE_PX;
                    let text_width = text_width_px_for_font_size(font_size, trimmed) as f32;
                    let body_line_height =
                        LAYOUT_BOX_TEXT_BODY_FONT_SIZE_PX * LAYOUT_BOX_TEXT_LINE_HEIGHT_FACTOR;
                    let y = body_anchor
                        .map(|(_, body_y, _)| body_y - body_line_height * 2.0)
                        .unwrap_or(layout.height_px() * 0.38);
                    slots.push(LayoutBoxTextSlot {
                        role,
                        text: trimmed.to_string(),
                        x: ((layout.width_px() - text_width) / 2.0).max(layout.margin_px() * 0.5),
                        y,
                        font_size,
                        line_height: font_size * 1.25,
                        source_span: fragment.source_span.clone(),
                        block_index: block.index,
                        layout_record_index: records.get(block.index).map(|record| record.index),
                        layout_record_byte_range: records
                            .get(block.index)
                            .map(|record| (record.byte_start, record.byte_end)),
                        layout_x_pt: records.get(block.index).and_then(|record| record.x_field),
                        layout_y_pt: records.get(block.index).and_then(|record| record.y_field),
                        layout_width_pt: records
                            .get(block.index)
                            .and_then(|record| record.width_field),
                        inferred_origin_pt: records
                            .get(block.index)
                            .and_then(layout_box_record_origin_pt),
                        placement_basis: "relativeToLayoutBoxBodyAnchor",
                    });
                }
                _ => {}
            }
        }
    }

    (!slots.is_empty()).then_some(LayoutBoxTextProjection {
        source: LAYOUT_BOX_TEXT_PATH,
        projection_kind: "layoutBoxTextProjection",
        block_count: blocks.len(),
        layout_record_count: records.len(),
        position_table_present: raw_stream_bytes(document, LAYOUT_BOX_TEXT_POSITION_TABLES_PATH)
            .is_some(),
        page_assignment_decoded: false,
        slots,
    })
}

pub(crate) fn layout_box_text_blocks(bytes: &[u8]) -> Vec<LayoutBoxTextBlock> {
    let mut blocks = Vec::new();
    let mut offset = 0usize;
    while let Some(relative) = bytes[offset..]
        .windows(LAYOUT_BOX_TEXT_MAGIC.len())
        .position(|window| window == LAYOUT_BOX_TEXT_MAGIC)
    {
        let byte_start = offset + relative;
        let Some(declared_unit_count) =
            read_be32_at(bytes, byte_start + LAYOUT_BOX_TEXT_MAGIC.len())
                .and_then(|value| usize::try_from(value).ok())
        else {
            break;
        };
        let payload_start = byte_start + LAYOUT_BOX_TEXT_MAGIC.len() + 4;
        let declared_payload_end =
            payload_start.saturating_add(declared_unit_count.saturating_mul(2));
        let next_magic = bytes
            .get(payload_start..)
            .and_then(|tail| {
                tail.windows(LAYOUT_BOX_TEXT_MAGIC.len())
                    .position(|window| window == LAYOUT_BOX_TEXT_MAGIC)
            })
            .map(|relative| payload_start + relative);
        let payload_end = declared_payload_end
            .min(next_magic.unwrap_or(bytes.len()))
            .min(bytes.len());
        if payload_start > payload_end {
            break;
        }
        let fragments = layout_box_text_fragments(bytes, payload_start, payload_end);
        blocks.push(LayoutBoxTextBlock {
            index: blocks.len(),
            byte_start,
            byte_end: payload_end,
            payload_start,
            payload_end,
            declared_unit_count,
            fragments,
        });
        offset = payload_end.max(byte_start + 1);
    }
    blocks
}

pub(crate) fn layout_box_text_fragments(
    source_bytes: &[u8],
    payload_start: usize,
    payload_end: usize,
) -> Vec<LayoutBoxTextFragment> {
    let Some(payload) = source_bytes.get(payload_start..payload_end) else {
        return Vec::new();
    };
    let mut fragments = map_document_text(payload)
        .entries()
        .iter()
        .filter(|entry| {
            matches!(
                entry.kind(),
                DocumentTextMapKind::TextRun
                    | DocumentTextMapKind::InlineText
                    | DocumentTextMapKind::SkippedInlineText
            ) && !entry.text().trim().is_empty()
        })
        .map(|entry| {
            let byte_start = payload_start + entry.byte_start();
            let byte_end = payload_start + entry.byte_end();
            LayoutBoxTextFragment {
                text: entry.text().trim_end().to_string(),
                source_span: TextSourceSpan::new(
                    byte_start,
                    byte_end,
                    byte_start / 2,
                    byte_end / 2,
                ),
            }
        })
        .collect::<Vec<_>>();
    if !fragments.is_empty() {
        return fragments;
    }

    if let Some((text, byte_start, byte_end)) =
        decode_plain_layout_box_text_payload(payload, payload_start)
    {
        fragments.push(LayoutBoxTextFragment {
            text,
            source_span: TextSourceSpan::new(byte_start, byte_end, byte_start / 2, byte_end / 2),
        });
    }
    fragments
}

pub(crate) fn decode_plain_layout_box_text_payload(
    payload: &[u8],
    payload_start: usize,
) -> Option<(String, usize, usize)> {
    let mut text = String::new();
    let mut first_text_unit = None;
    let mut last_text_unit = None;
    for (unit_index, chunk) in payload.chunks_exact(2).enumerate() {
        let code = u16::from_be_bytes([chunk[0], chunk[1]]);
        let character = match code {
            0x0009 => '\t',
            0x000a | 0x000d => '\n',
            0x0020..=0xd7ff | 0xe000..=0xfffd if code != 0xffff => char::from_u32(u32::from(code))?,
            _ => continue,
        };
        if !matches!(character, '\0') {
            first_text_unit.get_or_insert(unit_index);
            last_text_unit = Some(unit_index + 1);
            text.push(character);
        }
    }
    let text = text
        .trim_end_matches([' ', '\u{3000}', '\n', '\r', '\t'])
        .to_string();
    if text.trim().is_empty() {
        return None;
    }
    let byte_start = payload_start + first_text_unit.unwrap_or_default() * 2;
    let byte_end = payload_start + last_text_unit.unwrap_or_default() * 2;
    Some((text, byte_start, byte_end))
}

pub(crate) fn layout_box_text_role(block: &LayoutBoxTextBlock, text: &str) -> &'static str {
    let trimmed = text.trim();
    if trimmed.chars().count() >= LAYOUT_BOX_TEXT_BODY_MIN_CHARS {
        return "body";
    }
    if block.index == 0 && trimmed.chars().count() <= 32 && !trimmed.contains('\n') {
        return "title";
    }
    if trimmed.contains("より") || trimmed.contains("抜粋") || trimmed.contains('\'') {
        return "caption";
    }
    "label"
}

pub(crate) fn layout_box_record_text_box(
    block_index: usize,
    records: &[LayoutBoxRecordCandidate],
    _layout: PageLayout,
) -> Option<(&LayoutBoxRecordCandidate, f32, f32, f32, f32)> {
    let record = records.get(block_index)?;
    let x = record.x_field?;
    let y = record.y_field?;
    let width = record.width_field?;
    if !(LAYOUT_BOX_TEXT_MIN_RENDER_WIDTH_PT..=LAYOUT_BOX_TEXT_MAX_RENDER_WIDTH_PT).contains(&width)
        || x > LAYOUT_BOX_TEXT_MAX_RENDER_WIDTH_PT
        || y > 1200
    {
        return None;
    }
    let origin_pt = layout_box_record_origin_pt(record).unwrap_or(0.0);
    Some((
        record,
        (f32::from(x) + origin_pt) * PDF_POINT_TO_CSS_PX,
        (f32::from(y) + origin_pt) * PDF_POINT_TO_CSS_PX,
        f32::from(width) * PDF_POINT_TO_CSS_PX,
        origin_pt,
    ))
}

pub(crate) fn layout_box_wrapped_text_lines(
    text: &str,
    width_px: f32,
    font_size: f32,
) -> Vec<String> {
    let max_columns = ((width_px / (font_size * 0.55)).floor() as usize).max(8);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut current = String::new();
        let mut width = 0usize;
        for character in paragraph.trim_end().chars() {
            let char_width = display_column_width(character);
            if width > 0 && width + char_width > max_columns {
                lines.push(std::mem::take(&mut current));
                width = 0;
            }
            current.push(character);
            width += char_width;
        }
        if !current.trim().is_empty() {
            lines.push(current);
        }
    }
    lines
}
