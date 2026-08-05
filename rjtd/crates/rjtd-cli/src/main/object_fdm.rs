use std::collections::BTreeMap;

use rjtd_model::{ObjectFrameRecordCandidate, parse_document};

use crate::input::read_file;

use super::object_fdm_support::*;
use super::object_stream_support::*;
use super::style_support::{format_optional_text, format_string_counts};
use super::support::*;
use super::text_position_count_support::{
    format_be16_hex_fields, format_be16_signed_fields, format_optional_u16_decimal,
    format_optional_u16_hex, format_optional_u64,
};

pub(crate) fn run_object_image_frame_candidates(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-image-frame-candidates")?;
    let bytes = read_file(path)?;
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let mut source_count = 0usize;
    let mut frame_linked_count = 0usize;
    let mut missing_frame_count = 0usize;
    let mut preferred_counts = BTreeMap::<String, usize>::new();
    let mut total_frame_rows = 0usize;
    let mut total_dimensioned_payloads = 0usize;
    let mut total_aspect_candidates = 0usize;

    for candidate in document
        .object_stream_candidates()
        .iter()
        .filter(|candidate| !candidate.image_payload_spans().is_empty())
    {
        source_count += 1;
        let summary = summarize_object_image_frame_candidate(candidate);
        total_frame_rows += summary.frame_rows;
        let dimensioned_payloads = object_payload_dimension_count(candidate.image_payload_spans());
        let aspect_candidates = coordinate_payload_aspect_candidate_count(
            &summary.coordinate_pairs,
            candidate.image_payload_spans(),
        );
        total_dimensioned_payloads += dimensioned_payloads;
        total_aspect_candidates += aspect_candidates;
        if summary.frame_rows == 0 {
            missing_frame_count += 1;
        } else {
            frame_linked_count += 1;
        }
        *preferred_counts
            .entry(summary.preferred.to_string())
            .or_default() += 1;

        write_stdout_line(&format!(
            "object-image-frame-candidate\tsource={}\tembedding={}\tpayloads={}\tpayload-kinds={}\tpayload-dimensions={}\tdimensioned-payloads={}\tframe-rows={}\trow-families={}\trow12-tail-coordinate={}\trow12-tail-zero={}\trow20-tail-window={}\trow20-linked={}\tle-row12={}\tpreferred={}\tcoordinate-pairs={}\tbest-coordinate-aspect-delta-permille={}\tdecoded=false",
            escaped_path(candidate.path()),
            format_optional_usize(summary.embedding_index),
            candidate.image_payload_spans().len(),
            format_string_set(&summary.payload_kinds),
            format_object_payload_dimensions(candidate.image_payload_spans()),
            dimensioned_payloads,
            summary.frame_rows,
            format_string_counts(&summary.family_counts),
            summary.row12_tail_coordinate,
            summary.row12_tail_zero,
            summary.row20_tail_window,
            summary.row20_linked,
            summary.le_row12,
            summary.preferred,
            format_object_frame_coordinate_pairs(&summary.coordinate_pairs),
            format_optional_u64(best_coordinate_payload_aspect_delta_permille(
                &summary.coordinate_pairs,
                candidate.image_payload_spans()
            ))
        ))?;
    }

    write_stdout_line(&format!(
        "summary\tsources={}\tframe-linked={}\tmissing-frame={}\tframe-rows={}\tdimensioned-payloads={}\taspect-candidates={}\tpreferred={}\tdecoded=false",
        source_count,
        frame_linked_count,
        missing_frame_count,
        total_frame_rows,
        total_dimensioned_payloads,
        total_aspect_candidates,
        format_string_counts(&preferred_counts)
    ))?;
    Ok(())
}

