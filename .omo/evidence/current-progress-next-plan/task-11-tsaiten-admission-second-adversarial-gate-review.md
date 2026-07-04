# Task 11 Second Adversarial Gate Review

recommendation: APPROVE_BLOCKED

## blockers

- Checkbox 11 cannot be marked complete because both visible `tsaiten` table families do not satisfy source-only render admission.
- The scoring table remains blocked by `source-derived-layout-candidate-absent`: live assertions show `sourceLayoutCandidatePresent:false`, `sourceRenderLayoutPresent:false`, `sourceLayoutRenderable:false`, and `sourceOnlyPageYAdmissionReady:false`.
- The lower table remains blocked by `source-page-y-render-admission-not-ready`: live assertions show `sourceLayoutCandidatePresent:true` but `sourceRenderLayoutPresent:false`, `sourceLayoutRenderable:false`, and `sourceOnlyPageYAdmissionReady:false`.
- The lower table's raw PageMark y slot remains non-authoritative because `line-domain-projection-disagrees-with-page-mark-absolute-y-slot` is still present with the documented residual, and the table-family transform remains blocked by `source-gap-to-page-line-gap-transform-unstable-across-table-family`.

## originalIntent

Promote `tsaiten` to source-only readiness only when both visible table families satisfy the same admission contract, without using reference-backed geometry, selector-only bboxes, or single-family evidence as render authority.

## desiredOutcome

Both visible `tsaiten` table families would report source-only admission ready and no longer rely on visible reference fallback. If that cannot be proven safely, the correct user-visible outcome is a blocked disposition with checkbox 11 left unchecked and exact blockers documented.

## userOutcomeReview

The shipped state does not complete checkbox 11, and that is correct. The evidence, live model assertions, and docs all show visible `tsaiten` output remains reference-backed with two `referenceFallbackUsed:true` table groups. There was no per-family visible fallback removal and no source-only promotion. The remaining work is not an obvious one-line implementation path: scoring needs a complete source-derived layout candidate, and the lower family needs a decoded page-y transform or corroborated absolute-y slot semantics that agree with line-domain projection.

## checked artifact paths

- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-11-tsaiten-admission.log`
- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-11-notepad.md`
- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-11-code-review.md`
- `/Users/kimuj5090/Documents/rjtd/rjtd/crates/rjtd-model/src/lib.rs`
- `/Users/kimuj5090/Documents/rjtd/TODO.md`
- `/Users/kimuj5090/Documents/rjtd/TODO.ja.md`
- `/Users/kimuj5090/Documents/rjtd/openjtd-spec/rfc/0008-object-stream-candidates.md`
- `/Users/kimuj5090/Documents/rjtd/openjtd-spec/rfc/0008-object-stream-candidates.ja.md`

## exact evidence gaps

- No product gap found that would allow safe completion now.
- No documentation gap found for the exact remaining blockers: TODO and RFC entries already name `source-derived-layout-candidate-absent`, `source-page-y-render-admission-not-ready`, `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`, and `source-gap-to-page-line-gap-transform-unstable-across-table-family`.
- Artifact coverage is adequate across the evidence set: command exits, 0-test disclosure, qualified passing tests, workspace test pass, Manual QA matrix, programming coverage, remove-ai-slops/overfit coverage, notepad path, and cleanup receipt are present.

## live verification

- `rg` over model/TODO/RFC confirmed `referenceFallbackAdmissionGate`, `sourceReplacementBlockedReason`, `source-derived-layout-candidate-absent`, and `source-page-y-render-admission-not-ready`.
- `rg` over model/TODO/RFC confirmed `source-gap-to-page-line-gap-transform-unstable-across-table-family`, `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`, and `source-y-origin-selector-fragmented-by-table`.
- Live model assertions at `rjtd/crates/rjtd-model/src/lib.rs:85184` require exactly two `referenceFallbackUsed:true` groups.
- Live model assertions at `rjtd/crates/rjtd-model/src/lib.rs:85189` and `rjtd/crates/rjtd-model/src/lib.rs:85192` preserve the scoring and lower-table blocked states.

Final disposition: confirmed_blocked; checkbox_disposition: keep_unchecked.
