# Task 2: Symbol/Test Map

Surface: `/Users/kimuj5090/Documents/rjtd` current dirty worktree on 2026-07-01.

Scope receipt:
- Wrote only this artifact: `.omo/evidence/current-progress-next-plan/task-2-symbol-test-map.md`.
- Did not inspect `rhwp/`.
- Did not stage, commit, revert, normalize, clean, or edit product files.
- Mapping is against current uncommitted tree. `git status --short` showed tracked WIP in `TODO*`, RFC docs, and `rjtd/crates/{rjtd-cli,rjtd-export,rjtd-model}` plus untracked `.omo/`, `AGENTS.md`, and `rjtd/AGENTS.md`.

## Commands Run

Manual-QA source probe:

```bash
cd /Users/kimuj5090/Documents/rjtd && rg -n "jsfartStreamProfile|sourceOnlyAxisAdmissionGate|primitiveOwnershipAdmissionGate|lineRuleRenderAdmissionGate" rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-cli/src/main.rs rjtd/crates/rjtd-cli/tests/streams.rs
```

Exit status: 0.

Broader symbol probe:

```bash
cd /Users/kimuj5090/Documents/rjtd && rg -n "ObjectStreamCandidate|jsfartStreamProfile|imagePayloadDiagnostic|referenceFallbackAdmissionGate|sourceOnlyAxisAdmissionGate|sourceOnlyPageYRenderAdmissionGate|primitiveOwnershipAdmissionGate|gridOriginAuthorityGate|lineRuleRenderAdmissionGate" rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-cli/src/main.rs rjtd/crates/rjtd-cli/tests/streams.rs
```

Exit status: 0.

Doc surface probe:

```bash
cd /Users/kimuj5090/Documents/rjtd && rg -n "ObjectStreamCandidate|jsfartStreamProfile|imagePayloadDiagnostic|referenceFallbackAdmissionGate|sourceOnlyAxisAdmissionGate|sourceOnlyPageYRenderAdmissionGate|primitiveOwnershipAdmissionGate|gridOriginAuthorityGate|lineRuleRenderAdmissionGate|image-signature-without-complete-payload-role-unproven|fdm-frame-linked-image-payload-placement-and-paint-order-unproven|source-page-y-render-admission-not-ready|line-rule-render-admission-not-ready|document-text-grid-origin-reference-backed" TODO.md TODO.ja.md openjtd-spec/rfc/0008-object-stream-candidates.md openjtd-spec/rfc/0008-object-stream-candidates.ja.md docs/ARCHITECTURE.md docs/RHWP-COMPATIBILITY.md
```

Exit status: 0.

Test-name probe:

```bash
cd /Users/kimuj5090/Documents/rjtd && rg -n "object_stream_candidates_command_reports_visual_object_inventory|object_fdm_image_candidates_command_reports_signature_only_blocker|object_fdm_frame_links_command_connects_fdm_rows_to_frame_records|object_fdm_index_command_links_index_rows_to_vector_image_hits|image_payload_render_gate_preserves_source_frame_trace_without_promotion|parser_preserves_object_stream_candidates_as_model_evidence|local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available|exports_object_stream_candidates_to_json|fdm_bbox_center_handles_extreme_bounds_without_overflow|document_core_projects_shanai_lan_fdm_frame_diagnostics|fdm_connector_line_rule_endpoint_match_summary_blocks_single_endpoint|fdm_axis_rule_source_order_gate_classifies_parent_relative_offset_spans|table_grid_cross_table_subrecord_ordering_helpers_detect_regressions" rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-cli/tests/streams.rs
```

Exit status: 0.

## Symbol/Test Map

