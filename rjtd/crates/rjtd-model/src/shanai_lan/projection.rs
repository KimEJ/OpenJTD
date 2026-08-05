use super::*;
use crate::*;

pub(crate) fn shanai_lan_document_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ShanaiLanTextProjection> {
    if page_number != 1 || !document_has_shanai_lan_fdm_command_evidence(document) {
        return None;
    }

    let bytes = document_text_raw_stream(document)?;
    let style_resolver = DocumentTextStyleResolver::from_document_text_bytes(bytes);
    let map = map_document_text(bytes);
    let group_offsets = shanai_lan_text_group_offsets(bytes);
    let line_headers = shanai_lan_line_headers_in_groups(bytes, &group_offsets);
    let max_extent_units = shanai_lan_text_max_extent_units(bytes)
        .unwrap_or(0x0118)
        .saturating_sub(SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS)
        .max(1);
    let viewport = fdm_projection_viewport(layout);
    let grid_unit_px = viewport.width / f32::from(max_extent_units);
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let fallback_font_units = 12u16;
    let line_height_px =
        f32::from(fallback_font_units) * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR * scale_y;
    let mut slots = Vec::new();

    for entry in map
        .entries()
        .iter()
        .filter(|entry| entry.kind() == DocumentTextMapKind::TextRun)
    {
        let fragments = shanai_lan_visible_text_fragments(entry.text());
        if fragments.is_empty() {
            continue;
        }
        let source_span = TextSourceSpan::from_document_text_entry(entry);
        let line_header = shanai_lan_line_header_for_text_entry(bytes, entry);
        let group_index = shanai_lan_group_index_for_text_entry(&group_offsets, entry);
        let leading_units = leading_display_units(entry.text());
        let line_offset_units = line_header
            .as_ref()
            .map(|header| header.offset_units)
            .unwrap_or_default();
        let font_size_units = line_header
            .as_ref()
            .map(|header| header.font_size_units)
            .unwrap_or(fallback_font_units);
        let font_size = f32::from(font_size_units) * SHANAI_LAN_TEXT_FONT_SIZE_SCALE * scale_y;
        let y = group_index
            .map(|index| viewport.y + (index as f32 + 1.0) * line_height_px)
            .unwrap_or(viewport.y);
        let line_header_hex = line_header
            .as_ref()
            .and_then(|header| bytes.get(header.start..header.end))
            .map(hex_bytes)
            .unwrap_or_default();
        let line_header_raw_words = line_header
            .as_ref()
            .map(|header| header.raw_words)
            .unwrap_or([0; 12]);
        let line_header_same_segment_group_run = line_header.as_ref().and_then(|header| {
            group_index.and_then(|group_index| {
                shanai_lan_line_header_same_segment_group_run(
                    &line_headers,
                    group_index,
                    header.offset_units,
                    header.extent_units,
                )
            })
        });
        let fragment_count = fragments.len();
        let parent_text_unit_count = entry.text().encode_utf16().count();
        for (fragment_index, fragment) in fragments.iter().enumerate() {
            let fragment_grid_units = fragment.fragment_start_units.saturating_mul(2);
            let x = viewport.x
                + (f32::from(line_offset_units)
                    + leading_units.saturating_mul(2) as f32
                    + fragment_grid_units as f32)
                    * grid_unit_px;
            let fragment_source_span = source_span
                .subspan_by_units(fragment.source_start_units, fragment.source_end_units);
            let property_15_color_candidate =
                document_text_property_15_color_candidate(&style_resolver, &fragment_source_span);
            let fill = property_15_color_candidate
                .as_ref()
                .map(|candidate| candidate.css_color)
                .unwrap_or_else(fallback_text_fill_color);
            let fill_basis = property_15_color_candidate
                .as_ref()
                .map(|_| DOCUMENT_TEXT_PROPERTY_15_COLOR_BASIS)
                .unwrap_or("default-text-fill");
            let previous_gap_units = (fragment_index > 0).then(|| {
                fragment
                    .source_start_units
                    .saturating_sub(fragments[fragment_index - 1].source_end_units)
            });
            let next_gap_units = (fragment_index + 1 < fragment_count).then(|| {
                fragments[fragment_index + 1]
                    .source_start_units
                    .saturating_sub(fragment.source_end_units)
            });
            let text_count_range_evidence =
                shanai_lan_text_count_range_evidence(document, &fragment_source_span);
            let style_link_evidence = shanai_lan_text_style_link_evidence(
                document,
                bytes,
                entry,
                &text_count_range_evidence,
            );
            slots.push(ShanaiLanTextSlot {
                text: fragment.text.clone(),
                x,
                y,
                font_size,
                fill,
                fill_basis,
                document_text_property_15_color_candidate: property_15_color_candidate,
                style_link_evidence,
                source_span: fragment_source_span,
                fragment_context: ShanaiLanTextRunFragmentContext {
                    parent_source_span: source_span.clone(),
                    parent_text_unit_count,
                    fragment_index,
                    fragment_count,
                    fragment_source_start_units: fragment.source_start_units,
                    fragment_source_end_units: fragment.source_end_units,
                    previous_gap_units,
                    next_gap_units,
                    style_boundary_proven: false,
                    promotion_blocked_reason: "document-text-fragment-style-boundary-unproven",
                },
                text_count_range_evidence,
                group_index,
                line_offset_units,
                leading_units,
                fragment_start_units: fragment.fragment_start_units,
                split_from_text_run: fragment.split_from_text_run,
                line_header_hex: line_header_hex.clone(),
                line_header_raw_words,
                line_header_same_segment_group_run,
                line_header_same_segment_group_run_text_slot_count: None,
                line_header_same_segment_group_run_distinct_text_group_count: None,
            });
        }
    }

    attach_shanai_lan_line_header_same_segment_text_peer_counts(&mut slots);

    (!slots.is_empty()).then_some(ShanaiLanTextProjection {
        source: "/DocumentText",
        projection_kind: "documentTextGroupLineProjection",
        grid_unit_px,
        line_height_px,
        slots,
    })
}

