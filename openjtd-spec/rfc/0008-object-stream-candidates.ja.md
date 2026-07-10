# RFC 0008: Object and Embedded Image Stream Candidates

## Status

Diagnostic only.

この note の observations は decoded Ichitaro object semantics ではない。future model、layer、renderer work のための `decoded=false` evidence として保存する。

## Motivation

full-layout PDF export には extracted text だけでは足りない。rjtd は images、vector/shape objects、tables、および layout records を model/control/layer objects として recover してから exporter で render する必要がある。

これは rhwp-compatible policy に従う。exporter は raw CFB streams から直接 bytes を注入せず、document model と page/layer tree を consume する。

## Diagnostic Command

`rjtd object-stream-candidates <file>` は readable CFB streams を scan し、visual/object recovery に関係しそうな streams を report する。

最初の implementation は以下の evidence types で candidate streams を classify する。

| Evidence | Meaning |
| --- | --- |
| `object-path` | stream path が `EmbedItems`、`Embedding`、`JSFart`、`CompObj`、`Ole`、`Object`、`Bin` など embedding/object/OLE/binary-object naming を含む |
| `image-path` | stream path が `Image`、`Picture`、`Graphic`、`PNG`、`JPEG`、`BMP`、`WMF`、`EMF` など image-oriented naming を含む |
| `shape-path` | stream path が `Figure`、`Shape`、`Draw`、`Frame`、`LayoutBox`、`SVG` など shape/layout naming を含む |
| `table-path` | stream path が table/cell naming を含む。ただし position/style table names は除外する |
| `so-marker` | payload が preserved `SO\0\0` object/control marker family を含む |
| `image-signature` | payload が PNG、JPEG、GIF、TIFF、start-position BMP、placeable WMF など recognizable binary image signature を含む |
| `svg-signature` | payload が textual `<svg` evidence を含む |

Output rows は stream path、stream size、reason list、first image signature offsets、first SVG offsets、first SO marker offsets、short payload prefix、`decoded=false` を保存する。

## Current Sweep

current 61 local `.jtd`、`.jtt`、`.jttc` samples 全体で command を sweep した。

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

- `ichitaro-20030706232401-success-001-success_data-kazoku_ryoko.jtd` は `/EmbedItems`、`/Figure`、`/FigureData`、`/Frame`、`/LayoutBox` candidates を expose する。`/Figure` と `/PaperMark` にも `SO\0\0` hits が保存される。
- `ichitaro-20030316043238-success-001-success_data-iwata_file.jtd` は `/EmbedItems/Embedding */Contents` streams 内に JPEG signatures を expose し、`jpeg@67` や `jpeg@72` のような offsets が見える。
- `ichitaro-20030422210439-success-002-success_data-natsu.jtd` は 12 image-signature rows を expose し、多くは embedded `Contents` または `EmbeddedPress` streams 内の JPEG signatures である。
- `ichitaro-20030829031540-success-004-success_data-hyo.jtd` は path、SO marker、image signature、SVG signature のいずれでも object-stream candidates を expose しない。visible table content は named table object stream ではなく、`/DocumentText` controls/records と layout/style streams の decoding に依存する可能性が高い。

## Interpretation

Image recovery は stream/path candidates と binary image signatures により testable になった。複数 samples は short object header の後に embedded JPEG evidence を `EmbedItems` streams 内で持つ。

Shape and layout-object recovery は `/Figure`、`/FigureData`、`/Frame`、`/LayoutBox` families から始めるべきである。これは path-level candidates であり、decoded geometry ではない。

Table recovery は current corpus では named CFB table streams に依存すべきではない。`hyo` sample を含めて `table-path` が 0 であるため、table structure は `/DocumentText` controls、style streams、または layout mark records に encode されている可能性が高い。

## Model Preservation

parser はこれらの stream observations を top-level `objectStreamCandidates` として decoded-false model evidence に promote するようになった。

JSON export と app-core `getDocumentInfo` は各 candidate を以下の fields で expose する。

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

各 `imagePayloads` row は `kind`、`mime`、`signatureOffset`、`start`、`end`、`length`、`complete`、`objectEnvelope`、`payloadPrefixHex`、`decoded:false` を記録する。document model は payload bytes も内部に保持するため、future renderer は exporter から raw streams を開き直さず、model 経由で image resources を消費できる。

`objectEnvelope` field は payload 周辺の undecoded bytes を保存する。header start/end/length、header prefix、trailer start/end/length、trailer prefix、そして header 最後の 4 bytes が detected payload length と little-endian `u32` として完全一致する場合のみ conservative `declaredPayloadLength` candidate を記録する。さらに decoded-false `headerFields` として prefix `u16LePrefix`/`u32LePrefix` numeric candidates、および header が path-length byte と NUL-terminated embedded source path を持つ場合の `sourcePathCandidate` を expose する。これは evidence であり decoded Ichitaro object geometry ではない。

candidate-level `ownershipCandidate` も decoded-false である。これは stream path だけから derive され、`stream-path` basis、family、optional storage path、optional `embeddingIndex`、stream role を記録する。例として `EmbedItems` contents/embedded-press streams、`FigureData` `FDMVector`、root figure/frame/layout streams がある。page placement や final object geometry を証明するものではない。

candidate-level `ownershipReferences` field は decoded-false cross-stream evidence である。現在は path-derived `Embedding N` owner を持つ embedded image candidates にだけ attach し、その `N` の byte-pattern matches を `FigureData`、`/Figure`、`/Frame`、`/LayoutBox`、`/PageMark`、`/PaperMark` streams 内で記録する。各 row は `targetPath`、`encoding`、`totalMatches`、bounded `offsets` preview、`decoded:false` を持つ。これは candidate embedding index が object/layout-related streams に観測されることを示す evidence であり、authoritative record field や page geometry をまだ特定しない。

`getValidationWarnings` はこれらを `JtdObjectStreamCandidateDiagnosticOnly` として report する。

