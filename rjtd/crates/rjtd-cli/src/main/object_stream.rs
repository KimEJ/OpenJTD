use std::collections::BTreeMap;

use rjtd_core::container::{EntryKind, inspect_cfb_entries, read_cfb_stream};
use rjtd_model::parse_document;

use crate::input::read_file;

use super::object_fdm_support::*;
use super::object_stream_support::*;
use super::style_support::{format_optional_text, format_string_counts, format_usize_values};
use super::support::*;
use super::text_position_count_support::{
    format_be16_hex_fields, format_le16_fields, format_optional_u16_decimal,
};

pub(crate) fn run_so_records(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "so-records")?;
    let bytes = read_file(path)?;
    let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        match read_cfb_stream(&bytes, entry.path()) {
            Ok(stream) => {
                for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
                    write_stdout_line(&format!(
                        "record\t{}\t{}\t{}\t{}",
                        escaped_path(entry.path()),
                        offset,
                        format_le32_fields(&stream[offset..], SO_RECORD_DWORDS),
                        bytes_to_hex(stream_tail(&stream, offset, SO_RECORD_BYTES))
                    ))?;
                }
            }
            Err(error) => {
                write_stdout_line(&format!(
                    "unreadable\t{}\t{}",
                    escaped_path(entry.path()),
                    error
                ))?;
            }
        }
    }
    Ok(())
}

pub(crate) fn run_object_stream_candidates(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-stream-candidates")?;
    let bytes = read_file(path)?;
    let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
    let mut stream_count = 0usize;
    let mut unreadable_count = 0usize;
    let mut candidates = Vec::new();
    let mut reason_counts = BTreeMap::<&'static str, usize>::new();

    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        stream_count += 1;
        let stream = match read_cfb_stream(&bytes, entry.path()) {
            Ok(stream) => stream,
            Err(error) => {
                unreadable_count += 1;
                write_stdout_line(&format!(
                    "unreadable\t{}\t{}",
                    escaped_path(entry.path()),
                    error
                ))?;
                continue;
            }
        };

        let Some(candidate) = classify_object_stream_candidate(entry.path(), &stream) else {
            continue;
        };
        for reason in &candidate.reasons {
            *reason_counts.entry(reason).or_default() += 1;
        }
        candidates.push(candidate);
    }

    let visual_list_raster_count = candidates
        .iter()
        .filter(|candidate| candidate.visual_list.is_some())
        .count();
    let embedded_press_snapshot_count = candidates
        .iter()
        .filter(|candidate| candidate.embedded_press_snapshot.is_some())
        .count();
    let jseq3_formula_count = candidates
        .iter()
        .filter(|candidate| candidate.jseq3_formula.is_some())
        .count();
    let jsfart_stream_profile_count = candidates
        .iter()
        .filter(|candidate| candidate.jsfart_stream_profile.is_some())
        .count();
    write_stdout_line(&format!(
        "summary\tstreams={}\tcandidates={}\tunreadable={}\tobject-path={}\timage-path={}\tshape-path={}\ttable-path={}\tvisual-list-path={}\tvisual-list-raster={}\tfigure-link={}\tembedded-press-snapshot={}\tjseq3-formula={}\tjsfart-stream-profile={}\tso-marker={}\timage-signature={}\tsvg-signature={}\tdecoded=false",
        stream_count,
        candidates.len(),
        unreadable_count,
        object_stream_reason_count(&reason_counts, "object-path"),
        object_stream_reason_count(&reason_counts, "image-path"),
        object_stream_reason_count(&reason_counts, "shape-path"),
        object_stream_reason_count(&reason_counts, "table-path"),
        object_stream_reason_count(&reason_counts, "visual-list-path"),
        visual_list_raster_count,
        object_stream_reason_count(&reason_counts, "figure-link"),
        embedded_press_snapshot_count,
        jseq3_formula_count,
        jsfart_stream_profile_count,
        object_stream_reason_count(&reason_counts, "so-marker"),
        object_stream_reason_count(&reason_counts, "image-signature"),
        object_stream_reason_count(&reason_counts, "svg-signature"),
    ))?;

    for (index, candidate) in candidates.iter().enumerate() {
        write_stdout_line(&format!(
            "object-stream-candidate\t{}\tstream={}\tsize={}\treasons={}\timage-signatures={}\tsvg-offsets={}\tso-offsets={}\tvisual-list={}\tembedded-press-snapshot={}\tjseq3-formula={}\tjsfart-stream-profile={}\tprefix={}\tdecoded=false",
            index,
            escaped_path(&candidate.path),
            candidate.size,
            candidate.reasons.join(","),
            format_object_signature_hits(&candidate.image_signature_hits),
            format_usize_hit_list(&candidate.svg_offsets),
            format_usize_hit_list(&candidate.so_offsets),
            format_visual_list_candidate(candidate.visual_list.as_ref()),
            format_embedded_press_snapshot_candidate(candidate.embedded_press_snapshot.as_ref()),
            format_jseq3_formula_candidate(candidate.jseq3_formula.as_ref()),
            format_jsfart_stream_profile_candidate(candidate.jsfart_stream_profile.as_ref()),
            candidate.prefix_hex,
        ))?;
    }
    Ok(())
}

