# Architecture

The `rjtd` implementation separates interpretation of JTD source data from
rendering. OpenJTD's shared format research describes the source data and its
meaning; each implementation owns its output methods and approximations.
Other office code is an optional reference when useful and permitted. Its
architecture, addressing, dependencies, and API parity do not define JTD requirements.

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
      ├──── Text / Markdown / semantic HTML / diagnostic JSON
      └──── Rendering
                ├──── font measurement and layout
                ├──── SVG / page layers
                └──── PDF / browser output
```

## Layer Rules

- Container code discovers and opens logical streams.
- Stream code handles byte-level stream access.
- Record code decodes typed and unknown record boundaries.
- Model code owns semantic document structures.
- JTD source evidence determines model semantics; consumer-specific editing
  addresses belong at the adapter boundary, not in source interpretation.
- Export code consumes only the document model.

Exporters must not read raw container, stream, or record data directly.

## Unknown Preservation

Reverse engineering is incremental. Any data that is not understood yet must be carried forward as one of the unknown model shapes instead of being discarded.

- `UnknownRecord`
- `UnknownBlock`
- `UnknownStyle`
- `UnknownObject`

## Current Implementation Boundary

`rjtd-core` contains low-level container, stream and record parsers. `rjtd-model`
owns `Document`, parser integration and bounded source candidate queries. Building
it with `--no-default-features` excludes the rendering/application surface and
its bitmap dependencies. The `rendering` feature enables `DocumentCore`, page
construction, SVG/page-layer output and app state; `bitmap-images` adds bitmap
decoding and enables `rendering`. Both remain enabled by default for existing
consumers. `rjtd-export` owns serialization and the native PDF backend.

Source recognition for text styles, notes/fields, page/section/running instructions,
physical rows, images, figures and equations is separated from placement and
paint. Source queries retain mm100/source units, raw font names, flow/span and
unknown evidence. The app owns mutation, navigation, snapshots and print context;
rendering owns fallback layout, CSS/font measurement and output geometry. These
are internal boundaries in the existing workspace, with no separate renderer crate.

The core block model currently exposes `Paragraph` and `Unknown`; inlines expose
text, ruby, and unknown objects. Preserved table, style, layout, and object
candidates carry evidence until their semantics are proven. SVG/PDF rendering
uses this model with fallback layout and limited diagnostic projections.

`DocumentCore` provides read/render APIs and basic body-text editing. The
`JtdDocument` WASM wrapper retains the `HwpDocument` Rust/JS compatibility names and existing browser API, but many advanced calls
return defaults or no-op results. API presence does not establish JTD editing
support or round-trip preservation.

Basic document HTML export belongs to `rjtd-export`; app-core HTML clipboard
methods are a separate compatibility surface. For current scope and the next
completion criteria, see [feature status](FEATURE-STATUS.md),
[validation](VALIDATION.md), and the [roadmap](ROADMAP.md).

## Interpretation Core

Interpretation reads the input and retains its original flow, records, source
ranges, relationships, and unknown data. It establishes the meaning of fields
only to the extent supported by evidence. Its output is an owned document
model that extraction, inspection, rendering, and editing can consume.

Source coordinates and their units, writing direction, character properties,
saved line/page ranges, ruled-line boundaries, object anchors, and paint order
belong here when they express source semantics. A decoded layout instruction
does not become renderer-specific merely because it affects appearance.
Unresolved interpretations remain candidates with their source evidence.

The core must not depend on SVG/PDF generation, browser APIs, installed fonts,
glyph measurement, DPI conversion, or a fallback layout algorithm. Inspecting
a document must work without initializing a renderer. Output measurements
must not change source text, spans, field values, or evidence status.

Ruled lines annotate the original text flow. A rectangular `Table -> Row ->
Cell` projection may serve an output or editing operation, but it must not
replace the preserved source flow or become a prerequisite for parsing.

## Rendering

Rendering consumes the model and an explicit output context: available fonts,
font measurements, output scale, and dynamic values such as the print date.
It computes glyph advances, line placement, fallback pagination, ruby/script
placement, clipping, and paint operations, then produces SVG, page layers,
PDF, or browser output.

Renderers use interpreted source instructions where available and record
approximations separately. A successful visual match does not establish a
field's meaning. Synthetic bold, substitute-font metrics, a guessed baseline,
and backend-specific leader strokes remain rendering choices.

Renderers must not open the input container or decode raw model streams.
Existing functions that combine raw-field recognition with placement must
first expose the recognition result through the interpretation model. Only
then can their placement and paint code move behind the rendering boundary.
Rendering consumes model data in one direction; the core never calls back
into a renderer to finish source interpretation.

`DocumentCore` can remain an application facade during migration. Loading,
search, editing, selection, and snapshots are application concerns;
selection geometry and hit testing consume rendering results. The facade's
name and compatibility methods do not define the interpretation core.

## Migration and Regression Gates

Start with boundaries inside the existing workspace. A separate rendering
crate is justified only when its responsibilities and one-way dependencies
are clear; this plan adds no crate or dependency.

The initial ownership map follows existing code, rather than its file names:

| Current surface | Target responsibility | Required split |
| --- | --- | --- |
| `rjtd-core` and model `parse` | Container/record parsing and model loading | Keep resource limits and raw preservation independent of rendering |
| `document_text/flow`, fields/notes, and character-style interpretation | Source flow, relationships, and source properties | Separate any placement helpers from recognition results |
| `document_text/native_*`, `table_grid`, and `object_media/native_*` | Mixed source interpretation and rendering | Move raw-field recognition into model candidates first; keep placement, measured advances, and paint in rendering |
| `document_text/svg`, page-layer construction, and `table_grid_render_projection` | Rendering | Consume source instructions through the model; split remaining raw recognizers before moving |
| `rjtd-export` PDF and browser font measurement | Output backends | Depend on rendering/model results; never become a core dependency |
| `DocumentCore`, `search_render_editing`, and WASM facade | Application coordination | Preserve public calls; delegate rendering and consume its geometry |

Saved line/page ranges and coordinates remain source instructions even when
used by a renderer. Fallback line breaking and pagination are computed output.
Do not move either wholesale just because both currently live in a module
named `layout` or `native`. Model diagnostic JSON is also distinct from a
rendered page-layer JSON tree.

1. Classify existing functions by source interpretation, rendering, or app
   state. Split mixed functions at their source-evidence result, preserving
   unknown bytes and candidate status.
2. Move font measurement, output-unit conversion, SVG paint, and page-layer
   construction behind the rendering boundary. Keep existing public facade
   calls as delegating compatibility entry points.
3. Separate saved source line/page instructions from fallback layout. Share
   interpreted source instructions across extraction and renderers without
   making either consume the other's output.
4. Verify that parsing and model inspection build without a rendering
   backend. Consider a crate split only after that dependency check passes.

Every code-move step requires public parser/model regressions, formatting,
Clippy, workspace tests, WASM and MSRV checks, and comparisons of text, source
spans, unknown data, page dimensions, and rendered output. Use the existing
native pairs locally; report private or unavailable inputs separately.
Refactoring must preserve existing source semantics and rendering behavior.

## Format RFCs and Implementation Records

Format RFCs describe bytes, record boundaries, relationships, observed
behavior, hypotheses, counterexamples, and applicable authoring versions.
Each claim needs its own evidence and validation scope. A command or parser
output can identify a reproduction tool, but its data model and fallback
behavior are not the format contract.

The implementation's CLI commands, Rust APIs, JSON shapes, renderer choices,
test sweeps, and implementation priorities belong in
[rjtd research records](../rjtd/docs/research/README.md). Format-facing drafts
remain indexed in [openjtd-spec](../openjtd-spec/README.md). This local
separation does not amend the shared repositories or imply joint acceptance
of a draft.
