use super::*;

pub(super) const LAYOUT_BOX_PATH: &str = "/LayoutBox";

pub(super) const LAYOUT_MAP_DELTA_MIN: isize = -4096;

pub(super) const LAYOUT_MAP_DELTA_MAX: isize = 4096;

pub(super) const APP_PAGE_WIDTH_PX: f32 = 794.0;

pub(super) const APP_PAGE_HEIGHT_PX: f32 = 1123.0;

pub(super) const APP_PAGE_MARGIN_PX: f32 = 72.0;

pub(super) const APP_PAGE_DECORATION_FONT_SIZE_PX: f32 = 13.0;

pub(super) const TSAITEN_REFERENCE_PAGE_WIDTH_PX: f32 = 793.7;

pub(super) const TSAITEN_REFERENCE_PAGE_HEIGHT_PX: f32 = 1122.5;

pub(super) const SHANAI_LAN_REFERENCE_PAGE_WIDTH_PX: f32 = 1122.5;

pub(super) const SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX: f32 = 793.7;

pub(super) const LAYOUT_BOX_RECORD_PREFIX: &[u8; 4] = &[0x02, 0x01, 0x00, 0x08];

pub(super) const LAYOUT_BOX_RECORD_ORIGIN_FIELD_OFFSET: usize = 20;

pub(super) const LAYOUT_BOX_RECORD_Y_FIELD_OFFSET: usize = 24;

pub(super) const LAYOUT_BOX_RECORD_WIDTH_FIELD_OFFSET: usize = 72;

pub(super) const LAYOUT_BOX_RECORD_X_FIELD_OFFSET: usize = 84;

pub(super) const PAGE_FRAME_MIN_PATTERN_BAR_WIDTH_RATIO: f32 = 0.5;

pub(super) const PAGE_FRAME_MIN_PATTERN_BAR_Y_PX: f32 = 96.0;

pub(super) const PAGE_FRAME_MAX_PATTERN_BAR_HEIGHT_PX: f32 = 32.0;

pub(super) const PAGE_FRAME_MIN_TITLE_ASPECT_RATIO: f32 = 2.0;

pub(super) const PAGE_FRAME_PATTERN_DOT_RADIUS_PX: f32 = 0.75;

pub(super) const PAGE_FRAME_TIME_CAPTION_GAP_PX: f32 = 4.0;

pub(super) const DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET: usize = 16;

pub(super) const DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET: usize = 20;

pub(super) const PAGE_LAYOUT_STYLE_RECORD_CODE: u16 = 0x4444;

pub(super) const PAGE_LAYOUT_STYLE_PAGE_SIZE_SUBRECORD_CODE: u16 = 0x4001;

pub(super) const PAGE_LAYOUT_STYLE_PAGE_SIZE_WIDTH_OFFSET: usize = 4;

pub(super) const PAGE_LAYOUT_STYLE_PAGE_SIZE_HEIGHT_OFFSET: usize = 8;

pub(super) const PAGE_LAYOUT_STYLE_PAYLOAD_WIDTH_OFFSET: usize = 24;

pub(super) const PAGE_LAYOUT_STYLE_PAYLOAD_HEIGHT_OFFSET: usize = 28;

pub(super) fn page_layout_is_close_to_mm(
    layout: PageLayout,
    width_mm: f32,
    height_mm: f32,
) -> bool {
    let width = millimeters_to_css_px(width_mm);
    let height = millimeters_to_css_px(height_mm);
    (layout.width_px() - width).abs() < 1.0 && (layout.height_px() - height).abs() < 1.0
}

pub(super) fn page_layout_from_document(document: &Document) -> PageLayout {
    decoded_page_layout_from_styles(document.unknown_styles())
        .unwrap_or_default()
        .with_portrait_orientation()
}

