# current-progress-next-plan - Work Plan

## TL;DR (For humans)
**What you'll get:** A verified, commit-ready continuation of the current reverse-engineering work: first stabilize the existing diagnostics, then close the next evidence gates for table geometry and FDM ownership without asking the executor to make product calls.

**Why this approach:** The current tree already contains a broad model/export/CLI/spec WIP. The safest useful next move is to prove what is already there, then advance only source-backed gates that are explicitly blocking reference-fallback removal or primitive ownership.

**What it will NOT do:** It will not promote visible rendering from reference-backed, filename-backed, selector-only, or raw-stream exporter evidence. It will not copy from `rhwp/`, add dependencies, weaken tests, or auto-commit without an explicit commit step.

**Effort:** XL
**Risk:** High - the current diff crosses the model, exporter, CLI, tests, RFC/TODO docs, and local sample/PDF evidence paths.
**Decisions I made for you:** I treated the request as open-ended and chose defaults. Default 1: stabilize and verify the current WIP before starting new feature work. Default 2: make `tsaiten` source-only table/page-grid geometry the first implementation lane. Default 3: keep Q4/Q5 FDM primitive ownership and `shanai_lan` wiring diagnostic-only until source-backed admission gates prove render authority.

Your next move: run `$start-work .omo/plans/current-progress-next-plan.md`. Full execution detail follows below.

---

> TL;DR (machine): XL/high-risk plan to verify current diagnostic-preservation WIP, then advance `tsaiten`, Q4/Q5 FDM ownership, and `shanai_lan` source-backed gates with evidence-first QA.

## Scope
### Must have
- Preserve and verify the current diagnostic-preservation WIP in `TODO.md`, `openjtd-spec/rfc/0008-object-stream-candidates.md`, `rjtd/crates/rjtd-model/src/lib.rs`, `rjtd/crates/rjtd-export/src/lib.rs`, `rjtd/crates/rjtd-cli/src/main.rs`, and `rjtd/crates/rjtd-cli/tests/streams.rs`.
- Treat `rjtd/` as the implementation workspace and run Rust commands from `/Users/kimuj5090/Documents/rjtd/rjtd`.
- Keep exporters model-first: exporter code consumes `Document`/model-owned evidence and must not scan raw CFB/container/stream/record bytes directly.
- Preserve unknown and decoded-false data as evidence (`UnknownRecord`, `UnknownBlock`, `UnknownStyle`, `UnknownObject`, object candidates, diagnostic gates) rather than discarding or promoting it.
- Keep `rhwp/` read-only. Compare architecture, dependency choices, and testing style only; do not copy code.
- Verify source-backed gate names and blocker strings across model JSON, export JSON/SVG diagnostics, CLI output, TODO, and RFC docs.
- Maintain Japanese translated TODO/RFC files when English TODO/RFC content changes.
- Add regression tests before changing behavior in new implementation waves where existing coverage does not already lock the gate.
- Produce `.omo/evidence/` logs for every task and final wave.

### Must NOT have (guardrails, anti-slop, scope boundaries)
- Must not edit product code during planning. During execution, edit only files named by the relevant todo unless a failing compiler/test error proves a direct dependency.
- Must not promote visible rendering from reference-backed, filename-backed, selector-only, or manually calibrated evidence.
- Must not remove `decoded:false`, `diagnosticOnly:true`, `referenceBacked`, `placementProven:false`, `geometryDecoded:false`, `paintOrderDecoded:false`, or blocker fields unless the same todo proves the replacement source authority.
- Must not weaken, delete, or broad-match tests to make a failing run pass.
- Must not add new dependencies unless `rhwp/Cargo.toml` and local policy show an equivalent precedent and the todo explicitly allows it.
- Must not commit generated PDFs, local sample data, or `.omo/evidence/` unless the user explicitly asks for those artifacts to be versioned.

## Verification strategy
> Zero human intervention - all verification is agent-executed.
- Test decision: tests-after for stabilizing the current WIP; test-first for new gate-closing behavior in `tsaiten`, Q4/Q5 FDM ownership, and `shanai_lan` lanes.
- Baseline commands:
  - `cd /Users/kimuj5090/Documents/rjtd && git status --short --branch`
  - `cd /Users/kimuj5090/Documents/rjtd && git diff --stat`
  - `cd /Users/kimuj5090/Documents/rjtd && git diff --check`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo fmt --all --check`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo check --workspace`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test --workspace`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo clippy --workspace --all-targets -- -D warnings`
