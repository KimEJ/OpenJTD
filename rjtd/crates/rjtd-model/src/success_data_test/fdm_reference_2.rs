use super::*;
use crate::*;

pub(crate) fn success_data_test_fdm_index_row_order_promotion_gate(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmIndexRowOrderPromotionGate {
    let mut gate = SuccessDataTestFdmIndexRowOrderPromotionGate {
        command_count: classifications.len(),
        ..SuccessDataTestFdmIndexRowOrderPromotionGate::default()
    };

    for classification in classifications {
        for reference in &classification.index_row_references {
            gate.reference_count += 1;
            gate.referenced_command_relative_offsets
                .insert(classification.command.relative_offset());
            gate.referenced_row_indexes.insert(reference.row_index);
            gate.row_command_pairs
                .insert(SuccessDataTestFdmIndexRowCommandPair {
                    row_index: reference.row_index,
                    command_relative_offset: classification.command.relative_offset(),
                    match_kind: reference.match_kind,
                });
            gate.row_to_command_relative_offsets
                .entry(reference.row_index)
                .or_default()
                .insert(classification.command.relative_offset());
            if reference.valid_vector_offset {
                gate.valid_vector_offset_reference_count += 1;
            }
            match reference.match_kind {
                "command-relative-offset-field" => {
                    gate.command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    gate.source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
    }
    gate
}

pub(crate) fn push_success_data_test_fdm_index_row_order_promotion_gate_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let gate = success_data_test_fdm_index_row_order_promotion_gate(classifications);
    let render_promotion_blocked_reasons =
        success_data_test_fdm_index_row_order_promotion_blocked_reasons(classifications, &gate);
    let render_promotion_blocked_reason = render_promotion_blocked_reasons
        .first()
        .copied()
        .unwrap_or("none");
    output.push_str("{\"basis\":\"fdm-index-row-reference-command-order\",\"decoded\":false,\"ownershipProven\":false,\"paintOrderDecoded\":false");
    output.push_str(",\"renderPromotionContribution\":\"fdm-index-row-order-evidence-only\"");
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"renderPromotionBlockedReasons\":");
    push_json_string_slice_array(output, &render_promotion_blocked_reasons);
    output.push_str(",\"commandCount\":");
    output.push_str(&gate.command_count.to_string());
    output.push_str(",\"referencedCommandCount\":");
    output.push_str(&gate.referenced_command_count().to_string());
    output.push_str(",\"unreferencedCommandCount\":");
    output.push_str(&gate.unreferenced_command_count().to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&gate.unique_row_index_count().to_string());
    output.push_str(",\"referenceCount\":");
    output.push_str(&gate.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&gate.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &gate
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"allCommandsReferencedByIndexRowsCandidate\":");
    output.push_str(if gate.all_commands_referenced_by_index_rows_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if gate.one_to_one_row_command_reference_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if gate.single_row_backs_multiple_commands_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowOrderMatchesCommandOrderCandidate\":");
    output.push_str(if gate.row_order_matches_command_order_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"referencedCommandRelativeOffsets\":");
    push_usize_array_json(
        output,
        &gate
            .referenced_command_relative_offsets
            .iter()
            .copied()
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"referencedRowIndexes\":");
    push_usize_array_json(
        output,
        &gate
            .referenced_row_indexes
            .iter()
            .copied()
            .collect::<Vec<_>>(),
    );
    output.push_str(",\"rowCommandPairs\":");
    push_success_data_test_fdm_index_row_command_pairs_json(output, &gate.row_command_pairs);
    output.push_str(
        ",\"renderPaintOrderBasisCandidate\":\"fdm-index-row-command-pairs\",\"renderPaintOrderBasisDecoded\":false",
    );
    output.push('}');
}

pub(crate) fn success_data_test_fdm_index_row_order_promotion_blocked_reasons(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
    gate: &SuccessDataTestFdmIndexRowOrderPromotionGate,
) -> Vec<&'static str> {
    let mut reasons = Vec::new();
    if !gate.all_commands_referenced_by_index_rows_candidate() {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-reference-coverage-incomplete",
        );
    }
    if !gate.one_to_one_row_command_reference_candidate() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-reference-not-one-to-one");
    }
    if gate.single_row_backs_multiple_commands_candidate() {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-single-row-backs-multiple-commands",
        );
    }
    if !gate.row_order_matches_command_order_candidate() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-non-monotonic");
    }
    if gate.reference_count > 0 && gate.valid_vector_offset_reference_count == 0 {
        push_unique_static_str(
            &mut reasons,
            "fdm-index-row-order-valid-vector-offset-missing",
        );
    }
    if gate.command_relative_offset_field_reference_count > 0
        && gate.source_segment_relative_offset_field_reference_count > 0
    {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-offset-namespace-mixed");
    }

    let role_groups =
        success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);
    let mut role_paint_order_continuity_blocked = false;
    let mut role_paint_order_authority_pending = false;
    for group in role_groups.values() {
        let profile =
            success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
        role_paint_order_continuity_blocked |= profile.continuity_blocked();
        role_paint_order_authority_pending |= profile.paint_order_authority_pending();
    }
    if role_paint_order_continuity_blocked {
        push_unique_static_str(&mut reasons, "role-paint-order-continuity-unproven");
    }
    if role_paint_order_authority_pending {
        push_unique_static_str(&mut reasons, "role-paint-order-authority-unproven");
    }
    if reasons.is_empty() {
        push_unique_static_str(&mut reasons, "fdm-index-row-order-paint-authority-unproven");
    }
    reasons
}

