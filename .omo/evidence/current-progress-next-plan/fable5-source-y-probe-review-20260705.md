# Fable 5 Source-Y Probe Review (2026-07-05)

## Invocation

- Path: guarded `fable5-last-resort` plugin server, launched locally with `FABLE5_ENABLE_EXPENSIVE_CALLS=1`
- Registered MCP server note: the already-running MCP server still refused because it did not inherit the updated environment variable.
- Model: `claude-fable-5`
- Usage reported by plugin: `2425 input, 2989 output, estimated $0.1737 before cache/fallback adjustments`

## Decision

Fable's independent review agrees with the conservative local decision:

> Do not implement or render-promote anything from the current corpus.

The current corpus is render-blocked because no sample provides decoded source-only page origin or page-y transform authority. The observed PDF movement is useful as diagnostic cross-check data, but it is not render authority.

## Main Reasons

- Every representative `page-layer-tree` gate still reports `admissionReady=false`.
- The decisive blocker is still `decoded-line-mark-page-y-transform-missing`.
- Cases with `pageOriginAuthority=fallbackTextAnchors` are not safe to promote. The baseline source candidate y is about `127.3 px`, while the measured PDF table top is about `90.3 pt`.
- `013_table_moved_right` is a strong negative control: PDF-visible x movement exists, but current source, line, page tuple, and table signatures are identical.
- `010a/011a/012a` improve row-boundary evidence, but they are not isolated transform proof because they reshape line/page/table evidence together.
- `030/031/032/064` remain confounded by page setup/top-left/row-gap changes.
- `040a` through `040e` prove that the RTF/import margin sweep is not perturbing the target source fields.
- The failed `074_multi_page_table` manifest entry should be fixed or removed before treating the corpus as clean.

## Evidence Assessment

### Useful Diagnostic Evidence

- `010a/011a/012a`: row-boundary decoding looks better because `lineMarkRowsExactAndContiguous=true`, but page origin is still `none` and transform authority is absent.
- `013_table_moved_right`: valuable as a negative signal because it proves the current decoded signatures miss table x placement.
- `040a` through `040e`: useful only to reject the RTF/import sweep path for margin decoding.
- `074a/074b/074c/074d`: useful multi-page diagnostics, but not page-origin/page-y transform proof.

### Frozen Out Of Admission

- `030/031/032/064`: keep as regression fixtures, but do not use as admission proof until remade with stable page setup/top-left/row gaps.
- `010/011/020/021/022/040/050/055`: keep as diagnostic/regression fixtures only.
- `080/081/082`: valid paragraph controls with no table candidates.

## Missing Proof

Fable identified these missing proof classes:

- A direct decoded page-origin field or a proven line-mark stride-to-page-y transform.
- A structurally derived unit mapping, not a fitted PDF constant.
- A source-side representation of table x placement.
- Page-mark absolute-y semantics or an equivalent source-backed multi-page placement authority.
- A clean corpus with the missing `074_multi_page_table` artifact resolved.

## Plan Amendments

Modify the loop as follows:

1. Retire the RTF/import margin sweep from the admission path.
2. Add a diagnostic raw-stream or whole-file diff stage so source-silent cases like `013_table_moved_right` can localize the missing x-placement record family.
3. Freeze `030/031/032/064` out of render admission until same-setup replacements exist.
4. Fix or remove the missing `074_multi_page_table` manifest entry.
5. Reconsider admission only after isolated native Y/X series show a decoded page-origin authority and at least three monotonic, single-variable transform points.

## Smallest Next Sample Set

All samples should be created natively in Ichitaro, with the same version, same page setup, same content, saved once, and one intended edit per file.

### Y-Series

- Move only the table down by exact increments: `10mm`, `20mm`, `30mm`, `40mm`, `50mm`, or equivalent exact line increments.
- Keep page setup, font, table shape, row heights, column widths, and content unchanged.
- Goal: isolate a monotonic source delta that can prove or reject a stride/unit transform.

### X-Series

- Move only the table right by exact increments: `10mm`, `20mm`, `30mm`, `40mm`.
- Run whole-file/whole-stream diff because decoded signatures missed `013_table_moved_right`.
- Goal: find the source field family for table x placement.

### Origin Probe

- Duplicate the baseline with only top margin changed to two known exact values.
- Keep table position untouched.
- Goal: separate margin-origin fields from table-y fields.

### Corpus Cleanup

- Regenerate `074_multi_page_table`, or amend the manifest so the corpus has no failed case.

## Local Reconciliation

Fable's result changes the plan details but not the render decision:

- Current render implementation remains blocked.
- Current diagnostic work remains useful.
- The next implementable code task is diagnostic-only: add raw-stream/whole-file diff reporting for source-silent visual cases.
- The next evidence task is native isolated sample generation, not PDF comparison alone.
