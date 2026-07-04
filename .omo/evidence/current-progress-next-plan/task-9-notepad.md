# Task 9 Notepad: tsaiten regression coverage

- Worker task: Add regression coverage for `tsaiten` source-only geometry blockers before any geometry logic change.
- Product file touched: `rjtd/crates/rjtd-model/src/lib.rs`.
- Edit made: tightened `tests::table_grid_cross_table_subrecord_ordering_helpers_detect_regressions` with behavior-oriented assertions for:
  - `source-gap-to-page-line-gap-transform-unstable-across-table-family`
  - `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`
- Existing sample-backed test covers the full requested blocker set and was run because both local files exist:
  - `../rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.jtd`
  - `../rjtd-testdata/local-samples/ichitaro-20030120132956-0007-sp-dat-tsaiten.pdf`
- Required selector caveat: both unqualified `--exact` test invocations exited 0 with `running 0 tests`; fully qualified `tests::...` selectors were run and passed.
- Renderer output: unchanged.
- Plan/ledger: not edited.
- Evidence log: `/Users/kimuj5090/Documents/rjtd/.omo/evidence/current-progress-next-plan/task-9-tsaiten-regression.log`