pub(super) fn decoded_page_layout_from_styles(styles: &[UnknownStyle]) -> Option<PageLayout> {
    styles
        .iter()
        .find(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        .and_then(|style| page_layout_from_page_layout_style(style.payload()))
        .or_else(|| {
            styles
                .iter()
                .find(|style| style.name() == Some(DOCUMENT_VIEW_STYLES_PATH))
                .and_then(|style| page_layout_from_document_view_styles(style.payload()))
        })
}

pub(super) fn page_layout_from_document_view_styles(bytes: &[u8]) -> Option<PageLayout> {
    page_layout_from_encoded_mm100_shift8(
        read_be32_at(bytes, DOCUMENT_VIEW_STYLES_PAGE_WIDTH_OFFSET)?,
        read_be32_at(bytes, DOCUMENT_VIEW_STYLES_PAGE_HEIGHT_OFFSET)?,
    )
}

pub(super) fn page_layout_from_page_layout_style(bytes: &[u8]) -> Option<PageLayout> {
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
                    page_layout_from_encoded_mm100_shift8(
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
            page_layout_from_encoded_mm100_shift8(
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

pub(super) fn page_layout_from_encoded_mm100_shift8(
    width_field: u32,
    height_field: u32,
) -> Option<PageLayout> {
    let width_mm100 = width_field >> 8;
    let height_mm100 = height_field >> 8;
    if !paper_size_mm100_is_plausible(width_mm100) || !paper_size_mm100_is_plausible(height_mm100) {
        return None;
    }
    Some(PageLayout::new(
        hundredth_millimeters_to_css_px(width_mm100),
        hundredth_millimeters_to_css_px(height_mm100),
    ))
}

pub(super) fn layout_map_bases() -> &'static [LayoutMapBase] {
    &[
        LayoutMapBase::Unit,
        LayoutMapBase::UnitTimes2,
        LayoutMapBase::UnitDiv2Floor,
        LayoutMapBase::UnitDiv2Ceil,
    ]
}

pub(super) fn page_be32_field_points(page_mark: &PageMark) -> Vec<usize> {
    page_mark
        .entries()
        .iter()
        .flat_map(|entry| {
            entry
                .raw()
                .chunks_exact(4)
                .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as usize)
        })
        .collect()
}

pub(super) fn document_page_decoration_paired_slot_pairs(document: &Document) -> Vec<(u16, u16)> {
    let mut pairs = BTreeSet::new();
    document
        .unknown_styles()
        .iter()
        .filter(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        .for_each(|style| {
            for record in summarize_style_stream(style.payload()).records() {
                pairs.extend(page_layout_record_active_decoration_pairs(record));
            }
        });
    pairs.into_iter().collect()
}

pub(super) fn page_layout_record_active_decoration_pairs(
    record: &StyleStreamRecordSummary,
) -> Vec<(u16, u16)> {
    let active_slots = record
        .subrecords()
        .iter()
        .filter_map(|subrecord| {
            let code = subrecord.code();
            let slot = code >> 8;
            let part = code & 0xff;
            if !(0x31..=0x39).contains(&slot) || part != 0x05 {
                return None;
            }
            subrecord
                .payload()
                .first()
                .is_some_and(|byte| *byte != 0)
                .then_some(slot)
        })
        .collect::<BTreeSet<_>>();

    [(0x32, 0x33), (0x34, 0x35), (0x36, 0x37), (0x38, 0x39)]
        .iter()
        .filter(|(left, right)| active_slots.contains(left) && active_slots.contains(right))
        .copied()
        .collect()
}

pub(super) fn page_decoration_mark_evidence(
    document: &Document,
    page_index: usize,
) -> Option<PageDecorationMarkEvidence> {
    let page_index_u32 = u32::try_from(page_index).ok();
    let page_mark = document.page_marks().first();
    let paper_mark = document.paper_marks().first();

    let page_entry = page_mark.and_then(|mark| {
        page_index_u32
            .and_then(|index| {
                mark.entries()
                    .iter()
                    .find(|entry| entry.index() == Some(index))
            })
            .or_else(|| mark.entries().get(page_index))
    });
    let paper_entry = paper_mark.and_then(|mark| {
        page_index_u32
            .and_then(|index| mark.entries().iter().find(|entry| entry.index() == index))
            .or_else(|| mark.entries().get(page_index))
    });

    if page_entry.is_none() && paper_entry.is_none() {
        return None;
    }

    let row_index_aligned = page_entry
        .zip(paper_entry)
        .is_some_and(|(page, paper)| page.row_index() == paper.row_index());
    let mark_index_aligned = page_entry
        .and_then(DocumentPageMarkEntry::index)
        .zip(paper_entry.map(DocumentPaperMarkEntry::index))
        .is_some_and(|(page, paper)| page == paper);
    let entry_count_aligned = page_mark
        .zip(paper_mark)
        .is_some_and(|(page, paper)| page.entries().len() == paper.entries().len());

    Some(PageDecorationMarkEvidence {
        page_index,
        page_mark_entry_index: page_entry.map(DocumentPageMarkEntry::row_index),
        page_mark_index: page_entry.and_then(DocumentPageMarkEntry::index),
        page_mark_flags: page_entry.and_then(DocumentPageMarkEntry::flags),
        page_mark_line_start: page_entry.and_then(DocumentPageMarkEntry::line_start),
        page_mark_line_end: page_entry.and_then(DocumentPageMarkEntry::line_end),
        page_mark_u16_fields: page_entry
            .map(|entry| entry.u16_fields().to_vec())
            .unwrap_or_default(),
        paper_mark_entry_index: paper_entry.map(DocumentPaperMarkEntry::row_index),
        paper_mark_index: paper_entry.map(DocumentPaperMarkEntry::index),
        paper_mark_flags: paper_entry.map(DocumentPaperMarkEntry::flags),
        row_index_aligned,
        mark_index_aligned,
        entry_count_aligned,
    })
}

pub(super) fn document_page_decoration_slot_evidence(
    document: &Document,
) -> Vec<PageDecorationSlotEvidence> {
    let mut evidence = Vec::new();
    document
        .unknown_styles()
        .iter()
        .filter(|style| style.name() == Some(PAGE_LAYOUT_STYLE_PATH))
        .for_each(|style| {
            let summary = summarize_style_stream(style.payload());
            for (record_index, record) in summary.records().iter().enumerate() {
                evidence.extend(page_layout_record_decoration_slot_evidence(
                    record_index,
                    record,
                ));
            }
        });
    evidence
}

pub(super) fn page_layout_record_decoration_slot_evidence(
    record_index: usize,
    record: &StyleStreamRecordSummary,
) -> Vec<PageDecorationSlotEvidence> {
    let mut slots = BTreeMap::new();
    for subrecord in record.subrecords() {
        let code = subrecord.code();
        let slot = code >> 8;
        let part = code & 0xff;
        if !(0x31..=0x39).contains(&slot) || !(0x04..=0x07).contains(&part) {
            continue;
        }
        let evidence = slots
            .entry(slot)
            .or_insert_with(|| PageDecorationSlotEvidence {
                record_index,
                record_offset: record.offset(),
                record_label: record.label().map(str::to_string),
                slot,
                part04: None,
                part05: None,
                part06: None,
                part07: None,
            });
        match part {
            0x04 => evidence.part04 = Some(subrecord.payload().to_vec()),
            0x05 => evidence.part05 = Some(subrecord.payload().to_vec()),
            0x06 => evidence.part06 = Some(subrecord.payload().to_vec()),
            0x07 => evidence.part07 = Some(subrecord.payload().to_vec()),
            _ => {}
        }
    }
    slots.into_values().collect()
}

pub(super) fn page_chapter_title(
    lines: &[PageTextLine],
    chapter_titles: &[String],
) -> Option<String> {
    lines.iter().find_map(|line| {
        let trimmed = line.text().trim();
        chapter_titles
            .iter()
            .find(|title| trimmed.starts_with(title.as_str()))
            .cloned()
    })
}

pub(super) fn projected_page_breaks(document: &Document) -> BTreeMap<usize, Vec<usize>> {
    let mut breaks = BTreeMap::<usize, Vec<usize>>::new();
    for control in projected_text_controls(document) {
        if control.code != DOCUMENT_TEXT_PAGE_BREAK_CONTROL {
            continue;
        }
        breaks
            .entry(control.paragraph_index)
            .or_default()
            .push(control.char_offset);
    }
    for offsets in breaks.values_mut() {
        offsets.sort_unstable();
        offsets.dedup();
    }
    breaks
}

pub(super) fn split_line_at_page_breaks(
    line: PageTextLine,
    break_offsets: &[usize],
) -> Vec<PageLineSegment> {
    let Some(paragraph_index) = line.paragraph_index() else {
        return vec![PageLineSegment {
            line,
            break_after: false,
        }];
    };

    let mut segments = Vec::new();
    let mut segment_start = line.char_start();
    for break_offset in break_offsets.iter().copied() {
        if break_offset < segment_start || break_offset > line.char_end() {
            continue;
        }
        let text = text_by_char_range(
            line.text(),
            segment_start - line.char_start(),
            break_offset - line.char_start(),
        );
        if !text.is_empty() || break_offset == line.char_start() {
            segments.push(PageLineSegment {
                line: PageTextLine::new(text, Some(paragraph_index), segment_start, break_offset),
                break_after: true,
            });
        } else if let Some(last) = segments.last_mut() {
            last.break_after = true;
        } else {
            segments.push(PageLineSegment {
                line: PageTextLine::new(
                    String::new(),
                    Some(paragraph_index),
                    break_offset,
                    break_offset,
                ),
                break_after: true,
            });
        }
        segment_start = break_offset;
    }

    if segment_start < line.char_end() {
        segments.push(PageLineSegment {
            line: PageTextLine::new(
                text_by_char_range(
                    line.text(),
                    segment_start - line.char_start(),
                    line.char_end() - line.char_start(),
                ),
                Some(paragraph_index),
                segment_start,
                line.char_end(),
            ),
            break_after: false,
        });
    }

    if segments.is_empty() {
        segments.push(PageLineSegment {
            line,
            break_after: false,
        });
    }

    segments
}

pub(super) fn force_page_break(
    pages: &mut Vec<Vec<PageTextLine>>,
    current_page: &mut Vec<PageTextLine>,
) {
    while current_page
        .last()
        .is_some_and(|line| line.text().is_empty() && line.paragraph_index().is_none())
    {
        current_page.pop();
    }
    if current_page.iter().any(|line| !line.text().is_empty()) {
        pages.push(std::mem::take(current_page));
    } else {
        current_page.clear();
    }
}

pub(super) fn ok_page_count_json(page_count: u32) -> String {
    json_ok_with(&format!("\"pageCount\":{page_count}"))
}

pub(super) fn page_layer_tree_json(
    core: &DocumentCore,
    lines: &[PageTextLine],
    profile: &str,
    page_num: u32,
) -> String {
    let layout = core.page_layout;
    let font_family = document_font_family_css(&core.document);
    let mut output = format!(
        "{{\"schemaVersion\":1,\"schemaMinorVersion\":0,\"schema\":{{\"major\":1,\"minor\":0}},\"resourceTableVersion\":1,\"resourceTableMinorVersion\":0,\"resourceTable\":{{\"major\":1,\"minor\":0}},\"unit\":\"px\",\"coordinateSystem\":\"page\",\"profile\":{},\"writingMode\":\"{}\",\"writingModeDecoded\":false,\"outputOptions\":{{\"showParagraphMarks\":{},\"showControlCodes\":{},\"showTransparentBorders\":{},\"clipEnabled\":{},\"debugOverlay\":false}},\"pageWidth\":{:.1},\"pageHeight\":{:.1},\"root\":{{\"kind\":\"leaf\",\"bounds\":{{\"x\":0.0,\"y\":0.0,\"width\":{:.1},\"height\":{:.1}}},\"ops\":[",
        json_string(profile),
        core.writing_mode.as_str(),
        core.show_paragraph_marks,
        core.show_control_codes,
        core.show_transparent_borders,
        core.clip_enabled,
        layout.width_px(),
        layout.height_px(),
        layout.width_px(),
        layout.height_px()
    );
    let mut text_sources = Vec::new();
    push_page_layer_page_background_json(&mut output, layout);
    let page_frame_projection =
        page_frame_projection(&core.document, layout, page_num as usize + 1);
    if let Some(projection) = &page_frame_projection {
        for shape in &projection.shapes {
            output.push(',');
            push_page_layer_page_frame_shape_json(&mut output, projection, shape);
        }
    }
    if let Some(separator) =
        page_mark_section_separator_projection(&core.document, layout, page_num as usize + 1)
    {
        output.push(',');
        push_page_layer_page_mark_separator_json(&mut output, &separator);
    }
    let shanai_lan_text_projection =
        shanai_lan_document_text_projection(&core.document, layout, page_num as usize + 1);
    let shanai_lan_line_rule_projection = if page_num == 0 {
        shanai_lan_document_text_line_rule_projection(&core.document, layout, page_num as usize + 1)
    } else {
        None
    };
    let shanai_lan_sparse_table_border_topology =
        (page_num == 0).then(|| shanai_lan_sparse_table_border_topology_diagnostic(&core.document));
    if page_num == 0 {
        if let Some(diagnostic) = shanai_lan_sparse_table_border_topology
            .as_ref()
            .and_then(|diagnostic| diagnostic.as_ref())
        {
            output.push(',');
            push_page_layer_shanai_lan_sparse_table_border_topology_diagnostic_json(
                &mut output,
                layout,
                diagnostic,
            );
        }
        if let Some(projection) = &shanai_lan_line_rule_projection {
            output.push(',');
            push_page_layer_shanai_lan_line_rule_projection_summary_json(
                &mut output,
                layout,
                projection,
                shanai_lan_text_projection.as_ref(),
            );
            for (rule_index, rule) in projection.rules.iter().enumerate() {
                output.push(',');
                push_page_layer_shanai_lan_line_rule_json(
                    &mut output,
                    projection,
                    rule_index,
                    rule,
                    shanai_lan_text_projection.as_ref(),
                );
            }
        }
        if document_has_shanai_lan_fdm_command_evidence(&core.document)
            && let Some(bytes) = document_text_raw_stream(&core.document)
        {
            output.push(',');
            push_page_layer_shanai_lan_line_header_projection_candidate_summary_json(
                &mut output,
                layout,
                &core.document,
                bytes,
                &shanai_lan_line_mark_intervals(&core.document),
                shanai_lan_line_rule_projection.as_ref(),
            );
        }
        for diagnostic in visual_list_diagnostics(&core.document) {
            output.push(',');
            push_page_layer_visual_list_diagnostic_json(&mut output, layout, diagnostic);
        }
        for diagnostic in embedding_frame_diagnostics(&core.document) {
            if embedding_frame_render_bbox(layout, lines, &core.document, diagnostic).is_some() {
                output.push(',');
                push_page_layer_embedding_frame_diagnostic_json(
                    &mut output,
                    &core.document,
                    layout,
                    lines,
                    diagnostic,
                );
                if success_data_test_title_art_diagnostic_for_page(
                    &core.document,
                    diagnostic,
                    page_num as usize + 1,
                ) {
                    output.push(',');
                    push_page_layer_success_data_test_title_art_projection_json(
                        &mut output,
                        &core.document,
                        layout,
                        lines,
                        diagnostic,
                    );
                }
            }
        }
        for diagnostic in fdm_frame_diagnostics(&core.document) {
            if fdm_frame_diagnostic_bbox(layout, diagnostic).is_some() {
                output.push(',');
                push_page_layer_fdm_frame_diagnostic_json(&mut output, layout, diagnostic);
            }
        }
        let command_diagnostics = fdm_command_diagnostics(&core.document);
        if let Some(extent) = fdm_command_projection_extent(&command_diagnostics) {
            let primitive_diagnostics = fdm_vector_primitive_diagnostics(&core.document);
            output.push(',');
            push_page_layer_fdm_projection_extent_summary_json(
                &mut output,
                layout,
                &command_diagnostics,
                &primitive_diagnostics,
                extent,
            );
            for diagnostic in command_diagnostics.iter().copied() {
                if fdm_command_diagnostic_bbox(layout, diagnostic, extent).is_some() {
                    output.push(',');
                    push_page_layer_fdm_command_diagnostic_json(
                        &mut output,
                        layout,
                        diagnostic,
                        extent,
                    );
                }
            }
            for diagnostic in primitive_diagnostics.iter().copied() {
                if fdm_path_diagnostic_bbox(layout, diagnostic, extent).is_some() {
                    output.push(',');
                    push_page_layer_fdm_vector_primitive_json(
                        &mut output,
                        layout,
                        diagnostic,
                        extent,
                        &primitive_diagnostics,
                    );
                }
            }
            for diagnostic in primitive_diagnostics.iter().copied() {
                if fdm_path_span_filter_blocked(layout, diagnostic, extent).is_some() {
                    output.push(',');
                    push_page_layer_fdm_vector_primitive_large_span_blocked_json(
                        &mut output,
                        layout,
                        diagnostic,
                        extent,
                        &primitive_diagnostics,
                    );
                }
            }
            if let Some(text_projection) = shanai_lan_text_projection.as_ref() {
                let text_mask_cohorts =
                    fdm_text_mask_cohort_summaries(layout, &primitive_diagnostics, extent);
                if !text_mask_cohorts.is_empty() {
                    output.push(',');
                    push_page_layer_fdm_text_mask_cohort_summary_json(
                        &mut output,
                        layout,
                        &text_mask_cohorts,
                        text_projection,
                    );
                    output.push(',');
                    push_page_layer_fdm_text_mask_source_transform_candidate_summary_json(
                        &mut output,
                        layout,
                        &text_mask_cohorts,
                        text_projection,
                    );
                }
            }
            let fdm_open_stroke_axis_rules = shanai_lan_line_rule_projection
                .as_ref()
                .map(|projection| {
                    fdm_open_stroke_axis_rules(layout, &primitive_diagnostics, extent, projection)
                })
                .unwrap_or_default();
            for diagnostic in primitive_diagnostics.iter().copied() {
                if let Some(metric) = fdm_connector_candidate_metric(layout, diagnostic, extent) {
                    output.push(',');
                    push_page_layer_fdm_connector_candidate_json(
                        &mut output,
                        layout,
                        diagnostic,
                        extent,
                        metric,
                        &primitive_diagnostics,
                        shanai_lan_text_projection.as_ref(),
                        shanai_lan_line_rule_projection.as_ref(),
                        &fdm_open_stroke_axis_rules,
                    );
                }
            }
            if let Some(trace) = fdm_connector_order_trace_json(
                layout,
                &primitive_diagnostics,
                extent,
                shanai_lan_text_projection.as_ref(),
                shanai_lan_line_rule_projection.as_ref(),
                &fdm_open_stroke_axis_rules,
            ) {
                output.push(',');
                output.push_str(&trace);
            }
            if let Some(summary) = fdm_connector_graph_diagnostic_summary(
                layout,
                &primitive_diagnostics,
                extent,
                shanai_lan_text_projection.as_ref(),
                shanai_lan_line_rule_projection.as_ref(),
            ) {
                output.push(',');
                push_page_layer_fdm_connector_graph_diagnostic_summary_json(
                    &mut output,
                    layout,
                    summary,
                );
            }
            if let Some(summary) =
                fdm_open_stroke_cohort_summary(layout, &primitive_diagnostics, extent)
            {
                output.push(',');
                push_page_layer_fdm_open_stroke_cohort_summary_json(&mut output, layout, &summary);
            }
        }
    }
    if page_num != 0 {
        for diagnostic in embedding_frame_diagnostics(&core.document) {
            if success_data_test_title_art_diagnostic_for_page(
                &core.document,
                diagnostic,
                page_num as usize + 1,
            ) {
                output.push(',');
                push_page_layer_success_data_test_title_art_projection_json(
                    &mut output,
                    &core.document,
                    layout,
                    lines,
                    diagnostic,
                );
            }
        }
        if success_data_test_answer_sheet_page(&core.document, page_num as usize + 1) {
            output.push(',');
            push_page_layer_success_data_test_answer_sheet_projection_json(
                &mut output,
                layout,
                &core.document,
            );
        }
    }
    for op in success_data_test_fdm_reference_projection_layer_ops(
        &core.document,
        layout,
        page_num as usize + 1,
    ) {
        output.push(',');
        output.push_str(&op);
    }
    let form_projection =
        observed_form_text_projection(&core.document, layout, page_num as usize + 1);
    if let Some(projection) = &form_projection {
        for shape in &projection.shapes {
            output.push(',');
            push_page_layer_observed_form_shape_json(&mut output, projection, shape);
        }
        for slot in &projection.slots {
            output.push(',');
            push_page_layer_observed_form_text_slot_json(&mut output, layout, projection, slot);
        }
    }
    let mut first_op = false;
    let vertical_placement = vertical_page_text_placement(layout, lines);
    let layout_box_text_projection =
        layout_box_text_projection(&core.document, layout, page_num as usize + 1);

    if let Some(projection) = &shanai_lan_text_projection {
        output.push(',');
        push_page_layer_shanai_lan_text_style_evidence_summary_json(
            &mut output,
            layout,
            projection,
        );
        for slot in &projection.slots {
            let source_id = text_sources.len();
            output.push(',');
            push_page_layer_shanai_lan_text_slot_json(
                &mut output,
                source_id,
                projection,
                slot,
                &font_family,
            );
            let fragment = PageLayerTextFragment {
                text: slot.text.clone(),
                paragraph_index: None,
                char_start: 0,
                char_end: slot.text.chars().count(),
                source_span: Some(slot.source_span.clone()),
                ruby_annotation: None,
            };
            push_page_layer_text_source_json(&mut text_sources, source_id, &fragment);
        }
    }
    if let Some(slots) =
        success_data_test_resolved_top_text_projection(&core.document, page_num as usize + 1)
    {
        let figure_label_line =
            success_data_test_top_text_projection(&core.document, page_num as usize + 1).and_then(
                |static_slots| {
                    success_data_test_q4_figure_label_source_line(&core.document, static_slots)
                },
            );
        for slot in &slots {
            if figure_label_line.is_some() && slot.role == "figure-label" {
                continue;
            }
            if success_data_test_source_text_placement_candidate(
                &core.document,
                layout,
                slot.source_span.as_ref(),
                SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX,
            )
            .is_none()
            {
                continue;
            }
            let source_id = text_sources.len();
            output.push(',');
            push_page_layer_success_data_test_text_slot_json(
                &mut output,
                &core.document,
                layout,
                source_id,
                slot,
                &font_family,
            );
            let fragment = success_data_test_resolved_text_slot_fragment(slot);
            push_page_layer_text_source_json(&mut text_sources, source_id, &fragment);
        }
        if let Some(line) = &figure_label_line {
            for span in &line.spans {
                if success_data_test_source_text_placement_candidate(
                    &core.document,
                    layout,
                    Some(&span.source_span),
                    line.font_size,
                )
                .is_none()
                {
                    continue;
                }
                let source_id = text_sources.len();
                output.push(',');
                push_page_layer_success_data_test_figure_label_span_json(
                    &mut output,
                    &core.document,
                    layout,
                    source_id,
                    line,
                    span,
                    &font_family,
                );
                let fragment = success_data_test_figure_label_span_fragment(span);
                push_page_layer_text_source_json(&mut text_sources, source_id, &fragment);
            }
        }
        if let Some(summary) = success_data_test_text_placement_residual_summary_json(
            &core.document,
            layout,
            page_num as usize + 1,
        ) {
            output.push(',');
            output.push_str(&summary);
        }
    }
    if let Some(projection) = &layout_box_text_projection {
        for slot in &projection.slots {
            let source_id = text_sources.len();
            output.push(',');
            push_page_layer_layout_box_text_slot_json(
                &mut output,
                source_id,
                projection,
                slot,
                &font_family,
            );
            let fragment = PageLayerTextFragment {
                text: slot.text.clone(),
                paragraph_index: None,
                char_start: 0,
                char_end: slot.text.chars().count(),
                source_span: Some(slot.source_span.clone()),
                ruby_annotation: None,
            };
            push_page_layer_text_source_json(&mut text_sources, source_id, &fragment);
        }
    }
    if shanai_lan_text_projection.is_none() && form_projection.is_none() {
        let mut fallback_visual_line_index = 0usize;
        for (line_index, line) in lines.iter().enumerate() {
            if line.text().is_empty() {
                continue;
            }
            if success_data_test_top_text_line_should_skip(
                &core.document,
                page_num as usize + 1,
                line,
            ) {
                continue;
            }

            let frame_text_placement = if core.writing_mode.is_vertical() {
                None
            } else {
                page_frame_text_placement(
                    &core.document,
                    layout,
                    page_num as usize + 1,
                    fallback_visual_line_index,
                    line,
                )
            };
            let mut x = if core.writing_mode.is_vertical() {
                layout.width_px() as f64
                    - layout.margin_px() as f64
                    - ((line_index + 1) as f64 * APP_LINE_HEIGHT_PX as f64)
                    + vertical_placement.x_shift_px as f64
            } else {
                frame_text_placement
                    .map(|placement| placement.x)
                    .or_else(|| {
                        fallback_text_origin(layout, &core.document).map(|origin| origin.0 as f64)
                    })
                    .unwrap_or(layout.margin_px() as f64)
            };
            let mut y = if core.writing_mode.is_vertical() {
                vertical_placement.y_start_px as f64
            } else {
                frame_text_placement
                    .map(|placement| placement.y)
                    .or_else(|| {
                        fallback_text_origin(layout, &core.document).map(|origin| {
                            origin.1 as f64 + line_index as f64 * APP_LINE_HEIGHT_PX as f64
                        })
                    })
                    .unwrap_or(
                        layout.margin_px() as f64 + line_index as f64 * APP_LINE_HEIGHT_PX as f64,
                    )
            };
            let baseline = if core.writing_mode.is_vertical() {
                x + APP_FONT_SIZE_PX as f64
            } else {
                frame_text_placement
                    .map(|placement| placement.baseline)
                    .unwrap_or(y + APP_FONT_SIZE_PX as f64)
            };

            for fragment in page_text_line_fragments(&core.document, line) {
                if fragment.text.is_empty() {
                    continue;
                }

                let source_id = text_sources.len();
                if !first_op {
                    output.push(',');
                }
                first_op = false;
                let fill_color = fallback_text_fill_color();
                push_page_layer_text_run_json(
                    &mut output,
                    source_id,
                    PageLayerTextPlacement { x, y, baseline },
                    layout,
                    core.writing_mode,
                    &font_family,
                    fill_color,
                    &fragment,
                );
                push_page_layer_text_source_json(&mut text_sources, source_id, &fragment);
                if core.writing_mode.is_vertical() {
                    y += vertical_text_advance_px(&fragment.text);
                } else {
                    x += text_width_px(layout, &fragment.text);
                }
            }
            if !core.writing_mode.is_vertical() {
                fallback_visual_line_index += 1;
            }
        }
    }

    if let Some(decoration) = core.page_decoration(page_num as usize) {
        output.push(',');
        push_page_layer_decoration_json(&mut output, layout, &decoration);
    }

    if page_num == 0 {
        let mut overlay_index = 0usize;
        for candidate in core.document.table_candidates() {
            let Some(grid) = candidate.column_segment_grid_candidate() else {
                continue;
            };
            output.push(',');
            push_page_layer_table_grid_candidate_json(
                &mut output,
                layout,
                &core.document,
                lines,
                overlay_index,
                candidate,
                &grid,
            );
            overlay_index += 1;
        }

        for (overlay_index, diagnostic) in image_payload_diagnostics(&core.document)
            .into_iter()
            .take(APP_IMAGE_DIAGNOSTIC_MAX_OVERLAYS)
            .enumerate()
        {
            output.push(',');
            push_page_layer_image_payload_diagnostic_json(
                &mut output,
                layout,
                overlay_index,
                diagnostic,
            );
        }
    }

    output.push_str("]},\"textSources\":[");
    for (index, source) in text_sources.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(source);
    }
    output.push_str("],\"fontResources\":{\"blobs\":[],\"faces\":[]},\"usedFeatures\":[\"text.sourceTable\",\"text.sourceSpan\",\"text.v2.diagnostics\"],\"optionalFeatures\":[],\"knownFeatures\":[\"fontResources\",\"text.sourceTable\",\"text.sourceSpan\",\"text.v2.diagnostics\"],\"requiredFeatures\":[],\"text\":{\"defaultVariant\":\"textRun\",\"variants\":[\"textRun\"],\"variantSelection\":\"exclusiveVariantSet\",\"sourceTextPreserved\":true,\"clusterEncoding\":[\"utf8\",\"utf16\"],\"fallbackRequired\":true,\"placementAuthority\":\"compatibilityProjection\",\"externalizedVisuals\":[]},\"textV2\":{\"diagnostics\":[],\"validationIssues\":[],\"slotDiagnostics\":[]}}");
    output
}

pub(super) fn push_page_layer_page_background_json(output: &mut String, layout: PageLayout) {
    output.push_str("{\"type\":\"pageBackground\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"backgroundColor\":\"#ffffff\"}");
}

pub(super) fn push_page_layer_page_frame_shape_json(
    output: &mut String,
    projection: &PageFrameProjection,
    shape: &PageFrameShape,
) {
    output.push_str("{\"type\":\"pageFrameShape\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        shape.x, shape.y, shape.width, shape.height
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(projection.projection_kind));
    output.push_str(",\"role\":");
    output.push_str(&json_string(shape.role));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":true,\"styleDecoded\":false,\"placementProven\":true,\"pageAssignmentDecoded\":");
    output.push_str(if projection.page_assignment_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"recordCount\":");
    output.push_str(&projection.record_count.to_string());
    output.push_str(",\"rowIndex\":");
    output.push_str(&shape.row_index.to_string());
    output.push_str(",\"objectId\":");
    output.push_str(&shape.object_id.to_string());
    output.push_str(",\"objectType\":");
    output.push_str(&shape.object_type.to_string());
    output.push_str(",\"objectTypeHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", shape.object_type)));
    output.push_str(",\"sourceGeometry\":{\"x\":");
    output.push_str(&shape.source_x.to_string());
    output.push_str(",\"y\":");
    output.push_str(&shape.source_y.to_string());
    output.push_str(",\"width\":");
    output.push_str(&shape.source_width.to_string());
    output.push_str(",\"height\":");
    output.push_str(&shape.source_height.to_string());
    output.push_str(",\"cornerRadius\":");
    output.push_str(&shape.source_corner_radius.to_string());
    output.push_str("},\"sourceStyleId\":");
    output.push_str(&shape.source_style_id.to_string());
    output.push_str(",\"cornerRadius\":");
    output.push_str(&format!("{:.3}", shape.corner_radius));
    output.push_str(",\"placementBasis\":");
    output.push_str(&json_string(shape.placement_basis));
    output.push_str(",\"styleBasis\":");
    output.push_str(&json_string(shape.style_basis));
    output.push('}');
}

