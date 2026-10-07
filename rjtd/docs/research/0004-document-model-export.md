# rjtd research 0004: Document model and exports

Status: implementation record; not a shared format RFC

Japanese translation: [0004-document-model-export.ja.md](0004-document-model-export.ja.md)

## Implementation Scope

`Document`, `DocumentCore`, exporter APIs, and diagnostic JSON are rjtd contracts. The original record described an early LF/paragraph mapping and an HTML-not-implemented stage; those historical statements must not be used as current capability documentation. Current HTML and PDF paths exist. Source parsing/model ownership remain separate from output serialization.

The [format-facing record](../../../openjtd-spec/rfc/0004-document-model-export.md) retains source
observations and unresolved interpretations. Rust types, candidate admission,
JSON fields, CLI output, paint approximations, test coverage, and resource
policies are implementation behavior. They do not establish native semantics.

## Reproduction Entry Points

From the repository root, build the CLI with `cargo build --manifest-path rjtd/Cargo.toml -p rjtd-cli`. Use an input whose analysis and disclosure are
permitted. Inspect `rjtd/target/debug/rjtd --help` for each command's required arguments.

```sh
rjtd/target/debug/rjtd export path/to/document.jtd --format json
rjtd/target/debug/rjtd document-info path/to/document.jtd
```

Some commands require a page number, stream, format, or other argument.
Record the actual full command, input ID/hash, authoring version, implementation
commit, executed/skipped checks, and observed limitations with any result.
Private-corpus checks do not become public reproducibility by being listed here.

## Historical Evidence