- Focused commands for current WIP:
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-cli --test streams object_stream_candidates_command_reports_visual_object_inventory -- --exact`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-cli --test streams object_fdm_image_candidates_command_reports_signature_only_blocker -- --exact`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-cli --test streams object_fdm_frame_links_command_connects_fdm_rows_to_frame_records -- --exact`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-cli --test streams object_fdm_index_command_links_index_rows_to_vector_image_hits -- --exact`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model image_payload_render_gate_preserves_source_frame_trace_without_promotion -- --exact`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model parser_preserves_object_stream_candidates_as_model_evidence -- --exact`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-export exports_object_stream_candidates_to_json -- --exact`
  - `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-export local_success_data_test_exports_embedding_frame_candidates_when_reference_pdf_is_available -- --exact`
- Local-sample commands are conditional: tests already return early when `rjtd-testdata/local-samples` or matching reference PDFs are absent. If samples exist, preserve their assertions; if absent, record that the local-sample branch was skipped by test code and do not claim PDF visual coverage.
- Evidence root: `.omo/evidence/current-progress-next-plan/`. Each todo writes the concrete evidence filename listed in that todo, such as `task-1-current-state.md`, `task-18-full-rust.log`, `task-19-visual-qa.md`, `task-20-handoff.md`, and `f1-plan-compliance.log` through `f4-scope-fidelity.md`.

## Execution strategy
### Parallel execution waves
- Wave 0: Inventory and protect the dirty worktree before edits. Todos 1-3 can run in parallel after the current status is captured.
- Wave 1: Stabilize current WIP and cross-surface field/blocker consistency. Todos 4-8 can run in parallel after Wave 0.
- Wave 2: Close the next `tsaiten` source-only geometry evidence gap. Todos 9-11 are sequential because each depends on the previous gate evidence.
- Wave 3: Close Q4/Q5 FDM primitive ownership blockers. Todos 12-14 can run in parallel after Wave 1, but todo 14 depends on the tests from 12 and 13.
- Wave 4: Keep `shanai_lan` diagnostic gates coherent and prevent visible wiring regressions. Todos 15-17 can run in parallel after Wave 1.
- Wave 5: Whole-workspace verification, docs parity, and handoff. Todos 18-20 run after all implementation lanes.

### Dependency matrix
| Todo | Depends on | Blocks | Can parallelize with |
| --- | --- | --- | --- |
| 1 | none | 4, 18, 20 | 2, 3 |
| 2 | none | 4, 5, 6, 7, 8 | 1, 3 |
| 3 | none | 18, 19, 20 | 1, 2 |
| 4 | 1, 2 | 5, 6, 7, 8, 18 | none |
| 5 | 4 | 18, 19 | 6, 7, 8 |
| 6 | 4 | 12, 13, 14, 18 | 5, 7, 8 |
| 7 | 4 | 15, 16, 17, 18 | 5, 6, 8 |
| 8 | 4 | 18, 19 | 5, 6, 7 |
| 9 | 5 | 10 | 12, 15 |
| 10 | 9 | 11 | 13, 16 |
| 11 | 10 | 18, 19 | 14, 17 |
| 12 | 6 | 14 | 9, 15 |
| 13 | 6 | 14 | 10, 16 |
| 14 | 12, 13 | 18, 19 | 11, 17 |
| 15 | 7 | 17 | 9, 12 |
| 16 | 7 | 17 | 10, 13 |
| 17 | 15, 16 | 18, 19 | 11, 14 |
| 18 | 1-17 | 19 | none |
| 19 | 5-18 | 20 | none |
| 20 | 1-19 | F1-F4 | none |

## Todos
> Implementation + Test = ONE todo. Never separate.
- [x] 1. Capture the authoritative current-state ledger before edits
  What to do / Must NOT do: Record `git status --short --branch`, `git diff --stat`, `git diff --name-status`, and `git diff --numstat` into `.omo/evidence/current-progress-next-plan/task-1-current-state.md`. Classify pre-existing WIP vs planning artifacts. Must not stage, revert, or normalize user changes.
  Parallelization: Wave 0 | Blocked by: none | Blocks: 4, 18, 20
  References (executor has NO interview context - be exhaustive): `.omo/drafts/current-progress-next-plan.md`; `TODO.md:686`; `TODO.md:691`; `docs/ARCHITECTURE.md:27`; `docs/RHWP-COMPATIBILITY.md:30`
  Acceptance criteria (agent-executable): `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/task-1-current-state.md && rg -n "^## Branch|^## Tracked WIP|^## Untracked planning/guidance artifacts|^## Pre-existing implementation/doc WIP|^## Risks" .omo/evidence/current-progress-next-plan/task-1-current-state.md`
  QA scenarios (name the exact tool + invocation): Happy: `git status --short --branch` shows the same tracked WIP plus `.omo/`, `AGENTS.md`, and `rjtd/AGENTS.md`; save output in `.omo/evidence/current-progress-next-plan/task-1-current-state.md`. Failure: run `git diff --check`; if whitespace errors exist, record exact paths and stop before implementation edits. Evidence `.omo/evidence/current-progress-next-plan/task-1-current-state.md`
  Commit: N | planning evidence only

- [x] 2. Map changed symbols to verification surfaces
  What to do / Must NOT do: Build a symbol/test map for `ObjectStreamCandidate`, `jsfartStreamProfile`, `imagePayloadDiagnostic`, `referenceFallbackAdmissionGate`, `sourceOnlyAxisAdmissionGate`, `sourceOnlyPageYRenderAdmissionGate`, `primitiveOwnershipAdmissionGate`, `gridOriginAuthorityGate`, and `lineRuleRenderAdmissionGate`. Must not inspect `rhwp/` except to confirm policy or dependency precedent.
  Parallelization: Wave 0 | Blocked by: none | Blocks: 4, 5, 6, 7, 8
  References: `rjtd/crates/rjtd-cli/src/main.rs:5270`; `rjtd/crates/rjtd-export/src/lib.rs:783`; `rjtd/crates/rjtd-export/src/lib.rs:1873`; `rjtd/crates/rjtd-export/src/lib.rs:2315`; `rjtd/crates/rjtd-model/src/lib.rs:18508`; `rjtd/crates/rjtd-model/src/lib.rs:22827`; `rjtd/crates/rjtd-model/src/lib.rs:37559`; `rjtd/crates/rjtd-model/src/lib.rs:39643`; `rjtd/crates/rjtd-model/src/lib.rs:57397`; `rjtd/crates/rjtd-model/src/lib.rs:68908`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/task-2-symbol-test-map.md && rg -n "ObjectStreamCandidate|sourceOnlyAxisAdmissionGate|primitiveOwnershipAdmissionGate|lineRuleRenderAdmissionGate|cargo test" .omo/evidence/current-progress-next-plan/task-2-symbol-test-map.md`
  QA scenarios: Happy: run `rg -n "jsfartStreamProfile|sourceOnlyAxisAdmissionGate|primitiveOwnershipAdmissionGate|lineRuleRenderAdmissionGate" rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-cli/src/main.rs rjtd/crates/rjtd-cli/tests/streams.rs` and save a curated map. Failure: if a field appears in model but not export/CLI/docs where expected, record the missing surface and assign it to todo 4 or 5. Evidence `.omo/evidence/current-progress-next-plan/task-2-symbol-test-map.md`
  Commit: N | planning evidence only

- [x] 3. Confirm local sample and PDF tooling availability
  What to do / Must NOT do: Determine whether `rjtd-testdata/local-samples`, `openjtd-samples/pdf-output`, `pdftoppm`, `pdfinfo`, `sips`, and Swift/PDFKit are available. Must not regenerate PDFs yet.
  Parallelization: Wave 0 | Blocked by: none | Blocks: 18, 19, 20
  References: `README.md:30`; `README.md:39`; `rjtd/crates/rjtd-cli/tests/streams.rs:4331`; `rjtd/crates/rjtd-export/src/lib.rs:8079`; `rjtd/crates/rjtd-model/src/lib.rs:82706`; `rjtd/crates/rjtd-model/src/lib.rs:84951`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/task-3-tooling.md && rg -n "local-samples|pdf-output|pdftoppm|pdfinfo|sips|PDFKit|available|missing" .omo/evidence/current-progress-next-plan/task-3-tooling.md`
  QA scenarios: Happy: commands `command -v pdftoppm`, `command -v pdfinfo`, `command -v sips`, `command -v swift`, plus `test -d rjtd-testdata/local-samples` are recorded. Failure: missing tools are marked as verification gaps and replaced with structural Rust tests, not silent success claims. Evidence `.omo/evidence/current-progress-next-plan/task-3-tooling.md`
  Commit: N | planning evidence only

- [x] 4. Make current WIP compile and format cleanly without behavior changes
  What to do / Must NOT do: Fix only compile, formatting, clippy, or obvious string/field consistency errors caused by the existing WIP. Must not broaden renderer behavior or change admission semantics in this task.
  Parallelization: Wave 1 | Blocked by: 1, 2 | Blocks: 5, 6, 7, 8, 18
  References: `rjtd/Cargo.toml:1`; `rjtd/crates/rjtd-model/src/lib.rs:18739`; `rjtd/crates/rjtd-export/src/lib.rs:920`; `rjtd/crates/rjtd-cli/src/main.rs:5270`; `rjtd/crates/rjtd-cli/tests/streams.rs:2403`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo fmt --all --check && cargo check --workspace`
  QA scenarios: Happy: save successful fmt/check output to `.omo/evidence/current-progress-next-plan/task-4-compile.log`. Failure: introduce the smallest source edit needed to satisfy the compiler, then rerun the exact command and record before/after. Evidence `.omo/evidence/current-progress-next-plan/task-4-compile.log`
  Commit: N | keep with final atomic commit unless user asks for intermediate commits

