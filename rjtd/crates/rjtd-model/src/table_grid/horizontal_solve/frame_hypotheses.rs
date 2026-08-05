use super::*;
use crate::*;

pub(crate) fn push_table_grid_source_top_text_placement_coherence_mirror_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    candidate: &TableCandidate,
) {
    let readiness =
        table_grid_source_top_text_placement_readiness_for_candidate(layout, document, candidate);
    output.push_str(
        "{\"source\":\"topTextTableSourceGapEvidence.sourceTablePlacementCoherenceGate\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"sourceTopTextPlacementEvidencePresent\":");
    output.push_str(if readiness.is_some() { "true" } else { "false" });
    output.push_str(",\"sourceTopTextPlacementReady\":");
    output.push_str(
        if readiness.as_ref().is_some_and(|readiness| readiness.ready) {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"readinessBlockedReasons\":");
    match readiness.as_ref() {
        Some(readiness) => push_json_string_slice_array(output, &readiness.blocked_reasons),
        None => {
            push_json_string_slice_array(output, &["source-top-text-placement-evidence-absent"])
        }
    }
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-frame-top-text-placement-coherence\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    match readiness
        .as_ref()
        .and_then(TableGridSourceTopTextPlacementReadiness::blocked_reason)
    {
        Some(reason) => output.push_str(&json_string(reason)),
        None if readiness.as_ref().is_some_and(|readiness| readiness.ready) => {
            output.push_str("null")
        }
        None => output.push_str(&json_string("source-top-text-placement-evidence-absent")),
    }
    output.push('}');
}

pub(crate) fn table_grid_page_mark_horizontal_best_agreement_group(
    supports: &[TableGridHorizontalFrameCandidateSupport],
) -> Option<Vec<TableGridHorizontalFrameCandidateSupport>> {
    let mut groups: BTreeMap<(i32, i32), Vec<TableGridHorizontalFrameCandidateSupport>> =
        BTreeMap::new();
    for support in supports
        .iter()
        .filter(|support| support.frame_basis.starts_with("page-mark"))
    {
        groups
            .entry((
                rounded_milli(support.selected_x),
                rounded_milli(support.selected_width),
            ))
            .or_default()
            .push(support.clone());
    }
    let best_support_count = groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let best_group_count = groups
        .values()
        .filter(|supports| supports.len() == best_support_count)
        .count();
    (best_support_count > 1 && best_group_count == 1)
        .then(|| {
            groups
                .values()
                .find(|supports| supports.len() == best_support_count)
                .cloned()
        })
        .flatten()
}

pub(crate) fn push_table_grid_page_space_horizontal_frame_hypotheses_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    output.push('[');
    let mut emitted = false;
    if let Some(source_layout) = source_layout {
        let selected_start = f32::from(source_layout.x_unit_start);
        let selected_width = f32::from(
            source_layout
                .x_unit_end
                .saturating_sub(source_layout.x_unit_start),
        );
        let full_extent = f32::from(source_layout.x_unit_full_extent_units);
        if full_extent > 0.0 {
            push_table_grid_page_space_horizontal_frame_hypothesis_json(
                output,
                "page-body-frame",
                layout.margin_px(),
                layout.body_width_px(),
                selected_start,
                selected_width,
                full_extent,
                "page-body-frame-not-proven-for-table",
            );
            output.push(',');
            push_table_grid_page_space_horizontal_frame_hypothesis_json(
                output,
                "page-media-box",
                0.0,
                layout.width_px(),
                selected_start,
                selected_width,
                full_extent,
                "page-media-box-not-proven-for-table",
            );
            emitted = true;
            push_table_grid_page_space_horizontal_page_mark_raw_field_hypotheses_json(
                output,
                source_layout,
            );
        }
    }
    push_table_grid_page_space_horizontal_source_only_consensus_frame_hypotheses_json(
        output,
        layout,
        document,
        lines,
        candidate,
        &mut emitted,
    );
    output.push(']');
}

