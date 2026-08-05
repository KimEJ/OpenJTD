use rjtd_core::container::read_cfb_stream;
use rjtd_core::document_text::{map_document_text, read_document_text_payload};
use rjtd_core::layout_mark::{read_page_mark, read_paper_mark};
use rjtd_core::style_stream::{
    DOCUMENT_VIEW_STYLES_PATH, PAGE_LAYOUT_STYLE_PATH, TEXT_LAYOUT_STYLE_PATH, read_style_streams,
};
use rjtd_model::parse_document;

use crate::input::read_file;

use super::line_mark_support::{
    format_index_context, format_line_byte_offset_context, format_line_word_index_context,
};
use super::style_support::*;
use super::support::*;
use super::text_boundary_support::*;
use super::text_position_count_support::*;

pub(crate) fn run_text_boundary_candidates(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-candidates")?;
    let bytes = read_file(path)?;
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;

    for candidate in document.text_boundary_candidates() {
        write_stdout_line(&format!(
            "text-boundary-candidate\t{}\tkind={}\trange={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tinterval-kind={}\tfirst={}\tlast={}\tsource={}-{}\tdecoded=false",
            candidate.index(),
            candidate.kind(),
            candidate.text_count_range_index(),
            candidate.basis().as_str(),
            candidate.delimiter_code(),
            candidate.interval_count(),
            format_boundary_candidate_interval_kind(candidate.interval_count()),
            candidate.first_interval_index(),
            candidate.last_interval_index(),
            candidate.source_start(),
            candidate.source_end()
        ))?;
    }
    Ok(())
}

