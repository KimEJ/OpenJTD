# rhwp Compatibility Reference

OpenJTD uses `rjtd` as its current Rust implementation. `rjtd` takes as much as
possible from rhwp's structure and development approach instead of designing a
separate architecture from scratch.

The top-level workspace therefore keeps `rhwp/` as a local reference repository.

## Source

```text
https://github.com/edwardkim/rhwp.git
```

Current local clone reference:

```text
branch: main
commit: bc38ff55
```

## Clone Note

The rhwp repository includes Git LFS files. Because large PDF smudging can currently fail due to GitHub LFS budget limits, clones used only for source-structure reference should skip LFS downloads.

```sh
GIT_LFS_SKIP_SMUDGE=1 git clone https://github.com/edwardkim/rhwp.git rhwp
```

## Policy

- Use `rhwp/` as a read-only reference repository.
- Do not copy `rhwp/` code into rjtd.
- Compare structure, layer separation, testing style, and API philosophy first.
- Keep actual rjtd implementation code under `rjtd/`.
- Treat `rhwp/` as an external reference repository.

## Dependency And Implementation Policy

rjtd follows the rhwp approach for both dependency selection and direct implementation scope.

- If a new dependency or implementation approach is needed, first inspect `rhwp/Cargo.toml` and the rhwp implementation.
- If rhwp already uses a crate for the same task category, rjtd should prefer that crate and approach.
- If rhwp directly implements the same task category, rjtd should prefer direct implementation instead of adding a new dependency.
- Only pause and request an explicit decision when the problem has neither an rhwp dependency nor a direct-implementation precedent.
- Do not add a dependency that rhwp does not use purely for convenience.

Relevant dependency patterns currently confirmed in rhwp:

- CFB/OLE container: `cfb`
- Deflate compression/decompression: `flate2`
- binary endian helpers: `byteorder`
- ZIP/HWPX-style archives: `zip`
- legacy encodings/code pages: `encoding_rs`, `codepage`
- JSON model/debug output: `serde`, `serde_json`
- native SVG-to-PDF export: `usvg`, `svg2pdf`, `pdf-writer`

Current JTTC conclusions:

- Local `.jttc` samples show a `JustCompressedDocument` marker and `-lh5-` marker inside `/JSCompDocument`.
- rhwp has no LHA/LZH/LH5 dependency.
- Therefore rjtd does not add an LHA/LZH dependency without a separate decision.
- rjtd directly implements only the observed single `-lh5-` member profile to read `/DocumentText` from the inner CFB of `.jttc` files.
- The current implementation does not yet support LHA header checksums, CRC verification, multi-member archives, or other LHA methods.

Current malformed CFB conclusions:

- Some additional local `.jtd` samples fail in the standard `cfb` reader with `Malformed FAT`.
- rhwp directly implements `LenientCfbReader` in `rhwp/src/parser/cfb_reader.rs` and uses it as a fallback when the standard reader fails.
- rjtd therefore directly implemented a lenient CFB fallback in `rjtd-core::container` instead of looking for a new dependency.
- The fallback is used only after standard `cfb` parsing fails, and all 61 local samples have been confirmed to open with `rjtd info`.
- Two samples without `/DocumentText` are classified as `cfb-embedded-document-text` with raw `SsmgV.01`/`TextV.01` fragments.
- `rjtd cat` works on all 61 local samples.

Current DocumentText conclusions:

- rhwp creates raw records at the parser stage, then the model stage consumes them.
- rjtd mirrors this by keeping a `ParsedDocumentText` token layer in `rjtd-core` and having `rjtd-model` consume it.
- This stage only needs a UTF-16BE code-unit walk and observed control-boundary handling, so it is implemented directly without a new dependency.
- Inline segments skipped in plain text are still preserved as `SkippedInlineText` tokens and `UnknownObject` payloads.
- Observed ruby pairs now lift a visible base inline (`selector 0x0003`) plus phonetic annotation (`selector 0x0082`) into `Inline::Ruby`; plain text, Markdown, and PDF output continue to show the base text while JSON preserves the annotation text and raw payload.
- rhwp parses style records into model objects while also retaining original `raw_data`; rjtd now mirrors the preservation half by lifting observed style/layout streams into named `UnknownStyle` model entries before their record semantics are decoded.
- rjtd now also exposes neutral style stream record boundary candidates: Ssmg slot records with observed `0x5555`/`0x4444` markers, conservative UTF-16BE label candidates, and sequential `u16 code + u16 payload_len + payload` records where the stream supports that layout.
- `/DocumentTextPositionTables` is also separated into a diagnostic parser and `text-positions` command so unverified offset meanings do not leak into model generation.
- True `DocumentText` record boundaries and full style/ruby/layout semantics are still undecoded.

