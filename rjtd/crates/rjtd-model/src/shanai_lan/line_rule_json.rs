use super::*;
use crate::*;

pub(crate) fn push_page_layer_shanai_lan_line_rule_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    rule_index: usize,
    rule: &ShanaiLanLineRule,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    let topology = shanai_lan_line_rule_topology(projection, rule);
    let (x, y, width, height) = shanai_lan_line_rule_bbox(projection, rule);
    let component = shanai_lan_line_rule_component_for_rule(projection, rule_index);
    output.push_str("{\"type\":\"documentTextLineRuleProjection\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":{x:.3},\"y\":{y:.3},\"width\":{width:.3},\"height\":{height:.3}}}"
    ));
    output.push_str(",\"source\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":");
    output.push_str(&json_string(projection.projection_kind));
    output.push_str(",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"diagnosticOnly\":true,\"referenceBacked\":true");
    output.push_str(",\"ruleIndex\":");
    output.push_str(&rule_index.to_string());
    output.push_str(",\"projectionBasis\":\"documentTextLineHeaderGrid\",\"renderPromotionBlockedReason\":\"line-rule-placement-and-topology-unproven\"");
    output.push_str(",\"candidateSource\":");
    output.push_str(&json_string(rule.candidate_source));
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(projection.line_mark_profile));
    output.push_str(",\"lineMarkIntervalCount\":");
    output.push_str(&projection.line_mark_interval_count.to_string());
    output.push_str(",\"documentTextGroupCount\":");
    output.push_str(&projection.document_text_group_count.to_string());
    output.push_str(",\"documentTextLineHeaderCount\":");
    output.push_str(&projection.document_text_line_header_count.to_string());
    output.push_str(",\"skippedInlineLineHeaderCount\":");
    output.push_str(&projection.skipped_inline_line_header_count.to_string());
    output.push_str(",\"strokeColor\":\"#111111\",\"strokeWidth\":");
    output.push_str(&format!("{:.3}", projection.stroke_width));
    output.push_str(",\"orientation\":");
    output.push_str(&json_string(rule.orientation));
    output.push_str(",\"groupIndex\":");
    output.push_str(&rule.group_index.to_string());
    output.push_str(",\"endGroupIndex\":");
    output.push_str(&rule.end_group_index.to_string());
    output.push_str(",\"lineOffsetUnits\":");
    output.push_str(&rule.line_offset_units.to_string());
    output.push_str(",\"lineExtentUnits\":");
    output.push_str(&rule.line_extent_units.to_string());
    output.push_str(",\"lineHeaderHex\":");
    output.push_str(&json_string(&rule.line_header_hex));
    output.push_str(",\"lineHeaderRawWords\":");
    push_u16_array_json(output, &rule.line_header_raw_words);
    output.push_str(",\"lineHeaderRawWordsHex\":");
    push_u16_hex_array_json(output, &rule.line_header_raw_words);
    output.push_str(",\"topologyCandidate\":{\"orthogonalGraph\":");
    output.push_str(if topology.orthogonal_graph_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"startJunctionDegree\":");
    output.push_str(&topology.start_junction_degree.to_string());
    output.push_str(",\"endJunctionDegree\":");
    output.push_str(&topology.end_junction_degree.to_string());
    output.push_str(",\"isolatedEndpointCount\":");
    output.push_str(&topology.isolated_endpoint_count.to_string());
    output.push('}');
    output.push_str(",\"endpointAttachmentCandidates\":");
    push_shanai_lan_line_rule_endpoint_attachment_candidates_json(
        output,
        projection,
        rule,
        topology,
        text_projection,
    );
    output.push_str(",\"renderAdmissionGate\":");
    push_shanai_lan_line_rule_render_admission_gate_json(
        output,
        projection,
        rule_index,
        rule,
        topology,
        component.as_ref(),
        text_projection,
    );
    if let Some(line_mark) = rule.line_mark {
        output.push_str(",\"lineMarkRecordIndex\":");
        output.push_str(&line_mark.record_index.to_string());
        output.push_str(",\"lineMarkUnitInterval\":");
        output.push_str(&source_range_json(line_mark.unit_start, line_mark.unit_end));
        output.push_str(",\"lineMarkFlagHex\":");
        output.push_str(&json_string(&format!("0x{:04x}", line_mark.flag_word)));
    }
    output.push_str(",\"sourceByteRange\":");
    output.push_str(&source_range_json(
        rule.source_span.byte_start(),
        rule.source_span.byte_end(),
    ));
    output.push_str(",\"sourceUnitRange\":");
    output.push_str(&source_range_json(
        rule.source_span.unit_start(),
        rule.source_span.unit_end(),
    ));
    output.push_str(",\"gridUnitPx\":");
    output.push_str(&format!("{:.3}", projection.grid_unit_px));
    output.push_str(",\"lineHeightPx\":");
    output.push_str(&format!("{:.3}", projection.line_height_px));
    output.push('}');
}

