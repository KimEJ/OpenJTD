# rhwp Compatibility Reference

OpenJTD は現在の Rust implementation として `rjtd` を使う。`rjtd` は独自 architecture
を新しく設計するより、rhwp の構造と開発方式をできるだけ参考にする。

そのため、最上位ワークスペースに `rhwp/` をローカル参照リポジトリとして置く。

## Source

```text
https://github.com/edwardkim/rhwp.git
```

現在のローカルクローン基準:

```text
branch: main
commit: bc38ff55
```

## Clone Note

rhwp リポジトリには Git LFS files が含まれる。現在 GitHub LFS budget の制限により大容量 PDF smudge が失敗する可能性があるため、source structure 参照目的の clone では LFS download を skip する。

```sh
GIT_LFS_SKIP_SMUDGE=1 git clone https://github.com/edwardkim/rhwp.git rhwp
```

## Policy

- `rhwp/` は read-only reference repository として使う。
- rjtd implementation は `rhwp/` code を copy しない。
- structure、layer separation、test method、API philosophy をまず比較する。
- rjtd の実際の implementation code は `rjtd/` 以下に置く。
- `rhwp/` は外部 reference repository として扱う。

## Dependency And Implementation Policy

rjtd は dependency selection と direct implementation scope の両方で rhwp 方式に従う。

- 新しい dependency や implementation method が必要な場合、まず `rhwp/Cargo.toml` と rhwp implementation を確認する。
- rhwp が同じ task category ですでに使う crate があれば、rjtd もその crate と approach を優先する。
- rhwp が同じ task category を直接実装している場合、rjtd も新 dependency を追加せず direct implementation を優先する。
- rhwp に dependency も direct implementation precedent もない完全に別の問題の場合だけ、実装を止めて明示的な判断を求める。
- 便利さだけを理由に rhwp にない新 dependency を追加しない。

現在 rhwp で確認された関連 dependency patterns:

- CFB/OLE container: `cfb`
- Deflate compression/decompression: `flate2`
- binary endian helpers: `byteorder`
- ZIP/HWPX-style archives: `zip`
- legacy encodings/code pages: `encoding_rs`, `codepage`
- JSON model/debug output: `serde`, `serde_json`
- native SVG-to-PDF export: `usvg`, `svg2pdf`, `pdf-writer`

JTTC に関する現在の結論:

- local `.jttc` sample の `/JSCompDocument` には `JustCompressedDocument` と `-lh5-` marker が見える。
- rhwp には LHA/LZH/LH5 系 dependency はない。
- したがって rjtd は別途決定なしに LHA/LZH dependency を追加しない。
- rjtd は観察済みの single `-lh5-` member profile だけを直接実装し、`.jttc` inner CFB の `/DocumentText` を読む。
- 現在の実装は LHA header checksum と CRC verification、multi-member archive、他の LHA method をまだ support しない。

Malformed CFB に関する現在の結論:

- 追加 local `.jtd` samples の一部は標準 `cfb` reader で `Malformed FAT` error により失敗する。
- rhwp は `rhwp/src/parser/cfb_reader.rs` に `LenientCfbReader` を直接実装し、標準 reader failure 時の fallback として使う。
- したがって rjtd もこの問題では新 dependency を探さず、`rjtd-core::container` に lenient CFB fallback を直接実装した。
- fallback は標準 `cfb` parsing が失敗した後だけ使い、local sample 61 件すべてが `rjtd info` で開けることを確認した。
- `/DocumentText` がない 2 samples は raw `SsmgV.01`/`TextV.01` fragment を持つ `cfb-embedded-document-text` として分類する。
- `rjtd cat` は local sample 61 件すべてで動作する。

DocumentText に関する現在の結論:

