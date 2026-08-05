use super::*;
use crate::*;

pub(crate) fn success_data_test_title_art_shadow_texture_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Texture
                && !path.commands().is_empty()
                && embedded_press_title_art_state_word5(path)
                    == Some(EMBEDDED_PRESS_TITLE_ART_SHADOW_STATE_WORD5)
        })
        .collect::<Vec<_>>()
}

pub(crate) fn success_data_test_title_art_interstitial_texture_paths<'a>(
    snapshot: &'a ObjectEmbeddedPressSnapshotCandidate,
    partition: &TitleArtShadowPathPartition<'a>,
) -> Option<Vec<&'a ObjectEmbeddedPressVectorPathCandidate>> {
    let paths = snapshot.vector_paths();
    let shadow_max_index = partition
        .shadow_paths
        .iter()
        .filter_map(|target| embedded_press_vector_path_index(paths, target))
        .max()?;
    let main_min_index = partition
        .main_paths
        .iter()
        .filter_map(|target| embedded_press_vector_path_index(paths, target))
        .min()?;
    if shadow_max_index + 1 >= main_min_index {
        return None;
    }

    let texture_paths = paths[shadow_max_index + 1..main_min_index]
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Texture && !path.commands().is_empty()
        })
        .collect::<Vec<_>>();
    (texture_paths.len() == main_min_index - shadow_max_index - 1).then_some(texture_paths)
}

pub(crate) fn push_success_data_test_title_art_paint_state_summaries_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let outline_paths = success_data_test_title_art_rendered_paths(snapshot);
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let mut groups: Vec<(&str, Vec<&ObjectEmbeddedPressVectorPathCandidate>)> = Vec::new();
    if let Some(partition) = partition.as_ref() {
        groups.push(("shadowOutlines", partition.shadow_paths.clone()));
        if let Some(texture_paths) =
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        {
            groups.push(("interstitialTextureBlock", texture_paths));
        } else {
            groups.push((
                "preservedAllTexturePaths",
                success_data_test_title_art_texture_paths(snapshot),
            ));
        }
        groups.push(("mainOutlines", partition.main_paths.clone()));
    } else {
        groups.push(("allOutlines", outline_paths));
        groups.push((
            "preservedAllTexturePaths",
            success_data_test_title_art_texture_paths(snapshot),
        ));
    }

    output.push('[');
    for (index, (role, paths)) in groups.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_success_data_test_title_art_paint_state_summary_json(output, snapshot, role, paths);
    }
    output.push(']');
}

pub(crate) fn push_success_data_test_title_art_paint_state_sequence_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let paths = snapshot.vector_paths();
    let explicit_path_indexes = paths
        .iter()
        .enumerate()
        .filter(|(_, path)| !path.commands().is_empty() && !path.state_records().is_empty())
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();

    output.push_str("{\"source\":\"embeddedPressVectorPathSourceOrder\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false");
    output.push_str(",\"pathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"explicitTransitionCount\":");
    output.push_str(&explicit_path_indexes.len().to_string());
    output.push_str(",\"pathKindRuns\":");
    push_success_data_test_title_art_path_kind_runs_json(output, snapshot);
    output.push_str(",\"frontErasePaintTransitionGate\":");
    push_success_data_test_title_art_front_erase_paint_transition_gate_json(
        output,
        snapshot,
        &interstitial_texture_paths,
    );
    output.push_str(",\"explicitTransitions\":[");
    for (transition_index, path_index) in explicit_path_indexes.iter().enumerate() {
        if transition_index > 0 {
            output.push(',');
        }
        let path = &paths[*path_index];
        let next_explicit_path_index = explicit_path_indexes.get(transition_index + 1).copied();
        let inherited_span_end_path_index = next_explicit_path_index
            .map(|index| index.saturating_sub(1))
            .unwrap_or_else(|| paths.len().saturating_sub(1));
        let inherited_span = if *path_index <= inherited_span_end_path_index {
            &paths[*path_index..=inherited_span_end_path_index]
        } else {
            &[]
        };
        let inherited_texture_path_count = inherited_span
            .iter()
            .filter(|path| {
                path.kind() == ObjectEmbeddedPressVectorPathKind::Texture
                    && !path.commands().is_empty()
            })
            .count();
        let inherited_outline_path_count = inherited_span
            .iter()
            .filter(|path| {
                path.kind() == ObjectEmbeddedPressVectorPathKind::Outline
                    && !path.commands().is_empty()
            })
            .count();

        output.push_str("{\"pathIndex\":");
        output.push_str(&path_index.to_string());
        output.push_str(",\"pathKind\":");
        output.push_str(&json_string(path.kind().as_str()));
        output.push_str(",\"sourceOrderRole\":");
        output.push_str(&json_string(success_data_test_title_art_source_order_role(
            path,
            partition.as_ref(),
            &interstitial_texture_paths,
        )));
        output.push_str(",\"stateSourcePathIndex\":");
        output.push_str(&path_index.to_string());
        output.push_str(",\"nextExplicitPathIndex\":");
        push_option_usize_json(output, next_explicit_path_index);
        output.push_str(",\"inheritedSpanEndPathIndex\":");
        output.push_str(&inherited_span_end_path_index.to_string());
        output.push_str(",\"inheritedPathCount\":");
        output.push_str(&inherited_span.len().to_string());
        output.push_str(",\"inheritedTexturePathCount\":");
        output.push_str(&inherited_texture_path_count.to_string());
        output.push_str(",\"inheritedOutlinePathCount\":");
        output.push_str(&inherited_outline_path_count.to_string());
        output.push_str(",\"stateRecordCount\":");
        output.push_str(&path.state_records().len().to_string());
        output.push_str(",\"stateRecordTypes\":");
        push_u32_hex_array_json(
            output,
            &path
                .state_records()
                .iter()
                .map(ObjectEmbeddedPressStateRecordCandidate::record_type)
                .collect::<Vec<_>>(),
        );
        output.push_str(",\"stateRecords\":");
        push_embedded_press_path_state_records_json(output, path);
        output.push_str(",\"record46Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x46, 0);
        output.push_str(",\"record48Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x48, 0);
        output.push_str(",\"record60Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x60, 0);
        output.push_str(",\"record65Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x65, 0);
        output.push_str(",\"record70Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x70, 0);
        output.push_str(",\"record70Word3Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x70, 3);
        output.push_str(",\"record70Word7Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(output, path, 0x70, 7);
        output.push_str(",\"record82Word0Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(
            output,
            path,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            0,
        );
        output.push_str(",\"record82Word3Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(
            output,
            path,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            3,
        );
        output.push_str(",\"record82Word5Sequence\":");
        push_embedded_press_single_path_state_word_sequence_json(
            output,
            path,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        );
        output.push_str(",\"textureBezierHeader\":");
        push_embedded_press_path_texture_bezier_header_json(output, path);
        output.push('}');
    }
    output.push_str("]}");
}

pub(crate) fn success_data_test_title_art_front_erase_paint_transition_gate(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    interstitial_texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> TitleArtFrontErasePaintTransitionGate {
    let snapshot_paths = snapshot.vector_paths();
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_indexes =
        embedded_press_vector_path_indexes(snapshot_paths, interstitial_texture_paths);
    let shadow_indexes = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_vector_path_indexes(snapshot_paths, &partition.shadow_paths)
    });
    let main_indexes = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_vector_path_indexes(snapshot_paths, &partition.main_paths)
    });
    let explicit_state_texture_path_count = interstitial_texture_paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();
    let spans = embedded_press_title_art_front_erase_texture_state_spans(
        snapshot,
        interstitial_texture_paths,
    );
    let span_path_counts = spans.iter().map(|span| span.path_count).collect::<Vec<_>>();

    let shadow_values_48 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x48)
    });
    let texture_values_48 =
        embedded_press_title_art_state_record_word0_values(interstitial_texture_paths, 0x48);
    let main_values_48 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
    });
    let texture_values_70 = embedded_press_title_art_state_record_word0_values(
        interstitial_texture_paths,
        EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70,
    );
    let main_values_70 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(
            &partition.main_paths,
            EMBEDDED_PRESS_RECORD_PAINT_EFFECT_70,
        )
    });
    let shadow_values_82_word5 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word_values(
            &partition.shadow_paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        )
    });
    let texture_values_82_word5 = embedded_press_title_art_state_record_word_values(
        interstitial_texture_paths,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        5,
    );
    let main_values_82_word5 = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word_values(
            &partition.main_paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        )
    });
    let texture_values_82_word3 = embedded_press_title_art_state_record_word_values(
        interstitial_texture_paths,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        3,
    );

    let shadow_last_path_index = shadow_indexes.iter().max().copied();
    let interstitial_first_path_index = interstitial_indexes.iter().min().copied();
    let interstitial_last_path_index = interstitial_indexes.iter().max().copied();
    let main_first_path_index = main_indexes.iter().min().copied();
    let shadow_to_interstitial_boundary_adjacent =
        match (shadow_last_path_index, interstitial_first_path_index) {
            (Some(shadow_last), Some(texture_first)) => shadow_last + 1 == texture_first,
            _ => false,
        };
    let interstitial_to_main_boundary_adjacent =
        match (interstitial_last_path_index, main_first_path_index) {
            (Some(texture_last), Some(main_first)) => texture_last + 1 == main_first,
            _ => false,
        };
    let record48_separates_shadow_from_texture_and_main =
        shadow_values_48 == vec![1] && texture_values_48 == vec![0] && main_values_48 == vec![0];
    let record48_separates_texture_from_main =
        !texture_values_48.is_empty() && texture_values_48 != main_values_48;
    let record70_word0_separates_texture_from_main =
        !texture_values_70.is_empty() && texture_values_70 != main_values_70;
    let record82_word5_separates_texture_from_main =
        !texture_values_82_word5.is_empty() && texture_values_82_word5 != main_values_82_word5;
    let record82_word5_matches_shadow =
        !texture_values_82_word5.is_empty() && texture_values_82_word5 == shadow_values_82_word5;
    let record82_word3_is_white_paint_candidate = texture_values_82_word3 == vec![0x00ff_ffff];

    let paint_intent_inference = if record82_word5_matches_shadow
        && record82_word5_separates_texture_from_main
        && !record48_separates_texture_from_main
        && !record70_word0_separates_texture_from_main
    {
        "shadow-state-texture-inside-main-boundary-ambiguous"
    } else if record48_separates_texture_from_main || record70_word0_separates_texture_from_main {
        "texture-main-state-separated-candidate"
    } else if record82_word3_is_white_paint_candidate {
        "white-paint-candidate-without-boundary-separation"
    } else {
        "paint-intent-unclassified"
    };
    let transition_boundary_class = if partition.is_none() {
        "title-partition-missing"
    } else if interstitial_texture_paths.is_empty() {
        "interstitial-texture-absent"
    } else if shadow_to_interstitial_boundary_adjacent && interstitial_to_main_boundary_adjacent {
        "source-order-bracketed-interstitial-texture-block"
    } else {
        "source-order-boundary-not-contiguous"
    };
    let render_promotion_blocked_reason = if partition.is_none() {
        "front-erase-title-partition-missing"
    } else if interstitial_texture_paths.is_empty() {
        "front-erase-interstitial-texture-absent"
    } else if !(shadow_to_interstitial_boundary_adjacent && interstitial_to_main_boundary_adjacent)
    {
        "front-erase-source-order-boundary-not-contiguous"
    } else if !record48_separates_texture_from_main && !record70_word0_separates_texture_from_main {
        "front-erase-transition-boundary-main-state-not-separated"
    } else {
        "front-erase-transition-boundary-semantics-unproven"
    };

    TitleArtFrontErasePaintTransitionGate {
        partition_present: partition.is_some(),
        interstitial_texture_path_count: interstitial_texture_paths.len(),
        explicit_state_texture_path_count,
        inherited_texture_path_count: interstitial_texture_paths
            .len()
            .saturating_sub(explicit_state_texture_path_count),
        span_count: spans.len(),
        span_path_counts,
        shadow_last_path_index,
        interstitial_first_path_index,
        interstitial_last_path_index,
        main_first_path_index,
        shadow_to_interstitial_boundary_adjacent,
        interstitial_to_main_boundary_adjacent,
        record48_separates_shadow_from_texture_and_main,
        record48_separates_texture_from_main,
        record70_word0_separates_texture_from_main,
        record82_word5_separates_texture_from_main,
        record82_word5_matches_shadow,
        record82_word3_is_white_paint_candidate,
        paint_intent_inference,
        transition_boundary_class,
        render_promotion_blocked_reason,
    }
}

