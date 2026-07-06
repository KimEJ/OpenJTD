# Source-Y Probe Required Next Evidence (2026-07-05)

## Decision

Do not promote table-grid source-y rendering from the current probe corpus.

The current samples are useful diagnostics, but every source-y probe checked by
`page-layer-tree` still has `sourceOnlyPageYRenderAdmissionGate.admissionReady=false`.
The blocker is not "more samples in general"; it is missing proof for specific
source-to-page coordinate transforms.

## Why Implementation Is Blocked

- `010_table_moved_down_small` and `011_table_moved_down_large` have strong PDF
  table-y movement, but the model still classifies page-y authority as
  `fallbackTextAnchors` with `lineMarkPageOriginStridePresent=true`.
- The current render gate intentionally blocks this path with
  `line-mark-record-stride-to-page-y-transform-unproven`.
- Rendering with `fallbackTextAnchors` would be wrong for the probe corpus:
  `000_base_a` source candidate y is about 127.3 px, while the PDF table top is
  about 90.3 pt.
- `013_table_moved_right` is PDF-visible table-x movement, but source signatures
  are identical. It cannot justify source-only x placement.
- `030/031/032/064` are PDF-visible, but current model evidence is
  `sparseSiblingDerived` or missing renderable page-space geometry.
- `040_top_margin_plus` has page tuple changes, but the margin sweep variants
  imported from RTF are source-silent relative to their RTF baseline.

## Minimal Evidence Needed

### 1. Native Table-Y Stride Sweep

Purpose: prove or reject `lineMarkPageGridStrideRawRecordIndex` as a page-y
transform.

Create native Ichitaro samples from the same `000_base_a` structure:

| id suggestion | intended movement |
|---|---|
| `010n_table_down_1line` | table top moved down by exactly one visual text line |
| `010n_table_down_2line` | table top moved down by exactly two visual text lines |
| `010n_table_down_4line` | table top moved down by exactly four visual text lines |
| `010n_table_down_8line` | table top moved down by exactly eight visual text lines |

Required checks:

- same page setup, font, table shape, row heights, and column widths as
  `000_base_a`
- JTD and PDF pair for every sample
- measured PDF table top delta in points
- `line-mark-intervals` shows a monotonic record/index delta matching the PDF
  top delta within a small tolerance
- `page-layer-tree` still shows whether the transform is stride-only or direct
  `lineMarkPageGrid`

Promotion condition:

- at least four deltas plus base fit one linear transform
- no row-height or column-width changes
- no fallback text-anchor y is used as render authority

### 2. Native Top-Margin Sweep

Purpose: decode page tuple margin fields instead of relying on one native sample
and RTF-import source-silent variants.

Create native document-style samples:

| id suggestion | margin |
|---|---|
| `040n_top_margin_20mm` | 20 mm |
| `040n_top_margin_30mm_baseline` | 30 mm |
| `040n_top_margin_40mm` | 40 mm |
| `040n_top_margin_50mm` | 50 mm |
| `040n_top_margin_60mm` | 60 mm |

Required checks:

- all created by native Ichitaro document style/page setup, not RTF import
- identical table/text content
- PDF table top shifts monotonically with margin
- page tuple field deltas identify the same field family across the sweep

Promotion condition:

- page tuple field(s) map to margin in page space without reference PDF
- source-only page-y gate can name the decoded field, not just a candidate

### 3. Native Table-X Source Sweep

Purpose: find the source field that current extraction misses for
`013_table_moved_right`.

Create native samples:

| id suggestion | intended movement |
|---|---|
| `013n_table_right_small` | small right movement |
| `013n_table_right_large` | large right movement |
| `013n_table_left_small` | small left movement if Ichitaro allows it |

Required checks:

- PDF left coordinate changes with no table-y/width/row-height change
- stream-level binary diff locates the changed record(s)
- source signature extractor is updated to include that record family

Promotion condition:

- source signature differs in a field that varies monotonically with PDF x
- no promotion based only on visual PDF movement

### 4. Native Width/Grid Stability Set

Purpose: make `030/031/032/064` usable for source-only geometry instead of
`sparseSiblingDerived` diagnostics.

Create or repair native samples so they keep the baseline page setup and font:

| id suggestion | intended change |
|---|---|
| `030n_col1_width_plus_same_setup` | first column wider only |
| `031n_col2_width_plus_same_setup` | second column wider only |
| `032n_table_width_plus_same_setup` | whole table wider only |
| `064n_merged_header_same_setup` | merged header only |

Required checks:

- PDF top/left/row gaps remain baseline unless the intended change requires it
- `sourceDerivedLayoutReadiness.sourcePlacementEvidencePresent=true`
- matched rows/cell headers cover the table, not only sparse sibling evidence

Promotion condition:

- source-derived layout becomes `DecodedCompactPlacement`, or a new decoded
  placement path is proven without reference PDF

## Current Artifacts

- Coordinate/source/gate evidence:
  `.omo/evidence/current-progress-next-plan/source-y-probe-coordinate-evidence-20260705.md`
- Required next evidence:
  `.omo/evidence/current-progress-next-plan/source-y-probe-required-next-evidence-20260705.md`

## Current Stop Condition

Implementation is blocked by insufficient source semantics, not by missing
plumbing. The next productive loop is sample/evidence acquisition for the four
sets above, then re-run:

```bash
cd /Users/kimuj5090/Documents/rjtd/rjtd
cargo run -p rjtd-cli -- source-y-probe-audit ../ichitaro-source-y-probe
cargo run -p rjtd-cli -- page-layer-tree ../ichitaro-source-y-probe/files/<sample>.jtd 0
```

## Fable 5 Review Amendments

Fable 5 independently re-reviewed the updated corpus on 2026-07-05 and agreed
that render promotion is still blocked. The review added these plan changes:

- Retire the RTF/import margin sweep from the admission path. `040a` through
  `040e` are useful only as negative evidence because their source signatures
  stay identical against the `040b` baseline.
- Add a diagnostic raw-stream or whole-file diff stage before the next sample
  loop. `013_table_moved_right` is PDF-visible but decoded-signature-silent, so
  the current decoded diff path has a blind spot for table x placement.
- Freeze `030/031/032/064` out of the admission path until same-page-setup
  replacements exist. They remain regression fixtures, not render evidence.
- Fix or remove the missing `074_multi_page_table` manifest entry before
  treating the corpus as clean.
- Reconsider source-only render admission only after isolated native Y/X series
  produce decoded page-origin authority and at least three monotonic,
  single-variable transform points.

Full review artifact:
`.omo/evidence/current-progress-next-plan/fable5-source-y-probe-review-20260705.md`

## New Folder Sample Audit Addendum

The `ichitaro-source-y-probe/new` samples added after the Fable review are
useful but do not replace the existing corpus.

Positive evidence:

- The PDFs are clean one-page A4 files.
- The visual table is present.
- Down samples move `CELL1` vertically by about `16.8 pt` per step.
- Right samples move `CELL1` horizontally by about `5.3 pt` per step.
- Page size and the unintended axis stay stable.

Blocker:

- The new baseline is a one-row, three-column table shape.
- Current JTD decoding reports `tableCandidates=0` and
  `sparseTableCandidates=0` for the new samples.
- `page-layer-tree` therefore emits no `sourceOnlyPageYRenderAdmissionGate` for
  these files.
- Right tick variants are decoded-signature-silent against each other even
  though their PDFs move monotonically, which confirms the need for raw-stream
  or whole-file diff diagnostics.
- Stream hashing narrowed the tick-correlated movement to `/DocumentText` and
  `/RelatedDocuments`. `/DocumentText` word indexes `32`, `60`, `88`, and `116`
  move as `0x0002`, `0x0004`, `0x0006`, `0x0008` across the four right ticks.
  Treat this as a raw diagnostic lead, not render authority.

Action:

- Keep the old samples. Do not delete them; they still provide decoded
  multi-row table candidate coverage.
- Keep the new samples as diagnostic evidence, especially for the raw diff
  stage.
- Regenerate the same clean down/right sweeps as a native multi-row table,
  preferably the original 3-row x 2-column `R01C01` through `R03C02` shape.

Full audit artifact:
`.omo/evidence/current-progress-next-plan/source-y-probe-new-folder-audit-20260705.md`