pub(crate) fn attach_shanai_lan_line_header_same_segment_text_peer_counts(
    slots: &mut [ShanaiLanTextSlot],
) {
    let mut peer_counts = BTreeMap::<(u16, u16, usize, usize), (usize, BTreeSet<usize>)>::new();
    for slot in slots.iter() {
        let Some(run) = slot.line_header_same_segment_group_run else {
            continue;
        };
        let key = (
            run.offset_units,
            run.extent_units,
            run.start_group_index,
            run.end_group_index,
        );
        let entry = peer_counts.entry(key).or_default();
        entry.0 += 1;
        if let Some(group_index) = slot.group_index {
            entry.1.insert(group_index);
        }
    }

    for slot in slots.iter_mut() {
        let Some(run) = slot.line_header_same_segment_group_run else {
            continue;
        };
        let key = (
            run.offset_units,
            run.extent_units,
            run.start_group_index,
            run.end_group_index,
        );
        if let Some((text_slot_count, group_indexes)) = peer_counts.get(&key) {
            slot.line_header_same_segment_group_run_text_slot_count = Some(*text_slot_count);
            slot.line_header_same_segment_group_run_distinct_text_group_count =
                Some(group_indexes.len());
        }
    }
}

pub(crate) fn shanai_lan_document_text_line_rule_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<ShanaiLanLineRuleProjection> {
    if page_number != 1 || !document_has_shanai_lan_fdm_command_evidence(document) {
        return None;
    }

    let bytes = document_text_raw_stream(document)?;
    let line_mark_intervals = shanai_lan_line_mark_intervals(document);
    let line_mark_profile = shanai_lan_line_mark_profile(document);
    shanai_lan_document_text_line_rule_projection_from_bytes(
        bytes,
        layout,
        &line_mark_intervals,
        line_mark_profile,
    )
}