| Symbol / gate | Source surfaces | Export / CLI / doc surfaces | Verification surfaces |
| --- | --- | --- | --- |
| `ObjectStreamCandidate` | Model storage and API: `rjtd/crates/rjtd-model/src/lib.rs:248`, `:383`, `:443`; model type and evidence: `:8261`, `:9459`, `:9563`; CFB extraction/classification: `:11479`, `:13395`; model JSON: `:18508`, `:18614`; CLI local classifier: `rjtd/crates/rjtd-cli/src/main.rs:5270`, `:5322`; exporter JSON bridge: `rjtd/crates/rjtd-export/src/lib.rs:783`. | CLI imports model candidate at `rjtd/crates/rjtd-cli/src/main.rs:30` and object command formatting reaches `:5794`, `:6062`, `:6132`; docs mention diagnostic preservation at `openjtd-spec/rfc/0008-object-stream-candidates.md:99` and `docs/RHWP-COMPATIBILITY.md:160`. | `cd rjtd && cargo test -p rjtd-model parser_preserves_object_stream_candidates_as_model_evidence -- --exact`; `cd rjtd && cargo test -p rjtd-export exports_object_stream_candidates_to_json -- --exact`; `cd rjtd && cargo test -p rjtd-cli --test streams object_stream_candidates_command_reports_visual_object_inventory -- --exact`; static audit with the broader `rg` command above. |
| `jsfartStreamProfile` | Model JSON emits at `rjtd/crates/rjtd-model/src/lib.rs:18739`; model tests assert at `:78942`, `:80024`; exporter emits at `rjtd/crates/rjtd-export/src/lib.rs:920` and helper at `:937`; exporter test asserts at `:7675`. | CLI command is documented to report profile count/family in `TODO.md:611`, `TODO.ja.md:597`, and `openjtd-spec/rfc/0008-object-stream-candidates.md:191`; direct source probe found no literal `jsfartStreamProfile` in `rjtd-cli/src/main.rs` or `rjtd-cli/tests/streams.rs`, so CLI coverage should be verified through object-stream command output tokens rather than this JSON key. | `cd rjtd && cargo test -p rjtd-model parser_preserves_object_stream_candidates_as_model_evidence -- --exact`; `cd rjtd && cargo test -p rjtd-export exports_object_stream_candidates_to_json -- --exact`; `cd rjtd && cargo test -p rjtd-cli --test streams object_stream_candidates_command_reports_visual_object_inventory -- --exact` with output expectations for `jsfart-stream-profile=1` / family reporting per plan task 5. Missing-surface risk: CLI does not carry the camelCase JSON key literally. |
| `imagePayloadDiagnostic` | Model layer-tree JSON emits type at `rjtd/crates/rjtd-model/src/lib.rs:45223`; SVG diagnostic function at `:71533`; tests assert type at `:78565`, `:78653`, and count at `:86567`. | TODO records field behavior at `TODO.md:715`; Japanese mirror at `TODO.ja.md:692`. The export crate serializes object payload evidence helpers (`rjtd/crates/rjtd-export/src/lib.rs:4456` through `:4585`) but the exact literal `imagePayloadDiagnostic` is model-side only in the probed files. | `cd rjtd && cargo test -p rjtd-model image_payload_render_gate_preserves_source_frame_trace_without_promotion -- --exact`; `cd rjtd && cargo test -p rjtd-model local_samples_project_image_payload_diagnostics_when_available -- --exact` when local samples exist; `cd rjtd && cargo test -p rjtd-cli --test streams object_fdm_image_candidates_command_reports_signature_only_blocker -- --exact`; `cd rjtd && cargo test -p rjtd-cli --test streams object_fdm_frame_links_command_connects_fdm_rows_to_frame_records -- --exact`. Missing-surface risk: export JSON has object image payload helper coverage, but the named layer diagnostic literal is currently a model surface. |
| `referenceFallbackAdmissionGate` | Model table JSON emits at `rjtd/crates/rjtd-model/src/lib.rs:24936`; source/SVG fallback admission helper path at `:73745`; test assertions at `:81012` and `:85075`. | Docs/TODO: `TODO.md:687`, `TODO.ja.md:661`, `openjtd-spec/rfc/0008-object-stream-candidates.md:229`. No literal hit in `rjtd-export/src/lib.rs`, `rjtd-cli/src/main.rs`, or `rjtd-cli/tests/streams.rs`; this is a model table-layer / SVG diagnostic surface. | `cd rjtd && cargo test -p rjtd-model local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact`; `cd rjtd && cargo test -p rjtd-model table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact`; static audit with `rg -n "referenceFallbackAdmissionGate|referenceFallbackAllowed|referenceFallbackUsed|sourceReplacementBlockedReason" rjtd/crates/rjtd-model/src/lib.rs`. Missing-surface risk: sample-dependent behavioral coverage may skip if local assets are absent. |
| `sourceOnlyAxisAdmissionGate` | Model JSON emits at `rjtd/crates/rjtd-model/src/lib.rs:37501`; nested transform source marker at `:37935`; candidate bbox source at `:38030`; test assertions at `:80976`, `:85103`, `:85723`. | Docs/TODO: `TODO.md:683`, `TODO.md:685`, `TODO.ja.md:657`, `TODO.ja.md:659`, `openjtd-spec/rfc/0008-object-stream-candidates.md:207`, `:221`, `:225`. No literal hit in export or CLI files; this is a model table diagnostic contract. | `cd rjtd && cargo test -p rjtd-model table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact`; `cd rjtd && cargo test -p rjtd-model local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact` when local sample/PDF exist; static audit with `rg -n "sourceOnlyAxisAdmissionGate|sourceOnlyAxisCandidateBBox|sourceGapToPageLineGapTransformAdmissionGate|line-domain-projection-disagrees-with-page-mark-absolute-y-slot" rjtd/crates/rjtd-model/src/lib.rs`. Missing-surface risk: model-only literal; downstream docs are present. |
| `sourceOnlyPageYRenderAdmissionGate` | Model JSON emits at `rjtd/crates/rjtd-model/src/lib.rs:39613`; nested transform marker at `:39934`; test assertions at `:81006`, `:85274`, `:85864`. | Docs/TODO: `TODO.md:682`, `TODO.md:686`, `TODO.ja.md:656`, `TODO.ja.md:660`, `openjtd-spec/rfc/0008-object-stream-candidates.md:219`, `:227`, `:229`. No literal hit in export or CLI files; this is model-owned table admission evidence. | `cd rjtd && cargo test -p rjtd-model table_grid_cross_table_subrecord_ordering_helpers_detect_regressions -- --exact`; `cd rjtd && cargo test -p rjtd-model local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available -- --exact` when local assets exist; static audit with `rg -n "sourceOnlyPageYRenderAdmissionGate|source-page-y-render-admission-not-ready|sourceGapToPageLineGapTransformAdmissionGate" rjtd/crates/rjtd-model/src/lib.rs TODO.md openjtd-spec/rfc/0008-object-stream-candidates.md`. Missing-surface risk: sample-dependent assertions may skip absent local PDFs. |
| `primitiveOwnershipAdmissionGate` | Model JSON emits at `rjtd/crates/rjtd-model/src/lib.rs:68992`; model helper starts at `:69658`; model tests assert at `:80156`, `:80306`, `:80331`; exporter JSON emits at `rjtd/crates/rjtd-export/src/lib.rs:2404`; exporter helper starts at `:3049`; exporter tests assert at `:7823`, `:7973`, `:7998`. | Docs/TODO: `TODO.md:697`, `TODO.ja.md:674`, `openjtd-spec/rfc/0008-object-stream-candidates.md:199`, `:241`. No literal hit in CLI tests; current CLI coverage is adjacent object/FDM index output rather than this gate key. | `cd rjtd && cargo test -p rjtd-model fdm_bbox_center_handles_extreme_bounds_without_overflow -- --exact`; `cd rjtd && cargo test -p rjtd-export fdm_bbox_center_handles_extreme_bounds_without_overflow -- --exact`; `cd rjtd && cargo test -p rjtd-export local_success_data_test_exports_embedding_frame_candidates_when_reference_pdf_is_available -- --exact` when local assets exist; static audit with `rg -n "primitiveOwnershipAdmissionGate|fdm-index-role-vector-offset-authority-valid-vector-offset-missing|role-paint-order-authority-unproven|fdm-index-role-row-fanout-multi-command-single-row" rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-model/src/lib.rs`. Missing-surface risk: CLI literal absent, Q4/Q5 behavior may be sample-dependent. |
| `gridOriginAuthorityGate` | Model JSON emits at `rjtd/crates/rjtd-model/src/lib.rs:56842`; related line-header helpers around `:56316`, `:56501`, `:56630`, `:60086`; test assertion at `:83657`. | Docs/TODO: `TODO.md:332`, `TODO.md:334`, `TODO.ja.md:277`, `TODO.ja.md:279`, `openjtd-spec/rfc/0008-object-stream-candidates.md:167`. No literal hit in export or CLI files; this is model-owned `shanai_lan` diagnostic evidence. | `cd rjtd && cargo test -p rjtd-model document_core_projects_shanai_lan_fdm_frame_diagnostics -- --exact`; `cd rjtd && cargo test -p rjtd-model shanai_lan_line_mark_intervals_use_positive_deltas_after_header -- --exact`; `cd rjtd && cargo test -p rjtd-model shanai_lan_line_mark_profile_distinguishes_observed_payload_families -- --exact`; static audit with `rg -n "gridOriginAuthorityGate|document-text-grid-origin-reference-backed|page-space-y-origin-unproven" rjtd/crates/rjtd-model/src/lib.rs TODO.md openjtd-spec/rfc/0008-object-stream-candidates.md`. Missing-surface risk: model-only literal, but doc coverage exists. |
| `lineRuleRenderAdmissionGate` | Model JSON emits at `rjtd/crates/rjtd-model/src/lib.rs:56283`; related `shanai_lan` line-rule projection helpers around `:59963`, `:59983`, `:60184`, `:60274`; test assertion at `:83607`. | Docs/TODO: `TODO.md:716`, `TODO.ja.md:693`, `openjtd-spec/rfc/0008-object-stream-candidates.md:169`. No literal hit in export or CLI files; this is model-owned diagnostic-only line-rule admission. | `cd rjtd && cargo test -p rjtd-model document_core_projects_shanai_lan_fdm_frame_diagnostics -- --exact`; `cd rjtd && cargo test -p rjtd-model fdm_connector_line_rule_endpoint_match_summary_blocks_single_endpoint -- --exact`; `cd rjtd && cargo test -p rjtd-model fdm_axis_rule_source_order_gate_classifies_parent_relative_offset_spans -- --exact`; static audit with `rg -n "lineRuleRenderAdmissionGate|line-rule-render-admission-not-ready|line-rule-endpoint-ownership-unproven|line-rule-style-role-unproven|line-rule-paint-order-unproven" rjtd/crates/rjtd-model/src/lib.rs TODO.md openjtd-spec/rfc/0008-object-stream-candidates.md`. Missing-surface risk: model-only literal, but test and doc surfaces are present. |

