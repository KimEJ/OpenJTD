# RFC 0003: DocumentText Content and Associated Records

Status: draft for publication preparation; joint review pending

English source: [0003-document-text.md](0003-document-text.md)

## 範囲と根拠

観測した wire form と限定的な解釈を記録する。初期 local 観測と一太郎 2026 の対照文書は
検証範囲が異なり、歴史的な全 version への適用を証明しない。共同承認前に、主張ごとの公開可能な
入力 ID、hash、作成手順、独立検証結果が必要である。renderer の出力成功だけでは代わりにならない。

## 本文と inline の境界

named stream では byte 0 に `SsmgV.01`、byte 20 に `TextV.01`、byte 28 に BE-u32 の
UTF-16 unit 数がある。本文は byte 32 から `32 + 2 * length` までで、その後が byte-oriented
style event となる。byte 32 の直後から `0x001f` なしで可視本文が始まる場合もある。count の
32 bit 全体を使い、header の下位 word を形式 selector と見なさない。

marker-based fragment は named stream 外にもある。physical-file offset と logical-stream
位置を同一視しない。UTF-16BE と `0x001d ... 0x001e` cache が観測される。可視 base、
読み仮名、field cache、template instruction を区別する。対照 group-ruby profile では selector
`3` が base、`0x0082` が kana annotation に対応する。他の形には個別の framing が必要である。
`/LayoutBoxText` にも bounded `TextV.01` があり、control-only payload の印字可能 word は
可視本文や所有関係の証拠ではない。

## 文字属性の対応

style-event grammar は RFC 0009。次の対応は検証した source range に限る。

| Property | 観測した対応 |
| --- | --- |
| `2` | BE-u16、0.01 mm 単位。370 は約 10.5 pt、494 は約 14 pt。観測 profile の zero は default 復帰 |
| `3` | font ID。`0xffff` は default 復帰、ID zero は有効 |
| `15` | text range の `0x00BBGGRR` 色。`0xffffffff` は automatic/default。table control 全般の意味ではない |
| `20 = 0x84000000` | bold 選択 |
| `20 = 0x90000000` | italic 選択 |
| `20 = 0x80000010`, `13 = 1` | single underline 選択 |
| `20 = 0x80000c00` / `0x80000400`, `4 = 5 = 50` | 上付き・下付き 1/4 menu、半分の文字寸法 |

既知の sequential view `0x1006` payload は bytes 3–4 に default size、5–6 に default font ID を
持つ。完全な payload form が必要である。重複 ID、reserved value、mixed range、未知の property
幅では確定しない。font 名は原本情報であり、代替 font、合成 bold/shear、baseline、PDF paint は実装側で扱う。

## 保存済み field・脚注・目次

対照 class-zero field は長さ/tag が印刷日 28/`0x0033`、page number 15/`0x0035`、外部 link
12/`0x0048`。連続した完全な argument/value wrapper を確認する。`DATE`、`PAGENUMBER`、
HTTP(S) target が観測される。保存文字列だけでは一般 field 評価や改番規則を定義しない。

単一 link の `/Footnote` と `/FootnoteLink` は脚注本文・marker と本文 marker を対応付ける。
property 1 の参照 1/2 と `TextLayoutStyle` slot は当該 typography を裏付けるが、一般継承、
脚注印刷の ON/OFF、page-end/document-end、区切線、脚注 baseline は未解明である。

目次範囲には class `0x0020` の setting record 対がある。17-unit title context の後の empty cache
は 5、18-unit leader context の後は 3。raw 値 1/100 は実線/点線選択に対応し、leader record が
ない variant もある。保存 title・label・範囲は heading level や目次再生成の解読ではない。

## 書字方向・間隔・bookmark

既知 `0x1002` margin/direction profile では 32-byte 横書き payload の byte 10 は `0x40`、
33-byte 縦書きは bytes 10/11 が `0x50,0x01`。他の field と完全な profile の一致が必要であり、
edit/caret state を方向の識別子にしない。`0x1006` tail `0x0266` と既知 `0x100b` の組は
UI 字間 60% に対応するが、一般数式は未解読である。no-fit 二桁縦中横は class-zero/tag `0x0047`
と完全な cache 対を持つ。普通の三桁は任意の数字をこの形へ昇格しない反例となる。

16-word 段落 header の pitch 1000 は 10 mm と対応し、反復 pitch の table/body profile とは異なる。
先頭セル空白と可視 label の font property 範囲は異なり得る。font 値だけで glyph advance は決まらない。
`/MarkTag` と position table は RFC 0006 に記録する。

## 未解決事項

一般の日本語字間・行分散、font/style 継承、field expression、脚注配置、ruby 単位、bookmark
座標を解明する。原本情報と fallback 計測・描画近似を区別する。

## 実装記録と履歴

[実装の command・API・出力・過去の詳細記録](../../rjtd/docs/research/0003-document-text.ja.md).