pub(crate) fn shanai_lan_document_text_line_rule_projection_from_bytes(
    bytes: &[u8],
    layout: PageLayout,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    line_mark_profile: &'static str,
) -> Option<ShanaiLanLineRuleProjection> {
    let map = map_document_text(bytes);
    let group_offsets = shanai_lan_text_group_offsets(bytes);
    let document_text_group_count = group_offsets.len();
    let document_text_line_header_count =
        shanai_lan_line_headers_in_groups(bytes, &group_offsets).len();
    let raw_max_extent_units = shanai_lan_text_max_extent_units(bytes).unwrap_or(0x0118);
    let max_extent_units = raw_max_extent_units
        .saturating_sub(SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS)
        .max(1);
    let viewport = fdm_projection_viewport(layout);
    let grid_unit_px = viewport.width / f32::from(max_extent_units);
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let line_height_px = 12.0 * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR * scale_y;
    let mut rules = Vec::new();
    let mut anchor_units = BTreeSet::new();
    let mut skipped_inline_line_header_count = 0usize;

    for entry in map
        .entries()
        .iter()
        .filter(|entry| entry.kind() == DocumentTextMapKind::SkippedInlineText)
    {
        let mut offset = entry.byte_start();
        while offset + 24 <= entry.byte_end().min(bytes.len()) {
            if let Some(header) = shanai_lan_line_header_at(bytes, offset)
                && header.end <= entry.byte_end()
                && header.extent_units > header.offset_units
                && header.extent_units.saturating_sub(header.offset_units)
                    >= SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS
                && let Some(group_index) =
                    shanai_lan_group_index_for_text_entry(&group_offsets, entry)
            {
                let x1 = viewport.x + f32::from(header.offset_units) * grid_unit_px;
                let x2 = viewport.x + f32::from(header.extent_units) * grid_unit_px;
                let y = viewport.y + (group_index as f32 + 1.0) * line_height_px;
                let line_header_hex = bytes
                    .get(header.start..header.end)
                    .map(hex_bytes)
                    .unwrap_or_default();
                let line_mark = shanai_lan_line_mark_for_header(line_mark_intervals, &header);
                anchor_units.insert(header.offset_units);
                anchor_units.insert(header.extent_units);
                skipped_inline_line_header_count += 1;
                rules.push(ShanaiLanLineRule {
                    x1,
                    y1: y,
                    x2,
                    y2: y,
                    orientation: "horizontal",
                    candidate_source: "skippedInlineLineHeader",
                    source_span: TextSourceSpan::new(
                        header.start,
                        header.end,
                        header.start / 2,
                        header.end / 2,
                    ),
                    group_index,
                    end_group_index: group_index,
                    line_offset_units: header.offset_units,
                    line_extent_units: header.extent_units,
                    line_header_hex,
                    line_header_raw_words: header.raw_words,
                    line_mark,
                });
                offset = header.end;
            } else {
                offset += 2;
            }
        }
    }
    append_shanai_lan_vertical_anchor_line_rules(
        bytes,
        &group_offsets,
        raw_max_extent_units,
        &anchor_units,
        viewport,
        grid_unit_px,
        line_height_px,
        &mut rules,
        line_mark_intervals,
    );

    (!rules.is_empty()).then_some(ShanaiLanLineRuleProjection {
        source: "/DocumentText",
        projection_kind: "documentTextLineRuleProjection",
        line_mark_profile,
        line_mark_interval_count: line_mark_intervals.len(),
        document_text_group_count,
        document_text_line_header_count,
        skipped_inline_line_header_count,
        grid_unit_px,
        line_height_px,
        stroke_width: SHANAI_LAN_LINE_RULE_STROKE_WIDTH_PX * scale_y,
        rules,
    })
}

pub(crate) fn shanai_lan_line_mark_intervals(
    document: &Document,
) -> Vec<ShanaiLanLineMarkInterval> {
    let Some(bytes) = document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/LineMark")
        .map(RawStream::bytes)
    else {
        return Vec::new();
    };
    shanai_lan_line_mark_intervals_from_bytes(bytes)
}

pub(crate) fn shanai_lan_line_mark_profile(document: &Document) -> &'static str {
    document
        .raw_streams()
        .iter()
        .find(|stream| stream.name() == "/LineMark")
        .map(RawStream::bytes)
        .map(shanai_lan_line_mark_profile_from_bytes)
        .unwrap_or(SHANAI_LAN_LINE_MARK_PROFILE_ABSENT)
}

pub(crate) fn shanai_lan_line_mark_profile_from_bytes(bytes: &[u8]) -> &'static str {
    if !shanai_lan_line_mark_intervals_from_bytes(bytes).is_empty() {
        return SHANAI_LAN_LINE_MARK_PROFILE_BE_DELTA_V1;
    }
    if utf16le_ascii_contains(bytes, "MacrosStreamStyle") {
        return SHANAI_LAN_LINE_MARK_PROFILE_MACRO_STYLE;
    }
    SHANAI_LAN_LINE_MARK_PROFILE_UNPARSED
}

