# RFC 0008: Object and Embedded Image Stream Candidates

## Status

Diagnostic only.

The observations in this note are not decoded Ichitaro object semantics. They are preserved as `decoded=false` evidence for the future model, layer, and renderer work.

## Motivation

Full-layout PDF export requires more than extracted text. rjtd must recover images, vector/shape objects, tables, and their layout records as model/control/layer objects before the exporter renders them.

This follows the rhwp-compatible policy: exporters must consume the document model and page/layer tree, not inject bytes directly from raw CFB streams.

## Diagnostic Command

`rjtd object-stream-candidates <file>` scans every readable CFB stream and reports streams that look relevant to visual or object recovery.

The first implementation classifies candidate streams with these evidence types:

| Evidence | Meaning |
| --- | --- |
| `object-path` | stream path contains embedding/object/OLE/binary-object naming such as `EmbedItems`, `Embedding`, `JSFart`, `CompObj`, `Ole`, `Object`, or `Bin` |
| `image-path` | stream path contains image-oriented naming such as `Image`, `Picture`, `Graphic`, `PNG`, `JPEG`, `BMP`, `WMF`, or `EMF` |
| `shape-path` | stream path contains shape/layout naming such as `Figure`, `Shape`, `Draw`, `Frame`, `LayoutBox`, or `SVG` |
| `table-path` | stream path contains table/cell naming, excluding position/style table names |
| `so-marker` | payload contains the preserved `SO\0\0` object/control marker family |
| `image-signature` | payload contains a recognizable binary image signature such as PNG, JPEG, GIF, TIFF, BMP-at-start, or placeable WMF |
| `svg-signature` | payload contains textual `<svg` evidence |

Output rows preserve the stream path, stream size, reason list, first image signature offsets, first SVG offsets, first SO marker offsets, a short payload prefix, and `decoded=false`.

## Current Sweep

The command was swept across the current 61 local `.jtd`, `.jtt`, and `.jttc` samples.

| Metric | Count |
| --- | ---: |
| samples checked | 61 |
| readable streams scanned | 2,253 |
| candidate rows | 933 |
| files with any candidate | 43 |
| files with `object-path` evidence | 27 |
| files with `shape-path` evidence | 42 |
| files with `table-path` evidence | 0 |
| files with `so-marker` evidence | 4 |
| files with `image-signature` evidence | 17 |
| files with `svg-signature` evidence | 0 |
| unreadable candidate scan streams | 0 |
| total `object-path` rows | 429 |
| total `shape-path` rows | 503 |
| total `table-path` rows | 0 |
| total `so-marker` rows | 11 |
| total `image-signature` rows | 64 |
| total `svg-signature` rows | 0 |

Representative observations:

- `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` exposes `/EmbedItems`, `/Figure`, `/FigureData`, `/Frame`, and `/LayoutBox` candidates. It also preserves `SO\0\0` hits in `/Figure` and `/PaperMark`.
- `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` exposes JPEG signatures inside `/EmbedItems/Embedding */Contents` streams, with offsets such as `jpeg@67` and `jpeg@72`.
- `ichitaro-20030422210439-success-002-success_data-natsu.jtd` exposes 12 image-signature rows, mostly JPEG signatures inside embedded `Contents` or `EmbeddedPress` streams.
- `ichitaro-20030829031540-success-004-success_data-hyo.jtd` exposes no object-stream candidates by path, SO marker, image signature, or SVG signature. Its visible table content likely depends on `/DocumentText` control/record decoding plus layout/style streams, not a named table object stream.

## Interpretation

Image recovery is now testable through stream/path candidates and binary image signatures. Several samples have strong embedded JPEG evidence inside `EmbedItems` streams, often after a short object header.

Shape and layout-object recovery should start with `/Figure`, `/FigureData`, `/Frame`, and `/LayoutBox` families. These are path-level candidates, not decoded geometry.

Table recovery should not rely on named CFB table streams in the current corpus. The zero `table-path` result, especially in the `hyo` sample, suggests that table structure is more likely encoded in `/DocumentText` controls, style streams, or layout mark records.

## Model Preservation

The parser now promotes these stream observations into decoded-false model evidence as top-level `objectStreamCandidates`.

JSON export and app-core `getDocumentInfo` expose each candidate with:

- `path`
- `size`
- `reasons`
- `ownershipCandidate`
- `ownershipReferences`
- `fdmIndexEntries`
- `imageSignatures`
- `imagePayloads`
- `svgOffsets`
- `soOffsets`
- `payloadPrefixHex`
- `decoded:false`

Each `imagePayloads` row records `kind`, `mime`, `signatureOffset`, `start`, `end`, `length`, `complete`, optional `dimensions`, `objectEnvelope`, `payloadPrefixHex`, and `decoded:false`. The document model also retains the payload bytes internally so a future renderer can consume image resources through the model instead of reopening raw streams from an exporter.

The `objectEnvelope` field preserves the undecoded bytes around each payload: header start/end/length, header prefix, trailer start/end/length, trailer prefix, and a conservative `declaredPayloadLength` candidate when the final four header bytes exactly match the detected payload length as little-endian `u32`. It also exposes decoded-false `headerFields`: prefix `u16LePrefix`/`u32LePrefix` numeric candidates and a `sourcePathCandidate` when the header carries a path-length byte followed by a NUL-terminated embedded source path. This is evidence, not decoded Ichitaro object geometry.

