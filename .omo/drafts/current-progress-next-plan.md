---
slug: current-progress-next-plan
status: finalized
intent: unclear
plan: .omo/plans/current-progress-next-plan.md
next-action: execute with `$start-work .omo/plans/current-progress-next-plan.md`
approach: "Stabilize and verify the existing diagnostic-preservation WIP first, then sequence source-only geometry and FDM ownership proof work before any visible render promotion."
---

# Finalized: current-progress-next-plan

## Components (topology ledger)
- C1 | Current-state inventory: dirty tree, generated guidance, and planning artifacts are separated from pre-existing WIP. | active | git status; git diff --stat
- C2 | Object/FDM evidence hardening: JSFart profiles, image payload gates, FDM image/frame blockers, and CLI diagnostics stay decoded-false. | active | openjtd-spec/rfc/0008-object-stream-candidates.md:74-191; rjtd/crates/rjtd-cli/tests/streams.rs:2420
- C3 | Q4/Q5 FDM primitive ownership: row ownership, vector offset authority, fanout, and paint-order gates must explain role candidates before rendering. | active | TODO.md:691-700; openjtd-spec/rfc/0008-object-stream-candidates.md:193-205
- C4 | shanai_lan grid/wiring diagnostics: line-rule, PageMark, LineMark, FDMIndex, and connector evidence remain diagnostic until page origin and endpoint ownership are source-derived. | active | TODO.md:701-717; openjtd-spec/rfc/0008-object-stream-candidates.md:163-173
- C5 | success_data-test and ABC table source-layout gates: keep source-derived layout separated from selector/reference fallback diagnostics. | active | openjtd-spec/rfc/0008-object-stream-candidates.md:185-211
- C6 | tsaiten source-only table/page-grid geometry: next highest-priority blocker before reference calibration removal. | active | TODO.md:686-690; openjtd-spec/rfc/0008-object-stream-candidates.md:213-225
- C7 | Verification and release hygiene: compile/test/lint, sample sweeps, PDF visual checks, and final review evidence. | deferred | docs/ARCHITECTURE.md:27-44; docs/RHWP-COMPATIBILITY.md:30-46

## Open assumptions (announced defaults)
- A1 | First stabilize and verify the existing WIP instead of starting a new renderer feature. | The tree already has 13k+ insertions across model/export/CLI/TODO/RFC, so unverified expansion would increase risk. | yes
- A2 | Do not promote visible rendering unless the relevant admission gate is source-derived, non-reference-backed, and locked by tests. | Architecture requires exporters to consume the model and preserve unknown evidence. | yes
- A3 | Prioritize tsaiten source-only geometry next. | It is the first unchecked TODO and blocks reference table calibration removal. | yes
- A4 | Treat Q4/Q5 FDM primitive ownership as second priority. | The current blockers are explicit and narrow: vector offsets, fanout, row order, and paint order. | yes
- A5 | Keep shanai_lan line-rule/text-mask work diagnostic-only until page-space origin and endpoint ownership are proven. | Current evidence is useful but still reference-backed or semantically ambiguous. | yes
- A6 | Verification must include CLI/unit tests plus sample/PDF evidence, not only compile success. | Many blockers are visual or corpus-derived reverse-engineering claims. | yes

