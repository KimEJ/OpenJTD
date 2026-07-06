# Source Table Layout Promotion ULW Terminal Note

Status: quality-gate clean; blocked-safe no production render promotion.

Session:
- OMO session id: source-table-layout-promotion-20260705
- Goal id: G001-end-to-end-attempt-to-promote-source
- Goals file: .omo/ulw-loop/source-table-layout-promotion-20260705/goals.json
- Ledger file: .omo/ulw-loop/source-table-layout-promotion-20260705/ledger.jsonl

Decision:
- Do not promote the new PAGE01 source-derived table candidates to production
  source-only tableProjection.
- The table grouping evidence is good: all new PAGE01 samples expose 3x2 table
  candidates, right samples expose first-cell source offset deltas 2/4/6/8,
  and PDF coordinate sweeps match the expected right/down deltas.
- The render-admission RED assertion correctly stays false:
  diagnosticProjection, renderPromoted=false, and
  sourceDerivedLayoutRenderable=false remain true for baseline/right4/down4.

Exact blocker:
- Source page-Y origin remains undecoded because row-to-LineMark coupling is
  stride/interleaved rather than exact contiguous row boundaries.
- Horizontal full-extent semantics remain unproven as a secondary blocker.

Quality gate:
- Final gate reviewer recommendation: APPROVE.
- Gate review: clean.
- Iteration: clean.
- Code review: clean; remove-ai-slops clean; programming clean.
- Manual QA: clean.
- Codex goal mismatch: not a terminal quality blocker; classified by gate review
  as an operational bookkeeping limitation.

Verification evidence:
- .omo/evidence/source-table-layout-promotion-20260705/c001-source-and-pdf-sweep.txt
- .omo/evidence/source-table-layout-promotion-20260705/c002-render-admission-or-blocker.txt
- .omo/evidence/source-table-layout-promotion-20260705/c003-regression-safety.txt
- .omo/evidence/source-table-layout-promotion-20260705/final-review-fixes.txt
- .omo/evidence/source-table-layout-promotion-20260705-code-review-postfix.md
- .omo/evidence/source-table-layout-promotion-20260705/final-manual-qa-rerun/final-verdict.json
- .omo/evidence/source-table-layout-promotion-20260705/final-manual-qa/superseded-failures-resolution.txt

Commands verified after review fixes:
- cargo fmt --all --check
- cargo check --workspace
- cargo test -p rjtd-cli --test source_y_probe
- cargo test -p rjtd-cli --test source_y_probe -- --ignored --nocapture
- cargo test -p rjtd-model document_text_control -- --nocapture
- git diff --check

Review blocker fixes:
- Split probe_signals into responsibility-focused modules; final line-count
  audit keeps every probe source/test module below 250 lines.
- Added boundary and negative tests for the configured three-empty-row
  DocumentText-control table gap heuristic.
- Marked local corpus probe tests ignored/manual by default and made them fail
  loudly if explicitly run without the local fixture corpus.
- Explicitly superseded failed exploratory manual-QA artifacts with passing
  rerun artifacts.

Checkpoint note:
- `omo ulw-loop checkpoint --status complete` was attempted and rejected because
  the native Codex `get_goal` snapshot points to an older paused
  task-11-tsaiten-source-readiness-20260701 objective rather than this isolated
  source-table-layout-promotion-20260705 OMO session.
- The final gate reviewer classified this as operational bookkeeping, not a
  code/QA blocker. This terminal note and the ledger annotation are the durable
  completion record for this isolated OMO session.

Cleanup:
- All spawned reviewers/subagents used for this run were closed or completed.
- No dev server, browser context, tmux session, bound port, container, or temp
  service remains intentionally open.