The candidate-level `ownershipCandidate` is also decoded-false. It is derived only from the stream path and records a `stream-path` basis, family, optional storage path, optional `embeddingIndex`, and stream role. Examples include `EmbedItems` contents/embedded-press streams, `FigureData` `FDMVector`, and root figure/frame/layout streams. It does not prove page placement or final object geometry.

The candidate-level `ownershipReferences` field is decoded-false cross-stream evidence. It is currently attached only to embedded image candidates with a path-derived `Embedding N` owner and records byte-pattern matches for that `N` in `FigureData`, `/Figure`, `/Frame`, `/LayoutBox`, `/PageMark`, and `/PaperMark` streams. Each row records `targetPath`, `encoding`, `totalMatches`, a bounded `offsets` preview, and `decoded:false`. These rows prove that a candidate embedding index is observed elsewhere in object/layout-related streams; they do not yet identify the authoritative record field or page geometry.

`getValidationWarnings` reports these entries as `JtdObjectStreamCandidateDiagnosticOnly`.

A JSON export sweep across the same 61 local samples succeeds with 0 failures and preserves 933 `objectStreamCandidates` across 43 files. The sweep exposes 17 files with image-signature candidates, 4 files with SO-marker candidates, 0 table-path files, and 0 SVG-signature files. This matches the diagnostic CLI distribution while keeping the evidence inside the document model.

A second JSON export sweep over the model-preserved `imagePayloads` field also succeeds across all 61 local samples with 0 failures. Under strict JPEG SOF/SOS validation it finds 12 files with complete payload spans, 67 complete payloads total, 35 dimensioned payloads, and 629,024 preserved payload bytes: 35 JPEG rows across 5 files, 31 GIF89a rows across 9 files, and 1 GIF87a row in 1 file.

The same sweep preserves 67 object envelopes. Twenty rows expose a matching little-endian declared payload length, all currently in `Embedding */Contents` streams; large `FDMVector` and `EmbeddedPress` wrapper streams no longer expose promoted payload spans unless they pass strict payload validation.

Header field candidate sweep results: all 61 local samples export successfully, 66 payload rows expose a `sourcePathCandidate`, and every source-path candidate is currently in a `Contents` stream. The path extensions split into 34 `jpg` and 32 `gif` rows. The first little-endian prefix word is `9` in 59 rows and `4` in 6 rows, matching the dominant `09 00 01 00` and secondary `04 00 01 00` header families observed in embedded image contents.

App-core image payload diagnostics expose the same envelopes and header/source-path candidates in overlay, layer-tree, and SVG data attributes. These rows remain `diagnosticRenderable:true` but `renderable:false` with `ownershipProven:false`, `pageGeometryProven:false`, and `paintOrderDecoded:false` until the ownership reference, geometry, and paint-order fields are decoded.

Ownership candidate sweep results: all 61 local samples export successfully, 474 of 933 object stream candidates expose a path-derived `ownershipCandidate`, and all 67 image payload rows are covered by one. All promoted payload rows currently have the `contents` role. Candidate families include `embed-items` 335, `figure-data` 38, `figure` 31, `frame` 42, and `layout-box` 28.

Ownership reference sweep results: all 61 local samples export successfully, 12 files expose cross-stream reference candidates, 52 embedded image candidates have `ownershipReferences`, and the model preserves 604 reference rows with 9,949 total byte matches. Reference rows split by target family as `frame` 196, `figure-data` 130, `page-mark` 113, `paper-mark` 73, `layout-box` 65, and `figure` 27; by encoding as `u16-be` 183, `u16-le` 171, `u32-be` 141, and `u32-le` 109. The covered source candidates are all `embed-items` content rows, and 67 of 67 image payload rows now have cross-stream reference evidence.

`rjtd object-ownership-references <file>` expands those model-owned references into match-context diagnostics. For each reported preview offset it prints source stream, target stream, encoding, offset, total matches for that reference row, mod2/mod4 alignment, local context hex, and le/be 16/32-bit values read at the match offset. This command is diagnostic-only and does not create renderable geometry.

The 61-sample `object-ownership-references` sweep succeeds with 0 failures and reports 3,167 preview-offset rows across the same 12 files. Target-family rows split as `figure-data` 1,010, `frame` 897, `layout-box` 528, `page-mark` 498, `paper-mark` 151, and `figure` 83. At the match offset, `u16-be` and `u16-le` rows expose the embedding index directly as the corresponding 16-bit value; `u32-le` rows expose it in the low 16 bits. This narrows later record-field analysis but still does not identify the authoritative geometry field.

`rjtd object-ownership-reference-fields <file>` projects those same preview offsets onto candidate record strides. For each target path, encoding, stride, and field offset it summarizes match count, source count, embedding index set, row-index preview, and cross-row match count. The command intentionally tests every reported offset against a fixed stride set (`4,8,12,16,20,24,28,32,36,40,44,48,52,56,60,64,68,72,80,84`), so the output is a hypothesis surface rather than a decoded record table.