pub(crate) fn run_object_fdm_image_candidates(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-fdm-image-candidates")?;
    let bytes = read_file(path)?;
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let mut source_count = 0usize;
    let mut candidate_count = 0usize;
    let mut image_hit_count = 0usize;
    let mut complete_payload_count = 0usize;
    let mut plausible_bbox_count = 0usize;
    let renderable_count = 0usize;

    for candidate in document.object_stream_candidates() {
        let image_entries = candidate
            .fdm_index_entry_candidates()
            .iter()
            .filter(|entry| !entry.segment_image_signature_hits().is_empty())
            .collect::<Vec<_>>();
        if image_entries.is_empty() {
            continue;
        }

        source_count += 1;
        for entry in image_entries {
            let bbox = entry.bbox();
            let normalized = normalize_fdm_bbox(bbox);
            let bbox_order = fdm_bbox_order(bbox);
            let bbox_width = normalized.2.saturating_sub(normalized.0);
            let bbox_height = normalized.3.saturating_sub(normalized.1);
            let bbox_plausible = fdm_bbox_is_plausible(bbox);
            let complete_payloads = fdm_entry_complete_payload_count(candidate, entry);
            let reason = fdm_image_candidate_render_blocked_reason(entry, complete_payloads);
            candidate_count += 1;
            image_hit_count += entry.segment_image_signature_hits().len();
            complete_payload_count += complete_payloads;
            if bbox_plausible {
                plausible_bbox_count += 1;
            }

            write_stdout_line(&format!(
                "object-fdm-image-candidate\tsource={}\tindex={}\trow={}\tvector-offset={}\tnext-vector-offset={}\tvector-length={}\tkind=0x{:04x}\tbbox={},{},{},{}\tnormalized-bbox={},{},{},{}\tbbox-size={}x{}\tbbox-order={}\tbbox-plausible={}\timage-hits={}\tcomplete-payloads={}\timage-signatures={}\tsegment-image-signatures={}\trenderable=false\treason={}\tdecoded=false",
                escaped_path(candidate.path()),
                escaped_path(entry.index_path()),
                entry.row_index(),
                entry.vector_offset(),
                entry.next_vector_offset(),
                entry.vector_len(),
                entry.kind(),
                bbox.left(),
                bbox.top(),
                bbox.right(),
                bbox.bottom(),
                normalized.0,
                normalized.1,
                normalized.2,
                normalized.3,
                bbox_width,
                bbox_height,
                bbox_order,
                bbox_plausible,
                entry.segment_image_signature_hits().len(),
                complete_payloads,
                format_model_object_signature_hits(entry.image_signature_hits()),
                format_model_object_signature_hits(entry.segment_image_signature_hits()),
                reason
            ))?;
        }
    }

    write_stdout_line(&format!(
        "summary\tsources={}\tcandidates={}\timage-hits={}\tcomplete-payloads={}\tbbox-plausible={}\trenderable={}\tdecoded=false",
        source_count,
        candidate_count,
        image_hit_count,
        complete_payload_count,
        plausible_bbox_count,
        renderable_count
    ))?;
    Ok(())
}

