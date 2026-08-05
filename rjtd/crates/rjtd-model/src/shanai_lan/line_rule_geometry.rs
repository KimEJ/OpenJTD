use crate::*;

pub(crate) fn shanai_lan_line_rule_component_for_rule(
    projection: &ShanaiLanLineRuleProjection,
    rule_index: usize,
) -> Option<(usize, ShanaiLanLineRuleGraphComponentSummary)> {
    shanai_lan_line_rule_graph_component_summaries(projection)
        .into_iter()
        .enumerate()
        .find(|(_, component)| component.rule_indexes.contains(&rule_index))
}

pub(crate) fn shanai_lan_line_rule_component_orthogonal_candidate(
    component: &ShanaiLanLineRuleGraphComponentSummary,
) -> bool {
    !component.rule_indexes.is_empty()
        && component.horizontal_rule_count > 0
        && component.vertical_rule_count > 0
        && component.orthogonal_graph_rule_count == component.rule_indexes.len()
        && component.line_mark_matched_rule_count == component.rule_indexes.len()
}

pub(crate) fn shanai_lan_line_rule_topology(
    projection: &ShanaiLanLineRuleProjection,
    rule: &ShanaiLanLineRule,
) -> ShanaiLanLineRuleTopology {
    let start_junction_degree =
        shanai_lan_line_rule_junction_degree(&projection.rules, rule.x1, rule.y1);
    let end_junction_degree =
        shanai_lan_line_rule_junction_degree(&projection.rules, rule.x2, rule.y2);
    let isolated_endpoint_count =
        usize::from(start_junction_degree <= 1) + usize::from(end_junction_degree <= 1);
    ShanaiLanLineRuleTopology {
        start_junction_degree,
        end_junction_degree,
        isolated_endpoint_count,
        orthogonal_graph_candidate: matches!(rule.orientation, "horizontal" | "vertical")
            && isolated_endpoint_count < 2,
    }
}

