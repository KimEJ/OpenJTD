# RFC 0009: DocumentText Record Framing and Ruled-Line Flow

Status: draft for publication preparation; joint review pending

English source: [0009-document-text-paragraph-record.md](0009-document-text-paragraph-record.md)

## 範囲と反例

調査した non-inline `0x001c` record は class/length echo を持つ。ただし全 `0x001c` が段落 record
ではない。inline opener と literal control sequence が反例である。所属する本文範囲内で完全な
record を検証してから field を解釈する。

## wire layout

観測した big-endian text/control stream の u16 word は以下である。

```text
w0              0x001c
w1              class
w2              opener/footer を含む総 word 長
w3 .. w[len-5]  class 固有 payload
w[len-4]        length echo
w[len-3]        0x0000
w[len-2]        class echo
w[len-1]        0x001f
```

初期 2 文書・948 record で footer を確認したが、新たな public 再現を意味しない。
inline `001c 0001 0007 ... 001d` は別の cache/terminator 形状である。
class zero、`0x0010`、`0x0020`、`0x0030` があり、段落・設定・セルの名称は profile ごとに検証する。

## 罫線の流路

対照 parent profile には class `0x0010`、`w4=0x008f` がある。完全な `0x0030` は cell text に
先行し、boundary/coordinate-like field を持つ。表 context の `0x000e` は一語 separator であり、
`0x000a` は cell/paragraph 内にもある。separator 数や固定の矩形階層だけでは所有を定義できない。

先行本文が最初の row と同じ `0x000e` interval にありながら parent header より前に終わる例がある。
wrap、空セル、境界消去/merge、末尾空行、単一行/列、改ページをまたぐ表は、物理 source range と
boundary 宣言を保持して扱う。高水準の表再構成で前後・途中の本文を落とさず、framing のない文字列
一致だけで表としない。

余白、row/column span、宣言境界、line/page mark、文字属性は独立の制約である。過去の配置式や
border/font 既定値は実装候補であり、PDF 座標との一致は普遍的な原本単位の証明ではない。

## 末尾 style event

本文の終端 `32 + 2 * content_unit_count` の後に byte-oriented event があり、source-unit cursor は
unit 16 から始まる。`00 <BE-u32 length>` は run、`fe` から property ID・長さ・値を反復して
`ff 00` で閉じる event は一 source unit、`ff` は終端となる。変更は後続 run へ持続する。

| Property ID | value bytes |
| --- | ---: |
| 4–7, 9–12 | 1 |
| 1–3, 8, 13, 14, 18, 19 | 2 |
| 15–17, 20 | 4 |

未知 ID、幅不一致、末尾 bytes は未解釈のまま保持する。property 15 は対照 text range で
`0x00BBGGRR` 色と対応し、black/red/blue と automatic/default `0xffffffff` がある。一方、
非本文の table-state range にもあり、普遍的 color 解釈の反例となる。対応の範囲は RFC 0003。

## 必要な検証

許可された入力 ID/hash と native 作成 version を profile ごとに添える。wrap、余白、列数、行間、
merge、border、page flow を独立に変更して再現し、観測・仮説・出力近似・parser 回帰を区別する。
反例を保持し、一般 row 所有、style 継承、座標 normalization、edit/save grammar を確定しない。

## 実装記録と履歴

[実装の command・API・出力・過去の詳細記録](../../rjtd/docs/research/0009-document-text-paragraph-record.ja.md).