The 61-sample `object-ownership-reference-fields` sweep succeeds with 0 failures and reports 33,492 projected field groups across the same 12 files. The strongest cross-row-free candidates with stride >= 12 are currently `frame/u16-le/12/5` with 102 weighted matches, `frame/u16-be/12/7` with 89, and `frame/u16-be/20/15` with 70. This suggests the next analysis should focus on frame records, but it still does not prove which stride or field is semantically authoritative.

`rjtd object-frame-reference-records <file>` expands the strongest frame projections into candidate row bytes. It currently reports only the three strongest decoded-false projection families (`u16-le/12/5`, `u16-be/12/7`, and `u16-be/20/15`) and prints source stream, embedding index, target stream, encoding, stride, field offset, match offset, row index, row start, row hex, BE/LE 16-bit field views, and BE/LE 32-bit field views.

The 61-sample `object-frame-reference-records` sweep succeeds with 0 failures and expands 261 rows across 12 files: `u16-le/12/5` 102 rows, `u16-be/12/7` 89 rows, and `u16-be/20/15` 70 rows. The rows expose repeated byte families such as `00010000000N000000020001`-style 12-byte rows and `00000000010200380000000N` rows. These families are promising evidence for frame-record analysis, but they are not yet decoded placement geometry or paint operations.

`rjtd object-frame-record-families <file>` groups those expanded rows into named decoded-false diagnostic families. The names are observation buckets, not specification terms: for example, `frame-index-flag-row12` captures 12-byte rows whose big-endian word view has small trailing flag-like fields, while `frame-index-tail-coordinate-row12` and `frame-index-tail-window20` capture the repeated tail-window shapes seen in `/Frame` reference projections.

The 61-sample `object-frame-record-families` sweep succeeds with 0 failures and groups the same 261 records across 12 files. Family counts are `frame-index-tail-coordinate-row12` 65, `frame-index-tail-window20` 65, `frame-index-mixed-row12` 61, `frame-index-flag-row12` 41, `frame-index-tail-zero-row12` 22, `frame-index-mixed-window20` 5, and `frame-index-tail-mixed-row12` 2. The next promotion step should compare these row families against page/layout marks and image payload ownership before treating any field as authoritative geometry.

`rjtd object-frame-row-links <file>` checks whether expanded 20-byte frame windows contain a matching 12-byte frame row as their suffix. This is intended to distinguish independent record candidates from context windows around a smaller record.

The 61-sample `object-frame-row-links` sweep succeeds with 0 failures. It finds 70 row20 windows in 9 files; 65 link to a same-source 12-byte suffix row and 5 remain unlinked. Every linked row is `frame-index-tail-window20 -> frame-index-tail-coordinate-row12`. This strongly suggests that the `u16-be/20/15` `tail-window20` projection is usually a context window around the `u16-be/12/7` row rather than an independent authoritative record. The unlinked rows are all `frame-index-mixed-window20`, so they should stay separate until more object/header evidence explains them.

Parser/export JSON now preserves this evidence as `objectStreamCandidates[].frameReferenceRows`. Each row records `targetPath`, `encoding`, `stride`, `fieldOffset`, match `offset`, `rowIndex`, `rowStart`, family, raw `rowHex`, optional `suffixLink`, and `decoded:false`. This keeps later image-placement work model-first: exporters should consume these model-owned rows rather than scanning raw `/Frame` streams directly.

The 61-sample JSON export sweep succeeds with 0 failures and preserves `frameReferenceRows` in the same 12 positive files. It exports 261 rows and 65 suffix links; family counts match the CLI family sweep exactly.

`rjtd object-image-frame-candidates <file>` summarizes this evidence from the image payload source point of view. For each source with complete image payload spans, it reports the path-derived embedding index, payload kinds, payload dimensions, dimensioned payload count, total frame rows, row-family counts, row12 coordinate-looking candidates, row20 suffix-link counts, LE row12 counts, a diagnostic preferred bucket, coordinate-looking row12 pairs, and the best coordinate/payload aspect delta when both sides have dimensions. The command is still decoded-false: the preferred bucket and aspect match are investigation priorities, not renderable geometry.

The 61-sample `object-image-frame-candidates` sweep succeeds with 0 failures. It finds 12 files with image payload sources, 52 image sources, 52 frame-linked sources, 0 sources without `/Frame` rows, 261 frame rows, 35 dimensioned payloads, and 13 sources with coordinate/payload aspect candidates. Diagnostic preferred buckets split as `row12-tail-coordinate` 25, `row12-tail-zero` 7, `u16-le-row12` 19, and `none` 1. Only two sources currently have best aspect deltas at or below 250 permille, both in `natsu.jtd`; therefore `row12-tail-coordinate` remains promising, but aspect evidence is still too sparse to promote PDF image rendering.

`rjtd object-fdm-index <file>` inspects `/FigureData/*/FDMIndex` streams against their sibling `/FigureData/*/FDMVector` streams. The current observed row shape is a 20-byte header followed by 22-byte rows carrying a big-endian vector offset, a 16-bit kind field, and four big-endian signed bbox-like fields. The command links each row to the vector segment ending at the next greater vector offset and reports segment image signatures as decoded-false evidence.

The 61-sample `object-fdm-index` sweep succeeds with 0 failures. It finds 31 files with indexes, 39 index streams, 417 parsed rows, 6 rows with image signatures, 13 image hits, and 2 missing sibling vectors. This proves `FDMIndex`/`FDMVector` is a separate object-placement evidence path from `Embedding N` `/Frame` rows.