同じ 61 local samples の JSON export sweep は 0 failures で、43 files に 933 `objectStreamCandidates` を保存する。image-signature candidates を持つ files は 17、SO-marker candidates を持つ files は 4、table-path files は 0、SVG-signature files は 0 である。これは diagnostic CLI distribution と一致し、evidence を document model 内に保持する。

model-preserved `imagePayloads` field への 2 回目の JSON export sweep も、61 local samples すべてで 0 failures となった。strict JPEG SOF/SOS validation 後、complete payload spans を持つ files は 12、complete payloads は 67、dimensioned payloads は 35、preserved payload bytes は 629,024。内訳は JPEG 35 rows/5 files、GIF89a 31 rows/9 files、GIF87a 1 row/1 file である。

同じ sweep では object envelopes も 67 件保存される。little-endian declared payload length と一致する rows は 20 件で、現時点ではすべて `Embedding */Contents` streams に限られる。large `FDMVector` と `EmbeddedPress` wrapper streams は strict payload validation を通らない限り promoted payload spans として扱わない。

Header field candidate sweep results: 61 local samples はすべて export に成功し、66 payload rows が `sourcePathCandidate` を expose する。source-path candidates は現時点ですべて `Contents` stream 由来である。path extension の内訳は `jpg` 34 rows、`gif` 32 rows。first little-endian prefix word は 59 rows で `9`、6 rows で `4` であり、embedded image contents で観測される dominant `09 00 01 00` と secondary `04 00 01 00` header families に対応する。

App-core image payload diagnostics は同じ envelope と header/source-path candidates を overlay、layer-tree、SVG data attributes に expose する。これらの rows は `diagnosticRenderable:true` のままだが、ownership reference、geometry、paint-order fields が decode されるまでは `ownershipProven:false`、`pageGeometryProven:false`、`paintOrderDecoded:false`、`renderable:false` として扱う。

Ownership candidate sweep results: 61 local samples はすべて export に成功し、933 object stream candidates のうち 474 が path-derived `ownershipCandidate` を expose する。strict promoted image payload rows 67 はすべて ownership candidate に covered される。promoted payload rows の role はすべて `contents` である。candidate families は `embed-items` 335、`figure-data` 38、`figure` 31、`frame` 42、`layout-box` 28 である。

Ownership reference sweep results: 61 local samples はすべて export に成功し、12 files が cross-stream reference candidates を expose する。52 embedded image candidates が `ownershipReferences` を持ち、model は 604 reference rows と 9,949 total byte matches を保存する。reference rows の target family 内訳は `frame` 196、`figure-data` 130、`page-mark` 113、`paper-mark` 73、`layout-box` 65、`figure` 27。encoding 内訳は `u16-be` 183、`u16-le` 171、`u32-be` 141、`u32-le` 109。covered source candidates はすべて `embed-items` content rows で、strict image payload rows 67/67 が cross-stream reference evidence を持つ。

`rjtd object-ownership-references <file>` は、これらの model-owned references を match-context diagnostics として展開する。各 reported preview offset について source stream、target stream、encoding、offset、その reference row の total matches、mod2/mod4 alignment、local context hex、match offset で読んだ le/be 16/32-bit values を出力する。この command は diagnostic-only であり renderable geometry を作らない。

61-sample `object-ownership-references` sweep は 0 failures で、同じ 12 files から 3,167 preview-offset rows を報告する。target-family rows は `figure-data` 1,010、`frame` 897、`layout-box` 528、`page-mark` 498、`paper-mark` 151、`figure` 83。match offset では `u16-be` と `u16-le` rows が embedding index を対応する 16-bit value として直接 expose し、`u32-le` rows は low 16 bits に expose する。これは後続の record-field analysis を絞り込むが、authoritative geometry field をまだ特定しない。

`rjtd object-ownership-reference-fields <file>` は、同じ preview offsets を candidate record strides に投影する。target path、encoding、stride、field offset ごとに match count、source count、embedding index set、row-index preview、cross-row match count を summarize する。この command は各 reported offset を固定 stride set (`4,8,12,16,20,24,28,32,36,40,44,48,52,56,60,64,68,72,80,84`) に対して意図的に試すため、出力は decoded record table ではなく hypothesis surface である。

61-sample `object-ownership-reference-fields` sweep は 0 failures で、同じ 12 files から 33,492 projected field groups を報告する。cross-row-free かつ stride >= 12 の最も強い候補は現在 `frame/u16-le/12/5` が weighted matches 102、`frame/u16-be/12/7` が 89、`frame/u16-be/20/15` が 70 である。これは次の分析が frame records に集中すべきことを示唆するが、どの stride や field が semantically authoritative かはまだ証明しない。

`rjtd object-frame-reference-records <file>` は、最も強い frame projections を candidate row bytes として展開する。現在は decoded-false projection families のうち strongest three (`u16-le/12/5`、`u16-be/12/7`、`u16-be/20/15`) だけを報告し、source stream、embedding index、target stream、encoding、stride、field offset、match offset、row index、row start、row hex、BE/LE 16-bit field views、BE/LE 32-bit field views を出力する。

61-sample `object-frame-reference-records` sweep は 0 failures で、12 files から 261 rows を展開する。内訳は `u16-le/12/5` 102 rows、`u16-be/12/7` 89 rows、`u16-be/20/15` 70 rows。rows は `00010000000N000000020001` style の 12-byte rows や `00000000010200380000000N` rows のような repeated byte families を expose する。これらの families は frame-record analysis にとって有望な evidence だが、まだ decoded placement geometry や paint operations ではない。

`rjtd object-frame-record-families <file>` は、展開済み rows を named decoded-false diagnostic families に group する。これらの名前は specification terms ではなく observation buckets である。たとえば `frame-index-flag-row12` は big-endian word view の trailing fields が小さい flag-like values を持つ 12-byte rows を捕捉し、`frame-index-tail-coordinate-row12` と `frame-index-tail-window20` は `/Frame` reference projections で繰り返し現れる tail-window shapes を捕捉する。