pub(crate) fn shanai_lan_line_rule_graph_component_summaries(
    projection: &ShanaiLanLineRuleProjection,
) -> Vec<ShanaiLanLineRuleGraphComponentSummary> {
    let mut adjacency = vec![Vec::<usize>::new(); projection.rules.len()];
    for left_index in 0..projection.rules.len() {
        for right_index in (left_index + 1)..projection.rules.len() {
            if shanai_lan_line_rules_touch(
                &projection.rules[left_index],
                &projection.rules[right_index],
            ) {
                adjacency[left_index].push(right_index);
                adjacency[right_index].push(left_index);
            }
        }
    }

    let mut seen = vec![false; projection.rules.len()];
    let mut components = Vec::new();
    for start_index in 0..projection.rules.len() {
        if seen[start_index] {
            continue;
        }
        let mut stack = vec![start_index];
        seen[start_index] = true;
        let mut rule_indexes = Vec::new();
        while let Some(index) = stack.pop() {
            rule_indexes.push(index);
            for neighbor in adjacency[index].iter().copied() {
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
        rule_indexes.sort_unstable();
        components.push(shanai_lan_line_rule_graph_component_summary(
            projection,
            rule_indexes,
        ));
    }
    components
}

pub(crate) fn shanai_lan_line_rule_graph_component_summary(
    projection: &ShanaiLanLineRuleProjection,
    rule_indexes: Vec<usize>,
) -> ShanaiLanLineRuleGraphComponentSummary {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut horizontal_rule_count = 0usize;
    let mut vertical_rule_count = 0usize;
    let mut orthogonal_graph_rule_count = 0usize;
    let mut line_mark_matched_rule_count = 0usize;
    let mut isolated_endpoint_count = 0usize;
    let mut total_projected_length_px = 0.0f32;

    for rule_index in rule_indexes.iter().copied() {
        let rule = &projection.rules[rule_index];
        let (x, y, width, height) = shanai_lan_line_rule_bbox(projection, rule);
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
        match rule.orientation {
            "horizontal" => horizontal_rule_count += 1,
            "vertical" => vertical_rule_count += 1,
            _ => {}
        }
        if rule.line_mark.is_some() {
            line_mark_matched_rule_count += 1;
        }
        let topology = shanai_lan_line_rule_topology(projection, rule);
        if topology.orthogonal_graph_candidate {
            orthogonal_graph_rule_count += 1;
        }
        isolated_endpoint_count += topology.isolated_endpoint_count;
        total_projected_length_px += (rule.x2 - rule.x1).abs() + (rule.y2 - rule.y1).abs();
    }

    ShanaiLanLineRuleGraphComponentSummary {
        rule_indexes,
        bbox: (min_x, min_y, max_x - min_x, max_y - min_y),
        horizontal_rule_count,
        vertical_rule_count,
        orthogonal_graph_rule_count,
        line_mark_matched_rule_count,
        isolated_endpoint_count,
        total_projected_length_px,
    }
}

pub(crate) fn shanai_lan_line_rules_touch(
    left: &ShanaiLanLineRule,
    right: &ShanaiLanLineRule,
) -> bool {
    let (left_start, left_end) = shanai_lan_line_rule_endpoints(left);
    let (right_start, right_end) = shanai_lan_line_rule_endpoints(right);
    shanai_lan_line_rule_contains_point(right, left_start.0, left_start.1)
        || shanai_lan_line_rule_contains_point(right, left_end.0, left_end.1)
        || shanai_lan_line_rule_contains_point(left, right_start.0, right_start.1)
        || shanai_lan_line_rule_contains_point(left, right_end.0, right_end.1)
}

pub(crate) fn shanai_lan_line_rule_endpoints(rule: &ShanaiLanLineRule) -> ((f32, f32), (f32, f32)) {
    ((rule.x1, rule.y1), (rule.x2, rule.y2))
}

pub(crate) fn shanai_lan_line_rule_bbox(
    projection: &ShanaiLanLineRuleProjection,
    rule: &ShanaiLanLineRule,
) -> (f32, f32, f32, f32) {
    let x = rule.x1.min(rule.x2) - projection.stroke_width * 0.5;
    let y = rule.y1.min(rule.y2) - projection.stroke_width * 0.5;
    let width = (rule.x2 - rule.x1).abs() + projection.stroke_width;
    let height = (rule.y2 - rule.y1).abs() + projection.stroke_width;
    (x, y, width, height)
}

pub(crate) fn shanai_lan_line_rule_junction_degree(
    rules: &[ShanaiLanLineRule],
    x: f32,
    y: f32,
) -> usize {
    rules
        .iter()
        .filter(|rule| shanai_lan_line_rule_contains_point(rule, x, y))
        .count()
}

pub(crate) fn shanai_lan_line_rule_contains_point(
    rule: &ShanaiLanLineRule,
    x: f32,
    y: f32,
) -> bool {
    const EPSILON: f32 = 0.75;
    let min_x = rule.x1.min(rule.x2) - EPSILON;
    let max_x = rule.x1.max(rule.x2) + EPSILON;
    let min_y = rule.y1.min(rule.y2) - EPSILON;
    let max_y = rule.y1.max(rule.y2) + EPSILON;
    if !(min_x..=max_x).contains(&x) || !(min_y..=max_y).contains(&y) {
        return false;
    }
    match rule.orientation {
        "horizontal" => (rule.y1 - y).abs() <= EPSILON,
        "vertical" => (rule.x1 - x).abs() <= EPSILON,
        _ => false,
    }
}

pub(crate) fn push_shanai_lan_line_rule_endpoint_attachment_candidates_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    rule: &ShanaiLanLineRule,
    topology: ShanaiLanLineRuleTopology,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    output.push_str("{\"start\":");
    push_shanai_lan_line_rule_endpoint_attachment_candidate_json(
        output,
        projection,
        rule.x1,
        rule.y1,
        topology.start_junction_degree,
        text_projection,
    );
    output.push_str(",\"end\":");
    push_shanai_lan_line_rule_endpoint_attachment_candidate_json(
        output,
        projection,
        rule.x2,
        rule.y2,
        topology.end_junction_degree,
        text_projection,
    );
    output.push('}');
}

pub(crate) fn push_shanai_lan_line_rule_endpoint_attachment_candidate_json(
    output: &mut String,
    projection: &ShanaiLanLineRuleProjection,
    x: f32,
    y: f32,
    junction_degree: usize,
    text_projection: Option<&ShanaiLanTextProjection>,
) {
    output.push_str("{\"point\":");
    output.push_str(&format!("{{\"x\":{x:.3},\"y\":{y:.3}}}"));
    output.push_str(",\"junctionDegree\":");
    output.push_str(&junction_degree.to_string());
    output.push_str(",\"attachmentProven\":false,\"nearestTextSlot\":");
    if let Some((slot, distance_px, bbox)) =
        shanai_lan_nearest_text_slot_attachment(text_projection, x, y)
    {
        output.push_str("{\"text\":");
        output.push_str(&json_string(&slot.text));
        output.push_str(",\"distancePx\":");
        output.push_str(&format!("{distance_px:.3}"));
        output.push_str(",\"probeRadiusPx\":");
        output.push_str(&format!("{:.3}", projection.line_height_px));
        output.push_str(",\"withinLineHeight\":");
        output.push_str(if distance_px <= projection.line_height_px {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"bbox\":");
        output.push_str(&format!(
            "{{\"x\":{:.3},\"y\":{:.3},\"width\":{:.3},\"height\":{:.3}}}",
            bbox.0, bbox.1, bbox.2, bbox.3
        ));
        output.push_str(",\"groupIndex\":");
        match slot.group_index {
            Some(group_index) => output.push_str(&group_index.to_string()),
            None => output.push_str("null"),
        }
        output.push_str(",\"sourceByteRange\":");
        output.push_str(&source_range_json(
            slot.source_span.byte_start(),
            slot.source_span.byte_end(),
        ));
        output.push_str(",\"sourceUnitRange\":");
        output.push_str(&source_range_json(
            slot.source_span.unit_start(),
            slot.source_span.unit_end(),
        ));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn shanai_lan_nearest_text_slot_attachment<'a>(
    text_projection: Option<&'a ShanaiLanTextProjection>,
    x: f32,
    y: f32,
) -> Option<ShanaiLanTextSlotAttachment<'a>> {
    text_projection?
        .slots
        .iter()
        .map(|slot| {
            let bbox = shanai_lan_text_slot_bbox(slot);
            let distance_px = distance_from_point_to_bbox(x, y, bbox);
            (slot, distance_px, bbox)
        })
        .min_by(|left, right| left.1.partial_cmp(&right.1).unwrap_or(Ordering::Equal))
}

pub(crate) fn shanai_lan_text_slot_bbox(slot: &ShanaiLanTextSlot) -> (f32, f32, f32, f32) {
    let text_width =
        text_width_px_for_font_size(slot.font_size, &slot.text).max(f64::from(slot.font_size));
    (
        slot.x,
        slot.y,
        text_width as f32,
        slot.font_size * SHANAI_LAN_TEXT_GROUP_LINE_HEIGHT_FACTOR,
    )
}

pub(crate) fn shanai_lan_text_style_link_evidence(
    document: &Document,
    document_text_bytes: &[u8],
    text_entry: &DocumentTextMapEntry,
    text_count_range_evidence: &[ShanaiLanTextCountRangeEvidence],
) -> ShanaiLanTextStyleLinkEvidence {
    let text_layout_style_record_count =
        style_stream_record_count(document, TEXT_LAYOUT_STYLE_PATH);
    let document_view_style_group_count = document_view_style_group_count(document);
    let document_view_style_group_candidate =
        shanai_lan_document_view_style_group_candidate(text_count_range_evidence);
    let document_view_style_group_candidate_basis =
        document_view_style_group_candidate.map(|_| "document-text-position-count-tail-field-f7");
    let document_text_group_header_candidate =
        shanai_lan_document_text_group_header_candidate(document_text_bytes, text_entry);
    let document_text_inline_style_candidate =
        shanai_lan_document_text_inline_style_candidate(document_text_bytes, text_entry);
    ShanaiLanTextStyleLinkEvidence {
        source: "DocumentText+DocumentTextPositionTables+DocumentViewStyles",
        style_link_proven: false,
        text_layout_style_record_count,
        document_view_style_group_count,
        document_view_style_group_candidate,
        document_view_style_group_candidate_basis,
        document_text_group_header_candidate,
        document_text_inline_style_candidate,
        style_link_promotion_blocked_reason: "document-view-style-group-link-unproven",
        fill_color_promotion_blocked_reason: None,
    }
}

pub(crate) fn shanai_lan_document_text_group_header_candidate(
    bytes: &[u8],
    entry: &DocumentTextMapEntry,
) -> Option<ShanaiLanDocumentTextGroupHeaderCandidate> {
    let units = document_text_units(bytes);
    let header_start = (0..entry.unit_start()).rev().find(|index| {
        units.get(*index).copied() == Some(0x001c) && units.get(index + 1).copied() == Some(0x0010)
    })?;
    let text_marker_index = (header_start + 2..entry.unit_start())
        .find(|index| units.get(*index).copied() == Some(DOCUMENT_TEXT_TEXT_RUN_MARKER))?;
    let raw_words = units.get(header_start..=text_marker_index)?.to_vec();
    if raw_words.len() < 3 || raw_words.len() > 64 {
        return None;
    }
    let field_words = raw_words
        .get(2..raw_words.len().saturating_sub(1))
        .unwrap_or_default()
        .to_vec();
    let source_span = TextSourceSpan::new(
        header_start * 2,
        (text_marker_index + 1) * 2,
        header_start,
        text_marker_index + 1,
    );
    Some(ShanaiLanDocumentTextGroupHeaderCandidate {
        source_span,
        raw_words,
        field_words,
        distance_to_text_units: entry.unit_start().saturating_sub(text_marker_index + 1),
        promotion_blocked_reason: "document-text-group-header-semantics-unproven",
    })
}

pub(crate) fn shanai_lan_document_text_inline_style_candidate(
    bytes: &[u8],
    entry: &DocumentTextMapEntry,
) -> Option<ShanaiLanDocumentTextInlineStyleCandidate> {
    let units = document_text_units(bytes);
    let text_marker_index = entry.unit_start().checked_sub(1)?;
    if units.get(text_marker_index).copied() != Some(DOCUMENT_TEXT_TEXT_RUN_MARKER) {
        return None;
    }

    let search_start = text_marker_index.saturating_sub(32);
    let inline_end = (search_start..text_marker_index)
        .rev()
        .find(|index| units.get(*index).copied() == Some(0x001e))?;
    let inline_start = (search_start..inline_end)
        .rev()
        .find(|index| units.get(*index).copied() == Some(DOCUMENT_TEXT_INLINE_START_TAG as u16))?;
    let context_start = inline_start.checked_sub(6)?;
    let context_words = units.get(context_start..inline_start)?.to_vec();
    if !shanai_lan_document_text_inline_style_context(&context_words) {
        return None;
    }
    let selector = context_words.get(5).copied();
    let payload_words = units.get(inline_start + 1..inline_end)?.to_vec();
    let post_inline_words = units.get(inline_end + 1..=text_marker_index)?.to_vec();
    let raw_words = units.get(context_start..=text_marker_index)?.to_vec();
    let source_span = TextSourceSpan::new(
        context_start * 2,
        (text_marker_index + 1) * 2,
        context_start,
        text_marker_index + 1,
    );
    let distance_to_text_units = entry.unit_start().saturating_sub(inline_end + 1);

    Some(ShanaiLanDocumentTextInlineStyleCandidate {
        source_span,
        selector,
        context_words,
        payload_words,
        post_inline_words,
        raw_words,
        distance_to_text_units,
        promotion_blocked_reason: "document-text-inline-control-semantics-unproven",
    })
}

pub(crate) fn shanai_lan_document_text_inline_style_context(context_words: &[u16]) -> bool {
    context_words.len() == 6
        && context_words[0] == 0x001c
        && context_words[1] == 0x0001
        && context_words[2] == 0x0007
        && context_words[3] == 0x0000
}

pub(crate) fn shanai_lan_document_view_style_group_candidate(
    evidence: &[ShanaiLanTextCountRangeEvidence],
) -> Option<u16> {
    let mut candidate = None;
    for item in evidence {
        let Some(group_id) = item.tail_fields.get(7).copied() else {
            continue;
        };
        if !(1..=9).contains(&group_id) {
            continue;
        }
        match candidate {
            Some(existing) if existing != group_id => return None,
            Some(_) => {}
            None => candidate = Some(group_id),
        }
    }
    candidate
}

pub(crate) fn shanai_lan_text_count_range_evidence(
    document: &Document,
    span: &TextSourceSpan,
) -> Vec<ShanaiLanTextCountRangeEvidence> {
    let mut evidence = Vec::new();
    for range in document.text_count_ranges() {
        push_shanai_lan_text_count_range_evidence(
            &mut evidence,
            range,
            TextCountRangeOverlapBasis::Byte,
            span.byte_start(),
            span.byte_end(),
        );
        push_shanai_lan_text_count_range_evidence(
            &mut evidence,
            range,
            TextCountRangeOverlapBasis::Unit,
            span.unit_start(),
            span.unit_end(),
        );
    }
    evidence
}

pub(crate) fn push_shanai_lan_text_count_range_evidence(
    evidence: &mut Vec<ShanaiLanTextCountRangeEvidence>,
    range: &TextCountRange,
    basis: TextCountRangeOverlapBasis,
    span_start: usize,
    span_end: usize,
) {
    let range_start = range.start() as usize;
    let range_end = range.end() as usize;
    let overlap_start = span_start.max(range_start);
    let overlap_end = span_end.min(range_end);
    if overlap_start >= overlap_end {
        return;
    }
    evidence.push(ShanaiLanTextCountRangeEvidence {
        index: range.index(),
        family: range.family().to_string(),
        basis,
        range_start,
        range_end,
        overlap_start,
        overlap_end,
        declared_start: range.declared_start(),
        declared_end: range.declared_end(),
        tail_fields: range.tail_fields().to_vec(),
    });
}
