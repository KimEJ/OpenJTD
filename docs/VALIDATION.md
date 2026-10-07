# rjtd Validation Record

Recorded checkpoint: 2026-10-07. This records implementation checks and their
limits; it does not define future priorities or certify complete JTD fidelity.
See [feature status](FEATURE-STATUS.md), [roadmap](ROADMAP.md), and the
[fixture guide](../rjtd-testdata/README.md).

## Input and Revision Scope

The controlled local native batch contains 67 original/native-PDF pairs and
85 pages. Its checklist has 69 entries; T04/O07 are deferred. These original
inputs and detailed outputs remain private and were not moved to a public corpus.
The 2026-10-07 source-format audit used checkout `3869c48` with equivalent engine
sources on dev `cce9cb2`. That historical documentation checkpoint did not create a new implementation
or release. Later cleanup checks are recorded separately below.

The current 67-case SVG/layer output was checked against the previously
inspected rendering artifacts. PDF text, page dimensions/counts, bounded source
geometry/paint, source spans, and targeted source-profile regressions were
checked within their respective scopes. Updated bold-text, plain fixed-pitch,
and cell-padding cases use their corrected artifacts. Text extraction order
is distinguished from missing characters; printer image tiles and vector
primitive counts are not assumed to equal model object counts.

## What the Results Establish

The existing source-proven work for this input set is implemented, tested,
and present in the dev implementation. It does not prove every native glyph
position, general editing, arbitrary JTD version, or full format semantics.
The implementation CI recorded at
[run 37505779347](https://github.com/OpenJTD/rjtd/actions/runs/37505779347)
passed quality/MSRV gates with 508 tests passed, zero failed, and 31 ignored.
That is a recorded result for `cce9cb2`, not a CI result for uncommitted docs
or proof that ignored private-input assertions ran.

## Known Divergences

| Scope | Verified or retained | Remaining limitation |
| --- | --- | --- |
| P09 notes/ruby | Source note text/link, marker spans and bounded body/ruby typography | Bottom note body/separator not rendered; print/end-mode/gap/baseline fields unresolved |
| Japanese plain flow | Source rows, margins, indentation, font sizes and fixed pitch | General tracking and wrapped-line distribution unresolved |
| Bookmarks | Names and original directory/position bytes | Address units/normalization and navigation undecoded; universal +29 bias contradicted |
| O01/O02 images | Source image/frame size, mode and clearance | Inline baseline and exact fallback-font wrap split differ |
| General rendering | Bounded source profiles and preserved evidence | Exact native font/printer metrics and arbitrary variants not proved |
| Editing | Basic body operations and facade methods | Source-preserving mutation/save/reopen and general regeneration unfinished |

Controlled latest-version results do not resolve historical real-world
pagination/orientation counterexamples. Preserve those experiments and verify
applicability by rule and authoring version. Lack of an isolated contrast limits
field-role proof; it does not prove the format lacks the feature.

## Reproduction and Reporting

Use the [fixture tiers](../rjtd-testdata/README.md#verification-tiers). Report
input identity/provenance, authoring version, command, source revision, executed
assertions, skipped inputs, and divergences. Synthetic bytes prove a parser
contract, imported documents have their own authoring path, and native output
comparisons are scoped experiments. Preserve those distinctions.

Detailed local audit records and source-limit probes remain in the ignored
research workspace. A public checkout must not imply it contains those inputs.
The current quality workflow covers format/check/test/Clippy/docs/WASM/package/
notices/dependency audit/MSRV; fidelity and actual browser runtime require their
own checks. No new native input was created during this document cleanup.

## Interpretation/Rendering Cleanup Checks (2026-10-08)

The optional model `rendering` boundary is now checked separately from the
historical 67-pair format audit above. Current local checks passed: workspace
regressions with `local_` exclusion (483 passed, 0 failed, 17 ignored, 52 filtered),
source-only model tests (43 passed), rendering without bitmap decoding (228 passed,
11 ignored, 24 filtered), both Clippy configurations, workspace/source-only MSRV,
WASM, formatting and warning-free documentation. Source stream/record/image
budgets remain tested without rendering; app page-budget checks retain their
rendering configuration.

An independent consumer verified source-only parsing/candidate queries, the
absence of `DocumentCore` and bitmap dependencies, rendering without bitmaps,
and default rendering. Source model inspection was identical with and without
the renderer for 31 existing native cases. These inputs remain private.
The default output comparison uses 31 cases / 49 pages and the pre-cleanup rjtd
output; it measures behavior preservation, not equality with the original office.
Dynamic printing dates are controlled through the existing exporter API when
comparing PDFs across the midnight boundary.

The model/parser, resource-accounting, app state, snapshot and moved coordination
bodies were compared. No new external dependencies, shared-repository changes or
Windows inputs were introduced. Source candidates retain their bounded profiles
and raw/unknown evidence. WASM naming/API cleanup and subsequent-release
preflight remain separate tasks.

## WASM Naming and Compatibility Review (2026-10-08)

`JtdDocument` is the canonical Rust/viewer name. The Rust `HwpDocument` alias and
generated JS `HwpDocument` constructor are retained. `rjtd.mjs` exports the same
constructor under both names. The generated declarations retain all 266 existing
explicit export names; forwarding bodies are unchanged. No public SDK method was
deleted merely because the repository viewer does not use it.

The generated WASM runtime passed constructor/alias identity, all seven viewer
calls, invalid-input/date errors and retained unsupported-export checks. Seven
existing native JTD/JTT/JTTC cases constructed and rendered all pages. Workspace
checks passed with 485 tests, 0 failures, 17 ignored and 52 filtered. Clippy,
MSRV, WASM, documentation, package and distribution-notice checks passed. CI now
builds and exercises the generated SDK. This does not replace a full browser
session or establish editing/round-trip support. Subsequent-release preflight
remains separate work.