pub(crate) fn run_object_ownership_references(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-ownership-references")?;
    let bytes = read_file(path)?;
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let streams = readable_cfb_streams(&bytes)?;
    let mut source_count = 0usize;
    let mut reference_count = 0usize;
    let mut reported_offset_count = 0usize;
    let mut missing_target_count = 0usize;

    for candidate in document.object_stream_candidates() {
        let references = candidate.ownership_reference_candidates();
        if references.is_empty() {
            continue;
        }
        source_count += 1;
        reference_count += references.len();
        for reference in references {
            reported_offset_count += reference.offsets().len();
            let Some(target_stream) = streams.get(reference.target_path()) else {
                missing_target_count += 1;
                write_stdout_line(&format!(
                    "object-ownership-reference\tsource={}\ttarget={}\tencoding={}\toffset=-\ttotal={}\ttarget-missing=true\tdecoded=false",
                    escaped_path(candidate.path()),
                    escaped_path(reference.target_path()),
                    reference.encoding(),
                    reference.total_matches()
                ))?;
                continue;
            };

            let pattern_len = object_reference_pattern_len(reference.encoding());
            for offset in reference.offsets() {
                let context = object_reference_context(target_stream, *offset, pattern_len);
                write_stdout_line(&format!(
                    "object-ownership-reference\tsource={}\ttarget={}\tencoding={}\toffset={}\ttotal={}\tmod2={}\tmod4={}\twindow-start={}\twindow-hex={}\tle16={}\tbe16={}\tle32={}\tbe32={}\tdecoded=false",
                    escaped_path(candidate.path()),
                    escaped_path(reference.target_path()),
                    reference.encoding(),
                    offset,
                    reference.total_matches(),
                    offset % 2,
                    offset % 4,
                    context.start,
                    context.hex,
                    format_optional_u16_decimal(read_le16_candidate(target_stream, *offset)),
                    format_optional_u16_decimal(read_be16_candidate(target_stream, *offset)),
                    format_optional_u32(read_le32_candidate(target_stream, *offset)),
                    format_optional_u32(read_be32_at(target_stream, *offset))
                ))?;
            }
        }
    }

    write_stdout_line(&format!(
        "summary\tsources={}\treferences={}\treported-offsets={}\ttarget-missing={}\tdecoded=false",
        source_count, reference_count, reported_offset_count, missing_target_count
    ))?;
    Ok(())
}