`rjtd object-fdm-index-shape <file>` separates those raw row projections into shape families. It distinguishes exact 22-byte tables, declared-count prefix tables followed by auxiliary payload bytes, mixed declared rows, unknown-header streams, and missing sibling vectors.

The 61-sample `object-fdm-index-shape` sweep succeeds with 0 failures. It finds 39 indexes, 35 `fdm-index-v1` headers, 4 unknown headers, and 34 plausible declared counts. The raw whole-stream 22-byte projection has 417 rows and 252 invalid offsets, but the declared-count prefix projection has 147 rows, 43 invalid offsets, and the same 13 image hits. Shape counts are `row22-count-prefix` 17, `row22-exact` 14, `row22-mixed-declared` 3, `unknown-header` 3, and `missing-vector` 2. This means many previously invalid rows are likely auxiliary payload bytes after the FDMIndex table, not real FDMIndex rows.

`rjtd object-fdm-index-rows <file>` prints a row-level diagnostic view for those tables. It reports each row's scope (`declared`, `post-declared`, or `raw`), role (`vector-segment`, `coordinate-like-invalid`, or `invalid-vector-offset`), BE16/i16 field views, raw row bytes, and segment image signatures. The role is decoded-false: `coordinate-like-invalid` means the row resembles signed-coordinate data when viewed as BE16 fields, not that its semantic record type is decoded.

The 61-sample `object-fdm-index-rows` sweep also succeeds with 0 failures. It finds 31 files with indexes, 39 indexes, 417 rows, 147 declared rows, 253 post-declared rows, 17 raw rows, 165 valid vector rows, 252 invalid rows, 13 image hits, and 2 missing vectors. Role counts are `vector-segment` 165, `coordinate-like-invalid` 231, and `invalid-vector-offset` 21. All 43 declared invalid rows are `coordinate-like-invalid`, concentrated in three files, and none of those invalid declared rows are image-bearing vector segments.

Parser/export/app-core JSON now preserves the declared-count prefix rows as `objectStreamCandidates[].fdmIndexEntries` on the corresponding `FDMVector` candidate. Each row records `indexPath`, `vectorPath`, row/index/vector offsets, vector segment length, `kind`/`kindHex`, bbox-like fields, `validVectorOffset`, vector prefix, absolute image signatures, segment-relative image signatures, and `decoded:false`.

The 61-sample JSON export sweep succeeds with 0 failures and preserves 147 `fdmIndexEntries` in 30 candidates across 24 files. Three files have image-linked rows: 6 rows contain 13 image hits. The sweep also reports 104 valid vector offsets and 43 invalid or out-of-range vector offsets, so this evidence identifies the currently observed image-bearing FDMVector segments while reducing false auxiliary rows. It must still not be promoted to renderable page geometry or paint resources.

`rjtd object-fdm-image-candidates <file>` summarizes the image-bearing subset of model-owned `fdmIndexEntries`. For each row with segment image signatures, it reports the FDMVector source, FDMIndex source, row/vector offsets, `kind`, raw and normalized bbox-like fields, bbox order, bbox plausibility, segment image hits, complete image payload count, and `renderable:false`. Complete payload rows remain blocked as `page-placement-unproven`; signature-only rows use the narrower canonical blocker `image-signature-without-complete-payload-role-unproven` so payload extraction failure is not confused with page placement.

The 61-sample `object-fdm-image-candidates` sweep succeeds with 0 failures. It finds 3 files with candidates, 3 FDM sources, 6 image-bearing rows, 13 image hits, 0 strict complete payloads, 5 plausible bbox rows, and 0 renderable rows. The observed `FFD8FF` hits in FDMVector segments are JPEG-like byte patterns inside vector data, not valid JPEG payloads with SOF/SOS structure, so they remain signature-only diagnostic evidence.

PDF-backed `shanai_lan` diagnostics show that FDMIndex row bbox-like fields must be normalized as an entry-local axis pair (`x0,x1,y0,y1`) for entry extents and connector-relation gates. This normalization is not applied to FDMVector command source bboxes, which already use left/top/right/bottom order. After axis-pair normalization, the FDMIndex entry extent nearly matches the active command extent (`maxAbsDelta:36` source units), but it still does not prove the page-placement transform; the generated `RAID` glyph component remains about `10.7px` left and `12.1px` high against the reference PDF crop.

The same sample now preserves a decoded-false `fdmTextMaskSourceTransformCandidateSummary`. The current best bridge maps the top text-like FDM mask component for `RAID` to the `/DocumentText` slot `5`, yielding a reference-backed current grid offset range `29.285..38.632` and `sourceUnitsPerTextGridUnitX:38.196`. This is useful transform evidence, but it remains non-rendering until the text grid origin is source-derived, the same line-header run can be used as an unambiguous row anchor, and the mask-to-text baseline transform is validated across more samples.

`shanai_lan` line-header diagnostics also preserve a decoded-false `gridOriginAuthorityGate`. The selected visible line headers have complete LineMark coverage and a uniform source-domain fit: `/LineMark.recordIndex - /DocumentText.groupIndex == 1`, with PageMark entry row `0` covering the selected records (`lineStart:0`, `lineEnd:39`). This proves a row-domain relation but still not a page-space origin, so the current text grid remains reference-backed and non-promotable.

