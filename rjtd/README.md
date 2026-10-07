# rjtd

Rust toolset and document-engine workspace for OpenJTD

## Role

`rjtd` is the Rust toolset for OpenJTD. It analyzes and processes the JTD
document format used by the Japanese word processor Ichitaro, and it provides
the current parser, model, export, CLI, WASM, and app-core integration
components.

This folder is the implementation workspace in `OpenJTD/rjtd`. OpenJTD is the
shared research umbrella; rjtd and Tika JTD+ remain independent implementations.
Shared RFC review takes place in [OpenJTD/spec](https://github.com/OpenJTD/spec).

The overall project charter and ecosystem plan follow the top-level [docs/CHARTER.md](../docs/CHARTER.md).

## 0.0.1 Developer Preview

The dated 0.0.1 release is an experimental developer preview. Its published
packages are `rjtd-core`, `rjtd-model`, `rjtd-export`, `rjtd-cli`, and
`rjtd-wasm`; `rjtd-testkit` remains an internal workspace crate. This source
checkout may contain later unreleased development. See
[CHANGELOG.md](CHANGELOG.md) for the release scope and
[RELEASING.md](RELEASING.md) for the required publication order.

Observed `.jtd`, `.jtt`, and `.jttc` files are supported to different degrees.
The implementation is not a complete Ichitaro format specification. Values
named `Candidate`, `Unknown`, or `Diagnostic`, and JSON fields marked
`decoded: false`, retain reverse-engineering evidence without claiming final
semantics. All public APIs and command output schemas may change in later
0.0.x releases.

## JTD Native Engine and Design References

JTD source data and reproducible observations govern interpretation and the
model. Other office implementations are optional references when they help
resolve a concrete problem and their use is permitted. No external project's
model, dependency choices, or API parity is an acceptance criterion.
See the [architecture](../docs/ARCHITECTURE.md).

## Architecture Policy

`rjtd` separates source parsing, model ownership, and output through these layers.

```text
Document File
      │
      ▼
Container Layer
      │
      ▼
Stream Layer
      │
      ▼
Record Layer
      │
      ▼
Document Model
      │
      ├──── Plain Text / Markdown Export
      ├──── HTML Export
      ├──── JSON Export
      └──── App Core / SVG / PDF Export
```

Every feature must be implemented through these layers. No exporter may read source data directly. Exporters must go through the Document Model.

`rjtd-model::DocumentCore` provides loading, page queries, SVG/HTML rendering,
layer diagnostics, and basic body-text editing, search, selection, clipboard,
and snapshots. Source byte/unit spans are retained where parsed. The
`rjtd-wasm::JtdDocument` wrapper exposes the existing viewer browser API.
Renaming it and removing unused compatibility methods are separate code work
requiring consumer and generated-binding checks. API count is not a JTD
feature-completion criterion.

Many advanced formatting, table/cell, object, header/footer, and note methods
still return defaults, no-hit results, or no-op results. Some setting methods
return success without persisting decoded JTD settings. These compatibility
surfaces do not establish native editing or save support. HWP/HWPX export
returns empty bytes; `exportHwpVerify` reports that conversion is unimplemented.

Basic document HTML export is implemented in `rjtd-export` and the CLI, including
paragraph text and ruby markup. App-core rich HTML clipboard methods remain
limited; basic body selection can produce escaped paragraph text. For current
milestones and acceptance criteria, see the [roadmap](../docs/ROADMAP.md).

## Document Model First

The core of rjtd is not the parser. It is the Document Model.

Every parser must produce a Document Model. Every exporter must consume the Document Model.

## Unknown Preservation Rule

Never discard data that has not yet been analyzed.

```text
UnknownRecord
UnknownBlock
UnknownStyle
UnknownObject
```

This prevents data loss during reverse engineering.

## Default Resource Limits

The public parser surfaces reject source input larger than 64 MiB. LH5 members
reject declared output larger than 256 MiB; above a 1 MiB allowance, output must
also remain within 256 times the packed member size. Browser canvas rendering is
limited to 16,384 pixels per dimension and 64 MiPixels in total.

These are pre-stable safety ceilings, not compatibility guarantees. Stream,
record, embedded-image, and page construction use one `ParseLimits`-backed
resource budget for each limits-aware document load. The default budget caps
1,024 distinct CFB stream paths and 64 MiB of their cumulative accounted bytes;
65,536 retained frame or embedding records and 64 MiB of their record bytes;
1,024 retained images and 64 MiB of their retained payload/envelope bytes;
and 65,536 pages with 1 Mi retained page lines. Embedded-image width, height,
and cumulative header-derived pixels are also bounded at 16,384, 16,384, and
64 MiPixels.

Strict CFB streams are charged from declarations; lenient recovery uses bytes
reachable through the sector chain. Duplicate paths are counted once, and
actual reads are checked against that accounting. Image dimensions
come from PNG, GIF, BMP, or JPEG headers before retained payload/envelope
clones. This Rust model path does not decode or retain a bitmap, so these
limits do not claim to bound a downstream decoded-image allocation. Process
untrusted documents in an appropriately constrained environment and report
possible bypasses through the root [security policy](../SECURITY.md).

## Workspace Layout

```text
rjtd/
├── crates
│   ├── rjtd-core
│   ├── rjtd-model
│   ├── rjtd-export
│   ├── rjtd-cli
│   ├── rjtd-wasm
│   └── rjtd-testkit
├── docs
├── samples
├── fuzz
├── tests
└── tools
```

All six workspace crates have implementation or fixture-support roles.
Crate tests live under `crates/`; `docs/` contains the CLI and research
references. `fuzz/`, `samples/`, `tests/`, and `tools/` retain their actual
contents; directory presence alone does not establish an executable program.

## Commands

```sh
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## CLI and Development Documentation

Use [CLI diagnostics](docs/CLI-DIAGNOSTICS.md) for the command reference.
From this directory, common commands are:

```sh
cargo run -p rjtd-cli -- --help
cargo run -p rjtd-cli -- info path/to/document.jtd
cargo run -p rjtd-cli -- cat path/to/document.jtd
cargo run -p rjtd-cli -- export path/to/document.jtd --format json
cargo run -p rjtd-cli -- export path/to/document.jtd --format pdf -o output.pdf
```

Current scope is in [feature status](../docs/FEATURE-STATUS.md), executed checks
and divergences in [validation](../docs/VALIDATION.md), future sequence in the
[roadmap](../docs/ROADMAP.md), and actionable tasks in [TODO](../TODO.md).
Detailed source/implementation research is indexed [here](docs/research/README.md).
