# rjtd

Independent Rust JTD rendering engine and editor project for Ichitaro documents
(`.jtd`, `.jtt`, and `.jttc`).

`rjtd` is an independent implementation within the OpenJTD research ecosystem.
OpenJTD is the shared umbrella for JTD research, specifications, and validation
materials; rjtd and Tika JTD+ retain their own implementation goals. The
current phase focuses on a Rust toolset that builds the components
needed to get there: container inspection, text extraction, document modeling,
export, and viewer integration. The longer-term technical milestone is a
practical JTD engine that can support faithful layout rendering and editing.

Shared work lives in [OpenJTD/spec](https://github.com/OpenJTD/spec) and
[OpenJTD/corpus](https://github.com/OpenJTD/corpus). Organization policy
discussions use `community`, which remains private during initial setup;
`corpus-private` holds only material permitted for collaborator sharing.
The initial shared policy and manifest formats are drafts for joint review.
This repository's implementation, release flow, and local research records
remain independent.

## Current rjtd Components

- CFB/OLE container inventory for `.jtd`, `.jtt`, and `.jttc` files, including
  lenient fallback handling for malformed files.
- Text extraction from observed `/DocumentText` streams.
- Observed `.jttc` `JustCompressedDocument` and `-lh5-` payload support.
- Embedded `SsmgV.01` / `TextV.01` fragment recovery for files without a named
  `/DocumentText` stream.
- Text-oriented Document Model output as plain text, Markdown, JSON, basic HTML,
  and PDF with limited diagnostic layout projections.
- Diagnostic parsers for `/DocumentTextPositionTables`, `/LineMark`,
  `/PageMark`, `/PaperMark`, and object/control marker research.
- WASM bindings and a static browser viewer with page navigation and a text tab.

## Why OpenJTD matters

Ichitaro's proprietary JTD, JTT, and JTTC formats contain documents that may
need to remain readable beyond the software that created them. OpenJTD pairs
an Apache-2.0 Rust implementation (`rjtd`) with public specification notes,
making format research and compatibility work inspectable and reusable for
digital preservation, accessibility, and interoperability.

The project is intentionally conservative with untrusted documents: `rjtd`
separates observed/decoded behavior from experimental research, preserves
unknown structures where possible, and treats parser crashes, hangs, malformed
output, and excessive resource use as security concerns. It is not yet a
complete renderer or editor; see [Project Status](#project-status) and the
[roadmap](docs/ROADMAP.md) for the current limits.

## rjtd Quick Start

```sh
cd rjtd
cargo test --workspace

cargo run -p rjtd-cli -- info path/to/document.jtd
cargo run -p rjtd-cli -- cat path/to/document.jtd
cargo run -p rjtd-cli -- export path/to/document.jtd --format md
cargo run -p rjtd-cli -- export path/to/document.jtd --format html
cargo run -p rjtd-cli -- export path/to/document.jtd --format json
cargo run -p rjtd-cli -- export path/to/document.jtd --format pdf -o output.pdf
```

To refresh the local sample PDF artifacts used for visual regression checks,
run this from the repository root:

```sh
scripts/regenerate-pdf-output.sh
```

## Repository Layout

- [`rjtd/`](rjtd/) - Rust toolset and workspace for the current OpenJTD
  components: core engine, CLI, exporters, WASM wrapper, and test helpers.
- [`openjtd-spec/`](openjtd-spec/) - public specification notes and RFC records.
- [`docs/`](docs/) - charter, architecture, roadmap, and research policy.
- [`openjtd-samples/`](openjtd-samples/) - redistributable sample/output artifacts.
- [`rjtd-testdata/`](rjtd-testdata/) - test fixtures.
- [`openjtd.github.io/`](openjtd.github.io/) - static WASM viewer and GitHub Pages assets.

## Documentation

- [`rjtd/README.md`](rjtd/README.md) describes the `rjtd` Rust workspace, CLI,
  exporter, and diagnostic command surface.
- [`openjtd-spec/README.md`](openjtd-spec/README.md) indexes the specification work and
  RFC process.
- [`docs/CHARTER.md`](docs/CHARTER.md) defines the long-term vision and research policy.
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) defines engine layers and model boundaries.
- [`docs/ROADMAP.md`](docs/ROADMAP.md) owns the current milestone summary, next
  priorities, and completion criteria.
- [`TODO.md`](TODO.md) retains the detailed backlog and historical experiments;
  completed diagnostic tasks do not imply decoded format semantics.
- [`rjtd-testdata/README.md`](rjtd-testdata/README.md) explains fixture provenance
  and the distinction between portable checks and local reference validation.

## Design Reference

OpenJTD's repository layout and engine boundaries take inspiration from the
`rhwp` project. JTD source evidence governs its internal model; full rhwp Studio
compatibility is not a required milestone. See the
[rhwp reference and integration scope](docs/RHWP-COMPATIBILITY.md).

## Project Status

OpenJTD is in the reverse-engineering and component-building stage. It is not
yet a complete JTD rendering engine or editor, and the `rjtd` APIs, data model,
and diagnostic commands may still change.

Text extraction works for observed files, but full paragraph semantics, layout
fidelity, styles, tables, ruby annotations, images, and native editing behavior
are incomplete. PDF and SVG combine text-oriented fallback layout with limited
source- or reference-backed projections; native layout fidelity is not guaranteed.
Basic body-text editing exists, but structure-preserving editing and saving
back to JTD remain unfinished.

The next goal is reproducible reading and rendering of horizontal body text and
a simple table on one page. See the [roadmap](docs/ROADMAP.md#next-priorities) for
acceptance criteria and the path toward broader layout and editing support.

## Translations

English is the default documentation language. Japanese translations use
`*.ja.md`.

## Contributing and Security

See [CONTRIBUTING.md](CONTRIBUTING.md) for the Apache-2.0 and DCO contribution
terms, clean-room research rules, pull request flow, and sample provenance
requirements. Report possible vulnerabilities privately by following
[SECURITY.md](SECURITY.md); do not disclose vulnerability details in a public
issue or pull request.

## Support

Community help is best effort. For separately scoped paid engineering inquiries,
contact [the maintainer on Upwork](https://www.upwork.com/freelancers/eojinkim).
See [SUPPORT.md](SUPPORT.md) for boundaries; security reports follow
[SECURITY.md](SECURITY.md).

## License

OpenJTD-authored source code and documentation are licensed under the
[Apache License, Version 2.0](LICENSE).

Bundled sample and test-input documents, and other third-party materials, may
be subject to separate rights or terms. This license notice does not grant
rights to those materials.

Generated output may be distributed only when the rights in its input material
allow it; Apache-2.0 does not grant rights in input content represented by that
output. “Ichitaro”, “一太郎”, “JustSystems”, and other third-party names are
used descriptively to identify the document format or compatibility target, not
to imply affiliation with or endorsement by their respective owners. See
[THIRD_PARTY.md](THIRD_PARTY.md) for the boundary for local reference material.
