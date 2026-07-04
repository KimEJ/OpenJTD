# Task 11 Notepad

## Status

Blocked path. Source-only readiness was not promoted.

## Why

Both visible `tsaiten` table families do not satisfy the same source-only admission contract:

- Scoring table: no source-derived layout candidate; `sourceReplacementBlockedReason:"source-derived-layout-candidate-absent"`.
- Lower table: source candidate exists, but page-Y render admission is false; `sourceReplacementBlockedReason:"source-page-y-render-admission-not-ready"`.

The local sample assets exist and the local sample test passed. That test preserves visible reference fallback and asserts two `referenceFallbackUsed:true` groups.

## Verification

- Bare acceptance selector ran 0 tests; disclosed and reran as `tests::table_grid_cross_table_subrecord_ordering_helpers_detect_regressions`.
- Qualified helper test passed.
- Local tsaiten sample test passed with JTD/PDF assets present.
- `cargo test --workspace` passed.
- Static `rg` admission audit passed.

## Docs

No TODO/RFC edit needed. Current TODO/RFC entries already say current PDF-backed `tsaiten` visible tables still use legacy reference fallback and carry source replacement blockers.

## Cleanup

No spawned services, temp dirs, generated PDFs, dependency changes, staging, or commits.

## Re-execution 2026-07-01

Disposition remains hard blocked; recommendation is `keep_unchecked`.

Specific missing evidence:

- Scoring table needs full source-derived layout evidence: all 12 line-header cell matches, exact contiguous `/LineMark` row mapping, and `sourceOnlyPageYRenderAdmissionGate.admissionReady:true` without reference bbox or selector-only fallback.
- Lower table needs page-y authority: either stable decoded source-gap-to-page-line transform (`bestCandidateMaxAbsDeltaUnits:0`, family-stable) or PageMark absolute-y slot semantics where line-domain projection agrees. Current residual is `107.539px`.

Fresh verification:

- Bare helper selector: exit 0, 0 tests; disclosed.
- Qualified helper selector: exit 0, 1 passed.
- Local tsaiten sample test: exit 0, 1 passed with JTD/PDF assets present.
- Workspace: exit 0.
- Static admission audit: exit 0.
- TODO/RFC parity audit: exit 0.

Skill coverage recorded in the evidence log and code-review supplement:

- `omo:programming`: loaded Rust rules; no Rust edit because source evidence is insufficient.
- `omo:remove-ai-slops`: no product diff to deslop; overfit review rejects selector-only, single-family, and reference-backed promotion.