61-sample `object-frame-record-families` sweep は 0 failures で、同じ 12 files の 261 records を group する。Family counts は `frame-index-tail-coordinate-row12` 65、`frame-index-tail-window20` 65、`frame-index-mixed-row12` 61、`frame-index-flag-row12` 41、`frame-index-tail-zero-row12` 22、`frame-index-mixed-window20` 5、`frame-index-tail-mixed-row12` 2。次の promotion step では、どの field も authoritative geometry と扱う前に、これらの row families を page/layout marks と image payload ownership に照合するべきである。

`rjtd object-frame-row-links <file>` は、展開済み 20-byte frame windows が matching 12-byte frame row を suffix として含むかを確認する。これは independent record candidates と smaller record の周辺 context windows を区別するための diagnostic である。

61-sample `object-frame-row-links` sweep は 0 failures である。9 files に 70 row20 windows があり、そのうち 65 は same-source 12-byte suffix row に link し、5 は unlinked のまま残る。Linked row はすべて `frame-index-tail-window20 -> frame-index-tail-coordinate-row12` である。これは `u16-be/20/15` の `tail-window20` projection が通常 independent authoritative record ではなく、`u16-be/12/7` row の context window であることを強く示唆する。unlinked rows はすべて `frame-index-mixed-window20` なので、object/header evidence がさらに得られるまでは別扱いにするべきである。

Parser/export JSON は、この evidence を `objectStreamCandidates[].frameReferenceRows` として保存する。各 row は `targetPath`、`encoding`、`stride`、`fieldOffset`、match `offset`、`rowIndex`、`rowStart`、family、raw `rowHex`、optional `suffixLink`、`decoded:false` を持つ。これにより future image-placement work は model-first のまま維持される。exporters は raw `/Frame` streams を直接 scan するのではなく、model-owned rows を consume するべきである。

61-sample JSON export sweep は 0 failures で、同じ 12 positive files に `frameReferenceRows` を保存する。export された rows は 261、suffix links は 65 であり、family counts は CLI family sweep と完全に一致する。

`rjtd object-image-frame-candidates <file>` は、この evidence を image payload source 側から summarize する。complete image payload spans を持つ各 source について、path-derived embedding index、payload kinds、payload dimensions、dimensioned payload count、total frame rows、row-family counts、row12 coordinate-looking candidates、row20 suffix-link counts、LE row12 counts、diagnostic preferred bucket、coordinate-looking row12 pairs、coordinate/payload aspect delta を報告する。この command も decoded-false であり、preferred bucket や aspect match は調査優先度であって renderable geometry ではない。

61-sample `object-image-frame-candidates` sweep は 0 failures である。image payload source 付き files 12、image sources 52、frame-linked sources 52、`/Frame` rows を持たない sources 0、同じ 261 frame rows、dimensioned payloads 35、coordinate/payload aspect candidates 付き sources 13 を見つける。Diagnostic preferred buckets は `row12-tail-coordinate` 25、`row12-tail-zero` 7、`u16-le-row12` 19、`none` 1 に分かれる。best aspect delta が 250 permille 以下の sources は `natsu.jtd` の 2 件だけなので、`row12-tail-coordinate` は強い placement-analysis candidate だが、PDF image rendering に十分な coverage ではない。

`rjtd object-fdm-index <file>` は `/FigureData/*/FDMIndex` streams を sibling `/FigureData/*/FDMVector` streams と照合する。現在観測した row shape は 20-byte header の後に 22-byte rows が続き、big-endian vector offset、16-bit kind field、4 つの big-endian signed bbox-like fields を持つ。command は各 row を次に大きい vector offset までの vector segment に link し、segment image signatures を decoded-false evidence として報告する。

61-sample `object-fdm-index` sweep は 0 failures である。indexes 付き files 31、index streams 39、parsed rows 417、image signatures 付き rows 6、image hits 13、missing sibling vectors 2 を見つける。これは `FDMIndex`/`FDMVector` が `Embedding N` `/Frame` rows とは別の object-placement evidence path であることを示す。

`rjtd object-fdm-index-shape <file>` は、これらの raw row projections を shape families に分離する。exact 22-byte tables、auxiliary payload bytes が後続する declared-count prefix tables、mixed declared rows、unknown-header streams、missing sibling vectors を区別する。

61-sample `object-fdm-index-shape` sweep は 0 failures である。indexes 39、`fdm-index-v1` headers 35、unknown headers 4、plausible declared counts 34 を見つける。raw whole-stream 22-byte projection は 417 rows と invalid offsets 252 を持つが、declared-count prefix projection は 147 rows、invalid offsets 43、同じ image hits 13 を持つ。Shape counts は `row22-count-prefix` 17、`row22-exact` 14、`row22-mixed-declared` 3、`unknown-header` 3、`missing-vector` 2。これは、以前 invalid と見えた rows の多くが real FDMIndex rows ではなく FDMIndex table 後の auxiliary payload bytes である可能性が高いことを示す。

`rjtd object-fdm-index-rows <file>` は、これらの tables を row-level diagnostic view として出力する。各 row の scope (`declared`, `post-declared`, `raw`)、role (`vector-segment`, `coordinate-like-invalid`, `invalid-vector-offset`)、BE16/i16 field views、raw row bytes、segment image signatures を報告する。role は decoded-false である。`coordinate-like-invalid` は、BE16 fields として見たときに signed-coordinate data に似るという意味であり、semantic record type が decoded されたことを意味しない。

61-sample `object-fdm-index-rows` sweep も 0 failures である。indexes 付き files 31、indexes 39、rows 417、declared rows 147、post-declared rows 253、raw rows 17、valid vector rows 165、invalid rows 252、image hits 13、missing vectors 2 を見つける。Role counts は `vector-segment` 165、`coordinate-like-invalid` 231、`invalid-vector-offset` 21。declared invalid rows 43 はすべて `coordinate-like-invalid` で、3 files に集中し、それらの invalid declared rows は image-bearing vector segments ではない。

Parser/export/app-core JSON は、declared-count prefix rows を対応する `FDMVector` candidate の `objectStreamCandidates[].fdmIndexEntries` として保存する。各 row は `indexPath`、`vectorPath`、row/index/vector offsets、vector segment length、`kind`/`kindHex`、bbox-like fields、`validVectorOffset`、vector prefix、absolute image signatures、segment-relative image signatures、`decoded:false` を記録する。