pub(crate) fn push_success_data_test_title_art_front_erase_paint_transition_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    interstitial_texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let gate = success_data_test_title_art_front_erase_paint_transition_gate(
        snapshot,
        interstitial_texture_paths,
    );
    output.push_str("{\"source\":\"embeddedPressVectorPathSourceOrder+stateTransitions\",\"decoded\":false,\"sourceBacked\":true,\"diagnosticOnly\":true");
    output.push_str(",\"partitionPresent\":");
    output.push_str(if gate.partition_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"interstitialTexturePathCount\":");
    output.push_str(&gate.interstitial_texture_path_count.to_string());
    output.push_str(",\"explicitStateTexturePathCount\":");
    output.push_str(&gate.explicit_state_texture_path_count.to_string());
    output.push_str(",\"inheritedTexturePathCount\":");
    output.push_str(&gate.inherited_texture_path_count.to_string());
    output.push_str(",\"spanCount\":");
    output.push_str(&gate.span_count.to_string());
    output.push_str(",\"spanPathCounts\":");
    push_usize_array_json(output, &gate.span_path_counts);
    output.push_str(",\"shadowLastPathIndex\":");
    push_option_usize_json(output, gate.shadow_last_path_index);
    output.push_str(",\"interstitialFirstPathIndex\":");
    push_option_usize_json(output, gate.interstitial_first_path_index);
    output.push_str(",\"interstitialLastPathIndex\":");
    push_option_usize_json(output, gate.interstitial_last_path_index);
    output.push_str(",\"mainFirstPathIndex\":");
    push_option_usize_json(output, gate.main_first_path_index);
    output.push_str(",\"shadowToInterstitialBoundaryAdjacent\":");
    output.push_str(if gate.shadow_to_interstitial_boundary_adjacent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"interstitialToMainBoundaryAdjacent\":");
    output.push_str(if gate.interstitial_to_main_boundary_adjacent {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesShadowFromTextureAndMain\":");
    output.push_str(if gate.record48_separates_shadow_from_texture_and_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesTextureFromMain\":");
    output.push_str(if gate.record48_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word0SeparatesTextureFromMain\":");
    output.push_str(if gate.record70_word0_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record82Word5SeparatesTextureFromMain\":");
    output.push_str(if gate.record82_word5_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record82Word5MatchesShadow\":");
    output.push_str(if gate.record82_word5_matches_shadow {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record82Word3IsWhitePaintCandidate\":");
    output.push_str(if gate.record82_word3_is_white_paint_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"paintIntentInference\":");
    output.push_str(&json_string(gate.paint_intent_inference));
    output.push_str(",\"transitionBoundaryClass\":");
    output.push_str(&json_string(gate.transition_boundary_class));
    output.push_str(",\"promotionReady\":");
    output.push_str(if gate.promotion_ready() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(gate.render_promotion_blocked_reason));
    output.push('}');
}

pub(crate) fn push_success_data_test_title_art_path_kind_runs_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let mut runs: Vec<(ObjectEmbeddedPressVectorPathKind, usize, usize, usize)> = Vec::new();
    for (path_index, path) in snapshot.vector_paths().iter().enumerate() {
        if path.commands().is_empty() {
            continue;
        }
        match runs.last_mut() {
            Some((kind, _, end, count)) if *kind == path.kind() => {
                *end = path_index;
                *count += 1;
            }
            _ => runs.push((path.kind(), path_index, path_index, 1)),
        }
    }

    output.push('[');
    for (run_index, (kind, start, end, count)) in runs.iter().enumerate() {
        if run_index > 0 {
            output.push(',');
        }
        output.push_str("{\"pathKind\":");
        output.push_str(&json_string(kind.as_str()));
        output.push_str(",\"startPathIndex\":");
        output.push_str(&start.to_string());
        output.push_str(",\"endPathIndex\":");
        output.push_str(&end.to_string());
        output.push_str(",\"pathCount\":");
        output.push_str(&count.to_string());
        output.push('}');
    }
    output.push(']');
}

pub(crate) fn success_data_test_title_art_source_order_role(
    path: &ObjectEmbeddedPressVectorPathCandidate,
    partition: Option<&TitleArtShadowPathPartition<'_>>,
    interstitial_texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) -> &'static str {
    let Some(partition) = partition else {
        return "unpartitioned";
    };
    if partition
        .shadow_paths
        .iter()
        .any(|candidate| std::ptr::eq(*candidate, path))
    {
        "shadowOutlines"
    } else if interstitial_texture_paths
        .iter()
        .any(|candidate| std::ptr::eq(*candidate, path))
    {
        "interstitialTextureBlock"
    } else if partition
        .main_paths
        .iter()
        .any(|candidate| std::ptr::eq(*candidate, path))
    {
        "mainOutlines"
    } else {
        "outsideTitlePartition"
    }
}

pub(crate) fn push_success_data_test_title_art_paint_state_summary_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    role: &str,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let snapshot_paths = snapshot.vector_paths();
    let path_indexes = paths
        .iter()
        .filter_map(|path| embedded_press_vector_path_index(snapshot_paths, path))
        .collect::<Vec<_>>();
    let explicit_state_path_count = paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();
    let state_record_count = paths
        .iter()
        .map(|path| path.state_records().len())
        .sum::<usize>();
    let path_kind = embedded_press_title_art_path_kind_summary(paths);

    output.push_str("{\"role\":");
    output.push_str(&json_string(role));
    output.push_str(",\"pathKind\":");
    output.push_str(&json_string(path_kind));
    output.push_str(",\"pathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"firstPathIndex\":");
    push_option_usize_json(output, path_indexes.iter().min().copied());
    output.push_str(",\"lastPathIndex\":");
    push_option_usize_json(output, path_indexes.iter().max().copied());
    output.push_str(",\"explicitStatePathCount\":");
    output.push_str(&explicit_state_path_count.to_string());
    output.push_str(",\"inheritedStatePathCount\":");
    output.push_str(
        &paths
            .len()
            .saturating_sub(explicit_state_path_count)
            .to_string(),
    );
    output.push_str(",\"stateRecordCount\":");
    output.push_str(&state_record_count.to_string());
    output.push_str(",\"statePayloadSignatures\":");
    push_embedded_press_state_payload_signatures_json(output, paths);
    output.push_str(",\"statePayloadWordColumns\":");
    push_embedded_press_state_payload_word_columns_json(output, paths);
    output.push('}');
}