pub(crate) fn push_table_grid_page_space_horizontal_frame_candidate_agreement_gate_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) {
    let supports = table_grid_page_space_horizontal_frame_candidate_supports(
        layout,
        document,
        lines,
        candidate,
        source_layout,
    );
    let mut groups: BTreeMap<(i32, i32), Vec<TableGridHorizontalFrameCandidateSupport>> =
        BTreeMap::new();
    for support in supports {
        groups
            .entry((
                rounded_milli(support.selected_x),
                rounded_milli(support.selected_width),
            ))
            .or_default()
            .push(support);
    }
    let best_support_count = groups
        .values()
        .map(|supports| supports.len())
        .max()
        .unwrap_or(0);
    let best_group_count = groups
        .values()
        .filter(|supports| supports.len() == best_support_count)
        .count();
    let unique_best_supported = best_support_count > 1 && best_group_count == 1;
    let best_group = groups
        .values()
        .find(|supports| supports.len() == best_support_count && unique_best_supported);
    let source_only_selector_in_best_group = best_group.is_some_and(|supports| {
        supports
            .iter()
            .any(|support| support.contribution == "source-only-horizontal-field-selector")
    });
    let source_only_unique_selection_candidate_present =
        unique_best_supported && source_only_selector_in_best_group;
    let selection_ready = false;
    let mut blocked_reasons = Vec::new();
    if groups.is_empty() {
        blocked_reasons.push("source-horizontal-frame-candidates-absent");
    }
    if best_support_count <= 1 {
        blocked_reasons.push("source-horizontal-frame-candidate-agreement-missing");
    }
    if best_group_count > 1 {
        blocked_reasons.push("source-horizontal-frame-candidate-agreement-ambiguous");
    }
    if unique_best_supported {
        blocked_reasons.push("source-horizontal-field-semantics-still-unproven");
    }

    output.push_str(
        "{\"source\":\"pageSpaceHorizontalTransformGate.sourceFrameHypotheses agreement\"",
    );
    output.push_str(",\"diagnosticOnly\":true,\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"selectionReady\":");
    output.push_str(if selection_ready { "true" } else { "false" });
    output.push_str(",\"candidateCount\":");
    output.push_str(
        &groups
            .values()
            .map(|supports| supports.len())
            .sum::<usize>()
            .to_string(),
    );
    output.push_str(",\"agreementGroupCount\":");
    output.push_str(&groups.len().to_string());
    output.push_str(",\"bestSupportCount\":");
    output.push_str(&best_support_count.to_string());
    output.push_str(",\"uniqueBestSupported\":");
    output.push_str(if unique_best_supported {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyUniqueSelectionCandidatePresent\":");
    output.push_str(if source_only_unique_selection_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOnlyUniqueSelectionDiagnosticOnly\":true");
    output.push_str(",\"sourceOnlyUniqueSelectionPromotionReady\":false");
    output.push_str(",\"sourceOnlyUniqueSelectionPromotionBlockedReason\":");
    if source_only_unique_selection_candidate_present {
        output.push_str(&json_string("source-horizontal-field-semantics-unproven"));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"bestSupportedSelectedX\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_x)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedSelectedWidth\":");
    match best_group.and_then(|supports| supports.first()) {
        Some(support) => output.push_str(&format!("{:.3}", support.selected_width)),
        None => output.push_str("null"),
    }
    output.push_str(",\"bestSupportedFrameBases\":");
    match best_group {
        Some(supports) => {
            let frame_bases = supports
                .iter()
                .map(|support| support.frame_basis)
                .collect::<Vec<_>>();
            push_json_string_slice_array(output, &frame_bases);
        }
        None => output.push_str("[]"),
    }
    output.push_str(",\"agreementGroups\":[");
    for (index, supports) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let first = supports.first().unwrap();
        output.push_str("{\"selectedX\":");
        output.push_str(&format!("{:.3}", first.selected_x));
        output.push_str(",\"selectedWidth\":");
        output.push_str(&format!("{:.3}", first.selected_width));
        output.push_str(",\"supportCount\":");
        output.push_str(&supports.len().to_string());
        output.push_str(",\"frameBases\":");
        let frame_bases = supports
            .iter()
            .map(|support| support.frame_basis)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &frame_bases);
        output.push_str(",\"contributions\":");
        let contributions = supports
            .iter()
            .map(|support| support.contribution)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &contributions);
        output.push_str(",\"blockedReasons\":");
        let blocked = supports
            .iter()
            .map(|support| support.blocked_reason)
            .collect::<Vec<_>>();
        push_json_string_slice_array(output, &blocked);
        output.push('}');
    }
    output.push(']');
    output.push_str(",\"blockedReasons\":");
    push_json_string_slice_array(output, &blocked_reasons);
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-frame-candidate-agreement-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string("source-horizontal-field-semantics-unproven"));
    output.push('}');
}