pub(crate) fn push_page_layer_shanai_lan_line_rule_projection_summary_json(
    output: &mut String,
    layout: PageLayout,
    projection: &ShanaiLanLineRuleProjection,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    let mut candidate_source_counts = BTreeMap::<&'static str, usize>::new();
    let mut orientation_counts = BTreeMap::<&'static str, usize>::new();
    let component_summaries = shanai_lan_line_rule_graph_component_summaries(projection);
    let mut orthogonal_graph_candidate_count = 0usize;
    let mut no_isolated_endpoint_rule_count = 0usize;
    let mut one_isolated_endpoint_rule_count = 0usize;
    let mut two_isolated_endpoint_rule_count = 0usize;
    let mut line_mark_matched_rule_count = 0usize;
    let mut endpoint_attachment_within_line_height_count = 0usize;
    let mut both_endpoint_attachment_within_line_height_rule_count = 0usize;

    for rule in &projection.rules {
        *candidate_source_counts
            .entry(rule.candidate_source)
            .or_insert(0) += 1;
        *orientation_counts.entry(rule.orientation).or_insert(0) += 1;
        let topology = shanai_lan_line_rule_topology(projection, rule);
        if topology.orthogonal_graph_candidate {
            orthogonal_graph_candidate_count += 1;
        }
        match topology.isolated_endpoint_count {
            0 => no_isolated_endpoint_rule_count += 1,
            1 => one_isolated_endpoint_rule_count += 1,
            _ => two_isolated_endpoint_rule_count += 1,
        }
        if rule.line_mark.is_some() {
            line_mark_matched_rule_count += 1;
        }
        let start_attached = shanai_lan_line_rule_endpoint_attaches_to_text(
            rule.x1,
            rule.y1,
            projection,
            text_projection,
        );
        let end_attached = shanai_lan_line_rule_endpoint_attaches_to_text(
            rule.x2,
            rule.y2,
            projection,
            text_projection,
        );
        endpoint_attachment_within_line_height_count += usize::from(start_attached);
        endpoint_attachment_within_line_height_count += usize::from(end_attached);
        if start_attached && end_attached {
            both_endpoint_attachment_within_line_height_rule_count += 1;
        }
    }

    output.push_str("{\"type\":\"documentTextLineRuleProjectionSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"/DocumentText+/LineMark\"");
    output.push_str(",\"sourceStream\":");
    output.push_str(&json_string(projection.source));
    output.push_str(",\"projectionKind\":\"documentTextLineRuleProjectionSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"line-rule-placement-and-topology-unproven\"",
    );
    output.push_str(",\"renderPromotionBlockedDetail\":\"line-rule-endpoint-attachments-and-line-mark-row-boundaries-unproven\"");
    output.push_str(",\"lineMarkProfile\":");
    output.push_str(&json_string(projection.line_mark_profile));
    output.push_str(",\"ruleCount\":");
    output.push_str(&projection.rules.len().to_string());
    output.push_str(",\"candidateSourceCounts\":");
    push_static_str_count_map_json(output, &candidate_source_counts);
    output.push_str(",\"orientationCounts\":");
    push_static_str_count_map_json(output, &orientation_counts);
    output.push_str(",\"orthogonalGraphCandidateRuleCount\":");
    output.push_str(&orthogonal_graph_candidate_count.to_string());
    output.push_str(",\"noIsolatedEndpointRuleCount\":");
    output.push_str(&no_isolated_endpoint_rule_count.to_string());
    output.push_str(",\"oneIsolatedEndpointRuleCount\":");
    output.push_str(&one_isolated_endpoint_rule_count.to_string());
    output.push_str(",\"twoIsolatedEndpointRuleCount\":");
    output.push_str(&two_isolated_endpoint_rule_count.to_string());
    output.push_str(",\"lineMarkMatchedRuleCount\":");
    output.push_str(&line_mark_matched_rule_count.to_string());
    output.push_str(",\"lineMarkIntervalCount\":");
    output.push_str(&projection.line_mark_interval_count.to_string());
    output.push_str(",\"documentTextGroupCount\":");
    output.push_str(&projection.document_text_group_count.to_string());
    output.push_str(",\"documentTextLineHeaderCount\":");
    output.push_str(&projection.document_text_line_header_count.to_string());
    output.push_str(",\"skippedInlineLineHeaderCount\":");
    output.push_str(&projection.skipped_inline_line_header_count.to_string());
    output.push_str(",\"endpointCount\":");
    output.push_str(&(projection.rules.len() * 2).to_string());
    output.push_str(",\"endpointAttachmentWithinLineHeightCount\":");
    output.push_str(&endpoint_attachment_within_line_height_count.to_string());
    output.push_str(",\"bothEndpointAttachmentWithinLineHeightRuleCount\":");
    output.push_str(&both_endpoint_attachment_within_line_height_rule_count.to_string());
    output.push_str(",\"lineRuleRenderAdmissionGate\":");
    push_shanai_lan_line_rule_projection_render_admission_gate_json(
        output,
        projection,
        &component_summaries,
        orthogonal_graph_candidate_count,
        no_isolated_endpoint_rule_count,
        line_mark_matched_rule_count,
        both_endpoint_attachment_within_line_height_rule_count,
    );
    output.push_str(",\"lineRuleGraphComponentCount\":");
    output.push_str(&component_summaries.len().to_string());
    output.push_str(",\"largestLineRuleGraphComponentRuleCount\":");
    output.push_str(
        &component_summaries
            .iter()
            .map(|component| component.rule_indexes.len())
            .max()
            .unwrap_or(0)
            .to_string(),
    );
    output.push_str(",\"lineRuleGraphComponents\":");
    push_shanai_lan_line_rule_graph_components_json(output, &component_summaries);
    output.push_str(",\"gridUnitPx\":");
    output.push_str(&format!("{:.3}", projection.grid_unit_px));
    output.push_str(",\"lineHeightPx\":");
    output.push_str(&format!("{:.3}", projection.line_height_px));
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!("{:.3}", projection.stroke_width));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_shanai_lan_line_header_grid_origin_authority_gate_json(
    output: &mut String,
    document: &Document,
    line_headers: &[ShanaiLanLineHeaderInGroup],
    selected_horizontal_rules: &BTreeSet<(usize, usize, u16, u16)>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    grid_origin_x: f32,
    grid_origin_y: f32,
    grid_unit_px: f32,
    line_height_px: f32,
    raw_max_extent_units: u16,
    max_extent_units: u16,
) {
    let selected_headers = line_headers
        .iter()
        .filter(|line_header| {
            selected_horizontal_rules.contains(&(
                line_header.header.start,
                line_header.group_index,
                line_header.header.offset_units,
                line_header.header.extent_units,
            ))
        })
        .collect::<Vec<_>>();
    let mut selected_group_indexes = selected_headers
        .iter()
        .map(|line_header| line_header.group_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let selected_line_mark_record_indexes = selected_headers
        .iter()
        .filter_map(|line_header| {
            shanai_lan_line_mark_for_header(line_mark_intervals, &line_header.header)
                .map(|line_mark| line_mark.record_index)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let group_record_pairs = selected_headers
        .iter()
        .filter_map(|line_header| {
            shanai_lan_line_mark_for_header(line_mark_intervals, &line_header.header)
                .map(|line_mark| (line_header.group_index, line_mark.record_index))
        })
        .collect::<Vec<_>>();
    let record_index_minus_group_index_values = group_record_pairs
        .iter()
        .map(|(group_index, record_index)| *record_index as i32 - *group_index as i32)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let uniform_record_index_minus_group_index =
        record_index_minus_group_index_values.len() == 1 && !group_record_pairs.is_empty();
    let line_mark_record_indexes_contiguous =
        usize_values_are_contiguous(&selected_line_mark_record_indexes);
    let line_mark_record_stride = uniform_usize_stride(&selected_line_mark_record_indexes);
    let selected_line_mark_intervals = selected_line_mark_record_indexes
        .iter()
        .filter_map(|record_index| {
            line_mark_intervals
                .iter()
                .find(|interval| interval.record_index == *record_index)
                .copied()
        })
        .collect::<Vec<_>>();
    let page_mark_entry = shanai_lan_page_mark_entry_covering_line_mark_records(
        document,
        &selected_line_mark_record_indexes,
    );
    let page_mark_entry_count = document
        .page_marks()
        .first()
        .map(|page_mark| page_mark.entries().len())
        .unwrap_or_default();
    let all_selected_headers_have_line_mark =
        !selected_headers.is_empty() && group_record_pairs.len() == selected_headers.len();
    let source_domain_row_anchor_candidate =
        all_selected_headers_have_line_mark && uniform_record_index_minus_group_index;
    let page_mark_entry_coverage_ready =
        page_mark_entry.is_some() && !selected_line_mark_record_indexes.is_empty();

    selected_group_indexes.sort_unstable();

    output.push_str("{\"basis\":\"selectedDocumentTextLineHeaders+/LineMark+/PageMark\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"selectedLineHeaderCount\":");
    output.push_str(&selected_headers.len().to_string());
    output.push_str(",\"selectedGroupIndexes\":");
    push_usize_array_json(output, &selected_group_indexes);
    output.push_str(",\"selectedLineMarkRecordIndexes\":");
    push_usize_array_json(output, &selected_line_mark_record_indexes);
    output.push_str(",\"selectedLineMarkSourceUnitGate\":");
    push_shanai_lan_selected_line_mark_source_unit_gate_json(
        output,
        &selected_line_mark_record_indexes,
        &selected_line_mark_intervals,
    );
    output.push_str(",\"allSelectedHeadersHaveLineMark\":");
    output.push_str(&all_selected_headers_have_line_mark.to_string());
    output.push_str(",\"lineMarkRecordIndexesContiguous\":");
    output.push_str(&line_mark_record_indexes_contiguous.to_string());
    output.push_str(",\"lineMarkRecordStride\":");
    push_option_usize_json(output, line_mark_record_stride);
    output.push_str(",\"recordIndexMinusGroupIndexValues\":");
    push_i32_array_json(output, &record_index_minus_group_index_values);
    output.push_str(",\"uniformRecordIndexMinusGroupIndex\":");
    output.push_str(&uniform_record_index_minus_group_index.to_string());
    output.push_str(",\"sourceDomainRowAnchorCandidate\":");
    output.push_str(&source_domain_row_anchor_candidate.to_string());
    output.push_str(",\"pageMarkEntryCount\":");
    output.push_str(&page_mark_entry_count.to_string());
    output.push_str(",\"pageMarkEntryCoverageReady\":");
    output.push_str(&page_mark_entry_coverage_ready.to_string());
    output.push_str(",\"pageMarkEntryCoverage\":");
    if let Some(entry) = page_mark_entry {
        output.push_str("{\"rowIndex\":");
        output.push_str(&entry.row_index().to_string());
        output.push_str(",\"index\":");
        push_option_u32_json(output, entry.index());
        output.push_str(",\"flags\":");
        push_option_u32_json(output, entry.flags());
        output.push_str(",\"flagsHex\":");
        push_option_u32_hex_or_null_json(output, entry.flags());
        output.push_str(",\"lineStart\":");
        push_option_u32_json(output, entry.line_start());
        output.push_str(",\"lineEnd\":");
        push_option_u32_json(output, entry.line_end());
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"pageMarkEntryProfileGate\":");
    push_shanai_lan_page_mark_entry_profile_gate_json(output, page_mark_entry);
    output.push_str(",\"sourceOnlyGridDomain\":{\"rawMaxExtentUnits\":");
    output.push_str(&raw_max_extent_units.to_string());
    output.push_str(",\"maxExtentUnits\":");
    output.push_str(&max_extent_units.to_string());
    output.push_str(",\"textGridColumnOriginDecoded\":false,\"textGridRowOriginDecoded\":false}");
    output.push_str(",\"currentProjection\":{\"originX\":");
    output.push_str(&format!("{grid_origin_x:.3}"));
    output.push_str(",\"originY\":");
    output.push_str(&format!("{grid_origin_y:.3}"));
    output.push_str(",\"gridUnitPx\":");
    output.push_str(&format!("{grid_unit_px:.3}"));
    output.push_str(",\"lineHeightPx\":");
    output.push_str(&format!("{line_height_px:.3}"));
    output.push_str(",\"referenceBacked\":true}");
    output.push_str(",\"sourceOnlyPageMarkYValueProbe\":");
    push_shanai_lan_page_mark_y_value_probe_json(output, page_mark_entry, grid_origin_y);
    output.push_str(",\"pageSpaceOriginCandidate\":null");
    output.push_str(",\"pageSpaceOriginCandidateReady\":false");
    output.push_str(",\"promotionReady\":false");
    output.push_str(",\"blockedReasons\":[");
    let mut reasons = Vec::new();
    if selected_headers.is_empty() {
        reasons.push("selected-line-header-run-missing");
    }
    if !all_selected_headers_have_line_mark {
        reasons.push("selected-line-header-line-mark-coverage-incomplete");
    }
    if !uniform_record_index_minus_group_index {
        reasons.push("line-mark-record-index-to-document-text-group-fit-not-uniform");
    }
    if !page_mark_entry_coverage_ready {
        reasons.push("page-mark-entry-coverage-missing");
    }
    reasons.push("document-text-grid-origin-reference-backed");
    reasons.push("line-header-visible-rule-selector-unproven");
    reasons.push("page-space-y-origin-unproven");
    for (index, reason) in reasons.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&json_string(reason));
    }
    output.push_str(
        "],\"renderPromotionBlockedReason\":\"line-header-grid-origin-authority-unproven\"}",
    );
}

pub(crate) fn shanai_lan_page_mark_entry_covering_line_mark_records<'a>(
    document: &'a Document,
    line_mark_record_indexes: &[usize],
) -> Option<&'a DocumentPageMarkEntry> {
    let first = *line_mark_record_indexes.first()?;
    let last = *line_mark_record_indexes.last()?;
    document
        .page_marks()
        .first()?
        .entries()
        .iter()
        .find(|entry| {
            let (Some(start), Some(end)) = (entry.line_start(), entry.line_end()) else {
                return false;
            };
            start as usize <= first && last <= end as usize
        })
}

