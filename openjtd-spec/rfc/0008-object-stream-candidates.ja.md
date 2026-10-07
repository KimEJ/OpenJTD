# RFC 0008: Object Streams, Anchors, and Paint Candidates

Status: draft for publication preparation; joint review pending

English source: [0008-object-stream-candidates.md](0008-object-stream-candidates.md)

## 根拠の範囲

path、image signature、`SO\0\0` は候補データを示すだけで、所有、geometry、描画順序は確定しない。
`/Frame`、`/Figure`、`/LayoutBox`、`/EmbedItems`、`EmbeddingInfo`、`Contents`、
`EmbeddedPress`、`FDMIndex`、`FDMVector` がある。named table object がない罫線文書もあり、
流路は RFC 0009 で調べる。以下の対応は現行 version の対照 profile に限定する。

## 単一 PNG frame

76-byte Frame と 60-byte record（ID 0/type 1）が、完全な PNG、
`/EmbedItems/Embedding 1/Contents`、完全な source object/cache group に対応する。
offset は 60-byte row 内である。

| Offset | 対照での対応 |
| --- | --- |
| 16, BE-u16 | 0 inline、1 floating |
| 28/32/36/40 | 整数 x/y/width/height、隣接する下位 word は zero |
| 44 | 観測例の 200 は text clearance 2 mm |
| 46 | 1 wrap、4 front |

frame 寸法、image aspect、連続した source context が対応を裏付ける。class-zero/tag `0x0030`
record は inline `0x0107,0x0010`、floating `0x0507,0x0012`。単なる整数一致は所有の証拠ではない。
inline baseline、任意 alignment、crop、他の image set は未解明。

## 図形順序と fill

対照 3-figure profile は Frame row offset 8 の kind 4、12 の one-based slot と、別の Figure ID
順序を対応付ける。rectangle/ellipse の順序交換が bounds とは独立に裏付ける。この profile の
FDMIndex 座標は x1,x2,y1,y2 軸対である。

outline marker は rectangle `01000660`、ellipse `01000460`、line `01000160`。
fill parent `01000a60` は 46-byte prefix と nested outline child、offset 36 に BE-u32 BGR fill を
持つ。既知の opaque AlphaBlend profile が無透明ケースに伴う。任意 path、global world unit、
全 alpha mode、図形編集の解読ではない。

## 保存済み数式

観測した先頭 frame では `JSEQ.Document.3` の frame reference zero と primary/trailing 寸法一致が
対応する。他 class は個別に検証する。GCI snapshot には font `0x94`、select/restore `0x60`、
background `0x40`、一文字 TextOut `0xc8`、release `0x65` packet がある。size field の 370/240 は
通常/小文字寸法に対応する。TextOut の top reference は font baseline の解読ではない。
対照数式の順序付き文字は editable JSEQ3 の character/color packet と一致する。
一般 equation AST や全埋込み形式の抽出 algorithm は定義しない。

## 検証と限界

anchor、wrap、色、順序、数式を独立に変え、stream と native output を比較する。offset の所属
record、version、hash、許可を記録する。複数 image、rich/vertical anchor、font 代替、一般単位、
数式編集は範囲外であり、描画 algorithm と backend 差異は実装記録に置く。

## 実装記録と履歴

[実装の command・API・出力・過去の詳細記録](../../rjtd/docs/research/0008-object-stream-candidates.ja.md).