61-sample JSON export sweep は 0 failures で、24 files の 30 candidates に 147 `fdmIndexEntries` を保存する。image-linked rows を持つ files は 3 で、6 rows が 13 image hits を含む。また valid vector offsets は 104、invalid/out-of-range vector offsets は 43 であるため、この evidence は現在観測される image-bearing FDMVector segments を識別しつつ false auxiliary rows を減らす。ただし、まだ renderable page geometry や paint resources へ promote してはならない。

`rjtd object-fdm-image-candidates <file>` は、model-owned `fdmIndexEntries` のうち image-bearing subset を summarize する。segment image signatures を持つ各 row について、FDMVector source、FDMIndex source、row/vector offsets、`kind`、raw/normalized bbox-like fields、bbox order、bbox plausibility、segment image hits、complete image payload count、`renderable:false` を報告する。complete payload rows は `page-placement-unproven` のまま block される一方、signature-only rows はより狭い canonical blocker `image-signature-without-complete-payload-role-unproven` を使うため、payload extraction failure と page placement を混同しない。

61-sample `object-fdm-image-candidates` sweep は 0 failures である。candidates 付き files 3、FDM sources 3、image-bearing rows 6、image hits 13、strict complete payloads 0、plausible bbox rows 5、renderable rows 0 を見つける。FDMVector segments 内で観測される `FFD8FF` hits は、SOF/SOS structure を持つ valid JPEG payload ではなく vector data 内の JPEG-like byte patterns であるため、signature-only decoded-false evidence のまま残す。

PDF-backed `shanai_lan` diagnostics は、FDMIndex row の bbox-like fields を entry extents と connector-relation gates では entry-local axis pair (`x0,x1,y0,y1`) として normalize する必要があることを示す。この normalize は FDMVector command source bbox には適用しない。command source bbox はすでに left/top/right/bottom order である。axis-pair normalize 後、FDMIndex entry extent は active command extent にかなり近づく (`maxAbsDelta:36` source units) が、page-placement transform の証明にはまだならない。reference PDF crop に対して generated `RAID` glyph component は約 `10.7px` 左、`12.1px` 上に残る。

同じ sample は decoded-false `fdmTextMaskSourceTransformCandidateSummary` も保存する。現在の最有力 bridge は `RAID` の top text-like FDM mask component を `/DocumentText` slot `5` に対応させ、reference-backed current grid offset range `29.285..38.632` と `sourceUnitsPerTextGridUnitX:38.196` を得る。これは transform evidence として有用だが、text grid origin が source-derived になり、同じ line-header run が ambiguous でない row anchor として使え、mask-to-text baseline transform が追加 samples で検証されるまでは non-rendering のまま維持する。

`shanai_lan` line-header diagnostics は decoded-false `gridOriginAuthorityGate` も保存する。選択済み visible line headers は complete LineMark coverage と uniform source-domain fit を持つ: `/LineMark.recordIndex - /DocumentText.groupIndex == 1` であり、PageMark entry row `0` が selected records を cover する (`lineStart:0`, `lineEnd:39`)。これは row-domain relation を証明するが、page-space origin はまだ証明しないため、current text grid は reference-backed かつ non-promotable のままにする。

Raw `documentTextLineRuleProjection` candidates は、rule 単位と graph-component 単位の diagnostic-only render admission gate も持つようになった。各 rule は component membership、LineMark coverage、orthogonal/topology readiness、endpoint text-attachment candidates、明示的 blockers (`document-text-grid-origin-reference-backed`、`line-rule-topology-partial-orthogonal-coverage`、`line-rule-component-topology-unproven`、`line-rule-text-attachment-pair-absent`、`line-rule-endpoint-ownership-unproven`、`line-rule-component-endpoint-ownership-unproven`、`line-rule-component-line-mark-coverage-incomplete`、`line-rule-style-role-unproven`、`line-rule-component-style-role-unproven`、`line-rule-paint-order-unproven`、`line-rule-component-render-admission-not-ready`、`line-rule-text-attachment-pair-unproven`) を記録する。Summary gate は current PDF-backed `shanai_lan` state を `lineRuleRenderAdmissionGate`、`ruleCount:16`、`componentCount:6`、`orthogonalComponentCandidateCount:3`、`bothEndpointAttachmentWithinLineHeightRuleCount:0`、`promotionReady:false`、final blocker `line-rule-render-admission-not-ready` として aggregate する。これにより、source-derived page origin、endpoint ownership、style role、paint order が decode される前に、視覚的に魅力的な単一 line-rule overlay を promote しないようにする。

Selected `shanai_lan` `/DocumentText` line-header full-span candidates も `documentTextLineHeaderProjectionCandidateSummary.fullSpanRenderPromotionGate` の diagnostic-only evidence として保持する。この gate は `fullSpanRenderableCandidateCount:0` を報告し、segment clipping、endpoint ownership、paint order が decode されるまで各 candidate を `line-header-segment-clipping-and-endpoint-ownership-unproven` で block する。

同じ gate は `selectedLineMarkSourceUnitGate` evidence も保存する。selected LineMark records `[22,32]` は source-unit starts `[3908,5483]`、ends `[4121,5615]`、spans `[213,132]`、record delta `[10]`、unit-start delta `[1575]` を持つ。これは source-domain stride evidence として有用だが、stride sample は 1 件だけで source-unit-to-page-y transform もないため、non-promotable のままにする。

同じ gate は `sourceOnlyPageMarkYValueProbe` も含む。PageMark entry row `0` は parsed y-value candidates 58 件を生成し、current-origin probe に最も近いものは `parsedEntryU16` word `7` / byte `14`、value `39` で、current reference-backed origin から `0.300px` しか離れていない。companion `pageMarkEntryProfileGate` は selected entry が additive layout-origin profile ではなく `mixed-payload` であることを記録し、`lineBoundaryConflictGate` は同じ値が entry の `lineEnd=39` に一致することを記録するため、field role が line-boundary semantics から独立して証明されるまでは blocked probe のままにする。

