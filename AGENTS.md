# PROJECT KNOWLEDGE BASE

**Generated:** 2026-07-01 08:54:17 KST
**Commit:** 92ad606
**Branch:** dev

## OVERVIEW
OpenJTD is a reverse-engineering and Rust implementation workspace for Ichitaro
JTD/JTT/JTTC documents. The active product code lives in `rjtd/`; `rhwp/` is a
local reference project with its own rules and mixed Rust/Web platform surfaces.

## STRUCTURE
```text
rjtd/
|-- rjtd/                 # OpenJTD Rust workspace: core, model, export, CLI, WASM
|-- rhwp/                 # ignored local reference clone; follow rhwp/AGENTS.md
|-- openjtd-spec/         # public RFC/spec records, English + Japanese
|-- docs/                 # charter, architecture, roadmap, compatibility notes
|-- scripts/              # repo-local helpers, especially PDF artifact refresh
|-- openjtd-samples/      # redistributable samples and generated pdf-output
|-- rjtd-testdata/        # fixtures; local-samples may be private/untracked
|-- openjtd.github.io/    # Pages viewer; pkg/ is copied WASM output
|-- third-party/          # vendored external references
`-- tmp*/ .cargo-home/    # local scratch/cache; do not document as source
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| JTD parser/model/export work | `rjtd/` | Use `rjtd/AGENTS.md`; all implementation should stay here. |
| Architecture and policy | `docs/ARCHITECTURE.md`, `docs/RHWP-COMPATIBILITY.md` | Exporters consume model data only; preserve unknown data. |
| Public format records | `openjtd-spec/rfc/` | Keep English originals and Japanese translations aligned. |
| Local sample PDF refresh | `scripts/regenerate-pdf-output.sh` | Builds `rjtd-cli`, then writes `openjtd-samples/pdf-output/`. |
| Viewer deploy | `.github/workflows/deploy-viewer.yml` | `wasm-pack build --target web rjtd/crates/rjtd-wasm`. |
| Branch flow guard | `.github/workflows/pr-target-guard.yml` | Development PRs target `dev`; only `dev -> main` goes to `main`. |
| rhwp reference behavior | `rhwp/` | Ignored by root git; do not copy its code into `rjtd`. |

## CODE MAP
| Symbol / Surface | Type | Location | Role |
|------------------|------|----------|------|
| `rjtd` workspace | Cargo workspace | `rjtd/Cargo.toml` | Six-crate Rust implementation root; edition 2024. |
| `rjtd-core` | crate | `rjtd/crates/rjtd-core/src/lib.rs` | Container, stream, record, text, style, and layout-mark parsing. |
| `DocumentCore` | model/app-core | `rjtd/crates/rjtd-model/src/lib.rs` | Main model-owned app surface and fallback rendering API. |
| `to_pdf`, `to_json`, `to_markdown` | exporters | `rjtd/crates/rjtd-export/src/lib.rs` | Export only from the document model. |
| `run` / CLI commands | binary entry | `rjtd/crates/rjtd-cli/src/main.rs` | Diagnostics and export command dispatcher. |
| `HwpDocument` | WASM wrapper | `rjtd/crates/rjtd-wasm/src/lib.rs` | Browser-facing compatibility wrapper for Studio-style calls. |
| `Document` | rhwp model | `rhwp/src/model/document.rs` | Reference architecture; high centrality in rhwp. |
| `rhwp` CLI | binary entry | `rhwp/src/main.rs` | Reference CLI/export/diagnostic behavior. |

## CONVENTIONS
- English is the default root documentation language; Japanese translations use
  `*.ja.md`. Korean files are ignored at root by `*.ko.md`.
- Root `.gitignore` excludes `rhwp/`, `.cargo-home/`, `.omx/`, `tmp/`,
  `rjtd/target/`, private local samples, and generated sample PDFs.
- `rjtd` follows rhwp's structure, layer separation, data model design, and test
  strategy before inventing new architecture.
- If a dependency or implementation approach is needed in `rjtd`, inspect
  `rhwp/Cargo.toml` and the rhwp implementation first.
- Do not add a convenience dependency when rhwp has no matching precedent.

## ANTI-PATTERNS (THIS PROJECT)
- Do not copy `rhwp/` code into `rjtd`; use it as a read-only reference.
- Do not let exporters read raw container, stream, or record data directly.
- Do not drop unknown or undecoded input; preserve it as unknown model shapes or
  explicit `decoded:false`/diagnostic evidence.
- Do not promote fallback/reference-backed heuristics to decoded semantics.
- Do not treat generated outputs (`openjtd-samples/pdf-output/`,
  `openjtd.github.io/pkg/`, Cargo lock/build output) as source authority.

## COMMANDS
```bash
cd rjtd
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

cd ..
scripts/regenerate-pdf-output.sh
```

## NOTES
- `codegraph_explore` is available for this workspace; LSP symbol inventory was
  not fully exposed in this session, only grep-style reference lookup.
- `TODO.md`, `TODO.ja.md`, and `docs/RHWP-COMPATIBILITY.md` contain many live
  reverse-engineering constraints. Treat "blocked", "unproven", "decoded-false",
  and "reference-backed" as safety labels, not status noise.
- `rhwp/CLAUDE.md` already held strong project rules before this file was
  generated; `rhwp/AGENTS.md` summarizes them for Codex-style agents.
