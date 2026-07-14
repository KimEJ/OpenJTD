# rjtd

OpenJTD の Rust ツール群と document-engine workspace

## Role

`rjtd` は OpenJTD の Rust ツール群である。日本のワードプロセッサ「一太郎 (Ichitaro)」
で使われる JTD 文書形式を解析・処理し、現在の parser、model、export、CLI、WASM、
app-core integration components を提供する。

このフォルダは、OpenJTD 全体の中で Rust implementation workspace に相当する。

プロジェクト全体の憲章とエコシステム計画は、上位の [docs/CHARTER.ja.md](../docs/CHARTER.ja.md) に従う。

## 0.0.1 Developer Preview

最初の crates.io release は experimental developer preview である。
OpenJTD は `rjtd-core`、`rjtd-model`、`rjtd-export`、`rjtd-cli`、
`rjtd-wasm` を公開し、`rjtd-testkit` は workspace 内部専用のままにする。
release scope は [CHANGELOG.md](CHANGELOG.md)、必須の公開順序は
[RELEASING.md](RELEASING.md) を参照する。

観察済み `.jtd`、`.jtt`、`.jttc` files は異なる範囲で対応しているが、
実装は完全な Ichitaro format specification ではない。`Candidate`、
`Unknown`、`Diagnostic` と名付けた値、および `decoded: false` の JSON
fields は、final semantics を主張せず reverse-engineering evidence を保持する。
public API と command output schema は後続の 0.0.x release で変更され得る。

## Foundational Principle: Follow rhwp

`rjtd` engine は可能な限り rhwp プロジェクトの構造と思想を参考にする。

rhwp は HWP/HWPX documents のための現代的な Rust-based document engine である。`rjtd`
はその構造を JTD 領域の参考にする。

したがって、project structure、layer separation、data model design、test strategy はまず rhwp を参照する。新しい構造を設計するより、検証済みの構造を再利用する。

## Architecture Policy

`rjtd` は rhwp と同じく次の階層を維持する。

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
      ├──── Markdown Export
      ├──── HTML Export
      ├──── JSON Export
      └──── App Core / SVG / PDF Export
```

すべての機能はこの階層を通じて実装する。特定の Exporter が元データを直接読んではならない。必ず Document Model を経由する。

現在の `rjtd-model::DocumentCore` は rhwp の app-core flow に従い、`from_bytes`、`page_count`、`get_document_info`、`get_page_info`、page/section setting fallbacks、`render_page_svg`、`render_page_html`、layer/overlay fallback APIs を提供する。`get_page_layer_tree` は fallback `textRun` ops と rhwp-shaped `textSources`/`source` spans を出力し、parsed `/DocumentText` spans がある場合は JTD byte/unit source ranges も含める。layer envelope も schema/resource table versions、output options、empty font resources、feature lists、fallback `textV2` diagnostics を持つ rhwp-shaped output である。`rjtd-wasm` は rhwp Studio が期待する surface に合わせた名前の `HwpDocument` wrapper を提供する。

## Document Model First

rjtd の中核は Parser ではない。Document Model である。

すべての Parser は Document Model を生成しなければならない。すべての Exporter は Document Model を consume しなければならない。

## Unknown Preservation Rule

解析されていない data は絶対に破棄しない。

```text
UnknownRecord
UnknownBlock
UnknownStyle
UnknownObject
```

リバースエンジニアリング中の data loss を防ぐ。

## Default Resource Limits

public parser surface は 64 MiB を超える source input を拒否する。LH5 member
は 256 MiB を超える declared output を拒否し、1 MiB の allowance を超える
output は packed member size の 256 倍以内でなければならない。browser canvas
rendering は各辺 16,384 pixels、合計 64 MiPixels に制限される。

これらは pre-stable safety ceiling であり compatibility guarantee ではない。
stream、record、embedded image、page 単位の budget は引き続き強化中であるため、
untrusted document は適切に制限した環境で処理し、bypass の可能性は上位の
[security policy](../SECURITY.md) から報告する。

## Workspace Layout

```text
rjtd/
├── crates
│   ├── rjtd-core
│   ├── rjtd-model
│   ├── rjtd-export
│   ├── rjtd-cli
│   ├── rjtd-wasm
│   └── rjtd-testkit
├── docs
├── samples
├── fuzz
├── tests
└── tools
```

現在使っていない crate もあらかじめ作成する。これは project growth direction を固定するためである。

## Commands

```sh
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Current CLI