App-core `getPageOverlayImages` は同じ FDM rows を `unplacedDiagnostics` として expose し、`behind`、`front`、`imageCount` は empty/zero のまま維持する。各 diagnostic は `placementProven:false`、`renderable:false`、`decoded:false`、reason `page-placement-unproven` を持つ。これにより rhwp-shaped overlay API は callable だが、decoded page placement や paint resources を claim しない。

Parser/export/app-core JSON は `/Frame` fixed 60-byte records も decoded-false `objectFrameRecords` として保存する。観測された record layout は 16-byte header、offset 14 の big-endian declared count、60-byte rows を持つ。Rows は object id、record kind/type、geometry-looking fields を expose するが、units、page association、paint order が証明されるまでは diagnostic のまま扱う。

Image payload spans は model/export JSON に optional `dimensions` を持つようになった。標準 image dimensions は rhwp-aligned `image` dependency で読み、JPEG については狭い SOF metadata fallback を使う。JPEG payload span は valid SOF/SOS structure を持つ場合だけ promote され、SOI/EOI-like byte pairs だけでは complete payload と扱わない。

Image payload render admission は、より strict な source-backed gate を持つようになった。Model は stream path ownership candidate、payload source-path candidate、cross-stream ownership references がすべて揃う場合だけ `ownershipProven` を true として expose する。さらに `/Frame` reference-row evidence（`frameReferenceRowCount`、`frameCoordinateRowCount`、`frameLinkedWindowRowCount`、`frameGeometryCandidatePresent`）、`EmbeddingInfo -> /Frame` trace（`sourceFrameTrace`、`embeddingFrameTracePresent`、`sourceFrameRecordGeometryPresent`）、traced `/Frame` record から導いた diagnostic-only page-space candidate bbox（`candidateFrameBBox`）、payload-to-frame aspect-fit diagnostics（`payloadFrameAspectFit`、`aspectDeltaPermille`、`bestPayloadAspectDeltaPermille`、`currentPayloadBestFrameAspectCandidate`）を分離して報告する。これらの source facts がすべて存在しても payload は `renderable:false`、`pageGeometryProven:false` のままで、現在最も進んだ blocker は `image-payload-frame-geometry-present-but-page-assignment-and-paint-order-unproven` であり、candidate-frame placement と payload aspect matching も `page-assignment-and-paint-order-unproven` / `payload-selection-page-assignment-and-paint-order-unproven` として diagnostic-only に維持する。これにより、`/Frame` の geometry-like fields や aspect-compatible image payloads を semantics decode 前に page placement や paint-order authority として使わないようにする。

`rjtd object-fdm-frame-links <file>` は image-bearing FDMIndex rows とこれらの `/Frame` records を `fdm row index == frame object id` で相関させ、payload dimensions がある場合は frame size、payload dimensions、dimensioned payload counts、best frame/payload aspect delta も報告する。61-sample sweep は 0 failures で、positive files 3、FDM image rows 6、image hits 13、frame-linked rows 6、missing-frame rows 0、strict complete payloads 0、dimensioned payloads 0、renderable rows 0 を見つける。これは現在観測される FDM image rows が frame-record trail を持つことを示すが、FDMVector signature hit が paintable image payload であることはまだ証明しない。

Model、SVG diagnostics、CLI tests は、complete frame-linked payload evidence と signature-only FDMVector fragments を区別する。frame-linked complete payload fixtures は source-backed blocker `fdm-frame-linked-image-payload-placement-and-paint-order-unproven` で `renderable:false` のまま維持し、real signature-only `shanai_lan` rows は引き続き `image-signature-without-complete-payload-role-unproven` を報告する。どちらの blocker も generated output を source truth に promote せず、page assignment、payload role、paint order が decode されるまで保存された model/test evidence を記述する。

PDF-backed `success_data-test` title art では、JSFart/EmbeddedPress paint candidates を render promotion の前に decoded-false `sourcePaintRenderTrace` evidence として保存する。観測された JSFart paint words は `styleWord1=0x02141030`、`styleWord2=0x02141018`、`paintColorCandidate=0x00ffffff`、`paintFlagCandidate=0x00000001`、`effectWordCandidate=0x0000000a` である。active renderer は conservative front fill `#111111` を選択し続ける。trace conclusion は `source-paint-present-but-render-fill-not-promoted` で、source paint は white だが selected fill と一致せず、source-order interstitial texture path も `front-erase-texture-over-main-face-semantics-unproven` として block されるためである。

同じ title-art projection は、JSFart frame candidate と `/Frame` row を結ぶ decoded-false `sourceFrameRenderTrace` evidence も保存する。最初の title frame では `frameRef` と `/Frame.objectId` がどちらも `1` で、outer width `13260` と outer height `1327` も一致する。content dimensions は `13031x1054` である。これは有用な source coherence だが、active horizontal placement は未解決の frame/content split semantics に依存するため、frame-edge placement は promote せず `frame-content-split-horizontal-semantics-unproven` のまま block する。

local JSFart2Contents sweep では、8 files が `/JSFart2Contents` object-path candidates を持つが、current `MSTUDIO...` magic gate で `jsfartArt` として decode されるのは `success_data-test` の 2 streams のみである。decode された 2 streams は `paintColorCandidate=0x00ffffff`、`paintFlagCandidate=0x00000001`、`effectWordCandidate=0x0000000a` を共有し、`styleWord1/styleWord2` は 2 つの title frames 間で異なる。そのため、これらの words は source evidence のままであり、corpus-wide render authority ではない。

Parser/model/export JSON は、non-`MSTUDIO.OCX` variants を含むすべての `/JSFart2Contents` streams に `jsfartStreamProfile` を expose する。この profile は source-backed かつ decoded-false のみで、prefix family（`mstudio-ocx-utf16le`、`jsfart-object-utf16le`、`zero-prefix` など）、`magicFamilyHex`、UTF-16 preview、header prefix、より厳しい `jsfartArt` candidate の有無を記録する。`renderable` は false のままで、non-MSTUDIO variants は `jsfart-variant-layout-undecoded` として block される。`rjtd object-stream-candidates` CLI command も profile count/family を report し、sweep output 上で non-MSTUDIO source evidence と structured `jsfartArt` candidates を分離できるようにする。