pub(crate) fn run_table_candidates(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let path = required_path(args.next(), "table-candidates")?;
    let bytes = read_file(path)?;
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;

    for candidate in document.table_candidates() {
        write_stdout_line(&format!(
            "table-candidate\t{}\tkind={}\trange={}\tboundary={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tfirst={}\tlast={}\tsource={}-{}\tsparse={}\tcells={}/{}/{}\tmax-columns={}\tinterval-details={}\tdecoded=false",
            candidate.index(),
            candidate.kind(),
            format_model_table_source_index(candidate.text_count_range_index()),
            format_model_table_source_index(candidate.text_boundary_candidate_index()),
            candidate.basis().as_str(),
            candidate.delimiter_code(),
            candidate.interval_count(),
            candidate.first_interval_index(),
            candidate.last_interval_index(),
            candidate.source_start(),
            candidate.source_end(),
            candidate.is_sparse_document_text_control_run_candidate(),
            candidate.non_empty_cell_count_candidate(),
            candidate.empty_cell_count_candidate(),
            candidate.cell_count_candidate(),
            candidate.max_column_segment_count(),
            format_table_candidate_intervals(candidate)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_table_candidate_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "table-candidate-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;

    for candidate in document.table_candidates() {
        let basis = range_basis_from_candidate(candidate.basis().as_str());
        write_stdout_line(&format!(
            "table-candidate-context\t{}\trange={}\tboundary={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tsource={}-{}\tshape={}\tinterval-contexts={}\tdecoded=false",
            candidate.index(),
            format_model_table_source_index(candidate.text_count_range_index()),
            format_model_table_source_index(candidate.text_boundary_candidate_index()),
            candidate.basis().as_str(),
            candidate.delimiter_code(),
            candidate.interval_count(),
            candidate.source_start(),
            candidate.source_end(),
            format_table_candidate_text_shape(candidate, map.entries(), basis),
            format_table_candidate_interval_contexts(candidate, map.entries(), basis)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_table_cell_like_candidates(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "table-cell-like-candidates")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;

    for candidate in document.table_candidates() {
        let basis = range_basis_from_candidate(candidate.basis().as_str());
        if !is_table_candidate_cell_like(candidate, map.entries(), basis) {
            continue;
        }
        write_stdout_line(&format!(
            "table-cell-like-candidate\t{}\trange={}\tboundary={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tsource={}-{}\tshape={}\ttexts={}\tcolumn-split-candidate-rows={}\tmax-column-segment-count={}\tcolumn-segment-pattern-consistent={}\tcolumn-segment-pattern-mismatch-rows={}\tcolumn-grid-candidate={}\tcolumn-grid-shape={}\tcolumn-grid-pattern={}\tinterval-column-segments={}\tdecoded=false",
            candidate.index(),
            format_model_table_source_index(candidate.text_count_range_index()),
            format_model_table_source_index(candidate.text_boundary_candidate_index()),
            candidate.basis().as_str(),
            candidate.delimiter_code(),
            candidate.interval_count(),
            candidate.source_start(),
            candidate.source_end(),
            format_table_candidate_text_shape(candidate, map.entries(), basis),
            format_table_candidate_interval_texts(candidate, map.entries(), basis),
            candidate.column_split_candidate_row_count(),
            candidate.max_column_segment_count(),
            candidate.column_segment_pattern_consistent(),
            candidate.column_segment_pattern_mismatch_rows(),
            if candidate.column_segment_grid_candidate().is_some() {
                "true"
            } else {
                "false"
            },
            format_table_candidate_column_grid_shape(candidate),
            format_table_candidate_column_grid_pattern(candidate),
            format_table_candidate_interval_column_segments(candidate)
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_boundary_candidate_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-candidate-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;

    for candidate in document.text_boundary_candidates() {
        let basis = range_basis_from_candidate(candidate.basis().as_str());
        write_stdout_line(&format!(
            "text-boundary-candidate-context\t{}\trange={}\tbasis={}\tdelimiter=0x{:04x}\tintervals={}\tinterval-kind={}\tsource={}-{}\tline-breaks={}\ttext={}\tedges={}\tdecoded=false",
            candidate.index(),
            candidate.text_count_range_index(),
            candidate.basis().as_str(),
            candidate.delimiter_code(),
            candidate.interval_count(),
            format_boundary_candidate_interval_kind(candidate.interval_count()),
            candidate.source_start(),
            candidate.source_end(),
            range_line_break_count(
                map.entries(),
                candidate.source_start(),
                candidate.source_end(),
                basis
            ),
            format_candidate_range_preview(
                map.entries(),
                candidate.source_start(),
                candidate.source_end(),
                basis
            ),
            format_candidate_range_boundaries(
                map.entries(),
                candidate.source_start(),
                candidate.source_end(),
                basis
            )
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_boundary_candidate_agreement(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-candidate-agreement")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let mut pair_index = 0usize;

    for byte_candidate in document
        .text_boundary_candidates()
        .iter()
        .filter(|candidate| candidate.basis().as_str() == "byte")
    {
        let Some(unit_candidate) = document
            .text_boundary_candidates()
            .iter()
            .find(|candidate| {
                candidate.basis().as_str() == "unit"
                    && candidate.text_count_range_index() == byte_candidate.text_count_range_index()
                    && candidate.delimiter_code() == byte_candidate.delimiter_code()
            })
        else {
            continue;
        };
        let byte_text = range_visible_text(
            map.entries(),
            byte_candidate.source_start(),
            byte_candidate.source_end(),
            RangeBasis::Byte,
        );
        let unit_text = range_visible_text(
            map.entries(),
            unit_candidate.source_start(),
            unit_candidate.source_end(),
            RangeBasis::Unit,
        );
        let byte_line_breaks = text_line_break_count(&byte_text);
        let unit_line_breaks = text_line_break_count(&unit_text);

        write_stdout_line(&format!(
            "text-boundary-candidate-agreement\t{}\trange={}\tdelimiter=0x{:04x}\tbyte-index={}\tunit-index={}\tbyte-intervals={}\tunit-intervals={}\tbyte-interval-kind={}\tunit-interval-kind={}\tbyte-edge-good={}\tunit-edge-good={}\tbyte-line-breaks={}\tunit-line-breaks={}\ttext-match={}\tline-break-match={}\tbyte-text={}\tunit-text={}\tdecoded=false",
            pair_index,
            byte_candidate.text_count_range_index(),
            byte_candidate.delimiter_code(),
            byte_candidate.index(),
            unit_candidate.index(),
            byte_candidate.interval_count(),
            unit_candidate.interval_count(),
            format_boundary_candidate_interval_kind(byte_candidate.interval_count()),
            format_boundary_candidate_interval_kind(unit_candidate.interval_count()),
            is_boundary_candidate_edge_good(
                map.entries(),
                byte_candidate.source_start(),
                byte_candidate.source_end(),
                RangeBasis::Byte
            ),
            is_boundary_candidate_edge_good(
                map.entries(),
                unit_candidate.source_start(),
                unit_candidate.source_end(),
                RangeBasis::Unit
            ),
            byte_line_breaks,
            unit_line_breaks,
            byte_text == unit_text,
            byte_line_breaks == unit_line_breaks,
            escaped_text_preview(&byte_text, 80),
            escaped_text_preview(&unit_text, 80)
        ))?;
        pair_index += 1;
    }
    Ok(())
}

pub(crate) fn run_text_boundary_candidate_layout_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-candidate-layout-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
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

    let candidates = document
        .text_boundary_candidates()
        .iter()
        .filter(|candidate| {
            candidate.basis().as_str() == "unit"
                && candidate.delimiter_code() == 0x001c
                && candidate.interval_count() == 1
        })
        .collect::<Vec<_>>();
    let selected_count = candidates
        .iter()
        .filter(|candidate| {
            is_strict_unit_paragraph_candidate(
                map.entries(),
                candidate.source_start(),
                candidate.source_end(),
            )
        })
        .count();

    write_stdout_line(&format!(
        "summary\tunit-001c-single-candidates={}\trule-selected={}\tline-bytes={}\tline-words={}\tpage-rows={}\tpage-bytes={}\tpaper-rows={}\tpaper-bytes={}",
        candidates.len(),
        selected_count,
        format_optional_usize(line_stream.as_ref().map(|stream| stream.len())),
        format_optional_usize(line_words.as_ref().map(Vec::len)),
        format_optional_usize(page_mark.as_ref().map(|mark| mark.entries().len())),
        format_optional_usize(page_bytes),
        format_optional_usize(paper_mark.as_ref().map(|mark| mark.entries().len())),
        format_optional_usize(paper_bytes),
    ))?;

    for candidate in candidates {
        let text = range_visible_text(
            map.entries(),
            candidate.source_start(),
            candidate.source_end(),
            RangeBasis::Unit,
        );
        let line_breaks = text_line_break_count(&text);
        let edge_good = is_boundary_candidate_edge_good(
            map.entries(),
            candidate.source_start(),
            candidate.source_end(),
            RangeBasis::Unit,
        );
        let non_empty = !text.is_empty();
        let selected = edge_good && non_empty && line_breaks <= 1;
        write_stdout_line(&format!(
            "candidate\t{}\trange={}\tselected={}\tedge-good={}\tnon-empty={}\tline-breaks={}\tsource={}-{}\ttext={}\tline-word-start={}\tline-word-end={}\tline-byte-start={}\tline-byte-end={}\tpage-row-start={}\tpage-row-end={}\tpage-byte-start={}\tpage-byte-end={}\tpaper-row-start={}\tpaper-row-end={}\tpaper-byte-start={}\tpaper-byte-end={}\tdecoded=false",
            candidate.index(),
            candidate.text_count_range_index(),
            selected,
            edge_good,
            non_empty,
            line_breaks,
            candidate.source_start(),
            candidate.source_end(),
            escaped_text_preview(&text, 80),
            format_line_word_index_context(line_words.as_deref(), candidate.source_start()),
            format_line_word_index_context(line_words.as_deref(), candidate.source_end()),
            format_line_byte_offset_context(
                line_words.as_deref(),
                line_stream.as_ref().map(|stream| stream.len()),
                candidate.source_start()
            ),
            format_line_byte_offset_context(
                line_words.as_deref(),
                line_stream.as_ref().map(|stream| stream.len()),
                candidate.source_end()
            ),
            format_index_context(
                page_mark.as_ref().map(|mark| mark.entries().len()),
                candidate.source_start()
            ),
            format_index_context(
                page_mark.as_ref().map(|mark| mark.entries().len()),
                candidate.source_end()
            ),
            format_index_context(page_bytes, candidate.source_start()),
            format_index_context(page_bytes, candidate.source_end()),
            format_index_context(
                paper_mark.as_ref().map(|mark| mark.entries().len()),
                candidate.source_start()
            ),
            format_index_context(
                paper_mark.as_ref().map(|mark| mark.entries().len()),
                candidate.source_end()
            ),
            format_index_context(paper_bytes, candidate.source_start()),
            format_index_context(paper_bytes, candidate.source_end()),
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_boundary_layout_map(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-layout-map")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
    let line_words = line_stream
        .as_deref()
        .map(|stream| be16_words(stream).collect::<Vec<_>>());
    let page_mark = read_page_mark(&bytes).ok();
    let paper_mark = read_paper_mark(&bytes).ok();

    let candidates = collect_unit_001c_single_layout_candidates(
        map.entries(),
        document.text_boundary_candidates(),
    );
    let selected_count = candidates
        .iter()
        .filter(|candidate| candidate.selected)
        .count();
    let target_sets = layout_map_target_sets(
        line_words.as_deref(),
        page_mark.as_ref(),
        paper_mark.as_ref(),
    );
    let target_set_count = target_sets.len();
    let base_count = layout_map_bases().len();

    write_stdout_line(&format!(
        "summary\tunit-001c-single-candidates={}\trule-selected={}\ttarget-sets={}\tbases={}\tdelta-range={}..{}",
        candidates.len(),
        selected_count,
        target_set_count,
        base_count,
        LAYOUT_MAP_DELTA_MIN,
        LAYOUT_MAP_DELTA_MAX
    ))?;

    write_layout_map_best_rows("all", &candidates, &target_sets)?;
    let selected = candidates
        .iter()
        .copied()
        .filter(|candidate| candidate.selected)
        .collect::<Vec<_>>();
    write_layout_map_best_rows("selected", &selected, &target_sets)?;
    Ok(())
}

pub(crate) fn run_text_boundary_layout_map_rows(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-layout-map-rows")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
    let line_words = line_stream
        .as_deref()
        .map(|stream| be16_words(stream).collect::<Vec<_>>());
    let page_mark = read_page_mark(&bytes).ok();
    let paper_mark = read_paper_mark(&bytes).ok();

    let candidates = collect_unit_001c_single_layout_candidates(
        map.entries(),
        document.text_boundary_candidates(),
    );
    let selected_count = candidates
        .iter()
        .filter(|candidate| candidate.selected)
        .count();
    let target_sets = layout_map_target_sets(
        line_words.as_deref(),
        page_mark.as_ref(),
        paper_mark.as_ref(),
    );
    let base_count = layout_map_bases().len();
    write_stdout_line(&format!(
        "summary\tunit-001c-single-candidates={}\trule-selected={}\ttarget-sets={}\tbases={}\tlocal-rows={}",
        candidates.len(),
        selected_count,
        target_sets.len(),
        base_count,
        candidates.len() * target_sets.len() * base_count
    ))?;

    for candidate in &candidates {
        let text = range_visible_text(
            map.entries(),
            candidate.source_start,
            candidate.source_end,
            RangeBasis::Unit,
        );
        let range = document
            .text_count_ranges()
            .get(candidate.text_count_range_index);
        for target_set in &target_sets {
            for base in layout_map_bases() {
                let single = [*candidate];
                let (delta, score) = best_layout_map_delta(&single, target_set, *base);
                write_stdout_line(&format!(
                    "local\tcandidate={}\trange={}\tselected={}\ttarget={}\tbase={}\tdelta={}\tdelta-at-boundary={}\texact={}\ttotal-distance={}\tmax-distance={}\tstart-nearest={}\tend-nearest={}\tsource={}-{}\ttext={}\ttcnt={}\tdecoded=false",
                    candidate.index,
                    candidate.text_count_range_index,
                    candidate.selected,
                    target_set.name,
                    base.name(),
                    delta,
                    delta == LAYOUT_MAP_DELTA_MIN || delta == LAYOUT_MAP_DELTA_MAX,
                    score.exact_hits,
                    format_optional_usize(score.total_distance),
                    format_optional_usize(score.max_distance),
                    format_layout_map_endpoint(candidate.source_start, target_set, *base, delta),
                    format_layout_map_endpoint(candidate.source_end, target_set, *base, delta),
                    candidate.source_start,
                    candidate.source_end,
                    escaped_text_preview(&text, 80),
                    format_text_count_range_summary(range),
                ))?;
            }
        }
    }
    Ok(())
}

pub(crate) fn run_text_boundary_paragraph_like(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-paragraph-like")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
    let line_words = line_stream
        .as_deref()
        .map(|stream| be16_words(stream).collect::<Vec<_>>());
    let page_mark = read_page_mark(&bytes).ok();
    let paper_mark = read_paper_mark(&bytes).ok();
    let target_sets = layout_map_target_sets(
        line_words.as_deref(),
        page_mark.as_ref(),
        paper_mark.as_ref(),
    );
    let candidates = collect_unit_001c_single_layout_candidates(
        map.entries(),
        document.text_boundary_candidates(),
    );

    let mut rows = Vec::new();
    for candidate in &candidates {
        let evidence = layout_paragraph_like_evidence(candidate, &target_sets);
        rows.push((
            *candidate,
            evidence.paragraph_like,
            evidence.line_word_evidence,
            evidence.page_field_evidence,
        ));
    }
    let strict_selected = rows
        .iter()
        .filter(|(candidate, _, _, _)| candidate.selected)
        .count();
    let paragraph_like_count = rows
        .iter()
        .filter(|(_, paragraph_like, _, _)| *paragraph_like)
        .count();
    let selected_non_paragraph_like = strict_selected.saturating_sub(paragraph_like_count);
    write_stdout_line(&format!(
        "summary\tunit-001c-single-candidates={}\tstrict-selected={}\tparagraph-like={}\tselected-non-paragraph-like={}\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false",
        candidates.len(),
        strict_selected,
        paragraph_like_count,
        selected_non_paragraph_like
    ))?;

    for (candidate, paragraph_like, line_word_evidence, page_field_evidence) in rows {
        let text = range_visible_text(
            map.entries(),
            candidate.source_start,
            candidate.source_end,
            RangeBasis::Unit,
        );
        let range = document
            .text_count_ranges()
            .get(candidate.text_count_range_index);
        write_stdout_line(&format!(
            "candidate\t{}\trange={}\tstrict-selected={}\tparagraph-like={}\tline-word-evidence={}\tpage-field-evidence={}\tsource={}-{}\ttext={}\ttcnt={}\tdecoded=false",
            candidate.index,
            candidate.text_count_range_index,
            candidate.selected,
            paragraph_like,
            format_layout_exact_evidence(line_word_evidence.as_ref()),
            format_layout_exact_evidence(page_field_evidence.as_ref()),
            candidate.source_start,
            candidate.source_end,
            escaped_text_preview(&text, 80),
            format_text_count_range_summary(range),
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_boundary_paragraph_like_style_context(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-paragraph-like-style-context")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
    let line_words = line_stream
        .as_deref()
        .map(|stream| be16_words(stream).collect::<Vec<_>>());
    let page_mark = read_page_mark(&bytes).ok();
    let paper_mark = read_paper_mark(&bytes).ok();
    let target_sets = layout_map_target_sets(
        line_words.as_deref(),
        page_mark.as_ref(),
        paper_mark.as_ref(),
    );
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
    let candidates = collect_unit_001c_single_layout_candidates(
        map.entries(),
        document.text_boundary_candidates(),
    );
    let rows = candidates
        .iter()
        .map(|candidate| {
            (
                *candidate,
                layout_paragraph_like_evidence(candidate, &target_sets),
            )
        })
        .collect::<Vec<_>>();
    let strict_selected = rows
        .iter()
        .filter(|(candidate, _)| candidate.selected)
        .count();
    let paragraph_like_count = rows
        .iter()
        .filter(|(_, evidence)| evidence.paragraph_like)
        .count();
    let selected_non_paragraph_like = strict_selected.saturating_sub(paragraph_like_count);

    write_stdout_line(&format!(
        "summary\tunit-001c-single-candidates={}\tstrict-selected={}\tparagraph-like={}\tselected-non-paragraph-like={}\ttext-style-candidates={}\tpage-style-candidates={}\tview-style-records={}\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false",
        candidates.len(),
        strict_selected,
        paragraph_like_count,
        selected_non_paragraph_like,
        text_style_candidates.len(),
        page_style_candidates.len(),
        view_style_records
    ))?;

    for (candidate, evidence) in rows {
        let text = range_visible_text(
            map.entries(),
            candidate.source_start,
            candidate.source_end,
            RangeBasis::Unit,
        );
        let range = document
            .text_count_ranges()
            .get(candidate.text_count_range_index);
        let tail_fields = range.map(|range| range.tail_fields()).unwrap_or(&[]);
        let byte_range = range
            .map(|range| {
                format_byte_range_preview(
                    map.entries(),
                    range.start() as usize,
                    range.end() as usize,
                )
            })
            .unwrap_or_else(|| "-".to_string());
        let unit_range = range
            .map(|range| {
                format_unit_range_preview(
                    map.entries(),
                    range.start() as usize,
                    range.end() as usize,
                )
            })
            .unwrap_or_else(|| "-".to_string());
        write_stdout_line(&format!(
            "candidate\t{}\trange={}\tstrict-selected={}\tparagraph-like={}\tline-word-evidence={}\tpage-field-evidence={}\ttail-fields={}\ttext-style-id-hits={}\ttext-style-index-hits={}\tpage-style-id-hits={}\tpage-style-index-hits={}\tview-style-group-hits={}\tbyte-range={}\tunit-range={}\tsource={}-{}\ttext={}\ttcnt={}\tdecoded=false",
            candidate.index,
            candidate.text_count_range_index,
            candidate.selected,
            evidence.paragraph_like,
            format_layout_exact_evidence(evidence.line_word_evidence.as_ref()),
            format_layout_exact_evidence(evidence.page_field_evidence.as_ref()),
            format_indexed_u16_fields(tail_fields),
            format_style_id_hits(tail_fields, &text_style_candidates),
            format_style_index_hits(tail_fields, &text_style_candidates),
            format_style_id_hits(tail_fields, &page_style_candidates),
            format_style_index_hits(tail_fields, &page_style_candidates),
            format_view_style_group_hits(tail_fields, &view_style_groups),
            byte_range,
            unit_range,
            candidate.source_start,
            candidate.source_end,
            escaped_text_preview(&text, 80),
            format_text_count_range_summary(range),
        ))?;
    }
    Ok(())
}

pub(crate) fn run_text_boundary_paragraph_like_discriminators(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-boundary-paragraph-like-discriminators")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
    let line_words = line_stream
        .as_deref()
        .map(|stream| be16_words(stream).collect::<Vec<_>>());
    let page_mark = read_page_mark(&bytes).ok();
    let paper_mark = read_paper_mark(&bytes).ok();
    let target_sets = layout_map_target_sets(
        line_words.as_deref(),
        page_mark.as_ref(),
        paper_mark.as_ref(),
    );
    let style_streams = read_style_streams(&bytes).map_err(|error| error.to_string())?;
    let text_style_candidates =
        collect_labeled_style_candidates(&style_streams, TEXT_LAYOUT_STYLE_PATH);
    let page_style_candidates =
        collect_labeled_style_candidates(&style_streams, PAGE_LAYOUT_STYLE_PATH);
    let view_style_groups = collect_document_view_style_groups(&style_streams);
    let candidates = collect_unit_001c_single_layout_candidates(
        map.entries(),
        document.text_boundary_candidates(),
    );

    let mut paragraph_like = ParagraphLikeBucketSummary::default();
    let mut strict_non_paragraph = ParagraphLikeBucketSummary::default();
    let mut non_strict = ParagraphLikeBucketSummary::default();
    for candidate in &candidates {
        let evidence = layout_paragraph_like_evidence(candidate, &target_sets);
        let range = document
            .text_count_ranges()
            .get(candidate.text_count_range_index);
        let bucket = if evidence.paragraph_like {
            &mut paragraph_like
        } else if candidate.selected {
            &mut strict_non_paragraph
        } else {
            &mut non_strict
        };
        bucket.observe(
            candidate,
            &evidence,
            range,
            &text_style_candidates,
            &page_style_candidates,
            &view_style_groups,
        );
    }

    write_stdout_line(&format!(
        "summary\tunit-001c-single-candidates={}\tstrict-selected={}\tparagraph-like={}\tselected-non-paragraph-like={}\trule=strict-unit-001c-single+line-word-value-exact2+page-be32-field-exact2\tdecoded=false",
        candidates.len(),
        paragraph_like.strict_selected + strict_non_paragraph.strict_selected,
        paragraph_like.rows,
        strict_non_paragraph.rows
    ))?;
    write_stdout_line(&format!(
        "bucket\tparagraph-like\t{}",
        paragraph_like.format_fields()
    ))?;
    write_stdout_line(&format!(
        "bucket\tstrict-non-paragraph\t{}",
        strict_non_paragraph.format_fields()
    ))?;
    write_stdout_line(&format!(
        "bucket\tnon-strict\t{}",
        non_strict.format_fields()
    ))?;
    Ok(())
}

pub(crate) fn run_text_paragraph_boundary_targets(
    mut args: impl Iterator<Item = String>,
) -> Result<(), String> {
    let path = required_path(args.next(), "text-paragraph-boundary-targets")?;
    let bytes = read_file(path)?;
    let payload = read_document_text_payload(&bytes).map_err(|error| error.to_string())?;
    let map = map_document_text(payload.bytes());
    let document = parse_document(&bytes).map_err(|error| error.to_string())?;
    let line_stream = read_cfb_stream(&bytes, "/LineMark").ok();
    let line_words = line_stream
        .as_deref()
        .map(|stream| be16_words(stream).collect::<Vec<_>>());
    let page_mark = read_page_mark(&bytes).ok();

    write_stdout_line(&format!(
        "summary\ttext-paragraph-boundary-candidates={}\tline-words={}\tpage-rows={}\trule=strict-unit-001c-single+nonzero-tcnt-span+line-word-value-exact2+page-be32-field-exact2\tdecoded=false",
        document.text_paragraph_boundary_candidates().len(),
        format_optional_usize(line_words.as_ref().map(Vec::len)),
        format_optional_usize(page_mark.as_ref().map(|mark| mark.entries().len())),
    ))?;

    for candidate in document.text_paragraph_boundary_candidates() {
        let text = range_visible_text(
            map.entries(),
            candidate.source_start(),
            candidate.source_end(),
            RangeBasis::Unit,
        );
        let line_start =
            layout_evidence_value(candidate.source_start(), candidate.line_word_evidence());
        let line_end =
            layout_evidence_value(candidate.source_end(), candidate.line_word_evidence());
        let page_start =
            layout_evidence_value(candidate.source_start(), candidate.page_field_evidence());
        let page_end =
            layout_evidence_value(candidate.source_end(), candidate.page_field_evidence());
        let range = document
            .text_count_ranges()
            .get(candidate.text_count_range_index());
        write_stdout_line(&format!(
            "text-paragraph-boundary-target\t{}\tboundary={}\trange={}\tsource={}-{}\tspan={}\tline-word-evidence={}\tline-start={}\tline-end={}\tpage-field-evidence={}\tpage-start={}\tpage-end={}\ttext={}\ttcnt={}\tdecoded=false",
            candidate.index(),
            candidate.text_boundary_candidate_index(),
            candidate.text_count_range_index(),
            candidate.source_start(),
            candidate.source_end(),
            candidate.text_count_range_span(),
            format_model_layout_exact_evidence(candidate.line_word_evidence()),
            format_line_word_value_refs(line_words.as_deref(), line_start),
            format_line_word_value_refs(line_words.as_deref(), line_end),
            format_model_layout_exact_evidence(candidate.page_field_evidence()),
            format_page_be32_field_value_refs(page_mark.as_ref(), page_start),
            format_page_be32_field_value_refs(page_mark.as_ref(), page_end),
            escaped_text_preview(&text, 80),
            format_text_count_range_summary(range),
        ))?;
    }
    Ok(())
}
