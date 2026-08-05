use super::*;
use crate::*;

pub(crate) fn page_mark_section_separator_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<PageMarkSeparatorProjection> {
    if page_number != 1 {
        return None;
    }

    let page_mark_bytes = raw_stream_bytes(document, PAGE_MARK_PATH)?;
    let text_projection = layout_box_text_projection(document, layout, page_number)?;
    let caption = text_projection
        .slots
        .iter()
        .rev()
        .find(|slot| slot.role == "caption")?;
    let body = text_projection
        .slots
        .iter()
        .find(|slot| slot.role == "body")?;
    let caption_bottom = caption.y + caption.line_height;
    let body_top = body.y;
    if body_top <= caption_bottom {
        return None;
    }

    let frame_projection = page_frame_projection(document, layout, page_number)?;
    let bar = frame_projection
        .shapes
        .iter()
        .filter(|shape| shape.role == "horizontalPatternBar")
        .max_by(|left, right| {
            left.width
                .partial_cmp(&right.width)
                .unwrap_or(Ordering::Equal)
        })?;

    let candidate = page_mark_separator_candidate(page_mark_bytes)?;
    let y = page_mark_centipoints_to_css_px(
        u32::from(candidate.y_centipoints) + u32::from(candidate.advance_centipoints),
    );
    if y < caption_bottom || y > body_top {
        return None;
    }

    Some(PageMarkSeparatorProjection {
        source: PAGE_MARK_PATH,
        projection_kind: "pageMarkSectionSeparatorProjection",
        role: "sectionSeparator",
        x: bar.x,
        y,
        width: bar.width,
        stroke_width: PAGE_MARK_SEPARATOR_STROKE_WIDTH_PX,
        source_record_offset: candidate.record_offset,
        source_record_index: candidate.record_index,
        source_line_start: candidate.line_start,
        source_line_end: candidate.line_end,
        source_y_centipoints: candidate.y_centipoints,
        source_advance_centipoints: candidate.advance_centipoints,
        placement_basis: "pageMarkCentipointInsideLayoutBoxCaptionBodyGap",
        style_basis: "pageMarkSeparatorTailAndRecurringMarkAdvance",
        page_assignment_decoded: false,
    })
}

pub(crate) fn page_mark_separator_candidate(bytes: &[u8]) -> Option<PageMarkSeparatorCandidate> {
    let advance_centipoints = page_mark_recurring_advance_centipoints(bytes).unwrap_or(0);
    let headers = page_mark_record_headers(bytes);
    for (header_index, header) in headers.iter().enumerate() {
        let next_offset = headers
            .get(header_index + 1)
            .map(|next| next.offset)
            .unwrap_or(bytes.len());
        let tail_start = header.offset.checked_add(16)?;
        if tail_start > next_offset || next_offset > bytes.len() {
            continue;
        }
        let tail = &bytes[tail_start..next_offset];
        let Some(y_centipoints) = page_mark_separator_tail_y_centipoints(tail) else {
            continue;
        };
        return Some(PageMarkSeparatorCandidate {
            record_offset: header.offset,
            record_index: header.index,
            line_start: header.line_start,
            line_end: header.line_end,
            y_centipoints,
            advance_centipoints,
        });
    }
    None
}

pub(crate) fn page_mark_record_headers(bytes: &[u8]) -> Vec<PageMarkRecordHeader> {
    let mut headers = Vec::new();
    let mut offset = 12usize;
    while offset + 16 <= bytes.len() {
        let Some(index) = read_be32_at(bytes, offset) else {
            break;
        };
        let Some(flags) = read_be32_at(bytes, offset + 4) else {
            break;
        };
        let Some(line_start) = read_be32_at(bytes, offset + 8) else {
            break;
        };
        let Some(line_end) = read_be32_at(bytes, offset + 12) else {
            break;
        };
        if flags == 0x0001_0000 && index < 256 && line_start <= line_end && line_end < 10_000 {
            headers.push(PageMarkRecordHeader {
                offset,
                index,
                flags,
                line_start,
                line_end,
            });
        }
        offset += 1;
    }
    headers
}

pub(crate) fn page_mark_separator_tail_y_centipoints(tail: &[u8]) -> Option<u16> {
    if !tail
        .windows(4)
        .any(|window| window == [0xff, 0xff, 0x00, 0x00])
    {
        return None;
    }
    tail.chunks_exact(2)
        .filter_map(|chunk| {
            let value = u16::from_be_bytes([chunk[0], chunk[1]]);
            (PAGE_MARK_SEPARATOR_MIN_Y_CENTIPOINTS..=PAGE_MARK_SEPARATOR_MAX_Y_CENTIPOINTS)
                .contains(&value)
                .then_some(value)
        })
        .next_back()
}

pub(crate) fn page_mark_recurring_advance_centipoints(bytes: &[u8]) -> Option<u16> {
    let mut candidates = Vec::new();
    for offset in 0..bytes.len().saturating_sub(3) {
        if bytes[offset] == 0x00 && bytes[offset + 1] == 0xff {
            let value = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
            if (1..=2_000).contains(&value) {
                candidates.push(value);
            }
        }
    }
    candidates.sort_unstable();
    let mut best = None;
    let mut best_count = 0usize;
    let mut index = 0usize;
    while index < candidates.len() {
        let value = candidates[index];
        let count = candidates[index..]
            .iter()
            .take_while(|candidate| **candidate == value)
            .count();
        if count > best_count {
            best = Some(value);
            best_count = count;
        }
        index += count;
    }
    best
}

pub(crate) fn page_mark_centipoints_to_css_px(value: u32) -> f32 {
    value as f32 * PAGE_MARK_CENTIPOINT_TO_CSS_PX
}

pub(crate) fn push_page_mark_section_separator_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    let Some(separator) = page_mark_section_separator_projection(document, layout, page_number)
    else {
        return;
    };
    svg.push_str(&format!(
        "<line class=\"rjtd-page-mark-separator\" data-source=\"{}\" data-projection-kind=\"{}\" data-role=\"{}\" data-source-record-offset=\"{}\" data-source-record-index=\"{}\" data-source-line-start=\"{}\" data-source-line-end=\"{}\" data-source-y-centipoints=\"{}\" data-source-advance-centipoints=\"{}\" data-placement-basis=\"{}\" data-style-basis=\"{}\" x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#555555\" stroke-width=\"{:.2}\" stroke-dasharray=\"2.2 2.2\" stroke-linecap=\"butt\"/>",
        escape_xml(separator.source),
        escape_xml(separator.projection_kind),
        escape_xml(separator.role),
        separator.source_record_offset,
        separator.source_record_index,
        separator.source_line_start,
        separator.source_line_end,
        separator.source_y_centipoints,
        separator.source_advance_centipoints,
        escape_xml(separator.placement_basis),
        escape_xml(separator.style_basis),
        separator.x,
        separator.y,
        separator.x + separator.width,
        separator.y,
        separator.stroke_width
    ));
}
