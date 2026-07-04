# Task 17 Notepad

Task: Keep `shanai_lan` line-rule render admission diagnostic-only unless origin, endpoints, style, and paint order are proven together.

## Outcome

- Product files changed by this task: none.
- `lineRuleRenderAdmissionGate.promotionReady:false` remains observed in the CLI layer-tree artifact.
- Per-rule/per-component admission gates remain `promotionReady:false`.
- Visible line-rule SVG classes remain absent.
- Generated PDF artifact was produced only as evidence; no SVG/PDF rendering code changed.

## Command Exits

- `cargo test -p rjtd-model document_core_projects_shanai_lan_fdm_frame_diagnostics -- --exact`: exit 0, but 0 tests ran.
- `cargo test -p rjtd-model tests::document_core_projects_shanai_lan_fdm_frame_diagnostics -- --exact`: exit 0, 1 test passed.
- `cargo test --workspace`: exit 0.
- CLI `page-layer-tree` artifact generation: exit 0.
- CLI `page-svg` artifact generation: exit 0.
- CLI PDF export artifact generation: exit 0.
- Temporary target cleanup: exit 0.

## Manual QA Matrix

| Scenario | Invocation | Binary observable | Artifact |
| --- | --- | --- | --- |
| Happy: line-rule render gate stays diagnostic-only | `cargo run -p rjtd-cli -- page-layer-tree <shanai_lan.jtd> 0` | `lineRuleRenderAdmissionGate` count with `promotionReady:false` is 1; per-rule/component `renderAdmissionGate` with `promotionReady:false` is 22 | `.omo/evidence/current-progress-next-plan/task-17-artifacts/layer-tree.json` |
| Happy: no visible SVG line-rule overlay | `cargo run -p rjtd-cli -- page-svg <shanai_lan.jtd> 0` | `rjtd-shanai-lan-line-rule`, `rjtd-shanai-lan-line-rules`, and `rjtd-fdm-open-stroke-axis-rule-connector-readiness` are absent | `.omo/evidence/current-progress-next-plan/task-17-artifacts/page.svg` |
| Happy: PDF still exports without a line-rule promotion probe | `cargo run -p rjtd-cli -- export <shanai_lan.jtd> --format pdf -o generated.pdf` | PDF artifact is nonempty; no product/rendering code changed in this task | `.omo/evidence/current-progress-next-plan/task-17-artifacts/generated.pdf` |
| Failure guard: renderer compatibility not worsened by this task | `cargo test --workspace` | export suite passed, including available local PDFKit/CoreGraphics-oriented checks | `.omo/evidence/current-progress-next-plan/task-17-shanai-line-rule.log` |

## Review

- Code review: no product code was edited for task 17; the existing gate already requires source-backed origin/endpoints/style/paint-order proof and keeps `renderable:false`.
- Slop/overfit review: no sample-name condition, row-index special case, or single visually tempting rule render was added.
- Cleanup: temporary `CARGO_TARGET_DIR=/tmp/rjtd-task-17-cargo-target` removed; evidence artifacts intentionally retained.

## Risks

- The repository had unrelated pre-existing dirty product files, including `rjtd/crates/rjtd-model/src/lib.rs`; this task did not revert or overwrite them.
- The unqualified exact selector runs 0 tests, so the module-qualified selector is the meaningful focused regression.
