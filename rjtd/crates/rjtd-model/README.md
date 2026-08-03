# rjtd-model

Experimental document model and parser integration for Ichitaro JTD files.

`rjtd-model` is the model layer of
[OpenJTD](https://github.com/KimEJ/OpenJTD). It consumes the low-level evidence
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

## License

Apache-2.0.