## Findings (cited - path:lines)
- Branch is `dev...origin/dev`; dirty tree contains pre-existing WIP in TODO/RFC plus `rjtd-model`, `rjtd-export`, `rjtd-cli`, and tests, plus new `.omo/` and AGENTS guidance artifacts from planning/bootstrap.
- Current tracked WIP size is large: `8 files changed, 13234 insertions(+), 424 deletions(-)`. The largest files are `rjtd/crates/rjtd-model/src/lib.rs` and `rjtd/crates/rjtd-export/src/lib.rs`.
- Layer boundaries are explicit: exporters consume only the document model and must not read raw container/stream/record data directly (`docs/ARCHITECTURE.md:27-35`).
- Unknown reverse-engineering data must be preserved instead of discarded (`docs/ARCHITECTURE.md:37-44`).
- `rhwp/` is read-only reference material; do not copy code, and do not add convenience dependencies without rhwp precedent (`docs/RHWP-COMPATIBILITY.md:30-46`).
- Object stream work has moved diagnostic evidence into model/export/app surfaces while keeping it decoded-false; sweeps report preserved candidates and payload metadata (`openjtd-spec/rfc/0008-object-stream-candidates.md:74-141`).
- FDMIndex/FDMVector evidence is model-owned and still non-renderable; FDM image candidates are signature/frame evidence, not proven paint resources (`openjtd-spec/rfc/0008-object-stream-candidates.md:143-161`, `183-191`).
- Q4/Q5 primitive ownership now has source-backed blocker gates for offset authority, fanout, role vector offset, row order, and paint-order continuity, but no render promotion (`openjtd-spec/rfc/0008-object-stream-candidates.md:193-205`; `TODO.md:691-700`).
- ABC table source-layout admission is ready without reference fallback; `tsaiten` remains blocked by fragmented y evidence, PageMark disagreement, unstable source-gap transform, and source/reference coupling (`openjtd-spec/rfc/0008-object-stream-candidates.md:207-225`; `TODO.md:686-690`, `717`).
- `shanai_lan` has improved FDM/text/grid/line-rule diagnostics, but rendering remains blocked by reference-backed grid origin, endpoint ownership, style role, paint order, or image-signature fragment semantics (`openjtd-spec/rfc/0008-object-stream-candidates.md:163-173`; `TODO.md:701-717`).
- CLI tests have been updated to assert JSFart stream profiles and stricter FDM image/frame blocker reasons (`rjtd/crates/rjtd-cli/tests/streams.rs:2420-2449`, `2713-2765`).

## Decisions (with rationale)
- D1: Plan around completing and verifying the current diagnostic WIP before widening scope. Rationale: the diff is already broad and crosses model/export/CLI/test/spec docs.
- D2: Preserve the decoded-false/source-backed distinction as the central guardrail. Rationale: most current work is reverse-engineering evidence, not decoded layout authority.
- D3: Make `tsaiten` source-only page/table geometry the first execution lane after stabilization. Rationale: it is the first unchecked TODO and directly blocks reference fallback removal.
- D4: Keep Q4/Q5 FDM role candidates non-rendering until admission blockers are closed by source evidence and tests. Rationale: role grouping is tempting visually but ownership/paint order are still unproven.
- D5: Treat `shanai_lan` wiring and line-rule probes as diagnostics until page origin plus endpoint ownership are independently decoded. Rationale: previous straight-line/full-span render probes worsened PDF comparison.

## Scope IN
- Audit the current WIP and separate user/pre-existing changes from planning artifacts.
- Write an execution-ready plan for stabilization, verification, and next reverse-engineering work.
- Include exact verification commands, QA scenarios, evidence paths, and guardrails.
- Sequence work so each todo is executable without further interview context.

## Scope OUT (Must NOT have)
- No product code edits in this planning turn.
- No visible render promotion based on reference-backed, filename-backed, or selector-only evidence.
- No copying from `rhwp/`; use it only as read-only reference.
- No exporter raw stream scanning that bypasses the model.
- No new dependencies, no weakened tests, and no auto-commit.

## Finalized execution defaults
- ED1: The next executor should start from `.omo/plans/current-progress-next-plan.md`, not from this background record.
- ED2: The plan already includes stabilization of current WIP plus the first `tsaiten`, Q4/Q5, and `shanai_lan` evidence lanes; no additional interview is required before execution.
- ED3: Generated planning artifacts stay under `.omo/`; product docs change only when a numbered todo requires TODO/RFC parity.
- ED4: The desired next deliverable is commit-ready evidence and a handoff recommendation, but staging/committing remains explicit.

## Finalized review note
- Independent read-only Codex CLI reviews were run and their required fixes were folded into `.omo/plans/current-progress-next-plan.md`.
- The resolved review areas were final verification fields, dependency metadata, sample/PDF skip handling, exact visual QA commands, all-or-nothing visible `tsaiten` fallback removal, stale gate wording, and finalized context status.
- Native Momus-style subagent review was not run because the current tool policy disallows spawning subagents unless the user explicitly asks for subagents; read-only Codex CLI reviews were used as the independent high-accuracy review path.