pub(crate) fn run_object_fdm_frame_links(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-fdm-frame-links")?;
    let bytes = read_file(path)?;
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let mut source_count = 0usize;
    let mut candidate_count = 0usize;
    let mut frame_linked_count = 0usize;
    let mut missing_frame_count = 0usize;
    let mut complete_payload_count = 0usize;
    let renderable_count = 0usize;

    for candidate in document.object_stream_candidates() {
        let image_entries = candidate
            .fdm_index_entry_candidates()
            .iter()
            .filter(|entry| !entry.segment_image_signature_hits().is_empty())
            .collect::<Vec<_>>();
        if image_entries.is_empty() {
            continue;
        }

        source_count += 1;
        for entry in image_entries {
            let complete_payload_spans = fdm_entry_complete_payload_spans(candidate, entry);
            let complete_payloads = complete_payload_spans.len();
            let frame_record =
                fdm_frame_record_for_entry(document.object_frame_records(), entry.row_index());
            candidate_count += 1;
            complete_payload_count += complete_payloads;
            if frame_record.is_some() {
                frame_linked_count += 1;
            } else {
                missing_frame_count += 1;
            }
            let reason =
                fdm_frame_link_render_blocked_reason(frame_record, entry, &complete_payload_spans);

            write_stdout_line(&format!(
                "object-fdm-frame-link\tsource={}\tindex={}\trow={}\timage-hits={}\tcomplete-payloads={}\tframe-linked={}\tframe-source={}\tframe-row={}\tframe-start={}\tframe-object-id={}\tframe-kind={}\tframe-type={}\tframe-geometry={}\tframe-size={}\tpayload-dimensions={}\tdimensioned-payloads={}\tbest-aspect-delta-permille={}\tlink-basis=fdm-row-index-to-frame-object-id\trenderable=false\treason={}\tdecoded=false",
                escaped_path(candidate.path()),
                escaped_path(entry.index_path()),
                entry.row_index(),
                entry.segment_image_signature_hits().len(),
                complete_payloads,
                frame_record.is_some(),
                format_optional_text(frame_record.map(ObjectFrameRecordCandidate::source_path)),
                format_optional_usize(frame_record.map(ObjectFrameRecordCandidate::row_index)),
                format_optional_usize(frame_record.map(ObjectFrameRecordCandidate::row_start)),
                format_optional_u16_decimal(
                    frame_record.map(ObjectFrameRecordCandidate::object_id)
                ),
                format_optional_u16_hex(frame_record.map(ObjectFrameRecordCandidate::record_kind)),
                format_optional_u16_hex(frame_record.map(ObjectFrameRecordCandidate::object_type)),
                format_optional_frame_geometry(frame_record),
                format_optional_frame_size(frame_record),
                format_fdm_payload_dimensions(&complete_payload_spans),
                fdm_payload_dimension_count(&complete_payload_spans),
                format_optional_u64(best_frame_payload_aspect_delta_permille(
                    frame_record,
                    &complete_payload_spans
                )),
                reason
            ))?;
        }
    }

    write_stdout_line(&format!(
        "summary\tsources={}\tcandidates={}\tframe-linked={}\tmissing-frame={}\tcomplete-payloads={}\trenderable={}\tdecoded=false",
        source_count,
        candidate_count,
        frame_linked_count,
        missing_frame_count,
        complete_payload_count,
        renderable_count
    ))?;
    Ok(())
}