pub(crate) fn run_object_ownership_reference_fields(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-ownership-reference-fields")?;
    let bytes = read_file(path)?;
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let mut summaries = BTreeMap::<ObjectReferenceFieldKey, ObjectReferenceFieldSummary>::new();
    let mut source_count = 0usize;
    let mut reference_count = 0usize;
    let mut reported_offset_count = 0usize;

    for candidate in document.object_stream_candidates() {
        let Some(ownership) = candidate.ownership_candidate() else {
            continue;
        };
        let embedding_index = ownership.embedding_index();
        let references = candidate.ownership_reference_candidates();
        if references.is_empty() {
            continue;
        }
        source_count += 1;
        reference_count += references.len();

        for reference in references {
            let pattern_len = object_reference_pattern_len(reference.encoding());
            for offset in reference.offsets() {
                reported_offset_count += 1;
                for stride in OBJECT_REFERENCE_FIELD_STRIDES {
                    let field_offset = offset % stride;
                    let key = ObjectReferenceFieldKey::new(
                        reference.target_path(),
                        reference.encoding(),
                        *stride,
                        field_offset,
                    );
                    let summary = summaries.entry(key).or_default();
                    summary.matches += 1;
                    summary.row_indexes.insert(offset / stride);
                    summary.source_streams.insert(candidate.path().to_string());
                    if let Some(index) = embedding_index {
                        summary.embedding_indexes.insert(index);
                    }
                    if field_offset + pattern_len > *stride {
                        summary.cross_row_matches += 1;
                    }
                }
            }
        }
    }

    write_stdout_line(&format!(
        "summary\tsources={}\treferences={}\treported-offsets={}\tfield-groups={}\tstrides={}\tdecoded=false",
        source_count,
        reference_count,
        reported_offset_count,
        summaries.len(),
        format_usize_values(OBJECT_REFERENCE_FIELD_STRIDES)
    ))?;

    for (key, summary) in summaries {
        write_stdout_line(&format!(
            "object-ownership-reference-field\ttarget={}\tencoding={}\tstride={}\tfield-offset={}\tmatches={}\tsource-count={}\tembedding-indexes={}\trow-indexes={}\tcross-row={}\tdecoded=false",
            escaped_path(&key.target_path),
            key.encoding,
            key.stride,
            key.field_offset,
            summary.matches,
            summary.source_streams.len(),
            format_usize_set(&summary.embedding_indexes),
            format_usize_set(&summary.row_indexes),
            summary.cross_row_matches
        ))?;
    }
    Ok(())
}

pub(crate) fn run_object_frame_reference_records(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-frame-reference-records")?;
    let bytes = read_file(path)?;
    let collection = collect_object_frame_reference_records(&bytes)?;
    for record in &collection.records {
        write_stdout_line(&format!(
            "object-frame-reference-record\tsource={}\tembedding={}\ttarget={}\tencoding={}\tstride={}\tfield-offset={}\toffset={}\trow-index={}\trow-start={}\tcandidate={}\trow-hex={}\trow-be16={}\trow-le16={}\trow-be32={}\trow-le32={}\tdecoded=false",
            escaped_path(&record.source_path),
            format_optional_usize(record.embedding_index),
            escaped_path(&record.target_path),
            record.encoding,
            record.stride,
            record.field_offset,
            record.offset,
            record.row_index,
            record.row_start,
            record.candidate,
            bytes_to_hex(&record.row),
            format_be16_hex_fields(&record.row),
            format_le16_fields(&record.row),
            format_be32_fields(&record.row),
            format_le32_fields(&record.row, record.stride / 4)
        ))?;
    }

    write_stdout_line(&format!(
        "summary\tsources={}\tframe-references={}\trecords={}\tskipped={}\tcandidates={}\tdecoded=false",
        collection.source_count,
        collection.reference_count,
        collection.records.len(),
        collection.skipped_count,
        format_frame_reference_record_candidates()
    ))?;
    Ok(())
}

