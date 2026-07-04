recommendation: APPROVE

blockers: []

originalIntent:
- Task 9: add regression coverage for `tsaiten` source-only geometry blockers before subsequent geometry logic changes.
- Expected behavior: source-only `tsaiten` table geometry remains blocked unless the source-gap transform and PageMark absolute-y slot agreement are decoded/proven.
- Guardrail: do not promote source-only geometry to rendering, do not replace the reference fallback, and do not change renderer output as part of this checkbox.

desiredOutcome:
- Evidence and notepad artifacts exist and are non-empty.
- Evidence records command exits, the misleading unqualified `--exact` zero-test runs, fully qualified passing tests, Manual QA matrix, Code review / slop-overfit review, notepad path, and cleanup receipt.
- Live source contains regression assertions for:
  - `sourceOnlyAxisAdmissionGate`
  - `sourceOnlyPageYRenderAdmissionGate`
  - `source-gap-to-page-line-gap-transform-unstable-across-table-family`
  - `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`
- Local `tsaiten` JTD/PDF branch executes when both local files are present.
- Task-specific additions are regression/diagnostic coverage only; no dependency, renderer-output, reference-fallback, or unproven render-promotion change is attributable to task 9.

userOutcomeReview:
- Evidence exists: `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-9-tsaiten-regression.log` is present and non-empty.
- Notepad exists: `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-9-notepad.md` is present and non-empty.
- Evidence includes the required sections: command exits, zero-test disclosure for unqualified exact selectors, fully qualified 1-test passing selectors, local sample branch, Manual QA matrix, Code review / Slop-Overfit Review, Cleanup Receipt, and Notepad path.
- Independent rerun passed:
  - `cargo fmt --all --check` from `rjtd/` exited 0.
  - `cargo test -p rjtd-model tests::table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact` ran 1 test and passed.
  - `cargo test -p rjtd-model tests::local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact` ran 1 test and passed.
  - Unqualified exact selectors were rerun and each reported `running 0 tests`, matching the evidence disclosure.
- Independent live `rg` found the required gate/blocker strings in source logic and regression assertions at `rjtd/crates/rjtd-model/src/lib.rs`.
- Local sample files are present:
  - `/Users/kimuj5090/Documents/rjtd/rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd`
  - `/Users/kimuj5090/Documents/rjtd/rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.pdf`
- The local sample branch therefore executed during the fully qualified sample test; the test would only early-return if either file was absent.
- Source inspection confirmed the helper regression at `rjtd/crates/rjtd-model/src/lib.rs:85000` constructs concrete readiness/agreement inputs and asserts:
  - `table_family_transform_blocked_reason() == Some("source-gap-to-page-line-gap-transform-unstable-across-table-family")`
  - `table_grid_source_only_page_mark_absolute_y_slot_blocked_reason(...) == "line-domain-projection-disagrees-with-page-mark-absolute-y-slot"`
- Source inspection confirmed the sample-backed test asserts `referenceFallbackUsed:true`, `admissionReady:false`, `sourceBacked:true`, `referenceBacked:false`, blocker strings, and `renderPromoted:false` for the `tsaiten` layer-tree output.
- `git diff -- rjtd/Cargo.toml rjtd/Cargo.lock` was empty; no new dependency is present.
- The broad existing product WIP predates task 9. Task 1 captured `rjtd/crates/rjtd-model/src/lib.rs` at `10922` additions / `245` deletions before this start-work sequence; current numstat is `11029` additions / `245` deletions. The +107 line delta is consistent with the task-9 regression assertions and evidence note.
- No source-only render promotion or reference-fallback replacement is attributable to task 9. The inspected `tsaiten` assertions keep source-only gates non-admissible and reference fallback in use.

