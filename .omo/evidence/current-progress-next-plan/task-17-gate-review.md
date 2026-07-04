# Task 17 Gate Review

recommendation: APPROVE
verdict: confirmed

## OriginalIntent

Keep `shanai_lan` line-rule render admission diagnostic-only unless origin, endpoints, style, and paint order are proven together.

## DesiredOutcome

The user-visible outcome is that the `shanai_lan` line-rule evidence can be exposed as diagnostics in layer-tree output, but it must not be promoted into visible SVG/PDF line-rule or connector rendering. The layer-tree should continue to expose render admission gates with `promotionReady:false`, `renderable:false`, `diagnosticOnly:true`, and blocked reasons covering origin, endpoint ownership/attachment, style role, and paint order.

## UserOutcomeReview

Confirmed. The generated task-17 layer-tree artifact contains one `lineRuleRenderAdmissionGate` with `promotionReady:false`, 22 per-rule/per-component `renderAdmissionGate` objects with `promotionReady:false`, zero `promotionReady:true` hits, and zero line-rule projection objects matched as `renderable:true` by the lightweight audit. The blocked-reason audit found the required reasons, including `document-text-grid-origin-reference-backed`, `line-rule-endpoint-ownership-unproven`, `line-rule-style-role-unproven`, `line-rule-paint-order-unproven`, and `line-rule-render-admission-not-ready`.

The generated SVG lacks the visible promotion classes under review: `rjtd-shanai-lan-line-rule`, `rjtd-shanai-lan-line-rules`, and `rjtd-fdm-open-stroke-axis-rule-connector-readiness`. Generic `line-rule` strings are present only as diagnostic attributes/text metadata, not as the visible promotion classes. Source search found those forbidden class names only in tests asserting their absence.

## Blockers

None.

## Checked Artifact Paths

- `.omo/evidence/current-progress-next-plan/task-17-shanai-line-rule.log`
- `.omo/evidence/current-progress-next-plan/task-17-notepad.md`
- `.omo/evidence/current-progress-next-plan/task-17-artifacts/layer-tree.json`
- `.omo/evidence/current-progress-next-plan/task-17-artifacts/page.svg`
- `.omo/evidence/current-progress-next-plan/task-17-artifacts/generated.pdf`
- `rjtd/crates/rjtd-model/src/lib.rs`
- `rjtd/crates/rjtd-export/src/lib.rs`
- `rjtd/crates/rjtd-cli/src/main.rs`
- `rjtd/crates/rjtd-cli/tests/streams.rs`

## Evidence

- Required evidence files and artifact directory exist. Task-17 artifacts are exactly `generated.pdf`, `layer-tree.json`, and `page.svg` under `.omo/evidence/current-progress-next-plan/task-17-artifacts/`.
- Evidence log includes command exits: initial shared-target `COMMAND_EXIT: 130`, isolated unqualified exact `COMMAND_EXIT: 0`, qualified exact `COMMAND_EXIT: 0`, workspace test `COMMAND_EXIT: 0`, CLI artifact generation exits, and cleanup `COMMAND_EXIT: 0`.
- Evidence log discloses the unqualified exact selector ran 0 tests, then reran `tests::document_core_projects_shanai_lan_fdm_frame_diagnostics -- --exact`, which passed 1 test.
- Evidence log records `cargo test --workspace` passed with crate test batches including 124, 60, 30, 95, and 3 passed tests, followed by doc-test batches with 0 tests and final `COMMAND_EXIT: 0`.
- CLI artifact generation passed and produced nonempty artifacts: `layer-tree.json` 5,211,159 bytes, `page.svg` 2,318,488 bytes, `generated.pdf` 411,476 bytes.
- Manual QA matrix is present in the notepad and the log contains layer-tree/SVG/PDF generation plus artifact checks.
- Code review/slop-overfit section is present in the log and notepad. It is terse, but direct gate review found no unresolved slop attributable to task 17: no production source addition of visible line-rule/connector classes, no sample-name or row-index render promotion, no deletion-only/tautological task-17 test change, and no task-17 production extraction or normalization.
- Temporary `CARGO_TARGET_DIR=/tmp/rjtd-task-17-cargo-target` is absent.
- Generated task-17 artifacts are under `.omo/evidence`. Product output paths under `openjtd-samples`, `openjtd.github.io`, and `rjtd` do not contain task-17 `layer-tree.json`, `page.svg`, or `generated.pdf`. Older scratch artifacts exist under `tmp/pdfs`, with June 2026 mtimes, outside product paths.

