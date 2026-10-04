use super::*;
use crate::*;

/// Source-backed text placement for a narrow, verified family of horizontal
/// `DocumentText` control tables. This deliberately covers only a complete
/// first-page grid; more complex tables remain diagnostic candidates.
#[derive(Debug, Clone)]
pub(crate) struct NativeControlTableTextProjection {
    pub(crate) candidate_index: usize,
    pub(crate) slots: Vec<NativeControlTableTextSlot>,
    pub(crate) border: Option<NativeControlTableBorderProjection>,
}

#[derive(Debug, Clone)]
pub(crate) struct NativeControlTableBorderProjection {
    pub(crate) vertical_x: Vec<f32>,
    pub(crate) horizontal_y: Vec<f32>,
}

#[derive(Debug, Clone)]
pub(crate) struct NativeControlTableTextSlot {
    pub(crate) candidate_index: usize,
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) baseline_y: f32,
    pub(crate) font_size: DocumentTextFontSize,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) row_index: usize,
    pub(crate) column_index: usize,
    pub(crate) header_offset_units: u16,
    pub(crate) line_mark_record_index: usize,
    pub(crate) page_mark_pitch_mm100: u16,
}

/// Source-backed placement for the short plain-text run between two complete
/// native control-table projections. The constraints are intentionally narrow:
/// a pair of verified tables, one interstitial ASCII text run, and exact
/// `/LineMark` coverage on the first page. It keeps the source spaces because
/// they are part of the observed horizontal placement in this profile.
#[derive(Debug, Clone)]
pub(crate) struct NativeControlFlowTextProjection {
    pub(crate) slots: Vec<NativeControlFlowTextSlot>,
}

#[derive(Debug, Clone)]
pub(crate) struct NativeControlFlowTextSlot {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) baseline_y: f32,
    pub(crate) font_size: DocumentTextFontSize,
    pub(crate) source_span: TextSourceSpan,
    pub(crate) line_mark_record_index: usize,
    pub(crate) page_mark_pitch_mm100: u16,
    pub(crate) leading_ascii_space_count: usize,
    pub(crate) preceding_header_offset_units: u16,
    pub(crate) preceding_header_extent_units: u16,
}

#[derive(Debug, Clone, Copy)]
struct NativeControlTableRowHeader {
    start_unit: usize,
    grid_extent: u16,
    left_edge_units: u16,
}

pub(crate) fn native_control_table_text_projections(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    writing_mode: WritingMode,
) -> Vec<NativeControlTableTextProjection> {
    if page_number != 1 || writing_mode.is_vertical() || !layout.has_source_margins() {
        return Vec::new();
    }
    document
        .table_candidates()
        .iter()
        .filter_map(|candidate| native_control_table_text_projection(document, layout, candidate))
        .collect()
}

pub(crate) fn native_control_table_text_projection_contains(
    projections: &[NativeControlTableTextProjection],
    span: &TextSourceSpan,
) -> bool {
    projections
        .iter()
        .flat_map(|projection| &projection.slots)
        .any(|slot| {
            slot.source_span.unit_start() < span.unit_end()
                && span.unit_start() < slot.source_span.unit_end()
        })
}