pub(crate) fn table_grid_page_space_horizontal_frame_candidate_supports(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    source_layout: Option<&TableGridSourceDerivedLayout>,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    if let Some(source_layout) = source_layout {
        supports.extend(
            table_grid_page_space_horizontal_page_mark_raw_field_candidate_supports(source_layout),
        );
    }
    supports.extend(
        table_grid_source_only_horizontal_field_consensus_candidate_supports(
            layout, document, lines, candidate,
        ),
    );
    if let Some(selector_support) =
        table_grid_source_only_horizontal_field_selector_candidate_support(
            layout, document, lines, candidate,
        )
    {
        supports.push(selector_support);
    }
    supports
}

pub(crate) fn table_grid_page_space_horizontal_page_mark_raw_field_candidate_supports(
    source_layout: &TableGridSourceDerivedLayout,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    let Some(fields) = table_grid_source_layout_page_mark_u16_fields(source_layout) else {
        return supports;
    };
    let Some(word_14) = fields.get(14).copied() else {
        return supports;
    };
    let Some(word_21) = fields.get(21).copied() else {
        return supports;
    };
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-word21-direct",
        selected_x: f32::from(word_14),
        selected_width: f32::from(word_21),
        contribution: "source-horizontal-page-mark-raw-field-hypothesis",
        blocked_reason: "page-mark-raw-horizontal-field-semantics-unproven",
    });

    let Some(first_slot_units) = source_layout
        .x_unit_column_slot_width_units
        .first()
        .copied()
        .filter(|units| *units > 0)
    else {
        return supports;
    };
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let selected_width = f32::from(word_21) - f32::from(first_slot_units) * 0.5;
    if selected_x.is_finite() && selected_width.is_finite() && selected_width > 0.0 {
        supports.push(TableGridHorizontalFrameCandidateSupport {
            frame_basis: "page-mark-word14-word21-first-slot-adjusted",
            selected_x,
            selected_width,
            contribution: "source-horizontal-page-mark-raw-field-hypothesis",
            blocked_reason: "page-mark-raw-horizontal-field-semantics-unproven",
        });
    }
    supports
}

pub(crate) fn table_grid_source_only_horizontal_field_consensus_candidate_supports(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let Some((page_mark_fields, stable_first_slot_units, stable_first_gap_units)) =
        table_grid_source_only_horizontal_field_consensus_inputs(
            layout, document, lines, candidate,
        )
    else {
        return Vec::new();
    };
    table_grid_source_only_horizontal_field_consensus_supports(
        &page_mark_fields,
        stable_first_slot_units,
        stable_first_gap_units,
    )
}

pub(crate) fn table_grid_source_only_horizontal_field_selector_candidate_support(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Option<TableGridHorizontalFrameCandidateSupport> {
    let (page_mark_fields, stable_first_slot_units, stable_first_gap_units) =
        table_grid_source_only_horizontal_field_consensus_inputs(
            layout, document, lines, candidate,
        )?;
    let word_14 = page_mark_fields.get(14).copied()?;
    let first_slot_units = stable_first_slot_units.filter(|units| *units > 0)?;
    let compact_column_count = candidate
        .column_segment_grid_candidate()
        .map(|grid| grid.column_count())
        .unwrap_or_else(|| candidate.max_column_segment_count().max(1));
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    let (frame_basis, selected_width) = match compact_column_count {
        2 => {
            let word_21 = page_mark_fields.get(21).copied()?;
            (
                "page-mark-word14-first-slot-word21-half-slot",
                f32::from(word_21) - f32::from(first_slot_units) * 0.5,
            )
        }
        3 => {
            let word_15 = page_mark_fields.get(15).copied()?;
            let first_gap_units = stable_first_gap_units.filter(|units| *units > 0)?;
            (
                "page-mark-word14-first-slot-word15-half-gap",
                f32::from(word_15) - f32::from(first_gap_units) * 0.5,
            )
        }
        _ => return None,
    };
    if !selected_x.is_finite() || !selected_width.is_finite() || selected_width <= 0.0 {
        return None;
    }
    Some(TableGridHorizontalFrameCandidateSupport {
        frame_basis,
        selected_x,
        selected_width,
        contribution: "source-only-horizontal-field-selector",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    })
}

pub(crate) fn table_grid_source_only_horizontal_field_consensus_inputs(
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
) -> Option<(Vec<u16>, Option<u16>, Option<u16>)> {
    let current_source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = current_source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let page_mark_fields = source_layout_page_mark_fields
        .or(cross_table_page_mark_fields)?
        .to_vec();

    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate);
    let sparse_table_candidate_index = sibling
        .as_ref()
        .map(|evidence| evidence.sparse_candidate.index());
    let related_source_layouts = table_grid_related_horizontal_source_layout_summaries(
        layout,
        document,
        lines,
        candidate,
        sparse_table_candidate_index,
        current_source_layout.as_ref(),
    );
    if related_source_layouts.is_empty() {
        return None;
    }

    let first_slot_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_column_slot_units)
        .collect::<Vec<_>>();
    let first_gap_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_intercell_gap_units)
        .collect::<Vec<_>>();
    Some((
        page_mark_fields,
        single_u16_value(&first_slot_units),
        single_u16_value(&first_gap_units),
    ))
}

