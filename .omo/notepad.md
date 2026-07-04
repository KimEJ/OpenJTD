# G003 Sparse Geometry

- Tier: LIGHT — one focused Rust test plus one diagnostic-only model field inside an existing layer-tree JSON path.
- Skills:
- `omo:programming` — Rust edit/test discipline and failing-first requirement.
- `omo:lsp` — diagnostics check on the modified Rust file after the edit.
- Success criteria:
- RED: focused tsaiten test fails because the new sparse-sibling-derived compact geometry diagnostic is missing.
- GREEN: focused tsaiten test passes after the minimal model change, with no render-admission promotion.
- Manual QA: CLI `page-layer-tree` output shows the new diagnostic object while `sourceDerivedLayoutCandidate` stays `null` and `referenceFallbackAdmissionGate` keeps `sourceLayoutCandidatePresent:false` and `sourceOnlyPageYAdmissionReady:false`.
- Changed-file intent:
- `rjtd/crates/rjtd-model/src/lib.rs` only for product/test logic.
- `.omo/evidence/current-progress-next-plan/*.txt` for captured red/green evidence.
- Post-write review:
- `rjtd/crates/rjtd-model/src/lib.rs` remains an intentional large central file under `rjtd/AGENTS.md`; this change stayed local instead of splitting unrelated code.
- Single responsibility: the new helper only serializes sparse-sibling-derived compact geometry prerequisite evidence.
- Boundary purity / variants / escape hatches: no new unsafe, unwrap, expect, or type erasure added outside the existing test.
- Test lock: the tsaiten focused test now fails without the new diagnostic substring and passes with it.
- Diagnostics note: Rust LSP could not run because `rust-analyzer` is missing from the local toolchain; focused `cargo test` and live CLI evidence are the verification fallback for this turn.