- [x] 5. Lock object-stream and image-payload diagnostic preservation across model, export, and CLI
  What to do / Must NOT do: Ensure `jsfartStreamProfile`, strict image payload/envelope evidence, ownership/frame rows, FDM image candidates, and signature-only blockers are consistently exposed and tested. Must not mark any object/FDM image payload `renderable:true` unless page assignment and paint order are proven in the same task.
  Parallelization: Wave 1 | Blocked by: 4 | Blocks: 18, 19
  References: `openjtd-spec/rfc/0008-object-stream-candidates.md:74`; `openjtd-spec/rfc/0008-object-stream-candidates.md:181`; `openjtd-spec/rfc/0008-object-stream-candidates.md:191`; `rjtd/crates/rjtd-cli/tests/streams.rs:2403`; `rjtd/crates/rjtd-cli/tests/streams.rs:2714`; `rjtd/crates/rjtd-cli/tests/streams.rs:2742`; `rjtd/crates/rjtd-model/src/lib.rs:78505`; `rjtd/crates/rjtd-model/src/lib.rs:78635`; `rjtd/crates/rjtd-model/src/lib.rs:78733`; `rjtd/crates/rjtd-export/src/lib.rs:7453`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-cli --test streams object_stream_candidates_command_reports_visual_object_inventory -- --exact && cargo test -p rjtd-cli --test streams object_fdm_image_candidates_command_reports_signature_only_blocker -- --exact && cargo test -p rjtd-cli --test streams object_fdm_frame_links_command_connects_fdm_rows_to_frame_records -- --exact && cargo test -p rjtd-model image_payload_render_gate_preserves_source_frame_trace_without_promotion -- --exact && cargo test -p rjtd-export exports_object_stream_candidates_to_json -- --exact`
  QA scenarios: Happy: assert outputs include `jsfart-stream-profile=1`, `image-signature-without-complete-payload-role-unproven`, `fdm-frame-linked-image-payload-placement-and-paint-order-unproven`, and `renderable=false`. Failure: deliberately search for `renderable":true` in object image diagnostic paths and fail the task if it appears without matching page/paint proof in the same JSON object. Evidence `.omo/evidence/current-progress-next-plan/task-5-object-image.log`
  Commit: Y | `test(object-evidence): preserve decoded-false object image gates`

- [x] 6. Lock Q4/Q5 primitive ownership admission gates without render promotion
  What to do / Must NOT do: Ensure Q4/Q5 `primitiveOwnershipComparison` carries `ownershipGate`, `offsetFieldAuthorityGate`, `rowFanoutSegmentOwnerGate`, `roleVectorOffsetAuthorityGate`, `roleFanoutSegmentOwnerGate`, `paintOrderContinuityProfile`, `indexRowOrderPromotionGate`, and `primitiveOwnershipAdmissionGate`. Must not draw new Q4/Q5 primitives based on these gates.
  Parallelization: Wave 1 | Blocked by: 4 | Blocks: 12, 13, 14, 18
  References: `TODO.md:691`; `TODO.md:692`; `TODO.md:700`; `openjtd-spec/rfc/0008-object-stream-candidates.md:193`; `openjtd-spec/rfc/0008-object-stream-candidates.md:205`; `rjtd/crates/rjtd-export/src/lib.rs:2315`; `rjtd/crates/rjtd-export/src/lib.rs:3049`; `rjtd/crates/rjtd-model/src/lib.rs:68908`; `rjtd/crates/rjtd-model/src/lib.rs:69658`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-export fdm_bbox_center_handles_extreme_bounds_without_overflow -- --exact && cargo test -p rjtd-model fdm_bbox_center_handles_extreme_bounds_without_overflow -- --exact && rg -n "primitiveOwnershipAdmissionGate|fdm-index-role-vector-offset-authority-valid-vector-offset-missing|role-paint-order-authority-unproven|fdm-index-role-row-fanout-multi-command-single-row" crates/rjtd-export/src/lib.rs crates/rjtd-model/src/lib.rs`
  QA scenarios: Happy: always run the non-skipping fixture tests and static assertion audit; if `../rjtd-testdata/local-samples/ichitaro-20030228030923-success-002-success_data-test.jtd` and its `.pdf` exist, additionally run `cargo test -p rjtd-export local_success_data_test_exports_embedding_frame_candidates_when_reference_pdf_is_available -- --exact` and require Q4/Q5 blocker assertions. Failure: if local samples are absent, record `sample-dependent Q4/Q5 behavioral branch skipped` in evidence and do not claim PDF-backed Q4/Q5 behavior beyond the static assertion audit. Evidence `.omo/evidence/current-progress-next-plan/task-6-q4-q5-current.log`
  Commit: Y | `test(fdm): lock primitive ownership admission blockers`

- [x] 7. Lock `shanai_lan` diagnostic-only line-rule and connector gates
  What to do / Must NOT do: Ensure `gridOriginAuthorityGate`, selected LineMark source-unit evidence, PageMark Y probe, `lineRuleRenderAdmissionGate`, same-row FDM connector summaries, and image-signature fragment blockers stay diagnostic-only. Must not enable direct connector or full-span line-rule rendering.
  Parallelization: Wave 1 | Blocked by: 4 | Blocks: 15, 16, 17, 18
  References: `TODO.md:701`; `TODO.md:717`; `openjtd-spec/rfc/0008-object-stream-candidates.md:163`; `openjtd-spec/rfc/0008-object-stream-candidates.md:173`; `rjtd/crates/rjtd-model/src/lib.rs:56316`; `rjtd/crates/rjtd-model/src/lib.rs:57397`; `rjtd/crates/rjtd-model/src/lib.rs:82623`; `rjtd/crates/rjtd-export/src/lib.rs:8079`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model document_core_projects_shanai_lan_fdm_frame_diagnostics -- --exact && cargo test -p rjtd-model fdm_connector_line_rule_endpoint_match_summary_blocks_single_endpoint -- --exact && cargo test -p rjtd-model fdm_axis_rule_source_order_gate_classifies_parent_relative_offset_spans -- --exact`
  QA scenarios: Happy: assert `line-rule-render-admission-not-ready`, `document-text-grid-origin-reference-backed`, or equivalent blockers remain present and no new visible wiring is emitted. Failure: if a visual probe improves one metric but worsens mean/RMS or lacks source ownership, record rejection and keep the blocker. Evidence `.omo/evidence/current-progress-next-plan/task-7-shanai-current.log`
  Commit: Y | `test(shanai-lan): keep wiring gates diagnostic-only`

