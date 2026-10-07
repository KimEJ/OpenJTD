# rjtd Roadmap

This is the implementation roadmap for rjtd. It orders the work toward faithful
JTD reading, rendering, editing, and preservation. Current capabilities live
in [feature status](FEATURE-STATUS.md), measurements and limits in
[validation](VALIDATION.md), and executable tasks in [TODO](../TODO.md).
The [charter](CHARTER.md) and [architecture](ARCHITECTURE.md) define the scope.

## Scope

Preserve JTD source flow, ranges, relationships, and unknown data. Interpretation
owns the meaning of source instructions; rendering owns font measurement,
fallback layout, and painting. Source coordinates, units, writing direction,
formatting, and object order remain part of interpretation when evidenced.

This local plan does not change shared policies, move documents to other
repositories, or require implementation integration. Future shared review can
use independently reproducible local observations. It does not block work
permitted by the current project rules. External API parity, a common office
model, and other-format conversion are not completion criteria.

## Next Priorities

### 1. Separate the Interpretation Core

Split mixed source recognition and rendering functions at their evidence result.
Keep source text, records, relationships, unknown bytes, and raw source ranges
in the owned model. Preserve current callers during migration; choose crate
boundaries only after the dependencies are clear.

**Complete when:** parsing and model inspection build and run without a renderer
or font initialization, and source/model/resource-limit regressions remain
unchanged. Renaming modules alone does not meet this gate.

### 2. Make Local Interpretation Checks Reproducible

Use existing source spans, diagnostics, and test patterns to record inputs,
commands, implementation revision, expected observations, and counterexamples.
Distinguish native-authored, imported, and synthetic inputs. Start with text
extraction and low-level flow. A permitted minimum fixture set must reproduce
checks from another checkout; private checks remain a separately reported tier.
No organization-wide observation schema is defined in this step.

**Complete when:** the selected interpretation checks reproduce without
untracked private files, failures and unsupported cases are visible, and each
rule's input/version scope is traceable. A green test that skipped its input
is not independent evidence.

### 3. Establish the Rendering Boundary

Move font measurement, output-unit conversion, fallback line/page layout,
SVG/page-layer construction, and backend paint behind the model boundary.
Keep saved source line/page instructions distinct from computed layout. The
application facade coordinates loading, selection, editing, and render results.
Review wrapper names and unused compatibility methods against actual consumers.

**Complete when:** the core has no rendering dependency, renderers do not decode
raw input, existing text/source/model output is preserved, and SVG/layer/native
PDF comparisons plus WASM/browser contracts cover the moved behavior.

### 4. Extend Rules Beyond Verified Profiles

Resolve remaining source interpretations with discriminating observations,
then expand across independent documents and authoring versions. Keep the
Ichitaro 2026 baseline separate from historical applicability. Extend flow,
styles, page instructions, notes, and objects without fitting reference-PDF
coordinates or promoting unknown fields because a render looks plausible.

**Complete per rule when:** source evidence, independent contrasts, rejected
alternatives, known limits, and regressions explain its claimed scope. Text
extraction, flow, and visual fidelity have separate acceptance checks. Robustness
and executable malformed-input fuzzing accompany extension work.

### 5. Build Structure-Preserving Editing and Save

Mutate supported JTD structures while retaining unknown data and relationships.
Build editing on established source semantics; add external adapters only for
identified consumers and supported operations.

**Complete per operation when:** edit/save/reopen checks preserve intended
changes and unaffected source/unknown content. Returning success, changing a
fallback paragraph, or exposing a method is insufficient.

## Dependencies and Progress

Core separation precedes a final rendering-component boundary. Reproducible
interpretation checks can grow during that migration. Evidence-driven fixes
need not wait for all refactoring, but must retain the same source/renderer
boundary. Editing/save follows proven structures rather than an external UI.

Each step records executed and unavailable checks, residual risks, and the
source revision. Use the [architecture migration gates](ARCHITECTURE.md#migration-and-regression-gates).
Existing M1–M6 identifiers remain in the feature table and historical backlog;
they are not renumbered into this forward work sequence.
