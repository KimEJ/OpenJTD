#![doc = include_str!("../README.md")]

mod input;
mod probe_compare;
mod probe_corpus;
mod probe_format;
mod probe_line_diff;
mod probe_manifest;
mod probe_page_diff;
mod probe_signals;
mod probe_validation;

#[path = "main/mod.rs"]
mod cli;

pub(crate) const BROKEN_PIPE_EXIT: &str = "__rjtd_broken_pipe__";

fn main() {
    let code = match run(std::env::args().skip(1)) {
        Ok(()) => 0,
        Err(message) if message == BROKEN_PIPE_EXIT => 0,
        Err(message) => {
            eprintln!("error: {message}");
            2
        }
    };

    std::process::exit(code);
}

fn run(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let mut args = args.into_iter();

    match args.next().as_deref() {
        None | Some("-h") | Some("--help") => cli::help::print_help(),
        Some("streams") => cli::container::run_streams(args),
        Some("info") => cli::container::run_info(args),
        Some("dump-stream") => cli::container::run_dump_stream(args),
        Some("style-records") => cli::style::run_style_records(args),
        Some("page-layout-style-slots") => cli::style::run_page_layout_style_slots(args),
        Some("style-candidates") => cli::style::run_style_candidates(args),
        Some("text-layout-style-records") => cli::style::run_text_layout_style_records(args),
        Some("document-view-style-groups") => cli::style::run_document_view_style_groups(args),
        Some("paragraph-style-records") => cli::style::run_paragraph_style_records(args),
        Some("cfb-map") => cli::container::run_cfb_map(args),
        Some("cfb-dir") => cli::container::run_cfb_dir(args),
        Some("stream-meta") => cli::container::run_stream_meta(args),
        Some("stream-chain") => cli::container::run_stream_chain(args),
        Some("stream-words") => cli::stream_scan::run_stream_words(args),
        Some("stream-word-frequencies") => cli::stream_scan::run_stream_word_frequencies(args),
        Some("line-mark-tags") => cli::line_mark::run_line_mark_tags(args),
        Some("line-mark-intervals") => cli::line_mark::run_line_mark_intervals(args),
        Some("source-y-probe-audit") => cli::line_mark::run_source_y_probe_audit(args),
        Some("source-y-probe-compare") => cli::line_mark::run_source_y_probe_compare(args),
        Some("line-mark-text-context") => cli::line_mark::run_line_mark_text_context(args),
        Some("stream-dwords") => cli::stream_scan::run_stream_dwords(args),
        Some("stream-dword-frequencies") => cli::stream_scan::run_stream_dword_frequencies(args),
        Some("stream-text-probe") => cli::stream_scan::run_stream_text_probe(args),
        Some("stream-find") => cli::stream_scan::run_stream_find(args),
        Some("stream-find-bytes") => cli::stream_scan::run_stream_find_bytes(args),
        Some("so-records") => cli::object_stream::run_so_records(args),
        Some("object-stream-candidates") => cli::object_stream::run_object_stream_candidates(args),
        Some("object-ownership-references") => {
            cli::object_stream::run_object_ownership_references(args)
        }
        Some("object-ownership-reference-fields") => {
            cli::object_stream::run_object_ownership_reference_fields(args)
        }
        Some("object-frame-reference-records") => {
            cli::object_stream::run_object_frame_reference_records(args)
        }
        Some("object-frame-record-families") => {
            cli::object_stream::run_object_frame_record_families(args)
        }
        Some("object-frame-row-links") => cli::object_stream::run_object_frame_row_links(args),
        Some("object-image-frame-candidates") => {
            cli::object_fdm::run_object_image_frame_candidates(args)
        }
        Some("object-fdm-image-candidates") => {
            cli::object_fdm::run_object_fdm_image_candidates(args)
        }
        Some("object-fdm-frame-links") => cli::object_fdm::run_object_fdm_frame_links(args),
        Some("object-fdm-index") => cli::object_fdm::run_object_fdm_index(args),
        Some("object-fdm-index-shape") => cli::object_fdm::run_object_fdm_index_shape(args),
        Some("object-fdm-index-rows") => cli::object_fdm::run_object_fdm_index_rows(args),
        Some("so-record-clusters") => cli::object_stream::run_so_record_clusters(args),
        Some("so-record-fields") => cli::object_stream::run_so_record_fields(args),
        Some("so-record-geometry") => cli::object_stream::run_so_record_geometry(args),
        Some("so-record-halves") => cli::object_stream::run_so_record_halves(args),
        Some("cat") => cli::text_token::run_cat(args),
        Some("text-tokens") => cli::text_token::run_text_tokens(args),
        Some("text-control-context") => cli::text_token::run_text_control_context(args),
        Some("text-control-clusters") => cli::text_token::run_text_control_clusters(args),
        Some("text-control-ranges") => cli::text_token::run_text_control_ranges(args),
        Some("text-positions") => cli::text_position::run_text_positions(args),
        Some("text-position-mark-header") => {
            cli::text_position::run_text_position_mark_header(args)
        }
        Some("text-position-mark-summary") => {
            cli::text_position::run_text_position_mark_summary(args)
        }
        Some("text-position-counts") => cli::text_position_count::run_text_position_counts(args),
        Some("text-position-count-context") => {
            cli::text_position_count::run_text_position_count_context(args)
        }
        Some("text-position-count-tail-context") => {
            cli::text_position_count::run_text_position_count_tail_context(args)
        }
        Some("text-position-count-clusters") => {
            cli::text_position_count::run_text_position_count_clusters(args)
        }
        Some("text-position-count-candidates") => {
            cli::text_position_count::run_text_position_count_candidates(args)
        }
        Some("text-position-count-family") => {
            cli::text_position_count::run_text_position_count_family(args)
        }
        Some("text-position-count-fields") => {
            cli::text_position_count::run_text_position_count_fields(args)
        }
        Some("text-position-count-field-deltas") => {
            cli::text_position_count::run_text_position_count_field_deltas(args)
        }
        Some("text-position-count-tail-delta-scan") => {
            cli::text_position_count::run_text_position_count_tail_delta_scan(args)
        }
        Some("text-position-count-tail-delta-groups") => {
            cli::text_position_count::run_text_position_count_tail_delta_groups(args)
        }
        Some("text-position-count-tail-row-deltas") => {
            cli::text_position_count::run_text_position_count_tail_row_deltas(args)
        }
        Some("text-position-count-tail-row-context") => {
            cli::text_position_count::run_text_position_count_tail_row_context(args)
        }
        Some("text-position-count-tail-field-roles") => {
            cli::text_position_count::run_text_position_count_tail_field_roles(args)
        }
        Some("text-position-count-range-preview") => {
            cli::text_position_count::run_text_position_count_range_preview(args)
        }
        Some("text-position-count-range-boundaries") => {
            cli::text_position_count::run_text_position_count_range_boundaries(args)
        }
        Some("text-position-count-control-ranges") => {
            cli::text_position_count::run_text_position_count_control_ranges(args)
        }
        Some("text-boundary-candidates") => cli::text_boundary::run_text_boundary_candidates(args),
        Some("table-candidates") => cli::text_boundary::run_table_candidates(args),
        Some("table-candidate-context") => cli::text_boundary::run_table_candidate_context(args),
        Some("table-cell-like-candidates") => {
            cli::text_boundary::run_table_cell_like_candidates(args)
        }
        Some("text-boundary-candidate-context") => {
            cli::text_boundary::run_text_boundary_candidate_context(args)
        }
        Some("text-boundary-candidate-agreement") => {
            cli::text_boundary::run_text_boundary_candidate_agreement(args)
        }
        Some("text-boundary-candidate-layout-context") => {
            cli::text_boundary::run_text_boundary_candidate_layout_context(args)
        }
        Some("text-boundary-layout-map") => cli::text_boundary::run_text_boundary_layout_map(args),
        Some("text-boundary-layout-map-rows") => {
            cli::text_boundary::run_text_boundary_layout_map_rows(args)
        }
        Some("text-boundary-paragraph-like") => {
            cli::text_boundary::run_text_boundary_paragraph_like(args)
        }
        Some("text-boundary-paragraph-like-style-context") => {
            cli::text_boundary::run_text_boundary_paragraph_like_style_context(args)
        }
        Some("text-boundary-paragraph-like-discriminators") => {
            cli::text_boundary::run_text_boundary_paragraph_like_discriminators(args)
        }
        Some("text-paragraph-boundary-targets") => {
            cli::text_boundary::run_text_paragraph_boundary_targets(args)
        }
        Some("text-position-count-layout-context") => {
            cli::text_position_count::run_text_position_count_layout_context(args)
        }
        Some("text-position-style-context") => {
            cli::text_position_count::run_text_position_style_context(args)
        }
        Some("text-position-style-summary") => {
            cli::text_position_count::run_text_position_style_summary(args)
        }
        Some("paper-marks") => cli::page_mark::run_paper_marks(args),
        Some("paper-mark-shape") => cli::page_mark::run_paper_mark_shape(args),
        Some("page-marks") => cli::page_mark::run_page_marks(args),
        Some("page-mark-u16-profile") => cli::page_mark::run_page_mark_u16_profile(args),
        Some("page-mark-pitch-profile") => cli::page_mark::run_page_mark_pitch_profile(args),
        Some("page-mark-shape") => cli::page_mark::run_page_mark_shape(args),
        Some("text-map") => cli::text_position::run_text_map(args),
        Some("text-position-context") => cli::text_position::run_text_position_context(args),
        Some("text-position-line-context") => {
            cli::text_position::run_text_position_line_context(args)
        }
        Some("text-position-delta-scan") => cli::text_position::run_text_position_delta_scan(args),
        Some("page-layer-tree") => cli::render::run_page_layer_tree(args),
        Some("page-info") => cli::render::run_page_info(args),
        Some("document-info") => cli::render::run_document_info(args),
        Some("page-svg") => cli::render::run_page_svg(args),
        Some("export") => cli::render::run_export(args),
        Some(command) => Err(format!("unknown command: {command}")),
    }
}