pub(crate) fn run_object_fdm_index(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "object-fdm-index")?;
    let bytes = read_file(path)?;
    let streams = readable_cfb_streams(&bytes)?;
    let model_document = parse_document(&bytes).ok();
    let mut index_count = 0usize;
    let mut parsed_entries = 0usize;
    let mut entries_with_images = 0usize;
    let mut image_hit_count = 0usize;
    let mut offset_field_ref_rows = 0usize;
    let mut offset_field_ref_count = 0usize;
    let mut missing_vector_count = 0usize;

    for (index_path, index_stream) in streams
        .iter()
        .filter(|(path, _)| path.ends_with("/FDMIndex"))
    {
        index_count += 1;
        let Some(vector_path) = fdm_vector_path_for_index(index_path) else {
            missing_vector_count += 1;
            continue;
        };
        let Some(vector_stream) = streams.get(&vector_path) else {
            missing_vector_count += 1;
            write_stdout_line(&format!(
                "object-fdm-index-summary\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes=0\tdeclared-count={}\tparsed-entries=0\ttrailing-bytes=0\tentries-with-image=0\timage-hits=0\tvector-missing=true\tdecoded=false",
                escaped_path(index_path),
                escaped_path(&vector_path),
                index_stream.len(),
                format_optional_usize(fdm_index_declared_count(index_stream))
            ))?;
            continue;
        };

        let entries = parse_fdm_index_entries(index_stream, vector_stream.len());
        let raw_vector_commands = model_document
            .as_ref()
            .and_then(|document| fdm_raw_vector_commands_for_path(document, &vector_path))
            .unwrap_or_default();
        let vector_hits = image_signature_hits(vector_stream);
        let mut index_entries_with_images = 0usize;
        let mut index_image_hits = 0usize;
        let mut index_offset_field_ref_rows = 0usize;
        let mut index_offset_field_ref_count = 0usize;
        for entry in &entries {
            let segment = fdm_vector_segment(entry.vector_offset, &entries, vector_stream);
            let segment_hits = fdm_segment_signature_hits(&vector_hits, segment.start, segment.end);
            if !segment_hits.is_empty() {
                index_entries_with_images += 1;
                index_image_hits += segment_hits.len();
            }
            let offset_field_refs =
                fdm_index_offset_field_reference_summaries(entry, raw_vector_commands);
            if !offset_field_refs.is_empty() {
                index_offset_field_ref_rows += 1;
                index_offset_field_ref_count += offset_field_refs.len();
            }
        }
        parsed_entries += entries.len();
        entries_with_images += index_entries_with_images;
        image_hit_count += index_image_hits;
        offset_field_ref_rows += index_offset_field_ref_rows;
        offset_field_ref_count += index_offset_field_ref_count;

        write_stdout_line(&format!(
            "object-fdm-index-summary\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes={}\tdeclared-count={}\tparsed-entries={}\ttrailing-bytes={}\tentries-with-image={}\timage-hits={}\toffset-field-ref-rows={}\toffset-field-refs={}\tvector-missing=false\tdecoded=false",
            escaped_path(index_path),
            escaped_path(&vector_path),
            index_stream.len(),
            vector_stream.len(),
            format_optional_usize(fdm_index_declared_count(index_stream)),
            entries.len(),
            fdm_index_trailing_bytes(index_stream),
            index_entries_with_images,
            index_image_hits,
            index_offset_field_ref_rows,
            index_offset_field_ref_count
        ))?;

        for entry in entries.iter() {
            let segment = fdm_vector_segment(entry.vector_offset, &entries, vector_stream);
            let segment_hits = fdm_segment_signature_hits(&vector_hits, segment.start, segment.end);
            let relative_hits = fdm_relative_signature_hits(&segment_hits, segment.start);
            let vector_prefix = vector_stream
                .get(segment.start..segment.end)
                .unwrap_or_default();
            let offset_field_refs =
                fdm_index_offset_field_reference_summaries(entry, raw_vector_commands);
            write_stdout_line(&format!(
                "object-fdm-index-entry\tindex={}\tvector={}\trow={}\tindex-offset={}\tvector-offset={}\tnext-vector-offset={}\tvector-length={}\tkind=0x{:04x}\tbbox={},{},{},{}\tvalid-vector-offset={}\tvector-prefix={}\timage-signatures={}\tsegment-image-signatures={}\toffset-field-refs={}\tdecoded=false",
                escaped_path(index_path),
                escaped_path(&vector_path),
                entry.row_index,
                entry.index_offset,
                entry.vector_offset,
                segment.end,
                segment.end.saturating_sub(segment.start),
                entry.kind,
                entry.left,
                entry.top,
                entry.right,
                entry.bottom,
                entry.valid_vector_offset,
                format_hex_preview(vector_prefix, OBJECT_STREAM_PREFIX_PREVIEW_BYTES),
                format_object_signature_hits(&segment_hits),
                format_object_signature_hits(&relative_hits),
                format_offset_field_refs(&offset_field_refs)
            ))?;
        }
    }

    write_stdout_line(&format!(
        "summary\tindexes={}\tentries={}\tentries-with-image={}\timage-hits={}\toffset-field-ref-rows={}\toffset-field-refs={}\tmissing-vectors={}\tdecoded=false",
        index_count,
        parsed_entries,
        entries_with_images,
        image_hit_count,
        offset_field_ref_rows,
        offset_field_ref_count,
        missing_vector_count
    ))?;
    Ok(())
}