pub(crate) fn push_success_data_test_title_art_front_paint_candidate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    source_paint_candidate: Option<&ObjectJsfartArtPaintCandidate>,
) {
    let front_texture_paths = success_data_test_title_art_front_texture_paths(snapshot);
    let front_erase_texture_paths = success_data_test_title_art_front_erase_texture_paths(snapshot);
    let source_paint_color = source_paint_candidate.and_then(jsfart_paint_candidate_color_hex);
    let paint_state_color = embedded_press_snapshot_paint_state_color_hex(snapshot);
    let (paint_color, paint_source) = success_data_test_title_art_front_paint_color_candidate(
        source_paint_color.as_ref(),
        paint_state_color.as_ref(),
    );
    let color_gate =
        success_data_test_title_art_front_fill_render_color_gate(paint_color, paint_source);
    let render_path_count = if front_texture_paths.is_empty() {
        front_erase_texture_paths.len()
    } else {
        front_texture_paths.len()
    };
    let render_texture_path_source =
        if front_texture_paths.is_empty() && !front_erase_texture_paths.is_empty() {
            "source-order-interstitial-front-erase-texture"
        } else if !front_texture_paths.is_empty() {
            "main-state-texture-paths"
        } else {
            "none"
        };
    let render_promotion_blocked_reason =
        success_data_test_title_art_front_texture_render_promotion_blocked_reason(
            render_texture_path_source,
        );
    let visible_render_path_count = if render_promotion_blocked_reason.is_some() {
        0
    } else {
        render_path_count
    };
    let render_blocked_reason = if render_path_count == 0
        && (source_paint_color.is_some() || paint_state_color.is_some())
    {
        Some("no-main-state-or-front-owned-texture-paths")
    } else if source_paint_color.is_none() && paint_state_color.is_none() {
        Some("missing-source-paint-color")
    } else {
        render_promotion_blocked_reason
    };
    let direct_gray_candidate_present =
        embedded_press_title_art_direct_gray_candidate(&front_erase_texture_paths).is_some();
    let texture_source_paint_candidate_present = embedded_press_title_art_source_paint_candidate(
        &front_erase_texture_paths,
        source_paint_candidate,
    )
    .is_some();
    let texture_state_span_count = embedded_press_title_art_front_erase_texture_state_spans(
        snapshot,
        &front_erase_texture_paths,
    )
    .len();
    let transition_gate = success_data_test_title_art_front_erase_paint_transition_gate(
        snapshot,
        &front_erase_texture_paths,
    );
    let candidate_count = usize::from(color_gate.paint_color.is_some())
        + usize::from(direct_gray_candidate_present)
        + usize::from(texture_source_paint_candidate_present)
        + usize::from(texture_state_span_count > 0);

    output.push_str("{\"source\":\"JSFart2Contents+EmbeddedPressPaintState\",\"decoded\":false,\"sourceBacked\":");
    output.push_str(
        if source_paint_color.is_some() || paint_state_color.is_some() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"paintColor\":");
    if let Some(color) = color_gate.paint_color {
        output.push_str(&json_string(color));
    } else {
        output.push_str("null");
    }
    output.push_str(",\"paintColorSource\":");
    match color_gate.paint_source {
        Some(source) => output.push_str(&json_string(source)),
        None => output.push_str("null"),
    }
    output.push_str(",\"renderFillColor\":");
    output.push_str(&json_string(color_gate.render_fill));
    output.push_str(",\"renderFillColorSource\":");
    output.push_str(&json_string(color_gate.render_color_source));
    output.push_str(",\"renderFillColorSourceBacked\":");
    output.push_str(if color_gate.render_color_source_backed {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePaintColorMatchesRenderFill\":");
    output.push_str(if color_gate.source_paint_matches_render_fill {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderFillColorPromotionBlockedReason\":");
    output.push_str(&json_string(color_gate.render_color_blocked_reason));
    output.push_str(",\"sourcePaintRenderTrace\":");
    push_success_data_test_title_art_source_paint_render_trace_json(
        output,
        source_paint_candidate,
        color_gate,
        render_texture_path_source,
        render_blocked_reason,
    );
    output.push_str(",\"frontPaintArbitrationGate\":");
    output.push_str("{\"source\":\"JSFart2Contents+EmbeddedPressPaintState+frontEraseTextureProbes\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"candidateCount\":");
    output.push_str(&candidate_count.to_string());
    output.push_str(",\"selectedRenderPolicy\":\"conservative-front-fill\"");
    output.push_str(",\"selectedRenderFillColor\":");
    output.push_str(&json_string(color_gate.render_fill));
    output.push_str(",\"sourcePaintCandidatePresent\":");
    output.push_str(if color_gate.paint_color.is_some() {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourcePaintCandidateMatchesRenderFill\":");
    output.push_str(if color_gate.source_paint_matches_render_fill {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"directGrayCandidatePresent\":");
    output.push_str(if direct_gray_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textureSourcePaintCandidatePresent\":");
    output.push_str(if texture_source_paint_candidate_present {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"frontEraseTextureSpanCandidatePresent\":");
    output.push_str(if texture_state_span_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"frontEraseTextureSpanCount\":");
    output.push_str(&texture_state_span_count.to_string());
    output.push_str(",\"frontEraseTransitionBoundaryClass\":");
    output.push_str(&json_string(transition_gate.transition_boundary_class));
    output.push_str(",\"frontErasePaintIntentInference\":");
    output.push_str(&json_string(transition_gate.paint_intent_inference));
    output.push_str(",\"renderPromotionBlockedReasons\":[");
    output.push_str(&json_string(color_gate.render_color_blocked_reason));
    output.push(',');
    output.push_str(&json_string(
        render_promotion_blocked_reason.unwrap_or("none"),
    ));
    output.push(',');
    output.push_str(&json_string(
        transition_gate.render_promotion_blocked_reason,
    ));
    output.push(']');
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"front-paint-candidate-arbitration-unproven\"}",
    );
    output.push_str(",\"mainStateTexturePathCount\":");
    output.push_str(&front_texture_paths.len().to_string());
    output.push_str(",\"frontEraseTexturePathCount\":");
    output.push_str(&front_erase_texture_paths.len().to_string());
    output.push_str(",\"renderTexturePathSource\":");
    output.push_str(&json_string(render_texture_path_source));
    output.push_str(",\"renderPathCount\":");
    output.push_str(&render_path_count.to_string());
    output.push_str(",\"visibleRenderPathCount\":");
    output.push_str(&visible_render_path_count.to_string());
    output.push_str(",\"renderClipRule\":");
    output.push_str(&json_string("nonzero"));
    output.push_str(",\"renderClipRuleSource\":");
    output.push_str(&json_string("embedded-press-nonzero-winding"));
    output.push_str(",\"renderClipRulePixelChange\":true");
    output.push_str(",\"renderPromotionBlockedReason\":");
    if let Some(reason) = render_blocked_reason {
        output.push_str(&json_string(reason));
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn push_success_data_test_title_art_front_texture_role_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    front_erase_texture_paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();

    output.push_str(
        "{\"source\":\"embeddedPressPathStateRecordComparison\",\"decoded\":false,\"pixelChange\":",
    );
    output.push_str(if front_erase_texture_paths.is_empty() {
        "false"
    } else {
        "true"
    });
    output.push_str(",\"frontEraseTexturePathCount\":");
    output.push_str(&front_erase_texture_paths.len().to_string());
    output.push_str(",\"interstitialTexturePathCount\":");
    output.push_str(&interstitial_texture_paths.len().to_string());

    let record_48_separates_shadow_from_texture_and_main =
        partition.as_ref().is_some_and(|partition| {
            embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x48)
                == vec![1]
                && embedded_press_title_art_state_record_word0_values(
                    &interstitial_texture_paths,
                    0x48,
                ) == vec![0]
                && embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
                    == vec![0]
        });
    let record_48_separates_texture_from_main = partition.as_ref().is_some_and(|partition| {
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x48)
            != embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
    });
    let source_order_front_erase_candidate = partition.as_ref().is_some_and(|partition| {
        success_data_test_title_art_interstitial_front_erase_gate(
            snapshot,
            partition,
            front_erase_texture_paths,
        )
    });
    let blocked_interstitial_current_state_candidate = front_erase_texture_paths.is_empty()
        && !interstitial_texture_paths.is_empty()
        && record_48_separates_shadow_from_texture_and_main
        && !record_48_separates_texture_from_main;
    output.push_str(",\"record48SeparatesShadowFromTextureAndMain\":");
    output.push_str(if record_48_separates_shadow_from_texture_and_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesTextureFromMain\":");
    output.push_str(if record_48_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOrderFrontEraseCandidate\":");
    output.push_str(if source_order_front_erase_candidate {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(if source_order_front_erase_candidate {
        "source-order-interstitial-front-erase-texture"
    } else if blocked_interstitial_current_state_candidate {
        "blocked-current-paint-state-inheritance"
    } else if front_erase_texture_paths
        .iter()
        .all(|path| !path.state_records().is_empty())
        && !front_erase_texture_paths.is_empty()
    {
        "explicit-state-texture-paths"
    } else if front_erase_texture_paths.is_empty() {
        "none"
    } else {
        "current-paint-state-inheritance"
    }));
    let source_order_front_erase_render_promotion_blocked_reason =
        if source_order_front_erase_candidate {
            success_data_test_title_art_front_texture_render_promotion_blocked_reason(
                "source-order-interstitial-front-erase-texture",
            )
        } else {
            None
        };
    let source_order_front_erase_render_promoted = source_order_front_erase_candidate
        && source_order_front_erase_render_promotion_blocked_reason.is_none();
    output.push_str(",\"visibleRenderPathCount\":");
    output.push_str(
        &(if source_order_front_erase_render_promoted {
            front_erase_texture_paths.len()
        } else {
            0
        })
        .to_string(),
    );
    output.push_str(",\"renderPromoted\":");
    output.push_str(if source_order_front_erase_render_promoted {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"frontTexturePromotionBasis\":");
    output.push_str(&json_string(if source_order_front_erase_candidate {
        "source-order-interstitial-front-erase-texture"
    } else if front_erase_texture_paths
        .iter()
        .all(|path| !path.state_records().is_empty())
        && !front_erase_texture_paths.is_empty()
    {
        "explicit-state-texture-paths"
    } else if blocked_interstitial_current_state_candidate {
        "blocked-current-paint-state-inheritance"
    } else if front_erase_texture_paths.is_empty() {
        "none"
    } else {
        "current-paint-state-inheritance"
    }));
    output.push_str(",\"frontTexturePromotionRisk\":");
    output.push_str(&json_string(if source_order_front_erase_candidate {
        "source-order-texture-shares-record48-with-main-outline"
    } else if blocked_interstitial_current_state_candidate {
        "interstitial-texture-and-main-outline-share-record48-zero"
    } else if !front_erase_texture_paths.is_empty()
        && record_48_separates_shadow_from_texture_and_main
        && !record_48_separates_texture_from_main
    {
        "front-texture-and-main-outline-share-record48-zero"
    } else {
        "none"
    }));
    output.push_str(",\"renderPromotionBlockedReason\":");
    if source_order_front_erase_render_promoted {
        output.push_str("null");
    } else if let Some(reason) = source_order_front_erase_render_promotion_blocked_reason {
        output.push_str(&json_string(reason));
    } else {
        output.push_str(&json_string(
            if blocked_interstitial_current_state_candidate {
                "interstitial-texture-and-main-outline-share-record48-zero"
            } else if front_erase_texture_paths.is_empty() {
                "no-front-erase-texture-candidate"
            } else {
                "front-erase-texture-role-unproven"
            },
        ));
    }
    output.push_str(",\"frontEraseVisibleProbeGate\":");
    push_success_data_test_title_art_front_erase_visible_probe_gate_json(
        output,
        front_erase_texture_paths,
    );
    output.push_str(",\"groups\":[");
    if let Some(partition) = partition.as_ref() {
        push_success_data_test_title_art_role_gate_group_json(
            output,
            "shadowOutlines",
            &partition.shadow_paths,
        );
        output.push(',');
        push_success_data_test_title_art_role_gate_group_json(
            output,
            "interstitialTextureBlock",
            &interstitial_texture_paths,
        );
        output.push(',');
        push_success_data_test_title_art_role_gate_group_json(
            output,
            "mainOutlines",
            &partition.main_paths,
        );
    }
    output.push_str("]}");
}

pub(crate) fn push_success_data_test_title_art_texture_paint_phase_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();
    let shadow_record46_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x46)
    });
    let texture_record46_values =
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x46);
    let main_record46_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x46)
    });
    let shadow_record48_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x48)
    });
    let texture_record48_values =
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x48);
    let main_record48_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x48)
    });
    let shadow_record60_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x60)
    });
    let texture_record60_values =
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x60);
    let main_record60_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x60)
    });
    let shadow_record65_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.shadow_paths, 0x65)
    });
    let texture_record65_values =
        embedded_press_title_art_state_record_word0_values(&interstitial_texture_paths, 0x65);
    let main_record65_values = partition.as_ref().map_or_else(Vec::new, |partition| {
        embedded_press_title_art_state_record_word0_values(&partition.main_paths, 0x65)
    });
    let record46_one_appears_only_on_outlines = (shadow_record46_values.contains(&1)
        || main_record46_values.contains(&1))
        && !texture_record46_values.contains(&1);
    let texture_and_main_share_record46_zero =
        texture_record46_values.contains(&0) && main_record46_values.contains(&0);
    let texture_and_shadow_share_record46_zero =
        texture_record46_values.contains(&0) && shadow_record46_values.contains(&0);
    let record46_separates_texture_from_outlines = !texture_record46_values.is_empty()
        && !shadow_record46_values.is_empty()
        && !main_record46_values.is_empty()
        && texture_record46_values.iter().all(|value| {
            !shadow_record46_values.contains(value) && !main_record46_values.contains(value)
        });
    let record48_separates_shadow_from_texture_and_main = !shadow_record48_values.is_empty()
        && !texture_record48_values.is_empty()
        && !main_record48_values.is_empty()
        && shadow_record48_values != texture_record48_values
        && shadow_record48_values != main_record48_values;
    let record48_separates_texture_from_main =
        !texture_record48_values.is_empty() && texture_record48_values != main_record48_values;
    let record60_shared_across_roles = !texture_record60_values.is_empty()
        && shadow_record60_values == texture_record60_values
        && texture_record60_values == main_record60_values;
    let record65_shared_across_roles = !texture_record65_values.is_empty()
        && shadow_record65_values == texture_record65_values
        && texture_record65_values == main_record65_values;
    let mut promotion_proof_blocked_reasons = Vec::new();
    if partition.is_none() {
        promotion_proof_blocked_reasons.push("title-art-role-partition-missing");
    }
    if interstitial_texture_paths.is_empty() {
        promotion_proof_blocked_reasons.push("interstitial-texture-paths-missing");
    }
    if !record46_one_appears_only_on_outlines {
        promotion_proof_blocked_reasons.push("record46-outline-candidate-absent");
    }
    if !record46_separates_texture_from_outlines {
        promotion_proof_blocked_reasons
            .push("record46-texture-outline-value-sets-overlap-or-missing");
    }
    if texture_and_main_share_record46_zero {
        promotion_proof_blocked_reasons.push("record46-zero-shared-by-texture-and-main-outline");
    }
    if texture_and_shadow_share_record46_zero {
        promotion_proof_blocked_reasons.push("record46-zero-shared-by-texture-and-shadow-outline");
    }
    if !record48_separates_shadow_from_texture_and_main {
        promotion_proof_blocked_reasons
            .push("record48-shadow-texture-main-role-separation-missing");
    }
    if !record48_separates_texture_from_main {
        promotion_proof_blocked_reasons.push("record48-texture-main-role-separation-missing");
    }
    if record60_shared_across_roles {
        promotion_proof_blocked_reasons.push("record60-shared-across-roles");
    }
    if record65_shared_across_roles {
        promotion_proof_blocked_reasons.push("record65-shared-across-roles");
    }
    let record46_promotion_proof_ready = promotion_proof_blocked_reasons.is_empty();

    output.push_str("{\"source\":\"embeddedPressPathStateRecordComparison\",\"basis\":\"record46-word0-paint-phase-candidate\",\"decoded\":false,\"sourceBacked\":");
    output.push_str(
        if partition.is_some() && !interstitial_texture_paths.is_empty() {
            "true"
        } else {
            "false"
        },
    );
    output
        .push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false,\"visibleRenderPathCount\":0");
    output.push_str(",\"texturePathCount\":");
    output.push_str(&interstitial_texture_paths.len().to_string());
    output.push_str(",\"shadowOutlinePathCount\":");
    output.push_str(
        &partition
            .as_ref()
            .map_or(0, |partition| partition.shadow_paths.len())
            .to_string(),
    );
    output.push_str(",\"mainOutlinePathCount\":");
    output.push_str(
        &partition
            .as_ref()
            .map_or(0, |partition| partition.main_paths.len())
            .to_string(),
    );
    output.push_str(",\"textureRecord46Word0Values\":");
    push_u32_hex_array_json(output, &texture_record46_values);
    output.push_str(",\"shadowOutlineRecord46Word0Values\":");
    push_u32_hex_array_json(output, &shadow_record46_values);
    output.push_str(",\"mainOutlineRecord46Word0Values\":");
    push_u32_hex_array_json(output, &main_record46_values);
    output.push_str(",\"textureRecord48Word0Values\":");
    push_u32_hex_array_json(output, &texture_record48_values);
    output.push_str(",\"shadowOutlineRecord48Word0Values\":");
    push_u32_hex_array_json(output, &shadow_record48_values);
    output.push_str(",\"mainOutlineRecord48Word0Values\":");
    push_u32_hex_array_json(output, &main_record48_values);
    output.push_str(",\"textureRecord60Word0Values\":");
    push_u32_hex_array_json(output, &texture_record60_values);
    output.push_str(",\"shadowOutlineRecord60Word0Values\":");
    push_u32_hex_array_json(output, &shadow_record60_values);
    output.push_str(",\"mainOutlineRecord60Word0Values\":");
    push_u32_hex_array_json(output, &main_record60_values);
    output.push_str(",\"textureRecord65Word0Values\":");
    push_u32_hex_array_json(output, &texture_record65_values);
    output.push_str(",\"shadowOutlineRecord65Word0Values\":");
    push_u32_hex_array_json(output, &shadow_record65_values);
    output.push_str(",\"mainOutlineRecord65Word0Values\":");
    push_u32_hex_array_json(output, &main_record65_values);
    output.push_str(",\"record46OneAppearsOnlyOnOutlines\":");
    output.push_str(if record46_one_appears_only_on_outlines {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textureAndMainShareRecord46Zero\":");
    output.push_str(if texture_and_main_share_record46_zero {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"textureAndShadowShareRecord46Zero\":");
    output.push_str(if texture_and_shadow_share_record46_zero {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record46SeparatesTextureFromOutlines\":");
    output.push_str(if record46_separates_texture_from_outlines {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesShadowFromTextureAndMain\":");
    output.push_str(if record48_separates_shadow_from_texture_and_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record48SeparatesTextureFromMain\":");
    output.push_str(if record48_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record60SharedAcrossRoles\":");
    output.push_str(if record60_shared_across_roles {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record65SharedAcrossRoles\":");
    output.push_str(if record65_shared_across_roles {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record60SeparatesTextureFromOutlines\":false");
    output.push_str(",\"record65SeparatesTextureFromOutlines\":false");
    output.push_str(",\"promotionProofPolicy\":");
    output.push_str(&json_string(
        "record46-must-separate-texture-from-outlines-and-record48-must-separate-main-role",
    ));
    output.push_str(",\"record46PromotionProofReady\":");
    output.push_str(if record46_promotion_proof_ready {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"promotionProofBlockedReasons\":");
    push_json_string_slice_array(output, &promotion_proof_blocked_reasons);
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(if record46_one_appears_only_on_outlines {
        "record46-one-outline-paint-phase-candidate"
    } else {
        "record46-role-candidate-absent"
    }));
    output
        .push_str(",\"renderPromotionBlockedReason\":\"record46-paint-phase-semantics-unproven\"}");
}

pub(crate) fn push_success_data_test_title_art_shadow_paint_word_gate_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();
    let shadow_paths = partition
        .as_ref()
        .map(|partition| partition.shadow_paths.as_slice())
        .unwrap_or(&[]);
    let main_paths = partition
        .as_ref()
        .map(|partition| partition.main_paths.as_slice())
        .unwrap_or(&[]);

    let shadow_word0 = embedded_press_title_art_state_record_word_values(shadow_paths, 0x70, 0);
    let texture_word0 =
        embedded_press_title_art_state_record_word_values(&interstitial_texture_paths, 0x70, 0);
    let main_word0 = embedded_press_title_art_state_record_word_values(main_paths, 0x70, 0);
    let shadow_word1 = embedded_press_title_art_state_record_word_values(shadow_paths, 0x70, 1);
    let texture_word1 =
        embedded_press_title_art_state_record_word_values(&interstitial_texture_paths, 0x70, 1);
    let main_word1 = embedded_press_title_art_state_record_word_values(main_paths, 0x70, 1);
    let shadow_word3 = embedded_press_title_art_state_record_word_values(shadow_paths, 0x70, 3);
    let texture_word3 =
        embedded_press_title_art_state_record_word_values(&interstitial_texture_paths, 0x70, 3);
    let main_word3 = embedded_press_title_art_state_record_word_values(main_paths, 0x70, 3);
    let shadow_word7 = embedded_press_title_art_state_record_word_values(shadow_paths, 0x70, 7);
    let texture_word7 =
        embedded_press_title_art_state_record_word_values(&interstitial_texture_paths, 0x70, 7);
    let main_word7 = embedded_press_title_art_state_record_word_values(main_paths, 0x70, 7);

    let word0_separates_shadow =
        !shadow_word0.is_empty() && shadow_word0 != texture_word0 && shadow_word0 != main_word0;
    let word3_separates_shadow =
        !shadow_word3.is_empty() && shadow_word3 != texture_word3 && shadow_word3 != main_word3;
    let word7_separates_shadow =
        !shadow_word7.is_empty() && shadow_word7 != texture_word7 && shadow_word7 != main_word7;
    let word1_shared_across_roles =
        !shadow_word1.is_empty() && shadow_word1 == texture_word1 && texture_word1 == main_word1;
    let word0_separates_texture_from_main =
        !texture_word0.is_empty() && texture_word0 != main_word0;
    let word3_separates_texture_from_main =
        !texture_word3.is_empty() && texture_word3 != main_word3;
    let word7_separates_texture_from_main =
        !texture_word7.is_empty() && texture_word7 != main_word7;
    let shadow_effect = partition
        .as_ref()
        .and_then(|partition| embedded_press_title_art_shadow_effect(&partition.shadow_paths));
    let texture_effect = shadow_effect.as_ref().and_then(|effect| {
        embedded_press_title_art_texture_effect(&interstitial_texture_paths, &effect.fill_color)
    });

    output.push_str(
        "{\"source\":\"embeddedPressRecord70RoleComparison\",\"decoded\":false,\"sourceBacked\":",
    );
    output.push_str(
        if partition.is_some() && !interstitial_texture_paths.is_empty() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"record70Word0SeparatesShadowFromTextureAndMain\":");
    output.push_str(if word0_separates_shadow {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word3SeparatesShadowFromTextureAndMain\":");
    output.push_str(if word3_separates_shadow {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word7SeparatesShadowFromTextureAndMain\":");
    output.push_str(if word7_separates_shadow {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word1SharedAcrossRoles\":");
    output.push_str(if word1_shared_across_roles {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word0SeparatesTextureFromMain\":");
    output.push_str(if word0_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word3SeparatesTextureFromMain\":");
    output.push_str(if word3_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"record70Word7SeparatesTextureFromMain\":");
    output.push_str(if word7_separates_texture_from_main {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"shadowEffectCandidate\":");
    if let Some(effect) = shadow_effect.as_ref() {
        output.push_str("{\"basis\":\"record70.word0-percent-black-on-white\",\"word0\":");
        output.push_str(&effect.word0.to_string());
        output.push_str(",\"opacity\":");
        output.push_str(&format!("{:.3}", effect.opacity));
        output.push_str(",\"fillColor\":");
        output.push_str(&json_string(&effect.fill_color));
        output.push('}');
    } else {
        output.push_str("null");
    }
    output.push_str(",\"interstitialTextureEffectCandidate\":");
    if let Some(effect) = texture_effect.as_ref() {
        output.push_str("{\"basis\":\"record70.word0-percent-black-over-shadow\",\"word0\":");
        output.push_str(&effect.word0.to_string());
        output.push_str(",\"opacity\":");
        output.push_str(&format!("{:.3}", effect.opacity));
        output.push_str(",\"baseFillColor\":");
        output.push_str(&json_string(&effect.base_fill_color));
        output.push_str(",\"fillColor\":");
        output.push_str(&json_string(&effect.fill_color));
        output.push_str(",\"renderPromoted\":false,\"renderPromotionBlockedReason\":\"record70-separates-shadow-but-not-interstitial-texture-from-main\"}");
    } else {
        output.push_str("null");
    }
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(
        if word0_separates_shadow
            && word3_separates_shadow
            && word7_separates_shadow
            && !word0_separates_texture_from_main
            && !word3_separates_texture_from_main
            && !word7_separates_texture_from_main
        {
            "none"
        } else {
            "record70-role-separation-unproven"
        },
    ));
    output.push_str(",\"roles\":[");
    push_success_data_test_title_art_record70_role_json(output, "shadowOutlines", shadow_paths);
    output.push(',');
    push_success_data_test_title_art_record70_role_json(
        output,
        "interstitialTextureBlock",
        &interstitial_texture_paths,
    );
    output.push(',');
    push_success_data_test_title_art_record70_role_json(output, "mainOutlines", main_paths);
    output.push_str("]}");
}

pub(crate) fn push_success_data_test_title_art_record70_role_json(
    output: &mut String,
    role: &str,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    output.push_str("{\"role\":");
    output.push_str(&json_string(role));
    output.push_str(",\"pathKind\":");
    output.push_str(&json_string(embedded_press_title_art_path_kind_summary(
        paths,
    )));
    output.push_str(",\"pathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"record70Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(paths, 0x70, 0),
    );
    output.push_str(",\"record70Word1Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(paths, 0x70, 1),
    );
    output.push_str(",\"record70Word3Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(paths, 0x70, 3),
    );
    output.push_str(",\"record70Word7Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(paths, 0x70, 7),
    );
    output.push('}');
}

pub(crate) fn push_success_data_test_title_art_paint_role_separation_matrix_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) {
    let partition = embedded_press_title_art_shadow_path_partition(snapshot);
    let interstitial_texture_paths = partition
        .as_ref()
        .and_then(|partition| {
            success_data_test_title_art_interstitial_texture_paths(snapshot, partition)
        })
        .unwrap_or_default();
    let shadow_paths = partition
        .as_ref()
        .map(|partition| partition.shadow_paths.as_slice())
        .unwrap_or(&[]);
    let main_paths = partition
        .as_ref()
        .map(|partition| partition.main_paths.as_slice())
        .unwrap_or(&[]);

    let shadow_values = embedded_press_title_art_role_state_word_value_sets(shadow_paths);
    let texture_values =
        embedded_press_title_art_role_state_word_value_sets(&interstitial_texture_paths);
    let main_values = embedded_press_title_art_role_state_word_value_sets(main_paths);
    let mut keys = BTreeSet::<(u32, usize)>::new();
    keys.extend(shadow_values.keys().copied());
    keys.extend(texture_values.keys().copied());
    keys.extend(main_values.keys().copied());

    let mut shared_across_all_count = 0usize;
    let mut shadow_unique_count = 0usize;
    let mut texture_unique_count = 0usize;
    let mut main_unique_count = 0usize;
    let mut texture_main_disjoint_count = 0usize;
    let mut shadow_texture_shared_main_disjoint_count = 0usize;
    let mut missing_role_value_count = 0usize;

    for key in &keys {
        let empty = BTreeSet::<u32>::new();
        let shadow = shadow_values.get(key).unwrap_or(&empty);
        let texture = texture_values.get(key).unwrap_or(&empty);
        let main = main_values.get(key).unwrap_or(&empty);
        let present_in_all = !shadow.is_empty() && !texture.is_empty() && !main.is_empty();
        if !present_in_all {
            missing_role_value_count += 1;
        }
        if present_in_all && shadow == texture && texture == main {
            shared_across_all_count += 1;
        }
        if present_in_all && shadow.is_disjoint(texture) && shadow.is_disjoint(main) {
            shadow_unique_count += 1;
        }
        if present_in_all && texture.is_disjoint(shadow) && texture.is_disjoint(main) {
            texture_unique_count += 1;
        }
        if present_in_all && main.is_disjoint(shadow) && main.is_disjoint(texture) {
            main_unique_count += 1;
        }
        if !texture.is_empty() && !main.is_empty() && texture.is_disjoint(main) {
            texture_main_disjoint_count += 1;
        }
        if present_in_all && shadow == texture && texture.is_disjoint(main) {
            shadow_texture_shared_main_disjoint_count += 1;
        }
    }

    output.push_str(
        "{\"source\":\"embeddedPressRoleStateWordMatrix\",\"decoded\":false,\"sourceBacked\":",
    );
    output.push_str(
        if partition.is_some() && !interstitial_texture_paths.is_empty() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"rolePartitionBasis\":\"embeddedPressPathSourceOrder\"");
    output.push_str(",\"recordWordCandidateCount\":");
    output.push_str(&keys.len().to_string());
    output.push_str(",\"sharedAcrossAllRecordWordCount\":");
    output.push_str(&shared_across_all_count.to_string());
    output.push_str(",\"shadowUniqueRecordWordCount\":");
    output.push_str(&shadow_unique_count.to_string());
    output.push_str(",\"textureUniqueRecordWordCount\":");
    output.push_str(&texture_unique_count.to_string());
    output.push_str(",\"mainUniqueRecordWordCount\":");
    output.push_str(&main_unique_count.to_string());
    output.push_str(",\"textureMainDisjointRecordWordCount\":");
    output.push_str(&texture_main_disjoint_count.to_string());
    output.push_str(",\"shadowTextureSharedMainDisjointRecordWordCount\":");
    output.push_str(&shadow_texture_shared_main_disjoint_count.to_string());
    output.push_str(",\"missingRoleValueRecordWordCount\":");
    output.push_str(&missing_role_value_count.to_string());
    output.push_str(",\"textureOnlySeparatorPresent\":");
    output.push_str(if texture_unique_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"mainOnlySeparatorPresent\":");
    output.push_str(if main_unique_count > 0 {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"matrixConclusion\":");
    output.push_str(&json_string(
        if texture_unique_count == 0 && shadow_texture_shared_main_disjoint_count > 0 {
            "record-words-separate-main-from-shadow-state-texture-but-not-interstitial-texture-only"
        } else if texture_unique_count == 0 {
            "no-record-word-separates-interstitial-texture-only"
        } else {
            "texture-only-record-word-candidate-present"
        },
    ));
    output.push_str(",\"renderPromotionBlockedReason\":");
    output.push_str(&json_string(if texture_unique_count == 0 {
        "no-record-word-separates-interstitial-texture-from-both-outline-roles"
    } else {
        "texture-only-record-word-needs-cross-sample-validation"
    }));
    output.push_str(",\"recordWords\":[");
    for (index, key) in keys.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        let empty = BTreeSet::<u32>::new();
        let shadow = shadow_values.get(key).unwrap_or(&empty);
        let texture = texture_values.get(key).unwrap_or(&empty);
        let main = main_values.get(key).unwrap_or(&empty);
        push_success_data_test_title_art_role_matrix_record_word_json(
            output, *key, shadow, texture, main,
        );
    }
    output.push_str("]}");
}

pub(crate) fn push_success_data_test_title_art_role_matrix_record_word_json(
    output: &mut String,
    key: (u32, usize),
    shadow: &BTreeSet<u32>,
    texture: &BTreeSet<u32>,
    main: &BTreeSet<u32>,
) {
    let present_in_all = !shadow.is_empty() && !texture.is_empty() && !main.is_empty();
    let shared_across_all = present_in_all && shadow == texture && texture == main;
    let shadow_disjoint = present_in_all && shadow.is_disjoint(texture) && shadow.is_disjoint(main);
    let texture_disjoint =
        present_in_all && texture.is_disjoint(shadow) && texture.is_disjoint(main);
    let main_disjoint = present_in_all && main.is_disjoint(shadow) && main.is_disjoint(texture);
    let texture_main_disjoint =
        !texture.is_empty() && !main.is_empty() && texture.is_disjoint(main);
    let shadow_texture_shared_main_disjoint =
        present_in_all && shadow == texture && texture.is_disjoint(main);
    let interpretation = if texture_disjoint {
        "texture-only-candidate"
    } else if shadow_texture_shared_main_disjoint {
        "main-vs-shadow-state-texture-candidate"
    } else if shadow_disjoint {
        "shadow-vs-non-shadow-candidate"
    } else if main_disjoint {
        "main-vs-non-main-candidate"
    } else if shared_across_all {
        "shared-across-all-roles"
    } else if !present_in_all {
        "role-missing"
    } else {
        "overlapping-or-ambiguous"
    };

    output.push_str("{\"recordType\":");
    output.push_str(&key.0.to_string());
    output.push_str(",\"recordTypeHex\":");
    output.push_str(&json_string(&format!("0x{:02x}", key.0)));
    output.push_str(",\"wordIndex\":");
    output.push_str(&key.1.to_string());
    output.push_str(",\"shadowValues\":");
    push_u32_hex_array_json(output, &shadow.iter().copied().collect::<Vec<_>>());
    output.push_str(",\"textureValues\":");
    push_u32_hex_array_json(output, &texture.iter().copied().collect::<Vec<_>>());
    output.push_str(",\"mainValues\":");
    push_u32_hex_array_json(output, &main.iter().copied().collect::<Vec<_>>());
    output.push_str(",\"presentInAllRoles\":");
    output.push_str(if present_in_all { "true" } else { "false" });
    output.push_str(",\"sharedAcrossAllRoles\":");
    output.push_str(if shared_across_all { "true" } else { "false" });
    output.push_str(",\"shadowDisjointFromTextureAndMain\":");
    output.push_str(if shadow_disjoint { "true" } else { "false" });
    output.push_str(",\"textureDisjointFromShadowAndMain\":");
    output.push_str(if texture_disjoint { "true" } else { "false" });
    output.push_str(",\"mainDisjointFromShadowAndTexture\":");
    output.push_str(if main_disjoint { "true" } else { "false" });
    output.push_str(",\"textureMainDisjoint\":");
    output.push_str(if texture_main_disjoint {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"shadowTextureSharedMainDisjoint\":");
    output.push_str(if shadow_texture_shared_main_disjoint {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"interpretation\":");
    output.push_str(&json_string(interpretation));
    output.push('}');
}

pub(crate) fn push_success_data_test_title_art_role_gate_group_json(
    output: &mut String,
    role: &str,
    paths: &[&ObjectEmbeddedPressVectorPathCandidate],
) {
    let explicit_state_path_count = paths
        .iter()
        .filter(|path| !path.state_records().is_empty())
        .count();

    output.push_str("{\"role\":");
    output.push_str(&json_string(role));
    output.push_str(",\"pathKind\":");
    output.push_str(&json_string(embedded_press_title_art_path_kind_summary(
        paths,
    )));
    output.push_str(",\"pathCount\":");
    output.push_str(&paths.len().to_string());
    output.push_str(",\"explicitStatePathCount\":");
    output.push_str(&explicit_state_path_count.to_string());
    output.push_str(",\"inheritedStatePathCount\":");
    output.push_str(
        &paths
            .len()
            .saturating_sub(explicit_state_path_count)
            .to_string(),
    );
    output.push_str(",\"record46Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x46),
    );
    output.push_str(",\"record48Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x48),
    );
    output.push_str(",\"record60Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x60),
    );
    output.push_str(",\"record65Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x65),
    );
    output.push_str(",\"record70Word0Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word0_values(paths, 0x70),
    );
    output.push_str(",\"record82Word5Values\":");
    push_u32_hex_array_json(
        output,
        &embedded_press_title_art_state_record_word_values(
            paths,
            EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
            5,
        ),
    );
    output.push('}');
}

pub(crate) fn success_data_test_title_art_effective_texture_paths_for_word5(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    expected_word5: u32,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    embedded_press_effective_texture_paths_for_state_word(
        snapshot,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        5,
        expected_word5,
    )
}

pub(crate) fn success_data_test_title_art_effective_texture_word5_values(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<u32> {
    embedded_press_effective_texture_state_word_values(
        snapshot,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        5,
    )
}

pub(crate) fn success_data_test_title_art_effective_front_texture_word5_values(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<u32> {
    embedded_press_effective_texture_state_word_values(
        snapshot,
        EMBEDDED_PRESS_RECORD_PAINT_STATE_82,
        5,
    )
    .into_iter()
    .filter(|value| *value == EMBEDDED_PRESS_TITLE_ART_MAIN_STATE_WORD5)
    .collect::<Vec<_>>()
}

pub(crate) fn push_success_data_test_title_art_frame_svg(
    svg: &mut String,
    frame: &ObjectJsfartArtFrameCandidate,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) {
    let frame_x = x + frame.left() as f32 * scale_x;
    let frame_y = y + frame.top() as f32 * scale_y;
    let frame_width = frame.right().saturating_sub(frame.left()) as f32 * scale_x;
    let frame_height = frame.bottom().saturating_sub(frame.top()) as f32 * scale_y;
    if frame_width <= 0.0 || frame_height <= 0.0 {
        return;
    }

    let rx = frame.corner_radius_x() as f32 * scale_x;
    let ry = frame.corner_radius_y() as f32 * scale_y;
    let stroke_width = success_data_test_title_art_frame_stroke_width(frame, scale_x, scale_y);
    svg.push_str(&format!(
        "<rect class=\"rjtd-success-data-test-title-frame\" data-source=\"JSFart2Contents\" data-source-left=\"{}\" data-source-top=\"{}\" data-source-right=\"{}\" data-source-bottom=\"{}\" data-source-content-left=\"{}\" data-source-content-top=\"{}\" data-source-content-right=\"{}\" data-source-content-bottom=\"{}\" data-source-corner-radius-x=\"{}\" data-source-corner-radius-y=\"{}\" x=\"{frame_x:.2}\" y=\"{frame_y:.2}\" width=\"{frame_width:.2}\" height=\"{frame_height:.2}\" rx=\"{rx:.2}\" ry=\"{ry:.2}\" fill=\"none\" stroke=\"#111111\" stroke-width=\"{stroke_width:.2}\"/>",
        frame.left(),
        frame.top(),
        frame.right(),
        frame.bottom(),
        frame.content_left(),
        frame.content_top(),
        frame.content_right(),
        frame.content_bottom(),
        frame.corner_radius_x(),
        frame.corner_radius_y(),
    ));
}

pub(crate) fn success_data_test_title_art_frame_stroke_width(
    frame: &ObjectJsfartArtFrameCandidate,
    scale_x: f32,
    scale_y: f32,
) -> f32 {
    frame
        .stroke_width_candidate()
        .map(|value| value as f32 * ((scale_x + scale_y) / 2.0) * 0.5)
        .unwrap_or_else(|| ((scale_x + scale_y) / 2.0).max(1.0))
}

pub(crate) fn success_data_test_title_art_diagnostic_for_page(
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
    page_number: usize,
) -> bool {
    success_data_test_title_art_page_number(document, diagnostic) == Some(page_number)
}

pub(crate) fn success_data_test_title_art_page_number(
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> Option<usize> {
    if !success_data_test_title_art_source_matches(document, diagnostic) {
        return None;
    }

    let frame_ref = diagnostic.frame.frame_ref();
    success_data_test_title_art_frame_refs(document)
        .into_iter()
        .position(|candidate_frame_ref| candidate_frame_ref == frame_ref)
        .map(|index| index + 1)
}

pub(crate) fn success_data_test_title_art_source_matches(
    document: &Document,
    diagnostic: EmbeddingFrameDiagnostic<'_>,
) -> bool {
    if !document_has_success_data_test_projection_evidence(document)
        || diagnostic.frame.class_name() != "JSFart.Art.2"
    {
        return false;
    }

    let Some(snapshot) = diagnostic.embedded_press_snapshot else {
        return false;
    };
    let primary_size_matches_snapshot = u32::from(diagnostic.frame.primary_width())
        == snapshot.width()
        && u32::from(diagnostic.frame.primary_height()) == snapshot.height();
    let frame_height_matches_snapshot = diagnostic.frame.frame_height() == snapshot.height();
    let frame_width_matches_or_clips_snapshot = diagnostic.frame.frame_width() == snapshot.width()
        || (diagnostic.frame.frame_width() > 0
            && diagnostic.frame.frame_width() < snapshot.width()
            && diagnostic.frame.frame_width().saturating_mul(2) >= snapshot.width());
    let art_size_matches_snapshot = success_data_test_title_art_jsfart_art_candidate(
        document,
        diagnostic.frame.embedding_index(),
    )
    .is_some_and(|art| art.width() == snapshot.width() && art.height() == snapshot.height());

    primary_size_matches_snapshot
        && frame_height_matches_snapshot
        && frame_width_matches_or_clips_snapshot
        && art_size_matches_snapshot
}

pub(crate) fn success_data_test_title_art_frame_refs(document: &Document) -> Vec<u32> {
    let mut frame_refs = Vec::new();
    for diagnostic in embedding_frame_diagnostics(document) {
        if success_data_test_title_art_source_matches(document, diagnostic) {
            let frame_ref = diagnostic.frame.frame_ref();
            if frame_ref > 0 && !frame_refs.contains(&frame_ref) {
                frame_refs.push(frame_ref);
            }
        }
    }
    frame_refs
}

pub(crate) fn success_data_test_title_art_jsfart_art_candidate(
    document: &Document,
    embedding_index: usize,
) -> Option<&ObjectJsfartArtCandidate> {
    let path = format!("/EmbedItems/Embedding {embedding_index}/JSFart2Contents");
    document
        .object_stream_candidates()
        .iter()
        .find(|candidate| candidate.path() == path)
        .and_then(ObjectStreamCandidate::jsfart_art_candidate)
}

pub(crate) fn success_data_test_title_art_jsfart_frame_candidate(
    document: &Document,
    embedding_index: usize,
) -> Option<&ObjectJsfartArtFrameCandidate> {
    success_data_test_title_art_jsfart_art_candidate(document, embedding_index)
        .and_then(ObjectJsfartArtCandidate::frame_candidate)
}

pub(crate) fn success_data_test_title_art_frame_vertical_scale(
    frame_record_height: f32,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
) -> (f32, &'static str, u32) {
    if let Some(content_height) = source_frame_candidate
        .and_then(|frame| frame.content_bottom().checked_sub(frame.content_top()))
        .filter(|height| *height > 0)
    {
        return (
            frame_record_height / content_height as f32,
            "jsfartContentHeight",
            content_height,
        );
    }
    if snapshot.height() == 0 {
        return (0.0, "none", 0);
    }
    (
        frame_record_height / snapshot.height() as f32,
        "snapshotHeight",
        snapshot.height(),
    )
}

pub(crate) fn success_data_test_title_art_rendered_segment_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    snapshot
        .vector_segments()
        .iter()
        .filter(|segment| success_data_test_title_art_segment_should_render(segment))
        .count()
}

pub(crate) fn success_data_test_title_art_rendered_path_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Outline && !path.commands().is_empty()
        })
        .count()
}

#[cfg(test)]
pub(crate) fn success_data_test_title_art_shadow_path_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    let outline_count = success_data_test_title_art_rendered_path_count(snapshot);
    if outline_count > 1 && outline_count.is_multiple_of(2) {
        outline_count / 2
    } else {
        0
    }
}

pub(crate) fn success_data_test_title_art_rendered_texture_path_count(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> usize {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Texture && !path.commands().is_empty()
        })
        .count()
}

pub(crate) fn success_data_test_title_art_rendered_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    snapshot
        .vector_paths()
        .iter()
        .filter(|path| {
            path.kind() == ObjectEmbeddedPressVectorPathKind::Outline && !path.commands().is_empty()
        })
        .collect::<Vec<_>>()
}

pub(crate) fn success_data_test_title_art_horizontal_placement(
    frame_record_x: f32,
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
    scale_x: f32,
) -> SuccessDataTestTitleArtHorizontalPlacement {
    let content_left_adjustment =
        source_frame_candidate.map_or(0.0, |frame| frame.content_left() as f32 * scale_x);
    let content_left_only_x = (frame_record_x - content_left_adjustment).max(0.0);
    let stroke_width_candidate =
        source_frame_candidate.and_then(ObjectJsfartArtFrameCandidate::stroke_width_candidate);
    let stroke_outer_adjustment =
        stroke_width_candidate.map_or(0.0, |value| value as f32 * scale_x);
    if stroke_outer_adjustment > 0.0 {
        SuccessDataTestTitleArtHorizontalPlacement {
            frame_x: content_left_only_x,
            path_x: content_left_only_x,
            candidate_frame_x: (content_left_only_x - stroke_outer_adjustment).max(0.0),
            candidate_path_x: frame_record_x.max(0.0),
            content_left_adjustment,
            stroke_outer_adjustment,
            content_left_only_x,
            frame_record_x,
            basis: "jsfartContentLeft",
            render_promoted: false,
            stroke_width_candidate,
        }
    } else {
        SuccessDataTestTitleArtHorizontalPlacement {
            frame_x: content_left_only_x,
            path_x: content_left_only_x,
            candidate_frame_x: content_left_only_x,
            candidate_path_x: content_left_only_x,
            content_left_adjustment,
            stroke_outer_adjustment,
            content_left_only_x,
            frame_record_x,
            basis: "jsfartContentLeft",
            render_promoted: false,
            stroke_width_candidate,
        }
    }
}

pub(crate) fn push_success_data_test_title_art_horizontal_placement_json(
    output: &mut String,
    placement: SuccessDataTestTitleArtHorizontalPlacement,
) {
    output.push_str("{\"source\":\"JSFart2Contents.frameCandidate\",\"decoded\":false,\"sourceBacked\":true,\"referenceBacked\":false,\"basis\":");
    output.push_str(&json_string(placement.basis));
    output.push_str(",\"frameRecordX\":");
    output.push_str(&format!("{:.3}", placement.frame_record_x));
    output.push_str(",\"contentLeftOnlyX\":");
    output.push_str(&format!("{:.3}", placement.content_left_only_x));
    output.push_str(",\"frameX\":");
    output.push_str(&format!("{:.3}", placement.frame_x));
    output.push_str(",\"pathX\":");
    output.push_str(&format!("{:.3}", placement.path_x));
    output.push_str(",\"candidateFrameX\":");
    output.push_str(&format!("{:.3}", placement.candidate_frame_x));
    output.push_str(",\"candidatePathX\":");
    output.push_str(&format!("{:.3}", placement.candidate_path_x));
    output.push_str(",\"candidateBasis\":");
    output.push_str(&json_string(
        "jsfartFrameOuterEdgePlusFrameRecordContentOrigin",
    ));
    output.push_str(",\"contentLeftAdjustmentCssPx\":");
    output.push_str(&format!("{:.3}", placement.content_left_adjustment));
    output.push_str(",\"strokeWidthCandidateSourceUnits\":");
    match placement.stroke_width_candidate {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"strokeOuterAdjustmentCssPx\":");
    output.push_str(&format!("{:.3}", placement.stroke_outer_adjustment));
    output.push_str(",\"renderPromoted\":");
    output.push_str(if placement.render_promoted {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"renderPromotionBlockedReason\":");
    if placement.render_promoted {
        output.push_str("null");
    } else {
        output.push_str(&json_string(if placement.stroke_outer_adjustment > 0.0 {
            "frame-content-split-horizontal-semantics-unproven"
        } else {
            "jsfart-stroke-width-candidate-missing-for-horizontal-anchor"
        }));
    }
    output.push('}');
}

pub(crate) fn success_data_test_title_art_horizontal_placement_svg_attrs(
    placement: SuccessDataTestTitleArtHorizontalPlacement,
) -> String {
    format!(
        " data-horizontal-placement-basis=\"{}\" data-horizontal-placement-source=\"JSFart2Contents.frameCandidate\" data-horizontal-placement-source-backed=\"true\" data-horizontal-placement-render-promoted=\"{}\" data-horizontal-frame-record-x=\"{:.3}\" data-horizontal-content-left-only-x=\"{:.3}\" data-horizontal-frame-x=\"{:.3}\" data-horizontal-path-x=\"{:.3}\" data-horizontal-candidate-frame-x=\"{:.3}\" data-horizontal-candidate-path-x=\"{:.3}\" data-horizontal-candidate-basis=\"jsfartFrameOuterEdgePlusFrameRecordContentOrigin\" data-horizontal-stroke-width-source-units=\"{}\" data-horizontal-stroke-outer-adjustment-css-px=\"{:.3}\"",
        escape_xml(placement.basis),
        placement.render_promoted,
        placement.frame_record_x,
        placement.content_left_only_x,
        placement.frame_x,
        placement.path_x,
        placement.candidate_frame_x,
        placement.candidate_path_x,
        placement
            .stroke_width_candidate
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        placement.stroke_outer_adjustment
    )
}

pub(crate) fn success_data_test_title_art_source_frame_trace_conclusion(
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
    frame_ref: u32,
) -> &'static str {
    let Some(frame) = source_frame_candidate else {
        return "missing-jsfart-frame-candidate";
    };
    let Some(record) = frame_record else {
        return "missing-frame-record";
    };
    let outer_width = frame.right().saturating_sub(frame.left());
    let outer_height = frame.bottom().saturating_sub(frame.top());
    if u32::from(record.object_id()) == frame_ref
        && u32::from(record.width()) == outer_width
        && u32::from(record.height()) == outer_height
    {
        "frame-record-and-jsfart-outer-size-agree"
    } else {
        "frame-record-jsfart-outer-size-or-ref-mismatch"
    }
}

pub(crate) fn push_success_data_test_title_art_source_frame_render_trace_json(
    output: &mut String,
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
    frame_ref: u32,
    horizontal_placement: SuccessDataTestTitleArtHorizontalPlacement,
    frame_scale_y_basis: &str,
    frame_scale_y_source_units: u32,
) {
    let source_outer_width =
        source_frame_candidate.map(|frame| frame.right().saturating_sub(frame.left()));
    let source_outer_height =
        source_frame_candidate.map(|frame| frame.bottom().saturating_sub(frame.top()));
    let source_content_width = source_frame_candidate
        .map(|frame| frame.content_right().saturating_sub(frame.content_left()));
    let source_content_height = source_frame_candidate
        .map(|frame| frame.content_bottom().saturating_sub(frame.content_top()));
    let frame_record_width = frame_record.map(|record| u32::from(record.width()));
    let frame_record_height = frame_record.map(|record| u32::from(record.height()));
    let frame_ref_matches_object_id =
        frame_record.is_some_and(|record| u32::from(record.object_id()) == frame_ref);
    let outer_width_matches = source_outer_width
        .zip(frame_record_width)
        .is_some_and(|(source, record)| source == record);
    let outer_height_matches = source_outer_height
        .zip(frame_record_height)
        .is_some_and(|(source, record)| source == record);
    output.push_str(
        "{\"source\":\"JSFart2Contents.frameCandidate+/Frame\",\"decoded\":false,\"sourceBacked\":",
    );
    output.push_str(
        if source_frame_candidate.is_some() && frame_record.is_some() {
            "true"
        } else {
            "false"
        },
    );
    output.push_str(",\"diagnosticOnly\":true,\"renderPromoted\":false");
    output.push_str(",\"frameRef\":");
    output.push_str(&frame_ref.to_string());
    output.push_str(",\"frameRecordObjectId\":");
    match frame_record {
        Some(record) => output.push_str(&record.object_id().to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\"frameRefMatchesObjectId\":");
    output.push_str(if frame_ref_matches_object_id {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceOuterWidthUnits\":");
    push_option_u32_json(output, source_outer_width);
    output.push_str(",\"frameRecordWidthUnits\":");
    push_option_u32_json(output, frame_record_width);
    output.push_str(",\"outerWidthMatchesFrameRecord\":");
    output.push_str(if outer_width_matches { "true" } else { "false" });
    output.push_str(",\"sourceOuterHeightUnits\":");
    push_option_u32_json(output, source_outer_height);
    output.push_str(",\"frameRecordHeightUnits\":");
    push_option_u32_json(output, frame_record_height);
    output.push_str(",\"outerHeightMatchesFrameRecord\":");
    output.push_str(if outer_height_matches {
        "true"
    } else {
        "false"
    });
    output.push_str(",\"sourceContentWidthUnits\":");
    push_option_u32_json(output, source_content_width);
    output.push_str(",\"sourceContentHeightUnits\":");
    push_option_u32_json(output, source_content_height);
    output.push_str(",\"horizontalPlacementBasis\":");
    output.push_str(&json_string(horizontal_placement.basis));
    output.push_str(",\"selectedFrameX\":");
    output.push_str(&format!("{:.3}", horizontal_placement.frame_x));
    output.push_str(",\"candidateFrameX\":");
    output.push_str(&format!("{:.3}", horizontal_placement.candidate_frame_x));
    output.push_str(",\"frameScaleYBasis\":");
    output.push_str(&json_string(frame_scale_y_basis));
    output.push_str(",\"frameScaleYSourceUnits\":");
    if frame_scale_y_source_units == 0 {
        output.push_str("null");
    } else {
        output.push_str(&frame_scale_y_source_units.to_string());
    }
    output.push_str(",\"traceConclusion\":");
    output.push_str(&json_string(
        success_data_test_title_art_source_frame_trace_conclusion(
            source_frame_candidate,
            frame_record,
            frame_ref,
        ),
    ));
    output.push_str(
        ",\"renderPromotionBlockedReason\":\"frame-content-split-horizontal-semantics-unproven\"}",
    );
}

pub(crate) fn success_data_test_title_art_source_frame_render_trace_svg_attrs(
    source_frame_candidate: Option<&ObjectJsfartArtFrameCandidate>,
    frame_record: Option<&ObjectFrameRecordCandidate>,
    frame_ref: u32,
    horizontal_placement: SuccessDataTestTitleArtHorizontalPlacement,
    frame_scale_y_basis: &str,
    frame_scale_y_source_units: u32,
) -> String {
    let source_outer_width =
        source_frame_candidate.map(|frame| frame.right().saturating_sub(frame.left()));
    let source_outer_height =
        source_frame_candidate.map(|frame| frame.bottom().saturating_sub(frame.top()));
    let frame_record_width = frame_record.map(|record| u32::from(record.width()));
    let frame_record_height = frame_record.map(|record| u32::from(record.height()));
    let frame_ref_matches_object_id =
        frame_record.is_some_and(|record| u32::from(record.object_id()) == frame_ref);
    let outer_width_matches = source_outer_width
        .zip(frame_record_width)
        .is_some_and(|(source, record)| source == record);
    let outer_height_matches = source_outer_height
        .zip(frame_record_height)
        .is_some_and(|(source, record)| source == record);
    format!(
        " data-title-source-frame-trace-source=\"JSFart2Contents.frameCandidate+/Frame\" data-title-source-frame-trace-source-backed=\"{}\" data-title-source-frame-trace-render-promoted=\"false\" data-title-source-frame-trace-frame-ref=\"{}\" data-title-source-frame-trace-frame-record-object-id=\"{}\" data-title-source-frame-trace-frame-ref-matches-object-id=\"{}\" data-title-source-frame-trace-source-outer-width-units=\"{}\" data-title-source-frame-trace-frame-record-width-units=\"{}\" data-title-source-frame-trace-outer-width-matches-frame-record=\"{}\" data-title-source-frame-trace-source-outer-height-units=\"{}\" data-title-source-frame-trace-frame-record-height-units=\"{}\" data-title-source-frame-trace-outer-height-matches-frame-record=\"{}\" data-title-source-frame-trace-horizontal-placement-basis=\"{}\" data-title-source-frame-trace-selected-frame-x=\"{:.3}\" data-title-source-frame-trace-candidate-frame-x=\"{:.3}\" data-title-source-frame-trace-frame-scale-y-basis=\"{}\" data-title-source-frame-trace-frame-scale-y-units=\"{}\" data-title-source-frame-trace-conclusion=\"{}\" data-title-source-frame-trace-render-blocked-reason=\"frame-content-split-horizontal-semantics-unproven\"",
        source_frame_candidate.is_some() && frame_record.is_some(),
        frame_ref,
        frame_record
            .map(|record| record.object_id().to_string())
            .unwrap_or_else(|| "none".to_string()),
        frame_ref_matches_object_id,
        source_outer_width
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        frame_record_width
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        outer_width_matches,
        source_outer_height
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        frame_record_height
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        outer_height_matches,
        escape_xml(horizontal_placement.basis),
        horizontal_placement.frame_x,
        horizontal_placement.candidate_frame_x,
        escape_xml(frame_scale_y_basis),
        if frame_scale_y_source_units == 0 {
            "none".to_string()
        } else {
            frame_scale_y_source_units.to_string()
        },
        escape_xml(success_data_test_title_art_source_frame_trace_conclusion(
            source_frame_candidate,
            frame_record,
            frame_ref,
        ))
    )
}

pub(crate) fn success_data_test_title_art_main_outline_paths(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
) -> Vec<&ObjectEmbeddedPressVectorPathCandidate> {
    embedded_press_title_art_shadow_path_partition(snapshot).map_or_else(
        || success_data_test_title_art_rendered_paths(snapshot),
        |partition| partition.main_paths,
    )
}

pub(crate) fn success_data_test_title_art_projected_main_path_bbox(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    x: f32,
    y: f32,
    scale_x: f32,
    scale_y: f32,
) -> Option<SuccessDataTestProjectedPathBBox> {
    let paths = success_data_test_title_art_main_outline_paths(snapshot);
    let mut left = i32::MAX;
    let mut top = i32::MAX;
    let mut right = i32::MIN;
    let mut bottom = i32::MIN;
    let mut has_bbox = false;
    for path in paths {
        let Some((path_left, path_top, path_right, path_bottom)) =
            embedded_press_vector_path_sampled_source_bbox(
                path,
                SUCCESS_DATA_TEST_TITLE_ART_FACE_CURVE_SAMPLES,
            )
            .or_else(|| embedded_press_vector_path_source_bbox(path))
        else {
            continue;
        };
        left = left.min(path_left);
        top = top.min(path_top);
        right = right.max(path_right);
        bottom = bottom.max(path_bottom);
        has_bbox = true;
    }
    if !has_bbox {
        return None;
    }

    let projected_left = x + left as f32 * scale_x;
    let projected_top = y + top as f32 * scale_y;
    let projected_right = x + right as f32 * scale_x;
    let projected_bottom = y + bottom as f32 * scale_y;
    Some(SuccessDataTestProjectedPathBBox {
        x: projected_left,
        y: projected_top,
        width: projected_right - projected_left,
        height: projected_bottom - projected_top,
    })
}

pub(crate) fn success_data_test_title_art_path_scale_bbox_svg_attrs(
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    x: f32,
    source_path_y: f32,
    frame_path_y: f32,
    scale_x: f32,
    source_scale_y: f32,
    frame_scale_y: f32,
) -> String {
    let Some(source_scale_bbox) = success_data_test_title_art_projected_main_path_bbox(
        snapshot,
        x,
        source_path_y,
        scale_x,
        source_scale_y,
    ) else {
        return String::new();
    };
    let Some(frame_scale_bbox) = success_data_test_title_art_projected_main_path_bbox(
        snapshot,
        x,
        frame_path_y,
        scale_x,
        frame_scale_y,
    ) else {
        return String::new();
    };

    format!(
        " data-main-outline-scale-diagnostic=\"source-scale-vs-frame-scale\" data-main-outline-scale-diagnostic-pixel-change=\"false\" data-main-outline-source-scale-bbox-x=\"{:.3}\" data-main-outline-source-scale-bbox-y=\"{:.3}\" data-main-outline-source-scale-bbox-width=\"{:.3}\" data-main-outline-source-scale-bbox-height=\"{:.3}\" data-main-outline-frame-scale-bbox-x=\"{:.3}\" data-main-outline-frame-scale-bbox-y=\"{:.3}\" data-main-outline-frame-scale-bbox-width=\"{:.3}\" data-main-outline-frame-scale-bbox-height=\"{:.3}\"",
        source_scale_bbox.x,
        source_scale_bbox.y,
        source_scale_bbox.width,
        source_scale_bbox.height,
        frame_scale_bbox.x,
        frame_scale_bbox.y,
        frame_scale_bbox.width,
        frame_scale_bbox.height
    )
}

pub(crate) fn push_success_data_test_title_art_projected_path_bbox_json(
    output: &mut String,
    bbox: SuccessDataTestProjectedPathBBox,
) {
    output.push_str("{\"x\":");
    output.push_str(&format!("{:.3}", bbox.x));
    output.push_str(",\"y\":");
    output.push_str(&format!("{:.3}", bbox.y));
    output.push_str(",\"width\":");
    output.push_str(&format!("{:.3}", bbox.width));
    output.push_str(",\"height\":");
    output.push_str(&format!("{:.3}", bbox.height));
    output.push('}');
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_success_data_test_title_art_path_scale_bbox_diagnostic_json(
    output: &mut String,
    snapshot: &ObjectEmbeddedPressSnapshotCandidate,
    x: f32,
    source_path_y: f32,
    frame_path_y: f32,
    scale_x: f32,
    source_scale_y: f32,
    frame_scale_y: f32,
) {
    let source_scale_bbox = success_data_test_title_art_projected_main_path_bbox(
        snapshot,
        x,
        source_path_y,
        scale_x,
        source_scale_y,
    );
    let frame_scale_bbox = success_data_test_title_art_projected_main_path_bbox(
        snapshot,
        x,
        frame_path_y,
        scale_x,
        frame_scale_y,
    );
    output.push_str("{\"source\":\"embeddedPressMainOutlinePathSampledBbox\",\"pixelChange\":false,\"scaleComparisonDecoded\":false,\"currentRendererPathScale\":\"sourceScale\",\"frameClipScale\":\"frameScale\",\"renderPromotionBlockedReason\":\"title-art-y-scale-basis-unproven\",\"sourceScaleBbox\":");
    if let Some(bbox) = source_scale_bbox {
        push_success_data_test_title_art_projected_path_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push_str(",\"frameScaleBbox\":");
    if let Some(bbox) = frame_scale_bbox {
        push_success_data_test_title_art_projected_path_bbox_json(output, bbox);
    } else {
        output.push_str("null");
    }
    output.push('}');
}

pub(crate) fn success_data_test_title_art_segment_should_render(
    segment: &ObjectEmbeddedPressVectorSegmentCandidate,
) -> bool {
    let dx = segment.x1().abs_diff(segment.x2()) as f32;
    let dy = segment.y1().abs_diff(segment.y2()) as f32;
    (dx * dx + dy * dy).sqrt() <= SUCCESS_DATA_TEST_TITLE_ART_MAX_SEGMENT_SOURCE_LEN
}