Current application core / PDF conclusions:

- The rhwp app surface wraps `DocumentCore` with `HwpDocument` and first exposes open, page count, document info, and SVG rendering.
- rjtd first implemented `from_bytes`, `page_count`, `get_document_info`, `get_page_info`, `render_page_svg`, `render_page_html`, `get_page_layer_tree`, `get_page_overlay_images`, `get_canvaskit_replay_plan`, `set_file_name`, `get_source_format`, and `convert_to_editable` on `rjtd-model::DocumentCore`.
- rjtd then added text-line mapping to `DocumentCore` so the current page fallback can answer rhwp-shaped cursor and hit-test calls: `get_cursor_rect`, `hit_test`, `get_line_info`, and `move_vertical`.
- rjtd now supports a minimal editable body-text path on the app core: `insert_text`, `delete_text`, `split_paragraph`, `merge_paragraph`, `get_text_range`, `get_paragraph_length`, and `get_paragraph_count`. These update the fallback page cache so SVG/PDF/canvas/cursor APIs see the edited model.
- rjtd now has a model-owned fallback `WritingMode` path. `DocumentCore`, SVG page rendering, and `getPageLayerTree` can project `horizontal` or `vertical-rl`, but `writingModeDecoded:false` remains set until style/layout streams prove the real Ichitaro writing direction.
- Body selection and internal clipboard APIs are implemented for fallback text pages: `get_selection_rects`, `delete_range`, `copy_selection`, `paste_internal`, `has_internal_clipboard`, and `get_clipboard_text`.
- Undo snapshots are implemented for the fallback app core through cloned `Document`/page/caret/clipboard state: `save_snapshot`, `restore_snapshot`, and `discard_snapshot`.
- Body text search and replacement now follow the rhwp Studio JSON shapes for `searchText`, `searchAllText`, `replaceText`, `replaceOne`, and `replaceAll`. The current scope is body paragraphs only because rjtd has not decoded JTD table/cell/textbox structures yet.
- View-state toggles now mirror the low-risk rhwp app flags: paragraph marks, control codes, transparent borders, and clipping. They are stored on `DocumentCore`, but they do not yet alter true Ichitaro layout because the fallback renderer has no decoded object/control paint tree.
- Page-position lookup and document-tree navigation fallbacks now provide rhwp-shaped JSON for `getPositionOfPage`, `getPageOfPosition`, `findNextEditableControl`, `findNearestControlBackward`, `findNearestControlForward`, `getControlTextPositions`, and `navigateNextEditable`. Because rjtd has not decoded selectable JTD controls yet, true object navigation still falls back, while preserved `/DocumentText` boundary projections may appear as `type:"jtdControl"` diagnostics. Text navigation moves through body paragraphs.
- Field, header/footer, footnote, and endnote API surfaces now expose no-hit/no-op fallback JSON for rhwp Studio panels and edit modes. These methods intentionally return `inField:false`, `exists:false`, `ok:false`, empty lists, or conservative default settings until JTD field, note, and header/footer structures are decoded.
- Table and cell API surfaces now expose no-hit/no-op fallback JSON for rhwp Studio table commands, cursor paths, selection, formatting, and formulas. These methods return empty dimensions/bboxes/text, default cell/table properties, `ok:false` edit results, and conservative row/column/cell counts until JTD table structures are decoded.
- Picture, shape, equation, bookmark, form-object, stable-id, control-copy, and image-data API surfaces now expose no-hit/no-op fallback values. These keep rhwp Studio object panels and keyboard paths callable while JTD object/control records are still being decoded.
- HWP/HWPX export, HTML paste/export, page/column/new-number commands, header/footer formatting, footnote editing, style mutation, and numbering creation now have explicit fallback behavior. HWP/HWPX export returns empty bytes plus `exportHwpVerify` failure JSON rather than pretending to write a valid Hancom document.
- Basic formatting panel APIs now return conservative fallback values: default character properties, paragraph properties, a single `Normal` style, empty numbering/bullet lists, and no-op style/format application.
- Style APIs now expose preserved JTD style stream sources honestly: `getDocumentInfo`, `getStyleList`, and `getStyleDetail` report source stream counts/names/sizes, observed stream family (`ssmg`, `table`, or `unknown`), big-endian header fields, record candidates, and `decoded:false`.
- `getDocumentInfo` also includes `styleCandidateCount` and `styleCandidateNames` so the app can discover observed JTD style candidates before loading the full style list.
- `getStyleList` and `getStyleDetail` also expose labeled `/TextLayoutStyle` records as rhwp-shaped JTD style candidates. These entries retain `decoded:false`, source stream/offset/code metadata, and fallback char/paragraph properties until the real style IDs can be attached to paragraphs and text runs.
- `applyStyle` now persists those candidate IDs into the in-memory `Paragraph` style reference and `getStyleAt` reports the applied candidate. This mirrors rhwp's edit surface direction without claiming JTD stream mutation support yet.
- CLI diagnostics now mirror the same observation layer: `rjtd style-records <file>` prints preserved style stream summaries and record offsets/codes, while `rjtd style-candidates <file>` lists labeled `/TextLayoutStyle` candidates for cross-sample correlation.
- `rjtd text-layout-style-records <file>` prints all `/TextLayoutStyle` records, including unlabeled records, payload lengths, payload digests, short payload previews, BE16 fields, and the candidate IDs assigned to labeled records.
- `rjtd document-view-style-groups <file>` prints `/DocumentViewStyles` group records with payload lengths, payload digests, and short payload previews so group IDs can be compared before they are attached to the app-core model.
- `rjtd text-position-style-context <file>` correlates `TCntV.01` tail fields with text/page style candidate IDs and source record indexes. The command reports hits as evidence only; rjtd still does not attach parse-time style references from these fields.
- `rjtd text-position-style-summary <file>` aggregates those hits per tail field and also compares field values against `/DocumentViewStyles` group records such as `0x3104..0x3907`. The current local sweep shows `f1` group hits in the finance-like samples, but non-finance samples with `TCntV.01` entries have zero `/TextLayoutStyle` records and large `f1` values outside the 1..9 view-style group range, while `f7` remains near-constant and `f4` appears in one group-hit sample. The app-core model therefore still keeps these fields as diagnostics until the record semantics are proven.
- `rjtd text-position-count-tail-field-roles <file>` compares tail fields and adjacent field pairs against document-text unit/text hits at selected deltas. The local sweep strengthens the diagnostic-only decision: `f1/f2` often scores as a range-like pair, while `f7` is near-constant and usually has no direct text hit.
- Parsed `TextRun` model data now carries `/DocumentText` byte and UTF-16 source spans, mirroring rhwp's preference for UTF-16-positioned text metadata. Valid `TCntV.01` text-count entries are preserved as decoded-false `textCountRanges`; JSON export and `getDocumentInfo` expose their observed family, chosen start/end/span, declared start/end, tail fields, raw bytes, and byte/unit `documentTextOverlaps` against model text runs without attaching them to paragraph styles or true layout geometry yet.
- `/DocumentText` control boundary codes are also preserved as decoded-false `textControlBoundaries` with byte/unit source spans where known. rjtd does not yet promote these controls to paragraph, line, style, or object semantics; they remain evidence for the next boundary/layout inference pass.
- `controlRangeOverlaps` are also lifted into decoded-false `textBoundaryCandidates` in model/export/app-core JSON. These candidates give rhwp-shaped tools a stable place to inspect possible paragraph-boundary evidence, while `getValidationWarnings` reports them as diagnostic-only data instead of decoded paragraph records.
- `rjtd text-boundary-candidates <file>` prints the same model-derived candidates for local sweeps. The current 61-sample sweep finds 356 candidate rows across 10 files, including 134 multi-interval candidates, so rjtd still keeps this layer diagnostic-only.
- `rjtd text-boundary-candidate-context <file>` compares those candidates with visible text, line breaks, and source edge alignment. The current sweep shows `0x001c` single-interval edge-good rows are the most promising next filter, while `0x000e` often covers many line breaks and remains too coarse for paragraph promotion.
- `rjtd text-boundary-candidate-agreement <file>` compares byte and unit interpretations for the same candidate range. The current sweep shows exact text agreement is effectively absent, so app-core paragraph promotion should prefer further unit-basis `0x001c` single-candidate validation instead of requiring byte/unit text equality.
- `rjtd text-boundary-candidate-layout-context <file>` compares the stricter unit-basis candidates with direct `/LineMark`, `/PageMark`, and `/PaperMark` contexts. The current sweep finds no selected candidate whose start/end directly hits those layout streams, so rjtd must not treat layout marks as the same coordinate space as candidate source units.
- `rjtd text-boundary-layout-map <file>` scores those stricter candidates against sparse `/LineMark`, `/PageMark`, and `/PaperMark` point sets under several global unit transforms. The current sweep rejects a single global transform: exact hits are file-specific and often target different point families. App-core must therefore keep `textBoundaryCandidates` diagnostic-only until a row-local, section-local, or record-local base offset is proven.
- `rjtd text-boundary-layout-map-rows <file>` scores each stricter candidate independently and prints the linked `TCntV.01` row plus nearest layout points. The current sweep finds dual line-word/page-field row-local `exact=2` evidence for 10 of the 32 strict candidates in `iwata_file`, but not for the 3 strict finance candidates.
- `rjtd text-boundary-paragraph-like <file>` exposes that distinction as a diagnostic-only classifier. The current sweep reports 10 paragraph-like candidates and 25 strict selected but non-paragraph-like candidates, all still `decoded=false`. App-core must therefore keep this as evidence until a paragraph construction rule is proven against layout/style behavior.
- `rjtd text-boundary-paragraph-like-style-context <file>` adds linked `TCntV.01` tail fields and style/view-style hits to the same classifier. The current sweep still reports the same 10 paragraph-like candidates; none has text/page style candidate hits, while all 10 hit `/DocumentViewStyles` group evidence in `iwata_file`. Strict non-paragraph rows also have view-group hits, so app-core must treat this as default/flag-like evidence until proven otherwise and must not attach paragraph styles from it.
- `rjtd text-boundary-paragraph-like-discriminators <file>` summarizes those rows by bucket. The current sweep shows dual layout exactness only on paragraph-like rows, and in `iwata_file` those same rows are the only strict candidates with nonzero chosen `TCntV.01` spans. App-core can surface this as decoded-false evidence later, but must not rebuild real paragraphs until the coordinate target is explained.
- Model/export/app-core JSON now surfaces that stricter evidence as decoded-false `textParagraphBoundaryCandidates`. The preservation rule is strict unit `0x001c` single candidate, nonzero chosen `TCntV.01` span, and dual row-local exact endpoint evidence from both `line-word-value` and `page-be32-field`. A 61-sample JSON export sweep succeeds with 0 failures and preserves 10 candidates total, all in `iwata_file`; `getValidationWarnings` reports them as diagnostic-only data rather than decoded paragraph records.
- `rjtd text-paragraph-boundary-targets <file>` now traces those preserved candidates back to concrete `/LineMark` word indexes and `/PageMark` raw row fields. The first sweep shows some exact endpoints are non-unique, so rjtd must identify semantically eligible line/page fields before promoting these candidates into real paragraph or layout objects.
- Full-layout PDF support should continue to follow rhwp's architecture: decode JTD streams into model objects first, build a page/layer/paint tree from the model, render SVG pages, then assemble PDF through the existing `usvg`/`svg2pdf`/`pdf-writer` path. Tables, images, SVG/vector objects, and shapes should therefore be recovered as model/control/layer objects, not injected directly by the exporter.
- `rjtd object-stream-candidates <file>` now inventories non-text visual/object stream evidence with `decoded=false`: object/image/shape/table path hints, `SO\0\0` markers, image signatures, SVG signatures, and payload prefixes. The current 61-sample sweep finds 43 files with candidates and 17 files with embedded image signatures, but 0 named table-path candidates; table recovery should therefore proceed through `/DocumentText` control ranges and layout/style streams, while image recovery can start from `/EmbedItems/Embedding */Contents` candidates.
- Parser/export/app-core JSON now preserves that same evidence as decoded-false `objectStreamCandidates`; `getDocumentInfo` reports the candidate count and rows, while `getValidationWarnings` reports `JtdObjectStreamCandidateDiagnosticOnly`. A 61-sample JSON export sweep succeeds with 0 failures and preserves 933 candidates in 43 files, keeping the future image/shape/table renderer path model-first.
- `objectStreamCandidates` now also preserves complete detected image payload spans as model-owned bytes, exposed through decoded-false `imagePayloads` metadata in JSON. Each payload also carries optional dimensions and an undecoded `objectEnvelope` with header/trailer slices, conservative declared-length candidates, numeric header-field candidates, and source-path candidates. The current 61-sample JSON sweep, after strict JPEG SOF/SOS validation, finds 12 payload files, 67 complete payloads, 35 dimensioned payloads, 629,024 preserved bytes, 67 envelopes, 20 little-endian declared-length matches, and 66 source-path candidates, all in `Contents` rows. This mirrors rhwp's model-owned binary-data direction while still avoiding any claim that JTD object ownership, placement, or page paint geometry is decoded.
- `objectStreamCandidates` also exposes decoded-false path-derived `ownershipCandidate` entries for stream families such as `embed-items`, `figure-data`, `figure`, `frame`, and `layout-box`. The current 61-sample sweep finds 474 ownership candidates across 933 object candidates, and every preserved image payload row has one. This is the model-side bridge toward rhwp-style object/resource panels, but it remains path evidence until `Embedding N` references are correlated with figure/layout records.
- `objectStreamCandidates` now adds decoded-false `ownershipReferences` for embedded image candidates whose path-derived `Embedding N` appears as bytes inside `FigureData`, `/Figure`, `/Frame`, `/LayoutBox`, `/PageMark`, or `/PaperMark`. The current 61-sample sweep finds 12 files with references, 52 embedded image candidates with references, 604 reference rows, 9,949 total byte matches, and 67 of 67 strict image payload rows covered. This gives rhwp-style resource panels a model-owned cross-stream evidence trail, but rjtd must still prove the authoritative record field and page geometry before turning those rows into image paint ops.
- `rjtd object-ownership-references <file>` now expands that evidence with source/target streams, encoding, reported offset, total match count, mod2/mod4 alignment, local context hex, and le/be 16/32-bit values at the match. The 61-sample sweep reports 3,167 preview-offset rows with 0 failures. This is a diagnostic surface for narrowing object record fields, not a rendering path.
- `rjtd object-ownership-reference-fields <file>` projects those preview offsets onto candidate record strides and summarizes target/encoding/stride/field-offset groups with row-index previews and cross-row counts. The 61-sample sweep reports 33,492 projected field groups with 0 failures; the strongest cross-row-free stride >= 12 candidates currently point at `/Frame` rows. This helps decide where image placement decoding should go next without claiming rhwp-compatible image controls yet.
- `rjtd object-frame-reference-records <file>` expands the strongest `/Frame` projections into candidate row bytes and prints BE/LE 16-bit and 32-bit views. The current sweep expands 261 rows across 12 files and exposes repeated frame row families, but those rows remain decoded-false evidence until geometry units, page association, and paint order are proven.
- `rjtd object-frame-record-families <file>` groups those expanded `/Frame` rows into decoded-false observation buckets. The 61-sample sweep groups the same 261 records with 0 failures: the largest families are `frame-index-tail-coordinate-row12` 65, `frame-index-tail-window20` 65, `frame-index-mixed-row12` 61, and `frame-index-flag-row12` 41. This narrows the next object-placement work while preserving rhwp's model-first rule: exporters still must not read raw `/Frame` rows directly.
- `rjtd object-frame-row-links <file>` verifies that most `u16-be/20/15` `tail-window20` rows are context windows around `u16-be/12/7` rows. In the 61-sample sweep, 65 of 70 row20 windows link to a same-source 12-byte suffix row, and all linked pairs are `frame-index-tail-window20 -> frame-index-tail-coordinate-row12`. This makes `u16-be/12/7` the stronger candidate for model-level frame row decoding, while the 20-byte rows remain diagnostic context.
- Parser/export JSON now preserves the same decoded-false row evidence as `objectStreamCandidates[].frameReferenceRows`, including optional suffix links. The 61-sample JSON sweep preserves 261 rows and 65 suffix links with 0 failures. This moves the next image-placement step closer to rhwp's architecture: page paint construction can consume model-owned frame-row candidates instead of peeking into raw CFB streams from the exporter.
- `rjtd object-image-frame-candidates <file>` now summarizes image payload sources against that model-owned frame evidence, including payload dimensions and the best coordinate/payload aspect delta when both sides are available. The 61-sample sweep has 0 failures, finds 12 files with image payload sources, 52 image sources, 52 frame-linked sources, 0 missing-frame sources, 261 frame rows, 35 dimensioned payloads, and 13 sources with aspect candidates. Diagnostic preferred buckets are `row12-tail-coordinate` 25, `row12-tail-zero` 7, `u16-le-row12` 19, and `none` 1. This shows `row12-tail-coordinate` is still the strongest next placement-analysis target, but only two sources currently have aspect deltas <= 250 permille, so rhwp-style image controls or overlay images should not be claimed yet.
- Parser/export/app-core JSON also preserves decoded-false `objectStreamCandidates[].fdmIndexEntries` for `/FigureData/*/FDMIndex` rows linked to sibling `FDMVector` segments. The model now consumes only valid `fdm-index-v1` declared-count prefix rows instead of projecting the whole stream as 22-byte rows, because many streams carry auxiliary payload bytes after the row table. The 61-sample JSON sweep has 0 failures and preserves 147 rows in 30 candidates across 24 files; 6 rows in 3 files carry the same 13 image hits seen in the raw diagnostic sweep. A new `object-fdm-index-rows` sweep classifies all 43 declared invalid rows as coordinate-like diagnostics concentrated in 3 files, not image-bearing vector segments. This gives future page paint construction model-owned FDM evidence, but exporters must not render FDMVector images until page/frame/layout correlation is proven.
- `rjtd object-fdm-image-candidates <file>` and app-core `getPageOverlayImages` now surface image-bearing FDMVector rows as unplaced diagnostics. The 61-sample sweep has 0 failures and finds 3 files, 6 rows, 13 image hits, 0 strict complete payloads, 5 plausible bbox rows, and 0 renderable rows. The observed `FFD8FF` hits in FDMVector segments are JPEG-like vector-data patterns rather than valid JPEG payloads with SOF/SOS structure. `getPageOverlayImages` keeps `behind`, `front`, and `imageCount` empty/zero, and places the FDM rows under `unplacedDiagnostics` with `placementProven:false`, `renderable:false`, and `decoded:false`.
- Parser/export/app-core JSON also preserves decoded-false `/Frame` fixed 60-byte records as `objectFrameRecords`. Image payload spans now expose optional `dimensions`, using the rhwp-aligned `image` dependency plus a narrow JPEG SOF metadata fallback, and JPEG payload spans are only promoted after SOF/SOS validation. `rjtd object-fdm-frame-links <file>` correlates image-bearing FDMIndex rows with frame records by `fdm row index == frame object id` and reports frame size, payload dimensions, dimensioned payload counts, and best frame/payload aspect delta. The 61-sample sweep has 0 failures and links all 6 FDM image rows to frame records, with 13 image hits, 0 missing-frame rows, 0 strict complete payloads, 0 dimensioned payloads, and 0 renderable rows. This proves a stronger placement evidence trail, but geometry units, page association, paint order, and payload-to-image selection still must be decoded before rendering these images.
- App-core control navigation now projects those preserved boundaries onto fallback paragraph character offsets when neighboring source-spanned text runs make that safe. `getControlTextPositions` keeps rhwp's numeric-array shape, while `findNearestControlBackward` and `findNearestControlForward` can return `type:"jtdControl"` with `decoded:false` diagnostics instead of pretending the boundary is a decoded table, picture, shape, equation, field, or bookmark.
- `getPageControlLayout` also exposes those projected boundaries as per-page `type:"jtdControl"` diagnostics with fallback bounding boxes, `secIdx`, `paraIdx`, `controlIdx`, `charPos`, source, code, and `decoded:false`. This keeps rhwp-style page control panels callable without claiming that rjtd has decoded true Ichitaro controls yet.
- The WASM `HwpDocument` wrapper delegates the same candidate style APIs, with regression coverage for `getStyleList`, `applyStyle`, and `getStyleAt` on a labeled JTD style candidate.
- `getPageLayerTree` now emits a fallback `pageBackground` op, fallback `textRun` ops, and rhwp-shaped top-level `textSources` plus per-op `source` spans. When parsed `/DocumentText` source spans are known, those layer entries also expose JTD byte/unit source ranges. The layer envelope includes schema/resource table versions, output options, empty font resources, feature lists, and fallback `textV2` diagnostics. This is still fallback geometry, not decoded Ichitaro layout.
- `getPageLayerTree` carries fallback `writingMode` and per-text-run `isVertical`/`orientation` fields so vertical-writing render work can stay model-first. The current default remains horizontal because rjtd has not yet decoded the Ichitaro style/layout flag that selects vertical text.
- `getCanvasKitReplayPlan` now mirrors rhwp's mode policy for `default`/`compat` and reports fallback `pageBackground` and `textRun` paint ops as direct replay items with `replayPlane:"background"` and `replayPlane:"flow"`. It still reflects compatibility-projection geometry, not recovered Ichitaro native paint operations.
- `getValidationWarnings` follows rhwp's `count`/`summary`/`warnings` JSON envelope and reports JTD-specific fallback/preservation diagnostics such as fallback text pagination, preserved raw streams, undecoded style streams, preserved unknown objects, diagnostic-only `TCntV.01` ranges, and diagnostic-only `TCntV.01` control-range overlap evidence. `reflowLinesegs` refreshes the fallback page cache but still returns `0` because rjtd has not decoded Ichitaro line segments.
- `rjtd-wasm` exposes the same `HwpDocument` wrapper name as rhwp. The current wrapper provides `pageCount`, `getSectionCount`, `getDocumentInfo`, `getPageInfo`, `renderPageSvg`, `renderPageHtml`, `renderPageToCanvas`, `renderPageToCanvasFiltered`, `renderPageToCanvasLegacy`, `getPageLayerTree`, `getPageLayerTreeWithProfile`, `getPageOverlayImages`, `getCanvasKitReplayPlan`, `setFileName`, `getSourceFormat`, `convertToEditable`, `plainText`, body text editing/search/clipboard APIs, view/navigation APIs, field/header/footer/note fallback APIs, table/cell fallback APIs, object/form/bookmark fallback APIs, explicit export/HTML fallback APIs, and formatting/style/numbering fallback APIs.
- Page/section/page-border setters currently validate the section and return rhwp-shaped `{"ok":true,"pageCount":...}` JSON, but they do not persist decoded JTD page settings yet.
- Header/footer, footnote, and body-footnote-marker hit-test APIs currently return explicit `hit:false` fallback JSON because rjtd has not decoded those JTD structures yet.
- This facade is not yet an Ichitaro layout engine; it is a minimal compatibility surface that edits and renders the current `Document` model's text blocks as fallback pages.
- Formatting APIs do not yet preserve or mutate real JTD style streams; they keep rhwp Studio panels operational while style recovery remains a reverse-engineering task.
- Selection and clipboard APIs currently handle body text only. They do not yet copy/paste tables, shapes, images, fields, cells, footnotes, or preserved unknown inline objects.
- Search and replacement APIs currently handle body text only. They do not search cells, text boxes, headers, footers, footnotes, fields, or unknown preserved object payloads.
- Control navigation APIs currently do not expose real tables, pictures, shapes, fields, footnotes, or text boxes. They only keep rhwp Studio keyboard/navigation paths from failing while JTD object streams remain under analysis.
- Field, header/footer, footnote, and endnote APIs currently do not mutate or reveal decoded JTD structures. They are compatibility fallbacks, not recovered document semantics.
- Table and cell APIs currently do not mutate or reveal decoded JTD structures. They are compatibility fallbacks, not recovered table geometry, nested cell paths, or formula semantics.
- Picture, shape, equation, bookmark, and form APIs currently do not mutate or reveal decoded JTD structures. They are compatibility fallbacks, not recovered object geometry, image bytes, equations, bookmarks, or form values.
- HWP/HWPX export and HTML paste/export are explicit fallbacks. They do not yet produce valid Hancom output or preserve rich HTML semantics.
- `getPageLayerTree` now returns fallback `pageBackground` and `textRun` paint ops; replay APIs report those ops as direct background/flow items so app render paths do not look empty. Real paint operations should be filled in after JTD layout/style/object recovery progresses.
- rhwp native PDF export creates per-page SVGs, then assembles PDF output through `usvg`, `svg2pdf`, and `pdf-writer`.
- rjtd PDF export uses the same crates and direction. It currently produces text-only SVG pages and does not yet recover tables, images, or original page geometry.
- A conditional `rjtd-export` regression test now parses every available local `.jtd`, `.jtt`, and `.jttc` sample and verifies that each exports structurally plausible PDF bytes.
- Documents that preserve raw streams but cannot yet extract body text render a visible diagnostic notice in PDF/SVG output instead of silently producing blank pages.