pub(crate) fn shanai_lan_line_mark_intervals_from_bytes(
    bytes: &[u8],
) -> Vec<ShanaiLanLineMarkInterval> {
    let Some(count) = read_be16_at(bytes, LINE_MARK_BE_DELTA_COUNT_OFFSET).map(usize::from) else {
        return Vec::new();
    };
    if count == 0
        || bytes.len()
            < LINE_MARK_BE_DELTA_HEADER_BYTES
                + count.saturating_mul(LINE_MARK_BE_DELTA_RECORD_BYTES)
    {
        return Vec::new();
    }

    let mut intervals = Vec::new();
    let mut unit_start = LINE_MARK_BE_DELTA_BASE_UNIT;
    for record_index in 0..count {
        let offset = line_mark_be_delta_record_byte_offset(record_index);
        let Some(delta_word) = read_be16_at(bytes, offset) else {
            break;
        };
        let Some(flag_word) = read_be16_at(bytes, offset + 2) else {
            break;
        };
        let delta = delta_word as i16;
        if delta <= 0 {
            break;
        }
        let unit_end = unit_start.saturating_add(delta as usize);
        intervals.push(ShanaiLanLineMarkInterval {
            record_index,
            unit_start,
            unit_end,
            flag_word,
        });
        unit_start = unit_end;
    }
    intervals
}

