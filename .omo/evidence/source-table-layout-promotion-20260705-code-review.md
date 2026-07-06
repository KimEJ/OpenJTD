# Code Review: source-table-layout-promotion-20260705

## Verdict

codeQualityStatus: WATCH  
recommendation: APPROVE  
codeReview: clean  
removeAiSlops: clean  
programming: clean

Current blocker-fix state is clean for the requested review scope. The no-production-promotion outcome is supported: selected PAGE01 artifacts remain diagnostic-only with `projectionKind=diagnosticProjection`, `renderPromoted=false`, `sourceDerivedLayoutRenderable=false`, `decoded=false`, and explicit page-Y/horizontal blockers.

## Skill Perspective Check

- Ran: `code-review` skill loaded from `/Users/kimuj5090/.codex/skills/code-review/SKILL.md`.
- Ran: `remove-ai-slops` skill loaded from `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.15.1/skills/remove-ai-slops/SKILL.md`.
- Ran: `programming` skill loaded from `/Users/kimuj5090/.codex/plugins/cache/sisyphuslabs/omo/4.15.1/skills/programming/SKILL.md`.
- Ran: Rust programming reference loaded from `references/rust/README.md`.
- Ran: Rust `cargo-strict.md` reference consulted for clippy/quality-gate interpretation.
- Result: no current-diff blocker under either `remove-ai-slops` or `programming`. The previous oversized-probe and silent-skip blockers are fixed.

## Evidence Inspected

- `.omo/evidence/source-table-layout-promotion-20260705/final-review-fixes.txt`
- `.omo/evidence/source-table-layout-promotion-20260705/final-manual-qa/superseded-failures-resolution.txt`
- `.omo/evidence/source-table-layout-promotion-20260705/final-manual-qa/manualQa-matrix.json`
- `.omo/evidence/source-table-layout-promotion-20260705/c001-source-and-pdf-sweep.txt`
- `.omo/evidence/source-table-layout-promotion-20260705/c002-render-admission-or-blocker.txt`
- `.omo/evidence/source-table-layout-promotion-20260705/c003-regression-safety.txt`
- Direct source reads of the tracked diff plus untracked probe modules/tests.
- Direct render-gate reads around `table_grid_source_derived_layout_is_renderable`.

## Commands Rerun

- `git diff --check`: pass.
- `cargo fmt --all --check`: pass.
- `cargo check --workspace`: pass.
- `cargo test -p rjtd-cli --test source_y_probe`: pass, 4 ignored/manual tests by default.
- `cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture`: pass, 4 passed with local corpus present.
- `cargo test -p rjtd-model document_text_control -- --nocapture`: pass, 6 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: fails on pre-existing `rjtd-model/src/lib.rs` warnings outside this diff; see LOW.

## CRITICAL

None.

## HIGH

None.

## MEDIUM

None.

## LOW

1. `rjtd/crates/rjtd-model/src/lib.rs:22880`, `rjtd/crates/rjtd-model/src/lib.rs:25485`, `rjtd/crates/rjtd-model/src/lib.rs:37111`, `rjtd/crates/rjtd-model/src/lib.rs:48920`, `rjtd/crates/rjtd-model/src/lib.rs:48924`, `rjtd/crates/rjtd-model/src/lib.rs:63086`, `rjtd/crates/rjtd-model/src/lib.rs:66270`, `rjtd/crates/rjtd-model/src/lib.rs:71701`, `rjtd/crates/rjtd-model/src/lib.rs:72869` - full workspace clippy remains red.

   These failures are real project debt, but `git diff -U0` shows they are outside the current blocker-fix hunks and `git blame` points them to earlier commit `4a036025`. They do not block approval of this rerun, but they mean the strict lint gate is not globally clean.

2. Worktree hygiene remains staging-sensitive.

   Probe modules/tests and evidence are untracked, and `.omo/.DS_Store` is also untracked. This is not a code defect, but final staging must include the new probe modules/tests required by `main.rs` and avoid accidental metadata churn.

## Criteria Coverage

- C001 source/PDF sweep: covered. PAGE01 baseline/down/right samples expose stable 3x2 table candidates; down variants move LineMark/source offsets vertically while right variants shift line-header X offsets.
- C002 render admission/blocker: covered. Baseline/right4/down4 all keep `promotion_RED_assertion_passed=False`, `projectionKind=diagnosticProjection`, `renderPromoted=false`, and `sourceDerivedLayoutRenderable=false`.
- C003 regression safety: covered. I reran the default/ignored probe tests and model table-control tests locally; all passed as described above.
- Previous blocker fixes: covered. Probe files are all under 250 pure LOC, gap heuristic has boundary and negative tests, corpus tests are ignored/manual by default and fail loudly when explicitly run without fixtures, and final-review-fixes audits tracked plus untracked Rust files.

## Required Fixes

None for this review scope.

## Residual Risks

- Source page-Y origin and LineMark/page-origin transform remain undecoded; production source-only `tableProjection` must stay blocked.
- Horizontal full-extent semantics remain unproven.
- Full workspace clippy is red due pre-existing issues outside this diff.
- Local corpus tests are manual/ignored by default, so CI coverage depends on explicitly running `--ignored` in an environment with `ichitaro-source-y-probe`.

## Blockers

None.
