# Fable 5 PAGE01 Render Admission Review (2026-07-05)

Invocation:

- Tool path: local fable5-last-resort MCP script via shell JSON-RPC
- Model: claude-fable-5
- stop_reason: end_turn
- Usage: 871 input, 1142 output, estimated $0.0658 before cache/fallback adjustments

Question:

Can the new PAGE01 baseline/right/down samples justify source-only render
promotion, given the CLI table-flow-y summary and model
sourceOnlyPageYRenderAdmissionGate sweep?

Result:

## Promotion Decision

Fable 5 says promotion is not justified.

Reasoning summary:

- `lineMarkRecordDelta=1..4` and `firstRowSourceStart` movement are consistent
  stride correlations, not a proven source-to-page-y transform.
- `exactSourceRangeMatchCount=0` and `rowsExactAndContiguous=false` mean the
  LineMark records do not demonstrably delimit the actual table row source
  ranges.
- `lineMarkPageOriginPresent=false` and `pageOriginAuthority=fallbackTextAnchors`
  mean there is no decoded page-space origin.
- Existing model blockers are substantive:
  `direct-line-mark-page-origin-absent`,
  `line-mark-record-stride-to-page-y-transform-unproven`, and
  `decoded-line-mark-page-y-transform-missing`.
- Older/synthetic passing cases show the gate is satisfiable; PAGE01 lacks the
  required structures.

## Minimal Additional Evidence Required

Fable 5 recommends a same-family JTD sample set that proves all three:

1. Direct page origin record:
   `lineMarkPageOriginPresent=true`, not only stride.
2. Exact contiguous row boundaries:
   line-mark records exactly tile table row source ranges, with
   `exactSourceRangeMatchCount == rowCount` and `rowsExactAndContiguous=true`.
3. Decoded unit/transform:
   same-family displacement samples where origin-record deltas are explainable
   by a decoded in-file unit/transform, not fitted from rendered PDF positions.

If real-world JTDs never carry direct page-origin evidence, the correct outcome
for this family is permanent non-promotion, not gate relaxation.

## Non-Promotional Follow-Up

Fable 5 says the current non-promotional implementation is safe enough to stop:

- CLI admission summary records why promotion is blocked.
- Model admission sweep records no PAGE01 file is render-admissible.
- `decoded=false` and source-backed honesty remain intact.
- No sample-name/reference-PDF promotion is introduced.

Optional recommended improvement:

- Add or preserve a named non-admissible hypothesis, e.g.
  `strideCorrelationObserved=true` and `transformProven=false`, so future
  investigators do not mistake the block for "no signal found".

Action taken in this session:

- CLI already exposes the non-admissible stride correlation through
  `table-flow-y-summary` and `table-flow-y-admission-summary`.
- The model gate already exposes `lineMarkPageOriginStridePresent=true` and
  `admissionReady=false`.
- No render gate relaxation is justified.