pub(crate) fn native_control_flow_text_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
    writing_mode: WritingMode,
    table_projections: &[NativeControlTableTextProjection],
) -> Option<NativeControlFlowTextProjection> {
    if page_number != 1
        || writing_mode.is_vertical()
        || !layout.has_source_margins()
        || table_projections.len() != 2
    {
        return None;
    }

    let mut table_ranges = table_projections
        .iter()
        .map(|projection| {
            let start = projection
                .slots
                .iter()
                .map(|slot| slot.source_span.unit_start())
                .min()?;
            let end = projection
                .slots
                .iter()
                .map(|slot| slot.source_span.unit_end())
                .max()?;
            Some((start, end))
        })
        .collect::<Option<Vec<_>>>()?;
    table_ranges.sort_unstable();
    let [(first_start, first_end), (second_start, second_end)] = table_ranges.as_slice() else {
        return None;
    };
    if first_start >= first_end || first_end >= second_start || second_start >= second_end {
        return None;
    }

    let bytes = document_text_raw_stream(document)?;
    let text_map = map_document_text(bytes);
    let entries = text_map
        .entries()
        .iter()
        .filter(|entry| {
            entry.kind() == DocumentTextMapKind::TextRun
                && *first_end <= entry.unit_start()
                && entry.unit_end() <= *second_start
        })
        .collect::<Vec<_>>();
    let [entry] = entries.as_slice() else {
        return None;
    };
    let header = native_flow_preceding_header(bytes, *first_end * 2, entry.byte_start())?;

    let mut text_lines = native_flow_source_lines(entry)?;
    if text_lines.len() != 2 {
        return None;
    }
    let leading_space_count = text_lines.first()?.leading_ascii_space_count;
    if leading_space_count == 0
        || text_lines
            .iter()
            .any(|line| line.leading_ascii_space_count != leading_space_count)
    {
        return None;
    }

    let intervals = shanai_lan_line_mark_intervals(document);
    let page_mark = document.page_marks().first()?;
    let mut page_entry = None;
    for line in &mut text_lines {
        let interval = intervals
            .iter()
            .filter(|interval| {
                interval.unit_start <= line.source_span.unit_start()
                    && line.source_span.unit_end() <= interval.unit_end
            })
            .collect::<Vec<_>>();
        let [interval] = interval.as_slice() else {
            return None;
        };
        let entry = table_grid_page_mark_entry_for_line_mark_record(
            Some(page_mark),
            interval.record_index,
        )?;
        if entry.line_start()? != 0 || entry.index()? != 0 {
            return None;
        }
        if let Some(previous) = page_entry
            && previous != entry.row_index()
        {
            return None;
        }
        page_entry = Some(entry.row_index());
        line.line_mark_record_index = interval.record_index;
    }
    let pitch_mm100 = page_mark
        .entries()
        .get(page_entry?)?
        .u16_fields()
        .get(21)
        .copied()?;
    let pitch = hundredth_millimeters_to_css_px(u32::from(pitch_mm100));
    if !(APP_FONT_SIZE_PX..=APP_LINE_HEIGHT_PX * 1.25).contains(&pitch) {
        return None;
    }

    let default_font = document_default_font_size_px(document)?;
    let resolver = document_text_style_resolver(document)?;
    let slots = text_lines
        .into_iter()
        .map(|line| {
            let font_size =
                document_text_font_size(&resolver, &line.source_span, Some(default_font))?;
            let baseline_y =
                layout.margin_top_px() + line.line_mark_record_index as f32 * pitch + font_size.px;
            (baseline_y <= layout.height_px()).then_some(NativeControlFlowTextSlot {
                text: line.text,
                x: layout.margin_left_px(),
                baseline_y,
                font_size,
                source_span: line.source_span,
                line_mark_record_index: line.line_mark_record_index,
                page_mark_pitch_mm100: pitch_mm100,
                leading_ascii_space_count: line.leading_ascii_space_count,
                preceding_header_offset_units: header.offset_units,
                preceding_header_extent_units: header.extent_units,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(NativeControlFlowTextProjection { slots })
}

pub(crate) fn native_control_flow_text_projection_contains(
    projection: Option<&NativeControlFlowTextProjection>,
    span: &TextSourceSpan,
) -> bool {
    projection.is_some_and(|projection| {
        projection.slots.iter().any(|slot| {
            slot.source_span.unit_start() < span.unit_end()
                && span.unit_start() < slot.source_span.unit_end()
        })
    })
}

pub(crate) fn push_native_control_table_text_svg(
    svg: &mut String,
    projections: &[NativeControlTableTextProjection],
    font_family: &str,
) {
    for projection in projections {
        if let Some(border) = &projection.border {
            svg.push_str("<g class=\"rjtd-native-control-table-border\" data-projection-kind=\"nativeControlTableBorderProjection\" data-source-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"false\">");
            for x in &border.vertical_x {
                let (Some(y0), Some(y1)) =
                    (border.horizontal_y.first(), border.horizontal_y.last())
                else {
                    continue;
                };
                svg.push_str(&format!("<line x1=\"{x:.1}\" y1=\"{y0:.1}\" x2=\"{x:.1}\" y2=\"{y1:.1}\" stroke=\"#000000\" stroke-width=\"0.8\"/>"));
            }
            for y in &border.horizontal_y {
                let (Some(x0), Some(x1)) = (border.vertical_x.first(), border.vertical_x.last())
                else {
                    continue;
                };
                svg.push_str(&format!("<line x1=\"{x0:.1}\" y1=\"{y:.1}\" x2=\"{x1:.1}\" y2=\"{y:.1}\" stroke=\"#000000\" stroke-width=\"0.8\"/>"));
            }
            svg.push_str("</g>");
        }
        svg.push_str(&format!(
            "<g class=\"rjtd-native-control-table-text\" data-table-candidate-index=\"{}\" data-projection-kind=\"nativeControlTableTextProjection\" data-source-backed=\"true\" data-reference-backed=\"false\" data-decoded=\"false\" data-geometry-decoded=\"false\">",
            projection.candidate_index
        ));
        for slot in &projection.slots {
            svg.push_str(&format!(
                "<g data-row=\"{}\" data-column=\"{}\" data-header-offset-units=\"{}\" data-line-mark-record-index=\"{}\" data-page-mark-pitch-mm100=\"{}\" data-font-size-basis=\"{}\">",
                slot.row_index,
                slot.column_index,
                slot.header_offset_units,
                slot.line_mark_record_index,
                slot.page_mark_pitch_mm100,
                slot.font_size.basis,
            ));
            push_svg_text_run(
                svg,
                "rjtd-text rjtd-native-control-table-cell",
                slot.x,
                slot.baseline_y,
                font_family,
                slot.font_size.px,
                fallback_text_fill_color(),
                &slot.text,
                None,
            );
            svg.push_str("</g>");
        }
        svg.push_str("</g>");
    }
}

pub(crate) fn push_native_control_flow_text_svg(
    svg: &mut String,
    projection: Option<&NativeControlFlowTextProjection>,
    font_family: &str,
) {
    let Some(projection) = projection else {
        return;
    };
    svg.push_str(
        "<g class=\"rjtd-native-control-flow-text\" data-projection-kind=\"nativeControlFlowTextProjection\" data-source-backed=\"true\" data-reference-backed=\"false\" data-decoded=\"false\" data-geometry-decoded=\"false\">",
    );
    for slot in &projection.slots {
        svg.push_str(&format!(
            "<g data-line-mark-record-index=\"{}\" data-page-mark-pitch-mm100=\"{}\" data-leading-ascii-space-count=\"{}\" data-preceding-header-offset-units=\"{}\" data-preceding-header-extent-units=\"{}\" data-font-size-basis=\"{}\">",
            slot.line_mark_record_index,
            slot.page_mark_pitch_mm100,
            slot.leading_ascii_space_count,
            slot.preceding_header_offset_units,
            slot.preceding_header_extent_units,
            slot.font_size.basis,
        ));
        push_svg_text_run(
            svg,
            "rjtd-text rjtd-native-control-flow-line",
            slot.x,
            slot.baseline_y,
            font_family,
            slot.font_size.px,
            fallback_text_fill_color(),
            &slot.text,
            None,
        );
        svg.push_str("</g>");
    }
    svg.push_str("</g>");
}

#[derive(Debug)]
struct NativeControlFlowSourceLine {
    text: String,
    source_span: TextSourceSpan,
    leading_ascii_space_count: usize,
    line_mark_record_index: usize,
}

fn native_flow_preceding_header(
    bytes: &[u8],
    range_start: usize,
    entry_byte_start: usize,
) -> Option<ShanaiLanLineHeader> {
    let start = range_start.min(entry_byte_start) & !1;
    let headers = (start..entry_byte_start)
        .step_by(2)
        .filter_map(|offset| shanai_lan_line_header_at(bytes, offset))
        .filter(|header| header.end <= entry_byte_start)
        .filter(|header| header.offset_units == 0 && header.extent_units != 0)
        .collect::<Vec<_>>();
    let [header] = headers.as_slice() else {
        return None;
    };
    Some(*header)
}

fn native_flow_source_lines(
    entry: &DocumentTextMapEntry,
) -> Option<Vec<NativeControlFlowSourceLine>> {
    if entry.text().contains('\r') || !entry.text().is_ascii() {
        return None;
    }
    let mut offset_units = entry.unit_start();
    let mut lines = Vec::new();
    for chunk in entry.text().split_inclusive('\n') {
        let text = chunk.strip_suffix('\n').unwrap_or(chunk);
        let chunk_units = chunk.encode_utf16().count();
        let text_units = text.encode_utf16().count();
        if !text.is_empty() {
            if !text
                .chars()
                .all(|character| character.is_ascii_graphic() || character == ' ')
            {
                return None;
            }
            let leading_ascii_space_count = text
                .chars()
                .take_while(|character| *character == ' ')
                .count();
            if leading_ascii_space_count == text.chars().count() {
                return None;
            }
            lines.push(NativeControlFlowSourceLine {
                text: text.to_string(),
                source_span: TextSourceSpan::new(
                    offset_units * 2,
                    (offset_units + text_units) * 2,
                    offset_units,
                    offset_units + text_units,
                ),
                leading_ascii_space_count,
                line_mark_record_index: 0,
            });
        }
        offset_units = offset_units.checked_add(chunk_units)?;
    }
    (offset_units == entry.unit_end()).then_some(lines)
}

fn native_control_table_text_projection(
    document: &Document,
    layout: PageLayout,
    candidate: &TableCandidate,
) -> Option<NativeControlTableTextProjection> {
    if candidate.basis() != TextCountRangeOverlapBasis::Unit
        || !candidate.is_document_text_control_run_candidate()
    {
        return None;
    }
    let grid = candidate.column_segment_grid_candidate()?;
    let rows = table_candidate_document_text_line_header_rows(document, candidate);
    if rows.len() != grid.row_count() || rows.len() != candidate.intervals().len() {
        return None;
    }
    let bytes = document_text_raw_stream(document)?;
    let parents = rows
        .iter()
        .map(|row| native_parent_row_header(bytes, row.source_start))
        .collect::<Option<Vec<_>>>()?;
    let grid_extent = parents.first()?.grid_extent;
    if grid_extent == 0
        || parents.iter().any(|parent| {
            parent.grid_extent != grid_extent
                || parent.left_edge_units != parents[0].left_edge_units
        })
    {
        return None;
    }
    let resolved = table_grid_resolved_line_mark_rows_for_rows(document, candidate, &rows);
    if resolved.len() != rows.len() {
        return None;
    }
    let page_mark = document.page_marks().first()?;
    let mut page_entry = None;
    for ((row, parent), resolved) in rows.iter().zip(&parents).zip(&resolved) {
        let interval = resolved.interval;
        if interval.unit_start != parent.start_unit || interval.unit_end != row.source_end {
            return None;
        }
        let entry = table_grid_page_mark_entry_for_line_mark_record(
            Some(page_mark),
            interval.record_index,
        )?;
        if entry.line_start()? != 0 || entry.index()? != 0 {
            return None;
        }
        if let Some(previous) = page_entry
            && previous != entry.row_index()
        {
            return None;
        }
        page_entry = Some(entry.row_index());
    }
    let pitch_mm100 = page_mark
        .entries()
        .get(page_entry?)?
        .u16_fields()
        .get(21)
        .copied()?;
    let pitch = hundredth_millimeters_to_css_px(u32::from(pitch_mm100));
    if !(APP_FONT_SIZE_PX..=APP_LINE_HEIGHT_PX * 1.25).contains(&pitch) {
        return None;
    }
    let unit_px = layout.body_width_px() / f32::from(grid_extent);
    if !unit_px.is_finite() || unit_px <= 0.0 {
        return None;
    }
    let default_font = document_default_font_size_px(document)?;
    let resolver = document_text_style_resolver(document)?;
    let document_text_map = document_text_raw_stream(document).map(map_document_text)?;
    let sparse_rows = sparse_document_text_control_table_rows(document_text_map.entries());
    let first_headers = native_matched_cell_headers(rows.first()?, candidate.intervals().first()?)?;
    let first_offsets = first_headers
        .iter()
        .map(|header| header.offset_units)
        .collect::<Vec<_>>();
    let first_ends = first_headers
        .iter()
        .map(|header| header.extent_units)
        .collect::<Vec<_>>();
    if first_offsets.len() != grid.column_count()
        || first_offsets.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return None;
    }

    let mut slots = Vec::new();
    for (((row_index, interval), row), resolved) in candidate
        .intervals()
        .iter()
        .enumerate()
        .zip(&rows)
        .zip(&resolved)
    {
        let headers = native_matched_cell_headers(row, interval)?;
        if row.expected_cell_count != grid.column_count()
            || headers.len() != grid.column_count()
            || headers
                .iter()
                .any(|header| header.extent_units > grid_extent)
            || headers
                .iter()
                .map(|header| header.offset_units)
                .collect::<Vec<_>>()
                != first_offsets
            || headers
                .iter()
                .map(|header| header.extent_units)
                .collect::<Vec<_>>()
                != first_ends
            || interval.column_segments().len() != grid.column_count()
        {
            return None;
        }
        for (column_index, (segment, header)) in
            interval.column_segments().iter().zip(headers).enumerate()
        {
            let (start, end) = (segment.source_start()?, segment.source_end()?);
            if header.end / 2 != start || end <= start || segment.text().contains(['\n', '\r']) {
                return None;
            }
            let span = TextSourceSpan::new(start * 2, end * 2, start, end);
            let font_size = document_text_font_size(&resolver, &span, Some(default_font))?;
            let raw_text =
                table_grid_segment_source_raw_text(Some(&document_text_map), candidate, segment)
                    .unwrap_or_else(|| segment.text().to_string());
            let leading_spaces = raw_text
                .chars()
                .take_while(|character| *character == ' ')
                .count();
            if raw_text.trim_matches(' ') != segment.text() {
                return None;
            }
            let x = layout.margin_left_px()
                + (f32::from(header.offset_units) + leading_spaces as f32 * 2.0) * unit_px;
            let baseline_y = layout.margin_top_px()
                + resolved.interval.record_index as f32 * pitch
                + font_size.px;
            if !x.is_finite()
                || !baseline_y.is_finite()
                || x > layout.width_px()
                || baseline_y > layout.height_px()
            {
                return None;
            }
            slots.push(NativeControlTableTextSlot {
                candidate_index: candidate.index(),
                text: segment.text().to_string(),
                x,
                baseline_y,
                font_size,
                source_span: span,
                row_index,
                column_index,
                header_offset_units: header.offset_units,
                line_mark_record_index: resolved.interval.record_index,
                page_mark_pitch_mm100: pitch_mm100,
            });
        }
    }
    let border = native_control_table_border_projection(
        layout,
        grid_extent,
        parents[0].left_edge_units,
        &first_ends,
        &slots,
        native_control_table_trailing_empty_row_record(
            bytes,
            resolved.last()?.interval.unit_end,
            grid_extent,
            parents[0].left_edge_units,
            &sparse_rows,
            &shanai_lan_line_mark_intervals(document),
        ),
    );
    Some(NativeControlTableTextProjection {
        candidate_index: candidate.index(),
        slots,
        border,
    })
}

fn native_control_table_border_projection(
    layout: PageLayout,
    grid_extent: u16,
    left_edge_units: u16,
    header_ends: &[u16],
    slots: &[NativeControlTableTextSlot],
    trailing_empty_row_record: Option<usize>,
) -> Option<NativeControlTableBorderProjection> {
    let first = slots.first()?;
    if slots
        .iter()
        .any(|slot| (slot.font_size.px - first.font_size.px).abs() > f32::EPSILON)
    {
        return None;
    }
    let mut rows = slots
        .iter()
        .filter(|slot| slot.column_index == 0)
        .collect::<Vec<_>>();
    rows.sort_by_key(|slot| slot.row_index);
    if rows.is_empty()
        || (rows.len() == 1 && trailing_empty_row_record.is_none())
        || rows
            .windows(2)
            .any(|pair| pair[0].line_mark_record_index >= pair[1].line_mark_record_index)
    {
        return None;
    }
    let mut records = Vec::with_capacity(rows.len() + 1);
    records.push(rows.first()?.line_mark_record_index.checked_sub(1)?);
    records.extend(
        rows.windows(2)
            .map(|pair| pair[1].line_mark_record_index - 1),
    );
    records.push(
        trailing_empty_row_record.unwrap_or(rows.last()?.line_mark_record_index.checked_add(1)?),
    );
    let unit_px = layout.body_width_px() / f32::from(grid_extent);
    let vertical_x = std::iter::once(left_edge_units.checked_add(1)?)
        .chain(
            header_ends
                .iter()
                .map(|end| end.checked_add(1))
                .collect::<Option<Vec<_>>>()?,
        )
        .map(|unit| layout.margin_left_px() + f32::from(unit) * unit_px)
        .collect::<Vec<_>>();
    let pitch = hundredth_millimeters_to_css_px(u32::from(first.page_mark_pitch_mm100));
    let horizontal_y = records
        .into_iter()
        .map(|record| layout.margin_top_px() + record as f32 * pitch + first.font_size.px / 2.0)
        .collect::<Vec<_>>();
    Some(NativeControlTableBorderProjection {
        vertical_x,
        horizontal_y,
    })
}

fn native_control_table_trailing_empty_row_record(
    bytes: &[u8],
    start_unit: usize,
    grid_extent: u16,
    left_edge_units: u16,
    rows: &[DocumentTextControlTableRow],
    intervals: &[ShanaiLanLineMarkInterval],
) -> Option<usize> {
    let start = rows.iter().position(|row| row.source_start == start_unit)?;
    let mut final_record = None;
    let mut next_unit = start_unit;
    for row in rows.iter().skip(start).take(64) {
        if row.source_start != next_unit || row.cells.iter().any(|cell| !cell.text.is_empty()) {
            break;
        }
        let Some(parent) = native_parent_row_header(bytes, row.source_start) else {
            break;
        };
        if parent.start_unit != row.source_start
            || parent.grid_extent != grid_extent
            || parent.left_edge_units != left_edge_units
        {
            break;
        }
        let Some(interval) = intervals.iter().find(|interval| {
            interval.unit_start == row.source_start && interval.unit_end == row.source_end
        }) else {
            break;
        };
        final_record = Some(interval.record_index);
        next_unit = row.source_end;
    }
    final_record
}

fn native_matched_cell_headers<'a>(
    row: &'a TableCandidateLineHeaderRow,
    interval: &TableCandidateInterval,
) -> Option<Vec<&'a ShanaiLanLineHeader>> {
    let mut headers: Vec<&ShanaiLanLineHeader> = Vec::new();
    for segment in interval.column_segments() {
        let start = segment.source_start()?;
        let header = row
            .headers
            .iter()
            .filter(|header| header.end / 2 == start)
            .collect::<Vec<_>>();
        if header.len() != 1
            || headers
                .iter()
                .any(|existing| existing.start == header[0].start)
        {
            return None;
        }
        headers.push(header[0]);
    }
    Some(headers)
}

fn native_parent_row_header(
    bytes: &[u8],
    row_start_unit: usize,
) -> Option<NativeControlTableRowHeader> {
    let units = bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<_>>();
    let mut matches = Vec::new();
    for start in row_start_unit.saturating_sub(512)..=row_start_unit {
        if units.get(start..start + 4)? != [0x001c, 0x0010, *units.get(start + 2)?, 0] {
            continue;
        }
        let length = usize::from(*units.get(start + 2)?);
        let end = start.checked_add(length)?;
        if length < 13 || (start != row_start_unit && end != row_start_unit) {
            continue;
        }
        if units.get(end.checked_sub(4)?..end)? != [length as u16, 0, 0x0010, 0x001f] {
            continue;
        }
        matches.push(NativeControlTableRowHeader {
            start_unit: start,
            grid_extent: *units.get(start + 6)?,
            left_edge_units: *units.get(start + 8)?,
        });
    }
    (matches.len() == 1).then(|| matches[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_empty_rows_preserve_the_boundary_at_a_non_table_transition() {
        let mut bytes = vec![0; 192];
        for unit in [16, 48, 64] {
            let words = [
                0x001c_u16, 0x0010, 13, 0, 0x008f, 0, 160, 0, 14, 13, 0, 0x0010, 0x001f,
            ];
            for (index, word) in words.iter().enumerate() {
                bytes[(unit + index) * 2..(unit + index + 1) * 2]
                    .copy_from_slice(&word.to_be_bytes());
            }
        }
        let row = |index, start, end| DocumentTextControlTableRow {
            index,
            source_start: start,
            source_end: end,
            cells: Vec::new(),
        };
        let interval = |record_index, unit_start, unit_end| ShanaiLanLineMarkInterval {
            record_index,
            unit_start,
            unit_end,
            flag_word: 2,
        };
        let rows = [row(0, 16, 48), row(1, 48, 80), row(2, 80, 81)];
        let intervals = [
            interval(2, 16, 48),
            interval(3, 48, 80),
            interval(4, 80, 81),
        ];
        assert_eq!(
            native_control_table_trailing_empty_row_record(&bytes, 16, 160, 14, &rows, &intervals),
            Some(3)
        );

        // A gap in the row model must not be skipped even if a later header matches.
        assert_eq!(
            native_control_table_trailing_empty_row_record(
                &bytes,
                16,
                160,
                14,
                &[rows[0].clone(), row(1, 64, 80)],
                &intervals,
            ),
            Some(2)
        );
        assert_eq!(
            native_control_table_trailing_empty_row_record(
                &bytes,
                16,
                160,
                14,
                &rows,
                &[interval(2, 16, 47)],
            ),
            None
        );
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn local_native_control_table_projection_tracks_controlled_grid_coordinates() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/page01-grid",
        );
        for name in [
            "PAGE 01",
            "PAGE 01_down_4Low",
            "PAGE 01_right_4Tick",
            "PAGE 01_DOWNTEST_1LINE",
            "../baseline-sweep/000_base_a",
            "../baseline-sweep/013_table_moved_right",
            "../baseline-sweep/020_row1_height_plus",
        ] {
            let path = root.join(format!("{name}.jtd"));
            let document = parse_document(&std::fs::read(path).unwrap()).unwrap();
            let layout =
                page_layout_with_source_margins(&document, page_layout_from_document(&document));
            let projections = native_control_table_text_projections(
                &document,
                layout,
                1,
                WritingMode::Horizontal,
            );
            assert_eq!(projections.len(), 1, "{name}");
            assert_eq!(projections[0].slots.len(), 6, "{name}");
            let row_records = projections[0]
                .slots
                .iter()
                .filter(|slot| slot.column_index == 0)
                .map(|slot| slot.line_mark_record_index)
                .collect::<Vec<_>>();
            let expected_records = match name {
                "PAGE 01" => vec![2, 4],
                "PAGE 01_down_4Low" => vec![6, 8],
                "PAGE 01_right_4Tick" => vec![2, 4],
                "PAGE 01_DOWNTEST_1LINE" => vec![3, 5],
                "../baseline-sweep/000_base_a" => vec![1, 3, 5],
                "../baseline-sweep/013_table_moved_right" => vec![1, 3, 5],
                "../baseline-sweep/020_row1_height_plus" => vec![1, 5, 7],
                _ => row_records.clone(),
            };
            assert_eq!(row_records, expected_records, "{name}");
            if name == "PAGE 01" {
                let first = &projections[0].slots[0];
                assert!((first.x - 120.472_44).abs() < 0.01);
                assert_eq!(first.header_offset_units, 2);
                assert_eq!(first.page_mark_pitch_mm100, 592);
            }
            assert!(
                projections[0]
                    .slots
                    .windows(2)
                    .any(|pair| pair[1].baseline_y > pair[0].baseline_y),
                "{name}"
            );
        }
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn native_control_table_projection_rejects_wrapped_control_rows() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/baseline-sweep/050_wrapped_one_cell.jtd");
        let document = parse_document(&std::fs::read(path).unwrap()).unwrap();
        let layout =
            page_layout_with_source_margins(&document, page_layout_from_document(&document));
        assert!(
            native_control_table_text_projections(&document, layout, 1, WritingMode::Horizontal)
                .is_empty()
        );
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn local_native_control_table_projection_admits_only_the_verified_horizontal_series() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus");
        let admitted = [
            "baseline-sweep/000_base_a",
            "baseline-sweep/001_base_b_resave",
            "baseline-sweep/010_table_moved_down_small",
            "baseline-sweep/011_table_moved_down_large",
            "baseline-sweep/013_table_moved_right",
            "baseline-sweep/020_row1_height_plus",
            "baseline-sweep/021_row2_height_plus",
            "baseline-sweep/022_row3_height_plus",
            "baseline-sweep/040_top_margin_plus",
            "page01-grid/PAGE 01",
            "page01-grid/PAGE 01_DOWNTEST_1LINE",
            "page01-grid/PAGE 01_DOWNTEST_BASE",
            "page01-grid/PAGE 01_down_1Low",
            "page01-grid/PAGE 01_down_2Low",
            "page01-grid/PAGE 01_down_3Low",
            "page01-grid/PAGE 01_down_4Low",
            "page01-grid/PAGE 01_right_1Tick",
            "page01-grid/PAGE 01_right_2Tick",
            "page01-grid/PAGE 01_right_3Tick",
            "page01-grid/PAGE 01_right_4Tick",
        ];
        for relative in admitted {
            let document =
                parse_document(&std::fs::read(root.join(format!("{relative}.jtd"))).unwrap())
                    .unwrap();
            let layout =
                page_layout_with_source_margins(&document, page_layout_from_document(&document));
            let projections = native_control_table_text_projections(
                &document,
                layout,
                1,
                WritingMode::Horizontal,
            );
            assert_eq!(projections.len(), 1, "{relative}");
            assert_eq!(projections[0].slots.len(), 6, "{relative}");
            let core = DocumentCore::from_document(document);
            let svg = core.render_page_svg(0).unwrap();
            assert!(svg.contains("rjtd-native-control-table-text"), "{relative}");
            assert_eq!(svg.matches(">R01C01</text>").count(), 1, "{relative}");
            let layers = core.get_page_layer_tree(0).unwrap();
            assert_eq!(
                layers.matches("nativeControlTableTextProjection").count(),
                6,
                "{relative}"
            );
        }

        let relative = "baseline-sweep/050_wrapped_one_cell";
        let document =
            parse_document(&std::fs::read(root.join(format!("{relative}.jtd"))).unwrap()).unwrap();
        let layout =
            page_layout_with_source_margins(&document, page_layout_from_document(&document));
        assert!(
            native_control_table_text_projections(&document, layout, 1, WritingMode::Horizontal)
                .is_empty(),
            "{relative}"
        );
    }

    #[test]
    #[ignore = "requires local source-y probe corpus"]
    fn local_native_control_flow_projection_requires_two_bracketing_native_tables() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../../rjtd-testdata/local-samples/ichitaro-source-y-probe/corpus/baseline-sweep",
        );
        let document =
            parse_document(&std::fs::read(root.join("070_two_tables_vertical.jtd")).unwrap())
                .unwrap();
        let layout =
            page_layout_with_source_margins(&document, page_layout_from_document(&document));
        let tables =
            native_control_table_text_projections(&document, layout, 1, WritingMode::Horizontal);
        assert_eq!(tables.len(), 2);
        assert_eq!(
            tables.iter().map(|table| table.slots.len()).sum::<usize>(),
            8
        );
        let flow = native_control_flow_text_projection(
            &document,
            layout,
            1,
            WritingMode::Horizontal,
            &tables,
        )
        .expect("two bracketing native control tables with a source-mapped flow run");
        assert_eq!(flow.slots.len(), 2);
        assert_eq!(
            flow.slots
                .iter()
                .map(|slot| slot.text.as_str())
                .collect::<Vec<_>>(),
            vec!["        BETWEEN01", "        BETWEEN02"]
        );
        assert_eq!(
            flow.slots
                .iter()
                .map(|slot| slot.line_mark_record_index)
                .collect::<Vec<_>>(),
            vec![7, 9]
        );
        assert!(flow.slots.iter().all(|slot| {
            slot.page_mark_pitch_mm100 == 592
                && slot.leading_ascii_space_count == 8
                && slot.preceding_header_offset_units == 0
                && slot.preceding_header_extent_units == 14
        }));

        let core = DocumentCore::from_document(document);
        let svg = core.render_page_svg(0).unwrap();
        assert!(svg.contains("rjtd-native-control-flow-text"));
        assert_eq!(svg.matches("BETWEEN01").count(), 1);
        assert_eq!(svg.matches("BETWEEN02").count(), 1);
        let layers = core.get_page_layer_tree(0).unwrap();
        assert_eq!(layers.matches("nativeControlFlowTextProjection").count(), 2);

        let bytes = document_text_raw_stream(&core.document).unwrap();
        let rows = sparse_document_text_control_table_rows(map_document_text(bytes).entries());
        assert_eq!(
            native_control_table_trailing_empty_row_record(
                bytes,
                770,
                160,
                14,
                &rows,
                &shanai_lan_line_mark_intervals(&core.document),
            ),
            Some(17)
        );

        let single_table =
            parse_document(&std::fs::read(root.join("000_base_a.jtd")).unwrap()).unwrap();
        let single_layout = page_layout_with_source_margins(
            &single_table,
            page_layout_from_document(&single_table),
        );
        let single_tables = native_control_table_text_projections(
            &single_table,
            single_layout,
            1,
            WritingMode::Horizontal,
        );
        assert_eq!(single_tables.len(), 1);
        assert!(
            native_control_flow_text_projection(
                &single_table,
                single_layout,
                1,
                WritingMode::Horizontal,
                &single_tables,
            )
            .is_none()
        );
    }
}