pub(crate) fn push_shanai_lan_page_mark_entry_profile_gate_json(
    output: &mut String,
    page_mark_entry: Option<&DocumentPageMarkEntry>,
) {
    let profile = page_mark_entry.map(DocumentPageMarkEntry::u16_geometry_profile);
    let class_name = profile
        .as_ref()
        .map(PageMarkU16GeometryProfile::class_name)
        .unwrap_or("missing");
    let additive_geometry_profile = profile.is_some_and(|profile| {
        profile.non_zero_additive_unit_candidate() && !profile.selected_fields_all_zero()
    });
    let promotion_safe_profile = additive_geometry_profile;

    output.push_str("{\"source\":\"/PageMark u16 geometry profile\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"entryPresent\":");
    output.push_str(&page_mark_entry.is_some().to_string());
    output.push_str(",\"u16GeometryClass\":");
    output.push_str(&json_string(class_name));
    output.push_str(",\"additiveGeometryProfile\":");
    output.push_str(&additive_geometry_profile.to_string());
    output.push_str(",\"promotionSafeProfile\":");
    output.push_str(&promotion_safe_profile.to_string());
    output.push_str(",\"blockedReason\":");
    output.push_str(&json_string(if promotion_safe_profile {
        "page-mark-profile-still-needs-field-role-proof"
    } else {
        "page-mark-mixed-payload-profile-not-layout-origin-authority"
    }));
    output.push('}');
}

pub(crate) fn push_shanai_lan_selected_line_mark_source_unit_gate_json(
    output: &mut String,
    selected_record_indexes: &[usize],
    intervals: &[ShanaiLanLineMarkInterval],
) {
    let interval_record_indexes = intervals
        .iter()
        .map(|interval| interval.record_index)
        .collect::<Vec<_>>();
    let unit_starts = intervals
        .iter()
        .map(|interval| interval.unit_start)
        .collect::<Vec<_>>();
    let unit_ends = intervals
        .iter()
        .map(|interval| interval.unit_end)
        .collect::<Vec<_>>();
    let unit_spans = intervals
        .iter()
        .map(|interval| interval.unit_end.saturating_sub(interval.unit_start))
        .collect::<Vec<_>>();
    let record_index_deltas = interval_record_indexes
        .windows(2)
        .map(|window| window[1].saturating_sub(window[0]))
        .collect::<Vec<_>>();
    let unit_start_deltas = unit_starts
        .windows(2)
        .map(|window| window[1].saturating_sub(window[0]))
        .collect::<Vec<_>>();
    let source_unit_delta_per_record = record_index_deltas
        .first()
        .copied()
        .zip(unit_start_deltas.first().copied())
        .and_then(|(record_delta, unit_delta)| {
            (record_delta > 0).then_some(unit_delta as f32 / record_delta as f32)
        });
    let all_selected_records_have_intervals =
        selected_record_indexes.len() == intervals.len() && !selected_record_indexes.is_empty();
    let stride_candidate_sample_count = record_index_deltas.len();
    let stride_candidate_ready =
        all_selected_records_have_intervals && stride_candidate_sample_count >= 2;

    output.push_str("{\"source\":\"/LineMark selected record source-unit intervals\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"selectedRecordCount\":");
    output.push_str(&selected_record_indexes.len().to_string());
    output.push_str(",\"intervalRecordCount\":");
    output.push_str(&intervals.len().to_string());
    output.push_str(",\"allSelectedRecordsHaveIntervals\":");
    output.push_str(&all_selected_records_have_intervals.to_string());
    output.push_str(",\"recordIndexes\":");
    push_usize_array_json(output, &interval_record_indexes);
    output.push_str(",\"unitStarts\":");
    push_usize_array_json(output, &unit_starts);
    output.push_str(",\"unitEnds\":");
    push_usize_array_json(output, &unit_ends);
    output.push_str(",\"unitSpans\":");
    push_usize_array_json(output, &unit_spans);
    output.push_str(",\"recordIndexDeltas\":");
    push_usize_array_json(output, &record_index_deltas);
    output.push_str(",\"unitStartDeltas\":");
    push_usize_array_json(output, &unit_start_deltas);
    output.push_str(",\"sourceUnitDeltaPerRecordEstimate\":");
    push_optional_f32_json(output, source_unit_delta_per_record);
    output.push_str(",\"strideCandidateSampleCount\":");
    output.push_str(&stride_candidate_sample_count.to_string());
    output.push_str(",\"strideCandidateReady\":");
    output.push_str(&stride_candidate_ready.to_string());
    output.push_str(",\"promotionReady\":false,\"blockedReason\":");
    output.push_str(&json_string(if stride_candidate_ready {
        "line-mark-source-unit-to-page-y-transform-unproven"
    } else {
        "line-mark-source-unit-stride-insufficient-selected-rows"
    }));
    output.push('}');
}