```sh
cargo run -p rjtd-cli -- streams <file.jtd>
cargo run -p rjtd-cli -- info <file.jtd>
cargo run -p rjtd-cli -- dump-stream <file.jtd> /DocumentText
cargo run -p rjtd-cli -- cfb-map <file.jtd>
cargo run -p rjtd-cli -- cfb-dir <file.jtd>
cargo run -p rjtd-cli -- stream-meta <file.jtd> /DocumentText
cargo run -p rjtd-cli -- stream-chain <file.jtd> /DocumentText
cargo run -p rjtd-cli -- stream-words <file.jtd> /LineMark
cargo run -p rjtd-cli -- stream-word-frequencies <file.jtd> /LineMark
cargo run -p rjtd-cli -- line-mark-tags <file.jtd>
cargo run -p rjtd-cli -- line-mark-text-context <file.jtd>
cargo run -p rjtd-cli -- stream-dwords <file.jtd> /PageMark
cargo run -p rjtd-cli -- stream-dword-frequencies <file.jtd> /PageMark
cargo run -p rjtd-cli -- stream-text-probe <file.jtd> /PageMark
cargo run -p rjtd-cli -- stream-find <file.jtd> /PageMark
cargo run -p rjtd-cli -- stream-find-bytes <file.jtd> 534f0000
cargo run -p rjtd-cli -- so-records <file.jtd>
cargo run -p rjtd-cli -- so-record-clusters <file.jtd>
cargo run -p rjtd-cli -- so-record-fields <file.jtd>
cargo run -p rjtd-cli -- so-record-geometry <file.jtd>
cargo run -p rjtd-cli -- so-record-halves <file.jtd>
cargo run -p rjtd-cli -- cat <file.jtd>
cargo run -p rjtd-cli -- text-tokens <file.jtd>
cargo run -p rjtd-cli -- text-control-context <file.jtd> [control-code]
cargo run -p rjtd-cli -- text-control-clusters <file.jtd>
cargo run -p rjtd-cli -- text-positions <file.jtd>
cargo run -p rjtd-cli -- text-position-counts <file.jtd>
cargo run -p rjtd-cli -- text-position-count-context <file.jtd>
cargo run -p rjtd-cli -- text-position-count-tail-context <file.jtd>
cargo run -p rjtd-cli -- text-position-count-clusters <file.jtd>
cargo run -p rjtd-cli -- text-position-count-candidates <file.jtd>
cargo run -p rjtd-cli -- text-position-count-family <file.jtd>
cargo run -p rjtd-cli -- text-position-count-fields <file.jtd>
cargo run -p rjtd-cli -- text-position-count-field-deltas <file.jtd>
cargo run -p rjtd-cli -- text-position-count-tail-delta-scan <file.jtd>
cargo run -p rjtd-cli -- text-position-count-tail-delta-groups <file.jtd>
cargo run -p rjtd-cli -- text-position-count-tail-row-deltas <file.jtd>
cargo run -p rjtd-cli -- text-position-count-tail-row-context <file.jtd>
cargo run -p rjtd-cli -- text-position-count-range-preview <file.jtd>
cargo run -p rjtd-cli -- text-position-count-range-boundaries <file.jtd>
cargo run -p rjtd-cli -- text-position-count-layout-context <file.jtd>
cargo run -p rjtd-cli -- text-position-mark-header <file.jtd>
cargo run -p rjtd-cli -- text-position-mark-summary <file.jtd>
cargo run -p rjtd-cli -- paper-marks <file.jtd>
cargo run -p rjtd-cli -- paper-mark-shape <file.jtd>
cargo run -p rjtd-cli -- page-marks <file.jtd>
cargo run -p rjtd-cli -- page-mark-shape <file.jtd>
cargo run -p rjtd-cli -- text-map <file.jtd>
cargo run -p rjtd-cli -- text-position-context <file.jtd>
cargo run -p rjtd-cli -- text-position-line-context <file.jtd>
cargo run -p rjtd-cli -- text-position-delta-scan <file.jtd>
cargo run -p rjtd-cli -- export <file.jtd> --format json
cargo run -p rjtd-cli -- export <file.jtd> --format md
cargo run -p rjtd-cli -- export <file.jtd> --format text
cargo run -p rjtd-cli -- export <file.jtd> --format pdf -o output.pdf
```