pub(super) fn push_page_layer_decoration_json(
    output: &mut String,
    layout: PageLayout,
    decoration: &PageDecoration,
) {
    let x = page_decoration_x(layout, decoration.side);
    let y = layout.margin_px() * 0.55;
    output.push_str("{\"type\":\"pageDecoration\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{:.3},\"height\":{:.3}}}",
        APP_LINE_HEIGHT_PX,
        layout.body_height_px()
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(decoration.source));
    output.push_str(",\"projectionKind\":\"layoutStyleAutoTextProjection\",\"decoded\":false");
    output.push_str(",\"sidePolicy\":");
    output.push_str(&json_string(decoration.side_policy));
    output.push_str(",\"sidePolicyDecoded\":");
    output.push_str(if decoration.side_policy_decoded {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"facingPagesCandidate\":");
    output.push_str(if decoration.facing_pages_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"pairedSlotPairs\":");
    push_page_decoration_slot_pairs_json(output, &decoration.paired_slot_pairs);
    output.push_str(",\"slotEvidence\":");
    push_page_decoration_slot_evidence_json(output, &decoration.slot_evidence);
    output.push_str(",\"layoutMarkEvidence\":");
    push_page_decoration_mark_evidence_json(output, layout, decoration.mark_evidence.as_ref());
    output.push_str(",\"side\":");
    output.push_str(&json_string(decoration.side.as_str()));
    output.push_str(",\"pageNumber\":");
    output.push_str(&decoration.page_number.to_string());
    output.push_str(",\"headerText\":");
    output.push_str(&json_string(&decoration.header_text));
    output.push('}');
}

pub(super) fn push_page_decoration_mark_evidence_json(
    output: &mut String,
    layout: PageLayout,
    evidence: Option<&PageDecorationMarkEvidence>,
) {
    let Some(evidence) = evidence else {
        output.push_str("null");
        return;
    };

    output.push_str("{\"source\":\"/PageMark+/PaperMark\"");
    output.push_str(",\"present\":true,\"sourceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"pageIndex\":");
    output.push_str(&evidence.page_index.to_string());
    output.push_str(",\"pageNumber\":");
    output.push_str(&evidence.page_index.saturating_add(1).to_string());
    output.push_str(",\"pageMarkEntryIndex\":");
    push_optional_usize_json(output, evidence.page_mark_entry_index);
    output.push_str(",\"pageMarkIndex\":");
    push_option_u32_json(output, evidence.page_mark_index);
    output.push_str(",\"pageMarkFlags\":");
    push_option_u32_json(output, evidence.page_mark_flags);
    output.push_str(",\"pageMarkFlagsHex\":");
    push_option_u32_hex_or_null_json(output, evidence.page_mark_flags);
    output.push_str(",\"pageMarkLineStart\":");
    push_option_u32_json(output, evidence.page_mark_line_start);
    output.push_str(",\"pageMarkLineEnd\":");
    push_option_u32_json(output, evidence.page_mark_line_end);
    output.push_str(",\"pageMarkU16Fields\":");
    push_u16_array_json(output, &evidence.page_mark_u16_fields);
    output.push_str(",\"pageMarkU16FieldsHex\":");
    push_u16_hex_array_json(output, &evidence.page_mark_u16_fields);
    output.push_str(",\"pageMarkU16GeometryHypotheses\":");
    push_page_mark_u16_geometry_hypotheses_json(
        output,
        &evidence.page_mark_u16_fields,
        Some(PageMarkU16LayoutComparison {
            page_width_px: layout.width_px(),
            page_height_px: layout.height_px(),
            page_margin_px: layout.margin_px(),
            page_body_width_px: layout.body_width_px(),
        }),
    );
    output.push_str(",\"pagePitchEvidence\":");
    push_page_decoration_mark_pitch_evidence_json(output, layout, evidence);
    output.push_str(",\"paperMarkEntryIndex\":");
    push_optional_usize_json(output, evidence.paper_mark_entry_index);
    output.push_str(",\"paperMarkIndex\":");
    push_option_u32_json(output, evidence.paper_mark_index);
    output.push_str(",\"paperMarkFlags\":");
    push_option_u32_json(output, evidence.paper_mark_flags);
    output.push_str(",\"paperMarkFlagsHex\":");
    push_option_u32_hex_or_null_json(output, evidence.paper_mark_flags);
    output.push_str(",\"rowIndexAligned\":");
    output.push_str(if evidence.row_index_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"markIndexAligned\":");
    output.push_str(if evidence.mark_index_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"entryCountAligned\":");
    output.push_str(if evidence.entry_count_aligned {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionContribution\":\"page-row-association-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("paper-mark-flag-semantics-undecoded"));
    output.push('}');
}

pub(super) fn push_page_decoration_mark_pitch_evidence_json(
    output: &mut String,
    layout: PageLayout,
    evidence: &PageDecorationMarkEvidence,
) {
    let line_count = evidence
        .page_mark_line_start
        .zip(evidence.page_mark_line_end)
        .map(|(start, end)| end.saturating_sub(start).saturating_add(1));
    let line_gap_count = evidence
        .page_mark_line_start
        .zip(evidence.page_mark_line_end)
        .map(|(start, end)| end.saturating_sub(start));

    output.push_str("{\"source\":\"/PageMark+PageLayout\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"pageMarkEntryIndex\":");
    push_optional_usize_json(output, evidence.page_mark_entry_index);
    output.push_str(",\"pageMarkIndex\":");
    push_option_u32_json(output, evidence.page_mark_index);
    output.push_str(",\"lineStart\":");
    push_option_u32_json(output, evidence.page_mark_line_start);
    output.push_str(",\"lineEnd\":");
    push_option_u32_json(output, evidence.page_mark_line_end);
    output.push_str(",\"lineCount\":");
    push_option_u32_json(output, line_count);
    output.push_str(",\"lineGapCount\":");
    push_option_u32_json(output, line_gap_count);
    output.push_str(",\"pageSizePx\":{\"width\":");
    output.push_str(&format!("{:.3}", layout.width_px()));
    output.push_str(",\"height\":");
    output.push_str(&format!("{:.3}", layout.height_px()));
    output.push_str("},\"bodySizePx\":{\"width\":");
    output.push_str(&format!("{:.3}", layout.body_width_px()));
    output.push_str(",\"height\":");
    output.push_str(&format!("{:.3}", layout.body_height_px()));
    output.push_str("},\"marginPx\":");
    output.push_str(&format!("{:.3}", layout.margin_px()));
    output.push_str(",\"pageHeightPxPerLineCount\":");
    push_optional_f32_json(
        output,
        line_count
            .filter(|count| *count > 0)
            .map(|count| layout.height_px() / count as f32),
    );
    output.push_str(",\"pageHeightPxPerLineGap\":");
    push_optional_f32_json(
        output,
        line_gap_count
            .filter(|count| *count > 0)
            .map(|count| layout.height_px() / count as f32),
    );
    output.push_str(",\"bodyHeightPxPerLineCount\":");
    push_optional_f32_json(
        output,
        line_count
            .filter(|count| *count > 0)
            .map(|count| layout.body_height_px() / count as f32),
    );
    output.push_str(",\"bodyHeightPxPerLineGap\":");
    push_optional_f32_json(
        output,
        line_gap_count
            .filter(|count| *count > 0)
            .map(|count| layout.body_height_px() / count as f32),
    );
    output.push_str(",\"pageMarkSelectedFields\":");
    push_page_mark_selected_fields_from_parts_json(
        output,
        evidence.page_mark_entry_index,
        evidence.page_mark_line_start,
        evidence.page_mark_line_end,
        &evidence.page_mark_u16_fields,
    );
    output.push_str(",\"linePitchAgreementGate\":");
    push_page_mark_line_pitch_agreement_gate_json(
        output,
        layout,
        evidence.page_mark_line_start,
        evidence.page_mark_line_end,
        None,
        None,
        &evidence.page_mark_u16_fields,
    );
    output.push_str(",\"renderPromotionContribution\":\"page-mark-line-gap-pitch-diagnostic-only\",\"renderPromotionBlockedReason\":\"page-mark-line-pitch-semantics-unproven\"}");
}

pub(super) fn push_page_decoration_slot_pairs_json(output: &mut String, pairs: &[(u16, u16)]) {
    output.push('[');
    for (index, (left, right)) in pairs.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(&format!("0x{left:02x}/0x{right:02x}")));
    }
    output.push(']');
}

pub(super) fn push_page_decoration_slot_evidence_json(
    output: &mut String,
    evidence: &[PageDecorationSlotEvidence],
) {
    output.push('[');
    for (index, item) in evidence.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"recordIndex\":");
        output.push_str(&item.record_index.to_string());
        output.push_str(",\"recordOffset\":");
        output.push_str(&item.record_offset.to_string());
        output.push_str(",\"recordLabel\":");
        match &item.record_label {
            Some(label) => output.push_str(&json_string(label)),
            None => output.push_str("null"),
        }
        output.push_str(",\"slot\":");
        output.push_str(&json_string(&format!("0x{:02x}", item.slot)));
        output.push_str(",\"part05First\":");
        push_optional_hex_byte_json(output, item.part05.as_deref().and_then(|part| part.first()));
        output.push_str(",\"part05NonZero\":");
        output.push_str(
            if item
                .part05
                .as_deref()
                .and_then(|part| part.first())
                .is_some_and(|byte| *byte != 0)
            {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"part04Hex\":");
        push_optional_hex_bytes_json(output, item.part04.as_deref());
        output.push_str(",\"part05Hex\":");
        push_optional_hex_bytes_json(output, item.part05.as_deref());
        output.push_str(",\"part06Hex\":");
        push_optional_hex_bytes_json(output, item.part06.as_deref());
        output.push_str(",\"part07Hex\":");
        push_optional_hex_bytes_json(output, item.part07.as_deref());
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

pub(super) fn source_range_gap_minus_page_line_gap_units(
    source_range_gap_units: usize,
    page_line_gap_units: i32,
) -> i32 {
    let Ok(source_range_gap_units) = i32::try_from(source_range_gap_units) else {
        return i32::MAX;
    };
    source_range_gap_units.saturating_sub(page_line_gap_units)
}

pub(super) fn projected_bbox_page_coverage_ratio(
    layout: PageLayout,
    width: f32,
    height: f32,
) -> f32 {
    let page_area = layout.width_px() * layout.height_px();
    if page_area <= 0.0 {
        return 0.0;
    }
    ((width.max(0.0) * height.max(0.0)) / page_area).clamp(0.0, 1.0)
}

pub(super) fn push_page_layer_observed_form_shape_json(
    output: &mut String,
    projection: &ObservedFormTextProjection,
    shape: &ObservedFormShape,
) {
    output.push_str("{\"type\":\"formShapeProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
        shape.x, shape.y, shape.width, shape.height
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(projection.projection_kind));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":false,\"placementProven\":true");
    output.push_str(",\"role\":");
    output.push_str(&json_string(shape.role));
    output.push_str(",\"fill\":");
    output.push_str(&json_string(shape.fill));
    output.push_str(",\"stroke\":");
    match shape.stroke {
        Some(stroke) => output.push_str(&json_string(stroke)),
        None => output.push_str("null"),
    }
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!("{:.3}", shape.stroke_width));
    output.push_str(",\"rx\":");
    output.push_str(&format!("{:.3}", shape.rx));
    output.push('}');
}

