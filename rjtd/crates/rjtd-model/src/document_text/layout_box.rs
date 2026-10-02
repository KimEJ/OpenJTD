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
    if let Some(projection) =
        frame_linked_layout_box_text_projection(document, layout, bytes, &blocks, &records)
    {
        return Some(projection);
    }
    let body_anchor = layout_box_body_anchor(&blocks, &records, layout);
    let containers = linked_text_frame_container_ids(document);
    let title_frame_shape = document
        .object_frame_records()
        .iter()
        .filter(|record| !containers.contains(&record.object_id()))
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
                            frame_source: None,
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
                        frame_source: None,
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
                        frame_source: None,
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
    rjtd_core::document_text::map_document_text_content(payload)
        .entries()
        .iter()
        .filter(|entry| {
            matches!(
                entry.kind(),
                DocumentTextMapKind::TextRun | DocumentTextMapKind::InlineText
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
        .collect()
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

// Type-2 Frame rows correspond to dense LayoutBox/TextV records in stream
// order. The field historically called object_type is NOT a text-block index:
// tmogi2_2 permutes it independently of title, caption and body block order.
fn layout_box_frame_records(
    document: &Document,
    block_count: usize,
    record_count: usize,
) -> Option<Vec<&ObjectFrameRecordCandidate>> {
    let frames = document
        .object_frame_records()
        .iter()
        .filter(|frame| {
            frame.record_kind() == 0x0102
                && frame.declared_record_bytes() == 56
                && frame.record_len() == 60
                && read_be16_at(frame.raw_bytes(), 8) == Some(2)
                && read_be16_at(frame.raw_bytes(), 10) == Some(0)
        })
        .collect::<Vec<_>>();
    if frames.is_empty() || frames.len() != block_count || block_count != record_count {
        return None;
    }
    let ids = frames
        .iter()
        .map(|frame| frame.object_id())
        .collect::<BTreeSet<_>>();
    if ids.len() != frames.len() {
        return None;
    }
    Some(frames)
}

fn frame_text_references(bytes: &[u8]) -> impl Iterator<Item = (usize, u16)> + '_ {
    bytes
        .windows(28)
        .step_by(2)
        .enumerate()
        .filter(|(_, row)| {
            row[..10] == [0, 0x1c, 0, 0, 0, 14, 0, 0, 0, 0x30]
                && row[12..16] == [5, 7, 0, 0x12]
                && row[18..] == [0, 0, 0, 14, 0, 0, 0, 0, 0, 0x1f]
        })
        .map(|(unit, row)| (unit, u16::from_be_bytes([row[16], row[17]])))
}

pub(crate) fn linked_text_frame_container_ids(document: &Document) -> BTreeSet<u16> {
    let Some(bytes) = raw_stream_bytes(document, LAYOUT_BOX_TEXT_PATH) else {
        return BTreeSet::new();
    };
    let blocks = layout_box_text_blocks(bytes);
    let record_count = raw_stream_bytes(document, LAYOUT_BOX_PATH)
        .map(layout_box_record_candidates)
        .map_or(0, |records| records.len());
    let Some(frames) = layout_box_frame_records(document, blocks.len(), record_count) else {
        return BTreeSet::new();
    };
    let ids = frames
        .iter()
        .map(|frame| frame.object_id())
        .collect::<BTreeSet<_>>();
    frames
        .iter()
        .zip(&blocks)
        .filter_map(|(frame, block)| {
            if !block.fragments.is_empty() {
                return None;
            }
            let mut refs =
                frame_text_references(&bytes[block.payload_start..block.payload_end]).peekable();
            (refs.peek().is_some()
                && refs.all(|(_, id)| id != frame.object_id() && ids.contains(&id)))
            .then_some(frame.object_id())
        })
        .collect()
}

fn frame_linked_layout_box_text_projection(
    document: &Document,
    layout: PageLayout,
    bytes: &[u8],
    blocks: &[LayoutBoxTextBlock],
    records: &[LayoutBoxRecordCandidate],
) -> Option<LayoutBoxTextProjection> {
    if !bytes.starts_with(b"SsmgV.01") {
        return None;
    }
    let frames = layout_box_frame_records(document, blocks.len(), records.len())?;
    // Admit a complete one-level text-frame tree. More complex ownership and
    // page assignments remain diagnostic until their anchor rules are decoded.
    if frames.len() != document.object_frame_records().len() {
        return None;
    }
    let body = document_text_raw_stream(document)?;
    let mut roots = frame_text_references(body);
    let (anchor_unit, root_id) = roots.next()?;
    if roots.next().is_some() {
        return None;
    }
    let root_index = frames
        .iter()
        .position(|frame| frame.object_id() == root_id)?;
    let root = frames[root_index];
    let block = &blocks[root_index];
    if !block.fragments.is_empty() || root.width() == 0 || root.height() == 0 {
        return None;
    }
    let mut children = BTreeSet::new();
    for (_, id) in frame_text_references(&bytes[block.payload_start..block.payload_end]) {
        if id == root_id || !children.insert(id) {
            return None;
        }
    }
    if children.len() + 1 != frames.len() {
        return None;
    }
    if frames
        .iter()
        .any(|frame| read_be32_at(frame.raw_bytes(), 16) != Some(0x0001_0000))
    {
        return None;
    }
    let anchor_span = TextSourceSpan::new(
        anchor_unit * 2,
        (anchor_unit + 14) * 2,
        anchor_unit,
        anchor_unit + 14,
    );
    let (paragraph, char_offset) =
        project_control_boundary_to_text(&anchor_span, &paragraph_source_text_spans(document))?;
    if paragraph != 0 || char_offset != 0 {
        return None;
    }
    let default_font = document_default_font_size_px(document);
    let mut slots = Vec::new();
    let mut row_bands = BTreeMap::<(u32, u32), Vec<(u32, u32)>>::new();
    for (index, frame) in frames.iter().enumerate() {
        if frame.object_id() == root_id {
            continue;
        }
        if !children.contains(&frame.object_id()) {
            return None;
        }
        let block = &blocks[index];
        if block.fragments.len() != 1
            || frame_text_references(&bytes[block.payload_start..block.payload_end])
                .next()
                .is_some()
        {
            return None;
        }
        let fragment = &block.fragments[0];
        if fragment.text.contains(['\n', '\r']) || frame.width() == 0 || frame.height() == 0 {
            return None;
        }
        let (x0, y0) = (u32::from(frame.x()), u32::from(frame.y()));
        let (x1, y1) = (
            x0 + u32::from(frame.width()),
            y0 + u32::from(frame.height()),
        );
        if x1 > u32::from(root.width()) || y1 > u32::from(root.height()) {
            return None;
        }
        row_bands.entry((y0, y1)).or_default().push((x0, x1));
        let stream_start = block.byte_start.checked_sub(20)?;
        let stream_end = blocks
            .get(index + 1)
            .map(|next| next.byte_start)
            .unwrap_or(bytes.len());
        let stream = bytes.get(stream_start..stream_end)?;
        if !stream_start.is_multiple_of(2) || stream.get(20..28) != Some(b"TextV.01".as_slice()) {
            return None;
        }
        let span = TextSourceSpan::new(
            fragment
                .source_span
                .byte_start()
                .checked_sub(stream_start)?,
            fragment.source_span.byte_end().checked_sub(stream_start)?,
            fragment
                .source_span
                .unit_start()
                .checked_sub(stream_start / 2)?,
            fragment
                .source_span
                .unit_end()
                .checked_sub(stream_start / 2)?,
        );
        let resolver = DocumentTextStyleResolver::from_document_text_bytes(stream);
        let font = document_text_font_size(&resolver, &span, default_font)?;
        let x_mm100 = u32::from(root.x()) + x0;
        let y_mm100 = u32::from(root.y()) + y0;
        let x = layout.margin_left_px() + hundredth_millimeters_to_css_px(x_mm100);
        let y = layout.margin_top_px() + hundredth_millimeters_to_css_px(y_mm100);
        let width = frame_record_unit_to_css_px(frame.width());
        let height = frame_record_unit_to_css_px(frame.height());
        if x + width > layout.width_px() || y + height > layout.height_px() {
            return None;
        }
        slots.push(LayoutBoxTextSlot {
            role: "frame-linked-cell-text",
            text: fragment.text.clone(),
            x,
            y,
            font_size: font.px,
            line_height: height,
            source_span: fragment.source_span.clone(),
            block_index: index,
            layout_record_index: Some(index),
            layout_record_byte_range: Some((records[index].byte_start, records[index].byte_end)),
            layout_x_pt: None,
            layout_y_pt: None,
            layout_width_pt: None,
            inferred_origin_pt: None,
            placement_basis: "frame-mm100-relative-to-text-anchor",
            frame_source: Some(LayoutBoxFrameSource {
                frame_id: frame.object_id(),
                parent_frame_id: root_id,
                raw_word6: frame.object_type(),
                record_start: frame.row_start(),
                x_mm100,
                y_mm100,
            }),
        });
    }
    let mut next_y = 0;
    for ((y0, y1), mut cells) in row_bands {
        if y0 != next_y {
            return None;
        }
        cells.sort_unstable();
        let mut next_x = 0;
        for (x0, x1) in cells {
            if x0 != next_x {
                return None;
            }
            next_x = x1;
        }
        if next_x != u32::from(root.width()) {
            return None;
        }
        next_y = y1;
    }
    if next_y != u32::from(root.height()) {
        return None;
    }
    Some(LayoutBoxTextProjection {
        source: LAYOUT_BOX_TEXT_PATH,
        projection_kind: "frameLinkedTextProjection",
        block_count: blocks.len(),
        layout_record_count: records.len(),
        position_table_present: raw_stream_bytes(document, LAYOUT_BOX_TEXT_POSITION_TABLES_PATH)
            .is_some(),
        page_assignment_decoded: false,
        slots,
    })
}

#[cfg(test)]
mod frame_link_tests {
    use super::*;

    fn reference(id: u16) -> Vec<u16> {
        vec![
            0x001c, 0, 14, 0, 0x30, 0, 0x0507, 0x0012, id, 0, 14, 0, 0, 0x001f,
        ]
    }

    fn text_block(content: &[u16]) -> Vec<u8> {
        let mut bytes = vec![0; 20];
        bytes[..8].copy_from_slice(b"SsmgV.01");
        bytes.extend_from_slice(b"TextV.01");
        bytes.extend_from_slice(&(content.len() as u32).to_be_bytes());
        for word in content {
            bytes.extend_from_slice(&word.to_be_bytes());
        }
        bytes.extend_from_slice(&[0xfe, 2, 2, 1, 0x72, 0xff, 0]);
        if content.len() > 1 {
            bytes.push(0);
            bytes.extend_from_slice(&((content.len() - 1) as u32).to_be_bytes());
        }
        if !bytes.len().is_multiple_of(2) {
            bytes.push(0);
        }
        bytes
    }

    fn frame(id: u16, order: u16, geometry: [u16; 4]) -> ObjectFrameRecordCandidate {
        let mut bytes = vec![0; 60];
        for (offset, value) in [
            (0, 0x0102),
            (2, 56),
            (6, id),
            (8, 2),
            (12, order),
            (16, 1),
            (28, geometry[0]),
            (32, geometry[1]),
            (36, geometry[2]),
            (40, geometry[3]),
        ] {
            bytes[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
        }
        ObjectFrameRecordCandidate::new(
            "/Frame",
            usize::from(id),
            16 + usize::from(id) * 60,
            &bytes,
        )
    }

    fn linked_document() -> Document {
        let mut body = reference(9);
        body.push(u16::from(b'A'));
        let text = TextRun::with_source_span("A", None, Some(TextSourceSpan::new(60, 62, 30, 31)));
        let mut doc = Document::new(
            Metadata::default(),
            vec![Block::Paragraph(Paragraph::new(
                vec![Inline::Text(text)],
                None,
            ))],
        );
        doc.push_raw_stream(RawStream::new("/DocumentText", text_block(&body)));
        let mut refs = reference(5);
        refs.extend(reference(3));
        let mut boxes = text_block(&refs);
        boxes.extend(text_block(&"RIGHT".encode_utf16().collect::<Vec<_>>()));
        boxes.extend(text_block(&"LEFT".encode_utf16().collect::<Vec<_>>()));
        doc.push_raw_stream(RawStream::new(LAYOUT_BOX_TEXT_PATH, boxes));
        let mut records = Vec::new();
        for _ in 0..3 {
            records.extend_from_slice(&[2, 1, 0, 8]);
            records.extend_from_slice(&[0; 96]);
        }
        doc.push_raw_stream(RawStream::new(LAYOUT_BOX_PATH, records));
        doc.push_object_frame_record(frame(9, 1, [200, 500, 2000, 500]));
        doc.push_object_frame_record(frame(5, 3, [1000, 0, 1000, 500]));
        doc.push_object_frame_record(frame(3, 2, [0, 0, 1000, 500]));
        doc
    }

    #[test]
    fn links_nonsequential_frame_ids_in_stream_order_and_preserves_relative_cell_geometry() {
        let document = linked_document();
        let projection = layout_box_text_projection(&document, PageLayout::default(), 1).unwrap();
        assert_eq!(projection.projection_kind, "frameLinkedTextProjection");
        assert_eq!(
            projection
                .slots
                .iter()
                .map(|s| s.text.as_str())
                .collect::<Vec<_>>(),
            ["RIGHT", "LEFT"]
        );
        let left = &projection.slots[1];
        let right = &projection.slots[0];
        assert_eq!(left.frame_source.as_ref().unwrap().frame_id, 3);
        assert_eq!(right.frame_source.as_ref().unwrap().frame_id, 5);
        assert!((right.x - left.x - hundredth_millimeters_to_css_px(1000)).abs() < 0.001);
        assert_eq!(left.y, right.y);
        assert!(linked_text_frame_container_ids(&document).contains(&9));
        assert!(page_frame_projection(&document, PageLayout::default(), 1).is_none());
    }

    #[test]
    fn rejects_ambiguous_links_and_incomplete_or_overlapping_frame_grids() {
        for change in 0..3 {
            let mut document = linked_document();
            match change {
                0 => document.object_frame_records[1].object_id = 9,
                1 => document.object_frame_records[1].x = 900,
                _ => document.object_frame_records[1].x = 1100,
            }
            let bytes = raw_stream_bytes(&document, LAYOUT_BOX_TEXT_PATH).unwrap();
            let blocks = layout_box_text_blocks(bytes);
            let records =
                layout_box_record_candidates(raw_stream_bytes(&document, LAYOUT_BOX_PATH).unwrap());
            assert!(
                frame_linked_layout_box_text_projection(
                    &document,
                    PageLayout::default(),
                    bytes,
                    &blocks,
                    &records
                )
                .is_none()
            );
        }
    }

    #[test]
    #[ignore = "requires local tmogi2_2 sample"]
    fn local_frame_raw_word6_does_not_reorder_caption_and_body_text_blocks() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../../rjtd-testdata/local-samples/ichitaro-20030228195248-0007-sp-dat-tmogi2_2.jtd",
        );
        let document = parse_document(&std::fs::read(path).unwrap()).unwrap();
        let blocks =
            layout_box_text_blocks(raw_stream_bytes(&document, LAYOUT_BOX_TEXT_PATH).unwrap());
        let records =
            layout_box_record_candidates(raw_stream_bytes(&document, LAYOUT_BOX_PATH).unwrap());
        let frames = layout_box_frame_records(&document, blocks.len(), records.len()).unwrap();
        assert!(
            blocks[6]
                .fragments
                .iter()
                .any(|f| f.text.contains("毎日新聞"))
        );
        assert_eq!(frames[6].object_id(), 9);
        assert_eq!(frames[6].width(), 11800);
        assert_eq!(frames[7].object_id(), 10);
        assert_eq!(frames[7].width(), 13812);
        assert_eq!(frames[3].object_id(), 5);
        assert_eq!(frames[3].object_type(), 7);
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn local_frame_grid_preserves_six_cell_labels() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/baseline-sweep/054_font_size_paragraph_plus.jtd");
        let core = DocumentCore::from_bytes(&std::fs::read(path).unwrap()).unwrap();
        let document = core.document();
        let bytes = raw_stream_bytes(document, LAYOUT_BOX_TEXT_PATH).unwrap();
        let blocks = layout_box_text_blocks(bytes);
        let records =
            layout_box_record_candidates(raw_stream_bytes(document, LAYOUT_BOX_PATH).unwrap());
        let frames = layout_box_frame_records(document, blocks.len(), records.len()).unwrap();
        assert_eq!(frames.len(), 7);
        let projection = frame_linked_layout_box_text_projection(
            document,
            core.page_layout(),
            bytes,
            &blocks,
            &records,
        )
        .unwrap();
        assert_eq!(projection.slots.len(), 6);
        assert_eq!(projection.slots[0].text, "R01C01");
        assert!((projection.slots[0].x * 0.75 - 90.0).abs() < 0.01);
        assert!(((projection.slots[1].x - projection.slots[0].x) * 0.75 - 90.0).abs() < 0.01);
        assert!(((projection.slots[2].y - projection.slots[0].y) * 0.75 - 16.08).abs() < 0.12);
    }
}