pub(crate) fn success_data_test_fdm_index_row_reference_role_candidate_groups(
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> BTreeMap<&'static str, SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup> {
    let mut groups =
        BTreeMap::<&'static str, SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup>::new();
    for classification in classifications {
        if classification.index_row_references.is_empty() {
            continue;
        }
        for role_candidate in &classification.role_candidates {
            let group = groups.entry(*role_candidate).or_insert_with(|| {
                SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup {
                    role_candidate,
                    ..SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup::default()
                }
            });
            group
                .command_relative_offsets
                .insert(classification.command.relative_offset());
            for reference in &classification.index_row_references {
                group.reference_count += 1;
                group.row_indexes.insert(reference.row_index);
                group
                    .row_command_pairs
                    .insert(SuccessDataTestFdmIndexRowCommandPair {
                        row_index: reference.row_index,
                        command_relative_offset: classification.command.relative_offset(),
                        match_kind: reference.match_kind,
                    });
                if reference.valid_vector_offset {
                    group.valid_vector_offset_reference_count += 1;
                    match reference.match_kind {
                        "command-relative-offset-field" => {
                            group.valid_command_relative_offset_field_reference_count += 1;
                        }
                        "source-segment-relative-offset-field" => {
                            group.valid_source_segment_relative_offset_field_reference_count += 1;
                        }
                        _ => {}
                    }
                }
                match reference.match_kind {
                    "command-relative-offset-field" => {
                        group.command_relative_offset_field_reference_count += 1;
                    }
                    "source-segment-relative-offset-field" => {
                        group.source_segment_relative_offset_field_reference_count += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    groups
}

pub(crate) fn success_data_test_fdm_role_group_single_row_backs_multiple_commands(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) -> bool {
    let mut row_to_command_count = BTreeMap::<usize, usize>::new();
    for pair in &group.row_command_pairs {
        *row_to_command_count.entry(pair.row_index).or_default() += 1;
    }
    row_to_command_count.values().any(|count| *count > 1)
}

pub(crate) fn push_success_data_test_fdm_index_row_reference_role_candidate_groups_json(
    output: &mut String,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    let groups = success_data_test_fdm_index_row_reference_role_candidate_groups(classifications);

    output.push('[');
    for (index, group) in groups.values().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"roleCandidate\":");
        output.push_str(&json_string(group.role_candidate));
        output.push_str(",\"ownershipProven\":false");
        output.push_str(
            ",\"ownershipPromotionBlockedReason\":\"role-candidate-and-paint-order-unproven\"",
        );
        output.push_str(",\"referenceCount\":");
        output.push_str(&group.reference_count.to_string());
        output.push_str(",\"validVectorOffsetReferenceCount\":");
        output.push_str(&group.valid_vector_offset_reference_count.to_string());
        output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
        output.push_str(
            &group
                .command_relative_offset_field_reference_count
                .to_string(),
        );
        output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
        output.push_str(
            &group
                .source_segment_relative_offset_field_reference_count
                .to_string(),
        );
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(
            output,
            &group
                .command_relative_offsets
                .iter()
                .copied()
                .collect::<Vec<_>>(),
        );
        output.push_str(",\"rowIndexes\":");
        push_usize_array_json(
            output,
            &group.row_indexes.iter().copied().collect::<Vec<_>>(),
        );
        output.push_str(",\"uniqueCommandRelativeOffsetCount\":");
        output.push_str(&group.command_relative_offsets.len().to_string());
        output.push_str(",\"uniqueRowIndexCount\":");
        output.push_str(&group.row_indexes.len().to_string());
        output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
        output.push_str(
            if group.reference_count == group.command_relative_offsets.len()
                && group.reference_count == group.row_indexes.len()
            {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
        output.push_str(
            if group.row_indexes.len() == 1 && group.command_relative_offsets.len() > 1 {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"rowOrderMatchesCommandOrderCandidate\":");
        output.push_str(
            if success_data_test_fdm_row_command_pairs_are_monotonic(&group.row_command_pairs) {
                "true"
            } else {
                "false"
            },
        );
        output.push_str(",\"rowCommandPairs\":");
        push_success_data_test_fdm_index_row_command_pairs_json(output, &group.row_command_pairs);
        output.push_str(",\"roleVectorOffsetAuthorityGate\":");
        push_success_data_test_fdm_role_vector_offset_authority_gate_json(output, group);
        output.push_str(",\"roleFanoutSegmentOwnerGate\":");
        push_success_data_test_fdm_role_fanout_segment_owner_gate_json(output, group);
        output.push_str(",\"decoded\":false,\"paintOrderContinuityProfile\":");
        push_success_data_test_fdm_role_paint_order_continuity_profile_json(
            output,
            group,
            classifications,
        );
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn success_data_test_fdm_role_vector_offset_authority_blocked_reason(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) -> &'static str {
    let mixed_valid_offset_namespaces = group.valid_command_relative_offset_field_reference_count
        > 0
        && group.valid_source_segment_relative_offset_field_reference_count > 0;
    if group.valid_vector_offset_reference_count == 0 {
        "fdm-index-role-vector-offset-authority-valid-vector-offset-missing"
    } else if mixed_valid_offset_namespaces {
        "fdm-index-role-vector-offset-authority-mixed-valid-offset-namespaces"
    } else {
        "fdm-index-role-vector-offset-authority-semantics-unproven"
    }
}

pub(crate) fn push_success_data_test_fdm_role_vector_offset_authority_gate_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) {
    let invalid_vector_offset_reference_count = group
        .reference_count
        .saturating_sub(group.valid_vector_offset_reference_count);
    let invalid_command_relative_offset_field_reference_count = group
        .command_relative_offset_field_reference_count
        .saturating_sub(group.valid_command_relative_offset_field_reference_count);
    let invalid_source_segment_relative_offset_field_reference_count = group
        .source_segment_relative_offset_field_reference_count
        .saturating_sub(group.valid_source_segment_relative_offset_field_reference_count);
    let mixed_offset_namespaces_among_valid_refs =
        group.valid_command_relative_offset_field_reference_count > 0
            && group.valid_source_segment_relative_offset_field_reference_count > 0;
    let all_valid_references_use_command_relative_offset_field =
        group.valid_vector_offset_reference_count > 0
            && group.valid_command_relative_offset_field_reference_count
                == group.valid_vector_offset_reference_count;
    let all_valid_references_use_source_segment_relative_offset_field =
        group.valid_vector_offset_reference_count > 0
            && group.valid_source_segment_relative_offset_field_reference_count
                == group.valid_vector_offset_reference_count;
    let all_references_have_invalid_vector_offset =
        group.reference_count > 0 && group.valid_vector_offset_reference_count == 0;
    let render_promotion_blocked_reason =
        success_data_test_fdm_role_vector_offset_authority_blocked_reason(group);

    output.push_str("{\"basis\":\"fdm-index-role-vector-offset-authority-gate\",\"source\":\"FDMIndex.vectorOffset+FDMIndex role offset fields\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"roleCandidate\":");
    output.push_str(&json_string(group.role_candidate));
    output.push_str(",\"roleVectorOffsetAuthorityDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"fdm-index-role-vector-offset-authority-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"referenceCount\":");
    output.push_str(&group.reference_count.to_string());
    output.push_str(",\"validVectorOffsetReferenceCount\":");
    output.push_str(&group.valid_vector_offset_reference_count.to_string());
    output.push_str(",\"invalidVectorOffsetReferenceCount\":");
    output.push_str(&invalid_vector_offset_reference_count.to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"validCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .valid_command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"validSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .valid_source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"invalidCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(&invalid_command_relative_offset_field_reference_count.to_string());
    output.push_str(",\"invalidSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(&invalid_source_segment_relative_offset_field_reference_count.to_string());
    output.push_str(",\"allValidReferencesUseCommandRelativeOffsetField\":");
    output.push_str(if all_valid_references_use_command_relative_offset_field {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allValidReferencesUseSourceSegmentRelativeOffsetField\":");
    output.push_str(
        if all_valid_references_use_source_segment_relative_offset_field {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"mixedOffsetNamespacesAmongValidReferences\":");
    output.push_str(if mixed_offset_namespaces_among_valid_refs {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"allReferencesHaveInvalidVectorOffset\":");
    output.push_str(if all_references_have_invalid_vector_offset {
        "true"
    } else {
        "false"
    });
    output.push('}');
}

pub(crate) fn push_success_data_test_fdm_role_fanout_segment_owner_gate_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
) {
    let mut row_to_pairs = BTreeMap::<usize, Vec<SuccessDataTestFdmIndexRowCommandPair>>::new();
    for pair in &group.row_command_pairs {
        row_to_pairs.entry(pair.row_index).or_default().push(*pair);
    }

    let mut fanout_row_count = 0usize;
    let mut fanout_reference_count = 0usize;
    let mut fanout_command_relative_offset_field_reference_count = 0usize;
    let mut fanout_source_segment_relative_offset_field_reference_count = 0usize;
    let mut max_row_fanout = 0usize;
    for pairs in row_to_pairs.values() {
        max_row_fanout = max_row_fanout.max(pairs.len());
        if pairs.len() <= 1 {
            continue;
        }
        fanout_row_count += 1;
        fanout_reference_count += pairs.len();
        for pair in pairs {
            match pair.match_kind {
                "command-relative-offset-field" => {
                    fanout_command_relative_offset_field_reference_count += 1;
                }
                "source-segment-relative-offset-field" => {
                    fanout_source_segment_relative_offset_field_reference_count += 1;
                }
                _ => {}
            }
        }
    }

    let one_to_one_row_command_reference_candidate = group.reference_count
        == group.command_relative_offsets.len()
        && group.reference_count == group.row_indexes.len();
    let single_row_backs_multiple_commands_candidate =
        row_to_pairs.values().any(|pairs| pairs.len() > 1);
    let mixed_offset_field_namespaces = group.command_relative_offset_field_reference_count > 0
        && group.source_segment_relative_offset_field_reference_count > 0;
    let fanout_rows_use_command_relative_offset_fields = fanout_reference_count > 0
        && fanout_command_relative_offset_field_reference_count == fanout_reference_count;
    let fanout_rows_use_source_segment_offset_fields = fanout_reference_count > 0
        && fanout_source_segment_relative_offset_field_reference_count == fanout_reference_count;
    let render_promotion_blocked_reason = if single_row_backs_multiple_commands_candidate {
        "fdm-index-role-row-fanout-multi-command-single-row"
    } else if !one_to_one_row_command_reference_candidate {
        "fdm-index-role-row-reference-not-one-to-one"
    } else if mixed_offset_field_namespaces {
        "fdm-index-role-offset-namespace-mixed"
    } else if group.valid_vector_offset_reference_count == 0 {
        "fdm-index-role-valid-vector-offset-missing"
    } else {
        "fdm-index-role-segment-owner-semantics-unproven"
    };

    output.push_str("{\"basis\":\"fdm-index-role-row-fanout-segment-owner-gate\",\"source\":\"FDMIndex role row references+FDMVector source segments\",\"decoded\":false,\"sourceBacked\":true");
    output.push_str(",\"roleCandidate\":");
    output.push_str(&json_string(group.role_candidate));
    output.push_str(",\"roleOwnershipDecoded\":false,\"segmentOwnerDecoded\":false");
    output.push_str(
        ",\"renderPromotionContribution\":\"fdm-index-role-row-fanout-segment-owner-gate\"",
    );
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(render_promotion_blocked_reason));
    output.push_str(",\"referenceCount\":");
    output.push_str(&group.reference_count.to_string());
    output.push_str(",\"uniqueCommandRelativeOffsetCount\":");
    output.push_str(&group.command_relative_offsets.len().to_string());
    output.push_str(",\"uniqueRowIndexCount\":");
    output.push_str(&group.row_indexes.len().to_string());
    output.push_str(",\"commandRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .command_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"sourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(
        &group
            .source_segment_relative_offset_field_reference_count
            .to_string(),
    );
    output.push_str(",\"fanoutRowCount\":");
    output.push_str(&fanout_row_count.to_string());
    output.push_str(",\"fanoutReferenceCount\":");
    output.push_str(&fanout_reference_count.to_string());
    output.push_str(",\"fanoutCommandRelativeOffsetFieldReferenceCount\":");
    output.push_str(&fanout_command_relative_offset_field_reference_count.to_string());
    output.push_str(",\"fanoutSourceSegmentRelativeOffsetFieldReferenceCount\":");
    output.push_str(&fanout_source_segment_relative_offset_field_reference_count.to_string());
    output.push_str(",\"maxRowFanout\":");
    output.push_str(&max_row_fanout.to_string());
    output.push_str(",\"oneToOneRowCommandReferenceCandidate\":");
    output.push_str(if one_to_one_row_command_reference_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"singleRowBacksMultipleCommandsCandidate\":");
    output.push_str(if single_row_backs_multiple_commands_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mixedOffsetFieldNamespaces\":");
    output.push_str(if mixed_offset_field_namespaces {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseCommandRelativeOffsetFields\":");
    output.push_str(if fanout_rows_use_command_relative_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"fanoutRowsUseSourceSegmentOffsetFields\":");
    output.push_str(if fanout_rows_use_source_segment_offset_fields {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"rowsWithMultipleCommandRefs\":");
    push_success_data_test_fdm_role_fanout_rows_json(output, &row_to_pairs);
    output.push('}');
}

pub(crate) fn push_success_data_test_fdm_role_fanout_rows_json(
    output: &mut String,
    row_to_pairs: &BTreeMap<usize, Vec<SuccessDataTestFdmIndexRowCommandPair>>,
) {
    output.push('[');
    let mut emitted = 0usize;
    for (row_index, pairs) in row_to_pairs {
        if pairs.len() <= 1 {
            continue;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        let command_relative_offsets = pairs
            .iter()
            .map(|pair| pair.command_relative_offset)
            .collect::<Vec<_>>();
        let match_kinds = pairs
            .iter()
            .map(|pair| pair.match_kind)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        output.push_str("{\"rowIndex\":");
        output.push_str(&row_index.to_string());
        output.push_str(",\"commandReferenceCount\":");
        output.push_str(&pairs.len().to_string());
        output.push_str(",\"commandRelativeOffsets\":");
        push_usize_array_json(output, &command_relative_offsets);
        output.push_str(",\"matchKinds\":");
        push_json_string_slice_array(output, &match_kinds);
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn push_success_data_test_fdm_role_paint_order_continuity_profile_json(
    output: &mut String,
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) {
    output.push_str("{\"basis\":\"fdm-index-row-reference-role-command-span\",\"decoded\":false,\"sourceBacked\":true,\"paintOrderDecoded\":false");
    let profile = success_data_test_fdm_role_paint_order_continuity_profile(group, classifications);
    output.push_str(",\"commandRelativeOffsetSpanMin\":");
    push_option_usize_json(output, profile.span_min);
    output.push_str(",\"commandRelativeOffsetSpanMax\":");
    push_option_usize_json(output, profile.span_max);
    output.push_str(",\"roleCommandCount\":");
    output.push_str(&profile.role_command_count.to_string());
    output.push_str(",\"commandCountInSpan\":");
    output.push_str(&profile.command_count_in_span.to_string());
    output.push_str(",\"interleavedNonRoleCommandCount\":");
    output.push_str(&profile.interleaved_non_role_command_count.to_string());
    output.push_str(",\"hasInterleavedNonRoleCommands\":");
    output.push_str(if profile.interleaved_non_role_command_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"maxCommandOffsetGap\":");
    output.push_str(&profile.max_command_offset_gap.to_string());
    output.push_str(",\"commandOffsetContinuityScore\":");
    output.push_str(&format!("{:.3}", profile.continuity_score));
    output.push_str(",\"spanContiguousCandidate\":");
    output.push_str(if profile.span_contiguous_candidate() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintOrderAuthorityPending\":");
    output.push_str(if profile.paint_order_authority_pending() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"continuityBlocked\":");
    output.push_str(if profile.continuity_blocked() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(profile.render_promotion_blocked_reason()));
    output.push('}');
}

pub(crate) fn success_data_test_fdm_role_paint_order_continuity_profile(
    group: &SuccessDataTestFdmIndexRowReferenceRoleCandidateGroup,
    classifications: &[SuccessDataTestFdmPrimitiveOwnershipClassification<'_>],
) -> SuccessDataTestFdmRolePaintOrderContinuityProfile {
    let span_min = group.command_relative_offsets.iter().next().copied();
    let span_max = group.command_relative_offsets.iter().next_back().copied();
    let role_command_count = group.command_relative_offsets.len();
    let command_count_in_span = match (span_min, span_max) {
        (Some(min), Some(max)) => classifications
            .iter()
            .filter(|classification| {
                let offset = classification.command.relative_offset();
                offset >= min && offset <= max
            })
            .count(),
        _ => 0,
    };
    let interleaved_non_role_command_count =
        command_count_in_span.saturating_sub(role_command_count);
    let mut max_command_offset_gap = 0usize;
    let mut previous_offset = None;
    for offset in group.command_relative_offsets.iter().copied() {
        if let Some(previous) = previous_offset {
            max_command_offset_gap = max_command_offset_gap.max(offset.saturating_sub(previous));
        }
        previous_offset = Some(offset);
    }
    let continuity_score = if command_count_in_span == 0 {
        0.0
    } else {
        role_command_count as f32 / command_count_in_span as f32
    };

    SuccessDataTestFdmRolePaintOrderContinuityProfile {
        span_min,
        span_max,
        role_command_count,
        command_count_in_span,
        interleaved_non_role_command_count,
        max_command_offset_gap,
        continuity_score,
    }
}

pub(crate) fn success_data_test_fdm_row_command_pairs_are_monotonic(
    pairs: &BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
) -> bool {
    let mut previous_command_relative_offset = None;
    for pair in pairs {
        if previous_command_relative_offset
            .is_some_and(|previous| pair.command_relative_offset < previous)
        {
            return false;
        }
        previous_command_relative_offset = Some(pair.command_relative_offset);
    }
    true
}

pub(crate) fn push_success_data_test_fdm_index_row_command_pairs_json(
    output: &mut String,
    pairs: &BTreeSet<SuccessDataTestFdmIndexRowCommandPair>,
) {
    output.push('[');
    for (index, pair) in pairs.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&pair.row_index.to_string());
        output.push_str(",\"commandRelativeOffset\":");
        output.push_str(&pair.command_relative_offset.to_string());
        output.push_str(",\"matchKind\":");
        output.push_str(&json_string(pair.match_kind));
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn success_data_test_fdm_primitive_ownership_classification<'a>(
    projection: SuccessDataTestFdmProjection,
    command: &'a ObjectFdmVectorCommandCandidate,
    index_entries: &[ObjectFdmIndexEntryCandidate],
    anchor: Option<(ObjectFdmVectorPoint, i32)>,
) -> SuccessDataTestFdmPrimitiveOwnershipClassification<'a> {
    let mut role_candidates = Vec::new();
    let mut classification_basis = Vec::new();
    if let Some(ellipse) = command.ellipse() {
        if success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse) {
            role_candidates.push("main-circle-anchor");
            classification_basis.push("large-01000460-ellipse-anchor");
        } else if success_data_test_fdm_reference_ellipse_is_control_marker(
            projection, command, ellipse,
        ) {
            role_candidates.push("arc-candidate");
            role_candidates.push("control-ellipse-marker");
            classification_basis.push("tiny-ff000460-ellipse-control-marker");
        } else {
            role_candidates.push("arc-candidate");
            classification_basis.push("ellipse-boundary-primitive");
        }
    } else {
        let is_two_point_line = fdm_vector_marker_is_line(command.marker())
            && command.curve_segments().is_empty()
            && command.path_points().len() == 2;
        if is_two_point_line {
            role_candidates.push("line-candidate");
            classification_basis.push("fdm-line-marker-two-point-path");
            if let Some((center, radius)) = anchor {
                let boundary_count =
                    success_data_test_fdm_anchor_boundary_point_count(command, center, radius);
                let center_count =
                    success_data_test_fdm_anchor_center_point_count(command, center, radius);
                if boundary_count >= 2 {
                    role_candidates.push("chord-candidate");
                    classification_basis.push("both-endpoints-near-anchor-boundary");
                } else if boundary_count >= 1 && center_count >= 1 {
                    role_candidates.push("radial-line-candidate");
                    classification_basis.push("one-endpoint-near-anchor-center-one-near-boundary");
                }
            }
        }
        if !command.curve_segments().is_empty()
            || fdm_vector_marker_is_bezier_curve(command.marker())
        {
            role_candidates.push("arc-candidate");
            classification_basis.push("fdm-bezier-marker-or-control-points");
        }
        if command.path_points().len() >= 3 && !fdm_vector_path_is_closed(command.path_points()) {
            role_candidates.push("surface-boundary-candidate");
            classification_basis.push("open-polyline-with-three-or-more-points");
        }
        if fdm_connector_candidate_from_command(command).is_some() {
            role_candidates.push("connector-candidate");
            classification_basis.push("long-open-source-path");
        }
    }
    if role_candidates.is_empty() {
        role_candidates.push("unclassified-primitive");
        classification_basis.push("no-current-role-rule");
    }
    SuccessDataTestFdmPrimitiveOwnershipClassification {
        command,
        role_candidates,
        classification_basis,
        index_row_references: success_data_test_fdm_index_row_references(command, index_entries),
    }
}

pub(crate) fn success_data_test_fdm_index_row_references(
    command: &ObjectFdmVectorCommandCandidate,
    index_entries: &[ObjectFdmIndexEntryCandidate],
) -> Vec<SuccessDataTestFdmIndexRowReference> {
    let mut references = Vec::new();
    for entry in index_entries {
        let bbox = entry.bbox();
        let offset_value = bbox.left();
        if offset_value < 0 {
            continue;
        }
        let offset_value = offset_value as usize;
        let match_kind = if offset_value == command.relative_offset() {
            Some("command-relative-offset-field")
        } else if command
            .source_segment()
            .is_some_and(|segment| segment.relative_offset() == offset_value)
        {
            Some("source-segment-relative-offset-field")
        } else {
            None
        };
        let Some(match_kind) = match_kind else {
            continue;
        };
        references.push(SuccessDataTestFdmIndexRowReference {
            row_index: entry.row_index(),
            index_offset: entry.index_offset(),
            vector_offset: entry.vector_offset(),
            valid_vector_offset: entry.valid_vector_offset(),
            offset_field: "bbox.left",
            offset_value,
            match_kind,
        });
    }
    references
}

pub(crate) fn push_success_data_test_fdm_index_row_references_json(
    output: &mut String,
    references: &[SuccessDataTestFdmIndexRowReference],
) {
    output.push('[');
    for (index, reference) in references.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"rowIndex\":");
        output.push_str(&reference.row_index.to_string());
        output.push_str(",\"indexOffset\":");
        output.push_str(&reference.index_offset.to_string());
        output.push_str(",\"vectorOffset\":");
        output.push_str(&reference.vector_offset.to_string());
        output.push_str(",\"validVectorOffset\":");
        output.push_str(if reference.valid_vector_offset {
            "true"
        } else {
            "false"
        });
        output.push_str(",\"offsetField\":");
        output.push_str(&json_string(reference.offset_field));
        output.push_str(",\"offsetValue\":");
        output.push_str(&reference.offset_value.to_string());
        output.push_str(",\"matchKind\":");
        output.push_str(&json_string(reference.match_kind));
        output.push_str(",\"decoded\":false}");
    }
    output.push(']');
}

pub(crate) fn success_data_test_fdm_anchor_boundary_point_count(
    command: &ObjectFdmVectorCommandCandidate,
    center: ObjectFdmVectorPoint,
    radius: i32,
) -> usize {
    let tolerance = (radius / 12).max(24) as f32;
    command
        .path_points()
        .iter()
        .filter(|point| (fdm_point_distance(center, **point) - radius as f32).abs() <= tolerance)
        .count()
}

pub(crate) fn success_data_test_fdm_anchor_center_point_count(
    command: &ObjectFdmVectorCommandCandidate,
    center: ObjectFdmVectorPoint,
    radius: i32,
) -> usize {
    let tolerance = (radius / 8).max(24) as f32;
    command
        .path_points()
        .iter()
        .filter(|point| fdm_point_distance(center, **point) <= tolerance)
        .count()
}

pub(crate) fn push_success_data_test_fdm_reference_projections_json(
    output: &mut String,
    candidate: &ObjectStreamCandidate,
) {
    if candidate.path() != SUCCESS_DATA_TEST_FDM_VECTOR_PATH {
        output.push_str("[]");
        return;
    }
    let raw_commands = candidate.fdm_raw_vector_commands();
    output.push('[');
    let mut emitted = 0usize;
    for projection in success_data_test_fdm_reference_projections(candidate) {
        let commands = raw_commands
            .iter()
            .filter(|command| success_data_test_fdm_projection_command(projection, command))
            .collect::<Vec<_>>();
        if commands.is_empty() {
            continue;
        }
        if emitted > 0 {
            output.push(',');
        }
        emitted += 1;
        output.push_str("{\"role\":");
        output.push_str(&json_string(projection.role));
        output.push_str(",\"sourcePath\":");
        output.push_str(&json_string(candidate.path()));
        output.push_str(",\"projectionKind\":\"successDataTestFdmReferenceProjection\",\"decoded\":false,\"geometryDecoded\":true,\"placementProven\":false,\"referenceBacked\":true");
        output.push_str(",\"scaleMode\":");
        output.push_str(&json_string(projection.scale_mode.as_str()));
        output.push_str(",\"sourceBbox\":{\"left\":");
        output.push_str(&projection.source_left.to_string());
        output.push_str(",\"top\":");
        output.push_str(&projection.source_top.to_string());
        output.push_str(",\"right\":");
        output.push_str(&projection.source_right.to_string());
        output.push_str(",\"bottom\":");
        output.push_str(&projection.source_bottom.to_string());
        output.push_str("},\"referenceTargetBboxPx\":{\"x\":");
        output.push_str(&format!("{:.3}", projection.target_x_px));
        output.push_str(",\"y\":");
        output.push_str(&format!("{:.3}", projection.target_y_px));
        output.push_str(",\"width\":");
        output.push_str(&format!("{:.3}", projection.target_width_px));
        output.push_str(",\"height\":");
        output.push_str(&format!("{:.3}", projection.target_height_px));
        output.push_str("},\"commandCount\":");
        output.push_str(&commands.len().to_string());
        output.push_str(",\"sourceCohort\":");
        push_success_data_test_fdm_source_cohort_json(output, &commands);
        output.push_str(",\"renderPromotionBlockedReason\":");
        output.push_str(&json_string(
            success_data_test_fdm_source_cohort(&commands).blocked_reason(),
        ));
        output.push_str(",\"primitiveOwnershipComparison\":");
        push_success_data_test_fdm_primitive_ownership_comparison_json(
            output,
            projection,
            &commands,
            candidate.fdm_index_entry_candidates(),
            None,
        );
        output.push_str(",\"subdiagrams\":[");
        if let Some(subdiagrams) = success_data_test_q4_fdm_subdiagrams(projection, &commands) {
            for (index, subdiagram) in subdiagrams.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                output.push_str("{\"index\":");
                output.push_str(&subdiagram.index.to_string());
                output.push_str(",\"groupingSource\":\"nearest-main-circle-source-center\",\"groupingDecoded\":false,\"paintOrderDecoded\":false");
                output.push_str(",\"anchorRelativeOffset\":");
                output.push_str(&subdiagram.anchor_relative_offset.to_string());
                output.push_str(",\"anchorSourcePoint\":");
                push_fdm_vector_point_json(output, subdiagram.center);
                output.push_str(",\"commandCount\":");
                output.push_str(&subdiagram.commands.len().to_string());
                output.push_str(",\"sourceCohort\":");
                push_success_data_test_fdm_source_cohort_json(output, &subdiagram.commands);
                output.push_str(",\"renderPromotionBlockedReason\":");
                output.push_str(&json_string(
                    success_data_test_fdm_source_cohort(&subdiagram.commands).blocked_reason(),
                ));
                output.push_str(",\"primitiveOwnershipComparison\":");
                push_success_data_test_fdm_primitive_ownership_comparison_json(
                    output,
                    projection,
                    &subdiagram.commands,
                    candidate.fdm_index_entry_candidates(),
                    Some((subdiagram.center, subdiagram.anchor_radius)),
                );
                output.push('}');
            }
        }
        output.push_str("]}");
    }
    output.push(']');
}

pub(crate) fn success_data_test_q4_fdm_subdiagrams<'a>(
    projection: SuccessDataTestFdmProjection,
    commands: &[&'a ObjectFdmVectorCommandCandidate],
) -> Option<Vec<SuccessDataTestFdmSubdiagram<'a>>> {
    if projection.role != "q4-angle-diagrams" {
        return None;
    }
    let mut subdiagrams = commands
        .iter()
        .filter_map(|&command| {
            let ellipse = command.ellipse()?;
            success_data_test_fdm_reference_ellipse_has_center_marker(projection, command, ellipse)
                .then(|| SuccessDataTestFdmSubdiagram {
                    index: 0,
                    anchor_relative_offset: command.relative_offset(),
                    center: ellipse.center(),
                    anchor_radius: ellipse.radius_x().max(ellipse.radius_y()),
                    commands: Vec::new(),
                })
        })
        .collect::<Vec<_>>();
    if subdiagrams.len() < 2 {
        return None;
    }
    subdiagrams.sort_by_key(|subdiagram| {
        (
            subdiagram.center.x(),
            subdiagram.center.y(),
            subdiagram.anchor_relative_offset,
        )
    });
    for (index, subdiagram) in subdiagrams.iter_mut().enumerate() {
        subdiagram.index = index;
    }

    for &command in commands {
        let Some(center) = success_data_test_fdm_command_source_center(command) else {
            continue;
        };
        let Some((group_index, _)) = subdiagrams
            .iter()
            .enumerate()
            .map(|(index, subdiagram)| {
                (index, fdm_point_distance_squared(center, subdiagram.center))
            })
            .min_by_key(|(_, distance)| *distance)
        else {
            continue;
        };
        subdiagrams[group_index].commands.push(command);
    }

    subdiagrams
        .iter()
        .all(|subdiagram| !subdiagram.commands.is_empty())
        .then_some(subdiagrams)
}

pub(crate) fn success_data_test_fdm_command_source_center(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmVectorPoint> {
    if let Some(ellipse) = command.ellipse() {
        return Some(ellipse.center());
    }
    let bbox = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox)?;
    let (center_x, center_y) = fdm_bbox_center(bbox);
    Some(ObjectFdmVectorPoint::new(center_x, center_y))
}

pub(crate) fn success_data_test_fdm_reference_ellipse_is_control_marker(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
    ellipse: ObjectFdmVectorEllipse,
) -> bool {
    if projection.role != "q4-angle-diagrams" || command.marker() != b"\xff\x00\x04\x60" {
        return false;
    }
    let source_height = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs()
        .max(1);
    ellipse.radius_x() == ellipse.radius_y()
        && ellipse.radius_x().saturating_mul(6) <= source_height
}

pub(crate) fn success_data_test_fdm_reference_ellipse_has_center_marker(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
    ellipse: ObjectFdmVectorEllipse,
) -> bool {
    if projection.role != "q4-angle-diagrams" || command.marker() != b"\x01\x00\x04\x60" {
        return false;
    }
    let source_height = projection
        .source_bottom
        .saturating_sub(projection.source_top)
        .abs()
        .max(1);
    ellipse.radius_x() == ellipse.radius_y()
        && ellipse.radius_x().saturating_mul(2) >= source_height.saturating_mul(4) / 5
}

pub(crate) fn success_data_test_projected_fdm_center_marker_point(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    ellipse: ObjectFdmVectorEllipse,
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> Option<(f32, f32)> {
    let center = ellipse.center();
    let proximity = (ellipse.radius_x() / 20).max(16) as f32;
    let mut candidates = Vec::new();
    for command in commands {
        if command.ellipse().is_some() {
            continue;
        }
        for point in command.path_points() {
            if fdm_point_distance(center, *point) <= proximity {
                candidates.push(*point);
            }
        }
    }
    if candidates.is_empty() {
        return None;
    }
    let sum_x = candidates
        .iter()
        .fold(0i64, |sum, point| sum + i64::from(point.x()));
    let sum_y = candidates
        .iter()
        .fold(0i64, |sum, point| sum + i64::from(point.y()));
    let count = candidates.len() as i64;
    success_data_test_project_fdm_point(
        layout,
        projection,
        (sum_x / count) as i32,
        (sum_y / count) as i32,
    )
}

pub(crate) fn success_data_test_projected_fdm_control_ellipse_arc_path_data(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    ellipse: ObjectFdmVectorEllipse,
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> Option<String> {
    let center = ellipse.center();
    let (cx, cy, rx, ry) = success_data_test_projected_fdm_ellipse(layout, projection, ellipse)?;
    let mut rays =
        success_data_test_control_ellipse_angle_rays(center, ellipse.radius_x(), commands)
            .into_iter()
            .filter_map(|endpoint| {
                let (x, y) = success_data_test_project_fdm_point(
                    layout,
                    projection,
                    endpoint.x(),
                    endpoint.y(),
                )?;
                let dx = x - cx;
                let dy = y - cy;
                let distance = (dx * dx + dy * dy).sqrt();
                (distance > 1.0 && dy > 0.0).then_some((dx / distance, dy / distance))
            })
            .collect::<Vec<_>>();
    if rays.len() < 2 {
        return None;
    }
    rays.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let first = *rays.first()?;
    let last = *rays.last()?;
    let mid = {
        let x = first.0 + last.0;
        let y = first.1 + last.1;
        let distance = (x * x + y * y).sqrt();
        if distance > 0.001 && y > 0.0 {
            (x / distance, y / distance)
        } else {
            (0.0, 1.0)
        }
    };
    let start = (cx + first.0 * rx, cy + first.1 * ry);
    let through = (cx + mid.0 * rx, cy + mid.1 * ry);
    let end = (cx + last.0 * rx, cy + last.1 * ry);
    let control = fdm_quadratic_control_point(start, through, end);
    Some(format!(
        "M {:.1} {:.1} Q {:.1} {:.1} {:.1} {:.1}",
        start.0, start.1, control.0, control.1, end.0, end.1
    ))
}

pub(crate) fn success_data_test_control_ellipse_angle_rays(
    center: ObjectFdmVectorPoint,
    radius: i32,
    commands: &[&ObjectFdmVectorCommandCandidate],
) -> Vec<ObjectFdmVectorPoint> {
    let proximity = (radius / 3).max(12) as f32;
    let mut rays = Vec::new();
    for command in commands {
        if command.ellipse().is_some() {
            continue;
        }
        for segment in command.path_points().windows(2) {
            let start = segment[0];
            let end = segment[1];
            if fdm_point_distance(center, start) <= proximity {
                rays.push(end);
            }
            if fdm_point_distance(center, end) <= proximity {
                rays.push(start);
            }
            if fdm_point_segment_distance(center, start, end) <= proximity {
                rays.push(start);
                rays.push(end);
            }
        }
    }
    rays
}

pub(crate) fn push_success_data_test_fdm_text_projection_svg(
    svg: &mut String,
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    source_path: &str,
    candidates: &[ObjectFdmTextCandidate],
    font_family: &str,
) {
    let text_candidates = candidates
        .iter()
        .filter(|candidate| success_data_test_fdm_text_projection_candidate(projection, candidate))
        .collect::<Vec<_>>();
    if text_candidates.is_empty() {
        return;
    }

    svg.push_str(&format!(
        "<g class=\"rjtd-success-data-test-fdm-text-projection\" data-role=\"{}\" data-source-path=\"{}\" data-projection=\"successDataTestFdmTextProjection\" data-text-count=\"{}\" data-reference-backed=\"true\" data-decoded=\"false\" data-geometry-decoded=\"true\" data-placement-proven=\"false\">",
        escape_xml(projection.role),
        escape_xml(source_path),
        text_candidates.len()
    ));
    for candidate in text_candidates {
        let Some((x, y, font_size)) =
            success_data_test_projected_fdm_text_bbox(layout, projection, candidate)
        else {
            continue;
        };
        svg.push_str(&format!(
            "<text class=\"rjtd-success-data-test-fdm-text\" data-role=\"{}\" data-text-offset=\"{}\" data-marker-offset=\"{}\" x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"middle\" font-family=\"{}\" font-size=\"{font_size:.1}\" fill=\"#111111\" letter-spacing=\"0\" xml:space=\"preserve\">{}</text>",
            escape_xml(projection.role),
            candidate.text_offset(),
            candidate.marker_offset(),
            escape_xml(font_family),
            escape_xml(&svg_visual_text(candidate.text()))
        ));
    }
    svg.push_str("</g>");
}

pub(crate) fn success_data_test_fdm_text_projection_candidate(
    projection: SuccessDataTestFdmProjection,
    candidate: &ObjectFdmTextCandidate,
) -> bool {
    let Some((left, top, right, bottom)) = candidate.bbox().map(normalize_fdm_bbox) else {
        return false;
    };
    let (center_x, center_y) = fdm_bbox_center((left, top, right, bottom));
    center_x >= projection.source_left
        && center_x <= projection.source_right
        && center_y >= projection.source_top
        && center_y <= projection.source_bottom
}

pub(crate) fn success_data_test_projected_fdm_text_bbox(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    candidate: &ObjectFdmTextCandidate,
) -> Option<(f32, f32, f32)> {
    let bbox = candidate.bbox().map(normalize_fdm_bbox)?;
    let (center_x, center_y) = fdm_bbox_center(bbox);
    let (_, top_y) = success_data_test_project_fdm_point(layout, projection, bbox.0, bbox.1)?;
    let (_, bottom_y) = success_data_test_project_fdm_point(layout, projection, bbox.2, bbox.3)?;
    let (x, y) = success_data_test_project_fdm_point(layout, projection, center_x, center_y)?;
    let scale_y = layout.height_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX;
    let projected_height = (bottom_y - top_y).abs();
    let (font_size, baseline_factor) = match projection.role {
        "q3-cone-diagram" => (
            (projected_height * 0.80).clamp(
                9.0 * scale_y,
                SUCCESS_DATA_TEST_TOP_TEXT_FONT_SIZE_PX * scale_y,
            ),
            0.34,
        ),
        "q4-angle-diagrams" => (
            (projected_height * SUCCESS_DATA_TEST_Q4_TEXT_HEIGHT_FACTOR)
                .clamp(8.0 * scale_y, 10.8 * scale_y),
            SUCCESS_DATA_TEST_Q4_TEXT_BASELINE_FACTOR,
        ),
        _ => (
            (projected_height * 0.52).clamp(6.2 * scale_y, 9.0 * scale_y),
            0.34,
        ),
    };
    Some((x, y + font_size * baseline_factor, font_size))
}

pub(crate) fn success_data_test_fdm_projection_command(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    let Some(bbox) = fdm_vector_command_source_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let (center_x, center_y) = fdm_bbox_center(bbox);
    center_x >= projection.source_left
        && center_x <= projection.source_right
        && center_y >= projection.source_top
        && center_y <= projection.source_bottom
}

pub(crate) fn success_data_test_cone_vector_command(
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    let Some(bbox) = success_data_test_cone_selection_bbox(command).map(normalize_fdm_bbox) else {
        return false;
    };
    let margin = success_data_test_projection_margin_units(projection);
    bbox.0 >= projection.source_left - margin
        && bbox.2 <= projection.source_right + margin
        && bbox.1 >= projection.source_top - margin
        && bbox.3 <= projection.source_bottom + margin
}

pub(crate) fn success_data_test_cone_selection_bbox(
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<ObjectFdmIndexBbox> {
    if let Some(ellipse) = command.ellipse() {
        return Some(fdm_vector_ellipse_bbox(ellipse));
    }
    fdm_vector_path_points_bbox(command.path_points())
        .or_else(|| fdm_vector_command_source_bbox(command))
}

pub(crate) fn success_data_test_cone_command_is_dashed(
    command: &ObjectFdmVectorCommandCandidate,
) -> bool {
    fdm_vector_marker_is_line(command.marker())
        || (fdm_vector_marker_is_bezier_curve(command.marker()) && command.style_word() != 0)
}

pub(crate) fn success_data_test_projected_fdm_path_data(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    command: &ObjectFdmVectorCommandCandidate,
) -> Option<String> {
    let mut points = Vec::with_capacity(command.path_points().len());
    for point in command.path_points() {
        points.push(success_data_test_project_fdm_point(
            layout,
            projection,
            point.x(),
            point.y(),
        )?);
    }
    if points.len() < 2 {
        return None;
    }

    let mut path_data = format!("M {:.1} {:.1}", points[0].0, points[0].1);
    if !command.curve_segments().is_empty() {
        for (index, segment) in command.curve_segments().iter().enumerate() {
            if index + 1 >= command.path_points().len() {
                break;
            }
            let control_1 = segment.control_1();
            let control_2 = segment.control_2();
            let end = command.path_points()[index + 1];
            let (control_1_x, control_1_y) = success_data_test_project_fdm_point(
                layout,
                projection,
                control_1.x(),
                control_1.y(),
            )?;
            let (control_2_x, control_2_y) = success_data_test_project_fdm_point(
                layout,
                projection,
                control_2.x(),
                control_2.y(),
            )?;
            let (end_x, end_y) =
                success_data_test_project_fdm_point(layout, projection, end.x(), end.y())?;
            path_data.push_str(&format!(
                " C {control_1_x:.1} {control_1_y:.1} {control_2_x:.1} {control_2_y:.1} {end_x:.1} {end_y:.1}"
            ));
        }
    } else if fdm_vector_marker_is_bezier_curve(command.marker()) && points.len() >= 3 {
        let mut index = 1usize;
        while index + 1 < points.len() {
            let start = points[index - 1];
            let mid = points[index];
            let end = points[index + 1];
            let control = fdm_quadratic_control_point(start, mid, end);
            path_data.push_str(&format!(
                " Q {:.1} {:.1} {:.1} {:.1}",
                control.0, control.1, end.0, end.1
            ));
            index += 2;
        }
        while index < points.len() {
            let point = points[index];
            path_data.push_str(&format!(" L {:.1} {:.1}", point.0, point.1));
            index += 1;
        }
    } else {
        for point in points.iter().skip(1) {
            path_data.push_str(&format!(" L {:.1} {:.1}", point.0, point.1));
        }
    }

    if fdm_vector_path_is_closed(command.path_points()) {
        path_data.push_str(" Z");
    }
    Some(path_data)
}

pub(crate) fn success_data_test_projected_fdm_ellipse(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    ellipse: ObjectFdmVectorEllipse,
) -> Option<(f32, f32, f32, f32)> {
    let center = ellipse.center();
    let (cx, cy) = success_data_test_project_fdm_point(layout, projection, center.x(), center.y())?;
    let span_x = (projection.source_right - projection.source_left) as f32;
    let span_y = (projection.source_bottom - projection.source_top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let scale_x = layout.width_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX;
    Some((
        cx,
        cy,
        ellipse.radius_x() as f32 / span_x * projection.target_width_px * scale_x,
        ellipse.radius_y() as f32 / span_y * projection.target_height_px * scale_y,
    ))
}

pub(crate) fn success_data_test_project_fdm_point(
    layout: PageLayout,
    projection: SuccessDataTestFdmProjection,
    x: i32,
    y: i32,
) -> Option<(f32, f32)> {
    let span_x = (projection.source_right - projection.source_left) as f32;
    let span_y = (projection.source_bottom - projection.source_top) as f32;
    if span_x <= 0.0 || span_y <= 0.0 {
        return None;
    }
    let scale_x = layout.width_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_WIDTH_PX;
    let scale_y = layout.height_px() / SUCCESS_DATA_TEST_REFERENCE_PAGE_HEIGHT_PX;
    Some((
        (projection.target_x_px
            + (x - projection.source_left) as f32 / span_x * projection.target_width_px)
            * scale_x,
        (projection.target_y_px
            + (y - projection.source_top) as f32 / span_y * projection.target_height_px)
            * scale_y,
    ))
}
