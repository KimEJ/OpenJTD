# rjtd WORKSPACE GUIDE

**Generated:** 2026-07-01 08:54:17 KST
**Commit:** 92ad606
**Branch:** dev

## OVERVIEW
`rjtd/` is the active OpenJTD Rust workspace. It processes Ichitaro
`.jtd`, `.jtt`, and `.jttc` files through a rhwp-inspired layered engine.

## STRUCTURE
```text
rjtd/
|-- crates/rjtd-core      # low-level container/stream/record/text parsers
|-- crates/rjtd-model     # document model, DocumentCore, app-core fallbacks
|-- crates/rjtd-export    # text/html/markdown/json/pdf exporters
|-- crates/rjtd-cli       # diagnostics and export binary
|-- crates/rjtd-wasm      # wasm-bindgen wrapper surface
|-- crates/rjtd-testkit   # shared fixture helpers; test=false
|-- docs/ samples/ tests/ fuzz/ tools/
`-- target/ tmp/          # build/scratch output
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Workspace members/lints | `Cargo.toml` | Edition 2024, resolver 3, `unsafe_code = "forbid"`. |
| Architecture policy | `README.md` | Layered engine; compare rhwp first. |
| Container and stream access | `crates/rjtd-core/src/container.rs` | Standard `cfb` plus lenient fallback. |
| Text/layout/style evidence | `crates/rjtd-core/src/document_text.rs`, `layout_mark.rs`, `style_stream.rs` | Diagnostic parsers before model promotion. |
| Main model surface | `crates/rjtd-model/src/lib.rs` | `Document`, `DocumentCore`, decoded-false evidence. |
| Export behavior | `crates/rjtd-export/src/lib.rs` | Export from model only; PDF safety checks live here. |
| CLI commands | `crates/rjtd-cli/src/main.rs` | Broad diagnostic dispatcher; check before adding a new command. |
| WASM compatibility | `crates/rjtd-wasm/src/lib.rs` | `HwpDocument` wrapper mirrors rhwp Studio expectations. |
| Integration tests | `crates/rjtd-cli/tests/streams.rs` | Main standalone integration test file. |

## CODE MAP
| Surface | Type | Location | Role |
|---------|------|----------|------|
| `rjtd_core` modules | crate API | `crates/rjtd-core/src/lib.rs` | Re-export parser and stream modules. |
| `DocumentCore` | app core | `crates/rjtd-model/src/lib.rs` | Page info, SVG/HTML/layer tree, editing fallbacks. |
| `parse_document` | parser/model bridge | `crates/rjtd-model/src/lib.rs` | Builds model-owned evidence from core parsers. |
| `to_pdf_with_file_name` | exporter | `crates/rjtd-export/src/lib.rs` | PDF output with sample-aware diagnostics. |
| `run` | CLI dispatcher | `crates/rjtd-cli/src/main.rs` | Routes `streams`, `info`, style/layout diagnostics, `export`. |
| `HwpDocument` | WASM API | `crates/rjtd-wasm/src/lib.rs` | Browser wrapper named for rhwp compatibility. |

## CONVENTIONS
- Keep the layer order: container -> stream -> record -> document model -> export/app core.
- Every parser produces model data; every exporter consumes model data.
- Preserve unproven source information as `Unknown*`, diagnostic candidates, or
  `decoded:false` evidence instead of hiding it.
- Use rhwp precedent for structure, dependencies, and testing before adding a new pattern.
- Test names are behavior-first: `reads_*`, `detects_*`, `parses_*`,
  `reports_*`, `rejects_*`, `falls_back_to_*`.

## ANTI-PATTERNS
- Do not let `rjtd-export` inspect raw CFB streams to patch output.
- Do not infer true layout, table, style, paint, or page semantics from a single sample.
- Do not hardcode sample names unless the surrounding evidence explicitly marks a fallback.
- Do not weaken PDFKit/Preview safety checks to make PDFs render in only one engine.
- Do not treat `target/`, `tmp/`, or regenerated PDFs as source.

## COMMANDS
```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

cargo run -p rjtd-cli -- info path/to/document.jtd
cargo run -p rjtd-cli -- cat path/to/document.jtd
cargo run -p rjtd-cli -- export path/to/document.jtd --format pdf -o output.pdf
```

## NOTES
- `rjtd-testkit` has `[lib] test = false`; it is fixture support, not a direct test crate.
- `rjtd-model/src/lib.rs`, `rjtd-export/src/lib.rs`, and
  `rjtd-cli/src/main.rs` are intentionally large central files. Prefer
  behavior-preserving, well-tested edits over opportunistic splitting.