pub(crate) fn table_grid_source_only_horizontal_field_consensus_supports(
    page_mark_fields: &[u16],
    stable_first_slot_units: Option<u16>,
    stable_first_gap_units: Option<u16>,
) -> Vec<TableGridHorizontalFrameCandidateSupport> {
    let mut supports = Vec::new();
    let Some(word_14) = page_mark_fields.get(14).copied() else {
        return supports;
    };
    let Some(word_15) = page_mark_fields.get(15).copied() else {
        return supports;
    };
    let Some(word_21) = page_mark_fields.get(21).copied() else {
        return supports;
    };
    let Some(first_slot_units) = stable_first_slot_units.filter(|units| *units > 0) else {
        return supports;
    };
    let selected_x = f32::from(word_14) - f32::from(first_slot_units);
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-first-slot-word15-direct",
        selected_x,
        selected_width: f32::from(word_15),
        contribution: "source-only-horizontal-field-consensus",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    });
    if let Some(half_gap_adjustment) = stable_first_gap_units
        .filter(|units| *units > 0)
        .map(|units| f32::from(units) * 0.5)
    {
        supports.push(TableGridHorizontalFrameCandidateSupport {
            frame_basis: "page-mark-word14-first-slot-word15-half-gap",
            selected_x,
            selected_width: f32::from(word_15) - half_gap_adjustment,
            contribution: "source-only-horizontal-field-consensus",
            blocked_reason: "cross-table-horizontal-field-semantics-unproven",
        });
    }
    supports.push(TableGridHorizontalFrameCandidateSupport {
        frame_basis: "page-mark-word14-first-slot-word21-half-slot",
        selected_x,
        selected_width: f32::from(word_21) - f32::from(first_slot_units) * 0.5,
        contribution: "source-only-horizontal-field-consensus",
        blocked_reason: "cross-table-horizontal-field-semantics-unproven",
    });
    supports
}

pub(crate) fn push_table_grid_page_space_horizontal_source_only_consensus_frame_hypotheses_json(
    output: &mut String,
    layout: PageLayout,
    document: &Document,
    lines: &[PageTextLine],
    candidate: &TableCandidate,
    emitted: &mut bool,
) {
    let current_source_layout = candidate.column_segment_grid_candidate().and_then(|grid| {
        source_derived_table_grid_overlay_layout(
            layout,
            document,
            lines,
            0,
            candidate,
            grid.column_count(),
        )
    });
    let source_layout_page_mark_fields = current_source_layout
        .as_ref()
        .and_then(table_grid_source_layout_page_mark_u16_fields);
    let cross_table_probe = table_grid_cross_table_row_boundary_offset_probe(document, candidate);
    let cross_table_page_mark_fields = cross_table_probe
        .as_ref()
        .map(|probe| probe.page_mark_u16_field_preview.as_slice())
        .filter(|fields| !fields.is_empty());
    let Some(page_mark_fields) = source_layout_page_mark_fields.or(cross_table_page_mark_fields)
    else {
        return;
    };

    let sibling = table_grid_sparse_table_sibling_evidence(document, candidate);
    let sparse_table_candidate_index = sibling
        .as_ref()
        .map(|evidence| evidence.sparse_candidate.index());
    let related_source_layouts = table_grid_related_horizontal_source_layout_summaries(
        layout,
        document,
        lines,
        candidate,
        sparse_table_candidate_index,
        current_source_layout.as_ref(),
    );
    if related_source_layouts.is_empty() {
        return;
    }

    let first_slot_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_column_slot_units)
        .collect::<Vec<_>>();
    let first_gap_units = related_source_layouts
        .iter()
        .filter_map(|summary| summary.first_intercell_gap_units)
        .collect::<Vec<_>>();
    push_table_grid_source_only_horizontal_field_consensus_hypotheses_items_json(
        output,
        page_mark_fields,
        single_u16_value(&first_slot_units),
        single_u16_value(&first_gap_units),
        emitted,
    );
}