pub(crate) fn run_object_fdm_index_shape(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-fdm-index-shape")?;
    let bytes = read_file(path)?;
    let streams = readable_cfb_streams(&bytes)?;
    let mut index_count = 0usize;
    let mut header_v1_count = 0usize;
    let mut unknown_header_count = 0usize;
    let mut declared_plausible_count = 0usize;
    let mut stream_rows = 0usize;
    let mut stream_invalid_rows = 0usize;
    let mut declared_rows = 0usize;
    let mut declared_invalid_rows = 0usize;
    let mut declared_image_hits = 0usize;
    let mut shape_counts = BTreeMap::<String, usize>::new();

    for (index_path, index_stream) in streams
        .iter()
        .filter(|(path, _)| path.ends_with("/FDMIndex"))
    {
        index_count += 1;
        let Some(vector_path) = fdm_vector_path_for_index(index_path) else {
            continue;
        };
        let header_family = fdm_index_header_family(index_stream);
        if header_family == FDM_INDEX_HEADER_V1 {
            header_v1_count += 1;
        } else {
            unknown_header_count += 1;
        }
        let declared_count = fdm_index_declared_count(index_stream);
        let Some(vector_stream) = streams.get(&vector_path) else {
            *shape_counts
                .entry("missing-vector".to_string())
                .or_default() += 1;
            write_stdout_line(&format!(
                "object-fdm-index-shape\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes=0\theader-family={}\theader-u16be={}\tdeclared-count={}\tdeclared-plausible=false\trow22-stream-rows=0\trow22-trailing-bytes=0\tdeclared-row22=-\tpost-declared-bytes=-\tall-valid=0\tall-invalid=0\tall-image-rows=0\tall-image-hits=0\tdeclared-valid=0\tdeclared-invalid=0\tdeclared-image-rows=0\tdeclared-image-hits=0\tfirst-invalid-row=-\tfirst-invalid-offset=-\tshape=missing-vector\tdecoded=false",
                escaped_path(index_path),
                escaped_path(&vector_path),
                index_stream.len(),
                header_family,
                format_fdm_index_header_u16(index_stream),
                format_optional_usize(declared_count)
            ))?;
            continue;
        };

        let entries = parse_fdm_index_entries(index_stream, vector_stream.len());
        let vector_hits = image_signature_hits(vector_stream);
        let declared_plausible = header_family == FDM_INDEX_HEADER_V1
            && declared_count.is_some_and(|count| count <= entries.len());
        let declared_entry_count = if declared_plausible {
            declared_count.unwrap_or_default()
        } else {
            0
        };
        let declared_entries = &entries[..declared_entry_count];
        let all_stats = fdm_index_entry_stats(&entries, &vector_hits, vector_stream);
        let declared_stats = fdm_index_entry_stats(declared_entries, &vector_hits, vector_stream);
        let post_declared_bytes = declared_plausible.then(|| {
            index_stream.len().saturating_sub(
                FDM_INDEX_HEADER_BYTES + declared_entry_count * FDM_INDEX_ENTRY_BYTES,
            )
        });
        let shape = fdm_index_shape_family(
            header_family,
            declared_plausible,
            entries.len(),
            fdm_index_trailing_bytes(index_stream),
            declared_entry_count,
            &all_stats,
            &declared_stats,
        );

        if declared_plausible {
            declared_plausible_count += 1;
        }
        stream_rows += all_stats.rows;
        stream_invalid_rows += all_stats.invalid_offsets;
        declared_rows += declared_stats.rows;
        declared_invalid_rows += declared_stats.invalid_offsets;
        declared_image_hits += declared_stats.image_hits;
        *shape_counts.entry(shape.to_string()).or_default() += 1;

        write_stdout_line(&format!(
            "object-fdm-index-shape\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes={}\theader-family={}\theader-u16be={}\tdeclared-count={}\tdeclared-plausible={}\trow22-stream-rows={}\trow22-trailing-bytes={}\tdeclared-row22={}\tpost-declared-bytes={}\tall-valid={}\tall-invalid={}\tall-image-rows={}\tall-image-hits={}\tdeclared-valid={}\tdeclared-invalid={}\tdeclared-image-rows={}\tdeclared-image-hits={}\tfirst-invalid-row={}\tfirst-invalid-offset={}\tshape={}\tdecoded=false",
            escaped_path(index_path),
            escaped_path(&vector_path),
            index_stream.len(),
            vector_stream.len(),
            header_family,
            format_fdm_index_header_u16(index_stream),
            format_optional_usize(declared_count),
            declared_plausible,
            entries.len(),
            fdm_index_trailing_bytes(index_stream),
            format_optional_usize(declared_plausible.then_some(declared_entry_count)),
            format_optional_usize(post_declared_bytes),
            all_stats.valid_offsets,
            all_stats.invalid_offsets,
            all_stats.image_rows,
            all_stats.image_hits,
            declared_stats.valid_offsets,
            declared_stats.invalid_offsets,
            declared_stats.image_rows,
            declared_stats.image_hits,
            format_optional_usize(all_stats.first_invalid_row),
            format_optional_usize(all_stats.first_invalid_offset),
            shape
        ))?;
    }

    write_stdout_line(&format!(
        "summary\tindexes={}\theader-v1={}\tunknown-header={}\tdeclared-plausible={}\tstream-rows={}\tstream-invalid={}\tdeclared-rows={}\tdeclared-invalid={}\tdeclared-image-hits={}\tshapes={}\tdecoded=false",
        index_count,
        header_v1_count,
        unknown_header_count,
        declared_plausible_count,
        stream_rows,
        stream_invalid_rows,
        declared_rows,
        declared_invalid_rows,
        declared_image_hits,
        format_string_counts(&shape_counts)
    ))?;
    Ok(())
}

