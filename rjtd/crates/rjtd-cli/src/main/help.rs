use super::support::write_stdout;

pub(crate) fn print_help() -> Result<(), String> {
    write_stdout(
        "\
rjtd

Rust-based Ichitaro (JTD) Document Engine

Usage:
  rjtd streams <file.jtd>
  rjtd info <file.jtd>
  rjtd dump-stream <file.jtd> <stream-path>
  rjtd style-records <file.jtd>
  rjtd page-layout-style-slots <file.jtd>
  rjtd style-candidates <file.jtd>
  rjtd text-layout-style-records <file.jtd>
  rjtd document-view-style-groups <file.jtd>
  rjtd paragraph-style-records <file.jtd>
  rjtd cfb-map <file.jtd>
  rjtd cfb-dir <file.jtd>
  rjtd stream-meta <file.jtd> <stream-path>
  rjtd stream-chain <file.jtd> <stream-path>
  rjtd stream-words <file.jtd> <stream-path>
  rjtd stream-word-frequencies <file.jtd> <stream-path>
  rjtd line-mark-tags <file.jtd>
  rjtd line-mark-intervals <file.jtd>
  rjtd source-y-probe-audit <corpus-dir>
  rjtd source-y-probe-compare <base.jtd> <candidate.jtd>
  rjtd line-mark-text-context <file.jtd>
  rjtd stream-dwords <file.jtd> <stream-path>
  rjtd stream-dword-frequencies <file.jtd> <stream-path>
  rjtd stream-text-probe <file.jtd> <stream-path>
  rjtd stream-find <file.jtd> <stream-path>
  rjtd stream-find-bytes <file.jtd> <hex-bytes>
  rjtd so-records <file.jtd>
  rjtd object-stream-candidates <file.jtd>
  rjtd object-ownership-references <file.jtd>
  rjtd object-ownership-reference-fields <file.jtd>
  rjtd object-frame-reference-records <file.jtd>
  rjtd object-frame-record-families <file.jtd>
  rjtd object-frame-row-links <file.jtd>
  rjtd object-image-frame-candidates <file.jtd>
  rjtd object-fdm-image-candidates <file.jtd>
  rjtd object-fdm-frame-links <file.jtd>
  rjtd object-fdm-index <file.jtd>
  rjtd object-fdm-index-shape <file.jtd>
  rjtd object-fdm-index-rows <file.jtd>
  rjtd so-record-clusters <file.jtd>
  rjtd so-record-fields <file.jtd>
  rjtd so-record-geometry <file.jtd>
  rjtd so-record-halves <file.jtd>
  rjtd cat <file.jtd>
  rjtd text-tokens <file.jtd>
  rjtd text-control-context <file.jtd> [control-code]
  rjtd text-control-clusters <file.jtd> [control-code]
  rjtd text-control-ranges <file.jtd> [control-code]
  rjtd text-positions <file.jtd>
  rjtd text-position-mark-header <file.jtd>
  rjtd text-position-mark-summary <file.jtd>
  rjtd text-position-counts <file.jtd>
  rjtd text-position-count-context <file.jtd>
  rjtd text-position-count-tail-context <file.jtd>
  rjtd text-position-count-clusters <file.jtd>
  rjtd text-position-count-candidates <file.jtd>
  rjtd text-position-count-family <file.jtd>
  rjtd text-position-count-fields <file.jtd>
  rjtd text-position-count-field-deltas <file.jtd>
  rjtd text-position-count-tail-delta-scan <file.jtd>
  rjtd text-position-count-tail-delta-groups <file.jtd>
  rjtd text-position-count-tail-row-deltas <file.jtd>
  rjtd text-position-count-tail-row-context <file.jtd>
  rjtd text-position-count-tail-field-roles <file.jtd>
  rjtd text-position-count-range-preview <file.jtd>
  rjtd text-position-count-range-boundaries <file.jtd>
  rjtd text-position-count-control-ranges <file.jtd> [control-code]
  rjtd text-boundary-candidates <file.jtd>
  rjtd table-candidates <file.jtd>
  rjtd table-candidate-context <file.jtd>
  rjtd table-cell-like-candidates <file.jtd>
  rjtd text-boundary-candidate-context <file.jtd>
  rjtd text-boundary-candidate-agreement <file.jtd>
  rjtd text-boundary-candidate-layout-context <file.jtd>
  rjtd text-boundary-layout-map <file.jtd>
  rjtd text-boundary-layout-map-rows <file.jtd>
  rjtd text-boundary-paragraph-like <file.jtd>
  rjtd text-boundary-paragraph-like-style-context <file.jtd>
  rjtd text-boundary-paragraph-like-discriminators <file.jtd>
  rjtd text-paragraph-boundary-targets <file.jtd>
  rjtd text-position-count-layout-context <file.jtd>
  rjtd text-position-style-context <file.jtd>
  rjtd text-position-style-summary <file.jtd>
  rjtd paper-marks <file.jtd>
  rjtd paper-mark-shape <file.jtd>
  rjtd page-marks <file.jtd>
  rjtd page-mark-u16-profile <file.jtd>
  rjtd page-mark-pitch-profile <file.jtd>
  rjtd page-mark-shape <file.jtd>
  rjtd text-map <file.jtd>
  rjtd text-position-context <file.jtd>
  rjtd text-position-line-context <file.jtd>
  rjtd text-position-delta-scan <file.jtd>
  rjtd document-info <file.jtd>
  rjtd page-info <file.jtd> <zero-based-page-index>
  rjtd page-layer-tree <file.jtd> <zero-based-page-index>
  rjtd page-svg <file.jtd> <zero-based-page-index>
  rjtd export <file.jtd> --format <json|md|text|html|pdf> [-o output.pdf]
",
    )
}