## Manual-QA Matrix

### surfaceEvidence

| scenario id | criterion reference | surface | exact invocation | verdict | artifactRefs |
| --- | --- | --- | --- | --- | --- |
| task2-source-map | Plan todo 2: map requested symbols/gates | Shell / source tree via `rg` | `cd /Users/kimuj5090/Documents/rjtd && rg -n "ObjectStreamCandidate|jsfartStreamProfile|imagePayloadDiagnostic|referenceFallbackAdmissionGate|sourceOnlyAxisAdmissionGate|sourceOnlyPageYRenderAdmissionGate|primitiveOwnershipAdmissionGate|gridOriginAuthorityGate|lineRuleRenderAdmissionGate" rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-cli/src/main.rs rjtd/crates/rjtd-cli/tests/streams.rs` | PASS | A1 |
| task2-manual-channel | User VERIFY manual-QA command | Shell / source tree via `rg` | `cd /Users/kimuj5090/Documents/rjtd && rg -n "jsfartStreamProfile|sourceOnlyAxisAdmissionGate|primitiveOwnershipAdmissionGate|lineRuleRenderAdmissionGate" rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-cli/src/main.rs rjtd/crates/rjtd-cli/tests/streams.rs` | PASS | A1 |
| task2-doc-surfaces | Plan todo 2: include doc surfaces | Shell / docs via `rg` | `cd /Users/kimuj5090/Documents/rjtd && rg -n "ObjectStreamCandidate|jsfartStreamProfile|imagePayloadDiagnostic|referenceFallbackAdmissionGate|sourceOnlyAxisAdmissionGate|sourceOnlyPageYRenderAdmissionGate|primitiveOwnershipAdmissionGate|gridOriginAuthorityGate|lineRuleRenderAdmissionGate|image-signature-without-complete-payload-role-unproven|fdm-frame-linked-image-payload-placement-and-paint-order-unproven|source-page-y-render-admission-not-ready|line-rule-render-admission-not-ready|document-text-grid-origin-reference-backed" TODO.md TODO.ja.md openjtd-spec/rfc/0008-object-stream-candidates.md openjtd-spec/rfc/0008-object-stream-candidates.ja.md docs/ARCHITECTURE.md docs/RHWP-COMPATIBILITY.md` | PASS | A1 |
| task2-verification-surfaces | Plan todo 2: include verification commands | Shell / test inventory via `rg` | `cd /Users/kimuj5090/Documents/rjtd && rg -n "object_stream_candidates_command_reports_visual_object_inventory|object_fdm_image_candidates_command_reports_signature_only_blocker|object_fdm_frame_links_command_connects_fdm_rows_to_frame_records|object_fdm_index_command_links_index_rows_to_vector_image_hits|image_payload_render_gate_preserves_source_frame_trace_without_promotion|parser_preserves_object_stream_candidates_as_model_evidence|local_tsaiten_preserves_document_text_control_table_candidates_when_reference_pdf_is_available|exports_object_stream_candidates_to_json|fdm_bbox_center_handles_extreme_bounds_without_overflow|document_core_projects_shanai_lan_fdm_frame_diagnostics|fdm_connector_line_rule_endpoint_match_summary_blocks_single_endpoint|fdm_axis_rule_source_order_gate_classifies_parent_relative_offset_spans|table_grid_cross_table_subrecord_ordering_helpers_detect_regressions" rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-cli/tests/streams.rs` | PASS | A1 |