The [complete pre-separation record](https://github.com/OpenJTD/rjtd/blob/cce9cb20611c8c809a7376c8df6130aed14a0437/openjtd-spec/rfc/0004-document-model-export.md) preserves the original raw
examples, commands, local sweeps, rejected hypotheses, and implementation
history at a fixed commit. It mixes source observations with implementation
behavior and includes superseded capability statements; it is a historical
source, not the current format contract or an executable public fixture pack.
The public shared imports of RFC 0001/0003 retain their own import commits and
status; this local edit does not change them.

## Separation Work

Apply the [interpretation/rendering boundary](../../../docs/ARCHITECTURE.md)
before moving mixed functions. Preserve text, source spans, unknown data,
candidate labels, resource limits, and output behavior in focused regressions.
No Rust modules were moved by this documentation change.

## Preserved Pre-Separation Record

The following original mixed research record retains its detailed observations,
raw examples, experiments, counterexamples, and implementation history. Its
capability, policy, and hypothesis statements describe their historical stage;
read them together with the current scope above. Relative links were rebased.

Status: draft

Observed: 2026-06-18

Japanese translation: [0004-document-model-export.ja.md](0004-document-model-export.ja.md)

## Summary

`rjtd export` now follows the required layer boundary:

```text
Document File
  -> Container
  -> DocumentParser
  -> Document Model
  -> Exporter
```

Exporters do not read raw JTD/JTT/JTTC bytes directly.

## Implemented Commands

```sh
cargo run -p rjtd-cli -- export ../rjtd-testdata/local-samples/a5.jtd --format json
cargo run -p rjtd-cli -- export ../rjtd-testdata/local-samples/a5.jtd --format md
cargo run -p rjtd-cli -- export ../rjtd-testdata/local-samples/a5.jtd --format text
cargo run -p rjtd-cli -- export ../rjtd-testdata/local-samples/setsuden_05.jttc --format json
```

`html` is recognized as a planned format but is not implemented.

## Initial Model Mapping

The current model bridge is intentionally minimal:

- extracted text is normalized to LF line endings;
- non-empty lines become `Paragraph` blocks;
- each paragraph contains one `TextRun`;
- the raw `/DocumentText` stream, inner `.jttc` `/DocumentText`, or embedded fragment payload is preserved in the `Document` model;
- observed style/layout streams are preserved as named `UnknownStyle` model entries before their record semantics are decoded;
- style references are currently `null`;
- unknown styles, objects, and blocks remain part of the model API for future preservation work.

This mapping is not a final JTD model. It exists to make the exporter boundary real early in the project.

## JSON Shape

The initial JSON export is compact and model-oriented:

```json
{
  "metadata": { "title": null },
  "blocks": [
    {
      "type": "paragraph",
      "style": null,
      "inlines": [
        { "type": "text", "text": "銀河鉄道の夜", "style": null }
      ]
    }
  ],
  "unknownStyles": [],
  "unknownObjects": [],
  "textCountRanges": [],
  "textControlBoundaries": [],
  "textBoundaryCandidates": [],
  "textParagraphBoundaryCandidates": [],
  "rawStreams": [
    { "name": "/DocumentText", "size": 240104 }
  ]
}
```

Unknown payloads, when present in the model, are emitted as hex strings so the JSON exporter does not silently discard them.

Raw streams are summarized by name and size in JSON. Their bytes remain preserved in the in-memory model for later round-trip and structured parsing work.

Observed style/layout streams are currently emitted under `unknownStyles` with their stream path, raw payload, observed family, big-endian header fields, and neutral record boundary candidates. On `a5.jtd`, the preserved style streams are:

```text
/DocumentEditStyles
/DocumentViewStyles
/TextLayoutStyle
/PageLayoutStyle
/PageLayoutStyleHeader
```

The app-core compatibility JSON also reports these sources without presenting them as decoded paragraph styles: `getDocumentInfo` includes `styleStreamCount`, `styleCandidateCount`, `styleCandidateNames`, and `styleStreams`; `getStyleList` includes `sourceStreamCount`; and `getStyleDetail(0)` includes `decoded:false` plus `sourceStreams`.

For labeled `/TextLayoutStyle` records, app-core style APIs expose rhwp-shaped style candidates with stable candidate IDs, observed names, `decoded:false`, and source stream/offset/code metadata. These candidates are not yet paragraph or text-run style references.

When a user applies one of these candidates through app-core `applyStyle`, rjtd stores the candidate ID as the in-memory paragraph `StyleRef`. `getStyleAt` and model JSON can then observe the fallback style reference. This is an editable model state, not proof that the original JTD body/style streams can be rewritten yet.

The same observation data is available through CLI diagnostics. `rjtd style-records <file>` prints preserved style stream families, header candidates, record layouts, offsets, codes, payload lengths, and labels. `rjtd style-candidates <file>` prints the labeled `/TextLayoutStyle` subset as stable per-document candidate rows for cross-sample correlation. `rjtd text-layout-style-records <file>` prints all `/TextLayoutStyle` records, including unlabeled records, payload lengths, digests, BE16 fields, short previews, labels, and candidate IDs for labeled records. `rjtd document-view-style-groups <file>` prints `/DocumentViewStyles` group record payload lengths, digests, and short previews. `rjtd text-position-style-context <file>` compares `TCntV.01` tail fields with text/page style candidate IDs, record indexes, and `/DocumentViewStyles` group records; `rjtd text-position-style-summary <file>` aggregates those hits per tail field; `rjtd text-position-count-tail-field-roles <file>` compares tail fields and adjacent field pairs against document-text unit/text hits at selected deltas. These hits remain evidence rather than decoded model references.

Parsed `TextRun` values now carry `/DocumentText` byte and UTF-16 source spans when the parser can derive them from the source stream map. Valid `TCntV.01` text-count entries are preserved as decoded-false `textCountRanges` in the document model. JSON export and app-core `getDocumentInfo` expose the observed family, chosen start/end/span, declared start/end, tail fields, raw entry bytes, byte/unit `documentTextOverlaps` against model text runs, and candidate `controlRangeOverlaps` against `/DocumentText` intervals split by observed delimiter candidates such as `0x001c` and `0x000e`. Those overlaps are also mirrored as decoded-false top-level `textBoundaryCandidates` so application tools can inspect possible paragraph-boundary evidence without treating it as recovered paragraph semantics. The stricter nonzero-span, strict unit `0x001c`, dual-layout-exact subset is exposed separately as decoded-false `textParagraphBoundaryCandidates`. The parser still does not attach these ranges or candidates to paragraph styles, text-run styles, real paragraph boundaries, or final layout geometry. On the current 61 local samples, JSON export succeeds for every sample; 10 samples expose non-empty `textCountRanges`, all 10 expose at least one `documentTextOverlaps` entry, and `textParagraphBoundaryCandidates` preserves 10 rows total, all in `iwata_file`.

`/DocumentText` control boundary codes are preserved separately as decoded-false `textControlBoundaries`. When a boundary comes from the named `/DocumentText` stream, JSON export and app-core `getDocumentInfo` include its byte and UTF-16 unit source span. These boundary entries are not yet promoted to paragraph, line, style, or object semantics; they are retained as evidence for later boundary and layout inference.

Non-text visual/object stream evidence is preserved separately as decoded-false `objectStreamCandidates`. JSON export and app-core `getDocumentInfo` expose candidate path, size, reason list, path-derived ownership candidate, image signature offsets, image payload spans, image payload object envelopes, SVG offsets, SO marker offsets, and a short payload prefix. The model keeps complete detected image payload bytes internally, while JSON reports payload metadata, undecoded header/trailer envelope metadata, conservative declared-length candidates, prefix numeric header fields, source-path candidates, and a short `payloadPrefixHex` preview. `getValidationWarnings` reports these as `JtdObjectStreamCandidateDiagnosticOnly`. These entries are evidence for later image, shape, SVG, object, and table recovery; they are not yet page paint operations or decoded Ichitaro object geometry.

For app-core navigation, rjtd can project preserved `textControlBoundaries` onto fallback paragraph character offsets when neighboring source-spanned text runs make that projection safe. `getControlTextPositions` therefore keeps rhwp's numeric-array return shape. Nearest-control APIs may return `type:"jtdControl"` with `decoded:false` diagnostics, explicitly avoiding false table/picture/shape/equation/field/bookmark semantics.

`getPageControlLayout` uses the same projection for page-scoped diagnostics. It can return per-control entries with fallback bounding boxes, `secIdx`, `paraIdx`, `controlIdx`, `charPos`, source, code, and `decoded:false`; these entries are evidence-preserving placeholders, not decoded Ichitaro object geometry.

App-core `getPageLayerTree` now emits a fallback `pageBackground` paint op, fallback `textRun` paint ops, and rhwp-shaped top-level `textSources` plus per-op `source` spans. These text entries include JTD byte/unit source ranges when parsed `/DocumentText` spans are known. The layer envelope includes schema/resource table versions, output options, empty font resources, feature lists, and fallback `textV2` diagnostics. The geometry is still produced by rjtd's fallback text paginator; it is not decoded Ichitaro page layout.

`getCanvasKitReplayPlan` consumes the same fallback projection. It accepts rhwp-compatible `default` and `compat` modes, rejects unsupported modes, and reports fallback `pageBackground`/`textRun` operations as direct `replayPlane:"background"`/`replayPlane:"flow"` items so app render policy diagnostics do not look like an empty page.

The current style stream summaries intentionally use neutral observation labels:

- `family:"ssmg"` for streams beginning with `SsmgV.01`, currently including `TextLayoutStyle`, `PageLayoutStyle`, and many `PageLayoutStyleHeader` payloads.
- `family:"table"` for observed non-`SsmgV.01` style tables, currently including `DocumentEditStyles` and `DocumentViewStyles`.
- `headerU32Be` and `headerU16Be` expose big-endian header candidates without assigning final field names yet.
- `recordLayout:"ssmg-slots"` reports observed Ssmg slot records with `0x5555` or `0x4444` markers, using `offset`, numeric `code`, `codeHex`, `payloadLength`, and a conservative UTF-16BE `label` candidate when one is present.
- `recordLayout:"sequential"` reports observed table records that fit `u16 code + u16 payload_len + payload`; on `a5.jtd`, `/DocumentViewStyles` currently exposes 48 such candidates.
- `recordLayout:"none"` means no safe observed record boundary was found yet; the raw payload is still preserved.

## Local Sample Result

On `a5.jtd`, JSON export currently produces 490 paragraph blocks after blank lines are omitted.

The first block is:

```text
銀河鉄道の夜				宮沢 賢治
```

The table of contents includes recovered inline base text such as:

```text
一、午后の授業
五、天気輪の柱
八、鳥を捕る人
九、ジョバンニの切符
```

On observed `.jttc` template samples, JSON export opens `/JSCompDocument`, decompresses the inner CFB, and preserves the inner `/DocumentText` stream summary:

```json
{
  "blocks": [],
  "rawStreams": [
    { "name": "/DocumentText", "size": 564 }
  ]
}
```

Those samples currently produce zero non-empty paragraph blocks because the extracted text is blank/control-heavy.

On observed `cfb-embedded-document-text` samples, JSON export records the recovered source as `/EmbeddedDocumentText`:

```json
{
  "blocks": [
    { "type": "paragraph", "inlines": [{ "type": "text", "text": "参加者募集中団体名氏　名", "style": null }], "style": null }
  ],
  "rawStreams": [
    { "name": "/EmbeddedDocumentText", "size": 77380 }
  ]
}
```

This size is from `ichitaro-20030706231249-success-001-success_data-fujimoto_file.jtd`; other embedded samples differ.

For PDF regression coverage, `rjtd-export` includes a conditional local-sample smoke test. When `rjtd-testdata/local-samples` is present, the test parses every available `.jtd`, `.jtt`, and `.jttc` sample, exports each document to PDF bytes, and checks for a PDF header, page marker, EOF marker, and non-trivial size. This keeps the sample-wide PDF export path reproducible without requiring redistributable fixtures in the crate.

## Known Gaps

- Paragraphs are derived from extracted newlines, not decoded JTD paragraph records.
- Style streams are preserved as raw named `UnknownStyle` data, but style IDs are not attached to paragraphs or text runs yet.
- Only the observed ruby base/phonetic annotation pair is represented as structured `Inline::Ruby` model data; broader ruby/style semantics are not decoded yet.
- JTTC `JustCompressedDocument` support is limited to the observed single-member `-lh5-` profile.
- Embedded document text source boundaries are heuristic.

## Next Steps

- Decode paragraph and inline record boundaries in `/DocumentText`.
- Decode the preserved style streams and attach style references to paragraphs and text runs.
- Decode the remaining ruby/style controls beyond the currently observed base/phonetic pair.
- Interpret `.jttc` template/control-heavy `DocumentText` as model content instead of blank paragraphs.
- Replace embedded `/EmbeddedDocumentText` synthesis with proper object/stream ownership.
- Add stable expected JSON fixtures when redistributable samples are available.