pub(super) fn push_page_layer_source_span_json(
    output: &mut String,
    source_id: usize,
    fragment: &PageLayerTextFragment,
) {
    output.push_str(&format!(
        "{{\"id\":{},\"utf8Range\":{},\"utf16Range\":{}",
        source_id,
        source_range_json(0, fragment.text.len()),
        source_range_json(0, fragment.text.encode_utf16().count())
    ));
    if let Some(paragraph_index) = fragment.paragraph_index {
        output.push_str(",\"stableSourceKey\":");
        output.push_str(&json_string(&format!(
            "section:0/para:{paragraph_index}/char:{}",
            fragment.char_start
        )));
    }
    if let Some(span) = &fragment.source_span {
        output.push_str(",\"jtdByteRange\":");
        output.push_str(&source_range_json(span.byte_start(), span.byte_end()));
        output.push_str(",\"jtdUnitRange\":");
        output.push_str(&source_range_json(span.unit_start(), span.unit_end()));
    }
    output.push('}');
}

pub(super) fn page_frame_projection(
    document: &Document,
    layout: PageLayout,
    page_number: usize,
) -> Option<PageFrameProjection> {
    if page_number != 1 {
        return None;
    }

    let mut shapes = Vec::new();
    for record in document.object_frame_records() {
        if let Some(shape) = page_frame_shape(record, layout) {
            shapes.push(shape);
        }
    }

    if shapes.is_empty() {
        return None;
    }

    Some(PageFrameProjection {
        source: "/Frame",
        projection_kind: "pageFrameProjection",
        page_assignment_decoded: false,
        record_count: document.object_frame_records().len(),
        shapes,
    })
}

