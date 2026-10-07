# rjtd Feature Status

This describes the development source tree, not a promise about every file or
the published 0.0.1 package. The [changelog](../rjtd/CHANGELOG.md) records dated
releases; [validation](VALIDATION.md) states what was actually checked. Future
work belongs in the [roadmap](ROADMAP.md) and [TODO](../TODO.md).

## Status Terms

Implemented means a code path exists for its stated profile. Verified requires
a named input set and executed checks. Candidate, diagnostic, fallback,
`decoded:false`, and reference-backed remain limits on interpretation.
Method presence, a generated PDF, or a skipped local test is not fidelity proof.

## Original Milestones

| ID | Area | Current scope | Remaining boundary |
| --- | --- | --- | --- |
| M1 | Container exploration | Observed JTD/JTT/JTTC CFB inventories and bounded malformed-FAT recovery | Not universal container or version coverage |
| M2 | Text extraction | Named and embedded text, ruby base, observed LH5 inner document | General control/paragraph/style semantics remain incomplete |
| M3 | Document model | Source flow/spans, raw/unknown preservation, model candidates and app facade | Rendering/app state still coupled; structure-preserving editing/save unfinished |
| M4 | Exports | Text, Markdown, diagnostic JSON, basic HTML and native PDF | HTML is not full layout; PDF uses bounded source/fallback projections |
| M5 | Local format research | Format drafts separated from implementation records; RFC 0004 number reserved | Claim-level evidence and independent review remain required; shared documents untouched |
| M6 | WASM viewer | Browser-local selection/drop, page navigation, SVG and text | Not a full editor; runtime coverage and deployment availability are separate checks |

## Bounded Development Profiles

Source-backed candidates cover paragraph indentation/margins/pitch, font size
and character/font selections; ruled-line text/borders and blank/merged spans;
saved fields, TOC labels/leaders, running regions, paper orientation/cover;
vertical text/tatechuyoko; one PNG frame, simple figure order/fills, saved
JSEQ/GCI equation characters; and JTT/JTTC auxiliary inner streams.

These are bounded rules, not general decoding. Cached numbering/fields/TOC
content does not establish generation or editing. Exact native fonts, glyph
metrics, arbitrary objects/styles, and historical-version behavior remain
limited. See [known divergences](VALIDATION.md#known-divergences).

## Implementation Boundaries

`rjtd-core` provides low-level evidence. `rjtd-model` supports source-only
parsing/inspection with `--no-default-features`; optional `rendering` provides
`DocumentCore` and app/rendering state. Defaults retain bitmap rendering.
Bounded source candidate APIs expose raw units, font identity, sections/running
policy and logical row ranges without renderer initialization. `rjtd-export`
provides serialization/PDF. `rjtd-wasm` uses `JtdDocument` with the existing Rust/JS `HwpDocument` compatibility
names. The viewer uses the canonical constructor through `rjtd.mjs`; public
compatibility exports remain after consumer review. See the [model build configurations](../rjtd/crates/rjtd-model/README.md#build-configurations).