checkedArtifactPaths:
- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-9-tsaiten-regression.log`
- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-9-notepad.md`
- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-1-current-state.md`
- `/Users/kimuj5090/Documents/rjtd/rjtd/crates/rjtd-model/src/lib.rs`
- `/Users/kimuj5090/Documents/rjtd/rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd`
- `/Users/kimuj5090/Documents/rjtd/rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.pdf`
- `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.14.1/skills/remove-ai-slops/SKILL.md`
- `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.14.1/skills/programming/SKILL.md`
- `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.14.1/skills/programming/references/rust/README.md`
- `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.14.1/skills/programming/references/rust/proptest-insta.md`
- `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.14.1/skills/programming/references/rust/cargo-strict.md`
- `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.14.1/skills/programming/references/rust/libraries.md`
- `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.14.1/skills/programming/references/code-smells.md`

reproCommands:
- `ls -l .omo/evidence/current-progress-next-plan/task-9-tsaiten-regression.log .omo/evidence/current-progress-next-plan/task-9-notepad.md`
- `sed -n '1,260p' .omo/evidence/current-progress-next-plan/task-9-tsaiten-regression.log`
- `sed -n '1,260p' .omo/evidence/current-progress-next-plan/task-9-notepad.md`
- `rg -n "sourceOnlyAxisAdmissionGate|sourceOnlyPageYRenderAdmissionGate|source-gap-to-page-line-gap-transform-unstable-across-table-family|line-domain-projection-disagrees-with-page-mark-absolute-y-slot" .`
- `find rjtd-testdata/local-samples -maxdepth 1 -type f -name 'ichitaro-20030120132956-0007-sp-dat-tsaiten.*' -ls`
- `cd rjtd && cargo fmt --all --check`
- `cd rjtd && cargo test -p rjtd-model table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact`
- `cd rjtd && cargo test -p rjtd-model tests::table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact`
- `cd rjtd && cargo test -p rjtd-model local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact`
- `cd rjtd && cargo test -p rjtd-model tests::local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact`
- `git diff --numstat -- rjtd/crates/rjtd-model/src/lib.rs`
- `git diff -- rjtd/Cargo.toml rjtd/Cargo.lock`

slopAndProgrammingReview:
- Required `remove-ai-slops` and `programming` lenses were consulted directly, including Rust test/cargo/code-smell references relevant to this task.
- Direct anti-slop pass found no deletion-only tests, no test weakening, no tests that merely verify a requested removal, no excessive test sprawl, no new production extraction/parsing/normalization, and no unnecessary abstractions or dependencies attributable to task 9.
- The helper test is narrow branch coverage of diagnostic helper behavior, and the stronger local-sample test asserts observable layer-tree diagnostics and reference-fallback behavior. This is acceptable for the task because the user-visible contract is the diagnostic blocker surface.
- The evidence log's Code Review / Slop-Overfit Review explicitly covers the relevant false-confidence classes: no weakened tests, no tautological implementation mirroring, no unnecessary abstractions/dependencies, no generated output as source truth, and no unproven render promotion.
- The touched Rust file is far above the programming skill's 250 pure LOC ceiling, but task 1 records this as pre-existing central WIP. Splitting `rjtd-model/src/lib.rs` is outside checkbox 9 and would violate the user's narrow regression-before-logic-change intent.

exactEvidenceGaps:
- No isolated task-9 patch artifact was provided. Attribution is therefore based on the task-1 pre-existing WIP ledger plus the current diff-stat delta and inspected test additions.
- Whole-workspace test/clippy coverage was not run for this checkbox; the evidence and this gate both rely on scoped fmt plus the two relevant fully qualified tests.

AdversarialVerify:
  task: "9. Add regression coverage for tsaiten source-only geometry blockers before changing geometry logic."
  verdict: confirmed
  evidence:
    - "evidence/notepad artifacts exist and include the required command exits, zero-test disclosures, passing fully qualified tests, Manual QA matrix, review section, notepad path, and cleanup receipt"
    - "independent fmt and two fully qualified exact tests passed"
    - "unqualified exact selectors independently reproduced the 0-test hazard"
    - "live rg audits found sourceOnlyAxisAdmissionGate, sourceOnlyPageYRenderAdmissionGate, source-gap-to-page-line-gap-transform-unstable-across-table-family, and line-domain-projection-disagrees-with-page-mark-absolute-y-slot"
    - "local tsaiten JTD/PDF are present, so the sample-backed branch executed"
    - "source inspection confirms task-specific coverage asserts source-gap transform instability, absolute-y slot disagreement, reference fallback use, non-admission, and renderPromoted:false"
    - "no dependency or source-only render-promotion change is attributable to task 9"
  repro_commands:
    - "cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo fmt --all --check"
    - "cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model tests::table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact"
    - "cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model tests::local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact"
    - "cd /Users/kimuj5090/Documents/rjtd && rg -n \"sourceOnlyAxisAdmissionGate|sourceOnlyPageYRenderAdmissionGate|source-gap-to-page-line-gap-transform-unstable-across-table-family|line-domain-projection-disagrees-with-page-mark-absolute-y-slot\" ."
  confidence: high
  notes: "Approval is scoped to checkbox 9. It does not approve the broader pre-existing dirty product WIP as a whole."