The raw `documentTextLineRuleProjection` candidates now carry diagnostic-only render admission gates at both rule and graph-component granularity. Each rule records component membership, LineMark coverage, orthogonal/topology readiness, endpoint text-attachment candidates, and explicit blockers (`document-text-grid-origin-reference-backed`, endpoint ownership, style role, and paint order). The summary gate aggregates the current PDF-backed `shanai_lan` state as `ruleCount:16`, `componentCount:6`, `orthogonalComponentCandidateCount:3`, and `bothEndpointAttachmentWithinLineHeightRuleCount:0`, with `promotionReady:false`. This intentionally prevents promoting a visually tempting single line-rule overlay until source-derived page origin, endpoint ownership, style role, and paint order are decoded.

The same gate also preserves `selectedLineMarkSourceUnitGate` evidence. Selected LineMark records `[22,32]` have source-unit starts `[3908,5483]`, ends `[4121,5615]`, spans `[213,132]`, record delta `[10]`, and unit-start delta `[1575]`. This is useful source-domain stride evidence, but with only one stride sample and no source-unit-to-page-y transform, it remains non-promotable.

The same gate includes a `sourceOnlyPageMarkYValueProbe`. PageMark entry row `0` yields 58 parsed y-value candidates; the closest current-origin probe is `parsedEntryU16` word `7` / byte `14`, value `39`, only `0.300px` from the current reference-backed origin. A companion `pageMarkEntryProfileGate` records the selected entry as `mixed-payload` rather than an additive layout-origin profile, and `lineBoundaryConflictGate` records that the same value matches the entry `lineEnd=39`, so this remains a blocked probe until the field role is proven independently from line-boundary semantics.

App-core `getPageOverlayImages` exposes the same FDM rows as `unplacedDiagnostics` while keeping `behind`, `front`, and `imageCount` empty/zero. Each diagnostic has `placementProven:false`, `renderable:false`, `decoded:false`, and reason `page-placement-unproven`. This keeps the rhwp-shaped overlay API callable without claiming decoded page placement or paint resources.

Parser/export/app-core JSON also preserves `/Frame` fixed 60-byte records as decoded-false `objectFrameRecords`. The observed record layout has a 16-byte header, a big-endian declared count at offset 14, and 60-byte rows. The rows expose an object id, record kind/type, and geometry-looking fields, but these fields remain diagnostic until units, page association, and paint order are proven.

Image payload spans now carry optional `dimensions` in model/export JSON. Standard image dimensions are read through the rhwp-aligned `image` dependency, with a narrow JPEG SOF metadata fallback. JPEG payload spans are only promoted when the byte stream has valid SOF/SOS structure; SOI/EOI-like byte pairs alone are not enough.

Image payload render admission now has a stricter source-backed gate. The model exposes `ownershipProven` only when the stream path ownership candidate, payload source-path candidate, and cross-stream ownership references are all present. It separately reports `/Frame` reference-row evidence (`frameReferenceRowCount`, `frameCoordinateRowCount`, `frameLinkedWindowRowCount`, `frameGeometryCandidatePresent`), an `EmbeddingInfo -> /Frame` trace (`sourceFrameTrace`, `embeddingFrameTracePresent`, `sourceFrameRecordGeometryPresent`), a diagnostic-only page-space candidate bbox (`candidateFrameBBox`) derived from the traced `/Frame` record, and payload-to-frame aspect-fit diagnostics (`payloadFrameAspectFit`, `aspectDeltaPermille`, `bestPayloadAspectDeltaPermille`, `currentPayloadBestFrameAspectCandidate`). Even when all of those source facts are present, the payload remains `renderable:false` and `pageGeometryProven:false`; the most advanced current blocker is `image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven`, while candidate-frame placement and payload aspect matching remain diagnostic-only as `page-assignment-and-paint-order-unproven` / `payload-selection-page-assignment-and-paint-order-unproven`. This prevents `/Frame` geometry-looking fields or aspect-compatible image payloads from being used as page placement or paint-order authority before their semantics are decoded.

`rjtd object-fdm-frame-links <file>` correlates image-bearing FDMIndex rows with those `/Frame` records by `fdm row index == frame object id` and reports frame size, payload dimensions, dimensioned payload counts, and best frame/payload aspect delta when payload dimensions exist. The 61-sample sweep succeeds with 0 failures and finds 3 positive files, 6 FDM image rows, 13 image hits, 6 frame-linked rows, 0 missing-frame rows, 0 strict complete payloads, 0 dimensioned payloads, and 0 renderable rows. This proves that the currently observed FDM image rows have a frame-record trail, but it still does not prove that any FDMVector signature hit is a paintable image payload.

The model, SVG diagnostics, and CLI tests now distinguish complete frame-linked payload evidence from signature-only FDMVector fragments. Frame-linked complete payload fixtures stay `renderable:false` with the source-backed blocker `fdm-frame-linked-image-payload-placement-and-paint-order-unproven`, while real signature-only `shanai_lan` rows continue to report `image-signature-without-complete-payload-role-unproven`. Neither blocker promotes generated output to source truth; both describe preserved model/test evidence until page assignment, payload role, and paint order are decoded.