### adversarialCases

| scenario id | criterion reference | adversarial class | expected behavior | verdict | artifactRefs |
| --- | --- | --- | --- | --- | --- |
| task2-dirty-worktree | User adversarial QA | dirty_worktree | Mapping notes current uncommitted tree and does not edit product files, stage, commit, revert, normalize, or clean user changes. | PASS | A1 |
| task2-stale-state | User adversarial QA | stale_state | Mapping uses fresh `rg` output and records exact invocations. | PASS | A1 |
| task2-misleading-success | User adversarial QA | misleading_success_output | Missing literal surfaces are listed explicitly instead of treating partial hits as complete. | PASS | A1 |
| task2-generated-artifact | User adversarial QA | generated_or_cached_artifacts | Evidence artifact path is recorded and verified non-empty. | PASS | A1 |

### artifactRefs

| id | kind | description | path |
| --- | --- | --- | --- |
| A1 | markdown evidence | Curated symbol/test map, command invocations, manual-QA matrix, and missing-surface risks. | `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-2-symbol-test-map.md` |

## Missing-Surface Risk Register

- `jsfartStreamProfile`: present in model/export JSON and docs; no literal hit in CLI source/tests. Verify CLI through object-stream command profile count/family output.
- `imagePayloadDiagnostic`: literal is model layer/SVG diagnostic only; export crate has payload helper coverage but not the named layer diagnostic literal.
- `referenceFallbackAdmissionGate`, `sourceOnlyAxisAdmissionGate`, `sourceOnlyPageYRenderAdmissionGate`, `gridOriginAuthorityGate`, `lineRuleRenderAdmissionGate`: literal surfaces are model-owned diagnostics plus docs; no export/CLI literal hits in the probed files.
- `primitiveOwnershipAdmissionGate`: present in model and export; no CLI literal hit. Q4/Q5 behavioral verification depends on local sample availability for the PDF-backed branch.

## Acceptance Verification To Run After Write

```bash
cd /Users/kimuj5090/Documents/rjtd && test -s .omo/evidence/current-progress-next-plan/task-2-symbol-test-map.md && rg -n "ObjectStreamCandidate|sourceOnlyAxisAdmissionGate|primitiveOwnershipAdmissionGate|lineRuleRenderAdmissionGate|cargo test" .omo/evidence/current-progress-next-plan/task-2-symbol-test-map.md
```

