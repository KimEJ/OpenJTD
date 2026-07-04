# Gate Review: Task 8 Doc Sync Supplement

## recommendation
APPROVE

## blockers
- None.

## originalIntent
Synchronize the English and Japanese TODO/RFC task records with the current code-backed gate and blocker names for task 8, while keeping generated output out of the source-of-truth chain and avoiding Rust source edits for the evidence supplement.

## desiredOutcome
- `TODO.md`, `TODO.ja.md`, `openjtd-spec/rfc/0008-object-stream-candidates.md`, and `openjtd-spec/rfc/0008-object-stream-candidates.ja.md` contain the canonical gate/blocker names required by the plan acceptance command.
- English and Japanese records mirror the same canonical facts.
- Stale `role-span-paint-order-unproven` is absent from the four task docs.
- Replacement blockers `role-span-interleaved-non-role-commands` and `role-paint-order-authority-unproven` are backed by model/export source and tests.
- RFC wording treats model/test evidence as authority and explicitly does not promote generated output to source truth.
- Supplement evidence now includes manual QA, code-review/slop-overfit coverage, notepad path/artifact, and cleanup scope.

## userOutcomeReview
Confirmed. The supplemented evidence package now closes the prior blocker: the primary evidence log, task-specific code-review/slop-overfit report, and task-specific notepad artifact all exist and are non-empty. The log includes a manual QA matrix, code-review and notepad paths, cleanup notes, and supplement self-checks. Direct reruns confirm the canonical doc acceptance command, code/test authority, stale-string absence, replacement blocker parity, source-truth wording, and whitespace hygiene. Rust files remain dirty in the worktree, but current Rust numstat matches the earlier task-1 baseline and predates the supplement artifacts, so no Rust source edit is attributable to the supplement.

## checked artifact paths
- `.omo/plans/current-progress-next-plan.md`
- `.omo/evidence/current-progress-next-plan/task-8-doc-sync.log`
- `.omo/evidence/current-progress-next-plan/task-8-code-review.md`
- `.omo/evidence/current-progress-next-plan/task-8-notepad.md`
- `.omo/evidence/current-progress-next-plan/task-8-doc-sync-gate-review.md`
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
- Artifact existence: `ls -l` showed non-empty `task-8-doc-sync.log` (14519 bytes), `task-8-code-review.md` (2678 bytes), and `task-8-notepad.md` (1262 bytes).
- Supplement coverage: `rg` over `task-8-doc-sync.log` found `Manual QA matrix`, code-review artifact path, notepad artifact path, source-truth wording, Rust-source-no-edit check, and cleanup sections.
- Slop/overfit report coverage: `task-8-code-review.md` explicitly covers no excessive/useless tests, no weakened/deleted/skipped tests, no tautological implementation-mirroring checks, no unnecessary production extraction/parsing/normalization, no generated-output source truth, no Rust source edits, and verdict.
- Direct slop/programming pass: the supplement added no tests, production parsers, normalizers, abstractions, or Rust edits. The checks are string-level acceptance and authority probes against existing source/test strings, not implementation-mirroring helper tests.
- Acceptance command rerun: `rg -n "sourceOnlyAxisAdmissionGate|primitiveOwnershipAdmissionGate|lineRuleRenderAdmissionGate|image-signature-without-complete-payload-role-unproven|fdm-frame-linked-image-payload-placement-and-paint-order-unproven" TODO.md TODO.ja.md openjtd-spec/rfc/0008-object-stream-candidates.md openjtd-spec/rfc/0008-object-stream-candidates.ja.md` exited 0 with hits in English and Japanese records.
- Term parity: English/Japanese counts match for all checked canonical and replacement strings. `lineRuleRenderAdmissionGate` appears in both TODO files and in neither RFC file, so there is no language drift.
- Code/test authority: direct `rg` over `rjtd/crates/rjtd-model/src/lib.rs`, `rjtd/crates/rjtd-export/src/lib.rs`, `rjtd/crates/rjtd-cli/src/main.rs`, and `rjtd/crates/rjtd-cli/tests/streams.rs` found the canonical gate/blocker names.
- Stale blocker removal: the negative check for `role-span-paint-order-unproven` over the four task docs exited 0 and printed `no stale role-span-paint-order-unproven in task docs`.
- Replacement blockers: `role-span-interleaved-non-role-commands` and `role-paint-order-authority-unproven` appear in both language docs and in model/export implementation/tests.
- Source-truth wording: English and Japanese RFC line 185 state generated output is not promoted to source truth and that preserved model/test evidence is the authority.
- Whitespace hygiene: `git diff --check -- TODO.md TODO.ja.md openjtd-spec/rfc/0008-object-stream-candidates.md openjtd-spec/rfc/0008-object-stream-candidates.ja.md` exited 0 with no output.
- Rust attribution: current Rust `git diff --numstat` is `138/6`, `44/4`, `1846/165`, and `10922/245`, matching `.omo/evidence/current-progress-next-plan/task-1-current-state.md` and the prior gate review. Rust file mtimes predate the supplement evidence artifacts.

## exact evidence gaps
- No unresolved evidence gaps for task 8 after supplementation.
- Residual risk: the wider worktree still contains pre-existing Rust WIP outside task 8 and was not reviewed as part of this documentation/evidence-supplement gate.