`streams` は観察済み `.jtd`、`.jtt`、`.jttc` CFB samples で動作する。まず `cfb` crate を使い、malformed CFB files では narrow rhwp-style lenient FAT reader に fallback する。`.jttc` では outer CFB inventory を報告する。

`info` は検出した compound-document shape と key stream sizes を報告する。

`cfb-map` は special CFB sector chains を報告する。FAT sector ids、directory chain、mini FAT chain、root mini-stream chain を含み、root mini-stream chain が directory chain や他の special CFB structures と overlap する malformed files の検出に役立つ。

`cfb-dir` は raw CFB directory entries を directory id、object type、size、start sector、left/right/child ids、resolved path、raw name、name length とともに報告する。suspicious stream を nearby sibling entries や embedded object storages と比較するときに有用である。

`stream-meta` は one stream の CFB directory metadata を報告する。stream size、start sector、regular FAT/mini FAT のどちらに保存されているか、observed mini-stream sizes を含む。diagnostic only。

`stream-chain` は stream の FAT または miniFAT sector chain を展開し、chain status、sector ids、file または mini-stream byte offsets を含める。directory entry が structurally complete な sector chain を持つにもかかわらず stale、foreign、alternate payload bytes を指す可能性がある場合に有用である。

`stream-words`、`stream-word-frequencies`、`stream-dwords`、`stream-dword-frequencies` は任意の stream を raw big-endian 16-bit または 32-bit values として調べる。generic reverse-engineering diagnostics であり、record parser を意味しない。

`line-mark-tags` は `/LineMark` から current tag-like `0x1000`、`0x1001`、`0x1002` words を scan し、各 tag の word index、byte offset、previous four words、next six words を出力する。semantics を割り当てる前に LineMark records を grouping するための diagnostic である。

`line-mark-text-context` は各 `/LineMark` tag row を `/DocumentText` token map と比較する。tag の LineMark byte/unit contexts、immediate next word が raw `/DocumentText` に現れるか、first raw word hit、`line-mark-tags` と同じ surrounding LineMark words を報告する。

`stream-text-probe` は任意の stream から printable ASCII、UTF-16LE、UTF-16BE string candidates を scan する。stream entry が stale、foreign、alternate-encoded payload bytes を指しているように見える場合に有用である。

`stream-find` は一つの stream の exact bytes を同じ CFB file 内の他の readable streams すべてから検索する。stale または duplicate stream payload を likely owning stream まで追跡するのに役立つ。

`stream-find-bytes` は user-provided hex byte sequence をすべての readable stream から検索する。`SO` (`534f0000`) のような marker や coordinate-like field values を object/control streams 間で追跡するのに役立つ。

`so-records` は観察済み `SO\0\0` object/control marker をすべての readable stream から scan し、stream path、offset、first little-endian 32-bit fields、preserved raw bytes を出力する。diagnostic only。

`so-record-clusters` は `SO` records を preserved raw bytes で group し、counts と stream-offset locations を報告する。field semantics を decode する前に repeated default/control records と singleton geometry-like records を分離するのに有用である。

`so-record-fields` は各 `SO` record を little-endian 32-bit fields、signed view、low/high 16-bit view に展開する。coordinate-like values と `0x00000100`、`0x00000064` のような constants を比較するのに役立つ。

`so-record-geometry` は `SO\0\0` marker 後の first four payload fields を diagnostic geometry candidates として分類する。raw `f1..f4` values、`xyxy` width/height deltas、`xywh` right/bottom sums、preserved raw bytes を報告する。class names は意図的に保守的で、`geometry-like`、`default-control`、`packed-jseq3-like`、`packed-ffff-preamble`、`packed`、`truncated`、`unknown` を使う。

