# Task 11 Code Review: tsaiten Admission Re-execution

Result: no product findings requiring code edits in this task. Checkbox 11 remains blocked by missing source evidence, not by an implementation bug that can be safely patched now.

## Findings

1. Source-only promotion is still unsafe for the scoring table.
   Evidence: `rjtd/crates/rjtd-model/src/lib.rs` local assertions show `sourceLayoutCandidatePresent:false`, `sourceReplacementBlockedReason:"source-derived-layout-candidate-absent"`, `matchedCellHeaderCount:0/12`, `rowsWithoutHeaders:[1,2]`, and source-only axis blockers including `source-y-origin-selector-fragmented-by-table` and `source-gap-to-page-line-gap-transform-unstable-across-table-family`.

2. Source-only promotion is still unsafe for the lower table.
   Evidence: local assertions show `sourceLayoutCandidatePresent:true` but `sourceRenderLayoutPresent:false`, `sourceOnlyPageYAdmissionReady:false`, and `sourceReplacementBlockedReason:"source-page-y-render-admission-not-ready"`. The raw `768.000` y slot remains blocked by `line-domain-projection-disagrees-with-page-mark-absolute-y-slot` with residual `107.539`.

3. TODO/RFC parity is already sufficient.
   Evidence: English and Japanese TODO/RFC entries already name `source-derived-layout-candidate-absent`, `source-page-y-render-admission-not-ready`, `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`, and `source-gap-to-page-line-gap-transform-unstable-across-table-family`.

## Programming-Skill Coverage

- Rust programming skill was loaded before product-code decisions.
- No Rust code was edited, so no new type, allocation, unsafe, unwrap/expect, API, or boundary risk was introduced.
- The reviewed code keeps admission data model-owned and does not add exporter/raw-stream shortcuts.
- Future work needs a failing regression that proves both visible `tsaiten` families satisfy `sourceOnlyPageYAdmissionReady:true` before fallback suppression.

## Remove-AI-Slops Coverage

- Scope: task-11 product decision plus evidence artifacts.
- Behavior lock: focused helper test, local `tsaiten` test, workspace test, and static audits passed.
- No slop cleanup was applied because no task-11 product/test code diff exists in this re-execution.
- Overfit check passed: the review rejects promotion from selector-only bbox evidence, single raw PageMark absolute-y support, and reference-backed visual agreement.

## Recommendation

Keep checkbox 11 unchecked. The next product field/test needed is a source-only page-y transform proof: scoring must gain a complete source-derived layout candidate, and lower-table y must prove either a stable source-gap-to-page-line transform with zero max delta or PageMark absolute-y slot semantics that agree with the line-domain projection.
