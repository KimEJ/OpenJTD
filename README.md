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