## Live Audit Results

- `lineRuleRenderAdmissionGate promotionReady=false count`: 1
- `renderAdmissionGate promotionReady=false count`: 22
- `promotionReady=true count`: 0
- `renderable=true line-rule projection count`: 0
- `line-rule-render-admission-not-ready`: 17
- `document-text-grid-origin-reference-backed`: 26
- `line-rule-endpoint-ownership-unproven`: 14
- `line-rule-style-role-unproven`: 17
- `line-rule-paint-order-unproven`: 23
- SVG forbidden class audit: all three named visible line-rule/connector classes absent

## Source Attribution Review

The worktree has broad pre-existing dirty product files. Task-17 attribution is supported by timestamps and command transcript: product source mtimes precede the task-17 evidence artifacts (`rjtd-model/src/lib.rs` last modified 2026-07-01 10:43:06 KST; task-17 log started 2026-07-01 10:56:06 KST and artifacts were generated at 11:06), and the task-17 transcript contains test, CLI artifact generation, artifact audit, review, and temp cleanup commands only. This is sufficient to confirm no product source edit is attributable to task 17 in the available artifacts.

Direct source inspection of the relevant gate functions shows line-rule projection, component, and per-rule render admission JSON writers always emit `diagnosticOnly:true`, `renderable:false`, and `promotionReady:false`, while pushing blocked reasons for reference-backed origin, endpoint/topology gaps, style role, and paint order. No visible line-rule SVG renderer was added in the reviewed source.

## ReproCommands

```bash
test -f .omo/evidence/current-progress-next-plan/task-17-shanai-line-rule.log
test -f .omo/evidence/current-progress-next-plan/task-17-notepad.md
test -d .omo/evidence/current-progress-next-plan/task-17-artifacts

rg -n 'COMMAND_EXIT|ZERO_TEST_DISCLOSURE|Manual QA|Code review|slop|cleanup|task-17-notepad' \
  .omo/evidence/current-progress-next-plan/task-17-shanai-line-rule.log \
  .omo/evidence/current-progress-next-plan/task-17-notepad.md

rg -o '"lineRuleRenderAdmissionGate":\{[^}]*"promotionReady":false' \
  .omo/evidence/current-progress-next-plan/task-17-artifacts/layer-tree.json | wc -l
rg -o '"renderAdmissionGate":\{[^}]*"promotionReady":false' \
  .omo/evidence/current-progress-next-plan/task-17-artifacts/layer-tree.json | wc -l
rg -o '"promotionReady":true' \
  .omo/evidence/current-progress-next-plan/task-17-artifacts/layer-tree.json | wc -l
rg -o '"projectionKind":"documentTextLineRuleProjection"[^}]*"renderable":true' \
  .omo/evidence/current-progress-next-plan/task-17-artifacts/layer-tree.json | wc -l

for pat in \
  'line-rule-render-admission-not-ready' \
  'document-text-grid-origin-reference-backed' \
  'line-rule-endpoint-ownership-unproven' \
  'line-rule-style-role-unproven' \
  'line-rule-paint-order-unproven'
do
  rg -o "\"$pat\"" .omo/evidence/current-progress-next-plan/task-17-artifacts/layer-tree.json | wc -l
done

for pat in \
  'rjtd-shanai-lan-line-rule' \
  'rjtd-shanai-lan-line-rules' \
  'rjtd-fdm-open-stroke-axis-rule-connector-readiness'
do
  rg -q "$pat" .omo/evidence/current-progress-next-plan/task-17-artifacts/page.svg && echo present || echo absent
done

test ! -e /tmp/rjtd-task-17-cargo-target
find openjtd-samples openjtd.github.io rjtd -path '*/target' -prune -o \
  \( -name 'layer-tree.json' -o -name 'page.svg' -o -name 'generated.pdf' \) -print
```

## EvidenceGaps

- The worker's embedded code review/slop-overfit section is concise and does not enumerate every inapplicable `remove-ai-slops` category. Direct gate review compensated by checking the diff, tests, production source, and generated artifacts; no unresolved task-17 slop or overfit was found.
- Exact product-edit attribution cannot be proven from git alone because the repository is already dirty from prior tasks. Available timestamps, transcript commands, and source/output audits support the no-task-17-product-edit claim.

confidence: high