`so-record-halves` は各 `SO` payload dword を low/high 16-bit unsigned and signed halves として出力する。current samples では `JSEQ3Contents` の packed SO-like records が一つの packed field の low 16 bits を後続 dword に繰り返すため、この比較に有用である。

`cat` は現在 structured `ParsedDocumentText` token parser を使う。`/DocumentText` を持つ観察済み `.jtd` と `.jtt` samples を読み、common visible inline segments と control boundaries を含む。観察済み `.jttc` samples は LHA dependency を追加せず `/JSCompDocument` `JustCompressedDocument` `-lh5-` payloads を decode して開く。local sample が named `/DocumentText` を expose しない場合、observed embedded `SsmgV.01`/`TextV.01` fragments を復元し、format を `cfb-embedded-document-text` として報告できる。

`text-tokens` は structured `ParsedDocumentText` stream を tab-separated `text`、`inline`、`skipped-inline`、`control` rows として出力する。

`text-control-context` は各 `/DocumentText` control boundary を byte/unit range、previous/next map entries、nearest previous/next control boundary とともに出力する。optional decimal または hex control code で filter できる。例: `0x001c` または `14`。diagnostic only であり final control semantics は割り当てない。

`text-control-clusters` は隣接する `/DocumentText` control boundaries を group し、各 cluster の entry range、code sequence、byte/unit range、neighboring map entries を出力する。paragraph/table/object boundary candidates を絞り込むための diagnostic only command である。

`text-positions` は `/DocumentTextPositionTables` から initial parsed `MarkV.01` entries を tab-separated `id` と raw offset rows として出力する。diagnostic only で、model generation にはまだ使わない。

`text-position-counts` は `/DocumentTextPositionTables` から観察済み `TCntV.01` numeric entries を diagnostic rows として出力する。table は現在 stream offset `0x0024` から始まる 29-byte records と見える。

`text-position-count-context` は first two `TCntV.01` fields を `/DocumentText` token map と byte offsets/UTF-16 unit offsets の両方として比較する。diagnostic only。observed samples は mixed で、final coordinate semantics は未解読。

`text-position-count-tail-context` は tail `t1/t2` fields を `/DocumentText` token map と byte offsets/UTF-16 unit offsets の両方として比較する。current samples は byte-coordinate signal より unit-coordinate signal が強いが、完全な rule ではない。

`text-position-count-clusters` は `TCntV.01` records を provisional `(start, end)` pair で group し、duplicate raw-tail variants を表示する。`text-position-count-candidates` は `be32@0/4` と shifted `be32@1/5` field candidates の両方を出力する。one observed sample は両 candidate families を使う。`text-position-count-family` は record を current `be0` または `be1-shifted` diagnostic family として分類し、candidate offsets と remaining raw tail を出力する。`text-position-count-fields` は tail を `u16be` fields と extra trailing byte に展開する。`text-position-count-field-deltas` は chosen range span と tail `t1..t2` span を semantic field names なしで比較する。`text-position-count-tail-delta-scan` は `t1/t2` に small positive deltas を UTF-16 unit offsets として scan し、MarkV-like adjustment が hits を改善するかを検査する。`text-position-count-tail-delta-groups` は同じ scan を `(family,t0,t3,t4,t7)` pattern ごとにまとめ、subfamily-specific coordinate behavior を global offsets から分離する。`text-position-count-tail-row-deltas` は row granularity で同じ score を出し、document byte/unit length と chosen/tail spans を含める。`text-position-count-tail-row-context` は chosen start/end byte/unit contexts と best-delta tail contexts を同じ row に追加する。`text-position-count-range-preview` は chosen range が overlap する `/DocumentText` entries を byte/UTF-16 unit intervals として要約し、token-kind counts と escaped text preview を含める。`text-position-count-range-boundaries` は同じ byte/unit intervals に edge alignment、first/last/previous/next entries、control-code counts を追加する。`text-position-count-layout-context` は chosen family range を `/LineMark` word/byte offsets と parsed `/PageMark`/`/PaperMark` row/byte offsets と比較する。

`paper-marks` は観察済み `/PaperMark` header values と 8-byte `(index, flags)` rows を出力する。diagnostic only。row shape は現在の local `/PaperMark` streams の大半で安定しているが、header count values と flags の semantic meaning は未解読。