pub(crate) fn push_shanai_lan_page_mark_y_value_probe_json(
    output: &mut String,
    page_mark_entry: Option<&DocumentPageMarkEntry>,
    current_projection_origin_y: f32,
) {
    let mut candidates = Vec::<PageMarkScopedYValueCandidate>::new();
    if let Some(entry) = page_mark_entry {
        collect_page_mark_entry_y_value_candidates(&mut candidates, entry);
    }
    let in_page_range_candidates = candidates
        .iter()
        .filter(|candidate| {
            (0.0..=SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX).contains(&candidate.value_px)
        })
        .collect::<Vec<_>>();
    let nearest_current_origin =
        nearest_page_mark_scoped_y_candidate(current_projection_origin_y, &candidates);

    output.push_str("{\"source\":\"/PageMark parsed entry y-value candidates\"");
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"pageMarkEntryPresent\":");
    output.push_str(&page_mark_entry.is_some().to_string());
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidates.len().to_string());
    output.push_str(",\"inPageRangeCandidateCount\":");
    output.push_str(&in_page_range_candidates.len().to_string());
    output.push_str(",\"currentProjectionOriginY\":");
    output.push_str(&format!("{current_projection_origin_y:.3}"));
    output.push_str(",\"nearestCurrentProjectionOriginCandidate\":");
    if let Some((candidate, residual)) = nearest_current_origin {
        push_page_mark_scoped_y_candidate_json(output, candidate, residual);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"lineBoundaryConflictGate\":");
    push_shanai_lan_page_mark_y_line_boundary_conflict_json(
        output,
        page_mark_entry,
        nearest_current_origin.map(|(candidate, _)| candidate),
    );
    output.push_str(",\"candidatePreview\":");
    push_shanai_lan_page_mark_y_value_candidate_preview_json(output, &in_page_range_candidates);
    output.push_str(",\"selectionReady\":false,\"promotionReady\":false");
    output.push_str(",\"blockedReasons\":[\"page-mark-y-value-field-role-unproven\",\"document-text-grid-origin-reference-backed\",\"page-space-y-origin-unproven\"]");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"source-only-page-space-y-origin-unproven\"}",
    );
}

pub(crate) fn push_shanai_lan_page_mark_y_line_boundary_conflict_json(
    output: &mut String,
    page_mark_entry: Option<&DocumentPageMarkEntry>,
    nearest_candidate: Option<&PageMarkScopedYValueCandidate>,
) {
    let line_start = page_mark_entry.and_then(DocumentPageMarkEntry::line_start);
    let line_end = page_mark_entry.and_then(DocumentPageMarkEntry::line_end);
    let nearest_value = nearest_candidate.map(|candidate| candidate.value);
    let matches_line_start = matches!(
        (nearest_value, line_start),
        (Some(candidate_value), Some(line_start)) if candidate_value == line_start
    );
    let matches_line_end = matches!(
        (nearest_value, line_end),
        (Some(candidate_value), Some(line_end)) if candidate_value == line_end
    );
    let line_boundary_conflict = matches_line_start || matches_line_end;

    output.push_str(
        "{\"source\":\"/PageMark parsed entry lineStart/lineEnd vs nearest y candidate\"",
    );
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false,\"renderable\":false");
    output.push_str(",\"lineStart\":");
    push_option_u32_json(output, line_start);
    output.push_str(",\"lineEnd\":");
    push_option_u32_json(output, line_end);
    output.push_str(",\"nearestCandidateValue\":");
    push_option_u32_json(output, nearest_value);
    output.push_str(",\"matchesLineStart\":");
    output.push_str(&matches_line_start.to_string());
    output.push_str(",\"matchesLineEnd\":");
    output.push_str(&matches_line_end.to_string());
    output.push_str(",\"matchedBoundaryRoles\":[");
    let mut first = true;
    if matches_line_start {
        output.push_str("\"lineStart\"");
        first = false;
    }
    if matches_line_end {
        if !first {
            output.push(',');
        }
        output.push_str("\"lineEnd\"");
    }
    output.push_str("],\"lineBoundaryConflict\":");
    output.push_str(&line_boundary_conflict.to_string());
    output.push_str(",\"selectionReady\":false,\"promotionReady\":false");
    output.push_str(",\"blockedReason\":");
    output.push_str(&json_string(if line_boundary_conflict {
        "nearest-page-mark-y-candidate-overlaps-line-boundary"
    } else {
        "page-mark-y-value-field-role-unproven"
    }));
    output.push('}');
}

pub(crate) fn push_shanai_lan_page_mark_y_value_candidate_preview_json(
    output: &mut String,
    candidates: &[&PageMarkScopedYValueCandidate],
) {
    output.push('[');
    for (index, candidate) in candidates.iter().take(12).enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_page_mark_scoped_y_candidate_json(output, candidate, 0.0);
    }
    output.push(']');
}