PDF-backed `success_data-test` sample では、Q4/Q5 FDM reference projections の `primitiveOwnershipComparison` に decoded-false `offsetFieldAuthorityGate` を追加した。この gate は render promotion の前に、FDMIndex row の `bbox.left` reference candidates が command-relative offsets と source-segment-relative offsets のどちらに一致するかを比較する。Q4 は 20 references のうち command-relative 18、source-segment-relative 2、Q5 は 7 references のうち command-relative 1、source-segment-relative 6 に分かれる。どちらも `fdm-index-offset-field-authority-mixed-command-and-segment-fields` で block されるため、source data は useful row-command links を示すが、ownership や paint-order promotion に必要な authoritative offset namespace はまだ証明していない。

同じ projections は decoded-false `rowFanoutSegmentOwnerGate` evidence も保存する。Q4 は fanout がなく (`20` references / `20` unique rows、`maxRowFanout:1`)、mixed offset namespaces で block される。Q5 は 7 references に対して unique rows が 3 つで、row `40` が 4 commands、row `41` が 2 commands を持ち、fanout references 6 件はすべて source-segment-relative offset field を使う。これは有用な source evidence だが、1 つの FDMIndex row が複数 vector commands を正当に所有する理由を segment ownership と paint order が説明するまでは render authority ではない。

Role-level `indexRowReferenceRoleCandidateGroups` entries も、同じ decoded-false fanout 問題を `roleFanoutSegmentOwnerGate` として保存する。Q5 では `line-candidate` role が projection-level fanout を具体的な role blocker に絞り込む。command references `1992` と `2024` はどちらも FDMIndex row `40` に map し、fanout references はすべて source-segment-relative offsets を使い、この role は `fdm-index-role-row-fanout-multi-command-single-row` で block される。これにより role rendering は command-span paint-order continuity だけでなく、row fanout と segment ownership の説明にも依存する。

同じ `primitiveOwnershipComparison` path は render promotion の前に decoded-false `primitiveOwnershipAdmissionGate` も保持する。この gate は `ownershipGate`、`offsetFieldAuthorityGate`、`rowFanoutSegmentOwnerGate`、role-level `roleFanoutSegmentOwnerGate`、未解決の paint-order continuity を 1 つの blocker list に集約する。Q4 は mixed raw/segment cohorts、mixed offset namespaces、valid vector-offset role references missing、paint-order continuity で block される。Q5 はさらに projection-level と role-level の row fanout を記録する (`roleFanoutBlockedGroupCount:3`)。row/index `ownershipGate` は generic な primitive-role paint-order blocker を注入せず、paint-order の未解決性は specific な `role-paint-order-continuity-unproven` admission blocker と role-level profiles に保持する。この gate は source-backed diagnostic evidence であり、primitive ownership や paint order は promote しない。

Role-level paint-order profile は span continuity と render authority を分離する。command span に non-role commands が interleave する role は `role-span-interleaved-non-role-commands` で block し、source span が contiguous な role も FDMIndex row order が authoritative paint order であると証明されるまでは `role-paint-order-authority-unproven` として non-rendering のまま保持する。観測中の Q5 solid diagram では 4 role groups が 2 continuity-blocked roles (`arc-candidate`, `connector-candidate`) と 2 authority-pending roles (`line-candidate`, `surface-boundary-candidate`) に分離され、新しい primitive は render しない。

`indexRowOrderPromotionGate` も以前の generic primitive-role blocker ではなく、concrete な blocker list を報告する。Q4 は monotonic one-to-one row-command evidence を持つが、valid FDMIndex vector offsets missing、command-relative/source-segment offset namespace の混在、role paint-order continuity で block される。Q5 は shared valid-vector/namespace/continuity/authority blockers の前に row-order shape failures (`fdm-index-row-order-reference-not-one-to-one`, `fdm-index-row-order-single-row-backs-multiple-commands`) を先頭に出す。

各 role group は fanout の前に decoded-false `roleVectorOffsetAuthorityGate` evidence も持つ。この gate は vector-offset 問題を明示する。現在の role references は FDMIndex offset fields から見つかるが、一致した rows の `FDMIndex.vectorOffset` はまだ invalid であるため、Q4/Q5 の全 role groups は `fdm-index-role-vector-offset-authority-valid-vector-offset-missing` として block される。これにより `bbox.left` 風の offset matches は source evidence として保持しつつ、proven vector-offset ownership と誤認しないようにする。

`shanai_lan` FDM primitives では、background/page-fill promotion の前に `fdmVectorPrimitiveProjection.paintCoverage` と `fdmConnectorGraphDiagnosticSummary.pagePaintCoverageSummary` が non-hardcoded page-paint coverage diagnostics を expose する。Large-span filtered または page-fill-like primitives は、source role、page extent、paint order が decode されるまで `renderable:false` のまま `fdm-page-fill-source-evidence-unproven` で block される。

PDF-backed `success_data-test` ABC table では、`sourceOnlyAxisAdmissionGate` が active source-derived page-space solver と selector-only fallback diagnostics を分離するようになった。table が source-backed line-header units、exact `/LineMark` rows、`lineMarkPageGrid` origin、source y-origin solver からすでに renderable な場合、gate は `activeSourceLayoutAdmissionReady:true` と `admissionReady:true` を報告する。selector-only horizontal evidence absent や single-support y fallback evidence は diagnostic-only として保持され、active source-layout path の blocker にはしない。`tsaiten` table gates は renderable active source layout を持たず、fragmented または single-support y selector evidence に依存しているため、引き続き negative のままである。

