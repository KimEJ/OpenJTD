# OpenJTD Project Charter

## Shared Research and Independent Implementations

OpenJTD is a shared project for open research into Ichitaro's JTD, JTT, and
JTTC formats, specifications, observations, and validation materials. Its
shared knowledge should be independently verifiable and useful to more than
one implementation.

`rjtd` is the independent Rust engine in this repository. Its long-term goal
is faithful JTD reading, rendering, editing, and preservation. Tika JTD+
retains its own extraction and integration goals. Neither implementation's
API, document model, output format, or feature list defines the shared format.

## JTD Source First

JTD source data and reproducible observations govern interpretation and the
model. Preserve original text flow, boundary declarations, records, source
ranges, object relationships, and unknown data. A ruled-line region need not
become a rigid table container in the interpretation core.

Other office implementations may be consulted when they help resolve a
concrete problem and their use is permitted. Their architecture, addressing,
dependencies, and API coverage are not development obligations or completion
criteria. Reuse existing project utilities before adding new machinery;
new dependencies require explicit approval.

## Interpretation and Rendering

The [architecture](ARCHITECTURE.md) separates source interpretation from
rendering and application state:

```text
Document bytes -> container / streams / records -> owned document model
                                                    |-> extraction / inspection
                                                    |-> rendering -> SVG / PDF / browser
                                                    `-> supported editing / save
```

Source coordinates, units, writing direction, formatting instructions, line
and page ranges, anchors, and paint order belong to interpretation when
supported by evidence. Glyph measurement, substitute fonts, fallback layout,
DPI conversion, and output painting belong to rendering. Consumers use model
data; exporters and renderers do not reopen the source container.

This is a target boundary. Current `rjtd-model` still contains parsing/model
integration, rendering, and app state. Code separation requires its own
behavior-preserving migration and regression checks.

## Preservation and Evidence

Unknown records, blocks, styles, objects, and raw data remain available.
`Candidate`, `Unknown`, `Diagnostic`, `decoded:false`, and reference-backed
projections retain their evidence limits even when an output looks plausible.
Do not infer format semantics from a parser's successful output or a single
visual match.

Establish an Ichitaro 2026 baseline using native documents with one condition
changed at a time. Record older-version applicability separately by rule and
version. Controlled inputs and authorized real-world documents serve different
validation purposes. Keep observations, hypotheses, counterexamples, and
implementation behavior distinguishable.

## Research and Rights

Follow [CONTRIBUTING.md](../CONTRIBUTING.md) for the implementation's current
research rules. Public metadata, authorized document analysis, controlled
experiments, and behavior observable through normal application use provide
evidence. Reference artifacts and external code retain their own terms.
This charter does not grant permission to copy proprietary code, use private
SDKs, reconstruct implementation logic from decompiler output, or distribute
inputs or derived output without rights.

Shared policy and data formats remain subject to recorded joint review.
A local implementation document does not establish an organization-wide
agreement. Preserve input provenance, sharing basis, and allowed audience;
private storage alone is not permission to share.

## Repository Responsibilities

| Repository | Responsibility |
| --- | --- |
| `OpenJTD/community` | Shared policy discussions, decisions, and agreed organization records; private during initial setup |
| `OpenJTD/spec` | Public format RFCs, observations, reproduction procedures, and validation criteria |
| `OpenJTD/corpus` | Redistributable inputs and permitted public manifests |
| `OpenJTD/corpus-private` | Inputs permitted for sharing with named collaborators |
| `OpenJTD/rjtd` | Independent Rust implementation, release procedures, and local research records |

Shared RFC 0001/0003 imports retain their recorded historical status. Format
claims are prepared separately from [rjtd implementation records](../rjtd/docs/research/README.md).
Apache-2.0 is the default for jointly authored work; external documents retain
their original terms and individual provenance records.

## Workspace and Milestones

The `rjtd/` Rust workspace contains core, model, export, CLI, WASM, and testkit
crates. Top-level `docs/` owns implementation direction and boundaries;
`openjtd-spec/` contains local format drafts; samples, testdata, and viewer
sources retain their separate purposes. Optional external checkouts are not
required parts of the architecture or runtime.

Current scope is in [feature status](FEATURE-STATUS.md), evidence in
[validation](VALIDATION.md), and future acceptance in the [roadmap](ROADMAP.md):
container access, text extraction, model/preservation, exports, public format
research, and the WASM viewer. Completion is tied to source-backed behavior,
reproducible checks, and stated scope. Structure-preserving editing requires
edit/save/reopen verification. External integration is scoped only when a
consumer and supported operations are identified.
