# rjtd-model

Experimental document model and parser integration for Ichitaro JTD files.

`rjtd-model` is the model layer of
[OpenJTD](https://github.com/OpenJTD/rjtd). It consumes the low-level evidence
produced by `rjtd-core` and provides the `Document`, `DocumentParser`, and
`DocumentCore` APIs used by exporters, the CLI, WebAssembly bindings, and the
OpenJTD viewer.

## Developer preview

Version 0.0.1 is an experimental developer preview. All public APIs may change
in any later 0.0.x release. The model deliberately exposes diagnostic and
evidence-preserving types while the JTD format is still being decoded.

## What it provides

- `parse_document` and `IchitaroParser` entry points.
- A document model for metadata, paragraphs, text runs, ruby annotations,
  unknown blocks, styles, objects, and preserved raw streams.
- `DocumentCore` APIs for document/page information, text-first SVG and HTML
  rendering, search/edit fallbacks, selection, and viewer integration.
- Candidate structures for observed layout, table, object, image, and style
  evidence.

Observed `.jtd` and `.jtt` documents are supported through the shared parser;
observed `.jttc` compressed documents are supported where their inner document
can be recovered. This is not complete coverage of all Ichitaro versions or
document features.

## Primary Rust API

- `parse_document(&[u8]) -> rjtd_core::Result<Document>` is the one-shot JTD
  family parsing entry point. It applies the default input limits before model
  construction.
- `Document` owns the parsed metadata, blocks, raw streams, and retained
  evidence. It is the value consumed by `rjtd-export`.
- `DocumentCore` is the application-facing facade used by the viewer and
  `rjtd-wasm`. Construct it with `DocumentCore::from_document` or
  `DocumentCore::from_bytes` when page information, rendering, navigation, or
  editing fallbacks are needed.
- `parse_document_with_limits`, `DocumentCore::from_bytes_with_limits`, and
  `DocumentCore::from_document_with_limits` are the limits-aware alternatives
  when a caller must choose a non-default `ParseLimits` budget.
- `parse_document_with_budget`, `DocumentCore::from_bytes_with_budget`, and
  `DocumentCore::from_document_with_budget` are compositional Rust entry
  points for callers that must preserve one mutable `ResourceBudget` across
  parsing, model construction, and page construction.

When optional style or font sources exceed a resource limit, their
`ResourceLimit` error is propagated rather than silently treating the source
as absent. Other retained optional evidence remains subject to the preview
interpretation boundaries below.

`DocumentCore::get_document_info()` returns JSON for applications rather than
a typed, versioned interchange format. Its `version` field is the
`rjtd-model` package version (0.0.1 here), not the source document's version
and not a schema version. The related `get_page_layer_tree()` output carries
its own `schema` object; neither JSON shape is a compatibility promise during
the 0.0.1 preview.

## Example

```rust,no_run
let bytes = std::fs::read("document.jtd")?;
let document = rjtd_model::parse_document(&bytes)?;
println!("{} blocks", document.blocks().len());
# let core = rjtd_model::DocumentCore::from_document(document);
# println!("{} pages", core.page_count());
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Resource and interpretation limits

The default parser rejects source input larger than 64 MiB and inherits the
LH5 limits documented by `rjtd-core`. One mutable resource budget reserves
known CFB stream declarations before reads, charges retained frame and
embedding records before collection growth, charges embedded-image payload and
envelope bytes before cloning, and reserves page/page-line output before page
render preparation. Image dimensions are read from supported image headers;
this model path does not decode or retain bitmap pixels, so its image limits do
not claim coverage for a downstream bitmap decoder. A `Candidate` is an
observed-but-not-confirmed interpretation, `Unknown` preserves unclassified
source material, and `Diagnostic` records why an interpretation is incomplete.
JSON fields carrying `decoded: false` are evidence boundaries: consume the
retained bytes or display the diagnostic, but do not promote the field to
authoritative layout or document meaning. Advanced layout, tables, embedded
objects, styles, and editing APIs can return conservative fallback results.
The 0.0.1 model is not a lossless round-trip editing contract.

## Source flow and table projections

`Document::document_text_flow()` retains the original logical DocumentText
payload as ordered text, inline, control, framed-record, and opaque events.
Each event carries byte/UTF-16 source spans; record and opaque words remain
available without interpreting their flags as cells or logical rows. Named
TextV.01 content bounds exclude the style tail. Repeated span declarations and
line breaks remain distinct rather than being coalesced during source parsing.

Existing `table_candidates()` are derived compatibility/diagnostic views for
current consumers, not the source representation. Their normalized row/cell
text does not replace the original events. JSON export exposes the source flow
as `documentTextFlow` with `decoded:false`. A document constructed without JTD
payload bytes has no source flow; fallback paragraph edits do not rewrite it.
Logical wrapping/merging, ruled-line paint semantics, and saving source changes
still require independent evidence and implementation.

A bounded first-page horizontal ASCII/Japanese projection places physical ruled spans
directly from source events, LineMark intervals, PageMark pitch, and source
margins. Explicit left/distributed alignment can be shown without rebuilding
logical rows. Explicit center/right spans use source midpoint/end anchors and
SVG's font-aware `text-anchor`; layer JSON separates `anchorX` from estimated
bbox/glyph positions. This remains a first-page horizontal text profile, not
logical-cell reconstruction. Distributed spans retain their source extent as SVG
`textLength`/`lengthAdjust="spacing"` and layer-tree metadata. Overlapping table
fallbacks remain diagnostics instead of painting the same text twice. Raw span
flags and `decoded:false` remain visible. Inherited wrapped automatic spacing,
glyph metrics, and ruled borders are not established by this projection.

The controlled western-style `0x100b` profile exposes an
`english_justification_candidate()` independently of cell flags. In the
supported left-aligned ASCII flow, nonfinal lines can request word-only spacing
to their source extent. `render_page_svg_with_text_widths()` accepts actual font
advances from a paint backend; PDF uses its existing SVG font engine and the
viewer supplies browser text measurements through the optional WASM adapter.
Unmeasured SVG/layer output retains explicit unresolved-spacing metadata.
Unknown style profiles, wrapped `0x00ff` alignment, single-word tracking, and
full font-metric equivalence remain unproven. Exact first/last text-run
LineMark intervals keep surrounding body text outside the admitted ruled flow.

A separate, bounded ruled-band projection reads directional paint at
the source junction units. Up/down/right use style properties 1/2/3 for presets,
4/5/6 for visibility, and 15/16/17 for BGR24 colors. Transparent strokes remain
in layer evidence without being painted or deleting text/cell candidates.
Thin, thick-preset, and dashed-preset sizes are renderer approximations, not
decoded source units; geometry/paint stay `decoded:false`. Unknown values,
other junction patterns and logical merge ownership are not admitted. Exact
surrounding LineMark positions also prevent body text overlapping this grid.
Column counts and integer width remainders come from the source strips, not
equal division of the body width. Both an explicit final blank run and a bare
terminal junction are admitted. The existing control-table text projection also
enables exact first/last body-line positions when it owns all cell text.

Physical bands may have differing horizontal boundaries or additional text
lines. Source states suppress the partial boundary in a vertically merged
example; no logical rowspan is invented. Explicit cell line breaks disable
automatic word spacing, and a following inherited-left span is admitted only
when the previous physical line in the same grid/span ends with an explicit
break. Japanese text uses UTF-16 source lengths, not UTF-8 byte lengths; its
tracking and exact native font advances remain unproven. Whitespace-only spans
retain their source text/range without advancing the surrounding body fallback.
Table fallbacks overlapping either physical-flow or control-table text stay
diagnostic-only, so mixed sparse projections do not repaint cells.

Simple horizontal text or ruled documents with uniform visible-run font sizes can
use a bounded source pagination plan:
LineMark ranges split the existing paragraph text runs, and active fixed84
PageMark ranges assign the physical lines to pages. Paragraph/character addresses
and noncontiguous cell source fragments remain intact. Body text and rules use
page-local line indices; capacity-only entries do not create blank pages.
Native page/line counts are reserved before page text is cloned. Missing or
edited source spans, unknown/inline content, unsupported scripts, or per-page
layout-style streams retain the previous fallback. Metadata stays candidate /
`decoded:false`; exact font tracking and general section pagination are unproven.

Native line positions accumulate source advances instead of multiplying one
pitch by the record index. The bounded default uses the larger of the document
font and visible text fonts plus the retained PageMark gap. A repeated fixed-
pitch `0010/0020` prefix overrides only its source line. Prefixes can be standalone
or wrap a ruled parent; original words and style-unit offsets stay intact.
Padding spaces remain source text but do not make an otherwise uniform visible
font range mixed. Larger table/body fonts, local cell breaks, and surrounding
body text share the same source metric helper; arbitrary mixed runs remain
unproven. This does not establish exact native glyph advances.

Standalone paragraph candidates distinguish first-line and continuing-line
indentation from framed source attributes. The supported zero-right-indent
profile applies each indent only to its paragraph's physical lines. A controlled
after-space percentage uses document font height as a bounded em-based advance;
it is not a general spacing or native print-quantization decoder. Text without
ruled bands can use the same source line plan when all source records are known.
Raw attributes, unknown values, and `decoded:false` evidence remain available.

## License

Apache-2.0.