- [x] 8. Synchronize English/Japanese TODO and RFC records with current gate names
  What to do / Must NOT do: Align `TODO.md`, `TODO.ja.md`, `openjtd-spec/rfc/0008-object-stream-candidates.md`, and `.ja.md` with the exact blocker names and field names in code/tests. Must not let generated output become the source of truth; docs describe model/test evidence.
  Parallelization: Wave 1 | Blocked by: 4 | Blocks: 18, 19
  References: `TODO.md:686`; `TODO.md:729`; `openjtd-spec/rfc/0008-object-stream-candidates.md:74`; `openjtd-spec/rfc/0008-object-stream-candidates.md:225`; `docs/ARCHITECTURE.md:27`; `docs/RHWP-COMPATIBILITY.md:30`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && rg -n "sourceOnlyAxisAdmissionGate|primitiveOwnershipAdmissionGate|lineRuleRenderAdmissionGate|image-signature-without-complete-payload-role-unproven|fdm-frame-linked-image-payload-placement-and-paint-order-unproven" TODO.md TODO.ja.md openjtd-spec/rfc/0008-object-stream-candidates.md openjtd-spec/rfc/0008-object-stream-candidates.ja.md`
  QA scenarios: Happy: every newly introduced blocker appears in code/tests and at least the English RFC/TODO; Japanese mirrors the same facts. Failure: if docs mention stale blocker strings, update docs or tests so the same canonical string is used. Evidence `.omo/evidence/current-progress-next-plan/task-8-doc-sync.log`
  Commit: Y | `docs(jtd-evidence): sync diagnostic gate records`

- [x] 9. Add regression coverage for `tsaiten` source-only geometry blockers before changing geometry logic
  What to do / Must NOT do: Add or tighten tests that assert current `tsaiten` blockers: `source-derived-layout-candidate-absent`, `source-page-y-render-admission-not-ready`, `source-y-origin-selector-fragmented-by-table`, `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`, and `source-gap-to-page-line-gap-transform-unstable-across-table-family`. Must not change renderer output in this todo except to expose missing diagnostic fields.
  Parallelization: Wave 2 | Blocked by: 5 | Blocks: 10
  References: `TODO.md:686`; `TODO.md:690`; `TODO.md:717`; `openjtd-spec/rfc/0008-object-stream-candidates.md:213`; `openjtd-spec/rfc/0008-object-stream-candidates.md:225`; `rjtd/crates/rjtd-model/src/lib.rs:84951`; `rjtd/crates/rjtd-model/src/lib.rs:85075`; `rjtd/crates/rjtd-model/src/lib.rs:85103`; `rjtd/crates/rjtd-model/src/lib.rs:85864`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact && rg -n "sourceOnlyAxisAdmissionGate|sourceOnlyPageYRenderAdmissionGate|source-gap-to-page-line-gap-transform-unstable-across-table-family|line-domain-projection-disagrees-with-page-mark-absolute-y-slot" crates/rjtd-model/src/lib.rs`
  QA scenarios: Happy: always run the non-skipping helper test and static assertion audit; if `../rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd` and its `.pdf` exist, additionally run `cargo test -p rjtd-model local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact` and require both reference fallback and source-only blockers. Failure: if sample is absent, record `sample-dependent tsaiten behavioral branch skipped` and do not claim reference/PDF coverage. Evidence `.omo/evidence/current-progress-next-plan/task-9-tsaiten-regression.log`
  Commit: Y | `test(tsaiten): lock source-only geometry blockers`

- [x] 10. Derive the next `tsaiten` source-only page-space transform candidate without replacing reference fallback
  What to do / Must NOT do: Investigate additional source fields for scoring and lower tables, then add diagnostic fields only if they reduce ambiguity in X/width or page-Y semantics. Keep visible `tsaitenReferenceProjection` and `referenceFallbackUsed:true` until `sourceOnlyPageYRenderAdmissionGate.admissionReady` is source-backed for both visible table families.
  Parallelization: Wave 2 | Blocked by: 9 | Blocks: 11
  References: `TODO.md:690`; `openjtd-spec/rfc/0008-object-stream-candidates.md:213`; `openjtd-spec/rfc/0008-object-stream-candidates.md:225`; `rjtd/crates/rjtd-model/src/lib.rs:37559`; `rjtd/crates/rjtd-model/src/lib.rs:39643`; `rjtd/crates/rjtd-model/src/lib.rs:40205`; `rjtd/crates/rjtd-model/src/lib.rs:42253`; `rjtd/crates/rjtd-model/src/lib.rs:73443`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact && rg -n "sourceOnlyAxisCandidateBBox|sourceGapToPageLineGapTransformAdmissionGate|sourceOnlyPageMarkAbsoluteYSlotGate|page-mark-absolute-y-slot" crates/rjtd-model/src/lib.rs`
  QA scenarios: Happy: always run the non-skipping helper/static audit; if local `tsaiten` and PageMark PDF-backed samples exist, additionally run `cargo test -p rjtd-model local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact`, `cargo test -p rjtd-cli --test streams local_pdf_backed_page_mark_u16_profiles_stay_stable_when_available -- --exact`, and `cargo test -p rjtd-cli --test streams local_pdf_backed_page_mark_pitch_profiles_stay_stable_when_available -- --exact`. Failure: if a candidate depends on reference row tops, keep it under a `referenceBacked:true` diagnostic and do not wire it into admission. Evidence `.omo/evidence/current-progress-next-plan/task-10-tsaiten-transform.log`
  Commit: Y | `feat(tsaiten): expose source-only transform candidates`

- [ ] 11. Promote `tsaiten` only to source-only readiness when both table families satisfy the admission contract
  What to do / Must NOT do: If todo 10 proves a stable transform, update `sourceOnlyAxisAdmissionGate`, `sourceOnlyPageYRenderAdmissionGate`, and `referenceFallbackAdmissionGate` so visible reference fallback is suppressed only when both visible `tsaiten` table families satisfy the same source-only admission contract. Per-family readiness may be exposed diagnostically, but no per-family visible fallback removal is allowed. If either family remains insufficient, leave rendering unchanged and update TODO/RFC with the exact remaining blocker.
  Parallelization: Wave 2 | Blocked by: 10 | Blocks: 18, 19
  References: `TODO.md:687`; `TODO.md:688`; `TODO.md:690`; `openjtd-spec/rfc/0008-object-stream-candidates.md:207`; `openjtd-spec/rfc/0008-object-stream-candidates.md:225`; `rjtd/crates/rjtd-model/src/lib.rs:73745`; `rjtd/crates/rjtd-model/src/lib.rs:73771`; `rjtd/crates/rjtd-model/src/lib.rs:85075`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact && cargo test --workspace && rg -n "referenceFallbackAdmissionGate|referenceFallbackAllowed|referenceFallbackUsed|sourceReplacementBlockedReason" crates/rjtd-model/src/lib.rs`
  QA scenarios: Happy: if admission becomes ready and local `tsaiten` assets exist, assert `referenceFallbackAllowed:false`, `referenceFallbackUsed:false`, `sourceOnlyPageYAdmissionReady:true`, and source-only basis records for both visible table families in the local sample test. Failure: if either family remains blocked or assets are absent, assert the same visible reference output where testable plus updated blocker strings, and mark TODO item still unchecked rather than claiming source-only replacement. Evidence `.omo/evidence/current-progress-next-plan/task-11-tsaiten-admission.log`
  Commit: Y | `feat(tsaiten): gate reference fallback with source admission`