For the PDF-backed `success_data-test` title art, JSFart/EmbeddedPress paint candidates are now preserved as decoded-false `sourcePaintRenderTrace` evidence before any render promotion. The observed JSFart paint words are `styleWord1=0x02141030`, `styleWord2=0x02141018`, `paintColorCandidate=0x00ffffff`, `paintFlagCandidate=0x00000001`, and `effectWordCandidate=0x0000000a`. The active renderer still selects the conservative front fill `#111111`; the trace conclusion is `source-paint-present-but-render-fill-not-promoted` because the source paint is white, does not match the selected fill, and the source-order interstitial texture path remains blocked as `front-erase-texture-over-main-face-semantics-unproven`.

The same title-art projection preserves decoded-false `sourceFrameRenderTrace` evidence linking the JSFart frame candidate to the `/Frame` row. For the first title frame, `frameRef` and `/Frame.objectId` both equal `1`, and both sources agree on outer width `13260` and outer height `1327`; content dimensions are `13031x1054`. This is useful source coherence, but it does not promote frame-edge placement because the active horizontal placement still depends on unresolved frame/content split semantics and remains blocked as `frame-content-split-horizontal-semantics-unproven`.

The local JSFart2Contents sweep finds eight files with `/JSFart2Contents` object-path candidates, but only the two `success_data-test` streams currently decode as `jsfartArt` under the `MSTUDIO...` magic gate. Both decoded streams share `paintColorCandidate=0x00ffffff`, `paintFlagCandidate=0x00000001`, and `effectWordCandidate=0x0000000a`; `styleWord1/styleWord2` differ between the two title frames. These words therefore remain source evidence, not corpus-wide render authority.

Parser/model/export JSON now exposes `jsfartStreamProfile` for every `/JSFart2Contents` stream, including non-`MSTUDIO.OCX` variants. The profile is source-backed and decoded-false only: it records the prefix family (`mstudio-ocx-utf16le`, `jsfart-object-utf16le`, `zero-prefix`, etc.), `magicFamilyHex`, UTF-16 preview, header prefix, and whether the stricter `jsfartArt` candidate was present. Its `renderable` value stays false, and non-MSTUDIO variants remain blocked as `jsfart-variant-layout-undecoded`. The `rjtd object-stream-candidates` CLI command also reports the profile count/family so sweep output can separate non-MSTUDIO source evidence from structured `jsfartArt` candidates.

For the PDF-backed `success_data-test` sample, Q4/Q5 FDM reference projections now preserve an additional decoded-false `offsetFieldAuthorityGate` inside `primitiveOwnershipComparison`. The gate compares FDMIndex row `bbox.left` reference candidates against both command-relative offsets and source-segment-relative offsets before any render promotion. Q4 currently has 20 references split as 18 command-relative and 2 source-segment-relative; Q5 has 7 references split as 1 command-relative and 6 source-segment-relative. Both are blocked as `fdm-index-offset-field-authority-mixed-command-and-segment-fields`, which means the source data proves useful row-command links but not yet the authoritative offset namespace for ownership or paint-order promotion.

The same projections also preserve decoded-false `rowFanoutSegmentOwnerGate` evidence. Q4 has no row fanout (`20` references over `20` unique rows, `maxRowFanout:1`) but remains blocked by mixed offset namespaces. Q5 has three unique rows for seven references: row `40` backs four commands and row `41` backs two commands, with all six fanout references using the source-segment-relative offset field. This is useful source evidence, but it is not render authority until segment ownership and paint order explain why one FDMIndex row legitimately owns multiple vector commands.

The role-level `indexRowReferenceRoleCandidateGroups` entries now preserve the same decoded-false fanout question as `roleFanoutSegmentOwnerGate`. In Q5, the `line-candidate` role narrows the projection-level fanout to a concrete role blocker: command references `1992` and `2024` both map to FDMIndex row `40`, all fanout references use source-segment-relative offsets, and the role is blocked as `fdm-index-role-row-fanout-multi-command-single-row`. This makes role rendering dependent on explaining row fanout and segment ownership, not just on command-span paint-order continuity.

The same `primitiveOwnershipComparison` path now carries a decoded-false `primitiveOwnershipAdmissionGate` before any render promotion. The gate aggregates `ownershipGate`, `offsetFieldAuthorityGate`, `rowFanoutSegmentOwnerGate`, role-level `roleFanoutSegmentOwnerGate`, and unresolved paint-order continuity into a single blocker list. Q4 remains blocked by mixed raw/segment cohorts, mixed offset namespaces, missing valid vector-offset role references, and paint-order continuity. Q5 additionally records projection-level and role-level row fanout (`roleFanoutBlockedGroupCount:3`). The row/index `ownershipGate` no longer injects a generic primitive-role paint-order blocker; paint-order uncertainty is carried by the specific `role-paint-order-continuity-unproven` admission blocker and the role-level profiles. This gate is source-backed diagnostic evidence only; it does not promote primitive ownership or paint order.

The role-level paint-order profile now separates span continuity from render authority. A role whose command span contains interleaved non-role commands is blocked as `role-span-interleaved-non-role-commands`; a role whose source span is contiguous is still blocked as `role-paint-order-authority-unproven` until the FDMIndex row order is proven to be the authoritative paint order. In the observed Q5 solid diagram this splits the four role groups into two continuity-blocked roles (`arc-candidate`, `connector-candidate`) and two authority-pending roles (`line-candidate`, `surface-boundary-candidate`) without rendering any new primitive.