pub(crate) fn push_table_grid_page_space_horizontal_page_mark_raw_field_hypotheses_json(
    output: &mut String,
    source_layout: &TableGridSourceDerivedLayout,
) {
    let Some(fields) = table_grid_source_layout_page_mark_u16_fields(source_layout) else {
        return;
    };
    let Some(word_14) = fields.get(14).copied() else {
        return;
    };
    let Some(word_21) = fields.get(21).copied() else {
        return;
    };

    output.push(',');
    push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
        output,
        "page-mark-word14-word21-direct",
        f32::from(word_14),
        f32::from(word_21),
        word_14,
        word_21,
        None,
        0.0,
        0.0,
        "none",
    );

    let Some(first_slot_units) = source_layout
        .x_unit_column_slot_width_units
        .first()
        .copied()
        .filter(|units| *units > 0)
    else {
        return;
    };
    let x_adjustment_units = f32::from(first_slot_units);
    let width_adjustment_units = f32::from(first_slot_units) * 0.5;
    let selected_x = f32::from(word_14) - x_adjustment_units;
    let selected_width = f32::from(word_21) - width_adjustment_units;
    if selected_x.is_finite() && selected_width.is_finite() && selected_width > 0.0 {
        output.push(',');
        push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
            output,
            "page-mark-word14-word21-first-slot-adjusted",
            selected_x,
            selected_width,
            word_14,
            word_21,
            Some(first_slot_units),
            x_adjustment_units,
            width_adjustment_units,
            "line-header-first-column-slot",
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_page_space_horizontal_page_mark_raw_field_hypothesis_json(
    output: &mut String,
    frame_basis: &'static str,
    selected_x: f32,
    selected_width: f32,
    page_mark_word_14: u16,
    page_mark_word_21: u16,
    first_column_slot_units: Option<u16>,
    x_adjustment_units: f32,
    width_adjustment_units: f32,
    adjustment_basis: &'static str,
) {
    output.push_str("{\"frameBasis\":");
    output.push_str(&json_string(frame_basis));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"coordinateBasis\":\"page-mark-direct-u16-px\"");
    output.push_str(",\"pageMarkWord14\":");
    output.push_str(&page_mark_word_14.to_string());
    output.push_str(",\"pageMarkWord21\":");
    output.push_str(&page_mark_word_21.to_string());
    output.push_str(",\"firstColumnSlotUnits\":");
    push_optional_u16_json(output, first_column_slot_units);
    output.push_str(",\"xAdjustmentUnits\":");
    output.push_str(&format!("{x_adjustment_units:.3}"));
    output.push_str(",\"widthAdjustmentUnits\":");
    output.push_str(&format!("{width_adjustment_units:.3}"));
    output.push_str(",\"adjustmentBasis\":");
    output.push_str(&json_string(adjustment_basis));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(
        ",\"renderPromotionContribution\":\"source-horizontal-page-mark-raw-field-hypothesis\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        "page-mark-raw-horizontal-field-semantics-unproven",
    ));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_table_grid_page_space_horizontal_frame_hypothesis_json(
    output: &mut String,
    frame_basis: &'static str,
    frame_x: f32,
    frame_width: f32,
    selected_start_units: f32,
    selected_width_units: f32,
    full_extent_units: f32,
    blocked_reason: &'static str,
) {
    let selected_x = frame_x + frame_width * selected_start_units / full_extent_units;
    let selected_width = frame_width * selected_width_units / full_extent_units;
    output.push_str("{\"frameBasis\":");
    output.push_str(&json_string(frame_basis));
    output.push_str(",\"sourceBacked\":true,\"referenceBacked\":false,\"decoded\":false,\"geometryDecoded\":false,\"placementDerived\":false");
    output.push_str(",\"frameX\":");
    output.push_str(&format!("{frame_x:.3}"));
    output.push_str(",\"frameWidth\":");
    output.push_str(&format!("{frame_width:.3}"));
    output.push_str(",\"selectedX\":");
    output.push_str(&format!("{selected_x:.3}"));
    output.push_str(",\"selectedWidth\":");
    output.push_str(&format!("{selected_width:.3}"));
    output.push_str(",\"selectedStartUnits\":");
    output.push_str(&format!("{selected_start_units:.3}"));
    output.push_str(",\"selectedWidthUnits\":");
    output.push_str(&format!("{selected_width_units:.3}"));
    output.push_str(",\"fullExtentUnits\":");
    output.push_str(&format!("{full_extent_units:.3}"));
    output.push_str(",\"renderPromotionContribution\":\"source-horizontal-frame-hypothesis\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(blocked_reason));
    output.push('}');
}
