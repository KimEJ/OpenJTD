# Task 5 Gate Review

recommendation: REJECT

## blockers

- Missing code-review report for task 5 with explicit `omo:remove-ai-slops` and `omo:programming` coverage. Direct review was performed, but required report coverage is absent and unsupported.
- Missing task-5 manual QA matrix artifact. The evidence log records commands and a done-claim summary, but not the required manual QA matrix shape.
- Missing notepad path/artifact in the supplied gate input and workspace artifacts.
- Plan state remains inconsistent with the done claim: `.omo/plans/current-progress-next-plan.md` still shows task 5 unchecked.

## originalIntent

Lock object-stream and image-payload diagnostic preservation across model, export, and CLI. Preserve `jsfartStreamProfile`, image payload/envelope evidence, ownership/frame rows, FDM image candidates, signature-only blockers, and do not promote object/FDM image payloads to `renderable:true` without same-object page assignment and paint-order proof.

## desiredOutcome

The user-visible outcome is a verified checkbox-5 DoneClaim: non-empty evidence, acceptance command captured, exact tests proven not to be zero-test false positives, required blocker strings present, scoped object/FDM image diagnostics staying `renderable=false`, and no task-5 product source edits beyond the pre-existing dirty tree.

## userOutcomeReview

The narrow technical checks are confirmed. The evidence file is non-empty, includes the literal acceptance command, discloses the unqualified model/export `--exact` zero-test trap, and records supplementary fully qualified model/export tests that each ran one test and passed. Independent reruns reproduced this: the literal command exits 0 but runs zero tests for the two lib crates; the fully qualified `tests::...` commands run and pass.

Scoped static audit confirms the object/FDM image diagnostic paths use `renderable=false`: CLI `object-fdm-image-candidates` and `object-fdm-frame-links`, model `imagePayloadDiagnostic`, SVG `rjtd-image-payload-diagnostic`, and FDM frame diagnostics. Broad `renderable:true` hits exist elsewhere in reference-backed visual-list/title-art/FDM primitive projections, but the scoped object/FDM image diagnostic audit found no `renderable:true` matches.

Task attribution is limited by the pre-existing dirty tree. Task 1 recorded the same tracked product WIP before task 5. Current status still shows those tracked product files dirty, while task 5's changed artifact is untracked evidence. No product source edit is attributable to task 5 from the available artifacts.

## directSkillPass

- `omo:remove-ai-slops`: no task-5 production/test diff was introduced to clean. Scoped existing tests are exact-output/string contract tests, but they assert observable CLI/model/export diagnostics and specifically guard the requested blocker strings and renderability gates; no deletion-only, tautological removal-only, or implementation-mirroring task-5 change was found.
- `omo:programming`: Rust command reruns and source inspection found the scoped diagnostic paths conservative. The literal acceptance command is insufficient by itself because of zero-test lib filters; the supplemental `tests::...` commands are required evidence.

## checkedArtifactPaths

- `/Users/kimuj5090/Documents/rjtd/.omo/plans/current-progress-next-plan.md`
- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-5-object-image.log`
- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-1-current-state.md`
- `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-2-symbol-test-map.md`
- `/Users/kimuj5090/Documents/rjtd/.omo/start-work/ledger.jsonl`
- `/Users/kimuj5090/Documents/rjtd/rjtd/crates/rjtd-cli/src/main.rs`
- `/Users/kimuj5090/Documents/rjtd/rjtd/crates/rjtd-cli/tests/streams.rs`
- `/Users/kimuj5090/Documents/rjtd/rjtd/crates/rjtd-model/src/lib.rs`
- `/Users/kimuj5090/Documents/rjtd/rjtd/crates/rjtd-export/src/lib.rs`

## reproCommands

- `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-cli --test streams object_stream_candidates_command_reports_visual_object_inventory -- --exact && cargo test -p rjtd-cli --test streams object_fdm_image_candidates_command_reports_signature_only_blocker -- --exact && cargo test -p rjtd-cli --test streams object_fdm_frame_links_command_connects_fdm_rows_to_frame_records -- --exact && cargo test -p rjtd-model image_payload_render_gate_preserves_source_frame_trace_without_promotion -- --exact && cargo test -p rjtd-export exports_object_stream_candidates_to_json -- --exact`
- `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model tests::image_payload_render_gate_preserves_source_frame_trace_without_promotion -- --exact && cargo test -p rjtd-export tests::exports_object_stream_candidates_to_json -- --exact`
- `cd /Users/kimuj5090/Documents/rjtd/rjtd && cargo test -p rjtd-model -- --list | rg 'image_payload_render_gate_preserves_source_frame_trace_without_promotion' && cargo test -p rjtd-export -- --list | rg 'exports_object_stream_candidates_to_json'`
- `cd /Users/kimuj5090/Documents/rjtd && rg -n "jsfart-stream-profile=1|image-signature-without-complete-payload-role-unproven|fdm-frame-linked-image-payload-placement-and-paint-order-unproven|renderable=false" .omo/evidence/current-progress-next-plan/task-5-object-image.log rjtd/crates/rjtd-cli/tests/streams.rs rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-export/src/lib.rs rjtd/crates/rjtd-cli/src/main.rs TODO.md TODO.ja.md openjtd-spec/rfc/0008-object-stream-candidates.md openjtd-spec/rfc/0008-object-stream-candidates.ja.md`
- `cd /Users/kimuj5090/Documents/rjtd && if rg -n 'imagePayloadDiagnostic[^\n]*renderable\\?":true|rjtd-image-payload-diagnostic[^\n]*data-renderable="true"|object-fdm-(image-candidate|frame-link)[^\n]*renderable=true|object-fdm-(image-candidate|frame-link)[^\n]*renderable\\":true|fdmFrameDiagnosticProjection[^\n]*renderable\\?":true|rjtd-fdm-frame-diagnostic[^\n]*data-renderable="true"' rjtd/crates/rjtd-model/src/lib.rs rjtd/crates/rjtd-cli/src/main.rs rjtd/crates/rjtd-cli/tests/streams.rs rjtd/crates/rjtd-export/src/lib.rs; then exit 1; else echo 'no scoped object/FDM image renderable:true matches'; fi`

## exactEvidenceGaps

- No artifact matching a task-5 code-review report was found under `.omo/evidence/current-progress-next-plan/`; `rg "remove-ai-slops|programming|slop|overfit"` found no coverage.
- No `.omo/notepad.md` or `.omx/notepad.md` content was available.
- The task-5 evidence log has command transcripts but no manual QA matrix section.
- The supplied gate input did not include a diff snapshot specific to task 5; source attribution relies on task-1 preflight status plus current status.