`indexRowOrderPromotionGate` also reports a concrete blocker list rather than the earlier generic primitive-role blocker. Q4 has monotonic one-to-one row-command evidence but remains blocked by missing valid FDMIndex vector offsets, mixed command-relative/source-segment offset namespaces, and role paint-order continuity. Q5 reports row-order shape failures first (`fdm-index-row-order-reference-not-one-to-one` and `fdm-index-row-order-single-row-backs-multiple-commands`) before the shared valid-vector, namespace, continuity, and authority blockers.

Each role group now also carries decoded-false `roleVectorOffsetAuthorityGate` evidence before fanout is considered. The gate makes the vector-offset question explicit: current role references are found through FDMIndex offset fields, but the matching rows still have invalid `FDMIndex.vectorOffset` values, so all Q4/Q5 role groups are blocked as `fdm-index-role-vector-offset-authority-valid-vector-offset-missing`. This keeps `bbox.left`-style offset matches useful as source evidence while preventing them from being mistaken for proven vector-offset ownership.

For the PDF-backed `success_data-test` ABC table, `sourceOnlyAxisAdmissionGate` now separates the active source-derived page-space solver from selector-only fallback diagnostics. If the table is already renderable from source-backed line-header units, exact `/LineMark` rows, `lineMarkPageGrid` origin, and source y-origin solver, the gate reports `activeSourceLayoutAdmissionReady:true` and `admissionReady:true`; missing selector-only horizontal evidence or single-support y fallback evidence is preserved as diagnostic-only and no longer blocks the active source-layout path. The `tsaiten` table gates remain negative because they do not have a renderable active source layout and still depend on fragmented or single-support y selector evidence.

The same ABC table now reports the outer `topTextTableSourceGapEvidence` from the nested source-only readiness state. Once `sourceTopTextPlacementReadinessGate` proves the preceding instruction anchor, selected visible width, trailing header semantics, and adjacent `/LineMark`/`/PageMark` coupling, the outer evidence records `sourceTopTextPlacementReady:true`, no blocked reasons, and no render-promotion blocker. Reference bbox residuals remain diagnostic evidence only and do not supply render authority.

`totalWidthSemanticsGate` now distinguishes a true width decoder blocker from a next-gate handoff. When trailing/header evidence makes the selected visible range source-ready but the full line extent remains wider, the gate first records the `source-table-placement-coherence-gate` handoff. When that same source-only top-text/table placement coherence is present, the handoff is closed with `sourcePlacementCoherenceGateEvidencePresent:true`, `sourcePlacementCoherenceGateResolved:true`, `renderPromotionNextGate:null`, and no render-promotion blocker for the selected visible range. Samples without trailing/header evidence continue to report `source-total-width-semantics-unproven`.

For the PDF-backed `tsaiten` tables, the line-domain plus post-row-gap projection probe now carries a nested `sourceOnlyProjectionDomainGate`. The parent probe may still record reference residuals as diagnostics, but the nested gate is explicitly `referenceBacked:false` and keeps the source-only blockers separate: cross-domain treatment of line-domain y plus PageMark subrecord-gap units, selected records identified as post-row-gap records, incomplete or non-ordered-unique span coverage, and the still-missing page-y transform. This prevents near-reference residuals such as the scoring table's `235.087 + 65 -> 300.087` projection from being treated as render authority.

The same `tsaiten` y path now computes `sourceOnlyPageMarkAbsoluteYSlotGate` from source agreement instead of an unconditional semantic blocker. The gate chooses the best PageMark absolute-y slot relative to the line-domain plus post-row-gap projection and only clears `page-mark-absolute-y-slot-semantics-unproven` when those source candidates agree. Current candidates do not agree, including the lower table projection `875.539` versus absolute slot `768.000`, so the blocker is the more specific `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`.

That disagreement is now mirrored into the `sourceOnlyPageYOriginSelector` support path. The lower `tsaiten` table may still expose `selectedY:768.000` as a single-support source diagnostic, but the selector support and agreement group carry both `page-mark-absolute-y-slot-semantics-unproven` and `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`. A future source-only promotion therefore cannot treat the raw `768` slot as placement authority unless the line-domain projection also agrees.

The source-gap-to-page-line transform readiness is also mirrored into `sourceOnlyPageYOriginDomainGate` and `sourceOnlyPageYRenderAdmissionGate` through `sourceGapToPageLineGapTransformAdmissionGate`. This source-only gate reports `transformDomain:"source-unit-gap-to-page-mark-line-index-gap"`, `canDecodeSourceTransform:false`, `tableFamilyTransformStable:false`, best candidate `segment-offset-gap`, max delta `105`, and blockers `source-gap-to-page-line-gap-transform-not-stable`, `source-gap-to-page-line-gap-transform-unstable-across-table-family`, and `source-gap-to-page-line-gap-transform-undecoded`. Render admission therefore remains connected to decoded transition evidence rather than to reference table bboxes.

