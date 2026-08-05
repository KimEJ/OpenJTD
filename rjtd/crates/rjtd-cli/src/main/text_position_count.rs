use std::collections::{BTreeMap, BTreeSet};

use rjtd_core::Error;
use rjtd_core::container::read_cfb_stream;
use rjtd_core::document_text::{map_document_text, read_document_text_payload};
use rjtd_core::document_text_position::read_document_text_position_tables;
use rjtd_core::layout_mark::{read_page_mark, read_paper_mark};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, TEXT_LAYOUT_STYLE_PATH, read_style_streams,
};

use crate::input::read_file;

use super::line_mark_support::{
    format_index_context, format_line_byte_offset_context, format_line_word_index_context,
};
use super::style_support::*;
use super::support::*;
use super::text_boundary_support::*;
use super::text_position_count_support::*;

pub(crate) fn run_text_position_counts(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-counts")?;
    let bytes = read_file(path)?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    let Some(header) = table.text_count_header() else {
        return Err("DocumentTextPositionTables missing TCntV.01 count table".into());
    };
    write_stdout_line(&format!(
        "header\t{}\t{}\t{}\t{}\t{}",
        header.kind(),
        header.reserved(),
        header.declared_count(),
        header.entries_offset(),
        table.text_count_entries().len()
    ))?;
    for entry in table.text_count_entries() {
        write_stdout_line(&format!(
            "entry\t{}\t{}\t{}\t{}",
            entry.index(),
            entry.start_offset(),
            entry.end_offset(),
            bytes_to_hex(entry.raw())
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());

    for entry in table.text_count_entries() {
        let start = entry.start_offset() as usize;
        let end = entry.end_offset() as usize;
        write_stdout_line(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            entry.index(),
            entry.start_offset(),
            entry.end_offset(),
            format_byte_context(map.entries(), start),
            format_byte_context(map.entries(), end),
            format_unit_context(map.entries(), start),
            format_unit_context(map.entries(), end)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_tail_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-tail-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        let t1 = tail_fields.get(1).copied();
        let t2 = tail_fields.get(2).copied();
        write_stdout_line(&format!(
            "tail-context\t{}\t{}\t{}\t{}\tt1={}\tt2={}\ttspan={}\tt1-byte={}\tt2-byte={}\tt1-unit={}\tt2-unit={}",
            entry.index(),
            family,
            chosen_start,
            chosen_end,
            format_optional_u16_decimal(t1),
            format_optional_u16_decimal(t2),
            format_optional_i64(optional_tail_span(t1, t2)),
            format_optional_byte_context(map.entries(), t1),
            format_optional_byte_context(map.entries(), t2),
            format_optional_unit_context(map.entries(), t1),
            format_optional_unit_context(map.entries(), t2)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_clusters(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-clusters")?;
    let bytes = read_file(path)?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }

    let mut clusters = BTreeMap::new();
    for entry in table.text_count_entries() {
        clusters
            .entry((entry.start_offset(), entry.end_offset()))
            .or_insert_with(Vec::new)
            .push(entry);
    }

    for ((start, end), entries) in clusters {
        let indexes = entries
            .iter()
            .map(|entry| entry.index().to_string())
            .collect::<Vec<_>>()
            .join(",");
        let tail_variants = entries
            .iter()
            .map(|entry| bytes_to_hex(&entry.raw()[8..]))
            .collect::<BTreeSet<_>>();
        let tail_variant_count = tail_variants.len();
        let tail_variants = tail_variants.into_iter().collect::<Vec<_>>().join(",");
        write_stdout_line(&format!(
            "{start}\t{end}\t{}\t{indexes}\t{tail_variant_count}\t{tail_variants}",
            entries.len(),
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_candidates(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-candidates")?;
    let bytes = read_file(path)?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        write_stdout_line(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            entry.index(),
            read_be32_candidate(raw, 0),
            read_be32_candidate(raw, 4),
            read_be32_candidate(raw, 1),
            read_be32_candidate(raw, 5),
            bytes_to_hex(raw)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_family(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-family")?;
    let bytes = read_file(path)?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let be0_start = read_be32_candidate(raw, 0);
        let be0_end = read_be32_candidate(raw, 4);
        let be1_start = read_be32_candidate(raw, 1);
        let be1_end = read_be32_candidate(raw, 5);
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        write_stdout_line(&format!(
            "family\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\tlead=0x{:02x}\ttail={}",
            entry.index(),
            family,
            chosen_start,
            chosen_end,
            be0_start,
            be0_end,
            be1_start,
            be1_end,
            raw[0],
            bytes_to_hex(&raw[tail_offset..])
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_fields(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-fields")?;
    let bytes = read_file(path)?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail = &raw[tail_offset..];
        write_stdout_line(&format!(
            "fields\t{}\t{}\t{}\t{}\t{}\tlead=0x{:02x}\ttail-offset={}\ttail-be16={}\ttail-extra={}\traw={}",
            entry.index(),
            family,
            chosen_start,
            chosen_end,
            chosen_end.saturating_sub(chosen_start),
            raw[0],
            tail_offset,
            format_be16_hex_fields(tail),
            format_tail_extra_byte(tail),
            bytes_to_hex(raw)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_field_deltas(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-field-deltas")?;
    let bytes = read_file(path)?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let chosen_span = chosen_end.saturating_sub(chosen_start);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        let t1 = tail_fields.get(1).copied();
        let t2 = tail_fields.get(2).copied();
        let tail_span = optional_tail_span(t1, t2);
        write_stdout_line(&format!(
            "delta\t{}\t{}\t{}\t{}\t{}\ttail-offset={}\tt1={}\tt2={}\ttspan={}\tspan-relation={}\tstart-minus-t1={}\tend-minus-t2={}\tt0={}\tt3={}\tt4={}\tt7={}\traw={}",
            entry.index(),
            family,
            chosen_start,
            chosen_end,
            chosen_span,
            tail_offset,
            format_optional_u16_decimal(t1),
            format_optional_u16_decimal(t2),
            format_optional_i64(tail_span),
            format_span_relation(chosen_span, tail_span),
            format_optional_i64(t1.map(|value| chosen_start as i64 - value as i64)),
            format_optional_i64(t2.map(|value| chosen_end as i64 - value as i64)),
            format_optional_u16_hex(tail_fields.first().copied()),
            format_optional_u16_hex(tail_fields.get(3).copied()),
            format_optional_u16_hex(tail_fields.get(4).copied()),
            format_optional_u16_hex(tail_fields.get(7).copied()),
            bytes_to_hex(raw)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_tail_delta_scan(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-tail-delta-scan")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());

    for delta in 0..=64usize {
        let mut endpoints = 0usize;
        let mut unit_hits = 0usize;
        let mut text_hits = 0usize;
        let mut both_unit_rows = 0usize;
        let mut both_text_rows = 0usize;

        for entry in table.text_count_entries() {
            let raw = entry.raw();
            let family = classify_text_count_entry_family(raw);
            let tail_offset = text_count_entry_tail_offset(family);
            let tail_fields = read_be16_fields(&raw[tail_offset..]);
            let t1 = tail_fields.get(1).copied();
            let t2 = tail_fields.get(2).copied();
            let t1_unit_hit = count_tail_delta_hit(map.entries(), t1, delta, false);
            let t2_unit_hit = count_tail_delta_hit(map.entries(), t2, delta, false);
            let t1_text_hit = count_tail_delta_hit(map.entries(), t1, delta, true);
            let t2_text_hit = count_tail_delta_hit(map.entries(), t2, delta, true);

            endpoints += usize::from(t1.is_some()) + usize::from(t2.is_some());
            unit_hits += usize::from(t1_unit_hit) + usize::from(t2_unit_hit);
            text_hits += usize::from(t1_text_hit) + usize::from(t2_text_hit);
            if t1_unit_hit && t2_unit_hit {
                both_unit_rows += 1;
            }
            if t1_text_hit && t2_text_hit {
                both_text_rows += 1;
            }
        }

        write_stdout_line(&format!(
            "delta\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            delta,
            table.text_count_entries().len(),
            endpoints,
            unit_hits,
            text_hits,
            both_unit_rows,
            both_text_rows
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_tail_delta_groups(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-tail-delta-groups")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());
    let mut groups: TailDeltaGroups = BTreeMap::new();

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        let key = (
            family,
            tail_fields.first().copied(),
            tail_fields.get(3).copied(),
            tail_fields.get(4).copied(),
            tail_fields.get(7).copied(),
        );
        groups
            .entry(key)
            .or_default()
            .push((tail_fields.get(1).copied(), tail_fields.get(2).copied()));
    }

    for ((family, t0, t3, t4, t7), rows) in groups {
        let endpoints = rows
            .iter()
            .map(|(t1, t2)| usize::from(t1.is_some()) + usize::from(t2.is_some()))
            .sum::<usize>();
        let best = best_tail_deltas(map.entries(), &rows);

        let delta0 = score_tail_delta_group(map.entries(), &rows, 0);
        let delta29 = score_tail_delta_group(map.entries(), &rows, 29);
        let delta30 = score_tail_delta_group(map.entries(), &rows, 30);
        write_stdout_line(&format!(
            "group\t{}\tt0={}\tt3={}\tt4={}\tt7={}\trows={}\tendpoints={}\tbest-unit={}\tbest-text={}\td0={}\td29={}\td30={}",
            family,
            format_optional_u16_hex(t0),
            format_optional_u16_hex(t3),
            format_optional_u16_hex(t4),
            format_optional_u16_hex(t7),
            rows.len(),
            endpoints,
            format_best_unit_delta(best.unit_delta, best.unit_score),
            format_best_text_delta(best.text_delta, best.text_score),
            format_tail_delta_score(delta0),
            format_tail_delta_score(delta29),
            format_tail_delta_score(delta30)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_tail_row_deltas(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-tail-row-deltas")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());
    let document_units = map
        .entries()
        .last()
        .map(|entry| entry.unit_end())
        .unwrap_or_default();
    write_stdout_line(&format!(
        "summary\tentries={}\tdoc-bytes={}\tdoc-units={}",
        table.text_count_entries().len(),
        payload.bytes().len(),
        document_units
    ))?;

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        let t1 = tail_fields.get(1).copied();
        let t2 = tail_fields.get(2).copied();
        let rows = [(t1, t2)];
        let best = best_tail_deltas(map.entries(), &rows);

        let delta0 = score_tail_delta_group(map.entries(), &rows, 0);
        let delta29 = score_tail_delta_group(map.entries(), &rows, 29);
        let delta30 = score_tail_delta_group(map.entries(), &rows, 30);
        write_stdout_line(&format!(
            "row\t{}\t{}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tspan={}\tt1={}\tt2={}\ttspan={}\tbest-unit={}\tbest-text={}\td0={}\td29={}\td30={}",
            entry.index(),
            family,
            format_optional_u16_hex(tail_fields.first().copied()),
            format_optional_u16_hex(tail_fields.get(3).copied()),
            format_optional_u16_hex(tail_fields.get(4).copied()),
            format_optional_u16_hex(tail_fields.get(7).copied()),
            chosen_start,
            chosen_end,
            chosen_end.saturating_sub(chosen_start),
            format_optional_u16_decimal(t1),
            format_optional_u16_decimal(t2),
            format_optional_i64(optional_tail_span(t1, t2)),
            format_best_unit_delta(best.unit_delta, best.unit_score),
            format_best_text_delta(best.text_delta, best.text_score),
            format_tail_delta_score(delta0),
            format_tail_delta_score(delta29),
            format_tail_delta_score(delta30)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_tail_row_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-tail-row-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        let t1 = tail_fields.get(1).copied();
        let t2 = tail_fields.get(2).copied();
        let rows = [(t1, t2)];
        let best = best_tail_deltas(map.entries(), &rows);
        write_stdout_line(&format!(
            "row-context\t{}\t{}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tt1={}\tt2={}\tbest-unit={}\tbest-text={}\tstart-byte={}\tend-byte={}\tstart-unit={}\tend-unit={}\tt1-unit-best={}\tt2-unit-best={}\tt1-text-best={}\tt2-text-best={}",
            entry.index(),
            family,
            format_optional_u16_hex(tail_fields.first().copied()),
            format_optional_u16_hex(tail_fields.get(3).copied()),
            format_optional_u16_hex(tail_fields.get(4).copied()),
            format_optional_u16_hex(tail_fields.get(7).copied()),
            chosen_start,
            chosen_end,
            format_optional_u16_decimal(t1),
            format_optional_u16_decimal(t2),
            format_best_unit_delta(best.unit_delta, best.unit_score),
            format_best_text_delta(best.text_delta, best.text_score),
            format_byte_context(map.entries(), chosen_start as usize),
            format_byte_context(map.entries(), chosen_end as usize),
            format_unit_context(map.entries(), chosen_start as usize),
            format_unit_context(map.entries(), chosen_end as usize),
            format_optional_unit_context_with_delta(map.entries(), t1, best.unit_delta),
            format_optional_unit_context_with_delta(map.entries(), t2, best.unit_delta),
            format_optional_unit_context_with_delta(map.entries(), t1, best.text_delta),
            format_optional_unit_context_with_delta(map.entries(), t2, best.text_delta)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_tail_field_roles(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-tail-field-roles")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document_units = map
        .entries()
        .last()
        .map(|entry| entry.unit_end())
        .unwrap_or_default();
    let (position_status, table) = match read_document_text_position_tables(&bytes) {
        Ok(table) => ("ok".to_string(), Some(table)),
        Err(Error::NotFound(_)) => ("missing".to_string(), None),
        Err(Error::InvalidData(message)) => (format!("invalid:{}", escaped_text(&message)), None),
        Err(error) => return Err(error.to_string()),
    };
    let text_count_entries = table
        .as_ref()
        .map(|table| table.text_count_entries())
        .unwrap_or(&[]);
    let field_summaries =
        summarize_tail_field_roles(text_count_entries, map.entries(), &[0, 29, 30]);
    let pair_summaries =
        summarize_tail_field_pair_roles(text_count_entries, map.entries(), &[0, 29, 30]);

    write_stdout_line(&format!(
        "summary\tposition-status={}\tentries={}\tdoc-bytes={}\tdoc-units={}",
        position_status,
        text_count_entries.len(),
        payload.bytes().len(),
        document_units
    ))?;

    for (field_index, field) in field_summaries.iter().enumerate() {
        write_stdout_line(&format!(
            "field\tf{}\tnonzero={}\tdistinct={}\tvalues={}\tunit-d0={}\ttext-d0={}\tunit-d29={}\ttext-d29={}\tunit-d30={}\ttext-d30={}",
            field_index,
            field.nonzero_count,
            field.distinct_values.len(),
            format_u16_value_counts(&field.value_counts),
            field.delta_hit_count(0, false),
            field.delta_hit_count(0, true),
            field.delta_hit_count(29, false),
            field.delta_hit_count(29, true),
            field.delta_hit_count(30, false),
            field.delta_hit_count(30, true)
        ))?;
    }

    for (field_index, pair) in pair_summaries.iter().enumerate() {
        write_stdout_line(&format!(
            "pair\tf{}-f{}\tpairs={}\tendpoints={}\tspan-eq={}\tspan-lt={}\tspan-gt={}\tbest-unit={}\tbest-text={}\td0={}\td29={}\td30={}",
            field_index,
            field_index + 1,
            pair.pair_count,
            pair.endpoints,
            pair.span_eq_count,
            pair.span_lt_count,
            pair.span_gt_count,
            format_best_unit_delta(pair.best.unit_delta, pair.best.unit_score),
            format_best_text_delta(pair.best.text_delta, pair.best.text_score),
            format_tail_delta_score(pair.delta_score(0)),
            format_tail_delta_score(pair.delta_score(29)),
            format_tail_delta_score(pair.delta_score(30))
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_range_preview(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-range-preview")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        write_stdout_line(&format!(
            "range-preview\t{}\t{}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tspan={}\tbyte-range={}\tunit-range={}",
            entry.index(),
            family,
            format_optional_u16_hex(tail_fields.first().copied()),
            format_optional_u16_hex(tail_fields.get(3).copied()),
            format_optional_u16_hex(tail_fields.get(4).copied()),
            format_optional_u16_hex(tail_fields.get(7).copied()),
            chosen_start,
            chosen_end,
            chosen_end.saturating_sub(chosen_start),
            format_byte_range_preview(map.entries(), chosen_start as usize, chosen_end as usize),
            format_unit_range_preview(map.entries(), chosen_start as usize, chosen_end as usize)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_range_boundaries(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-range-boundaries")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        write_stdout_line(&format!(
            "range-boundary\t{}\t{}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tspan={}\tbyte-boundary={}\tunit-boundary={}",
            entry.index(),
            family,
            format_optional_u16_hex(tail_fields.first().copied()),
            format_optional_u16_hex(tail_fields.get(3).copied()),
            format_optional_u16_hex(tail_fields.get(4).copied()),
            format_optional_u16_hex(tail_fields.get(7).copied()),
            chosen_start,
            chosen_end,
            chosen_end.saturating_sub(chosen_start),
            format_byte_range_boundaries(map.entries(), chosen_start as usize, chosen_end as usize),
            format_unit_range_boundaries(map.entries(), chosen_start as usize, chosen_end as usize)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_control_ranges(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-control-ranges")?;
    let filter = args
        .next()
        .map(|value| parse_u16_argument(&value))
        .transpose()?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }
    let map = map_document_text(payload.bytes());
    let ranges = build_control_delimited_ranges(map.entries(), filter);

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (chosen_start, chosen_end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        write_stdout_line(&format!(
            "count-control-range\t{}\t{}\tdelimiter={}\tt0={}\tt3={}\tt4={}\tt7={}\tstart={}\tend={}\tspan={}\tbyte-ranges={}\tunit-ranges={}",
            entry.index(),
            family,
            format_control_range_delimiter(filter),
            format_optional_u16_hex(tail_fields.first().copied()),
            format_optional_u16_hex(tail_fields.get(3).copied()),
            format_optional_u16_hex(tail_fields.get(4).copied()),
            format_optional_u16_hex(tail_fields.get(7).copied()),
            chosen_start,
            chosen_end,
            chosen_end.saturating_sub(chosen_start),
            format_control_range_hits(
                map.entries(),
                &ranges,
                chosen_start as usize,
                chosen_end as usize,
                RangeBasis::Byte,
            ),
            format_control_range_hits(
                map.entries(),
                &ranges,
                chosen_start as usize,
                chosen_end as usize,
                RangeBasis::Unit,
            )
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_count_layout_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-count-layout-context")?;
    let bytes = read_file(path)?;
    let table = read_document_text_position_tables(&bytes).map_err(|error| error.to_string())?;
    if table.text_count_entries().is_empty() {
        return Err("DocumentTextPositionTables missing TCntV.01 count entries".into());
    }

    let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
    let line_words = line_stream
        .as_deref()
        .map(|stream| be16_words(stream).collect::<Vec<_>>());
    let page_mark = read_page_mark(&bytes).ok();
    let page_bytes = read_cfb_stream(&bytes, "/PageMark")
        .ok()
        .map(|stream| stream.len());
    let paper_mark = read_paper_mark(&bytes).ok();
    let paper_bytes = read_cfb_stream(&bytes, "/PaperMark")
        .ok()
        .map(|stream| stream.len());

    write_stdout_line(&format!(
        "summary\tentries={}\tline-bytes={}\tline-words={}\tpage-rows={}\tpage-bytes={}\tpaper-rows={}\tpaper-bytes={}",
        table.text_count_entries().len(),
        format_optional_usize(line_stream.as_ref().map(|stream| stream.len())),
        format_optional_usize(line_words.as_ref().map(Vec::len)),
        format_optional_usize(page_mark.as_ref().map(|mark| mark.entries().len())),
        format_optional_usize(page_bytes),
        format_optional_usize(paper_mark.as_ref().map(|mark| mark.entries().len())),
        format_optional_usize(paper_bytes),
    ))?;

    for entry in table.text_count_entries() {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (start, end) = text_count_entry_chosen_range(raw, family);
        write_stdout_line(&format!(
            "entry\t{}\t{}\t{}\t{}\tline-word-start={}\tline-word-end={}\tline-byte-start={}\tline-byte-end={}\tpage-row-start={}\tpage-row-end={}\tpage-byte-start={}\tpage-byte-end={}\tpaper-row-start={}\tpaper-row-end={}\tpaper-byte-start={}\tpaper-byte-end={}",
            entry.index(),
            family,
            start,
            end,
            format_line_word_index_context(line_words.as_deref(), start as usize),
            format_line_word_index_context(line_words.as_deref(), end as usize),
            format_line_byte_offset_context(
                line_words.as_deref(),
                line_stream.as_ref().map(|stream| stream.len()),
                start as usize
            ),
            format_line_byte_offset_context(
                line_words.as_deref(),
                line_stream.as_ref().map(|stream| stream.len()),
                end as usize
            ),
            format_index_context(
                page_mark.as_ref().map(|mark| mark.entries().len()),
                start as usize
            ),
            format_index_context(
                page_mark.as_ref().map(|mark| mark.entries().len()),
                end as usize
            ),
            format_index_context(page_bytes, start as usize),
            format_index_context(page_bytes, end as usize),
            format_index_context(
                paper_mark.as_ref().map(|mark| mark.entries().len()),
                start as usize
            ),
            format_index_context(
                paper_mark.as_ref().map(|mark| mark.entries().len()),
                end as usize
            ),
            format_index_context(paper_bytes, start as usize),
            format_index_context(paper_bytes, end as usize),
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_style_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-style-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let (position_status, table) = match read_document_text_position_tables(&bytes) {
        Ok(table) => ("ok".to_string(), Some(table)),
        Err(Error::NotFound(_)) => ("missing".to_string(), None),
        Err(Error::InvalidData(message)) => (format!("invalid:{}", escaped_text(&message)), None),
        Err(error) => return Err(error.to_string()),
    };
    let text_count_entries = table
        .as_ref()
        .map(|table| table.text_count_entries())
        .unwrap_or(&[]);
    let style_streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
    let text_style_candidates =
        collect_labeled_style_candidates(&style_streams, TEXT_LAYOUT_STYLE_PATH);
    let page_style_candidates =
        collect_labeled_style_candidates(&style_streams, PAGE_LAYOUT_STYLE_PATH);
    let view_style_groups = collect_document_view_style_groups(&style_streams);
    let view_style_records = style_streams
        .iter()
        .find(|stream| stream.name() == DOCUMENT_VIEW_STYLES_PATH)
        .map(|stream| stream.summary().records().len())
        .unwrap_or_default();

    write_stdout_line(&format!(
        "summary\tposition-status={}\tentries={}\ttext-style-candidates={}\tpage-style-candidates={}\tview-style-records={}",
        position_status,
        text_count_entries.len(),
        text_style_candidates.len(),
        page_style_candidates.len(),
        view_style_records
    ))?;

    for entry in text_count_entries {
        let raw = entry.raw();
        let family = classify_text_count_entry_family(raw);
        let (start, end) = text_count_entry_chosen_range(raw, family);
        let tail_offset = text_count_entry_tail_offset(family);
        let tail_fields = read_be16_fields(&raw[tail_offset..]);
        write_stdout_line(&format!(
            "entry\t{}\t{}\tstart={}\tend={}\tspan={}\ttail-fields={}\ttext-style-id-hits={}\ttext-style-index-hits={}\tpage-style-id-hits={}\tpage-style-index-hits={}\tview-style-group-hits={}\tbyte-range={}",
            entry.index(),
            family,
            start,
            end,
            end.saturating_sub(start),
            format_indexed_u16_fields(&tail_fields),
            format_style_id_hits(&tail_fields, &text_style_candidates),
            format_style_index_hits(&tail_fields, &text_style_candidates),
            format_style_id_hits(&tail_fields, &page_style_candidates),
            format_style_index_hits(&tail_fields, &page_style_candidates),
            format_view_style_group_hits(&tail_fields, &view_style_groups),
            format_byte_range_preview(map.entries(), start as usize, end as usize)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_position_style_summary(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-position-style-summary")?;
    let bytes = read_file(path)?;
    let (position_status, table) = match read_document_text_position_tables(&bytes) {
        Ok(table) => ("ok".to_string(), Some(table)),
        Err(Error::NotFound(_)) => ("missing".to_string(), None),
        Err(Error::InvalidData(message)) => (format!("invalid:{}", escaped_text(&message)), None),
        Err(error) => return Err(error.to_string()),
    };
    let text_count_entries = table
        .as_ref()
        .map(|table| table.text_count_entries())
        .unwrap_or(&[]);
    let style_streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
    let text_style_candidates =
        collect_labeled_style_candidates(&style_streams, TEXT_LAYOUT_STYLE_PATH);
    let page_style_candidates =
        collect_labeled_style_candidates(&style_streams, PAGE_LAYOUT_STYLE_PATH);
    let view_style_groups = collect_document_view_style_groups(&style_streams);
    let view_style_records = style_streams
        .iter()
        .find(|stream| stream.name() == DOCUMENT_VIEW_STYLES_PATH)
        .map(|stream| stream.summary().records().len())
        .unwrap_or_default();
    let field_summaries = summarize_text_position_style_fields(
        text_count_entries,
        &text_style_candidates,
        &page_style_candidates,
        &view_style_groups,
    );

    write_stdout_line(&format!(
        "summary\tposition-status={}\tentries={}\ttext-style-candidates={}\tpage-style-candidates={}\tview-style-records={}",
        position_status,
        text_count_entries.len(),
        text_style_candidates.len(),
        page_style_candidates.len(),
        view_style_records
    ))?;

    for (field_index, field) in field_summaries.iter().enumerate() {
        write_stdout_line(&format!(
            "field\tf{}\tnonzero={}\tdistinct={}\tvalues={}\ttext-style-id-hits={}\ttext-style-index-hits={}\tpage-style-id-hits={}\tpage-style-index-hits={}\tview-style-group-hits={}",
            field_index,
            field.nonzero_count,
            field.distinct_values.len(),
            format_u16_value_counts(&field.value_counts),
            format_candidate_id_hit_counts(&field.text_style_id_hits, &text_style_candidates),
            format_candidate_index_hit_counts(&field.text_style_index_hits, &text_style_candidates),
            format_candidate_id_hit_counts(&field.page_style_id_hits, &page_style_candidates),
            format_candidate_index_hit_counts(&field.page_style_index_hits, &page_style_candidates),
            format_view_style_group_hit_counts(&field.view_style_group_hits, &view_style_groups)
        ))?;
    }
    Ok(())
}