pub(crate) fn push_page_layer_shanai_lan_line_header_projection_candidate_summary_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    bytes: &[u8],
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    selected_projection: Option<&ShanaiLanLineRuleProjection>,
) {
    let map = map_document_text(bytes);
    let skipped_inline_spans = map
        .entries()
        .iter()
        .filter(|entry| entry.kind() == DocumentTextMapKind::SkippedInlineText)
        .map(|entry| (entry.byte_start(), entry.byte_end()))
        .collect::<Vec<_>>();
    let group_offsets = shanai_lan_text_group_offsets(bytes);
    let line_headers = shanai_lan_line_headers_in_groups(bytes, &group_offsets);
    let raw_max_extent_units = shanai_lan_text_max_extent_units(bytes).unwrap_or(0x0118);
    let max_extent_units = raw_max_extent_units
        .saturating_sub(SHANAI_LAN_TEXT_GRID_EXTENT_GUTTER_UNITS)
        .max(1);
    let viewport = fdm_projection_viewport(layout);
    let scale_y = layout.height_px() / SHANAI_LAN_REFERENCE_PAGE_HEIGHT_PX;
    let fallback_grid_unit_px = viewport.width / f32::from(max_extent_units);
    let fallback_line_height_px = 12.0 * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR * scale_y;
    let fallback_stroke_width = SHANAI_LAN_LINE_RULE_STROKE_WIDTH_PX * scale_y;
    let (grid_origin_x, grid_origin_y, grid_unit_px, line_height_px, stroke_width) =
        selected_projection
            .and_then(|projection| {
                projection
                    .rules
                    .iter()
                    .find(|rule| rule.orientation == "horizontal")
                    .map(|rule| {
                        (
                            rule.x1 - f32::from(rule.line_offset_units) * projection.grid_unit_px,
                            rule.y1 - (rule.group_index as f32 + 1.0) * projection.line_height_px,
                            projection.grid_unit_px,
                            projection.line_height_px,
                            projection.stroke_width,
                        )
                    })
            })
            .unwrap_or((
                viewport.x,
                viewport.y,
                fallback_grid_unit_px,
                fallback_line_height_px,
                fallback_stroke_width,
            ));
    let selected_horizontal_rules = selected_projection
        .map(|projection| {
            projection
                .rules
                .iter()
                .filter(|rule| rule.orientation == "horizontal")
                .map(|rule| {
                    (
                        rule.source_span.byte_start(),
                        rule.group_index,
                        rule.line_offset_units,
                        rule.line_extent_units,
                    )
                })
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let mut all_line_header_count = 0usize;
    let mut long_line_header_count = 0usize;
    let mut skipped_inline_long_line_header_count = 0usize;
    let mut selected_skipped_inline_long_line_header_count = 0usize;
    let mut group_counts = BTreeMap::<usize, usize>::new();

    output.push_str("{\"type\":\"documentTextLineHeaderProjectionCandidateSummary\",\"bbox\":");
    output.push_str(&format!(
        "{{\"x\":0.000,\"y\":0.000,\"width\":{:.3},\"height\":{:.3}}}",
        layout.width_px(),
        layout.height_px()
    ));
    output.push_str(",\"source\":\"/DocumentText\"");
    output.push_str(",\"sourceStream\":\"/DocumentText\"");
    output.push_str(",\"projectionKind\":\"documentTextLineHeaderProjectionCandidateSummary\"");
    output.push_str(",\"decoded\":false,\"diagnosticOnly\":true,\"geometryDecoded\":true,\"placementProven\":false,\"renderable\":false,\"referenceBacked\":true");
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"line-header-visible-rule-selector-unproven\"",
    );
    output.push_str(
        ",\"selectorBasis\":\"current-horizontal-rule-promotion-requires-skipped-inline-text\"",
    );
    output.push_str(",\"fullSpanRenderPromotionGate\":{\"basis\":\"documentTextLineHeaderCandidate+sourceMapContext\",\"sourceBacked\":true,\"decoded\":false,\"diagnosticOnly\":true,\"renderable\":false");
    output.push_str(",\"renderPromotionBlockedReason\":\"line-header-segment-clipping-and-endpoint-ownership-unproven\"");
    output.push_str(",\"requiresSegmentClippingDecoded\":true,\"requiresEndpointOwnershipDecoded\":true,\"requiresPaintOrderDecoded\":true");
    output.push_str(",\"fullSpanRenderableCandidateCount\":0}");
    output.push_str(",\"gridOriginAuthorityGate\":");
    push_shanai_lan_line_header_grid_origin_authority_gate_json(
        output,
        document,
        &line_headers,
        &selected_horizontal_rules,
        line_mark_intervals,
        grid_origin_x,
        grid_origin_y,
        grid_unit_px,
        line_height_px,
        raw_max_extent_units,
        max_extent_units,
    );
    output.push_str(",\"gridUnitPx\":");
    output.push_str(&format!("{grid_unit_px:.3}"));
    output.push_str(",\"lineHeightPx\":");
    output.push_str(&format!("{line_height_px:.3}"));
    output.push_str(",\"strokeWidth\":");
    output.push_str(&format!("{stroke_width:.3}"));
    output.push_str(",\"minSegmentUnits\":");
    output.push_str(&SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS.to_string());
    output.push_str(",\"rawMaxExtentUnits\":");
    output.push_str(&raw_max_extent_units.to_string());
    output.push_str(",\"maxExtentUnits\":");
    output.push_str(&max_extent_units.to_string());
    output.push_str(",\"candidates\":[");

    let mut emitted = 0usize;
    for line_header in &line_headers {
        all_line_header_count += 1;
        let header = line_header.header;
        if header.extent_units <= header.offset_units {
            continue;
        }
        let segment_units = header.extent_units - header.offset_units;
        if segment_units < SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS {
            continue;
        }
        long_line_header_count += 1;
        *group_counts.entry(line_header.group_index).or_default() += 1;
        let skipped_inline = skipped_inline_spans
            .iter()
            .any(|(start, end)| *start <= header.start && header.end <= *end);
        if skipped_inline {
            skipped_inline_long_line_header_count += 1;
        }
        let selected_as_horizontal_rule = selected_horizontal_rules.contains(&(
            header.start,
            line_header.group_index,
            header.offset_units,
            header.extent_units,
        ));
        if selected_as_horizontal_rule && skipped_inline {
            selected_skipped_inline_long_line_header_count += 1;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        let x = grid_origin_x + f32::from(header.offset_units) * grid_unit_px;
        let y = grid_origin_y + (line_header.group_index as f32 + 1.0) * line_height_px;
        let width = f32::from(segment_units) * grid_unit_px;
        let half_stroke = stroke_width * 0.5;
        output.push_str("{\"groupIndex\":");
        output.push_str(&line_header.group_index.to_string());
        output.push_str(",\"lineOffsetUnits\":");
        output.push_str(&header.offset_units.to_string());
        output.push_str(",\"lineExtentUnits\":");
        output.push_str(&header.extent_units.to_string());
        output.push_str(",\"segmentUnits\":");
        output.push_str(&segment_units.to_string());
        output.push_str(",\"bbox\":");
        push_bbox_tuple_json(
            output,
            (
                x - half_stroke,
                y - half_stroke,
                width + stroke_width,
                stroke_width,
            ),
        );
        output.push_str(",\"candidateSource\":");
        output.push_str(&json_string(if skipped_inline {
            "skippedInlineText"
        } else {
            "documentTextLineHeader"
        }));
        output.push_str(",\"selectedAsHorizontalRule\":");
        output.push_str(if selected_as_horizontal_rule {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"fullSpanRenderPromotionGate\":{\"basis\":\"documentTextLineHeaderCandidate+sourceMapContext\",\"sourceBacked\":true,\"decoded\":false,\"renderable\":false");
        output.push_str(",\"fullSpanCandidate\":");
        output.push_str(if selected_as_horizontal_rule {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"renderPromotionBlockedReason\":\"line-header-segment-clipping-and-endpoint-ownership-unproven\"}");
        output.push_str(",\"lineMarkRecordIndex\":");
        if let Some(line_mark) = shanai_lan_line_mark_for_header(line_mark_intervals, &header) {
            output.push_str(&line_mark.record_index.to_string());
        } else {
            output.push_str("null");
        }
        output.push_str(",\"lineMarkContext\":");
        push_shanai_lan_line_header_line_mark_context_json(output, line_mark_intervals, &header);
        output.push_str(",\"documentTextMapContext\":");
        push_shanai_lan_line_header_map_context_json(output, map.entries(), &header);
        output.push_str(",\"sameSegmentGroupRun\":");
        push_shanai_lan_line_header_same_segment_group_run_json(
            output,
            &line_headers,
            line_header.group_index,
            header.offset_units,
            header.extent_units,
        );
        output.push_str(",\"lineHeaderRawWordsHex\":");
        push_u16_hex_array_json(output, &header.raw_words);
        output.push_str(",\"sourceByteRange\":");
        output.push_str(&source_range_json(header.start, header.end));
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(header.start / 2, header.end / 2));
        output.push('}');
    }
    output.push_str("],\"allLineHeaderCount\":");
    output.push_str(&all_line_header_count.to_string());
    output.push_str(",\"longLineHeaderCandidateCount\":");
    output.push_str(&long_line_header_count.to_string());
    output.push_str(",\"skippedInlineLongLineHeaderCandidateCount\":");
    output.push_str(&skipped_inline_long_line_header_count.to_string());
    output.push_str(",\"selectedSkippedInlineLongLineHeaderCandidateCount\":");
    output.push_str(&selected_skipped_inline_long_line_header_count.to_string());
    output.push_str(",\"unselectedLongLineHeaderCandidateCount\":");
    output.push_str(
        &long_line_header_count
            .saturating_sub(selected_skipped_inline_long_line_header_count)
            .to_string(),
    );
    output.push_str(",\"candidateGroupCounts\":");
    push_usize_count_map_json(output, &group_counts);
    output.push_str(",\"sameSegmentGroupRuns\":");
    push_shanai_lan_line_header_same_segment_group_runs_json(
        output,
        map.entries(),
        &line_headers,
        &skipped_inline_spans,
        &selected_horizontal_rules,
        line_mark_intervals,
    );
    output.push('}');
}

pub(crate) fn push_shanai_lan_line_header_line_mark_context_json(
    output: &mut String,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    header: &ShanaiLanLineHeader,
) {
    let Some(line_mark) = shanai_lan_line_mark_for_header(line_mark_intervals, header) else {
        output.push_str("null");
        return;
    };
    output.push_str("{\"recordIndex\":");
    output.push_str(&line_mark.record_index.to_string());
    output.push_str(",\"unitRange\":");
    output.push_str(&source_range_json(line_mark.unit_start, line_mark.unit_end));
    output.push_str(",\"flagWord\":");
    output.push_str(&line_mark.flag_word.to_string());
    output.push_str(",\"flagWordHex\":");
    output.push_str(&json_string(&format!("0x{:04x}", line_mark.flag_word)));
    output.push_str(",\"headerUnitOffsetFromLineMarkStart\":");
    output.push_str(
        &(header.start / 2)
            .saturating_sub(line_mark.unit_start)
            .to_string(),
    );
    output.push_str(",\"headerWithinLineMark\":");
    output.push_str(
        if line_mark.unit_start <= header.start / 2 && header.end / 2 <= line_mark.unit_end {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(crate) fn push_shanai_lan_line_header_map_context_json(
    output: &mut String,
    entries: &[DocumentTextMapEntry],
    header: &ShanaiLanLineHeader,
) {
    let containing = entries
        .iter()
        .find(|entry| entry.byte_start() <= header.start && header.end <= entry.byte_end());
    let previous = entries
        .iter()
        .rev()
        .find(|entry| entry.byte_end() <= header.start);
    let next = entries
        .iter()
        .find(|entry| entry.byte_start() >= header.end);

    output.push_str("{\"containingEntry\":");
    push_document_text_map_entry_brief_json(output, containing);
    output.push_str(",\"previousEntry\":");
    push_document_text_map_entry_brief_json(output, previous);
    output.push_str(",\"nextEntry\":");
    push_document_text_map_entry_brief_json(output, next);
    output.push_str(",\"insideSkippedInlineText\":");
    output.push_str(
        if containing.is_some_and(|entry| entry.kind() == DocumentTextMapKind::SkippedInlineText) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"insideTextRun\":");
    output.push_str(
        if containing.is_some_and(|entry| entry.kind() == DocumentTextMapKind::TextRun) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"adjacentToSkippedInlineText\":");
    output.push_str(
        if previous
            .or(next)
            .is_some_and(|entry| entry.kind() == DocumentTextMapKind::SkippedInlineText)
        {
            "true"
        } else {
            "false"
        },
    );
    output.push('}');
}

pub(crate) fn push_shanai_lan_line_header_same_segment_group_run_json(
    output: &mut String,
    line_headers: &[ShanaiLanLineHeaderInGroup],
    group_index: usize,
    offset_units: u16,
    extent_units: u16,
) {
    if let Some(run) = shanai_lan_line_header_same_segment_group_run(
        line_headers,
        group_index,
        offset_units,
        extent_units,
    ) {
        push_shanai_lan_line_header_same_segment_group_run_value_json(output, run);
    } else {
        output.push_str("null");
    }
}

pub(crate) fn shanai_lan_line_header_same_segment_group_run(
    line_headers: &[ShanaiLanLineHeaderInGroup],
    group_index: usize,
    offset_units: u16,
    extent_units: u16,
) -> Option<ShanaiLanLineHeaderSameSegmentGroupRun> {
    let groups = line_headers
        .iter()
        .filter(|line_header| {
            line_header.header.offset_units == offset_units
                && line_header.header.extent_units == extent_units
        })
        .map(|line_header| line_header.group_index)
        .collect::<BTreeSet<_>>();
    if !groups.contains(&group_index) {
        return None;
    }
    let mut start_group = group_index;
    while start_group > 0 && groups.contains(&(start_group - 1)) {
        start_group -= 1;
    }
    let mut end_group = group_index;
    while groups.contains(&(end_group + 1)) {
        end_group += 1;
    }
    let group_count = end_group.saturating_sub(start_group) + 1;
    Some(ShanaiLanLineHeaderSameSegmentGroupRun {
        offset_units,
        extent_units,
        start_group_index: start_group,
        end_group_index: end_group,
        group_count,
        position_in_run: group_index.saturating_sub(start_group),
    })
}

pub(crate) fn push_shanai_lan_line_header_same_segment_group_run_value_json(
    output: &mut String,
    run: ShanaiLanLineHeaderSameSegmentGroupRun,
) {
    output.push_str("{\"basis\":\"same-offset-extent-contiguous-groups\"");
    output.push_str(",\"offsetUnits\":");
    output.push_str(&run.offset_units.to_string());
    output.push_str(",\"extentUnits\":");
    output.push_str(&run.extent_units.to_string());
    output.push_str(",\"startGroupIndex\":");
    output.push_str(&run.start_group_index.to_string());
    output.push_str(",\"endGroupIndex\":");
    output.push_str(&run.end_group_index.to_string());
    output.push_str(",\"groupCount\":");
    output.push_str(&run.group_count.to_string());
    output.push_str(",\"positionInRun\":");
    output.push_str(&run.position_in_run.to_string());
    output.push('}');
}

pub(crate) fn push_shanai_lan_line_header_same_segment_group_runs_json(
    output: &mut String,
    entries: &[DocumentTextMapEntry],
    line_headers: &[ShanaiLanLineHeaderInGroup],
    skipped_inline_spans: &[(usize, usize)],
    selected_horizontal_rules: &BTreeSet<(usize, usize, u16, u16)>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
) {
    let mut by_segment = BTreeMap::<(u16, u16), Vec<ShanaiLanLineHeaderInGroup>>::new();
    for line_header in line_headers {
        let header = line_header.header;
        if header.extent_units <= header.offset_units {
            continue;
        }
        if header.extent_units - header.offset_units < SHANAI_LAN_LINE_RULE_MIN_SEGMENT_UNITS {
            continue;
        }
        by_segment
            .entry((header.offset_units, header.extent_units))
            .or_default()
            .push(*line_header);
    }

    let mut first = true;
    output.push('[');
    for ((offset_units, extent_units), mut segment_headers) in by_segment {
        segment_headers
            .sort_by_key(|line_header| (line_header.group_index, line_header.header.start));
        let mut run_start = 0usize;
        while run_start < segment_headers.len() {
            let mut run_end = run_start;
            while run_end + 1 < segment_headers.len()
                && segment_headers[run_end + 1].group_index
                    == segment_headers[run_end].group_index + 1
            {
                run_end += 1;
            }
            if !first {
                output.push(',');
            }
            first = false;
            push_shanai_lan_line_header_same_segment_group_run_summary_json(
                output,
                entries,
                &segment_headers[run_start..=run_end],
                skipped_inline_spans,
                selected_horizontal_rules,
                line_mark_intervals,
                offset_units,
                extent_units,
            );
            run_start = run_end + 1;
        }
    }
    output.push(']');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_shanai_lan_line_header_same_segment_group_run_summary_json(
    output: &mut String,
    entries: &[DocumentTextMapEntry],
    run: &[ShanaiLanLineHeaderInGroup],
    skipped_inline_spans: &[(usize, usize)],
    selected_horizontal_rules: &BTreeSet<(usize, usize, u16, u16)>,
    line_mark_intervals: &[ShanaiLanLineMarkInterval],
    offset_units: u16,
    extent_units: u16,
) {
    let mut selected_horizontal_count = 0usize;
    let mut skipped_inline_count = 0usize;
    let mut no_containing_map_entry_count = 0usize;
    let mut text_run_containing_count = 0usize;
    let mut containing_entry_kind_counts = BTreeMap::<&'static str, usize>::new();
    let mut line_mark_flag_counts = BTreeMap::<String, usize>::new();

    for line_header in run {
        let header = line_header.header;
        if selected_horizontal_rules.contains(&(
            header.start,
            line_header.group_index,
            header.offset_units,
            header.extent_units,
        )) {
            selected_horizontal_count += 1;
        }
        if skipped_inline_spans
            .iter()
            .any(|(start, end)| *start <= header.start && header.end <= *end)
        {
            skipped_inline_count += 1;
        }
        match entries
            .iter()
            .find(|entry| entry.byte_start() <= header.start && header.end <= entry.byte_end())
        {
            Some(entry) => {
                *containing_entry_kind_counts
                    .entry(entry.kind().as_str())
                    .or_default() += 1;
                if entry.kind() == DocumentTextMapKind::TextRun {
                    text_run_containing_count += 1;
                }
            }
            None => no_containing_map_entry_count += 1,
        }
        if let Some(line_mark) = shanai_lan_line_mark_for_header(line_mark_intervals, &header) {
            *line_mark_flag_counts
                .entry(format!("0x{:04x}", line_mark.flag_word))
                .or_default() += 1;
        }
    }

    let start_group = run
        .first()
        .map(|line_header| line_header.group_index)
        .unwrap_or_default();
    let end_group = run
        .last()
        .map(|line_header| line_header.group_index)
        .unwrap_or(start_group);

    output.push_str("{\"basis\":\"same-offset-extent-contiguous-groups\"");
    output.push_str(",\"offsetUnits\":");
    output.push_str(&offset_units.to_string());
    output.push_str(",\"extentUnits\":");
    output.push_str(&extent_units.to_string());
    output.push_str(",\"segmentUnits\":");
    output.push_str(&extent_units.saturating_sub(offset_units).to_string());
    output.push_str(",\"startGroupIndex\":");
    output.push_str(&start_group.to_string());
    output.push_str(",\"endGroupIndex\":");
    output.push_str(&end_group.to_string());
    output.push_str(",\"groupCount\":");
    output.push_str(&run.len().to_string());
    output.push_str(",\"selectedHorizontalRuleCount\":");
    output.push_str(&selected_horizontal_count.to_string());
    output.push_str(",\"skippedInlineCount\":");
    output.push_str(&skipped_inline_count.to_string());
    output.push_str(",\"noContainingMapEntryCount\":");
    output.push_str(&no_containing_map_entry_count.to_string());
    output.push_str(",\"textRunContainingCount\":");
    output.push_str(&text_run_containing_count.to_string());
    output.push_str(",\"containingEntryKindCounts\":");
    push_static_str_count_map_json(output, &containing_entry_kind_counts);
    output.push_str(",\"lineMarkFlagCounts\":");
    push_string_count_map_json(output, &line_mark_flag_counts, "flagWordHex");
    output.push_str(",\"renderable\":false,\"renderPromotionBlockedReason\":\"line-header-run-visibility-selector-unproven\"}");
}

pub(crate) fn shanai_lan_line_rule_endpoint_attaches_to_text(
    x: f32,
    y: f32,
    projection: &ShanaiLanLineRuleProjection,
    text_projection: Option<&ShanaiLanTextProjection>,
) -> bool {
    shanai_lan_nearest_text_slot_attachment(text_projection, x, y)
        .is_some_and(|(_, distance_px, _)| distance_px <= projection.line_height_px)
}

pub(crate) fn push_shanai_lan_line_rule_graph_components_json(
    output: &mut String,
    components: &[ShanaiLanLineRuleGraphComponentSummary],
) {
    output.push('[');
    for (component_index, component) in components.iter().enumerate() {
        if component_index > 0 {
            output.push(',');
        }
        output.push_str("{\"componentIndex\":");
        output.push_str(&component_index.to_string());
        output.push_str(",\"ruleIndexes\":");
        push_usize_array_json(output, &component.rule_indexes);
        output.push_str(",\"bbox\":");
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
            component.bbox.0, component.bbox.1, component.bbox.2, component.bbox.3
        ));
        output.push_str(",\"ruleCount\":");
        output.push_str(&component.rule_indexes.len().to_string());
        output.push_str(",\"horizontalRuleCount\":");
        output.push_str(&component.horizontal_rule_count.to_string());
        output.push_str(",\"verticalRuleCount\":");
        output.push_str(&component.vertical_rule_count.to_string());
        output.push_str(",\"orthogonalGraphRuleCount\":");
        output.push_str(&component.orthogonal_graph_rule_count.to_string());
        output.push_str(",\"lineMarkMatchedRuleCount\":");
        output.push_str(&component.line_mark_matched_rule_count.to_string());
        output.push_str(",\"endpointCount\":");
        output.push_str(&(component.rule_indexes.len() * 2).to_string());
        output.push_str(",\"isolatedEndpointCount\":");
        output.push_str(&component.isolated_endpoint_count.to_string());
        output.push_str(",\"totalProjectedLengthPx\":");
        output.push_str(&format!("{:.3}", component.total_projected_length_px));
        output.push_str(",\"orthogonalComponentCandidate\":");
        output.push_str(json_bool(
            shanai_lan_line_rule_component_orthogonal_candidate(component),
        ));
        output.push_str(",\"lineMarkCoverageComplete\":");
        output.push_str(json_bool(
            component.line_mark_matched_rule_count == component.rule_indexes.len(),
        ));
        output.push_str(",\"renderAdmissionGate\":");
        push_shanai_lan_line_rule_component_render_admission_gate_json(output, component);
        output.push_str(",\"renderable\":false,\"renderPromotionBlockedReason\":\"line-rule-component-placement-and-style-unproven\"}");
    }
    output.push(']');
}

pub(crate) fn push_shanai_lan_line_rule_projection_render_admission_gate_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    components: &[ShanaiLanLineRuleGraphComponentSummary],
    orthogonal_graph_candidate_count: usize,
    no_isolated_endpoint_rule_count: usize,
    line_mark_matched_rule_count: usize,
    both_endpoint_attachment_within_line_height_rule_count: usize,
) {
    let orthogonal_component_count = components
        .iter()
        .filter(|component| shanai_lan_line_rule_component_orthogonal_candidate(component))
        .count();
    let line_mark_coverage_complete = line_mark_matched_rule_count == projection.rules.len();
    let has_endpoint_attachment_pair = both_endpoint_attachment_within_line_height_rule_count > 0;
    let mut blocked_reasons = Vec::new();
    blocked_reasons.push("document-text-grid-origin-reference-backed");
    if orthogonal_graph_candidate_count < projection.rules.len() {
        blocked_reasons.push("line-rule-topology-partial-orthogonal-coverage");
    }
    if orthogonal_component_count < components.len() {
        blocked_reasons.push("line-rule-component-topology-unproven");
    }
    if !line_mark_coverage_complete {
        blocked_reasons.push("line-rule-line-mark-coverage-incomplete");
    }
    if no_isolated_endpoint_rule_count < projection.rules.len() {
        blocked_reasons.push("line-rule-endpoint-ownership-unproven");
    }
    if !has_endpoint_attachment_pair {
        blocked_reasons.push("line-rule-text-attachment-pair-absent");
    }
    blocked_reasons.push("line-rule-style-role-unproven");
    blocked_reasons.push("line-rule-paint-order-unproven");

    output.push_str("{\"source\":\"/DocumentText+/LineMark line-rule render admission\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementDerived\":false,\"renderable\":false,\"promotionReady\":false");
    output.push_str(",\"ruleCount\":");
    output.push_str(&projection.rules.len().to_string());
    output.push_str(",\"componentCount\":");
    output.push_str(&components.len().to_string());
    output.push_str(",\"orthogonalGraphCandidateRuleCount\":");
    output.push_str(&orthogonal_graph_candidate_count.to_string());
    output.push_str(",\"orthogonalComponentCandidateCount\":");
    output.push_str(&orthogonal_component_count.to_string());
    output.push_str(",\"lineMarkCoverageComplete\":");
    output.push_str(json_bool(line_mark_coverage_complete));
    output.push_str(",\"noIsolatedEndpointRuleCount\":");
    output.push_str(&no_isolated_endpoint_rule_count.to_string());
    output.push_str(",\"bothEndpointAttachmentWithinLineHeightRuleCount\":");
    output.push_str(&both_endpoint_attachment_within_line_height_rule_count.to_string());
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionBlockedReason\":\"line-rule-render-admission-not-ready\"}");
}

pub(crate) fn push_shanai_lan_line_rule_component_render_admission_gate_json(
    output: &mut String,
    component: &ShanaiLanLineRuleGraphComponentSummary,
) {
    let orthogonal_component_candidate =
        shanai_lan_line_rule_component_orthogonal_candidate(component);
    let line_mark_coverage_complete =
        component.line_mark_matched_rule_count == component.rule_indexes.len();
    let mut blocked_reasons = Vec::new();
    blocked_reasons.push("document-text-grid-origin-reference-backed");
    if !orthogonal_component_candidate {
        blocked_reasons.push("line-rule-component-topology-unproven");
    }
    if component.isolated_endpoint_count > 0 {
        blocked_reasons.push("line-rule-component-endpoint-ownership-unproven");
    }
    if !line_mark_coverage_complete {
        blocked_reasons.push("line-rule-component-line-mark-coverage-incomplete");
    }
    blocked_reasons.push("line-rule-component-style-role-unproven");
    blocked_reasons.push("line-rule-paint-order-unproven");

    output.push_str("{\"source\":\"/DocumentText+/LineMark line-rule component render admission\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementDerived\":false,\"renderable\":false,\"promotionReady\":false");
    output.push_str(",\"orthogonalComponentCandidate\":");
    output.push_str(json_bool(orthogonal_component_candidate));
    output.push_str(",\"lineMarkCoverageComplete\":");
    output.push_str(json_bool(line_mark_coverage_complete));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"line-rule-component-render-admission-not-ready\"}",
    );
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_shanai_lan_line_rule_render_admission_gate_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    rule_index: usize,
    rule: &ShanaiLanLineRule,
    topology: ShanaiLanLineRuleTopology,
    component: Option<&(usize, ShanaiLanLineRuleGraphComponentSummary)>,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    let start_attached = shanai_lan_line_rule_endpoint_attaches_to_text(
        rule.x1,
        rule.y1,
        projection,
        text_projection,
    );
    let end_attached = shanai_lan_line_rule_endpoint_attaches_to_text(
        rule.x2,
        rule.y2,
        projection,
        text_projection,
    );
    let component_candidate = component
        .map(|(_, component)| shanai_lan_line_rule_component_orthogonal_candidate(component))
        .unwrap_or(false);
    let component_index = component.map(|(component_index, _)| *component_index);
    let component_rule_count = component.map(|(_, component)| component.rule_indexes.len());
    let has_line_mark = rule.line_mark.is_some();
    let mut blocked_reasons = Vec::new();
    blocked_reasons.push("document-text-grid-origin-reference-backed");
    if !topology.orthogonal_graph_candidate {
        blocked_reasons.push("line-rule-topology-not-orthogonal-network");
    }
    if topology.isolated_endpoint_count > 0 {
        blocked_reasons.push("line-rule-endpoint-ownership-unproven");
    }
    if !start_attached || !end_attached {
        blocked_reasons.push("line-rule-text-attachment-pair-unproven");
    }
    if !has_line_mark {
        blocked_reasons.push("line-rule-line-mark-record-missing");
    }
    if !component_candidate {
        blocked_reasons.push("line-rule-component-topology-unproven");
    }
    blocked_reasons.push("line-rule-style-role-unproven");
    blocked_reasons.push("line-rule-paint-order-unproven");

    output.push_str("{\"source\":\"/DocumentText+/LineMark line-rule render admission\"");
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":true,\"decoded\":false,\"geometryDecoded\":true,\"placementDerived\":false,\"renderable\":false,\"promotionReady\":false");
    output.push_str(",\"ruleIndex\":");
    output.push_str(&rule_index.to_string());
    output.push_str(",\"componentIndex\":");
    push_option_usize_json(output, component_index);
    output.push_str(",\"componentRuleCount\":");
    push_option_usize_json(output, component_rule_count);
    output.push_str(",\"lineMarkMatched\":");
    output.push_str(json_bool(has_line_mark));
    output.push_str(",\"orthogonalGraphCandidate\":");
    output.push_str(json_bool(topology.orthogonal_graph_candidate));
    output.push_str(",\"componentOrthogonalCandidate\":");
    output.push_str(json_bool(component_candidate));
    output.push_str(",\"startEndpointTextAttachmentCandidate\":");
    output.push_str(json_bool(start_attached));
    output.push_str(",\"endEndpointTextAttachmentCandidate\":");
    output.push_str(json_bool(end_attached));
    output.push_str(",\"bothEndpointTextAttachmentCandidate\":");
    output.push_str(json_bool(start_attached && end_attached));
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(",\"renderPromotionBlockedReason\":\"line-rule-render-admission-not-ready\"}");
}
