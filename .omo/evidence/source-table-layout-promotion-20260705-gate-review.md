# Gate Review: source-table-layout-promotion-20260705

recommendation: APPROVE
gateReview: clean
iteration: clean

## originalIntent

Attempt end-to-end promotion of source-derived Ichitaro PAGE 01 3x2 table layout from diagnostic evidence to renderable table projection when justified. Use `ichitaro-source-y-probe/new` baseline/down/right JTD+PDF samples. Do not hardcode sample names or reference PDFs for production rendering. Preserve `decoded:false` and source-backed honesty unless the source transform is proven.

## desiredOutcome

A terminal, evidence-backed outcome: either source-only `tableProjection` promotion with proven X/Y source transform, or a blocked-safe no-promotion decision with the precise missing transform recorded and render admission staying false.

## userOutcomeReview

The shipped user outcome is blocked-safe no production source-only `tableProjection` promotion. That matches the original brief because the source transform was not proven. C002 keeps `projectionKind=diagnosticProjection`, `renderPromoted=false`, `sourceDerivedLayoutRenderable=false`, `referenceBacked=false`, and names the page-Y/LineMark stride blocker plus horizontal admission blocker.

The implementation iteration is clean for this scope. The probe CLI surfaces are diagnostic and emit `ready=false`; production diff adds no render-promotion/sample-reference hardcoding. The document-text control table gap change is covered by boundary and split tests. Local-corpus probe tests are ignored by default and pass when explicitly run with the local corpus.

## criteriaCoverage

- originalIntent: covered by an evidence-backed blocked-safe no-promotion decision, not by promotion.
- desiredOutcome: covered by C001/C002/C003 plus final manual QA and post-fix review.
- userOutcomeReview: clean; the user receives a defensible "not promoted because source page-Y origin/LineMark transform remains undecoded" result.

## checkedArtifactPaths

- `.omo/evidence/source-table-layout-promotion-20260705-code-review-postfix.md`
- `.omo/evidence/source-table-layout-promotion-20260705-code-review.md`
- `.omo/evidence/source-table-layout-promotion-20260705/final-review-fixes.txt`
- `.omo/evidence/source-table-layout-promotion-20260705/final-manual-qa-rerun/final-verdict.json`
- `.omo/evidence/source-table-layout-promotion-20260705/final-manual-qa/superseded-failures-resolution.txt`
- `.omo/evidence/source-table-layout-promotion-20260705/final-manual-qa/manualQa-matrix.json`
- `.omo/evidence/source-table-layout-promotion-20260705/c001-source-and-pdf-sweep.txt`
- `.omo/evidence/source-table-layout-promotion-20260705/c002-render-admission-or-blocker.txt`
- `.omo/evidence/source-table-layout-promotion-20260705/c003-regression-safety.txt`
- `.omo/ulw-loop/source-table-layout-promotion-20260705/goals.json`
- `.omo/ulw-loop/source-table-layout-promotion-20260705/ledger.jsonl`
- `rjtd/crates/rjtd-cli/src/main.rs`
- `rjtd/crates/rjtd-cli/src/probe_*.rs`
- `rjtd/crates/rjtd-cli/src/probe_signals/*.rs`
- `rjtd/crates/rjtd-cli/tests/source_y_probe.rs`
- `rjtd/crates/rjtd-model/src/lib.rs`

## directVerification

- Loaded/consulted `remove-ai-slops` and `programming` with Rust reference.
- Direct anti-slop pass: no added production diff lines for `tableProjection`, `renderPromoted`, `sourceDerivedLayoutRenderable`, PAGE 01/PAGE01, `pdf-output`, `ichitaro-source-y-probe`, or reference-PDF hardcoding.
- Direct production/probe grep: sample names and corpus paths appear only in ignored local-corpus tests.
- Direct LOC audit: all new probe source/test files are below 250 pure LOC; largest audited file was 196 pure LOC.
- Direct overfit/slop test review: model tests assert observable table candidate grouping/splitting; CLI corpus tests are sample-specific but isolated as ignored/manual diagnostics and are not used to justify production rendering.
- `cargo fmt --all --check`: pass.
- `cargo check --workspace`: pass.
- `cargo test -p rjtd-cli --test source_y_probe -- --nocapture`: pass, 4 ignored/manual.
- `cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture`: pass, 4 passed.
- `cargo test -p rjtd-model document_text_control -- --nocapture`: pass, 6 passed.
- `git diff --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: fails only on pre-existing `rjtd-model/src/lib.rs` findings outside the current diff, matching the code-review report.

## blockers

None.

## exactEvidenceGaps

No terminal evidence gaps for this gate. Residual technical gaps are the intended blockers: source page-Y origin/LineMark stride transform remains undecoded, and horizontal full-extent semantics remain unproven.

## operationalNotes

`goals.json` still says `status:"in_progress"` and native Codex goal state does not align with the executor's OMO state. Given the clean criteria artifacts, ledger evidence captures, and explicit post-fix review, this is an operational bookkeeping limitation, not a terminal quality blocker. Terminal OMO state can be recorded via ledger/explicit artifact if the normal checkpoint command rejects the objective mismatch.

## requiredNextAction

None for code or QA. Optional bookkeeping: record the terminal blocked-safe completion in OMO state/ledger using the explicit artifact path if the native Codex goal mismatch prevents the normal checkpoint command.

## final

recommendation: APPROVE