- [x] 12. Prove or narrow Q4/Q5 FDM vector-offset authority before using role references
  What to do / Must NOT do: Add tests/diagnostics around `roleVectorOffsetAuthorityGate` and `indexRowOrderPromotionGate` to identify which FDMIndex field, if any, is authoritative for role ownership. Must not treat `bbox.left` matches as vector-offset authority without valid `FDMIndex.vectorOffset` evidence.
  Parallelization: Wave 3 | Blocked by: 6 | Blocks: 14
  References: `TODO.md:691`; `TODO.md:700`; `openjtd-spec/rfc/0008-object-stream-candidates.md:199`; `openjtd-spec/rfc/0008-object-stream-candidates.md:205`; `rjtd/crates/rjtd-export/src/lib.rs:3213`; `rjtd/crates/rjtd-export/src/lib.rs:3618`; `rjtd/crates/rjtd-model/src/lib.rs:3618`; `rjtd/crates/rjtd-model/src/lib.rs:70634`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-export fdm_bbox_center_handles_extreme_bounds_without_overflow -- --exact && rg -n "roleVectorOffsetAuthorityGate|fdm-index-role-vector-offset-authority-valid-vector-offset-missing|validVectorOffsetReferenceCount|allReferencesHaveInvalidVectorOffset" crates/rjtd-export/src/lib.rs crates/rjtd-model/src/lib.rs`
  QA scenarios: Happy: always run the non-skipping fixture/static audit; if `../rjtd-testdata/local-samples/ichitaro-20030228030923-success-002-success_data-test.jtd` and its `.pdf` exist, additionally run `cargo test -p rjtd-export local_success_data_test_exports_embedding_frame_candidates_when_reference_pdf_is_available -- --exact` and require Q4/Q5 JSON to show valid/invalid vector-offset counts plus `fdm-index-role-vector-offset-authority-valid-vector-offset-missing` until proven. Failure: if a candidate has no valid vector offset, do not remove the blocker; if assets are absent, record the sample-dependent branch as skipped and do not claim PDF-backed Q4/Q5 behavior. Evidence `.omo/evidence/current-progress-next-plan/task-12-q4-q5-vector-offset.log`
  Commit: Y | `feat(fdm): narrow vector offset authority blockers`

- [x] 13. Prove or narrow Q4/Q5 row fanout and segment ownership semantics
  What to do / Must NOT do: Extend or test `rowFanoutSegmentOwnerGate` and `roleFanoutSegmentOwnerGate` so Q5 multi-command single-row cases explain whether one row can own multiple commands. Must not collapse fanout blockers into a generic primitive-role blocker.
  Parallelization: Wave 3 | Blocked by: 6 | Blocks: 14
  References: `TODO.md:695`; `TODO.md:696`; `TODO.md:697`; `openjtd-spec/rfc/0008-object-stream-candidates.md:195`; `openjtd-spec/rfc/0008-object-stream-candidates.md:197`; `rjtd/crates/rjtd-export/src/lib.rs:2682`; `rjtd/crates/rjtd-export/src/lib.rs:3734`; `rjtd/crates/rjtd-model/src/lib.rs:69658`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-export fdm_bbox_center_handles_extreme_bounds_without_overflow -- --exact && rg -n "rowFanoutSegmentOwnerGate|roleFanoutSegmentOwnerGate|fdm-index-role-row-fanout-multi-command-single-row|rowsWithMultipleCommandRefs" crates/rjtd-export/src/lib.rs crates/rjtd-model/src/lib.rs`
  QA scenarios: Happy: always run the non-skipping fixture/static audit; if `../rjtd-testdata/local-samples/ichitaro-20030228030923-success-002-success_data-test.jtd` and its `.pdf` exist, additionally run `cargo test -p rjtd-export local_success_data_test_exports_embedding_frame_candidates_when_reference_pdf_is_available -- --exact` and require Q5 line-candidate role to report row 40 backing `[1992,2024]` and block as `fdm-index-role-row-fanout-multi-command-single-row` unless a source-backed ownership rule is added. Failure: any one-row-to-many-command promotion without segment ownership proof fails the task; if assets are absent, record the sample-dependent branch as skipped. Evidence `.omo/evidence/current-progress-next-plan/task-13-q4-q5-fanout.log`
  Commit: Y | `feat(fdm): expose row fanout ownership evidence`

- [x] 14. Gate Q4/Q5 paint-order promotion with continuity and authority evidence
  What to do / Must NOT do: Combine todos 12 and 13 with paint-order continuity profiles. Promote only a diagnostic readiness field unless vector offset, row fanout, row order, and paint-order authority are all source-backed. Must not draw additional Q4/Q5 primitives in SVG/PDF before all gates agree.
  Parallelization: Wave 3 | Blocked by: 12, 13 | Blocks: 18, 19
  References: `TODO.md:692`; `TODO.md:698`; `TODO.md:699`; `openjtd-spec/rfc/0008-object-stream-candidates.md:199`; `openjtd-spec/rfc/0008-object-stream-candidates.md:203`; `rjtd/crates/rjtd-export/src/lib.rs:3892`; `rjtd/crates/rjtd-export/src/lib.rs:3942`; `rjtd/crates/rjtd-model/src/lib.rs:69658`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-export fdm_bbox_center_handles_extreme_bounds_without_overflow -- --exact && cargo test --workspace && rg -n "paintOrderContinuityProfile|role-span-interleaved-non-role-commands|role-paint-order-authority-unproven|primitiveOwnershipAdmissionGate" crates/rjtd-export/src/lib.rs crates/rjtd-model/src/lib.rs`
  QA scenarios: Happy: always run the non-skipping fixture/static audit and workspace tests; if `../rjtd-testdata/local-samples/ichitaro-20030228030923-success-002-success_data-test.jtd` and its `.pdf` exist, additionally run `cargo test -p rjtd-export local_success_data_test_exports_embedding_frame_candidates_when_reference_pdf_is_available -- --exact` and require contiguous roles to become at most `paintOrderAuthorityPending`, interleaved roles to stay `role-span-interleaved-non-role-commands`, and top-level admission to remain non-rendering unless all blockers clear. Failure: any visible render diff without all blockers clearing is reverted within this todo; if assets are absent, record the sample-dependent branch as skipped. Evidence `.omo/evidence/current-progress-next-plan/task-14-q4-q5-paint-order.log`
  Commit: Y | `feat(fdm): gate primitive ownership by paint order`

- [x] 15. Extend `shanai_lan` page-origin evidence without relying on reference-backed grid origin
  What to do / Must NOT do: Investigate `/LineMark`, `/PageMark`, and line-header relations for a source-derived page-space origin candidate. Keep `gridOriginAuthorityGate.referenceBacked:true` and render blockers if the candidate still depends on reference pixels.
  Parallelization: Wave 4 | Blocked by: 7 | Blocks: 17
  References: `TODO.md:716`; `openjtd-spec/rfc/0008-object-stream-candidates.md:167`; `openjtd-spec/rfc/0008-object-stream-candidates.md:173`; `rjtd/crates/rjtd-model/src/lib.rs:56316`; `rjtd/crates/rjtd-model/src/lib.rs:56501`; `rjtd/crates/rjtd-model/src/lib.rs:56630`; `rjtd/crates/rjtd-model/src/lib.rs:60086`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model shanai_lan_line_mark_intervals_use_positive_deltas_after_header -- --exact && cargo test -p rjtd-model shanai_lan_line_mark_profile_distinguishes_observed_payload_families -- --exact`
  QA scenarios: Happy: new or existing diagnostics separate row-domain evidence from page-space origin authority. Failure: if the only candidate is a near-reference residual, record it as `referenceBacked:true` and keep render admission blocked. Evidence `.omo/evidence/current-progress-next-plan/task-15-shanai-origin.log`
  Commit: Y | `feat(shanai-lan): separate page origin evidence`

