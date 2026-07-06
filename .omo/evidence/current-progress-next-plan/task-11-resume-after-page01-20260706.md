# Task 11 Resume Check After PAGE01 Probe Work

Date: 2026-07-06 KST
Result: still blocked; no source-only `tsaiten` promotion.

## Why This Check Was Needed

The PAGE01 probe work clarified a separate local table family. Before returning to the original `current-progress-next-plan.md`, task 11 needed a current-state check to ensure the `tsaiten` blocker strings still match live model output.

## Current Live Result

Focused tests still pass:

```bash
cd rjtd
cargo test -p rjtd-model tests::table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact
cargo test -p rjtd-model tests::local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact
```

Result:

```text
1 passed
1 passed
```

Layer tree probe:

```bash
./rjtd/target/debug/rjtd page-layer-tree \
  "rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd" 0
```

The two visible reference-backed `tsaiten` table candidates still preserve reference fallback:

- table candidate `0`
  - `referenceBacked:true`
  - `referenceFallbackAllowed:true`
  - `referenceFallbackUsed:true`
  - `sourceOnlyPageYAdmissionReady:false`
  - current `sourceReplacementBlockedReason:"source-derived-layout-not-renderable"`
  - current source-derived blocker: `sparse-sibling-derived-candidate-render-ineligible`
  - `sourceOnlyAxisAdmissionGate.admissionReady:false`
- table candidate `3`
  - `referenceBacked:true`
  - `referenceFallbackAllowed:true`
  - `referenceFallbackUsed:true`
  - `sourceOnlyPageYAdmissionReady:false`
  - current `sourceReplacementBlockedReason:"source-page-y-render-admission-not-ready"`
  - `sourceOnlyPageYAdmissionClass:"flow-y-stride-only-diagnostic"`
  - `sourceOnlyAxisAdmissionGate.admissionReady:false`

Two middle diagnostic table candidates are source-backed diagnostic projections, but they are not visible reference replacement candidates and do not make task 11 admissible.

## Drift Fixed

Older task-11 evidence said the scoring visible table was blocked by `source-derived-layout-candidate-absent`. Live output now has a source-derived candidate, but it is not renderable:

```text
sourceReplacementBlockedReason=source-derived-layout-not-renderable
renderPromotionBlockedReason=sparse-sibling-derived-candidate-render-ineligible
```

The conclusion is unchanged: no promotion. TODO/RFC wording was updated to use the current live replacement blocker.

## Current Unblock Conditions

Task 11 remains blocked until both visible table families satisfy the same source-only admission contract.

The scoring table needs a renderable source-derived layout, not merely sparse-sibling diagnostic geometry.

The lower table needs a decoded page-Y transform, such as:

- a stable source-gap-to-page-line transform with `bestCandidateMaxAbsDeltaUnits=0` and `tableFamilyTransformStable=true`; or
- independently proven PageMark absolute-y slot semantics where line-domain projection agrees with the raw absolute slot.

Until then:

- keep `referenceFallbackUsed:true`
- keep `sourceOnlyPageYAdmissionReady:false`
- keep `sourceOnlyAxisAdmissionGate.admissionReady:false`
- do not suppress visible reference fallback for either visible `tsaiten` table family