pub(crate) fn run_object_frame_record_families(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-frame-record-families")?;
    let bytes = read_file(path)?;
    let collection = collect_object_frame_reference_records(&bytes)?;
    let mut families = BTreeMap::<String, ObjectFrameRecordFamilySummary>::new();

    for record in &collection.records {
        let family = classify_object_frame_reference_record(record);
        let summary = families.entry(family.to_string()).or_default();
        summary.rows += 1;
        summary.candidates.insert(record.candidate.clone());
        if let Some(index) = record.embedding_index {
            summary.embedding_indexes.insert(index);
        }
        summary.examples.insert(bytes_to_hex(&record.row));
    }

    for (family, summary) in &families {
        write_stdout_line(&format!(
            "object-frame-record-family\tfamily={}\trows={}\tcandidates={}\tembeddings={}\texamples={}\tdecoded=false",
            family,
            summary.rows,
            format_string_set(&summary.candidates),
            format_usize_set(&summary.embedding_indexes),
            format_string_set(&summary.examples)
        ))?;
    }

    write_stdout_line(&format!(
        "summary\tfamilies={}\trecords={}\tskipped={}\tcandidates={}\tdecoded=false",
        families.len(),
        collection.records.len(),
        collection.skipped_count,
        format_frame_reference_record_candidates()
    ))?;
    Ok(())
}

pub(crate) fn run_object_frame_row_links(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "object-frame-row-links")?;
    let bytes = read_file(path)?;
    let collection = collect_object_frame_reference_records(&bytes)?;
    let row12_records = collection
        .records
        .iter()
        .filter(|record| record.stride == 12)
        .collect::<Vec<_>>();
    let mut row20_count = 0usize;
    let mut linked_count = 0usize;
    let mut relation_counts = BTreeMap::<String, usize>::new();
    let mut pair_counts = BTreeMap::<String, usize>::new();

    for record in collection
        .records
        .iter()
        .filter(|record| record.stride == 20 && record.field_offset == 15)
    {
        row20_count += 1;
        let suffix = object_frame_row_suffix(record, 12).unwrap_or(&[]);
        let (relation, matched) = find_object_frame_suffix_match(record, suffix, &row12_records);
        if matched.is_some() {
            linked_count += 1;
        }
        *relation_counts.entry(relation.to_string()).or_insert(0) += 1;

        let row_family = classify_object_frame_reference_record(record);
        let suffix_family = matched
            .map(classify_object_frame_reference_record)
            .unwrap_or("unmatched-suffix");
        *pair_counts
            .entry(format!("{row_family}->{suffix_family}"))
            .or_insert(0) += 1;

        write_stdout_line(&format!(
            "object-frame-row-link\tsource={}\tembedding={}\trow20-family={}\trow20-start={}\trow20-index={}\tprefix-hex={}\tsuffix-hex={}\trelation={}\tsuffix-family={}\tmatched-source={}\tmatched-row-start={}\tmatched-row-index={}\tdecoded=false",
            escaped_path(&record.source_path),
            format_optional_usize(record.embedding_index),
            row_family,
            record.row_start,
            record.row_index,
            bytes_to_hex(object_frame_row_prefix(record, 12).unwrap_or(&[])),
            bytes_to_hex(suffix),
            relation,
            suffix_family,
            format_optional_text(matched.map(|record| record.source_path.as_str())),
            format_optional_usize(matched.map(|record| record.row_start)),
            format_optional_usize(matched.map(|record| record.row_index))
        ))?;
    }

    write_stdout_line(&format!(
        "summary\trow20={}\tlinked={}\tunlinked={}\trelations={}\tfamily-pairs={}\tdecoded=false",
        row20_count,
        linked_count,
        row20_count.saturating_sub(linked_count),
        format_string_counts(&relation_counts),
        format_string_counts(&pair_counts)
    ))?;
    Ok(())
}

pub(crate) fn run_so_record_clusters(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "so-record-clusters")?;
    let bytes = read_file(path)?;
    let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
    let mut clusters = BTreeMap::<Vec<u8>, Vec<String>>::new();
    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        let stream = match read_cfb_stream(&bytes, entry.path()) {
            Ok(stream) => stream,
            Err(error) => {
                write_stdout_line(&format!(
                    "unreadable\t{}\t{}",
                    escaped_path(entry.path()),
                    error
                ))?;
                continue;
            }
        };
        for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
            let raw = stream_tail(&stream, offset, SO_RECORD_BYTES).to_vec();
            clusters.entry(raw).or_default().push(format!(
                "{}@{}",
                escaped_path(entry.path()),
                offset
            ));
        }
    }

    for (raw, locations) in clusters {
        write_stdout_line(&format!(
            "cluster\t{}\t{}\t{}\t{}",
            locations.len(),
            format_le32_fields(&raw, SO_RECORD_DWORDS),
            bytes_to_hex(&raw),
            locations.join(",")
        ))?;
    }
    Ok(())
}