- [x] 16. Extend `shanai_lan` endpoint ownership and connector source-order evidence
  What to do / Must NOT do: Strengthen same-row FDM open-stroke axis-rule connector diagnostics so endpoint owner, connector parent, axis-rule parent, and image-bearing segment relations are explicit. Must not draw connector lines.
  Parallelization: Wave 4 | Blocked by: 7 | Blocks: 17
  References: `TODO.md:701`; `TODO.md:707`; `TODO.md:709`; `openjtd-spec/rfc/0008-object-stream-candidates.md:163`; `rjtd/crates/rjtd-model/src/lib.rs:53952`; `rjtd/crates/rjtd-model/src/lib.rs:54949`; `rjtd/crates/rjtd-model/src/lib.rs:57338`; `rjtd/crates/rjtd-model/src/lib.rs:57690`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model fdm_connector_line_rule_endpoint_matches_horizontal_tight_span -- --exact && cargo test -p rjtd-model fdm_connector_line_rule_endpoint_match_summary_blocks_single_endpoint -- --exact && cargo test -p rjtd-model fdm_connector_parent_normalized_order_requires_parent_relative_offset_between_nearest_owner_parents -- --exact && cargo test -p rjtd-model fdm_axis_rule_source_order_gate_classifies_parent_relative_offset_spans -- --exact`
  QA scenarios: Happy: diagnostics expose dual-endpoint/source-order counts and still block rendering when ownership or paint order is absent. Failure: if a straight-line probe improves one count but worsens mean/RMS or lacks ownership proof, record rejection and do not promote. Evidence `.omo/evidence/current-progress-next-plan/task-16-shanai-connectors.log`
  Commit: Y | `feat(shanai-lan): narrow connector ownership evidence`

- [x] 17. Keep `shanai_lan` line-rule render admission diagnostic-only unless origin, endpoints, style, and paint order are proven together
  What to do / Must NOT do: Update `lineRuleRenderAdmissionGate` only to make blockers more specific or prove a complete source-backed admission. Must not render a single visually tempting rule in isolation.
  Parallelization: Wave 4 | Blocked by: 15, 16 | Blocks: 18, 19
  References: `TODO.md:713`; `TODO.md:716`; `openjtd-spec/rfc/0008-object-stream-candidates.md:169`; `rjtd/crates/rjtd-model/src/lib.rs:57397`; `rjtd/crates/rjtd-model/src/lib.rs:57453`; `rjtd/crates/rjtd-model/src/lib.rs:57489`; `rjtd/crates/rjtd-model/src/lib.rs:57749`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model document_core_projects_shanai_lan_fdm_frame_diagnostics -- --exact && cargo test --workspace`
  QA scenarios: Happy: `lineRuleRenderAdmissionGate.promotionReady:false` remains unless all required fields prove source-backed authority; SVG/PDF output is unchanged unless all gates clear. Failure: if a line-rule render probe worsens Poppler/PDFKit/CoreGraphics or lacks source proof, preserve it only as rejected evidence in TODO/RFC. Evidence `.omo/evidence/current-progress-next-plan/task-17-shanai-line-rule.log`
  Commit: Y | `feat(shanai-lan): gate line-rule rendering`