pub(crate) fn run_object_fdm_index_rows(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-fdm-index-rows")?;
    let bytes = read_file(path)?;
    let streams = readable_cfb_streams(&bytes)?;
    let model_document = parse_document(&bytes).ok();
    let mut index_count = 0usize;
    let mut row_count = 0usize;
    let mut declared_rows = 0usize;
    let mut post_declared_rows = 0usize;
    let mut raw_rows = 0usize;
    let mut valid_rows = 0usize;
    let mut invalid_rows = 0usize;
    let mut image_hits = 0usize;
    let mut offset_field_ref_rows = 0usize;
    let mut offset_field_ref_count = 0usize;
    let mut missing_vector_count = 0usize;
    let mut role_counts = BTreeMap::<String, usize>::new();

    for (index_path, index_stream) in streams
        .iter()
        .filter(|(path, _)| path.ends_with("/FDMIndex"))
    {
        index_count += 1;
        let Some(vector_path) = fdm_vector_path_for_index(index_path) else {
            continue;
        };
        let header_family = fdm_index_header_family(index_stream);
        let declared_count = fdm_index_declared_count(index_stream);
        let Some(vector_stream) = streams.get(&vector_path) else {
            missing_vector_count += 1;
            write_stdout_line(&format!(
                "object-fdm-index-rows-summary\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes=0\theader-family={}\tdeclared-count={}\trows=0\tdeclared-rows=0\tpost-declared-rows=0\traw-rows=0\tvalid-rows=0\tinvalid-rows=0\timage-hits=0\toffset-field-ref-rows=0\toffset-field-refs=0\troles=-\tvector-missing=true\tdecoded=false",
                escaped_path(index_path),
                escaped_path(&vector_path),
                index_stream.len(),
                header_family,
                format_optional_usize(declared_count)
            ))?;
            continue;
        };

        let entries = parse_fdm_index_entries(index_stream, vector_stream.len());
        let declared_plausible = header_family == FDM_INDEX_HEADER_V1
            && declared_count.is_some_and(|count| count <= entries.len());
        let declared_entry_count = if declared_plausible {
            declared_count.unwrap_or_default()
        } else {
            0
        };
        let raw_vector_commands = model_document
            .as_ref()
            .and_then(|document| fdm_raw_vector_commands_for_path(document, &vector_path))
            .unwrap_or_default();
        let vector_hits = image_signature_hits(vector_stream);
        let mut index_rows = 0usize;
        let mut index_declared_rows = 0usize;
        let mut index_post_declared_rows = 0usize;
        let mut index_raw_rows = 0usize;
        let mut index_valid_rows = 0usize;
        let mut index_invalid_rows = 0usize;
        let mut index_image_hits = 0usize;
        let mut index_offset_field_ref_rows = 0usize;
        let mut index_offset_field_ref_count = 0usize;
        let mut index_role_counts = BTreeMap::<String, usize>::new();

        for entry in &entries {
            let scope =
                fdm_index_row_scope(entry.row_index, declared_plausible, declared_entry_count);
            let role = fdm_index_row_role(entry);
            let segment = fdm_vector_segment(entry.vector_offset, &entries, vector_stream);
            let segment_hits = fdm_segment_signature_hits(&vector_hits, segment.start, segment.end);
            let relative_hits = fdm_relative_signature_hits(&segment_hits, segment.start);
            let offset_field_refs =
                fdm_index_offset_field_reference_summaries(entry, raw_vector_commands);

            index_rows += 1;
            match scope {
                "declared" => index_declared_rows += 1,
                "post-declared" => index_post_declared_rows += 1,
                _ => index_raw_rows += 1,
            }
            if entry.valid_vector_offset {
                index_valid_rows += 1;
            } else {
                index_invalid_rows += 1;
            }
            index_image_hits += segment_hits.len();
            if !offset_field_refs.is_empty() {
                index_offset_field_ref_rows += 1;
                index_offset_field_ref_count += offset_field_refs.len();
            }
            *index_role_counts.entry(role.to_string()).or_default() += 1;

            write_stdout_line(&format!(
                "object-fdm-index-row\tindex={}\tvector={}\trow={}\tscope={}\trole={}\tindex-offset={}\tvector-offset={}\tnext-vector-offset={}\tvector-length={}\tkind=0x{:04x}\tbbox={},{},{},{}\tvalid-vector-offset={}\tbe16={}\ti16={}\trow-bytes={}\timage-signatures={}\tsegment-image-signatures={}\toffset-field-refs={}\tdecoded=false",
                escaped_path(index_path),
                escaped_path(&vector_path),
                entry.row_index,
                scope,
                role,
                entry.index_offset,
                entry.vector_offset,
                segment.end,
                segment.end.saturating_sub(segment.start),
                entry.kind,
                entry.left,
                entry.top,
                entry.right,
                entry.bottom,
                entry.valid_vector_offset,
                format_be16_hex_fields(&entry.row),
                format_be16_signed_fields(&entry.row),
                format_hex_preview(&entry.row, FDM_INDEX_ENTRY_BYTES),
                format_object_signature_hits(&segment_hits),
                format_object_signature_hits(&relative_hits),
                format_offset_field_refs(&offset_field_refs)
            ))?;
        }

        row_count += index_rows;
        declared_rows += index_declared_rows;
        post_declared_rows += index_post_declared_rows;
        raw_rows += index_raw_rows;
        valid_rows += index_valid_rows;
        invalid_rows += index_invalid_rows;
        image_hits += index_image_hits;
        offset_field_ref_rows += index_offset_field_ref_rows;
        offset_field_ref_count += index_offset_field_ref_count;
        for (role, count) in index_role_counts.iter() {
            *role_counts.entry(role.clone()).or_default() += *count;
        }

        write_stdout_line(&format!(
            "object-fdm-index-rows-summary\tindex={}\tvector={}\tindex-bytes={}\tvector-bytes={}\theader-family={}\tdeclared-count={}\trows={}\tdeclared-rows={}\tpost-declared-rows={}\traw-rows={}\tvalid-rows={}\tinvalid-rows={}\timage-hits={}\toffset-field-ref-rows={}\toffset-field-refs={}\troles={}\tvector-missing=false\tdecoded=false",
            escaped_path(index_path),
            escaped_path(&vector_path),
            index_stream.len(),
            vector_stream.len(),
            header_family,
            format_optional_usize(declared_count),
            index_rows,
            index_declared_rows,
            index_post_declared_rows,
            index_raw_rows,
            index_valid_rows,
            index_invalid_rows,
            index_image_hits,
            index_offset_field_ref_rows,
            index_offset_field_ref_count,
            format_string_counts(&index_role_counts)
        ))?;
    }

    write_stdout_line(&format!(
        "summary\tindexes={}\trows={}\tdeclared-rows={}\tpost-declared-rows={}\traw-rows={}\tvalid-rows={}\tinvalid-rows={}\timage-hits={}\toffset-field-ref-rows={}\toffset-field-refs={}\tmissing-vectors={}\troles={}\tdecoded=false",
        index_count,
        row_count,
        declared_rows,
        post_declared_rows,
        raw_rows,
        valid_rows,
        invalid_rows,
        image_hits,
        offset_field_ref_rows,
        offset_field_ref_count,
        missing_vector_count,
        format_string_counts(&role_counts)
    ))?;
    Ok(())
}
