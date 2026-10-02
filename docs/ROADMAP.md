# OpenJTD Roadmap

OpenJTD's long-term goal remains a faithful JTD rendering engine and editor.
The current phase is reverse engineering and component development. The next
delivery goal is reproducible reading and rendering for a bounded profile:
horizontal body text and a simple table on one page.

This document owns the milestone summary and next priorities. The
[charter](CHARTER.md) defines the long-term direction;
[TODO.md](../TODO.md) retains detailed tasks and research history;
[openjtd-spec](../openjtd-spec/README.md) records format evidence.

## Reading Status

- **Implemented** means a code path exists for the stated scope, not that every
  Ichitaro version or native layout is supported.
- **Diagnostic / fallback** means evidence or an approximate projection exists.
  `decoded:false`, `Candidate`, and `reference-backed` remain limits on semantic
  claims even when the viewer displays a result.
- **Verified** results must name the input set and checks performed. A skipped
  local-sample test or a generated PDF is not proof of rendering fidelity.
- **Planned** work is unfinished; the acceptance criteria below define completion.

## M1: Container Explorer

Implemented for observed CFB/OLE `.jtd`, `.jtt`, and `.jttc` files through
`rjtd streams` and `rjtd info`, including a lenient malformed-FAT fallback.
This establishes container access, not full document-format coverage.

## M2: Text Extraction

Implemented for observed `/DocumentText` streams, ruby base text, embedded
`SsmgV.01` / `TextV.01` fragments, and the observed JTTC
`JustCompressedDocument` / `-lh5-` profile. `rjtd cat` exposes this path.
Full paragraph, control, and style semantics remain incomplete.

## M3: Document Model

Implemented as a text-oriented model with paragraphs, text runs, ruby, source
spans, and preserved unknown data. Table, style, image, and layout candidates
carry research evidence; they are not a complete semantic model.

SVG and native PDF output, basic body-text editing, search, selection, and
snapshots exist. Rendering combines fallback text layout with limited source-
or reference-backed projections. General layout fidelity, structure-preserving
editing, and saving back to JTD remain unfinished.

## M4: Markdown Export

Minimal model-based Markdown export is implemented, alongside plain text,
JSON, and basic HTML. HTML preserves paragraph text and ruby markup; it does
not reproduce full layout. Heading, list, table, and style semantics depend on
further model decoding. Rich HTML clipboard APIs are a separate, unfinished
app-core surface.

## M5: Public Specification

Ongoing. [RFCs 0001–0009](../openjtd-spec/README.md) record container, text,
compression, layout, object, and paragraph-record observations in English and
Japanese. These are incremental research records, not a complete specification.
New decoding claims need supporting samples and preserved counterexamples.

## M6: WASM Viewer

The [static viewer](../openjtd.github.io/README.md) implements file selection/drop,
page navigation, SVG rendering, and a plain-text tab. Documents are processed
in the browser. The repository's deployment workflow builds from `main` and
targets GitHub Pages; earlier Cloudflare experiments are not the current
configuration.

Browser binding coverage does not imply full editing support. The viewer
inherits model and renderer limitations. Browser runtime regression coverage
and visible fidelity warnings remain next work. Deployment availability must
be checked separately from the presence of a workflow.

## Next Priorities

### Fidelity checkpoint: 2026-10-02

The current implementation preserves leading `TextV.01` text, bounds named
content before the style tail, and retains UTF-16 source ranges for supplementary
characters. Supported source styles provide font sizes and per-edge page margins;
explicit page-layout margins take precedence over document-view defaults.

First-page horizontal control tables now have source-derived cell-text placement
and basic SVG/PDF border projections. Local evidence covers 20 single-table
variants plus one two-table document with interstitial text and trailing empty
rows. A separate complete Frame/LayoutBox grid profile preserves six cell labels.
These profiles stay `decoded:false`: exact font metrics, border paint styles,
generic wrapping, merged/sparse cells, and general page assignment remain unfinished.
Details and excluded cases are recorded in [RFC 0008](../openjtd-spec/rfc/0008-object-stream-candidates.md)
and [RFC 0009](../openjtd-spec/rfc/0009-document-text-paragraph-record.md).

Large tables and pagination remain active implementation work with existing
reference pairs. For example, the local food-classification table still produces
26 fallback pages against a one-page reference. Further vertical-writing and
JTT/JTTC reference pairs, original font environments, and controlled image/shape
samples would extend verification; their absence does not block work on existing
reproduction failures. No complete-fidelity claim is made for any format.

### Acceptance work

1. **Make accuracy checks reproducible.** Establish a provenance-reviewed
   fixture manifest and a redistributable minimum corpus. Report executed,
   skipped, and known-divergent cases separately. Record text, page count and
   orientation, table structure, and geometry expectations. Existing controlled
   probes must distinguish native-authored cases from imported surrogates.
   Completion requires another checkout to reproduce the checks without
   untracked private files.
2. **Finish one bounded rendering profile.** Decode paragraph/line boundaries
   and the page origin, column widths, row heights, and text positions of a
   simple one-page horizontal document. Completion requires the same rule on
   independent documents without reference-PDF coordinates or sample-specific
   calibration. Keep rejected hypotheses and unresolved candidates diagnostic-only.
3. **Extend proven rules.** Add multi-page flow and styles, then vertical writing
   and image/object ownership and paint order. Each extension needs cross-sample
   regression evidence. Add executable malformed-input fuzz targets; the current
   `rjtd/fuzz/` directory is only a placeholder.
4. **Build structure-preserving editing and save.** Expose supported editing
   capabilities explicitly and preserve unknown data through mutations.
   Completion requires edit/save/reopen checks. Full rhwp Studio parity,
   common-format IR, and HWP/HWPX export remain longer-term work.

## Verification Boundary

The [Rust quality workflow](../.github/workflows/rust-quality.yml) covers formatting,
checks, tests, Clippy, documentation, WASM, packaging, notices, dependency audit,
and MSRV. These establish quality within their scope, not native-layout equivalence.

Some local-sample tests return early when source/reference files are absent;
others require `--ignored`. Generated PDF artifacts are also local. The
[fixture guide](../rjtd-testdata/README.md) explains the verification tiers.
Known pagination/orientation divergences remain visible until decoded source
rules replace the fallback behavior.