- [ ] 18. Run full Rust verification and capture exact failures before any handoff
  What to do / Must NOT do: Run the complete workspace verification suite after all implementation todos. If failures appear, fix only failures introduced by this plan; record pre-existing or environment-only gaps separately.
  Parallelization: Wave 5 | Blocked by: 1-17 | Blocks: 19
  References: `README.md:30`; `TODO.md:32`; `rjtd/Cargo.toml:1`; `rjtd/crates/rjtd-cli/tests/streams.rs:4877`; `rjtd/crates/rjtd-cli/tests/streams.rs:4958`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo fmt --all --check && cargo check --workspace && cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`
  QA scenarios: Happy: all commands exit 0 and logs are stored. Failure: each failing command gets a copied stderr excerpt, root-cause note, and follow-up fix in the same task until green or proven pre-existing/environmental. Evidence `.omo/evidence/current-progress-next-plan/task-18-full-rust.log`
  Commit: N | verification only

- [ ] 19. Run local sample/PDF visual QA when assets and tools are present
  What to do / Must NOT do: If local samples, reference PDFs, and tools exist, run the exact inspect-only command below. It covers `success_data-test`, `tsaiten`, and `shanai_lan`; writes explicit generated/reference PDF and PNG evidence fields; and exits with a recorded skip instead of claiming visual coverage when required assets or tools are missing. Must not claim pixel equivalence.
  Current preflight note: PDF export and page-1 comparison evidence was generated out of order on 2026-07-01 after task 11 was confirmed blocked. Treat `.omo/evidence/current-progress-next-plan/task-19-visual-qa.md`, `.omo/evidence/current-progress-next-plan/task-19-visual-compare.md`, and `.omo/evidence/current-progress-next-plan/task-19-compare/task-19-page1-comparison-contact-sheet.png` as preflight evidence only. They do not complete this checkbox, do not unblock 20/F1-F4, and must be rerun or explicitly refreshed after 11 and 18 are complete.
  Exact visual QA command:
  ```bash
  set -eu
  cd /Users/kimuj5090/Documents/rjtd
  evidence=".omo/evidence/current-progress-next-plan"
  page_dir="$evidence/task-19-pages"
  report="$evidence/task-19-visual-qa.md"
  mkdir -p "$page_dir" openjtd-samples/pdf-output
  : > "$report"

  for tool in pdfinfo pdftoppm sips; do
    if ! command -v "$tool" >/dev/null 2>&1; then
      printf 'visual QA skipped: missing tool %s\n' "$tool" >> "$report"
      exit 0
    fi
  done

  cargo build --manifest-path rjtd/Cargo.toml -p rjtd-cli

  for stem in \
    ichitaro-20030228030923-success-002-success_data-test \
    ichitaro-20030120132956-0007-sp-dat-tsaiten \
    ichitaro-20030315134715-success-001-success_data-shanai_lan
  do
    sample="rjtd-testdata/local-samples/${stem}.jtd"
    reference="rjtd-testdata/local-samples/${stem}.pdf"
    generated="openjtd-samples/pdf-output/${stem}.pdf"

    if [ ! -f "$sample" ] || [ ! -f "$reference" ]; then
      printf 'visual QA skipped: missing asset for %s sample=%s reference=%s\n' "$stem" "$sample" "$reference" >> "$report"
      continue
    fi

    rjtd/target/debug/rjtd export "$sample" --format pdf -o "$generated"

    generated_info="$evidence/task-19-${stem}-generated.pdfinfo"
    reference_info="$evidence/task-19-${stem}-reference.pdfinfo"
    pdfinfo "$generated" > "$generated_info"
    pdfinfo "$reference" > "$reference_info"

    generated_prefix="$page_dir/${stem}-generated"
    reference_prefix="$page_dir/${stem}-reference"
    pdftoppm -png -f 1 -l 1 "$generated" "$generated_prefix"
    pdftoppm -png -f 1 -l 1 "$reference" "$reference_prefix"
    generated_png="${generated_prefix}-1.png"
    reference_png="${reference_prefix}-1.png"

    [ -s "$generated_png" ] || { printf 'visual QA failed: generated page 1 PNG missing or empty for %s\n' "$stem" >> "$report"; exit 1; }
    [ -s "$reference_png" ] || { printf 'visual QA failed: reference page 1 PNG missing or empty for %s\n' "$stem" >> "$report"; exit 1; }

    generated_page_count="$(awk -F: '/^Pages:/ {gsub(/^[ \t]+/, "", $2); print $2; exit}' "$generated_info")"
    reference_page_count="$(awk -F: '/^Pages:/ {gsub(/^[ \t]+/, "", $2); print $2; exit}' "$reference_info")"
    generated_page_size="$(awk -F: '/^Page size:/ {gsub(/^[ \t]+/, "", $2); print $2; exit}' "$generated_info")"
    reference_page_size="$(awk -F: '/^Page size:/ {gsub(/^[ \t]+/, "", $2); print $2; exit}' "$reference_info")"

    if [ "$generated_page_count" != "$reference_page_count" ]; then
      printf 'visual QA failed: page count mismatch for %s generated=%s reference=%s\n' "$stem" "$generated_page_count" "$reference_page_count" >> "$report"
      exit 1
    fi

    {
      printf '## %s\n' "$stem"
      printf 'generated_pdf: %s\n' "$generated"
      printf 'reference_pdf: %s\n' "$reference"
      printf 'generated_page_count: %s\n' "$generated_page_count"
      printf 'reference_page_count: %s\n' "$reference_page_count"
      printf 'generated_page_size: %s\n' "$generated_page_size"
      printf 'reference_page_size: %s\n' "$reference_page_size"
      printf 'generated_page1_png: %s\n' "$generated_png"
      printf 'reference_page1_png: %s\n' "$reference_png"
      printf 'generated_png_bytes: '
      wc -c < "$generated_png"
      printf 'reference_png_bytes: '
      wc -c < "$reference_png"
      printf 'generated_png_dimensions:\n'
      sips -g pixelWidth -g pixelHeight "$generated_png"
      printf 'reference_png_dimensions:\n'
      sips -g pixelWidth -g pixelHeight "$reference_png"
      printf '\n'
    } >> "$report"
  done

  rg -n "visual QA skipped|generated_pdf|reference_pdf|generated_page_count|reference_page_count|generated_page_size|reference_page_size|generated_page1_png|reference_page1_png" "$report"
  ```
  Parallelization: Wave 5 | Blocked by: 5-18 | Blocks: 20
  References: `README.md:39`; `TODO.md:705`; `TODO.md:713`; `TODO.md:715`; `rjtd/crates/rjtd-export/src/lib.rs:8079`; `rjtd/crates/rjtd-model/src/lib.rs:82706`; `rjtd/crates/rjtd-model/src/lib.rs:84951`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/task-19-visual-qa.md && test -s .omo/evidence/current-progress-next-plan/task-19-visual-compare.md && test -s .omo/evidence/current-progress-next-plan/task-19-compare/task-19-page1-comparison-contact-sheet.png && rg -n "success_data-test|tsaiten|shanai_lan|generated_pdf|reference_pdf|generated_page_count|reference_page_count|generated_page_size|reference_page_size|generated_page1_png|reference_page1_png|skipped|missing" .omo/evidence/current-progress-next-plan/task-19-visual-qa.md && rg -n "mean abs channel diff|Visual notes|shanai_lan|connector lines|does not establish pixel equivalence" .omo/evidence/current-progress-next-plan/task-19-visual-compare.md`
  QA scenarios: Happy: for each representative stem, `task-19-visual-qa.md` records `generated_pdf`, `reference_pdf`, `generated_page_count`, `reference_page_count`, `generated_page_size`, `reference_page_size`, `generated_page1_png`, `reference_page1_png`, generated/reference PNG dimensions, and byte sizes; `task-19-visual-compare.md` records page-1 diff metrics and links the contact sheet. Failure: if a tool is absent, record `visual QA skipped: missing tool pdfinfo`, `visual QA skipped: missing tool pdftoppm`, or `visual QA skipped: missing tool sips`; if a sample/reference asset is absent, record a concrete message such as `visual QA skipped: missing asset for ichitaro-20030120132956-0007-sp-dat-tsaiten sample=rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd reference=rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.pdf`. Do not claim visual coverage; if any generated PDF has page count 0, missing page size, missing page 1 PNG, zero-byte PNG, different page count from reference, or a comparison note names a material visual gap such as missing `shanai_lan` connector lines, carry that as a remaining risk and return to the relevant implementation todo before handoff. Evidence `.omo/evidence/current-progress-next-plan/task-19-visual-qa.md`; `.omo/evidence/current-progress-next-plan/task-19-visual-compare.md`
  Commit: N | generated visual evidence is not committed unless requested

- [ ] 20. Prepare final handoff, dirty-tree summary, and Lore commit recommendation
  What to do / Must NOT do: Summarize changed files, evidence logs, remaining risks, and suggested atomic commit boundaries. Include the task 19 preflight-vs-formal distinction and carry the `shanai_lan` connector-line visual gap from `task-19-visual-compare.md` as a remaining risk until a later gated task proves it resolved. Do not stage or commit unless the user explicitly asks. If committing later, use the Lore Commit Protocol with `Constraint`, `Rejected`, `Confidence`, `Scope-risk`, `Directive`, `Tested`, and `Not-tested` trailers as useful.
  Parallelization: Wave 5 | Blocked by: 1-19 | Blocks: F1-F4
  References: `AGENTS.md`; `.omo/drafts/current-progress-next-plan.md`; `.omo/plans/current-progress-next-plan.md`; `docs/ARCHITECTURE.md:27`; `docs/RHWP-COMPATIBILITY.md:30`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/task-20-handoff.md && rg -n "Changed files|Verification|Remaining risks|Commit recommendation|Not-tested|task-19-visual-compare|preflight|shanai_lan connector" .omo/evidence/current-progress-next-plan/task-20-handoff.md`
  QA scenarios: Happy: handoff names every changed tracked file, every new artifact, exact verification commands, remaining risks, the task 19 comparison/contact-sheet artifacts, and whether visual QA was formal or preflight. Failure: if `git status --short` includes unexpected unrelated files, classify them and keep them out of commit recommendations; if preflight visual evidence exists while task 19 remains unchecked, state that it is not completion evidence. Evidence `.omo/evidence/current-progress-next-plan/task-20-handoff.md`
  Commit: N | handoff only unless user asks to commit

## Final verification wave
> Runs after todo 20. All final checks must be agent-verifiable and recorded before handoff.
- [ ] F1. Plan compliance audit
  What to do / Must NOT do: Verify every Must Have/Must NOT Have item maps to completed todos and evidence. Must not count unchecked todos, skipped sample tests, or missing evidence as completion.
  References: `.omo/plans/current-progress-next-plan.md`; `.omo/evidence/current-progress-next-plan/`; `docs/ARCHITECTURE.md:27`; `docs/RHWP-COMPATIBILITY.md:30`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/f1-plan-compliance.log && rg -n "Must have|Must NOT|evidence present|no unchecked required evidence" .omo/evidence/current-progress-next-plan/f1-plan-compliance.log`
  QA scenarios: Happy: create a compliance matrix showing every scope item and the evidence file that proves it. Failure: any missing evidence or skipped sample claim fails F1 and returns to the owning todo. Evidence `.omo/evidence/current-progress-next-plan/f1-plan-compliance.log`
  Commit: N | final verification only