同じ ABC table は outer `topTextTableSourceGapEvidence` も nested source-only readiness state から報告する。`sourceTopTextPlacementReadinessGate` が preceding instruction anchor、selected visible width、trailing header semantics、adjacent `/LineMark`/`/PageMark` coupling を証明した場合、outer evidence は `sourceTopTextPlacementReady:true`、blocked reasons なし、render-promotion blocker なしを記録する。Reference bbox residuals は diagnostic evidence のままであり、render authority には使わない。

`totalWidthSemanticsGate` は true width decoder blocker と next-gate handoff を区別するようになった。trailing/header evidence により selected visible range が source-ready だが full line extent がまだ広い場合、gate はまず `source-table-placement-coherence-gate` handoff を記録する。同じ source-only top-text/table placement coherence が存在する場合、その handoff は `sourcePlacementCoherenceGateEvidencePresent:true`、`sourcePlacementCoherenceGateResolved:true`、`renderPromotionNextGate:null`、render-promotion blocker なしとして閉じられる。trailing/header evidence を持たない samples は引き続き `source-total-width-semantics-unproven` を報告する。

PDF-backed `tsaiten` tables では、line-domain plus post-row-gap projection probe が nested `sourceOnlyProjectionDomainGate` を持つようになった。parent probe は diagnostic として reference residuals を記録できるが、nested gate は明示的に `referenceBacked:false` とし、source-only blockers を分離する。具体的には line-domain y と PageMark subrecord-gap units を cross-domain に足していること、selected records が post-row-gap records であること、不完全または ordered-unique でない span coverage、そして未解決の page-y transform を保持する。これにより scoring table の `235.087 + 65 -> 300.087` のような near-reference residual を render authority として扱わない。

同じ `tsaiten` y path では、`sourceOnlyPageMarkAbsoluteYSlotGate` も unconditional semantic blocker ではなく source agreement から計算するようになった。gate は line-domain plus post-row-gap projection に対して最良の PageMark absolute-y slot を選び、それらの source candidates が一致する場合だけ `page-mark-absolute-y-slot-semantics-unproven` を解除する。current candidates は lower table の projection `875.539` と absolute slot `768.000` を含めて一致しないため、blocker はより具体的な `line-domain-projection-disagrees-with-page-mark-absolute-y-slot` になる。

この disagreement は `sourceOnlyPageYOriginSelector` の support path にも mirror されるようになった。lower `tsaiten` table は引き続き `selectedY:768.000` を single-support source diagnostic として expose できるが、selector support と agreement group は `page-mark-absolute-y-slot-semantics-unproven` と `line-domain-projection-disagrees-with-page-mark-absolute-y-slot` の両方を持つ。したがって将来の source-only promotion は、line-domain projection も一致しない限り raw `768` slot を placement authority として扱えない。

source-gap-to-page-line transform readiness も `sourceOnlyPageYOriginDomainGate` と `sourceOnlyPageYRenderAdmissionGate` の両方に `sourceGapToPageLineGapTransformAdmissionGate` として mirror される。この source-only gate は `transformDomain:"source-unit-gap-to-page-mark-line-index-gap"`、`canDecodeSourceTransform:false`、`tableFamilyTransformStable:false`、best candidate `segment-offset-gap`、max delta `105`、blockers `source-gap-to-page-line-gap-transform-not-stable`、`source-gap-to-page-line-gap-transform-unstable-across-table-family`、`source-gap-to-page-line-gap-transform-undecoded` を報告する。したがって render admission は reference table bboxes ではなく decoded transition evidence に接続されたままになる。

combined `sourceOnlyAxisAdmissionGate` も同じ y-selector support blockers、PageMark absolute-slot agreement fields、source-gap transform admission gate を mirror する。これにより x/y coupling diagnostic は下位の y path まで source-only のままになる。scoring table は fragmented cross-table y evidence を持ち、lower table は raw `768.000` slot を `line-domain-projection-disagrees-with-page-mark-absolute-y-slot`、residual `107.539`、未 decode の source-gap transform blockers と一緒にだけ保持する。したがって axis coupling は y path が decode されるまで、単に selected されただけでは render されない。

`sourceGapToPageLineGapReadinessHints` と mirror された各 `sourceGapToPageLineGapTransformAdmissionGate` は candidate-level transform taxonomy を保存するようになった。candidate count、exact-match count、best-candidate transition coverage、best-candidate spread、lowest-spread candidate、full candidate summaries、declined transform candidates と decline reasons を報告する。current `tsaiten` evidence は best max-delta candidate が `segment-offset-gap` (`105`) で、lowest-spread candidate が `direct-source-range-gap` (`12.250`) であるため、transition rule は source-only page-y promotion に使えるほど stable ではなく、意図的に blocked のままにする。

同じ gates は、全 transitions が 1 つの PageMark/table family 内に留まる場合に diagnostic-only `affineRowSourceStartGapFit` も expose する。この fit は exact rational source evidence である。`tsaiten` は `numeratorSlope:143`、`denominatorSlope:6`、`numeratorIntercept:671`、`denominatorIntercept:6`、`maxAbsResidual:1.000`、`sampleCount:3`、`familyScoped:true`、`fitStable:true` を emit する。`success_data-test` ABC-table evidence は residual `1/3` 以内で整合するが、`hyo` は `tsaiten` formula と `51.667` ずれて矛盾し、probe corpus は別 family (`1103/17*y + 276/17`) に fit する。affine candidate は常に `selected:false` で、`affine-row-source-start-gap-family-transform-authority-unproven` により declined される。diagnostic candidate count は増えるが、`canDecodeSourceTransform:false`、`admissionReady:false`、render blockers は変更しない。worker-10 の PageMark slot sweep は broad raw-slot semantics に対する別の negative result として残る。lower `tsaiten` の raw slot `768.000` は reference top `768.014` と一致するが、scoring table は raw slot `1024.000` と reference top `301.005` が矛盾し、ABC table には同じ slot が存在しない。