pub(super) fn page_frame_shape(
    record: &ObjectFrameRecordCandidate,
    layout: PageLayout,
) -> Option<PageFrameShape> {
    match record.object_type() {
        PAGE_FRAME_TITLE_OBJECT_TYPE => page_frame_title_shape(record, layout),
        PAGE_FRAME_PATTERN_BAR_OBJECT_TYPE => page_frame_pattern_bar_shape(record, layout, false),
        PAGE_FRAME_PATTERN_BAR_BOTTOM_OBJECT_TYPE => {
            page_frame_pattern_bar_shape(record, layout, true)
        }
        _ => None,
    }
}

pub(super) fn page_frame_title_shape(
    record: &ObjectFrameRecordCandidate,
    layout: PageLayout,
) -> Option<PageFrameShape> {
    if record.width() == 0 || record.height() == 0 {
        return None;
    }

    let y = frame_record_unit_to_css_px(record.y());
    let width = frame_record_unit_to_css_px(record.width());
    let height = frame_record_unit_to_css_px(record.height());
    if !(48.0..=layout.width_px() * 0.75).contains(&width)
        || !(8.0..=layout.height_px() * 0.12).contains(&height)
    {
        return None;
    }
    if width / height.max(1.0) < PAGE_FRAME_MIN_TITLE_ASPECT_RATIO {
        return None;
    }

    let x = if record.x() == 0 {
        (layout.width_px() - width) / 2.0
    } else {
        frame_record_unit_to_css_px(record.x())
    };
    let corner_radius = frame_record_unit_to_css_px(record.corner_radius());

    Some(PageFrameShape {
        role: "titleRoundedFrame",
        row_index: record.row_index(),
        object_id: record.object_id(),
        object_type: record.object_type(),
        x,
        y,
        width,
        height,
        corner_radius,
        source_x: record.x(),
        source_y: record.y(),
        source_width: record.width(),
        source_height: record.height(),
        source_corner_radius: record.corner_radius(),
        source_style_id: record.style_id(),
        placement_basis: if record.x() == 0 {
            "centeredFrameRecordWidth"
        } else {
            "frameRecordFields"
        },
        style_basis: "frameRecordObjectTypeAndStyleId",
    })
}