- [ ] F2. Code quality review
  What to do / Must NOT do: Review the final diff for boundary violations: exporter raw stream access, `rhwp/` copying, weakened tests, missing docs parity, and accidental visible rendering promotion. Block handoff if any diagnostic-only guardrail is weakened without source-backed proof.
  References: `docs/ARCHITECTURE.md:27`; `docs/RHWP-COMPATIBILITY.md:30`; `TODO.md:686`; `TODO.md:691`; `TODO.md:701`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/f2-code-quality.md && rg -n "raw stream access|rhwp|test weakening|render promotion|docs parity" .omo/evidence/current-progress-next-plan/f2-code-quality.md`
  QA scenarios: Happy: diff review finds no boundary violations and records checked files. Failure: any raw exporter scan, copied `rhwp` code, weakened assertion, or unproven render promotion blocks handoff. Evidence `.omo/evidence/current-progress-next-plan/f2-code-quality.md`
  Commit: N | final verification only
- [ ] F3. Real manual QA
  What to do / Must NOT do: Run agent-executed manual QA: final Rust suite plus available local sample/PDF checks. Must not claim local-sample or visual coverage when assets/tools are missing, and must not treat preflight task 19 evidence as final manual QA unless it was rerun or explicitly refreshed after 11 and 18 completed.
  References: `README.md:30`; `README.md:39`; `.omo/evidence/current-progress-next-plan/task-18-full-rust.log`; `.omo/evidence/current-progress-next-plan/task-19-visual-qa.md`; `.omo/evidence/current-progress-next-plan/task-19-visual-compare.md`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/f3-manual-qa.md && rg -n "cargo fmt|cargo check|cargo test|cargo clippy|visual QA|skipped|task-19-visual-compare|preflight|shanai_lan connector" .omo/evidence/current-progress-next-plan/f3-manual-qa.md`
  QA scenarios: Happy: Rust suite is green and visual/sample checks are either executed with evidence or explicitly skipped with missing asset/tool names; if task 19 visual evidence is reused, F3 states whether it is preflight or refreshed final evidence. Failure: any failing command, unqualified visual claim, missing comparison artifact, or unreported `shanai_lan` connector-line visual gap blocks handoff. Evidence `.omo/evidence/current-progress-next-plan/f3-manual-qa.md`
  Commit: N | final verification only
- [ ] F4. Scope fidelity
  What to do / Must NOT do: Confirm no product code was touched outside named todo scopes, no generated artifacts were committed unintentionally, and all remaining risks are explicitly reported. Include task 19 preflight PDFs/PNGs/contact sheets in generated-artifact classification and keep the unchecked formal task status visible. Must not hide unrelated dirty worktree entries.
  References: `.omo/evidence/current-progress-next-plan/task-1-current-state.md`; `.omo/evidence/current-progress-next-plan/task-20-handoff.md`; `AGENTS.md`
  Acceptance criteria: `cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/f4-scope-fidelity.md && rg -n "dirty worktree|out of scope|generated artifacts|remaining risks|task-19-compare|openjtd-samples/pdf-output|unchecked" .omo/evidence/current-progress-next-plan/f4-scope-fidelity.md`
  QA scenarios: Happy: final status is classified by tracked WIP, planning artifacts, generated artifacts, unchecked formal tasks, and remaining risks. Failure: any unexpected file is classified before handoff; generated artifacts are not staged unless explicitly requested; preflight task 19 artifacts are not used to imply the checkbox is complete. Evidence `.omo/evidence/current-progress-next-plan/f4-scope-fidelity.md`
  Commit: N | final verification only

## Commit strategy
- Default: no automatic commit. Stop after verified handoff unless the user asks to stage/commit.
- If the user asks for one commit, use one atomic commit only after the final verification wave is green.
- Suggested commit intent line: `Preserve source-backed JTD diagnostic gates before rendering promotion`
- Suggested body: explain that the change stabilizes model-owned diagnostic evidence and narrows source-only admission gates before any visible rendering promotion.
- Suggested trailers:
  - `Constraint: Exporters must consume model-owned evidence and must not scan raw streams directly`
  - `Constraint: rhwp is read-only reference material`
  - `Rejected: Promote reference-backed tsaiten calibration to source layout | source-only page-y transform remains unproven`
  - `Rejected: Draw Q4/Q5 role candidates from bbox.left references | vector-offset authority, fanout, and paint order remain unproven`
  - `Rejected: Draw shanai_lan connector/line-rule probes from local visual improvement | endpoint ownership and page origin remain unproven`
  - `Confidence: medium`
  - `Scope-risk: broad`
  - `Directive: Do not remove decoded-false blockers without source-backed admission tests`
  - `Tested: cargo fmt --all --check; cargo check --workspace; cargo test --workspace; cargo clippy --workspace --all-targets -- -D warnings; focused CLI/model/export tests; local sample/PDF QA when available`
  - `Not-tested: Any local sample/PDF branch skipped because local assets or PDF tools were unavailable`

## Success criteria
- `.omo/evidence/current-progress-next-plan/` contains task evidence for todos 1-20 and final verification receipts F1-F4.
- `cargo fmt --all --check`, `cargo check --workspace`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` pass from `/Users/kimuj5090/Documents/rjtd/rjtd`, or any non-pass is proven pre-existing/environmental with exact evidence.
- Current object/FDM/image diagnostics remain decoded-false and non-rendering unless a todo proves page geometry and paint order from source-backed gates.
- `tsaiten` visible reference fallback is removed only when both visible table families satisfy the same source-only admission contract; otherwise all visible `tsaiten` rendering stays reference-fallback with explicit source replacement blockers.
- Q4/Q5 primitive ownership remains non-rendering unless vector offset, row fanout, row order, and paint order all have source-backed proof.
- `shanai_lan` connector and line-rule rendering remains blocked unless page origin, endpoint ownership, style role, and paint order are proven together.
- TODO/RFC English and Japanese files agree with code/test field names and blocker strings.
- Final handoff names changed files, simplifications or gate narrowings made, remaining risks, and exact verification evidence.
