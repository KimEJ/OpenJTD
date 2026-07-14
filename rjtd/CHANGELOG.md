# Changelog

This file records user-visible changes to the OpenJTD Rust workspace.

## 0.0.1 - 2026-07-14

### Added

- Initial crates.io releases for `rjtd-core`, `rjtd-model`,
  `rjtd-export`, `rjtd-cli`, and `rjtd-wasm` under Apache-2.0.
- Low-level CFB and JTD-family stream parsing with evidence-preserving
  diagnostics.
- Experimental document model, text-first rendering, export, CLI, and browser
  binding surfaces.
- Default input, LH5 decompression, and browser canvas resource ceilings for
  untrusted documents.
- Support exercised against observed `.jtd`, `.jtt`, and `.jttc` files.

### Known limitations

- Format coverage is evidence-driven and incomplete across Ichitaro versions.
- Values and output fields marked `decoded: false`, `Candidate`, `Unknown`, or
  `Diagnostic` are not final semantic interpretations.
- Stream, record, embedded-image, and page-level budgets remain incomplete.
- Advanced layout, styles, tables, embedded objects, and editing use
  conservative fallbacks and are not lossless round-trip guarantees.
- All public APIs and command output schemas may change in later 0.0.x
  releases.

`rjtd-testkit` remains an internal workspace crate and is never published.