pub(super) fn page_frame_pattern_bar_shape(
    record: &ObjectFrameRecordCandidate,
    layout: PageLayout,
    bottom_origin: bool,
) -> Option<PageFrameShape> {
    if record.width() == 0 || record.height() == 0 {
        return None;
    }

    let x = frame_record_unit_to_css_px(record.x());
    let width = frame_record_unit_to_css_px(record.width());
    let height = frame_record_unit_to_css_px(record.height());
    let source_y = frame_record_unit_to_css_px(record.y());
    let y = if bottom_origin {
        layout.height_px() - source_y - height
    } else {
        source_y
    };
    if width < layout.width_px() * PAGE_FRAME_MIN_PATTERN_BAR_WIDTH_RATIO
        || y < PAGE_FRAME_MIN_PATTERN_BAR_Y_PX
        || height > PAGE_FRAME_MAX_PATTERN_BAR_HEIGHT_PX
    {
        return None;
    }

    Some(PageFrameShape {
        role: "horizontalPatternBar",
        row_index: record.row_index(),
        object_id: record.object_id(),
        object_type: record.object_type(),
        x,
        y,
        width,
        height,
        corner_radius: height * 0.45,
        source_x: record.x(),
        source_y: record.y(),
        source_width: record.width(),
        source_height: record.height(),
        source_corner_radius: record.corner_radius(),
        source_style_id: record.style_id(),
        placement_basis: if bottom_origin {
            "frameRecordBottomOriginFields"
        } else {
            "frameRecordFields"
        },
        style_basis: "frameRecordObjectTypeAndStyleId",
    })
}