- rhwp は parser stage で raw record を作り、model stage がその結果を consume する。
- rjtd も `rjtd-core` に `ParsedDocumentText` token layer を置き、`rjtd-model` がそれを consume するように合わせた。
- この段階は UTF-16BE code unit walk と観察済み control boundary handling だけを必要とするため、新 dependency なしに直接実装した。
- plain text で skip する inline segment も `SkippedInlineText` token と `UnknownObject` payload として保存する。
- observed ruby pair は visible base inline (`selector 0x0003`) と phonetic annotation (`selector 0x0082`) を `Inline::Ruby` として model に lift する。plain text、Markdown、PDF は base text を表示し、JSON は annotation text と raw payload を保存する。
- rhwp は style records を model objects に parse しつつ original `raw_data` も保持する。rjtd は record semantics decode 前の preservation half として、observed style/layout streams を named `UnknownStyle` model entries に lift する。
- rjtd は neutral style stream record boundary candidates も expose する。observed `0x5555`/`0x4444` markers を持つ Ssmg slot records、conservative UTF-16BE label candidates、stream が対応する場合の sequential `u16 code + u16 payload_len + payload` records を報告する。
- `/DocumentTextPositionTables` も separate diagnostic parser と `text-positions` command に分離し、まだ検証されていない offset meaning を model generation に混ぜない。
- true `DocumentText` record boundary と full style/ruby/layout meaning はまだ未解析である。

Application core / PDF に関する現在の結論:

