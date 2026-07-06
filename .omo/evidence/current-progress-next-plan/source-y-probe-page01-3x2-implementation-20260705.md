# PAGE01 3x2 Source-Y Probe Implementation Note

Date: 2026-07-05
Scope: `ichitaro-source-y-probe/new/PAGE 01.jtd`

## Summary

The refreshed `PAGE 01.jtd` baseline is now recognized as a document-text control-run table candidate.
The current implementation safely reaches table topology and cell evidence, but does not promote the table to visible SVG/PDF rendering because page-space origin and row-baseline admission remain unproven.

## Implemented

- `table-candidates` now reports one `documentTextControlRunTableCandidate`.
- The candidate has `intervals=2`, `cells=6/0/6`, and `max-columns=3`.
- `page-layer-tree` now contains a `tableGridCandidate` with `rowCount=2`, `colCountCandidate=3`, and six cell records.
- The collector now tolerates up to three empty document-text control rows between visible table rows. This matches the refreshed PAGE01 control stream.
- Added regression coverage for the refreshed PAGE01 sample in `rjtd-cli` tests.
- Added model coverage for two-row, three-column document-text control tables exposing a column grid candidate.

## Not Promoted To Visible Rendering

`page-svg` and PDF still render the six cell texts as fallback text, not as a visible table grid.

Current blocker:

- `source-derived-layout-not-renderable`
- `page-space-origin-and-row-baseline-unproven`

Reason:

- `DocumentText` line headers match all six cells and provide source-backed cell topology.
- The layer tree has diagnostic table layout evidence.
- The SVG renderer only draws table overlays when a source-derived layout is renderable or a reference fallback is admitted.
- PAGE01 currently has `renderPromoted=false`; forcing visible rendering would promote unproven page-space coordinates.

## Sample Status

The refreshed baseline file is newer than the movement variants:

- `PAGE 01.jtd`: refreshed 2026-07-05 21:23 KST
- `PAGE 01_down_*` and `PAGE 01_right_*`: older variants

The existing movement variants still report `candidateCandidates=0`, so they cannot validate table movement against the refreshed 3x2 baseline.

## Evidence Commands

```bash
cd /Users/kimuj5090/Documents/rjtd/rjtd
cargo run -p rjtd-cli --quiet -- table-candidates ../ichitaro-source-y-probe/new/'PAGE 01.jtd'
cargo run -p rjtd-cli --quiet -- page-layer-tree ../ichitaro-source-y-probe/new/'PAGE 01.jtd' 0
cargo run -p rjtd-cli --quiet -- page-svg ../ichitaro-source-y-probe/new/'PAGE 01.jtd' 0
cargo test -p rjtd-cli --test source_y_probe local_new_probe_baseline_reports_document_text_table_candidate_when_available -- --exact
cargo test -p rjtd-model document_text_control_two_row_three_column_table_exposes_column_grid_candidate
```

## Next Required Evidence

Regenerate the movement variants from the refreshed 3x2 baseline before attempting page-space render promotion:

- Move the same 3x2 table down by known offsets.
- Move the same 3x2 table right by known offsets.
- Keep cell labels stable: `R01C01` through `R02C03`.
- Export matching PDFs for each moved variant.

Promotion remains unsafe until the source-only page-space gate can be proven from those regenerated variants.