pub(super) fn layout_box_body_anchor_from_document(
    document: &Document,
    layout: PageLayout,
) -> Option<(f32, f32, f32)> {
    let text_bytes = raw_stream_bytes(document, LAYOUT_BOX_TEXT_PATH)?;
    let layout_bytes = raw_stream_bytes(document, LAYOUT_BOX_PATH)?;
    let blocks = layout_box_text_blocks(text_bytes);
    let records = layout_box_record_candidates(layout_bytes);
    layout_box_body_anchor(&blocks, &records, layout)
}

pub(super) fn layout_box_record_candidates(bytes: &[u8]) -> Vec<LayoutBoxRecordCandidate> {
    let starts = find_layout_box_record_starts(bytes);
    starts
        .iter()
        .enumerate()
        .map(|(index, byte_start)| {
            let byte_end = starts.get(index + 1).copied().unwrap_or(bytes.len());
            let record = &bytes[*byte_start..byte_end];
            LayoutBoxRecordCandidate {
                index,
                byte_start: *byte_start,
                byte_end,
                origin_field: read_be16_at(record, LAYOUT_BOX_RECORD_ORIGIN_FIELD_OFFSET),
                x_field: read_be16_at(record, LAYOUT_BOX_RECORD_X_FIELD_OFFSET),
                y_field: read_be16_at(record, LAYOUT_BOX_RECORD_Y_FIELD_OFFSET),
                width_field: read_be16_at(record, LAYOUT_BOX_RECORD_WIDTH_FIELD_OFFSET),
            }
        })
        .collect()
}

