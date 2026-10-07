# openjtd.github.io

This directory contains OpenJTD's static WASM viewer for `.jtd`, `.jtt`, and
`.jttc` documents. It supports local file selection/drop, SVG page navigation,
and a plain-text tab. Document contents are processed in the browser.

The viewer uses `rjtd-wasm::JtdDocument` and inherits the model's fallback and
diagnostic rendering limits. It is not a full editor or a guarantee of native
Ichitaro layout fidelity. See [feature status](../docs/FEATURE-STATUS.md),
[validation](../docs/VALIDATION.md), and the [roadmap](../docs/ROADMAP.md).

## Build and Deployment

The [deployment workflow](../.github/workflows/deploy-viewer.yml) is the source
of truth for the build and asset-copy steps. It builds with
`wasm-pack build --target web rjtd/crates/rjtd-wasm`, replaces `pkg/` with the
generated package, includes distribution notices, and publishes to GitHub Pages
from `main` to <https://openjtd.github.io/rjtd/>. Workflow dispatch is also
restricted to `main` for the build. GitHub Pages URLs do not redirect after
a repository transfer, so the former personal-account viewer URL is obsolete.

`pkg/` is generated output. A source checkout alone may not contain a runnable
viewer until the WASM package is built and copied. Earlier Cloudflare deployment
experiments are historical; the checked-in workflow now targets GitHub Pages.
Check the deployment run separately to establish hosted availability.

## Verification

The Rust quality workflow checks the WASM target and viewer source contracts.
Those checks do not run an actual browser session. Runtime regression coverage and rendering-limit warnings remain open
implementation work in [TODO](../TODO.md#rendering-and-application-boundary).
The original M6 scope is recorded in feature status.

The viewer imports the canonical constructor from `rjtd.mjs`. It aliases the
generated SDK `HwpDocument` class, preserving direct SDK and legacy Rust callers.