`paper-mark-shape` は `/PaperMark` stream length、declared CFB size、header values、fixed 8-byte row candidates を出力する。normal `/PaperMark` rows と stale/foreign payload bytes を分ける non-failing diagnostic である。

`page-marks` は観察済み `/PageMark` row families を header values、family name、raw-preserved rows、preserved trailing bytes として出力する。diagnostic only。現在は fixed 84-byte rows、fixed 84-byte rows with a tail、count-plus-one variable rows、count-variable rows を cover するが、すべての local `/PageMark` variants は cover しない。

`page-mark-shape` は `/PageMark` stream length、declared CFB size、header values、candidate row formulas を出力する。fixed 84-byte rows、header-count rows、2-byte-trimmed variants などを含み、parser を広げる前に variants を分類する reverse-engineering helper である。

`text-map` は structured `/DocumentText` token map を byte ranges、UTF-16 unit ranges、token kind、selector/code metadata、各 range 内に入る `MarkV.01` ids とともに出力する。diagnostic only。

`text-position-context` は各 `MarkV.01` offset を raw byte offset、UTF-16 unit offset、provisional `unit + 29` probe の三通りで token map と比較する。`text-position-delta-scan` は unit deltas `0..64` を unit hits と visible text hits で採点する。`text-position-mark-header` は `MarkV.01` と first entry の間の raw six bytes を出力する。`text-position-mark-summary` は Mark header を `/DocumentText`、`/LineMark`、`/PageMark`、`/PaperMark` metrics と相関させる。`text-position-line-context` は Mark header と entry offsets を `/LineMark` word contexts と nearest tag rows と比較する。current `a5.jtd` family samples では `unit + 29` が visible heading text に着地することが多いが、delta scan はそれが unique ではないことを示す。

`style-records` は preserved style stream summaries と record candidates を family、header candidates、record layout、offsets、codes、payload lengths、conservative labels 付きで出力する。`style-candidates` は cross-sample correlation 用に labeled `/TextLayoutStyle` candidates を stable per-document rows として列挙する。`text-layout-style-records` は all `/TextLayoutStyle` records を payload digests と previews 付きで出力する。`document-view-style-groups` は `/DocumentViewStyles` group record payload lengths、digests、short previews を出力する。`text-position-style-context` は `TCntV.01` tail fields を text/page style candidate IDs、record indexes、`/DocumentViewStyles` group records と比較し、`text-position-style-summary` はそれらの hits を tail field ごとに aggregate し、`text-position-count-tail-field-roles` は tail fields と adjacent pairs を document-text unit/text hits と比較する。parsed `TextRun` values は `/DocumentText` byte/UTF-16 source span を保存し、valid `TCntV.01` entries は model JSON と app-core document info で byte/unit `documentTextOverlaps` 付き decoded-false `textCountRanges` として保存される。これらは reverse-engineering diagnostics であり、decoded paragraph style assignments ではまだない。

`export` は `DocumentParser` entry point 経由で parse し、`ParsedDocumentText` を consume して minimal `Document` model を構築する。raw text source を model に保存し、skipped inline text を `UnknownObject` payloads として保存し、observed ruby base/phonetic pairs を `Inline::Ruby` に promote し、observed style/layout streams を named `UnknownStyle` entries として保存したうえで、JSON、Markdown、plain text、native PDF を出力する。plain text、Markdown、PDF は visible ruby base text を使い、JSON は annotation text、style stream names、observed style stream family/header summaries、conservative label candidates 付き neutral record boundary candidates、raw payloads を保持する。PDF export には `-o`/`--output` が必要で、rhwp の native pipeline direction に従う: `DocumentCore` が text SVG pages を render し、`rjtd-export` が `svg2pdf` と `pdf-writer` で変換する。観察済み `.jttc` では decompressed inner CFB 由来の `/DocumentText` と style streams が保存される。embedded samples では source は `/EmbeddedDocumentText` として保存される。preserved raw streams はあるが extractable text がまだない document では、PDF/SVG output は silent blank page ではなく visible diagnostic notice を表示する。HTML export は後の milestone に reserved。