pub(super) fn find_layout_box_record_starts(bytes: &[u8]) -> Vec<usize> {
    bytes
        .windows(LAYOUT_BOX_RECORD_PREFIX.len())
        .enumerate()
        .filter_map(|(offset, window)| (window == LAYOUT_BOX_RECORD_PREFIX).then_some(offset))
        .collect()
}

pub(super) fn layout_box_body_anchor(
    blocks: &[LayoutBoxTextBlock],
    records: &[LayoutBoxRecordCandidate],
    layout: PageLayout,
) -> Option<(f32, f32, f32)> {
    blocks.iter().find_map(|block| {
        block
            .fragments
            .iter()
            .any(|fragment| layout_box_text_role(block, &fragment.text) == "body")
            .then(|| layout_box_record_text_box(block.index, records, layout))
            .flatten()
            .map(|(_, x, y, width, _)| (x, y, width))
    })
}

pub(super) fn layout_box_record_origin_pt(record: &LayoutBoxRecordCandidate) -> Option<f32> {
    record
        .origin_field
        .map(|value| f32::from(value) * (2.0 / 3.0))
}

pub(super) fn push_page_frame_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    document: &Document,
    page_number: usize,
) {
    let Some(projection) = page_frame_projection(document, layout, page_number) else {
        return;
    };

    let pattern_id = format!("rjtd-page-frame-pattern-{page_number}");
    svg.push_str(&format!(
        "<defs><pattern id=\"{}\" width=\"5\" height=\"5\" patternUnits=\"userSpaceOnUse\"><rect width=\"5\" height=\"5\" fill=\"#fbfbfb\"/><circle cx=\"1.4\" cy=\"1.4\" r=\"{:.2}\" fill=\"#d1d1d1\"/><circle cx=\"3.9\" cy=\"3.9\" r=\"{:.2}\" fill=\"#d1d1d1\"/></pattern></defs>",
        escape_xml(&pattern_id),
        PAGE_FRAME_PATTERN_DOT_RADIUS_PX,
        PAGE_FRAME_PATTERN_DOT_RADIUS_PX
    ));
    svg.push_str(&format!(
        "<g class=\"rjtd-page-frame-projection\" data-source=\"{}\" data-projection-kind=\"{}\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-style-decoded=\"false\" data-placement-proven=\"true\" data-page-assignment-decoded=\"{}\" data-record-count=\"{}\">",
        escape_xml(projection.source),
        escape_xml(projection.projection_kind),
        projection.page_assignment_decoded,
        projection.record_count
    ));
    for shape in &projection.shapes {
        match shape.role {
            "titleRoundedFrame" => {
                push_title_rounded_frame_svg(svg, shape);
            }
            "horizontalPatternBar" => {
                push_horizontal_pattern_bar_svg(svg, shape, &pattern_id);
            }
            _ => {}
        }
    }
    svg.push_str("</g>");
}

pub(super) fn push_page_decoration_svg(
    svg: &mut String,
    layout: PageLayout,
    _writing_mode: WritingMode,
    decoration: &PageDecoration,
    font_family: &str,
) {
    let x = page_decoration_x(layout, decoration.side);
    let header_y = layout.margin_px() * 0.55;
    let page_number_y = layout.height_px() - (layout.margin_px() * 0.45);
    let text_anchor = decoration.side.text_anchor();
    let font_family = escape_xml(font_family);
    svg.push_str(&format!(
        "<text class=\"rjtd-running-header\" data-source=\"{}\" data-projection-kind=\"layoutStyleAutoTextProjection\" data-decoded=\"false\" data-side=\"{}\" data-side-policy=\"{}\" data-side-policy-decoded=\"{}\" data-facing-pages-candidate=\"{}\" x=\"{x:.1}\" y=\"{header_y:.1}\" text-anchor=\"{text_anchor}\" font-family=\"{font_family}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
        escape_xml(decoration.source),
        decoration.side.as_str(),
        escape_xml(decoration.side_policy),
        decoration.side_policy_decoded,
        decoration.facing_pages_candidate,
        APP_PAGE_DECORATION_FONT_SIZE_PX,
        escape_xml(&svg_visual_text(&decoration.header_text))
    ));

    svg.push_str(&format!(
        "<text class=\"rjtd-page-number\" data-source=\"{}\" data-projection-kind=\"layoutStyleAutoTextProjection\" data-decoded=\"false\" data-side=\"{}\" data-side-policy=\"{}\" data-side-policy-decoded=\"{}\" data-facing-pages-candidate=\"{}\" x=\"{x:.1}\" y=\"{page_number_y:.1}\" text-anchor=\"{text_anchor}\" font-family=\"{font_family}\" font-size=\"{:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
        escape_xml(decoration.source),
        decoration.side.as_str(),
        escape_xml(decoration.side_policy),
        decoration.side_policy_decoded,
        decoration.facing_pages_candidate,
        APP_PAGE_DECORATION_FONT_SIZE_PX,
        decoration.page_number
    ));
}

pub(super) fn page_decoration_x(layout: PageLayout, side: PageDecorationSide) -> f32 {
    match side {
        PageDecorationSide::Left => layout.margin_px(),
        PageDecorationSide::Right => layout.width_px() - layout.margin_px(),
    }
}
