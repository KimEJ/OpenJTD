# Development Tasks

Execution order follows the [roadmap](docs/ROADMAP.md). Current capabilities
and original M1–M6 IDs are in [feature status](docs/FEATURE-STATUS.md);
measurements and limits are in [validation](docs/VALIDATION.md).
The [complete earlier backlog](rjtd/docs/research/legacy-backlog.md) preserves
all completed experiments, open investigations, and counterexamples. An open
historical item remains unresolved unless its own evidence establishes completion.

## Interpretation Core

- [ ] Classify mixed source recognition, rendering, and app-state functions.
- [ ] Expose source recognition results through the owned model, preserving flow, spans, raw/unknown data, and resource limits.
- [x] Verify parsing/model inspection without font or rendering initialization.

## Reproducible Interpretation Checks

- [ ] Select permitted minimum fixtures using existing test/diagnostic patterns.
- [ ] Record authoring path/version, identity, command, revision, expected observations, and counterexamples.
- [ ] Make selected checks run from another checkout; report private, missing, skipped, and unsupported cases separately.

## Rendering and Application Boundary

- [ ] Separate saved source line/page instructions from computed fallback layout.
- [ ] Move font measurement, output units, SVG/page-layer construction, and paint behind the model boundary.
- [ ] Review HwpDocument naming, generated bindings, viewer callers, and unused compatibility methods together.
- [ ] Preserve source/model behavior, output regressions, and WASM/browser contracts during moves.

## Evidence and Coverage

- [ ] Decode note-area print/placement/separator fields with discriminating source evidence.
- [ ] Resolve Japanese tracking/distribution, bookmark coordinate normalization, and inline-baseline limits.
- [ ] Expand rules to independent documents and versions; preserve historical pagination/orientation counterexamples.
- [ ] Add executable malformed-input fuzz targets and broaden resource-boundary regressions where justified.

## Editing and Release Preparation

- [ ] Verify each structure-preserving edit through save/reopen with unknown data intact.
- [ ] Review rich clipboard, selection, and hit-test support against established JTD semantics.
- [ ] Update release tooling currently fixed to the initial 0.0.1/unallocated-name procedure before preparing a later version.

Document reorganization is complete independently of these code tasks. Shared
policy/schema review and repository migration are outside this local pass.
