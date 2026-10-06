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

## JTD 固有のエンジンと設計参考

エンジンの model と挙動は JTD の原本根拠に従う。rhwp は layer 分離・保存・検証の有用な
先例であり、必須の HWP document model やエディタ API 全体の契約ではない。
[rhwp の参照・連携範囲](../docs/RHWP-COMPATIBILITY.ja.md) を参照する。

## Architecture Policy

`rjtd` は原本解析、model の所有、出力を次の階層に分離する。

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
      ├──── Plain Text / Markdown Export
      ├──── HTML Export
      ├──── JSON Export
      └──── App Core / SVG / PDF Export
```

すべての機能はこの階層を通じて実装する。特定の Exporter が元データを直接読んではならない。必ず Document Model を経由する。

`rjtd-model::DocumentCore` は読込、ページ照会、SVG/HTML 描画、layer diagnostics、
基本的な本文編集・検索・選択・clipboard・snapshot を提供する。解析できた箇所の
source byte/unit span を保持する。`rjtd-wasm::HwpDocument` は rhwp 形状の browser API
を公開する。
既存の名前と動作済みのビューア呼出しは維持するが、Studio API 全体の coverage は
JTD 機能の完了目標ではない。

高度な書式、表・セル、オブジェクト、header/footer、note の多くのメソッドは、既定値、
no-hit、no-op を返す。設定メソッドの一部は decoded JTD settings を保存せず成功を返す。
これらの互換 surface は native 編集や保存への対応を意味しない。HWP/HWPX 出力は空の
byte 列を返し、`exportHwpVerify` は変換が未実装であることを報告する。

基本的な文書 HTML 出力は `rjtd-export` と CLI に実装され、段落テキストとルビ markup
を含む。app-core の rich HTML clipboard は限定的で、本文選択は escape 済みの段落
テキストを出力できる。現在のマイルストーンと完了条件は [roadmap](../docs/ROADMAP.ja.md)
を参照する。

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
limits-aware な文書読込では、stream、record、embedded image、page の構築で同じ
`ParseLimits` resource budget を共有する。既定値は CFB の異なる stream path 1,024 件と
合計計上サイズ 64 MiB、保持する frame/embedding record 65,536 件と 64 MiB、画像
1,024 件と payload/envelope 合計 64 MiB、ページ 65,536 件と保持する page line 1 Mi 件。
画像 header 由来の幅・高さは各 16,384、累積 pixel 数は 64 MiPixels に制限する。

Strict CFB は宣言サイズ、lenient recovery は sector chain から到達可能な byte 数を
計上する。同じ path は 1 回だけ数え、実際の読込量を計上値と照合する。画像の寸法は
payload/envelope を clone する前に PNG、GIF、BMP、JPEG の header から取得する。
このモデル読込経路では bitmap を decode・保持しないため、後段の decoded-image
allocation を制限するという主張ではない。信頼できない文書は適切に制限した環境で処理し、
bypass の可能性は上位の [security policy](../SECURITY.md) から報告する。

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

6 つの workspace crate はそれぞれ実装または fixture support の役割を持つ。
`fuzz/`、`docs/`、`samples/`、`tests/`、`tools/` は予約された workspace directory で、
crate のテストは `crates/` 以下にある。directory の存在だけで fuzzing や test program の
実装済みを意味するわけではない。

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
cargo run -p rjtd-cli -- export <file.jtd> --format html
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

`export` は `DocumentParser` entry point 経由で parse し、`ParsedDocumentText` を consume して minimal `Document` model を構築する。raw text source を model に保存し、skipped inline text を `UnknownObject` payloads として保存し、observed ruby base/phonetic pairs を `Inline::Ruby` に promote し、observed style/layout streams を named `UnknownStyle` entries として保存したうえで、JSON、Markdown、plain text、基本 HTML、native PDF を出力する。plain text、Markdown、PDF は visible ruby base text を使い、JSON は annotation text、style stream names、observed style stream family/header summaries、conservative label candidates 付き neutral record boundary candidates、raw payloads を保持する。PDF export には `-o`/`--output` が必要で、model を経由する SVG-to-PDF 経路を使う: `DocumentCore` が text SVG pages を render し、`rjtd-export` が `svg2pdf` と `pdf-writer` で変換する。観察済み `.jttc` では decompressed inner CFB 由来の `/DocumentText` と style streams が保存される。embedded samples では source は `/EmbeddedDocumentText` として保存される。preserved raw streams はあるが extractable text がまだない document では、PDF/SVG output は silent blank page ではなく visible diagnostic notice を表示する。基本 HTML は段落とルビ markup を出力する。完全な layout と rich clipboard semantics は未完成。

限定的な文字範囲の太字・斜体・一本下線候補、半サイズの上付き・下付き、色・サイズ変更、font ID と既定値復帰を保持する。SVG/PDF と viewer は backend の文字送りを run 間で再利用する。flag、font mapping、synthetic paint、添字位置、折返し、native 空白の candidate/fallback 制限は [RFC 0003](../openjtd-spec/rfc/0003-document-text.ja.md) を参照。 PDF の synthetic bold は選択可能な文字を一度だけ記録し、同じ解決済み font の glyph outline で stroke を描く。SVG/viewer の paint と model evidence は維持する。候補の太字強度を変えず、PDF text extraction の重複を防ぐ。 PDF の generic family は実在する font を選び、macOS の優先 font と利用可能な Linux fallback を使う。

補助の脚注・リンク・bookmark tag stream を raw のまま保持する。限定した一 link の脚注 profile は、source に対応した note text 候補を JSON にも公開する。本文、脚注配置、field 評価は分離して扱う。

named text の明示改ページ後も本文を保持し、model inline span は表示 UTF-16 unit を指す。raw wrapper は保全する。限定した見出し・一覧 cache は物理 source 行と fixed84 page 範囲を再利用する。別の動的 field profile と一般 native typography は未解決。

限定した保存済み目次領域の title/page label を保全し、設定 record に本文行高を割り当てず物理 page 範囲を再利用する。一般 tab stop、生成と編集可能な目次 semantics は未解読。統制した保存済み leader profile は後述する。

限定した印刷日・page・link cache と source 対応を保持する。印刷日は明示 render context（browser/Unix PDF の local date を既定で使用）で描画し、raw cache を変更しない。外部 link の色・下線と HTTP(S) SVG target は対応 record に従う。一般的な再番号付け、bookmark 位置、別 field profile は未解決。

JTTC model は解凍した inner CFB を補助 layout mark・note/bookmark・object/frame stream に再利用し、圧縮 source と共有 resource limit を保全する。JTT/JTTC の source-page 配置は同じ model 経路に従う。

統制した単一書式の縦長・横長・縦長 profile は、SVG・page/layer info・PDF・canvas 寸法で page ごとの layout を選ぶ。未知の書式対応や異なる余白・紙 profile は fallback を保ち、他 page typography・一般 section 編集は未解決。

plain `/Header` slot を raw のまま保存し、source に対応する text 候補を公開する。限定した global 横書き profile の header・footer・見開きの切替・表紙非表示・有効な中央 page-number pattern を SVG/PDF と layer info に描画する。slot role、名目 anchor、font metrics、一般的な番号付けは候補のまま。[RFC 0007](../openjtd-spec/rfc/0007-layout-mark-streams.ja.md) を参照。

統制した modern view profile で横書き・縦書きと source の30mm余白を選ぶ。plain 縦書きは source の列と PageMark pitch を再利用し、既知の字間60% profile と二桁・幅に収めない縦中横 cache を SVG/PDF と layer の共通投影で描画する。数字を本文に復元し、raw control と unknown cache を保全する。他 profile、native font metrics、英字・空白の送り、縦用 glyph 置換は candidate/fallback の制限を残す。[RFC 0003](../openjtd-spec/rfc/0003-document-text.ja.md) を参照。

統制した単一 PNG profile の frame/cache/image を関連付け、source 位置・寸法、inline 挿入、両側回避、前面描画を SVG/PDF と layer に反映する。raw data と隠れた本文を保全する。inline baseline と代替 font の回避に差が残り、複数画像・rich/縦書き・別 profile は diagnostic のまま。[RFC 0008](../openjtd-spec/rfc/0008-object-stream-candidates.ja.md) を参照。

統制した四角形・楕円・直線 profile は source 空白行 anchor、frame geometry、FDM command、fill color、独立に照合した paint order を関連付ける。SVG/PDF と layer は投影を共有し、raw 図形 stream と decoded-false evidence を保全する。一般単位、透明・connector profile、編集可能な図形 semantics は未解決。[RFC 0008](../openjtd-spec/rfc/0008-object-stream-candidates.ja.md) を参照。

統制した JSEQ/GCI text snapshot は zero-based の最初の frame を関連付け、source 数式文字・italic/添字サイズ・確保した本文行を SVG/PDF と layer に描画する。editable formula と snapshot の文字 packet 一致を要求する。font baseline と一般数式・編集 semantics は候補のまま。[RFC 0008](../openjtd-spec/rfc/0008-object-stream-candidates.ja.md) を参照。

統制した global 横長・字間60% profile は、物理 source 行と和文の字間を再利用し、英字 advance は backend に委ねる。SVG/PDF と layer は source span と decoded-false geometry を保つ。他 tracking profile と正確な空白 metrics は未解決。

統制した保存済み目次 profile は、実線・点線 leader と題名に隣接する page label を区別する。完全な title/leader record と空 cache に限定し、一つの区切り cell、本文右端の label、物理 source 行、backend の文字送りを SVG/PDF と layer に反映する。leader metrics と一般 tab/navigation/edit semantics は decoded-false 候補のまま。[RFC 0003](../openjtd-spec/rfc/0003-document-text.ja.md) を参照。

標準脚注 marker は本文・脚注の個別 source span と照合済み parent-style 参照を保持する。限定した横書き本文 marker を半サイズの上付きに置き、literal な二段落 profile は一つの source 行間を維持する。parsed ruby base の source span・font advance を保持し、限定した grouped kana 間隔を描画する。SVG/PDF/layer geometry は候補のまま。脚注領域配置と一般 style/ruby 継承は未解明。[RFC 0003](../openjtd-spec/rfc/0003-document-text.ja.md) を参照。

統制した方向混在の中間 page は、選択した page style の照合済み字間60%を和文に反映する。既知4006/400a profile で同じ font size・100% scale・検証したstyle/page/margin対応を要求する。前後の縦長pageは従来描画を維持し、SVG/PDF/layer は字間basisを公開する。他page typographyと正確なfont/空白metricsは未解明。

統制した plain 段落の固定10mm profile は、物理的な折返し行と source に対応する改行幅を SVG/PDF・layer に反映する。単独の保存済み pitch はその段落だけに適用し、未知 profile、source 対応の喪失、縦書き・grid 混在は fallback を維持する。raw 本文・PageMark を変更せず、glyph metrics は候補のまま。

採用済み control grid の padding は、表示 label と独立に先頭空白自身の source font size を使う。SVG/PDF・layer の位置は限定した相対 size 調整を共有し、raw 空白・source 範囲と個別の padding provenance を保持する。混在・未対応の scale profile は fallback を保つ。

限定した `/MarkTag` の bookmark 名を model/JSON の source 候補として公開し、directory 値と byte 範囲を保持する。元の position table は通常・圧縮 container のどちらでも raw を保存する。位置・navigation・編集は未解明のままで、heuristic offset を bookmark 座標として示さない。