pub(crate) fn run_so_record_fields(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "so-record-fields")?;
    let bytes = read_file(path)?;
    let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        let stream = match read_cfb_stream(&bytes, entry.path()) {
            Ok(stream) => stream,
            Err(error) => {
                write_stdout_line(&format!(
                    "unreadable\t{}\t{}",
                    escaped_path(entry.path()),
                    error
                ))?;
                continue;
            }
        };
        for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
            for (field_index, field) in
                le32_dwords(stream_tail(&stream, offset, SO_RECORD_BYTES)).enumerate()
            {
                write_stdout_line(&format!(
                    "field\t{}\t{}\t{}\t0x{:08x}\t{}\t{}\t0x{:04x}\t{}\t0x{:04x}\t{}",
                    escaped_path(entry.path()),
                    offset,
                    field_index,
                    field,
                    field,
                    field as i32,
                    field as u16,
                    field as u16,
                    (field >> 16) as u16,
                    (field >> 16) as u16
                ))?;
            }
        }
    }
    Ok(())
}

pub(crate) fn run_so_record_geometry(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "so-record-geometry")?;
    let bytes = read_file(path)?;
    let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        let stream = match read_cfb_stream(&bytes, entry.path()) {
            Ok(stream) => stream,
            Err(error) => {
                write_stdout_line(&format!(
                    "unreadable\t{}\t{}",
                    escaped_path(entry.path()),
                    error
                ))?;
                continue;
            }
        };
        for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
            let raw = stream_tail(&stream, offset, SO_RECORD_BYTES);
            let fields = le32_dwords(raw).collect::<Vec<_>>();
            let (f1, f2, f3, f4, xyxy_width, xyxy_height, xywh_right, xywh_bottom) =
                format_so_geometry_candidate(&fields);
            write_stdout_line(&format!(
                "candidate\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                escaped_path(entry.path()),
                offset,
                classify_so_geometry_fields(&fields),
                f1,
                f2,
                f3,
                f4,
                xyxy_width,
                xyxy_height,
                xywh_right,
                xywh_bottom,
                bytes_to_hex(raw)
            ))?;
        }
    }
    Ok(())
}

pub(crate) fn run_so_record_halves(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "so-record-halves")?;
    let bytes = read_file(path)?;
    let entries = inspect_cfb_entries(&bytes).map_err(|error| error.to_string())?;
    for entry in entries
        .iter()
        .filter(|entry| entry.kind() == EntryKind::Stream)
    {
        let stream = match read_cfb_stream(&bytes, entry.path()) {
            Ok(stream) => stream,
            Err(error) => {
                write_stdout_line(&format!(
                    "unreadable\t{}\t{}",
                    escaped_path(entry.path()),
                    error
                ))?;
                continue;
            }
        };
        for offset in find_subslice_offsets(&stream, SO_RECORD_MARKER) {
            let raw = stream_tail(&stream, offset, SO_RECORD_BYTES);
            let fields = le32_dwords(raw).collect::<Vec<_>>();
            write_stdout_line(&format!(
                "halves\t{}\t{}\t{}\tlo_u16={}\thi_u16={}\tlo_i16={}\thi_i16={}\t{}",
                escaped_path(entry.path()),
                offset,
                classify_so_geometry_fields(&fields),
                format_so_u16_halves(&fields, false),
                format_so_u16_halves(&fields, true),
                format_so_i16_halves(&fields, false),
                format_so_i16_halves(&fields, true),
                bytes_to_hex(raw)
            ))?;
        }
    }
    Ok(())
}
