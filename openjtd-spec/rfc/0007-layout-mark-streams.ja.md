# RFC 0007: Layout Marks and Page Instructions

Status: draft for publication preparation; joint review pending

English source: [0007-layout-mark-streams.md](0007-layout-mark-streams.md)

## 範囲

`/LineMark`、`/PageMark`、`/PaperMark`、`/PageLayoutStyle`、`/Header` に layout 関連の
record がある。存在や格納形状だけで一般 layout algorithm や page count は確定しない。
初期の過去文書と現行 version の対照 profile は別々に範囲を記録する。

## mark family

PageMark の一 family は 12-byte header と 84-byte row。header は `(N,16,N-1)` の例があるが、
N は格納 row 数や可視 page 数と一致するとは限らない。variable-row family もある。source line
範囲に対応する値はあるが、mixed payload と sentinel は全 row を body page とする解釈の反例となる。
PaperMark の一 family は `(N,12,N-1)` header と BE-u32 index/flags 対を持ち、調査例の N は
PageMark と同じである。意味と全 flag は未解読。

LineMark は入力により `0x0914`、`0x090b`、`0x0912` などで始まり、`0x1000`/`0x1001`/`0x1002`
の tag-like 値を持つ。対照 profile は source-unit interval と物理行・縦書き column を対応付けるが、
header word の初期推測だけで方向や他 family への一般規則は解読できない。

## page style と方向

対照 class `0x0020` record は style ID 1 を適用し ID 0 に戻す。既知の 3-page 対照は middle page
だけ landscape、PageMark flags `0x00050100`、前後は `0x00010000`。完全な profile 内の対応であり、
一般継承 grammar ではない。

258-byte sequential view `0x1001` は offsets 126/130 と 154/158 に LE-u32 stock size 対を反復する。
関連する 267-byte form は 9-byte orientation prefix を加え、1/5 に BE-u32 寸法を持つ。
寸法のコピーと余白が一致する必要がある。方向・間隔は RFC 0003。原本の単位と page-relative
anchor は形式解釈、pixel 変換と font baseline は出力実装に属する。

## plain header slot

観測 `/Header` は `SsmgV.01`、byte 8 の BE-u32 count、byte 12 の `0x100` で始まる。
`16+512*i` から 512-byte slot があり、+4 に `TextV.01`、+12 に BE-u32 unit count、+16 に本文、
その後に zero byte と反復 count がある。+260 は空の `TCntV.01`。末尾 table は zero prefix/suffix と
slot ごとの BE-u32 6語 `[slotID,0,17+2*units,1,1,2*i]` を持つ。

対照 ID 0/1/2/5/6 は page number・primary/secondary header/footer に対応する候補。
odd/even と cover も既知 view profile に限定する。sequential view extent 後の trailer は code/u32-length
section `0x1001`/`0x1002` であり、u16-length style record ではない。観測した 2026-byte settings
payload の byte 8 の BE-u32 は page-number printing に伴い 0→2。slot の存在だけでは印刷 ON にならない。

## 未解決事項

他 row family、counter、原点、flag、section 継承、styled/multiline slot、任意改番を解明する。
入力 version と設定を主張に添え、保存 source 位置と backend font 計測を区別する。描画成功は
未知 field、脚注 layout、一般 pagination の解読ではない。

## 実装記録と履歴

[実装の command・API・出力・過去の詳細記録](../../rjtd/docs/research/0007-layout-mark-streams.ja.md).