`document-info` は `/PaperMark` vertical-writing evidence を bit-level かつ decoded-false のまま保持する。flag word `0x00010011` の bit 0 は、current corpus では parsed vertical-rl Ginga samples (`46.jtd`, `a5.jtd`, `b6.jtd`) のみに出るため、`paperMarkFlagBit0VerticalCandidate:true` と evidence `paper-mark-flag-bit0-vertical-corpus-consistent` として報告する。bit 17 は `paperMarkFlagBit17IndexStepCandidate:true` として別扱いにする。portrait `dousoukai.jtd` が set し、landscape `shanai_lan` が set しないため landscape bit ではなく、bit-16-only step `1` に対する PaperMark index step `2` を追跡する。cross-version semantics が証明されるまで、既存の `writingModeCandidateDecoded:false` と blocker `paper-mark-writing-mode-flag-semantics-unproven` は維持する。

`sourceOnlyAxisAdmissionGate` は diagnostic-only の `sourceOnlyAxisCandidateBBox` も emit する。これは best source-only horizontal candidate と selected source-only y candidate を reference bbox selection なしで結合する。current `tsaiten` sample では scoring table candidate が `174,235.087,421,93.192`、lower table candidate が `174,768,554,63` である。どちらも source-backed comparison target にすぎず、`selectionReady:false`、`referenceBacked:false` のまま `source-page-space-axis-selector-coupling-unproven` で block される。

`sourcePageYTransformGate` は、reference table calibration を将来外すための source-only admission contract として `sourceOnlyPageYRenderAdmissionGate` も持つようになった。この gate は direct `lineMarkPageGrid` origin、exact `/LineMark` rows、decoded y solver が揃う既存の source-rendered ABC table では `admissionReady:true` を報告する。同じ gate は current `tsaiten` table candidates を reference coordinates なしで non-rendering に保つ。scoring table は direct origin missing、page-space origin ではない cross-table line-domain evidence、fragmented selector support、source/subrecord ordering contradiction、PageMark absolute-slot disagreement で block される。lower table は stride-only origin、non-exact line rows、single-support fallback、non-unique selected post-row-gap coverage、同じ PageMark disagreement、`sparse-sibling-derived-candidate-render-ineligible` で block される。

local PAGE01 right/down probe family も diagnostic-only admission evidence として保持する。right-tick files は `/DocumentText` line-header の X movement が線形 (`firstCellOffsetDelta=2/4/6/8`) であることを示すが、source-only horizontal selector と x/y coupling はまだ render-admissible ではないため `sourceOnlyAxisAdmissionGate.admissionReady` は false のままになる。down files は `/LineMark` stride correlation を示すが、一太郎 editor の movement は table だけでなく following text も一緒に押し下げるため、独立した table-only page-Y origin の proof にはならない。したがって layer tree は `sourceOnlyPageYAdmissionClass:"flow-y-stride-only-diagnostic"`、`pageOriginAuthority:"fallbackTextAnchors"`、`sourceOnlyPageYRenderAdmissionGate.admissionReady:false` を保持し、これらの samples は blocker semantics を固定するだけで render promotion には使わない。

Table layer diagnostics は `referenceFallbackAdmissionGate` も expose するようになった。これは既存の visible reference-fallback boolean を source-only page-y admission contract に接続する behavior-preserving audit surface である。この gate は `success_data-test` ABC tables が renderable source-derived layout を使いながら reference fallback を suppress していることを報告する (`referenceFallbackAllowed:false`、`referenceFallbackUsed:false`、`sourceOnlyPageYAdmissionReady:true`、`blockedReason:"active-source-layout-admission-suppresses-reference-fallback"`)。current PDF-backed `tsaiten` visible tables はまだ legacy reference fallback を使うが、`source-derived-layout-not-renderable` や `source-page-y-render-admission-not-ready` のような source replacement blockers を明示するため、rendered output を変えずに残りの calibration-removal work を inspect できる。

`tsaiten_table_grid_overlay_layout` は generic `table_grid_overlay_layout` fallback path から外した。explicit reference helpers と reference-backed diagnostics は legacy calibration を引き続き expose するため visible `tsaiten` output は変わらない。一方 generic table bbox fallback は source-derived layout または generic page/body-anchor heuristic を使い、`tsaiten` constants を silent に借りない。これにより今後の removal surface は `reference_table_grid_overlay_layout` と reference-only probes に絞られる。

`pageMarkScopedYTransformProbe` は probe top level で reference-target status を明示するようになった。source-backed PageMark/LineMark record matches の横に `referenceBBoxUsed:true`、`referenceTargetBasis:"referenceTableBBox.rowTopTargets"`、`sourceOnlyReplacementBlockedReason:"page-mark-scoped-y-transform-targets-reference-backed"` を emit する。これにより reference row-top residual probe は calibration diagnostics として保持しつつ、source-only render authority と誤読されない。source-only promotion は `sourceOnlyPageYOriginSelector`、`sourceOnlyPageMarkAbsoluteYSlotGate`、`lineHeaderLineMarkCouplingEvidence` から来なければならない。

## Next Work

- image payload signatures 前の semantic object header fields を decode し、`/Figure`、`/Frame`、`/LayoutBox`、layout mark evidence と接続する。
- `/Frame` geometry units、page association、paint order、payload-to-image selection、remaining coordinate-like FDMIndex diagnostic rows を decode してから FDMVector images を render する。
- mixed command-relative/source-segment references に対する FDMIndex offset-field authority を証明してから、Q4/Q5 FDM primitive ownership または paint order を promote する。
- `roleVectorOffsetAuthorityGate` を解除する前に、`FDMIndex.vectorOffset`、`bbox.left`、または別の row-local field のどれが authoritative role ownership reference なのかを証明する。
- Q4/Q5 primitive を rendering に admit する前に、`primitiveOwnershipAdmissionGate` の ownership、offset namespace、row fanout、role fanout、valid vector-offset references、paint-order continuity blocker を解決する。
- ownership references を page geometry に promote する前に、どの `Embedding N` reference encoding と record-local offset が semantically authoritative かを証明する。
- object ownership と page geometry が証明された後にのみ、preserved image payload bytes を model-level image resources に接続する。
- non-text PDF rendering の前に decoded object/layout records から real page/layer paint operations を構築する。
- table semantics は stream-name matching ではなく、`/DocumentText` control ranges と layout/style streams から調査する。
