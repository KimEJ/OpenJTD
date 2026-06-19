# OpenJTD Project Charter

Open-source JTD Rendering Engine and Editor Project for Ichitaro Documents

Open Infrastructure for Ichitaro Documents

## Vision

OpenJTD is an open-source JTD rendering engine and editor project for the
document format used by the Japanese word processor Ichitaro.

This project is not just a file converter.

The final goal is an open-source JTD rendering engine and editor. The current
implementation focus is `rjtd`, a Rust toolset for analysis, parsing, document
modeling, export, and viewer integration. The long-term technical goal is a
practical JTD engine that can support faithful layout rendering and editing.

## Foundational Principle

### Follow rhwp

OpenJTD takes inspiration from the structure and philosophy of the rhwp project
wherever possible.

rhwp is a modern Rust-based document engine for HWP/HWPX documents.

`rjtd` uses rhwp's structure as a reference for the JTD domain.

Therefore, project structure, layer separation, data model design, and test strategy should first be compared with rhwp.

Reuse a proven structure instead of inventing a new one.

## Relationship with rhwp

OpenJTD is not a competitor to rhwp.

It is a sister project that shares the same philosophy.

```text
rhwp
 ├─ HWP
 ├─ HWPX
 └─ Hancom ecosystem

OpenJTD
 ├─ JTD
 ├─ JTDC
 └─ Ichitaro ecosystem
```

In the long term, the following shared ecosystem should be considered.

```text
Document Ecosystem
 ├─ rhwp
 ├─ OpenJTD
 ├─ common-document-model
 ├─ common-renderer
 ├─ common-exporter
 └─ common-viewer
```

Future goals include:

- HWP ↔ JTD common API
- Common document-format IR
- Common Viewer
- Common Exporter
- Apache Tika plugin

## Architecture Policy

The `rjtd` engine keeps the same layered architecture as rhwp.

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
      ├──── Markdown Export
      ├──── HTML Export
      ├──── JSON Export
      └──── Future Renderer
```

Every feature must be implemented through these layers.

No exporter may read source data directly.

Exporters must go through the Document Model.

## Workspace Structure

The top-level workspace keeps whole-project planning and each subproject together.

```text
openjtd-workspace/
├── docs
│   ├── CHARTER.md
│   ├── ARCHITECTURE.md
│   ├── ROADMAP.md
│   └── RHWP-COMPATIBILITY.md
├── rhwp
├── rjtd
├── openjtd-spec
├── openjtd-samples
├── rjtd-testdata
└── openjtd.github.io
```

The top-level `docs` directory contains the project charter, ecosystem planning, rhwp inheritance policy, and long-term roadmap.

`rhwp` is a local external reference clone used to compare `rjtd`'s structure,
API philosophy, and test strategy.

The `rjtd` directory contains the Rust toolset and engine implementation.

## rjtd Engine Repository Structure

The Rust engine repository keeps the rhwp structure as closely as possible.

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

Crates that are not currently used are still created early.

This fixes the intended growth direction of the project.

## Document Model First

The core of rjtd is not the parser.

It is the Document Model.

Every parser must produce a Document Model.

Every exporter must consume the Document Model.

```text
JTD
  ↓
Parser
  ↓
Document Model
  ↓
Exporter
```

## Unknown Preservation Rule

Never discard data that has not yet been analyzed.

Preserve it as:

```text
UnknownRecord
UnknownBlock
UnknownStyle
UnknownObject
```

This prevents data loss during reverse engineering.

## Reverse Engineering Policy

rjtd follows a clean-room reverse-engineering policy.

Allowed:

- File analysis
- Binary structure analysis
- Sample comparison
- Documentation

Forbidden:

- Copying Ichitaro code
- Using private SDKs
- Copyright infringement

## Initial Milestones

### M1: Container Explorer

```text
rjtd streams sample.jtd
```

Goals:

- Analyze CFB
- Obtain the stream list

### M2: Text Extraction

```text
rjtd cat sample.jtd
```

Goal:

- Extract text

### M3: Document Model

```text
rjtd export sample.jtd --format json
```

Goals:

- Paragraph
- TextRun
- Style

Create the structure.

### M4: Markdown Export

```text
rjtd export sample.jtd --format md
```

### M5: Public Specification

Operate a separate repository.

```text
openjtd-spec
```

Record reverse-engineering results in RFC form.

`openjtd-spec` is treated as a peer project to the `rjtd` code. For a closed
format such as JTD, the specification repository may eventually become a larger
asset than the code.

## Long-Term Vision

OpenJTD ultimately provides three things:

1. JTD Editor
2. JTD Engine and Rust Toolset
3. JTD Specification and Document Ecosystem

The goal is not to make "a library that can read JTD", but to build "an open ecosystem that can understand JTD".

## GitHub Organization Model

The initial GitHub organization should use the following structure. The current top-level workspace already reflects this layout.

```text
openjtd/
├── docs
├── rjtd
├── openjtd-spec
├── openjtd-samples
├── rjtd-testdata
└── openjtd.github.io
```

The principle that `openjtd-spec` is a peer project to the `rjtd` code is also
reflected in the organization structure.
