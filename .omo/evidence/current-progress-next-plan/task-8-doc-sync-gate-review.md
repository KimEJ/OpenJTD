# Gate Review: Task 8 Doc Sync

## recommendation
REJECT

## blockers
- Task-specific code-review report is absent. Final-gate policy requires a review report that explicitly covers the same `remove-ai-slops` / `programming` skill perspective and overfit/slop criteria; `rg` over `.omo/evidence/current-progress-next-plan`, `.omo/plans/current-progress-next-plan.md`, and `.omo/start-work/ledger.jsonl` found no task-8 code review report or overfit/slop coverage.
- Task-specific manual QA matrix is absent from `task-8-doc-sync.log`. The log has QA checks and reproducible commands, but no `manualQa` matrix comparable to task 1/task 3 artifacts.
- Notepad path was not provided in the review input and no task-8 notepad artifact was found.

## originalIntent
Synchronize English and Japanese TODO/RFC documentation with the current code-backed gate/blocker names for task 8, without editing Rust source or treating generated output as source truth.

## desiredOutcome
- The four task docs use canonical current gate names.
- English and Japanese records mirror the same facts.
- Stale `role-span-paint-order-unproven` wording is gone.
- Replacement blockers are code-backed.
- Docs state model/test evidence is authority, not generated output.
- Task 8 touches docs/evidence only, not Rust source.

## userOutcomeReview
Direct task-8 checks are confirmed: acceptance `rg`, code/test `rg`, stale-string removal, replacement blocker parity, doc whitespace check, and source-truth wording all pass. However, the final-gate artifact package is incomplete because the required code-review/slop-coverage report and task-specific manual QA matrix are missing. Under the final-gate rules, this cannot be approved even though the requested doc-sync behavior appears satisfied.

## checked artifact paths
- `.omo/plans/current-progress-next-plan.md`
- `.omo/evidence/current-progress-next-plan/task-8-doc-sync.log`
- `.omo/evidence/current-progress-next-plan/task-1-current-state.md`
- `.omo/start-work/ledger.jsonl`
- `TODO.md`
- `TODO.ja.md`
- `openjtd-spec/rfc/0008-object-stream-candidates.md`
- `openjtd-spec/rfc/0008-object-stream-candidates.ja.md`
- `rjtd/crates/rjtd-model/src/lib.rs`
- `rjtd/crates/rjtd-export/src/lib.rs`
- `rjtd/crates/rjtd-cli/src/main.rs`
- `rjtd/crates/rjtd-cli/tests/streams.rs`

## evidence
- Evidence file exists and is non-empty: `ls -l .omo/evidence/current-progress-next-plan/task-8-doc-sync.log` reported size `6487`.
- Evidence file records required scenarios: `rg -n "Scenario: canonical gate/blocker docs acceptance|Scenario: canonical names exist in code/tests|Scenario: stale blocker string removal|Scenario: replacement paint-order blocker parity|git diff --check|Scenario: cleanup receipt|Cleanup:|Temporary acceptance output|QA checks|RFC wording explicitly states" .omo/evidence/current-progress-next-plan/task-8-doc-sync.log` matched all required sections.
- Acceptance command rerun passed: canonical names appeared in all four docs, including RFC lines 159/185/201/209/223/227/243 and TODO/TODO.ja line groups for image blockers, `sourceOnlyAxisAdmissionGate`, `primitiveOwnershipAdmissionGate`, and `lineRuleRenderAdmissionGate`.
- Code/test authority rerun passed: canonical names appeared in `rjtd/crates/rjtd-model/src/lib.rs`, `rjtd/crates/rjtd-export/src/lib.rs`, `rjtd/crates/rjtd-cli/src/main.rs`, and `rjtd/crates/rjtd-cli/tests/streams.rs`.
- Stale drift check passed: `role-span-paint-order-unproven` was absent from the four task docs.
- Replacement blocker parity passed: `role-span-interleaved-non-role-commands` and `role-paint-order-authority-unproven` appeared in English docs, Japanese docs, and model/export code/tests.
- `git diff --check -- TODO.md TODO.ja.md openjtd-spec/rfc/0008-object-stream-candidates.md openjtd-spec/rfc/0008-object-stream-candidates.ja.md` exited 0 with no output.
- Source-truth wording is present in English and Japanese RFCs at line 185, stating generated output is not promoted to source truth and that blockers describe preserved model/test evidence.
- Rust dirty files are pre-existing WIP: task 1 current-state ledger already listed the same Rust modified files and identical current `git diff --numstat` counts for Rust paths (`138/6`, `44/4`, `1846/165`, `10922/245`). Task-8 evidence and ledger claim only docs plus evidence changed, with cleanup noting no Rust source edits.

## exact evidence gaps
- No task-8 code-review report path was provided or found.
- No artifact was found that explicitly applies `remove-ai-slops` overfit/slop criteria to task 8 and confirms absence of excessive/useless tests, deletion-only tests, tautological checks, implementation-mirroring tests, or unnecessary production extraction/parsing/normalization.
- No task-specific `manualQa` matrix was found in `.omo/evidence/current-progress-next-plan/task-8-doc-sync.log`.
- No notepad path was provided in the gate input.