- rhwp app surface は `HwpDocument` wrapper が `DocumentCore` を包み、open、page count、document info、SVG rendering をまず提供する。
- rjtd は `rjtd-model::DocumentCore` に `from_bytes`、`page_count`、`get_document_info`、`get_page_info`、`render_page_svg`、`render_page_html`、`get_page_layer_tree`、`get_page_overlay_images`、`get_canvaskit_replay_plan`、`set_file_name`、`get_source_format`、`convert_to_editable` を先に実装した。
- rjtd は model-owned fallback `WritingMode` path も持つ。`DocumentCore`、SVG page rendering、`getPageLayerTree` は `horizontal` または `vertical-rl` を project できるが、real Ichitaro writing direction を style/layout streams から証明するまでは `writingModeDecoded:false` のままである。
- `rjtd-wasm` は rhwp と同じ `HwpDocument` wrapper 名を expose する。current wrapper は `pageCount`、`getDocumentInfo`、`getPageInfo`、`renderPageSvg`、`renderPageHtml`、`getPageLayerTree`、`getPageLayerTreeWithProfile`、`getPageOverlayImages`、`getCanvasKitReplayPlan`、`setFileName`、`getSourceFormat`、`convertToEditable`、`plainText` を提供する。
- Style APIs は preserved JTD style stream sources を honest に expose する。`getDocumentInfo`、`getStyleList`、`getStyleDetail` は source stream counts/names/sizes、observed stream family (`ssmg`、`table`、`unknown`)、big-endian header fields、record candidates、`decoded:false` を報告する。
- `getDocumentInfo` は `styleCandidateCount` と `styleCandidateNames` も含み、app が full style list を load する前に observed JTD style candidates を discover できるようにする。
- `getStyleList` と `getStyleDetail` は labeled `/TextLayoutStyle` records も rhwp-shaped JTD style candidates として expose する。これらの entries は real style IDs が paragraphs と text runs に attach されるまで、`decoded:false`、source stream/offset/code metadata、fallback char/paragraph properties を保持する。
- `applyStyle` はこれらの candidate IDs を in-memory `Paragraph` style reference に保存し、`getStyleAt` は applied candidate を報告する。これは rhwp の edit surface direction に合わせたもので、JTD stream mutation support を主張するものではない。
- CLI diagnostics も同じ observation layer を使う。`rjtd style-records <file>` は preserved style stream summaries と record offsets/codes を出力し、`rjtd style-candidates <file>` は cross-sample correlation 用に labeled `/TextLayoutStyle` candidates を列挙する。
- `rjtd text-layout-style-records <file>` は unlabeled records、payload lengths、payload digests、short payload previews、BE16 fields、labeled records に割り当てた candidate IDs を含む all `/TextLayoutStyle` records を出力する。
- `rjtd document-view-style-groups <file>` は `/DocumentViewStyles` group records の payload lengths、payload digests、short payload previews を出力し、group IDs を app-core model に attach する前に比較できるようにする。
- `rjtd text-position-style-context <file>` は `TCntV.01` tail fields と text/page style candidate IDs・source record indexes を相関させる。この command は hits を evidence として報告するだけで、rjtd はまだこれらの fields から parse-time style references を attach しない。
- `rjtd text-position-style-summary <file>` はこれらの hits を tail field ごとに aggregate し、`0x3104..0x3907` のような `/DocumentViewStyles` group records とも比較する。current local sweep では finance-like samples で `f1` group hits が出る一方、`TCntV.01` entries を持つ non-finance samples は `/TextLayoutStyle` records が 0 で、1..9 の view-style group range を大きく超える `f1` values を持つ。`f7` は near-constant のままで、`f4` は one group-hit sample にだけ現れる。そのため、record semantics が証明されるまで app-core model はこれらの fields を diagnostics に留める。
- `rjtd text-position-count-tail-field-roles <file>` は selected deltas で tail fields と adjacent field pairs を document-text unit/text hits と比較する。local sweep は diagnostic-only decision を強める。`f1/f2` はしばしば range-like pair として score し、`f7` は near-constant で、多くの場合 direct text hit を持たない。
- parsed `TextRun` model data は `/DocumentText` byte/UTF-16 source span を持つようになった。これは rhwp が UTF-16-positioned text metadata を重視する方向に合わせたもの。valid `TCntV.01` text-count entries は decoded-false `textCountRanges` として保存され、JSON export と `getDocumentInfo` は observed family、chosen start/end/span、declared start/end、tail fields、raw bytes、model text runs に対する byte/unit `documentTextOverlaps` を expose する。ただし paragraph styles や true layout geometry にはまだ attach しない。
- `/DocumentText` control boundary codes も decoded-false `textControlBoundaries` として保存し、可能な場合は byte/unit source spans を attach する。rjtd はこれらの controls をまだ paragraph、line、style、object semantics へ promote せず、次の boundary/layout inference pass の evidence として保持する。
- `controlRangeOverlaps` も decoded-false `textBoundaryCandidates` として model/export/app-core JSON に lift する。これにより rhwp-shaped tools は possible paragraph-boundary evidence を安定した場所で inspect でき、`getValidationWarnings` は decoded paragraph records ではなく diagnostic-only data として報告する。
- multi-interval `textBoundaryCandidates` も decoded-false `tableCandidates` として model/export/app-core JSON に保存し、interval ごとの source range、text preview、line-break count も保持する。`getValidationWarnings` は `JtdTableCandidateDiagnosticOnly` を報告するが、row/cell boundaries と geometry が証明されるまで table/cell APIs は compatibility fallback のままである。
- `rjtd table-candidates <file>` は同じ model-derived table candidates を local sweep 用に出力する。current 61-sample sweep では 10 files に 134 candidates があり、byte-basis rows 65、unit-basis rows 69、max 44 intervals、interval line breaks を持つ candidates 0 である。diagnostic surface は安定したが、rows、columns、cells、table geometry はまだ証明されていない。
- `rjtd text-boundary-candidates <file>` は同じ model-derived candidates を local sweep 用に出力する。current 61-sample sweep では 10 files に 356 candidate rows があり、そのうち 134 rows が multi-interval candidates なので、rjtd はこの layer をまだ diagnostic-only に保つ。
- `rjtd text-boundary-candidate-context <file>` はこれらの candidates を visible text、line breaks、source edge alignment と比較する。current sweep では `0x001c` single-interval edge-good rows が次の filter として最も有望であり、`0x000e` は many line breaks を覆うことが多く paragraph promotion にはまだ粗すぎる。
- `rjtd text-boundary-candidate-agreement <file>` は同じ candidate range の byte interpretation と unit interpretation を比較する。current sweep では exact text agreement が実質的に存在しないため、app-core paragraph promotion は byte/unit text equality を要求するのではなく、unit-basis `0x001c` single-candidate validation をさらに進めるべきである。
- `rjtd text-boundary-candidate-layout-context <file>` はより strict な unit-basis candidates を direct `/LineMark`、`/PageMark`、`/PaperMark` contexts と比較する。current sweep では selected candidate の start/end がこれらの layout streams に direct hit するものは 0 であり、rjtd は layout marks を candidate source units と同じ coordinate space として扱ってはならない。
- app-core control navigation は、source-spanned text runs との隣接関係が安全に確認できる boundary を fallback paragraph character offsets に project する。`getControlTextPositions` は rhwp と同じ numeric-array shape を保ち、`findNearestControlBackward` / `findNearestControlForward` は decoded table/picture/shape/equation/field/bookmark のふりをせず、`type:"jtdControl"` と `decoded:false` diagnostics を返すことがある。
- `getPageControlLayout` も projected boundaries を page ごとの `type:"jtdControl"` diagnostics として expose する。fallback bounding boxes、`secIdx`、`paraIdx`、`controlIdx`、`charPos`、source、code、`decoded:false` を含めるが、true Ichitaro controls を decode 済みという意味ではない。
- `rjtd text-boundary-layout-map <file>` は stricter candidates を sparse `/LineMark`、`/PageMark`、`/PaperMark` point sets と複数の global unit transforms で score する。current sweep は single global transform を reject する。exact hits は file-specific で、しばしば別々の point families を target にするため、row-local、section-local、または record-local base offset が証明されるまで app-core は `textBoundaryCandidates` を diagnostic-only に保つ。
- `rjtd text-boundary-layout-map-rows <file>` は stricter candidates を個別に score し、linked `TCntV.01` row と nearest layout points を出力する。current sweep では `iwata_file` の strict candidates 32 のうち 10 件に dual line-word/page-field row-local `exact=2` evidence があるが、strict finance candidates 3 にはない。
- `rjtd text-boundary-paragraph-like <file>` はこの差分を diagnostic-only classifier として expose する。current sweep は paragraph-like candidates 10、strict selected but non-paragraph-like candidates 25 を報告するが、すべて `decoded=false` のままである。app-core は layout/style behavior に対して paragraph construction rule が証明されるまで、これを evidence として保持する。
- `rjtd text-boundary-paragraph-like-style-context <file>` は同じ classifier に linked `TCntV.01` tail fields と style/view-style hits を追加する。current sweep でも paragraph-like candidates は同じ 10 件で、text/page style candidate hits は 0 件だが、10 件すべてが `iwata_file` の `/DocumentViewStyles` group evidence を持つ。strict non-paragraph rows も view-group hits を持つため、反証されるまでは default/flag-like evidence として扱い、app-core は paragraph styles を attach してはならない。
- `rjtd text-boundary-paragraph-like-discriminators <file>` はこれらの rows を bucket ごとに要約する。current sweep では dual layout exactness は paragraph-like rows だけに現れ、`iwata_file` では同じ rows だけが nonzero chosen `TCntV.01` spans を持つ strict candidates である。app-core は将来これを decoded-false evidence として expose できるが、coordinate target が説明されるまで real paragraphs を rebuild してはならない。
- model/export/app-core JSON は、この stricter evidence を decoded-false `textParagraphBoundaryCandidates` として expose するようになった。保存 rule は strict unit `0x001c` single candidate、nonzero chosen `TCntV.01` span、`line-word-value` と `page-be32-field` の両方に row-local exact endpoint evidence があること。61-sample JSON export sweep は 0 failures で、合計 10 candidates が保存され、すべて `iwata_file` 由来である。`getValidationWarnings` は decoded paragraph records ではなく diagnostic-only data として報告する。
- `rjtd text-paragraph-boundary-targets <file>` は preserved candidates を concrete `/LineMark` word indexes と `/PageMark` raw row fields へ trace する。初回 sweep では一部の exact endpoints が non-unique であるため、rjtd はこれらを real paragraph/layout objects へ promote する前に semantic target となる line/page fields を特定しなければならない。
- full-layout PDF support は rhwp と同じ architecture を続ける。JTD streams をまず model objects に decode し、model から page/layer/paint tree を作り、SVG pages を render して既存の `usvg`/`svg2pdf`/`pdf-writer` 経路で PDF を assemble する。tables、images、SVG/vector objects、shapes は exporter が raw stream を直接読むのではなく model/control/layer objects として recover する必要がある。
- `rjtd object-stream-candidates <file>` は non-text visual/object stream evidence を `decoded=false` で inventory する。object/image/shape/table path hints、`SO\0\0` markers、image signatures、SVG signatures、payload prefixes を出す。current 61-sample sweep では candidates を持つ files 43、embedded image signature files 17 を見つけたが、named table-path candidate は 0 である。したがって table recovery は `/DocumentText` control ranges と layout/style streams から進め、image recovery は `/EmbedItems/Embedding */Contents` candidates から始める。
- parser/export/app-core JSON は同じ evidence を decoded-false `objectStreamCandidates` として保存する。`getDocumentInfo` は candidate count と rows を report し、`getValidationWarnings` は `JtdObjectStreamCandidateDiagnosticOnly` を report する。61-sample JSON export sweep は 0 failures で、43 files に 933 candidates を保存するため、future image/shape/table renderer path は model-first のまま維持される。
- `objectStreamCandidates` は complete detected image payload spans も model-owned bytes として保存し、JSON では decoded-false `imagePayloads` metadata として expose する。各 payload は optional dimensions と、header/trailer slices、conservative declared-length candidates、numeric header-field candidates、source-path candidates を持つ undecoded `objectEnvelope` も含む。strict JPEG SOF/SOS validation 後の current 61-sample JSON sweep は payload files 12、complete payloads 67、dimensioned payloads 35、preserved bytes 629,024、envelopes 67、little-endian declared-length matches 20、source-path candidates 66 を見つける。source-path candidates はすべて `Contents` rows 由来である。これは rhwp の model-owned binary-data direction に合わせるものだが、JTD object ownership、placement、page paint geometry を decode 済みとは主張しない。
- `objectStreamCandidates` は `embed-items`、`figure-data`、`figure`、`frame`、`layout-box` などの stream families に対して decoded-false path-derived `ownershipCandidate` entries も expose する。current 61-sample sweep では 933 object candidates 中 474 ownership candidates が見つかり、strict promoted image payload rows 67 はすべて ownership candidate を持つ。これは rhwp-style object/resource panels へ向かう model-side bridge だが、`Embedding N` references が figure/layout records と correlate されるまでは path evidence に留まる。
- `objectStreamCandidates` は path-derived `Embedding N` が `FigureData`、`/Figure`、`/Frame`、`/LayoutBox`、`/PageMark`、`/PaperMark` 内に byte pattern として現れる embedded image candidates に対して decoded-false `ownershipReferences` も追加する。current 61-sample sweep では references 付き files 12、references 付き embedded image candidates 52、reference rows 604、total byte matches 9,949、strict image payload rows 67/67 が covered される。これは rhwp-style resource panels へ model-owned cross-stream evidence trail を提供するが、rjtd はこれらを image paint ops に変換する前に authoritative record field と page geometry をまだ証明しなければならない。
- `rjtd object-ownership-references <file>` は、この evidence を source/target streams、encoding、reported offset、total match count、mod2/mod4 alignment、local context hex、match offset の le/be 16/32-bit values として展開する。61-sample sweep は 0 failures で 3,167 preview-offset rows を報告する。これは object record fields を絞り込むための diagnostic surface であり、rendering path ではない。
- `rjtd object-ownership-reference-fields <file>` は、これらの preview offsets を candidate record strides に投影し、target/encoding/stride/field-offset groups を row-index previews と cross-row counts 付きで summarize する。61-sample sweep は 0 failures で 33,492 projected field groups を報告し、cross-row-free かつ stride >= 12 の最も強い候補は現在 `/Frame` rows を指す。これは image placement decoding の次の場所を決めるための材料であり、rhwp-compatible image controls をまだ主張しない。
- `rjtd object-frame-reference-records <file>` は、最も強い `/Frame` projections を candidate row bytes として展開し、BE/LE 16-bit と 32-bit views を出力する。current sweep は 12 files で 261 rows を展開し、repeated frame row families を expose するが、geometry units、page association、paint order が証明されるまでは decoded-false evidence に留まる。
- `rjtd object-frame-record-families <file>` は、展開済み `/Frame` rows を decoded-false observation buckets に group する。61-sample sweep は同じ 261 records を 0 failures で group し、largest families は `frame-index-tail-coordinate-row12` 65、`frame-index-tail-window20` 65、`frame-index-mixed-row12` 61、`frame-index-flag-row12` 41 である。これは次の object-placement work を絞り込むが、rhwp の model-first rule は維持する。exporters は raw `/Frame` rows を直接読んではならない。
- `rjtd object-frame-row-links <file>` は、ほとんどの `u16-be/20/15` `tail-window20` rows が `u16-be/12/7` rows の context windows であることを検証する。61-sample sweep では 70 row20 windows のうち 65 が same-source 12-byte suffix row に link し、linked pairs はすべて `frame-index-tail-window20 -> frame-index-tail-coordinate-row12` である。したがって model-level frame row decoding では `u16-be/12/7` がより強い候補であり、20-byte rows は diagnostic context に留める。
- Parser/export JSON は同じ decoded-false row evidence を `objectStreamCandidates[].frameReferenceRows` として保存し、optional suffix links も含める。61-sample JSON sweep は 0 failures で 261 rows と 65 suffix links を保存する。これにより次の image-placement step は rhwp architecture に近づく。page paint construction は exporter から raw CFB streams を覗くのではなく、model-owned frame-row candidates を consume できる。
- `rjtd object-image-frame-candidates <file>` は、この model-owned frame evidence を image payload source 基準で summarize し、payload dimensions と coordinate/payload aspect delta も出力する。61-sample sweep は 0 failures、image payload source 付き files 12、image sources 52、frame-linked sources 52、missing-frame sources 0、frame rows 261、dimensioned payloads 35、coordinate/payload aspect candidates 付き sources 13 を報告する。Preferred diagnostic buckets は `row12-tail-coordinate` 25、`row12-tail-zero` 7、`u16-le-row12` 19、`none` 1 である。best aspect delta が 250 permille 以下の sources は `natsu.jtd` の 2 件だけなので、`row12-tail-coordinate` は強い placement-analysis candidate だが PDF image rendering に十分な coverage ではない。
- Parser/export/app-core JSON は、`/FigureData/*/FDMIndex` rows を sibling `FDMVector` segments に link した decoded-false `objectStreamCandidates[].fdmIndexEntries` も保存する。model は now whole stream を 22-byte rows として最後まで projection せず、valid `fdm-index-v1` declared-count prefix rows だけを consume する。多くの streams が row table 後に auxiliary payload bytes を持つためである。61-sample JSON sweep は 0 failures で、24 files の 30 candidates に 147 rows を保存し、3 files の 6 rows が raw diagnostic sweep と同じ 13 image hits を持つ。新しい `object-fdm-index-rows` sweep は declared invalid rows 43 件すべてを、3 files に集中した coordinate-like diagnostics として分類し、image-bearing vector segments ではないことを確認する。これは future page paint construction のための model-owned FDM evidence だが、page/frame/layout correlation が証明されるまで exporter は FDMVector images を render してはならない。
- `rjtd object-fdm-image-candidates <file>` と app-core `getPageOverlayImages` は、image-bearing FDMVector rows を unplaced diagnostics として expose する。61-sample sweep は 0 failures で、files 3、rows 6、image hits 13、strict complete payloads 0、plausible bbox rows 5、renderable rows 0 を見つける。FDMVector segments 内で観測される `FFD8FF` hits は、SOF/SOS structure を持つ valid JPEG payload ではなく vector data 内の JPEG-like byte patterns である。`getPageOverlayImages` は `behind`、`front`、`imageCount` を empty/zero に保ち、FDM rows を `placementProven:false`、`renderable:false`、`decoded:false` 付きの `unplacedDiagnostics` に置く。
- Parser/export/app-core JSON は、`/Frame` fixed 60-byte records も decoded-false `objectFrameRecords` として保存する。Image payload spans は optional `dimensions` を expose し、rhwp-aligned `image` dependency と狭い JPEG SOF metadata fallback を使う。JPEG payload spans は SOF/SOS validation 後にだけ promote される。`rjtd object-fdm-frame-links <file>` は image-bearing FDMIndex rows と frame records を `fdm row index == frame object id` で相関させ、frame size、payload dimensions、dimensioned payload counts、best frame/payload aspect delta も報告する。61-sample sweep は 0 failures で、FDM image rows 6 件すべてを frame records に link し、image hits 13、missing-frame rows 0、strict complete payloads 0、dimensioned payloads 0、renderable rows 0 を報告する。これは placement evidence trail を強めるが、実際の rendering 前には geometry units、page association、paint order、payload-to-image selection をさらに decode する必要がある。
- `getPageLayerTree` は fallback `pageBackground` op、fallback `textRun` ops、rhwp-shaped `textSources`/`source` spans を一緒に expose する。可能な場合、text layer entries は JTD byte/unit source ranges も含む。
- `getPageLayerTree` は fallback `writingMode` と per-text-run `isVertical`/`orientation` fields も保持する。vertical-writing renderer work はこれにより model-first のまま進められるが、current default は horizontal であり、Ichitaro style/layout flag はまだ decoded されていない。
- `getCanvasKitReplayPlan` は rhwp の `default`/`compat` mode policy に従い、fallback `pageBackground`/`textRun` paint ops をそれぞれ `replayPlane:"background"`/`replayPlane:"flow"` の direct replay items として report する。これは compatibility-projection geometry であり、recovered Ichitaro native paint operations ではない。
- WASM `HwpDocument` wrapper は同じ candidate style APIs を delegate し、labeled JTD style candidate に対する `getStyleList`、`applyStyle`、`getStyleAt` の regression coverage を持つ。
- `getPageLayerTree` は fallback `pageBackground` op、fallback `textRun` ops、rhwp-shaped top-level `textSources` および per-op `source` spans を出力する。parsed `/DocumentText` source spans が分かる場合、layer entries は JTD byte/unit source ranges も expose する。layer envelope は schema/resource table versions、output options、empty font resources、feature lists、fallback `textV2` diagnostics も含む。これはまだ fallback geometry であり、decoded Ichitaro layout ではない。
- `getValidationWarnings` は rhwp の `count`/`summary`/`warnings` JSON envelope に従い、fallback text pagination、preserved raw streams、undecoded style streams、preserved unknown objects、diagnostic-only `TCntV.01` ranges、diagnostic-only `TCntV.01` control-range overlap evidence など JTD 固有の fallback/preservation diagnostics を報告する。`reflowLinesegs` は fallback page cache を refresh するが、Ichitaro line segments はまだ decode していないため `0` を返す。
- この facade はまだ Ichitaro layout engine ではなく、current `Document` model の text blocks を page SVG として render する minimal compatibility surface である。
- `getPageLayerTree` は現在 fallback `pageBackground` と `textRun` paint ops を返し、replay APIs はそれらを direct background/flow items として report する。real paint operations は JTD layout/style/object recovery が進んだ後に埋める。
- rhwp の native PDF export は page ごとの SVG を作り、`usvg`、`svg2pdf`、`pdf-writer` で PDF output を assemble する。
- rjtd PDF export も同じ crates と direction を使う。現在は text-only SVG pages であり、tables、images、original page geometry はまだ recover しない。
- conditional `rjtd-export` regression test は available local `.jtd`、`.jtt`、`.jttc` samples すべてを parse し、各 sample が structurally plausible PDF bytes に export されることを確認する。
- body text を抽出できないが raw streams を保存している documents は、silent blank page を避けるため PDF/SVG output に visible diagnostic notice を render する。