pub(crate) fn shanai_lan_line_mark_for_header(
    intervals: &[ShanaiLanLineMarkInterval],
    header: &ShanaiLanLineHeader,
) -> Option<ShanaiLanLineMarkInterval> {
    let unit = header.start / 2;
    intervals
        .iter()
        .copied()
        .find(|interval| interval.unit_start <= unit && unit < interval.unit_end)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn append_shanai_lan_vertical_anchor_line_rules(
    bytes: &[u8],
    group_offsets: &[usize],
    raw_max_extent_units: u16,
    anchor_units: &BTreeSet<u16>,
    viewport: FdmProjectionViewport,
    grid_unit_px: f32,
    line_height_px: f32,
    rules: &mut Vec<ShanaiLanLineRule>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) {
    let hidden_groups = rules
        .iter()
        .filter(|rule| rule.orientation == "horizontal")
        .map(|rule| rule.group_index)
        .collect::<BTreeSet<_>>();
    let Some(first_hidden_group) = hidden_groups.iter().next().copied() else {
        return;
    };
    let Some(last_hidden_group) = hidden_groups.iter().next_back().copied() else {
        return;
    };

    let min_group = first_hidden_group.saturating_sub(4);
    let max_group = last_hidden_group.saturating_add(1);
    let line_headers = shanai_lan_line_headers_in_groups(bytes, group_offsets);

    for anchor_unit in anchor_units {
        if *anchor_unit == 0 || *anchor_unit == raw_max_extent_units {
            continue;
        }

        let mut support_by_group = BTreeMap::new();
        for line_header in line_headers.iter().filter(|line_header| {
            (min_group..=max_group).contains(&line_header.group_index)
                && (line_header.header.offset_units == *anchor_unit
                    || line_header.header.extent_units == *anchor_unit)
        }) {
            support_by_group
                .entry(line_header.group_index)
                .or_insert(*line_header);
        }

        let mut run_start: Option<ShanaiLanLineHeaderInGroup> = None;
        let mut previous: Option<ShanaiLanLineHeaderInGroup> = None;
        for line_header in support_by_group.values().copied() {
            match previous {
                Some(previous_header)
                    if line_header.group_index == previous_header.group_index + 1 =>
                {
                    previous = Some(line_header);
                }
                Some(previous_header) => {
                    push_shanai_lan_vertical_anchor_line_rule(
                        bytes,
                        viewport,
                        grid_unit_px,
                        line_height_px,
                        *anchor_unit,
                        run_start.unwrap_or(previous_header),
                        previous_header,
                        rules,
                        line_mark_intervals,
                    );
                    run_start = Some(line_header);
                    previous = Some(line_header);
                }
                None => {
                    run_start = Some(line_header);
                    previous = Some(line_header);
                }
            }
        }
        if let (Some(run_start), Some(previous)) = (run_start, previous) {
            push_shanai_lan_vertical_anchor_line_rule(
                bytes,
                viewport,
                grid_unit_px,
                line_height_px,
                *anchor_unit,
                run_start,
                previous,
                rules,
                line_mark_intervals,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_shanai_lan_vertical_anchor_line_rule(
    bytes: &[u8],
    viewport: FdmProjectionViewport,
    grid_unit_px: f32,
    line_height_px: f32,
    anchor_unit: u16,
    run_start: ShanaiLanLineHeaderInGroup,
    run_end: ShanaiLanLineHeaderInGroup,
    rules: &mut Vec<ShanaiLanLineRule>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) {
    if run_end.group_index <= run_start.group_index {
        return;
    }
    let x = viewport.x + f32::from(anchor_unit) * grid_unit_px;
    let y1 = viewport.y + (run_start.group_index as f32 + 1.0) * line_height_px;
    let y2 = viewport.y + (run_end.group_index as f32 + 1.0) * line_height_px;
    let source_start = run_start.header.start.min(run_end.header.start);
    let source_end = run_start.header.end.max(run_end.header.end);
    let line_header_hex = bytes
        .get(run_start.header.start..run_start.header.end)
        .map(hex_bytes)
        .unwrap_or_default();
    let line_mark = shanai_lan_line_mark_for_header(line_mark_intervals, &run_start.header);
    rules.push(ShanaiLanLineRule {
        x1: x,
        y1,
        x2: x,
        y2,
        orientation: "vertical",
        candidate_source: "verticalAnchorRunFromLineHeaders",
        source_span: TextSourceSpan::new(
            source_start,
            source_end,
            source_start / 2,
            source_end / 2,
        ),
        group_index: run_start.group_index,
        end_group_index: run_end.group_index,
        line_offset_units: anchor_unit,
        line_extent_units: anchor_unit,
        line_header_hex,
        line_header_raw_words: run_start.header.raw_words,
        line_mark,
    });
}

pub(crate) fn shanai_lan_line_headers_in_groups(
    bytes: &[u8],
    group_offsets: &[usize],
) -> Vec<ShanaiLanLineHeaderInGroup> {
    let mut headers = Vec::new();
    let mut offset = 0usize;
    while offset + 24 <= bytes.len() {
        if let Some(header) = shanai_lan_line_header_at(bytes, offset) {
            if let Some(group_index) =
                shanai_lan_group_index_for_byte_offset(group_offsets, header.start)
            {
                headers.push(ShanaiLanLineHeaderInGroup {
                    group_index,
                    header,
                });
            }
            offset = header.end;
        } else {
            offset += 2;
        }
    }
    headers
}

pub(crate) fn shanai_lan_text_group_offsets(bytes: &[u8]) -> Vec<usize> {
    let mut offsets = Vec::new();
    let mut offset = 0usize;
    while offset + 4 <= bytes.len() {
        if bytes[offset..].starts_with(&[0x00, 0x1c, 0x00, 0x10]) {
            offsets.push(offset);
        }
        offset += 2;
    }
    offsets
}

pub(crate) fn shanai_lan_text_max_extent_units(bytes: &[u8]) -> Option<u16> {
    let mut max_extent: Option<u16> = None;
    let mut offset = 0usize;
    while offset + 24 <= bytes.len() {
        if let Some(header) = shanai_lan_line_header_at(bytes, offset) {
            max_extent = Some(max_extent.unwrap_or(0).max(header.extent_units));
            offset = header.end;
        } else {
            offset += 2;
        }
    }
    max_extent
}

pub(crate) fn shanai_lan_group_index_for_text_entry(
    group_offsets: &[usize],
    entry: &DocumentTextMapEntry,
) -> Option<usize> {
    shanai_lan_group_index_for_byte_offset(group_offsets, entry.byte_start())
}

pub(crate) fn shanai_lan_group_index_for_byte_offset(
    group_offsets: &[usize],
    byte_offset: usize,
) -> Option<usize> {
    group_offsets
        .iter()
        .rposition(|offset| *offset < byte_offset)
}

pub(crate) fn shanai_lan_line_header_for_text_entry(
    bytes: &[u8],
    entry: &DocumentTextMapEntry,
) -> Option<ShanaiLanLineHeader> {
    let search_start = entry.byte_start().saturating_sub(64);
    let mut offset = entry.byte_start().saturating_sub(2);
    while offset >= search_start && offset + 24 <= bytes.len() {
        if let Some(header) = shanai_lan_line_header_at(bytes, offset)
            && header.end <= entry.byte_start()
        {
            return Some(header);
        }
        if offset < 2 {
            break;
        }
        offset -= 2;
    }
    None
}

pub(crate) fn shanai_lan_line_header_at(
    bytes: &[u8],
    offset: usize,
) -> Option<ShanaiLanLineHeader> {
    if offset + 24 > bytes.len() || !bytes[offset..].starts_with(&[0x00, 0x1c, 0x00, 0x30]) {
        return None;
    }
    let mut words = [0u16; 12];
    for (index, chunk) in bytes[offset..offset + 24].chunks_exact(2).enumerate() {
        words[index] = u16::from_be_bytes([chunk[0], chunk[1]]);
    }
    if words[2] == 0
        || words[6] != 0x00ff
        || words[7] != 0
        || words[9] != 0
        || words[10] != 0x0030
        || words[11] != 0x001f
    {
        return None;
    }
    Some(ShanaiLanLineHeader {
        offset_units: words[4],
        extent_units: words[5],
        font_size_units: words[2],
        raw_words: words,
        start: offset,
        end: offset + 24,
    })
}

pub(crate) fn shanai_lan_visible_text_fragments(text: &str) -> Vec<ShanaiLanTextFragment> {
    let characters = text.chars().collect::<Vec<_>>();
    let mut visible_start = 0usize;
    while visible_start < characters.len()
        && matches!(
            characters[visible_start],
            ' ' | '\u{3000}' | '\n' | '\r' | '\t'
        )
    {
        visible_start += 1;
    }

    let mut visible_end = characters.len();
    while visible_end > visible_start
        && matches!(
            characters[visible_end - 1],
            ' ' | '\u{3000}' | '\n' | '\r' | '\t'
        )
    {
        visible_end -= 1;
    }

    if visible_start >= visible_end {
        return Vec::new();
    }

    let visible_text = characters[visible_start..visible_end]
        .iter()
        .collect::<String>();
    if visible_text.trim_matches(|character| matches!(character, ' ' | '\u{3000}')) == "#" {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut fragment_start = visible_start;
    let mut offset = visible_start;
    while offset < visible_end {
        if matches!(characters[offset], ' ' | '\u{3000}') {
            let gap_start = offset;
            let mut gap_end = offset;
            while gap_end < visible_end && matches!(characters[gap_end], ' ' | '\u{3000}') {
                gap_end += 1;
            }
            let gap_units = shanai_lan_spacing_units_for_chars(&characters[gap_start..gap_end]);
            if gap_units >= SHANAI_LAN_TEXT_FRAGMENT_GAP_UNITS
                && fragment_start < gap_start
                && gap_end < visible_end
            {
                ranges.push((fragment_start, gap_start));
                fragment_start = gap_end;
            }
            offset = gap_end;
        } else {
            offset += 1;
        }
    }
    if fragment_start < visible_end {
        ranges.push((fragment_start, visible_end));
    }

    let split_from_text_run = ranges.len() > 1;
    ranges
        .into_iter()
        .filter_map(|(start, end)| {
            let fragment_text = characters[start..end].iter().collect::<String>();
            let trimmed =
                fragment_text.trim_matches(|character| matches!(character, ' ' | '\u{3000}'));
            if trimmed.is_empty() || trimmed == "#" {
                return None;
            }
            Some(ShanaiLanTextFragment {
                text: trimmed.to_string(),
                source_start_units: utf16_units_for_chars(&characters[..start]),
                source_end_units: utf16_units_for_chars(&characters[..end]),
                fragment_start_units: shanai_lan_spacing_units_for_chars(
                    &characters[visible_start..start],
                ),
                split_from_text_run,
            })
        })
        .collect()
}

pub(crate) fn shanai_lan_spacing_units_for_chars(characters: &[char]) -> usize {
    characters
        .iter()
        .map(|character| match character {
            '\u{3000}' => 2,
            '\t' => APP_TAB_COLUMNS,
            _ => character.len_utf16(),
        })
        .sum()
}
