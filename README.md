# OpenJTD

Open-source JTD rendering engine and editor project for Ichitaro documents
(`.jtd`, `.jtt`, and `.jttc`).

OpenJTD aims to become an open-source JTD rendering engine and editor. The
current phase focuses on `rjtd`, a Rust toolset that builds the components
needed to get there: container inspection, text extraction, document modeling,
export, and viewer integration. The longer-term technical milestone is a
practical JTD engine that can support faithful layout rendering and editing.

## Current rjtd Components

- CFB/OLE container inventory for `.jtd`, `.jtt`, and `.jttc` files, including
  lenient fallback handling for malformed files.
- Text extraction from observed `/DocumentText` streams.
- Observed `.jttc` `JustCompressedDocument` and `-lh5-` payload support.
- Embedded `SsmgV.01` / `TextV.01` fragment recovery for files without a named
  `/DocumentText` stream.
- Minimal Document Model output as plain text, Markdown, JSON, and text-oriented
  PDF.
- Diagnostic parsers for `/DocumentTextPositionTables`, `/LineMark`,
  `/PageMark`, `/PaperMark`, and object/control marker research.
- WASM wrapper support used by early viewer integration experiments.

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

## Maintainer automation

If API credits become available, maintainers plan to use them for focused,
auditable assistance with:

- PR review against the layered architecture, public specification, and
  conservative decoded/experimental boundaries.
- Regression triage and corpus minimization without moving private,
  proprietary, or redistribution-restricted documents into public services.
- English/Japanese specification synchronization checks.
- Security and resource-limit reviews for untrusted documents.
- Release-readiness automation for tests, CI, documentation, and release
  metadata.

This plan does not assume selection for any support program or receipt of
credits; maintainers retain final review and merge decisions.

## rjtd Quick Start

```sh
cd rjtd
cargo test --workspace

cargo run -p rjtd-cli -- info path/to/document.jtd
cargo run -p rjtd-cli -- cat path/to/document.jtd
cargo run -p rjtd-cli -- export path/to/document.jtd --format md
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
- [`openjtd.github.io/`](openjtd.github.io/) - future project site.

## Documentation

- [`rjtd/README.md`](rjtd/README.md) describes the `rjtd` Rust workspace, CLI,
  exporter, and diagnostic command surface.
- [`openjtd-spec/README.md`](openjtd-spec/README.md) indexes the specification work and
  RFC process.
- [`docs/CHARTER.md`](docs/CHARTER.md), [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md),
  and [`docs/ROADMAP.md`](docs/ROADMAP.md) explain the project direction.

## Design Reference

OpenJTD's repository layout and engine boundaries take inspiration from the
`rhwp` project structure, adapted for JTD.

## Project Status

OpenJTD is in the reverse-engineering and component-building stage. It is not
yet a complete JTD rendering engine or editor, and the `rjtd` APIs, data model,
and diagnostic commands may still change.

Text extraction works for observed files, but full paragraph semantics, layout
fidelity, styles, tables, ruby annotations, images, and native editing behavior
are incomplete. PDF and SVG output should be treated as text-oriented fallback
output, not native layout reproduction.

## Translations

English is the default documentation language. Japanese translations use
`*.ja.md`.

## Contributing and Security

See [CONTRIBUTING.md](CONTRIBUTING.md) for the Apache-2.0 and DCO contribution
terms, clean-room research rules, pull request flow, and sample provenance
requirements. Report possible vulnerabilities privately by following
[SECURITY.md](SECURITY.md); do not disclose vulnerability details in a public
issue or pull request.

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