The combined `sourceOnlyAxisAdmissionGate` mirrors the same y-selector support blockers, PageMark absolute-slot agreement fields, and source-gap transform admission gate. This keeps the x/y coupling diagnostic source-only all the way down: the scoring table carries fragmented cross-table y evidence and the lower table carries the raw `768.000` slot only with `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`, residual `107.539`, and the undecoded source-gap transform blockers. Axis coupling therefore remains non-rendering until the y path is decoded, not merely selected.

`sourceGapToPageLineGapReadinessHints` and every mirrored `sourceGapToPageLineGapTransformAdmissionGate` now preserve candidate-level transform taxonomy. They report candidate count, exact-match count, best-candidate transition coverage, best-candidate spread, the lowest-spread candidate, full candidate summaries, and declined transform candidates with decline reasons. Current `tsaiten` evidence intentionally remains blocked because the best max-delta candidate is `segment-offset-gap` (`105`) while the lowest-spread candidate is `direct-source-range-gap` (`12.250`), proving that the transition rule is not yet stable enough for source-only page-y promotion.

`sourceOnlyAxisAdmissionGate` also emits a diagnostic-only `sourceOnlyAxisCandidateBBox`. This combines the best source-only horizontal candidate with the selected source-only y candidate without reference bbox selection. In the current `tsaiten` sample, the scoring table candidate is `174,235.087,421,93.192` and the lower table candidate is `174,768,554,63`. Both are source-backed comparison targets only: `selectionReady:false`, `referenceBacked:false`, and still blocked as `source-page-space-axis-selector-coupling-unproven`.

`sourcePageYTransformGate` now also includes `sourceOnlyPageYRenderAdmissionGate`, a source-only admission contract for future removal of reference table calibration. The gate reports `admissionReady:true` for the already source-rendered ABC table when a direct `lineMarkPageGrid` origin, exact `/LineMark` rows, and decoded y solver are present. The same gate keeps all current `tsaiten` table candidates non-rendering without consulting reference coordinates: scoring is blocked by missing direct origin, cross-table line-domain evidence that is not a page-space origin, fragmented selector support, source/subrecord ordering contradiction, and PageMark absolute-slot disagreement; the lower table is blocked by stride-only origin, non-exact line rows, single-support fallback, non-unique selected post-row-gap coverage, and the same PageMark disagreement.

Table layer diagnostics now also expose `referenceFallbackAdmissionGate`, a behavior-preserving audit surface that connects the existing visible reference-fallback boolean to the source-only page-y admission contract. The gate reports that `success_data-test` ABC tables suppress reference fallback while using the renderable source-derived layout (`referenceFallbackAllowed:false`, `referenceFallbackUsed:false`, `sourceOnlyPageYAdmissionReady:true`). Current PDF-backed `tsaiten` visible tables still use the legacy reference fallback, but they now carry explicit source replacement blockers such as `source-derived-layout-candidate-absent` and `source-page-y-render-admission-not-ready`, making the remaining calibration-removal work visible without changing rendered output.

`tsaiten_table_grid_overlay_layout` is no longer used by the generic `table_grid_overlay_layout` fallback path. Explicit reference helpers and reference-backed diagnostics still expose the legacy calibration, so visible `tsaiten` output remains unchanged, but generic table bbox fallback now uses source-derived layout or the generic page/body-anchor heuristic instead of silently borrowing `tsaiten` constants. This narrows the future removal surface to `reference_table_grid_overlay_layout` and reference-only probes.

`pageMarkScopedYTransformProbe` now declares its reference-target status at the probe top level. It emits `referenceBBoxUsed:true`, `referenceTargetBasis:"referenceTableBBox.rowTopTargets"`, and `sourceOnlyReplacementBlockedReason:"page-mark-scoped-y-transform-targets-reference-backed"` beside the source-backed PageMark/LineMark record matches. This keeps the reference row-top residual probe available for calibration diagnostics while preventing it from being mistaken for source-only render authority; source-only promotion must come from `sourceOnlyPageYOriginSelector`, `sourceOnlyPageMarkAbsoluteYSlotGate`, and `lineHeaderLineMarkCouplingEvidence`.

## Next Work

- Decode the semantic object header fields preceding image payload signatures and connect them to `/Figure`, `/Frame`, `/LayoutBox`, and layout mark evidence.
- Decode `/Frame` geometry units, page association, paint order, payload-to-image selection, and the remaining coordinate-like FDMIndex diagnostic rows before rendering FDMVector images.
- Prove FDMIndex offset-field authority for mixed command-relative/source-segment references before promoting Q4/Q5 FDM primitive ownership or paint order.
- Prove whether `FDMIndex.vectorOffset`, `bbox.left`, or another row-local field is the authoritative role ownership reference before clearing `roleVectorOffsetAuthorityGate`.
- Resolve `primitiveOwnershipAdmissionGate` blockers across ownership, offset namespace, row fanout, role fanout, valid vector-offset references, and paint-order continuity before admitting any Q4/Q5 primitive to rendering.
- Prove which `Embedding N` reference encoding and record-local offset is semantically authoritative before promoting ownership references into page geometry.
- Connect preserved image payload bytes to model-level image resources only after object ownership and page geometry are proven.
- Build real page/layer paint operations from decoded object and layout records before adding non-text PDF rendering.
- Investigate table semantics through `/DocumentText` control ranges and layout/style streams rather than stream-name matching.
